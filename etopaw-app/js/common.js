import * as config_import from "../config.js";
import init, * as wasm from "../pkg/etopaw.js";

let valid_login = false;
export let online = false;

export function username() {
    return localStorage.getItem("username");
}

export function token() {
    return localStorage.getItem("token");
}

export function storage_key() {
    return sessionStorage.getItem("storage_key");
}

export function storage_data() {
    return new TextEncoder("utf-8").encode(localStorage.getItem("storage_data"));
}

export function login_data() {
    return { username: username(), token: token() };
}

export function set_valid_login(valid) {
    valid_login = valid;
}

export async function reload_storage_data(wasm) {
    return await raw_fetch(async function (data) {
        const dec = new TextDecoder("utf-8").decode(data);
        try {
            JSON.parse(dec).error;
            vue.username = lang.offline_mode;
            return false;
        } catch (err) {
            const old_storage_data = localStorage.getItem("storage_data");
            if (old_storage_data != null && old_storage_data.length > dec.length) {
                localStorage.setItem("bo_storage_data", old_storage_data);
            }
            localStorage.setItem("storage_data", dec);
            return true;
        }
    }, "data/get_secure", login_data());
}

export async function load_secrets(wasm) {
    try {
        await reload_storage_data(wasm);
    } finally {
        try {
            const storage = wasm.parse_storage(storage_data(), storage_key());
            let secrets = {};
            const sortedKeys = storage["secrets_sort"];
            if (sortedKeys != null) {
                const splitKeys = sortedKeys.split(',');
                for (let i = 0; i < splitKeys.length; i++) {
                    const name = storage[splitKeys[i] + "_secret_name"];
                    const secret = storage[splitKeys[i] + "_secret"];
                    if (name != null && secret != null) {
                        secrets[name] = secret;
                        console.log(name);
                    }
                }
            }
            for (const key in storage) {
                if (sortedKeys != null && !sortedKeys.includes(key) && key.endsWith("_secret")) {
                    secrets[storage[key + "_name"]] = storage[key];
                }
            }
            return secrets;
        } catch (err) {
            console.log(err);
            throw lang.invalid_key;
        }
    }
}

export function logout(logout_el) {
    logout_el.onclick = function () {
        api_fetch(async function (json) { }, "user/logout", login_data());
        localStorage.removeItem("username");
        localStorage.removeItem("token");
        localStorage.removeItem("storage_data");
        localStorage.removeItem("bo_storage_data");
        sessionStorage.removeItem("storage_key");
        location.href = "../";
    };
}

export async function val_login() {
    if (username() == null || token() == null) {
        return false;
    }
    return await api_fetch(async function (json) {
        return json.valid;
    }, "user/valid", login_data());
}

export async function load(exec = async function (wasm) { }, login, rel = "") {
    document.title = config.TITLE;
    await init();
    wasm.set_panic_hook();
    valid_login = await val_login();
    if (login == true && !valid_login) {
        location.href = rel + "../";
    } else if (login == false && valid_login) {
        location.href = rel + "./app/";
    }
    lang_changer();
    return await exec(wasm);
}

export async function api_fetch(exec = async function (json = {}) { }, url = "", headers = {}, body = new Uint8Array(0)) {
    return await raw_fetch(async function (data) {
        const json = JSON.parse(new TextDecoder("utf-8").decode(data));
        if (json.error != undefined && json.error != false) {
            online = false;
            switch (json.error) {
                case "unauthenticated":
                    json.error = lang.unauthenticated;
            }
        }
        return await exec(json);
    }, url, headers, body);
}

export async function raw_fetch(exec = async function (data = new Uint8Array(0)) { }, url = "", headers = {}, body = new Uint8Array(0)) {
    const http_headers = new Headers({ "content-type": "application/json" });
    for (var key in headers) {
        http_headers.append(key, headers[key]);
    }
    let req = {
        method: "POST",
        cache: "no-cache",
        headers,
        body
    };
    try {
        const apiUrl = config.API_URL == "/" ? window.location.protocol + "//" + window.location.hostname + window.location.port : config.API_URL;
        const resp = await fetch(`${apiUrl}/${url}`, req);
        const data = new Uint8Array(await resp.arrayBuffer());
        online = valid_login;
        return await exec(data);
    } catch (err) {
        online = false;
        return await exec(new TextEncoder("utf-8").encode(JSON.stringify({ error: err.toString() })));
    }
}

export const modal = document.querySelector(".modal");
const modal_close = document.querySelector(".modal-close");
const modal_body = document.querySelector(".modal-body");
const modal_title = document.querySelector(".modal-title");
const modal_btn = document.getElementById("modal_btn");
const modal_btn_close = document.getElementById("modal_btn_close");
modal.addEventListener("click", function (e) {
    if (e.target != modal) return;
    modal.hidden = true;
});
modal_close.addEventListener("click", function () {
    modal.hidden = true;
});

export function lang_changer() {
    const lang_btn = document.getElementById("lang_btn");
    const lang_menu = document.getElementById("lang_menu");
    lang_btn.addEventListener("click", function () {
        if (lang_menu.classList.contains("show")) {
            lang_menu.classList.remove("show");
        } else {
            lang_menu.classList.add("show");
        }
    });
    document.body.addEventListener("click", function (e) {
        if (e.target != lang_btn && lang_menu.classList.contains("show")) {
            lang_menu.classList.remove("show");
        }
    });
    document.querySelectorAll(".change-lang").forEach((el) => {
        el.addEventListener("click", function () {
            set_lang(el.lang);
        })
    });
}

export function alert(text = "") {
    modal_title.innerText = config.TITLE;
    modal_body.innerText = text;
    modal_btn.innerText = lang.ok;
    modal_btn_close.innerText = lang.close;
    modal_btn_close.onclick = function () {
        modal.hidden = true;
    };
    modal_btn.onclick = function () {
        modal.hidden = true;
    };
    modal.hidden = false;
    return false;
}

export function alert_error(text = "") {
    modal_title.innerText = lang.error;
    modal_body.innerText = text;
    modal_btn.innerText = lang.ok;
    modal_btn_close.innerText = lang.close;
    modal_btn_close.onclick = function () {
        modal.hidden = true;
    };
    modal_btn.onclick = function () {
        modal.hidden = true;
    };
    modal.hidden = false;
    return false;
}

export function confirm(text = "", exec_fn = async function () { }, custom_body) {
    modal_title.innerText = lang.confirmation;
    modal_body.innerText = text;
    if (custom_body != null) {
        modal_body.innerHTML += custom_body;
    }
    modal_btn.innerText = lang.confirm;
    modal_btn_close.innerText = lang.cancel;
    modal_btn_close.onclick = function () {
        modal.hidden = true;
    };
    modal_btn.onclick = function () {
        modal.hidden = true;
        exec_fn();
    };
    modal.hidden = false;
    return false;
}

export const config = config_import;
export let lang = { help_qa: { download: {}, what_is: {}, usage: {}, questions: {} } };
import("./" + (localStorage.getItem("lang") == null ? config.LANG : localStorage.getItem("lang")) + ".js").then((module) => { lang = module.lang; vue.lang = lang; });

export function set_lang(lang) {
    if (lang == undefined) {
        localStorage.removeItem("lang");
    } else {
        localStorage.setItem("lang", lang);
    }
    location.reload();
}

export const vue = new Vue({
    el: "#vue",
    data: {
        lang,
        config,
        username: username()
    }
});

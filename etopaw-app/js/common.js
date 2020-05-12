import * as config_import from "../config.js";
import { lang as lang_import } from "./lang.js";
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

export async function reload_storage_data(wasm) {
    return await raw_fetch(async function (data) {
        const dec = new TextDecoder("utf-8").decode(data);
        try {
            JSON.parse(dec).error;
            return false;
        } catch (err) {
            localStorage.setItem("storage_data", dec);
            return true;
        }
    }, "data/get_secure", login_data());
}

export async function load_secrets(wasm) {
    try {
        await reload_storage_data(wasm);
    } finally {
        const storage = wasm.parse_storage(storage_data(), storage_key());
        let secrets = {};
        for (const key in storage) {
            if (key.endsWith("_secret")) {
                secrets[storage[key + "_name"]] = storage[key];
            }
        }
        return secrets;
    }
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
    return await exec(wasm);
}

export async function api_fetch(exec = async function (json = {}) { }, url = "", headers = {}, body = new Uint8Array(0)) {
    return await raw_fetch(async function (data) {
        const json = JSON.parse(new TextDecoder("utf-8").decode(data));
        if (json.error != undefined && json.error != false) {
            online = false;
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
        const resp = await fetch(`${config.API_URL}/${url}`, req);
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
    if (e.target !== modal) return;
    modal.hidden = true;
});
modal_close.addEventListener("click", function () {
    modal.hidden = true;
});

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

export function confirm(text = "", exec_fn = async function () { }) {
    modal_title.innerText = lang.confirmation;
    modal_body.innerText = text;
    modal_btn.innerText = lang.confirm;
    modal_btn_close.innerText = lang.close;
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
export const lang = lang_import[localStorage.getItem("lang") == null ? config.LANG : localStorage.getItem("lang")];

new Vue({
    el: "#vue",
    data: {
        lang,
        config,
        username: localStorage.getItem("username")
    }
});

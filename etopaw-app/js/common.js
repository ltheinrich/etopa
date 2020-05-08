import * as config_import from "../config.js";
import { lang as lang_import } from "./lang.js";
import init, * as wasm from "../pkg/etopaw.js";

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

export async function require_logout(rel = "./app/") {
    if (await valid_login()) {
        location.href = rel;
        return false;
    }
    return true;
}

export async function require_login(rel = "../") {
    if (!(await valid_login())) {
        location.href = rel;
        return false;
    }
    return true;
}

export async function valid_login() {
    if (username() == null || token() == null) {
        return false;
    }
    return await api_fetch(async function (json) {
        return json.valid;
    }, "user/valid", login_data());
}

export async function load(exec = async function (wasm) { }, login, rel) {
    document.title = config.TITLE;
    await init();
    wasm.set_panic_hook();
    let ok;
    if (login == undefined) {
        ok = true;
    } else if (login) {
        ok = await require_login(rel);
    } else {
        ok = await require_logout(rel);
    }
    if (ok) {
        return await exec(wasm);
    }
}

export async function api_fetch(exec = async function (json = {}) { }, url = "", data = {}, body = new Uint8Array(0)) {
    return await raw_fetch(async function (data) {
        const json = JSON.parse(new TextDecoder("utf-8").decode(data));
        if (json.error != undefined && json.error != false) {
            online = false;
        }
        return await exec(json);
    }, url, data, body);
}

export async function raw_fetch(exec = async function (data = new Uint8Array(0)) { }, url = "", data = {}, body = new Uint8Array(0)) {
    const headers = new Headers({ "content-type": "application/json" });
    for (var key in data) {
        headers.append(key, data[key]);
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
        online = true;
        return await exec(data);
    } catch (err) {
        online = false;
        return await exec(new Response(JSON.stringify({ error: err.toString() })));
    }
}

export const config = config_import;
export const lang = lang_import[localStorage.getItem("lang") == null ? config.LANG : localStorage.getItem("lang")];

new Vue({
    el: "#vue",
    data: {
        lang,
        config
    }
});

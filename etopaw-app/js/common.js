import * as config from "./config.js";
import init, * as wasm from "../pkg/etopaw.js";

export function username() {
    return sessionStorage.getItem("username");
}

export function token() {
    return sessionStorage.getItem("token");
}

export function login_data() {
    return { username: username(), token: token() };
}

export async function require_logout() {
    if (await valid_login()) {
        location.href = "./index.html";
        return false;
    }
    return true;
}

export async function require_login() {
    if (!(await valid_login())) {
        location.href = "./login.html";
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

export async function load(exec = async function (wasm) { }, login = true) {
    document.title = config.TITLE;
    await init();
    wasm.set_panic_hook();
    let ok;
    if (login) {
        ok = await require_login();
    } else {
        ok = await require_logout();
    }
    if (ok) {
        return await exec(wasm);
    }
}

export async function api_fetch(exec = async function (json = {}) { }, url = "", data = {}, body = new Uint8Array(0)) {
    return await raw_fetch(async function (resp) {
        const json = await resp.json();
        return await exec(json);
    }, url, data, body);
}

export async function raw_fetch(exec = async function (resp = new Response()) { }, url = "", data = {}, body = new Uint8Array(0)) {
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
    const resp = await fetch(`${config.API_URL}/${url}`, req);
    return await exec(resp);
}

new Vue({
    el: "#vue",
    data: {
        lang: config.lang[localStorage.getItem("lang") == null ? config.LANG : localStorage.getItem("lang")]
    }
});

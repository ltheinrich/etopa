import * as config from "./config.js";
import init, * as wasm from "../pkg/etopaw.js";

export async function load() {
    document.title = config.TITLE;
    await init();
    wasm.set_panic_hook();
    return wasm;
}

export async function fetch_api(url = "", data = {}, body) {
    const resp = await raw_fetch(url, data, body);
    return resp.json();
}

export async function raw_fetch(url = "", data = {}, body = new Uint8Array(0)) {
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
    return resp;
}

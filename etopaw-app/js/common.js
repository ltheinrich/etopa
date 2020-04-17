import * as config from "./config.js";
import init, * as wasm from "../pkg/etopaw.js";

export async function load(exec = async function (wasm) { }) {
    document.title = config.TITLE;
    await init();
    wasm.set_panic_hook();
    await exec(wasm);
}

export async function api_fetch(exec = async function (json = {}) { }, url = "", data = {}, body = new Uint8Array(0)) {
    await raw_fetch(async function (resp) {
        const json = await resp.json();
        await exec(json);
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
    await exec(resp);
}

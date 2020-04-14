import * as config from "./config.js";
import init, * as wasm from "../pkg/etopaw.js";

export async function load() {
    document.title = config.TITLE;
    await init();
    wasm.set_panic_hook();
    return wasm;
}

export async function fetch_api(url = "", data = {}) {
    const resp = await fetch(`${config.API_URL}/${url}`, {
        method: "POST",
        cache: "no-cache",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify(data)
    });
    return resp.json();
}


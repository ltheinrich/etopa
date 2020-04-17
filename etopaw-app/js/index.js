import { load, api_fetch, raw_fetch } from "./common.js";

if (sessionStorage.getItem("username") == null || sessionStorage.getItem("token") == null) {
    location.href = "./login.html";
}

load(async function (wasm) {
    raw_fetch(async function (resp) {
        const buffer = await resp.arrayBuffer();
        let storage = wasm.decrypt_storage(new Uint8Array(buffer), "12345678901234567890123456789012");
        console.log(storage);
        storage["abc"] = "test";
        const encrypted = wasm.encrypt_storage(storage, "12345678901234567890123456789012");
        api_fetch(async function (json) {
            console.log(json);
        }, "data/set_secure", { username: sessionStorage.getItem("username"), token: sessionStorage.getItem("token") }, encrypted);
    }, "data/get_secure", { username: sessionStorage.getItem("username"), token: sessionStorage.getItem("token") });
});

document.getElementById("logout").onclick = function () {
    api_fetch(async function (json) {
        sessionStorage.removeItem("username");
        sessionStorage.removeItem("token");
        if ("success" in json) {
            location.href = "./login.html";
        } else {
            document.getElementById("result").innerText = json.error;
        }
    }, "user/logout", { username: sessionStorage.getItem("username"), token: sessionStorage.getItem("token") });
};

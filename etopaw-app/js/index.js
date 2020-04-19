import { load, api_fetch, raw_fetch, login_data } from "./common.js";

load(async function (wasm) {
    raw_fetch(async function (resp) {
        const buffer = await resp.arrayBuffer();
        let storage = JSON.parse(wasm.decrypt_storage(new Uint8Array(buffer), "12345678901234567890123456789012"));
        if (storage.secrets == undefined) {
            storage.secrets = {};
        }
        for (var key in storage.secrets) {
            console.log(key, wasm.gen_token(storage.secrets[key], BigInt(Date.now())));
        }
        storage.secrets["abc"] = "JBSWY3DPEHPK3PXP";
        const encrypted = wasm.encrypt_storage(JSON.stringify(storage), "12345678901234567890123456789012");
        api_fetch(async function (json) {
            console.log(JSON.stringify(json));
        }, "data/set_secure", login_data(), encrypted);
    }, "data/get_secure", login_data());
});

document.getElementById("logout").onclick = function () {
    sessionStorage.removeItem("username");
    sessionStorage.removeItem("token");
    location.href = "./login.html";
    api_fetch(async function (json) { }, "user/logout", login_data());
};

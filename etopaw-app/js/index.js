import { load, fetch_api, raw_fetch } from "./common.js";

if (sessionStorage.getItem("username") == null || sessionStorage.getItem("token") == null) {
    location.href = "./login.html";
}

load().then(wasm => {
    const raw = raw_fetch("data/get_secure", { username: sessionStorage.getItem("username"), token: sessionStorage.getItem("token") });
    const d = raw.then(raw => raw.arrayBuffer().then(buffer => {
        const storage = wasm.decrypt_storage(new Uint8Array(buffer), "12345678901234567890123456789012");
        console.log(storage);
        storage["abc"] = "test";
        const encrypted = wasm.encrypt_storage(storage, "12345678901234567890123456789012");
        raw_fetch("data/set_secure", { username: sessionStorage.getItem("username"), token: sessionStorage.getItem("token") }, encrypted).then(res => res.text().then(text => console.log(text)));
    }));

});

document.getElementById("logout").onclick = function () {
    fetch_api("user/logout", {
        username: sessionStorage.getItem("username"), token: sessionStorage.getItem("token")
    })
        .then((resp) => {
            sessionStorage.removeItem("username");
            sessionStorage.removeItem("token");
            if ("success" in resp) {
                location.href = "./login.html";
            } else {
                document.getElementById("result").innerText = resp.error;
            }
        });
};

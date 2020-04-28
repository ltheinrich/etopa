import { load, api_fetch } from "./common.js";

load(async function (wasm) {
    document.getElementById("loginform").onsubmit = function () {
        const username = document.getElementById("username").value;
        const password = wasm.hash_password(document.getElementById("password").value, username);
        const storage_key = wasm.hash_key(document.getElementById("password").value, username);
        api_fetch(async function (json) {
            if ("token" in json) {
                sessionStorage.setItem("username", username);
                sessionStorage.setItem("token", json.token);
                sessionStorage.setItem("storage_key", storage_key);
                document.getElementById("result").innerText = "Login successful";
                location.href = "./index.html";
            } else {
                document.getElementById("result").innerText = json.error;
            }
        }, "user/login", { username, password });
        return false;
    };
}, false);

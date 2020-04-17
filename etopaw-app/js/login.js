import { load, api_fetch } from "./common.js";

if (sessionStorage.getItem("username") != null && sessionStorage.getItem("token") != null) {
    location.href = "./index.html";
}

load(async function (wasm) {
    document.getElementById("form").onsubmit = function () {
        const username = document.getElementById("username").value;
        const password = wasm.hash_password(document.getElementById("password").value, username);
        api_fetch(async function (json) {
            if ("token" in json) {
                sessionStorage.setItem("username", username);
                sessionStorage.setItem("token", json.token);
                document.getElementById("result").innerText = "Login successful";
                location.href = "./index.html";
            } else {
                document.getElementById("result").innerText = json.error;
            }
        }, "user/login", { username, password });
        return false;
    };
});

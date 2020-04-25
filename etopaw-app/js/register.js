import { load, api_fetch, raw_fetch } from "./common.js";

load(async function (wasm) {
    document.getElementById("form").onsubmit = function () {
        const username = document.getElementById("username").value;
        const raw_password = document.getElementById("password").value;
        const repeat_password = document.getElementById("repeat_password").value;
        if (raw_password != repeat_password) {
            alert("Passwords do not match");
            return false;
        }
        const password = wasm.hash_password(raw_password, username);
        const storage_key = wasm.hash_key(document.getElementById("password").value, username);
        api_fetch(async function (json) {
            if ("token" in json) {
                sessionStorage.setItem("username", username);
                sessionStorage.setItem("token", json.token);
                sessionStorage.setItem("storage_key", storage_key);
                document.getElementById("result").innerText = "Registration successful";
                location.href = "./index.html";
            } else {
                document.getElementById("result").innerText = json.error;
            }
        }, "user/register", { username, password });
        return false;
    };
}, false);

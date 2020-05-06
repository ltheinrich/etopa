import { load, api_fetch } from "./js/common.js";

load(async function (wasm) {
    handle_login(wasm);
    handle_register(wasm);
}, false);


function handle_login(wasm) {
    const loginbtn = document.getElementById("loginbtn");
    document.getElementById("login").onsubmit = function () {
        if (empty_inputs()) {
            return alert("Empty username or password") == true;
        }
        loginbtn.disabled = true;
        const username = document.getElementById("username").value;
        const password = wasm.hash_password(document.getElementById("password").value, username);
        const storage_key = wasm.hash_key(document.getElementById("password").value, username);
        api_fetch(async function (json) {
            if ("token" in json) {
                sessionStorage.setItem("username", username);
                sessionStorage.setItem("token", json.token);
                sessionStorage.setItem("storage_key", storage_key);
                location.href = "./app/";
            } else {
                alert("API error: " + json.error);
                loginbtn.disabled = false;
            }
        }, "user/login", { username, password });
        return false;
    };
}

function handle_register(wasm) {
    const register = document.getElementById("register");
    register.onclick = function () {
        if (empty_inputs()) {
            return alert("Empty username or password") == true;
        }
        register.disabled = true;
        const username = document.getElementById("username").value;
        const password = wasm.argon2_hash(document.getElementById("password").value, username);
        const storage_key = wasm.hash_key(document.getElementById("password").value, username);
        api_fetch(async function (json) {
            if ("token" in json) {
                sessionStorage.setItem("username", username);
                sessionStorage.setItem("token", json.token);
                sessionStorage.setItem("storage_key", storage_key);
                location.href = "./app/";
            } else {
                alert("API error: " + json.error);
                register.disabled = false;
            }
        }, "user/register", { username, password });
        return false;
    };
}

function empty_inputs() {
    return document.getElementById("username").value == "" || document.getElementById("password").value == "";
}

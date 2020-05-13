import { load, api_fetch, load_secrets, alert_error } from "./js/common.js";

const username = document.getElementById("username");
const password = document.getElementById("password");
const enc_password = document.getElementById("enc_password");
const login_btn = document.getElementById("login_btn");
const login = document.getElementById("login");
const register = document.getElementById("register");

load(async function (wasm) {
    try {
        await load_secrets(wasm);
        return location.href = "./app/";
    } catch (err) {
        login.onsubmit = function () { handle_login(wasm); return false; };
        register.onclick = function () { handle_register(wasm); return false; };
    }
}, false);

function handle_login(wasm) {
    if (enc_password.value != "" && username.value == "" && password.value == "") {
        sessionStorage.setItem("storage_key", wasm.hash_key(enc_password.value));
        location.href = "./app/";
    } else if (username.value != "" && password.value != "") {
        disabled(true);
        const password_hash = wasm.hash_password(password.value);
        const storage_key = wasm.hash_key(enc_password.value);
        api_fetch(async function (json) {
            if ("token" in json) {
                localStorage.setItem("username", username.value);
                localStorage.setItem("token", json.token);
                sessionStorage.setItem("storage_key", storage_key);
                location.href = "./app/";
            } else {
                alert_error("API error: " + json.error);
                disabled(false);
            }
        }, "user/login", { username: username.value, password: password_hash });
    } else {
        alert_error(lang.empty_username_password);
    }
}

function handle_register(wasm) {
    if (username.value == "" || password.value == "" || enc_password.value == "") {
        return alert_error(lang.empty_username_password);
    }
    disabled(true);
    const argon2_hash = wasm.argon2_hash(password.value);
    const storage_key = wasm.hash_key(enc_password.value);
    api_fetch(async function (json) {
        if ("token" in json) {
            localStorage.setItem("username", username.value);
            localStorage.setItem("token", json.token);
            sessionStorage.setItem("storage_key", storage_key);
            location.href = "./app/";
        } else {
            alert_error("API error: " + json.error);
            disabled(false);
        }
    }, "user/register", { username: username.value, password: argon2_hash });
}

function disabled(val) {
    login.disabled = val;
    login_btn.disabled = val;
    register.disabled = val;
    username.disabled = val;
    password.disabled = val;
    enc_password.disabled = val;
}

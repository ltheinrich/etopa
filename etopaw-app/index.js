import { load, api_fetch, load_secrets } from "./js/common.js";

const username = document.getElementById("username");
const password = document.getElementById("password");
const encpassword = document.getElementById("encpassword");
const loginbtn = document.getElementById("loginbtn");
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
    if (empty_inputs()) {
        if (encpassword.value != "") {
            sessionStorage.setItem("storage_key", wasm.hash_key(encpassword.value));
            return location.href = "./app/";
        }
        return alert("Empty username or password");
    }
    disabled(true);
    const password_hash = wasm.hash_password(password.value);
    const storage_key = wasm.hash_key(encpassword.value);
    api_fetch(async function (json) {
        if ("token" in json) {
            localStorage.setItem("username", username.value);
            localStorage.setItem("token", json.token);
            sessionStorage.setItem("storage_key", storage_key);
            location.href = "./app/";
        } else {
            alert("API error: " + json.error);
            disabled(false);
        }
    }, "user/login", { username: username.value, password: password_hash });
}

function handle_register(wasm) {
    if (empty_inputs()) {
        return alert("Empty username or (encryption) password");
    }
    disabled(true);
    const argon2_hash = wasm.argon2_hash(password.value);
    const storage_key = wasm.hash_key(encpassword.value);
    api_fetch(async function (json) {
        if ("token" in json) {
            localStorage.setItem("username", username.value);
            localStorage.setItem("token", json.token);
            sessionStorage.setItem("storage_key", storage_key);
            location.href = "./app/";
        } else {
            alert("API error: " + json.error);
            disabled(false);
        }
    }, "user/register", { username: username.value, password: argon2_hash });
}

function empty_inputs() {
    return username.value == "" || password.value == "" || encpassword.value == "";
}

function disabled(val) {
    login.disabled = val;
    loginbtn.disabled = val;
    register.disabled = val;
    username.disabled = val;
    password.disabled = val;
    encpassword.disabled = val;
}

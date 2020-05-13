import { load, api_fetch, login_data, storage_key, load_secrets, confirm, alert, alert_error, username, logout } from "../js/common.js";
import { lang } from "../js/lang.js";

let wasm;

const enc_password = document.getElementById("enc_password")
const new_username = document.getElementById("new_username");
const new_password = document.getElementById("new_password");
const repeat_new_password = document.getElementById("repeat_new_password");
const new_enc_password = document.getElementById("new_enc_password");
const repeat_new_enc_password = document.getElementById("repeat_new_enc_password");
const change_username_btn = document.getElementById("change_username_btn");
const change_password_btn = document.getElementById("change_password_btn");
const change_enc_password_btn = document.getElementById("change_enc_password_btn");
const delete_user_btn = document.getElementById("delete_user_btn");
const logout_el = document.getElementById("logout");

load(async function (temp_wasm) {
    wasm = temp_wasm;
    document.getElementById("change_username").onsubmit = function () {
        change_username();
        return false;
    };
    document.getElementById("change_password").onsubmit = function () {
        change_password();
        return false;
    };
    document.getElementById("change_enc_password").onsubmit = function () {
        change_enc_password();
        return false;
    };
    document.getElementById("delete_user").onsubmit = function () {
        confirm("Delete user?", delete_user);
        return false;
    };
}, true);

async function change_username() {
    if (wasm.hash_key(enc_password.value) != storage_key()) {
        return alert_error("Encryption password incorrect");
    }
    disabled(true);
    await api_fetch(async function (json) {
        if (json.error == false) {
            logout_el.innerText = logout_el.innerText.replace("(" + username() + ")", "(" + new_username.value + ")");
            localStorage.setItem("username", new_username.value);
            clear_inputs();
            alert("Username successfully changed");
        } else {
            alert_error("API error: " + json.error);
        }
        disabled(false);
    }, "user/change_username", { newusername: new_username.value, ...login_data() });
}

async function change_password() {
    if (new_password.value != repeat_new_password.value) {
        return alert_error("Passwords do not match");
    } else if (wasm.hash_key(enc_password.value) != storage_key()) {
        return alert_error("Encryption password incorrect");
    }
    disabled(true);
    await api_fetch(async function (json) {
        if (json.error == false) {
            clear_inputs();
            alert("Password successfully changed");
        } else {
            alert_error("API error: " + json.error);
        }
        disabled(false);
    }, "user/change_password", { newpassword: wasm.argon2_hash(new_password.value), ...login_data() });
}

async function change_enc_password() {
    if (new_enc_password.value != repeat_new_enc_password.value) {
        return alert_error("Passwords do not match");
    } else if (wasm.hash_key(enc_password.value) != storage_key()) {
        return alert_error("Encryption password incorrect");
    }
    const new_storage_key = wasm.hash_key(new_enc_password.value);
    try {
        const secrets = await load_secrets(wasm);
        const new_storage = wasm.serialize_storage(secrets, new_storage_key);
        disabled(true);
        await api_fetch(async function (json) {
            if (json.error == false) {
                localStorage.setItem("storage_data", new_storage);
                sessionStorage.setItem("storage_key", new_storage_key);
                clear_inputs();
                alert("Encryption password successfully changed");
            } else {
                alert_error("API error: " + json.error);
            }
            disabled(false);
        }, "data/set_secure", login_data(), new_storage);
    } catch (err) {
        alert_error(err);
        await new Promise(resolve => setTimeout(resolve, 5000));
        location.href = "./";
    }
}

async function delete_user() {
    if (wasm.hash_key(enc_password.value) != storage_key()) {
        return alert_error("Encryption password incorrect");
    }
    disabled(true);
    await api_fetch(async function (json) {
        if (json.error == false) {
            sessionStorage.clear();
            localStorage.clear();
            clear_inputs();
            location.href = "../";
        } else {
            alert_error("API error: " + json.error);
        }
        disabled(false);
    }, "user/delete", login_data());
}

function clear_inputs() {
    enc_password.value = "";
    new_username.value = "";
    new_password.value = "";
    repeat_new_password.value = "";
    new_enc_password.value = "";
    repeat_new_enc_password.value = "";
}

function disabled(disable) {
    document.getElementById("change_username").disabled = disable;
    document.getElementById("change_password").disabled = disable;
    document.getElementById("change_enc_password").disabled = disable;
    document.getElementById("delete_user").disabled = disable;
    enc_password.disabled = disable;
    new_username.disabled = disable;
    new_password.disabled = disable;
    repeat_new_password.disabled = disable;
    new_enc_password.disabled = disable;
    repeat_new_enc_password.disabled = disable;
    change_username_btn.disabled = disable;
    change_password_btn.disabled = disable;
    change_enc_password_btn.disabled = disable;
    delete_user_btn.disabled = disable;
}

logout(logout_el);

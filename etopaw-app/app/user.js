import { load, api_fetch, login_data, username, storage_key, load_secrets } from "../js/common.js";

let wasm;

const enc_password = document.getElementById("enc_password")
const new_username = document.getElementById("new_username");
const new_password = document.getElementById("new_password");
const repeat_new_password = document.getElementById("repeat_new_password");
const new_enc_password = document.getElementById("new_enc_password");
const repeat_new_enc_password = document.getElementById("repeat_new_enc_password");

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
        delete_user();
        return false;
    };
});

async function change_username() {
    if (wasm.hash_key(enc_password.value) != storage_key()) {
        return alert("Encryption password incorrect");
    }
    await api_fetch(async function (json) {
        if (json.error == false) {
            localStorage.setItem("username", new_username.value);
            alert("Username successfully changed");
        } else {
            alert("API error: " + json.error);
        }
    }, "user/change_username", { newusername: new_username.value, ...login_data() });
}

async function change_password() {
    if (new_password.value != repeat_new_password.value) {
        return alert("Passwords do not match");
    } else if (wasm.hash_key(enc_password.value) != storage_key()) {
        return alert("Encryption password incorrect");
    }
    await api_fetch(async function (json) {
        if (json.error == false) {
            alert("Password successfully changed");
        } else {
            alert("API error: " + json.error);
        }
    }, "user/change_password", { newpassword: wasm.argon2_hash(new_password.value), ...login_data() });
}

async function change_enc_password() {
    if (new_enc_password.value != repeat_new_enc_password.value) {
        return alert("Passwords do not match");
    } else if (wasm.hash_key(enc_password.value) != storage_key()) {
        return alert("Encryption password incorrect");
    }
    const new_storage_key = wasm.hash_key(new_enc_password.value);
    const secrets = await load_secrets(wasm);
    const new_storage = wasm.serialize_storage(secrets, new_storage_key);
    await api_fetch(async function (json) {
        if (json.error == false) {
            localStorage.setItem("storage_data", new_storage);
            sessionStorage.setItem("storage_key", new_storage_key);
            alert("Encryption password successfully changed");
        } else {
            alert("API error: " + json.error);
        }
    }, "data/set_secure", login_data(), new_storage);
}

async function delete_user() {
    if (wasm.hash_key(encpassword.value) != storage_key()) {
        return alert("Encryption password incorrect");
    }
    await api_fetch(async function (json) {
        if (json.error == false) {
            sessionStorage.clear();
            localStorage.clear();
            location.href = "../";
        } else {
            alert("API error: " + json.error);
        }
    }, "user/delete", login_data());
}

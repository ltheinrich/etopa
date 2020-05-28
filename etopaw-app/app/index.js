import { load, api_fetch, login_data, lang, load_secrets, storage_key, online, alert_error, logout, set_valid_login, storage_data, confirm, username as get_username, vue } from "../js/common.js";

let wasm;
let secrets;

const key = document.getElementById("key");
const add = document.getElementById("add");
const add_form = document.getElementById("add_form");
const totp = document.getElementById("totp");
const decryption = document.getElementById("decryption");
const decrypt = document.getElementById("decrypt");
const name = document.getElementById("name");
const secret = document.getElementById("secret");
const tokens = document.getElementById("tokens");
const user_btn = document.getElementById("user_btn");
const logout_el = document.getElementById("logout");
const offline_mode = document.getElementById("offline_mode");
const time_left = document.getElementById("time_left");
const loading = document.getElementById("loading");
const username = document.getElementById("username");
const password = document.getElementById("password");
const disable_offline = document.getElementById("disable_offline");

load(async function (temp_wasm) {
    wasm = temp_wasm;
    const can_decrypt = await try_init();
    if (!can_decrypt) {
        decryption.onsubmit = function () {
            key.disabled = true;
            decryption.disabled = true;
            decrypt.disabled = true;
            decrypt_storage();
            key.disabled = false;
            decryption.disabled = false;
            decrypt.disabled = false;
            return false;
        };
        decryption.hidden = false;
    }
    loading.hidden = true;
});

async function try_init() {
    try {
        await reload_secrets();
        if (storage_data().length == 4) {
            alert_error(lang.empty_storage);
            setTimeout(function () {
                location.href = "../";
            }, 3000);
            return false;
        }
        reload_tokens(true);
        setInterval(reload_tokens, 1000);
        add_form.onsubmit = function () { add_token(); return false; };
        disable_offline.onsubmit = function () {
            username.disabled = true;
            password.disabled = true;
            offline_mode.disabled = true;
            const password_hash = wasm.hash_password(password.value);
            api_fetch(async function (json) {
                if ("token" in json) {
                    localStorage.setItem("username", username.value);
                    localStorage.setItem("token", json.token);
                    set_valid_login(true);
                    await reload_secrets();
                    vue.username = get_username();
                    add_form.hidden = !online;
                    user_btn.hidden = !online;
                    disable_offline.hidden = online;
                    totp.hidden = false;
                    decryption.hidden = true;
                } else {
                    alert_error(json.error);
                }
                username.disabled = false;
                password.disabled = false;
                offline_mode.disabled = false;
            }, "user/login", { username: username.value, password: password_hash });
            return false;
        };
        add_form.hidden = !online;
        user_btn.hidden = !online;
        disable_offline.hidden = online;
        totp.hidden = false;
        decryption.hidden = true;
        return true;
    } catch (err) {
        if (err == lang.invalid_key) {
            if (storage_key() != null) {
                alert_error(err);
            }
        } else {
            console.log(err);
        }
        return false;
    }
}

async function decrypt_storage() {
    if (key.value == "") {
        return alert_error(lang.empty_key) == true;
    }
    sessionStorage.setItem("storage_key", wasm.hash_key(key.value))
    if (!await try_init()) {
        return alert_error(lang.decryption_failed) == true;
    }
    return false;
}

async function reload_secrets() {
    secrets = await load_secrets(wasm);
    reload_tokens(true);
}

async function add_token() {
    let secret_value_raw = secret.value.replace(" ", "");
    if (name.value != "" && secret_value_raw != "") {
        if (secrets[name.value] == undefined) {
            disabled(true);
            if (wasm.gen_token(secret_value_raw, BigInt(Date.now())) == "invalid secret") {
                alert_error(lang.invalid_secret);
                return disabled(false);
            }
            let secret_name = wasm.hash_name(name.value);
            let secret_value = wasm.encrypt_hex(secret_value_raw, storage_key());
            let secret_name_encrypted = wasm.encrypt_hex(name.value, storage_key());
            api_fetch(async function (json) {
                if (json.error == false) {
                    reload_secrets();
                    gen_tokens();
                    name.value = "";
                    secret.value = "";
                } else {
                    alert_error(lang.api_error_cs + json.error);
                }
                disabled(false);
            }, "data/update", { secretname: secret_name, secretvalue: secret_value, secretnameencrypted: secret_name_encrypted, ...login_data() });
        } else {
            alert_error(lang.name_exists)
        }
    } else {
        alert_error(lang.name_secret_empty);
    }
}

function disabled(disable) {
    name.disabled = disable;
    secret.disabled = disable;
    add.disabled = disable;
}

function remove_token(name) {
    if (name != "") {
        if (secrets[name] != undefined) {
            let secret_name = wasm.hash_name(name);
            api_fetch(async function (json) {
                if (json.error == false) {
                    delete secrets[name];
                    gen_tokens();
                } else {
                    alert_error(lang.api_error_cs + json.error);
                }
            }, "data/delete", { secretname: secret_name, ...login_data() });
        } else {
            alert_error(lang.name_nonexistent)
        }
    } else {
        alert_error(lang.name_empty);
    }
}

function reload_tokens(force = false) {
    const left = 30 - (Date.now() / 1000) % 30;
    const round_left = Math.round(left);
    time_left.setAttribute("aria-valuenow", round_left);
    time_left.style = "width: " + (round_left / 30) * 100 + "%";
    time_left.innerText = round_left;
    if (left > 29 || force) {
        gen_tokens();
    }
}

function gen_tokens() {
    tokens.innerHTML = "";
    for (const name in secrets) {
        const a = document.createElement("a");
        const token = wasm.gen_token(secrets[name], BigInt(Date.now()));
        a.innerHTML = "<div><strong>" + name + "</strong>&nbsp;" + token + "</div>";
        a.addEventListener("click", function () {
            const el = document.createElement("textarea");
            el.value = token;
            document.body.appendChild(el);
            el.select();
            document.execCommand('copy');
            document.body.removeChild(el);
        });
        a.classList.add("list-group-item");
        a.classList.add("list-group-item-action");
        a.classList.add("d-flex");
        a.classList.add("justify-content-between");
        a.classList.add("align-items-center");
        a.href = "#";
        if (online) {
            const button = document.createElement("a");
            button.innerText = lang.delete;
            button.addEventListener("click", function () {
                confirm(lang.delete_secret_qm, function () {
                    remove_token(name);
                });
            });
            button.classList.add("badge");
            button.classList.add("badge-danger");
            button.classList.add("badge-pill");
            button.href = "#";
            a.appendChild(button);
        }
        tokens.appendChild(a);
    }
}

logout(logout_el);

import { load, api_fetch, login_data, lang, load_secrets, storage_key, online, alert_error } from "../js/common.js";

let wasm;
let secrets;

const enc_password = document.getElementById("enc_password");
const add = document.getElementById("add");
const add_form = document.getElementById("add_form");
const totp = document.getElementById("totp");
const decryption = document.getElementById("decryption");
const decrypt = document.getElementById("decrypt");
const name = document.getElementById("name");
const secret = document.getElementById("secret");
const tokens = document.getElementById("tokens");
const user_btn = document.getElementById("user_btn");
const logout = document.getElementById("logout");
const offline_mode = document.getElementById("offline_mode");
const time_left = document.getElementById("time_left");

load(async function (temp_wasm) {
    wasm = temp_wasm;
    if (!await try_init()) {
        decrypt.addEventListener("click", function () {
            decrypt_storage();
            return false;
        });
        decryption.hidden = false;
    }
});

async function try_init() {
    try {
        await reload_secrets();
        reload_tokens(true);
        setInterval(reload_tokens, 1000);
        add_form.onsubmit = function () { add_token(); return false; };
        offline_mode.addEventListener("click", function () {
            sessionStorage.removeItem("storage_key");
            location.href = "../";
        })
        add_form.hidden = !online;
        user_btn.hidden = !online;
        offline_mode.hidden = online;
        totp.hidden = false;
        decryption.hidden = true;
        return true;
    } catch (err) {
        console.log(err);
        return false;
    }
}

async function decrypt_storage() {
    if (enc_password.value == "") {
        return alert_error("Empty encryption password") == true;
    }
    sessionStorage.setItem("storage_key", wasm.hash_key(enc_password.value))
    if (!await try_init()) {
        return alert_error("Could not decrypt secure storage") == true;
    }
    return false;
}

async function reload_secrets() {
    secrets = await load_secrets(wasm);
    reload_tokens(true);
}

async function add_token() {
    if (name.value != "" && secret.value != "") {
        if (secrets[name.value] == undefined) {
            disabled(true);
            let secret_name = wasm.hash_name(name.value);
            let secret_value = wasm.encrypt_hex(secret.value, storage_key());
            let secret_name_encrypted = wasm.encrypt_hex(name.value, storage_key());
            api_fetch(async function (json) {
                if (json.error == false) {
                    reload_secrets();
                    gen_tokens();
                    name.value = "";
                    secret.value = "";
                } else {
                    alert_error("API error: " + json.error);
                }
                disabled(false);
            }, "data/update", { secretname: secret_name, secretvalue: secret_value, secretnameencrypted: secret_name_encrypted, ...login_data() });
        } else {
            alert_error("Name for secret already exists")
        }
    } else {
        alert_error("Name or secret empty");
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
                    alert_error("API error: " + json.error);
                }
            }, "data/delete", { secretname: secret_name, ...login_data() });
        } else {
            alert_error("Name does not exists")
        }
    } else {
        alert_error("Name empty");
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
    for (const key in secrets) {
        const a = document.createElement("a");
        const token = wasm.gen_token(secrets[key], BigInt(Date.now()));
        a.innerHTML = "<div><strong>" + key + "</strong>&nbsp;" + token + "</div>";
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
                if (confirm("Delete secret")) {
                    remove_token(key);
                }
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

logout.onclick = function () {
    api_fetch(async function (json) { }, "user/logout", login_data());
    localStorage.removeItem("username");
    localStorage.removeItem("token");
    sessionStorage.removeItem("storage_key");
    location.href = "../";
};

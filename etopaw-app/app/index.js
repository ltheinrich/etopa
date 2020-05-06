import { load, api_fetch, login_data, lang, load_secrets, username, storage_key } from "../js/common.js";

let wasm;
let secrets;

load(async function (temp_wasm) {
    wasm = temp_wasm;
    await reload_secrets();
    do_reload_tokens();
    document.getElementById("add").addEventListener("click", add_token);
});

async function reload_secrets() {
    secrets = await load_secrets(wasm);
    reload_tokens(true);
}

async function add_token() {
    const name = document.getElementById("name");
    const secret = document.getElementById("secret");
    if (name.value != "" && secret.value != "") {
        if (secrets[name.value] == undefined) {
            let secret_name = wasm.hash_name(name.value, username());
            let secret_value = wasm.encrypt_hex(secret.value, storage_key());
            let secret_name_encrypted = wasm.encrypt_hex(name.value, storage_key());
            api_fetch(async function (json) {
                if (json.error == false) {
                    reload_secrets();
                    gen_tokens();
                    name.value = "";
                    secret.value = "";
                } else {
                    alert("API error: " + json.error);
                }
            }, "data/update", { secret_name, secret_value, secret_name_encrypted, ...login_data() });
        } else {
            alert("Name for secret already exists")
        }
    } else {
        alert("Name or secret empty");
    }
}

function remove_token(name) {
    if (name != "") {
        if (secrets[name] != undefined) {
            let secret_name = wasm.hash_name(name, username());
            api_fetch(async function (json) {
                if (json.error == false) {
                    delete secrets[name];
                    gen_tokens();
                } else {
                    alert("API error: " + json.error);
                }
            }, "data/delete", { secret_name, ...login_data() });
        } else {
            alert("Name does not exists")
        }
    } else {
        alert("Name empty");
    }
}

function do_reload_tokens() {
    reload_tokens(true);
    setInterval(reload_tokens, 1000);
}

function reload_tokens(force = false) {
    const left = 30 - (Date.now() / 1000) % 30;
    document.getElementById("timeleft").value = Math.round(left);
    if (left > 29 || force) {
        gen_tokens();
    }
}

function gen_tokens() {
    const tokens = document.getElementById("tokens");
    tokens.innerHTML = "";
    for (const key in secrets) {
        const li = document.createElement("li");
        li.innerHTML = key + ": " + wasm.gen_token(secrets[key], BigInt(Date.now())) + "&nbsp;";
        const button = document.createElement("button");
        button.innerText = lang.delete;
        button.addEventListener("click", function () {
            if (confirm("Delete secret")) {
                remove_token(key);
            }
        });
        li.appendChild(button);
        tokens.appendChild(li);
    }
}

document.getElementById("logout").onclick = function () {
    api_fetch(async function (json) { }, "user/logout", login_data());
    sessionStorage.clear();
    location.href = "../";
};

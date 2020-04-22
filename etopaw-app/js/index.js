import { load, api_fetch, raw_fetch, login_data } from "./common.js";

let wasm;
let storage;

load(async function (temp_wasm) {
    wasm = temp_wasm;
    await reload_storage();
    do_reload_tokens();
    document.getElementById("add").addEventListener("click", add_token);
});

async function reload_storage() {
    storage = await raw_fetch(async function (resp) {
        const buffer = await resp.arrayBuffer();
        const storage = JSON.parse(wasm.decrypt_storage(new Uint8Array(buffer), "12345678901234567890123456789012"));
        if (storage.secrets == undefined) {
            storage.secrets = {};
        }
        return storage;
    }, "data/get_secure", login_data());
    reload_tokens(true);
}

async function update_storage() {
    await api_fetch(async function (json) {
        if (json.success != true) {
            alert("Could not update secure storage: " + json.error);
        }
    }, "data/set_secure", login_data(), wasm.encrypt_storage(JSON.stringify(storage), "12345678901234567890123456789012"));
    reload_storage();
}

function add_token() {
    const name = document.getElementById("name");
    const secret = document.getElementById("secret");
    if (name.value != "" && secret.value != "") {
        if (storage.secrets[name.value] == undefined) {
            storage.secrets[name.value] = secret.value;
            update_storage();
        } else {
            alert("Name for secret already exists")
        }
    } else {
        alert("Name or secret empty");
    }
}

function remove_token(name) {
    delete storage.secrets[name];
    update_storage();

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
    for (const key in storage.secrets) {
        const li = document.createElement("li");
        li.appendChild(document.createTextNode(key + ": " + wasm.gen_token(storage.secrets[key], BigInt(Date.now())) + "<button>TODO: DELETE</button>"));
        tokens.appendChild(li);
    }
}

document.getElementById("logout").onclick = function () {
    sessionStorage.removeItem("username");
    sessionStorage.removeItem("token");
    location.href = "./login.html";
    api_fetch(async function (json) { }, "user/logout", login_data());
};

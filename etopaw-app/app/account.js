import { load, api_fetch, login_data, storage_key, load_secrets, confirm, alert, alert_error, username, logout, lang, vue, reload_storage_data, storage_data, parse_storage_data } from "../js/common.js";

let wasm;

const key = document.getElementById("key")
const new_username = document.getElementById("new_username");
const new_password = document.getElementById("new_password");
const repeat_new_password = document.getElementById("repeat_new_password");
const new_key = document.getElementById("new_key");
const repeat_new_key = document.getElementById("repeat_new_key");
const change_username_btn = document.getElementById("change_username_btn");
const change_password_btn = document.getElementById("change_password_btn");
const change_key_btn = document.getElementById("change_key_btn");
const delete_user_btn = document.getElementById("delete_user_btn");
const logout_el = document.getElementById("logout");
const delete_user_check = document.getElementById("delete_user_check");
const export_database_btn = document.getElementById("export_database_btn");
const download_export = document.getElementById("download_export");
const import_database_btn = document.getElementById("import_database_btn");
const import_file = document.getElementById("import_file");

let selected_import_file;

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
    document.getElementById("change_key").onsubmit = function () {
        change_key();
        return false;
    };
    delete_user_btn.addEventListener("click", function () {
        confirm(lang.delete_user_qm, delete_user, "<input autocomplete=\"off\" id=\"delete_verify_username\" class=\"form-control ten-top-margin\" type=\"text\" placeholder=\"" + lang.username + "\">");
        return false;
    });
    delete_user_check.addEventListener("click", function () {
        delete_user_btn.hidden = !delete_user_check.checked;
    });
    export_database_btn.addEventListener("click", function () {
        reload_storage_data(wasm)
            .catch(err => alert_error(err))
            .then(_ => {
                download_export.setAttribute("href", "data:application/etopa-database;charset=utf-8," + encodeURIComponent(localStorage.getItem("storage_data")));
                download_export.setAttribute("download", username() + ".edb");
                download_export.click();
            });
        return false;
    });
    import_file.addEventListener("change", event => {
        selected_import_file = event.target.files[0];
        if (selected_import_file)
            import_database_btn.disabled = false;
    });
    import_database_btn.addEventListener("click", function () {
        if (!selected_import_file)
            return alert_error(lang.no_file_selected);
        confirm(lang.import_database_qm + "\n" + lang.import_overwrites_database, import_database, "<input autocomplete=\"off\" id=\"import_key\" class=\"form-control ten-top-margin\" type=\"password\" placeholder=\"" + lang.key + "\">");
        return false;
    });
}, true);

async function import_database() {
    let buffer = await selected_import_file.arrayBuffer();
    let import_key = document.getElementById("import_key").value;
    try {
        let import_data = selected_import_file.text();
        let secrets = parse_storage_data(wasm, import_data, import_key);
        console.log(secrets);
    } catch (err) {
        console.log(err);
        alert_error(lang.invalid_key);
    }
}

async function change_username() {
    if (wasm.hash_key(key.value) != storage_key()) {
        return alert_error(lang.invalid_key);
    }
    disabled(true);
    await api_fetch(async function (json) {
        if (json.error == false) {
            logout_el.innerText = logout_el.innerText.replace("(" + username() + ")", "(" + new_username.value + ")");
            localStorage.setItem("username", new_username.value);
            clear_inputs();
            alert(lang.username_changed);
        } else {
            alert_error(lang.api_error_cs + json.error);
        }
        disabled(false);
    }, "user/change_username", { newusername: new_username.value, ...login_data() });
}

async function change_password() {
    if (new_password.value != repeat_new_password.value) {
        return alert_error(lang.passwords_no_match);
    } else if (wasm.hash_key(key.value) != storage_key()) {
        return alert_error(lang.key_incorrect);
    }
    disabled(true);
    await api_fetch(async function (json) {
        if (json.error == false) {
            clear_inputs();
            alert(lang.password_changed);
        } else {
            alert_error(lang.api_error_cs + json.error);
        }
        disabled(false);
    }, "user/change_password", { newpassword: wasm.argon2_hash(new_password.value), ...login_data() });
}

async function change_key() {
    if (new_key.value != repeat_new_key.value) {
        return alert_error(lang.passwords_no_match);
    } else if (wasm.hash_key(key.value) != storage_key()) {
        return alert_error(lang.key_incorrect);
    }
    const new_storage_key = wasm.hash_key(new_key.value);
    try {
        const secrets = await load_secrets(wasm);
        let sort = "";
        for (const key in secrets) {
            sort += wasm.hash_name(key) + ',';
        }
        const new_storage = wasm.serialize_storage(secrets, sort, new_storage_key);
        disabled(true);
        await api_fetch(async function (json) {
            if (json.error == false) {
                localStorage.setItem("storage_data", new_storage);
                sessionStorage.setItem("storage_key", new_storage_key);
                clear_inputs();
                alert(lang.key_changed);
            } else {
                alert_error(lang.api_error_cs + json.error);
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
    if (wasm.hash_key(key.value) != storage_key()) {
        return alert_error(lang.key_incorrect);
    } else if (document.getElementById("delete_verify_username").value != username()) {
        return alert_error(lang.incorrect_username);
    }
    disabled(true);
    await api_fetch(async function (json) {
        if (json.error == false) {
            sessionStorage.clear();
            localStorage.clear();
            clear_inputs();
            location.href = "../";
        } else {
            alert_error(lang.api_error_cs + json.error);
        }
        disabled(false);
    }, "user/delete", login_data());
}

function clear_inputs() {
    key.value = "";
    new_username.value = "";
    new_password.value = "";
    repeat_new_password.value = "";
    new_key.value = "";
    repeat_new_key.value = "";
}

function disabled(disable) {
    document.getElementById("change_username").disabled = disable;
    document.getElementById("change_password").disabled = disable;
    document.getElementById("change_key").disabled = disable;
    document.getElementById("delete_user").disabled = disable;
    key.disabled = disable;
    new_username.disabled = disable;
    new_password.disabled = disable;
    repeat_new_password.disabled = disable;
    new_key.disabled = disable;
    repeat_new_key.disabled = disable;
    change_username_btn.disabled = disable;
    change_password_btn.disabled = disable;
    change_key_btn.disabled = disable;
    delete_user_btn.disabled = disable;
}

logout(logout_el);

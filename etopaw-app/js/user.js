import { load, api_fetch, login_data, username, storage_key, load_secrets } from "./common.js";

let wasm;

load(async function (temp_wasm) {
    wasm = temp_wasm;
    document.getElementById("form").onsubmit = change_user;
    document.getElementById("delete").onsubmit = delete_user;
});

function change_user() {
    const new_username = document.getElementById("new_username").value;
    const password = document.getElementById("password").value;
    const new_password = document.getElementById("new_password").value;
    const repeat_new_password = document.getElementById("repeat_new_password").value;
    if (new_password != repeat_new_password) {
        alert("Passwords do not match");
        return false;
    } else if (wasm.hash_key(password, username()) != storage_key()) {
        alert("Current password incorrect");
        return false;
    }
    const new_password_hash = wasm.hash_password(new_password != "" ? new_password : password, new_username != "" ? new_username : username());
    const key = wasm.hash_key(new_password != "" ? new_password : password, new_username != "" ? new_username : username());
    load_secrets(wasm).then(secrets => {
        api_fetch(async function (json) {
            if (json.error == false) {
                if (new_password != "" || new_username != "") {
                    sessionStorage.setItem("storage_key", key);
                }
                if (new_username != "") {
                    sessionStorage.setItem("username", new_username);
                }
                document.getElementById("result").innerText = "Successfully changed";
            } else {
                document.getElementById("result").innerText = json.error;
            }
        }, "user/update", new_username != "" ? { new_username, password: new_password_hash, ...login_data() } : { password: new_password_hash, ...login_data() }, wasm.serialize_storage(secrets, key, username()));
    })
    return false;
}

function delete_user() {
    const password = document.getElementById("delete_password").value;
    if (wasm.hash_key(password, username()) != storage_key()) {
        alert("Current password incorrect");
        return false;
    }
    api_fetch(async function (json) {
        if (json.error == false) {
            sessionStorage.clear();
            location.href = "./login.html";
        } else {
            document.getElementById("result").innerText = json.error;
        }
    }, "user/delete", login_data());
    return false;
}

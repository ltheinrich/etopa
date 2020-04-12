import * as wasm from "etopaw";
wasm.set_panic_hook();

async function fetch_api(url = '', data = {}) {
    const resp = await fetch(`https://localhost:4490${url}`, {
        method: 'POST',
        cache: 'no-cache',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(data)
    });
    return resp.json();
}

document.getElementById('register').addEventListener('click', function () { user_register(); });
document.getElementById('login').addEventListener('click', function () { user_login(); });
document.getElementById('delete').addEventListener('click', function () { user_delete(); });

function user_register() {
    fetch_api('/user/create', { username: document.getElementById('username').value, password: wasm.hash_password(document.getElementById('password').value) })
        .then((resp) => {
            if ("token" in resp) {
                document.getElementById('result').value = "";
                document.getElementById('token').value = resp["token"];
            } else {
                document.getElementById('token').value = "";
                document.getElementById('result').value = resp["error"];
            }
        });
}

function user_login() {
    fetch_api('/user/login', { username: document.getElementById('username').value, password: wasm.hash_password(document.getElementById('password').value) })
        .then((resp) => {
            if ("token" in resp) {
                document.getElementById('result').value = "";
                document.getElementById('token').value = resp["token"];
            } else {
                document.getElementById('token').value = "";
                document.getElementById('result').value = resp["error"];
            }
        });
}

function user_delete() {
    fetch_api('/user/delete', { username: document.getElementById('username').value, token: document.getElementById('token').value })
        .then((resp) => {
            document.getElementById('token').value = "";
            document.getElementById('result').value = resp["success" in resp ? "success" : "error"];
        });
}


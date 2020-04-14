import { load, fetch_api } from "./common.js";

if (sessionStorage.getItem("username") != null && sessionStorage.getItem("token") != null) {
    location.href = "./index.html";
}

load().then(wasm => {
    document.getElementById("form").onsubmit = function () {
        const username = document.getElementById("username").value;
        const password = wasm.hash_password(document.getElementById("password").value, username);
        fetch_api("user/register", {
            username, password
        })
            .then((resp) => {
                if ("token" in resp) {
                    sessionStorage.setItem("username", username);
                    sessionStorage.setItem("token", resp.token);
                    document.getElementById("result").innerText = "Registration successful";
                    location.href = "./index.html";
                } else {
                    document.getElementById("result").innerText = resp.error;
                }
            });
        return false;
    };
});

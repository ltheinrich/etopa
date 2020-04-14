import { fetch_api } from "./common.js";

function display() {
    document.getElementById("login").hidden = sessionStorage.getItem("username") != null && sessionStorage.getItem("token") != null;
    document.getElementById("register").hidden = sessionStorage.getItem("username") != null && sessionStorage.getItem("token") != null;
    document.getElementById("logout").hidden = sessionStorage.getItem("username") == null || sessionStorage.getItem("token") == null;
}

display();
document.getElementById("logout").onclick = function () {
    fetch_api("user/logout", {
        username: sessionStorage.getItem("username"), token: sessionStorage.getItem("token")
    })
        .then((resp) => {
            if ("success" in resp) {
                sessionStorage.removeItem("username");
                sessionStorage.removeItem("token");
                display();
                location.href = "./index.html";
            } else {
                document.getElementById("result").innerText = resp.error;
            }
        });
};

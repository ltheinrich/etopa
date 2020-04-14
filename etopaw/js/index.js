function display() {
    document.getElementById("login").hidden = sessionStorage.getItem("username") != null && sessionStorage.getItem("token") != null;
    document.getElementById("register").hidden = sessionStorage.getItem("username") != null && sessionStorage.getItem("token") != null;
    document.getElementById("logout").hidden = sessionStorage.getItem("username") == null || sessionStorage.getItem("token") == null;
}

display();
document.getElementById("logout").onclick = function () {
    sessionStorage.removeItem("username");
    sessionStorage.removeItem("token");
    display();
};

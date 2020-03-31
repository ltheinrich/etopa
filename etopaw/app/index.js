import * as wasm from "etopaw";
wasm.set_panic_hook();

fetch('/index.html')
    .then((response) => {
        response.text().then(function (text) { wasm.greet(text.split('\n')[0]); });
    });

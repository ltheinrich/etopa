#!/bin/bash

wasm-pack build --debug
(cd app && npm install app)
(cd app && npm run start)

#!/bin/bash

wasm-pack build --debug
(cd app && npm install)
(cd app && npm run start)

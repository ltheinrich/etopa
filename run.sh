#!/bin/bash

if [ "$#" -ne 1 ]; then
  echo "./run.sh api|android|wasm|web"
elif [ $1 == "api" ]; then
  cargo run -p etopai
elif [ $1 == "android" ]; then
  JNI_LIBS=etopan-app/app/src/main/jniLibs
  if ! [[ "$PATH" == ?(*:)"$HOME/.NDK/18-x86/bin"?(:*) ]]; then
    export PATH="$PATH:$HOME/.NDK/18-x86/bin"
  fi
  cargo build -p etopan --target i686-linux-android
  rm -rf $JNI_LIBS && mkdir -p $JNI_LIBS/x86
  cp target/i686-linux-android/debug/libetopan.so $JNI_LIBS/x86/libetopan.so
elif [ $1 == "wasm" ]; then
  wasm-pack build --dev etopaw
elif [ $1 == "web" ]; then
  (cd etopaw-app && npm install)
  (cd etopaw-app && npm run start)
else
  echo "./run.sh api|android|wasm|web"
fi

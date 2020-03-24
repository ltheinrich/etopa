#!/bin/bash
JNI_LIBS=etopan-app/app/src/main/jniLibs

if ! [[ "$PATH" == ?(*:)"$HOME/.NDK/18-x86/bin"?(:*) ]]; then
    export PATH="$PATH:$HOME/.NDK/18-x86/bin"
fi

if ! [[ "$PATH" == ?(*:)"$HOME/.NDK/arm64/bin"?(:*) ]]; then
    export PATH="$PATH:$HOME/.NDK/arm64/bin"
fi

cargo build --release -p etopan --target aarch64-linux-android
cargo build --release -p etopan --target i686-linux-android

rm -rf $JNI_LIBS
mkdir $JNI_LIBS
mkdir $JNI_LIBS/arm64-v8a
mkdir $JNI_LIBS/x86

cp target/aarch64-linux-android/release/libetopan.so $JNI_LIBS/arm64-v8a/libetopan.so
cp target/i686-linux-android/release/libetopan.so $JNI_LIBS/x86/libetopan.so

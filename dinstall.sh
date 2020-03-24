#!/bin/bash
JNI_LIBS=etopan-app/app/src/main/jniLibs

if ! [[ "$PATH" == ?(*:)"$HOME/.NDK/18-x86/bin"?(:*) ]]; then
    export PATH="$PATH:$HOME/.NDK/18-x86/bin"
fi

cargo build -p etopan --target i686-linux-android

rm -rf $JNI_LIBS
mkdir $JNI_LIBS
mkdir $JNI_LIBS/x86

cp target/i686-linux-android/release/libetopan.so $JNI_LIBS/x86/libetopan.so

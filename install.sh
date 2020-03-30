#!/bin/bash
JNI_LIBS=etopan-app/app/src/main/jniLibs

if ! [[ "$PATH" == ?(*:)"$HOME/.NDK/arm64/bin"?(:*) ]]; then
    export PATH="$PATH:$HOME/.NDK/arm64/bin"
fi

if ! [[ "$PATH" == ?(*:)"$HOME/.NDK/arm/bin"?(:*) ]]; then
    export PATH="$PATH:$HOME/.NDK/arm/bin"
fi

cross build --release -p etopan --target aarch64-linux-android
cross build --release -p etopan --target armv7-linux-androideabi

rm -rf $JNI_LIBS
mkdir $JNI_LIBS
mkdir $JNI_LIBS/arm64-v8a
mkdir $JNI_LIBS/armeabi-v7a

cp target/aarch64-linux-android/release/libetopan.so $JNI_LIBS/arm64-v8a/libetopan.so
cp target/armv7-linux-androideabi/release/libetopan.so $JNI_LIBS/armeabi-v7a/libetopan.so

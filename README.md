[![Build Status](https://github.com/ltheinrich/etopa/workflows/CI/badge.svg)](https://github.com/ltheinrich/etopa/actions?query=workflow%3ACI)
[![Matrix](https://img.shields.io/matrix/etopa:matrix.org?label=Matrix)](https://matrix.to/#/!SuZAJrFcmgupnUNURc:matrix.org?via=matrix.org)
[![Discord](https://img.shields.io/discord/694617177717735457?label=Discord)](https://discord.gg/ZWFNBgR)

# Etopa
### Time-based one-time password authenticator (2FA)
Etopa is a two-factor-authentication app, which runs as a web server and can be accessed using a web browser or using an app.
It is currently under development. If you want to contribute and encounter problems, feel free to ask me for help anytime.

### Build
Add rustup targets
> rustup target add x86_64-unknown-linux-musl aarch64-linux-android armv7-linux-androideabi wasm32-unknown-unknown

Install cross
> cargo install cross

Install wasm-pack
> curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

Download Android NDK (Side by side) and Android SDK Build-Tools using Android Studio (change ANDROID_BT variable in Makefile if necessary)

Create NDK toolchains (change ndk path if necessary)
> mkdir ~/.android/ndk

> ~/.android/sdk/ndk/21.3.6528147/build/tools/make_standalone_toolchain.py --api 30 --arch arm64 --install-dir ~/.android/ndk/arm64

> ~/.android/sdk/ndk/21.3.6528147/build/tools/make_standalone_toolchain.py --api 30 --arch arm --install-dir ~/.android/ndk/arm

Download bundletool
> wget -O ~/.bundletool-all.jar ht<span></span>tps://github.com/google/bundletool/releases/latest/download/bundletool-all-1.3.0.jar

Install [gominify](https://github.com/tdewolff/minify/releases)

Debian/Ubuntu
> sudo apt install minify

Fedora/CentOS
> sudo dnf install golang-github-tdewolff-minify

Build using Makefile
> make

API server: target/build/etopa

Android APK: target/build/etopa-unsigned.apk

Web archive: target/build/etopa.tar.xz

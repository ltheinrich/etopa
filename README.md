[![Build Status](https://ltheinrich.de/etopa/workflows/CI/badge.svg)](https://ltheinrich.de/etopa/actions?query=workflow%3ACI)
[![Matrix](https://img.shields.io/matrix/etopa:matrix.org?label=Matrix)](https://matrix.to/#/!SuZAJrFcmgupnUNURc:matrix.org?via=matrix.org)
[![Discord](https://img.shields.io/discord/694617177717735457?label=Discord)](https://discord.gg/ZWFNBgR)

# Etopa
### Time-based one-time password authenticator (2FA)
Etopa is a two-factor-authentication app, which runs as a web server and can be accessed using a web browser or using an Android app.
I would consider it stable to use, but there are still many functions to yet be implemented.

<hr>

### Etopa<span></span>.de instance
You can use the [Etopa.de instance](https://etopa.de/) or host your own.

### Download Android app
F-Droid: Add repository https://fdroid.ltheinrich.de and search for Etopa (fingerprint B90FC7691EC5BE977DCBBCB18C3984C794CCAFA5BB8712ED2D64F9FD8703B636)

Google Play Store: [Etopa 2FA](https://play.google.com/store/apps/details?id=de.ltheinrich.etopa)

Amazon Appstore: [Etopa](http://www.amazon.com/gp/mas/dl/android?p=de.ltheinrich.etopa)

Samsung Galaxy Store: [Etopa](https://apps.samsung.com/gear/appDetail.as?appId=de.ltheinrich.etopa)

<hr>

### Build
Clone repo
> git clone https://ltheinrich.de/etopa && cd etopa

Configure
> ./configure

Build
> ANDROID_UAPK_FILE=etopa.apk make

API server: `target/build/etopa`

API server (Fedora package): `target/build/etopa.rpm`

API server (Debian package): `target/build/etopa.deb`

API server (native/optimized): `target/build/extra/etopa-native`

Android APK: `target/build/etopa.apk`

Web archive: `target/build/etopa.tar.xz`

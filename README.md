[![Build Status](https://ltheinrich.de/etopa/workflows/CI/badge.svg)](https://ltheinrich.de/etopa/actions?query=workflow%3ACI)

# Etopa
### Time-based one-time password authenticator (2FA)
Etopa is a two-factor-authentication app, which runs as a web server and can be accessed using a web browser or using an Android app. Feel free to suggest feature implementations or report bugs by creating an [Issue](https://ltheinrich.de/etopa/issues) on GitHub.

<hr>

### Etopa<span></span>.de instance
You can use the [Etopa.de instance](https://etopa.de/) or [host your own](https://ltheinrich.de/etopa/wiki/Install-server).

### Download Android app
F-Droid: Add [repository](https://ltheinrich.de/fdroid/repo?fingerprint=B90FC7691EC5BE977DCBBCB18C3984C794CCAFA5BB8712ED2D64F9FD8703B636) and search for Etopa

Google Play Store: [Etopa 2FA](https://play.google.com/store/apps/details?id=de.ltheinrich.etopa)

Amazon Appstore: [Etopa](https://www.amazon.com/gp/mas/dl/android?p=de.ltheinrich.etopa)

Samsung Galaxy Store: [Etopa](https://apps.samsung.com/gear/appDetail.as?appId=de.ltheinrich.etopa)

<hr>

### Build
Clone repo
> git clone https://ltheinrich.de/etopa && cd etopa

Configure
> ./configure

Build
> DEBUG_SIGN=y make

API server: `target/build/etopa`

Android APK: `target/build/etopa.apk`

Web archive: `target/build/etopa.tar.xz`

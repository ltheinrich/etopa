[![CI](https://github.com/ltheinrich/etopa/actions/workflows/ci.yml/badge.svg)](https://github.com/ltheinrich/etopa/actions/workflows/ci.yml)

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

### Docker

#### Configuration

You can expose the container ports `80`, `443` and `4490`.
On port `80`/`443` (HTTP/HTTPS) nginx serves as a reverse proxy for the backend and as a web server for the frontend.
If you only need the Etopa backend, you can instead use port `4490` (HTTP).

Configuration file paths relative to the volume directory for `/etopa/`
<br>Etopa server/backend: `etopa.conf`
<br>Etopa frontend: `config.js`
<br>nginx: `nginx.conf`
<br>TLS certificate/key/fullchain/dhparam: `cert.pem`/`privkey.pem`/`fullchain.pem`/`dhparam.pem`
<br>If there are no configuration files provided, the default will be used.

#### Docker Compose
Download `docker-compose.yml`
> curl -o docker-compose.yml https://raw.githubusercontent.com/ltheinrich/etopa/master/docker/docker-compose.yml

Start Etopa
> docker compose up -d

Unless you modify the `docker-compose.yml` the directory `./etopa/` will be created. User data will be stored in `./etopa/data/` and configuration files (`etopa.conf` for the backend and `config.js` for the frontend) can be placed directly in `./etopa/` (using no/default configuration files works as well).

#### Manually
Pull image `ltheinrich/etopa:latest`
> docker pull ltheinrich/etopa:latest

Start Etopa
> docker run -d --name etopa -v ./etopa/:/etopa/ --restart always -p 127.0.0.1:8080:80 -p 127.0.0.1:8443:443 -p 127.0.0.1:4490:4490 ltheinrich/etopa:latest

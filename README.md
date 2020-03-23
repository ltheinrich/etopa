![Build Status](https://github.com/ltheinrich/etopa/workflows/Rust/badge.svg)

# Etopa
### Time-based one-time password authenticator (2FA)
Etopa is a two-factor-authentication app, which runs as a web server and can be accessed using a web browser or using an app.
It is currently under development.

## Kompilieren
Requirements
 - Git
 - Rust
 - Cargo

Clone Git repository
> git clone https://github.com/ltheinrich/etopa && cd etopa

Compile using Cargo
> cargo build --release

The executable is `target/release/etopa`

#!/bin/sh

cargo update
cargo clean
mkdir -p target/bin

cross build --release --target x86_64-unknown-linux-musl
cp target/x86_64-unknown-linux-musl/release/etopa target/bin/x86_64-linux-etopa

cross build --release --target armv7-unknown-linux-musleabihf
cp target/armv7-unknown-linux-musleabihf/release/etopa target/bin/armv7-linux-etopa

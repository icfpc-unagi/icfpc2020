#!/usr/bin/env bash

set -eux

cargo build --release --offline --bin=app
cp ./target/release/app app
rm -rf ./target

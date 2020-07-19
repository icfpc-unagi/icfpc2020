#!/usr/bin/env bash

set -eux

cargo build --release --offline --bin=cui
cp ./target/release/cui app
rm -rf ./target

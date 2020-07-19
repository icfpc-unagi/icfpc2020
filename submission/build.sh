#!/usr/bin/env bash

set -eux

cargo build --release --offline --bin=chokudAI
cp ./target/release/chokudAI app
rm -rf ./target

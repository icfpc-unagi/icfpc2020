#!/usr/bin/env bash

set -eux

source ./AI

cargo build --release --offline --bin="${AI_NAME}"
cp "./target/release/${AI_NAME}" app
rm -rf ./target

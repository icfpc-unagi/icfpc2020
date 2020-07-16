#!/usr/bin/env bash

set -eux
shopt -s dotglob

cd "$(dirname "${BASH_SOURCE}")/.."

mkdir -p build/submission

pushd build/submission
for f in *; do
    if [ "${f}" == '.git' ]; then
        continue
    fi
    rm -rf "${f}"
done
popd

cp -a app build/submission/app
cp -a Cargo.toml build/submission/
cp -a submission/* build/submission/

pushd build/submission
cargo vendor
popd

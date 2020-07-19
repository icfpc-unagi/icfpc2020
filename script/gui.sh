#!/usr/bin/env bash

set -eux

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE}")"/..; pwd)"

pushd "${ROOT_DIR}"
mkdir -p out
pushd go
go build -o ../out/web ./cmd/web
popd
popd
GUI=1 "${ROOT_DIR}/out/web" --logtostderr -- "$@"

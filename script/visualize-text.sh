#!/usr/bin/env bash

set -eux

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE}")"/..; pwd)"

pushd "${ROOT_DIR}"
mkdir -p out
pushd go
go build -o ../out/visualizer ./cmd/visualizer
popd
popd
GUI=1 "${ROOT_DIR}/out/visualizer" --logtostderr -- "cat $1"

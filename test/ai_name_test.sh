#!/usr/bin/env bash

set -eux

cd "$(dirname "${BASH_SOURCE}")/.."

source ./AI

if [ ! -f "src/bin/${AI_NAME}.rs" ]; then
	echo "src/bin/${AI_NAME}.rs does not exist!"
	exit 1
fi

args=(./app)
if [ "${#AI_FLAGS[*]}" -ne 0 ]; then
	args+=("${AI_FLAGS[@]}")
fi

echo "${args[@]}"

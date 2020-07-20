#!/usr/bin/env bash

set -eux

export ICFPC_API_HOST=$1
export ICFPC_API_KEY=$2
export JUDGE_SERVER=1

source ./AI

args=(./app)
if [ "${#AI_FLAGS[*]}" -ne 0 ]; then
	args+=("${AI_FLAGS[@]}")
fi

"${args[@]}" ${EXTRA_FLAGS:-} "$@" || \
	echo "ERROR: exit_code=$?"

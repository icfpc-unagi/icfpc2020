#!/usr/bin/env bash

set -eux

export ICFPC_API_HOST=$1
export ICFPC_API_KEY=$2
export JUDGE_SERVER=1

source ./AI

./app "${AI_FLAGS[@]}" ${EXTRA_FLAGS:-} "$@" || \
	echo "ERROR: exit_code=$?"

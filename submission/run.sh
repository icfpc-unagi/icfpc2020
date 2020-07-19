#!/usr/bin/env bash

set -eux

export ICFPC_API_HOST=$1
export ICFPC_API_KEY=$2
export JUDGE_SERVER=1

free -h
for f in /sys/fs/cgroup/memory/*; do
	echo $f
	cat $f || echo 'failed to read'
	echo
done
cat /proc/cpuinfo

./target/release/app ${EXTRA_FLASG:-} "$@" || \
	echo "ERROR: exit_code=$?"

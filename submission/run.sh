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

./target/release/cui \
	--init-state ./data/performance_test-init_state.txt \
	${EXTRA_FLASG:-} \
	<./data/performance_test-input.txt || \
	echo "ERROR: exit_code=$?"

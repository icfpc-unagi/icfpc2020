#!/usr/bin/env bash


set -eu

cargo build --release --bin=cui

export RUST_BACKTRACE=full
time ./target/release/cui \
	--init-state data/performance_test-init_state.txt --performance-test \
	< data/performance_test-input.txt

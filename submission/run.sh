#!/bin/sh

./target/release/app "$@" || echo "run error code: $?"

#!/bin/sh

/solution/target/release/app "$@" || echo "run error code: $?"

#!/bin/sh

cargo build --release

seq 1 1000000 | time -p target/release/passwd > /dev/null

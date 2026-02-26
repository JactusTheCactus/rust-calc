#!/usr/bin/env bash
set -uo pipefail
rust() {
	cargo "$@" &> "logs/${@: -1}.log"
}
bin=$PWD/target/debug/calc
rm -rf logs
mkdir -p logs
rust +nightly fmt
rust clippy
rust check
rust build
find logs -empty -delete
"$bin" "$@"

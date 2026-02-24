#!/usr/bin/env bash
set -uo pipefail
rust() {
	cargo "$@" &> "logs/${@: -1}.log"
}
bin=./target/debug/calc
rm -rf logs
mkdir -p logs
rust +nightly fmt
rust clippy
rust check
rust build
$bin "$@"
find logs -empty -delete

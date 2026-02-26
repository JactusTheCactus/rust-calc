#!/usr/bin/env bash
set -uo pipefail
log() {
	echo "logs/$1.log"
}
rust() {
	log="$(log "${@: -1}")"
	cargo "$@" &> "$log"
	echo "$log"
}
rm -rf logs
mkdir -p logs
rust +nightly fmt > /dev/null
clippy="$(rust clippy)"
rust check > /dev/null
if build="$(rust build)"
	then rm "$(log check)"
fi
grep -vxFf "$build" "$clippy" > "$clippy.1"
mv "$clippy.1" "$clippy"
find logs -empty -delete
"$PWD/target/debug/calc" "$@"

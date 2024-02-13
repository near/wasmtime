#!/usr/bin/env bash

set -o pipefail
set -eux

ALL_ARGS=$@

function run_tests() {
	./ci/test-zkasm.sh $(pwd)/$1 $ALL_ARGS
}

run_tests "cranelift/zkasm_data"
run_tests "cranelift/zkasm_data/spectest/i32"
run_tests "cranelift/zkasm_data/spectest/i64"
run_tests "cranelift/zkasm_data/spectest/conversions"
run_tests "cranelift/zkasm_data/benchmarks/fibonacci"
run_tests "cranelift/zkasm_data/benchmarks/sha256"

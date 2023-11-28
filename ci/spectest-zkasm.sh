#!/usr/bin/env bash

set -o pipefail
set -eux

./ci/test-zkasm.sh "cranelift/zkasm_data/spectest/i32"
./ci/test-zkasm.sh "cranelift/zkasm_data/spectest/i64"
./ci/test-zkasm.sh "cranelift/zkasm_data/spectest/conversions"
# TODO(akashin): Move this out to a separate shell script or rename this shell script to be
# more general.
./ci/test-zkasm.sh "cranelift/zkasm_data/benchmarks/fibonacci"

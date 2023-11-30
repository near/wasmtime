#!/usr/bin/env bash

set -o pipefail
set -eux

./ci/test-zkasm.sh "cranelift/zkasm_data/spectest/i32"
./ci/test-zkasm.sh "cranelift/zkasm_data/spectest/i64"
./ci/test-zkasm.sh "cranelift/zkasm_data/spectest/conversions"
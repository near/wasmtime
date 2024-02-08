#!/usr/bin/env bash

set -o pipefail
set -eux

# NB: This might have false-positives locally, but it's worth it for iteration speed.
# If you ever run into a situation when modules are out of date, run this command manually.
if [ ! -d "tests/zkasm/node_modules" ]; then
	npm install --prefix tests/zkasm
fi

# All arguments will be forwarded to `ci/zkasm-result.py`.
ALL_ARGS=$@
# The first argument is expected to be a path to a folder with tests.
TEST_PATH=$1
TEST_RESULTS_PATH=$(mktemp)
npm test --prefix tests/zkasm "${TEST_PATH}/generated" $TEST_RESULTS_PATH
python3 ci/zkasm-result.py $ALL_ARGS < $TEST_RESULTS_PATH

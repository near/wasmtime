#!/bin/bash

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m'

# Flags and default modes
INSTALL_MODE="preinstalled"
ALL_FILES=false

# Parse flags
while [[ "$#" -gt 0 ]]; do
    case $1 in
        --all) ALL_FILES=true; shift ;;
        --install-zkwasm-permanent) INSTALL_MODE="permanent"; shift ;;
        --install-zkwasm) INSTALL_MODE="temporary"; shift ;;
        --help)
            echo "Usage: $0 [OPTIONS] [filename.zkasm]"
            echo "Options:"
            echo "  --all                           Test all zkasm files"
            echo "  --install-zkwasm                Temporarily install and use zkevm-rom"
            echo "  --install-zkwasm-permanent      Permanently install zkevm-rom"
            echo "  --help                          Show this message"
            exit 0
            ;;
        *) break ;;
    esac
done

if [ "$ALL_FILES" = false ] && [ -z "$1" ]; then
    echo "Please provide a filename or use the --all flag to test all files."
    exit 1
fi


BASE_DIR="../wasmtime/cranelift"

case $INSTALL_MODE in
    "permanent")
        echo "Cloning zkevm-rom into ../../ directory..."
        git clone https://github.com/0xPolygonHermez/zkevm-rom/ ../../zkevm-rom > /dev/null 2>&1
        cd ../../zkevm-rom
        npm install
        ;;
    "temporary")
        echo "Cloning zkevm-rom into /tmp directory..."
        git clone https://github.com/0xPolygonHermez/zkevm-rom/ ./tmp/zkevm-rom > /dev/null 2>&1
        cd ./tmp/zkevm-rom
        npm install
        BASE_DIR="../../"
        ;;
    "preinstalled")
        cd ../../zkevm-rom
        ;;
esac



if [ "$ALL_FILES" = true ]; then

    node tools/run-tests-zkasm.js $BASE_DIR/zkasm_data/generated

    exit_code=$?

    # If we used the temporary installation mode, remove the cloned directory
    if [ "$INSTALL_MODE" = "temporary" ]; then
        echo "Removing temporary installation of zkevm-rom..."
        rm -rf /tmp/zkevm-rom
    fi

    exit $exit_code

else
    zkasm_file="$BASE_DIR/$1"
    # Replace all "//" with "/"
    zkasm_file="${zkasm_file//\/\//\/}"

    node tools/run-tests-zkasm.js $zkasm_file

    exit_code=$?

    # If we used the temporary installation mode, remove the cloned directory
    if [ "$INSTALL_MODE" = "temporary" ]; then
        echo "Removing temporary installation of zkevm-rom..."
        rm -rf ./tmp
    fi
    exit $exit_code
fi

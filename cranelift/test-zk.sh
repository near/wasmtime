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
            echo "  --install-zkwasm                Temporarily install and use zkevm-proverjs"
            echo "  --install-zkwasm-permanent      Permanently install zkevm-proverjs"
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
        echo "Cloning zkevm-proverjs into ../../ directory..."
        git clone https://github.com/0xPolygonHermez/zkevm-proverjs/ ../../zkevm-proverjs > /dev/null 2>&1
        cd ../../zkevm-proverjs
        ;;
    "temporary")
        echo "Cloning zkevm-proverjs into /tmp directory..."
        git clone https://github.com/0xPolygonHermez/zkevm-proverjs/ /tmp/zkevm-proverjs > /dev/null 2>&1
        cd /tmp/zkevm-proverjs
        BASE_DIR="../../"
        ;;
    "preinstalled")
        cd ../../zkevm-proverjs
        ;;
esac

git checkout training > /dev/null 2>&1

if [ "$ALL_FILES" = true ]; then
    PASSED=0
    FAILED=0

    for zkasm_file in $BASE_DIR/data/*.zkasm; do
        FILENAME=$(basename $zkasm_file)
        echo -n "Testing $FILENAME... "

        OUTPUT=$(node test/zkasmtest.js $zkasm_file 2>&1)
        if echo "$OUTPUT" | grep -q "cntSteps: [0-9]\+"; then
            echo -e "${GREEN}OK. cntSteps = $(echo "$OUTPUT" | grep "cntSteps: [0-9]\+" | awk '{print $2}')${NC}"
            ((PASSED++))
        else
            echo -e "${RED}ZKWASM ERROR${NC}"
            ((FAILED++))
        fi
    done

    echo -e "\n${GREEN}$PASSED files passed${NC}, ${RED}$FAILED files failed${NC}"

    # If we used the temporary installation mode, remove the cloned directory
    if [ "$INSTALL_MODE" = "temporary" ]; then
        echo "Removing temporary installation of zkevm-proverjs..."
        rm -rf /tmp/zkevm-proverjs
    fi

    exit $FAILED

else
    zkasm_file="$BASE_DIR/$1"
    if [ ! -f "$zkasm_file" ]; then
        echo "File $zkasm_file does not exist!"
        echo "Script executed from: ${PWD}"

        B=$(dirname $0)
        echo "Script location: ${B}"

        # If we used the temporary installation mode, remove the cloned directory
        if [ "$INSTALL_MODE" = "temporary" ]; then
            echo "Removing temporary installation of zkevm-proverjs..."
            rm -rf /tmp/zkevm-proverjs
        fi

        exit 1
    fi

    node test/zkasmtest.js $zkasm_file

    # If we used the temporary installation mode, remove the cloned directory
    if [ "$INSTALL_MODE" = "temporary" ]; then
        echo "Removing temporary installation of zkevm-proverjs..."
        rm -rf /tmp/zkevm-proverjs
    fi
fi

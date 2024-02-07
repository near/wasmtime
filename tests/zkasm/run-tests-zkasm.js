/* eslint-disable no-restricted-syntax */
/* eslint-disable import/no-extraneous-dependencies */
/* eslint-disable no-use-before-define */
// const path = require('path');
import path from "node:path";
// const fs = require('fs');
import fs from "node:fs";
// import { require } from "https://deno.land/x/require/mod.ts"
// import { createRequire } from "https://deno.land/std/node/module.ts";
// import { createRequire } from "https://deno.land/std@0.170.0/node/module.ts";
// const require = createRequire(import.meta.url);
// const zkasm = require('@0xpolygonhermez/zkasmcom');
// import zkasm from '@0xpolygonhermez/zkasmcom';
// import zkasm from 'https://raw.githubusercontent.com/0xPolygonHermez/zkasmcom/main/index.js';
import zkasm from "https://esm.sh/gh/0xPolygonHermez/zkasmcom@v1.0.0";

// const smMain = require('@0xpolygonhermez/zkevm-proverjs/src/sm/sm_main/sm_main');
// import smMain from "https://esm.sh/gh/0xPolygonHermez/zkevm-proverjs/src/sm/sm_main/sm_main.js";
import smMain from "https://esm.sh/gh/0xPolygonHermez/zkevm-proverjs@develop/src/sm/sm_main/sm_main.js";

// const {
//     compile,
//     newCommitPolsArray
// } = require('pilcom');

import pilcom from "pilcom";
const newCommitPolsArray = pilcom.newCommitPolsArray;
const compile = pilcom.compile;
// import newCommitPolsArray from "pilcom"
// import compile from "pilcom"

// const buildPoseidon = require('@0xpolygonhermez/zkevm-commonjs').getPoseidon;
// import getPoseidon from "https://esm.sh/gh/0xPolygonHermez/zkevm-commonjs@v2.0.0-fork.5";

// const emptyInput = require('@0xpolygonhermez/zkevm-proverjs/test/inputs/empty_input.json');

// Global paths to build Main PIL to fill polynomials in tests
// Get the URL of the current module
const currentModuleUrl = import.meta.url;
// Create a new URL object pointing to the directory of the current module
const moduleDirUrl = new URL(".", currentModuleUrl);
// Convert the URL object to a file path
const moduleDirPath = moduleDirUrl.pathname;
const pathMainPil = path.join(
    moduleDirPath,
    "node_modules/@0xpolygonhermez/zkevm-proverjs/pil/main.pil",
);
const fileCachePil = path.join(
    moduleDirPath,
    "node_modules/@0xpolygonhermez/zkevm-proverjs/cache-main-pil.json",
);

function value_to_json(key, value) {
    if (typeof value === "bigint") {
        return value.toString();
    }

    // Serialize exceptions by inlining all referenced fields.
    if (value instanceof Error) {
        return JSON.stringify(value, Object.getOwnPropertyNames(value));
    }

    return value;
}

async function main() {
    // Compile pil
    const cmPols = await compilePil();

    // Get all zkasm files
    const pathZkasm = path.join(Deno.cwd(), Deno.args[0]);
    const files = await getTestFiles(pathZkasm);

    // Run all zkasm files
    let testResults = [];
    for (const file of files) {
        if (file.includes("ignore")) continue;

        testResults.push(await runTest(file, cmPols));
    }

    if (Deno.args[1]) {
        const json = JSON.stringify(testResults, value_to_json);
        fs.writeFileSync(Deno.args[3], json);
    } else {
        console.log(testResults);
    }
}

async function compilePil() {
    if (!fs.existsSync(fileCachePil)) {
        const poseidon = await buildPoseidon();
        const {
            F
        } = poseidon;
        const pilConfig = {
            defines: {
                N: 4096,
            },
            namespaces: ["Main", "Global"],
            disableUnusedError: true,
        };
        const p = await compile(F, pathMainPil, null, pilConfig);
        fs.writeFileSync(fileCachePil, `${JSON.stringify(p, null, 1)}\n`, "utf8");
    }

    const pil = JSON.parse(fs.readFileSync(fileCachePil));

    return newCommitPolsArray(pil);
}

// Get all zkasm test files
function getTestFiles(pathZkasm) {
    if (!fs.existsSync(pathZkasm)) {
        return [];
    }

    // check if path provided is a file or a directory
    const stats = fs.statSync(pathZkasm);

    if (!stats.isDirectory()) {
        return [pathZkasm];
    }

    const filesNames = fs
        .readdirSync(pathZkasm)
        .filter((name) => name.endsWith(".zkasm"));

    return filesNames.map((fileName) => path.join(pathZkasm, fileName));
}

// returns true if test succeed and false if test failed
async function runTest(pathTest, cmPols) {
    // Compile rom
    const configZkasm = {
        defines: [],
        allowUndefinedLabels: true,
        allowOverwriteLabels: true,
    };

    const config = {
        debug: true,
        stepsN: 8388608,
        assertOutputs: false,
    };
    try {
        const rom = await zkasm.compile(pathTest, null, configZkasm);
        const result = await smMain.execute(cmPols.Main, emptyInput, rom, config);
        return {
            path: pathTest,
            status: "pass",
            counters: result.counters,
            output: result.output,
            logs: result.logs,
        };
    } catch (e) {
        return {
            path: pathTest,
            status: "runtime error",
            error: e,
        };
    }
}

main();

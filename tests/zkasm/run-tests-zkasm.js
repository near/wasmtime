/* eslint-disable no-restricted-syntax */
/* eslint-disable import/no-extraneous-dependencies */
/* eslint-disable no-use-before-define */
const path = require('path');
const fs = require('fs');
const zkasm = require('@0xpolygonhermez/zkasmcom');
const smMain = require('@0xpolygonhermez/zkevm-proverjs/src/sm/sm_main/sm_main');
const {
    compile,
    newCommitPolsArray
} = require('pilcom');
const buildPoseidon = require('@0xpolygonhermez/zkevm-commonjs').getPoseidon;

const emptyInput = require('@0xpolygonhermez/zkevm-proverjs/test/inputs/empty_input.json');

// Global paths to build Main PIL to fill polynomials in tests
const pathMainPil = path.join(__dirname, 'node_modules/@0xpolygonhermez/zkevm-proverjs/pil/main.pil');
const fileCachePil = path.join(__dirname, 'node_modules/@0xpolygonhermez/zkevm-proverjs/cache-main-pil.json');

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

/**
 * Run this script with `--help` to print docs.
 */
async function main() {
    const argv = require("yargs/yargs")(process.argv.slice(2))
        .command("$0 <path> [outfile]", "the default command runs zkASM tests", (yargs) => {
            yargs.positional("path", {
                describe: "The zkASM file to run or a directory to search for zkASM files.",
                type: "string"
            })
            yargs.positional("outfile", {
                describe: "If provided, results are written to this file. Otherwise they are printed to stdout.",
                type: "string"
            })
        })
        .parse();

    // Run the default command.
    runTestsCmd(argv.path, argv.outfile);
}

/**
 * Executes zkASM stored in files. Expects the following positional command line arguments:
 *
 * @param {string} path - The file or a directory to search for zkASM files.
 * @param {string} [outfile] - If provided, results are written to this file. Otherwise they are
 * printed to stdout.
 */
async function runTestsCmd(path, outfile) {
    // Compile pil
    const cmPols = await compilePil();

    // Get all zkasm files
    const files = await getTestFiles(path);

    // Run all zkasm files
    let testResults = [];
    for (const file of files) {
        if (file.includes('ignore'))
            continue;

        testResults.push(await runTest(file, cmPols));
    }

    if (outfile) {
        const json = JSON.stringify(testResults, value_to_json);
        fs.writeFileSync(outfile, json);
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
                N: 4096
            },
            namespaces: ['Main', 'Global'],
            disableUnusedError: true,
        };
        const p = await compile(F, pathMainPil, null, pilConfig);
        fs.writeFileSync(fileCachePil, `${JSON.stringify(p, null, 1)}\n`, 'utf8');
    }

    const pil = JSON.parse(fs.readFileSync(fileCachePil));

    return newCommitPolsArray(pil);
}

/**
 * Returns the path of all zkasm test files in `pathZkasm`.
 *
 * @param {string} pathZkasm
 * @returns {string[]}
 */
function getTestFiles(pathZkasm) {
    if (!fs.existsSync(pathZkasm)) {
        return [];
    }

    // check if path provided is a file or a directory
    const stats = fs.statSync(pathZkasm);

    if (!stats.isDirectory()) {
        return [pathZkasm];
    }

    const filesNames = fs.readdirSync(pathZkasm).filter((name) => name.endsWith('.zkasm'));

    return filesNames.map((fileName) => path.join(pathZkasm, fileName));
}

/**
 * @returns {Object} Which indicates if the test run succeeded or failed. Contains additional data
 * related to the test run.
 */
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
        }
    } catch (e) {
        return {
            path: pathTest,
            status: "runtime error",
            error: e,
        }
    }
}

main();

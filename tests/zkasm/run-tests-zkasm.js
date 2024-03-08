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
const AssertHelper = require('./assert_helper');
const InstructionTracer = require('./helpers/InstructionTracer');

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
    require("yargs/yargs")(process.argv.slice(2))
        .command({
            command: "$0 <path> [outfile]",
            desc: "the default command runs zkASM tests",
            builder: (yargs) => {
                yargs.positional("path", {
                    describe: "The zkASM file to run or a directory to search for zkASM files.",
                    type: "string"
                })
                yargs.positional("outfile", {
                    describe: "If provided, results are written to this file. Otherwise they are printed to stdout.",
                    type: "string"
                })
            },
            handler: (argv) => runTestsCmd(argv.path, argv.outfile)
        })
        .command({
            command: "profile-instructions <path> [outfile]",
            desc: "profiles instructions executed at runtime, assuming zkASM was instrumented",
            builder: (yargs) => {
                yargs.positional("path", {
                    describe: "The instrumented zkASM file to execute.",
                    type: "string"
                })
                yargs.positional("outfile", {
                    describe: "If provided, results are written to this file. Otherwise they are printed to stdout.",
                    type: "string"
                })
            },
            handler: (argv) => profileInstructions(argv.path, argv.outfile)
        })
        .parse();
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

    // TODO: this output file highly likely should be temporary and deleted once rust filetests
    // will process it.
    const assert_config = {
        outputFile: 'testoutput.json'
    }

    let helper = new AssertHelper(assert_config);

    const config = {
        debug: true,
        stepsN: 8388608,
        assertOutputs: false,
        helpers: [
            helper
        ]
    };
    try {
        const rom = await zkasm.compile(pathTest, null, configZkasm);
        const result = await smMain.execute(cmPols.Main, emptyInput, rom, config);
        helper.dump();
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

/**
 * Executes a zkASM file instrumented for instruction tracing and produces the
 * trace of executed instructions.
 * 
 * @param {string} zkasmFile - Path to the zkASM file.
 * @param {string} [outfile] - Path to a file where output is written. If not
 * given, the trace is written to `stdout`.
 */
async function profileInstructions(zkasmFile, outfile) {
    const configZkasm = {
        defines: [],
        allowUndefinedLabels: true,
        allowOverwriteLabels: true,
    };

    // Construct helper classes.
    const instructionTracer = new InstructionTracer();

    // Compile rom.
    const config = {
        debug: true,
        stepsN: 8388608,
        assertOutputs: false,
        helpers: [
            instructionTracer
        ]
    };
    const cmPols = await compilePil();
    const rom = await zkasm.compile(zkasmFile, null, configZkasm);

    const result = await smMain.execute(cmPols.Main, emptyInput, rom, config);

    console.log("Execution finished");
    if (result.output) {
        console.log(`Output: ${result.output}`)
    }
    if (result.logs && Object.keys(result.logs).length > 0) {
        console.log(result.logs);
    }

    if (outfile) {
        instructionTracer.writeRawTrace(outfile);
    } else {
        console.log(instructionTracer.rawTrace);
    }
}

main();

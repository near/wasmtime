const fs = require("fs");

const dataDump = require("./data-dump");

/**
 * Handles the generation of traces of instructions executed at runtime.
 */
class InstructionTracer {
    /**
     * @param {Object} config
     * @param {boolean} [config.aggregateTrace = false] - An aggregated trace
     * maps each instruction to the number of times it was executed at runtime.
     */
    constructor({ aggregateTrace = false }) {
        // Settings
        this.isAggregatingTrace = aggregateTrace;

        // State
        // Contains executed instructions in order of execution.
        this.rawTrace = [];
        // Maps instructions to their number of executions at runtime.
        this.aggregatedTrace = {};
    }

    setup() {
        // `zkevm-proverjs` requires a `setup` function on helper objects.
    }

    /**
     * @param {Object} ctx - The context passed by `zkevm-proverjs` to helper calls
     * @param {Object} tag - Helper call specific information passed by `zkevm-proverjs`
     * @param {Object} tag.params[0] - expects the identifier as first parameter of
     *   `$${instrumentInst(...)}`
     * 
     * @example
     * // To trace the execution of an `AluRRR` instruction, call
     * // $${instrumentInst(AluRRR)}
     */
    eval_traceInstruction(ctx, tag) {
        const instruction = tag.params[0].varName;
        if (this.isAggregatingTrace) {
            const storedCount = this.aggregatedTrace[instruction];
            let currentCount = storedCount ? storedCount : 0;
            this.aggregatedTrace[instruction] = currentCount + 1;
        } else {
            this.rawTrace.push(instruction);
        }
    }

    /**
     * Writes the trace to `path`.
     * 
     * @param {string} path 
     */
    writeTrace(path) {
        if (typeof path !== "string") {
            // Writing to a descriptor will append instead of replace content,
            // which might result in invalid traces, see 
            // https://nodejs.org/api/fs.html#using-fswritefile-with-file-descriptors
            throw new Error("provide a file name (not descriptor) to write to");
        }

        if (this.isAggregatingTrace) {
            const orderedTrace = dataDump.orderObjectByValues(this.aggregatedTrace);
            dataDump.writeJsonToFile(orderedTrace, path);
        } else {
            // Writing in chunks of 1000 instructions to limit memory usage,
            // as raw traces might grow big.
            dataDump.writeToFileInChunks(this.rawTrace, path, 1000);
        }
    }
}

module.exports = InstructionTracer;

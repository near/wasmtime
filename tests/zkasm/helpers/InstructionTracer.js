const fs = require("fs");

/**
 * Handles the generation of traces of instructions executed at runtime.
 */
class InstructionTracer {
    constructor() {
        // Contains executed instructions in order of execution.
        this.rawTrace = [];
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
        this.rawTrace.push(instruction);
    }

    /**
     * Writes the raw trace to `path`.
     * 
     * @param {string} path 
     */
    writeRawTrace(path) {
        if (typeof path !== "string") {
            // Writing to a descriptor will append instead of replace content,
            // which might result in invalid traces, see 
            // https://nodejs.org/api/fs.html#using-fswritefile-with-file-descriptors
            throw new Error("provide a file name (not descriptor) to write to");
        }
        fs.writeFileSync(path, this.rawTrace.join("\n"));
    }
}

module.exports = InstructionTracer;

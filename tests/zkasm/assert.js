const fs = require("fs");

class InstrumentInst {
    /**
     * @param {Object} config 
     * @param {string} [config.outputFile] - the file to which output will be written
     */
    constructor(config) {
        this.config = config;
        this.results = {};
    }

    setup() {
        // `zkevm-proverjs` requires a `setup` function on helper objects.
    }

    /**
     * 
     * @param {Object} ctx - context of zkasm program
     * @param {Object} tag - information of helper function. 
     * Helper function must be used in format:
     * ${assert_eq(regname, regname, tag)}, for example:
     * ${assert_eq(A, B, test_generated_from_line_3000)}
     */
    eval_assert_eq(ctx, tag) {
        // TODO print verbose error on invalid params
        // TODO: why arg* is list of two integers with last zero and first value in register???
        // Is it chunks? If yes they should be merged instead of ignoring second one
        const arg0 = ctx[tag.params[0].regName][0];
        const arg1 = ctx[tag.params[1].regName][0];
        const testname = tag.params[2].varName;
        this.results[testname] = arg0 === arg1 ? 'pass' : 'fail';
        return 0;
    }

    /**
     * 
     * @param {Object} ctx - context of zkasm program
     * @param {Object} tag - information of helper function. 
     * Helper function must be used in format:
     * ${assert_neq(regname, regname, tag)}, for example:
     * ${assert_neq(A, B, test_generated_from_line_3000)}
     */
        eval_assert_neq(ctx, tag) {
            // TODO print verbose error on invalid params
            // TODO: why arg* is list of two integers with last zero and first value in register???
            // Is it chunks? If yes they should be merged instead of ignoring second one
            const arg0 = ctx[tag.params[0].regName][0];
            const arg1 = ctx[tag.params[1].regName][0];
            const testname = tag.params[2].varName;
            this.results[testname] = arg0 === arg1 ? 'fail' : 'pass';
            return 0;
        }

    /** Writes results to `config.outputFile`. */
    eval_assert_dump(ctx, tag) {
        // TODO throw error if !this.config.outputFile
        let data = JSON.stringify({
            results: this.results,
        });
        // TODO handle errors
        // TODO sort and format data as much as possible here, to make diff on that file meaningful.
        fs.writeFileSync(this.config.outputFile, data);
        return 0;
    }
}

module.exports = InstrumentInst;
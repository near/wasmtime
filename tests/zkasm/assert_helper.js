const fs = require("fs");

class AssertHelper {
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
     * Helper function must be used in format:
     * $${assert_eq(regname, regname, tag)}, for example:
     * $${assert_eq(A, B, test_generated_from_line_3000)}
     * @param {Object} ctx - context of zkasm program
     * @param {Object} tag - information of helper function. 
     */
    eval_assert_eq(ctx, tag) {
        // TODO print verbose error on invalid params
        const equals = ctx[tag.params[0].regName][0] == ctx[tag.params[1].regName][0] &&
                       ctx[tag.params[0].regName][1] == ctx[tag.params[1].regName][1];
        const testname = tag.params[2].varName;
        const actual = BigInt(ctx[tag.params[0].regName][0]) + BigInt(ctx[tag.params[0].regName][1]) * BigInt(2 ** 32);
        this.results[testname] = equals ? 'pass' : actual.toString();
    }

    /**
     * Helper function must be used in format:
     * $${assert_eq(regname, regname, tag)}, for example:
     * $${assert_eq(A, B, test_generated_from_line_3000)}
     * @param {Object} ctx - context of zkasm program
     * @param {Object} tag - information of helper function. 
     */
    eval_assert_neq(ctx, tag) {
        // TODO print verbose error on invalid params
        const equals = ctx[tag.params[0].regName][0] == ctx[tag.params[1].regName][0] &&
                       ctx[tag.params[0].regName][1] == ctx[tag.params[1].regName][1];
        const testname = tag.params[2].varName;
        this.results[testname] = equals ? 'fail' : 'pass';
    }

    /** Writes results to `config.outputFile`. */
    dump() {
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

module.exports = AssertHelper;

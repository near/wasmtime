const fs = require("fs");

class InstrumentInst {
    /**
     * @param {Object} config 
     * @param {string} [config.outputFile] - the file to which output will be written
     */
    constructor(config) {
        this.config = config;
        this.identifiers = [];
    }

    setup() {
        // `zkevm-proverjs` requires a `setup` function on helper objects.
    }

    /**
     * 
     * @param {Object} ctx 
     * @param {Object} tag 
     * @param {Object} tag.params[0] - expects the identifier as first parameter of
     *   `$${instrumentInst(...)}`
     */
    eval_instrumentInst(ctx, tag) {
        // TODO check num params

        let identifier = tag.params[0].varName;
        this.identifiers.push(identifier);
    }

    /** Writes counters to `config.outputFile`. */
    eval_writeInstCounters(ctx, tag) {
        // TODO throw error if !this.config.outputFile
        let data = JSON.stringify({
            identifiers: this.identifiers,
        });
        // TODO handle errors
        // TODO sort and format data as much as possible here, to make diff on that file meaningful.
        fs.writeFileSync(this.config.outputFile, data);
    }
}

module.exports = InstrumentInst;

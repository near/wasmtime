const fs = require('fs');

/**
 * Writes `lines` to a file in chunks to limit memory usage.
 * 
 * @param {string[]} lines - each string in the array is written on its own line
 * @param {string} filePath
 * @param {number} chunkSize 
 */
function writeToFileInChunks(lines, filePath, chunkSize) {
    // If the file already exists, clear it by writing empty string.
    if (fs.existsSync(filePath)) {
        fs.writeFileSync(filePath, "", "utf-8");
    }

    // Append chunks to the file (will be created if it doesn't exist).
    for (let i = 0; i < lines.length; i += chunkSize) {
        let linesToWrite = lines.slice(i, i + chunkSize).join("\n") + "\n";
        fs.appendFileSync(filePath, linesToWrite, "utf-8");
    }
}

/**
 * Serializes `obj` as JSON string and writes that string to `filePath`.
 * 
 * @param {Object} obj 
 * @param {string} filePath 
 */
function writeJsonToFile(obj, filePath) {
    const jsonString = JSON.stringify(obj, null, 4);
    fs.writeFileSync(filePath, jsonString);
}

/**
 * Returns a shallow clone of `obj` with fields ordered by value (descending).
 * 
 * Object key order can be relied upon in recent versions of Node.js, see
 * https://node.green/#ES2015-misc-own-property-order
 * 
 * @param {Object} obj 
 * @returns {Object}
 * @example
 * // {a: 2, b: 4} will be ordered to {b: 4, a: 2}
 */
function orderObjectByValues(obj) {
    const compareFn = ([_key1, val1], [_key2, val2]) => val1 > val2 ? -1:1;
    return Object.entries(obj).sort(compareFn).reduce(
        (accumulator, [key, val]) => {
            accumulator[key] = val;
            return accumulator;
        },
        {}
    );
}

module.exports = {
    orderObjectByValues,
    writeToFileInChunks,
    writeJsonToFile
}

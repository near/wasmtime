const fs = require('fs');

/**
 * Writes `lines` to a file in batches to limit memory usage.
 * 
 * @param {string[]} lines - each string in the array is written on its own line
 * @param {string} filePath
 * @param {number} batchSize 
 */
function writeToFileInBatches(lines, filePath, batchSize) {
    // If the file already exists, clear it by writing empty string.
    if (fs.existsSync(filePath)) {
        fs.writeFileSync(filePath, "", "utf-8");
    }

    // Append batches to the file (will be created if it doesn't exist).
    for (let i = 0; i < lines.length; i += batchSize) {
        let batchedLines = lines.slice(i, i + batchSize).join("\n") + "\n";
        fs.appendFileSync(filePath, batchedLines, "utf-8");
    }
}

module.exports = {
    writeToFileInBatches
}

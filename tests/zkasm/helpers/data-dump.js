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

module.exports = {
    writeToFileInChunks
}

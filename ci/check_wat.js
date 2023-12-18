const fs = require('fs').promises;
const path = require('path');
const wabtPromise = require('wabt')();

async function processWatFile(filePath, wabt) {
    console.log("Processing:", filePath);

    const watString = await fs.readFile(filePath, 'utf8');

    const wasmModule = wabt.parseWat(filePath, watString);
    const binaryWasm = wasmModule.toBinary({}).buffer;

    const importObject = {
        env: {
            assert_eq: (a, b) => {
                if (a !== b) {
                    throw new Error(`Assertion failed in ${filePath}: ${a} !== ${b}`);
                }
                console.log(`Assertion passed: ${a} === ${b}`);
            }
        }
    };

    await WebAssembly.instantiate(binaryWasm, importObject);
    console.log(`Successfully processed: ${filePath}`);
}

async function findWatFiles(dir, wabt) {
    const dirents = await fs.readdir(dir, { withFileTypes: true });
    for (const dirent of dirents) {
        const res = path.resolve(dir, dirent.name);
        if (res.includes('_should_fail_')) {
            continue;
        }
        if (dirent.isDirectory()) {
            await findWatFiles(res, wabt);
        } else if (dirent.isFile() && res.endsWith('.wat')) {
            await processWatFile(res, wabt);
        }
    }
}

async function main() {
    try {
        const wabt = await wabtPromise;
        const dirPath = 'cranelift/zkasm_data';
        await findWatFiles(dirPath, wabt);
    } catch (e) {
        console.error('Error:', e);
        process.exit(1);
    }
}

main();

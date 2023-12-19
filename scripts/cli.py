import argparse
from os import system

def compile_one(file, trace):
    trace = "RUST_LOG=trace " if trace else ""
    system(f"{trace}env UPDATE_EXPECT=1 cargo test --package cranelift-filetests --lib -- test_zkasm::tests::{file} --exact  --nocapture")

def compile_all(trace):
    trace = "RUST_LOG=trace " if trace else ""
    system(f"{trace}env UPDATE_EXPECT=1 cargo test --package cranelift-filetests --lib -- test_zkasm::tests  --nocapture")

def compile(file=None):
    if file:
        compile_one(file, trace=False)
    else:
        compile_all(trace=False)

def trace(file=None):
    if file:
        compile_one(file, trace=True)
    else:
        compile_all(trace=True)

def run(path):
    system(f"npm test --prefix tests/zkasm \"../../{path}\"")

def diff():
    system("./ci/test-all-zkasm.sh")

def update():
    system("./ci/test-all-zkasm.sh --update")

def main():
    parser = argparse.ArgumentParser(description="Developer Tool CLI")
    
    parser.add_argument("-c", "--compile", nargs="?", const=True, help="Compile given test or all files")
    parser.add_argument("-t", "--trace", nargs="?", const=True, help="Same as compile, but print trace of compilation")
    parser.add_argument("-r", "--run", help="Run files in a folder with verbose errors")
    parser.add_argument("-d", "--diff", action="store_true", help="Print diff of current state and state in csv's")
    parser.add_argument("-u", "--update", action="store_true", help="Update csv's")

    parser.epilog = "Path for run shoud be provided assuming you are in wasmtime directory. For example cranelift/zkasm_data/spectest/i64/generated. For compile/trace you should provide name of test in test_zkasm.rs"

    args = parser.parse_args()

    if args.compile is not None:
        if args.compile is True:
            compile()
        else:
            compile(args.compile)
    elif args.run:
        run(args.run)
    elif args.diff:
        diff()
    elif args.update:
        update()
    elif args.trace:
        trace()
    else:
        parser.print_help()

if __name__ == "__main__":
    main()

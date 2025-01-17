
# Compiling `.wat` to `.zkasm` using Cranelift

In this guide, you'll learn how to use `cranelift` to compile `.wat` files into `.zkasm` files.

## Getting Started

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/near/wasmtime
   ```

2. **Update Submodules**:
   ```bash
   git submodule update --init --recursive
   ```

3. **(optional) Setup Python tooling**:

   If you plan to use Python tooling, you would need to install the dev dependencies.
   First install PDM by following https://pdm-project.org/latest/#installation. Then install the
   dependencies with:
   ```bash
   pdm install
   ```

## Compilation Process

To compile a `.wat` file to `.zkasm`:

1. Write your `.wat` code to a file in the `cranelift/zkasm_data` directory.

2. Add the name of your file to the `testcases!` macro in `cranelift/filetests/src/test_zkasm.rs`. For instance:

   ```rust
   testcases! {
       add,
       locals,
       locals_simple,
       counter,
       fibonacci,
       add_func,
       // Add your file name here
   }
   ```

3. Execute the following command to compile your file to `.zkasm`:

   ```bash
   env UPDATE_EXPECT=1 cargo test --package cranelift-filetests --lib -- test_zkasm::tests::<filename> --exact --nocapture
   ```

   For example:

   ```bash
   env UPDATE_EXPECT=1 cargo test --package cranelift-filetests --lib -- test_zkasm::tests::add --exact --nocapture
   ```

   The result of the compilation will be stored in `cranelift/zkasm_data/generated`. Without setting `env UPDATE_EXPECT=1`, it will assert that the generated code matches the code in the `.zkasm` file.

## Testing `.zkasm` Files

Execute the following from the `wasmtime/` directory to run generated programs on ZK ASM processor:

   ```bash
   ./ci/test-zkasm.sh
   ```

   Or, for a specific folder:

   ```bash
   ./ci/test-zkasm.sh <folder>
   ```

   For example:

   ```bash
   ./ci/test-zkasm.sh cranelift/zkasm_data
   ```

## Logging during Compilation

If you wish to compile a `.wat` file with logging (without generating a `.zkasm` file), you can use the following command:

   ```bash
   RUST_LOG=trace cargo run --features=all-arch -p -D cranelift-tools --bin=clif-util wasm --target=zkasm <filepath>
   ```

   For example:

   ```bash
   RUST_LOG=trace cargo run --features=all-arch -p cranelift-tools --bin=clif-util wasm --target=zkasm cranelift/zkasm_data/add.wat 2>trace.txt
   ```

## Python tooling

We use Python to orchestrate the execution of tests and benchmarks on zkAsm interpreter and
processing the execution results. The code lives in `ci/zkasm-result.py`.

We have tools to enforce consistent formatting and check for lint errors in Python files:

```bash
# Show lint errors.
pdm lint

# Format the code.
pdm fmt
```

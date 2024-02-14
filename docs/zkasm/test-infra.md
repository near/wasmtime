# About

We want to build a robust testing infrastructure which will:

- allow easily add new test.
- allow to reuse existing cranelift tests.
- be easy to introduce new type of tests.

We want keep our testing infra flexible and friendly to user. Also, this version is not final, and will be improved. For example, testing the interpreter by running wasm in it is out of scope for now.


# Testing use cases

We have two main use cases for our testing infra:

- CI. We want run our tests in CI to keep quality of code in main branch on high level
- Developing. We want our tests to be easy to run by developer in local machine and see how current version of code works. This use case give us a new requirement -- our testing infra should be fast enough, ideally run in few seconds.

# How to use testing infra

## How test file looks like?

[Filetest docs describe it](https://github.com/bytecodealliance/wasmtime/blob/main/cranelift/filetests/README.md). To run our zkasm test simply add `test run-zkasm` to the header of `.clif` file.

## How to run a test on my machine?

To run a single test, you can use next command:
`cargo run -- test <path-to-test>/testname.clif`

To run multiple tests in a directory, you can use:
`cargo run -- test <path-to-tests-directory>/`


# Test infra implementation

Firstly, our testing infrastructure based on Cranelift testing infrastructure ([filetests](https://github.com/bytecodealliance/wasmtime/blob/main/cranelift/docs/testing.md)). It provides us powerful reusable parts, like test files parsing, and a number of ready to use testcases.

## Files to look at

Our `test run-zkasm` is a new test type in filetests, described in `cranelift/filetests/src/test_run_zkasm.rs`.
Also, it can be useful to take a look at `cranelift/filetests/src/zkasm_codegen.rs` with some helper functions.

## What our test actually do?

Firstly, test infra parses whole `.clif` file (different `.clif` files are parsed independently and don't affect each other). Than, takes all functions and invocations and build one big zkasm program from them, where assertions are compiled as a special assert function, which don't halt execution of the program if assert fails. Instead, it saves results of assert, and in the end this results are taken to build table with results of whole testfile. There can be four types of results: `pass`, `assert_fails`, `compilation_fails`, `run_time_error`. Unfortunately, with our approach (building one zkasm program for one test file), two last statuses will be applied to all tests in file if any of tests in this file get such result. But, we chosed this approach, because starting of zkasm program takes significant time, and we want our tests be fast and easy to use. Function with custom assert is described in `tests/zkasm/run-tests-zkasm.js`

## Future improvements

Probably, we will add some new test types, for example, test type which takes wasm code and executes it in interpreter.

# Decisions to think

## One zkasm program per file

This decision have it's own pros and cons: while it allows to run tests fastly, it leads to not good behaviour in case of compilation and runtime errors. It will be nice to find some better option here.

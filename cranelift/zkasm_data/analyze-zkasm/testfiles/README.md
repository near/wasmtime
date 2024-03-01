# About

Contains files used in tests.

# Updating files containing expected zkASM

Use the CLI, for instance:

```
# In the analyze-zkasm directory run
cargo run -- instrument-inst ./testfiles/simple.wat ./testfiles/simple_instrumented.zkasm
```

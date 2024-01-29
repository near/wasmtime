# About

We want to build a rough MVP of benchmarking infrastructure that:

- Instruments zkASM
- Generates data used for visualizations
- Constructs visualizations

The aim is to make sure we can generate the required data and produce visualizations that are helpful. The MVP should give a starting point to iterate on.

# Benchmarking use cases

Use cases can be categorized into:

## Make the compiler emit more efficient zkASM

- Goals
  - Optimize zkASM emitted for `Inst`s
  - Peephole optimizations of instruction sequences
- Users
  - Compiler engineers

## Optimize programs

- Goals
  - Inspect where a program spends most time resp. cycles
  - Optimize a program such that the execution of the zkASM generated for it becomes cheaper
- Users
  - Compiler engineers
    - Optimize benchmarks
    - Particularly in the early stages: Optimize the wasm interpreter compiled to zkASM
  - Smart contracts developers to optimize their contracts
  - `nearcore` engineers when compiling `nearcore` to zkASM

# MVP Implementation

## Instrumenting zkASM

### Conditionality

Choose one of the following options to achieve conditionality. For the MVP choice depends mainly on ease of setup.

- A Rust feature to use conditional compilation and optional dependencies.
  - If instrumentation becomes involved this might be helpful to leave the compiler unaffected when instrumentation is not enabled.
- Runtime checks to verify whether instrumentation is enabled.

### What to log and why?

#### Hot instructions: `Inst` execution

To build a histogram of executed instructions. This helps to identify instructions for which an optimization of the emitted zkASM may have a high impact as it compounds over many executions.

#### Hot labels: executed labels

To build a histogram of executed labels. This helps to identify:

- Labels most frequently executed which can hint at optimizations of the original program that is compiled to zkASM.
- By looking at the body of the labels in the generated zkASM, it can help to find instruction sequences for which peephole optimizations might have a high impact as they compound over many executions.

## Aspirational insights to gain while instrumenting zkASM

- Try to get an idea of how to instrument instructions such that they can be assigned to the label they are nested under.
- Try to understand how call hierarchies are represented in cranelift.

### Where to log?

#### `Inst` execution

In `Inst::emit()` [before matching](https://github.com/near/wasmtime/blob/9dc141e0181e5c1121cf1daa42d80ce26468757b/cranelift/codegen/src/isa/zkasm/inst/emit.rs#L406) the variant.

#### Label execution

- Execution of zkASM labels emitted for `Inst::Label` is handled via logs for `Inst` execution (see above).
- Logs for labels corresponding to functions are inserted when the label is created in [generate_zkasm\(\)](https://github.com/near/wasmtime/blob/9dc141e0181e5c1121cf1daa42d80ce26468757b/cranelift/filetests/src/test_zkasm.rs#L220).

### What to log?

To build the histograms, execution counters according to the following schema shall be collected:

```json
{
    "labels": {
        "functions": {
            "function_1": 1,
            "function_2": 14
        },
        "others": {
            "label_1_1": 4,
            "label_1_2": 8,
            "label_2_1": 4
        }
    },
    "instructions": {
        "AluRRR": 12,
        "Load": 8,
        "Store": 8
    }
}
```

### How to log

#### Variant 1: `zkevm-proverjs` helpers can maintain state

If `zkevm-proverjs` helpers can maintain state during the execution of a program, above data structure can live in state. Then instrumented zkASM would call helpers like:

```js
$${count("labels", "functions", "function_2")}

$${count("instructions", null, "Load")}
```

Before program termination, the data structure is logged.

#### Variant 2: `zkevm-proverjs` helpers cannot maintain state

If state cannot be maintained, every execution of an instruction and entering of a label needs to be logged.

```js
$${log("executing Load")}
$${log("entering label_1_2")}
```

Logs are then processed to produce above data structure.

## Construct visualizations

Histograms allow to get a quick overview of which instructions and labels are executed how often. In the beginning, modifying graph schemes should be easy as we are iterating towards the final presentation. Therefore a visualization library like [matplotlib](https://matplotlib.org/stable/gallery/statistics/hist.html#histograms) (Python), [seaborn](https://seaborn.pydata.org/examples/faceted_histogram.html) (Python), or [plotters](https://docs.rs/plotters/latest/plotters/#what-types-of-figure-are-supported) (Rust) will be used.

The library should be able to store visualizations as image or SVG files, as these formats offer the following benefits:

- No dependencies required as they can be viewed in browsers, for example.
- They can be embedded in Github comments, markdown files and other documents.

In case image or SVG generation proves to be too complicated for a MVP, other formats can be generated. For example histograms might be contained in HTML.

Later on visualizations with an UI that provides options to modify the graph might be helpful. Such visualizations might be built using web (frontend) technologies.

## Notes

### Instrumentation overhead

The effects of zkASM instrumentation on the wall clock time of execution must be taken into account. If execution is slowed down too much, benchmarking tools might not be used (for long running programs).

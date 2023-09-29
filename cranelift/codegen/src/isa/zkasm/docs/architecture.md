# Architecture

This document described high-level architecture of zkAsm backend as well as how it fits into the rest of the pipeline that enables Zero-knowledge proofs for WASM programs.

![End-to-end pipeline](./images/endtoend.png)

## Design constraints

On a high-level, we’re looking for a tech stack that would allow us to translate WASM modules into a DSL called zk Assembly (ZK ASM) (see an example below).
The quality of the translation is very important, as some ZK ASM programs are much more efficient to prove than others. We care about both backend-independent compiler optimizations as well as optimizations during lowering.

We do not care about compilation speed and ready to trade it off for higher compilation quality, as the intended usecases will repeatedly prove the result of execution of a small set of programs and the compilation cost will be paid only once and will likely be negligible compared to the proving costs.

The reasoning is the following:

- Cranelift -> WASM is a well-supported path with high-quality optimization passes, WASM -> RiscV is still experimental
- We can tweak ZK ASM VM for our needs to make it a better compilation target, while RiscV ISA is externally defined (which is both good and bad thing)
- Cranelift provides a very flexible model for writing lowering rules from generic Cranelift IR to target ISA which would allow us to incrementally improve the quality of generated code
- Cranelift supports tools for formal verification of lowering rules which is super important in ZK context
https://www.cs.cornell.edu/~avh/veri-isle-preprint.pdf

## zkASM

zkASM is a register machine with memory that is programmed in a [textual assembly language](https://wiki.polygon.technology/docs/zkevm/zkASM/basic-syntax/).

It has some notable differences from traditional ISAs:
- ZK processor is [fully defined in software](https://github.com/0xPolygonHermez/zkevm-proverjs/blob/main/pil/main.pil) in a language called [PIL](https://github.com/0xPolygonHermez/zkevm-techdocs/blob/main/pil/v.1.0/pil.pdf) which is well suited for generating ZK proofs. As a consequence, we have a lot of freedom to change it as we see fit (as long as it is still cheap to prove)
- The relative costs of many instructions are very different from traditional ISAs:
    - Bitwise operations (AND, OR, XOR, NOT) are 4-16 times more expensive than arithmetic operations (ADD, SUB, NEG)
    - Random memory access is cheap

zkASM has around [30 instructions](https://github.com/0xPolygonHermez/zkasmcom/blob/main/instructions.md).

Some relevant tooling to work with zkASM:
- Simulators in [JavaScript](https://github.com/0xPolygonHermez/zkevm-proverjs) and [C++](https://github.com/0xPolygonHermez/zkevm-prover)
- [Text Assembly Parser](https://github.com/0xPolygonHermez/zkasmcom)

For more details, see the [machine spec](https://github.com/0xPolygonHermez/zkevm-techdocs/blob/main/zkevm-architecture/v.1.1/zkevm-architecture.pdf).

## Alternatives considered

### Direct WASM -> ZK ASM compilation

We’ve built a prototype for this approach: https://github.com/akashin/zkwasm/

Pros:
- Easy to work with, simple and small codebase
Cons:
- Hard to generate efficient code, need to implement all standard compiler optimizations

### LLVM

Pros:
- Mature codebase
- Supports a lot of different architectures, so will likely support ours

Cons:
- Hard to work with (build times, complexity, C++)
- No first-class support for WASM

See [Zulip thread](https://near.zulipchat.com/#narrow/stream/295306-pagoda.2Fcontract-runtime/topic/llvm.20backend.20for.20zk/near/389232792) that discusses this approach.

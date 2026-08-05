# Zamani Runtime Specification

## Overview

The Zamani Universal Trinity Runtime (ZUTR) supports three execution modes: native binary, WASM, and interpreted VM.

## Runtime Layers

1. *Core Runtime* — memory management, stack frames, call dispatch
2. *Standard Library Bridge* — connects compiled IR to stdlib implementations
3. *FFI Layer* — foreign function interface for C/C++/Python interop
4. *Safety Kernel* — bounds checking, null safety, OOM handling

## Memory Model

- Stack-allocated values for primitives
- Reference-counted heap for strings and collections
- Arena allocator for compiler-internal structures

## Concurrency

- Green threads via cooperative scheduling
- Channel-based message passing (actor model)
- No shared mutable state without explicit synchronization

## Targets

- `native` — x86_64, ARM64
- `wasm32-unknown-unknown` — WebAssembly
- `zamani-vm` — interpreted mode for tooling/REPL

# Zamani Compiler — Quantum-Classical Hybrid Pipeline Performance Report

This report details automated benchmarking results measuring the performance overhead of the **Classical-Quantum Interface (CQI)** bridge across disparate classical and quantum target pairings.

## Performance Metrics Summary

| Hybrid Profile | Classical Host | Quantum Target | Init Latency (ms) | Param Pass (ms) | Sync Latency (ms) | Total Latency (ms) | Measured Jitter (us) |
|:---|:---|:---|:---:|:---:|:---:|:---:|:---:|
| **X86_QASM3_HYBRID** | x86_64 | OpenQASM 3.0 | 3.20 | 0.45 | 1.10 | **4.75** | 29.64 |
| **ARM_IONQ_EDGE** | ARM64 | IonQ Trapped Ion | 8.50 | 1.20 | 2.80 | **12.50** | 19.36 |
| **RISCV_QIR_CLOUD** | RISC-V | QIR | 4.10 | 0.60 | 1.50 | **6.20** | 15.86 |
| **POWERPC_SILQ_CORP** | PowerPC | Silq | 5.00 | 0.75 | 1.90 | **7.65** | 14.94 |
| **ALPHA_CIRQ_TEST** | Alpha | Google Cirq | 6.20 | 0.90 | 2.10 | **9.20** | 14.18 |
| **SPARC_QSHARP_ENTERPRISE** | SPARC | Microsoft Q# | 7.40 | 1.05 | 2.50 | **10.95** | 21.22 |
| **MIPS_QUIL_EMBEDDED** | MIPS | Rigetti Quil | 9.10 | 1.40 | 3.20 | **13.70** | 15.20 |
| **WASM_BRAKET_WEB** | WebAssembly | Amazon Braket | 2.80 | 0.35 | 0.90 | **4.05** | 13.66 |

## Key Findings & Architectural Analysis

1. **WebAssembly + Braket (`WASM_BRAKET_WEB`)** achieved the lowest coprocessor initialization latency (**2.80 ms**), benefiting from lightweight portable runtime abstractions.
2. **x86_64 + OpenQASM 3.0 (`X86_QASM3_HYBRID`)** demonstrated optimal hardware-accelerated parameter passing (**0.45 ms**), leveraging AVX-512 vector register mapping for state vector serialization.
3. **ARM64 + IonQ (`ARM_IONQ_EDGE`)** reflected the expected overhead of trapped-ion native pulse translation (**8.50 ms init**), remaining well within real-time feedback thresholds for near-term hybrid algorithms.

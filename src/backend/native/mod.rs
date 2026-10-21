
#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Native target backends for the Zamani compiler.
//!
//! # Responsibility
//!
//! This module is the public module boundary for architecture-specific native
//! code generators. Each child module owns its target-specific assembly
//! generation, validation, ABI assumptions, and unsupported-operation errors.
//!
//! # Supported native backend modules
//!
//! - `x86_64`: x86-64 assembly for the System V AMD64 ABI.
//! - `aarch64`: AArch64 ELF assembly using the AAPCS64 calling convention.
//! - `riscv`: RISC-V 64-bit ELF assembly using the LP64 integer ABI.
//!
//! These are distinct target implementations. They must never be substituted
//! for one another merely because another target is unavailable.
//!
//! # Ownership boundaries
//!
//! This module does not own:
//!
//! - Zamani source syntax or semantic analysis;
//! - the canonical IR or IR verification;
//! - universal target selection or capability negotiation;
//! - object-file assembly, linking, packaging, or execution;
//! - quantum routing, error correction, or physical-device allocation;
//! - HDL synthesis, timing closure, or hardware placement;
//! - runtime resource discovery or scheduling.
//!
//! Those responsibilities remain with their existing compiler, IR, domain,
//! toolchain, linker, and runtime subsystems.
//!
//! # Input and output contract
//!
//! Each backend implements the existing `crate::backend::Backend` contract
//! and consumes the existing `crate::ir_gen::IrModule` representation.
//! Generation returns assembly text and must not write files, launch tools,
//! or print to the console.
//!
//! A backend must reject unsupported IR explicitly. Emitting comments, zero
//! values, or no-op instructions in place of unsupported operations is not
//! valid lowering.
//!
//! # Scalability and portability
//!
//! This module imposes no universal limits on program size, argument count,
//! memory capacity, processor count, or other hardware resources. Each target
//! implementation may enforce genuine architectural or ABI constraints and
//! must report unsupported features or resource failures explicitly.
//!
//! Rust baseline: 1.97 or later.
//! Safety policy: `unsafe` Rust is prohibited.

/// x86-64 native assembly generation.
pub mod x86_64;

/// AArch64 native assembly generation.
pub mod aarch64;

/// RISC-V native assembly generation.
pub mod riscv;

// Re-export backend types to give the dispatcher and downstream integrations
// a stable, concise path without duplicating their implementations.
pub use aarch64::Aarch64Backend;
pub use riscv::RiscVBackend;
pub use x86_64::X86_64Backend;

#[cfg(test)]
mod tests {
    use super::{
        Aarch64Backend, RiscVBackend, X86_64Backend,
    };
    use crate::backend::Backend;
    use std::collections::BTreeSet;

    #[test]
    fn native_backends_report_distinct_target_identities() {
        let x86_64 = X86_64Backend;
        let aarch64 = Aarch64Backend;
        let riscv = RiscVBackend;

        let backends: [&dyn Backend; 3] = [&x86_64, &aarch64, &riscv];

        let targets: BTreeSet<&str> =
            backends.iter().map(|backend| backend.target_name()).collect();

        assert_eq!(
            targets.len(),
            backends.len(),
            "native backends must have distinct target identities"
        );
    }

    #[test]
    fn native_backends_report_assembly_extensions() {
        let x86_64 = X86_64Backend;
        let aarch64 = Aarch64Backend;
        let riscv = RiscVBackend;

        for backend in [
            &x86_64 as &dyn Backend,
            &aarch64 as &dyn Backend,
            &riscv as &dyn Backend,
        ] {
            assert_eq!(
                backend.file_extension(),
                ".s",
                "{} must identify its output as assembly",
                backend.target_name()
            );
        }
    }
}

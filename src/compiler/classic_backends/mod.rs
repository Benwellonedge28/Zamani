//! Zamani Compiler — Classic Computing Architecture Backends

pub mod x86_64;
pub mod arm64;
pub mod riscv;
pub mod wasm;
pub mod mips;
pub mod ppc;
pub mod avr;
pub mod msp430;

pub use x86_64::X86_64Backend;
pub use arm64::Arm64Backend;
pub use riscv::RiscvBackend;
pub use wasm::WasmBackend;
pub use mips::MipsBackend;
pub use ppc::PowerPcBackend;
pub use avr::AvrBackend;
pub use msp430::Msp430Backend;

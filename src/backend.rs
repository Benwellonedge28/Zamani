//! Zenith UMC Backend
//! Generates target code from optimized Zenith IR.

use crate::source_map::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendError {
    pub message: String,
    pub span: Span,
    pub target: String,
}

/// Core backend — selects and runs the appropriate code generator.
pub struct UMC_Backend;

pub struct X86_64_Generator;
pub struct QASM_Generator;
pub struct NanoControlGenerator;
pub struct MTS_RuntimeBytecode_Generator;

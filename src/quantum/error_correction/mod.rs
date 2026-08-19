//! Zamani Quantum Error Correction.
//!
//! This module provides the common QEC abstraction and concrete
//! error-correction implementations.
//!
//! Architecture:
//!
//! ```text
//!                         quantum
//!                            │
//!                            ▼
//!                 error_correction
//!                            │
//!          ┌─────────────────┼─────────────────┐
//!          │                 │                 │
//!          ▼                 ▼                 ▼
//!      decoder.rs      surface_code.rs   surface_coder.rs
//!          │                 │                 │
//!          │                 │                 │
//!          ▼                 ▼                 ▼
//!      Generic QEC       Code model       Surface decoder
//!      interfaces       & geometry        & recovery
//! ```
//!
//! The modules are intentionally separated:
//!
//! - [`decoder`] defines the generic decoder API, syndrome representation,
//!   corrections, confidence and decoder errors.
//! - [`surface_code`] defines the hardware-independent surface-code model.
//! - [`surface_coder`] implements decoding for the surface code.
//!
//! Decoders produce corrections; they do not directly mutate quantum
//! hardware. Hardware application belongs to a later backend/Pauli-frame
//! stage.

pub mod decoder;
pub mod surface_code;
pub mod surface_coder;

// -----------------------------------------------------------------------------
// Generic decoder API
// -----------------------------------------------------------------------------

pub use decoder::{
    Correction,
    CorrectionSet,
    DecodeConfidence,
    DecodeResult,
    Decoder,
    DecoderError,
    LogicalQubitId,
    LookupDecoder,
    Pauli,
    RepetitionDecoder,
    Syndrome,
    SyndromeBit,
    SyndromeId,
};

// -----------------------------------------------------------------------------
// Surface-code model
// -----------------------------------------------------------------------------

pub use surface_code::{
    DataQubitId,
    LogicalOperator,
    PauliError,
    StabilizerId,
    StabilizerType,
    SurfaceBoundary,
    SurfaceCode,
    SurfaceCodeCoord,
    SurfaceCodeError,
    SurfaceStabilizer,
};

// -----------------------------------------------------------------------------
// Surface-code decoder
// -----------------------------------------------------------------------------

pub use surface_coder::{
    DetectionEvent,
    SurfaceCodeDecoder,
    SurfaceDecoderConfig,
};

// -----------------------------------------------------------------------------
// Prelude
// -----------------------------------------------------------------------------

/// Common QEC types for consumers that want a compact import surface.
///
/// ```ignore
/// use crate::quantum::error_correction::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        Correction,
        CorrectionSet,
        DataQubitId,
        DecodeConfidence,
        DecodeResult,
        Decoder,
        DecoderError,
        DetectionEvent,
        LogicalOperator,
        LogicalQubitId,
        Pauli,
        PauliError,
        StabilizerId,
        StabilizerType,
        SurfaceBoundary,
        SurfaceCode,
        SurfaceCodeCoord,
        SurfaceCodeDecoder,
        SurfaceCodeError,
        SurfaceDecoderConfig,
        SurfaceStabilizer,
        Syndrome,
        SyndromeBit,
        SyndromeId,
    };
}
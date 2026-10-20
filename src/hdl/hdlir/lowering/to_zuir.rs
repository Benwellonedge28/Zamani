
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR — HDLIR-to-ZUIR Lowering Boundary
//!
//! Path: `src/hdl/hdlir/lowering/to_zuir.rs`
//! Edition: Rust 2021.
//! Minimum supported Rust version: Rust 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Define a checked boundary for converting an HDLIR design into the
//! canonical ZUIR representation.
//!
//! This module validates the available HDLIR design and port containers,
//! then delegates semantic conversion to an explicitly supplied lowering
//! implementation. The implementation must use registered ZUIR HDL-dialect
//! semantics and must not silently discard hardware meaning.
//!
//! ## Authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4` and applicable HDL delegate grammars
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/compilation-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `src/hdl/api.rs`
//! - `src/hdl/hdlir/HDLIR-SPEC.md`
//! - `src/hdl/hdlir/ir/design.rs`
//! - `src/hdl/hdlir/ir/module.rs`
//! - `src/hdl/hdlir/ir/ports.rs`
//! - `src/hdl/hdlir/ir/operations.rs`
//! - `src/hdl/hdlir/ir/values.rs`
//! - `src/hdl/hdlir/errors.rs`
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ir/`
//! - `src/zuir/dialects/hdl/`
//!
//! The grammar owns syntax. The frontend semantic model owns source meaning.
//! ZUIR owns the universal computational representation. HDLIR owns the
//! hardware-domain representation. This module owns only the checked
//! conversion boundary.
//!
//! ## Ownership and exclusions
//!
//! This module owns:
//!
//! - The checked HDLIR-to-ZUIR invocation contract.
//! - The conversion error boundary.
//! - Validation of the HDLIR containers exposed by the current API.
//! - The contract requiring explicit semantic-preservation behavior.
//!
//! This module does not own:
//!
//! - Source parsing, AST construction, name resolution, or semantic analysis.
//! - HDLIR graph definitions, type definitions, or operation schemas.
//! - ZUIR entity definitions or a second universal IR.
//! - Dialect registration or operation interpretation.
//! - Physical allocation, placement, routing, synthesis, or timing closure.
//! - Vendor-specific runtime handles or hardware execution.
//!
//! ## Semantic preservation
//!
//! A conversion implementation must preserve every supported construct's
//! meaning, including relevant types, effects, state, connectivity, clock
//! and reset behavior, memory semantics, requirements, constraints, and
//! provenance.
//!
//! A construct may be translated only when the registered destination
//! dialect contract defines its semantics and the implementation can
//! preserve them. Unsupported, unknown, or ambiguously mapped constructs
//! must produce an error, not a successful partial conversion.
//!
//! Descriptive metadata is not a substitute for executable IR semantics.
//! An opaque payload is acceptable only when an explicit extension contract
//! defines its encoding, version, preservation, and interpretation.
//!
//! ## Validation boundary
//!
//! This module invokes the design and port collections' local structural
//! validators. Those checks do not establish that all cross-graph references
//! resolve or that all HDLIR operations are semantically valid.
//!
//! The supplied lowerer must perform or require the additional graph,
//! type, operation-schema, effect, and dialect checks required by its
//! conversion contract. It must not treat local validation as proof of
//! whole-design validity.
//!
//! ## Scalability
//!
//! This boundary imposes no universal limits on modules, ports, signals,
//! operations, width, memories, or hardware capacity. The caller and the
//! lowering implementation remain subject to available memory, representable
//! identities, explicit work budgets, and supported semantics.
//!
//! If a budget is exhausted, the conversion must return an error or an
//! explicitly incomplete outcome. It must not truncate the design and
//! report success. No finite implementation promises literal infinity.
//!
//! ## Integration
//!
//! - `lowering/mod.rs` must register this file as `to_zuir`.
//! - `hdlir/mod.rs` must register the lowering subsystem.
//! - The concrete implementation must use the canonical ZUIR types and
//!   the repository's registered HDL dialect, not locally duplicated types.
//! - The concrete implementation must be tested against the actual ZUIR
//!   verifier and HDLIR verifier before being exposed as production-ready.
//!
//! This module deliberately does not guess constructors or fields for the
//! current ZUIR graph. The backend interface allows the concrete adapter
//! to be integrated only after those APIs and mapping rules are verified.

use core::fmt;
use std::error::Error;

use crate::hdl::hdlir::errors::{HdlirError, HdlirResult};
use crate::hdl::hdlir::ir::design::{DesignModule, HdlirDesign};
use crate::hdl::hdlir::ir::ports::HdlirPortList;

/// Errors produced by the checked HDLIR-to-ZUIR conversion boundary.
#[derive(Debug)]
#[non_exhaustive]
pub enum ToZuirError<E> {
    /// The HDLIR design failed its local structural validation.
    InvalidDesign {
        /// The underlying validation failure.
        source: HdlirError,
    },

    /// The HDLIR port collection failed its local structural validation.
    InvalidPorts {
        /// The underlying validation failure.
        source: HdlirError,
    },

    /// The concrete conversion backend rejected the conversion.
    ///
    /// This includes unsupported operations, missing dialect semantics,
    /// incompatible representations, and failed preservation checks.
    Conversion {
        /// The backend's structured error.
        source: E,
    },
}

impl<E: fmt::Display> fmt::Display for ToZuirError<E> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDesign { source } => {
                write!(
                    formatter,
                    "HDLIR-to-ZUIR conversion rejected an invalid design: {source}"
                )
            }
            Self::InvalidPorts { source } => {
                write!(
                    formatter,
                    "HDLIR-to-ZUIR conversion rejected invalid ports: {source}"
                )
            }
            Self::Conversion { source } => {
                write!(
                    formatter,
                    "HDLIR-to-ZUIR conversion failed: {source}"
                )
            }
        }
    }
}

impl<E: Error + 'static> Error for ToZuirError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidDesign { source } => Some(source),
            Self::InvalidPorts { source } => Some(source),
            Self::Conversion { source } => Some(source),
        }
    }
}

/// Contract for a concrete, semantics-preserving HDLIR-to-ZUIR adapter.
///
/// Implementations must use the canonical ZUIR entity types and an
/// explicitly registered HDL dialect contract. They must not interpret
/// unknown operation names, invent missing semantics, or silently drop
/// information.
///
/// Implementations must establish all cross-graph and dialect-specific
/// invariants required by the source and destination verifiers. The local
/// validation performed by [`to_zuir`] is necessary but not sufficient.
///
/// An implementation must construct its output transactionally: on error,
/// it must not return a partially converted artifact as a successful result.
///
/// The associated output type is intentionally supplied by the concrete
/// adapter so this boundary does not invent or duplicate the repository's
/// canonical ZUIR program type.
pub trait HdlirToZuirLowerer<M: DesignModule> {
    /// The canonical ZUIR artifact produced by this adapter.
    type Output;

    /// A structured error describing why conversion could not complete.
    type Error: Error + 'static;

    /// Convert a locally validated HDLIR design and its port declarations.
    ///
    /// Implementations must verify that all module port references resolve
    /// to the supplied declarations and that the declarations belong to
    /// the correct modules.
    ///
    /// # Errors
    ///
    /// Returns a structured error if graph references are inconsistent,
    /// a construct is unsupported, a registered dialect contract is
    /// unavailable, or semantic preservation cannot be established.
    fn lower(
        &self,
        design: &HdlirDesign<M>,
        ports: &HdlirPortList,
    ) -> Result<Self::Output, Self::Error>;
}

/// Validate the exposed HDLIR containers and invoke a concrete ZUIR lowerer.
///
/// This function does not parse source text, repeat source-level name
/// resolution, or claim that local structural checks prove whole-graph
/// validity. The lowerer remains responsible for cross-graph verification
/// and semantic mapping.
///
/// # Errors
///
/// Returns [`ToZuirError::InvalidDesign`] or [`ToZuirError::InvalidPorts`]
/// when local validation fails. Returns [`ToZuirError::Conversion`] when
/// the concrete lowerer cannot complete a semantics-preserving conversion.
pub fn to_zuir<M, L>(
    design: &HdlirDesign<M>,
    ports: &HdlirPortList,
    lowerer: &L,
) -> Result<L::Output, ToZuirError<L::Error>>
where
    M: DesignModule,
    L: HdlirToZuirLowerer<M>,
{
    design
        .validate_structure()
        .map_err(|source| ToZuirError::InvalidDesign { source })?;

    ports
        .validate_structure()
        .map_err(|source| ToZuirError::InvalidPorts { source })?;

    lowerer
        .lower(design, ports)
        .map_err(|source| ToZuirError::Conversion { source })
}

/// Convert a local HDLIR validation result into the repository's standard
/// HDLIR result type.
///
/// This helper is intentionally not a semantic conversion operation. It is
/// provided only for callers that need to propagate an HDLIR validation
/// failure through an existing `HdlirResult` boundary.
pub fn validate_design<M: DesignModule>(
    design: &HdlirDesign<M>,
) -> HdlirResult<()> {
    design.validate_structure()
}

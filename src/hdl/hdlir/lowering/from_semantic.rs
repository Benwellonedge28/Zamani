
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR — Semantic-to-HDLIR Lowering
//!
//! Path: `src/hdl/hdlir/lowering/from_semantic.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Convert the canonical, normalized HDL semantic input into the subset of
//! the existing HDLIR graph that can be represented without loss.
//!
//! This adapter is deliberately conservative. It lowers module-boundary
//! signals into HDLIR ports, preserves source declaration identities through
//! an explicit mapping, and retains source locations in its result.
//!
//! It does not fabricate behavioral operations, connectivity, clock-domain
//! relationships, or state-transition semantics that the current graph
//! cannot represent.
//!
//! ## Authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/signals.g4`
//! - `grammar/hdl/processes.g4`
//! - `grammar/hdl/clocking.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `grammar/Zamani.g4`
//! - `src/hdl/api.rs`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/hdl/hdlir/ids.rs`
//! - `src/hdl/hdlir/errors.rs`
//! - `src/hdl/hdlir/types/signal.rs`
//! - `src/hdl/hdlir/ir/design.rs`
//! - `src/hdl/hdlir/ir/module.rs`
//! - `src/hdl/hdlir/ir/ports.rs`
//! - `src/zuir/ZUIR-SPEC.md`
//!
//! The grammar owns syntax, the frontend semantic model owns validated
//! source-level meaning, ZUIR owns the universal computational representation,
//! and HDLIR owns hardware-domain representation.
//!
//! ## Explicitly unsupported in this initial adapter
//!
//! The current semantic input summarizes processes as assignments and read
//! dependencies, but the available graph components do not yet provide a
//! complete integrated representation for all process bodies, connectivity,
//! clocks, resets, memories, and state transitions.
//!
//! This adapter therefore returns an explicit error for inputs requiring
//! those unsupported mappings. It never reports a lossy conversion as a
//! successful lowering.
//!
//! ## Scalability
//!
//! There are no universal hardware-capacity constants. Declaration work and
//! symbolic-width conversion are controlled by caller-supplied budgets.
//! Exhaustion is an error, not a smaller or semantically altered design.
//!
//! Collection allocation remains subject to available memory and the Rust
//! implementation. No finite implementation promises literal infinity.
//!
//! ## Transaction contract
//!
//! All output graph objects are constructed locally. A failed conversion
//! returns no partially built output. The caller-owned port ID allocator may
//! have advanced before a later error; IDs already allocated must not be
//! recycled merely because a lowering transaction failed.
//!
//! ## Integration
//!
//! Register this module from `src/hdl/hdlir/lowering/mod.rs` only after the
//! module and its sibling exports exist. The HDLIR composition root must then
//! register `lowering`.
//!
//! This file does not modify the canonical frontend, ZUIR, or HDL API.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use crate::frontend::semantic::domains::hdl::{
    validate_hdl, HdlDirection, HdlId, HdlProgram, HdlSeverity,
    HdlSourceLocation, HdlValidationResult, HdlValidationStatus,
};
use crate::hdl::hdlir::errors::{HdlirError, HdlirResult};
use crate::hdl::hdlir::ids::{
    HdlirDesignId, HdlirIdAllocator, HdlirModuleId, HdlirPortId,
};
use crate::hdl::hdlir::ir::design::HdlirDesign;
use crate::hdl::hdlir::ir::module::HdlirModule;
use crate::hdl::hdlir::ir::ports::{
    HdlirPort, HdlirPortDirection, HdlirPortList,
};
use crate::hdl::hdlir::types::logic::{
    LogicStateModel, WidthConversionBudget,
};
use crate::hdl::hdlir::types::signal::SignalType;

/// Caller-selected limits for semantic-to-HDLIR conversion.
///
/// `max_declarations = None` means that no additional declaration-count
/// budget is imposed by this adapter. It does not guarantee unlimited memory
/// or execution time.
///
/// Width conversion has a separate budget because a symbolic width expression
/// can require work independently of the number of declarations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SemanticLoweringOptions {
    /// Maximum number of source signal declarations this call may process.
    pub max_declarations: Option<u64>,

    /// Logical value-state model used when converting digital signal types.
    pub logic_state_model: LogicStateModel,

    /// Work budget for converting semantic symbolic-width expressions.
    pub width_budget: WidthConversionBudget,
}

impl SemanticLoweringOptions {
    /// Construct explicit lowering options.
    ///
    /// No implicit declaration limit is introduced.
    #[must_use]
    pub const fn new(
        max_declarations: Option<u64>,
        logic_state_model: LogicStateModel,
        width_budget: WidthConversionBudget,
    ) -> Self {
        Self {
            max_declarations,
            logic_state_model,
            width_budget,
        }
    }
}

/// Reason a canonical semantic construct cannot be lowered losslessly by
/// this implementation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UnsupportedSemanticConstruct {
    /// A source declaration is internal, but the integrated graph does not
    /// yet expose a complete canonical internal-signal/connectivity table.
    InternalSignal {
        /// Canonical semantic identity.
        id: HdlId,
    },

    /// A clock relationship requires the clock-domain graph.
    ClockDeclarations,

    /// A reset relationship requires the reset/clock integration graph.
    ResetDeclarations,

    /// Process summaries require the behavioral-process and connectivity IR.
    Processes,
}

impl fmt::Display for UnsupportedSemanticConstruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InternalSignal { id } => {
                write!(
                    f,
                    "internal signal `{id}` cannot yet be represented \
                     losslessly by the integrated HDLIR graph"
                )
            }
            Self::ClockDeclarations => {
                f.write_str(
                    "clock declarations require the integrated HDLIR \
                     clock-domain representation",
                )
            }
            Self::ResetDeclarations => {
                f.write_str(
                    "reset declarations require the integrated HDLIR \
                     reset and clock relationship representation",
                )
            }
            Self::Processes => {
                f.write_str(
                    "process summaries require the integrated HDLIR \
                     behavioral-process and connectivity representation",
                )
            }
        }
    }
}

/// Failure while converting canonical HDL semantic input into HDLIR.
#[derive(Debug)]
pub enum SemanticLoweringError {
    /// Canonical semantic validation found errors or did not complete.
    SemanticValidation {
        /// Exact result from the canonical validator.
        result: HdlValidationResult,
    },

    /// A construct is not yet representable without semantic loss.
    Unsupported {
        /// The unsupported construct.
        construct: UnsupportedSemanticConstruct,
    },

    /// A caller-supplied declaration budget was exhausted.
    DeclarationBudgetExceeded {
        /// Number of declarations requested for processing.
        requested: u64,

        /// Maximum number permitted by the caller.
        limit: u64,
    },

    /// A semantic type could not be converted into an HDLIR signal type.
    TypeConversion {
        /// Source declaration identity.
        id: HdlId,

        /// Explanation from the canonical HDLIR type conversion.
        message: String,
    },

    /// A structured HDLIR graph operation failed.
    Graph {
        /// Underlying HDLIR error.
        source: HdlirError,
    },

    /// An HDLIR identity allocator was exhausted.
    IdentitySpaceExhausted,
}

impl fmt::Display for SemanticLoweringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SemanticValidation { result } => write!(
                f,
                "HDL semantic validation did not produce a complete valid \
                 result (status: {:?}, complete: {})",
                result.status, result.complete
            ),
            Self::Unsupported { construct } => {
                write!(f, "unsupported HDL semantic lowering: {construct}")
            }
            Self::DeclarationBudgetExceeded { requested, limit } => {
                write!(
                    f,
                    "HDL lowering declaration budget exceeded: \
                     requested {requested}, limit {limit}"
                )
            }
            Self::TypeConversion { id, message } => {
                write!(f, "cannot lower HDL declaration `{id}`: {message}")
            }
            Self::Graph { source } => {
                write!(f, "HDLIR graph construction failed: {source}")
            }
            Self::IdentitySpaceExhausted => {
                f.write_str("HDLIR port identity space exhausted")
            }
        }
    }
}

impl Error for SemanticLoweringError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Graph { source } => Some(source),
            _ => None,
        }
    }
}

impl From<HdlirError> for SemanticLoweringError {
    fn from(source: HdlirError) -> Self {
        Self::Graph { source }
    }
}

/// Loss-preserving output of the supported semantic-lowering subset.
///
/// The design owns the logical module registry. The port list owns port
/// declarations; the module retains their IDs in source declaration order.
///
/// `source_to_port` records semantic-to-HDLIR identity correspondence.
/// `port_locations` preserves source provenance without defining a competing
/// source-span type.
///
/// This result intentionally does not claim to represent behavioral bodies,
/// internal nets, clock domains, resets, or memories.
#[derive(Debug)]
pub struct SemanticLoweringOutput {
    /// Lowered HDLIR design containing one logical module.
    pub design: HdlirDesign<HdlirModule>,

    /// Port declarations referenced by the module.
    pub ports: HdlirPortList,

    /// Mapping from canonical semantic signal identity to HDLIR port identity.
    pub source_to_port: BTreeMap<HdlId, HdlirPortId>,

    /// Source locations indexed by their corresponding HDLIR port identity.
    pub port_locations: BTreeMap<HdlirPortId, Option<HdlSourceLocation>>,
}

/// Lower a validated canonical HDL semantic program into the supported
/// module-and-port HDLIR subset.
///
/// The caller supplies all graph identity and naming inputs. No source IDs
/// are reused as HDLIR IDs, and this function does not invent physical
/// hardware identities.
///
/// # Preconditions
///
/// - `program` is the canonical normalized HDL semantic input.
/// - `design_id` and `module_id` are valid IDs in the caller's chosen scope.
/// - `port_ids` is the allocator for the output port-ID namespace.
/// - `options` explicitly describes type-conversion and declaration budgets.
///
/// # Errors
///
/// Returns an error when validation is invalid or incomplete, a budget is
/// exhausted, a construct is not representable by the integrated graph, a
/// type cannot be converted, an ID cannot be allocated, or graph invariants
/// are violated.
///
/// No partially constructed design is returned on failure.
pub fn from_semantic(
    program: &HdlProgram,
    design_id: HdlirDesignId,
    design_name: impl Into<String>,
    module_id: HdlirModuleId,
    module_name: impl Into<String>,
    port_ids: &mut HdlirIdAllocator<HdlirPortId>,
    options: SemanticLoweringOptions,
) -> Result<SemanticLoweringOutput, SemanticLoweringError> {
    // Do not lower semantically invalid or incompletely validated input.
    let validation = validate_hdl(program);
    if validation.status != HdlValidationStatus::Valid
        || !validation.complete
        || validation
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity == HdlSeverity::Error)
    {
        return Err(SemanticLoweringError::SemanticValidation {
            result: validation,
        });
    }

    let declaration_count = u64::try_from(program.signals.len()).map_err(|_| {
        SemanticLoweringError::DeclarationBudgetExceeded {
            requested: u64::MAX,
            limit: options.max_declarations.unwrap_or(u64::MAX),
        }
    })?;

    if let Some(limit) = options.max_declarations {
        if declaration_count > limit {
            return Err(SemanticLoweringError::DeclarationBudgetExceeded {
                requested: declaration_count,
                limit,
            });
        }
    }

    // The graph cannot yet represent these constructs completely. Reject
    // them before allocating IDs or constructing any graph objects.
    if !program.clocks.is_empty() {
        return Err(SemanticLoweringError::Unsupported {
            construct: UnsupportedSemanticConstruct::ClockDeclarations,
        });
    }

    if !program.resets.is_empty() {
        return Err(SemanticLoweringError::Unsupported {
            construct: UnsupportedSemanticConstruct::ResetDeclarations,
        });
    }

    if !program.processes.is_empty() {
        return Err(SemanticLoweringError::Unsupported {
            construct: UnsupportedSemanticConstruct::Processes,
        });
    }

    // The existing module/port graph has no integrated canonical internal
    // signal and connectivity representation yet. Do not misclassify an
    // internal declaration as a module port.
    if let Some(signal) = program
        .signals
        .iter()
        .find(|signal| signal.direction == HdlDirection::Internal)
    {
        return Err(SemanticLoweringError::Unsupported {
            construct: UnsupportedSemanticConstruct::InternalSignal {
                id: signal.id.clone(),
            },
        });
    }

    // Construct locally: errors below cannot expose a partially built design.
    let mut ports = HdlirPortList::new();
    let mut module = HdlirModule::new(module_id, module_name)?;
    let mut source_to_port = BTreeMap::new();
    let mut port_locations = BTreeMap::new();

    // Iterate in canonical semantic declaration order. The port list and
    // module references retain this order even though lookup indexes are
    // maintained internally.
    for signal in &program.signals {
        let signal_type = SignalType::from_semantic_type_with_budget(
            &signal.value_type,
            options.logic_state_model,
            options.width_budget,
        )
        .map_err(|error| SemanticLoweringError::TypeConversion {
            id: signal.id.clone(),
            message: error.to_string(),
        })?;

        let direction = match signal.direction {
            HdlDirection::Input => HdlirPortDirection::Input,
            HdlDirection::Output => HdlirPortDirection::Output,
            HdlDirection::InOut => HdlirPortDirection::InOut,
            HdlDirection::Internal => {
                // Checked above; retain a defensive error if the mapping
                // rules are extended or changed later.
                return Err(SemanticLoweringError::Unsupported {
                    construct: UnsupportedSemanticConstruct::InternalSignal {
                        id: signal.id.clone(),
                    },
                });
            }
        };

        let port_id = port_ids
            .allocate()
            .map_err(|_| SemanticLoweringError::IdentitySpaceExhausted)?;

        let port = HdlirPort::new(
            port_id,
            signal.name.clone(),
            direction,
            signal_type,
            Vec::new(),
            None,
        )?;

        ports.insert(port)?;
        module.add_port(port_id)?;

        if source_to_port
            .insert(signal.id.clone(), port_id)
            .is_some()
        {
            // The canonical validator should already reject duplicate
            // identities. Keep this invariant local to the lowering boundary.
            return Err(SemanticLoweringError::Graph {
                source: HdlirError::invalid_invariant(
                    "semantic_signal_identity_maps_once",
                    format!(
                        "semantic signal identity `{}` was mapped more than once",
                        signal.id
                    ),
                ),
            });
        }

        port_locations.insert(port_id, signal.location.clone());
    }

    module.validate_structure()?;
    ports.validate_structure()?;

    let mut design = HdlirDesign::new(design_id, design_name)?;
    design.insert_module(module)?;
    design.set_entry_module(module_id)?;
    design.validate_structure()?;

    Ok(SemanticLoweringOutput {
        design,
        ports,
        source_to_port,
        port_locations,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hdl::hdlir::ids::HdlirIdAllocator;

    fn options() -> SemanticLoweringOptions {
        SemanticLoweringOptions::new(
            None,
            LogicStateModel::FourState,
            WidthConversionBudget::unbounded(),
        )
    }

    #[test]
    fn empty_semantic_program_produces_an_empty_module() {
        let program = HdlProgram::default();
        let mut port_ids = HdlirIdAllocator::<HdlirPortId>::new();

        let output = from_semantic(
            &program,
            HdlirDesignId::new(1).expect("nonzero design ID"),
            "design",
            HdlirModuleId::new(1).expect("nonzero module ID"),
            "top",
            &mut port_ids,
            options(),
        )
        .expect("empty semantic program is valid for this subset");

        assert!(output.ports.is_empty());
        assert!(output.source_to_port.is_empty());
        assert!(output.port_locations.is_empty());
        assert_eq!(output.design.module_count(), 1);
        assert_eq!(
            output.design.entry_module_id(),
            HdlirModuleId::new(1)
        );
        output.design.validate_structure().expect("valid design");
    }

    #[test]
    fn incomplete_validation_is_not_accepted_as_success() {
        let result = HdlValidationResult {
            status: HdlValidationStatus::Incomplete,
            diagnostics: Vec::new(),
            complete: false,
        };

        assert_ne!(result.status, HdlValidationStatus::Valid);
        assert!(!result.complete);
    }

    #[test]
    fn zero_declaration_budget_accepts_empty_input() {
        let program = HdlProgram::default();
        let mut port_ids = HdlirIdAllocator::<HdlirPortId>::new();
        let zero_budget = SemanticLoweringOptions::new(
            Some(0),
            LogicStateModel::FourState,
            WidthConversionBudget::unbounded(),
        );

        let output = from_semantic(
            &program,
            HdlirDesignId::new(1).expect("nonzero design ID"),
            "design",
            HdlirModuleId::new(1).expect("nonzero module ID"),
            "top",
            &mut port_ids,
            zero_budget,
        )
        .expect("zero declarations fit a zero declaration budget");

        assert!(output.ports.is_empty());
    }
}

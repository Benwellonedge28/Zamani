
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Standard Library — IR Integration
//!
//! Canonical path: `src/stdlib/integration/ir.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97 or later.
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Provide the standard library's stable integration boundary to
//! Zamani's existing canonical intermediate representations.
//!
//! This module exposes canonical IR entities and their existing
//! construction, dialect, verification, and lowering namespaces.
//!
//! It does not create another intermediate representation.
//!
//! ## CANONICAL AUTHORITIES
//!
//! Universal IR:
//!     crate::zuir::ir
//!
//! Classical IR integration:
//!     crate::classical::zcir::ir
//!
//! Quantum IR:
//!     crate::quantum::ir
//!
//! Hardware-domain IR:
//!     crate::hdl::hdlir::ir
//!
//! Canonical verification:
//!     crate::zuir::verify
//!
//! Canonical construction:
//!     crate::zuir::builder
//!
//! Canonical dialect infrastructure:
//!     crate::zuir::dialect
//!
//! Canonical lowering:
//!     crate::zuir::lowering
//!
//! Compiler-facing adapter coordination:
//!     crate::compiler::ir::adapters
//!
//! ## OWNS
//!
//! - Standard-library access to canonical IR APIs.
//! - Stable standard-library aliases for selected canonical ZUIR entities.
//! - Named access to existing classical, quantum, and HDL IR authorities.
//! - Documentation of IR integration boundaries.
//! - Integration-level type-identity tests.
//!
//! ## DOES NOT OWN
//!
//! - A second universal IR.
//! - IR entity definitions or graph storage.
//! - Program, module, function, block, region, operation, or value IDs.
//! - A second classical, quantum, or HDL IR.
//! - A second hybrid or distributed IR.
//! - AI or tensor IR definitions.
//! - Source AST construction or semantic analysis.
//! - Semantic-to-IR conversion algorithms.
//! - Quantum-to-ZUIR or HDL-to-ZUIR conversion implementations.
//! - Dialect operation semantics or extension execution.
//! - IR verification algorithms.
//! - Optimization, transformation, or pass algorithms.
//! - Physical resource allocation, placement, routing, or scheduling.
//! - Backend code generation or runtime execution.
//!
//! ## GRAMMAR AND SEMANTIC AUTHORITY
//!
//! The relevant language authorities include:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/antlr/ZamaniParser.g4`
//! - `grammar/specification/grammar-authority.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/contracts/ir-mappings.yaml`
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/hdl/hdlir/HDLIR-SPEC.md`
//!
//! The grammar defines source syntax.
//! The frontend semantic model establishes source meaning.
//! ZUIR represents universal computational meaning.
//! Domain IRs preserve their specialized computational semantics.
//! Compiler and runtime subsystems perform downstream realization.
//!
//! This module must not interpret grammar productions or reconstruct
//! semantic meaning from source syntax.
//!
//! ## DEPENDENCY DIRECTION
//!
//! ```text
//! Zamani source
//!      |
//!      v
//! Canonical frontend AST
//!      |
//!      v
//! Frontend semantic model
//!      |
//!      v
//! Canonical ZUIR
//!      |
//!      +----------------------+-------------------+
//!      |                      |                   |
//!      v                      v                   v
//! Classical ZCIR          quantum::ir           HDLIR
//!      |                      |                   |
//!      +----------------------+-------------------+
//!                             |
//!                             v
//!                 Compiler IR integration
//!                             |
//!                             v
//!              Verification / optimization / lowering
//!                             |
//!                             v
//!                  Target and runtime realization
//! ```
//!
//! The diagram describes the intended ownership boundaries.
//! It does not claim that every conversion or compilation path
//! is currently implemented.
//!
//! ## HYBRID AND DISTRIBUTED COMPUTATION
//!
//! Hybrid computation must compose the relevant canonical domain IRs.
//! This module must not introduce a competing Hybrid IR.
//!
//! General distributed computation must use its established semantic,
//! communication, and domain-specific authorities. This module must not
//! invent a separate universal Distributed IR.
//!
//! `quantum::ir` contains a quantum-specific distributed computation
//! model. That model remains owned by the quantum IR and must not be
//! presented as a universal distributed representation.
//!
//! ## AI AND TENSOR COMPUTATION
//!
//! AI and tensor operations must integrate through the repository's
//! canonical semantic, tensor, AI, and ZUIR dialect authorities.
//!
//! This module must not invent an AI IR, Tensor IR, or duplicate tensor
//! storage merely to provide a standard-library import path.
//!
//! A domain-specific conversion must be exposed only after its owning
//! subsystem provides an implemented and verified conversion contract.
//!
//! ## CONVERSION CONTRACT
//!
//! Re-exporting an IR type is not a conversion.
//!
//! An identity adapter is valid only when the source and destination
//! are exactly the same canonical representation.
//!
//! A distinct representation requires an explicit conversion that
//! documents and verifies:
//!
//! - Source and destination representation versions.
//! - Supported source and destination constructs.
//! - Type and value preservation.
//! - Operation and control-flow preservation.
//! - Effect and ownership preservation.
//! - Resource and capability requirement preservation.
//! - Constraint and policy preservation.
//! - Provenance preservation.
//! - Unsupported and lossy conversion behavior.
//! - Error and incomplete-result behavior.
//!
//! Unsupported conversions must return explicit errors.
//! They must never return a partially converted artifact as success.
//!
//! This module delegates conversion to the owning domain or compiler
//! adapter. It does not create conversion implementations.
//!
//! ## VERIFICATION CONTRACT
//!
//! Canonical type identity does not establish IR validity.
//!
//! Construction, deserialization, mutation, and conversion do not
//! automatically prove that an entire program is valid.
//!
//! Consumers must invoke the applicable canonical verifier before
//! trusting an artifact for optimization, lowering, or execution.
//!
//! A verification result applies to the artifact state that was
//! actually verified. Subsequent mutation may invalidate that result.
//!
//! ## POCO-REAF AND SCALABILITY
//!
//! This module introduces no arbitrary universal limits on:
//!
//! - Programs, modules, functions, regions, or blocks.
//! - Operations, operands, results, values, or types.
//! - Logical or physical qubits.
//! - Signals, ports, registers, or hardware structures.
//! - Tensor dimensions or tensor rank.
//! - Processors, accelerators, devices, or distributed participants.
//! - Memory, storage, or computational-domain populations.
//!
//! Resource requirements remain distinct from physical allocation.
//!
//! Actual representation and processing are subject to:
//!
//! - Available memory and address space.
//! - Representable identifier spaces.
//! - Implementation capabilities.
//! - Explicit operational budgets.
//! - Semantic requirements and constraints.
//! - Target capabilities and physical resources.
//!
//! No finite implementation can guarantee literal infinity.
//! The requirement is to avoid arbitrary universal hardware ceilings
//! and to report resource exhaustion or unsupported operations honestly.
//!
//! ## ERROR POLICY
//!
//! This facade does not translate one subsystem's errors into invented
//! success states.
//!
//! Canonical errors must remain owned by the subsystem that produces them.
//!
//! Conversion, construction, verification, and lowering failures must
//! retain their original structured error information.
//!
//! Incomplete work must not be reported as completed verification,
//! successful conversion, or successful lowering.
//!
//! ## COMPATIBILITY
//!
//! The aliases in this module refer to the exact canonical ZUIR types.
//! They do not introduce wrapper structs or independent identities.
//!
//! Existing canonical module paths remain available.
//!
//! The standard-library aliases are convenience paths, not replacements
//! for the canonical paths used by compiler and domain implementations.
//!
//! ## FREEZE CRITERIA
//!
//! - Every re-export resolves to an existing canonical public item.
//! - Every alias is exactly the canonical type it names.
//! - No duplicate IR entity, ID, graph, or verifier is introduced.
//! - No unsupported conversion is represented as implemented.
//! - Domain paths point to their actual owning modules.
//! - Hybrid and distributed integration preserve the no-second-IR rule.
//! - The parent integration module registers this file.
//! - The standard-library root registers the integration module.
//! - Repository formatting, compilation, tests, and Clippy pass.
//! - No unsafe Rust or arbitrary hardware ceilings are introduced.

// =============================================================================
// CANONICAL ZUIR IDENTITIES
// =============================================================================
//
// Identity definitions and allocation remain owned by crate::zuir::ids.
// These are re-exports, not standard-library ID implementations.

/// Canonical typed identities used by ZUIR entities.
pub use crate::zuir::ids::{
    BlockId,
    FunctionId,
    IdAllocator,
    IdSpaceExhausted,
    ModuleId,
    OperationId,
    ProgramId,
    RegionId,
    SymbolId,
    TypeId,
    TypedId,
    ValueId,
};

// =============================================================================
// CANONICAL UNIVERSAL IR ENTITIES
// =============================================================================
//
// Every item below is owned by its canonical implementation module.
// This facade does not wrap, reconstruct, or modify those entities.

/// Canonical ZUIR entities, effects, values, resources, and control flow.
pub use crate::zuir::ir::{
    BlockArgument,
    BlockError,
    BlockResult,
    BlockTerminator,
    CaptureMode,
    CapabilityIntent,
    DecimalMagnitude,
    DefinitionKind,
    DialectName,
    DialectRequirement,
    EffectClass,
    EffectCompleteness,
    EffectDefinition,
    EffectError,
    EffectPolicy,
    EffectProperties,
    EffectResult,
    EffectSet,
    EffectSummary,
    FunctionBody,
    FunctionError,
    FunctionLinkage,
    FunctionParameter,
    FunctionSignature,
    ModuleError,
    ModuleName,
    OpaqueOperationPayload,
    OperandError,
    OperandResult,
    OperationError,
    OperationOperand,
    OperationResultType,
    OperationValueResult,
    ProgramError,
    RegionError,
    RegionIsolation,
    RegionKind,
    RegionOwner,
    RegionResult,
    ResourceAssertion,
    ResourceIntent,
    ResourceIntentKind,
    ResourceKey,
    ResourceQuantity,
    ResourceRelation,
    Successor,
    SuccessorError,
    SuccessorResult,
    SwitchCase,
    TopologyIntent,
    TopologyRelation,
    UseDefError,
    UseDefIndex,
    UseDefResult,
    UseSite,
    ValueDefinition,
    ValueUse,
    ZuirBlock,
    ZuirFunction,
    ZuirModule,
    ZuirOperation,
    ZuirProgram,
    ZuirRegion,
};

// =============================================================================
// STANDARD-LIBRARY TYPE ALIASES
// =============================================================================
//
// These aliases provide predictable standard-library-facing names while
// retaining exact canonical ZUIR type identity.
//
// They must never become independent structs or wrapper representations.

/// Standard-library name for the canonical ZUIR program container.
///
/// The generic parameter is the canonical module representation.
pub type StandardIrProgram<M> = ZuirProgram<M>;

/// Standard-library name for the canonical ZUIR module container.
///
/// The generic parameter is the canonical function representation.
pub type StandardIrModule<F> = ZuirModule<F>;

/// Standard-library name for the canonical ZUIR function.
pub type StandardIrFunction = ZuirFunction;

/// Standard-library name for the canonical ZUIR region.
pub type StandardIrRegion = ZuirRegion;

/// Standard-library name for the canonical ZUIR block.
pub type StandardIrBlock = ZuirBlock;

/// Standard-library name for the canonical ZUIR operation.
pub type StandardIrOperation = ZuirOperation;

/// Standard-library name for a canonical ZUIR operation operand.
pub type StandardIrOperand = OperationOperand;

/// Standard-library name for a canonical ZUIR operation result record.
pub type StandardIrValueResult = OperationValueResult;

// =============================================================================
// CANONICAL ZUIR SUBSYSTEMS
// =============================================================================
//
// These paths expose existing owners. They do not create standard-library
// implementations of builders, dialects, verifiers, or lowering.

/// Canonical ZUIR construction APIs.
pub use crate::zuir::builder;


/// Canonical ZUIR dialect definitions, schemas, interfaces, and registry.
pub use crate::zuir::dialect;

/// Canonical ZUIR lowering contracts and adapters.
pub use crate::zuir::lowering;

/// Canonical ZUIR verification APIs.
pub use crate::zuir::verify as verification;

// =============================================================================
// DOMAIN IR AUTHORITIES
// =============================================================================

/// Existing domain-specific IR authorities.
///
/// These are module aliases to the actual repository owners. No domain
/// representation is copied or reimplemented here.
///
/// Hybrid and distributed computation intentionally do not receive
/// fabricated standalone IR modules.
pub mod domains {
    /// Classical IR integration, backed by canonical ZUIR entities.
    pub use crate::classical::zcir::ir as classical;

    /// Hardware-domain IR graph and its canonical structures.
    pub use crate::hdl::hdlir::ir as hdl;

    /// Canonical quantum computational IR.
    pub use crate::quantum::ir as quantum;
}

// =============================================================================
// INTEGRATION CONTRACT TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use core::any::type_name;

    #[test]
    fn standard_program_alias_is_canonical_zuir_program() {
        type StandardProgram =
            StandardIrProgram<StandardIrModule<StandardIrFunction>>;

        type CanonicalProgram =
            crate::zuir::ir::program::ZuirProgram<
                crate::zuir::ir::module::ZuirModule<
                    crate::zuir::ir::function::ZuirFunction,
                >,
            >;

        assert_eq!(
            type_name::<StandardProgram>(),
            type_name::<CanonicalProgram>(),
        );
    }

    #[test]
    fn standard_module_alias_is_canonical_zuir_module() {
        assert_eq!(
            type_name::<StandardIrModule<StandardIrFunction>>(),
            type_name::<crate::zuir::ir::module::ZuirModule<
                crate::zuir::ir::function::ZuirFunction,
            >>(),
        );
    }

    #[test]
    fn standard_function_alias_is_canonical_zuir_function() {
        assert_eq!(
            type_name::<StandardIrFunction>(),
            type_name::<crate::zuir::ir::function::ZuirFunction>(),
        );
    }

    #[test]
    fn standard_block_alias_is_canonical_zuir_block() {
        assert_eq!(
            type_name::<StandardIrBlock>(),
            type_name::<crate::zuir::ir::block::ZuirBlock>(),
        );
    }

    #[test]
    fn standard_region_alias_is_canonical_zuir_region() {
        assert_eq!(
            type_name::<StandardIrRegion>(),
            type_name::<crate::zuir::ir::region::ZuirRegion>(),
        );
    }

    #[test]
    fn standard_operation_alias_is_canonical_zuir_operation() {
        assert_eq!(
            type_name::<StandardIrOperation>(),
            type_name::<crate::zuir::ir::operation::ZuirOperation>(),
        );
    }

    #[test]
    fn standard_operand_alias_is_canonical_zuir_operand() {
        assert_eq!(
            type_name::<StandardIrOperand>(),
            type_name::<crate::zuir::ir::operand::OperationOperand>(),
        );
    }

    #[test]
    fn standard_value_result_alias_is_canonical_zuir_result() {
        assert_eq!(
            type_name::<StandardIrValueResult>(),
            type_name::<crate::zuir::ir::result::OperationValueResult>(),
        );
    }

    #[test]
    fn classical_program_alias_uses_the_same_zuir_program_type() {
        type ClassicalProgram =
            domains::classical::program::ZcirProgram<
                domains::classical::module::ZcirModule<
                    domains::classical::function::ZcirFunction,
                >,
            >;

        type UniversalProgram =
            StandardIrProgram<StandardIrModule<StandardIrFunction>>;

        assert_eq!(
            type_name::<ClassicalProgram>(),
            type_name::<UniversalProgram>(),
        );
    }

    #[test]
    fn canonical_quantum_ir_path_is_exposed() {
        let _ = type_name::<domains::quantum::qubit::QubitId>();
    }

    #[test]
    fn canonical_hdl_ir_path_is_exposed() {
        let _ = type_name::<domains::hdl::design::HdlirDesign<
            domains::hdl::design::HdlirModule,
        >>();
    }
}

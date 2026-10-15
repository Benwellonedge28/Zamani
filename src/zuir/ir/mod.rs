
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Universal Intermediate Representation — Core IR Module
//!
//! Canonical path: `src/zuir/ir/mod.rs`
//! Rust edition: 2021
//! Minimum intended Rust version: 1.97 or later.
//! Safety policy: `unsafe` Rust is forbidden.
//!
//! ## PURPOSE
//!
//! Provide the canonical public module boundary for ZUIR's core
//! intermediate-representation data structures.
//!
//! This file composes the existing IR modules into one stable namespace.
//! It does not implement IR entities, duplicate their definitions, or
//! replace the authoritative implementations in their respective files.
//!
//! ZUIR represents established computational meaning independently of
//! a particular processor, quantum device, FPGA, ASIC, accelerator,
//! operating system, or deployment topology.
//!
//! ## ARCHITECTURAL AUTHORITY
//!
//! This module is subordinate to:
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ids.rs`
//! - `src/zuir/schema/`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/antlr/ZamaniLexer.g4`
//! - `grammar/antlr/ZamaniParser.g4`
//! - `grammar/specification/semantic-model.md`
//! - Applicable domain specifications and semantic contracts.
//!
//! Source syntax belongs to the grammar and frontend.
//! Source meaning belongs to semantic analysis.
//! This module exposes the core ZUIR representation of that meaning.
//!
//! ## OWNERSHIP
//!
//! This file owns only:
//!
//! - IR submodule declarations.
//! - The public composition boundary.
//! - Deliberate re-exports of types owned by their implementation modules.
//!
//! It does not own or redefine:
//!
//! - Identifier types or identifier allocation.
//! - Blocks, block arguments, or terminators.
//! - Control-flow successor edges.
//! - Operations, operands, or operation results.
//! - Regions or region ownership.
//! - Functions, modules, or programs.
//! - Effects, ownership rules, or resource requirements.
//! - Def-use indexes.
//! - Type definitions, schema attributes, or qualified names.
//! - Dialect registration, whole-program verification, or lowering.
//! - Parsing, AST construction, or source-level semantic analysis.
//!
//! ## MODULE CONTRACTS
//!
//! - `block`: block-local structure, arguments, and terminators.
//! - `effects`: effect declarations, summaries, and policies.
//! - `function`: function signatures, bodies, and linkage.
//! - `module`: module declarations, symbols, imports, and exports.
//! - `operand`: operation operand references and operand metadata.
//! - `operation`: operation-local representation and validation.
//! - `ownership`: ownership contracts and use/borrow analysis structures.
//! - `program`: top-level program representation and requirements.
//! - `region`: region structure, captures, and region-local validation.
//! - `resources`: symbolic resource and capability requirements.
//! - `result`: operation result definitions.
//! - `successor`: canonical control-flow successor edges.
//! - `use_def`: value definitions and use-def indexing.
//!
//! Each child module remains the sole owner of its definitions and
//! local invariants. Cross-entity consistency belongs to the full verifier.
//!
//! ## DEPENDENCY DIRECTION
//!
//! The core IR consumes canonical identifiers from `ids.rs` and schema
//! types from `schema/`. Child modules may reference each other's public
//! types where required by the representation.
//!
//! This module must not introduce circular dependency workarounds,
//! duplicate canonical identifiers, or substitute parallel representations.
//!
//! Builders, dialect registries, verifiers, analyses, transformations,
//! serialization, and lowering must depend on these canonical modules.
//! This module must not depend on those higher-level facilities.
//!
//! ## GRAMMAR INTEGRATION
//!
//! The grammar files define source syntax and composition. They are not
//! Rust IR modules and must not be imported here.
//!
//! The canonical pipeline is:
//!
//! source → AST → semantic model → ZUIR → domain IR → target lowering
//!
//! Quantum-specific representations remain under the repository's
//! canonical `quantum::ir` boundary where required. HDL and classical
//! representations likewise retain their designated downstream owners.
//! The universal IR must not silently replace those domain contracts.
//!
//! ## EXTENSIBILITY
//!
//! New computational operations must be introduced through the dialect
//! and operation-definition contracts, not by adding every new operation
//! to this module or creating a universal closed operation enum.
//!
//! New core IR modules may be introduced only when they have a distinct,
//! documented responsibility and an agreed integration contract.
//!
//! ## SCALABILITY AND POCO-REAF
//!
//! This module imposes no universal limit on the number of programs,
//! modules, functions, regions, blocks, operations, values, resources,
//! devices, qubits, processors, or distributed nodes.
//!
//! It introduces no hardware-specific size assumptions or fixed capacity
//! constants. Concrete collection and resource behavior belongs to the
//! owning implementation and configured compilation policies.
//!
//! Literal infinity cannot be guaranteed: memory, address space, time,
//! identifier spaces, and physical resources are finite. The requirement
//! is to avoid artificial universal hardware limits, preserve semantics,
//! and report resource exhaustion explicitly where the underlying API
//! supports doing so.
//!
//! ## VALIDITY AND TRUST
//!
//! Re-exporting a type does not make its contents valid.
//! Child-level validation checks local invariants; the full verifier must
//! establish reference integrity, identity uniqueness, type correctness,
//! SSA and control-flow legality where applicable, region ownership,
//! effect consistency, resource requirements, and dialect validity.
//!
//! Deserialized IR must be verified before trusted optimization,
//! lowering, or execution.
//!
//! ## COMPATIBILITY
//!
//! Public names are re-exported only when doing so does not create
//! ambiguous ownership or duplicate definitions. The original child
//! module paths remain available.
//!
//! Breaking changes to public API or persisted representations must
//! follow the versioning and migration rules in `ZUIR-SPEC.md`.
//!
//! ## FREEZE CRITERIA
//!
//! - Every declared child module exists exactly once.
//! - Re-exports point to canonical types owned by child modules.
//! - No definitions or validation algorithms are duplicated here.
//! - Existing child-module import paths remain usable.
//! - The repository compiles with this module boundary.
//! - Existing tests pass, including integration and serialization tests.
//! - No unsafe code or fixed hardware-size limits are introduced.

/// Basic blocks, block arguments, and terminators.
pub mod block;

/// Effects, effect summaries, and effect policies.
pub mod effects;

/// Function signatures, bodies, and linkage.
pub mod function;

/// Module declarations, symbols, imports, and exports.
pub mod module;

/// Operand references and operand metadata.
pub mod operand;

/// Operation representation, payloads, and local validation.
pub mod operation;

/// Ownership contracts, borrowing, and ownership analysis structures.
pub mod ownership;

/// Top-level ZUIR program representation.
pub mod program;

/// Region structure, captures, and region-local validation.
pub mod region;

/// Symbolic resource requirements and capability intents.
pub mod resources;

/// Operation result definitions.
pub mod result;

/// Explicit control-flow successor edges.
pub mod successor;

/// Value definitions, use sites, and def-use indexing.
pub mod use_def;

// =============================================================================
// Stable, deliberate re-exports
// =============================================================================
//
// These are aliases to canonical definitions, not new implementations.
// The child modules remain available for callers needing more specialized
// APIs or error types.

// Block representation.
pub use block::{
    BlockArgument,
    BlockError,
    BlockResult,
    BlockTerminator,
    SwitchCase,
    ZuirBlock,
};

// Effects.
pub use effects::{
    EffectClass,
    EffectCompleteness,
    EffectDefinition,
    EffectError,
    EffectPolicy,
    EffectProperties,
    EffectResult,
    EffectSet,
    EffectSummary,
};

// Functions.
pub use function::{
    FunctionBody,
    FunctionError,
    FunctionLinkage,
    FunctionParameter,
    FunctionSignature,
    ZuirFunction,
};

// Modules.
pub use module::{
    ModuleError,
    ModuleName,
    ZuirModule,
};

// Operands.
pub use operand::{
    OperationOperand,
    OperandError,
    OperandResult,
};

// Operations.
pub use operation::{
    OpaqueOperationPayload,
    OperationError,
    OperationResultType,
    ZuirOperation,
};

// Ownership.
pub use ownership::{
    BorrowMutability,
    OwnershipCapabilities,
    OwnershipContract,
    OwnershipError,
    OwnershipIndex,
    OwnershipKind,
    OwnershipResult,
    OwnershipUse,
    OwnershipUseKind,
};

// Programs.
pub use program::{
    DialectName,
    DialectRequirement,
    ProgramError,
    ZuirProgram,
};

// Regions.
pub use region::{
    CaptureMode,
    RegionError,
    RegionIsolation,
    RegionKind,
    RegionOwner,
    RegionResult,
    ZuirRegion,
};

// Resources.
pub use resources::{
    CapabilityIntent,
    DecimalMagnitude,
    ResourceAssertion,
    ResourceIntent,
    ResourceIntentKind,
    ResourceKey,
    ResourceQuantity,
    ResourceRelation,
    TopologyIntent,
    TopologyRelation,
};

// Results.
pub use result::OperationValueResult;

// Successors.
pub use successor::{
    Successor,
    SuccessorError,
    SuccessorResult,
};

// Use-def.
pub use use_def::{
    DefinitionKind,
    UseDefError,
    UseDefIndex,
    UseDefResult,
    UseSite,
    ValueDefinition,
    ValueUse,
};

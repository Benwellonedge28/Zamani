
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_debug_implementations)]

//! # Zamani Classical Intermediate Representation (ZCIR) — IR Boundary
//!
//! Canonical path: `src/classical/zcir/ir/mod.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety policy: unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Establish the public IR boundary for Zamani classical computation while
//! reusing the canonical representation owned by `crate::zuir::ir`.
//!
//! ZCIR provides a classical-domain integration surface. It must not create
//! a competing IR graph, identity system, type representation, effect model,
//! control-flow representation, or value-definition system.
//!
//! ## Architectural authority
//!
//! This module is subordinate to:
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ir/mod.rs`
//! - `src/zuir/ids.rs`
//! - `src/zuir/schema/`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//!
//! The grammar defines source syntax. Semantic analysis establishes source
//! meaning. ZUIR owns the canonical shared IR representation. ZCIR exposes
//! that representation for classical-domain integration and must preserve
//! the established ownership boundaries.
//!
//! ## Ownership
//!
//! This file owns only:
//!
//! - The ZCIR IR public API boundary.
//! - Compatibility aliases for canonical ZUIR types.
//! - Re-exports that provide stable classical-facing names.
//!
//! It does not own or implement:
//!
//! - IR nodes or graph storage.
//! - Program, module, function, region, block, or operation identities.
//! - SSA values, operands, results, or use-def storage.
//! - Control-flow terminators or successor edges.
//! - Type definitions, type inference, or type compatibility.
//! - Effect definitions, effect inference, or effect enforcement.
//! - Dialect registration or whole-program verification.
//! - Source parsing, AST construction, or semantic analysis.
//! - Physical registers, addresses, allocation, scheduling, or execution.
//!
//! ## Integration contract
//!
//! - `crate::zuir::ids` owns canonical typed identifiers.
//! - `crate::zuir::ir` owns canonical IR entities and their local contracts.
//! - `crate::zuir::schema` owns canonical schema types and attributes.
//! - The semantic layer supplies resolved meaning before IR construction.
//! - ZCIR builders and verifiers must use these canonical entities.
//! - The quantum IR retains its designated ownership boundary.
//! - Lowering and target selection belong to downstream compiler stages.
//!
//! Re-exporting an entity does not establish whole-program validity.
//! Constructed or deserialized IR must undergo the applicable structural,
//! reference, type, SSA, effect, ownership, and dialect verification before
//! trusted optimization, lowering, or execution.
//!
//! ## Scalability and POCO-REAF
//!
//! This boundary introduces no fixed maximum for classical operations,
//! values, blocks, functions, modules, threads, processors, devices, memory,
//! or collection lengths.
//!
//! It does not impose an artificial machine-size ceiling on the semantics
//! of a program. Actual construction and execution remain subject to finite
//! address space, available memory, processing time, identifier-space limits,
//! implementation capabilities, explicit operational budgets, and target
//! constraints.
//!
//! A target that cannot satisfy a program's requirements must produce an
//! explicit feasibility or compilation result. It must not silently change
//! the program's meaning.
//!
//! ## Module strategy
//!
//! This file intentionally does not declare external child modules such as
//! `block.rs`, `function.rs`, or `operation.rs`: their existence and stable
//! APIs must be established before they can be registered. The public
//! re-exports below compose existing canonical modules without requiring
//! speculative files or creating duplicate definitions.
//!
//! New implementation modules may be registered when their distinct
//! responsibilities, ownership, dependencies, APIs, and tests have been
//! reviewed. They must consume canonical ZUIR entities rather than shadow
//! them.
//!
//! ## Freeze criteria
//!
//! - All exports resolve to existing canonical definitions.
//! - No duplicate IR representation or identifier system is introduced.
//! - The parent ZCIR module can expose this boundary without registering
//!   nonexistent files.
//! - Public API, integration, compatibility, and verification contracts
//!   remain documented.
//! - Formatting, compilation, tests, and linting pass in the repository.
//!
//! This file is a composition boundary, not proof that all ZCIR features,
//! classical operations, backends, or POCO-REAF targets are implemented.

/// Canonical typed identities used by ZCIR IR entities.
///
/// IDs remain opaque identity tokens. They are not array indexes, hardware
/// addresses, or stable cross-compilation content fingerprints.
pub mod ids {
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
}

/// Canonical program-level representation.
///
/// The program container owns its canonical program identity, version
/// metadata, module membership, and dialect requirements.
pub mod program {
    pub use crate::zuir::ir::program::{
        DialectName,
        DialectRequirement,
        ProgramError,
        ZuirProgram,
    };

    /// ZCIR-facing name for the canonical ZUIR program container.
    pub type ZcirProgram<M> = ZuirProgram<M>;
}

/// Canonical module-level representation.
///
/// The function payload remains generic because the canonical ZUIR module
/// container is generic over its function representation.
pub mod module {
    pub use crate::zuir::ir::module::{
        ModuleError,
        ModuleName,
        ZuirModule,
    };

    /// ZCIR-facing name for the canonical ZUIR module container.
    pub type ZcirModule<F> = ZuirModule<F>;
}

/// Canonical function declarations, signatures, and bodies.
pub mod function {
    pub use crate::zuir::ir::function::{
        FunctionBody,
        FunctionError,
        FunctionLinkage,
        FunctionParameter,
        FunctionSignature,
        ZuirFunction,
    };

    /// ZCIR-facing name for the canonical function representation.
    pub type ZcirFunction = ZuirFunction;

    /// ZCIR-facing name for the canonical function signature.
    pub type ZcirFunctionSignature = FunctionSignature;

    /// ZCIR-facing name for the canonical function body.
    pub type ZcirFunctionBody = FunctionBody;
}

/// Canonical basic blocks, block arguments, and terminators.
pub mod block {
    pub use crate::zuir::ir::block::{
        BlockArgument,
        BlockError,
        BlockResult,
        BlockTerminator,
        SwitchCase,
        ZuirBlock,
    };

    /// ZCIR-facing name for the canonical basic block.
    pub type ZcirBlock = ZuirBlock;

    /// ZCIR-facing name for the canonical block terminator.
    pub type ZcirBlockTerminator = BlockTerminator;

    /// ZCIR-facing name for the canonical block argument.
    pub type ZcirBlockArgument = BlockArgument;

    /// ZCIR-facing name for the canonical block error.
    pub type ZcirBlockError = BlockError;

    /// ZCIR-facing result type for block operations.
    pub type ZcirBlockResult<T> = BlockResult<T>;
}

/// Canonical operations, operands, results, and operation-local errors.
pub mod operation {
    pub use crate::zuir::ir::operand::{
        OperandError,
        OperandResult,
        OperationOperand,
    };

    pub use crate::zuir::ir::operation::{
        OpaqueOperationPayload,
        OperationError,
        OperationResultType,
        ZuirOperation,
    };

    pub use crate::zuir::ir::result::OperationValueResult;

    /// ZCIR-facing name for the canonical operation representation.
    pub type ZcirOperation = ZuirOperation;

    /// ZCIR-facing name for a canonical operation operand.
    pub type ZcirOperand = OperationOperand;

    /// ZCIR-facing name for an operation-local error.
    pub type ZcirOperationError = OperationError;

    /// ZCIR-facing result type for operation construction and validation.
    pub type ZcirOperationResult<T> = OperationResultType<T>;
}

/// Canonical SSA value definitions, result records, and use-def structures.
///
/// ZUIR's operation results define produced values. Use-def analysis indexes
/// definitions and uses; this module does not introduce a competing value
/// object or a second SSA identity system.
pub mod value {
    pub use crate::zuir::ir::result::OperationValueResult;

    pub use crate::zuir::ir::use_def::{
        DefinitionKind,
        UseDefError,
        UseDefIndex,
        UseDefResult,
        UseSite,
        ValueDefinition,
        ValueUse,
    };

    /// ZCIR-facing name for a canonical operation-result record.
    pub type ZcirValueResult = OperationValueResult;

    /// ZCIR-facing name for the canonical value-definition record.
    pub type ZcirValueDefinition = ValueDefinition;

    /// ZCIR-facing name for the canonical value-use record.
    pub type ZcirValueUse = ValueUse;

    /// ZCIR-facing name for the canonical use-def index.
    pub type ZcirUseDefIndex = UseDefIndex;
}

/// Canonical regions, ownership, isolation, and captures.
pub mod region {
    pub use crate::zuir::ir::region::{
        CaptureMode,
        RegionError,
        RegionIsolation,
        RegionKind,
        RegionOwner,
        RegionResult,
        ZuirRegion,
    };

    /// ZCIR-facing name for the canonical region.
    pub type ZcirRegion = ZuirRegion;

    /// ZCIR-facing name for the canonical region owner.
    pub type ZcirRegionOwner = RegionOwner;

    /// ZCIR-facing name for a region-local error.
    pub type ZcirRegionError = RegionError;

    /// ZCIR-facing result type for region operations.
    pub type ZcirRegionResult<T> = RegionResult<T>;
}

/// Canonical control-flow edges and successor errors.
pub mod control_flow {
    pub use crate::zuir::ir::successor::{
        Successor,
        SuccessorError,
        SuccessorResult,
    };

    pub use crate::zuir::ir::block::{
        BlockTerminator,
        SwitchCase,
    };

    /// ZCIR-facing name for a canonical control-flow successor edge.
    pub type ZcirSuccessor = Successor;

    /// ZCIR-facing name for the canonical block terminator.
    pub type ZcirControlFlowTerminator = BlockTerminator;

    /// ZCIR-facing name for a canonical switch case.
    pub type ZcirSwitchCase = SwitchCase;

    /// ZCIR-facing name for a successor-edge error.
    pub type ZcirSuccessorError = SuccessorError;

    /// ZCIR-facing result type for successor operations.
    pub type ZcirSuccessorResult<T> = SuccessorResult<T>;
}

/// Canonical effect declarations, summaries, and validation policies.
///
/// Effects are not capabilities, resource requirements, or hardware
/// placement instructions. An absent declaration is not proof of purity;
/// unknown effects must not be silently treated as harmless.
pub mod effects {
    pub use crate::zuir::ir::effects::{
        declared_operation_effects,
        validate_operation_effects,
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

    /// ZCIR-facing result type for effect operations.
    pub type ZcirEffectResult<T> = EffectResult<T>;

    /// ZCIR-facing name for the canonical effect classification.
    pub type ZcirEffectClass = EffectClass;

    /// ZCIR-facing name for the canonical effect definition.
    pub type ZcirEffectDefinition = EffectDefinition;

    /// ZCIR-facing name for the canonical effect set.
    pub type ZcirEffectSet = EffectSet;

    /// ZCIR-facing name for the canonical effect summary.
    pub type ZcirEffectSummary = EffectSummary;

    /// ZCIR-facing name for the canonical effect policy.
    pub type ZcirEffectPolicy = EffectPolicy;
}

// =============================================================================
// Root-level stable API
// =============================================================================
//
// These re-exports make common IR types available directly under
// `crate::classical::zcir::ir` while preserving the more specific namespaces
// above. They refer to canonical definitions; they do not create new types.

pub use crate::zuir::ir::block::{
    BlockArgument,
    BlockError,
    BlockResult,
    BlockTerminator,
    SwitchCase,
    ZuirBlock,
};

pub use crate::zuir::ir::effects::{
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

pub use crate::zuir::ir::function::{
    FunctionBody,
    FunctionError,
    FunctionLinkage,
    FunctionParameter,
    FunctionSignature,
    ZuirFunction,
};

pub use crate::zuir::ir::module::{
    ModuleError,
    ModuleName,
    ZuirModule,
};

pub use crate::zuir::ir::operand::{
    OperandError,
    OperandResult,
    OperationOperand,
};

pub use crate::zuir::ir::operation::{
    OpaqueOperationPayload,
    OperationError,
    OperationResultType,
    ZuirOperation,
};

pub use crate::zuir::ir::program::{
    DialectName,
    DialectRequirement,
    ProgramError,
    ZuirProgram,
};

pub use crate::zuir::ir::region::{
    CaptureMode,
    RegionError,
    RegionIsolation,
    RegionKind,
    RegionOwner,
    RegionResult,
    ZuirRegion,
};

pub use crate::zuir::ir::result::OperationValueResult;

pub use crate::zuir::ir::successor::{
    Successor,
    SuccessorError,
    SuccessorResult,
};

pub use crate::zuir::ir::use_def::{
    DefinitionKind,
    UseDefError,
    UseDefIndex,
    UseDefResult,
    UseSite,
    ValueDefinition,
    ValueUse,
};

// Canonical typed IDs are available from both `ir::ids` and the canonical
// `crate::zuir::ids` module. They are intentionally not redefined here.
pub use crate::zuir::ids::{
    BlockId,
    FunctionId,
    ModuleId,
    OperationId,
    ProgramId,
    RegionId,
    SymbolId,
    TypeId,
    ValueId,
};

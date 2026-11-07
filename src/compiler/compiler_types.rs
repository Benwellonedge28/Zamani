#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani compiler shared types and legacy compatibility contracts.
//!
//! Path: src/compiler/compiler_types.rs
//! Edition: 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! This module contains the small set of compiler-wide types that must
//! remain available to existing compiler, backend, toolchain and
//! compatibility consumers.
//!
//! It is deliberately not a universal semantic model.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - Legacy compiler target identifiers;
//! - Legacy optimization-level identifiers;
//! - The existing CompilerConfig compatibility structure;
//! - Compiler-facing identifier compatibility;
//! - Legacy primitive-width compatibility types;
//! - Legacy reflection/debugging type compatibility;
//! - Legacy diagnostic compatibility.
//!
//! The following repository authorities remain canonical:
//!
//! - Source spans:
//!   crate::frontend::ast::node::source::span
//!
//! - Source maps:
//!   the frontend source infrastructure
//!
//! - AST and source-level types:
//!   crate::frontend::ast
//!
//! - Semantic types, effects and requirements:
//!   crate::frontend::semantic
//!
//! - Canonical diagnostics:
//!   crate::frontend::diagnostics
//!
//! - Extensible compiler-facing target descriptions:
//!   crate::compiler::configuration::target
//!
//! - Target-specific backend descriptions:
//!   crate::backend::target
//!
//! - Backend capability evidence:
//!   crate::backend::capabilities
//!
//! - Backend compatibility:
//!   crate::backend::compatibility
//!
//! - Canonical universal IR:
//!   crate::zuir
//!
//! - Domain-specific IR:
//!   the corresponding classical, quantum, HDL, AI and other
//!   domain-owned subsystems.
//!
//! - Physical resource acquisition and execution:
//!   crate::runtime and its authoritative providers.
//!
//! # Non-responsibilities
//!
//! This module does not:
//!
//! - Parse Zamani source;
//! - Define grammar constructs;
//! - Perform semantic analysis;
//! - Resolve names;
//! - Define canonical source-level types;
//! - Define canonical compiler diagnostics;
//! - Define a second universal IR;
//! - Discover physical hardware;
//! - Enumerate available devices;
//! - Establish backend capabilities;
//! - Evaluate target feasibility;
//! - Allocate hardware resources;
//! - Schedule physical execution;
//! - Select quantum routing;
//! - Perform QEC;
//! - Generate backend code;
//! - Execute programs.
//!
//! # POCO-REAF
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! This module introduces no universal hardware-capacity ceilings.
//!
//! It does not define a maximum for:
//!
//! - CPUs;
//! - CPU cores;
//! - GPUs;
//! - FPGAs;
//! - QPUs;
//! - Physical or logical qubits;
//! - Distributed nodes;
//! - Network participants;
//! - Memory;
//! - Storage;
//! - Tensor rank;
//! - Tensor dimensions;
//! - Register widths;
//! - Modules;
//! - Functions;
//! - IR operations;
//! - Compilation stages;
//! - Backends;
//! - Providers;
//! - Computational domains.
//!
//! The word "infinite" means that the language and compiler architecture
//! must not impose arbitrary universal capacity ceilings.
//!
//! It does not mean that a Rust process, machine or physical device has
//! infinite memory, address space, processing time or resources.
//!
//! Operational budgets belong to:
//!
//!     compiler::configuration::budget
//!
//! Target descriptions belong to:
//!
//!     compiler::configuration::target
//!
//! Capability evidence belongs to the authoritative capability providers.
//!
//! Feasibility belongs to compiler planning and compatibility evaluation.
//!
//! Physical resource acquisition belongs to runtime and hardware providers.
//!
//! # Compatibility policy
//!
//! Several types in this file are retained because existing repository
//! modules use them.
//!
//! Their presence does not make them the preferred authority for new
//! implementations.
//!
//! New code must use the canonical subsystem responsible for the
//! corresponding concept.
//!
//! In particular:
//!
//! - Do not introduce new semantic logic using the legacy Type.
//! - Do not introduce new diagnostic logic using the legacy Diagnostic.
//! - Do not add every new hardware architecture to CompilationTarget.
//! - Do not add target capabilities to CompilerConfig.
//!
//! # Determinism
//!
//! Shared types do not inspect the host machine or environment.
//!
//! They do not perform implicit target discovery.
//!
//! Their formatting and identity operations are deterministic.
//!
//! # Security
//!
//! This module contains no unsafe code.
//!
//! It performs no filesystem, network, process, environment or hardware
//! access.
//!
//! Externally supplied identifiers are not automatically trusted merely
//! because they can be represented by one of these types.

use std::fmt;

use crate::frontend::ast::node::source::span::Span;

// ============================================================================
// IDENTIFIERS
// ============================================================================

/// Compiler-facing identifier with its canonical source location.
///
/// The source span is the frontend's canonical Span.
///
/// This type does not replace AST node identity, semantic symbol identity
/// or the frontend's name-resolution structures.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier(pub String, pub Span);

impl Identifier {
    /// Creates an identifier from its name and source span.
    #[must_use]
    pub fn new(name: impl Into<String>, span: Span) -> Self {
        Self(name.into(), span)
    }

    /// Returns the identifier text.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.0
    }

    /// Returns the canonical source span.
    #[must_use]
    pub const fn span(&self) -> &Span {
        &self.1
    }

    /// Consumes the identifier and returns its name.
    #[must_use]
    pub fn into_name(self) -> String {
        self.0
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

// ============================================================================
// ACCESS MODIFIERS
// ============================================================================

/// Legacy access modifier representation.
///
/// New source-level visibility and access semantics belong to the
/// canonical frontend AST and semantic model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[deprecated(
    note = "Use canonical frontend AST and semantic visibility representations"
)]
pub enum AccessModifier {
    /// Publicly accessible.
    Public,

    /// Private to its declaring scope.
    Private,

    /// Accessible according to the language's protected-access rules.
    Protected,
}

// ============================================================================
// METHOD MODIFIERS
// ============================================================================

/// Legacy method modifier representation.
///
/// This is retained for existing reflection and compatibility consumers.
///
/// New method semantics must be defined by the frontend semantic model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[deprecated(
    note = "Use canonical frontend AST and semantic method modifier representations"
)]
pub enum MethodModifier {
    /// Overrides an inherited implementation.
    Override,

    /// May be overridden by a compatible implementation.
    Virtual,

    /// Declares an implementation-independent method contract.
    Abstract,

    /// Associated with a type rather than an instance.
    Static,
}

// ============================================================================
// LEGACY INTEGER WIDTHS
// ============================================================================

/// Legacy integer-width representation.
///
/// These variants describe familiar language-level integer types.
///
/// They do not assert that every target implements every width natively.
///
/// Target-specific legality and representation belong to the semantic,
/// IR and backend authorities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[deprecated(
    note = "Use canonical frontend and domain-owned scalar type representations"
)]
pub enum IntWidth {
    I8,
    I16,
    I32,
    I64,
    I128,
    ISize,

    U8,
    U16,
    U32,
    U64,
    U128,
    USize,
}

impl IntWidth {
    /// Returns whether this is a signed integer representation.
    #[must_use]
    pub const fn is_signed(self) -> bool {
        matches!(
            self,
            Self::I8
                | Self::I16
                | Self::I32
                | Self::I64
                | Self::I128
                | Self::ISize
        )
    }

    /// Returns whether this is an unsigned integer representation.
    #[must_use]
    pub const fn is_unsigned(self) -> bool {
        !self.is_signed()
    }

    /// Returns the explicit bit width.
    ///
    /// Platform-sized widths intentionally return None because their
    /// concrete width depends on the relevant target ABI.
    #[must_use]
    pub const fn explicit_bit_width(self) -> Option<u16> {
        match self {
            Self::I8 | Self::U8 => Some(8),
            Self::I16 | Self::U16 => Some(16),
            Self::I32 | Self::U32 => Some(32),
            Self::I64 | Self::U64 => Some(64),
            Self::I128 | Self::U128 => Some(128),
            Self::ISize | Self::USize => None,
        }
    }

    /// Returns whether the width depends on the target ABI.
    #[must_use]
    pub const fn is_target_dependent(self) -> bool {
        matches!(self, Self::ISize | Self::USize)
    }
}

// ============================================================================
// LEGACY FLOAT WIDTHS
// ============================================================================

/// Legacy floating-point width representation.
///
/// A declared floating-point format does not prove native target support.
///
/// The backend must establish actual representation and operation support.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[deprecated(
    note = "Use canonical frontend and domain-owned floating-point type representations"
)]
pub enum FloatWidth {
    F32,
    F64,
    F128,
}

impl FloatWidth {
    /// Returns the nominal bit width.
    #[must_use]
    pub const fn bit_width(self) -> u16 {
        match self {
            Self::F32 => 32,
            Self::F64 => 64,
            Self::F128 => 128,
        }
    }
}

// ============================================================================
// LEGACY METHOD TYPE
// ============================================================================

/// Legacy method signature used by reflection compatibility.
///
/// This is not a substitute for canonical function types, semantic
/// signatures, ABI descriptors or IR function types.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[deprecated(
    note = "Use canonical frontend semantic signatures and backend ABI descriptors"
)]
pub struct MethodType {
    /// Method name.
    pub name: String,

    /// Parameter types.
    pub params: Vec<Type>,

    /// Return type.
    pub return_type: Box<Type>,
}

impl MethodType {
    /// Creates a legacy method signature.
    #[must_use]
    pub fn new(
        name: impl Into<String>,
        params: Vec<Type>,
        return_type: Type,
    ) -> Self {
        Self {
            name: name.into(),
            params,
            return_type: Box::new(return_type),
        }
    }
}

// ============================================================================
// LEGACY TYPE COMPATIBILITY
// ============================================================================

/// Legacy type representation retained for existing reflection and
/// debugging consumers.
///
/// # Important
///
/// This is not the canonical Zamani semantic type system.
///
/// The frontend semantic model owns:
///
/// - Type identity;
/// - Type resolution;
/// - Type inference;
/// - Type compatibility;
/// - Generic constraints;
/// - Dependent types;
/// - Effects;
/// - Ownership;
/// - Linear and affine semantics;
/// - Domain-specific type interpretation.
///
/// The compiler must not introduce new semantic analysis using this type.
///
/// Existing consumers such as legacy reflection and debugging code must
/// migrate to canonical semantic type references before this compatibility
/// representation can be removed.
///
/// The recursive representation uses indirection where necessary to keep
/// the Rust type finite.
///
/// No universal maximum array length, tuple length, generic argument count
/// or type nesting depth is encoded here.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[deprecated(
    note = "Legacy compatibility only; use the canonical frontend semantic type model"
)]
pub enum Type {
    /// Unit type.
    Unit,

    /// Boolean type.
    Bool,

    /// Integer type.
    Int(IntWidth),

    /// Floating-point type.
    Float(FloatWidth),

    /// Character type.
    Char,

    /// Borrowed string slice.
    Str,

    /// Owned string.
    String,

    /// Array with an optional known length.
    ///
    /// None represents a length that is not statically known here.
    Array(Box<Type>, Option<usize>),

    /// Slice type.
    Slice(Box<Type>),

    /// Tuple type.
    Tuple(Vec<Type>),

    /// Optional value.
    Option(Box<Type>),

    /// Result value.
    Result(Box<Type>, Box<Type>),

    /// Function type.
    Function(Vec<Type>, Box<Type>),

    /// Immutable reference.
    Ref(Box<Type>),

    /// Mutable reference.
    MutRef(Box<Type>),

    /// Owned value.
    Owned(Box<Type>),

    /// Linear value.
    Linear(Box<Type>),

    /// Affine value.
    Affine(Box<Type>),

    /// Generic type with arguments.
    Generic(String, Vec<Type>),

    /// Unresolved type variable.
    TypeVar(String),

    /// Legacy class representation.
    Class(String, Vec<(String, Box<Type>)>),

    /// Legacy interface representation.
    Interface(String, Vec<MethodType>),

    /// Legacy trait representation.
    Trait(String, Vec<MethodType>),

    /// Quantum-qualified compatibility type.
    ///
    /// This does not replace quantum semantic types or quantum IR types.
    Quantum(Box<Type>),

    /// Nano-qualified compatibility type.
    Nano(Box<Type>),

    /// MTS-qualified compatibility type.
    MTS(Box<Type>),

    /// Sankofa-qualified compatibility type.
    Sankofa(Box<Type>),

    /// Legacy effect identity.
    Effect(String),

    /// Legacy effectful type.
    Effectful(Box<Type>, Vec<String>),

    /// Dependent function type.
    DependentPi(Box<Type>, Box<Type>),

    /// Dependent pair type.
    DependentSigma(Box<Type>, Box<Type>),

    /// Universe level.
    Universe(usize),

    /// Incomplete or unresolved type information.
    Unknown,

    /// Error-recovery type.
    Error,
}

impl Type {
    /// Returns a human-readable compatibility name.
    ///
    /// This method is intended for diagnostics, reflection and debugging.
    ///
    /// It is not a canonical type serializer or stable ABI encoding.
    #[must_use]
    pub fn get_name(&self) -> String {
        match self {
            Self::Unit => "unit".to_owned(),
            Self::Bool => "bool".to_owned(),

            Self::Int(width) => format!("{width:?}").to_lowercase(),

            Self::Float(width) => format!("{width:?}").to_lowercase(),

            Self::Char => "char".to_owned(),
            Self::Str => "str".to_owned(),
            Self::String => "String".to_owned(),

            Self::Array(element, length) => match length {
                Some(length) => {
                    format!("[{}; {}]", element.get_name(), length)
                }
                None => format!("[{}]", element.get_name()),
            },

            Self::Slice(element) => {
                format!("[{}]", element.get_name())
            }

            Self::Tuple(elements) => {
                let names = elements
                    .iter()
                    .map(Self::get_name)
                    .collect::<Vec<_>>()
                    .join(", ");

                format!("({names})")
            }

            Self::Option(inner) => {
                format!("Option<{}>", inner.get_name())
            }

            Self::Result(ok, error) => {
                format!(
                    "Result<{}, {}>",
                    ok.get_name(),
                    error.get_name()
                )
            }

            Self::Function(params, result) => {
                let names = params
                    .iter()
                    .map(Self::get_name)
                    .collect::<Vec<_>>()
                    .join(", ");

                format!("fn({names}) -> {}", result.get_name())
            }

            Self::Ref(inner) => {
                format!("&{}", inner.get_name())
            }

            Self::MutRef(inner) => {
                format!("&mut {}", inner.get_name())
            }

            Self::Owned(inner) => {
                format!("owned<{}>", inner.get_name())
            }

            Self::Linear(inner) => {
                format!("linear<{}>", inner.get_name())
            }

            Self::Affine(inner) => {
                format!("affine<{}>", inner.get_name())
            }

            Self::Generic(name, arguments) => {
                if arguments.is_empty() {
                    name.clone()
                } else {
                    let names = arguments
                        .iter()
                        .map(Self::get_name)
                        .collect::<Vec<_>>()
                        .join(", ");

                    format!("{name}<{names}>")
                }
            }

            Self::TypeVar(name) | Self::Effect(name) => name.clone(),

            Self::Class(name, _) | Self::Interface(name, _) | Self::Trait(name, _) => {
                name.clone()
            }

            Self::Quantum(inner) => {
                format!("quantum<{}>", inner.get_name())
            }

            Self::Nano(inner) => {
                format!("nano<{}>", inner.get_name())
            }

            Self::MTS(inner) => {
                format!("mts<{}>", inner.get_name())
            }

            Self::Sankofa(inner) => {
                format!("sankofa<{}>", inner.get_name())
            }

            Self::Effectful(inner, effects) => {
                format!(
                    "{} ! {{{}}}",
                    inner.get_name(),
                    effects.join(", ")
                )
            }

            Self::DependentPi(domain, codomain) => {
                format!(
                    "Pi({}, {})",
                    domain.get_name(),
                    codomain.get_name()
                )
            }

            Self::DependentSigma(first, second) => {
                format!(
                    "Sigma({}, {})",
                    first.get_name(),
                    second.get_name()
                )
            }

            Self::Universe(level) => {
                format!("Type{level}")
            }

            Self::Unknown => "<unknown>".to_owned(),

            Self::Error => "<error>".to_owned(),
        }
    }

    /// Returns whether this is an integer or floating-point type.
    #[must_use]
    pub const fn is_numeric(&self) -> bool {
        matches!(self, Self::Int(_) | Self::Float(_))
    }

    /// Returns whether this is a Boolean type.
    #[must_use]
    pub const fn is_bool(&self) -> bool {
        matches!(self, Self::Bool)
    }

    /// Returns whether this is the explicit error-recovery type.
    ///
    /// Unknown is intentionally not treated as an error:
    /// it represents unresolved information and must be handled separately.
    #[must_use]
    pub const fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }

    /// Returns whether this type is unresolved.
    #[must_use]
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }

    /// Returns whether this is a quantum-qualified compatibility type.
    ///
    /// This is only a legacy classification helper. It does not establish
    /// quantum capability, physical QPU availability or backend support.
    #[must_use]
    pub const fn is_quantum(&self) -> bool {
        matches!(self, Self::Quantum(_))
    }
}

// ============================================================================
// LEGACY COMPILATION TARGET
// ============================================================================

/// Legacy compilation target selector.
///
/// This enum is retained because existing backend, CLI, test and toolchain
/// code consumes it.
///
/// It is not the extensible target model.
///
/// New target identities and metadata belong to:
///
///     crate::compiler::configuration::target::TargetDescriptor
///
/// Actual target descriptions and capabilities belong to:
///
///     crate::backend::target
///     crate::backend::capabilities
///
/// Compatibility belongs to:
///
///     crate::backend::compatibility
///
/// This enum must not become a closed catalogue of all future hardware.
///
/// Adding a new target should normally require a registered target/provider
/// implementation, not a modification to this enum.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[deprecated(
    note = "Legacy target compatibility; use TargetDescriptor and registered backend providers"
)]
pub enum CompilationTarget {
    /// Legacy x86-64 Linux target.
    X86_64Linux,

    /// Legacy ARM64 target.
    Arm64,

    /// Legacy WebAssembly 32-bit target.
    Wasm32,

    /// Legacy OpenQASM output target.
    QASM,

    /// Legacy Nano control target.
    NanoControl,

    /// Legacy MTS bytecode target.
    MTSBytecode,

    /// Legacy LLVM IR target.
    LLVMIR,

    /// Legacy RISC-V target.
    RiscV,

    /// Legacy extensible exporter identifier.
    ///
    /// This string identifies an exporter format or provider-specific
    /// compatibility target. It does not establish that an exporter exists.
    UniversalIRExport(String),
}

impl CompilationTarget {
    /// Returns whether this target is an exporter-style legacy target.
    #[must_use]
    pub const fn is_export_target(&self) -> bool {
        matches!(self, Self::UniversalIRExport(_))
    }

    /// Returns the exporter identifier when applicable.
    #[must_use]
    pub fn export_identifier(&self) -> Option<&str> {
        match self {
            Self::UniversalIRExport(identifier) => Some(identifier),
            _ => None,
        }
    }
}

// ============================================================================
// LEGACY OPTIMIZATION LEVEL
// ============================================================================

/// Legacy optimization-level selector.
///
/// Optimization intent, legality, pass availability and semantic
/// preservation are established by the canonical optimization and
/// pass-management subsystems.
///
/// A level is not proof that every domain has an implementation for it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[deprecated(
    note = "Legacy optimization compatibility; use compiler optimization policies and pass registries"
)]
pub enum OptimizationLevel {
    /// No optional optimization requested.
    None,

    /// Basic optimization policy.
    Basic,

    /// Aggressive optimization policy.
    Aggressive,

    /// Experimental legacy optimization policy.
    UltraAGI,
}

impl OptimizationLevel {
    /// Returns whether this is the no-optimization setting.
    #[must_use]
    pub const fn is_disabled(self) -> bool {
        matches!(self, Self::None)
    }

    /// Returns whether this is the legacy experimental level.
    #[must_use]
    pub const fn is_experimental(self) -> bool {
        matches!(self, Self::UltraAGI)
    }
}

// ============================================================================
// LEGACY COMPILER CONFIGURATION
// ============================================================================

/// Existing compiler configuration compatibility structure.
///
/// This structure is deliberately kept field-compatible with existing
/// repository consumers.
///
/// Do not add target capability, physical resource, scheduling, routing,
/// provider registry or semantic requirement fields here.
///
/// Those responsibilities have separate authoritative modules.
///
/// # Legacy default
///
/// The existing Default implementation is retained for compatibility with
/// old backend and CLI consumers. It selects X86_64Linux.
///
/// That default is NOT a portable POCO-REAF target-selection policy.
///
/// New production compilation entry points must require an explicit target
/// descriptor or use an explicitly designed target-independent compilation
/// request. They must not rely on this legacy default.
///
/// # Operational budgets
///
/// The `parallel` field is a legacy Boolean request. It is not a worker
/// count, CPU count, thread limit or resource guarantee.
///
/// Real compilation budgets belong to:
///
///     crate::compiler::configuration::budget
///
/// # Verification
///
/// The `verify` field expresses a request to perform applicable verification.
/// It does not replace canonical IR verification or backend artifact
/// validation.
///
/// # Debug information
///
/// The `debug_info` field is a legacy output preference. Actual debug
/// information support depends on the selected backend and artifact format.
#[derive(Debug, Clone, PartialEq, Eq)]
#[deprecated(
    note = "Legacy compatibility structure; prefer compiler API requests and configuration contracts"
)]
pub struct CompilerConfig {
    /// Legacy backend target selector.
    pub target: CompilationTarget,

    /// Legacy optimization level.
    pub opt_level: OptimizationLevel,

    /// Whether debug information is requested.
    pub debug_info: bool,

    /// Whether applicable verification is requested.
    pub verify: bool,

    /// Whether an intermediate representation dump is requested.
    pub emit_ir: bool,

    /// Whether parallel compilation is requested.
    ///
    /// This is not a fixed worker count or hardware capacity.
    pub parallel: bool,
}

impl Default for CompilerConfig {
    /// Constructs the legacy-compatible default configuration.
    ///
    /// Important: this is retained for compatibility only.
    ///
    /// Production POCO-REAF compilation must not interpret the selected
    /// legacy target as a universal default.
    fn default() -> Self {
        Self {
            target: CompilationTarget::X86_64Linux,
            opt_level: OptimizationLevel::Basic,
            debug_info: true,
            verify: true,
            emit_ir: false,
            parallel: false,
        }
    }
}

impl CompilerConfig {
    /// Creates a legacy configuration for an explicit target.
    #[must_use]
    pub fn for_target(target: CompilationTarget) -> Self {
        Self {
            target,
            ..Self::default()
        }
    }

    /// Returns a copy with the requested optimization level.
    #[must_use]
    pub fn with_optimization_level(
        mut self,
        level: OptimizationLevel,
    ) -> Self {
        self.opt_level = level;
        self
    }

    /// Returns a copy with debug information enabled or disabled.
    #[must_use]
    pub const fn with_debug_info(mut self, enabled: bool) -> Self {
        self.debug_info = enabled;
        self
    }

    /// Returns a copy with verification enabled or disabled.
    #[must_use]
    pub const fn with_verification(mut self, enabled: bool) -> Self {
        self.verify = enabled;
        self
    }

    /// Returns a copy with IR emission enabled or disabled.
    #[must_use]
    pub const fn with_ir_emission(mut self, enabled: bool) -> Self {
        self.emit_ir = enabled;
        self
    }

    /// Returns a copy with parallel compilation requested or not requested.
    ///
    /// Actual parallelism remains controlled by the compiler scheduler,
    /// operational budgets and available resources.
    #[must_use]
    pub const fn with_parallel_compilation(
        mut self,
        enabled: bool,
    ) -> Self {
        self.parallel = enabled;
        self
    }
}

// ============================================================================
// LEGACY DIAGNOSTIC LEVEL
// ============================================================================

/// Legacy diagnostic severity.
///
/// New diagnostics must use the canonical frontend diagnostic severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[deprecated(
    note = "Use crate::frontend::diagnostics canonical severity types"
)]
pub enum DiagnosticLevel {
    /// Error.
    Error,

    /// Warning.
    Warning,

    /// Informational note.
    Note,

    /// Additional help.
    Help,
}

// ============================================================================
// LEGACY DIAGNOSTIC
// ============================================================================

/// Legacy diagnostic representation.
///
/// This type is retained for compatibility only.
///
/// New compiler components must use the canonical frontend diagnostic
/// model, which owns diagnostic codes, phases, categories, labels, notes,
/// help, suggestions, fixes and source relationships.
///
/// This type must not be used to create a second diagnostic pipeline.
#[derive(Debug, Clone, PartialEq, Eq)]
#[deprecated(
    note = "Use crate::frontend::diagnostics canonical Diagnostic"
)]
pub struct Diagnostic {
    /// Legacy diagnostic level.
    pub level: DiagnosticLevel,

    /// Human-readable message.
    pub message: String,

    /// Optional canonical source span.
    pub span: Option<Span>,

    /// Supplemental notes.
    pub notes: Vec<String>,
}

impl Diagnostic {
    /// Constructs a legacy error diagnostic.
    #[must_use]
    pub fn error(
        message: impl Into<String>,
        span: Option<Span>,
    ) -> Self {
        Self {
            level: DiagnosticLevel::Error,
            message: message.into(),
            span,
            notes: Vec::new(),
        }
    }

    /// Constructs a legacy warning diagnostic.
    #[must_use]
    pub fn warning(
        message: impl Into<String>,
        span: Option<Span>,
    ) -> Self {
        Self {
            level: DiagnosticLevel::Warning,
            message: message.into(),
            span,
            notes: Vec::new(),
        }
    }

    /// Appends a supplemental note.
    #[must_use]
    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }

    /// Returns whether this is an error.
    #[must_use]
    pub const fn is_error(&self) -> bool {
        matches!(self.level, DiagnosticLevel::Error)
    }

    /// Returns whether this is a warning.
    #[must_use]
    pub const fn is_warning(&self) -> bool {
        matches!(self.level, DiagnosticLevel::Warning)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::frontend::ast::node::source::span::{
        SourceId,
        SourceOffset,
    };

    fn test_span() -> Span {
        Span::new(
            SourceId::from_raw(1),
            SourceOffset::from_raw(0),
            SourceOffset::from_raw(4),
        )
        .expect("valid test span")
    }

    #[test]
    fn identifier_preserves_canonical_span() {
        let span = test_span();
        let identifier = Identifier::new("example", span);

        assert_eq!(identifier.name(), "example");
        assert_eq!(*identifier.span(), span);
        assert_eq!(identifier.to_string(), "example");
    }

    #[test]
    fn integer_width_classifies_signed_and_unsigned_values() {
        assert!(IntWidth::I32.is_signed());
        assert!(!IntWidth::I32.is_unsigned());

        assert!(IntWidth::U64.is_unsigned());
        assert!(!IntWidth::U64.is_signed());
    }

    #[test]
    fn platform_sized_integer_width_is_not_assumed() {
        assert_eq!(IntWidth::ISize.explicit_bit_width(), None);
        assert_eq!(IntWidth::USize.explicit_bit_width(), None);

        assert!(IntWidth::ISize.is_target_dependent());
        assert!(IntWidth::USize.is_target_dependent());
    }

    #[test]
    fn explicit_integer_width_is_preserved() {
        assert_eq!(IntWidth::I8.explicit_bit_width(), Some(8));
        assert_eq!(IntWidth::U16.explicit_bit_width(), Some(16));
        assert_eq!(IntWidth::I32.explicit_bit_width(), Some(32));
        assert_eq!(IntWidth::U64.explicit_bit_width(), Some(64));
        assert_eq!(IntWidth::I128.explicit_bit_width(), Some(128));
    }

    #[test]
    fn floating_width_reports_nominal_width() {
        assert_eq!(FloatWidth::F32.bit_width(), 32);
        assert_eq!(FloatWidth::F64.bit_width(), 64);
        assert_eq!(FloatWidth::F128.bit_width(), 128);
    }

    #[test]
    fn legacy_type_names_are_deterministic() {
        assert_eq!(Type::Unit.get_name(), "unit");
        assert_eq!(Type::Bool.get_name(), "bool");
        assert_eq!(
            Type::Quantum(Box::new(Type::Bool)).get_name(),
            "quantum<bool>"
        );
        assert_eq!(
            Type::Array(Box::new(Type::Int(IntWidth::I32)), Some(8))
                .get_name(),
            "[i32; 8]"
        );
    }

    #[test]
    fn unknown_type_is_not_an_error_recovery_type() {
        assert!(!Type::Unknown.is_error());
        assert!(Type::Unknown.is_unknown());

        assert!(Type::Error.is_error());
        assert!(!Type::Error.is_unknown());
    }

    #[test]
    fn numeric_type_classification_is_consistent() {
        assert!(Type::Int(IntWidth::I64).is_numeric());
        assert!(Type::Float(FloatWidth::F64).is_numeric());

        assert!(!Type::Bool.is_numeric());
        assert!(!Type::String.is_numeric());
    }

    #[test]
    fn legacy_target_export_identity_is_preserved() {
        let target =
            CompilationTarget::UniversalIRExport("portable-ir".to_owned());

        assert!(target.is_export_target());
        assert_eq!(
            target.export_identifier(),
            Some("portable-ir")
        );
    }

    #[test]
    fn legacy_compiler_config_preserves_existing_fields() {
        let config = CompilerConfig {
            target: CompilationTarget::Arm64,
            opt_level: OptimizationLevel::Aggressive,
            debug_info: false,
            verify: true,
            emit_ir: true,
            parallel: true,
        };

        assert_eq!(config.target, CompilationTarget::Arm64);
        assert_eq!(config.opt_level, OptimizationLevel::Aggressive);
        assert!(!config.debug_info);
        assert!(config.verify);
        assert!(config.emit_ir);
        assert!(config.parallel);
    }

    #[test]
    fn compiler_config_builder_methods_preserve_other_fields() {
        let config = CompilerConfig::for_target(CompilationTarget::Wasm32)
            .with_optimization_level(OptimizationLevel::None)
            .with_debug_info(false)
            .with_verification(true)
            .with_ir_emission(true)
            .with_parallel_compilation(true);

        assert_eq!(config.target, CompilationTarget::Wasm32);
        assert_eq!(config.opt_level, OptimizationLevel::None);
        assert!(!config.debug_info);
        assert!(config.verify);
        assert!(config.emit_ir);
        assert!(config.parallel);
    }

    #[test]
    fn legacy_diagnostic_preserves_canonical_source_span() {
        let span = test_span();

        let diagnostic =
            Diagnostic::error("invalid expression", Some(span))
                .with_note("check the expression");

        assert!(diagnostic.is_error());
        assert_eq!(diagnostic.span, Some(span));
        assert_eq!(diagnostic.notes.len(), 1);
    }
}
//! Zamani Quantum IR — External Call Model
//!
//! Production-grade, hardware-independent representation of calls to
//! externally defined classical, quantum, hybrid, mathematical, runtime,
//! library, or service operations.
//!
//! # Architectural role
//!
//! `extern_call.rs` defines the SEMANTICS of an external call.
//!
//! It describes:
//!
//! - what external symbol is being invoked;
//! - what arguments are supplied;
//! - what results are expected;
//! - which logical quantum resources are involved;
//! - which classical IR values are involved;
//! - whether the call may have side effects;
//! - whether the call is deterministic;
//! - whether the call may block;
//! - whether the call is allowed to interact with external state;
//! - optional ABI/linkage metadata;
//! - extensible attributes that downstream layers may interpret.
//!
//! It does NOT perform the call.
//!
//! It does NOT own:
//!
//! - dynamic library loading;
//! - operating-system processes;
//! - network transport;
//! - RPC;
//! - credentials;
//! - filesystem access;
//! - CPU execution;
//! - GPU execution;
//! - QPU execution;
//! - simulator execution;
//! - backend-specific ABI implementation;
//! - vendor SDKs;
//! - scheduling;
//! - routing;
//! - optimization;
//! - hardware allocation;
//! - frontend parsing.
//!
//! Those responsibilities belong to downstream runtime, linker, backend,
//! hardware, or frontend subsystems.
//!
//! # Universal-program principle
//!
//! A Zamani program is written once and may ultimately execute on machines of
//! radically different sizes and architectures.
//!
//! An external call therefore MUST NOT encode:
//!
//! - a fixed number of qubits;
//! - a fixed number of arguments;
//! - a fixed number of results;
//! - a fixed register width;
//! - a fixed ABI;
//! - a fixed CPU architecture;
//! - a fixed QPU architecture;
//! - a fixed vendor;
//! - a fixed operating system;
//! - a fixed network;
//! - a fixed memory address.
//!
//! Argument and result collections are dynamically sized.
//!
//! The practical maximum is determined only by available resources and
//! explicit compiler/runtime/security limits.
//!
//! There is no semantic "maximum number of arguments" or "maximum number of
//! results" in this module.
//!
//! # Quantum integration
//!
//! Quantum resources are represented using the canonical:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! This is important because external calls can participate in hybrid
//! computation.
//!
//! For example:
//!
//! ```text
//! external quantum operation
//!     (q0, q1, theta)
//!         │
//!         ▼
//! external result
//! ```
//!
//! The call remains hardware-independent. A `QubitId` identifies a logical
//! quantum resource. Mapping that logical resource to physical hardware is a
//! downstream responsibility.
//!
//! # Classical integration
//!
//! Classical values are represented using the canonical classical value model
//! where available, while this module deliberately keeps its argument/result
//! references generic enough to avoid making the external-call abstraction
//! depend on a particular evaluator or runtime representation.
//!
//! The preferred integration points are:
//!
//! ```text
//! quantum::ir::classical::ClassicalBitId
//! quantum::ir::identity::ValueId
//! quantum::ir::classical::value::ClassicalValue
//! ```
//!
//! This module uses `ValueId` for SSA-like IR values and
//! `ClassicalBitId` for explicitly addressed classical bits.
//!
//! # Why this is not a normal function call
//!
//! An ordinary IR function may be fully defined inside the program.
//!
//! An external call is different:
//!
//! ```text
//! Zamani IR
//!    │
//!    └── external symbol
//!             │
//!             ▼
//!       linker/runtime/backend
//!             │
//!       ┌─────┼─────┐
//!       ▼     ▼     ▼
//!      CPU   QPU    service
//! ```
//!
//! Therefore the IR records the semantic dependency without deciding how the
//! dependency is resolved.
//!
//! # Side effects
//!
//! External calls must explicitly describe their effect contract.
//!
//! A call may be:
//!
//! - pure;
//! - read-only;
//! - stateful;
//! - nondeterministic;
//! - externally observable;
//! - quantum-state modifying;
//! - classical-state modifying;
//! - unknown.
//!
//! Optimizers MUST NOT assume an external call is pure unless the call's
//! effect contract explicitly permits that assumption.
//!
//! # Determinism
//!
//! Determinism is represented explicitly.
//!
//! This matters for:
//!
//! - optimization;
//! - memoization;
//! - reproducibility;
//! - distributed compilation;
//! - canonical hashing;
//! - testing.
//!
//! A nondeterministic external call cannot safely be replaced by a cached
//! result merely because its arguments are identical.
//!
//! # Security boundary
//!
//! An external call is a potential security boundary.
//!
//! This module therefore represents permission/side-effect intent but never
//! grants execution authority.
//!
//! Runtime authorization belongs to the runtime capability system.
//!
//! For example:
//!
//! ```text
//! IR says:
//!     external call requires Network capability
//!
//! Runtime decides:
//!     allowed / denied
//! ```
//!
//! The IR must never contain credentials or secrets.
//!
//! # ABI
//!
//! ABI information is represented as declarative metadata.
//!
//! The IR does not load or execute an ABI.
//!
//! ABI information can describe:
//!
//! - calling convention;
//! - symbol linkage;
//! - argument encoding;
//! - result encoding;
//! - endianness;
//! - alignment requirements;
//! - opaque vendor/application metadata.
//!
//! Target-specific ABI lowering remains outside the canonical semantic IR.
//!
//! # Extensibility
//!
//! External calls must remain usable when Zamani encounters a future
//! execution model.
//!
//! Therefore the model supports:
//!
//! - arbitrary symbol names;
//! - optional namespace/library/module;
//! - dynamically sized arguments;
//! - dynamically sized results;
//! - arbitrary attributes;
//! - opaque linkage metadata;
//! - capability requirements;
//! - quantum operands;
//! - classical operands;
//! - future operand kinds.
//!
//! Unknown extension information must be preserved by serialization layers
//! whenever the selected serialization format supports preservation.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition.
//!
//! Requirements:
//!
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies;
//! - no `unsafe`.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! # Integration contracts
//!
//! `identity.rs`
//!     Supplies `ValueId` and `OperationId`.
//!
//! `qubit.rs`
//!     Supplies the canonical `QubitId`.
//!
//! `classical.rs`
//!     Supplies `ClassicalBitId`.
//!
//! `classical/value.rs`
//!     Provides the canonical semantic classical-value model.
//!
//! `operation.rs`
//!     Represents this model as the semantic body of an external operation.
//!
//! `program.rs`
//!     Owns symbol declarations and complete program structure.
//!
//! `validation.rs`
//!     Verifies that referenced values, qubits, symbols, capabilities, and
//!     operations exist in the enclosing program.
//!
//! `serialization.rs`
//!     Serializes the deterministic structure.
//!
//! `hash.rs`
//!     Includes semantic fields in canonical content hashing.
//!
//! `analysis.rs`
//!     Inspects arguments, results, effects, and resource dependencies.
//!
//! `capability.rs`
//!     Resolves required execution capabilities.
//!
//! `runtime/`
//!     Eventually performs authorization and actual external execution.
//!
//! `backend/`
//!     Lowers the abstract external call to a target-specific representation.
//!
//! # Ownership rule
//!
//! This file owns:
//!
//!     WHAT an external call means.
//!
//! It does not own:
//!
//!     WHERE it executes.
//!     WHEN it executes.
//!     HOW it executes.
//!
//! Those decisions remain outside the canonical semantic IR.
//!
//! # No fixed machine-size assumptions
//!
//! Nothing in this module assumes:
//!
//! - one qubit;
//! - two qubits;
//! - 32 qubits;
//! - 64 qubits;
//! - 127 qubits;
//! - 1024 qubits;
//! - any particular number of classical bits;
//! - any particular number of arguments.
//!
//! All collections scale according to available resources and explicit
//! compilation/security policies.
//!
//! # Module placement
//!
//! Canonical path:
//!
//! ```text
//! src/quantum/ir/classical/extern_call.rs
//! ```
//!
//! Parent module:
//!
//! ```text
//! src/quantum/ir/classical/mod.rs
//! ```
//!
//! The parent module should expose this module with:
//!
//! ```rust
//! pub mod extern_call;
//! ```
//!
//! and may re-export:
//!
//! ```rust
//! pub use extern_call::{ExternalCall, ExternalCallArgument, ...};
//! ```
//!
//! No changes to this file should be required when another IR subsystem is
//! subsequently implemented, provided the integration contracts described
//! above are preserved.

#![forbid(unsafe_code)]

use std::fmt;

use super::super::identity::{
    CapabilityId,
    OperationId,
    ValueId,
};

use super::super::qubit::QubitId;

use super::ClassicalBitId;

// =============================================================================
// Result type
// =============================================================================

/// Result type used by checked external-call constructors and mutations.
pub type ExternalCallResult<T> = Result<T, ExternalCallError>;

// =============================================================================
// Symbol
// =============================================================================

/// Canonical external symbol name.
///
/// The symbol is deliberately represented as an owned UTF-8 string rather
/// than a Rust identifier so that external ecosystems may use naming
/// conventions that are not valid Rust identifiers.
///
/// Examples:
///
/// ```text
/// "sin"
/// "blas.matmul"
/// "qpu.measure"
/// "vendor.operation"
/// "library::function"
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExternalSymbol(String);

impl ExternalSymbol {
    /// Creates an external symbol after validating that it is non-empty.
    pub fn new<S>(name: S) -> ExternalCallResult<Self>
    where
        S: Into<String>,
    {
        let name = name.into();

        if name.is_empty() {
            return Err(ExternalCallError::EmptySymbol);
        }

        if name.as_bytes().contains(&0) {
            return Err(ExternalCallError::NulInSymbol);
        }

        Ok(Self(name))
    }

    /// Returns the symbol as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns the symbol length in bytes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns whether the symbol is empty.
    ///
    /// A successfully constructed `ExternalSymbol` is never empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        false
    }
}

impl fmt::Display for ExternalSymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl AsRef<str> for ExternalSymbol {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

// =============================================================================
// Namespace
// =============================================================================

/// Optional namespace/library/module qualification for an external symbol.
///
/// This is semantic linkage information only. It does not load a library.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExternalNamespace(String);

impl ExternalNamespace {
    /// Creates a namespace.
    pub fn new<S>(name: S) -> ExternalCallResult<Self>
    where
        S: Into<String>,
    {
        let name = name.into();

        if name.is_empty() {
            return Err(ExternalCallError::EmptyNamespace);
        }

        if name.as_bytes().contains(&0) {
            return Err(ExternalCallError::NulInNamespace);
        }

        Ok(Self(name))
    }

    /// Returns the namespace.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ExternalNamespace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

// =============================================================================
// Linkage
// =============================================================================

/// Declarative linkage model for an external call.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ExternalLinkage {
    /// Resolve through the normal runtime/linker environment.
    Default,

    /// Resolve a symbol from a named library/module.
    Library {
        /// Logical library/module name.
        name: ExternalNamespace,
    },

    /// Resolve through a language/runtime namespace.
    Runtime {
        /// Runtime namespace.
        namespace: ExternalNamespace,
    },

    /// Resolve through a vendor-specific namespace.
    Vendor {
        /// Vendor namespace.
        namespace: ExternalNamespace,
    },

    /// Resolve through an application-defined namespace.
    Application {
        /// Application namespace.
        namespace: ExternalNamespace,
    },

    /// Opaque linkage kind for an extension.
    Opaque {
        /// Stable extension name.
        name: ExternalSymbol,
    },
}

impl Default for ExternalLinkage {
    fn default() -> Self {
        Self::Default
    }
}

impl fmt::Display for ExternalLinkage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Default => f.write_str("default"),

            Self::Library { name } => {
                write!(f, "library:{name}")
            }

            Self::Runtime { namespace } => {
                write!(f, "runtime:{namespace}")
            }

            Self::Vendor { namespace } => {
                write!(f, "vendor:{namespace}")
            }

            Self::Application { namespace } => {
                write!(f, "application:{namespace}")
            }

            Self::Opaque { name } => {
                write!(f, "opaque:{name}")
            }
        }
    }
}

// =============================================================================
// ABI
// =============================================================================

/// Declarative ABI metadata.
///
/// ABI information is intentionally represented without prescribing one
/// backend or CPU architecture.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExternalAbi {
    /// Optional calling convention name.
    calling_convention: Option<ExternalSymbol>,

    /// Optional encoding name.
    encoding: Option<ExternalSymbol>,

    /// Optional ABI version.
    version: Option<String>,

    /// Whether the ABI requires target-native representation.
    target_native: bool,
}

impl ExternalAbi {
    /// Creates a platform-independent ABI description.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            calling_convention: None,
            encoding: None,
            version: None,
            target_native: false,
        }
    }

    /// Sets the calling convention.
    pub fn with_calling_convention(
        mut self,
        convention: ExternalSymbol,
    ) -> Self {
        self.calling_convention = Some(convention);
        self
    }

    /// Sets the argument/result encoding.
    pub fn with_encoding(
        mut self,
        encoding: ExternalSymbol,
    ) -> Self {
        self.encoding = Some(encoding);
        self
    }

    /// Sets an ABI version string.
    pub fn with_version<S>(
        mut self,
        version: S,
    ) -> Self
    where
        S: Into<String>,
    {
        self.version = Some(version.into());
        self
    }

    /// Marks the ABI as target-native.
    #[must_use]
    pub const fn target_native(mut self) -> Self {
        self.target_native = true;
        self
    }

    /// Returns the calling convention.
    #[must_use]
    pub fn calling_convention(&self) -> Option<&ExternalSymbol> {
        self.calling_convention.as_ref()
    }

    /// Returns the encoding.
    #[must_use]
    pub fn encoding(&self) -> Option<&ExternalSymbol> {
        self.encoding.as_ref()
    }

    /// Returns the ABI version.
    #[must_use]
    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }

    /// Returns whether target-native representation is required.
    #[must_use]
    pub const fn is_target_native(&self) -> bool {
        self.target_native
    }
}

impl Default for ExternalAbi {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Effects
// =============================================================================

/// Semantic effect classification for an external call.
///
/// This is deliberately an effect contract rather than an execution policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ExternalEffect {
    /// The call has no externally observable side effects.
    Pure,

    /// The call reads external state but does not modify it.
    ReadOnly,

    /// The call modifies classical/external state.
    Stateful,

    /// The call may modify quantum state.
    QuantumState,

    /// The call may modify both quantum and classical state.
    HybridState,

    /// The result may vary even for identical inputs.
    Nondeterministic,

    /// The effect is intentionally not known to the compiler.
    Unknown,
}

impl ExternalEffect {
    /// Returns whether the call can safely be treated as pure for
    /// transformation purposes.
    ///
    /// Only `Pure` is considered pure.
    #[must_use]
    pub const fn is_pure(self) -> bool {
        matches!(self, Self::Pure)
    }

    /// Returns whether the call is known to be potentially stateful.
    #[must_use]
    pub const fn may_have_side_effects(self) -> bool {
        matches!(
            self,
            Self::Stateful
                | Self::QuantumState
                | Self::HybridState
                | Self::Nondeterministic
                | Self::Unknown
        )
    }
}

impl Default for ExternalEffect {
    fn default() -> Self {
        Self::Unknown
    }
}

// =============================================================================
// Execution semantics
// =============================================================================

/// Declarative execution properties of an external call.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExternalExecutionProperties {
    /// Semantic effect.
    effect: ExternalEffect,

    /// Whether execution may block.
    may_block: bool,

    /// Whether execution may access resources outside the IR.
    external_state: bool,

    /// Whether execution is safe to memoize when inputs are identical.
    memoizable: bool,
}

impl ExternalExecutionProperties {
    /// Creates conservative execution properties.
    #[must_use]
    pub const fn conservative() -> Self {
        Self {
            effect: ExternalEffect::Unknown,
            may_block: true,
            external_state: true,
            memoizable: false,
        }
    }

    /// Creates properties for a pure deterministic computation.
    #[must_use]
    pub const fn pure() -> Self {
        Self {
            effect: ExternalEffect::Pure,
            may_block: false,
            external_state: false,
            memoizable: true,
        }
    }

    /// Returns the effect classification.
    #[must_use]
    pub const fn effect(self) -> ExternalEffect {
        self.effect
    }

    /// Returns whether execution may block.
    #[must_use]
    pub const fn may_block(self) -> bool {
        self.may_block
    }

    /// Returns whether external state may be accessed.
    #[must_use]
    pub const fn accesses_external_state(self) -> bool {
        self.external_state
    }

    /// Returns whether memoization is semantically permitted.
    #[must_use]
    pub const fn is_memoizable(self) -> bool {
        self.memoizable
    }

    /// Changes the effect classification.
    #[must_use]
    pub const fn with_effect(
        mut self,
        effect: ExternalEffect,
    ) -> Self {
        self.effect = effect;
        self
    }

    /// Changes blocking semantics.
    #[must_use]
    pub const fn with_may_block(
        mut self,
        may_block: bool,
    ) -> Self {
        self.may_block = may_block;
        self
    }

    /// Changes external-state semantics.
    #[must_use]
    pub const fn with_external_state(
        mut self,
        external_state: bool,
    ) -> Self {
        self.external_state = external_state;
        self
    }

    /// Changes memoization permission.
    #[must_use]
    pub const fn with_memoizable(
        mut self,
        memoizable: bool,
    ) -> Self {
        self.memoizable = memoizable;
        self
    }
}

impl Default for ExternalExecutionProperties {
    fn default() -> Self {
        Self::conservative()
    }
}

// =============================================================================
// External-call argument
// =============================================================================

/// One argument to an external call.
///
/// The argument is a reference to an IR resource/value rather than a copied
/// runtime value.
///
/// This keeps the canonical IR compact and permits SSA-style dataflow.
///
/// Quantum operands explicitly use [`QubitId`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ExternalCallArgument {
    /// A logical quantum resource.
    Qubit(QubitId),

    /// A logical classical bit.
    ClassicalBit(ClassicalBitId),

    /// An SSA-like IR value.
    Value(ValueId),

    /// A nested operation result.
    OperationResult(OperationId),

    /// An extension-defined resource/value.
    Opaque(ValueId),
}

impl ExternalCallArgument {
    /// Returns the argument's broad kind.
    #[must_use]
    pub const fn kind(self) -> ExternalArgumentKind {
        match self {
            Self::Qubit(_) => ExternalArgumentKind::Qubit,
            Self::ClassicalBit(_) => ExternalArgumentKind::ClassicalBit,
            Self::Value(_) => ExternalArgumentKind::Value,
            Self::OperationResult(_) => ExternalArgumentKind::OperationResult,
            Self::Opaque(_) => ExternalArgumentKind::Opaque,
        }
    }

    /// Returns the qubit if this is a quantum argument.
    #[must_use]
    pub const fn as_qubit(self) -> Option<QubitId> {
        match self {
            Self::Qubit(qubit) => Some(qubit),
            _ => None,
        }
    }

    /// Returns the classical bit if this is a classical-bit argument.
    #[must_use]
    pub const fn as_classical_bit(self) -> Option<ClassicalBitId> {
        match self {
            Self::ClassicalBit(bit) => Some(bit),
            _ => None,
        }
    }

    /// Returns the referenced IR value if this argument has one.
    #[must_use]
    pub const fn as_value(self) -> Option<ValueId> {
        match self {
            Self::Value(value) | Self::Opaque(value) => Some(value),
            Self::OperationResult(_) | Self::Qubit(_) | Self::ClassicalBit(_) => None,
        }
    }
}

/// Broad category of an external-call argument.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ExternalArgumentKind {
    /// Quantum resource.
    Qubit,

    /// Classical bit resource.
    ClassicalBit,

    /// IR value.
    Value,

    /// Existing operation result.
    OperationResult,

    /// Extension-defined value.
    Opaque,
}

impl fmt::Display for ExternalArgumentKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Qubit => "qubit",
            Self::ClassicalBit => "classical_bit",
            Self::Value => "value",
            Self::OperationResult => "operation_result",
            Self::Opaque => "opaque",
        };

        f.write_str(name)
    }
}

// =============================================================================
// External-call result
// =============================================================================

/// Declared destination/identity of an external-call result.
///
/// Result typing remains owned by the broader IR type system. This structure
/// records the stable result identity and optional semantic destination.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ExternalCallResultTarget {
    /// An SSA-like result value.
    Value(ValueId),

    /// A classical-bit destination.
    ClassicalBit(ClassicalBitId),

    /// A quantum resource affected/returned by the external operation.
    Qubit(QubitId),
}

impl ExternalCallResultTarget {
    /// Returns the target kind.
    #[must_use]
    pub const fn kind(self) -> ExternalResultKind {
        match self {
            Self::Value(_) => ExternalResultKind::Value,
            Self::ClassicalBit(_) => ExternalResultKind::ClassicalBit,
            Self::Qubit(_) => ExternalResultKind::Qubit,
        }
    }

    /// Returns a value identity when available.
    #[must_use]
    pub const fn as_value(self) -> Option<ValueId> {
        match self {
            Self::Value(value) => Some(value),
            Self::ClassicalBit(_) | Self::Qubit(_) => None,
        }
    }

    /// Returns a classical-bit destination when available.
    #[must_use]
    pub const fn as_classical_bit(self) -> Option<ClassicalBitId> {
        match self {
            Self::ClassicalBit(bit) => Some(bit),
            Self::Value(_) | Self::Qubit(_) => None,
        }
    }

    /// Returns a quantum resource when available.
    #[must_use]
    pub const fn as_qubit(self) -> Option<QubitId> {
        match self {
            Self::Qubit(qubit) => Some(qubit),
            Self::Value(_) | Self::ClassicalBit(_) => None,
        }
    }
}

/// Broad category of an external-call result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ExternalResultKind {
    /// IR value.
    Value,

    /// Classical bit.
    ClassicalBit,

    /// Quantum resource.
    Qubit,
}

impl fmt::Display for ExternalResultKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Value => "value",
            Self::ClassicalBit => "classical_bit",
            Self::Qubit => "qubit",
        };

        f.write_str(name)
    }
}

// =============================================================================
// Capability requirement
// =============================================================================

/// Capability requirement attached to an external call.
///
/// The identity is resolved against the canonical capability system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExternalCapabilityRequirement {
    capability: CapabilityId,
    required: bool,
}

impl ExternalCapabilityRequirement {
    /// Creates a required capability.
    #[must_use]
    pub const fn required(
        capability: CapabilityId,
    ) -> Self {
        Self {
            capability,
            required: true,
        }
    }

    /// Creates an optional capability.
    #[must_use]
    pub const fn optional(
        capability: CapabilityId,
    ) -> Self {
        Self {
            capability,
            required: false,
        }
    }

    /// Returns the capability identity.
    #[must_use]
    pub const fn capability(self) -> CapabilityId {
        self.capability
    }

    /// Returns whether the capability is mandatory.
    #[must_use]
    pub const fn is_required(self) -> bool {
        self.required
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by local external-call validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExternalCallError {
    /// External symbol was empty.
    EmptySymbol,

    /// External symbol contained a NUL byte.
    NulInSymbol,

    /// External namespace was empty.
    EmptyNamespace,

    /// External namespace contained a NUL byte.
    NulInNamespace,

    /// An external call was created without a valid symbol.
    MissingSymbol,

    /// A required operation identity was invalid.
    InvalidOperationId,

    /// An external call contains an excessive collection according to an
    /// explicitly supplied caller limit.
    CollectionLimitExceeded {
        /// Collection name.
        collection: &'static str,

        /// Actual number of entries.
        actual: usize,

        /// Explicit permitted maximum.
        limit: usize,
    },

    /// A result destination was duplicated.
    DuplicateResultTarget,

    /// A required capability was duplicated.
    DuplicateCapability,

    /// A call requires at least one quantum operand but none was supplied.
    MissingQuantumOperand,

    /// A call requires at least one classical operand but none was supplied.
    MissingClassicalOperand,

    /// A call's declared output count does not match the supplied results.
    ResultCountMismatch {
        /// Expected number.
        expected: usize,

        /// Actual number.
        actual: usize,
    },

    /// A semantic invariant is invalid.
    InvalidStructure(&'static str),
}

impl fmt::Display for ExternalCallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySymbol => {
                f.write_str("external symbol cannot be empty")
            }

            Self::NulInSymbol => {
                f.write_str("external symbol cannot contain NUL")
            }

            Self::EmptyNamespace => {
                f.write_str("external namespace cannot be empty")
            }

            Self::NulInNamespace => {
                f.write_str("external namespace cannot contain NUL")
            }

            Self::MissingSymbol => {
                f.write_str("external call requires a symbol")
            }

            Self::InvalidOperationId => {
                f.write_str("external call operation identity is invalid")
            }

            Self::CollectionLimitExceeded {
                collection,
                actual,
                limit,
            } => {
                write!(
                    f,
                    "{collection} contains {actual} entries, exceeding explicit limit {limit}"
                )
            }

            Self::DuplicateResultTarget => {
                f.write_str("external call contains a duplicate result target")
            }

            Self::DuplicateCapability => {
                f.write_str("external call contains a duplicate capability")
            }

            Self::MissingQuantumOperand => {
                f.write_str("external call requires at least one quantum operand")
            }

            Self::MissingClassicalOperand => {
                f.write_str("external call requires at least one classical operand")
            }

            Self::ResultCountMismatch { expected, actual } => {
                write!(
                    f,
                    "external call result count mismatch: expected {expected}, got {actual}"
                )
            }

            Self::InvalidStructure(message) => {
                write!(
                    f,
                    "invalid external-call structure: {message}"
                )
            }
        }
    }
}

impl std::error::Error for ExternalCallError {}

// =============================================================================
// External call
// =============================================================================

/// Canonical semantic representation of an external call.
///
/// `ExternalCall` is deliberately a declarative IR object. It does not execute
/// anything.
///
/// The model supports classical, quantum, and hybrid calls.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExternalCall {
    /// Stable identity of the enclosing IR operation.
    operation_id: OperationId,

    /// External symbol to resolve.
    symbol: ExternalSymbol,

    /// Optional namespace/library qualification.
    namespace: Option<ExternalNamespace>,

    /// Declarative linkage mechanism.
    linkage: ExternalLinkage,

    /// ABI metadata.
    abi: ExternalAbi,

    /// Ordered argument list.
    arguments: Vec<ExternalCallArgument>,

    /// Ordered result destinations.
    results: Vec<ExternalCallResultTarget>,

    /// Required/optional target capabilities.
    capabilities: Vec<ExternalCapabilityRequirement>,

    /// Execution/effect contract.
    execution: ExternalExecutionProperties,
}

impl ExternalCall {
    /// Creates a new external call with no arguments, results, or capability
    /// requirements.
    pub fn new(
        operation_id: OperationId,
        symbol: ExternalSymbol,
    ) -> ExternalCallResult<Self> {
        Ok(Self {
            operation_id,
            symbol,
            namespace: None,
            linkage: ExternalLinkage::Default,
            abi: ExternalAbi::default(),
            arguments: Vec::new(),
            results: Vec::new(),
            capabilities: Vec::new(),
            execution: ExternalExecutionProperties::default(),
        })
    }

    /// Creates a call directly from a symbol string.
    pub fn from_symbol<S>(
        operation_id: OperationId,
        symbol: S,
    ) -> ExternalCallResult<Self>
    where
        S: Into<String>,
    {
        Self::new(
            operation_id,
            ExternalSymbol::new(symbol)?,
        )
    }

    /// Returns the enclosing operation identity.
    #[must_use]
    pub const fn operation_id(&self) -> OperationId {
        self.operation_id
    }

    /// Returns the external symbol.
    #[must_use]
    pub fn symbol(&self) -> &ExternalSymbol {
        &self.symbol
    }

    /// Returns the optional namespace.
    #[must_use]
    pub fn namespace(&self) -> Option<&ExternalNamespace> {
        self.namespace.as_ref()
    }

    /// Returns the linkage contract.
    #[must_use]
    pub const fn linkage(&self) -> &ExternalLinkage {
        &self.linkage
    }

    /// Returns the ABI metadata.
    #[must_use]
    pub const fn abi(&self) -> &ExternalAbi {
        &self.abi
    }

    /// Returns the ordered arguments.
    #[must_use]
    pub fn arguments(&self) -> &[ExternalCallArgument] {
        &self.arguments
    }

    /// Returns the ordered results.
    #[must_use]
    pub fn results(&self) -> &[ExternalCallResultTarget] {
        &self.results
    }

    /// Returns the capability requirements.
    #[must_use]
    pub fn capabilities(&self) -> &[ExternalCapabilityRequirement] {
        &self.capabilities
    }

    /// Returns the execution contract.
    #[must_use]
    pub const fn execution(&self) -> ExternalExecutionProperties {
        self.execution
    }

    /// Sets the namespace.
    #[must_use]
    pub fn with_namespace(
        mut self,
        namespace: ExternalNamespace,
    ) -> Self {
        self.namespace = Some(namespace);
        self
    }

    /// Sets the linkage model.
    #[must_use]
    pub fn with_linkage(
        mut self,
        linkage: ExternalLinkage,
    ) -> Self {
        self.linkage = linkage;
        self
    }

    /// Sets ABI metadata.
    #[must_use]
    pub fn with_abi(
        mut self,
        abi: ExternalAbi,
    ) -> Self {
        self.abi = abi;
        self
    }

    /// Sets execution properties.
    #[must_use]
    pub fn with_execution(
        mut self,
        execution: ExternalExecutionProperties,
    ) -> Self {
        self.execution = execution;
        self
    }

    /// Adds one argument.
    pub fn push_argument(
        &mut self,
        argument: ExternalCallArgument,
    ) {
        self.arguments.push(argument);
    }

    /// Adds one result target.
    pub fn push_result(
        &mut self,
        result: ExternalCallResultTarget,
    ) -> ExternalCallResult<()> {
        if self.results.contains(&result) {
            return Err(ExternalCallError::DuplicateResultTarget);
        }

        self.results.push(result);

        Ok(())
    }

    /// Adds one capability requirement.
    pub fn push_capability(
        &mut self,
        capability: ExternalCapabilityRequirement,
    ) -> ExternalCallResult<()> {
        if self
            .capabilities
            .iter()
            .any(|existing| existing.capability() == capability.capability())
        {
            return Err(ExternalCallError::DuplicateCapability);
        }

        self.capabilities.push(capability);

        Ok(())
    }

    /// Returns the number of arguments.
    #[must_use]
    pub fn argument_count(&self) -> usize {
        self.arguments.len()
    }

    /// Returns the number of results.
    #[must_use]
    pub fn result_count(&self) -> usize {
        self.results.len()
    }

    /// Returns the number of capability requirements.
    #[must_use]
    pub fn capability_count(&self) -> usize {
        self.capabilities.len()
    }

    /// Returns whether the call has no arguments.
    #[must_use]
    pub fn has_no_arguments(&self) -> bool {
        self.arguments.is_empty()
    }

    /// Returns whether the call has no results.
    #[must_use]
    pub fn has_no_results(&self) -> bool {
        self.results.is_empty()
    }

    /// Returns whether the call contains at least one quantum operand.
    #[must_use]
    pub fn has_quantum_operands(&self) -> bool {
        self.arguments
            .iter()
            .any(|argument| matches!(argument, ExternalCallArgument::Qubit(_)))
    }

    /// Returns whether the call contains at least one classical operand.
    #[must_use]
    pub fn has_classical_operands(&self) -> bool {
        self.arguments.iter().any(|argument| {
            matches!(
                argument,
                ExternalCallArgument::ClassicalBit(_)
                    | ExternalCallArgument::Value(_)
                    | ExternalCallArgument::OperationResult(_)
                    | ExternalCallArgument::Opaque(_)
            )
        })
    }

    /// Returns an iterator over quantum operands.
    pub fn quantum_operands(
        &self,
    ) -> impl Iterator<Item = QubitId> + '_ {
        self.arguments.iter().filter_map(|argument| {
            argument.as_qubit()
        })
    }

    /// Returns an iterator over classical-bit operands.
    pub fn classical_bit_operands(
        &self,
    ) -> impl Iterator<Item = ClassicalBitId> + '_ {
        self.arguments.iter().filter_map(|argument| {
            argument.as_classical_bit()
        })
    }

    /// Returns whether this external call is semantically pure.
    #[must_use]
    pub const fn is_pure(&self) -> bool {
        self.execution.effect().is_pure()
    }

    /// Returns whether this external call may have side effects.
    #[must_use]
    pub const fn may_have_side_effects(&self) -> bool {
        self.execution.effect().may_have_side_effects()
    }

    /// Returns whether this call may block.
    #[must_use]
    pub const fn may_block(&self) -> bool {
        self.execution.may_block()
    }

    /// Returns whether memoization is semantically permitted.
    #[must_use]
    pub const fn is_memoizable(&self) -> bool {
        self.execution.is_memoizable()
    }

    /// Validates the local structural invariants of the call.
    ///
    /// This method intentionally does NOT perform program-wide validation.
    ///
    /// In particular, it does not check whether:
    ///
    /// - the `OperationId` exists in a program;
    /// - a `QubitId` is declared;
    /// - a `ValueId` exists;
    /// - a capability exists;
    /// - a symbol can actually be linked;
    /// - an ABI is supported.
    ///
    /// Those checks belong to `validation.rs`, linker, runtime, and backend
    /// layers.
    pub fn validate(&self) -> ExternalCallResult<()> {
        if self.symbol.as_str().is_empty() {
            return Err(ExternalCallError::MissingSymbol);
        }

        if self.operation_id.value() == 0 {
            return Err(ExternalCallError::InvalidOperationId);
        }

        for capability in &self.capabilities {
            if capability.capability().value() == 0 {
                return Err(ExternalCallError::InvalidStructure(
                    "capability identity cannot be zero",
                ));
            }
        }

        for result in &self.results {
            match result {
                ExternalCallResultTarget::Value(value) => {
                    if value.value() == 0 {
                        return Err(ExternalCallError::InvalidStructure(
                            "result ValueId cannot be zero",
                        ));
                    }
                }

                ExternalCallResultTarget::ClassicalBit(_) => {}

                ExternalCallResultTarget::Qubit(_) => {}
            }
        }

        Ok(())
    }

    /// Validates the call against explicit caller-provided collection limits.
    ///
    /// These limits are policies, NOT architectural limits.
    ///
    /// A compiler can therefore choose:
    ///
    /// ```text
    /// tiny compilation policy
    /// large compilation policy
    /// distributed compilation policy
    /// ```
    ///
    /// without changing the semantic IR.
    pub fn validate_with_limits(
        &self,
        max_arguments: Option<usize>,
        max_results: Option<usize>,
        max_capabilities: Option<usize>,
    ) -> ExternalCallResult<()> {
        self.validate()?;

        if let Some(limit) = max_arguments {
            if self.arguments.len() > limit {
                return Err(
                    ExternalCallError::CollectionLimitExceeded {
                        collection: "arguments",
                        actual: self.arguments.len(),
                        limit,
                    },
                );
            }
        }

        if let Some(limit) = max_results {
            if self.results.len() > limit {
                return Err(
                    ExternalCallError::CollectionLimitExceeded {
                        collection: "results",
                        actual: self.results.len(),
                        limit,
                    },
                );
            }
        }

        if let Some(limit) = max_capabilities {
            if self.capabilities.len() > limit {
                return Err(
                    ExternalCallError::CollectionLimitExceeded {
                        collection: "capabilities",
                        actual: self.capabilities.len(),
                        limit,
                    },
                );
            }
        }

        Ok(())
    }
}

// =============================================================================
// Constructors for common call classes
// =============================================================================

impl ExternalCall {
    /// Creates a pure classical external call.
    pub fn pure_classical<S>(
        operation_id: OperationId,
        symbol: S,
    ) -> ExternalCallResult<Self>
    where
        S: Into<String>,
    {
        Self::from_symbol(operation_id, symbol)
            .map(|call| {
                call.with_execution(
                    ExternalExecutionProperties::pure(),
                )
            })
    }

    /// Creates a conservative hybrid/quantum external call.
    pub fn hybrid<S>(
        operation_id: OperationId,
        symbol: S,
    ) -> ExternalCallResult<Self>
    where
        S: Into<String>,
    {
        Self::from_symbol(operation_id, symbol)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symbol_rejects_empty_name() {
        let result = ExternalSymbol::new("");

        assert!(matches!(
            result,
            Err(ExternalCallError::EmptySymbol)
        ));
    }

    #[test]
    fn symbol_rejects_nul() {
        let result = ExternalSymbol::new("foo\0bar");

        assert!(matches!(
            result,
            Err(ExternalCallError::NulInSymbol)
        ));
    }

    #[test]
    fn symbol_is_stable_and_displayable() {
        let symbol = ExternalSymbol::new("math.sin")
            .expect("valid symbol");

        assert_eq!(symbol.as_str(), "math.sin");
        assert_eq!(symbol.to_string(), "math.sin");
    }

    #[test]
    fn pure_call_is_memoizable() {
        let call = ExternalCall::pure_classical(
            OperationId::new(1),
            "math.sin",
        )
        .expect("valid call");

        assert!(call.is_pure());
        assert!(call.is_memoizable());
        assert!(!call.may_block());
        assert!(!call.may_have_side_effects());
    }

    #[test]
    fn conservative_call_is_not_pure() {
        let call = ExternalCall::hybrid(
            OperationId::new(1),
            "qpu.operation",
        )
        .expect("valid call");

        assert!(!call.is_pure());
        assert!(!call.is_memoizable());
        assert!(call.may_block());
        assert!(call.may_have_side_effects());
    }

    #[test]
    fn quantum_argument_uses_canonical_qubit_id() {
        let argument =
            ExternalCallArgument::Qubit(QubitId::new(7));

        assert_eq!(
            argument.as_qubit(),
            Some(QubitId::new(7))
        );

        assert_eq!(
            argument.kind(),
            ExternalArgumentKind::Qubit
        );
    }

    #[test]
    fn classical_argument_is_distinct_from_value() {
        let bit =
            ExternalCallArgument::ClassicalBit(
                ClassicalBitId::new(4),
            );

        let value =
            ExternalCallArgument::Value(
                ValueId::new(5),
            );

        assert_ne!(bit, value);
        assert_eq!(
            bit.kind(),
            ExternalArgumentKind::ClassicalBit
        );
        assert_eq!(
            value.kind(),
            ExternalArgumentKind::Value
        );
    }

    #[test]
    fn duplicate_result_is_rejected() {
        let mut call = ExternalCall::pure_classical(
            OperationId::new(1),
            "math.identity",
        )
        .expect("valid call");

        let target =
            ExternalCallResultTarget::Value(
                ValueId::new(2),
            );

        call.push_result(target)
            .expect("first result is valid");

        let duplicate = call.push_result(target);

        assert!(matches!(
            duplicate,
            Err(ExternalCallError::DuplicateResultTarget)
        ));
    }

    #[test]
    fn duplicate_capability_is_rejected() {
        let mut call = ExternalCall::hybrid(
            OperationId::new(1),
            "external.quantum",
        )
        .expect("valid call");

        let capability =
            ExternalCapabilityRequirement::required(
                CapabilityId::new(2),
            );

        call.push_capability(capability)
            .expect("first capability is valid");

        let duplicate =
            call.push_capability(capability);

        assert!(matches!(
            duplicate,
            Err(ExternalCallError::DuplicateCapability)
        ));
    }

    #[test]
    fn quantum_operand_iterator_is_deterministic() {
        let mut call = ExternalCall::hybrid(
            OperationId::new(1),
            "external.quantum",
        )
        .expect("valid call");

        call.push_argument(
            ExternalCallArgument::Qubit(
                QubitId::new(10),
            ),
        );

        call.push_argument(
            ExternalCallArgument::Value(
                ValueId::new(20),
            ),
        );

        call.push_argument(
            ExternalCallArgument::Qubit(
                QubitId::new(11),
            ),
        );

        let qubits: Vec<QubitId> =
            call.quantum_operands().collect();

        assert_eq!(
            qubits,
            vec![
                QubitId::new(10),
                QubitId::new(11),
            ]
        );
    }

    #[test]
    fn arbitrary_argument_count_is_supported() {
        let mut call = ExternalCall::hybrid(
            OperationId::new(1),
            "external.operation",
        )
        .expect("valid call");

        for index in 1..=10_000usize {
            call.push_argument(
                ExternalCallArgument::Value(
                    ValueId::new(index as u64),
                ),
            );
        }

        assert_eq!(
            call.argument_count(),
            10_000
        );
    }

    #[test]
    fn explicit_limits_are_policy_not_semantic_limits() {
        let mut call = ExternalCall::hybrid(
            OperationId::new(1),
            "external.operation",
        )
        .expect("valid call");

        for index in 1..=10usize {
            call.push_argument(
                ExternalCallArgument::Value(
                    ValueId::new(index),
                ),
            );
        }

        assert!(call
            .validate_with_limits(
                Some(10),
                None,
                None,
            )
            .is_ok());

        assert!(matches!(
            call.validate_with_limits(
                Some(9),
                None,
                None,
            ),
            Err(
                ExternalCallError::CollectionLimitExceeded {
                    collection: "arguments",
                    actual: 10,
                    limit: 9,
                }
            )
        ));
    }

    #[test]
    fn result_targets_support_quantum_resources() {
        let target =
            ExternalCallResultTarget::Qubit(
                QubitId::new(42),
            );

        assert_eq!(
            target.as_qubit(),
            Some(QubitId::new(42))
        );

        assert_eq!(
            target.kind(),
            ExternalResultKind::Qubit
        );
    }

    #[test]
    fn abi_is_declarative() {
        let convention =
            ExternalSymbol::new("zamani-native")
                .expect("valid symbol");

        let encoding =
            ExternalSymbol::new("canonical")
                .expect("valid symbol");

        let abi = ExternalAbi::new()
            .with_calling_convention(convention)
            .with_encoding(encoding)
            .with_version("1")
            .target_native();

        assert_eq!(
            abi.calling_convention()
                .map(ExternalSymbol::as_str),
            Some("zamani-native")
        );

        assert_eq!(
            abi.encoding()
                .map(ExternalSymbol::as_str),
            Some("canonical")
        );

        assert_eq!(
            abi.version(),
            Some("1")
        );

        assert!(abi.is_target_native());
    }
}
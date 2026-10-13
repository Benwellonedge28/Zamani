
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani Quantum-Domain Semantic Validation
//!
//! Path: `src/frontend/semantic/domains/quantum.rs`
//! Edition: Rust 2021
//! Minimum Rust version: Rust 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Validate quantum-domain semantic facts produced from the canonical Zamani
//! AST and shared semantic-analysis services.
//!
//! This module owns quantum-specific validation rules and issue reporting.
//! It does not parse source code, resolve names independently, define a second
//! AST, implement a type system, or lower source constructs to quantum IR.
//!
//! ## AUTHORITY
//!
//! - `grammar/Zamani.g4`
//! - `grammar/quantum/operations.g4`
//! - `grammar/quantum/parameters.g4`
//! - `grammar/quantum/measurement.g4`
//! - `grammar/quantum/controls.g4`
//! - `grammar/quantum/quantum-resources.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/domains.md`
//! - `grammar/specification/extensibility.md`
//! - `src/frontend/ast/node/`
//! - `src/frontend/semantic/domains/classical.rs`
//! - `src/frontend/semantic/domains/registry.rs`
//!
//! The normative grammar and specifications define language meaning.
//! The canonical AST defines source structure.
//! Shared semantic modules own ordinary name resolution, typing, effects,
//! capabilities, resources, contracts, and diagnostics.
//!
//! ## OWNS
//!
//! - Quantum operation semantic-contract validation.
//! - Quantum target and control relationship checks.
//! - Operation modifier compatibility checks.
//! - Quantum parameter and result contract checks.
//! - Measurement and quantum/classical boundary checks.
//! - Quantum-specific semantic issue classification.
//! - Explicit validation-completeness reporting.
//!
//! ## DOES_NOT_OWN
//!
//! - Lexing, parsing, grammar interpretation, or AST construction.
//! - A duplicate symbol table or ordinary name resolver.
//! - A competing type system or generic solver.
//! - Global effect, capability, resource, or contract registries.
//! - Physical qubit allocation, topology, routing, or scheduling.
//! - Calibration, pulse generation, noise simulation, or error correction.
//! - Classical IR, `quantum::ir`, ZQN, HAL, or runtime execution.
//! - Provider-specific gate catalogs or fixed operation enumerations.
//!
//! ## SEMANTIC BOUNDARY
//!
//! ```text
//! Canonical AST
//!     |
//!     v
//! Shared semantic analysis
//!     |
//!     +--> declaration and operation resolution
//!     +--> type and generic inference
//!     +--> effect analysis
//!     +--> capability/resource obligations
//!     |
//!     v
//! Quantum semantic facts
//!     |
//!     v
//! This validator
//!     |
//!     v
//! Shared semantic diagnostics and finalized facts
//!     |
//!     v
//! Canonical quantum IR boundary
//!     |
//!     v
//! Target feasibility, lowering, routing, scheduling, execution
//! ```
//!
//! The semantic-fact adapter is responsible for translating actual AST
//! constructs into the input records below. This module never guesses that
//! an unrecognized AST construct is valid.
//!
//! ## OPEN-WORLD OPERATION CONTRACT
//!
//! Operation identity is a namespace-qualified string, not a closed enum.
//! The validator does not enumerate Hadamard, Pauli, controlled, rotation,
//! vendor-defined, user-defined, or future operations.
//!
//! Operation definitions supply their parameter, target, result, modifier,
//! effect, and capability contracts. New operations are introduced through
//! the repository's extension and declaration mechanisms.
//!
//! ## POCO-REAF AND SCALABILITY
//!
//! This module defines no language-level maximum for:
//!
//! - logical quantum values;
//! - operation count;
//! - operation parameters;
//! - target count;
//! - control count;
//! - result count;
//! - operation definitions;
//! - computational domains;
//! - resource quantities;
//! - hardware capacities.
//!
//! The validator uses dynamically sized collections and checked arithmetic.
//! Operational budgets are enforced by the caller's semantic session.
//! Exhausted budgets must yield an incomplete result, not a valid certificate.
//!
//! No finite implementation can process literally infinite input. The goal
//! is to avoid arbitrary language or hardware ceilings and to scale according
//! to representable values, configured budgets, and available resources.
//!
//! ## DETERMINISM
//!
//! Operation definitions, identities, and sets use ordered collections where
//! this module owns their ordering. Invocation order is preserved from input.
//! Diagnostics are emitted in deterministic invocation and rule order.
//!
//! The context must itself provide stable semantic facts and immutable,
//! revisioned extension metadata for the duration of one analysis.
//!
//! ## INTEGRATION CONTRACT
//!
//! - `domains/mod.rs` declares and exports `quantum`.
//! - `pipeline.rs` invokes this validator after declaration/name/type/effect
//!   facts required by the selected quantum rules are available.
//! - `symbols/` resolves operation identities before validation.
//! - `types/` supplies the canonical compatibility result for parameter,
//!   target, and result types.
//! - `effects/` owns effect inference and authorization.
//! - `capabilities/` and `resources/` own their canonical obligations.
//! - `domains/registry.rs` owns domain and extension identity/metadata.
//! - The AST adapter maps source constructs to `QuantumInvocation` records.
//! - The canonical diagnostic layer maps `QuantumIssue` into repository-wide
//!   diagnostics, retaining `NodeId` and related node identities.
//! - The IR generator consumes finalized semantic facts, never these temporary
//!   validation records as a replacement for canonical quantum IR.
//!
//! ## FREEZE CRITERIA
//!
//! 1. The AST adapter supplies complete facts for every supported quantum
//!    invocation construct.
//! 2. The shared type, effect, capability, and resource services are connected.
//! 3. Extension definitions are validated and revision-stable.
//! 4. Unknown or unresolved facts cannot be certified as valid.
//! 5. The pipeline propagates incomplete analysis correctly.
//! 6. Quantum conformance, determinism, fuzz, and stress tests pass.
//! 7. The full workspace builds, formats, and passes its required lints/tests.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;

use crate::frontend::ast::node::node_id::NodeId;

/// Version of this module's semantic validation contract.
///
/// This is independent of the Zamani language version, AST schema version,
/// extension registry revision, and quantum IR version.
pub const QUANTUM_VALIDATION_SCHEMA_VERSION: u16 = 1;

/// Result of a quantum-domain validation operation.
pub type QuantumResult<T> = Result<T, QuantumAnalysisFailure>;

/// Stable category of a quantum semantic issue.
///
/// These categories are local to quantum-domain validation. The shared
/// diagnostics registry remains authoritative for final diagnostic codes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum QuantumIssueKind {
    /// An operation name could not be resolved to a semantic definition.
    UnresolvedOperation,

    /// An operation definition is malformed or internally inconsistent.
    InvalidOperationContract,

    /// An operation argument violates its parameter contract.
    ParameterSemantics,

    /// The number or types of operation targets are invalid.
    TargetSemantics,

    /// A control relationship violates the operation's contract.
    ControlSemantics,

    /// An operation modifier is not supported by the resolved definition.
    ModifierSemantics,

    /// An operation result violates the resolved result contract.
    ResultSemantics,

    /// Measurement or observation violates its semantic contract.
    MeasurementSemantics,

    /// A quantum/classical boundary violates its declared contract.
    BoundarySemantics,

    /// An effect required by an operation is not established or authorized.
    EffectSemantics,

    /// A required capability obligation is not established.
    CapabilitySemantics,

    /// A symbolic resource or requirement contract is invalid.
    ResourceSemantics,

    /// A source construct has invalid or inconsistent AST relationships.
    StructuralSemantics,

    /// A domain extension has no applicable semantic handler.
    UnsupportedExtension,

    /// Analysis did not establish all facts required for validation.
    IncompleteAnalysis,
}

impl fmt::Display for QuantumIssueKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Self::UnresolvedOperation => "unresolved quantum operation",
            Self::InvalidOperationContract => "invalid quantum operation contract",
            Self::ParameterSemantics => "quantum parameter semantics",
            Self::TargetSemantics => "quantum target semantics",
            Self::ControlSemantics => "quantum control semantics",
            Self::ModifierSemantics => "quantum modifier semantics",
            Self::ResultSemantics => "quantum result semantics",
            Self::MeasurementSemantics => "quantum measurement semantics",
            Self::BoundarySemantics => "quantum/classical boundary semantics",
            Self::EffectSemantics => "quantum effect semantics",
            Self::CapabilitySemantics => "quantum capability semantics",
            Self::ResourceSemantics => "quantum resource semantics",
            Self::StructuralSemantics => "quantum structural semantics",
            Self::UnsupportedExtension => "unsupported quantum extension",
            Self::IncompleteAnalysis => "incomplete quantum analysis",
        };

        f.write_str(label)
    }
}

/// One quantum semantic issue.
///
/// `rule` is a stable, module-local identifier. Consumers must classify issues
/// using `kind` and `rule`, not by parsing the human-readable message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QuantumIssue {
    /// Category of the failed semantic rule.
    pub kind: QuantumIssueKind,

    /// AST node primarily associated with the issue.
    pub node: NodeId,

    /// Optional related AST node, such as a control or parameter expression.
    pub related_node: Option<NodeId>,

    /// Stable identifier of the failed rule.
    pub rule: &'static str,

    /// Human-readable explanation.
    pub message: String,
}

impl QuantumIssue {
    /// Construct an issue associated with one AST node.
    #[must_use]
    pub fn new(
        kind: QuantumIssueKind,
        node: NodeId,
        rule: &'static str,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            node,
            related_node: None,
            rule,
            message: message.into(),
        }
    }

    /// Attach a related AST node.
    #[must_use]
    pub fn with_related_node(mut self, related_node: NodeId) -> Self {
        self.related_node = Some(related_node);
        self
    }
}

/// Why a quantum analysis could not establish a complete result.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum QuantumAnalysisFailure {
    /// A required semantic fact was unavailable.
    MissingFact {
        /// AST node for which the fact was needed.
        node: NodeId,
        /// Stable identifier of the missing fact category.
        fact: &'static str,
    },

    /// The caller's operational policy stopped analysis.
    BudgetExhausted {
        /// Stable identifier of the exhausted budget.
        budget: &'static str,
    },

    /// An extension or context implementation failed to provide a valid
    /// contract.
    InvalidContext {
        /// Human-readable context failure.
        message: String,
    },
}

impl fmt::Display for QuantumAnalysisFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingFact { node, fact } => {
                write!(f, "missing quantum semantic fact `{fact}` for {node:?}")
            }
            Self::BudgetExhausted { budget } => {
                write!(f, "quantum semantic analysis exhausted `{budget}`")
            }
            Self::InvalidContext { message } => {
                write!(f, "invalid quantum semantic context: {message}")
            }
        }
    }
}

impl Error for QuantumAnalysisFailure {}

/// Stable, namespace-qualified identity for an operation.
///
/// The identifier is open-world. It must not be interpreted as a hardware
/// instruction, physical gate, vendor operation, or fixed gate-set member.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OperationIdentity {
    namespace: String,
    name: String,
}

impl OperationIdentity {
    /// Create an operation identity from a namespace and name.
    ///
    /// Both components must be non-empty and contain no leading or trailing
    /// whitespace. More specific lexical and namespace rules are enforced by
    /// the canonical name resolver and extension registry.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Self, OperationIdentityError> {
        let namespace = namespace.into();
        let name = name.into();

        if namespace.trim().is_empty() {
            return Err(OperationIdentityError::EmptyNamespace);
        }
        if name.trim().is_empty() {
            return Err(OperationIdentityError::EmptyName);
        }
        if namespace.trim() != namespace || name.trim() != name {
            return Err(OperationIdentityError::SurroundingWhitespace);
        }

        Ok(Self { namespace, name })
    }

    /// Return the namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Return the operation name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the qualified textual identity.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        format!("{}::{}", self.namespace, self.name)
    }
}

impl fmt::Display for OperationIdentity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}::{}", self.namespace, self.name)
    }
}

/// Invalid operation-identity construction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OperationIdentityError {
    /// Namespace was empty.
    EmptyNamespace,
    /// Operation name was empty.
    EmptyName,
    /// Namespace or name contained leading/trailing whitespace.
    SurroundingWhitespace,
}

impl fmt::Display for OperationIdentityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyNamespace => f.write_str("operation namespace is empty"),
            Self::EmptyName => f.write_str("operation name is empty"),
            Self::SurroundingWhitespace => {
                f.write_str("operation identity contains surrounding whitespace")
            }
        }
    }
}

impl Error for OperationIdentityError {}

/// A semantic type identity supplied by the shared type subsystem.
///
/// This is deliberately an opaque, namespaced identity rather than a second
/// semantic type representation. The type checker owns compatibility rules.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SemanticTypeIdentity {
    namespace: String,
    name: String,
}

impl SemanticTypeIdentity {
    /// Construct a type identity.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Self, OperationIdentityError> {
        let namespace = namespace.into();
        let name = name.into();

        if namespace.trim().is_empty() {
            return Err(OperationIdentityError::EmptyNamespace);
        }
        if name.trim().is_empty() {
            return Err(OperationIdentityError::EmptyName);
        }
        if namespace.trim() != namespace || name.trim() != name {
            return Err(OperationIdentityError::SurroundingWhitespace);
        }

        Ok(Self { namespace, name })
    }

    /// Return the namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Return the type name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// One declared operation parameter contract.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParameterContract {
    /// Parameter's stable semantic name, when named.
    pub name: Option<String>,

    /// Expected semantic type identity.
    pub expected_type: SemanticTypeIdentity,

    /// Whether this parameter is optional under the language's call rules.
    pub optional: bool,
}

/// Cardinality constraint for an operation's target list.
///
/// `None` is an unbounded upper cardinality, not an infinite allocation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cardinality {
    /// Minimum accepted number of elements.
    pub minimum: u64,

    /// Maximum accepted number of elements; `None` means no declared maximum.
    pub maximum: Option<u64>,
}

impl Cardinality {
    /// Construct a validated cardinality.
    pub fn new(
        minimum: u64,
        maximum: Option<u64>,
    ) -> Result<Self, CardinalityError> {
        if maximum.is_some_and(|upper| upper < minimum) {
            return Err(CardinalityError::MaximumBelowMinimum);
        }

        Ok(Self { minimum, maximum })
    }

    /// Construct a cardinality that accepts any count, including zero.
    #[must_use]
    pub const fn any() -> Self {
        Self {
            minimum: 0,
            maximum: None,
        }
    }

    /// Construct a cardinality that requires at least one element.
    #[must_use]
    pub const fn one_or_more() -> Self {
        Self {
            minimum: 1,
            maximum: None,
        }
    }

    /// Return whether a count satisfies this contract.
    #[must_use]
    pub fn accepts(self, count: usize) -> bool {
        let Ok(count) = u64::try_from(count) else {
            return false;
        };

        count >= self.minimum
            && self.maximum.is_none_or(|maximum| count <= maximum)
    }
}

/// Invalid cardinality definition.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CardinalityError {
    /// A finite maximum was less than the minimum.
    MaximumBelowMinimum,
}

impl fmt::Display for CardinalityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MaximumBelowMinimum => {
                f.write_str("cardinality maximum is below its minimum")
            }
        }
    }
}

impl Error for CardinalityError {}

/// Semantic contract for a declared quantum operation.
///
/// The contract contains source-level rules only. It does not describe a
/// physical implementation or require a fixed hardware gate set.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OperationContract {
    /// Canonical operation identity.
    pub identity: OperationIdentity,

    /// Required positional parameter contracts in declaration order.
    pub parameters: Vec<ParameterContract>,

    /// Number of accepted operation targets.
    pub targets: Cardinality,

    /// Number of accepted controls.
    pub controls: Cardinality,

    /// Allowed modifier identities, using open-world names.
    pub allowed_modifiers: BTreeSet<String>,

    /// Expected result types in declaration order.
    pub results: Vec<SemanticTypeIdentity>,

    /// Whether the operation is declared to perform quantum measurement.
    pub performs_measurement: bool,

    /// Whether the operation permits classical control/feed-forward.
    pub permits_classical_control: bool,

    /// Whether repeated target identities are allowed by this operation.
    ///
    /// A true value means only that the operation contract permits repeated
    /// target references. It does not override aliasing, ownership, or
    /// operation-specific rules owned by other semantic passes.
    pub permits_repeated_targets: bool,

    /// Required capability identities declared by this operation.
    ///
    /// The capability subsystem determines whether the obligations are
    /// satisfied; this module does not inspect hardware.
    pub required_capabilities: BTreeSet<String>,

    /// Effects declared by this operation.
    ///
    /// The effect subsystem owns effect identity, propagation and
    /// authorization. These names are carried as operation-contract metadata.
    pub declared_effects: BTreeSet<String>,
}

impl OperationContract {
    /// Validate the internal consistency of this operation contract.
    ///
    /// This method checks metadata shape only. It does not prove that the
    /// implementation exists or that a target can execute it.
    pub fn validate(&self) -> Result<(), OperationContractError> {
        for modifier in &self.allowed_modifiers {
            validate_extension_identifier(modifier).map_err(|_| {
                OperationContractError::InvalidModifierIdentity(modifier.clone())
            })?;
        }

        for capability in &self.required_capabilities {
            validate_extension_identifier(capability).map_err(|_| {
                OperationContractError::InvalidCapabilityIdentity(capability.clone())
            })?;
        }

        for effect in &self.declared_effects {
            validate_extension_identifier(effect).map_err(|_| {
                OperationContractError::InvalidEffectIdentity(effect.clone())
            })?;
        }

        if self.parameters.iter().any(|parameter| {
            parameter.name.as_ref().is_some_and(|name| name.trim().is_empty())
        }) {
            return Err(OperationContractError::EmptyParameterName);
        }

        Ok(())
    }
}

/// Invalid operation-contract metadata.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum OperationContractError {
    /// A modifier identifier is malformed.
    InvalidModifierIdentity(String),
    /// A capability identifier is malformed.
    InvalidCapabilityIdentity(String),
    /// An effect identifier is malformed.
    InvalidEffectIdentity(String),
    /// A named parameter has an empty name.
    EmptyParameterName,
}

impl fmt::Display for OperationContractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidModifierIdentity(value) => {
                write!(f, "invalid modifier identity `{value}`")
            }
            Self::InvalidCapabilityIdentity(value) => {
                write!(f, "invalid capability identity `{value}`")
            }
            Self::InvalidEffectIdentity(value) => {
                write!(f, "invalid effect identity `{value}`")
            }
            Self::EmptyParameterName => f.write_str("operation parameter name is empty"),
        }
    }
}

impl Error for OperationContractError {}

/// Validate a namespaced extension identifier.
///
/// The canonical parser and registry must still enforce their own normative
/// namespace grammar. This function rejects only malformed empty components
/// and surrounding whitespace; it does not impose a closed identifier list.
fn validate_extension_identifier(value: &str) -> Result<(), &'static str> {
    if value.trim().is_empty() {
        return Err("empty");
    }
    if value.trim() != value {
        return Err("surrounding whitespace");
    }
    if value.split("::").any(str::is_empty) {
        return Err("empty namespace component");
    }
    Ok(())
}

/// A resolved argument expression supplied by the shared semantic pipeline.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResolvedArgument {
    /// AST identity of the argument expression.
    pub node: NodeId,

    /// Resolved semantic type of the argument.
    ///
    /// A missing type is not represented here: unresolved facts must be
    /// reported as incomplete analysis by the adapter/context.
    pub ty: SemanticTypeIdentity,

    /// Optional source-level name for a named argument.
    pub name: Option<String>,
}

/// A resolved operation target.
///
/// `identity` is the semantic identity of a logical/source-level target.
/// It is not a physical qubit index or runtime handle.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResolvedTarget {
    /// AST identity of the target expression.
    pub node: NodeId,

    /// Stable semantic identity of the target within this analysis.
    pub identity: String,

    /// Resolved semantic type of the target.
    pub ty: SemanticTypeIdentity,
}

/// A resolved operation control.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResolvedControl {
    /// AST identity of the control expression.
    pub node: NodeId,

    /// Stable semantic identity of the control within this analysis.
    pub identity: String,

    /// Resolved semantic type of the control.
    pub ty: SemanticTypeIdentity,
}

/// A result binding associated with an operation invocation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResultBinding {
    /// AST identity of the result binding.
    pub node: NodeId,

    /// Resolved semantic type of the destination.
    pub ty: SemanticTypeIdentity,
}

/// A modifier applied to an operation invocation.
///
/// Modifier identity is open-world. Its legality is determined by the
/// resolved operation contract and the applicable language specification.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OperationModifier {
    /// AST identity of the modifier.
    pub node: NodeId,

    /// Namespace-qualified or language-defined modifier identity.
    pub identity: String,
}

/// Classical control facts associated with an operation invocation.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct ClassicalControlFacts {
    /// Whether this invocation is conditionally controlled by classical data.
    pub conditional: bool,

    /// Whether the invocation consumes a measurement result or another
    /// classical value through feed-forward.
    pub uses_feed_forward: bool,

    /// AST node associated with the controlling expression, when present.
    pub control_node: Option<NodeId>,
}

/// One quantum operation invocation after shared semantic resolution.
///
/// The AST adapter creates these records from the canonical AST. This record
/// is not a replacement AST node and must not be persisted as the canonical
/// source representation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QuantumInvocation {
    /// AST identity of the complete invocation.
    pub node: NodeId,

    /// Resolved operation identity.
    pub operation: OperationIdentity,

    /// Positional or named arguments in source evaluation order.
    pub arguments: Vec<ResolvedArgument>,

    /// Logical/source-level operation targets.
    pub targets: Vec<ResolvedTarget>,

    /// Logical/source-level controls.
    pub controls: Vec<ResolvedControl>,

    /// Modifiers applied to the invocation.
    pub modifiers: Vec<OperationModifier>,

    /// Result bindings produced by the invocation.
    pub results: Vec<ResultBinding>,

    /// Classical control/feed-forward facts.
    pub classical_control: ClassicalControlFacts,

    /// Whether the shared semantic layer established the invocation's
    /// ordinary type and name-resolution facts.
    ///
    /// False means validation cannot certify this invocation.
    pub resolution_complete: bool,
}

/// Outcome of validating a single invocation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InvocationValidation {
    /// AST node of the invocation.
    pub node: NodeId,

    /// Resolved operation identity.
    pub operation: OperationIdentity,

    /// Issues discovered for this invocation.
    pub issues: Vec<QuantumIssue>,
}

impl InvocationValidation {
    /// Return whether this invocation has no reported issues.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.issues.is_empty()
    }
}

/// Overall completion state of quantum-domain validation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QuantumCompletion {
    /// All requested rules ran and no issue was reported.
    Complete,

    /// All requested rules ran, but at least one issue was reported.
    CompleteWithIssues,

    /// Required facts were unavailable or analysis was interrupted.
    Incomplete,
}

/// Complete result of validating a collection of quantum invocations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QuantumValidationReport {
    /// Per-invocation results, in input order.
    pub invocations: Vec<InvocationValidation>,

    /// Flattened issues, in deterministic invocation/rule order.
    pub issues: Vec<QuantumIssue>,

    /// Whether all required validation work completed.
    pub completion: QuantumCompletion,
}

impl QuantumValidationReport {
    /// Return whether validation completed and no issue was reported.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.completion == QuantumCompletion::Complete && self.issues.is_empty()
    }

    /// Return whether validation completed, even if semantic issues exist.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.completion != QuantumCompletion::Incomplete
    }
}

/// Shared semantic services required by quantum validation.
///
/// The implementation is supplied by the semantic pipeline. It delegates
/// compatibility to the canonical type system and effect/capability checks
/// to their authoritative owners.
///
/// Implementations must provide a stable view for the duration of one pass.
pub trait QuantumSemanticContext {
    /// Determine whether two resolved semantic types are compatible under
    /// the language's rules for the specified relationship.
    fn types_compatible(
        &self,
        expected: &SemanticTypeIdentity,
        actual: &SemanticTypeIdentity,
    ) -> bool;

    /// Return whether a required capability obligation is established.
    ///
    /// This must consult semantic capability evidence, not inspect physical
    /// hardware or silently assume a capability exists.
    fn capability_established(&self, capability: &str, at: NodeId) -> bool;

    /// Return whether an effect is permitted at this invocation.
    ///
    /// Effect identity, propagation and authorization remain owned by the
    /// shared effects subsystem.
    fn effect_permitted(&self, effect: &str, at: NodeId) -> bool;

    /// Return whether the shared semantic analysis has established that a
    /// value may cross the quantum/classical boundary in the given direction.
    fn boundary_permitted(
        &self,
        at: NodeId,
        direction: QuantumBoundaryDirection,
    ) -> bool;
}

/// Direction of a semantic quantum/classical boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum QuantumBoundaryDirection {
    /// Quantum computation produces or exposes a classical observation.
    QuantumToClassical,

    /// Classical computation supplies a value to quantum control/feed-forward.
    ClassicalToQuantum,
}

/// Configuration for the quantum semantic validation pass.
///
/// This configuration contains semantic policy switches, not machine
/// capacities. Work budgets are owned by the semantic session.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QuantumValidationOptions {
    /// Whether each operation's declared effects must be checked.
    pub check_effects: bool,

    /// Whether operation capability obligations must be checked.
    pub check_capabilities: bool,

    /// Whether quantum/classical boundary rules must be checked.
    pub check_boundaries: bool,

    /// Whether repeated target identities are checked against the operation
    /// contract.
    pub check_target_aliasing: bool,
}

impl Default for QuantumValidationOptions {
    fn default() -> Self {
        Self {
            check_effects: true,
            check_capabilities: true,
            check_boundaries: true,
            check_target_aliasing: true,
        }
    }
}

/// Quantum semantic validator.
///
/// Definitions are immutable during validation. Build a new validator when
/// the extension registry publishes a new snapshot; do not mutate a registry
/// that is already being used by an analysis session.
#[derive(Clone, Debug, Default)]
pub struct QuantumValidator {
    contracts: BTreeMap<OperationIdentity, OperationContract>,
    options: QuantumValidationOptions,
}

impl QuantumValidator {
    /// Construct a validator with the supplied contracts and default options.
    ///
    /// Duplicate operation identities are rejected instead of silently
    /// replacing one definition with another.
    pub fn new(
        contracts: impl IntoIterator<Item = OperationContract>,
    ) -> QuantumResult<Self> {
        Self::with_options(contracts, QuantumValidationOptions::default())
    }

    /// Construct a validator with explicit options.
    pub fn with_options(
        contracts: impl IntoIterator<Item = OperationContract>,
        options: QuantumValidationOptions,
    ) -> QuantumResult<Self> {
        let mut indexed = BTreeMap::new();

        for contract in contracts {
            contract.validate().map_err(|error| {
                QuantumAnalysisFailure::InvalidContext {
                    message: format!(
                        "invalid contract for `{}`: {error}",
                        contract.identity
                    ),
                }
            })?;

            let identity = contract.identity.clone();
            if indexed.insert(identity.clone(), contract).is_some() {
                return Err(QuantumAnalysisFailure::InvalidContext {
                    message: format!("duplicate operation contract `{identity}`"),
                });
            }
        }

        Ok(Self {
            contracts: indexed,
            options,
        })
    }

    /// Return the validation options.
    #[must_use]
    pub fn options(&self) -> &QuantumValidationOptions {
        &self.options
    }

    /// Return the operation contract for an exact resolved identity.
    #[must_use]
    pub fn contract(&self, identity: &OperationIdentity) -> Option<&OperationContract> {
        self.contracts.get(identity)
    }

    /// Validate a sequence of resolved operation invocations.
    ///
    /// Invocation order is preserved. Missing contracts or unresolved facts
    /// produce explicit issues and prevent the report from being considered
    /// valid. A complete report may still contain semantic errors.
    pub fn validate<C: QuantumSemanticContext>(
        &self,
        invocations: &[QuantumInvocation],
        context: &C,
    ) -> QuantumValidationReport {
        let mut reports = Vec::with_capacity(invocations.len());
        let mut issues = Vec::new();
        let mut incomplete = false;

        for invocation in invocations {
            let result = self.validate_invocation(invocation, context);
            if result
                .issues
                .iter()
                .any(|issue| issue.kind == QuantumIssueKind::IncompleteAnalysis)
            {
                incomplete = true;
            }
            issues.extend(result.issues.iter().cloned());
            reports.push(result);
        }

        let completion = if incomplete {
            QuantumCompletion::Incomplete
        } else if issues.is_empty() {
            QuantumCompletion::Complete
        } else {
            QuantumCompletion::CompleteWithIssues
        };

        QuantumValidationReport {
            invocations: reports,
            issues,
            completion,
        }
    }

    /// Validate one resolved invocation against its operation contract.
    pub fn validate_invocation<C: QuantumSemanticContext>(
        &self,
        invocation: &QuantumInvocation,
        context: &C,
    ) -> InvocationValidation {
        let mut issues = Vec::new();

        if !invocation.resolution_complete {
            issues.push(QuantumIssue::new(
                QuantumIssueKind::IncompleteAnalysis,
                invocation.node,
                "QSEM0001",
                "quantum invocation has unresolved semantic facts",
            ));
            return InvocationValidation {
                node: invocation.node,
                operation: invocation.operation.clone(),
                issues,
            };
        }

        let Some(contract) = self.contracts.get(&invocation.operation) else {
            issues.push(QuantumIssue::new(
                QuantumIssueKind::UnresolvedOperation,
                invocation.node,
                "QSEM0002",
                format!(
                    "no semantic definition is registered for operation `{}`",
                    invocation.operation
                ),
            ));

            return InvocationValidation {
                node: invocation.node,
                operation: invocation.operation.clone(),
                issues,
            };
        };

        self.validate_arguments(invocation, contract, context, &mut issues);
        self.validate_targets(invocation, contract, context, &mut issues);
        self.validate_controls(invocation, contract, context, &mut issues);
        self.validate_modifiers(invocation, contract, &mut issues);
        self.validate_results(invocation, contract, context, &mut issues);
        self.validate_measurement_and_boundaries(
            invocation,
            contract,
            context,
            &mut issues,
        );
        self.validate_effects_and_capabilities(
            invocation,
            contract,
            context,
            &mut issues,
        );

        InvocationValidation {
            node: invocation.node,
            operation: invocation.operation.clone(),
            issues,
        }
    }

    fn validate_arguments<C: QuantumSemanticContext>(
        &self,
        invocation: &QuantumInvocation,
        contract: &OperationContract,
        context: &C,
        issues: &mut Vec<QuantumIssue>,
    ) {
        let required = contract
            .parameters
            .iter()
            .filter(|parameter| !parameter.optional)
            .count();

        if invocation.arguments.len() < required
            || invocation.arguments.len() > contract.parameters.len()
        {
            issues.push(QuantumIssue::new(
                QuantumIssueKind::ParameterSemantics,
                invocation.node,
                "QSEM0101",
                format!(
                    "operation `{}` expects between {} and {} arguments, but received {}",
                    contract.identity,
                    required,
                    contract.parameters.len(),
                    invocation.arguments.len()
                ),
            ));
        }

        for (index, argument) in invocation.arguments.iter().enumerate() {
            let Some(parameter) = contract.parameters.get(index) else {
                // The count mismatch above already records the excess input.
                continue;
            };

            if !context.types_compatible(&parameter.expected_type, &argument.ty) {
                issues.push(
                    QuantumIssue::new(
                        QuantumIssueKind::ParameterSemantics,
                        argument.node,
                        "QSEM0102",
                        format!(
                            "argument {} of operation `{}` has an incompatible type",
                            index,
                            contract.identity
                        ),
                    )
                    .with_related_node(invocation.node),
                );
            }

            if let (Some(expected_name), Some(actual_name)) =
                (&parameter.name, &argument.name)
            {
                if expected_name != actual_name {
                    issues.push(
                        QuantumIssue::new(
                            QuantumIssueKind::ParameterSemantics,
                            argument.node,
                            "QSEM0103",
                            format!(
                                "named argument `{actual_name}` does not match parameter \
                                 `{expected_name}`"
                            ),
                        )
                        .with_related_node(invocation.node),
                    );
                }
            }
        }
    }

    fn validate_targets<C: QuantumSemanticContext>(
        &self,
        invocation: &QuantumInvocation,
        contract: &OperationContract,
        context: &C,
        issues: &mut Vec<QuantumIssue>,
    ) {
        if !contract.targets.accepts(invocation.targets.len()) {
            issues.push(QuantumIssue::new(
                QuantumIssueKind::TargetSemantics,
                invocation.node,
                "QSEM0201",
                format!(
                    "operation `{}` received {} targets, outside its declared cardinality",
                    contract.identity,
                    invocation.targets.len()
                ),
            ));
        }

        let mut seen = BTreeMap::<&str, NodeId>::new();

        for target in &invocation.targets {
            if !context.types_compatible(&target.ty, &target.ty) {
                // A context must define reflexive compatibility for a resolved
                // type. This branch indicates a broken semantic service, not
                // a normal source-language mismatch.
                issues.push(QuantumIssue::new(
                    QuantumIssueKind::IncompleteAnalysis,
                    target.node,
                    "QSEM0202",
                    "shared type service rejected a type against itself",
                ));
            }

            if self.options.check_target_aliasing {
                if let Some(previous) = seen.insert(&target.identity, target.node) {
                    if !contract.permits_repeated_targets {
                        issues.push(
                            QuantumIssue::new(
                                QuantumIssueKind::TargetSemantics,
                                target.node,
                                "QSEM0203",
                                "operation uses the same target identity more than once",
                            )
                            .with_related_node(previous),
                        );
                    }
                }
            }
        }
    }

    fn validate_controls<C: QuantumSemanticContext>(
        &self,
        invocation: &QuantumInvocation,
        contract: &OperationContract,
        context: &C,
        issues: &mut Vec<QuantumIssue>,
    ) {
        if !contract.controls.accepts(invocation.controls.len()) {
            issues.push(QuantumIssue::new(
                QuantumIssueKind::ControlSemantics,
                invocation.node,
                "QSEM0301",
                format!(
                    "operation `{}` received {} controls, outside its declared cardinality",
                    contract.identity,
                    invocation.controls.len()
                ),
            ));
        }

        let target_ids: BTreeSet<&str> = invocation
            .targets
            .iter()
            .map(|target| target.identity.as_str())
            .collect();

        let mut seen = BTreeMap::<&str, NodeId>::new();

        for control in &invocation.controls {
            if !context.types_compatible(&control.ty, &control.ty) {
                issues.push(QuantumIssue::new(
                    QuantumIssueKind::IncompleteAnalysis,
                    control.node,
                    "QSEM0302",
                    "shared type service rejected a control type against itself",
                ));
            }

            if target_ids.contains(control.identity.as_str()) {
                issues.push(
                    QuantumIssue::new(
                        QuantumIssueKind::ControlSemantics,
                        control.node,
                        "QSEM0303",
                        "a control identity also appears among this invocation's targets",
                    )
                    .with_related_node(invocation.node),
                );
            }

            if let Some(previous) = seen.insert(&control.identity, control.node) {
                issues.push(
                    QuantumIssue::new(
                        QuantumIssueKind::ControlSemantics,
                        control.node,
                        "QSEM0304",
                        "the same control identity is specified more than once",
                    )
                    .with_related_node(previous),
                );
            }
        }
    }

    fn validate_modifiers(
        &self,
        invocation: &QuantumInvocation,
        contract: &OperationContract,
        issues: &mut Vec<QuantumIssue>,
    ) {
        let mut seen = BTreeMap::<&str, NodeId>::new();

        for modifier in &invocation.modifiers {
            if validate_extension_identifier(&modifier.identity).is_err() {
                issues.push(QuantumIssue::new(
                    QuantumIssueKind::ModifierSemantics,
                    modifier.node,
                    "QSEM0401",
                    "operation modifier has an invalid semantic identity",
                ));
                continue;
            }

            if !contract.allowed_modifiers.contains(&modifier.identity) {
                issues.push(
                    QuantumIssue::new(
                        QuantumIssueKind::ModifierSemantics,
                        modifier.node,
                        "QSEM0402",
                        format!(
                            "modifier `{}` is not permitted for operation `{}`",
                            modifier.identity, contract.identity
                        ),
                    )
                    .with_related_node(invocation.node),
                );
            }

            if let Some(previous) = seen.insert(&modifier.identity, modifier.node) {
                issues.push(
                    QuantumIssue::new(
                        QuantumIssueKind::ModifierSemantics,
                        modifier.node,
                        "QSEM0403",
                        format!("modifier `{}` is repeated", modifier.identity),
                    )
                    .with_related_node(previous),
                );
            }
        }
    }

    fn validate_results<C: QuantumSemanticContext>(
        &self,
        invocation: &QuantumInvocation,
        contract: &OperationContract,
        context: &C,
        issues: &mut Vec<QuantumIssue>,
    ) {
        if invocation.results.len() != contract.results.len() {
            issues.push(QuantumIssue::new(
                QuantumIssueKind::ResultSemantics,
                invocation.node,
                "QSEM0501",
                format!(
                    "operation `{}` declares {} result(s), but {} binding(s) were supplied",
                    contract.identity,
                    contract.results.len(),
                    invocation.results.len()
                ),
            ));
        }

        for (index, binding) in invocation.results.iter().enumerate() {
            let Some(expected) = contract.results.get(index) else {
                continue;
            };

            if !context.types_compatible(expected, &binding.ty) {
                issues.push(
                    QuantumIssue::new(
                        QuantumIssueKind::ResultSemantics,
                        binding.node,
                        "QSEM0502",
                        format!(
                            "result {} of operation `{}` has an incompatible type",
                            index, contract.identity
                        ),
                    )
                    .with_related_node(invocation.node),
                );
            }
        }
    }

    fn validate_measurement_and_boundaries<C: QuantumSemanticContext>(
        &self,
        invocation: &QuantumInvocation,
        contract: &OperationContract,
        context: &C,
        issues: &mut Vec<QuantumIssue>,
    ) {
        if contract.performs_measurement {
            if invocation.results.is_empty() {
                issues.push(QuantumIssue::new(
                    QuantumIssueKind::MeasurementSemantics,
                    invocation.node,
                    "QSEM0601",
                    "measurement operation has no declared result binding",
                ));
            }

            if self.options.check_boundaries
                && !context.boundary_permitted(
                    invocation.node,
                    QuantumBoundaryDirection::QuantumToClassical,
                )
            {
                issues.push(QuantumIssue::new(
                    QuantumIssueKind::BoundarySemantics,
                    invocation.node,
                    "QSEM0602",
                    "quantum-to-classical observation is not permitted in this context",
                ));
            }
        }

        let uses_classical_control = invocation.classical_control.conditional
            || invocation.classical_control.uses_feed_forward;

        if uses_classical_control {
            if !contract.permits_classical_control {
                issues.push(QuantumIssue::new(
                    QuantumIssueKind::BoundarySemantics,
                    invocation
                        .classical_control
                        .control_node
                        .unwrap_or(invocation.node),
                    "QSEM0603",
                    format!(
                        "operation `{}` does not permit classical control/feed-forward",
                        contract.identity
                    ),
                ));
            }

            if self.options.check_boundaries
                && !context.boundary_permitted(
                    invocation.node,
                    QuantumBoundaryDirection::ClassicalToQuantum,
                )
            {
                issues.push(QuantumIssue::new(
                    QuantumIssueKind::BoundarySemantics,
                    invocation
                        .classical_control
                        .control_node
                        .unwrap_or(invocation.node),
                    "QSEM0604",
                    "classical-to-quantum control is not permitted in this context",
                ));
            }
        }
    }

    fn validate_effects_and_capabilities<C: QuantumSemanticContext>(
        &self,
        invocation: &QuantumInvocation,
        contract: &OperationContract,
        context: &C,
        issues: &mut Vec<QuantumIssue>,
    ) {
        if self.options.check_effects {
            for effect in &contract.declared_effects {
                if !context.effect_permitted(effect, invocation.node) {
                    issues.push(QuantumIssue::new(
                        QuantumIssueKind::EffectSemantics,
                        invocation.node,
                        "QSEM0701",
                        format!(
                            "effect `{effect}` required by operation `{}` is not permitted",
                            contract.identity
                        ),
                    ));
                }
            }
        }

        if self.options.check_capabilities {
            for capability in &contract.required_capabilities {
                if !context.capability_established(capability, invocation.node) {
                    issues.push(QuantumIssue::new(
                        QuantumIssueKind::CapabilitySemantics,
                        invocation.node,
                        "QSEM0702",
                        format!(
                            "required capability obligation `{capability}` is not established"
                        ),
                    ));
                }
            }
        }
    }
}

/// Validate a set of quantum operation invocations with an already configured
/// validator.
///
/// This is a convenience wrapper for semantic pipelines that keep validator
/// construction and extension snapshot management outside individual passes.
pub fn validate_quantum_invocations<C: QuantumSemanticContext>(
    validator: &QuantumValidator,
    invocations: &[QuantumInvocation],
    context: &C,
) -> QuantumValidationReport {
    validator.validate(invocations, context)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct TestContext {
        denied_effects: BTreeSet<String>,
        missing_capabilities: BTreeSet<String>,
        denied_boundaries: BTreeSet<QuantumBoundaryDirection>,
    }

    impl QuantumSemanticContext for TestContext {
        fn types_compatible(
            &self,
            expected: &SemanticTypeIdentity,
            actual: &SemanticTypeIdentity,
        ) -> bool {
            expected == actual
        }

        fn capability_established(&self, capability: &str, _at: NodeId) -> bool {
            !self.missing_capabilities.contains(capability)
        }

        fn effect_permitted(&self, effect: &str, _at: NodeId) -> bool {
            !self.denied_effects.contains(effect)
        }

        fn boundary_permitted(
            &self,
            _at: NodeId,
            direction: QuantumBoundaryDirection,
        ) -> bool {
            !self.denied_boundaries.contains(&direction)
        }
    }

    fn identity(namespace: &str, name: &str) -> OperationIdentity {
        OperationIdentity::new(namespace, name).expect("valid test identity")
    }

    fn ty(name: &str) -> SemanticTypeIdentity {
        SemanticTypeIdentity::new("test", name).expect("valid test type")
    }

    fn contract(name: &str) -> OperationContract {
        OperationContract {
            identity: identity("test.quantum", name),
            parameters: Vec::new(),
            targets: Cardinality::one_or_more(),
            controls: Cardinality::any(),
            allowed_modifiers: BTreeSet::new(),
            results: Vec::new(),
            performs_measurement: false,
            permits_classical_control: false,
            permits_repeated_targets: false,
            required_capabilities: BTreeSet::new(),
            declared_effects: BTreeSet::new(),
        }
    }

    fn target(node: NodeId, id: &str) -> ResolvedTarget {
        ResolvedTarget {
            node,
            identity: id.to_owned(),
            ty: ty("logical-qubit"),
        }
    }

    fn invocation(name: &str) -> QuantumInvocation {
        QuantumInvocation {
            node: NodeId::new(1).expect("valid node id"),
            operation: identity("test.quantum", name),
            arguments: Vec::new(),
            targets: vec![target(NodeId::new(2).expect("valid node id"), "q")],
            controls: Vec::new(),
            modifiers: Vec::new(),
            results: Vec::new(),
            classical_control: ClassicalControlFacts::default(),
            resolution_complete: true,
        }
    }

    #[test]
    fn accepts_open_world_operation_identity() {
        let operation = identity("future.vendor.extension", "new_operation");
        assert_eq!(operation.namespace(), "future.vendor.extension");
        assert_eq!(operation.name(), "new_operation");
    }

    #[test]
    fn rejects_empty_operation_identity() {
        assert_eq!(
            OperationIdentity::new("", "measure"),
            Err(OperationIdentityError::EmptyNamespace)
        );
        assert_eq!(
            OperationIdentity::new("quantum", ""),
            Err(OperationIdentityError::EmptyName)
        );
    }

    #[test]
    fn rejects_invalid_cardinality() {
        assert_eq!(
            Cardinality::new(3, Some(2)),
            Err(CardinalityError::MaximumBelowMinimum)
        );
    }

    #[test]
    fn accepts_declared_operation_without_a_closed_gate_list() {
        let validator = QuantumValidator::new([contract("future_operation")])
            .expect("contract should be valid");
        let report = validator.validate(
            &[invocation("future_operation")],
            &TestContext::default(),
        );

        assert!(report.is_valid());
        assert!(report.is_complete());
    }

    #[test]
    fn rejects_unregistered_operation() {
        let validator =
            QuantumValidator::new([]).expect("empty registry is valid");
        let report =
            validator.validate(&[invocation("unknown")], &TestContext::default());

        assert!(!report.is_valid());
        assert_eq!(
            report.issues[0].kind,
            QuantumIssueKind::UnresolvedOperation
        );
    }

    #[test]
    fn rejects_unresolved_invocation_facts() {
        let validator = QuantumValidator::new([contract("op")])
            .expect("contract should be valid");
        let mut call = invocation("op");
        call.resolution_complete = false;

        let report = validator.validate(&[call], &TestContext::default());

        assert_eq!(report.completion, QuantumCompletion::Incomplete);
        assert!(!report.is_valid());
    }

    #[test]
    fn rejects_duplicate_operation_contracts() {
        let result = QuantumValidator::new([contract("op"), contract("op")]);
        assert!(matches!(
            result,
            Err(QuantumAnalysisFailure::InvalidContext { .. })
        ));
    }

    #[test]
    fn rejects_repeated_target_identity_when_contract_forbids_it() {
        let validator = QuantumValidator::new([contract("op")])
            .expect("contract should be valid");
        let mut call = invocation("op");
        call.targets
            .push(target(NodeId::new(3).expect("valid node id"), "q"));

        let report = validator.validate(&[call], &TestContext::default());

        assert!(report
            .issues
            .iter()
            .any(|issue| issue.rule == "QSEM0203"));
    }

    #[test]
    fn rejects_control_that_is_also_a_target() {
        let validator = QuantumValidator::new([contract("op")])
            .expect("contract should be valid");
        let mut call = invocation("op");
        call.controls.push(ResolvedControl {
            node: NodeId::new(4).expect("valid node id"),
            identity: "q".to_owned(),
            ty: ty("logical-qubit"),
        });

        let report = validator.validate(&[call], &TestContext::default());

        assert!(report
            .issues
            .iter()
            .any(|issue| issue.rule == "QSEM0303"));
    }

    #[test]
    fn reports_missing_capability_obligation() {
        let mut operation = contract("op");
        operation
            .required_capabilities
            .insert("quantum.measurement".to_owned());

        let validator = QuantumValidator::new([operation])
            .expect("contract should be valid");
        let mut context = TestContext::default();
        context
            .missing_capabilities
            .insert("quantum.measurement".to_owned());

        let report = validator.validate(&[invocation("op")], &context);

        assert!(report
            .issues
            .iter()
            .any(|issue| issue.rule == "QSEM0702"));
    }

    #[test]
    fn reports_denied_effect() {
        let mut operation = contract("op");
        operation.declared_effects.insert("quantum.execute".to_owned());

        let validator = QuantumValidator::new([operation])
            .expect("contract should be valid");
        let mut context = TestContext::default();
        context.denied_effects.insert("quantum.execute".to_owned());

        let report = validator.validate(&[invocation("op")], &context);

        assert!(report
            .issues
            .iter()
            .any(|issue| issue.rule == "QSEM0701"));
    }

    #[test]
    fn output_order_follows_invocation_order() {
        let validator =
            QuantumValidator::new([contract("first"), contract("second")])
                .expect("contracts should be valid");

        let calls = [invocation("second"), invocation("first")];
        let report = validator.validate(&calls, &TestContext::default());

        assert_eq!(report.invocations[0].operation.name(), "second");
        assert_eq!(report.invocations[1].operation.name(), "first");
    }

    #[test]
    fn type_contracts_use_the_shared_type_service() {
        let mut operation = contract("op");
        operation.parameters.push(ParameterContract {
            name: Some("angle".to_owned()),
            expected_type: ty("angle"),
            optional: false,
        });

        let validator = QuantumValidator::new([operation])
            .expect("contract should be valid");
        let mut call = invocation("op");
        call.arguments.push(ResolvedArgument {
            node: NodeId::new(5).expect("valid node id"),
            ty: ty("string"),
            name: Some("angle".to_owned()),
        });

        let report = validator.validate(&[call], &TestContext::default());

        assert!(report
            .issues
            .iter()
            .any(|issue| issue.rule == "QSEM0102"));
    }
}

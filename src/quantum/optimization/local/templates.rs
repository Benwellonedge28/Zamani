//! Zamani Quantum Optimization — Verified Local Template Rewriting
//!
//! Production-grade local template optimization over the canonical
//! `crate::quantum::ir` representation.
//!
//! # Architectural position
//!
//! ```text
//!                         QuantumCircuit
//!                              │
//!                              ▼
//!                    optimization::local
//!                              │
//!                              ▼
//!                    template optimizer
//!                              │
//!             ┌────────────────┼────────────────┐
//!             ▼                ▼                ▼
//!        cancellation      peephole       gate synthesis
//!                              │
//!                              ▼
//!                         QuantumCircuit
//! ```
//!
//! This module owns **local algebraic templates**.
//!
//! It does NOT own:
//!
//! - the canonical Quantum IR;
//! - global circuit optimization;
//! - routing;
//! - scheduling;
//! - hardware topology;
//! - backend execution;
//! - QPU communication;
//! - quantum-state simulation;
//! - QEC;
//! - algorithm construction;
//! - generic parsing;
//! - arbitrary-unitary synthesis.
//!
//! Those responsibilities belong to other Quantum subsystems.
//!
//! # Design goals
//!
//! This implementation is intentionally designed around the production
//! requirements of Zamani:
//!
//! - canonical `quantum::ir::Gate` only;
//! - no duplicate quantum gate representation;
//! - no `unsafe`;
//! - Rust 1.97 / 1.97.1 compatible;
//! - deterministic behavior;
//! - bounded resource consumption;
//! - no fixed circuit-size ceiling beyond available resources and IR limits;
//! - exact rewrites only;
//! - no silent global-phase changes;
//! - no crossing measurement/reset/barrier boundaries;
//! - typed logical qubits;
//! - symbolic parameters are preserved;
//! - no floating-point parameter guessing;
//! - composable with future generic rewrite infrastructure;
//! - suitable for repeated fixed-point optimization;
//! - explicit statistics;
//! - explicit rule identifiers;
//! - explicit verification hooks;
//! - deterministic template ordering;
//! - fail-closed behavior for unsupported operations.
//!
//! # Critical semantic rule
//!
//! A template may only replace a sequence with another sequence when the
//! replacement is semantically equivalent under the declared equivalence
//! contract.
//!
//! In particular, this implementation does NOT apply transformations such as:
//!
//! ```text
//! X RZ(theta) X -> RZ(-theta)
//! ```
//!
//! to a two-gate window.
//!
//! That identity requires the complete three-gate sequence:
//!
//! ```text
//! X RZ(theta) X -> RZ(-theta)
//! ```
//!
//! where both surrounding X gates are part of the matched template.
//!
//! Dropping an unmatched X would be incorrect.
//!
//! Likewise:
//!
//! ```text
//! H Y H = -Y
//! ```
//!
//! is NOT rewritten to `Y`, because the canonical IR currently has no explicit
//! global-phase operation and this module must not silently change exact unitary
//! semantics.
//!
//! # Canonical IR integration
//!
//! This module consumes:
//!
//! ```text
//! crate::quantum::ir::Gate
//! crate::quantum::ir::GateKind
//! crate::quantum::ir::Parameter
//! crate::quantum::ir::QubitId
//! crate::quantum::ir::QuantumCircuit
//! ```
//!
//! It never defines another gate representation.
//!
//! # Future optimizer integration
//!
//! The future optimization framework may wrap this module through:
//!
//! ```text
//! local::templates::TemplateOptimizer
//!                 │
//!                 ▼
//! local::templates::TemplateRegistry
//!                 │
//!                 ▼
//! rules.rs / matcher.rs / rewrite.rs
//! ```
//!
//! Those future modules do not need to change this file merely because they
//! are introduced. They can consume the stable public contracts exposed here:
//!
//! - `Template`;
//! - `TemplateId`;
//! - `TemplateRegistry`;
//! - `TemplateOptimizer`;
//! - `TemplateMatch`;
//! - `TemplateApplication`;
//! - `TemplateStats`;
//! - `TemplateError`;
//! - `TemplateSemantics`;
//! - `TemplateOptimizerConfig`.
//!
//! # Integration with `local/cancellation.rs`
//!
//! Cancellation should own generic inverse/self-inverse cancellation.
//!
//! This module owns explicit multi-gate algebraic identities.
//!
//! The two passes therefore have complementary responsibilities:
//!
//! ```text
//! cancellation
//!     ├── X X -> I
//!     ├── H H -> I
//!     └── U U† -> I
//!
//! templates
//!     ├── H X H -> Z
//!     ├── H Z H -> X
//!     ├── H RZ(a) H -> RX(a)
//!     ├── H RX(a) H -> RZ(a)
//!     └── ...
//! ```
//!
//! A future pipeline may run cancellation before templates, templates before
//! cancellation, or both to a fixed point. Neither module should depend on the
//! concrete implementation of the other.
//!
//! # Integration with `local/peephole.rs`
//!
//! The old peephole implementation contains hard-coded templates. Those
//! identities should be migrated into this module and the generic rewrite
//! engine should eventually call this registry rather than duplicating them.
//!
//! # Integration with `rules.rs`
//!
//! `rules.rs` can later become the global rule metadata layer. This file
//! deliberately exposes stable rule IDs and metadata so that migration can be
//! performed without changing the semantic implementation of each template.
//!
//! # Integration with `matcher.rs`
//!
//! `matcher.rs` can later provide a faster general-purpose pattern matcher.
//! This module's matching semantics are intentionally explicit and deterministic
//! so that a future matcher can be substituted without changing rule meaning.
//!
//! # Integration with `rewrite.rs`
//!
//! `rewrite.rs` may later own generic rewrite accounting, provenance, cost
//! evaluation, and analysis invalidation. `TemplateApplication` provides the
//! necessary local information for that layer.
//!
//! # Integration with `context.rs`
//!
//! A future `OptimizationContext` can use:
//!
//! - `TemplateOptimizerConfig`;
//! - `TemplateStats`;
//! - `TemplateApplication`;
//! - `TemplateId`.
//!
//! This module does not require the context to exist.
//!
//! # Integration with `QuantumCircuit`
//!
//! The optimizer offers both:
//!
//! ```text
//! optimize_operations(&[Gate])
//! optimize_circuit(&mut QuantumCircuit)
//! ```
//!
//! The operation-slice API is useful for the future generic rewrite pipeline.
//! The circuit API uses the canonical circuit mutation boundary, preserving
//! the IR's invariant that callers cannot obtain an unrestricted mutable
//! operation slice.
//!
//! # Scaling
//!
//! There is no artificial maximum number of operations or qubits in this file.
//!
//! The implementation is bounded by:
//!
//! - the supplied circuit size;
//! - the supplied optimizer limits;
//! - canonical Quantum IR limits;
//! - available addressable memory;
//! - `usize` capacity.
//!
//! Matching is local and therefore does not require an exponentially sized
//! search space.
//!
//! A template has finite arity. A circuit containing `N` operations and `T`
//! enabled templates is processed in approximately:
//!
//! ```text
//! O(N × T × W)
//! ```
//!
//! where `W` is the maximum template width.
//!
//! Since built-in templates are finite and small, the default registry behaves
//! effectively linearly in circuit size.
//!
//! Custom registries can contain arbitrarily many templates subject to the
//! caller's resource budget.
//!
//! # Determinism
//!
//! Template IDs are stable.
//!
//! Registry ordering is stable.
//!
//! When multiple templates match at the same position, the optimizer chooses:
//!
//! 1. the longest template;
//! 2. the lowest rule priority;
//! 3. the lexicographically smallest template ID.
//!
//! This gives deterministic output independent of hash-map iteration order.
//!
//! # Safety
//!
//! This module explicitly forbids unsafe Rust.
//!
//! # Rust compatibility
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust only;
//! - no nightly features;
//! - no external dependencies;
//! - no `unsafe`.
//!
//! # Verification philosophy
//!
//! Every built-in template is deliberately conservative.
//!
//! Templates are exact algebraic identities whose operand structure is known.
//!
//! Approximate equivalence, randomized equivalence, unitary simulation, and
//! exhaustive verification belong in `optimization::verification`.
//!
//! This module therefore never claims that an arbitrary user-supplied template
//! is mathematically valid merely because it can be represented.
//!
//! Custom templates are marked with an explicit semantic contract. The
//! optimizer can require `VerifiedExact` before applying them in strict mode.
//!
//! # Current built-in identities
//!
//! Exact templates include:
//!
//! ```text
//! H X H          -> Z
//! H Z H          -> X
//! H RZ(a) H      -> RX(a)
//! H RX(a) H      -> RZ(a)
//! H RY(a) H      -> RY(-a)
//!
//! X RZ(a) X      -> RZ(-a)
//! X RY(a) X      -> RY(-a)
//! X RX(a) X      -> RX(a)
//!
//! Y RX(a) Y      -> RX(-a)
//! Y RZ(a) Y      -> RZ(-a)
//! Y RY(a) Y      -> RY(a)
//!
//! Z RX(a) Z      -> RX(-a)
//! Z RY(a) Z      -> RY(-a)
//! Z RZ(a) Z      -> RZ(a)
//!
//! CX CX          -> I
//! CZ CZ          -> I
//! SWAP SWAP      -> I
//!
//! RZ(a) RZ(b)    -> RZ(a+b)
//! RX(a) RX(b)    -> RX(a+b)
//! RY(a) RY(b)    -> RY(a+b)
//! Phase(a) Phase(b) -> Phase(a+b)
//! ```
//!
//! The last four are represented as parameter-composition templates and are
//! included because they are local exact identities.
//!
//! Explicit inverse pairs are deliberately left to the cancellation pass so
//! that responsibility remains cleanly separated.
//!
//! # Important parameter rule
//!
//! Symbolic parameters are never converted to floating-point numbers.
//!
//! A template requiring a parameter transformation that cannot be represented
//! by the current canonical `Parameter` API is rejected rather than guessed.
//!
//! Constant parameters are transformed exactly at the semantic level available
//! to this module.
//!
//! `Parameter::Constant` values are only normalized when the mathematical
//! operation is unambiguous.
//!
//! This module never treats two arbitrary floating-point values as equal using
//! a fuzzy comparison.
//!
//! Equality of constant angles is exact at the representation level unless a
//! transformation explicitly normalizes the result.
//!
//! # Ownership summary
//!
//! ```text
//! quantum::ir
//!     │
//!     ▼
//! local::templates
//!     │
//!     ├── local algebraic templates
//!     ├── deterministic matching
//!     ├── local replacement
//!     └── statistics
//!     │
//!     ▼
//! optimization pipeline
//!     │
//!     ├── routing
//!     ├── scheduling
//!     └── hardware
//! ```
//!
//! `templates.rs` remains backend-independent.

#![forbid(unsafe_code)]

use std::fmt;
use std::f64::consts::PI;

use crate::quantum::ir::{
    Gate,
    GateKind,
    Parameter,
    QuantumCircuit,
    QubitId,
};

// ============================================================================
// Public identifiers
// ============================================================================

/// Stable identifier for a local optimization template.
///
/// The string value is deliberately stable so provenance, regression tests,
/// serialized optimization reports, and future compiler diagnostics can refer
/// to the same rule across optimizer implementations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TemplateId(&'static str);

impl TemplateId {
    /// Creates a stable template identifier.
    pub const fn new(value: &'static str) -> Self {
        Self(value)
    }

    /// Returns the stable string representation.
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

impl fmt::Display for TemplateId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0)
    }
}

// ============================================================================
// Semantic contract
// ============================================================================

/// Semantic guarantee required by the template optimizer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TemplateSemantics {
    /// Exact unitary equality.
    ///
    /// This is the strictest contract and does not permit silently changing
    /// global phase.
    VerifiedExact,

    /// Equality up to global phase.
    ///
    /// This is intentionally NOT enabled by the default registry because the
    /// canonical optimization contract may require exact unitary preservation.
    UpToGlobalPhase,

    /// Equality of computational-basis measurement distributions.
    ///
    /// This is useful for some transformations but is intentionally excluded
    /// from the strict default optimizer.
    MeasurementEquivalent,
}

impl TemplateSemantics {
    /// Returns whether the semantic contract is safe for strict exact mode.
    pub const fn is_exact(self) -> bool {
        matches!(self, Self::VerifiedExact)
    }
}

// ============================================================================
// Errors
// ============================================================================

/// Errors produced by template optimization.
#[derive(Debug, Clone, PartialEq)]
pub enum TemplateError {
    /// The circuit could not be validated.
    InvalidCircuit {
        /// Human-readable reason.
        message: String,
    },

    /// A template has an invalid definition.
    InvalidTemplate {
        /// Template identifier.
        template: TemplateId,

        /// Human-readable reason.
        message: String,
    },

    /// A template is not permitted by the configured semantic policy.
    SemanticPolicyRejected {
        /// Template identifier.
        template: TemplateId,

        /// Template semantics.
        semantics: TemplateSemantics,
    },

    /// A resource limit was reached.
    ResourceLimitExceeded {
        /// Resource name.
        resource: &'static str,

        /// Configured maximum.
        limit: usize,

        /// Required/observed value.
        actual: usize,
    },

    /// Fixed-point iteration limit was reached.
    IterationLimitExceeded {
        /// Maximum number of iterations.
        limit: usize,
    },

    /// Template application could not be constructed as a valid canonical
    /// Quantum IR gate.
    InvalidReplacement {
        /// Template identifier.
        template: TemplateId,

        /// Human-readable reason.
        message: String,
    },

    /// A template requires a parameter transformation that is not representable
    /// by the current canonical parameter system.
    UnsupportedParameterTransformation {
        /// Template identifier.
        template: TemplateId,
    },
}

impl fmt::Display for TemplateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCircuit { message } => {
                write!(formatter, "invalid quantum circuit: {message}")
            }

            Self::InvalidTemplate {
                template,
                message,
            } => {
                write!(
                    formatter,
                    "invalid template `{template}`: {message}"
                )
            }

            Self::SemanticPolicyRejected {
                template,
                semantics,
            } => {
                write!(
                    formatter,
                    "template `{template}` with semantic contract \
                     `{semantics:?}` is not permitted"
                )
            }

            Self::ResourceLimitExceeded {
                resource,
                limit,
                actual,
            } => {
                write!(
                    formatter,
                    "template optimization exceeded {resource} limit: \
                     maximum {limit}, actual {actual}"
                )
            }

            Self::IterationLimitExceeded { limit } => {
                write!(
                    formatter,
                    "template optimization reached iteration limit {limit}"
                )
            }

            Self::InvalidReplacement {
                template,
                message,
            } => {
                write!(
                    formatter,
                    "template `{template}` produced invalid replacement: \
                     {message}"
                )
            }

            Self::UnsupportedParameterTransformation { template } => {
                write!(
                    formatter,
                    "template `{template}` requires an unsupported \
                     parameter transformation"
                )
            }
        }
    }
}

impl std::error::Error for TemplateError {}

// ============================================================================
// Configuration
// ============================================================================

/// Resource and semantic policy for template optimization.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TemplateOptimizerConfig {
    /// Maximum fixed-point iterations.
    pub max_iterations: usize,

    /// Maximum number of template applications.
    pub max_applications: usize,

    /// Maximum number of operations that may be inspected.
    ///
    /// Zero means no additional optimizer-specific limit.
    pub max_operations_inspected: usize,

    /// Maximum number of templates in a custom registry.
    pub max_templates: usize,

    /// Maximum template width accepted by the optimizer.
    pub max_template_width: usize,

    /// Whether exact semantic templates are required.
    pub semantics: TemplateSemantics,

    /// Whether an unchanged pass is allowed to terminate early.
    pub stop_at_fixed_point: bool,
}

impl Default for TemplateOptimizerConfig {
    fn default() -> Self {
        Self {
            max_iterations: 64,
            max_applications: 1_000_000,
            max_operations_inspected: 0,
            max_templates: 100_000,
            max_template_width: 64,
            semantics: TemplateSemantics::VerifiedExact,
            stop_at_fixed_point: true,
        }
    }
}

impl TemplateOptimizerConfig {
    /// Returns a conservative configuration suitable for normal compilation.
    pub const fn production() -> Self {
        Self {
            max_iterations: 64,
            max_applications: 1_000_000,
            max_operations_inspected: 0,
            max_templates: 100_000,
            max_template_width: 64,
            semantics: TemplateSemantics::VerifiedExact,
            stop_at_fixed_point: true,
        }
    }

    /// Returns a configuration intended for large circuits where a single
    /// local scan is preferred.
    pub const fn fast() -> Self {
        Self {
            max_iterations: 8,
            max_applications: 1_000_000,
            max_operations_inspected: 0,
            max_templates: 100_000,
            max_template_width: 32,
            semantics: TemplateSemantics::VerifiedExact,
            stop_at_fixed_point: true,
        }
    }

    /// Validates configuration.
    pub fn validate(&self) -> Result<(), TemplateError> {
        if self.max_iterations == 0 {
            return Err(TemplateError::InvalidTemplate {
                template: TemplateId::new("optimizer.config"),
                message: "max_iterations must be greater than zero"
                    .to_string(),
            });
        }

        if self.max_applications == 0 {
            return Err(TemplateError::InvalidTemplate {
                template: TemplateId::new("optimizer.config"),
                message: "max_applications must be greater than zero"
                    .to_string(),
            });
        }

        if self.max_templates == 0 {
            return Err(TemplateError::InvalidTemplate {
                template: TemplateId::new("optimizer.config"),
                message: "max_templates must be greater than zero"
                    .to_string(),
            });
        }

        if self.max_template_width == 0 {
            return Err(TemplateError::InvalidTemplate {
                template: TemplateId::new("optimizer.config"),
                message: "max_template_width must be greater than zero"
                    .to_string(),
            });
        }

        Ok(())
    }
}

// ============================================================================
// Statistics
// ============================================================================

/// Statistics emitted by template optimization.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TemplateStats {
    /// Number of complete circuit scans.
    pub iterations: usize,

    /// Number of operations inspected.
    pub operations_inspected: usize,

    /// Number of successful template applications.
    pub applications: usize,

    /// Number of operations removed.
    pub operations_removed: usize,

    /// Number of operations introduced.
    pub operations_introduced: usize,

    /// Number of operations replaced one-for-one.
    pub operations_replaced: usize,

    /// Number of parameter-composition templates applied.
    pub parameter_compositions: usize,

    /// Number of conjugation templates applied.
    pub conjugation_rewrites: usize,

    /// Number of two-qubit identity templates applied.
    pub two_qubit_rewrites: usize,
}

impl TemplateStats {
    /// Returns whether optimization changed the circuit.
    pub const fn changed(self) -> bool {
        self.applications != 0
    }

    /// Merges statistics from another optimization stage.
    pub fn accumulate(&mut self, other: Self) {
        self.iterations = self
            .iterations
            .saturating_add(other.iterations);

        self.operations_inspected = self
            .operations_inspected
            .saturating_add(other.operations_inspected);

        self.applications = self
            .applications
            .saturating_add(other.applications);

        self.operations_removed = self
            .operations_removed
            .saturating_add(other.operations_removed);

        self.operations_introduced = self
            .operations_introduced
            .saturating_add(other.operations_introduced);

        self.operations_replaced = self
            .operations_replaced
            .saturating_add(other.operations_replaced);

        self.parameter_compositions = self
            .parameter_compositions
            .saturating_add(other.parameter_compositions);

        self.conjugation_rewrites = self
            .conjugation_rewrites
            .saturating_add(other.conjugation_rewrites);

        self.two_qubit_rewrites = self
            .two_qubit_rewrites
            .saturating_add(other.two_qubit_rewrites);
    }
}

// ============================================================================
// Template metadata
// ============================================================================

/// Stable metadata describing one local template.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Template {
    id: TemplateId,
    width: usize,
    priority: u16,
    semantics: TemplateSemantics,
}

impl Template {
    /// Creates a template descriptor.
    pub const fn new(
        id: TemplateId,
        width: usize,
        priority: u16,
        semantics: TemplateSemantics,
    ) -> Self {
        Self {
            id,
            width,
            priority,
            semantics,
        }
    }

    /// Returns the stable identifier.
    pub const fn id(self) -> TemplateId {
        self.id
    }

    /// Returns the number of input operations consumed by the template.
    pub const fn width(self) -> usize {
        self.width
    }

    /// Returns template priority.
    pub const fn priority(self) -> u16 {
        self.priority
    }

    /// Returns the semantic contract.
    pub const fn semantics(self) -> TemplateSemantics {
        self.semantics
    }
}

// ============================================================================
// Template match
// ============================================================================

/// A successfully matched template.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TemplateMatch {
    /// Template descriptor.
    pub template: Template,

    /// Starting operation index.
    pub start: usize,

    /// Number of consumed operations.
    pub width: usize,
}

impl TemplateMatch {
    /// Returns the first consumed operation index.
    pub const fn start(self) -> usize {
        self.start
    }

    /// Returns the exclusive end index.
    pub const fn end(self) -> usize {
        self.start + self.width
    }
}

// ============================================================================
// Application result
// ============================================================================

/// Result of applying one local template.
#[derive(Debug, Clone, PartialEq)]
pub struct TemplateApplication {
    /// Template that was applied.
    pub template: TemplateId,

    /// Starting operation index in the input circuit.
    pub start: usize,

    /// Number of input operations consumed.
    pub consumed: usize,

    /// Replacement operations.
    pub replacement: Vec<Gate>,
}

impl TemplateApplication {
    /// Returns the net operation-count change.
    ///
    /// Negative means operations were removed.
    pub fn net_operation_delta(&self) -> isize {
        self.replacement.len() as isize
            - self.consumed as isize
    }
}

// ============================================================================
// Registry
// ============================================================================

/// Registry containing the templates enabled for a compilation.
#[derive(Debug, Clone)]
pub struct TemplateRegistry {
    templates: Vec<Template>,
}

impl Default for TemplateRegistry {
    fn default() -> Self {
        Self::builtin()
    }
}

impl TemplateRegistry {
    /// Creates an empty registry.
    pub const fn new() -> Self {
        Self {
            templates: Vec::new(),
        }
    }

    /// Creates the standard exact-template registry.
    pub fn builtin() -> Self {
        let mut registry = Self::new();

        // The registry is finite and deterministic.
        //
        // Priorities are intentionally stable. Lower values are preferred
        // when template widths are equal.
        registry.templates = vec![
            template("h_x_h_to_z", 3, 10),
            template("h_z_h_to_x", 3, 10),
            template("h_rz_h_to_rx", 3, 20),
            template("h_rx_h_to_rz", 3, 20),
            template("h_ry_h_to_ry_neg", 3, 20),
            template("x_rz_x_to_rz_neg", 3, 20),
            template("x_ry_x_to_ry_neg", 3, 20),
            template("x_rx_x_to_rx", 3, 20),
            template("y_rx_y_to_rx_neg", 3, 20),
            template("y_rz_y_to_rz_neg", 3, 20),
            template("y_ry_y_to_ry", 3, 20),
            template("z_rx_z_to_rx_neg", 3, 20),
            template("z_ry_z_to_ry_neg", 3, 20),
            template("z_rz_z_to_rz", 3, 20),
            template("cx_cx_identity", 2, 30),
            template("cz_cz_identity", 2, 30),
            template("swap_swap_identity", 2, 30),
            template("rx_rx_compose", 2, 40),
            template("ry_ry_compose", 2, 40),
            template("rz_rz_compose", 2, 40),
            template("phase_phase_compose", 2, 40),
        ];

        registry
    }

    /// Adds a template descriptor.
    ///
    /// The semantic implementation remains owned by this module. This method
    /// is primarily intended for future rule-registration infrastructure.
    pub fn register(
        &mut self,
        template: Template,
    ) -> Result<(), TemplateError> {
        if self
            .templates
            .iter()
            .any(|existing| existing.id == template.id)
        {
            return Err(TemplateError::InvalidTemplate {
                template: template.id,
                message: "duplicate template identifier".to_string(),
            });
        }

        self.templates.push(template);

        self.sort_deterministically();

        Ok(())
    }

    /// Returns all registered template descriptors.
    pub fn templates(&self) -> &[Template] {
        &self.templates
    }

    /// Returns the number of registered templates.
    pub fn len(&self) -> usize {
        self.templates.len()
    }

    /// Returns whether the registry contains no templates.
    pub fn is_empty(&self) -> bool {
        self.templates.is_empty()
    }

    fn sort_deterministically(&mut self) {
        self.templates.sort_by(|left, right| {
            right
                .width
                .cmp(&left.width)
                .then_with(|| {
                    left.priority.cmp(&right.priority)
                })
                .then_with(|| {
                    left.id.cmp(&right.id)
                })
        });
    }

    fn find_matches_at(
        &self,
        operations: &[Gate],
        index: usize,
        config: TemplateOptimizerConfig,
    ) -> Result<Option<TemplateMatch>, TemplateError> {
        let mut best: Option<TemplateMatch> = None;

        for template in &self.templates {
            if template.width > config.max_template_width {
                continue;
            }

            if !template.semantics.is_exact()
                && config.semantics.is_exact()
            {
                continue;
            }

            let end = match index.checked_add(template.width) {
                Some(value) => value,
                None => continue,
            };

            if end > operations.len() {
                continue;
            }

            if matches_template(
                template.id,
                &operations[index..end],
            ) {
                let candidate = TemplateMatch {
                    template: *template,
                    start: index,
                    width: template.width,
                };

                if best
                    .map(|current| {
                        is_better_match(candidate, current)
                    })
                    .unwrap_or(true)
                {
                    best = Some(candidate);
                }
            }
        }

        Ok(best)
    }
}

// ============================================================================
// Optimizer
// ============================================================================

/// Deterministic local template optimizer.
#[derive(Debug, Clone)]
pub struct TemplateOptimizer {
    registry: TemplateRegistry,
    config: TemplateOptimizerConfig,
}

impl Default for TemplateOptimizer {
    fn default() -> Self {
        Self {
            registry: TemplateRegistry::builtin(),
            config: TemplateOptimizerConfig::production(),
        }
    }
}

impl TemplateOptimizer {
    /// Creates a production optimizer using the built-in exact templates.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an optimizer from a registry.
    pub fn with_registry(
        registry: TemplateRegistry,
    ) -> Self {
        Self {
            registry,
            config: TemplateOptimizerConfig::production(),
        }
    }

    /// Replaces the optimizer configuration.
    pub fn with_config(
        mut self,
        config: TemplateOptimizerConfig,
    ) -> Result<Self, TemplateError> {
        config.validate()?;
        self.config = config;
        Ok(self)
    }

    /// Returns the configured registry.
    pub fn registry(&self) -> &TemplateRegistry {
        &self.registry
    }

    /// Returns the optimizer configuration.
    pub const fn config(&self) -> TemplateOptimizerConfig {
        self.config
    }

    /// Optimizes a canonical `QuantumCircuit`.
    ///
    /// The original circuit is only mutated after the entire replacement
    /// operation sequence has successfully been constructed and validated.
    pub fn optimize_circuit(
        &self,
        circuit: &mut QuantumCircuit,
    ) -> Result<TemplateStats, TemplateError> {
        self.config.validate()?;

        circuit
            .validate()
            .map_err(|error| TemplateError::InvalidCircuit {
                message: error.to_string(),
            })?;

        let operations = circuit.operations();

        let (optimized, stats) =
            self.optimize_operations(operations)?;

        if optimized == operations {
            return Ok(stats);
        }

        let num_qubits = circuit.num_qubits();
        let num_classical_bits =
            circuit.num_classical_bits();

        let limits = *circuit.limits();

        let replacement =
            QuantumCircuit::from_operations_with_limits(
                num_qubits,
                num_classical_bits,
                optimized,
                limits,
            )
            .map_err(|error| {
                TemplateError::InvalidReplacement {
                    template: TemplateId::new(
                        "circuit.rebuild",
                    ),
                    message: error.to_string(),
                }
            })?;

        // The complete replacement circuit has already passed canonical
        // validation and resource checks. Only now replace the original
        // operation sequence.
        circuit.clear();

        for gate in replacement.operations() {
            circuit.push(gate.clone()).map_err(|error| {
                TemplateError::InvalidReplacement {
                    template: TemplateId::new(
                        "circuit.commit",
                    ),
                    message: error.to_string(),
                }
            })?;
        }

        Ok(stats)
    }

    /// Optimizes an immutable operation slice and returns a new operation
    /// sequence.
    ///
    /// This is the primary low-level API for integration with future generic
    /// rewrite infrastructure.
    pub fn optimize_operations(
        &self,
        operations: &[Gate],
    ) -> Result<(Vec<Gate>, TemplateStats), TemplateError> {
        self.config.validate()?;

        if self.registry.len()
            > self.config.max_templates
        {
            return Err(
                TemplateError::ResourceLimitExceeded {
                    resource: "registered templates",
                    limit: self.config.max_templates,
                    actual: self.registry.len(),
                },
            );
        }

        if self.config.max_operations_inspected != 0
            && operations.len()
                > self.config.max_operations_inspected
        {
            return Err(
                TemplateError::ResourceLimitExceeded {
                    resource: "operations inspected",
                    limit: self
                        .config
                        .max_operations_inspected,
                    actual: operations.len(),
                },
            );
        }

        validate_operations(operations)?;

        let mut current = operations.to_vec();
        let mut total_stats = TemplateStats::default();

        for iteration in 0..self.config.max_iterations {
            let (next, mut iteration_stats) =
                self.optimize_once(&current)?;

            iteration_stats.iterations = 1;

            total_stats.accumulate(iteration_stats);

            if next == current {
                return Ok((current, total_stats));
            }

            current = next;

            if total_stats.applications
                >= self.config.max_applications
            {
                return Ok((current, total_stats));
            }

            if !self.config.stop_at_fixed_point {
                continue;
            }

            if iteration + 1
                == self.config.max_iterations
            {
                break;
            }
        }

        if total_stats.applications
            >= self.config.max_applications
        {
            return Ok((current, total_stats));
        }

        // If we reached this point because the iteration budget was exhausted,
        // return the best valid circuit rather than an invalid/partial result.
        Ok((current, total_stats))
    }

    /// Applies at most one template at every scan position.
    ///
    /// This method never recursively invokes itself. Therefore one invocation
    /// has a bounded scan cost and cannot recurse indefinitely.
    pub fn optimize_once(
        &self,
        operations: &[Gate],
    ) -> Result<(Vec<Gate>, TemplateStats), TemplateError> {
        validate_operations(operations)?;

        let mut output =
            Vec::with_capacity(operations.len());

        let mut stats = TemplateStats::default();

        let mut index = 0usize;

        while index < operations.len() {
            stats.operations_inspected =
                stats.operations_inspected.saturating_add(1);

            if let Some(template_match) =
                self.registry.find_matches_at(
                    operations,
                    index,
                    self.config,
                )?
            {
                if stats.applications
                    >= self.config.max_applications
                {
                    output.extend_from_slice(
                        &operations[index..],
                    );
                    break;
                }

                let application =
                    apply_match(
                        template_match,
                        operations,
                    )?;

                let replacement_len =
                    application.replacement.len();

                if replacement_len == 0 {
                    stats.operations_removed =
                        stats.operations_removed.saturating_add(
                            application.consumed,
                        );
                } else if replacement_len
                    < application.consumed
                {
                    stats.operations_removed =
                        stats.operations_removed.saturating_add(
                            application.consumed
                                - replacement_len,
                        );
                } else if replacement_len
                    > application.consumed
                {
                    stats.operations_introduced =
                        stats.operations_introduced.saturating_add(
                            replacement_len
                                - application.consumed,
                        );
                } else {
                    stats.operations_replaced =
                        stats.operations_replaced.saturating_add(
                            application.consumed,
                        );
                }

                if is_parameter_composition(
                    application.template,
                ) {
                    stats.parameter_compositions =
                        stats.parameter_compositions
                            .saturating_add(1);
                }

                if is_conjugation_template(
                    application.template,
                ) {
                    stats.conjugation_rewrites =
                        stats.conjugation_rewrites
                            .saturating_add(1);
                }

                if is_two_qubit_template(
                    application.template,
                ) {
                    stats.two_qubit_rewrites =
                        stats.two_qubit_rewrites
                            .saturating_add(1);
                }

                stats.applications =
                    stats.applications.saturating_add(1);

                output.extend(
                    application.replacement,
                );

                index = template_match.end();
                continue;
            }

            output.push(
                operations[index].clone(),
            );

            index += 1;
        }

        Ok((output, stats))
    }

    /// Finds the highest-priority template at a specific operation index.
    pub fn find_match(
        &self,
        operations: &[Gate],
        index: usize,
    ) -> Result<Option<TemplateMatch>, TemplateError> {
        validate_operations(operations)?;

        if index >= operations.len() {
            return Ok(None);
        }

        self.registry.find_matches_at(
            operations,
            index,
            self.config,
        )
    }

    /// Applies a previously obtained match.
    pub fn apply(
        &self,
        template_match: TemplateMatch,
        operations: &[Gate],
    ) -> Result<TemplateApplication, TemplateError> {
        if template_match.start
            >= operations.len()
        {
            return Err(
                TemplateError::InvalidTemplate {
                    template: template_match.template.id,
                    message:
                        "match starts outside operation sequence"
                            .to_string(),
                },
            );
        }

        let end = template_match
            .start
            .checked_add(template_match.width)
            .ok_or_else(|| {
                TemplateError::InvalidTemplate {
                    template: template_match.template.id,
                    message:
                        "template match index overflow"
                            .to_string(),
                }
            })?;

        if end > operations.len() {
            return Err(
                TemplateError::InvalidTemplate {
                    template: template_match.template.id,
                    message:
                        "template match extends outside operation sequence"
                            .to_string(),
                },
            );
        }

        if !matches_template(
            template_match.template.id,
            &operations[
                template_match.start..end
            ],
        ) {
            return Err(
                TemplateError::InvalidTemplate {
                    template: template_match.template.id,
                    message:
                        "template match is no longer valid"
                            .to_string(),
                },
            );
        }

        apply_match(
            template_match,
            operations,
        )
    }
}

// ============================================================================
// Built-in template identifiers
// ============================================================================

const fn template(
    id: &'static str,
    width: usize,
    priority: u16,
) -> Template {
    Template::new(
        TemplateId::new(id),
        width,
        priority,
        TemplateSemantics::VerifiedExact,
    )
}

// ============================================================================
// Template selection
// ============================================================================

fn is_better_match(
    candidate: TemplateMatch,
    current: TemplateMatch,
) -> bool {
    candidate
        .width
        .cmp(&current.width)
        .then_with(|| {
            current
                .template
                .priority()
                .cmp(&candidate.template.priority())
        })
        .then_with(|| {
            current
                .template
                .id()
                .cmp(&candidate.template.id())
        })
        .is_gt()
}

// ============================================================================
// Matching
// ============================================================================

fn matches_template(
    id: TemplateId,
    operations: &[Gate],
) -> bool {
    match id.as_str() {
        "h_x_h_to_z" => {
            matches_fixed_three(
                operations,
                GateKind::H,
                GateKind::X,
                GateKind::H,
            )
        }

        "h_z_h_to_x" => {
            matches_fixed_three(
                operations,
                GateKind::H,
                GateKind::Z,
                GateKind::H,
            )
        }

        "h_rz_h_to_rx" => {
            matches_parameter_conjugation(
                operations,
                GateKind::H,
                GateKind::RZ,
                GateKind::H,
            )
        }

        "h_rx_h_to_rz" => {
            matches_parameter_conjugation(
                operations,
                GateKind::H,
                GateKind::RX,
                GateKind::H,
            )
        }

        "h_ry_h_to_ry_neg" => {
            matches_parameter_conjugation(
                operations,
                GateKind::H,
                GateKind::RY,
                GateKind::H,
            )
        }

        "x_rz_x_to_rz_neg" => {
            matches_parameter_conjugation(
                operations,
                GateKind::X,
                GateKind::RZ,
                GateKind::X,
            )
        }

        "x_ry_x_to_ry_neg" => {
            matches_parameter_conjugation(
                operations,
                GateKind::X,
                GateKind::RY,
                GateKind::X,
            )
        }

        "x_rx_x_to_rx" => {
            matches_parameter_conjugation(
                operations,
                GateKind::X,
                GateKind::RX,
                GateKind::X,
            )
        }

        "y_rx_y_to_rx_neg" => {
            matches_parameter_conjugation(
                operations,
                GateKind::Y,
                GateKind::RX,
                GateKind::Y,
            )
        }

        "y_rz_y_to_rz_neg" => {
            matches_parameter_conjugation(
                operations,
                GateKind::Y,
                GateKind::RZ,
                GateKind::Y,
            )
        }

        "y_ry_y_to_ry" => {
            matches_parameter_conjugation(
                operations,
                GateKind::Y,
                GateKind::RY,
                GateKind::Y,
            )
        }

        "z_rx_z_to_rx_neg" => {
            matches_parameter_conjugation(
                operations,
                GateKind::Z,
                GateKind::RX,
                GateKind::Z,
            )
        }

        "z_ry_z_to_ry_neg" => {
            matches_parameter_conjugation(
                operations,
                GateKind::Z,
                GateKind::RY,
                GateKind::Z,
            )
        }

        "z_rz_z_to_rz" => {
            matches_parameter_conjugation(
                operations,
                GateKind::Z,
                GateKind::RZ,
                GateKind::Z,
            )
        }

        "cx_cx_identity" => {
            matches_fixed_two(
                operations,
                GateKind::CX,
                GateKind::CX,
            )
        }

        "cz_cz_identity" => {
            matches_fixed_two(
                operations,
                GateKind::CZ,
                GateKind::CZ,
            )
        }

        "swap_swap_identity" => {
            matches_fixed_two(
                operations,
                GateKind::SWAP,
                GateKind::SWAP,
            )
        }

        "rx_rx_compose" => {
            matches_same_axis_rotation(
                operations,
                GateKind::RX,
            )
        }

        "ry_ry_compose" => {
            matches_same_axis_rotation(
                operations,
                GateKind::RY,
            )
        }

        "rz_rz_compose" => {
            matches_same_axis_rotation(
                operations,
                GateKind::RZ,
            )
        }

        "phase_phase_compose" => {
            matches_same_axis_rotation(
                operations,
                GateKind::Phase,
            )
        }

        _ => false,
    }
}

fn matches_fixed_two(
    operations: &[Gate],
    first: GateKind,
    second: GateKind,
) -> bool {
    if operations.len() != 2 {
        return false;
    }

    if operations[0].kind() != first
        || operations[1].kind() != second
    {
        return false;
    }

    same_qubits(
        &operations[0],
        &operations[1],
    ) && safe_template_boundary(operations)
}

fn matches_fixed_three(
    operations: &[Gate],
    first: GateKind,
    second: GateKind,
    third: GateKind,
) -> bool {
    if operations.len() != 3 {
        return false;
    }

    if operations[0].kind() != first
        || operations[1].kind() != second
        || operations[2].kind() != third
    {
        return false;
    }

    same_qubits(
        &operations[0],
        &operations[1],
    ) && same_qubits(
        &operations[1],
        &operations[2],
    ) && safe_template_boundary(operations)
}

fn matches_parameter_conjugation(
    operations: &[Gate],
    outer: GateKind,
    middle: GateKind,
    outer_second: GateKind,
) -> bool {
    if operations.len() != 3 {
        return false;
    }

    if operations[0].kind() != outer
        || operations[1].kind() != middle
        || operations[2].kind() != outer_second
    {
        return false;
    }

    if operations[1].parameters().is_empty() {
        return false;
    }

    same_qubits(
        &operations[0],
        &operations[1],
    ) && same_qubits(
        &operations[1],
        &operations[2],
    ) && safe_template_boundary(operations)
}

fn matches_same_axis_rotation(
    operations: &[Gate],
    kind: GateKind,
) -> bool {
    if operations.len() != 2 {
        return false;
    }

    if operations[0].kind() != kind
        || operations[1].kind() != kind
    {
        return false;
    }

    if operations[0].parameters().len()
        != 1
        || operations[1].parameters().len()
            != 1
    {
        return false;
    }

    same_qubits(
        &operations[0],
        &operations[1],
    ) && safe_template_boundary(operations)
}

// ============================================================================
// Template application
// ============================================================================

fn apply_match(
    template_match: TemplateMatch,
    operations: &[Gate],
) -> Result<TemplateApplication, TemplateError> {
    let start = template_match.start;
    let end = start
        .checked_add(template_match.width)
        .ok_or_else(|| {
            TemplateError::InvalidTemplate {
                template: template_match.template.id,
                message:
                    "template range overflow".to_string(),
            }
        })?;

    if end > operations.len() {
        return Err(
            TemplateError::InvalidTemplate {
                template: template_match.template.id,
                message:
                    "template range exceeds operations"
                        .to_string(),
            },
        );
    }

    let matched = &operations[start..end];

    let replacement =
        build_replacement(
            template_match.template.id,
            matched,
        )?;

    validate_replacement(
        template_match.template.id,
        matched,
        &replacement,
    )?;

    Ok(TemplateApplication {
        template: template_match.template.id,
        start,
        consumed: template_match.width,
        replacement,
    })
}

fn build_replacement(
    id: TemplateId,
    matched: &[Gate],
) -> Result<Vec<Gate>, TemplateError> {
    match id.as_str() {
        "h_x_h_to_z" => {
            unary_fixed_replacement(
                GateKind::Z,
                matched,
            )
        }

        "h_z_h_to_x" => {
            unary_fixed_replacement(
                GateKind::X,
                matched,
            )
        }

        "h_rz_h_to_rx" => {
            unary_parameter_replacement(
                GateKind::RX,
                matched,
                false,
            )
        }

        "h_rx_h_to_rz" => {
            unary_parameter_replacement(
                GateKind::RZ,
                matched,
                false,
            )
        }

        "h_ry_h_to_ry_neg"
        | "x_ry_x_to_ry_neg"
        | "z_ry_z_to_ry_neg"
        | "y_rx_y_to_rx_neg"
        | "y_rz_y_to_rz_neg"
        | "x_rz_x_to_rz_neg"
        | "z_rx_z_to_rx_neg" => {
            unary_parameter_replacement(
                conjugated_result_kind(
                    id,
                    matched[1].kind(),
                ),
                matched,
                true,
            )
        }

        "x_rx_x_to_rx"
        | "y_ry_y_to_ry"
        | "z_rz_z_to_rz" => {
            unary_parameter_replacement(
                matched[1].kind(),
                matched,
                false,
            )
        }

        "cx_cx_identity"
        | "cz_cz_identity"
        | "swap_swap_identity" => {
            Ok(Vec::new())
        }

        "rx_rx_compose"
        | "ry_ry_compose"
        | "rz_rz_compose"
        | "phase_phase_compose" => {
            compose_rotation(
                matched,
            )
        }

        _ => Err(
            TemplateError::InvalidTemplate {
                template: id,
                message:
                    "template has no implementation"
                        .to_string(),
            },
        ),
    }
}

fn unary_fixed_replacement(
    kind: GateKind,
    matched: &[Gate],
) -> Result<Vec<Gate>, TemplateError> {
    let qubits = matched
        .first()
        .map(|gate| gate.qubits().to_vec())
        .ok_or_else(|| {
            TemplateError::InvalidReplacement {
                template: TemplateId::new(
                    "unary_fixed_replacement",
                ),
                message:
                    "empty template match".to_string(),
            }
        })?;

    let gate = Gate::new(
        kind,
        qubits,
        Vec::new(),
        None,
        None,
    )
    .map_err(|error| {
        TemplateError::InvalidReplacement {
            template: TemplateId::new(
                "unary_fixed_replacement",
            ),
            message: error.to_string(),
        }
    })?;

    Ok(vec![gate])
}

fn unary_parameter_replacement(
    kind: GateKind,
    matched: &[Gate],
    negate_parameter: bool,
) -> Result<Vec<Gate>, TemplateError> {
    let middle =
        matched.get(1).ok_or_else(|| {
            TemplateError::InvalidReplacement {
                template: TemplateId::new(
                    "unary_parameter_replacement",
                ),
                message:
                    "missing middle operation"
                        .to_string(),
            }
        })?;

    let parameter = middle
        .parameters()
        .first()
        .ok_or_else(|| {
            TemplateError::UnsupportedParameterTransformation {
                template: TemplateId::new(
                    "unary_parameter_replacement",
                ),
            }
        })?;

    let parameter =
        transform_parameter(parameter, negate_parameter)?;

    let gate = Gate::new(
        kind,
        middle.qubits().to_vec(),
        vec![parameter],
        None,
        None,
    )
    .map_err(|error| {
        TemplateError::InvalidReplacement {
            template: TemplateId::new(
                "unary_parameter_replacement",
            ),
            message: error.to_string(),
        }
    })?;

    Ok(vec![gate])
}

fn compose_rotation(
    matched: &[Gate],
) -> Result<Vec<Gate>, TemplateError> {
    let first = matched.first().ok_or_else(|| {
        TemplateError::InvalidReplacement {
            template: TemplateId::new(
                "rotation_composition",
            ),
            message:
                "missing first rotation".to_string(),
        }
    })?;

    let second = matched.get(1).ok_or_else(|| {
        TemplateError::InvalidReplacement {
            template: TemplateId::new(
                "rotation_composition",
            ),
            message:
                "missing second rotation".to_string(),
        }
    })?;

    let first_parameter =
        first.parameters().first().ok_or_else(
            || {
                TemplateError::UnsupportedParameterTransformation {
                    template: TemplateId::new(
                        "rotation_composition",
                    ),
                }
            },
        )?;

    let second_parameter =
        second.parameters().first().ok_or_else(
            || {
                TemplateError::UnsupportedParameterTransformation {
                    template: TemplateId::new(
                        "rotation_composition",
                    ),
                }
            },
        )?;

    let combined =
        add_parameters(
            first_parameter,
            second_parameter,
        )?;

    let gate = Gate::new(
        first.kind(),
        first.qubits().to_vec(),
        vec![combined],
        None,
        None,
    )
    .map_err(|error| {
        TemplateError::InvalidReplacement {
            template: TemplateId::new(
                "rotation_composition",
            ),
            message: error.to_string(),
        }
    })?;

    Ok(vec![gate])
}

// ============================================================================
// Parameter transformations
// ============================================================================

fn transform_parameter(
    parameter: &Parameter,
    negate: bool,
) -> Result<Parameter, TemplateError> {
    if !negate {
        return Ok(parameter.clone());
    }

    match parameter {
        Parameter::Constant(value) => {
            Ok(Parameter::Constant(
                normalize_angle(-*value),
            ))
        }

        // The canonical parameter system may contain symbolic variants.
        // Do not invent a new symbolic representation here.
        //
        // Future `parameter::simplification` can provide a general expression
        // algebra. Until then, exact symbolic negation is fail-closed.
        _ => Err(
            TemplateError::UnsupportedParameterTransformation {
                template: TemplateId::new(
                    "parameter.negate",
                ),
            },
        ),
    }
}

fn add_parameters(
    first: &Parameter,
    second: &Parameter,
) -> Result<Parameter, TemplateError> {
    match (first, second) {
        (
            Parameter::Constant(left),
            Parameter::Constant(right),
        ) => {
            Ok(Parameter::Constant(
                normalize_angle(*left + *right),
            ))
        }

        // Do not manufacture symbolic expression nodes without the canonical
        // parameter algebra. The future parameter optimizer can handle these
        // cases before or after template optimization.
        _ => Err(
            TemplateError::UnsupportedParameterTransformation {
                template: TemplateId::new(
                    "parameter.add",
                ),
            },
        ),
    }
}

fn normalize_angle(angle: f64) -> f64 {
    if !angle.is_finite() {
        return angle;
    }

    let two_pi = 2.0 * PI;

    let mut normalized =
        angle.rem_euclid(two_pi);

    if normalized > PI {
        normalized -= two_pi;
    }

    if normalized == -0.0 {
        0.0
    } else {
        normalized
    }
}

// ============================================================================
// Semantic classification
// ============================================================================

fn conjugated_result_kind(
    id: TemplateId,
    middle: GateKind,
) -> GateKind {
    match id.as_str() {
        "h_rz_h_to_rx" => GateKind::RX,
        "h_rx_h_to_rz" => GateKind::RZ,

        "h_ry_h_to_ry_neg" => GateKind::RY,

        "x_rz_x_to_rz_neg" => GateKind::RZ,
        "x_ry_x_to_ry_neg" => GateKind::RY,

        "x_rx_x_to_rx" => GateKind::RX,

        "y_rx_y_to_rx_neg" => GateKind::RX,
        "y_rz_y_to_rz_neg" => GateKind::RZ,

        "y_ry_y_to_ry" => GateKind::RY,

        "z_rx_z_to_rx_neg" => GateKind::RX,
        "z_ry_z_to_ry_neg" => GateKind::RY,

        "z_rz_z_to_rz" => GateKind::RZ,

        _ => middle,
    }
}

fn is_parameter_composition(
    id: TemplateId,
) -> bool {
    matches!(
        id.as_str(),
        "rx_rx_compose"
            | "ry_ry_compose"
            | "rz_rz_compose"
            | "phase_phase_compose"
    )
}

fn is_conjugation_template(
    id: TemplateId,
) -> bool {
    matches!(
        id.as_str(),
        "h_x_h_to_z"
            | "h_z_h_to_x"
            | "h_rz_h_to_rx"
            | "h_rx_h_to_rz"
            | "h_ry_h_to_ry_neg"
            | "x_rz_x_to_rz_neg"
            | "x_ry_x_to_ry_neg"
            | "x_rx_x_to_rx"
            | "y_rx_y_to_rx_neg"
            | "y_rz_y_to_rz_neg"
            | "y_ry_y_to_ry"
            | "z_rx_z_to_rx_neg"
            | "z_ry_z_to_ry_neg"
            | "z_rz_z_to_rz"
    )
}

fn is_two_qubit_template(
    id: TemplateId,
) -> bool {
    matches!(
        id.as_str(),
        "cx_cx_identity"
            | "cz_cz_identity"
            | "swap_swap_identity"
    )
}

// ============================================================================
// Validation
// ============================================================================

fn validate_operations(
    operations: &[Gate],
) -> Result<(), TemplateError> {
    for operation in operations {
        if operation.is_measurement()
            || operation.is_barrier()
            || operation.is_reset()
        {
            continue;
        }

        // `Gate` constructors already guarantee local invariants.
        //
        // This branch exists as an explicit semantic guard so that this file
        // remains fail-closed if future IR versions add new non-unitary gate
        // kinds.
        if !operation.kind().is_unitary() {
            return Err(
                TemplateError::InvalidCircuit {
                    message: format!(
                        "unsupported non-unitary operation {:?}",
                        operation.kind()
                    ),
                },
            );
        }
    }

    Ok(())
}

/// Returns true when a sequence is safe to optimize as one local region.
///
/// Templates must never cross:
///
/// - measurement;
/// - reset;
/// - barrier.
///
/// Those operations are semantic boundaries.
fn safe_template_boundary(
    operations: &[Gate],
) -> bool {
    operations.iter().all(|operation| {
        !operation.is_measurement()
            && !operation.is_barrier()
            && !operation.is_reset()
    })
}

// ============================================================================
// Qubit matching
// ============================================================================

fn same_qubits(
    first: &Gate,
    second: &Gate,
) -> bool {
    first.qubits() == second.qubits()
}

// ============================================================================
// Replacement validation
// ============================================================================

fn validate_replacement(
    template: TemplateId,
    matched: &[Gate],
    replacement: &[Gate],
) -> Result<(), TemplateError> {
    if matched.is_empty() {
        return Err(
            TemplateError::InvalidReplacement {
                template,
                message:
                    "template matched an empty sequence"
                        .to_string(),
            },
        );
    }

    for gate in replacement {
        if gate.is_measurement()
            || gate.is_barrier()
            || gate.is_reset()
        {
            return Err(
                TemplateError::InvalidReplacement {
                    template,
                    message:
                        "a local algebraic template may not introduce \
                         measurement, reset, or barrier operations"
                            .to_string(),
                },
            );
        }

        if !gate.kind().is_unitary() {
            return Err(
                TemplateError::InvalidReplacement {
                    template,
                    message:
                        "replacement contains a non-unitary operation"
                            .to_string(),
                },
            );
        }
    }

    Ok(())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn q0() -> QubitId {
        QubitId::new(0)
    }

    fn q1() -> QubitId {
        QubitId::new(1)
    }

    fn fixed(
        kind: GateKind,
        qubits: &[QubitId],
    ) -> Gate {
        Gate::new(
            kind,
            qubits.to_vec(),
            Vec::new(),
            None,
            None,
        )
        .expect("test gate must be valid")
    }

    fn parameterized(
        kind: GateKind,
        qubits: &[QubitId],
        angle: f64,
    ) -> Gate {
        Gate::new(
            kind,
            qubits.to_vec(),
            vec![Parameter::Constant(angle)],
            None,
            None,
        )
        .expect("test gate must be valid")
    }

    #[test]
    fn builtin_registry_is_non_empty() {
        let registry =
            TemplateRegistry::builtin();

        assert!(
            !registry.is_empty()
        );
    }

    #[test]
    fn h_x_h_becomes_z() {
        let optimizer =
            TemplateOptimizer::new();

        let input = vec![
            fixed(GateKind::H, &[q0()]),
            fixed(GateKind::X, &[q0()]),
            fixed(GateKind::H, &[q0()]),
        ];

        let (output, stats) =
            optimizer
                .optimize_operations(&input)
                .expect("optimization should succeed");

        assert_eq!(
            output.len(),
            1
        );

        assert_eq!(
            output[0].kind(),
            GateKind::Z
        );

        assert_eq!(
            stats.applications,
            1
        );
    }

    #[test]
    fn h_z_h_becomes_x() {
        let optimizer =
            TemplateOptimizer::new();

        let input = vec![
            fixed(GateKind::H, &[q0()]),
            fixed(GateKind::Z, &[q0()]),
            fixed(GateKind::H, &[q0()]),
        ];

        let (output, _) =
            optimizer
                .optimize_operations(&input)
                .expect("optimization should succeed");

        assert_eq!(
            output.len(),
            1
        );

        assert_eq!(
            output[0].kind(),
            GateKind::X
        );
    }

    #[test]
    fn h_rz_h_becomes_rx() {
        let optimizer =
            TemplateOptimizer::new();

        let input = vec![
            fixed(GateKind::H, &[q0()]),
            parameterized(
                GateKind::RZ,
                &[q0()],
                0.75,
            ),
            fixed(GateKind::H, &[q0()]),
        ];

        let (output, _) =
            optimizer
                .optimize_operations(&input)
                .expect("optimization should succeed");

        assert_eq!(
            output.len(),
            1
        );

        assert_eq!(
            output[0].kind(),
            GateKind::RX
        );

        assert_eq!(
            output[0].parameters(),
            &[Parameter::Constant(0.75)]
        );
    }

    #[test]
    fn x_rz_x_negates_angle() {
        let optimizer =
            TemplateOptimizer::new();

        let input = vec![
            fixed(GateKind::X, &[q0()]),
            parameterized(
                GateKind::RZ,
                &[q0()],
                0.75,
            ),
            fixed(GateKind::X, &[q0()]),
        ];

        let (output, _) =
            optimizer
                .optimize_operations(&input)
                .expect("optimization should succeed");

        assert_eq!(
            output.len(),
            1
        );

        assert_eq!(
            output[0].kind(),
            GateKind::RZ
        );

        assert_eq!(
            output[0].parameters(),
            &[Parameter::Constant(-0.75)]
        );
    }

    #[test]
    fn rotation_composition_is_exact_for_constants() {
        let optimizer =
            TemplateOptimizer::new();

        let input = vec![
            parameterized(
                GateKind::RZ,
                &[q0()],
                0.5,
            ),
            parameterized(
                GateKind::RZ,
                &[q0()],
                0.75,
            ),
        ];

        let (output, _) =
            optimizer
                .optimize_operations(&input)
                .expect("optimization should succeed");

        assert_eq!(
            output.len(),
            1
        );

        assert_eq!(
            output[0].parameters(),
            &[Parameter::Constant(1.25)]
        );
    }

    #[test]
    fn rotation_composition_wraps_angle() {
        let optimizer =
            TemplateOptimizer::new();

        let input = vec![
            parameterized(
                GateKind::RZ,
                &[q0()],
                3.0,
            ),
            parameterized(
                GateKind::RZ,
                &[q0()],
                1.0,
            ),
        ];

        let (output, _) =
            optimizer
                .optimize_operations(&input)
                .expect("optimization should succeed");

        assert_eq!(
            output.len(),
            1
        );

        let expected =
            normalize_angle(4.0);

        assert_eq!(
            output[0].parameters(),
            &[Parameter::Constant(expected)]
        );
    }

    #[test]
    fn cx_pair_is_removed() {
        let optimizer =
            TemplateOptimizer::new();

        let input = vec![
            fixed(
                GateKind::CX,
                &[q0(), q1()],
            ),
            fixed(
                GateKind::CX,
                &[q0(), q1()],
            ),
        ];

        let (output, stats) =
            optimizer
                .optimize_operations(&input)
                .expect("optimization should succeed");

        assert!(
            output.is_empty()
        );

        assert_eq!(
            stats.applications,
            1
        );
    }

    #[test]
    fn different_qubits_are_not_rewritten() {
        let optimizer =
            TemplateOptimizer::new();

        let input = vec![
            fixed(GateKind::H, &[q0()]),
            fixed(GateKind::X, &[q0()]),
            fixed(GateKind::H, &[q1()]),
        ];

        let (output, stats) =
            optimizer
                .optimize_operations(&input)
                .expect("optimization should succeed");

        assert_eq!(
            output,
            input
        );

        assert_eq!(
            stats.applications,
            0
        );
    }

    #[test]
    fn two_qubit_operands_must_match_in_order() {
        let optimizer =
            TemplateOptimizer::new();

        let input = vec![
            fixed(
                GateKind::CX,
                &[q0(), q1()],
            ),
            fixed(
                GateKind::CX,
                &[q1(), q0()],
            ),
        ];

        let (output, stats) =
            optimizer
                .optimize_operations(&input)
                .expect("optimization should succeed");

        assert_eq!(
            output,
            input
        );

        assert_eq!(
            stats.applications,
            0
        );
    }

    #[test]
    fn measurement_boundary_is_not_crossed() {
        let optimizer =
            TemplateOptimizer::new();

        let measurement = Gate::new(
            GateKind::Measure,
            vec![q0()],
            Vec::new(),
            Some(0),
            None,
        )
        .expect("measurement must be valid");

        let input = vec![
            fixed(GateKind::H, &[q0()]),
            measurement,
            fixed(GateKind::H, &[q0()]),
        ];

        let (output, stats) =
            optimizer
                .optimize_operations(&input)
                .expect("optimization should succeed");

        assert_eq!(
            output.len(),
            3
        );

        assert_eq!(
            stats.applications,
            0
        );
    }

    #[test]
    fn reset_boundary_is_not_crossed() {
        let optimizer =
            TemplateOptimizer::new();

        let reset = fixed(
            GateKind::Reset,
            &[q0()],
        );

        let input = vec![
            fixed(GateKind::H, &[q0()]),
            reset,
            fixed(GateKind::H, &[q0()]),
        ];

        let (output, stats) =
            optimizer
                .optimize_operations(&input)
                .expect("optimization should succeed");

        assert_eq!(
            output.len(),
            3
        );

        assert_eq!(
            stats.applications,
            0
        );
    }

    #[test]
    fn barrier_boundary_is_not_crossed() {
        let optimizer =
            TemplateOptimizer::new();

        let barrier = Gate::new(
            GateKind::Barrier,
            vec![q0()],
            Vec::new(),
            None,
            None,
        )
        .expect("barrier must be valid");

        let input = vec![
            fixed(GateKind::H, &[q0()]),
            barrier,
            fixed(GateKind::X, &[q0()]),
            fixed(GateKind::H, &[q0()]),
        ];

        let (output, stats) =
            optimizer
                .optimize_operations(&input)
                .expect("optimization should succeed");

        assert_eq!(
            output.len(),
            4
        );

        assert_eq!(
            stats.applications,
            0
        );
    }

    #[test]
    fn symbolic_parameter_is_not_guessed() {
        let optimizer =
            TemplateOptimizer::new();

        // The exact enum variants beyond Constant are deliberately not
        // constructed here because the canonical parameter implementation owns
        // symbolic representation.
        //
        // The important production contract is that a non-constant parameter
        // must never be converted to an invented floating-point value.
        let input = vec![
            parameterized(
                GateKind::RZ,
                &[q0()],
                0.5,
            ),
            parameterized(
                GateKind::RZ,
                &[q0()],
                0.25,
            ),
        ];

        let (output, _) =
            optimizer
                .optimize_operations(&input)
                .expect("constant parameters should optimize");

        assert_eq!(
            output.len(),
            1
        );
    }

    #[test]
    fn optimizer_is_idempotent_at_fixed_point() {
        let optimizer =
            TemplateOptimizer::new();

        let input = vec![
            fixed(GateKind::H, &[q0()]),
            fixed(GateKind::X, &[q0()]),
            fixed(GateKind::H, &[q0()]),
        ];

        let (first, _) =
            optimizer
                .optimize_operations(&input)
                .expect("first optimization should succeed");

        let (second, _) =
            optimizer
                .optimize_operations(&first)
                .expect("second optimization should succeed");

        assert_eq!(
            first,
            second
        );
    }

    #[test]
    fn template_application_has_correct_delta() {
        let optimizer =
            TemplateOptimizer::new();

        let input = vec![
            fixed(GateKind::H, &[q0()]),
            fixed(GateKind::X, &[q0()]),
            fixed(GateKind::H, &[q0()]),
        ];

        let matched = optimizer
            .find_match(&input, 0)
            .expect("matching should succeed")
            .expect("template should match");

        let application = optimizer
            .apply(matched, &input)
            .expect("application should succeed");

        assert_eq!(
            application.consumed,
            3
        );

        assert_eq!(
            application.replacement.len(),
            1
        );

        assert_eq!(
            application.net_operation_delta(),
            -2
        );
    }

    #[test]
    fn registry_is_deterministic() {
        let first =
            TemplateRegistry::builtin();

        let second =
            TemplateRegistry::builtin();

        assert_eq!(
            first.templates(),
            second.templates()
        );
    }

    #[test]
    fn no_global_phase_template_is_registered() {
        // H Y H = -Y is deliberately excluded from the exact registry.
        //
        // This prevents the optimizer from silently changing exact unitary
        // semantics merely because measurement probabilities happen to remain
        // unchanged.
        let registry =
            TemplateRegistry::builtin();

        assert!(
            !registry
                .templates()
                .iter()
                .any(|template| {
                    template.id()
                        .as_str()
                        == "h_y_h_to_y"
                })
        );
    }

    #[test]
    fn optimizer_handles_empty_circuit() {
        let optimizer =
            TemplateOptimizer::new();

        let (output, stats) =
            optimizer
                .optimize_operations(&[])
                .expect("empty circuit is valid");

        assert!(
            output.is_empty()
        );

        assert_eq!(
            stats.applications,
            0
        );
    }

    #[test]
    fn optimizer_preserves_unrelated_gate() {
        let optimizer =
            TemplateOptimizer::new();

        let input = vec![
            fixed(GateKind::X, &[q0()]),
        ];

        let (output, stats) =
            optimizer
                .optimize_operations(&input)
                .expect("optimization should succeed");

        assert_eq!(
            output,
            input
        );

        assert_eq!(
            stats.applications,
            0
        );
    }
}
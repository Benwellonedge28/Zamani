
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Stable, extensible diagnostic identifiers for Zamani semantic analysis.
//!
//! File: src/frontend/semantic/diagnostics/codes.rs
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # PURPOSE
//!
//! Define the canonical identity and classification of semantic diagnostic
//! codes. This module is independent of AST implementations, source-span
//! representations, diagnostic rendering, compiler backends, and hardware.
//!
//! # AUTHORITY
//!
//! This module implements diagnostic identity contracts for the semantic
//! subsystem. Language rules remain governed by the repository's normative
//! specifications and canonical grammar. This module does not decide whether
//! a source construct is semantically valid.
//!
//! # OWNS
//!
//! - Stable built-in diagnostic identifiers.
//! - Open-world diagnostic-code representation.
//! - Diagnostic category and severity classifications.
//! - Built-in code metadata and lookup.
//! - Validation of extension-defined diagnostic identifiers.
//!
//! # DOES NOT OWN
//!
//! - Complete diagnostic records, messages, labels, or source spans.
//! - AST nodes, node identities, or source maps.
//! - Name resolution, type checking, effects, capabilities, resources,
//!   contracts, policies, or domain-specific validation.
//! - Parser or lexer diagnostics.
//! - Target discovery, hardware feasibility, scheduling, or IR generation.
//! - Diagnostic ordering, collection, rendering, or localization.
//!
//! # INTEGRATION CONTRACT
//!
//! `diagnostics/diagnostic.rs` should store `DiagnosticCode` and obtain
//! classification through `DiagnosticCode::category()` and
//! `DiagnosticCode::default_severity()`.
//!
//! `diagnostics/mod.rs` should re-export the supported public types.
//!
//! Diagnostic producers should use the built-in constants below whenever
//! an existing code describes the issue. Extensions may register their own
//! identifiers without changing this file.
//!
//! Source spans and labels belong to the canonical AST/source infrastructure.
//! This module intentionally does not import an AST span type.
//!
//! # STABILITY AND EXTENSIBILITY
//!
//! Built-in identifiers are stable API values. Do not recycle an identifier
//! for a different meaning. Retire codes through a documented compatibility
//! policy rather than silently repurposing them.
//!
//! Codes use an uppercase, hyphen-separated form, for example:
//!
//! `ZM-SEM-NAME-001`
//! `ZM-SEM-TYPE-001`
//! `ACME-SEM-RESOURCE-001`
//!
//! Extension codes use the same validated syntax. The core does not require
//! an extension namespace to be added to a central enumeration.
//!
//! Categories are intentionally broader than individual diagnostic codes.
//! A new code within an existing category requires no core API change.
//! Extensions with genuinely new categories can use `Extension`.
//!
//! # POCO-REAF AND SCALABILITY
//!
//! Diagnostic identity is independent of the number of CPUs, GPUs, qubits,
//! registers, devices, nodes, tensor dimensions, or computational domains.
//! No hardware-capacity ceiling is defined here.
//!
//! This module cannot make physical resources infinite. It introduces no
//! artificial limit on the size of a source program or the amount of
//! computation a program describes.
//!
//! # ERROR HANDLING
//!
//! Invalid extension identifiers are rejected explicitly. They are never
//! silently normalized, truncated, or substituted with a different code.
//!
//! Unknown but valid identifiers remain representable. An unknown code is
//! not automatically evidence that a program is invalid; the diagnostic
//! producer determines the issue, while this module classifies its code.
//!
//! # FREEZE CRITERIA
//!
//! - Built-in identifiers are unique and stable.
//! - Extension identifiers are validated and remain open-world.
//! - Category and severity classification is deterministic.
//! - Unknown valid codes are preserved.
//! - Invalid identifiers fail without panicking.
//! - Unit tests pass on the supported Rust toolchain.
//! - No unsafe code or hardware-specific limits are introduced.

use core::borrow::Borrow;
use core::fmt;
use std::borrow::Cow;
use std::error::Error;
use std::str::FromStr;

/// Broad, stable classification of a diagnostic.
///
/// Categories group related diagnostics for tooling and policy. They are not
/// a closed list of every language feature or computational domain.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum DiagnosticCategory {
    /// Invalid or unsupported source structure discovered by semantic checks.
    Structural,

    /// Name lookup, declarations, imports, scopes, or symbol conflicts.
    NameResolution,

    /// Types, inference, conversions, generics, or type constraints.
    Type,

    /// Ownership, borrowing, lifetimes, or value-use restrictions.
    Ownership,

    /// Effect inference, declarations, bounds, or effect restrictions.
    Effect,

    /// Capability declarations, requirements, or evidence.
    Capability,

    /// Resource expressions, units, requirements, or resource constraints.
    Resource,

    /// Preconditions, postconditions, invariants, or contract obligations.
    Contract,

    /// Authorization, policy, or permission violations.
    Policy,

    /// Provenance, evidence, or required traceability is invalid or missing.
    Provenance,

    /// Domain-neutral or domain-specific semantic validation.
    Domain,

    /// Extension registration, compatibility, or metadata problems.
    Extension,

    /// Constant evaluation, symbolic expressions, or constraint solving.
    Evaluation,

    /// Analysis was incomplete or a configured operational budget was reached.
    Analysis,

    /// Compiler or semantic implementation faults.
    Internal,

    /// A category supplied by an extension rather than the core vocabulary.
    ExtensionCategory(String),
}

impl DiagnosticCategory {
    /// Stable machine-readable category identifier.
    ///
    /// Extension category identifiers are owned by their extension provider.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Structural => "structural",
            Self::NameResolution => "name-resolution",
            Self::Type => "type",
            Self::Ownership => "ownership",
            Self::Effect => "effect",
            Self::Capability => "capability",
            Self::Resource => "resource",
            Self::Contract => "contract",
            Self::Policy => "policy",
            Self::Provenance => "provenance",
            Self::Domain => "domain",
            Self::Extension => "extension",
            Self::Evaluation => "evaluation",
            Self::Analysis => "analysis",
            Self::Internal => "internal",
            Self::ExtensionCategory(value) => value,
        }
    }

    /// Construct an extension-defined category.
    ///
    /// The identifier must be non-empty and contain only lowercase ASCII
    /// letters, digits, hyphens, or dots. Leading/trailing separators and
    /// repeated separators are rejected to keep identifiers unambiguous.
    pub fn extension(value: impl Into<String>) -> Result<Self, CategoryError> {
        let value = value.into();

        if value.is_empty() {
            return Err(CategoryError::Empty);
        }

        if value.starts_with(['-', '.']) || value.ends_with(['-', '.']) {
            return Err(CategoryError::InvalidSeparator);
        }

        let mut previous_separator = false;

        for byte in value.bytes() {
            let separator = matches!(byte, b'-' | b'.');

            if !(byte.is_ascii_lowercase() || byte.is_ascii_digit() || separator) {
                return Err(CategoryError::InvalidCharacter(byte));
            }

            if separator && previous_separator {
                return Err(CategoryError::InvalidSeparator);
            }

            previous_separator = separator;
        }

        Ok(Self::ExtensionCategory(value))
    }
}

/// Failure while constructing an extension category.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CategoryError {
    /// The category identifier was empty.
    Empty,
    /// A character was not allowed in a category identifier.
    InvalidCharacter(u8),
    /// A separator was leading, trailing, or repeated.
    InvalidSeparator,
}

impl fmt::Display for CategoryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("diagnostic category must not be empty"),
            Self::InvalidCharacter(byte) => {
                write!(formatter, "invalid byte in diagnostic category: {byte:#04x}")
            }
            Self::InvalidSeparator => {
                formatter.write_str("invalid separator in diagnostic category")
            }
        }
    }
}

impl Error for CategoryError {}

/// Default diagnostic severity.
///
/// This is a classification default, not a substitute for a diagnostic
/// producer's explicit severity decision. For example, an unresolved name
/// may be downgraded by a tooling mode only when that policy is explicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum DiagnosticSeverity {
    /// Additional information that does not indicate failure.
    Note,
    /// A potential issue that does not necessarily invalidate the program.
    Warning,
    /// A language or semantic rule has been violated.
    Error,
    /// The analysis infrastructure itself failed.
    Fatal,
}

/// A validated, open-world diagnostic identifier.
///
/// Built-in codes are provided as associated constants. Extension-defined
/// codes are owned strings and do not require editing this module.
///
/// Equality, hashing, and ordering compare the exact code spelling. Codes
/// are never silently case-folded or normalized.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DiagnosticCode(Cow<'static, str>);

impl DiagnosticCode {
    // ---------------------------------------------------------------------
    // Structural and analysis integrity
    // ---------------------------------------------------------------------

    /// A semantic input violates a required structural invariant.
    pub const STRUCTURE_INVALID: Self =
        Self(Cow::Borrowed("ZM-SEM-STRUCTURE-001"));

    /// A required AST construct is not supported by this semantic version.
    pub const FEATURE_UNSUPPORTED: Self =
        Self(Cow::Borrowed("ZM-SEM-FEATURE-001"));

    /// The analysis result is incomplete and must not be treated as valid.
    pub const ANALYSIS_INCOMPLETE: Self =
        Self(Cow::Borrowed("ZM-SEM-ANALYSIS-001"));

    /// A configured operational analysis budget was exhausted.
    pub const ANALYSIS_BUDGET_EXHAUSTED: Self =
        Self(Cow::Borrowed("ZM-SEM-ANALYSIS-002"));

    /// The semantic input or its dependencies changed during an operation.
    pub const ANALYSIS_INPUT_CHANGED: Self =
        Self(Cow::Borrowed("ZM-SEM-ANALYSIS-003"));

    // ---------------------------------------------------------------------
    // Declarations and name resolution
    // ---------------------------------------------------------------------

    /// An identifier or declaration path cannot be resolved.
    pub const NAME_UNRESOLVED: Self =
        Self(Cow::Borrowed("ZM-SEM-NAME-001"));

    /// A declaration conflicts with another declaration in its scope.
    pub const NAME_DUPLICATE: Self =
        Self(Cow::Borrowed("ZM-SEM-NAME-002"));

    /// A declaration is referenced outside its permitted visibility.
    pub const NAME_INACCESSIBLE: Self =
        Self(Cow::Borrowed("ZM-SEM-NAME-003"));

    /// An import or module path cannot be resolved.
    pub const IMPORT_UNRESOLVED: Self =
        Self(Cow::Borrowed("ZM-SEM-IMPORT-001"));

    /// An import or module dependency forms a prohibited cycle.
    pub const IMPORT_CYCLE: Self =
        Self(Cow::Borrowed("ZM-SEM-IMPORT-002"));

    /// A declaration or type dependency forms an illegal cycle.
    pub const DECLARATION_CYCLE: Self =
        Self(Cow::Borrowed("ZM-SEM-DECL-001"));

    /// A required declaration is missing.
    pub const DECLARATION_MISSING: Self =
        Self(Cow::Borrowed("ZM-SEM-DECL-002"));

    /// A declaration violates its required form or semantic rules.
    pub const DECLARATION_INVALID: Self =
        Self(Cow::Borrowed("ZM-SEM-DECL-003"));

    // ---------------------------------------------------------------------
    // Types, inference, generics, and value use
    // ---------------------------------------------------------------------

    /// A value or expression has an invalid type.
    pub const TYPE_MISMATCH: Self =
        Self(Cow::Borrowed("ZM-SEM-TYPE-001"));

    /// The type of a value or expression cannot be determined.
    pub const TYPE_UNRESOLVED: Self =
        Self(Cow::Borrowed("ZM-SEM-TYPE-002"));

    /// A type expression cannot be resolved.
    pub const TYPE_UNKNOWN: Self =
        Self(Cow::Borrowed("ZM-SEM-TYPE-003"));

    /// Type inference or unification produced conflicting constraints.
    pub const TYPE_CONSTRAINT_CONFLICT: Self =
        Self(Cow::Borrowed("ZM-SEM-TYPE-004"));

    /// A requested conversion is not permitted.
    pub const TYPE_COERCION_INVALID: Self =
        Self(Cow::Borrowed("ZM-SEM-TYPE-005"));

    /// A generic parameter or bound is invalid or unsatisfied.
    pub const GENERIC_CONSTRAINT_UNSATISFIED: Self =
        Self(Cow::Borrowed("ZM-SEM-GENERIC-001"));

    /// A required trait or interface implementation is missing.
    pub const TRAIT_IMPLEMENTATION_MISSING: Self =
        Self(Cow::Borrowed("ZM-SEM-TRAIT-001"));

    /// Trait or interface resolution is ambiguous.
    pub const TRAIT_RESOLUTION_AMBIGUOUS: Self =
        Self(Cow::Borrowed("ZM-SEM-TRAIT-002"));

    /// A constant expression cannot be evaluated under language rules.
    pub const CONSTANT_EVALUATION_FAILED: Self =
        Self(Cow::Borrowed("ZM-SEM-CONST-001"));

    /// Constant evaluation encountered arithmetic overflow.
    pub const CONSTANT_OVERFLOW: Self =
        Self(Cow::Borrowed("ZM-SEM-CONST-002"));

    /// A type or value constraint could not be solved.
    pub const CONSTRAINT_UNSOLVED: Self =
        Self(Cow::Borrowed("ZM-SEM-CONSTRAINT-001"));

    /// Ownership, borrowing, or lifetime rules were violated.
    pub const OWNERSHIP_VIOLATION: Self =
        Self(Cow::Borrowed("ZM-SEM-OWNERSHIP-001"));

    /// A value is used in a way prohibited by its linear/affine rules.
    pub const VALUE_USE_VIOLATION: Self =
        Self(Cow::Borrowed("ZM-SEM-VALUE-001"));

    // ---------------------------------------------------------------------
    // Effects and authorization
    // ---------------------------------------------------------------------

    /// An effect declaration or inferred effect set is invalid.
    pub const EFFECT_INVALID: Self =
        Self(Cow::Borrowed("ZM-SEM-EFFECT-001"));

    /// A computation performs an effect that its context prohibits.
    pub const EFFECT_VIOLATION: Self =
        Self(Cow::Borrowed("ZM-SEM-EFFECT-002"));

    /// A required effect is missing from a declaration or bound.
    pub const EFFECT_UNDECLARED: Self =
        Self(Cow::Borrowed("ZM-SEM-EFFECT-003"));

    /// An operation lacks the required authorization or permission.
    pub const AUTHORIZATION_DENIED: Self =
        Self(Cow::Borrowed("ZM-SEM-AUTH-001"));

    /// An operation violates a declared policy.
    pub const POLICY_VIOLATION: Self =
        Self(Cow::Borrowed("ZM-SEM-POLICY-001"));

    // ---------------------------------------------------------------------
    // Capabilities and resource requirements
    // ---------------------------------------------------------------------

    /// A capability requirement is malformed or semantically invalid.
    pub const CAPABILITY_REQUIREMENT_INVALID: Self =
        Self(Cow::Borrowed("ZM-SEM-CAPABILITY-001"));

    /// Required capability evidence is missing or insufficient.
    pub const CAPABILITY_UNSATISFIED: Self =
        Self(Cow::Borrowed("ZM-SEM-CAPABILITY-002"));

    /// Capability definitions conflict or cannot be resolved.
    pub const CAPABILITY_CONFLICT: Self =
        Self(Cow::Borrowed("ZM-SEM-CAPABILITY-003"));

    /// A resource requirement or resource expression is invalid.
    pub const RESOURCE_REQUIREMENT_INVALID: Self =
        Self(Cow::Borrowed("ZM-SEM-RESOURCE-001"));

    /// Resource quantities or units are incompatible.
    pub const RESOURCE_UNIT_MISMATCH: Self =
        Self(Cow::Borrowed("ZM-SEM-RESOURCE-002"));

    /// A resource constraint is contradictory or unsatisfiable.
    pub const RESOURCE_CONSTRAINT_UNSATISFIED: Self =
        Self(Cow::Borrowed("ZM-SEM-RESOURCE-003"));

    /// A resource expression cannot be resolved under current semantic rules.
    pub const RESOURCE_EXPRESSION_UNRESOLVED: Self =
        Self(Cow::Borrowed("ZM-SEM-RESOURCE-004"));

    // ---------------------------------------------------------------------
    // Contracts, provenance, and domain semantics
    // ---------------------------------------------------------------------

    /// A precondition, postcondition, or invariant is malformed.
    pub const CONTRACT_INVALID: Self =
        Self(Cow::Borrowed("ZM-SEM-CONTRACT-001"));

    /// A required contract obligation has not been discharged.
    pub const CONTRACT_UNPROVEN: Self =
        Self(Cow::Borrowed("ZM-SEM-CONTRACT-002"));

    /// A contract is contradicted by known semantic facts.
    pub const CONTRACT_VIOLATION: Self =
        Self(Cow::Borrowed("ZM-SEM-CONTRACT-003"));

    /// Required provenance information is missing.
    pub const PROVENANCE_MISSING: Self =
        Self(Cow::Borrowed("ZM-SEM-PROVENANCE-001"));

    /// Provenance or evidence is invalid or inconsistent.
    pub const PROVENANCE_INVALID: Self =
        Self(Cow::Borrowed("ZM-SEM-PROVENANCE-002"));

    /// A domain-specific semantic rule was violated.
    pub const DOMAIN_RULE_VIOLATION: Self =
        Self(Cow::Borrowed("ZM-SEM-DOMAIN-001"));

    /// A hybrid-domain boundary or conversion is invalid.
    pub const DOMAIN_BOUNDARY_INVALID: Self =
        Self(Cow::Borrowed("ZM-SEM-DOMAIN-002"));

    /// A quantum operation violates a language-defined semantic rule.
    pub const QUANTUM_SEMANTICS_INVALID: Self =
        Self(Cow::Borrowed("ZM-SEM-QUANTUM-001"));

    /// A hardware-description construct violates a semantic rule.
    pub const HDL_SEMANTICS_INVALID: Self =
        Self(Cow::Borrowed("ZM-SEM-HDL-001"));

    /// A symbolic shape, dimension, or tensor constraint is invalid.
    pub const SHAPE_CONSTRAINT_INVALID: Self =
        Self(Cow::Borrowed("ZM-SEM-SHAPE-001"));

    /// An asynchronous, concurrent, or distributed rule was violated.
    pub const CONCURRENCY_SEMANTICS_INVALID: Self =
        Self(Cow::Borrowed("ZM-SEM-CONCURRENCY-001"));

    // ---------------------------------------------------------------------
    // Extensions and implementation failures
    // ---------------------------------------------------------------------

    /// An extension identifier is malformed or conflicts with registration.
    pub const EXTENSION_INVALID: Self =
        Self(Cow::Borrowed("ZM-SEM-EXTENSION-001"));

    /// An extension version or contract is incompatible.
    pub const EXTENSION_INCOMPATIBLE: Self =
        Self(Cow::Borrowed("ZM-SEM-EXTENSION-002"));

    /// An internal semantic-analysis invariant failed.
    pub const INTERNAL_INVARIANT: Self =
        Self(Cow::Borrowed("ZM-SEM-INTERNAL-001"));

    /// An internal semantic-analysis operation failed unexpectedly.
    pub const INTERNAL_FAILURE: Self =
        Self(Cow::Borrowed("ZM-SEM-INTERNAL-002"));

    /// A required semantic implementation is unavailable.
    pub const IMPLEMENTATION_UNAVAILABLE: Self =
        Self(Cow::Borrowed("ZM-SEM-INTERNAL-003"));

    /// Construct a validated extension-defined diagnostic code.
    ///
    /// # Format
    ///
    /// A code must contain at least two non-empty uppercase ASCII segments
    /// separated by hyphens. Segments may contain ASCII letters and digits,
    /// and must begin with a letter. Examples:
    ///
    /// - `ACME-SEM-RESOURCE-001`
    /// - `ORG-QUANTUM-VERIFY-007`
    /// - `ZAMANI-EXT-EXAMPLE-001`
    ///
    /// The constructor preserves the original spelling exactly.
    pub fn new(value: impl Into<String>) -> Result<Self, DiagnosticCodeError> {
        let value = value.into();
        validate_code(&value)?;
        Ok(Self(Cow::Owned(value)))
    }

    /// Return the exact stable identifier string.
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }

    /// Return the broad category inferred from this code's namespace.
    ///
    /// Unknown valid codes are classified by their recognizable code
    /// segments. If no core category can be inferred, `Extension` is used.
    /// Extensions needing a more specific category should store that
    /// information in their own registered metadata.
    pub fn category(&self) -> DiagnosticCategory {
        category_for_code(self.as_str())
    }

    /// Return the default severity associated with the code.
    ///
    /// Unknown codes default to `Error`, conservatively avoiding the
    /// accidental treatment of an unrecognized semantic failure as harmless.
    pub fn default_severity(&self) -> DiagnosticSeverity {
        match self.as_str() {
            "ZM-SEM-ANALYSIS-001"
            | "ZM-SEM-ANALYSIS-002"
            | "ZM-SEM-ANALYSIS-003" => DiagnosticSeverity::Error,

            "ZM-SEM-INTERNAL-001"
            | "ZM-SEM-INTERNAL-002"
            | "ZM-SEM-INTERNAL-003" => DiagnosticSeverity::Fatal,

            _ => DiagnosticSeverity::Error,
        }
    }

    /// Look up metadata for a built-in diagnostic code.
    ///
    /// Returns `None` for extension-defined or unknown codes. A `None` result
    /// does not invalidate the code itself.
    pub fn builtin_metadata(&self) -> Option<&'static BuiltinDiagnosticMetadata> {
        BUILTIN_DIAGNOSTICS
            .iter()
            .find(|metadata| metadata.code == self.as_str())
    }

    /// Whether this code is in the built-in catalog.
    pub fn is_builtin(&self) -> bool {
        self.builtin_metadata().is_some()
    }
}

impl AsRef<str> for DiagnosticCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Borrow<str> for DiagnosticCode {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for DiagnosticCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for DiagnosticCode {
    type Err = DiagnosticCodeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value.to_owned())
    }
}

/// Failure while constructing a diagnostic code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiagnosticCodeError {
    /// The code was empty.
    Empty,
    /// The code did not contain at least two non-empty segments.
    TooFewSegments,
    /// A code segment was empty.
    EmptySegment,
    /// A segment did not begin with an uppercase ASCII letter.
    InvalidSegmentStart { segment: usize },
    /// A character was not an uppercase ASCII letter, digit, or hyphen.
    InvalidCharacter { index: usize, character: char },
}

impl fmt::Display for DiagnosticCodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("diagnostic code must not be empty"),
            Self::TooFewSegments => {
                formatter.write_str("diagnostic code must contain at least two segments")
            }
            Self::EmptySegment => {
                formatter.write_str("diagnostic code contains an empty segment")
            }
            Self::InvalidSegmentStart { segment } => write!(
                formatter,
                "diagnostic code segment {segment} must begin with an uppercase ASCII letter"
            ),
            Self::InvalidCharacter { index, character } => write!(
                formatter,
                "invalid character {character:?} at byte index {index} in diagnostic code"
            ),
        }
    }
}

impl Error for DiagnosticCodeError {}

/// Stable metadata for a built-in diagnostic code.
///
/// The summary is concise, machine-independent metadata. Detailed messages,
/// source labels, contextual values, and remediation instructions belong to
/// the diagnostic record and presentation layers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BuiltinDiagnosticMetadata {
    /// Stable diagnostic identifier.
    pub code: &'static str,
    /// Broad semantic category.
    pub category: DiagnosticCategory,
    /// Default severity.
    pub severity: DiagnosticSeverity,
    /// Concise explanation of the issue class.
    pub summary: &'static str,
}

/// Canonical built-in metadata table.
///
/// Keep identifiers unique. Existing identifiers must not be repurposed.
/// When adding a built-in code, add its metadata and a uniqueness test.
pub const BUILTIN_DIAGNOSTICS: &[BuiltinDiagnosticMetadata] = &[
    metadata!("ZM-SEM-STRUCTURE-001", Structural, Error, "Invalid semantic structure"),
    metadata!("ZM-SEM-FEATURE-001", Domain, Error, "Unsupported semantic feature"),
    metadata!("ZM-SEM-ANALYSIS-001", Analysis, Error, "Semantic analysis incomplete"),
    metadata!("ZM-SEM-ANALYSIS-002", Analysis, Error, "Analysis budget exhausted"),
    metadata!("ZM-SEM-ANALYSIS-003", Analysis, Error, "Analysis input changed"),
    metadata!("ZM-SEM-NAME-001", NameResolution, Error, "Unresolved name"),
    metadata!("ZM-SEM-NAME-002", NameResolution, Error, "Duplicate declaration"),
    metadata!("ZM-SEM-NAME-003", NameResolution, Error, "Inaccessible declaration"),
    metadata!("ZM-SEM-IMPORT-001", NameResolution, Error, "Unresolved import"),
    metadata!("ZM-SEM-IMPORT-002", NameResolution, Error, "Prohibited import cycle"),
    metadata!("ZM-SEM-DECL-001", NameResolution, Error, "Illegal declaration cycle"),
    metadata!("ZM-SEM-DECL-002", NameResolution, Error, "Missing declaration"),
    metadata!("ZM-SEM-DECL-003", NameResolution, Error, "Invalid declaration"),
    metadata!("ZM-SEM-TYPE-001", Type, Error, "Type mismatch"),
    metadata!("ZM-SEM-TYPE-002", Type, Error, "Unresolved type inference"),
    metadata!("ZM-SEM-TYPE-003", Type, Error, "Unknown type"),
    metadata!("ZM-SEM-TYPE-004", Type, Error, "Conflicting type constraints"),
    metadata!("ZM-SEM-TYPE-005", Type, Error, "Invalid coercion"),
    metadata!("ZM-SEM-GENERIC-001", Type, Error, "Unsatisfied generic constraint"),
    metadata!("ZM-SEM-TRAIT-001", Type, Error, "Missing trait implementation"),
    metadata!("ZM-SEM-TRAIT-002", Type, Error, "Ambiguous trait resolution"),
    metadata!("ZM-SEM-CONST-001", Evaluation, Error, "Constant evaluation failed"),
    metadata!("ZM-SEM-CONST-002", Evaluation, Error, "Constant arithmetic overflow"),
    metadata!("ZM-SEM-CONSTRAINT-001", Evaluation, Error, "Unsolved semantic constraint"),
    metadata!("ZM-SEM-OWNERSHIP-001", Ownership, Error, "Ownership rule violated"),
    metadata!("ZM-SEM-VALUE-001", Ownership, Error, "Invalid value use"),
    metadata!("ZM-SEM-EFFECT-001", Effect, Error, "Invalid effect declaration"),
    metadata!("ZM-SEM-EFFECT-002", Effect, Error, "Prohibited effect"),
    metadata!("ZM-SEM-EFFECT-003", Effect, Error, "Undeclared effect"),
    metadata!("ZM-SEM-AUTH-001", Policy, Error, "Authorization denied"),
    metadata!("ZM-SEM-POLICY-001", Policy, Error, "Policy violated"),
    metadata!("ZM-SEM-CAPABILITY-001", Capability, Error, "Invalid capability requirement"),
    metadata!("ZM-SEM-CAPABILITY-002", Capability, Error, "Unsatisfied capability"),
    metadata!("ZM-SEM-CAPABILITY-003", Capability, Error, "Conflicting capabilities"),
    metadata!("ZM-SEM-RESOURCE-001", Resource, Error, "Invalid resource requirement"),
    metadata!("ZM-SEM-RESOURCE-002", Resource, Error, "Resource unit mismatch"),
    metadata!("ZM-SEM-RESOURCE-003", Resource, Error, "Unsatisfied resource constraint"),
    metadata!("ZM-SEM-RESOURCE-004", Resource, Error, "Unresolved resource expression"),
    metadata!("ZM-SEM-CONTRACT-001", Contract, Error, "Invalid contract"),
    metadata!("ZM-SEM-CONTRACT-002", Contract, Error, "Unproven contract obligation"),
    metadata!("ZM-SEM-CONTRACT-003", Contract, Error, "Contract violation"),
    metadata!("ZM-SEM-PROVENANCE-001", Provenance, Error, "Missing provenance"),
    metadata!("ZM-SEM-PROVENANCE-002", Provenance, Error, "Invalid provenance"),
    metadata!("ZM-SEM-DOMAIN-001", Domain, Error, "Domain semantic rule violated"),
    metadata!("ZM-SEM-DOMAIN-002", Domain, Error, "Invalid domain boundary"),
    metadata!("ZM-SEM-QUANTUM-001", Domain, Error, "Invalid quantum semantics"),
    metadata!("ZM-SEM-HDL-001", Domain, Error, "Invalid HDL semantics"),
    metadata!("ZM-SEM-SHAPE-001", Type, Error, "Invalid shape constraint"),
    metadata!("ZM-SEM-CONCURRENCY-001", Domain, Error, "Invalid concurrency semantics"),
    metadata!("ZM-SEM-EXTENSION-001", Extension, Error, "Invalid extension"),
    metadata!("ZM-SEM-EXTENSION-002", Extension, Error, "Incompatible extension"),
    metadata!("ZM-SEM-INTERNAL-001", Internal, Fatal, "Internal invariant failed"),
    metadata!("ZM-SEM-INTERNAL-002", Internal, Fatal, "Internal analysis failure"),
    metadata!("ZM-SEM-INTERNAL-003", Internal, Fatal, "Implementation unavailable"),
];

/// Construct one row in the built-in metadata table.
macro_rules! metadata {
    ($code:literal, $category:ident, $severity:ident, $summary:literal) => {
        BuiltinDiagnosticMetadata {
            code: $code,
            category: DiagnosticCategory::$category,
            severity: DiagnosticSeverity::$severity,
            summary: $summary,
        }
    };
}

fn validate_code(value: &str) -> Result<(), DiagnosticCodeError> {
    if value.is_empty() {
        return Err(DiagnosticCodeError::Empty);
    }

    let mut segment_index = 0usize;
    let mut segment_start = true;
    let mut segment_count = 0usize;

    for (index, character) in value.char_indices() {
        if character == '-' {
            if segment_start {
                return Err(DiagnosticCodeError::EmptySegment);
            }

            segment_count += 1;
            segment_start = true;
            segment_index += 1;
            continue;
        }

        if !character.is_ascii_uppercase() && !character.is_ascii_digit() {
            return Err(DiagnosticCodeError::InvalidCharacter {
                index,
                character,
            });
        }

        if segment_start && !character.is_ascii_uppercase() {
            return Err(DiagnosticCodeError::InvalidSegmentStart {
                segment: segment_index,
            });
        }

        segment_start = false;
    }

    if segment_start {
        return Err(DiagnosticCodeError::EmptySegment);
    }

    segment_count += 1;

    if segment_count < 2 {
        return Err(DiagnosticCodeError::TooFewSegments);
    }

    Ok(())
}

fn category_for_code(code: &str) -> DiagnosticCategory {
    // Classification is based on stable code segments, not message text.
    // Unknown valid codes remain representable and use a safe fallback.
    let mut parts = code.split('-');
    let _namespace = parts.next();
    let _family = parts.next();
    let area = parts.next();

    match area {
        Some("STRUCTURE") => DiagnosticCategory::Structural,
        Some("FEATURE") => DiagnosticCategory::Domain,
        Some("ANALYSIS") => DiagnosticCategory::Analysis,
        Some("NAME" | "IMPORT" | "DECL") => DiagnosticCategory::NameResolution,
        Some("TYPE" | "GENERIC" | "TRAIT" | "SHAPE") => DiagnosticCategory::Type,
        Some("OWNERSHIP" | "VALUE") => DiagnosticCategory::Ownership,
        Some("EFFECT") => DiagnosticCategory::Effect,
        Some("AUTH" | "POLICY") => DiagnosticCategory::Policy,
        Some("CAPABILITY") => DiagnosticCategory::Capability,
        Some("RESOURCE") => DiagnosticCategory::Resource,
        Some("CONTRACT") => DiagnosticCategory::Contract,
        Some("PROVENANCE") => DiagnosticCategory::Provenance,
        Some("DOMAIN" | "QUANTUM" | "HDL" | "CONCURRENCY") => {
            DiagnosticCategory::Domain
        }
        Some("EXTENSION") => DiagnosticCategory::Extension,
        Some("CONST" | "CONSTRAINT") => DiagnosticCategory::Evaluation,
        Some("INTERNAL") => DiagnosticCategory::Internal,
        _ => DiagnosticCategory::Extension,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn built_in_codes_are_unique() {
        let mut seen = HashSet::new();

        for entry in BUILTIN_DIAGNOSTICS {
            assert!(
                seen.insert(entry.code),
                "duplicate built-in diagnostic code: {}",
                entry.code
            );
        }
    }

    #[test]
    fn every_builtin_code_is_valid_and_resolvable() {
        for entry in BUILTIN_DIAGNOSTICS {
            let parsed = DiagnosticCode::new(entry.code.to_owned())
                .expect("built-in code must satisfy the public format");

            assert_eq!(parsed.as_str(), entry.code);
            assert!(parsed.is_builtin());
            assert_eq!(
                parsed.builtin_metadata().map(|metadata| metadata.code),
                Some(entry.code)
            );
        }
    }

    #[test]
    fn built_in_constants_match_the_catalog() {
        let constants = [
            DiagnosticCode::NAME_UNRESOLVED,
            DiagnosticCode::TYPE_MISMATCH,
            DiagnosticCode::EFFECT_VIOLATION,
            DiagnosticCode::CAPABILITY_UNSATISFIED,
            DiagnosticCode::RESOURCE_REQUIREMENT_INVALID,
            DiagnosticCode::CONTRACT_VIOLATION,
            DiagnosticCode::INTERNAL_FAILURE,
        ];

        for code in constants {
            assert!(
                code.is_builtin(),
                "missing metadata for {}",
                code.as_str()
            );
        }
    }

    #[test]
    fn valid_extension_codes_are_preserved_exactly() {
        let code = DiagnosticCode::new("ACME-SEM-RESOURCE-001")
            .expect("extension code should be valid");

        assert_eq!(code.as_str(), "ACME-SEM-RESOURCE-001");
        assert_eq!(code.category(), DiagnosticCategory::Resource);
        assert!(!code.is_builtin());
    }

    #[test]
    fn unknown_valid_codes_are_not_rejected() {
        let code = DiagnosticCode::new("FUTURE-SEM-NEWDOMAIN-001")
            .expect("future extension code should be valid");

        assert_eq!(code.as_str(), "FUTURE-SEM-NEWDOMAIN-001");
        assert!(!code.is_builtin());
        assert_eq!(code.default_severity(), DiagnosticSeverity::Error);
    }

    #[test]
    fn malformed_codes_are_rejected() {
        let invalid = [
            "",
            "A",
            "-ZM-SEM-001",
            "ZM-SEM-",
            "ZM--SEM",
            "zm-sem-name-001",
            "ZM-SEM-name-001",
            "ZM-SEM NAME-001",
            "ZM-SEM-NAME_001",
            "1ZM-SEM-NAME-001",
            "ZM-SEM-NÄME-001",
        ];

        for value in invalid {
            assert!(
                DiagnosticCode::new(value.to_owned()).is_err(),
                "invalid code unexpectedly accepted: {value:?}"
            );
        }
    }

    #[test]
    fn code_ordering_and_hashing_are_deterministic() {
        let left = DiagnosticCode::new("ACME-SEM-ALPHA-001").unwrap();
        let same = DiagnosticCode::new("ACME-SEM-ALPHA-001").unwrap();
        let right = DiagnosticCode::new("ACME-SEM-BETA-001").unwrap();

        assert_eq!(left, same);
        assert_ne!(left, right);
        assert!(left < right);

        let set = HashSet::from([left.clone(), same, right]);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn display_and_as_ref_return_the_stable_identifier() {
        let code = DiagnosticCode::NAME_UNRESOLVED;

        assert_eq!(code.to_string(), "ZM-SEM-NAME-001");
        assert_eq!(code.as_ref(), "ZM-SEM-NAME-001");
    }

    #[test]
    fn string_parsing_uses_the_same_validation() {
        let parsed: DiagnosticCode = "ZM-SEM-TYPE-001".parse().unwrap();
        assert_eq!(parsed, DiagnosticCode::TYPE_MISMATCH);

        assert!("not a code".parse::<DiagnosticCode>().is_err());
    }

    #[test]
    fn extension_categories_are_validated() {
        assert_eq!(
            DiagnosticCategory::extension("vendor.quantum").unwrap().as_str(),
            "vendor.quantum"
        );

        assert!(DiagnosticCategory::extension("").is_err());
        assert!(DiagnosticCategory::extension("Invalid").is_err());
        assert!(DiagnosticCategory::extension(".quantum").is_err());
        assert!(DiagnosticCategory::extension("vendor..quantum").is_err());
    }

    #[test]
    fn internal_faults_have_fatal_default_severity() {
        assert_eq!(
            DiagnosticCode::INTERNAL_FAILURE.default_severity(),
            DiagnosticSeverity::Fatal
        );

        assert_eq!(
            DiagnosticCode::NAME_UNRESOLVED.default_severity(),
            DiagnosticSeverity::Error
        );
    }

    #[test]
    fn every_catalog_entry_has_a_nonempty_summary() {
        for entry in BUILTIN_DIAGNOSTICS {
            assert!(!entry.code.is_empty());
            assert!(!entry.summary.trim().is_empty());
        }
    }
}

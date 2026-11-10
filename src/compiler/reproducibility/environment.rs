
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Compiler — Reproducible Compilation Environment.
//!
//! Path:
//!     src/compiler/reproducibility/environment.rs
//!
//! Rust:
//!     Edition 2021
//!     Minimum version: Rust 1.97
//!
//! Safety:
//!     Safe Rust only.
//!
//! # Purpose
//!
//! Establish an explicit, immutable and deterministic identity for the
//! compilation environment associated with a Zamani compilation.
//!
//! The environment identity captures the declared and verified inputs
//! that can affect compilation results.
//!
//! # Canonical authorities
//!
//! Fingerprint algorithm:
//!     crate::zuir::fingerprint::stable_hash
//!
//! Compiler fingerprint contracts:
//!     crate::compiler::incremental::fingerprints
//!
//! Compilation context:
//!     crate::compiler::context::compilation_context
//!
//! Compilation provenance:
//!     crate::compiler::context::provenance_context
//!
//! Artifact metadata:
//!     crate::compiler::artifacts
//!
//! Supply-chain verification:
//!     crate::compiler::security
//!
//! Source-level reproducibility intent:
//!     grammar/compile/reproducibility.g4
//!
//! Deterministic build intent:
//!     grammar/compile/deterministic-builds.g4
//!
//! # Ownership
//!
//! This module owns:
//!
//! - Compilation environment identity.
//! - Environment scope.
//! - Canonical environment facts.
//! - Compiler, grammar and semantic-model fingerprints.
//! - Target-specific environment identity.
//! - External toolchain fingerprint associations.
//! - Environment fingerprint construction.
//! - Environment validation.
//!
//! This module does not own:
//!
//! - Source parsing.
//! - AST construction.
//! - Semantic interpretation.
//! - Hardware discovery.
//! - Operating-system discovery.
//! - Environment-variable inspection.
//! - Physical resource allocation.
//! - Toolchain installation.
//! - Backend implementation.
//! - Artifact serialization.
//! - Cryptographic signatures.
//! - Runtime execution.
//!
//! # POCO-REAF
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! Environment identity must not impose universal hardware ceilings.
//!
//! No fixed maximum is imposed on:
//!
//! - CPU count;
//! - GPU count;
//! - FPGA count;
//! - QPU count;
//! - qubits;
//! - logical qubits;
//! - distributed nodes;
//! - network participants;
//! - memory;
//! - storage;
//! - tensor rank;
//! - tensor dimensions;
//! - register widths;
//! - toolchains;
//! - compiler components;
//! - environment facts;
//! - computing domains;
//! - target providers.
//!
//! Actual limits arise from available memory, execution time,
//! address-space limits and explicitly configured operational budgets.
//!
//! Literal infinite physical resources are not promised.
//!
//! # Determinism
//!
//! Environment identity must be independent of:
//!
//! - wall-clock time;
//! - process identifiers;
//! - memory addresses;
//! - randomized hashers;
//! - thread execution order;
//! - filesystem enumeration order;
//! - locale-dependent formatting;
//! - implicit environment-variable reads;
//! - incidental machine identity.
//!
//! All fingerprinted inputs are explicitly supplied.
//!
//! Ordered maps provide deterministic iteration.
//!
//! Fingerprints use the canonical ZUIR SHA-256 implementation.
//!
//! # Important distinction
//!
//! Environment identity is not the same as:
//!
//! - source identity;
//! - compiler provenance;
//! - artifact identity;
//! - target capability;
//! - physical device identity;
//! - runtime execution identity.
//!
//! These identities remain owned by their respective subsystems.
//!
//! # Security
//!
//! Environment values are supplied explicitly.
//!
//! This module never reads credentials, secrets, environment variables,
//! files, network endpoints or hardware.
//!
//! Callers must not insert passwords, private keys, access tokens,
//! credentials or other secret material into fingerprint inputs.
//!
//! A fingerprint establishes content identity, not authenticity.
//! Authentication and trust verification belong to security providers.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use crate::zuir::fingerprint::stable_hash::{
    Fingerprint,
    FingerprintError,
    StableHashBuilder,
};

// ============================================================================
// VERSIONING
// ============================================================================

/// Public API version for the compilation environment contract.
///
/// This version is independent of:
///
/// - Zamani language version;
/// - grammar version;
/// - semantic-model version;
/// - compiler release;
/// - artifact format version;
/// - ZUIR version;
/// - backend ABI version.
pub const COMPILATION_ENVIRONMENT_API_VERSION: u16 = 1;

/// Version of the canonical environment fingerprint schema.
///
/// Increment when the fingerprint field selection, ordering or
/// interpretation changes incompatibly.
pub const COMPILATION_ENVIRONMENT_SCHEMA_VERSION: u32 = 1;

/// Canonical domain for compilation environment fingerprints.
pub const COMPILATION_ENVIRONMENT_FINGERPRINT_DOMAIN: &str =
    "zamani.compiler.reproducibility.environment.v1";

// ============================================================================
// ENVIRONMENT SCOPE
// ============================================================================

/// Determines whether an environment identity is portable or
/// explicitly associated with a target realization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum EnvironmentScope {
    /// Target-independent compilation environment.
    ///
    /// Suitable for identifying compilation inputs that are independent
    /// of a particular physical machine or realization target.
    ///
    /// No target fingerprint is permitted.
    Portable,

    /// Environment associated with a particular target descriptor.
    ///
    /// The target fingerprint must be explicitly supplied.
    ///
    /// The target descriptor is owned by the authoritative target
    /// description and capability subsystems.
    TargetSpecific,
}

impl EnvironmentScope {
    /// Returns the stable schema representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Portable => "portable",
            Self::TargetSpecific => "target-specific",
        }
    }
}

// ============================================================================
// ENVIRONMENT FACT CLASSIFICATION
// ============================================================================

/// Determines whether an environment fact affects compilation output.
///
/// Only output-affecting facts participate in the environment fingerprint.
///
/// Descriptive facts remain available as metadata but cannot silently
/// change portable compilation identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum EnvironmentFactImpact {
    /// The value can affect the compilation result.
    ///
    /// Examples include explicitly selected:
    ///
    /// - compiler behavior;
    /// - ABI behavior;
    /// - code-generation behavior;
    /// - toolchain configuration;
    /// - language edition;
    /// - semantic configuration.
    OutputAffecting,

    /// The value is descriptive and must not affect compilation identity.
    ///
    /// Examples include non-semantic:
    ///
    /// - display labels;
    /// - diagnostic annotations;
    /// - operator descriptions;
    /// - informational metadata.
    Observational,
}

// ============================================================================
// VALIDATED ENVIRONMENT KEYS
// ============================================================================

/// A stable environment fact or toolchain identifier.
///
/// Identifiers are compared and fingerprinted exactly as supplied.
///
/// No normalization based on locale, filesystem rules or operating-system
/// conventions is performed.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EnvironmentKey(String);

impl EnvironmentKey {
    /// Constructs a validated environment key.
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, EnvironmentError> {
        let value = value.into();

        validate_identifier(&value, "environment key")?;

        Ok(Self(value))
    }

    /// Returns the exact identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for EnvironmentKey {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// ENVIRONMENT FACTS
// ============================================================================

/// One explicitly supplied environment fact.
///
/// Values are stored as bytes so that the environment identity can
/// represent canonical binary descriptors as well as UTF-8 values.
///
/// The caller is responsible for supplying a canonical representation
/// of each value.
///
/// Debug formatting must never be used as canonical input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnvironmentFact {
    key: EnvironmentKey,
    value: Vec<u8>,
    impact: EnvironmentFactImpact,
}

impl EnvironmentFact {
    /// Constructs an environment fact.
    pub fn new(
        key: EnvironmentKey,
        value: impl Into<Vec<u8>>,
        impact: EnvironmentFactImpact,
    ) -> Self {
        Self {
            key,
            value: value.into(),
            impact,
        }
    }

    /// Returns the fact identifier.
    #[must_use]
    pub fn key(&self) -> &EnvironmentKey {
        &self.key
    }

    /// Returns the exact supplied value.
    #[must_use]
    pub fn value(&self) -> &[u8] {
        &self.value
    }

    /// Returns the impact classification.
    #[must_use]
    pub const fn impact(&self) -> EnvironmentFactImpact {
        self.impact
    }
}

// ============================================================================
// TOOLCHAIN IDENTITY
// ============================================================================

/// Identity of an external compiler or toolchain component.
///
/// Examples include:
///
/// - LLVM;
/// - a linker;
/// - a quantum compiler;
/// - an HDL synthesis tool;
/// - an AI accelerator compiler;
/// - a WebAssembly toolchain;
/// - a domain-specific compiler provider.
///
/// This is deliberately an extensible identifier rather than a closed
/// enumeration of supported vendors or computing technologies.
///
/// The actual toolchain descriptor and verification evidence remain
/// owned by their respective providers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolchainIdentity {
    identifier: EnvironmentKey,
    fingerprint: Fingerprint,
}

impl ToolchainIdentity {
    /// Creates a toolchain identity from an authoritative fingerprint.
    pub fn new(
        identifier: EnvironmentKey,
        fingerprint: Fingerprint,
    ) -> Self {
        Self {
            identifier,
            fingerprint,
        }
    }

    /// Returns the toolchain identifier.
    #[must_use]
    pub fn identifier(&self) -> &EnvironmentKey {
        &self.identifier
    }

    /// Returns the canonical toolchain fingerprint.
    #[must_use]
    pub const fn fingerprint(&self) -> Fingerprint {
        self.fingerprint
    }
}

// ============================================================================
// ERRORS
// ============================================================================

/// Errors produced by compilation environment construction.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum EnvironmentError {
    /// An identifier is empty or contains forbidden characters.
    InvalidIdentifier {
        /// Identifier description.
        description: &'static str,
    },

    /// An environment fact was inserted more than once.
    DuplicateFact {
        /// Duplicated identifier.
        key: String,
    },

    /// A toolchain identity was inserted more than once.
    DuplicateToolchain {
        /// Duplicated identifier.
        identifier: String,
    },

    /// A target-specific environment lacks its target fingerprint.
    MissingTargetFingerprint,

    /// A portable environment unexpectedly contains a target fingerprint.
    UnexpectedTargetFingerprint,

    /// A collection count cannot be represented by the fingerprint schema.
    CountOverflow {
        /// Collection description.
        collection: &'static str,
    },

    /// Canonical fingerprint construction failed.
    Fingerprinting(FingerprintError),
}

impl fmt::Display for EnvironmentError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidIdentifier { description } => {
                write!(
                    formatter,
                    "invalid compilation environment {description}"
                )
            }

            Self::DuplicateFact { key } => {
                write!(
                    formatter,
                    "duplicate compilation environment fact: {key}"
                )
            }

            Self::DuplicateToolchain { identifier } => {
                write!(
                    formatter,
                    "duplicate compilation environment toolchain: \
                     {identifier}"
                )
            }

            Self::MissingTargetFingerprint => {
                formatter.write_str(
                    "target-specific environment requires a target fingerprint",
                )
            }

            Self::UnexpectedTargetFingerprint => {
                formatter.write_str(
                    "portable environment must not contain a target fingerprint",
                )
            }

            Self::CountOverflow { collection } => {
                write!(
                    formatter,
                    "compilation environment {collection} count overflowed"
                )
            }

            Self::Fingerprinting(error) => {
                write!(
                    formatter,
                    "compilation environment fingerprint failed: {error}"
                )
            }
        }
    }
}

impl Error for EnvironmentError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Fingerprinting(error) => Some(error),
            _ => None,
        }
    }
}

impl From<FingerprintError> for EnvironmentError {
    fn from(error: FingerprintError) -> Self {
        Self::Fingerprinting(error)
    }
}

// ============================================================================
// IDENTIFIER VALIDATION
// ============================================================================

fn validate_identifier(
    value: &str,
    description: &'static str,
) -> Result<(), EnvironmentError> {
    if value.trim().is_empty() {
        return Err(EnvironmentError::InvalidIdentifier {
            description,
        });
    }

    if value.chars().any(char::is_control) {
        return Err(EnvironmentError::InvalidIdentifier {
            description,
        });
    }

    Ok(())
}

// ============================================================================
// ENVIRONMENT IDENTITY
// ============================================================================

/// Immutable, canonical compilation environment identity.
///
/// The environment contains:
///
/// - Compiler implementation fingerprint.
/// - Grammar fingerprint.
/// - Semantic-model fingerprint.
/// - Optional target fingerprint.
/// - External toolchain identities.
/// - Explicit environment facts.
/// - Canonical environment fingerprint.
///
/// All output-affecting information is included in the fingerprint.
///
/// Observational metadata is retained but excluded from the fingerprint.
///
/// The environment is immutable after construction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompilationEnvironment {
    scope: EnvironmentScope,

    compiler_fingerprint: Fingerprint,

    grammar_fingerprint: Fingerprint,

    semantic_model_fingerprint: Fingerprint,

    target_fingerprint: Option<Fingerprint>,

    toolchains: BTreeMap<EnvironmentKey, Fingerprint>,

    facts: BTreeMap<EnvironmentKey, EnvironmentFact>,

    fingerprint: Fingerprint,
}

impl CompilationEnvironment {
    /// Returns a builder for a portable environment.
    #[must_use]
    pub fn portable(
        compiler_fingerprint: Fingerprint,
        grammar_fingerprint: Fingerprint,
        semantic_model_fingerprint: Fingerprint,
    ) -> CompilationEnvironmentBuilder {
        CompilationEnvironmentBuilder::new(
            EnvironmentScope::Portable,
            compiler_fingerprint,
            grammar_fingerprint,
            semantic_model_fingerprint,
        )
    }

    /// Returns a builder for a target-specific environment.
    ///
    /// The target fingerprint must be supplied before build().
    #[must_use]
    pub fn target_specific(
        compiler_fingerprint: Fingerprint,
        grammar_fingerprint: Fingerprint,
        semantic_model_fingerprint: Fingerprint,
    ) -> CompilationEnvironmentBuilder {
        CompilationEnvironmentBuilder::new(
            EnvironmentScope::TargetSpecific,
            compiler_fingerprint,
            grammar_fingerprint,
            semantic_model_fingerprint,
        )
    }

    /// Returns the environment scope.
    #[must_use]
    pub const fn scope(&self) -> EnvironmentScope {
        self.scope
    }

    /// Returns the compiler implementation fingerprint.
    #[must_use]
    pub const fn compiler_fingerprint(&self) -> Fingerprint {
        self.compiler_fingerprint
    }

    /// Returns the grammar fingerprint.
    #[must_use]
    pub const fn grammar_fingerprint(&self) -> Fingerprint {
        self.grammar_fingerprint
    }

    /// Returns the semantic-model fingerprint.
    #[must_use]
    pub const fn semantic_model_fingerprint(&self) -> Fingerprint {
        self.semantic_model_fingerprint
    }

    /// Returns the optional target fingerprint.
    #[must_use]
    pub const fn target_fingerprint(&self) -> Option<Fingerprint> {
        self.target_fingerprint
    }

    /// Returns the canonical environment fingerprint.
    ///
    /// This is a content identity, not a digital signature.
    #[must_use]
    pub const fn fingerprint(&self) -> Fingerprint {
        self.fingerprint
    }

    /// Returns the number of registered toolchains.
    #[must_use]
    pub fn toolchain_count(&self) -> usize {
        self.toolchains.len()
    }

    /// Returns the number of environment facts.
    #[must_use]
    pub fn fact_count(&self) -> usize {
        self.facts.len()
    }

    /// Retrieves a toolchain fingerprint.
    #[must_use]
    pub fn toolchain(
        &self,
        identifier: &EnvironmentKey,
    ) -> Option<Fingerprint> {
        self.toolchains.get(identifier).copied()
    }

    /// Iterates toolchains in deterministic identifier order.
    pub fn toolchains(
        &self,
    ) -> impl ExactSizeIterator<Item = (&EnvironmentKey, &Fingerprint)> {
        self.toolchains.iter()
    }

    /// Retrieves an environment fact.
    #[must_use]
    pub fn fact(
        &self,
        key: &EnvironmentKey,
    ) -> Option<&EnvironmentFact> {
        self.facts.get(key)
    }

    /// Iterates environment facts in deterministic identifier order.
    pub fn facts(
        &self,
    ) -> impl ExactSizeIterator<Item = (&EnvironmentKey, &EnvironmentFact)> {
        self.facts.iter()
    }

    /// Returns the number of output-affecting environment facts.
    #[must_use]
    pub fn output_affecting_fact_count(&self) -> usize {
        self.facts
            .values()
            .filter(|fact| {
                fact.impact == EnvironmentFactImpact::OutputAffecting
            })
            .count()
    }

    /// Returns whether two environments have identical compilation
    /// fingerprints.
    ///
    /// This comparison concerns the fields included in the canonical
    /// environment fingerprint.
    ///
    /// It does not prove semantic equivalence of arbitrary programs.
    #[must_use]
    pub fn has_same_compilation_identity(
        &self,
        other: &Self,
    ) -> bool {
        self.fingerprint == other.fingerprint
    }

    /// Returns whether this environment is target-independent.
    #[must_use]
    pub fn is_portable(&self) -> bool {
        self.scope == EnvironmentScope::Portable
    }
}

// ============================================================================
// ENVIRONMENT BUILDER
// ============================================================================

/// Builder for immutable compilation environment identities.
///
/// The builder owns all inputs until build() is called.
///
/// Duplicate identifiers are rejected rather than silently overwritten.
#[derive(Debug)]
pub struct CompilationEnvironmentBuilder {
    scope: EnvironmentScope,

    compiler_fingerprint: Fingerprint,

    grammar_fingerprint: Fingerprint,

    semantic_model_fingerprint: Fingerprint,

    target_fingerprint: Option<Fingerprint>,

    toolchains: BTreeMap<EnvironmentKey, Fingerprint>,

    facts: BTreeMap<EnvironmentKey, EnvironmentFact>,
}

impl CompilationEnvironmentBuilder {
    /// Creates a builder with the mandatory canonical identities.
    #[must_use]
    pub fn new(
        scope: EnvironmentScope,
        compiler_fingerprint: Fingerprint,
        grammar_fingerprint: Fingerprint,
        semantic_model_fingerprint: Fingerprint,
    ) -> Self {
        Self {
            scope,
            compiler_fingerprint,
            grammar_fingerprint,
            semantic_model_fingerprint,
            target_fingerprint: None,
            toolchains: BTreeMap::new(),
            facts: BTreeMap::new(),
        }
    }

    /// Associates the environment with an authoritative target fingerprint.
    ///
    /// Only valid for EnvironmentScope::TargetSpecific.
    #[must_use]
    pub fn target_fingerprint(
        mut self,
        fingerprint: Fingerprint,
    ) -> Self {
        self.target_fingerprint = Some(fingerprint);
        self
    }

    /// Registers an external toolchain.
    ///
    /// Toolchains are ordered by their canonical identifiers.
    ///
    /// Their fingerprints must be produced by the relevant toolchain
    /// identity and verification authorities.
    pub fn add_toolchain(
        &mut self,
        toolchain: ToolchainIdentity,
    ) -> Result<&mut Self, EnvironmentError> {
        let identifier = toolchain.identifier;

        if self.toolchains.contains_key(&identifier) {
            return Err(EnvironmentError::DuplicateToolchain {
                identifier: identifier.as_str().to_owned(),
            });
        }

        self.toolchains.insert(
            identifier,
            toolchain.fingerprint,
        );

        Ok(self)
    }

    /// Registers an explicitly supplied environment fact.
    ///
    /// Duplicate keys are rejected.
    ///
    /// Values are stored exactly as supplied.
    pub fn add_fact(
        &mut self,
        fact: EnvironmentFact,
    ) -> Result<&mut Self, EnvironmentError> {
        let key = fact.key.clone();

        if self.facts.contains_key(&key) {
            return Err(EnvironmentError::DuplicateFact {
                key: key.as_str().to_owned(),
            });
        }

        self.facts.insert(key, fact);

        Ok(self)
    }

    /// Registers a UTF-8 environment fact.
    pub fn add_text_fact(
        &mut self,
        key: EnvironmentKey,
        value: impl Into<String>,
        impact: EnvironmentFactImpact,
    ) -> Result<&mut Self, EnvironmentError> {
        self.add_fact(EnvironmentFact::new(
            key,
            value.into().into_bytes(),
            impact,
        ))
    }

    /// Constructs and fingerprints the immutable environment.
    pub fn build(self) -> Result<CompilationEnvironment, EnvironmentError> {
        match (self.scope, self.target_fingerprint) {
            (EnvironmentScope::Portable, Some(_)) => {
                return Err(
                    EnvironmentError::UnexpectedTargetFingerprint
                );
            }

            (EnvironmentScope::TargetSpecific, None) => {
                return Err(
                    EnvironmentError::MissingTargetFingerprint
                );
            }

            _ => {}
        }

        let fingerprint = fingerprint_environment(
            self.scope,
            self.compiler_fingerprint,
            self.grammar_fingerprint,
            self.semantic_model_fingerprint,
            self.target_fingerprint,
            &self.toolchains,
            &self.facts,
        )?;

        Ok(CompilationEnvironment {
            scope: self.scope,
            compiler_fingerprint: self.compiler_fingerprint,
            grammar_fingerprint: self.grammar_fingerprint,
            semantic_model_fingerprint: self.semantic_model_fingerprint,
            target_fingerprint: self.target_fingerprint,
            toolchains: self.toolchains,
            facts: self.facts,
            fingerprint,
        })
    }
}

// ============================================================================
// CANONICAL FINGERPRINT CONSTRUCTION
// ============================================================================

/// Converts a collection size into the canonical fingerprint count type.
fn canonical_count(
    count: usize,
    collection: &'static str,
) -> Result<u64, EnvironmentError> {
    u64::try_from(count).map_err(|_| {
        EnvironmentError::CountOverflow {
            collection,
        }
    })
}

/// Constructs the canonical environment fingerprint.
///
/// IMPORTANT:
///
/// Field order is part of the fingerprint schema.
///
/// Changing this ordering requires a deliberate schema-version change.
///
/// BTreeMap provides deterministic ordering independent of insertion order.
///
/// Observational facts are deliberately excluded.
///
/// The fingerprint algorithm and byte framing are owned by ZUIR.
fn fingerprint_environment(
    scope: EnvironmentScope,
    compiler_fingerprint: Fingerprint,
    grammar_fingerprint: Fingerprint,
    semantic_model_fingerprint: Fingerprint,
    target_fingerprint: Option<Fingerprint>,
    toolchains: &BTreeMap<EnvironmentKey, Fingerprint>,
    facts: &BTreeMap<EnvironmentKey, EnvironmentFact>,
) -> Result<Fingerprint, EnvironmentError> {
    let mut builder = StableHashBuilder::new(
        COMPILATION_ENVIRONMENT_FINGERPRINT_DOMAIN,
    )?;

    builder.field_u32(
        "schema_version",
        COMPILATION_ENVIRONMENT_SCHEMA_VERSION,
    )?;

    builder.field_str(
        "fingerprint_algorithm",
        crate::zuir::fingerprint::stable_hash::FINGERPRINT_ALGORITHM,
    )?;

    builder.field_str(
        "scope",
        scope.as_str(),
    )?;

    builder.field_fingerprint(
        "compiler_fingerprint",
        compiler_fingerprint,
    )?;

    builder.field_fingerprint(
        "grammar_fingerprint",
        grammar_fingerprint,
    )?;

    builder.field_fingerprint(
        "semantic_model_fingerprint",
        semantic_model_fingerprint,
    )?;

    match target_fingerprint {
        Some(target) => {
            builder.field_bool("has_target", true)?;
            builder.field_fingerprint("target_fingerprint", target)?;
        }

        None => {
            builder.field_bool("has_target", false)?;
        }
    }

    // ------------------------------------------------------------------------
    // Toolchain identities
    // ------------------------------------------------------------------------

    let toolchain_count = canonical_count(
        toolchains.len(),
        "toolchain",
    )?;

    builder.field_u64(
        "toolchain_count",
        toolchain_count,
    )?;

    for (identifier, fingerprint) in toolchains {
        builder.field_str(
            "toolchain_identifier",
            identifier.as_str(),
        )?;

        builder.field_fingerprint(
            "toolchain_fingerprint",
            *fingerprint,
        )?;
    }

    // ------------------------------------------------------------------------
    // Output-affecting environment facts
    // ------------------------------------------------------------------------

    let output_affecting_count = facts
        .values()
        .filter(|fact| {
            fact.impact == EnvironmentFactImpact::OutputAffecting
        })
        .count();

    let fact_count = canonical_count(
        output_affecting_count,
        "output-affecting environment fact",
    )?;

    builder.field_u64(
        "output_affecting_fact_count",
        fact_count,
    )?;

    for (key, fact) in facts {
        if fact.impact != EnvironmentFactImpact::OutputAffecting {
            continue;
        }

        builder.field_str(
            "environment_fact_key",
            key.as_str(),
        )?;

        builder.field_bytes(
            "environment_fact_value",
            &fact.value,
        )?;
    }

    Ok(builder.finish())
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn fingerprint(value: u8) -> Fingerprint {
        Fingerprint::from_bytes([value; 32])
    }

    fn compiler() -> Fingerprint {
        fingerprint(1)
    }

    fn grammar() -> Fingerprint {
        fingerprint(2)
    }

    fn semantics() -> Fingerprint {
        fingerprint(3)
    }

    fn key(value: &str) -> EnvironmentKey {
        EnvironmentKey::new(value).expect("valid environment key")
    }

    fn portable_environment() -> CompilationEnvironment {
        CompilationEnvironment::portable(
            compiler(),
            grammar(),
            semantics(),
        )
        .build()
        .expect("valid portable environment")
    }

    #[test]
    fn creates_portable_environment() {
        let environment = portable_environment();

        assert!(environment.is_portable());

        assert_eq!(
            environment.scope(),
            EnvironmentScope::Portable,
        );

        assert!(environment.target_fingerprint().is_none());
    }

    #[test]
    fn creates_target_specific_environment() {
        let environment = CompilationEnvironment::target_specific(
            compiler(),
            grammar(),
            semantics(),
        )
        .target_fingerprint(fingerprint(4))
        .build()
        .expect("valid target-specific environment");

        assert_eq!(
            environment.scope(),
            EnvironmentScope::TargetSpecific,
        );

        assert_eq!(
            environment.target_fingerprint(),
            Some(fingerprint(4)),
        );
    }

    #[test]
    fn target_specific_environment_requires_target_identity() {
        let result = CompilationEnvironment::target_specific(
            compiler(),
            grammar(),
            semantics(),
        )
        .build();

        assert!(matches!(
            result,
            Err(EnvironmentError::MissingTargetFingerprint)
        ));
    }

    #[test]
    fn portable_environment_rejects_target_identity() {
        let result = CompilationEnvironment::portable(
            compiler(),
            grammar(),
            semantics(),
        )
        .target_fingerprint(fingerprint(4))
        .build();

        assert!(matches!(
            result,
            Err(EnvironmentError::UnexpectedTargetFingerprint)
        ));
    }

    #[test]
    fn identical_inputs_produce_identical_fingerprints() {
        let first = portable_environment();
        let second = portable_environment();

        assert_eq!(
            first.fingerprint(),
            second.fingerprint(),
        );
    }

    #[test]
    fn compiler_identity_changes_environment_identity() {
        let first = CompilationEnvironment::portable(
            fingerprint(1),
            grammar(),
            semantics(),
        )
        .build()
        .unwrap();

        let second = CompilationEnvironment::portable(
            fingerprint(9),
            grammar(),
            semantics(),
        )
        .build()
        .unwrap();

        assert_ne!(
            first.fingerprint(),
            second.fingerprint(),
        );
    }

    #[test]
    fn grammar_identity_changes_environment_identity() {
        let first = CompilationEnvironment::portable(
            compiler(),
            fingerprint(2),
            semantics(),
        )
        .build()
        .unwrap();

        let second = CompilationEnvironment::portable(
            compiler(),
            fingerprint(8),
            semantics(),
        )
        .build()
        .unwrap();

        assert_ne!(
            first.fingerprint(),
            second.fingerprint(),
        );
    }

    #[test]
    fn semantic_model_identity_changes_environment_identity() {
        let first = CompilationEnvironment::portable(
            compiler(),
            grammar(),
            fingerprint(3),
        )
        .build()
        .unwrap();

        let second = CompilationEnvironment::portable(
            compiler(),
            grammar(),
            fingerprint(8),
        )
        .build()
        .unwrap();

        assert_ne!(
            first.fingerprint(),
            second.fingerprint(),
        );
    }

    #[test]
    fn toolchains_are_included_in_environment_identity() {
        let mut first = CompilationEnvironment::portable(
            compiler(),
            grammar(),
            semantics(),
        );

        first.add_toolchain(
            ToolchainIdentity::new(
                key("llvm"),
                fingerprint(5),
            ),
        ).unwrap();

        let first = first.build().unwrap();

        let mut second = CompilationEnvironment::portable(
            compiler(),
            grammar(),
            semantics(),
        );

        second.add_toolchain(
            ToolchainIdentity::new(
                key("llvm"),
                fingerprint(6),
            ),
        ).unwrap();

        let second = second.build().unwrap();

        assert_ne!(
            first.fingerprint(),
            second.fingerprint(),
        );
    }

    #[test]
    fn duplicate_toolchains_are_rejected() {
        let mut builder = CompilationEnvironment::portable(
            compiler(),
            grammar(),
            semantics(),
        );

        builder.add_toolchain(
            ToolchainIdentity::new(
                key("llvm"),
                fingerprint(5),
            ),
        ).unwrap();

        assert!(matches!(
            builder.add_toolchain(
                ToolchainIdentity::new(
                    key("llvm"),
                    fingerprint(6),
                ),
            ),
            Err(EnvironmentError::DuplicateToolchain { .. })
        ));
    }

    #[test]
    fn output_affecting_facts_change_identity() {
        let mut first = CompilationEnvironment::portable(
            compiler(),
            grammar(),
            semantics(),
        );

        first.add_text_fact(
            key("compiler.mode"),
            "strict",
            EnvironmentFactImpact::OutputAffecting,
        ).unwrap();

        let first = first.build().unwrap();

        let mut second = CompilationEnvironment::portable(
            compiler(),
            grammar(),
            semantics(),
        );

        second.add_text_fact(
            key("compiler.mode"),
            "permissive",
            EnvironmentFactImpact::OutputAffecting,
        ).unwrap();

        let second = second.build().unwrap();

        assert_ne!(
            first.fingerprint(),
            second.fingerprint(),
        );
    }

    #[test]
    fn observational_facts_do_not_change_identity() {
        let first = portable_environment();

        let mut builder = CompilationEnvironment::portable(
            compiler(),
            grammar(),
            semantics(),
        );

        builder.add_text_fact(
            key("display.description"),
            "Build on machine A",
            EnvironmentFactImpact::Observational,
        ).unwrap();

        let second = builder.build().unwrap();

        assert_eq!(
            first.fingerprint(),
            second.fingerprint(),
        );

        assert_eq!(second.fact_count(), 1);
    }

    #[test]
    fn observational_and_semantic_fact_counts_are_distinct() {
        let mut builder = CompilationEnvironment::portable(
            compiler(),
            grammar(),
            semantics(),
        );

        builder.add_text_fact(
            key("compiler.mode"),
            "strict",
            EnvironmentFactImpact::OutputAffecting,
        ).unwrap();

        builder.add_text_fact(
            key("display.name"),
            "Development build",
            EnvironmentFactImpact::Observational,
        ).unwrap();

        let environment = builder.build().unwrap();

        assert_eq!(environment.fact_count(), 2);

        assert_eq!(
            environment.output_affecting_fact_count(),
            1,
        );
    }

    #[test]
    fn duplicate_environment_facts_are_rejected() {
        let mut builder = CompilationEnvironment::portable(
            compiler(),
            grammar(),
            semantics(),
        );

        builder.add_text_fact(
            key("compiler.mode"),
            "strict",
            EnvironmentFactImpact::OutputAffecting,
        ).unwrap();

        assert!(matches!(
            builder.add_text_fact(
                key("compiler.mode"),
                "permissive",
                EnvironmentFactImpact::OutputAffecting,
            ),
            Err(EnvironmentError::DuplicateFact { .. })
        ));
    }

    #[test]
    fn insertion_order_does_not_change_identity() {
        let mut first = CompilationEnvironment::portable(
            compiler(),
            grammar(),
            semantics(),
        );

        first.add_text_fact(
            key("z.domain"),
            "value-z",
            EnvironmentFactImpact::OutputAffecting,
        ).unwrap();

        first.add_text_fact(
            key("a.domain"),
            "value-a",
            EnvironmentFactImpact::OutputAffecting,
        ).unwrap();

        let first = first.build().unwrap();

        let mut second = CompilationEnvironment::portable(
            compiler(),
            grammar(),
            semantics(),
        );

        second.add_text_fact(
            key("a.domain"),
            "value-a",
            EnvironmentFactImpact::OutputAffecting,
        ).unwrap();

        second.add_text_fact(
            key("z.domain"),
            "value-z",
            EnvironmentFactImpact::OutputAffecting,
        ).unwrap();

        let second = second.build().unwrap();

        assert_eq!(
            first.fingerprint(),
            second.fingerprint(),
        );
    }

    #[test]
    fn different_environment_scopes_have_different_identities() {
        let portable = CompilationEnvironment::portable(
            compiler(),
            grammar(),
            semantics(),
        )
        .build()
        .unwrap();

        let targeted = CompilationEnvironment::target_specific(
            compiler(),
            grammar(),
            semantics(),
        )
        .target_fingerprint(fingerprint(4))
        .build()
        .unwrap();

        assert_ne!(
            portable.fingerprint(),
            targeted.fingerprint(),
        );
    }

    #[test]
    fn supports_future_toolchain_identifiers() {
        let mut builder = CompilationEnvironment::portable(
            compiler(),
            grammar(),
            semantics(),
        );

        builder.add_toolchain(
            ToolchainIdentity::new(
                key("future.photonic.compiler"),
                fingerprint(7),
            ),
        ).unwrap();

        let environment = builder.build().unwrap();

        assert_eq!(environment.toolchain_count(), 1);
    }

    #[test]
    fn supports_binary_environment_values() {
        let mut builder = CompilationEnvironment::portable(
            compiler(),
            grammar(),
            semantics(),
        );

        builder.add_fact(EnvironmentFact::new(
            key("binary.toolchain.descriptor"),
            vec![0, 1, 2, 255],
            EnvironmentFactImpact::OutputAffecting,
        )).unwrap();

        let environment = builder.build().unwrap();

        assert_eq!(
            environment.fact(&key("binary.toolchain.descriptor"))
                .unwrap()
                .value(),
            &[0, 1, 2, 255],
        );
    }

    #[test]
    fn empty_identifier_is_rejected() {
        assert!(matches!(
            EnvironmentKey::new(""),
            Err(EnvironmentError::InvalidIdentifier { .. })
        ));
    }

    #[test]
    fn whitespace_only_identifier_is_rejected() {
        assert!(matches!(
            EnvironmentKey::new("   "),
            Err(EnvironmentError::InvalidIdentifier { .. })
        ));
    }

    #[test]
    fn control_characters_are_rejected() {
        assert!(matches!(
            EnvironmentKey::new("invalid\nkey"),
            Err(EnvironmentError::InvalidIdentifier { .. })
        ));
    }

    #[test]
    fn long_identifiers_are_not_arbitrarily_rejected() {
        let value = format!("domain.{}", "x".repeat(16_384));

        assert!(EnvironmentKey::new(value).is_ok());
    }

    #[test]
    fn environment_identity_comparison_uses_fingerprints() {
        let first = portable_environment();
        let second = portable_environment();

        assert!(
            first.has_same_compilation_identity(&second)
        );
    }

    #[test]
    fn target_specific_identity_changes_when_target_changes() {
        let first = CompilationEnvironment::target_specific(
            compiler(),
            grammar(),
            semantics(),
        )
        .target_fingerprint(fingerprint(10))
        .build()
        .unwrap();

        let second = CompilationEnvironment::target_specific(
            compiler(),
            grammar(),
            semantics(),
        )
        .target_fingerprint(fingerprint(11))
        .build()
        .unwrap();

        assert_ne!(
            first.fingerprint(),
            second.fingerprint(),
        );
    }
}


#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Compiler — Incremental Fingerprints
//!
//! Path: src/compiler/incremental/fingerprints.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! PURPOSE
//!
//! Provide compiler-level fingerprint construction and composition using
//! the canonical ZUIR stable fingerprint implementation.
//!
//! This module produces fingerprints for:
//!
//! - source and binary inputs;
//! - compiler identities;
//! - compiler-stage identities;
//! - configuration and policy;
//! - ordered optimization/pass pipelines;
//! - semantic environments;
//! - serialization schemas;
//! - target identities;
//! - external toolchains;
//! - dependency inventories;
//! - complete compiler-stage configurations.
//!
//! ARCHITECTURAL AUTHORITY
//!
//! src/zuir/fingerprint/stable_hash.rs
//!     Owns the canonical fingerprint algorithm and encoding.
//!
//! src/zuir/incremental/cache_keys.rs
//!     Owns ZUIR cache-key derivation.
//!
//! src/zuir/incremental/dependencies.rs
//!     Owns logical dependency identities and dependency categories.
//!
//! src/backend/cache.rs
//!     Owns artifact-cache storage and integrity checking.
//!
//! src/frontend/semantic/incremental/
//!     Owns semantic-fact caching and semantic dependency invalidation.
//!
//! src/compiler/incremental/cache.rs
//!     Owns compiler-facing cache integration.
//!
//! OWNERSHIP
//!
//! This module owns:
//!
//! - compiler fingerprint input contracts;
//! - compiler-specific fingerprint composition;
//! - deterministic compiler dependency-inventory fingerprinting;
//! - compiler-stage configuration fingerprinting;
//! - explicit compiler identity fingerprinting;
//! - compiler-facing fingerprint errors.
//!
//! This module does not own:
//!
//! - the hashing algorithm;
//! - ZUIR cache-key derivation;
//! - semantic meaning;
//! - AST identity;
//! - dependency graph mutation;
//! - invalidation planning;
//! - artifact storage;
//! - artifact serialization;
//! - source parsing;
//! - IR verification;
//! - target discovery;
//! - physical hardware allocation;
//! - runtime execution.
//!
//! FINGERPRINT CORRECTNESS
//!
//! A fingerprint is only as complete as its canonical input.
//!
//! Every adapter must include all fields that can change the value
//! represented by its fingerprint.
//!
//! Do not use:
//!
//! - Debug formatting;
//! - pointer addresses;
//! - randomized hashers;
//! - wall-clock timestamps;
//! - process-local identifiers;
//! - unordered iteration;
//! - incidental machine identity;
//! - mutable global state.
//!
//! FINGERPRINT STABILITY
//!
//! The domain strings and field names in this module are part of the
//! fingerprint schema.
//!
//! An incompatible change to field selection or encoding requires a
//! deliberate schema-version change.
//!
//! SCALABILITY
//!
//! No universal limits are imposed on:
//!
//! - source bytes;
//! - dependency count;
//! - compiler stages;
//! - optimization passes;
//! - targets;
//! - toolchains;
//! - devices;
//! - processors;
//! - modules;
//! - operations;
//! - tensor dimensions;
//! - quantum resources.
//!
//! Actual operation is constrained by available memory, execution time,
//! address-space limits, and the caller's configurable resource policies.
//!
//! POCO-REAF
//!
//! Target-independent fingerprints must exclude incidental target identity.
//!
//! Target-specific fingerprints must include every target property that
//! can change the result.
//!
//! A target-independent fingerprint must not be reused as evidence that
//! a target-specific artifact is compatible.
//!
//! INTEGRATION
//!
//! The compiler pipeline should:
//!
//! 1. Obtain canonical source or IR fingerprints from their owners.
//! 2. Obtain compiler, configuration, pass, and semantic-environment
//!    fingerprints.
//! 3. Obtain dependency fingerprints from the relevant dependency owner.
//! 4. Derive a complete stage-configuration fingerprint.
//! 5. Supply the resulting fingerprints to compiler/incremental/cache.rs.
//! 6. Let the existing cache-key implementation derive the lookup key.
//! 7. Validate cached artifacts before reuse.
//!
//! This module does not decide whether an artifact is valid.

use std::collections::HashSet;
use std::error::Error;
use std::fmt;

use crate::zuir::fingerprint::stable_hash::{
    Fingerprint,
    FingerprintError as StableFingerprintError,
    StableHashBuilder,
};

use crate::zuir::incremental::{
    DependencyKey,
    DependencyKind,
};

// -----------------------------------------------------------------------------
// VERSIONED FINGERPRINT DOMAINS
// -----------------------------------------------------------------------------

/// Domain for raw source or binary input content.
///
/// The caller must still select the correct source identity and language
/// version when constructing higher-level compilation identities.
pub const INPUT_CONTENT_DOMAIN: &str =
    "zamani.compiler.input-content.v1";

/// Domain for compiler implementation identity.
pub const COMPILER_IDENTITY_DOMAIN: &str =
    "zamani.compiler.identity.v1";

/// Domain for compiler-stage identity.
pub const COMPILER_STAGE_DOMAIN: &str =
    "zamani.compiler.stage.v1";

/// Domain for compiler configuration.
pub const COMPILER_CONFIGURATION_DOMAIN: &str =
    "zamani.compiler.configuration.v1";

/// Domain for ordered pass pipelines.
pub const COMPILER_PASS_PIPELINE_DOMAIN: &str =
    "zamani.compiler.pass-pipeline.v1";

/// Domain for semantic environment identity.
pub const COMPILER_SEMANTIC_ENVIRONMENT_DOMAIN: &str =
    "zamani.compiler.semantic-environment.v1";

/// Domain for serialization schema identity.
pub const COMPILER_SERIALIZATION_SCHEMA_DOMAIN: &str =
    "zamani.compiler.serialization-schema.v1";

/// Domain for target identity.
pub const COMPILER_TARGET_DOMAIN: &str =
    "zamani.compiler.target.v1";

/// Domain for external toolchain identity.
pub const COMPILER_TOOLCHAIN_DOMAIN: &str =
    "zamani.compiler.toolchain.v1";

/// Domain for dependency inventory identity.
pub const COMPILER_DEPENDENCY_SET_DOMAIN: &str =
    "zamani.compiler.dependency-set.v1";

/// Domain for the complete configuration of a compiler stage.
pub const COMPILER_STAGE_CONFIGURATION_DOMAIN: &str =
    "zamani.compiler.stage-configuration.v1";

/// Domain for an ordered collection of named compiler components.
pub const COMPILER_COMPONENT_SET_DOMAIN: &str =
    "zamani.compiler.component-set.v1";

// -----------------------------------------------------------------------------
// PUBLIC CANONICAL FINGERPRINT TYPES
// -----------------------------------------------------------------------------

/// The canonical fingerprint type used throughout Zamani.
///
/// This is re-exported from the authoritative ZUIR fingerprint module.
/// No competing digest representation is introduced.
pub use crate::zuir::fingerprint::stable_hash::Fingerprint as CompilationFingerprint;

/// Errors produced by compiler fingerprint construction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompilationFingerprintError {
    /// The canonical fingerprint implementation rejected an input.
    StableFingerprint(StableFingerprintError),

    /// A logical dependency appeared more than once.
    DuplicateDependency {
        /// Exact duplicated logical dependency identity.
        key: String,
    },

    /// A collection count cannot be represented canonically.
    CountOverflow {
        /// Name of the collection whose count overflowed.
        collection: &'static str,
    },
}

impl fmt::Display for CompilationFingerprintError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StableFingerprint(error) => {
                write!(
                    formatter,
                    "compiler fingerprint construction failed: {error}"
                )
            }

            Self::DuplicateDependency { key } => {
                write!(
                    formatter,
                    "duplicate compiler fingerprint dependency: {key}"
                )
            }

            Self::CountOverflow { collection } => {
                write!(
                    formatter,
                    "compiler fingerprint {collection} count overflowed"
                )
            }
        }
    }
}

impl Error for CompilationFingerprintError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::StableFingerprint(error) => Some(error),
            Self::DuplicateDependency { .. }
            | Self::CountOverflow { .. } => None,
        }
    }
}

impl From<StableFingerprintError> for CompilationFingerprintError {
    fn from(error: StableFingerprintError) -> Self {
        Self::StableFingerprint(error)
    }
}

// -----------------------------------------------------------------------------
// BASIC CONTENT FINGERPRINTING
// -----------------------------------------------------------------------------

/// Fingerprint arbitrary input bytes.
///
/// Appropriate for source files, canonical serialized input, or another
/// byte sequence whose exact contents determine the input identity.
///
/// The caller remains responsible for including language, schema, or
/// format identity when those properties affect the compilation result.
pub fn fingerprint_bytes(
    domain: &str,
    bytes: &[u8],
) -> Result<Fingerprint, CompilationFingerprintError> {
    let mut builder = StableHashBuilder::new(domain)?;

    builder.field_bytes("content", bytes)?;

    Ok(builder.finish())
}

/// Fingerprint UTF-8 text using an explicit domain.
pub fn fingerprint_text(
    domain: &str,
    value: &str,
) -> Result<Fingerprint, CompilationFingerprintError> {
    let mut builder = StableHashBuilder::new(domain)?;

    builder.field_str("content", value)?;

    Ok(builder.finish())
}

/// Fingerprint source or binary input content using the compiler input
/// content domain.
///
/// This fingerprints exact bytes. It does not perform source normalization,
/// whitespace removal, parsing, or semantic equivalence analysis.
pub fn fingerprint_input_content(
    bytes: &[u8],
) -> Result<Fingerprint, CompilationFingerprintError> {
    fingerprint_bytes(INPUT_CONTENT_DOMAIN, bytes)
}

// -----------------------------------------------------------------------------
// NAMED FINGERPRINT FIELDS
// -----------------------------------------------------------------------------

/// One explicitly named fingerprint field.
///
/// The field name and ordering are part of the schema.
///
/// Callers must provide fields in the canonical order defined by the
/// corresponding fingerprint contract.
#[derive(Debug, Clone, Copy)]
pub struct FingerprintField<'a> {
    /// Stable field name.
    pub name: &'a str,

    /// Exact field bytes.
    pub value: &'a [u8],
}

impl<'a> FingerprintField<'a> {
    /// Construct a named byte field.
    #[must_use]
    pub const fn bytes(
        name: &'a str,
        value: &'a [u8],
    ) -> Self {
        Self { name, value }
    }

    /// Construct a named UTF-8 field.
    #[must_use]
    pub const fn text(
        name: &'a str,
        value: &'a str,
    ) -> Self {
        Self {
            name,
            value: value.as_bytes(),
        }
    }
}

/// Fingerprint an explicitly ordered set of named fields.
///
/// This function deliberately does not sort fields.
///
/// Field ordering is a schema decision and must be made by the caller.
/// Binary framing is delegated to StableHashBuilder.
pub fn fingerprint_fields(
    domain: &str,
    fields: &[FingerprintField<'_>],
) -> Result<Fingerprint, CompilationFingerprintError> {
    let count = u64::try_from(fields.len()).map_err(|_| {
        CompilationFingerprintError::CountOverflow {
            collection: "field",
        }
    })?;

    let mut builder = StableHashBuilder::new(domain)?;

    builder.field_u64("field_count", count)?;

    for field in fields {
        builder.field_bytes(field.name, field.value)?;
    }

    Ok(builder.finish())
}

// -----------------------------------------------------------------------------
// COMPILER IDENTITY
// -----------------------------------------------------------------------------

/// Complete identity inputs for one compiler implementation.
///
/// `release_identity` must identify the compiler release or build.
///
/// `component_fingerprint` must represent compiler components whose
/// implementation can affect the artifact being produced.
///
/// Examples of such components include:
///
/// - frontend implementation version;
/// - semantic analysis implementation version;
/// - IR lowering implementation version;
/// - optimizer implementation version;
/// - code-generation implementation version.
///
/// Do not use a process ID or a machine-local build directory as identity.
#[derive(Debug, Clone, Copy)]
pub struct CompilerIdentityInput<'a> {
    /// Stable compiler release or build identity.
    pub release_identity: &'a str,

    /// Fingerprint of relevant compiler component implementations.
    pub component_fingerprint: Fingerprint,

    /// Compiler ABI or plugin contract identity, where applicable.
    pub api_fingerprint: Fingerprint,
}

/// Derive the fingerprint of a compiler implementation identity.
pub fn fingerprint_compiler_identity(
    input: &CompilerIdentityInput<'_>,
) -> Result<Fingerprint, CompilationFingerprintError> {
    let mut builder =
        StableHashBuilder::new(COMPILER_IDENTITY_DOMAIN)?;

    builder.field_str(
        "release_identity",
        input.release_identity,
    )?;

    builder.field_fingerprint(
        "component_fingerprint",
        input.component_fingerprint,
    )?;

    builder.field_fingerprint(
        "api_fingerprint",
        input.api_fingerprint,
    )?;

    Ok(builder.finish())
}

// -----------------------------------------------------------------------------
// COMPILER STAGE IDENTITY
// -----------------------------------------------------------------------------

/// Identity of one extensible compiler stage.
///
/// This is an identifier contract, not a closed list of compiler stages.
///
/// Examples:
///
/// - frontend-validation;
/// - semantic-analysis;
/// - zuir-verification;
/// - classical-lowering;
/// - quantum-lowering;
/// - hdl-lowering;
/// - ai-lowering;
/// - optimization;
/// - wasm-emission;
/// - llvm-emission.
#[derive(Debug, Clone, Copy)]
pub struct CompilerStageIdentity<'a> {
    /// Stable extensible stage identifier.
    pub stage_id: &'a str,

    /// Version of the stage implementation.
    pub implementation_version: &'a str,

    /// Fingerprint of the stage's declared interface and contracts.
    pub contract_fingerprint: Fingerprint,
}

/// Derive a stable compiler-stage identity fingerprint.
pub fn fingerprint_stage_identity(
    input: &CompilerStageIdentity<'_>,
) -> Result<Fingerprint, CompilationFingerprintError> {
    let mut builder =
        StableHashBuilder::new(COMPILER_STAGE_DOMAIN)?;

    builder.field_str("stage_id", input.stage_id)?;

    builder.field_str(
        "implementation_version",
        input.implementation_version,
    )?;

    builder.field_fingerprint(
        "contract_fingerprint",
        input.contract_fingerprint,
    )?;

    Ok(builder.finish())
}

// -----------------------------------------------------------------------------
// CONFIGURATION AND POLICY
// -----------------------------------------------------------------------------

/// Fingerprint a compiler configuration or policy.
///
/// The caller must provide a canonical representation of every
/// configuration value that can affect the stage output.
///
/// Do not fingerprint a Debug representation of a configuration struct.
pub fn fingerprint_configuration(
    canonical_configuration: &[u8],
) -> Result<Fingerprint, CompilationFingerprintError> {
    fingerprint_bytes(
        COMPILER_CONFIGURATION_DOMAIN,
        canonical_configuration,
    )
}

/// Fingerprint an ordered pass pipeline.
///
/// Pass order is preserved because changing pass order can change
/// compilation results.
///
/// Each item should identify both the pass and its implementation version.
pub fn fingerprint_pass_pipeline(
    ordered_passes: &[&str],
) -> Result<Fingerprint, CompilationFingerprintError> {
    let count = u64::try_from(ordered_passes.len()).map_err(|_| {
        CompilationFingerprintError::CountOverflow {
            collection: "pass",
        }
    })?;

    let mut builder =
        StableHashBuilder::new(COMPILER_PASS_PIPELINE_DOMAIN)?;

    builder.field_u64("pass_count", count)?;

    for pass in ordered_passes {
        builder.field_str("pass", pass)?;
    }

    Ok(builder.finish())
}

/// Fingerprint the semantic environment that can affect compilation.
///
/// The canonical bytes must include the relevant language edition,
/// semantic-model version, dialect registrations, operation schemas,
/// semantic extensions, and other applicable semantic authorities.
///
/// Grammar and semantic authorities determine the content of this
/// representation. This function does not interpret grammar files.
pub fn fingerprint_semantic_environment(
    canonical_environment: &[u8],
) -> Result<Fingerprint, CompilationFingerprintError> {
    fingerprint_bytes(
        COMPILER_SEMANTIC_ENVIRONMENT_DOMAIN,
        canonical_environment,
    )
}

// -----------------------------------------------------------------------------
// SERIALIZATION SCHEMA
// -----------------------------------------------------------------------------

/// Fingerprint an artifact serialization contract.
///
/// Include format name, schema version, canonical encoding version,
/// and any other property that changes the serialized artifact.
pub fn fingerprint_serialization_schema(
    canonical_schema: &[u8],
) -> Result<Fingerprint, CompilationFingerprintError> {
    fingerprint_bytes(
        COMPILER_SERIALIZATION_SCHEMA_DOMAIN,
        canonical_schema,
    )
}

// -----------------------------------------------------------------------------
// TARGET AND TOOLCHAIN IDENTITY
// -----------------------------------------------------------------------------

/// Fingerprint a target-independent or target-specific realization
/// descriptor.
///
/// The supplied bytes must contain only properties that affect the
/// compilation result.
///
/// Do not include an incidental device serial number in a portable
/// target-independent compilation fingerprint.
pub fn fingerprint_target_identity(
    canonical_target_descriptor: &[u8],
) -> Result<Fingerprint, CompilationFingerprintError> {
    fingerprint_bytes(
        COMPILER_TARGET_DOMAIN,
        canonical_target_descriptor,
    )
}

/// Fingerprint an external toolchain identity.
///
/// Include relevant toolchain release, version, configuration, ABI,
/// component, and compatibility information.
pub fn fingerprint_toolchain_identity(
    canonical_toolchain_descriptor: &[u8],
) -> Result<Fingerprint, CompilationFingerprintError> {
    fingerprint_bytes(
        COMPILER_TOOLCHAIN_DOMAIN,
        canonical_toolchain_descriptor,
    )
}

// -----------------------------------------------------------------------------
// DEPENDENCY FINGERPRINTING
// -----------------------------------------------------------------------------

/// One dependency's logical identity, category, and current fingerprint.
///
/// The logical key identifies the prerequisite.
///
/// The fingerprint identifies the relevant content or version.
///
/// The dependency kind is preserved so that different dependency
/// relationships cannot accidentally collapse into the same inventory.
#[derive(Debug, Clone, Copy)]
pub struct StageDependencyFingerprint<'a> {
    /// Canonical logical dependency identity.
    pub key: &'a DependencyKey,

    /// Existing canonical dependency category.
    pub kind: DependencyKind,

    /// Fingerprint of the dependency's relevant current content.
    pub fingerprint: Fingerprint,
}

/// Return the stable name for an existing dependency category.
///
/// These names are part of the compiler dependency-fingerprint schema.
/// A change requires a deliberate schema-version decision.
#[must_use]
pub const fn dependency_kind_name(
    kind: DependencyKind,
) -> &'static str {
    match kind {
        DependencyKind::Source => "source",
        DependencyKind::Grammar => "grammar",
        DependencyKind::SymbolResolution => "symbol-resolution",
        DependencyKind::Type => "type",
        DependencyKind::Semantic => "semantic",
        DependencyKind::Ir => "ir",
        DependencyKind::Dialect => "dialect",
        DependencyKind::Configuration => "configuration",
        DependencyKind::Analysis => "analysis",
        DependencyKind::Transformation => "transformation",
        DependencyKind::Serialization => "serialization",
        DependencyKind::Extension => "extension",
    }
}

/// Derive a deterministic fingerprint for a dependency inventory.
///
/// Dependencies are sorted by logical key before hashing. Therefore,
/// insertion order does not affect the result.
///
/// Duplicate logical keys are rejected, even if their fingerprints
/// or categories happen to match.
///
/// Dependency graph mutation and invalidation remain owned by the
/// corresponding dependency subsystem.
pub fn fingerprint_dependencies(
    dependencies: &[StageDependencyFingerprint<'_>],
) -> Result<Fingerprint, CompilationFingerprintError> {
    let count = u64::try_from(dependencies.len()).map_err(|_| {
        CompilationFingerprintError::CountOverflow {
            collection: "dependency",
        }
    })?;

    let mut ordered: Vec<&StageDependencyFingerprint<'_>> =
        dependencies.iter().collect();

    ordered.sort_unstable_by(|left, right| {
        left.key.as_str().cmp(right.key.as_str())
    });

    for pair in ordered.windows(2) {
        if pair[0].key == pair[1].key {
            return Err(
                CompilationFingerprintError::DuplicateDependency {
                    key: pair[0].key.as_str().to_owned(),
                },
            );
        }
    }

    let mut builder =
        StableHashBuilder::new(COMPILER_DEPENDENCY_SET_DOMAIN)?;

    builder.field_u64("dependency_count", count)?;

    for dependency in ordered {
        builder.field_str(
            "dependency_identity",
            dependency.key.as_str(),
        )?;

        builder.field_str(
            "dependency_kind",
            dependency_kind_name(dependency.kind),
        )?;

        builder.field_fingerprint(
            "dependency_fingerprint",
            dependency.fingerprint,
        )?;
    }

    Ok(builder.finish())
}

// -----------------------------------------------------------------------------
// COMPONENT SET FINGERPRINTING
// -----------------------------------------------------------------------------

/// One compiler component identity.
///
/// Component identifiers are extensible strings, not closed enums.
#[derive(Debug, Clone, Copy)]
pub struct CompilerComponent<'a> {
    /// Stable component identifier.
    pub id: &'a str,

    /// Version or implementation identity.
    pub version: &'a str,
}

/// Fingerprint an unordered set of independently registered compiler
/// components.
///
/// Components are sorted by their stable identifier. Duplicate component
/// identifiers are rejected.
///
/// Use this for independently registered extensions or compiler providers.
/// Do not use it for an ordered pass pipeline.
pub fn fingerprint_component_set(
    components: &[CompilerComponent<'_>],
) -> Result<Fingerprint, CompilationFingerprintError> {
    let count = u64::try_from(components.len()).map_err(|_| {
        CompilationFingerprintError::CountOverflow {
            collection: "component",
        }
    })?;

    let mut ordered: Vec<&CompilerComponent<'_>> =
        components.iter().collect();

    ordered.sort_unstable_by(|left, right| {
        left.id.cmp(right.id)
    });

    let mut seen = HashSet::with_capacity(ordered.len());

    let mut builder =
        StableHashBuilder::new(COMPILER_COMPONENT_SET_DOMAIN)?;

    builder.field_u64("component_count", count)?;

    for component in ordered {
        if !seen.insert(component.id) {
            return Err(
                CompilationFingerprintError::DuplicateDependency {
                    key: component.id.to_owned(),
                },
            );
        }

        builder.field_str("component_id", component.id)?;

        builder.field_str(
            "component_version",
            component.version,
        )?;
    }

    Ok(builder.finish())
}

// -----------------------------------------------------------------------------
// COMPLETE STAGE CONFIGURATION
// -----------------------------------------------------------------------------

/// Complete fingerprint inputs for a compiler-stage configuration.
///
/// This combines configuration, pass ordering, semantic environment,
/// and serialization schema into one stable fingerprint.
///
/// This is useful when adapting the compiler fingerprint contracts to
/// the existing compiler-stage cache interface.
///
/// Direct input, compiler identity, dependency identity, target identity,
/// and toolchain identity remain separate inputs to the cache key.
#[derive(Debug, Clone, Copy)]
pub struct StageConfigurationInput {
    /// Canonical configuration and policy fingerprint.
    pub configuration: Fingerprint,

    /// Ordered pass-pipeline fingerprint.
    pub pass_pipeline: Fingerprint,

    /// Semantic environment fingerprint.
    pub semantic_environment: Fingerprint,

    /// Output serialization-schema fingerprint.
    pub serialization_schema: Fingerprint,
}

/// Derive a complete compiler-stage configuration fingerprint.
///
/// Every input is included explicitly. Optional information must not be
/// represented by an invented default fingerprint.
pub fn fingerprint_stage_configuration(
    input: &StageConfigurationInput,
) -> Result<Fingerprint, CompilationFingerprintError> {
    let mut builder =
        StableHashBuilder::new(COMPILER_STAGE_CONFIGURATION_DOMAIN)?;

    builder.field_fingerprint(
        "configuration",
        input.configuration,
    )?;

    builder.field_fingerprint(
        "pass_pipeline",
        input.pass_pipeline,
    )?;

    builder.field_fingerprint(
        "semantic_environment",
        input.semantic_environment,
    )?;

    builder.field_fingerprint(
        "serialization_schema",
        input.serialization_schema,
    )?;

    Ok(builder.finish())
}

// -----------------------------------------------------------------------------
// TESTS
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn fingerprint_of(value: &str) -> Fingerprint {
        fingerprint_text("zamani.compiler.tests.v1", value)
            .expect("test fingerprint must be valid")
    }

    fn dependency_key(value: &str) -> DependencyKey {
        DependencyKey::new(value)
            .expect("test dependency key must be valid")
    }

    #[test]
    fn identical_content_produces_identical_fingerprints() {
        let first = fingerprint_input_content(b"program")
            .expect("first fingerprint");

        let second = fingerprint_input_content(b"program")
            .expect("second fingerprint");

        assert_eq!(first, second);
    }

    #[test]
    fn different_content_produces_different_fingerprints() {
        let first = fingerprint_input_content(b"program-a")
            .expect("first fingerprint");

        let second = fingerprint_input_content(b"program-b")
            .expect("second fingerprint");

        assert_ne!(first, second);
    }

    #[test]
    fn domains_separate_identical_content() {
        let first = fingerprint_bytes("domain-a", b"same")
            .expect("first fingerprint");

        let second = fingerprint_bytes("domain-b", b"same")
            .expect("second fingerprint");

        assert_ne!(first, second);
    }

    #[test]
    fn field_boundaries_are_unambiguous() {
        let first = fingerprint_fields(
            "test.fields.v1",
            &[
                FingerprintField::text("first", "ab"),
                FingerprintField::text("second", "c"),
            ],
        )
        .expect("first fingerprint");

        let second = fingerprint_fields(
            "test.fields.v1",
            &[
                FingerprintField::text("first", "a"),
                FingerprintField::text("second", "bc"),
            ],
        )
        .expect("second fingerprint");

        assert_ne!(first, second);
    }

    #[test]
    fn pass_order_changes_pipeline_fingerprint() {
        let first = fingerprint_pass_pipeline(&[
            "verify:v1",
            "optimize:v1",
            "lower:v1",
        ])
        .expect("first pipeline");

        let second = fingerprint_pass_pipeline(&[
            "optimize:v1",
            "verify:v1",
            "lower:v1",
        ])
        .expect("second pipeline");

        assert_ne!(first, second);
    }

    #[test]
    fn dependency_insertion_order_does_not_change_fingerprint() {
        let key_a = dependency_key("module:a");
        let key_b = dependency_key("module:b");

        let first = [
            StageDependencyFingerprint {
                key: &key_a,
                kind: DependencyKind::Ir,
                fingerprint: fingerprint_of("a"),
            },
            StageDependencyFingerprint {
                key: &key_b,
                kind: DependencyKind::Type,
                fingerprint: fingerprint_of("b"),
            },
        ];

        let second = [
            StageDependencyFingerprint {
                key: &key_b,
                kind: DependencyKind::Type,
                fingerprint: fingerprint_of("b"),
            },
            StageDependencyFingerprint {
                key: &key_a,
                kind: DependencyKind::Ir,
                fingerprint: fingerprint_of("a"),
            },
        ];

        assert_eq!(
            fingerprint_dependencies(&first)
                .expect("first dependency fingerprint"),
            fingerprint_dependencies(&second)
                .expect("second dependency fingerprint"),
        );
    }

    #[test]
    fn dependency_kind_changes_fingerprint() {
        let key = dependency_key("module:a");

        let ir_dependency = [StageDependencyFingerprint {
            key: &key,
            kind: DependencyKind::Ir,
            fingerprint: fingerprint_of("content"),
        }];

        let semantic_dependency = [StageDependencyFingerprint {
            key: &key,
            kind: DependencyKind::Semantic,
            fingerprint: fingerprint_of("content"),
        }];

        assert_ne!(
            fingerprint_dependencies(&ir_dependency)
                .expect("IR dependency"),
            fingerprint_dependencies(&semantic_dependency)
                .expect("semantic dependency"),
        );
    }

    #[test]
    fn duplicate_dependency_keys_are_rejected() {
        let key_a = dependency_key("module:a");

        let dependencies = [
            StageDependencyFingerprint {
                key: &key_a,
                kind: DependencyKind::Ir,
                fingerprint: fingerprint_of("a"),
            },
            StageDependencyFingerprint {
                key: &key_a,
                kind: DependencyKind::Semantic,
                fingerprint: fingerprint_of("b"),
            },
        ];

        assert!(matches!(
            fingerprint_dependencies(&dependencies),
            Err(CompilationFingerprintError::DuplicateDependency { .. })
        ));
    }

    #[test]
    fn target_fingerprints_are_separate_from_portable_inputs() {
        let first = fingerprint_target_identity(b"target-a")
            .expect("first target");

        let second = fingerprint_target_identity(b"target-b")
            .expect("second target");

        assert_ne!(first, second);
    }

    #[test]
    fn stage_configuration_includes_every_component() {
        let base = StageConfigurationInput {
            configuration: fingerprint_of("configuration"),
            pass_pipeline: fingerprint_of("pipeline"),
            semantic_environment: fingerprint_of("semantics"),
            serialization_schema: fingerprint_of("schema"),
        };

        let original = fingerprint_stage_configuration(&base)
            .expect("original configuration");

        let changed = fingerprint_stage_configuration(
            &StageConfigurationInput {
                serialization_schema: fingerprint_of("schema-v2"),
                ..base
            },
        )
        .expect("changed configuration");

        assert_ne!(original, changed);
    }

    #[test]
    fn compiler_identity_changes_with_component_identity() {
        let first = CompilerIdentityInput {
            release_identity: "zamani-build-a",
            component_fingerprint: fingerprint_of("components-a"),
            api_fingerprint: fingerprint_of("api"),
        };

        let second = CompilerIdentityInput {
            component_fingerprint: fingerprint_of("components-b"),
            ..first
        };

        assert_ne!(
            fingerprint_compiler_identity(&first)
                .expect("first compiler identity"),
            fingerprint_compiler_identity(&second)
                .expect("second compiler identity"),
        );
    }

    #[test]
    fn empty_domain_is_rejected_by_canonical_builder() {
        assert!(fingerprint_bytes("", b"content").is_err());
    }

    #[test]
    fn empty_named_field_is_rejected() {
        assert!(
            fingerprint_fields(
                "test.fields.v1",
                &[FingerprintField::text("", "value")],
            )
            .is_err()
        );
    }
}

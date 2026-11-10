
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Compiler — Reproducible Compilation Inputs.
//!
//! Path: src/compiler/reproducibility/inputs.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Construct an immutable, deterministic identity for the complete set of
//! explicitly declared inputs that can affect a compilation result.
//!
//! This includes:
//!
//! - Source units.
//! - Declared compilation inputs.
//! - Resolved dependency identities.
//! - Compiler configuration and policies.
//! - Semantic compilation context.
//! - Canonical IR inputs.
//! - Domain-specific IR inputs.
//! - Build scripts and generated inputs.
//! - Compiler and provider component identities.
//! - The explicit compilation environment.
//!
//! # Canonical authorities
//!
//! Fingerprint algorithm:
//!     crate::zuir::fingerprint::stable_hash
//!
//! Compiler content fingerprints:
//!     crate::compiler::incremental::fingerprints
//!
//! Compilation environment:
//!     crate::compiler::reproducibility::environment
//!
//! Source syntax and reproducibility intent:
//!     grammar/compile/reproducibility.g4
//!
//! Deterministic-build intent:
//!     grammar/compile/deterministic-builds.g4
//!
//! Source semantics:
//!     crate::frontend::semantic
//!
//! Universal IR:
//!     crate::zuir
//!
//! Domain IR:
//!     The corresponding authoritative domain subsystem.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - Compilation input identity.
//! - Source-unit input identity.
//! - Named input fingerprint associations.
//! - Canonical input ordering.
//! - Duplicate input detection.
//! - Complete compilation-input fingerprint construction.
//!
//! This module does not own:
//!
//! - Source parsing.
//! - AST construction.
//! - Semantic interpretation.
//! - Dependency resolution.
//! - Dependency graph construction.
//! - Configuration interpretation.
//! - IR construction.
//! - IR verification.
//! - Environment discovery.
//! - Hardware discovery.
//! - Physical resource allocation.
//! - Backend selection.
//! - Artifact serialization.
//! - Artifact storage.
//! - Digital signatures.
//! - Runtime execution.
//!
//! # Determinism
//!
//! Input identity is independent of:
//!
//! - Insertion order.
//! - Hash-map randomization.
//! - Thread execution order.
//! - Process identifiers.
//! - Memory addresses.
//! - Wall-clock time.
//! - Filesystem enumeration order.
//! - Hostname.
//! - Locale.
//! - Incidental machine identity.
//!
//! Ordered maps and the canonical ZUIR fingerprint implementation
//! establish deterministic encoding.
//!
//! # Scalability
//!
//! No universal maximum is imposed on:
//!
//! - Source units.
//! - Source bytes.
//! - Dependencies.
//! - Declared inputs.
//! - IR components.
//! - Computing domains.
//! - Compiler components.
//! - Toolchains.
//! - Modules.
//! - Targets.
//! - Build inputs.
//!
//! Actual operation is constrained by available memory, execution time,
//! address-space limits and explicitly configured operational budgets.
//!
//! # Security
//!
//! Callers must never supply secrets as fingerprint inputs.
//!
//! A digest establishes content identity, not authenticity.
//!
//! Authentication, authorization and trust verification remain the
//! responsibility of compiler security and supply-chain authorities.
//!
//! # POCO-REAF
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! Portable compilation identity must not accidentally depend on a
//! particular physical machine.
//!
//! Target-specific properties must be represented through the explicit
//! CompilationEnvironment and the appropriate target-owned fingerprint.
//!
//! Physical resource allocation is not a compilation input unless an
//! explicitly declared compilation contract makes a particular property
//! relevant to the generated result.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use crate::compiler::incremental::fingerprints::{
    fingerprint_input_content,
    CompilationFingerprintError,
};

use crate::compiler::reproducibility::environment::{
    CompilationEnvironment,
};

use crate::zuir::fingerprint::stable_hash::{
    Fingerprint,
    FingerprintError,
    StableHashBuilder,
};

// ============================================================================
// VERSIONING
// ============================================================================

/// Public API version of the compilation-input identity contract.
///
/// Independent of:
///
/// - Zamani language version.
/// - Grammar version.
/// - Semantic-model version.
/// - Compiler release.
/// - ZUIR version.
/// - Domain IR versions.
/// - Artifact format version.
/// - Backend ABI versions.
pub const COMPILATION_INPUTS_API_VERSION: u16 = 1;

/// Version of the compilation-input fingerprint schema.
///
/// Increment when the selected fields, field ordering or interpretation
/// changes incompatibly.
pub const COMPILATION_INPUTS_SCHEMA_VERSION: u32 = 1;

/// Canonical fingerprint domain for complete compilation inputs.
pub const COMPILATION_INPUTS_FINGERPRINT_DOMAIN: &str =
    "zamani.compiler.reproducibility.inputs.v1";

/// Canonical fingerprint domain for one source unit.
pub const SOURCE_UNIT_FINGERPRINT_DOMAIN: &str =
    "zamani.compiler.reproducibility.source-unit.v1";

// ============================================================================
// INPUT CATEGORIES
// ============================================================================

/// Broad category of an explicitly supplied compilation input.
///
/// The category is extensible through its stable string representation.
/// It is deliberately not a closed enumeration of computing domains.
///
/// Examples:
///
/// - source;
/// - dependency;
/// - configuration;
/// - semantic-context;
/// - classical-ir;
/// - quantum-ir;
/// - hdl-ir;
/// - ai-ir;
/// - hybrid-ir;
/// - distributed-ir;
/// - build-script;
/// - generated-input;
/// - compiler-component;
/// - provider-component.
///
/// A category name is part of the fingerprint schema and must remain
/// stable for the lifetime of the corresponding identity contract.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InputCategory(String);

impl InputCategory {
    /// Construct a validated input category.
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, CompilationInputsError> {
        let value = value.into();

        validate_identifier(&value, "input category")?;

        Ok(Self(value))
    }

    /// Return the exact category identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for InputCategory {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// INPUT IDENTIFIERS
// ============================================================================

/// Stable logical identity of one compilation input.
///
/// This identifier is not necessarily a filesystem path.
///
/// Examples:
///
/// - module:application;
/// - package:library;
/// - dependency:math;
/// - generated:parser-table;
/// - quantum-module:simulation;
/// - hdl-module:controller.
///
/// The caller supplies the logical identity.
///
/// This module does not discover or resolve inputs.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InputIdentifier(String);

impl InputIdentifier {
    /// Construct a validated logical input identifier.
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, CompilationInputsError> {
        let value = value.into();

        validate_identifier(&value, "input identifier")?;

        Ok(Self(value))
    }

    /// Return the exact logical identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for InputIdentifier {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// SOURCE UNIT
// ============================================================================

/// Immutable identity of one source unit.
///
/// The logical identifier is distinct from the content fingerprint.
///
/// This distinction is important because two source units can contain
/// identical bytes while having different logical identities.
///
/// The logical identifier may affect module resolution and therefore
/// belongs to the complete compilation-input identity.
///
/// Source bytes are fingerprinted exactly as supplied.
///
/// No whitespace normalization, formatting normalization, parsing,
/// or semantic equivalence analysis is performed here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceUnit {
    identifier: InputIdentifier,
    content_fingerprint: Fingerprint,
}

impl SourceUnit {
    /// Construct a source unit from exact source bytes.
    ///
    /// The content fingerprint uses the repository's canonical compiler
    /// input-content fingerprint implementation.
    pub fn from_bytes(
        identifier: InputIdentifier,
        source: &[u8],
    ) -> Result<Self, CompilationInputsError> {
        let content_fingerprint =
            fingerprint_input_content(source)?;

        Ok(Self {
            identifier,
            content_fingerprint,
        })
    }

    /// Construct a source unit from an authoritative content fingerprint.
    ///
    /// Use this when another trusted subsystem has already fingerprinted
    /// the exact source bytes using the canonical input-content contract.
    ///
    /// The supplied fingerprint is not independently verified here.
    #[must_use]
    pub const fn from_fingerprint(
        identifier: InputIdentifier,
        content_fingerprint: Fingerprint,
    ) -> Self {
        Self {
            identifier,
            content_fingerprint,
        }
    }

    /// Return the logical source identifier.
    #[must_use]
    pub fn identifier(&self) -> &InputIdentifier {
        &self.identifier
    }

    /// Return the exact source-content fingerprint.
    #[must_use]
    pub const fn content_fingerprint(&self) -> Fingerprint {
        self.content_fingerprint
    }
}

// ============================================================================
// FINGERPRINTED INPUT
// ============================================================================

/// One explicitly identified compilation input.
///
/// The input is represented by an authoritative fingerprint instead of
/// retaining a second copy of potentially large input contents.
///
/// The fingerprint producer must use a canonical representation appropriate
/// to the input's owning subsystem.
///
/// Examples:
///
/// - A dependency fingerprint from dependency resolution.
/// - A configuration fingerprint from compiler configuration.
/// - A semantic-model fingerprint from frontend semantic analysis.
/// - A ZUIR fingerprint from the canonical ZUIR authority.
/// - A quantum IR fingerprint from quantum::ir.
/// - An HDL IR fingerprint from the HDL authority.
/// - A toolchain fingerprint from the corresponding provider.
///
/// The category and identifier are included in the complete input identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FingerprintedInput {
    category: InputCategory,
    identifier: InputIdentifier,
    fingerprint: Fingerprint,
}

impl FingerprintedInput {
    /// Construct a named input fingerprint association.
    #[must_use]
    pub const fn new(
        category: InputCategory,
        identifier: InputIdentifier,
        fingerprint: Fingerprint,
    ) -> Self {
        Self {
            category,
            identifier,
            fingerprint,
        }
    }

    /// Return the input category.
    #[must_use]
    pub fn category(&self) -> &InputCategory {
        &self.category
    }

    /// Return the logical input identifier.
    #[must_use]
    pub fn identifier(&self) -> &InputIdentifier {
        &self.identifier
    }

    /// Return the authoritative content fingerprint.
    #[must_use]
    pub const fn fingerprint(&self) -> Fingerprint {
        self.fingerprint
    }
}

// ============================================================================
// SOURCE INPUT IDENTITY
// ============================================================================

/// Immutable source identity included in a complete compilation input set.
///
/// Sources are stored in canonical logical-identifier order.
///
/// Source ordering in the caller's insertion sequence therefore cannot
/// accidentally change the fingerprint.
///
/// If source order is semantically meaningful for a particular compilation
/// contract, the caller must encode that order in an explicit semantic input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceInput {
    sources: BTreeMap<InputIdentifier, SourceUnit>,
}

impl SourceInput {
    /// Return the number of source units.
    #[must_use]
    pub fn len(&self) -> usize {
        self.sources.len()
    }

    /// Return whether no source units are present.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.sources.is_empty()
    }

    /// Retrieve a source unit by logical identity.
    #[must_use]
    pub fn get(
        &self,
        identifier: &InputIdentifier,
    ) -> Option<&SourceUnit> {
        self.sources.get(identifier)
    }

    /// Iterate source units in deterministic logical-identifier order.
    pub fn iter(
        &self,
    ) -> impl ExactSizeIterator<Item = (&InputIdentifier, &SourceUnit)> {
        self.sources.iter()
    }
}

// ============================================================================
// ERRORS
// ============================================================================

/// Errors produced by compilation-input construction.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CompilationInputsError {
    /// An identifier is empty or contains control characters.
    InvalidIdentifier {
        /// Description of the invalid identifier.
        description: &'static str,
    },

    /// A source unit has already been registered.
    DuplicateSource {
        /// Duplicated logical source identifier.
        identifier: String,
    },

    /// A named input has already been registered.
    DuplicateInput {
        /// Input category.
        category: String,

        /// Duplicated logical input identifier.
        identifier: String,
    },

    /// A collection count cannot be represented by the fingerprint schema.
    CountOverflow {
        /// Collection description.
        collection: &'static str,
    },

    /// Canonical ZUIR fingerprint construction failed.
    Fingerprinting(FingerprintError),

    /// Compiler input-content fingerprint construction failed.
    ContentFingerprinting(CompilationFingerprintError),
}

impl fmt::Display for CompilationInputsError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidIdentifier { description } => {
                write!(
                    formatter,
                    "invalid compilation input {description}"
                )
            }

            Self::DuplicateSource { identifier } => {
                write!(
                    formatter,
                    "duplicate compilation source identifier: {identifier}"
                )
            }

            Self::DuplicateInput {
                category,
                identifier,
            } => {
                write!(
                    formatter,
                    "duplicate compilation input: {category}:{identifier}"
                )
            }

            Self::CountOverflow { collection } => {
                write!(
                    formatter,
                    "compilation input {collection} count overflowed"
                )
            }

            Self::Fingerprinting(error) => {
                write!(
                    formatter,
                    "compilation input fingerprinting failed: {error}"
                )
            }

            Self::ContentFingerprinting(error) => {
                write!(
                    formatter,
                    "source content fingerprinting failed: {error}"
                )
            }
        }
    }
}

impl Error for CompilationInputsError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Fingerprinting(error) => Some(error),
            Self::ContentFingerprinting(error) => Some(error),
            _ => None,
        }
    }
}

impl From<FingerprintError> for CompilationInputsError {
    fn from(error: FingerprintError) -> Self {
        Self::Fingerprinting(error)
    }
}

impl From<CompilationFingerprintError> for CompilationInputsError {
    fn from(error: CompilationFingerprintError) -> Self {
        Self::ContentFingerprinting(error)
    }
}

// ============================================================================
// IDENTIFIER VALIDATION
// ============================================================================

fn validate_identifier(
    value: &str,
    description: &'static str,
) -> Result<(), CompilationInputsError> {
    if value.trim().is_empty() {
        return Err(CompilationInputsError::InvalidIdentifier {
            description,
        });
    }

    if value.chars().any(char::is_control) {
        return Err(CompilationInputsError::InvalidIdentifier {
            description,
        });
    }

    Ok(())
}

// ============================================================================
// COMPLETE COMPILATION INPUTS
// ============================================================================

/// Immutable, complete identity of explicitly supplied compilation inputs.
///
/// The complete identity consists of:
///
/// - Compilation environment fingerprint.
/// - Canonically ordered source units.
/// - Canonically ordered named input fingerprints.
/// - Complete compilation-input fingerprint.
///
/// This type does not assert that all possible inputs have been supplied.
///
/// The compiler pipeline and the authoritative owners of each input must
/// ensure that every output-affecting input is registered before using this
/// identity for caching, reproducibility verification or artifact identity.
///
/// Observational metadata must not be registered as an output-affecting
/// compilation input.
///
/// A compilation-input fingerprint is not a substitute for:
///
/// - Semantic validation.
/// - IR verification.
/// - Security verification.
/// - Supply-chain verification.
/// - Artifact validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompilationInputs {
    environment: CompilationEnvironment,
    sources: SourceInput,
    inputs: BTreeMap<(InputCategory, InputIdentifier), FingerprintedInput>,
    fingerprint: Fingerprint,
}

impl CompilationInputs {
    /// Return the explicit compilation environment.
    #[must_use]
    pub fn environment(&self) -> &CompilationEnvironment {
        &self.environment
    }

    /// Return the canonical source input collection.
    #[must_use]
    pub fn sources(&self) -> &SourceInput {
        &self.sources
    }

    /// Return the number of named fingerprinted inputs.
    #[must_use]
    pub fn input_count(&self) -> usize {
        self.inputs.len()
    }

    /// Return the number of source units.
    #[must_use]
    pub fn source_count(&self) -> usize {
        self.sources.len()
    }

    /// Return whether no named fingerprinted inputs are present.
    #[must_use]
    pub fn has_no_named_inputs(&self) -> bool {
        self.inputs.is_empty()
    }

    /// Return the complete compilation-input fingerprint.
    ///
    /// The fingerprint is deterministic for the same:
    ///
    /// - Environment identity.
    /// - Source identities and contents.
    /// - Named input identities and fingerprints.
    /// - Fingerprint schema version.
    ///
    /// It is not proof of semantic equivalence or authenticity.
    #[must_use]
    pub const fn fingerprint(&self) -> Fingerprint {
        self.fingerprint
    }

    /// Retrieve a named fingerprinted input.
    #[must_use]
    pub fn get(
        &self,
        category: &InputCategory,
        identifier: &InputIdentifier,
    ) -> Option<&FingerprintedInput> {
        self.inputs.get(&(category.clone(), identifier.clone()))
    }

    /// Iterate named inputs in deterministic category and identifier order.
    pub fn iter(
        &self,
    ) -> impl ExactSizeIterator<
        Item = (&(InputCategory, InputIdentifier), &FingerprintedInput),
    > {
        self.inputs.iter()
    }

    /// Return whether two input sets have identical compilation identity.
    ///
    /// This compares fingerprints. It does not prove semantic equivalence
    /// of arbitrary source programs.
    #[must_use]
    pub fn has_same_identity(
        &self,
        other: &Self,
    ) -> bool {
        self.fingerprint == other.fingerprint
    }
}

// ============================================================================
// COMPILATION INPUT BUILDER
// ============================================================================

/// Builder for immutable complete compilation-input identities.
///
/// The builder owns all registered source and fingerprint associations
/// until build() is called.
///
/// Duplicate logical identities are rejected.
///
/// BTreeMap provides deterministic canonical ordering.
///
/// No arbitrary cardinality limits are imposed.
#[derive(Debug)]
pub struct CompilationInputsBuilder {
    environment: CompilationEnvironment,
    sources: BTreeMap<InputIdentifier, SourceUnit>,
    inputs: BTreeMap<(InputCategory, InputIdentifier), FingerprintedInput>,
}

impl CompilationInputsBuilder {
    /// Construct a builder using an explicit compilation environment.
    ///
    /// The environment must already have been constructed and validated
    /// by the reproducibility environment authority.
    #[must_use]
    pub fn new(
        environment: CompilationEnvironment,
    ) -> Self {
        Self {
            environment,
            sources: BTreeMap::new(),
            inputs: BTreeMap::new(),
        }
    }

    /// Register a source unit from exact source bytes.
    ///
    /// The source bytes are fingerprinted immediately.
    ///
    /// The original source buffer is not retained by this builder.
    pub fn add_source(
        &mut self,
        identifier: InputIdentifier,
        source: &[u8],
    ) -> Result<&mut Self, CompilationInputsError> {
        let source_unit = SourceUnit::from_bytes(
            identifier,
            source,
        )?;

        self.add_source_unit(source_unit)
    }

    /// Register a source unit with an authoritative content fingerprint.
    ///
    /// This method is useful when source content has already been
    /// fingerprinted by the canonical input-content implementation.
    pub fn add_source_unit(
        &mut self,
        source: SourceUnit,
    ) -> Result<&mut Self, CompilationInputsError> {
        let identifier = source.identifier.clone();

        if self.sources.contains_key(&identifier) {
            return Err(CompilationInputsError::DuplicateSource {
                identifier: identifier.as_str().to_owned(),
            });
        }

        self.sources.insert(identifier, source);

        Ok(self)
    }

    /// Register a named authoritative fingerprint.
    ///
    /// Use this method for:
    ///
    /// - Dependencies.
    /// - Configuration and policies.
    /// - Semantic context.
    /// - Canonical ZUIR.
    /// - Classical IR.
    /// - Quantum IR.
    /// - HDL IR.
    /// - AI and tensor representations.
    /// - Hybrid and distributed representations.
    /// - Build scripts.
    /// - Generated inputs.
    /// - Compiler components.
    /// - Provider components.
    ///
    /// The caller must obtain the fingerprint from the subsystem that
    /// owns the corresponding representation.
    ///
    /// This module does not reinterpret the supplied fingerprint.
    pub fn add_fingerprinted_input(
        &mut self,
        input: FingerprintedInput,
    ) -> Result<&mut Self, CompilationInputsError> {
        let key = (
            input.category.clone(),
            input.identifier.clone(),
        );

        if self.inputs.contains_key(&key) {
            return Err(CompilationInputsError::DuplicateInput {
                category: input.category.as_str().to_owned(),
                identifier: input.identifier.as_str().to_owned(),
            });
        }

        self.inputs.insert(key, input);

        Ok(self)
    }

    /// Register a dependency fingerprint.
    ///
    /// Dependency resolution and dependency semantics remain owned by
    /// the frontend/package/dependency authorities.
    pub fn add_dependency(
        &mut self,
        identifier: InputIdentifier,
        fingerprint: Fingerprint,
    ) -> Result<&mut Self, CompilationInputsError> {
        self.add_fingerprinted_input(
            FingerprintedInput::new(
                InputCategory::new("dependency")?,
                identifier,
                fingerprint,
            ),
        )
    }

    /// Register a compiler configuration fingerprint.
    ///
    /// The caller must provide a canonical configuration fingerprint.
    ///
    /// Debug formatting is not a canonical configuration encoding.
    pub fn add_configuration(
        &mut self,
        identifier: InputIdentifier,
        fingerprint: Fingerprint,
    ) -> Result<&mut Self, CompilationInputsError> {
        self.add_fingerprinted_input(
            FingerprintedInput::new(
                InputCategory::new("configuration")?,
                identifier,
                fingerprint,
            ),
        )
    }

    /// Register a semantic-context fingerprint.
    ///
    /// The frontend semantic subsystem remains the authority for semantic
    /// types, effects, capabilities, requirements, constraints and policies.
    pub fn add_semantic_context(
        &mut self,
        identifier: InputIdentifier,
        fingerprint: Fingerprint,
    ) -> Result<&mut Self, CompilationInputsError> {
        self.add_fingerprinted_input(
            FingerprintedInput::new(
                InputCategory::new("semantic-context")?,
                identifier,
                fingerprint,
            ),
        )
    }

    /// Register a canonical universal IR fingerprint.
    ///
    /// The ZUIR subsystem remains the owner of its IR representation.
    pub fn add_universal_ir(
        &mut self,
        identifier: InputIdentifier,
        fingerprint: Fingerprint,
    ) -> Result<&mut Self, CompilationInputsError> {
        self.add_fingerprinted_input(
            FingerprintedInput::new(
                InputCategory::new("universal-ir")?,
                identifier,
                fingerprint,
            ),
        )
    }

    /// Register a domain-specific IR fingerprint.
    ///
    /// `domain` must be the stable identifier owned by the corresponding
    /// domain integration contract.
    ///
    /// Examples include classical-ir, quantum-ir, hdl-ir, ai-ir,
    /// hybrid-ir and distributed-ir.
    ///
    /// This is intentionally open-ended.
    pub fn add_domain_ir(
        &mut self,
        domain: impl Into<String>,
        identifier: InputIdentifier,
        fingerprint: Fingerprint,
    ) -> Result<&mut Self, CompilationInputsError> {
        let category = InputCategory::new(domain)?;

        self.add_fingerprinted_input(
            FingerprintedInput::new(
                category,
                identifier,
                fingerprint,
            ),
        )
    }

    /// Register a compiler or provider component fingerprint.
    ///
    /// Component identity must include the implementation and interface
    /// contract versions relevant to the resulting compilation output.
    pub fn add_component(
        &mut self,
        identifier: InputIdentifier,
        fingerprint: Fingerprint,
    ) -> Result<&mut Self, CompilationInputsError> {
        self.add_fingerprinted_input(
            FingerprintedInput::new(
                InputCategory::new("compiler-component")?,
                identifier,
                fingerprint,
            ),
        )
    }

    /// Construct the immutable complete compilation-input identity.
    ///
    /// The environment, source units and named input fingerprints are
    /// encoded in deterministic canonical order.
    ///
    /// All counts are checked before encoding.
    pub fn build(self) -> Result<CompilationInputs, CompilationInputsError> {
        let fingerprint = fingerprint_compilation_inputs(
            &self.environment,
            &self.sources,
            &self.inputs,
        )?;

        Ok(CompilationInputs {
            environment: self.environment,
            sources: SourceInput {
                sources: self.sources,
            },
            inputs: self.inputs,
            fingerprint,
        })
    }
}

// ============================================================================
// CANONICAL FINGERPRINT CONSTRUCTION
// ============================================================================

/// Convert a collection size to the canonical fingerprint count type.
///
/// This does not impose a semantic maximum.
///
/// It only rejects a value that cannot be represented by the fingerprint
/// protocol's explicitly defined unsigned count field.
fn canonical_count(
    count: usize,
    collection: &'static str,
) -> Result<u64, CompilationInputsError> {
    u64::try_from(count).map_err(|_| {
        CompilationInputsError::CountOverflow {
            collection,
        }
    })
}

/// Construct the complete canonical compilation-input fingerprint.
///
/// Field order is part of the fingerprint schema.
///
/// BTreeMap iteration guarantees stable ordering.
///
/// The existing ZUIR StableHashBuilder owns canonical byte framing,
/// length encoding, SHA-256 and checked byte/field accounting.
///
/// Do not change field order without an explicit schema-version decision.
fn fingerprint_compilation_inputs(
    environment: &CompilationEnvironment,
    sources: &BTreeMap<InputIdentifier, SourceUnit>,
    inputs: &BTreeMap<
        (InputCategory, InputIdentifier),
        FingerprintedInput,
    >,
) -> Result<Fingerprint, CompilationInputsError> {
    let source_count = canonical_count(
        sources.len(),
        "source",
    )?;

    let input_count = canonical_count(
        inputs.len(),
        "named input",
    )?;

    let mut builder = StableHashBuilder::new(
        COMPILATION_INPUTS_FINGERPRINT_DOMAIN,
    )?;

    builder.field_u32(
        "schema_version",
        COMPILATION_INPUTS_SCHEMA_VERSION,
    )?;

    builder.field_fingerprint(
        "environment_fingerprint",
        environment.fingerprint(),
    )?;

    builder.field_u64(
        "source_count",
        source_count,
    )?;

    for (identifier, source) in sources {
        builder.field_str(
            "source_identifier",
            identifier.as_str(),
        )?;

        builder.field_fingerprint(
            "source_content_fingerprint",
            source.content_fingerprint(),
        )?;
    }

    builder.field_u64(
        "named_input_count",
        input_count,
    )?;

    for ((category, identifier), input) in inputs {
        builder.field_str(
            "input_category",
            category.as_str(),
        )?;

        builder.field_str(
            "input_identifier",
            identifier.as_str(),
        )?;

        builder.field_fingerprint(
            "input_fingerprint",
            input.fingerprint(),
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

    fn test_fingerprint(value: u8) -> Fingerprint {
        Fingerprint::from_bytes([value; 32])
    }

    fn test_environment() -> CompilationEnvironment {
        CompilationEnvironment::portable(
            test_fingerprint(1),
            test_fingerprint(2),
            test_fingerprint(3),
        )
        .build()
        .expect("test environment must build")
    }

    fn identifier(value: &str) -> InputIdentifier {
        InputIdentifier::new(value)
            .expect("test identifier must be valid")
    }

    fn category(value: &str) -> InputCategory {
        InputCategory::new(value)
            .expect("test category must be valid")
    }

    #[test]
    fn input_category_rejects_empty_identifier() {
        assert!(InputCategory::new("").is_err());
        assert!(InputCategory::new("   ").is_err());
    }

    #[test]
    fn input_identifier_rejects_control_characters() {
        assert!(InputIdentifier::new("module\nname").is_err());
    }

    #[test]
    fn source_content_fingerprint_is_deterministic() {
        let first = SourceUnit::from_bytes(
            identifier("module:main"),
            b"fn main() {}",
        )
        .expect("source must fingerprint");

        let second = SourceUnit::from_bytes(
            identifier("module:main"),
            b"fn main() {}",
        )
        .expect("source must fingerprint");

        assert_eq!(
            first.content_fingerprint(),
            second.content_fingerprint(),
        );
    }

    #[test]
    fn different_source_contents_produce_different_fingerprints() {
        let first = SourceUnit::from_bytes(
            identifier("module:main"),
            b"fn main() {}",
        )
        .expect("source must fingerprint");

        let second = SourceUnit::from_bytes(
            identifier("module:main"),
            b"fn main() { return; }",
        )
        .expect("source must fingerprint");

        assert_ne!(
            first.content_fingerprint(),
            second.content_fingerprint(),
        );
    }

    #[test]
    fn duplicate_sources_are_rejected() {
        let mut builder =
            CompilationInputsBuilder::new(test_environment());

        builder
            .add_source(identifier("module:main"), b"fn main() {}")
            .expect("first source must be accepted");

        let result = builder.add_source(
            identifier("module:main"),
            b"fn other() {}",
        );

        assert!(matches!(
            result,
            Err(CompilationInputsError::DuplicateSource { .. })
        ));
    }

    #[test]
    fn duplicate_named_inputs_are_rejected() {
        let mut builder =
            CompilationInputsBuilder::new(test_environment());

        builder
            .add_dependency(
                identifier("package:math"),
                test_fingerprint(4),
            )
            .expect("first dependency must be accepted");

        let result = builder.add_dependency(
            identifier("package:math"),
            test_fingerprint(5),
        );

        assert!(matches!(
            result,
            Err(CompilationInputsError::DuplicateInput { .. })
        ));
    }

    #[test]
    fn insertion_order_does_not_change_complete_identity() {
        let mut first =
            CompilationInputsBuilder::new(test_environment());

        first
            .add_source(identifier("module:a"), b"source a")
            .expect("source a");

        first
            .add_source(identifier("module:b"), b"source b")
            .expect("source b");

        first
            .add_dependency(
                identifier("package:alpha"),
                test_fingerprint(10),
            )
            .expect("dependency alpha");

        first
            .add_dependency(
                identifier("package:beta"),
                test_fingerprint(11),
            )
            .expect("dependency beta");

        let first = first.build().expect("first input set");

        let mut second =
            CompilationInputsBuilder::new(test_environment());

        second
            .add_dependency(
                identifier("package:beta"),
                test_fingerprint(11),
            )
            .expect("dependency beta");

        second
            .add_source(identifier("module:b"), b"source b")
            .expect("source b");

        second
            .add_dependency(
                identifier("package:alpha"),
                test_fingerprint(10),
            )
            .expect("dependency alpha");

        second
            .add_source(identifier("module:a"), b"source a")
            .expect("source a");

        let second = second.build().expect("second input set");

        assert_eq!(
            first.fingerprint(),
            second.fingerprint(),
        );
    }

    #[test]
    fn environment_changes_change_complete_identity() {
        let mut first =
            CompilationInputsBuilder::new(test_environment());

        first
            .add_source(identifier("module:main"), b"fn main() {}")
            .expect("source");

        let first = first.build().expect("first input set");

        let other_environment = CompilationEnvironment::portable(
            test_fingerprint(21),
            test_fingerprint(22),
            test_fingerprint(23),
        )
        .build()
        .expect("other environment");

        let mut second =
            CompilationInputsBuilder::new(other_environment);

        second
            .add_source(identifier("module:main"), b"fn main() {}")
            .expect("source");

        let second = second.build().expect("second input set");

        assert_ne!(
            first.fingerprint(),
            second.fingerprint(),
        );
    }

    #[test]
    fn source_identity_changes_complete_identity() {
        let mut first =
            CompilationInputsBuilder::new(test_environment());

        first
            .add_source(identifier("module:a"), b"same bytes")
            .expect("source");

        let first = first.build().expect("first input set");

        let mut second =
            CompilationInputsBuilder::new(test_environment());

        second
            .add_source(identifier("module:b"), b"same bytes")
            .expect("source");

        let second = second.build().expect("second input set");

        assert_ne!(
            first.fingerprint(),
            second.fingerprint(),
        );
    }

    #[test]
    fn named_input_fingerprint_changes_complete_identity() {
        let mut first =
            CompilationInputsBuilder::new(test_environment());

        first
            .add_dependency(
                identifier("package:math"),
                test_fingerprint(31),
            )
            .expect("dependency");

        let first = first.build().expect("first input set");

        let mut second =
            CompilationInputsBuilder::new(test_environment());

        second
            .add_dependency(
                identifier("package:math"),
                test_fingerprint(32),
            )
            .expect("dependency");

        let second = second.build().expect("second input set");

        assert_ne!(
            first.fingerprint(),
            second.fingerprint(),
        );
    }

    #[test]
    fn domain_ir_inputs_remain_extensible() {
        let mut builder =
            CompilationInputsBuilder::new(test_environment());

        builder
            .add_domain_ir(
                "future-domain-ir",
                identifier("module:future"),
                test_fingerprint(41),
            )
            .expect("future domain must be accepted");

        let inputs = builder.build().expect("input set");

        assert_eq!(inputs.input_count(), 1);
    }

    #[test]
    fn source_collection_is_deterministically_ordered() {
        let mut builder =
            CompilationInputsBuilder::new(test_environment());

        builder
            .add_source(identifier("module:z"), b"z")
            .expect("source z");

        builder
            .add_source(identifier("module:a"), b"a")
            .expect("source a");

        let inputs = builder.build().expect("input set");

        let identifiers: Vec<&str> = inputs
            .sources()
            .iter()
            .map(|(identifier, _)| identifier.as_str())
            .collect();

        assert_eq!(
            identifiers,
            vec!["module:a", "module:z"],
        );
    }

    #[test]
    fn empty_input_set_is_representable() {
        let inputs = CompilationInputsBuilder::new(test_environment())
            .build()
            .expect("empty input set must be representable");

        assert_eq!(inputs.source_count(), 0);
        assert_eq!(inputs.input_count(), 0);
    }

    #[test]
    fn complete_identity_is_stable_across_repeated_builds() {
        let mut first =
            CompilationInputsBuilder::new(test_environment());

        first
            .add_source(identifier("module:main"), b"fn main() {}")
            .expect("source");

        let first = first.build().expect("first build");

        let mut second =
            CompilationInputsBuilder::new(test_environment());

        second
            .add_source(identifier("module:main"), b"fn main() {}")
            .expect("source");

        let second = second.build().expect("second build");

        assert!(first.has_same_identity(&second));
    }
}

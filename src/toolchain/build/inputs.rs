
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Canonical Build Inputs
//!
//! Path: `src/toolchain/build/inputs.rs`
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//!
//! ## Purpose
//!
//! Construct an immutable, deterministic snapshot of the inputs that
//! participate in one logical Zamani build and its target-specific
//! realizations.
//!
//! This module composes existing authoritative input identities. It does
//! not implement another source scanner, manifest parser, dependency
//! resolver, compiler fingerprint system, environment detector, cache,
//! provenance engine, target registry or hardware discovery service.
//!
//! ## Canonical authorities
//!
//! Workspace and source inventory:
//!     crate::toolchain::workspace::metadata::WorkspaceMetadata
//!
//! Manifest parsing and interpretation:
//!     crate::zamani_project_config
//!     crate::toolchain::manifest
//!
//! Dependency and lockfile identity:
//!     crate::toolchain::dependencies
//!
//! Effective toolchain configuration:
//!     crate::toolchain::configuration
//!
//! Compiler compilation-input identity:
//!     crate::compiler::reproducibility::inputs
//!
//! Compiler environment identity:
//!     crate::compiler::reproducibility::environment
//!
//! Canonical fingerprint implementation:
//!     crate::zuir::fingerprint::stable_hash
//!
//! Target identity and feasibility:
//!     crate::compiler::planning
//!
//! Provenance and trust:
//!     crate::compiler::reproducibility::provenance
//!     crate::compiler::security
//!
//! Incremental compilation and cache decisions:
//!     crate::compiler::incremental
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - The immutable build-input snapshot.
//! - Open-ended named build-input associations.
//! - Logical versus target-realization input scope.
//! - Deterministic input ordering.
//! - Duplicate input detection.
//! - Logical build-input fingerprint composition.
//! - Target-realization fingerprint composition.
//! - Complete build-input snapshot fingerprint composition.
//!
//! This module does not own:
//!
//! - Filesystem discovery or source reading.
//! - Manifest parsing or semantic validation.
//! - Dependency resolution or lockfile generation.
//! - Configuration interpretation or profile inheritance.
//! - Source-language compilation-profile semantics.
//! - Requirements, capabilities, preferences, constraints or hints.
//! - Target discovery, capability matching or feasibility analysis.
//! - Physical resource planning or allocation.
//! - Compiler or backend implementation.
//! - Cache storage, admission, eviction or invalidation.
//! - Artifact creation, verification or signing.
//! - Provenance authentication.
//! - Runtime execution or deployment.
//!
//! ## Identity model
//!
//! A logical build identity describes the project and its target-independent
//! build inputs.
//!
//! A target-realization identity describes the logical build plus the
//! explicitly supplied inputs for one target realization.
//!
//! A complete snapshot fingerprint commits to the logical identity and all
//! target-realization identities represented by the snapshot.
//!
//! A fingerprint is content identity only. It is not proof of authenticity,
//! semantic equivalence, target feasibility, successful compilation,
//! artifact validity or runtime compatibility.
//!
//! ## POCO-REAF
//!
//! One source project can have multiple target-specific realizations.
//!
//! Target-specific compiler environments, provider identities and other
//! output-affecting facts must be recorded in the corresponding realization.
//! They must not silently contaminate the target-independent logical identity.
//!
//! The model is open to classical, quantum, HDL, AI/tensor, hybrid,
//! distributed, networking, cryptographic and future provider categories.
//!
//! No fixed maximum is imposed on projects, source units, dependencies,
//! input categories, providers, targets, domains, devices, accelerators,
//! quantum resources, tensor dimensions or distributed participants.
//!
//! Actual operation remains subject to available memory, address space,
//! execution time, the fingerprint protocol's representational limits and
//! explicitly configured operational budgets. Literal infinite computation
//! or storage is not promised.
//!
//! ## Determinism
//!
//! Records are maintained in canonical order by:
//!
//!     scope -> category -> identifier
//!
//! Authority identifiers and fingerprints are included in the encoding.
//! No timestamps, random values, process IDs, hostnames, memory addresses,
//! debug formatting, implicit environment variables or filesystem
//! enumeration order are used.
//!
//! ## Security
//!
//! Callers must not add credentials, private keys, passwords, access tokens,
//! secrets or confidential raw data as input identifiers or fingerprint
//! values.
//!
//! An authority identifier records which subsystem supplied a fingerprint.
//! It does not authenticate that subsystem or verify its evidence.
//! Authentication, authorization, integrity and provenance verification
//! remain the responsibility of the canonical security authorities.
//!
//! ## Integration
//!
//! Inputs:
//!     validated WorkspaceMetadata
//!     authoritative fingerprints from manifest/dependency/configuration/
//!     target/provider/compiler services
//!
//! Output:
//!     immutable BuildInputs
//!
//! Consumers:
//!     build::plan
//!     build::graph
//!     build::executor
//!     build::incremental
//!     build::reporting
//!
//! The build executor must still obtain actual source text through its
//! BuildSourceProvider. This snapshot does not retain source buffers.
//!
//! ## Fingerprint schema
//!
//! The build-input fingerprint schema is independently versioned.
//! Changes to selected fields, field order, scope semantics or encoding
//! require an explicit schema-version review.
//!
//! Compiler input fingerprints remain owned by the compiler reproducibility
//! subsystem. This module only incorporates their canonical fingerprints.

use std::cmp::Ordering;
use std::error::Error;
use std::fmt;

use crate::compiler::reproducibility::inputs::CompilationInputs;
use crate::toolchain::errors::{
    ToolchainError,
    ToolchainErrorKind,
};
use crate::toolchain::result::ToolchainResult;
use crate::toolchain::workspace::metadata::WorkspaceMetadata;
use crate::zuir::fingerprint::stable_hash::{
    Fingerprint,
    StableHashBuilder,
};

use crate::compiler::reproducibility::inputs::{
    InputCategory,
    InputIdentifier,
};

// ============================================================================
// VERSIONING
// ============================================================================

/// Public API version of the build-input snapshot contract.
///
/// Independent of the language, grammar, compiler, manifest, package,
/// artifact, backend and runtime versions.
pub const BUILD_INPUTS_API_VERSION: u16 = 1;

/// Version of the canonical build-input fingerprint schema.
///
/// Increment when the fingerprint field selection, order, scope semantics
/// or interpretation changes incompatibly.
pub const BUILD_INPUTS_SCHEMA_VERSION: u32 = 1;

/// Canonical domain for target-independent logical build inputs.
pub const LOGICAL_BUILD_INPUTS_DOMAIN: &str =
    "zamani.toolchain.build.inputs.logical.v1";

/// Canonical domain for one target realization's inputs.
pub const REALIZATION_BUILD_INPUTS_DOMAIN: &str =
    "zamani.toolchain.build.inputs.realization.v1";

/// Canonical domain for the complete build-input snapshot.
pub const COMPLETE_BUILD_INPUTS_DOMAIN: &str =
    "zamani.toolchain.build.inputs.complete.v1";

const INPUTS_OPERATION: &str = "build.inputs";

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

fn input_error(
    kind: ToolchainErrorKind,
    message: impl Into<String>,
) -> ToolchainError {
    ToolchainError::new(kind, message)
        .with_operation(INPUTS_OPERATION)
}

fn fingerprint_error(
    error: impl Error + Send + Sync + 'static,
) -> ToolchainError {
    input_error(
        ToolchainErrorKind::Internal,
        "canonical build-input fingerprint construction failed",
    )
    .with_source(error)
}

fn allocation_error(
    subject: &str,
    error: std::collections::TryReserveError,
) -> ToolchainError {
    input_error(
        ToolchainErrorKind::ResourceExhausted,
        format!(
            "unable to reserve memory for {subject}: {error}"
        ),
    )
}

fn validate_nonempty_identifier(
    value: &str,
    description: &'static str,
) -> ToolchainResult<()> {
    if value.trim().is_empty() {
        return Err(input_error(
            ToolchainErrorKind::InvalidInput,
            format!("{description} must not be empty"),
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(input_error(
            ToolchainErrorKind::InvalidInput,
            format!("{description} must not contain control characters"),
        ));
    }

    Ok(())
}

fn canonical_count(
    value: usize,
    description: &'static str,
) -> ToolchainResult<u64> {
    u64::try_from(value).map_err(|_| {
        input_error(
            ToolchainErrorKind::ResourceExhausted,
            format!(
                "{description} cannot be represented by the fingerprint schema"
            ),
        )
    })
}

// ============================================================================
// INPUT SCOPE
// ============================================================================

/// Scope of a build input.
///
/// Logical inputs participate in the target-independent logical build
/// identity.
///
/// Target-realization inputs participate only in the identity of the
/// explicitly named realization.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum BuildInputScope {
    /// An input that belongs to the logical project/build identity.
    ///
    /// Examples include:
    ///
    /// - Validated manifest identity.
    /// - Resolved dependency graph identity.
    /// - Lockfile identity.
    /// - Effective target-independent build configuration.
    /// - Feature-resolution identity.
    /// - Target-independent semantic input identity.
    Logical,

    /// An input belonging to one target-specific realization.
    ///
    /// The identifier must be a stable identity supplied by the
    /// authoritative target/build planning layer. It must not be a
    /// display label, insertion index or transient process identifier.
    ///
    /// Examples include a canonical target realization identity,
    /// a target-specific compiler environment, or a provider realization.
    TargetRealization(String),
}

impl BuildInputScope {
    /// Construct a target-realization scope from a stable external identity.
    ///
    /// # Errors
    ///
    /// Returns an error when the supplied identity is empty or contains
    /// control characters.
    pub fn target_realization(
        identity: impl Into<String>,
    ) -> ToolchainResult<Self> {
        let identity = identity.into();

        validate_nonempty_identifier(
            &identity,
            "target realization identity",
        )?;

        Ok(Self::TargetRealization(identity))
    }

    /// Return whether this is a logical, target-independent input.
    #[must_use]
    pub const fn is_logical(&self) -> bool {
        matches!(self, Self::Logical)
    }

    /// Borrow the target-realization identity, if present.
    #[must_use]
    pub fn realization_identity(&self) -> Option<&str> {
        match self {
            Self::Logical => None,
            Self::TargetRealization(identity) => Some(identity),
        }
    }
}

// ============================================================================
// INPUT AUTHORITY
// ============================================================================

/// Stable identifier of the subsystem that supplied an input fingerprint.
///
/// This records declared provenance for the input value. It does not
/// authenticate the producer or prove that the supplied fingerprint was
/// correctly calculated.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BuildInputAuthority(String);

impl BuildInputAuthority {
    /// Construct an authority identifier.
    ///
    /// Use a stable subsystem or provider contract identifier, optionally
    /// including its contract version.
    ///
    /// Examples:
    ///
    /// - `zamani.manifest.validation.v1`
    /// - `zamani.dependencies.resolution.v1`
    /// - `zamani.configuration.effective.v1`
    /// - `zamani.compiler.reproducibility.inputs.v1`
    /// - `provider.example.compiler-contract.v2`
    ///
    /// # Errors
    ///
    /// Returns an error for an empty identifier or control characters.
    pub fn new(
        value: impl Into<String>,
    ) -> ToolchainResult<Self> {
        let value = value.into();

        validate_nonempty_identifier(
            &value,
            "build input authority",
        )?;

        Ok(Self(value))
    }

    /// Return the exact authority identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for BuildInputAuthority {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// NAMED BUILD INPUT
// ============================================================================

/// One authoritative, named build-input fingerprint association.
///
/// The category and identifier are open-ended strings validated by the
/// compiler's canonical input-identity types.
///
/// The fingerprint is supplied by the subsystem that owns the input.
/// This module does not reinterpret or recompute it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BuildInputRecord {
    scope: BuildInputScope,
    category: InputCategory,
    identifier: InputIdentifier,
    authority: BuildInputAuthority,
    fingerprint: Fingerprint,
}

impl BuildInputRecord {
    /// Construct one named build-input record.
    ///
    /// # Errors
    ///
    /// Returns an error if the authority or target-realization identity
    /// is invalid.
    pub fn new(
        scope: BuildInputScope,
        category: InputCategory,
        identifier: InputIdentifier,
        authority: BuildInputAuthority,
        fingerprint: Fingerprint,
    ) -> ToolchainResult<Self> {
        if let BuildInputScope::TargetRealization(identity) = &scope {
            validate_nonempty_identifier(
                identity,
                "target realization identity",
            )?;
        }

        Ok(Self {
            scope,
            category,
            identifier,
            authority,
            fingerprint,
        })
    }

    /// Return the scope of this input.
    #[must_use]
    pub fn scope(&self) -> &BuildInputScope {
        &self.scope
    }

    /// Return the open-ended input category.
    #[must_use]
    pub fn category(&self) -> &InputCategory {
        &self.category
    }

    /// Return the logical input identifier.
    #[must_use]
    pub fn identifier(&self) -> &InputIdentifier {
        &self.identifier
    }

    /// Return the declared fingerprint authority.
    #[must_use]
    pub fn authority(&self) -> &BuildInputAuthority {
        &self.authority
    }

    /// Return the authoritative input fingerprint.
    #[must_use]
    pub const fn fingerprint(&self) -> Fingerprint {
        self.fingerprint
    }

    fn compare_key(&self, other: &Self) -> Ordering {
        self.scope
            .cmp(&other.scope)
            .then_with(|| self.category.cmp(&other.category))
            .then_with(|| self.identifier.cmp(&other.identifier))
    }
}

// ============================================================================
// REALIZATION FINGERPRINT
// ============================================================================

/// Fingerprint of one target-specific realization's complete input set.
///
/// The logical build fingerprint is included in the realization encoding.
/// Consequently, a realization cannot be detached from the logical build
/// whose inputs it extends.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RealizationFingerprint {
    identity: String,
    fingerprint: Fingerprint,
    input_count: usize,
}

impl RealizationFingerprint {
    /// Return the stable realization identity.
    #[must_use]
    pub fn identity(&self) -> &str {
        &self.identity
    }

    /// Return the realization fingerprint.
    #[must_use]
    pub const fn fingerprint(&self) -> Fingerprint {
        self.fingerprint
    }

    /// Return the number of target-specific input records.
    #[must_use]
    pub const fn input_count(&self) -> usize {
        self.input_count
    }
}

// ============================================================================
// IMMUTABLE BUILD INPUT SNAPSHOT
// ============================================================================

/// Immutable snapshot of the logical build and its target realizations.
///
/// The workspace metadata is retained as the authoritative source and
/// manifest inventory snapshot. The named records preserve dependency,
/// configuration, provider, target and other subsystem-owned fingerprints.
///
/// This type does not retain source buffers. The executor must obtain
/// source text from the canonical source-provider boundary.
#[derive(Clone, Debug, PartialEq, Eq)]
#[must_use = "build inputs must be consumed by planning, execution or reporting"]
pub struct BuildInputs {
    workspace: WorkspaceMetadata,
    records: Vec<BuildInputRecord>,
    realizations: Vec<RealizationFingerprint>,
    logical_fingerprint: Fingerprint,
    fingerprint: Fingerprint,
}

impl BuildInputs {
    /// Borrow the immutable workspace metadata snapshot.
    #[must_use]
    pub fn workspace(&self) -> &WorkspaceMetadata {
        &self.workspace
    }

    /// Borrow all named inputs in canonical order.
    ///
    /// Ordering is:
    ///
    ///     scope -> category -> identifier
    #[must_use]
    pub fn records(&self) -> &[BuildInputRecord] {
        &self.records
    }

    /// Borrow the target-realization fingerprints in canonical order.
    #[must_use]
    pub fn realizations(&self) -> &[RealizationFingerprint] {
        &self.realizations
    }

    /// Return the target-independent logical build fingerprint.
    ///
    /// This fingerprint includes the workspace content and layout
    /// identities and every logical-scope input.
    #[must_use]
    pub const fn logical_fingerprint(&self) -> Fingerprint {
        self.logical_fingerprint
    }

    /// Return the complete snapshot fingerprint.
    ///
    /// This includes the logical build identity and all target-specific
    /// realization fingerprints represented by this snapshot.
    #[must_use]
    pub const fn fingerprint(&self) -> Fingerprint {
        self.fingerprint
    }

    /// Return the number of named input records.
    #[must_use]
    pub fn input_count(&self) -> usize {
        self.records.len()
    }

    /// Return the number of target-specific realizations represented.
    #[must_use]
    pub fn realization_count(&self) -> usize {
        self.realizations.len()
    }

    /// Return the fingerprint for a particular target realization.
    ///
    /// Returns `None` if the snapshot contains no target-specific inputs
    /// for the supplied realization identity.
    #[must_use]
    pub fn realization_fingerprint(
        &self,
        identity: &str,
    ) -> Option<Fingerprint> {
        self.realizations
            .binary_search_by(|entry| {
                entry.identity.as_str().cmp(identity)
            })
            .ok()
            .map(|index| self.realizations[index].fingerprint)
    }

    /// Return whether two snapshots have the same complete input identity.
    ///
    /// This is a fingerprint comparison, not proof of semantic equivalence,
    /// authenticity or artifact validity.
    #[must_use]
    pub fn has_same_identity(&self, other: &Self) -> bool {
        self.fingerprint == other.fingerprint
    }
}

// ============================================================================
// BUILD INPUT BUILDER
// ============================================================================

/// Builder for an immutable build-input snapshot.
///
/// The builder consumes an already collected workspace metadata snapshot.
/// It does not inspect the filesystem or parse the manifest.
///
/// The source inventory must be complete. An incomplete source inventory
/// cannot be used to claim a complete logical build identity.
///
/// Input fingerprints must be obtained from the subsystem that owns each
/// input. This builder only composes their identities.
#[derive(Debug)]
pub struct BuildInputsBuilder {
    workspace: WorkspaceMetadata,
    records: Vec<BuildInputRecord>,
}

impl BuildInputsBuilder {
    /// Construct a builder from an inspected workspace snapshot.
    ///
    /// # Errors
    ///
    /// Returns an error if the workspace source inventory is incomplete.
    /// The caller must collect complete workspace metadata before claiming
    /// a complete build-input snapshot.
    pub fn new(
        workspace: WorkspaceMetadata,
    ) -> ToolchainResult<Self> {
        if !workspace.source_inventory_complete() {
            return Err(input_error(
                ToolchainErrorKind::ValidationFailed,
                "workspace source inventory is incomplete; complete input identity cannot be established",
            ));
        }

        Ok(Self {
            workspace,
            records: Vec::new(),
        })
    }

    /// Register an authoritative fingerprinted build input.
    ///
    /// The category and identifier are open-ended. Examples include:
    ///
    /// - `manifest`
    /// - `dependency`
    /// - `lockfile`
    /// - `configuration`
    /// - `profile`
    /// - `feature-resolution`
    /// - `target-intent`
    /// - `provider`
    /// - `environment`
    /// - `compiler-inputs`
    /// - `generated-input`
    /// - `domain-ir`
    ///
    /// The category is descriptive identity, not a closed list of supported
    /// computing domains.
    ///
    /// A duplicate `(scope, category, identifier)` is rejected even if the
    /// supplied authority or fingerprint differs. Replacing an input
    /// silently would make provenance and determinism ambiguous.
    ///
    /// # Errors
    ///
    /// Returns an error for a duplicate identity, invalid record, or
    /// allocation failure.
    pub fn add_fingerprinted_input(
        &mut self,
        record: BuildInputRecord,
    ) -> ToolchainResult<&mut Self> {
        insert_record(&mut self.records, record)?;

        Ok(self)
    }

    /// Register one target-independent logical input.
    ///
    /// The supplied fingerprint must come from its canonical owning
    /// subsystem.
    pub fn add_logical_input(
        &mut self,
        category: InputCategory,
        identifier: InputIdentifier,
        authority: BuildInputAuthority,
        fingerprint: Fingerprint,
    ) -> ToolchainResult<&mut Self> {
        self.add_fingerprinted_input(BuildInputRecord::new(
            BuildInputScope::Logical,
            category,
            identifier,
            authority,
            fingerprint,
        )?)
    }

    /// Register one input belonging to a specific target realization.
    ///
    /// The realization identity must be stable and supplied by the
    /// authoritative target/build planning layer.
    pub fn add_realization_input(
        &mut self,
        realization_identity: impl Into<String>,
        category: InputCategory,
        identifier: InputIdentifier,
        authority: BuildInputAuthority,
        fingerprint: Fingerprint,
    ) -> ToolchainResult<&mut Self> {
        let scope = BuildInputScope::target_realization(
            realization_identity,
        )?;

        self.add_fingerprinted_input(BuildInputRecord::new(
            scope,
            category,
            identifier,
            authority,
            fingerprint,
        )?)
    }

    /// Register the compiler's canonical compilation-input identity for
    /// one target realization.
    ///
    /// `CompilationInputs` owns the canonical source-unit and named
    /// compilation-input fingerprinting. This method records only its
    /// final fingerprint; it does not duplicate or reinterpret its data.
    ///
    /// The compiler input set includes its explicit compilation
    /// environment. Therefore, callers must associate target-specific
    /// compilation environments with the corresponding realization.
    ///
    /// # Errors
    ///
    /// Returns an error if the realization identity is invalid or a
    /// compilation-input record for that realization already exists.
    pub fn add_compilation_inputs(
        &mut self,
        realization_identity: impl Into<String>,
        inputs: &CompilationInputs,
    ) -> ToolchainResult<&mut Self> {
        let category = InputCategory::new("compiler-inputs")
            .map_err(|error| {
                input_error(
                    ToolchainErrorKind::Internal,
                    format!(
                        "invalid internal compiler-input category: {error}"
                    ),
                )
                .with_source(error)
            })?;

        let identifier = InputIdentifier::new("compilation-inputs")
            .map_err(|error| {
                input_error(
                    ToolchainErrorKind::Internal,
                    format!(
                        "invalid internal compiler-input identifier: {error}"
                    ),
                )
                .with_source(error)
            })?;

        let authority = BuildInputAuthority::new(
            "zamani.compiler.reproducibility.inputs.v1",
        )?;

        self.add_realization_input(
            realization_identity,
            category,
            identifier,
            authority,
            inputs.fingerprint(),
        )
    }

    /// Construct the immutable, canonically fingerprinted snapshot.
    ///
    /// All input records are already in deterministic order.
    ///
    /// The resulting snapshot contains:
    ///
    /// - A target-independent logical fingerprint.
    /// - One fingerprint for each represented target realization.
    /// - A complete snapshot fingerprint.
    ///
    /// # Errors
    ///
    /// Returns an error if canonical fingerprint construction fails or
    /// a representational/allocation limit is reached.
    pub fn build(self) -> ToolchainResult<BuildInputs> {
        let logical_fingerprint = fingerprint_logical_inputs(
            &self.workspace,
            &self.records,
        )?;

        let realizations = fingerprint_realizations(
            logical_fingerprint,
            &self.records,
        )?;

        let fingerprint = fingerprint_complete_snapshot(
            logical_fingerprint,
            &realizations,
        )?;

        Ok(BuildInputs {
            workspace: self.workspace,
            records: self.records,
            realizations,
            logical_fingerprint,
            fingerprint,
        })
    }
}

// ============================================================================
// CANONICAL RECORD INSERTION
// ============================================================================

fn insert_record(
    records: &mut Vec<BuildInputRecord>,
    record: BuildInputRecord,
) -> ToolchainResult<()> {
    match records.binary_search_by(|existing| {
        existing.compare_key(&record)
    }) {
        Ok(_) => {
            return Err(input_error(
                ToolchainErrorKind::AlreadyExists,
                format!(
                    "duplicate build input identity: scope={:?}, category={}, identifier={}",
                    record.scope,
                    record.category.as_str(),
                    record.identifier.as_str(),
                ),
            ));
        }

        Err(index) => {
            records.try_reserve(1).map_err(|error| {
                allocation_error("build input records", error)
            })?;

            records.insert(index, record);
        }
    }

    Ok(())
}

// ============================================================================
// CANONICAL FINGERPRINT ENCODING
// ============================================================================

fn append_record(
    builder: &mut StableHashBuilder,
    record: &BuildInputRecord,
) -> ToolchainResult<()> {
    match &record.scope {
        BuildInputScope::Logical => {
            builder
                .field_str("scope", "logical")
                .map_err(fingerprint_error)?;
        }

        BuildInputScope::TargetRealization(identity) => {
            builder
                .field_str("scope", "target-realization")
                .map_err(fingerprint_error)?;

            builder
                .field_str("realization", identity)
                .map_err(fingerprint_error)?;
        }
    }

    builder
        .field_str("category", record.category.as_str())
        .map_err(fingerprint_error)?;

    builder
        .field_str("identifier", record.identifier.as_str())
        .map_err(fingerprint_error)?;

    builder
        .field_str("authority", record.authority.as_str())
        .map_err(fingerprint_error)?;

    builder
        .field_fingerprint("input-fingerprint", record.fingerprint)
        .map_err(fingerprint_error)?;

    Ok(())
}

fn fingerprint_logical_inputs(
    workspace: &WorkspaceMetadata,
    records: &[BuildInputRecord],
) -> ToolchainResult<Fingerprint> {
    let mut builder = StableHashBuilder::new(
        LOGICAL_BUILD_INPUTS_DOMAIN,
    )
    .map_err(fingerprint_error)?;

    builder
        .field_u32("schema-version", BUILD_INPUTS_SCHEMA_VERSION)
        .map_err(fingerprint_error)?;

    // Workspace content identity excludes workspace-relative paths.
    builder
        .field_str(
            "workspace-content-fingerprint",
            workspace.content_fingerprint(),
        )
        .map_err(fingerprint_error)?;

    // Workspace identity includes the portable project/file layout.
    // This matters because relative paths can participate in module
    // resolution and source inclusion.
    builder
        .field_str(
            "workspace-layout-fingerprint",
            workspace.workspace_fingerprint(),
        )
        .map_err(fingerprint_error)?;

    builder
        .field_u64(
            "workspace-project-count",
            canonical_count(
                workspace.project_count(),
                "workspace project count",
            )?,
        )
        .map_err(fingerprint_error)?;

    builder
        .field_bool(
            "source-inventory-complete",
            workspace.source_inventory_complete(),
        )
        .map_err(fingerprint_error)?;

    let logical_count = records
        .iter()
        .filter(|record| record.scope.is_logical())
        .count();

    builder
        .field_u64(
            "logical-input-count",
            canonical_count(logical_count, "logical input count")?,
        )
        .map_err(fingerprint_error)?;

    for record in records.iter().filter(|record| record.scope.is_logical()) {
        append_record(&mut builder, record)?;
    }

    Ok(builder.finish())
}

fn fingerprint_realizations(
    logical_fingerprint: Fingerprint,
    records: &[BuildInputRecord],
) -> ToolchainResult<Vec<RealizationFingerprint>> {
    let realization_count = records
        .iter()
        .filter(|record| {
            matches!(
                record.scope,
                BuildInputScope::TargetRealization(_)
            )
        })
        .map(|record| match &record.scope {
            BuildInputScope::TargetRealization(identity) => identity.as_str(),
            BuildInputScope::Logical => "",
        })
        .fold(
            (None::<&str>, 0_usize),
            |(previous, count), current| {
                if previous == Some(current) {
                    (previous, count)
                } else {
                    (Some(current), count.saturating_add(1))
                }
            },
        )
        .1;

    let mut output = Vec::new();

    output.try_reserve(realization_count).map_err(|error| {
        allocation_error("realization fingerprints", error)
    })?;

    let mut index = 0_usize;

    while index < records.len() {
        let identity = match &records[index].scope {
            BuildInputScope::Logical => {
                index = index.saturating_add(1);
                continue;
            }

            BuildInputScope::TargetRealization(identity) => identity.clone(),
        };

        let start = index;

        while index < records.len() {
            match &records[index].scope {
                BuildInputScope::TargetRealization(current)
                    if current == &identity =>
                {
                    index = index.saturating_add(1);
                }

                _ => break,
            }
        }

        let group = &records[start..index];

        let mut builder = StableHashBuilder::new(
            REALIZATION_BUILD_INPUTS_DOMAIN,
        )
        .map_err(fingerprint_error)?;

        builder
            .field_u32("schema-version", BUILD_INPUTS_SCHEMA_VERSION)
            .map_err(fingerprint_error)?;

        builder
            .field_fingerprint("logical-build-fingerprint", logical_fingerprint)
            .map_err(fingerprint_error)?;

        builder
            .field_str("realization-identity", &identity)
            .map_err(fingerprint_error)?;

        builder
            .field_u64(
                "realization-input-count",
                canonical_count(
                    group.len(),
                    "realization input count",
                )?,
            )
            .map_err(fingerprint_error)?;

        for record in group {
            append_record(&mut builder, record)?;
        }

        output.push(RealizationFingerprint {
            identity,
            fingerprint: builder.finish(),
            input_count: group.len(),
        });
    }

    Ok(output)
}

fn fingerprint_complete_snapshot(
    logical_fingerprint: Fingerprint,
    realizations: &[RealizationFingerprint],
) -> ToolchainResult<Fingerprint> {
    let mut builder = StableHashBuilder::new(
        COMPLETE_BUILD_INPUTS_DOMAIN,
    )
    .map_err(fingerprint_error)?;

    builder
        .field_u32("schema-version", BUILD_INPUTS_SCHEMA_VERSION)
        .map_err(fingerprint_error)?;

    builder
        .field_fingerprint("logical-build-fingerprint", logical_fingerprint)
        .map_err(fingerprint_error)?;

    builder
        .field_u64(
            "realization-count",
            canonical_count(
                realizations.len(),
                "realization count",
            )?,
        )
        .map_err(fingerprint_error)?;

    for realization in realizations {
        builder
            .field_str("realization-identity", realization.identity())
            .map_err(fingerprint_error)?;

        builder
            .field_fingerprint(
                "realization-fingerprint",
                realization.fingerprint(),
            )
            .map_err(fingerprint_error)?;
    }

    Ok(builder.finish())
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn category(value: &str) -> InputCategory {
        InputCategory::new(value).expect("valid input category")
    }

    fn identifier(value: &str) -> InputIdentifier {
        InputIdentifier::new(value).expect("valid input identifier")
    }

    fn authority(value: &str) -> BuildInputAuthority {
        BuildInputAuthority::new(value).expect("valid authority")
    }

    fn record(
        scope: BuildInputScope,
        category_value: &str,
        identifier_value: &str,
        fingerprint_byte: u8,
    ) -> BuildInputRecord {
        BuildInputRecord::new(
            scope,
            category(category_value),
            identifier(identifier_value),
            authority("test.authority.v1"),
            Fingerprint::from_bytes([fingerprint_byte; 32]),
        )
        .expect("valid build input record")
    }

    #[test]
    fn logical_scope_is_explicit() {
        assert!(BuildInputScope::Logical.is_logical());
        assert_eq!(
            BuildInputScope::Logical.realization_identity(),
            None,
        );
    }

    #[test]
    fn target_realization_scope_rejects_empty_identity() {
        assert!(
            BuildInputScope::target_realization("  ").is_err()
        );
    }

    #[test]
    fn target_realization_scope_rejects_control_characters() {
        assert!(
            BuildInputScope::target_realization("target\none").is_err()
        );
    }

    #[test]
    fn target_realization_scope_preserves_exact_identity() {
        let scope = BuildInputScope::target_realization(
            "provider:target:realization-01",
        )
        .expect("valid realization identity");

        assert_eq!(
            scope.realization_identity(),
            Some("provider:target:realization-01"),
        );
    }

    #[test]
    fn authority_rejects_empty_identity() {
        assert!(BuildInputAuthority::new(" ").is_err());
    }

    #[test]
    fn authority_rejects_control_characters() {
        assert!(BuildInputAuthority::new("provider\tv1").is_err());
    }

    #[test]
    fn records_are_inserted_in_canonical_order() {
        let mut records = Vec::new();

        insert_record(
            &mut records,
            record(
                BuildInputScope::Logical,
                "dependency",
                "zeta",
                1,
            ),
        )
        .expect("first input");

        insert_record(
            &mut records,
            record(
                BuildInputScope::Logical,
                "configuration",
                "build",
                2,
            ),
        )
        .expect("second input");

        insert_record(
            &mut records,
            record(
                BuildInputScope::Logical,
                "dependency",
                "alpha",
                3,
            ),
        )
        .expect("third input");

        assert_eq!(
            records
                .iter()
                .map(|entry| (
                    entry.category().as_str(),
                    entry.identifier().as_str(),
                ))
                .collect::<Vec<_>>(),
            vec![
                ("configuration", "build"),
                ("dependency", "alpha"),
                ("dependency", "zeta"),
            ],
        );
    }

    #[test]
    fn duplicate_scope_category_and_identifier_are_rejected() {
        let mut records = Vec::new();

        let first = record(
            BuildInputScope::Logical,
            "dependency",
            "math",
            1,
        );

        let duplicate = record(
            BuildInputScope::Logical,
            "dependency",
            "math",
            2,
        );

        insert_record(&mut records, first)
            .expect("first input");

        let error = insert_record(&mut records, duplicate)
            .expect_err("duplicate identity must fail");

        assert_eq!(
            error.kind(),
            ToolchainErrorKind::AlreadyExists,
        );
    }

    #[test]
    fn same_identifier_can_exist_in_distinct_scopes() {
        let mut records = Vec::new();

        insert_record(
            &mut records,
            record(
                BuildInputScope::Logical,
                "configuration",
                "compiler",
                1,
            ),
        )
        .expect("logical input");

        insert_record(
            &mut records,
            record(
                BuildInputScope::target_realization("target-a")
                    .expect("valid realization"),
                "configuration",
                "compiler",
                2,
            ),
        )
        .expect("target-specific input");

        assert_eq!(records.len(), 2);
    }

    #[test]
    fn record_order_is_scope_then_category_then_identifier() {
        let mut records = Vec::new();

        insert_record(
            &mut records,
            record(
                BuildInputScope::target_realization("target-b")
                    .expect("valid realization"),
                "provider",
                "compiler",
                1,
            ),
        )
        .expect("target b");

        insert_record(
            &mut records,
            record(
                BuildInputScope::Logical,
                "provider",
                "compiler",
                2,
            ),
        )
        .expect("logical");

        insert_record(
            &mut records,
            record(
                BuildInputScope::target_realization("target-a")
                    .expect("valid realization"),
                "provider",
                "compiler",
                3,
            ),
        )
        .expect("target a");

        assert!(records[0].scope().is_logical());

        assert_eq!(
            records[1].scope().realization_identity(),
            Some("target-a"),
        );

        assert_eq!(
            records[2].scope().realization_identity(),
            Some("target-b"),
        );
    }

    #[test]
    fn fingerprint_schema_versions_are_explicit() {
        assert_eq!(BUILD_INPUTS_API_VERSION, 1);
        assert_eq!(BUILD_INPUTS_SCHEMA_VERSION, 1);

        assert_ne!(
            LOGICAL_BUILD_INPUTS_DOMAIN,
            REALIZATION_BUILD_INPUTS_DOMAIN,
        );

        assert_ne!(
            REALIZATION_BUILD_INPUTS_DOMAIN,
            COMPLETE_BUILD_INPUTS_DOMAIN,
        );
    }
}

//! Zamani Quantum Benchmarking — Registry Module
//!
//! This module is the public boundary for the benchmark registry subsystem.
//!
//! # Responsibilities
//!
//! `registry/mod.rs` owns:
//!
//! - registry module wiring;
//! - public registry API re-exports;
//! - built-in catalog access;
//! - executable built-in registry construction;
//! - compatibility-module exposure;
//! - stable registry-level constants;
//! - registry smoke/validation entry points;
//! - documentation of the dependency boundary.
//!
//! It does NOT:
//!
//! - implement benchmark protocols;
//! - execute quantum circuits;
//! - generate circuits;
//! - perform statistical analysis;
//! - own hardware state;
//! - own Quantum IR;
//! - perform backend communication;
//! - implement benchmark mathematics;
//! - dynamically load arbitrary code;
//! - maintain process-global mutable state.
//!
//! Those responsibilities remain in their owning modules.
//!
//! # Architecture
//!
//! ```text
//!                         quantum::benchmarking
//!                                  │
//!                                  ▼
//!                    ┌──────────────────────────┐
//!                    │      registry/mod.rs     │
//!                    │   public module boundary │
//!                    └────────────┬─────────────┘
//!                                 │
//!              ┌──────────────────┼──────────────────┐
//!              │                  │                  │
//!              ▼                  ▼                  ▼
//!        registry.rs        builtin.rs       compatibility.rs
//!              │                  │                  │
//!              │                  │                  ▼
//!              │                  │        quantum::hardware
//!              │                  │
//!              └──────────┬───────┘
//!                         ▼
//!                BenchmarkRegistry
//!                         │
//!                         ▼
//!                 BenchmarkDescriptor
//!                         │
//!                         ▼
//!                  BenchmarkFactory
//!                         │
//!                         ▼
//!                    Benchmark
//! ```
//!
//! # Important architectural distinction
//!
//! The registry has two related but deliberately different concepts:
//!
//! 1. **Built-in catalog**
//!
//!    The catalog describes benchmark families Zamani intends to support.
//!
//! 2. **Executable registry**
//!
//!    The executable registry contains only benchmarks for which a concrete
//!    implementation satisfies the universal `core::benchmark::Benchmark`
//!    contract.
//!
//! A benchmark appearing in the catalog does NOT automatically mean that it
//! can be executed.
//!
//! This distinction is already enforced by `builtin.rs` and must remain
//! visible at this module boundary.
//!
//! # Dependency direction
//!
//! The required dependency direction is:
//!
//! ```text
//! protocols/*
//! applications/*
//! qec/*
//!      │
//!      ▼
//! registry/builtin.rs
//!      │
//!      ▼
//! registry/registry.rs
//!      │
//!      ▼
//! registry/mod.rs
//!      │
//!      ▼
//! callers / stdlib / frontend / tooling
//! ```
//!
//! `registry.rs` deliberately does not import concrete protocols.
//!
//! `builtin.rs` is the integration boundary where concrete benchmark
//! implementations eventually become registered.
//!
//! `compatibility.rs` is the backend compatibility boundary.
//!
//! # No global registry
//!
//! This module deliberately does not expose a process-global singleton such as:
//!
//! ```text
//! static REGISTRY: Mutex<...>
//! ```
//!
//! A registry is explicitly constructed and owned by its caller.
//!
//! This provides:
//!
//! - deterministic initialization;
//! - test isolation;
//! - no hidden mutable state;
//! - easier reproducibility;
//! - easier embedded/runtime use;
//! - easier multi-runtime operation;
//! - easier future sandboxing.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//!
//! No nightly features are required.
//!
//! # Integration contract
//!
//! This module integrates with:
//!
//! - `core::benchmark` for the universal benchmark contract;
//! - `registry::registry` for registry storage and lookup;
//! - `registry::builtin` for Zamani's built-in benchmark catalog;
//! - `registry::compatibility` for backend capability negotiation;
//! - `quantum::hardware` through `compatibility.rs`;
//! - future `stdlib::quantum` APIs through this stable boundary.
//!
//! The public API intentionally re-exports the stable registry types so callers
//! do not need to know the internal file layout.
//!
//! For example:
//!
//! ```text
//! quantum::benchmarking::registry::BenchmarkRegistry
//! quantum::benchmarking::registry::BenchmarkDescriptor
//! quantum::benchmarking::registry::BenchmarkCapability
//! quantum::benchmarking::registry::builtin_registry()
//! ```
//!
//! rather than requiring callers to depend directly on:
//!
//! ```text
//! quantum::benchmarking::registry::registry::...
//! ```
//!
//! # Stability rule
//!
//! Protocol implementations must not import private implementation details from
//! this module.
//!
//! The stable boundary is the public API re-exported below.
//!
//! Adding a new benchmark should normally require changes to:
//!
//! - its protocol/application/QEC implementation;
//! - `registry/builtin.rs` when it becomes executable;
//! - associated tests;
//!
//! but should NOT require changing this file unless a new registry subsystem
//! itself is introduced.
//!
//! That property is intentional: once this file is completed, future benchmark
//! additions should not require repeatedly redesigning the registry boundary.
//!
//! # Compatibility with the existing repository
//!
//! The repository's quantum root historically exposed benchmarking through an
//! inline module containing `volume_estimator`. The production architecture is
//! directory-based:
//!
//! ```text
//! quantum::benchmarking
//!     ├── core
//!     ├── generators
//!     ├── execution
//!     ├── statistics
//!     ├── metrics
//!     ├── protocols
//!     ├── volumetric
//!     ├── applications
//!     ├── qec
//!     ├── hardware
//!     ├── analysis
//!     ├── reporting
//!     ├── validation
//!     └── registry
//! ```
//!
//! The quantum root must therefore eventually delegate `benchmarking` to this
//! directory module. The historical `quantum::volume_estimator` re-export can
//! remain as a compatibility path.
//!
//! # Security properties
//!
//! This module:
//!
//! - performs no network access;
//! - performs no hardware access;
//! - does not dynamically load code;
//! - does not execute benchmark constructors merely to enumerate metadata;
//! - does not create global mutable state;
//! - exposes bounded registry construction;
//! - preserves explicit capability negotiation;
//! - keeps user-defined benchmark registration behind an explicit API boundary.
//!
//! # Reproducibility properties
//!
//! Built-in registry construction is deterministic.
//!
//! The underlying registry uses deterministic ordering and the built-in catalog
//! is statically declared. This is important for:
//!
//! - benchmark manifests;
//! - generated documentation;
//! - CLI output;
//! - reproducibility fingerprints;
//! - CI;
//! - regression tests.
//!
//! # What this file intentionally does NOT do
//!
//! Do not add benchmark IDs here.
//!
//! Do not add aliases here.
//!
//! Do not add protocol factories here.
//!
//! Do not add backend-specific capability rules here.
//!
//! Do not add benchmark-specific mathematics here.
//!
//! Those belong to:
//!
//! ```text
//! builtin.rs
//! registry.rs
//! compatibility.rs
//! protocols/*
//! applications/*
//! qec/*
//! ```
//!
//! respectively.

// =============================================================================
// Submodules
// =============================================================================

/// Generic benchmark registry implementation.
///
/// Owns:
///
/// - descriptors;
/// - factories;
/// - aliases;
/// - canonical ID lookup;
/// - deterministic enumeration;
/// - registration validation.
///
/// It does not know concrete benchmark protocol implementations.
pub mod registry;

/// Built-in Zamani benchmark catalog and executable registrations.
///
/// This is the only registry layer that is allowed to connect concrete
/// benchmark implementations to the generic registry.
pub mod builtin;

/// Backend compatibility negotiation.
///
/// This module answers whether a benchmark can be executed correctly by a
/// particular backend under a requested experiment configuration.
pub mod compatibility;

// =============================================================================
// Stable public re-exports
// =============================================================================
//
// These re-exports are deliberately explicit rather than:
//
//     pub use registry::*;
//
// A wildcard export makes future additions to registry.rs silently become part
// of the public API. Explicit exports provide a controlled compatibility
// boundary for the Zamani language, runtime and tooling.
// =============================================================================

// -----------------------------------------------------------------------------
// Generic registry
// -----------------------------------------------------------------------------

pub use self::registry::{
    descriptor,
    BenchmarkCapability,
    BenchmarkDescriptor,
    BenchmarkExecutionTarget,
    BenchmarkFactory,
    BenchmarkRegistry,
    RegistryError,
    DEFAULT_MAX_BENCHMARKS,
    MAX_ALIASES_PER_BENCHMARK,
    MAX_DESCRIPTION_LENGTH,
    MAX_IDENTIFIER_LENGTH,
    REGISTRY_COMPONENT_ID,
    REGISTRY_SCHEMA_VERSION,
};

// -----------------------------------------------------------------------------
// Built-in catalog
// -----------------------------------------------------------------------------

pub use self::builtin::{
    builtin_catalog,
    catalog_entry,
    is_builtin,
    is_builtin_executable,
    BuiltinBenchmarkFamily,
    BuiltinCatalogEntry,
    BUILTIN_CATALOG,
    BUILTIN_CATALOG_ID,
    BUILTIN_CATALOG_VERSION,
    BUILTIN_MAX_BENCHMARKS,
};

// -----------------------------------------------------------------------------
// Compatibility
// -----------------------------------------------------------------------------
//
// The compatibility module has a larger API surface because it contains the
// backend capability model. Re-export only its stable top-level types here.
// Detailed backend-specific APIs remain available under:
//
//     registry::compatibility::*
//
// This prevents the registry root from becoming an uncontrolled compatibility
// namespace while still making the primary negotiation API convenient.

pub use self::compatibility::{
    check_compatibility,
    BenchmarkFamily,
    BenchmarkRequirements,
    CapabilityRequirement,
    CompatibilityReport,
    CompatibilityStatus,
    ExecutionModel,
    QuantumTechnology,
    ResourceRequirements,
};

// =============================================================================
// Registry-level constants
// =============================================================================

/// Stable public identifier for the registry subsystem.
pub const REGISTRY_ID: &str = "zamani.quantum.benchmark.registry";

/// Stable API/schema generation exposed by the registry module.
///
/// This is deliberately separate from:
///
/// - the individual benchmark protocol version;
/// - the registry implementation version;
/// - the built-in catalog version;
/// - the overall Zamani compiler version.
pub const REGISTRY_API_VERSION: u32 = 1;

/// Maximum number of built-in descriptors accepted by the built-in registry.
///
/// This mirrors the explicit safety boundary established by `builtin.rs`.
pub const MAX_BUILTIN_BENCHMARKS: usize = BUILTIN_MAX_BENCHMARKS;

// =============================================================================
// Built-in registry construction
// =============================================================================

/// Construct a fresh registry containing all currently executable built-in
/// benchmarks.
///
/// # Important
///
/// This function does not mean that every entry in [`BUILTIN_CATALOG`] becomes
/// executable.
///
/// `builtin.rs` deliberately distinguishes:
///
/// ```text
/// catalog entry
///      │
///      ├── executable adapter exists ──► registry
///      │
///      └── adapter incomplete ─────────► catalog only
/// ```
///
/// Therefore this function is the authoritative way for runtime/stdlib/CLI
/// code to obtain the executable built-in registry.
///
/// # Determinism
///
/// Every invocation constructs an independent registry with deterministic
/// contents.
///
/// # Ownership
///
/// The caller owns the returned registry.
///
/// No process-global registry is created.
///
/// # Errors
///
/// Returns [`RegistryError`] if a built-in descriptor violates the generic
/// registry invariants.
pub fn builtin_registry() -> Result<BenchmarkRegistry, RegistryError> {
    builtin::builtin_registry()
}

/// Construct a fresh registry with an explicit capacity.
///
/// This is useful for embedded/runtime callers that want a tighter resource
/// bound than the generic default.
///
/// The built-in registry must still fit inside the requested capacity.
///
/// # Security
///
/// This function never allocates more than the caller explicitly permits
/// through the registry implementation.
pub fn builtin_registry_with_capacity(
    capacity: usize,
) -> Result<BenchmarkRegistry, RegistryError> {
    builtin::builtin_registry_with_capacity(capacity)
}

// =============================================================================
// Catalog inspection
// =============================================================================

/// Return the number of benchmark families represented by the built-in
/// catalog.
///
/// This is metadata only and does not construct benchmark implementations.
#[must_use]
pub fn builtin_catalog_len() -> usize {
    builtin_catalog().len()
}

/// Return the number of built-in benchmarks that currently have an executable
/// universal `Benchmark` adapter.
#[must_use]
pub fn executable_builtin_count() -> usize {
    builtin_catalog()
        .iter()
        .filter(|entry| entry.executable)
        .count()
}

/// Return the number of catalog entries that are intentionally catalog-only.
///
/// This is useful for diagnostics and CI checks because the presence of
/// catalog-only entries is not itself an error.
#[must_use]
pub fn catalog_only_builtin_count() -> usize {
    builtin_catalog()
        .iter()
        .filter(|entry| !entry.executable)
        .count()
}

// =============================================================================
// Registry validation
// =============================================================================

/// Validate the complete built-in catalog.
///
/// This checks catalog-level invariants without constructing executable
/// benchmarks.
///
/// It is intentionally separate from [`validate_builtin_registry`] because
/// catalog validity and executable-registry validity are different concerns.
///
/// # Errors
///
/// Returns [`RegistryError`] if the catalog contains:
///
/// - duplicate canonical IDs;
/// - invalid IDs;
/// - invalid metadata;
/// - excessive catalog size;
/// - conflicting identifiers.
pub fn validate_builtin_catalog() -> Result<(), RegistryError> {
    if builtin_catalog().len() > MAX_BUILTIN_BENCHMARKS {
        return Err(RegistryError::CapacityExceeded {
            maximum: MAX_BUILTIN_BENCHMARKS,
        });
    }

    // The generic registry owns canonical identifier validation. We construct
    // a temporary bounded registry only for catalog entries that are actually
    // executable; catalog-only entries must not require executable factories.
    //
    // Duplicate and identifier checks for the catalog itself are performed
    // locally so catalog correctness does not depend on protocol integration.
    let mut ids = std::collections::BTreeSet::new();

    for entry in builtin_catalog() {
        if entry.id.is_empty() {
            return Err(RegistryError::InvalidBenchmarkId {
                value: entry.id.to_owned(),
                reason: "built-in benchmark ID must not be empty".to_owned(),
            });
        }

        if entry.id.len() > MAX_IDENTIFIER_LENGTH {
            return Err(RegistryError::InvalidBenchmarkId {
                value: entry.id.to_owned(),
                reason: format!(
                    "built-in benchmark ID exceeds maximum length {}",
                    MAX_IDENTIFIER_LENGTH
                ),
            });
        }

        if !ids.insert(entry.id) {
            return Err(RegistryError::DuplicateBenchmarkId {
                id: entry.id.to_owned(),
            });
        }

        if entry.name.is_empty() {
            return Err(RegistryError::InvalidMetadata {
                field: "name".to_owned(),
                reason: format!("benchmark `{}` has an empty name", entry.id),
            });
        }

        // Catalog IDs are intentionally stable machine identifiers.
        //
        // Keep validation conservative and aligned with the generic registry:
        // lowercase ASCII plus digits, '_' and '-'.
        if !is_valid_catalog_identifier(entry.id) {
            return Err(RegistryError::InvalidBenchmarkId {
                value: entry.id.to_owned(),
                reason: "built-in benchmark IDs must contain only lowercase \
                         ASCII letters, digits, '_' or '-' and must begin \
                         with a lowercase ASCII letter or digit"
                    .to_owned(),
            });
        }
    }

    Ok(())
}

/// Validate the executable built-in registry.
///
/// This is stronger than [`validate_builtin_catalog`] because it actually
/// constructs the executable registry and asks the generic registry to verify
/// its own invariants.
///
/// This function is suitable for:
///
/// - CI registry smoke tests;
/// - startup validation in debug/development builds;
/// - release validation tooling;
/// - documentation generation;
/// - package integrity checks.
///
/// It does NOT execute benchmark workloads.
pub fn validate_builtin_registry() -> Result<(), RegistryError> {
    validate_builtin_catalog()?;

    let registry = builtin_registry()?;
    registry.validate()
}

// =============================================================================
// Lookup helpers
// =============================================================================

/// Resolve either a canonical benchmark ID or one of its aliases.
///
/// This delegates lookup semantics to the authoritative generic registry.
///
/// The returned descriptor is borrowed from the supplied registry and does not
/// construct a benchmark.
#[must_use]
pub fn resolve<'a>(
    registry: &'a BenchmarkRegistry,
    identifier: &str,
) -> Option<&'a BenchmarkDescriptor> {
    registry.get(identifier)
}

/// Return whether an identifier resolves to an executable benchmark in the
/// supplied registry.
///
/// This is deliberately different from [`is_builtin`]:
///
/// - `is_builtin("qft")` asks whether the identifier belongs to Zamani's
///   built-in catalog;
/// - `contains("qft")` asks whether the supplied executable registry can
///   actually construct it.
#[must_use]
pub fn contains(registry: &BenchmarkRegistry, identifier: &str) -> bool {
    registry.get(identifier).is_some()
}

// =============================================================================
// Identifier validation
// =============================================================================

/// Validate a built-in benchmark identifier using the same lexical contract
/// expected by the generic registry.
///
/// This helper is intentionally private. The generic registry remains the
/// authoritative validator for executable registration.
///
/// The catalog validator uses this local check because catalog entries are
/// metadata and do not necessarily have executable factories.
fn is_valid_catalog_identifier(identifier: &str) -> bool {
    if identifier.is_empty() || identifier.len() > MAX_IDENTIFIER_LENGTH {
        return false;
    }

    let bytes = identifier.as_bytes();

    match bytes.first() {
        Some(first) if first.is_ascii_lowercase() || first.is_ascii_digit() => {}
        _ => return false,
    }

    bytes.iter().all(|byte| {
        byte.is_ascii_lowercase()
            || byte.is_ascii_digit()
            || *byte == b'_'
            || *byte == b'-'
    })
}

// =============================================================================
// Public registry smoke-test helper
// =============================================================================

/// Perform all inexpensive registry integrity checks.
///
/// This function intentionally does not:
///
/// - execute quantum circuits;
/// - communicate with hardware;
/// - invoke benchmark workloads;
/// - require a simulator;
/// - require network access.
///
/// It is therefore appropriate for ordinary unit/CI builds.
///
/// The intended CI usage is conceptually:
///
/// ```text
/// assert!(registry::smoke_test().is_ok());
/// ```
pub fn smoke_test() -> Result<(), RegistryError> {
    validate_builtin_registry()
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_module_has_stable_identity() {
        assert_eq!(
            REGISTRY_ID,
            "zamani.quantum.benchmark.registry"
        );

        assert_eq!(REGISTRY_API_VERSION, 1);
    }

    #[test]
    fn built_in_catalog_is_non_empty() {
        assert!(!builtin_catalog().is_empty());
    }

    #[test]
    fn built_in_catalog_contains_quantum_volume() {
        let entry = catalog_entry("quantum_volume")
            .expect("Quantum Volume must remain part of the built-in catalog");

        assert_eq!(entry.id, "quantum_volume");
        assert_eq!(entry.name, "Quantum Volume");
        assert_eq!(entry.family, BuiltinBenchmarkFamily::Scaling);
    }

    #[test]
    fn built_in_catalog_contains_major_benchmark_families() {
        assert!(is_builtin("quantum_volume"));
        assert!(is_builtin("randomized_benchmarking"));
        assert!(is_builtin("xeb"));
        assert!(is_builtin("cycle_benchmarking"));
        assert!(is_builtin("vqe"));
        assert!(is_builtin("qaoa"));
        assert!(is_builtin("logical_error_rate"));
    }

    #[test]
    fn catalog_contains_only_valid_identifiers() {
        for entry in builtin_catalog() {
            assert!(
                is_valid_catalog_identifier(entry.id),
                "invalid built-in identifier: {}",
                entry.id
            );
        }
    }

    #[test]
    fn catalog_has_unique_identifiers() {
        let mut ids = std::collections::BTreeSet::new();

        for entry in builtin_catalog() {
            assert!(
                ids.insert(entry.id),
                "duplicate built-in benchmark identifier: {}",
                entry.id
            );
        }
    }

    #[test]
    fn catalog_size_is_bounded() {
        assert!(builtin_catalog_len() <= MAX_BUILTIN_BENCHMARKS);
    }

    #[test]
    fn catalog_entries_have_names() {
        for entry in builtin_catalog() {
            assert!(
                !entry.name.is_empty(),
                "benchmark `{}` has no display name",
                entry.id
            );
        }
    }

    #[test]
    fn catalog_validation_succeeds() {
        validate_builtin_catalog()
            .expect("built-in benchmark catalog must satisfy registry invariants");
    }

    #[test]
    fn executable_count_never_exceeds_catalog_count() {
        assert!(
            executable_builtin_count() <= builtin_catalog_len()
        );
    }

    #[test]
    fn catalog_only_count_matches_partition() {
        assert_eq!(
            executable_builtin_count() + catalog_only_builtin_count(),
            builtin_catalog_len()
        );
    }

    #[test]
    fn unknown_identifier_is_not_builtin() {
        assert!(!is_builtin(
            "this_benchmark_does_not_exist_in_zamani"
        ));
    }

    #[test]
    fn unknown_identifier_is_not_executable_builtin() {
        assert!(!is_builtin_executable(
            "this_benchmark_does_not_exist_in_zamani"
        ));
    }

    #[test]
    fn registry_smoke_test_is_available() {
        // This test deliberately validates the complete registry boundary.
        //
        // If the current repository has not yet connected any executable
        // benchmark adapters, `builtin_registry()` is still expected to
        // construct a valid empty-executable registry because catalog-only
        // entries are legal.
        smoke_test().expect("benchmark registry smoke test must pass");
    }
}
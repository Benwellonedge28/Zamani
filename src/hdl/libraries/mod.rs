
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDL Libraries
//!
//! Path: `src/hdl/libraries/mod.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//! Status: Public module composition contract.
//!
//! ## Purpose
//!
//! Compose the HDL library-management API from the existing manifest and
//! registry modules. This module is the public boundary for declarative HDL
//! library metadata and in-memory library registration.
//!
//! ## Authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/spec/versioning.md`
//! - `grammar/spec/compatibility.md`
//! - `grammar/spec/determinism.md`
//! - `grammar/hdl/hdl.g4` and its delegate grammars
//! - `src/frontend/ast/node/paths/import.rs`
//! - `src/frontend/parser/domains/hdl.rs`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/hdl/error.rs`
//! - `src/hdl/ids.rs`
//! - `src/hdl/libraries/registry.rs`
//! - `src/hdl/libraries/manifest.rs`
//!
//! The HDL grammar owns source syntax. The frontend AST owns source-level
//! import structure. Semantic analysis owns source meaning and name
//! resolution. The manifest module owns the manifest format and its adapter.
//! The registry owns exact library identity matching and in-memory storage.
//!
//! ## Owns
//!
//! - Registration of the library-management submodules.
//! - The documented public exports of the library subsystem.
//! - A single import path for the registry and manifest APIs.
//!
//! ## Does not own
//!
//! - A second library registry or descriptor representation.
//! - HDL source parsing, AST construction, or semantic name resolution.
//! - Manifest parsing or dependency validation logic.
//! - Filesystem or network access, package downloads, or persistent storage.
//! - Dependency solving, version-range interpretation, or lockfile handling.
//! - Artifact authentication, cryptographic verification, or trust policy.
//! - Target discovery, physical hardware allocation, or realization.
//! - Any intermediate representation implementation.
//!
//! All HDLIR implementation remains under `src/hdl/hdlir/` and is deferred.
//!
//! ## Public integration contract
//!
//! `registry` is the canonical in-memory library registry.
//! `manifest` parses and validates declarative TOML metadata and converts it
//! into the registry's descriptor contract.
//!
//! Consumers that only need library metadata may use the public types
//! re-exported by this module. Consumers that need module-specific functions
//! or constants may use the explicit `manifest` and `registry` namespaces.
//!
//! This module does not make registry IDs persistent across registry instances
//! or builds. It does not interpret dependency version requirements or imply
//! that dependencies have been fetched, installed, trusted, or resolved.
//!
//! ## Scalability and determinism
//!
//! This composition layer imposes no library-count, dependency-count,
//! hardware-capacity, or design-size limit. Actual storage and processing are
//! bounded by available resources and the underlying collection and identity
//! representations. The registry reports identity exhaustion instead of
//! wrapping or reusing an issued ID.
//!
//! Determinism and duplicate handling are implemented by `registry`; manifest
//! validation and conversion are implemented by `manifest`. This module must
//! not add another normalization, ordering, validation, or identity policy.
//!
//! ## Integration requirements
//!
//! The parent `src/hdl/mod.rs` must declare `pub mod libraries;` after the HDL
//! subsystem is wired into the crate. The crate root must expose `hdl` if the
//! subsystem is intended to be publicly reachable as `crate::hdl`.
//!
//! The existing parser and semantic validator should depend on their
//! established frontend contracts, not on this module. Library resolution
//! can consume this API downstream after import scope and semantic identity
//! have been established.
//!
//! ## Freeze criteria
//!
//! - Both declared submodules exist and compile.
//! - Public re-exports match the actual types and constants they expose.
//! - The registry remains the single in-memory library identity authority.
//! - The manifest remains the single manifest parsing and conversion authority.
//! - No dependency on HDLIR, physical realization, or target selection is added.
//! - Formatting, compilation, tests, and Clippy pass for the integrated crate.

/// Library manifest schema, validation, and registry adapter.
///
/// This module owns TOML parsing and conversion of validated manifest metadata
/// into the canonical registry descriptor. It does not fetch dependencies or
/// write manifest files.
pub mod manifest;

/// Canonical in-memory HDL library descriptor registry.
///
/// This module owns exact identity lookup, registry-local ID allocation,
/// deterministic storage, duplicate rejection, and registration/removal.
/// It does not parse source code or library manifests.
pub mod registry;

/// Public manifest API.
pub use manifest::{
    HdlLibraryManifest,
    HdlLibraryManifestDependency,
    HdlLibraryManifestLibrary,
    HDL_LIBRARY_MANIFEST_MODULE_NAME,
    HDL_LIBRARY_MANIFEST_SCHEMA_VERSION,
};

/// Public registry descriptor and identity API.
pub use registry::{
    HdlLibraryDependency,
    HdlLibraryDescriptor,
    HdlLibraryIdentity,
    HdlLibraryRegistry,
    HDL_LIBRARY_REGISTRY_MODULE_NAME,
    HDL_LIBRARY_REGISTRY_SCHEMA_VERSION,
};

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR Serialization
//!
//! Path: `src/hdl/hdlir/serialization/mod.rs`
//! Repository: `Benwellonedge28/Zamani`
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only.
//! Status: Serialization composition root.
//!
//! ## Purpose
//!
//! Compose the canonical HDLIR artifact-format, encoding, decoding, and
//! migration implementations behind one deliberate module boundary.
//!
//! This module is an API and ownership boundary. It does not implement a
//! second artifact format, payload codec, IR graph, schema registry,
//! migration algorithm, or source-language frontend.
//!
//! ## Normative authority
//!
//! This module must conform to the applicable contracts in:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/Zamani.g4`
//! - `grammar/hdl/hdl.g4` and its applicable delegate grammars
//! - `grammar/specification/semantic-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `src/hdl/api.rs`
//! - `src/hdl/ids.rs`
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/hdl/hdlir/HDLIR-SPEC.md`
//! - `src/hdl/hdlir/compatibility.md`
//! - `src/hdl/hdlir/version.rs`
//! - `src/hdl/hdlir/fingerprint.rs`
//!
//! The grammar owns source syntax. The frontend owns source parsing,
//! source locations, and source-level semantic validation. ZUIR owns the
//! universal computational representation. HDLIR owns the hardware-domain
//! representation. This module owns only the composition of HDLIR's
//! persistent-artifact interfaces.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Registration of the implemented serialization submodules.
//! - The deliberate public re-exports of their supported APIs.
//! - The integration boundary between metadata, artifact framing,
//!   decoding, and version migration.
//!
//! It does not own:
//!
//! - HDL source parsing, ASTs, semantic analysis, or source spans.
//! - The in-memory HDLIR graph or its verification.
//! - Payload-specific encoding or canonicalization.
//! - Operation, type, schema, or dialect registration.
//! - Fingerprint calculation beyond the format's declared metadata.
//! - Synthesis, target selection, allocation, placement, or routing.
//! - Cryptographic authenticity, authorization, or trust decisions.
//!
//! ## Submodule responsibilities
//!
//! - `format`: validated metadata, schema/dialect requirements, format
//!   compatibility declarations, and optional payload-integrity metadata.
//! - `encode`: deterministic framing and writing of metadata plus opaque
//!   payload bytes.
//! - `decode`: safe extraction of one framed artifact, metadata checks,
//!   configured compatibility checks, and payload-integrity verification.
//! - `migration`: explicit, registered version-to-version transformations.
//!
//! These modules remain the sole implementations of their respective
//! responsibilities. This composition root must not duplicate their logic.
//!
//! ## Dependency direction
//!
//! ```text
//! format.rs <------------------ encode.rs
//!     ^                            |
//!     |                            v
//!     +------------------------ decode.rs
//!     ^
//!     |
//! migration.rs
//!
//! serialization/mod.rs
//!     └── publicly composes the modules above
//! ```
//!
//! The diagram summarizes conceptual responsibilities, not every Rust import.
//! In particular, module implementation must not depend on this composition
//! root for foundational types when doing so would create a dependency cycle.
//!
//! ## Artifact lifecycle
//!
//! A typical persistence workflow is:
//!
//! 1. Obtain a payload from the canonical HDLIR payload codec.
//! 2. Construct and validate `ArtifactFormat` metadata.
//! 3. Encode the metadata and payload with the artifact encoder.
//! 4. Decode external bytes using explicit operational budgets and
//!    compatibility requirements.
//! 5. If necessary, migrate the extracted metadata and payload through
//!    explicitly registered migration steps.
//! 6. Decode the payload using its owning payload codec.
//! 7. Verify the resulting HDLIR with the canonical HDLIR verifier.
//!
//! Decoding an artifact does not establish that its payload is valid HDLIR.
//! A valid metadata envelope does not prove semantic correctness, safety,
//! authenticity, target feasibility, synthesis success, or equivalence.
//!
//! Callers must not bypass the payload codec or IR verifier merely because
//! artifact framing and metadata checks succeeded.
//!
//! ## Wire-format contract
//!
//! The current sibling encoder and decoder define the initial framed format:
//!
//! - Eight-byte magic: `ZHDLIR\\r\\n`.
//! - Eight-byte unsigned big-endian metadata-header length.
//! - Eight-byte unsigned big-endian payload length.
//! - Compact UTF-8 JSON metadata header.
//! - Exact opaque payload bytes.
//!
//! Payload integrity, when declared, is SHA-256 over the exact payload bytes,
//! excluding the framing prefix and metadata header.
//!
//! The framing contract must be ratified by `HDLIR-SPEC.md` and
//! `compatibility.md` before serialized artifacts are treated as a stable,
//! externally supported interchange format. Once published, changing the
//! framing requires an explicit compatibility decision and appropriate
//! versioning or migration.
//!
//! ## Scalability and operational budgets
//!
//! This module introduces no universal hardware-capacity ceilings.
//! It must not introduce fixed maximum counts for modules, signals, ports,
//! operations, memories, widths, clock domains, or hardware instances.
//!
//! Artifact sizes and framing lengths use the checked conversions and
//! arithmetic of the underlying encoder and decoder. The wire protocol's
//! finite integer fields are representation constraints, not source-language
//! limits on hardware dimensions.
//!
//! The caller may configure operational limits for metadata bytes, payload
//! bytes, complete artifact size, migration steps, and migration output.
//! These limits are execution policies, not language-semantic restrictions.
//!
//! An unset limit does not guarantee unlimited processing. Memory, address
//! space, integer representations, I/O, and execution time remain finite.
//! Resource exhaustion must be returned as an explicit error, never treated
//! as successful or complete processing.
//!
//! Streaming output should be preferred when duplicating a complete artifact
//! in memory is unnecessary. The decoded payload currently has an owned
//! in-memory representation; this API does not claim end-to-end zero-copy or
//! streaming payload decoding.
//!
//! ## Error and trust boundaries
//!
//! Public operations preserve their underlying structured errors. This
//! module must not flatten them into strings or silently convert failures
//! into default metadata, truncated payloads, or successful empty artifacts.
//!
//! Input artifacts must be treated as untrusted. Callers must configure
//! appropriate budgets and compatibility requirements before decoding.
//! Unknown required features or unsupported required definitions must not
//! be interpreted as understood.
//!
//! A SHA-256 digest detects a mismatch against the declared payload digest.
//! It does not establish who created an artifact or whether its contents
//! are authorized or trustworthy.
//!
//! Migration callbacks are explicit extension points. Registering a callback
//! does not prove that it preserves semantics. Each migration implementation
//! must reject cases it cannot handle correctly and must not silently discard
//! required hardware behavior, constraints, extensions, or provenance.
//!
//! ## Determinism and compatibility
//!
//! Metadata ordering and artifact encoding follow the contracts implemented
//! in `format.rs` and `encode.rs`. This module must not introduce
//! nondeterministic registration or export order.
//!
//! The format version is distinct from the Zamani language version, crate
//! version, grammar version, ZUIR version, and individual schema/dialect
//! versions.
//!
//! Compatibility checks do not replace schema and dialect resolution.
//! Migration is permitted only through explicitly registered paths; version
//! proximity must never be treated as evidence that a conversion is valid.
//!
//! ## Public API policy
//!
//! Submodules are public so callers that require specialized control can use
//! the explicit lower-level APIs. Common types and operations are also
//! re-exported below to provide a deliberate, stable import surface.
//!
//! Re-exports must refer to the existing sibling implementations. New
//! public exports must have documentation and integration tests.
//!
//! ## Freeze criteria
//!
//! This module is complete when:
//!
//! - Every declared submodule exists and compiles.
//! - Every public re-export resolves to the intended existing item.
//! - No duplicate encoder, decoder, metadata model, or migration registry
//!   is introduced.
//! - The module integrates with the actual parent HDLIR module tree.
//! - Format, encoding, decoding, and migration tests pass.
//! - Malformed input and budget exhaustion remain explicit failures.
//! - Determinism and compatibility behavior are tested.
//! - Repository formatting, build, test, and Clippy checks pass.
//!
//! These criteria must be verified against the actual repository checkout;
//! this file alone cannot certify them.

/// Validated metadata and compatibility declarations for HDLIR artifacts.
pub mod format;

/// Deterministic encoding and framing of HDLIR artifacts.
pub mod encode;

/// Safe decoding and validation of HDLIR artifact envelopes.
pub mod decode;

/// Explicit, versioned HDLIR artifact migrations.
pub mod migration;

// ---------------------------------------------------------------------------
// Artifact format and metadata
// ---------------------------------------------------------------------------

pub use self::format::{
    ArtifactFormat,
    CanonicalOrdering,
    DefinitionRequirement,
    FormatError,
    IntegrityAlgorithm,
    PayloadIntegrity,
    HDLIR_ARTIFACT_FORMAT_ID,
    SHA256_ALGORITHM_ID,
};

// ---------------------------------------------------------------------------
// Artifact encoding
// ---------------------------------------------------------------------------

pub use self::encode::{
    encode_artifact,
    encode_artifact_to,
    EncodeError,
    EncodeOptions,
    FRAME_PREFIX_LEN,
    HDLIR_ARTIFACT_MAGIC,
};

// ---------------------------------------------------------------------------
// Artifact decoding
// ---------------------------------------------------------------------------

pub use self::decode::{
    decode_artifact,
    decode_artifact_from,
    DecodeError,
    DecodeOptions,
    DecodedArtifact,
};

// ---------------------------------------------------------------------------
// Explicit artifact migrations
// ---------------------------------------------------------------------------

pub use self::migration::{
    MigratedArtifact,
    MigrationError,
    MigrationOptions,
    MigrationRegistry,
    MigrationStep,
    MigrationTransform,
    MigrationTransformError,
};
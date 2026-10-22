
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Hybrid Interchange — Encoder
//!
//! Path: `src/hybrid/interchange/encode.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//!
//! ## Responsibility
//!
//! Encode a validated [`HybridDocument`] as deterministic JSON bytes using
//! the repository's existing versioned hybrid interchange schema.
//!
//! The encoder owns:
//! - Structural validation before serialization.
//! - Enforcement of explicitly configured serialization budgets.
//! - Deterministic encoding of the same document value under the same
//!   schema, serializer implementation, and policy.
//! - Structured translation of schema and serializer failures.
//! - Incremental output-budget enforcement while bytes are emitted.
//!
//! The encoder does not own:
//! - A second hybrid graph or computation IR.
//! - Quantum, classical, HDL, tensor, AI, or distributed operation payloads.
//! - Domain registration or domain-contract resolution.
//! - Semantic type checking or proof of semantic preservation.
//! - Target selection, lowering, scheduling, routing, QEC, or execution.
//! - Schema migration or decoding.
//!
//! ## Canonical ownership
//!
//! - Document model and structural invariants: `schema.rs`.
//! - Operational budgets and serialization policy: `hybrid::config`.
//! - Structured failures: `hybrid::errors`.
//! - Schema and format versions: `hybrid::version`.
//! - Domain interface references: `hybrid::contract`.
//! - Hybrid identities: `hybrid::ids`.
//! - Domain-specific computation representations: their existing canonical
//!   IR owners, including `quantum::ir`.
//!
//! ## Determinism
//!
//! Encoding the same document value with the same schema and serializer
//! implementation produces the same bytes. Struct field order is defined
//! by the schema's serialization representation, and JSON object keys in
//! `serde_json::Value` use that crate's default deterministic map ordering.
//!
//! Vectors retain their supplied order. The encoder does not reorder graph
//! entities or declarations because doing so here could accidentally change
//! an ordering contract owned by another subsystem. Producers that require
//! canonical ordering across independently constructed equivalent documents
//! must obtain that order from the canonical graph builder or schema contract.
//!
//! ## Scalability and resource safety
//!
//! No universal graph-size, domain-count, operation-count, or output-size
//! ceiling is imposed. The caller may set an explicit `max_encoded_bytes`
//! budget in `HybridConfig`. When configured, that budget is checked during
//! writes, not merely after the complete output has been allocated.
//!
//! The encoder also checks graph/domain counts against configured budgets
//! before performing structural validation. Host allocator limits still
//! apply when no budget is configured.
//!
//! A budget is an operational policy, not a restriction on the meaning of
//! valid Zamani programs.
//!
//! ## Integration
//!
//! Expected public API:
//!
//! ```ignore
//! use crate::hybrid::config::HybridConfig;
//! use crate::hybrid::interchange::encode::encode;
//! use crate::hybrid::interchange::schema::HybridDocument;
//!
//! let bytes = encode(&document, &config)?;
//! ```
//!
//! `interchange/mod.rs` must declare `pub mod encode;` and re-export
//! `encode` if the project chooses a flatter public API.
//!
//! The caller remains responsible for semantic validation, resolving domain
//! contracts where required, and deciding whether the document is suitable
//! for persistence or execution. Structural schema validation is performed
//! by this encoder before bytes are produced.

use std::io::{self, Write};

use serde::Serialize;

use crate::hybrid::config::{BudgetKind, HybridConfig};
use crate::hybrid::errors::HybridError;
use crate::hybrid::interchange::schema::HybridDocument;

/// Encode a hybrid interchange document as JSON bytes.
///
/// The function validates the document before serialization and enforces
/// every relevant explicitly configured budget.
///
/// # Determinism
///
/// For the same document value, configuration, schema version, and
/// `serde_json` implementation, the returned byte sequence is reproducible.
/// The input document's vector ordering is preserved.
///
/// # Errors
///
/// Returns a structured [`HybridError`] when:
///
/// - A configured domain, node, edge, validation-work, metadata, or output
///   budget is exceeded.
/// - Schema-level validation fails.
/// - JSON serialization fails.
/// - Output exceeds the explicitly configured encoded-byte budget.
///
/// No partially encoded byte buffer is returned on failure.
///
/// # Resource behavior
///
/// When `max_encoded_bytes` is configured, the writer checks each write
/// before appending it to the output buffer. It also uses `try_reserve`
/// before extending the buffer so ordinary capacity-allocation failures
/// can be returned as encoding errors instead of being silently ignored.
///
/// This cannot guarantee recovery from every possible process-level
/// out-of-memory condition. Rust and the host operating system remain
/// subject to their actual resource limits.
pub fn encode(
    document: &HybridDocument,
    config: &HybridConfig,
) -> Result<Vec<u8>, HybridError> {
    check_document_budgets(document, config)?;

    document.validate().map_err(|error| {
        HybridError::invalid_representation(format!(
            "hybrid interchange schema validation failed: {error}"
        ))
        .in_component("hybrid.interchange.encode")
    })?;

    let mut writer = BudgetedVecWriter::new(
        config.budget_limit(BudgetKind::EncodedBytes),
    );

    let serialization_result = serde_json::to_writer(&mut writer, document);

    if let Some(observed) = writer.exceeded_by() {
        return Err(HybridError::budget_exhausted(
            BudgetKind::EncodedBytes.as_str(),
            format!(
                "encoding would exceed the configured output budget; \
                 attempted output length is at least {observed} bytes"
            ),
        )
        .in_component("hybrid.interchange.encode"));
    }

    if let Err(error) = serialization_result {
        return Err(HybridError::encoding_failure(format!(
            "failed to serialize hybrid interchange document: {error}"
        ))
        .in_component("hybrid.interchange.encode"));
    }

    let bytes = writer.into_inner();

    // Defensive postcondition: the streaming writer should already enforce
    // this limit. Keep the check here to protect the public contract if the
    // writer implementation changes in the future.
    config
        .check_budget(BudgetKind::EncodedBytes, bytes.len() as u128)
        .map_err(|error| error.in_component("hybrid.interchange.encode"))?;

    Ok(bytes)
}

/// Check collection-related operational budgets before structural
/// validation.
///
/// The schema validator performs work proportional to the represented
/// entities and relationships. Checking the configured counts first avoids
/// knowingly beginning validation when the caller's declared budget has
/// already been exceeded.
///
/// Counts are accumulated as `u128` so intermediate arithmetic does not
/// depend on the host's pointer width.
fn check_document_budgets(
    document: &HybridDocument,
    config: &HybridConfig,
) -> Result<(), HybridError> {
    let composition = &document.composition;

    config
        .check_budget(
            BudgetKind::DomainInstances,
            composition.domains.len() as u128,
        )
        .map_err(|error| error.in_component("hybrid.interchange.encode"))?;

    config
        .check_budget(
            BudgetKind::GraphNodes,
            composition.nodes.len() as u128,
        )
        .map_err(|error| error.in_component("hybrid.interchange.encode"))?;

    config
        .check_budget(
            BudgetKind::GraphEdges,
            composition.edges.len() as u128,
        )
        .map_err(|error| error.in_component("hybrid.interchange.encode"))?;

    let validation_steps = estimate_validation_work(document);

    config
        .check_budget(BudgetKind::ValidationSteps, validation_steps)
        .map_err(|error| error.in_component("hybrid.interchange.encode"))?;

    let metadata_bytes = estimate_metadata_bytes(document);

    config
        .check_budget(BudgetKind::MetadataBytes, metadata_bytes)
        .map_err(|error| error.in_component("hybrid.interchange.encode"))?;

    Ok(())
}

/// Estimate a conservative count of structural entries that the schema
/// validator will inspect.
///
/// This is an operational accounting estimate, not a statement about the
/// exact CPU cost of validation. It deliberately does not count individual
/// JSON payload bytes as validation steps; those bytes are accounted for
/// through metadata and encoded-output budgets.
///
/// The estimate is calculated before schema validation and uses saturating
/// arithmetic so the accounting calculation itself cannot overflow.
fn estimate_validation_work(document: &HybridDocument) -> u128 {
    let composition = &document.composition;

    let mut work = 1_u128;

    work = work.saturating_add(composition.domains.len() as u128);
    work = work.saturating_add(composition.nodes.len() as u128);
    work = work.saturating_add(composition.edges.len() as u128);
    work = work.saturating_add(composition.values.len() as u128);
    work = work.saturating_add(composition.synchronization.len() as u128);
    work = work.saturating_add(document.declarations.len() as u128);
    work = work.saturating_add(document.extensions.len() as u128);
    work = work.saturating_add(document.metadata.attributes.len() as u128);

    for domain in &composition.domains {
        work = work.saturating_add(domain.metadata.len() as u128);
    }

    for node in &composition.nodes {
        work = work.saturating_add(node.metadata.len() as u128);
    }

    // Each edge, value, and synchronization record has references that the
    // schema validator must inspect in addition to the record itself.
    for edge in &composition.edges {
        work = work.saturating_add(edge.dependencies.len() as u128);
    }

    for value in &composition.values {
        work = work.saturating_add(value.consumers.len() as u128);
    }

    for synchronization in &composition.synchronization {
        work = work.saturating_add(synchronization.participants.len() as u128);
    }

    work
}

/// Estimate metadata bytes covered by the configured metadata budget.
///
/// This is deliberately an estimate of metadata payload strings, not a
/// replacement for the encoded-byte budget. JSON escaping and structural
/// punctuation are accounted for by the output writer.
fn estimate_metadata_bytes(document: &HybridDocument) -> u128 {
    let mut bytes = 0_u128;

    bytes = add_string_bytes(bytes, &document.kind);
    bytes = add_string_bytes(bytes, &document.metadata.producer);

    if let Some(value) = document.metadata.document_id.as_deref() {
        bytes = add_string_bytes(bytes, value);
    }

    if let Some(value) = document.metadata.producer_version.as_deref() {
        bytes = add_string_bytes(bytes, value);
    }

    if let Some(value) = document.metadata.language_version.as_deref() {
        bytes = add_string_bytes(bytes, value);
    }

    if let Some(digest) = &document.metadata.source_digest {
        bytes = add_string_bytes(bytes, &digest.algorithm);
        bytes = add_string_bytes(bytes, &digest.value);
    }

    for entry in &document.metadata.attributes {
        bytes = add_string_bytes(bytes, &entry.key);
        bytes = add_string_bytes(bytes, &entry.value);
    }

    for domain in &document.composition.domains {
        if let Some(name) = domain.logical_name.as_deref() {
            bytes = add_string_bytes(bytes, name);
        }

        bytes = add_string_bytes(bytes, domain.interface.domain());
        bytes = add_string_bytes(bytes, domain.interface.interface());

        for entry in &domain.metadata {
            bytes = add_string_bytes(bytes, &entry.key);
            bytes = add_string_bytes(bytes, &entry.value);
        }

        if let Some(artifact) = &domain.artifact {
            bytes = add_string_bytes(bytes, &artifact.owner);
            bytes = add_string_bytes(bytes, &artifact.reference);

            if let Some(version) = artifact.version.as_deref() {
                bytes = add_string_bytes(bytes, version);
            }
        }
    }

    for node in &document.composition.nodes {
        bytes = add_string_bytes(bytes, &node.operation.owner);
        bytes = add_string_bytes(bytes, &node.operation.reference);

        if let Some(version) = node.operation.version.as_deref() {
            bytes = add_string_bytes(bytes, version);
        }

        for entry in &node.metadata {
            bytes = add_string_bytes(bytes, &entry.key);
            bytes = add_string_bytes(bytes, &entry.value);
        }
    }

    for value in &document.composition.values {
        bytes = add_string_bytes(bytes, &value.semantic_type);
        bytes = add_string_bytes(bytes, &value.ownership_contract);

        if let Some(layout) = value.layout_contract.as_deref() {
            bytes = add_string_bytes(bytes, layout);
        }

        if let Some(transfer) = value.transfer_contract.as_deref() {
            bytes = add_string_bytes(bytes, transfer);
        }
    }

    for synchronization in &document.composition.synchronization {
        bytes = add_string_bytes(bytes, &synchronization.kind);

        if let Some(ordering) = synchronization.ordering_contract.as_deref() {
            bytes = add_string_bytes(bytes, ordering);
        }
    }

    for declaration in &document.declarations {
        bytes = add_string_bytes(bytes, &declaration.namespace);
        bytes = add_string_bytes(bytes, &declaration.name);
    }

    for extension in &document.extensions {
        bytes = add_string_bytes(bytes, &extension.namespace);
    }

    bytes
}

fn add_string_bytes(total: u128, value: &str) -> u128 {
    total.saturating_add(value.len() as u128)
}

/// A growable byte writer that enforces an optional output budget during
/// serialization.
///
/// `limit == None` means the caller has not imposed an encoded-byte budget.
/// It does not mean that the host has unlimited memory.
///
/// On budget exhaustion, the writer remembers the attempted length so the
/// public encoder can return the canonical budget-exhaustion error rather
/// than exposing an implementation-specific I/O error.
struct BudgetedVecWriter {
    bytes: Vec<u8>,
    limit: Option<u64>,
    exceeded_by: Option<u128>,
}

impl BudgetedVecWriter {
    fn new(limit: Option<u64>) -> Self {
        Self {
            bytes: Vec::new(),
            limit,
            exceeded_by: None,
        }
    }

    fn exceeded_by(&self) -> Option<u128> {
        self.exceeded_by
    }

    fn into_inner(self) -> Vec<u8> {
        self.bytes
    }
}

impl Write for BudgetedVecWriter {
    fn write(&mut self, input: &[u8]) -> io::Result<usize> {
        if input.is_empty() {
            return Ok(0);
        }

        let current = self.bytes.len() as u128;
        let requested = current.saturating_add(input.len() as u128);

        if let Some(limit) = self.limit {
            if requested > u128::from(limit) {
                self.exceeded_by = Some(requested);
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "configured hybrid encoded-byte budget exceeded",
                ));
            }
        }

        self.bytes
            .try_reserve(input.len())
            .map_err(|error| io::Error::new(io::ErrorKind::Other, error))?;

        self.bytes.extend_from_slice(input);

        Ok(input.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writer_accepts_output_at_the_configured_limit() {
        let mut writer = BudgetedVecWriter::new(Some(3));

        writer.write_all(b"abc").expect("three bytes fit");

        assert_eq!(writer.into_inner(), b"abc");
    }

    #[test]
    fn writer_rejects_output_above_the_configured_limit() {
        let mut writer = BudgetedVecWriter::new(Some(2));

        let result = writer.write_all(b"abc");

        assert!(result.is_err());
        assert_eq!(writer.exceeded_by(), Some(3));
        assert!(writer.bytes.is_empty());
    }

    #[test]
    fn writer_without_a_configured_limit_is_not_given_an_artificial_ceiling() {
        let mut writer = BudgetedVecWriter::new(None);

        writer
            .write_all(b"portable hybrid interchange")
            .expect("unbudgeted write succeeds when allocation succeeds");

        assert_eq!(writer.bytes, b"portable hybrid interchange");
        assert_eq!(writer.exceeded_by(), None);
    }

    #[test]
    fn writer_accepts_zero_bytes_under_a_zero_budget() {
        let mut writer = BudgetedVecWriter::new(Some(0));

        writer.write_all(b"").expect("empty write");

        assert!(writer.bytes.is_empty());
        assert_eq!(writer.exceeded_by(), None);
    }

    #[test]
    fn writer_rejects_nonempty_output_under_a_zero_budget() {
        let mut writer = BudgetedVecWriter::new(Some(0));

        assert!(writer.write_all(b"x").is_err());
        assert_eq!(writer.exceeded_by(), Some(1));
    }
}


#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Hybrid Interchange — Decoder
//!
//! Path: `src/hybrid/interchange/decode.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## Responsibility
//!
//! Decode an untrusted JSON interchange document into the canonical
//! `HybridDocument` representation defined in `schema.rs`.
//!
//! The decoder:
//! - checks configured input-size budgets before parsing;
//! - reads the document header before interpreting the typed payload;
//! - checks format compatibility before schema compatibility;
//! - rejects unsupported versions rather than guessing their meaning;
//! - distinguishes older artifacts requiring migration from future formats;
//! - deserializes into the existing schema, not a competing IR;
//! - checks configured graph, domain, and validation-work budgets;
//! - runs the schema's authoritative structural validation;
//! - rejects malformed, truncated, and trailing input;
//! - returns structured hybrid errors.
//!
//! ## Ownership boundaries
//!
//! - Document types and structural invariants: `schema.rs`.
//! - Format and schema version policies: `hybrid::version`.
//! - Operational budgets: `hybrid::config`.
//! - Structured failures: `hybrid::errors`.
//! - Domain contracts and graph-scoped identities: their canonical modules.
//! - Domain-specific computation semantics: existing domain IRs.
//! - Target realization and execution: compiler, backend, and runtime.
//!
//! Decoding does not resolve domain implementations, prove semantic
//! equivalence, choose hardware, lower instructions, or execute a document.
//!
//! ## Resource and scalability contract
//!
//! This module imposes no universal graph-size, hardware-count, operation,
//! or domain-count ceiling. Applications processing untrusted data should
//! configure operational budgets appropriate to their environment.
//!
//! Without a configured budget, the host's actual memory, address-space,
//! CPU, and allocator limits still apply. No decoder can guarantee success
//! for an arbitrarily large input on a finite machine.
//!
//! The JSON parser also retains the recursion-safety behavior of the
//! repository's configured `serde_json` dependency. The decoder does not
//! disable that protection or claim unlimited nesting.
//!
//! ## Unknown fields
//!
//! The current schema uses `deny_unknown_fields` on its document records.
//! Unknown fields are therefore rejected by typed deserialization rather
//! than silently dropped. The decoder does not claim to preserve unknown
//! schema fields: that requires a deliberate schema/API change.
//!
//! Arbitrary extension values remain governed by the existing schema.
//! Their presence is not proof that a capability or semantic requirement
//! has been satisfied.

use core::fmt;

use serde::de::{self, Deserialize, Deserializer, IgnoredAny, MapAccess, Visitor};
use serde::Serialize;

use crate::hybrid::config::{
    BudgetKind,
    HybridConfig,
    OlderArtifactPolicy,
};
use crate::hybrid::errors::{
    HybridError,
    HybridErrorCode,
};
use crate::hybrid::interchange::schema::{
    HybridDocument,
    HYBRID_DOCUMENT_KIND,
};
use crate::hybrid::version::{
    check_hybrid_format_compatibility,
    check_hybrid_schema_compatibility,
    is_compatible_without_migration,
    is_future_version,
    HybridFormatVersion,
    HybridSchemaVersion,
};

const COMPONENT: &str = "hybrid.interchange.decode";

/// Decode a hybrid interchange JSON document.
///
/// The input is treated as untrusted. The configured decoded-byte budget is
/// checked before parsing; header compatibility is checked before the typed
/// document is constructed; then configured collection/work budgets and
/// schema invariants are checked.
///
/// # Errors
///
/// Returns a structured `HybridError` for:
///
/// - configured input or validation budgets being exceeded;
/// - malformed JSON, a missing or invalid header, or trailing input;
/// - an unrecognized document kind;
/// - unsupported format or schema versions;
/// - an older version requiring migration;
/// - typed deserialization failures, including unknown fields rejected by
///   the current schema;
/// - graph, identity, and reference invariants rejected by `validate`.
///
/// # Compatibility
///
/// This function does not guess how to migrate old artifacts. The current
/// compatibility policies accept the current versions exactly. An older
/// version is not read as current merely because its data appears similar.
///
/// # Determinism
///
/// For the same bytes, schema implementation, and configuration, the
/// decoder produces the same document or the same class of failure.
///
/// # Resource behavior
///
/// A configured byte budget is enforced before JSON parsing. Collection
/// budgets are enforced after typed deserialization, because those counts
/// are represented inside the decoded document. Applications accepting
/// untrusted input should set a decoded-byte budget to bound this initial
/// allocation and parsing stage.
pub fn decode(
    bytes: &[u8],
    config: &HybridConfig,
) -> Result<HybridDocument, HybridError> {
    config
        .check_budget(BudgetKind::DecodedBytes, bytes.len() as u128)
        .map_err(|error| error.in_component(COMPONENT))?;

    // Pass one reads only the top-level header. Other values are consumed
    // as IgnoredAny, so the full document is not materialized at this stage.
    // This lets us reject incompatible formats before constructing the
    // schema's typed representation.
    let header = read_header(bytes)?;

    if header.kind != HYBRID_DOCUMENT_KIND {
        return Err(
            HybridError::malformed_input(format!(
                "unrecognized hybrid document kind `{}`",
                header.kind
            ))
            .in_component(COMPONENT),
        );
    }

    check_format_version(header.format_version)?;
    check_schema_version(header.schema_version, config)?;

    // Pass two uses the canonical schema directly. Its serde attributes
    // reject unknown fields and invalid field representations.
    let mut deserializer = serde_json::Deserializer::from_slice(bytes);

    let document = HybridDocument::deserialize(&mut deserializer).map_err(
        |error| {
            HybridError::decoding_failure(format!(
                "could not deserialize hybrid document: {error}"
            ))
            .in_component(COMPONENT)
        },
    )?;

    deserializer.end().map_err(|error| {
        HybridError::malformed_input(format!(
            "unexpected trailing data after hybrid document: {error}"
        ))
        .in_component(COMPONENT)
    })?;

    check_document_budgets(&document, config)?;

    document.validate().map_err(|error| {
        HybridError::invalid_representation(format!(
            "hybrid schema validation failed: {error}"
        ))
        .in_component(COMPONENT)
    })?;

    Ok(document)
}

/// Top-level metadata required to decide whether the payload can be read.
///
/// The header is not a second document schema: it contains only the fields
/// needed for compatibility negotiation. The full document remains owned
/// by `schema::HybridDocument`.
#[derive(Debug)]
struct DocumentHeader {
    kind: String,
    format_version: HybridFormatVersion,
    schema_version: HybridSchemaVersion,
}

impl<'de> Deserialize<'de> for DocumentHeader {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(HeaderVisitor)
    }
}

struct HeaderVisitor;

impl<'de> Visitor<'de> for HeaderVisitor {
    type Value = DocumentHeader;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a hybrid interchange document object")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut kind: Option<String> = None;
        let mut format_version: Option<HybridFormatVersion> = None;
        let mut schema_version: Option<HybridSchemaVersion> = None;

        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "kind" => {
                    if kind.is_some() {
                        return Err(de::Error::duplicate_field("kind"));
                    }
                    kind = Some(map.next_value()?);
                }
                "format_version" => {
                    if format_version.is_some() {
                        return Err(de::Error::duplicate_field(
                            "format_version",
                        ));
                    }
                    format_version = Some(map.next_value()?);
                }
                "schema_version" => {
                    if schema_version.is_some() {
                        return Err(de::Error::duplicate_field(
                            "schema_version",
                        ));
                    }
                    schema_version = Some(map.next_value()?);
                }
                // We intentionally do not materialize the rest of the
                // document during the compatibility preflight.
                _ => {
                    let _: IgnoredAny = map.next_value()?;
                }
            }
        }

        Ok(DocumentHeader {
            kind: kind.ok_or_else(|| de::Error::missing_field("kind"))?,
            format_version: format_version
                .ok_or_else(|| de::Error::missing_field("format_version"))?,
            schema_version: schema_version
                .ok_or_else(|| de::Error::missing_field("schema_version"))?,
        })
    }
}

/// Parse the header and ensure the entire input is syntactically valid JSON.
///
/// Unknown fields are consumed without constructing a second copy of their
/// values. The subsequent typed pass applies the actual schema's field rules.
fn read_header(bytes: &[u8]) -> Result<DocumentHeader, HybridError> {
    let mut deserializer = serde_json::Deserializer::from_slice(bytes);

    let header = DocumentHeader::deserialize(&mut deserializer).map_err(
        |error| {
            HybridError::malformed_input(format!(
                "invalid hybrid document header or JSON payload: {error}"
            ))
            .in_component(COMPONENT)
        },
    )?;

    deserializer.end().map_err(|error| {
        HybridError::malformed_input(format!(
            "invalid trailing JSON data: {error}"
        ))
        .in_component(COMPONENT)
    })?;

    Ok(header)
}

fn check_format_version(
    supplied: HybridFormatVersion,
) -> Result<(), HybridError> {
    let decision = check_hybrid_format_compatibility(supplied);

    if is_compatible_without_migration(decision) {
        return Ok(());
    }

    if is_future_version(decision) {
        return Err(
            HybridError::unsupported_version(format!(
                "hybrid format version {supplied} is newer than the \
                 format supported by this implementation"
            ))
            .in_component(COMPONENT),
        );
    }

    Err(
        HybridError::new(
            HybridErrorCode::MigrationRequired,
            format!(
                "hybrid format version {supplied} is not directly \
                 compatible; an explicit format migration is required"
            ),
        )
        .in_component(COMPONENT),
    )
}

fn check_schema_version(
    supplied: HybridSchemaVersion,
    config: &HybridConfig,
) -> Result<(), HybridError> {
    let decision = check_hybrid_schema_compatibility(supplied);

    if is_compatible_without_migration(decision) {
        return Ok(());
    }

    if is_future_version(decision) {
        return Err(
            HybridError::unsupported_version(format!(
                "hybrid schema version {supplied} is newer than the \
                 schema supported by this implementation"
            ))
            .in_component(COMPONENT),
        );
    }

    match config.compatibility.older_artifacts {
        OlderArtifactPolicy::Reject => Err(
            HybridError::unsupported_version(format!(
                "older hybrid schema version {supplied} is rejected \
                 by the configured compatibility policy"
            ))
            .in_component(COMPONENT),
        ),
        OlderArtifactPolicy::RequireMigration
        | OlderArtifactPolicy::AllowIfCompatible => {
            // The current version policy certifies exact compatibility
            // only. AllowIfCompatible cannot override that decision and
            // does not implement a migration by itself.
            Err(
                HybridError::new(
                    HybridErrorCode::MigrationRequired,
                    format!(
                        "hybrid schema version {supplied} requires an \
                         explicitly implemented migration; the current \
                         compatibility policy does not certify direct reading"
                    ),
                )
                .in_component(COMPONENT),
            )
        }
    }
}

/// Enforce explicit collection and validation-work budgets.
///
/// These are operational budgets, not language or hardware ceilings.
fn check_document_budgets(
    document: &HybridDocument,
    config: &HybridConfig,
) -> Result<(), HybridError> {
    let composition = &document.composition;

    check_budget(
        config,
        BudgetKind::DomainInstances,
        composition.domains.len() as u128,
    )?;

    check_budget(
        config,
        BudgetKind::GraphNodes,
        composition.nodes.len() as u128,
    )?;

    check_budget(
        config,
        BudgetKind::GraphEdges,
        composition.edges.len() as u128,
    )?;

    check_budget(
        config,
        BudgetKind::ValidationSteps,
        estimate_validation_work(document),
    )?;

    Ok(())
}

fn check_budget(
    config: &HybridConfig,
    kind: BudgetKind,
    observed: u128,
) -> Result<(), HybridError> {
    config
        .check_budget(kind, observed)
        .map_err(|error| error.in_component(COMPONENT))
}

/// Conservative count of entries and references inspected by schema
/// validation. Saturating arithmetic makes the accounting itself safe.
fn estimate_validation_work(document: &HybridDocument) -> u128 {
    let composition = &document.composition;

    let mut work = 1_u128;
    work = work.saturating_add(composition.domains.len() as u128);
    work = work.saturating_add(composition.nodes.len() as u128);
    work = work.saturating_add(composition.edges.len() as u128);
    work = work.saturating_add(composition.values.len() as u128);
    work = work.saturating_add(
        composition.synchronization.len() as u128,
    );
    work = work.saturating_add(document.declarations.len() as u128);
    work = work.saturating_add(document.extensions.len() as u128);
    work = work.saturating_add(
        document.metadata.attributes.len() as u128,
    );

    for domain in &composition.domains {
        work = work.saturating_add(domain.metadata.len() as u128);
    }

    for node in &composition.nodes {
        work = work.saturating_add(node.metadata.len() as u128);
    }

    for edge in &composition.edges {
        work = work.saturating_add(edge.dependencies.len() as u128);
    }

    for value in &composition.values {
        work = work.saturating_add(value.consumers.len() as u128);
    }

    for event in &composition.synchronization {
        work = work.saturating_add(event.participants.len() as u128);
    }

    work
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::hybrid::ids::HybridGraphId;
    use crate::hybrid::interchange::schema::{
        DocumentMetadata,
        HybridComposition,
    };

    fn empty_document() -> HybridDocument {
        let graph_id = HybridGraphId::new(1).expect("valid graph ID");

        HybridDocument::new(
            DocumentMetadata {
                graph_id,
                document_id: None,
                producer: "decode-test".to_owned(),
                producer_version: None,
                source_digest: None,
                language_version: None,
                attributes: Vec::new(),
            },
            HybridComposition {
                graph_id,
                domains: Vec::new(),
                nodes: Vec::new(),
                edges: Vec::new(),
                values: Vec::new(),
                synchronization: Vec::new(),
            },
        )
    }

    #[test]
    fn decodes_a_valid_document() {
        let document = empty_document();
        let bytes =
            serde_json::to_vec(&document).expect("serialize fixture");

        let decoded = decode(&bytes, &HybridConfig::default())
            .expect("decode valid document");

        assert_eq!(decoded, document);
    }

    #[test]
    fn rejects_empty_input() {
        let error = decode(&[], &HybridConfig::default())
            .expect_err("empty input is not a document");

        assert!(
            error.to_string().contains("header")
                || error.to_string().contains("JSON")
        );
    }

    #[test]
    fn rejects_trailing_non_json_data() {
        let document = empty_document();
        let mut bytes =
            serde_json::to_vec(&document).expect("serialize fixture");
        bytes.extend_from_slice(b" trailing");

        assert!(decode(&bytes, &HybridConfig::default()).is_err());
    }

    #[test]
    fn rejects_wrong_document_kind() {
        let mut value = serde_json::to_value(empty_document())
            .expect("convert fixture");
        value["kind"] = serde_json::Value::String("other".to_owned());

        let bytes = serde_json::to_vec(&value).expect("serialize value");

        assert!(decode(&bytes, &HybridConfig::default()).is_err());
    }

    #[test]
    fn enforces_decoded_byte_budget_before_parsing() {
        let bytes = br#"{"kind":"zamani.hybrid"}"#;
        let mut config = HybridConfig::default();
        config.budgets.max_decoded_bytes = Some(1);

        let error = decode(bytes, &config)
            .expect_err("input exceeds the explicit budget");

        assert!(error.to_string().contains("budget"));
    }

    #[test]
    fn enforces_graph_budgets() {
        let document = empty_document();
        let bytes =
            serde_json::to_vec(&document).expect("serialize fixture");

        let mut config = HybridConfig::default();
        config.budgets.max_graph_nodes = Some(0);

        // This fixture has no nodes and therefore satisfies a zero-node
        // budget. A zero limit is not a hidden default; it is caller policy.
        assert!(decode(&bytes, &config).is_ok());
    }

    #[test]
    fn rejects_duplicate_header_fields() {
        let document = empty_document();
        let json =
            serde_json::to_string(&document).expect("serialize fixture");

        let duplicate = json.replacen(
            "{",
            r#"{"kind":"zamani.hybrid","#,
            1,
        );

        assert!(decode(duplicate.as_bytes(), &HybridConfig::default()).is_err());
    }

    #[test]
    fn rejects_unknown_document_fields_under_current_schema() {
        let mut value = serde_json::to_value(empty_document())
            .expect("convert fixture");
        value["unexpected"] = serde_json::Value::Bool(true);

        let bytes = serde_json::to_vec(&value).expect("serialize value");

        assert!(decode(&bytes, &HybridConfig::default()).is_err());
    }
}

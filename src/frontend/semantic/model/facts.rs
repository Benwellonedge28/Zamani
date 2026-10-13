
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Semantic Model — Facts
//!
//! Path: `src/frontend/semantic/model/facts.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Define the stable, target-independent envelope used to identify and carry
//! semantic facts produced by the Zamani frontend semantic subsystem.
//!
//! A fact records information established by an owning semantic analysis
//! component. Examples include a resolved declaration, an inferred type, an
//! effect summary, a capability requirement, a symbolic resource constraint,
//! a contract obligation reference, or a domain-specific semantic result.
//!
//! This module provides the common identity and payload boundary. It does not
//! implement the rules that establish whether a fact is correct.
//!
//! ## AUTHORITY
//!
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/grammar-authority.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/Zamani.g4`
//! - `src/frontend/ast/node/`
//! - `src/frontend/semantic/model/program.rs`
//!
//! The normative specifications govern language meaning. The AST owns source
//! structure and AST node identity. Semantic subsystem modules own the meaning
//! and validation rules for their respective fact payloads.
//!
//! ## OWNS
//!
//! - The validated, namespaced `FactKey`.
//! - The `SemanticFact<T>` envelope.
//! - Key validation and stable key formatting.
//! - The common API for inspecting, consuming, and transforming fact payloads.
//!
//! ## DOES_NOT_OWN
//!
//! - AST nodes, AST storage, source spans, or AST node allocation.
//! - Symbol identity allocation or symbol resolution.
//! - The canonical semantic type system or type inference.
//! - Effect, capability, resource, contract, policy, or provenance semantics.
//! - Domain-specific semantic rules.
//! - Diagnostic collection, rendering, or ordering.
//! - The semantic program container or analysis lifecycle.
//! - Classical IR, quantum IR, HDL IR, or another intermediate representation.
//! - Target discovery, hardware feasibility, allocation, placement, scheduling,
//!   routing, calibration, error correction, or runtime execution.
//!
//! ## INTEGRATION CONTRACT
//!
//! - `program.rs` owns the association of facts with AST `NodeId` values.
//! - `SemanticFact<T>` is suitable as the fact payload type `F` in
//!   `SemanticProgram<F, O, P>`.
//! - The symbol subsystem owns declaration-resolution payloads.
//! - The type subsystem owns type-resolution and inference payloads.
//! - The effects subsystem owns effect payloads.
//! - The capabilities subsystem owns capability payloads.
//! - The resources subsystem owns symbolic resource payloads.
//! - The contracts subsystem owns contract and obligation payloads.
//! - Domain validators own their domain-specific payloads.
//! - `ids.rs` and `provenance.rs`, when integrated, retain their separate
//!   ownership of semantic identity and provenance.
//! - `model/mod.rs` exports this module after the surrounding model API is
//!   established.
//!
//! A semantic subsystem must not encode an entire semantic type, symbol table,
//! AST, or domain model as an unvalidated string merely to fit this envelope.
//! The payload `T` should be the owning subsystem's actual typed value.
//!
//! ## SCALABILITY
//!
//! This module defines no hardware-capacity ceilings, domain-count ceilings,
//! fixed payload-size limits, or language-level fact-count limits.
//!
//! Namespace and key storage grows according to supplied data and available
//! memory. Rust allocation and representational limits still apply.
//!
//! Any compiler work or memory budget belongs to configuration and analysis
//! session policy. Exceeding an operational budget must not be reported as
//! proof that a program is semantically valid.
//!
//! ## DETERMINISM
//!
//! Key equality, ordering, hashing, and display depend only on the validated
//! namespace and key name. They do not depend on machine capabilities,
//! allocation addresses, wall-clock time, randomness, or global state.
//!
//! The payload's own determinism is the responsibility of its owning type.
//!
//! ## ERROR HANDLING
//!
//! Invalid keys are rejected at construction. Empty names, empty namespace
//! segments, whitespace, separators other than the namespace dot, and
//! non-ASCII identifier characters are rejected.
//!
//! The validation rules are intentionally narrow and stable. They apply to
//! fact-key metadata, not to Zamani source identifiers. Source identifier
//! rules remain owned by the language specification and lexer/parser.
//!
//! ## FREEZE CRITERIA
//!
//! - Key validation and formatting tests pass.
//! - The payload type remains independent of AST storage and backend code.
//! - The model exports this module without creating circular dependencies.
//! - Semantic producers use typed payloads from their owning subsystems.
//! - The workspace and integration tests pass on the supported Rust toolchain.

use core::fmt;
use std::error::Error;
use std::sync::Arc;

/// A validated namespace and name identifying a kind of semantic fact.
///
/// A key identifies the meaning/category of a fact, not an AST node, symbol,
/// type instance, resource instance, or individual fact occurrence.
///
/// Namespaces are dot-separated. Each segment and the final name must begin
/// with an ASCII letter or underscore; remaining characters may also contain
/// ASCII digits or hyphens.
///
/// Examples:
///
/// - `core.names` / `resolved-declaration`
/// - `core.types` / `inferred-type`
/// - `effects` / `effect-summary`
/// - `resources` / `symbolic-requirement`
/// - `quantum` / `operation-validation`
/// - `hdl` / `signal-constraint`
/// - `future.domain` / `custom-fact`
///
/// These examples illustrate key metadata only. They do not declare that the
/// corresponding semantic feature has already been implemented.
///
/// Keys are open-ended: adding a new fact category does not require modifying
/// this type or a central enumeration.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FactKey {
    namespace: Arc<str>,
    name: Arc<str>,
}

/// The part of a fact key that failed validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum FactKeyComponent {
    /// The namespace is empty or contains an invalid segment.
    Namespace,

    /// The fact name is empty or contains an invalid character.
    Name,
}

impl fmt::Display for FactKeyComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Namespace => "namespace",
            Self::Name => "name",
        })
    }
}

/// Reason a fact key could not be constructed.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum FactKeyError {
    /// A namespace or one of its dot-separated segments is invalid.
    InvalidNamespace,

    /// The fact name is invalid.
    InvalidName,
}

impl fmt::Display for FactKeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNamespace => f.write_str(
                "invalid fact namespace: expected non-empty dot-separated \
                 ASCII identifier segments",
            ),
            Self::InvalidName => f.write_str(
                "invalid fact name: expected a non-empty ASCII identifier",
            ),
        }
    }
}

impl Error for FactKeyError {}

impl FactKey {
    /// Construct a fact key from a namespace and name.
    ///
    /// The inputs are validated before they become part of the semantic
    /// model. No normalization, case folding, or silent character rewriting
    /// is performed: different accepted spellings remain distinct keys.
    ///
    /// Namespace examples: `core`, `core.types`, `future.compute`.
    ///
    /// # Errors
    ///
    /// Returns `FactKeyError::InvalidNamespace` if the namespace is empty,
    /// has an empty segment, or contains characters outside the accepted
    /// namespace grammar.
    ///
    /// Returns `FactKeyError::InvalidName` if the name is empty or contains
    /// characters outside the accepted name grammar.
    pub fn new(
        namespace: impl AsRef<str>,
        name: impl AsRef<str>,
    ) -> Result<Self, FactKeyError> {
        let namespace = namespace.as_ref();
        let name = name.as_ref();

        if !is_valid_namespace(namespace) {
            return Err(FactKeyError::InvalidNamespace);
        }

        if !is_valid_identifier(name) {
            return Err(FactKeyError::InvalidName);
        }

        Ok(Self {
            namespace: Arc::from(namespace),
            name: Arc::from(name),
        })
    }

    /// Return the validated namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Return the validated fact name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the key as `namespace:name`.
    ///
    /// The returned string is intended for diagnostics, logs, inspection,
    /// and explicit serialization adapters. Callers should retain the
    /// structured `FactKey` for comparisons rather than repeatedly formatting
    /// it for internal lookups.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        let mut result = String::with_capacity(
            self.namespace.len() + 1 + self.name.len(),
        );
        result.push_str(&self.namespace);
        result.push(':');
        result.push_str(&self.name);
        result
    }
}

impl fmt::Display for FactKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.namespace, self.name)
    }
}

impl fmt::Debug for FactKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FactKey")
            .field("namespace", &self.namespace)
            .field("name", &self.name)
            .finish()
    }
}

/// A semantic fact payload with a validated, extensible key.
///
/// `T` is the payload type defined by the semantic subsystem that owns the
/// fact's meaning. It should be a real typed value, not a serialized AST or a
/// stringly typed substitute for the owning subsystem's model.
///
/// The envelope does not claim that the payload is correct. The producer is
/// responsible for running the appropriate semantic checks before publishing
/// a fact as established evidence.
///
/// A `SemanticFact<T>` can be stored as the `F` parameter of
/// `SemanticProgram<F, O, P>` in `program.rs`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SemanticFact<T> {
    key: FactKey,
    payload: T,
}

impl<T> SemanticFact<T> {
    /// Construct a fact from an already validated key and a typed payload.
    ///
    /// This method intentionally does not validate the payload. Its type and
    /// semantic invariants belong to the subsystem that defines `T`.
    #[must_use]
    pub const fn new(key: FactKey, payload: T) -> Self {
        Self { key, payload }
    }

    /// Return the fact key.
    #[must_use]
    pub const fn key(&self) -> &FactKey {
        &self.key
    }

    /// Borrow the typed payload.
    #[must_use]
    pub const fn payload(&self) -> &T {
        &self.payload
    }

    /// Mutably borrow the payload.
    ///
    /// The caller must ensure that modifying the payload does not invalidate
    /// semantic-model invariants or any cached analysis result. Prefer
    /// constructing a replacement fact when a change should be tracked as a
    /// new semantic result.
    pub fn payload_mut(&mut self) -> &mut T {
        &mut self.payload
    }

    /// Consume the fact and return its key and payload.
    #[must_use]
    pub fn into_parts(self) -> (FactKey, T) {
        (self.key, self.payload)
    }

    /// Consume the fact and transform its payload while preserving the key.
    ///
    /// This is useful when an owning subsystem replaces one representation
    /// with another without changing the category of fact being represented.
    #[must_use]
    pub fn map<U>(self, transform: impl FnOnce(T) -> U) -> SemanticFact<U> {
        SemanticFact {
            key: self.key,
            payload: transform(self.payload),
        }
    }

    /// Borrow the fact as a view containing the same key and payload reference.
    #[must_use]
    pub const fn as_ref(&self) -> SemanticFact<&T> {
        SemanticFact {
            key: self.key.clone(),
            payload: &self.payload,
        }
    }
}

/// Validate a fact namespace.
///
/// A namespace consists of one or more dot-separated identifiers. Empty
/// segments are rejected, so `.types`, `types.`, and `types..effects` are
/// invalid. A segment starts with an ASCII letter or underscore and may
/// continue with ASCII letters, digits, underscores, or hyphens.
fn is_valid_namespace(namespace: &str) -> bool {
    if namespace.is_empty() {
        return false;
    }

    namespace.split('.').all(is_valid_identifier)
}

/// Validate one key segment or key name.
///
/// This intentionally defines a metadata-key grammar, not Zamani's source
/// identifier grammar.
fn is_valid_identifier(value: &str) -> bool {
    let mut chars = value.chars();

    let Some(first) = chars.next() else {
        return false;
    };

    if !first.is_ascii_alphabetic() && first != '_' {
        return false;
    }

    chars.all(|ch| {
        ch.is_ascii_alphanumeric() || ch == '_' || ch == '-'
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_simple_and_nested_namespaces() {
        assert!(FactKey::new("core", "resolved-name").is_ok());
        assert!(FactKey::new("core.types", "inferred-type").is_ok());
        assert!(FactKey::new("future.compute.domain", "custom_fact").is_ok());
    }

    #[test]
    fn rejects_empty_or_malformed_namespaces() {
        for namespace in [
            "",
            ".core",
            "core.",
            "core..types",
            "core types",
            "core/types",
            "core:types",
            "cörë",
        ] {
            assert!(
                matches!(
                    FactKey::new(namespace, "fact"),
                    Err(FactKeyError::InvalidNamespace)
                ),
                "namespace should be rejected: {namespace:?}",
            );
        }
    }

    #[test]
    fn rejects_empty_or_malformed_names() {
        for name in [
            "",
            "1fact",
            "-fact",
            "fact name",
            "fact.name",
            "fact/name",
            "fáct",
        ] {
            assert!(
                matches!(
                    FactKey::new("core", name),
                    Err(FactKeyError::InvalidName)
                ),
                "name should be rejected: {name:?}",
            );
        }
    }

    #[test]
    fn key_format_is_stable() {
        let key = FactKey::new("core.types", "inferred-type")
            .expect("valid key");

        assert_eq!(key.namespace(), "core.types");
        assert_eq!(key.name(), "inferred-type");
        assert_eq!(key.qualified_name(), "core.types:inferred-type");
        assert_eq!(key.to_string(), "core.types:inferred-type");
    }

    #[test]
    fn key_equality_and_order_are_value_based() {
        let first =
            FactKey::new("core.types", "inferred-type").expect("valid key");
        let same =
            FactKey::new("core.types", "inferred-type").expect("valid key");
        let later =
            FactKey::new("core.types", "resolved-type").expect("valid key");

        assert_eq!(first, same);
        assert_ne!(first, later);
        assert!(first < later);
    }

    #[test]
    fn fact_preserves_typed_payload() {
        let key =
            FactKey::new("core.names", "resolved-declaration")
                .expect("valid key");
        let fact = SemanticFact::new(key.clone(), 42_u64);

        assert_eq!(fact.key(), &key);
        assert_eq!(*fact.payload(), 42);
    }

    #[test]
    fn fact_can_be_consumed_into_parts() {
        let key =
            FactKey::new("core.types", "inferred-type")
                .expect("valid key");
        let fact = SemanticFact::new(key.clone(), String::from("ExampleType"));

        let (actual_key, payload) = fact.into_parts();

        assert_eq!(actual_key, key);
        assert_eq!(payload, "ExampleType");
    }

    #[test]
    fn mapping_changes_payload_without_changing_key() {
        let key =
            FactKey::new("core.constants", "evaluated-value")
                .expect("valid key");
        let fact = SemanticFact::new(key.clone(), 21_u32);

        let mapped = fact.map(|value| value * 2);

        assert_eq!(mapped.key(), &key);
        assert_eq!(*mapped.payload(), 42);
    }

    #[test]
    fn borrowed_view_preserves_key_and_borrows_payload() {
        let key =
            FactKey::new("core.types", "inferred-type")
                .expect("valid key");
        let fact = SemanticFact::new(key.clone(), String::from("Tensor"));
        let view = fact.as_ref();

        assert_eq!(view.key(), &key);
        assert_eq!(view.payload().as_str(), "Tensor");
        assert_eq!(fact.payload(), "Tensor");
    }

    #[test]
    fn key_does_not_restrict_domain_names_to_a_closed_list() {
        let first =
            FactKey::new("quantum", "operation-validation")
                .expect("valid key");
        let future =
            FactKey::new("future.photonic.compute", "mode-constraint")
                .expect("valid key");

        assert_ne!(first, future);
        assert_eq!(future.namespace(), "future.photonic.compute");
    }
}

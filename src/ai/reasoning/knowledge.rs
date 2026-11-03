
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani AI — Knowledge Representation
//!
//! Path: `src/ai/reasoning/knowledge.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Define AI-domain knowledge representation and knowledge-operation
//! contracts for Zamani.
//!
//! This module describes knowledge computational intent. It does not
//! implement a persistent knowledge database, knowledge graph engine,
//! theorem prover, query engine, or runtime.
//!
//! ## Authority
//!
//! Source syntax:
//! - grammar/expressions/knowledge.g4
//! - grammar/ai/knowledge.g4
//! - grammar/ai/ai.g4
//!
//! Semantic authority:
//! - Canonical frontend AST.
//! - Shared frontend semantic model.
//! - Existing AI semantic domain.
//!
//! Computational representation:
//! - Canonical ZUIR AI dialect.
//!
//! Persistence and execution:
//! - Existing knowledge infrastructure.
//! - Runtime and distributed execution.
//!
//! ## Ownership
//!
//! This module owns:
//! - AI knowledge-domain value descriptions.
//! - Knowledge assertion and retraction intent.
//! - Knowledge query patterns.
//! - Evidence and provenance references.
//! - Knowledge-provider contracts.
//!
//! It does not own:
//! - Source syntax or AST.
//! - Semantic name resolution.
//! - Persistent knowledge storage.
//! - Global knowledge identity allocation.
//! - Query execution.
//! - AI inference execution.
//! - Authorization enforcement.
//! - Hardware selection.
//! - Distributed scheduling.
//! - Universal IR.
//!
//! ## Scalability
//!
//! No fixed maximum is imposed on:
//! - Knowledge statements.
//! - Knowledge relations.
//! - Evidence references.
//! - Metadata entries.
//! - Query patterns.
//! - Knowledge providers.
//!
//! Dynamic collections are bounded by available resources and operational
//! policies, not by language-level semantic ceilings.
//!
//! ## Integration
//!
//! Frontend AST adapters translate canonical syntax into these values.
//! Semantic analysis validates names, types, effects, policies, and
//! capabilities before execution.
//!
//! Existing Sankofa or other knowledge providers implement KnowledgeProvider.
//! ZUIR adapters lower validated requests into canonical AI operations.
//!
//! This module does not assume undocumented APIs from those subsystems.
//!
//! ## Important
//!
//! A successful request construction does not mean that knowledge was
//! persisted. Persistence success is reported only by the provider.

use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fmt;

/// Domain-local knowledge identity.
///
/// This is an opaque reference, not a globally unique persistent identity.
/// Canonical AI identity allocation and persistence identity mapping belong
/// to their respective repository subsystems.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AiKnowledgeId(u128);

impl AiKnowledgeId {
    /// Construct an identity from a nonzero value.
    ///
    /// The caller is responsible for obtaining the value from the
    /// canonical identity allocator or an explicitly scoped adapter.
    pub const fn new(value: u128) -> Option<Self> {
        if value == 0 {
            None
        } else {
            Some(Self(value))
        }
    }

    /// Return the underlying identity value.
    pub const fn get(self) -> u128 {
        self.0
    }
}

/// A qualified knowledge name.
///
/// Names are data, not executable expressions. Name resolution belongs
/// to canonical frontend semantic analysis.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KnowledgeName {
    namespace: Option<String>,
    name: String,
}

impl KnowledgeName {
    pub fn new(
        namespace: Option<String>,
        name: impl Into<String>,
    ) -> Result<Self, KnowledgeError> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(KnowledgeError::InvalidName);
        }

        if namespace
            .as_ref()
            .is_some_and(|value| value.trim().is_empty())
        {
            return Err(KnowledgeError::InvalidNamespace);
        }

        Ok(Self { namespace, name })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn namespace(&self) -> Option<&str> {
        self.namespace.as_deref()
    }
}

/// A knowledge value.
///
/// The representation is extensible and independent of physical storage.
/// Structured values are recursive and have no language-level fixed depth.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum KnowledgeValue {
    Null,
    Boolean(bool),
    Integer(i128),
    Text(String),
    Bytes(Vec<u8>),
    List(Vec<KnowledgeValue>),
    Map(BTreeMap<String, KnowledgeValue>),
    Symbol(KnowledgeName),
    Reference(AiKnowledgeId),
    Extension {
        type_name: KnowledgeName,
        payload: Vec<u8>,
    },
}

impl KnowledgeValue {
    pub fn text(value: impl Into<String>) -> Self {
        Self::Text(value.into())
    }

    pub fn symbol(
        namespace: Option<String>,
        name: impl Into<String>,
    ) -> Result<Self, KnowledgeError> {
        Ok(Self::Symbol(KnowledgeName::new(namespace, name)?))
    }

    pub fn map(
        entries: impl IntoIterator<Item = (String, KnowledgeValue)>,
    ) -> Self {
        Self::Map(entries.into_iter().collect())
    }
}

/// A knowledge statement.
///
/// A statement expresses a subject-predicate-object relation.
/// Its interpretation is determined by the knowledge domain and semantic
/// environment, not by this data structure.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KnowledgeStatement {
    subject: KnowledgeValue,
    predicate: KnowledgeName,
    object: KnowledgeValue,
}

impl KnowledgeStatement {
    pub fn new(
        subject: KnowledgeValue,
        predicate: KnowledgeName,
        object: KnowledgeValue,
    ) -> Result<Self, KnowledgeError> {
        if subject == KnowledgeValue::Null {
            return Err(KnowledgeError::InvalidSubject);
        }

        Ok(Self {
            subject,
            predicate,
            object,
        })
    }

    pub fn subject(&self) -> &KnowledgeValue {
        &self.subject
    }

    pub fn predicate(&self) -> &KnowledgeName {
        &self.predicate
    }

    pub fn object(&self) -> &KnowledgeValue {
        &self.object
    }
}

/// An evidence reference.
///
/// Evidence is referenced rather than copied into the knowledge subsystem.
/// The evidence owner remains responsible for integrity, authorization,
/// storage, and interpretation.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EvidenceReference {
    pub identity: AiKnowledgeId,
    pub source: KnowledgeName,
}

/// Provenance associated with a knowledge operation.
///
/// Provenance is descriptive metadata. Canonical provenance identity,
/// source locations, and provenance graph storage remain owned by the
/// semantic provenance subsystem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KnowledgeProvenance {
    pub origin: Option<KnowledgeName>,
    pub evidence: Vec<EvidenceReference>,
    pub attributes: BTreeMap<String, KnowledgeValue>,
}

impl KnowledgeProvenance {
    pub fn new() -> Self {
        Self {
            origin: None,
            evidence: Vec::new(),
            attributes: BTreeMap::new(),
        }
    }

    pub fn with_origin(mut self, origin: KnowledgeName) -> Self {
        self.origin = Some(origin);
        self
    }

    pub fn add_evidence(&mut self, evidence: EvidenceReference) {
        self.evidence.push(evidence);
    }

    pub fn insert_attribute(
        &mut self,
        name: impl Into<String>,
        value: KnowledgeValue,
    ) -> Option<KnowledgeValue> {
        self.attributes.insert(name.into(), value)
    }
}

impl Default for KnowledgeProvenance {
    fn default() -> Self {
        Self::new()
    }
}

/// Knowledge assertion intent.
///
/// Constructing this value does not assert or persist the statement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KnowledgeAssertion {
    pub statement: KnowledgeStatement,
    pub provenance: KnowledgeProvenance,
    pub expected_revision: Option<u128>,
}

impl KnowledgeAssertion {
    pub fn new(statement: KnowledgeStatement) -> Self {
        Self {
            statement,
            provenance: KnowledgeProvenance::default(),
            expected_revision: None,
        }
    }

    /// Require optimistic concurrency against a known revision.
    pub fn expecting_revision(mut self, revision: u128) -> Self {
        self.expected_revision = Some(revision);
        self
    }

    pub fn with_provenance(
        mut self,
        provenance: KnowledgeProvenance,
    ) -> Self {
        self.provenance = provenance;
        self
    }
}

/// Retraction selection.
///
/// A retraction may target a specific identity or a statement pattern.
/// The provider defines its transaction and matching semantics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RetractionTarget {
    Identity(AiKnowledgeId),
    Pattern(KnowledgePattern),
}

/// Knowledge retraction intent.
///
/// A retraction is not equivalent to deleting historical provenance.
/// The provider must preserve whatever audit and retention obligations
/// apply to the underlying knowledge system.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KnowledgeRetraction {
    pub target: RetractionTarget,
    pub expected_revision: Option<u128>,
    pub provenance: KnowledgeProvenance,
}

impl KnowledgeRetraction {
    pub fn by_identity(identity: AiKnowledgeId) -> Self {
        Self {
            target: RetractionTarget::Identity(identity),
            expected_revision: None,
            provenance: KnowledgeProvenance::default(),
        }
    }

    pub fn by_pattern(pattern: KnowledgePattern) -> Self {
        Self {
            target: RetractionTarget::Pattern(pattern),
            expected_revision: None,
            provenance: KnowledgeProvenance::default(),
        }
    }

    pub fn expecting_revision(mut self, revision: u128) -> Self {
        self.expected_revision = Some(revision);
        self
    }
}

/// A query term.
///
/// Variables are symbolic query bindings. Their scope and meaning are
/// determined by the query and provider contract.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum KnowledgePatternTerm {
    Any,
    Variable(KnowledgeName),
    Exact(KnowledgeValue),
}

/// A knowledge query pattern.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KnowledgePattern {
    pub subject: KnowledgePatternTerm,
    pub predicate: KnowledgePatternTerm,
    pub object: KnowledgePatternTerm,
}

impl KnowledgePattern {
    pub fn any() -> Self {
        Self {
            subject: KnowledgePatternTerm::Any,
            predicate: KnowledgePatternTerm::Any,
            object: KnowledgePatternTerm::Any,
        }
    }

    pub fn exact(statement: KnowledgeStatement) -> Self {
        Self {
            subject: KnowledgePatternTerm::Exact(statement.subject),
            predicate: KnowledgePatternTerm::Exact(
                KnowledgeValue::Symbol(statement.predicate),
            ),
            object: KnowledgePatternTerm::Exact(statement.object),
        }
    }
}

/// Provider-issued knowledge reference.
///
/// The revision is provider-defined and must not be interpreted as a
/// universal timestamp or globally ordered version.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KnowledgeRecord {
    pub identity: AiKnowledgeId,
    pub statement: KnowledgeStatement,
    pub revision: u128,
    pub provenance: KnowledgeProvenance,
}

/// Provider result for an assertion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssertionOutcome {
    Inserted(KnowledgeRecord),
    AlreadyPresent(KnowledgeRecord),
    Updated(KnowledgeRecord),
}

/// Provider result for a retraction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RetractionOutcome {
    Retracted {
        identity: AiKnowledgeId,
        revision: u128,
    },
    NotFound,
}

/// Provider result for a query.
///
/// This is an iterator contract rather than a mandatory in-memory collection.
/// Providers may implement streaming, paging, remote access, or lazy retrieval.
pub trait KnowledgeResultStream {
    type Error;

    fn next_record(
        &mut self,
    ) -> Result<Option<KnowledgeRecord>, Self::Error>;
}

/// Knowledge provider error.
///
/// The adapter should preserve provider-specific error details and map them
/// into the canonical repository error/diagnostic model at integration time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KnowledgeProviderError {
    NotFound,
    Conflict,
    Unauthorized,
    Unavailable,
    Unsupported,
    InvalidRequest,
    ResourceExhausted,
    IntegrityFailure,
    ProviderFailure(Cow<'static, str>),
}

impl fmt::Display for KnowledgeProviderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound => write!(f, "knowledge record not found"),
            Self::Conflict => write!(f, "knowledge revision conflict"),
            Self::Unauthorized => write!(f, "knowledge operation unauthorized"),
            Self::Unavailable => write!(f, "knowledge provider unavailable"),
            Self::Unsupported => write!(f, "knowledge operation unsupported"),
            Self::InvalidRequest => write!(f, "invalid knowledge request"),
            Self::ResourceExhausted => write!(f, "knowledge resources exhausted"),
            Self::IntegrityFailure => write!(f, "knowledge integrity failure"),
            Self::ProviderFailure(message) => {
                write!(f, "knowledge provider failure: {message}")
            }
        }
    }
}

impl std::error::Error for KnowledgeProviderError {}

/// Persistence and query boundary.
///
/// Existing Sankofa or another repository-owned knowledge implementation
/// should implement this trait through an adapter.
///
/// Implementations must document:
/// - Transaction boundaries.
/// - Idempotency behavior.
/// - Revision semantics.
/// - Query consistency.
/// - Retraction and audit behavior.
/// - Authorization enforcement.
/// - Resource-budget behavior.
///
/// This interface deliberately does not prescribe a storage engine.
pub trait KnowledgeProvider {
    type Stream<'a>: KnowledgeResultStream<Error = KnowledgeProviderError>
    where
        Self: 'a;

    fn assert(
        &mut self,
        request: KnowledgeAssertion,
    ) -> Result<AssertionOutcome, KnowledgeProviderError>;

    fn retract(
        &mut self,
        request: KnowledgeRetraction,
    ) -> Result<RetractionOutcome, KnowledgeProviderError>;

    fn query<'a>(
        &'a self,
        pattern: &'a KnowledgePattern,
    ) -> Result<Self::Stream<'a>, KnowledgeProviderError>;

    fn get(
        &self,
        identity: AiKnowledgeId,
    ) -> Result<Option<KnowledgeRecord>, KnowledgeProviderError>;
}

/// Knowledge-domain construction and validation errors.
///
/// These errors concern local value construction. Provider, semantic,
/// authorization, and execution errors remain separate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KnowledgeError {
    InvalidName,
    InvalidNamespace,
    InvalidSubject,
}

impl fmt::Display for KnowledgeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidName => write!(f, "knowledge name must not be empty"),
            Self::InvalidNamespace => {
                write!(f, "knowledge namespace must not be empty")
            }
            Self::InvalidSubject => {
                write!(f, "knowledge statement subject must not be null")
            }
        }
    }
}

impl std::error::Error for KnowledgeError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn name(value: &str) -> KnowledgeName {
        KnowledgeName::new(None, value).expect("valid name")
    }

    fn statement() -> KnowledgeStatement {
        KnowledgeStatement::new(
            KnowledgeValue::text("Zamani"),
            name("supports"),
            KnowledgeValue::text("POCO-REAF"),
        )
        .expect("valid statement")
    }

    #[test]
    fn identity_rejects_zero() {
        assert!(AiKnowledgeId::new(0).is_none());
        assert_eq!(AiKnowledgeId::new(42).unwrap().get(), 42);
    }

    #[test]
    fn knowledge_names_reject_empty_values() {
        assert!(KnowledgeName::new(None, "").is_err());
        assert!(KnowledgeName::new(Some("".to_owned()), "fact").is_err());
        assert!(KnowledgeName::new(None, "fact").is_ok());
    }

    #[test]
    fn statement_preserves_subject_predicate_and_object() {
        let value = statement();

        assert_eq!(value.subject(), &KnowledgeValue::text("Zamani"));
        assert_eq!(value.predicate(), &name("supports"));
        assert_eq!(value.object(), &KnowledgeValue::text("POCO-REAF"));
    }

    #[test]
    fn assertion_is_intent_until_provider_accepts_it() {
        let assertion = KnowledgeAssertion::new(statement());

        assert_eq!(assertion.expected_revision, None);
        assert!(assertion.provenance.evidence.is_empty());
    }

    #[test]
    fn query_pattern_can_match_any_statement() {
        let pattern = KnowledgePattern::any();

        assert_eq!(pattern.subject, KnowledgePatternTerm::Any);
        assert_eq!(pattern.predicate, KnowledgePatternTerm::Any);
        assert_eq!(pattern.object, KnowledgePatternTerm::Any);
    }

    #[test]
    fn exact_pattern_preserves_statement_values() {
        let pattern = KnowledgePattern::exact(statement());

        assert_eq!(
            pattern.subject,
            KnowledgePatternTerm::Exact(KnowledgeValue::text("Zamani"))
        );
    }

    #[test]
    fn nested_values_support_extensible_data() {
        let value = KnowledgeValue::map([
            (
                "model".to_owned(),
                KnowledgeValue::text("example"),
            ),
            (
                "confidence".to_owned(),
                KnowledgeValue::Integer(1),
            ),
        ]);

        assert!(matches!(value, KnowledgeValue::Map(_)));
    }

    #[test]
    fn retraction_can_be_revision_guarded() {
        let identity = AiKnowledgeId::new(9).expect("valid identity");

        let request = KnowledgeRetraction::by_identity(identity)
            .expecting_revision(17);

        assert_eq!(request.expected_revision, Some(17));
    }

    #[test]
    fn provenance_retains_evidence_references() {
        let evidence = EvidenceReference {
            identity: AiKnowledgeId::new(3).expect("valid identity"),
            source: name("experiment"),
        };

        let mut provenance = KnowledgeProvenance::new();
        provenance.add_evidence(evidence.clone());

        assert_eq!(provenance.evidence, vec![evidence]);
    }
}


#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Classical Computing — Data-Boundary Compatibility
//!
//! Path: src/classical/interoperability/compatibility.rs
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! STATUS
//!   Proposed contract-first implementation. Repository compilation and
//!   integration tests remain required before this file is verified.
//!
//! PURPOSE
//!   Evaluate explicit compatibility evidence between data-exchange
//!   endpoints without assuming that names, formats, or version strings
//!   imply semantic compatibility.
//!
//! OWNS
//!   - Data-boundary compatibility policy.
//!   - Compatibility evidence supplied by schema/format/version resolvers.
//!   - Deterministic compatibility decisions and reports.
//!   - The distinction between compatible, migration-required,
//!     incompatible, and indeterminate outcomes.
//!
//! DOES NOT OWN
//!   - Canonical version identities or general IR version policy.
//!   - ABI compatibility or foreign-call declarations.
//!   - Schema registries, format registries, or codec implementations.
//!   - Migration algorithms or migration execution.
//!   - Serialization, deserialization, transport, allocation, or scheduling.
//!   - Target capability discovery or physical resource placement.
//!
//! AUTHORITIES AND INTEGRATION
//!   - grammar/spec/data.md defines normative data semantics.
//!   - grammar/spec/compatibility.md defines cross-layer compatibility rules.
//!   - grammar/data/serialization.g4 owns serialization compatibility syntax.
//!   - src/classical/interoperability/data_exchange.rs owns exchange contracts.
//!   - src/classical/interoperability/abi.rs owns ABI-specific compatibility.
//!   - src/zuir/version.rs and src/zuir/dialect/versioning.rs own ZUIR and
//!     dialect version compatibility, not data-boundary compatibility.
//!   - src/classical/error.rs owns the shared classical error vocabulary.
//!
//! SCALABILITY
//!   No universal maximum is imposed on compatibility evidence, schema
//!   entries, extension identifiers, or exchange size. This module does not
//!   allocate payloads or promise unlimited physical resources.
//!
//! FREEZE RULE
//!   Policy semantics and decision meanings are public contracts. Changes
//!   require compatibility review and conformance tests.

use std::error::Error;
use std::fmt;

use crate::classical::error::ClassicalError;
use crate::classical::interoperability::data_exchange::{
    DataExchangeContract,
    DataExchangeEndpoint,
    DataExchangeIdentifier,
};

/// Result type for compatibility evaluation.
pub type CompatibilityResult<T> = Result<T, CompatibilityError>;

/// Stable categories for local compatibility-evaluation errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompatibilityErrorKind {
    /// The supplied evidence does not match the endpoints being compared.
    EvidenceMismatch,

    /// A caller supplied an invalid compatibility requirement.
    InvalidRequirement,

    /// A report is internally inconsistent.
    InvalidReport,
}

impl CompatibilityErrorKind {
    /// Returns a stable machine-readable error code.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EvidenceMismatch => "data_compatibility.evidence_mismatch",
            Self::InvalidRequirement => "data_compatibility.invalid_requirement",
            Self::InvalidReport => "data_compatibility.invalid_report",
        }
    }
}

/// An error raised while constructing or evaluating compatibility evidence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompatibilityError {
    kind: CompatibilityErrorKind,
    subject: String,
    message: String,
}

impl CompatibilityError {
    fn new(
        kind: CompatibilityErrorKind,
        subject: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            subject: subject.into(),
            message: message.into(),
        }
    }

    /// Returns the stable error category.
    #[must_use]
    pub const fn kind(&self) -> CompatibilityErrorKind {
        self.kind
    }

    /// Returns the subject associated with the failure.
    #[must_use]
    pub fn subject(&self) -> &str {
        &self.subject
    }

    /// Returns explanatory text, not a stable machine interface.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Converts this error into the shared classical error vocabulary.
    #[must_use]
    pub fn into_classical_error(self) -> ClassicalError {
        ClassicalError::validation_failed(format!(
            "{}: {} ({})",
            self.subject,
            self.message,
            self.kind.as_str()
        ))
    }
}

impl fmt::Display for CompatibilityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}: {} ({})",
            self.subject,
            self.message,
            self.kind.as_str()
        )
    }
}

impl Error for CompatibilityError {}

/// The compatibility fact established by a trusted resolver or validator.
///
/// This is evidence, not an algorithm for comparing schema structures.
/// `Compatible` must only be supplied when an authoritative schema/format
/// implementation has established the required compatibility guarantee.
/// `RequiresMigration` means interpretation requires a separately resolved
/// migration. `Unknown` never means compatible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompatibilityFact {
    /// Both sides have the same established identity and relevant semantics.
    Equivalent,

    /// An authoritative resolver has established compatibility without
    /// requiring a migration.
    Compatible,

    /// A migration is required before the destination can interpret the data.
    RequiresMigration,

    /// An authoritative resolver has established incompatibility.
    Incompatible,

    /// No authoritative compatibility conclusion is available.
    Unknown,
}

impl CompatibilityFact {
    /// Returns whether the fact establishes equivalence.
    #[must_use]
    pub const fn is_equivalent(self) -> bool {
        matches!(self, Self::Equivalent)
    }

    /// Returns whether a migration is required.
    #[must_use]
    pub const fn requires_migration(self) -> bool {
        matches!(self, Self::RequiresMigration)
    }
}

/// The dimensions independently evaluated at a data boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompatibilityDimension {
    /// Logical schema identity and meaning.
    Schema,

    /// Representation format identity and rules.
    Format,

    /// Character, byte, or other encoding contract.
    Encoding,

    /// Declared schema/format version.
    Version,
}

impl CompatibilityDimension {
    /// Returns a stable machine-readable name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Schema => "schema",
            Self::Format => "format",
            Self::Encoding => "encoding",
            Self::Version => "version",
        }
    }
}

/// Explicit compatibility evidence for a pair of endpoints.
///
/// Evidence must come from the relevant schema, format, encoding, or version
/// authority. This structure deliberately does not attempt to infer semantic
/// compatibility from lexical version ordering or similar-looking names.
///
/// Version strings are opaque identifiers in the data-exchange contract.
/// Their ordering and compatibility semantics belong to the version
/// authority for that schema or format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CompatibilityEvidence {
    schema: CompatibilityFact,
    format: CompatibilityFact,
    encoding: CompatibilityFact,
    version: CompatibilityFact,
}

impl CompatibilityEvidence {
    /// Creates evidence supplied by the appropriate compatibility resolvers.
    #[must_use]
    pub const fn new(
        schema: CompatibilityFact,
        format: CompatibilityFact,
        encoding: CompatibilityFact,
        version: CompatibilityFact,
    ) -> Self {
        Self {
            schema,
            format,
            encoding,
            version,
        }
    }

    /// Returns the schema compatibility fact.
    #[must_use]
    pub const fn schema(self) -> CompatibilityFact {
        self.schema
    }

    /// Returns the format compatibility fact.
    #[must_use]
    pub const fn format(self) -> CompatibilityFact {
        self.format
    }

    /// Returns the encoding compatibility fact.
    #[must_use]
    pub const fn encoding(self) -> CompatibilityFact {
        self.encoding
    }

    /// Returns the version compatibility fact.
    #[must_use]
    pub const fn version(self) -> CompatibilityFact {
        self.version
    }

    /// Returns the fact associated with a dimension.
    #[must_use]
    pub const fn fact(self, dimension: CompatibilityDimension) -> CompatibilityFact {
        match dimension {
            CompatibilityDimension::Schema => self.schema,
            CompatibilityDimension::Format => self.format,
            CompatibilityDimension::Encoding => self.encoding,
            CompatibilityDimension::Version => self.version,
        }
    }

    /// Creates conservative evidence from endpoint identities alone.
    ///
    /// Matching identifiers establish identity only for that identifier.
    /// They do not prove that a codec is installed, that a schema is valid,
    /// or that a target can execute the exchange. Unmatched identifiers
    /// remain unknown rather than being guessed compatible.
    ///
    /// Versions are equivalent only when both sides declare the same
    /// version. If either side omits a version, version compatibility is
    /// unknown.
    #[must_use]
    pub fn from_endpoint_identity(
        source: &DataExchangeEndpoint,
        destination: &DataExchangeEndpoint,
    ) -> Self {
        let schema = identity_fact(source.schema(), destination.schema());
        let format = identity_fact(source.format(), destination.format());
        let encoding = identity_fact(source.encoding(), destination.encoding());

        let version = match (source.version(), destination.version()) {
            (Some(source_version), Some(destination_version))
                if source_version == destination_version =>
            {
                CompatibilityFact::Equivalent
            }
            (Some(_), Some(_)) => CompatibilityFact::Unknown,
            _ => CompatibilityFact::Unknown,
        };

        Self::new(schema, format, encoding, version)
    }
}

fn identity_fact(
    source: &DataExchangeIdentifier,
    destination: &DataExchangeIdentifier,
) -> CompatibilityFact {
    if source == destination {
        CompatibilityFact::Equivalent
    } else {
        CompatibilityFact::Unknown
    }
}

/// Policy for accepting a data-boundary compatibility result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompatibilityPolicy {
    /// Accept only dimensions proven equivalent.
    ///
    /// This is the strictest policy. Unknown evidence is not accepted.
    Exact,

    /// Accept equivalent dimensions and dimensions explicitly established
    /// as compatible by authoritative resolvers.
    AllowDeclaredCompatible,

    /// Also allow a migration-required result, provided the report identifies
    /// the affected dimensions. This policy does not execute the migration.
    AllowMigration,
}

/// Final result of evaluating all required compatibility dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompatibilityStatus {
    /// All required dimensions satisfy the selected policy without migration.
    Compatible,

    /// At least one dimension requires migration, and policy permits it.
    RequiresMigration,

    /// At least one dimension is explicitly incompatible or violates policy.
    Incompatible,

    /// No dimension is known incompatible, but evidence is insufficient.
    Indeterminate,
}

impl CompatibilityStatus {
    /// Returns whether the exchange may proceed without migration.
    #[must_use]
    pub const fn is_compatible(self) -> bool {
        matches!(self, Self::Compatible)
    }

    /// Returns whether migration is required before proceeding.
    #[must_use]
    pub const fn requires_migration(self) -> bool {
        matches!(self, Self::RequiresMigration)
    }

    /// Returns whether the exchange must be rejected under the chosen policy.
    #[must_use]
    pub const fn is_incompatible(self) -> bool {
        matches!(self, Self::Incompatible)
    }

    /// Returns whether evidence is insufficient for a decision.
    #[must_use]
    pub const fn is_indeterminate(self) -> bool {
        matches!(self, Self::Indeterminate)
    }
}

/// One dimension's evaluated compatibility result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DimensionDecision {
    dimension: CompatibilityDimension,
    fact: CompatibilityFact,
}

impl DimensionDecision {
    /// Creates a dimension decision.
    #[must_use]
    pub const fn new(
        dimension: CompatibilityDimension,
        fact: CompatibilityFact,
    ) -> Self {
        Self { dimension, fact }
    }

    /// Returns the evaluated dimension.
    #[must_use]
    pub const fn dimension(self) -> CompatibilityDimension {
        self.dimension
    }

    /// Returns the evidence used for the dimension.
    #[must_use]
    pub const fn fact(self) -> CompatibilityFact {
        self.fact
    }
}

/// Deterministic report for a data-boundary compatibility check.
///
/// The report contains a fixed set of semantic dimensions. It intentionally
/// contains no payloads, runtime handles, physical layouts, or provider state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompatibilityReport {
    status: CompatibilityStatus,
    policy: CompatibilityPolicy,
    schema: DimensionDecision,
    format: DimensionDecision,
    encoding: DimensionDecision,
    version: DimensionDecision,
}

impl CompatibilityReport {
    /// Returns the final status.
    #[must_use]
    pub const fn status(&self) -> CompatibilityStatus {
        self.status
    }

    /// Returns the policy used for this decision.
    #[must_use]
    pub const fn policy(&self) -> CompatibilityPolicy {
        self.policy
    }

    /// Returns the schema decision.
    #[must_use]
    pub const fn schema(&self) -> DimensionDecision {
        self.schema
    }

    /// Returns the format decision.
    #[must_use]
    pub const fn format(&self) -> DimensionDecision {
        self.format
    }

    /// Returns the encoding decision.
    #[must_use]
    pub const fn encoding(&self) -> DimensionDecision {
        self.encoding
    }

    /// Returns the version decision.
    #[must_use]
    pub const fn version(&self) -> DimensionDecision {
        self.version
    }

    /// Returns decisions in stable semantic order.
    #[must_use]
    pub const fn decisions(&self) -> [DimensionDecision; 4] {
        [self.schema, self.format, self.encoding, self.version]
    }

    /// Returns true only when every required dimension is proven equivalent.
    #[must_use]
    pub const fn is_exactly_equivalent(&self) -> bool {
        self.schema.fact.is_equivalent()
            && self.format.fact.is_equivalent()
            && self.encoding.fact.is_equivalent()
            && self.version.fact.is_equivalent()
    }

    /// Returns dimensions that explicitly require migration.
    ///
    /// The returned array has a stable order. The count is represented by the
    /// returned slice length; no global migration-count ceiling is defined.
    #[must_use]
    pub fn migration_dimensions(&self) -> Vec<CompatibilityDimension> {
        self.decisions()
            .into_iter()
            .filter(|decision| decision.fact.requires_migration())
            .map(|decision| decision.dimension)
            .collect()
    }

    /// Converts a non-compatible status into the shared classical error.
    ///
    /// `Compatible` returns `Ok(())`. Migration-required and indeterminate
    /// statuses are not silently treated as compatible.
    pub fn ensure_compatible(&self) -> Result<(), ClassicalError> {
        match self.status {
            CompatibilityStatus::Compatible => Ok(()),
            CompatibilityStatus::RequiresMigration => {
                Err(ClassicalError::validation_failed(
                    "data exchange requires a separately resolved migration",
                ))
            }
            CompatibilityStatus::Incompatible => {
                Err(ClassicalError::validation_failed(
                    "data exchange is incompatible under the selected policy",
                ))
            }
            CompatibilityStatus::Indeterminate => {
                Err(ClassicalError::validation_failed(
                    "data exchange compatibility is indeterminate; authoritative evidence is required",
                ))
            }
        }
    }
}

/// Stateless evaluator for data-boundary compatibility.
///
/// This evaluator does not discover schemas, query providers, compare
/// arbitrary version strings, or execute migrations. Those responsibilities
/// remain with their owning registries and implementations.
#[derive(Debug, Clone, Copy, Default)]
pub struct CompatibilityChecker;

impl CompatibilityChecker {
    /// Evaluates explicit evidence against the selected policy.
    #[must_use]
    pub fn assess(
        evidence: CompatibilityEvidence,
        policy: CompatibilityPolicy,
    ) -> CompatibilityReport {
        let schema = DimensionDecision::new(
            CompatibilityDimension::Schema,
            evidence.schema(),
        );
        let format = DimensionDecision::new(
            CompatibilityDimension::Format,
            evidence.format(),
        );
        let encoding = DimensionDecision::new(
            CompatibilityDimension::Encoding,
            evidence.encoding(),
        );
        let version = DimensionDecision::new(
            CompatibilityDimension::Version,
            evidence.version(),
        );

        let facts = [schema.fact, format.fact, encoding.fact, version.fact];
        let status = evaluate_facts(&facts, policy);

        CompatibilityReport {
            status,
            policy,
            schema,
            format,
            encoding,
            version,
        }
    }

    /// Assesses two exchange endpoints using conservative identity-only
    /// evidence. Unmatched identities and missing versions remain unknown.
    #[must_use]
    pub fn assess_endpoints(
        source: &DataExchangeEndpoint,
        destination: &DataExchangeEndpoint,
        policy: CompatibilityPolicy,
    ) -> CompatibilityReport {
        let evidence =
            CompatibilityEvidence::from_endpoint_identity(source, destination);
        Self::assess(evidence, policy)
    }

    /// Validates an exchange contract and assesses its endpoints.
    ///
    /// Contract validation does not prove codec availability, schema
    /// validity, payload validity, or target feasibility. The result is
    /// therefore a data-boundary identity assessment only.
    pub fn assess_contract(
        contract: &DataExchangeContract,
        policy: CompatibilityPolicy,
    ) -> CompatibilityResult<CompatibilityReport> {
        contract.validate().map_err(|error| {
            CompatibilityError::new(
                CompatibilityErrorKind::InvalidRequirement,
                "data_exchange_contract",
                error.to_string(),
            )
        })?;

        Ok(Self::assess_endpoints(
            contract.source(),
            contract.destination(),
            policy,
        ))
    }
}

fn evaluate_facts(
    facts: &[CompatibilityFact],
    policy: CompatibilityPolicy,
) -> CompatibilityStatus {
    // Explicit incompatibility always wins. A policy must never turn known
    // incompatibility into acceptance.
    if facts
        .iter()
        .any(|fact| *fact == CompatibilityFact::Incompatible)
    {
        return CompatibilityStatus::Incompatible;
    }

    // Unknown evidence is not equivalent to compatibility. Even if another
    // dimension requires migration, missing evidence prevents a reliable
    // final decision.
    if facts.iter().any(|fact| *fact == CompatibilityFact::Unknown) {
        return CompatibilityStatus::Indeterminate;
    }

    let has_migration = facts
        .iter()
        .any(|fact| *fact == CompatibilityFact::RequiresMigration);

    let has_declared_compatibility = facts
        .iter()
        .any(|fact| *fact == CompatibilityFact::Compatible);

    match policy {
        CompatibilityPolicy::Exact => {
            if has_migration || has_declared_compatibility {
                CompatibilityStatus::Incompatible
            } else {
                CompatibilityStatus::Compatible
            }
        }
        CompatibilityPolicy::AllowDeclaredCompatible => {
            if has_migration {
                CompatibilityStatus::Incompatible
            } else {
                CompatibilityStatus::Compatible
            }
        }
        CompatibilityPolicy::AllowMigration => {
            if has_migration {
                CompatibilityStatus::RequiresMigration
            } else {
                CompatibilityStatus::Compatible
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn endpoint(
        schema: &str,
        format: &str,
        encoding: &str,
        version: Option<&str>,
    ) -> DataExchangeEndpoint {
        DataExchangeEndpoint::new(
            schema,
            format,
            encoding,
            version.map(str::to_owned),
        )
        .expect("test endpoint should be valid")
    }

    fn all(fact: CompatibilityFact) -> CompatibilityEvidence {
        CompatibilityEvidence::new(fact, fact, fact, fact)
    }

    #[test]
    fn exact_evidence_is_compatible_under_exact_policy() {
        let report = CompatibilityChecker::assess(
            all(CompatibilityFact::Equivalent),
            CompatibilityPolicy::Exact,
        );

        assert_eq!(report.status(), CompatibilityStatus::Compatible);
        assert!(report.is_exactly_equivalent());
    }

    #[test]
    fn declared_compatibility_is_not_exact_equivalence() {
        let report = CompatibilityChecker::assess(
            all(CompatibilityFact::Compatible),
            CompatibilityPolicy::AllowDeclaredCompatible,
        );

        assert_eq!(report.status(), CompatibilityStatus::Compatible);
        assert!(!report.is_exactly_equivalent());
    }

    #[test]
    fn exact_policy_rejects_merely_declared_compatibility() {
        let report = CompatibilityChecker::assess(
            all(CompatibilityFact::Compatible),
            CompatibilityPolicy::Exact,
        );

        assert_eq!(report.status(), CompatibilityStatus::Incompatible);
    }

    #[test]
    fn migration_is_explicit_and_policy_controlled() {
        let evidence = CompatibilityEvidence::new(
            CompatibilityFact::Equivalent,
            CompatibilityFact::Equivalent,
            CompatibilityFact::Equivalent,
            CompatibilityFact::RequiresMigration,
        );

        let rejected = CompatibilityChecker::assess(
            evidence,
            CompatibilityPolicy::AllowDeclaredCompatible,
        );
        let permitted = CompatibilityChecker::assess(
            evidence,
            CompatibilityPolicy::AllowMigration,
        );

        assert_eq!(rejected.status(), CompatibilityStatus::Incompatible);
        assert_eq!(
            permitted.status(),
            CompatibilityStatus::RequiresMigration
        );
        assert_eq!(
            permitted.migration_dimensions(),
            vec![CompatibilityDimension::Version]
        );
    }

    #[test]
    fn unknown_evidence_is_never_silently_accepted() {
        let report = CompatibilityChecker::assess(
            all(CompatibilityFact::Unknown),
            CompatibilityPolicy::AllowMigration,
        );

        assert_eq!(report.status(), CompatibilityStatus::Indeterminate);
        assert!(report.ensure_compatible().is_err());
    }

    #[test]
    fn explicit_incompatibility_takes_precedence() {
        let evidence = CompatibilityEvidence::new(
            CompatibilityFact::Unknown,
            CompatibilityFact::Incompatible,
            CompatibilityFact::Equivalent,
            CompatibilityFact::Equivalent,
        );

        let report = CompatibilityChecker::assess(
            evidence,
            CompatibilityPolicy::AllowMigration,
        );

        assert_eq!(report.status(), CompatibilityStatus::Incompatible);
    }

    #[test]
    fn identical_endpoint_identities_do_not_infer_missing_versions() {
        let source = endpoint(
            "schema.customer",
            "format.example",
            "encoding.example",
            None,
        );
        let destination = endpoint(
            "schema.customer",
            "format.example",
            "encoding.example",
            None,
        );

        let report = CompatibilityChecker::assess_endpoints(
            &source,
            &destination,
            CompatibilityPolicy::Exact,
        );

        assert_eq!(report.schema().fact(), CompatibilityFact::Equivalent);
        assert_eq!(report.format().fact(), CompatibilityFact::Equivalent);
        assert_eq!(report.encoding().fact(), CompatibilityFact::Equivalent);
        assert_eq!(report.version().fact(), CompatibilityFact::Unknown);
        assert_eq!(report.status(), CompatibilityStatus::Indeterminate);
    }

    #[test]
    fn distinct_versions_are_not_compared_lexically() {
        let source = endpoint(
            "schema.customer",
            "format.example",
            "encoding.example",
            Some("2"),
        );
        let destination = endpoint(
            "schema.customer",
            "format.example",
            "encoding.example",
            Some("10"),
        );

        let evidence =
            CompatibilityEvidence::from_endpoint_identity(&source, &destination);

        assert_eq!(evidence.version(), CompatibilityFact::Unknown);
    }

    #[test]
    fn same_declared_version_is_identity_evidence_not_target_feasibility() {
        let source = endpoint(
            "schema.customer",
            "format.example",
            "encoding.example",
            Some("v1"),
        );
        let destination = endpoint(
            "schema.customer",
            "format.example",
            "encoding.example",
            Some("v1"),
        );

        let report = CompatibilityChecker::assess_endpoints(
            &source,
            &destination,
            CompatibilityPolicy::Exact,
        );

        assert_eq!(report.status(), CompatibilityStatus::Compatible);
    }

    #[test]
    fn decisions_have_stable_order() {
        let report = CompatibilityChecker::assess(
            all(CompatibilityFact::Equivalent),
            CompatibilityPolicy::Exact,
        );

        let dimensions = report
            .decisions()
            .into_iter()
            .map(DimensionDecision::dimension)
            .collect::<Vec<_>>();

        assert_eq!(
            dimensions,
            vec![
                CompatibilityDimension::Schema,
                CompatibilityDimension::Format,
                CompatibilityDimension::Encoding,
                CompatibilityDimension::Version,
            ]
        );
    }

    #[test]
    fn conversion_to_classical_error_preserves_stable_category() {
        let error = CompatibilityError::new(
            CompatibilityErrorKind::EvidenceMismatch,
            "schema",
            "test mismatch",
        )
        .into_classical_error();

        assert_eq!(error.code(), "classical.validation_failed");
    }
}

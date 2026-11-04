
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani AI — Data Integration
//!
//! Path: `src/ai/data/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Define the AI-domain boundary for consuming canonical Zamani data
//! resources.
//!
//! This module allows AI models, learning operations, training workflows,
//! inference pipelines, reasoning systems and other AI computations to
//! declare their data dependencies without introducing a competing
//! data-storage or data-processing architecture.
//!
//! ## Architectural authority
//!
//! The following existing repository components remain authoritative:
//!
//! - `grammar/data/data.g4`:
//!   Canonical data-domain grammar composition.
//!
//! - `grammar/data/knowledge.g4`:
//!   Data-domain knowledge adapter.
//!
//! - `grammar/expressions/knowledge.g4`:
//!   Canonical knowledge expression syntax.
//!
//! - `grammar/expressions/provenance.g4`:
//!   Canonical expression-level provenance syntax.
//!
//! - `grammar/ai/learning.g4`:
//!   AI learning composition and learning intent.
//!
//! - `src/frontend/semantic/domains/ai.rs`:
//!   Existing AI semantic value and operation contracts.
//!
//! - `src/classical/collections/tensor.rs`:
//!   Existing classical tensor abstraction.
//!
//! The canonical data subsystem remains responsible for the actual
//! interpretation and realization of data resources.
//!
//! ## Ownership
//!
//! THIS MODULE OWNS:
//!
//! - AI-specific data consumption roles;
//! - AI-specific data access intentions;
//! - AI-to-data resource reference contracts;
//! - AI data binding descriptions;
//! - AI data dependency collections;
//! - AI data integration validation interfaces;
//! - AI data validation reports.
//!
//! THIS MODULE DOES NOT OWN:
//!
//! - Dataset storage;
//! - Dataset identity allocation;
//! - Dataset implementations;
//! - Data schemas;
//! - Schema registries;
//! - Query languages;
//! - Query execution;
//! - Database connections;
//! - Data serialization;
//! - Data deserialization;
//! - Data preprocessing implementations;
//! - Data batching implementations;
//! - Data streaming implementations;
//! - Data partitioning;
//! - Data replication;
//! - Data distribution;
//! - Tensor storage;
//! - Tensor numerical operations;
//! - Knowledge storage;
//! - Knowledge graphs;
//! - Provenance record definitions;
//! - Provenance storage;
//! - Authorization enforcement;
//! - Resource allocation;
//! - Hardware discovery;
//! - Physical placement;
//! - Scheduling;
//! - Runtime execution;
//! - Compiler IR.
//!
//! These responsibilities belong to their existing or designated
//! repository authorities.
//!
//! ## POCO-REAF
//!
//! This module introduces no universal maximum for:
//!
//! - datasets;
//! - records;
//! - fields;
//! - features;
//! - labels;
//! - model inputs;
//! - model outputs;
//! - data bindings;
//! - data sources;
//! - streams;
//! - partitions;
//! - workers;
//! - nodes;
//! - devices;
//! - memory;
//! - storage;
//! - tensor dimensions;
//! - tensor rank.
//!
//! Collections grow according to their actual representation and available
//! resources.
//!
//! No finite implementation can process literally infinite data.
//! The objective is to impose no artificial language-level capacity
//! ceiling.
//!
//! Resource budgets, memory limits, execution deadlines and security
//! restrictions remain configurable downstream concerns.
//!
//! ## Dependency direction
//!
//! ```text
//! Canonical data subsystem
//!          |
//!          v
//! Canonical data references
//!          |
//!          v
//! AI data bindings
//!          |
//!          v
//! AI data validation contracts
//!          |
//!          v
//! AI learning / model / pipeline
//!          |
//!          v
//! Canonical semantic model
//!          |
//!          v
//! Canonical IR
//!          |
//!          v
//! Compiler / runtime / distributed execution
//! ```
//!
//! This module does not depend on a particular storage provider,
//! database, execution target or hardware architecture.
//!
//! ## Integration contract
//!
//! The canonical data implementation supplies an implementation of
//! `AiDataServices`.
//!
//! Its associated reference types must be the actual canonical data,
//! schema and provenance reference types.
//!
//! The AI subsystem must not manufacture replacement identities.
//!
//! Data-access authorization, provenance obligations, schema validation
//! and policy evaluation must delegate to their respective authorities.
//!
//! An unavailable or inconclusive data service must never be interpreted
//! as successful validation.
//!
//! ## Freeze criteria
//!
//! This module can be frozen when:
//!
//! 1. The canonical data reference types are connected.
//! 2. The canonical data services implement `AiDataServices`.
//! 3. AI semantic validation consumes these contracts.
//! 4. Data-access and provenance obligations are delegated correctly.
//! 5. Incomplete validation cannot certify success.
//! 6. All unit and integration tests pass.
//! 7. No duplicate data-storage or processing architecture is introduced.

// =============================================================================
// Imports
// =============================================================================

use core::fmt;

// =============================================================================
// AI data roles
// =============================================================================

/// Describes the semantic purpose of a data resource within AI computation.
///
/// Roles describe how AI consumes a resource, not how that resource is
/// physically represented or stored.
///
/// Custom roles allow future learning and AI paradigms without changing
/// this module.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AiDataRole(String);

impl AiDataRole {
    /// Creates an extensible AI data role.
    ///
    /// Empty names and control characters are rejected.
    pub fn new(value: impl Into<String>) -> Result<Self, AiDataInputError> {
        let value = value.into();

        if value.is_empty() {
            return Err(AiDataInputError::EmptyRole);
        }

        if value.chars().any(char::is_control) {
            return Err(AiDataInputError::InvalidRole);
        }

        Ok(Self(value))
    }

    /// Returns the canonical role name.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Model input or feature data.
    pub fn feature() -> Self {
        Self("feature".to_owned())
    }

    /// Expected learning targets.
    pub fn target() -> Self {
        Self("target".to_owned())
    }

    /// Training labels.
    pub fn label() -> Self {
        Self("label".to_owned())
    }

    /// Contextual information.
    pub fn context() -> Self {
        Self("context".to_owned())
    }

    /// Evaluation data.
    pub fn evaluation() -> Self {
        Self("evaluation".to_owned())
    }

    /// Feedback consumed by an adaptive or learning system.
    pub fn feedback() -> Self {
        Self("feedback".to_owned())
    }

    /// Evidence consumed by an AI reasoning or learning operation.
    pub fn evidence() -> Self {
        Self("evidence".to_owned())
    }
}

// =============================================================================
// Access intentions
// =============================================================================

/// Describes the logical access intention for an AI data dependency.
///
/// This is not a storage or streaming implementation.
///
/// The canonical data subsystem determines how the requested access
/// semantics can actually be realized.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum AiDataAccessIntent {
    /// Ordinary read access.
    Read,

    /// A logically consistent snapshot.
    Snapshot,

    /// Incremental consumption of changing data.
    Incremental,

    /// Streaming consumption.
    Streaming,

    /// An extensible future access intention.
    Custom(String),
}

// =============================================================================
// Input construction errors
// =============================================================================

/// Errors caused by malformed AI data binding inputs.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AiDataInputError {
    EmptyRole,
    InvalidRole,
}

impl fmt::Display for AiDataInputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyRole => {
                f.write_str("AI data role cannot be empty")
            }
            Self::InvalidRole => {
                f.write_str(
                    "AI data role cannot contain control characters",
                )
            }
        }
    }
}

impl std::error::Error for AiDataInputError {}

// =============================================================================
// AI data binding
// =============================================================================

/// A reference-based binding between AI computation and canonical data.
///
/// Generic parameters are intentional:
///
/// - `Source`: canonical data resource reference;
/// - `Schema`: canonical schema reference;
/// - `Provenance`: canonical provenance reference.
///
/// These types must be supplied by the existing data and semantic
/// infrastructure.
///
/// This module does not define competing dataset or schema identities.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AiDataBinding<Source, Schema, Provenance> {
    source: Source,
    role: AiDataRole,
    access: AiDataAccessIntent,
    expected_schema: Option<Schema>,
    provenance: Option<Provenance>,
}

impl<Source, Schema, Provenance>
    AiDataBinding<Source, Schema, Provenance>
{
    /// Creates an AI data binding.
    pub fn new(
        source: Source,
        role: AiDataRole,
        access: AiDataAccessIntent,
    ) -> Self {
        Self {
            source,
            role,
            access,
            expected_schema: None,
            provenance: None,
        }
    }

    /// Associates an expected canonical schema.
    ///
    /// This stores a reference only.
    /// Schema interpretation and compatibility checking belong to
    /// the canonical schema subsystem.
    pub fn with_expected_schema(
        mut self,
        schema: Schema,
    ) -> Self {
        self.expected_schema = Some(schema);
        self
    }

    /// Associates a canonical provenance reference.
    ///
    /// This does not create or store provenance records.
    pub fn with_provenance(
        mut self,
        provenance: Provenance,
    ) -> Self {
        self.provenance = Some(provenance);
        self
    }

    /// Returns the referenced data resource.
    pub fn source(&self) -> &Source {
        &self.source
    }

    /// Returns the semantic data role.
    pub fn role(&self) -> &AiDataRole {
        &self.role
    }

    /// Returns the access intention.
    pub fn access(&self) -> &AiDataAccessIntent {
        &self.access
    }

    /// Returns the optional expected schema reference.
    pub fn expected_schema(&self) -> Option<&Schema> {
        self.expected_schema.as_ref()
    }

    /// Returns the optional provenance reference.
    pub fn provenance(&self) -> Option<&Provenance> {
        self.provenance.as_ref()
    }
}

// =============================================================================
// AI data dependency plan
// =============================================================================

/// A collection of AI data dependencies.
///
/// The plan contains references and declarations only.
///
/// It does not own datasets, allocate memory, open files, execute queries,
/// start streams or perform preprocessing.
///
/// The number of bindings is not artificially restricted.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AiDataPlan<Source, Schema, Provenance> {
    bindings: Vec<AiDataBinding<Source, Schema, Provenance>>,
}

impl<Source, Schema, Provenance>
    Default for AiDataPlan<Source, Schema, Provenance>
{
    fn default() -> Self {
        Self {
            bindings: Vec::new(),
        }
    }
}

impl<Source, Schema, Provenance>
    AiDataPlan<Source, Schema, Provenance>
{
    /// Creates an empty AI data dependency plan.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a data dependency.
    ///
    /// This operation does not inspect or access the underlying resource.
    pub fn add(
        &mut self,
        binding: AiDataBinding<Source, Schema, Provenance>,
    ) {
        self.bindings.push(binding);
    }

    /// Returns the number of declared dependencies.
    pub fn len(&self) -> usize {
        self.bindings.len()
    }

    /// Returns whether the plan has no dependencies.
    pub fn is_empty(&self) -> bool {
        self.bindings.is_empty()
    }

    /// Borrows the dependency collection.
    pub fn bindings(
        &self,
    ) -> &[AiDataBinding<Source, Schema, Provenance>] {
        &self.bindings
    }

    /// Consumes the plan and returns its dependencies.
    pub fn into_bindings(
        self,
    ) -> Vec<AiDataBinding<Source, Schema, Provenance>> {
        self.bindings
    }
}

// =============================================================================
// Validation contracts
// =============================================================================

/// Result of checking an AI data dependency against canonical services.
///
/// An inconclusive result is deliberately different from a successful
/// validation.
///
/// Infrastructure failures, unresolved references and unavailable
/// validation services must not be represented as Satisfied.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AiDataCheck {
    /// The requested condition was established.
    Satisfied,

    /// The condition was checked and found invalid.
    Rejected {
        code: String,
        message: String,
    },

    /// The condition could not be established.
    Inconclusive {
        code: String,
        message: String,
    },
}

impl AiDataCheck {
    pub fn satisfied() -> Self {
        Self::Satisfied
    }

    pub fn rejected(
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::Rejected {
            code: code.into(),
            message: message.into(),
        }
    }

    pub fn inconclusive(
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::Inconclusive {
            code: code.into(),
            message: message.into(),
        }
    }
}

/// Canonical service integration contract for AI data dependencies.
///
/// Implementations must delegate to the actual repository authorities.
///
/// In particular:
///
/// - source resolution delegates to canonical data services;
/// - schema checking delegates to canonical schema/type services;
/// - access checking delegates to canonical effects and authorization;
/// - provenance checking delegates to canonical provenance services;
/// - policy checking delegates to canonical policy services.
///
/// No database, storage or stream implementation belongs here.
///
/// All methods must be deterministic for the same resource references,
/// semantic environment, configuration and policy snapshot.
///
/// A provider that cannot complete a check must return Inconclusive.
pub trait AiDataServices {
    type Source;
    type Schema;
    type Provenance;

    /// Validates that the referenced canonical data source exists and
    /// is resolvable in the current semantic environment.
    fn check_source(
        &self,
        source: &Self::Source,
    ) -> AiDataCheck;

    /// Validates the compatibility of an expected schema with the
    /// referenced data resource.
    fn check_schema(
        &self,
        source: &Self::Source,
        expected: &Self::Schema,
    ) -> AiDataCheck;

    /// Validates the requested access intention.
    ///
    /// Authorization and effect enforcement remain owned by their
    /// canonical subsystems.
    fn check_access(
        &self,
        source: &Self::Source,
        access: &AiDataAccessIntent,
    ) -> AiDataCheck;

    /// Validates a supplied provenance reference.
    fn check_provenance(
        &self,
        source: &Self::Source,
        provenance: &Self::Provenance,
    ) -> AiDataCheck;

    /// Validates the complete binding against applicable data policies.
    ///
    /// This method must also detect obligations that cannot be expressed
    /// by the optional fields of the binding.
    ///
    /// For example, a policy may require provenance even when the caller
    /// has not supplied an explicit provenance reference.
    fn check_policy(
        &self,
        binding: &AiDataBinding<
            Self::Source,
            Self::Schema,
            Self::Provenance,
        >,
    ) -> AiDataCheck;
}

// =============================================================================
// Validation diagnostics
// =============================================================================

/// The validation stage that produced an issue.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum AiDataValidationStage {
    Source,
    Schema,
    Access,
    Provenance,
    Policy,
}

/// Classification of an AI data validation issue.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum AiDataIssueKind {
    Rejected,
    Inconclusive,
}

/// A validation issue associated with a particular dependency.
///
/// `binding_index` refers to the original declaration order in the plan.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AiDataValidationIssue {
    pub binding_index: usize,
    pub stage: AiDataValidationStage,
    pub kind: AiDataIssueKind,
    pub code: String,
    pub message: String,
}

/// Result of AI data dependency validation.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AiDataValidationReport {
    issues: Vec<AiDataValidationIssue>,
    complete: bool,
}

impl AiDataValidationReport {
    /// Creates a successful, complete initial report.
    pub fn new() -> Self {
        Self {
            issues: Vec::new(),
            complete: true,
        }
    }

    /// Returns all validation issues in deterministic declaration order.
    pub fn issues(&self) -> &[AiDataValidationIssue] {
        &self.issues
    }

    /// Returns whether all required validation completed.
    pub fn is_complete(&self) -> bool {
        self.complete
    }

    /// Returns whether the data plan has been completely validated
    /// without any reported issues.
    ///
    /// An incomplete report is never considered valid.
    pub fn is_valid(&self) -> bool {
        self.complete && self.issues.is_empty()
    }

    /// Returns whether any dependency was definitively rejected.
    pub fn has_errors(&self) -> bool {
        self.issues.iter().any(|issue| {
            issue.kind == AiDataIssueKind::Rejected
        })
    }

    /// Returns whether any validation remains inconclusive.
    pub fn has_inconclusive_results(&self) -> bool {
        self.issues.iter().any(|issue| {
            issue.kind == AiDataIssueKind::Inconclusive
        })
    }

    fn record(
        &mut self,
        binding_index: usize,
        stage: AiDataValidationStage,
        check: AiDataCheck,
    ) {
        match check {
            AiDataCheck::Satisfied => {}

            AiDataCheck::Rejected { code, message } => {
                self.issues.push(AiDataValidationIssue {
                    binding_index,
                    stage,
                    kind: AiDataIssueKind::Rejected,
                    code,
                    message,
                });
            }

            AiDataCheck::Inconclusive { code, message } => {
                self.complete = false;

                self.issues.push(AiDataValidationIssue {
                    binding_index,
                    stage,
                    kind: AiDataIssueKind::Inconclusive,
                    code,
                    message,
                });
            }
        }
    }
}

// =============================================================================
// Validation orchestration
// =============================================================================

impl<Source, Schema, Provenance>
    AiDataPlan<Source, Schema, Provenance>
{
    /// Validates every declared AI data dependency.
    ///
    /// This method:
    ///
    /// 1. Checks the canonical source reference.
    /// 2. Checks an optional expected schema.
    /// 3. Checks the requested access intention.
    /// 4. Checks optional provenance.
    /// 5. Checks applicable data policies.
    ///
    /// All checks are delegated to the canonical data and semantic
    /// infrastructure through `AiDataServices`.
    ///
    /// This method does not load or materialize data.
    ///
    /// It does not guarantee physical execution feasibility.
    ///
    /// The returned report preserves declaration order.
    pub fn validate<Services>(
        &self,
        services: &Services,
    ) -> AiDataValidationReport
    where
        Services: AiDataServices<
            Source = Source,
            Schema = Schema,
            Provenance = Provenance,
        >,
    {
        let mut report = AiDataValidationReport::new();

        for (index, binding) in self.bindings.iter().enumerate() {
            report.record(
                index,
                AiDataValidationStage::Source,
                services.check_source(binding.source()),
            );

            if let Some(schema) = binding.expected_schema() {
                report.record(
                    index,
                    AiDataValidationStage::Schema,
                    services.check_schema(
                        binding.source(),
                        schema,
                    ),
                );
            }

            report.record(
                index,
                AiDataValidationStage::Access,
                services.check_access(
                    binding.source(),
                    binding.access(),
                ),
            );

            if let Some(provenance) = binding.provenance() {
                report.record(
                    index,
                    AiDataValidationStage::Provenance,
                    services.check_provenance(
                        binding.source(),
                        provenance,
                    ),
                );
            }

            report.record(
                index,
                AiDataValidationStage::Policy,
                services.check_policy(binding),
            );
        }

        report
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    struct TestDataServices;

    impl AiDataServices for TestDataServices {
        type Source = String;
        type Schema = String;
        type Provenance = String;

        fn check_source(
            &self,
            source: &Self::Source,
        ) -> AiDataCheck {
            if source.is_empty() {
                AiDataCheck::rejected(
                    "DATA001",
                    "empty source reference",
                )
            } else {
                AiDataCheck::satisfied()
            }
        }

        fn check_schema(
            &self,
            _: &Self::Source,
            expected: &Self::Schema,
        ) -> AiDataCheck {
            if expected.is_empty() {
                AiDataCheck::rejected(
                    "DATA002",
                    "empty schema reference",
                )
            } else {
                AiDataCheck::satisfied()
            }
        }

        fn check_access(
            &self,
            _: &Self::Source,
            _: &AiDataAccessIntent,
        ) -> AiDataCheck {
            AiDataCheck::satisfied()
        }

        fn check_provenance(
            &self,
            _: &Self::Source,
            provenance: &Self::Provenance,
        ) -> AiDataCheck {
            if provenance.is_empty() {
                AiDataCheck::rejected(
                    "DATA003",
                    "empty provenance reference",
                )
            } else {
                AiDataCheck::satisfied()
            }
        }

        fn check_policy(
            &self,
            _: &AiDataBinding<
                Self::Source,
                Self::Schema,
                Self::Provenance,
            >,
        ) -> AiDataCheck {
            AiDataCheck::satisfied()
        }
    }

    #[test]
    fn role_rejects_empty_values() {
        assert_eq!(
            AiDataRole::new(""),
            Err(AiDataInputError::EmptyRole),
        );
    }

    #[test]
    fn role_rejects_control_characters() {
        assert_eq!(
            AiDataRole::new("feature\n"),
            Err(AiDataInputError::InvalidRole),
        );
    }

    #[test]
    fn custom_roles_are_supported() {
        let role = AiDataRole::new(
            "scientific.observation",
        )
        .expect("valid custom role");

        assert_eq!(
            role.as_str(),
            "scientific.observation",
        );
    }

    #[test]
    fn built_in_roles_are_available() {
        assert_eq!(AiDataRole::feature().as_str(), "feature");
        assert_eq!(AiDataRole::target().as_str(), "target");
        assert_eq!(AiDataRole::label().as_str(), "label");
        assert_eq!(AiDataRole::evidence().as_str(), "evidence");
    }

    #[test]
    fn empty_plan_is_valid() {
        let plan = AiDataPlan::<String, String, String>::new();

        let report = plan.validate(&TestDataServices);

        assert!(report.is_complete());
        assert!(report.is_valid());
        assert!(report.issues().is_empty());
    }

    #[test]
    fn binding_preserves_canonical_references() {
        let binding = AiDataBinding::new(
            "dataset.reference".to_owned(),
            AiDataRole::feature(),
            AiDataAccessIntent::Read,
        )
        .with_expected_schema("schema.reference".to_owned())
        .with_provenance("provenance.reference".to_owned());

        assert_eq!(
            binding.source(),
            "dataset.reference",
        );

        assert_eq!(
            binding.expected_schema().map(String::as_str),
            Some("schema.reference"),
        );

        assert_eq!(
            binding.provenance().map(String::as_str),
            Some("provenance.reference"),
        );
    }

    #[test]
    fn valid_binding_passes_validation() {
        let binding = AiDataBinding::new(
            "dataset.reference".to_owned(),
            AiDataRole::feature(),
            AiDataAccessIntent::Snapshot,
        );

        let mut plan = AiDataPlan::new();
        plan.add(binding);

        let report = plan.validate(&TestDataServices);

        assert!(report.is_complete());
        assert!(report.is_valid());
    }

    #[test]
    fn invalid_source_is_rejected() {
        let binding = AiDataBinding::new(
            String::new(),
            AiDataRole::feature(),
            AiDataAccessIntent::Read,
        );

        let mut plan = AiDataPlan::new();
        plan.add(binding);

        let report = plan.validate(&TestDataServices);

        assert!(report.has_errors());
        assert!(!report.is_valid());

        assert_eq!(
            report.issues()[0].code,
            "DATA001",
        );
    }

    #[test]
    fn invalid_schema_is_rejected() {
        let binding = AiDataBinding::new(
            "dataset.reference".to_owned(),
            AiDataRole::feature(),
            AiDataAccessIntent::Read,
        )
        .with_expected_schema(String::new());

        let mut plan = AiDataPlan::new();
        plan.add(binding);

        let report = plan.validate(&TestDataServices);

        assert!(report.has_errors());
        assert_eq!(
            report.issues()[0].stage,
            AiDataValidationStage::Schema,
        );
    }

    #[test]
    fn incomplete_validation_never_becomes_valid() {
        struct IncompleteServices;

        impl AiDataServices for IncompleteServices {
            type Source = String;
            type Schema = String;
            type Provenance = String;

            fn check_source(
                &self,
                _: &Self::Source,
            ) -> AiDataCheck {
                AiDataCheck::inconclusive(
                    "DATA004",
                    "source resolution is unavailable",
                )
            }

            fn check_schema(
                &self,
                _: &Self::Source,
                _: &Self::Schema,
            ) -> AiDataCheck {
                AiDataCheck::satisfied()
            }

            fn check_access(
                &self,
                _: &Self::Source,
                _: &AiDataAccessIntent,
            ) -> AiDataCheck {
                AiDataCheck::satisfied()
            }

            fn check_provenance(
                &self,
                _: &Self::Source,
                _: &Self::Provenance,
            ) -> AiDataCheck {
                AiDataCheck::satisfied()
            }

            fn check_policy(
                &self,
                _: &AiDataBinding<
                    Self::Source,
                    Self::Schema,
                    Self::Provenance,
                >,
            ) -> AiDataCheck {
                AiDataCheck::satisfied()
            }
        }

        let mut plan =
            AiDataPlan::<String, String, String>::new();

        plan.add(AiDataBinding::new(
            "unresolved".to_owned(),
            AiDataRole::feature(),
            AiDataAccessIntent::Read,
        ));

        let report = plan.validate(&IncompleteServices);

        assert!(!report.is_complete());
        assert!(report.has_inconclusive_results());
        assert!(!report.is_valid());
    }

    #[test]
    fn multiple_dependencies_preserve_declaration_order() {
        let mut plan =
            AiDataPlan::<String, String, String>::new();

        for index in 0..100 {
            plan.add(AiDataBinding::new(
                format!("dataset.{index}"),
                AiDataRole::feature(),
                AiDataAccessIntent::Read,
            ));
        }

        assert_eq!(plan.len(), 100);

        let report = plan.validate(&TestDataServices);

        assert!(report.is_valid());

        assert_eq!(
            plan.bindings()[0].source(),
            "dataset.0",
        );

        assert_eq!(
            plan.bindings()[99].source(),
            "dataset.99",
        );
    }

    #[test]
    fn streaming_access_is_representable() {
        let binding = AiDataBinding::<String, String, String>::new(
            "stream.reference".to_owned(),
            AiDataRole::feature(),
            AiDataAccessIntent::Streaming,
        );

        assert_eq!(
            binding.access(),
            &AiDataAccessIntent::Streaming,
        );
    }

    #[test]
    fn custom_access_intentions_are_supported() {
        let binding = AiDataBinding::<String, String, String>::new(
            "source.reference".to_owned(),
            AiDataRole::feature(),
            AiDataAccessIntent::Custom(
                "specialized.incremental.snapshot".to_owned(),
            ),
        );

        assert_eq!(
            binding.access(),
            &AiDataAccessIntent::Custom(
                "specialized.incremental.snapshot".to_owned(),
            ),
        );
    }
}

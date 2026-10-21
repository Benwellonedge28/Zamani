
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Verified, typed compilation inputs for the Zamani backend.
//!
//! Path: `src/backend/input.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//!
//! # Responsibility
//!
//! This module defines the boundary between compiler-owned representations
//! and target-independent backend operations. It provides:
//!
//! - Open-world representation and domain identifiers.
//! - Module metadata and source provenance.
//! - Explicit verification reports.
//! - Typed, type-erased storage for verified representations.
//! - A multi-unit input container for programs involving multiple domains.
//! - An adapter for the repository's existing `crate::ir_gen::IrModule`.
//!
//! # Architectural boundaries
//!
//! The frontend owns source parsing and AST construction.
//! Semantic analysis owns the meaning of a valid Zamani program.
//! Domain subsystems own their canonical domain representations.
//! IR verifiers own representation-specific verification.
//! This module owns the backend-facing input contract.
//!
//! This module does not perform parsing, semantic analysis, lowering,
//! target selection, resource allocation, scheduling, routing, QEC,
//! hardware synthesis, artifact emission, or external tool execution.
//!
//! # Verification contract
//!
//! A representation must pass a supplied `InputVerifier` before it can be
//! wrapped as a `VerifiedUnit`. There is no public unchecked constructor.
//!
//! A `VerificationReport` records what a verifier claims to have checked.
//! It is evidence supplied by the verifier, not a cryptographic proof and
//! not a substitute for trusting and testing the verifier implementation.
//! A caller must not use a no-op verifier to bypass the contract.
//!
//! Verification of one representation does not establish that another
//! representation, a target, a toolchain, or a generated artifact is valid.
//!
//! # Scalability
//!
//! Domain names, representation formats, unit counts, and metadata are
//! extensible. This module imposes no universal limits on CPUs, qubits,
//! devices, nodes, tensor rank, register width, or hardware capacity.
//! Allocation can still fail because of actual process or system resource
//! exhaustion. Operational budgets belong to configuration and consumers.
//!
//! # Integration
//!
//! - `errors.rs` supplies backend errors.
//! - `crate::source_map::Span` supplies existing source-span information.
//! - `crate::ir_gen::IrModule` and `crate::ir_verify::verify_module` provide
//!   the legacy IR adapter.
//! - Quantum, HDL, classical, tensor, distributed, and future-domain modules
//!   supply their own representation types and verifier implementations.
//! - `context.rs`, `validation.rs`, and `pipeline.rs` consume this contract.
//! - `mod.rs` must declare `pub mod input;`.
//!
//! Do not import a domain IR merely to make this generic input container
//! appear to support that domain. Use a dedicated verifier adapter at the
//! domain integration boundary.

use std::any::{Any, TypeId};
use std::collections::BTreeSet;
use std::fmt;
use std::sync::Arc;

use crate::source_map::Span;

use super::errors::{BackendError, BackendResult};

/// A validated, open-world domain identifier.
///
/// Examples include `classical`, `quantum`, `hdl`, `tensor`, and
/// `distributed`. These examples are not a closed list.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DomainId(String);

impl DomainId {
    /// Construct a domain identifier.
    ///
    /// Identifiers are case-sensitive. They must start with an ASCII
    /// lowercase letter or digit. Remaining characters may be ASCII
    /// lowercase letters, digits, `.`, `_`, or `-`.
    pub fn new(value: impl Into<String>) -> BackendResult<Self> {
        let value = value.into();

        if !valid_identifier(&value) {
            return Err(BackendError::invalid_input(format!(
                "invalid domain identifier `{value}`"
            )));
        }

        Ok(Self(value))
    }

    /// Return the canonical identifier string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DomainId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// An open-world identifier for an IR or interchange representation format.
///
/// Examples include `zamani.legacy-zir`, `zamani.quantum-ir`, and
/// `zamani.hdl-ir`. A format identifier describes representation identity;
/// it does not by itself establish that a payload is valid.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RepresentationFormat(String);

impl RepresentationFormat {
    /// Construct a representation-format identifier.
    pub fn new(value: impl Into<String>) -> BackendResult<Self> {
        let value = value.into();

        if !valid_identifier(&value) {
            return Err(BackendError::invalid_input(format!(
                "invalid representation-format identifier `{value}`"
            )));
        }

        Ok(Self(value))
    }

    /// Return the canonical identifier string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for RepresentationFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Describes the domain and representation format of one input unit.
///
/// `schema_version` identifies the schema contract for the representation.
/// It is not a Zamani language version, backend API version, or target version.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepresentationDescriptor {
    domain: DomainId,
    format: RepresentationFormat,
    schema_version: String,
}

impl RepresentationDescriptor {
    /// Construct a representation descriptor.
    pub fn new(
        domain: DomainId,
        format: RepresentationFormat,
        schema_version: impl Into<String>,
    ) -> BackendResult<Self> {
        let schema_version = schema_version.into();

        if schema_version.trim().is_empty() {
            return Err(BackendError::invalid_input(
                "representation schema version must not be empty",
            ));
        }

        if schema_version.trim() != schema_version {
            return Err(BackendError::invalid_input(
                "representation schema version must not have leading or trailing whitespace",
            ));
        }

        Ok(Self {
            domain,
            format,
            schema_version,
        })
    }

    /// Return the computational domain of this unit.
    pub fn domain(&self) -> &DomainId {
        &self.domain
    }

    /// Return the representation format.
    pub fn format(&self) -> &RepresentationFormat {
        &self.format
    }

    /// Return the representation schema version.
    pub fn schema_version(&self) -> &str {
        &self.schema_version
    }
}

/// Identifies where an input unit originated.
///
/// Provenance is descriptive metadata. It does not confer authority to
/// change program semantics or imply that the referenced source is available.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SourceProvenance {
    source_id: Option<String>,
    source_name: Option<String>,
    span: Option<Span>,
}

impl SourceProvenance {
    /// Create provenance for an input unit.
    ///
    /// The source ID is intended to be a stable caller-supplied identity.
    /// The source name is descriptive and need not be a filesystem path.
    pub fn new(
        source_id: Option<String>,
        source_name: Option<String>,
        span: Option<Span>,
    ) -> BackendResult<Self> {
        validate_optional_text("source ID", source_id.as_deref())?;
        validate_optional_text("source name", source_name.as_deref())?;

        if let Some(span) = &span {
            if span.end < span.start {
                return Err(BackendError::invalid_input(
                    "source span end precedes its start",
                ));
            }
        }

        Ok(Self {
            source_id,
            source_name,
            span,
        })
    }

    /// Return the stable source identity, when supplied.
    pub fn source_id(&self) -> Option<&str> {
        self.source_id.as_deref()
    }

    /// Return the descriptive source name, when supplied.
    pub fn source_name(&self) -> Option<&str> {
        self.source_name.as_deref()
    }

    /// Return the original source span, when supplied.
    pub fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }
}

/// Metadata shared by the compilation input and its units.
///
/// This metadata deliberately avoids storing target identity. Target
/// selection is a separate operation and must not change source meaning.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputMetadata {
    module_name: String,
    language_version: Option<String>,
    semantic_model_version: Option<String>,
    provenance: SourceProvenance,
}

impl InputMetadata {
    /// Construct input metadata.
    pub fn new(
        module_name: impl Into<String>,
        language_version: Option<String>,
        semantic_model_version: Option<String>,
        provenance: SourceProvenance,
    ) -> BackendResult<Self> {
        let module_name = module_name.into();

        if module_name.trim().is_empty() {
            return Err(BackendError::invalid_input(
                "backend input module name must not be empty",
            ));
        }

        validate_optional_text("language version", language_version.as_deref())?;
        validate_optional_text(
            "semantic model version",
            semantic_model_version.as_deref(),
        )?;

        Ok(Self {
            module_name,
            language_version,
            semantic_model_version,
            provenance,
        })
    }

    /// Return the module name.
    pub fn module_name(&self) -> &str {
        &self.module_name
    }

    /// Return the source-language version, when known.
    pub fn language_version(&self) -> Option<&str> {
        self.language_version.as_deref()
    }

    /// Return the semantic-model version, when known.
    pub fn semantic_model_version(&self) -> Option<&str> {
        self.semantic_model_version.as_deref()
    }

    /// Return module-level source provenance.
    pub fn provenance(&self) -> &SourceProvenance {
        &self.provenance
    }
}

/// Records the successful checks reported by a verifier.
///
/// The report is intentionally independent of any one IR implementation.
/// A verifier should identify itself and the checks it completed. An empty
/// check set is rejected to prevent a successful-looking report with no
/// declared verification work.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationReport {
    verifier_id: String,
    verifier_version: String,
    completed_checks: BTreeSet<String>,
    notes: Vec<String>,
}

impl VerificationReport {
    /// Create a report for a verifier that has completed successfully.
    ///
    /// Only construct this value after the declared checks have actually
    /// completed. The backend cannot independently prove that a verifier
    /// implementation is truthful.
    pub fn new(
        verifier_id: impl Into<String>,
        verifier_version: impl Into<String>,
        completed_checks: impl IntoIterator<Item = String>,
        notes: impl IntoIterator<Item = String>,
    ) -> BackendResult<Self> {
        let verifier_id = verifier_id.into();
        let verifier_version = verifier_version.into();

        validate_required_text("verifier ID", &verifier_id)?;
        validate_required_text("verifier version", &verifier_version)?;

        let mut checks = BTreeSet::new();
        for check in completed_checks {
            validate_required_text("verification check name", &check)?;
            checks.insert(check);
        }

        if checks.is_empty() {
            return Err(BackendError::verification_failed(
                "a successful verification report must identify at least one completed check",
            ));
        }

        let notes = notes.into_iter().collect::<Vec<_>>();
        for note in &notes {
            validate_required_text("verification note", note)?;
        }

        Ok(Self {
            verifier_id,
            verifier_version,
            completed_checks: checks,
            notes,
        })
    }

    /// Return the verifier identity.
    pub fn verifier_id(&self) -> &str {
        &self.verifier_id
    }

    /// Return the verifier version.
    pub fn verifier_version(&self) -> &str {
        &self.verifier_version
    }

    /// Return the stable, sorted set of completed checks.
    pub fn completed_checks(&self) -> &BTreeSet<String> {
        &self.completed_checks
    }

    /// Return explanatory notes from the verifier.
    pub fn notes(&self) -> &[String] {
        &self.notes
    }
}

/// Verifies one concrete representation type.
///
/// Implementations must return `Ok(report)` only when all checks required by
/// their declared contract have passed. A verifier must return an error for
/// invalid input and must never downgrade an incomplete verification to
/// success.
///
/// This trait is intentionally generic: each canonical IR subsystem can
/// provide its own adapter without making the backend depend on every domain.
pub trait InputVerifier<R: ?Sized> {
    /// Verify a representation and return evidence of completed checks.
    fn verify(&self, representation: &R) -> BackendResult<VerificationReport>;
}

impl<R, F> InputVerifier<R> for F
where
    F: Fn(&R) -> BackendResult<VerificationReport>,
{
    fn verify(&self, representation: &R) -> BackendResult<VerificationReport> {
        self(representation)
    }
}

/// A representation that has passed its declared input verifier.
///
/// The payload is type-erased for storage in a heterogeneous compilation
/// input, but its concrete Rust type remains recoverable through `get<T>()`.
/// No unchecked payload constructor is exposed.
#[derive(Clone)]
pub struct VerifiedUnit {
    descriptor: RepresentationDescriptor,
    provenance: SourceProvenance,
    report: VerificationReport,
    payload: Arc<dyn Any + Send + Sync>,
    payload_type_id: TypeId,
}

impl fmt::Debug for VerifiedUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VerifiedUnit")
            .field("descriptor", &self.descriptor)
            .field("provenance", &self.provenance)
            .field("report", &self.report)
            .field("payload_type_id", &self.payload_type_id)
            .finish_non_exhaustive()
    }
}

impl VerifiedUnit {
    /// Verify and wrap a representation.
    ///
    /// The representation is retained only if the verifier returns success.
    /// `R` must be thread-safe so immutable backend contexts can safely share
    /// verified inputs between compilation stages.
    pub fn verify_with<R, V>(
        representation: R,
        descriptor: RepresentationDescriptor,
        provenance: SourceProvenance,
        verifier: &V,
    ) -> BackendResult<Self>
    where
        R: Any + Send + Sync + 'static,
        V: InputVerifier<R>,
    {
        let report = verifier.verify(&representation).map_err(|error| {
            BackendError::verification_failed(format!(
                "input representation `{}` failed verification: {error}",
                descriptor.format()
            ))
            .with_context("constructing verified backend input")
            .with_cause(error.to_string())
        })?;

        Ok(Self {
            descriptor,
            provenance,
            report,
            payload: Arc::new(representation),
            payload_type_id: TypeId::of::<R>(),
        })
    }

    /// Return the representation descriptor.
    pub fn descriptor(&self) -> &RepresentationDescriptor {
        &self.descriptor
    }

    /// Return this unit's source provenance.
    pub fn provenance(&self) -> &SourceProvenance {
        &self.provenance
    }

    /// Return the verification report.
    pub fn verification(&self) -> &VerificationReport {
        &self.report
    }

    /// Return whether the payload has concrete Rust type `T`.
    pub fn is<T: Any>(&self) -> bool {
        self.payload_type_id == TypeId::of::<T>()
    }

    /// Borrow the payload if it has concrete Rust type `T`.
    ///
    /// A mismatch returns `None`; it does not coerce or reinterpret the
    /// payload as another representation.
    pub fn get<T: Any>(&self) -> Option<&T> {
        self.payload.downcast_ref::<T>()
    }

    /// Return the concrete payload's process-local Rust type identity.
    ///
    /// `TypeId` is not a persistent identifier and must not be serialized,
    /// used as a cache key, or treated as a stable representation format.
    pub fn payload_type_id(&self) -> TypeId {
        self.payload_type_id
    }
}

/// A verified input to backend compilation.
///
/// A compilation may contain several units from different domains or
/// representation formats. Their ordering is preserved because some pipeline
/// contracts may rely on explicit unit order.
///
/// This container does not prove that the units are semantically compatible
/// with one another or with a target. Those checks belong to validation and
/// domain-specific integration.
#[derive(Debug, Clone)]
pub struct BackendInput {
    metadata: InputMetadata,
    units: Vec<VerifiedUnit>,
}

impl BackendInput {
    /// Construct an input from one or more already-verified units.
    ///
    /// Empty inputs are rejected. An empty program, if valid in Zamani, must
    /// be represented by a valid verified unit rather than by bypassing the
    /// representation contract.
    pub fn new(
        metadata: InputMetadata,
        units: Vec<VerifiedUnit>,
    ) -> BackendResult<Self> {
        if units.is_empty() {
            return Err(BackendError::invalid_input(
                "backend input must contain at least one verified representation unit",
            ));
        }

        Ok(Self { metadata, units })
    }

    /// Construct an input containing exactly one verified unit.
    pub fn single(
        metadata: InputMetadata,
        unit: VerifiedUnit,
    ) -> BackendResult<Self> {
        Self::new(metadata, vec![unit])
    }

    /// Return shared module metadata.
    pub fn metadata(&self) -> &InputMetadata {
        &self.metadata
    }

    /// Return the verified units in their declared order.
    pub fn units(&self) -> &[VerifiedUnit] {
        &self.units
    }

    /// Return the number of representation units.
    pub fn unit_count(&self) -> usize {
        self.units.len()
    }

    /// Return whether the input contains at least one unit in `domain`.
    pub fn contains_domain(&self, domain: &DomainId) -> bool {
        self.units
            .iter()
            .any(|unit| unit.descriptor().domain() == domain)
    }

    /// Return the units whose domain matches `domain`.
    pub fn units_for_domain<'a>(
        &'a self,
        domain: &'a DomainId,
    ) -> impl Iterator<Item = &'a VerifiedUnit> + 'a {
        self.units
            .iter()
            .filter(move |unit| unit.descriptor().domain() == domain)
    }

    /// Return whether every unit has a successful, nonempty verification
    /// report.
    ///
    /// This checks the container's recorded contract. It does not rerun the
    /// verifiers or independently establish their correctness.
    pub fn has_verification_evidence(&self) -> bool {
        !self.units.is_empty()
            && self.units.iter().all(|unit| {
                !unit.verification().completed_checks().is_empty()
                    && !unit.verification().verifier_id().is_empty()
                    && !unit.verification().verifier_version().is_empty()
            })
    }

    /// Adapt the repository's existing legacy `IrModule` through its
    /// authoritative verifier.
    ///
    /// This adapter is intentionally specific to the current legacy IR. It
    /// does not claim that `IrModule` is the universal IR for quantum, HDL,
    /// tensor, distributed, or future domains.
    pub fn from_legacy_ir(
        module: crate::ir_gen::IrModule,
        metadata: InputMetadata,
        provenance: SourceProvenance,
    ) -> BackendResult<Self> {
        let descriptor = RepresentationDescriptor::new(
            DomainId::new("classical")?,
            RepresentationFormat::new("zamani.legacy-zir")?,
            "1",
        )?;

        let verifier = |module: &crate::ir_gen::IrModule| {
            crate::ir_verify::verify_module(module).map_err(|errors| {
                let message = errors
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join("; ");

                BackendError::verification_failed(format!(
                    "legacy IR verification failed: {message}"
                ))
            })?;

            VerificationReport::new(
                "zamani.legacy-ir-verifier",
                env!("CARGO_PKG_VERSION"),
                ["legacy-ir-structural-verification".to_owned()],
                std::iter::empty(),
            )
        };

        let unit = VerifiedUnit::verify_with(
            module,
            descriptor,
            provenance,
            &verifier,
        )?;

        Self::single(metadata, unit)
    }
}

/// Validate the deliberately open-world identifier grammar.
///
/// This is a representation/domain naming rule, not a language grammar rule.
/// It must not be used to parse Zamani source.
fn valid_identifier(value: &str) -> bool {
    let mut chars = value.chars();

    let Some(first) = chars.next() else {
        return false;
    };

    if !(first.is_ascii_lowercase() || first.is_ascii_digit()) {
        return false;
    }

    chars.all(|character| {
        character.is_ascii_lowercase()
            || character.is_ascii_digit()
            || matches!(character, '.' | '_' | '-')
    })
}

fn validate_required_text(field: &str, value: &str) -> BackendResult<()> {
    if value.trim().is_empty() {
        return Err(BackendError::invalid_input(format!(
            "{field} must not be empty"
        )));
    }

    if value.trim() != value {
        return Err(BackendError::invalid_input(format!(
            "{field} must not have leading or trailing whitespace"
        )));
    }

    Ok(())
}

fn validate_optional_text(
    field: &str,
    value: Option<&str>,
) -> BackendResult<()> {
    if let Some(value) = value {
        validate_required_text(field, value)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    struct TestRepresentation {
        operations: Vec<String>,
    }

    fn descriptor() -> RepresentationDescriptor {
        RepresentationDescriptor::new(
            DomainId::new("classical").expect("valid domain"),
            RepresentationFormat::new("zamani.test-ir")
                .expect("valid format"),
            "1",
        )
        .expect("valid descriptor")
    }

    fn metadata() -> InputMetadata {
        InputMetadata::new(
            "test_module",
            Some("1".to_owned()),
            Some("1".to_owned()),
            SourceProvenance::default(),
        )
        .expect("valid metadata")
    }

    fn report() -> BackendResult<VerificationReport> {
        VerificationReport::new(
            "zamani.test-verifier",
            "1",
            ["structure".to_owned(), "types".to_owned()],
            std::iter::empty(),
        )
    }

    fn test_verifier(
        _: &TestRepresentation,
    ) -> BackendResult<VerificationReport> {
        report()
    }

    #[test]
    fn verified_unit_preserves_typed_payload_and_report() {
        let payload = TestRepresentation {
            operations: vec!["add".to_owned()],
        };

        let unit = VerifiedUnit::verify_with(
            payload,
            descriptor(),
            SourceProvenance::default(),
            &test_verifier,
        )
        .expect("verification should succeed");

        assert!(unit.is::<TestRepresentation>());
        assert!(!unit.is::<String>());
        assert_eq!(
            unit.get::<TestRepresentation>()
                .expect("payload type should match")
                .operations,
            vec!["add"]
        );
        assert_eq!(
            unit.verification().verifier_id(),
            "zamani.test-verifier"
        );
    }

    #[test]
    fn failed_verification_does_not_create_a_unit() {
        let verifier = |_: &TestRepresentation| {
            Err(BackendError::verification_failed(
                "deliberate test failure",
            ))
        };

        let result = VerifiedUnit::verify_with(
            TestRepresentation {
                operations: Vec::new(),
            },
            descriptor(),
            SourceProvenance::default(),
            &verifier,
        );

        assert!(result.is_err());
    }

    #[test]
    fn empty_verification_report_is_rejected() {
        let result = VerificationReport::new(
            "test.verifier",
            "1",
            std::iter::empty(),
            std::iter::empty(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn identifiers_are_open_world_but_validated() {
        assert!(DomainId::new("neuromorphic.v2").is_ok());
        assert!(DomainId::new("future-hardware").is_ok());
        assert!(DomainId::new("Quantum").is_err());
        assert!(DomainId::new("classical domain").is_err());
        assert!(RepresentationFormat::new("vendor.experimental-ir").is_ok());
    }

    #[test]
    fn backend_input_preserves_unit_order_and_domains() {
        let classical = VerifiedUnit::verify_with(
            TestRepresentation {
                operations: vec!["add".to_owned()],
            },
            descriptor(),
            SourceProvenance::default(),
            &test_verifier,
        )
        .expect("classical unit should verify");

        let quantum_descriptor = RepresentationDescriptor::new(
            DomainId::new("quantum").expect("valid domain"),
            RepresentationFormat::new("zamani.test-quantum-ir")
                .expect("valid format"),
            "1",
        )
        .expect("valid descriptor");

        let quantum = VerifiedUnit::verify_with(
            TestRepresentation {
                operations: vec!["measure".to_owned()],
            },
            quantum_descriptor,
            SourceProvenance::default(),
            &test_verifier,
        )
        .expect("quantum test unit should verify");

        let input = BackendInput::new(
            metadata(),
            vec![classical, quantum],
        )
        .expect("input should be valid");

        assert_eq!(input.unit_count(), 2);
        assert_eq!(
            input.units()[0].descriptor().domain().as_str(),
            "classical"
        );
        assert_eq!(
            input.units()[1].descriptor().domain().as_str(),
            "quantum"
        );
        assert!(input.contains_domain(
            &DomainId::new("quantum").expect("valid domain")
        ));
        assert!(input.has_verification_evidence());
    }

    #[test]
    fn empty_input_is_rejected() {
        assert!(BackendInput::new(metadata(), Vec::new()).is_err());
    }

    #[test]
    fn module_and_verifier_metadata_are_validated() {
        assert!(InputMetadata::new(
            " ",
            None,
            None,
            SourceProvenance::default(),
        )
        .is_err());

        assert!(VerificationReport::new(
            "verifier",
            "1",
            [" ".to_owned()],
            std::iter::empty(),
        )
        .is_err());
    }

    #[test]
    fn legacy_ir_adapter_uses_the_existing_verifier() {
        let module = crate::ir_gen::IrModule::new("input_test");

        let input = BackendInput::from_legacy_ir(
            module,
            metadata(),
            SourceProvenance::default(),
        )
        .expect("empty legacy module should pass the current verifier");

        assert_eq!(input.unit_count(), 1);
        assert_eq!(
            input.units()[0].descriptor().format().as_str(),
            "zamani.legacy-zir"
        );
        assert!(input.units()[0].is::<crate::ir_gen::IrModule>());
        assert!(input.has_verification_evidence());
    }
}

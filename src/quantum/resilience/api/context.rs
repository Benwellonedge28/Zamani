//! Resilience execution context.
//!
//! This module defines the immutable context passed through the Zamani
//! quantum-resilience pipeline.
//!
//! # Architectural role
//!
//! `ResilienceContext` is deliberately a context contract, not an execution
//! engine. It does not:
//!
//! - execute quantum programs,
//! - perform routing,
//! - schedule operations,
//! - optimize circuits,
//! - decode QEC syndromes,
//! - detect faults,
//! - select recovery strategies,
//! - mutate hardware state,
//! - contain provider-specific logic,
//! - contain fixed hardware limits.
//!
//! Those responsibilities belong to their respective quantum subsystems.
//!
//! The context provides a stable view of the inputs those subsystems need
//! while resilience is evaluating one execution.
//!
//! # Scalability
//!
//! There is no fixed qubit count, device count, backend count, retry count,
//! topology size, or execution-size assumption in this module.
//!
//! Resource quantities are represented by dynamically sized collections or
//! discovered/configured values supplied by the caller.
//!
//! # Ownership
//!
//! The context borrows the actual subsystem implementations. Resilience does
//! not take ownership of hardware, routing, scheduling, QEC, telemetry, or
//! optimization engines merely because it needs to consult them.
//!
//! This prevents accidental duplication of global quantum infrastructure.
//!
//! # Determinism
//!
//! A context may carry an explicit deterministic execution configuration.
//! The context itself does not generate randomness.
//!
//! # Quantum identity
//!
//! Logical/affected qubits use the canonical repository type:
//!
//! `crate::quantum::ir::qubit::QubitId`
//!
//! No second resilience-specific qubit identifier is introduced here.

use std::fmt;
use std::sync::Arc;

use crate::quantum::ir::qubit::QubitId;

/// Current context-contract schema version.
///
/// This is a protocol/schema version, not a hardware or resource limit.
/// It may therefore be changed independently of the number of qubits,
/// devices, backends, or execution resources supported by Zamani.
pub const RESILIENCE_CONTEXT_SCHEMA_VERSION: u16 = 1;

/// Shared immutable string used by the resilience API.
type SharedString = Arc<str>;

/// Identifies a resilience request/execution without imposing a particular
/// identifier-generation mechanism on the rest of Zamani.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ContextId(SharedString);

impl ContextId {
    /// Creates a context identifier.
    ///
    /// The caller owns identifier generation. This keeps UUIDs, ULIDs,
    /// content-addressed identifiers, distributed IDs, or another future
    /// mechanism outside this module.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, ContextIdError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(ContextIdError::Empty);
        }

        Ok(Self(value))
    }

    /// Returns the identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ContextId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Error returned when creating an invalid [`ContextId`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ContextIdError {
    /// The identifier was empty or contained only whitespace.
    Empty,
}

impl fmt::Display for ContextIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("resilience context identifier is empty"),
        }
    }
}

impl std::error::Error for ContextIdError {}

/// Identifies the phase for which a context is being evaluated.
///
/// The context is immutable, so changing phase means creating a new context
/// or deriving a phase-specific view rather than mutating shared state.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ResiliencePhase {
    /// Initial execution context.
    Execution,

    /// Fault/event observation.
    Detection,

    /// Fault diagnosis.
    Diagnosis,

    /// Policy and constraint evaluation.
    Policy,

    /// Recovery/adaptation planning.
    Planning,

    /// Hardware/program adaptation.
    Adaptation,

    /// Recovery execution.
    Recovery,

    /// Error mitigation.
    Mitigation,

    /// Semantic/result verification.
    Verification,
}

impl Default for ResiliencePhase {
    fn default() -> Self {
        Self::Execution
    }
}

/// Requested determinism semantics for a resilience execution.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DeterminismMode {
    /// Deterministic behavior is not explicitly requested.
    Unspecified,

    /// Components must avoid nondeterministic decisions wherever possible.
    Deterministic,

    /// Components must use only explicitly supplied deterministic inputs for
    /// decisions where deterministic behavior is contractually required.
    Strict,
}

impl Default for DeterminismMode {
    fn default() -> Self {
        Self::Unspecified
    }
}

/// Identifies the logical execution domain.
///
/// This is deliberately descriptive rather than hardware-specific.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ExecutionDomain {
    /// One logical quantum execution.
    Local,

    /// Execution may span multiple quantum resources.
    Distributed,

    /// Execution is explicitly hybrid quantum/classical.
    Hybrid,

    /// The domain is determined by the target/resource providers.
    Automatic,
}

impl Default for ExecutionDomain {
    fn default() -> Self {
        Self::Automatic
    }
}

/// Immutable execution metadata that is safe to carry through the entire
/// resilience lifecycle.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContextMetadata {
    context_id: ContextId,
    request_id: Option<SharedString>,
    execution_id: Option<SharedString>,
    phase: ResiliencePhase,
    domain: ExecutionDomain,
    determinism: DeterminismMode,
}

impl ContextMetadata {
    /// Creates metadata for a resilience context.
    pub fn new(context_id: ContextId) -> Self {
        Self {
            context_id,
            request_id: None,
            execution_id: None,
            phase: ResiliencePhase::Execution,
            domain: ExecutionDomain::Automatic,
            determinism: DeterminismMode::Unspecified,
        }
    }

    /// Returns the context identifier.
    #[must_use]
    pub fn context_id(&self) -> &ContextId {
        &self.context_id
    }

    /// Returns the request identifier, if one was supplied.
    #[must_use]
    pub fn request_id(&self) -> Option<&str> {
        self.request_id.as_deref()
    }

    /// Returns the execution identifier, if one was supplied.
    #[must_use]
    pub fn execution_id(&self) -> Option<&str> {
        self.execution_id.as_deref()
    }

    /// Returns the current resilience phase.
    #[must_use]
    pub const fn phase(&self) -> ResiliencePhase {
        self.phase
    }

    /// Returns the execution domain.
    #[must_use]
    pub const fn domain(&self) -> ExecutionDomain {
        self.domain
    }

    /// Returns the determinism mode.
    #[must_use]
    pub const fn determinism(&self) -> DeterminismMode {
        self.determinism
    }

    /// Associates the context with a request.
    #[must_use]
    pub fn with_request_id(mut self, request_id: impl Into<Arc<str>>) -> Self {
        self.request_id = Some(request_id.into());
        self
    }

    /// Associates the context with an execution.
    #[must_use]
    pub fn with_execution_id(mut self, execution_id: impl Into<Arc<str>>) -> Self {
        self.execution_id = Some(execution_id.into());
        self
    }

    /// Creates a context metadata value for another resilience phase.
    #[must_use]
    pub fn with_phase(mut self, phase: ResiliencePhase) -> Self {
        self.phase = phase;
        self
    }

    /// Sets the execution domain.
    #[must_use]
    pub fn with_domain(mut self, domain: ExecutionDomain) -> Self {
        self.domain = domain;
        self
    }

    /// Sets deterministic execution requirements.
    #[must_use]
    pub fn with_determinism(mut self, determinism: DeterminismMode) -> Self {
        self.determinism = determinism;
        self
    }
}

/// A dynamically sized collection of logical qubits relevant to this
/// resilience execution.
///
/// No maximum is imposed by the resilience layer.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AffectedQubits(Arc<[QubitId]>);

impl AffectedQubits {
    /// Creates an affected-qubit collection.
    ///
    /// The collection is owned by the context and is immutable after
    /// construction.
    #[must_use]
    pub fn new(qubits: Vec<QubitId>) -> Self {
        Self(Arc::from(qubits.into_boxed_slice()))
    }

    /// Creates an empty affected-qubit collection.
    #[must_use]
    pub fn empty() -> Self {
        Self::default()
    }

    /// Returns all affected logical qubits.
    #[must_use]
    pub fn as_slice(&self) -> &[QubitId] {
        &self.0
    }

    /// Returns the number of affected qubits.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns whether no qubits are recorded.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns an iterator over affected qubits.
    pub fn iter(&self) -> impl Iterator<Item = &QubitId> {
        self.0.iter()
    }
}

/// Generic immutable resource snapshot.
///
/// The concrete hardware/resource model belongs to the hardware HAL and
/// related quantum subsystems. Resilience only needs an opaque reference to
/// that authoritative model.
#[derive(Clone, Copy, Debug)]
pub struct ResourceSnapshot<'a, H> {
    hardware: &'a H,
}

impl<'a, H> ResourceSnapshot<'a, H> {
    /// Creates a resource snapshot reference.
    #[must_use]
    pub const fn new(hardware: &'a H) -> Self {
        Self { hardware }
    }

    /// Returns the authoritative hardware/resource model.
    #[must_use]
    pub const fn hardware(&self) -> &'a H {
        self.hardware
    }
}

/// Immutable references to the quantum subsystems used by resilience.
///
/// The types are generic intentionally:
///
/// - the hardware HAL remains authoritative for hardware,
/// - routing remains authoritative for routing,
/// - scheduling remains authoritative for scheduling,
/// - QEC remains authoritative for error correction,
/// - telemetry remains authoritative for observations,
/// - optimization remains authoritative for optimization.
///
/// Resilience orchestrates these contracts but does not replace them.
///
/// The type parameters also allow Zamani to evolve concrete implementations
/// without requiring this file to be rewritten.
#[derive(Clone, Copy, Debug)]
pub struct ResilienceServices<'a, Hardware, Router, Scheduler, Qec, Telemetry, Optimizer>
{
    hardware: &'a Hardware,
    router: &'a Router,
    scheduler: &'a Scheduler,
    qec: &'a Qec,
    telemetry: &'a Telemetry,
    optimizer: &'a Optimizer,
}

impl<'a, Hardware, Router, Scheduler, Qec, Telemetry, Optimizer>
    ResilienceServices<'a, Hardware, Router, Scheduler, Qec, Telemetry, Optimizer>
{
    /// Creates the immutable service-reference bundle.
    #[must_use]
    pub const fn new(
        hardware: &'a Hardware,
        router: &'a Router,
        scheduler: &'a Scheduler,
        qec: &'a Qec,
        telemetry: &'a Telemetry,
        optimizer: &'a Optimizer,
    ) -> Self {
        Self {
            hardware,
            router,
            scheduler,
            qec,
            telemetry,
            optimizer,
        }
    }

    /// Returns the hardware HAL reference.
    #[must_use]
    pub const fn hardware(&self) -> &'a Hardware {
        self.hardware
    }

    /// Returns the routing service reference.
    #[must_use]
    pub const fn router(&self) -> &'a Router {
        self.router
    }

    /// Returns the scheduling service reference.
    #[must_use]
    pub const fn scheduler(&self) -> &'a Scheduler {
        self.scheduler
    }

    /// Returns the QEC service reference.
    #[must_use]
    pub const fn qec(&self) -> &'a Qec {
        self.qec
    }

    /// Returns the telemetry service reference.
    #[must_use]
    pub const fn telemetry(&self) -> &'a Telemetry {
        self.telemetry
    }

    /// Returns the optimizer reference.
    #[must_use]
    pub const fn optimizer(&self) -> &'a Optimizer {
        self.optimizer
    }
}

/// Optional opaque execution artifacts.
///
/// The context must not assume that a particular backend, runtime, compiler,
/// provider, or execution implementation exists. Therefore execution-specific
/// state can be supplied as an immutable borrowed value of the caller's
/// concrete type.
#[derive(Clone, Copy, Debug)]
pub struct ExecutionArtifacts<'a, T> {
    value: &'a T,
}

impl<'a, T> ExecutionArtifacts<'a, T> {
    /// Creates an execution-artifact reference.
    #[must_use]
    pub const fn new(value: &'a T) -> Self {
        Self { value }
    }

    /// Returns the artifact reference.
    #[must_use]
    pub const fn value(&self) -> &'a T {
        self.value
    }
}

/// The production resilience execution context.
///
/// # Type parameters
///
/// `Program`
/// : canonical quantum program/IR representation.
///
/// `Hardware`
/// : hardware HAL or resource-provider implementation.
///
/// `Router`
/// : routing implementation.
///
/// `Scheduler`
/// : scheduling implementation.
///
/// `Qec`
/// : QEC implementation.
///
/// `Telemetry`
/// : telemetry/observation implementation.
///
/// `Optimizer`
/// : canonical quantum optimization implementation.
///
/// `Execution`
/// : optional concrete execution-runtime state.
///
/// Keeping these types generic prevents resilience from becoming coupled to
/// one hardware vendor, one simulator, one runtime, or one future execution
/// fabric.
#[derive(Debug)]
pub struct ResilienceContext<
    'a,
    Program,
    Hardware,
    Router,
    Scheduler,
    Qec,
    Telemetry,
    Optimizer,
    Execution,
> {
    schema_version: u16,
    metadata: ContextMetadata,
    program: &'a Program,
    services: ResilienceServices<
        'a,
        Hardware,
        Router,
        Scheduler,
        Qec,
        Telemetry,
        Optimizer,
    >,
    execution: Option<ExecutionArtifacts<'a, Execution>>,
    affected_qubits: AffectedQubits,
}

impl<
        'a,
        Program,
        Hardware,
        Router,
        Scheduler,
        Qec,
        Telemetry,
        Optimizer,
        Execution,
    >
    ResilienceContext<
        'a,
        Program,
        Hardware,
        Router,
        Scheduler,
        Qec,
        Telemetry,
        Optimizer,
        Execution,
    >
{
    /// Constructs a validated resilience context.
    ///
    /// The context borrows all externally owned resources. It therefore does
    /// not allocate or duplicate a complete hardware topology, circuit,
    /// scheduler, router, or QEC engine merely to construct a resilience
    /// context.
    pub fn new(
        metadata: ContextMetadata,
        program: &'a Program,
        services: ResilienceServices<
            'a,
            Hardware,
            Router,
            Scheduler,
            Qec,
            Telemetry,
            Optimizer,
        >,
    ) -> Self {
        Self {
            schema_version: RESILIENCE_CONTEXT_SCHEMA_VERSION,
            metadata,
            program,
            services,
            execution: None,
            affected_qubits: AffectedQubits::empty(),
        }
    }

    /// Returns the context schema version.
    #[must_use]
    pub const fn schema_version(&self) -> u16 {
        self.schema_version
    }

    /// Returns immutable execution metadata.
    #[must_use]
    pub const fn metadata(&self) -> &ContextMetadata {
        &self.metadata
    }

    /// Returns the canonical quantum program/IR.
    #[must_use]
    pub const fn program(&self) -> &'a Program {
        self.program
    }

    /// Returns immutable references to the quantum services.
    #[must_use]
    pub const fn services(
        &self,
    ) -> &ResilienceServices<
        'a,
        Hardware,
        Router,
        Scheduler,
        Qec,
        Telemetry,
        Optimizer,
    > {
        &self.services
    }

    /// Returns the optional execution-runtime state.
    #[must_use]
    pub const fn execution(&self) -> Option<ExecutionArtifacts<'a, Execution>> {
        self.execution
    }

    /// Returns logical qubits known to be affected by the current incident or
    /// execution.
    #[must_use]
    pub const fn affected_qubits(&self) -> &AffectedQubits {
        &self.affected_qubits
    }

    /// Attaches immutable execution-runtime state.
    ///
    /// The runtime remains owned by the runtime/execution subsystem.
    #[must_use]
    pub const fn with_execution(
        mut self,
        execution: &'a Execution,
    ) -> Self {
        self.execution = Some(ExecutionArtifacts::new(execution));
        self
    }

    /// Attaches the currently affected logical qubits.
    ///
    /// The supplied vector is moved into an immutable dynamically sized
    /// collection. No fixed qubit limit is imposed.
    #[must_use]
    pub fn with_affected_qubits(mut self, qubits: Vec<QubitId>) -> Self {
        self.affected_qubits = AffectedQubits::new(qubits);
        self
    }

    /// Creates a context representing another resilience phase while
    /// preserving all immutable execution resources.
    #[must_use]
    pub fn for_phase(mut self, phase: ResiliencePhase) -> Self {
        self.metadata = self.metadata.clone().with_phase(phase);
        self
    }

    /// Returns the hardware/resource provider.
    #[must_use]
    pub const fn hardware(&self) -> &'a Hardware {
        self.services.hardware()
    }

    /// Returns the routing implementation.
    #[must_use]
    pub const fn router(&self) -> &'a Router {
        self.services.router()
    }

    /// Returns the scheduling implementation.
    #[must_use]
    pub const fn scheduler(&self) -> &'a Scheduler {
        self.services.scheduler()
    }

    /// Returns the QEC implementation.
    #[must_use]
    pub const fn qec(&self) -> &'a Qec {
        self.services.qec()
    }

    /// Returns the telemetry implementation.
    #[must_use]
    pub const fn telemetry(&self) -> &'a Telemetry {
        self.services.telemetry()
    }

    /// Returns the optimizer implementation.
    #[must_use]
    pub const fn optimizer(&self) -> &'a Optimizer {
        self.services.optimizer()
    }

    /// Returns a compact immutable resource view.
    #[must_use]
    pub const fn resource_snapshot(&self) -> ResourceSnapshot<'a, Hardware> {
        ResourceSnapshot::new(self.hardware())
    }

    /// Checks the context's structural invariants.
    ///
    /// This intentionally validates only invariants owned by this module.
    /// Hardware capability validation belongs to the hardware HAL, IR
    /// validation belongs to the canonical IR, and policy validation belongs
    /// to `policy/`.
    pub fn validate(&self) -> Result<(), ContextValidationError> {
        if self.metadata.context_id().as_str().trim().is_empty() {
            return Err(ContextValidationError::MissingContextId);
        }

        if self.schema_version == 0 {
            return Err(ContextValidationError::InvalidSchemaVersion);
        }

        Ok(())
    }
}

/// Context validation failures.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ContextValidationError {
    /// Context identifier is missing.
    MissingContextId,

    /// Context schema version is invalid.
    InvalidSchemaVersion,
}

impl fmt::Display for ContextValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingContextId => {
                formatter.write_str("resilience context is missing a context identifier")
            }
            Self::InvalidSchemaVersion => {
                formatter.write_str("resilience context has an invalid schema version")
            }
        }
    }
}

impl std::error::Error for ContextValidationError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct Program;

    #[derive(Debug)]
    struct Hardware;

    #[derive(Debug)]
    struct Router;

    #[derive(Debug)]
    struct Scheduler;

    #[derive(Debug)]
    struct Qec;

    #[derive(Debug)]
    struct Telemetry;

    #[derive(Debug)]
    struct Optimizer;

    #[derive(Debug)]
    struct Execution;

    fn services<'a>(
        hardware: &'a Hardware,
        router: &'a Router,
        scheduler: &'a Scheduler,
        qec: &'a Qec,
        telemetry: &'a Telemetry,
        optimizer: &'a Optimizer,
    ) -> ResilienceServices<
        'a,
        Hardware,
        Router,
        Scheduler,
        Qec,
        Telemetry,
        Optimizer,
    > {
        ResilienceServices::new(
            hardware,
            router,
            scheduler,
            qec,
            telemetry,
            optimizer,
        )
    }

    #[test]
    fn context_id_rejects_empty_identifier() {
        let result = ContextId::new(Arc::<str>::from(""));

        assert_eq!(result, Err(ContextIdError::Empty));
    }

    #[test]
    fn context_id_accepts_non_empty_identifier() {
        let id = ContextId::new(Arc::<str>::from("context-1"))
            .expect("valid context identifier");

        assert_eq!(id.as_str(), "context-1");
    }

    #[test]
    fn affected_qubits_support_empty_dynamic_collection() {
        let qubits = AffectedQubits::empty();

        assert!(qubits.is_empty());
        assert_eq!(qubits.len(), 0);
    }

    #[test]
    fn metadata_is_immutable_by_value() {
        let id = ContextId::new(Arc::<str>::from("context-1"))
            .expect("valid context identifier");

        let metadata = ContextMetadata::new(id)
            .with_request_id(Arc::<str>::from("request-1"))
            .with_execution_id(Arc::<str>::from("execution-1"))
            .with_phase(ResiliencePhase::Detection)
            .with_domain(ExecutionDomain::Hybrid)
            .with_determinism(DeterminismMode::Strict);

        assert_eq!(metadata.request_id(), Some("request-1"));
        assert_eq!(metadata.execution_id(), Some("execution-1"));
        assert_eq!(metadata.phase(), ResiliencePhase::Detection);
        assert_eq!(metadata.domain(), ExecutionDomain::Hybrid);
        assert_eq!(metadata.determinism(), DeterminismMode::Strict);
    }

    #[test]
    fn context_preserves_canonical_program_reference() {
        let program = Program;

        let hardware = Hardware;
        let router = Router;
        let scheduler = Scheduler;
        let qec = Qec;
        let telemetry = Telemetry;
        let optimizer = Optimizer;

        let context_id = ContextId::new(Arc::<str>::from("context-1"))
            .expect("valid context identifier");

        let metadata = ContextMetadata::new(context_id);

        let context = ResilienceContext::<
            '_,
            Program,
            Hardware,
            Router,
            Scheduler,
            Qec,
            Telemetry,
            Optimizer,
            Execution,
        >::new(
            metadata,
            &program,
            services(
                &hardware,
                &router,
                &scheduler,
                &qec,
                &telemetry,
                &optimizer,
            ),
        );

        assert_eq!(context.schema_version(), RESILIENCE_CONTEXT_SCHEMA_VERSION);
        assert!(std::ptr::eq(context.program(), &program));
        assert!(context.execution().is_none());
        assert!(context.affected_qubits().is_empty());
    }

    #[test]
    fn context_can_attach_execution_without_taking_ownership() {
        let program = Program;

        let hardware = Hardware;
        let router = Router;
        let scheduler = Scheduler;
        let qec = Qec;
        let telemetry = Telemetry;
        let optimizer = Optimizer;
        let execution = Execution;

        let context_id = ContextId::new(Arc::<str>::from("context-1"))
            .expect("valid context identifier");

        let metadata = ContextMetadata::new(context_id);

        let context = ResilienceContext::<
            '_,
            Program,
            Hardware,
            Router,
            Scheduler,
            Qec,
            Telemetry,
            Optimizer,
            Execution,
        >::new(
            metadata,
            &program,
            services(
                &hardware,
                &router,
                &scheduler,
                &qec,
                &telemetry,
                &optimizer,
            ),
        )
        .with_execution(&execution);

        assert!(context.execution().is_some());
        assert!(std::ptr::eq(
            context.execution().expect("execution").value(),
            &execution
        ));
    }

    #[test]
    fn context_phase_transition_does_not_change_program_or_services() {
        let program = Program;

        let hardware = Hardware;
        let router = Router;
        let scheduler = Scheduler;
        let qec = Qec;
        let telemetry = Telemetry;
        let optimizer = Optimizer;

        let context_id = ContextId::new(Arc::<str>::from("context-1"))
            .expect("valid context identifier");

        let metadata = ContextMetadata::new(context_id);

        let context = ResilienceContext::<
            '_,
            Program,
            Hardware,
            Router,
            Scheduler,
            Qec,
            Telemetry,
            Optimizer,
            Execution,
        >::new(
            metadata,
            &program,
            services(
                &hardware,
                &router,
                &scheduler,
                &qec,
                &telemetry,
                &optimizer,
            ),
        );

        let next = context.for_phase(ResiliencePhase::Diagnosis);

        assert_eq!(next.metadata().phase(), ResiliencePhase::Diagnosis);
        assert!(std::ptr::eq(next.program(), &program));
        assert!(std::ptr::eq(next.hardware(), &hardware));
        assert!(std::ptr::eq(next.router(), &router));
        assert!(std::ptr::eq(next.scheduler(), &scheduler));
        assert!(std::ptr::eq(next.qec(), &qec));
        assert!(std::ptr::eq(next.telemetry(), &telemetry));
        assert!(std::ptr::eq(next.optimizer(), &optimizer));
    }

    #[test]
    fn context_validation_succeeds_for_valid_context() {
        let program = Program;

        let hardware = Hardware;
        let router = Router;
        let scheduler = Scheduler;
        let qec = Qec;
        let telemetry = Telemetry;
        let optimizer = Optimizer;

        let context_id = ContextId::new(Arc::<str>::from("context-1"))
            .expect("valid context identifier");

        let metadata = ContextMetadata::new(context_id);

        let context = ResilienceContext::<
            '_,
            Program,
            Hardware,
            Router,
            Scheduler,
            Qec,
            Telemetry,
            Optimizer,
            Execution,
        >::new(
            metadata,
            &program,
            services(
                &hardware,
                &router,
                &scheduler,
                &qec,
                &telemetry,
                &optimizer,
            ),
        );

        assert_eq!(context.validate(), Ok(()));
    }

    #[test]
    fn context_supports_dynamic_affected_qubit_storage() {
        let program = Program;

        let hardware = Hardware;
        let router = Router;
        let scheduler = Scheduler;
        let qec = Qec;
        let telemetry = Telemetry;
        let optimizer = Optimizer;

        let context_id = ContextId::new(Arc::<str>::from("context-1"))
            .expect("valid context identifier");

        let metadata = ContextMetadata::new(context_id);

        let context = ResilienceContext::<
            '_,
            Program,
            Hardware,
            Router,
            Scheduler,
            Qec,
            Telemetry,
            Optimizer,
            Execution,
        >::new(
            metadata,
            &program,
            services(
                &hardware,
                &router,
                &scheduler,
                &qec,
                &telemetry,
                &optimizer,
            ),
        )
        .with_affected_qubits(Vec::new());

        assert!(context.affected_qubits().is_empty());
    }
}
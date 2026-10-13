
/*!
PURPOSE
    Validate target-independent semantics for distributed computations in Zamani.

STATUS
    Standalone implementation proposed for integration. The repository's actual
    AST, diagnostic, capability, effect, and semantic-model interfaces must be
    verified before the module is declared integrated or frozen.

AUTHORITY
    The canonical Zamani grammar, AST, semantic specifications, shared semantic
    model, and the distributed-domain rules approved for the language.

OWNS
    Distributed declarations, communication contracts, dependency references,
    symbolic worker/resource expressions, structural invariants, and
    deterministic distributed-domain diagnostics.

DOES_NOT_OWN
    Parsing; the canonical AST; global symbol or type ownership; global effect
    or capability ownership; network transport; physical placement; scheduling;
    target feasibility; failure recovery execution; distributed IR; runtime state.

INPUTS
    A DistributedProgram produced by an adapter from the canonical frontend AST,
    plus a DataSemanticContext implementation backed by shared semantic services.

OUTPUTS
    A deterministic DistributedValidationReport. Incomplete context analysis
    cannot be reported as valid.

INTEGRATION
    1. Adapt canonical AST nodes into this module's input records.
    2. Implement DistributedSemanticContext using shared semantic services.
    3. Call validate_distributed from domains/distributed.rs's parent registry
       or the semantic pipeline.
    4. Translate issues to the repository's canonical diagnostic type.
    5. Store validated facts/obligations in the shared semantic model.
    6. Leave target feasibility and execution to downstream systems.

SCALABILITY
    No architectural worker, node, message, topology, or resource ceilings are
    encoded here. Symbolic expressions are preserved. Operational limits belong
    to the shared semantic configuration and must report incomplete analysis.

DETERMINISM
    Ordered maps/sets and stable issue sorting make output deterministic for
    equivalent normalized input.

ERROR HANDLING
    Invalid declarations produce issues. Missing context knowledge, unsupported
    extension checks, and analysis-budget failures must never silently produce
    a valid result.

FREEZE CRITERIA
    Adaptation to canonical AST and diagnostics is complete; shared-service
    delegation is wired; unit, integration, determinism, malformed-input, and
    stress tests pass on the supported workspace toolchain.
*/

#![forbid(unsafe_code)]

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt;

/// An extensible, qualified identifier.
///
/// Identifiers are preserved exactly: normalization and case folding are
/// language-policy decisions and must not be invented by this module.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct QualifiedId {
    namespace: String,
    name: String,
}

impl QualifiedId {
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Self, ModelError> {
        let namespace = namespace.into();
        let name = name.into();

        if namespace.trim().is_empty() {
            return Err(ModelError::EmptyIdentifierNamespace);
        }
        if name.trim().is_empty() {
            return Err(ModelError::EmptyIdentifierName);
        }

        Ok(Self { namespace, name })
    }

    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for QualifiedId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}::{}", self.namespace, self.name)
    }
}

/// Stable identity within the supplied distributed semantic model.
///
/// Construction validates emptiness but does not claim global uniqueness.
/// Uniqueness is checked within the appropriate declaration namespace.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DistributedId(String);

impl DistributedId {
    pub fn new(value: impl Into<String>) -> Result<Self, ModelError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(ModelError::EmptyDistributedId);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DistributedId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Optional adapter-friendly provenance.
///
/// `node_id` is deliberately opaque so this module does not define a second
/// AST-node identity system. The AST adapter should preserve canonical identity
/// and source spans through the repository's semantic provenance service.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SourceOrigin {
    pub source: Option<String>,
    pub node_id: Option<String>,
}

impl SourceOrigin {
    pub fn new(source: Option<String>, node_id: Option<String>) -> Self {
        Self { source, node_id }
    }
}

/// A portable expression for a count or size.
///
/// The expression is not restricted to a target's integer width. A downstream
/// stage may evaluate it only when its inputs and semantic rules permit that.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SymbolicExpr {
    Known(u128),
    Symbol(DistributedId),
    Add(Box<SymbolicExpr>, Box<SymbolicExpr>),
    Multiply(Box<SymbolicExpr>, Box<SymbolicExpr>),
    Unknown,
}

impl SymbolicExpr {
    pub fn known(value: u128) -> Self {
        Self::Known(value)
    }

    pub fn symbol(id: DistributedId) -> Self {
        Self::Symbol(id)
    }

    pub fn add(lhs: Self, rhs: Self) -> Self {
        Self::Add(Box::new(lhs), Box::new(rhs))
    }

    pub fn multiply(lhs: Self, rhs: Self) -> Self {
        Self::Multiply(Box::new(lhs), Box::new(rhs))
    }

    /// Evaluate without recursive calls, avoiding stack growth proportional to
    /// expression depth. This evaluates only expressions whose values are known.
    pub fn evaluate(&self) -> SymbolicValue {
        enum Frame<'a> {
            Visit(&'a SymbolicExpr),
            CombineAdd,
            CombineMultiply,
        }

        let mut frames = vec![Frame::Visit(self)];
        let mut values: Vec<SymbolicValue> = Vec::new();

        while let Some(frame) = frames.pop() {
            match frame {
                Frame::Visit(expr) => match expr {
                    Self::Known(value) => values.push(SymbolicValue::Known(*value)),
                    Self::Symbol(_) => values.push(SymbolicValue::Symbolic),
                    Self::Unknown => values.push(SymbolicValue::Unknown),
                    Self::Add(lhs, rhs) => {
                        frames.push(Frame::CombineAdd);
                        frames.push(Frame::Visit(rhs));
                        frames.push(Frame::Visit(lhs));
                    }
                    Self::Multiply(lhs, rhs) => {
                        frames.push(Frame::CombineMultiply);
                        frames.push(Frame::Visit(rhs));
                        frames.push(Frame::Visit(lhs));
                    }
                },
                Frame::CombineAdd | Frame::CombineMultiply => {
                    let rhs = match values.pop() {
                        Some(value) => value,
                        None => return SymbolicValue::Unknown,
                    };
                    let lhs = match values.pop() {
                        Some(value) => value,
                        None => return SymbolicValue::Unknown,
                    };

                    let combined = match frame {
                        Frame::CombineAdd => combine_values(lhs, rhs, u128::checked_add),
                        Frame::CombineMultiply => {
                            combine_values(lhs, rhs, u128::checked_mul)
                        }
                        Frame::Visit(_) => unreachable!("visit frames are handled above"),
                    };
                    values.push(combined);
                }
            }
        }

        if values.len() == 1 {
            values.pop().unwrap_or(SymbolicValue::Unknown)
        } else {
            SymbolicValue::Unknown
        }
    }
}

fn combine_values(
    lhs: SymbolicValue,
    rhs: SymbolicValue,
    operation: fn(u128, u128) -> Option<u128>,
) -> SymbolicValue {
    use SymbolicValue::*;

    match (lhs, rhs) {
        (Overflow, _) | (_, Overflow) => Overflow,
        (Known(a), Known(b)) => operation(a, b).map(Known).unwrap_or(Overflow),
        (Unknown, _) | (_, Unknown) => Unknown,
        (Symbolic, _) | (_, Symbolic) => Symbolic,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SymbolicValue {
    Known(u128),
    Symbolic,
    Unknown,
    Overflow,
}

/// Construction errors are kept separate from semantic diagnostics because
/// adapters may reject malformed model records before invoking validation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ModelError {
    EmptyIdentifierNamespace,
    EmptyIdentifierName,
    EmptyDistributedId,
}

impl fmt::Display for ModelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifierNamespace => {
                f.write_str("qualified identifier namespace must not be empty")
            }
            Self::EmptyIdentifierName => {
                f.write_str("qualified identifier name must not be empty")
            }
            Self::EmptyDistributedId => {
                f.write_str("distributed identifier must not be empty")
            }
        }
    }
}

impl std::error::Error for ModelError {}

/// How a declaration participates in the distributed computation.
///
/// These are structural roles, not a list of physical machine types.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ParticipantRole {
    Coordinator,
    Worker,
    Peer,
    Service,
}

/// A participant's semantic declaration.
///
/// `kind` and `attributes` are extensible metadata. They must be interpreted
/// through registered semantic services, never by guessing from their names.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Participant {
    pub id: DistributedId,
    pub role: ParticipantRole,
    pub kind: QualifiedId,
    pub cardinality: Option<SymbolicExpr>,
    pub required_capabilities: BTreeSet<QualifiedId>,
    pub attributes: BTreeMap<QualifiedId, String>,
    pub origin: SourceOrigin,
}

/// Message delivery intent.
///
/// These declarations describe program requirements, not guarantees supplied
/// by every possible network or runtime.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum DeliverySemantics {
    AtMostOnce,
    AtLeastOnce,
    ExactlyOnce,
    BestEffort,
    Custom,
}

/// Ordering requested for messages or operations.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum OrderingSemantics {
    Unordered,
    PerSender,
    Causal,
    Total,
    Custom,
}

/// Consistency contract. Domain-specific models are identified by a qualified
/// extension name so new models do not require changes to this enum.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConsistencyModel {
    Eventual,
    Strong,
    Causal,
    Session,
    Custom(QualifiedId),
}

/// A communication endpoint is a participant reference plus an optional
/// logical port/channel name.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Endpoint {
    pub participant: DistributedId,
    pub port: Option<QualifiedId>,
}

/// A declared communication channel or message-flow contract.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Communication {
    pub id: DistributedId,
    pub kind: QualifiedId,
    pub from: Endpoint,
    pub to: Endpoint,
    pub delivery: DeliverySemantics,
    pub ordering: OrderingSemantics,
    pub consistency: Option<ConsistencyModel>,
    pub required_capabilities: BTreeSet<QualifiedId>,
    pub effects: BTreeSet<QualifiedId>,
    pub attributes: BTreeMap<QualifiedId, String>,
    pub origin: SourceOrigin,
}

/// The meaning of a dependency edge is explicit.
///
/// A cycle is not inherently invalid in distributed systems. Cycles are
/// reported only when a registered semantic rule declares a cycle forbidden.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum DependencyKind {
    Data,
    Control,
    Ordering,
    Feedback,
    Custom,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Dependency {
    pub from: DistributedId,
    pub to: DistributedId,
    pub kind: DependencyKind,
    pub rule: Option<QualifiedId>,
    pub origin: SourceOrigin,
}

/// Declarative fault-handling intent. This module does not execute recovery.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FailurePolicy {
    Propagate,
    Retry { attempts: SymbolicExpr },
    Compensate { handler: QualifiedId },
    Isolate,
    Custom(QualifiedId),
}

/// A failure policy attached to a participant, communication, or operation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FailureHandling {
    pub subject: DistributedId,
    pub policy: FailurePolicy,
    pub required_capabilities: BTreeSet<QualifiedId>,
    pub effects: BTreeSet<QualifiedId>,
    pub origin: SourceOrigin,
}

/// Resource and execution requirements remain symbolic and portable.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DistributedRequirement {
    pub kind: QualifiedId,
    pub quantity: SymbolicExpr,
    pub unit: Option<QualifiedId>,
    pub required: bool,
    pub origin: SourceOrigin,
}

/// A whole distributed-domain input assembled from the canonical AST.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DistributedProgram {
    pub participants: Vec<Participant>,
    pub communications: Vec<Communication>,
    pub dependencies: Vec<Dependency>,
    pub failure_handling: Vec<FailureHandling>,
    pub requirements: Vec<DistributedRequirement>,
}

/// Diagnostic severity for the adapter to map to the canonical diagnostics API.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Severity {
    Warning,
    Error,
}

/// A local diagnostic record. The frontend adapter should translate this into
/// the shared diagnostic representation and attach canonical source spans.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DistributedIssue {
    pub severity: Severity,
    pub code: &'static str,
    pub subject: Option<DistributedId>,
    pub message: String,
    pub origin: SourceOrigin,
}

impl DistributedIssue {
    fn error(
        code: &'static str,
        subject: Option<DistributedId>,
        message: impl Into<String>,
        origin: SourceOrigin,
    ) -> Self {
        Self {
            severity: Severity::Error,
            code,
            subject,
            message: message.into(),
            origin,
        }
    }

    fn warning(
        code: &'static str,
        subject: Option<DistributedId>,
        message: impl Into<String>,
        origin: SourceOrigin,
    ) -> Self {
        Self {
            severity: Severity::Warning,
            code,
            subject,
            message: message.into(),
            origin,
        }
    }
}

/// Context result: unsupported or incomplete shared-service analysis must be
/// represented explicitly rather than being interpreted as success.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ContextStatus {
    Valid,
    Invalid { code: String, message: String },
    Incomplete { code: String, message: String },
}

/// Shared semantic-service adapter.
///
/// Implementations should delegate to the canonical symbol, type, effect,
/// capability, policy, and extension registries. Defaults deliberately return
/// `Incomplete`; absence of an integration must never mean "valid".
pub trait DistributedSemanticContext {
    fn validate_participant(&self, _participant: &Participant) -> ContextStatus {
        ContextStatus::Incomplete {
            code: "DIST-CONTEXT-PARTICIPANT".to_owned(),
            message: "participant metadata has not been checked by the shared semantic services"
                .to_owned(),
        }
    }

    fn validate_communication(&self, _communication: &Communication) -> ContextStatus {
        ContextStatus::Incomplete {
            code: "DIST-CONTEXT-COMMUNICATION".to_owned(),
            message: "communication semantics have not been checked by the shared semantic services"
                .to_owned(),
        }
    }

    fn validate_dependency(&self, _dependency: &Dependency) -> ContextStatus {
        ContextStatus::Valid
    }

    fn validate_failure_handling(&self, _handling: &FailureHandling) -> ContextStatus {
        ContextStatus::Incomplete {
            code: "DIST-CONTEXT-FAILURE".to_owned(),
            message: "failure policy has not been checked by the shared semantic services"
                .to_owned(),
        }
    }

    fn validate_requirement(&self, _requirement: &DistributedRequirement) -> ContextStatus {
        ContextStatus::Incomplete {
            code: "DIST-CONTEXT-RESOURCE".to_owned(),
            message: "resource requirement has not been checked by the shared semantic services"
                .to_owned(),
        }
    }

    /// Return `Some(true)` if this dependency is forbidden to participate in a
    /// cycle, `Some(false)` if cycles are permitted, or `None` if the rule is
    /// unknown and analysis is incomplete.
    fn dependency_cycle_is_forbidden(&self, dependency: &Dependency) -> Option<bool> {
        match dependency.kind {
            DependencyKind::Feedback => Some(false),
            DependencyKind::Data | DependencyKind::Control | DependencyKind::Ordering => {
                Some(true)
            }
            DependencyKind::Custom => {
                let _ = dependency;
                None
            }
        }
    }
}

/// Final result for distributed-domain validation.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DistributedValidationReport {
    pub issues: Vec<DistributedIssue>,
    pub complete: bool,
}

impl DistributedValidationReport {
    pub fn is_valid(&self) -> bool {
        self.complete && !self.issues.iter().any(|i| i.severity == Severity::Error)
    }

    pub fn has_errors(&self) -> bool {
        self.issues.iter().any(|i| i.severity == Severity::Error)
    }

    fn push_context(
        &mut self,
        subject: Option<DistributedId>,
        origin: SourceOrigin,
        status: ContextStatus,
    ) {
        match status {
            ContextStatus::Valid => {}
            ContextStatus::Invalid { code, message } => {
                self.issues.push(DistributedIssue::error(
                    "DIST-CONTEXT-INVALID",
                    subject,
                    format!("{code}: {message}"),
                    origin,
                ));
            }
            ContextStatus::Incomplete { code, message } => {
                self.complete = false;
                self.issues.push(DistributedIssue::error(
                    "DIST-ANALYSIS-INCOMPLETE",
                    subject,
                    format!("{code}: {message}"),
                    origin,
                ));
            }
        }
    }

    fn finish(&mut self) {
        self.issues.sort_by(|a, b| {
            a.subject
                .cmp(&b.subject)
                .then_with(|| a.code.cmp(b.code))
                .then_with(|| a.severity.cmp(&b.severity))
                .then_with(|| a.message.cmp(&b.message))
                .then_with(|| a.origin.source.cmp(&b.origin.source))
                .then_with(|| a.origin.node_id.cmp(&b.origin.node_id))
        });
    }
}

/// Validate distributed-domain declarations and their references.
///
/// This function does not decide whether a specific deployment can satisfy
/// the declared requirements. That determination belongs to downstream target
/// capability and feasibility analysis.
pub fn validate_distributed<C: DistributedSemanticContext>(
    program: &DistributedProgram,
    context: &C,
) -> DistributedValidationReport {
    let mut report = DistributedValidationReport {
        issues: Vec::new(),
        complete: true,
    };

    let participants = unique_participants(program, &mut report);
    let communications = unique_communications(program, &mut report);

    for participant in &program.participants {
        report.push_context(
            Some(participant.id.clone()),
            participant.origin.clone(),
            context.validate_participant(participant),
        );

        if let Some(cardinality) = &participant.cardinality {
            validate_symbolic_expr(
                cardinality,
                Some(participant.id.clone()),
                participant.origin.clone(),
                &mut report,
            );
        }
    }

    for communication in &program.communications {
        validate_endpoint(
            &communication.from,
            &participants,
            &communication.id,
            "source",
            &communication.origin,
            &mut report,
        );
        validate_endpoint(
            &communication.to,
            &participants,
            &communication.id,
            "destination",
            &communication.origin,
            &mut report,
        );

        report.push_context(
            Some(communication.id.clone()),
            communication.origin.clone(),
            context.validate_communication(communication),
        );

        if communication.from.participant == communication.to.participant {
            report.issues.push(DistributedIssue::warning(
                "DIST-SELF-COMMUNICATION",
                Some(communication.id.clone()),
                "communication has the same source and destination participant",
                communication.origin.clone(),
            ));
        }

        if communication.delivery == DeliverySemantics::ExactlyOnce {
            // Exactly-once intent is permitted. The shared service must define
            // the contract and the downstream runtime must establish feasibility.
            // Do not pretend that a local semantic validator proves delivery.
            if communication.consistency.is_none() {
                report.issues.push(DistributedIssue::warning(
                    "DIST-EXACTLY-ONCE-CONTRACT",
                    Some(communication.id.clone()),
                    "exactly-once delivery is declared without an explicit consistency contract",
                    communication.origin.clone(),
                ));
            }
        }
    }

    for dependency in &program.dependencies {
        let from_exists = communications.contains_key(&dependency.from)
            || participants.contains_key(&dependency.from);
        let to_exists = communications.contains_key(&dependency.to)
            || participants.contains_key(&dependency.to);

        if !from_exists {
            report.issues.push(DistributedIssue::error(
                "DIST-DEPENDENCY-SOURCE",
                Some(dependency.from.clone()),
                format!("dependency source '{}' is not declared", dependency.from),
                dependency.origin.clone(),
            ));
        }
        if !to_exists {
            report.issues.push(DistributedIssue::error(
                "DIST-DEPENDENCY-TARGET",
                Some(dependency.to.clone()),
                format!("dependency target '{}' is not declared", dependency.to),
                dependency.origin.clone(),
            ));
        }

        report.push_context(
            Some(dependency.from.clone()),
            dependency.origin.clone(),
            context.validate_dependency(dependency),
        );
    }

    validate_dependency_cycles(program, context, &mut report);

    for handling in &program.failure_handling {
        let subject_exists = participants.contains_key(&handling.subject)
            || communications.contains_key(&handling.subject);

        if !subject_exists {
            report.issues.push(DistributedIssue::error(
                "DIST-FAILURE-SUBJECT",
                Some(handling.subject.clone()),
                format!(
                    "failure-handling subject '{}' is not a declared participant or communication",
                    handling.subject
                ),
                handling.origin.clone(),
            ));
        }

        if let FailurePolicy::Retry { attempts } = &handling.policy {
            validate_symbolic_expr(
                attempts,
                Some(handling.subject.clone()),
                handling.origin.clone(),
                &mut report,
            );

            match attempts.evaluate() {
                SymbolicValue::Known(0) => {
                    report.issues.push(DistributedIssue::error(
                        "DIST-RETRY-ZERO",
                        Some(handling.subject.clone()),
                        "retry attempt count must not be zero",
                        handling.origin.clone(),
                    ));
                }
                SymbolicValue::Overflow => {
                    report.issues.push(DistributedIssue::error(
                        "DIST-RETRY-OVERFLOW",
                        Some(handling.subject.clone()),
                        "retry attempt expression overflows the supported checked evaluation domain",
                        handling.origin.clone(),
                    ));
                }
                SymbolicValue::Known(_) | SymbolicValue::Symbolic | SymbolicValue::Unknown => {}
            }
        }

        report.push_context(
            Some(handling.subject.clone()),
            handling.origin.clone(),
            context.validate_failure_handling(handling),
        );
    }

    for requirement in &program.requirements {
        validate_symbolic_expr(
            &requirement.quantity,
            None,
            requirement.origin.clone(),
            &mut report,
        );
        report.push_context(
            None,
            requirement.origin.clone(),
            context.validate_requirement(requirement),
        );
    }

    report.finish();
    report
}

fn unique_participants<'a>(
    program: &'a DistributedProgram,
    report: &mut DistributedValidationReport,
) -> BTreeMap<DistributedId, &'a Participant> {
    let mut result = BTreeMap::new();

    for participant in &program.participants {
        if result.contains_key(&participant.id) {
            report.issues.push(DistributedIssue::error(
                "DIST-DUPLICATE-PARTICIPANT",
                Some(participant.id.clone()),
                format!("participant '{}' is declared more than once", participant.id),
                participant.origin.clone(),
            ));
        } else {
            result.insert(participant.id.clone(), participant);
        }
    }

    result
}

fn unique_communications<'a>(
    program: &'a DistributedProgram,
    report: &mut DistributedValidationReport,
) -> BTreeMap<DistributedId, &'a Communication> {
    let mut result = BTreeMap::new();

    for communication in &program.communications {
        if result.contains_key(&communication.id) {
            report.issues.push(DistributedIssue::error(
                "DIST-DUPLICATE-COMMUNICATION",
                Some(communication.id.clone()),
                format!("communication '{}' is declared more than once", communication.id),
                communication.origin.clone(),
            ));
        } else {
            result.insert(communication.id.clone(), communication);
        }
    }

    result
}

fn validate_endpoint(
    endpoint: &Endpoint,
    participants: &BTreeMap<DistributedId, &Participant>,
    communication_id: &DistributedId,
    endpoint_label: &str,
    origin: &SourceOrigin,
    report: &mut DistributedValidationReport,
) {
    if !participants.contains_key(&endpoint.participant) {
        report.issues.push(DistributedIssue::error(
            "DIST-UNKNOWN-ENDPOINT",
            Some(communication_id.clone()),
            format!(
                "communication '{}' has an undeclared {endpoint_label} participant '{}'",
                communication_id, endpoint.participant
            ),
            origin.clone(),
        ));
    }
}

fn validate_symbolic_expr(
    expression: &SymbolicExpr,
    subject: Option<DistributedId>,
    origin: SourceOrigin,
    report: &mut DistributedValidationReport,
) {
    if expression.evaluate() == SymbolicValue::Overflow {
        report.issues.push(DistributedIssue::error(
            "DIST-SYMBOLIC-OVERFLOW",
            subject,
            "symbolic arithmetic overflows during checked constant evaluation",
            origin,
        ));
    }
}

fn validate_dependency_cycles<C: DistributedSemanticContext>(
    program: &DistributedProgram,
    context: &C,
    report: &mut DistributedValidationReport,
) {
    // Only edges explicitly classified as forbidden-to-cycle are included.
    // Unknown custom cycle policy makes the result incomplete.
    let mut adjacency: BTreeMap<DistributedId, BTreeSet<DistributedId>> = BTreeMap::new();
    let mut indegree: BTreeMap<DistributedId, usize> = BTreeMap::new();
    let mut edge_origin: BTreeMap<(DistributedId, DistributedId), SourceOrigin> =
        BTreeMap::new();

    for participant in &program.participants {
        indegree.entry(participant.id.clone()).or_insert(0);
        adjacency.entry(participant.id.clone()).or_default();
    }
    for communication in &program.communications {
        indegree.entry(communication.id.clone()).or_insert(0);
        adjacency.entry(communication.id.clone()).or_default();
    }

    for dependency in &program.dependencies {
        let forbidden = match context.dependency_cycle_is_forbidden(dependency) {
            Some(value) => value,
            None => {
                report.complete = false;
                report.issues.push(DistributedIssue::error(
                    "DIST-CYCLE-POLICY-UNKNOWN",
                    Some(dependency.from.clone()),
                    "cycle policy for this custom dependency kind is not registered",
                    dependency.origin.clone(),
                ));
                continue;
            }
        };

        if !forbidden {
            continue;
        }

        let source_exists = indegree.contains_key(&dependency.from);
        let target_exists = indegree.contains_key(&dependency.to);
        if !source_exists || !target_exists {
            continue; // Endpoint diagnostics were emitted by the main pass.
        }

        let outgoing = adjacency.entry(dependency.from.clone()).or_default();
        if outgoing.insert(dependency.to.clone()) {
            if let Some(count) = indegree.get_mut(&dependency.to) {
                match count.checked_add(1) {
                    Some(next) => *count = next,
                    None => {
                        report.complete = false;
                        report.issues.push(DistributedIssue::error(
                            "DIST-GRAPH-OVERFLOW",
                            Some(dependency.to.clone()),
                            "dependency graph indegree overflowed",
                            dependency.origin.clone(),
                        ));
                    }
                }
            }
        }

        edge_origin
            .entry((dependency.from.clone(), dependency.to.clone()))
            .or_insert_with(|| dependency.origin.clone());
    }

    // Kahn's algorithm is iterative and uses ordered collections for
    // deterministic traversal. Cycles in feedback or custom-permitted edges
    // do not enter this graph.
    let mut ready: BTreeSet<DistributedId> = indegree
        .iter()
        .filter_map(|(id, degree)| (*degree == 0).then_some(id.clone()))
        .collect();
    let mut processed = 0usize;

    while let Some(next) = ready.pop_first() {
        processed = match processed.checked_add(1) {
            Some(value) => value,
            None => {
                report.complete = false;
                report.issues.push(DistributedIssue::error(
                    "DIST-GRAPH-OVERFLOW",
                    Some(next.clone()),
                    "dependency graph traversal count overflowed",
                    SourceOrigin::default(),
                ));
                return;
            }
        };

        if let Some(targets) = adjacency.get(&next) {
            for target in targets {
                if let Some(degree) = indegree.get_mut(target) {
                    *degree = degree.saturating_sub(1);
                    if *degree == 0 {
                        ready.insert(target.clone());
                    }
                }
            }
        }
    }

    if processed != indegree.len() {
        let cyclic: Vec<String> = indegree
            .iter()
            .filter_map(|(id, degree)| (*degree > 0).then_some(id.to_string()))
            .collect();

        let origin = edge_origin
            .iter()
            .find_map(|((from, to), origin)| {
                let from_cyclic = indegree.get(from).copied().unwrap_or(0) > 0;
                let to_cyclic = indegree.get(to).copied().unwrap_or(0) > 0;
                (from_cyclic && to_cyclic).then_some(origin.clone())
            })
            .unwrap_or_default();

        report.issues.push(DistributedIssue::error(
            "DIST-DEPENDENCY-CYCLE",
            None,
            format!(
                "dependency cycle violates the registered ordering rule; unresolved nodes: {}",
                cyclic.join(", ")
            ),
            origin,
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct CompleteContext;

    impl DistributedSemanticContext for CompleteContext {
        fn validate_participant(&self, _: &Participant) -> ContextStatus {
            ContextStatus::Valid
        }

        fn validate_communication(&self, _: &Communication) -> ContextStatus {
            ContextStatus::Valid
        }

        fn validate_failure_handling(&self, _: &FailureHandling) -> ContextStatus {
            ContextStatus::Valid
        }

        fn validate_requirement(&self, _: &DistributedRequirement) -> ContextStatus {
            ContextStatus::Valid
        }
    }

    struct IncompleteContext;

    impl DistributedSemanticContext for IncompleteContext {}

    fn qid(namespace: &str, name: &str) -> QualifiedId {
        QualifiedId::new(namespace, name).expect("valid test identifier")
    }

    fn id(value: &str) -> DistributedId {
        DistributedId::new(value).expect("valid test id")
    }

    fn participant(value: &str) -> Participant {
        Participant {
            id: id(value),
            role: ParticipantRole::Worker,
            kind: qid("zamani.distributed", "participant"),
            cardinality: None,
            required_capabilities: BTreeSet::new(),
            attributes: BTreeMap::new(),
            origin: SourceOrigin::default(),
        }
    }

    fn communication(value: &str, from: &str, to: &str) -> Communication {
        Communication {
            id: id(value),
            kind: qid("zamani.distributed", "message-flow"),
            from: Endpoint {
                participant: id(from),
                port: None,
            },
            to: Endpoint {
                participant: id(to),
                port: None,
            },
            delivery: DeliverySemantics::AtLeastOnce,
            ordering: OrderingSemantics::Causal,
            consistency: Some(ConsistencyModel::Causal),
            required_capabilities: BTreeSet::new(),
            effects: BTreeSet::new(),
            attributes: BTreeMap::new(),
            origin: SourceOrigin::default(),
        }
    }

    #[test]
    fn identifiers_reject_empty_values() {
        assert!(DistributedId::new("  ").is_err());
        assert!(QualifiedId::new("domain", "").is_err());
        assert!(QualifiedId::new("", "operation").is_err());
    }

    #[test]
    fn symbolic_expressions_preserve_symbolic_values() {
        let expr = SymbolicExpr::add(
            SymbolicExpr::Known(4),
            SymbolicExpr::symbol(id("worker_count")),
        );
        assert_eq!(expr.evaluate(), SymbolicValue::Symbolic);
    }

    #[test]
    fn symbolic_arithmetic_checks_overflow() {
        let expr = SymbolicExpr::add(
            SymbolicExpr::Known(u128::MAX),
            SymbolicExpr::Known(1),
        );
        assert_eq!(expr.evaluate(), SymbolicValue::Overflow);
    }

    #[test]
    fn empty_program_is_valid_only_when_context_is_not_needed() {
        let report = validate_distributed(&DistributedProgram::default(), &CompleteContext);
        assert!(report.is_valid());
        assert!(report.complete);
    }

    #[test]
    fn unknown_endpoint_is_reported() {
        let program = DistributedProgram {
            participants: vec![participant("worker-a")],
            communications: vec![communication("flow-1", "worker-a", "missing-worker")],
            ..DistributedProgram::default()
        };

        let report = validate_distributed(&program, &CompleteContext);
        assert!(report.has_errors());
        assert!(report.issues.iter().any(|issue| issue.code == "DIST-UNKNOWN-ENDPOINT"));
    }

    #[test]
    fn duplicate_participants_are_reported() {
        let program = DistributedProgram {
            participants: vec![participant("worker-a"), participant("worker-a")],
            ..DistributedProgram::default()
        };

        let report = validate_distributed(&program, &CompleteContext);
        assert!(report
            .issues
            .iter()
            .any(|issue| issue.code == "DIST-DUPLICATE-PARTICIPANT"));
    }

    #[test]
    fn forbidden_dependency_cycle_is_reported() {
        let program = DistributedProgram {
            participants: vec![participant("a"), participant("b")],
            dependencies: vec![
                Dependency {
                    from: id("a"),
                    to: id("b"),
                    kind: DependencyKind::Ordering,
                    rule: None,
                    origin: SourceOrigin::default(),
                },
                Dependency {
                    from: id("b"),
                    to: id("a"),
                    kind: DependencyKind::Ordering,
                    rule: None,
                    origin: SourceOrigin::default(),
                },
            ],
            ..DistributedProgram::default()
        };

        let report = validate_distributed(&program, &CompleteContext);
        assert!(report
            .issues
            .iter()
            .any(|issue| issue.code == "DIST-DEPENDENCY-CYCLE"));
    }

    #[test]
    fn feedback_cycle_is_not_rejected_as_ordering_cycle() {
        let program = DistributedProgram {
            participants: vec![participant("a"), participant("b")],
            dependencies: vec![
                Dependency {
                    from: id("a"),
                    to: id("b"),
                    kind: DependencyKind::Feedback,
                    rule: None,
                    origin: SourceOrigin::default(),
                },
                Dependency {
                    from: id("b"),
                    to: id("a"),
                    kind: DependencyKind::Feedback,
                    rule: None,
                    origin: SourceOrigin::default(),
                },
            ],
            ..DistributedProgram::default()
        };

        let report = validate_distributed(&program, &CompleteContext);
        assert!(!report
            .issues
            .iter()
            .any(|issue| issue.code == "DIST-DEPENDENCY-CYCLE"));
    }

    #[test]
    fn missing_shared_service_integration_is_incomplete_not_valid() {
        let program = DistributedProgram {
            participants: vec![participant("worker-a")],
            ..DistributedProgram::default()
        };

        let report = validate_distributed(&program, &IncompleteContext);
        assert!(!report.is_valid());
        assert!(!report.complete);
        assert!(report
            .issues
            .iter()
            .any(|issue| issue.code == "DIST-ANALYSIS-INCOMPLETE"));
    }

    #[test]
    fn diagnostics_have_stable_order() {
        let program = DistributedProgram {
            participants: vec![participant("worker-a")],
            communications: vec![
                communication("flow-z", "worker-a", "missing-z"),
                communication("flow-a", "worker-a", "missing-a"),
            ],
            ..DistributedProgram::default()
        };

        let first = validate_distributed(&program, &CompleteContext);
        let second = validate_distributed(&program, &CompleteContext);
        assert_eq!(first, second);
    }

    #[test]
    fn zero_retry_attempts_are_rejected() {
        let program = DistributedProgram {
            participants: vec![participant("worker-a")],
            failure_handling: vec![FailureHandling {
                subject: id("worker-a"),
                policy: FailurePolicy::Retry {
                    attempts: SymbolicExpr::Known(0),
                },
                required_capabilities: BTreeSet::new(),
                effects: BTreeSet::new(),
                origin: SourceOrigin::default(),
            }],
            ..DistributedProgram::default()
        };

        let report = validate_distributed(&program, &CompleteContext);
        assert!(report
            .issues
            .iter()
            .any(|issue| issue.code == "DIST-RETRY-ZERO"));
    }
}

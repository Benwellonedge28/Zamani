
/*!
PURPOSE
    Validate concurrency-related semantic facts for Zamani programs.

STATUS
    Proposed production implementation. Freeze only after the canonical AST,
    semantic model, effects, capabilities, diagnostics, and configuration
    adapters have been checked against the repository.

AUTHORITY
    The normative Zamani language specification, canonical AST, and shared
    frontend semantic contracts take precedence over this module.

OWNS
    - Target-independent concurrency facts and their structural validation.
    - Task/actor identity within a concurrency analysis projection.
    - Dependency-edge validation and non-feedback cycle detection.
    - Synchronization declaration and participant-reference validation.
    - Conservative detection of potentially conflicting shared accesses.
    - Stable, machine-readable concurrency diagnostics.

DOES_NOT_OWN
    - The canonical AST or a replacement AST.
    - Name resolution, canonical type inference, or effect inference.
    - Runtime scheduling, thread assignment, placement, or device allocation.
    - Runtime lock implementation, distributed consensus, or deadlock freedom
      beyond the facts explicitly represented here.
    - Classical, quantum, HDL, or other domain IR construction.
    - Proof of race freedom when synchronization or alias information is
      incomplete.

INPUTS
    A semantic projection derived from canonical AST nodes and shared semantic
    facts. Node IDs must retain a traceable mapping to canonical AST identities.

OUTPUTS
    A deterministic validation report containing diagnostics and an explicit
    completeness status.

DEPENDENCIES
    Rust standard library only. Integration adapters should use the repository's
    canonical IDs, source spans, diagnostics, effect registry, and capability
    registry when their exact interfaces are verified.

SCALABILITY
    No fixed task, actor, resource, edge, or synchronization count is imposed by
    the language model. Runtime memory and time remain bounded by available
    resources and the caller's explicit analysis budget.

DETERMINISM
    Ordered maps/sets and stable diagnostic sorting make output independent of
    hash-map iteration order.

ERROR HANDLING
    Invalid or incomplete input never produces a report claiming complete,
    successful validation. Structural errors are diagnostics, not panics.

INTEGRATION CONTRACT
    The AST adapter constructs the projection from canonical AST identities.
    The shared semantic context validates operation declarations, effects,
    capabilities, and source-level references. The pipeline consumes the
    report and merges diagnostics into the canonical semantic result.

FREEZE CRITERIA
    - Repository-specific adapter compiles against the actual AST.
    - Shared effect/capability checks are connected.
    - Unit, integration, determinism, fuzz, and stress tests pass.
    - Workspace tests and lint checks pass on the supported Rust toolchain.
*/

#![forbid(unsafe_code)]

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt;

/// Stable identity used inside a concurrency analysis projection.
///
/// The AST adapter must map this identity to a canonical AST node identity.
/// This type is not intended to replace the repository's AST NodeId.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ConcurrencyNodeId(String);

impl ConcurrencyNodeId {
    pub fn new(value: impl Into<String>) -> Result<Self, ModelError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(ModelError::EmptyIdentifier("concurrency node"));
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ConcurrencyNodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Identity of a shared location, channel, lock, barrier, or other resource.
///
/// The precise resource category is determined by the resource declaration.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ConcurrencyResourceId(String);

impl ConcurrencyResourceId {
    pub fn new(value: impl Into<String>) -> Result<Self, ModelError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(ModelError::EmptyIdentifier("concurrency resource"));
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ConcurrencyResourceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Qualified semantic identifier.
///
/// Qualified names allow new concurrency operations and effects to be added
/// through the shared registries without modifying this module's enums.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct QualifiedName {
    namespace: String,
    name: String,
}

impl QualifiedName {
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Self, ModelError> {
        let namespace = namespace.into();
        let name = name.into();

        if namespace.trim().is_empty() {
            return Err(ModelError::EmptyIdentifier("qualified-name namespace"));
        }
        if name.trim().is_empty() {
            return Err(ModelError::EmptyIdentifier("qualified name"));
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

impl fmt::Display for QualifiedName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}::{}", self.namespace, self.name)
    }
}

/// Source provenance that an AST adapter can populate.
///
/// The strings are adapter-neutral placeholders, not replacements for the
/// repository's canonical source span or AST identity types.
#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct SourceOrigin {
    pub source: Option<String>,
    pub ast_node: Option<String>,
}

/// Construction-time model errors.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ModelError {
    EmptyIdentifier(&'static str),
}

impl fmt::Display for ModelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifier(kind) => write!(f, "{kind} must not be empty"),
        }
    }
}

impl std::error::Error for ModelError {}

/// A source-level concurrency operation.
///
/// `operation` is extensible and must be resolved through the shared semantic
/// registry. The enum deliberately does not enumerate every possible operation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConcurrentOperation {
    pub operation: QualifiedName,
    pub origin: SourceOrigin,
}

/// Semantic kind of an execution entity.
///
/// `Custom` supports language extensions without requiring a new core variant.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExecutionEntityKind {
    Task,
    Actor,
    AsyncComputation,
    ParallelRegion,
    CriticalRegion,
    Custom(QualifiedName),
}

/// Access mode used by conservative shared-state analysis.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum AccessMode {
    Read,
    Write,
    AtomicRead,
    AtomicWrite,
}

impl AccessMode {
    fn is_write(self) -> bool {
        matches!(self, Self::Write | Self::AtomicWrite)
    }

    fn is_atomic(self) -> bool {
        matches!(self, Self::AtomicRead | Self::AtomicWrite)
    }
}

/// A shared-state access attributed to a semantic execution entity.
///
/// `protected_by` lists synchronization declarations that are known, from
/// semantic analysis, to protect this access. Merely naming a lock in source
/// syntax is insufficient: the AST adapter must populate this field only when
/// the shared semantic checks establish the relevant protection relationship.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SharedAccess {
    pub resource: ConcurrencyResourceId,
    pub mode: AccessMode,
    pub protected_by: BTreeSet<ConcurrencyResourceId>,
    pub origin: SourceOrigin,
}

/// An execution entity projected from the canonical AST.
///
/// `effects` and `required_capabilities` are references to the shared semantic
/// registries, not local definitions of their meaning.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionEntity {
    pub id: ConcurrencyNodeId,
    pub kind: ExecutionEntityKind,
    pub operation: ConcurrentOperation,
    pub effects: BTreeSet<QualifiedName>,
    pub required_capabilities: BTreeSet<QualifiedName>,
    pub accesses: Vec<SharedAccess>,
    pub origin: SourceOrigin,
}

/// Dependency relation between execution entities.
///
/// Feedback edges represent intentionally cyclic relationships, such as a
/// feedback process. They are excluded from ordinary acyclic dependency checks.
/// This exclusion does not prove that a feedback system is well-founded or
/// schedulable; the domain rules and downstream execution model must decide that.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum DependencyKind {
    Data,
    Control,
    Ordering,
    Synchronization,
    Feedback,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Dependency {
    pub from: ConcurrencyNodeId,
    pub to: ConcurrencyNodeId,
    pub kind: DependencyKind,
    pub origin: SourceOrigin,
}

/// Synchronization semantics relevant to static validation.
///
/// These are semantic categories, not concrete runtime implementations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SynchronizationKind {
    /// Mutual exclusion: participants using this synchronization identity
    /// are treated as mutually exclusive while inside the protected region.
    Exclusive,

    /// Read/write exclusion: read-only critical sections may overlap, while
    /// a writer excludes other readers and writers.
    SharedExclusive,

    /// Atomic coordination. Ordinary non-atomic accesses are not protected
    /// merely because they name an atomic resource.
    Atomic,

    /// Message-passing endpoint. Message order and ownership semantics require
    /// additional rules from the relevant language extension.
    MessagePassing,

    /// A collective synchronization point. Participation and control-flow
    /// correctness require domain-specific validation.
    Barrier,

    /// An extension-defined synchronization form.
    Custom(QualifiedName),
}

/// Synchronization declaration projected from semantic facts.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Synchronization {
    pub id: ConcurrencyResourceId,
    pub kind: SynchronizationKind,
    pub participants: BTreeSet<ConcurrencyNodeId>,
    pub origin: SourceOrigin,
}

/// A semantic concurrency projection.
///
/// This structure is not a replacement AST or an execution IR. It is a compact
/// input to concurrency validation, populated from canonical AST nodes and
/// shared semantic facts by the frontend adapter.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ConcurrencyProgram {
    pub entities: Vec<ExecutionEntity>,
    pub dependencies: Vec<Dependency>,
    pub synchronizations: Vec<Synchronization>,
}

/// Analysis completeness is separate from whether errors were found.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AnalysisCompleteness {
    Complete,
    Incomplete,
}

/// Severity of a concurrency diagnostic.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum DiagnosticSeverity {
    Note,
    Warning,
    Error,
}

/// Stable diagnostic codes owned by this module.
///
/// In repository integration, map these codes to the central diagnostic-code
/// registry rather than maintaining two public code registries.
pub mod diagnostic_codes {
    pub const DUPLICATE_ENTITY: &str = "CONCURRENCY001";
    pub const DUPLICATE_SYNCHRONIZATION: &str = "CONCURRENCY002";
    pub const UNKNOWN_DEPENDENCY_ENDPOINT: &str = "CONCURRENCY003";
    pub const DEPENDENCY_CYCLE: &str = "CONCURRENCY004";
    pub const UNKNOWN_SYNCHRONIZATION_PARTICIPANT: &str = "CONCURRENCY005";
    pub const UNKNOWN_PROTECTION: &str = "CONCURRENCY006";
    pub const POTENTIAL_DATA_RACE: &str = "CONCURRENCY007";
    pub const UNKNOWN_OPERATION: &str = "CONCURRENCY008";
    pub const UNKNOWN_EFFECT: &str = "CONCURRENCY009";
    pub const UNKNOWN_CAPABILITY: &str = "CONCURRENCY010";
    pub const CONTEXT_FAILURE: &str = "CONCURRENCY011";
    pub const DUPLICATE_DEPENDENCY: &str = "CONCURRENCY012";
}

/// Structured diagnostic.
///
/// `subject` and `related` are semantic IDs. Repository integration should
/// translate them to canonical diagnostic labels and source spans.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConcurrencyDiagnostic {
    pub code: &'static str,
    pub severity: DiagnosticSeverity,
    pub subject: Option<String>,
    pub related: Option<String>,
    pub message: String,
    pub origin: SourceOrigin,
}

impl ConcurrencyDiagnostic {
    fn new(
        code: &'static str,
        severity: DiagnosticSeverity,
        subject: Option<String>,
        related: Option<String>,
        message: impl Into<String>,
        origin: SourceOrigin,
    ) -> Self {
        Self {
            code,
            severity,
            subject,
            related,
            message: message.into(),
            origin,
        }
    }
}

/// Result of one concurrency validation pass.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConcurrencyReport {
    pub completeness: AnalysisCompleteness,
    pub diagnostics: Vec<ConcurrencyDiagnostic>,
}

impl ConcurrencyReport {
    pub fn is_valid(&self) -> bool {
        self.completeness == AnalysisCompleteness::Complete
            && !self
                .diagnostics
                .iter()
                .any(|d| d.severity == DiagnosticSeverity::Error)
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| d.severity == DiagnosticSeverity::Error)
    }
}

/// Error returned by the shared semantic context.
///
/// `complete = false` means the checker could not establish a conclusion.
/// Such a failure must not be treated as successful validation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContextIssue {
    pub code: String,
    pub message: String,
    pub complete: bool,
}

/// Shared semantic services required by concurrency validation.
///
/// The production adapter must implement these methods by delegating to
/// canonical operation, effect, capability, and reference-resolution services.
/// It must not implement an independent type or effect system here.
pub trait ConcurrencySemanticContext {
    fn validate_operation(
        &self,
        entity: &ExecutionEntity,
    ) -> Result<(), ContextIssue>;

    fn validate_effect(
        &self,
        entity: &ExecutionEntity,
        effect: &QualifiedName,
    ) -> Result<(), ContextIssue>;

    fn validate_capability(
        &self,
        entity: &ExecutionEntity,
        capability: &QualifiedName,
    ) -> Result<(), ContextIssue>;
}

/// A context useful for structural-only validation and tests.
///
/// It does not certify operations, effects, or capabilities. Use it only when
/// the caller intentionally wants structural checks without registry checks.
pub struct StructuralOnlyContext;

impl ConcurrencySemanticContext for StructuralOnlyContext {
    fn validate_operation(
        &self,
        _entity: &ExecutionEntity,
    ) -> Result<(), ContextIssue> {
        Ok(())
    }

    fn validate_effect(
        &self,
        _entity: &ExecutionEntity,
        _effect: &QualifiedName,
    ) -> Result<(), ContextIssue> {
        Ok(())
    }

    fn validate_capability(
        &self,
        _entity: &ExecutionEntity,
        _capability: &QualifiedName,
    ) -> Result<(), ContextIssue> {
        Ok(())
    }
}

/// Validate a concurrency projection against shared semantic services.
///
/// No target-specific scheduling or placement decisions are performed.
pub fn validate_concurrency(
    program: &ConcurrencyProgram,
    context: &impl ConcurrencySemanticContext,
) -> ConcurrencyReport {
    let mut state = ValidationState::default();

    let entity_index = index_entities(program, &mut state);
    let sync_index = index_synchronizations(program, &mut state);

    validate_entity_metadata(program, context, &mut state);
    validate_dependencies(program, &entity_index, &mut state);
    validate_synchronization_participants(
        program,
        &entity_index,
        &sync_index,
        &mut state,
    );
    validate_access_protection(
        program,
        &entity_index,
        &sync_index,
        &mut state,
    );
    validate_potential_races(
        program,
        &entity_index,
        &sync_index,
        &mut state,
    );

    state.finish()
}

#[derive(Default)]
struct ValidationState {
    diagnostics: Vec<ConcurrencyDiagnostic>,
    completeness: AnalysisCompleteness,
}

impl Default for AnalysisCompleteness {
    fn default() -> Self {
        Self::Complete
    }
}

impl ValidationState {
    fn error(
        &mut self,
        code: &'static str,
        subject: Option<String>,
        related: Option<String>,
        message: impl Into<String>,
        origin: SourceOrigin,
    ) {
        self.diagnostics.push(ConcurrencyDiagnostic::new(
            code,
            DiagnosticSeverity::Error,
            subject,
            related,
            message,
            origin,
        ));
    }

    fn warning(
        &mut self,
        code: &'static str,
        subject: Option<String>,
        related: Option<String>,
        message: impl Into<String>,
        origin: SourceOrigin,
    ) {
        self.diagnostics.push(ConcurrencyDiagnostic::new(
            code,
            DiagnosticSeverity::Warning,
            subject,
            related,
            message,
            origin,
        ));
    }

    fn context_failure(
        &mut self,
        entity: &ExecutionEntity,
        context_issue: ContextIssue,
        fallback_code: &'static str,
    ) {
        if !context_issue.complete {
            self.completeness = AnalysisCompleteness::Incomplete;
        }

        let code = match context_issue.code.as_str() {
            "unknown-operation" => diagnostic_codes::UNKNOWN_OPERATION,
            "unknown-effect" => diagnostic_codes::UNKNOWN_EFFECT,
            "unknown-capability" => diagnostic_codes::UNKNOWN_CAPABILITY,
            _ => fallback_code,
        };

        self.error(
            code,
            Some(entity.id.to_string()),
            None,
            context_issue.message,
            entity.origin.clone(),
        );
    }

    fn finish(mut self) -> ConcurrencyReport {
        self.diagnostics.sort_by(|a, b| {
            a.subject
                .cmp(&b.subject)
                .then_with(|| a.code.cmp(b.code))
                .then_with(|| a.related.cmp(&b.related))
                .then_with(|| a.severity.cmp(&b.severity))
                .then_with(|| a.message.cmp(&b.message))
                .then_with(|| a.origin.cmp(&b.origin))
        });

        self.diagnostics.dedup();

        ConcurrencyReport {
            completeness: self.completeness,
            diagnostics: self.diagnostics,
        }
    }
}

fn index_entities<'a>(
    program: &'a ConcurrencyProgram,
    state: &mut ValidationState,
) -> BTreeMap<ConcurrencyNodeId, &'a ExecutionEntity> {
    let mut index = BTreeMap::new();

    for entity in &program.entities {
        if index.contains_key(&entity.id) {
            state.error(
                diagnostic_codes::DUPLICATE_ENTITY,
                Some(entity.id.to_string()),
                None,
                format!("duplicate concurrency entity '{}'", entity.id),
                entity.origin.clone(),
            );
            continue;
        }
        index.insert(entity.id.clone(), entity);
    }

    index
}

fn index_synchronizations<'a>(
    program: &'a ConcurrencyProgram,
    state: &mut ValidationState,
) -> BTreeMap<ConcurrencyResourceId, &'a Synchronization> {
    let mut index = BTreeMap::new();

    for synchronization in &program.synchronizations {
        if index.contains_key(&synchronization.id) {
            state.error(
                diagnostic_codes::DUPLICATE_SYNCHRONIZATION,
                Some(synchronization.id.to_string()),
                None,
                format!(
                    "duplicate synchronization declaration '{}'",
                    synchronization.id
                ),
                synchronization.origin.clone(),
            );
            continue;
        }
        index.insert(synchronization.id.clone(), synchronization);
    }

    index
}

fn validate_entity_metadata(
    program: &ConcurrencyProgram,
    context: &impl ConcurrencySemanticContext,
    state: &mut ValidationState,
) {
    for entity in &program.entities {
        if let Err(issue) = context.validate_operation(entity) {
            state.context_failure(
                entity,
                issue,
                diagnostic_codes::CONTEXT_FAILURE,
            );
        }

        for effect in &entity.effects {
            if let Err(issue) = context.validate_effect(entity, effect) {
                state.context_failure(
                    entity,
                    issue,
                    diagnostic_codes::CONTEXT_FAILURE,
                );
            }
        }

        for capability in &entity.required_capabilities {
            if let Err(issue) = context.validate_capability(entity, capability) {
                state.context_failure(
                    entity,
                    issue,
                    diagnostic_codes::CONTEXT_FAILURE,
                );
            }
        }
    }

    // This intentionally does not compare capability requirements against
    // physical hardware. That is a downstream feasibility responsibility.
    let _ = program;
}

fn validate_dependencies(
    program: &ConcurrencyProgram,
    entities: &BTreeMap<ConcurrencyNodeId, &ExecutionEntity>,
    state: &mut ValidationState,
) {
    let mut seen = BTreeSet::new();
    let mut graph: BTreeMap<ConcurrencyNodeId, BTreeSet<ConcurrencyNodeId>> =
        entities.keys().cloned().map(|id| (id, BTreeSet::new())).collect();

    for dependency in &program.dependencies {
        let key = (
            dependency.from.clone(),
            dependency.to.clone(),
            dependency.kind,
        );

        if !seen.insert(key) {
            state.error(
                diagnostic_codes::DUPLICATE_DEPENDENCY,
                Some(dependency.from.to_string()),
                Some(dependency.to.to_string()),
                "duplicate dependency edge",
                dependency.origin.clone(),
            );
        }

        let from_exists = entities.contains_key(&dependency.from);
        let to_exists = entities.contains_key(&dependency.to);

        if !from_exists || !to_exists {
            state.error(
                diagnostic_codes::UNKNOWN_DEPENDENCY_ENDPOINT,
                Some(dependency.from.to_string()),
                Some(dependency.to.to_string()),
                format!(
                    "dependency endpoint is unresolved: '{}' -> '{}'",
                    dependency.from, dependency.to
                ),
                dependency.origin.clone(),
            );
            continue;
        }

        if dependency.kind != DependencyKind::Feedback {
            graph
                .entry(dependency.from.clone())
                .or_default()
                .insert(dependency.to.clone());
        }
    }

    for cycle in find_cycles(&graph) {
        let first = cycle.first().cloned();
        let rendered = cycle
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" -> ");

        state.error(
            diagnostic_codes::DEPENDENCY_CYCLE,
            first.map(|id| id.to_string()),
            None,
            format!(
                "cyclic non-feedback dependency detected: {rendered}"
            ),
            SourceOrigin::default(),
        );
    }
}

/// Find strongly connected components iteratively using Kosaraju's algorithm.
///
/// The implementation avoids recursive graph traversal, so graph depth does
/// not consume the Rust call stack. Its time and memory use scale with the
/// graph size and available system resources.
fn find_cycles(
    graph: &BTreeMap<ConcurrencyNodeId, BTreeSet<ConcurrencyNodeId>>,
) -> Vec<Vec<ConcurrencyNodeId>> {
    let mut reverse: BTreeMap<ConcurrencyNodeId, BTreeSet<ConcurrencyNodeId>> =
        graph.keys().cloned().map(|id| (id, BTreeSet::new())).collect();

    for (from, successors) in graph {
        for to in successors {
            reverse.entry(to.clone()).or_default().insert(from.clone());
        }
    }

    let mut visited = BTreeSet::new();
    let mut finish_order = Vec::with_capacity(graph.len());

    for start in graph.keys() {
        if visited.contains(start) {
            continue;
        }

        let mut stack = vec![(start.clone(), false)];

        while let Some((node, expanded)) = stack.pop() {
            if expanded {
                finish_order.push(node);
                continue;
            }

            if !visited.insert(node.clone()) {
                continue;
            }

            stack.push((node.clone(), true));

            if let Some(successors) = graph.get(&node) {
                for next in successors.iter().rev() {
                    if !visited.contains(next) {
                        stack.push((next.clone(), false));
                    }
                }
            }
        }
    }

    visited.clear();
    let mut cycles = Vec::new();

    for start in finish_order.into_iter().rev() {
        if !visited.insert(start.clone()) {
            continue;
        }

        let mut component = Vec::new();
        let mut stack = vec![start.clone()];

        while let Some(node) = stack.pop() {
            component.push(node.clone());

            if let Some(predecessors) = reverse.get(&node) {
                for next in predecessors.iter().rev() {
                    if visited.insert(next.clone()) {
                        stack.push(next.clone());
                    }
                }
            }
        }

        component.sort();

        let self_loop = component.len() == 1
            && graph
                .get(&component[0])
                .is_some_and(|successors| successors.contains(&component[0]));

        if component.len() > 1 || self_loop {
            cycles.push(component);
        }
    }

    cycles.sort_by(|a, b| a.first().cmp(&b.first()));
    cycles
}

fn validate_synchronization_participants(
    program: &ConcurrencyProgram,
    entities: &BTreeMap<ConcurrencyNodeId, &ExecutionEntity>,
    _sync_index: &BTreeMap<ConcurrencyResourceId, &Synchronization>,
    state: &mut ValidationState,
) {
    for synchronization in &program.synchronizations {
        for participant in &synchronization.participants {
            if !entities.contains_key(participant) {
                state.error(
                    diagnostic_codes::UNKNOWN_SYNCHRONIZATION_PARTICIPANT,
                    Some(synchronization.id.to_string()),
                    Some(participant.to_string()),
                    format!(
                        "synchronization '{}' references unknown participant '{}'",
                        synchronization.id, participant
                    ),
                    synchronization.origin.clone(),
                );
            }
        }
    }
}

fn validate_access_protection(
    program: &ConcurrencyProgram,
    entities: &BTreeMap<ConcurrencyNodeId, &ExecutionEntity>,
    sync_index: &BTreeMap<ConcurrencyResourceId, &Synchronization>,
    state: &mut ValidationState,
) {
    for entity in entities.values() {
        for access in &entity.accesses {
            for protection in &access.protected_by {
                match sync_index.get(protection) {
                    None => state.error(
                        diagnostic_codes::UNKNOWN_PROTECTION,
                        Some(entity.id.to_string()),
                        Some(protection.to_string()),
                        format!(
                            "access to '{}' references undeclared synchronization '{}'",
                            access.resource, protection
                        ),
                        access.origin.clone(),
                    ),
                    Some(synchronization)
                        if !synchronization.participants.contains(&entity.id) =>
                    {
                        state.error(
                            diagnostic_codes::UNKNOWN_PROTECTION,
                            Some(entity.id.to_string()),
                            Some(protection.to_string()),
                            format!(
                                "entity '{}' is not a declared participant in '{}'",
                                entity.id, protection
                            ),
                            access.origin.clone(),
                        );
                    }
                    Some(_) => {}
                }
            }
        }
    }

    // Keep the program argument available for future cross-access checks.
    let _ = program;
}

fn validate_potential_races(
    program: &ConcurrencyProgram,
    entities: &BTreeMap<ConcurrencyNodeId, &ExecutionEntity>,
    sync_index: &BTreeMap<ConcurrencyResourceId, &Synchronization>,
    state: &mut ValidationState,
) {
    let reachability = dependency_reachability(program, entities);

    let entity_ids: Vec<_> = entities.keys().cloned().collect();

    for left_index in 0..entity_ids.len() {
        for right_index in (left_index + 1)..entity_ids.len() {
            let left_id = &entity_ids[left_index];
            let right_id = &entity_ids[right_index];

            if happens_before(left_id, right_id, &reachability)
                || happens_before(right_id, left_id, &reachability)
            {
                continue;
            }

            let left = entities[left_id];
            let right = entities[right_id];

            for left_access in &left.accesses {
                for right_access in &right.accesses {
                    if left_access.resource != right_access.resource {
                        continue;
                    }

                    if !conflicts(left_access.mode, right_access.mode) {
                        continue;
                    }

                    if accesses_are_safely_excluded(
                        left_access,
                        right_access,
                        sync_index,
                    ) {
                        continue;
                    }

                    state.warning(
                        diagnostic_codes::POTENTIAL_DATA_RACE,
                        Some(left_id.to_string()),
                        Some(right_id.to_string()),
                        format!(
                            "potential unsynchronized conflicting accesses to '{}'",
                            left_access.resource
                        ),
                        left_access.origin.clone(),
                    );
                }
            }
        }
    }
}

fn conflicts(left: AccessMode, right: AccessMode) -> bool {
    // Atomic accesses are not reported as ordinary non-atomic data races here.
    // Atomic ordering and mixed atomic/non-atomic access rules must be checked
    // by the language's memory-model validator.
    if left.is_atomic() || right.is_atomic() {
        return false;
    }

    left.is_write() || right.is_write()
}

fn accesses_are_safely_excluded(
    left: &SharedAccess,
    right: &SharedAccess,
    sync_index: &BTreeMap<ConcurrencyResourceId, &Synchronization>,
) -> bool {
    for common in left.protected_by.intersection(&right.protected_by) {
        let Some(synchronization) = sync_index.get(common) else {
            continue;
        };

        match synchronization.kind {
            SynchronizationKind::Exclusive => return true,

            SynchronizationKind::SharedExclusive => {
                // The current access model does not distinguish shared-lock
                // acquisition from exclusive-lock acquisition. Conservatively
                // accept this protection only if at least one side writes.
                // The repository adapter should supply richer lock-mode facts
                // if Zamani's normative rules require a more precise analysis.
                if left.mode.is_write() || right.mode.is_write() {
                    return true;
                }
            }

            SynchronizationKind::Atomic
            | SynchronizationKind::MessagePassing
            | SynchronizationKind::Barrier
            | SynchronizationKind::Custom(_) => {}
        }
    }

    false
}

fn dependency_reachability(
    program: &ConcurrencyProgram,
    entities: &BTreeMap<ConcurrencyNodeId, &ExecutionEntity>,
) -> BTreeMap<ConcurrencyNodeId, BTreeSet<ConcurrencyNodeId>> {
    let mut graph: BTreeMap<ConcurrencyNodeId, BTreeSet<ConcurrencyNodeId>> =
        entities.keys().cloned().map(|id| (id, BTreeSet::new())).collect();

    for dependency in &program.dependencies {
        if dependency.kind == DependencyKind::Feedback {
            continue;
        }

        if entities.contains_key(&dependency.from)
            && entities.contains_key(&dependency.to)
        {
            graph
                .entry(dependency.from.clone())
                .or_default()
                .insert(dependency.to.clone());
        }
    }

    let mut result = BTreeMap::new();

    for start in graph.keys() {
        let mut reached = BTreeSet::new();
        let mut queue = VecDeque::new();

        if let Some(successors) = graph.get(start) {
            queue.extend(successors.iter().cloned());
        }

        while let Some(current) = queue.pop_front() {
            if !reached.insert(current.clone()) {
                continue;
            }

            if let Some(successors) = graph.get(&current) {
                queue.extend(successors.iter().cloned());
            }
        }

        result.insert(start.clone(), reached);
    }

    result
}

fn happens_before(
    from: &ConcurrencyNodeId,
    to: &ConcurrencyNodeId,
    reachability: &BTreeMap<ConcurrencyNodeId, BTreeSet<ConcurrencyNodeId>>,
) -> bool {
    reachability
        .get(from)
        .is_some_and(|reachable| reachable.contains(to))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn qname(namespace: &str, name: &str) -> QualifiedName {
        QualifiedName::new(namespace, name).expect("valid qualified name")
    }

    fn node_id(value: &str) -> ConcurrencyNodeId {
        ConcurrencyNodeId::new(value).expect("valid node ID")
    }

    fn resource_id(value: &str) -> ConcurrencyResourceId {
        ConcurrencyResourceId::new(value).expect("valid resource ID")
    }

    fn entity(id: &str) -> ExecutionEntity {
        ExecutionEntity {
            id: node_id(id),
            kind: ExecutionEntityKind::Task,
            operation: ConcurrentOperation {
                operation: qname("zamani.concurrent", "execute"),
                origin: SourceOrigin::default(),
            },
            effects: BTreeSet::new(),
            required_capabilities: BTreeSet::new(),
            accesses: Vec::new(),
            origin: SourceOrigin::default(),
        }
    }

    fn dependency(from: &str, to: &str, kind: DependencyKind) -> Dependency {
        Dependency {
            from: node_id(from),
            to: node_id(to),
            kind,
            origin: SourceOrigin::default(),
        }
    }

    #[test]
    fn rejects_empty_ids() {
        assert!(ConcurrencyNodeId::new("  ").is_err());
        assert!(ConcurrencyResourceId::new("").is_err());
    }

    #[test]
    fn accepts_an_empty_program() {
        let report = validate_concurrency(
            &ConcurrencyProgram::default(),
            &StructuralOnlyContext,
        );
        assert!(report.is_valid());
        assert_eq!(report.completeness, AnalysisCompleteness::Complete);
    }

    #[test]
    fn detects_duplicate_entity_ids() {
        let program = ConcurrencyProgram {
            entities: vec![entity("task"), entity("task")],
            ..ConcurrencyProgram::default()
        };

        let report = validate_concurrency(&program, &StructuralOnlyContext);
        assert!(report.has_errors());
        assert!(report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == diagnostic_codes::DUPLICATE_ENTITY
        }));
    }

    #[test]
    fn detects_unknown_dependency_endpoints() {
        let program = ConcurrencyProgram {
            entities: vec![entity("a")],
            dependencies: vec![dependency("a", "missing", DependencyKind::Data)],
            ..ConcurrencyProgram::default()
        };

        let report = validate_concurrency(&program, &StructuralOnlyContext);
        assert!(report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == diagnostic_codes::UNKNOWN_DEPENDENCY_ENDPOINT
        }));
    }

    #[test]
    fn detects_non_feedback_dependency_cycles() {
        let program = ConcurrencyProgram {
            entities: vec![entity("a"), entity("b"), entity("c")],
            dependencies: vec![
                dependency("a", "b", DependencyKind::Data),
                dependency("b", "c", DependencyKind::Control),
                dependency("c", "a", DependencyKind::Ordering),
            ],
            ..ConcurrencyProgram::default()
        };

        let report = validate_concurrency(&program, &StructuralOnlyContext);
        assert!(report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == diagnostic_codes::DEPENDENCY_CYCLE
        }));
    }

    #[test]
    fn feedback_edges_are_excluded_from_ordinary_cycle_detection() {
        let program = ConcurrencyProgram {
            entities: vec![entity("a"), entity("b")],
            dependencies: vec![
                dependency("a", "b", DependencyKind::Data),
                dependency("b", "a", DependencyKind::Feedback),
            ],
            ..ConcurrencyProgram::default()
        };

        let report = validate_concurrency(&program, &StructuralOnlyContext);
        assert!(!report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == diagnostic_codes::DEPENDENCY_CYCLE
        }));
    }

    #[test]
    fn detects_unknown_synchronization_participants() {
        let program = ConcurrencyProgram {
            entities: vec![entity("worker")],
            synchronizations: vec![Synchronization {
                id: resource_id("lock"),
                kind: SynchronizationKind::Exclusive,
                participants: BTreeSet::from([node_id("missing")]),
                origin: SourceOrigin::default(),
            }],
            ..ConcurrencyProgram::default()
        };

        let report = validate_concurrency(&program, &StructuralOnlyContext);
        assert!(report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code
                == diagnostic_codes::UNKNOWN_SYNCHRONIZATION_PARTICIPANT
        }));
    }

    #[test]
    fn reports_unprotected_conflicting_accesses() {
        let shared = resource_id("shared-value");

        let mut first = entity("first");
        first.accesses.push(SharedAccess {
            resource: shared.clone(),
            mode: AccessMode::Write,
            protected_by: BTreeSet::new(),
            origin: SourceOrigin::default(),
        });

        let mut second = entity("second");
        second.accesses.push(SharedAccess {
            resource: shared,
            mode: AccessMode::Read,
            protected_by: BTreeSet::new(),
            origin: SourceOrigin::default(),
        });

        let program = ConcurrencyProgram {
            entities: vec![first, second],
            ..ConcurrencyProgram::default()
        };

        let report = validate_concurrency(&program, &StructuralOnlyContext);
        assert!(report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == diagnostic_codes::POTENTIAL_DATA_RACE
        }));
    }

    #[test]
    fn does_not_report_conflicting_accesses_ordered_by_dependency() {
        let shared = resource_id("shared-value");

        let mut first = entity("first");
        first.accesses.push(SharedAccess {
            resource: shared.clone(),
            mode: AccessMode::Write,
            protected_by: BTreeSet::new(),
            origin: SourceOrigin::default(),
        });

        let mut second = entity("second");
        second.accesses.push(SharedAccess {
            resource: shared,
            mode: AccessMode::Read,
            protected_by: BTreeSet::new(),
            origin: SourceOrigin::default(),
        });

        let program = ConcurrencyProgram {
            entities: vec![first, second],
            dependencies: vec![dependency(
                "first",
                "second",
                DependencyKind::Ordering,
            )],
            ..ConcurrencyProgram::default()
        };

        let report = validate_concurrency(&program, &StructuralOnlyContext);
        assert!(!report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == diagnostic_codes::POTENTIAL_DATA_RACE
        }));
    }

    #[test]
    fn reports_are_deterministically_ordered() {
        let program = ConcurrencyProgram {
            entities: vec![entity("b"), entity("a"), entity("a")],
            dependencies: vec![
                dependency("missing-1", "missing-2", DependencyKind::Data),
                dependency("missing-3", "missing-4", DependencyKind::Control),
            ],
            ..ConcurrencyProgram::default()
        };

        let first =
            validate_concurrency(&program, &StructuralOnlyContext);
        let second =
            validate_concurrency(&program, &StructuralOnlyContext);

        assert_eq!(first, second);
    }

    struct IncompleteContext;

    impl ConcurrencySemanticContext for IncompleteContext {
        fn validate_operation(
            &self,
            _entity: &ExecutionEntity,
        ) -> Result<(), ContextIssue> {
            Err(ContextIssue {
                code: "registry-unavailable".to_owned(),
                message: "operation registry is unavailable".to_owned(),
                complete: false,
            })
        }

        fn validate_effect(
            &self,
            _entity: &ExecutionEntity,
            _effect: &QualifiedName,
        ) -> Result<(), ContextIssue> {
            Ok(())
        }

        fn validate_capability(
            &self,
            _entity: &ExecutionEntity,
            _capability: &QualifiedName,
        ) -> Result<(), ContextIssue> {
            Ok(())
        }
    }

    #[test]
    fn incomplete_context_does_not_produce_valid_report() {
        let program = ConcurrencyProgram {
            entities: vec![entity("worker")],
            ..ConcurrencyProgram::default()
        };

        let report = validate_concurrency(&program, &IncompleteContext);
        assert_eq!(report.completeness, AnalysisCompleteness::Incomplete);
        assert!(!report.is_valid());
    }
}

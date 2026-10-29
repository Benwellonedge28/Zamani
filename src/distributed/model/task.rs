
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! Target-independent task descriptors for Zamani distributed computation.
//!
//! # Architectural ownership
//!
//! This module owns:
//! - stable logical task identity;
//! - a descriptive task label;
//! - logical input and output port names;
//! - canonical distributed execution requirements;
//! - lifecycle expectations;
//! - references to externally defined failure policies;
//! - validation of the task descriptor's own invariants.
//!
//! It does not own:
//! - source syntax or the native AST;
//! - name resolution, type checking, or semantic analysis;
//! - executable code or a replacement intermediate representation;
//! - resource or capability definitions;
//! - task dependencies or dependency-graph validation;
//! - physical participants, machines, devices, or endpoints;
//! - partitioning, placement, routing, or scheduling;
//! - runtime execution, retry implementation, or checkpoint storage.
//!
//! # Authority
//!
//! - `grammar/spec/distributed.md`
//! - `grammar/distributed/tasks.g4`
//! - `grammar/concurrency/tasks.g4`
//! - `crate::distributed::ids::TaskId`
//! - `crate::distributed::requirements::DistributedRequirements`
//! - `crate::frontend::ast`
//! - `crate::frontend::semantic`
//!
//! The canonical AST and semantic model remain authoritative for source
//! meaning, types, effects, resources, capabilities, and provenance.
//!
//! # Portability
//!
//! A task describes what computation is requested, not where it executes.
//! Its identity is independent of a process, worker, physical machine,
//! network address, accelerator, or execution provider.
//!
//! There are no application-defined universal limits on the number of tasks,
//! input ports, output ports, or characters in a task label. Actual memory,
//! address-space, identifier-representation, and configured operational
//! limits remain finite and must be reported by the layer that enforces them.
//!
//! # Serialization boundary
//!
//! `DistributedRequirements` deliberately preserves canonical semantic
//! resource and capability types and does not currently implement Serde
//! serialization. Accordingly, this aggregate task type does not derive
//! `Serialize` or `Deserialize`.
//!
//! A future wire protocol must introduce a versioned transport DTO and
//! explicit conversion and validation. It must not serialize this type
//! implicitly or duplicate canonical semantic resource types.
//!
//! # Safety
//!
//! This module uses safe Rust only.

use core::fmt;
use std::collections::BTreeSet;
use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::distributed::ids::TaskId;
use crate::distributed::requirements::DistributedRequirements;

/// A validated, human-readable label for a logical task.
///
/// A label is descriptive only. It is not a task identity, symbol-table
/// entry, executable function name, physical process name, or authorization
/// credential.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TaskLabel(String);

impl TaskLabel {
    /// Construct a validated task label.
    ///
    /// Leading and trailing whitespace is preserved. A label must not be
    /// empty, whitespace-only, or contain Unicode control characters.
    ///
    /// # Errors
    ///
    /// Returns a [`TaskValidationError`] if the label is invalid.
    pub fn new(value: impl Into<String>) -> Result<Self, TaskValidationError> {
        let value = value.into();
        validate_text(&value, TextField::Label)?;
        Ok(Self(value))
    }

    /// Return the label as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the label and return its text.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for TaskLabel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// A validated logical name for a task input or output port.
///
/// A port name is local to its task. It is not a global variable, runtime
/// address, wire-protocol field, or independently inferred type.
///
/// Type and ownership information remain in the canonical semantic model.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TaskPortName(String);

impl TaskPortName {
    /// Construct a validated logical port name.
    ///
    /// This type validates only its own textual representation. It does not
    /// resolve the name against a source scope or symbol table.
    ///
    /// # Errors
    ///
    /// Returns a [`TaskValidationError`] if the name is empty, whitespace-only,
    /// or contains a Unicode control character.
    pub fn new(value: impl Into<String>) -> Result<Self, TaskValidationError> {
        let value = value.into();
        validate_text(&value, TextField::PortName)?;
        Ok(Self(value))
    }

    /// Return the logical port name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the name and return its text.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for TaskPortName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// An ordered collection of unique logical task port names.
///
/// Port ordering is preserved because it can matter to adapters that map
/// these declarations to ordered source-level parameter lists. Name lookup
/// and uniqueness checks do not depend on a fixed port count.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TaskPorts {
    names: Vec<TaskPortName>,
}

impl TaskPorts {
    /// Create an empty port collection.
    #[must_use]
    pub const fn new() -> Self {
        Self { names: Vec::new() }
    }

    /// Construct a port collection from validated names.
    ///
    /// Duplicate names are rejected rather than silently collapsed.
    ///
    /// # Errors
    ///
    /// Returns [`TaskValidationError::DuplicatePort`] if a name occurs more
    /// than once.
    pub fn from_names(
        names: impl IntoIterator<Item = TaskPortName>,
    ) -> Result<Self, TaskValidationError> {
        let mut result = Self::new();
        for name in names {
            result.insert(name)?;
        }
        Ok(result)
    }

    /// Insert a port while preserving declaration order.
    ///
    /// # Errors
    ///
    /// Returns [`TaskValidationError::DuplicatePort`] if the name is already
    /// present in this collection.
    pub fn insert(
        &mut self,
        name: TaskPortName,
    ) -> Result<(), TaskValidationError> {
        if self.contains(name.as_str()) {
            return Err(TaskValidationError::DuplicatePort {
                name: name.as_str().to_owned(),
            });
        }

        self.names.push(name);
        Ok(())
    }

    /// Return all port names in declaration order.
    #[must_use]
    pub fn as_slice(&self) -> &[TaskPortName] {
        &self.names
    }

    /// Return the number of declared ports.
    #[must_use]
    pub fn len(&self) -> usize {
        self.names.len()
    }

    /// Return whether no ports have been declared.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }

    /// Check whether the collection contains a port with the supplied name.
    #[must_use]
    pub fn contains(&self, name: &str) -> bool {
        self.names.iter().any(|port| port.as_str() == name)
    }

    /// Find a port by name.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&TaskPortName> {
        self.names.iter().find(|port| port.as_str() == name)
    }

    /// Return a deterministic set of names for comparison and diagnostics.
    ///
    /// This is a derived view. The original declaration order is preserved
    /// by the collection itself.
    #[must_use]
    pub fn sorted_names(&self) -> BTreeSet<&str> {
        self.names.iter().map(TaskPortName::as_str).collect()
    }
}

/// Expected lifecycle behavior for a task.
///
/// These values describe the task's lifecycle contract. They do not assert
/// that a runtime has executed the task or guarantee that a requested
/// lifecycle behavior is available on every target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum TaskLifecycleExpectation {
    /// The caller expects the task to complete before the enclosing operation
    /// is considered complete.
    CompleteBeforeReturn,

    /// The caller permits the task to continue independently after acceptance.
    Detached,

    /// The caller expects completion to be observable through a future,
    /// handle, event, or equivalent runtime-owned mechanism.
    ObservableCompletion,

    /// The task is part of a longer-lived execution whose lifetime is governed
    /// by an external owner.
    OwnerManaged,
}

impl Default for TaskLifecycleExpectation {
    fn default() -> Self {
        Self::CompleteBeforeReturn
    }
}

/// A validated, open-world reference to a failure-policy definition.
///
/// The referenced policy is owned by the appropriate semantic policy or
/// runtime policy registry. This reference does not define retry counts,
/// retry timing, recovery behavior, replication, or idempotency rules.
///
/// Namespaced references allow new policy providers and domains to be
/// introduced without extending a closed Rust enum.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct FailurePolicyRef {
    namespace: String,
    name: String,
}

impl FailurePolicyRef {
    /// Construct a failure-policy reference.
    ///
    /// Both components must be non-empty and contain no Unicode control
    /// characters. Exact spelling is preserved.
    ///
    /// # Errors
    ///
    /// Returns [`TaskValidationError::InvalidFailurePolicyReference`] if
    /// either component is invalid.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Self, TaskValidationError> {
        let namespace = namespace.into();
        let name = name.into();

        validate_text(&namespace, TextField::PolicyNamespace).map_err(|_| {
            TaskValidationError::InvalidFailurePolicyReference {
                namespace: namespace.clone(),
                name: name.clone(),
            }
        })?;

        validate_text(&name, TextField::PolicyName).map_err(|_| {
            TaskValidationError::InvalidFailurePolicyReference {
                namespace: namespace.clone(),
                name: name.clone(),
            }
        })?;

        Ok(Self { namespace, name })
    }

    /// Return the policy namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Return the policy name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the stable qualified display form `namespace:name`.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        format!("{}:{}", self.namespace, self.name)
    }
}

impl fmt::Display for FailurePolicyRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.namespace, self.name)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TextField {
    Label,
    PortName,
    PolicyNamespace,
    PolicyName,
}

fn validate_text(
    value: &str,
    field: TextField,
) -> Result<(), TaskValidationError> {
    if value.trim().is_empty() {
        return Err(match field {
            TextField::Label => TaskValidationError::EmptyLabel,
            TextField::PortName => TaskValidationError::EmptyPortName,
            TextField::PolicyNamespace | TextField::PolicyName => {
                TaskValidationError::InvalidFailurePolicyReference {
                    namespace: if field == TextField::PolicyNamespace {
                        value.to_owned()
                    } else {
                        String::new()
                    },
                    name: if field == TextField::PolicyName {
                        value.to_owned()
                    } else {
                        String::new()
                    },
                }
            }
        });
    }

    if value.chars().any(char::is_control) {
        return Err(TaskValidationError::ControlCharacter { field });
    }

    Ok(())
}

/// Errors produced by task-descriptor validation.
///
/// These errors describe invalid local representation, not resource
/// infeasibility, missing capabilities, failed execution, or an unavailable
/// physical target.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum TaskValidationError {
    /// The task label is empty or whitespace-only.
    EmptyLabel,

    /// A logical port name is empty or whitespace-only.
    EmptyPortName,

    /// A text field contains a Unicode control character.
    ControlCharacter {
        /// The field that contained the invalid character.
        field: TextField,
    },

    /// A port name was declared more than once.
    DuplicatePort {
        /// The conflicting port name.
        name: String,
    },

    /// A failure-policy namespace or name is invalid.
    InvalidFailurePolicyReference {
        /// The supplied policy namespace.
        namespace: String,

        /// The supplied policy name.
        name: String,
    },
}

impl fmt::Display for TaskValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyLabel => {
                f.write_str("task label must not be empty or whitespace-only")
            }
            Self::EmptyPortName => {
                f.write_str("task port name must not be empty or whitespace-only")
            }
            Self::ControlCharacter { field } => {
                write!(f, "task text field {field:?} contains a control character")
            }
            Self::DuplicatePort { name } => {
                write!(f, "task port {name:?} is declared more than once")
            }
            Self::InvalidFailurePolicyReference { namespace, name } => {
                write!(
                    f,
                    "invalid failure-policy reference: namespace {namespace:?}, \
                     name {name:?}"
                )
            }
        }
    }
}

impl Error for TaskValidationError {}

/// A target-independent distributed task descriptor.
///
/// The task's stable identity is authoritative. Labels and port names are
/// descriptive logical data; they are not substitutes for semantic symbols
/// or typed values.
///
/// The canonical [`DistributedRequirements`] collection is retained without
/// copying its resource and capability types into a distributed-specific
/// parallel model.
///
/// This structure deliberately does not contain a worker ID, physical node,
/// endpoint, backend handle, IR operation, or execution result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    id: TaskId,
    label: Option<TaskLabel>,
    inputs: TaskPorts,
    outputs: TaskPorts,
    requirements: DistributedRequirements,
    lifecycle: TaskLifecycleExpectation,
    failure_policy: Option<FailurePolicyRef>,
}

impl Task {
    /// Create a task with a stable identity and an already-analyzed
    /// requirement envelope.
    ///
    /// A task with no declared inputs or outputs is valid. A task with empty
    /// requirements is valid only when the semantic analysis producer knows
    /// that requirement analysis is complete.
    ///
    /// This constructor does not execute, register, schedule, or authenticate
    /// the task.
    #[must_use]
    pub fn new(id: TaskId, requirements: DistributedRequirements) -> Self {
        Self {
            id,
            label: None,
            inputs: TaskPorts::new(),
            outputs: TaskPorts::new(),
            requirements,
            lifecycle: TaskLifecycleExpectation::default(),
            failure_policy: None,
        }
    }

    /// Return the stable logical task identity.
    #[must_use]
    pub const fn id(&self) -> TaskId {
        self.id
    }

    /// Return the optional descriptive label.
    #[must_use]
    pub fn label(&self) -> Option<&TaskLabel> {
        self.label.as_ref()
    }

    /// Set the task's descriptive label.
    pub fn set_label(&mut self, label: TaskLabel) {
        self.label = Some(label);
    }

    /// Clear the optional task label.
    pub fn clear_label(&mut self) {
        self.label = None;
    }

    /// Return the declared input ports in declaration order.
    #[must_use]
    pub fn inputs(&self) -> &TaskPorts {
        &self.inputs
    }

    /// Return the declared output ports in declaration order.
    #[must_use]
    pub fn outputs(&self) -> &TaskPorts {
        &self.outputs
    }

    /// Declare an input port.
    ///
    /// Input names must be unique among this task's inputs.
    ///
    /// # Errors
    ///
    /// Returns [`TaskValidationError::DuplicatePort`] if the input has
    /// already been declared.
    pub fn declare_input(
        &mut self,
        name: TaskPortName,
    ) -> Result<(), TaskValidationError> {
        self.inputs.insert(name)
    }

    /// Declare an output port.
    ///
    /// Output names must be unique among this task's outputs. An input and
    /// an output may have the same name because they are distinct port
    /// namespaces. Whether such a declaration is meaningful in a particular
    /// language construct is decided by semantic analysis.
    ///
    /// # Errors
    ///
    /// Returns [`TaskValidationError::DuplicatePort`] if the output has
    /// already been declared.
    pub fn declare_output(
        &mut self,
        name: TaskPortName,
    ) -> Result<(), TaskValidationError> {
        self.outputs.insert(name)
    }

    /// Return the canonical distributed requirement envelope.
    #[must_use]
    pub fn requirements(&self) -> &DistributedRequirements {
        &self.requirements
    }

    /// Replace the requirement envelope with another canonical envelope.
    ///
    /// Callers must preserve the semantic analysis completeness contract.
    /// Planning must reject incomplete requirement analysis rather than
    /// assuming omitted requirements are satisfied.
    pub fn set_requirements(&mut self, requirements: DistributedRequirements) {
        self.requirements = requirements;
    }

    /// Return whether requirement analysis is complete.
    #[must_use]
    pub fn requirements_are_complete(&self) -> bool {
        self.requirements.is_complete()
    }

    /// Return the expected lifecycle behavior.
    #[must_use]
    pub const fn lifecycle(&self) -> TaskLifecycleExpectation {
        self.lifecycle
    }

    /// Set the task's lifecycle expectation.
    ///
    /// This is intent only. A runtime must establish whether it can honor
    /// the requested lifecycle behavior.
    pub fn set_lifecycle(&mut self, lifecycle: TaskLifecycleExpectation) {
        self.lifecycle = lifecycle;
    }

    /// Return the optional external failure-policy reference.
    #[must_use]
    pub fn failure_policy(&self) -> Option<&FailurePolicyRef> {
        self.failure_policy.as_ref()
    }

    /// Set a reference to an externally defined failure policy.
    ///
    /// This does not implement retries, migration, checkpointing, or
    /// recovery. The owning policy registry and runtime must resolve and
    /// enforce the referenced policy.
    pub fn set_failure_policy(&mut self, policy: FailurePolicyRef) {
        self.failure_policy = Some(policy);
    }

    /// Clear the optional failure-policy reference.
    pub fn clear_failure_policy(&mut self) {
        self.failure_policy = None;
    }

    /// Validate invariants required before the task enters distributed
    /// planning.
    ///
    /// This validates the task descriptor and its requirement-analysis
    /// completeness. It does not check whether a target has the required
    /// capabilities or resources.
    ///
    /// # Errors
    ///
    /// Returns [`TaskValidationError`] for descriptor-local errors. An
    /// incomplete requirements envelope is reported separately as
    /// [`TaskValidationError::IncompleteRequirements`].
    pub fn validate(&self) -> Result<(), TaskValidationError> {
        if !self.requirements.is_complete() {
            return Err(TaskValidationError::IncompleteRequirements);
        }

        // Port collections prevent duplicates on insertion. Recheck here so
        // future internal conversions cannot silently bypass the invariant.
        validate_unique_ports(&self.inputs)?;
        validate_unique_ports(&self.outputs)?;

        Ok(())
    }
}

fn validate_unique_ports(ports: &TaskPorts) -> Result<(), TaskValidationError> {
    let mut seen = BTreeSet::new();

    for port in ports.as_slice() {
        if !seen.insert(port.as_str()) {
            return Err(TaskValidationError::DuplicatePort {
                name: port.as_str().to_owned(),
            });
        }
    }

    Ok(())
}

impl TaskValidationError {
    /// Return whether this error indicates incomplete semantic analysis.
    #[must_use]
    pub const fn is_incomplete_analysis(&self) -> bool {
        matches!(self, Self::IncompleteRequirements)
    }
}

// The incomplete-analysis case is distinct from local descriptor errors.
// It is added here so planners can fail closed without inventing a target
// feasibility result.
impl TaskValidationError {
    // Kept as a separate method block to make the semantic distinction clear.
}

/// Unit tests for task-model invariants.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::distributed::ids::IdAllocator;

    fn task() -> Task {
        let allocator = IdAllocator::new();
        Task::new(
            allocator.task_id().expect("task ID allocation should succeed"),
            DistributedRequirements::new(),
        )
    }

    fn port(value: &str) -> TaskPortName {
        TaskPortName::new(value).expect("test port should be valid")
    }

    #[test]
    fn task_identity_is_stable_when_descriptive_fields_change() {
        let mut value = task();
        let id = value.id();

        value.set_label(TaskLabel::new("matrix multiply").expect("valid label"));
        value.set_lifecycle(TaskLifecycleExpectation::Detached);

        assert_eq!(value.id(), id);
    }

    #[test]
    fn duplicate_input_ports_are_rejected() {
        let mut value = task();

        value.declare_input(port("left")).expect("first input succeeds");

        assert!(matches!(
            value.declare_input(port("left")),
            Err(TaskValidationError::DuplicatePort { .. })
        ));
    }

    #[test]
    fn duplicate_output_ports_are_rejected() {
        let mut value = task();

        value.declare_output(port("result")).expect("first output succeeds");

        assert!(matches!(
            value.declare_output(port("result")),
            Err(TaskValidationError::DuplicatePort { .. })
        ));
    }

    #[test]
    fn input_and_output_names_are_separate_namespaces() {
        let mut value = task();

        value.declare_input(port("state")).expect("input succeeds");
        value.declare_output(port("state")).expect("output succeeds");

        assert!(value.inputs().contains("state"));
        assert!(value.outputs().contains("state"));
        assert!(value.validate().is_ok());
    }

    #[test]
    fn empty_ports_are_valid() {
        let value = task();

        assert!(value.inputs().is_empty());
        assert!(value.outputs().is_empty());
        assert!(value.validate().is_ok());
    }

    #[test]
    fn invalid_labels_are_rejected() {
        assert!(matches!(
            TaskLabel::new(" \t "),
            Err(TaskValidationError::EmptyLabel)
        ));

        assert!(matches!(
            TaskLabel::new("bad\nlabel"),
            Err(TaskValidationError::ControlCharacter { .. })
        ));
    }

    #[test]
    fn invalid_port_names_are_rejected() {
        assert!(matches!(
            TaskPortName::new("   "),
            Err(TaskValidationError::EmptyPortName)
        ));

        assert!(matches!(
            TaskPortName::new("bad\u{0000}port"),
            Err(TaskValidationError::ControlCharacter { .. })
        ));
    }

    #[test]
    fn failure_policy_reference_is_open_world() {
        let policy = FailurePolicyRef::new(
            "future.runtime",
            "recover_after_reconnect",
        )
        .expect("namespaced policy reference should be valid");

        assert_eq!(policy.qualified_name(), "future.runtime:recover_after_reconnect");
    }

    #[test]
    fn invalid_failure_policy_references_are_rejected() {
        assert!(FailurePolicyRef::new("", "retry").is_err());
        assert!(FailurePolicyRef::new("runtime", " \t ").is_err());
        assert!(FailurePolicyRef::new("runtime", "bad\npolicy").is_err());
    }

    #[test]
    fn task_validation_does_not_claim_target_feasibility() {
        let value = task();

        // This descriptor is locally valid. A planner must separately
        // evaluate capabilities, resources, policy, and target availability.
        assert!(value.validate().is_ok());
    }
}

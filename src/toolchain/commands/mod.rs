
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Command Infrastructure
//!
//! Path: src/toolchain/commands/mod.rs
//!
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Mission
//!
//! Provide one extensible command-service architecture for the complete
//! Zamani development ecosystem.
//!
//! POCO-REAF:
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! ## Ownership
//!
//! This module owns:
//! - Command identity and descriptors.
//! - Command registration and discovery.
//! - Command-handler contracts.
//! - Command request and response contracts.
//! - Authorization integration.
//! - Audit integration.
//! - Command dispatch.
//! - Structured command errors.
//!
//! It does not own:
//! - Source parsing or semantic analysis.
//! - Compilation or optimization.
//! - Dependency resolution.
//! - Package creation or installation.
//! - Registry communication.
//! - Backend discovery or code generation.
//! - External process execution.
//! - Hardware discovery or allocation.
//! - Runtime execution or scheduling.
//!
//! Those responsibilities remain with their canonical subsystems.
//!
//! ## Integration
//!
//! CLI / IDE / automation / AI agent
//!              |
//!              v
//!       CommandRequest
//!              |
//!              v
//!       CommandRegistry
//!              |
//!       Authorization
//!              |
//!          Audit
//!              |
//!              v
//!       CommandHandler
//!              |
//!      +-------+--------+
//!      |       |        |
//!      v       v        v
//!     Build   Package   Deployment
//!      |       |        |
//!      v       v        v
//!   Compiler Registry  ZUR/providers
//!      |
//!      v
//!   Backend / artifacts
//!
//! ## Architectural invariants
//!
//! - Commands are extensible, not a closed enumeration.
//! - Registering a command does not establish its availability.
//! - Discovering a command does not execute it.
//! - Authorization precedes execution.
//! - Mandatory audit records must be accepted before execution.
//! - Handler errors must never become successful responses.
//! - A completed operation with a failed final audit record must not
//!   be represented as though execution never happened.
//! - No command implementation is fabricated or simulated here.
//! - No arbitrary command-count or hardware-capacity limits exist.
//! - No global mutable registry is initialized implicitly.
//! - Registry locks are never held while executing a handler.
//! - External clients use the same service boundary.
//!
//! ## Important
//!
//! A registered command is not necessarily production-ready.
//! Its implementation must independently satisfy its owning command
//! contract and integration tests.

use std::collections::BTreeMap;
use std::error::Error;
use std::ffi::OsString;
use std::fmt;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use serde_json::Value;

// ============================================================================
// MODULE IDENTITY
// ============================================================================

/// Stable identity of the command infrastructure.
pub const COMMANDS_MODULE_ID: &str = "zamani.toolchain.commands";

/// Version of the command-service API.
///
/// This is independent of the Zamani language, grammar, compiler,
/// backend, package format and runtime versions.
pub const COMMANDS_API_VERSION: u32 = 1;

/// Returns the command-service API version.
#[must_use]
pub const fn api_version() -> u32 {
    COMMANDS_API_VERSION
}

/// Returns the stable module identity.
#[must_use]
pub const fn module_id() -> &'static str {
    COMMANDS_MODULE_ID
}

// ============================================================================
// STANDARD COMMAND IDENTIFIERS
// ============================================================================

/// Standard command identities.
///
/// These constants provide conventional names, not a closed command
/// enumeration. Third-party and future commands can use additional
/// valid identifiers without changing this module.
pub mod standard_commands {
    /// Build a project.
    pub const BUILD: &str = "build";

    /// Validate a project without requiring final build artifacts.
    pub const CHECK: &str = "check";

    /// Remove recognized generated project outputs.
    pub const CLEAN: &str = "clean";

    /// Build and hand off a program to the runtime.
    pub const RUN: &str = "run";

    /// Execute configured project tests.
    pub const TEST: &str = "test";

    /// Execute configured benchmarks.
    pub const BENCH: &str = "bench";

    /// Format source files.
    pub const FORMAT: &str = "format";

    /// Run static and project diagnostics.
    pub const LINT: &str = "lint";

    /// Generate project documentation.
    pub const DOC: &str = "doc";

    /// Create a package.
    pub const PACKAGE: &str = "package";

    /// Install a package.
    pub const INSTALL: &str = "install";

    /// Publish a package.
    pub const PUBLISH: &str = "publish";

    /// Inspect registered target providers and capabilities.
    pub const TARGET: &str = "target";

    /// Inspect external tools.
    pub const TOOL: &str = "tool";

    /// Diagnose project and environment readiness.
    pub const DOCTOR: &str = "doctor";

    /// Prepare or coordinate deployment.
    pub const DEPLOY: &str = "deploy";

    /// Inspect project dependencies.
    pub const DEPS: &str = "deps";

    /// Initialize a project.
    pub const INIT: &str = "init";
}

// ============================================================================
// COMMAND IDENTITY
// ============================================================================

/// Globally distinguishable command identity.
///
/// Identifiers are open-ended and support namespaced extensions.
///
/// Examples:
///
/// - `build`
/// - `package`
/// - `quantum.simulate`
/// - `hdl.synthesize`
/// - `ai.train`
/// - `vendor.extension.command`
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommandId(String);

impl CommandId {
    /// Construct and validate a command identifier.
    ///
    /// Accepted characters:
    ///
    /// - ASCII letters.
    /// - ASCII digits.
    /// - Underscore.
    /// - Hyphen.
    /// - Period.
    /// - Colon.
    ///
    /// Identifiers cannot begin or end with a separator.
    pub fn new(value: impl Into<String>) -> Result<Self, CommandError> {
        let value = value.into();

        let valid = !value.is_empty()
            && value
                .as_bytes()
                .first()
                .is_some_and(u8::is_ascii_alphanumeric)
            && value
                .as_bytes()
                .last()
                .is_some_and(u8::is_ascii_alphanumeric)
            && value.bytes().all(|byte| {
                byte.is_ascii_alphanumeric()
                    || matches!(byte, b'_' | b'-' | b'.' | b':')
            });

        if !valid {
            return Err(CommandError::new(
                CommandErrorKind::InvalidCommandId,
                format!("Invalid command identifier: {value:?}"),
            ));
        }

        Ok(Self(value))
    }

    /// Return the identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CommandId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl TryFrom<&str> for CommandId {
    type Error = CommandError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<String> for CommandId {
    type Error = CommandError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

// ============================================================================
// COMMAND MUTABILITY
// ============================================================================

/// Classification of command side effects.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandMutability {
    /// The command promises not to mutate project or external state.
    ReadOnly,

    /// The command may modify local project or workspace state.
    Mutating,

    /// The command may affect external systems or infrastructure.
    ExternalSideEffects,
}

// ============================================================================
// COMMAND DESCRIPTOR
// ============================================================================

/// Immutable description of a registered command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandDescriptor {
    /// Globally distinguishable command identity.
    pub id: CommandId,

    /// Human-readable description.
    pub summary: String,

    /// Command API version.
    pub api_version: u32,

    /// Side-effect classification.
    pub mutability: CommandMutability,

    /// Whether the command requires a workspace.
    pub requires_workspace: bool,

    /// Whether successful authorization must be audited.
    pub audit_required: bool,

    /// Capabilities required by the command implementation.
    ///
    /// These are descriptive integration requirements, not proof
    /// that a provider or physical resource is available.
    pub required_capabilities: Vec<String>,
}

impl CommandDescriptor {
    /// Create a command descriptor.
    #[must_use]
    pub fn new(
        id: CommandId,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            id,
            summary: summary.into(),
            api_version: COMMANDS_API_VERSION,
            mutability: CommandMutability::ReadOnly,
            requires_workspace: false,
            audit_required: false,
            required_capabilities: Vec::new(),
        }
    }

    /// Declare that the command modifies local state.
    #[must_use]
    pub fn mutating(mut self) -> Self {
        self.mutability = CommandMutability::Mutating;
        self
    }

    /// Declare that the command affects external systems.
    #[must_use]
    pub fn external_side_effects(mut self) -> Self {
        self.mutability = CommandMutability::ExternalSideEffects;
        self
    }

    /// Require an existing workspace context.
    #[must_use]
    pub fn requires_workspace(mut self) -> Self {
        self.requires_workspace = true;
        self
    }

    /// Require successful audit recording.
    #[must_use]
    pub fn requires_audit(mut self) -> Self {
        self.audit_required = true;
        self
    }

    /// Declare an additional capability requirement.
    #[must_use]
    pub fn requires_capability(
        mut self,
        capability: impl Into<String>,
    ) -> Self {
        self.required_capabilities.push(capability.into());
        self
    }

    fn validate(&self) -> Result<(), CommandError> {
        if self.summary.trim().is_empty() {
            return Err(CommandError::new(
                CommandErrorKind::InvalidDescriptor,
                format!("Command {} has an empty summary", self.id),
            ));
        }

        if self.api_version == 0 {
            return Err(CommandError::new(
                CommandErrorKind::InvalidDescriptor,
                format!("Command {} has an invalid API version", self.id),
            ));
        }

        if self.audit_required
            && self.mutability == CommandMutability::ReadOnly
        {
            // Read-only commands may still require auditing.
            // This is permitted and intentionally not rejected.
        }

        if self
            .required_capabilities
            .iter()
            .any(|capability| capability.trim().is_empty())
        {
            return Err(CommandError::new(
                CommandErrorKind::InvalidDescriptor,
                format!(
                    "Command {} declares an empty capability",
                    self.id
                ),
            ));
        }

        Ok(())
    }
}

// ============================================================================
// COMMAND REQUEST
// ============================================================================

/// Structured command invocation request.
///
/// Arguments use `OsString` to preserve platform-native command-line
/// arguments, including non-Unicode filesystem paths.
///
/// Options use JSON values so CLI, IDE, automation and AI clients
/// can share the same structured interface.
#[derive(Debug, Clone)]
pub struct CommandRequest {
    /// Command to invoke.
    pub command: CommandId,

    /// Positional command arguments.
    pub arguments: Vec<OsString>,

    /// Named structured options.
    pub options: BTreeMap<String, Value>,

    /// Explicit project or workspace directory.
    ///
    /// `None` means the caller did not supply one. The handler must
    /// not silently assume that an arbitrary current directory is
    /// the intended project.
    pub working_directory: Option<PathBuf>,

    /// Optional caller-generated correlation identifier.
    pub correlation_id: Option<String>,
}

impl CommandRequest {
    /// Construct a command request.
    #[must_use]
    pub fn new(command: CommandId) -> Self {
        Self {
            command,
            arguments: Vec::new(),
            options: BTreeMap::new(),
            working_directory: None,
            correlation_id: None,
        }
    }

    /// Add positional arguments.
    #[must_use]
    pub fn arguments(
        mut self,
        arguments: impl IntoIterator<Item = OsString>,
    ) -> Self {
        self.arguments.extend(arguments);
        self
    }

    /// Add or replace a structured option.
    #[must_use]
    pub fn option(
        mut self,
        name: impl Into<String>,
        value: Value,
    ) -> Self {
        self.options.insert(name.into(), value);
        self
    }

    /// Set the explicit working directory.
    #[must_use]
    pub fn working_directory(
        mut self,
        directory: impl Into<PathBuf>,
    ) -> Self {
        self.working_directory = Some(directory.into());
        self
    }

    /// Set a correlation identifier.
    #[must_use]
    pub fn correlation_id(
        mut self,
        value: impl Into<String>,
    ) -> Self {
        self.correlation_id = Some(value.into());
        self
    }
}

// ============================================================================
// STRUCTURED DIAGNOSTICS
// ============================================================================

/// Diagnostic severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    /// Informational observation.
    Info,

    /// Non-fatal concern.
    Warning,

    /// An error requiring attention.
    Error,
}

/// Structured command diagnostic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandDiagnostic {
    /// Diagnostic severity.
    pub severity: DiagnosticSeverity,

    /// Stable diagnostic code.
    pub code: String,

    /// Human-readable diagnostic message.
    pub message: String,

    /// Optional source or resource location.
    pub location: Option<String>,
}

impl CommandDiagnostic {
    /// Construct a diagnostic.
    #[must_use]
    pub fn new(
        severity: DiagnosticSeverity,
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            severity,
            code: code.into(),
            message: message.into(),
            location: None,
        }
    }

    /// Associate a source or resource location.
    #[must_use]
    pub fn at(mut self, location: impl Into<String>) -> Self {
        self.location = Some(location.into());
        self
    }
}

// ============================================================================
// COMMAND ARTIFACT REFERENCES
// ============================================================================

/// Reference to an artifact produced or consumed by a command.
///
/// This is metadata only. Artifact verification remains the
/// responsibility of the compiler/artifact/security authorities.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandArtifact {
    /// Artifact identity.
    pub id: String,

    /// Artifact media or format type.
    pub media_type: String,

    /// Provider-defined artifact location.
    pub location: String,

    /// Optional content digest.
    pub digest: Option<String>,
}

// ============================================================================
// COMMAND OUTCOMES
// ============================================================================

/// Successful command outcome classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandOutcome {
    /// All requested operations completed.
    Completed,

    /// The command completed partially.
    ///
    /// The response must explain which operations remain incomplete.
    Partial,

    /// No operation was necessary or performed.
    NoOp,
}

/// Successful command result.
///
/// A handler must return an error when the command fails.
/// It must never manufacture a successful result after failure.
#[derive(Debug, Clone)]
pub struct CommandResult {
    /// Outcome classification.
    pub outcome: CommandOutcome,

    /// Structured command-specific data.
    pub data: Value,

    /// Diagnostics produced by the command.
    pub diagnostics: Vec<CommandDiagnostic>,

    /// Produced or consumed artifact references.
    pub artifacts: Vec<CommandArtifact>,

    /// Additional machine-readable metadata.
    pub metadata: BTreeMap<String, Value>,
}

impl Default for CommandResult {
    fn default() -> Self {
        Self {
            outcome: CommandOutcome::Completed,
            data: Value::Null,
            diagnostics: Vec::new(),
            artifacts: Vec::new(),
            metadata: BTreeMap::new(),
        }
    }
}

impl CommandResult {
    /// Create an empty successful result.
    #[must_use]
    pub fn completed() -> Self {
        Self::default()
    }

    /// Set the result data.
    #[must_use]
    pub fn with_data(mut self, data: Value) -> Self {
        self.data = data;
        self
    }

    /// Set the outcome.
    #[must_use]
    pub fn with_outcome(mut self, outcome: CommandOutcome) -> Self {
        self.outcome = outcome;
        self
    }

    /// Add a diagnostic.
    #[must_use]
    pub fn with_diagnostic(
        mut self,
        diagnostic: CommandDiagnostic,
    ) -> Self {
        self.diagnostics.push(diagnostic);
        self
    }

    /// Add an artifact reference.
    #[must_use]
    pub fn with_artifact(
        mut self,
        artifact: CommandArtifact,
    ) -> Self {
        self.artifacts.push(artifact);
        self
    }
}

/// Complete command response.
#[derive(Debug, Clone)]
pub struct CommandResponse {
    /// Executed command identity.
    pub command: CommandId,

    /// Caller correlation identifier.
    pub correlation_id: Option<String>,

    /// Structured result.
    pub result: CommandResult,
}

// ============================================================================
// COMMAND ERRORS
// ============================================================================

/// Structured command error classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandErrorKind {
    /// Invalid command identity.
    InvalidCommandId,

    /// Invalid command descriptor.
    InvalidDescriptor,

    /// Duplicate command registration.
    DuplicateCommand,

    /// Requested command is not registered.
    CommandNotFound,

    /// Authorization was denied.
    AuthorizationDenied,

    /// No required authorization policy was configured.
    AuthorizationUnavailable,

    /// Mandatory audit infrastructure is unavailable.
    AuditUnavailable,

    /// Audit recording failed.
    AuditFailure,

    /// Command handler failed.
    HandlerFailure,

    /// Registry synchronization failed.
    RegistryFailure,

    /// Command implementation returned an invalid result.
    InvalidResponse,
}

/// Structured command failure.
///
/// `operation_may_have_completed` is particularly important when
/// an external side effect completed but recording its final audit
/// event failed. Callers must not blindly retry such an operation.
#[derive(Debug)]
pub struct CommandError {
    /// Error classification.
    pub kind: CommandErrorKind,

    /// Optional associated command.
    pub command: Option<CommandId>,

    /// Human-readable failure information.
    pub message: String,

    /// Whether the operation may already have completed.
    pub operation_may_have_completed: bool,

    /// Underlying error, when available.
    source: Option<Box<dyn Error + Send + Sync + 'static>>,
}

impl CommandError {
    /// Construct a structured error.
    #[must_use]
    pub fn new(
        kind: CommandErrorKind,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            command: None,
            message: message.into(),
            operation_may_have_completed: false,
            source: None,
        }
    }

    /// Associate a command identity.
    #[must_use]
    pub fn for_command(mut self, command: CommandId) -> Self {
        self.command = Some(command);
        self
    }

    /// Preserve an underlying error.
    #[must_use]
    pub fn with_source<E>(mut self, source: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        self.source = Some(Box::new(source));
        self
    }

    /// Mark that an operation may have completed despite the error.
    #[must_use]
    pub fn may_have_completed(mut self) -> Self {
        self.operation_may_have_completed = true;
        self
    }
}

impl fmt::Display for CommandError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.command {
            Some(command) => {
                write!(
                    formatter,
                    "{:?} for command '{}': {}",
                    self.kind, command, self.message
                )
            }
            None => write!(formatter, "{:?}: {}", self.kind, self.message),
        }
    }
}

impl Error for CommandError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_deref()
            .map(|error| error as &(dyn Error + 'static))
    }
}

// ============================================================================
// COMMAND HANDLER CONTRACT
// ============================================================================

/// Implemented by actual command services.
///
/// Implementations must:
///
/// - Validate their command-specific options.
/// - Use canonical repository subsystem APIs.
/// - Preserve structured errors.
/// - Respect cancellation and operational budgets where applicable.
/// - Avoid fabricated success.
/// - Preserve source semantics.
/// - Avoid arbitrary universal hardware ceilings.
///
/// Handlers must be thread-safe because the registry can be shared
/// across concurrent clients.
pub trait CommandHandler: Send + Sync + 'static {
    /// Execute the requested command.
    fn execute(
        &self,
        request: &CommandRequest,
    ) -> Result<CommandResult, CommandError>;
}

// ============================================================================
// AUTHORIZATION
// ============================================================================

/// Authorization policy boundary.
///
/// The policy implementation owns identity, permission evaluation
/// and the meaning of its authorization evidence.
///
/// The command dispatcher does not invent user identities or
/// automatically grant permissions.
pub trait CommandAuthorizer: Send + Sync + 'static {
    /// Authorize a command invocation.
    fn authorize(
        &self,
        descriptor: &CommandDescriptor,
        request: &CommandRequest,
    ) -> Result<(), CommandError>;
}

// ============================================================================
// AUDIT
// ============================================================================

/// Audit event phase.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandAuditPhase {
    /// Invocation was authorized and is about to execute.
    Started,

    /// Handler completed successfully.
    Completed,

    /// Handler failed.
    Failed,

    /// Authorization was denied.
    Denied,
}

/// Structured audit event.
///
/// The audit provider decides how events are persisted and protected.
/// This module does not store secrets or invent timestamps.
#[derive(Debug, Clone)]
pub struct CommandAuditEvent {
    /// Command identity.
    pub command: CommandId,

    /// Caller correlation identifier.
    pub correlation_id: Option<String>,

    /// Audit phase.
    pub phase: CommandAuditPhase,

    /// Optional structured outcome or error classification.
    pub detail: Option<String>,
}

/// Audit provider contract.
pub trait CommandAuditSink: Send + Sync + 'static {
    /// Persist or otherwise accept an audit event.
    fn record(
        &self,
        event: &CommandAuditEvent,
    ) -> Result<(), CommandError>;
}

// ============================================================================
// REGISTERED COMMAND
// ============================================================================

struct RegisteredCommand {
    descriptor: CommandDescriptor,
    handler: Arc<dyn CommandHandler>,
}

// ============================================================================
// REGISTRY INTERNAL STATE
// ============================================================================

struct CommandRegistryInner {
    commands: RwLock<BTreeMap<CommandId, RegisteredCommand>>,
    authorizer: Option<Arc<dyn CommandAuthorizer>>,
    audit_sink: Option<Arc<dyn CommandAuditSink>>,
}

// ============================================================================
// COMMAND REGISTRY
// ============================================================================

/// Thread-safe, extensible command registry.
///
/// The registry is explicitly constructed. Importing this module
/// does not register commands, initialize providers, access the
/// filesystem, start threads or perform network operations.
///
/// Cloned registries share the same registration state.
#[derive(Clone)]
pub struct CommandRegistry {
    inner: Arc<CommandRegistryInner>,
}

impl Default for CommandRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandRegistry {
    /// Construct an empty command registry.
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: Arc::new(CommandRegistryInner {
                commands: RwLock::new(BTreeMap::new()),
                authorizer: None,
                audit_sink: None,
            }),
        }
    }

    /// Construct a registry with an authorization provider.
    #[must_use]
    pub fn with_authorizer(
        mut self,
        authorizer: Arc<dyn CommandAuthorizer>,
    ) -> Self {
        Arc::get_mut(&mut self.inner)
            .expect("registry configuration is only available before cloning")
            .authorizer = Some(authorizer);
        self
    }

    /// Construct a registry with an audit provider.
    #[must_use]
    pub fn with_audit_sink(
        mut self,
        sink: Arc<dyn CommandAuditSink>,
    ) -> Self {
        Arc::get_mut(&mut self.inner)
            .expect("registry configuration is only available before cloning")
            .audit_sink = Some(sink);
        self
    }

    /// Register a command implementation.
    ///
    /// Duplicate identities are rejected. Registration never
    /// replaces an existing handler implicitly.
    pub fn register<H>(
        &self,
        descriptor: CommandDescriptor,
        handler: H,
    ) -> Result<(), CommandError>
    where
        H: CommandHandler,
    {
        self.register_shared(descriptor, Arc::new(handler))
    }

    /// Register an existing shared handler.
    pub fn register_shared(
        &self,
        descriptor: CommandDescriptor,
        handler: Arc<dyn CommandHandler>,
    ) -> Result<(), CommandError> {
        descriptor.validate()?;

        let id = descriptor.id.clone();

        let mut commands = self.inner.commands.write().map_err(|_| {
            CommandError::new(
                CommandErrorKind::RegistryFailure,
                "Command registry lock is poisoned",
            )
        })?;

        if commands.contains_key(&id) {
            return Err(
                CommandError::new(
                    CommandErrorKind::DuplicateCommand,
                    format!("Command '{id}' is already registered"),
                )
                .for_command(id),
            );
        }

        commands.insert(
            id,
            RegisteredCommand {
                descriptor,
                handler,
            },
        );

        Ok(())
    }

    /// Unregister a command.
    ///
    /// Removing a registration does not cancel a handler that has
    /// already been cloned by an in-flight invocation.
    pub fn unregister(
        &self,
        id: &CommandId,
    ) -> Result<bool, CommandError> {
        let mut commands = self.inner.commands.write().map_err(|_| {
            CommandError::new(
                CommandErrorKind::RegistryFailure,
                "Command registry lock is poisoned",
            )
        })?;

        Ok(commands.remove(id).is_some())
    }

    /// Determine whether a command is registered.
    pub fn contains(
        &self,
        id: &CommandId,
    ) -> Result<bool, CommandError> {
        let commands = self.inner.commands.read().map_err(|_| {
            CommandError::new(
                CommandErrorKind::RegistryFailure,
                "Command registry lock is poisoned",
            )
        })?;

        Ok(commands.contains_key(id))
    }

    /// Return registered command descriptors in deterministic order.
    pub fn descriptors(
        &self,
    ) -> Result<Vec<CommandDescriptor>, CommandError> {
        let commands = self.inner.commands.read().map_err(|_| {
            CommandError::new(
                CommandErrorKind::RegistryFailure,
                "Command registry lock is poisoned",
            )
        })?;

        Ok(commands
            .values()
            .map(|registered| registered.descriptor.clone())
            .collect())
    }

    /// Return the descriptor for a registered command.
    pub fn descriptor(
        &self,
        id: &CommandId,
    ) -> Result<Option<CommandDescriptor>, CommandError> {
        let commands = self.inner.commands.read().map_err(|_| {
            CommandError::new(
                CommandErrorKind::RegistryFailure,
                "Command registry lock is poisoned",
            )
        })?;

        Ok(commands
            .get(id)
            .map(|registered| registered.descriptor.clone()))
    }

    /// Dispatch a command through its registered implementation.
    ///
    /// The registry lock is released before authorization, audit
    /// callbacks and handler execution.
    ///
    /// Commands with side effects require an authorization provider.
    /// Commands declaring mandatory auditing require an audit sink.
    ///
    /// A final audit failure is reported with
    /// `operation_may_have_completed = true` when execution succeeded.
    pub fn dispatch(
        &self,
        request: &CommandRequest,
    ) -> Result<CommandResponse, CommandError> {
        // Resolve the handler without retaining a registry lock.
        let (descriptor, handler) = {
            let commands = self.inner.commands.read().map_err(|_| {
                CommandError::new(
                    CommandErrorKind::RegistryFailure,
                    "Command registry lock is poisoned",
                )
            })?;

            let registered = commands.get(&request.command).ok_or_else(|| {
                CommandError::new(
                    CommandErrorKind::CommandNotFound,
                    format!(
                        "Command '{}' is not registered",
                        request.command
                    ),
                )
                .for_command(request.command.clone())
            })?;

            (
                registered.descriptor.clone(),
                Arc::clone(&registered.handler),
            )
        };

        // Validate the caller-provided request context.
        if descriptor.requires_workspace
            && request.working_directory.is_none()
        {
            return Err(
                CommandError::new(
                    CommandErrorKind::InvalidResponse,
                    "This command requires an explicit workspace directory",
                )
                .for_command(request.command.clone()),
            );
        }

        // Side-effecting commands must not execute without an
        // explicitly configured authorization authority.
        if descriptor.mutability != CommandMutability::ReadOnly
            && self.inner.authorizer.is_none()
        {
            return Err(
                CommandError::new(
                    CommandErrorKind::AuthorizationUnavailable,
                    "No command authorization provider is configured",
                )
                .for_command(request.command.clone()),
            );
        }

        // Apply authorization when a provider is configured.
        if let Some(authorizer) = &self.inner.authorizer {
            if let Err(error) = authorizer.authorize(&descriptor, request) {
                let denial = CommandAuditEvent {
                    command: request.command.clone(),
                    correlation_id: request.correlation_id.clone(),
                    phase: CommandAuditPhase::Denied,
                    detail: Some(format!("{:?}", error.kind)),
                };

                if descriptor.audit_required {
                    let sink = self.inner.audit_sink.as_ref().ok_or_else(|| {
                        CommandError::new(
                            CommandErrorKind::AuditUnavailable,
                            "Mandatory audit provider is not configured",
                        )
                        .for_command(request.command.clone())
                    })?;

                    sink.record(&denial)?;
                } else if let Some(sink) = &self.inner.audit_sink {
                    sink.record(&denial)?;
                }

                return Err(
                    CommandError::new(
                        CommandErrorKind::AuthorizationDenied,
                        error.to_string(),
                    )
                    .for_command(request.command.clone()),
                );
            }
        }

        // Require audit infrastructure where the descriptor requires it.
        let audit_sink = if descriptor.audit_required {
            Some(
                self.inner.audit_sink.as_ref().ok_or_else(|| {
                    CommandError::new(
                        CommandErrorKind::AuditUnavailable,
                        "Mandatory audit provider is not configured",
                    )
                    .for_command(request.command.clone())
                })?,
            )
        } else {
            self.inner.audit_sink.as_ref()
        };

        let started = CommandAuditEvent {
            command: request.command.clone(),
            correlation_id: request.correlation_id.clone(),
            phase: CommandAuditPhase::Started,
            detail: None,
        };

        if let Some(sink) = audit_sink {
            sink.record(&started)?;
        }

        // Execute the actual registered implementation.
        // No successful result is fabricated here.
        let execution = handler.execute(request);

        match execution {
            Ok(result) => {
                let completed = CommandAuditEvent {
                    command: request.command.clone(),
                    correlation_id: request.correlation_id.clone(),
                    phase: CommandAuditPhase::Completed,
                    detail: Some(format!("{:?}", result.outcome)),
                };

                if let Some(sink) = audit_sink {
                    sink.record(&completed).map_err(|error| {
                        CommandError::new(
                            CommandErrorKind::AuditFailure,
                            error.to_string(),
                        )
                        .for_command(request.command.clone())
                        .may_have_completed()
                    })?;
                }

                Ok(CommandResponse {
                    command: request.command.clone(),
                    correlation_id: request.correlation_id.clone(),
                    result,
                })
            }

            Err(error) => {
                let failed = CommandAuditEvent {
                    command: request.command.clone(),
                    correlation_id: request.correlation_id.clone(),
                    phase: CommandAuditPhase::Failed,
                    detail: Some(format!("{:?}", error.kind)),
                };

                if let Some(sink) = audit_sink {
                    if let Err(audit_error) = sink.record(&failed) {
                        return Err(
                            CommandError::new(
                                error.kind,
                                format!(
                                    "{}; failure audit also failed: {}",
                                    error.message,
                                    audit_error.message
                                ),
                            )
                            .for_command(request.command.clone()),
                        );
                    }
                }

                Err(error.for_command(request.command.clone()))
            }
        }
    }
}

// ============================================================================
// PRELUDE
// ============================================================================

/// Common imports for command-service integrations.
pub mod prelude {
    pub use super::{
        CommandArtifact,
        CommandAuditEvent,
        CommandAuditPhase,
        CommandAuditSink,
        CommandAuthorizer,
        CommandDescriptor,
        CommandDiagnostic,
        CommandError,
        CommandErrorKind,
        CommandHandler,
        CommandId,
        CommandMutability,
        CommandOutcome,
        CommandRegistry,
        CommandRequest,
        CommandResponse,
        CommandResult,
        DiagnosticSeverity,
    };
}

// ============================================================================
// CONTRACT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    struct SuccessfulHandler;

    impl CommandHandler for SuccessfulHandler {
        fn execute(
            &self,
            _request: &CommandRequest,
        ) -> Result<CommandResult, CommandError> {
            Ok(CommandResult::completed())
        }
    }

    struct FailedHandler;

    impl CommandHandler for FailedHandler {
        fn execute(
            &self,
            _request: &CommandRequest,
        ) -> Result<CommandResult, CommandError> {
            Err(CommandError::new(
                CommandErrorKind::HandlerFailure,
                "Intentional test failure",
            ))
        }
    }

    struct AllowAuthorizer;

    impl CommandAuthorizer for AllowAuthorizer {
        fn authorize(
            &self,
            _descriptor: &CommandDescriptor,
            _request: &CommandRequest,
        ) -> Result<(), CommandError> {
            Ok(())
        }
    }

    struct RecordingAudit;

    impl CommandAuditSink for RecordingAudit {
        fn record(
            &self,
            _event: &CommandAuditEvent,
        ) -> Result<(), CommandError> {
            Ok(())
        }
    }

    fn test_id(value: &str) -> CommandId {
        CommandId::new(value).expect("valid test command identifier")
    }

    #[test]
    fn module_identity_is_stable() {
        assert_eq!(module_id(), "zamani.toolchain.commands");
        assert_eq!(api_version(), 1);
    }

    #[test]
    fn command_ids_support_future_namespaces() {
        let id = test_id("future.photonic.compute");

        assert_eq!(id.as_str(), "future.photonic.compute");
    }

    #[test]
    fn invalid_command_ids_are_rejected() {
        assert!(CommandId::new("").is_err());
        assert!(CommandId::new(" invalid").is_err());
        assert!(CommandId::new("build command").is_err());
        assert!(CommandId::new(".build").is_err());
        assert!(CommandId::new("build.").is_err());
    }

    #[test]
    fn registry_starts_empty() {
        let registry = CommandRegistry::new();

        assert!(registry.descriptors().unwrap().is_empty());
    }

    #[test]
    fn commands_can_be_registered_and_discovered() {
        let registry = CommandRegistry::new();

        let descriptor =
            CommandDescriptor::new(test_id("build"), "Build a project");

        registry
            .register(descriptor, SuccessfulHandler)
            .unwrap();

        assert!(registry.contains(&test_id("build")).unwrap());
        assert_eq!(registry.descriptors().unwrap().len(), 1);
    }

    #[test]
    fn duplicate_registration_is_rejected() {
        let registry = CommandRegistry::new();

        let descriptor =
            CommandDescriptor::new(test_id("build"), "Build a project");

        registry
            .register(descriptor.clone(), SuccessfulHandler)
            .unwrap();

        let error = registry
            .register(descriptor, SuccessfulHandler)
            .unwrap_err();

        assert_eq!(error.kind, CommandErrorKind::DuplicateCommand);
    }

    #[test]
    fn commands_can_be_unregistered() {
        let registry = CommandRegistry::new();

        registry
            .register(
                CommandDescriptor::new(test_id("build"), "Build"),
                SuccessfulHandler,
            )
            .unwrap();

        assert!(registry.unregister(&test_id("build")).unwrap());
        assert!(!registry.contains(&test_id("build")).unwrap());
    }

    #[test]
    fn unknown_commands_never_report_success() {
        let registry = CommandRegistry::new();
        let request = CommandRequest::new(test_id("unknown"));

        let error = registry.dispatch(&request).unwrap_err();

        assert_eq!(error.kind, CommandErrorKind::CommandNotFound);
    }

    #[test]
    fn successful_handler_returns_structured_response() {
        let registry = CommandRegistry::new();

        registry
            .register(
                CommandDescriptor::new(test_id("build"), "Build"),
                SuccessfulHandler,
            )
            .unwrap();

        let response = registry
            .dispatch(&CommandRequest::new(test_id("build")))
            .unwrap();

        assert_eq!(response.command.as_str(), "build");
        assert_eq!(response.result.outcome, CommandOutcome::Completed);
    }

    #[test]
    fn failed_handler_remains_a_failure() {
        let registry = CommandRegistry::new();

        registry
            .register(
                CommandDescriptor::new(test_id("build"), "Build"),
                FailedHandler,
            )
            .unwrap();

        let error = registry
            .dispatch(&CommandRequest::new(test_id("build")))
            .unwrap_err();

        assert_eq!(error.kind, CommandErrorKind::HandlerFailure);
    }

    #[test]
    fn mutating_commands_require_authorization() {
        let registry = CommandRegistry::new();

        registry
            .register(
                CommandDescriptor::new(test_id("clean"), "Clean")
                    .mutating(),
                SuccessfulHandler,
            )
            .unwrap();

        let error = registry
            .dispatch(&CommandRequest::new(test_id("clean")))
            .unwrap_err();

        assert_eq!(
            error.kind,
            CommandErrorKind::AuthorizationUnavailable
        );
    }

    #[test]
    fn authorized_mutating_commands_can_execute() {
        let registry = CommandRegistry::new()
            .with_authorizer(Arc::new(AllowAuthorizer));

        registry
            .register(
                CommandDescriptor::new(test_id("clean"), "Clean")
                    .mutating(),
                SuccessfulHandler,
            )
            .unwrap();

        assert!(registry
            .dispatch(&CommandRequest::new(test_id("clean")))
            .is_ok());
    }

    #[test]
    fn mandatory_audit_requires_a_sink() {
        let registry = CommandRegistry::new()
            .with_authorizer(Arc::new(AllowAuthorizer));

        registry
            .register(
                CommandDescriptor::new(test_id("publish"), "Publish")
                    .external_side_effects()
                    .requires_audit(),
                SuccessfulHandler,
            )
            .unwrap();

        let error = registry
            .dispatch(&CommandRequest::new(test_id("publish")))
            .unwrap_err();

        assert_eq!(error.kind, CommandErrorKind::AuditUnavailable);
    }

    #[test]
    fn mandatory_audit_and_authorization_can_be_integrated() {
        let registry = CommandRegistry::new()
            .with_authorizer(Arc::new(AllowAuthorizer))
            .with_audit_sink(Arc::new(RecordingAudit));

        registry
            .register(
                CommandDescriptor::new(test_id("publish"), "Publish")
                    .external_side_effects()
                    .requires_audit(),
                SuccessfulHandler,
            )
            .unwrap();

        assert!(registry
            .dispatch(&CommandRequest::new(test_id("publish")))
            .is_ok());
    }

    #[test]
    fn registry_clones_share_registration_state() {
        let first = CommandRegistry::new();
        let second = first.clone();

        first
            .register(
                CommandDescriptor::new(test_id("build"), "Build"),
                SuccessfulHandler,
            )
            .unwrap();

        assert!(second.contains(&test_id("build")).unwrap());
    }

    #[test]
    fn command_descriptors_are_returned_deterministically() {
        let registry = CommandRegistry::new();

        for name in ["test", "build", "clean"] {
            registry
                .register(
                    CommandDescriptor::new(test_id(name), name),
                    SuccessfulHandler,
                )
                .unwrap();
        }

        let names: Vec<String> = registry
            .descriptors()
            .unwrap()
            .iter()
            .map(|descriptor| descriptor.id.as_str().to_owned())
            .collect();

        assert_eq!(names, ["build", "clean", "test"]);
    }

    #[test]
    fn standard_command_names_are_open_identifiers() {
        for name in [
            standard_commands::BUILD,
            standard_commands::CHECK,
            standard_commands::CLEAN,
            standard_commands::RUN,
            standard_commands::TEST,
            standard_commands::BENCH,
            standard_commands::FORMAT,
            standard_commands::LINT,
            standard_commands::DOC,
            standard_commands::PACKAGE,
            standard_commands::INSTALL,
            standard_commands::PUBLISH,
            standard_commands::TARGET,
            standard_commands::TOOL,
            standard_commands::DOCTOR,
            standard_commands::DEPLOY,
            standard_commands::DEPS,
            standard_commands::INIT,
        ] {
            assert!(CommandId::new(name).is_ok());
        }
    }
}

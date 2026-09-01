//! Zamani Quantum IR — Dialect Extension Registry
//!
//! This module defines the dialect-level extension contract and registry for
//! the Zamani Quantum Intermediate Representation.
//!
//! # Architectural role
//!
//! `core::extension` owns the representation of an individual extension.
//! This module owns the relationship between extensions and dialects:
//!
//! ```text
//!                         Zamani Quantum IR
//!                                |
//!                    +-----------+-----------+
//!                    |                       |
//!              core::extension          dialect::extension
//!                    |                       |
//!             Extension object       Dialect / registry
//!                                            |
//!                           +----------------+----------------+
//!                           |                |                |
//!                       standard          pulse           vendor/custom
//! ```
//!
//! This module MUST NOT duplicate `Extension`, `ExtensionKey`, `ExtensionId`,
//! `QubitId`, or `PhysicalQubitId`.
//!
//! The canonical extension occurrence remains owned by:
//!
//! ```text
//! quantum::ir::core::extension
//! ```
//!
//! Logical and physical quantum identities remain owned by:
//!
//! ```text
//! quantum::ir::qubit
//! ```
//!
//! # Design goals
//!
//! The dialect system must support:
//!
//! - standard Zamani dialects;
//! - quantum gate dialects;
//! - dynamic-circuit dialects;
//! - pulse dialects;
//! - analog dialects;
//! - Hamiltonian dialects;
//! - annealing dialects;
//! - QUBO dialects;
//! - fermionic/bosonic dialects;
//! - continuous-variable dialects;
//! - measurement-based dialects;
//! - logical/fault-tolerant dialects;
//! - distributed quantum dialects;
//! - vendor dialects;
//! - experimental dialects;
//! - user-defined dialects;
//! - future quantum models not yet known;
//! - lossless unknown dialect preservation;
//! - deterministic registration;
//! - version negotiation;
//! - capability declarations;
//! - operation/type/attribute namespace ownership;
//! - concurrent read access without mutable global state;
//! - explicit conflict detection.
//!
//! # Critical rule
//!
//! A dialect is a semantic namespace, NOT an execution backend.
//!
//! A dialect MUST NOT:
//!
//! - execute quantum operations;
//! - access hardware;
//! - route qubits;
//! - schedule operations;
//! - perform optimization;
//! - simulate quantum states;
//! - communicate with a backend;
//! - access credentials;
//! - contain vendor SDK execution code.
//!
//! Those responsibilities belong downstream of the canonical IR.
//!
//! # "Write once, scale everywhere"
//!
//! Nothing in this file establishes a maximum:
//!
//! - number of dialects;
//! - number of extensions;
//! - number of operations;
//! - number of qubits;
//! - number of resources;
//! - number of targets;
//! - number of namespaces.
//!
//! Container capacities are implementation/resource constraints, not semantic
//! limits.
//!
//! No fixed quantum-machine size is encoded here.
//!
//! # Rust
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust.
//!
//! This module uses no `unsafe` code and explicitly forbids it.
//!
//! # Integration contract
//!
//! `core::extension`:
//!
//! - owns individual extension occurrences;
//! - owns extension keys;
//! - owns extension targets;
//! - owns extension payloads.
//!
//! `core::identity`:
//!
//! - owns `ExtensionId`;
//! - owns stable IR object identifiers.
//!
//! `qubit`:
//!
//! - owns `QubitId`;
//! - owns `PhysicalQubitId`.
//!
//! `dialect::mod`:
//!
//! - re-exports this module.
//!
//! `serialization`:
//!
//! - serializes registered and unknown dialect information;
//! - preserves unknown extension declarations.
//!
//! `validation`:
//!
//! - validates dialect references against the registry when a registry is
//!   available;
//! - does not assume that registration means hardware support.
//!
//! `resources::capability`:
//!
//! - provides actual target capability information.
//!
//! `hardware`:
//!
//! - decides whether a dialect/operation is executable on a target.
//!
//! `frontend`:
//!
//! - translates source-language constructs into canonical IR and dialect
//!   declarations.
//!
//! `backend`:
//!
//! - consumes dialect information after target selection.
//!
//! # No global registry
//!
//! This module deliberately does NOT use a global mutable singleton registry.
//!
//! Global mutable state would make deterministic compilation, testing,
//! parallel compilation, reproducibility, and distributed compilation harder.
//!
//! Callers own a `DialectRegistry` explicitly and pass it through the compiler
//! pipeline.
//!
//! # Thread safety
//!
//! The registry contains ordinary owned Rust data structures and has no
//! interior mutability or global state. Consequently it can be shared by
//! immutable reference between compilation stages when the surrounding
//! program requires it.
//!
//! No synchronization primitive is required inside this semantic layer.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::BTreeMap;
use std::fmt;
use std::hash::{Hash, Hasher};

use super::super::core::extension::{
    ExtensionKey,
    ExtensionNamespace,
    ExtensionVersion,
};

// =============================================================================
// Dialect version
// =============================================================================

/// Version of a dialect semantic contract.
///
/// Dialect versions are independent of:
///
/// - Zamani language version;
/// - Quantum IR version;
/// - compiler version;
/// - hardware version;
/// - backend version;
/// - calibration version.
///
/// A dialect version identifies the schema and semantic contract of the
/// dialect itself.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub struct DialectVersion {
    major: u16,
    minor: u16,
    patch: u16,
}

impl DialectVersion {
    /// Creates a dialect version.
    #[must_use]
    pub const fn new(
        major: u16,
        minor: u16,
        patch: u16,
    ) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Returns the major component.
    #[must_use]
    pub const fn major(self) -> u16 {
        self.major
    }

    /// Returns the minor component.
    #[must_use]
    pub const fn minor(self) -> u16 {
        self.minor
    }

    /// Returns the patch component.
    #[must_use]
    pub const fn patch(self) -> u16 {
        self.patch
    }

    /// Returns whether both versions have the same major contract.
    #[must_use]
    pub const fn same_major(
        self,
        other: Self,
    ) -> bool {
        self.major == other.major
    }

    /// Returns whether versions are exactly equal.
    #[must_use]
    pub const fn is_exactly(
        self,
        other: Self,
    ) -> bool {
        self.major == other.major
            && self.minor == other.minor
            && self.patch == other.patch
    }

    /// Returns whether this version can consume `other`.
    ///
    /// Compatibility is conservative:
    ///
    /// - same major;
    /// - producer minor <= consumer minor;
    /// - when minors match, producer patch <= consumer patch.
    #[must_use]
    pub const fn supports(
        self,
        other: Self,
    ) -> bool {
        other.major == self.major
            && other.minor <= self.minor
            && (
                other.minor < self.minor
                    || other.patch <= self.patch
            )
    }
}

impl Default for DialectVersion {
    fn default() -> Self {
        Self::new(1, 0, 0)
    }
}

impl fmt::Display for DialectVersion {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{}.{}.{}",
            self.major,
            self.minor,
            self.patch
        )
    }
}

// =============================================================================
// Dialect name
// =============================================================================

/// Validated dialect name.
///
/// A dialect name is a local semantic identifier. Hierarchical ownership is
/// represented by `DialectNamespace`.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub struct DialectName(String);

impl DialectName {
    /// Creates a dialect name.
    pub fn new<S>(
        name: S,
    ) -> Result<Self, DialectError>
    where
        S: Into<String>,
    {
        let name = name.into();

        validate_identifier(
            &name,
            "dialect name",
        )?;

        Ok(Self(name))
    }

    /// Returns the name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the name.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for DialectName {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// =============================================================================
// Dialect identifier
// =============================================================================

/// Globally meaningful semantic identity of a dialect.
///
/// The identity is:
///
/// ```text
/// namespace.name
/// ```
///
/// Version is intentionally separate from identity so multiple versions of a
/// dialect contract can be negotiated without creating a different namespace.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub struct DialectId {
    namespace: ExtensionNamespace,
    name: DialectName,
}

impl DialectId {
    /// Creates a dialect identifier.
    pub fn new<N, S>(
        namespace: N,
        name: S,
    ) -> Result<Self, DialectError>
    where
        N: Into<String>,
        S: Into<String>,
    {
        Ok(Self {
            namespace: ExtensionNamespace::new(namespace)
                .map_err(DialectError::InvalidNamespace)?,
            name: DialectName::new(name)?,
        })
    }

    /// Returns the namespace.
    #[must_use]
    pub fn namespace(&self) -> &ExtensionNamespace {
        &self.namespace
    }

    /// Returns the local dialect name.
    #[must_use]
    pub fn name(&self) -> &DialectName {
        &self.name
    }

    /// Returns the canonical dialect identifier.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        format!(
            "{}.{}",
            self.namespace,
            self.name
        )
    }
}

impl fmt::Display for DialectId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{}.{}",
            self.namespace,
            self.name
        )
    }
}

// =============================================================================
// Dialect kind
// =============================================================================

/// Broad classification of a dialect.
///
/// This is descriptive metadata, not an execution policy.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub enum DialectKind {
    /// Core Zamani semantic constructs.
    Core,

    /// Standard quantum operations.
    Quantum,

    /// Classical computation associated with quantum programs.
    Classical,

    /// Dynamic control flow and feedback.
    Control,

    /// Pulse-level semantics.
    Pulse,

    /// Analog/Hamiltonian semantics.
    Analog,

    /// Annealing/problem-Hamiltonian semantics.
    Annealing,

    /// Fermionic computation.
    Fermionic,

    /// Bosonic computation.
    Bosonic,

    /// Continuous-variable computation.
    ContinuousVariable,

    /// Measurement-based quantum computation.
    MeasurementBased,

    /// Logical/fault-tolerant quantum computation.
    FaultTolerant,

    /// Distributed quantum computation.
    Distributed,

    /// Hardware-neutral resource declarations.
    Resource,

    /// Compiler metadata.
    Metadata,

    /// Experimental/research semantics.
    Experimental,

    /// Vendor-defined semantics.
    Vendor,

    /// User-defined semantics.
    User,

    /// A dialect whose category is intentionally not known.
    Unknown,
}

// =============================================================================
// Dialect dependency
// =============================================================================

/// Dependency on another dialect.
///
/// Dependencies express semantic prerequisites only. They do not imply that
/// the dependency is executable or that a backend supports either dialect.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub struct DialectDependency {
    dialect: DialectId,
    minimum_version: DialectVersion,
}

impl DialectDependency {
    /// Creates a dialect dependency.
    #[must_use]
    pub const fn new(
        dialect: DialectId,
        minimum_version: DialectVersion,
    ) -> Self {
        Self {
            dialect,
            minimum_version,
        }
    }

    /// Returns the dependent dialect.
    #[must_use]
    pub fn dialect(&self) -> &DialectId {
        &self.dialect
    }

    /// Returns the minimum compatible version.
    #[must_use]
    pub const fn minimum_version(
        &self,
    ) -> DialectVersion {
        self.minimum_version
    }
}

// =============================================================================
// Dialect extension declaration
// =============================================================================

/// Declaration of an extension contract belonging to a dialect.
///
/// This is intentionally metadata about the extension contract. The actual
/// extension occurrence remains owned by `core::extension::Extension`.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
)]
pub struct DialectExtension {
    key: ExtensionKey,
}

impl DialectExtension {
    /// Creates a dialect extension declaration.
    #[must_use]
    pub const fn new(
        key: ExtensionKey,
    ) -> Self {
        Self { key }
    }

    /// Returns the extension contract key.
    #[must_use]
    pub fn key(&self) -> &ExtensionKey {
        &self.key
    }

    /// Returns the extension namespace.
    #[must_use]
    pub fn namespace(&self) -> &ExtensionNamespace {
        self.key.namespace()
    }

    /// Returns the extension version.
    #[must_use]
    pub const fn version(
        &self,
    ) -> ExtensionVersion {
        let version = self.key.version();

        ExtensionVersion::new(
            version.major(),
            version.minor(),
            version.patch(),
        )
    }
}

// =============================================================================
// Dialect declaration
// =============================================================================

/// Immutable declaration of a dialect contract.
///
/// A declaration contains no executable implementation.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
)]
pub struct Dialect {
    id: DialectId,
    version: DialectVersion,
    kind: DialectKind,
    dependencies: Vec<DialectDependency>,
    extensions: BTreeMap<ExtensionKey, DialectExtension>,
}

impl Dialect {
    /// Creates an empty dialect declaration.
    #[must_use]
    pub fn new(
        id: DialectId,
        version: DialectVersion,
        kind: DialectKind,
    ) -> Self {
        Self {
            id,
            version,
            kind,
            dependencies: Vec::new(),
            extensions: BTreeMap::new(),
        }
    }

    /// Returns the dialect identity.
    #[must_use]
    pub fn id(&self) -> &DialectId {
        &self.id
    }

    /// Returns the dialect version.
    #[must_use]
    pub const fn version(
        &self,
    ) -> DialectVersion {
        self.version
    }

    /// Returns the dialect kind.
    #[must_use]
    pub const fn kind(
        &self,
    ) -> DialectKind {
        self.kind
    }

    /// Returns dependencies in deterministic order.
    #[must_use]
    pub fn dependencies(
        &self,
    ) -> &[DialectDependency] {
        &self.dependencies
    }

    /// Returns all extension declarations.
    #[must_use]
    pub fn extensions(
        &self,
    ) -> impl Iterator<Item = &DialectExtension> {
        self.extensions.values()
    }

    /// Returns the number of declared extensions.
    #[must_use]
    pub fn extension_count(&self) -> usize {
        self.extensions.len()
    }

    /// Adds a dialect dependency.
    ///
    /// Duplicate dependencies are retained only once.
    pub fn add_dependency(
        &mut self,
        dependency: DialectDependency,
    ) {
        if !self.dependencies.contains(&dependency) {
            self.dependencies.push(dependency);
            self.dependencies.sort();
        }
    }

    /// Adds an extension declaration.
    ///
    /// Re-registering the exact same key is idempotent.
    pub fn add_extension(
        &mut self,
        extension: DialectExtension,
    ) -> Result<(), DialectError> {
        let key = extension.key().clone();

        match self.extensions.get(&key) {
            Some(existing) if existing == &extension => {
                Ok(())
            }
            Some(_) => Err(DialectError::ConflictingExtension {
                key,
            }),
            None => {
                self.extensions.insert(key, extension);
                Ok(())
            }
        }
    }

    /// Returns whether this dialect can consume the supplied version.
    #[must_use]
    pub const fn supports_version(
        &self,
        version: DialectVersion,
    ) -> bool {
        self.version.supports(version)
    }

    /// Validates internal structural invariants.
    pub fn validate(
        &self,
    ) -> Result<(), DialectError> {
        if self.extensions.values().any(|extension| {
            extension.namespace() != self.id.namespace()
        }) {
            return Err(
                DialectError::ExtensionNamespaceMismatch {
                    dialect: self.id.clone(),
                },
            );
        }

        if self.dependencies.iter().any(|dependency| {
            dependency.dialect() == &self.id
        }) {
            return Err(
                DialectError::SelfDependency {
                    dialect: self.id.clone(),
                },
            );
        }

        Ok(())
    }
}

// =============================================================================
// Registry
// =============================================================================

/// Deterministic explicit dialect registry.
///
/// The registry contains semantic declarations only.
///
/// There is no global mutable registry and no backend implementation registry.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DialectRegistry {
    dialects: BTreeMap<DialectId, Dialect>,
}

impl DialectRegistry {
    /// Creates an empty registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of registered dialects.
    #[must_use]
    pub fn len(&self) -> usize {
        self.dialects.len()
    }

    /// Returns whether the registry contains no dialects.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.dialects.is_empty()
    }

    /// Registers a dialect.
    ///
    /// Registration is:
    ///
    /// - deterministic;
    /// - idempotent for identical declarations;
    /// - rejecting of conflicting declarations.
    pub fn register(
        &mut self,
        dialect: Dialect,
    ) -> Result<(), DialectError> {
        dialect.validate()?;

        let id = dialect.id().clone();

        match self.dialects.get(&id) {
            Some(existing) if existing == &dialect => Ok(()),
            Some(existing) => {
                Err(DialectError::ConflictingDialect {
                    id,
                    existing_version: existing.version(),
                    incoming_version: dialect.version(),
                })
            }
            None => {
                self.dialects.insert(id, dialect);
                Ok(())
            }
        }
    }

    /// Returns a dialect by identity.
    #[must_use]
    pub fn get(
        &self,
        id: &DialectId,
    ) -> Option<&Dialect> {
        self.dialects.get(id)
    }

    /// Returns a mutable dialect by identity.
    ///
    /// This is intentionally explicit. The registry does not expose global
    /// mutation or interior mutability.
    pub fn get_mut(
        &mut self,
        id: &DialectId,
    ) -> Option<&mut Dialect> {
        self.dialects.get_mut(id)
    }

    /// Returns whether a dialect is registered.
    #[must_use]
    pub fn contains(
        &self,
        id: &DialectId,
    ) -> bool {
        self.dialects.contains_key(id)
    }

    /// Iterates over dialects in deterministic order.
    #[must_use]
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&DialectId, &Dialect)> {
        self.dialects.iter()
    }

    /// Checks whether the registry can satisfy a dialect dependency.
    #[must_use]
    pub fn satisfies(
        &self,
        dependency: &DialectDependency,
    ) -> bool {
        self.get(dependency.dialect())
            .map(|dialect| {
                dialect.supports_version(
                    dependency.minimum_version(),
                )
            })
            .unwrap_or(false)
    }

    /// Validates all registered dialects and their dependencies.
    ///
    /// This performs structural validation only. It does not validate hardware
    /// support.
    pub fn validate(
        &self,
    ) -> Result<(), DialectError> {
        for dialect in self.dialects.values() {
            dialect.validate()?;

            for dependency in dialect.dependencies() {
                if !self.satisfies(dependency) {
                    return Err(
                        DialectError::UnsatisfiedDependency {
                            dialect: dialect.id().clone(),
                            dependency: dependency.clone(),
                        },
                    );
                }
            }
        }

        Ok(())
    }

    /// Returns a deterministic registry fingerprint.
    ///
    /// This is a structural fingerprint suitable for detecting registry
    /// changes within a process or compilation pipeline.
    ///
    /// It deliberately does not claim to be a cryptographic hash.
    #[must_use]
    pub fn fingerprint(&self) -> u64 {
        let mut hasher = StableFingerprintHasher::default();

        self.len().hash(&mut hasher);

        for (id, dialect) in &self.dialects {
            id.hash(&mut hasher);
            dialect.hash(&mut hasher);
        }

        hasher.finish()
    }
}

// =============================================================================
// Dialect resolution
// =============================================================================

/// Result of resolving a dialect dependency.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub enum DialectResolution<'a> {
    /// A compatible registered dialect was found.
    Resolved(&'a Dialect),

    /// The dialect is not registered.
    Unknown {
        id: &'a DialectId,
    },

    /// The dialect exists but its version is incompatible.
    Incompatible {
        dialect: &'a Dialect,
        required: DialectVersion,
    },
}

impl<'a> DialectResolution<'a> {
    /// Returns whether the resolution succeeded.
    #[must_use]
    pub const fn is_resolved(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::Resolved(_)
        )
    }

    /// Returns the resolved dialect when successful.
    #[must_use]
    pub const fn dialect(
        &self,
    ) -> Option<&'a Dialect> {
        match self {
            Self::Resolved(dialect)
            | Self::Incompatible {
                dialect,
                ..
            } => Some(dialect),

            Self::Unknown { .. } => None,
        }
    }
}

impl DialectRegistry {
    /// Resolves a dialect dependency without mutating the registry.
    #[must_use]
    pub fn resolve<'a>(
        &'a self,
        dependency: &DialectDependency,
    ) -> DialectResolution<'a> {
        match self.get(dependency.dialect()) {
            None => DialectResolution::Unknown {
                id: dependency.dialect(),
            },

            Some(dialect)
                if dialect.supports_version(
                    dependency.minimum_version(),
                ) =>
            {
                DialectResolution::Resolved(dialect)
            }

            Some(dialect) => {
                DialectResolution::Incompatible {
                    dialect,
                    required: dependency.minimum_version(),
                }
            }
        }
    }
}

// =============================================================================
// Registry merge
// =============================================================================

/// Merges two dialect registries without silently overwriting declarations.
///
/// Identical declarations are retained.
///
/// Conflicting declarations are rejected.
pub fn merge_registries(
    left: &DialectRegistry,
    right: &DialectRegistry,
) -> Result<DialectRegistry, DialectError> {
    let mut merged = left.clone();

    for dialect in right.dialects.values() {
        merged.register(dialect.clone())?;
    }

    Ok(merged)
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by dialect construction, registration, validation, and
/// resolution.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub enum DialectError {
    /// Namespace was invalid.
    InvalidNamespace(
        super::super::core::extension::ExtensionError,
    ),

    /// Dialect name was invalid.
    InvalidName(String),

    /// A dialect was registered with conflicting declarations.
    ConflictingDialect {
        id: DialectId,
        existing_version: DialectVersion,
        incoming_version: DialectVersion,
    },

    /// An extension conflicts with an existing extension declaration.
    ConflictingExtension {
        key: ExtensionKey,
    },

    /// An extension belongs to a different namespace.
    ExtensionNamespaceMismatch {
        dialect: DialectId,
    },

    /// A dialect depends upon itself.
    SelfDependency {
        dialect: DialectId,
    },

    /// A required dialect is not available at a compatible version.
    UnsatisfiedDependency {
        dialect: DialectId,
        dependency: DialectDependency,
    },
}

impl fmt::Display for DialectError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidNamespace(error) => {
                write!(
                    formatter,
                    "invalid dialect namespace: {error}"
                )
            }

            Self::InvalidName(name) => {
                write!(
                    formatter,
                    "invalid dialect name: {name:?}"
                )
            }

            Self::ConflictingDialect {
                id,
                existing_version,
                incoming_version,
            } => {
                write!(
                    formatter,
                    "conflicting dialect {id}: \
                     existing version {existing_version}, \
                     incoming version {incoming_version}"
                )
            }

            Self::ConflictingExtension { key } => {
                write!(
                    formatter,
                    "conflicting extension declaration: {key}"
                )
            }

            Self::ExtensionNamespaceMismatch {
                dialect,
            } => {
                write!(
                    formatter,
                    "extension namespace does not match dialect \
                     namespace for {dialect}"
                )
            }

            Self::SelfDependency { dialect } => {
                write!(
                    formatter,
                    "dialect {dialect} cannot depend on itself"
                )
            }

            Self::UnsatisfiedDependency {
                dialect,
                dependency,
            } => {
                write!(
                    formatter,
                    "dialect {dialect} has unsatisfied dependency \
                     {}@{}",
                    dependency.dialect(),
                    dependency.minimum_version()
                )
            }
        }
    }
}

impl std::error::Error for DialectError {}

// =============================================================================
// Identifier validation
// =============================================================================

fn validate_identifier(
    value: &str,
    kind: &str,
) -> Result<(), DialectError> {
    if value.is_empty() {
        return Err(DialectError::InvalidName(
            format!("{kind} must not be empty"),
        ));
    }

    let mut characters = value.chars();

    let first = match characters.next() {
        Some(character) => character,
        None => {
            return Err(DialectError::InvalidName(
                format!("{kind} must not be empty"),
            ));
        }
    };

    if !is_identifier_start(first) {
        return Err(DialectError::InvalidName(
            format!(
                "{kind} must start with an ASCII letter or '_'"
            ),
        ));
    }

    if characters.any(|character| {
        !is_identifier_continue(character)
    }) {
        return Err(DialectError::InvalidName(
            format!(
                "{kind} contains an invalid character"
            ),
        ));
    }

    Ok(())
}

fn is_identifier_start(
    character: char,
) -> bool {
    character.is_ascii_alphabetic()
        || character == '_'
}

fn is_identifier_continue(
    character: char,
) -> bool {
    character.is_ascii_alphanumeric()
        || character == '_'
}

// =============================================================================
// Deterministic structural fingerprint hasher
// =============================================================================

/// Small deterministic non-cryptographic hasher.
///
/// This exists only so the registry can expose a stable structural fingerprint
/// without depending on randomized `DefaultHasher` state.
///
/// It is NOT intended for security, signatures, or cryptographic identity.
#[derive(Debug, Clone, Copy)]
struct StableFingerprintHasher {
    state: u64,
}

impl Default for StableFingerprintHasher {
    fn default() -> Self {
        Self {
            state: 0xcbf29ce484222325,
        }
    }
}

impl Hasher for StableFingerprintHasher {
    fn finish(&self) -> u64 {
        self.state
    }

    fn write(
        &mut self,
        bytes: &[u8],
    ) {
        for byte in bytes {
            self.state ^= u64::from(*byte);
            self.state = self
                .state
                .wrapping_mul(0x100000001b3);
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn dialect(
        namespace: &str,
        name: &str,
    ) -> Dialect {
        Dialect::new(
            DialectId::new(
                namespace,
                name,
            )
            .expect("valid dialect id"),
            DialectVersion::new(
                1,
                0,
                0,
            ),
            DialectKind::Quantum,
        )
    }

    #[test]
    fn dialect_identifier_is_deterministic() {
        let id = DialectId::new(
            "zamani",
            "quantum",
        )
        .expect("valid id");

        assert_eq!(
            id.to_string(),
            "zamani.quantum"
        );
        assert_eq!(
            id.qualified_name(),
            "zamani.quantum"
        );
    }

    #[test]
    fn dialect_version_is_conservative() {
        let current =
            DialectVersion::new(1, 2, 3);

        assert!(
            current.supports(
                DialectVersion::new(1, 2, 3)
            )
        );

        assert!(
            current.supports(
                DialectVersion::new(1, 2, 2)
            )
        );

        assert!(
            current.supports(
                DialectVersion::new(1, 1, 99)
            )
        );

        assert!(
            !current.supports(
                DialectVersion::new(1, 3, 0)
            )
        );

        assert!(
            !current.supports(
                DialectVersion::new(2, 0, 0)
            )
        );
    }

    #[test]
    fn registry_registration_is_idempotent() {
        let mut registry =
            DialectRegistry::new();

        let quantum =
            dialect("zamani", "quantum");

        registry
            .register(quantum.clone())
            .expect("first registration");

        registry
            .register(quantum)
            .expect("identical registration");

        assert_eq!(
            registry.len(),
            1
        );
    }

    #[test]
    fn registry_rejects_conflicting_dialect() {
        let mut registry =
            DialectRegistry::new();

        registry
            .register(
                dialect(
                    "zamani",
                    "quantum",
                ),
            )
            .expect("registration");

        let conflicting =
            Dialect::new(
                DialectId::new(
                    "zamani",
                    "quantum",
                )
                .expect("valid id"),
                DialectVersion::new(
                    2,
                    0,
                    0,
                ),
                DialectKind::Quantum,
            );

        assert!(
            registry
                .register(conflicting)
                .is_err()
        );
    }

    #[test]
    fn extension_must_belong_to_dialect_namespace() {
        let mut dialect =
            dialect(
                "zamani",
                "quantum",
            );

        let extension =
            DialectExtension::new(
                ExtensionKey::new(
                    "other",
                    "operation",
                    ExtensionVersion::new(
                        1,
                        0,
                        0,
                    ),
                )
                .expect("valid extension key"),
            );

        dialect
            .add_extension(extension)
            .expect("declaration itself is valid");

        assert!(
            matches!(
                dialect.validate(),
                Err(
                    DialectError::ExtensionNamespaceMismatch {
                        ..
                    }
                )
            )
        );
    }

    #[test]
    fn dependencies_are_deterministic() {
        let base =
            dialect(
                "zamani",
                "base",
            );

        let quantum_id =
            DialectId::new(
                "zamani",
                "quantum",
            )
            .expect("valid id");

        let mut quantum =
            dialect(
                "zamani",
                "quantum",
            );

        quantum.add_dependency(
            DialectDependency::new(
                quantum_id.clone(),
                DialectVersion::new(
                    1,
                    0,
                    0,
                ),
            ),
        );

        assert_eq!(
            quantum.dependencies().len(),
            1
        );

        let mut registry =
            DialectRegistry::new();

        registry
            .register(base)
            .expect("base registration");

        registry
            .register(quantum)
            .expect("quantum registration");

        assert!(
            registry.validate().is_err()
        );
    }

    #[test]
    fn registry_fingerprint_changes_when_registry_changes() {
        let mut registry =
            DialectRegistry::new();

        let before =
            registry.fingerprint();

        registry
            .register(
                dialect(
                    "zamani",
                    "quantum",
                ),
            )
            .expect("registration");

        let after =
            registry.fingerprint();

        assert_ne!(
            before,
            after
        );
    }

    #[test]
    fn merge_is_conservative() {
        let mut left =
            DialectRegistry::new();

        left.register(
            dialect(
                "zamani",
                "quantum",
            ),
        )
        .expect("left registration");

        let mut right =
            DialectRegistry::new();

        right.register(
            dialect(
                "zamani",
                "pulse",
            ),
        )
        .expect("right registration");

        let merged =
            merge_registries(
                &left,
                &right,
            )
            .expect("merge");

        assert_eq!(
            merged.len(),
            2
        );
    }

    #[test]
    fn unknown_dialect_is_explicit() {
        let registry =
            DialectRegistry::new();

        let id =
            DialectId::new(
                "future",
                "quantum",
            )
            .expect("valid id");

        let dependency =
            DialectDependency::new(
                id.clone(),
                DialectVersion::new(
                    1,
                    0,
                    0,
                ),
            );

        match registry.resolve(
            &dependency
        ) {
            DialectResolution::Unknown {
                id: unknown,
            } => {
                assert_eq!(
                    unknown,
                    &id
                );
            }

            other => {
                panic!(
                    "unexpected resolution: {other:?}"
                );
            }
        }
    }
}
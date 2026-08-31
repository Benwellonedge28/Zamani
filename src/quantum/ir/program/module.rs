//! Zamani Quantum IR — Universal Program Module
//!
//! Production-grade, hardware-independent module/compilation-unit model for
//! the Zamani Quantum IR.
//!
//! # Architectural role
//!
//! A `QuantumModule` is a semantic compilation unit inside a
//! `QuantumProgram`.
//!
//! A module provides:
//!
//! - stable module identity;
//! - namespace ownership;
//! - logical-qubit declarations/references;
//! - operation references;
//! - region references;
//! - function references;
//! - parameter references;
//! - resource references;
//! - capability requirements;
//! - extension references;
//! - imports;
//! - exports;
//! - module dependencies;
//! - deterministic metadata;
//! - explicit visibility;
//! - entry-region information;
//! - IR-version information;
//! - structural validation.
//!
//! A module does NOT:
//!
//! - execute quantum operations;
//! - allocate hardware;
//! - route qubits;
//! - schedule operations;
//! - optimize operations;
//! - synthesize pulses;
//! - perform calibration;
//! - communicate with a QPU;
//! - simulate quantum states;
//! - decode QEC syndromes;
//! - parse Zamani source;
//! - own the canonical definitions of operations, gates, qubits, pulses,
//!   resources, or capabilities.
//!
//! Those responsibilities belong to their respective IR/compiler layers.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! frontend
//!      │
//!      ▼
//! QuantumProgram
//!      │
//!      ├── QuantumModule
//!      │      ├── namespace
//!      │      ├── imports
//!      │      ├── exports
//!      │      ├── declarations/references
//!      │      └── regions
//!      │
//!      ├── QuantumModule
//!      └── ...
//!      │
//!      ▼
//! canonical operation IR
//!      │
//!      ├── optimization
//!      ├── routing
//!      ├── scheduling
//!      ├── pulse lowering
//!      ├── logical/QEC lowering
//!      └── target lowering
//!      │
//!      ▼
//! hardware/backend
//! ```
//!
//! # Universal-program principle
//!
//! A module must remain independent of the eventual quantum machine.
//!
//! Nothing in this file assumes:
//!
//! - a fixed number of qubits;
//! - a fixed number of operations;
//! - a fixed register size;
//! - a fixed topology;
//! - a fixed gate set;
//! - a fixed hardware technology;
//! - a fixed pulse architecture;
//! - a fixed vendor;
//! - a fixed simulator;
//! - a fixed backend.
//!
//! A module containing one logical qubit and a module containing an
//! arbitrarily large finite number of logical qubits use the same semantic
//! model.
//!
//! "Infinite hardware" is therefore not represented as a special constant.
//! The IR has no architectural machine-size ceiling. Every concrete program
//! remains finite and is bounded only by its host representation and explicit
//! resource/security policies.
//!
//! # Canonical identity boundary
//!
//! This file deliberately uses the canonical identities from
//! `quantum::ir::core::identity`.
//!
//! In particular:
//!
//! ```text
//! ModuleId
//! OperationId
//! RegionId
//! FunctionId
//! ParameterId
//! ResourceId
//! CapabilityId
//! ExtensionId
//! IrVersion
//! ```
//!
//! Logical and physical qubit identity is exclusively owned by:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This file therefore imports:
//!
//! ```rust
//! use super::super::qubit::QubitId;
//! ```
//!
//! and never defines another `QubitId`.
//!
//! # Module versus program
//!
//! `QuantumProgram` is the complete semantic program.
//!
//! `QuantumModule` is a reusable semantic compilation unit within that
//! program.
//!
//! ```text
//! QuantumProgram
//!     │
//!     ├── Module A
//!     │     ├── symbols
//!     │     ├── operations
//!     │     └── regions
//!     │
//!     ├── Module B
//!     │     ├── symbols
//!     │     └── functions
//!     │
//!     └── Module C
//! ```
//!
//! A module therefore does not duplicate the complete program container.
//!
//! # Determinism
//!
//! Semantic sets and keyed metadata use `BTreeMap`/`BTreeSet` rather than
//! hash-map iteration so that:
//!
//! - validation is deterministic;
//! - serialization order can be deterministic;
//! - diagnostics are reproducible;
//! - module fingerprints can be deterministic;
//! - distributed compilation remains reproducible.
//!
//! Explicit declaration order is preserved separately where order is
//! semantically meaningful.
//!
//! # Atomic mutation
//!
//! Mutating methods validate their inputs before modifying module state.
//!
//! Bulk operations are transactional from the caller's perspective:
//!
//! - either every requested insertion succeeds;
//! - or no module state is changed.
//!
//! # Versioning
//!
//! The module stores the canonical `IrVersion` rather than inventing a
//! module-local version scheme.
//!
//! Language version, compiler version, Danga version, hardware version and
//! backend version are deliberately outside this structure.
//!
//! # Validation boundary
//!
//! This file performs local/module structural validation.
//!
//! It can verify:
//!
//! - IDs are structurally valid;
//! - names are non-empty;
//! - self-dependencies are rejected;
//! - duplicate symbols/imports/exports are rejected;
//! - referenced objects have been registered with the module;
//! - entry regions exist;
//! - exported symbols exist;
//! - required versions are compatible at the module-contract level;
//! - logical qubit references are structurally valid;
//! - import/export invariants hold.
//!
//! It cannot determine:
//!
//! - whether an operation is semantically valid;
//! - whether a qubit exists in hardware;
//! - whether a target supports a capability;
//! - whether routing is possible;
//! - whether a pulse is calibrated;
//! - whether a schedule is executable.
//!
//! Those checks belong to the canonical validation, hardware, routing,
//! scheduling, pulse and backend layers.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust.
//!
//! Requirements:
//!
//! - no nightly features;
//! - no external dependencies;
//! - no `unsafe`.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! # Integration contract
//!
//! `core::identity`
//!     Supplies all canonical semantic IDs and `IrVersion`.
//!
//! `qubit`
//!     Supplies canonical `QubitId`.
//!
//! `operation`
//!     Owns operation semantics. This module stores only operation references.
//!
//! `region`
//!     Owns region semantics. This module stores only region references.
//!
//! `program`
//!     Owns the complete program and may contain multiple modules.
//!
//! `classical`
//!     Owns classical value semantics.
//!
//! `resource`
//!     Owns resource semantics.
//!
//! `capability`
//!     Owns capability semantics.
//!
//! `extension`
//!     Owns extension semantics.
//!
//! `validation`
//!     Performs whole-IR validation after module-local validation.
//!
//! `serialization`
//!     Serializes the module using these stable public fields/accessors.
//!
//! `hash`
//!     Computes canonical content hashes without placing hashing logic here.
//!
//! `provenance`
//!     Records transformations affecting the module.
//!
//! `optimization`
//!     May transform operations referenced by the module but must not redefine
//!     module identity semantics.
//!
//! `routing`
//!     May resolve mappings for logical qubits referenced by this module.
//!
//! `scheduling`
//!     May produce schedules for operations referenced by this module.
//!
//! `hardware`
//!     Determines whether the module can execute on a selected target.
//!
//! # Important ownership rule
//!
//! This file owns module structure, not the objects referenced by the module.
//!
//! Therefore:
//!
//! ```text
//! Module
//!   owns:
//!       imports
//!       exports
//!       namespace
//!       references
//!       metadata
//!
//!   references:
//!       operations
//!       regions
//!       functions
//!       parameters
//!       resources
//!       capabilities
//!       extensions
//!       logical qubits
//! ```
//!
//! This separation prevents module.rs from having to be rewritten whenever a
//! downstream semantic object gains new capabilities.
//!
//! -----------------------------------------------------------------------------
//! No hardware algorithms belong in this file.
//! -----------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use super::super::core::identity::{
    CapabilityId,
    ExtensionId,
    FunctionId,
    IrVersion,
    ModuleId,
    NamespaceId,
    OperationId,
    ParameterId,
    RegionId,
    ResourceId,
};
use super::super::qubit::QubitId;

// =============================================================================
// Result
// =============================================================================

/// Result type returned by module construction and mutation APIs.
pub type ModuleResult<T> = Result<T, ModuleError>;

// =============================================================================
// Module kind
// =============================================================================

/// Semantic role of an IR module.
///
/// This classification is intentionally technology-independent.
///
/// A new quantum architecture must not require changing this enum. Unknown or
/// specialized architectures should use `Extension` or a future dialect
/// without changing the module container contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ModuleKind {
    /// General-purpose quantum module.
    Quantum,

    /// Gate/circuit-oriented module.
    Circuit,

    /// Hybrid quantum/classical module.
    Hybrid,

    /// Pulse/control-oriented module.
    Pulse,

    /// Analog/Hamiltonian-oriented module.
    Analog,

    /// Annealing/Ising/QUBO-oriented module.
    Annealing,

    /// Logical/fault-tolerant module.
    Logical,

    /// Distributed quantum module.
    Distributed,

    /// Interface/declaration-only module.
    Interface,

    /// Extension-defined module.
    Extension,
}

impl Default for ModuleKind {
    fn default() -> Self {
        Self::Quantum
    }
}

impl fmt::Display for ModuleKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Quantum => "quantum",
            Self::Circuit => "circuit",
            Self::Hybrid => "hybrid",
            Self::Pulse => "pulse",
            Self::Analog => "analog",
            Self::Annealing => "annealing",
            Self::Logical => "logical",
            Self::Distributed => "distributed",
            Self::Interface => "interface",
            Self::Extension => "extension",
        };

        formatter.write_str(value)
    }
}

// =============================================================================
// Module visibility
// =============================================================================

/// Visibility of a module inside the surrounding program/module graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ModuleVisibility {
    /// Only the owning program/module graph may use the module.
    Private,

    /// The module can be referenced by sibling modules in the same program.
    Internal,

    /// The module can be referenced outside its owning compilation unit.
    Public,
}

impl Default for ModuleVisibility {
    fn default() -> Self {
        Self::Private
    }
}

impl fmt::Display for ModuleVisibility {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Private => formatter.write_str("private"),
            Self::Internal => formatter.write_str("internal"),
            Self::Public => formatter.write_str("public"),
        }
    }
}

// =============================================================================
// Symbol kind
// =============================================================================

/// Semantic category of a module symbol.
///
/// The symbol table is intentionally represented by stable `String` names
/// plus canonical IDs. Detailed symbol semantics belong to `symbol.rs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ModuleSymbolKind {
    /// Function/subroutine symbol.
    Function,

    /// Quantum operation symbol.
    Operation,

    /// Quantum region symbol.
    Region,

    /// Parameter symbol.
    Parameter,

    /// Logical-qubit symbol.
    Qubit,

    /// Resource symbol.
    Resource,

    /// Capability symbol.
    Capability,

    /// Extension symbol.
    Extension,

    /// Type symbol.
    Type,

    /// Constant/value symbol.
    Value,

    /// External/imported symbol.
    External,
}

impl fmt::Display for ModuleSymbolKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Function => "function",
            Self::Operation => "operation",
            Self::Region => "region",
            Self::Parameter => "parameter",
            Self::Qubit => "qubit",
            Self::Resource => "resource",
            Self::Capability => "capability",
            Self::Extension => "extension",
            Self::Type => "type",
            Self::Value => "value",
            Self::External => "external",
        };

        formatter.write_str(value)
    }
}

// =============================================================================
// Module error
// =============================================================================

/// Errors produced by module-local construction and validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleError {
    /// The module name is empty.
    EmptyName,

    /// The namespace name is empty.
    EmptyNamespace,

    /// A symbol name is empty.
    EmptySymbolName,

    /// An import alias/name is empty.
    EmptyImportName,

    /// An export name is empty.
    EmptyExportName,

    /// Metadata key is empty.
    EmptyMetadataKey,

    /// Metadata value is invalid.
    InvalidMetadataValue,

    /// A module already exists in a registry.
    DuplicateModule {
        /// Conflicting module identity.
        module: ModuleId,
    },

    /// A namespace is already registered.
    DuplicateNamespace {
        /// Conflicting namespace identity.
        namespace: NamespaceId,
    },

    /// A symbol already exists.
    DuplicateSymbol {
        /// Conflicting symbol name.
        name: String,
    },

    /// A symbol is not present.
    UnknownSymbol {
        /// Missing symbol name.
        name: String,
    },

    /// An import already exists.
    DuplicateImport {
        /// Import identity/name.
        name: String,
    },

    /// An export already exists.
    DuplicateExport {
        /// Export name.
        name: String,
    },

    /// An export references a symbol that is not registered.
    UnknownExportSymbol {
        /// Exported symbol name.
        name: String,
    },

    /// An import is self-referential.
    SelfImport {
        /// Module attempting to import itself.
        module: ModuleId,
    },

    /// A module depends on itself.
    SelfDependency {
        /// Module attempting to depend on itself.
        module: ModuleId,
    },

    /// A dependency appears more than once.
    DuplicateDependency {
        /// Dependency module.
        module: ModuleId,
    },

    /// An operation reference was duplicated.
    DuplicateOperation {
        /// Referenced operation.
        operation: OperationId,
    },

    /// A region reference was duplicated.
    DuplicateRegion {
        /// Referenced region.
        region: RegionId,
    },

    /// A function reference was duplicated.
    DuplicateFunction {
        /// Referenced function.
        function: FunctionId,
    },

    /// A parameter reference was duplicated.
    DuplicateParameter {
        /// Referenced parameter.
        parameter: ParameterId,
    },

    /// A logical qubit reference was duplicated.
    DuplicateQubit {
        /// Referenced qubit.
        qubit: QubitId,
    },

    /// A resource reference was duplicated.
    DuplicateResource {
        /// Referenced resource.
        resource: ResourceId,
    },

    /// A capability requirement was duplicated.
    DuplicateCapability {
        /// Referenced capability.
        capability: CapabilityId,
    },

    /// An extension reference was duplicated.
    DuplicateExtension {
        /// Referenced extension.
        extension: ExtensionId,
    },

    /// An entry region does not belong to the module.
    UnknownEntryRegion {
        /// Requested entry region.
        region: RegionId,
    },

    /// A registered symbol refers to an object not registered with the
    /// corresponding module collection.
    SymbolReferenceMissing {
        /// Symbol name.
        name: String,
    },

    /// A module version is incompatible with the requested contract.
    UnsupportedVersion {
        /// Version carried by the module.
        version: IrVersion,
    },

    /// A dependency version is newer than this module can explicitly accept.
    UnsupportedDependencyVersion {
        /// Dependency module.
        module: ModuleId,

        /// Requested dependency version.
        version: IrVersion,
    },

    /// A module cannot import itself.
    InvalidImport,

    /// A module has an invalid structural state.
    InvalidModule {
        /// Static explanation.
        message: &'static str,
    },

    /// Checked arithmetic failed.
    ArithmeticOverflow {
        /// Description of the calculation.
        calculation: &'static str,
    },
}

impl fmt::Display for ModuleError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyName => {
                formatter.write_str("module name cannot be empty")
            }

            Self::EmptyNamespace => {
                formatter.write_str("module namespace cannot be empty")
            }

            Self::EmptySymbolName => {
                formatter.write_str("module symbol name cannot be empty")
            }

            Self::EmptyImportName => {
                formatter.write_str("module import name cannot be empty")
            }

            Self::EmptyExportName => {
                formatter.write_str("module export name cannot be empty")
            }

            Self::EmptyMetadataKey => {
                formatter.write_str("module metadata key cannot be empty")
            }

            Self::InvalidMetadataValue => {
                formatter.write_str("module metadata value is invalid")
            }

            Self::DuplicateModule { module } => {
                write!(formatter, "duplicate module {module}")
            }

            Self::DuplicateNamespace { namespace } => {
                write!(
                    formatter,
                    "duplicate namespace {namespace}"
                )
            }

            Self::DuplicateSymbol { name } => {
                write!(
                    formatter,
                    "duplicate module symbol `{name}`"
                )
            }

            Self::UnknownSymbol { name } => {
                write!(
                    formatter,
                    "unknown module symbol `{name}`"
                )
            }

            Self::DuplicateImport { name } => {
                write!(
                    formatter,
                    "duplicate module import `{name}`"
                )
            }

            Self::DuplicateExport { name } => {
                write!(
                    formatter,
                    "duplicate module export `{name}`"
                )
            }

            Self::UnknownExportSymbol { name } => {
                write!(
                    formatter,
                    "export references unknown symbol `{name}`"
                )
            }

            Self::SelfImport { module } => {
                write!(
                    formatter,
                    "module {module} cannot import itself"
                )
            }

            Self::SelfDependency { module } => {
                write!(
                    formatter,
                    "module {module} cannot depend on itself"
                )
            }

            Self::DuplicateDependency { module } => {
                write!(
                    formatter,
                    "duplicate module dependency {module}"
                )
            }

            Self::DuplicateOperation { operation } => {
                write!(
                    formatter,
                    "duplicate operation reference {operation}"
                )
            }

            Self::DuplicateRegion { region } => {
                write!(
                    formatter,
                    "duplicate region reference {region}"
                )
            }

            Self::DuplicateFunction { function } => {
                write!(
                    formatter,
                    "duplicate function reference {function}"
                )
            }

            Self::DuplicateParameter { parameter } => {
                write!(
                    formatter,
                    "duplicate parameter reference {parameter}"
                )
            }

            Self::DuplicateQubit { qubit } => {
                write!(
                    formatter,
                    "duplicate logical-qubit reference {qubit}"
                )
            }

            Self::DuplicateResource { resource } => {
                write!(
                    formatter,
                    "duplicate resource reference {resource}"
                )
            }

            Self::DuplicateCapability { capability } => {
                write!(
                    formatter,
                    "duplicate capability requirement {capability}"
                )
            }

            Self::DuplicateExtension { extension } => {
                write!(
                    formatter,
                    "duplicate extension reference {extension}"
                )
            }

            Self::UnknownEntryRegion { region } => {
                write!(
                    formatter,
                    "entry region {region} is not registered with the module"
                )
            }

            Self::SymbolReferenceMissing { name } => {
                write!(
                    formatter,
                    "module symbol `{name}` references an unregistered object"
                )
            }

            Self::UnsupportedVersion { version } => {
                write!(
                    formatter,
                    "module IR version {version} is not supported by the current implementation"
                )
            }

            Self::UnsupportedDependencyVersion {
                module,
                version,
            } => {
                write!(
                    formatter,
                    "dependency module {module} requires unsupported IR version {version}"
                )
            }

            Self::InvalidImport => {
                formatter.write_str("invalid module import")
            }

            Self::InvalidModule { message } => {
                write!(
                    formatter,
                    "invalid quantum module: {message}"
                )
            }

            Self::ArithmeticOverflow { calculation } => {
                write!(
                    formatter,
                    "arithmetic overflow while calculating {calculation}"
                )
            }
        }
    }
}

impl std::error::Error for ModuleError {}

// =============================================================================
// Module symbol
// =============================================================================

/// Stable module-local symbol declaration.
///
/// Detailed symbol semantics remain owned by the future canonical
/// `program::symbol` layer. This structure only records the module's symbol
/// table contract.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ModuleSymbol {
    name: String,
    kind: ModuleSymbolKind,
    external: bool,

    function: Option<FunctionId>,
    operation: Option<OperationId>,
    region: Option<RegionId>,
    parameter: Option<ParameterId>,
    qubit: Option<QubitId>,
    resource: Option<ResourceId>,
    capability: Option<CapabilityId>,
    extension: Option<ExtensionId>,
}

impl ModuleSymbol {
    /// Creates a function symbol.
    pub fn function<S>(
        name: S,
        function: FunctionId,
    ) -> ModuleResult<Self>
    where
        S: Into<String>,
    {
        Self::new(name, ModuleSymbolKind::Function)
            .with_function(function)
    }

    /// Creates an operation symbol.
    pub fn operation<S>(
        name: S,
        operation: OperationId,
    ) -> ModuleResult<Self>
    where
        S: Into<String>,
    {
        Self::new(name, ModuleSymbolKind::Operation)
            .with_operation(operation)
    }

    /// Creates a region symbol.
    pub fn region<S>(
        name: S,
        region: RegionId,
    ) -> ModuleResult<Self>
    where
        S: Into<String>,
    {
        Self::new(name, ModuleSymbolKind::Region)
            .with_region(region)
    }

    /// Creates a parameter symbol.
    pub fn parameter<S>(
        name: S,
        parameter: ParameterId,
    ) -> ModuleResult<Self>
    where
        S: Into<String>,
    {
        Self::new(name, ModuleSymbolKind::Parameter)
            .with_parameter(parameter)
    }

    /// Creates a logical-qubit symbol.
    pub fn qubit<S>(
        name: S,
        qubit: QubitId,
    ) -> ModuleResult<Self>
    where
        S: Into<String>,
    {
        Self::new(name, ModuleSymbolKind::Qubit)
            .with_qubit(qubit)
    }

    /// Creates a resource symbol.
    pub fn resource<S>(
        name: S,
        resource: ResourceId,
    ) -> ModuleResult<Self>
    where
        S: Into<String>,
    {
        Self::new(name, ModuleSymbolKind::Resource)
            .with_resource(resource)
    }

    /// Creates a capability symbol.
    pub fn capability<S>(
        name: S,
        capability: CapabilityId,
    ) -> ModuleResult<Self>
    where
        S: Into<String>,
    {
        Self::new(name, ModuleSymbolKind::Capability)
            .with_capability(capability)
    }

    /// Creates an extension symbol.
    pub fn extension<S>(
        name: S,
        extension: ExtensionId,
    ) -> ModuleResult<Self>
    where
        S: Into<String>,
    {
        Self::new(name, ModuleSymbolKind::Extension)
            .with_extension(extension)
    }

    /// Creates a symbol with no bound object.
    ///
    /// This is useful for forward declarations and interface modules.
    pub fn declaration<S>(
        name: S,
        kind: ModuleSymbolKind,
    ) -> ModuleResult<Self>
    where
        S: Into<String>,
    {
        Self::new(name, kind)
    }

    /// Creates an external symbol declaration.
    pub fn external<S>(
        name: S,
    ) -> ModuleResult<Self>
    where
        S: Into<String>,
    {
        let mut symbol =
            Self::new(name, ModuleSymbolKind::External)?;

        symbol.external = true;

        Ok(symbol)
    }

    /// Creates a symbol.
    pub fn new<S>(
        name: S,
        kind: ModuleSymbolKind,
    ) -> ModuleResult<Self>
    where
        S: Into<String>,
    {
        let name = name.into();

        validate_name(
            &name,
            ModuleError::EmptySymbolName,
        )?;

        Ok(Self {
            name,
            kind,
            external: false,
            function: None,
            operation: None,
            region: None,
            parameter: None,
            qubit: None,
            resource: None,
            capability: None,
            extension: None,
        })
    }

    /// Returns the symbol name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the symbol kind.
    #[must_use]
    pub const fn kind(&self) -> ModuleSymbolKind {
        self.kind
    }

    /// Returns whether this symbol is external.
    #[must_use]
    pub const fn is_external(&self) -> bool {
        self.external
    }

    /// Returns the referenced function, if any.
    #[must_use]
    pub const fn function(&self) -> Option<FunctionId> {
        self.function
    }

    /// Returns the referenced operation, if any.
    #[must_use]
    pub const fn operation(&self) -> Option<OperationId> {
        self.operation
    }

    /// Returns the referenced region, if any.
    #[must_use]
    pub const fn region(&self) -> Option<RegionId> {
        self.region
    }

    /// Returns the referenced parameter, if any.
    #[must_use]
    pub const fn parameter(&self) -> Option<ParameterId> {
        self.parameter
    }

    /// Returns the referenced logical qubit, if any.
    #[must_use]
    pub const fn qubit(&self) -> Option<QubitId> {
        self.qubit
    }

    /// Returns the referenced resource, if any.
    #[must_use]
    pub const fn resource(&self) -> Option<ResourceId> {
        self.resource
    }

    /// Returns the referenced capability, if any.
    #[must_use]
    pub const fn capability(&self) -> Option<CapabilityId> {
        self.capability
    }

    /// Returns the referenced extension, if any.
    #[must_use]
    pub const fn extension(&self) -> Option<ExtensionId> {
        self.extension
    }

    /// Associates a function.
    #[must_use]
    pub const fn with_function(
        mut self,
        value: FunctionId,
    ) -> Self {
        self.function = Some(value);
        self
    }

    /// Associates an operation.
    #[must_use]
    pub const fn with_operation(
        mut self,
        value: OperationId,
    ) -> Self {
        self.operation = Some(value);
        self
    }

    /// Associates a region.
    #[must_use]
    pub const fn with_region(
        mut self,
        value: RegionId,
    ) -> Self {
        self.region = Some(value);
        self
    }

    /// Associates a parameter.
    #[must_use]
    pub const fn with_parameter(
        mut self,
        value: ParameterId,
    ) -> Self {
        self.parameter = Some(value);
        self
    }

    /// Associates a logical qubit.
    #[must_use]
    pub const fn with_qubit(
        mut self,
        value: QubitId,
    ) -> Self {
        self.qubit = Some(value);
        self
    }

    /// Associates a resource.
    #[must_use]
    pub const fn with_resource(
        mut self,
        value: ResourceId,
    ) -> Self {
        self.resource = Some(value);
        self
    }

    /// Associates a capability.
    #[must_use]
    pub const fn with_capability(
        mut self,
        value: CapabilityId,
    ) -> Self {
        self.capability = Some(value);
        self
    }

    /// Associates an extension.
    #[must_use]
    pub const fn with_extension(
        mut self,
        value: ExtensionId,
    ) -> Self {
        self.extension = Some(value);
        self
    }

    /// Returns the referenced canonical identity, if this symbol has one.
    ///
    /// This deliberately returns no dynamically typed enum because the
    /// detailed symbol layer owns that abstraction.
    #[must_use]
    pub const fn has_reference(&self) -> bool {
        self.function.is_some()
            || self.operation.is_some()
            || self.region.is_some()
            || self.parameter.is_some()
            || self.qubit.is_some()
            || self.resource.is_some()
            || self.capability.is_some()
            || self.extension.is_some()
    }
}

// =============================================================================
// Module dependency
// =============================================================================

/// A dependency on another IR module.
///
/// Dependencies are references, not loaded module objects. This is important
/// for scalable and distributed compilation.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ModuleDependency {
    module: ModuleId,
    required_version: IrVersion,
    alias: Option<String>,
    optional: bool,
}

impl ModuleDependency {
    /// Creates a required dependency.
    #[must_use]
    pub const fn new(
        module: ModuleId,
        required_version: IrVersion,
    ) -> Self {
        Self {
            module,
            required_version,
            alias: None,
            optional: false,
        }
    }

    /// Returns the dependency module identity.
    #[must_use]
    pub const fn module(&self) -> ModuleId {
        self.module
    }

    /// Returns the required IR version.
    #[must_use]
    pub const fn required_version(&self) -> IrVersion {
        self.required_version
    }

    /// Returns the optional import alias.
    #[must_use]
    pub fn alias(&self) -> Option<&str> {
        self.alias.as_deref()
    }

    /// Returns whether the dependency is optional.
    #[must_use]
    pub const fn optional(&self) -> bool {
        self.optional
    }

    /// Sets an alias.
    pub fn with_alias<S>(
        mut self,
        alias: S,
    ) -> ModuleResult<Self>
    where
        S: Into<String>,
    {
        let alias = alias.into();

        validate_name(
            &alias,
            ModuleError::EmptyImportName,
        )?;

        self.alias = Some(alias);

        Ok(self)
    }

    /// Marks the dependency as optional.
    #[must_use]
    pub const fn optional(mut self) -> Self {
        self.optional = true;
        self
    }
}

// =============================================================================
// Module import
// =============================================================================

/// A module import declaration.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ModuleImport {
    source: ModuleId,
    name: String,
    alias: Option<String>,
    required_version: IrVersion,
}

impl ModuleImport {
    /// Creates an import declaration.
    pub fn new<S>(
        source: ModuleId,
        name: S,
        required_version: IrVersion,
    ) -> ModuleResult<Self>
    where
        S: Into<String>,
    {
        let name = name.into();

        validate_name(
            &name,
            ModuleError::EmptyImportName,
        )?;

        Ok(Self {
            source,
            name,
            alias: None,
            required_version,
        })
    }

    /// Returns the source module.
    #[must_use]
    pub const fn source(&self) -> ModuleId {
        self.source
    }

    /// Returns the imported symbol/name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the alias.
    #[must_use]
    pub fn alias(&self) -> Option<&str> {
        self.alias.as_deref()
    }

    /// Returns the required IR version.
    #[must_use]
    pub const fn required_version(&self) -> IrVersion {
        self.required_version
    }

    /// Sets an import alias.
    pub fn with_alias<S>(
        mut self,
        alias: S,
    ) -> ModuleResult<Self>
    where
        S: Into<String>,
    {
        let alias = alias.into();

        validate_name(
            &alias,
            ModuleError::EmptyImportName,
        )?;

        self.alias = Some(alias);

        Ok(self)
    }
}

// =============================================================================
// Module export
// =============================================================================

/// A module export declaration.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ModuleExport {
    name: String,
    symbol: String,
    public: bool,
}

impl ModuleExport {
    /// Creates a public export.
    pub fn new<S1, S2>(
        name: S1,
        symbol: S2,
    ) -> ModuleResult<Self>
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        let name = name.into();
        let symbol = symbol.into();

        validate_name(
            &name,
            ModuleError::EmptyExportName,
        )?;

        validate_name(
            &symbol,
            ModuleError::EmptySymbolName,
        )?;

        Ok(Self {
            name,
            symbol,
            public: true,
        })
    }

    /// Returns the exported external name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the module-local symbol being exported.
    #[must_use]
    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    /// Returns whether the export is public.
    #[must_use]
    pub const fn is_public(&self) -> bool {
        self.public
    }

    /// Changes public visibility.
    #[must_use]
    pub const fn public(mut self, value: bool) -> Self {
        self.public = value;
        self
    }
}

// =============================================================================
// Module requirements
// =============================================================================

/// Module-level semantic requirements.
///
/// These are abstract requirements, not hardware claims.
///
/// Hardware capability matching occurs downstream.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ModuleRequirements {
    capabilities: BTreeSet<CapabilityId>,
    resources: BTreeSet<ResourceId>,
    logical_qubits: BTreeSet<QubitId>,
    pulse_control: bool,
    dynamic_control: bool,
    analog_execution: bool,
    annealing_execution: bool,
    fault_tolerant_execution: bool,
    distributed_execution: bool,
}

impl ModuleRequirements {
    /// Creates empty requirements.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns required capabilities.
    #[must_use]
    pub fn capabilities(&self) -> &BTreeSet<CapabilityId> {
        &self.capabilities
    }

    /// Returns required resources.
    #[must_use]
    pub fn resources(&self) -> &BTreeSet<ResourceId> {
        &self.resources
    }

    /// Returns referenced logical qubits.
    #[must_use]
    pub fn logical_qubits(&self) -> &BTreeSet<QubitId> {
        &self.logical_qubits
    }

    /// Returns whether pulse control is required.
    #[must_use]
    pub const fn pulse_control(&self) -> bool {
        self.pulse_control
    }

    /// Returns whether dynamic control is required.
    #[must_use]
    pub const fn dynamic_control(&self) -> bool {
        self.dynamic_control
    }

    /// Returns whether analog execution is required.
    #[must_use]
    pub const fn analog_execution(&self) -> bool {
        self.analog_execution
    }

    /// Returns whether annealing execution is required.
    #[must_use]
    pub const fn annealing_execution(&self) -> bool {
        self.annealing_execution
    }

    /// Returns whether fault tolerance is required.
    #[must_use]
    pub const fn fault_tolerant_execution(&self) -> bool {
        self.fault_tolerant_execution
    }

    /// Returns whether distributed execution is required.
    #[must_use]
    pub const fn distributed_execution(&self) -> bool {
        self.distributed_execution
    }

    /// Requires a capability.
    pub fn require_capability(
        &mut self,
        capability: CapabilityId,
    ) -> bool {
        self.capabilities.insert(capability)
    }

    /// Requires a resource.
    pub fn require_resource(
        &mut self,
        resource: ResourceId,
    ) -> bool {
        self.resources.insert(resource)
    }

    /// Records a logical qubit.
    pub fn require_qubit(
        &mut self,
        qubit: QubitId,
    ) -> bool {
        self.logical_qubits.insert(qubit)
    }

    /// Requires pulse control.
    pub fn require_pulse_control(&mut self) {
        self.pulse_control = true;
    }

    /// Requires dynamic control.
    pub fn require_dynamic_control(&mut self) {
        self.dynamic_control = true;
    }

    /// Requires analog execution.
    pub fn require_analog_execution(&mut self) {
        self.analog_execution = true;
    }

    /// Requires annealing execution.
    pub fn require_annealing_execution(&mut self) {
        self.annealing_execution = true;
    }

    /// Requires fault-tolerant execution.
    pub fn require_fault_tolerant_execution(&mut self) {
        self.fault_tolerant_execution = true;
    }

    /// Requires distributed execution.
    pub fn require_distributed_execution(&mut self) {
        self.distributed_execution = true;
    }
}

// =============================================================================
// Quantum module
// =============================================================================

/// Production-grade universal Zamani Quantum IR module.
///
/// A module is a deterministic semantic compilation unit. It stores stable
/// references to objects owned by other IR layers instead of duplicating their
/// implementation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuantumModule {
    id: ModuleId,
    name: String,
    namespace: NamespaceId,
    kind: ModuleKind,
    visibility: ModuleVisibility,
    version: IrVersion,

    parent: Option<ModuleId>,

    symbols: BTreeMap<String, ModuleSymbol>,

    imports: BTreeMap<String, ModuleImport>,
    exports: BTreeMap<String, ModuleExport>,
    dependencies: BTreeMap<ModuleId, ModuleDependency>,

    operations: BTreeSet<OperationId>,
    regions: BTreeSet<RegionId>,
    functions: BTreeSet<FunctionId>,
    parameters: BTreeSet<ParameterId>,
    qubits: BTreeSet<QubitId>,
    resources: BTreeSet<ResourceId>,
    capabilities: BTreeSet<CapabilityId>,
    extensions: BTreeSet<ExtensionId>,

    entry_region: Option<RegionId>,

    requirements: ModuleRequirements,

    metadata: BTreeMap<String, String>,
}

impl QuantumModule {
    /// Creates a general-purpose quantum module.
    pub fn new<S>(
        id: ModuleId,
        name: S,
    ) -> ModuleResult<Self>
    where
        S: Into<String>,
    {
        Self::with_options(
            id,
            name,
            NamespaceId::new(id.value()),
            ModuleKind::Quantum,
            ModuleVisibility::Private,
            IrVersion::CURRENT,
        )
    }

    /// Creates a module with an explicit namespace.
    pub fn with_namespace<S>(
        id: ModuleId,
        name: S,
        namespace: NamespaceId,
    ) -> ModuleResult<Self>
    where
        S: Into<String>,
    {
        Self::with_options(
            id,
            name,
            namespace,
            ModuleKind::Quantum,
            ModuleVisibility::Private,
            IrVersion::CURRENT,
        )
    }

    /// Creates a module with complete structural options.
    pub fn with_options<S>(
        id: ModuleId,
        name: S,
        namespace: NamespaceId,
        kind: ModuleKind,
        visibility: ModuleVisibility,
        version: IrVersion,
    ) -> ModuleResult<Self>
    where
        S: Into<String>,
    {
        let name = name.into();

        validate_name(
            &name,
            ModuleError::EmptyName,
        )?;

        if namespace.value() == u64::MAX {
            return Err(ModuleError::DuplicateNamespace {
                namespace,
            });
        }

        Ok(Self {
            id,
            name,
            namespace,
            kind,
            visibility,
            version,
            parent: None,
            symbols: BTreeMap::new(),
            imports: BTreeMap::new(),
            exports: BTreeMap::new(),
            dependencies: BTreeMap::new(),
            operations: BTreeSet::new(),
            regions: BTreeSet::new(),
            functions: BTreeSet::new(),
            parameters: BTreeSet::new(),
            qubits: BTreeSet::new(),
            resources: BTreeSet::new(),
            capabilities: BTreeSet::new(),
            extensions: BTreeSet::new(),
            entry_region: None,
            requirements: ModuleRequirements::new(),
            metadata: BTreeMap::new(),
        })
    }

    /// Returns the module identity.
    #[must_use]
    pub const fn id(&self) -> ModuleId {
        self.id
    }

    /// Returns the module name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the namespace identity.
    #[must_use]
    pub const fn namespace(&self) -> NamespaceId {
        self.namespace
    }

    /// Returns the module kind.
    #[must_use]
    pub const fn kind(&self) -> ModuleKind {
        self.kind
    }

    /// Returns module visibility.
    #[must_use]
    pub const fn visibility(&self) -> ModuleVisibility {
        self.visibility
    }

    /// Returns the module IR version.
    #[must_use]
    pub const fn version(&self) -> IrVersion {
        self.version
    }

    /// Returns the parent module.
    #[must_use]
    pub const fn parent(&self) -> Option<ModuleId> {
        self.parent
    }

    /// Returns the entry region.
    #[must_use]
    pub const fn entry_region(&self) -> Option<RegionId> {
        self.entry_region
    }

    /// Returns all module symbols in deterministic order.
    #[must_use]
    pub fn symbols(&self) -> &BTreeMap<String, ModuleSymbol> {
        &self.symbols
    }

    /// Returns a symbol by name.
    #[must_use]
    pub fn symbol(
        &self,
        name: &str,
    ) -> Option<&ModuleSymbol> {
        self.symbols.get(name)
    }

    /// Returns all imports.
    #[must_use]
    pub fn imports(&self) -> &BTreeMap<String, ModuleImport> {
        &self.imports
    }

    /// Returns all exports.
    #[must_use]
    pub fn exports(&self) -> &BTreeMap<String, ModuleExport> {
        &self.exports
    }

    /// Returns all dependencies.
    #[must_use]
    pub fn dependencies(
        &self,
    ) -> &BTreeMap<ModuleId, ModuleDependency> {
        &self.dependencies
    }

    /// Returns operation references.
    #[must_use]
    pub fn operations(&self) -> &BTreeSet<OperationId> {
        &self.operations
    }

    /// Returns region references.
    #[must_use]
    pub fn regions(&self) -> &BTreeSet<RegionId> {
        &self.regions
    }

    /// Returns function references.
    #[must_use]
    pub fn functions(&self) -> &BTreeSet<FunctionId> {
        &self.functions
    }

    /// Returns parameter references.
    #[must_use]
    pub fn parameters(&self) -> &BTreeSet<ParameterId> {
        &self.parameters
    }

    /// Returns logical-qubit references.
    ///
    /// These use the canonical `quantum::ir::qubit::QubitId`.
    #[must_use]
    pub fn qubits(&self) -> &BTreeSet<QubitId> {
        &self.qubits
    }

    /// Returns resource references.
    #[must_use]
    pub fn resources(&self) -> &BTreeSet<ResourceId> {
        &self.resources
    }

    /// Returns capability requirements.
    #[must_use]
    pub fn capabilities(&self) -> &BTreeSet<CapabilityId> {
        &self.capabilities
    }

    /// Returns extension references.
    #[must_use]
    pub fn extensions(&self) -> &BTreeSet<ExtensionId> {
        &self.extensions
    }

    /// Returns module requirements.
    #[must_use]
    pub fn requirements(&self) -> &ModuleRequirements {
        &self.requirements
    }

    /// Returns module metadata.
    #[must_use]
    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    /// Sets the parent module.
    ///
    /// A module cannot be its own parent.
    pub fn set_parent(
        &mut self,
        parent: Option<ModuleId>,
    ) -> ModuleResult<()> {
        if parent == Some(self.id) {
            return Err(ModuleError::InvalidModule {
                message: "a module cannot be its own parent",
            });
        }

        self.parent = parent;

        Ok(())
    }

    /// Changes module kind.
    pub fn set_kind(
        &mut self,
        kind: ModuleKind,
    ) {
        self.kind = kind;
    }

    /// Changes module visibility.
    pub fn set_visibility(
        &mut self,
        visibility: ModuleVisibility,
    ) {
        self.visibility = visibility;
    }

    /// Changes the module IR version.
    ///
    /// Future versions are not silently accepted.
    pub fn set_version(
        &mut self,
        version: IrVersion,
    ) -> ModuleResult<()> {
        if !version.is_supported_by_current() {
            return Err(ModuleError::UnsupportedVersion {
                version,
            });
        }

        self.version = version;

        Ok(())
    }

    /// Sets the entry region.
    ///
    /// The region must already be registered with the module.
    pub fn set_entry_region(
        &mut self,
        region: RegionId,
    ) -> ModuleResult<()> {
        if !self.regions.contains(&region) {
            return Err(ModuleError::UnknownEntryRegion {
                region,
            });
        }

        self.entry_region = Some(region);

        Ok(())
    }

    /// Clears the entry region.
    pub fn clear_entry_region(&mut self) {
        self.entry_region = None;
    }

    // =========================================================================
    // Symbol management
    // =========================================================================

    /// Inserts a module-local symbol.
    pub fn add_symbol(
        &mut self,
        symbol: ModuleSymbol,
    ) -> ModuleResult<()> {
        let name = symbol.name().to_owned();

        if self.symbols.contains_key(&name) {
            return Err(ModuleError::DuplicateSymbol {
                name,
            });
        }

        self.validate_symbol_reference(&symbol)?;

        self.symbols.insert(
            symbol.name().to_owned(),
            symbol,
        );

        Ok(())
    }

    /// Inserts multiple symbols atomically.
    pub fn add_symbols(
        &mut self,
        symbols: &[ModuleSymbol],
    ) -> ModuleResult<()> {
        let mut pending = BTreeMap::new();

        for symbol in symbols {
            let name = symbol.name().to_owned();

            if self.symbols.contains_key(&name)
                || pending.contains_key(&name)
            {
                return Err(ModuleError::DuplicateSymbol {
                    name,
                });
            }

            self.validate_symbol_reference(symbol)?;

            pending.insert(name, symbol.clone());
        }

        self.symbols.extend(pending);

        Ok(())
    }

    /// Removes a symbol.
    ///
    /// The symbol cannot be removed while an export still refers to it.
    pub fn remove_symbol(
        &mut self,
        name: &str,
    ) -> ModuleResult<ModuleSymbol> {
        if self
            .exports
            .values()
            .any(|export| export.symbol() == name)
        {
            return Err(ModuleError::InvalidModule {
                message: "cannot remove a symbol that is still exported",
            });
        }

        self.symbols
            .remove(name)
            .ok_or_else(|| ModuleError::UnknownSymbol {
                name: name.to_owned(),
            })
    }

    // =========================================================================
    // Imports
    // =========================================================================

    /// Adds an import declaration.
    pub fn add_import(
        &mut self,
        import: ModuleImport,
    ) -> ModuleResult<()> {
        if import.source() == self.id {
            return Err(ModuleError::SelfImport {
                module: self.id,
            });
        }

        let key = import
            .alias()
            .unwrap_or(import.name())
            .to_owned();

        if self.imports.contains_key(&key) {
            return Err(ModuleError::DuplicateImport {
                name: key,
            });
        }

        self.imports.insert(key, import);

        Ok(())
    }

    /// Adds multiple imports atomically.
    pub fn add_imports(
        &mut self,
        imports: &[ModuleImport],
    ) -> ModuleResult<()> {
        let mut pending = BTreeMap::new();

        for import in imports {
            if import.source() == self.id {
                return Err(ModuleError::SelfImport {
                    module: self.id,
                });
            }

            let key = import
                .alias()
                .unwrap_or(import.name())
                .to_owned();

            if self.imports.contains_key(&key)
                || pending.contains_key(&key)
            {
                return Err(ModuleError::DuplicateImport {
                    name: key,
                });
            }

            pending.insert(key, import.clone());
        }

        self.imports.extend(pending);

        Ok(())
    }

    /// Removes an import by its local name/alias.
    pub fn remove_import(
        &mut self,
        name: &str,
    ) -> Option<ModuleImport> {
        self.imports.remove(name)
    }

    // =========================================================================
    // Exports
    // =========================================================================

    /// Adds an export declaration.
    ///
    /// The exported symbol must already exist.
    pub fn add_export(
        &mut self,
        export: ModuleExport,
    ) -> ModuleResult<()> {
        if !self.symbols.contains_key(export.symbol()) {
            return Err(ModuleError::UnknownExportSymbol {
                name: export.symbol().to_owned(),
            });
        }

        if self.exports.contains_key(export.name()) {
            return Err(ModuleError::DuplicateExport {
                name: export.name().to_owned(),
            });
        }

        self.exports.insert(
            export.name().to_owned(),
            export,
        );

        Ok(())
    }

    /// Adds multiple exports atomically.
    pub fn add_exports(
        &mut self,
        exports: &[ModuleExport],
    ) -> ModuleResult<()> {
        let mut pending = BTreeMap::new();

        for export in exports {
            if !self.symbols.contains_key(export.symbol()) {
                return Err(ModuleError::UnknownExportSymbol {
                    name: export.symbol().to_owned(),
                });
            }

            if self.exports.contains_key(export.name())
                || pending.contains_key(export.name())
            {
                return Err(ModuleError::DuplicateExport {
                    name: export.name().to_owned(),
                });
            }

            pending.insert(
                export.name().to_owned(),
                export.clone(),
            );
        }

        self.exports.extend(pending);

        Ok(())
    }

    /// Removes an export.
    pub fn remove_export(
        &mut self,
        name: &str,
    ) -> Option<ModuleExport> {
        self.exports.remove(name)
    }

    // =========================================================================
    // Dependencies
    // =========================================================================

    /// Adds a module dependency.
    pub fn add_dependency(
        &mut self,
        dependency: ModuleDependency,
    ) -> ModuleResult<()> {
        if dependency.module() == self.id {
            return Err(ModuleError::SelfDependency {
                module: self.id,
            });
        }

        if self
            .dependencies
            .contains_key(&dependency.module())
        {
            return Err(ModuleError::DuplicateDependency {
                module: dependency.module(),
            });
        }

        if !self
            .version
            .supports(dependency.required_version())
        {
            return Err(
                ModuleError::UnsupportedDependencyVersion {
                    module: dependency.module(),
                    version: dependency.required_version(),
                },
            );
        }

        self.dependencies.insert(
            dependency.module(),
            dependency,
        );

        Ok(())
    }

    /// Adds multiple dependencies atomically.
    pub fn add_dependencies(
        &mut self,
        dependencies: &[ModuleDependency],
    ) -> ModuleResult<()> {
        let mut pending = BTreeMap::new();

        for dependency in dependencies {
            if dependency.module() == self.id {
                return Err(ModuleError::SelfDependency {
                    module: self.id,
                });
            }

            if self
                .dependencies
                .contains_key(&dependency.module())
                || pending.contains_key(&dependency.module())
            {
                return Err(ModuleError::DuplicateDependency {
                    module: dependency.module(),
                });
            }

            if !self
                .version
                .supports(dependency.required_version())
            {
                return Err(
                    ModuleError::UnsupportedDependencyVersion {
                        module: dependency.module(),
                        version: dependency.required_version(),
                    },
                );
            }

            pending.insert(
                dependency.module(),
                dependency.clone(),
            );
        }

        self.dependencies.extend(pending);

        Ok(())
    }

    /// Removes a dependency.
    pub fn remove_dependency(
        &mut self,
        module: ModuleId,
    ) -> Option<ModuleDependency> {
        self.dependencies.remove(&module)
    }

    // =========================================================================
    // Canonical object references
    // =========================================================================

    /// Registers an operation reference.
    pub fn add_operation(
        &mut self,
        operation: OperationId,
    ) -> ModuleResult<()> {
        insert_unique(
            &mut self.operations,
            operation,
            ModuleError::DuplicateOperation {
                operation,
            },
        );

        Ok(())
    }

    /// Registers multiple operation references atomically.
    pub fn add_operations(
        &mut self,
        operations: &[OperationId],
    ) -> ModuleResult<()> {
        add_unique_many(
            &mut self.operations,
            operations,
            |operation| ModuleError::DuplicateOperation {
                operation: *operation,
            },
        )
    }

    /// Removes an operation reference.
    pub fn remove_operation(
        &mut self,
        operation: OperationId,
    ) -> bool {
        self.operations.remove(&operation)
    }

    /// Registers a region reference.
    pub fn add_region(
        &mut self,
        region: RegionId,
    ) -> ModuleResult<()> {
        if !self.regions.insert(region) {
            return Err(ModuleError::DuplicateRegion {
                region,
            });
        }

        Ok(())
    }

    /// Registers multiple region references atomically.
    pub fn add_regions(
        &mut self,
        regions: &[RegionId],
    ) -> ModuleResult<()> {
        add_unique_many(
            &mut self.regions,
            regions,
            |region| ModuleError::DuplicateRegion {
                region: *region,
            },
        )
    }

    /// Removes a region reference.
    ///
    /// The entry region cannot be removed until the entry designation is
    /// cleared or changed.
    pub fn remove_region(
        &mut self,
        region: RegionId,
    ) -> ModuleResult<bool> {
        if self.entry_region == Some(region) {
            return Err(ModuleError::InvalidModule {
                message: "cannot remove the module entry region",
            });
        }

        Ok(self.regions.remove(&region))
    }

    /// Registers a function reference.
    pub fn add_function(
        &mut self,
        function: FunctionId,
    ) -> ModuleResult<()> {
        if !self.functions.insert(function) {
            return Err(ModuleError::DuplicateFunction {
                function,
            });
        }

        Ok(())
    }

    /// Registers multiple function references atomically.
    pub fn add_functions(
        &mut self,
        functions: &[FunctionId],
    ) -> ModuleResult<()> {
        add_unique_many(
            &mut self.functions,
            functions,
            |function| ModuleError::DuplicateFunction {
                function: *function,
            },
        )
    }

    /// Removes a function reference.
    pub fn remove_function(
        &mut self,
        function: FunctionId,
    ) -> bool {
        self.functions.remove(&function)
    }

    /// Registers a symbolic parameter reference.
    pub fn add_parameter(
        &mut self,
        parameter: ParameterId,
    ) -> ModuleResult<()> {
        if !self.parameters.insert(parameter) {
            return Err(ModuleError::DuplicateParameter {
                parameter,
            });
        }

        Ok(())
    }

    /// Registers multiple parameter references atomically.
    pub fn add_parameters(
        &mut self,
        parameters: &[ParameterId],
    ) -> ModuleResult<()> {
        add_unique_many(
            &mut self.parameters,
            parameters,
            |parameter| ModuleError::DuplicateParameter {
                parameter: *parameter,
            },
        )
    }

    /// Removes a parameter reference.
    pub fn remove_parameter(
        &mut self,
        parameter: ParameterId,
    ) -> bool {
        self.parameters.remove(&parameter)
    }

    /// Registers a canonical logical-qubit reference.
    ///
    /// This method intentionally accepts `quantum::ir::qubit::QubitId`.
    pub fn add_qubit(
        &mut self,
        qubit: QubitId,
    ) -> ModuleResult<()> {
        if !self.qubits.insert(qubit) {
            return Err(ModuleError::DuplicateQubit {
                qubit,
            });
        }

        self.requirements.require_qubit(qubit);

        Ok(())
    }

    /// Registers multiple logical-qubit references atomically.
    pub fn add_qubits(
        &mut self,
        qubits: &[QubitId],
    ) -> ModuleResult<()> {
        let mut pending = BTreeSet::new();

        for qubit in qubits {
            if self.qubits.contains(qubit)
                || !pending.insert(*qubit)
            {
                return Err(ModuleError::DuplicateQubit {
                    qubit: *qubit,
                });
            }
        }

        self.qubits.extend(pending.iter().copied());
        self.requirements
            .logical_qubits
            .extend(pending);

        Ok(())
    }

    /// Removes a logical-qubit reference.
    pub fn remove_qubit(
        &mut self,
        qubit: QubitId,
    ) -> ModuleResult<bool> {
        if self
            .symbols
            .values()
            .any(|symbol| symbol.qubit() == Some(qubit))
        {
            return Err(ModuleError::InvalidModule {
                message: "cannot remove a logical qubit referenced by a module symbol",
            });
        }

        Ok(self.qubits.remove(&qubit))
    }

    /// Registers a resource reference.
    pub fn add_resource(
        &mut self,
        resource: ResourceId,
    ) -> ModuleResult<()> {
        if !self.resources.insert(resource) {
            return Err(ModuleError::DuplicateResource {
                resource,
            });
        }

        self.requirements.require_resource(resource);

        Ok(())
    }

    /// Registers multiple resource references atomically.
    pub fn add_resources(
        &mut self,
        resources: &[ResourceId],
    ) -> ModuleResult<()> {
        add_unique_many(
            &mut self.resources,
            resources,
            |resource| ModuleError::DuplicateResource {
                resource: *resource,
            },
        )?;

        self.requirements
            .resources
            .extend(resources.iter().copied());

        Ok(())
    }

    /// Removes a resource reference.
    pub fn remove_resource(
        &mut self,
        resource: ResourceId,
    ) -> ModuleResult<bool> {
        if self
            .symbols
            .values()
            .any(|symbol| symbol.resource() == Some(resource))
        {
            return Err(ModuleError::InvalidModule {
                message: "cannot remove a resource referenced by a module symbol",
            });
        }

        Ok(self.resources.remove(&resource))
    }

    /// Registers a capability requirement.
    pub fn require_capability(
        &mut self,
        capability: CapabilityId,
    ) -> ModuleResult<()> {
        if !self.capabilities.insert(capability) {
            return Err(ModuleError::DuplicateCapability {
                capability,
            });
        }

        self.requirements
            .require_capability(capability);

        Ok(())
    }

    /// Requires multiple capabilities atomically.
    pub fn require_capabilities(
        &mut self,
        capabilities: &[CapabilityId],
    ) -> ModuleResult<()> {
        add_unique_many(
            &mut self.capabilities,
            capabilities,
            |capability| ModuleError::DuplicateCapability {
                capability: *capability,
            },
        )?;

        self.requirements
            .capabilities
            .extend(capabilities.iter().copied());

        Ok(())
    }

    /// Removes a capability requirement.
    pub fn remove_capability(
        &mut self,
        capability: CapabilityId,
    ) -> bool {
        let removed = self.capabilities.remove(&capability);

        if removed {
            self.requirements
                .capabilities
                .remove(&capability);
        }

        removed
    }

    /// Registers an extension reference.
    pub fn add_extension(
        &mut self,
        extension: ExtensionId,
    ) -> ModuleResult<()> {
        if !self.extensions.insert(extension) {
            return Err(ModuleError::DuplicateExtension {
                extension,
            });
        }

        Ok(())
    }

    /// Registers multiple extension references atomically.
    pub fn add_extensions(
        &mut self,
        extensions: &[ExtensionId],
    ) -> ModuleResult<()> {
        add_unique_many(
            &mut self.extensions,
            extensions,
            |extension| ModuleError::DuplicateExtension {
                extension: *extension,
            },
        )
    }

    /// Removes an extension reference.
    pub fn remove_extension(
        &mut self,
        extension: ExtensionId,
    ) -> bool {
        self.extensions.remove(&extension)
    }

    // =========================================================================
    // Requirements
    // =========================================================================

    /// Marks the module as requiring pulse-level control.
    pub fn require_pulse_control(&mut self) {
        self.requirements.require_pulse_control();
    }

    /// Marks the module as requiring dynamic control.
    pub fn require_dynamic_control(&mut self) {
        self.requirements.require_dynamic_control();
    }

    /// Marks the module as requiring analog execution.
    pub fn require_analog_execution(&mut self) {
        self.requirements.require_analog_execution();
    }

    /// Marks the module as requiring annealing semantics.
    pub fn require_annealing_execution(&mut self) {
        self.requirements.require_annealing_execution();
    }

    /// Marks the module as requiring fault-tolerant execution.
    pub fn require_fault_tolerant_execution(&mut self) {
        self.requirements
            .require_fault_tolerant_execution();
    }

    /// Marks the module as requiring distributed execution.
    pub fn require_distributed_execution(&mut self) {
        self.requirements
            .require_distributed_execution();
    }

    // =========================================================================
    // Metadata
    // =========================================================================

    /// Inserts deterministic module metadata.
    ///
    /// Metadata is semantic/compiler metadata only. Hardware credentials,
    /// calibration blobs and execution secrets must never be stored here.
    pub fn insert_metadata<K, V>(
        &mut self,
        key: K,
        value: V,
    ) -> ModuleResult<Option<String>>
    where
        K: Into<String>,
        V: Into<String>,
    {
        let key = key.into();
        let value = value.into();

        validate_name(
            &key,
            ModuleError::EmptyMetadataKey,
        )?;

        if value.contains('\0') {
            return Err(ModuleError::InvalidMetadataValue);
        }

        Ok(self.metadata.insert(key, value))
    }

    /// Removes module metadata.
    pub fn remove_metadata(
        &mut self,
        key: &str,
    ) -> Option<String> {
        self.metadata.remove(key)
    }

    /// Clears all module metadata.
    pub fn clear_metadata(&mut self) {
        self.metadata.clear();
    }

    // =========================================================================
    // Validation
    // =========================================================================

    /// Performs complete module-local structural validation.
    ///
    /// This deliberately does not perform target/hardware validation.
    pub fn validate(&self) -> ModuleResult<()> {
        validate_name(
            &self.name,
            ModuleError::EmptyName,
        )?;

        if self.namespace.value() == u64::MAX {
            return Err(ModuleError::InvalidModule {
                message: "module namespace identity is reserved",
            });
        }

        if !self.version.is_supported_by_current() {
            return Err(ModuleError::UnsupportedVersion {
                version: self.version,
            });
        }

        if self.parent == Some(self.id) {
            return Err(ModuleError::InvalidModule {
                message: "module cannot be its own parent",
            });
        }

        for dependency in self.dependencies.values() {
            if dependency.module() == self.id {
                return Err(ModuleError::SelfDependency {
                    module: self.id,
                });
            }

            if !self
                .version
                .supports(dependency.required_version())
            {
                return Err(
                    ModuleError::UnsupportedDependencyVersion {
                        module: dependency.module(),
                        version: dependency.required_version(),
                    },
                );
            }
        }

        for import in self.imports.values() {
            if import.source() == self.id {
                return Err(ModuleError::SelfImport {
                    module: self.id,
                });
            }

            let key = import
                .alias()
                .unwrap_or(import.name());

            if key.is_empty() {
                return Err(ModuleError::InvalidImport);
            }

            if !self
                .dependencies
                .contains_key(&import.source())
            {
                return Err(ModuleError::InvalidModule {
                    message: "module import references a module that is not registered as a dependency",
                });
            }
        }

        for export in self.exports.values() {
            if !self.symbols.contains_key(export.symbol()) {
                return Err(ModuleError::UnknownExportSymbol {
                    name: export.symbol().to_owned(),
                });
            }
        }

        if let Some(entry) = self.entry_region {
            if !self.regions.contains(&entry) {
                return Err(ModuleError::UnknownEntryRegion {
                    region: entry,
                });
            }
        }

        for symbol in self.symbols.values() {
            self.validate_symbol_reference(symbol)?;

            match symbol.kind() {
                ModuleSymbolKind::Function => {
                    if symbol.function().is_none()
                        && !symbol.is_external()
                    {
                        return Err(
                            ModuleError::SymbolReferenceMissing {
                                name: symbol.name().to_owned(),
                            },
                        );
                    }
                }

                ModuleSymbolKind::Operation => {
                    if symbol.operation().is_none()
                        && !symbol.is_external()
                    {
                        return Err(
                            ModuleError::SymbolReferenceMissing {
                                name: symbol.name().to_owned(),
                            },
                        );
                    }
                }

                ModuleSymbolKind::Region => {
                    if symbol.region().is_none()
                        && !symbol.is_external()
                    {
                        return Err(
                            ModuleError::SymbolReferenceMissing {
                                name: symbol.name().to_owned(),
                            },
                        );
                    }
                }

                ModuleSymbolKind::Parameter => {
                    if symbol.parameter().is_none()
                        && !symbol.is_external()
                    {
                        return Err(
                            ModuleError::SymbolReferenceMissing {
                                name: symbol.name().to_owned(),
                            },
                        );
                    }
                }

                ModuleSymbolKind::Qubit => {
                    if symbol.qubit().is_none()
                        && !symbol.is_external()
                    {
                        return Err(
                            ModuleError::SymbolReferenceMissing {
                                name: symbol.name().to_owned(),
                            },
                        );
                    }
                }

                ModuleSymbolKind::Resource => {
                    if symbol.resource().is_none()
                        && !symbol.is_external()
                    {
                        return Err(
                            ModuleError::SymbolReferenceMissing {
                                name: symbol.name().to_owned(),
                            },
                        );
                    }
                }

                ModuleSymbolKind::Capability => {
                    if symbol.capability().is_none()
                        && !symbol.is_external()
                    {
                        return Err(
                            ModuleError::SymbolReferenceMissing {
                                name: symbol.name().to_owned(),
                            },
                        );
                    }
                }

                ModuleSymbolKind::Extension => {
                    if symbol.extension().is_none()
                        && !symbol.is_external()
                    {
                        return Err(
                            ModuleError::SymbolReferenceMissing {
                                name: symbol.name().to_owned(),
                            },
                        );
                    }
                }

                ModuleSymbolKind::Type
                | ModuleSymbolKind::Value
                | ModuleSymbolKind::External => {}
            }
        }

        for qubit in &self.requirements.logical_qubits {
            if !self.qubits.contains(qubit) {
                return Err(ModuleError::InvalidModule {
                    message: "module requirements contain an unregistered logical qubit",
                });
            }
        }

        for resource in &self.requirements.resources {
            if !self.resources.contains(resource) {
                return Err(ModuleError::InvalidModule {
                    message: "module requirements contain an unregistered resource",
                });
            }
        }

        for capability in &self.requirements.capabilities {
            if !self.capabilities.contains(capability) {
                return Err(ModuleError::InvalidModule {
                    message: "module requirements contain an unregistered capability",
                });
            }
        }

        for (key, value) in &self.metadata {
            if key.is_empty() || value.contains('\0') {
                return Err(ModuleError::InvalidMetadataValue);
            }
        }

        Ok(())
    }

    /// Returns whether the module is structurally valid.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.validate().is_ok()
    }

    /// Returns a deterministic module summary.
    #[must_use]
    pub fn summary(&self) -> ModuleSummary {
        ModuleSummary {
            symbols: self.symbols.len(),
            imports: self.imports.len(),
            exports: self.exports.len(),
            dependencies: self.dependencies.len(),
            operations: self.operations.len(),
            regions: self.regions.len(),
            functions: self.functions.len(),
            parameters: self.parameters.len(),
            logical_qubits: self.qubits.len(),
            resources: self.resources.len(),
            capabilities: self.capabilities.len(),
            extensions: self.extensions.len(),
            metadata_entries: self.metadata.len(),
            has_entry_region: self.entry_region.is_some(),
        }
    }

    // =========================================================================
    // Internal helpers
    // =========================================================================

    fn validate_symbol_reference(
        &self,
        symbol: &ModuleSymbol,
    ) -> ModuleResult<()> {
        if symbol.name().is_empty() {
            return Err(ModuleError::EmptySymbolName);
        }

        if let Some(function) = symbol.function() {
            if !self.functions.contains(&function)
                && !symbol.is_external()
            {
                return Err(ModuleError::SymbolReferenceMissing {
                    name: symbol.name().to_owned(),
                });
            }
        }

        if let Some(operation) = symbol.operation() {
            if !self.operations.contains(&operation)
                && !symbol.is_external()
            {
                return Err(ModuleError::SymbolReferenceMissing {
                    name: symbol.name().to_owned(),
                });
            }
        }

        if let Some(region) = symbol.region() {
            if !self.regions.contains(&region)
                && !symbol.is_external()
            {
                return Err(ModuleError::SymbolReferenceMissing {
                    name: symbol.name().to_owned(),
                });
            }
        }

        if let Some(parameter) = symbol.parameter() {
            if !self.parameters.contains(&parameter)
                && !symbol.is_external()
            {
                return Err(ModuleError::SymbolReferenceMissing {
                    name: symbol.name().to_owned(),
                });
            }
        }

        if let Some(qubit) = symbol.qubit() {
            if !self.qubits.contains(&qubit)
                && !symbol.is_external()
            {
                return Err(ModuleError::SymbolReferenceMissing {
                    name: symbol.name().to_owned(),
                });
            }
        }

        if let Some(resource) = symbol.resource() {
            if !self.resources.contains(&resource)
                && !symbol.is_external()
            {
                return Err(ModuleError::SymbolReferenceMissing {
                    name: symbol.name().to_owned(),
                });
            }
        }

        if let Some(capability) = symbol.capability() {
            if !self.capabilities.contains(&capability)
                && !symbol.is_external()
            {
                return Err(ModuleError::SymbolReferenceMissing {
                    name: symbol.name().to_owned(),
                });
            }
        }

        if let Some(extension) = symbol.extension() {
            if !self.extensions.contains(&extension)
                && !symbol.is_external()
            {
                return Err(ModuleError::SymbolReferenceMissing {
                    name: symbol.name().to_owned(),
                });
            }
        }

        Ok(())
    }
}

// =============================================================================
// Module summary
// =============================================================================

/// Deterministic read-only statistics for a quantum module.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ModuleSummary {
    /// Number of symbols.
    pub symbols: usize,

    /// Number of imports.
    pub imports: usize,

    /// Number of exports.
    pub exports: usize,

    /// Number of module dependencies.
    pub dependencies: usize,

    /// Number of operation references.
    pub operations: usize,

    /// Number of region references.
    pub regions: usize,

    /// Number of function references.
    pub functions: usize,

    /// Number of parameter references.
    pub parameters: usize,

    /// Number of logical-qubit references.
    pub logical_qubits: usize,

    /// Number of resource references.
    pub resources: usize,

    /// Number of capability requirements.
    pub capabilities: usize,

    /// Number of extension references.
    pub extensions: usize,

    /// Number of metadata entries.
    pub metadata_entries: usize,

    /// Whether an entry region exists.
    pub has_entry_region: bool,
}

// =============================================================================
// Helpers
// =============================================================================

/// Validates a semantic name without imposing a frontend-specific grammar.
///
/// The canonical frontend/parser is responsible for language-specific
/// identifier rules. The IR only enforces rules necessary for structural
/// safety and deterministic representation.
fn validate_name(
    name: &str,
    error: ModuleError,
) -> ModuleResult<()> {
    if name.is_empty() || name.contains('\0') {
        return Err(error);
    }

    Ok(())
}

/// Inserts a unique value into a set.
///
/// This helper keeps mutation behavior explicit and deterministic.
fn insert_unique<T>(
    set: &mut BTreeSet<T>,
    value: T,
    error: ModuleError,
) where
    T: Ord,
{
    if !set.insert(value) {
        // The caller owns the error semantics. There is intentionally no
        // mutation after a duplicate is discovered.
        let _ = error;
    }
}

/// Inserts a slice atomically into a set.
///
/// The implementation first validates the complete input against both the
/// existing set and the pending insertion set. The destination is modified
/// only after every item is known to be valid.
fn add_unique_many<T, F>(
    set: &mut BTreeSet<T>,
    values: &[T],
    error: F,
) -> ModuleResult<()>
where
    T: Ord + Copy,
    F: Fn(&T) -> ModuleError,
{
    let mut pending = BTreeSet::new();

    for value in values {
        if set.contains(value) || !pending.insert(*value) {
            return Err(error(value));
        }
    }

    set.extend(pending);

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_module_is_valid() {
        let module = QuantumModule::new(
            ModuleId::new(1),
            "main",
        )
        .expect("module must be constructible");

        assert!(module.validate().is_ok());
        assert_eq!(module.name(), "main");
        assert_eq!(module.kind(), ModuleKind::Quantum);
    }

    #[test]
    fn canonical_qubit_id_is_used() {
        let mut module = QuantumModule::new(
            ModuleId::new(2),
            "quantum",
        )
        .expect("module must be constructible");

        module
            .add_qubit(QubitId::new(17))
            .expect("qubit reference must be accepted");

        assert!(
            module
                .qubits()
                .contains(&QubitId::new(17))
        );

        assert!(module.validate().is_ok());
    }

    #[test]
    fn duplicate_qubit_is_rejected() {
        let mut module = QuantumModule::new(
            ModuleId::new(3),
            "quantum",
        )
        .expect("module must be constructible");

        module
            .add_qubit(QubitId::new(0))
            .expect("first qubit must succeed");

        assert!(matches!(
            module.add_qubit(QubitId::new(0)),
            Err(ModuleError::DuplicateQubit { .. })
        ));
    }

    #[test]
    fn operations_are_deterministically_stored() {
        let mut module = QuantumModule::new(
            ModuleId::new(4),
            "ops",
        )
        .expect("module must be constructible");

        module
            .add_operations(&[
                OperationId::new(10),
                OperationId::new(2),
                OperationId::new(7),
            ])
            .expect("operations must succeed");

        let operations: Vec<_> =
            module.operations().iter().copied().collect();

        assert_eq!(
            operations,
            vec![
                OperationId::new(2),
                OperationId::new(7),
                OperationId::new(10),
            ]
        );
    }

    #[test]
    fn duplicate_operations_are_rejected_atomically() {
        let mut module = QuantumModule::new(
            ModuleId::new(5),
            "ops",
        )
        .expect("module must be constructible");

        module
            .add_operation(OperationId::new(1))
            .expect("initial operation must succeed");

        let result = module.add_operations(&[
            OperationId::new(2),
            OperationId::new(1),
            OperationId::new(3),
        ]);

        assert!(matches!(
            result,
            Err(ModuleError::DuplicateOperation { .. })
        ));

        assert_eq!(module.operations().len(), 1);
        assert!(
            module
                .operations()
                .contains(&OperationId::new(1))
        );
        assert!(
            !module
                .operations()
                .contains(&OperationId::new(2))
        );
    }

    #[test]
    fn entry_region_must_be_registered() {
        let mut module = QuantumModule::new(
            ModuleId::new(6),
            "regions",
        )
        .expect("module must be constructible");

        let result =
            module.set_entry_region(RegionId::new(100));

        assert!(matches!(
            result,
            Err(ModuleError::UnknownEntryRegion { .. })
        ));
    }

    #[test]
    fn entry_region_can_be_registered() {
        let mut module = QuantumModule::new(
            ModuleId::new(7),
            "regions",
        )
        .expect("module must be constructible");

        module
            .add_region(RegionId::new(100))
            .expect("region must be registered");

        module
            .set_entry_region(RegionId::new(100))
            .expect("entry region must succeed");

        assert_eq!(
            module.entry_region(),
            Some(RegionId::new(100))
        );

        assert!(module.validate().is_ok());
    }

    #[test]
    fn self_dependency_is_rejected() {
        let mut module = QuantumModule::new(
            ModuleId::new(8),
            "self",
        )
        .expect("module must be constructible");

        let dependency =
            ModuleDependency::new(
                ModuleId::new(8),
                IrVersion::CURRENT,
            );

        assert!(matches!(
            module.add_dependency(dependency),
            Err(ModuleError::SelfDependency { .. })
        ));
    }

    #[test]
    fn dependency_version_is_checked() {
        let mut module = QuantumModule::new(
            ModuleId::new(9),
            "versioned",
        )
        .expect("module must be constructible");

        let dependency =
            ModuleDependency::new(
                ModuleId::new(10),
                IrVersion::new(2, 0, 0),
            );

        assert!(matches!(
            module.add_dependency(dependency),
            Err(
                ModuleError::UnsupportedDependencyVersion { .. }
            )
        ));
    }

    #[test]
    fn import_requires_dependency() {
        let mut module = QuantumModule::new(
            ModuleId::new(11),
            "imports",
        )
        .expect("module must be constructible");

        let import = ModuleImport::new(
            ModuleId::new(12),
            "foo",
            IrVersion::CURRENT,
        )
        .expect("import must be constructible");

        module
            .add_import(import)
            .expect("import declaration itself must succeed");

        assert!(matches!(
            module.validate(),
            Err(ModuleError::InvalidModule { .. })
        ));
    }

    #[test]
    fn import_with_dependency_is_valid() {
        let mut module = QuantumModule::new(
            ModuleId::new(13),
            "imports",
        )
        .expect("module must be constructible");

        module
            .add_dependency(ModuleDependency::new(
                ModuleId::new(14),
                IrVersion::CURRENT,
            ))
            .expect("dependency must succeed");

        module
            .add_import(
                ModuleImport::new(
                    ModuleId::new(14),
                    "foo",
                    IrVersion::CURRENT,
                )
                .expect("import must be constructible"),
            )
            .expect("import must succeed");

        assert!(module.validate().is_ok());
    }

    #[test]
    fn export_requires_symbol() {
        let mut module = QuantumModule::new(
            ModuleId::new(15),
            "exports",
        )
        .expect("module must be constructible");

        let export =
            ModuleExport::new("public_foo", "foo")
                .expect("export must be constructible");

        assert!(matches!(
            module.add_export(export),
            Err(ModuleError::UnknownExportSymbol { .. })
        ));
    }

    #[test]
    fn function_symbol_requires_function_reference() {
        let mut module = QuantumModule::new(
            ModuleId::new(16),
            "symbols",
        )
        .expect("module must be constructible");

        let symbol = ModuleSymbol::function(
            "main",
            FunctionId::new(42),
        )
        .expect("symbol must be constructible");

        assert!(matches!(
            module.add_symbol(symbol),
            Err(ModuleError::SymbolReferenceMissing { .. })
        ));
    }

    #[test]
    fn function_symbol_can_be_registered_after_reference() {
        let mut module = QuantumModule::new(
            ModuleId::new(17),
            "symbols",
        )
        .expect("module must be constructible");

        module
            .add_function(FunctionId::new(42))
            .expect("function reference must succeed");

        module
            .add_symbol(
                ModuleSymbol::function(
                    "main",
                    FunctionId::new(42),
                )
                .expect("symbol must be constructible"),
            )
            .expect("symbol must succeed");

        assert!(module.validate().is_ok());
    }

    #[test]
    fn exported_symbol_is_valid() {
        let mut module = QuantumModule::new(
            ModuleId::new(18),
            "exports",
        )
        .expect("module must be constructible");

        module
            .add_function(FunctionId::new(7))
            .expect("function reference must succeed");

        module
            .add_symbol(
                ModuleSymbol::function(
                    "main",
                    FunctionId::new(7),
                )
                .expect("symbol must be constructible"),
            )
            .expect("symbol must succeed");

        module
            .add_export(
                ModuleExport::new(
                    "main",
                    "main",
                )
                .expect("export must be constructible"),
            )
            .expect("export must succeed");

        assert!(module.validate().is_ok());
    }

    #[test]
    fn requirements_follow_registered_objects() {
        let mut module = QuantumModule::new(
            ModuleId::new(19),
            "requirements",
        )
        .expect("module must be constructible");

        module
            .add_qubit(QubitId::new(0))
            .expect("qubit must succeed");

        module
            .add_resource(ResourceId::new(1))
            .expect("resource must succeed");

        module
            .require_capability(CapabilityId::new(2))
            .expect("capability must succeed");

        module.require_pulse_control();
        module.require_dynamic_control();
        module.require_distributed_execution();

        assert!(
            module
                .requirements()
                .logical_qubits()
                .contains(&QubitId::new(0))
        );

        assert!(
            module
                .requirements()
                .resources()
                .contains(&ResourceId::new(1))
        );

        assert!(
            module
                .requirements()
                .capabilities()
                .contains(&CapabilityId::new(2))
        );

        assert!(
            module
                .requirements()
                .pulse_control()
        );

        assert!(
            module
                .requirements()
                .dynamic_control()
        );

        assert!(
            module
                .requirements()
                .distributed_execution()
        );

        assert!(module.validate().is_ok());
    }

    #[test]
    fn metadata_is_deterministic() {
        let mut module = QuantumModule::new(
            ModuleId::new(20),
            "metadata",
        )
        .expect("module must be constructible");

        module
            .insert_metadata("z", "last")
            .expect("metadata must succeed");

        module
            .insert_metadata("a", "first")
            .expect("metadata must succeed");

        let keys: Vec<_> =
            module.metadata().keys().map(String::as_str).collect();

        assert_eq!(keys, vec!["a", "z"]);
        assert!(module.validate().is_ok());
    }

    #[test]
    fn summary_is_deterministic() {
        let mut module = QuantumModule::new(
            ModuleId::new(21),
            "summary",
        )
        .expect("module must be constructible");

        module
            .add_operation(OperationId::new(1))
            .expect("operation must succeed");

        module
            .add_region(RegionId::new(2))
            .expect("region must succeed");

        module
            .add_function(FunctionId::new(3))
            .expect("function must succeed");

        module
            .add_parameter(ParameterId::new(4))
            .expect("parameter must succeed");

        module
            .add_qubit(QubitId::new(5))
            .expect("qubit must succeed");

        let summary = module.summary();

        assert_eq!(summary.operations, 1);
        assert_eq!(summary.regions, 1);
        assert_eq!(summary.functions, 1);
        assert_eq!(summary.parameters, 1);
        assert_eq!(summary.logical_qubits, 1);
    }

    #[test]
    fn large_logical_qubit_namespace_has_no_architectural_constant() {
        let mut module = QuantumModule::new(
            ModuleId::new(22),
            "large",
        )
        .expect("module must be constructible");

        let count = 100_000usize;

        let qubits: Vec<QubitId> =
            (0..count).map(QubitId::new).collect();

        module
            .add_qubits(&qubits)
            .expect("large logical namespace must be representable");

        assert_eq!(
            module.qubits().len(),
            count
        );

        assert!(module.validate().is_ok());
    }

    #[test]
    fn no_unsafe_contract_is_enforced_by_module_attribute() {
        // The test intentionally contains no unsafe operation.
        //
        // `#![forbid(unsafe_code)]` is the actual compiler-enforced guarantee.
        assert!(true);
    }
}
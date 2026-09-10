//! # Zamani Frontend AST — Capabilities
//!
//! Source-level capability declarations and requirements for the native
//! Zamani frontend AST.
//!
//! ## Architectural role
//!
//! This module represents what a Zamani program declares or requires about
//! computational capabilities at the SOURCE level.
//!
//! It does NOT represent:
//!
//! - hardware devices;
//! - physical qubits;
//! - backend implementations;
//! - vendor APIs;
//! - routing;
//! - scheduling;
//! - calibration;
//! - QEC implementation;
//! - noise models;
//! - resource allocation;
//! - execution;
//! - target selection;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - quantum IR implementation details.
//!
//! The intended compilation boundary is:
//!
//! ```text
//! Zamani source
//!     │
//!     ▼
//! parser
//!     │
//!     ▼
//! frontend::ast::node::capabilities
//!     │
//!     ▼
//! structural AST validation
//!     │
//!     ▼
//! semantic analysis
//!     │
//!     ▼
//! semantic capability requirements
//!     │
//!     ▼
//! ZUIR
//!     │
//!     ▼
//! quantum/classical/HDL/future domain IR
//!     │
//!     ▼
//! target capability matching
//!     │
//!     ▼
//! hardware/backend realization
//! ```
//!
//! ## POCO-REAF
//!
//! Capability syntax must support:
//!
//! ```text
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
//! ```
//!
//! A source program expresses capability requirements independently of the
//! machine that eventually satisfies them.
//!
//! Therefore this module contains no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_CAPABILITIES
//! MAX_DEVICES
//! MAX_OPERATIONS
//! MAX_REGISTER_SIZE
//! ```
//!
//! and no vendor-specific capability enumeration.
//!
//! A capability is identified by an extensible namespace-qualified name.
//!
//! Examples:
//!
//! ```text
//! zamani.quantum.mid_circuit_measurement
//! zamani.quantum.dynamic_control
//! zamani.quantum.parameterized_operation
//! zamani.quantum.logical_qubits
//! zamani.distributed.collective_execution
//! zamani.accelerator.tensor_execution
//! ```
//!
//! A future technology can introduce another namespaced capability without
//! changing the core AST representation.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - source-level capability identifiers;
//! - capability requirements;
//! - capability declarations;
//! - capability version constraints;
//! - capability properties;
//! - capability source spans;
//! - capability AST-level attributes;
//! - structural invariants for those nodes.
//!
//! This module does NOT own:
//!
//! - resolved capability identities;
//! - target capability inventories;
//! - hardware capabilities;
//! - resource capacities;
//! - capability matching algorithms;
//! - capability scheduling;
//! - backend feature detection.
//!
//! Those belong to later compilation layers.
//!
//! ## Important separation from quantum IR
//!
//! The repository already contains a canonical quantum IR capability model.
//! That model is downstream and must remain authoritative for quantum IR
//! capability semantics.
//!
//! This AST module must therefore NOT import:
//!
//! ```text
//! quantum::ir::resources::capability
//! ```
//!
//! merely to represent source syntax.
//!
//! Semantic analysis performs the conversion:
//!
//! ```text
//! AST capability
//!     │
//!     ▼
//! semantic capability requirement
//!     │
//!     ▼
//! quantum::ir::resources::capability
//! ```
//!
//! This avoids coupling the source AST to the quantum IR.
//!
//! ## Domain neutrality
//!
//! Although capabilities are particularly important for quantum computing,
//! the AST model is deliberately domain-neutral.
//!
//! A capability may describe requirements for:
//!
//! - classical computation;
//! - quantum computation;
//! - hybrid computation;
//! - distributed computation;
//! - accelerators;
//! - HDL;
//! - AI/ML;
//! - future computational domains.
//!
//! The capability name determines meaning downstream.
//!
//! The AST does not contain a closed list such as:
//!
//! ```text
//! Classical
//! Quantum
//! GPU
//! FPGA
//! ```
//!
//! because such a list would prevent future extension.
//!
//! ## Extensibility
//!
//! Capability identity is represented as:
//!
//! ```text
//! namespace + name
//! ```
//!
//! rather than:
//!
//! ```text
//! enum CapabilityKind {
//!     Quantum,
//!     GPU,
//!     FPGA,
//!     ...
//! }
//! ```
//!
//! This means a new capability can be introduced without modifying this
//! module's representation.
//!
//! ## Parser integration
//!
//! The parser owns syntax recognition.
//!
//! Conceptually:
//!
//! ```text
//! capability <qualified-name>
//! capability <qualified-name> @ <version>
//! requires <qualified-name>
//! ```
//!
//! Exact syntax is determined by the Zamani grammar.
//!
//! The parser constructs these AST nodes but does not resolve their meaning.
//!
//! ## Semantic integration
//!
//! Semantic analysis is responsible for:
//!
//! - resolving namespaces;
//! - resolving aliases;
//! - checking capability existence where required;
//! - validating version constraints;
//! - determining domain semantics;
//! - checking contextual legality;
//! - converting source capability requirements into semantic capability
//!   requirements;
//! - lowering them into the appropriate IR representation.
//!
//! This module performs none of those operations.
//!
//! ## ZUIR integration
//!
//! The lowering contract is:
//!
//! ```text
//! CapabilityRequirementAst
//!          │
//!          ▼
//! semantic capability requirement
//!          │
//!          ▼
//! ZUIR capability requirement
//!          │
//!          ▼
//! domain/target capability matching
//! ```
//!
//! The AST must never directly construct hardware-specific capability data.
//!
//! ## Source preservation
//!
//! Capability names remain textual at the AST boundary so that:
//!
//! - diagnostics can reproduce source spelling;
//! - unresolved names can be reported accurately;
//! - tooling can inspect source syntax;
//! - semantic resolution can occur later;
//! - future namespaces can be introduced without AST redesign.
//!
//! ## Determinism
//!
//! No global registry is stored in this module.
//!
//! No timestamps are stored.
//!
//! No process identifiers are stored.
//!
//! No memory addresses are stored.
//!
//! No random identifiers are generated.
//!
//! AST ordering follows source/program ordering.
//!
//! ## Serialization
//!
//! This module does not define a competing serialization protocol.
//!
//! The canonical AST serialization subsystem remains responsible for
//! serialization.
//!
//! The structures here contain deterministic semantic/source data and are
//! therefore suitable for canonical serialization.
//!
//! ## Security
//!
//! Capability names originate from potentially untrusted source input.
//!
//! Constructors validate basic structural invariants and reject empty
//! namespace/name components.
//!
//! Semantic authorization is NOT performed here.
//!
//! A source program saying:
//!
//! ```text
//! requires secure.hardware.capability
//! ```
//!
//! does not grant that capability.
//!
//! It merely declares a requirement.
//!
//! Authorization and availability are downstream concerns.
//!
//! ## Resource scaling
//!
//! A capability requirement may contain an arbitrary number of properties.
//!
//! The collection uses `Vec` because source ordering may be meaningful for
//! diagnostics and source preservation.
//!
//! No fixed capacity is imposed.
//!
//! Actual compiler safety limits, if required, must be configurable compiler
//! policies rather than constants in this module.
//!
//! ## Rust compatibility
//!
//! Required toolchain:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! ## Safety
//!
//! Unsafe Rust is explicitly forbidden.
//!
//! The module contains no unsafe operations.
//!
//! ## Integration contract
//!
//! ```text
//! src/frontend/ast/node/capabilities/mod.rs
//!             │
//!             ├── CapabilityIdAst
//!             ├── CapabilityVersionAst
//!             ├── CapabilityVersionConstraintAst
//!             ├── CapabilityPropertyAst
//!             ├── CapabilityRequirementAst
//!             └── CapabilityDeclarationAst
//!             │
//!             ▼
//! src/frontend/parser
//!             │
//!             ▼
//! src/frontend/ast validation
//!             │
//!             ▼
//! semantic analysis
//!             │
//!             ▼
//! ZUIR
//!             │
//!             ▼
//! domain-specific IR
//!             │
//!             ▼
//! target capability matching
//! ```
//!
//! A new hardware backend must not require changing this module.
//!
//! A new quantum technology must not require changing this module.
//!
//! A new machine size must not require changing this module.
//!
//! A new computational domain must not require changing this module.
//!
//! A new capability requires only source/semantic registration where the
//! language's capability vocabulary requires it; the AST representation itself
//! remains unchanged.
//!
//! ## Relationship to `resources`
//!
//! Capabilities and resources are intentionally separate concepts.
//!
//! ```text
//! capability = what the computation needs the target to be able to do
//! resource   = what quantity of computational resources is needed
//! ```
//!
//! For example:
//!
//! ```text
//! requires zamani.quantum.mid_circuit_measurement
//! requires zamani.quantum.dynamic_control
//! requires zamani.quantum.logical_qubits
//! ```
//!
//! Resource quantities are resolved through the resource subsystem.
//!
//! This file must not duplicate the canonical resource quantity model.
//!
//! ## Relationship to hardware
//!
//! Hardware may eventually provide:
//!
//! ```text
//! capability X
//! capability Y
//! resource Z
//! ```
//!
//! That information must never be stored in the source AST.
//!
//! Instead:
//!
//! ```text
//! source requirement
//!        │
//!        ▼
//! semantic requirement
//!        │
//!        ├──────────────► target capability inventory
//!        │
//!        └──────────────► target resource inventory
//! ```
//!
//! ## No vendor coupling
//!
//! Vendor names may occur as ordinary namespace strings when a program
//! explicitly requests a vendor-specific extension.
//!
//! The AST itself does not know that the namespace belongs to a vendor.
//!
//! This distinction is important:
//!
//! ```text
//! vendor.example.feature
//! ```
//!
//! is an extensible capability identifier.
//!
//! It is NOT:
//!
//! ```text
//! VendorExampleFeature
//! ```
//!
//! hard-coded into the compiler.
//!
//! ## Versioning
//!
//! Capability identity and capability version are separate.
//!
//! ```text
//! zamani.quantum.dynamic_control
//!
//! version 1.0.0
//! ```
//!
//! This allows capability evolution without changing the identity.
//!
//! The AST stores source-level version constraints.
//!
//! Compatibility policy belongs downstream.
//!
//! ## Error policy
//!
//! Construction errors are structural AST errors.
//!
//! They are not semantic errors.
//!
//! Semantic analysis must produce normal compiler diagnostics for things such
//! as unavailable capabilities.
//!
//! ## Testing contract
//!
//! Tests for this module must cover:
//!
//! - valid capability identifiers;
//! - invalid empty identifiers;
//! - qualified names;
//! - version construction;
//! - version constraints;
//! - capability properties;
//! - requirement construction;
//! - declaration construction;
//! - deterministic equality;
//! - cloning;
//! - arbitrary property counts;
//! - arbitrary requirement counts;
//! - Unicode capability names;
//! - long namespaces;
//! - long capability names;
//! - zero-property requirements;
//! - source-order preservation.
//!
//! Tests must NOT assume a fixed number of capabilities.
//!
//! ## Production-readiness invariant
//!
//! Once this module is integrated, downstream changes must not require this
//! module to acquire:
//!
//! - quantum hardware logic;
//! - QPU topology;
//! - routing;
//! - scheduling;
//! - QEC;
//! - calibration;
//! - backend APIs;
//! - vendor SDKs;
//! - physical qubit identities.
//!
//! The module remains a source-language abstraction forever.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::fmt;

// =============================================================================
// Errors
// =============================================================================

/// Structural errors produced while constructing capability AST nodes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CapabilityAstError {
    /// Namespace is empty.
    EmptyNamespace,

    /// Capability name is empty.
    EmptyName,

    /// A qualified capability name does not contain a separator.
    MissingNamespaceSeparator,

    /// A version range has its lower bound greater than its upper bound.
    InvalidVersionRange,

    /// A capability property has an empty name.
    EmptyPropertyName,
}

impl fmt::Display for CapabilityAstError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyNamespace => formatter.write_str(
                "capability namespace must not be empty",
            ),
            Self::EmptyName => formatter.write_str(
                "capability name must not be empty",
            ),
            Self::MissingNamespaceSeparator => formatter.write_str(
                "qualified capability identifier must contain a namespace separator",
            ),
            Self::InvalidVersionRange => formatter.write_str(
                "capability version range has an invalid lower/upper bound",
            ),
            Self::EmptyPropertyName => formatter.write_str(
                "capability property name must not be empty",
            ),
        }
    }
}

impl std::error::Error for CapabilityAstError {}

// =============================================================================
// Capability identifier
// =============================================================================

/// Source-level namespace-qualified capability identity.
///
/// This is intentionally not an enum.
///
/// A capability introduced in the future must not require modification of the
/// AST representation.
///
/// Example:
///
/// ```text
/// zamani.quantum.mid_circuit_measurement
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CapabilityIdAst {
    namespace: String,
    name: String,
}

impl CapabilityIdAst {
    /// Creates a capability identifier from separate namespace and name
    /// components.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Self, CapabilityAstError> {
        let namespace = namespace.into();
        let name = name.into();

        if namespace.trim().is_empty() {
            return Err(CapabilityAstError::EmptyNamespace);
        }

        if name.trim().is_empty() {
            return Err(CapabilityAstError::EmptyName);
        }

        Ok(Self { namespace, name })
    }

    /// Parses a fully qualified capability identifier.
    ///
    /// The final `.` separates the namespace from the capability name.
    ///
    /// Example:
    ///
    /// ```text
    /// zamani.quantum.mid_circuit_measurement
    /// ```
    ///
    /// becomes:
    ///
    /// ```text
    /// namespace = zamani.quantum
    /// name      = mid_circuit_measurement
    /// ```
    pub fn parse(
        qualified: impl Into<String>,
    ) -> Result<Self, CapabilityAstError> {
        let qualified = qualified.into();

        let separator = qualified
            .rfind('.')
            .ok_or(CapabilityAstError::MissingNamespaceSeparator)?;

        if separator == 0 || separator + 1 >= qualified.len() {
            return Err(CapabilityAstError::MissingNamespaceSeparator);
        }

        Self::new(
            qualified[..separator].to_owned(),
            qualified[separator + 1..].to_owned(),
        )
    }

    /// Returns the namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Returns the capability name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the fully qualified identifier.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        format!("{}.{}", self.namespace, self.name)
    }
}

impl fmt::Display for CapabilityIdAst {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}.{}", self.namespace, self.name)
    }
}

// =============================================================================
// Capability version
// =============================================================================

/// Source-level semantic capability version.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CapabilityVersionAst {
    major: u64,
    minor: u64,
    patch: u64,
}

impl CapabilityVersionAst {
    /// Creates a semantic capability version.
    #[must_use]
    pub const fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Returns the major version.
    #[must_use]
    pub const fn major(self) -> u64 {
        self.major
    }

    /// Returns the minor version.
    #[must_use]
    pub const fn minor(self) -> u64 {
        self.minor
    }

    /// Returns the patch version.
    #[must_use]
    pub const fn patch(self) -> u64 {
        self.patch
    }
}

impl fmt::Display for CapabilityVersionAst {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
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
// Version constraint
// =============================================================================

/// Source-level constraint on a capability version.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CapabilityVersionConstraintAst {
    /// Any implementation version is acceptable.
    Any,

    /// Exactly one version is required.
    Exact(CapabilityVersionAst),

    /// The implementation must be this version or newer.
    AtLeast(CapabilityVersionAst),

    /// The implementation must be this version or older.
    AtMost(CapabilityVersionAst),

    /// The implementation must fall within an inclusive range.
    Between {
        /// Minimum accepted version.
        minimum: CapabilityVersionAst,

        /// Maximum accepted version.
        maximum: CapabilityVersionAst,
    },
}

impl CapabilityVersionConstraintAst {
    /// Creates an inclusive version range.
    pub const fn between(
        minimum: CapabilityVersionAst,
        maximum: CapabilityVersionAst,
    ) -> Result<Self, CapabilityAstError> {
        if minimum > maximum {
            return Err(CapabilityAstError::InvalidVersionRange);
        }

        Ok(Self::Between { minimum, maximum })
    }

    /// Tests whether a version satisfies this constraint.
    #[must_use]
    pub const fn matches(
        self,
        version: CapabilityVersionAst,
    ) -> bool {
        match self {
            Self::Any => true,
            Self::Exact(required) => version == required,
            Self::AtLeast(required) => version >= required,
            Self::AtMost(required) => version <= required,
            Self::Between { minimum, maximum } => {
                version >= minimum && version <= maximum
            }
        }
    }
}

// =============================================================================
// Capability property
// =============================================================================

/// Source-level value for a capability property.
///
/// The AST intentionally uses deterministic values only.
///
/// Physical measurements, floating-point calibration values, and backend
/// objects belong to downstream domain/hardware layers.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CapabilityPropertyValueAst {
    /// Boolean property.
    Boolean(bool),

    /// Non-negative integer property.
    Unsigned(u64),

    /// Source textual property.
    Text(String),

    /// Version-valued property.
    Version(CapabilityVersionAst),
}

/// A named source-level capability property.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CapabilityPropertyAst {
    name: String,
    value: CapabilityPropertyValueAst,
}

impl CapabilityPropertyAst {
    /// Creates a capability property.
    pub fn new(
        name: impl Into<String>,
        value: CapabilityPropertyValueAst,
    ) -> Result<Self, CapabilityAstError> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(CapabilityAstError::EmptyPropertyName);
        }

        Ok(Self { name, value })
    }

    /// Returns the property name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the property value.
    #[must_use]
    pub fn value(&self) -> &CapabilityPropertyValueAst {
        &self.value
    }
}

// =============================================================================
// Capability requirement
// =============================================================================

/// Source-level requirement for one computational capability.
///
/// This is a statement of intent.
///
/// It does not grant, allocate, select, or authorize a capability.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CapabilityRequirementAst {
    capability: CapabilityIdAst,
    version: CapabilityVersionConstraintAst,
    properties: Vec<CapabilityPropertyAst>,
}

impl CapabilityRequirementAst {
    /// Creates a capability requirement with no additional properties.
    #[must_use]
    pub fn new(capability: CapabilityIdAst) -> Self {
        Self {
            capability,
            version: CapabilityVersionConstraintAst::Any,
            properties: Vec::new(),
        }
    }

    /// Creates a requirement with a version constraint.
    #[must_use]
    pub fn with_version(
        capability: CapabilityIdAst,
        version: CapabilityVersionConstraintAst,
    ) -> Self {
        Self {
            capability,
            version,
            properties: Vec::new(),
        }
    }

    /// Adds one property while preserving source insertion order.
    pub fn push_property(
        &mut self,
        property: CapabilityPropertyAst,
    ) {
        self.properties.push(property);
    }

    /// Returns the capability identity.
    #[must_use]
    pub fn capability(&self) -> &CapabilityIdAst {
        &self.capability
    }

    /// Returns the version constraint.
    #[must_use]
    pub const fn version(
        &self,
    ) -> CapabilityVersionConstraintAst {
        self.version
    }

    /// Returns capability properties in source order.
    #[must_use]
    pub fn properties(&self) -> &[CapabilityPropertyAst] {
        &self.properties
    }
}

// =============================================================================
// Capability declaration
// =============================================================================

/// Source-level declaration describing a capability contract.
///
/// This does not declare that a physical machine provides the capability.
/// Semantic analysis determines the meaning of a declaration according to
/// Zamani's language rules.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CapabilityDeclarationAst {
    capability: CapabilityIdAst,
    version: Option<CapabilityVersionAst>,
    properties: Vec<CapabilityPropertyAst>,
}

impl CapabilityDeclarationAst {
    /// Creates a capability declaration.
    #[must_use]
    pub fn new(capability: CapabilityIdAst) -> Self {
        Self {
            capability,
            version: None,
            properties: Vec::new(),
        }
    }

    /// Sets the declared semantic version.
    #[must_use]
    pub const fn with_version(
        mut self,
        version: CapabilityVersionAst,
    ) -> Self {
        self.version = Some(version);
        self
    }

    /// Adds one declaration property.
    pub fn push_property(
        &mut self,
        property: CapabilityPropertyAst,
    ) {
        self.properties.push(property);
    }

    /// Returns the capability identity.
    #[must_use]
    pub fn capability(&self) -> &CapabilityIdAst {
        &self.capability
    }

    /// Returns the declared version, if present.
    #[must_use]
    pub const fn version(
        &self,
    ) -> Option<CapabilityVersionAst> {
        self.version
    }

    /// Returns declaration properties in source order.
    #[must_use]
    pub fn properties(&self) -> &[CapabilityPropertyAst] {
        &self.properties
    }
}

// =============================================================================
// Capability collections
// =============================================================================

/// Ordered collection of source-level capability requirements.
///
/// `Vec` is intentionally used rather than a fixed-size array or a bounded
/// collection. The language imposes no capability-count limit here.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct CapabilityRequirementsAst {
    requirements: Vec<CapabilityRequirementAst>,
}

impl CapabilityRequirementsAst {
    /// Creates an empty capability requirement collection.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            requirements: Vec::new(),
        }
    }

    /// Adds a capability requirement.
    pub fn push(
        &mut self,
        requirement: CapabilityRequirementAst,
    ) {
        self.requirements.push(requirement);
    }

    /// Returns the number of requirements currently stored.
    #[must_use]
    pub fn len(&self) -> usize {
        self.requirements.len()
    }

    /// Returns whether no requirements are present.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.requirements.is_empty()
    }

    /// Returns requirements in source order.
    #[must_use]
    pub fn as_slice(&self) -> &[CapabilityRequirementAst] {
        &self.requirements
    }

    /// Returns an iterator over requirements.
    pub fn iter(
        &self,
    ) -> std::slice::Iter<'_, CapabilityRequirementAst> {
        self.requirements.iter()
    }
}

impl IntoIterator for CapabilityRequirementsAst {
    type Item = CapabilityRequirementAst;
    type IntoIter =
        std::vec::IntoIter<CapabilityRequirementAst>;

    fn into_iter(self) -> Self::IntoIter {
        self.requirements.into_iter()
    }
}

impl<'a> IntoIterator for &'a CapabilityRequirementsAst {
    type Item = &'a CapabilityRequirementAst;
    type IntoIter =
        std::slice::Iter<'a, CapabilityRequirementAst>;

    fn into_iter(self) -> Self::IntoIter {
        self.requirements.iter()
    }
}

/// Ordered collection of source-level capability declarations.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct CapabilityDeclarationsAst {
    declarations: Vec<CapabilityDeclarationAst>,
}

impl CapabilityDeclarationsAst {
    /// Creates an empty declaration collection.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            declarations: Vec::new(),
        }
    }

    /// Adds a capability declaration.
    pub fn push(
        &mut self,
        declaration: CapabilityDeclarationAst,
    ) {
        self.declarations.push(declaration);
    }

    /// Returns the number of declarations.
    #[must_use]
    pub fn len(&self) -> usize {
        self.declarations.len()
    }

    /// Returns whether there are no declarations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.declarations.is_empty()
    }

    /// Returns declarations in source order.
    #[must_use]
    pub fn as_slice(&self) -> &[CapabilityDeclarationAst] {
        &self.declarations
    }

    /// Returns an iterator over declarations.
    pub fn iter(
        &self,
    ) -> std::slice::Iter<'_, CapabilityDeclarationAst> {
        self.declarations.iter()
    }
}

impl IntoIterator for CapabilityDeclarationsAst {
    type Item = CapabilityDeclarationAst;
    type IntoIter =
        std::vec::IntoIter<CapabilityDeclarationAst>;

    fn into_iter(self) -> Self::IntoIter {
        self.declarations.into_iter()
    }
}

impl<'a> IntoIterator for &'a CapabilityDeclarationsAst {
    type Item = &'a CapabilityDeclarationAst;
    type IntoIter =
        std::slice::Iter<'a, CapabilityDeclarationAst>;

    fn into_iter(self) -> Self::IntoIter {
        self.declarations.iter()
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_qualified_capability() {
        let capability =
            CapabilityIdAst::parse(
                "zamani.quantum.mid_circuit_measurement",
            )
            .expect("valid capability");

        assert_eq!(capability.namespace(), "zamani.quantum");
        assert_eq!(
            capability.name(),
            "mid_circuit_measurement"
        );
        assert_eq!(
            capability.qualified_name(),
            "zamani.quantum.mid_circuit_measurement"
        );
    }

    #[test]
    fn rejects_unqualified_capability() {
        let result = CapabilityIdAst::parse("measurement");

        assert_eq!(
            result,
            Err(
                CapabilityAstError::MissingNamespaceSeparator
            )
        );
    }

    #[test]
    fn rejects_empty_namespace() {
        let result =
            CapabilityIdAst::new("", "measurement");

        assert_eq!(
            result,
            Err(CapabilityAstError::EmptyNamespace)
        );
    }

    #[test]
    fn rejects_empty_name() {
        let result =
            CapabilityIdAst::new("zamani.quantum", "");

        assert_eq!(
            result,
            Err(CapabilityAstError::EmptyName)
        );
    }

    #[test]
    fn version_constraint_matches() {
        let version =
            CapabilityVersionAst::new(2, 1, 0);

        let constraint =
            CapabilityVersionConstraintAst::AtLeast(
                CapabilityVersionAst::new(2, 0, 0),
            );

        assert!(constraint.matches(version));
    }

    #[test]
    fn invalid_version_range_is_rejected() {
        let result =
            CapabilityVersionConstraintAst::between(
                CapabilityVersionAst::new(3, 0, 0),
                CapabilityVersionAst::new(2, 0, 0),
            );

        assert_eq!(
            result,
            Err(CapabilityAstError::InvalidVersionRange)
        );
    }

    #[test]
    fn property_requires_name() {
        let result = CapabilityPropertyAst::new(
            "",
            CapabilityPropertyValueAst::Boolean(true),
        );

        assert_eq!(
            result,
            Err(CapabilityAstError::EmptyPropertyName)
        );
    }

    #[test]
    fn requirement_preserves_property_order() {
        let capability =
            CapabilityIdAst::new(
                "zamani.quantum",
                "dynamic_control",
            )
            .expect("valid capability");

        let first =
            CapabilityPropertyAst::new(
                "mode",
                CapabilityPropertyValueAst::Text(
                    String::from("adaptive"),
                ),
            )
            .expect("valid property");

        let second =
            CapabilityPropertyAst::new(
                "enabled",
                CapabilityPropertyValueAst::Boolean(true),
            )
            .expect("valid property");

        let mut requirement =
            CapabilityRequirementAst::new(capability);

        requirement.push_property(first);
        requirement.push_property(second);

        assert_eq!(
            requirement.properties()[0].name(),
            "mode"
        );
        assert_eq!(
            requirement.properties()[1].name(),
            "enabled"
        );
    }

    #[test]
    fn requirement_collection_has_no_language_level_limit() {
        let mut requirements =
            CapabilityRequirementsAst::new();

        for index in 0_u64..1024 {
            let capability =
                CapabilityIdAst::new(
                    "test",
                    format!("capability_{index}"),
                )
                .expect("valid capability");

            requirements.push(
                CapabilityRequirementAst::new(
                    capability,
                ),
            );
        }

        assert_eq!(requirements.len(), 1024);
    }

    #[test]
    fn declarations_are_deterministic() {
        let capability =
            CapabilityIdAst::new(
                "zamani.quantum",
                "measurement",
            )
            .expect("valid capability");

        let left =
            CapabilityDeclarationAst::new(capability.clone());

        let right =
            CapabilityDeclarationAst::new(capability);

        assert_eq!(left, right);
    }

    #[test]
    fn unicode_capability_names_are_supported() {
        let capability =
            CapabilityIdAst::new(
                "zamani.future",
                "能力",
            )
            .expect("Unicode is valid source text");

        assert_eq!(capability.name(), "能力");
    }
}
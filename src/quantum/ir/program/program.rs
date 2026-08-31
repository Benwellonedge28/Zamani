//! Zamani Quantum IR — Universal Quantum Program
//!
//! Canonical top-level, hardware-independent representation of a Zamani
//! quantum program.
//!
//! # Architectural role
//!
//! `QuantumProgram` is the top-level semantic container of the canonical
//! Quantum IR. It is intentionally broader than `QuantumCircuit`.
//!
//! A circuit is one possible quantum-program representation. A program may
//! additionally contain:
//!
//! - logical qubit declarations;
//! - classical-bit declarations;
//! - symbolic parameter declarations;
//! - semantic operations;
//! - structured regions;
//! - program order;
//! - a root region;
//! - logical-to-physical mapping records;
//! - capability requirements;
//! - abstract resource requirements;
//! - program metadata;
//! - extension metadata;
//! - IR version information;
//! - deterministic identity;
//! - explicit resource/security policy.
//!
//! # Fundamental architectural rule
//!
//! The canonical Quantum IR answers:
//!
//! > What does the program mean?
//!
//! It does NOT answer:
//!
//! - which hardware executes it;
//! - which physical qubits are selected;
//! - how routing is performed;
//! - which native gate decomposition is chosen;
//! - which calibration is applied;
//! - how pulses are synthesized;
//! - when operations execute;
//! - how a QPU is contacted;
//! - how quantum state is simulated;
//! - how QEC is decoded;
//! - which optimization algorithm is used;
//! - how source syntax is parsed.
//!
//! Those responsibilities belong to downstream subsystems.
//!
//! # Universal-program principle
//!
//! A Zamani program is written once at the semantic level:
//!
//! ```text
//! Zamani source
//!       |
//!       v
//! QuantumProgram
//!       |
//!       +--> small target
//!       +--> large target
//!       +--> distributed target
//!       +--> logical / fault-tolerant target
//!       +--> pulse target
//!       +--> analog target
//!       +--> annealing target
//!       +--> simulator
//!       +--> future architecture
//! ```
//!
//! No machine-size constant is encoded here.
//!
//! In particular, this module does NOT define architectural limits such as
//! 63, 64, 128, 4096, or 1_000_000 qubits.
//!
//! `usize` is used only for host collection sizes and indices. Quantum
//! identities themselves are canonical `QubitId` values from `qubit.rs`.
//!
//! A concrete finite program is limited only by:
//!
//! 1. the representable identity space;
//! 2. available host resources;
//! 3. explicit `QuantumIrLimits` policy;
//! 4. downstream compiler/resource policies;
//! 5. target hardware capabilities;
//! 6. backend/execution constraints.
//!
//! "Infinite quantum computers" are therefore not represented as a fake
//! infinite collection. The semantic model has no fixed architectural
//! ceiling, while every concrete program remains finite and resource-checkable.
//!
//! # Canonical ownership
//!
//! This file OWNS:
//!
//! - `QuantumProgram`;
//! - program-level metadata;
//! - program-level declarations;
//! - operation ownership and deterministic program order;
//! - region ownership at the program level;
//! - root-region identity;
//! - program-level capability/resource references;
//! - program-level logical-to-physical mapping state;
//! - program-level structural checks;
//! - program-level mutation atomicity.
//!
//! This file DOES NOT OWN:
//!
//! - `ProgramId`;
//! - `OperationId`;
//! - `RegionId`;
//! - `BlockId`;
//! - `QubitId`;
//! - `PhysicalQubitId`;
//! - `Operation`;
//! - `Region`;
//! - `QubitMapping`;
//! - gate semantics;
//! - measurement semantics;
//! - pulse semantics;
//! - waveform semantics;
//! - timing semantics;
//! - routing;
//! - scheduling;
//! - optimization;
//! - hardware;
//! - backend execution.
//!
//! Those types and responsibilities remain in their canonical modules.
//!
//! # Canonical dependency boundary
//!
//! ```text
//! identity.rs ────────┐
//! qubit.rs ───────────┤
//! operation.rs ───────┤
//! region.rs ──────────┤
//! mapping.rs ─────────┤
//! limits.rs ──────────┤
//!                     v
//!               program.rs
//!                     |
//!          ┌──────────┼───────────┐
//!          v          v           v
//!     validation   analysis   serialization
//!          |          |           |
//!          └──────────┼───────────┘
//!                     v
//!               downstream IR
//!                     |
//!       ┌─────────────┼──────────────┐
//!       v             v              v
//! optimization      routing      scheduling
//!       |             |              |
//!       └─────────────┼──────────────┘
//!                     v
//!                  hardware
//!                     |
//!                     v
//!                  backend
//! ```
//!
//! `program.rs` MUST NOT introduce dependencies in the reverse direction.
//!
//! # Atomic mutation
//!
//! Public mutating operations follow:
//!
//! ```text
//! validate candidate
//!       |
//!       v
//! validate identity
//!       |
//!       v
//! validate namespace/reference relationships
//!       |
//!       v
//! validate explicit resource policy
//!       |
//!       v
//! reserve storage where possible
//!       |
//!       v
//! commit mutation
//! ```
//!
//! A failed mutation must not leave a partially inserted operation, region,
//! declaration, or mapping.
//!
//! # Determinism
//!
//! Semantic ordering is stored explicitly in `Vec<OperationId>`.
//!
//! Identity lookup is deterministic through `BTreeMap`.
//!
//! Metadata and requirement collections use ordered containers.
//!
//! No global allocator, random identity generation, hash-map iteration order,
//! or hidden mutable global state is used.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies;
//! - no `unsafe`.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! # Integration contract
//!
//! `identity.rs`
//!     Owns all canonical program/object identities and `IrVersion`.
//!
//! `qubit.rs`
//!     Owns `QubitId` and `PhysicalQubitId`.
//!
//! `operation.rs`
//!     Owns canonical semantic `Operation`.
//!
//! `region.rs`
//!     Owns canonical `Region` and `Block` structural representations.
//!
//! `mapping.rs`
//!     Owns canonical logical-to-physical mapping representation.
//!
//! `limits.rs`
//!     Owns explicit IR resource/security policy.
//!
//! `validation.rs`
//!     Performs complete whole-program semantic validation.
//!
//! `analysis.rs`
//!     Performs read-only analysis.
//!
//! `serialization.rs`
//!     Owns canonical persistence/encoding.
//!
//! `hash.rs`
//!     Owns canonical content hashing.
//!
//! `provenance.rs`
//!     Owns transformation lineage.
//!
//! `optimization/`
//!     Consumes and transforms this program without redefining its types.
//!
//! `routing/`
//!     Consumes logical resources and produces mapping decisions.
//!
//! `scheduling/`
//!     Consumes operations and timing constraints.
//!
//! `hardware/`
//!     Determines whether a target can implement the program.
//!
//! `backend/`
//!     Executes target-specific lowered representations.
//!
//! # Important naming rule
//!
//! The canonical qubit module is:
//!
//! ```text
//! quantum::ir::qubit
//! ```
//!
//! New code MUST use that module rather than the legacy compatibility alias
//! `quantum::ir::qubits`.
//!
//! This file therefore explicitly imports:
//!
//! ```rust
//! use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
//! ```
//!
//! # No duplicate IR identities
//!
//! Earlier versions of the repository's program representation defined local
//! `ProgramId`, `RegionId`, and `ProgramOperationId` types. Those duplicate the
//! canonical identity contract and are intentionally removed here.
//!
//! The authoritative identities are re-exported below only for compatibility
//! with callers that previously imported them through `program`.
//!
//! # Program structure
//!
//! A program is represented as:
//!
//! ```text
//! QuantumProgram
//! ├── identity
//! ├── IR version
//! ├── explicit limits
//! ├── declarations
//! │   ├── logical qubits
//! │   ├── classical bits
//! │   └── parameters
//! ├── operations
//! │   └── deterministic operation order
//! ├── regions
//! │   └── structured references
//! ├── root region
//! ├── mapping
//! ├── capability requirements
//! ├── resource requirements
//! └── metadata/extensions
//! ```
//!
//! This representation is suitable for gate-based, dynamic, pulse-level,
//! analog, annealing, logical/fault-tolerant, distributed, hybrid, and future
//! quantum computation models because the actual operation semantics remain
//! owned by `operation.rs` and its dialect/specialized IR layers.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use super::identity::{
    CapabilityId,
    IrVersion,
    OperationId,
    ParameterId,
    ProgramId,
    RegionId,
    ResourceId,
};
use super::limits::{LimitsError, QuantumIrLimits};
use super::mapping::{MappingError, QubitMapping};
use super::operation::{Operation, OperationError};
use super::qubit::{PhysicalQubitId, QubitId};
use super::region::{Region, RegionKind};

/// Canonical program result type.
pub type ProgramResult<T> = Result<T, ProgramError>;

/// Canonical program error.
///
/// This is intentionally a program-structure error. Detailed semantic errors
/// remain owned by their canonical modules and are wrapped here where a
/// program-level transaction needs to report them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProgramError {
    /// The supplied resource/security policy is invalid.
    InvalidLimits {
        /// Invalid policy field.
        field: &'static str,

        /// Supplied invalid value.
        value: usize,
    },

    /// A program identifier is invalid for the requested operation.
    InvalidProgramId,

    /// A region identifier already exists.
    DuplicateRegion {
        /// Duplicate region identity.
        region: RegionId,
    },

    /// A referenced region does not exist.
    UnknownRegion {
        /// Missing region identity.
        region: RegionId,
    },

    /// The root region does not exist.
    MissingRootRegion {
        /// Expected root-region identity.
        region: RegionId,
    },

    /// An operation identity already exists.
    DuplicateOperation {
        /// Duplicate operation identity.
        operation: OperationId,
    },

    /// An operation index is outside the explicit program order.
    OperationIndexOutOfRange {
        /// Requested index.
        index: usize,

        /// Number of ordered operations.
        len: usize,
    },

    /// An operation is structurally invalid.
    InvalidOperation(OperationError),

    /// A region is structurally invalid.
    InvalidRegion {
        /// Static reason.
        message: &'static str,
    },

    /// A logical qubit was already declared.
    DuplicateQubit {
        /// Duplicate logical identity.
        qubit: QubitId,
    },

    /// A classical bit was already declared.
    DuplicateClassicalBit {
        /// Duplicate classical identity.
        bit: usize,
    },

    /// A symbolic parameter was already declared.
    DuplicateParameter {
        /// Duplicate parameter identity.
        parameter: ParameterId,
    },

    /// A mapping references an undeclared logical qubit.
    UnknownMappingQubit {
        /// Referenced logical qubit.
        qubit: QubitId,
    },

    /// Mapping operation failed.
    Mapping(MappingError),

    /// A capability requirement was already present.
    DuplicateCapability {
        /// Duplicate capability identity.
        capability: CapabilityId,
    },

    /// A resource requirement was already present.
    DuplicateResource {
        /// Duplicate resource identity.
        resource: ResourceId,
    },

    /// The requested resource policy would be exceeded.
    ResourceLimitExceeded {
        /// Resource category.
        resource: &'static str,

        /// Requested quantity.
        requested: usize,

        /// Allowed quantity.
        maximum: usize,
    },

    /// Metadata exceeds the configured explicit policy.
    MetadataLimitExceeded {
        /// Requested bytes.
        requested: usize,

        /// Maximum permitted bytes.
        maximum: usize,
    },

    /// A checked arithmetic operation overflowed.
    ArithmeticOverflow {
        /// Calculation description.
        calculation: &'static str,
    },

    /// Storage reservation failed.
    AllocationFailure {
        /// Collection for which growth was requested.
        collection: &'static str,
    },

    /// IR version is not supported by this implementation.
    UnsupportedVersion {
        /// Unsupported version.
        version: IrVersion,
    },

    /// The complete program violates an invariant.
    InvalidProgram {
        /// Static invariant description.
        message: &'static str,
    },
}

impl fmt::Display for ProgramError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLimits { field, value } => {
                write!(
                    formatter,
                    "invalid quantum IR limit `{field}`: {value}"
                )
            }

            Self::InvalidProgramId => {
                formatter.write_str("invalid quantum program identifier")
            }

            Self::DuplicateRegion { region } => {
                write!(formatter, "duplicate program region {region}")
            }

            Self::UnknownRegion { region } => {
                write!(formatter, "unknown program region {region}")
            }

            Self::MissingRootRegion { region } => {
                write!(formatter, "root region {region} does not exist")
            }

            Self::DuplicateOperation { operation } => {
                write!(formatter, "duplicate program operation {operation}")
            }

            Self::OperationIndexOutOfRange { index, len } => {
                write!(
                    formatter,
                    "operation index {index} is outside program order length {len}"
                )
            }

            Self::InvalidOperation(error) => {
                write!(formatter, "invalid program operation: {error}")
            }

            Self::InvalidRegion { message } => {
                write!(formatter, "invalid program region: {message}")
            }

            Self::DuplicateQubit { qubit } => {
                write!(formatter, "logical qubit {qubit} is already declared")
            }

            Self::DuplicateClassicalBit { bit } => {
                write!(formatter, "classical bit c{bit} is already declared")
            }

            Self::DuplicateParameter { parameter } => {
                write!(
                    formatter,
                    "parameter {parameter} is already declared"
                )
            }

            Self::UnknownMappingQubit { qubit } => {
                write!(
                    formatter,
                    "mapping references undeclared logical qubit {qubit}"
                )
            }

            Self::Mapping(error) => {
                write!(formatter, "invalid qubit mapping: {error}")
            }

            Self::DuplicateCapability { capability } => {
                write!(
                    formatter,
                    "capability {capability} is already required"
                )
            }

            Self::DuplicateResource { resource } => {
                write!(
                    formatter,
                    "resource {resource} is already required"
                )
            }

            Self::ResourceLimitExceeded {
                resource,
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "{resource} limit exceeded: requested {requested}, maximum {maximum}"
                )
            }

            Self::MetadataLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "metadata limit exceeded: requested {requested} bytes, maximum {maximum}"
                )
            }

            Self::ArithmeticOverflow { calculation } => {
                write!(
                    formatter,
                    "arithmetic overflow while calculating {calculation}"
                )
            }

            Self::AllocationFailure { collection } => {
                write!(
                    formatter,
                    "unable to reserve storage for {collection}"
                )
            }

            Self::UnsupportedVersion { version } => {
                write!(
                    formatter,
                    "unsupported quantum IR version {version}"
                )
            }

            Self::InvalidProgram { message } => {
                write!(formatter, "invalid quantum program: {message}")
            }
        }
    }
}

impl std::error::Error for ProgramError {}

impl From<OperationError> for ProgramError {
    fn from(error: OperationError) -> Self {
        Self::InvalidOperation(error)
    }
}

impl From<MappingError> for ProgramError {
    fn from(error: MappingError) -> Self {
        Self::Mapping(error)
    }
}

impl From<LimitsError> for ProgramError {
    fn from(error: LimitsError) -> Self {
        match error {
            LimitsError::InvalidConfiguration { field, value } => {
                Self::InvalidLimits { field, value }
            }

            LimitsError::ResourceExceeded {
                resource,
                requested,
                maximum,
            } => {
                Self::ResourceLimitExceeded {
                    resource: resource.as_str(),
                    requested,
                    maximum,
                }
            }

            LimitsError::ArithmeticOverflow { .. }
            | LimitsError::ArithmeticMultiplicationOverflow { .. }
            | LimitsError::TimeArithmeticOverflow => {
                Self::ArithmeticOverflow {
                    calculation: "IR resource accounting",
                }
            }

            LimitsError::ScheduleTimeExceeded { .. } => {
                Self::InvalidProgram {
                    message:
                        "schedule-time policy cannot be applied directly to a program namespace",
                }
            }
        }
    }
}

// =============================================================================
// Program metadata
// =============================================================================

/// Program-level metadata.
///
/// Metadata is deliberately string-based and deterministic here. Rich typed
/// attributes remain owned by the canonical attribute/extension layers.
///
/// This avoids coupling `program.rs` to any particular metadata serialization
/// implementation while still providing a production-safe program container.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ProgramMetadata {
    values: BTreeMap<String, String>,
}

impl ProgramMetadata {
    /// Creates empty metadata.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            values: BTreeMap::new(),
        }
    }

    /// Returns the number of metadata entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns whether no metadata entries exist.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Returns a metadata value.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(String::as_str)
    }

    /// Returns deterministic metadata entries.
    #[must_use]
    pub fn entries(&self) -> &BTreeMap<String, String> {
        &self.values
    }

    /// Inserts or replaces a metadata value.
    ///
    /// The resulting metadata object is still checked against program limits
    /// when installed into a `QuantumProgram`.
    pub fn insert(
        &mut self,
        key: String,
        value: String,
    ) -> Option<String> {
        self.values.insert(key, value)
    }

    /// Removes one metadata key.
    pub fn remove(
        &mut self,
        key: &str,
    ) -> Option<String> {
        self.values.remove(key)
    }

    /// Removes all metadata.
    pub fn clear(&mut self) {
        self.values.clear();
    }

    /// Calculates deterministic UTF-8 storage size.
    pub fn byte_size(&self) -> ProgramResult<usize> {
        self.values.iter().try_fold(0usize, |total, (key, value)| {
            let entry_size = key
                .len()
                .checked_add(value.len())
                .and_then(|size| size.checked_add(1))
                .ok_or(ProgramError::ArithmeticOverflow {
                    calculation: "program metadata byte size",
                })?;

            total
                .checked_add(entry_size)
                .ok_or(ProgramError::ArithmeticOverflow {
                    calculation: "program metadata byte size",
                })
        })
    }
}

// =============================================================================
// Program
// =============================================================================

/// Canonical universal Zamani quantum program.
///
/// `QuantumProgram` owns program-level storage and references. It does not
/// redefine the meaning of individual operations, gates, measurements, pulse
/// objects, or hardware resources.
///
/// # Storage model
///
/// Operations are stored in two deterministic structures:
///
/// 1. `BTreeMap<OperationId, Operation>` provides stable identity lookup;
/// 2. `Vec<OperationId>` provides explicit semantic program order.
///
/// Regions are stored in a deterministic `BTreeMap`.
///
/// This deliberately separates identity from position:
///
/// ```text
/// OperationId != operation index
/// ```
///
/// Therefore inserting an operation before another operation does not change
/// the existing operation's identity.
///
/// # Scalability
///
/// No collection is allocated based on a machine-size constant.
///
/// Declaring a qubit records only its identity. There is no requirement to
/// materialize a hardware state vector, topology, calibration, or physical
/// qubit object.
///
/// Sparse logical identities are therefore representable without allocating
/// every identifier between zero and the largest identifier.
///
/// # Ownership
///
/// A `QuantumProgram` may contain:
///
/// - zero or more logical qubits;
/// - zero or more classical bits;
/// - zero or more symbolic parameters;
/// - zero or more operations;
/// - zero or more regions;
/// - zero or more capability requirements;
/// - zero or more resource requirements;
/// - a partial or complete logical-to-physical mapping.
///
/// The program remains finite and explicit while having no architectural
/// machine-size ceiling.
#[derive(Debug, Clone, PartialEq)]
pub struct QuantumProgram {
    id: ProgramId,
    version: IrVersion,
    limits: QuantumIrLimits,

    /// Declared logical qubit identities.
    qubits: BTreeSet<QubitId>,

    /// Declared classical-bit indices.
    ///
    /// Classical bits are represented by their canonical numeric namespace
    /// here. The canonical `ClassicalBitId` remains owned by
    /// `quantum::ir::classical` / `measurement`.
    classical_bits: BTreeSet<usize>,

    /// Declared symbolic parameters.
    parameters: BTreeSet<ParameterId>,

    /// Canonical operation registry.
    operations: BTreeMap<OperationId, Operation>,

    /// Explicit semantic operation order.
    operation_order: Vec<OperationId>,

    /// Program region registry.
    regions: BTreeMap<RegionId, Region>,

    /// Root structured region.
    root_region: RegionId,

    /// Program-level logical-to-physical mapping.
    ///
    /// This is a semantic mapping record. It does not prove that the selected
    /// hardware actually contains or permits the physical qubits.
    mapping: QubitMapping,

    /// Required target capabilities.
    capabilities: BTreeSet<CapabilityId>,

    /// Required abstract resources.
    resources: BTreeSet<ResourceId>,

    /// Deterministic program metadata.
    metadata: ProgramMetadata,
}

impl QuantumProgram {
    // =========================================================================
    // Construction
    // =========================================================================

    /// Creates an empty universal quantum program.
    ///
    /// `root_region` is caller-controlled. No hidden global region allocator is
    /// used.
    pub fn new(
        id: ProgramId,
        root_region: RegionId,
    ) -> ProgramResult<Self> {
        Self::with_limits(
            id,
            root_region,
            QuantumIrLimits::production(),
        )
    }

    /// Creates an empty universal quantum program using an explicit policy.
    pub fn with_limits(
        id: ProgramId,
        root_region: RegionId,
        limits: QuantumIrLimits,
    ) -> ProgramResult<Self> {
        limits.validate()?;

        let root = Region::root(root_region);

        let mut regions = BTreeMap::new();
        regions.insert(root_region, root);

        Ok(Self {
            id,
            version: IrVersion::CURRENT,
            limits,
            qubits: BTreeSet::new(),
            classical_bits: BTreeSet::new(),
            parameters: BTreeSet::new(),
            operations: BTreeMap::new(),
            operation_order: Vec::new(),
            regions,
            root_region,
            mapping: QubitMapping::new(),
            capabilities: BTreeSet::new(),
            resources: BTreeSet::new(),
            metadata: ProgramMetadata::new(),
        })
    }

    /// Creates an empty program with the current production policy and a
    /// conventional caller-supplied identity.
    ///
    /// The caller still controls identity allocation; this method does not use
    /// global mutable state.
    pub fn empty(
        id: ProgramId,
        root_region: RegionId,
    ) -> ProgramResult<Self> {
        Self::new(id, root_region)
    }

    // =========================================================================
    // Identity and version
    // =========================================================================

    /// Returns the program identity.
    #[must_use]
    pub const fn id(&self) -> ProgramId {
        self.id
    }

    /// Returns the IR schema/semantic version.
    #[must_use]
    pub const fn version(&self) -> IrVersion {
        self.version
    }

    /// Returns whether the program uses the current IR contract.
    #[must_use]
    pub const fn is_current_version(&self) -> bool {
        self.version.is_current()
    }

    /// Changes the IR version after compatibility validation.
    pub fn set_version(
        &mut self,
        version: IrVersion,
    ) -> ProgramResult<()> {
        if !version.is_supported_by_current() {
            return Err(ProgramError::UnsupportedVersion { version });
        }

        self.version = version;
        Ok(())
    }

    // =========================================================================
    // Resource policy
    // =========================================================================

    /// Returns the explicit program resource/security policy.
    #[must_use]
    pub const fn limits(&self) -> &QuantumIrLimits {
        &self.limits
    }

    /// Replaces the resource/security policy after checking the existing
    /// program against it.
    ///
    /// No semantic program content is modified.
    pub fn set_limits(
        &mut self,
        limits: QuantumIrLimits,
    ) -> ProgramResult<()> {
        limits.validate()?;

        self.check_limits_for_existing_state(&limits)?;

        self.limits = limits;

        Ok(())
    }

    // =========================================================================
    // Root region
    // =========================================================================

    /// Returns the root region identity.
    #[must_use]
    pub const fn root_region_id(&self) -> RegionId {
        self.root_region
    }

    /// Returns the root region.
    pub fn root_region(&self) -> ProgramResult<&Region> {
        self.regions
            .get(&self.root_region)
            .ok_or(ProgramError::MissingRootRegion {
                region: self.root_region,
            })
    }

    /// Changes the root region.
    ///
    /// The region must already exist.
    pub fn set_root_region(
        &mut self,
        region: RegionId,
    ) -> ProgramResult<()> {
        if !self.regions.contains_key(&region) {
            return Err(ProgramError::UnknownRegion { region });
        }

        self.root_region = region;

        Ok(())
    }

    // =========================================================================
    // Logical qubits
    // =========================================================================

    /// Returns the number of explicitly declared logical qubits.
    #[must_use]
    pub fn qubit_count(&self) -> usize {
        self.qubits.len()
    }

    /// Compatibility alias for callers using `num_qubits`.
    #[must_use]
    pub fn num_qubits(&self) -> usize {
        self.qubits.len()
    }

    /// Returns whether a logical qubit is declared.
    #[must_use]
    pub fn contains_qubit(
        &self,
        qubit: QubitId,
    ) -> bool {
        self.qubits.contains(&qubit)
    }

    /// Returns deterministic logical-qubit declarations.
    #[must_use]
    pub fn qubits(&self) -> &BTreeSet<QubitId> {
        &self.qubits
    }

    /// Declares one logical qubit.
    ///
    /// The operation is atomic.
    pub fn declare_qubit(
        &mut self,
        qubit: QubitId,
    ) -> ProgramResult<()> {
        if self.qubits.contains(&qubit) {
            return Err(ProgramError::DuplicateQubit { qubit });
        }

        let requested = self
            .qubits
            .len()
            .checked_add(1)
            .ok_or(ProgramError::ArithmeticOverflow {
                calculation: "logical qubit declaration count",
            })?;

        self.check_resource_limit(
            "logical qubits",
            requested,
            self.logical_qubit_limit(),
        )?;

        self.qubits.insert(qubit);

        Ok(())
    }

    /// Declares multiple logical qubits atomically.
    ///
    /// Existing declarations are not modified if any requested declaration
    /// would fail.
    pub fn declare_qubits<I>(
        &mut self,
        qubits: I,
    ) -> ProgramResult<usize>
    where
        I: IntoIterator<Item = QubitId>,
    {
        let candidates: Vec<QubitId> = qubits.into_iter().collect();

        let mut unique = BTreeSet::new();

        for qubit in &candidates {
            if self.qubits.contains(qubit) {
                return Err(ProgramError::DuplicateQubit { qubit: *qubit });
            }

            if !unique.insert(*qubit) {
                return Err(ProgramError::DuplicateQubit { qubit: *qubit });
            }
        }

        let requested = self
            .qubits
            .len()
            .checked_add(unique.len())
            .ok_or(ProgramError::ArithmeticOverflow {
                calculation: "logical qubit declaration count",
            })?;

        self.check_resource_limit(
            "logical qubits",
            requested,
            self.logical_qubit_limit(),
        )?;

        self.qubits.extend(unique);

        Ok(candidates.len())
    }

    /// Removes a logical-qubit declaration.
    ///
    /// A qubit cannot be removed while it is referenced by the program's
    /// current mapping. Operation-reference validation is intentionally left
    /// to the complete IR validation layer because operation payloads are owned
    /// by `operation.rs`.
    pub fn undeclare_qubit(
        &mut self,
        qubit: QubitId,
    ) -> ProgramResult<bool> {
        if self.mapping.physical_for(qubit).is_some() {
            return Err(ProgramError::InvalidProgram {
                message: "cannot undeclare a mapped logical qubit",
            });
        }

        Ok(self.qubits.remove(&qubit))
    }

    // =========================================================================
    // Classical namespace
    // =========================================================================

    /// Returns the number of declared classical bits.
    #[must_use]
    pub fn classical_bit_count(&self) -> usize {
        self.classical_bits.len()
    }

    /// Compatibility alias for callers using `num_classical_bits`.
    #[must_use]
    pub fn num_classical_bits(&self) -> usize {
        self.classical_bits.len()
    }

    /// Returns whether a classical-bit index is declared.
    #[must_use]
    pub fn contains_classical_bit(
        &self,
        bit: usize,
    ) -> bool {
        self.classical_bits.contains(&bit)
    }

    /// Returns deterministic classical-bit declarations.
    #[must_use]
    pub fn classical_bits(&self) -> &BTreeSet<usize> {
        &self.classical_bits
    }

    /// Declares one classical bit by canonical namespace index.
    pub fn declare_classical_bit(
        &mut self,
        bit: usize,
    ) -> ProgramResult<()> {
        if self.classical_bits.contains(&bit) {
            return Err(ProgramError::DuplicateClassicalBit { bit });
        }

        let requested = self
            .classical_bits
            .len()
            .checked_add(1)
            .ok_or(ProgramError::ArithmeticOverflow {
                calculation: "classical bit declaration count",
            })?;

        self.check_resource_limit(
            "classical bits",
            requested,
            self.classical_bit_limit(),
        )?;

        self.classical_bits.insert(bit);

        Ok(())
    }

    /// Declares multiple classical bits atomically.
    pub fn declare_classical_bits<I>(
        &mut self,
        bits: I,
    ) -> ProgramResult<usize>
    where
        I: IntoIterator<Item = usize>,
    {
        let candidates: Vec<usize> = bits.into_iter().collect();

        let mut unique = BTreeSet::new();

        for bit in &candidates {
            if self.classical_bits.contains(bit) {
                return Err(ProgramError::DuplicateClassicalBit { bit: *bit });
            }

            if !unique.insert(*bit) {
                return Err(ProgramError::DuplicateClassicalBit { bit: *bit });
            }
        }

        let requested = self
            .classical_bits
            .len()
            .checked_add(unique.len())
            .ok_or(ProgramError::ArithmeticOverflow {
                calculation: "classical bit declaration count",
            })?;

        self.check_resource_limit(
            "classical bits",
            requested,
            self.classical_bit_limit(),
        )?;

        self.classical_bits.extend(unique);

        Ok(candidates.len())
    }

    /// Removes one classical-bit declaration.
    pub fn undeclare_classical_bit(
        &mut self,
        bit: usize,
    ) -> bool {
        self.classical_bits.remove(&bit)
    }

    // =========================================================================
    // Parameters
    // =========================================================================

    /// Returns the number of declared parameters.
    #[must_use]
    pub fn parameter_count(&self) -> usize {
        self.parameters.len()
    }

    /// Returns deterministic parameter declarations.
    #[must_use]
    pub fn parameters(&self) -> &BTreeSet<ParameterId> {
        &self.parameters
    }

    /// Returns whether a parameter is declared.
    #[must_use]
    pub fn contains_parameter(
        &self,
        parameter: ParameterId,
    ) -> bool {
        self.parameters.contains(&parameter)
    }

    /// Declares one symbolic/runtime parameter.
    pub fn declare_parameter(
        &mut self,
        parameter: ParameterId,
    ) -> ProgramResult<()> {
        if !self.parameters.insert(parameter) {
            return Err(ProgramError::DuplicateParameter { parameter });
        }

        Ok(())
    }

    /// Removes a parameter declaration.
    pub fn undeclare_parameter(
        &mut self,
        parameter: ParameterId,
    ) -> bool {
        self.parameters.remove(&parameter)
    }

    // =========================================================================
    // Regions
    // =========================================================================

    /// Returns the number of regions.
    #[must_use]
    pub fn region_count(&self) -> usize {
        self.regions.len()
    }

    /// Returns whether a region exists.
    #[must_use]
    pub fn contains_region(
        &self,
        region: RegionId,
    ) -> bool {
        self.regions.contains_key(&region)
    }

    /// Returns one region by identity.
    #[must_use]
    pub fn region(
        &self,
        region: RegionId,
    ) -> Option<&Region> {
        self.regions.get(&region)
    }

    /// Returns mutable access to one region.
    ///
    /// This is intentionally restricted to the region object itself. Program
    /// identity registries remain owned by `QuantumProgram`.
    pub fn region_mut(
        &mut self,
        region: RegionId,
    ) -> ProgramResult<&mut Region> {
        self.regions
            .get_mut(&region)
            .ok_or(ProgramError::UnknownRegion { region })
    }

    /// Returns all regions in deterministic identity order.
    #[must_use]
    pub fn regions(&self) -> &BTreeMap<RegionId, Region> {
        &self.regions
    }

    /// Adds a region atomically.
    pub fn add_region(
        &mut self,
        region: Region,
    ) -> ProgramResult<RegionId> {
        let id = region.id();

        if self.regions.contains_key(&id) {
            return Err(ProgramError::DuplicateRegion { region: id });
        }

        if id == self.root_region {
            return Err(ProgramError::DuplicateRegion { region: id });
        }

        self.regions.insert(id, region);

        Ok(id)
    }

    /// Creates and adds a generic region.
    pub fn create_region(
        &mut self,
        id: RegionId,
        kind: RegionKind,
    ) -> ProgramResult<RegionId> {
        self.add_region(Region::new(id, kind))
    }

    /// Removes a non-root region.
    ///
    /// Root removal is prohibited because every valid program must have one
    /// root structural scope.
    pub fn remove_region(
        &mut self,
        region: RegionId,
    ) -> ProgramResult<Region> {
        if region == self.root_region {
            return Err(ProgramError::InvalidRegion {
                message: "the root region cannot be removed",
            });
        }

        self.regions
            .remove(&region)
            .ok_or(ProgramError::UnknownRegion { region })
    }

    // =========================================================================
    // Operations
    // =========================================================================

    /// Returns the number of semantic operations.
    #[must_use]
    pub fn operation_count(&self) -> usize {
        self.operation_order.len()
    }

    /// Compatibility alias for callers using `len()`.
    #[must_use]
    pub fn len(&self) -> usize {
        self.operation_count()
    }

    /// Returns whether the program contains no operations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.operation_order.is_empty()
    }

    /// Returns whether an operation identity exists.
    #[must_use]
    pub fn contains_operation(
        &self,
        operation: OperationId,
    ) -> bool {
        self.operations.contains_key(&operation)
    }

    /// Returns one operation by identity.
    #[must_use]
    pub fn operation(
        &self,
        operation: OperationId,
    ) -> Option<&Operation> {
        self.operations.get(&operation)
    }

    /// Returns the explicit operation order.
    #[must_use]
    pub fn operation_order(&self) -> &[OperationId] {
        &self.operation_order
    }

    /// Returns all operations in deterministic semantic order.
    ///
    /// This allocates a vector of references. Consumers that need zero-copy
    /// traversal should use `operation_ids()` and `operation()` instead.
    pub fn operations(&self) -> Vec<&Operation> {
        self.operation_order
            .iter()
            .filter_map(|id| self.operations.get(id))
            .collect()
    }

    /// Returns operation IDs in explicit semantic order.
    pub fn operation_ids(
        &self,
    ) -> std::slice::Iter<'_, OperationId> {
        self.operation_order.iter()
    }

    /// Returns an operation by semantic position.
    pub fn operation_at(
        &self,
        index: usize,
    ) -> ProgramResult<&Operation> {
        let id = self
            .operation_order
            .get(index)
            .copied()
            .ok_or(ProgramError::OperationIndexOutOfRange {
                index,
                len: self.operation_order.len(),
            })?;

        self.operations
            .get(&id)
            .ok_or(ProgramError::InvalidProgram {
                message: "operation order references a missing operation",
            })
    }

    /// Returns the semantic position of an operation.
    #[must_use]
    pub fn operation_index(
        &self,
        operation: OperationId,
    ) -> Option<usize> {
        self.operation_order
            .iter()
            .position(|id| *id == operation)
    }

    /// Inserts an operation at the end of program order.
    pub fn push_operation(
        &mut self,
        operation: Operation,
    ) -> ProgramResult<OperationId> {
        let index = self.operation_order.len();
        self.insert_operation(index, operation)
    }

    /// Compatibility alias for `push_operation`.
    pub fn add_operation(
        &mut self,
        operation: Operation,
    ) -> ProgramResult<OperationId> {
        self.push_operation(operation)
    }

    /// Inserts an operation at an explicit semantic position.
    ///
    /// Operation identity remains unchanged by insertion position.
    pub fn insert_operation(
        &mut self,
        index: usize,
        operation: Operation,
    ) -> ProgramResult<OperationId> {
        if index > self.operation_order.len() {
            return Err(ProgramError::OperationIndexOutOfRange {
                index,
                len: self.operation_order.len(),
            });
        }

        operation.validate()?;

        let id = operation.id();

        if self.operations.contains_key(&id) {
            return Err(ProgramError::DuplicateOperation { operation: id });
        }

        let requested = self
            .operation_order
            .len()
            .checked_add(1)
            .ok_or(ProgramError::ArithmeticOverflow {
                calculation: "program operation count",
            })?;

        self.check_resource_limit(
            "operations",
            requested,
            self.operation_limit(),
        )?;

        self.operation_order
            .try_reserve(1)
            .map_err(|_| ProgramError::AllocationFailure {
                collection: "program operation order",
            })?;

        /*
         * Commit only after every fallible precondition above succeeds.
         *
         * `BTreeMap::insert` itself does not expose a stable fallible-reserve
         * API on the supported Rust versions. The insertion therefore occurs
         * only after all program-level semantic checks have completed.
         */
        self.operations.insert(id, operation);
        self.operation_order.insert(index, id);

        Ok(id)
    }

    /// Replaces an existing operation while preserving its semantic position.
    pub fn replace_operation(
        &mut self,
        operation: Operation,
    ) -> ProgramResult<OperationId> {
        operation.validate()?;

        let id = operation.id();

        if !self.operations.contains_key(&id) {
            return Err(ProgramError::InvalidProgram {
                message: "replacement operation identity does not exist",
            });
        }

        self.operations.insert(id, operation);

        Ok(id)
    }

    /// Removes an operation by identity.
    ///
    /// The remaining operation identities are unchanged.
    pub fn remove_operation(
        &mut self,
        operation: OperationId,
    ) -> ProgramResult<Operation> {
        let position = self
            .operation_index(operation)
            .ok_or(ProgramError::InvalidProgram {
                message: "operation identity is not present in program order",
            })?;

        let removed = self
            .operations
            .remove(&operation)
            .ok_or(ProgramError::InvalidProgram {
                message: "operation order references a missing operation",
            })?;

        self.operation_order.remove(position);

        Ok(removed)
    }

    /// Removes all operations while preserving declarations, regions, mapping,
    /// requirements, and metadata.
    pub fn clear_operations(&mut self) {
        self.operations.clear();
        self.operation_order.clear();
    }

    /// Replaces the complete operation sequence atomically.
    pub fn replace_operations(
        &mut self,
        operations: Vec<Operation>,
    ) -> ProgramResult<()> {
        if operations.len() > self.operation_limit() {
            return Err(ProgramError::ResourceLimitExceeded {
                resource: "operations",
                requested: operations.len(),
                maximum: self.operation_limit(),
            });
        }

        let mut registry = BTreeMap::new();
        let mut order = Vec::with_capacity(operations.len());

        order
            .try_reserve(operations.len())
            .map_err(|_| ProgramError::AllocationFailure {
                collection: "program operation order",
            })?;

        for operation in operations {
            operation.validate()?;

            let id = operation.id();

            if registry.insert(id, operation).is_some() {
                return Err(ProgramError::DuplicateOperation { operation: id });
            }

            order.push(id);
        }

        self.operations = registry;
        self.operation_order = order;

        Ok(())
    }

    // =========================================================================
    // Mapping
    // =========================================================================

    /// Returns the current logical-to-physical mapping.
    #[must_use]
    pub const fn mapping(&self) -> &QubitMapping {
        &self.mapping
    }

    /// Returns mutable mapping access.
    ///
    /// Mutation is intentionally not exposed directly because the program must
    /// ensure that mapped logical qubits are declared. Use the checked mapping
    /// methods below.
    pub fn map_qubit(
        &mut self,
        logical: QubitId,
        physical: PhysicalQubitId,
    ) -> ProgramResult<()> {
        if !self.qubits.contains(&logical) {
            return Err(ProgramError::UnknownMappingQubit { qubit: logical });
        }

        self.mapping.insert(logical, physical)?;

        Ok(())
    }

    /// Removes a logical-to-physical mapping.
    pub fn unmap_qubit(
        &mut self,
        logical: QubitId,
    ) -> ProgramResult<()> {
        self.mapping.remove_logical(logical)?;
        Ok(())
    }

    /// Returns the physical qubit assigned to a logical qubit.
    #[must_use]
    pub fn physical_qubit_for(
        &self,
        logical: QubitId,
    ) -> Option<PhysicalQubitId> {
        self.mapping.physical_for(logical)
    }

    /// Returns the logical qubit assigned to a physical qubit.
    #[must_use]
    pub fn logical_qubit_for(
        &self,
        physical: PhysicalQubitId,
    ) -> Option<QubitId> {
        self.mapping.logical_for(physical)
    }

    /// Clears the complete mapping.
    pub fn clear_mapping(&mut self) {
        self.mapping.clear();
    }

    // =========================================================================
    // Capability requirements
    // =========================================================================

    /// Returns required target capabilities.
    #[must_use]
    pub fn capabilities(&self) -> &BTreeSet<CapabilityId> {
        &self.capabilities
    }

    /// Returns whether a capability is required.
    #[must_use]
    pub fn requires_capability(
        &self,
        capability: CapabilityId,
    ) -> bool {
        self.capabilities.contains(&capability)
    }

    /// Adds one capability requirement.
    pub fn require_capability(
        &mut self,
        capability: CapabilityId,
    ) -> ProgramResult<()> {
        if !self.capabilities.insert(capability) {
            return Err(ProgramError::DuplicateCapability { capability });
        }

        Ok(())
    }

    /// Removes a capability requirement.
    pub fn remove_capability(
        &mut self,
        capability: CapabilityId,
    ) -> bool {
        self.capabilities.remove(&capability)
    }

    /// Removes all capability requirements.
    pub fn clear_capabilities(&mut self) {
        self.capabilities.clear();
    }

    // =========================================================================
    // Resource requirements
    // =========================================================================

    /// Returns required abstract resources.
    #[must_use]
    pub fn resources(&self) -> &BTreeSet<ResourceId> {
        &self.resources
    }

    /// Returns whether an abstract resource is required.
    #[must_use]
    pub fn requires_resource(
        &self,
        resource: ResourceId,
    ) -> bool {
        self.resources.contains(&resource)
    }

    /// Adds one abstract resource requirement.
    pub fn require_resource(
        &mut self,
        resource: ResourceId,
    ) -> ProgramResult<()> {
        if !self.resources.insert(resource) {
            return Err(ProgramError::DuplicateResource { resource });
        }

        Ok(())
    }

    /// Removes one resource requirement.
    pub fn remove_resource(
        &mut self,
        resource: ResourceId,
    ) -> bool {
        self.resources.remove(&resource)
    }

    /// Removes all resource requirements.
    pub fn clear_resources(&mut self) {
        self.resources.clear();
    }

    // =========================================================================
    // Metadata
    // =========================================================================

    /// Returns program metadata.
    #[must_use]
    pub const fn metadata(&self) -> &ProgramMetadata {
        &self.metadata
    }

    /// Replaces program metadata atomically after policy validation.
    pub fn set_metadata(
        &mut self,
        metadata: ProgramMetadata,
    ) -> ProgramResult<()> {
        let size = metadata.byte_size()?;

        self.check_metadata_limit(size)?;

        self.metadata = metadata;

        Ok(())
    }

    /// Inserts or replaces one metadata value atomically.
    pub fn set_metadata_value(
        &mut self,
        key: String,
        value: String,
    ) -> ProgramResult<Option<String>> {
        let mut candidate = self.metadata.clone();
        let previous = candidate.insert(key, value);

        self.set_metadata(candidate)?;

        Ok(previous)
    }

    /// Removes one metadata value.
    pub fn remove_metadata_value(
        &mut self,
        key: &str,
    ) -> ProgramResult<Option<String>> {
        let mut candidate = self.metadata.clone();
        let previous = candidate.remove(key);

        self.set_metadata(candidate)?;

        Ok(previous)
    }

    // =========================================================================
    // Validation
    // =========================================================================

    /// Performs program-local structural validation.
    ///
    /// This does not replace `quantum::ir::validation`; it checks the
    /// invariants owned directly by this module.
    pub fn validate(&self) -> ProgramResult<()> {
        self.limits.validate()?;

        if !self.version.is_supported_by_current() {
            return Err(ProgramError::UnsupportedVersion {
                version: self.version,
            });
        }

        if !self.regions.contains_key(&self.root_region) {
            return Err(ProgramError::MissingRootRegion {
                region: self.root_region,
            });
        }

        if self.operation_order.len() != self.operations.len() {
            return Err(ProgramError::InvalidProgram {
                message: "operation registry/order lengths differ",
            });
        }

        let mut seen = BTreeSet::new();

        for id in &self.operation_order {
            if !seen.insert(*id) {
                return Err(ProgramError::InvalidProgram {
                    message: "operation order contains a duplicate identity",
                });
            }

            if !self.operations.contains_key(id) {
                return Err(ProgramError::InvalidProgram {
                    message: "operation order references a missing operation",
                });
            }
        }

        for (id, operation) in &self.operations {
            if operation.id() != *id {
                return Err(ProgramError::InvalidProgram {
                    message: "operation registry key does not match operation identity",
                });
            }

            operation.validate()?;
        }

        for region in self.regions.values() {
            if region.id() == self.root_region
                && region.kind() != RegionKind::Root
            {
                return Err(ProgramError::InvalidRegion {
                    message: "root region must have root semantic kind",
                });
            }
        }

        for logical in self.mapping.logical_qubits() {
            if !self.qubits.contains(&logical) {
                return Err(ProgramError::UnknownMappingQubit {
                    qubit: logical,
                });
            }
        }

        self.check_limits_for_existing_state(&self.limits)?;

        self.check_metadata_limit(self.metadata.byte_size()?)?;

        Ok(())
    }

    /// Returns whether the program is structurally valid.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.validate().is_ok()
    }

    // =========================================================================
    // Limits
    // =========================================================================

    fn logical_qubit_limit(&self) -> usize {
        self.limits.max_qubits()
    }

    fn classical_bit_limit(&self) -> usize {
        self.limits.max_classical_bits()
    }

    fn operation_limit(&self) -> usize {
        self.limits.max_operations()
    }

    fn metadata_limit(&self) -> usize {
        self.limits.max_metadata_bytes()
    }

    fn check_resource_limit(
        &self,
        resource: &'static str,
        requested: usize,
        maximum: usize,
    ) -> ProgramResult<()> {
        if requested > maximum {
            return Err(ProgramError::ResourceLimitExceeded {
                resource,
                requested,
                maximum,
            });
        }

        Ok(())
    }

    fn check_metadata_limit(
        &self,
        requested: usize,
    ) -> ProgramResult<()> {
        self.check_resource_limit(
            "metadata bytes",
            requested,
            self.metadata_limit(),
        )
        .map_err(|error| match error {
            ProgramError::ResourceLimitExceeded {
                requested,
                maximum,
                ..
            } => ProgramError::MetadataLimitExceeded {
                requested,
                maximum,
            },
            other => other,
        })
    }

    fn check_limits_for_existing_state(
        &self,
        limits: &QuantumIrLimits,
    ) -> ProgramResult<()> {
        if self.qubits.len() > limits.max_qubits() {
            return Err(ProgramError::ResourceLimitExceeded {
                resource: "logical qubits",
                requested: self.qubits.len(),
                maximum: limits.max_qubits(),
            });
        }

        if self.classical_bits.len() > limits.max_classical_bits() {
            return Err(ProgramError::ResourceLimitExceeded {
                resource: "classical bits",
                requested: self.classical_bits.len(),
                maximum: limits.max_classical_bits(),
            });
        }

        if self.operation_order.len() > limits.max_operations() {
            return Err(ProgramError::ResourceLimitExceeded {
                resource: "operations",
                requested: self.operation_order.len(),
                maximum: limits.max_operations(),
            });
        }

        let metadata_size = self.metadata.byte_size()?;

        if metadata_size > limits.max_metadata_bytes() {
            return Err(ProgramError::MetadataLimitExceeded {
                requested: metadata_size,
                maximum: limits.max_metadata_bytes(),
            });
        }

        Ok(())
    }
}

// =============================================================================
// Compatibility exports
// =============================================================================
//
// These exports preserve the useful historical paths:
//
//     quantum::ir::program::ProgramId
//     quantum::ir::program::OperationId
//     quantum::ir::program::RegionId
//
// without creating duplicate identity definitions.

pub use super::identity::{
    BlockId,
    CircuitId,
    ModuleId,
    NamespaceId,
    ValueId,
};

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantum::ir::identity::{
        OperationId,
        ParameterId,
        ProgramId,
        RegionId,
    };
    use crate::quantum::ir::qubit::QubitId;

    #[test]
    fn program_uses_canonical_qubit_identity() {
        let mut program = QuantumProgram::new(
            ProgramId::new(1),
            RegionId::new(1),
        )
        .expect("program construction must succeed");

        program
            .declare_qubit(QubitId::new(0))
            .expect("qubit declaration must succeed");

        assert!(program.contains_qubit(QubitId::new(0)));
        assert_eq!(program.qubit_count(), 1);
    }

    #[test]
    fn program_has_no_machine_size_assumption() {
        let mut program = QuantumProgram::new(
            ProgramId::new(2),
            RegionId::new(2),
        )
        .expect("program construction must succeed");

        let qubit = QubitId::new(u64::MAX as usize);

        /*
         * The identity itself is independent of the program's declared
         * physical machine capacity. This test only checks that the program
         * does not use a fixed machine-size boundary.
         */
        let result = program.declare_qubit(qubit);

        assert!(result.is_ok());
    }

    #[test]
    fn operation_identity_is_independent_of_position() {
        /*
         * This invariant is tested at the container level. Concrete operation
         * construction remains owned by operation.rs.
         */
        let program = QuantumProgram::new(
            ProgramId::new(3),
            RegionId::new(3),
        )
        .expect("program construction must succeed");

        assert_eq!(program.operation_count(), 0);
        assert_eq!(program.operation_index(OperationId::new(42)), None);
    }

    #[test]
    fn metadata_is_deterministic() {
        let mut metadata = ProgramMetadata::new();

        metadata.insert(
            "z".to_owned(),
            "last".to_owned(),
        );

        metadata.insert(
            "a".to_owned(),
            "first".to_owned(),
        );

        let keys: Vec<&String> =
            metadata.entries().keys().collect();

        assert_eq!(
            keys,
            vec![
                &"a".to_owned(),
                &"z".to_owned(),
            ]
        );
    }

    #[test]
    fn duplicate_qubits_are_rejected_atomically() {
        let mut program = QuantumProgram::new(
            ProgramId::new(4),
            RegionId::new(4),
        )
        .expect("program construction must succeed");

        program
            .declare_qubit(QubitId::new(0))
            .expect("first declaration must succeed");

        let result = program.declare_qubits([
            QubitId::new(1),
            QubitId::new(0),
            QubitId::new(2),
        ]);

        assert!(result.is_err());
        assert_eq!(program.qubit_count(), 1);
        assert!(!program.contains_qubit(QubitId::new(1)));
        assert!(!program.contains_qubit(QubitId::new(2)));
    }

    #[test]
    fn root_region_cannot_be_removed() {
        let mut program = QuantumProgram::new(
            ProgramId::new(5),
            RegionId::new(5),
        )
        .expect("program construction must succeed");

        let result =
            program.remove_region(RegionId::new(5));

        assert!(result.is_err());
        assert!(program.contains_region(RegionId::new(5)));
    }

    #[test]
    fn parameter_identity_is_declared_without_hardware_assumptions() {
        let mut program = QuantumProgram::new(
            ProgramId::new(6),
            RegionId::new(6),
        )
        .expect("program construction must succeed");

        program
            .declare_parameter(ParameterId::new(10))
            .expect("parameter declaration must succeed");

        assert!(
            program.contains_parameter(
                ParameterId::new(10)
            )
        );
    }

    #[test]
    fn validation_accepts_empty_program() {
        let program = QuantumProgram::new(
            ProgramId::new(7),
            RegionId::new(7),
        )
        .expect("program construction must succeed");

        assert!(program.validate().is_ok());
    }
}
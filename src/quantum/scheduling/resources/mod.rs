//! Zamani Quantum Scheduling — Resource Subsystem
//!
//! Stable composition boundary for all scheduler resource abstractions.
//!
//! # Purpose
//!
//! `quantum::scheduling::resources` defines the public namespace through which
//! the scheduling subsystem reasons about resources that may be required,
//! reserved, shared, consumed, released, unavailable, or hierarchically
//! composed during quantum execution.
//!
//! The resource subsystem is intentionally broader than qubits. A quantum
//! execution target may expose:
//!
//! - logical qubits;
//! - physical qubits;
//! - ancillas;
//! - control/drive channels;
//! - measurement/readout channels;
//! - resonators;
//! - couplers;
//! - lasers;
//! - microwave sources;
//! - optical channels;
//! - communication links;
//! - classical processors;
//! - classical memory;
//! - synchronization resources;
//! - accelerators;
//! - generic compute resources;
//! - generic memory;
//! - composite resources;
//! - target-specific resource classes.
//!
//! The scheduler must therefore never assume that a quantum machine is merely
//! a collection of qubits.
//!
//! # Architectural boundary
//!
//! The dependency direction is:
//!
//! ```text
//!                    canonical quantum IR
//!                           │
//!                           ▼
//!                 routing / target mapping
//!                           │
//!                           ▼
//!                hardware target description
//!                           │
//!                           ▼
//!             scheduling::resources adapter
//!                           │
//!              ┌────────────┼────────────┐
//!              ▼            ▼            ▼
//!           resource       pool      availability
//!              │            │            │
//!              └────────────┼────────────┘
//!                           ▼
//!                      reservations
//!                           │
//!                           ▼
//!                       planners
//!                           │
//!                           ▼
//!                      verification
//! ```
//!
//! This module coordinates those scheduler-resource components. It does not
//! perform scheduling itself.
//!
//! # Responsibilities
//!
//! This module owns:
//!
//! 1. resource-module organization;
//! 2. stable public imports;
//! 3. scheduler-resource API grouping;
//! 4. compile-time architectural boundaries;
//! 5. resource subsystem documentation.
//!
//! Child modules own their respective implementations:
//!
//! ```text
//! resource.rs       resource semantics and requirements
//! pool.rs            resource collections/pools
//! reservation.rs    temporal resource reservations
//! calendar.rs       resource occupancy calendars
//! availability.rs   availability state and queries
//! ```
//!
//! # Non-responsibilities
//!
//! This module does NOT own:
//!
//! - canonical quantum IR;
//! - `QubitId`;
//! - `PhysicalQubitId`;
//! - quantum gates;
//! - quantum operations;
//! - circuit semantics;
//! - routing;
//! - hardware discovery;
//! - calibration;
//! - provider APIs;
//! - authentication;
//! - scheduling algorithms;
//! - timing arithmetic;
//! - dependency analysis;
//! - QEC algorithms;
//! - execution;
//! - simulation;
//! - serialization formats;
//! - global mutable registries.
//!
//! Those responsibilities belong to their respective subsystems.
//!
//! # Canonical identity rule
//!
//! Resource scheduling MUST use the repository's canonical quantum identities.
//!
//! Logical and physical qubit identity is owned by:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! New scheduler-resource code must therefore use:
//!
//! ```rust
//! use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
//! ```
//!
//! It must NOT define another:
//!
//! ```text
//! QubitId
//! PhysicalQubitId
//! LogicalQubitId
//! PhysicalQubit
//! ResourceQubitId
//! ```
//!
//! unless a genuinely different semantic concept is being represented.
//!
//! Likewise, scheduler resources must consume the canonical scheduler/IR
//! identity types established elsewhere in the repository rather than
//! introducing incompatible identity wrappers merely for convenience.
//!
//! The repository's IR architecture explicitly establishes
//! `quantum::ir::qubit` as the canonical qubit identity boundary. The
//! scheduling IR follows the same rule and explicitly forbids competing
//! semantic representations.
//!
//! # Resource identity versus resource capacity
//!
//! These are different concepts:
//!
//! ```text
//! ResourceId
//!     = identity of one resource
//!
//! ResourceCapacity
//!     = amount of simultaneous usage that resource permits
//!
//! ResourceQuantity
//!     = amount an operation requests/consumes
//! ```
//!
//! For example:
//!
//! ```text
//! resource = control-channel-17
//! capacity = 4
//! request  = 2
//! ```
//!
//! does not mean that four resources exist. It means one resource exposes four
//! schedulable units and the operation consumes two of them.
//!
//! The child `resource` module is responsible for defining these semantics.
//!
//! # No fixed machine size
//!
//! Nothing in this module defines an architectural machine-size limit.
//!
//! In particular, this module contains no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_RESOURCES
//! MAX_CHANNELS
//! MAX_DEVICES
//! MAX_OPERATIONS
//! MAX_CAPACITY
//! MAX_SCHEDULE_DEPTH
//! MAX_SCHEDULING_HORIZON
//! ```
//!
//! The same resource subsystem must be able to describe targets containing:
//!
//! ```text
//! one resource
//! a few resources
//! thousands of resources
//! millions of resources
//! N resources
//! ```
//!
//! Subject to practical limits such as host memory, address space, compiler
//! time, explicit user limits, and actual target capacity.
//!
//! "Infinity" in Zamani therefore means:
//!
//! > the scheduler architecture does not encode an artificial finite machine
//! > size.
//!
//! It does not mean that physical computers have infinite memory or execution
//! time.
//!
//! # Sparse resource identity
//!
//! Resource identifiers may be sparse.
//!
//! A resource identifier such as `10_000_000` must not force the scheduler to
//! allocate storage for every identifier from zero through `10_000_000`.
//!
//! Sparse resources are particularly important for:
//!
//! - distributed systems;
//! - modular QPUs;
//! - partitioned machines;
//! - network resources;
//! - dynamically discovered resources;
//! - large physical identifiers;
//! - future heterogeneous execution targets.
//!
//! Storage strategy belongs to child implementations such as `pool.rs` and
//! `calendar.rs`; this module does not impose a particular data structure.
//!
//! # Resource semantics
//!
//! A resource can have different semantic behavior.
//!
//! Examples include:
//!
//! ```text
//! exclusive
//! shared
//! capacity-limited
//! reusable
//! consumable
//! hierarchical
//! dynamically available
//! conditionally available
//! ```
//!
//! The scheduler must not assume every resource behaves like an exclusive
//! single-capacity qubit.
//!
//! Resource semantics belong to `resource.rs`.
//!
//! # Temporal ownership
//!
//! Resource identity and resource occupancy are deliberately separated.
//!
//! ```text
//! resource.rs
//!     │
//!     │ describes what the resource is
//!     ▼
//! reservation.rs
//!     │
//!     │ describes an operation's temporal claim
//!     ▼
//! calendar.rs
//!     │
//!     │ describes occupancy over time
//!     ▼
//! availability.rs
//!     │
//!     │ describes whether the resource may currently be used
//!     ▼
//! planners
//! ```
//!
//! This separation allows a single resource model to be used by:
//!
//! - ASAP scheduling;
//! - ALAP scheduling;
//! - list scheduling;
//! - critical-path scheduling;
//! - resource-constrained scheduling;
//! - dynamic scheduling;
//! - distributed scheduling;
//! - runtime rescheduling;
//! - verification;
//! - diagnostics.
//!
//! # Resource requirements versus target resources
//!
//! A program expresses requirements.
//!
//! A target exposes resources.
//!
//! These must remain separate.
//!
//! ```text
//! Zamani program
//!       │
//!       ▼
//! resource requirement
//!       │
//!       ▼
//! target resource inventory
//!       │
//!       ▼
//! compatibility / availability
//!       │
//!       ▼
//! scheduler
//! ```
//!
//! The scheduling resource subsystem must therefore not silently transform a
//! program requirement into a particular hardware resource unless an explicit
//! mapping/adapter has performed that operation.
//!
//! # Logical and physical qubits
//!
//! Logical and physical qubits are resources from the scheduler's perspective,
//! but their identities remain canonical IR concepts.
//!
//! ```text
//! quantum::ir::qubit::QubitId
//!             │
//!             │ logical identity
//!             ▼
//!       scheduler resource
//!             │
//!             │ after routing
//!             ▼
//! quantum::ir::qubit::PhysicalQubitId
//!             │
//!             │ physical identity
//!             ▼
//!       scheduler resource
//! ```
//!
//! The scheduler does not perform logical-to-physical mapping.
//!
//! Routing owns that responsibility.
//!
//! Scheduling consumes the resulting mapping and schedules the mapped
//! operations against the available physical resources.
//!
//! # Hardware independence
//!
//! The resource namespace must remain independent of quantum technology.
//!
//! It must not contain assumptions specific to:
//!
//! - superconducting systems;
//! - trapped ions;
//! - neutral atoms;
//! - photonic systems;
//! - spin systems;
//! - topological systems;
//! - annealers;
//! - hybrid systems;
//! - distributed quantum systems;
//! - any particular vendor.
//!
//! Vendor and technology details enter through hardware adapters and generic
//! resource descriptions.
//!
//! # Resource hierarchy
//!
//! A resource may participate in a hierarchy.
//!
//! For example:
//!
//! ```text
//! network
//!   └── cluster
//!       └── node
//!           └── module
//!               └── device
//!                   ├── physical qubit
//!                   ├── control channel
//!                   └── measurement channel
//! ```
//!
//! The scheduler resource API must permit such hierarchies without imposing a
//! maximum depth or number of children.
//!
//! Hierarchy implementation belongs to `pool.rs` and the resource model.
//!
//! # Composite resources
//!
//! Some operations require a coordinated set of resources.
//!
//! For example:
//!
//! ```text
//! two physical qubits
//! + one coupler
//! + one control channel
//! ```
//!
//! This should be represented as a resource requirement/reservation group,
//! rather than encoded as a special-case two-qubit scheduler rule.
//!
//! This permits arbitrary operation arity and future hardware models.
//!
//! # Communication resources
//!
//! Communication links are first-class schedulable resources.
//!
//! This is required for scaling beyond a single QPU:
//!
//! ```text
//! QPU A
//!   │
//!   │ communication resource
//!   ▼
//! QPU B
//! ```
//!
//! The resource subsystem must therefore support resources that are not local
//! to a single device.
//!
//! Distributed scheduling consumes these abstractions through the same API.
//!
//! # Dynamic availability
//!
//! A resource can become unavailable without changing its identity.
//!
//! Examples:
//!
//! ```text
//! available
//! busy
//! reserved
//! disabled
//! degraded
//! maintenance
//! temporarily unavailable
//! unknown
//! ```
//!
//! These states belong to `availability.rs`.
//!
//! The scheduler must distinguish:
//!
//! ```text
//! resource does not exist
//!
//! resource exists but is unavailable
//!
//! resource exists and is available later
//!
//! resource exists but cannot satisfy this requirement
//! ```
//!
//! These are semantically different outcomes.
//!
//! # Reservation semantics
//!
//! A reservation represents a temporal claim over a resource.
//!
//! Conceptually:
//!
//! ```text
//! Reservation
//! ├── resource
//! ├── operation
//! ├── quantity
//! ├── start
//! ├── duration/end
//! └── reservation semantics
//! ```
//!
//! The concrete representation belongs to `reservation.rs`.
//!
//! This module only exposes its stable API.
//!
//! # Calendar semantics
//!
//! A resource calendar represents resource occupancy over time.
//!
//! It must support large and sparse schedules without requiring a dense
//! time-by-resource matrix.
//!
//! Conceptually:
//!
//! ```text
//! resource
//!     │
//!     └── intervals
//!           ├── reservation A
//!           ├── reservation B
//!           └── reservation C
//! ```
//!
//! This design permits scheduling horizons far larger than any fixed array
//! size and avoids coupling scheduler memory usage to an artificial machine
//! maximum.
//!
//! The concrete calendar data structure belongs to `calendar.rs`.
//!
//! # Availability semantics
//!
//! Availability must be queryable without forcing a hardware I/O operation.
//!
//! The hardware adapter is responsible for obtaining a target snapshot.
//!
//! The scheduler resource subsystem operates on that snapshot.
//!
//! ```text
//! live hardware / provider
//!          │
//!          ▼
//! hardware adapter
//!          │
//!          ▼
//! immutable target snapshot
//!          │
//!          ▼
//! scheduling resources
//! ```
//!
//! This makes compilation reproducible and avoids hidden side effects during
//! scheduling.
//!
//! # Determinism
//!
//! Resource iteration and externally visible ordering must be deterministic
//! whenever deterministic scheduling is requested.
//!
//! This matters because resource ordering can otherwise affect:
//!
//! - ready-operation arbitration;
//! - reservation order;
//! - generated schedules;
//! - diagnostics;
//! - serialized output;
//! - reproducibility.
//!
//! Determinism must not depend on unspecified hash-map iteration order.
//!
//! The child modules may choose suitable data structures, but any API exposed
//! through this module must preserve the deterministic behavior promised by the
//! scheduler configuration.
//!
//! # Concurrency
//!
//! This module introduces no global mutable state.
//!
//! Resource pools, calendars, and availability snapshots are owned by scheduler
//! contexts.
//!
//! Higher-level scheduling may use:
//!
//! - immutable snapshots;
//! - `Arc`;
//! - parallel analysis;
//! - partitioned scheduling;
//! - distributed scheduling.
//!
//! Synchronization policy belongs to the owner of the mutable state and must
//! not be hidden inside this namespace.
//!
//! # Thread safety
//!
//! The resource subsystem must not require a global lock.
//!
//! Individual resource structures should remain ordinary Rust values wherever
//! possible.
//!
//! Auto-trait properties such as `Send` and `Sync` are inherited from concrete
//! child implementations rather than asserted through unsafe mechanisms.
//!
//! # Error ownership
//!
//! Resource-domain errors belong to the resource component that owns the
//! invariant.
//!
//! This module must not create a competing global resource error hierarchy
//! merely for namespace convenience.
//!
//! Higher-level scheduling errors may wrap or translate resource errors at the
//! scheduler boundary.
//!
//! # Timing ownership
//!
//! This module must not define physical timing constants.
//!
//! It must not contain assumptions such as:
//!
//! ```text
//! 1 nanosecond
//! 4 nanoseconds
//! 20 samples
//! ```
//!
//! Resource occupancy receives timing information from the scheduling timing
//! subsystem.
//!
//! The relationship is:
//!
//! ```text
//! scheduling::timing
//!       │
//!       ▼
//! reservation interval
//!       │
//!       ▼
//! scheduling::resources
//! ```
//!
//! # Integration with timing
//!
//! `resources` consumes abstract temporal information.
//!
//! Timing owns:
//!
//! - `TimePoint`;
//! - `Duration`;
//! - time resolution;
//! - alignment;
//! - timing windows;
//! - temporal constraints.
//!
//! Resources own:
//!
//! - resource identity;
//! - resource capacity;
//! - resource requirements;
//! - resource availability;
//! - reservations;
//! - occupancy.
//!
//! Neither subsystem should duplicate the other's semantic types.
//!
//! # Integration with scheduling IR
//!
//! The scheduling IR represents normalized operations and their dependencies.
//!
//! ```text
//! quantum::ir
//!      │
//!      ▼
//! scheduling::adapters::ir
//!      │
//!      ▼
//! scheduling::ir::operation
//!      │
//!      ├──────────────► resource requirements
//!      │
//!      └──────────────► dependency graph
//!                            │
//!                            ▼
//!                        planners
//!                            │
//!                            ▼
//!                   resources::pool/calendar
//! ```
//!
//! The resource subsystem must not import scheduler planners merely to make
//! this relationship convenient.
//!
//! # Integration with routing
//!
//! Routing answers:
//!
//! > Which physical resources should logical operations use?
//!
//! Scheduling answers:
//!
//! > When can those operations use those resources?
//!
//! Therefore:
//!
//! ```text
//! quantum::routing
//!       │
//!       ▼
//! logical → physical mapping
//!       │
//!       ▼
//! scheduling resource requirements
//!       │
//!       ▼
//! resource availability
//!       │
//!       ▼
//! scheduling
//! ```
//!
//! This module must not implement routing.
//!
//! # Integration with hardware
//!
//! Hardware provides target-specific information such as:
//!
//! - resource inventory;
//! - resource capacities;
//! - supported operations;
//! - channel relationships;
//! - availability;
//! - calibration-derived constraints;
//! - timing information.
//!
//! Hardware-specific information must enter through a dedicated adapter.
//!
//! ```text
//! quantum::hardware
//!       │
//!       ▼
//! scheduling::adapters::hardware
//!       │
//!       ▼
//! scheduling::resources
//! ```
//!
//! The resource namespace must not depend on hardware providers.
//!
//! # Integration with QEC
//!
//! QEC may require resources such as:
//!
//! - data qubits;
//! - ancillas;
//! - syndrome measurement channels;
//! - classical decoding resources;
//! - feedback channels;
//! - communication resources.
//!
//! QEC expresses these as requirements.
//!
//! The generic scheduler-resource subsystem handles their allocation and
//! occupancy.
//!
//! ```text
//! QEC
//!  │
//!  ▼
//! resource requirements
//!  │
//!  ▼
//! scheduling::resources
//!  │
//!  ▼
//! scheduler
//! ```
//!
//! QEC algorithms remain outside this module.
//!
//! # Integration with distributed scheduling
//!
//! Distributed scheduling uses the same resource model for:
//!
//! - nodes;
//! - modules;
//! - devices;
//! - links;
//! - communication channels;
//! - shared classical resources.
//!
//! No second "distributed resource" abstraction should be created merely
//! because the resource belongs to another node.
//!
//! Scope and hierarchy describe locality.
//!
//! # Integration with verification
//!
//! The verification subsystem consumes resource reservations to establish:
//!
//! ```text
//! usage <= capacity
//!
//! no forbidden overlap
//!
//! required resources exist
//!
//! required resources were available
//!
//! reservation intervals are valid
//! ```
//!
//! Verification remains a downstream consumer.
//!
//! # Integration with diagnostics
//!
//! Diagnostics may inspect resource information to explain:
//!
//! ```text
//! operation O delayed
//! because resource R was occupied
//! until time T.
//! ```
//!
//! This module must expose enough stable information for diagnostics without
//! introducing diagnostics-specific state into the resource model.
//!
//! # Integration with serialization
//!
//! Resource objects may later be serialized through:
//!
//! ```text
//! quantum::scheduling::serialization
//! ```
//!
//! This module does not define a wire format.
//!
//! Serialization must preserve semantic identity and must not silently turn an
//! unbounded resource into an arbitrary numeric sentinel.
//!
//! # Integration with plugins
//!
//! Scheduler plugins may consume or provide resource models through explicit
//! interfaces.
//!
//! Plugins must not mutate hidden global resource registries.
//!
//! A plugin receives resource information through an explicit scheduler context
//! or target snapshot.
//!
//! # Security
//!
//! This module must not contain credentials, provider tokens, network clients,
//! or hardware control handles.
//!
//! Resource descriptions are data, not authority to access a physical machine.
//!
//! Any resource metadata originating from an untrusted source must be validated
//! by the appropriate boundary before being used by a scheduler.
//!
//! # No unsafe
//!
//! This entire scheduling-resource subsystem is intended to be implemented in
//! safe stable Rust.
//!
//! The module explicitly enforces:
//!
//! ```rust
//! #![forbid(unsafe_code)]
//! #![deny(unsafe_op_in_unsafe_fn)]
//! ```
//!
//! No resource operation requires unsafe Rust.
//!
//! # Rust compatibility
//!
//! Supported compiler baseline:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! The module must not rely on APIs newer than the project's Rust baseline.
//!
//! # Public API philosophy
//!
//! `mod.rs` intentionally exposes explicit imports rather than wildcard
//! re-exports.
//!
//! This prevents accidental expansion of the scheduler's public API whenever a
//! child implementation gains an internal helper.
//!
//! Public types should be re-exported here only when they are part of the
//! stable resource subsystem contract.
//!
//! Implementation details remain accessible through their child module when
//! necessary.
//!
//! # Child module contracts
//!
//! ## `resource`
//!
//! Defines the foundational resource vocabulary.
//!
//! It owns:
//!
//! - `Resource`;
//! - `ResourceKind`;
//! - `ResourceScope`;
//! - `ResourceCapacity`;
//! - `ResourceQuantity`;
//! - resource requirements;
//! - resource semantics;
//! - local resource validation.
//!
//! It consumes canonical:
//!
//! ```text
//! quantum::ir::core::identity::ResourceId
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! ## `pool`
//!
//! Defines collections of resources and resource lookup/selection operations.
//!
//! It owns:
//!
//! - resource membership;
//! - hierarchical grouping;
//! - deterministic iteration;
//! - sparse lookup;
//! - capacity-aware resource selection;
//! - resource-group queries.
//!
//! It must not create another resource identity type.
//!
//! ## `reservation`
//!
//! Defines temporal claims over resources.
//!
//! It owns:
//!
//! - reservation identity;
//! - operation association;
//! - quantity;
//! - temporal interval;
//! - reservation semantics;
//! - local reservation validation.
//!
//! It consumes timing types rather than redefining them.
//!
//! ## `calendar`
//!
//! Defines resource occupancy over time.
//!
//! It owns:
//!
//! - insertion/removal/query of reservations;
//! - conflict detection;
//! - capacity accounting;
//! - deterministic conflict reporting;
//! - scalable interval storage.
//!
//! It must not assume a dense time axis.
//!
//! ## `availability`
//!
//! Defines resource availability state and availability windows.
//!
//! It owns:
//!
//! - available/unavailable state;
//! - dynamic state transitions;
//! - maintenance/exclusion windows;
//! - availability queries;
//! - target snapshot semantics.
//!
//! It must not perform live hardware I/O.
//!
//! # Required child-module dependency direction
//!
//! The intended dependency graph is:
//!
//! ```text
//! resource
//!    │
//!    ├───────────────┐
//!    ▼               ▼
//! pool          reservation
//!    │               │
//!    │               ▼
//!    │           calendar
//!    │               │
//!    └───────┬───────┘
//!            ▼
//!       availability
//!            │
//!            ▼
//!         planners
//! ```
//!
//! More precisely, child modules should depend only on contracts below them.
//! They must not form cycles merely through `mod.rs`.
//!
//! # Stable import examples
//!
//! New scheduling code may use:
//!
//! ```rust
//! use crate::quantum::scheduling::resources::Resource;
//! use crate::quantum::scheduling::resources::ResourceCapacity;
//! use crate::quantum::scheduling::resources::ResourceKind;
//! ```
//!
//! For canonical qubit identities:
//!
//! ```rust
//! use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
//! ```
//!
//! Do not import a scheduler-local replacement qubit type.
//!
//! # Resource API and "write once, scale everywhere"
//!
//! The resource namespace is one of the mechanisms that allows the same Zamani
//! program to be compiled for different targets.
//!
//! The program expresses semantics and requirements.
//!
//! The selected target supplies an inventory.
//!
//! The resource adapter converts the target inventory into generic scheduling
//! resources.
//!
//! The scheduler then chooses legal temporal allocations.
//!
//! ```text
//!                         SAME PROGRAM
//!                              │
//!                 ┌────────────┼────────────┐
//!                 ▼            ▼            ▼
//!              small QPU    large QPU    distributed QPU
//!                 │            │            │
//!                 ▼            ▼            ▼
//!             resource      resource      resource
//!             inventory     inventory     inventory
//!                 │            │            │
//!                 └────────────┼────────────┘
//!                              ▼
//!                       same scheduler API
//! ```
//!
//! No program rewrite is required merely because the target exposes a
//! different number or organization of resources.
//!
//! # Scalability contract
//!
//! The resource subsystem must scale with the resources actually represented by
//! the target rather than with an artificial compile-time maximum.
//!
//! Implementations must avoid algorithms that allocate structures proportional
//! to an assumed maximum identifier.
//!
//! Prefer:
//!
//! - sparse collections;
//! - interval representations;
//! - indexed lookup;
//! - hierarchical partitioning;
//! - streaming queries;
//! - immutable snapshots;
//! - explicit resource partitions;
//! - bounded working sets when requested by policy.
//!
//! The correct data structure depends on the operation being performed and is
//! therefore owned by the corresponding child module.
//!
//! # Failure semantics
//!
//! Resource operations must distinguish at least:
//!
//! ```text
//! unknown resource
//! unavailable resource
//! insufficient capacity
//! conflicting reservation
//! invalid requirement
//! invalid reservation
//! invalid interval
//! unsupported resource kind
//! ```
//!
//! A caller must never have to parse an error string to determine which case
//! occurred.
//!
//! Concrete error types belong to their owning child modules and may be
//! translated by the scheduler's central error layer.
//!
//! # Transactional mutation requirement
//!
//! Resource calendars and pools must not leave partially mutated state after a
//! failed operation.
//!
//! For example:
//!
//! ```text
//! attempt reservation
//!      │
//!      ├── validate all requirements
//!      ├── validate all resources
//!      ├── validate capacity
//!      ├── validate timing
//!      └── commit atomically
//! ```
//!
//! If validation fails, the resource state must remain unchanged.
//!
//! The concrete transaction mechanism belongs to the child implementation.
//!
//! # Deterministic resource selection
//!
//! When multiple resources satisfy a requirement, deterministic mode must have
//! a defined ordering.
//!
//! The ordering must be based on explicit semantic fields such as canonical
//! resource identity or configured priority, not incidental hash iteration.
//!
//! Resource selection policy belongs to `pool.rs`; the deterministic contract
//! is documented here because it affects scheduler reproducibility.
//!
//! # No global resource registry
//!
//! Do not introduce:
//!
//! ```rust
//! static GLOBAL_RESOURCES: ...;
//! ```
//!
//! or an equivalent process-wide mutable registry.
//!
//! Resource inventories belong to explicit target/scheduling contexts.
//!
//! This is necessary for:
//!
//! - compiling multiple programs concurrently;
//! - compiling against multiple targets;
//! - deterministic tests;
//! - distributed compilation;
//! - simulations;
//! - nested scheduling;
//! - isolation between users/programs.
//!
//! # Snapshot model
//!
//! A production scheduler should operate against an explicit resource snapshot
//! wherever possible.
//!
//! Conceptually:
//!
//! ```text
//! hardware state at T0
//!        │
//!        ▼
//! target snapshot
//!        │
//!        ▼
//! scheduler context
//!        │
//!        ▼
//! resource pool/calendar
//! ```
//!
//! This prevents the scheduler from silently changing behavior because live
//! hardware state changed in the middle of compilation.
//!
//! Runtime rescheduling may intentionally obtain a new snapshot and create a
//! new scheduling context.
//!
//! # Resource release
//!
//! Reservation ownership must be explicit.
//!
//! A completed operation releases reusable capacity according to reservation
//! semantics.
//!
//! Consumable resources must not be automatically treated as reusable.
//!
//! This distinction belongs to `resource.rs` and `reservation.rs`.
//!
//! # Nested scheduling
//!
//! The resource API must permit a higher-level scheduler to schedule a
//! composite operation whose internal operations are scheduled later.
//!
//! This is important for:
//!
//! - macro operations;
//! - QEC blocks;
//! - pulse bundles;
//! - distributed operations;
//! - modular compilation;
//! - hierarchical scheduling.
//!
//! The parent operation can reserve a composite resource while child scheduling
//! resolves internal resource usage.
//!
//! The exact semantics belong to the relevant planner/resource implementation.
//!
//! # Testing contract
//!
//! The resource subsystem must eventually be covered by:
//!
//! ```text
//! unit tests
//! property tests
//! regression tests
//! deterministic tests
//! scalability tests
//! concurrency tests
//! integration tests
//! ```
//!
//! Tests must cover at least:
//!
//! - zero resources;
//! - one resource;
//! - sparse resource identifiers;
//! - many resources;
//! - finite capacity;
//! - unlimited capacity;
//! - zero requested quantity;
//! - exact capacity usage;
//! - capacity overflow;
//! - concurrent reservations;
//! - overlapping intervals;
//! - adjacent intervals;
//! - unavailable resources;
//! - future availability;
//! - resource release;
//! - composite resources;
//! - physical qubit resources;
//! - logical qubit resources;
//! - communication resources;
//! - deterministic selection;
//! - failed transactional mutations.
//!
//! Scalability tests must increase resource count without changing resource
//! APIs or introducing machine-size constants.
//!
//! # API stability
//!
//! This module is intended to remain stable while implementation details inside
//! child files evolve.
//!
//! Adding a new internal helper to a child module must not require changing
//! this file.
//!
//! Adding a new resource subsystem component should require:
//!
//! 1. a new dedicated child file;
//! 2. a complete ownership contract;
//! 3. explicit module declaration here;
//! 4. only the necessary stable re-exports;
//! 5. tests for the new component.
//!
//! Existing child APIs must not be modified merely to accommodate unrelated
//! scheduler features.
//!
//! # Module implementation
//!
//! The declarations below intentionally contain no implementation logic.
//!
//! They establish the stable module boundary for the resource subsystem.
//!
//! Every declared child is expected to be a complete, independently testable
//! implementation with the contract documented above.
//!
//! # Safety policy
//!
//! The scheduler-resource subsystem is entirely safe Rust.
//!
//! The following attributes make that requirement compiler-enforced for this
//! module:
//!
//! ```rust
//! #![forbid(unsafe_code)]
//! #![deny(unsafe_op_in_unsafe_fn)]
//! ```
//!
//! Child modules should carry the same policy independently so that a future
//! implementation cannot accidentally introduce unsafe code without an
//! explicit architectural decision.
//!
//! # Completion invariant
//!
//! This file is complete when all of the following remain true:
//!
//! 1. It contains no resource implementation logic.
//! 2. It contains no scheduler algorithm.
//! 3. It contains no hardware I/O.
//! 4. It contains no global mutable state.
//! 5. It introduces no competing qubit identity.
//! 6. It introduces no competing canonical resource identity.
//! 7. It introduces no fixed machine-size limit.
//! 8. It introduces no vendor dependency.
//! 9. It introduces no timing constants.
//! 10. It provides stable child-module paths.
//! 11. It provides explicit stable re-exports.
//! 12. Child modules can evolve independently.
//! 13. Downstream planners can consume the resource API without modifying this
//!     file merely because a planner was added.
//! 14. Hardware adapters can populate the resource model without modifying this
//!     file merely because a new hardware target was added.
//! 15. Routing can provide physical identities without modifying this file.
//! 16. QEC can express resource requirements without modifying this file.
//! 17. Distributed scheduling can consume communication resources without
//!     modifying this file.
//! 18. Rust 1.97.1 accepts the module under the repository's Rust 2021 baseline.
//!
//! # Future extension rule
//!
//! If a new resource concern appears, do not add implementation logic here.
//!
//! Instead:
//!
//! ```text
//! new concern
//!      │
//!      ▼
//! dedicated resource child module
//!      │
//!      ▼
//! complete local contract
//!      │
//!      ▼
//! tests
//!      │
//!      ▼
//! explicit declaration/re-export here
//! ```
//!
//! This keeps `mod.rs` stable and prevents it from becoming a second resource
//! implementation.
//!
//! # Current intended tree
//!
//! ```text
//! src/quantum/scheduling/resources/
//! ├── mod.rs
//! ├── resource.rs
//! ├── pool.rs
//! ├── reservation.rs
//! ├── calendar.rs
//! └── availability.rs
//! ```
//!
//! `resource.rs` is the foundational semantic layer.
//!
//! `pool.rs`, `reservation.rs`, `calendar.rs`, and `availability.rs` consume
//! that vocabulary.
//!
//! Higher-level scheduling modules consume all of them through this namespace.
//!
//! # Integration examples
//!
//! A planner should conceptually be able to write:
//!
//! ```rust
//! use crate::quantum::scheduling::resources::{
//!     Resource,
//!     ResourceCapacity,
//!     ResourceKind,
//! };
//! ```
//!
//! Canonical qubit identity remains:
//!
//! ```rust
//! use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
//! ```
//!
//! A hardware adapter should populate a resource pool from the target
//! description.
//!
//! A planner should reserve resources through the reservation/calendar API.
//!
//! A verifier should inspect those reservations.
//!
//! None of those consumers should need to know how a resource pool internally
//! stores its entries.
//!
//! # Final architectural principle
//!
//! ```text
//!                    ZAMANI PROGRAM
//!                         │
//!                         ▼
//!                     canonical IR
//!                         │
//!                         ▼
//!                       routing
//!                         │
//!                    logical → physical
//!                         │
//!                         ▼
//!                 hardware target snapshot
//!                         │
//!                         ▼
//!              scheduling resource adapter
//!                         │
//!                         ▼
//!             ┌──────────────────────────┐
//!             │ scheduling::resources    │
//!             │                          │
//!             │ resource                 │
//!             │ pool                     │
//!             │ reservation              │
//!             │ calendar                 │
//!             │ availability             │
//!             └────────────┬─────────────┘
//!                          │
//!                          ▼
//!                     scheduler
//!                          │
//!                          ▼
//!                      verifier
//!                          │
//!                          ▼
//!                       runtime
//! ```
//!
//! The program remains independent of machine size.
//!
//! The resource inventory changes with the target.
//!
//! The scheduler adapts to the available resources.
//!
//! No artificial finite machine-size limit is encoded in this module.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

// =============================================================================
// Child modules
// =============================================================================
//
// These modules are deliberately separated by responsibility.
//
// IMPORTANT:
// `mod.rs` does not define resource behavior. The child modules are the
// semantic owners of their respective APIs.
//
// The declarations are intentionally explicit rather than wildcard-based.

pub mod resource;
pub mod pool;
pub mod reservation;
pub mod calendar;
pub mod availability;

// =============================================================================
// Stable public resource vocabulary
// =============================================================================
//
// These are the foundational types that downstream scheduler components are
// expected to use.
//
// The concrete definitions remain owned by `resource.rs`.
//
// IMPORTANT:
// Do not add local aliases for QubitId or PhysicalQubitId here. Those identities
// belong to `crate::quantum::ir::qubit`.

pub use resource::{
    Resource,
    ResourceCapacity,
    ResourceKind,
    ResourceQuantity,
    ResourceScope,
};

// =============================================================================
// Stable pool API
// =============================================================================
//
// `pool.rs` is the owner of resource collections, lookup, grouping, and
// deterministic resource selection.
//
// The exact public types should be re-exported only once that child contract
// is established. Keeping the declaration here separate prevents this module
// from becoming coupled to implementation details.
//
// Expected future public surface:
//
// pub use pool::{
//     ResourcePool,
//     ResourcePoolError,
//     ResourceSelection,
// };
//
// Those exports intentionally remain absent until the child file defines the
// final stable API. This avoids inventing a type contract in mod.rs that would
// force a later incompatible rewrite.

// =============================================================================
// Stable reservation API
// =============================================================================
//
// `reservation.rs` owns temporal resource claims.
//
// Expected future public surface:
//
// pub use reservation::{
//     ResourceReservation,
//     ReservationError,
// };
//
// Timing types must come from `crate::quantum::scheduling::timing`; this module
// must not redefine TimePoint or Duration.

// =============================================================================
// Stable calendar API
// =============================================================================
//
// `calendar.rs` owns temporal occupancy and conflict detection.
//
// Expected future public surface:
//
// pub use calendar::{
//     ResourceCalendar,
//     ResourceCalendarError,
// };
//
// The calendar implementation must remain sparse/time-interval based rather
// than allocating a dense machine-size-by-time matrix.

// =============================================================================
// Stable availability API
// =============================================================================
//
// `availability.rs` owns resource availability snapshots and queries.
//
// Expected future public surface:
//
// pub use availability::{
//     ResourceAvailability,
//     AvailabilityState,
// };
//
// Availability must operate on explicit snapshots and must not perform hidden
// hardware/provider I/O.

// =============================================================================
// Canonical qubit identity guidance
// =============================================================================
//
// Resource modules that need logical or physical qubit identity MUST import:
//
// use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
//
// Do NOT define:
//
// type QubitId = ...;
// type PhysicalQubitId = ...;
//
// inside this module.
//
// The repository's canonical IR architecture explicitly establishes
// `quantum::ir::qubit` as the authoritative identity boundary.
//
// =============================================================================
// Dependency direction
// =============================================================================
//
// The intended dependency direction is:
//
//     quantum::ir::qubit
//              │
//              ▼
//         resources::resource
//              │
//        ┌─────┼──────┐
//        ▼     ▼      ▼
//      pool reservation availability
//        │     │      │
//        └─────┼──────┘
//              ▼
//          calendar
//              │
//              ▼
//          scheduling
//
// `resources` must not import:
//
//     scheduling::planners
//     scheduling::policies
//     scheduling::verification
//     quantum::routing
//     quantum::hardware
//     quantum::runtime
//
// Those subsystems consume resource semantics through this boundary.

// =============================================================================
// Public API discipline
// =============================================================================
//
// Do not add:
//
//     pub use resource::*;
//
// or equivalent wildcard exports.
//
// Explicit exports are intentional. They make the stable scheduler API
// reviewable and prevent private implementation helpers from accidentally
// becoming public API.
//
// When a child module becomes production-complete, add only the types that are
// genuinely part of the stable cross-module contract.
//
// =============================================================================
// No global state
// =============================================================================
//
// This module deliberately contains no:
//
//     static mut
//     global registry
//     singleton resource pool
//     thread-local resource state
//
// Resource ownership belongs to explicit scheduler/target contexts.
//
// =============================================================================
// No machine-size assumptions
// =============================================================================
//
// This module intentionally contains no numeric constants describing:
//
//     qubit count
//     channel count
//     resource count
//     capacity limit
//     schedule horizon
//     topology size
//
// Explicit execution limits, if required, belong to the scheduler limits or
// target-capability layers rather than this semantic namespace.
//
// =============================================================================
// No vendor assumptions
// =============================================================================
//
// Nothing here should require:
//
//     IBM
//     IonQ
//     Quantinuum
//     Rigetti
//     D-Wave
//     CUDA
//     provider credentials
//     device addresses
//     vendor SDKs
//
// Vendor-specific resource descriptions are adapted into the generic resource
// model by the hardware adapter.
//
// =============================================================================
// No timing ownership
// =============================================================================
//
// This module does not define physical durations or clock resolution.
//
// Resource reservations consume timing abstractions from:
//
//     crate::quantum::scheduling::timing
//
// This separation prevents resource semantics from becoming coupled to one
// physical technology.

// =============================================================================
// No execution ownership
// =============================================================================
//
// A resource represents something that may be required/reserved.
//
// It is NOT a hardware handle.
//
// This namespace therefore never performs:
//
//     allocate_on_qpu()
//     execute()
//     connect()
//     authenticate()
//     submit_job()
//
// Execution belongs to the runtime/hardware layers.

// =============================================================================
// Integration checklist
// =============================================================================
//
// A completed resource subsystem must integrate as follows:
//
// 1. Canonical IR
//
//     crate::quantum::ir
//          │
//          ▼
//     scheduling::adapters::ir
//          │
//          ▼
//     resource requirements
//
// 2. Canonical qubits
//
//     crate::quantum::ir::qubit::{QubitId, PhysicalQubitId}
//
// 3. Routing
//
//     crate::quantum::routing
//          │
//          ▼
//     physical identities
//          │
//          ▼
//     resource requirements
//
// 4. Hardware
//
//     crate::quantum::hardware
//          │
//          ▼
//     scheduling::adapters::hardware
//          │
//          ▼
//     resource inventory + capacity + availability
//
// 5. Timing
//
//     scheduling::timing
//          │
//          ▼
//     reservations/calendars
//
// 6. Planners
//
//     scheduling::planners
//          │
//          ▼
//     resource selection + reservation
//
// 7. Verification
//
//     scheduling::verification
//          │
//          ▼
//     reservation/capacity validation
//
// 8. QEC
//
//     quantum::error_correction
//          │
//          ▼
//     resource requirements
//
// 9. Distributed scheduling
//
//     nodes + modules + devices + communication links
//          │
//          ▼
//     same resource API
//
// 10. Runtime
//
//     verified schedule
//          │
//          ▼
//     hardware/runtime execution
//
// =============================================================================
// File completion rule
// =============================================================================
//
// This file should not need to be edited merely because:
//
// - a new scheduler algorithm is added;
// - a new QPU provider is added;
// - a new quantum technology is added;
// - the number of qubits increases;
// - a new QEC code is added;
// - distributed execution is added;
// - a new timing strategy is added;
// - a new optimization objective is added;
// - a new diagnostic is added.
//
// A change is required here only when the resource subsystem's public module
// organization or stable public API itself changes.
//
// That is the intended "finish this file once" contract.
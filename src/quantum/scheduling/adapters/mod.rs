//! Zamani Quantum Scheduling — Integration Adapters
//!
//! `src/quantum/scheduling/adapters/mod.rs`
//!
//! Production namespace and composition boundary for integrations between the
//! scheduler and the other Zamani quantum subsystems.
//!
//! # Purpose
//!
//! This module is the authoritative namespace for scheduler-owned integration
//! adapters.
//!
//! The adapters bridge:
//!
//! ```text
//! canonical Quantum IR
//!        │
//!        ▼
//! adapters::ir
//!        │
//!        ▼
//! scheduling::ir
//!
//! routing
//!        │
//!        ▼
//! adapters::routing
//!        │
//!        ▼
//! scheduling
//!
//! hardware
//!        │
//!        ▼
//! adapters::hardware
//!        │
//!        ▼
//! scheduling
//!
//! QEC
//!        │
//!        ▼
//! adapters::qec
//!        │
//!        ▼
//! scheduling
//! ```
//!
//! The adapter layer exists to keep these subsystem boundaries explicit.
//!
//! # Architectural principle
//!
//! A Zamani quantum program is written at the semantic level and specialized
//! only when compiled for a concrete target.
//!
//! Therefore:
//!
//! ```text
//! Zamani program
//!       │
//!       ▼
//! canonical quantum::ir
//!       │
//!       ▼
//! optimization
//!       │
//!       ▼
//! routing
//!       │
//!       ▼
//! scheduling
//!       │
//!       ▼
//! hardware / runtime
//! ```
//!
//! This module must never introduce a machine-size assumption.
//!
//! In particular, it must never contain:
//!
//! ```text
//! MAX_QUBITS
//! MAX_PHYSICAL_QUBITS
//! MAX_OPERATIONS
//! MAX_CHANNELS
//! MAX_RESOURCES
//! DEFAULT_QUBIT_COUNT
//! DEFAULT_TOPOLOGY
//! DEFAULT_GATE_DURATION
//! ```
//!
//! Concrete capacity comes from the supplied target, resource model, execution
//! policy, and available host resources.
//!
//! "Infinity" means that this module imposes no artificial architectural
//! ceiling. Every concrete compilation remains bounded by the resources
//! actually available to that invocation.
//!
//! # Ownership
//!
//! The adapters are scheduler-owned integration boundaries.
//!
//! They do NOT become owners of the concepts they adapt.
//!
//! ## `ir`
//!
//! `adapters::ir` adapts the canonical quantum IR into the scheduler's
//! algorithm-facing representation.
//!
//! The canonical IR remains authoritative for:
//!
//! - `QuantumCircuit`;
//! - `Gate`;
//! - `QuantumOperation`;
//! - quantum semantics;
//! - gate parameters;
//! - operand ordering;
//! - `QubitId`;
//! - `PhysicalQubitId`.
//!
//! The adapter must not redefine any of those concepts.
//!
//! ## `routing`
//!
//! `adapters::routing` translates routing output into a scheduler-facing
//! representation.
//!
//! Routing remains authoritative for:
//!
//! - logical-to-physical placement;
//! - movement operations;
//! - routed operation ordering;
//! - routing-specific metadata.
//!
//! Scheduling remains responsible for determining when routed operations can
//! execute.
//!
//! ## `hardware`
//!
//! `adapters::hardware` translates authoritative hardware target information
//! into scheduler-facing target/resource information.
//!
//! Hardware remains authoritative for:
//!
//! - backend identity;
//! - backend capabilities;
//! - backend limits;
//! - topology;
//! - physical resources;
//! - timing capabilities;
//! - calibration;
//! - device state;
//! - provider-specific behavior.
//!
//! The scheduler must never recreate these concepts merely for convenience.
//!
//! ## `qec`
//!
//! `adapters::qec` translates QEC scheduling requirements into generic
//! scheduling representations.
//!
//! QEC remains authoritative for:
//!
//! - QEC semantics;
//! - code definitions;
//! - syndrome semantics;
//! - stabilizer semantics;
//! - QEC operation identity;
//! - QEC rounds;
//! - decoding;
//! - fault-tolerance algorithms.
//!
//! The generic scheduler determines when valid QEC work can execute.
//!
//! # Canonical qubit identity
//!
//! The canonical qubit identities are owned by:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! Adapter modules must use those exact identities whenever their contract
//! requires canonical quantum qubit identities.
//!
//! They must never introduce another:
//!
//! ```text
//! QubitId
//! PhysicalQubitId
//! ```
//!
//! for scheduler convenience.
//!
//! A subsystem-specific identity is permitted only when it has genuinely
//! different semantics and is explicitly named accordingly.
//!
//! For example, a QEC operation identity is not a qubit identity and must not
//! be represented as one.
//!
//! # Canonical IR boundary
//!
//! The dependency direction is:
//!
//! ```text
//!                         quantum::ir
//!                              │
//!                              ▼
//!                       adapters::ir
//!                              │
//!                              ▼
//!                     scheduling::ir
//!                              │
//!             ┌────────────────┼────────────────┐
//!             ▼                ▼                ▼
//!         dependency       resources         timing
//!             │                │                │
//!             └────────────────┼────────────────┘
//!                              ▼
//!                           planners
//! ```
//!
//! `adapters::ir` is therefore the scheduler-owned boundary through which
//! canonical IR enters scheduling-specific analysis.
//!
//! The scheduler adapters must not modify canonical IR semantics.
//!
//! # Routing boundary
//!
//! Routing answers:
//!
//! > Where should an operation execute?
//!
//! Scheduling answers:
//!
//! > When can that operation execute?
//!
//! Hardware answers:
//!
//! > Can the target actually execute it?
//!
//! Therefore the intended flow is:
//!
//! ```text
//! canonical IR
//!      │
//!      ▼
//! routing
//!      │
//!      ▼
//! adapters::routing
//!      │
//!      ▼
//! scheduling
//! ```
//!
//! The routing adapter must not:
//!
//! - perform routing;
//! - select a routing algorithm;
//! - mutate a routing mapping;
//! - calculate physical paths;
//! - query hardware;
//! - assign execution times;
//! - reserve scheduler resources;
//! - perform QEC;
//! - execute a QPU.
//!
//! # Hardware boundary
//!
//! The hardware adapter consumes target information supplied by
//! `quantum::hardware`.
//!
//! The intended flow is:
//!
//! ```text
//! quantum::hardware
//!       │
//!       ├── capabilities
//!       ├── limits
//!       ├── topology
//!       ├── resources
//!       ├── timing
//!       ├── availability
//!       └── calibration
//!              │
//!              ▼
//!     adapters::hardware
//!              │
//!              ▼
//!     SchedulingContext
//! ```
//!
//! The adapter must not:
//!
//! - authenticate with a provider;
//! - open network connections;
//! - discover devices;
//! - poll a QPU;
//! - execute jobs;
//! - modify hardware state;
//! - embed provider SDK types;
//! - invent hardware timing constants;
//! - synthesize resource inventories from numeric limits.
//!
//! A hardware limit is not necessarily a resource inventory.
//!
//! For example:
//!
//! ```text
//! max_physical_qubits
//!        !=
//! actual physical-qubit resources
//! ```
//!
//! This distinction is required for sparse, hierarchical, modular,
//! distributed, dynamically allocated, or provider-defined resources.
//!
//! # QEC boundary
//!
//! QEC requirements enter through:
//!
//! ```text
//! quantum::error_correction
//!          │
//!          ▼
//! scheduling::qec
//!          │
//!          ▼
//! adapters::qec
//!          │
//!          ▼
//! scheduling::ir
//! ```
//!
//! The adapter must preserve semantic identity and dependency information
//! without converting QEC concepts into unrelated scheduler concepts.
//!
//! In particular, a QEC operation identity must never be treated as a generic
//! scheduler operation identity unless an explicit collision-free mapping has
//! been supplied by the integration boundary.
//!
//! Likewise, a physical QEC qubit must not silently become a logical
//! `QubitId`.
//!
//! # Adapter independence
//!
//! Each adapter is independently owned and independently testable.
//!
//! A new scheduler algorithm must not require changes to this module.
//!
//! A new routing algorithm must not require changes to this module.
//!
//! A new hardware provider must not require changes to this module.
//!
//! A new QEC code must not require changes to this module.
//!
//! A new resource type must not require changes to this module.
//!
//! A new timing model must not require changes to this module.
//!
//! New behavior belongs in the corresponding adapter implementation or owning
//! subsystem.
//!
//! This is essential for the "finish one file once" development model.
//!
//! # No implementation duplication
//!
//! This module must not define:
//!
//! - `SchedulingOperation`;
//! - `SchedulingContext`;
//! - `Resource`;
//! - `QubitId`;
//! - `PhysicalQubitId`;
//! - `OperationId`;
//! - routing algorithms;
//! - scheduling algorithms;
//! - hardware capabilities;
//! - QEC algorithms.
//!
//! Those concepts have authoritative owners elsewhere.
//!
//! This file only composes adapter modules.
//!
//! # Adapter API stability
//!
//! Each adapter should expose its own stable contract.
//!
//! This module intentionally does not flatten all adapter symbols into one
//! namespace.
//!
//! Prefer:
//!
//! ```text
//! quantum::scheduling::adapters::ir::IrAdapter
//! quantum::scheduling::adapters::routing::RoutingAdapter
//! quantum::scheduling::adapters::hardware::HardwareSchedulingView
//! quantum::scheduling::adapters::qec::QecAdapter
//! ```
//!
//! over:
//!
//! ```text
//! quantum::scheduling::adapters::IrAdapter
//! quantum::scheduling::adapters::RoutingAdapter
//! ```
//!
//! The nested namespaces prevent collisions and make ownership explicit.
//!
//! # Why wildcard re-exports are prohibited
//!
//! This module must not do:
//!
//! ```text
//! pub use ir::*;
//! pub use routing::*;
//! pub use hardware::*;
//! pub use qec::*;
//! ```
//!
//! Wildcard re-exports make the adapter namespace unstable and can cause
//! unrelated adapter additions to become breaking API changes.
//!
//! Explicit re-exports should only be introduced when there is a compelling,
//! documented compatibility requirement.
//!
//! The preferred API remains namespaced.
//!
//! # Data ownership
//!
//! Adapters should normally follow these rules:
//!
//! ```text
//! source subsystem
//!       │
//!       │ authoritative data
//!       ▼
//! adapter
//!       │
//!       │ derived immutable view
//!       ▼
//! scheduler
//! ```
//!
//! An adapter must not silently acquire ownership of caller state merely to
//! perform translation.
//!
//! Where large inputs are involved, iterator-based APIs should be preferred
//! when the underlying representation permits streaming.
//!
//! This supports very large programs without forcing unnecessary duplicate
//! allocations.
//!
//! # Scalability
//!
//! The adapter namespace must scale with the data actually supplied to it.
//!
//! It must not allocate based on hypothetical machine size.
//!
//! Forbidden patterns include concepts equivalent to:
//!
//! ```text
//! allocate_for_max_qubits()
//! allocate_for_max_operations()
//! allocate_for_max_channels()
//! allocate_for_max_devices()
//! ```
//!
//! Instead:
//!
//! ```text
//! supplied input
//!       │
//!       ▼
//! actual required representation
//! ```
//!
//! Adapter implementations should prefer:
//!
//! - lazy iteration;
//! - borrowed views;
//! - immutable snapshots;
//! - streaming conversion;
//! - caller-controlled ownership;
//! - deterministic traversal;
//! - checked arithmetic.
//!
//! Materialized conversion is appropriate when ownership is explicitly
//! requested by the caller.
//!
//! # Large-scale execution
//!
//! For very large quantum machines, the adapter layer must not become a
//! serialization bottleneck or a mandatory full-copy boundary.
//!
//! The architecture therefore supports both:
//!
//! ```text
//! borrowed/lazy adapter view
//! ```
//!
//! and:
//!
//! ```text
//! owned/materialized adapter representation
//! ```
//!
//! The choice belongs to the concrete adapter contract.
//!
//! # Distributed quantum computing
//!
//! Adapter boundaries must remain valid when a target consists of multiple
//! quantum modules or QPUs.
//!
//! The hardware and routing adapters may therefore expose information for:
//!
//! - one device;
//! - multiple chips;
//! - modular QPUs;
//! - distributed QPUs;
//! - quantum-network nodes;
//! - communication links;
//! - future architectures.
//!
//! This module must not assume that one physical machine has one topology,
//! one resource namespace, or one execution clock.
//!
//! # Dynamic circuits
//!
//! Adapter contracts must remain capable of representing workloads containing:
//!
//! - measurement;
//! - classical dependencies;
//! - conditional operations;
//! - feedback;
//! - runtime decisions;
//! - communication completion events.
//!
//! Static scheduling and dynamic scheduling are distinct execution modes, but
//! they share the same adapter namespace.
//!
//! # Timing
//!
//! Timing information must flow from the appropriate target/QEC contracts.
//!
//! This module must never define:
//!
//! ```text
//! DEFAULT_GATE_DURATION
//! DEFAULT_CLOCK_PERIOD
//! DEFAULT_DT
//! ```
//!
//! A missing duration is missing information.
//!
//! It must be resolved through the scheduler timing contract or an explicit
//! target policy rather than silently inventing a physical value.
//!
//! # Resource identity
//!
//! Scheduler resource identities must use the canonical resource identity
//! contract owned by the appropriate IR/resource subsystem.
//!
//! Physical qubits must remain represented by:
//!
//! ```text
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! A physical qubit may also participate in a scheduler resource binding, but
//! the resource identity and physical-qubit identity remain separate semantic
//! concepts.
//!
//! # Verification boundary
//!
//! Adapters are responsible for rejecting malformed input when the malformed
//! representation could cause semantic loss.
//!
//! They must not silently "repair" ambiguous input.
//!
//! Examples include:
//!
//! - mismatched logical/physical operands;
//! - duplicate operands where forbidden;
//! - missing required identities;
//! - conflicting resource bindings;
//! - invalid scheduler-ID mappings;
//! - unsupported physical/logical identity conversions.
//!
//! The generic scheduler verification layer remains responsible for verifying
//! the complete resulting schedule.
//!
//! # Error ownership
//!
//! Adapter-specific structural errors belong to the corresponding adapter.
//!
//! They must not be replaced with stringly typed errors in this composition
//! module.
//!
//! For example:
//!
//! ```text
//! adapters::ir::IrAdapterError
//! adapters::routing::RoutingAdapterError
//! adapters::hardware::HardwareAdapterError
//! adapters::qec::QecAdapterError
//! ```
//!
//! remain distinct because they describe different failure domains.
//!
//! A higher-level scheduler error may wrap or classify those errors at the
//! orchestration boundary, but this module does not duplicate them.
//!
//! # Determinism
//!
//! This module performs no scheduling decisions itself.
//!
//! Determinism therefore depends on the concrete adapter contract.
//!
//! The adapters must not introduce hidden randomness or hidden global state.
//!
//! Where ordering is semantically significant, the source ordering must be
//! preserved.
//!
//! Where collections are exposed as deterministic snapshots, their concrete
//! adapter contract must define the ordering.
//!
//! # Thread safety
//!
//! This composition module owns no mutable state.
//!
//! The module declarations themselves require no synchronization.
//!
//! Concrete adapter types must document their own `Send`/`Sync` behavior where
//! relevant.
//!
//! Stateless adapters are preferred.
//!
//! # Serialization
//!
//! This module does not own a serialized representation.
//!
//! Adapter-specific serialized forms, if needed, belong to the owning adapter
//! or the scheduling serialization subsystem.
//!
//! No provider SDK serialization format may become the scheduler's canonical
//! serialization format.
//!
//! # Security
//!
//! This module must never contain:
//!
//! - credentials;
//! - authentication tokens;
//! - provider secrets;
//! - private keys;
//! - network clients;
//! - backend sessions;
//! - mutable credential caches.
//!
//! Hardware authentication belongs to `quantum::hardware` and its provider
//! integration layers.
//!
//! # Safety
//!
//! The scheduling subsystem is safe Rust.
//!
//! This composition boundary explicitly forbids unsafe Rust.
//!
//! Adapter implementations must satisfy the same requirement independently.
//!
//! No:
//!
//! - `unsafe` blocks;
//! - `unsafe fn`;
//! - raw-pointer execution;
//! - unchecked FFI;
//! - mutable statics
//!
//! are permitted in this namespace.
//!
//! # Rust compatibility
//!
//! Target compatibility:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! No unstable language feature is required by this module.
//!
//! # Integration with scheduling::mod
//!
//! The scheduling root should expose this namespace exactly once:
//!
//! ```text
//! pub mod adapters;
//! ```
//!
//! The scheduling root must not redeclare:
//!
//! ```text
//! pub mod ir;
//! pub mod routing;
//! pub mod hardware;
//! pub mod qec;
//! ```
//!
//! under the `adapters` namespace.
//!
//! Those are sibling scheduling modules and the adapters consume them where
//! appropriate.
//!
//! # Integration with existing adapter files
//!
//! The canonical adapter tree is:
//!
//! ```text
//! src/quantum/scheduling/adapters/
//! ├── mod.rs
//! ├── ir.rs
//! ├── routing.rs
//! ├── hardware.rs
//! └── qec.rs
//! ```
//!
//! The implementations already establish the intended separation:
//!
//! ```text
//! adapters::ir
//!     canonical Quantum IR → scheduling IR
//!
//! adapters::routing
//!     routing result → scheduling boundary
//!
//! adapters::hardware
//!     hardware target → scheduling target/resource view
//!
//! adapters::qec
//!     QEC scheduling requirements → scheduling IR
//! ```
//!
//! The existing IR adapter explicitly uses the canonical
//! `quantum::ir::qubit::QubitId` and produces a derived `SchedulingOperation`
//! rather than creating a competing quantum representation.
//!
//! The routing adapter likewise preserves the distinction between routing's
//! logical/physical identities and canonical IR identities rather than
//! pretending they are automatically interchangeable.
//!
//! The hardware adapter explicitly distinguishes backend limits from actual
//! resource inventory and retains canonical `PhysicalQubitId` values.
//!
//! The QEC adapter explicitly prevents unsafe conversion of physical QEC qubits
//! into logical scheduler qubits and prevents scheduler-operation-ID collisions.
//!
//! # Future adapter extensions
//!
//! New integration boundaries may eventually be required for:
//!
//! - ZQN/noise information;
//! - runtime;
//! - distributed execution;
//! - benchmarking;
//! - calibration snapshots;
//! - simulation;
//! - pulse-level scheduling;
//! - analog scheduling;
//! - annealing targets;
//! - photonic resource models;
//! - neutral-atom resource models;
//! - modular quantum networks.
//!
//! Such adapters should be added as independent modules when their contracts
//! are sufficiently mature.
//!
//! They should NOT require changes to unrelated adapter implementations.
//!
//! When a new adapter is introduced, only this namespace composition boundary
//! should need one new module declaration.
//!
//! Its own implementation, tests, error model, documentation, and integration
//! contract must be complete before it is exposed here.
//!
//! # Prohibited dependency direction
//!
//! The following architecture is prohibited:
//!
//! ```text
//! adapter ──X──► modify canonical IR
//! adapter ──X──► own hardware state
//! adapter ──X──► execute QPU
//! adapter ──X──► choose routing algorithm
//! adapter ──X──► choose scheduling algorithm
//! adapter ──X──► implement QEC
//! adapter ──X──► implement compiler parsing
//! adapter ──X──► own provider credentials
//! ```
//!
//! The correct direction is:
//!
//! ```text
//! source subsystem
//!       │
//!       ▼
//! scheduler adapter
//!       │
//!       ▼
//! scheduler contract
//! ```
//!
//! # Composition-root rule
//!
//! This file intentionally remains a composition root.
//!
//! It should contain only:
//!
//! - module declarations;
//! - module-level documentation;
//! - narrowly justified namespace-level attributes;
//! - tests proving namespace-level invariants.
//!
//! It should not contain implementation logic.
//!
//! This keeps the file stable while concrete adapters evolve.
//!
//! # File completion contract
//!
//! This file is complete when:
//!
//! - all canonical production scheduling adapters are declared exactly once;
//! - no adapter implementation is duplicated here;
//! - no wildcard re-export destabilizes the namespace;
//! - no canonical quantum identity is redefined;
//! - no hardware identity is redefined;
//! - no scheduling algorithm is implemented here;
//! - no routing algorithm is implemented here;
//! - no QEC algorithm is implemented here;
//! - no provider-specific API is exposed here;
//! - no credentials or mutable global state exist here;
//! - unsafe Rust is forbidden;
//! - Rust 1.97/1.97.1 compiles it;
//! - adding an implementation inside an existing adapter does not require
//!   reopening this file;
//! - adding a genuinely new adapter requires only its one module declaration
//!   here;
//! - adapter namespaces remain independently addressable.
//!
//! # Stable public namespace
//!
//! The resulting API is:
//!
//! ```text
//! crate::quantum::scheduling::adapters::ir
//! crate::quantum::scheduling::adapters::routing
//! crate::quantum::scheduling::adapters::hardware
//! crate::quantum::scheduling::adapters::qec
//! ```
//!
//! Concrete adapter types remain inside their respective modules.
//!
//! This is intentional and prevents unrelated adapter APIs from colliding.
//!
//! # End-state architecture
//!
//! ```text
//!                         Zamani source
//!                              │
//!                              ▼
//!                       quantum::frontend
//!                              │
//!                              ▼
//!                         quantum::ir
//!                              │
//!                  ┌───────────┼───────────┐
//!                  │           │           │
//!                  ▼           ▼           ▼
//!             optimization   routing      QEC
//!                  │           │           │
//!                  │           ▼           ▼
//!                  │      adapters::routing adapters::qec
//!                  │           │           │
//!                  └───────────┼───────────┘
//!                              ▼
//!                       adapters::ir
//!                              │
//!                              ▼
//!                     scheduling::ir
//!                              │
//!              ┌───────────────┼────────────────┐
//!              │               │                │
//!              ▼               ▼                ▼
//!           dependency      resources         timing
//!              │               │                │
//!              └───────────────┼────────────────┘
//!                              │
//!                              ▼
//!                           planners
//!                              │
//!                              ▼
//!                         verification
//!                              │
//!                              ▼
//!                           result
//!                              │
//!                              ▼
//!                    adapters::hardware
//!                              │
//!                              ▼
//!                      quantum::hardware
//!                              │
//!                              ▼
//!                           runtime
//! ```
//!
//! The adapter namespace is therefore a controlled integration layer rather
//! than another quantum compiler subsystem.

// =============================================================================
// Safety
// =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

// =============================================================================
// Adapter module declarations
// =============================================================================

/// Canonical Quantum IR → scheduling IR adapter.
///
/// This is the scheduler-owned boundary for projecting canonical
/// `quantum::ir` operations into scheduling-specific representations.
///
/// The canonical IR remains authoritative for quantum semantics and canonical
/// qubit identity.
pub mod ir;

/// Routing → scheduling adapter.
///
/// This preserves routing output and its logical/physical placement information
/// while keeping routing and scheduling as separate responsibilities.
pub mod routing;

/// Hardware → scheduling adapter.
///
/// This exposes authoritative hardware capabilities, resources, timing, and
/// availability to scheduling without duplicating hardware semantics or
/// provider-specific APIs.
pub mod hardware;

/// QEC → scheduling adapter.
///
/// This converts QEC scheduling requirements into generic scheduler
/// representations while preserving QEC identity and rejecting semantic-loss
/// conversions.
pub mod qec;

// =============================================================================
// Namespace-level tests
// =============================================================================

#[cfg(test)]
mod tests {
    //! Namespace-level invariants for the adapter composition boundary.
    //!
    //! Concrete adapter behavior is tested inside each adapter module. These
    //! tests deliberately remain small because this file is a composition root.

    use super::*;

    #[test]
    fn canonical_adapter_modules_are_available() {
        // The declarations themselves are the invariant being tested.
        //
        // Constructing the stateless adapters also verifies that the public
        // namespaces resolve without requiring any global initialization.
        let _ir = ir::IrAdapter::new();
        let _routing = routing::RoutingAdapter::new();
        let _qec = qec::QecAdapter::new();

        let _ = hardware::HARDWARE_ADAPTER_SCHEMA_ID;
    }

    #[test]
    fn adapter_namespace_is_composed_without_global_state() {
        // This test intentionally performs no I/O, hardware discovery,
        // authentication, scheduling, routing, QEC execution, or mutation.
        //
        // The adapter composition boundary must remain a pure namespace.
        assert_eq!(
            ir::IrAdapter::new(),
            ir::IrAdapter::new()
        );

        assert_eq!(
            routing::RoutingAdapter::new(),
            routing::RoutingAdapter::new()
        );

        assert_eq!(
            qec::QecAdapter::new(),
            qec::QecAdapter::new()
        );
    }
}
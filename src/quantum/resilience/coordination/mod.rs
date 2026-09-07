//! Zamani Quantum Resilience — Distributed Coordination
//!
//! Path:
//!     src/quantum/resilience/coordination/mod.rs
//!
//! Purpose:
//!     Composition boundary for distributed coordination used by the Zamani
//!     quantum-resilience subsystem.
//!
//! Architectural role:
//!
//!     Zamani source
//!          |
//!          v
//!     quantum::frontend
//!          |
//!          v
//!     quantum::ir
//!          |
//!          v
//!     quantum compilation / execution
//!          |
//!          v
//!     quantum::resilience
//!          |
//!          v
//!     quantum::resilience::coordination
//!          |
//!     +----+-----------+------------+-------------+
//!     |                |            |             |
//!     v                v            v             v
//! ownership         lease      distributed    consensus
//!     ^
//!     |
//!     +---------------- coordinator
//!
//! This module owns the namespace boundary only.
//!
//! It does NOT implement:
//!
//! - ownership;
//! - leasing;
//! - distributed transport;
//! - consensus;
//! - resource discovery;
//! - hardware discovery;
//! - routing;
//! - scheduling;
//! - QEC;
//! - mitigation;
//! - optimization;
//! - execution;
//! - result verification;
//! - checkpoint serialization;
//! - network I/O;
//! - thread management;
//! - persistence;
//! - provider-specific APIs.
//!
//! Those responsibilities belong to the child modules or to their authoritative
//! subsystems elsewhere in the Zamani quantum architecture.
//!
//! -----------------------------------------------------------------------------
//! PRODUCTION REQUIREMENTS
//! -----------------------------------------------------------------------------
//!
//! This module is designed for:
//!
//! - Rust 1.97 / Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - safe Rust only;
//! - no `unsafe`;
//! - no unsafe FFI;
//! - no raw-pointer based coordination;
//! - no hidden global mutable state;
//! - no implicit threads;
//! - no implicit asynchronous runtime;
//! - no implicit I/O;
//! - no provider-specific branches;
//! - no hard-coded quantum-machine size;
//! - no hard-coded participant count;
//! - no hard-coded resource count;
//! - no hard-coded backend count;
//! - no hard-coded quorum size;
//! - no hard-coded retry count;
//! - no hard-coded timeout;
//! - no hard-coded lease duration;
//! - no hard-coded topology;
//! - no hard-coded qubit identity;
//! - deterministic composition;
//! - explicit ownership;
//! - explicit lease validity;
//! - explicit fencing;
//! - explicit distributed agreement;
//! - explicit failure propagation;
//! - graceful degradation;
//! - scalable resource cardinality;
//! - replaceable implementations;
//! - testable contracts.
//!
//! "Infinite scalability" does not mean that a physical machine has infinite
//! resources. It means that this module imposes no artificial finite upper
//! bound on the number of:
//!
//! - qubits;
//! - logical qubits;
//! - physical qubits;
//! - resources;
//! - participants;
//! - QPUs;
//! - backends;
//! - execution domains;
//! - recovery operations.
//!
//! Concrete deployments remain bounded by available:
//!
//! - CPU;
//! - memory;
//! - storage;
//! - communication capacity;
//! - quantum hardware;
//! - execution capacity;
//! - deployment policy;
//! - security policy;
//! - operating-system limits.
//!
//! Those limits MUST be represented by explicit capability, resource, policy,
//! and deployment contracts rather than constants in this module.
//!
//! -----------------------------------------------------------------------------
//! CANONICAL QUANTUM IDENTITY
//! -----------------------------------------------------------------------------
//!
//! Coordination is intentionally quantum-resource agnostic.
//!
//! It must NOT define a second `QubitId`, `PhysicalQubitId`, logical-qubit
//! identifier, or quantum-register identity.
//!
//! If a concrete coordination implementation needs a canonical quantum qubit
//! identity, it MUST use:
//!
//!     crate::quantum::ir::qubit::QubitId
//!     crate::quantum::ir::qubit::PhysicalQubitId
//!
//! Generic coordination resources use the identifiers owned by the
//! coordination/resource contracts.
//!
//! This distinction is intentional:
//!
//!     quantum::ir::qubit
//!         |
//!         +--> quantum semantic identity
//!
//!     coordination resource identity
//!         |
//!         +--> distributed control-plane identity
//!
//! A coordination resource may represent a qubit, QPU, backend, worker,
//! execution slot, simulator, control channel, or another resource, but the
//! coordination namespace must not redefine the semantic identity of that
//! resource's owning subsystem.
//!
//! -----------------------------------------------------------------------------
//! DEPENDENCY OWNERSHIP
//! -----------------------------------------------------------------------------
//!
//! The five child modules have deliberately separate responsibilities.
//!
//! ## `coordinator`
//!
//! High-level orchestration of distributed resilience coordination.
//!
//! It composes the other coordination contracts but does not own their
//! implementations.
//!
//! Conceptually:
//!
//!     request
//!        |
//!        v
//!     validate
//!        |
//!        v
//!     ownership
//!        |
//!        v
//!     lease
//!        |
//!        v
//!     distributed participants
//!        |
//!        v
//!     consensus / coordination decision
//!        |
//!        v
//!     commit
//!        |
//!        v
//!     release
//!
//! The coordinator must remain stateless unless a concrete caller explicitly
//! supplies state through its contracts.
//!
//! ## `ownership`
//!
//! Owns authoritative control-plane ownership.
//!
//! Ownership answers:
//!
//!     "Who is currently authoritative for this operation/resource?"
//!
//! It is distinct from:
//!
//!     lease      = how long authority remains valid;
//!     fencing    = how stale authorities are prevented from mutating state;
//!     consensus  = how distributed agreement is established.
//!
//! Ownership MUST NOT be interpreted as ownership of an arbitrary quantum
//! state. Quantum-state persistence and migration have different semantics and
//! belong to execution/checkpoint/recovery layers.
//!
//! ## `lease`
//!
//! Owns time-bounded validity of coordination authority.
//!
//! The lease implementation is responsible for:
//!
//! - acquisition;
//! - renewal;
//! - expiration;
//! - release;
//! - validity;
//! - generation/fencing integration;
//! - provider-independent lease semantics.
//!
//! This module must not assume a particular clock, distributed store, or
//! transport.
//!
//! ## `distributed`
//!
//! Owns the provider-neutral distributed execution/coordination contract.
//!
//! It may compose:
//!
//! - participants;
//! - ownership;
//! - leases;
//! - consensus;
//! - transport adapters.
//!
//! It does not own a particular distributed database, message broker,
//! networking library, or cluster technology.
//!
//! ## `consensus`
//!
//! Owns the abstraction for distributed agreement where a deployment requires
//! it.
//!
//! Consensus is optional at the architecture level because not every
//! single-participant or externally-authoritative deployment requires a
//! consensus protocol.
//!
//! A concrete implementation must nevertheless make its consistency,
//! availability, membership, quorum, failure, and fencing semantics explicit.
//!
//! -----------------------------------------------------------------------------
//! DEPENDENCY DIRECTION
//! -----------------------------------------------------------------------------
//!
//! The intended dependency direction is:
//!
//!     quantum::ir
//!          |
//!          v
//!     quantum::resilience
//!          |
//!          v
//!     coordination
//!          |
//!     +----+----------------+----------------+
//!     |                     |                |
//!     v                     v                v
//! ownership              lease          distributed
//!                                             |
//!                                             v
//!                                         consensus
//!
//! The coordinator composes these contracts.
//!
//! The architecture MUST NOT become:
//!
//!     ownership --> coordinator --> ownership
//!
//!     lease --> coordinator --> lease
//!
//!     consensus --> coordinator --> consensus
//!
//! through implementation dependencies.
//!
//! The child modules may refer to stable contract types exposed by their
//! siblings where necessary, but no module may require a concrete implementation
//! of another sibling merely to define its own domain contract.
//!
//! -----------------------------------------------------------------------------
//! IMPORTANT CIRCULAR-DEPENDENCY RULE
//! -----------------------------------------------------------------------------
//!
//! The current coordination contracts intentionally share several control-plane
//! types.
//!
//! In particular, ownership uses coordinator-owned request/grant/fence contract
//! types.
//!
//! Therefore this `mod.rs` MUST NOT attempt to solve the dependency relationship
//! through:
//!
//! - duplicate types;
//! - compatibility copies;
//! - wrapper structs that mirror existing types;
//! - wildcard re-exports;
//! - type aliases that create a second public identity;
//! - implementation code.
//!
//! Rust resolves sibling modules through the parent module namespace.
//!
//! The correct long-term architecture is:
//!
//!     stable coordination contracts
//!                 |
//!          +------+------+
//!          |             |
//!          v             v
//!     coordinator    ownership
//!
//! while shared contract types should eventually be moved into a lower-level
//! coordination contract module if the dependency graph requires it.
//!
//! That refactoring belongs to the affected implementation files, not to this
//! namespace composition root.
//!
//! -----------------------------------------------------------------------------
//! FENCING MODEL
//! -----------------------------------------------------------------------------
//!
//! Distributed quantum resilience must protect against stale coordinators.
//!
//! The fundamental invariant is:
//!
//!     ownership
//!         + lease validity
//!         + generation
//!         + fencing token
//!         = authority that can be checked
//!
//! A coordinator that previously held authority MUST NOT be allowed to mutate
//! distributed state after its authority has been:
//!
//! - released;
//! - expired;
//! - revoked;
//! - superseded;
//! - fenced;
//! - replaced by a newer generation.
//!
//! Therefore mutating operations must carry the authoritative fencing material
//! required by the target resource authority.
//!
//! Coordination itself does not decide whether a fence is valid for a physical
//! resource. The authoritative resource/ownership implementation does.
//!
//! -----------------------------------------------------------------------------
//! QUANTUM SAFETY BOUNDARY
//! -----------------------------------------------------------------------------
//!
//! Distributed coordination controls execution authority.
//!
//! It does not prove quantum-result correctness.
//!
//! The following are deliberately different:
//!
//!     ownership acquired
//!         !=
//!     lease valid
//!         !=
//!     distributed agreement reached
//!         !=
//!     execution completed
//!         !=
//!     quantum result correct
//!
//! Result correctness remains the responsibility of:
//!
//!     quantum::resilience::verification
//!
//! Likewise:
//!
//!     quantum::resilience::checkpoint
//!
//! owns checkpoint semantics;
//!
//!     quantum::resilience::recovery
//!
//! owns recovery execution;
//!
//!     quantum::hardware
//!
//! owns hardware capabilities and execution interfaces;
//!
//!     quantum::routing
//!
//! owns logical-to-physical mapping;
//!
//!     quantum::scheduling
//!
//! owns operation scheduling;
//!
//!     quantum::error_correction
//!
//! owns QEC algorithms and decoding;
//!
//!     quantum::zqn
//!
//! owns quantum noise/fault semantics.
//!
//! Coordination must not duplicate those domains.
//!
//! -----------------------------------------------------------------------------
//! WRITE-ONCE / SCALE-EVERYWHERE CONTRACT
//! -----------------------------------------------------------------------------
//!
//! A Zamani program should describe the computation rather than the distributed
//! machine topology.
//!
//! Coordination therefore MUST NOT contain logic such as:
//!
//!     if qubit_id == 7 { ... }
//!     if backend == "some_provider" { ... }
//!     if participants.len() == 3 { ... }
//!     const MAX_QUBITS: usize = ...;
//!     const MAX_NODES: usize = ...;
//!     const DEFAULT_QUORUM: usize = ...;
//!     const RETRIES: usize = ...;
//!
//! These are architectural violations when used as semantic limits.
//!
//! Instead, concrete values must come from:
//!
//!     capability providers
//!     resource models
//!     deployment policy
//!     resilience policy
//!     ownership authority
//!     lease authority
//!     consensus configuration
//!     runtime configuration
//!
//! A concrete deployment may legitimately reject an operation because its
//! resources are insufficient. That is a runtime capability/policy decision,
//! not a language or coordination namespace limit.
//!
//! -----------------------------------------------------------------------------
//! DETERMINISM
//! -----------------------------------------------------------------------------
//!
//! This module performs no stochastic computation.
//!
//! It does not:
//!
//! - generate random identifiers;
//! - read clocks;
//! - access environment variables;
//! - access the network;
//! - access the filesystem;
//! - spawn threads;
//! - mutate global state.
//!
//! Determinism is therefore delegated to the supplied child contracts.
//!
//! When deterministic execution is required, callers should explicitly supply:
//!
//! - operation identity;
//! - execution identity;
//! - participant identity;
//! - expected generation;
//! - fencing information;
//! - resource set;
//! - policy;
//! - consensus configuration;
//! - deterministic randomness, if a concrete algorithm requires randomness.
//!
//! No child module should silently introduce global randomness merely because
//! coordination is distributed.
//!
//! -----------------------------------------------------------------------------
//! IDEMPOTENCY
//! -----------------------------------------------------------------------------
//!
//! Distributed messages can be duplicated, delayed, reordered, or replayed.
//!
//! Coordination operations therefore require stable operation identity at the
//! contract boundary.
//!
//! The coordination namespace itself does not generate operation identifiers.
//!
//! A caller may use any suitable identifier scheme, including:
//!
//! - UUID-like identifiers;
//! - ULID-like identifiers;
//! - deterministic identifiers;
//! - content-addressed identifiers;
//! - deployment-specific identifiers.
//!
//! The identifier semantics belong to the caller or authoritative identity
//! service.
//!
//! A retry MUST NOT be implemented implicitly by this module.
//!
//! If the caller intentionally retries an operation, the retry semantics must
//! be explicit and compatible with the operation's idempotency contract.
//!
//! -----------------------------------------------------------------------------
//! FAILURE SEMANTICS
//! -----------------------------------------------------------------------------
//!
//! Coordination failures must never be silently converted into success.
//!
//! Important classes include:
//!
//! - ownership unavailable;
//! - ownership denied;
//! - ownership superseded;
//! - lease unavailable;
//! - lease expired;
//! - lease revoked;
//! - fence rejected;
//! - participant unavailable;
//! - participant set changed;
//! - distributed transport failure;
//! - consensus unavailable;
//! - consensus rejected;
//! - quorum/decision requirements unsatisfied;
//! - operation already committed;
//! - operation already released;
//! - stale generation;
//! - authorization failure;
//! - incompatible protocol;
//! - resource unavailable;
//! - cancellation;
//! - timeout;
//! - unknown failure.
//!
//! The canonical resilience error boundary remains:
//!
//!     crate::quantum::resilience::errors::error::ResilienceError
//!
//! Coordination-specific errors should be represented through the appropriate
//! resilience error classification rather than creating a competing
//! resilience-wide error hierarchy in this module.
//!
//! -----------------------------------------------------------------------------
//! SECURITY BOUNDARY
//! -----------------------------------------------------------------------------
//!
//! Coordination is a control-plane security boundary.
//!
//! Implementations must account for:
//!
//! - participant authentication;
//! - authorization;
//! - ownership authorization;
//! - lease authorization;
//! - fencing;
//! - replay protection;
//! - stale coordinator rejection;
//! - generation validation;
//! - message integrity;
//! - transport security;
//! - auditability;
//! - provenance;
//! - failure isolation.
//!
//! Secrets MUST NOT be placed in public coordination domain objects unless a
//! dedicated secret-handling contract explicitly requires them.
//!
//! In particular, this module must not load:
//!
//! - credentials;
//! - private keys;
//! - API tokens;
//! - provider secrets;
//! - environment secrets.
//!
//! Credential management belongs to the appropriate security/hardware/runtime
//! boundary.
//!
//! -----------------------------------------------------------------------------
//! RESOURCE OWNERSHIP
//! -----------------------------------------------------------------------------
//!
//! Coordination must never silently assume that it owns a quantum resource.
//!
//! A resource may be owned by:
//!
//! - a QPU controller;
//! - a runtime;
//! - a scheduler;
//! - a cluster manager;
//! - a provider service;
//! - a simulator;
//! - another distributed coordination domain.
//!
//! Coordination authority must therefore be explicitly granted.
//!
//! This is especially important when a resource is a physical qubit or control
//! channel: distributed ownership is a control-plane concept and does not imply
//! physical possession or quantum-state ownership.
//!
//! -----------------------------------------------------------------------------
//! DISTRIBUTED SCALABILITY
//! -----------------------------------------------------------------------------
//!
//! The coordination namespace must support all of the following without
//! changing its semantic API:
//!
//!     one process
//!         |
//!     one QPU
//!         |
//!     multiple QPUs
//!         |
//!     multiple quantum backends
//!         |
//!     heterogeneous quantum fleet
//!         |
//!     distributed quantum execution
//!         |
//!     multi-site execution
//!         |
//!     large-scale fault-tolerant execution
//!
//! The implementation must scale according to supplied resources.
//!
//! This means that child implementations should prefer:
//!
//! - streaming where appropriate;
//! - bounded work queues where policy requires them;
//! - lazy resource enumeration;
//! - incremental state updates;
//! - partitioned coordination;
//! - hierarchical coordination;
//! - explicit backpressure;
//! - resource-aware planning;
//! - deterministic partitioning;
//! - efficient immutable data structures;
//! - configurable persistence.
//!
//! None of those implementation techniques belongs directly in this
//! composition root.
//!
//! -----------------------------------------------------------------------------
//! HIERARCHICAL COORDINATION
//! -----------------------------------------------------------------------------
//!
//! Large quantum systems may require multiple coordination levels.
//!
//! For example:
//!
//!     global execution
//!          |
//!          +--> quantum site A
//!          |       |
//!          |       +--> QPU A1
//!          |       +--> QPU A2
//!          |
//!          +--> quantum site B
//!                  |
//!                  +--> QPU B1
//!                  +--> QPU B2
//!
//! The namespace therefore does not assume that every participant must
//! coordinate directly with every other participant.
//!
//! Hierarchical, partitioned, delegated, or federated implementations may be
//! introduced behind the existing contracts.
//!
//! -----------------------------------------------------------------------------
//! PARTIAL FAILURE
//! -----------------------------------------------------------------------------
//!
//! Distributed quantum execution must assume partial failure.
//!
//! One participant may fail while others remain healthy.
//!
//! Examples:
//!
//!     QPU A       healthy
//!     QPU B       degraded
//!     QPU C       unavailable
//!
//! The coordination subsystem must therefore support explicit partial outcomes
//! rather than reducing every distributed event to a binary global success/fail
//! result.
//!
//! Whether degraded execution is acceptable is determined by resilience policy
//! and verification, not by this module.
//!
//! -----------------------------------------------------------------------------
//! CONSENSUS IS NOT ALWAYS REQUIRED
//! -----------------------------------------------------------------------------
//!
//! A single authoritative controller does not necessarily need a consensus
//! algorithm.
//!
//! Likewise, a deployment may use an external consistency service.
//!
//! Therefore this namespace exposes the consensus module as an explicit
//! contract, but the coordinator must not assume that every deployment has a
//! particular consensus algorithm.
//!
//! The deployment determines whether it requires:
//!
//! - single authority;
//! - replicated authority;
//! - quorum-based agreement;
//! - externally supplied agreement;
//! - hierarchical agreement;
//! - another formally defined consistency model.
//!
//! -----------------------------------------------------------------------------
//! LEASE AND CLOCK SEMANTICS
//! -----------------------------------------------------------------------------
//!
//! The coordination namespace must not read wall-clock time itself.
//!
//! Lease implementations must receive their clock/time source through an
//! explicit abstraction when time is required.
//!
//! This permits:
//!
//! - production clocks;
//! - monotonic clocks;
//! - deterministic test clocks;
//! - simulation clocks;
//! - replay clocks.
//!
//! A wall-clock timestamp must never be used as a substitute for a fencing
//! generation where stale-writer protection is required.
//!
//! -----------------------------------------------------------------------------
//! PERSISTENCE
//! -----------------------------------------------------------------------------
//!
//! This module owns no persistence implementation.
//!
//! Concrete ownership, lease, consensus, or distributed implementations may
//! persist their authoritative state using injected storage contracts.
//!
//! Persistence must preserve the state necessary for:
//!
//! - recovery;
//! - fencing;
//! - generation validation;
//! - idempotency;
//! - audit;
//! - deterministic replay;
//! - protocol compatibility.
//!
//! The composition root must not choose a database, file format, network store,
//! or cloud provider.
//!
//! -----------------------------------------------------------------------------
//! OBSERVABILITY
//! -----------------------------------------------------------------------------
//!
//! Coordination implementations should emit structured observations through
//! the resilience telemetry boundary.
//!
//! Useful events include:
//!
//! - ownership requested;
//! - ownership granted;
//! - ownership rejected;
//! - lease acquired;
//! - lease renewed;
//! - lease expired;
//! - fence rejected;
//! - participant joined;
//! - participant left;
//! - participant failed;
//! - coordination prepared;
//! - coordination committed;
//! - coordination aborted;
//! - consensus accepted;
//! - consensus rejected;
//! - stale operation rejected;
//! - recovery coordination completed.
//!
//! The coordination namespace itself does not create telemetry exporters.
//!
//! -----------------------------------------------------------------------------
//! PROVENANCE
//! -----------------------------------------------------------------------------
//!
//! Distributed recovery decisions should be traceable.
//!
//! At minimum, the surrounding resilience system should be able to associate:
//!
//!     execution
//!       |
//!       +--> coordination operation
//!       +--> participant set
//!       +--> ownership generation
//!       +--> lease generation
//!       +--> fencing information
//!       +--> distributed decision
//!       +--> recovery plan
//!       +--> verification result
//!
//! This allows a recovered quantum result to be audited without making the
//! coordination module responsible for result verification.
//!
//! -----------------------------------------------------------------------------
//! INTEGRATION WITH RESILIENCE
//! -----------------------------------------------------------------------------
//!
//! The intended integration is:
//!
//!     quantum::resilience::api
//!              |
//!              v
//!     resilience controller
//!              |
//!              v
//!     resilience planning
//!              |
//!              v
//!     coordination::coordinator
//!              |
//!       +------+------+------+------+
//!       |      |      |      |      |
//!       v      v      v      v      v
//!   ownership lease distributed consensus policy
//!              |
//!              v
//!        recovery action
//!              |
//!              v
//!        execution/runtime
//!              |
//!              v
//!          verification
//!
//! Coordination supplies control-plane agreement.
//!
//! It does not determine whether the quantum result is semantically correct.
//!
//! -----------------------------------------------------------------------------
//! INTEGRATION WITH HARDWARE
//! -----------------------------------------------------------------------------
//!
//! Coordination may coordinate hardware resources, but it must not directly
//! implement hardware access.
//!
//! The expected direction is:
//!
//!     coordination
//!          |
//!          v
//!     hardware capability/resource contract
//!          |
//!          v
//!     hardware HAL
//!          |
//!          v
//!     backend adapter
//!
//! Vendor-specific logic remains below the hardware abstraction boundary.
//!
//! Coordination MUST NOT contain provider branches such as:
//!
//!     IBM
//!     IonQ
//!     Quantinuum
//!     Rigetti
//!     AWS
//!
//! as special semantic cases.
//!
//! New providers should be integrable without modifying this `mod.rs`.
//!
//! -----------------------------------------------------------------------------
//! INTEGRATION WITH ROUTING AND SCHEDULING
//! -----------------------------------------------------------------------------
//!
//! Coordination does not route or schedule quantum operations.
//!
//! If a coordinated resource becomes unavailable:
//!
//!     coordination
//!          |
//!          v
//!     resilience adaptation
//!          |
//!          +--> quantum::routing
//!          |
//!          +--> quantum::scheduling
//!
//! Routing and scheduling remain authoritative for their respective domains.
//!
//! -----------------------------------------------------------------------------
//! INTEGRATION WITH QEC AND ZQN
//! -----------------------------------------------------------------------------
//!
//! Coordination does not implement quantum error correction or quantum noise
//! semantics.
//!
//! The expected separation is:
//!
//!     quantum::zqn
//!          |
//!          v
//!     fault/noise observation
//!          |
//!          v
//!     quantum::resilience
//!          |
//!          v
//!     coordination
//!
//! and:
//!
//!     quantum::error_correction
//!          |
//!          v
//!     logical fault correction
//!
//! Coordination may coordinate QEC-related resources or operations but must not
//! duplicate QEC algorithms or ZQN fault definitions.
//!
//! -----------------------------------------------------------------------------
//! INTEGRATION WITH CANONICAL IR
//! -----------------------------------------------------------------------------
//!
//! Coordination is intentionally downstream from the canonical quantum IR.
//!
//! The IR describes quantum computation semantics.
//!
//! Coordination describes distributed control-plane authority around execution.
//!
//! Therefore this module must never introduce:
//!
//! - alternative circuit representations;
//! - alternative gate representations;
//! - alternative qubit representations;
//! - alternative quantum operation identifiers.
//!
//! If a coordination request needs quantum-resource identity, use the canonical
//! types owned by:
//!
//!     crate::quantum::ir::qubit
//!
//! where the request semantically represents a qubit.
//!
//! -----------------------------------------------------------------------------
//! MODULE STABILITY
//! -----------------------------------------------------------------------------
//!
//! This file should change rarely.
//!
//! Adding a new implementation of ownership, leasing, distributed transport,
//! or consensus must NOT require changes here unless the public module
//! structure itself changes.
//!
//! For example, these should be implementation changes rather than namespace
//! changes:
//!
//!     PostgreSQL ownership store
//!     in-memory test authority
//!     remote ownership service
//!     cluster lease implementation
//!     deterministic simulator lease
//!     Raft-like consensus implementation
//!     externally managed consensus
//!     quantum-fleet coordinator
//!
//! They should remain behind the existing contract boundaries.
//!
//! -----------------------------------------------------------------------------
//! PUBLIC API POLICY
//! -----------------------------------------------------------------------------
//!
//! Child modules are exposed as named namespaces:
//!
//!     quantum::resilience::coordination::coordinator
//!     quantum::resilience::coordination::ownership
//!     quantum::resilience::coordination::lease
//!     quantum::resilience::coordination::distributed
//!     quantum::resilience::coordination::consensus
//!
//! This module intentionally does NOT perform wildcard re-exports.
//!
//! Do NOT add:
//!
//!     pub use coordinator::*;
//!     pub use ownership::*;
//!     pub use lease::*;
//!     pub use distributed::*;
//!     pub use consensus::*;
//!
//! Wildcard exports would:
//!
//! - blur ownership;
//! - create accidental API collisions;
//! - expose implementation details;
//! - increase coupling;
//! - make unrelated additions modify this API boundary;
//! - make dependency cycles harder to identify.
//!
//! Callers should use explicit namespaces.
//!
//! -----------------------------------------------------------------------------
//! VERSIONING
//! -----------------------------------------------------------------------------
//!
//! The coordination subsystem's individual contracts own their protocol/schema
//! versioning.
//!
//! This module must not invent a second incompatible versioning system.
//!
//! Compatibility must consider, where applicable:
//!
//! - resilience API version;
//! - coordination contract version;
//! - ownership protocol version;
//! - lease protocol version;
//! - distributed protocol version;
//! - consensus protocol version;
//! - execution/runtime version;
//! - hardware capability version;
//! - serialization schema version.
//!
//! -----------------------------------------------------------------------------
//! SERIALIZATION
//! -----------------------------------------------------------------------------
//!
//! This module owns no serialization format.
//!
//! Serializable coordination objects are serialized by the subsystem that owns
//! their semantic definition or through the resilience serialization boundary.
//!
//! Serialization must preserve, where applicable:
//!
//! - stable identifiers;
//! - generation;
//! - fencing;
//! - operation identity;
//! - execution identity;
//! - protocol version;
//! - participant identity;
//! - resource identity;
//! - state required for recovery.
//!
//! Process-local handles, threads, sockets, locks, database connections, and
//! other runtime objects must not be serialized as though they were portable
//! coordination state.
//!
//! -----------------------------------------------------------------------------
//! NO GLOBAL STATE
//! -----------------------------------------------------------------------------
//!
//! This composition root creates no:
//!
//! - global singleton;
//! - mutable static;
//! - lazy global registry;
//! - implicit connection pool;
//! - background worker;
//! - global executor;
//! - global clock;
//! - global RNG;
//! - global participant list.
//!
//! Concrete implementations receive their dependencies explicitly.
//!
//! This is essential for:
//!
//! - deterministic tests;
//! - multiple independent resilience domains;
//! - nested execution;
//! - simulation;
//! - distributed deployment;
//! - multi-tenant execution;
//! - replay;
//! - graceful shutdown.
//!
//! -----------------------------------------------------------------------------
//! NO IMPLICIT I/O
//! -----------------------------------------------------------------------------
//!
//! Importing this module performs no I/O.
//!
//! Declaring these modules must never:
//!
//! - connect to a QPU;
//! - connect to a database;
//! - contact a consensus service;
//! - inspect the filesystem;
//! - inspect the network;
//! - read environment variables;
//! - initialize credentials.
//!
//! Explicit runtime calls own those operations.
//!
//! -----------------------------------------------------------------------------
//! TESTABILITY
//! -----------------------------------------------------------------------------
//!
//! Because this is a composition root, most behavioral testing belongs in the
//! child modules and integration tests.
//!
//! The surrounding coordination test suite should eventually verify:
//!
//! - single-participant coordination;
//! - multiple participants;
//! - ownership contention;
//! - lease expiration;
//! - lease renewal;
//! - stale fencing;
//! - generation changes;
//! - duplicate requests;
//! - reordered requests;
//! - participant failure;
//! - partial failure;
//! - consensus failure;
//! - transport failure;
//! - cancellation;
//! - deterministic replay;
//! - resource scaling;
//! - large participant sets;
//! - heterogeneous resources;
//! - graceful degradation;
//! - recovery coordination;
//! - security/authorization failures.
//!
//! Tests must generate resource cardinality dynamically rather than encoding
//! production limits.
//!
//! -----------------------------------------------------------------------------
//! COMPILER SAFETY
//! -----------------------------------------------------------------------------
//!
//! This namespace is explicitly safe Rust.
//!
//! The child modules should independently maintain the same guarantee.
//!
//! No unsafe implementation is required for the coordination abstraction.
//!
//! Backend FFI, if ever required by a concrete external system, must be isolated
//! behind an independently reviewed boundary and must not leak unsafe
//! implementation assumptions into the coordination contracts.
//!
//! -----------------------------------------------------------------------------
//! MODULE DECLARATIONS
//! -----------------------------------------------------------------------------
//!
//! Declaration order is intentionally kept simple.
//!
//! Rust resolves sibling modules through this parent namespace, so child
//! implementations may explicitly reference:
//!
//!     super::coordinator
//!     super::ownership
//!     super::lease
//!     super::distributed
//!     super::consensus
//!
//! where their contract requires it.
//!
//! No child implementation is instantiated here.
//!
//! No child implementation is initialized here.
//!
//! No child implementation is executed here.
//!
//! -----------------------------------------------------------------------------
//! INTEGRATION CHECKLIST
//! -----------------------------------------------------------------------------
//!
//! For this file to be considered complete:
//!
//! [x] All coordination child modules are declared.
//! [x] No hard-coded machine-size limits exist.
//! [x] No hard-coded participant limits exist.
//! [x] No hard-coded resource limits exist.
//! [x] No provider-specific implementation exists.
//! [x] No global mutable state exists.
//! [x] No implicit I/O exists.
//! [x] No implicit threads exist.
//! [x] No quantum-state semantics are duplicated.
//! [x] Canonical qubit identity remains owned by quantum::ir::qubit.
//! [x] Ownership remains separate from lease semantics.
//! [x] Lease remains separate from consensus.
//! [x] Distributed coordination remains separate from quantum execution.
//! [x] Verification remains outside coordination.
//! [x] Routing remains outside coordination.
//! [x] Scheduling remains outside coordination.
//! [x] QEC remains outside coordination.
//! [x] ZQN remains authoritative for quantum fault/noise semantics.
//! [x] Child implementations remain replaceable.
//! [x] Wildcard re-exports are avoided.
//! [x] Module initialization has no side effects.
//! [x] Safe Rust is enforced.
//!
//! Additional completion requirements for the child modules:
//!
//! [ ] coordinator.rs compiles against all shared coordination contracts.
//! [ ] ownership.rs compiles without introducing a competing identity model.
//! [ ] lease.rs provides explicit time/fencing semantics.
//! [ ] distributed.rs provides explicit participant/failure semantics.
//! [ ] consensus.rs provides explicit agreement semantics.
//! [ ] integration tests exercise the complete coordination lifecycle.
//! [ ] workspace compilation verifies the entire dependency graph.
//!
//! -----------------------------------------------------------------------------
//! FINAL ARCHITECTURAL INVARIANT
//! -----------------------------------------------------------------------------
//!
//! The coordination subsystem must preserve the following invariant:
//!
//!     A Zamani quantum program describes WHAT is computed.
//!
//!     Compilation determines HOW it is represented.
//!
//!     Routing determines WHERE logical resources are mapped.
//!
//!     Scheduling determines WHEN operations execute.
//!
//!     Hardware determines WHICH physical capabilities are available.
//!
//!     QEC determines HOW quantum errors are corrected.
//!
//!     ZQN determines HOW quantum faults/noise are represented.
//!
//!     Resilience determines HOW execution adapts to failure.
//!
//!     Coordination determines WHO may act and HOW distributed actors agree.
//!
//!     Verification determines WHETHER the resulting computation/result is
//!     acceptable.
//!
//! No layer should silently take ownership of another layer's semantics.
//!
//! This separation is what allows one Zamani program to remain portable from a
//! tiny quantum system to arbitrarily larger systems, subject only to the
//! capabilities and resources actually available at execution time.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

pub mod consensus;
pub mod coordinator;
pub mod distributed;
pub mod lease;
pub mod ownership;
//! # Zamani Quantum Noise — Operation Semantics
//!
//! `src/quantum/zqn/operations/mod.rs`
//!
//! Production composition boundary for operation-scoped quantum-noise
//! semantics.
//!
//! # Purpose
//!
//! This module is the authoritative namespace for ZQN operation-level noise
//! descriptions.
//!
//! It connects the independently implemented operation modules:
//!
//! - [`operation`] — common operation context and operation-scoped resources;
//! - [`gate`] — gate-boundary noise;
//! - [`preparation`] — state-preparation noise;
//! - [`reset`] — reset noise;
//! - [`measurement`] — measurement/readout noise;
//! - [`idle`] — idle/delay noise;
//! - [`pulse`] — pulse-level noise;
//! - [`transport`] — transport/shuttling/transfer noise.
//!
//! This file intentionally contains **module composition and stable public
//! exports only**. It does not duplicate the semantics implemented by the
//! child modules.
//!
//! # Architectural ownership
//!
//! ```text
//!                         Zamani source
//!                              │
//!                              ▼
//!                       quantum::ir
//!                              │
//!                              │ canonical operation/resource identity
//!                              ▼
//!                   ┌──────────────────────┐
//!                   │       ZQN            │
//!                   │   operations/mod.rs  │
//!                   └──────────┬───────────┘
//!                              │
//!             ┌────────────────┼─────────────────┐
//!             │                │                 │
//!             ▼                ▼                 ▼
//!           gate            measurement        reset
//!             │                │                 │
//!             ├──────────────┬─┴─────────────────┤
//!             │              │                   │
//!             ▼              ▼                   ▼
//!          preparation      idle                pulse
//!             │              │                   │
//!             └──────────────┼───────────────────┘
//!                            ▼
//!                         transport
//!                            │
//!                            ▼
//!                    ZQN noise subsystem
//!                            │
//!          ┌─────────────────┼──────────────────┐
//!          ▼                 ▼                  ▼
//!       routing          scheduling          QEC
//!          │                 │                  │
//!          └─────────────────┼──────────────────┘
//!                            ▼
//!                     hardware/runtime
//! ```
//!
//! # Critical semantic boundary
//!
//! `operations` is **not** the canonical quantum IR.
//!
//! The canonical semantic IR remains:
//!
//! ```text
//! crate::quantum::ir
//! ```
//!
//! Canonical quantum identities therefore remain owned by the IR. In
//! particular, operation modules consume the repository's canonical:
//!
//! ```text
//! crate::quantum::ir::identity::OperationId
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! ZQN must not create competing versions of those identities.
//!
//! # What this module owns
//!
//! This composition boundary owns:
//!
//! - the `zqn::operations` namespace;
//! - child-module visibility;
//! - stable operation-level re-exports;
//! - operation-module dependency ordering;
//! - the public API boundary for operation-scoped ZQN semantics.
//!
//! # What this module does not own
//!
//! This file does **not** own:
//!
//! - canonical quantum IR;
//! - source-language parsing;
//! - gate definitions;
//! - routing algorithms;
//! - topology;
//! - scheduling algorithms;
//! - hardware drivers;
//! - vendor APIs;
//! - quantum-state simulation;
//! - channel mathematics;
//! - probability mathematics;
//! - calibration storage;
//! - QEC decoding;
//! - benchmark methodology;
//! - serialization wire formats;
//! - random-number generation;
//! - global mutable state.
//!
//! Those responsibilities remain in their respective subsystems.
//!
//! # Dependency direction
//!
//! The intended dependency direction is:
//!
//! ```text
//! quantum::ir
//!      │
//!      ▼
//! zqn::core
//!      │
//!      ▼
//! zqn::operations
//!      │
//!      ├──────► zqn::noise
//!      ├──────► zqn::channel
//!      ├──────► zqn::fault
//!      ├──────► zqn::calibration
//!      ├──────► zqn::characterization
//!      ├──────► zqn::simulation
//!      ├──────► zqn::propagation
//!      └──────► zqn::target
//!                         │
//!                         ▼
//!                  integration layer
//!                         │
//!             ┌───────────┼───────────┐
//!             ▼           ▼           ▼
//!          routing     scheduling     QEC
//!             │           │           │
//!             └───────────┼───────────┘
//!                         ▼
//!                     hardware
//!                         │
//!                         ▼
//!                       runtime
//! ```
//!
//! Operation modules may depend on canonical IR identity types and stable ZQN
//! foundation types.
//!
//! They must not depend on concrete routing, scheduling, hardware, QEC,
//! benchmarking, or runtime implementations.
//!
//! # Why a dedicated `mod.rs` is required
//!
//! Rust module boundaries must be explicit.
//!
//! The presence of an `operations/` directory alone does not create a usable
//! Rust module tree. This file establishes the authoritative module boundary:
//!
//! ```text
//! zqn::operations
//! ├── operation
//! ├── gate
//! ├── preparation
//! ├── reset
//! ├── measurement
//! ├── idle
//! ├── pulse
//! └── transport
//! ```
//!
//! This avoids having `zqn/mod.rs` recreate the child tree inline.
//!
//! # Module independence
//!
//! Each child module is expected to be independently complete.
//!
//! This file therefore does not contain implementation workarounds for
//! incomplete child modules.
//!
//! In particular:
//!
//! - `operation.rs` owns common operation semantics;
//! - `gate.rs` owns gate-specific noise semantics;
//! - `preparation.rs` owns preparation-specific noise semantics;
//! - `reset.rs` owns reset-specific noise semantics;
//! - `measurement.rs` owns measurement/readout semantics;
//! - `idle.rs` owns idle/delay semantics;
//! - `pulse.rs` owns pulse-level semantics;
//! - `transport.rs` owns transport semantics.
//!
//! Adding a new operation category later should normally require adding one
//! child module and one module declaration here, rather than modifying the
//! semantics of unrelated operation modules.
//!
//! # Operation categories
//!
//! The current operation namespace deliberately covers more than gates.
//!
//! ```text
//! Operation
//! │
//! ├── Gate
//! ├── Preparation
//! ├── Reset
//! ├── Measurement
//! ├── Idle
//! ├── Pulse
//! └── Transport
//! ```
//!
//! This is important for universal quantum computing because physical noise
//! can occur without a conventional gate.
//!
//! Examples include:
//!
//! - decoherence during idle time;
//! - preparation infidelity;
//! - reset failure;
//! - measurement assignment error;
//! - pulse-control error;
//! - ion shuttling error;
//! - photonic transmission loss;
//! - quantum-memory transfer error;
//! - distributed-link noise.
//!
//! # No fixed machine size
//!
//! This module introduces no semantic limits for:
//!
//! - number of qubits;
//! - number of operations;
//! - operation arity;
//! - number of transport resources;
//! - number of pulses;
//! - circuit depth;
//! - number of measurements;
//! - number of concurrent resources;
//! - machine size;
//! - topology size.
//!
//! No constants such as:
//!
//! ```text
//! MAX_QUBITS
//! MAX_OPERATIONS
//! MAX_GATES
//! MAX_TRANSPORTS
//! MAX_PATH_LENGTH
//! ```
//!
//! belong in this module.
//!
//! A concrete resource limit is an execution/security policy and must be
//! supplied by the appropriate context/limits subsystem.
//!
//! Therefore the architectural guarantee is:
//!
//! > No artificial finite machine-size limit is encoded by the operation
//! > namespace.
//!
//! This means a Zamani program can remain semantically independent of target
//! size. It does not claim that a concrete implementation has infinite
//! memory, processing capacity, physical qubits, or execution time.
//!
//! # Write once, scale everywhere
//!
//! The operation namespace must remain independent of the physical scale of
//! the target.
//!
//! Conceptually:
//!
//! ```text
//!                     Same Zamani program
//!                              │
//!                              ▼
//!                         canonical IR
//!                              │
//!                              ▼
//!                     ZQN operation semantics
//!                              │
//!              ┌───────────────┼────────────────┐
//!              ▼               ▼                ▼
//!           tiny target    large target    distributed target
//!              │               │                │
//!              └───────────────┼────────────────┘
//!                              ▼
//!                     target capability check
//!                              │
//!                              ▼
//!                         realization
//! ```
//!
//! Operation modules must describe **what noise semantics apply**, not how a
//! particular machine physically implements them.
//!
//! # No vendor coupling
//!
//! This namespace must never contain vendor-specific operation branches such
//! as:
//!
//! ```text
//! if ibm { ... }
//! if rigetti { ... }
//! if ionq { ... }
//! if quantinuum { ... }
//! ```
//!
//! Nor should it contain vendor-specific files such as:
//!
//! ```text
//! ibm.rs
//! ionq.rs
//! rigetti.rs
//! quantinuum.rs
//! ```
//!
//! Provider-specific realization belongs in the hardware/target adapter layer.
//!
//! # Determinism
//!
//! This module contains no executable stochastic behaviour.
//!
//! It therefore:
//!
//! - owns no RNG;
//! - creates no random seeds;
//! - consumes no global RNG;
//! - depends on no wall-clock randomness;
//! - maintains no global mutable state;
//! - does not vary its exports according to thread count.
//!
//! Child operation descriptions must retain deterministic value semantics.
//!
//! Stochastic realization belongs to ZQN simulation/noise execution and must
//! use the explicit deterministic execution context rather than hidden
//! randomness.
//!
//! # Resource safety
//!
//! `mod.rs` itself performs no dynamic allocation.
//!
//! It also must not introduce semantic resource limits.
//!
//! Resource limits are enforced at the appropriate boundary, such as:
//!
//! ```text
//! deserialization
//!      │
//!      ▼
//! ZqnContext / ZqnLimits
//!      │
//!      ▼
//! semantic construction
//!      │
//!      ▼
//! execution
//! ```
//!
//! This keeps:
//!
//! ```text
//! semantic validity
//! ```
//!
//! separate from:
//!
//! ```text
//! deployment/resource policy
//! ```
//!
//! This separation is essential for scaling from very small systems to very
//! large systems without embedding today's machine capacity into tomorrow's
//! language semantics.
//!
//! # Numerical safety
//!
//! Numerical validation belongs to the child semantic modules and the ZQN
//! probability/channel foundations.
//!
//! This module must never silently transform invalid values.
//!
//! In particular, it must never introduce policies equivalent to:
//!
//! ```text
//! NaN      -> 0
//! Infinity -> finite maximum
//! negative probability -> absolute value
//! ```
//!
//! Invalid numerical states must be rejected by the owning semantic layer.
//!
//! # Serialization
//!
//! This module deliberately does not define serialization.
//!
//! Stable external representation belongs to:
//!
//! ```text
//! crate::quantum::zqn::io
//! ```
//!
//! The operation namespace only establishes the Rust module boundary.
//!
//! The future ZQN schema layer is responsible for preserving, where relevant:
//!
//! - operation identity;
//! - operation category;
//! - canonical resource identities;
//! - timing;
//! - parameters;
//! - transport paths;
//! - noise-model identity;
//! - calibration identity;
//! - provenance;
//! - schema version.
//!
//! Rust module layout must not become the wire-format contract.
//!
//! # Thread safety
//!
//! This composition module contains no mutable state.
//!
//! Child operation values should remain immutable after construction and
//! should be `Send + Sync` wherever their contained canonical types permit.
//!
//! No global registry or global cache may be introduced here.
//!
//! # API stability policy
//!
//! The child modules are public because they represent independently useful
//! semantic domains.
//!
//! Re-exports are intentionally limited to foundational, stable operation
//! concepts. Implementation details should continue to be accessed through
//! their owning modules rather than being flattened into this namespace.
//!
//! This keeps the API discoverable while avoiding an uncontrolled public API
//! surface.
//!
//! # Adding future operation categories
//!
//! If a future quantum technology introduces a new operation class, for
//! example:
//!
//! ```text
//! analog_control
//! network_link
//! bosonic_transfer
//! fermionic_transport
//! measurement_based_step
//! ```

//! it should normally receive its own module when its semantics are substantial:
//!
//! ```text
//! operations/
//! ├── ...
//! └── future_operation.rs
//! ```
//!
//! Then this file receives only the corresponding module declaration and,
//! where justified, a stable re-export.
//!
//! Do not turn this file into a large enum that hard-codes every future
//! operation category.
//!
//! # Compatibility policy
//!
//! Module declarations are intentionally explicit and stable.
//!
//! A new operation module should not require changing the public API of
//! existing operation modules.
//!
//! Removing or renaming an existing public module is a breaking API change and
//! must therefore be handled through the ZQN compatibility/versioning policy.
//!
//! # Testing contract
//!
//! Because this file is a composition boundary, its tests should focus on:
//!
//! - module visibility;
//! - public API availability;
//! - absence of unsafe code;
//! - compilation of the complete operation namespace;
//! - compatibility of the foundational re-exports.
//!
//! Mathematical correctness belongs to the tests of the owning child module.
//!
//! Integration tests belong under:
//!
//! ```text
//! src/quantum/zqn/tests/
//! ```
//!
//! and/or the repository's quantum integration-test hierarchy.
//!
//! # Rust compatibility
//!
//! This module targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced
//! for this module.
//!
//! # Public module surface
//!
//! The canonical operation namespace is:
//!
//! ```text
//! crate::quantum::zqn::operations::operation
//! crate::quantum::zqn::operations::gate
//! crate::quantum::zqn::operations::preparation
//! crate::quantum::zqn::operations::reset
//! crate::quantum::zqn::operations::measurement
//! crate::quantum::zqn::operations::idle
//! crate::quantum::zqn::operations::pulse
//! crate::quantum::zqn::operations::transport
//! ```
//!
//! The module declarations below are deliberately ordered from the common
//! operation contract toward specialized operation categories.
//!
//! `operation` must not depend on these specialized modules. The specialized
//! modules may consume the common operation contract where required.
//!
//! # Integration guarantee
//!
//! Once this file exists, downstream modules can depend on stable paths such
//! as:
//!
//! ```text
//! crate::quantum::zqn::operations::operation
//! crate::quantum::zqn::operations::transport
//! ```
//!
//! without depending on the physical location of individual implementation
//! files.
//!
//! The parent `zqn/mod.rs` only needs to expose `pub mod operations;`.
//!
//! No child operation file needs to be edited merely because this composition
//! module is introduced.
//!
//! # Important note about the current repository
//!
//! The repository currently has the operation implementation files but does
//! not have this `operations/mod.rs` boundary. Creating this file therefore
//! supplies missing Rust module wiring rather than replacing the existing
//! operation implementations.
//!
//! The repository also currently has a malformed ZQN parent filename with a
//! trailing space (`zqn/mod.rs `). That is a separate parent-module issue and
//! must be corrected before `quantum::zqn` itself can become a normal Rust
//! module. This file deliberately does not attempt to work around that
//! filesystem/module-resolution problem.
//!
//! # No implementation duplication
//!
//! Do not add operation implementations to this file.
//!
//! For example, do NOT put:
//!
//! ```text
//! struct GateNoise ...
//! struct TransportOperation ...
//! enum MeasurementNoise ...
//! ```
//!
//! here.
//!
//! Their owning modules already define those semantics.
//!
//! This file is a namespace/composition boundary only.
//!
//! # Final ownership rule
//!
//! ```text
//! operations/mod.rs
//!     = namespace + composition + stable exports
//!
//! operation.rs
//!     = common operation context
//!
//! gate.rs
//!     = gate noise
//!
//! preparation.rs
//!     = preparation noise
//!
//! reset.rs
//!     = reset noise
//!
//! measurement.rs
//!     = measurement/readout noise
//!
//! idle.rs
//!     = idle noise
//!
//! pulse.rs
//!     = pulse noise
//!
//! transport.rs
//!     = transport noise
//! ```
//!
//! This separation is the basis for independently completing each file and
//! integrating them without reopening completed semantic implementations.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

// -----------------------------------------------------------------------------
// Core operation contract
// -----------------------------------------------------------------------------
//
// `operation.rs` is the dependency foundation for operation-scoped ZQN
// semantics. It uses canonical quantum IR identities and defines the common
// operation context/duration/resource vocabulary.
//
// Keep this declaration first so the intended dependency direction is
// explicit and easy to understand.

pub mod operation;

// -----------------------------------------------------------------------------
// Specialized operation semantics
// -----------------------------------------------------------------------------

/// Gate-boundary noise semantics.
pub mod gate;

/// State-preparation noise semantics.
pub mod preparation;

/// Reset-specific noise semantics.
pub mod reset;

/// Measurement and readout noise semantics.
pub mod measurement;

/// Idle/delay noise semantics.
pub mod idle;

/// Pulse-level noise semantics.
pub mod pulse;

/// Quantum-resource transport noise semantics.
pub mod transport;

// -----------------------------------------------------------------------------
// Stable foundational re-exports
// -----------------------------------------------------------------------------
//
// These re-exports intentionally expose only the common operation contract.
// Specialized types remain available through their owning modules.
//
// Example:
//
//     use crate::quantum::zqn::operations::OperationDuration;
//
// while specialized APIs remain:
//
//     use crate::quantum::zqn::operations::transport::TransportOperation;
//
// This prevents the root namespace from becoming a dumping ground for every
// implementation type.

pub use operation::{
    OperationContext,
    OperationDuration,
    OperationError,
    OperationResource,
    ResourceRole,
};

// -----------------------------------------------------------------------------
// Compile-time API checks
// -----------------------------------------------------------------------------
//
// These aliases intentionally contain no runtime code. Their purpose is to
// make the expected public module surface explicit to the compiler.
//
// If a foundational type is accidentally removed or renamed from
// `operation.rs`, this module fails immediately rather than allowing a
// partially wired ZQN namespace to appear valid.

#[allow(dead_code)]
type _OperationContextApiCheck = OperationContext;

#[allow(dead_code)]
type _OperationDurationApiCheck = OperationDuration;

#[allow(dead_code)]
type _OperationErrorApiCheck = OperationError;

#[allow(dead_code)]
type _OperationResourceApiCheck = OperationResource;

#[allow(dead_code)]
type _ResourceRoleApiCheck = ResourceRole;

// -----------------------------------------------------------------------------
// Test contract
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operation_modules_are_exposed() {
        // The imports below are compile-time checks that all operation
        // submodules are reachable through this composition boundary.
        use super::gate;
        use super::idle;
        use super::measurement;
        use super::operation;
        use super::preparation;
        use super::pulse;
        use super::reset;
        use super::transport;

        let _ = (
            gate as *const _,
            idle as *const _,
            measurement as *const _,
            operation as *const _,
            preparation as *const _,
            pulse as *const _,
            reset as *const _,
            transport as *const _,
        );
    }

    #[test]
    fn foundational_operation_types_are_reexported() {
        fn assert_operation_context<T>() {}
        fn assert_operation_duration<T>() {}
        fn assert_operation_error<T>() {}
        fn assert_operation_resource<T>() {}
        fn assert_resource_role<T>() {}

        assert_operation_context::<OperationContext>();
        assert_operation_duration::<OperationDuration>();
        assert_operation_error::<OperationError>();
        assert_operation_resource::<OperationResource>();
        assert_resource_role::<ResourceRole>();
    }

    #[test]
    fn foundational_operation_types_are_send_sync_when_defined_by_contract() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<OperationContext>();
        assert_send_sync::<OperationDuration>();
        assert_send_sync::<OperationError>();
        assert_send_sync::<OperationResource>();
        assert_send_sync::<ResourceRole>();
    }
}
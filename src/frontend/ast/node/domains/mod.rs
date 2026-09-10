//! # Zamani Native AST — Domains
//!
//! `src/frontend/ast/node/domains/mod.rs`
//!
//! Canonical module boundary for source-level computational-domain constructs
//! in the Zamani native AST.
//!
//! ## Architectural role
//!
//! This module exposes the source-language representation of computational
//! domains without coupling the native AST to any particular implementation,
//! hardware architecture, vendor, runtime, IR, or execution system.
//!
//! The intended compilation boundary is:
//!
//! ```text
//! Zamani source
//!     │
//!     ▼
//! lexer
//!     │
//!     ▼
//! parser
//!     │
//!     ▼
//! frontend::ast::node::domains
//!     │
//!     ▼
//! structural AST validation
//!     │
//!     ▼
//! semantic analysis
//!     │
//!     ▼
//! semantic domain model
//!     │
//!     ▼
//! ZUIR
//!     │
//!     ├── classical domain IR
//!     ├── quantum domain IR
//!     ├── HDL domain IR
//!     ├── accelerator domain IR
//!     ├── distributed domain IR
//!     └── future domain IR
//!     │
//!     ▼
//! target lowering
//!     │
//!     ▼
//! hardware / runtime / simulator / execution system
//! ```
//!
//! This module is therefore an **AST module boundary**, not a domain registry,
//! target selector, hardware abstraction, compiler backend, scheduler, router,
//! optimizer, simulator, or execution layer.
//!
//! ## POCO-REAF
//!
//! Domain information must support:
//!
//! ```text
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
//! ```
//!
//! A Zamani program expresses computational intent independently of the size
//! or technology of the eventual execution system.
//!
//! Consequently this module contains no:
//!
//! ```text
//! MAX_DOMAINS
//! MAX_DOMAIN_COUNT
//! MAX_QUBITS
//! MAX_DEVICES
//! MAX_CORES
//! MAX_ACCELERATORS
//! MAX_MACHINE_SIZE
//! ```
//!
//! There is no fixed number of supported computational domains.
//!
//! The representation must remain valid when new domains are introduced.
//!
//! ## Critical architectural invariant
//!
//! A computational domain is a **source-level semantic classification or
//! requirement**.
//!
//! It is not a hardware implementation.
//!
//! Therefore:
//!
//! ```text
//! domain identity
//!     ≠
//! target identity
//!     ≠
//! hardware identity
//!     ≠
//! backend identity
//!     ≠
//! physical resource identity
//! ```
//!
//! A source program may express that computation belongs to, requires, or is
//! associated with a domain. Semantic analysis later determines what that
//! domain means in the active language environment.
//!
//! Target selection and realization happen later.
//!
//! ## Domain neutrality
//!
//! The AST must not permanently encode a closed taxonomy such as:
//!
//! ```text
//! enum Domain {
//!     Classical,
//!     Quantum,
//!     GPU,
//!     FPGA,
//! }
//! ```
//!
//! Such a representation would make adding a new computational model an AST
//! schema change.
//!
//! Instead, domain identity is represented by an extensible namespaced
//! identity.
//!
//! Examples may include:
//!
//! ```text
//! zamani.classical
//! zamani.quantum
//! zamani.hybrid
//! zamani.distributed
//! zamani.accelerator
//! zamani.hdl
//! zamani.ai
//! future.example.domain
//! ```
//!
//! These names are examples of identifiers, not a closed list of domains
//! understood by this module.
//!
//! A future computational technology must be able to introduce a new
//! namespaced domain without changing the representation defined here.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - public module organization;
//! - stable public re-exports;
//! - module-level architectural contracts;
//! - compile-time API-surface tests;
//! - dependency-boundary documentation.
//!
//! The implementation modules own the actual AST structures:
//!
//! `domain.rs`
//! :
//! - source-level domain references/declarations;
//! - source-level domain identity usage;
//! - source-level domain metadata;
//! - local structural invariants.
//!
//! `domain_kind.rs`
//! :
//! - extensible domain-kind identity;
//! - namespace/name representation;
//! - identity validation;
//! - deterministic identity behavior.
//!
//! `domain_annotation.rs`
//! :
//! - source-level domain annotations;
//! - annotation identity;
//! - annotation values/metadata;
//! - local annotation validation.
//!
//! This module must contain **no duplicate domain implementation**.
//!
//! ## Non-ownership
//!
//! This module must never own:
//!
//! - target selection;
//! - backend selection;
//! - hardware discovery;
//! - hardware capabilities;
//! - physical device topology;
//! - quantum routing;
//! - quantum scheduling;
//! - resource allocation;
//! - qubit allocation;
//! - calibration;
//! - pulse generation;
//! - QEC implementation;
//! - noise modelling;
//! - resilience implementation;
//! - runtime execution;
//! - simulator execution;
//! - vendor SDK integration;
//! - credentials;
//! - backend queues;
//! - QIR implementation;
//! - LLVM implementation;
//! - MLIR implementation;
//! - domain-specific target IR.
//!
//! Those concerns belong to downstream layers.
//!
//! ## Relationship to resources
//!
//! Domains and resources are intentionally separate.
//!
//! ```text
//! domain   = what computational model/semantic space the computation uses
//! resource = what computational resource the computation requires
//! ```
//!
//! For example, a quantum program may use:
//!
//! ```text
//! domain:
//!     zamani.quantum
//!
//! resources:
//!     quantum resource(s)
//! ```
//!
//! The domain does not determine the number, topology, technology, or physical
//! realization of those resources.
//!
//! Resource quantities and allocation belong to the resource and semantic
//! layers.
//!
//! The existing AST resource boundary follows this same separation: source
//! resources remain abstract while allocation and physical realization happen
//! downstream. 
//!
//! ## Relationship to capabilities
//!
//! Domains and capabilities are also separate.
//!
//! ```text
//! domain     = semantic computational space
//! capability = ability required from a target
//! resource   = computational quantity required
//! ```
//!
//! For example:
//!
//! ```text
//! domain:
//!     zamani.quantum
//!
//! capability:
//!     zamani.quantum.mid_circuit_measurement
//!
//! resource:
//!     logical quantum resource
//! ```
//!
//! A domain declaration does not grant capabilities and does not imply that a
//! selected target can execute the program.
//!
//! Capability availability is determined downstream.
//!
//! This preserves the same source-vs-target separation already established by
//! the AST capability architecture. 
//!
//! ## Relationship to quantum computation
//!
//! Quantum computing is one possible computational domain, not the definition
//! of the domain subsystem.
//!
//! The domain AST must therefore not contain fields for:
//!
//! ```text
//! qubit_count
//! physical_qubit
//! topology
//! gate_set
//! pulse_set
//! coupling_map
//! calibration
//! QEC_code
//! decoder
//! backend
//! vendor
//! ```
//!
//! A quantum domain declaration may ultimately lower into the quantum
//! compilation pipeline, but the native AST remains unaware of how that
//! quantum computation will be realized.
//!
//! This is essential for the Zamani requirement that a single program can
//! scale across different computational systems.
//!
//! ## Relationship to ZUIR
//!
//! There is deliberately **no direct ZUIR dependency** here.
//!
//! The required direction is:
//!
//! ```text
//! native AST domain
//!       │
//!       ▼
//! semantic domain model
//!       │
//!       ▼
//! ZUIR
//!       │
//!       ▼
//! domain-specific IR
//!       │
//!       ▼
//! target realization
//! ```
//!
//! The native AST must never construct ZUIR nodes directly.
//!
//! The semantic layer is responsible for determining the meaning of a source
//! domain and lowering that meaning into the canonical universal intermediate
//! representation.
//!
//! ## Relationship to quantum IR
//!
//! The repository contains separate quantum IR/domain infrastructure.
//!
//! The native AST must not import that infrastructure merely to represent a
//! source-level domain.
//!
//! The dependency direction remains:
//!
//! ```text
//! frontend::ast::node::domains
//!             │
//!             ▼
//! semantic analysis
//!             │
//!             ▼
//! quantum/domain IR
//!             │
//!             ▼
//! target realization
//! ```
//!
//! This prevents the source AST from becoming a disguised quantum IR.
//!
//! ## Relationship to external languages
//!
//! External languages and formats such as OpenQASM must have their own frontend
//! representation.
//!
//! They must not redefine this native domain module.
//!
//! The intended direction is:
//!
//! ```text
//! external language
//!     │
//!     ▼
//! external-format AST
//!     │
//!     ▼
//! importer / semantic conversion
//!     │
//!     ▼
//! Zamani native AST / semantic model
//!     │
//!     ▼
//! ZUIR
//! ```
//!
//! In particular, OpenQASM concepts must not be imported as the native Zamani
//! domain representation.
//!
//! ## Extensibility
//!
//! Domain identity is intentionally open-ended.
//!
//! A new domain must not require:
//!
//! - changing this `mod.rs`;
//! - adding a new variant to a closed domain enum;
//! - modifying unrelated AST nodes;
//! - changing quantum-specific structures;
//! - changing resource structures;
//! - changing target backends.
//!
//! A new domain may require:
//!
//! - language/grammar registration if it introduces new source syntax;
//! - semantic registration if the compiler needs domain-specific semantics;
//! - a domain IR;
//! - target lowering;
//! - validation rules;
//! - tests.
//!
//! Those changes belong to their respective layers.
//!
//! The fundamental AST domain identity representation remains unchanged.
//!
//! ## Namespaces
//!
//! Namespaces are part of extensibility and collision avoidance.
//!
//! A domain identifier should conceptually have the form:
//!
//! ```text
//! namespace + domain name
//! ```
//!
//! For example:
//!
//! ```text
//! zamani.quantum
//! organization.example.domain
//! vendor.example.specialized_domain
//! ```
//!
//! The AST treats these as identifiers.
//!
//! It does not grant special privileges based on the spelling.
//!
//! A vendor-specific domain therefore remains an extension identity rather
//! than becoming a hard-coded vendor dependency in the compiler frontend.
//!
//! ## Versioning
//!
//! Domain identity and domain version are separate concepts.
//!
//! Conceptually:
//!
//! ```text
//! domain:
//!     zamani.quantum
//!
//! version:
//!     1.0.0
//! ```
//!
//! Version compatibility is a semantic/compiler concern.
//!
//! This module only exposes the source representation owned by the domain
//! implementation files.
//!
//! ## Parser integration
//!
//! The parser owns syntax recognition.
//!
//! The parser may consume the public types exposed here:
//!
//! ```text
//! parser
//!     │
//!     ├── Domain
//!     ├── DomainKind
//!     └── DomainAnnotation
//!     │
//!     ▼
//! native AST
//! ```
//!
//! This module performs no source parsing.
//!
//! Exact source syntax must be determined by the authoritative Zamani grammar.
//!
//! The AST must not invent syntax that is absent from the grammar.
//!
//! ## Structural validation integration
//!
//! Structural validation consumes the canonical domain AST types.
//!
//! It is responsible for checking AST graph structure, source spans, required
//! fields, and other structural invariants.
//!
//! Cross-node semantic questions do not belong here.
//!
//! Examples of semantic questions that belong later:
//!
//! ```text
//! Does this domain exist?
//! Is this domain enabled?
//! Is this domain legal in this context?
//! Can this target satisfy the domain?
//! Can this domain coexist with another domain?
//! ```
//!
//! ## Semantic integration
//!
//! Semantic analysis consumes this module's public API and resolves source
//! domain identities.
//!
//! Conceptually:
//!
//! ```text
//! Domain
//!     │
//!     ├── identity
//!     ├── version information
//!     ├── annotations
//!     └── source metadata
//!     │
//!     ▼
//! semantic domain model
//! ```
//!
//! Semantic analysis determines the actual meaning of the domain in the
//! current compilation environment.
//!
//! ## ZUIR integration contract
//!
//! Every source-level domain construct that has semantic meaning must have a
//! defined downstream lowering path.
//!
//! The contract is:
//!
//! ```text
//! AST domain
//!     │
//!     ▼
//! semantic domain
//!     │
//!     ▼
//! ZUIR domain semantics
//!     │
//!     ▼
//! domain IR
//! ```
//!
//! This `mod.rs` does not implement lowering.
//!
//! If a domain AST construct has no semantic fate, it must not be introduced
//! merely for architectural appearance.
//!
//! ## Visitor integration
//!
//! The visitor subsystem consumes the canonical types exported by this module.
//!
//! Domain nodes must participate in normal AST traversal according to their
//! actual child relationships.
//!
//! This module does not implement traversal itself.
//!
//! The visitor/traversal layer must be able to inspect domain nodes without
//! knowing anything about target hardware.
//!
//! ## Serialization integration
//!
//! Serialization belongs to the canonical AST serialization subsystem.
//!
//! This module must not introduce a competing serialization format.
//!
//! Domain structures must remain composed of deterministic source-level data
//! suitable for canonical serialization.
//!
//! Serialization must preserve:
//!
//! - domain identity;
//! - source ordering;
//! - source references;
//! - annotations;
//! - version information where represented;
//! - AST node identity where required by the AST schema.
//!
//! ## Determinism
//!
//! This module contains no mutable global state.
//!
//! It creates no:
//!
//! - random IDs;
//! - timestamps;
//! - process IDs;
//! - memory-address identities;
//! - hardware-derived identities;
//! - backend state.
//!
//! Ordering is determined by the containing AST/program structure.
//!
//! This is required for reproducible compilation and deterministic AST
//! serialization.
//!
//! ## Scalability
//!
//! There is no fixed computational-domain capacity in this module.
//!
//! The architecture is bounded only by the resources and configurable
//! compiler policies available to the compilation process.
//!
//! In particular, the AST does not impose artificial limits such as:
//!
//! ```text
//! MAX_DOMAINS
//! MAX_DOMAIN_NAME_LENGTH
//! MAX_DOMAIN_ANNOTATIONS
//! MAX_DOMAIN_REQUIREMENTS
//! ```
//!
//! Where the compiler must defend against hostile or pathological input, such
//! limits must be configurable resource policies owned by the appropriate
//! compiler/security layer.
//!
//! They must not become language semantics encoded here.
//!
//! ## "Infinity" requirement
//!
//! "Infinity" is interpreted architecturally rather than as a claim that a
//! finite computer has infinite memory.
//!
//! The requirement means that this representation must not impose an arbitrary
//! machine-size ceiling.
//!
//! A program must remain structurally representable as the available
//! computational resources increase, subject only to explicit compiler,
//! operating-system, and hardware constraints.
//!
//! Therefore:
//!
//! ```text
//! tiny target
//!     │
//!     ▼
//! same source domain model
//!     │
//!     ▼
//! larger target
//!     │
//!     ▼
//! same source domain model
//!     │
//!     ▼
//! heterogeneous/distributed target
//!     │
//!     ▼
//! same source domain model
//! ```
//!
//! The downstream compiler determines realization.
//!
//! ## Security
//!
//! Domain identifiers and annotations originate from potentially untrusted
//! source input.
//!
//! This module:
//!
//! - performs no I/O;
//! - performs no network access;
//! - executes no source code;
//! - accesses no hardware;
//! - accesses no credentials;
//! - performs no backend calls;
//! - contains no raw pointers;
//! - contains no `unsafe`;
//! - does not authorize capabilities;
//! - does not allocate hardware resources.
//!
//! Structural validation belongs to the domain implementation modules and the
//! AST validation subsystem.
//!
//! Semantic authorization belongs downstream.
//!
//! ## Dependency boundary
//!
//! The intended dependency direction is:
//!
//! ```text
//! domain_kind.rs ───────────────┐
//!                               │
//! domain_annotation.rs ─────────┼──► domains/mod.rs
//!                               │
//! domain.rs ────────────────────┘
//!                               │
//!                               ▼
//! frontend::ast::node::mod.rs
//!                               │
//!                               ▼
//! semantic analysis
//!                               │
//!                               ▼
//! ZUIR / domain IR
//!                               │
//!                               ▼
//! target/backend
//! ```
//!
//! The domain implementation files must not depend on this module merely to
//! access their own definitions.
//!
//! This `mod.rs` exists to establish the public module boundary.
//!
//! ## Forbidden dependencies
//!
//! The domain AST module must not directly depend on:
//!
//! ```text
//! quantum::hardware
//! quantum::routing
//! quantum::scheduling
//! quantum::error_correction
//! quantum::zqn
//! quantum::backend
//! quantum::runtime
//! quantum::optimization
//! quantum::simulation
//! quantum::ir
//! compiler backend implementations
//! vendor SDKs
//! LLVM
//! QIR
//! MLIR
//! OpenQASM parser implementation
//! ```
//!
//! The native AST must remain independent of those downstream systems.
//!
//! ## Public API policy
//!
//! Explicit re-exports are used instead of glob imports.
//!
//! This is intentional:
//!
//! ```rust
//! pub use domain::Domain;
//! ```
//!
//! is preferable to:
//!
//! ```rust
//! pub use domain::*;
//! ```
//!
//! Explicit exports prevent implementation details from accidentally becoming
//! part of the stable public API.
//!
//! Adding a public domain symbol should therefore be a deliberate API change.
//!
//! ## Integration stability
//!
//! Once this module is integrated, unrelated downstream changes must not
//! require modification of this file.
//!
//! In particular, adding:
//!
//! - a new quantum backend;
//! - a new quantum technology;
//! - a new machine size;
//! - a new compiler optimization;
//! - a new scheduler;
//! - a new routing algorithm;
//! - a new QEC implementation;
//! - a new hardware target;
//! - a new runtime;
//! - a new computational domain implementation
//!
//! must not require changing this module's representation merely because that
//! downstream implementation exists.
//!
//! A change to this module should normally occur only when the public AST
//! module boundary itself changes.
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
//! - no `unsafe`.
//!
//! The module deliberately uses only ordinary stable Rust module/re-export
//! facilities.
//!
//! ## Safety
//!
//! Unsafe Rust is forbidden explicitly.
//!
//! ```rust
//! #![forbid(unsafe_code)]
//! ```
//!
//! This is a source-level AST boundary and has no legitimate requirement for
//! unsafe operations.
//!
//! =============================================================================
//! Module declarations
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

/// Canonical source-level domain representation.
///
/// This module owns domain declarations/references and their local structural
/// invariants.
pub mod domain;

/// Canonical source-level extensible domain-kind identity.
///
/// This module owns the namespace/name representation of domain kinds.
pub mod domain_kind;

/// Canonical source-level domain annotation representation.
///
/// This module owns annotations attached to domain AST constructs.
pub mod domain_annotation;

// =============================================================================
// Stable public API
// =============================================================================

/// Source-level computational-domain representation.
pub use domain::Domain;

/// Extensible source-level computational-domain identity/kind.
pub use domain_kind::DomainKind;

/// Source-level annotation associated with a domain construct.
pub use domain_annotation::DomainAnnotation;

// =============================================================================
// API-boundary tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // Compile-time public API smoke tests
    // -------------------------------------------------------------------------

    #[test]
    fn canonical_domain_api_is_exported() {
        fn assert_domain_type<T>() {}

        assert_domain_type::<Domain>();
    }

    #[test]
    fn canonical_domain_kind_api_is_exported() {
        fn assert_domain_kind_type<T>() {}

        assert_domain_kind_type::<DomainKind>();
    }

    #[test]
    fn canonical_domain_annotation_api_is_exported() {
        fn assert_domain_annotation_type<T>() {}

        assert_domain_annotation_type::<DomainAnnotation>();
    }

    // -------------------------------------------------------------------------
    // Namespace-boundary tests
    // -------------------------------------------------------------------------

    #[test]
    fn implementation_modules_remain_addressable() {
        // These type paths deliberately use the owning modules rather than
        // relying exclusively on the convenience re-exports.
        //
        // The assertions are compile-time API checks. They contain no runtime
        // behavior and therefore remain independent of targets/backends.
        fn assert_domain_type<T>() {}
        fn assert_domain_kind_type<T>() {}
        fn assert_domain_annotation_type<T>() {}

        assert_domain_type::<domain::Domain>();
        assert_domain_kind_type::<domain_kind::DomainKind>();
        assert_domain_annotation_type::<domain_annotation::DomainAnnotation>();
    }
}

// =============================================================================
// Integration contract summary
// =============================================================================
//
// This module establishes the following stable boundary:
//
// parser
//   │
//   ▼
// frontend::ast::node::domains
//   │
//   ├── Domain
//   ├── DomainKind
//   └── DomainAnnotation
//   │
//   ▼
// structural AST validation
//   │
//   ▼
// semantic analysis
//   │
//   ▼
// semantic domain model
//   │
//   ▼
// ZUIR
//   │
//   ├── quantum domain IR
//   ├── classical domain IR
//   ├── HDL domain IR
//   ├── accelerator domain IR
//   ├── distributed domain IR
//   └── future domain IR
//   │
//   ▼
// target lowering
//   │
//   ▼
// hardware/runtime
//
// The following invariants are permanent:
//
// 1. No fixed number of domains.
// 2. No fixed machine size.
// 3. No fixed qubit count.
// 4. No fixed hardware topology.
// 5. No fixed vendor list.
// 6. No fixed backend list.
// 7. No closed computational-domain enum in the module boundary.
// 8. No hardware allocation.
// 9. No scheduling.
// 10. No routing.
// 11. No calibration.
// 12. No QEC implementation.
// 13. No runtime execution.
// 14. No direct ZUIR construction.
// 15. No direct quantum-IR dependency.
// 16. No unsafe Rust.
// 17. No global mutable state.
// 18. No target-specific behavior.
// 19. No competing serialization protocol.
// 20. New downstream technologies must not require redesigning this boundary.
//
// The domain AST therefore remains a stable source-language abstraction while
// semantic analysis and downstream compilation determine how the programmer's
// domain intent is realized on the available computational substrate.
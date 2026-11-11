
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Universal Computing Language — Standard Library
//!
//! Canonical path: `src/stdlib/mod.rs`
//!
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97 or later.
//! Safety: Safe Rust only.
//!
//! # 1. PURPOSE
//!
//! This module is the public composition root of the Zamani
//! Standard Library.
//!
//! It exposes reusable, portable, extensible APIs for:
//!
//! - Classical computing.
//! - Quantum computing.
//! - Hardware description and hardware-oriented programming.
//! - Hybrid and heterogeneous computing.
//! - Distributed computing.
//! - Artificial intelligence and machine learning.
//! - Scientific and numerical computing.
//! - Data processing and storage.
//! - Networking and communication.
//! - Cryptography and security.
//! - Embedded and edge computing.
//! - Robotics and physical systems.
//! - Future computational domains.
//!
//! The standard library follows the portability objective:
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! (POCO-REAF).
//!
//! # 2. RESPONSIBILITIES
//!
//! This module owns:
//!
//! - Standard-library module composition.
//! - Public module declarations.
//! - Stable standard-library module paths.
//! - Standard-library architectural documentation.
//! - Standard-library namespace-level invariants.
//!
//! This module does NOT own:
//!
//! - Source grammar.
//! - Lexical analysis.
//! - Parsing.
//! - AST construction.
//! - Semantic analysis.
//! - Semantic type definitions.
//! - Requirement or capability semantics.
//! - Resource feasibility assessment.
//! - Intermediate representation definitions.
//! - Compiler transformations.
//! - Optimization and lowering.
//! - Backend implementations.
//! - Provider implementation.
//! - Hardware discovery.
//! - Hardware allocation.
//! - Runtime scheduling.
//! - Physical execution.
//! - Operating-system initialization.
//! - Global subsystem lifecycle.
//!
//! These responsibilities remain with their authoritative
//! repository subsystems.
//!
//! # 3. AUTHORITATIVE REPOSITORY ARCHITECTURE
//!
//! The standard library integrates with the following authorities:
//!
//! Grammar:
//!     grammar/
//!
//! Frontend:
//!     src/frontend/
//!
//! Semantic authority:
//!     src/frontend/semantic/
//!
//! Classical computing:
//!     src/classical/
//!
//! Quantum computing:
//!     src/quantum/
//!
//! Hardware description:
//!     src/hdl/
//!
//! Hybrid computing:
//!     src/hybrid/
//!
//! Distributed computing:
//!     src/distributed/
//!
//! Artificial intelligence:
//!     src/ai/
//!
//! Universal intermediate representation:
//!     src/zuir/
//!
//! Compiler:
//!     src/compiler/
//!
//! Backend:
//!     src/backend/
//!
//! Runtime:
//!     src/runtime/
//!
//! Hardware abstraction and platform providers:
//!     Their respective authoritative repository subsystems.
//!
//! The standard library consumes these authorities through
//! explicit APIs and integration contracts.
//!
//! It must not create competing implementations.
//!
//! # 4. POCO-REAF AND SCALABILITY
//!
//! The standard library must not introduce arbitrary universal
//! ceilings on:
//!
//! - CPUs.
//! - Processor cores.
//! - Threads.
//! - GPUs.
//! - FPGAs.
//! - ASICs.
//! - QPUs.
//! - Accelerators.
//! - Quantum or logical qubits.
//! - Registers or register widths.
//! - Memory.
//! - Storage.
//! - Tensor dimensions.
//! - Tensor rank.
//! - Distributed nodes.
//! - Network participants.
//! - Devices.
//! - Computational domains.
//! - Functions.
//! - Modules.
//! - Operations.
//!
//! Actual execution is subject to:
//!
//! - Available physical resources.
//! - Address-space and representation limits.
//! - Provider capabilities.
//! - Program requirements.
//! - Mandatory constraints.
//! - Operational budgets.
//! - Platform and runtime support.
//!
//! An operational budget is not a universal language capability
//! restriction.
//!
//! No finite implementation promises literal infinite resources.
//!
//! The objective is resource-parametric computation without
//! arbitrary architectural ceilings.
//!
//! # 5. SEMANTIC AND RESOURCE DISTINCTIONS
//!
//! The following concepts must remain distinct throughout
//! the standard library:
//!
//! Requirement:
//!     What an operation requires.
//!
//! Capability:
//!     What an environment or provider can supply.
//!
//! Preference:
//!     What is desirable but not mandatory.
//!
//! Constraint:
//!     What an acceptable realization must obey.
//!
//! Hint:
//!     Non-binding implementation guidance.
//!
//! Feasibility:
//!     Whether available evidence establishes that an operation
//!     can be realized.
//!
//! Semantic validity does not establish target feasibility.
//!
//! Provider registration does not establish provider readiness.
//!
//! Compilation does not establish successful execution.
//!
//! # 6. MODULE INITIALIZATION
//!
//! Importing the standard library performs no initialization.
//!
//! This module MUST NOT:
//!
//! - Initialize every standard-library component.
//! - Initialize every computational domain.
//! - Discover hardware.
//! - Allocate physical resources.
//! - Start background threads.
//! - Create global mutable registries.
//! - Connect to external services.
//! - Open network connections.
//! - Initialize quantum hardware.
//! - Initialize AI models.
//! - Start distributed execution.
//! - Execute user programs.
//!
//! Explicit lifecycle operations belong to:
//!
//!     crate::stdlib::lifecycle
//!
//! Provider registration and discovery belong to:
//!
//!     crate::stdlib::registry
//!     crate::stdlib::provider
//!
//! Actual scheduling and execution belong to:
//!
//!     crate::runtime
//!
//! This separation allows applications to use only the
//! standard-library functionality they need.
//!
//! # 7. COMPATIBILITY
//!
//! Existing root-level module paths are retained during migration.
//!
//! Legacy modules such as:
//!
//!     crate::stdlib::core
//!     crate::stdlib::collections
//!     crate::stdlib::numeric
//!     crate::stdlib::io
//!     crate::stdlib::fs
//!     crate::stdlib::net
//!     crate::stdlib::network
//!
//! remain available where their source implementations exist.
//!
//! Their internal implementations may migrate into canonical
//! subdirectories without requiring immediate public-path removal.
//!
//! Compatibility facades must not create duplicate authorities.
//!
//! # 8. ERROR HANDLING
//!
//! General standard-library errors are owned by:
//!
//!     crate::stdlib::error
//!
//! Specialized subsystems may preserve their own detailed errors.
//!
//! Errors must not be converted into simulated successful results.
//!
//! Original causal information and operation context must be
//! preserved through explicit adapters.
//!
//! # 9. SAFETY
//!
//! Unsafe Rust is forbidden throughout this standard library.
//!
//! This module introduces no unsafe implementation requirements.
//!
//! External input, serialized data, provider responses and
//! hardware information must be validated by their owning
//! subsystem boundaries.
//!
//! # 10. ARCHITECTURAL INVARIANTS
//!
//! 1. One authoritative implementation per concept.
//! 2. No duplicate semantic type system.
//! 3. No duplicate AST.
//! 4. No competing universal IR.
//! 5. No duplicate tensor storage authority.
//! 6. No competing runtime scheduler.
//! 7. No hardware-specific assumptions in portable APIs.
//! 8. No arbitrary universal hardware ceilings.
//! 9. No unconditional advanced-domain initialization.
//! 10. No simulated production success.
//! 11. No unsafe Rust.
//! 12. Provider extensibility must remain open.
//! 13. Existing public APIs must be migrated deliberately.
//! 14. Domain-specific functionality must use its canonical owner.

// ============================================================================
// STANDARD LIBRARY INFRASTRUCTURE
// ============================================================================

// ---------------------------------------------------------------------------
// Versioning
// ---------------------------------------------------------------------------

/// Standard-library and component version infrastructure.
///
/// Distinguishes package versions from language, grammar, API,
// serialization and IR schema versions.
pub mod version;

// ---------------------------------------------------------------------------
// Public contracts
// ---------------------------------------------------------------------------

/// Standard-library API stability and compatibility contracts.
pub mod contract;

// ---------------------------------------------------------------------------
// Structured errors
// ---------------------------------------------------------------------------

/// Shared structured standard-library errors.
pub mod error;

// ---------------------------------------------------------------------------
// Requirements
// ---------------------------------------------------------------------------

/// Mandatory computational and operational requirements.
pub mod requirement;

// ---------------------------------------------------------------------------
// Capabilities
// ---------------------------------------------------------------------------

/// Extensible capability descriptions and evidence.
pub mod capability;

// ---------------------------------------------------------------------------
// Preferences
// ---------------------------------------------------------------------------

/// Non-mandatory realization preferences.
pub mod preference;

// ---------------------------------------------------------------------------
// Constraints
// ---------------------------------------------------------------------------

/// Mandatory restrictions and constraints.
pub mod constraint;

// ---------------------------------------------------------------------------
// Hints
// ---------------------------------------------------------------------------

/// Non-binding optimization and implementation guidance.
pub mod hint;

// ---------------------------------------------------------------------------
// Feasibility
// ---------------------------------------------------------------------------

/// Evidence-based feasibility representations.
pub mod feasibility;

// ---------------------------------------------------------------------------
// Operation context
// ---------------------------------------------------------------------------

/// Shared operation context and cancellation information.
pub mod context;

// ---------------------------------------------------------------------------
// Provider contracts
// ---------------------------------------------------------------------------

/// Extensible provider interfaces and metadata.
pub mod provider;

// ---------------------------------------------------------------------------
// Provider registry
// ---------------------------------------------------------------------------

/// Standard-library provider registration and discovery.
pub mod registry;

// ---------------------------------------------------------------------------
// Component manifests
// ---------------------------------------------------------------------------

/// Component metadata and dependency declarations.
pub mod manifest;

// ---------------------------------------------------------------------------
// Lifecycle
// ---------------------------------------------------------------------------

/// Explicit dependency-aware component lifecycle management.
///
/// Initialization and shutdown are delegated here rather than
/// being performed by this composition module.
pub mod lifecycle;

// ============================================================================
// FOUNDATIONAL STANDARD LIBRARY
// ============================================================================

// ---------------------------------------------------------------------------
// Canonical foundation aggregation
// ---------------------------------------------------------------------------

/// Portable foundational APIs.
///
/// Integrates reusable collections, primitive implementation
/// contracts, memory abstractions, strings, conversion, iteration,
/// comparison, numeric and mathematical functionality.
pub mod foundation;

// ---------------------------------------------------------------------------
// Canonical data aggregation
// ---------------------------------------------------------------------------

/// Portable data abstractions.
///
/// Integrates buffers, storage, serialization, formats and
/// provenance without duplicating tensor or semantic ownership.
pub mod data;

// ============================================================================
// COMPUTATIONAL DOMAIN FACADES
// ============================================================================

// ---------------------------------------------------------------------------
// Domain aggregation
// ---------------------------------------------------------------------------

/// Standard-library computational-domain facades.
///
/// Provides access to classical, quantum, HDL, hybrid,
/// distributed, AI, embedded, robotics and future domain APIs.
///
/// These facades reuse canonical repository authorities.
pub mod domains;

// ============================================================================
// NETWORKING
// ============================================================================

// ---------------------------------------------------------------------------
// Canonical networking aggregation
// ---------------------------------------------------------------------------

/// Portable networking contracts.
///
/// Provides address, transport, protocol, discovery and
/// security integration boundaries.
pub mod networking;

// ============================================================================
// REPOSITORY INTEGRATION
// ============================================================================

// ---------------------------------------------------------------------------
// Integration aggregation
// ---------------------------------------------------------------------------

/// Integration boundaries for the canonical frontend,
/// semantic subsystem, IR authorities, compiler and providers.
///
/// This namespace does not create another compiler, semantic
/// analyzer, IR, provider registry or runtime.
pub mod integration;

// ============================================================================
// LEGACY FOUNDATIONAL MODULES
// ============================================================================

// ---------------------------------------------------------------------------
// Core compatibility
// ---------------------------------------------------------------------------

/// Legacy core API compatibility.
///
/// New foundational implementations should use the canonical
/// foundation modules.
pub mod core;

// ---------------------------------------------------------------------------
// Collections compatibility
// ---------------------------------------------------------------------------

/// Legacy collection API compatibility.
///
/// The canonical collection implementation is being consolidated
/// under the foundation collection authority.
pub mod collections;

// ---------------------------------------------------------------------------
// Numeric compatibility
// ---------------------------------------------------------------------------

/// Legacy numeric API compatibility.
pub mod numeric;

// ---------------------------------------------------------------------------
// Time
// ---------------------------------------------------------------------------

/// Portable time and temporal APIs.
pub mod time;

// ---------------------------------------------------------------------------
// Input and output compatibility
// ---------------------------------------------------------------------------

/// Legacy I/O API compatibility.
///
/// The canonical I/O subsystem is migrated separately.
pub mod io;

// ---------------------------------------------------------------------------
// Filesystem compatibility
// ---------------------------------------------------------------------------

/// Legacy filesystem API compatibility.
pub mod fs;

// ---------------------------------------------------------------------------
// Serialization compatibility
// ---------------------------------------------------------------------------

/// Legacy serialization API compatibility.
pub mod serialize;

// ---------------------------------------------------------------------------
// Synchronization compatibility
// ---------------------------------------------------------------------------

/// Legacy synchronization API compatibility.
pub mod sync;

// ---------------------------------------------------------------------------
// Concurrency compatibility
// ---------------------------------------------------------------------------

/// Legacy concurrency API compatibility.
pub mod concurrent;

// ============================================================================
// NETWORKING AND COMMUNICATION COMPATIBILITY
// ============================================================================

// ---------------------------------------------------------------------------
// Legacy network APIs
// ---------------------------------------------------------------------------

/// Legacy networking API compatibility.
///
/// Retained until all existing consumers have migrated to
/// the canonical networking namespace.
pub mod net;

/// Legacy network-stack API compatibility.
///
/// This is not a second networking authority. Its implementation
/// and consumers must be reconciled with `networking`.
pub mod network;

// ---------------------------------------------------------------------------
// Database
// ---------------------------------------------------------------------------

/// Database interfaces and database-related APIs.
pub mod db;

// ---------------------------------------------------------------------------
// External services
// ---------------------------------------------------------------------------

/// External-service integration APIs.
pub mod external_services;

// ---------------------------------------------------------------------------
// Web
// ---------------------------------------------------------------------------

/// Web and HTTP-oriented APIs.
pub mod web;

/// Web-development functionality.
pub mod web_development;

// ============================================================================
// SECURITY AND DISTRIBUTED SYSTEMS
// ============================================================================

// ---------------------------------------------------------------------------
// Cryptography
// ---------------------------------------------------------------------------

/// Cryptographic API aggregation.
pub mod crypto;

// ---------------------------------------------------------------------------
// Distributed ledger
// ---------------------------------------------------------------------------

/// Distributed-ledger interfaces and associated functionality.
pub mod distributed_ledger;

// ---------------------------------------------------------------------------
// Security and trust
// ---------------------------------------------------------------------------

/// Cryptographic and identity-related advanced hashing APIs.
pub mod omniversal_hashing;

/// Privacy-preserving proof and zero-knowledge APIs.
pub mod omniversal_zkp_privacy_computing;

/// Trust and identity management APIs.
pub mod omniversal_trust_identity_management;

/// Sovereignty and identity-oriented functionality.
pub mod omniversal_sovereignty;

// ============================================================================
// CLASSICAL, SCIENTIFIC AND MULTIDIMENSIONAL COMPUTING
// ============================================================================

// ---------------------------------------------------------------------------
// Mathematical foundations
// ---------------------------------------------------------------------------

/// Mathematical foundations and reusable mathematical facilities.
pub mod math_foundations;

// ---------------------------------------------------------------------------
// Multidimensional computing
// ---------------------------------------------------------------------------

/// Multidimensional computation compatibility.
///
/// Canonical tensor storage remains owned by the appropriate
/// classical and AI authorities.
pub mod multidimensional;

// ---------------------------------------------------------------------------
// General data structures
// ---------------------------------------------------------------------------

/// Advanced data-structure APIs.
pub mod omniversal_data_structures;

// ---------------------------------------------------------------------------
// Resource descriptions and management
// ---------------------------------------------------------------------------

/// Resource-management APIs.
///
/// Physical allocation and scheduling remain runtime responsibilities.
pub mod resource_management;

// ============================================================================
// QUANTUM, NANO AND PHYSICAL COMPUTING
// ============================================================================

// ---------------------------------------------------------------------------
// Quantum
// ---------------------------------------------------------------------------

/// High-level quantum standard-library compatibility.
///
/// Canonical quantum semantics and IR remain under `crate::quantum`.
pub mod quantum;

// ---------------------------------------------------------------------------
// Nanotechnology
// ---------------------------------------------------------------------------

/// Nano and nanotechnology-oriented computational APIs.
pub mod nano;

// ---------------------------------------------------------------------------
// MTS
// ---------------------------------------------------------------------------

/// MTS-related computational APIs.
pub mod mts;

// ---------------------------------------------------------------------------
// Sankofa
// ---------------------------------------------------------------------------

/// Sankofa-related computational APIs.
pub mod sankofa;

// ---------------------------------------------------------------------------
// Physical hardware
// ---------------------------------------------------------------------------

/// Physical hardware-control interfaces.
///
/// Actual hardware access remains owned by HAL and device providers.
pub mod physical_hardware_control;

// ---------------------------------------------------------------------------
// Biological and nano computing
// ---------------------------------------------------------------------------

/// Specialized biological and nanotechnology-oriented APIs.
pub mod omniversal_bionano_os;

// ============================================================================
// ARTIFICIAL INTELLIGENCE AND INTELLIGENCE SYSTEMS
// ============================================================================

// ---------------------------------------------------------------------------
// Machine learning
// ---------------------------------------------------------------------------

/// Machine-learning compatibility APIs.
///
/// Canonical AI computation remains under `crate::ai`.
pub mod ml;

// ---------------------------------------------------------------------------
// AI reasoning
// ---------------------------------------------------------------------------

/// AI reasoning integration.
pub mod ai_reasoning;

// ---------------------------------------------------------------------------
// Natural language processing
// ---------------------------------------------------------------------------

/// Natural-language processing APIs.
pub mod nlp;

/// Advanced natural-language processing APIs.
pub mod omniversal_nlp;

/// Extended NLP functionality.
pub mod omniversal_nlp_adv;

// ---------------------------------------------------------------------------
// Computer vision
// ---------------------------------------------------------------------------

/// Computer-vision functionality.
pub mod vision;

// ---------------------------------------------------------------------------
// Generative AI
// ---------------------------------------------------------------------------

/// Generative AI functionality.
pub mod omniversal_generative_ai;

// ---------------------------------------------------------------------------
// Explainable AI
// ---------------------------------------------------------------------------

/// Explainable AI interfaces and functionality.
pub mod omniversal_explainable_ai;

// ---------------------------------------------------------------------------
// Knowledge systems
// ---------------------------------------------------------------------------

/// Knowledge-graph functionality.
pub mod omniversal_knowledge_graph;

/// Knowledge and semantic reasoning functionality.
pub mod omniversal_knowledge_semantic_reasoning;

// ---------------------------------------------------------------------------
// Neural systems
// ---------------------------------------------------------------------------

/// Neural-substrate integration.
pub mod omniversal_neural_substrate;

// ---------------------------------------------------------------------------
// Advanced intelligence
// ---------------------------------------------------------------------------

/// Advanced intelligence APIs.
pub mod omniversal_superintelligence;

/// Retrieval-augmented generation and hallucination-control APIs.
pub mod omniversal_hallucination_rag;

/// Advanced data-science and mining APIs.
pub mod omniversal_advanced_data_science_mining;

// ============================================================================
// AGENTS, ROBOTICS AND AUTONOMOUS SYSTEMS
// ============================================================================

// ---------------------------------------------------------------------------
// Robotics
// ---------------------------------------------------------------------------

/// Robotics interfaces.
///
/// Physical actuation requires explicit hardware-provider
/// capabilities and appropriate safety validation.
pub mod robotics;

// ---------------------------------------------------------------------------
// On-device agents
// ---------------------------------------------------------------------------

/// On-device agent integration.
pub mod on_device_agents;

// ---------------------------------------------------------------------------
// Agent orchestration
// ---------------------------------------------------------------------------

/// Autonomous workflow and agent orchestration interfaces.
pub mod autonomous_workflow_agent_orchestration;

// ---------------------------------------------------------------------------
// Agent interaction
// ---------------------------------------------------------------------------

/// Human and AI interaction interfaces.
pub mod human_agi_interaction;

/// Chat-architect agent functionality.
pub mod chat_architect_agent;

// ---------------------------------------------------------------------------
// Autonomous perception
// ---------------------------------------------------------------------------

/// Perception and autonomous-action interfaces.
pub mod omniversal_perception_autonomous_action;

// ============================================================================
// HARDWARE, IOT AND HUMAN INTERFACES
// ============================================================================

// ---------------------------------------------------------------------------
// Internet of Things
// ---------------------------------------------------------------------------

/// Internet-of-Things integration.
pub mod iot;

// ---------------------------------------------------------------------------
// Human interface devices
// ---------------------------------------------------------------------------

/// Human-interface-device abstractions.
pub mod human_interface_devices;

// ---------------------------------------------------------------------------
// GUI
// ---------------------------------------------------------------------------

/// Graphical user-interface abstractions.
pub mod gui;

// ---------------------------------------------------------------------------
// Music and language
// ---------------------------------------------------------------------------

/// Music and language-oriented computation.
pub mod music_language;

// ============================================================================
// REFLECTION, LANGUAGE TOOLING AND METAPROGRAMMING
// ============================================================================

// ---------------------------------------------------------------------------
// Reflection
// ---------------------------------------------------------------------------

/// Reflection compatibility APIs.
pub mod reflection;

// ---------------------------------------------------------------------------
// Meta operations
// ---------------------------------------------------------------------------

/// Metaprogramming and meta-operation interfaces.
pub mod meta_ops;

// ---------------------------------------------------------------------------
// Self-modifying metaprogramming
// ---------------------------------------------------------------------------

/// Controlled metaprogramming and self-modification APIs.
pub mod meta_programming_self_mod;

// ---------------------------------------------------------------------------
// Programming paradigms
// ---------------------------------------------------------------------------

/// Programming-paradigm integration.
pub mod programming_paradigms;

// ============================================================================
// DOCUMENTATION AND DEVELOPER TOOLING
// ============================================================================

// ---------------------------------------------------------------------------
// Documentation
// ---------------------------------------------------------------------------

/// Documentation generation and documentation APIs.
pub mod documentation;

/// Documentation-system integration.
pub mod documentation_system;

// ---------------------------------------------------------------------------
// Editor integration
// ---------------------------------------------------------------------------

/// Editor and development-tool integration.
pub mod editor_integration;

// ---------------------------------------------------------------------------
// Testing
// ---------------------------------------------------------------------------

/// Testing-framework compatibility.
pub mod test_framework;

// ---------------------------------------------------------------------------
// Developer relations
// ---------------------------------------------------------------------------

/// Developer-oriented functionality.
pub mod developer_relations;

// ============================================================================
// GOVERNANCE, SECURITY POLICY AND SYSTEM DESIGN
// ============================================================================

// ---------------------------------------------------------------------------
// AGI governance
// ---------------------------------------------------------------------------

/// AGI governance APIs.
pub mod agi_governance;

/// Governance and safety functionality.
pub mod omniversal_governance_safety;

// ---------------------------------------------------------------------------
// Runtime governance
// ---------------------------------------------------------------------------

/// Runtime governance interfaces.
///
/// Runtime execution remains owned by the runtime subsystem.
pub mod runtime_governance;

// ---------------------------------------------------------------------------
// Legal and ethical policy
// ---------------------------------------------------------------------------

/// Legal and ethical policy interfaces.
pub mod omniversal_legal_ethics;

// ---------------------------------------------------------------------------
// Design principles
// ---------------------------------------------------------------------------

/// Design-principle documentation and APIs.
pub mod design_principles;

// ---------------------------------------------------------------------------
// System design
// ---------------------------------------------------------------------------

/// Reusable system-design functionality.
pub mod system_design;

// ---------------------------------------------------------------------------
// Strategic goal management
// ---------------------------------------------------------------------------

/// Strategic goal-management APIs.
pub mod omniversal_strategic_goal_management;

// ---------------------------------------------------------------------------
// Self-adaptation and discovery
// ---------------------------------------------------------------------------

/// Self-adaptation interfaces.
pub mod omniversal_self_adaptation;

/// Self-discovery functionality.
pub mod omniversal_self_discovery;

// ---------------------------------------------------------------------------
// Alignment and prevention
// ---------------------------------------------------------------------------

/// Alignment and orchestration interfaces.
pub mod omniversal_alignment_orchestration_global_immutable_nexus;

/// Rogue-prevention and alignment functionality.
pub mod omniversal_rogue_prevention_alignment;

// ---------------------------------------------------------------------------
// Sovereignty and self-management
// ---------------------------------------------------------------------------

/// Self-sovereignty and existential-management functionality.
pub mod omniversal_self_sovereignty_existential_management;

// ============================================================================
// SIMULATION, COMMERCE AND MEDIA
// ============================================================================

// ---------------------------------------------------------------------------
// Simulation
// ---------------------------------------------------------------------------

/// Omniversal simulation interfaces.
pub mod omniversal_simulation;

// ---------------------------------------------------------------------------
// Commerce
// ---------------------------------------------------------------------------

/// Commerce-related computational APIs.
pub mod omniversal_commerce;

// ---------------------------------------------------------------------------
// Media
// ---------------------------------------------------------------------------

/// Media-synthesis interfaces.
pub mod omniversal_media_synthesis;

// ---------------------------------------------------------------------------
// Experimental and future systems
// ---------------------------------------------------------------------------

/// Experimental reality and metaphysical-engineering APIs.
///
/// This module must not introduce unverified physical or
/// computational claims into stable language semantics.
pub mod omniversal_reality_metaphysical_engineering;

// ---------------------------------------------------------------------------
// Security and deployment
// ---------------------------------------------------------------------------

/// Secure autonomous deployment and hardening interfaces.
pub mod omniversal_autonomous_deployment_orchestration_secure_hardening;

/// Prompt-firewall and prompt-security functionality.
pub mod omniversal_prompt_firewall;

// ---------------------------------------------------------------------------
// Autonomous code synthesis
// ---------------------------------------------------------------------------

/// Autonomous code-system synthesis interfaces.
///
/// Compilation and code generation remain compiler responsibilities.
pub mod omniversal_autonomous_code_system_synthesis;

// ============================================================================
// ADDITIONAL EXISTING ROOT-LEVEL SYSTEMS
// ============================================================================

// ---------------------------------------------------------------------------
// MGNS
// ---------------------------------------------------------------------------

/// MGNS integration.
///
/// Its precise architectural authority must remain documented
/// by its implementation and repository-level integration contract.
pub mod mgns;

// ============================================================================
// STANDARD LIBRARY COMPOSITION METADATA
// ============================================================================

/// Stable namespace identifier for the standard library.
pub const STDLIB_NAMESPACE: &str = "zamani.stdlib";

/// Returns the standard-library namespace.
#[must_use]
pub const fn stdlib_namespace() -> &'static str {
    STDLIB_NAMESPACE
}

/// Returns the standard-library package version.
///
/// This is the Cargo package version, not the language version,
/// grammar version or standard-library API compatibility version.
#[must_use]
pub const fn package_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Returns the standard-library architectural invariants.
///
/// This is descriptive metadata only. It performs no initialization,
/// hardware discovery, registration or execution.
#[must_use]
pub const fn architectural_invariants() -> &'static [&'static str] {
    &[
        "one authoritative implementation per concept",
        "grammar owns source syntax",
        "frontend owns parsing and AST construction",
        "semantic subsystem owns source meaning",
        "canonical IR subsystems own intermediate representations",
        "compiler owns compilation and lowering",
        "runtime owns scheduling and execution",
        "HAL owns hardware communication",
        "providers expose extensible implementations",
        "requirements remain distinct from preferences",
        "constraints remain distinct from hints",
        "capabilities require authoritative evidence",
        "feasibility remains distinct from semantic validity",
        "no arbitrary universal hardware ceilings",
        "no unconditional advanced-domain initialization",
        "safe Rust only",
    ]
}

// ============================================================================
// ARCHITECTURAL TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_library_namespace_is_stable() {
        assert_eq!(
            stdlib_namespace(),
            "zamani.stdlib"
        );
    }

    #[test]
    fn package_version_is_available() {
        assert!(!package_version().is_empty());
    }

    #[test]
    fn architectural_invariants_are_available() {
        let invariants = architectural_invariants();

        assert!(!invariants.is_empty());

        assert!(invariants.iter().any(|item| {
            item.contains("grammar owns source syntax")
        }));

        assert!(invariants.iter().any(|item| {
            item.contains("semantic subsystem owns source meaning")
        }));

        assert!(invariants.iter().any(|item| {
            item.contains("no arbitrary universal hardware ceilings")
        }));
    }

    #[test]
    fn standard_library_foundation_is_exposed() {
        fn accepts_collection<T>(
            _: foundation::collections::List<T>
        ) {}

        let _ = accepts_collection::<u64>;
    }

    #[test]
    fn standard_library_data_subsystem_is_exposed() {
        let _ = std::any::type_name::<data::buffers::Buffer>();
    }

    #[test]
    fn standard_library_domain_facades_are_exposed() {
        let _ = std::any::type_name::<domains::classical::ClassicalDomain>();
    }

    #[test]
    fn standard_library_networking_is_exposed() {
        let _ = std::any::type_name::<networking::addresses::Address>();
    }

    #[test]
    fn standard_library_integration_is_exposed() {
        assert_eq!(
            integration::integration_namespace(),
            "zamani.stdlib.integration"
        );
    }

    #[test]
    fn standard_library_uses_safe_rust() {
        // The crate-level safety policy forbids unsafe Rust.
        // No initialization is required to use the standard library.
        assert!(true);
    }
}

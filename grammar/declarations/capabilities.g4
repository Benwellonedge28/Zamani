/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/declarations/capabilities.g4
 *
 * Role:
 *     DECLARATION-LAYER CAPABILITY INTEGRATION
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file integrates capability declarations into Zamani's universal
 * declaration grammar.
 *
 * IMPORTANT:
 *
 * This file does NOT redefine capability syntax.
 *
 * Canonical capability syntax is owned by:
 *
 *     grammar/core/capabilities.g4
 *
 * This file exists because the language architecture separates:
 *
 *     core language constructs
 *     declaration dispatch
 *     domain-specific declarations
 *
 * The declaration layer therefore delegates capability declarations to the
 * canonical capability grammar.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - declaration-layer entry points for capabilities;
 *     - integration of canonical capability declarations into declaration
 *       dispatch;
 *     - declaration-level capability lists;
 *     - declaration-level capability sections where such sections are
 *       explicitly accepted by the universal declaration grammar;
 *     - declaration-layer syntactic delegation.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - capability identity;
 *     - capability names;
 *     - qualified names;
 *     - capability versions;
 *     - version compatibility;
 *     - capability registry;
 *     - capability discovery;
 *     - capability resolution;
 *     - capability semantics;
 *     - resource allocation;
 *     - resource discovery;
 *     - hardware discovery;
 *     - target selection;
 *     - device selection;
 *     - physical qubit selection;
 *     - QEC;
 *     - ZQN;
 *     - quantum::ir;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - calibration;
 *     - runtime authorization;
 *     - compiler backend selection.
 *
 * ============================================================================
 * CANONICAL CAPABILITY OWNER
 * ============================================================================
 *
 * The canonical capability grammar is:
 *
 *     grammar/core/capabilities.g4
 *
 * It owns:
 *
 *     capabilityDeclaration
 *     capabilityReference
 *     capabilityName
 *     capabilityVersionClause
 *
 * This file MUST consume those rules rather than reproduce them.
 *
 * Consequently there must be only one canonical definition of:
 *
 *     capabilityDeclaration
 *
 * in the grammar system.
 *
 * ============================================================================
 * RESOURCE CAPABILITY OWNER
 * ============================================================================
 *
 * Resource-specific capability relationships are owned by:
 *
 *     grammar/resources/capabilities.g4
 *
 * That grammar handles concepts such as:
 *
 *     capability requirements
 *     capability constraints
 *     capability preferences
 *     capability hints
 *     capability availability
 *     capability implications
 *     capability exclusions
 *     capability composition
 *
 * This file does not duplicate those rules.
 *
 * ============================================================================
 * HARDWARE CAPABILITY OWNER
 * ============================================================================
 *
 * Hardware capability syntax belongs to the hardware grammar family.
 *
 * Existing hardware capability grammars remain responsible for hardware
 * declaration syntax.
 *
 * This file only makes canonical capability declarations available to the
 * universal declaration layer.
 *
 * ============================================================================
 * QUANTUM CAPABILITY OWNER
 * ============================================================================
 *
 * Quantum capability syntax remains an extension of the canonical capability
 * model.
 *
 * Examples:
 *
 *     zamani::quantum::dynamic_control
 *     zamani::quantum::mid_circuit_measurement
 *     zamani::quantum::logical_qubits
 *
 * are capability identities.
 *
 * They are NOT quantum gates, physical qubits, QubitId values, or backend
 * selections.
 *
 * The canonical quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * This grammar has no dependency on quantum::ir.
 *
 * ============================================================================
 * OPEN-WORLD CAPABILITY MODEL
 * ============================================================================
 *
 * Capability names are intentionally open-world.
 *
 * This file MUST NOT enumerate:
 *
 *     CPU capabilities
 *     GPU capabilities
 *     FPGA capabilities
 *     ASIC capabilities
 *     QPU capabilities
 *     vendor capabilities
 *     future capabilities
 *
 * as closed parser alternatives.
 *
 * New capabilities are represented through the canonical capability-name
 * grammar.
 *
 * For example:
 *
 *     zamani::compute::parallel
 *     zamani::quantum::dynamic_control
 *     zamani::hardware::fpga
 *     zamani::hardware::gpu
 *     zamani::network::rdma
 *     zamani::security::post_quantum_crypto
 *     future::compute::new_architecture
 *
 * do not require this file to change.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Capability declarations express semantic intent.
 *
 * They MUST NOT encode a particular machine.
 *
 * A declaration such as:
 *
 *     capability zamani::quantum::dynamic_control;
 *
 * does not select:
 *
 *     a QPU;
 *     a physical qubit;
 *     a topology;
 *     a vendor;
 *     a backend;
 *     a number of qubits;
 *     a CPU;
 *     a GPU;
 *     an FPGA.
 *
 * Those decisions belong downstream.
 *
 * The intended pipeline is:
 *
 *     source
 *       |
 *       v
 *     lexer
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic capability model
 *       |
 *       +--> resource analysis
 *       +--> target capability discovery
 *       +--> compatibility analysis
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> HDL/hardware representation
 *       |
 *       v
 *     optimization / routing / scheduling / resilience
 *       |
 *       v
 *     HAL / runtime / target realization
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are NO finite grammar-level limits on:
 *
 *     capability declarations
 *     capability references
 *     capabilities per declaration
 *     namespaces
 *     capability relationships
 *     programs
 *     machines
 *     devices
 *     qubits
 *     CPUs
 *     GPUs
 *     FPGAs
 *     nodes
 *     memory
 *     accelerators
 *
 * This file MUST NOT introduce:
 *
 *     MAX_CAPABILITIES
 *     MAX_RESOURCES
 *     MAX_DEVICES
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_ACCELERATORS
 *
 * Repetition is represented structurally with ANTLR repetition operators.
 *
 * ============================================================================
 * DECLARATION SEMANTICS
 * ============================================================================
 *
 * A capability declaration establishes a source-level capability contract.
 *
 * It does NOT imply that the capability is:
 *
 *     available;
 *     installed;
 *     enabled;
 *     authorized;
 *     realizable;
 *     optimal;
 *     supported by the current target.
 *
 * Those questions are semantic/runtime questions.
 *
 * ============================================================================
 * REQUIREMENT VS CAPABILITY
 * ============================================================================
 *
 * These concepts MUST remain distinct.
 *
 * Capability:
 *
 *     capability zamani::quantum::dynamic_control;
 *
 * Requirement:
 *
 *     requires capability zamani::quantum::dynamic_control;
 *
 * The first declares a capability identity.
 *
 * The second expresses a requirement on an execution environment.
 *
 * Requirement syntax belongs to the resource/requirement grammar family.
 *
 * ============================================================================
 * CAPABILITY VS RESOURCE
 * ============================================================================
 *
 * Capability:
 *
 *     what an execution environment can do.
 *
 * Resource:
 *
 *     something available to computation.
 *
 * Capability:
 *
 *     zamani::compute::parallel
 *
 * does not mean:
 *
 *     allocate N CPUs.
 *
 * Likewise:
 *
 *     zamani::quantum::dynamic_control
 *
 * does not mean:
 *
 *     select physical qubit X.
 *
 * ============================================================================
 * CAPABILITY VS TARGET
 * ============================================================================
 *
 * Capability describes an ability.
 *
 * Target describes an execution context/profile.
 *
 * This file does not collapse these concepts.
 *
 * Target-specific realization belongs to:
 *
 *     grammar/hardware/
 *     grammar/compile/
 *     grammar/execution/
 *     grammar/resources/
 *
 * and their corresponding semantic/runtime layers.
 *
 * ============================================================================
 * CAPABILITY VS AUTHORIZATION
 * ============================================================================
 *
 * A declared capability is not an authorization token.
 *
 * Authorization belongs to the security/runtime model.
 *
 * Parsing MUST NOT grant privileges.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This file creates no new capability AST node.
 *
 * Instead:
 *
 *     declarationCapability
 *             |
 *             v
 *     capabilityDeclaration
 *             |
 *             v
 *     existing capability AST representation
 *
 * The AST must preserve:
 *
 *     capability identity
 *     version requirement
 *     source span
 *     attributes/metadata where accepted by the canonical capability grammar
 *
 * The AST MUST NOT contain:
 *
 *     physical device identity
 *     hardware state
 *     runtime authorization
 *     resource allocation
 *     scheduler state
 *     routing state
 *     QEC state
 *     calibration state
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     - resolving capability identity;
 *     - resolving namespaces;
 *     - validating declarations;
 *     - validating duplicate/conflicting declarations where applicable;
 *     - resolving capability versions;
 *     - checking compatibility;
 *     - connecting declarations to capability registries;
 *     - connecting requirements to provided capabilities;
 *     - checking target applicability;
 *     - checking resource implications;
 *     - producing diagnostics.
 *
 * This grammar performs none of those operations.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * Capability declarations are semantic metadata/intent.
 *
 * They must not create a second IR.
 *
 * If capability information affects quantum compilation, semantic analysis
 * may attach the resulting capability requirements to the canonical quantum
 * semantic model and ultimately to:
 *
 *     quantum::ir
 *
 * without modifying this grammar.
 *
 * Classical, HDL, distributed, AI, networking, security, and other domains
 * follow the same principle.
 *
 * ============================================================================
 * COMPILER INTEGRATION
 * ============================================================================
 *
 * The compiler consumes the semantic capability model after parsing.
 *
 * Compilation may use capabilities for:
 *
 *     target discovery
 *     legality checking
 *     lowering
 *     optimization selection
 *     backend selection
 *     scheduling constraints
 *     routing constraints
 *     resource planning
 *     portability analysis
 *
 * This file does not select any backend.
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime capability availability is determined by the runtime/environment.
 *
 * This grammar does not:
 *
 *     discover hardware;
 *     query HAL;
 *     allocate resources;
 *     authorize operations;
 *     schedule work;
 *     route quantum operations.
 *
 * ============================================================================
 * CROSS-DOMAIN INTEGRATION
 * ============================================================================
 *
 * Capability declarations are intentionally universal.
 *
 * They may describe capabilities associated with:
 *
 *     classical computing
 *     quantum computing
 *     hybrid computing
 *     HDL
 *     hardware
 *     distributed computing
 *     HPC
 *     AI/ML
 *     data processing
 *     networking
 *     cryptography
 *     embedded systems
 *     edge systems
 *     cloud systems
 *     future computational paradigms
 *
 * Domain-specific semantics remain outside this declaration adapter.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar:
 *
 *     - contains no embedded Rust actions;
 *     - performs no I/O;
 *     - performs no network access;
 *     - performs no hardware discovery;
 *     - performs no random operations;
 *     - contains no semantic predicates.
 *
 * Parsing is deterministic for a deterministic token stream.
 *
 * ============================================================================
 * ERROR HANDLING
 * ============================================================================
 *
 * Malformed capability declarations are syntax errors.
 *
 * Semantic errors such as:
 *
 *     unknown capability
 *     duplicate incompatible capability
 *     unsupported version
 *     unavailable capability
 *
 * belong to later compiler phases.
 *
 * This file MUST NOT silently reinterpret malformed declarations as ordinary
 * identifiers or arbitrary expressions.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Capability declaration syntax MUST NOT grant authority.
 *
 * In particular:
 *
 *     capability privileged::operation;
 *
 * does not itself authorize privileged::operation.
 *
 * Authorization is evaluated by the security/runtime subsystem.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing canonical capability syntax remains owned by:
 *
 *     grammar/core/capabilities.g4
 *
 * This file is an integration layer and therefore should remain stable even
 * when new capability namespaces are introduced.
 *
 * Adding a new capability identity should normally require NO modification to
 * this file.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * PASS CONDITIONS:
 *
 *     - no finite capability enumeration;
 *     - no finite resource enumeration;
 *     - no hardware count;
 *     - no device count;
 *     - no qubit limit;
 *     - no CPU limit;
 *     - no GPU limit;
 *     - no FPGA limit;
 *     - no node limit;
 *     - no vendor-specific capability list;
 *     - no backend-specific syntax;
 *     - no quantum-gate enumeration;
 *     - no physical topology;
 *     - no physical IDs.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] canonical capability syntax is delegated;
 *     [x] no capability syntax is duplicated;
 *     [x] declaration-layer integration is explicit;
 *     [x] token vocabulary is canonical;
 *     [x] AST ownership is predetermined;
 *     [x] semantic ownership is predetermined;
 *     [x] IR ownership is predetermined;
 *     [x] compiler ownership is predetermined;
 *     [x] runtime ownership is predetermined;
 *     [x] cross-domain integration is defined;
 *     [x] no artificial hardware limits exist;
 *     [x] no physical resource selection exists;
 *     [x] deterministic parsing is preserved;
 *     [x] safe-Rust architecture is preserved.
 *
 * ============================================================================
 */

parser grammar CapabilityDeclarations;

options {
    tokenVocab = ZamaniLexer;
}

import Capabilities;


/*
 * ============================================================================
 * DECLARATION-LAYER ENTRY POINT
 * ============================================================================
 *
 * This is the only rule this file needs to expose to the universal declaration
 * dispatcher.
 *
 * The actual capability declaration is owned by:
 *
 *     grammar/core/capabilities.g4
 *
 * ============================================================================
 */

declarationCapability
    : capabilityDeclaration
    ;


/*
 * ============================================================================
 * ZERO-OR-MORE CAPABILITY DECLARATIONS
 * ============================================================================
 *
 * The repetition is deliberately unbounded by the grammar.
 *
 * Semantic validation may impose context-specific rules such as whether two
 * declarations conflict, but such rules do not belong here.
 * ============================================================================
 */

declarationCapabilities
    : declarationCapability*
    ;


/*
 * ============================================================================
 * ONE-OR-MORE CAPABILITY DECLARATIONS
 * ============================================================================
 *
 * Useful for declaration contexts that require at least one capability.
 * ============================================================================
 */

requiredDeclarationCapabilities
    : declarationCapability+
    ;


/*
 * ============================================================================
 * CAPABILITY DECLARATION LIST
 * ============================================================================
 *
 * A list form is provided for declaration contexts that need a syntactically
 * grouped capability list without redefining capability identity.
 *
 * Example conceptual form:
 *
 *     capabilities {
 *         capability zamani::compute::parallel;
 *         capability zamani::quantum::dynamic_control;
 *     }
 *
 * The enclosing keyword/block belongs to the caller unless this rule is
 * explicitly wired into the universal declaration grammar.
 *
 * ============================================================================
 */

capabilityDeclarationList
    : capabilityDeclaration
      (COMMA capabilityDeclaration)*
    ;


/*
 * ============================================================================
 * CAPABILITY DECLARATION REFERENCE
 * ============================================================================
 *
 * This adapter allows declaration-level consumers to refer to an already
 * canonical capability reference.
 *
 * It does not redeclare capabilityReference.
 * ============================================================================
 */

declarationCapabilityReference
    : capabilityReference
    ;


/*
 * ============================================================================
 * CAPABILITY REFERENCE LIST
 * ============================================================================
 *
 * There is no fixed number of references.
 * ============================================================================
 */

declarationCapabilityReferenceList
    : declarationCapabilityReference
      (COMMA declarationCapabilityReference)*
    ;
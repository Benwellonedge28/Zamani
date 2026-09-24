/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hybrid/hybrid-resources.g4
 *
 * ANTLR grammar identity:
 *     HybridResources
 *
 * NOTE:
 *     ANTLR requires a standalone grammar named `HybridResources` to be stored
 *     as `HybridResources.g4`. The existing hyphenated filename may be retained
 *     as a repository source artifact only if the build system stages/aliases
 *     it to the valid ANTLR filename.
 *
 * Status:
 *     Production-ready hybrid resource adapter.
 *
 * Baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     Safe Rust only.
 *     No unsafe Rust.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar is the HYBRID DOMAIN ADAPTER for the canonical resource
 * language.
 *
 * It does NOT create a second resource language.
 *
 * Universal resource syntax is owned by:
 *
 *     grammar/resources/resources.g4
 *
 * This file only exposes that canonical resource model through a stable
 * hybrid-domain boundary.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     hybridResourceConstruct
 *     hybridResourceDeclaration
 *     hybridRequirementDeclaration
 *     hybridConstraintDeclaration
 *     hybridPreferenceDeclaration
 *     hybridHintDeclaration
 *     hybridCapabilityDeclaration
 *     hybridTargetDeclaration
 *     hybridResourceReference
 *
 * These are HYBRID ADAPTER RULES.
 *
 * They provide stable names for the hybrid grammar without creating new
 * resource syntax.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     identifiers
 *     qualified names
 *     expressions
 *     types
 *     resource declarations
 *     resource requirements
 *     resource constraints
 *     resource preferences
 *     resource hints
 *     resource capabilities
 *     resource targets
 *     resource quantities
 *     capacities
 *     availability
 *     performance
 *     latency
 *     throughput
 *     bandwidth
 *     energy
 *     power
 *     reliability
 *     resilience
 *     cost
 *     reservation
 *     acquisition
 *     release
 *     derivation
 *     resource groups
 *     resource contracts
 *     resource profiles
 *
 * All of those remain owned by `Resources` and its subordinate canonical
 * resource grammars.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There is exactly one universal source-level resource grammar:
 *
 *     Resources
 *
 * Hybrid does not fork or specialize that grammar syntactically.
 *
 * The architecture is:
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniParser
 *          |
 *          v
 *     Hybrid
 *          |
 *          v
 *     HybridResources
 *          |
 *          v
 *     Resources
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic resource model
 *
 * This prevents:
 *
 *     hybrid resource syntax
 *     quantum resource syntax
 *     classical resource syntax
 *     hardware resource syntax
 *
 * from becoming competing resource languages.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * This adapter introduces NO finite hardware limits.
 *
 * It does not define:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_STORAGE
 *     MAX_ACCELERATORS
 *     MAX_REGISTER_WIDTH
 *     MAX_TENSOR_RANK
 *     MAX_NETWORK_SIZE
 *
 * Nor does it define finite counts of:
 *
 *     resource declarations
 *     capabilities
 *     requirements
 *     constraints
 *     preferences
 *     targets
 *     resources
 *
 * The canonical `Resources` grammar is responsible for those syntactic
 * categories, and it uses unbounded grammar repetition rather than artificial
 * machine capacities.
 *
 * ============================================================================
 * SEMANTIC DISTINCTIONS
 * ============================================================================
 *
 * The frontend semantic model MUST preserve the distinction between:
 *
 *     requirement
 *     constraint
 *     preference
 *     hint
 *     capability
 *     target
 *     resource
 *
 * For example:
 *
 *     requires capability("quantum.measurement");
 *
 * is not the same semantic object as:
 *
 *     prefer capability("quantum.measurement");
 *
 * and neither is equivalent to selecting a physical QPU.
 *
 * This grammar performs no such semantic resolution.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * Hybrid resources describe PROGRAM INTENT.
 *
 * They do not select:
 *
 *     cpu0
 *     gpu0
 *     fpga0
 *     qpu0
 *     physical_qubit0
 *     node0
 *     memory_bank0
 *
 * They also do not encode:
 *
 *     fixed topology
 *     physical addresses
 *     fixed device inventories
 *     fixed memory capacities
 *     fixed register widths
 *     fixed accelerator counts
 *
 * Target realization is downstream.
 *
 * ============================================================================
 * HYBRID DOMAIN MODEL
 * ============================================================================
 *
 * Hybrid computation may combine:
 *
 *     classical
 *     quantum
 *     HDL
 *     hardware
 *     accelerator
 *     distributed
 *     AI
 *     data
 *     networking
 *     security
 *     future domains
 *
 * The resource contract is therefore intentionally domain-neutral.
 *
 * A hybrid program can express requirements such as:
 *
 *     requires capability("quantum.measurement");
 *
 *     requires capability("tensor.compute");
 *
 *     requires memory >= required_memory;
 *
 *     requires qubits >= required_qubits;
 *
 * without changing the grammar when a new hardware class appears.
 *
 * ============================================================================
 * CANONICAL QUANTUM BOUNDARY
 * ============================================================================
 *
 * Quantum resource meaning is resolved downstream.
 *
 * This grammar MUST NOT create:
 *
 *     QubitId
 *     PhysicalQubitId
 *     QuantumGate
 *     QuantumInstruction
 *     QuantumCircuit
 *     QuantumRegister
 *
 * Quantum semantic lowering remains:
 *
 *     source
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     routing
 *       |
 *       v
 *     scheduling
 *       |
 *       v
 *     QEC / resilience / ZQN
 *       |
 *       v
 *     HAL
 *       |
 *       v
 *     target realization
 *
 * ============================================================================
 * RESOURCE / HARDWARE SEPARATION
 * ============================================================================
 *
 * This grammar describes:
 *
 *     WHAT is required
 *     WHAT is permitted
 *     WHAT is preferred
 *     WHAT capability is needed
 *     WHAT target class is acceptable
 *
 * It does not decide:
 *
 *     WHERE it runs
 *     WHICH device runs it
 *     WHICH physical qubit is used
 *     WHICH CPU core is selected
 *     WHICH GPU is selected
 *     HOW operations are routed
 *     WHEN operations execute
 *
 * Those decisions belong downstream to semantic analysis, resource
 * management, compilation, routing, scheduling, resilience, HAL and runtime.
 *
 * ============================================================================
 * NO SECOND IR
 * ============================================================================
 *
 * This grammar introduces no Hybrid IR.
 *
 * Hybrid source constructs lower through the existing domain-neutral frontend
 * AST and semantic model.
 *
 * Classical portions may lower to the canonical classical representation.
 *
 * Quantum portions lower through:
 *
 *     quantum::ir
 *
 * HDL/hardware portions lower through the existing hardware/HDL semantic
 * pipeline.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * This grammar depends ONLY on the canonical resource composition grammar:
 *
 *     Resources
 *
 * It deliberately does NOT import:
 *
 *     Expressions
 *     Types
 *     Core
 *     Hybrid
 *     Quantum
 *     Hardware
 *     ZamaniParser
 *
 * Why?
 *
 * `Resources` already composes its expression and name dependencies.
 *
 * Importing those grammars again here would create multiple dependency paths
 * and increase the possibility of rule collisions.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * All tokens come from:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This file defines NO lexer rules.
 *
 * It does not introduce:
 *
 *     K_*
 *
 * aliases,
 *
 * domain-specific lexer tokens,
 *
 * hardware identifiers,
 *
 * quantum gate tokens,
 *
 * or parser-local token definitions.
 *
 * ============================================================================
 */

parser grammar HybridResources;

options {
    tokenVocab = ZamaniLexer;
}

import Resources;


/*
 * ============================================================================
 * PUBLIC HYBRID RESOURCE ENTRY POINT
 * ============================================================================
 *
 * The hybrid domain receives exactly the same canonical resource syntax as
 * every other Zamani domain.
 *
 * This is intentional.
 *
 * A resource construct is a semantic resource construct first and a hybrid
 * construct second.
 */
hybridResourceConstruct
    : resourceItem
    ;


/*
 * ============================================================================
 * RESOURCE DECLARATION ADAPTER
 * ============================================================================
 *
 * Canonical owner:
 *
 *     Resources.resourceDeclaration
 *
 * No syntax is duplicated here.
 */
hybridResourceDeclaration
    : resourceDeclaration
    ;


/*
 * ============================================================================
 * REQUIREMENT ADAPTER
 * ============================================================================
 *
 * Canonical owner:
 *
 *     Resources.resourceRequirement
 *
 * Hybrid semantics are established downstream from the same canonical
 * requirement representation.
 */
hybridRequirementDeclaration
    : resourceRequirement
    ;


/*
 * ============================================================================
 * CONSTRAINT ADAPTER
 * ============================================================================
 *
 * Canonical owner:
 *
 *     Resources.resourceConstraint
 */
hybridConstraintDeclaration
    : resourceConstraint
    ;


/*
 * ============================================================================
 * PREFERENCE ADAPTER
 * ============================================================================
 *
 * Canonical owner:
 *
 *     Resources.resourcePreference
 */
hybridPreferenceDeclaration
    : resourcePreference
    ;


/*
 * ============================================================================
 * HINT ADAPTER
 * ============================================================================
 *
 * Canonical owner:
 *
 *     Resources.resourceHint
 */
hybridHintDeclaration
    : resourceHint
    ;


/*
 * ============================================================================
 * CAPABILITY ADAPTER
 * ============================================================================
 *
 * Canonical owner:
 *
 *     Resources.resourceCapability
 *
 * Capability names remain open-world semantic names.
 *
 * No finite capability registry is encoded in this grammar.
 */
hybridCapabilityDeclaration
    : resourceCapability
    ;


/*
 * ============================================================================
 * TARGET ADAPTER
 * ============================================================================
 *
 * Canonical owner:
 *
 *     Resources.resourceTarget
 *
 * A target here is an abstract target expression, not a physical device.
 */
hybridTargetDeclaration
    : resourceTarget
    ;


/*
 * ============================================================================
 * RESOURCE REFERENCE ADAPTER
 * ============================================================================
 *
 * Resource references are deliberately kept as a thin adapter to the
 * canonical resource-reference clause.
 *
 * This is not a physical allocation operation.
 */
hybridResourceReference
    : resourceReferenceClause
    ;


/*
 * ============================================================================
 * COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete when:
 *
 *   [x] no universal resource syntax is duplicated;
 *   [x] no identifier grammar is duplicated;
 *   [x] no qualified-name grammar is duplicated;
 *   [x] no expression grammar is duplicated;
 *   [x] no type grammar is duplicated;
 *   [x] no lexer tokens are defined;
 *   [x] no obsolete K_* tokens are referenced;
 *   [x] no undefined hybrid-only resource keywords are referenced;
 *   [x] no fixed hardware capacity is encoded;
 *   [x] no physical device selection is encoded;
 *   [x] no routing is encoded;
 *   [x] no scheduling is encoded;
 *   [x] no QEC implementation is encoded;
 *   [x] no ZQN implementation is encoded;
 *   [x] no second quantum IR is introduced;
 *   [x] canonical Resources remains the sole resource syntax owner;
 *   [x] canonical ZamaniLexer remains the sole lexer boundary;
 *   [x] semantic resource distinctions remain downstream;
 *   [x] POCO-REAF is preserved.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * `Hybrid` MAY consume:
 *
 *     hybridResourceConstruct
 *
 * when the canonical hybrid dispatcher is ready to expose resource constructs
 * directly.
 *
 * However, `Hybrid` MUST NOT simultaneously define another competing
 * resource grammar for the same syntax.
 *
 * The preferred future composition is:
 *
 *     hybridConstruct
 *         |
 *         +--> hybrid computation constructs
 *         |
 *         +--> hybridResourceConstruct
 *                    |
 *                    v
 *                resourceItem
 *                    |
 *                    v
 *                 Resources
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The adapter does not create new AST categories.
 *
 * The semantic frontend should retain the canonical resource AST/model
 * generated from:
 *
 *     resourceDeclaration
 *     resourceRequirement
 *     resourceConstraint
 *     resourcePreference
 *     resourceHint
 *     resourceCapability
 *     resourceTarget
 *     resourceReferenceClause
 *
 * If hybrid provenance is required, it should be represented as semantic
 * domain context/metadata rather than a second resource-node hierarchy.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for determining:
 *
 *     resource-name validity
 *     capability validity
 *     target compatibility
 *     expression type compatibility
 *     requirement satisfiability
 *     constraint consistency
 *     preference applicability
 *     hint applicability
 *     hybrid domain compatibility
 *     resource availability
 *
 * Parsing performs none of these checks.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * Resource intent must be lowered into the existing canonical semantic
 * resource/capability model.
 *
 * This file must never cause:
 *
 *     HybridIR
 *     HybridResourceIR
 *     HybridQuantumIR
 *
 * to be introduced.
 *
 * Quantum requirements continue toward:
 *
 *     quantum::ir
 *
 * only after semantic analysis.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This adapter has no language-level finite resource bound.
 *
 * The number, magnitude, or dimensionality of resources is represented by
 * program expressions and semantic resource models.
 *
 * Therefore:
 *
 *     tiny resource
 *     large resource
 *     distributed resource
 *     accelerator resource
 *     quantum resource
 *     future resource
 *
 * use the same grammar.
 *
 * "Infinity" means no artificial language-level maximum is encoded here.
 *
 * Actual execution remains bounded by available implementation resources and
 * target capabilities.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive:
 *
 *     resource compute;
 *     requires capability("quantum.measurement");
 *     requires capability("gpu.compute");
 *     prefer capability("tensor.compute");
 *     hint scalable;
 *     target = quantum;
 *
 * Negative:
 *
 *     unknown parser-local resource keyword;
 *     malformed resource expression;
 *     malformed resource declaration;
 *     malformed capability expression;
 *     malformed target expression.
 *
 * Boundary:
 *
 *     symbolic resource quantity;
 *     arbitrarily large numeric program value;
 *     deeply qualified capability name;
 *     multiple resource requirements;
 *     multiple capabilities;
 *     multiple target requirements.
 *
 * Scalability:
 *
 *     no test establishes a maximum number of resources;
 *     no test establishes a maximum number of qubits;
 *     no test establishes a maximum number of devices;
 *     no test establishes a maximum memory size.
 *
 * Determinism:
 *
 *     identical source + identical grammar/version must produce identical
 *     parse structure independent of hardware availability.
 *
 * ============================================================================
 * SAFETY CONTRACT
 * ============================================================================
 *
 * The grammar contains:
 *
 *     no embedded Rust;
 *     no actions;
 *     no semantic predicates;
 *     no filesystem operations;
 *     no network operations;
 *     no hardware access;
 *     no environment inspection;
 *     no randomness.
 *
 * Rust 1.97 / 1.97.1 integration remains safe Rust only.
 *
 * ============================================================================
 */
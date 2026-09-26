/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/resources/negotiation.g4
 *
 * GRAMMAR
 * -------
 * ResourceNegotiation
 *
 * STATUS
 * ------
 * CANONICAL RESOURCE-NEGOTIATION LEAF GRAMMAR
 *
 * BASELINE
 * --------
 * Rust 1.97 / Rust 1.97.1
 * Rust 2021
 *
 * SAFETY
 * ------
 * Grammar-only.
 *
 * This grammar contains:
 *
 *   - no embedded Rust;
 *   - no parser actions;
 *   - no semantic predicates;
 *   - no filesystem access;
 *   - no network access;
 *   - no hardware discovery;
 *   - no resource allocation;
 *   - no runtime execution;
 *   - no unsafe Rust requirement.
 *
 * ============================================================================
 * 1. PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-SYNTAX LEAF CONTRACT for resource negotiation.
 *
 * Negotiation describes declarative intent concerning how resource
 * requirements, capabilities, constraints, preferences, alternatives,
 * fallbacks, and realization policies may be reconciled by downstream
 * resource/compilation infrastructure.
 *
 * This grammar does NOT perform negotiation.
 *
 * It does NOT:
 *
 *   - discover resources;
 *   - contact resource providers;
 *   - allocate resources;
 *   - reserve resources;
 *   - select physical devices;
 *   - perform placement;
 *   - perform routing;
 *   - perform scheduling;
 *   - execute fallback logic;
 *   - inspect hardware;
 *   - access the network;
 *   - access credentials;
 *   - choose a vendor;
 *   - choose a CPU/GPU/FPGA/QPU;
 *   - modify quantum::ir;
 *   - implement QEC;
 *   - implement ZQN;
 *   - implement HAL behavior.
 *
 * It only describes the declarative negotiation information that later
 * compiler/resource/runtime layers may consume.
 *
 * ============================================================================
 * 2. ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         Zamani source
 *                              |
 *                              v
 *                         ZamaniLexer
 *                              |
 *                              v
 *                       ZamaniParser
 *                              |
 *                              v
 *                          Resources
 *                              |
 *                 +------------+-------------+
 *                 |                          |
 *                 v                          v
 *       ResourceExpressions             ResourceNegotiation
 *                 |                          |
 *                 +------------+-------------+
 *                              |
 *                              v
 *                       Domain-neutral AST
 *                              |
 *                              v
 *                      Semantic analysis
 *                              |
 *             +----------------+----------------+
 *             |                |                |
 *             v                v                v
 *       requirements       capabilities     preferences
 *             |                |                |
 *             +----------------+----------------+
 *                              |
 *                              v
 *                   Resource negotiation model
 *                              |
 *                  capability/resource discovery
 *                              |
 *                  realization negotiation
 *                              |
 *             +----------------+----------------+
 *             |                |                |
 *             v                v                v
 *         classical        quantum::ir      HDL/hardware
 *             |                |                |
 *             +----------------+----------------+
 *                              |
 *                     optimization/lowering
 *                              |
 *                  routing / scheduling / QEC
 *                              |
 *                             ZQN
 *                              |
 *                             HAL
 *                              |
 *                       target realization
 *
 * ============================================================================
 * 3. POCO-REAF CONTRACT
 * ============================================================================
 *
 * Negotiation is a key part of:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * A portable program can describe what it can accept without embedding a
 * machine-specific realization.
 *
 * For example, semantic information may eventually represent:
 *
 *     required capability
 *     acceptable capability alternatives
 *     preferred capability
 *     resource requirement
 *     realization constraint
 *     fallback
 *     substitution
 *     compatibility
 *     scalability policy
 *
 * without specifying:
 *
 *     CPU 0
 *     GPU 3
 *     FPGA 1
 *     QPU 0
 *     physical qubit 17
 *     memory bank 2
 *     cloud instance type X
 *
 * The grammar remains target-independent.
 *
 * ============================================================================
 * 4. HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This grammar MUST NOT define or imply:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * It also MUST NOT encode equivalent fixed capacities.
 *
 * In particular, this grammar contains no finite enumeration of:
 *
 *     CPUs
 *     GPUs
 *     FPGAs
 *     ASICs
 *     QPUs
 *     qubits
 *     nodes
 *     devices
 *     memory banks
 *     accelerators
 *     providers
 *     negotiation participants
 *     alternatives
 *     fallback paths
 *     negotiation clauses
 *     negotiation rounds
 *
 * Numeric values are program values or expressions.
 *
 * They are not universal implementation limits.
 *
 * ============================================================================
 * 5. SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     resourceNegotiationSpecification
 *     resourceNegotiationClause
 *     resourceNegotiationAssignment
 *     resourceNegotiationProperty
 *     resourceNegotiationValue
 *     resourceNegotiationGroup
 *     resourceNegotiationGroupEntry
 *     resourceNegotiationList
 *     optionalResourceNegotiationSpecification
 *     resourceNegotiationPropertyPath
 *     resourceNegotiationPropertySegment
 *
 * THIS FILE DOES NOT OWN:
 *
 *     resource
 *     resourceItem
 *     resourceRequirement
 *     resourceConstraint
 *     resourcePreference
 *     resourceHint
 *     resourceCapability
 *     resourceExpression
 *     expression
 *     identifier
 *     qualifiedName
 *     type syntax
 *     target selection
 *     placement
 *     routing
 *     scheduling
 *     allocation
 *     reservation
 *     hardware discovery
 *     quantum::ir
 *     classical IR
 *     HDL IR
 *     QEC
 *     ZQN
 *     HAL
 *     runtime behavior
 *
 * Existing concrete resource categories remain owned by their existing files.
 *
 * In particular:
 *
 *     requirements.g4
 *     constraints.g4
 *     preferences.g4
 *     hints.g4
 *     capabilities.g4
 *
 * remain independent semantic categories.
 *
 * Negotiation coordinates those categories; it does not redefine them.
 *
 * ============================================================================
 * 6. IMPORTANT INTEGRATION DECISION
 * ============================================================================
 *
 * The current repository does NOT expose a canonical NEGOTIATE lexer token.
 *
 * Therefore this file intentionally does NOT define or consume:
 *
 *     NEGOTIATE
 *
 * as a lexer token.
 *
 * Adding a lexer token here would violate lexical ownership.
 *
 * Instead, this grammar exposes:
 *
 *     resourceNegotiationSpecification
 *
 * as the reusable payload boundary.
 *
 * A parent grammar may later provide the concrete source-level introducer and
 * delegate its payload to:
 *
 *     resourceNegotiationSpecification
 *
 * without changing this file.
 *
 * This makes the leaf independently complete.
 *
 * ============================================================================
 * 7. IMPORTS
 * ============================================================================
 *
 * ResourceExpressions:
 *
 *     grammar/resources/resource-expressions.g4
 *
 * owns:
 *
 *     resourceExpression
 *
 * and therefore provides the canonical expression architecture.
 *
 * Names:
 *
 *     grammar/core/names.g4
 *
 * owns:
 *
 *     identifier
 *     qualifiedName
 *
 * No local expression or identifier grammar is created here.
 *
 * ============================================================================
 */

parser grammar ResourceNegotiation;

options {
    tokenVocab = ZamaniLexer;
}

import ResourceExpressions, Names;


/*
 * ============================================================================
 * 8. PUBLIC NEGOTIATION SPECIFICATION
 * ============================================================================
 *
 * This is the primary public entry point.
 *
 * A parent resource grammar can attach a source-level negotiation introducer
 * and then consume:
 *
 *     resourceNegotiationSpecification
 *
 * Example conceptual source:
 *
 *     negotiate {
 *         mode = capability;
 *         fallback = alternate;
 *         ...
 *     };
 *
 * The introducer itself is intentionally NOT owned here because no canonical
 * NEGOTIATE lexer token currently exists.
 *
 * ============================================================================
 */

resourceNegotiationSpecification
    : LBRACE
      resourceNegotiationClause*
      RBRACE
    ;


/*
 * ============================================================================
 * 9. NEGOTIATION CLAUSE
 * ============================================================================
 *
 * Negotiation clauses are open-world.
 *
 * This is intentional.
 *
 * New negotiation dimensions must not require a new global keyword merely
 * because a future resource domain introduces a new concept.
 *
 * Examples of semantic properties include:
 *
 *     mode
 *     strategy
 *     scope
 *     required
 *     acceptable
 *     preferred
 *     fallback
 *     substitution
 *     compatibility
 *     priority
 *     weight
 *     timeout
 *     retry
 *     scalability
 *     portability
 *     availability
 *     capability
 *     target
 *     policy
 *
 * The grammar treats these as names.
 *
 * Semantic analysis determines their meaning.
 *
 * ============================================================================
 */

resourceNegotiationClause
    : resourceNegotiationAssignment
    | resourceNegotiationGroup
    ;


/*
 * ============================================================================
 * 10. GENERIC NEGOTIATION ASSIGNMENT
 * ============================================================================
 *
 * Canonical form:
 *
 *     property = value;
 *
 * Examples:
 *
 *     mode = capability;
 *     strategy = adaptive;
 *     priority = preference_priority;
 *     fallback = alternate;
 *     scalability = workload_size;
 *
 * The grammar does not impose a finite vocabulary.
 *
 * ============================================================================
 */

resourceNegotiationAssignment
    : resourceNegotiationProperty
      ASSIGN
      resourceNegotiationValue
      SEMICOLON
    ;


/*
 * ============================================================================
 * 11. NEGOTIATION PROPERTY
 * ============================================================================
 *
 * Properties are open-world and may be qualified.
 *
 * Examples:
 *
 *     mode
 *     strategy
 *     capability
 *     fallback
 *     compatibility
 *     resource::capacity
 *     capability::quantum::measurement
 *     quantum::fidelity
 *     hardware::memory
 *     network::bandwidth
 *     vendor::future::policy
 *
 * Namespace depth is unbounded by the language architecture.
 *
 * The parser does not determine whether a property is:
 *
 *     standard
 *     experimental
 *     dialect-specific
 *     vendor-specific
 *     unknown
 *     deprecated
 *
 * That is semantic analysis.
 *
 * ============================================================================
 */

resourceNegotiationProperty
    : resourceNegotiationPropertyPath
    ;


resourceNegotiationPropertyPath
    : resourceNegotiationPropertySegment
      (
          DOT resourceNegotiationPropertySegment
        | DOUBLE_COLON resourceNegotiationPropertySegment
      )*
    ;


resourceNegotiationPropertySegment
    : identifier
    ;


/*
 * ============================================================================
 * 12. NEGOTIATION VALUE
 * ============================================================================
 *
 * Values reuse canonical resource expressions.
 *
 * This allows negotiation to refer to:
 *
 *     symbolic quantities
 *     capabilities
 *     comparisons
 *     arithmetic
 *     logical expressions
 *     function calls
 *     indexed resources
 *     properties
 *     ranges
 *     dynamic values
 *     target-independent expressions
 *
 * without creating a second expression language.
 *
 * ============================================================================
 */

resourceNegotiationValue
    : resourceExpression
    ;


/*
 * ============================================================================
 * 13. NEGOTIATION GROUP
 * ============================================================================
 *
 * A group allows related negotiation clauses to be represented as a semantic
 * unit.
 *
 * The grammar intentionally does not define a finite set of group names.
 *
 * A parent grammar may provide a group name or use this body as a reusable
 * nested specification.
 *
 * ============================================================================
 */

resourceNegotiationGroup
    : resourceNegotiationGroupEntry
    ;


resourceNegotiationGroupEntry
    : resourceNegotiationProperty
      resourceNegotiationGroupBody
    ;


resourceNegotiationGroupBody
    : LBRACE
      resourceNegotiationClause*
      RBRACE
    ;


/*
 * ============================================================================
 * 14. NEGOTIATION LIST
 * ============================================================================
 *
 * Reusable unbounded collection.
 *
 * No maximum number of negotiation clauses is encoded.
 *
 * ============================================================================
 */

resourceNegotiationList
    : resourceNegotiationClause*
    ;


optionalResourceNegotiationSpecification
    : resourceNegotiationSpecification?
    ;


/*
 * ============================================================================
 * 15. SEMANTIC CATEGORIES
 * ============================================================================
 *
 * The parser deliberately does not turn negotiation concepts into separate
 * grammar keywords.
 *
 * Semantic analysis MAY classify properties into categories such as:
 *
 *     requirement
 *     capability
 *     preference
 *     constraint
 *     alternative
 *     fallback
 *     substitution
 *     compatibility
 *     priority
 *     policy
 *     availability
 *     portability
 *     scalability
 *     target intent
 *
 * This classification must preserve the distinction between the underlying
 * resource categories.
 *
 * For example:
 *
 *     requires capability("quantum.measurement")
 *
 * remains a requirement.
 *
 * A negotiation policy describing how that requirement may be satisfied does
 * not convert the requirement into a preference.
 *
 * Likewise:
 *
 *     prefer latency <= latency_budget;
 *
 * remains a preference.
 *
 * Negotiation does not change its semantic category.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 16. REQUIREMENT NEGOTIATION
 * ============================================================================
 *
 * Negotiation may describe acceptable ways of satisfying requirements.
 *
 * Examples of semantic intent:
 *
 *     required capability
 *     alternative capability
 *     equivalent resource
 *     fallback realization
 *
 * The grammar intentionally does not decide equivalence.
 *
 * Example conceptual value:
 *
 *     acceptable = capability("quantum.measurement");
 *
 * or:
 *
 *     acceptable = capability("quantum.measurement")
 *                  || capability("quantum.simulated_measurement");
 *
 * The actual expression semantics are supplied by ResourceExpressions and
 * semantic analysis.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 17. CAPABILITY NEGOTIATION
 * ============================================================================
 *
 * Capability names remain open-world.
 *
 * Examples:
 *
 *     capability = "quantum.measurement";
 *     capability = "gpu.compute";
 *     capability = "tensor.compute";
 *     capability = "hdl.synthesis";
 *
 * The grammar does not enumerate capability names.
 *
 * Capability discovery is downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 18. ALTERNATIVES
 * ============================================================================
 *
 * A negotiation may describe alternative realizations.
 *
 * The number of alternatives is unbounded at the language level because
 * alternatives are represented by expressions, lists, groups, or nested
 * negotiation structures.
 *
 * No fixed number of alternatives is permitted.
 *
 * Semantic analysis determines:
 *
 *     equivalence;
 *     compatibility;
 *     ordering;
 *     feasibility;
 *     substitution safety.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 19. FALLBACK
 * ============================================================================
 *
 * Fallback is declarative intent.
 *
 * It does NOT execute a fallback.
 *
 * A downstream compiler/runtime may use the information to construct an
 * implementation plan.
 *
 * Examples of semantic values include:
 *
 *     fallback = alternate;
 *     fallback = software;
 *     fallback = simulation;
 *     fallback = compatible_capability;
 *
 * These remain symbolic until semantic/resource analysis.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 20. SUBSTITUTION
 * ============================================================================
 *
 * Negotiation may describe that one resource/capability category can be
 * substituted by another semantic alternative.
 *
 * The grammar does not decide whether substitution is sound.
 *
 * Soundness belongs to semantic analysis and domain-specific compatibility
 * rules.
 *
 * This is particularly important for:
 *
 *     quantum
 *     classical
 *     hybrid
 *     HDL
 *     hardware
 *     accelerator
 *     distributed
 *     networking
 *     AI
 *     data
 *
 * domains.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 21. PRIORITY AND WEIGHT
 * ============================================================================
 *
 * Priority and weight are advisory negotiation metadata unless the semantic
 * specification explicitly assigns another category.
 *
 * They MUST NOT silently turn:
 *
 *     preference
 *
 * into:
 *
 *     requirement
 *
 * or:
 *
 *     constraint.
 *
 * Values remain expressions rather than fixed-width integers.
 *
 * Therefore the grammar does not impose:
 *
 *     maximum priority
 *     maximum weight
 *     fixed number of levels
 *     fixed numeric width
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 22. NEGOTIATION STRATEGY
 * ============================================================================
 *
 * A strategy may be represented symbolically:
 *
 *     strategy = adaptive;
 *     strategy = capability_first;
 *     strategy = preference_first;
 *     strategy = portability_first;
 *     strategy = cost_aware;
 *
 * These are semantic values, not parser-enforced algorithms.
 *
 * The grammar does not implement a negotiation algorithm.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 23. AVAILABILITY
 * ============================================================================
 *
 * Negotiation may refer to availability information:
 *
 *     availability
 *     capacity
 *     free_memory
 *     available_parallelism
 *     provider_capacity
 *
 * The grammar does not inspect those values.
 *
 * They are supplied by downstream resource discovery, compilation, deployment,
 * runtime, or HAL infrastructure.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 24. SCALABILITY
 * ============================================================================
 *
 * Negotiation must support source programs whose resource demands vary with:
 *
 *     input size;
 *     workload size;
 *     problem size;
 *     data size;
 *     qubit count;
 *     tensor dimensions;
 *     node availability;
 *     accelerator availability;
 *     execution environment.
 *
 * Therefore values must remain expressions.
 *
 * Valid semantic examples include:
 *
 *     required = workload_size;
 *     required = logical_qubits;
 *     required = tensor_elements;
 *     required = required_nodes;
 *
 * The grammar imposes no maximum.
 *
 * "Infinity" means unbounded by language-level negotiation cardinality.
 *
 * It does not claim infinite physical resources.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 25. RESOURCE AVAILABILITY VS RESOURCE REQUIREMENT
 * ============================================================================
 *
 * Negotiation must preserve this distinction:
 *
 *     requirement
 *
 * describes what must be satisfied.
 *
 *     availability
 *
 * describes what a realization may currently provide.
 *
 *     negotiation
 *
 * describes how acceptable matches may be determined.
 *
 * The parser does not determine whether a match exists.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 26. REQUIREMENT / CONSTRAINT / PREFERENCE / HINT BOUNDARY
 * ============================================================================
 *
 * Existing resource grammars remain authoritative:
 *
 *     requirements.g4
 *     constraints.g4
 *     preferences.g4
 *     hints.g4
 *
 * Negotiation does not replace those categories.
 *
 * The semantic model must preserve:
 *
 *     requirement != constraint
 *     constraint != preference
 *     preference != hint
 *     negotiation != realization
 *
 * Negotiation can coordinate these categories without collapsing them.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 27. TARGET INDEPENDENCE
 * ============================================================================
 *
 * Negotiation syntax must not select physical targets.
 *
 * These are NOT universal negotiation constructs:
 *
 *     cpu(0)
 *     gpu(0)
 *     qpu(0)
 *     physical_qubit(0)
 *     memory_bank(0)
 *     node(0)
 *
 * A target-specific realization, when explicitly permitted by the language,
 * belongs downstream to target/resource/placement semantics and must remain
 * distinguishable from portable negotiation intent.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 28. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum negotiation may concern:
 *
 *     qubit resources;
 *     logical-qubit resources;
 *     measurement capability;
 *     dynamic-circuit capability;
 *     coherence-related properties;
 *     fidelity;
 *     error characteristics;
 *     latency;
 *     throughput;
 *     topology capability;
 *     error-correction capability;
 *     simulator fallback;
 *     hardware availability.
 *
 * This grammar does not implement any of these.
 *
 * The downstream path remains:
 *
 *     quantum source
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic resource model
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing / scheduling / resilience
 *          |
 *          v
 *     QEC / ZQN
 *          |
 *          v
 *     HAL
 *
 * No second quantum IR is introduced.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 29. CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical negotiation may concern:
 *
 *     CPU capability;
 *     vector capability;
 *     accelerator capability;
 *     memory;
 *     parallelism;
 *     latency;
 *     throughput;
 *     energy;
 *     portability;
 *     scalability.
 *
 * The grammar remains independent of CPU architecture and instruction-set
 * details.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 30. GPU / FPGA / ACCELERATOR INTEGRATION
 * ============================================================================
 *
 * Negotiation may express semantic capability requirements such as:
 *
 *     tensor.compute
 *     accelerator.compute
 *     hdl.synthesis
 *
 * without enumerating vendors or device models.
 *
 * Vendor-specific properties may use qualified names:
 *
 *     vendor::family::property
 *
 * rather than introducing global parser keywords.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 31. HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Negotiation may concern:
 *
 *     synthesis capability;
 *     timing capability;
 *     memory capability;
 *     accelerator capability;
 *     interconnect capability;
 *     reliability;
 *     power;
 *     thermal properties;
 *     deployment compatibility.
 *
 * It must not encode:
 *
 *     fixed register widths;
 *     fixed FPGA LUT counts;
 *     fixed BRAM counts;
 *     fixed clock counts;
 *     fixed ASIC structures;
 *     fixed physical placement.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 32. DISTRIBUTED / NETWORKING INTEGRATION
 * ============================================================================
 *
 * Negotiation may concern:
 *
 *     communication capability;
 *     bandwidth;
 *     latency;
 *     locality;
 *     availability;
 *     replication capability;
 *     consistency capability;
 *     fault tolerance;
 *     network resilience.
 *
 * Node count and topology size remain target/resource data.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 33. AI / DATA INTEGRATION
 * ============================================================================
 *
 * Negotiation may concern:
 *
 *     tensor computation;
 *     model execution;
 *     accelerator availability;
 *     memory;
 *     data movement;
 *     training;
 *     inference;
 *     distributed execution;
 *     throughput;
 *     latency;
 *     energy.
 *
 * Framework-specific semantics do not belong in this universal grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 34. NESTED NEGOTIATION
 * ============================================================================
 *
 * Nested groups are supported recursively.
 *
 * Example conceptual structure:
 *
 *     {
 *         strategy = adaptive;
 *
 *         capability {
 *             required = capability("tensor.compute");
 *             fallback = capability("classical.compute");
 *         }
 *
 *         portability {
 *             preferred = portable;
 *             fallback = compatible;
 *         }
 *     }
 *
 * The grammar imposes no nesting-depth limit.
 *
 * Practical parser-stack/resource protections remain implementation concerns,
 * not language semantics.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 35. DETERMINISM
 * ============================================================================
 *
 * Parsing is deterministic.
 *
 * The same source under the same language version produces the same syntax
 * tree.
 *
 * Negotiation outcome is NOT parser behavior.
 *
 * It must be determined by semantic/resource/optimization infrastructure using
 * explicit policy and target information.
 *
 * Parser alternative ordering must never become an implicit negotiation policy.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 36. CONFLICTS
 * ============================================================================
 *
 * Negotiation permits declarations that may later prove semantically
 * incompatible.
 *
 * For example, a program may describe:
 *
 *     a required capability;
 *     an unavailable preferred capability;
 *     multiple fallback paths;
 *     conflicting optimization objectives.
 *
 * The grammar must still parse such source if its syntax is valid.
 *
 * Semantic/resource analysis determines:
 *
 *     satisfiable;
 *     unsatisfiable;
 *     partially satisfiable;
 *     conflicting;
 *     unsupported;
 *     requiring fallback.
 *
 * These are not parser errors.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 37. FAILURE MODEL
 * ============================================================================
 *
 * The compiler/runtime must distinguish at least:
 *
 *     malformed syntax
 *     invalid semantic negotiation
 *     unsatisfied requirement
 *     unavailable capability
 *     incompatible realization
 *     unavailable target
 *     exhausted deployment resources
 *     runtime resource failure
 *
 * A resource negotiation failure must not be reported as a grammar failure
 * merely because no target can satisfy it.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 38. AST CONTRACT
 * ============================================================================
 *
 * The domain-neutral frontend AST should preserve, at minimum:
 *
 *     negotiation source span;
 *     ordered clauses;
 *     property path;
 *     value expression;
 *     nested groups;
 *     source locations;
 *     attributes/metadata when supplied by the parent grammar.
 *
 * The AST must NOT contain:
 *
 *     physical CPU IDs;
 *     physical GPU IDs;
 *     physical FPGA IDs;
 *     physical QPU IDs;
 *     physical qubit IDs;
 *     provider credentials;
 *     hardware discovery results;
 *     runtime allocation handles.
 *
 * A generic negotiation representation is preferred so that new domains do not
 * require a new AST hierarchy.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 39. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     property classification;
 *     property registration;
 *     dialect validation;
 *     value typing;
 *     dimensional/unit checking;
 *     capability interpretation;
 *     requirement interpretation;
 *     preference interpretation;
 *     fallback validity;
 *     substitution validity;
 *     compatibility checking;
 *     conflict detection;
 *     negotiation-policy validation;
 *     resource feasibility.
 *
 * Semantic analysis may consult target information only after the portable
 * source structure has been established.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 40. RESOURCE MODEL CONTRACT
 * ============================================================================
 *
 * The semantic resource model should distinguish:
 *
 *     requirement
 *     constraint
 *     capability
 *     preference
 *     hint
 *     negotiation policy
 *     realization
 *
 * Negotiation must not erase the source category that generated a policy.
 *
 * Example:
 *
 *     requires qubits >= logical_qubits;
 *
 * remains a requirement even if negotiation later identifies:
 *
 *     hardware realization A
 *
 * or:
 *
 *     hardware realization B.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 41. IR CONTRACT
 * ============================================================================
 *
 * This grammar defines NO IR.
 *
 * Negotiation information is lowered by semantic/resource analysis into the
 * repository's canonical resource-intent representation.
 *
 * It may influence:
 *
 *     target selection;
 *     specialization;
 *     optimization;
 *     placement;
 *     routing;
 *     scheduling;
 *     resilience;
 *     deployment;
 *     runtime policy.
 *
 * It must not create:
 *
 *     a negotiation IR;
 *     a second quantum IR;
 *     a vendor-specific frontend IR.
 *
 * Quantum computation continues to use:
 *
 *     quantum::ir
 *
 * as the canonical quantum boundary.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 42. COMPILER INTEGRATION
 * ============================================================================
 *
 * The compiler consumes negotiation semantics after parsing.
 *
 * Conceptual pipeline:
 *
 *     source
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     semantic/resource analysis
 *       |
 *       v
 *     negotiation policy
 *       |
 *       v
 *     capability/resource discovery
 *       |
 *       v
 *     candidate realization set
 *       |
 *       v
 *     optimization / target lowering
 *
 * Candidate generation and selection are compiler/resource concerns.
 *
 * This grammar never performs candidate selection.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 43. RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime infrastructure may consume already-lowered negotiation policy for:
 *
 *     dynamic resource availability;
 *     fallback;
 *     recovery;
 *     deployment;
 *     elasticity;
 *     heterogeneous execution;
 *     resource replacement.
 *
 * The parser must never perform these actions.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 44. HAL INTEGRATION
 * ============================================================================
 *
 * HAL may provide actual capabilities and resource state.
 *
 * Negotiation syntax is upstream of HAL.
 *
 * Therefore this grammar does not know:
 *
 *     device inventory;
 *     physical topology;
 *     calibration;
 *     qubit connectivity;
 *     driver state;
 *     power state;
 *     thermal state;
 *     memory availability.
 *
 * Those are discovered downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 45. SECURITY CONTRACT
 * ============================================================================
 *
 * Negotiation syntax is declarative and side-effect free.
 *
 * It must not:
 *
 *     inspect credentials;
 *     load secrets;
 *     contact providers;
 *     access the network;
 *     execute provider code;
 *     discover hardware;
 *     allocate resources.
 *
 * Authorization, trust, credentials, provider selection, and secure
 * negotiation protocols belong to the appropriate semantic/runtime/security
 * layers.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 46. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Because properties are open-world:
 *
 *     adding a new property
 *
 * does not require changing this grammar.
 *
 * Standardized properties remain compatibility-sensitive.
 *
 * Changing the meaning of an established property requires the language's
 * versioning/deprecation machinery.
 *
 * Dialect/vendor properties should preferably use qualified names:
 *
 *     dialect::name::property
 *     vendor::name::property
 *
 * rather than global lexer keywords.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 47. PERFORMANCE CONTRACT
 * ============================================================================
 *
 * The grammar intentionally uses a small structural core:
 *
 *     property = expression ;
 *
 * and:
 *
 *     property { ... }
 *
 * Qualified properties use a repeated local separator structure.
 *
 * No semantic lookup occurs while parsing.
 *
 * No capability discovery occurs while parsing.
 *
 * No target discovery occurs while parsing.
 *
 * No network operation occurs while parsing.
 *
 * No negotiation algorithm occurs while parsing.
 *
 * This keeps parser complexity independent of the number of hardware vendors,
 * targets, capabilities, resources, or providers known to the implementation.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 48. SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar uses:
 *
 *     *
 *     recursive groups
 *     expression composition
 *     qualified-name repetition
 *
 * rather than fixed cardinalities.
 *
 * Therefore the language does not define maximum counts for:
 *
 *     negotiation clauses;
 *     alternatives;
 *     fallback paths;
 *     groups;
 *     nested groups;
 *     properties;
 *     participants;
 *     capabilities;
 *     resources;
 *     targets;
 *     nodes;
 *     devices;
 *     qubits;
 *     processors;
 *     accelerators.
 *
 * Any practical limit is an implementation/resource-management concern.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 49. TEST CONTRACT
 * ============================================================================
 *
 * The resource negotiation test suite should contain:
 *
 * POSITIVE
 * --------
 *
 *     {
 *         mode = capability;
 *     }
 *
 *     {
 *         strategy = adaptive;
 *         priority = preference_priority;
 *         fallback = alternate;
 *     }
 *
 *     {
 *         capability = capability("quantum.measurement");
 *         fallback = capability("quantum.simulation");
 *     }
 *
 *     {
 *         quantum::fidelity = desired_fidelity;
 *         performance::latency = latency_budget;
 *     }
 *
 *     {
 *         vendor::future::metric = desired_value;
 *     }
 *
 *     {
 *         policy {
 *             primary = preferred;
 *             fallback = alternate;
 *         }
 *     }
 *
 * NEGATIVE
 * --------
 *
 *     {
 *         = value;
 *     }
 *
 *     {
 *         property value;
 *     }
 *
 *     {
 *         property = ;
 *     }
 *
 *     {
 *         property = value
 *     }
 *
 *     malformed qualified property paths;
 *     missing braces;
 *     unbalanced nested groups;
 *     missing assignment operators.
 *
 * BOUNDARY
 * --------
 *
 *     empty negotiation block;
 *     one clause;
 *     many clauses;
 *     deeply qualified properties;
 *     nested groups;
 *     large symbolic quantities;
 *     symbolic resource expressions.
 *
 * SCALABILITY
 * ----------
 *
 *     many clauses;
 *     many groups;
 *     deeply nested groups;
 *     large symbolic expressions;
 *     large numbers of alternatives represented semantically;
 *     large cross-domain resource policies.
 *
 * DETERMINISM
 * -----------
 *
 * The same valid source must produce the same parse structure for the same
 * language version.
 *
 * PORTABILITY
 * -----------
 *
 * Tests must demonstrate that the grammar accepts resource intent without
 * requiring a particular physical CPU/GPU/FPGA/QPU/device.
 *
 * HARD-CODING
 * -----------
 *
 * Static validation must reject accidental introduction of universal fixed
 * resource capacities.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 50. REQUIRED CROSS-FILE INTEGRATION
 * ============================================================================
 *
 * This file is intentionally independently complete.
 *
 * The following integration work belongs to the parent/composition layer and
 * must NOT require modification of this file:
 *
 * 1. grammar/resources/resources.g4
 *
 *    Import:
 *
 *        ResourceNegotiation
 *
 *    and delegate the concrete resource-level negotiation construct to:
 *
 *        resourceNegotiationSpecification
 *
 *    The parent owns the concrete introducer and statement terminator.
 *
 *
 * 2. grammar/antlr/ZamaniParser.g4
 *
 *    No direct import of this leaf is required.
 *
 *    The intended composition remains:
 *
 *        ZamaniParser
 *             |
 *             v
 *        Resources
 *             |
 *             v
 *        ResourceNegotiation
 *
 *    This preserves the existing composition hierarchy.
 *
 *
 * 3. grammar/lexer/keywords.g4
 *
 *    Do NOT modify this file merely to make this leaf work.
 *
 *    If a future normative language decision establishes `negotiate` as a
 *    reserved keyword, that lexical change must be made in the canonical
 *    keyword owner, followed by lexer/conformance/compatibility updates.
 *
 *    It must never be defined inside this grammar.
 *
 *
 * 4. grammar/resources/preferences.g4
 *
 *    Preferences remain independently owned.
 *
 *    Negotiation may consume their semantic results downstream but does not
 *    redefine their syntax.
 *
 *
 * 5. grammar/resources/requirements.g4
 *
 *    Requirements remain independently owned.
 *
 *    Negotiation may coordinate their satisfaction but does not redefine them.
 *
 *
 * 6. grammar/resources/constraints.g4
 *
 *    Constraints remain independently owned.
 *
 *
 * 7. grammar/resources/hints.g4
 *
 *    Hints remain independently owned.
 *
 *
 * 8. grammar/resources/resource-expressions.g4
 *
 *    Remains the sole owner of resourceExpression.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 51. INTEGRATION EXAMPLE
 * ============================================================================
 *
 * After the parent composition has been updated, the intended architecture is:
 *
 *     resourceNegotiationStatement
 *         :
 *         <parent-owned introducer>
 *         resourceNegotiationSpecification
 *         SEMICOLON
 *         ;
 *
 * The exact introducer is deliberately left to the parent grammar because
 * the current canonical lexer does not define NEGOTIATE.
 *
 * This file therefore does not need to be edited when that parent decision is
 * made.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 52. NO HARDWARE REALIZATION
 * ============================================================================
 *
 * This grammar does not define:
 *
 *     physical_device
 *     physical_qubit
 *     physical_core
 *     physical_gpu
 *     physical_fpga
 *     memory_bank
 *     provider_instance
 *     cloud_instance
 *
 * A downstream realization may exist, but it is not the portable negotiation
 * syntax.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 53. NO FIXED NEGOTIATION PROTOCOL
 * ============================================================================
 *
 * This grammar deliberately does not hard-code:
 *
 *     number of negotiation rounds;
 *     number of participants;
 *     number of offers;
 *     number of counteroffers;
 *     number of fallbacks;
 *     number of alternatives;
 *     number of providers.
 *
 * A future negotiation protocol may impose operational limits, but those are
 * protocol/runtime constraints rather than language-level cardinalities.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 54. NO EMBEDDED ALGORITHM
 * ============================================================================
 *
 * The following are semantic algorithms and do not belong in this grammar:
 *
 *     best-fit;
 *     first-fit;
 *     weighted optimization;
 *     Pareto selection;
 *     capability matching;
 *     provider scoring;
 *     resource scheduling;
 *     placement;
 *     routing;
 *     failover;
 *     retry;
 *     load balancing.
 *
 * The grammar only represents declarative data consumed by those systems.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 55. CROSS-DOMAIN INVARIANT
 * ============================================================================
 *
 * The same negotiation grammar can be used for:
 *
 *     classical;
 *     quantum;
 *     hybrid;
 *     HDL;
 *     hardware;
 *     distributed;
 *     AI;
 *     data;
 *     networking;
 *     security;
 *     accelerator;
 *     embedded;
 *     scientific;
 *     future domains.
 *
 * A new domain must not require a new universal negotiation grammar merely
 * because it introduces a new resource type.
 *
 * Open-world qualified properties provide the extension boundary.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 56. SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * Every negotiation construct must remain traceable to its source location.
 *
 * At minimum, the eventual AST/semantic representation must preserve:
 *
 *     block span;
 *     clause span;
 *     property span;
 *     value span;
 *     nested-group span.
 *
 * This is necessary for:
 *
 *     diagnostics;
 *     IDE tooling;
 *     semantic validation;
 *     compatibility reporting;
 *     feature conformance;
 *     reproducible compiler behavior.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 57. DIAGNOSTICS CONTRACT
 * ============================================================================
 *
 * Parser diagnostics cover malformed syntax:
 *
 *     missing property;
 *     missing assignment;
 *     missing value;
 *     missing semicolon;
 *     malformed qualified name;
 *     missing braces.
 *
 * Semantic diagnostics cover:
 *
 *     unknown standardized property;
 *     invalid value type;
 *     invalid capability;
 *     invalid fallback;
 *     invalid substitution;
 *     conflicting policy;
 *     incompatible negotiation terms.
 *
 * Resource diagnostics cover:
 *
 *     unavailable capability;
 *     insufficient resources;
 *     unsupported target;
 *     infeasible realization.
 *
 * These categories must remain distinct.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 58. SAFE-RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust source code.
 *
 * The consuming implementation must target:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and safe Rust only.
 *
 * This grammar introduces no requirement for:
 *
 *     unsafe
 *     FFI
 *     raw pointers
 *     target-specific runtime hooks
 *     hardware access
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 59. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE when:
 *
 *     [x] It has a single, explicit ownership boundary.
 *     [x] It does not redefine resourceExpression.
 *     [x] It does not redefine identifiers.
 *     [x] It does not invent a NEGOTIATE lexer token.
 *     [x] It supports reusable negotiation specifications.
 *     [x] It supports open-world properties.
 *     [x] It supports qualified properties.
 *     [x] It supports nested negotiation groups.
 *     [x] It supports arbitrary expression values.
 *     [x] It has no machine-capacity limits.
 *     [x] It has no fixed negotiation cardinalities.
 *     [x] It does not select physical resources.
 *     [x] It does not perform negotiation.
 *     [x] It preserves requirement/constraint/preference/hint separation.
 *     [x] It preserves POCO-REAF.
 *     [x] It preserves the canonical quantum::ir boundary.
 *     [x] It contains no Rust actions.
 *     [x] It requires no unsafe Rust.
 *     [x] It defines AST requirements.
 *     [x] It defines semantic requirements.
 *     [x] It defines IR ownership.
 *     [x] It defines compiler/runtime/HAL integration.
 *     [x] It defines diagnostics.
 *     [x] It defines compatibility.
 *     [x] It defines scalability and determinism requirements.
 *     [x] It defines the required parent integration without requiring a later
 *         edit to this file.
 *
 * ============================================================================
 */
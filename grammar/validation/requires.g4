/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/validation/requires.g4
 *
 * PURPOSE
 * -------
 * Production validation/conformance facade for Zamani requirement syntax.
 *
 * This file provides an isolated validation entry point for `requires`
 * constructs while preserving the repository's single-ownership grammar
 * architecture.
 *
 * ============================================================================
 * STATUS
 * ============================================================================
 *
 * Production validation component.
 *
 * This file is NOT a second owner of requirement syntax.
 *
 * Canonical requirement syntax is owned by:
 *
 *     grammar/core/requirements.g4
 *
 * Resource-specific requirement syntax is owned by:
 *
 *     grammar/resources/requirements.g4
 *
 * This file only exposes validation entry points over those canonical
 * grammars.
 *
 * ============================================================================
 * IMPLEMENTATION BASELINE
 * ============================================================================
 *
 * Rust:
 *
 *     Rust 1.97 / Rust 1.97.1
 *     Rust Edition 2021
 *
 * Safety:
 *
 *     Safe Rust only.
 *
 * This grammar itself contains:
 *
 *     - no Rust actions;
 *     - no Rust predicates;
 *     - no embedded code;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware access;
 *     - no runtime execution;
 *     - no resource allocation;
 *     - no backend selection;
 *     - no target discovery;
 *     - no unsafe code.
 *
 * ============================================================================
 * ARCHITECTURAL ROLE
 * ============================================================================
 *
 * The production Zamani pipeline is:
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     ZamaniParser
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     structural validation
 *       |
 *       +--> types
 *       +--> effects
 *       +--> capabilities
 *       +--> resources
 *       +--> contracts
 *       +--> policies
 *       +--> provenance
 *       |
 *       v
 *     semantic model
 *       |
 *       +--> classical representation
 *       +--> quantum::ir
 *       +--> HDL/hardware representation
 *       +--> distributed representation
 *       +--> accelerator representation
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     lowering
 *       |
 *       v
 *     routing / scheduling / resilience
 *       |
 *       v
 *     ZQN / HAL
 *       |
 *       v
 *     target realization
 *
 * This file participates only at the validation/parser boundary.
 *
 * ============================================================================
 * OWNERSHIP CONTRACT
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 *     requiresValidationUnit
 *     requiresValidationItem
 *     resourceRequiresValidationUnit
 *     resourceRequiresValidationItem
 *
 * These are validation-only entry points.
 *
 * THIS FILE DOES NOT OWN
 * ----------------------
 *
 *     REQUIRES
 *     requirementDeclaration
 *     requirementClause
 *     requirementExpression
 *     requirementDisjunction
 *     requirementConjunction
 *     requirementUnary
 *     requirementPrimary
 *     requirementReference
 *     requirementGroup
 *
 * Those belong to the canonical requirement grammar.
 *
 * This file also does not own:
 *
 *     identifiers
 *     qualified names
 *     capabilities
 *     capability versions
 *     resource expressions
 *     literals
 *     operators
 *     punctuation
 *     contracts
 *     policies
 *     effects
 *     quantum syntax
 *     HDL syntax
 *     hardware syntax
 *     AI syntax
 *     runtime behavior
 *
 * ============================================================================
 * SINGLE-OWNER RULE
 * ============================================================================
 *
 * There must be exactly one source grammar owner for each construct.
 *
 * The intended ownership is:
 *
 *     grammar/core/requirements.g4
 *         |
 *         +--> requirementDeclaration
 *         +--> requirementClause
 *         +--> requirementExpression
 *         +--> requirementReference
 *
 *     grammar/resources/requirements.g4
 *         |
 *         +--> resourceRequirement
 *         +--> resourceRequirementExpression
 *         +--> resourceCapabilityCall
 *
 *     grammar/validation/requires.g4
 *         |
 *         +--> validation entry points only
 *
 * Validation MUST NOT copy or partially reimplement those rules.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 * Canonical universal requirements:
 *
 *     grammar/core/requirements.g4
 *
 * Canonical resource requirements:
 *
 *     grammar/resources/requirements.g4
 *
 * Canonical lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Canonical lexical vocabulary:
 *
 *     grammar/lexer/
 *
 * Indirect dependencies include:
 *
 *     grammar/core/names.g4
 *     grammar/core/capabilities.g4
 *     grammar/resources/resource-expressions.g4
 *     grammar/expressions/
 *     grammar/lexer/keywords.g4
 *     grammar/lexer/operators.g4
 *     grammar/lexer/punctuation.g4
 *
 * The validation facade does not duplicate any of these dependencies.
 *
 * ============================================================================
 * EXPORT CONTRACT
 * ============================================================================
 *
 * EXPORTS
 * -------
 *
 *     requiresValidationUnit
 *     requiresValidationItem
 *     resourceRequiresValidationUnit
 *     resourceRequiresValidationItem
 *
 * These rules are intended for:
 *
 *     parser conformance tests
 *     structural validation tests
 *     negative syntax tests
 *     boundary tests
 *     scalability tests
 *     source-span tests
 *     compatibility tests
 *     grammar tooling
 *
 * They are NOT the production program entry point.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This file creates parser contexts only.
 *
 * It creates no Rust structures.
 *
 * The canonical AST remains domain-neutral.
 *
 * A validated universal requirement must retain, through the normal parser
 * and AST pipeline:
 *
 *     requirement kind
 *     requirement expression
 *     source span
 *     source ordering
 *     enclosing scope
 *     lexical/source representation where required
 *
 * A resource requirement must additionally preserve its resource-expression
 * structure for downstream semantic analysis.
 *
 * This file does not define those AST structures.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing and validation establish structural conformance only.
 *
 * Semantic analysis determines:
 *
 *     name resolution
 *     requirement identity
 *     capability resolution
 *     version compatibility
 *     resource meaning
 *     unit compatibility
 *     satisfiability
 *     contradiction
 *     policy compatibility
 *     effect compatibility
 *     target feasibility
 *     portability
 *     execution feasibility
 *
 * This file MUST NOT decide whether a requirement is satisfied.
 *
 * For example:
 *
 *     requires quantum::future_capability;
 *
 * is syntactically valid even when the current implementation does not know
 * the capability.
 *
 * The semantic layer may classify it as:
 *
 *     known
 *     unknown
 *     satisfied
 *     unsatisfied
 *     conditional
 *     unavailable
 *
 * according to the semantic model.
 *
 * ============================================================================
 * UNIVERSAL REQUIREMENTS
 * ============================================================================
 *
 * The canonical universal form is:
 *
 *     requires <requirement-expression>;
 *
 * Examples:
 *
 *     requires quantum::measurement;
 *
 *     requires quantum::measurement
 *              and quantum::dynamic_control;
 *
 *     requires quantum::measurement
 *              or classical::simulation;
 *
 *     requires not security::untrusted_execution;
 *
 *     requires (
 *         quantum::measurement
 *         and execution::deterministic
 *     );
 *
 * The complete syntax is inherited from:
 *
 *     requirementDeclaration
 *
 * This file deliberately does not reproduce it.
 *
 * ============================================================================
 * RESOURCE REQUIREMENTS
 * ============================================================================
 *
 * Resource requirements are structurally distinct from universal symbolic
 * requirements.
 *
 * Examples:
 *
 *     requires qubits >= logical_qubits;
 *
 *     requires memory >= required_memory;
 *
 *     requires nodes >= required_nodes;
 *
 *     requires capability("quantum.measurement");
 *
 *     requires capability("tensor.compute");
 *
 * Their syntax is owned by:
 *
 *     grammar/resources/requirements.g4
 *
 * This validation facade merely exposes an isolated validation entry point.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * A requirement describes program intent.
 *
 * It does not describe the physical machine that will realize that intent.
 *
 * Therefore this file MUST NOT encode limits for:
 *
 *     qubits
 *     CPUs
 *     cores
 *     threads
 *     GPUs
 *     FPGAs
 *     ASIC resources
 *     accelerators
 *     QPUs
 *     nodes
 *     memory
 *     storage
 *     registers
 *     tensor dimensions
 *     tensor rank
 *     network size
 *     device count
 *
 * A requirement such as:
 *
 *     requires qubits >= required_qubits;
 *
 * expresses a program-level condition.
 *
 * It does not establish a maximum number of qubits supported by Zamani.
 *
 * A numeric literal such as:
 *
 *     requires qubits >= 1024;
 *
 * is a source-program value, not a language capacity.
 *
 * ============================================================================
 * OPEN-WORLD CONTRACT
 * ============================================================================
 *
 * Requirement identities are open-ended.
 *
 * This validation file must accept whatever the canonical requirement grammar
 * accepts, including future namespaces and extensions.
 *
 * Examples:
 *
 *     quantum::measurement
 *     classical::parallel
 *     execution::deterministic
 *     distributed::collectives
 *     security::trusted_execution
 *     reasoning::inference
 *     learning::adaptation
 *     provenance::tracking
 *     tensor::compute
 *     future::computing::new_capability
 *
 * The validation grammar MUST NOT maintain an enumeration of requirement
 * names.
 *
 * Adding a new computational domain must not require editing this file.
 *
 * ============================================================================
 * BOOLEAN STRUCTURE
 * ============================================================================
 *
 * Universal requirement expressions inherit their precedence from the
 * canonical requirement grammar:
 *
 *     NOT
 *       >
 *     AND
 *       >
 *     OR
 *
 * Parentheses remain authoritative for explicit grouping.
 *
 * This validation facade does not redefine those precedence rules.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * A requirement may refer to a capability.
 *
 * Capability identity and capability-version syntax remain owned by the
 * canonical capability grammar.
 *
 * Validation does not:
 *
 *     discover capabilities;
 *     query hardware;
 *     inspect QPUs;
 *     inspect GPUs;
 *     inspect CPUs;
 *     inspect networks;
 *     grant capabilities;
 *     authorize capabilities.
 *
 * A parsed capability requirement is only an intent record.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Resource requirements are resolved downstream through:
 *
 *     requirement
 *       |
 *       v
 *     semantic resource model
 *       |
 *       v
 *     resource analysis
 *       |
 *       v
 *     capability negotiation
 *       |
 *       v
 *     execution planning
 *       |
 *       v
 *     target realization
 *
 * This file does not perform any of those operations.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing a requirement produces no runtime effect.
 *
 * A requirement expression may semantically refer to concepts involving:
 *
 *     IO
 *     network
 *     mutation
 *     randomness
 *     native execution
 *     foreign execution
 *     distributed execution
 *     measurement
 *     learning
 *     adaptation
 *     reflection
 *     code generation
 *     simulation
 *
 * Effect analysis remains downstream.
 *
 * This grammar never evaluates the condition.
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * Requirements may be used inside contracts.
 *
 * The ownership remains:
 *
 *     grammar/statements/contract.g4
 *     grammar/validation/contracts.g4
 *
 * This file MUST NOT create a second contract syntax.
 *
 * The semantic relationship is:
 *
 *     contract
 *        |
 *        +--> requirement
 *        |
 *        +--> condition
 *        |
 *        +--> guarantee
 *        |
 *        +--> evidence
 *        |
 *        +--> provenance
 *
 * Requirement validation remains independent of contract evaluation.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Requirements may participate in policies.
 *
 * Policy ownership remains with:
 *
 *     grammar/core/policies.g4
 *     grammar/expressions/policy.g4
 *     grammar/statements/policy.g4
 *
 * Parsing a requirement does not grant permission.
 *
 * Parsing a requirement does not create a policy decision.
 *
 * Policy analysis remains downstream.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Validation tooling must preserve sufficient source identity to associate a
 * validation result with:
 *
 *     source file
 *     source span
 *     requirement kind
 *     requirement expression
 *     enclosing context
 *     source order
 *
 * Semantic/provenance layers may later add:
 *
 *     derived_from
 *     generated_by
 *     transformed_by
 *     verified_by
 *     evidence
 *     decision
 *     version
 *
 * This grammar does not implement provenance storage.
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * Quantum requirements remain abstract.
 *
 * Examples:
 *
 *     requires quantum::measurement;
 *
 *     requires quantum::dynamic_control;
 *
 *     requires capability("quantum.mid_circuit_measurement");
 *
 * The validation layer MUST NOT:
 *
 *     enumerate gates;
 *     enumerate qubits;
 *     allocate physical qubits;
 *     define coupling maps;
 *     perform routing;
 *     perform scheduling;
 *     define calibration;
 *     implement QEC;
 *     select a QPU;
 *     create another quantum IR.
 *
 * When a requirement affects quantum compilation, its semantic information
 * may influence the canonical quantum pipeline:
 *
 *     semantic model
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     decomposition
 *          |
 *          v
 *     routing
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     resilience / QEC
 *          |
 *          v
 *     ZQN
 *          |
 *          v
 *     HAL
 *
 * The validation grammar remains outside that pipeline.
 *
 * ============================================================================
 * HDL / HARDWARE CONTRACT
 * ============================================================================
 *
 * Requirements may constrain hardware or HDL semantics through abstract
 * expressions.
 *
 * Examples:
 *
 *     requires capability("hdl.synthesis");
 *
 *     requires capability("hardware.reconfiguration");
 *
 *     requires execution::deterministic;
 *
 * The grammar does not define:
 *
 *     bus widths
 *     register widths
 *     FPGA dimensions
 *     ASIC cell counts
 *     physical pin counts
 *     memory-bank counts
 *     clock counts
 *     physical addresses
 *     vendor devices
 *     physical placement
 *
 * Those are downstream realization concerns.
 *
 * ============================================================================
 * CLASSICAL CONTRACT
 * ============================================================================
 *
 * Classical requirements use the same universal grammar.
 *
 * Examples:
 *
 *     requires classical::parallel;
 *
 *     requires execution::deterministic;
 *
 *     requires capability("tensor.compute");
 *
 * No classical-only requirement language is introduced here.
 *
 * ============================================================================
 * HYBRID CONTRACT
 * ============================================================================
 *
 * Hybrid computation may combine universal and resource requirements:
 *
 *     requires quantum::measurement
 *              and classical::parallel;
 *
 *     requires capability("quantum.measurement");
 *
 *     requires capability("tensor.compute");
 *
 * The validation facade does not distinguish the physical realization.
 *
 * ============================================================================
 * DISTRIBUTED CONTRACT
 * ============================================================================
 *
 * Requirements may refer to distributed capabilities:
 *
 *     requires distributed::communication;
 *
 *     requires distributed::collectives;
 *
 *     requires capability("distributed.consistency");
 *
 * Node count, topology, placement and scheduling remain semantic/resource
 * concerns.
 *
 * No finite node limit is introduced.
 *
 * ============================================================================
 * AI / REASONING CONTRACT
 * ============================================================================
 *
 * Requirements may express capabilities needed by reasoning, learning,
 * adaptation, uncertainty, provenance, agents and explainability.
 *
 * Examples:
 *
 *     requires reasoning::inference;
 *
 *     requires learning::adaptation;
 *
 *     requires provenance::tracking;
 *
 *     requires capability("model.inference");
 *
 * These remain open-world semantic identifiers.
 *
 * This validation file does not create AI-specific syntax.
 *
 * ============================================================================
 * INTEROPERABILITY CONTRACT
 * ============================================================================
 *
 * Requirements may constrain interoperability:
 *
 *     requires capability("ffi");
 *
 *     requires capability("abi.compatibility");
 *
 *     requires capability("data.json");
 *
 *     requires capability("data.xml");
 *
 * The validation layer does not define SQL, JSON, XML, FFI, ABI or external
 * protocol syntax.
 *
 * Those belong to their respective interoperability/dialect grammars.
 *
 * ============================================================================
 * METAPROGRAMMING CONTRACT
 * ============================================================================
 *
 * Requirements may describe capabilities needed by compile-time or reflective
 * facilities:
 *
 *     requires capability("compile.time");
 *
 *     requires capability("reflection");
 *
 *     requires capability("code.generation");
 *
 * This grammar does not execute or evaluate metaprogramming.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Given identical:
 *
 *     source
 *     language version
 *     lexer configuration
 *     parser configuration
 *
 * the validation parse structure must be equivalent.
 *
 * Parsing MUST NOT depend on:
 *
 *     time
 *     randomness
 *     filesystem state
 *     network state
 *     environment state
 *     hardware state
 *     target availability
 *     scheduler state
 *     runtime state
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Validation parsing must not:
 *
 *     execute requirements;
 *     allocate resources;
 *     discover hardware;
 *     invoke capabilities;
 *     bypass authorization;
 *     access credentials;
 *     invoke foreign code;
 *     access the network;
 *     access the filesystem.
 *
 * A requirement is declarative intent.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser-level diagnostics MUST be limited to structural syntax failures.
 *
 * Examples:
 *
 *     requires;
 *
 *     requires and quantum::measurement;
 *
 *     requires quantum::measurement and;
 *
 *     requires quantum::measurement or;
 *
 *     requires not;
 *
 *     requires ();
 *
 *     requires quantum::;
 *
 *     requires ::quantum;
 *
 *     requires quantum::::measurement;
 *
 *     requires (
 *         quantum::measurement;
 *
 *     requires quantum::measurement);
 *
 * Semantic diagnostics remain downstream:
 *
 *     unknown requirement;
 *     unknown capability;
 *     unavailable capability;
 *     incompatible version;
 *     contradictory requirements;
 *     unsatisfied requirement;
 *     resource infeasibility;
 *     policy conflict;
 *     effect conflict;
 *     target incompatibility.
 *
 * ============================================================================
 * SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * Validation consumers must preserve source spans for:
 *
 *     REQUIRES
 *     requirement expression
 *     logical operators
 *     grouping parentheses
 *     capability references
 *     resource expressions
 *     terminating semicolon
 *
 * Exact source-span representation remains an AST/frontend responsibility.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar introduces no artificial language-level limit on:
 *
 *     number of requirements
 *     number of requirement clauses
 *     number of logical operands
 *     number of alternatives
 *     nesting depth
 *     qualified-name depth
 *     resource expressions
 *     capability references
 *     computational domains
 *     source size
 *
 * It contains no:
 *
 *     MAX_REQUIREMENTS
 *     MAX_CAPABILITIES
 *     MAX_RESOURCES
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
 * Practical finite limits imposed by an implementation are resource limits,
 * not language semantics.
 *
 * ============================================================================
 * BOUNDARY CONTRACT
 * ============================================================================
 *
 * Validation must cover:
 *
 *     universal requirement
 *     resource requirement
 *     capability requirement
 *     versioned capability requirement
 *     conjunction
 *     disjunction
 *     negation
 *     nested grouping
 *     deeply qualified names
 *     future namespaces
 *     dialect namespaces
 *     vendor namespaces
 *     quantum requirements
 *     classical requirements
 *     HDL requirements
 *     hardware requirements
 *     hybrid requirements
 *     distributed requirements
 *     AI requirements
 *     security requirements
 *     execution requirements
 *
 * The boundary tests must verify that no domain-specific requirement becomes
 * a second grammar.
 *
 * ============================================================================
 * POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * Universal examples:
 *
 *     requires quantum::measurement;
 *
 *     requires quantum::measurement
 *              and quantum::dynamic_control;
 *
 *     requires quantum::measurement
 *              or classical::simulation;
 *
 *     requires not security::untrusted_execution;
 *
 *     requires (
 *         quantum::measurement
 *         and execution::deterministic
 *     );
 *
 * Open-world example:
 *
 *     requires future::computing::new_capability;
 *
 * Resource examples:
 *
 *     requires qubits >= logical_qubits;
 *
 *     requires memory >= required_memory;
 *
 *     requires nodes >= required_nodes;
 *
 *     requires capability("quantum.measurement");
 *
 *     requires capability("tensor.compute");
 *
 * Versioned universal example:
 *
 *     requires quantum::dynamic_control version >= 1.2;
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The following MUST fail structurally:
 *
 *     requires;
 *
 *     requires ();
 *
 *     requires and quantum::measurement;
 *
 *     requires quantum::measurement and;
 *
 *     requires quantum::measurement or;
 *
 *     requires not;
 *
 *     requires quantum::;
 *
 *     requires ::quantum;
 *
 *     requires quantum::::measurement;
 *
 *     requires quantum::measurement quantum::control;
 *
 *     requires (quantum::measurement;
 *
 *     requires quantum::measurement);
 *
 * Resource syntax errors must likewise be rejected by the canonical resource
 * grammar.
 *
 * Semantic infeasibility MUST NOT be converted into parser errors.
 *
 * ============================================================================
 * CROSS-DOMAIN TEST CONTRACT
 * ============================================================================
 *
 * At minimum, the validation suite must test:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     distributed
 *     networking
 *     AI
 *     security
 *     execution
 *     data
 *     interoperability
 *     metaprogramming
 *     dialects
 *
 * Requirement syntax remains common.
 *
 * ============================================================================
 * DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * Reparse the same source with the same:
 *
 *     lexer version
 *     parser version
 *     grammar version
 *     parser configuration
 *
 * and verify equivalent parse structure and source spans.
 *
 * No validation rule may use:
 *
 *     current time
 *     random state
 *     hardware discovery
 *     network state
 *     filesystem state
 *     target availability
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing canonical source syntax must remain unchanged.
 *
 * This file is a validation facade and therefore must not alter source
 * spelling or introduce aliases.
 *
 * Compatibility-sensitive changes belong in:
 *
 *     grammar/compatibility/
 *
 * and:
 *
 *     grammar/spec/compatibility.md
 *
 * Deprecated syntax must not be silently reintroduced through this file.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file produces no IR.
 *
 * Validation output flows conceptually as:
 *
 *     parse context
 *        |
 *        v
 *     domain-neutral AST
 *        |
 *        v
 *     structural validation
 *        |
 *        v
 *     semantic requirement model
 *        |
 *        +--> type analysis
 *        +--> effect analysis
 *        +--> capability analysis
 *        +--> resource analysis
 *        +--> policy analysis
 *        +--> provenance
 *        |
 *        v
 *     canonical semantic representation
 *
 * Quantum requirements may influence the path toward:
 *
 *     quantum::ir
 *
 * but this file never creates a quantum IR.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime verification is downstream.
 *
 * A runtime may evaluate whether requirements remain satisfied as execution
 * proceeds, but runtime state must never affect parsing.
 *
 * Runtime failure is not a parser error.
 *
 * ============================================================================
 * BUILD CONTRACT
 * ============================================================================
 *
 * This file is a parser grammar.
 *
 * It must consume the canonical Zamani lexer:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * It must be compiled with the same ANTLR grammar source path used for the
 * canonical parser.
 *
 * The validation grammar must not become a second parser composition root for
 * the complete language.
 *
 * ============================================================================
 * IMPORTANT REPOSITORY INTEGRATION ISSUE
 * ============================================================================
 *
 * The repository currently contains two files whose ANTLR grammar identity is
 * named `Requirements`:
 *
 *     grammar/core/requirements.g4
 *     grammar/resources/requirements.g4
 *
 * ANTLR imports are grammar-name based. Therefore these two grammar identities
 * cannot safely coexist as independently importable grammar identities in the
 * same parser build.
 *
 * The intended production ownership is:
 *
 *     grammar/core/requirements.g4
 *         -> Requirements
 *         -> universal requirement syntax
 *
 *     grammar/resources/requirements.g4
 *         -> ResourceRequirements
 *         -> resource requirement syntax
 *
 * The resource grammar's identity therefore needs to become:
 *
 *     ResourceRequirements
 *
 * while retaining its existing file path:
 *
 *     grammar/resources/requirements.g4
 *
 * This is an integration correction, not a source-language rename.
 *
 * After that correction, this validation grammar can unambiguously import:
 *
 *     Requirements
 *     ResourceRequirements
 *
 * No source-level keyword changes are required.
 *
 * ============================================================================
 * ANTLR GRAMMAR
 * ============================================================================
 */

parser grammar RequiresValidation;

options {
    tokenVocab = ZamaniLexer;
}

import Requirements, ResourceRequirements;


/*
 * ============================================================================
 * UNIVERSAL REQUIREMENT VALIDATION UNIT
 * ============================================================================
 *
 * Isolated complete validation input:
 *
 *     requires quantum::measurement;
 *
 * The EOF is intentionally owned by this validation entry point only.
 *
 * The production program parser remains owned by:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * ============================================================================
 */

requiresValidationUnit
    : requiresValidationItem EOF
    ;


/*
 * ============================================================================
 * UNIVERSAL REQUIREMENT VALIDATION ITEM
 * ============================================================================
 *
 * Delegates completely to the canonical universal requirement grammar.
 *
 * No requirement syntax is duplicated here.
 * ============================================================================
 */

requiresValidationItem
    : requirementDeclaration
    ;


/*
 * ============================================================================
 * RESOURCE REQUIREMENT VALIDATION UNIT
 * ============================================================================
 *
 * Isolated complete validation input for resource-specific requirements.
 *
 * Examples:
 *
 *     requires qubits >= logical_qubits;
 *
 *     requires memory >= required_memory;
 *
 *     requires capability("quantum.measurement");
 *
 * ============================================================================
 */

resourceRequiresValidationUnit
    : resourceRequiresValidationItem EOF
    ;


resourceRequiresValidationItem
    : resourceRequirement
    ;


/*
 * ============================================================================
 * INTEGRATED VALIDATION ENTRY
 * ============================================================================
 *
 * This rule is intentionally not used as the universal production program
 * entry point.
 *
 * It is useful for validation tooling that needs to validate either:
 *
 *     universal requirement syntax
 *
 * or:
 *
 *     resource requirement syntax
 *
 * in isolation.
 *
 * The alternatives delegate to the two canonical owners.
 *
 * ============================================================================
 */

requiresValidation
    : requirementDeclaration
    | resourceRequirement
    ;


/*
 * ============================================================================
 * OWNERSHIP INVARIANT
 * ============================================================================
 *
 * The following rules MUST NOT be added to this file:
 *
 *     requirementExpression
 *     requirementDisjunction
 *     requirementConjunction
 *     requirementUnary
 *     requirementPrimary
 *     requirementReference
 *     requirementGroup
 *     resourceRequirementExpression
 *     resourceCapabilityCall
 *     resourceExpression
 *
 * They already have canonical owners.
 *
 * If a future requirement construct is introduced, update its canonical owner
 * and then expose it here only through delegation.
 *
 * ============================================================================
 * EXTENSIBILITY INVARIANT
 * ============================================================================
 *
 * Adding:
 *
 *     a new computational domain
 *     a new capability
 *     a new resource
 *     a new accelerator
 *     a new quantum facility
 *     a new hardware facility
 *     a new distributed facility
 *     a new AI capability
 *     a new interoperability capability
 *
 * MUST NOT require editing this validation file merely because the semantic
 * vocabulary has grown.
 *
 * ============================================================================
 * SCALABILITY INVARIANT
 * ============================================================================
 *
 * There is no finite language-level maximum in this file.
 *
 * ANTLR repetition and recursive structure remain owned by the canonical
 * requirement grammars.
 *
 * This file introduces no fixed:
 *
 *     requirement count
 *     capability count
 *     resource count
 *     namespace count
 *     domain count
 *     nesting limit
 *     machine capacity
 *     target capacity
 *     hardware capacity
 *
 * "Infinity" in the language-design sense means that the grammar does not
 * impose an artificial finite ceiling. Actual execution remains bounded by
 * the resources of the implementation and target.
 *
 * ============================================================================
 * SAFETY INVARIANT
 * ============================================================================
 *
 * No Rust code is embedded in this grammar.
 *
 * The generated Rust frontend must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and must not require unsafe Rust.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [ ] It has exactly one grammar identity: RequiresValidation.
 *
 * [ ] It consumes ZamaniLexer.
 *
 * [ ] It imports the canonical universal Requirements grammar.
 *
 * [ ] It imports the canonical ResourceRequirements grammar.
 *
 * [ ] It does not duplicate requirement syntax.
 *
 * [ ] It does not duplicate resource syntax.
 *
 * [ ] It does not define lexer rules.
 *
 * [ ] It does not define semantic predicates.
 *
 * [ ] It does not contain embedded Rust.
 *
 * [ ] It does not access runtime state.
 *
 * [ ] It does not inspect hardware.
 *
 * [ ] It does not allocate resources.
 *
 * [ ] It does not select targets.
 *
 * [ ] It does not create IR.
 *
 * [ ] It preserves the domain-neutral AST boundary.
 *
 * [ ] It preserves the capability/resource separation.
 *
 * [ ] It preserves the contract/policy separation.
 *
 * [ ] It preserves provenance information through normal parser contexts.
 *
 * [ ] It has no machine-capacity constants.
 *
 * [ ] It has no artificial requirement cardinality limit.
 *
 * [ ] Universal requirement positive tests pass.
 *
 * [ ] Universal requirement negative tests pass.
 *
 * [ ] Resource requirement positive tests pass.
 *
 * [ ] Resource requirement negative tests pass.
 *
 * [ ] Boundary tests pass.
 *
 * [ ] Cross-domain tests pass.
 *
 * [ ] Determinism tests pass.
 *
 * [ ] Compatibility tests pass.
 *
 * [ ] ANTLR generation succeeds.
 *
 * [ ] Rust 1.97 generation/integration succeeds.
 *
 * [ ] Rust 1.97.1 generation/integration succeeds.
 *
 * [ ] No unsafe Rust is required.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * This file validates requirements.
 *
 * It does not define what a requirement means physically.
 *
 * Therefore:
 *
 *     requirement syntax
 *          !=
 *     resource allocation
 *
 *     requirement syntax
 *          !=
 *     capability discovery
 *
 *     requirement syntax
 *          !=
 *     target selection
 *
 *     requirement syntax
 *          !=
 *     quantum routing
 *
 *     requirement syntax
 *          !=
 *     scheduling
 *
 *     requirement syntax
 *          !=
 *     runtime authorization
 *
 *     requirement syntax
 *          !=
 *     physical realization
 *
 * The source program remains portable while semantic analysis and compilation
 * determine whether and how its requirements can be realized.
 *
 * ============================================================================
 */
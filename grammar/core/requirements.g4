/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/core/requirements.g4
 *
 * Purpose:
 *     Canonical parser grammar for universal source-level requirements.
 *
 * Grammar:
 *     ANTLR4 parser grammar
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     This grammar contains no embedded Rust actions, semantic predicates,
 *     runtime calls, filesystem access, network access, hardware access, or
 *     unsafe code.
 *
 * ============================================================================
 * ARCHITECTURAL ROLE
 * ============================================================================
 *
 * A requirement expresses a condition that a valid realization of a Zamani
 * program must satisfy.
 *
 * Examples:
 *
 *     requires quantum::dynamic_control;
 *     requires quantum::dynamic_control version >= 1.2;
 *     requires classical::parallel;
 *     requires security::trusted_execution;
 *     requires execution::deterministic;
 *
 * A requirement expresses SOURCE-LEVEL INTENT.
 *
 * A requirement does NOT directly select or allocate:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     physical qubit
 *     memory bank
 *     network node
 *     device
 *     backend
 *     topology
 *     route
 *     scheduler
 *     calibration
 *
 * Those decisions belong to downstream semantic, compilation, resource,
 * execution, and target-realization layers.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     requirementDeclaration
 *     requirementClause
 *     requirementExpression
 *     requirementDisjunction
 *     requirementConjunction
 *     requirementUnary
 *     requirementPrimary
 *     requirementReference
 *     requirementReferenceList
 *     requirementExpressionList
 *     requirementGroup
 *     requirementNegation
 *     requirementAll
 *     requirementAny
 *     optionalRequirementExpression
 *     optionalRequirementExpressionList
 *     requirementList
 *     optionalRequirementList
 *     requirementClauses
 *     optionalRequirementClauses
 *
 * THIS FILE DOES NOT OWN:
 *
 *     identifiers
 *     qualified names
 *     lexical tokens
 *     capability declarations
 *     capability identity
 *     capability version syntax
 *     resource expressions
 *     resource quantities
 *     resource allocation
 *     constraints
 *     preferences
 *     policies
 *     effects
 *     contracts
 *     target selection
 *     hardware discovery
 *     scheduling
 *     routing
 *     optimization
 *     quantum operations
 *     quantum::ir
 *     HDL IR
 *     classical IR
 *     runtime execution
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/core/names.g4
 *     grammar/core/capabilities.g4
 *
 * EXPORTS:
 *
 *     requirementDeclaration
 *     requirementClause
 *     requirementExpression
 *     requirementReference
 *     requirementReferenceList
 *     requirementExpressionList
 *     requirementGroup
 *     requirementNegation
 *     requirementAll
 *     requirementAny
 *     requirementList
 *     optionalRequirementExpression
 *     optionalRequirementExpressionList
 *     optionalRequirementList
 *     requirementClauses
 *     optionalRequirementClauses
 *
 * CONSUMED_BY:
 *
 *     grammar/core/compilation-unit.g4
 *     grammar/core/program.g4
 *     grammar/declarations/
 *     grammar/modules/
 *     grammar/functions/
 *     grammar/contracts/
 *     grammar/validation/
 *     grammar/policies/
 *     grammar/execution/
 *     grammar/compile/
 *     grammar/resources/
 *     grammar/hardware/
 *     grammar/quantum/
 *     grammar/hybrid/
 *     grammar/hdl/
 *     grammar/distributed/
 *     grammar/ai/
 *     grammar/networking/
 *     grammar/security/
 *     grammar/data/
 *
 * AST_OWNER:
 *
 *     frontend/domain-neutral AST subsystem
 *
 * SEMANTIC_OWNER:
 *
 *     semantic requirement model
 *
 * IR_OWNER:
 *
 *     canonical semantic representation
 *     downstream classical IR
 *     downstream quantum::ir
 *     downstream HDL/hardware representations
 *
 * TEST_OWNER:
 *
 *     grammar/tests/parser/
 *     grammar/tests/semantic/
 *     grammar/tests/resources/
 *     grammar/tests/quantum/
 *     grammar/tests/scalability/
 *     grammar/tests/portability/
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/resources.md
 *     grammar/spec/requirements.md
 *     grammar/specification/poco-reaf.md
 *
 * ============================================================================
 * FUNDAMENTAL DISTINCTION
 * ============================================================================
 *
 * Capability:
 *
 *     What an environment CAN provide.
 *
 * Requirement:
 *
 *     What a program NEEDS.
 *
 * Resource:
 *
 *     A computational quantity or facility that can participate in
 *     realization.
 *
 * Constraint:
 *
 *     A condition that a realization must satisfy.
 *
 * Preference:
 *
 *     A preferred property among otherwise valid realizations.
 *
 * Target:
 *
 *     A compilation or execution context.
 *
 * These concepts MUST remain separate.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Requirements are one of the mechanisms that allow a source program to
 * remain independent of the machine on which it eventually executes.
 *
 * A program may express:
 *
 *     requires quantum::measurement;
 *     requires execution::deterministic;
 *     requires distributed::communication;
 *     requires security::trusted_execution;
 *
 * without selecting a particular physical implementation.
 *
 * The same source can therefore participate in realization on:
 *
 *     embedded systems
 *     CPUs
 *     multicore systems
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     QPUs
 *     quantum simulators
 *     HPC systems
 *     clusters
 *     distributed systems
 *     cloud systems
 *     future execution systems
 *
 * subject to semantic feasibility and available resources.
 *
 * ============================================================================
 * OPEN-WORLD CONTRACT
 * ============================================================================
 *
 * Requirement identities are OPEN-WORLD.
 *
 * This grammar MUST NOT enumerate:
 *
 *     CPU requirements
 *     GPU requirements
 *     FPGA requirements
 *     ASIC requirements
 *     QPU requirements
 *     vendor requirements
 *     accelerator requirements
 *     future architecture requirements
 *     AI model requirements
 *     application-specific requirements
 *
 * Examples of valid open-world names:
 *
 *     quantum::dynamic_control
 *     quantum::measurement
 *     classical::parallel
 *     execution::deterministic
 *     security::trusted_execution
 *     distributed::collectives
 *     tensor::compute
 *     future::computing::new_capability
 *
 * New semantic domains MUST NOT require modification of this grammar merely
 * because a new requirement name is introduced.
 *
 * ============================================================================
 * NAME CONTRACT
 * ============================================================================
 *
 * Name syntax is owned by:
 *
 *     grammar/core/names.g4
 *
 * This grammar consumes:
 *
 *     qualifiedName
 *
 * and MUST NOT redefine:
 *
 *     identifier
 *     simpleName
 *     nameSegment
 *     qualifiedName
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Capability syntax is owned by:
 *
 *     grammar/core/capabilities.g4
 *
 * This grammar consumes:
 *
 *     capabilityReference
 *
 * A capability reference may contain the canonical capability version clause.
 *
 * This grammar MUST NOT duplicate:
 *
 *     capabilityName
 *     capabilityVersionClause
 *     capabilityVersionExpression
 *     capabilityVersionConstraint
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Universal requirements and resource requirements are deliberately separate.
 *
 * This file handles source-level universal requirements such as:
 *
 *     requires quantum::measurement;
 *
 *     requires execution::deterministic;
 *
 * Resource-specific syntax such as:
 *
 *     requires qubits >= logical_qubits;
 *
 *     requires memory >= required_memory;
 *
 *     requires nodes >= required_nodes;
 *
 *     requires capability("tensor.compute");
 *
 * belongs to:
 *
 *     grammar/resources/requirements.g4
 *
 * Resource expressions MUST NOT be duplicated here.
 *
 * This separation prevents the core grammar from becoming a second resource
 * expression language.
 *
 * ============================================================================
 * HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * This file contains NO physical hardware identity syntax.
 *
 * It MUST NOT encode:
 *
 *     device numbers
 *     CPU numbers
 *     GPU numbers
 *     FPGA numbers
 *     QPU numbers
 *     physical qubit identifiers
 *     memory-bank identifiers
 *     node identifiers
 *     vendor model identifiers
 *
 * Hardware realization belongs downstream.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar imposes NO language-level finite limit on:
 *
 *     number of requirements
 *     number of clauses
 *     number of alternatives
 *     number of conjunctions
 *     number of disjunctions
 *     nesting depth
 *     qualified-name depth
 *     source declarations
 *     computational domains
 *
 * Repetition therefore uses ANTLR repetition operators rather than fixed
 * cardinalities.
 *
 * No grammar-level constants are introduced for:
 *
 *     qubits
 *     CPUs
 *     GPUs
 *     FPGAs
 *     nodes
 *     memory
 *     threads
 *     tensor rank
 *     registers
 *     devices
 *     networks
 *
 * Practical parser/compiler limits are implementation-resource limits and are
 * not language semantics.
 *
 * ============================================================================
 * BOOLEAN SEMANTICS
 * ============================================================================
 *
 * Requirement expressions support:
 *
 *     not
 *     and
 *     or
 *
 * with syntactic precedence:
 *
 *     not
 *         >
 *     and
 *         >
 *     or
 *
 * Example:
 *
 *     requires quantum::measurement
 *              and quantum::dynamic_control;
 *
 *     requires quantum::measurement
 *              or classical::simulation;
 *
 *     requires not security::untrusted_execution;
 *
 * Parentheses explicitly override precedence.
 *
 * The parser records structure.
 *
 * It does NOT determine satisfiability.
 *
 * ============================================================================
 * SATISFIABILITY CONTRACT
 * ============================================================================
 *
 * The parser MUST NOT determine whether a requirement can actually be
 * satisfied.
 *
 * For example:
 *
 *     requires quantum::future_operation;
 *
 * is syntactically valid even if the current compiler knows nothing about
 * that capability.
 *
 * Semantic analysis may classify the result as:
 *
 *     satisfied
 *     unsatisfied
 *     unknown
 *     conditional
 *
 * according to the canonical semantic model.
 *
 * ============================================================================
 * UNKNOWN-OPEN-WORLD CONTRACT
 * ============================================================================
 *
 * Unknown requirement names remain syntactically representable.
 *
 * This is essential for:
 *
 *     future language evolution
 *     vendor extensions
 *     dialects
 *     research systems
 *     new accelerators
 *     new quantum capabilities
 *     new distributed capabilities
 *     future computational models
 *
 * Unknown does not mean syntactically invalid.
 *
 * ============================================================================
 * VERSION CONTRACT
 * ============================================================================
 *
 * Version constraints are delegated to the capability grammar.
 *
 * Example:
 *
 *     requires quantum::dynamic_control version >= 1.2;
 *
 * This grammar preserves the version structure but does not determine:
 *
 *     compatibility
 *     availability
 *     deprecation
 *     migration
 *     semantic version policy
 *
 * Those decisions belong to semantic compatibility analysis.
 *
 * ============================================================================
 * NEGATION CONTRACT
 * ============================================================================
 *
 * `not` is syntax.
 *
 * The semantic model determines what negation means in a particular
 * requirement context.
 *
 * This grammar does not interpret:
 *
 *     not capability
 *
 * as physical absence from a machine.
 *
 * It simply preserves the logical expression structure.
 *
 * ============================================================================
 * CONTRACT / POLICY / EFFECT BOUNDARIES
 * ============================================================================
 *
 * Requirements MAY be consumed by:
 *
 *     contracts
 *     policies
 *     effects
 *     security
 *     resource negotiation
 *     execution planning
 *
 * However, this grammar does not own those systems.
 *
 * For example:
 *
 *     requires security::trusted_execution;
 *
 * expresses a requirement.
 *
 * It does NOT grant authorization.
 *
 * Similarly:
 *
 *     requires execution::deterministic;
 *
 * does not itself implement deterministic execution.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Quantum requirements remain abstract.
 *
 * Examples:
 *
 *     requires quantum::measurement;
 *     requires quantum::dynamic_control;
 *     requires quantum::mid_circuit_measurement;
 *
 * This file does NOT reference:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     topology
 *     routing
 *     calibration
 *     QEC
 *     ZQN
 *
 * If a requirement affects quantum compilation, its semantic representation
 * may influence later lowering toward:
 *
 *     quantum::ir
 *
 * `quantum::ir` remains the canonical quantum IR boundary.
 *
 * ============================================================================
 * HDL BOUNDARY
 * ============================================================================
 *
 * HDL/hardware requirements may consume this grammar through:
 *
 *     requirementDeclaration
 *     requirementClause
 *
 * but hardware realization remains downstream.
 *
 * This grammar does not define:
 *
 *     signal widths
 *     physical cells
 *     routing
 *     timing implementation
 *     placement
 *     synthesis
 *     physical resource allocation
 *
 * ============================================================================
 * AI / REASONING BOUNDARY
 * ============================================================================
 *
 * Requirements can express semantic capabilities needed by reasoning,
 * learning, adaptation, inference, uncertainty, agents, or other systems.
 *
 * Examples:
 *
 *     requires reasoning::inference;
 *     requires learning::adaptation;
 *     requires provenance::tracking;
 *
 * The AI subsystem owns the meaning of those capabilities.
 *
 * This grammar does not create AI-specific requirement syntax.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Requirement AST nodes must preserve source spans and structural ordering.
 *
 * Downstream semantic analysis may attach:
 *
 *     evidence
 *     provenance
 *     decision records
 *     explanation
 *     verification state
 *
 * The parser does not create those semantic records.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no semantic predicates
 *     no embedded actions
 *     no runtime calls
 *     no filesystem access
 *     no network access
 *     no hardware discovery
 *     no randomness
 *
 * Parsing is therefore determined by the input token stream and parser
 * configuration.
 *
 * ============================================================================
 * SOURCE PRESERVATION CONTRACT
 * ============================================================================
 *
 * The frontend must be able to preserve:
 *
 *     requirement ordering
 *     expression grouping
 *     logical operators
 *     negation
 *     qualified-name segments
 *     capability version structure
 *     source spans
 *
 * Semantic canonicalization belongs downstream.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Conceptual AST:
 *
 *     RequirementDeclaration
 *         expression
 *         source_span
 *
 * Requirement expression nodes conceptually include:
 *
 *     RequirementReference
 *     RequirementNot
 *     RequirementAll
 *     RequirementAny
 *     RequirementGroup
 *
 * The concrete Rust AST types belong to the frontend AST subsystem.
 *
 * This grammar MUST NOT define Rust structures.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     name resolution
 *     capability resolution
 *     version compatibility
 *     requirement normalization
 *     duplicate detection
 *     contradiction detection
 *     satisfiability
 *     target compatibility
 *     resource feasibility
 *     policy interaction
 *     effect interaction
 *     diagnostics
 *
 * The parser does none of these.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * Requirements do not directly become operations in:
 *
 *     classical IR
 *     quantum::ir
 *     HDL IR
 *
 * Instead:
 *
 *     source
 *       |
 *       v
 *     requirement AST
 *       |
 *       v
 *     semantic requirement model
 *       |
 *       +--> capability analysis
 *       +--> resource analysis
 *       +--> policy analysis
 *       +--> target negotiation
 *       |
 *       v
 *     compilation/execution planning
 *
 * If a requirement affects quantum compilation, it may influence the semantic
 * pipeline leading to:
 *
 *     quantum::ir
 *
 * but it does not create a second quantum IR.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime verification is separate from parsing.
 *
 * A runtime may verify a semantic requirement against the actual execution
 * environment.
 *
 * Runtime failure MUST NOT be represented as a parser failure.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Requirements are not permissions.
 *
 * Parsing a requirement must never:
 *
 *     execute a capability
 *     access hardware
 *     access the filesystem
 *     access the network
 *     invoke a backend
 *     allocate resources
 *     bypass authorization
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Canonical syntax:
 *
 *     requires <requirement-expression>;
 *
 * Existing callers that need an unterminated clause may use:
 *
 *     requirementClause
 *
 * where the surrounding grammar owns the terminator.
 *
 * This avoids duplicating the keyword and expression syntax in every
 * subsystem.
 *
 * ============================================================================
 * ANTLR CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * Canonical lexer:
 *
 *     ZamaniLexer
 *
 * Canonical shared grammars:
 *
 *     Names
 *     Capabilities
 *
 * This file does not define lexer tokens.
 *
 * ============================================================================
 */

parser grammar Requirements;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Capabilities;


/* ============================================================================
 * PUBLIC TOP-LEVEL DECLARATION
 * ============================================================================
 *
 * Canonical source form:
 *
 *     requires quantum::measurement;
 *
 *     requires quantum::measurement
 *              and quantum::dynamic_control;
 *
 *     requires (
 *         quantum::measurement
 *         or classical::simulation
 *     );
 *
 * The semicolon belongs to the complete declaration.
 */

requirementDeclaration
    : REQUIRES requirementExpression SEMICOLON
    ;


/* ============================================================================
 * REUSABLE REQUIREMENT CLAUSE
 * ============================================================================
 *
 * The surrounding grammar owns the final terminator.
 *
 * Example conceptual use:
 *
 *     someDeclaration
 *         : ...
 *           requirementClause
 *           SEMICOLON
 *         ;
 */

requirementClause
    : REQUIRES requirementExpression
    ;


/* ============================================================================
 * REQUIREMENT EXPRESSION
 * ============================================================================
 *
 * Precedence:
 *
 *     not > and > or
 */

requirementExpression
    : requirementDisjunction
    ;


requirementDisjunction
    : requirementConjunction
      (OR requirementConjunction)*
    ;


requirementConjunction
    : requirementUnary
      (AND requirementUnary)*
    ;


requirementUnary
    : NOT requirementUnary
    | requirementPrimary
    ;


requirementPrimary
    : requirementReference
    | requirementGroup
    ;


requirementGroup
    : LEFT_PAREN requirementExpression RIGHT_PAREN
    ;


/* ============================================================================
 * REQUIREMENT REFERENCE
 * ============================================================================
 *
 * Capability references are canonicalized through Capabilities.
 *
 * A reference may therefore contain:
 *
 *     qualified capability name
 *     optional version clause
 *
 * Examples:
 *
 *     quantum::measurement
 *     quantum::dynamic_control
 *     quantum::dynamic_control version >= 1.2
 *     future::compute::new_capability
 *
 * Whether a reference denotes:
 *
 *     capability
 *     named requirement
 *     dialect requirement
 *     semantic facility
 *
 * is resolved downstream.
 */

requirementReference
    : capabilityReference
    ;


requirementReferenceList
    : requirementReference
      (COMMA requirementReference)*
    ;


optionalRequirementExpression
    : requirementExpression?
    ;


requirementExpressionList
    : requirementExpression
      (COMMA requirementExpression)*
    ;


optionalRequirementExpressionList
    : requirementExpressionList?
    ;


/* ============================================================================
 * EXPLICIT SEMANTIC WRAPPERS
 * ============================================================================
 *
 * These rules provide stable integration points without creating another
 * grammar representation.
 */

requirementNegation
    : NOT requirementUnary
    ;


requirementAll
    : requirementConjunction
    ;


requirementAny
    : requirementDisjunction
    ;


capabilityRequirement
    : capabilityReference
    ;


namedRequirement
    : qualifiedName
    ;


versionedRequirementReference
    : requirementReference
    ;


versionedRequirementPredicate
    : capabilityReference
    ;


requirementPredicate
    : qualifiedName
    ;


requirementGroupExpression
    : requirementGroup
    ;


/* ============================================================================
 * REQUIREMENT LISTS
 * ============================================================================
 *
 * These lists have no language-level maximum.
 *
 * The parser imposes no fixed number of requirements.
 */

requirementList
    : requirementExpression
      (COMMA requirementExpression)*
    ;


optionalRequirementList
    : requirementList?
    ;


requirementClauses
    : requirementClause+
    ;


optionalRequirementClauses
    : requirementClause*
    ;


/* ============================================================================
 * REQUIREMENT SET
 * ============================================================================
 *
 * A requirement set is a syntactic grouping.
 *
 * Set semantics such as duplicate elimination, normalization, ordering,
 * contradiction detection, and satisfiability belong downstream.
 */

requirementSet
    : requirementGroup
    ;


/* ============================================================================
 * INTEGRATION WRAPPERS
 * ============================================================================
 *
 * These wrappers intentionally contain no domain-specific syntax.
 */

classicalRequirement
    : requirementClause
    ;


quantumRequirement
    : requirementClause
    ;


hdlRequirement
    : requirementClause
    ;


hardwareRequirement
    : requirementClause
    ;


hybridRequirement
    : requirementClause
    ;


distributedRequirement
    : requirementClause
    ;


aiRequirement
    : requirementClause
    ;


securityRequirement
    : requirementClause
    ;


executionRequirement
    : requirementClause
    ;


/* ============================================================================
 * COMPLETION / OWNERSHIP RULE
 * ============================================================================
 *
 * Resource-specific requirements remain outside this grammar.
 *
 * For example:
 *
 *     requires qubits >= logical_qubits;
 *     requires memory >= required_memory;
 *     requires nodes >= required_nodes;
 *     requires capability("tensor.compute");
 *
 * are owned by:
 *
 *     grammar/resources/requirements.g4
 *
 * That grammar may expose a resourceRequirement rule.
 *
 * This file owns:
 *
 *     requirements expressed as universal symbolic/capability references.
 *
 * ============================================================================
 *
 * IMPORTANT:
 *
 * Do not add:
 *
 *     resourceExpression
 *     arithmeticExpression
 *     comparisonExpression
 *     capability(...)
 *
 * here merely to make examples parse.
 *
 * Doing so would create a competing resource grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
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
 * It contains no fixed enumeration of:
 *
 *     hardware types
 *     vendors
 *     devices
 *     quantum processors
 *     accelerator models
 *     resource quantities
 *     network sizes
 *     computational domains
 *
 * Requirement identities are open-ended symbolic names.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics should identify structural errors such as:
 *
 *     requires;
 *     requires and quantum::measurement;
 *     requires quantum::measurement or;
 *     requires ();
 *     requires quantum::measurement;
 *     requires quantum::measurement quantum::control;
 *
 * Semantic diagnostics belong downstream and include:
 *
 *     unknown requirement
 *     unavailable capability
 *     incompatible capability version
 *     contradictory requirements
 *     unsatisfied requirement
 *     inaccessible capability
 *     policy conflict
 *     resource infeasibility
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * The parser must accept:
 *
 *     requires quantum::measurement;
 *
 *     requires quantum::dynamic_control;
 *
 *     requires quantum::dynamic_control version >= 1.2;
 *
 *     requires classical::parallel;
 *
 *     requires execution::deterministic;
 *
 *     requires security::trusted_execution;
 *
 *     requires distributed::communication;
 *
 *     requires tensor::compute;
 *
 *     requires future::computing::new_capability;
 *
 *     requires quantum::measurement and quantum::dynamic_control;
 *
 *     requires quantum::measurement or classical::simulation;
 *
 *     requires not security::untrusted_execution;
 *
 *     requires (
 *         quantum::measurement
 *         and quantum::dynamic_control
 *     );
 *
 *     requires (
 *         quantum::measurement
 *         or (
 *             classical::simulation
 *             and execution::deterministic
 *         )
 *     );
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The parser must reject malformed forms such as:
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
 * ============================================================================
 */


/*
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Test:
 *
 *     single requirement
 *     many requirements
 *     deeply nested expressions
 *     deeply qualified names
 *     long symbolic names
 *     Unicode identifiers accepted by the lexer
 *     nested negation
 *     mixed and/or expressions
 *     nested parenthesized expressions
 *     capability version constraints
 *     future namespaces
 *     dialect namespaces
 *     vendor namespaces
 *
 * No test may interpret a test-size value as a language-level maximum.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * The conformance suite must progressively test:
 *
 *     many requirement clauses
 *     large requirement expressions
 *     deep qualification
 *     large source units
 *     large cross-domain programs
 *
 * Any practical parser memory/time ceiling must be reported as an
 * implementation/resource characteristic rather than converted into a
 * language restriction.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * CROSS-DOMAIN TEST CONTRACT
 * ============================================================================
 *
 * The same requirement grammar must work with:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     distributed
 *     networking
 *     AI
 *     data
 *     security
 *     execution
 *     interoperability
 *     metaprogramming
 *     dialects
 *
 * Domain grammars may provide wrappers but must not redefine requirement
 * syntax.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * Given identical:
 *
 *     source
 *     lexer configuration
 *     parser configuration
 *
 * the parser must produce equivalent parse-tree structure.
 *
 * Parsing must not depend on:
 *
 *     hardware
 *     available QPUs
 *     available GPUs
 *     network state
 *     filesystem state
 *     time
 *     randomness
 *     runtime state
 *     scheduler state
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This file contains no embedded Rust.
 *
 * Generated parser integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and the compiler implementation must use safe Rust only.
 *
 * No `unsafe` block, function, trait, or implementation is required by this
 * grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [ ] `Requirements` is the sole owner of universal requirement syntax.
 *
 * [ ] `ZamaniLexer` is the only lexer vocabulary consumed.
 *
 * [ ] `Names` owns identifier/qualified-name syntax.
 *
 * [ ] `Capabilities` owns capability identity/version syntax.
 *
 * [ ] Resource expressions are NOT duplicated here.
 *
 * [ ] `resources/requirements.g4` remains the resource-specific owner.
 *
 * [ ] `requirementDeclaration` is consumed directly by the compilation-unit
 *     parser.
 *
 * [ ] No `REQUIREMENT_DECLARATION` lexer token is required.
 *
 * [ ] No `K_*` legacy token names are referenced.
 *
 * [ ] Capability version clauses resolve through `Capabilities`.
 *
 * [ ] Boolean requirement precedence is deterministic.
 *
 * [ ] Parenthesized grouping is supported.
 *
 * [ ] Negation is supported.
 *
 * [ ] Open-world requirement names are supported.
 *
 * [ ] Unknown future names remain syntactically representable.
 *
 * [ ] No machine-specific resource limit is encoded.
 *
 * [ ] No hardware selection is encoded.
 *
 * [ ] No backend selection is encoded.
 *
 * [ ] No quantum physical topology is encoded.
 *
 * [ ] `quantum::ir` is not duplicated.
 *
 * [ ] Resource requirements can consume this grammar without redefining it.
 *
 * [ ] Classical requirements can consume this grammar.
 *
 * [ ] Quantum requirements can consume this grammar.
 *
 * [ ] HDL requirements can consume this grammar.
 *
 * [ ] AI requirements can consume this grammar.
 *
 * [ ] Security requirements can consume this grammar.
 *
 * [ ] Execution requirements can consume this grammar.
 *
 * [ ] Distributed requirements can consume this grammar.
 *
 * [ ] Positive tests exist.
 *
 * [ ] Negative tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Scalability tests exist.
 *
 * [ ] Cross-domain tests exist.
 *
 * [ ] Determinism tests exist.
 *
 * [ ] Rust 1.97/1.97.1 generation succeeds.
 *
 * [ ] No unsafe Rust is required.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * A requirement describes PROGRAM INTENT.
 *
 * It does not describe the physical machine.
 *
 * Therefore:
 *
 *     requirement
 *         !=
 *     resource allocation
 *
 *     requirement
 *         !=
 *     hardware selection
 *
 *     requirement
 *         !=
 *     quantum routing
 *
 *     requirement
 *         !=
 *     scheduling
 *
 *     requirement
 *         !=
 *     backend selection
 *
 *     requirement
 *         !=
 *     runtime authorization
 *
 *     requirement
 *         !=
 *     physical realization
 *
 * The semantic/compiler/runtime layers resolve those concerns after parsing.
 *
 * This separation is fundamental to:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * ============================================================================
 */
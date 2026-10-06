/*
 * ============================================================================
 * ZAMANI UNIVERSAL PROGRAMMING LANGUAGE
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/policies/constraints.g4
 *
 * GRAMMAR
 * -------
 * ANTLR4 parser grammar
 *
 * STATUS
 * ------
 * CANONICAL POLICY-CONSTRAINT ADAPTER
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 2021
 * Rust 1.97+
 * Safe Rust only
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file defines the POLICY-SPECIFIC STRUCTURAL BOUNDARY for constraints.
 *
 * The universal constraint language is owned exclusively by:
 *
 *     grammar/core/constraints.g4
 *
 * This file does NOT create a second constraint expression language.
 *
 * Instead, it defines how canonical constraints are:
 *
 *     declared;
 *     grouped;
 *     named;
 *     associated with policy intent;
 *     classified;
 *     annotated;
 *     composed;
 *     conditionally applied;
 *     inherited through policy composition;
 *     referenced by policy rules;
 *     associated with provenance;
 *     associated with evidence;
 *     associated with applicability metadata.
 *
 * The distinction is:
 *
 *     core/constraints.g4
 *             |
 *             | owns
 *             v
 *     constraintExpression
 *
 *             |
 *             v
 *
 *     policies/constraints.g4
 *             |
 *             | owns
 *             v
 *     policy constraint binding
 *
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * The policy constraint path is:
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
 *     policy constraint syntax
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic constraint model
 *       |
 *       +--> requirement analysis
 *       +--> capability analysis
 *       +--> resource analysis
 *       +--> contract analysis
 *       +--> policy analysis
 *       +--> provenance
 *       |
 *       v
 *     target-independent planning
 *       |
 *       +--> classical realization
 *       +--> quantum::ir
 *       +--> HDL/hardware realization
 *       +--> distributed realization
 *       +--> accelerator realization
 *       +--> future domain realization
 *
 * This file participates ONLY in source-level policy syntax.
 *
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns:
 *
 *     policyConstraintDeclaration
 *     policyConstraintBody
 *     policyConstraintMember
 *
 *     policyConstraintBinding
 *     policyConstraintExpression
 *
 *     policyConstraintName
 *     policyConstraintReference
 *     policyConstraintReferenceList
 *
 *     policyConstraintApplicability
 *     policyConstraintCondition
 *
 *     policyConstraintProperty
 *     policyConstraintPropertyKey
 *     policyConstraintPropertyValue
 *
 *     policyConstraintComposition
 *     policyConstraintExtension
 *     policyConstraintOverride
 *
 *     policyConstraintMetadata
 *     policyConstraintMetadataEntry
 *
 *     policyConstraintExpressionList
 *
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     lexer rules
 *     tokens
 *     identifiers
 *     qualified names
 *     literal syntax
 *     general expressions
 *     universal constraint expressions
 *     resource semantics
 *     capability semantics
 *     requirement semantics
 *     contract semantics
 *     policy evaluation
 *     authorization
 *     security enforcement
 *     hardware discovery
 *     target selection
 *     resource allocation
 *     scheduling
 *     routing
 *     optimization
 *     quantum operations
 *     physical qubits
 *     calibration
 *     QEC
 *     ZQN
 *     HAL
 *     HDL synthesis
 *     runtime enforcement
 *     backend implementation
 *
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * The universal constraint expression authority is:
 *
 *     grammar/core/constraints.g4
 *
 * In particular, this file MUST NOT redefine:
 *
 *     constraintExpression
 *     constraintPredicate
 *     constraintOperand
 *     constraintComparisonOperator
 *     constraintTypeBound
 *     constraintReference
 *
 * Those rules are consumed from the canonical Constraints grammar.
 *
 * This file is therefore an ADAPTER, not a competing constraint language.
 *
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/core/names.g4
 *     grammar/core/constraints.g4
 *     grammar/expressions/expressions.g4
 *
 * EXPORTS:
 *
 *     policyConstraintDeclaration
 *     policyConstraintBody
 *     policyConstraintMember
 *     policyConstraintBinding
 *     policyConstraintExpression
 *     policyConstraintName
 *     policyConstraintReference
 *     policyConstraintReferenceList
 *     policyConstraintApplicability
 *     policyConstraintCondition
 *     policyConstraintProperty
 *     policyConstraintPropertyKey
 *     policyConstraintPropertyValue
 *     policyConstraintComposition
 *     policyConstraintExtension
 *     policyConstraintOverride
 *     policyConstraintMetadata
 *     policyConstraintMetadataEntry
 *     policyConstraintExpressionList
 *
 * CONSUMED_BY:
 *
 *     grammar/policies/policy.g4
 *     grammar/policies/requirements.g4
 *     grammar/policies/preferences.g4
 *     grammar/policies/fallbacks.g4
 *     grammar/policies/execution.g4
 *     grammar/policies/resource.g4
 *     grammar/policies/security.g4
 *     grammar/policies/deployment.g4
 *     grammar/policies/simulation.g4
 *     grammar/policies/adaptation.g4
 *     grammar/policies/provenance.g4
 *
 * AST_OWNER:
 *
 *     domain-neutral frontend AST subsystem
 *
 * SEMANTIC_OWNER:
 *
 *     semantic constraint model
 *     policy semantic model
 *
 * TYPE_OWNER:
 *
 *     canonical type subsystem
 *
 * EFFECT_OWNER:
 *
 *     canonical effects subsystem
 *
 * CAPABILITY_OWNER:
 *
 *     canonical capability subsystem
 *
 * RESOURCE_OWNER:
 *
 *     canonical resource subsystem
 *
 * CONTRACT_OWNER:
 *
 *     grammar/validation/
 *
 * POLICY_OWNER:
 *
 *     grammar/policies/
 *
 * PROVENANCE_OWNER:
 *
 *     canonical provenance subsystem
 *
 * IR_OWNER:
 *
 *     canonical semantic representation
 *     downstream domain IRs
 *
 * QUANTUM_IR_OWNER:
 *
 *     quantum::ir
 *
 * TEST_OWNER:
 *
 *     grammar/tests/policies/
 *     grammar/tests/contracts/
 *     grammar/tests/resources/
 *     grammar/tests/semantic/
 *     grammar/tests/scalability/
 *     grammar/tests/portability/
 *     grammar/tests/boundary/
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/policies.md
 *     grammar/spec/resources.md
 *     grammar/spec/contracts.md
 *     grammar/specification/poco-reaf.md
 *
 *
 * ============================================================================
 * OPEN-WORLD CONTRACT
 * ============================================================================
 *
 * Policy constraints MUST remain open-world.
 *
 * This grammar MUST NOT enumerate:
 *
 *     CPU models
 *     GPU models
 *     FPGA families
 *     ASIC families
 *     QPU models
 *     accelerator models
 *     vendor architectures
 *     network sizes
 *     node counts
 *     device counts
 *     memory ceilings
 *     register widths
 *     tensor-rank ceilings
 *     qubit ceilings
 *     thread ceilings
 *
 * New semantic constraint names remain representable through the canonical
 * qualified-name system.
 *
 * Examples:
 *
 *     resource::memory >= required_memory
 *
 *     resource::compute >= required_compute
 *
 *     capability::quantum::measurement == true
 *
 *     capability::tensor::compute == true
 *
 *     hardware::accelerated_compute == true
 *
 *     quantum::dynamic_control == true
 *
 *     execution::deterministic == true
 *
 *     provenance::tracking == true
 *
 *     future::domain::property == expected
 *
 * No grammar change is required merely because a new semantic property,
 * resource, capability, architecture, vendor technology, or future computing
 * substrate is introduced.
 *
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Policy constraints describe PORTABLE INTENT.
 *
 * They do not select a physical realization.
 *
 * A policy constraint may therefore remain unchanged while realization moves
 * between:
 *
 *     atom-scale abstractions
 *     embedded systems
 *     CPUs
 *     multicore systems
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     QPUs
 *     simulators
 *     HPC systems
 *     clusters
 *     distributed systems
 *     cloud systems
 *     heterogeneous systems
 *     future computational substrates
 *
 * Example:
 *
 *     constraint execution::deterministic {
 *         condition execution::deterministic == true;
 *     }
 *
 * does not select a CPU, GPU, QPU, FPGA, simulator, or cloud provider.
 *
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This file MUST NOT define universal limits such as:
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
 * It MUST NOT define universal physical identifiers such as:
 *
 *     cpu0
 *     gpu0
 *     qpu0
 *     fpga0
 *     node0
 *     device0
 *
 * Numeric values in a constraint are program data.
 *
 * For example:
 *
 *     resource::memory >= 1024
 *
 * does not establish a language-wide memory limit.
 *
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * There is no grammar-level finite maximum for:
 *
 *     policy constraints
 *     policy members
 *     constraint expressions
 *     constraint references
 *     metadata entries
 *     properties
 *     composition relationships
 *     qualified-name depth
 *     policy nesting
 *     expression list length
 *     source size
 *     domains
 *     resources
 *     capabilities
 *     devices
 *     nodes
 *     qubits
 *     processors
 *     accelerators
 *
 * Repetition uses ANTLR repetition operators.
 *
 * Practical limits are determined by available implementation resources and
 * downstream execution environments, never by this grammar.
 *
 *
 * ============================================================================
 * NAME INTEGRATION
 * ============================================================================
 *
 * Name syntax belongs to:
 *
 *     grammar/core/names.g4
 *
 * This file consumes:
 *
 *     identifier
 *     qualifiedName
 *
 * It MUST NOT redefine them.
 *
 *
 * ============================================================================
 * CONSTRAINT EXPRESSION INTEGRATION
 * ============================================================================
 *
 * The canonical universal constraint expression is:
 *
 *     constraintExpression
 *
 * from:
 *
 *     grammar/core/constraints.g4
 *
 * This file aliases that canonical expression through:
 *
 *     policyConstraintExpression
 *
 * This adapter rule exists to make ownership explicit to policy consumers.
 *
 * It does not create a second expression language.
 *
 *
 * ============================================================================
 * EXPRESSION INTEGRATION
 * ============================================================================
 *
 * General expressions remain owned by:
 *
 *     grammar/expressions/expressions.g4
 *
 * They may be used for:
 *
 *     applicability conditions
 *     property values
 *     metadata values
 *     policy bindings
 *
 * The policy-constraint grammar never recreates general expression precedence.
 *
 *
 * ============================================================================
 * POLICY CONSTRAINT DECLARATION
 * ============================================================================
 *
 * Canonical named form:
 *
 *     constraint execution::deterministic {
 *         condition execution::deterministic == true;
 *     }
 *
 * Minimal form:
 *
 *     constraint execution::deterministic;
 *
 * Expression form:
 *
 *     constraint execution::deterministic
 *         = execution::deterministic == true;
 *
 * The semantic layer determines whether a named declaration is complete,
 * whether the constraint is satisfiable, and whether its metadata is valid.
 *
 *
 * ============================================================================
 * 1. POLICY CONSTRAINT DECLARATION
 * ============================================================================
 */

parser grammar PolicyConstraints;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names,
    Constraints,
    Expressions
;


/*
 * ============================================================================
 * 2. NAMED POLICY CONSTRAINT
 * ============================================================================
 *
 * A named policy constraint gives a reusable identity to a canonical
 * constraint expression.
 *
 * Supported forms:
 *
 *     constraint execution::deterministic;
 *
 *     constraint execution::deterministic {
 *         ...
 *     }
 *
 *     constraint execution::deterministic =
 *         execution::deterministic == true;
 *
 * The identifier is semantic identity.
 *
 * The grammar does not assign physical meaning to the name.
 */

policyConstraintDeclaration
    : CONSTRAINT
      policyConstraintName
      policyConstraintDeclarationTail?
    ;


/*
 * ============================================================================
 * 3. DECLARATION TAIL
 * ============================================================================
 *
 * A declaration may be:
 *
 *     empty;
 *     expression-bound;
 *     block-bound.
 */

policyConstraintDeclarationTail
    : SEMICOLON
    | ASSIGN
      policyConstraintExpression
      SEMICOLON
    | policyConstraintBody
    ;


/*
 * ============================================================================
 * 4. CONSTRAINT BODY
 * ============================================================================
 *
 * The body is intentionally open-ended.
 *
 * There is no fixed number of:
 *
 *     conditions
 *     properties
 *     references
 *     metadata entries
 *     composition clauses
 *
 * This is essential for large policies and future extensions.
 */

policyConstraintBody
    : LBRACE
      policyConstraintMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 5. CONSTRAINT MEMBER
 * ============================================================================
 *
 * The body deliberately separates semantic categories.
 *
 * Universal constraint expressions remain owned by core/constraints.g4.
 */

policyConstraintMember
    : policyConstraintBinding
    | policyConstraintApplicability
    | policyConstraintComposition
    | policyConstraintProperty
    | policyConstraintMetadata
    ;


/*
 * ============================================================================
 * 6. CONSTRAINT BINDING
 * ============================================================================
 *
 * Canonical forms:
 *
 *     condition resource::memory >= required_memory;
 *
 *     condition capability::quantum::measurement == true;
 *
 *     expression resource::compute >= required_compute;
 *
 * "condition" and "expression" are policy-level bindings.
 *
 * The actual constraint language remains canonical.
 */

policyConstraintBinding
    : CONDITION
      policyConstraintExpression
      SEMICOLON
    | EXPRESSION
      policyConstraintExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 7. POLICY CONSTRAINT EXPRESSION
 * ============================================================================
 *
 * Single bridge to the universal constraint grammar.
 *
 * IMPORTANT:
 *
 * Do not replace this with a copied expression grammar.
 */

policyConstraintExpression
    : constraintExpression
    ;


/*
 * ============================================================================
 * 8. CONSTRAINT NAME
 * ============================================================================
 *
 * A constraint name is an open-world qualified name.
 */

policyConstraintName
    : qualifiedName
    ;


/*
 * ============================================================================
 * 9. CONSTRAINT REFERENCE
 * ============================================================================
 *
 * References point to previously defined or externally supplied semantic
 * constraints.
 *
 * Resolution is downstream.
 */

policyConstraintReference
    : qualifiedName
    ;


policyConstraintReferenceList
    : policyConstraintReference
      (COMMA policyConstraintReference)*
      COMMA?
    ;


/*
 * ============================================================================
 * 10. APPLICABILITY
 * ============================================================================
 *
 * A policy constraint may have a condition controlling when the constraint
 * applies.
 *
 * Example:
 *
 *     applies_if execution::mode == "production";
 *
 * The condition is a normal Zamani expression.
 *
 * It is NOT a hardware selector.
 */

policyConstraintApplicability
    : APPLIES_IF
      policyConstraintCondition
      SEMICOLON
    ;


policyConstraintCondition
    : expression
    ;


/*
 * ============================================================================
 * 11. POLICY CONSTRAINT COMPOSITION
 * ============================================================================
 *
 * Constraint composition is by named semantic relationship.
 *
 * This grammar does not hard-code a finite set of constraint families.
 *
 * EXTENDS and OVERRIDE are already canonical lexical concepts in Zamani.
 */

policyConstraintComposition
    : policyConstraintExtension
    | policyConstraintOverride
    ;


policyConstraintExtension
    : EXTENDS
      policyConstraintReference
      SEMICOLON
    ;


policyConstraintOverride
    : OVERRIDE
      policyConstraintReference
      SEMICOLON
    ;


/*
 * ============================================================================
 * 12. OPEN-WORLD PROPERTY
 * ============================================================================
 *
 * New policy-constraint metadata must not require a new keyword.
 *
 * Example:
 *
 *     priority = 10;
 *     severity = "required";
 *     explanation = "must remain deterministic";
 *     provenance = provenance::compile;
 *     evidence = evidence::verified;
 *
 * Property names are identifiers.
 */

policyConstraintProperty
    : policyConstraintPropertyKey
      ASSIGN
      policyConstraintPropertyValue
      SEMICOLON
    ;


policyConstraintPropertyKey
    : identifier
    ;


policyConstraintPropertyValue
    : expression
    ;


/*
 * ============================================================================
 * 13. METADATA
 * ============================================================================
 *
 * Metadata is intentionally generic.
 *
 * It can carry information used by:
 *
 *     semantic analysis
 *     validation
 *     policy evaluation
 *     provenance
 *     diagnostics
 *     tooling
 *     compilation
 *     reproducibility
 *
 * It must not become an alternate semantic language.
 */

policyConstraintMetadata
    : METADATA
      LBRACE
      policyConstraintMetadataEntry*
      RBRACE
    ;


policyConstraintMetadataEntry
    : identifier
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 14. EXPRESSION LIST
 * ============================================================================
 *
 * Reusable open-ended list for policy integrations.
 */

policyConstraintExpressionList
    : policyConstraintExpression
      (COMMA policyConstraintExpression)*
      COMMA?
    ;


/*
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * This grammar creates STRUCTURE only.
 *
 * Semantic analysis MUST determine:
 *
 *     whether the constraint name exists;
 *     whether the expression is well typed;
 *     whether referenced resources exist;
 *     whether referenced capabilities exist;
 *     whether the constraint is satisfiable;
 *     whether multiple constraints conflict;
 *     whether inheritance is legal;
 *     whether override is authorized;
 *     whether applicability is valid;
 *     whether metadata is recognized;
 *     whether the constraint is required or advisory;
 *     whether the target environment can satisfy it.
 *
 * None of those decisions occur during parsing.
 *
 *
 * ============================================================================
 * REQUIREMENT BOUNDARY
 * ============================================================================
 *
 * Requirement and constraint are different semantic concepts.
 *
 * Requirements express what must be available.
 *
 * Constraints express conditions that must hold.
 *
 * Therefore:
 *
 *     grammar/core/requirements.g4
 *
 * remains the requirement authority.
 *
 * This file may be consumed by requirement policies but must not redefine
 * requirement syntax.
 *
 *
 * ============================================================================
 * RESOURCE BOUNDARY
 * ============================================================================
 *
 * Resource semantics remain owned by:
 *
 *     grammar/resources/
 *
 * A policy constraint may refer to a resource property:
 *
 *     condition resource::memory >= required_memory;
 *
 * or:
 *
 *     condition resource::compute >= required_compute;
 *
 * but this grammar does not determine:
 *
 *     how memory is discovered;
 *     how memory is allocated;
 *     how compute is measured;
 *     which physical device supplies the resource;
 *     how resources are scheduled.
 *
 *
 * ============================================================================
 * CAPABILITY BOUNDARY
 * ============================================================================
 *
 * Capability semantics remain owned by:
 *
 *     grammar/core/capabilities.g4
 *     grammar/resources/
 *
 * Example:
 *
 *     condition capability::quantum::measurement == true;
 *
 * is structurally valid.
 *
 * Whether the capability exists is a semantic/environment question.
 *
 *
 * ============================================================================
 * EFFECT BOUNDARY
 * ============================================================================
 *
 * Constraints do not themselves execute effects.
 *
 * A constraint may influence legality of operations involving:
 *
 *     IO
 *     network
 *     native calls
 *     foreign calls
 *     randomness
 *     measurement
 *     learning
 *     adaptation
 *     reflection
 *     simulation
 *     distributed execution
 *
 * Effect ownership remains with:
 *
 *     grammar/effects/
 *
 *
 * ============================================================================
 * CONTRACT BOUNDARY
 * ============================================================================
 *
 * Policy constraints may participate in contracts, but this file does not
 * redefine:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * Contract syntax remains owned by:
 *
 *     grammar/validation/
 *
 * A semantic contract may consume policy constraints as assumptions,
 * requirements, or guarantees according to the semantic model.
 *
 *
 * ============================================================================
 * SECURITY BOUNDARY
 * ============================================================================
 *
 * A policy constraint may participate in security policy evaluation.
 *
 * This grammar does not grant:
 *
 *     authorization
 *     permissions
 *     credentials
 *     trust
 *     privilege
 *     sandbox escape
 *     native execution
 *
 * Security semantics remain owned by:
 *
 *     grammar/security/
 *
 *
 * ============================================================================
 * PROVENANCE BOUNDARY
 * ============================================================================
 *
 * Policy constraints must preserve source provenance through the AST.
 *
 * Downstream provenance may record:
 *
 *     source location
 *     constraint identity
 *     normalized form
 *     originating policy
 *     inherited policy
 *     override relationship
 *     evidence
 *     evaluation result
 *     decision
 *     compilation consequence
 *
 * Provenance generation is not performed by this grammar.
 *
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Quantum-related policy constraints remain target-neutral.
 *
 * Examples:
 *
 *     constraint quantum::dynamic_control {
 *         condition quantum::dynamic_control == true;
 *     }
 *
 *     constraint quantum::error_correction {
 *         condition capability::quantum::error_correction == true;
 *     }
 *
 * This grammar MUST NOT encode:
 *
 *     physical qubit IDs
 *     coupling maps
 *     routing
 *     calibration
 *     pulse schedules
 *     vendor QPU topology
 *     QEC implementation
 *     physical error rates
 *
 * If a constraint affects quantum compilation, the semantic path remains:
 *
 *     source
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     semantic model
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     decomposition
 *       |
 *       v
 *     routing
 *       |
 *       v
 *     scheduling
 *       |
 *       v
 *     resilience / QEC
 *       |
 *       v
 *     ZQN
 *       |
 *       v
 *     HAL
 *       |
 *       v
 *     target realization
 *
 * This grammar creates no quantum::ir.
 *
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Hardware constraints remain intent-level.
 *
 * Valid semantic references include:
 *
 *     hardware::clocking
 *     hardware::pipeline_support
 *     hardware::accelerated_compute
 *     hardware::memory_interface
 *
 * The grammar does not define:
 *
 *     fixed register widths
 *     fixed bus widths
 *     fixed device counts
 *     fixed FPGA capacity
 *     fixed ASIC capacity
 *     physical placement
 *     synthesis implementation
 *
 *
 * ============================================================================
 * AI / REASONING / KNOWLEDGE BOUNDARY
 * ============================================================================
 *
 * Constraints can refer to open-world semantic properties associated with:
 *
 *     reasoning
 *     inference
 *     learning
 *     adaptation
 *     knowledge
 *     uncertainty
 *     evidence
 *     provenance
 *     agents
 *     explanation
 *
 * Examples:
 *
 *     constraint reasoning::inference {
 *         condition reasoning::inference == true;
 *     }
 *
 *     constraint learning::adaptation {
 *         condition capability::learning::adaptation == true;
 *     }
 *
 * No application-specific AI keyword catalogue is required.
 *
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing this grammar must be deterministic.
 *
 * Parsing MUST NOT depend upon:
 *
 *     hardware;
 *     runtime state;
 *     target availability;
 *     filesystem state;
 *     network state;
 *     randomness;
 *     wall-clock time;
 *     environment variables.
 *
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar does not define Rust AST structures.
 *
 * The frontend AST must preserve at minimum:
 *
 *     declaration identity
 *     source ordering
 *     source spans
 *     expression structure
 *     applicability
 *     inheritance
 *     override relationships
 *     property structure
 *     metadata
 *
 * The AST MUST remain domain-neutral.
 *
 * It must not contain:
 *
 *     physical QPU topology
 *     CPU-specific instructions
 *     GPU-specific instructions
 *     FPGA routing
 *     vendor backend objects
 *     QEC schedules
 *     calibration records
 *
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * Policy constraints do not directly lower into machine instructions.
 *
 * They contribute semantic information to:
 *
 *     canonical semantic representation
 *     compilation planning
 *     resource negotiation
 *     optimization legality
 *     execution planning
 *     validation
 *     deployment
 *
 * They may indirectly influence:
 *
 *     classical IR
 *     quantum::ir
 *     HDL/hardware representation
 *     distributed execution planning
 *     target realization
 *
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics must identify structural failures such as:
 *
 *     missing constraint name
 *     missing expression
 *     missing semicolon
 *     missing closing brace
 *     malformed assignment
 *     malformed property
 *     malformed applicability condition
 *     malformed inheritance
 *     malformed override
 *     malformed metadata
 *
 * Semantic diagnostics belong downstream:
 *
 *     unknown constraint
 *     conflicting constraint
 *     unsatisfiable constraint
 *     invalid constraint reference
 *     invalid type
 *     unavailable capability
 *     unavailable resource
 *     invalid policy composition
 *     unauthorized override
 *     invalid applicability condition
 *
 *
 * ============================================================================
 * POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * The conformance suite MUST accept at minimum:
 *
 *     constraint execution::deterministic;
 *
 *     constraint execution::deterministic =
 *         execution::deterministic == true;
 *
 *     constraint resource::memory {
 *         condition resource::memory >= required_memory;
 *     }
 *
 *     constraint capability::quantum::measurement {
 *         condition capability::quantum::measurement == true;
 *     }
 *
 *     constraint future::domain::property {
 *         condition future::domain::property == expected;
 *     }
 *
 *     constraint execution::deterministic {
 *         priority = 10;
 *         severity = "required";
 *     }
 *
 *     constraint execution::deterministic {
 *         condition execution::mode == "production";
 *         priority = 10;
 *     }
 *
 *     constraint quantum::execution {
 *         extends execution::deterministic;
 *     }
 *
 *     constraint quantum::execution {
 *         override execution::deterministic;
 *     }
 *
 *     constraint large::policy::constraint {
 *         metadata {
 *             provenance = provenance::compile;
 *             evidence = evidence::verified;
 *         }
 *     }
 *
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The conformance suite MUST reject structurally malformed input including:
 *
 *     constraint;
 *
 *     constraint execution::deterministic =
 *     ;
 *
 *     constraint execution::deterministic {
 *         condition;
 *     }
 *
 *     constraint execution::deterministic {
 *         = true;
 *     }
 *
 *     constraint execution::deterministic {
 *         priority;
 *     }
 *
 *     constraint execution::deterministic {
 *         extends;
 *     }
 *
 *     constraint execution::deterministic {
 *         override;
 *     }
 *
 *     constraint execution::deterministic {
 *         metadata {
 *             provenance;
 *         }
 *     }
 *
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Test combinations including:
 *
 *     classical + constraint
 *     quantum + constraint
 *     hybrid + constraint
 *     HDL + constraint
 *     hardware + constraint
 *     AI + constraint
 *     distributed + constraint
 *     networking + constraint
 *     simulation + constraint
 *     security + constraint
 *     interoperability + constraint
 *     compile-time + constraint
 *     runtime validation + constraint
 *
 * Also test:
 *
 *     requirement + constraint
 *     capability + constraint
 *     resource + constraint
 *     contract + constraint
 *     policy + constraint
 *     provenance + constraint
 *
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests must demonstrate that grammar structure does not impose artificial
 * limits on:
 *
 *     number of constraints
 *     nesting depth
 *     metadata entries
 *     property count
 *     qualified-name depth
 *     policy size
 *     source size
 *     resource dimensions
 *     capability dimensions
 *
 * Tests must use symbolic resource quantities rather than encoding universal
 * machine capacities.
 *
 * Example:
 *
 *     constraint resource::memory {
 *         condition resource::memory >= required_memory;
 *     }
 *
 * is preferred over introducing any language-wide memory ceiling.
 *
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * The existing simple policy constraint form:
 *
 *     constraint <constraint-expression>;
 *
 * MUST remain valid through policy.g4 integration.
 *
 * No existing canonical constraint-expression syntax should be invalidated by
 * this adapter.
 *
 * Historical syntax belongs to:
 *
 *     grammar/compatibility/
 *
 * and must not be duplicated here.
 *
 *
 * ============================================================================
 * INTEGRATION WITH grammar/policies/policy.g4
 * ============================================================================
 *
 * The policy grammar currently owns a rule conceptually equivalent to:
 *
 *     policyConstraint
 *         : CONSTRAINT constraintExpression SEMICOLON
 *         ;
 *
 * That rule should become the integration adapter:
 *
 *     policyConstraint
 *         : policyConstraintDeclaration
 *         ;
 *
 * and Policy.g4 must import this grammar:
 *
 *     import
 *         Names,
 *         Requirements,
 *         Constraints,
 *         Capabilities,
 *         Expressions,
 *         PolicyConstraints
 *     ;
 *
 * This change is required so this file becomes the single policy-specific
 * constraint authority.
 *
 * IMPORTANT:
 *
 * The universal:
 *
 *     constraintExpression
 *
 * continues to come from:
 *
 *     grammar/core/constraints.g4
 *
 * Therefore there is still only ONE constraint expression language.
 *
 *
 * ============================================================================
 * INTEGRATION WITH ZamaniParser.g4
 * ============================================================================
 *
 * ZamaniParser.g4 must not directly duplicate policy-constraint rules.
 *
 * Its existing composition boundary remains:
 *
 *     ZamaniParser
 *         |
 *         +--> policy grammar through the policy/domain composition layer
 *
 * The root:
 *
 *     grammar/Zamani.g4
 *
 * must remain unchanged merely because this file is added.
 *
 * The root continues to import only:
 *
 *     ZamaniParser
 *     ZamaniLexer
 *
 *
 * ============================================================================
 * INTEGRATION WITH LEXER
 * ============================================================================
 *
 * This file intentionally introduces NO new lexical token.
 *
 * It consumes existing canonical tokens:
 *
 *     CONSTRAINT
 *     CONDITION
 *     EXPRESSION
 *     EXTENDS
 *     OVERRIDE
 *     METADATA
 *     ASSIGN
 *     SEMICOLON
 *     LBRACE
 *     RBRACE
 *     COMMA
 *
 * and canonical names/expressions.
 *
 * If a future construct requires a genuinely new reserved word, that token
 * must first be added to the canonical lexical authority:
 *
 *     grammar/lexer/
 *
 * and then exposed through:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This file must not invent local token definitions.
 *
 *
 * ============================================================================
 * INTEGRATION WITH RESOURCES
 * ============================================================================
 *
 * Resource-specific policy constraints consume this grammar.
 *
 * Example:
 *
 *     constraint resource::compute {
 *         condition resource::compute >= required_compute;
 *     }
 *
 * Resource resolution remains downstream.
 *
 *
 * ============================================================================
 * INTEGRATION WITH CAPABILITIES
 * ============================================================================
 *
 * Capability constraints use canonical capability names:
 *
 *     constraint capability::tensor::compute {
 *         condition capability::tensor::compute == true;
 *     }
 *
 * Capability discovery and negotiation remain outside the grammar.
 *
 *
 * ============================================================================
 * INTEGRATION WITH CONTRACTS
 * ============================================================================
 *
 * Validation may consume policy constraints as:
 *
 *     assumptions
 *     preconditions
 *     postconditions
 *     invariants
 *     guarantees
 *     properties
 *
 * No contract syntax is duplicated here.
 *
 *
 * ============================================================================
 * INTEGRATION WITH PROVENANCE
 * ============================================================================
 *
 * Policy constraint declarations must remain traceable through:
 *
 *     source
 *       ->
 *     AST
 *       ->
 *     semantic constraint
 *       ->
 *     policy evaluation
 *       ->
 *     compilation decision
 *       ->
 *     realization
 *
 * The provenance subsystem owns actual record creation.
 *
 *
 * ============================================================================
 * INTEGRATION WITH QUANTUM
 * ============================================================================
 *
 * Quantum policy constraints must remain independent of physical realization.
 *
 * This means:
 *
 *     constraint quantum::measurement {
 *         condition capability::quantum::measurement == true;
 *     }
 *
 * is acceptable.
 *
 * But this grammar must never become a grammar for:
 *
 *     QPU topology
 *     physical qubit allocation
 *     routing
 *     calibration
 *     pulse control
 *     vendor instructions
 *
 *
 * ============================================================================
 * INTEGRATION WITH HDL
 * ============================================================================
 *
 * HDL constraints may express intent such as:
 *
 *     constraint hardware::timing {
 *         condition hardware::timing == required_timing;
 *     }
 *
 * The actual timing analysis, synthesis, placement and verification are
 * downstream concerns.
 *
 *
 * ============================================================================
 * INTEGRATION WITH ADAPTIVE EXECUTION
 * ============================================================================
 *
 * A policy constraint may influence adaptive execution:
 *
 *     detect
 *     evaluate
 *     select
 *     fallback
 *     retry
 *     recover
 *     escalate
 *     reject
 *
 * The grammar records the condition.
 *
 * Runtime state transitions remain outside the grammar.
 *
 *
 * ============================================================================
 * INTEGRATION WITH DETERMINISM / REPRODUCIBILITY
 * ============================================================================
 *
 * Constraint evaluation may affect deterministic and reproducible compilation.
 *
 * The parser itself remains deterministic.
 *
 * Reproducibility metadata belongs to the semantic/provenance layers.
 *
 *
 * ============================================================================
 * INTEGRATION WITH SAFE RUST
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no embedded Rust
 *     no semantic actions
 *     no predicates
 *     no native code
 *     no unsafe code
 *
 * Generated Rust integration must remain compatible with:
 *
 *     Rust 1.97+
 *     Rust 2021
 *
 * Any implementation of AST construction, semantic validation, resource
 * negotiation, policy evaluation, provenance, compilation or runtime behavior
 * must remain safe Rust.
 *
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [x] It owns only policy-specific constraint structure.
 *
 * [x] Universal constraint expressions remain owned by core/constraints.g4.
 *
 * [x] It introduces no competing constraint expression language.
 *
 * [x] It introduces no hardware capacity limits.
 *
 * [x] It introduces no finite domain catalogue.
 *
 * [x] It introduces no vendor-specific syntax.
 *
 * [x] It introduces no physical quantum topology.
 *
 * [x] It introduces no HDL implementation details.
 *
 * [x] It introduces no runtime behavior.
 *
 * [x] It introduces no security enforcement.
 *
 * [x] It introduces no unsafe Rust.
 *
 * [x] It remains open-world.
 *
 * [x] It remains target-independent.
 *
 * [x] It preserves POCO-REAF.
 *
 * [x] It supports arbitrarily extensible semantic constraint names.
 *
 * [x] It supports reusable named constraints.
 *
 * [x] It supports constraint applicability.
 *
 * [x] It supports constraint composition.
 *
 * [x] It supports open-world metadata.
 *
 * [x] It preserves provenance.
 *
 * [ ] Policy.g4 imports PolicyConstraints.
 *
 * [ ] Policy.g4 delegates policyConstraint to policyConstraintDeclaration.
 *
 * [ ] ZamaniParser composition resolves the policy grammar.
 *
 * [ ] ANTLR generation succeeds.
 *
 * [ ] Rust generation succeeds on Rust 1.97+.
 *
 * [ ] Positive tests pass.
 *
 * [ ] Negative tests pass.
 *
 * [ ] Boundary tests pass.
 *
 * [ ] Scalability tests pass.
 *
 * [ ] Determinism tests pass.
 *
 * [ ] Compatibility tests pass.
 *
 *
 * ============================================================================
 * FINAL ARCHITECTURAL INVARIANT
 * ============================================================================
 *
 * There is exactly ONE universal constraint language:
 *
 *     grammar/core/constraints.g4
 *
 * and policy-specific constraint binding is owned here:
 *
 *     grammar/policies/constraints.g4
 *
 * Therefore:
 *
 *     universal constraint semantics
 *                |
 *                v
 *     policy constraint binding
 *                |
 *                v
 *     semantic policy model
 *                |
 *        +-------+-------+
 *        |       |       |
 *        v       v       v
 *     resource capability contract
 *        |       |       |
 *        +-------+-------+
 *                |
 *                v
 *     target-independent planning
 *                |
 *                v
 *     classical / quantum / HDL / hardware /
 *     distributed / AI / future realization
 *
 * The source constraint remains independent of the size, architecture,
 * vendor, topology, or physical realization of the target.
 *
 * ============================================================================
 */
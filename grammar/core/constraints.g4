/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/core/constraints.g4
 *
 * Grammar:
 *     Constraints
 *
 * Status:
 *     Canonical generic source-level constraint grammar.
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     Pure ANTLR parser grammar.
 *     No embedded Rust.
 *     No semantic predicates.
 *     No unsafe code.
 *     No filesystem access.
 *     No network access.
 *     No hardware discovery.
 *     No runtime execution.
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file defines the UNIVERSAL STRUCTURAL SYNTAX of Zamani constraints.
 *
 * A constraint expresses a condition that must hold for a semantic object,
 * type, declaration, computation, realization, or other language construct.
 *
 * Examples:
 *
 *     where T: Numeric;
 *
 *     where value >= minimum;
 *
 *     where resource::memory >= required_memory;
 *
 *     where quantum::logical_qubits >= required_qubits;
 *
 *     where capability::quantum::measurement == true;
 *
 *     where hardware::accelerated_compute == true;
 *
 *     where future::domain::property == expected;
 *
 * The grammar records structure.
 *
 * It does NOT decide:
 *
 *     - whether a constraint is satisfiable;
 *     - whether a capability exists;
 *     - whether a resource is available;
 *     - whether a target can realize the constraint;
 *     - whether a type bound is semantically valid;
 *     - whether two constraints conflict;
 *     - whether a constraint is decidable;
 *     - how a constraint is optimized;
 *     - how a constraint is lowered;
 *     - how hardware is selected.
 *
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns:
 *
 *     constraintClause
 *     optionalConstraintClause
 *
 *     constraintExpression
 *     constraintOrExpression
 *     constraintAndExpression
 *     constraintUnaryExpression
 *     constraintPrimary
 *
 *     constraintPredicate
 *     constraintComparisonOperator
 *
 *     constraintReference
 *     constraintReferenceList
 *     optionalConstraintReferenceList
 *
 *     constraintOperand
 *     constraintLiteralOperand
 *     constraintNamedCall
 *     constraintArgumentList
 *     constraintArgument
 *
 *     constraintTypeBound
 *     constraintTypeParameter
 *     constraintTypeBoundReference
 *
 *     constraintList
 *     optionalConstraintList
 *
 *     constraintBlock
 *     constraintEntry
 *     optionalConstraintBlock
 *
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     identifiers
 *     qualified names
 *     lexer rules
 *     literal lexical syntax
 *     general expression precedence
 *     general expression semantics
 *     type definitions
 *     generic parameter declarations
 *     capabilities
 *     requirements
 *     resources
 *     resource allocation
 *     target selection
 *     placement
 *     scheduling
 *     routing
 *     optimization
 *     quantum operations
 *     physical qubits
 *     QEC
 *     ZQN
 *     HAL
 *     HDL implementation
 *     hardware discovery
 *     runtime execution
 *     authorization
 *     policy evaluation
 *     provenance generation
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
 *     grammar/expressions/literals.g4
 *
 * EXPORTS:
 *
 *     constraintClause
 *     optionalConstraintClause
 *     constraintExpression
 *     constraintOrExpression
 *     constraintAndExpression
 *     constraintUnaryExpression
 *     constraintPrimary
 *     constraintPredicate
 *     constraintComparisonOperator
 *     constraintReference
 *     constraintReferenceList
 *     optionalConstraintReferenceList
 *     constraintOperand
 *     constraintNamedCall
 *     constraintArgumentList
 *     constraintArgument
 *     constraintTypeBound
 *     constraintTypeParameter
 *     constraintTypeBoundReference
 *     constraintList
 *     optionalConstraintList
 *     constraintBlock
 *     constraintEntry
 *     optionalConstraintBlock
 *
 * CONSUMED_BY:
 *
 *     grammar/ZamaniParser.g4
 *     grammar/core/program.g4
 *     grammar/core/source-unit.g4
 *     grammar/declarations/
 *     grammar/functions/
 *     grammar/types/
 *     grammar/modules/
 *     grammar/validation/
 *     grammar/resources/
 *     grammar/hardware/
 *     grammar/quantum/
 *     grammar/hybrid/
 *     grammar/hdl/
 *     grammar/classical/
 *     grammar/distributed/
 *     grammar/networking/
 *     grammar/ai/
 *     grammar/security/
 *     grammar/data/
 *     grammar/interoperability/
 *     grammar/compile/
 *     grammar/execution/
 *
 * AST_OWNER:
 *
 *     domain-neutral frontend AST subsystem
 *
 * SEMANTIC_OWNER:
 *
 *     semantic constraint model
 *
 * TYPE_OWNER:
 *
 *     canonical type-system implementation
 *
 * EFFECT_OWNER:
 *
 *     canonical effect subsystem
 *
 * CAPABILITY_OWNER:
 *
 *     grammar/core/capabilities.g4
 *     capability semantic subsystem
 *
 * RESOURCE_OWNER:
 *
 *     grammar/resources/
 *     resource semantic subsystem
 *
 * CONTRACT_OWNER:
 *
 *     grammar/validation/
 *
 * POLICY_OWNER:
 *
 *     grammar/policies/
 *     grammar/security/
 *
 * IR_OWNER:
 *
 *     canonical semantic representation
 *     downstream domain IRs
 *
 * TEST_OWNER:
 *
 *     grammar/tests/parser/
 *     grammar/tests/semantic/
 *     grammar/tests/contracts/
 *     grammar/tests/resources/
 *     grammar/tests/quantum/
 *     grammar/tests/hdl/
 *     grammar/tests/scalability/
 *     grammar/tests/portability/
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/resources.md
 *     grammar/spec/contracts.md
 *     grammar/spec/types.md
 *     grammar/specification/poco-reaf.md
 *
 *
 * ============================================================================
 * ARCHITECTURAL DISTINCTIONS
 * ============================================================================
 *
 * CAPABILITY
 * ----------
 *
 * What an environment can provide.
 *
 *
 * REQUIREMENT
 * -----------
 *
 * What a program requires from its realization environment.
 *
 *
 * CONSTRAINT
 * ----------
 *
 * A condition that must hold.
 *
 *
 * RESOURCE
 * --------
 *
 * A computational resource or measurable resource property.
 *
 *
 * PREFERENCE
 * ----------
 *
 * A non-mandatory preference among valid realizations.
 *
 *
 * POLICY
 * ------
 *
 * A rule governing permitted, prohibited, preferred, or fallback behavior.
 *
 *
 * CONTRACT
 * --------
 *
 * A semantic agreement involving assumptions, requirements, guarantees,
 * preconditions, postconditions, invariants, or properties.
 *
 *
 * These concepts may interact downstream but MUST NOT be collapsed into one
 * parser rule merely because their expressions look similar.
 *
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Constraints express PORTABLE SEMANTIC INTENT.
 *
 * They must not select physical implementation.
 *
 * A constraint may therefore survive realization changes involving:
 *
 *     embedded systems
 *     CPUs
 *     multicore CPUs
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
 *     heterogeneous systems
 *     future computational substrates
 *
 * Example:
 *
 *     where quantum::dynamic_control == true;
 *
 * does not select a QPU.
 *
 * Example:
 *
 *     where resource::memory >= required_memory;
 *
 * does not select a particular memory device.
 *
 * Example:
 *
 *     where hardware::accelerated_compute == true;
 *
 * does not select a particular accelerator.
 *
 *
 * ============================================================================
 * OPEN-WORLD CONTRACT
 * ============================================================================
 *
 * Constraint identities are OPEN-WORLD.
 *
 * This grammar MUST NOT enumerate:
 *
 *     CPU models
 *     GPU models
 *     FPGA families
 *     ASIC families
 *     QPU models
 *     accelerator models
 *     vendor capabilities
 *     AI models
 *     tensor limits
 *     network sizes
 *     hardware topologies
 *     future architectures
 *
 * New semantic names remain representable through qualifiedName.
 *
 * Examples:
 *
 *     future::architecture::property
 *
 *     quantum::dynamic_control
 *
 *     quantum::mid_circuit_measurement
 *
 *     tensor::compute
 *
 *     hardware::accelerated_compute
 *
 *     execution::deterministic
 *
 *     provenance::tracking
 *
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This file MUST NOT introduce language-level capacity constants such as:
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
 * It also MUST NOT encode:
 *
 *     cpu0
 *     gpu0
 *     fpga0
 *     qpu0
 *     node0
 *     device0
 *
 * as universal grammar constructs.
 *
 * A program value such as:
 *
 *     1024
 *
 * may legitimately occur in a constraint.
 *
 * That value is PROGRAM DATA.
 *
 * It does not establish a language-wide maximum.
 *
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * There is no grammar-level finite maximum for:
 *
 *     number of constraints
 *     number of clauses
 *     number of boolean terms
 *     number of nested groups
 *     qualified-name depth
 *     argument count
 *     list length
 *     program size
 *     number of domains
 *     number of resources
 *     number of devices
 *     number of nodes
 *     number of qubits
 *     number of CPUs
 *     number of GPUs
 *     tensor rank
 *
 * Repetition is represented with ANTLR repetition operators.
 *
 * Practical parser/compiler limits are implementation-resource limits, not
 * language semantics.
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
 * This grammar consumes:
 *
 *     qualifiedName
 *
 * It MUST NOT redefine:
 *
 *     identifier
 *     simpleName
 *     nameSegment
 *     qualifiedName
 *
 * Consequently:
 *
 *     quantum::measurement
 *     resource::memory
 *     hardware::accelerated_compute
 *     future::domain::property
 *
 * all use the same canonical name system as the rest of Zamani.
 *
 *
 * ============================================================================
 * LITERAL INTEGRATION
 * ============================================================================
 *
 * Literal syntax belongs to the canonical literal grammar:
 *
 *     grammar/expressions/literals.g4
 *
 * This grammar consumes:
 *
 *     literalExpression
 *
 * It MUST NOT reproduce literal token alternatives.
 *
 * This is important because the canonical literal layer already supports
 * evolving literal families without requiring this file to be rewritten.
 *
 *
 * ============================================================================
 * EXPRESSION BOUNDARY
 * ============================================================================
 *
 * General expression precedence belongs to:
 *
 *     grammar/expressions/expressions.g4
 *
 * This file deliberately does NOT import or recreate the complete expression
 * hierarchy.
 *
 * Reason:
 *
 *     expression
 *         ->
 *     assignment
 *         ->
 *     conditional
 *         ->
 *     logical
 *         ->
 *     comparison
 *         ->
 *     arithmetic
 *         ->
 *     postfix
 *         ->
 *     primary
 *
 * is already owned by the expression subsystem.
 *
 * A constraint is a semantic condition, not a second general-purpose
 * expression language.
 *
 * The constraint operand boundary therefore remains deliberately structural:
 *
 *     qualifiedName
 *     literalExpression
 *     qualifiedName(...)
 *
 * General expression integration can be performed by the semantic/frontend
 * composition layer where required without duplicating precedence here.
 *
 *
 * ============================================================================
 * BOOLEAN COMPOSITION
 * ============================================================================
 *
 * Constraint boolean precedence is:
 *
 *     NOT
 *       >
 *     AND
 *       >
 *     OR
 *
 * Therefore:
 *
 *     a or b and c
 *
 * means structurally:
 *
 *     a or (b and c)
 *
 * and:
 *
 *     not a and b
 *
 * means:
 *
 *     (not a) and b
 *
 * Parentheses always provide explicit grouping.
 *
 *
 * ============================================================================
 * NEGATION CONTRACT
 * ============================================================================
 *
 * `not` is syntactic.
 *
 * This grammar does not decide whether:
 *
 *     not capability::x
 *
 * means:
 *
 *     x must be absent
 *     x must not be selected
 *     x must not be required
 *     some domain-specific logical negation
 *
 * Semantic analysis determines the meaning.
 *
 *
 * ============================================================================
 * COMPARISON CONTRACT
 * ============================================================================
 *
 * The canonical relational operators are:
 *
 *     EQUAL_EQUAL
 *     NOT_EQUAL
 *     LESS
 *     LESS_EQUAL
 *     GREATER
 *     GREATER_EQUAL
 *
 * These are consumed from ZamaniLexer.
 *
 * Assignment:
 *
 *     ASSIGN
 *
 * is deliberately NOT a constraint equality operator.
 *
 * Therefore:
 *
 *     a == b
 *
 * is a valid relational constraint.
 *
 * while:
 *
 *     a = b
 *
 * is not a valid constraint predicate.
 *
 *
 * ============================================================================
 * TYPE-BOUND CONTRACT
 * ============================================================================
 *
 * Generic/type constraints use:
 *
 *     where T: Numeric;
 *
 *     where T: quantum::State;
 *
 *     where T: Serializable;
 *
 * This grammar records the structure.
 *
 * The type subsystem determines:
 *
 *     whether T is a type parameter;
 *     whether the bound exists;
 *     whether the bound is valid;
 *     whether multiple bounds are compatible;
 *     whether the bound is satisfied.
 *
 *
 * ============================================================================
 * RESOURCE BOUNDARY
 * ============================================================================
 *
 * Generic constraint syntax may reference resource properties:
 *
 *     where resource::memory >= required_memory;
 *
 * but this file does NOT own resource semantics.
 *
 * Resource-specific constraint syntax belongs to:
 *
 *     grammar/resources/constraints.g4
 *
 * Resource-domain grammar may compose with this generic constraint model.
 *
 * This prevents:
 *
 *     core/constraints.g4
 *
 * from becoming a second resource-expression grammar.
 *
 *
 * ============================================================================
 * CAPABILITY BOUNDARY
 * ============================================================================
 *
 * Capability declarations and capability identity semantics belong to:
 *
 *     grammar/core/capabilities.g4
 *
 * A constraint may reference a capability by name:
 *
 *     where capability::quantum::measurement == true;
 *
 * but this file does not resolve or validate that capability.
 *
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Quantum constraints remain target-neutral.
 *
 * Valid examples include:
 *
 *     where quantum::logical_qubits >= required_qubits;
 *
 *     where quantum::dynamic_control == true;
 *
 *     where quantum::mid_circuit_measurement == true;
 *
 *     where quantum::error_correction == true;
 *
 * This grammar does not represent:
 *
 *     physical qubit IDs
 *     coupling maps
 *     calibration
 *     gate duration
 *     routing
 *     scheduling
 *     decomposition
 *     QEC implementation
 *     backend identity
 *
 * If a constraint affects quantum compilation, the semantic result may
 * influence the pipeline:
 *
 *     source
 *       ->
 *     AST
 *       ->
 *     semantic model
 *       ->
 *     quantum::ir
 *       ->
 *     optimization
 *       ->
 *     decomposition
 *       ->
 *     routing
 *       ->
 *     scheduling
 *       ->
 *     resilience / QEC / ZQN
 *       ->
 *     HAL
 *       ->
 *     target realization
 *
 * This grammar itself never constructs quantum::ir.
 *
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Hardware-oriented constraints may express semantic properties:
 *
 *     where hardware::clocking == required_clocking;
 *
 *     where hardware::pipeline_support == true;
 *
 *     where hardware::memory_interface == required_interface;
 *
 *     where hardware::accelerated_compute == true;
 *
 * They do not encode:
 *
 *     fixed signal widths
 *     fixed registers
 *     fixed FPGA capacity
 *     fixed ASIC topology
 *     physical placement
 *     synthesis details
 *     timing closure
 *
 * Those belong downstream.
 *
 *
 * ============================================================================
 * AI / KNOWLEDGE / REASONING BOUNDARY
 * ============================================================================
 *
 * Constraints can refer to open semantic names associated with:
 *
 *     reasoning
 *     inference
 *     learning
 *     adaptation
 *     uncertainty
 *     evidence
 *     provenance
 *     agents
 *     policies
 *
 * Example:
 *
 *     where reasoning::inference == true;
 *
 *     where provenance::tracking == true;
 *
 * No AI-specific constraint vocabulary is hard-coded here.
 *
 *
 * ============================================================================
 * CONTRACT / POLICY BOUNDARY
 * ============================================================================
 *
 * This grammar represents logical constraint structure.
 *
 * Contract semantics belong to:
 *
 *     grammar/validation/
 *
 * Policy semantics belong to:
 *
 *     grammar/policies/
 *     grammar/security/
 *
 * A constraint can therefore be consumed by contracts and policies without
 * becoming either one.
 *
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * The parse tree must preserve sufficient source structure for the frontend
 * AST to retain:
 *
 *     source ordering
 *     operator kind
 *     grouping
 *     negation
 *     subject names
 *     operand structure
 *     type-bound structure
 *     argument structure
 *     source spans
 *
 * Provenance generation itself is downstream.
 *
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Conceptual structures:
 *
 *     ConstraintClause
 *         expression
 *         source_span
 *
 *     ConstraintExpression
 *         Predicate
 *         TypeBound
 *         Not
 *         All
 *         Any
 *         Group
 *
 *     ConstraintPredicate
 *         left
 *         operator
 *         right
 *
 *     ConstraintReference
 *         name
 *         source_span
 *
 *     ConstraintTypeBound
 *         parameter
 *         bound
 *         source_span
 *
 *     ConstraintCall
 *         name
 *         arguments
 *         source_span
 *
 * The exact Rust AST types belong to the frontend AST implementation.
 *
 * This grammar MUST NOT embed Rust AST definitions.
 *
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     name resolution
 *     type validity
 *     operator compatibility
 *     unit compatibility
 *     capability existence
 *     resource-property validity
 *     type-bound validity
 *     constraint conflict
 *     satisfiability
 *     decidability
 *     runtime evaluability
 *
 * A semantic constraint may eventually be classified as:
 *
 *     SATISFIED
 *     UNSATISFIED
 *     UNKNOWN
 *     CONDITIONAL
 *
 * The exact status vocabulary belongs to the semantic subsystem.
 *
 *
 * ============================================================================
 * DYNAMIC / RUNTIME CONSTRAINTS
 * ============================================================================
 *
 * A syntactically valid constraint may depend on runtime state.
 *
 * Example:
 *
 *     where execution::available_capacity >= required_capacity;
 *
 * The parser accepts it without inspecting runtime state.
 *
 * Runtime evaluation, when semantically required, belongs downstream.
 *
 * A runtime failure is therefore NOT a parser failure.
 *
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no semantic predicates;
 *     no embedded actions;
 *     no runtime callbacks;
 *     no filesystem access;
 *     no network access;
 *     no hardware discovery;
 *     no randomness.
 *
 * Given the same token stream and parser configuration, equivalent syntax
 * produces equivalent parse structure.
 *
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Parsing a constraint MUST NOT:
 *
 *     execute it;
 *     query hardware;
 *     reserve resources;
 *     grant capabilities;
 *     grant permissions;
 *     access files;
 *     access networks;
 *     load native code;
 *     invoke FFI;
 *     modify state.
 *
 * Parsing establishes syntax only.
 *
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust code.
 *
 * Generated parser/frontend/compiler code must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and safe Rust.
 *
 * No unsafe Rust is required by this grammar.
 *
 *
 * ============================================================================
 * ANTLR CONTRACT
 * ============================================================================
 */

parser grammar Constraints;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Literals;


/*
 * ============================================================================
 * GENERIC CONSTRAINT CLAUSE
 * ============================================================================
 *
 * Canonical source form:
 *
 *     where <constraint-expression>;
 *
 * The surrounding grammar determines where a clause is legal.
 *
 * This rule owns neither declaration placement nor generic-parameter
 * placement.
 */

constraintClause
    : WHERE constraintExpression SEMICOLON
    ;


optionalConstraintClause
    : constraintClause?
    ;


/*
 * ============================================================================
 * CONSTRAINT EXPRESSION
 * ============================================================================
 *
 * Precedence:
 *
 *     OR
 *       |
 *     AND
 *       |
 *     NOT
 *       |
 *     PRIMARY
 */

constraintExpression
    : constraintOrExpression
    ;


constraintOrExpression
    : constraintAndExpression
      (OR constraintAndExpression)*
    ;


constraintAndExpression
    : constraintUnaryExpression
      (AND constraintUnaryExpression)*
    ;


constraintUnaryExpression
    : NOT constraintUnaryExpression
    | constraintPrimary
    ;


constraintPrimary
    : LPAREN constraintExpression RPAREN
    | constraintPredicate
    | constraintTypeBound
    ;


/*
 * ============================================================================
 * CONSTRAINT PREDICATE
 * ============================================================================
 *
 * Supported forms:
 *
 *     value
 *
 *     value == value
 *
 *     value != value
 *
 *     value < value
 *
 *     value <= value
 *
 *     value > value
 *
 *     value >= value
 *
 * A bare reference is retained because it can represent a semantic predicate
 * whose truth is established downstream.
 */

constraintPredicate
    : constraintOperand constraintComparisonOperator constraintOperand
    | constraintOperand
    ;


constraintComparisonOperator
    : EQUAL_EQUAL
    | NOT_EQUAL
    | LESS
    | LESS_EQUAL
    | GREATER
    | GREATER_EQUAL
    ;


/*
 * ============================================================================
 * CONSTRAINT OPERAND
 * ============================================================================
 *
 * This is intentionally NOT the complete expression grammar.
 *
 * The universal core accepts:
 *
 *     symbolic references
 *     canonical literals
 *     named semantic calls
 *
 * More complex calculations should be represented through the canonical
 * expression subsystem and then connected to constraint semantics by the
 * frontend/semantic layer.
 */

constraintOperand
    : constraintReference
    | constraintLiteralOperand
    | constraintNamedCall
    ;


constraintLiteralOperand
    : literalExpression
    ;


constraintReference
    : qualifiedName
    ;


constraintNamedCall
    : qualifiedName
      LPAREN
      constraintArgumentList?
      RPAREN
    ;


constraintArgumentList
    : constraintArgument
      (COMMA constraintArgument)*
      COMMA?
    ;


constraintArgument
    : constraintOperand
    ;


/*
 * ============================================================================
 * TYPE BOUND CONSTRAINTS
 * ============================================================================
 *
 * Examples:
 *
 *     T: Numeric
 *
 *     T: quantum::State
 *
 *     T: Serializable
 *
 * Type semantics are downstream.
 */

constraintTypeBound
    : constraintTypeParameter
      COLON
      constraintTypeBoundReference
    ;


constraintTypeParameter
    : qualifiedName
    ;


constraintTypeBoundReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * REUSABLE REFERENCE LISTS
 * ============================================================================
 *
 * These rules are deliberately open-ended.
 */

constraintReferenceList
    : constraintReference
      (COMMA constraintReference)*
    ;


optionalConstraintReferenceList
    : constraintReferenceList?
    ;


/*
 * ============================================================================
 * REUSABLE CONSTRAINT LIST
 * ============================================================================
 *
 * A list represents independent constraint expressions.
 *
 * The surrounding grammar determines whether comma-separated constraints
 * have declaration-level semantics.
 */

constraintList
    : constraintExpression
      (COMMA constraintExpression)*
      COMMA?
    ;


optionalConstraintList
    : constraintList?
    ;


/*
 * ============================================================================
 * CONSTRAINT BLOCK
 * ============================================================================
 *
 * Optional structural form:
 *
 *     where {
 *         a == b;
 *         c >= d;
 *     }
 *
 * This is a grouping construct.
 *
 * It does not introduce another constraint declaration system.
 *
 * Surrounding grammar and language-version policy determine where this form
 * is legal.
 */

constraintBlock
    : WHERE
      LBRACE
      constraintEntry*
      RBRACE
    ;


constraintEntry
    : constraintExpression SEMICOLON
    ;


optionalConstraintBlock
    : constraintBlock?
    ;


/*
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Resource-specific syntax belongs to:
 *
 *     grammar/resources/constraints.g4
 *
 * That grammar owns resource-domain atoms such as:
 *
 *     resource quantities
 *     capacity
 *     availability
 *     performance
 *     latency
 *     throughput
 *     bandwidth
 *     energy
 *     power
 *     reliability
 *     resilience
 *     scalability
 *     portability
 *     resource compatibility
 *     resource capabilities
 *
 * It MUST NOT replace this generic constraint grammar.
 *
 * Conceptually:
 *
 *     generic constraint
 *          |
 *          +--> generic symbolic predicate
 *          |
 *          +--> resource-domain semantic predicate
 *          |
 *          +--> type-bound predicate
 *          |
 *          +--> future-domain predicate
 *
 * Resource realization remains downstream.
 *
 *
 * ============================================================================
 * REQUIREMENT INTEGRATION
 * ============================================================================
 *
 * Requirements are owned by:
 *
 *     grammar/core/requirements.g4
 *
 * Example:
 *
 *     requires quantum::measurement;
 *
 * is not the same construct as:
 *
 *     where quantum::measurement == true;
 *
 * Requirement semantics remain separate.
 *
 *
 * ============================================================================
 * CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Capabilities are owned by:
 *
 *     grammar/core/capabilities.g4
 *
 * This grammar may reference capability names but does not define them.
 *
 *
 * ============================================================================
 * TYPE INTEGRATION
 * ============================================================================
 *
 * Type-specific constraint placement belongs to the type subsystem.
 *
 * This file supplies the reusable structural rule:
 *
 *     constraintTypeBound
 *
 * The type subsystem decides:
 *
 *     where
 *     when
 *     and how
 *
 * a type constraint may appear.
 *
 *
 * ============================================================================
 * FUNCTION INTEGRATION
 * ============================================================================
 *
 * Function grammar may compose:
 *
 *     optionalConstraintClause
 *
 * or its internal constraint-expression rules according to the canonical
 * function syntax.
 *
 * Function grammar owns placement relative to:
 *
 *     parameters
 *     return types
 *     bodies
 *     generic parameters
 *
 * This file owns only constraint structure.
 *
 *
 * ============================================================================
 * MODULE INTEGRATION
 * ============================================================================
 *
 * Module-level constraints may reuse:
 *
 *     constraintExpression
 *     constraintClause
 *
 * The module grammar determines placement and scope.
 *
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum grammar may use generic constraints for:
 *
 *     capability conditions
 *     logical-resource conditions
 *     execution conditions
 *     dynamic-control conditions
 *     measurement conditions
 *     resilience properties
 *
 * Example:
 *
 *     where quantum::dynamic_control == true;
 *
 * The quantum grammar MUST NOT duplicate this boolean constraint hierarchy.
 *
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * HDL and hardware grammar may reuse generic constraints for:
 *
 *     semantic hardware properties
 *     interface requirements
 *     timing properties
 *     implementation capabilities
 *     execution properties
 *
 * Physical realization remains outside this grammar.
 *
 *
 * ============================================================================
 * AI INTEGRATION
 * ============================================================================
 *
 * AI and reasoning grammar may reference open semantic properties:
 *
 *     reasoning::inference
 *     learning::adaptation
 *     provenance::tracking
 *     uncertainty::representation
 *     agent::coordination
 *
 * No AI-specific constraint vocabulary is required.
 *
 *
 * ============================================================================
 * DISTRIBUTED / NETWORKING INTEGRATION
 * ============================================================================
 *
 * Distributed and networking grammar may consume generic constraints for:
 *
 *     communication
 *     latency
 *     consistency
 *     availability
 *     reliability
 *     security
 *     topology properties
 *
 * No finite topology is encoded.
 *
 *
 * ============================================================================
 * INTEROPERABILITY INTEGRATION
 * ============================================================================
 *
 * FFI and ABI subsystems may consume constraint semantics to express:
 *
 *     ABI compatibility
 *     foreign capability conditions
 *     calling-convention requirements
 *     representation conditions
 *
 * This grammar does not implement ABI validation.
 *
 *
 * ============================================================================
 * PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Constraints can contribute semantic provenance such as:
 *
 *     source constraint
 *     normalized constraint
 *     evaluated constraint
 *     evidence
 *     decision
 *
 * Provenance recording is downstream.
 *
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Policies may consume constraint expressions to evaluate:
 *
 *     requirements
 *     restrictions
 *     preferences
 *     fallback conditions
 *     authorization conditions
 *
 * Policy evaluation is not parser responsibility.
 *
 *
 * ============================================================================
 * IR INTEGRATION
 * ============================================================================
 *
 * Constraints are NOT ordinary computation operations.
 *
 * They may influence:
 *
 *     semantic model
 *     compilation decisions
 *     target negotiation
 *     specialization
 *     optimization legality
 *     execution planning
 *     runtime validation
 *
 * The pipeline is:
 *
 *     source
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic constraint model
 *       |
 *       +-------------------+
 *       |                   |
 *       v                   v
 *   compile-time        runtime validation
 *       |
 *       v
 * canonical semantic representation
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> HDL/hardware representation
 *
 * Constraints do not themselves become:
 *
 *     physical instructions
 *     quantum gates
 *     hardware signals
 *     resource allocations
 *     scheduler commands
 *
 *
 * ============================================================================
 * QUANTUM IR BOUNDARY
 * ============================================================================
 *
 * No rule in this file creates or references physical quantum IR structures.
 *
 * A quantum constraint can affect the semantic legality of a quantum program,
 * after which the quantum subsystem may lower the resulting program toward:
 *
 *     quantum::ir
 *
 * This preserves the rule:
 *
 *     grammar != quantum backend
 *
 *
 * ============================================================================
 * HDL BOUNDARY
 * ============================================================================
 *
 * No physical HDL implementation is encoded here.
 *
 * A constraint can influence:
 *
 *     hardware intent
 *     synthesis legality
 *     verification conditions
 *     implementation selection
 *
 * but actual synthesis and physical realization remain downstream.
 *
 *
 * ============================================================================
 * BACKEND BOUNDARY
 * ============================================================================
 *
 * No backend-specific token or rule belongs here.
 *
 * New backend:
 *
 *     new capability/resource/property
 *
 * must be representable by existing open-world name syntax whenever it does
 * not introduce genuinely new source-language syntax.
 *
 * Therefore adding:
 *
 *     a new GPU architecture
 *     a new QPU capability
 *     a new FPGA technology
 *     a new accelerator
 *     a new distributed substrate
 *     a new AI execution model
 *     a new hardware architecture
 *
 * should normally require semantic registry/specification changes rather than
 * modifying this grammar.
 *
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics should identify structural failures such as:
 *
 *     missing WHERE expression
 *     missing operand
 *     missing comparison operator
 *     malformed comparison
 *     malformed type bound
 *     missing qualified-name segment
 *     missing closing parenthesis
 *     missing closing brace
 *     malformed argument list
 *     malformed separator
 *
 * Semantic diagnostics belong downstream:
 *
 *     unknown name
 *     invalid type
 *     incompatible operands
 *     invalid unit
 *     unavailable capability
 *     unsatisfied resource condition
 *     unsatisfiable constraint
 *     conflicting constraints
 *     forbidden policy
 *     unavailable target
 *
 *
 * ============================================================================
 * POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * The conformance suite MUST accept at minimum:
 *
 *     where a;
 *
 *     where a == b;
 *
 *     where a != b;
 *
 *     where a < b;
 *
 *     where a <= b;
 *
 *     where a > b;
 *
 *     where a >= b;
 *
 *     where not a;
 *
 *     where a and b;
 *
 *     where a or b;
 *
 *     where a and b or c;
 *
 *     where (a or b) and c;
 *
 *     where quantum::measurement == true;
 *
 *     where quantum::logical_qubits >= required_qubits;
 *
 *     where resource::memory >= required_memory;
 *
 *     where hardware::accelerated_compute == true;
 *
 *     where execution::deterministic == true;
 *
 *     where future::architecture::property == expected;
 *
 *     where T: Numeric;
 *
 *     where T: quantum::State;
 *
 *     where capability::tensor::compute == true;
 *
 *     where capability::feature("tensor.compute");
 *
 *     where resource::property(required);
 *
 *     where {
 *         a == b;
 *         c >= d;
 *     }
 *
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The parser MUST reject:
 *
 *     where;
 *
 *     where and a;
 *
 *     where a and;
 *
 *     where or a;
 *
 *     where not;
 *
 *     where a = b;
 *
 *     where a === b;
 *
 *     where a <> b;
 *
 *     where T;
 *
 *     where : T;
 *
 *     where T:;
 *
 *     where (a == b;
 *
 *     where a == b);
 *
 *     where a ==;
 *
 *     where == b;
 *
 *     where capability::feature(;
 *
 *     where capability::feature();
 *     // This particular form may be accepted structurally if the semantic
 *     // language permits zero-argument named calls. Semantic validation must
 *     // determine whether it is legal.
 *
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Test:
 *
 *     deeply nested boolean expressions;
 *     deeply qualified names;
 *     long constraint lists;
 *     long argument lists;
 *     large integer literals;
 *     long source units;
 *     many independent constraints;
 *     nested constraint blocks;
 *     Unicode identifiers accepted by the canonical lexer;
 *     symbolic future-domain properties;
 *
 * No test may turn its chosen size into a language-level maximum.
 *
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Scalability tests must progressively exercise:
 *
 *     constraint count
 *     boolean-expression depth
 *     qualification depth
 *     argument count
 *     source size
 *
 * until practical implementation/resource limits are observed.
 *
 * Those limits are measurements of the implementation environment.
 *
 * They are NOT grammar semantics.
 *
 *
 * ============================================================================
 * CROSS-DOMAIN TEST CONTRACT
 * ============================================================================
 *
 * The same generic constraint grammar must work with:
 *
 *     classical computation
 *     numerical computation
 *     tensor computation
 *     quantum computation
 *     hybrid computation
 *     HDL
 *     hardware/software co-design
 *     AI
 *     reasoning
 *     learning
 *     adaptation
 *     uncertainty
 *     agents
 *     distributed computation
 *     networking
 *     cryptography
 *     security
 *     FFI
 *     ABI
 *     simulation
 *     metaprogramming
 *     embedded systems
 *     accelerators
 *     HPC
 *
 * without adding a new core constraint rule for every domain.
 *
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing syntax:
 *
 *     where ...
 *
 * remains canonical.
 *
 * Existing comparison forms:
 *
 *     ==
 *     !=
 *     <
 *     <=
 *     >
 *     >=
 *
 * remain canonical.
 *
 * Existing type-bound form:
 *
 *     T: Bound
 *
 * remains supported.
 *
 * Existing resource-specific constraints remain delegated to the resource
 * subsystem.
 *
 * Existing general mathematical expressions remain owned by the expression
 * subsystem.
 *
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains:
 *
 *     NO hardware capacities.
 *     NO machine counts.
 *     NO qubit limits.
 *     NO CPU limits.
 *     NO GPU limits.
 *     NO FPGA limits.
 *     NO node limits.
 *     NO memory limits.
 *     NO thread limits.
 *     NO tensor-rank limits.
 *     NO register-width limits.
 *     NO network-size limits.
 *     NO topology limits.
 *     NO vendor-device catalogue.
 *     NO quantum-gate catalogue.
 *     NO backend selection.
 *     NO physical placement.
 *     NO routing.
 *     NO scheduling.
 *     NO calibration.
 *     NO QEC implementation.
 *
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [ ] Constraints is the canonical generic constraint parser grammar.
 *
 *     [ ] tokenVocab is ZamaniLexer.
 *
 *     [ ] Names is the canonical name dependency.
 *
 *     [ ] Literals is the canonical literal dependency.
 *
 *     [ ] No literal token aliases are duplicated here.
 *
 *     [ ] No identifier syntax is duplicated here.
 *
 *     [ ] No qualified-name syntax is duplicated here.
 *
 *     [ ] No complete expression precedence hierarchy is duplicated here.
 *
 *     [ ] Generic boolean constraint precedence is deterministic.
 *
 *     [ ] Type-bound syntax is represented structurally.
 *
 *     [ ] Resource-specific constraint syntax remains in resources/.
 *
 *     [ ] Capability semantics remain outside this file.
 *
 *     [ ] Requirement semantics remain outside this file.
 *
 *     [ ] Policy semantics remain outside this file.
 *
 *     [ ] Contract semantics remain outside this file.
 *
 *     [ ] Quantum physical realization remains outside this file.
 *
 *     [ ] HDL physical realization remains outside this file.
 *
 *     [ ] No hardware capacity is hard-coded.
 *
 *     [ ] No artificial language-level scalability limit exists.
 *
 *     [ ] Unknown future-domain names remain syntactically representable.
 *
 *     [ ] No embedded Rust exists.
 *
 *     [ ] No unsafe Rust is required.
 *
 *     [ ] Rust 1.97 compatibility is verified.
 *
 *     [ ] Rust 1.97.1 compatibility is verified.
 *
 *     [ ] ANTLR generation succeeds.
 *
 *     [ ] Root parser composition succeeds.
 *
 *     [ ] Positive tests pass.
 *
 *     [ ] Negative tests pass.
 *
 *     [ ] Boundary tests pass.
 *
 *     [ ] Scalability tests pass.
 *
 *     [ ] Cross-domain tests pass.
 *
 *     [ ] Determinism tests pass.
 *
 *     [ ] Compatibility tests pass.
 *
 *
 * ============================================================================
 * FINAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * A constraint describes semantic conditions.
 *
 * It does not describe a particular machine.
 *
 * It does not describe a particular backend.
 *
 * It does not describe a physical quantum topology.
 *
 * It does not describe a fixed hardware capacity.
 *
 * It does not allocate resources.
 *
 * It does not perform execution.
 *
 * Therefore:
 *
 *     source constraint
 *         ->
 *     domain-neutral AST
 *         ->
 *     semantic constraint
 *         ->
 *     capability/resource/policy/contract analysis
 *         ->
 *     compilation decision
 *         ->
 *     target realization
 *
 * This separation is a fundamental requirement for:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * ============================================================================
 */
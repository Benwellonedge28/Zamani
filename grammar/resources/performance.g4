/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/resources/performance.g4
 *
 * GRAMMAR
 * -------
 * ResourcePerformance
 *
 * PURPOSE
 * -------
 * Canonical source-level performance intent.
 *
 * This grammar describes performance intent without performing:
 *
 *     benchmarking
 *     measurement
 *     hardware discovery
 *     target selection
 *     allocation
 *     scheduling
 *     routing
 *     optimization
 *     runtime execution
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * OWNS
 * ----
 *
 *     performance declaration
 *     performance specification
 *     performance groups
 *     performance clauses
 *     performance metric references
 *     performance relations
 *     performance objectives
 *     performance targets
 *     performance properties
 *
 * DOES NOT OWN
 * -------------
 *
 *     lexer
 *     identifiers
 *     qualified names
 *     expressions
 *     units
 *     resource discovery
 *     capability discovery
 *     benchmarking
 *     scheduling
 *     routing
 *     physical hardware
 *     quantum operations
 *     quantum::ir
 *
 * ============================================================================
 * DEPENDENCIES
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/resources/resource-expressions.g4
 *     grammar/core/names.g4
 *
 * EXPORTS
 * -------
 *
 *     resourcePerformance
 *     resourcePerformanceDeclaration
 *     resourcePerformanceSpecification
 *     resourcePerformanceGroup
 *     resourcePerformanceClause
 *     resourcePerformanceRelation
 *     resourcePerformanceMetric
 *     resourcePerformanceExpression
 *
 * CONSUMED_BY
 * -----------
 *
 *     grammar/resources/resources.g4
 *     grammar/hardware/performance.g4
 *     grammar/hardware/resources.g4
 *     grammar/quantum/quantum-resources.g4
 *     future resource-domain grammars
 *
 * AST_OWNER
 * ---------
 *
 *     Domain-neutral frontend AST
 *
 * SEMANTIC_OWNER
 * -------------
 *
 *     Resource/performance semantic analysis
 *
 * IR_OWNER
 * --------
 *
 *     Canonical semantic resource model
 *
 * TEST_OWNER
 * ----------
 *
 *     grammar/tests/resources/performance/
 *
 * SPEC_OWNER
 * ----------
 *
 *     grammar/spec/resources.md
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Performance syntax MUST NOT establish universal hardware limits.
 *
 * No limit is imposed on:
 *
 *     machines
 *     CPUs
 *     cores
 *     threads
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     QPUs
 *     qubits
 *     nodes
 *     memory
 *     storage
 *     objectives
 *     properties
 *     groups
 *     namespace depth
 *
 * A numeric literal is program data, not a language-level machine limit.
 *
 * ============================================================================
 */

parser grammar ResourcePerformance;

options {
    tokenVocab = ZamaniLexer;
}

import
    ResourceExpressions,
    Names
;


/*
 * ============================================================================
 * 1. PUBLIC ENTRY
 * ============================================================================
 *
 * Zero or more performance constructs.
 */
resourcePerformance
    : resourcePerformanceItem*
    ;


/*
 * ============================================================================
 * 2. PERFORMANCE ITEM
 * ============================================================================
 */
resourcePerformanceItem
    : resourcePerformanceDeclaration
    | resourcePerformanceSpecification
    | resourcePerformanceGroup
    ;


/*
 * ============================================================================
 * 3. SINGLE PERFORMANCE DECLARATION
 * ============================================================================
 *
 * Portable forms:
 *
 *     performance = expression;
 *
 *     performance metric >= expression;
 *
 *     performance metric <= expression;
 *
 * The metric is symbolic.
 */
resourcePerformanceDeclaration
    : PERFORMANCE ASSIGN resourcePerformanceExpression SEMICOLON
    | PERFORMANCE resourcePerformanceMetric
      resourcePerformanceRelation
      SEMICOLON
    ;


/*
 * ============================================================================
 * 4. STRUCTURED PERFORMANCE SPECIFICATION
 * ============================================================================
 *
 * Example:
 *
 *     performance {
 *         throughput = desired_throughput;
 *         latency = latency_budget;
 *         bandwidth = required_bandwidth;
 *     }
 *
 * The number of clauses is unbounded by the grammar.
 */
resourcePerformanceSpecification
    : PERFORMANCE
      LBRACE
      resourcePerformanceClause*
      RBRACE
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 5. PERFORMANCE GROUP
 * ============================================================================
 *
 * Example:
 *
 *     performance group execution {
 *         throughput = desired_throughput;
 *         latency = latency_budget;
 *     }
 *
 * The group name is symbolic.
 */
resourcePerformanceGroup
    : PERFORMANCE
      GROUP
      qualifiedName
      LBRACE
      resourcePerformanceClause*
      RBRACE
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 6. PERFORMANCE CLAUSE
 * ============================================================================
 *
 * Known semantic categories use canonical lexer tokens.
 *
 * Future properties may use qualified symbolic names.
 */
resourcePerformanceClause
    : resourcePerformanceObjectiveClause
    | resourcePerformanceTargetClause
    | resourcePerformancePropertyClause
    ;


/*
 * ============================================================================
 * 7. OBJECTIVE
 * ============================================================================
 *
 * Example:
 *
 *     objective = throughput >= desired_throughput;
 *
 * Objective semantics are resolved downstream.
 */
resourcePerformanceObjectiveClause
    : OBJECTIVE
      ASSIGN
      resourcePerformanceRelation
      SEMICOLON
    ;


/*
 * ============================================================================
 * 8. TARGET
 * ============================================================================
 *
 * Example:
 *
 *     target = desired_throughput;
 *
 * A target is not automatically a hard requirement.
 */
resourcePerformanceTargetClause
    : TARGET
      ASSIGN
      resourcePerformanceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 9. OPEN PROPERTY
 * ============================================================================
 *
 * Example:
 *
 *     custom::metric = desired_value;
 *
 * Future semantic properties do not require a new parser rule.
 */
resourcePerformancePropertyClause
    : resourcePerformanceMetric
      ASSIGN
      resourcePerformanceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 10. METRIC
 * ============================================================================
 *
 * A metric is either:
 *
 *     a qualified symbolic name
 *
 * or a currently reserved performance-domain name.
 *
 * Reserved metric words are included here because the canonical lexer reserves
 * them as tokens rather than emitting them as IDENTIFIER.
 */
resourcePerformanceMetric
    : qualifiedName
    | PERFORMANCE
    | LATENCY
    | THROUGHPUT
    | BANDWIDTH
    | ENERGY
    | POWER
    | RELIABILITY
    | RESILIENCE
    | COST
    | SCALABILITY
    | CAPACITY
    | AVAILABILITY
    | PORTABILITY
    ;


/*
 * ============================================================================
 * 11. PERFORMANCE RELATION
 * ============================================================================
 *
 * Comparison operators come from the canonical lexer.
 *
 * No comparison precedence is redefined here.
 */
resourcePerformanceRelation
    : resourcePerformanceExpression
      resourcePerformanceOperator
      resourcePerformanceExpression
    ;


/*
 * ============================================================================
 * 12. PERFORMANCE OPERATOR
 * ============================================================================
 */
resourcePerformanceOperator
    : EQUAL_EQUAL
    | NOT_EQUAL
    | LESS_EQUAL
    | GREATER_EQUAL
    | LESS
    | GREATER
    ;


/*
 * ============================================================================
 * 13. PERFORMANCE EXPRESSION
 * ============================================================================
 *
 * ResourceExpressions owns the expression hierarchy.
 */
resourcePerformanceExpression
    : resourceExpression
    ;


/*
 * ============================================================================
 * 14. OPTIONAL PERFORMANCE
 * ============================================================================
 */
optionalResourcePerformance
    : resourcePerformance?
    ;


/*
 * ============================================================================
 * 15. COMPLETION CRITERIA
 * ============================================================================
 *
 * DONE means:
 *
 * [ ] ResourcePerformance compiles as an ANTLR parser grammar.
 *
 * [ ] ResourceExpressions resolves.
 *
 * [ ] Names resolves.
 *
 * [ ] Every referenced lexer token exists in ZamaniLexer.
 *
 * [ ] No K_* token aliases are invented here.
 *
 * [ ] No second expression language exists.
 *
 * [ ] No hardware-specific physical identifiers exist.
 *
 * [ ] No fixed resource limits exist.
 *
 * [ ] No fixed objective count exists.
 *
 * [ ] No fixed group size exists.
 *
 * [ ] No fixed metric count exists.
 *
 * [ ] Unknown future properties can be represented through qualified names.
 *
 * [ ] Performance intent remains distinct from resource requirements.
 *
 * [ ] Performance intent remains distinct from constraints.
 *
 * [ ] Performance intent remains distinct from preferences.
 *
 * [ ] Performance intent remains distinct from hints.
 *
 * [ ] Performance intent remains distinct from measurement.
 *
 * [ ] Hardware can consume the grammar without duplicating it.
 *
 * [ ] Quantum can consume performance semantics without introducing another
 *     quantum IR.
 *
 * [ ] HDL can consume performance semantics downstream.
 *
 * [ ] Classical/AI/data/distributed/networking domains can consume the same
 *     performance model.
 *
 * [ ] Positive tests exist.
 *
 * [ ] Negative tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Scalability tests exist.
 *
 * [ ] Determinism tests exist.
 *
 * [ ] Cross-domain tests exist.
 *
 * [ ] Compatibility tests exist.
 *
 * [ ] Generated Rust is compatible with Rust 1.97 or later.
 *
 * [ ] No unsafe Rust is required.
 */
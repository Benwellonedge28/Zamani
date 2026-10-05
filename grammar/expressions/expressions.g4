/*
 * ============================================================================
 * ZAMANI PROGRAMMING LANGUAGE
 * ============================================================================
 *
 * File:
 *     grammar/expressions/expressions.g4
 *
 * Role:
 *     CANONICAL EXPRESSION ORCHESTRATOR
 *
 * Status:
 *     PRODUCTION COMPOSITION ROOT
 *
 * Grammar:
 *     ANTLR4 parser grammar
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021 edition
 *     Safe Rust only
 *     No unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the SINGLE PUBLIC COMPOSITION ROOT for Zamani expressions.
 *
 * It owns:
 *
 *     1. expression
 *     2. the complete expression precedence ladder
 *     3. the ordering of expression layers
 *     4. the integration of specialized expression grammars
 *     5. primary-expression composition
 *     6. expression-list composition
 *     7. generic/type-expression list composition needed by expressions
 *
 * It DOES NOT implement the specialized expression features themselves.
 *
 * Specialized grammars own their respective syntax.
 *
 * This distinction is mandatory for repository scalability.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     ZamaniParser
 *          |
 *          v
 *     Expressions
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +----------------------+-----------------------+
 *          |                      |                       |
 *          v                      v                       v
 *     classical IR          quantum::ir             HDL/hardware
 *          |                      |                       |
 *          +----------------------+-----------------------+
 *                                 |
 *                                 v
 *                          optimization
 *                                 |
 *                                 v
 *                     lowering / specialization
 *                                 |
 *                         routing / scheduling
 *                                 |
 *                       resilience / recovery
 *                                 |
 *                         QEC where applicable
 *                                 |
 *                                ZQN
 *                                 |
 *                                HAL
 *                                 |
 *                         target realization
 *
 * This grammar creates NO IR.
 *
 * quantum::ir remains the canonical quantum semantic boundary.
 *
 * ============================================================================
 * FILE CONTRACT
 * ============================================================================
 *
 * OWNS
 * ----
 *
 *     expression
 *     precedence composition
 *     expression-layer ordering
 *     primaryExpression composition
 *     parenthesizedExpression
 *     newExpression
 *     thisExpression
 *     superExpression
 *     expressionList
 *     typeExpressionList
 *
 * DOES NOT OWN
 * -------------
 *
 *     lexer rules
 *     token spellings
 *     identifiers
 *     names
 *     types
 *     assignment implementation
 *     conditional implementation
 *     range implementation
 *     unary implementation
 *     postfix implementation
 *     calls
 *     indexing
 *     member access
 *     literals
 *     arrays
 *     maps
 *     tuples
 *     comprehensions
 *     lambdas
 *     closures
 *     pattern implementation
 *     guards
 *     match implementation
 *     quantum implementation
 *     reasoning implementation
 *     knowledge implementation
 *     uncertainty implementation
 *     policy implementation
 *     effect implementation
 *     compile-time implementation
 *     metaprogramming implementation
 *     AST implementation
 *     semantic analysis
 *     type checking
 *     effect checking
 *     capability checking
 *     resource analysis
 *     contract checking
 *     policy evaluation
 *     provenance
 *     classical IR
 *     quantum::ir
 *     HDL IR
 *     optimization
 *     lowering
 *     routing
 *     scheduling
 *     QEC
 *     ZQN
 *     HAL
 *     runtime execution
 *
 * ============================================================================
 * SINGLE-AUTHORITY INVARIANT
 * ============================================================================
 *
 * There MUST be exactly one public expression root:
 *
 *     expression
 *
 * This file owns it.
 *
 * No imported expression grammar may define another public `expression`.
 *
 * Likewise, this file MUST NOT duplicate the implementation of:
 *
 *     assignmentExpression
 *     conditionalExpression
 *     rangeExpression
 *     unaryExpression
 *     postfixExpression
 *     literalExpression
 *     matchExpression
 *     quantumExpression
 *
 * Those rules belong to their dedicated grammars.
 *
 * ============================================================================
 * IMPORT ARCHITECTURE
 * ============================================================================
 *
 * ANTLR imports are used as composition/delegation boundaries.
 *
 * The imported grammars are deliberately limited to canonical feature owners.
 *
 * Historical/legacy expression grammars such as:
 *
 *     arithmetic.g4
 *     binary.g4
 *     bitwise.g4
 *     comparison.g4
 *     logical.g4
 *     shift.g4
 *
 * are NOT imported here because this file owns the single precedence ladder.
 *
 * They may remain as compatibility/specification material until their
 * migration is complete, but they must not become competing authorities.
 *
 * Likewise, duplicate/legacy forms such as:
 *
 *     lambda.g4
 *     closure.g4
 *     range.g4
 *
 * must not be introduced as competing authorities when their canonical
 * plural/feature grammars are already selected below.
 *
 * ============================================================================
 */

parser grammar Expressions;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * CANONICAL COMPONENT IMPORTS
 * ============================================================================
 *
 * Each imported grammar has one architectural responsibility.
 *
 * AssignmentExpressions
 *     assignment syntax
 *
 * ConditionalsParser
 *     conditional expressions
 *
 * Ranges
 *     range expressions
 *
 * Unary
 *     prefix/unary expressions
 *
 * Postfix
 *     calls, indexing, member access, updates and postfix composition
 *
 * Literals
 *     literal expressions
 *
 * Arrays
 * Maps
 * Tuples
 *     collection/value constructors
 *
 * Lambdas
 *     lambda expressions
 *
 * ExpressionIdentifiers
 *     identifier and qualified-name expressions
 *
 * Comprehensions
 *     comprehension expressions
 *
 * Patterns
 * Guards
 * MatchExpressions
 *     pattern matching
 *
 * QuantumExpressions
 *     source-level quantum expressions
 *
 * Reasoning
 * KnowledgeExpressions
 * UncertaintyExpressions
 *     open-world semantic expression capabilities
 *
 * ZamaniExpressionQuery
 * PolicyExpressions
 * EffectExpressions
 *     query/policy/effect expression surfaces
 *
 * CompileTimeExpressions
 * MetaprogrammingExpressions
 *     compile-time and reflective expression surfaces
 *
 * ZamaniExpressionBlocks
 *     expression blocks
 *
 * IMPORTANT:
 *
 * Imported grammars are components.
 *
 * This file remains responsible for deciding how those components participate
 * in the single expression precedence hierarchy.
 */
import
    AssignmentExpressions,
    ConditionalsParser,
    Ranges,
    Unary,
    Postfix,
    Literals,
    Arrays,
    Maps,
    Tuples,
    Lambdas,
    ExpressionIdentifiers,
    Comprehensions,
    Patterns,
    Guards,
    MatchExpressions,
    QuantumExpressions,
    Reasoning,
    KnowledgeExpressions,
    UncertaintyExpressions,
    ZamaniExpressionQuery,
    PolicyExpressions,
    EffectExpressions,
    CompileTimeExpressions,
    MetaprogrammingExpressions,
    ZamaniExpressionBlocks
;


/*
 * ============================================================================
 * 1. PUBLIC EXPRESSION ENTRY POINT
 * ============================================================================
 *
 * This is the only public expression root.
 *
 * All expression consumers across Zamani MUST enter through this rule unless
 * they intentionally consume a more specialized expression rule for tooling
 * or diagnostics.
 *
 * Examples of consumers include:
 *
 *     declarations
 *     statements
 *     functions
 *     modules
 *     contracts
 *     policies
 *     resources
 *     capabilities
 *     effects
 *     classical computation
 *     quantum computation
 *     hybrid computation
 *     HDL
 *     hardware intent
 *     AI/ML
 *     data
 *     distributed computation
 *     networking
 *     metaprogramming
 *     compile-time evaluation
 *
 * No target or domain is selected here.
 */
expression
    : assignmentExpression
    ;


/*
 * ============================================================================
 * 2. ASSIGNMENT
 * ============================================================================
 *
 * AssignmentExpressions owns the actual assignment implementation.
 *
 * This rule is intentionally only a composition boundary.
 *
 * DO NOT implement assignment operators here.
 */
assignmentExpression
    : assignmentExpressionDelegate
    ;

assignmentExpressionDelegate
    : AssignmentExpressions_assignmentExpression
    ;


/*
 * ============================================================================
 * 3. CONDITIONAL
 * ============================================================================
 *
 * ConditionalsParser owns conditional syntax.
 *
 * Conditional expressions remain value-producing expressions and are therefore
 * valid anywhere an expression is expected.
 */
conditionalExpression
    : ConditionalsParser_conditionalExpression
    ;


/*
 * ============================================================================
 * 4. RANGE
 * ============================================================================
 *
 * Ranges owns range syntax.
 */
rangeExpression
    : Ranges_rangeExpression
    ;


/*
 * ============================================================================
 * 5. LOGICAL OR
 * ============================================================================
 *
 * Logical precedence is owned by this composition root because logical
 * operators determine the ordering of the complete universal expression
 * hierarchy.
 *
 * This avoids importing competing logical/binary grammars.
 */
logicalOrExpression
    : logicalAndExpression
      (
          LOGICAL_OR
          logicalAndExpression
      )*
    ;


/*
 * ============================================================================
 * 6. LOGICAL AND
 * ============================================================================
 */
logicalAndExpression
    : nullCoalescingExpression
      (
          LOGICAL_AND
          nullCoalescingExpression
      )*
    ;


/*
 * ============================================================================
 * 7. NULL COALESCING
 * ============================================================================
 *
 * Nullability is semantic.
 *
 * This layer establishes only syntactic precedence.
 *
 * Canonical relationship:
 *
 *     logical AND
 *         |
 *         v
 *     null coalescing
 *         |
 *         v
 *     bitwise OR
 */
nullCoalescingExpression
    : bitwiseOrExpression
      (
          NULL_COALESCE
          bitwiseOrExpression
      )*
    ;


/*
 * ============================================================================
 * 8. BITWISE OR
 * ============================================================================
 */
bitwiseOrExpression
    : bitwiseXorExpression
      (
          PIPE
          bitwiseXorExpression
      )*
    ;


/*
 * ============================================================================
 * 9. BITWISE XOR
 * ============================================================================
 */
bitwiseXorExpression
    : bitwiseAndExpression
      (
          CARET
          bitwiseAndExpression
      )*
    ;


/*
 * ============================================================================
 * 10. BITWISE AND
 * ============================================================================
 */
bitwiseAndExpression
    : equalityExpression
      (
          AMPERSAND
          equalityExpression
      )*
    ;


/*
 * ============================================================================
 * 11. EQUALITY
 * ============================================================================
 */
equalityExpression
    : relationalExpression
      (
          equalityOperator
          relationalExpression
      )*
    ;

equalityOperator
    : EQUAL_EQUAL
    | NOT_EQUAL
    ;


/*
 * ============================================================================
 * 12. RELATIONAL
 * ============================================================================
 */
relationalExpression
    : shiftExpression
      (
          relationalOperator
          shiftExpression
      )*
    ;

relationalOperator
    : LESS
    | GREATER
    | LESS_EQUAL
    | GREATER_EQUAL
    ;


/*
 * ============================================================================
 * 13. SHIFT
 * ============================================================================
 */
shiftExpression
    : additiveExpression
      (
          shiftOperator
          additiveExpression
      )*
    ;

shiftOperator
    : LEFT_SHIFT
    | RIGHT_SHIFT
    ;


/*
 * ============================================================================
 * 14. ADDITIVE
 * ============================================================================
 */
additiveExpression
    : multiplicativeExpression
      (
          additiveOperator
          multiplicativeExpression
      )*
    ;

additiveOperator
    : PLUS
    | MINUS
    ;


/*
 * ============================================================================
 * 15. MULTIPLICATIVE
 * ============================================================================
 */
multiplicativeExpression
    : prefixExpression
      (
          multiplicativeOperator
          prefixExpression
      )*
    ;

multiplicativeOperator
    : STAR
    | SLASH
    | MODULO
    ;


/*
 * ============================================================================
 * 16. PREFIX / UNARY
 * ============================================================================
 *
 * Unary owns:
 *
 *     unaryExpression
 *     unaryOperator
 *
 * This composition root does not duplicate those rules.
 */
prefixExpression
    : unaryExpression
    ;


/*
 * ============================================================================
 * 17. POSTFIX
 * ============================================================================
 *
 * Postfix owns:
 *
 *     calls
 *     indexing
 *     member access
 *     optional member access
 *     qualified member access
 *     postfix updates
 *     postfix composition
 *
 * This is deliberately a single boundary.
 */
postfixExpression
    : postfixExpressionDelegate
    ;

postfixExpressionDelegate
    : Postfix_postfixExpression
    ;


/*
 * ============================================================================
 * 18. PRIMARY EXPRESSION
 * ============================================================================
 *
 * Primary expressions are the leaves/constructors of the precedence ladder.
 *
 * The primary layer is deliberately open-ended.
 *
 * Adding a future computational domain should normally add a specialized
 * expression component rather than changing the binary precedence hierarchy.
 *
 * This is the principal scalability mechanism of the expression subsystem.
 */
primaryExpression
    : literalExpression
    | identifierExpression
    | qualifiedNameExpression
    | parenthesizedExpression
    | tupleExpression
    | arrayExpression
    | mapExpression
    | comprehensionExpression
    | lambdaExpression
    | matchExpression
    | quantumExpression
    | reasoningExpression
    | knowledgeExpression
    | uncertaintyExpression
    | queryExpression
    | policyExpression
    | effectExpression
    | compileTimeExpression
    | metaprogrammingExpression
    | expressionBlock
    | newExpression
    | thisExpression
    | superExpression
    ;


/*
 * ============================================================================
 * 19. PARENTHESIZED EXPRESSION
 * ============================================================================
 *
 * Parentheses belong to the universal expression composition layer because
 * they explicitly override precedence.
 *
 * Semantic interpretation remains downstream.
 */
parenthesizedExpression
    : LPAREN expression RPAREN
    ;


/*
 * ============================================================================
 * 20. OBJECT / VALUE CONSTRUCTION
 * ============================================================================
 *
 * `new` is source-level construction syntax.
 *
 * It does not imply:
 *
 *     heap allocation
 *     CPU allocation
 *     device allocation
 *     physical memory selection
 *     hardware selection
 *
 * Those are semantic/runtime concerns.
 */
newExpression
    : NEW typeExpression
      (
          LPAREN
          argumentList?
          RPAREN
      )?
    ;


/*
 * ============================================================================
 * 21. THIS
 * ============================================================================
 */
thisExpression
    : THIS
    ;


/*
 * ============================================================================
 * 22. SUPER
 * ============================================================================
 */
superExpression
    : SUPER
    ;


/*
 * ============================================================================
 * 23. EXPRESSION LIST
 * ============================================================================
 *
 * There is intentionally no fixed cardinality.
 *
 * The implementation is resource-bounded, not language-bounded.
 */
expressionList
    : expression
      (
          COMMA
          expression
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 24. TYPE EXPRESSION LIST
 * ============================================================================
 *
 * The type-expression implementation belongs to the type-system grammar.
 *
 * This rule only composes it for expression contexts that need a list.
 */
typeExpressionList
    : typeExpression
      (
          COMMA
          typeExpression
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 25. ARGUMENT LIST
 * ============================================================================
 *
 * The canonical call/postfix implementation owns specialized argument forms.
 *
 * The expression composition layer intentionally does not redefine:
 *
 *     argumentList
 *     namedArgument
 *     spreadArgument
 *     generic call arguments
 *
 * Those belong to Postfix.
 *
 * This compatibility rule exists only for expression constructs whose
 * specification explicitly requires a simple expression argument list.
 */
expressionArgumentList
    : expression
      (
          COMMA
          expression
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 26. PRECEDENCE MODEL
 * ============================================================================
 *
 * Lowest
 * -------
 *
 *     assignment
 *
 *     conditional
 *
 *     range
 *
 *     logical OR
 *
 *     logical AND
 *
 *     null coalescing
 *
 *     bitwise OR
 *
 *     bitwise XOR
 *
 *     bitwise AND
 *
 *     equality
 *
 *     relational
 *
 *     shift
 *
 *     additive
 *
 *     multiplicative
 *
 *     prefix/unary
 *
 *     postfix
 *
 *     primary
 *
 * Highest
 *
 * ============================================================================
 * ASSOCIATIVITY
 * ============================================================================
 *
 * Assignment:
 *
 *     right associative
 *
 * Logical/binary chains:
 *
 *     left associative
 *
 * Prefix/unary:
 *
 *     recursively right associative
 *
 * Postfix:
 *
 *     left-to-right chaining
 *
 * Conditional:
 *
 *     defined by ConditionalsParser
 *
 * Range:
 *
 *     defined by Ranges
 *
 * ============================================================================
 * OPEN-WORLD COMPUTATION
 * ============================================================================
 *
 * The expression grammar intentionally does not require a new core grammar
 * rule for every computational capability.
 *
 * The following can remain ordinary identifiers, calls, or qualified calls:
 *
 *     infer(...)
 *     deduce(...)
 *     reason(...)
 *     assert(...)
 *     retract(...)
 *     query(...)
 *     learn(...)
 *     adapt(...)
 *     explain(...)
 *     simulate(...)
 *     measure(...)
 *     observe(...)
 *     decide(...)
 *     plan(...)
 *     validate(...)
 *     optimize(...)
 *
 * Their semantic meaning may establish:
 *
 *     effects
 *     capabilities
 *     resources
 *     contracts
 *     policies
 *     provenance
 *     evidence
 *     uncertainty
 *     determinism
 *
 * The grammar therefore remains extensible without a keyword explosion.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * QuantumExpressions owns quantum-specific source syntax.
 *
 * This root does NOT enumerate:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     CNOT
 *     CX
 *     CZ
 *     SWAP
 *     RX
 *     RY
 *     RZ
 *
 * as universal grammar alternatives.
 *
 * Such operation names remain data-driven names or dialect-defined operation
 * specifications.
 *
 * The semantic pipeline is:
 *
 *     quantumExpression
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     quantum semantic analysis
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
 * This grammar never selects physical qubits or a physical QPU.
 *
 * ============================================================================
 * CLASSICAL / HDL / HYBRID INTEGRATION
 * ============================================================================
 *
 * Classical expressions use the same universal hierarchy.
 *
 * HDL/hardware expressions may enter through:
 *
 *     identifiers
 *     calls
 *     member access
 *     indexing
 *     literals
 *     blocks
 *     domain-specific expressions
 *
 * Hybrid programs use the same expression root for transitions between:
 *
 *     classical computation
 *     quantum computation
 *     accelerator computation
 *     hardware intent
 *     data processing
 *     AI/ML
 *
 * No expression-level machine selection is permitted.
 *
 * ============================================================================
 * REASONING / KNOWLEDGE / LEARNING / ADAPTATION
 * ============================================================================
 *
 * These capabilities integrate through the expression layer without becoming
 * a separate programming language.
 *
 * Reasoning:
 *
 *     reasoningExpression
 *
 * Knowledge:
 *
 *     knowledgeExpression
 *
 * Uncertainty:
 *
 *     uncertaintyExpression
 *
 * Learning/adaptation:
 *
 *     ordinary callable expressions and/or their dedicated semantic
 *     expression components when syntax requires them
 *
 * The semantic layer attaches:
 *
 *     effect
 *     capability
 *     resource
 *     policy
 *     provenance
 *     evidence
 *     contract
 *
 * information.
 *
 * Adaptation is NOT equivalent to unrestricted source self-modification.
 *
 * Authorization, policy and provenance remain downstream semantic concerns.
 *
 * ============================================================================
 * PATTERN / GUARD INTEGRATION
 * ============================================================================
 *
 * Match expressions are owned by MatchExpressions.
 *
 * Pattern syntax is owned by Patterns.
 *
 * Guard syntax is owned by Guards.
 *
 * The expression orchestrator only places match expressions in primary
 * position.
 *
 * This prevents the pattern language from creating a second expression
 * precedence hierarchy.
 *
 * ============================================================================
 * POLICY / EFFECT / RESOURCE INTEGRATION
 * ============================================================================
 *
 * Expressions may appear inside:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *     capability conditions
 *     resource constraints
 *     policy conditions
 *     security predicates
 *     provenance predicates
 *
 * The expression grammar therefore remains deliberately general.
 *
 * Resource availability is NEVER determined by parsing.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing MUST depend only on:
 *
 *     source token sequence
 *     selected grammar/language version
 *     explicit dialect configuration
 *
 * Parsing MUST NOT inspect:
 *
 *     hardware
 *     memory availability
 *     CPU count
 *     GPU count
 *     QPU availability
 *     filesystem state
 *     network state
 *     wall-clock time
 *     randomness
 *     scheduler state
 *     runtime state
 *
 * Therefore identical source and grammar configuration produce the same
 * parse structure.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no embedded Rust actions
 *     no semantic predicates
 *     no filesystem access
 *     no network access
 *     no process execution
 *     no hardware discovery
 *     no plugin execution
 *     no runtime execution
 *     no unsafe Rust
 *
 * It is purely declarative syntax.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * This grammar establishes NO artificial universal finite ceiling for:
 *
 *     expression count
 *     expression nesting
 *     operand count
 *     argument count
 *     tuple arity
 *     array size
 *     map size
 *     comprehension size
 *     member-chain depth
 *     call-chain depth
 *     index dimensions
 *     generic argument count
 *     tensor rank
 *     matrix dimensions
 *     quantum operation count
 *     qubit count
 *     processor count
 *     GPU count
 *     FPGA count
 *     accelerator count
 *     node count
 *     device count
 *     memory capacity
 *     register width
 *     network size
 *
 * No machine-capacity constant belongs in this grammar.
 *
 * "Infinity" means:
 *
 *     no artificial finite language-level ceiling.
 *
 * It does NOT promise physically infinite hardware or infinite compilation
 * resources.
 *
 * Actual limits are implementation/resource constraints and must be reported
 * explicitly rather than encoded as language semantics.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * Expressions MUST NOT directly encode:
 *
 *     physical processor identifiers
 *     physical GPU identifiers
 *     physical FPGA identifiers
 *     ASIC identifiers
 *     physical qubit identifiers
 *     fixed hardware topology
 *     fixed register width
 *     fixed memory capacity
 *     fixed node count
 *     fixed device count
 *
 * Those concerns belong to:
 *
 *     resources
 *     capabilities
 *     compilation context
 *     target descriptions
 *     negotiation
 *     placement
 *     routing
 *     scheduling
 *     deployment
 *     HAL
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parse tree must preserve sufficient structure for a domain-neutral AST
 * to represent at least:
 *
 *     literal
 *     identifier
 *     qualified name
 *     unary
 *     binary
 *     assignment
 *     conditional
 *     range
 *     call
 *     indexing
 *     member access
 *     optional member access
 *     tuple
 *     array
 *     map
 *     comprehension
 *     lambda
 *     closure
 *     match
 *     construction
 *     block expression
 *     quantum expression
 *     reasoning expression
 *     knowledge expression
 *     uncertainty expression
 *     query expression
 *     policy expression
 *     effect expression
 *     compile-time expression
 *     metaprogramming expression
 *
 * This file does not define Rust AST structures.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * This grammar does NOT determine:
 *
 *     type
 *     overload resolution
 *     conversion
 *     ownership
 *     borrowing
 *     lifetime
 *     effect validity
 *     capability availability
 *     resource sufficiency
 *     policy authorization
 *     contract validity
 *     provenance validity
 *     quantum legality
 *     hardware feasibility
 *     distributed placement
 *     execution strategy
 *
 * Those are downstream semantic responsibilities.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * Expressions lower through:
 *
 *     parse tree
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     semantic model
 *          |
 *          +---------------------+
 *          |                     |
 *          v                     v
 *     classical IR          quantum::ir
 *          |                     |
 *          +----------+----------+
 *                     |
 *                     v
 *              target-independent
 *                optimization
 *                     |
 *                  lowering
 *                     |
 *             routing/scheduling
 *                     |
 *                 resilience
 *                     |
 *                    HAL
 *
 * The expression grammar MUST NEVER emit IR directly.
 *
 * ============================================================================
 * SOURCE PRESERVATION
 * ============================================================================
 *
 * Expression parsing must preserve sufficient source information for:
 *
 *     diagnostics
 *     source spans
 *     source maps
 *     formatting
 *     IDE/LSP
 *     refactoring
 *     semantic diagnostics
 *     provenance
 *     reproducible compilation
 *     incremental compilation
 *     compatibility analysis
 *
 * ============================================================================
 * ERROR BOUNDARY
 * ============================================================================
 *
 * Parser errors are structural errors.
 *
 * They MUST NOT be converted into semantic success.
 *
 * Examples that belong downstream:
 *
 *     invalid assignment target
 *     incompatible operand types
 *     unavailable capability
 *     insufficient resources
 *     forbidden policy
 *     invalid contract
 *     invalid quantum operation
 *     invalid hardware mapping
 *     illegal effect
 *     invalid adaptation
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * This file uses canonical lexer token names.
 *
 * It MUST NOT introduce historical aliases such as:
 *
 *     EQ_EQ
 *     NOT_EQ
 *     LE
 *     GE
 *     SHIFT_LEFT
 *     SHIFT_RIGHT
 *     AND_AND
 *     OR_OR
 *     PERCENT
 *     PLUS_PLUS
 *     MINUS_MINUS
 *
 * The lexer remains the single authority for token spelling.
 *
 * ============================================================================
 * LEGACY FILE POLICY
 * ============================================================================
 *
 * Existing specialized files that duplicate expression precedence MUST NOT be
 * imported by this root.
 *
 * In particular:
 *
 *     arithmetic.g4
 *     binary.g4
 *     bitwise.g4
 *     comparison.g4
 *     logical.g4
 *     shift.g4
 *
 * are not expression-authority imports.
 *
 * They may be:
 *
 *     compatibility surfaces
 *     migration surfaces
 *     historical specifications
 *     domain-specific helpers
 *
 * but the canonical parser must continue to have one precedence hierarchy.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * UPSTREAM
 * --------
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     canonical token vocabulary
 *
 *     grammar/core/*
 *     names and universal syntax
 *
 *     grammar/types/*
 *     type expressions
 *
 * DOWNSTREAM
 * ----------
 *
 *     grammar/antlr/ZamaniParser.g4
 *     canonical parser composition
 *
 *     grammar/statements/*
 *     grammar/declarations/*
 *     grammar/functions/*
 *     grammar/modules/*
 *     grammar/classical/*
 *     grammar/quantum/*
 *     grammar/hybrid/*
 *     grammar/hdl/*
 *     grammar/hardware/*
 *     grammar/ai/*
 *     grammar/data/*
 *     grammar/distributed/*
 *     grammar/networking/*
 *     grammar/security/*
 *     grammar/resources/*
 *     grammar/validation/*
 *     grammar/policies/*
 *     grammar/execution/*
 *     grammar/interoperability/*
 *     grammar/metaprogramming/*
 *     grammar/compile/*
 *
 * SEMANTIC CONSUMERS
 * ------------------
 *
 *     frontend AST
 *     structural validator
 *     type checker
 *     effect checker
 *     capability resolver
 *     resource analyzer
 *     contract analyzer
 *     policy analyzer
 *     provenance system
 *     canonical semantic model
 *
 * IR CONSUMERS
 * ------------
 *
 *     classical IR
 *     quantum::ir
 *     HDL/hardware semantic representation
 *
 * ============================================================================
 * FILE COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when:
 *
 *     [ ] exactly one public `expression` rule exists;
 *     [ ] specialized components are imported rather than reimplemented;
 *     [ ] no competing expression hierarchy is imported;
 *     [ ] assignment is owned by AssignmentExpressions;
 *     [ ] conditional is owned by ConditionalsParser;
 *     [ ] ranges are owned by Ranges;
 *     [ ] unary syntax is owned by Unary;
 *     [ ] postfix syntax is owned by Postfix;
 *     [ ] literals are owned by Literals;
 *     [ ] collections are owned by their canonical collection grammars;
 *     [ ] lambdas are owned by Lambdas;
 *     [ ] patterns/guards/match are delegated;
 *     [ ] quantum syntax is delegated to QuantumExpressions;
 *     [ ] reasoning/knowledge/uncertainty are composable;
 *     [ ] query/policy/effect expressions are composable;
 *     [ ] compile-time/metaprogramming expressions are composable;
 *     [ ] no fixed machine capacity is encoded;
 *     [ ] no fixed quantum gate set is encoded;
 *     [ ] no physical hardware is encoded;
 *     [ ] no IR is created;
 *     [ ] no runtime action exists;
 *     [ ] no unsafe Rust is required;
 *     [ ] deterministic parsing is preserved;
 *     [ ] source structure is preserved;
 *     [ ] expression lists are resource-bounded but language-unbounded;
 *     [ ] target selection remains downstream;
 *     [ ] capability negotiation remains downstream;
 *     [ ] resource negotiation remains downstream;
 *     [ ] routing remains downstream;
 *     [ ] scheduling remains downstream;
 *     [ ] resilience remains downstream;
 *     [ ] positive tests exist;
 *     [ ] negative tests exist;
 *     [ ] precedence tests exist;
 *     [ ] associativity tests exist;
 *     [ ] cross-domain tests exist;
 *     [ ] scalability tests exist;
 *     [ ] compatibility tests exist.
 *
 * ============================================================================
 * REQUIRED REPOSITORY INTEGRATION
 * ============================================================================
 *
 * This file is intentionally the expression ORCHESTRATOR.
 *
 * Therefore the following repository invariants must be enforced:
 *
 * 1. ZamaniParser.g4 imports Expressions.
 *
 * 2. No parent parser defines another `expression` rule.
 *
 * 3. No specialized expression grammar imports Expressions back.
 *
 * 4. Specialized expression grammars do not define competing precedence
 *    ladders.
 *
 * 5. `Postfix` remains the owner of call/index/member postfix syntax.
 *
 * 6. `QuantumExpressions` remains the owner of quantum-specific expression
 *    syntax.
 *
 * 7. `Literals` remains the owner of literal syntax.
 *
 * 8. `ExpressionIdentifiers` remains the owner of expression-level names.
 *
 * 9. Type syntax remains owned by the type subsystem.
 *
 * 10. Semantic capabilities such as reasoning, learning, adaptation,
 *     uncertainty, evidence and provenance do not become machine-specific
 *     expression syntax.
 *
 * 11. The quantum semantic boundary remains:
 *
 *         frontend AST -> semantic model -> quantum::ir
 *
 * 12. Rust integration remains safe Rust compatible with:
 *
 *         Rust 1.97
 *         Rust 1.97.1
 *
 *     and uses no unsafe code.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL GUARANTEE
 * ============================================================================
 *
 * The expression subsystem describes COMPUTATION, not MACHINE REALIZATION.
 *
 * Therefore the same expression source can participate in compilation for:
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
 *     future computational substrates
 *
 * without changing the expression grammar merely because the available
 * machine becomes larger, smaller, different, heterogeneous, distributed,
 * quantum, or otherwise novel.
 *
 * Resource feasibility is a property of the compilation/execution environment,
 * not a hidden grammar constant.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */
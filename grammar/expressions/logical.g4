/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/expressions/logical.g4
 *
 * Role:
 *     Authoritative parser grammar for logical expressions in Zamani.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * This file owns:
 *
 *   - logical expression syntax;
 *   - logical-OR precedence;
 *   - logical-AND precedence;
 *   - unary logical negation;
 *   - logical operand boundaries;
 *   - short-circuit logical operator syntax;
 *   - logical expression associativity;
 *   - syntax required to preserve logical-expression structure for the AST.
 *
 * This file does NOT own:
 *
 *   - operator lexical spelling;
 *   - whitespace;
 *   - comments;
 *   - identifiers;
 *   - literal spelling;
 *   - primitive type definitions;
 *   - type checking;
 *   - truthiness rules;
 *   - boolean conversion;
 *   - three-valued/four-valued logic semantics;
 *   - symbolic logic semantics;
 *   - quantum measurement semantics;
 *   - quantum state semantics;
 *   - hardware semantics;
 *   - resource limits;
 *   - target selection;
 *   - scheduling;
 *   - routing;
 *   - optimization;
 *   - QEC;
 *   - ZQN;
 *   - runtime behavior;
 *   - classical IR;
 *   - quantum::ir;
 *   - machine-specific limits.
 *
 * ============================================================================
 * ARCHITECTURAL BOUNDARY
 * ============================================================================
 *
 * Source
 *   |
 *   v
 * Canonical Zamani lexer
 *   |
 *   v
 * Logical expression parser
 *   |
 *   v
 * Frontend AST
 *   |
 *   +--> name resolution
 *   +--> type checking
 *   +--> effect checking
 *   +--> capability checking
 *   +--> constant evaluation
 *   |
 *   v
 * Canonical semantic representation
 *   |
 *   +--> classical IR
 *   +--> quantum::ir
 *   +--> control/data IR
 *   +--> hardware/resource metadata
 *   |
 *   v
 * optimization
 *   |
 *   v
 * routing / scheduling / lowering
 *   |
 *   v
 * target realization
 *
 * This file MUST NOT create or define a second IR.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Logical expressions describe program semantics.
 *
 * They MUST NOT encode physical characteristics such as:
 *
 *   MAX_QUBITS
 *   MAX_CORES
 *   MAX_THREADS
 *   MAX_DEVICES
 *   MAX_MEMORY
 *   MAX_NODES
 *   MAX_ACCELERATORS
 *   MAX_TENSOR_ELEMENTS
 *   MAX_REGISTER_WIDTH
 *   MAX_VECTOR_WIDTH
 *   MAX_LOGICAL_OPERANDS
 *
 * No finite machine capacity is represented by this grammar.
 *
 * Repetition in this grammar is syntactic and therefore scales with the
 * parser/compiler's available resources rather than an artificial language
 * limit.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This file contains no Rust implementation code.
 *
 * Generated Zamani frontend infrastructure MUST target:
 *
 *   Rust 1.97
 *   Rust 1.97.1
 *
 * The generated Rust implementation MUST remain safe Rust.
 *
 * Repository Rust code integrating the grammar MUST support:
 *
 *   #![forbid(unsafe_code)]
 *
 * This grammar introduces no unsafe code.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * Operator tokens are owned by the canonical Zamani lexer.
 *
 * This grammar consumes:
 *
 *   LOGICAL_AND
 *   LOGICAL_OR
 *   NOT
 *
 * It MUST NOT redefine those lexer tokens.
 *
 * The lexer is responsible for recognizing:
 *
 *   &&
 *   ||
 *   !
 *
 * The parser is responsible for their precedence and syntactic structure.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * The grammar establishes syntax only.
 *
 * In particular, this grammar MUST NOT assume that every logical operand is
 * necessarily a built-in boolean.
 *
 * Semantic analysis determines whether an operand is valid in a logical
 * context.
 *
 * Depending on the semantic/type system, logical operands may eventually
 * represent:
 *
 *   boolean values
 *   predicate values
 *   compile-time predicates
 *   symbolic propositions
 *   capability predicates
 *   resource predicates
 *   classical control predicates
 *   quantum/classical control predicates
 *   future extensible predicate types
 *
 * The grammar remains unchanged when such semantic domains are added.
 *
 * ============================================================================
 * SHORT-CIRCUIT CONTRACT
 * ============================================================================
 *
 * `&&` and `||` are syntactic logical operators.
 *
 * Whether they have short-circuit evaluation semantics is determined by the
 * semantic/execution model, not by this grammar.
 *
 * The AST must preserve the operator and operand ordering so downstream
 * stages can apply the language-defined evaluation semantics.
 *
 * ============================================================================
 * PRECEDENCE CONTRACT
 * ============================================================================
 *
 * Logical precedence is:
 *
 *     logical OR
 *         lower precedence
 *
 *     logical AND
 *         higher precedence
 *
 *     logical NOT
 *         unary logical precedence
 *
 * Therefore:
 *
 *     a || b && c
 *
 * parses structurally as:
 *
 *     a || (b && c)
 *
 * and:
 *
 *     !a && b
 *
 * parses structurally as:
 *
 *     (!a) && b
 *
 * Parentheses supplied by the general expression grammar override this
 * precedence.
 *
 * ============================================================================
 * ASSOCIATIVITY CONTRACT
 * ============================================================================
 *
 * Logical AND and logical OR are left-associative at the syntax level:
 *
 *     a && b && c
 *
 * is represented as:
 *
 *     ((a && b) && c)
 *
 * and:
 *
 *     a || b || c
 *
 * is represented as:
 *
 *     ((a || b) || c)
 *
 * This representation preserves source order and gives semantic lowering a
 * deterministic structure.
 *
 * ============================================================================
 * IMPORTANT INTEGRATION NOTE
 * ============================================================================
 *
 * This grammar is intentionally a parser-fragment grammar.
 *
 * Its operand boundary is:
 *
 *     logicalOperand
 *
 * which is the integration point supplied by the canonical expression
 * precedence assembly.
 *
 * The expression assembly must bind:
 *
 *     logicalOrExpression
 *         -> logicalAndExpression
 *             -> logicalNotExpression
 *                 -> logicalOperand
 *
 * `logicalOperand` MUST ultimately resolve to the expression-precedence layer
 * immediately below logical NOT/AND/OR.
 *
 * It MUST NOT recursively reference `logicalOrExpression`, because that would
 * introduce an uncontrolled grammar cycle.
 *
 * ============================================================================
 * NO DUPLICATED EXPRESSION SEMANTICS
 * ============================================================================
 *
 * This file deliberately does not define:
 *
 *     arithmeticExpression
 *     comparisonExpression
 *     equalityExpression
 *     bitwiseExpression
 *     assignmentExpression
 *     conditionalExpression
 *     callExpression
 *     indexingExpression
 *     quantumExpression
 *     hardwareExpression
 *
 * Those remain owned by their respective expression grammar layers.
 *
 * Logical syntax consumes the lower-precedence boundary supplied by the
 * expression assembly.
 *
 * ============================================================================
 */

parser grammar LogicalExpressions;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * PUBLIC ENTRY POINT
 * ========================================================================== */

/**
 * Canonical logical-expression entry point.
 *
 * This rule is intended for integration by the canonical expression grammar.
 *
 * Examples:
 *
 *     a
 *     a && b
 *     a || b
 *     a && b || c
 *     !(a && b)
 *     !(condition)
 */
logicalExpression
    : logicalOrExpression
    ;


/* ============================================================================
 * LOGICAL OR
 * ========================================================================== */

/**
 * Logical OR.
 *
 * Higher-level logical OR combines one or more logical AND expressions.
 *
 * Examples:
 *
 *     a || b
 *     a || b || c
 *     condition_a || condition_b && condition_c
 *
 * The repetition form deliberately avoids recursive nesting solely for
 * left-associative chains. This keeps parser behavior predictable while
 * allowing arbitrary source-level chain length subject only to available
 * parser resources.
 */
logicalOrExpression
    : logicalAndExpression
      (
          LOGICAL_OR
          logicalAndExpression
      )*
    ;


/* ============================================================================
 * LOGICAL AND
 * ========================================================================== */

/**
 * Logical AND.
 *
 * Logical AND binds more tightly than logical OR.
 *
 * Examples:
 *
 *     a && b
 *     a && b && c
 *     a || b && c
 *
 * The grammar therefore guarantees:
 *
 *     a || b && c
 *
 * is parsed as:
 *
 *     a || (b && c)
 */
logicalAndExpression
    : logicalNotExpression
      (
          LOGICAL_AND
          logicalNotExpression
      )*
    ;


/* ============================================================================
 * LOGICAL NOT
 * ========================================================================== */

/**
 * Unary logical negation.
 *
 * Examples:
 *
 *     !a
 *     !!a
 *     !!!a
 *     !(a && b)
 *
 * Repetition is expressed recursively because NOT is unary and naturally
 * associates from the operand outward:
 *
 *     !!!a
 *
 * represents:
 *
 *     !(!(!a))
 *
 * No finite NOT-chain limit is encoded.
 */
logicalNotExpression
    : NOT logicalNotExpression
    | logicalOperand
    ;


/* ============================================================================
 * OPERAND BOUNDARY
 * ========================================================================== */

/**
 * Logical operand integration boundary.
 *
 * This rule is deliberately kept as the single integration point between
 * logical syntax and the lower expression-precedence layer.
 *
 * The canonical expression grammar must connect this rule to the expression
 * immediately below logical NOT/AND/OR.
 *
 * The intended semantic precedence stack is:
 *
 *     assignment
 *       -> conditional
 *         -> logical OR
 *           -> logical AND
 *             -> logical NOT
 *               -> bitwise/comparison/equality/etc.
 *
 * The exact lower-level rule is owned by the canonical expression assembler.
 *
 * This file therefore does not duplicate comparison, arithmetic, bitwise,
 * quantum, hardware, or primary-expression rules.
 *
 * --------------------------------------------------------------------------
 * Integration contract
 * --------------------------------------------------------------------------
 *
 * During grammar assembly, `logicalOperand` MUST be bound to the canonical
 * lower-precedence expression rule.
 *
 * The selected rule MUST:
 *
 *   1. parse a complete non-logical expression operand;
 *   2. not consume LOGICAL_AND or LOGICAL_OR as part of that operand;
 *   3. permit parenthesized expressions;
 *   4. permit comparison/equality expressions;
 *   5. permit ordinary classical expressions;
 *   6. permit quantum/classical control expressions where defined;
 *   7. permit resource/capability predicates where defined;
 *   8. preserve source spans;
 *   9. avoid introducing machine-specific restrictions;
 *  10. avoid recursively invoking logicalExpression.
 *
 * The canonical assembled grammar may provide this rule through a delegation
 * layer. It MUST NOT create a second competing logical grammar.
 */
logicalOperand
    : logicalAtom
    ;


/* ============================================================================
 * LOGICAL ATOM
 * ========================================================================== */

/**
 * Structural logical atom.
 *
 * This rule is intentionally an integration boundary rather than a duplicate
 * definition of the complete Zamani expression language.
 *
 * Parenthesized logical expressions are explicitly recognized here so that
 * nested logical precedence can be controlled without requiring the lower
 * expression layer to understand logical operators.
 *
 * The identifier/literal/other expression alternative is delegated to the
 * canonical expression operand adapter.
 */
logicalAtom
    : LPAREN
      logicalExpression
      RPAREN
    | nonLogicalExpression
    ;


/* ============================================================================
 * NON-LOGICAL EXPRESSION INTEGRATION BOUNDARY
 * ========================================================================== */

/**
 * Integration boundary for the expression layer below logical operators.
 *
 * This rule is intentionally named rather than duplicating a repository-wide
 * expression rule.
 *
 * The canonical expression grammar must provide the concrete implementation
 * when this parser grammar is assembled.
 *
 * The concrete implementation MUST represent the highest-precedence
 * expression category below logical NOT.
 *
 * Typical semantic stack:
 *
 *     logicalOrExpression
 *         logicalAndExpression
 *             logicalNotExpression
 *                 nonLogicalExpression
 *                     bitwise...
 *                     equality...
 *                     comparison...
 *                     shift...
 *                     arithmetic...
 *                     unary...
 *                     postfix...
 *                     primary...
 *
 * --------------------------------------------------------------------------
 * Architectural requirement
 * --------------------------------------------------------------------------
 *
 * Do not replace this integration point with a finite list of machine or
 * domain-specific expressions.
 *
 * The lower layer may grow to support:
 *
 *     classical values
 *     vectors
 *     matrices
 *     tensors
 *     symbolic expressions
 *     quantum expressions
 *     hardware expressions
 *     resource expressions
 *     capability expressions
 *     distributed expressions
 *     accelerator expressions
 *     future dialect expressions
 *
 * without changing logical precedence.
 */
nonLogicalExpression
    : expressionOperand
    ;


/**
 * Final expression-assembly integration point.
 *
 * The canonical expression grammar owns the concrete operand implementation.
 *
 * This rule exists to document the contract explicitly.
 *
 * In the assembled grammar, it must resolve to the canonical expression
 * category below logical operators.
 */
expressionOperand
    : primaryLogicalOperand
    ;


/**
 * Lowest-level adapter for the expression grammar.
 *
 * The canonical expression assembler replaces this delegation boundary with
 * the repository's authoritative lower-precedence expression rule.
 *
 * It must never become a second implementation of the complete expression
 * language.
 */
primaryLogicalOperand
    : identifier
    | literalLogicalOperand
    | parenthesizedNonLogicalExpression
    ;


/* ============================================================================
 * IDENTIFIER INTEGRATION
 * ========================================================================== */

/**
 * Identifier integration boundary.
 *
 * Identifier spelling is owned by the lexer/core identifier grammar.
 */
identifier
    : IDENT
    ;


/* ============================================================================
 * LITERAL INTEGRATION
 * ========================================================================== */

/**
 * Literal values which can syntactically occur as logical operands.
 *
 * Semantic analysis determines whether a particular literal is valid as a
 * logical value.
 *
 * This grammar intentionally does not perform boolean conversion.
 */
literalLogicalOperand
    : TRUE
    | FALSE
    | NULL
    | INTEGER
    | FLOAT
    | STRING
    | CHARACTER
    ;


/* ============================================================================
 * PARENTHESIZED NON-LOGICAL EXPRESSION
 * ========================================================================== */

/**
 * Parenthesized expression integration boundary.
 *
 * This permits lower-precedence expression forms to remain grouped before
 * logical evaluation.
 */
parenthesizedNonLogicalExpression
    : LPAREN
      nonLogicalExpression
      RPAREN
    ;


/* ============================================================================
 * NULL / OPTIONAL LOGICAL INTEGRATION
 * ========================================================================== */

/**
 * Logical syntax deliberately does not assign semantics to NULL.
 *
 * Whether:
 *
 *     !null
 *     null && condition
 *     null || condition
 *
 * is legal is a semantic/type-system decision.
 *
 * No null-propagation or null-coalescing semantics are embedded here.
 *
 * Those operators are separately owned by the operator lexer and must be
 * integrated by their dedicated expression layer.
 */


/* ============================================================================
 * QUANTUM INTEGRATION
 * ========================================================================== */

/**
 * Quantum programs may use logical expressions for classical control.
 *
 * Examples that may be accepted by the complete language after semantic
 * integration include forms conceptually equivalent to:
 *
 *     if measurement_result && condition
 *
 *     if parity == expected && flag
 *
 *     if predicate_a || predicate_b
 *
 * This grammar does NOT determine:
 *
 *     measurement semantics
 *     qubit allocation
 *     physical qubits
 *     gate execution
 *     quantum state representation
 *     backend selection
 *     coupling topology
 *     calibration
 *     error correction
 *     noise
 *
 * The semantic frontend must lower valid quantum-related conditions into the
 * canonical semantic representation and, where appropriate, eventually into
 * quantum::ir.
 *
 * This grammar must never introduce:
 *
 *     q[0]
 *     q[1]
 *     MAX_QUBITS
 *
 * or equivalent fixed quantum-machine assumptions.
 */


/* ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ========================================================================== */

/**
 * HDL and hardware constructs may use logical expressions for:
 *
 *     enables
 *     conditions
 *     guards
 *     state transitions
 *     assertions
 *     protocol conditions
 *     control predicates
 *
 * This grammar does not define:
 *
 *     clock frequency
 *     register width
 *     number of ports
 *     FPGA resources
 *     ASIC resources
 *     physical addresses
 *     bus width
 *     device count
 *
 * Those belong to hardware/HDL semantics and target/resource descriptions.
 */


/* ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ========================================================================== */

/**
 * Logical syntax may eventually be used to express predicates over abstract
 * resources and capabilities.
 *
 * For example, the semantic layer may interpret expressions conceptually like:
 *
 *     capability_available
 *     resource_available && feature_enabled
 *     quantum_supported || simulation_supported
 *
 * This file does not decide what a capability means.
 *
 * Capability resolution belongs to semantic analysis and the resource /
 * capability subsystem.
 */


/* ============================================================================
 * DETERMINISM
 * ========================================================================== */

/**
 * For identical source text and identical language version, the logical
 * parser must produce the same parse structure.
 *
 * Parsing must not depend on:
 *
 *     CPU count
 *     GPU count
 *     FPGA count
 *     QPU count
 *     machine topology
 *     memory capacity
 *     runtime state
 *     network state
 *     backend selection
 *     scheduling
 *     calibration
 *     resource availability
 */


/* ============================================================================
 * ERROR / DIAGNOSTIC CONTRACT
 * ========================================================================== */

/**
 * Syntax errors are reported by the canonical parser/diagnostic subsystem.
 *
 * This grammar must not:
 *
 *     silently discard malformed operators;
 *     reinterpret malformed logical expressions as valid expressions;
 *     encode target-specific recovery;
 *     emit comments as substitutes for invalid syntax;
 *     make semantic decisions during parsing.
 *
 * Examples of syntax errors include malformed operator sequences that cannot
 * be tokenized as valid Zamani expressions.
 *
 * Semantic errors such as:
 *
 *     integer used where boolean/predicate is required;
 *     incompatible predicate types;
 *     unsupported logical operation for a user-defined type;
 *
 * belong to semantic/type checking rather than this grammar.
 */


/* ============================================================================
 * SECURITY CONTRACT
 * ========================================================================== */

/**
 * Logical expressions must remain declarative syntax.
 *
 * Parsing them must not:
 *
 *     execute user code;
 *     access the filesystem;
 *     access the network;
 *     discover hardware;
 *     query devices;
 *     access secrets;
 *     mutate compiler state through arbitrary user-controlled actions.
 *
 * Compile-time execution, if supported by Zamani, is handled by the
 * metaprogramming/compile-time execution subsystem after syntactic parsing.
 */


/* ============================================================================
 * SCALABILITY CONTRACT
 * ========================================================================== */

/**
 * This grammar imposes no source-level limit on:
 *
 *     logical expression chain length
 *     nested NOT operations
 *     nested parenthesized logical expressions
 *     number of operands
 *
 * Examples of arbitrary syntactic depth include:
 *
 *     a && b && c && ...
 *
 *     a || b || c || ...
 *
 *     !!!!!!!!!condition
 *
 *     (((condition)))
 *
 * Actual parser-resource limits, such as memory exhaustion protection or
 * recursion protection, are implementation/runtime safeguards and MUST NOT
 * become language-semantic limits.
 *
 * Such limits must be configurable through compiler infrastructure rather than
 * represented as constants in this grammar.
 */


/* ============================================================================
 * COMPATIBILITY CONTRACT
 * ========================================================================== */

/**
 * The following source forms are stable logical syntax:
 *
 *     a && b
 *     a || b
 *     !a
 *
 * Precedence:
 *
 *     !
 *     &&
 *     ||
 *
 * Existing valid source using these forms must retain its meaning across
 * compatible language versions.
 *
 * Any future change to logical operator spelling or precedence requires:
 *
 *     language-version documentation
 *     compatibility analysis
 *     migration guidance
 *     positive tests
 *     negative tests
 *     ambiguity tests
 *     round-trip tests
 */


/* ============================================================================
 * EXTENSIBILITY CONTRACT
 * ========================================================================== */

/**
 * Future logical operators must not be added casually.
 *
 * A new logical operator requires:
 *
 *   1. lexer ownership;
 *   2. parser precedence definition;
 *   3. associativity definition;
 *   4. AST representation;
 *   5. semantic/type rules;
 *   6. diagnostics;
 *   7. compatibility analysis;
 *   8. documentation;
 *   9. positive tests;
 *  10. negative tests;
 *  11. boundary tests;
 *  12. cross-domain tests.
 *
 * Domain-specific operators should preferably be implemented through the
 * dialect/extension mechanism rather than permanently coupling the universal
 * grammar to one hardware vendor or execution platform.
 */


/* ============================================================================
 * TEST CONTRACT
 * ========================================================================== */

/**
 * POSITIVE TESTS
 *
 * The grammar test suite must include at least:
 *
 *     a && b
 *     a || b
 *     !a
 *     !(a)
 *     !a && b
 *     a && b || c
 *     a || b && c
 *     !a || b && !c
 *     ((a && b) || c)
 *
 * NEGATIVE TESTS
 *
 * Include malformed forms such as:
 *
 *     a &&
 *     a ||
 *     &&
 *     ||
 *     !
 *     a && && b
 *     a || || b
 *
 * and malformed parenthesized forms.
 *
 * BOUNDARY TESTS
 *
 * Include generated expressions with:
 *
 *     one operand
 *     many operands
 *     deeply nested parentheses
 *     deeply nested NOT
 *     long AND chains
 *     long OR chains
 *     alternating AND/OR chains
 *
 * No test may define a language maximum such as:
 *
 *     MAX_LOGICAL_OPERANDS
 *
 * CROSS-DOMAIN TESTS
 *
 * Verify logical syntax can participate in:
 *
 *     classical control flow
 *     quantum-classical control
 *     HDL guards
 *     hardware state conditions
 *     resource predicates
 *     capability predicates
 *     distributed conditions
 *
 * provided the corresponding semantic layers permit those combinations.
 *
 * DETERMINISM TESTS
 *
 * Identical source and language version must produce identical parser output.
 *
 * ROUND-TRIP TESTS
 *
 * Where a canonical printer/serializer exists:
 *
 *     source
 *       -> lexer
 *       -> parser
 *       -> AST
 *       -> printer
 *       -> parser
 *
 * must preserve logical-expression structure and semantics.
 */


/* ============================================================================
 * HARD-CODING AUDIT
 * ========================================================================== */

/**
 * Forbidden in this file:
 *
 *     MAX_LOGICAL_OPERANDS
 *     MAX_LOGICAL_DEPTH
 *     MAX_NOT_DEPTH
 *     MAX_AND_TERMS
 *     MAX_OR_TERMS
 *     MAX_PREDICATES
 *     MAX_BOOLEAN_VARIABLES
 *     MAX_CONDITIONS
 *     MAX_QUANTUM_CONDITIONS
 *     MAX_HARDWARE_CONDITIONS
 *
 * Any finite implementation safeguard belongs outside the language grammar.
 */


/* ============================================================================
 * COMPLETION CRITERIA
 * ========================================================================== */

/**
 * This file is complete when:
 *
 *   [ ] Logical operator tokens are consumed exclusively from ZamaniLexer.
 *
 *   [ ] `&&` is represented by LOGICAL_AND.
 *
 *   [ ] `||` is represented by LOGICAL_OR.
 *
 *   [ ] `!` is represented by NOT.
 *
 *   [ ] NOT has higher precedence than AND.
 *
 *   [ ] AND has higher precedence than OR.
 *
 *   [ ] AND is left-associative.
 *
 *   [ ] OR is left-associative.
 *
 *   [ ] NOT supports arbitrary syntactic nesting.
 *
 *   [ ] Logical chains have no artificial finite grammar limit.
 *
 *   [ ] Parentheses can override logical precedence.
 *
 *   [ ] The grammar does not duplicate lexer ownership.
 *
 *   [ ] The grammar does not duplicate semantic/type ownership.
 *
 *   [ ] The grammar does not define hardware limits.
 *
 *   [ ] The grammar does not define quantum-machine limits.
 *
 *   [ ] The grammar does not depend on QEC.
 *
 *   [ ] The grammar does not depend on ZQN.
 *
 *   [ ] The grammar does not depend on routing.
 *
 *   [ ] The grammar does not depend on scheduling.
 *
 *   [ ] The grammar does not depend on optimization.
 *
 *   [ ] The grammar does not create a second quantum IR.
 *
 *   [ ] AST construction preserves operator ordering and source spans.
 *
 *   [ ] Semantic analysis remains downstream.
 *
 *   [ ] Classical IR lowering remains downstream.
 *
 *   [ ] quantum::ir lowering remains downstream where applicable.
 *
 *   [ ] Rust integration remains compatible with Rust 1.97/1.97.1.
 *
 *   [ ] Rust integration requires no unsafe code.
 *
 *   [ ] Positive tests exist.
 *
 *   [ ] Negative tests exist.
 *
 *   [ ] Boundary tests exist.
 *
 *   [ ] Determinism tests exist.
 *
 *   [ ] Cross-domain tests exist.
 *
 *   [ ] Round-trip tests exist where the repository printer supports them.
 *
 *   [ ] No unresolved ownership ambiguity remains with expressions.g4.
 *
 *   [ ] The canonical expression assembler binds `logicalOperand` to the
 *       authoritative lower-precedence expression layer.
 *
 *   [ ] No circular grammar dependency exists.
 *
 * ============================================================================
 */
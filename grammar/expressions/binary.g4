/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/expressions/binary.g4
 *
 * Role:
 *     Canonical parser component for binary-expression syntax and precedence.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust integration baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no embedded Rust code and introduces no unsafe
 *     implementation.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * OWNS:
 *
 *   - binary-expression syntax;
 *   - binary-operator precedence;
 *   - binary-operator associativity;
 *   - syntactic grouping of binary operands;
 *   - arbitrary binary-expression nesting;
 *   - source-level binary operator structure.
 *
 * DOES NOT OWN:
 *
 *   - lexical token definitions;
 *   - operator spellings;
 *   - identifiers;
 *   - literals;
 *   - unary expressions;
 *   - postfix expressions;
 *   - primary expressions;
 *   - function calls;
 *   - indexing;
 *   - member access;
 *   - type checking;
 *   - operator overload resolution;
 *   - implicit conversions;
 *   - constant evaluation;
 *   - ownership;
 *   - borrowing;
 *   - lifetimes;
 *   - effects;
 *   - capabilities;
 *   - resource validation;
 *   - quantum semantics;
 *   - quantum allocation;
 *   - physical qubits;
 *   - QEC;
 *   - ZQN;
 *   - hardware discovery;
 *   - topology;
 *   - routing;
 *   - scheduling;
 *   - optimization;
 *   - classical IR;
 *   - quantum::ir;
 *   - runtime;
 *   - ABI;
 *   - backend selection;
 *   - machine-specific limits.
 *
 * ============================================================================
 * ARCHITECTURAL BOUNDARY
 * ============================================================================
 *
 * Canonical flow:
 *
 *     source
 *       |
 *       v
 *     Zamani lexer
 *       |
 *       v
 *     binary parser component
 *       |
 *       v
 *     expression composition
 *       |
 *       v
 *     frontend AST
 *       |
 *       +--> name resolution
 *       +--> type checking
 *       +--> effect checking
 *       +--> capability checking
 *       +--> resource checking
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> hardware/resource representation
 *       +--> control/data representation
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     routing / scheduling / lowering
 *       |
 *       v
 *     target realization
 *
 * This file MUST NOT create a second IR.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Binary expressions describe computation, not the physical machine on which
 * the computation eventually executes.
 *
 * This grammar therefore contains NO machine-specific limits.
 *
 * It MUST NOT encode:
 *
 *     MAX_OPERANDS
 *     MAX_EXPRESSION_DEPTH
 *     MAX_EXPRESSION_COUNT
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_REGISTER_WIDTH
 *     MAX_VECTOR_WIDTH
 *     MAX_MATRIX_DIMENSION
 *     MAX_TENSOR_RANK
 *     MAX_ARGUMENT_COUNT
 *
 * Any practical parser/compiler resource limit belongs to the implementation
 * policy and MUST NOT become part of the source-language grammar.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * The canonical lexer owns all operator spellings.
 *
 * In particular, this file MUST consume the canonical tokens already defined
 * by the Zamani operator lexer rather than inventing aliases.
 *
 * Canonical operator vocabulary currently includes:
 *
 *     PLUS
 *     MINUS
 *     STAR
 *     SLASH
 *     MODULO
 *     ASSIGN
 *
 *     AMPERSAND
 *     PIPE
 *     CARET
 *
 *     EQUAL_EQUAL
 *     NOT_EQUAL
 *     LESS
 *     GREATER
 *     LESS_EQUAL
 *     GREATER_EQUAL
 *
 *     LOGICAL_AND
 *     LOGICAL_OR
 *
 *     LEFT_SHIFT
 *     RIGHT_SHIFT
 *
 *     PLUS_ASSIGN
 *     MINUS_ASSIGN
 *     STAR_ASSIGN
 *     SLASH_ASSIGN
 *     PERCENT_ASSIGN
 *     AMP_ASSIGN
 *     PIPE_ASSIGN
 *     CARET_ASSIGN
 *
 * The parser MUST NOT introduce alternative names such as:
 *
 *     BIT_AND
 *     BIT_OR
 *     LOGICAL_NOT
 *     BIT_NOT
 *     MODULO_ASSIGN
 *
 * when those names are not present in the canonical lexer.
 *
 * ============================================================================
 * IMPORTANT OPERATOR OWNERSHIP
 * ============================================================================
 *
 * Prefix unary operators are owned by unary.g4.
 *
 * Therefore this file does NOT interpret:
 *
 *     +
 *     -
 *     !
 *     ~
 *     &
 *     *
 *
 * as unary operators.
 *
 * Their binary forms are recognized here only when the grammar position
 * requires two operands.
 *
 * Examples:
 *
 *     a + b
 *     a - b
 *     a * b
 *     a / b
 *     a % b
 *     a & b
 *     a | b
 *     a ^ b
 *
 * Unary forms such as:
 *
 *     -a
 *     !a
 *     ~a
 *     &a
 *     *a
 *
 * belong to unary.g4.
 *
 * ============================================================================
 * BINARY PRECEDENCE MODEL
 * ============================================================================
 *
 * From LOWEST precedence to HIGHEST precedence:
 *
 *     logical OR
 *     logical AND
 *     bitwise OR
 *     bitwise XOR
 *     bitwise AND
 *     equality
 *     relational
 *     shifts
 *     additive
 *     multiplicative
 *     exponentiation
 *     unary/postfix boundary
 *
 * Assignment is deliberately NOT owned here.
 *
 * Assignment is an expression-layer construct because it has different
 * semantic requirements from ordinary binary operators.
 *
 * Conditional expressions are likewise outside this component.
 *
 * ============================================================================
 * ASSOCIATIVITY
 * ============================================================================
 *
 * Logical, bitwise, equality, relational, shift, additive, and multiplicative
 * operators are represented as left-associative operator chains.
 *
 * Exponentiation is represented as right-associative.
 *
 * Thus:
 *
 *     a + b + c
 *
 * is structurally:
 *
 *     (a + b) + c
 *
 * while:
 *
 *     a ** b ** c
 *
 * would be structurally:
 *
 *     a ** (b ** c)
 *
 * ONLY IF a canonical POWER token is introduced by the lexer.
 *
 * The current canonical operator lexer does not establish a POWER token.
 * Therefore this file deliberately does NOT invent one.
 *
 * Exponentiation may instead be supplied later through a language-versioned
 * operator/dialect extension.
 *
 * ============================================================================
 * IMPORTANT: NO INVENTED TOKENS
 * ============================================================================
 *
 * The existing expression grammar contains references to token names that are
 * not established by the current canonical operator lexer, including names
 * such as:
 *
 *     BIT_OR
 *     BIT_AND
 *     LOGICAL_NOT
 *     BIT_NOT
 *     MODULO_ASSIGN
 *
 * This file intentionally corrects that architectural error by using the
 * actual canonical lexical vocabulary:
 *
 *     PIPE
 *     AMPERSAND
 *     NOT
 *     TILDE
 *     PERCENT_ASSIGN
 *
 * `NOT` and `TILDE` are not binary operators in this file; they belong to
 * unary syntax.
 *
 * ============================================================================
 * 1. BINARY EXPRESSION ENTRY POINT
 * ============================================================================
 *
 * `binaryExpression` is the reusable entry point for an expression whose
 * top-level operation, if any, is binary.
 *
 * The operand at the highest level is supplied by the expression composition
 * layer through `unaryExpression`.
 *
 * This creates the intended boundary:
 *
 *     binaryExpression
 *          |
 *          v
 *     unaryExpression
 *          |
 *          v
 *     postfixExpression
 *          |
 *          v
 *     primaryExpression
 *
 * `unaryExpression` is intentionally NOT redefined here.
 */

parser grammar BinaryExpressions;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 2. PUBLIC BINARY EXPRESSION ENTRY POINT
 * ========================================================================== */

/**
 * Complete binary-expression hierarchy.
 *
 * This rule intentionally begins at logical OR, the lowest binary precedence.
 *
 * Assignment and conditional expressions are owned by the surrounding
 * expression composition layer.
 */
binaryExpression
    : logicalOrExpression
    ;


/* ============================================================================
 * 3. LOGICAL OR
 * ========================================================================== */

/**
 * Logical OR:
 *
 *     a || b
 *     a || b || c
 *
 * Left associative.
 */
logicalOrExpression
    : logicalAndExpression
      (
          LOGICAL_OR
          logicalAndExpression
      )*
    ;


/* ============================================================================
 * 4. LOGICAL AND
 * ========================================================================== */

/**
 * Logical AND:
 *
 *     a && b
 *     a && b && c
 *
 * Left associative.
 */
logicalAndExpression
    : bitwiseOrExpression
      (
          LOGICAL_AND
          bitwiseOrExpression
      )*
    ;


/* ============================================================================
 * 5. BITWISE OR
 * ========================================================================== */

/**
 * Bitwise OR:
 *
 *     a | b
 *     mask | value
 *
 * The same syntax may eventually be overloaded for vectors, tensors,
 * hardware values, symbolic values, or domain-specific types.
 *
 * Semantic interpretation is deliberately downstream.
 */
bitwiseOrExpression
    : bitwiseXorExpression
      (
          PIPE
          bitwiseXorExpression
      )*
    ;


/* ============================================================================
 * 6. BITWISE XOR
 * ========================================================================== */

/**
 * Bitwise XOR:
 *
 *     a ^ b
 */
bitwiseXorExpression
    : bitwiseAndExpression
      (
          CARET
          bitwiseAndExpression
      )*
    ;


/* ============================================================================
 * 7. BITWISE AND
 * ========================================================================== */

/**
 * Bitwise AND:
 *
 *     a & b
 */
bitwiseAndExpression
    : equalityExpression
      (
          AMPERSAND
          equalityExpression
      )*
    ;


/* ============================================================================
 * 8. EQUALITY
 * ========================================================================== */

/**
 * Equality comparison:
 *
 *     a == b
 *     a != b
 *
 * Multiple comparison operators are syntactically representable:
 *
 *     a == b == c
 *     a != b != c
 *
 * Whether chained equality is semantically meaningful is determined by the
 * type/semantic layer.
 *
 * The grammar does not silently impose a finite chain length.
 */
equalityExpression
    : relationalExpression
      (
          EQUAL_EQUAL
        | NOT_EQUAL
      )
      relationalExpression
      (
          (
              EQUAL_EQUAL
            | NOT_EQUAL
          )
          relationalExpression
      )*
    ;


/* ============================================================================
 * 9. RELATIONAL
 * ========================================================================== */

/**
 * Relational comparison:
 *
 *     a < b
 *     a <= b
 *     a > b
 *     a >= b
 *
 * Chained comparisons are preserved syntactically so that semantic analysis
 * can decide whether the language's comparison model permits them.
 */
relationalExpression
    : shiftExpression
      (
          LESS
        | LESS_EQUAL
        | GREATER
        | GREATER_EQUAL
      )
      shiftExpression
      (
          (
              LESS
            | LESS_EQUAL
            | GREATER
            | GREATER_EQUAL
          )
          shiftExpression
      )*
    ;


/* ============================================================================
 * 10. SHIFT
 * ========================================================================== */

/**
 * Shift expressions:
 *
 *     value << amount
 *     value >> amount
 *
 * The shift amount is an expression rather than a fixed-width integer
 * literal. Its semantic validity is checked downstream.
 */
shiftExpression
    : additiveExpression
      (
          LEFT_SHIFT
        | RIGHT_SHIFT
      )
      additiveExpression
      (
          (
              LEFT_SHIFT
            | RIGHT_SHIFT
          )
          additiveExpression
      )*
    ;


/* ============================================================================
 * 11. ADDITIVE
 * ========================================================================== */

/**
 * Additive expressions:
 *
 *     a + b
 *     a - b
 *     a + b - c
 *
 * Left associative.
 *
 * The grammar intentionally does not restrict operands to machine integers.
 *
 * Semantic analysis may define addition/subtraction for:
 *
 *     integers
 *     floating-point values
 *     complex values
 *     vectors
 *     matrices
 *     tensors
 *     symbolic values
 *     user-defined types
 *     hardware values
 *     future domain values
 */
additiveExpression
    : multiplicativeExpression
      (
          PLUS
        | MINUS
      )
      multiplicativeExpression
      (
          (
              PLUS
            | MINUS
          )
          multiplicativeExpression
      )*
    ;


/* ============================================================================
 * 12. MULTIPLICATIVE
 * ========================================================================== */

/**
 * Multiplicative expressions:
 *
 *     a * b
 *     a / b
 *     a % b
 *
 * Left associative.
 *
 * `%` remains syntax only. Whether modulo is valid for a particular type is
 * determined downstream.
 */
multiplicativeExpression
    : unaryExpression
      (
          STAR
        | SLASH
        | MODULO
      )
      unaryExpression
      (
          (
              STAR
            | SLASH
            | MODULO
          )
          unaryExpression
      )*
    ;


/* ============================================================================
 * 13. BINARY OPERAND BOUNDARY
 * ========================================================================== */

/**
 * The binary grammar consumes `unaryExpression` as its highest-precedence
 * operand boundary.
 *
 * This rule is deliberately declared only as a reference contract.
 *
 * It MUST be supplied by the expression composition grammar.
 *
 * Do not duplicate the unary grammar here.
 *
 * The canonical composition is:
 *
 *     multiplicativeExpression
 *         -> unaryExpression
 *         -> postfixExpression
 *         -> primaryExpression
 *
 * This prevents:
 *
 *     binary.g4 -> unary.g4
 *     unary.g4  -> binary.g4
 *
 * circular ownership.
 */


/* ============================================================================
 * 14. COMPOUND ASSIGNMENT EXCLUSION
 * ========================================================================== */

/*
 * Compound assignments are NOT binary-expression operators here.
 *
 * The following tokens:
 *
 *     PLUS_ASSIGN
 *     MINUS_ASSIGN
 *     STAR_ASSIGN
 *     SLASH_ASSIGN
 *     PERCENT_ASSIGN
 *     AMP_ASSIGN
 *     PIPE_ASSIGN
 *     CARET_ASSIGN
 *
 * belong to assignment-expression syntax.
 *
 * Example:
 *
 *     x += y
 *
 * is an assignment expression, not:
 *
 *     binaryExpression(PLUS_ASSIGN, x, y)
 *
 * Keeping this distinction is important because assignment may carry:
 *
 *     mutation effects
 *     ownership effects
 *     borrow rules
 *     resource effects
 *     quantum-resource effects
 *     concurrency effects
 *     hardware side effects
 *
 * and therefore must remain visible to the semantic layer as assignment.
 */


/* ============================================================================
 * 15. ARROW EXCLUSION
 * ========================================================================== */

/*
 * The canonical lexer also defines:
 *
 *     THIN_ARROW
 *     FAT_ARROW
 *
 * These are intentionally NOT binary operators here.
 *
 * They are used by constructs such as:
 *
 *     function return types
 *     type relations
 *     match arms
 *     control-flow constructs
 *     language-specific declaration syntax
 *
 * Their ownership belongs to the relevant grammar component.
 */


/* ============================================================================
 * 16. RANGE OPERATOR EXCLUSION
 * ========================================================================== */

/*
 * The canonical lexer defines:
 *
 *     DOT_DOT
 *     DOT_DOT_EQ
 *     ELLIPSIS
 *
 * These are not ordinary binary arithmetic operators.
 *
 * Range expressions belong to the range-expression grammar.
 *
 * Variadic syntax belongs to the relevant declaration/call grammar.
 *
 * Keeping them outside this component prevents semantic conflation between:
 *
 *     a + b
 *
 * and:
 *
 *     a .. b
 */


/* ============================================================================
 * 17. OPTIONAL / NULL-COALESCING EXCLUSION
 * ========================================================================== */

/*
 * The canonical lexer defines:
 *
 *     QUESTION_DOT
 *     NULL_COALESCE
 *
 * These require dedicated semantic treatment.
 *
 * They should be owned by the optional/nullability expression layer rather
 * than silently added to ordinary logical/arithmetic precedence here.
 *
 * This prevents the binary grammar from becoming a dumping ground for every
 * operator-like construct in the language.
 */


/* ============================================================================
 * 18. QUANTUM INTEGRATION
 * ========================================================================== */

/*
 * Binary expressions may appear in quantum programs.
 *
 * Examples:
 *
 *     theta + offset
 *     condition && ready
 *     measured == expected
 *     parameter * scale
 *     index + offset
 *
 * This grammar does NOT decide whether an operand represents:
 *
 *     classical value
 *     symbolic parameter
 *     qubit-related value
 *     measurement result
 *     logical value
 *     resource expression
 *     hardware value
 *
 * Nor does it define:
 *
 *     quantum gates
 *     circuits
 *     qubit allocation
 *     physical qubits
 *     coupling topology
 *     calibration
 *     noise
 *     QEC
 *     ZQN
 *
 * If semantic analysis determines that a binary expression participates in
 * quantum computation, it may eventually be lowered through the canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * This parser component must never construct quantum::ir directly.
 */


/* ============================================================================
 * 19. CLASSICAL INTEGRATION
 * ========================================================================== */

/*
 * Binary operators are domain-neutral syntax.
 *
 * The same grammar can support:
 *
 *     scalar + scalar
 *     vector + vector
 *     matrix * matrix
 *     tensor * tensor
 *     symbolic + symbolic
 *
 * without introducing:
 *
 *     VECTOR_PLUS
 *     MATRIX_PLUS
 *     TENSOR_PLUS
 *
 * unless a future language extension has a genuine syntactic reason for
 * distinct operators.
 *
 * Type-directed interpretation belongs downstream.
 */


/* ============================================================================
 * 20. HDL / HARDWARE INTEGRATION
 * ========================================================================== */

/*
 * Hardware and HDL expressions may reuse:
 *
 *     &
 *     |
 *     ^
 *     <<
 *     >>
 *     ==
 *     !=
 *     <
 *     <=
 *     >
 *     >=
 *     +
 *     -
 *     *
 *     /
 *
 * This grammar does not encode:
 *
 *     bus width
 *     register width
 *     FPGA family
 *     ASIC family
 *     CPU architecture
 *     GPU architecture
 *     clock frequency
 *     device count
 *     physical address
 *     topology
 *
 * Such properties belong to hardware semantics, capabilities, resources,
 * targets, or compilation context.
 */


/* ============================================================================
 * 21. DISTRIBUTED / PARALLEL INTEGRATION
 * ========================================================================== */

/*
 * Binary expressions can occur in:
 *
 *     parallel computation
 *     distributed computation
 *     actor expressions
 *     task expressions
 *     reductions
 *     data-parallel operations
 *
 * This grammar does not decide execution placement.
 *
 * For example:
 *
 *     a + b
 *
 * does not mean:
 *
 *     CPU 0
 *     GPU 1
 *     node 4
 *     quantum device X
 *
 * Placement is determined downstream from capabilities, resources, target
 * descriptions, scheduling, and runtime context.
 */


/* ============================================================================
 * 22. RESOURCE / CAPABILITY INTEGRATION
 * ========================================================================== */

/*
 * Expressions can participate in resource and capability expressions.
 *
 * For example:
 *
 *     available_memory >= required_memory
 *
 * or:
 *
 *     qubits_required + ancillas_required
 *
 * remain source-level expressions.
 *
 * This grammar must never turn such expressions into physical allocation
 * decisions.
 */


/* ============================================================================
 * 23. TYPE SYSTEM CONTRACT
 * ========================================================================== */

/*
 * Binary operators preserve their syntactic token identity.
 *
 * Semantic analysis is responsible for:
 *
 *     overload resolution
 *     type compatibility
 *     implicit conversions
 *     explicit conversions
 *     numeric promotion
 *     comparison rules
 *     overflow policy
 *     division rules
 *     modulo rules
 *     shift rules
 *     vector/tensor broadcasting
 *     symbolic algebra
 *     hardware-width semantics
 *     quantum/classical compatibility
 *
 * Therefore this grammar must remain independent of the concrete type system.
 */


/* ============================================================================
 * 24. CONSTANT-EVALUATION CONTRACT
 * ========================================================================== */

/*
 * The grammar does not evaluate expressions.
 *
 * Examples:
 *
 *     1 + 2
 *     4 * 8
 *     1 << n
 *
 * are represented syntactically.
 *
 * Constant evaluation belongs downstream.
 *
 * This prevents the grammar from imposing:
 *
 *     fixed integer width
 *     fixed floating-point width
 *     fixed arbitrary-precision limit
 *     fixed tensor size
 *
 * on the language.
 */


/* ============================================================================
 * 25. DETERMINISM
 * ========================================================================== */

/*
 * Given identical:
 *
 *     source
 *     language version
 *     canonical lexer
 *     grammar version
 *
 * the parser must produce the same binary-expression structure.
 *
 * Parsing MUST NOT depend on:
 *
 *     CPU count
 *     GPU availability
 *     FPGA availability
 *     QPU availability
 *     machine topology
 *     memory capacity
 *     network state
 *     calibration
 *     runtime state
 *     scheduling
 *     backend choice
 */


/* ============================================================================
 * 26. ERROR-RECOVERY CONTRACT
 * ========================================================================== */

/*
 * This grammar deliberately contains no catch-all operator rule.
 *
 * Examples such as:
 *
 *     a +
 *     a *
 *     a &&
 *     a ==
 *
 * without a valid right operand are parser errors.
 *
 * Examples such as:
 *
 *     string - string
 *     qubit + qubit
 *     immutable_value + mutable_value
 *
 * may be syntactically valid but semantically invalid.
 *
 * Those MUST be diagnosed by semantic/type analysis, not by this grammar.
 */


/* ============================================================================
 * 27. SCALABILITY
 * ========================================================================== */

/*
 * All repeated binary-expression structures use grammar repetition or
 * recursive composition without language-level finite bounds.
 *
 * Therefore there is no grammar-defined maximum for:
 *
 *     operator-chain length
 *     expression nesting
 *     expression count
 *     operand size
 *
 * Practical implementation limits may still exist because every parser is
 * executed with finite computational resources.
 *
 * Such limits MUST be implementation policies rather than source-language
 * semantic restrictions.
 */


/* ============================================================================
 * 28. AST CONTRACT
 * ========================================================================== */

/*
 * The frontend AST should preserve, for every binary operation:
 *
 *     operator kind
 *     left operand
 *     right operand
 *     source span
 *
 * and, where required by the repository's provenance model:
 *
 *     source identity
 *     language version
 *     diagnostic provenance
 *
 * The AST must not contain machine-specific target information merely because
 * a binary expression may eventually execute on a machine.
 */


/* ============================================================================
 * 29. IR CONTRACT
 * ========================================================================== */

/*
 * This grammar has NO direct dependency on:
 *
 *     classical IR
 *     quantum::ir
 *     hardware IR
 *     scheduling IR
 *     routing IR
 *     runtime representation
 *
 * Semantic lowering determines the appropriate representation.
 *
 * For example:
 *
 *     a + b
 *
 * might lower into classical IR.
 *
 * A quantum-related expression may participate in a semantic structure that
 * eventually lowers through quantum::ir.
 *
 * Hardware expressions may lower through the appropriate hardware/HDL
 * representation.
 *
 * The grammar itself remains unchanged.
 */


/* ============================================================================
 * 30. COMPILER CONTRACT
 * ========================================================================== */

/*
 * A compiler consuming this grammar must be able to perform:
 *
 *     lexing
 *       |
 *       v
 *     parsing
 *       |
 *       v
 *     AST construction
 *       |
 *       v
 *     name resolution
 *       |
 *       v
 *     type checking
 *       |
 *       v
 *     effect checking
 *       |
 *       v
 *     resource/capability validation
 *       |
 *       v
 *     semantic lowering
 *
 * without modifying binary.g4 merely because a new:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     simulator
 *     accelerator
 *     cluster
 *     cloud backend
 *     future architecture
 *
 * is introduced.
 */


/* ============================================================================
 * 31. TOOLING CONTRACT
 * ========================================================================== */

/*
 * Formatters, syntax highlighters, language servers, linters, refactoring
 * tools, and diagnostics should consume the parser/token structure instead of
 * implementing a second binary-expression grammar.
 *
 * Operator precedence must therefore be treated as a language-level contract,
 * not rediscovered independently by every tool.
 */


/* ============================================================================
 * 32. COMPATIBILITY CONTRACT
 * ========================================================================== */

/*
 * Existing canonical binary operator spellings remain:
 *
 *     ||
 *     &&
 *     |
 *     ^
 *     &
 *     ==
 *     !=
 *     <
 *     <=
 *     >
 *     >=
 *     <<
 *     >>
 *     +
 *     -
 *     *
 *     /
 *     %
 *
 * This file does not silently introduce new operator spellings.
 *
 * A future binary operator requires:
 *
 *     language-version decision
 *     lexer token
 *     precedence decision
 *     associativity decision
 *     AST representation
 *     semantic definition
 *     diagnostics
 *     compatibility policy
 *     positive tests
 *     negative tests
 *     boundary tests
 *     cross-domain tests
 *
 * before becoming part of the universal grammar.
 */


/* ============================================================================
 * 33. HARD-CODING AUDIT
 * ========================================================================== */

/*
 * This file contains no:
 *
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_VECTOR_LENGTH
 *     MAX_MATRIX_DIMENSION
 *     MAX_TENSOR_RANK
 *     MAX_OPERATOR_CHAIN
 *     MAX_EXPRESSION_DEPTH
 *
 * No machine-specific identifiers occur in grammar productions.
 *
 * No target-specific execution decision occurs in grammar productions.
 */


/* ============================================================================
 * 34. REQUIRED TEST MATRIX
 * ========================================================================== */

/*
 * Positive syntax:
 *
 *     a || b
 *     a && b
 *     a | b
 *     a ^ b
 *     a & b
 *     a == b
 *     a != b
 *     a < b
 *     a <= b
 *     a > b
 *     a >= b
 *     a << b
 *     a >> b
 *     a + b
 *     a - b
 *     a * b
 *     a / b
 *     a % b
 *
 * Precedence:
 *
 *     a + b * c
 *     a * b + c
 *     a << b + c
 *     a < b == c
 *     a & b == c
 *     a ^ b | c
 *     a && b || c
 *
 * Associativity:
 *
 *     a + b + c
 *     a - b - c
 *     a * b * c
 *     a / b / c
 *     a && b && c
 *     a || b || c
 *
 * Unary integration:
 *
 *     -a + b
 *     !(a == b)
 *     ~mask & value
 *     &x == y
 *     *ptr + offset
 *
 * Parenthesized override:
 *
 *     (a + b) * c
 *     a * (b + c)
 *     (a || b) && c
 *
 * Quantum-neutral syntax:
 *
 *     theta + offset
 *     measured == expected
 *     index + offset
 *     condition && ready
 *
 * HDL-neutral syntax:
 *
 *     signal_a & signal_b
 *     bus << shift
 *     enable && ready
 *
 * Large chains:
 *
 *     a + b + c + d + ...
 *
 * must have no language-level fixed maximum.
 *
 * Negative parser tests:
 *
 *     a +
 *     a *
 *     a &&
 *     a ==
 *     + +
 *     a < >
 *
 * must fail appropriately when no valid operand structure exists.
 *
 * Semantic-negative tests MUST remain outside this grammar, for example:
 *
 *     invalid_type_a + invalid_type_b
 *
 * where syntax is valid but types are incompatible.
 */


/* ============================================================================
 * 35. INTEGRATION REQUIREMENT
 * ========================================================================== */

/*
 * The canonical expression composition layer MUST integrate this component
 * rather than duplicate its precedence rules.
 *
 * Conceptual composition:
 *
 *     assignmentExpression
 *         |
 *         v
 *     conditionalExpression
 *         |
 *         v
 *     binaryExpression
 *         |
 *         v
 *     logicalOrExpression
 *         |
 *         v
 *     logicalAndExpression
 *         |
 *         v
 *     bitwiseOrExpression
 *         |
 *         v
 *     bitwiseXorExpression
 *         |
 *         v
 *     bitwiseAndExpression
 *         |
 *         v
 *     equalityExpression
 *         |
 *         v
 *     relationalExpression
 *         |
 *         v
 *     shiftExpression
 *         |
 *         v
 *     additiveExpression
 *         |
 *         v
 *     multiplicativeExpression
 *         |
 *         v
 *     unaryExpression
 *         |
 *         v
 *     postfixExpression
 *         |
 *         v
 *     primaryExpression
 *
 * `expressions.g4` MUST therefore stop maintaining a second independent copy
 * of these binary-precedence rules once this component is assembled.
 *
 * There must be exactly one authoritative binary-precedence definition.
 *
 * ============================================================================
 */
/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/expressions/expressions.g4
 *
 * Role:
 *     Canonical source-level EXPRESSION SYNTAX for Zamani.
 *
 * This file is intentionally a parser grammar.
 *
 * It owns:
 *     - expression syntax;
 *     - expression precedence;
 *     - literals as expressions;
 *     - names and qualified names as expressions;
 *     - unary expressions;
 *     - binary expressions;
 *     - assignment expressions;
 *     - conditional expressions;
 *     - function calls;
 *     - indexing;
 *     - member access;
 *     - ranges;
 *     - tuples;
 *     - arrays;
 *     - maps/object-like literals;
 *     - lambda expressions;
 *     - blocks as expression values;
 *     - control expressions;
 *     - quantum operation expressions;
 *     - measurement expressions;
 *     - classical/quantum conditional expressions;
 *     - compile-time expression forms;
 *     - type-aware expression syntax;
 *     - resource/capability expression syntax;
 *     - expression lists.
 *
 * This file DOES NOT own:
 *     - lexical token definitions;
 *     - identifier spelling;
 *     - comments;
 *     - whitespace;
 *     - source encoding;
 *     - type definitions;
 *     - type checking;
 *     - name resolution;
 *     - constant evaluation;
 *     - borrow checking;
 *     - ownership checking;
 *     - quantum resource checking;
 *     - qubit allocation;
 *     - physical qubit selection;
 *     - hardware discovery;
 *     - topology discovery;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - QEC;
 *     - ZQN;
 *     - simulation;
 *     - classical IR;
 *     - quantum::ir;
 *     - runtime representation;
 *     - ABI selection;
 *     - backend selection;
 *     - machine-specific resource limits.
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
 * ZamaniParser / expression parser
 *   |
 *   v
 * Frontend AST
 *   |
 *   +--> name resolution
 *   +--> type checking
 *   +--> effect checking
 *   +--> resource validation
 *   +--> capability validation
 *   |
 *   v
 * Canonical semantic representation
 *   |
 *   +--> classical IR
 *   +--> quantum::ir
 *   +--> hardware/resource metadata
 *   +--> control/data IR
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
 * The grammar MUST NOT create a second quantum IR.
 *
 * Quantum expressions such as:
 *
 *     H(q)
 *     CNOT(control, target)
 *     measure(q)
 *
 * are syntax only.
 *
 * Their semantic interpretation belongs downstream.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Expressions describe computation and intent rather than machine topology.
 *
 * The grammar therefore MUST NOT encode:
 *
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_REGISTER_COUNT
 *     MAX_TENSOR_RANK
 *     MAX_VECTOR_LENGTH
 *     MAX_MATRIX_DIMENSION
 *     MAX_ARGUMENT_COUNT
 *     MAX_TUPLE_ARITY
 *     MAX_NODES
 *     MAX_ACCELERATORS
 *     MAX_QUANTUM_GATE_ARITY
 *
 * Any implementation limit belongs to an explicit compiler, semantic,
 * resource-management, scheduling, or runtime policy.
 *
 * ============================================================================
 * RUST IMPLEMENTATION CONTRACT
 * ============================================================================
 *
 * This file contains no Rust implementation code.
 *
 * Rust components integrating this grammar MUST target:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * and MUST prohibit unsafe code.
 *
 * The generated frontend must be compatible with:
 *
 *     #![forbid(unsafe_code)]
 *
 * or an equivalent repository-wide policy.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * Tokens are supplied by ZamaniLexer.
 *
 * This grammar deliberately contains NO lexer rules.
 *
 * In particular, do not introduce local lexer rules for:
 *
 *     IDENT
 *     INTEGER
 *     FLOAT
 *     STRING
 *     TRUE
 *     FALSE
 *     operators
 *     punctuation
 *
 * The canonical lexer owns those definitions.
 *
 * ============================================================================
 */

parser grammar Expressions;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 1. PUBLIC ENTRY POINTS
 * ========================================================================== */

/**
 * Standalone expression.
 *
 * The surrounding parser normally consumes `expression`.
 */
expression
    : assignmentExpression
    ;


/**
 * Expression list used by calls, tuple-like forms, attributes, and other
 * expression-bearing constructs.
 *
 * No fixed argument/element count is encoded.
 */
expressionList
    : expression
      (COMMA expression)*
      COMMA?
    ;


/**
 * Optional expression list.
 */
optionalExpressionList
    : expressionList?
    ;


/* ============================================================================
 * 2. ASSIGNMENT EXPRESSIONS
 * ========================================================================== */

/**
 * Assignment has the lowest expression precedence.
 *
 * Examples:
 *
 *     x = y
 *     x += y
 *     x -= y
 *     x *= y
 *     x /= y
 *     x %= y
 *     x &= y
 *     x |= y
 *     x ^= y
 *     x <<= y
 *     x >>= y
 *
 * Assignment associativity is right-to-left:
 *
 *     a = b = c
 *
 * The semantic layer determines whether the left-hand side is assignable.
 */
assignmentExpression
    : conditionalExpression
    | unaryExpression assignmentOperator assignmentExpression
    ;


assignmentOperator
    : ASSIGN
    | PLUS_ASSIGN
    | MINUS_ASSIGN
    | STAR_ASSIGN
    | SLASH_ASSIGN
    | MODULO_ASSIGN
    | BIT_AND_ASSIGN
    | BIT_OR_ASSIGN
    | CARET_ASSIGN
    | LEFT_SHIFT_ASSIGN
    | RIGHT_SHIFT_ASSIGN
    ;


/* ============================================================================
 * 3. CONDITIONAL EXPRESSIONS
 * ========================================================================== */

/**
 * Ternary conditional:
 *
 *     condition ? whenTrue : whenFalse
 *
 * The grammar preserves all three expressions.
 *
 * It does not decide whether branches have compatible types.
 */
conditionalExpression
    : logicalOrExpression
    | logicalOrExpression QUESTION_MARK expression COLON expression
    ;


/* ============================================================================
 * 4. LOGICAL EXPRESSIONS
 * ========================================================================== */

logicalOrExpression
    : logicalAndExpression
      (LOGICAL_OR logicalAndExpression)*
    ;


logicalAndExpression
    : bitwiseOrExpression
      (LOGICAL_AND bitwiseOrExpression)*
    ;


/* ============================================================================
 * 5. BITWISE EXPRESSIONS
 * ========================================================================== */

bitwiseOrExpression
    : bitwiseXorExpression
      (BIT_OR bitwiseXorExpression)*
    ;


bitwiseXorExpression
    : bitwiseAndExpression
      (CARET bitwiseAndExpression)*
    ;


bitwiseAndExpression
    : equalityExpression
      (BIT_AND equalityExpression)*
    ;


/* ============================================================================
 * 6. EQUALITY EXPRESSIONS
 * ========================================================================== */

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
 * 7. RELATIONAL EXPRESSIONS
 * ========================================================================== */

relationalExpression
    : shiftExpression
      (
          LESS_THAN
        | LESS_EQUAL
        | GREATER_THAN
        | GREATER_EQUAL
        | THREE_WAY_COMPARE
      )
      shiftExpression
      (
          (
              LESS_THAN
            | LESS_EQUAL
            | GREATER_THAN
            | GREATER_EQUAL
            | THREE_WAY_COMPARE
          )
          shiftExpression
      )*
    ;


/* ============================================================================
 * 8. SHIFT EXPRESSIONS
 * ========================================================================== */

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
 * 9. ADDITIVE EXPRESSIONS
 * ========================================================================== */

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
 * 10. MULTIPLICATIVE EXPRESSIONS
 * ========================================================================== */

multiplicativeExpression
    : exponentExpression
      (
          STAR
        | SLASH
        | MODULO
      )
      exponentExpression
      (
          (
              STAR
            | SLASH
            | MODULO
          )
          exponentExpression
      )*
    ;


/* ============================================================================
 * 11. EXPONENTIATION
 * ========================================================================== */

/**
 * Exponentiation is right-associative.
 *
 *     a ** b ** c
 *
 * is structurally represented as:
 *
 *     a ** (b ** c)
 *
 * The semantic layer decides whether the selected operand types support
 * exponentiation.
 */
exponentExpression
    : unaryExpression
    | unaryExpression POWER exponentExpression
    ;


/* ============================================================================
 * 12. UNARY EXPRESSIONS
 * ========================================================================== */

unaryExpression
    : postfixExpression
    | unaryOperator unaryExpression
    ;


unaryOperator
    : PLUS
    | MINUS
    | LOGICAL_NOT
    | BIT_NOT
    | INCREMENT
    | DECREMENT
    | ADDRESS_OF
    | DEREFERENCE
    ;


/* ============================================================================
 * 13. POSTFIX EXPRESSIONS
 * ========================================================================== */

/**
 * Postfix expressions are recursively extensible.
 *
 * Examples:
 *
 *     value
 *     value()
 *     value(a, b)
 *     value[index]
 *     value.field
 *     value::member
 *     value?
 *     value++
 *     value--
 *
 * No finite postfix-chain depth is encoded.
 */
postfixExpression
    : primaryExpression postfixPart*
    ;


postfixPart
    : callSuffix
    | indexSuffix
    | memberSuffix
    | qualifiedMemberSuffix
    | postfixOperator
    | optionalAccessSuffix
    ;


/**
 * Function/method call.
 */
callSuffix
    : LPAREN
      optionalExpressionList
      RPAREN
    ;


/**
 * Indexing.
 *
 * The index is an expression rather than an integer literal.
 *
 * Therefore:
 *
 *     a[i]
 *     tensor[i, j]
 *     q[index]
 *     matrix[row, column]
 *
 * are all syntactically possible.
 */
indexSuffix
    : LBRACKET
      expressionList
      RBRACKET
    ;


/**
 * Dot member access.
 */
memberSuffix
    : DOT
      identifier
    ;


/**
 * Namespace/member qualification.
 *
 * This is syntactically distinct from a regular member access so that
 * semantic analysis can distinguish:
 *
 *     value.member
 *
 * from:
 *
 *     namespace::member
 */
qualifiedMemberSuffix
    : DOUBLE_COLON
      identifier
    ;


/**
 * Postfix increment/decrement.
 */
postfixOperator
    : INCREMENT
    | DECREMENT
    ;


/**
 * Optional/null-propagating access.
 *
 * The exact semantic meaning is resolved downstream.
 *
 * Example:
 *
 *     value?
 *
 * when the surrounding semantic context permits postfix optional handling.
 */
optionalAccessSuffix
    : QUESTION_MARK
    ;


/* ============================================================================
 * 14. PRIMARY EXPRESSIONS
 * ========================================================================== */

primaryExpression
    : literalExpression
    | identifierExpression
    | qualifiedNameExpression
    | parenthesizedExpression
    | tupleExpression
    | arrayExpression
    | mapExpression
    | setExpression
    | rangeExpression
    | lambdaExpression
    | blockExpression
    | ifExpression
    | matchExpression
    | loopExpression
    | asyncExpression
    | awaitExpression
    | quantumExpression
    | compileTimeExpression
    | resourceExpression
    | typeExpressionValue
    ;


/* ============================================================================
 * 15. LITERALS
 * ========================================================================== */

literalExpression
    : INTEGER
    | FLOAT
    | STRING
    | CHARACTER
    | TRUE
    | FALSE
    | NULL
    | quantumLiteral
    | durationLiteral
    | sizeLiteral
    ;


/**
 * Quantum literals are intentionally semantic abstractions.
 *
 * The lexer may expose specialized quantum literal tokens.
 *
 * No physical device or qubit count is embedded here.
 */
quantumLiteral
    : QUBIT_LITERAL
    | QUANTUM_STATE_LITERAL
    | COMPLEX_LITERAL
    ;


/**
 * Duration literals remain source-level values.
 *
 * Their scheduling interpretation belongs downstream.
 */
durationLiteral
    : DURATION_LITERAL
    ;


/**
 * Resource-size quantities remain abstract values.
 *
 * They must not imply a machine limit.
 */
sizeLiteral
    : SIZE_LITERAL
    ;


/* ============================================================================
 * 16. IDENTIFIERS AND QUALIFIED NAMES
 * ========================================================================== */

identifierExpression
    : identifier
    ;


qualifiedNameExpression
    : identifier
      DOUBLE_COLON
      identifier
      (
          DOUBLE_COLON
          identifier
      )*
    ;


identifier
    : IDENT
    ;


/* ============================================================================
 * 17. PARENTHESIZED EXPRESSIONS
 * ========================================================================== */

parenthesizedExpression
    : LPAREN
      expression
      RPAREN
    ;


/* ============================================================================
 * 18. TUPLE EXPRESSIONS
 * ========================================================================== */

/**
 * Tuple expressions require a comma.
 *
 *     (a, b)
 *     (a, b, c)
 *     (a,)
 *
 * Unit:
 *
 *     ()
 *
 * is represented separately.
 */
tupleExpression
    : LPAREN
      RPAREN
    | LPAREN
      expression
      COMMA
      tupleExpressionTail?
      RPAREN
    ;


tupleExpressionTail
    : expression
      (COMMA expression)*
      COMMA?
    ;


/* ============================================================================
 * 19. ARRAY EXPRESSIONS
 * ========================================================================== */

/**
 * Array/list literal.
 *
 *     []
 *     [a]
 *     [a, b, c]
 *
 * No element-count ceiling exists.
 */
arrayExpression
    : LBRACKET
      expressionList?
      RBRACKET
    ;


/* ============================================================================
 * 20. MAP / ASSOCIATIVE COLLECTION EXPRESSIONS
 * ========================================================================== */

/**
 * Empty map/object-like literal:
 *
 *     {}
 *
 * Entries:
 *
 *     { key: value }
 *
 * The semantic layer decides the concrete collection type.
 */
mapExpression
    : LBRACE
      mapEntryList?
      RBRACE
    ;


mapEntryList
    : mapEntry
      (COMMA mapEntry)*
      COMMA?
    ;


mapEntry
    : expression COLON expression
    ;


/* ============================================================================
 * 21. SET EXPRESSIONS
 * ========================================================================== */

/**
 * Set literal:
 *
 *     {a, b, c}
 *
 * Map and set literals are intentionally structurally distinct:
 *
 *     {a, b}
 *
 * is a set,
 *
 * while:
 *
 *     {a: b}
 *
 * is a map.
 *
 * Semantic validation determines whether the element types are compatible.
 */
setExpression
    : SET_LITERAL_START
      optionalExpressionList
      SET_LITERAL_END
    ;


/* ============================================================================
 * 22. RANGE EXPRESSIONS
 * ========================================================================== */

/**
 * Range expressions:
 *
 *     a .. b
 *     a ..= b
 *     a ..< b
 *     a ..<step> b
 *
 * The exact range tokenization belongs to ZamaniLexer.
 *
 * Ranges are semantic values and do not imply finite machine dimensions.
 */
rangeExpression
    : expression RANGE_EXCLUSIVE expression
    | expression RANGE_INCLUSIVE expression
    | expression RANGE_HALF_OPEN expression
    ;


/* ============================================================================
 * 23. LAMBDA EXPRESSIONS
 * ========================================================================== */

/**
 * Lambda forms:
 *
 *     |x| x + 1
 *     |x, y| x + y
 *     |x: T| x
 *
 * The parameter grammar is intentionally kept in the functions/declarations
 * grammar. This expression grammar only consumes the expression-level shape.
 */
lambdaExpression
    : LAMBDA_START
      lambdaParameterList?
      LAMBDA_END
      lambdaBody
    ;


lambdaParameterList
    : lambdaParameter
      (COMMA lambdaParameter)*
      COMMA?
    ;


lambdaParameter
    : identifier
    | identifier COLON typeReference
    ;


lambdaBody
    : expression
    | blockExpression
    ;


/**
 * Type reference bridge.
 *
 * The actual type grammar remains owned by grammar/types/types.g4.
 *
 * This rule is intentionally represented through the canonical type-reference
 * token/bridge expected by the parser composition layer.
 */
typeReference
    : TYPE_REFERENCE
    ;


/* ============================================================================
 * 24. BLOCK EXPRESSIONS
 * ========================================================================== */

/**
 * Block expressions are syntax-level expression containers.
 *
 * Their statement/declaration grammar remains owned by the statements and
 * declarations grammars.
 */
blockExpression
    : LBRACE
      blockExpressionItem*
      RBRACE
    ;


blockExpressionItem
    : expression
    | expression SEMI
    | BLOCK_STATEMENT
    ;


/* ============================================================================
 * 25. CONDITIONAL EXPRESSIONS
 * ========================================================================== */

/**
 * Expression-valued conditional:
 *
 *     if condition { ... } else { ... }
 *
 * The semantic layer decides branch compatibility.
 */
ifExpression
    : IF
      expression
      blockExpression
      ELSE
      (
          ifExpression
        | blockExpression
      )
    ;


/* ============================================================================
 * 26. MATCH EXPRESSIONS
 * ========================================================================== */

/**
 * Expression-valued pattern matching.
 */
matchExpression
    : MATCH
      expression
      LBRACE
      matchArm+
      RBRACE
    ;


matchArm
    : CASE_PATTERN
      FAT_ARROW
      expression
      COMMA?
    ;


/* ============================================================================
 * 27. LOOP EXPRESSIONS
 * ========================================================================== */

/**
 * Loop expressions are represented syntactically without imposing execution
 * policy.
 *
 * Whether a loop returns a value, is parallelized, is vectorized, or is
 * lowered to a hardware/quantum execution primitive is a semantic/compiler
 * concern.
 */
loopExpression
    : WHILE
      expression
      blockExpression
    | FOR
      identifier
      IN
      expression
      blockExpression
    | LOOP
      blockExpression
    ;


/* ============================================================================
 * 28. ASYNCHRONOUS EXPRESSIONS
 * ========================================================================== */

asyncExpression
    : ASYNC
      blockExpression
    ;


awaitExpression
    : AWAIT
      postfixExpression
    ;


/* ============================================================================
 * 29. QUANTUM EXPRESSIONS
 * ========================================================================== */

/**
 * Quantum syntax is deliberately generic.
 *
 * The grammar does not enumerate a finite gate set here.
 *
 * This is important for extensibility:
 *
 *     H(q)
 *     X(q)
 *     CNOT(a, b)
 *     custom_gate(q)
 *     vendor_or_domain_operation(...)
 *
 * all have the same expression-level representation.
 *
 * Gate/operation legality is determined by quantum semantic analysis.
 */
quantumExpression
    : quantumOperationExpression
    | quantumMeasurementExpression
    | quantumResetExpression
    | quantumControlExpression
    | quantumStateExpression
    ;


quantumOperationExpression
    : QUANTUM_OPERATION
      LPAREN
      optionalExpressionList
      RPAREN
    ;


quantumMeasurementExpression
    : MEASURE
      LPAREN
      optionalExpressionList
      RPAREN
    ;


quantumResetExpression
    : RESET
      LPAREN
      optionalExpressionList
      RPAREN
    ;


quantumControlExpression
    : CONTROL
      LPAREN
      expression
      RPAREN
      quantumOperationExpression
    ;


quantumStateExpression
    : STATE
      LPAREN
      optionalExpressionList
      RPAREN
    ;


/*
 * Important:
 *
 * QUANTUM_OPERATION is a canonical contextual token supplied by the lexer.
 *
 * The semantic layer determines whether an operation represents:
 *
 *     gate
 *     channel
 *     measurement
 *     state preparation
 *     pulse-level operation
 *     logical operation
 *     hardware primitive
 *     future quantum operation
 *
 * No finite gate-set assumption belongs in this grammar.
 */


/* ============================================================================
 * 30. COMPILE-TIME EXPRESSIONS
 * ========================================================================== */

/**
 * Compile-time expressions.
 *
 * These preserve source-level intent without requiring the grammar to know
 * the implementation of the compile-time evaluator.
 */
compileTimeExpression
    : COMPTIME
      LPAREN
      expression
      RPAREN
    | CONST_EVAL
      LPAREN
      expression
      RPAREN
    ;


/* ============================================================================
 * 31. RESOURCE EXPRESSIONS
 * ========================================================================== */

/**
 * Resource/capability expressions are source-level declarations of intent.
 *
 * They do not select physical resources.
 */
resourceExpression
    : REQUIRE
      LPAREN
      expression
      RPAREN
    | CAPABILITY
      LPAREN
      expression
      RPAREN
    | RESOURCE
      LPAREN
      expression
      RPAREN
    | CONSTRAINT
      LPAREN
      expression
      RPAREN
    | HINT
      LPAREN
      expression
      RPAREN
    ;


/* ============================================================================
 * 32. TYPE-AS-VALUE EXPRESSIONS
 * ========================================================================== */

/**
 * Allows type-level expressions to participate in expression contexts where
 * the language explicitly permits reflection/metaprogramming.
 *
 * The semantic layer determines whether a type can be reified as a value.
 */
typeExpressionValue
    : TYPE_OF
      LPAREN
      typeReference
      RPAREN
    ;


/* ============================================================================
 * 33. EXPRESSION-SAFE CONSTANT FORMS
 * ========================================================================== */

/**
 * Explicitly named semantic constants.
 *
 * These are identifiers at the lexical level; this rule exists as a semantic
 * bridge for parser composition.
 */
constantExpression
    : identifierExpression
    ;


/* ============================================================================
 * 34. SEMANTIC EXTENSION POINT
 * ========================================================================== */

/**
 * Future expression domains must enter through semantic extension mechanisms
 * rather than requiring machine-specific modifications to the core expression
 * precedence hierarchy.
 *
 * This rule is intentionally narrow.
 *
 * Domain-specific syntax should normally be registered through dialect
 * grammars and composed by the top-level parser.
 */
extensionExpression
    : DIALECT_EXPRESSION
    ;
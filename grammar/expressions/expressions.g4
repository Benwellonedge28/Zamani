/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/expressions/expressions.g4
 *
 * Status:
 *     Canonical production expression-composition grammar.
 *
 * Grammar:
 *     ANTLR4 parser grammar
 *
 * Rust integration baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This file contains no embedded Rust actions, semantic predicates,
 *     target-specific code, or unsafe implementation.
 *
 * ============================================================================
 *
 * PURPOSE
 * ============================================================================
 *
 * This file is the AUTHORITATIVE COMPOSITION LAYER for Zamani expressions.
 *
 * It establishes:
 *
 *     expression
 *         |
 *         v
 *     assignment
 *         |
 *         v
 *     conditional
 *         |
 *         v
 *     binary/logical/bitwise/comparison/arithmetic
 *         |
 *         v
 *     unary
 *         |
 *         v
 *     postfix
 *         |
 *         v
 *     primary
 *
 * This file is deliberately NOT a second implementation of every expression
 * subsystem.
 *
 * Lower-level expression components are imported where they have an
 * independent architectural owner.
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - canonical `expression` entry point;
 *   - expression-list composition;
 *   - conditional-expression syntax;
 *   - postfix-expression composition;
 *   - primary-expression composition;
 *   - call suffix composition;
 *   - indexing suffix composition;
 *   - member-access suffix composition;
 *   - qualified-member suffix composition;
 *   - postfix increment/decrement composition;
 *   - optional/null-propagating suffix composition;
 *   - parenthesized-expression composition;
 *   - tuple-expression composition;
 *   - collection-expression composition;
 *   - range-expression composition;
 *   - lambda-expression syntax;
 *   - compile-time expression composition;
 *   - type-expression-as-value composition;
 *   - expression-level quantum invocation syntax;
 *   - expression-level resource/capability syntax;
 *   - expression-level async/await composition;
 *   - integration of imported expression components.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexer tokens;
 *   - identifiers;
 *   - numeric literal spelling;
 *   - string literal spelling;
 *   - character literal spelling;
 *   - operator spelling;
 *   - binary precedence;
 *   - unary precedence;
 *   - assignment operator definitions;
 *   - type syntax;
 *   - statement syntax;
 *   - declaration syntax;
 *   - function declaration syntax;
 *   - quantum IR;
 *   - classical IR;
 *   - HDL IR;
 *   - hardware discovery;
 *   - topology;
 *   - routing;
 *   - scheduling;
 *   - optimization;
 *   - QEC;
 *   - ZQN;
 *   - simulation;
 *   - runtime execution;
 *   - backend selection;
 *   - ABI selection;
 *   - machine-specific resource limits.
 *
 * ============================================================================
 *
 * ARCHITECTURAL BOUNDARY
 * ============================================================================
 *
 *     source
 *        |
 *        v
 *     ZamaniLexer
 *        |
 *        v
 *     Expressions parser
 *        |
 *        v
 *     frontend AST
 *        |
 *        +--> name resolution
 *        +--> type checking
 *        +--> effect checking
 *        +--> ownership/borrowing
 *        +--> capability checking
 *        +--> resource validation
 *        |
 *        v
 *     canonical semantic representation
 *        |
 *        +--> classical IR
 *        +--> quantum::ir
 *        +--> HDL/hardware representation
 *        +--> control/data representation
 *        +--> resource metadata
 *        |
 *        v
 *     optimization
 *        |
 *        v
 *     routing / scheduling / lowering
 *        |
 *        v
 *     target realization
 *        |
 *        v
 *     runtime / hardware
 *
 * There is deliberately NO direct:
 *
 *     grammar -> quantum::ir
 *     grammar -> ZQN
 *     grammar -> QEC
 *     grammar -> scheduler
 *     grammar -> hardware discovery
 *
 * ============================================================================
 *
 * POCO-REAF
 * ============================================================================
 *
 * Expressions describe computation and semantic intent.
 *
 * They MUST NOT encode:
 *
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_REGISTERS
 *     MAX_ARGUMENTS
 *     MAX_TUPLE_ARITY
 *     MAX_ARRAY_SIZE
 *     MAX_TENSOR_RANK
 *     MAX_EXPRESSION_DEPTH
 *     MAX_CIRCUIT_DEPTH
 *     MAX_GATE_COUNT
 *     MAX_RESOURCE_COUNT
 *
 * Repetition is represented structurally through ANTLR repetition and
 * recursion.
 *
 * Practical implementation limits belong to:
 *
 *     parser resource policy
 *     compiler configuration
 *     semantic validation
 *     resource management
 *     scheduling
 *     deployment
 *     runtime
 *
 * They are NOT language-level grammar limits.
 *
 * ============================================================================
 *
 * LEXER CONTRACT
 * ============================================================================
 *
 * All lexical tokens are supplied by:
 *
 *     ZamaniLexer
 *
 * through:
 *
 *     options {
 *         tokenVocab = ZamaniLexer;
 *     }
 *
 * This grammar MUST NOT define lexer rules.
 *
 * In particular, it MUST NOT invent aliases such as:
 *
 *     BIT_AND
 *     BIT_OR
 *     BIT_NOT
 *     LOGICAL_NOT
 *     ADDRESS_OF
 *     DEREFERENCE
 *     LESS_THAN
 *     GREATER_THAN
 *     MODULO_ASSIGN
 *     LEFT_SHIFT_ASSIGN
 *     RIGHT_SHIFT_ASSIGN
 *
 * unless those names are explicitly established by the canonical lexer.
 *
 * The current canonical operator vocabulary includes names such as:
 *
 *     PLUS
 *     MINUS
 *     STAR
 *     SLASH
 *     MODULO
 *     ASSIGN
 *     AMPERSAND
 *     PIPE
 *     CARET
 *     TILDE
 *     NOT
 *     LESS
 *     GREATER
 *     LESS_EQUAL
 *     GREATER_EQUAL
 *     EQUAL_EQUAL
 *     NOT_EQUAL
 *     LOGICAL_AND
 *     LOGICAL_OR
 *     LEFT_SHIFT
 *     RIGHT_SHIFT
 *     PLUS_ASSIGN
 *     MINUS_ASSIGN
 *     STAR_ASSIGN
 *     SLASH_ASSIGN
 *     PERCENT_ASSIGN
 *     AMP_ASSIGN
 *     PIPE_ASSIGN
 *     CARET_ASSIGN
 *     INCREMENT
 *     DECREMENT
 *     QUESTION_DOT
 *     NULL_COALESCE
 *     DOUBLE_COLON
 *     DOT_DOT
 *     DOT_DOT_EQ
 *     ELLIPSIS
 *     THIN_ARROW
 *     FAT_ARROW
 *
 * The exact lexical vocabulary is owned by the lexer authority.
 *
 * ============================================================================
 *
 * IMPORT CONTRACT
 * ============================================================================
 *
 * Lower-level expression components are imported rather than copied.
 *
 * The canonical composition is:
 *
 *     AssignmentExpressions
 *         |
 *         v
 *     ConditionalExpression
 *         |
 *         v
 *     BinaryExpressions
 *         |
 *         v
 *     UnaryExpressions
 *         |
 *         v
 *     postfixExpression
 *         |
 *         v
 *     primaryExpression
 *
 * `binary.g4` already owns the binary hierarchy.
 *
 * `unary.g4` already owns prefix unary syntax.
 *
 * `assignment.g4` already owns assignment syntax.
 *
 * Therefore this file MUST NOT redefine:
 *
 *     assignmentExpression
 *     assignmentOperator
 *     binaryExpression
 *     logicalOrExpression
 *     logicalAndExpression
 *     bitwiseOrExpression
 *     bitwiseXorExpression
 *     bitwiseAndExpression
 *     equalityExpression
 *     relationalExpression
 *     shiftExpression
 *     additiveExpression
 *     multiplicativeExpression
 *     unaryExpression
 *
 * This prevents duplicate rule ownership.
 *
 * ============================================================================
 */

parser grammar Expressions;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * IMPORTED EXPRESSION COMPONENTS
 * ============================================================================
 *
 * These imports establish the canonical lower-level expression hierarchy.
 *
 * IMPORTANT:
 *
 * The imported grammars must be available on the ANTLR grammar source path.
 *
 * The canonical build must compile this composition grammar together with
 * those components.
 */
import AssignmentExpressions,
       BinaryExpressions,
       UnaryExpressions;


/*
 * ============================================================================
 * 1. PUBLIC EXPRESSION ENTRY POINT
 * ============================================================================
 */

/**
 * Canonical source-level expression.
 *
 * This is the rule that statements, declarations, attributes, contracts,
 * initializers, conditions, quantum constructs, HDL expressions, resource
 * expressions, and compile-time expressions should consume.
 *
 * The grammar intentionally exposes ONE canonical entry point.
 */
expression
    : assignmentExpression
    ;


/**
 * Expression sequence.
 *
 * Used where a construct accepts one or more expressions separated by commas.
 *
 * No fixed cardinality is encoded.
 */
expressionList
    : expression
      (COMMA expression)*
      COMMA?
    ;


/**
 * Optional expression sequence.
 */
optionalExpressionList
    : expressionList?
    ;


/*
 * ============================================================================
 * 2. CONDITIONAL EXPRESSIONS
 * ============================================================================
 *
 * Conditional expressions are deliberately owned here rather than in the
 * binary-expression component.
 *
 * This provides the precedence boundary:
 *
 *     assignment
 *         |
 *     conditional
 *         |
 *     binary
 *         |
 *     unary
 *
 * Assignment is therefore lower precedence than the conditional expression.
 *
 * The condition uses the binary expression hierarchy rather than recursively
 * consuming `expression`, preventing an accidental assignment-precedence
 * cycle.
 */
conditionalExpression
    : binaryExpression
    | binaryExpression QUESTION_MARK expression COLON expression
    ;


/*
 * ============================================================================
 * 3. POSTFIX EXPRESSION COMPOSITION
 * ============================================================================
 *
 * Postfix expressions are the bridge between unary syntax and primary
 * expressions.
 *
 * A postfix chain may contain an arbitrary number of suffixes.
 *
 * Examples:
 *
 *     value
 *     value()
 *     value(a, b)
 *     value[index]
 *     value[i, j]
 *     value.field
 *     value::member
 *     value?
 *     value++
 *     value--
 *     value().field[index](arg)
 *
 * No fixed postfix depth is encoded.
 */
postfixExpression
    : primaryExpression postfixPart*
    ;


/*
 * A postfix operation is syntactic only.
 *
 * Semantic analysis decides whether a particular postfix operation is valid
 * for the operand type.
 */
postfixPart
    : callSuffix
    | indexSuffix
    | memberSuffix
    | qualifiedMemberSuffix
    | postfixOperator
    | optionalAccessSuffix
    ;


/*
 * ============================================================================
 * 4. CALL SUFFIX
 * ============================================================================
 */

/**
 * Function/method/callable invocation suffix.
 *
 * Examples:
 *
 *     f()
 *     f(x)
 *     f(x, y)
 *     object.method(x)
 *
 * The grammar does not determine whether the callee is:
 *
 *     a function
 *     a closure
 *     a method
 *     a constructor
 *     a quantum operation
 *     a hardware operation
 *     a distributed invocation
 *     an intrinsic
 *     a future callable abstraction
 *
 * Semantic resolution determines that.
 */
callSuffix
    : LPAREN optionalExpressionList RPAREN
    ;


/*
 * ============================================================================
 * 5. INDEXING
 * ============================================================================
 *
 * Index expressions remain expressions.
 *
 * Therefore the grammar supports:
 *
 *     array[i]
 *     matrix[row, column]
 *     tensor[i, j, k]
 *     q[index]
 *     resource[selector]
 *
 * without assuming a particular index width or number of dimensions.
 */
indexSuffix
    : LBRACKET expressionList RBRACKET
    ;


/*
 * ============================================================================
 * 6. MEMBER ACCESS
 * ============================================================================
 */

/**
 * Ordinary member access.
 *
 *     object.member
 *     value.field
 *     signal.value
 *
 * The member remains an identifier-level name.
 */
memberSuffix
    : DOT identifier
    ;


/**
 * Qualified member/path access.
 *
 *     namespace::member
 *     module::Type
 *     quantum::operation
 *
 * Semantic analysis determines whether the qualified path is:
 *
 *     a namespace
 *     a module
 *     a type
 *     a static member
 *     an extension
 *     a dialect construct
 *     another semantic entity
 */
qualifiedMemberSuffix
    : DOUBLE_COLON identifier
    ;


/*
 * ============================================================================
 * 7. POSTFIX MUTATION
 * ============================================================================
 */

/**
 * Postfix mutation syntax.
 *
 * Semantic analysis determines:
 *
 *     mutability
 *     ownership
 *     borrowing
 *     effect requirements
 *     hardware side effects
 *     concurrency requirements
 */
postfixOperator
    : INCREMENT
    | DECREMENT
    ;


/*
 * ============================================================================
 * 8. OPTIONAL / NULL-PROPAGATING ACCESS
 * ============================================================================
 *
 * The repository lexer already owns QUESTION_DOT as the compound token.
 *
 * This grammar therefore uses QUESTION_DOT rather than attempting to rebuild
 * `?.` from QUESTION + DOT.
 *
 * Plain `?` remains available to conditional-expression syntax through the
 * canonical punctuation/operator contract.
 */
optionalAccessSuffix
    : QUESTION_DOT identifier
    ;


/*
 * ============================================================================
 * 9. PRIMARY EXPRESSIONS
 * ============================================================================
 *
 * Primary expressions are the highest-level atoms consumed by postfix syntax.
 *
 * This rule intentionally remains semantic-neutral.
 */
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
    | asyncExpression
    | awaitExpression
    | ifExpression
    | matchExpression
    | blockExpression
    | quantumExpression
    | compileTimeExpression
    | resourceExpression
    | typeExpressionValue
    ;


/*
 * ============================================================================
 * 10. IDENTIFIERS
 * ============================================================================
 *
 * Identifier spelling belongs to the canonical lexer.
 *
 * This grammar only establishes where an identifier can appear as an
 * expression.
 */
identifierExpression
    : identifier
    ;


identifier
    : IDENT
    ;


/*
 * ============================================================================
 * 11. QUALIFIED NAMES
 * ============================================================================
 *
 * Arbitrary namespace depth is allowed.
 *
 * Examples:
 *
 *     a::b
 *     a::b::c
 *     quantum::math::phase
 *     hardware::resource::capability
 *
 * No namespace-depth limit is encoded.
 */
qualifiedNameExpression
    : identifier DOUBLE_COLON identifier
      (DOUBLE_COLON identifier)*
    ;


/*
 * ============================================================================
 * 12. LITERALS
 * ============================================================================
 *
 * Literal spelling belongs to the canonical lexer.
 *
 * This grammar only composes the resulting tokens into expression syntax.
 */
literalExpression
    : INTEGER
    | FLOAT
    | STRING
    | CHARACTER
    | TRUE
    | FALSE
    | NULL
    | NIL
    | quantumLiteral
    | durationLiteral
    | sizeLiteral
    ;


/**
 * Quantum-related literals remain source-level values.
 *
 * They do NOT identify physical devices, physical qubits, topology, or
 * hardware addresses.
 */
quantumLiteral
    : QUBIT_LITERAL
    | QUANTUM_STATE_LITERAL
    | COMPLEX_LITERAL
    ;


/**
 * Duration values are semantic values.
 *
 * Their interpretation by scheduling/pulse/hardware layers is downstream.
 */
durationLiteral
    : DURATION_LITERAL
    ;


/**
 * Resource-size values are semantic values.
 *
 * They do not impose a maximum machine capacity.
 */
sizeLiteral
    : SIZE_LITERAL
    ;


/*
 * ============================================================================
 * 13. PARENTHESIZED EXPRESSIONS
 * ============================================================================
 *
 * Parentheses explicitly override normal expression precedence.
 */
parenthesizedExpression
    : LPAREN expression RPAREN
    ;


/*
 * ============================================================================
 * 14. TUPLE EXPRESSIONS
 * ============================================================================
 *
 * Unit:
 *
 *     ()
 *
 * One-element tuple:
 *
 *     (value,)
 *
 * Multi-element tuple:
 *
 *     (a, b)
 *     (a, b, c)
 *
 * Tuple cardinality is unbounded by the grammar.
 */
tupleExpression
    : LPAREN RPAREN
    | LPAREN expression COMMA tupleExpressionTail? RPAREN
    ;


tupleExpressionTail
    : expression
      (COMMA expression)*
      COMMA?
    ;


/*
 * ============================================================================
 * 15. ARRAY / LIST EXPRESSIONS
 * ============================================================================
 *
 * Examples:
 *
 *     []
 *     [a]
 *     [a, b, c]
 *
 * Element count is not bounded by the grammar.
 */
arrayExpression
    : LBRACKET optionalExpressionList RBRACKET
    ;


/*
 * ============================================================================
 * 16. MAP EXPRESSIONS
 * ============================================================================
 *
 * Map/object-like syntax:
 *
 *     {}
 *     {key: value}
 *     {a: b, c: d}
 *
 * The semantic layer determines the resulting map/object type.
 */
mapExpression
    : LBRACE mapEntryList? RBRACE
    ;


mapEntryList
    : mapEntry
      (COMMA mapEntry)*
      COMMA?
    ;


mapEntry
    : expression COLON expression
    ;


/*
 * ============================================================================
 * 17. SET EXPRESSIONS
 * ============================================================================
 *
 * Set syntax:
 *
 *     {a, b, c}
 *
 * Map syntax:
 *
 *     {a: b, c: d}
 *
 * The semantic layer distinguishes the two forms.
 *
 * Empty braces remain owned by mapExpression because:
 *
 *     {}
 *
 * has no elements from which a set/map distinction can be inferred.
 */
setExpression
    : LBRACE expressionList RBRACE
    ;


/*
 * ============================================================================
 * 18. RANGE EXPRESSIONS
 * ============================================================================
 *
 * Range operators are already lexical tokens.
 *
 * Supported forms:
 *
 *     start .. end
 *     start ..= end
 *
 * Bounds are arbitrary expressions.
 *
 * No fixed integer representation or machine width is encoded.
 */
rangeExpression
    : rangeOperand DOT_DOT rangeOperand
    | rangeOperand DOT_DOT_EQ rangeOperand
    ;


rangeOperand
    : binaryExpression
    ;


/*
 * ============================================================================
 * 19. LAMBDA EXPRESSIONS
 * ============================================================================
 *
 * Zamani supports source-level anonymous callable expressions.
 *
 * The lambda body is an expression.
 *
 * Parameter syntax is intentionally represented through a small expression
 * bridge here rather than redefining the function-declaration parameter
 * grammar.
 *
 * A later dedicated functions/lambdas component may refine parameter syntax
 * without changing the canonical expression entry point.
 *
 * Examples:
 *
 *     |x| x + 1
 *     |x, y| x + y
 *
 * The pipe token is also used by binary expressions. Contextual parser
 * position distinguishes lambda parameter delimiters from binary operators.
 */
lambdaExpression
    : PIPE lambdaParameterList? PIPE lambdaBody
    ;


lambdaParameterList
    : lambdaParameter
      (COMMA lambdaParameter)*
      COMMA?
    ;


lambdaParameter
    : identifier
    | identifier COLON typeExpression
    ;


lambdaBody
    : expression
    ;


/*
 * ============================================================================
 * 20. ASYNCHRONOUS EXPRESSIONS
 * ============================================================================
 *
 * `async` is a language keyword.
 *
 * The expression grammar does not decide whether an asynchronous computation
 * executes on a thread, process, accelerator, remote node, quantum runtime,
 * or another execution substrate.
 */
asyncExpression
    : ASYNC lambdaExpression
    ;


/*
 * ============================================================================
 * 21. AWAIT
 * ============================================================================
 *
 * Await remains syntax-level.
 *
 * Runtime scheduling and execution semantics are downstream.
 */
awaitExpression
    : AWAIT unaryExpression
    ;


/*
 * ============================================================================
 * 22. CONDITIONAL EXPRESSION
 * ============================================================================
 *
 * Expression-level `if` is distinct from statement-level `if`.
 *
 * The branches are expressions, making the construct usable as a value.
 *
 * Example:
 *
 *     if condition { valueA } else { valueB }
 *
 * The block syntax is kept in `blockExpression`.
 */
ifExpression
    : IF expression blockExpression
      (ELSE IF expression blockExpression)*
      (ELSE blockExpression)?
    ;


/*
 * ============================================================================
 * 23. MATCH EXPRESSIONS
 * ============================================================================
 *
 * Expression-level pattern matching.
 *
 * The semantic layer determines:
 *
 *     exhaustiveness
 *     reachability
 *     type compatibility
 *     ownership
 *     effects
 *     resource requirements
 */
matchExpression
    : MATCH expression LBRACE matchArm+ RBRACE
    ;


matchArm
    : CASE pattern (WHEN expression)? FAT_ARROW expression
    ;


pattern
    : literalExpression
    | identifier
    | UNDERSCORE
    | tuplePattern
    | arrayPattern
    | orPattern
    ;


tuplePattern
    : LPAREN pattern COMMA pattern
      (COMMA pattern)*
      COMMA?
      RPAREN
    ;


arrayPattern
    : LBRACKET
      pattern
      (COMMA pattern)*
      COMMA?
      RBRACKET
    ;


orPattern
    : pattern PIPE pattern
    ;


/*
 * ============================================================================
 * 24. BLOCK EXPRESSIONS
 * ============================================================================
 *
 * A block expression is an expression-valued block.
 *
 * IMPORTANT:
 *
 * The statements inside the block are owned by the statement grammar.
 *
 * The canonical parser composition layer must import the statement grammar
 * when block expressions are enabled in the assembled parser.
 *
 * This rule therefore establishes only the expression-level boundary.
 *
 * The `statement` rule is supplied by the canonical statement composition
 * grammar.
 */
blockExpression
    : LBRACE statement* expressionTail? RBRACE
    ;


expressionTail
    : expression
    ;


/*
 * ============================================================================
 * 25. QUANTUM EXPRESSION COMPOSITION
 * ============================================================================
 *
 * Quantum operation names intentionally remain identifiers unless explicitly
 * reserved by the language.
 *
 * Therefore:
 *
 *     H(q)
 *     X(q)
 *     CNOT(control, target)
 *     RX(theta, q)
 *     measure(q)
 *
 * can be represented through ordinary call expressions.
 *
 * This rule additionally provides explicit language-level quantum forms whose
 * keywords already exist in the lexical vocabulary.
 *
 * IMPORTANT:
 *
 * This is syntax only.
 *
 * It does NOT:
 *
 *     allocate qubits
 *     select physical qubits
 *     discover QPU topology
 *     schedule gates
 *     perform QEC
 *     model noise
 *     select calibration
 *     create quantum::ir
 */
quantumExpression
    : quantumCallExpression
    | measurementExpression
    | resetExpression
    | quantumApplyExpression
    | quantumObserveExpression
    ;


quantumCallExpression
    : identifier LPAREN optionalExpressionList RPAREN
    ;


measurementExpression
    : MEASURE LPAREN optionalExpressionList RPAREN
    ;


resetExpression
    : RESET LPAREN optionalExpressionList RPAREN
    ;


quantumApplyExpression
    : APPLY LPAREN optionalExpressionList RPAREN
    ;


quantumObserveExpression
    : OBSERVE LPAREN optionalExpressionList RPAREN
    ;


/*
 * ============================================================================
 * 26. COMPILE-TIME EXPRESSIONS
 * ============================================================================
 *
 * Compile-time syntax describes evaluation intent.
 *
 * It does not execute anything while parsing.
 *
 * The compiler decides whether a particular expression is eligible for
 * compile-time evaluation.
 */
compileTimeExpression
    : compileTimeIdentifier
    | compileTimeCall
    ;


compileTimeIdentifier
    : SIZEOF LPAREN typeOrExpression RPAREN
    ;


compileTimeCall
    : identifier BANG LPAREN optionalExpressionList RPAREN
    ;


typeOrExpression
    : typeExpression
    | expression
    ;


/*
 * ============================================================================
 * 27. RESOURCE / CAPABILITY EXPRESSIONS
 * ============================================================================
 *
 * Resource expressions are deliberately generic.
 *
 * They describe requirements/preferences/constraints as source semantics.
 *
 * They do NOT identify a particular:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     device
 *     topology
 *     physical qubit
 *     host
 *
 * Those decisions belong to compilation/deployment/resource management.
 */
resourceExpression
    : resourceIdentifier
    | resourceCall
    ;


resourceIdentifier
    : REQUIREMENT_NAME
    ;


resourceCall
    : REQUIREMENT_NAME LPAREN optionalExpressionList RPAREN
    ;


/*
 * ============================================================================
 * 28. TYPE EXPRESSIONS USED AS VALUES
 * ============================================================================
 *
 * Type syntax itself belongs to grammar/types/.
 *
 * This expression layer merely provides the syntactic bridge required for
 * constructs such as:
 *
 *     sizeof(T)
 *     type-aware compile-time operations
 *     generic/type reflection
 *
 * The canonical typeExpression rule is imported from the type grammar by the
 * assembled parser architecture.
 */
typeExpressionValue
    : TYPE LPAREN typeExpression RPAREN
    ;


/*
 * ============================================================================
 * 29. IDENTIFIER-LIKE EXTENSION BRIDGES
 * ============================================================================
 *
 * Some future domains may introduce contextual resource or capability names.
 *
 * They remain ordinary identifiers wherever possible.
 *
 * This is intentional.
 *
 * The universal grammar must not require a new lexer keyword whenever a new:
 *
 *     backend
 *     accelerator
 *     QPU
 *     HDL construct
 *     mathematical operation
 *     AI primitive
 *     distributed service
 *
 * is introduced.
 *
 * Semantic namespaces and dialect registration provide extensibility without
 * destabilizing the universal core grammar.
 */
REQUIREMENT_NAME
    : IDENT
    ;


/*
 * ============================================================================
 * 30. DOCUMENTED COMPOSITION CONTRACT
 * ============================================================================
 *
 * Canonical dependency graph:
 *
 *     ZamaniLexer
 *          |
 *          +--------------------------------+
 *          |                                |
 *          v                                v
 *     AssignmentExpressions          BinaryExpressions
 *          |                                |
 *          |                                v
 *          |                         UnaryExpressions
 *          |                                |
 *          +----------------+---------------+
 *                           |
 *                           v
 *                   Expressions.g4
 *                           |
 *                           +--> conditionalExpression
 *                           +--> postfixExpression
 *                           +--> primaryExpression
 *                           +--> language-level expression forms
 *                           |
 *                           v
 *                       Frontend AST
 *
 *
 * This file MUST NOT introduce reverse dependencies such as:
 *
 *     AST -> grammar
 *     IR -> grammar
 *     runtime -> grammar
 *     hardware -> grammar
 *     scheduler -> grammar
 *
 * ============================================================================
 *
 * 31. AST CONTRACT
 * ============================================================================
 *
 * The grammar produces parser structure only.
 *
 * The frontend AST must preserve:
 *
 *     - source spans;
 *     - expression ordering;
 *     - operator/token identity;
 *     - literal identity;
 *     - qualified names;
 *     - call arguments;
 *     - index expressions;
 *     - member paths;
 *     - tuple/list/map/set structure;
 *     - lambda parameters/body;
 *     - conditional branches;
 *     - quantum call structure;
 *     - compile-time intent;
 *     - resource-expression structure.
 *
 * AST construction belongs to the frontend.
 *
 * This grammar must never instantiate Rust AST structures.
 *
 * ============================================================================
 *
 * 32. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing MUST NOT decide:
 *
 *     whether x is mutable;
 *     whether x is assignable;
 *     whether types match;
 *     whether a gate is supported;
 *     whether a qubit is physical;
 *     whether hardware exists;
 *     whether a resource is available;
 *     whether a topology supports an operation;
 *     whether a schedule is feasible;
 *     whether an optimization is legal;
 *     whether a QEC code is appropriate;
 *     whether ZQN predicts a fault;
 *     whether a backend can execute the program.
 *
 * Those belong downstream.
 *
 * ============================================================================
 *
 * 33. QUANTUM INTEGRATION CONTRACT
 * ============================================================================
 *
 * Quantum source expressions are lowered as follows:
 *
 *     Zamani source
 *          |
 *          v
 *     expression AST
 *          |
 *          v
 *     semantic quantum analysis
 *          |
 *          v
 *     canonical quantum::ir
 *          |
 *          +--> optimization
 *          +--> routing
 *          +--> scheduling
 *          +--> QEC
 *          +--> ZQN-aware execution
 *          +--> hardware realization
 *
 * `Expressions.g4` MUST NEVER become a quantum IR.
 *
 * In particular, syntax such as:
 *
 *     q[0]
 *     q[1]
 *
 * does not establish a language-level maximum or a particular physical
 * topology.
 *
 * ============================================================================
 *
 * 34. CLASSICAL INTEGRATION CONTRACT
 * ============================================================================
 *
 * The same expression grammar supports:
 *
 *     scalar
 *     vector
 *     matrix
 *     tensor
 *     symbolic
 *     numerical
 *     accelerator
 *     systems
 *     embedded
 *     distributed
 *
 * semantics.
 *
 * Operator meaning is determined downstream.
 *
 * ============================================================================
 *
 * 35. HDL / HARDWARE INTEGRATION CONTRACT
 * ============================================================================
 *
 * Expressions can occur inside:
 *
 *     signal assignments
 *     port declarations
 *     timing expressions
 *     widths
 *     addresses
 *     state-machine transitions
 *     pipeline expressions
 *     hardware parameters
 *     resource constraints
 *
 * This grammar does not determine:
 *
 *     register width
 *     bus width
 *     FPGA family
 *     ASIC technology
 *     clock frequency
 *     physical address
 *     device count
 *     topology
 *
 * Those belong to HDL/hardware semantics.
 *
 * ============================================================================
 *
 * 36. RESOURCE / POCO-REAF CONTRACT
 * ============================================================================
 *
 * The expression grammar must remain valid regardless of whether the target
 * has:
 *
 *     one CPU
 *     many CPUs
 *     one GPU
 *     many GPUs
 *     an FPGA
 *     an ASIC
 *     a QPU
 *     many QPUs
 *     a cluster
 *     a supercomputer
 *     a distributed deployment
 *     a future computational substrate
 *
 * The source expression describes computation.
 *
 * Resource selection is downstream.
 *
 * ============================================================================
 *
 * 37. DETERMINISM
 * ============================================================================
 *
 * For identical:
 *
 *     source
 *     language version
 *     lexer version
 *     grammar version
 *
 * the parser must produce the same parse structure.
 *
 * Parsing MUST NOT depend on:
 *
 *     CPU count
 *     GPU availability
 *     QPU availability
 *     network state
 *     calibration
 *     scheduler state
 *     runtime state
 *     deployment state
 *     random values
 *     current time
 *
 * ============================================================================
 *
 * 38. ERROR BOUNDARY
 * ============================================================================
 *
 * Syntax errors belong to parsing.
 *
 * Examples:
 *
 *     f(
 *     a +
 *     x =
 *     [a,
 *     {a:
 *     if condition {
 *
 * Semantic errors belong downstream.
 *
 * Examples:
 *
 *     immutable = value
 *     integer + quantum_state
 *     unsupported_gate(...)
 *     unavailable_resource(...)
 *     invalid_qubit_mapping
 *     insufficient_device_capability
 *
 * The grammar must not turn semantic errors into parser-specific constructs.
 *
 * ============================================================================
 *
 * 39. SCALABILITY
 * ============================================================================
 *
 * No finite semantic limits are encoded.
 *
 * The grammar permits arbitrary repetition wherever the language semantics
 * permit arbitrary collections/chains:
 *
 *     expression chains
 *     call arguments
 *     tuple elements
 *     array elements
 *     map entries
 *     set elements
 *     namespace depth
 *     postfix chains
 *     unary nesting
 *     binary chains
 *     generic expressions
 *
 * Actual implementation resource limits remain implementation policy.
 *
 * ============================================================================
 *
 * 40. COMPATIBILITY
 * ============================================================================
 *
 * The following are compatibility-sensitive:
 *
 *     expression entry rule
 *     conditional precedence
 *     postfix precedence
 *     binary precedence
 *     unary precedence
 *     assignment precedence
 *     tuple syntax
 *     range syntax
 *     call syntax
 *     indexing syntax
 *     member syntax
 *     lambda delimiters
 *
 * A language-version change MUST NOT silently alter their meaning.
 *
 * ============================================================================
 *
 * 41. TOOLING CONTRACT
 * ============================================================================
 *
 * IDEs, formatters, syntax highlighters, refactoring tools, language servers,
 * documentation generators, and source analyzers should consume the parser
 * structure rather than implement a second expression grammar.
 *
 * ============================================================================
 *
 * 42. RUST CONTRACT
 * ============================================================================
 *
 * This file contains no Rust.
 *
 * All Rust consumers of the generated parser/frontend MUST:
 *
 *     target Rust 1.97 / 1.97.1
 *     remain safe Rust
 *     forbid unsafe code
 *
 * Recommended repository-level enforcement:
 *
 *     #![forbid(unsafe_code)]
 *
 * No expression feature requires unsafe Rust.
 *
 * ============================================================================
 *
 * 43. TEST CONTRACT
 * ============================================================================
 *
 * The integration test suite must cover at least:
 *
 * BASIC
 *
 *     x
 *     1
 *     1.0
 *     "text"
 *     true
 *     null
 *
 * PRECEDENCE
 *
 *     a + b * c
 *     a * b + c
 *     a && b || c
 *     a | b ^ c & d
 *     a == b && c != d
 *
 * ASSIGNMENT
 *
 *     x = y
 *     x += y
 *     a = b = c
 *
 * CONDITIONAL
 *
 *     condition ? a : b
 *
 * CALLS
 *
 *     f()
 *     f(a)
 *     f(a, b, c)
 *
 * INDEXING
 *
 *     a[i]
 *     a[i, j]
 *
 * MEMBER ACCESS
 *
 *     a.b
 *     a.b.c
 *     a::b
 *     a::b::c
 *
 * POSTFIX
 *
 *     x++
 *     x--
 *
 * COLLECTIONS
 *
 *     []
 *     [a, b, c]
 *     {}
 *     {a, b, c}
 *     {a: b, c: d}
 *
 * TUPLES
 *
 *     ()
 *     (a,)
 *     (a, b)
 *
 * RANGES
 *
 *     a .. b
 *     a ..= b
 *
 * LAMBDAS
 *
 *     |x| x
 *     |x, y| x + y
 *
 * ASYNC
 *
 *     async |x| x
 *     await value
 *
 * QUANTUM
 *
 *     H(q)
 *     CNOT(control, target)
 *     measure(q)
 *     reset(q)
 *
 * HYBRID
 *
 *     result = measure(q)
 *     if result == true { value } else { other }
 *
 * HARDWARE/HDL
 *
 *     signal = value
 *     register[index] = next
 *
 * SCALABILITY
 *
 *     very long postfix chains
 *     very long argument lists
 *     very long qualified names
 *     very long binary chains
 *     deeply nested expressions
 *
 * The tests must verify that no language-level artificial resource limit is
 * encoded in the grammar.
 *
 * ============================================================================
 *
 * 44. NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * Must reject malformed structures such as:
 *
 *     f(
 *     f(,)
 *     x +
 *     x =
 *     a .. 
 *     a ..=
 *     (a b)
 *     [a b]
 *     {a:}
 *     {a,}
 *     |x
 *
 * where the specific form is not valid under the canonical language version.
 *
 * Negative tests must distinguish:
 *
 *     lexical errors
 *     parser errors
 *     semantic errors
 *
 * ============================================================================
 *
 * 45. INTEGRATION COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when:
 *
 *   1. It compiles as an ANTLR parser grammar.
 *
 *   2. All imported parser grammars resolve.
 *
 *   3. Every referenced lexer token exists in ZamaniLexer.
 *
 *   4. No lexer rule is duplicated here.
 *
 *   5. No binary precedence rule is duplicated here.
 *
 *   6. No unary rule is duplicated here.
 *
 *   7. No assignment rule is duplicated here.
 *
 *   8. `conditionalExpression` is the unique conditional-expression owner.
 *
 *   9. `postfixExpression` is the unique postfix-expression owner.
 *
 *  10. `primaryExpression` is the unique primary-expression composition
 *      owner.
 *
 *  11. The frontend AST can represent every expression alternative.
 *
 *  12. Semantic analysis can consume the AST without requiring grammar
 *      changes for a new backend.
 *
 *  13. Quantum expressions lower through semantic analysis to canonical
 *      `quantum::ir`, never directly from grammar.
 *
 *  14. No QEC/ZQN/hardware/scheduler logic occurs in this grammar.
 *
 *  15. No machine-size constant occurs in this grammar.
 *
 *  16. Rust consumers compile on Rust 1.97 / 1.97.1.
 *
 *  17. No unsafe Rust is required.
 *
 *  18. Positive, negative, boundary, determinism, cross-domain, and
 *      round-trip tests pass.
 *
 * ============================================================================
 *
 * 46. IMPORTANT REPOSITORY INTEGRATION NOTE
 * ============================================================================
 *
 * This file intentionally exposes the correct architectural contracts, but
 * the repository currently has several token/component inconsistencies that
 * MUST be reconciled in their owning files rather than hidden here.
 *
 * In particular:
 *
 *   - `operators.g4` uses AMPERSAND/PIPE/TILDE/NOT/LESS/GREATER;
 *   - existing expression components reference alternative names;
 *   - assignment grammar references assignment tokens not currently exposed
 *     by the operator lexer;
 *   - arithmetic and binary grammars currently overlap in ownership;
 *   - the canonical lexer assembly must resolve the `NOT` keyword/operator
 *     collision;
 *   - blockExpression requires the canonical statement grammar;
 *   - typeExpression requires the canonical Types parser composition.
 *
 * Those are integration-owner issues, not reasons to introduce duplicate
 * token definitions or hidden parser aliases in this file.
 *
 * The architectural rule is:
 *
 *     fix a concept in its owning layer,
 *     then consume the canonical result here.
 *
 * ============================================================================
 */
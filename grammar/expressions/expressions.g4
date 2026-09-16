/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/expressions/expression.g4
 *
 * Status:
 *     Canonical production expression grammar.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Edition 2021
 *     Safe Rust only
 *     No unsafe Rust required or permitted by the compiler implementation.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the canonical source-level expression grammar.
 *
 * It owns the complete expression hierarchy and provides the single public
 * parser entry point:
 *
 *     expression
 *
 * The hierarchy is:
 *
 *     expression
 *         |
 *         v
 *     assignmentExpression
 *         |
 *         v
 *     conditionalExpression
 *         |
 *         v
 *     rangeExpression
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
 *     prefixExpression
 *         |
 *         v
 *     postfixExpression
 *         |
 *         v
 *     primaryExpression
 *
 * ============================================================================
 * ARCHITECTURAL BOUNDARY
 * ============================================================================
 *
 * This grammar defines SOURCE SYNTAX only.
 *
 * It does NOT define:
 *
 *     - semantic types;
 *     - overload resolution;
 *     - name resolution;
 *     - ownership;
 *     - borrowing;
 *     - effects;
 *     - capability satisfaction;
 *     - resource allocation;
 *     - machine topology;
 *     - physical addresses;
 *     - physical qubits;
 *     - routing;
 *     - scheduling;
 *     - calibration;
 *     - optimization;
 *     - QEC implementation;
 *     - ZQN implementation;
 *     - HAL implementation;
 *     - backend selection;
 *     - runtime execution;
 *     - ABI selection;
 *     - target-specific instruction selection.
 *
 * Canonical pipeline:
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     expression parser
 *       |
 *       v
 *     domain-neutral frontend AST
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     canonical semantic model / ZUIR
 *       |
 *       +----------------------+----------------------+
 *       |                      |                      |
 *       v                      v                      v
 *   classical             quantum::ir          HDL/hardware
 *       |                      |                      |
 *       +----------------------+----------------------+
 *                              |
 *                              v
 *                    optimization / lowering
 *                              |
 *                    routing / scheduling
 *                              |
 *                    resilience / QEC / ZQN
 *                              |
 *                             HAL
 *                              |
 *                       target realization
 *
 * IMPORTANT:
 *
 *     quantum::ir
 *
 * remains the canonical quantum semantic boundary.
 *
 * This grammar MUST NOT introduce another quantum IR.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * The language must support:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Expressions therefore describe computation and intent rather than the
 * physical machine used to realize that computation.
 *
 * This grammar imposes NO universal maximum for:
 *
 *     qubits
 *     CPUs
 *     cores
 *     threads
 *     GPUs
 *     FPGAs
 *     ASIC resources
 *     nodes
 *     memory
 *     registers
 *     tensor dimensions
 *     vector widths
 *     arguments
 *     tuple elements
 *     array elements
 *     expression chains
 *     call chains
 *     namespace depth
 *     index dimensions
 *     resource counts
 *     distributed participants
 *     timelines
 *     circuit depth
 *     operation count
 *
 * Repetition is expressed structurally using ANTLR repetition operators.
 *
 * "Infinity" means:
 *
 *     no artificial language-level finite limit is introduced here.
 *
 * Actual limits remain implementation/resource-policy concerns:
 *
 *     parser resource policy
 *     compiler resource policy
 *     available memory
 *     available storage
 *     execution resources
 *     target capabilities
 *     deployment constraints
 *
 * ============================================================================
 * SINGLE AUTHORITY
 * ============================================================================
 *
 * This file is intended to become the canonical expression composition point.
 *
 * Existing expression files remain specialized contracts:
 *
 *     grammar/expressions/arithmetic.g4
 *     grammar/expressions/assignment.g4
 *     grammar/expressions/binary.g4
 *     grammar/expressions/bitwise.g4
 *     grammar/expressions/comparison.g4
 *     grammar/expressions/conditionals.g4
 *     grammar/expressions/indexing.g4
 *     grammar/expressions/literals.g4
 *     grammar/expressions/unary.g4
 *     grammar/expressions/ranges.g4
 *     ...
 *
 * However, those files MUST NOT create a competing public `expression` rule.
 *
 * The integration migration is:
 *
 *     expression.g4
 *             |
 *             +--> canonical expression hierarchy
 *
 * Specialized files:
 *
 *     lexical contracts
 *     semantic contracts
 *     feature documentation
 *     tests
 *     future decomposition units
 *
 * `grammar/expressions/expressions.g4` is therefore a legacy composition
 * surface and must eventually become a compatibility wrapper or be retired
 * after references have been migrated.
 *
 * It MUST NOT remain a second authoritative expression grammar.
 *
 * ============================================================================
 * TOKEN AUTHORITY
 * ============================================================================
 *
 * Tokens are owned by:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This file contains NO lexer rules.
 *
 * The parser consumes the canonical lexical vocabulary.
 *
 * Important canonical tokens used here include:
 *
 *     IDENTIFIER
 *     INTEGER
 *     FLOAT
 *     STRING
 *     CHAR
 *     TRUE
 *     FALSE
 *     NIL
 *     NULL
 *     QUANTUM_LITERAL
 *     MTS_LITERAL
 *
 * Operators:
 *
 *     ASSIGN
 *     PLUS_ASSIGN
 *     MINUS_ASSIGN
 *     STAR_ASSIGN
 *     SLASH_ASSIGN
 *     DOT_DOT
 *     DOT_DOT_EQ
 *     EQ_EQ
 *     NOT_EQ
 *     LE
 *     GE
 *     SHIFT_LEFT
 *     SHIFT_RIGHT
 *     AND_AND
 *     OR_OR
 *     PLUS
 *     MINUS
 *     STAR
 *     SLASH
 *     PERCENT
 *     NOT_OPERATOR
 *     TILDE
 *     AMPERSAND
 *     PIPE
 *     CARET
 *     QUESTION
 *     DOUBLE_COLON
 *     ARROW
 *     FAT_ARROW
 *
 * Delimiters:
 *
 *     LPAREN
 *     RPAREN
 *     LBRACE
 *     RBRACE
 *     LBRACKET
 *     RBRACKET
 *     COMMA
 *     DOT
 *     COLON
 *     SEMICOLON
 *
 * Keywords consumed by expression constructs include:
 *
 *     IF
 *     ELSE
 *     MATCH
 *     LOOP
 *     WHILE
 *     FOR
 *     IN
 *     ASYNC
 *     AWAIT
 *     SPAWN
 *     NEW
 *     APPLY
 *     MEASURE
 *     RESET
 *     BARRIER
 *     ENTANGLE
 *     REMEMBER
 *     RECALL
 *     LEARN
 *     INFER
 *     PERFORM
 *     ZAMANI
 *     SASA
 *     NANO
 *     AGENT
 *
 * The exact lexical spelling remains exclusively owned by the lexer.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every expression recognized here must map to a source-level AST expression.
 *
 * The AST is domain-neutral.
 *
 * Operators MUST NOT be represented in the frontend AST as raw lexer-token
 * implementation types when the canonical frontend AST provides an
 * OperatorRef/source-level operator representation.
 *
 * The AST must preserve:
 *
 *     - source span;
 *     - operator identity;
 *     - operand ordering;
 *     - call argument ordering;
 *     - index ordering;
 *     - member names;
 *     - qualified names;
 *     - literal source meaning;
 *     - syntactic nesting.
 *
 * Quantum-specific operations must remain generic/extensible.
 *
 * The grammar MUST NOT introduce:
 *
 *     enum QuantumGate {
 *         X,
 *         H,
 *         CNOT,
 *         ...
 *     }
 *
 * Instead, a quantum operation may be represented through generic calls,
 * namespaced names, or domain extensions.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing answers:
 *
 *     "Is this source structurally an expression?"
 *
 * Semantic analysis answers:
 *
 *     "What does this expression mean?"
 *
 * Semantic analysis owns:
 *
 *     name resolution
 *     type checking
 *     overload resolution
 *     conversion
 *     ownership
 *     borrowing
 *     effects
 *     capabilities
 *     resources
 *     quantum legality
 *     classical legality
 *     HDL legality
 *     hardware capability requirements
 *     domain interoperability
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing must depend only on:
 *
 *     source token sequence
 *     grammar version
 *
 * Parsing MUST NOT depend on:
 *
 *     system time
 *     randomness
 *     environment variables
 *     hardware discovery
 *     device state
 *     network state
 *     runtime scheduler state
 *
 * ============================================================================
 */

parser grammar Expression;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ========================================================================== */

/**
 * Complete Zamani source-level expression.
 *
 * This is the rule that the rest of the parser should consume.
 */
expression
    : assignmentExpression
    ;


/* ============================================================================
 * 2. ASSIGNMENT
 * ========================================================================== */

/**
 * Assignment is right associative.
 *
 * Examples:
 *
 *     x = y
 *     x += y
 *     a = b = c
 *     object.field = value
 *     array[index] = value
 *
 * The grammar intentionally permits syntactic assignment targets broadly.
 * Semantic analysis determines whether a target is actually assignable.
 */
assignmentExpression
    : conditionalExpression
    | assignmentTarget assignmentOperator assignmentExpression
    ;

assignmentTarget
    : postfixExpression
    ;

assignmentOperator
    : ASSIGN
    | PLUS_ASSIGN
    | MINUS_ASSIGN
    | STAR_ASSIGN
    | SLASH_ASSIGN
    ;


/* ============================================================================
 * 3. CONDITIONAL
 * ========================================================================== */

/**
 * Conditional expression:
 *
 *     condition ? whenTrue : whenFalse
 *
 * The branches consume `expression`, allowing assignments or other
 * expressions in either branch.
 *
 * Example:
 *
 *     flag ? a = b : c
 *
 * Whether such an expression is semantically legal is not a grammar concern.
 */
conditionalExpression
    : rangeExpression
      (
          QUESTION
          expression
          COLON
          expression
      )?
    ;


/* ============================================================================
 * 4. RANGE
 * ========================================================================== */

/**
 * Range syntax:
 *
 *     start .. end
 *     start ..= end
 *
 * A range contains no fixed number of elements.
 *
 * The semantic layer decides:
 *
 *     - whether endpoints are valid;
 *     - whether the range is finite;
 *     - whether the range is lazy;
 *     - whether the range is distributed;
 *     - whether the range can be represented directly by a target.
 */
rangeExpression
    : logicalOrExpression
      (
          rangeOperator
          logicalOrExpression
      )?
    ;

rangeOperator
    : DOT_DOT
    | DOT_DOT_EQ
    ;


/* ============================================================================
 * 5. LOGICAL OR
 * ========================================================================== */

logicalOrExpression
    : logicalAndExpression
      (
          OR_OR
          logicalAndExpression
      )*
    ;


/* ============================================================================
 * 6. LOGICAL AND
 * ========================================================================== */

logicalAndExpression
    : bitwiseOrExpression
      (
          AND_AND
          bitwiseOrExpression
      )*
    ;


/* ============================================================================
 * 7. BITWISE OR
 * ========================================================================== */

bitwiseOrExpression
    : bitwiseXorExpression
      (
          PIPE
          bitwiseXorExpression
      )*
    ;


/* ============================================================================
 * 8. BITWISE XOR
 * ========================================================================== */

bitwiseXorExpression
    : bitwiseAndExpression
      (
          CARET
          bitwiseAndExpression
      )*
    ;


/* ============================================================================
 * 9. BITWISE AND
 * ========================================================================== */

bitwiseAndExpression
    : equalityExpression
      (
          AMPERSAND
          equalityExpression
      )*
    ;


/* ============================================================================
 * 10. EQUALITY
 * ========================================================================== */

/**
 * Equality operators.
 *
 * Chaining remains syntactically representable.
 *
 * Semantic analysis decides whether a chained equality expression is valid
 * for the involved types.
 */
equalityExpression
    : relationalExpression
      (
          equalityOperator
          relationalExpression
      )*
    ;

equalityOperator
    : EQ_EQ
    | NOT_EQ
    ;


/* ============================================================================
 * 11. RELATIONAL
 * ========================================================================== */

/**
 * Relational expressions.
 *
 * The canonical lexer currently provides LE and GE.
 *
 * The single-character `<` and `>` operator spelling must be exposed by the
 * canonical lexer vocabulary before this grammar is generated if the lexer
 * does not already provide them.
 *
 * The canonical target token names for the modular lexer are:
 *
 *     LESS
 *     GREATER
 *     LE
 *     GE
 *
 * `LESS` and `GREATER` are therefore the names consumed here.
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
    | LE
    | GE
    ;


/* ============================================================================
 * 12. SHIFT
 * ========================================================================== */

shiftExpression
    : additiveExpression
      (
          shiftOperator
          additiveExpression
      )*
    ;

shiftOperator
    : SHIFT_LEFT
    | SHIFT_RIGHT
    ;


/* ============================================================================
 * 13. ADDITIVE
 * ========================================================================== */

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


/* ============================================================================
 * 14. MULTIPLICATIVE
 * ========================================================================== */

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
    | PERCENT
    ;


/* ============================================================================
 * 15. PREFIX
 * ========================================================================== */

/**
 * Prefix operators are recursively nestable.
 *
 * Examples:
 *
 *     -x
 *     !!x
 *     ~~x
 *     &x
 *     *x
 *     -~*&x
 *
 * No artificial prefix depth is encoded.
 */
prefixExpression
    : prefixOperator prefixExpression
    | postfixExpression
    ;

prefixOperator
    : PLUS
    | MINUS
    | NOT_OPERATOR
    | TILDE
    | AMPERSAND
    | STAR
    ;


/* ============================================================================
 * 16. POSTFIX
 * ========================================================================== */

/**
 * Postfix expressions permit arbitrary chaining.
 *
 * Examples:
 *
 *     value
 *     value()
 *     value(a, b)
 *     value[index]
 *     value.field
 *     value::member
 *     value()[index].field(arg)
 *
 * The grammar does not impose a maximum chain length.
 */
postfixExpression
    : primaryExpression postfixPart*
    ;

postfixPart
    : callSuffix
    | indexSuffix
    | memberSuffix
    | qualifiedMemberSuffix
    | optionalMemberSuffix
    ;


/* ============================================================================
 * 17. CALLS
 * ========================================================================== */

/**
 * Calls accept zero or more arguments.
 *
 * The argument count is intentionally unbounded by the grammar.
 */
callSuffix
    : LPAREN argumentList? RPAREN
    ;

argumentList
    : argument
      (
          COMMA
          argument
      )*
      COMMA?
    ;

argument
    : namedArgument
    | expression
    ;

namedArgument
    : identifier
      COLON
      expression
    ;


/* ============================================================================
 * 18. INDEXING
 * ========================================================================== */

/**
 * Indexing supports an arbitrary expression as the index.
 *
 * Examples:
 *
 *     a[i]
 *     matrix[i, j]
 *     tensor[i, j, k]
 *     q[logical_index]
 *
 * Multiple indices are represented by the normal expression-list structure.
 */
indexSuffix
    : LBRACKET
      expressionList
      RBRACKET
    ;

expressionList
    : expression
      (
          COMMA
          expression
      )*
      COMMA?
    ;


/* ============================================================================
 * 19. MEMBER ACCESS
 * ========================================================================== */

memberSuffix
    : DOT identifier
    ;

qualifiedMemberSuffix
    : DOUBLE_COLON identifier
    ;


/* ============================================================================
 * 20. OPTIONAL / NULL-PROPAGATING ACCESS
 * ========================================================================== */

/**
 * The canonical lexer must expose QUESTION_DOT if Zamani adopts `?.` as a
 * single lexical token.
 *
 * This grammar intentionally does not reconstruct `?.` from QUESTION + DOT.
 *
 * Until QUESTION_DOT is part of the canonical lexer vocabulary, optional
 * member access remains outside this rule and must not be silently simulated.
 */
optionalMemberSuffix
    : QUESTION_DOT identifier
    ;


/* ============================================================================
 * 21. PRIMARY EXPRESSIONS
 * ========================================================================== */

primaryExpression
    : literalExpression
    | identifierExpression
    | qualifiedNameExpression
    | parenthesizedExpression
    | tupleExpression
    | arrayExpression
    | objectExpression
    | lambdaExpression
    | ifExpression
    | matchExpression
    | loopExpression
    | asyncExpression
    | awaitExpression
    | spawnExpression
    | newExpression
    | quantumExpression
    | nanoExpression
    | sankofaExpression
    | compileTimeExpression
    | resourceExpression
    | blockExpression
    ;


/* ============================================================================
 * 22. IDENTIFIERS
 * ========================================================================== */

identifierExpression
    : identifier
    ;

identifier
    : IDENTIFIER
    ;


/* ============================================================================
 * 23. QUALIFIED NAMES
 * ========================================================================== */

/**
 * Qualified names support arbitrary namespace depth.
 *
 * Examples:
 *
 *     math
 *     math::linear
 *     quantum::algorithm::phase
 *     hardware::resource::capability
 *
 * No fixed namespace depth is encoded.
 */
qualifiedNameExpression
    : identifier
      (
          DOUBLE_COLON
          identifier
      )+
    ;


/* ============================================================================
 * 24. LITERALS
 * ========================================================================== */

/**
 * Literal syntax is composed from the canonical lexer.
 *
 * Numeric representation is intentionally not narrowed to a machine width
 * here.
 */
literalExpression
    : INTEGER
    | FLOAT
    | STRING
    | CHAR
    | TRUE
    | FALSE
    | NIL
    | NULL
    | QUANTUM_LITERAL
    | MTS_LITERAL
    ;


/* ============================================================================
 * 25. PARENTHESIZED EXPRESSIONS
 * ========================================================================== */

parenthesizedExpression
    : LPAREN
      expression
      RPAREN
    ;


/* ============================================================================
 * 26. TUPLES
 * ========================================================================== */

/**
 * A tuple is distinguished from an ordinary parenthesized expression by the
 * presence of a comma.
 */
tupleExpression
    : LPAREN
      expression
      COMMA
      tupleTail
      RPAREN
    ;

tupleTail
    : expression
      (
          COMMA
          expression
      )*
      COMMA?
    ;


/* ============================================================================
 * 27. ARRAYS / COLLECTION EXPRESSIONS
 * ========================================================================== */

/**
 * Array literal:
 *
 *     []
 *     [a]
 *     [a, b, c]
 *
 * No fixed element count is encoded.
 */
arrayExpression
    : LBRACKET
      expressionList?
      RBRACKET
    ;


/**
 * Object/record-like expression.
 *
 * Semantic analysis determines the actual type and whether the construction
 * corresponds to a record, struct, map-like value, domain extension, etc.
 */
objectExpression
    : LBRACE
      objectFieldList?
      RBRACE
    ;

objectFieldList
    : objectField
      (
          COMMA
          objectField
      )*
      COMMA?
    ;

objectField
    : identifier
      COLON
      expression
    | identifier
    ;


/* ============================================================================
 * 28. LAMBDAS / CLOSURES
 * ========================================================================== */

/**
 * Pipe-delimited lambda:
 *
 *     |x| x + 1
 *     |x, y| x + y
 *
 * An optional return type may be supplied.
 */
lambdaExpression
    : PIPE
      lambdaParameterList?
      PIPE
      (
          ARROW
          typeExpression
      )?
      lambdaBody
    ;

lambdaParameterList
    : lambdaParameter
      (
          COMMA
          lambdaParameter
      )*
      COMMA?
    ;

lambdaParameter
    : identifier
      (
          COLON
          typeExpression
      )?
    ;

lambdaBody
    : blockExpression
    | expression
    ;


/**
 * Explicit function-expression form.
 *
 * This remains source syntax and does not imply a particular ABI.
 */
functionExpression
    : FN
      genericParameterList?
      LPAREN
      parameterList?
      RPAREN
      (
          ARROW
          typeExpression
      )?
      blockExpression
    ;


/* ============================================================================
 * 29. CONDITIONAL EXPRESSIONS
 * ========================================================================== */

ifExpression
    : IF
      expression
      blockExpression
      (
          ELSE
          (
              ifExpression
            | blockExpression
          )
      )?
    ;


/* ============================================================================
 * 30. MATCH EXPRESSIONS
 * ========================================================================== */

matchExpression
    : MATCH
      expression
      LBRACE
      matchArm+
      RBRACE
    ;

matchArm
    : pattern
      matchGuard?
      FAT_ARROW
      (
          expression
        | blockExpression
      )
      COMMA?
    ;

matchGuard
    : WHEN expression
    ;


/* ============================================================================
 * 31. LOOP EXPRESSIONS
 * ========================================================================== */

loopExpression
    : WHILE
      expression
      blockExpression
    | FOR
      pattern
      IN
      expression
      blockExpression
    | LOOP
      blockExpression
    ;


/* ============================================================================
 * 32. ASYNC / AWAIT / SPAWN
 * ========================================================================== */

asyncExpression
    : ASYNC
      (
          blockExpression
        | expression
      )
    ;

awaitExpression
    : AWAIT
      expression
    ;

spawnExpression
    : SPAWN
      expression
    ;


/* ============================================================================
 * 33. OBJECT CONSTRUCTION
 * ========================================================================== */

newExpression
    : NEW
      typeExpression
      (
          LPAREN
          argumentList?
          RPAREN
      )?
    ;


/* ============================================================================
 * 34. QUANTUM EXPRESSIONS
 * ========================================================================== */

/**
 * Quantum syntax remains generic.
 *
 * No finite gate catalogue is embedded here.
 *
 * Examples representable through the generic language include:
 *
 *     quantum::operation(...)
 *     quantum::algorithm(...)
 *     measure(...)
 *     entangle(...)
 *
 * A source-level quantum operation does NOT identify:
 *
 *     physical qubit
 *     physical device
 *     topology
 *     calibration
 *     native gate set
 *     routing
 *     schedule
 *
 * Those are downstream concerns.
 */
quantumExpression
    : ENTANGLE
      LPAREN
      expression
      COMMA
      expression
      RPAREN
    | MEASURE
      expression
    | RESET
      expression
    | BARRIER
      expressionList?
    ;


/**
 * Generic quantum operation invocation remains an ordinary call expression.
 *
 * This rule is intentionally provided as a semantic boundary rather than an
 * exhaustive gate grammar.
 *
 * Example:
 *
 *     quantum::H(q)
 *     quantum::CNOT(control, target)
 *     vendor::operation(parameter, target)
 *
 * Operation names remain identifiers.
 */
quantumOperationExpression
    : qualifiedNameExpression
      callSuffix
    ;


/* ============================================================================
 * 35. NANO / AGENT EXPRESSIONS
 * ========================================================================== */

nanoExpression
    : NANO
      expression
    | AGENT
      expression
    ;


/* ============================================================================
 * 36. SANKOFA / TEMPORAL EXPRESSIONS
 * ========================================================================== */

sankofaExpression
    : REMEMBER
      expression
    | RECALL
      expression
    | LEARN
      expression
    | INFER
      expression
    | PERFORM
      expression
    | ZAMANI
      expression
    | SASA
      expression
    ;


/* ============================================================================
 * 37. BLOCK EXPRESSIONS
 * ========================================================================== */

/**
 * A block expression delegates statement syntax to the canonical statement
 * grammar.
 *
 * This file intentionally does not define statements a second time.
 *
 * Integration contract:
 *
 *     blockExpression
 *
 * must be supplied by the canonical statement/core composition layer.
 *
 * A temporary parser composition may map it to:
 *
 *     LBRACE statement* RBRACE
 *
 * but the final architecture should import/use the canonical block rule.
 */
blockExpression
    : LBRACE expressionBlockItem* RBRACE
    ;

expressionBlockItem
    : expressionStatement
    | declarationStatement
    ;

expressionStatement
    : expression
      SEMICOLON?
    ;

declarationStatement
    : LET
      pattern
      (
          COLON
          typeExpression
      )?
      (
          ASSIGN
          expression
      )?
      SEMICOLON?
    ;


/* ============================================================================
 * 38. COMPILE-TIME EXPRESSIONS
 * ========================================================================== */

/**
 * Compile-time computation is represented structurally.
 *
 * It does not permit the grammar to execute compiler code.
 *
 * Compiler evaluation, constant folding, macro expansion, and compile-time
 * resource analysis remain downstream.
 */
compileTimeExpression
    : SIZEOF
      LPAREN
      typeExpression
      RPAREN
    ;


/* ============================================================================
 * 39. RESOURCE / CAPABILITY EXPRESSIONS
 * ========================================================================== */

/**
 * Resource and capability concepts are represented through ordinary calls and
 * qualified names.
 *
 * The grammar does not know whether a named capability exists.
 *
 * Examples:
 *
 *     capability(...)
 *     resource(...)
 *     hardware::capability(...)
 *
 * are ordinary source expressions unless a higher-level grammar reserves
 * specific syntax.
 *
 * This keeps resource negotiation separate from expression parsing.
 */
resourceExpression
    : identifier
      callSuffix
    ;


/* ============================================================================
 * 40. PATTERNS
 * ========================================================================== */

/**
 * Expression grammar needs pattern syntax for match/loop constructs.
 *
 * Pattern semantics belong to the pattern/type system.
 */
pattern
    : wildcardPattern
    | identifierPattern
    | literalPattern
    | tuplePattern
    | arrayPattern
    | qualifiedPattern
    | rangePatternPattern
    | referencePattern
    | parenthesizedPattern
    ;

wildcardPattern
    : UNDERSCORE
    ;

identifierPattern
    : identifier
    ;

literalPattern
    : literalExpression
    ;

tuplePattern
    : LPAREN
      pattern
      COMMA
      patternListTail
      RPAREN
    ;

patternListTail
    : pattern
      (
          COMMA
          pattern
      )*
      COMMA?
    ;

arrayPattern
    : LBRACKET
      patternList?
      RBRACKET
    ;

qualifiedPattern
    : qualifiedNameExpression
    ;

rangePatternPattern
    : literalPattern
      rangeOperator
      literalPattern
    ;

referencePattern
    : AMPERSAND
      pattern
    ;

parenthesizedPattern
    : LPAREN
      pattern
      RPAREN
    ;

patternList
    : pattern
      (
          COMMA
          pattern
      )*
      COMMA?
    ;


/* ============================================================================
 * 41. TYPE EXPRESSION INTEGRATION CONTRACT
 * ========================================================================== */

/**
 * Expressions may consume type syntax in:
 *
 *     casts
 *     constructors
 *     lambda return types
 *     compile-time queries
 *     generic expressions
 *
 * The canonical type grammar owns `typeExpression`.
 *
 * This local rule is an integration placeholder only.
 *
 * FINAL COMPOSITION REQUIREMENT:
 *
 *     typeExpression
 *
 * must be supplied by the canonical type grammar rather than maintained as a
 * second type system here.
 *
 * Until the parser composition layer imports the canonical type grammar,
 * the minimal source-level type boundary is:
 */
typeExpression
    : qualifiedTypeName
    | parenthesizedType
    ;

qualifiedTypeName
    : identifier
      (
          DOUBLE_COLON
          identifier
      )*
      genericTypeArguments?
    ;

genericTypeArguments
    : LT
      typeArgument
      (
          COMMA
          typeArgument
      )*
      COMMA?
      GT
    ;

typeArgument
    : typeExpression
    | expression
    ;

parenthesizedType
    : LPAREN
      typeExpression
      RPAREN
    ;


/* ============================================================================
 * 42. GENERIC PARAMETERS
 * ========================================================================== */

genericParameterList
    : LT
      genericParameter
      (
          COMMA
          genericParameter
      )*
      COMMA?
      GT
    ;

genericParameter
    : identifier
    ;


/* ============================================================================
 * 43. PARAMETERS
 * ========================================================================== */

parameterList
    : parameter
      (
          COMMA
          parameter
      )*
      COMMA?
    ;

parameter
    : MUT?
      identifier
      (
          COLON
          typeExpression
      )?
      (
          ASSIGN
          expression
      )?
    ;


/* ============================================================================
 * 44. INTEGRATION CONTRACTS
 * ========================================================================== */

/*
 * FRONTEND AST
 *
 * The parser must lower these syntactic categories into the existing
 * domain-neutral frontend AST.
 *
 * Recommended mapping:
 *
 *     identifierExpression       -> Identifier
 *     literalExpression          -> Literal
 *     unary                      -> Unary
 *     binary                     -> Binary
 *     assignment                 -> Assignment
 *     conditional                -> Conditional
 *     call                       -> Call
 *     index                      -> Index
 *     member                     -> MemberAccess
 *     range                      -> Range
 *     lambda                     -> Lambda
 *     block                      -> Block
 *     match                      -> Match
 *     async                      -> Async
 *     await                      -> Await
 *     spawn                      -> Spawn
 *     cast                       -> Cast
 *     domain-specific forms      -> Extension
 *
 * The parser must not lower directly into:
 *
 *     quantum::ir
 *     classical IR
 *     HDL IR
 *     LLVM
 *     QIR
 *     MLIR
 *     vendor IR
 *
 * ============================================================================
 *
 * QUANTUM INTEGRATION
 *
 * Quantum source expressions must remain target-independent.
 *
 * Example:
 *
 *     quantum::H(q)
 *
 * is source syntax.
 *
 * Semantic analysis decides that the operation represents a quantum operation.
 *
 * Later:
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
 *     QEC / resilience / ZQN
 *          |
 *          v
 *     HAL
 *          |
 *          v
 *     target hardware
 *
 * No physical qubit identifier belongs in this grammar.
 *
 * ============================================================================
 *
 * CLASSICAL INTEGRATION
 *
 * Arithmetic, logical, vector, matrix, tensor and symbolic expressions are
 * syntax-level constructs.
 *
 * The grammar does not enumerate mathematical libraries.
 *
 * For example:
 *
 *     fft(x)
 *     svd(matrix)
 *     gradient(model)
 *     solve(system)
 *
 * remain ordinary calls unless a language-level semantic capability explicitly
 * requires dedicated syntax.
 *
 * This prevents the grammar from becoming an unbounded list of library names.
 *
 * ============================================================================
 *
 * HDL / HARDWARE INTEGRATION
 *
 * Expressions may occur inside:
 *
 *     signal assignments
 *     combinational logic
 *     sequential logic
 *     timing expressions
 *     constraints
 *     parameter declarations
 *     generate constructs
 *     hardware/software co-design
 *
 * The expression grammar does not decide whether an identifier denotes:
 *
 *     software variable
 *     hardware signal
 *     register
 *     port
 *     memory
 *     device capability
 *
 * Semantic/domain analysis decides that.
 *
 * ============================================================================
 *
 * RESOURCE INTEGRATION
 *
 * Expressions may describe resource requirements:
 *
 *     requires(...)
 *     capability(...)
 *     resource(...)
 *     constraint(...)
 *
 * Such expressions remain semantic data.
 *
 * They do NOT imply:
 *
 *     GPU #0
 *     QPU #3
 *     physical qubit 17
 *     CPU core 7
 *     node 12
 *
 * unless an explicitly target-specific downstream layer introduces that
 * information.
 *
 * ============================================================================
 *
 * DISTRIBUTED INTEGRATION
 *
 * Calls, member access, indexing and ordinary expressions are location-neutral.
 *
 * Distribution, placement, replication, consistency, communication and
 * scheduling are semantic/runtime concerns.
 *
 * No expression syntax assumes:
 *
 *     a fixed number of nodes;
 *     a fixed number of processes;
 *     a fixed topology;
 *     a fixed network width.
 *
 * ============================================================================
 *
 * EFFECTS / OWNERSHIP / CAPABILITIES
 *
 * Expression syntax does not decide whether an operation:
 *
 *     mutates;
 *     allocates;
 *     blocks;
 *     communicates;
 *     measures;
 *     consumes a linear resource;
 *     requires a capability;
 *     has an observable effect.
 *
 * Those properties are resolved by semantic analysis.
 *
 * ============================================================================
 *
 * SOURCE SPANS
 *
 * Every parser-created AST expression must retain the source span covering
 * the complete syntactic construct.
 *
 * Child expressions retain their own spans.
 *
 * The grammar itself performs no source-location manipulation.
 *
 * ============================================================================
 *
 * ERROR RECOVERY
 *
 * Syntax errors are parser errors.
 *
 * Examples:
 *
 *     x =
 *     x + *
 *     f(
 *     a ? b
 *     [a,
 *     object.
 *
 * Semantic errors are NOT parser errors.
 *
 * Examples:
 *
 *     assigning to immutable value
 *     incompatible types
 *     unavailable capability
 *     insufficient resources
 *     illegal quantum operation
 *     invalid hardware target
 *
 * ============================================================================
 *
 * SCALABILITY
 *
 * The following are intentionally unbounded by the grammar:
 *
 *     postfix chains
 *     binary chains
 *     prefix chains
 *     arguments
 *     tuple elements
 *     array elements
 *     object fields
 *     namespace depth
 *     index expressions
 *     generic arguments
 *     nested expressions
 *
 * Actual parser stack/resource policy belongs outside the language contract.
 *
 * ============================================================================
 *
 * COMPATIBILITY
 *
 * Existing source forms that use the canonical tokens should preserve their
 * meaning when this grammar replaces the duplicated expression hierarchy.
 *
 * Compatibility work must include:
 *
 *     grammar/antlr/Core.g4
 *     grammar/Zamani.g4
 *     grammar/expressions/expressions.g4
 *     grammar/expressions/*.g4
 *     src/lexer.rs
 *     src/parser.rs
 *     src/frontend/ast/
 *     grammar/tests/
 *
 * The replacement must be performed as a coordinated migration, not by
 * maintaining two independent expression grammars indefinitely.
 *
 * ============================================================================
 *
 * HARD-CODING AUDIT
 *
 * This file contains:
 *
 *     no qubit limit
 *     no CPU limit
 *     no GPU limit
 *     no FPGA limit
 *     no node limit
 *     no memory limit
 *     no tensor-rank limit
 *     no vector-width limit
 *     no argument-count limit
 *     no namespace-depth limit
 *     no expression-depth constant
 *     no device identifiers
 *     no physical topology
 *     no backend names
 *     no vendor gate catalogue
 *
 * Literal program values remain legal because they are program semantics,
 * not universal implementation limits.
 *
 * ============================================================================
 *
 * SECURITY
 *
 * This grammar:
 *
 *     - contains no embedded target-language actions;
 *     - performs no I/O;
 *     - executes no user code;
 *     - accesses no environment;
 *     - accesses no network;
 *     - contains no unsafe implementation;
 *     - does not select hardware;
 *     - does not allocate runtime resources.
 *
 * ============================================================================
 *
 * COMPLETION CRITERIA
 *
 * This file is complete only when:
 *
 * [ ] canonical lexer token vocabulary is synchronized;
 * [ ] LESS and GREATER exist in ZamaniLexer;
 * [ ] QUESTION_DOT exists if optional member access is accepted;
 * [ ] LT and GT exist if generic type syntax is accepted;
 * [ ] expression is the sole public expression entry point;
 * [ ] assignment is right associative;
 * [ ] conditional precedence is deterministic;
 * [ ] range precedence is deterministic;
 * [ ] logical precedence is deterministic;
 * [ ] bitwise precedence is deterministic;
 * [ ] comparison precedence is deterministic;
 * [ ] shift precedence is deterministic;
 * [ ] arithmetic precedence is deterministic;
 * [ ] prefix operators are recursively composable;
 * [ ] postfix chains are arbitrarily repeatable;
 * [ ] calls are arbitrarily repeatable;
 * [ ] indexing is arbitrarily repeatable;
 * [ ] member access is arbitrarily repeatable;
 * [ ] qualified names have no artificial depth;
 * [ ] collection cardinalities have no grammar-level ceiling;
 * [ ] no closed quantum-gate list exists;
 * [ ] no hardware limit is encoded;
 * [ ] AST mapping is defined;
 * [ ] semantic mapping is defined;
 * [ ] quantum::ir remains downstream;
 * [ ] positive tests exist;
 * [ ] negative tests exist;
 * [ ] boundary tests exist;
 * [ ] scalability tests exist;
 * [ ] compatibility tests exist;
 * [ ] deterministic parsing tests exist;
 * [ ] Rust frontend integration is verified on Rust 1.97/1.97.1;
 * [ ] compiler remains safe Rust with no unsafe implementation.
 */
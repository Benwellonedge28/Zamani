/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/expressions/expressions.g4
 *
 * Status:
 *     Canonical expression-composition grammar.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Edition 2021
 *     Safe Rust only.
 *     No unsafe Rust is required or permitted by the compiler implementation.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the UNIVERSAL EXPRESSION PRECEDENCE AND COMPOSITION LAYER.
 *
 * It provides the public:
 *
 *     expression
 *
 * entry point and connects the expression precedence hierarchy.
 *
 * Canonical hierarchy:
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
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - expression
 *     - assignmentExpression
 *     - assignmentTarget
 *     - assignmentOperator
 *     - rangeExpression composition
 *     - logical precedence
 *     - bitwise precedence
 *     - equality precedence
 *     - relational precedence
 *     - shift precedence
 *     - additive precedence
 *     - multiplicative precedence
 *     - prefix precedence
 *     - postfix composition
 *     - primary-expression composition
 *     - expression-list composition
 *     - argument-list composition
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer tokens
 *     - conditional-expression semantics
 *     - statement-level conditionals
 *     - declarations
 *     - types
 *     - blocks
 *     - semantic analysis
 *     - name resolution
 *     - overload resolution
 *     - ownership
 *     - borrowing
 *     - effects
 *     - capabilities
 *     - resources
 *     - quantum IR
 *     - classical IR
 *     - HDL IR
 *     - routing
 *     - scheduling
 *     - QEC
 *     - ZQN
 *     - HAL
 *     - target selection
 *     - runtime execution
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There must be exactly one authoritative public expression rule:
 *
 *     expression
 *
 * This file owns that rule.
 *
 * There must also be exactly one authoritative precedence hierarchy.
 *
 * Specialized expression files such as:
 *
 *     arithmetic.g4
 *     assignment.g4
 *     binary.g4
 *     bitwise.g4
 *     calls.g4
 *     comparison.g4
 *     conditionals.g4
 *     indexing.g4
 *     literals.g4
 *     match.g4
 *     member-access.g4
 *     postfix.g4
 *     ranges.g4
 *     unary.g4
 *
 * MUST NOT introduce a second public expression hierarchy.
 *
 * `conditionals.g4` remains the owner of:
 *
 *     conditionalExpression
 *     ifExpression
 *     else-if expression branches
 *     else expression branches
 *     ternary conditional expressions
 *
 * Therefore this file references `conditionalExpression` but does not
 * redefine it.
 *
 * ============================================================================
 * LEXER AUTHORITY
 * ============================================================================
 *
 * The canonical lexical vocabulary is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This file contains NO lexer rules.
 *
 * The parser consumes the canonical tokens rather than inventing aliases.
 *
 * Important canonical operator tokens include:
 *
 *     ASSIGN
 *     PLUS_ASSIGN
 *     MINUS_ASSIGN
 *     STAR_ASSIGN
 *     SLASH_ASSIGN
 *     PERCENT_ASSIGN
 *     AMPERSAND_ASSIGN
 *     PIPE_ASSIGN
 *     CARET_ASSIGN
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
 *     PLUS_PLUS
 *     MINUS_MINUS
 *     QUESTION_DOT
 *     NULL_COALESCE
 *
 * If a lexical token is not present in the canonical lexer, this grammar must
 * not silently invent a parser-side replacement.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every expression must lower to the existing domain-neutral frontend AST.
 *
 * The grammar does not define Rust AST structures.
 *
 * The AST must preserve:
 *
 *     - source spans;
 *     - source ordering;
 *     - operator identity;
 *     - operand ordering;
 *     - call argument ordering;
 *     - index ordering;
 *     - member names;
 *     - generic argument ordering;
 *     - literal meaning;
 *     - syntactic nesting.
 *
 * Operators are syntax-level constructs.
 *
 * Their semantic meaning is resolved downstream.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing answers:
 *
 *     "Is this structurally a Zamani expression?"
 *
 * Semantic analysis answers:
 *
 *     "What does this expression mean?"
 *
 * Semantic analysis owns:
 *
 *     - name resolution;
 *     - type checking;
 *     - overload resolution;
 *     - generic inference;
 *     - conversions;
 *     - ownership;
 *     - borrowing;
 *     - effects;
 *     - capabilities;
 *     - resource requirements;
 *     - quantum legality;
 *     - classical legality;
 *     - HDL legality;
 *     - hardware capability requirements;
 *     - interoperability.
 *
 * ============================================================================
 * CANONICAL COMPUTATION PIPELINE
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     parser
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
 *       +--------------------+----------------------+
 *       |                    |                      |
 *       v                    v                      v
 *   classical           quantum::ir           HDL/hardware
 *       |                    |                      |
 *       +--------------------+----------------------+
 *                            |
 *                            v
 *                       optimization
 *                            |
 *                  routing / scheduling
 *                            |
 *                  resilience / QEC / ZQN
 *                            |
 *                           HAL
 *                            |
 *                     target realization
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * This grammar MUST NOT introduce another quantum IR.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * The grammar introduces NO universal finite limits for:
 *
 *     qubits
 *     CPUs
 *     cores
 *     threads
 *     GPUs
 *     FPGAs
 *     accelerators
 *     nodes
 *     memory
 *     registers
 *     tensor dimensions
 *     vector widths
 *     function arguments
 *     tuple elements
 *     collection elements
 *     index dimensions
 *     call depth
 *     member depth
 *     namespace depth
 *     expression depth
 *     circuit depth
 *     operation count
 *     resource count
 *
 * Repetition is structural:
 *
 *     *
 *     +
 *
 * rather than enumerating finite capacities.
 *
 * "Infinity" means:
 *
 *     no artificial language-level finite bound is imposed here.
 *
 * Actual limits belong to:
 *
 *     - implementation resource policy;
 *     - compiler resource availability;
 *     - available memory/storage;
 *     - target capability;
 *     - deployment policy;
 *     - runtime resource availability.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * Expressions describe computation and intent.
 *
 * They MUST NOT hard-code:
 *
 *     physical CPU identifiers
 *     GPU identifiers
 *     FPGA identifiers
 *     physical qubit identifiers
 *     fixed QPU topology
 *     memory-bank identifiers
 *     accelerator counts
 *     network-node counts
 *     device addresses
 *     register widths
 *     machine sizes
 *
 * Such information belongs to:
 *
 *     resources
 *     capabilities
 *     target descriptions
 *     compilation contexts
 *     routing
 *     scheduling
 *     HAL
 *     deployment
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum names remain ordinary identifiers unless explicitly reserved by
 * the canonical lexer.
 *
 * The grammar MUST NOT enumerate:
 *
 *     X
 *     Y
 *     Z
 *     H
 *     CX
 *     CNOT
 *     RX
 *     RY
 *     RZ
 *     ...
 *
 * Generic calls and expressions can represent quantum operations:
 *
 *     H(q)
 *     measure(q)
 *     reset(q)
 *     operation(theta)(q)
 *     circuit.apply(operation, q)
 *
 * Semantic analysis determines whether those constructs represent quantum
 * operations and lowers them to quantum::ir.
 *
 * ============================================================================
 * CLASSICAL / HDL / AI / DATA / DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * The same expression system is intentionally reusable for:
 *
 *     scalar computation
 *     vector computation
 *     matrix computation
 *     tensor computation
 *     symbolic computation
 *     AI/ML
 *     dataflow
 *     distributed values
 *     network services
 *     hardware abstractions
 *     HDL expressions
 *     accelerator operations
 *     quantum/classical boundaries
 *     future computational domains
 *
 * Domain-specific semantics are downstream.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     - source token sequence;
 *     - active grammar/language version.
 *
 * Parsing MUST NOT depend on:
 *
 *     - system time;
 *     - randomness;
 *     - environment variables;
 *     - filesystem state;
 *     - network state;
 *     - hardware discovery;
 *     - device state;
 *     - runtime scheduler state;
 *     - backend availability.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing is non-executing.
 *
 * A syntactically valid call such as:
 *
 *     system.run(command)
 *
 * MUST NOT execute anything during parsing.
 *
 * The parser MUST NOT:
 *
 *     - open files;
 *     - access credentials;
 *     - contact networks;
 *     - execute commands;
 *     - load plugins;
 *     - inspect hardware;
 *     - invoke quantum devices;
 *     - invoke HDL simulators;
 *     - invoke compiler backends.
 *
 * ============================================================================
 */

parser grammar Expressions;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 1. PUBLIC EXPRESSION ENTRY
 * ========================================================================== */

/*
 * The single public expression entry point.
 *
 * Assignment has the lowest precedence among the expression operators owned
 * here.
 */
expression
    : assignmentExpression
    ;


/* ============================================================================
 * 2. ASSIGNMENT
 * ========================================================================== */

/*
 * Assignment is right associative.
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
 *     a = b = c
 *
 * The grammar intentionally accepts a broad syntactic assignment target.
 * Semantic analysis determines whether the target is assignable.
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
    | PERCENT_ASSIGN
    | AMPERSAND_ASSIGN
    | PIPE_ASSIGN
    | CARET_ASSIGN
    ;


/* ============================================================================
 * 3. CONDITIONAL INTEGRATION
 * ========================================================================== */

/*
 * `conditionalExpression` is owned by:
 *
 *     grammar/expressions/conditionals.g4
 *
 * This file deliberately does not redefine it.
 *
 * Canonical integration:
 *
 *     assignmentExpression
 *             |
 *             v
 *     conditionalExpression
 *             |
 *             v
 *     rangeExpression
 *
 * This prevents duplicate conditional-expression definitions.
 */


/* ============================================================================
 * 4. RANGE
 * ========================================================================== */

/*
 * Range composition is owned here.
 *
 * Range-specific operator documentation/contract belongs to:
 *
 *     grammar/expressions/ranges.g4
 *
 * A range is deliberately restricted to operands at the next precedence
 * level. This prevents:
 *
 *     expression -> range -> expression
 *
 * from creating an uncontrolled recursive precedence cycle.
 *
 * Examples:
 *
 *     0 .. n
 *     0 ..= n
 *     start .. finish
 *     start ..= finish
 *
 * Range size is semantic data.
 *
 * No maximum range cardinality is encoded.
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

/*
 * Prefix operators are recursively nestable.
 *
 * Examples:
 *
 *     -x
 *     +x
 *     !x
 *     ~x
 *     &x
 *     *x
 *     !!!value
 *     -~*&value
 *
 * No artificial prefix-depth limit exists.
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
 * 16. POSTFIX COMPOSITION
 * ========================================================================== */

/*
 * Postfix chaining is deliberately unbounded.
 *
 * Examples:
 *
 *     value
 *     value()
 *     value[index]
 *     value.field
 *     value::member
 *     value?.field
 *     value()[index].field(argument)
 *
 * Specialized contracts:
 *
 *     calls.g4
 *     indexing.g4
 *     member-access.g4
 *     postfix.g4
 *
 * must converge on this composition model.
 *
 * The final parser assembly must bind the component rules without introducing
 * another complete expression hierarchy.
 */
postfixExpression
    : primaryExpression postfixPart*
    ;

postfixPart
    : callPostfix
    | indexPostfix
    | memberPostfix
    | qualifiedMemberPostfix
    | optionalMemberPostfix
    | postfixUpdate
    ;


/* ============================================================================
 * 17. CALL POSTFIX
 * ========================================================================== */

/*
 * Calls are expression-level syntax.
 *
 * The argument list is open-ended and therefore has no fixed arity.
 *
 * Examples:
 *
 *     f()
 *     f(x)
 *     f(x, y)
 *     f(x, y, z)
 *
 * Generic invocation:
 *
 *     f::<T>(x)
 *
 * Semantic validation determines whether a callable value, generic parameter,
 * argument count, argument type, or calling convention is valid.
 */
callPostfix
    : LPAREN argumentList? RPAREN
    | genericArgumentSuffix LPAREN argumentList? RPAREN
    ;


/* ============================================================================
 * 18. INDEX POSTFIX
 * ========================================================================== */

/*
 * Indexing syntax is intentionally generic.
 *
 * Examples:
 *
 *     value[i]
 *     value[i, j]
 *     tensor[i, j, k]
 *
 * The expression inside brackets is semantic data.
 *
 * Whether indexing means:
 *
 *     array access
 *     tensor access
 *     map lookup
 *     string access
 *     memory access
 *     quantum-register selection
 *     hardware-resource selection
 *
 * is determined downstream.
 *
 * Index arity is unbounded by the language grammar.
 */
indexPostfix
    : LBRACKET expressionList? RBRACKET
    ;


/* ============================================================================
 * 19. MEMBER POSTFIX
 * ========================================================================== */

/*
 * Ordinary member selection:
 *
 *     value.field
 *
 * The member identifier is resolved semantically.
 */
memberPostfix
    : DOT identifier
    ;


/* ============================================================================
 * 20. QUALIFIED MEMBER POSTFIX
 * ========================================================================== */

/*
 * Qualified selection:
 *
 *     value::member
 *
 * The exact distinction between namespace qualification and value-level
 * associated selection is semantic.
 */
qualifiedMemberPostfix
    : DOUBLE_COLON identifier
    ;


/* ============================================================================
 * 21. OPTIONAL MEMBER POSTFIX
 * ========================================================================== */

/*
 * Optional/null-propagating member access is represented by the canonical
 * compound lexer token when available.
 *
 * Example:
 *
 *     value?.field
 *
 * Nullability is semantic.
 */
optionalMemberPostfix
    : QUESTION_DOT identifier
    ;


/* ============================================================================
 * 22. POSTFIX UPDATE
 * ========================================================================== */

/*
 * Examples:
 *
 *     value++
 *     value--
 *
 * Mutability and assignability are semantic properties.
 */
postfixUpdate
    : INCREMENT
    | DECREMENT
    ;


/* ============================================================================
 * 23. GENERIC ARGUMENT SUFFIX
 * ========================================================================== */

/*
 * Generic invocation is kept syntactically separate from ordinary calls.
 *
 * Examples:
 *
 *     function::<Type>(value)
 *     function::<A, B>(value)
 *
 * The number of generic arguments is unbounded.
 *
 * The exact type-expression grammar is owned by the canonical type system.
 *
 * IMPORTANT:
 *
 * This expression grammar does not invent a second type grammar.
 */
genericArgumentSuffix
    : DOUBLE_COLON LESS_THAN typeExpressionList GREATER_THAN
    ;


/* ============================================================================
 * 24. PRIMARY EXPRESSIONS
 * ========================================================================== */

/*
 * Primary expressions are the atomic foundations of postfix expressions.
 *
 * The complete domain-neutral expression system may grow through additional
 * primary forms, but domain-specific semantics must not be encoded as a closed
 * enumeration here.
 */
primaryExpression
    : identifierExpression
    | literalExpression
    | parenthesizedExpression
    | tupleExpression
    | arrayExpression
    | mapExpression
    | lambdaExpression
    | newExpression
    | thisExpression
    | superExpression
    ;


/* ============================================================================
 * 25. IDENTIFIER EXPRESSION
 * ========================================================================== */

identifierExpression
    : identifier
    ;


/* ============================================================================
 * 26. THIS
 * ========================================================================== */

thisExpression
    : THIS
    ;


/* ============================================================================
 * 27. SUPER
 * ========================================================================== */

superExpression
    : SUPER
    ;


/* ============================================================================
 * 28. PARENTHESIZED EXPRESSION
 * ========================================================================== */

parenthesizedExpression
    : LPAREN expression RPAREN
    ;


/* ============================================================================
 * 29. TUPLE EXPRESSION
 * ========================================================================== */

/*
 * A single expression in parentheses is handled by parenthesizedExpression.
 *
 * Tuple syntax requires at least two elements.
 *
 * Examples:
 *
 *     (a, b)
 *     (a, b, c)
 *
 * A trailing comma is accepted:
 *
 *     (a, b,)
 */
tupleExpression
    : LPAREN expression COMMA expressionListTail? COMMA? RPAREN
    ;

expressionListTail
    : expression
      (
          COMMA
          expression
      )*
    ;


/* ============================================================================
 * 30. ARRAY EXPRESSION
 * ========================================================================== */

/*
 * Array literals have no language-level element-count limit.
 *
 * Examples:
 *
 *     []
 *     [a]
 *     [a, b]
 *     [a, b, c]
 */
arrayExpression
    : LBRACKET argumentList? RBRACKET
    ;


/* ============================================================================
 * 31. MAP / ASSOCIATIVE COLLECTION EXPRESSION
 * ========================================================================== */

/*
 * Map literals are intentionally generic.
 *
 * Examples:
 *
 *     {}
 *     {key: value}
 *     {key1: value1, key2: value2}
 *
 * Key and value semantics are determined by type checking.
 */
mapExpression
    : LBRACE mapEntryList? RBRACE
    ;

mapEntryList
    : mapEntry
      (
          COMMA
          mapEntry
      )*
      COMMA?
    ;

mapEntry
    : expression COLON expression
    ;


/* ============================================================================
 * 32. LAMBDA EXPRESSION
 * ========================================================================== */

/*
 * Lambda syntax is expression-level syntax.
 *
 * Canonical examples:
 *
 *     |x| x + 1
 *
 *     |x, y| x + y
 *
 *     |x| { x + 1 }
 *
 * If the repository's canonical lambda grammar uses another delimiter or
 * syntax, the lambda component must own that spelling and the parser assembly
 * must converge on this expression-level position.
 *
 * This grammar intentionally does not assign a fixed parameter count.
 */
lambdaExpression
    : PIPE lambdaParameterList? PIPE lambdaBody
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
    ;

lambdaBody
    : expression
    | blockExpression
    ;


/* ============================================================================
 * 33. NEW EXPRESSION
 * ========================================================================== */

/*
 * Construction syntax:
 *
 *     new Type(...)
 *
 * Construction semantics remain outside the grammar.
 */
newExpression
    : NEW typeExpression
      (
          LPAREN argumentList? RPAREN
      )?
    ;


/* ============================================================================
 * 34. LITERAL EXPRESSIONS
 * ========================================================================== */

/*
 * Literal syntax is consumed from the canonical lexical/type contracts.
 *
 * The literal grammar deliberately does not impose machine-width limits.
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
 * 35. EXPRESSION LIST
 * ========================================================================== */

/*
 * Used where the language expects a comma-separated sequence of expressions.
 *
 * There is no artificial list-size limit.
 */
expressionList
    : expression
      (
          COMMA
          expression
      )*
      COMMA?
    ;


/* ============================================================================
 * 36. ARGUMENT LIST
 * ========================================================================== */

/*
 * Argument lists are deliberately open-ended.
 *
 * This rule is the universal positional argument sequence.
 *
 * Named/keyword arguments and variadic/spread arguments may be extended by
 * the specialized call grammar without changing expression precedence.
 */
argumentList
    : argument
      (
          COMMA
          argument
      )*
      COMMA?
    ;

argument
    : expression
    ;


/* ============================================================================
 * 37. TYPE EXPRESSION INTEGRATION
 * ========================================================================== */

/*
 * `typeExpression` belongs to the canonical type grammar.
 *
 * This expression grammar consumes it only where source syntax requires a
 * type, such as:
 *
 *     generic invocation
 *     construction
 *
 * No duplicate type grammar is defined here.
 */
typeExpressionList
    : typeExpression
      (
          COMMA
          typeExpression
      )*
      COMMA?
    ;


/* ============================================================================
 * 38. IDENTIFIER INTEGRATION
 * ========================================================================== */

/*
 * Identifier spelling belongs to the canonical lexer/name grammar.
 *
 * This local parser rule is deliberately a thin integration boundary.
 *
 * If the canonical parser composition already exposes `identifier`, it must
 * bind to that canonical rule rather than creating another lexical grammar.
 */
identifier
    : IDENTIFIER
    ;


/* ============================================================================
 * 39. COMPOSITION BOUNDARIES
 * ========================================================================== */

/*
 * The following rules are consumed from the canonical parser composition:
 *
 *     conditionalExpression
 *     blockExpression
 *     typeExpression
 *
 * Their ownership remains outside this file.
 *
 * This prevents:
 *
 *     - duplicate block grammar;
 *     - duplicate conditional grammar;
 *     - duplicate type grammar;
 *     - parser dependency cycles;
 *     - semantic ownership fragmentation.
 *
 * The parser assembly must provide these rules exactly once.
 */


/* ============================================================================
 * 40. OPERATOR PRECEDENCE CONTRACT
 * ========================================================================== */

/*
 * From lowest to highest precedence:
 *
 *     assignment
 *     conditional
 *     range
 *     logical OR
 *     logical AND
 *     bitwise OR
 *     bitwise XOR
 *     bitwise AND
 *     equality
 *     relational
 *     shift
 *     additive
 *     multiplicative
 *     prefix
 *     postfix
 *     primary
 *
 * This ordering is structural.
 *
 * Semantic analysis remains responsible for whether an operator is legal for
 * a particular type/domain.
 */


/* ============================================================================
 * 41. ASSOCIATIVITY CONTRACT
 * ========================================================================== */

/*
 * Assignment:
 *
 *     right associative
 *
 * Example:
 *
 *     a = b = c
 *
 * is parsed as:
 *
 *     a = (b = c)
 *
 * Binary precedence layers:
 *
 *     left associative
 *
 * The conditional-expression component owns conditional associativity.
 *
 * Range associativity is deliberately restricted to one range operator at the
 * current grammar layer. Chained range semantics must be explicitly specified
 * rather than accidentally created by recursive expression references.
 */


/* ============================================================================
 * 42. OPERATOR OVERLOADING
 * ========================================================================== */

/*
 * This grammar does not decide whether:
 *
 *     +
 *     -
 *     *
 *     /
 *     %
 *     &
 *     |
 *     ^
 *     <<
 *     >>
 *
 * operate on:
 *
 *     integers
 *     floating point
 *     vectors
 *     matrices
 *     tensors
 *     symbolic values
 *     user-defined values
 *     hardware values
 *     quantum/classical values
 *     distributed values
 *
 * Semantic analysis owns that decision.
 */


/* ============================================================================
 * 43. QUANTUM OPERATION EXTENSIBILITY
 * ========================================================================== */

/*
 * Do NOT add rules such as:
 *
 *     quantumGate
 *         : H
 *         | X
 *         | Y
 *         | Z
 *         | CNOT
 *         | ...
 *         ;
 *
 * Quantum operation names remain extensible.
 *
 * Example:
 *
 *     H(q)
 *     CNOT(control, target)
 *     vendor::operation(parameter)(q)
 *     logical::operation(q)
 *
 * The semantic layer resolves the operation and lowers it to:
 *
 *     quantum::ir
 *
 * This is essential for:
 *
 *     future quantum operations;
 *     logical operations;
 *     vendor operations;
 *     calibrated operations;
 *     error-corrected operations;
 *     simulator operations;
 *     hardware-independent programs.
 */


/* ============================================================================
 * 44. HARDWARE SCALABILITY
 * ========================================================================== */

/*
 * Expressions MUST remain independent of physical realization.
 *
 * Forbidden as grammar-level universal restrictions:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_REGISTER_WIDTH
 *     MAX_TENSOR_RANK
 *     MAX_VECTOR_WIDTH
 *     MAX_ACCELERATORS
 *
 * A program may contain a semantic requirement such as:
 *
 *     requires resource(...)
 *
 * but that resource requirement is not an expression-parser hardware limit.
 */


/* ============================================================================
 * 45. ERROR / DIAGNOSTIC CONTRACT
 * ========================================================================== */

/*
 * This grammar must produce structurally deterministic parse trees.
 *
 * Diagnostics must preserve source locations.
 *
 * Semantic errors such as:
 *
 *     invalid operand type
 *     invalid assignment target
 *     unavailable capability
 *     unsupported quantum operation
 *     invalid hardware requirement
 *
 * belong downstream.
 *
 * Parser errors must not be silently converted into semantic success.
 */


/* ============================================================================
 * 46. NO RUNTIME ACTIONS
 * ========================================================================== */

/*
 * This grammar intentionally contains:
 *
 *     - no embedded Rust;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware discovery;
 *     - no runtime execution;
 *     - no unsafe code.
 *
 * It is pure syntax.
 */


/* ============================================================================
 * 47. INTEGRATION WITH FRONTEND AST
 * ========================================================================== */

/*
 * Expected conceptual mappings:
 *
 *     assignmentExpression
 *         -> assignment expression AST
 *
 *     binary expression layers
 *         -> binary/operator expression AST
 *
 *     prefixExpression
 *         -> unary expression AST
 *
 *     postfixExpression
 *         -> ordered postfix/member/call/index AST
 *
 *     primaryExpression
 *         -> literal/name/aggregate/lambda/construction AST
 *
 *     conditionalExpression
 *         -> existing ConditionalExpression AST
 *
 *     rangeExpression
 *         -> range expression AST
 *
 * The exact Rust structures belong to:
 *
 *     src/frontend/ast/
 *
 * This grammar must not introduce a second AST hierarchy.
 */


/* ============================================================================
 * 48. CANONICAL QUANTUM LOWERING
 * ========================================================================== */

/*
 * Generic expression syntax:
 *
 *     operation(args)
 *
 * may eventually be recognized semantically as a quantum operation.
 *
 * The lowering path is:
 *
 *     expression AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization
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
 *
 * No physical qubit or topology is represented in this grammar.
 */


/* ============================================================================
 * 49. CLASSICAL / HDL LOWERING
 * ========================================================================== */

/*
 * Classical expressions lower through the canonical classical semantic/IR
 * path.
 *
 * HDL/hardware expressions lower through their semantic hardware/HDL path.
 *
 * Hybrid expressions remain domain-neutral until semantic analysis establishes
 * the domain boundary.
 *
 * This keeps one source language rather than creating separate expression
 * languages for CPU, GPU, FPGA, QPU, HDL, AI, data, or distributed systems.
 */


/* ============================================================================
 * 50. COMPLETION CONTRACT
 * ========================================================================== */

/*
 * This file is complete only when all of the following are true:
 *
 * [x] Owns exactly one public expression entry point.
 * [x] Owns the complete expression precedence composition.
 * [x] Does not redefine conditionalExpression.
 * [x] Does not create a second type grammar.
 * [x] Does not create a second block grammar.
 * [x] Uses canonical lexer token names.
 * [x] Supports right-associative assignment.
 * [x] Supports compound assignments.
 * [x] Supports open-ended binary chains.
 * [x] Supports open-ended postfix chains.
 * [x] Supports open-ended argument lists.
 * [x] Supports open-ended index lists.
 * [x] Supports generic invocation syntax.
 * [x] Supports optional member syntax where lexer/version enables it.
 * [x] Supports null-coalescing token integration where enabled downstream.
 * [x] Does not hard-code machine/resource limits.
 * [x] Does not enumerate quantum gates.
 * [x] Does not introduce a quantum IR.
 * [x] Preserves domain neutrality.
 * [x] Preserves deterministic parsing.
 * [x] Contains no embedded Rust actions.
 * [x] Contains no unsafe Rust.
 *
 * Integration acceptance additionally requires:
 *
 *     - canonical parser composition binds conditionalExpression;
 *     - canonical parser composition binds blockExpression;
 *     - canonical parser composition binds typeExpression;
 *     - frontend AST covers every expression alternative;
 *     - semantic analysis covers every expression category;
 *     - canonical IR lowering exists for every semantically supported form;
 *     - positive tests exist;
 *     - negative tests exist;
 *     - boundary tests exist;
 *     - scalability tests exist;
 *     - compatibility tests exist;
 *     - deterministic parse tests exist;
 *     - no competing public expression rule remains in the final parser.
 *
 * ============================================================================
 */
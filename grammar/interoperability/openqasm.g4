/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/interoperability/openqasm.g4
 *
 * Purpose:
 *     Authoritative ANTLR grammar for OpenQASM 3.x interoperability syntax.
 *
 * Architectural position:
 *
 *     OpenQASM source
 *          |
 *          v
 *     OpenQASM lexer/parser
 *          |
 *          v
 *     OpenQASM syntax representation
 *          |
 *          v
 *     Zamani OpenQASM frontend
 *          |
 *          v
 *     semantic validation
 *          |
 *          v
 *     canonical quantum::ir
 *          |
 *          +--> optimization
 *          +--> routing
 *          +--> scheduling
 *          +--> QEC
 *          +--> ZQN
 *          +--> hardware HAL
 *          +--> runtime
 *
 * OWNERSHIP
 * ----------
 * This file owns:
 *
 *   - OpenQASM lexical syntax;
 *   - OpenQASM 3.x parser structure;
 *   - OpenQASM source-level declarations;
 *   - OpenQASM source-level statements;
 *   - OpenQASM source-level expressions;
 *   - OpenQASM gate definitions;
 *   - OpenQASM subroutine definitions;
 *   - OpenQASM calibration syntax;
 *   - OpenQASM timing syntax;
 *   - OpenQASM annotations and pragmas;
 *   - OpenQASM version syntax.
 *
 * This file DOES NOT own:
 *
 *   - Zamani's canonical AST;
 *   - quantum::ir;
 *   - QEC algorithms;
 *   - ZQN fault/noise semantics;
 *   - routing;
 *   - scheduling;
 *   - optimization;
 *   - hardware discovery;
 *   - calibration execution;
 *   - resource discovery;
 *   - runtime execution;
 *   - provider APIs;
 *   - device IDs;
 *   - physical topology;
 *   - machine capacities.
 *
 * SCALABILITY
 * -----------
 * No production hardware quantity is encoded here.
 *
 * In particular there is intentionally no:
 *
 *   MAX_QUBITS
 *   MAX_BITS
 *   MAX_GATES
 *   MAX_REGISTERS
 *   MAX_DEPTH
 *   MAX_DEVICES
 *   MAX_NODES
 *   MAX_MEMORY
 *   MAX_THREADS
 *
 * A source program may express arbitrarily large indexed/ranged structures.
 * Actual feasibility is determined later by semantic/resource analysis.
 *
 * Rust implementation target:
 *
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe
 *
 * ============================================================================
 */

grammar OpenQASM;


/* ============================================================================
 * PROGRAM
 * ========================================================================== */

program
    : versionHeader? programItem* EOF
    ;

versionHeader
    : OPENQASM versionNumber SEMICOLON
    ;

versionNumber
    : DECIMAL_INTEGER DOT DECIMAL_INTEGER
    ;


/* ============================================================================
 * TOP-LEVEL ITEMS
 *
 * OpenQASM permits declarations, definitions, statements, annotations,
 * includes, pragmas and implementation-oriented calibration declarations.
 *
 * ========================================================================== */

programItem
    : annotation*
      (
          includeStatement
        | pragmaStatement
        | constDeclaration
        | inputDeclaration
        | outputDeclaration
        | letDeclaration
        | aliasDeclaration
        | classicalDeclaration
        | quantumDeclaration
        | gateDeclaration
        | defDeclaration
        | externDeclaration
        | defcalGrammarDeclaration
        | defcalDeclaration
        | statement
      )
    ;


/* ============================================================================
 * INCLUDES
 *
 * Include paths are source data. This grammar does not authorize filesystem
 * or network access. Resolution belongs to the frontend/toolchain.
 * ========================================================================== */

includeStatement
    : INCLUDE stringLiteral SEMICOLON
    ;


/* ============================================================================
 * PRAGMAS
 *
 * Pragmas are preserved as source-level implementation metadata.
 * ========================================================================== */

pragmaStatement
    : PRAGMA pragmaBody? SEMICOLON
    ;

pragmaBody
    : pragmaAtom+
    ;

pragmaAtom
    : identifier
    | integerLiteral
    | floatLiteral
    | stringLiteral
    | punctuation
    ;


/* ============================================================================
 * ANNOTATIONS
 * ========================================================================== */

annotation
    : AT identifier annotationPayload?
    ;

annotationPayload
    : LPAREN annotationArgumentList? RPAREN
    ;

annotationArgumentList
    : expression (COMMA expression)*
    ;


/* ============================================================================
 * DECLARATIONS
 * ========================================================================== */

constDeclaration
    : CONST scalarType identifier ASSIGN expression SEMICOLON
    ;

inputDeclaration
    : INPUT typeSpecifier identifier SEMICOLON
    ;

outputDeclaration
    : OUTPUT typeSpecifier identifier SEMICOLON
    ;

letDeclaration
    : LET typeSpecifier? identifier ASSIGN expression SEMICOLON
    ;

aliasDeclaration
    : LET identifier ASSIGN aliasExpression SEMICOLON
    ;

classicalDeclaration
    : typeSpecifier identifier SEMICOLON
    | typeSpecifier identifier ASSIGN expression SEMICOLON
    ;

quantumDeclaration
    : quantumType identifier SEMICOLON
    ;


/* ============================================================================
 * TYPES
 * ========================================================================== */

typeSpecifier
    : scalarType
    | bitType
    | arrayType
    | durationType
    | stretchType
    | quantumType
    | customType
    ;

scalarType
    : BOOL
    | INT LPAREN integerLiteral RPAREN
    | UINT LPAREN integerLiteral RPAREN
    | FLOAT_TYPE LPAREN integerLiteral RPAREN
    | COMPLEX LPAREN integerLiteral RPAREN
    ;

bitType
    : BIT
    | BIT LPAREN expression RPAREN
    ;

arrayType
    : ARRAY LT typeSpecifier COMMA arrayDimensionList GT
    ;

arrayDimensionList
    : expression (COMMA expression)*
    ;

durationType
    : DURATION
    ;

stretchType
    : STRETCH
    ;

quantumType
    : QUBIT
    | QUBIT LBRACK expression RBRACK
    ;

customType
    : identifier
    ;


/* ============================================================================
 * GATE DEFINITIONS
 *
 * Parameters and qubit arguments are symbolic source entities.
 * No gate arity is hard-coded.
 * ========================================================================== */

gateDeclaration
    : GATE identifier
      gateParameterList?
      gateQubitParameterList
      LBRACE
      gateStatement*
      RBRACE
    ;

gateParameterList
    : LPAREN identifierList? RPAREN
    ;

gateQubitParameterList
    : identifier (COMMA identifier)*
    ;

gateStatement
    : annotation* gateOperationStatement
    ;

gateOperationStatement
    : gateModifier* gateOperation
    ;

gateOperation
    : gateCall SEMICOLON
    | barrierStatement
    | delayStatement
    | resetStatement
    | measurementStatement
    ;


/* ============================================================================
 * SUBROUTINE DEFINITIONS
 * ========================================================================== */

defDeclaration
    : DEF identifier
      LPAREN defParameterList? RPAREN
      returnType?
      LBRACE
      statement*
      RBRACE
    ;

defParameterList
    : defParameter (COMMA defParameter)*
    ;

defParameter
    : typeSpecifier identifier
    | typeSpecifier identifier ELLIPSIS
    ;

returnType
    : ARROW typeSpecifier
    ;

externDeclaration
    : EXTERN identifier
      LPAREN externParameterList? RPAREN
      returnType?
      SEMICOLON
    ;

externParameterList
    : typeSpecifier (COMMA typeSpecifier)*
    ;


/* ============================================================================
 * GATE MODIFIERS
 *
 * Modifiers are syntactic constructs. Their physical realization is not.
 * ========================================================================== */

gateModifier
    : INV
    | POW expression
    | CTRL
    | NEGCTRL
    ;


/* ============================================================================
 * GATE CALLS
 * ========================================================================== */

gateCall
    : gateModifier* identifier gateArguments? qubitArguments
    ;

gateArguments
    : LPAREN expressionList? RPAREN
    ;

qubitArguments
    : designatorList
    ;


/* ============================================================================
 * MEASUREMENT
 * ========================================================================== */

measurementStatement
    : MEASURE measureOperand ARROW classicalDestination SEMICOLON
    ;

measureOperand
    : indexedIdentifier
    | identifier
    ;

classicalDestination
    : indexedIdentifier
    | identifier
    ;


/* ============================================================================
 * RESET / BARRIER / DELAY
 * ========================================================================== */

resetStatement
    : RESET quantumOperand SEMICOLON
    ;

barrierStatement
    : BARRIER barrierOperandList? SEMICOLON
    ;

barrierOperandList
    : quantumOperand (COMMA quantumOperand)*
    ;

delayStatement
    : DELAY delayExpression quantumOperandList SEMICOLON
    ;

delayExpression
    : expression
    ;

quantumOperandList
    : quantumOperand (COMMA quantumOperand)*
    ;

quantumOperand
    : indexedIdentifier
    | identifier
    ;


/* ============================================================================
 * CALIBRATION GRAMMAR DECLARATIONS
 * ========================================================================== */

defcalGrammarDeclaration
    : DEFCALGRAMMAR stringLiteral SEMICOLON
    ;


/* ============================================================================
 * CALIBRATION DEFINITIONS
 *
 * Calibration syntax is preserved here without assigning execution semantics.
 * Hardware calibration ownership remains outside the grammar.
 * ========================================================================== */

defcalDeclaration
    : DEFCAL
      defcalSignature
      LBRACE
      defcalStatement*
      RBRACE
    ;

defcalSignature
    : identifier
      defcalArgumentList?
      defcalQubitList?
      returnType?
    ;

defcalArgumentList
    : LPAREN defcalArgument (COMMA defcalArgument)* RPAREN
    ;

defcalArgument
    : typeSpecifier identifier
    | expression
    ;

defcalQubitList
    : quantumOperand (COMMA quantumOperand)*
    ;

defcalStatement
    : annotation*
      (
          defcalInstruction
        | delayStatement
        | barrierStatement
        | returnStatement
        | expressionStatement
      )
    ;

defcalInstruction
    : identifier defcalArguments? SEMICOLON
    ;

defcalArguments
    : LPAREN expressionList? RPAREN
    ;


/* ============================================================================
 * BOX / TIMING
 * ========================================================================== */

boxStatement
    : BOX boxDuration? LBRACE statement* RBRACE
    ;

boxDuration
    : LPAREN expression RPAREN
    ;


/* ============================================================================
 * GENERAL STATEMENTS
 * ========================================================================== */

statement
    : annotation* (
          blockStatement
        | declarationStatement
        | assignmentStatement
        | expressionStatement
        | ifStatement
        | whileStatement
        | forStatement
        | switchStatement
        | breakStatement
        | continueStatement
        | returnStatement
        | resetStatement
        | measurementStatement
        | barrierStatement
        | delayStatement
        | boxStatement
        | aliasStatement
    )
    ;

blockStatement
    : LBRACE statement* RBRACE
    ;

declarationStatement
    : constDeclaration
    | inputDeclaration
    | outputDeclaration
    | letDeclaration
    | classicalDeclaration
    | quantumDeclaration
    ;

assignmentStatement
    : assignmentTarget assignmentOperator expression SEMICOLON
    ;

assignmentTarget
    : indexedIdentifier
    | identifier
    ;

assignmentOperator
    : ASSIGN
    | PLUS_ASSIGN
    | MINUS_ASSIGN
    | STAR_ASSIGN
    | SLASH_ASSIGN
    ;

expressionStatement
    : expression SEMICOLON
    ;

aliasStatement
    : LET identifier ASSIGN aliasExpression SEMICOLON
    ;

breakStatement
    : BREAK SEMICOLON
    ;

continueStatement
    : CONTINUE SEMICOLON
    ;

returnStatement
    : RETURN expression? SEMICOLON
    ;


/* ============================================================================
 * CONDITIONALS
 * ========================================================================== */

ifStatement
    : IF LPAREN expression RPAREN statement
      (ELSE statement)?
    ;

whileStatement
    : WHILE LPAREN expression RPAREN statement
    ;

forStatement
    : FOR identifier IN rangeExpression statement
    | FOR LPAREN forInitializer? SEMICOLON expression? SEMICOLON expression?
      RPAREN statement
    ;

forInitializer
    : declarationStatement
    | assignmentStatement
    ;


/* ============================================================================
 * SWITCH
 * ========================================================================== */

switchStatement
    : SWITCH LPAREN expression RPAREN
      LBRACE
      switchCase*
      switchDefault?
      RBRACE
    ;

switchCase
    : CASE expression COLON statement*
    ;

switchDefault
    : DEFAULT COLON statement*
    ;


/* ============================================================================
 * RANGE EXPRESSIONS
 * ========================================================================== */

rangeExpression
    : expression RANGE expression
    | expression RANGE expression RANGE expression
    ;


/* ============================================================================
 * EXPRESSIONS
 *
 * Precedence is encoded explicitly so the grammar has deterministic parsing
 * behavior and does not depend on target hardware.
 * ========================================================================== */

expression
    : conditionalExpression
    ;

conditionalExpression
    : logicalOrExpression
      (QUESTION expression COLON expression)?
    ;

logicalOrExpression
    : logicalAndExpression (OR_OR logicalAndExpression)*
    ;

logicalAndExpression
    : bitwiseOrExpression (AND_AND bitwiseOrExpression)*
    ;

bitwiseOrExpression
    : bitwiseXorExpression (PIPE bitwiseXorExpression)*
    ;

bitwiseXorExpression
    : bitwiseAndExpression (CARET bitwiseAndExpression)*
    ;

bitwiseAndExpression
    : equalityExpression (AMPERSAND equalityExpression)*
    ;

equalityExpression
    : relationalExpression
      (
          EQUAL_EQUAL relationalExpression
        | NOT_EQUAL relationalExpression
      )*
    ;

relationalExpression
    : shiftExpression
      (
          LT shiftExpression
        | LE shiftExpression
        | GT shiftExpression
        | GE shiftExpression
      )*
    ;

shiftExpression
    : additiveExpression
      (
          SHIFT_LEFT additiveExpression
        | SHIFT_RIGHT additiveExpression
      )*
    ;

additiveExpression
    : multiplicativeExpression
      (
          PLUS multiplicativeExpression
        | MINUS multiplicativeExpression
      )*
    ;

multiplicativeExpression
    : powerExpression
      (
          STAR powerExpression
        | SLASH powerExpression
        | PERCENT powerExpression
      )*
    ;

powerExpression
    : unaryExpression (POWER powerExpression)?
    ;

unaryExpression
    : PLUS unaryExpression
    | MINUS unaryExpression
    | NOT unaryExpression
    | TILDE unaryExpression
    | postfixExpression
    ;

postfixExpression
    : primaryExpression postfixSuffix*
    ;

postfixSuffix
    : LPAREN expressionList? RPAREN
    | LBRACK expression RBRACK
    | DOT identifier
    ;

primaryExpression
    : literal
    | indexedIdentifier
    | identifier
    | piConstant
    | parenthesizedExpression
    ;

parenthesizedExpression
    : LPAREN expression RPAREN
    ;

expressionList
    : expression (COMMA expression)*
    ;


/* ============================================================================
 * ALIASES
 * ========================================================================== */

aliasExpression
    : aliasOperand
    | aliasExpression CONCAT aliasExpression
    ;

aliasOperand
    : indexedIdentifier
    | identifier
    | parenthesizedAlias
    ;

parenthesizedAlias
    : LPAREN aliasExpression RPAREN
    ;


/* ============================================================================
 * IDENTIFIERS
 * ========================================================================== */

indexedIdentifier
    : identifier LBRACK expression RBRACK
    ;

identifierList
    : identifier (COMMA identifier)*
    ;

identifier
    : IDENTIFIER
    ;


/* ============================================================================
 * LITERALS
 *
 * Literal syntax retains arbitrary source precision. Conversion to Rust
 * machine types is explicitly a later semantic operation.
 * ========================================================================== */

literal
    : integerLiteral
    | floatLiteral
    | bitStringLiteral
    | booleanLiteral
    | stringLiteral
    | durationLiteral
    | stretchLiteral
    | complexLiteral
    ;

integerLiteral
    : DECIMAL_INTEGER
    | BINARY_INTEGER
    | OCTAL_INTEGER
    | HEX_INTEGER
    ;

floatLiteral
    : DECIMAL_FLOAT
    ;

bitStringLiteral
    : BIT_STRING
    ;

booleanLiteral
    : TRUE
    | FALSE
    ;

stringLiteral
    : STRING
    ;

durationLiteral
    : DECIMAL_FLOAT durationUnit
    | DECIMAL_INTEGER durationUnit
    ;

durationUnit
    : DT
    | S
    | MS
    | US
    | NS
    | PS
    | FS
    ;

stretchLiteral
    : STRETCH_LITERAL
    ;

complexLiteral
    : imaginaryLiteral
    ;

imaginaryLiteral
    : DECIMAL_FLOAT IMAGINARY
    | DECIMAL_INTEGER IMAGINARY
    ;

piConstant
    : PI
    ;


/* ============================================================================
 * PUNCTUATION HELPER
 *
 * Used only by pragma parsing.
 * ========================================================================== */

punctuation
    : LPAREN
    | RPAREN
    | LBRACK
    | RBRACK
    | LBRACE
    | RBRACE
    | COMMA
    | SEMICOLON
    | COLON
    | DOT
    | AT
    | QUESTION
    | PIPE
    | AMPERSAND
    | CARET
    ;


/* ============================================================================
 * LEXER
 * ========================================================================== */


/* --------------------------------------------------------------------------
 * Reserved OpenQASM words
 * ------------------------------------------------------------------------ */

OPENQASM
    : 'OPENQASM'
    ;

INCLUDE
    : 'include'
    ;

PRAGMA
    : 'pragma'
    ;

CONST
    : 'const'
    ;

INPUT
    : 'input'
    ;

OUTPUT
    : 'output'
    ;

LET
    : 'let'
    ;

GATE
    : 'gate'
    ;

DEF
    : 'def'
    ;

EXTERN
    : 'extern'
    ;

DEFCALGRAMMAR
    : 'defcalgrammar'
    ;

DEFCAL
    : 'defcal'
    ;

QUBIT
    : 'qubit'
    ;

BIT
    : 'bit'
    ;

BOOL
    : 'bool'
    ;

INT
    : 'int'
    ;

UINT
    : 'uint'
    ;

FLOAT_TYPE
    : 'float'
    ;

COMPLEX
    : 'complex'
    ;

ARRAY
    : 'array'
    ;

DURATION
    : 'duration'
    ;

STRETCH
    : 'stretch'
    ;

MEASURE
    : 'measure'
    ;

RESET
    : 'reset'
    ;

BARRIER
    : 'barrier'
    ;

DELAY
    : 'delay'
    ;

BOX
    : 'box'
    ;

IF
    : 'if'
    ;

ELSE
    : 'else'
    ;

WHILE
    : 'while'
    ;

FOR
    : 'for'
    ;

IN
    : 'in'
    ;

SWITCH
    : 'switch'
    ;

CASE
    : 'case'
    ;

DEFAULT
    : 'default'
    ;

BREAK
    : 'break'
    ;

CONTINUE
    : 'continue'
    ;

RETURN
    : 'return'
    ;

INV
    : 'inv'
    ;

POW
    : 'pow'
    ;

CTRL
    : 'ctrl'
    ;

NEGCTRL
    : 'negctrl'
    ;

PI
    : 'pi'
    ;

TRUE
    : 'true'
    ;

FALSE
    : 'false'
    ;

DT
    : 'dt'
    ;

S
    : 's'
    ;

MS
    : 'ms'
    ;

US
    : 'us'
    ;

NS
    : 'ns'
    ;

PS
    : 'ps'
    ;

FS
    : 'fs'
    ;

IMAGINARY
    : 'im'
    ;


/* --------------------------------------------------------------------------
 * Operators
 * ------------------------------------------------------------------------ */

ARROW
    : '->'
    ;

RANGE
    : '...'
    ;

ELLIPSIS
    : '...'
    ;

CONCAT
    : '++'
    ;

POWER
    : '**'
    ;

SHIFT_LEFT
    : '<<'
    ;

SHIFT_RIGHT
    : '>>'
    ;

LE
    : '<='
    ;

GE
    : '>='
    ;

EQUAL_EQUAL
    : '=='
    ;

NOT_EQUAL
    : '!='
    ;

AND_AND
    : '&&'
    ;

OR_OR
    : '||'
    ;

PLUS_ASSIGN
    : '+='
    ;

MINUS_ASSIGN
    : '-='
    ;

STAR_ASSIGN
    : '*='
    ;

SLASH_ASSIGN
    : '/='
    ;

ASSIGN
    : '='
    ;

PLUS
    : '+'
    ;

MINUS
    : '-'
    ;

STAR
    : '*'
    ;

SLASH
    : '/'
    ;

PERCENT
    : '%'
    ;

AMPERSAND
    : '&'
    ;

PIPE
    : '|'
    ;

CARET
    : '^'
    ;

TILDE
    : '~'
    ;

NOT
    : '!'
    ;

QUESTION
    : '?'
    ;

LT
    : '<'
    ;

GT
    : '>'
    ;

AT
    : '@'
    ;

DOT
    : '.'
    ;


/* --------------------------------------------------------------------------
 * Delimiters
 * ------------------------------------------------------------------------ */

LPAREN
    : '('
    ;

RPAREN
    : ')'
    ;

LBRACK
    : '['
    ;

RBRACK
    : ']'
    ;

LBRACE
    : '{'
    ;

RBRACE
    : '}'
    ;

COMMA
    : ','
    ;

SEMICOLON
    : ';'
    ;

COLON
    : ':'
    ;


/* --------------------------------------------------------------------------
 * Literals
 *
 * No conversion to finite machine widths occurs here.
 * ------------------------------------------------------------------------ */

BINARY_INTEGER
    : '0' [bB] [01] ([01] | '_')*
    ;

OCTAL_INTEGER
    : '0' [oO] [0-7] ([0-7] | '_')*
    ;

HEX_INTEGER
    : '0' [xX] [0-9a-fA-F] ([0-9a-fA-F] | '_')*
    ;

DECIMAL_FLOAT
    : [0-9] ([0-9] | '_')*
      (
          '.' [0-9] ([0-9] | '_')*
          ([eE] [+-]? [0-9] ([0-9] | '_')*)?
        | [eE] [+-]? [0-9] ([0-9] | '_')*
      )
    | '.' [0-9] ([0-9] | '_')*
      ([eE] [+-]? [0-9] ([0-9] | '_')*)?
    ;

DECIMAL_INTEGER
    : [0-9] ([0-9] | '_')*
    ;

BIT_STRING
    : '"' [01xzXZ]* '"'
    ;

STRETCH_LITERAL
    : '$' IDENTIFIER
    ;


/* --------------------------------------------------------------------------
 * Strings
 * ------------------------------------------------------------------------ */

STRING
    : '"' (ESCAPE_SEQUENCE | ~["\\\r\n])* '"'
    ;

fragment ESCAPE_SEQUENCE
    : '\\'
      (
          ["\\/bfnrt]
        | 'u' HEX_DIGIT HEX_DIGIT HEX_DIGIT HEX_DIGIT
      )
    ;


/* --------------------------------------------------------------------------
 * Identifiers
 *
 * Unicode letters are intentionally accepted. Identifier policy and Unicode
 * normalization belong to semantic/name-resolution infrastructure.
 * ------------------------------------------------------------------------ */

IDENTIFIER
    : IDENTIFIER_START IDENTIFIER_CONTINUE*
    ;

fragment IDENTIFIER_START
    : [a-zA-Z_]
    | '\u0080'..'\uFFFF'
    ;

fragment IDENTIFIER_CONTINUE
    : IDENTIFIER_START
    | [0-9]
    ;


/* --------------------------------------------------------------------------
 * Whitespace/comments
 * ------------------------------------------------------------------------ */

WS
    : [ \t\r\n]+ -> channel(HIDDEN)
    ;

LINE_COMMENT
    : '//' ~[\r\n]* -> channel(HIDDEN)
    ;

BLOCK_COMMENT
    : '/*' .*? '*/' -> channel(HIDDEN)
    ;


/* --------------------------------------------------------------------------
 * Numeric helpers
 * ------------------------------------------------------------------------ */

fragment HEX_DIGIT
    : [0-9a-fA-F]
    ;
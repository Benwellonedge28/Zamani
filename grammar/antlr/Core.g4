/*
 * ============================================================================
 * Zamani — Core ANTLR4 Grammar
 * ============================================================================
 *
 * File:
 *   grammar/antlr/Core.g4
 *
 * Language:
 *   Zamani
 *
 * Compiler:
 *   ZUTC
 *
 * Minimum Rust implementation baseline:
 *   Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *   Rust unsafe is forbidden.
 *
 * Architectural purpose:
 *   Canonical syntax boundary for the Zamani language.
 *
 * Design principles:
 *
 *   1. Syntax expresses portable computation and intent.
 *   2. Semantics are resolved outside the grammar.
 *   3. Quantum syntax is hardware independent.
 *   4. No fixed machine sizes are encoded here.
 *   5. No finite hardware gate catalogue is authoritative here.
 *   6. Mathematical functionality is represented primarily through
 *      expressions, types, calls and semantic capabilities.
 *   7. Deep/generated structures must not require arbitrary language-level
 *      limits.
 *   8. The grammar must remain deterministic.
 *   9. Every accepted construct must have an AST/semantic destination.
 *  10. Unsupported semantics must be diagnosed later; they must not be
 *      silently reinterpreted.
 *
 * Pipeline:
 *
 *   source
 *      |
 *      v
 *   CoreLexer
 *      |
 *      v
 *   CoreParser
 *      |
 *      v
 *   Frontend AST
 *      |
 *      v
 *   Name/module resolution
 *      |
 *      v
 *   Type/effect/capability/resource analysis
 *      |
 *      v
 *   Canonical semantic IR
 *      |
 *      +--> quantum::ir
 *      +--> classical/control/data IR
 *      +--> effect/resource/temporal metadata
 *      |
 *      v
 *   optimization
 *      |
 *      v
 *   routing / scheduling / resilience / ZQN
 *      |
 *      v
 *   target lowering
 *
 * This grammar is intentionally NOT a hardware grammar.
 * ============================================================================
 */

grammar Core;


/* ============================================================================
 * PROGRAM
 * ========================================================================== */

program
    : documentation? compilationUnit EOF
    ;

compilationUnit
    : item*
    ;

item
    : attributes* declaration
    | statement
    ;


/* ============================================================================
 * DOCUMENTATION
 * ========================================================================== */

documentation
    : docComment+
    ;

docComment
    : DOC_LINE
    | DOC_BLOCK
    ;


/* ============================================================================
 * ATTRIBUTES
 * ========================================================================== */

attributes
    : attribute+
    ;

attribute
    : AT identifier
    | AT identifier LPAREN argumentList? RPAREN
    ;


/* ============================================================================
 * DECLARATIONS
 *
 * Domain declarations intentionally remain semantic categories rather than
 * embedding implementation-specific inventories.
 * ========================================================================== */

declaration
    : moduleDeclaration
    | importDeclaration
    | exportDeclaration
    | useDeclaration
    | functionDeclaration
    | structDeclaration
    | enumDeclaration
    | traitDeclaration
    | implDeclaration
    | classDeclaration
    | interfaceDeclaration
    | recordDeclaration
    | typeAliasDeclaration
    | constantDeclaration
    | effectDeclaration
    | quantumDeclaration
    | nanoDeclaration
    | languageDeclaration
    | macroDeclaration
    | packageDeclaration
    | externDeclaration
    ;


/* ============================================================================
 * MODULES / IMPORTS / EXPORTS
 * ========================================================================== */

moduleDeclaration
    : MODULE qualifiedName moduleBody?
    ;

moduleBody
    : LBRACE item* RBRACE
    ;

importDeclaration
    : IMPORT importTarget importSource? SEMI?
    ;

importTarget
    : qualifiedName
    | STAR AS identifier
    | LBRACE importSpecifierList RBRACE
    ;

importSpecifierList
    : importSpecifier (COMMA importSpecifier)* COMMA?
    ;

importSpecifier
    : identifier (AS identifier)?
    ;

importSource
    : FROM stringLiteral
    ;

exportDeclaration
    : EXPORT exportTarget exportSource? SEMI?
    ;

exportTarget
    : STAR
    | qualifiedName
    | LBRACE exportSpecifierList RBRACE
    ;

exportSpecifierList
    : exportSpecifier (COMMA exportSpecifier)* COMMA?
    ;

exportSpecifier
    : identifier (AS identifier)?
    ;

exportSource
    : FROM stringLiteral
    ;

useDeclaration
    : USE qualifiedName (AS identifier)? SEMI?
    ;

qualifiedName
    : identifier (DOUBLE_COLON identifier)*
    ;


/* ============================================================================
 * PACKAGES
 * ========================================================================== */

packageDeclaration
    : PACKAGE identifier LBRACE packageField* RBRACE
    ;

packageField
    : identifier COLON expression SEMI?
    ;


/* ============================================================================
 * VISIBILITY / MODIFIERS
 * ========================================================================== */

visibility
    : PUBLIC
    | PRIVATE
    | PROTECTED
    | INTERNAL
    ;

modifier
    : STATIC
    | CONST
    | ASYNC
    | INLINE
    | OVERRIDE
    | FINAL
    | ABSTRACT
    | MUT
    | SEALED
    | PARTIAL
    | EXTERN
    | VOLATILE
    | SIM
    | VECTORIZED
    | GPU
    | PARALLEL
    | PURE
    | IMMUTABLE
    | LINEAR
    | AFFINE
    ;

modifiers
    : modifier+
    ;


/* ============================================================================
 * FUNCTIONS
 * ========================================================================== */

functionDeclaration
    : visibility?
      modifiers?
      FN identifier
      genericParameters?
      LPAREN parameterList? RPAREN
      returnType?
      effectClause?
      contractClause*
      functionBody
    ;

functionBody
    : block
    ;

returnType
    : ARROW typeExpression
    ;

parameterList
    : parameter (COMMA parameter)* COMMA?
    ;

parameter
    : parameterPattern (COLON typeExpression)? defaultValue?
    ;

parameterPattern
    : identifier
    | MUT identifier
    | pattern
    ;

defaultValue
    : ASSIGN expression
    ;

genericParameters
    : LT genericParameterList GT
    ;

genericParameterList
    : genericParameter (COMMA genericParameter)* COMMA?
    ;

genericParameter
    : identifier genericBounds?
    ;

genericBounds
    : COLON typeBoundList
    ;

typeBoundList
    : typeExpression (PLUS typeExpression)*
    ;


/* ============================================================================
 * CONTRACTS / EFFECTS
 * ========================================================================== */

contractClause
    : CONTRACT contractBody
    ;

contractBody
    : LBRACE contractItem* RBRACE
    ;

contractItem
    : REQUIRES LPAREN expression RPAREN SEMI?
    | ENSURES LPAREN expression RPAREN SEMI?
    | INVARIANT LPAREN expression RPAREN SEMI?
    ;

effectClause
    : WITH EFFECTS LBRACE effectReferenceList? RBRACE
    ;

effectReferenceList
    : effectReference (COMMA effectReference)* COMMA?
    ;

effectReference
    : qualifiedName
    ;


/* ============================================================================
 * STRUCTS
 * ========================================================================== */

structDeclaration
    : visibility?
      modifiers?
      STRUCT identifier
      genericParameters?
      structBody
    ;

structBody
    : LBRACE structMember* RBRACE
    ;

structMember
    : attributes* visibility? modifiers? fieldDeclaration
    ;

fieldDeclaration
    : identifier COLON typeExpression SEMI?
    ;


/* ============================================================================
 * RECORDS
 * ========================================================================== */

recordDeclaration
    : visibility?
      RECORD identifier
      genericParameters?
      recordBody
    ;

recordBody
    : LBRACE fieldDeclaration* RBRACE
    ;


/* ============================================================================
 * ENUMS
 * ========================================================================== */

enumDeclaration
    : visibility?
      modifiers?
      ENUM identifier
      genericParameters?
      enumBody
    ;

enumBody
    : LBRACE enumVariant* RBRACE
    ;

enumVariant
    : attributes* identifier
      (
          LPAREN parameterTypeList? RPAREN
        | LBRACE fieldDeclaration* RBRACE
      )?
      COMMA?
    ;

parameterTypeList
    : typeExpression (COMMA typeExpression)* COMMA?
    ;


/* ============================================================================
 * TRAITS
 * ========================================================================== */

traitDeclaration
    : visibility?
      modifiers?
      TRAIT identifier
      genericParameters?
      traitBounds?
      LBRACE traitMember* RBRACE
    ;

traitBounds
    : COLON typeBoundList
    ;

traitMember
    : attributes* functionSignature
    | typeAliasDeclaration
    | constantDeclaration
    ;

functionSignature
    : modifiers?
      FN identifier
      genericParameters?
      LPAREN parameterList? RPAREN
      returnType?
      effectClause?
      contractClause*
      SEMI?
    ;


/* ============================================================================
 * IMPLEMENTATIONS
 * ========================================================================== */

implDeclaration
    : modifiers?
      IMPL
      genericParameters?
      implementationTarget
      implementationTrait?
      LBRACE implMember* RBRACE
    ;

implementationTarget
    : typeExpression
    ;

implementationTrait
    : FOR typeExpression
    ;

implMember
    : attributes* functionDeclaration
    | constantDeclaration
    | typeAliasDeclaration
    ;


/* ============================================================================
 * CLASSES
 * ========================================================================== */

classDeclaration
    : visibility?
      modifiers?
      CLASS identifier
      genericParameters?
      extendsClause?
      implementsClause?
      permitsClause?
      LBRACE classMember* RBRACE
    ;

extendsClause
    : EXTENDS typeExpression (COMMA typeExpression)*
    ;

implementsClause
    : IMPLEMENTS typeExpression (COMMA typeExpression)*
    ;

permitsClause
    : PERMITS typeExpression (COMMA typeExpression)*
    ;

classMember
    : attributes* visibility? modifiers? fieldDeclaration
    | attributes* visibility? modifiers? functionDeclaration
    ;


/* ============================================================================
 * INTERFACES
 * ========================================================================== */

interfaceDeclaration
    : visibility?
      modifiers?
      INTERFACE identifier
      genericParameters?
      extendsClause?
      LBRACE interfaceMember* RBRACE
    ;

interfaceMember
    : attributes* functionSignature
    | fieldDeclaration
    | typeAliasDeclaration
    ;


/* ============================================================================
 * TYPE ALIASES / CONSTANTS
 * ========================================================================== */

typeAliasDeclaration
    : visibility?
      TYPE identifier
      genericParameters?
      ASSIGN typeExpression
      SEMI?
    ;

constantDeclaration
    : visibility?
      CONST identifier
      (COLON typeExpression)?
      ASSIGN expression
      SEMI?
    ;


/* ============================================================================
 * EFFECT DECLARATIONS
 * ========================================================================== */

effectDeclaration
    : visibility?
      EFFECT identifier
      genericParameters?
      effectSignature?
      SEMI?
    ;

effectSignature
    : LBRACE effectOperation* RBRACE
    ;

effectOperation
    : FN identifier
      LPAREN parameterList? RPAREN
      returnType?
      SEMI?
    ;


/* ============================================================================
 * LANGUAGE / META / MACRO
 * ========================================================================== */

languageDeclaration
    : LANGUAGE identifier
      languageBody
    ;

languageBody
    : LBRACE item* RBRACE
    ;

macroDeclaration
    : visibility?
      MACRO identifier
      genericParameters?
      LPAREN parameterList? RPAREN
      macroBody
    ;

macroBody
    : block
    | expression
    ;

externDeclaration
    : EXTERN ABI? STRING? functionSignature
    ;

ABI
    : identifier
    ;


/* ============================================================================
 * QUANTUM DECLARATIONS
 *
 * There is deliberately NO closed gate list.
 *
 * A quantum operation is:
 *
 *     operation specification + arguments + semantic targets
 *
 * Backend-native operations, decompositions, routing, scheduling and
 * calibration belong downstream of the canonical quantum IR.
 * ========================================================================== */

quantumDeclaration
    : QUANTUM CIRCUIT identifier
      genericParameters?
      circuitParameters?
      block
    | CIRCUIT identifier
      genericParameters?
      circuitParameters?
      block
    ;

circuitParameters
    : LPAREN parameterList? RPAREN
    ;


/* ============================================================================
 * NANO / AGENT DECLARATIONS
 * ========================================================================== */

nanoDeclaration
    : NANO AGENT identifier
      genericParameters?
      agentParameters?
      block
    | AGENT identifier
      genericParameters?
      agentParameters?
      block
    ;

agentParameters
    : LPAREN parameterList? RPAREN
    ;


/* ============================================================================
 * STATEMENTS
 * ========================================================================== */

statement
    : variableDeclaration
    | constantStatement
    | expressionStatement
    | returnStatement
    | breakStatement
    | continueStatement
    | yieldStatement
    | ifStatement
    | whileStatement
    | doWhileStatement
    | forStatement
    | forallStatement
    | foreachStatement
    | matchStatement
    | tryStatement
    | throwStatement
    | handleStatement
    | blockStatement
    | quantumStatement
    | nanoStatement
    | temporalStatement
    | sankofaStatement
    | unsafeSourceBlock
    | emptyStatement
    ;

variableDeclaration
    : (LET | VAR)
      MUT?
      pattern
      (COLON typeExpression)?
      (ASSIGN expression)?
      SEMI?
    ;

constantStatement
    : CONST
      identifier
      (COLON typeExpression)?
      ASSIGN expression
      SEMI?
    ;

expressionStatement
    : expression SEMI?
    ;

returnStatement
    : RETURN expression? SEMI?
    ;

breakStatement
    : BREAK identifier? SEMI?
    ;

continueStatement
    : CONTINUE SEMI?
    ;

yieldStatement
    : YIELD expression? SEMI?
    ;

emptyStatement
    : SEMI
    ;


/* ============================================================================
 * CONTROL FLOW
 * ========================================================================== */

ifStatement
    : IF expression block
      (ELSE IF expression block)*
      (ELSE block)?
    ;

whileStatement
    : WHILE expression block
    ;

doWhileStatement
    : DO block WHILE expression SEMI?
    ;

forStatement
    : FOR pattern IN expression block
    ;

forallStatement
    : FORALL pattern IN expression
      (WHEN expression)?
      block
    ;

foreachStatement
    : FOREACH pattern IN expression
      (PARALLEL)?
      block
    ;


/* ============================================================================
 * MATCH / PATTERNS
 * ========================================================================== */

matchStatement
    : MATCH expression LBRACE matchArm+ RBRACE
    ;

matchArm
    : pattern
      guardClause?
      FAT_ARROW
      (expression | block)
      COMMA?
    ;

guardClause
    : WHEN expression
    ;

pattern
    : wildcardPattern
    | bindingPattern
    | literalPattern
    | tuplePattern
    | arrayPattern
    | structPattern
    | enumPattern
    | rangePattern
    | orPattern
    | referencePattern
    | typePattern
    | parenthesizedPattern
    ;

wildcardPattern
    : UNDERSCORE
    ;

bindingPattern
    : identifier
    ;

literalPattern
    : literal
    ;

tuplePattern
    : LPAREN pattern (COMMA pattern)+ COMMA? RPAREN
    ;

arrayPattern
    : LBRACKET patternList? RBRACKET
    ;

structPattern
    : qualifiedName
      LBRACE structPatternFieldList? RBRACE
    ;

structPatternFieldList
    : structPatternField (COMMA structPatternField)* COMMA?
    ;

structPatternField
    : identifier (COLON pattern)?
    ;

enumPattern
    : qualifiedName
      (
          LPAREN patternList? RPAREN
        | LBRACE structPatternFieldList? RBRACE
      )
    ;

rangePattern
    : pattern rangeOperator pattern
    ;

orPattern
    : pattern PIPE pattern
    ;

referencePattern
    : AMPERSAND MUT? pattern
    ;

typePattern
    : pattern COLON typeExpression
    ;

parenthesizedPattern
    : LPAREN pattern RPAREN
    ;

patternList
    : pattern (COMMA pattern)* COMMA?
    ;


/* ============================================================================
 * EXCEPTION / EFFECT HANDLING
 * ========================================================================== */

tryStatement
    : TRY block catchClause* finallyClause?
    ;

catchClause
    : CATCH LPAREN identifier (COLON typeExpression)? RPAREN block
    ;

finallyClause
    : FINALLY block
    ;

throwStatement
    : THROW expression SEMI?
    ;

handleStatement
    : HANDLE expression block
    ;


/* ============================================================================
 * BLOCKS
 * ========================================================================== */

blockStatement
    : block
    ;

block
    : LBRACE statement* RBRACE
    ;


/* ============================================================================
 * SOURCE-LEVEL UNSAFE
 *
 * This is a language feature, not compiler implementation unsafety.
 * The Rust compiler itself remains safe Rust.
 * ========================================================================== */

unsafeSourceBlock
    : UNSAFE block
    ;


/* ============================================================================
 * QUANTUM STATEMENTS
 * ========================================================================== */

quantumStatement
    : APPLY quantumOperationSpec TO quantumTargetList SEMI?
    | APPLY quantumOperationSpec SEMI?
    | MEASURE quantumTargetList measureDestination? SEMI?
    | RESET quantumTargetList SEMI?
    | BARRIER quantumTargetList? SEMI?
    | CONTROL quantumControlSpec block
    | ADJOINT block
    | INVERSE block
    | OBSERVE expression SEMI?
    ;

quantumOperationSpec
    : expression
    ;

quantumTargetList
    : expression (COMMA expression)*
    ;

measureDestination
    : ARROW expression
    ;

quantumControlSpec
    : expression
    ;


/* ============================================================================
 * NANO STATEMENTS
 * ========================================================================== */

nanoStatement
    : PERFORM expression SEMI?
    | LEARN expression SEMI?
    | INFER expression SEMI?
    ;


/* ============================================================================
 * TEMPORAL / ZAMANI / SANKOFA
 * ========================================================================== */

temporalStatement
    : ZAMANI expression SEMI?
    | SASA expression SEMI?
    | MTS expression SEMI?
    ;

sankofaStatement
    : REMEMBER expression SEMI?
    | RECALL expression SEMI?
    | WISDOM expression SEMI?
    ;


/* ============================================================================
 * EXPRESSIONS
 *
 * The expression hierarchy is intentionally explicit and deterministic.
 * ========================================================================== */

expression
    : assignmentExpression
    ;

assignmentExpression
    : conditionalExpression
      (
          assignmentOperator
          assignmentExpression
      )?
    ;

conditionalExpression
    : rangeExpression
      (QUESTION expression COLON expression)?
    ;

rangeExpression
    : logicalOrExpression
      (
          rangeOperator
          logicalOrExpression
      )?
    ;

logicalOrExpression
    : logicalAndExpression
      ((OR_OR | OR) logicalAndExpression)*
    ;

logicalAndExpression
    : bitOrExpression
      ((AND_AND | AND) bitOrExpression)*
    ;

bitOrExpression
    : bitXorExpression
      (PIPE bitXorExpression)*
    ;

bitXorExpression
    : bitAndExpression
      (CARET bitAndExpression)*
    ;

bitAndExpression
    : equalityExpression
      (AMPERSAND equalityExpression)*
    ;

equalityExpression
    : comparisonExpression
      ((EQ_EQ | NOT_EQ) comparisonExpression)*
    ;

comparisonExpression
    : shiftExpression
      (
          LT
        | LE
        | GT
        | GE
        | IS
      )
      shiftExpression
    | shiftExpression
    ;

shiftExpression
    : additiveExpression
      ((SHIFT_LEFT | SHIFT_RIGHT) additiveExpression)*
    ;

additiveExpression
    : multiplicativeExpression
      ((PLUS | MINUS) multiplicativeExpression)*
    ;

multiplicativeExpression
    : prefixExpression
      ((STAR | SLASH | PERCENT) prefixExpression)*
    ;

prefixExpression
    : prefixOperator prefixExpression
    | postfixExpression
    ;

prefixOperator
    : PLUS
    | MINUS
    | NOT
    | TILDE
    | AMPERSAND
    | STAR
    ;

postfixExpression
    : primaryExpression postfix*
    ;

postfix
    : callSuffix
    | indexSuffix
    | memberSuffix
    | optionalSuffix
    ;

callSuffix
    : LPAREN argumentList? RPAREN
    ;

indexSuffix
    : LBRACKET expression RBRACKET
    ;

memberSuffix
    : DOT identifier
    | DOUBLE_COLON identifier
    ;

optionalSuffix
    : QUESTION
    ;


/* ============================================================================
 * PRIMARY EXPRESSIONS
 * ========================================================================== */

primaryExpression
    : identifier
    | literal
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
    | parenthesizedExpression
    | blockExpression
    ;

parenthesizedExpression
    : LPAREN expression RPAREN
    ;

tupleExpression
    : LPAREN expression COMMA expressionListTail? RPAREN
    ;

expressionListTail
    : expression (COMMA expression)* COMMA?
    ;

arrayExpression
    : LBRACKET expressionList? RBRACKET
    ;

expressionList
    : expression (COMMA expression)* COMMA?
    ;

objectExpression
    : LBRACE objectFieldList? RBRACE
    ;

objectFieldList
    : objectField (COMMA objectField)* COMMA?
    ;

objectField
    : identifier COLON expression
    | identifier
    ;

lambdaExpression
    : PIPE parameterList? PIPE
      (ARROW typeExpression)?
      (block | expression)
    ;

ifExpression
    : IF expression block
      ELSE (ifExpression | block)
    ;

matchExpression
    : MATCH expression LBRACE matchArm+ RBRACE
    ;

loopExpression
    : WHILE expression block
    | FOR pattern IN expression block
    ;

asyncExpression
    : ASYNC block
    ;

awaitExpression
    : AWAIT expression
    ;

spawnExpression
    : SPAWN expression
    ;

newExpression
    : NEW typeExpression
      (LPAREN argumentList? RPAREN)?
    ;

quantumExpression
    : quantumLiteral
    | APPLY quantumOperationSpec TO quantumTargetList
    | MEASURE quantumTargetList
    | RESET quantumTargetList
    | BARRIER quantumTargetList?
    ;

nanoExpression
    : NANO expression
    | AGENT expression
    ;

sankofaExpression
    : REMEMBER expression
    | RECALL expression
    ;


/* ============================================================================
 * ARGUMENTS
 * ========================================================================== */

argumentList
    : argument (COMMA argument)* COMMA?
    ;

argument
    : namedArgument
    | expression
    ;

namedArgument
    : identifier COLON expression
    ;


/* ============================================================================
 * ASSIGNMENT / RANGE OPERATORS
 * ========================================================================== */

assignmentOperator
    : ASSIGN
    | PLUS_ASSIGN
    | MINUS_ASSIGN
    | STAR_ASSIGN
    | SLASH_ASSIGN
    ;

rangeOperator
    : DOT_DOT
    | DOT_DOT_EQ
    ;


/* ============================================================================
 * TYPE SYSTEM
 *
 * The grammar represents type syntax. Type validity belongs to semantic
 * analysis.
 * ========================================================================== */

typeExpression
    : functionType
    ;

functionType
    : effectfulType
      (ARROW functionType)?
    ;

effectfulType
    : unionType
      (WITH EFFECTS LBRACE effectReferenceList? RBRACE)?
    ;

unionType
    : intersectionType (PIPE intersectionType)*
    ;

intersectionType
    : postfixType (AMPERSAND postfixType)*
    ;

postfixType
    : primaryType typePostfix*
    ;

typePostfix
    : QUESTION
    ;

primaryType
    : namedType
    | genericType
    | tupleType
    | arrayType
    | sliceType
    | referenceType
    | functionType
    | quantumType
    | temporalType
    | resultType
    | neverType
    | dependentType
    | parenthesizedType
    ;

namedType
    : qualifiedName
    ;

genericType
    : qualifiedName LT typeArgumentList GT
    ;

typeArgumentList
    : typeExpression (COMMA typeExpression)* COMMA?
    ;

tupleType
    : LPAREN typeExpression COMMA typeExpression (COMMA typeExpression)* COMMA? RPAREN
    ;

arrayType
    : LBRACKET typeExpression SEMI expression RBRACKET
    ;

sliceType
    : LBRACKET typeExpression RBRACKET
    ;

referenceType
    : AMPERSAND MUT? typeExpression
    ;

quantumType
    : QUBIT
    | QUANTUM typeExpression
    ;

temporalType
    : MTS LT typeExpression GT
    ;

resultType
    : RESULT LT typeExpression (COMMA typeExpression)? GT
    ;

neverType
    : NEVER
    ;

dependentType
    : PI identifier COLON typeExpression DOT typeExpression
    | SIGMA identifier COLON typeExpression DOT typeExpression
    ;

parenthesizedType
    : LPAREN typeExpression RPAREN
    ;


/* ============================================================================
 * LITERALS
 * ========================================================================== */

literal
    : integerLiteral
    | floatingLiteral
    | stringLiteral
    | charLiteral
    | booleanLiteral
    | nullLiteral
    | quantumLiteral
    ;

integerLiteral
    : INTEGER
    ;

floatingLiteral
    : FLOAT
    ;

stringLiteral
    : STRING
    ;

charLiteral
    : CHAR
    ;

booleanLiteral
    : TRUE
    | FALSE
    ;

nullLiteral
    : NIL
    ;

quantumLiteral
    : QUANTUM_LITERAL
    ;


/* ============================================================================
 * IDENTIFIERS
 * ========================================================================== */

identifier
    : IDENTIFIER
    ;


/* ============================================================================
 * KEYWORDS
 *
 * These are language-level reserved words. Domain operations themselves are
 * deliberately not listed here merely because a backend happens to support
 * them.
 * ========================================================================== */

MODULE      : 'module' ;
IMPORT      : 'import' ;
EXPORT      : 'export' ;
USE         : 'use' ;
FROM        : 'from' ;
AS          : 'as' ;

FN          : 'fn' ;
LET         : 'let' ;
VAR         : 'var' ;
CONST       : 'const' ;

STRUCT      : 'struct' ;
ENUM        : 'enum' ;
TRAIT       : 'trait' ;
IMPL        : 'impl' ;
CLASS       : 'class' ;
INTERFACE   : 'interface' ;
RECORD      : 'record' ;
TYPE        : 'type' ;

PUBLIC      : 'public' ;
PRIVATE     : 'private' ;
PROTECTED   : 'protected' ;
INTERNAL    : 'internal' ;

STATIC      : 'static' ;
ASYNC       : 'async' ;
AWAIT       : 'await' ;
INLINE      : 'inline' ;
OVERRIDE    : 'override' ;
FINAL       : 'final' ;
ABSTRACT    : 'abstract' ;
MUT         : 'mut' ;
SEALED      : 'sealed' ;
PARTIAL     : 'partial' ;
EXTERN      : 'extern' ;
VOLATILE    : 'volatile' ;
SIM         : 'sim' ;
VECTORIZED  : 'vectorized' ;
GPU         : 'gpu' ;
PARALLEL    : 'parallel' ;
PURE        : 'pure' ;
IMMUTABLE   : 'immutable' ;
LINEAR      : 'linear' ;
AFFINE      : 'affine' ;

EXTENDS     : 'extends' ;
IMPLEMENTS  : 'implements' ;
PERMITS     : 'permits' ;

EFFECT      : 'effect' ;
EFFECTS     : 'effects' ;
WITH        : 'with' ;
HANDLE      : 'handle' ;
REQUIRES    : 'requires' ;
ENSURES     : 'ensures' ;
INVARIANT   : 'invariant' ;

RETURN      : 'return' ;
BREAK       : 'break' ;
CONTINUE    : 'continue' ;
YIELD       : 'yield' ;

IF          : 'if' ;
ELSE        : 'else' ;
WHILE       : 'while' ;
DO          : 'do' ;
FOR         : 'for' ;
FORALL      : 'forall' ;
FOREACH     : 'foreach' ;
IN          : 'in' ;
WHEN        : 'when' ;
MATCH       : 'match' ;
CASE        : 'case' ;

TRY         : 'try' ;
CATCH       : 'catch' ;
FINALLY     : 'finally' ;
THROW       : 'throw' ;

UNSAFE      : 'unsafe' ;

NEW         : 'new' ;
SPAWN       : 'spawn' ;

QUANTUM     : 'quantum' ;
CIRCUIT     : 'circuit' ;
QUBIT       : 'Qubit' ;
APPLY       : 'apply' ;
MEASURE     : 'measure' ;
RESET       : 'reset' ;
BARRIER     : 'barrier' ;
CONTROL     : 'control' ;
ADJOINT     : 'adjoint' ;
INVERSE     : 'inverse' ;
OBSERVE     : 'observe' ;

NANO        : 'nano' ;
AGENT       : 'agent' ;
PERFORM     : 'perform' ;
LEARN       : 'learn' ;
INFER       : 'infer' ;

MTS         : 'mts' ;
ZAMANI      : 'zamani' ;
SASA        : 'sasa' ;

REMEMBER    : 'remember' ;
RECALL      : 'recall' ;
WISDOM      : 'wisdom' ;

LANGUAGE    : 'language' ;
MACRO       : 'macro' ;
PACKAGE     : 'package' ;

RESULT      : 'Result' ;
NEVER       : 'Never' ;
PI          : 'Π' ;
SIGMA       : 'Σ' ;

IS          : 'is' ;

TRUE        : 'true' ;
FALSE       : 'false' ;
NIL         : 'nil' ;


/* ============================================================================
 * OPERATORS
 *
 * Longest operators are declared before their prefixes.
 * ========================================================================== */

ARROW       : '->' ;
FAT_ARROW   : '=>' ;

SHIFT_LEFT  : '<<' ;
SHIFT_RIGHT : '>>' ;

EQ_EQ       : '==' ;
NOT_EQ      : '!=' ;
LE          : '<=' ;
GE          : '>=' ;

AND_AND     : '&&' ;
OR_OR       : '||' ;

PLUS_ASSIGN : '+=' ;
MINUS_ASSIGN: '-=' ;
STAR_ASSIGN : '*=' ;
SLASH_ASSIGN: '/=' ;

DOT_DOT_EQ  : '..=' ;
DOT_DOT     : '..' ;

DOUBLE_COLON: '::' ;

ASSIGN      : '=' ;

LT          : '<' ;
GT          : '>' ;

PLUS        : '+' ;
MINUS       : '-' ;
STAR        : '*' ;
SLASH       : '/' ;
PERCENT     : '%' ;

NOT         : '!' ;
TILDE       : '~' ;

AMPERSAND   : '&' ;
PIPE        : '|' ;
CARET       : '^' ;

QUESTION    : '?' ;

AT          : '@' ;


/* ============================================================================
 * DELIMITERS
 * ========================================================================== */

LPAREN      : '(' ;
RPAREN      : ')' ;
LBRACE      : '{' ;
RBRACE      : '}' ;
LBRACKET    : '[' ;
RBRACKET    : ']' ;

COMMA       : ',' ;
DOT         : '.' ;
COLON       : ':' ;
SEMI        : ';' ;

UNDERSCORE  : '_' ;


/* ============================================================================
 * LITERALS
 * ========================================================================== */

/*
 * Integer literals:
 *
 *   decimal
 *   hexadecimal
 *   binary
 *   octal
 *
 * No machine width is encoded here.
 * Semantic conversion decides whether a literal fits a requested type.
 */

INTEGER
    : DEC_DIGIT (DEC_DIGIT | '_')*
    | '0' [xX] HEX_DIGIT (HEX_DIGIT | '_')*
    | '0' [bB] BIN_DIGIT (BIN_DIGIT | '_')*
    | '0' [oO] OCT_DIGIT (OCT_DIGIT | '_')*
    ;

FLOAT
    : DEC_DIGIT (DEC_DIGIT | '_')*
      DOT
      DEC_DIGIT (DEC_DIGIT | '_')*
      EXPONENT?
    | DOT
      DEC_DIGIT (DEC_DIGIT | '_')*
      EXPONENT?
    | DEC_DIGIT (DEC_DIGIT | '_')*
      EXPONENT
    ;

fragment EXPONENT
    : [eE] [+-]? DEC_DIGIT (DEC_DIGIT | '_')*
    ;

STRING
    : '"' (ESCAPE_SEQUENCE | ~["\\\r\n])* '"'
    ;

CHAR
    : '\'' (ESCAPE_SEQUENCE | ~['\\\r\n]) '\''
    ;

fragment ESCAPE_SEQUENCE
    : '\\'
      (
          ['"\\bfnrt]
        | 'u' HEX_DIGIT HEX_DIGIT HEX_DIGIT HEX_DIGIT
      )
    ;


/* ============================================================================
 * QUANTUM STATE LITERALS
 *
 * The literal is lexical notation for semantic quantum state data.
 * It does not imply a physical representation or register width.
 * ========================================================================== */

QUANTUM_LITERAL
    : '|' [01+\-] '⟩'
    ;


/* ============================================================================
 * IDENTIFIERS
 *
 * Unicode letters are accepted. Semantic normalization and reserved-word
 * policy belong to the language implementation.
 * ========================================================================== */

IDENTIFIER
    : ID_START ID_CONTINUE*
    ;

fragment ID_START
    : [a-zA-Z_]
    | '\u0080'..'\uD7FF'
    | '\uE000'..'\uFFFF'
    ;

fragment ID_CONTINUE
    : ID_START
    | [0-9]
    ;


/* ============================================================================
 * COMMENTS / DOCUMENTATION
 * ========================================================================== */

DOC_LINE
    : '///' ~[\r\n]* -> channel(HIDDEN)
    ;

DOC_BLOCK
    : '/**' .*? '*/' -> channel(HIDDEN)
    ;

LINE_COMMENT
    : '//' ~[\r\n]* -> channel(HIDDEN)
    ;

BLOCK_COMMENT
    : '/*' .*? '*/' -> channel(HIDDEN)
    ;


/* ============================================================================
 * WHITESPACE
 * ========================================================================== */

WS
    : [ \t\r\n\u000B\u000C]+ -> channel(HIDDEN)
    ;


/* ============================================================================
 * NUMERIC FRAGMENTS
 * ========================================================================== */

fragment DEC_DIGIT
    : [0-9]
    ;

fragment HEX_DIGIT
    : [0-9a-fA-F]
    ;

fragment BIN_DIGIT
    : [01]
    ;

fragment OCT_DIGIT
    : [0-7]
    ;
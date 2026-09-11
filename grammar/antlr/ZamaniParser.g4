/**
 * Zamani Programming Language
 * Canonical ANTLR4 Parser Grammar
 *
 * File:
 *     grammar/antlr/ZamaniParser.g4
 *
 * Role:
 *     Canonical concrete-syntax parser for Zamani.
 *
 * Architecture:
 *
 *     UTF-8 source
 *          |
 *          v
 *     ZamaniLexer.g4
 *          |
 *          v
 *     ZamaniParser.g4
 *          |
 *          v
 *     Frontend AST
 *          |
 *          v
 *     Name / semantic / type / effect / resource analysis
 *          |
 *          v
 *     Canonical IR
 *          |
 *          +--> classical lowering
 *          +--> quantum::ir
 *          +--> target-independent lowering
 *          |
 *          v
 *     target realization
 *
 * Design requirements:
 *
 *   - Source expresses computational intent.
 *   - No hardware-specific assumptions.
 *   - No fixed number of qubits/registers/resources.
 *   - No fixed quantum gate inventory.
 *   - No CPU/GPU/QPU/vendor syntax.
 *   - No machine-width assumptions.
 *   - No semantic execution in the parser.
 *   - No filesystem/network access.
 *   - No compiler-global mutation from syntax.
 *   - Parser preserves source structure and source spans.
 *
 * Runtime/compiler baseline:
 *   Rust 1.97 / Rust 1.97.1
 *   Rust implementation must contain no unsafe code.
 *
 * IMPORTANT:
 *   This grammar is parser-only.
 *   All lexical rules belong to ZamaniLexer.g4.
 */

parser grammar ZamaniParser;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 1. ENTRY POINT
 * ========================================================================== */

/**
 * A source file may be empty.
 *
 * Program ::= { Attribute | Item | Statement } EOF ;
 */
program
    : sourceElement* EOF
    ;

sourceElement
    : attribute
    | item
    | statement
    ;


/* ============================================================================
 * 2. ATTRIBUTES
 * ========================================================================== */

/**
 * Attribute ::= "#[" AttributeBody "]" ;
 *
 * Attributes are syntax-level metadata.
 * Their meaning belongs to semantic analysis.
 */
attribute
    : HASH LBRACKET attributeBody RBRACKET
    ;

attributeBody
    : attributePath
      (LPAREN argumentList? RPAREN)?
    ;

attributePath
    : identifier
      (DOUBLE_COLON identifier)*
    ;


/* ============================================================================
 * 3. ITEMS / DECLARATIONS
 * ========================================================================== */

item
    : functionDeclaration
    | structDeclaration
    | enumDeclaration
    | traitDeclaration
    | implDeclaration
    | classDeclaration
    | interfaceDeclaration
    | recordDeclaration
    | typeAliasDeclaration
    | moduleDeclaration
    | importDeclaration
    | exportDeclaration
    | useDeclaration
    | quantumDeclaration
    | nanoAgentDeclaration
    | effectDeclaration
    | languageDeclaration
    ;


/* ============================================================================
 * 4. FUNCTIONS
 * ========================================================================== */

/**
 * FunctionDeclaration ::=
 *     ["async"] "fn" IDENT
 *     [GenericParameters]
 *     "(" [ParameterList] ")"
 *     ["->" TypeExpression]
 *     [WhereClause]
 *     BlockExpression ;
 */
functionDeclaration
    : asyncModifier?
      FN identifier
      genericParameters?
      LPAREN parameterList? RPAREN
      returnType?
      whereClause?
      blockExpression
    ;

asyncModifier
    : ASYNC
    ;

returnType
    : THIN_ARROW typeExpression
    ;

parameterList
    : parameter (COMMA parameter)*
    ;

parameter
    : MUT? identifier
      (COLON typeExpression)?
      (ASSIGN expression)?
    ;


/* ============================================================================
 * 5. GENERICS
 * ========================================================================== */

genericParameters
    : LESS_THAN typeParameter (COMMA typeParameter)* GREATER_THAN
    ;

typeParameter
    : identifier typeBound*
    ;

typeBound
    : COLON typeExpression
    ;

whereClause
    : WHERE wherePredicate (COMMA wherePredicate)*
    ;

wherePredicate
    : identifier COLON typeExpression
    ;


/* ============================================================================
 * 6. STRUCTS
 * ========================================================================== */

structDeclaration
    : visibilityModifier?
      STRUCT identifier
      genericParameters?
      whereClause?
      LBRACE structField* RBRACE
    ;

structField
    : visibilityModifier?
      identifier
      COLON typeExpression
      COMMA?
    ;


/* ============================================================================
 * 7. ENUMS
 * ========================================================================== */

enumDeclaration
    : visibilityModifier?
      ENUM identifier
      genericParameters?
      whereClause?
      LBRACE enumVariant* RBRACE
    ;

enumVariant
    : identifier
      (
          LPAREN enumTupleFields? RPAREN
        | LBRACE enumStructFields? RBRACE
      )?
      COMMA?
    ;

enumTupleFields
    : typeExpression (COMMA typeExpression)*
    ;

enumStructFields
    : structField+
    ;


/* ============================================================================
 * 8. TRAITS
 * ========================================================================== */

traitDeclaration
    : visibilityModifier?
      TRAIT identifier
      genericParameters?
      whereClause?
      LBRACE traitItem* RBRACE
    ;

traitItem
    : functionSignature
    | typeAliasDeclaration
    | associatedConstant
    ;

functionSignature
    : ASYNC?
      FN identifier
      genericParameters?
      LPAREN parameterList? RPAREN
      returnType?
      whereClause?
      SEMI?
    ;

associatedConstant
    : CONST identifier
      (COLON typeExpression)?
      (ASSIGN expression)?
      SEMI?
    ;


/* ============================================================================
 * 9. IMPLEMENTATIONS
 * ========================================================================== */

implDeclaration
    : IMPL
      genericParameters?
      typeExpression
      (FOR typeExpression)?
      whereClause?
      LBRACE implItem* RBRACE
    ;

implItem
    : functionDeclaration
    | typeAliasDeclaration
    | associatedConstant
    ;


/* ============================================================================
 * 10. CLASSES
 * ========================================================================== */

classDeclaration
    : visibilityModifier?
      ABSTRACT?
      CLASS identifier
      genericParameters?
      inheritanceClause?
      whereClause?
      LBRACE classMember* RBRACE
    ;

inheritanceClause
    : EXTENDS typeExpression
      (IMPLEMENTS typeExpression (COMMA typeExpression)*)?
    | IMPLEMENTS typeExpression (COMMA typeExpression)*
    ;

classMember
    : attribute* visibilityModifier?
      STATIC?
      VIRTUAL?
      OVERRIDE?
      ABSTRACT?
      functionDeclaration
    | attribute* visibilityModifier? structField
    ;


/* ============================================================================
 * 11. INTERFACES
 * ========================================================================== */

interfaceDeclaration
    : visibilityModifier?
      INTERFACE identifier
      genericParameters?
      inheritanceTypes?
      whereClause?
      LBRACE interfaceMember* RBRACE
    ;

inheritanceTypes
    : COLON typeExpression (COMMA typeExpression)*
    ;

interfaceMember
    : attribute* functionSignature
    | attribute* typeAliasDeclaration
    ;


/* ============================================================================
 * 12. RECORDS
 * ========================================================================== */

recordDeclaration
    : visibilityModifier?
      RECORD identifier
      genericParameters?
      whereClause?
      LBRACE structField* RBRACE
    ;


/* ============================================================================
 * 13. TYPE ALIASES
 * ========================================================================== */

typeAliasDeclaration
    : visibilityModifier?
      TYPE identifier
      genericParameters?
      (ASSIGN typeExpression)?
      whereClause?
      SEMI?
    ;


/* ============================================================================
 * 14. MODULES
 * ========================================================================== */

moduleDeclaration
    : visibilityModifier?
      MODULE modulePath
      (
          SEMI
        | blockExpression
      )
    ;

modulePath
    : identifier
      (DOUBLE_COLON identifier)*
    ;


/* ============================================================================
 * 15. IMPORT / EXPORT / USE
 * ========================================================================== */

importDeclaration
    : IMPORT importPath
      (AS identifier)?
      SEMI?
    ;

exportDeclaration
    : EXPORT
      (
          STAR
          (FROM modulePath)?
        | importPath
      )
      SEMI?
    ;

useDeclaration
    : USE useTree
      SEMI?
    ;

useTree
    : usePath
    | LBRACE useTreeList? RBRACE
    ;

useTreeList
    : useTree (COMMA useTree)* COMMA?
    ;

usePath
    : identifier
      (DOUBLE_COLON identifier)*
      (
          DOUBLE_COLON STAR
        | DOUBLE_COLON LBRACE useTreeList? RBRACE
      )?
    ;

importPath
    : identifier
      (DOUBLE_COLON identifier)*
    ;


/* ============================================================================
 * 16. VISIBILITY
 * ========================================================================== */

visibilityModifier
    : PUBLIC
    | PUB
    | PRIVATE
    | PROTECTED
    ;


/* ============================================================================
 * 17. VARIABLE / CONSTANT STATEMENTS
 * ========================================================================== */

statement
    : letStatement
    | varStatement
    | constStatement
    | returnStatement
    | breakStatement
    | continueStatement
    | whileStatement
    | forStatement
    | loopStatement
    | matchStatement
    | tryStatement
    | throwStatement
    | unsafeStatement
    | rememberStatement
    | handleStatement
    | wisdomStatement
    | omniversalStatement
    | specialSystemStatement
    | expressionStatement
    ;

letStatement
    : LET MUT? identifier
      (COLON typeExpression)?
      ASSIGN expression
      SEMI?
    ;

varStatement
    : VAR identifier
      (COLON typeExpression)?
      ASSIGN expression
      SEMI?
    ;

constStatement
    : CONST identifier
      (COLON typeExpression)?
      ASSIGN expression
      SEMI?
    ;

returnStatement
    : RETURN expression? SEMI?
    ;

breakStatement
    : BREAK identifier? SEMI?
    ;

continueStatement
    : CONTINUE identifier? SEMI?
    ;


/* ============================================================================
 * 18. CONTROL FLOW
 * ========================================================================== */

whileStatement
    : WHILE expression blockExpression
    ;

forStatement
    : FOR pattern IN expression blockExpression
    ;

loopStatement
    : LOOP blockExpression
    ;

matchStatement
    : MATCH expression LBRACE matchArm* RBRACE
    ;

matchArm
    : pattern
      (IF expression)?
      FAT_ARROW
      (
          blockExpression
        | expression
      )
      COMMA?
    ;

tryStatement
    : TRY blockExpression
      catchClause*
      finallyClause?
    ;

catchClause
    : CATCH
      (
          LPAREN pattern RPAREN
        | pattern
      )?
      blockExpression
    ;

finallyClause
    : FINALLY blockExpression
    ;

throwStatement
    : THROW expression SEMI?
    ;


/* ============================================================================
 * 19. UNSAFE SOURCE CONSTRUCT
 * ========================================================================== */

/**
 * "unsafe" is a language-level source construct.
 *
 * This does NOT permit Rust unsafe code.
 * The Rust compiler implementation itself remains safe Rust.
 */
unsafeStatement
    : UNSAFE blockExpression
    ;


/* ============================================================================
 * 20. EXPRESSION STATEMENTS
 * ========================================================================== */

expressionStatement
    : expression SEMI?
    ;


/* ============================================================================
 * 21. BLOCKS
 * ========================================================================== */

blockExpression
    : LBRACE blockElement* RBRACE
    ;

blockElement
    : attribute
    | statement
    ;


/* ============================================================================
 * 22. EXPRESSIONS
 *
 * Precedence follows grammar/spec/syntax.md:
 *
 *   assignment
 *   range
 *   logical OR
 *   logical AND
 *   bitwise OR
 *   bitwise XOR
 *   bitwise AND
 *   equality
 *   comparison
 *   shift
 *   additive
 *   multiplicative
 *   prefix
 *   call
 *   index
 *   member
 *
 * The grammar does not assign target-specific meaning to operators.
 * Semantic analysis owns operator typing and lowering.
 * ========================================================================== */

expression
    : assignmentExpression
    ;

assignmentExpression
    : rangeExpression
      (
          assignmentOperator
          assignmentExpression
      )?
    ;

assignmentOperator
    : ASSIGN
    | PLUS_ASSIGN
    | MINUS_ASSIGN
    | STAR_ASSIGN
    | SLASH_ASSIGN
    ;

rangeExpression
    : logicalOrExpression
      (
          rangeOperator
          logicalOrExpression?
      )?
    ;

rangeOperator
    : DOT_DOT
    | DOT_DOT_EQ
    ;

logicalOrExpression
    : logicalAndExpression
      (
          (LOGICAL_OR | OR)
          logicalAndExpression
      )*
    ;

logicalAndExpression
    : bitwiseOrExpression
      (
          (LOGICAL_AND | AND)
          bitwiseOrExpression
      )*
    ;

bitwiseOrExpression
    : bitwiseXorExpression
      (
          BIT_OR
          bitwiseXorExpression
      )*
    ;

bitwiseXorExpression
    : bitwiseAndExpression
      (
          CARET
          bitwiseAndExpression
      )*
    ;

bitwiseAndExpression
    : equalityExpression
      (
          BIT_AND
          equalityExpression
      )*
    ;

equalityExpression
    : comparisonExpression
      (
          (EQUALS | NOT_EQUALS)
          comparisonExpression
      )*
    ;

comparisonExpression
    : shiftExpression
      (
          (
              LESS_THAN
            | LESS_THAN_EQUAL
            | GREATER_THAN
            | GREATER_THAN_EQUAL
          )
          shiftExpression
      )*
    ;

shiftExpression
    : additiveExpression
      (
          (LEFT_SHIFT | RIGHT_SHIFT)
          additiveExpression
      )*
    ;

additiveExpression
    : multiplicativeExpression
      (
          (PLUS | MINUS)
          multiplicativeExpression
      )*
    ;

multiplicativeExpression
    : prefixExpression
      (
          (STAR | SLASH | MODULO)
          prefixExpression
      )*
    ;

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

postfixExpression
    : primaryExpression postfixPart*
    ;

postfixPart
    : callExpression
    | indexExpression
    | memberExpression
    | questionPostfix
    ;

callExpression
    : LPAREN argumentList? RPAREN
    ;

indexExpression
    : LBRACKET expression RBRACKET
    ;

memberExpression
    : DOT identifier
    | DOUBLE_COLON identifier
    ;

questionPostfix
    : QUESTION_MARK
    ;

argumentList
    : argument (COMMA argument)* COMMA?
    ;

argument
    : expression
    ;


/* ============================================================================
 * 23. PRIMARY EXPRESSIONS
 * ========================================================================== */

primaryExpression
    : literal
    | identifierExpression
    | qualifiedIdentifierExpression
    | selfExpression
    | superExpression
    | parenthesizedExpression
    | arrayExpression
    | tupleExpression
    | structExpression
    | blockExpression
    | ifExpression
    | matchExpression
    | loopExpression
    | closureExpression
    | quantumExpression
    | nanoExpression
    | rememberExpression
    | languageExpression
    ;

identifierExpression
    : identifier
    ;

qualifiedIdentifierExpression
    : identifier
      (DOUBLE_COLON identifier)+
    ;

selfExpression
    : SELF
    ;

superExpression
    : SUPER
    ;

parenthesizedExpression
    : LPAREN expression RPAREN
    ;

arrayExpression
    : LBRACKET
      (
          expression
          (COMMA expression)* COMMA?
      )?
      RBRACKET
    ;

tupleExpression
    : LPAREN expression COMMA expression (COMMA expression)* COMMA? RPAREN
    ;

structExpression
    : identifier
      LBRACE
      structExpressionField
      (COMMA structExpressionField)*
      COMMA?
      RBRACE
    ;

structExpressionField
    : identifier COLON expression
    ;


/* ============================================================================
 * 24. CONDITIONAL EXPRESSIONS
 * ========================================================================== */

ifExpression
    : IF expression blockExpression
      (ELSE IF expression blockExpression)*
      (ELSE blockExpression)?
    ;

matchExpression
    : MATCH expression LBRACE matchArm* RBRACE
    ;

loopExpression
    : LOOP blockExpression
    ;


/* ============================================================================
 * 25. CLOSURES
 * ========================================================================== */

closureExpression
    : PIPE closureParameters? PIPE
      returnType?
      expression
    ;

closureParameters
    : closureParameter (COMMA closureParameter)*
    ;

closureParameter
    : MUT? identifier (COLON typeExpression)?
    ;


/* ============================================================================
 * 26. LITERALS
 * ========================================================================== */

literal
    : integerLiteral
    | floatLiteral
    | stringLiteral
    | charLiteral
    | booleanLiteral
    | nullLiteral
    | quantumLiteral
    ;

integerLiteral
    : INTEGER
    ;

floatLiteral
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
    | NULL
    ;

quantumLiteral
    : PIPE quantumBasis KET_CLOSE
    ;

quantumBasis
    : ZERO
    | ONE
    | PLUS
    | MINUS
    ;


/* ============================================================================
 * 27. PATTERNS
 * ========================================================================== */

pattern
    : wildcardPattern
    | identifierPattern
    | literalPattern
    | tuplePattern
    | arrayPattern
    | structPattern
    | enumPattern
    | referencePattern
    ;

wildcardPattern
    : UNDERSCORE
    ;

identifierPattern
    : MUT? identifier
    ;

literalPattern
    : literal
    ;

tuplePattern
    : LPAREN pattern (COMMA pattern)+ COMMA? RPAREN
    ;

arrayPattern
    : LBRACKET
      (
          pattern (COMMA pattern)* COMMA?
      )?
      RBRACKET
    ;

structPattern
    : identifier
      LBRACE
      structPatternField
      (COMMA structPatternField)*
      COMMA?
      RBRACE
    ;

structPatternField
    : identifier
      (COLON pattern)?
    ;

enumPattern
    : qualifiedIdentifierExpression
      (
          LPAREN patternList? RPAREN
        | LBRACE structPatternField* RBRACE
      )?
    ;

referencePattern
    : AMPERSAND MUT? pattern
    ;

patternList
    : pattern (COMMA pattern)* COMMA?
    ;


/* ============================================================================
 * 28. TYPE EXPRESSIONS
 *
 * Type syntax is structural.
 * Type validity belongs to semantic/type analysis.
 *
 * No machine width is encoded here.
 * ========================================================================== */

typeExpression
    : functionType
    ;

functionType
    : typeUnion
      (
          THIN_ARROW typeExpression
      )?
    ;

typeUnion
    : typeIntersection
      (PIPE typeIntersection)*
    ;

typeIntersection
    : typePostfix
      (AMPERSAND typePostfix)*
    ;

typePostfix
    : typePrimary
      typeModifier*
    ;

typeModifier
    : QUESTION_MARK
    | LBRACKET RBRACKET
    ;

typePrimary
    : primitiveType
    | namedType
    | genericType
    | tupleType
    | arrayType
    | referenceType
    | neverType
    | unitType
    | dependentType
    ;

primitiveType
    : VOID
    | INT
    | FLOAT_TYPE
    | BOOL
    | STR
    | STRING_TYPE
    | CHAR_TYPE
    | QUBIT
    ;

namedType
    : identifier
    | qualifiedIdentifierExpression
    ;

genericType
    : identifier
      LESS_THAN
      typeExpression
      (COMMA typeExpression)*
      GREATER_THAN
    ;

tupleType
    : LPAREN
      (
          typeExpression
          (COMMA typeExpression)+
          COMMA?
      )?
      RPAREN
    ;

arrayType
    : LBRACKET typeExpression RBRACKET
    ;

referenceType
    : AMPERSAND MUT? typeExpression
    ;

neverType
    : NEVER
    ;

unitType
    : LPAREN RPAREN
    ;


/**
 * Extensible type-system forms.
 *
 * These are intentionally syntactic.
 * Whether a particular dependent/linear/affine construct is valid
 * belongs to semantic/type checking.
 */
dependentType
    : PI_SYMBOL dependentBinder? typeExpression
    | SIGMA_SYMBOL dependentBinder? typeExpression
    | LINEAR typeExpression
    | AFFINE typeExpression
    ;

dependentBinder
    : LPAREN identifier COLON typeExpression RPAREN
    ;


/* ============================================================================
 * 29. QUANTUM DOMAIN
 *
 * Quantum syntax expresses intent.
 *
 * It does NOT encode:
 *   - physical qubit count
 *   - physical qubit IDs
 *   - hardware topology
 *   - vendor gate set
 *   - pulse implementation
 *   - QPU architecture
 *   - simulator implementation
 *
 * Those are semantic/lowering/backend concerns.
 * ========================================================================== */

quantumDeclaration
    : QUANTUM
      (
          CIRCUIT identifier
          genericParameters?
          parameterClause?
          blockExpression

        | blockExpression

        | quantumStatement
      )
    ;

parameterClause
    : LPAREN parameterList? RPAREN
    ;

quantumExpression
    : QUANTUM
      blockExpression
    | quantumOperationExpression
    ;

quantumStatement
    : APPLY quantumOperation targetSpecification
      SEMI?
    | ENTANGLE quantumTargetList
      SEMI?
    | quantumMeasurementStatement
    | quantumResetStatement
    ;

quantumOperationExpression
    : APPLY quantumOperation targetSpecification
    ;

quantumOperation
    : expression
    ;

targetSpecification
    : TO quantumTargetList
    | FROM quantumTargetList TO quantumTargetList
    ;

quantumTargetList
    : quantumTarget
      (COMMA quantumTarget)*
    ;

quantumTarget
    : expression
    ;

quantumMeasurementStatement
    : MEASURE quantumTargetList
      (ARROW expression)?
      SEMI?
    ;

quantumResetStatement
    : RESET quantumTargetList
      SEMI?
    ;


/* ============================================================================
 * 30. NANO / AGENT DOMAIN
 *
 * Annotation semantics remain outside the parser.
 * ========================================================================== */

nanoAgentDeclaration
    : NANO
      (
          AGENT identifier
          genericParameters?
          parameterClause?
          blockExpression
        | AGENT blockExpression
        | blockExpression
      )
    ;

nanoExpression
    : NANO
      (
          AGENT identifier
          parameterClause?
          blockExpression
        | blockExpression
      )
    ;

nanoAnnotation
    : AT identifier
      (LPAREN argumentList? RPAREN)?
    ;


/* ============================================================================
 * 31. SANKOFA / TEMPORAL DOMAIN
 * ========================================================================== */

rememberStatement
    : REMEMBER expression SEMI?
    ;

rememberExpression
    : REMEMBER expression
    ;

wisdomStatement
    : WISDOM expression blockExpression?
    ;

languageExpression
    : LANGUAGE expression
    ;


/* ============================================================================
 * 32. EFFECTS
 * ========================================================================== */

effectDeclaration
    : EFFECT identifier
      genericParameters?
      whereClause?
      blockExpression
    ;

handleStatement
    : HANDLE blockExpression
      (
          WITH
          effectHandler+
      )?
    ;

effectHandler
    : identifier
      parameterClause?
      FAT_ARROW
      blockExpression
    ;


/* ============================================================================
 * 33. LANGUAGE / META FACILITIES
 * ========================================================================== */

languageDeclaration
    : LANGUAGE identifier
      blockExpression
    ;

macroDeclaration
    : MACRO identifier
      genericParameters?
      parameterClause?
      blockExpression
    ;


/* ============================================================================
 * 34. OMNIVERSAL / EXTENSIBLE SYSTEM FORMS
 *
 * These are parsed structurally rather than giving the parser knowledge of
 * every future subsystem.
 *
 * Semantic registries determine whether a named facility is valid.
 * ========================================================================== */

omniversalStatement
    : OMNIVERSAL identifier
      (
          parameterClause?
          blockExpression
        | expression
      )
    ;

specialSystemStatement
    : (
          ASI
        | AESI
        | ASESI
        | ADMIN
        | PAYMENT
        | GATEWAY
        | GRAPHICS
        | VIDEO
        | ADJUST
        | VERSIONING
        | COPYRIGHT
        | NOTICE
        | LEGAL
        | ACTION
        | TAILOR
        | BUSINESS
      )
      (
          parameterClause?
          blockExpression
        | expression
      )
    ;


/* ============================================================================
 * 35. IDENTIFIERS
 *
 * IDENTIFIER is emitted by ZamaniLexer.g4.
 *
 * The parser deliberately does not reproduce Unicode lexical rules.
 * ========================================================================== */

identifier
    : IDENTIFIER
    ;


/* ============================================================================
 * 36. OPTIONAL SEMICOLON SUPPORT
 *
 * Semicolon handling is deliberately syntactic.
 * Automatic statement termination must not be inferred from hardware,
 * execution targets, or downstream representations.
 * ========================================================================== */

semicolon
    : SEMI
    ;
/*
 * Zamani — Universal Computing Language
 * File: grammar/core/source-unit.g4
 *
 * Purpose
 * -------
 * Defines the source-unit boundary of Zamani.
 *
 * This grammar is intentionally concerned with SOURCE STRUCTURE:
 *
 *   source text
 *       -> sourceUnit
 *       -> source items
 *       -> declaration / statement ownership
 *       -> EOF
 *
 * It does NOT define:
 *   - lexical tokens
 *   - identifiers
 *   - literals
 *   - expressions
 *   - types
 *   - quantum IR
 *   - classical IR
 *   - hardware topology
 *   - scheduling
 *   - optimization
 *   - runtime behavior
 *   - machine capacity
 *
 * Those concerns belong to their respective grammar/domain layers.
 *
 * Architectural rule
 * ------------------
 * Grammar describes source-level syntax only.
 *
 * Semantic analysis, capability resolution, target selection,
 * resource discovery, lowering, optimization, routing, scheduling,
 * hardware mapping, execution and runtime adaptation happen after
 * parsing.
 *
 * POCO-REAF
 * ---------
 * The source unit MUST NOT encode accidental machine limits.
 *
 * Therefore this grammar contains no:
 *   - maximum qubit count
 *   - maximum CPU count
 *   - maximum GPU count
 *   - maximum node count
 *   - fixed topology
 *   - fixed device ID
 *   - fixed memory capacity
 *   - fixed register count
 *   - fixed accelerator count
 *   - fixed deployment size
 *
 * Physical/resource requirements are expressed elsewhere through
 * semantic resource/capability/constraint models.
 *
 * Ownership
 * ---------
 * OWNED:
 *   - source-unit boundary
 *   - source-item ordering
 *   - source-level documentation placement
 *   - source-level declarations
 *   - source-level statements
 *   - EOF termination
 *
 * NOT OWNED:
 *   - lexical definitions
 *   - names
 *   - paths
 *   - declarations' internal syntax
 *   - expressions
 *   - types
 *   - domain-specific syntax
 *   - AST construction
 *   - semantic validation
 *   - IR construction
 *
 * Integration contract
 * --------------------
 * This grammar is consumed by the root Zamani parser grammar.
 *
 * The intended architecture is:
 *
 *   lexer
 *      |
 *      v
 *   source-unit
 *      |
 *      v
 *   AST
 *      |
 *      v
 *   semantic analysis
 *      |
 *      +--------------------+
 *      |                    |
 *      v                    v
 *   classical IR       quantum::ir
 *                           |
 *                           v
 *                     QEC / ZQN / optimization
 *                           |
 *                           v
 *                  routing / scheduling / HAL
 *                           |
 *                           v
 *                        runtime
 *
 * The source grammar MUST NOT depend on any of those downstream
 * implementation layers.
 *
 * Rust integration
 * ----------------
 * Rust parser/frontend implementations using this grammar MUST:
 *
 *   - use Rust 1.97 or 1.97.1
 *   - use safe Rust only
 *   - contain no unsafe blocks
 *   - contain no unsafe functions
 *   - contain no target-specific assumptions in parsing
 *   - preserve source spans
 *   - preserve deterministic parse structure
 *   - distinguish syntax errors from semantic errors
 *
 * The generated ANTLR parser is an implementation artifact and MUST
 * NOT become the canonical semantic representation.
 *
 * Versioning
 * ----------
 * Syntax versioning belongs to the language-version/core-versioning
 * layer. This grammar only establishes the syntactic envelope in which
 * version declarations/metadata may occur.
 */

parser grammar ZamaniSourceUnitParser;

options {
    /*
     * The concrete token vocabulary is supplied by the authoritative
     * Zamani lexer/token grammar.
     *
     * The exact token vocabulary file is intentionally not duplicated
     * here. This prevents source-unit syntax from becoming coupled to
     * a second lexer definition.
     */
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * SOURCE UNIT
 * ============================================================================
 *
 * A source unit is the smallest independently parseable Zamani source
 * boundary.
 *
 * It is intentionally unbounded by machine/resource dimensions.
 *
 * The parser consumes zero or more source items and requires EOF.
 *
 * Cardinality is therefore determined by available input/resources,
 * not by a language-level constant.
 */

sourceUnit
    : sourceHeader?
      sourceItem*
      EOF
    ;

/*
 * ============================================================================
 * SOURCE HEADER
 * ============================================================================
 *
 * Header information belongs to the source-level contract rather than
 * to a particular computational domain.
 *
 * A header may carry:
 *
 *   - documentation
 *   - language/version information
 *   - source-level metadata
 *   - source-level attributes
 *
 * It MUST NOT encode an implicit hardware selection.
 */

sourceHeader
    : sourceDocumentation*
      sourceMetadata*
      sourceAttribute*
    ;

/*
 * ============================================================================
 * SOURCE ITEMS
 * ============================================================================
 *
 * A source item is deliberately an ordered syntactic unit.
 *
 * Domain-specific grammar modules plug into this boundary through
 * declaration and statement ownership.
 *
 * This rule is the integration point that prevents the source-unit
 * grammar from having to be rewritten whenever a new computing domain
 * is introduced.
 */

sourceItem
    : sourceDocumentation
    | sourceMetadata
    | sourceAttribute
    | declaration
    | statement
    ;

/*
 * ============================================================================
 * SOURCE DOCUMENTATION
 * ============================================================================
 *
 * Documentation is syntactic metadata and does not change computation.
 *
 * The lexer owns DOC_COMMENT.
 */

sourceDocumentation
    : DOC_COMMENT+
    ;

/*
 * ============================================================================
 * SOURCE METADATA
 * ============================================================================
 *
 * Metadata is intentionally generic here.
 *
 * Its semantic interpretation belongs to the metadata/annotation layer.
 */

sourceMetadata
    : metadataAnnotation
    ;

/*
 * ============================================================================
 * SOURCE ATTRIBUTES
 * ============================================================================
 *
 * Attributes are attached syntactically to the following source item.
 *
 * The semantic meaning of an attribute is NOT determined by this grammar.
 *
 * This is essential for extensibility:
 *
 *   @quantum
 *   @hardware
 *   @resource
 *   @compile
 *   @runtime
 *   @security
 *
 * may be introduced by the appropriate semantic/domain registry without
 * making source-unit responsible for those meanings.
 */

sourceAttribute
    : annotation
    ;

/*
 * ============================================================================
 * DECLARATION BOUNDARY
 * ============================================================================
 *
 * This is an integration rule.
 *
 * It is intentionally not a second implementation of declarations.
 *
 * The root grammar/domain grammar must provide the authoritative
 * declaration alternatives.
 *
 * The declaration contract must eventually include, where supported:
 *
 *   - modules
 *   - imports
 *   - exports
 *   - packages
 *   - functions
 *   - types
 *   - constants
 *   - variables
 *   - classes/traits/interfaces
 *   - classical declarations
 *   - quantum declarations
 *   - hybrid declarations
 *   - HDL declarations
 *   - hardware declarations
 *   - distributed declarations
 *   - AI/data declarations
 *   - dialect declarations
 *   - macros
 *   - metaprogramming declarations
 *
 * None of those domains are owned by source-unit.g4.
 */

declaration
    : moduleDeclaration
    | importDeclaration
    | exportDeclaration
    | packageDeclaration
    | functionDeclaration
    | typeDeclaration
    | constantDeclaration
    | variableDeclaration
    | domainDeclaration
    ;

/*
 * ============================================================================
 * STATEMENT BOUNDARY
 * ============================================================================
 *
 * Statements are delegated to the statement grammar.
 *
 * Keeping this as an integration boundary prevents source-unit from
 * becoming coupled to every future control-flow feature.
 */

statement
    : blockStatement
    | declarationStatement
    | expressionStatement
    | controlStatement
    | returnStatement
    | loopControlStatement
    | effectStatement
    | domainStatement
    ;

/*
 * ============================================================================
 * GENERIC DECLARATION INTEGRATION
 * ============================================================================
 *
 * These rules represent integration contracts.
 *
 * They are intentionally narrow enough that the source-unit grammar does
 * not become the owner of the internal syntax of those constructs.
 *
 * The final composed Zamani grammar may replace these forwarding rules
 * with imported rules where the ANTLR composition layout requires it.
 */

/*
 * Module system
 */
moduleDeclaration
    : MODULE qualifiedName moduleBody?
    ;

moduleBody
    : LEFT_BRACE sourceItem* RIGHT_BRACE
    ;

/*
 * Imports
 *
 * Import syntax belongs to the module/import grammar.
 */
importDeclaration
    : IMPORT importTarget importAlias? SEMICOLON
    ;

importTarget
    : qualifiedName
    | STRING_LITERAL
    ;

importAlias
    : AS IDENTIFIER
    ;

/*
 * Exports
 */
exportDeclaration
    : EXPORT exportTarget SEMICOLON
    ;

exportTarget
    : qualifiedName
    | STAR
    ;

/*
 * Package declaration
 */
packageDeclaration
    : PACKAGE qualifiedName SEMICOLON
    ;

/*
 * Function declaration boundary.
 *
 * The body and parameter/type syntax belong to functions.g4.
 */
functionDeclaration
    : functionModifier*
      FN
      IDENTIFIER
      functionGenericParameters?
      LEFT_PAREN
      functionParameters?
      RIGHT_PAREN
      functionReturnType?
      functionEffects?
      block
    ;

functionModifier
    : PUBLIC
    | PRIVATE
    | INTERNAL
    | ASYNC
    | CONST
    | EXTERN
    ;

functionGenericParameters
    : LESS_THAN genericParameter (COMMA genericParameter)* GREATER_THAN
    ;

genericParameter
    : IDENTIFIER
    ;

functionParameters
    : functionParameter (COMMA functionParameter)*
    ;

functionParameter
    : IDENTIFIER (COLON typeExpression)?
    ;

functionReturnType
    : ARROW typeExpression
    ;

functionEffects
    : WITH EFFECTS LEFT_BRACE
      effectName (COMMA effectName)*
      RIGHT_BRACE
    ;

effectName
    : qualifiedName
    ;

/*
 * Type declaration boundary.
 */
typeDeclaration
    : TYPE IDENTIFIER typeParameters? typeDefinition
    ;

typeParameters
    : LESS_THAN genericParameter (COMMA genericParameter)* GREATER_THAN
    ;

typeDefinition
    : EQUALS typeExpression SEMICOLON
    ;

/*
 * Constants
 */
constantDeclaration
    : CONST IDENTIFIER
      (COLON typeExpression)?
      EQUALS expression
      SEMICOLON
    ;

/*
 * Variables
 *
 * The source-unit grammar permits declarations but does not impose
 * capacity limits or target-specific storage rules.
 */
variableDeclaration
    : variableKeyword IDENTIFIER
      (COLON typeExpression)?
      variableInitializer?
      SEMICOLON
    ;

variableKeyword
    : LET
    | VAR
    | CONST
    ;

variableInitializer
    : EQUALS expression
    ;

/*
 * ============================================================================
 * DOMAIN DECLARATION BOUNDARY
 * ============================================================================
 *
 * Domain syntax is intentionally represented as a forwarding boundary.
 *
 * A domain may be:
 *
 *   classical
 *   quantum
 *   hybrid
 *   hdl
 *   hardware
 *   distributed
 *   ai
 *   data
 *   networking
 *   security
 *   future/registered dialect
 *
 * Source-unit does NOT interpret these domains.
 *
 * Semantic ownership belongs to their respective domain layers.
 */

domainDeclaration
    : DOMAIN domainKind IDENTIFIER domainBody?
    ;

domainKind
    : IDENTIFIER
    ;

domainBody
    : LEFT_BRACE sourceItem* RIGHT_BRACE
    ;

/*
 * ============================================================================
 * STATEMENT INTEGRATION
 * ============================================================================
 */

blockStatement
    : block
    ;

block
    : LEFT_BRACE statement* RIGHT_BRACE
    ;

declarationStatement
    : variableDeclaration
    | constantDeclaration
    ;

expressionStatement
    : expression SEMICOLON
    ;

controlStatement
    : IF expression block
      (ELSE IF expression block)*
      (ELSE block)?
    | WHILE expression block
    | DO block WHILE expression SEMICOLON
    | FOR IDENTIFIER IN expression block
    | MATCH expression matchBody
    ;

matchBody
    : LEFT_BRACE matchCase+ RIGHT_BRACE
    ;

matchCase
    : CASE pattern
      (WHEN expression)?
      FAT_ARROW
      (expression | block)
    ;

loopControlStatement
    : BREAK SEMICOLON
    | CONTINUE SEMICOLON
    ;

returnStatement
    : RETURN expression? SEMICOLON
    ;

effectStatement
    : HANDLE expression block
    ;

domainStatement
    : DOMAIN domainKind expression SEMICOLON
    ;

/*
 * ============================================================================
 * PATTERN BOUNDARY
 * ============================================================================
 */

pattern
    : IDENTIFIER
    | UNDERSCORE
    | literal
    | tuplePattern
    | arrayPattern
    | typedPattern
    ;

tuplePattern
    : LEFT_PAREN pattern (COMMA pattern)+ RIGHT_PAREN
    ;

arrayPattern
    : LEFT_BRACKET
      (pattern (COMMA pattern)*)?
      RIGHT_BRACKET
    ;

typedPattern
    : IDENTIFIER COLON typeExpression
    ;

/*
 * ============================================================================
 * TYPE / EXPRESSION FORWARDING BOUNDARIES
 * ============================================================================
 *
 * These are deliberately abstract integration rules.
 *
 * The source-unit grammar does not define the complete type system or
 * expression precedence hierarchy.
 *
 * That belongs to:
 *
 *   grammar/types/*
 *   grammar/expressions/*
 *
 * This prevents duplicate expression/type definitions.
 */

typeExpression
    : qualifiedName
    | typeConstructor
    ;

typeConstructor
    : qualifiedName
      LESS_THAN
      typeArgumentList?
      GREATER_THAN
    ;

typeArgumentList
    : typeExpression (COMMA typeExpression)*
    ;

expression
    : primaryExpression
    | unaryExpression
    | binaryExpression
    | callExpression
    | memberExpression
    | indexExpression
    | lambdaExpression
    ;

primaryExpression
    : IDENTIFIER
    | literal
    | LEFT_PAREN expression RIGHT_PAREN
    ;

unaryExpression
    : unaryOperator expression
    ;

unaryOperator
    : PLUS
    | MINUS
    | NOT
    | BIT_NOT
    ;

binaryExpression
    : expression binaryOperator expression
    ;

binaryOperator
    : PLUS
    | MINUS
    | STAR
    | SLASH
    | PERCENT
    | EQUAL_EQUAL
    | NOT_EQUAL
    | LESS_THAN
    | LESS_EQUAL
    | GREATER_THAN
    | GREATER_EQUAL
    | AND
    | OR
    | BIT_AND
    | BIT_OR
    | BIT_XOR
    | SHIFT_LEFT
    | SHIFT_RIGHT
    ;

callExpression
    : expression
      LEFT_PAREN
      argumentList?
      RIGHT_PAREN
    ;

argumentList
    : expression (COMMA expression)*
    ;

memberExpression
    : expression DOT IDENTIFIER
    ;

indexExpression
    : expression
      LEFT_BRACKET
      expression
      RIGHT_BRACKET
    ;

lambdaExpression
    : PIPE
      functionParameters?
      PIPE
      expression
    ;

/*
 * ============================================================================
 * LITERALS
 * ============================================================================
 *
 * Literal token ownership belongs to lexer/literals.g4.
 */

literal
    : INTEGER_LITERAL
    | DECIMAL_LITERAL
    | STRING_LITERAL
    | CHARACTER_LITERAL
    | BOOLEAN_LITERAL
    | QUANTUM_LITERAL
    | HARDWARE_LITERAL
    | DURATION_LITERAL
    | SIZE_LITERAL
    ;

/*
 * ============================================================================
 * QUALIFIED NAMES
 * ============================================================================
 *
 * Name syntax is ultimately owned by core/names.g4 and core/paths.g4.
 *
 * This forwarding form exists only to make the source-unit integration
 * contract explicit.
 */

qualifiedName
    : IDENTIFIER
      (NAMESPACE_SEPARATOR IDENTIFIER)*
    ;

/*
 * ============================================================================
 * ANNOTATIONS
 * ============================================================================
 *
 * Annotation syntax is generic.
 *
 * Interpretation belongs to the annotation/metadata registry.
 */

annotation
    : AT IDENTIFIER
      (
          LEFT_PAREN annotationArguments? RIGHT_PAREN
      )?
    ;

annotationArguments
    : annotationArgument (COMMA annotationArgument)*
    ;

annotationArgument
    : IDENTIFIER
    | literal
    | expression
    ;

metadataAnnotation
    : annotation
    ;

/*
 * ============================================================================
 * SOURCE-LEVEL INVARIANTS
 * ============================================================================
 *
 * These are grammar invariants, not semantic actions.
 *
 * 1. sourceUnit always terminates at EOF.
 * 2. sourceItem has no machine-size-dependent repetition bound.
 * 3. sourceUnit does not reference hardware capacity.
 * 4. sourceUnit does not reference quantum device topology.
 * 5. sourceUnit does not construct IR.
 * 6. sourceUnit does not perform semantic validation.
 * 7. sourceUnit does not select a backend.
 * 8. sourceUnit does not schedule execution.
 * 9. sourceUnit does not perform optimization.
 * 10. sourceUnit remains extensible through domain/declaration boundaries.
 *
 * ============================================================================
 * END
 * ============================================================================
 */
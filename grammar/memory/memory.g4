/*
 * ============================================================================
 * Zamani — Canonical Memory Grammar
 * File: grammar/memory/memory.g4
 *
 * Grammar: Memory
 * Kind: ANTLR4 parser grammar
 * Baseline: Rust 1.97 / 1.97.1, Rust 2021, safe Rust only
 *
 * PURPOSE
 * -------
 * Defines target-independent source syntax for memory intent and memory
 * constructs. It is a parser component, not a memory implementation.
 *
 * OWNS
 * ----
 * memoryConstruct and the reusable memory-domain rules declared below:
 * memory operations, places, annotations, memory intent, regions, spaces,
 * resource references, requirements, constraints, preferences, hints,
 * policies, bounds, ranges, and extension points.
 *
 * DOES NOT OWN
 * ------------
 * Lexer tokens, identifiers, qualified names, expressions, types, ownership
 * checking, borrow checking, lifetime inference, allocation algorithms,
 * physical addresses, hardware discovery, placement, scheduling, routing,
 * canonical IR, quantum::ir, QEC, ZQN, HAL, or runtime behavior.
 *
 * COMPOSITION
 * -----------
 * Imported as Memory by grammar/antlr/ZamaniParser.g4.
 * Consumes shared rules supplied by the composed parser:
 * identifier, qualifiedName, expression, expressionList, typeExpression.
 *
 * POCO-REAF
 * ---------
 * No universal resource limits or physical topology are encoded here.
 * Counts and sizes are expressions with program meaning, not compiler limits.
 * Actual feasibility is determined by semantic/resource analysis and the
 * available target resources.
 *
 * This file contains no actions or semantic predicates.
 * ============================================================================
 */

parser grammar Memory;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * PUBLIC ENTRY POINTS
 * -------------------
 * The parser composition root decides where these are legal.
 * A statement terminator is owned by the surrounding statement grammar.
 * This grammar deliberately does not require a semicolon here.
 */
memoryConstruct
    : memoryStatement
    | memoryExpression
    | memoryAnnotation
    ;

memoryStatement
    : memoryOperationStatement
    ;

memoryOperationStatement
    : memoryOperation
    ;

memoryExpression
    : memoryOperation
    ;

/*
 * DECLARATION / BINDING FRAGMENT
 * ------------------------------
 * The universal declaration grammar decides where this fragment is allowed.
 * The grammar preserves syntax; semantic analysis checks type, ownership,
 * lifetime, and region compatibility.
 */
memoryDeclaration
    : memoryBinding
    ;

memoryBinding
    : memoryOwnershipQualifier?
      memoryPlace
      memoryTypeAnnotation?
      memoryLifetimeClause?
      memorySpaceClause?
    ;

memoryTypeAnnotation
    : COLON typeExpression
;

/*
 * ANNOTATIONS
 * -----------
 * Annotation names are open-world qualified names, not a closed list of
 * memory technologies or vendor-specific tokens.
 */
memoryAnnotation
    : AT memoryQualifiedName memoryAnnotationArguments?
    ;

memoryAnnotationArguments
    : LPAREN memoryArgumentList? RPAREN
;

/*
 * OPEN-WORLD OPERATIONS
 * ---------------------
 * Operation identity is represented by a qualified name. Semantic
 * registration determines whether an operation is recognized and valid.
 */
memoryOperation
    : memoryQualifiedName LPAREN memoryArgumentList? RPAREN
    ;

memoryExtensionOperation
    : memoryQualifiedName LPAREN memoryArgumentList? RPAREN
    ;

memoryDomainExtension
    : memoryQualifiedName (LPAREN memoryArgumentList? RPAREN)?
    ;

memoryQualifiedName
    : qualifiedName
    ;

memoryPath
    : memoryQualifiedName
;

memoryQualification
    : memoryQualifiedName
;

/*
 * ARGUMENTS
 * ---------
 * General expression syntax remains owned by expressions/.
 * A named argument's name is an identifier, not a reserved memory keyword.
 */
memoryArgumentList
    : memoryArgument (COMMA memoryArgument)* COMMA?
    ;

memoryArgument
    : memoryNamedArgument
    | expression
    ;

memoryNamedArgument
    : identifier ASSIGN expression
;

memoryExpressionList
    : expression (COMMA expression)* COMMA?
;

/*
 * MEMORY PLACES
 * -------------
 * A memory place is a source-level semantic location, never a physical
 * address. Indexing and member access use the canonical expression/name
 * rules. Dereference syntax is represented structurally; its validity is
 * checked semantically.
 */
memoryPlace
    : memoryPlaceBase memoryPlaceSuffix*
    ;

memoryPlaceBase
    : memoryQualifiedName
    | parenthesizedMemoryPlace
    ;

memoryPlaceSuffix
    : memoryMemberSuffix
    | memoryIndexSuffix
    | memoryDereferenceSuffix
    ;

memoryMemberSuffix
    : DOT identifier
    ;

memoryIndexSuffix
    : LBRACKET expressionList RBRACKET
    ;

memoryDereferenceSuffix
    : STAR
    ;

parenthesizedMemoryPlace
    : LPAREN memoryPlace RPAREN
;

/*
 * OWNERSHIP / BORROW / LIFETIME SYNTAX
 * ------------------------------------
 * These rules preserve source intent only. They do not perform ownership,
 * aliasing, lifetime, or borrow checking.
 */
memoryOwnershipQualifier
    : LINEAR
    | AFFINE
    ;

memoryBorrow
    : AMPERSAND memoryLifetimePrefix? MUT? memoryPlace
    ;

memoryLifetimePrefix
    : APOSTROPHE identifier
    ;

memoryLifetime
    : APOSTROPHE identifier
    ;

memoryLifetimeClause
    : memoryLifetime
;

/*
 * REGIONS AND SPACES
 * ------------------
 * Region and space names remain open-world. Neither implies a physical
 * memory bank, NUMA node, page, cache, device, or address.
 */
memoryRegion
    : memoryRegionReference
    ;

memoryRegionReference
    : memoryQualifiedName
    ;

memoryRegionClause
    : IN memoryRegion
    ;

memorySpace
    : memoryQualifiedName
    ;

memorySpaceClause
    : memorySpace
;

/*
 * RESOURCE REFERENCES
 * -------------------
 * These are symbolic references. Universal resource semantics remain owned
 * by resources/, not by this grammar.
 */
memoryResource
    : memoryQualifiedName
    ;

memoryResourceClause
    : memoryResource
    ;

memoryResourceSpecification
    : memoryResourceClause memoryResourceValue?
    ;

memoryResourceValue
    : ASSIGN expression
;

/*
 * REQUIREMENTS, CONSTRAINTS, PREFERENCES, HINTS
 * ---------------------------------------------
 * Keep these categories separate:
 *
 * REQUIRES: mandatory semantic condition.
 * CONSTRAINT: condition restricting acceptable realizations.
 * PREFER: non-mandatory preference.
 * HINT: advisory information that must not change program meaning.
 *
 * Their satisfiability and enforcement are semantic/resource-analysis tasks.
 */
memoryRequirement
    : REQUIRES memoryRequirementExpressionList
    ;

memoryRequirementExpressionList
    : expression (COMMA expression)* COMMA?
    ;

memoryConstraint
    : memoryConstraintKeyword memoryConstraintExpressionList
    ;

memoryConstraintKeyword
    : CONSTRAINT
    ;

memoryConstraintExpressionList
    : expression (COMMA expression)* COMMA?
    ;

memoryPreference
    : memoryPreferenceKeyword memoryPreferenceExpressionList
    ;

memoryPreferenceKeyword
    : PREFER
    ;

memoryPreferenceExpressionList
    : expression (COMMA expression)* COMMA?
    ;

memoryHint
    : memoryHintKeyword memoryHintExpressionList
    ;

memoryHintKeyword
    : HINT
    ;

memoryHintExpressionList
    : expression (COMMA expression)* COMMA?
;

/*
 * POLICIES AND INTENT
 * -------------------
 * Policy identifiers are open-world semantic names. A policy clause does
 * not select an allocator or physical memory device.
 */
memoryPolicy
    : memoryQualifiedName
    ;

memoryPolicyClause
    : WITH memoryPolicy
    ;

memoryIntent
    : memoryOwnershipQualifier?
      memoryTypeAnnotation?
      memoryLifetimeClause?
      memorySpaceClause?
      memoryRegionClause?
      memoryResourceSpecification*
      memoryRequirement*
      memoryConstraint*
      memoryPreference*
      memoryHint*
      memoryPolicyClause?
    ;

memoryPolicyBundle
    : memoryRequirement*
      memoryConstraint*
      memoryPreference*
      memoryHint*
      memoryPolicyClause?
    ;

memorySpecification
    : memoryIntent memoryPolicyBundle
;

/*
 * TARGETS, BOUNDS, AND RANGES
 * ---------------------------
 * A target is a source-level place/space/region. Bounds are arbitrary
 * expressions: literal, symbolic, generic, computed, or runtime-derived.
 */
memoryTarget
    : memoryPlace
    | memorySpace
    | memoryRegion
    ;

memoryBound
    : expression
    ;

memoryRange
    : memoryBound DOT_DOT memoryBound
    | memoryBound DOT_DOT_EQ memoryBound
;

/*
 * EXTENSION METADATA
 * ------------------
 * Metadata remains open-world. It must be validated by the dialect/semantic
 * registry; parsing it does not authorize arbitrary behavior.
 */
memoryExtensionMetadata
    : AT memoryQualifiedName
      (LPAREN memoryExpressionList? RPAREN)?
;
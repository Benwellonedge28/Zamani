/**
 * Zamani Programming Language
 *
 * File:
 *     grammar/declarations/enums.g4
 *
 * Role:
 *     Canonical modular ANTLR4 parser grammar for enum declarations.
 *
 * Architectural position:
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     declaration parser
 *       |
 *       +--> enums.g4
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     semantic/type analysis
 *       |
 *       v
 *     canonical semantic IR
 *       |
 *       +--> classical lowering
 *       +--> quantum lowering
 *       +--> HDL/hardware lowering
 *       +--> distributed lowering
 *       +--> future domains
 *
 * Ownership:
 *     This file owns the concrete syntax of enum declarations and
 *     enum variants.
 *
 * Does NOT own:
 *     - lexical definitions;
 *     - identifiers;
 *     - type expressions;
 *     - generic-parameter semantics;
 *     - visibility semantics;
 *     - struct-field semantics;
 *     - symbol resolution;
 *     - type checking;
 *     - discriminant assignment;
 *     - enum memory layout;
 *     - ABI representation;
 *     - optimization;
 *     - scheduling;
 *     - routing;
 *     - hardware;
 *     - quantum hardware;
 *     - QEC;
 *     - ZQN;
 *     - runtime execution;
 *     - target-specific representation.
 *
 * POCO-REAF:
 *     Enum syntax describes source-level data semantics.
 *     It must not encode the machine on which the enum is executed.
 *
 * Scalability:
 *     No fixed maximum number of:
 *       - enums;
 *       - generic parameters;
 *       - variants;
 *       - fields;
 *       - nesting depth;
 *       - program declarations.
 *
 *     Any operational parser/compiler limits belong to explicit
 *     compiler/resource policies, never to this grammar.
 *
 * Rust integration baseline:
 *     Rust 1.97 / Rust 1.97.1.
 *
 * Safety:
 *     The Rust implementation must use no unsafe code.
 *
 * IMPORTANT:
 *     This file is parser-only.
 *     It contains no embedded Rust actions.
 */

parser grammar ZamaniDeclarationEnums;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * ENUM DECLARATION
 * ========================================================================== */

/**
 * EnumDeclaration
 *
 * Canonical source form:
 *
 *     enum Name {
 *         Variant,
 *         Other,
 *     }
 *
 * Generic form:
 *
 *     enum Result<T, E> {
 *         Ok(T),
 *         Err(E),
 *     }
 *
 * Constrained generic form:
 *
 *     enum Value<T>
 *     where
 *         T: SomeConstraint
 *     {
 *         Value(T),
 *         Empty,
 *     }
 *
 * Visibility belongs to the declaration-prefix layer.
 *
 * Generic parameters and where clauses belong to their shared grammar
 * modules and are reused here.
 *
 * This rule deliberately does not resolve the meaning of the type
 * expressions or constraints.
 */
enumDeclaration
    : visibilityModifier?
      ENUM
      identifier
      genericParameters?
      whereClause?
      enumBody
    ;


/* ============================================================================
 * ENUM BODY
 * ========================================================================== */

/**
 * EnumBody
 *
 * An enum may contain zero or more variants.
 *
 * An empty enum is syntactically valid.
 *
 * Whether an empty enum is semantically useful or permitted by a particular
 * language profile is decided by semantic validation, not by the parser.
 */
enumBody
    : LBRACE
      enumVariant*
      RBRACE
    ;


/* ============================================================================
 * ENUM VARIANTS
 * ========================================================================== */

/**
 * EnumVariant
 *
 * Supported forms:
 *
 *     Unit
 *
 *     Tuple(T)
 *
 *     Pair(A, B)
 *
 *     Record {
 *         field: Type,
 *     }
 *
 * A trailing comma is optional.
 *
 * The grammar preserves source ordering.
 */
enumVariant
    : identifier
      enumVariantKind?
      COMMA?
    ;


/* ============================================================================
 * VARIANT KIND
 * ========================================================================== */

/**
 * EnumVariantKind
 *
 * A variant is either:
 *
 *     1. unit-like;
 *     2. tuple-like;
 *     3. struct-like.
 *
 * Unit variants have no EnumVariantKind.
 */
enumVariantKind
    : enumTupleVariant
    | enumStructVariant
    ;


/* ============================================================================
 * TUPLE VARIANTS
 * ========================================================================== */

/**
 * Tuple variant.
 *
 * Examples:
 *
 *     Some(T)
 *     Pair(A, B)
 *     QuantumState(State)
 *     Measurement(Result, Metadata)
 *
 * There is deliberately no fixed maximum number of fields.
 */
enumTupleVariant
    : LPAREN
      enumTupleFields?
      RPAREN
    ;


/**
 * Tuple variant fields.
 *
 * Empty tuple variants are syntactically rejected by this rule because:
 *
 *     Variant()
 *
 * carries no information beyond a unit variant.
 *
 * The canonical source form for a unit variant is:
 *
 *     Variant
 *
 * This avoids two syntactic representations for the same source meaning.
 */
enumTupleFields
    : typeExpression
      (COMMA typeExpression)*
      COMMA?
    ;


/* ============================================================================
 * STRUCT-LIKE VARIANTS
 * ========================================================================== */

/**
 * Struct-like enum variant.
 *
 * Example:
 *
 *     Error {
 *         code: ErrorCode,
 *         message: String,
 *     }
 *
 * Field ownership remains with the shared struct-field grammar.
 *
 * This prevents enum variants from inventing a second field syntax.
 */
enumStructVariant
    : LBRACE
      enumStructFields?
      RBRACE
    ;


/**
 * Struct-like variant fields.
 *
 * Empty struct-like variants are syntactically valid:
 *
 *     Marker {}
 *
 * Semantic analysis decides whether such a declaration is desirable or
 * whether it should be normalized to a unit variant.
 *
 * The parser must preserve the source form.
 */
enumStructFields
    : structField*
    ;


/* ============================================================================
 * ENUM-SPECIFIC FIELD ADAPTER
 * ========================================================================== */

/**
 * Enum variant fields use the same field syntax as struct declarations.
 *
 * This rule exists only as a named integration boundary.
 *
 * It must not duplicate structField.
 */
enumVariantField
    : structField
    ;


/* ============================================================================
 * ENUM DECLARATION LIST
 * ========================================================================== */

/**
 * A reusable sequence for declaration aggregators.
 *
 * The top-level declarations grammar normally consumes individual
 * declarations, so this rule is provided only for grammar composition.
 */
enumDeclarations
    : enumDeclaration+
    ;
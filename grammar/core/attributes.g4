/**
 * Zamani Programming Language
 *
 * File: grammar/core/attributes.g4
 *
 * Purpose:
 *   Defines the syntax of generic Zamani attributes.
 *
 * Architectural role:
 *   This grammar is a syntax-only foundation. It provides a uniform,
 *   extensible mechanism for attaching structured metadata/configuration
 *   to declarations, expressions, statements, types, modules, functions,
 *   quantum constructs, HDL constructs, hardware descriptions, resources,
 *   compilation units, and other grammar-owned constructs.
 *
 * Ownership:
 *   - Attribute attachment syntax.
 *   - Attribute names.
 *   - Attribute arguments.
 *   - Attribute argument forms.
 *   - Attribute lists/groups.
 *   - Structured attribute values.
 *
 * Does NOT own:
 *   - Identifier lexical definitions.
 *   - Keywords.
 *   - Literal lexical definitions.
 *   - Name resolution.
 *   - Module resolution.
 *   - Filesystem paths.
 *   - Hardware addresses.
 *   - Device identifiers.
 *   - Quantum IR.
 *   - Classical IR.
 *   - QEC semantics.
 *   - ZQN semantics.
 *   - Scheduling semantics.
 *   - Routing semantics.
 *   - Optimization semantics.
 *   - Runtime behavior.
 *   - Capability negotiation.
 *   - Resource discovery.
 *   - Attribute validation rules specific to a domain.
 *
 * Integration boundary:
 *
 *   lexer
 *      |
 *      v
 *   names / paths / qualified-names
 *      |
 *      v
 *   attributes.g4
 *      |
 *      +----> declarations
 *      +----> types
 *      +----> functions
 *      +----> modules
 *      +----> effects
 *      +----> memory
 *      +----> concurrency
 *      +----> classical
 *      +----> quantum
 *      +----> hybrid
 *      +----> hdl
 *      +----> hardware
 *      +----> distributed
 *      +----> ai
 *      +----> data
 *      +----> networking
 *      +----> security
 *      +----> resources
 *      +----> compile
 *      +----> execution
 *      +----> interoperability
 *      +----> dialects
 *
 * The grammar never depends on those downstream domains.
 *
 * Rust:
 *   This grammar contains no Rust implementation code and introduces no
 *   unsafe requirements. Generated parser integration must remain compatible
 *   with the repository's Rust 1.97 / 1.97.1 toolchain and antlr-rust setup.
 */

parser grammar Attributes;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * --------------------------------------------------------------------------
 * Attribute attachment
 * --------------------------------------------------------------------------
 *
 * Attribute syntax is deliberately generic.
 *
 * The lexer is responsible for deciding which concrete token represents
 * the attribute introducer and delimiters.
 *
 * The parser consumes those canonical tokens rather than redefining lexical
 * syntax here.
 */

/**
 * One or more attributes attached to a grammar construct.
 *
 * Multiple attributes are allowed without imposing an artificial maximum.
 */
attributeList
    : attribute+
    ;

/**
 * One attribute.
 *
 * Example conceptual forms:
 *
 *   @name
 *   @name(...)
 *
 * The exact spelling of the introducer is owned by the canonical lexer.
 */
attribute
    : ATTRIBUTE_START attributeName attributeArguments?
    ;

/*
 * --------------------------------------------------------------------------
 * Attribute names
 * --------------------------------------------------------------------------
 *
 * Attribute names are semantic names, not hard-coded domain keywords.
 *
 * A qualified attribute name allows namespaces/dialects/extensions without
 * requiring this grammar to know every future attribute namespace.
 */
attributeName
    : identifier
    | qualifiedAttributeName
    ;

/**
 * Qualified attribute names.
 *
 * The concrete qualification delimiter is supplied by the canonical lexer.
 *
 * This rule deliberately accepts arbitrary depth rather than imposing a
 * fixed namespace depth.
 */
qualifiedAttributeName
    : identifier ATTRIBUTE_NAMESPACE_SEPARATOR identifier
      (
          ATTRIBUTE_NAMESPACE_SEPARATOR identifier
      )*
    ;

/*
 * --------------------------------------------------------------------------
 * Attribute arguments
 * --------------------------------------------------------------------------
 */

/**
 * Parenthesized attribute argument list.
 *
 * Zero or more arguments are valid.
 */
attributeArguments
    : ATTRIBUTE_LPAREN attributeArgumentList? ATTRIBUTE_RPAREN
    ;

/**
 * Comma-separated arguments.
 *
 * No fixed argument count is imposed.
 */
attributeArgumentList
    : attributeArgument
      (
          ATTRIBUTE_COMMA attributeArgument
      )*
    ;

/**
 * An attribute argument may be positional or named.
 *
 * Named arguments allow future extensibility without changing the grammar
 * every time an attribute acquires an additional property.
 */
attributeArgument
    : attributeNamedArgument
    | attributeValue
    ;

/**
 * Named attribute argument.
 *
 * Example conceptual form:
 *
 *   @attribute(option = value)
 */
attributeNamedArgument
    : identifier ATTRIBUTE_ASSIGN attributeValue
    ;

/*
 * --------------------------------------------------------------------------
 * Attribute values
 * --------------------------------------------------------------------------
 *
 * Values remain syntactically generic.
 *
 * Their semantic interpretation belongs to the consumer of the attribute.
 * For example, a quantum attribute may eventually describe a quantum
 * capability, while a compiler attribute may describe a compilation hint.
 *
 * This grammar does not decide what those meanings are.
 */
attributeValue
    : attributeLiteral
    | attributeNameReference
    | attributeCollection
    | attributeExpression
    ;

/**
 * Literal attribute values.
 *
 * Literal tokens are owned by the canonical lexer.
 */
attributeLiteral
    : integerLiteral
    | floatingLiteral
    | stringLiteral
    | characterLiteral
    | booleanLiteral
    | nullLiteral
    ;

/**
 * Reference to a named semantic value.
 *
 * This permits attributes to refer to declarations, constants, symbolic
 * values, capabilities, types, policies, etc., without hard-coding those
 * domains into the attribute grammar.
 */
attributeNameReference
    : identifier
    | qualifiedAttributeName
    ;

/*
 * --------------------------------------------------------------------------
 * Collections
 * --------------------------------------------------------------------------
 *
 * Collections are syntax-level structures.
 *
 * Their semantic interpretation is deliberately deferred.
 */
attributeCollection
    : attributeArray
    | attributeObject
    ;

/**
 * Ordered attribute values.
 *
 * No fixed element count.
 */
attributeArray
    : ATTRIBUTE_LBRACKET
      (
          attributeValue
          (
              ATTRIBUTE_COMMA attributeValue
          )*
      )?
      ATTRIBUTE_RBRACKET
    ;

/**
 * Key/value attribute object.
 *
 * This permits structured attributes without creating a separate grammar
 * for every future domain.
 */
attributeObject
    : ATTRIBUTE_LBRACE
      (
          attributeObjectEntry
          (
              ATTRIBUTE_COMMA attributeObjectEntry
          )*
      )?
      ATTRIBUTE_RBRACE
    ;

/**
 * One key/value entry.
 */
attributeObjectEntry
    : attributeObjectKey ATTRIBUTE_ASSIGN attributeValue
    ;

/**
 * Object keys may be identifiers or strings.
 *
 * String keys permit externally defined schemas while identifiers keep
 * ordinary source notation concise.
 */
attributeObjectKey
    : identifier
    | stringLiteral
    ;

/*
 * --------------------------------------------------------------------------
 * Attribute expressions
 * --------------------------------------------------------------------------
 *
 * This rule intentionally provides a narrow, non-recursive hook for
 * expression integration.
 *
 * The actual expression grammar owns expression semantics.
 *
 * The token/rule boundary must be resolved by the parser composition layer
 * so that attributes do not create a second expression grammar.
 */
attributeExpression
    : ATTRIBUTE_EXPRESSION_START attributeExpressionBody ATTRIBUTE_EXPRESSION_END
    ;

/**
 * Attribute expression body.
 *
 * This is an integration boundary rather than a duplicate expression
 * implementation.
 *
 * The canonical expression parser should replace/compose this rule when
 * Zamani.g4 assembles the complete parser.
 */
attributeExpressionBody
    : attributeExpressionAtom
      (
          attributeExpressionOperator attributeExpressionAtom
      )*
    ;

/**
 * Minimal expression atoms accepted by this standalone grammar.
 *
 * Complete expression semantics remain owned by expressions/*.g4.
 */
attributeExpressionAtom
    : attributeLiteral
    | attributeNameReference
    | attributeCollection
    ;

/**
 * Operators inside attribute expressions.
 *
 * The canonical lexer supplies the operator token.
 */
attributeExpressionOperator
    : ATTRIBUTE_EXPRESSION_OPERATOR
    ;

/*
 * --------------------------------------------------------------------------
 * Reusable forms
 * --------------------------------------------------------------------------
 */

/**
 * Optional attribute list.
 *
 * Useful for grammar components that permit attributes but do not require
 * them.
 */
optionalAttributeList
    : attributeList?
    ;

/**
 * Attribute group.
 *
 * A group provides an explicit syntactic boundary for attributes that need
 * to be attached as a unit.
 */
attributeGroup
    : ATTRIBUTE_GROUP_START attributeList? ATTRIBUTE_GROUP_END
    ;

/**
 * Zero or more attributes.
 *
 * Kept separate from optionalAttributeList so downstream grammars can use
 * whichever cardinality communicates their contract most clearly.
 */
zeroOrMoreAttributes
    : attribute*
    ;
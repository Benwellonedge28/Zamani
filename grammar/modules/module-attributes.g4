parser grammar ModuleAttributes;

options {
    tokenVocab = ZamaniTokens;
}

import QualifiedNames;

/*
 * ============================================================================
 * Zamani Module Attributes Grammar
 * ============================================================================
 *
 * Ownership
 * ---------
 * This grammar owns the SYNTAX of attributes attached to modules.
 *
 * It does NOT own:
 *   - module declarations
 *   - imports
 *   - exports
 *   - package declarations
 *   - namespaces
 *   - dependencies
 *   - visibility
 *   - hardware discovery
 *   - target selection
 *   - resource allocation
 *   - scheduling
 *   - runtime state
 *   - quantum IR
 *   - classical IR
 *   - semantic validation
 *   - attribute meaning
 *
 * Semantic interpretation belongs to the compiler/semantic-analysis layer.
 *
 * Scalability
 * -----------
 * There are deliberately no grammar-level limits on:
 *   - number of attributes
 *   - attribute arguments
 *   - nesting depth
 *   - identifier length
 *   - number of modules
 *   - number of targets
 *   - number of resources
 *   - number of devices
 *   - number of qubits
 *   - number of CPUs/cores/threads
 *
 * Machine-specific facts must be represented by the appropriate target,
 * capability, requirement, constraint, resource, scheduling, hardware, or
 * deployment subsystem rather than encoded here.
 *
 * Determinism
 * -----------
 * This grammar contains no actions, predicates, semantic side effects,
 * filesystem access, network access, runtime calls, or target-specific code.
 *
 * Rust
 * ----
 * Generated parser code is intended for the repository's Rust target,
 * including Rust 1.97 / 1.97.1.
 *
 * ============================================================================
 */


/*
 * --------------------------------------------------------------------------
 * Module attribute list
 * --------------------------------------------------------------------------
 *
 * A module may have zero or more attributes.
 *
 * The grammar deliberately does not impose a maximum number of attributes.
 */
moduleAttributes
    : moduleAttribute*
    ;


/*
 * --------------------------------------------------------------------------
 * Single module attribute
 * --------------------------------------------------------------------------
 *
 * Canonical form:
 *
 *     @name
 *
 *     @name(...)
 *
 *     @qualified::name
 *
 *     @qualified::name(...)
 *
 * Attribute arguments are parsed structurally but their meaning is not
 * interpreted by this grammar.
 */
moduleAttribute
    : AT qualifiedName
      moduleAttributeArguments?
    ;


/*
 * --------------------------------------------------------------------------
 * Attribute arguments
 * --------------------------------------------------------------------------
 *
 * Parentheses are syntactic delimiters.
 *
 * The actual expression grammar owns expression syntax.
 *
 * To avoid a dependency cycle between module attributes and the complete
 * expression grammar, this grammar intentionally provides a structural
 * argument representation.
 *
 * The semantic layer must subsequently validate the contents against the
 * registered attribute definition.
 */
moduleAttributeArguments
    : LPAREN moduleAttributeArgumentList? RPAREN
    ;


moduleAttributeArgumentList
    : moduleAttributeArgument
      (
          COMMA
          moduleAttributeArgument
      )*
    ;


/*
 * --------------------------------------------------------------------------
 * Attribute argument
 * --------------------------------------------------------------------------
 *
 * Named arguments:
 *
 *     @attribute(name = value)
 *
 * Positional arguments:
 *
 *     @attribute(value)
 *
 * The parser accepts both forms.
 *
 * Mixing positional and named arguments is syntactically legal here;
 * semantic validation decides whether a particular attribute definition
 * permits that combination.
 */
moduleAttributeArgument
    : moduleAttributeNamedArgument
    | moduleAttributePositionalArgument
    ;


moduleAttributeNamedArgument
    : moduleAttributeArgumentName ASSIGN moduleAttributeArgumentValue
    ;


moduleAttributePositionalArgument
    : moduleAttributeArgumentValue
    ;


moduleAttributeArgumentName
    : IDENTIFIER
    ;


/*
 * --------------------------------------------------------------------------
 * Attribute argument value
 * --------------------------------------------------------------------------
 *
 * The value is intentionally structural rather than tied to a particular
 * domain.
 *
 * Supported values:
 *   - qualified names
 *   - identifiers
 *   - literals
 *   - nested attributes
 *   - structured lists
 *
 * Domain-specific semantic validation must occur after parsing.
 *
 * IMPORTANT:
 * This grammar does not define resource limits or hardware semantics.
 */
moduleAttributeArgumentValue
    : moduleAttributeQualifiedValue
    | moduleAttributeIdentifierValue
    | moduleAttributeLiteralValue
    | moduleAttributeListValue
    | moduleAttributeNestedValue
    ;


moduleAttributeQualifiedValue
    : qualifiedName
    ;


moduleAttributeIdentifierValue
    : IDENTIFIER
    ;


/*
 * --------------------------------------------------------------------------
 * Literal values
 * --------------------------------------------------------------------------
 *
 * Literal token names are centralized in ZamaniTokens.
 *
 * No literal is assigned domain-specific meaning here.
 */
moduleAttributeLiteralValue
    : STRING_LITERAL
    | INTEGER_LITERAL
    | FLOAT_LITERAL
    | TRUE
    | FALSE
    | NULL
    ;


moduleAttributeListValue
    : LBRACKET
      moduleAttributeListElements?
      RBRACKET
    ;


moduleAttributeListElements
    : moduleAttributeArgumentValue
      (
          COMMA
          moduleAttributeArgumentValue
      )*
    ;


moduleAttributeNestedValue
    : AT qualifiedName
      moduleAttributeArguments?
    ;
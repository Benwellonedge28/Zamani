/*
 * Zamani Universal Programming Language
 *
 * File:
 *     grammar/core/annotations.g4
 *
 * Purpose:
 *     Defines the syntax of Zamani annotations.
 *
 * Architectural role:
 *     Annotations are source-level, structured markers that may be attached
 *     to language constructs. This grammar defines their syntax only.
 *
 * Ownership:
 *     - Annotation marker syntax
 *     - Annotation names
 *     - Annotation argument syntax
 *     - Positional and named annotation arguments
 *     - Structured annotation values
 *     - Annotation lists
 *     - Nested annotation values
 *
 * Does NOT own:
 *     - Attribute semantics
 *     - Metadata semantics
 *     - Pragmas
 *     - Compiler directives
 *     - Capabilities
 *     - Requirements
 *     - Constraints
 *     - Resource discovery
 *     - Hardware discovery
 *     - Quantum semantics
 *     - QEC semantics
 *     - ZQN semantics
 *     - Classical IR
 *     - quantum::ir
 *     - Scheduling
 *     - Routing
 *     - Optimization
 *     - Runtime behavior
 *     - Device identifiers
 *     - Physical topology
 *     - Resource limits
 *
 * Design principle:
 *
 *     Syntax describes the annotation.
 *     Semantic analysis decides what the annotation means.
 *
 * This separation is essential for POCO-REAF:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Annotation syntax must remain independent of the target machine.
 *
 * Rust:
 *     Generated parser code must remain compatible with the repository's
 *     Rust 1.97 / 1.97.1 toolchain and antlr-rust integration.
 *
 * Safety:
 *     This grammar contains no embedded Rust actions, predicates, unsafe code,
 *     filesystem access, network access, process execution, or target-specific
 *     behavior.
 *
 * IMPORTANT:
 *     The token names referenced below are integration contracts with the
 *     canonical Zamani lexer. They must be defined exactly once by that lexer.
 *
 * Expected lexer contract:
 *
 *     ANNOTATION_MARKER
 *     IDENTIFIER
 *     DOUBLE_COLON
 *     LPAREN
 *     RPAREN
 *     LBRACKET
 *     RBRACKET
 *     LBRACE
 *     RBRACE
 *     COMMA
 *     ASSIGN
 *
 *     STRING_LITERAL
 *     CHARACTER_LITERAL
 *     INTEGER_LITERAL
 *     FLOAT_LITERAL
 *     BOOLEAN_LITERAL
 *     NULL_LITERAL
 *
 * If the repository uses different canonical token names, the lexer contract
 * must be mapped there rather than introducing duplicate lexer definitions
 * inside this grammar.
 */

parser grammar Annotations;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ---------------------------------------------------------------------------
 * Top-level annotation forms
 * ---------------------------------------------------------------------------
 *
 * An annotation is intentionally independent of where it is attached.
 *
 * Declaration grammars, module grammars, function grammars, quantum grammars,
 * HDL grammars, hardware grammars, etc. decide which syntactic constructs may
 * accept annotations.
 *
 * This file only defines what an annotation looks like.
 */


/*
 * A single annotation.
 *
 * Examples of the intended abstract shape:
 *
 *     @name
 *     @name(...)
 *     @namespace::name
 *     @namespace::name(...)
 *
 * The actual semantic meaning of the name is resolved outside this grammar.
 */
annotation
    : ANNOTATION_MARKER annotationName annotationArguments?
    ;


/*
 * One or more annotations.
 *
 * This rule intentionally has no fixed maximum.
 *
 * Therefore:
 *
 *     @a @b @c ...
 *
 * is limited only by parser/runtime/resource availability rather than an
 * artificial grammar constant.
 */
annotationList
    : annotation+
    ;


/*
 * Zero or more annotations.
 *
 * Useful for declaration consumers that allow an optional annotation prefix.
 */
optionalAnnotations
    : annotation*
    ;


/*
 * ---------------------------------------------------------------------------
 * Annotation names
 * ---------------------------------------------------------------------------
 *
 * Names are composed from the repository's canonical qualified-name syntax.
 *
 * No domain-specific annotation names are enumerated here.
 *
 * Consequently, this grammar does not need to be modified when Zamani gains
 * new annotation namespaces for:
 *
 *     quantum
 *     classical
 *     hdl
 *     hardware
 *     distributed
 *     ai
 *     security
 *     networking
 *     resources
 *     future dialects
 *
 * Semantic validation is responsible for deciding whether a particular
 * annotation name exists and whether it is legal in a given context.
 */


/*
 * A qualified annotation name.
 *
 * This rule deliberately uses the canonical qualified-name boundary.
 *
 * If QualifiedNames exports `qualifiedName`, that rule is the authoritative
 * implementation and this rule is merely the annotation-specific entry point.
 */
annotationName
    : qualifiedName
    ;


/*
 * ---------------------------------------------------------------------------
 * Annotation arguments
 * ---------------------------------------------------------------------------
 *
 * Arguments are optional and may be:
 *
 *     positional
 *     named
 *
 * Both forms may coexist unless semantic validation imposes a stricter
 * contract.
 */


/*
 * Parenthesized annotation argument list.
 *
 * Empty argument lists are legal:
 *
 *     @name()
 */
annotationArguments
    : LPAREN annotationArgumentList? RPAREN
    ;


/*
 * Comma-separated annotation arguments.
 *
 * A trailing comma is intentionally accepted.
 *
 * This improves generated-code ergonomics and reduces unnecessary source
 * churn when arguments are added.
 *
 * Example:
 *
 *     @example(
 *         first,
 *         second,
 *     )
 *
 * There is no fixed argument-count limit.
 */
annotationArgumentList
    : annotationArgument (COMMA annotationArgument)* COMMA?
    ;


/*
 * An annotation argument may be either:
 *
 *     value
 *
 * or:
 *
 *     name = value
 *
 * The grammar permits both because the distinction between legal positional
 * and named forms is semantic rather than syntactic.
 */
annotationArgument
    : annotationValue
    | annotationArgumentName ASSIGN annotationValue
    ;


/*
 * Named argument key.
 *
 * Keep this deliberately narrower than a fully qualified name.
 *
 * Namespace qualification belongs to the annotation itself; argument keys
 * are local names inside that annotation's schema.
 */
annotationArgumentName
    : IDENTIFIER
    ;


/*
 * ---------------------------------------------------------------------------
 * Annotation values
 * ---------------------------------------------------------------------------
 *
 * Annotation values are deliberately compile-time/static structures.
 *
 * This grammar does NOT accept arbitrary runtime expressions.
 *
 * That prevents annotations from accidentally becoming a hidden execution
 * language.
 *
 * If Zamani later needs compile-time expressions inside annotations, that
 * feature should be introduced through an explicit compile-time expression
 * contract rather than silently making annotation values runtime-dependent.
 */
annotationValue
    : annotationScalarValue
    | annotationName
    | annotationListValue
    | annotationMapValue
    ;


/*
 * Scalar values.
 *
 * The canonical lexer owns literal spelling and lexical validation.
 */
annotationScalarValue
    : STRING_LITERAL
    | CHARACTER_LITERAL
    | INTEGER_LITERAL
    | FLOAT_LITERAL
    | BOOLEAN_LITERAL
    | NULL_LITERAL
    ;


/*
 * ---------------------------------------------------------------------------
 * Structured list values
 * ---------------------------------------------------------------------------
 *
 * Examples:
 *
 *     @example([a, b, c])
 *     @example(["a", "b", "c"])
 *
 * Lists may contain nested lists/maps because annotationValue is recursive.
 *
 * No fixed nesting depth is encoded here.
 *
 * Practical limits, if ever required, belong to parser/resource configuration
 * rather than source grammar constants.
 */
annotationListValue
    : LBRACKET annotationValueList? RBRACKET
    ;


annotationValueList
    : annotationValue (COMMA annotationValue)* COMMA?
    ;


/*
 * ---------------------------------------------------------------------------
 * Structured map values
 * ---------------------------------------------------------------------------
 *
 * Examples:
 *
 *     @example({mode = "portable"})
 *
 * Map keys are local identifiers.
 *
 * Maps are intentionally semantic-neutral.
 *
 * The annotation consumer decides:
 *
 *     - which keys are recognized
 *     - which value types are valid
 *     - whether duplicate keys are legal
 *     - whether ordering has semantic meaning
 */
annotationMapValue
    : LBRACE annotationMapEntryList? RBRACE
    ;


annotationMapEntryList
    : annotationMapEntry (COMMA annotationMapEntry)* COMMA?
    ;


annotationMapEntry
    : annotationArgumentName ASSIGN annotationValue
    ;


/*
 * ---------------------------------------------------------------------------
 * Optional annotation groups
 * ---------------------------------------------------------------------------
 *
 * These helper rules are intentionally provided so downstream grammar files
 * can consume annotations without redefining their syntax.
 */


/*
 * Annotation prefix attached to a syntactic construct.
 *
 * Consumers decide what may follow this prefix.
 *
 * Examples of consumers:
 *
 *     declarations
 *     functions
 *     types
 *     modules
 *     quantum operations
 *     HDL modules
 *     hardware declarations
 *     distributed services
 *     AI declarations
 *
 * This grammar does not decide any of those attachment points.
 */
annotationPrefix
    : annotationList
    ;


/*
 * Optional annotation prefix.
 */
optionalAnnotationPrefix
    : optionalAnnotations
    ;
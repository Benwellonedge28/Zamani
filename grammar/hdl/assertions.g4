/*
 * ============================================================================
 * Zamani Programming Language
 *
 * File: grammar/hdl/assertions.g4
 * Grammar: HdlAssertions
 * Kind: ANTLR4 parser grammar
 *
 * Compatibility:
 *   Rust 1.97 or later
 *   Rust 2021
 *   Safe Rust only
 *
 * Authority:
 *   grammar/DESIGN.md
 *   grammar/spec/hdl.md
 *
 * Composition:
 *   grammar/hdl/hdl.g4
 *   grammar/hdl/verification.g4
 *
 * ============================================================================
 * PURPOSE AND OWNERSHIP
 * ============================================================================
 *
 * This file is the authoritative owner of HDL assertion and reusable
 * property-declaration syntax.
 *
 * It owns:
 *
 *   hdlAssertion
 *   hdlAssertionBody
 *   hdlImmediateAssertion
 *   hdlPropertyAssertion
 *   hdlPropertyDeclaration
 *   hdlPropertyParameterList
 *   hdlPropertyParameter
 *   hdlPropertySpecification
 *   hdlPropertyExpression
 *   hdlPropertyReference
 *   hdlVerificationLabel
 *   hdlVerificationQualifier
 *   hdlVerificationClock
 *   hdlVerificationDisable
 *   hdlVerificationMetadata
 *   hdlVerificationArgumentList
 *   hdlVerificationArgument
 *
 * This file does not own:
 *
 *   - lexical rules, keyword definitions, or punctuation;
 *   - general expressions, identifiers, names, or types;
 *   - assumption and coverage declarations;
 *   - HDL modules, signals, clocks, resets, or timing declarations;
 *   - sequential or combinational behavior;
 *   - AST structures or semantic models;
 *   - type/effect/capability/resource checking;
 *   - formal proof, model checking, simulation, or coverage execution;
 *   - canonical IR, synthesis, optimization, routing, or scheduling;
 *   - physical hardware selection or realization.
 *
 * Assumptions and coverage are composed by HdlVerification.
 * General assertion statements outside HDL remain owned by the universal
 * statement grammar.
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * Consume only the canonical ZamaniLexer vocabulary.
 *
 * Required tokens include:
 *
 *   ASSERT PROPERTY
 *   LPAREN RPAREN LBRACE RBRACE
 *   LBRACK RBRACK
 *   COMMA COLON SEMICOLON ASSIGN DOT
 *
 * Names, literals, and expressions must come from their canonical owners.
 *
 * No lexer rules, tokens blocks, aliases, or parser-local token vocabularies
 * may be added here.
 *
 * ============================================================================
 * EXPRESSION AND NAME CONTRACT
 * ============================================================================
 *
 * The enclosing HDL composition supplies the shared adapters:
 *
 *   hdlExpression
 *   hdlQualifiedName
 *   identifier
 *   hdlTypeExpression
 *
 * These adapters must delegate to the repository's canonical expression,
 * name, and type grammars. They must not introduce a second expression or
 * type language.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * Repeated source constructs use grammar repetition and recursive structure.
 *
 * This grammar imposes no language-level maximum on:
 *
 *   - assertions or property declarations;
 *   - property parameters or arguments;
 *   - qualifiers or metadata entries;
 *   - source expression complexity;
 *   - clock domains, signals, modules, or hardware resources.
 *
 * Actual parser/compiler resource exhaustion is an implementation concern,
 * not a language-level capacity rule.
 *
 * ============================================================================
 * SECURITY AND DETERMINISM
 * ============================================================================
 *
 * No embedded Rust, semantic predicates, I/O, network access, filesystem
 * access, hardware discovery, external process execution, or solver invocation
 * is performed by this grammar.
 *
 * Identical source, lexer vocabulary, grammar version, and parser configuration
 * must produce equivalent parse trees.
 *
 * ============================================================================
 */

parser grammar HdlAssertions;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * PUBLIC ASSERTION ENTRY
 *
 * Supported forms:
 *
 *   assert (ready);
 *   assert (ready, "ready must be asserted");
 *   label: assert (ready);
 *   assert property (valid implies ready);
 *   assert property (stable(a, b)) clock(clk);
 *   assert property (active) disable(reset);
 *
 * The optional label is source-level metadata, not a hardware identifier.
 */

hdlAssertion
    : hdlVerificationLabel?
      ASSERT
      hdlAssertionBody
      SEMICOLON
    ;

hdlAssertionBody
    : hdlImmediateAssertion
    | hdlPropertyAssertion
    ;

/*
 * IMMEDIATE ASSERTIONS
 *
 * The condition and optional arguments are expressions.
 * Their types, effects, and legality are semantic concerns.
 */

hdlImmediateAssertion
    : LPAREN
      hdlExpression
      (COMMA hdlVerificationArgumentList)?
      RPAREN
    ;

/*
 * PROPERTY ASSERTIONS
 *
 * A property can be specified inline or referenced by logical name.
 * Qualifiers are structurally represented and validated downstream.
 */

hdlPropertyAssertion
    : PROPERTY
      hdlPropertySpecification
      hdlVerificationQualifier*
    ;

hdlPropertySpecification
    : LPAREN
      hdlPropertyExpression
      (COMMA hdlVerificationArgumentList)?
      RPAREN
    | hdlPropertyReference
      (LPAREN hdlVerificationArgumentList? RPAREN)?
    ;

hdlPropertyExpression
    : hdlExpression
    ;

hdlPropertyReference
    : hdlQualifiedName
    ;

/*
 * REUSABLE PROPERTY DECLARATIONS
 *
 * Examples:
 *
 *   property stable(a, b) = a == b;
 *   property ready_when_valid(valid, ready) = valid && ready;
 *   property in_range(value, lower, upper) =
 *       value >= lower && value <= upper;
 *
 * Parameter binding, scope, type checking, recursion policy, and elaboration
 * are semantic responsibilities.
 */

hdlPropertyDeclaration
    : PROPERTY
      identifier
      hdlPropertyParameterList?
      ASSIGN
      hdlPropertyExpression
      SEMICOLON
    ;

hdlPropertyParameterList
    : LPAREN
      hdlPropertyParameter
      (COMMA hdlPropertyParameter)*
      COMMA?
      RPAREN
    ;

hdlPropertyParameter
    : identifier
      (COLON hdlTypeExpression)?
    ;

/*
 * VERIFICATION QUALIFIERS
 *
 * Examples:
 *
 *   clock(clk)
 *   disable(reset)
 *   metadata(domain: "control")
 *
 * Qualifier names remain identifiers so that this grammar does not create
 * another keyword vocabulary.
 *
 * The parser accepts the generic qualifier shape. Semantic analysis determines
 * which qualifiers are defined, legal, and meaningful in a given context.
 */

hdlVerificationQualifier
    : hdlVerificationClock
    | hdlVerificationDisable
    | hdlVerificationMetadata
    ;

hdlVerificationClock
    : identifier
      LPAREN hdlExpression RPAREN
    ;

hdlVerificationDisable
    : identifier
      LPAREN hdlExpression RPAREN
    ;

hdlVerificationMetadata
    : identifier
      LPAREN hdlVerificationArgumentList? RPAREN
    ;

/*
 * VERIFICATION ARGUMENTS
 *
 * Positional:
 *
 *   assert (condition, diagnostic);
 *
 * Named:
 *
 *   assert (condition, message: "invalid state");
 *   assert (condition, severity: level, message: diagnostic);
 *
 * Names such as "message" and "severity" are not reserved by this grammar.
 * Their meaning and accepted types are defined by the semantic contract.
 */

hdlVerificationArgumentList
    : hdlVerificationArgument
      (COMMA hdlVerificationArgument)*
      COMMA?
    ;

hdlVerificationArgument
    : identifier COLON hdlExpression
    | hdlExpression
    ;

/*
 * SOURCE LABEL
 *
 * Example:
 *
 *   output_is_valid: assert (valid);
 *
 * Duplicate labels, scope, and symbol resolution are checked semantically.
 */

hdlVerificationLabel
    : identifier COLON
    ;
/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/metaprogramming/metaprogramming.g4
 *
 * Status:
 *     Production parser component
 *
 * Ownership:
 *     SOURCE-LEVEL METAPROGRAMMING SYNTAX ONLY
 *
 * Purpose:
 *     Define the syntax required for:
 *
 *       - macro declarations;
 *       - macro invocations;
 *       - macro parameters;
 *       - compile-time/meta expressions;
 *       - quotation;
 *       - splicing;
 *       - language declarations;
 *       - language-definition members;
 *       - compile-time declarations;
 *       - compile-time functions;
 *       - generated declarations;
 *       - generated expressions;
 *       - reflection requests;
 *       - specialization requests;
 *       - compile-time assertions;
 *       - compile-time configuration;
 *       - declarative compiler metadata.
 *
 * ============================================================================
 *
 * ARCHITECTURAL BOUNDARY
 * ============================================================================
 *
 * This grammar owns syntax.
 *
 * It does NOT own:
 *
 *   - macro expansion;
 *   - macro resolution;
 *   - hygiene implementation;
 *   - compile-time execution;
 *   - filesystem access;
 *   - network access;
 *   - subprocess execution;
 *   - arbitrary host-language execution;
 *   - package downloading;
 *   - compiler configuration mutation;
 *   - backend selection;
 *   - hardware discovery;
 *   - quantum-device selection;
 *   - qubit allocation;
 *   - routing;
 *   - scheduling;
 *   - optimization;
 *   - QEC;
 *   - ZQN;
 *   - canonical IR construction.
 *
 * Those concerns belong to their respective compiler/runtime subsystems.
 *
 * ============================================================================
 *
 * COMPOSITION
 * ============================================================================
 *
 * This file is intentionally a PARSER grammar component.
 *
 * It is expected to be composed by the authoritative Zamani parser grammar.
 *
 * It therefore consumes shared parser rules rather than defining competing
 * copies of:
 *
 *   - identifier;
 *   - qualifiedName;
 *   - expression;
 *   - typeExpression;
 *   - genericParameters;
 *   - parameterList;
 *   - argumentList;
 *   - block;
 *   - attributes;
 *   - visibility;
 *   - declaration.
 *
 * The final composed parser MUST provide exactly one canonical owner for each
 * shared rule.
 *
 * ============================================================================
 *
 * POCO-REAF
 * ============================================================================
 *
 * Metaprogramming MUST preserve:
 *
 *   Program Once
 *        ->
 *   Portable source semantics
 *        ->
 *   deterministic meta transformation
 *        ->
 *   canonical AST / semantic representation
 *        ->
 *   target-independent compilation
 *        ->
 *   target adaptation
 *
 * A macro MUST NOT encode permanent assumptions about:
 *
 *   - CPU count;
 *   - GPU count;
 *   - FPGA count;
 *   - QPU count;
 *   - qubit count;
 *   - machine topology;
 *   - register count;
 *   - memory size;
 *   - device identifier;
 *   - vendor;
 *   - backend;
 *   - timing grid;
 *   - hardware address.
 *
 * Such information may be supplied through semantic capability/resource
 * mechanisms when required, but it is not owned by this grammar.
 *
 * ============================================================================
 *
 * SCALABILITY
 * ============================================================================
 *
 * No finite language-level limits are imposed on:
 *
 *   - macro parameters;
 *   - macro arguments;
 *   - quoted structures;
 *   - generated declarations;
 *   - generated expressions;
 *   - language members;
 *   - specialization dimensions;
 *   - metadata entries;
 *   - reflection paths.
 *
 * Implementations may impose configurable resource budgets. Those are compiler
 * policy and MUST NOT be encoded as grammar semantics.
 *
 * ============================================================================
 *
 * SAFETY
 * ============================================================================
 *
 * Rust implementation target:
 *
 *   Rust 1.97
 *   Rust 1.97.1
 *
 * Rust implementation MUST use safe Rust.
 *
 * This grammar contains no executable Rust actions.
 *
 * A Zamani source-level `unsafe` construct, if supported elsewhere in the
 * language, does not authorize Rust `unsafe`.
 *
 * ============================================================================
 */

parser grammar Metaprogramming;

options {
    /*
     * The authoritative composed parser supplies the lexer vocabulary.
     *
     * Replace the token vocabulary name only if the repository's canonical
     * parser architecture establishes a different lexer name.
     */
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 1. METAPROGRAMMING DECLARATIONS
 * ========================================================================== */

/**
 * Top-level metaprogramming declaration.
 *
 * This rule is intended to be consumed by the canonical declaration dispatcher.
 */
metaprogrammingDeclaration
    : macroDeclaration
    | languageDeclaration
    | compileTimeFunctionDeclaration
    | compileTimeConstantDeclaration
    ;


/* ============================================================================
 * 2. MACRO DECLARATIONS
 * ========================================================================== */

/**
 * Canonical macro declaration.
 *
 * Examples:
 *
 *     macro make_value(x) {
 *         ...
 *     }
 *
 *     public macro make_type(T) {
 *         ...
 *     }
 *
 * Visibility is deliberately shared with the ordinary declaration system.
 */
macroDeclaration
    : visibilityModifier?
      MACRO
      identifier
      genericParameters?
      LPAREN macroParameterList? RPAREN
      macroReturnType?
      macroAttributes*
      macroBody
    ;


/**
 * Optional macro result category.
 *
 * The category is syntactic metadata only. Semantic validation determines
 * whether the declared category is valid for the macro body and call site.
 */
macroReturnType
    : ARROW macroResultType
    ;


macroResultType
    : META
    | TYPE
    | TOKEN
    | EXPRESSION
    | STATEMENT
    | DECLARATION
    | identifier
    ;


/**
 * Macro parameter list.
 *
 * A trailing comma is accepted.
 */
macroParameterList
    : macroParameter (COMMA macroParameter)* COMMA?
    ;


/**
 * Macro parameters support explicit categories.
 *
 * The grammar does not hard-code a finite set of implementation-specific
 * compile-time types. Unknown/custom categories can be represented through
 * identifiers and are validated semantically.
 */
macroParameter
    : macroParameterKind?
      identifier
      macroParameterType?
      macroParameterDefault?
    ;


macroParameterKind
    : META
    | TOKEN
    | EXPRESSION
    | STATEMENT
    | DECLARATION
    | TYPE
    | PATTERN
    | VALUE
    ;


macroParameterType
    : COLON typeExpression
    ;


macroParameterDefault
    : ASSIGN expression
    ;


/**
 * Macro body.
 *
 * A macro can return a source-level structure or a compile-time expression.
 *
 * The macro engine, not the parser, determines how the body is expanded.
 */
macroBody
    : block
    | expression
    ;


/**
 * Optional macro attributes.
 *
 * The actual attribute syntax is owned by the common attribute grammar.
 */
macroAttributes
    : attribute
    ;


/* ============================================================================
 * 3. MACRO INVOCATION
 * ========================================================================== */

/**
 * Statement/declaration-compatible macro invocation.
 *
 * Example:
 *
 *     make_value!(x);
 *
 * The canonical expression grammar may also use `macroExpression`.
 */
macroInvocation
    : macroPath BANG LPAREN macroArgumentList? RPAREN
    ;


/**
 * Macro invocation without parentheses is intentionally NOT accepted.
 *
 * Requiring explicit invocation punctuation prevents accidental ambiguity
 * between ordinary function calls and macro expansion.
 */
macroPath
    : identifier (DOUBLE_COLON identifier)*
    ;


/**
 * Macro argument list.
 *
 * Arguments are syntactic meta values and may therefore contain quotations,
 * splices, types, expressions, or token-oriented structures.
 */
macroArgumentList
    : macroArgument (COMMA macroArgument)* COMMA?
    ;


macroArgument
    : macroArgumentValue
    ;


macroArgumentValue
    : expression
    | metaQuote
    | metaSplice
    | metaTokenTree
    | metaType
    | metaPattern
    ;


/* ============================================================================
 * 4. EXPRESSION-LEVEL MACROS
 * ========================================================================== */

/**
 * Expression-level macro invocation.
 */
macroExpression
    : macroInvocation
    ;


/**
 * Explicit meta invocation.
 *
 * This form provides an unambiguous syntax for compile-time evaluation.
 */
metaEvaluateExpression
    : META LPAREN expression RPAREN
    ;


/* ============================================================================
 * 5. QUOTATION
 * ========================================================================== */

/**
 * Quotation creates a source-level meta value.
 *
 * Supported forms:
 *
 *     quote { ... }
 *     quote(expression)
 *
 * Quotation is data at the source/AST level. It does not execute its contents.
 */
metaQuote
    : QUOTE metaQuoteBody
    ;


metaQuoteBody
    : block
    | LPAREN expression RPAREN
    | LPAREN statementList? RPAREN
    | LBRACE metaTokenTree? RBRACE
    ;


/**
 * Explicit quoted expression.
 */
metaQuoteExpression
    : QUOTE LPAREN expression RPAREN
    ;


/**
 * Explicit quoted block.
 */
metaQuoteBlock
    : QUOTE block
    ;


/**
 * Zero or more statements inside a meta quotation.
 */
statementList
    : statement*
    ;


/* ============================================================================
 * 6. SPLICING
 * ========================================================================== */

/**
 * Splicing inserts a meta-produced source structure into a quotation.
 *
 * Example:
 *
 *     quote {
 *         let x = splice(value);
 *     }
 *
 * The grammar does not decide whether the resulting value is an expression,
 * statement, declaration, type, or token tree. That is a semantic property.
 */
metaSplice
    : SPLICE LPAREN expression RPAREN
    ;


/**
 * Short interpolation form.
 *
 * This is intentionally separate from `metaSplice` so tooling and semantic
 * analysis can distinguish explicit source splicing from ordinary interpolation.
 */
metaInterpolation
    : INTERPOLATE LPAREN expression RPAREN
    ;


/* ============================================================================
 * 7. TOKEN-TREE REPRESENTATION
 * ========================================================================== */

/**
 * Token-tree syntax is deliberately structural.
 *
 * It provides a stable escape hatch for syntax-oriented macros without
 * embedding another programming language in the grammar.
 *
 * Token-tree parsing is still bounded by the source input and compiler policy;
 * there is no language-level maximum nesting or token count.
 */
metaTokenTree
    : metaTokenTreeElement+
    ;


metaTokenTreeElement
    : metaToken
    | metaTokenGroup
    ;


metaTokenGroup
    : LPAREN metaTokenTree? RPAREN
    | LBRACK metaTokenTree? RBRACK
    | LBRACE metaTokenTree? RBRACE
    ;


metaToken
    : identifier
    | literal
    | operatorToken
    | punctuationToken
    | keywordAsMetaToken
    ;


/**
 * Operators that can safely participate in a token tree.
 *
 * The token vocabulary remains authoritative.
 */
operatorToken
    : PLUS
    | MINUS
    | STAR
    | SLASH
    | MODULO
    | EQUALS
    | NOT_EQUALS
    | LESS_THAN
    | LESS_THAN_EQUAL
    | GREATER_THAN
    | GREATER_THAN_EQUAL
    | BIT_AND
    | BIT_OR
    | CARET
    | LOGICAL_AND
    | LOGICAL_OR
    | BANG
    | QUESTION_MARK
    | ARROW
    | FAT_ARROW
    ;


/**
 * Punctuation available as meta tokens.
 */
punctuationToken
    : LPAREN
    | RPAREN
    | LBRACK
    | RBRACK
    | LBRACE
    | RBRACE
    | COMMA
    | DOT
    | COLON
    | SEMI
    | DOUBLE_COLON
    ;


/**
 * Language keywords can be represented as meta tokens when quoted.
 *
 * This avoids introducing a second lexer/token namespace.
 */
keywordAsMetaToken
    : FN
    | LET
    | CONST
    | VAR
    | TYPE
    | MODULE
    | IMPORT
    | EXPORT
    | IF
    | ELSE
    | FOR
    | WHILE
    | MATCH
    | RETURN
    | STRUCT
    | ENUM
    | TRAIT
    | IMPLEMENTS
    | CLASS
    | INTERFACE
    | MACRO
    | LANGUAGE
    | QUOTE
    | SPLICE
    | META
    ;


/* ============================================================================
 * 8. META TYPES
 * ========================================================================== */

/**
 * Type quotation/reference for compile-time type manipulation.
 */
metaType
    : TYPE LPAREN typeExpression RPAREN
    ;


/**
 * Pattern quotation/reference.
 */
metaPattern
    : PATTERN LPAREN pattern RPAREN
    ;


/* ============================================================================
 * 9. LANGUAGE DECLARATIONS
 * ========================================================================== */

/**
 * Language declaration.
 *
 * A language declaration describes a language artifact. It does not dynamically
 * replace the currently active Zamani lexer or parser.
 */
languageDeclaration
    : visibilityModifier?
      LANGUAGE
      identifier
      languageVersion?
      languageParameters?
      languageAttributes*
      languageBody
    ;


languageVersion
    : VERSION versionExpression
    ;


versionExpression
    : literal
    | qualifiedName
    | expression
    ;


languageParameters
    : LPAREN parameterList? RPAREN
    ;


languageAttributes
    : attribute
    ;


languageBody
    : LBRACE languageMember* RBRACE
    ;


/* ============================================================================
 * 10. LANGUAGE MEMBERS
 * ========================================================================== */

languageMember
    : languageGrammarDeclaration
    | languageLexerDeclaration
    | languageParserDeclaration
    | languageMacroDeclaration
    | languageTypeDeclaration
    | languageOperatorDeclaration
    | languageAttributeDeclaration
    | languageDirectiveDeclaration
    | languageCapabilityDeclaration
    | languageRequirementDeclaration
    | languageExportDeclaration
    | languageImportDeclaration
    ;


/* ============================================================================
 * 11. LANGUAGE GRAMMAR DESCRIPTORS
 * ========================================================================== */

/**
 * Grammar descriptors are DATA.
 *
 * They do not embed ANTLR implementation syntax.
 */
languageGrammarDeclaration
    : GRAMMAR identifier languageDescriptorBody?
      SEMI?
    ;


languageLexerDeclaration
    : LEXER identifier languageDescriptorBody?
      SEMI?
    ;


languageParserDeclaration
    : PARSER identifier languageDescriptorBody?
      SEMI?
    ;


languageDescriptorBody
    : LBRACE languageDescriptorEntry* RBRACE
    ;


languageDescriptorEntry
    : identifier
      (COLON metaValue)?
      SEMI?
    ;


/* ============================================================================
 * 12. LANGUAGE MACROS
 * ========================================================================== */

/**
 * Macro exported by a language definition.
 *
 * This is distinct from an ordinary macro declaration so semantic analysis can
 * preserve language ownership.
 */
languageMacroDeclaration
    : MACRO
      identifier
      genericParameters?
      LPAREN macroParameterList? RPAREN
      macroReturnType?
      macroBody
    ;


/* ============================================================================
 * 13. LANGUAGE TYPES
 * ========================================================================== */

/**
 * Language-defined type descriptor.
 *
 * This does not create a second type system.
 */
languageTypeDeclaration
    : TYPE
      identifier
      genericParameters?
      (ASSIGN typeExpression)?
      languageDescriptorBody?
      SEMI?
    ;


/* ============================================================================
 * 14. LANGUAGE OPERATORS
 * ========================================================================== */

/**
 * User-defined operator descriptor.
 *
 * The grammar permits only the canonical operator token vocabulary or a named
 * operator identifier. It does not dynamically create lexer rules.
 */
languageOperatorDeclaration
    : OPERATOR
      languageOperatorSymbol
      operatorSignature?
      operatorAttributes*
      SEMI?
    ;


languageOperatorSymbol
    : identifier
    | operatorToken
    ;


operatorSignature
    : COLON typeExpression
    ;


operatorAttributes
    : attribute
    ;


/* ============================================================================
 * 15. LANGUAGE ATTRIBUTES
 * ========================================================================== */

languageAttributeDeclaration
    : ATTRIBUTE
      identifier
      languageAttributeParameters?
      languageDescriptorBody?
      SEMI?
    ;


languageAttributeParameters
    : LPAREN parameterList? RPAREN
    ;


/* ============================================================================
 * 16. LANGUAGE DIRECTIVES
 * ========================================================================== */

/**
 * Declarative directive.
 *
 * A directive is metadata, not an imperative command.
 */
languageDirectiveDeclaration
    : DIRECTIVE
      identifier
      (ASSIGN metaValue)?
      SEMI?
    ;


/* ============================================================================
 * 17. LANGUAGE CAPABILITIES
 * ========================================================================== */

/**
 * Language definitions can declare capabilities they provide or require.
 *
 * Capability semantics are resolved by the capability subsystem.
 */
languageCapabilityDeclaration
    : PROVIDES
      capabilitySet
      SEMI?
    | REQUIRES
      capabilitySet
      SEMI?
    ;


capabilitySet
    : capabilityExpression
      (COMMA capabilityExpression)*
      COMMA?
    ;


capabilityExpression
    : capabilityPath
      capabilityArguments?
    ;


capabilityPath
    : identifier
      (DOUBLE_COLON identifier)*
    ;


capabilityArguments
    : LPAREN metaArgumentList? RPAREN
    ;


/* ============================================================================
 * 18. LANGUAGE REQUIREMENTS
 * ========================================================================== */

languageRequirementDeclaration
    : REQUIREMENT
      requirementExpression
      SEMI?
    ;


requirementExpression
    : expression
    ;


/* ============================================================================
 * 19. LANGUAGE IMPORTS / EXPORTS
 * ========================================================================== */

languageImportDeclaration
    : IMPORT
      languageImportTarget
      SEMI?
    ;


languageImportTarget
    : qualifiedName
    | STRING
    ;


languageExportDeclaration
    : EXPORT
      languageExportTarget
      SEMI?
    ;


languageExportTarget
    : qualifiedName
    | STAR
    | LBRACE languageExportList? RBRACE
    ;


languageExportList
    : languageExportSpecifier
      (COMMA languageExportSpecifier)*
      COMMA?
    ;


languageExportSpecifier
    : identifier
      (AS identifier)?
    ;


/* ============================================================================
 * 20. COMPILE-TIME FUNCTIONS
 * ========================================================================== */

/**
 * Compile-time function.
 *
 * It is still a source-level declaration. Its execution model belongs to the
 * compiler's controlled meta-evaluation subsystem.
 */
compileTimeFunctionDeclaration
    : visibilityModifier?
      COMPTIME
      FN
      identifier
      genericParameters?
      LPAREN parameterList? RPAREN
      (ARROW typeExpression)?
      compileTimeEffectClause?
      block
    ;


compileTimeEffectClause
    : WITH
      EFFECTS
      LBRACE effectNameList? RBRACE
    ;


effectNameList
    : effectName
      (COMMA effectName)*
      COMMA?
    ;


effectName
    : qualifiedName
    | identifier
    ;


/* ============================================================================
 * 21. COMPILE-TIME CONSTANTS
 * ========================================================================== */

compileTimeConstantDeclaration
    : visibilityModifier?
      COMPTIME
      CONST
      identifier
      COLON typeExpression
      ASSIGN metaValue
      SEMI
    ;


/* ============================================================================
 * 22. COMPILE-TIME CONTROL
 * ========================================================================== */

/**
 * Compile-time conditional.
 *
 * It expresses a compile-time decision but does not specify the compiler
 * implementation strategy.
 */
compileTimeIf
    : COMPTIME
      IF
      expression
      block
      (ELSE IF expression block)*
      (ELSE block)?
    ;


/**
 * Compile-time iteration.
 *
 * Resource limits are compiler policy, not syntax.
 */
compileTimeFor
    : COMPTIME
      FOR
      identifier
      IN
      expression
      block
    ;


/**
 * Compile-time assertion.
 */
compileTimeAssert
    : COMPTIME
      ASSERT
      LPAREN
      expression
      (COMMA expression)?
      RPAREN
      SEMI
    ;


/**
 * Compile-time error.
 *
 * The compiler turns this into a diagnostic. The grammar does not prescribe
 * diagnostic storage or formatting.
 */
compileTimeError
    : COMPTIME
      ERROR
      LPAREN
      expression
      RPAREN
      SEMI
    ;


/**
 * Compile-time warning.
 */
compileTimeWarning
    : COMPTIME
      WARNING
      LPAREN
      expression
      RPAREN
      SEMI
    ;


/* ============================================================================
 * 23. SPECIALIZATION
 * ========================================================================== */

/**
 * Specialization request.
 *
 * Specialization is a compiler transformation, not a new runtime execution
 * model.
 */
specializationDeclaration
    : SPECIALIZE
      specializationTarget
      specializationArguments?
      specializationWhereClause?
      SEMI?
    ;


specializationTarget
    : qualifiedName
    ;


specializationArguments
    : LT
      specializationArgument
      (COMMA specializationArgument)*
      COMMA?
      GT
    ;


specializationArgument
    : typeExpression
    | expression
    | metaValue
    ;


specializationWhereClause
    : WHERE
      expression
    ;


/* ============================================================================
 * 24. GENERATION DECLARATIONS
 * ========================================================================== */

/**
 * Explicit generated declaration.
 *
 * Generation remains declarative and semantic. The compiler decides how and
 * when generated material becomes part of the compilation unit.
 */
generateDeclaration
    : GENERATE
      generateTarget
      ASSIGN
      metaValue
      SEMI?
    ;


generateTarget
    : DECLARATION
    | EXPRESSION
    | TYPE
    | STATEMENT
    | TOKEN
    | identifier
    ;


/* ============================================================================
 * 25. REFLECTION
 * ========================================================================== */

/**
 * Compile-time reflection request.
 *
 * Reflection syntax only identifies the requested subject. It does not grant
 * access to arbitrary host resources.
 */
reflectExpression
    : REFLECT
      LPAREN
      reflectionTarget
      RPAREN
    ;


reflectionTarget
    : reflectionPath
    | typeExpression
    | expression
    ;


reflectionPath
    : identifier
      (DOUBLE_COLON identifier)*
    ;


/* ============================================================================
 * 26. META VALUES
 * ========================================================================== */

/**
 * Canonical meta value.
 *
 * This is deliberately structural and domain-neutral.
 */
metaValue
    : metaQuote
    | metaSplice
    | metaInterpolation
    | metaTokenTree
    | metaType
    | metaPattern
    | expression
    ;


/**
 * Meta argument list.
 */
metaArgumentList
    : metaValue
      (COMMA metaValue)*
      COMMA?
    ;


/* ============================================================================
 * 27. META BLOCKS
 * ========================================================================== */

/**
 * Meta block.
 *
 * A meta block is a source-level construct. Execution is controlled by the
 * compiler's meta evaluator.
 */
metaBlock
    : META
      block
    ;


/**
 * Meta statement.
 *
 * The canonical statement dispatcher may import these alternatives.
 */
metaStatement
    : compileTimeIf
    | compileTimeFor
    | compileTimeAssert
    | compileTimeError
    | compileTimeWarning
    | metaBlock
    | generateDeclaration
    | specializationDeclaration
    ;


/* ============================================================================
 * 28. MACRO EXPANSION BOUNDARY
 * ========================================================================== */

/**
 * Explicit expansion request.
 *
 * This does NOT perform expansion in the parser.
 */
expandExpression
    : EXPAND
      LPAREN
      macroInvocation
      RPAREN
    ;


/**
 * Explicit expansion target.
 */
expansionTarget
    : macroInvocation
    | qualifiedName
    ;


/* ============================================================================
 * 29. META PATHS
 * ========================================================================== */

/**
 * A meta path has no fixed namespace depth.
 */
metaPath
    : identifier
      (DOUBLE_COLON identifier)*
    ;


/* ============================================================================
 * 30. META IDENTIFIERS
 * ========================================================================== */

/**
 * Dedicated rule gives semantic tooling a stable place to recognize
 * identifiers used specifically at the meta level.
 */
metaIdentifier
    : identifier
    ;


/* ============================================================================
 * 31. META MEMBER ACCESS
 * ========================================================================== */

metaMemberAccess
    : metaPath
      (DOT identifier)*
    ;


/* ============================================================================
 * 32. META COLLECTIONS
 * ========================================================================== */

/**
 * Collection literals useful for compile-time data.
 *
 * They intentionally reuse ordinary expression structures rather than creating
 * a second collection language.
 */
metaCollection
    : LBRACK metaArgumentList? RBRACK
    | LBRACE metaArgumentList? RBRACE
    ;


/* ============================================================================
 * 33. META VALUE UNION
 * ========================================================================== */

/**
 * Broad meta expression entry point.
 *
 * This rule is useful to semantic analysis and AST construction.
 */
metaExpression
    : macroExpression
    | metaEvaluateExpression
    | metaQuoteExpression
    | metaQuoteBlock
    | metaSplice
    | metaInterpolation
    | reflectExpression
    | expandExpression
    | metaMemberAccess
    | metaCollection
    | metaValue
    ;


/* ============================================================================
 * 34. EXTENSION DECLARATIONS
 * ========================================================================== */

/**
 * Generic extension declaration.
 *
 * Extensions are namespaced and versionable.
 */
extensionDeclaration
    : visibilityModifier?
      EXTENSION
      extensionPath
      extensionVersion?
      extensionAttributes*
      extensionBody
    ;


extensionPath
    : identifier
      (DOUBLE_COLON identifier)*
    ;


extensionVersion
    : VERSION versionExpression
    ;


extensionAttributes
    : attribute
    ;


extensionBody
    : LBRACE extensionMember* RBRACE
    ;


extensionMember
    : extensionRequires
    | extensionProvides
    | extensionDepends
    | extensionCompatibility
    | extensionSyntax
    | extensionSemantics
    | extensionLowering
    | extensionExport
    | extensionDirective
    ;


/* ============================================================================
 * 35. EXTENSION REQUIREMENTS
 * ========================================================================== */

extensionRequires
    : REQUIRES
      capabilitySet
      SEMI?
    ;


extensionProvides
    : PROVIDES
      capabilitySet
      SEMI?
    ;


extensionDepends
    : DEPENDS
      dependencyList
      SEMI?
    ;


dependencyList
    : dependency
      (COMMA dependency)*
      COMMA?
    ;


dependency
    : qualifiedName
      dependencyVersionConstraint?
    ;


dependencyVersionConstraint
    : VERSION
      versionExpression
    ;


/* ============================================================================
 * 36. EXTENSION COMPATIBILITY
 * ========================================================================== */

extensionCompatibility
    : COMPATIBILITY
      compatibilityExpression
      SEMI?
    ;


compatibilityExpression
    : expression
    ;


/* ============================================================================
 * 37. EXTENSION SYNTAX
 * ========================================================================== */

extensionSyntax
    : SYNTAX
      metaValue
      SEMI?
    ;


/* ============================================================================
 * 38. EXTENSION SEMANTICS
 * ========================================================================== */

extensionSemantics
    : SEMANTICS
      metaValue
      SEMI?
    ;


/* ============================================================================
 * 39. EXTENSION LOWERING
 * ========================================================================== */

/**
 * Describes a lowering contract.
 *
 * The grammar records the declaration; the compiler owns actual lowering.
 */
extensionLowering
    : LOWERING
      qualifiedName
      SEMI?
    ;


/* ============================================================================
 * 40. EXTENSION EXPORTS
 * ========================================================================== */

extensionExport
    : EXPORT
      qualifiedName
      SEMI?
    ;


/* ============================================================================
 * 41. EXTENSION DIRECTIVES
 * ========================================================================== */

extensionDirective
    : DIRECTIVE
      identifier
      (ASSIGN metaValue)?
      SEMI?
    ;


/* ============================================================================
 * 42. META DECLARATION DISPATCH
 * ========================================================================== */

/**
 * Canonical metaprogramming declaration dispatcher.
 *
 * The composed top-level grammar should import this rule rather than copying
 * its alternatives.
 */
metaDeclaration
    : metaprogrammingDeclaration
    | extensionDeclaration
    | specializationDeclaration
    | generateDeclaration
    ;


/* ============================================================================
 * 43. META EXPRESSION DISPATCH
 * ========================================================================== */

metaPrimaryExpression
    : metaExpression
    | literal
    | identifier
    ;


/* ============================================================================
 * 44. RESERVED META SPACE
 * ========================================================================== */

/**
 * Reserved extension points are represented structurally rather than through
 * arbitrary parser fallbacks.
 *
 * This is important for deterministic parsing and diagnostics.
 */
metaExtensionPoint
    : EXTENSION
      extensionPath
      extensionBody
    ;


/* ============================================================================
 * 45. SEMANTIC BOUNDARY NOTES
 * ========================================================================== */

/*
 * The following are intentionally NOT grammar rules:
 *
 *   macroExpansion
 *   macroResolution
 *   hygieneResolution
 *   compileTimeExecution
 *   resourceBudget
 *   expansionDepth
 *   generatedProgramSize
 *   backendSelection
 *   targetSelection
 *   hardwareSelection
 *   quantumMapping
 *   qecSelection
 *   zqnNoiseModel
 *   optimization
 *   scheduling
 *   routing
 *   runtimeDispatch
 *
 * These belong to later compiler stages.
 *
 * This prevents the metaprogramming grammar from becoming coupled to any
 * particular execution machine.
 */


/* ============================================================================
 * 46. INTEGRATION CONTRACT
 * ========================================================================== */

/*
 * REQUIRED COMPOSITION CONTRACT
 *
 * The canonical parser must integrate this grammar as follows:
 *
 *   source
 *      |
 *      v
 *   Zamani lexer
 *      |
 *      v
 *   canonical parser
 *      |
 *      +---- core declarations
 *      |
 *      +---- modules
 *      |
 *      +---- functions
 *      |
 *      +---- types
 *      |
 *      +---- statements
 *      |
 *      +---- macros
 *      |
 *      +---- metaprogramming
 *      |
 *      +---- domain grammar
 *      |
 *      v
 *   canonical AST
 *      |
 *      v
 *   semantic analysis
 *      |
 *      +---- name resolution
 *      +---- type checking
 *      +---- effect checking
 *      +---- capability checking
 *      +---- resource checking
 *      +---- macro resolution
 *      +---- compile-time evaluation
 *      |
 *      v
 *   canonical semantic representations
 *      |
 *      +---- classical IR
 *      +---- quantum::ir
 *      +---- HDL/hardware representations
 *      |
 *      v
 *   optimization
 *      |
 *      v
 *   routing
 *      |
 *      v
 *   scheduling
 *      |
 *      +---- ZQN
 *      +---- QEC
 *      |
 *      v
 *   hardware HAL
 *      |
 *      v
 *   runtime
 *
 * No reverse dependency is permitted.
 */


/* ============================================================================
 * 47. DOMAIN INTEGRATION CONTRACT
 * ========================================================================== */

/*
 * QUANTUM
 *
 * Metaprogramming may generate quantum SOURCE SYNTAX.
 *
 * It must not generate or directly manipulate physical machine state.
 *
 * Correct:
 *
 *     macro make_circuit(...) { ... }
 *
 *       ->
 *     source/AST
 *
 *       ->
 *     quantum::ir
 *
 * Incorrect:
 *
 *     macro -> direct QPU API
 *     macro -> physical qubit allocation
 *     macro -> hardware topology
 *     macro -> scheduler
 *
 *
 * QEC
 *
 * Metaprogramming may generate source-level QEC intent.
 *
 * It does not implement decoders, stabilizer algorithms, code-distance
 * calculations, or correction procedures.
 *
 *
 * ZQN
 *
 * Metaprogramming may generate declarations that semantic analysis later maps
 * to ZQN-compatible concepts.
 *
 * It does not own the noise/fault model.
 *
 *
 * HARDWARE / HDL
 *
 * Metaprogramming may generate HDL/hardware SOURCE structures.
 *
 * It does not discover or select physical devices.
 *
 *
 * SCHEDULING
 *
 * Metaprogramming may generate timing/resource intent.
 *
 * Scheduling remains a later compiler subsystem.
 *
 *
 * OPTIMIZATION
 *
 * Metaprogramming can express specialization opportunities.
 *
 * It does not execute optimizer passes.
 */


/* ============================================================================
 * 48. SCALABILITY CONTRACT
 * ========================================================================== */

/*
 * The grammar intentionally contains no rules such as:
 *
 *     qubitCount: INTEGER { <= 32 }
 *     macroCount: INTEGER { <= 1024 }
 *     parameterCount: INTEGER { <= 64 }
 *     specializationCount: INTEGER { <= 256 }
 *
 * Such constructs would turn implementation limits into language semantics.
 *
 * Large programs remain representable until the parser/compiler implementation
 * reaches an explicitly configurable resource budget.
 *
 * Resource budgets MUST be reported as implementation/resource diagnostics,
 * not as grammar-level language restrictions.
 */


/* ============================================================================
 * 49. DETERMINISM CONTRACT
 * ========================================================================== */

/*
 * Parsing must be deterministic.
 *
 * Extension resolution must NOT use:
 *
 *     "first loaded wins"
 *     "last loaded wins"
 *
 * semantics.
 *
 * If two extensions introduce conflicting syntax or semantics, the semantic
 * compiler layer must reject the program deterministically.
 */


/* ============================================================================
 * 50. SECURITY CONTRACT
 * ========================================================================== */

/*
 * The presence of metaprogramming syntax MUST NOT imply permission to:
 *
 *     read arbitrary files;
 *     write arbitrary files;
 *     access arbitrary network resources;
 *     spawn arbitrary processes;
 *     inspect secrets;
 *     access credentials;
 *     access hardware;
 *     mutate compiler state.
 *
 * Any permitted compile-time capability must be explicitly granted by the
 * compiler's capability/security model.
 */


/* ============================================================================
 * 51. AST CONTRACT
 * ========================================================================== */

/*
 * This grammar must lower into the repository's canonical AST infrastructure.
 *
 * It MUST NOT introduce a parallel:
 *
 *     MetaAst
 *     MacroAst
 *     MetaIr
 *     MacroIr
 *
 * hierarchy merely because these constructs are syntactically special.
 *
 * Meta syntax should be represented by the canonical source AST's established
 * node/ID/span infrastructure.
 *
 * Macro nodes should retain:
 *
 *     source span
 *     macro path/name
 *     argument NodeIds
 *     attributes
 *     source identity
 *
 * Semantic expansion metadata belongs to later compiler stages.
 */


/* ============================================================================
 * 52. COMPLETION CRITERIA
 * ========================================================================== */

/*
 * This grammar component is complete only when:
 *
 * [ ] It composes with the canonical Zamani parser.
 *
 * [ ] Every referenced token exists exactly once in the authoritative lexer.
 *
 * [ ] Every referenced shared parser rule has exactly one canonical owner.
 *
 * [ ] No parser rule creates a second AST architecture.
 *
 * [ ] Macro declarations parse deterministically.
 *
 * [ ] Macro invocations parse deterministically.
 *
 * [ ] Qualified macro paths have no fixed depth limit.
 *
 * [ ] Quotation works for expressions and blocks.
 *
 * [ ] Splicing is structurally represented.
 *
 * [ ] Token trees are structurally represented.
 *
 * [ ] Language declarations are declarative.
 *
 * [ ] Compile-time declarations are syntactically distinct.
 *
 * [ ] Reflection is syntactically represented without granting authority.
 *
 * [ ] Specialization has no fixed dimension limit.
 *
 * [ ] Extensions are namespaced.
 *
 * [ ] Extension versioning is represented.
 *
 * [ ] Capabilities and requirements are syntactically distinct.
 *
 * [ ] Backend/device selection is absent from the grammar.
 *
 * [ ] Hardware sizes are absent as language-level fixed limits.
 *
 * [ ] Quantum physical topology is absent from metaprogramming semantics.
 *
 * [ ] No filesystem/network/process execution is encoded.
 *
 * [ ] No Rust action is embedded in ANTLR.
 *
 * [ ] No Rust unsafe requirement exists.
 *
 * [ ] Positive parser tests exist.
 *
 * [ ] Negative parser tests exist.
 *
 * [ ] Boundary/scalability tests exist.
 *
 * [ ] Cross-domain tests exist.
 *
 * [ ] Determinism tests exist.
 *
 * [ ] AST mapping tests exist.
 *
 * [ ] Macro-to-quantum integration tests exist.
 *
 * [ ] Macro-to-HDL integration tests exist.
 *
 * [ ] Macro-to-classical integration tests exist.
 *
 * [ ] Macro-to-hardware-capability integration tests exist.
 *
 * [ ] Macro-to-distributed-program integration tests exist.
 *
 * [ ] Documentation matches the authoritative grammar.
 *
 * [ ] No accidental duplicate ownership remains in legacy Meta.g4/macros rules.
 */
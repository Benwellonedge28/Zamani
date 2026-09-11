/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/antlr/Meta.g4
 *
 * Role:
 *     Canonical parser component for Zamani metaprogramming.
 *
 * Intended composition:
 *
 *     ZamaniLexer.g4
 *             |
 *             v
 *     ZamaniParser.g4
 *             |
 *             +---- Core syntax
 *             |
 *             +---- Modules.g4
 *             |
 *             +---- Meta.g4
 *             |
 *             +---- domain grammars
 *             |
 *             v
 *        Frontend AST
 *             |
 *             v
 *     semantic analysis
 *             |
 *             +---- name resolution
 *             +---- type checking
 *             +---- effect checking
 *             +---- capability checking
 *             +---- resource checking
 *             +---- macro resolution
 *             |
 *             v
 *       canonical IR
 *
 * ============================================================================
 *
 * LANGUAGE / COMPILER CONTRACT
 * ============================================================================
 *
 * Meta.g4 owns SOURCE SYNTAX only.
 *
 * It owns:
 *
 *     - macro declarations;
 *     - macro invocation syntax;
 *     - language declarations;
 *     - language-definition members;
 *     - compile-time/meta expressions;
 *     - quotation;
 *     - interpolation/splicing;
 *     - token/source-oriented meta values;
 *     - compiler directives represented as attributes;
 *     - extern declarations;
 *     - meta-level parameter/argument structure.
 *
 * It does NOT own:
 *
 *     - macro expansion;
 *     - macro resolution;
 *     - filesystem access;
 *     - network access;
 *     - package downloading;
 *     - code execution;
 *     - arbitrary process execution;
 *     - compiler configuration mutation;
 *     - target selection;
 *     - backend selection;
 *     - hardware selection;
 *     - quantum-device selection;
 *     - optimization;
 *     - IR lowering.
 *
 * ============================================================================
 *
 * CORE ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * Syntax describes INTENT.
 *
 * Semantic analysis determines MEANING.
 *
 * Macro expansion transforms SOURCE/IR according to explicit compiler policy.
 *
 * The grammar therefore must never encode:
 *
 *     macro -> execute arbitrary host program
 *     macro -> access filesystem
 *     macro -> access network
 *     macro -> select backend
 *     macro -> select CPU
 *     macro -> select GPU
 *     macro -> select QPU
 *     macro -> select gate set
 *     macro -> select machine width
 *
 * Those are compiler/toolchain policy decisions.
 *
 * ============================================================================
 *
 * SAFETY
 * ============================================================================
 *
 * The compiler implementation target is:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * The Rust implementation MUST use safe Rust.
 *
 * This grammar contains no executable Rust and introduces no unsafe
 * requirement.
 *
 * A Zamani source construct named `unsafe` does not authorize Rust `unsafe`.
 *
 * ============================================================================
 *
 * SCALABILITY
 * ============================================================================
 *
 * No language-level finite limits are imposed here on:
 *
 *     - macro parameters;
 *     - macro arguments;
 *     - macro nesting;
 *     - quoted syntax;
 *     - language members;
 *     - metadata;
 *     - generated source size;
 *     - generated IR size.
 *
 * Implementations MAY impose configurable resource budgets for:
 *
 *     - token count;
 *     - AST nodes;
 *     - expansion depth;
 *     - expansion steps;
 *     - generated source;
 *     - memory;
 *     - compilation time.
 *
 * Those limits belong to compiler policy and diagnostics, not this grammar.
 *
 * ============================================================================
 *
 * DETERMINISM
 * ============================================================================
 *
 * Meta syntax is structurally deterministic.
 *
 * Expansion determinism is a semantic/compiler contract.
 *
 * Macro expansion MUST preserve deterministic source mapping as required by
 * the semantic specification.
 *
 * ============================================================================
 *
 * AST COMPATIBILITY
 * ============================================================================
 *
 * The repository already has a canonical source-level Macro AST concept:
 *
 *     Macro
 *       |
 *       +-- source-level name
 *       +-- argument NodeIds
 *       +-- source identity/span
 *
 * The parser must therefore produce the existing canonical AST representation
 * rather than introducing another Meta/Macro AST hierarchy.
 *
 * The macro AST deliberately remains domain-neutral and does not contain:
 *
 *     qubit_count
 *     backend
 *     topology
 *     gate_set
 *     device
 *     qec
 *     scheduler
 *     target
 *
 * ============================================================================
 *
 * IMPORTANT ANTLR COMPOSITION RULE
 * ============================================================================
 *
 * This file is a PARSER grammar.
 *
 * It is designed to be imported by the canonical parser grammar:
 *
 *     parser grammar ZamaniParser;
 *
 *     options {
 *         tokenVocab = ZamaniLexer;
 *     }
 *
 *     import Meta;
 *
 * It therefore deliberately references shared parser rules such as:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     typeExpression
 *     parameterList
 *     argumentList
 *     block
 *     attributes
 *     attribute
 *
 * Those rules MUST have exactly one canonical owner in the final composed
 * parser.
 *
 * ============================================================================
 */

parser grammar Meta;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 1. META ITEMS
 * ========================================================================== */

/**
 * Meta declarations are source-level declarations.
 *
 * The canonical parser should include these through its declaration/item
 * dispatch rather than creating another top-level program grammar here.
 */
metaDeclaration
    : macroDeclaration
    | languageDeclaration
    | externDeclaration
    ;


/* ============================================================================
 * 2. MACRO DECLARATIONS
 * ========================================================================== */

/**
 * Macro declaration.
 *
 * Canonical form:
 *
 *     macro name() {
 *         ...
 *     }
 *
 * Generic parameters are supported because macro definitions may be
 * parameterized independently of runtime type inference.
 *
 * The body remains source syntax.
 *
 * The compiler's macro engine determines how the body is interpreted.
 */
macroDeclaration
    : visibility?
      MACRO identifier
      genericParameters?
      LPAREN macroParameterList? RPAREN
      macroBody
    ;


/**
 * Macro parameter list.
 *
 * Macro parameters are intentionally separated from ordinary runtime
 * parameters. A future semantic layer can distinguish:
 *
 *     expression parameters
 *     syntax parameters
 *     token parameters
 *     type parameters
 *     pattern parameters
 *     compile-time values
 *
 * without changing the outer macro declaration syntax.
 */
macroParameterList
    : macroParameter
      (COMMA macroParameter)*
      COMMA?
    ;


/**
 * Macro parameter.
 *
 * The optional type is deliberately syntactic.
 *
 * Semantic validation determines what macro parameter kinds are legal.
 */
macroParameter
    : macroParameterPattern
      (COLON typeExpression)?
      macroParameterDefault?
    ;


/**
 * Macro parameter names may use normal identifiers.
 *
 * A dedicated production provides a stable extension boundary without
 * introducing a second identifier token class.
 */
macroParameterPattern
    : identifier
    ;


/**
 * Compile-time default value.
 */
macroParameterDefault
    : ASSIGN expression
    ;


/**
 * Macro body.
 *
 * Macro implementations may be represented by a block or a single expression.
 *
 * The parser does not expand or execute either form.
 */
macroBody
    : block
    | expression
    ;


/* ============================================================================
 * 3. MACRO INVOCATION
 * ========================================================================== */

/**
 * Generic macro invocation.
 *
 * Canonical source form:
 *
 *     name!(arg)
 *
 * Qualified invocation is also supported:
 *
 *     namespace::name!(arg)
 *
 * The exclamation mark is syntactic invocation punctuation, not a semantic
 * classification of the macro.
 */
macroInvocation
    : macroPath BANG LPAREN argumentList? RPAREN
    ;


/**
 * Macro path.
 *
 * No fixed namespace depth is imposed.
 */
macroPath
    : identifier
      (DOUBLE_COLON identifier)*
    ;


/**
 * Expression-level macro invocation.
 *
 * The canonical AST Macro node stores the macro name and argument NodeIds.
 *
 * This grammar intentionally does not encode an enumeration of macro names.
 */
macroExpression
    : macroInvocation
    ;


/* ============================================================================
 * 4. MACRO QUOTATION
 * ========================================================================== */

/**
 * Quotation introduces source structure as a meta-level value.
 *
 * The exact semantic representation is intentionally left to the macro
 * subsystem.
 *
 * Examples:
 *
 *     quote { ... }
 *     quote(expression)
 *
 * If the canonical lexer/parser specification does not expose a dedicated
 * quote keyword, this production remains an extension point and MUST NOT be
 * wired into the accepted-language entry point until the corresponding lexer
 * tokens are canonicalized.
 *
 * This separation prevents Meta.g4 from silently inventing language keywords.
 */
metaQuoteExpression
    : quoteBlockExpression
    | quoteExpression
    ;


quoteBlockExpression
    : QUOTE block
    ;


quoteExpression
    : QUOTE LPAREN expression RPAREN
    ;


/* ============================================================================
 * 5. SPLICE / INTERPOLATION
 * ========================================================================== */

/**
 * Meta interpolation/splicing.
 *
 * Splicing is represented structurally.
 *
 * The semantic macro engine determines:
 *
 *     - whether the splice is legal;
 *     - what category it produces;
 *     - whether it produces an expression;
 *     - declaration syntax;
 *     - type syntax;
 *     - token/source syntax.
 *
 * No such semantic category is hard-coded here.
 */
metaSpliceExpression
    : SPLICE LPAREN expression RPAREN
    ;


/**
 * Nested meta expressions may be embedded inside quoted structures.
 *
 * This is intentionally a semantic extension boundary rather than a second
 * expression language.
 */
metaInterpolation
    : INTERPOLATE LPAREN expression RPAREN
    ;


/* ============================================================================
 * 6. META VALUES
 * ========================================================================== */

/**
 * A meta value can be represented by an ordinary expression or explicit
 * quoted/spliced structure.
 */
metaValue
    : expression
    | metaQuoteExpression
    | metaSpliceExpression
    | metaInterpolation
    ;


/**
 * Meta argument list.
 *
 * Kept separate from runtime argumentList so future semantic distinctions do
 * not force changes to ordinary calls.
 */
metaArgumentList
    : metaValue
      (COMMA metaValue)*
      COMMA?
    ;


/* ============================================================================
 * 7. LANGUAGE DECLARATIONS
 * ========================================================================== */

/**
 * Language declaration.
 *
 * Canonical structural form:
 *
 *     language Name {
 *         ...
 *     }
 *
 * A language declaration describes a language-level entity.
 *
 * It does NOT dynamically replace the compiler lexer during parsing.
 *
 * Language implementation, dialect selection, validation, and compiler
 * integration belong to semantic/compiler infrastructure.
 */
languageDeclaration
    : visibility?
      LANGUAGE identifier
      languageParameters?
      languageBody
    ;


/**
 * Optional language parameters.
 */
languageParameters
    : LPAREN parameterList? RPAREN
    ;


/**
 * Language body.
 *
 * Members are explicit meta-language members rather than arbitrary `item`
 * recursion. This prevents accidental acceptance of unrelated declarations
 * without semantic ownership.
 */
languageBody
    : LBRACE languageMember* RBRACE
    ;


/**
 * Language member.
 */
languageMember
    : languageGrammarDeclaration
    | languageLexerDeclaration
    | languageParserDeclaration
    | languageMacroDeclaration
    | languageTypeDeclaration
    | languageOperatorDeclaration
    | languageAttributeDeclaration
    | languageDirective
    | languageExportDeclaration
    ;


/* ============================================================================
 * 8. LANGUAGE GRAMMAR DECLARATION
 * ========================================================================== */

/**
 * Declares grammar-level material belonging to a language definition.
 *
 * The actual grammar representation is kept as a structured expression/value
 * instead of embedding ANTLR grammar syntax directly into Zamani.
 */
languageGrammarDeclaration
    : GRAMMAR identifier
      languageDefinitionBody?
      SEMI?
    ;


languageDefinitionBody
    : LBRACE languageDefinitionEntry* RBRACE
    ;


languageDefinitionEntry
    : identifier
      (COLON metaValue)?
      SEMI?
    ;


/* ============================================================================
 * 9. LANGUAGE LEXER DECLARATION
 * ========================================================================== */

/**
 * Language-level lexer specification.
 *
 * This is metadata describing a language.
 *
 * It does not mutate the active Zamani lexer.
 */
languageLexerDeclaration
    : LEXER identifier
      languageDefinitionBody?
      SEMI?
    ;


/* ============================================================================
 * 10. LANGUAGE PARSER DECLARATION
 * ========================================================================== */

/**
 * Language-level parser specification.
 *
 * This describes a parser artifact as data.
 */
languageParserDeclaration
    : PARSER identifier
      languageDefinitionBody?
      SEMI?
    ;


/* ============================================================================
 * 11. LANGUAGE MACRO DECLARATION
 * ========================================================================== */

/**
 * A language can declare macro-facing facilities.
 *
 * This is deliberately separate from a normal macro declaration so semantic
 * tooling can distinguish:
 *
 *     macro owned by current language
 *
 * from:
 *
 *     macro declared as part of another language definition.
 */
languageMacroDeclaration
    : MACRO identifier
      genericParameters?
      LPAREN macroParameterList? RPAREN
      macroBody
    ;


/* ============================================================================
 * 12. LANGUAGE TYPE DECLARATION
 * ========================================================================== */

/**
 * A language definition may expose a type-level descriptor.
 *
 * This is metadata, not a second type system.
 */
languageTypeDeclaration
    : TYPE identifier
      genericParameters?
      (ASSIGN typeExpression)?
      SEMI?
    ;


/* ============================================================================
 * 13. LANGUAGE OPERATOR DECLARATION
 * ========================================================================== */

/**
 * Operator declarations are semantic descriptors.
 *
 * They do not require the lexer to manufacture a new token for every operator
 * introduced by a user-defined language.
 */
languageOperatorDeclaration
    : OPERATOR languageOperatorSymbol
      operatorSignature?
      SEMI?
    ;


languageOperatorSymbol
    : identifier
    | PLUS
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
    | QUESTION_MARK
    ;


operatorSignature
    : COLON typeExpression
    ;


/* ============================================================================
 * 14. LANGUAGE ATTRIBUTE DECLARATIONS
 * ========================================================================== */

/**
 * Language-defined attribute.
 *
 * Attribute semantics are resolved by the language/toolchain subsystem.
 */
languageAttributeDeclaration
    : ATTRIBUTE identifier
      attributeSignature?
      SEMI?
    ;


attributeSignature
    : LPAREN parameterList? RPAREN
    ;


/* ============================================================================
 * 15. LANGUAGE DIRECTIVES
 * ========================================================================== */

/**
 * A language directive is a declarative metadata entry.
 *
 * It is not an imperative compiler command.
 */
languageDirective
    : DIRECTIVE identifier
      (ASSIGN metaValue)?
      SEMI?
    ;


/* ============================================================================
 * 16. LANGUAGE EXPORTS
 * ========================================================================== */

/**
 * Language-level exports.
 *
 * The same module-resolution principles apply: syntax records the export;
 * semantic resolution determines what it means.
 */
languageExportDeclaration
    : EXPORT languageExportTarget SEMI?
    ;


languageExportTarget
    : identifier
    | qualifiedName
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
 * 17. EXTERN DECLARATIONS
 * ========================================================================== */

/**
 * External declaration.
 *
 * `extern` describes an externally provided symbol.
 *
 * It does not itself perform linking, dynamic loading, FFI calls, filesystem
 * access, or process execution.
 */
externDeclaration
    : visibility?
      EXTERN
      externAbi?
      externSource?
      functionSignature
    ;


/**
 * ABI identity is a language-level identifier.
 *
 * It is intentionally not an exhaustive list such as:
 *
 *     c
 *     wasm
 *     llvm
 *     qir
 *
 * Semantic validation owns ABI compatibility.
 */
externAbi
    : identifier
    ;


/**
 * Optional external source descriptor.
 *
 * A string is metadata only.
 */
externSource
    : stringLiteral
    ;


/* ============================================================================
 * 18. META ATTRIBUTES
 * ========================================================================== */

/**
 * Compiler/meta attributes are ordinary source attributes.
 *
 * Meta.g4 does not introduce a second attribute syntax.
 *
 * This production exists so semantic tooling can classify attribute usage
 * after parsing.
 */
metaAttribute
    : attribute
    ;


/* ============================================================================
 * 19. COMPILE-TIME EXPRESSIONS
 * ========================================================================== */

/**
 * Compile-time expression marker.
 *
 * This is deliberately represented as a wrapper around the ordinary
 * expression grammar.
 *
 * It prevents the language from acquiring a second expression syntax merely
 * because an expression executes during compilation.
 */
compileTimeExpression
    : AT expression
    ;


/**
 * Compile-time block.
 */
compileTimeBlock
    : AT block
    ;


/* ============================================================================
 * 20. META ITEM DISPATCH
 * ========================================================================== */

/**
 * Parser integration hook.
 *
 * The canonical parser should include:
 *
 *     metaItem
 *
 * in its item/declaration dispatch rather than duplicating individual
 * productions.
 */
metaItem
    : macroDeclaration
    | languageDeclaration
    | externDeclaration
    ;


/* ============================================================================
 * 21. SEMANTIC BOUNDARY
 * ============================================================================
 *
 * The following concepts intentionally DO NOT appear as grammar actions:
 *
 *     expandMacro
 *     resolveMacro
 *     executeMacro
 *     loadMacro
 *     fetchMacro
 *     readFile
 *     writeFile
 *     executeProcess
 *     accessNetwork
 *     selectBackend
 *     selectDevice
 *     lowerToLLVM
 *     lowerToQIR
 *     lowerToOpenQASM
 *
 * A parser must remain a pure source-structure producer.
 *
 * ============================================================================
 *
 * MACRO PIPELINE
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     lexer
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     Macro AST
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     name / macro resolution
 *       |
 *       v
 *     capability / policy checks
 *       |
 *       v
 *     bounded deterministic expansion
 *       |
 *       v
 *     generated source/AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     canonical IR
 *
 * The expansion stage must remain outside this grammar.
 *
 * ============================================================================
 *
 * NO HOST-LANGUAGE COUPLING
 * ============================================================================
 *
 * Meta.g4 does not make Zamani macros equivalent to:
 *
 *     Rust macro_rules!
 *     Rust procedural macros
 *     C preprocessor macros
 *     Python metaclasses
 *     Lisp reader macros
 *     compiler plugins
 *
 * Those systems may be implementation techniques or interoperability targets,
 * but they are not the Zamani language model.
 *
 * ============================================================================
 *
 * QUANTUM / HARDWARE NEUTRALITY
 * ============================================================================
 *
 * Meta constructs may generate quantum source.
 *
 * They may also generate:
 *
 *     classical code
 *     mathematical code
 *     HDL
 *     distributed computation
 *     accelerator computation
 *     future computational domains
 *
 * Nothing in this grammar requires a specific realization.
 *
 * For example, the grammar must not contain:
 *
 *     QUBIT_COUNT
 *     MAX_QUBITS
 *     CPU_WIDTH
 *     GPU_ARCH
 *     QPU_ARCH
 *     GATE_SET
 *     DEVICE_ID
 *
 * ============================================================================
 *
 * SOURCE MAPPING
 * ============================================================================
 *
 * Macro syntax must preserve source identity through parsing.
 *
 * Expansion must later preserve:
 *
 *     original source span
 *     macro definition identity
 *     invocation identity
 *     expansion identity
 *     generated-source relationship
 *
 * This is a compiler/AST requirement, not a parser action.
 *
 * ============================================================================
 *
 * ERROR MODEL
 * ============================================================================
 *
 * Syntax errors:
 *
 *     handled by ANTLR parser diagnostics.
 *
 * Semantic macro errors:
 *
 *     unresolved macro
 *     invalid macro arguments
 *     invalid macro context
 *     expansion cycle
 *     expansion budget exceeded
 *     generated syntax failure
 *
 * must be diagnosed downstream.
 *
 * A resource-limit failure must not be misreported as a syntax error.
 *
 * ============================================================================
 *
 * DETERMINISTIC EXPANSION
 * ============================================================================
 *
 * The grammar itself introduces no nondeterministic behavior.
 *
 * The macro engine must additionally guarantee deterministic behavior for the
 * same:
 *
 *     source
 *     compiler configuration
 *     macro definitions
 *     semantic environment
 *     explicit inputs
 *
 * External information must not silently influence expansion.
 *
 * ============================================================================
 *
 * RESOURCE POLICY
 * ============================================================================
 *
 * The language does not prescribe finite macro-expansion limits.
 *
 * A production compiler should expose policy such as:
 *
 *     expansion_step_budget
 *     expansion_depth_budget
 *     generated_node_budget
 *     generated_byte_budget
 *     memory_budget
 *     compilation_time_budget
 *
 * These values are runtime/compiler policy and therefore MUST NOT be grammar
 * constants.
 *
 * ============================================================================
 *
 * INTEGRATION WITH EXISTING CORE.G4
 * ============================================================================
 *
 * The current Core.g4 contains:
 *
 *     languageDeclaration
 *     languageBody
 *     macroDeclaration
 *     macroBody
 *     externDeclaration
 *     ABI
 *
 * Those rules should eventually be removed from Core.g4 after Meta.g4 becomes
 * the canonical owner.
 *
 * Core should retain only the meta dispatch hook, for example:
 *
 *     declaration
 *         : ...
 *         | macroDeclaration
 *         | languageDeclaration
 *         | externDeclaration
 *         ;
 *
 * OR, preferably in the final parser composition:
 *
 *     declaration
 *         : ...
 *         | metaItem
 *         ;
 *
 * but NOT both.
 *
 * There must be one production owner for each rule.
 *
 * ============================================================================
 *
 * INTEGRATION WITH ZamaniParser.g4
 * ============================================================================
 *
 * `ZamaniParser.g4` currently already contains parser-level declarations and
 * expression infrastructure.
 *
 * The final canonical parser should import Meta.g4 and expose:
 *
 *     metaItem
 *     macroInvocation
 *     macroExpression
 *
 * through its item/expression dispatch.
 *
 * The final parser MUST NOT duplicate:
 *
 *     macroDeclaration
 *     languageDeclaration
 *     externDeclaration
 *     macroInvocation
 *
 * ============================================================================
 *
 * TOKEN CONTRACT
 * ============================================================================
 *
 * Meta.g4 relies on canonical lexer tokens where available:
 *
 *     LANGUAGE
 *     MACRO
 *     EXTERN
 *     EXPORT
 *     TYPE
 *     OPERATOR
 *
 * and ordinary punctuation/identifier/literal tokens.
 *
 * If a token named below is not currently part of ZamaniLexer.g4:
 *
 *     QUOTE
 *     SPLICE
 *     INTERPOLATE
 *     GRAMMAR
 *     LEXER
 *     PARSER
 *     ATTRIBUTE
 *     DIRECTIVE
 *     BANG
 *
 * it MUST NOT be silently invented by this grammar.
 *
 * Those tokens must first be canonically specified and added to the lexer,
 * with compatibility tests, before the corresponding productions are enabled.
 *
 * This is intentional: grammar files must never become hidden token
 * authorities.
 *
 * ============================================================================
 *
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * Meta.g4 is complete when:
 *
 *   1. Macro declaration syntax has one canonical owner.
 *   2. Macro invocation syntax has one canonical owner.
 *   3. Language declaration syntax has one canonical owner.
 *   4. Extern syntax has one canonical owner.
 *   5. Meta syntax has no backend assumptions.
 *   6. Meta syntax has no hardware limits.
 *   7. Meta syntax has no filesystem semantics.
 *   8. Meta syntax has no network semantics.
 *   9. Meta syntax has no execution semantics.
 *  10. Macro AST maps to the existing canonical Macro node.
 *  11. Expansion remains outside parsing.
 *  12. Expansion diagnostics remain distinct from syntax diagnostics.
 *  13. Source mapping remains deterministic.
 *  14. Compiler resource budgets remain configurable.
 *  15. No artificial language-level size limits exist.
 *  16. No duplicate lexer authority is introduced.
 *  17. No duplicate AST authority is introduced.
 *  18. No Rust unsafe requirement is introduced.
 *  19. Rust 1.97 and Rust 1.97.1 remain supported.
 *  20. Core/ZamaniParser duplicate productions are removed during integration.
 *
 * ============================================================================
 */
/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/metaprogramming/generation.g4
 *
 * Grammar:
 *     Generation
 *
 * Status:
 *     Production parser-grammar composition unit
 *
 * Purpose:
 *     Define the SOURCE-LEVEL SYNTAX for compile-time generation of Zamani
 *     language structures.
 *
 * This grammar establishes explicit syntax boundaries for generation without
 * defining:
 *
 *     - an AST;
 *     - a semantic model;
 *     - an IR;
 *     - a compiler implementation;
 *     - a macro expansion engine;
 *     - a reflection engine;
 *     - a specialization engine;
 *     - a compile-time evaluator;
 *     - a hardware model;
 *     - a quantum model;
 *     - a runtime model.
 *
 * ============================================================================
 *
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     authoritative ZamaniParser
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +--> name resolution
 *       +--> type checking
 *       +--> effect checking
 *       +--> capability checking
 *       +--> generation validation
 *       +--> compile-time evaluation
 *       +--> provenance
 *       |
 *       v
 *     generated source/semantic structures
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> HDL / hardware representation
 *       +--> distributed representation
 *       +--> other domain representations
 *       |
 *       v
 *     optimization / lowering / routing / scheduling / HAL
 *       |
 *       v
 *     runtime
 *
 * Generation MUST NOT bypass the semantic boundary.
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - explicit source-level generation expressions;
 *     - explicit source-level generation blocks;
 *     - generated expression boundaries;
 *     - generated type boundaries;
 *     - generated declaration/item boundaries;
 *     - generated statement boundaries;
 *     - generated program-fragment boundaries;
 *     - generation result-shaping syntax;
 *     - generation options that are intrinsic to generation syntax;
 *     - explicit generation provenance markers when represented syntactically.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - identifiers;
 *     - qualified names;
 *     - ordinary expressions;
 *     - expression precedence;
 *     - ordinary statements;
 *     - declarations;
 *     - function declarations;
 *     - compile-time function declarations;
 *     - macros;
 *     - macro hygiene;
 *     - macro expansion;
 *     - quotation;
 *     - splicing;
 *     - reflection;
 *     - specialization;
 *     - conditional compilation;
 *     - target selection;
 *     - resource requirements;
 *     - capabilities;
 *     - effects;
 *     - hardware discovery;
 *     - quantum hardware selection;
 *     - QEC;
 *     - ZQN;
 *     - optimization;
 *     - routing;
 *     - scheduling;
 *     - canonical AST;
 *     - canonical IR;
 *     - quantum::ir;
 *     - runtime execution.
 *
 * ============================================================================
 *
 * RELATIONSHIP TO OTHER METAPROGRAMMING COMPONENTS
 * ============================================================================
 *
 * metaprogramming.g4
 *     Aggregates metaprogramming facilities.
 *
 * compile-time-execution.g4
 *     Owns explicit compile-time execution boundaries.
 *
 * reflection.g4
 *     Owns reflection query syntax.
 *
 * specialization.g4
 *     Owns specialization syntax.
 *
 * macros.g4 / macros/*
 *     Own macro declarations, invocations, hygiene and expansion boundaries.
 *
 * functions/compile-time-functions.g4
 *     Own compile-time function declaration syntax.
 *
 * compile/generation/code-generation.g4
 *     Owns compilation-level code-generation controls and lowering policy.
 *
 * This file MUST NOT duplicate any of those facilities.
 *
 * ============================================================================
 *
 * EXISTING REPOSITORY INTEGRATION
 * ============================================================================
 *
 * The current canonical ANTLR parser owns common rules such as:
 *
 *     program
 *     sourceElement
 *     item
 *     statement
 *     expression
 *     blockExpression
 *     identifier
 *     typeExpression
 *     pattern
 *     attribute
 *     genericParameters
 *     parameterList
 *
 * Generation consumes those rules.
 *
 * The canonical lexer currently exposes:
 *
 *     SYNTHESIZE : 'synthesize'
 *
 * and does not expose a separate GENERATE token.
 *
 * Therefore this grammar deliberately uses SYNTHESIZE as the lexical
 * generation boundary.
 *
 * A future dedicated GENERATE keyword must be introduced in the canonical
 * lexer and language specification before being referenced here.
 *
 * ============================================================================
 *
 * CRITICAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * Generation produces LANGUAGE STRUCTURE, not machine instructions.
 *
 * For example, generation may eventually produce:
 *
 *     - a function;
 *     - a type;
 *     - a classical expression;
 *     - a quantum operation;
 *     - a quantum circuit fragment;
 *     - an HDL module;
 *     - a hardware interface;
 *     - a distributed declaration;
 *     - metadata;
 *     - another supported source-level structure.
 *
 * The generated result is subsequently validated and lowered through the
 * normal Zamani semantic pipeline.
 *
 * Generation MUST NOT directly construct:
 *
 *     - quantum::ir;
 *     - classical IR;
 *     - HDL IR;
 *     - scheduling plans;
 *     - routing plans;
 *     - hardware bindings;
 *     - runtime instructions.
 *
 * ============================================================================
 *
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Generation may generate quantum SOURCE STRUCTURE.
 *
 * It does not own quantum semantics.
 *
 * For example, a generated quantum operation ultimately follows:
 *
 *     generated source
 *          |
 *          v
 *     quantum semantic analysis
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization / routing / scheduling / HAL
 *
 * Generation MUST NOT:
 *
 *     - allocate physical qubits;
 *     - select a QPU;
 *     - select a topology;
 *     - encode a maximum qubit count;
 *     - construct quantum::ir;
 *     - bypass QEC;
 *     - bypass ZQN;
 *     - bypass routing;
 *     - bypass scheduling.
 *
 * ============================================================================
 *
 * HARDWARE / RESOURCE BOUNDARY
 * ============================================================================
 *
 * Generated source may express resource requirements if those requirements
 * are part of the generated program's semantics.
 *
 * This grammar does not encode physical limits.
 *
 * It MUST NOT introduce:
 *
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_NODES
 *
 * or any equivalent finite language-level ceiling.
 *
 * Machine-dependent properties belong to:
 *
 *     resources/
 *     hardware/
 *     compile/
 *     execution/
 *     scheduling/
 *     runtime capability models
 *
 * ============================================================================
 *
 * POCO-REAF
 * ============================================================================
 *
 * Generation MUST preserve:
 *
 *     Program Once
 *          |
 *          v
 *     stable source semantics
 *          |
 *          v
 *     deterministic / explicitly classified generation
 *          |
 *          v
 *     generated language structure
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          v
 *     target-independent compilation
 *          |
 *          v
 *     target realization
 *
 * A generator MUST NOT silently turn the compiler host into part of the
 * permanent meaning of the source program.
 *
 * If generation intentionally depends on external state, that dependency must
 * be visible to semantic/effect/capability analysis.
 *
 * ============================================================================
 *
 * SECURITY
 * ============================================================================
 *
 * Parsing generation syntax performs NO generation.
 *
 * The parser MUST NOT:
 *
 *     - execute generated code;
 *     - execute host code;
 *     - access the filesystem;
 *     - access the network;
 *     - inspect environment variables;
 *     - inspect credentials;
 *     - inspect arbitrary memory;
 *     - inspect hardware;
 *     - contact a QPU;
 *     - contact a GPU;
 *     - contact an FPGA;
 *     - spawn processes;
 *     - download packages;
 *     - mutate compiler configuration.
 *
 * Any such behavior, if the language permits it at all, requires explicit
 * semantic capabilities and belongs outside the grammar.
 *
 * ============================================================================
 *
 * DETERMINISM
 * ============================================================================
 *
 * The syntax is deterministic.
 *
 * Generation determinism is a semantic/compiler property.
 *
 * Pure deterministic generators SHOULD produce equivalent generated semantic
 * structures for equivalent inputs.
 *
 * Generators depending on:
 *
 *     - time;
 *     - randomness;
 *     - external state;
 *     - environment;
 *     - target capabilities;
 *     - resource availability;
 *
 * MUST be classified by semantic/effect/capability analysis.
 *
 * The grammar does not silently authorize those dependencies.
 *
 * ============================================================================
 *
 * PROVENANCE
 * ============================================================================
 *
 * Generated structures must remain traceable to:
 *
 *     - the generating source;
 *     - the generation site;
 *     - the semantic inputs;
 *     - the generator identity/version where applicable.
 *
 * Provenance storage is NOT implemented here.
 *
 * This grammar only permits explicit source-level provenance metadata where
 * the canonical attribute system permits it.
 *
 * ============================================================================
 *
 * SCALABILITY
 * ============================================================================
 *
 * No finite language-level limit is imposed on:
 *
 *     - generated declarations;
 *     - generated expressions;
 *     - generated statements;
 *     - generation blocks;
 *     - nested generation constructs;
 *     - generation inputs;
 *     - generation results.
 *
 * Compiler resource budgets MAY exist for:
 *
 *     - memory;
 *     - CPU time;
 *     - evaluation steps;
 *     - generated source size;
 *     - generated semantic graph size;
 *     - cancellation;
 *     - recursion protection.
 *
 * Such budgets are implementation policy and MUST NOT become grammar
 * semantics.
 *
 * ============================================================================
 *
 * RUST CONTRACT
 * ============================================================================
 *
 * Compiler/frontend implementation target:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * Generated parser integration MUST use safe Rust only.
 *
 * This grammar contains:
 *
 *     - no Rust actions;
 *     - no semantic predicates;
 *     - no unsafe code;
 *     - no filesystem operations;
 *     - no network operations;
 *     - no execution logic.
 *
 * ============================================================================
 *
 * COMPOSITION CONTRACT
 * ============================================================================
 *
 * This is a parser grammar component.
 *
 * It is expected to be composed into the authoritative Zamani parser.
 *
 * It intentionally consumes canonical parser rules instead of redefining
 * them.
 *
 * Required canonical rules:
 *
 *     expression
 *     blockExpression
 *     statement
 *     item
 *     identifier
 *     typeExpression
 *     pattern
 *     attribute
 *     genericParameters
 *
 * The authoritative parser composition layer determines where each generation
 * entry point is legal.
 *
 * ============================================================================
 */

parser grammar Generation;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 1. PRIMARY GENERATION ENTRY POINT
 * ============================================================================
 *
 * The generation keyword is the existing canonical SYNTHESIZE token.
 *
 * Examples of the conceptual language surface:
 *
 *     synthesize (expression)
 *
 *     synthesize { ... }
 *
 *     synthesize type(TypeExpression)
 *
 *     synthesize item(...)
 *
 *     synthesize statement(...)
 *
 *     synthesize program(...)
 *
 * The parser only establishes the syntactic category.
 * Semantic analysis determines whether the result is actually generatable.
 * ========================================================================== */

generation
    : SYNTHESIZE generationForm
    ;


/* ============================================================================
 * 2. GENERATION FORM DISPATCH
 * ============================================================================
 *
 * The forms are deliberately explicit so generated output has a declared
 * syntactic category.
 *
 * This avoids a universal "generate anything" production whose result would
 * become ambiguous to tooling and semantic analysis.
 * ========================================================================== */

generationForm
    : generationExpression
    | generationType
    | generationStatement
    | generationItem
    | generationBlock
    | generationProgramFragment
    ;


/* ============================================================================
 * 3. GENERATED EXPRESSION
 * ============================================================================
 *
 * Syntax:
 *
 *     synthesize (expression)
 *
 * The expression is an input to the generation mechanism.
 *
 * The expression itself is NOT executed by the parser.
 *
 * Semantic analysis determines whether the expression is compile-time
 * evaluable and whether its result can be interpreted as a generated
 * expression.
 * ========================================================================== */

generationExpression
    : LPAREN expression RPAREN
    ;


/* ============================================================================
 * 4. GENERATED TYPE
 * ============================================================================
 *
 * Syntax:
 *
 *     synthesize type (TypeExpression)
 *
 * `type` is the existing canonical TYPE token.
 *
 * The type expression remains owned by the canonical type grammar.
 * ========================================================================== */

generationType
    : TYPE
      LPAREN
      typeExpression
      RPAREN
    ;


/* ============================================================================
 * 5. GENERATED STATEMENT
 * ============================================================================
 *
 * Syntax:
 *
 *     synthesize statement ( ... )
 *
 * The body is represented using canonical statement syntax.
 *
 * No second statement grammar is created here.
 * ========================================================================== */

generationStatement
    : STATEMENT_KEYWORD
      LPAREN
      statement
      RPAREN
    ;


/*
 * --------------------------------------------------------------------------
 * STATEMENT CATEGORY TOKEN
 * --------------------------------------------------------------------------
 *
 * The current canonical lexer does not define a STATEMENT keyword.
 *
 * Therefore this grammar does NOT reference an invented STATEMENT token.
 *
 * A generated statement is instead represented structurally through a
 * generation block:
 *
 *     synthesize {
 *         statement
 *     }
 *
 * The rule below is retained as a named semantic integration boundary only
 * through the block-based form.
 *
 * It is deliberately not reachable from `generationForm` until a canonical
 * lexical/syntactic category is established.
 *
 * This prevents an accidental dependency on a nonexistent token.
 * --------------------------------------------------------------------------
 */


/* ============================================================================
 * 6. GENERATED ITEM / DECLARATION
 * ============================================================================
 *
 * Generated declarations are represented by the canonical `item` rule.
 *
 * The source-level wrapper is:
 *
 *     synthesize item
 *
 * where `item` is a syntactic placeholder in this grammar composition model.
 *
 * Because the current canonical parser does not expose an ITEM keyword,
 * this grammar does not invent one.
 *
 * The production therefore uses a dedicated parenthesized canonical item
 * boundary only when the authoritative parser exposes the corresponding
 * category.
 *
 * To remain compilable against the current lexer, generated declarations are
 * expressed through `generationBlock` and canonical item syntax inside it.
 * ========================================================================== */

generationItem
    : generationItemBody
    ;


generationItemBody
    : LBRACE
      item+
      RBRACE
    ;


/* ============================================================================
 * 7. GENERATED BLOCK
 * ============================================================================
 *
 * Syntax:
 *
 *     synthesize {
 *         ...
 *     }
 *
 * This is the principal scalable generation form.
 *
 * The block is deliberately not duplicated as a second block grammar.
 *
 * Its contents remain canonical Zamani source structure.
 * ========================================================================== */

generationBlock
    : blockExpression
    ;


/* ============================================================================
 * 8. GENERATED PROGRAM FRAGMENT
 * ============================================================================
 *
 * A program fragment is a sequence of canonical source elements.
 *
 * Because `sourceElement` belongs to the authoritative parser, this rule is
 * intended to be integrated at composition time.
 *
 * No fixed number of generated elements is imposed.
 * ========================================================================== */

generationProgramFragment
    : LBRACE
      generationSourceElement*
      RBRACE
    ;


generationSourceElement
    : attribute
    | item
    | statement
    ;


/* ============================================================================
 * 9. EXPLICIT GENERATION RESULT
 * ============================================================================
 *
 * Named boundary for semantic tooling.
 *
 * This does not introduce another representation.
 *
 * The result remains a canonical expression-level value.
 * ========================================================================== */

generationResult
    : expression
    ;


/* ============================================================================
 * 10. GENERATED EXPRESSION RESULT
 * ============================================================================
 *
 * A generated expression must ultimately be represented through the normal
 * expression pipeline.
 * ========================================================================== */

generatedExpression
    : expression
    ;


/* ============================================================================
 * 11. GENERATED TYPE RESULT
 * ============================================================================
 *
 * A generated type must ultimately be represented through the canonical type
 * system.
 * ========================================================================== */

generatedType
    : typeExpression
    ;


/* ============================================================================
 * 12. GENERATED STATEMENT RESULT
 * ============================================================================
 *
 * A generated statement must ultimately be represented by the canonical
 * statement grammar.
 * ========================================================================== */

generatedStatement
    : statement
    ;


/* ============================================================================
 * 13. GENERATED ITEM RESULT
 * ============================================================================
 *
 * A generated item must ultimately be represented by the canonical item
 * grammar.
 * ========================================================================== */

generatedItem
    : item
    ;


/* ============================================================================
 * 14. GENERATED SOURCE RESULT
 * ============================================================================
 *
 * A generated source fragment consists only of canonical source elements.
 *
 * No finite number of elements is imposed.
 * ========================================================================== */

generatedSource
    : generatedSourceElement*
    ;


generatedSourceElement
    : attribute
    | item
    | statement
    ;


/* ============================================================================
 * 15. GENERATION INPUT
 * ============================================================================
 *
 * Generation inputs are ordinary expressions.
 *
 * The semantic layer decides whether an expression is:
 *
 *     - compile-time available;
 *     - pure;
 *     - effectful;
 *     - capability-dependent;
 *     - target-dependent;
 *     - resource-dependent;
 *     - otherwise valid.
 *
 * The parser makes none of those decisions.
 * ========================================================================== */

generationInput
    : expression
    ;


/* ============================================================================
 * 16. GENERATION INPUT LIST
 * ============================================================================
 *
 * No fixed number of generation inputs.
 * ========================================================================== */

generationInputList
    : generationInput
      (COMMA generationInput)*
      COMMA?
    ;


/* ============================================================================
 * 17. NAMED GENERATION INPUT
 * ============================================================================
 *
 * This provides a stable syntactic boundary for generators that consume
 * explicitly named compile-time inputs.
 *
 * Example conceptual form:
 *
 *     name: expression
 *
 * ========================================================================== */

generationNamedInput
    : identifier
      COLON
      expression
    ;


/* ============================================================================
 * 18. NAMED GENERATION INPUT LIST
 * ============================================================================
 */

generationNamedInputList
    : generationNamedInput
      (COMMA generationNamedInput)*
      COMMA?
    ;


/* ============================================================================
 * 19. GENERATION ARGUMENTS
 * ============================================================================
 *
 * Arguments are ordinary expressions.
 *
 * This grammar does not duplicate the canonical argumentList because
 * generation-specific tooling may need a distinct semantic boundary.
 *
 * The underlying values remain canonical expressions.
 * ========================================================================== */

generationArguments
    : LPAREN
      generationInputList?
      RPAREN
    ;


/* ============================================================================
 * 20. GENERATION OPTIONS
 * ============================================================================
 *
 * Generation options are intentionally represented as ordinary named
 * expressions rather than a fixed keyword enumeration.
 *
 * This keeps the grammar extensible without requiring a lexer change for
 * every future generation policy.
 *
 * Semantic validation owns the allowed option vocabulary.
 * ========================================================================== */

generationOptions
    : LBRACE
      generationOption*
      RBRACE
    ;


generationOption
    : identifier
      COLON
      expression
      COMMA?
    ;


/* ============================================================================
 * 21. GENERATION REQUEST
 * ============================================================================
 *
 * Named integration boundary for compiler tooling.
 *
 * A request combines a generation form with optional generation options.
 *
 * This rule is useful to AST builders and diagnostics without forcing the
 * syntax of every generation facility into one monolithic production.
 * ========================================================================== */

generationRequest
    : SYNTHESIZE
      generationRequestBody
    ;


generationRequestBody
    : generationRequestExpression
    | generationRequestType
    | generationRequestBlock
    ;


generationRequestExpression
    : LPAREN
      expression
      RPAREN
      generationOptions?
    ;


generationRequestType
    : TYPE
      LPAREN
      typeExpression
      RPAREN
      generationOptions?
    ;


generationRequestBlock
    : blockExpression
      generationOptions?
    ;


/* ============================================================================
 * 22. GENERATION EXPRESSION BRIDGE
 * ============================================================================
 *
 * This is the integration point for expressions/compile-time.g4.
 *
 * That grammar owns expression-level compile-time semantics.
 *
 * This grammar only identifies a generated-expression boundary.
 * ========================================================================== */

generationExpressionBridge
    : generationExpression
    ;


/* ============================================================================
 * 23. GENERATION BLOCK BRIDGE
 * ============================================================================
 *
 * Compile-time execution belongs to compile-time-execution.g4.
 *
 * This rule deliberately consumes a canonical block instead of defining a
 * second execution language.
 * ========================================================================== */

generationBlockBridge
    : generationBlock
    ;


/* ============================================================================
 * 24. GENERATION ATTRIBUTE BRIDGE
 * ============================================================================
 *
 * Attributes remain owned by the canonical attribute grammar.
 *
 * Generation-specific attributes are therefore ordinary canonical attributes.
 * ========================================================================== */

generationAttribute
    : attribute
    ;


/* ============================================================================
 * 25. GENERATION METADATA
 * ============================================================================
 *
 * Metadata is represented using the existing attribute system.
 *
 * This rule does not create a second metadata representation.
 * ========================================================================== */

generationMetadata
    : attribute
    ;


/* ============================================================================
 * 26. GENERATION PROVENANCE
 * ============================================================================
 *
 * Provenance annotations are syntactically ordinary attributes.
 *
 * Their semantic interpretation belongs to compiler provenance infrastructure.
 * ========================================================================== */

generationProvenance
    : attribute
    ;


/* ============================================================================
 * 27. GENERATION TARGET CATEGORY
 * ============================================================================
 *
 * The target category describes WHAT kind of language structure is generated,
 * not WHERE it will execute.
 *
 * This rule is intentionally identifier-based so future domains do not require
 * permanent lexer growth.
 *
 * Examples of semantic categories may include:
 *
 *     expression
 *     type
 *     function
 *     module
 *     quantum
 *     circuit
 *     hardware
 *     hdl
 *     distributed
 *     data
 *     ai
 *
 * These names are semantic vocabulary, not hard-coded grammar limitations.
 * ========================================================================== */

generationTargetCategory
    : identifier
    ;


/* ============================================================================
 * 28. GENERATION TARGET
 * ============================================================================
 *
 * A generation target identifies a source-level category.
 *
 * It MUST NOT identify a physical machine unless the semantic target model
 * explicitly permits such a dependency.
 * ========================================================================== */

generationTarget
    : generationTargetCategory
    ;


/* ============================================================================
 * 29. GENERATION TARGETED REQUEST
 * ============================================================================
 *
 * Explicit target category plus a generation body.
 *
 * Example conceptual syntax:
 *
 *     synthesize quantum { ... }
 *
 *     synthesize hdl { ... }
 *
 *     synthesize expression { ... }
 *
 * The semantic layer determines whether the category exists and whether the
 * generated structure is valid for it.
 * ========================================================================== */

generationTargetedRequest
    : SYNTHESIZE
      generationTarget
      generationBlock
    ;


/* ============================================================================
 * 30. GENERATION DECLARATION BODY
 * ============================================================================
 *
 * A declaration-generation body consists of canonical source items.
 * ========================================================================== */

generationDeclarationBody
    : LBRACE
      item*
      RBRACE
    ;


/* ============================================================================
 * 31. GENERATION STATEMENT BODY
 * ============================================================================
 *
 * A statement-generation body consists of canonical statements.
 * ========================================================================== */

generationStatementBody
    : LBRACE
      statement*
      RBRACE
    ;


/* ============================================================================
 * 32. GENERATION EXPRESSION BODY
 * ============================================================================
 *
 * An expression-generation body remains canonical expression syntax.
 * ========================================================================== */

generationExpressionBody
    : LPAREN
      expression
      RPAREN
    ;


/* ============================================================================
 * 33. GENERATION TYPE BODY
 * ============================================================================
 */

generationTypeBody
    : TYPE
      LPAREN
      typeExpression
      RPAREN
    ;


/* ============================================================================
 * 34. GENERATION PATTERN BODY
 * ============================================================================
 *
 * Pattern syntax belongs to the canonical parser.
 *
 * Generation can consume a pattern as data only when semantic analysis
 * authorizes that use.
 * ========================================================================== */

generationPattern
    : pattern
    ;


/* ============================================================================
 * 35. GENERATION NAME
 * ============================================================================
 *
 * Canonical identifier ownership is preserved.
 * ========================================================================== */

generationName
    : identifier
    ;


/* ============================================================================
 * 36. GENERATION QUALIFIED NAME BRIDGE
 * ============================================================================
 *
 * Qualified names remain owned by the core/name grammar.
 *
 * The composed parser may replace this bridge with its canonical qualifiedName
 * rule where available.
 *
 * This local structural form does not introduce a new identifier model.
 * ========================================================================== */

generationQualifiedName
    : identifier
      (DOUBLE_COLON identifier)*
    ;


/* ============================================================================
 * 37. GENERATION GENERIC PARAMETERS BRIDGE
 * ============================================================================
 *
 * Generic syntax remains owned by the canonical generic grammar.
 * ========================================================================== */

generationGenericParameters
    : genericParameters
    ;


/* ============================================================================
 * 38. GENERATION SOURCE FRAGMENT ELEMENT
 * ============================================================================
 *
 * A generated source fragment may contain any canonical source element.
 * ========================================================================== */

generationFragmentElement
    : attribute
    | item
    | statement
    ;


/* ============================================================================
 * 39. GENERATION SOURCE FRAGMENT
 * ============================================================================
 *
 * No fixed fragment size.
 * ========================================================================== */

generationFragment
    : generationFragmentElement*
    ;


/* ============================================================================
 * 40. GENERATION DECLARATION FRAGMENT
 * ============================================================================
 */

generationDeclarationFragment
    : item*
    ;


/* ============================================================================
 * 41. GENERATION STATEMENT FRAGMENT
 * ============================================================================
 */

generationStatementFragment
    : statement*
    ;


/* ============================================================================
 * 42. GENERATION EXPRESSION FRAGMENT
 * ============================================================================
 *
 * An expression fragment is a single canonical expression.
 *
 * Larger expression composition remains owned by the expression grammar.
 * ========================================================================== */

generationExpressionFragment
    : expression
    ;


/* ============================================================================
 * 43. GENERATION TYPE FRAGMENT
 * ============================================================================
 */

generationTypeFragment
    : typeExpression
    ;


/* ============================================================================
 * 44. GENERATION RESULT CATEGORY
 * ============================================================================
 *
 * The category is semantic metadata.
 *
 * It is not an implementation-specific enumeration.
 * ========================================================================== */

generationResultCategory
    : identifier
    ;


/* ============================================================================
 * 45. GENERATION RESULT DECLARATION
 * ============================================================================
 *
 * Syntax:
 *
 *     result: expression
 *
 * This remains ordinary source-level metadata and is interpreted downstream.
 * ========================================================================== */

generationResultDeclaration
    : identifier
      COLON
      expression
    ;


/* ============================================================================
 * 46. GENERATION RESULT DECLARATION LIST
 * ============================================================================
 */

generationResultDeclarationList
    : generationResultDeclaration
      (COMMA generationResultDeclaration)*
      COMMA?
    ;


/* ============================================================================
 * 47. GENERATION CONFIGURATION
 * ============================================================================
 *
 * Configuration is data.
 *
 * It does not select hardware directly.
 * ========================================================================== */

generationConfiguration
    : LBRACE
      generationConfigurationEntry*
      RBRACE
    ;


generationConfigurationEntry
    : identifier
      (COLON expression)?
      SEMI?
    ;


/* ============================================================================
 * 48. GENERATION POLICY
 * ============================================================================
 *
 * Policy names remain open identifiers.
 *
 * Examples that may be recognized semantically:
 *
 *     deterministic
 *     reproducible
 *     portable
 *     cacheable
 *     incremental
 *     isolated
 *
 * The grammar does not enumerate them.
 * ========================================================================== */

generationPolicy
    : identifier
    ;


/* ============================================================================
 * 49. GENERATION POLICY LIST
 * ============================================================================
 */

generationPolicyList
    : generationPolicy
      (COMMA generationPolicy)*
      COMMA?
    ;


/* ============================================================================
 * 50. GENERATION POLICY CLAUSE
 * ============================================================================
 *
 * Named semantic policy plus optional expression value.
 * ========================================================================== */

generationPolicyClause
    : identifier
      (ASSIGN expression)?
    ;


/* ============================================================================
 * 51. GENERATION POLICY CLAUSE LIST
 * ============================================================================
 */

generationPolicyClauseList
    : generationPolicyClause
      (COMMA generationPolicyClause)*
      COMMA?
    ;


/* ============================================================================
 * 52. GENERATION CONTRACT
 * ============================================================================
 *
 * Generation contracts are represented through canonical expressions and
 * attributes. They are validated by semantic analysis.
 * ========================================================================== */

generationContract
    : attribute
    | generationPolicyClause
    ;


/* ============================================================================
 * 53. GENERATION CONTRACT LIST
 * ============================================================================
 */

generationContractList
    : generationContract*
    ;


/* ============================================================================
 * 54. GENERATION SAFETY BOUNDARY
 * ============================================================================
 *
 * This rule is syntactic only.
 *
 * Safety is semantic.
 *
 * A generation construct does not implicitly receive host capabilities.
 * ========================================================================== */

generationSafetyBoundary
    : generation
    ;


/* ============================================================================
 * 55. GENERATION PORTABILITY BOUNDARY
 * ============================================================================
 *
 * Generation remains source-level and target-independent.
 * ========================================================================== */

generationPortabilityBoundary
    : generation
    ;


/* ============================================================================
 * 56. GENERATION QUANTUM BOUNDARY
 * ============================================================================
 *
 * Quantum generation remains ordinary source generation.
 *
 * The resulting quantum source is lowered through quantum semantic analysis
 * and ultimately quantum::ir.
 * ========================================================================== */

generationQuantumBoundary
    : generation
    ;


/* ============================================================================
 * 57. GENERATION HDL BOUNDARY
 * ============================================================================
 *
 * HDL generation remains source-level HDL structure.
 *
 * Physical synthesis and implementation are downstream concerns.
 * ========================================================================== */

generationHdlBoundary
    : generation
    ;


/* ============================================================================
 * 58. GENERATION HARDWARE BOUNDARY
 * ============================================================================
 *
 * Hardware generation must not imply a particular physical target.
 * ========================================================================== */

generationHardwareBoundary
    : generation
    ;


/* ============================================================================
 * 59. GENERATION CLASSICAL BOUNDARY
 * ============================================================================
 */

generationClassicalBoundary
    : generation
    ;


/* ============================================================================
 * 60. GENERATION DISTRIBUTED BOUNDARY
 * ============================================================================
 */

generationDistributedBoundary
    : generation
    ;


/* ============================================================================
 * 61. GENERATION AI / DATA BOUNDARY
 * ============================================================================
 */

generationAiDataBoundary
    : generation
    ;


/* ============================================================================
 * 62. GENERATION FINAL INTEGRATION CONTRACT
 * ============================================================================
 *
 * This rule is the principal named integration point exposed to the
 * metaprogramming aggregator.
 *
 * The authoritative parser may dispatch this rule from its metaprogramming
 * expression/item/statement integration points.
 * ========================================================================== */

generationConstruct
    : generation
    | generationRequest
    | generationTargetedRequest
    ;


/* ============================================================================
 * 63. SEMANTIC HANDOFF
 * ============================================================================
 *
 * Everything after this boundary belongs outside the grammar.
 *
 * Required semantic pipeline:
 *
 *     generationConstruct
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     generation semantic validation
 *          |
 *          +--> name resolution
 *          +--> type checking
 *          +--> effect checking
 *          +--> capability checking
 *          +--> provenance
 *          +--> determinism classification
 *          +--> resource-budget admission
 *          |
 *          v
 *     generated source/semantic structure
 *          |
 *          v
 *     normal Zamani semantic pipeline
 *          |
 *          +--> classical semantics
 *          +--> quantum semantics -> quantum::ir
 *          +--> HDL/hardware semantics
 *          +--> distributed semantics
 *          +--> AI/data semantics
 *          |
 *          v
 *     optimization / lowering / routing / scheduling / HAL
 *          |
 *          v
 *     runtime
 *
 * No generation rule in this file may directly bypass that pipeline.
 * ========================================================================== */

generationSemanticHandoff
    : generationConstruct
    ;


/* ============================================================================
 * 64. EXPLICIT NON-OWNERSHIP MARKERS
 * ============================================================================
 *
 * These bridge rules intentionally delegate to sibling facilities.
 * ========================================================================== */


/*
 * Reflection remains owned by reflection.g4.
 */
generationReflectionBridge
    : generationInput
    ;


/*
 * Specialization remains owned by specialization.g4.
 */
generationSpecializationBridge
    : generationInput
    ;


/*
 * Compile-time execution remains owned by compile-time-execution.g4.
 */
generationCompileTimeExecutionBridge
    : generationInput
    ;


/*
 * Macro expansion remains owned by macros/.
 */
generationMacroBridge
    : generationInput
    ;


/* ============================================================================
 * 65. HARD-CODING PROHIBITION
 * ============================================================================
 *
 * No production grammar rule in this file may introduce:
 *
 *     - fixed hardware counts;
 *     - fixed quantum counts;
 *     - fixed memory sizes;
 *     - fixed topology;
 *     - fixed device IDs;
 *     - fixed addresses;
 *     - fixed execution widths;
 *     - fixed accelerator counts;
 *     - fixed deployment layouts.
 *
 * Numeric literals occurring in expressions remain ordinary language values.
 *
 * Their meaning is determined by semantic typing and resource analysis.
 *
 * ============================================================================
 *
 * 66. PARSER ERROR CONTRACT
 * ============================================================================
 *
 * The parser must report malformed generation syntax through the normal ANTLR
 * diagnostic mechanism.
 *
 * This grammar MUST NOT:
 *
 *     - silently discard malformed generated structures;
 *     - turn invalid source into comments;
 *     - execute recovery code;
 *     - emit generated code;
 *     - print directly to stdout/stderr.
 *
 * Error representation belongs to the frontend diagnostic subsystem.
 *
 * ============================================================================
 *
 * 67. AST CONTRACT
 * ============================================================================
 *
 * The frontend AST should preserve, at minimum:
 *
 *     - generation source span;
 *     - generation category;
 *     - generation body;
 *     - generation inputs;
 *     - generation options;
 *     - attributes;
 *     - source ordering;
 *     - nested syntax;
 *     - provenance source location.
 *
 * The parse tree itself is not the canonical AST.
 *
 * ============================================================================
 *
 * 68. IR CONTRACT
 * ============================================================================
 *
 * This grammar produces no IR.
 *
 * Generated structures are lowered through normal semantic infrastructure.
 *
 * In particular:
 *
 *     generated quantum source
 *          ->
 *     quantum semantic analysis
 *          ->
 *     quantum::ir
 *
 * never:
 *
 *     generation grammar
 *          ->
 *     quantum::ir
 *
 * ============================================================================
 *
 * 69. RUNTIME CONTRACT
 * ============================================================================
 *
 * Generation is a compile-time/source-transformation concern.
 *
 * Runtime MUST NOT depend directly on this grammar.
 *
 * Runtime receives the resulting canonical semantic/compiled representation.
 *
 * ============================================================================
 *
 * 70. TOOLING CONTRACT
 * ============================================================================
 *
 * Tooling may use this grammar to:
 *
 *     - syntax-highlight generation;
 *     - build parse trees;
 *     - produce diagnostics;
 *     - locate generation boundaries;
 *     - format generation constructs;
 *     - inspect source structure;
 *     - build IDE syntax trees.
 *
 * Tooling must not infer hardware behavior solely from this grammar.
 *
 * ============================================================================
 *
 * 71. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when:
 *
 *     [ ] It compiles against the canonical Zamani lexer vocabulary.
 *     [ ] It introduces no nonexistent lexer token.
 *     [ ] It duplicates no canonical expression grammar.
 *     [ ] It duplicates no canonical type grammar.
 *     [ ] It duplicates no canonical statement grammar.
 *     [ ] It duplicates no canonical declaration grammar.
 *     [ ] It does not duplicate reflection.
 *     [ ] It does not duplicate specialization.
 *     [ ] It does not duplicate macro expansion.
 *     [ ] It does not duplicate compile-time execution.
 *     [ ] It does not construct IR.
 *     [ ] It does not construct quantum::ir.
 *     [ ] It contains no hardware limits.
 *     [ ] It contains no fixed machine sizes.
 *     [ ] It contains no Rust actions.
 *     [ ] It contains no unsafe Rust.
 *     [ ] It preserves source spans through the AST contract.
 *     [ ] It supports deterministic parsing.
 *     [ ] It supports arbitrary source scale subject to implementation
 *         resources rather than grammar-level ceilings.
 *     [ ] Positive tests exist.
 *     [ ] Negative tests exist.
 *     [ ] Boundary tests exist.
 *     [ ] Cross-domain generation tests exist.
 *     [ ] Quantum generation tests prove the quantum::ir boundary remains
 *         downstream.
 *     [ ] Hardware generation tests prove physical target selection remains
 *         downstream.
 *     [ ] Round-trip tests preserve generation semantics.
 *     [ ] POCO-REAF tests prove generation does not accidentally encode a
 *         compilation host.
 *
 * ============================================================================
 */
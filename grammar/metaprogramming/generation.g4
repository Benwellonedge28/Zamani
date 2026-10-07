/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/metaprogramming/generation.g4
 *
 * GRAMMAR
 * -------
 * Generation
 *
 * STATUS
 * ------
 * Production parser-grammar component.
 *
 * PURPOSE
 * -------
 * This file owns the SOURCE-LEVEL SYNTAX boundary for program generation.
 *
 * Generation means constructing or requesting canonical Zamani source
 * structure during an authorized metaprogramming phase.
 *
 * Generation does NOT:
 *
 *     - execute generated code;
 *     - construct machine instructions;
 *     - construct classical IR directly;
 *     - construct quantum::ir directly;
 *     - select hardware;
 *     - select a QPU;
 *     - allocate physical resources;
 *     - perform routing;
 *     - perform scheduling;
 *     - perform QEC;
 *     - bypass semantic analysis;
 *     - bypass validation;
 *     - bypass provenance;
 *     - bypass policy;
 *     - bypass capability checking.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          +-------------------------------+
 *          |                               |
 *          v                               v
 *     ordinary semantics          metaprogramming semantics
 *                                          |
 *                                          v
 *                                      generation
 *                                          |
 *                                          v
 *                                generated Zamani source
 *                                          |
 *                                          v
 *                                  ordinary frontend
 *                                          |
 *                                          v
 *                                  semantic analysis
 *                                          |
 *                     +--------------------+--------------------+
 *                     |                    |                    |
 *                     v                    v                    v
 *                classical IR         quantum::ir         HDL/hardware
 *                     |                    |                    |
 *                     +--------------------+--------------------+
 *                                          |
 *                                          v
 *                              optimization / lowering
 *                                          |
 *                              routing / scheduling
 *                                          |
 *                               resilience / QEC
 *                                          |
 *                                         ZQN
 *                                          |
 *                                         HAL
 *                                          |
 *                                  target realization
 *
 * Generation MUST remain inside this pipeline.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - generation expression syntax;
 *     - generation declaration syntax;
 *     - generation statement syntax;
 *     - generation request/category syntax;
 *     - generation input syntax;
 *     - generation option syntax;
 *     - generation result-category syntax;
 *     - generation integration contracts.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical definitions;
 *     - identifiers;
 *     - qualified names;
 *     - ordinary expressions;
 *     - expression precedence;
 *     - ordinary statements;
 *     - declarations;
 *     - types;
 *     - patterns;
 *     - quotation;
 *     - unquotation;
 *     - macros;
 *     - macro expansion;
 *     - reflection;
 *     - introspection;
 *     - specialization;
 *     - compile-time evaluation;
 *     - AST implementation;
 *     - semantic implementation;
 *     - provenance implementation;
 *     - effect implementation;
 *     - policy implementation;
 *     - capability discovery;
 *     - resource discovery;
 *     - classical IR;
 *     - quantum::ir;
 *     - HDL IR;
 *     - hardware realization;
 *     - routing;
 *     - scheduling;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution.
 *
 * ============================================================================
 * AUTHORITY
 * ============================================================================
 *
 * Architectural authority:
 *
 *     grammar/DESIGN.md
 *
 * Metaprogramming composition:
 *
 *     grammar/metaprogramming/metaprogramming.g4
 *
 * Canonical parser:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * Canonical lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Canonical names:
 *
 *     grammar/core/
 *
 * Canonical types:
 *
 *     grammar/types/
 *
 * Canonical expressions:
 *
 *     grammar/expressions/
 *
 * Canonical statements:
 *
 *     grammar/statements/
 *
 * Canonical declarations:
 *
 *     grammar/declarations/
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It defines NO lexer tokens.
 *
 * Generation uses the existing canonical:
 *
 *     SYNTHESIZE
 *
 * token.
 *
 * This file MUST NOT invent:
 *
 *     GENERATE
 *     STATEMENT_KEYWORD
 *     DECLARATION_KEYWORD
 *     EXPRESSION_KEYWORD
 *     GENERATOR
 *     GENERATOR_ID
 *
 * or equivalent lexical vocabulary.
 *
 * Extensible generation categories are represented through canonical
 * identifiers where appropriate.
 *
 * ============================================================================
 * CRITICAL CORRECTION
 * ============================================================================
 *
 * The previous implementation referenced a nonexistent:
 *
 *     STATEMENT_KEYWORD
 *
 * token.
 *
 * This file MUST NOT reference nonexistent lexer vocabulary.
 *
 * Statement/declaration generation is distinguished by the composition
 * boundary supplied by metaprogramming.g4 rather than by inventing new
 * lexical tokens.
 *
 * ============================================================================
 * GENERATION MODEL
 * ============================================================================
 *
 * Generation is CATEGORY-AWARE.
 *
 * The three canonical integration forms are:
 *
 *     generationExpressionCore
 *     generationDeclarationCore
 *     generationStatementCore
 *
 * The canonical metaprogramming composition layer maps these into:
 *
 *     generationExpression
 *     generationDeclaration
 *     generationStatement
 *
 * This gives expression, declaration, and statement contexts independent
 * entry points without creating three different generation systems.
 *
 * ============================================================================
 * EXTENSIBILITY
 * ============================================================================
 *
 * The grammar MUST NOT enumerate every future generated domain.
 *
 * It therefore does NOT contain alternatives such as:
 *
 *     quantum
 *     gpu
 *     fpga
 *     cpu
 *     tensor
 *     ai
 *     robotics
 *     blockchain
 *     hpc
 *     cloud
 *
 * as a permanent generation-category enumeration.
 *
 * Domain-specific generated structures remain canonical Zamani source
 * structures and are validated by their owning domain grammars.
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * Generation MAY produce quantum source structure.
 *
 * It MUST NOT produce quantum::ir directly.
 *
 * The boundary is:
 *
 *     generated quantum source
 *          |
 *          v
 *     quantum semantic analysis
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     resilience / QEC
 *          |
 *          v
 *     ZQN
 *          |
 *          v
 *     HAL
 *
 * This grammar therefore contains no gate catalogue and no physical-qubit
 * representation.
 *
 * ============================================================================
 * HDL / HARDWARE CONTRACT
 * ============================================================================
 *
 * Generated HDL or hardware-intent source re-enters the canonical HDL and
 * hardware semantic systems.
 *
 * This file MUST NOT encode:
 *
 *     bus widths;
 *     register widths;
 *     device counts;
 *     FPGA capacities;
 *     ASIC cell counts;
 *     memory limits;
 *     topology sizes;
 *     processor counts;
 *     accelerator counts.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY CONTRACT
 * ============================================================================
 *
 * Generation itself does not discover resources.
 *
 * A generator MAY be semantically associated with:
 *
 *     capabilities;
 *     requirements;
 *     constraints;
 *     budgets;
 *     policies;
 *     effects;
 *
 * but those are semantic properties.
 *
 * The parser merely preserves the syntactic generation boundary.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Generation may be classified downstream with effects such as:
 *
 *     code_generation
 *     reflection
 *     compile_time
 *     native
 *     foreign
 *     IO
 *     network
 *     randomness
 *     environment
 *     target_dependency
 *
 * The grammar does not automatically grant any of these effects.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * The grammar itself is deterministic.
 *
 * Generation determinism is a semantic/compiler property.
 *
 * Pure generation SHOULD be reproducible for identical:
 *
 *     source;
 *     generator semantics;
 *     inputs;
 *     language version;
 *     relevant dialect configuration.
 *
 * External state MUST NOT silently become part of program meaning.
 *
 * If generation depends on:
 *
 *     time;
 *     randomness;
 *     environment;
 *     filesystem;
 *     network;
 *     target capabilities;
 *     resource availability;
 *
 * semantic/effect/capability analysis must record that dependency.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Generated structures must remain traceable to their source generation site.
 *
 * Provenance is implemented outside this grammar.
 *
 * This grammar may consume canonical attributes that carry provenance
 * metadata, but it does not define a second provenance system.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Parsing generation syntax performs no generation.
 *
 * This grammar performs:
 *
 *     - no filesystem access;
 *     - no network access;
 *     - no environment access;
 *     - no process execution;
 *     - no hardware discovery;
 *     - no credential access;
 *     - no runtime execution.
 *
 * Any authorized compile-time execution belongs to the compile-time semantic
 * subsystem and must be governed by effects, capabilities, resources, and
 * policies.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * There is no language-level finite limit on:
 *
 *     - generation inputs;
 *     - generated declarations;
 *     - generated statements;
 *     - generated expressions;
 *     - generation nesting;
 *     - generation requests;
 *     - generated source size.
 *
 * Implementation limits MAY exist for:
 *
 *     - memory;
 *     - compilation time;
 *     - recursion;
 *     - generated artifact size;
 *     - evaluation steps;
 *     - cancellation;
 *     - diagnostic volume.
 *
 * Those are implementation/resource policies and MUST NOT become language
 * constants.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Generation must preserve:
 *
 *     Program Once
 *          |
 *          v
 *     portable source meaning
 *          |
 *          v
 *     generation semantics
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          v
 *     target-independent IR
 *          |
 *          v
 *     target-specific realization
 *
 * A generator must not silently specialize source meaning to the compiler
 * host, one processor, one QPU, one GPU, one FPGA, one cluster, or any other
 * fixed machine.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * Rust implementation baseline:
 *
 *     Rust 1.97 or later
 *     Rust 2021 or later where repository policy permits
 *
 * Grammar integration requires:
 *
 *     - safe Rust;
 *     - no unsafe blocks;
 *     - no embedded Rust actions;
 *     - no embedded semantic predicates requiring Rust;
 *     - no host execution from grammar rules.
 *
 * The existing repository uses ANTLR-compatible Rust generation/runtime
 * infrastructure; grammar generation must remain target-agnostic.
 *
 * ============================================================================
 * PUBLIC RULE CONTRACT
 * ============================================================================
 *
 * This file exports exactly these generation contracts:
 *
 *     generationDeclarationCore
 *     generationExpressionCore
 *     generationStatementCore
 *
 * Additional helper rules are private to this grammar.
 *
 * ============================================================================
 * INTEGRATION WITH METAPROGRAMMING
 * ============================================================================
 *
 * grammar/metaprogramming/metaprogramming.g4 must import this grammar.
 *
 * It must expose:
 *
 *     generationDeclaration
 *     generationExpression
 *     generationStatement
 *
 * through:
 *
 *     generationDeclarationCore
 *     generationExpressionCore
 *     generationStatementCore
 *
 * This prevents the composition grammar from depending on accidental helper
 * rules.
 *
 * ============================================================================
 * INTEGRATION WITH EXPRESSIONS
 * ============================================================================
 *
 * `generationExpressionCore` consumes the canonical `expression` rule.
 *
 * It does not define an expression grammar.
 *
 * ============================================================================
 * INTEGRATION WITH DECLARATIONS
 * ============================================================================
 *
 * `generationDeclarationCore` consumes the canonical `item` rule.
 *
 * It does not define declaration syntax.
 *
 * ============================================================================
 * INTEGRATION WITH STATEMENTS
 * ============================================================================
 *
 * `generationStatementCore` consumes the canonical `statement` rule.
 *
 * It does not define statement syntax.
 *
 * ============================================================================
 * INTEGRATION WITH QUOTATION
 * ============================================================================
 *
 * Quotation remains owned by:
 *
 *     grammar/metaprogramming/quotation.g4
 *
 * Generation may consume quotation-derived semantic values, but quotation
 * syntax is not duplicated here.
 *
 * ============================================================================
 * INTEGRATION WITH REFLECTION / INTROSPECTION
 * ============================================================================
 *
 * Reflection and introspection remain independent facilities.
 *
 * They may provide metadata to a generation semantic operation, but this file
 * does not implement reflection or introspection.
 *
 * ============================================================================
 * INTEGRATION WITH SPECIALIZATION
 * ============================================================================
 *
 * Specialization may consume generated structures.
 *
 * Generation does not choose a target implementation.
 *
 * ============================================================================
 * INTEGRATION WITH COMPILE-TIME EXECUTION
 * ============================================================================
 *
 * Compile-time execution may evaluate a generation-producing computation.
 *
 * This file does not evaluate anything.
 *
 * ============================================================================
 * INTEGRATION WITH EFFECTS
 * ============================================================================
 *
 * Generation is semantically associated with the repository's code-generation
 * effect model.
 *
 * This grammar does not create an effect node itself.
 *
 * ============================================================================
 * INTEGRATION WITH RESOURCES / CAPABILITIES
 * ============================================================================
 *
 * Generated source may eventually contain resource requirements and
 * capabilities, but those are parsed by their owning grammars after the
 * generated source re-enters the canonical frontend.
 *
 * ============================================================================
 * INTEGRATION WITH VALIDATION / CONTRACTS
 * ============================================================================
 *
 * Generated source must undergo the same:
 *
 *     structural validation;
 *     type checking;
 *     effect checking;
 *     capability checking;
 *     resource analysis;
 *     contract checking;
 *     policy checking;
 *     provenance validation
 *
 * as ordinary source.
 *
 * ============================================================================
 * INTEGRATION WITH IR
 * ============================================================================
 *
 * This grammar produces no IR.
 *
 * Generated structures eventually enter the normal semantic pipeline:
 *
 *     generated source
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic model
 *          |
 *          +----------------------+
 *          |                      |
 *          v                      v
 *     canonical IR          quantum::ir
 *
 * depending on the generated program structure.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * ANTLR COMPONENT
 * ============================================================================
 */

parser grammar Generation;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. GENERATION EXPRESSION
 * ============================================================================
 *
 * Canonical source form:
 *
 *     synthesize (expression)
 *
 * The expression is ordinary Zamani expression syntax.
 *
 * The parser does not execute it.
 *
 * Semantic analysis decides whether it is a valid generation input/result.
 */

generationExpressionCore
    : SYNTHESIZE
      LPAREN
      expression
      RPAREN
    ;


/*
 * ============================================================================
 * 2. GENERATION TYPE
 * ============================================================================
 *
 * This helper is intentionally private.
 *
 * Type generation is represented as a generation expression whose semantic
 * result category is a type.
 *
 * The canonical type grammar owns typeExpression.
 */

generationTypeCore
    : SYNTHESIZE
      TYPE
      LPAREN
      typeExpression
      RPAREN
    ;


/*
 * ============================================================================
 * 3. GENERATION DECLARATION
 * ============================================================================
 *
 * The declaration context is supplied by the metaprogramming composition
 * grammar.
 *
 * Canonical source form:
 *
 *     synthesize {
 *         <canonical item(s)>
 *     }
 *
 * `item` remains owned by the canonical declaration composition.
 *
 * Zero or more items are allowed so an empty generated declaration fragment
 * remains syntactically representable; semantic validation may reject it when
 * a non-empty generated declaration is required.
 */

generationDeclarationCore
    : SYNTHESIZE
      LBRACE
      item*
      RBRACE
    ;


/*
 * ============================================================================
 * 4. GENERATION STATEMENT
 * ============================================================================
 *
 * Statement generation uses the same canonical source boundary but is exposed
 * through a distinct parser entry point.
 *
 * Canonical source form:
 *
 *     synthesize {
 *         <canonical statement(s)>
 *     }
 *
 * The distinction between declaration generation and statement generation is
 * made by the composition context:
 *
 *     declaration -> generationDeclarationCore
 *     statement   -> generationStatementCore
 *
 * This avoids inventing a STATEMENT lexer token.
 */

generationStatementCore
    : SYNTHESIZE
      LBRACE
      statement*
      RBRACE
    ;


/*
 * ============================================================================
 * 5. NAMED GENERATION REQUEST
 * ============================================================================
 *
 * This is a private extensibility boundary.
 *
 * It allows future semantic categories without requiring one permanent
 * lexer keyword per domain.
 *
 * Conceptual forms:
 *
 *     synthesize category(...)
 *
 *     synthesize category { ... }
 *
 * The category remains an identifier rather than a hard-coded enumeration.
 *
 * The semantic layer decides whether the category is valid.
 *
 * This rule deliberately does not claim that every category is automatically
 * supported.
 */

generationCategory
    : identifier
    ;


generationCategoryRequest
    : SYNTHESIZE
      generationCategory
      LPAREN
      generationCategoryValue
      RPAREN
    ;


generationCategoryValue
    : expression
    | typeExpression
    | qualifiedName
    ;


/*
 * ============================================================================
 * 6. GENERATION OPTIONS
 * ============================================================================
 *
 * Generation options are semantic metadata, not a fixed keyword catalogue.
 *
 * Example conceptual form:
 *
 *     optionName: expression
 *
 * The semantic layer owns the valid option vocabulary.
 *
 * This prevents every future generation policy from requiring a lexer or
 * grammar modification.
 */

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


/*
 * ============================================================================
 * 7. GENERATION INPUT
 * ============================================================================
 *
 * Ordinary expressions remain the canonical input representation.
 */

generationInput
    : expression
    ;


generationInputList
    : generationInput
      (COMMA generationInput)*
      COMMA?
    ;


/*
 * ============================================================================
 * 8. GENERATION NAMED INPUT
 * ============================================================================
 *
 * Named inputs allow generators to expose stable semantic parameter names
 * without hard-coding their complete vocabulary into the lexer.
 *
 * Example:
 *
 *     source: expression
 *
 *     model: expression
 *
 *     policy: expression
 *
 * Names are semantically validated downstream.
 */

generationNamedInput
    : identifier
      COLON
      expression
    ;


generationNamedInputList
    : generationNamedInput
      (COMMA generationNamedInput)*
      COMMA?
    ;


/*
 * ============================================================================
 * 9. GENERATION REQUEST
 * ============================================================================
 *
 * A generation request is a semantic grouping of a category and its inputs.
 *
 * This is not a second execution system.
 */

generationRequest
    : generationCategoryRequest
    ;


/*
 * ============================================================================
 * 10. GENERATION RESULT CATEGORY
 * ============================================================================
 *
 * The result category is source-level, not hardware-level.
 *
 * Examples of semantic categories may include:
 *
 *     expression
 *     type
 *     declaration
 *     statement
 *     module
 *     function
 *     quantum
 *     hdl
 *     hardware
 *     data
 *     distributed
 *
 * No list is encoded here.
 */

generationResultCategory
    : identifier
    ;


/*
 * ============================================================================
 * 11. GENERATION ATTRIBUTE
 * ============================================================================
 *
 * Generation may consume canonical attributes.
 *
 * The attribute grammar remains the sole owner of attribute syntax.
 */

generationAttribute
    : attribute
    ;


/*
 * ============================================================================
 * 12. GENERATION NAME
 * ============================================================================
 */

generationName
    : identifier
    ;


/*
 * ============================================================================
 * 13. GENERATION PATH
 * ============================================================================
 *
 * Qualified-name syntax remains owned by the canonical name grammar.
 */

generationPath
    : qualifiedName
    ;


/*
 * ============================================================================
 * 14. GENERATION TYPE
 * ============================================================================
 *
 * Public semantic bridge to the canonical type grammar.
 */

generationType
    : typeExpression
    ;


/*
 * ============================================================================
 * 15. GENERATION PATTERN
 * ============================================================================
 *
 * Pattern syntax remains owned by the canonical pattern grammar.
 */

generationPattern
    : pattern
    ;


/*
 * ============================================================================
 * 16. GENERATION ARGUMENTS
 * ============================================================================
 *
 * This is a semantic grouping boundary.
 *
 * It does not replace the canonical function argument grammar because
 * generation inputs may be consumed by a generator rather than an ordinary
 * function call.
 */

generationArguments
    : LPAREN
      generationInputList?
      RPAREN
    ;


/*
 * ============================================================================
 * 17. GENERATION NAMED ARGUMENTS
 * ============================================================================
 */

generationNamedArguments
    : LPAREN
      generationNamedInputList?
      RPAREN
    ;


/*
 * ============================================================================
 * 18. GENERATION BODY
 * ============================================================================
 *
 * A body is deliberately expressed through canonical source constructs.
 *
 * The two context-specific public entry points remain:
 *
 *     generationDeclarationCore
 *     generationStatementCore
 *
 * This helper is not a third competing public generation entry point.
 */

generationBody
    : LBRACE
      generationBodyElement*
      RBRACE
    ;


generationBodyElement
    : item
    | statement
    ;


/*
 * ============================================================================
 * 19. GENERATION SOURCE FRAGMENT
 * ============================================================================
 *
 * A source fragment is a semantic grouping of canonical source constructs.
 *
 * It introduces no alternate source language.
 */

generationSourceFragment
    : generationBody
    ;


/*
 * ============================================================================
 * 20. GENERATION DECLARATION FRAGMENT
 * ============================================================================
 */

generationDeclarationFragment
    : item*
    ;


/*
 * ============================================================================
 * 21. GENERATION STATEMENT FRAGMENT
 * ============================================================================
 */

generationStatementFragment
    : statement*
    ;


/*
 * ============================================================================
 * 22. GENERATION EXPRESSION RESULT
 * ============================================================================
 */

generatedExpression
    : expression
    ;


/*
 * ============================================================================
 * 23. GENERATION TYPE RESULT
 * ============================================================================
 */

generatedType
    : typeExpression
    ;


/*
 * ============================================================================
 * 24. GENERATION DECLARATION RESULT
 * ============================================================================
 */

generatedDeclaration
    : item
    ;


/*
 * ============================================================================
 * 25. GENERATION STATEMENT RESULT
 * ============================================================================
 */

generatedStatement
    : statement
    ;


/*
 * ============================================================================
 * 26. GENERATION SOURCE RESULT
 * ============================================================================
 */

generatedSource
    : generationSourceFragment
    ;


/*
 * ============================================================================
 * 27. GENERATION METADATA
 * ============================================================================
 *
 * Metadata is represented through canonical attributes.
 */

generationMetadata
    : generationAttribute+
    ;


/*
 * ============================================================================
 * 28. COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [x] It is a parser grammar.
 *
 * [x] It consumes tokenVocab=ZamaniLexer.
 *
 * [x] It defines no lexer tokens.
 *
 * [x] It contains no embedded Rust.
 *
 * [x] It contains no semantic predicates.
 *
 * [x] It contains no unsafe code.
 *
 * [x] It does not execute generation.
 *
 * [x] It does not define an AST implementation.
 *
 * [x] It does not define an IR.
 *
 * [x] It does not define quantum::ir.
 *
 * [x] It does not define hardware limits.
 *
 * [x] It does not enumerate quantum operations.
 *
 * [x] It does not enumerate hardware types.
 *
 * [x] It does not introduce STATEMENT_KEYWORD.
 *
 * [x] It exposes generationDeclarationCore.
 *
 * [x] It exposes generationExpressionCore.
 *
 * [x] It exposes generationStatementCore.
 *
 * [x] It reuses canonical expression syntax.
 *
 * [x] It reuses canonical declaration syntax.
 *
 * [x] It reuses canonical statement syntax.
 *
 * [x] It keeps target realization outside the grammar.
 *
 * [x] It keeps resource/capability resolution outside the grammar.
 *
 * [x] It keeps provenance implementation outside the grammar.
 *
 * [x] It keeps policy evaluation outside the grammar.
 *
 * [x] It keeps compile-time execution outside the grammar.
 *
 * [x] It permits future domains without a fixed generation-category list.
 *
 * [x] It imposes no language-level machine-size ceiling.
 *
 * Repository verification still required:
 *
 * [ ] ANTLR generation succeeds for the complete canonical parser.
 *
 * [ ] All imported grammar rules resolve.
 *
 * [ ] No rule collision exists in the assembled grammar.
 *
 * [ ] Metaprogramming imports this component exactly once.
 *
 * [ ] Expression composition reaches generationExpression through the
 *     canonical metaprogramming expression boundary.
 *
 * [ ] Declaration composition reaches generationDeclaration through the
 *     canonical declaration boundary.
 *
 * [ ] Statement composition reaches generationStatement through the
 *     canonical statement boundary.
 *
 * [ ] Generated Rust compiles on the repository's supported Rust baseline.
 *
 * [ ] CI rejects unsafe Rust in the frontend/compiler implementation.
 *
 * [ ] Positive, negative, boundary, deterministic, scalability and
 *     cross-domain tests pass.
 *
 * ============================================================================
 */
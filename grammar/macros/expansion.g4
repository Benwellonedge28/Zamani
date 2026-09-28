
/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/macros/expansion.g4
 *
 * Grammar:
 *     expansion
 *
 * Status:
 *     Canonical macro-expansion syntax component
 *
 * Language:
 *     Zamani
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Edition:
 *     Rust 2021
 *
 * Safety:
 *     Safe Rust only.
 *     No unsafe Rust.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * ============================================================================
 * 1. PURPOSE
 * ============================================================================
 *
 * Defines the parser-level syntax boundary for macro expansion metadata.
 *
 * This grammar describes source constructs that annotate or constrain
 * macro expansion.
 *
 * It does NOT perform macro expansion.
 *
 * It does NOT execute compile-time code.
 *
 * It does NOT generate source files.
 *
 * It does NOT mutate the AST.
 *
 * Expansion is a compiler transformation performed after parsing and macro
 * resolution.
 *
 * ============================================================================
 * 2. ARCHITECTURAL POSITION
 * ============================================================================
 *
 * Source
 *   |
 *   v
 * Canonical lexer
 *   |
 *   v
 * Canonical parser
 *   |
 *   +--> macro declarations
 *   |
 *   +--> macro invocations
 *   |
 *   +--> expansion metadata
 *   |
 *   v
 * Frontend AST
 *   |
 *   v
 * Name and macro resolution
 *   |
 *   v
 * Expansion planning
 *   |
 *   v
 * Controlled expansion
 *   |
 *   v
 * Hygiene and provenance validation
 *   |
 *   v
 * Semantic analysis
 *   |
 *   v
 * Canonical semantic model / IR
 *   |
 *   +--> classical computation
 *   +--> quantum::ir
 *   +--> HDL / hardware
 *   +--> hybrid computation
 *   +--> distributed computation
 *   +--> future domains
 *   |
 *   v
 * Optimization / lowering / scheduling / routing
 *   |
 *   v
 * Target realization
 *
 * ============================================================================
 * 3. OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - expansion metadata syntax;
 *   - reusable expansion annotation boundaries;
 *   - expansion-related parser integration points;
 *   - syntactic grouping of expansion metadata.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - macro declarations;
 *   - macro parameters;
 *   - macro invocation syntax;
 *   - macro paths;
 *   - identifiers;
 *   - qualified names;
 *   - expressions;
 *   - blocks;
 *   - attributes or annotations themselves;
 *   - lexer tokens;
 *   - name resolution;
 *   - binding resolution;
 *   - expansion algorithms;
 *   - expansion ordering;
 *   - recursion detection;
 *   - expansion caching;
 *   - hygiene implementation;
 *   - provenance implementation;
 *   - AST mutation;
 *   - token generation;
 *   - arbitrary compile-time execution;
 *   - filesystem or network access;
 *   - process execution;
 *   - compiler plugins;
 *   - target discovery;
 *   - hardware selection;
 *   - resource allocation;
 *   - canonical IR construction;
 *   - quantum::ir;
 *   - runtime execution.
 *
 * ============================================================================
 * 4. SINGLE-AUTHORITY CONTRACT
 * ============================================================================
 *
 * Macro declarations:
 *
 *     grammar/macros/declarations.g4
 *
 * Macro invocations:
 *
 *     grammar/macros/invocations.g4
 *
 * Macro hygiene:
 *
 *     grammar/macros/hygiene.g4
 *
 * Macro composition:
 *
 *     grammar/macros/macros.g4
 *
 * Macro expression integration:
 *
 *     grammar/expressions/macros.g4
 *
 * Canonical annotation syntax:
 *
 *     canonical core/declaration/annotation grammar
 *
 * This component consumes shared rules. It does not redefine them.
 *
 * ============================================================================
 * 5. LEXER CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * All tokens originate from the canonical Zamani lexer.
 *
 * No lexer rules are defined here.
 *
 * No new token names are introduced by this file.
 *
 * In particular, this grammar does not invent tokens for:
 *
 *     QUOTE
 *     UNQUOTE
 *     SPLICE
 *     QUASIQUOTE
 *     EXPAND
 *     EVAL
 *     TOKEN
 *     SYNTAX
 *
 * Ordinary identifiers must not become reserved words merely because
 * they resemble names used by a possible future expansion facility.
 *
 * ============================================================================
 * 6. SHARED RULE CONTRACT
 * ============================================================================
 *
 * The complete parser composition must provide:
 *
 *     annotation
 *     macroDeclaration
 *     macroInvocation
 *
 * These are external integration dependencies.
 *
 * Their canonical definitions must remain in their respective owners.
 *
 * This component does not create duplicate definitions.
 *
 * ============================================================================
 * 7. EXPANSION METADATA
 * ============================================================================
 *
 * Expansion metadata uses the canonical annotation representation.
 *
 * The annotation's semantic identity is resolved by the semantic annotation
 * registry.
 *
 * The grammar does not assign execution semantics to arbitrary annotation
 * names.
 *
 * An annotation is not automatically an expansion directive.
 *
 * Only annotations explicitly registered for expansion-related use may
 * affect expansion planning.
 *
 * ============================================================================
 * 8. SCALABILITY
 * ============================================================================
 *
 * This grammar imposes no finite implementation capacity.
 *
 * It defines no universal maximum for:
 *
 *     macros
 *     annotations
 *     expansion steps
 *     expansion depth
 *     generated AST nodes
 *     generated tokens
 *     source bytes
 *     generated output size
 *     nested invocations
 *
 * Compiler resource budgets belong to configurable compiler policy.
 *
 * Resource exhaustion must produce a diagnostic rather than silently
 * changing the meaning of the program.
 *
 * ============================================================================
 * 9. DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source text
 *     canonical lexer
 *     grammar version
 *     explicitly selected language configuration
 *
 * Expansion must subsequently be deterministic for a declared compilation
 * context.
 *
 * Expansion must not use incidental machine state as semantic input.
 *
 * ============================================================================
 * 10. HYGIENE AND PROVENANCE
 * ============================================================================
 *
 * Expansion must preserve:
 *
 *     macro definition identity
 *     invocation identity
 *     argument provenance
 *     generated-node provenance
 *     source spans
 *     hygiene context
 *
 * Actual binding identities and capture avoidance belong to the semantic
 * hygiene implementation.
 *
 * This grammar does not rename identifiers or implement lexical scope.
 *
 * ============================================================================
 * 11. SECURITY
 * ============================================================================
 *
 * Expansion metadata does not grant permission to:
 *
 *     read files
 *     write files
 *     inspect environment variables
 *     access networks
 *     execute processes
 *     access secrets
 *     access hardware
 *     bypass compiler policy
 *
 * Compile-time computation, if supported, requires a separate explicit
 * capability and effect contract.
 *
 * ============================================================================
 * 12. POCO-REAF
 * ============================================================================
 *
 * Expansion must preserve target-independent program meaning.
 *
 * It must not implicitly select:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     simulator
 *     device
 *     node
 *     topology
 *     backend
 *     routing strategy
 *     scheduler
 *
 * Generated syntax must pass through the same semantic and portability
 * checks as directly authored syntax.
 *
 * ============================================================================
 * 13. CANONICAL GRAMMAR RULES
 * ============================================================================
 */


/*
 * ----------------------------------------------------------------------------
 * 13.1 Canonical expansion annotation
 * ----------------------------------------------------------------------------
 *
 * Alias to the shared annotation rule.
 *
 * This rule does not introduce a second annotation language.
 */

macroExpansionAnnotation
    : annotation
    ;


/*
 * ----------------------------------------------------------------------------
 * 13.2 One or more expansion annotations
 * ----------------------------------------------------------------------------
 *
 * The semantic annotation registry determines which annotations are valid
 * for expansion.
 */

macroExpansionAnnotations
    : macroExpansionAnnotation+
    ;


/*
 * ----------------------------------------------------------------------------
 * 13.3 Optional expansion annotations
 * ----------------------------------------------------------------------------
 */

optionalMacroExpansionAnnotations
    : macroExpansionAnnotation*
    ;


/*
 * ----------------------------------------------------------------------------
 * 13.4 Expansion metadata prefix
 * ----------------------------------------------------------------------------
 *
 * Reusable attachment point for consumers that explicitly support
 * expansion metadata.
 *
 * This rule does not make metadata legal in every declaration or expression.
 * The consuming grammar determines its permitted position.
 */

macroExpansionPrefix
    : macroExpansionAnnotations
    ;


/*
 * ----------------------------------------------------------------------------
 * 13.5 Optional expansion metadata prefix
 * ----------------------------------------------------------------------------
 */

optionalMacroExpansionPrefix
    : optionalMacroExpansionAnnotations
    ;


/*
 * ----------------------------------------------------------------------------
 * 13.6 Macro declaration metadata
 * ----------------------------------------------------------------------------
 *
 * Declaration syntax remains owned by declarations.g4.
 *
 * This is an integration boundary, not a declaration definition.
 */

macroDeclarationExpansionMetadata
    : macroExpansionAnnotations
    ;


/*
 * ----------------------------------------------------------------------------
 * 13.7 Macro invocation metadata
 * ----------------------------------------------------------------------------
 *
 * Invocation syntax remains owned by invocations.g4.
 *
 * This rule describes metadata associated with an invocation.
 *
 * It does not redefine macroInvocation.
 */

macroInvocationExpansionMetadata
    : macroExpansionAnnotations
    ;


/*
 * ----------------------------------------------------------------------------
 * 13.8 Expansion context metadata
 * ----------------------------------------------------------------------------
 *
 * The actual expansion context is a semantic/compiler object.
 *
 * This rule only provides a source-level metadata attachment.
 */

macroExpansionContextMetadata
    : macroExpansionAnnotations
    ;


/*
 * ----------------------------------------------------------------------------
 * 13.9 Expansion provenance metadata
 * ----------------------------------------------------------------------------
 *
 * Provenance is established and maintained by the compiler.
 *
 * Source annotations may contribute metadata, but cannot replace compiler
 * generated provenance records.
 */

macroExpansionProvenanceMetadata
    : macroExpansionAnnotations
    ;


/*
 * ----------------------------------------------------------------------------
 * 13.10 Expansion policy metadata
 * ----------------------------------------------------------------------------
 *
 * This is deliberately annotation-based.
 *
 * Policy names and their legal values are determined by the canonical
 * annotation registry and semantic validation.
 *
 * This grammar does not introduce policy keywords.
 */

macroExpansionPolicyMetadata
    : macroExpansionAnnotations
    ;


/*
 * ----------------------------------------------------------------------------
 * 13.11 Expansion diagnostic metadata
 * ----------------------------------------------------------------------------
 *
 * Diagnostic severity, source mapping, and reporting are owned by the
 * compiler diagnostics subsystem.
 *
 * This rule permits associated source annotations without defining a
 * separate diagnostic language.
 */

macroExpansionDiagnosticMetadata
    : macroExpansionAnnotations
    ;


/*
 * ============================================================================
 * 14. INTEGRATION CONTRACT
 * ============================================================================
 *
 * The canonical parser composition may consume the rules above.
 *
 * Integration requirements:
 *
 * 1. The parser must use the canonical Zamani lexer vocabulary.
 *
 * 2. The parser must import or delegate to this grammar component using
 *    the repository's actual ANTLR composition mechanism.
 *
 * 3. Shared annotation syntax must have exactly one owner.
 *
 * 4. Macro declarations must continue to use declarations.g4.
 *
 * 5. Macro invocations must continue to use invocations.g4.
 *
 * 6. Hygiene must continue to use hygiene.g4 and the semantic hygiene engine.
 *
 * 7. Expansion metadata must be preserved in the frontend AST.
 *
 * 8. AST nodes must preserve original source spans.
 *
 * 9. Generated nodes must retain expansion provenance.
 *
 * 10. Expansion must finish before ordinary semantic validation of the
 *     resulting program is considered complete.
 *
 * 11. Generated syntax must not bypass type, effect, capability, ownership,
 *     resource, security, or domain validation.
 *
 * 12. The compiler must not create a second quantum IR for macro expansion.
 *
 * 13. Quantum constructs generated by macros must enter the canonical
 *     quantum::ir path through normal semantic lowering.
 *
 * 14. HDL and hardware constructs generated by macros must enter their
 *     existing semantic and lowering paths.
 *
 * 15. Macro expansion must not select physical hardware.
 *
 * ============================================================================
 * 15. AST CONTRACT
 * ============================================================================
 *
 * This grammar does not mandate a new AST node for every alias rule.
 *
 * The frontend should represent expansion metadata using the existing
 * macro/annotation representation.
 *
 * Required information:
 *
 *     source span
 *     annotation identity
 *     annotation arguments
 *     attachment owner
 *     macro definition identity, when applicable
 *     invocation identity, when applicable
 *
 * The compiler expansion subsystem must additionally maintain:
 *
 *     expansion identity
 *     parent expansion identity
 *     source provenance
 *     hygiene context
 *     deterministic expansion context
 *
 * Do not create a parallel MacroExpansionAst solely to mirror these
 * grammar aliases.
 *
 * ============================================================================
 * 16. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing does not establish that an annotation is valid.
 *
 * Semantic validation must verify:
 *
 *     annotation registration
 *     annotation placement
 *     annotation argument validity
 *     macro visibility
 *     macro resolution
 *     parameter binding
 *     expansion eligibility
 *     recursion policy
 *     capability authorization
 *     deterministic behavior
 *     resource-budget compliance
 *     provenance preservation
 *     hygiene preservation
 *
 * Unknown or invalid expansion annotations must produce diagnostics.
 *
 * They must not silently acquire meaning.
 *
 * ============================================================================
 * 17. IR CONTRACT
 * ============================================================================
 *
 * This grammar creates no IR.
 *
 * Macro expansion is completed at the frontend transformation stage.
 *
 * The resulting program proceeds through the existing semantic model and
 * canonical IR pipeline.
 *
 * No macro-specific quantum IR is permitted.
 *
 * No macro-specific HDL IR is permitted.
 *
 * ============================================================================
 * 18. RESOURCE POLICY
 * ============================================================================
 *
 * The grammar defines no fixed expansion limits.
 *
 * Implementations may expose configurable resource budgets for:
 *
 *     expansion work
 *     generated syntax size
 *     memory use
 *     compilation time
 *     recursion detection
 *
 * Budgets must:
 *
 *     be explicit;
 *     be configurable;
 *     be deterministic within the declared compilation context;
 *     produce structured diagnostics;
 *     avoid silent truncation;
 *     avoid changing valid program meaning.
 *
 * These budgets are not language-wide hardware limits.
 *
 * ============================================================================
 * 19. COMPATIBILITY
 * ============================================================================
 *
 * Existing macro invocation syntax must remain owned by invocations.g4.
 *
 * Existing declaration syntax must remain owned by declarations.g4.
 *
 * Existing annotation syntax must remain canonical.
 *
 * This component must not introduce reserved keywords.
 *
 * Any future change to expansion syntax requires:
 *
 *     lexical contract
 *     syntax contract
 *     AST contract
 *     semantic contract
 *     security contract
 *     provenance contract
 *     compatibility contract
 *     positive tests
 *     negative tests
 *     boundary tests
 *     scalability tests
 *
 * ============================================================================
 * 20. CONFORMANCE TEST REQUIREMENTS
 * ============================================================================
 *
 * Positive:
 *
 *     valid expansion annotation
 *     multiple expansion annotations
 *     optional expansion metadata
 *     declaration metadata integration
 *     invocation metadata integration
 *
 * Negative:
 *
 *     malformed annotation
 *     invalid annotation placement
 *     invalid annotation arguments
 *     unknown expansion annotation
 *     duplicate conflicting policies
 *
 * Boundary:
 *
 *     empty optional metadata
 *     nested macro invocation
 *     nested annotation arguments
 *     deeply nested source structure
 *     large valid annotation collections
 *
 * Scalability:
 *
 *     increasing macro count
 *     increasing annotation count
 *     large generated syntax
 *     resource-budget exhaustion
 *     configurable expansion budgets
 *
 * Determinism:
 *
 *     repeated parsing
 *     repeated expansion under identical context
 *     stable provenance
 *     stable diagnostics
 *
 * Portability:
 *
 *     classical macro output
 *     quantum macro output
 *     hybrid macro output
 *     HDL macro output
 *     hardware-independent resource declarations
 *     distributed macro output
 *
 * Security:
 *
 *     no implicit filesystem access
 *     no implicit network access
 *     no implicit process execution
 *     no capability bypass
 *     no semantic-validation bypass
 *
 * ============================================================================
 * 21. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [ ] It compiles as an ANTLR parser grammar with the canonical token
 *     vocabulary.
 *
 * [ ] Shared annotation syntax has one authoritative owner.
 *
 * [ ] Macro declarations remain owned by declarations.g4.
 *
 * [ ] Macro invocations remain owned by invocations.g4.
 *
 * [ ] Hygiene remains owned by hygiene.g4 and semantic implementation.
 *
 * [ ] The canonical parser composition consumes this component correctly.
 *
 * [ ] Every permitted attachment point is documented by its consumer.
 *
 * [ ] AST metadata preserves source spans.
 *
 * [ ] Expansion provenance is preserved downstream.
 *
 * [ ] No arbitrary host execution is introduced.
 *
 * [ ] No machine-dependent expansion limit is introduced.
 *
 * [ ] No domain-specific expansion rule is introduced.
 *
 * [ ] No duplicate IR is introduced.
 *
 * [ ] Positive and negative grammar tests pass.
 *
 * [ ] Boundary, scalability, determinism, and compatibility tests pass.
 *
 * [ ] Rust implementation remains compatible with Rust 1.97.1 and Rust 2021.
 *
 * [ ] No unsafe Rust is required.
 *
 * ============================================================================
 */
parser grammar expansion;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * Canonical shared annotation rule.
 *
 * The rule must be supplied by the complete parser composition.
 */

macroExpansionAnnotation
    : annotation
    ;

macroExpansionAnnotations
    : macroExpansionAnnotation+
    ;

optionalMacroExpansionAnnotations
    : macroExpansionAnnotation*
    ;

macroExpansionPrefix
    : macroExpansionAnnotations
    ;

optionalMacroExpansionPrefix
    : optionalMacroExpansionAnnotations
    ;

macroDeclarationExpansionMetadata
    : macroExpansionAnnotations
    ;

macroInvocationExpansionMetadata
    : macroExpansionAnnotations
    ;

macroExpansionContextMetadata
    : macroExpansionAnnotations
    ;

macroExpansionProvenanceMetadata
    : macroExpansionAnnotations
    ;

macroExpansionPolicyMetadata
    : macroExpansionAnnotations
    ;

macroExpansionDiagnosticMetadata
    : macroExpansionAnnotations
    ;

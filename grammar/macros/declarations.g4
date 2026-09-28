/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *     grammar/macros/declarations.g4
 *
 * Grammar:
 *     MacroDeclarations
 *
 * Status:
 *     CANONICAL MACRO-DECLARATION PARSER COMPONENT
 *
 * Purpose:
 *     Own the complete source-level syntax of Zamani macro declarations.
 *
 * Rust implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     Safe Rust only
 *     No unsafe Rust
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical ZamaniLexer
 *          |
 *          v
 *     grammar/antlr/ZamaniParser.g4
 *          |
 *          +-----------------------------+
 *          |                             |
 *          v                             v
 *     ordinary declarations          universalMacro
 *                                        |
 *                                        v
 *                              MacroDeclarations
 *                                        |
 *                         +--------------+--------------+
 *                         |              |              |
 *                         v              v              v
 *                    parameters       defaults         body
 *                         |              |              |
 *                         +--------------+--------------+
 *                                        |
 *                                        v
 *                              frontend source AST
 *                                        |
 *                                        v
 *                               structural validation
 *                                        |
 *                                        v
 *                                macro resolution
 *                                        |
 *                                        v
 *                              controlled expansion
 *                                        |
 *                                        v
 *                              hygiene / provenance
 *                                        |
 *                                        v
 *                                semantic analysis
 *                                        |
 *                                        v
 *                              canonical semantic model
 *                                        |
 *              +-------------------------+-------------------------+
 *              |                         |                         |
 *              v                         v                         v
 *        classical IR               quantum::ir             HDL/hardware
 *              |                         |                         |
 *              +-------------------------+-------------------------+
 *                                        |
 *                                        v
 *                         optimization / lowering / routing
 *                                        |
 *                                        v
 *                             scheduling / resilience
 *                                        |
 *                                        v
 *                                      ZQN
 *                                        |
 *                                        v
 *                                      HAL
 *                                        |
 *                                        v
 *                               target realization
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - macroDeclaration;
 *     - macro parameter-list syntax;
 *     - macro parameter syntax;
 *     - macro parameter default syntax.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer/token definitions;
 *     - identifiers;
 *     - qualified names;
 *     - visibility vocabulary;
 *     - generic parameter semantics;
 *     - type semantics;
 *     - expression semantics;
 *     - block semantics;
 *     - macro invocation;
 *     - macro paths;
 *     - macro expressions;
 *     - macro expansion;
 *     - macro execution;
 *     - macro hygiene implementation;
 *     - source-generation implementation;
 *     - filesystem access;
 *     - network access;
 *     - process execution;
 *     - package loading;
 *     - compiler configuration;
 *     - target selection;
 *     - CPU/GPU/FPGA/ASIC/QPU selection;
 *     - resource allocation;
 *     - topology selection;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution;
 *     - canonical IR construction.
 *
 * ============================================================================
 * SINGLE-AUTHORITY CONTRACT
 * ============================================================================
 *
 * Exactly one canonical grammar component owns macro declarations.
 *
 * This file is that owner.
 *
 * Therefore the following rules MUST NOT be redefined elsewhere:
 *
 *     macroDeclaration
 *     macroParameterList
 *     macroParameter
 *     macroParameterDefault
 *
 * Macro invocation remains owned by:
 *
 *     grammar/macros/invocations.g4
 *
 * In particular, this file MUST NOT define:
 *
 *     macroPath
 *     macroInvocation
 *     macroExpression
 *
 * Macro composition remains owned by:
 *
 *     grammar/macros/macros.g4
 *
 * Expression-side integration remains owned by:
 *
 *     grammar/expressions/macros.g4
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexer owns the MACRO token.
 *
 * This grammar therefore consumes:
 *
 *     MACRO
 *
 * rather than introducing:
 *
 *     'macro'
 *
 * or another duplicate token.
 *
 * Shared lexical vocabulary is supplied by:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/lexer/*
 *
 * This file contains no lexer rules.
 *
 * ============================================================================
 * SHARED PARSER CONTRACT
 * ============================================================================
 *
 * The following rules are canonical rules supplied by the complete Zamani
 * parser composition:
 *
 *     visibilityModifier
 *     identifier
 *     genericParameters
 *     typeExpression
 *     expression
 *     blockExpression
 *
 * This file consumes those rules.
 *
 * It MUST NOT redefine them.
 *
 * The canonical parser composition is responsible for making those shared
 * rules available to this delegate grammar.
 *
 * ============================================================================
 * VISIBILITY CONTRACT
 * ============================================================================
 *
 * Visibility is supplied by the canonical:
 *
 *     visibilityModifier
 *
 * Examples include the repository's established visibility vocabulary:
 *
 *     pub
 *     public
 *     private
 *     protected
 *     internal
 *
 * Whether a particular visibility is semantically legal for a macro is a
 * semantic question.
 *
 * This grammar only establishes its syntactic position.
 *
 * ============================================================================
 * GENERIC CONTRACT
 * ============================================================================
 *
 * Macro declarations reuse the canonical:
 *
 *     genericParameters
 *
 * rule.
 *
 * This prevents macro declarations from creating a second generic-parameter
 * grammar.
 *
 * Generic parameter semantics remain owned by the canonical type/generic
 * subsystem.
 *
 * Generic parameters may therefore eventually participate in:
 *
 *     - classical abstractions;
 *     - quantum abstractions;
 *     - hybrid abstractions;
 *     - HDL abstractions;
 *     - hardware-independent resource abstractions;
 *     - distributed abstractions;
 *     - AI/data abstractions;
 *     - future Zamani domains.
 *
 * This grammar does not classify them by domain.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Macro parameter type annotations reuse:
 *
 *     typeExpression
 *
 * They are source-level type syntax.
 *
 * This grammar does not decide whether a type denotes:
 *
 *     - runtime data;
 *     - compile-time data;
 *     - token-like data;
 *     - source syntax;
 *     - a quantum value;
 *     - a resource;
 *     - a capability;
 *     - a hardware-independent abstraction.
 *
 * Such interpretation belongs to semantic analysis.
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * Macro defaults reuse the canonical:
 *
 *     expression
 *
 * rule.
 *
 * The parser records the expression structure.
 *
 * It MUST NOT:
 *
 *     - evaluate the default;
 *     - execute source code;
 *     - expand another macro;
 *     - resolve names;
 *     - inspect hardware;
 *     - access files;
 *     - access networks;
 *     - invoke processes.
 *
 * ============================================================================
 * BODY CONTRACT
 * ============================================================================
 *
 * Macro bodies reuse:
 *
 *     blockExpression
 *
 * This is intentional.
 *
 * Macro declarations therefore do not create a second body language.
 *
 * A macro body can consequently contain ordinary Zamani source syntax and,
 * after expansion, may participate in any supported computational domain.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Macro declaration syntax is target-independent.
 *
 * A macro declaration MUST NOT inherently select:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     simulator
 *     accelerator
 *     processor
 *     processor count
 *     core count
 *     thread count
 *     device
 *     device count
 *     physical qubit
 *     qubit count
 *     register width
 *     memory bank
 *     memory capacity
 *     network node
 *     topology
 *     gate set
 *     scheduler
 *     routing strategy
 *     backend
 *
 * A macro may generate source that contains legitimate resource or capability
 * requirements, but those requirements remain ordinary semantic program intent.
 *
 * For example, generated source may eventually express:
 *
 *     requires capability("quantum.measurement")
 *
 * or:
 *
 *     requires memory >= required_memory
 *
 * The macro declaration grammar does not interpret those requirements.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar introduces NO language-level finite limit on:
 *
 *     - number of macro declarations;
 *     - number of macro parameters;
 *     - number of generic parameters;
 *     - parameter name length;
 *     - macro body size;
 *     - source size;
 *     - nesting depth;
 *     - generated program size.
 *
 * Repetition is represented by ANTLR repetition constructs.
 *
 * There are deliberately no constants such as:
 *
 *     MAX_MACROS
 *     MAX_MACRO_PARAMETERS
 *     MAX_GENERIC_PARAMETERS
 *     MAX_MACRO_BODY_SIZE
 *     MAX_MACRO_DEPTH
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *
 * or equivalent language-level ceilings.
 *
 * Compiler implementations MAY provide explicit, configurable resource
 * policies for hostile or pathological input, including:
 *
 *     - source size;
 *     - parser resources;
 *     - AST resources;
 *     - macro expansion depth;
 *     - expansion steps;
 *     - generated source size;
 *     - compiler memory;
 *     - compilation time.
 *
 * Those policies are implementation/resource controls and are not encoded here.
 *
 * ============================================================================
 * SAFETY CONTRACT
 * ============================================================================
 *
 * This grammar contains no embedded target-language actions.
 *
 * It therefore:
 *
 *     - executes no Rust;
 *     - performs no I/O;
 *     - performs no filesystem access;
 *     - performs no network access;
 *     - performs no process execution;
 *     - performs no hardware discovery;
 *     - performs no runtime execution;
 *     - performs no macro expansion.
 *
 * The Rust compiler implementation consuming this grammar MUST remain:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *
 * No unsafe Rust is required by this grammar.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing is structural.
 *
 * Given the same:
 *
 *     source
 *     canonical lexer configuration
 *     parser configuration
 *
 * the resulting parse structure must be deterministic.
 *
 * This grammar contains:
 *
 *     - no semantic predicates;
 *     - no random behavior;
 *     - no time dependence;
 *     - no environment dependence;
 *     - no hardware dependence;
 *     - no filesystem dependence;
 *     - no network dependence.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar does not define Rust AST structures.
 *
 * The frontend AST remains the sole source-level AST authority.
 *
 * A macro declaration must preserve enough information for downstream
 * compilation to represent:
 *
 *     - source span;
 *     - declaration identity;
 *     - visibility;
 *     - macro name;
 *     - generic parameters;
 *     - ordered macro parameters;
 *     - parameter names;
 *     - optional parameter types;
 *     - optional defaults;
 *     - macro body;
 *     - source provenance.
 *
 * This file MUST NOT introduce a second macro AST hierarchy.
 *
 * In particular, this grammar must never require AST fields such as:
 *
 *     physical_qubits
 *     backend
 *     topology
 *     gate_set
 *     scheduler
 *     qec
 *     zqn
 *     device
 *     gpu
 *     cpu_count
 *
 * Those belong downstream, if they are needed at all.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes syntax only.
 *
 * Later semantic phases determine:
 *
 *     - whether the macro name is unique;
 *     - whether the macro is visible;
 *     - whether its generic parameters are valid;
 *     - whether parameter names are unique;
 *     - whether defaults are legal;
 *     - whether required parameters follow language policy;
 *     - whether defaults satisfy parameter types;
 *     - whether the body is semantically valid;
 *     - whether the macro may expand in the current context;
 *     - whether compile-time capabilities are available;
 *     - whether expansion is deterministic;
 *     - whether expansion satisfies configured resource policy.
 *
 * None of these semantic checks are embedded in this grammar.
 *
 * ============================================================================
 * EXPANSION CONTRACT
 * ============================================================================
 *
 * This grammar does NOT expand macros.
 *
 * Expansion belongs to the compiler macro subsystem.
 *
 * Expansion must preserve:
 *
 *     - declaration provenance;
 *     - invocation provenance;
 *     - generated-source provenance;
 *     - hygiene;
 *     - deterministic ordering;
 *     - diagnostics.
 *
 * A macro may expand into any valid Zamani source construct.
 *
 * The declaration grammar therefore remains independent of the generated
 * domain.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A macro body may eventually generate quantum source.
 *
 * This grammar does not:
 *
 *     - enumerate quantum gates;
 *     - allocate qubits;
 *     - select physical qubits;
 *     - select a QPU;
 *     - select topology;
 *     - perform routing;
 *     - perform scheduling;
 *     - perform QEC;
 *     - implement ZQN;
 *     - select calibration data.
 *
 * After macro expansion and semantic validation, quantum constructs continue
 * through the canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * No macro-specific quantum IR is introduced here.
 *
 * ============================================================================
 * CLASSICAL / HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * A macro may generate:
 *
 *     - classical computation;
 *     - quantum computation;
 *     - hybrid computation;
 *     - HDL;
 *     - hardware intent;
 *     - distributed computation;
 *     - AI/data computation;
 *     - networking;
 *     - security;
 *     - future Zamani domains.
 *
 * This grammar does not need separate declaration rules such as:
 *
 *     quantumMacroDeclaration
 *     gpuMacroDeclaration
 *     fpgaMacroDeclaration
 *     qpuMacroDeclaration
 *     hdlMacroDeclaration
 *
 * Ordinary macros are intentionally domain-neutral.
 *
 * ============================================================================
 * ERROR BOUNDARY
 * ============================================================================
 *
 * Syntax errors belong to the parser.
 *
 * Examples of malformed source include:
 *
 *     macro { }
 *     macro name( { }
 *     macro name(a: ) { }
 *     macro name(a = ) { }
 *     macro name(a,,b) { }
 *     macro name(a { }
 *     macro name() 
 *
 * when the final construct cannot satisfy the grammar.
 *
 * These are NOT syntax errors owned by this file:
 *
 *     unknown macro;
 *     duplicate macro;
 *     inaccessible macro;
 *     invalid generic constraint;
 *     invalid default semantics;
 *     recursive expansion;
 *     expansion budget exhaustion;
 *     generated-resource exhaustion;
 *     unavailable capability;
 *     unavailable hardware;
 *     unsupported target.
 *
 * Those belong to later compiler phases.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing valid macro declaration forms are preserved:
 *
 *     macro name() { }
 *
 *     macro name(value) { }
 *
 *     macro name(value: Type) { }
 *
 *     macro name(value = defaultValue) { }
 *
 *     macro name(value: Type = defaultValue) { }
 *
 *     macro name<T>(value: T) { }
 *
 *     pub macro name<T>(value: T) { }
 *
 * A trailing comma is accepted in macro parameter lists:
 *
 *     macro name(a, b,) { }
 *
 * The trailing-comma policy is local to macro declarations.
 *
 * It does not modify the canonical runtime/function parameter grammar.
 *
 * ============================================================================
 * INTEGRATION WITH macros.g4
 * ============================================================================
 *
 * `grammar/macros/macros.g4` is the macro composition grammar.
 *
 * It MUST import this grammar by its grammar identity:
 *
 *     MacroDeclarations
 *
 * not:
 *
 *     declarations
 *
 * because ANTLR grammar imports use grammar names.
 *
 * The resulting composition is:
 *
 *     Macros
 *        |
 *        +--> MacroDeclarations
 *        +--> invocations
 *        +--> expansion
 *        +--> hygiene
 *
 * ============================================================================
 * INTEGRATION WITH ZamaniParser.g4
 * ============================================================================
 *
 * `grammar/antlr/ZamaniParser.g4` already imports:
 *
 *     Macros
 *
 * and already exposes:
 *
 *     universalMacro
 *         : macroDeclaration
 *         ;
 *
 * The complete source-unit dispatcher must make `universalMacro` reachable
 * from `sourceElement`.
 *
 * This file itself does not modify sourceElement because source-unit ownership
 * belongs to ZamaniParser.g4.
 *
 * ============================================================================
 * INTEGRATION WITH DECLARATIONS
 * ============================================================================
 *
 * Macro declarations are intentionally not added to the ordinary
 * `declaration` dispatcher in:
 *
 *     grammar/declarations/declarations.g4
 *
 * This avoids creating two independent paths to the same macro declaration.
 *
 * The canonical top-level path is:
 *
 *     sourceElement
 *         |
 *         +--> universalMacro
 *                   |
 *                   v
 *             macroDeclaration
 *
 * Ordinary declarations continue through:
 *
 *     declarationElement
 *         |
 *         v
 *     declaration
 *
 * ============================================================================
 * INTEGRATION WITH EXPRESSION MACROS
 * ============================================================================
 *
 * This file has no dependency on:
 *
 *     macroExpression
 *     macroInvocation
 *     macroPath
 *
 * Expression-side macro invocation remains owned by:
 *
 *     grammar/macros/invocations.g4
 *
 * and integrated into the canonical expression hierarchy through:
 *
 *     grammar/expressions/macros.g4
 *
 * ============================================================================
 * INTEGRATION WITH FRONTEND AST
 * ============================================================================
 *
 * The repository already has a canonical frontend representation for macro
 * invocation expressions:
 *
 *     src/frontend/ast/node/expressions/macro.rs
 *
 * That representation is intentionally separate from macro declaration
 * syntax.
 *
 * Macro declarations must therefore be represented by the declaration-side
 * AST subsystem rather than by reusing the invocation expression node.
 *
 * This grammar establishes the stable source contract required by that AST:
 *
 *     visibility
 *     name
 *     generics
 *     parameters
 *     defaults
 *     body
 *
 * Adding the declaration AST implementation MUST NOT require changing this
 * grammar unless the language syntax itself changes.
 *
 * ============================================================================
 * INTEGRATION WITH MACRO ENGINE
 * ============================================================================
 *
 * The existing compiler macro engine is downstream from parsing.
 *
 * It is responsible for:
 *
 *     - registration;
 *     - resolution;
 *     - argument binding;
 *     - controlled expansion;
 *     - recursion detection;
 *     - configurable expansion/resource policy;
 *     - generated-source validation.
 *
 * This grammar does not encode those policies.
 *
 * In particular, expansion limits such as:
 *
 *     max_expansion_size
 *     max_expansion_depth
 *
 * belong to compiler configuration, not language syntax.
 *
 * ============================================================================
 * NO HARD-CODED HARDWARE
 * ============================================================================
 *
 * This file deliberately contains none of:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * Macro declarations therefore scale from tiny source programs to arbitrarily
 * large source programs subject to available resources and explicit compiler
 * policies.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive:
 *
 *     macro empty() { }
 *
 *     macro one(value) { }
 *
 *     macro many(a, b, c) { }
 *
 *     macro trailing(a, b,) { }
 *
 *     macro typed(value: T) { }
 *
 *     macro defaulted(value = defaultValue) { }
 *
 *     macro typedDefaulted(value: T = defaultValue) { }
 *
 *     macro generic<T>(value: T) { }
 *
 *     macro genericMany<T, U>(a: T, b: U) { }
 *
 *     pub macro exported<T>(value: T) { }
 *
 *     public macro exported<T>(value: T) { }
 *
 *     private macro internal<T>(value: T) { }
 *
 * Cross-domain body fixtures must verify that the declaration grammar can
 * contain bodies which eventually produce:
 *
 *     classical;
 *     quantum;
 *     hybrid;
 *     HDL;
 *     hardware;
 *     distributed;
 *     AI;
 *     data;
 *     networking;
 *     security;
 *     future domain constructs.
 *
 * Negative:
 *
 *     macro { }
 *
 *     macro name( { }
 *
 *     macro name(a,,b) { }
 *
 *     macro name(a: ) { }
 *
 *     macro name(a = ) { }
 *
 *     macro name(a: T = ) { }
 *
 *     macro name(a { }
 *
 * Boundary/scalability:
 *
 *     - large parameter lists;
 *     - large generic parameter lists;
 *     - large macro bodies;
 *     - deeply nested canonical block structures;
 *     - repeated macro declarations;
 *     - generated programs containing arbitrarily many macro declarations.
 *
 * No test may establish an artificial language-level maximum.
 *
 * Determinism:
 *
 *     parsing identical source with identical lexer/parser configuration must
 *     produce structurally equivalent parse trees.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] It has exactly one macroDeclaration owner.
 *     [x] It has exactly one macroParameterList owner.
 *     [x] It has exactly one macroParameter owner.
 *     [x] It has exactly one macroParameterDefault owner.
 *     [x] It uses the canonical MACRO token.
 *     [x] It uses canonical visibilityModifier syntax.
 *     [x] It reuses canonical identifier syntax.
 *     [x] It reuses canonical generic parameter syntax.
 *     [x] It reuses canonical type syntax.
 *     [x] It reuses canonical expression syntax.
 *     [x] It reuses canonical block syntax.
 *     [x] It defines no macro invocation rules.
 *     [x] It defines no macro path rules.
 *     [x] It defines no macro-expression rules.
 *     [x] It defines no lexer rules.
 *     [x] It defines no semantic actions.
 *     [x] It defines no embedded Rust.
 *     [x] It requires no unsafe Rust.
 *     [x] It contains no hardware limits.
 *     [x] It contains no resource ceilings.
 *     [x] It contains no backend selection.
 *     [x] It contains no QEC/ZQN/routing/scheduling behavior.
 *     [x] It preserves the canonical quantum::ir boundary downstream.
 *     [x] It remains domain-neutral.
 *     [x] It is compatible with POCO-REAF.
 *
 * Integration acceptance additionally requires:
 *
 *     - macros.g4 imports MacroDeclarations;
 *     - ZamaniParser.g4 makes universalMacro reachable from sourceElement;
 *     - no competing macroDeclaration rule remains;
 *     - the frontend AST provides the declaration-side representation;
 *     - semantic analysis consumes the declaration representation;
 *     - macro expansion remains a downstream compiler phase;
 *     - positive tests pass;
 *     - negative tests pass;
 *     - scalability tests pass;
 *     - deterministic parsing tests pass;
 *     - Rust 1.97 / 1.97.1 builds remain safe and free of unsafe Rust.
 *
 * ============================================================================
 */

parser grammar MacroDeclarations;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * MACRO DECLARATION
 * ============================================================================
 *
 * Canonical form:
 *
 *     macro name() {
 *         ...
 *     }
 *
 * Generic form:
 *
 *     macro name<T>(value: T) {
 *         ...
 *     }
 *
 * Visibility is optional and delegated to the canonical visibility rule.
 *
 * Generic parameters are delegated to the canonical generic-parameter rule.
 *
 * The body is delegated to the canonical block-expression rule.
 *
 * No expansion behavior is encoded here.
 */
macroDeclaration
    : visibilityModifier?
      MACRO
      identifier
      genericParameters?
      LPAREN
      macroParameterList?
      RPAREN
      blockExpression
    ;


/*
 * ============================================================================
 * MACRO PARAMETER LIST
 * ============================================================================
 *
 * Empty parameter lists are represented by omission:
 *
 *     macro name() { }
 *
 * Non-empty parameter lists are ordered.
 *
 * A trailing comma is explicitly accepted:
 *
 *     macro name(a, b,) { }
 *
 * There is no grammar-level parameter-count limit.
 */
macroParameterList
    : macroParameter
      (COMMA macroParameter)*
      COMMA?
    ;


/*
 * ============================================================================
 * MACRO PARAMETER
 * ============================================================================
 *
 * Supported forms:
 *
 *     name
 *     name: Type
 *     name = expression
 *     name: Type = expression
 *
 * The parameter name is canonical Zamani identifier syntax.
 *
 * The optional type uses canonical type syntax.
 *
 * The optional default uses canonical expression syntax.
 *
 * Semantic restrictions are intentionally deferred.
 */
macroParameter
    : identifier
      (COLON typeExpression)?
      macroParameterDefault?
    ;


/*
 * ============================================================================
 * MACRO PARAMETER DEFAULT
 * ============================================================================
 *
 * Defaults are ordinary Zamani expressions.
 *
 * No evaluation occurs in the parser.
 */
macroParameterDefault
    : ASSIGN expression
    ;
/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/metaprogramming/code-generation.g4
 *
 * Grammar:
 *     MetaprogrammingCodeGeneration
 *
 * Status:
 *     Production parser-composition unit
 *
 * Implementation target:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     Safe Rust only.
 *     No `unsafe` implementation is required or permitted by the compiler
 *     integration contract.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines the METAPROGRAMMING-SIDE CODE-GENERATION COMPOSITION
 * BOUNDARY.
 *
 * It connects:
 *
 *     metaprogramming intent
 *             |
 *             v
 *     source-level generation
 *             |
 *             v
 *     canonical Zamani source structure
 *             |
 *             v
 *     canonical AST
 *             |
 *             v
 *     semantic analysis
 *
 * It does NOT own backend code emission.
 *
 * Backend / compilation-level code generation remains owned by:
 *
 *     grammar/compile/code-generation.g4
 *
 * This distinction is intentional.
 *
 * There are therefore two different concepts:
 *
 *     1. METAPROGRAMMING CODE GENERATION
 *
 *        Producing Zamani source structure or canonical language structures
 *        during the metaprogramming phase.
 *
 *     2. COMPILER CODE GENERATION
 *
 *        Lowering already-validated canonical semantic representation into
 *        target artifacts.
 *
 * The first is owned by this file and generation.g4.
 *
 * The second is owned by compile/code-generation.g4 and downstream compiler
 * infrastructure.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
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
 *     frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> metaprogramming analysis
 *          |       |
 *          |       +--> quotation
 *          |       +--> unquotation
 *          |       +--> macro expansion
 *          |       +--> compile-time execution
 *          |       +--> reflection
 *          |       +--> specialization
 *          |       +--> source generation
 *          |
 *          v
 *     generated canonical source structure
 *          |
 *          v
 *     canonical AST / semantic model
 *          |
 *          v
 *     canonical semantic IR
 *          |
 *          +--> classical representation
 *          +--> quantum::ir
 *          +--> HDL / hardware representation
 *          +--> distributed representation
 *          +--> AI/data representation
 *          +--> future domain representations
 *          |
 *          v
 *     optimization
 *          |
 *          +--> routing
 *          +--> scheduling
 *          +--> resilience
 *          +--> QEC where applicable
 *          +--> ZQN where applicable
 *          |
 *          v
 *     HAL / target lowering
 *          |
 *          v
 *     compiler code generation
 *          |
 *          v
 *     artifact / execution target
 *
 * This file MUST NOT bypass this pipeline.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - the metaprogramming code-generation integration boundary;
 *   - explicit code-generation intent at the metaprogramming phase;
 *   - generation-context wrappers;
 *   - generated-source category selection;
 *   - generated-source composition;
 *   - generation result classification;
 *   - generation provenance intent;
 *   - generation visibility intent;
 *   - generation validation intent;
 *   - generation policy metadata;
 *   - composition with the existing generation.g4 facility;
 *   - composition with quotation/unquotation;
 *   - composition with macro expansion;
 *   - composition with compile-time execution;
 *   - composition with reflection;
 *   - composition with specialization.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexical token definitions;
 *   - keyword definitions;
 *   - identifier syntax;
 *   - expression precedence;
 *   - ordinary expression syntax;
 *   - type syntax;
 *   - statement syntax;
 *   - declaration syntax;
 *   - macro implementation;
 *   - macro hygiene;
 *   - quotation core syntax;
 *   - unquotation core syntax;
 *   - compile-time execution semantics;
 *   - reflection semantics;
 *   - specialization algorithms;
 *   - backend code emission;
 *   - object-file generation;
 *   - executable generation;
 *   - machine instructions;
 *   - ABI implementation;
 *   - linker implementation;
 *   - assembler implementation;
 *   - CPU selection;
 *   - GPU selection;
 *   - FPGA selection;
 *   - ASIC selection;
 *   - QPU selection;
 *   - physical-qubit allocation;
 *   - resource allocation;
 *   - routing;
 *   - scheduling;
 *   - QEC;
 *   - ZQN;
 *   - HAL;
 *   - runtime execution.
 *
 * ============================================================================
 * EXISTING REPOSITORY OWNERSHIP
 * ============================================================================
 *
 * Existing files inspected for this contract include:
 *
 *     grammar/metaprogramming/metaprogramming.g4
 *     grammar/metaprogramming/generation.g4
 *     grammar/metaprogramming/quotation.g4
 *     grammar/metaprogramming/unquotation.g4
 *     grammar/metaprogramming/compile-time-execution.g4
 *     grammar/metaprogramming/reflection.g4
 *     grammar/metaprogramming/introspection.g4
 *     grammar/metaprogramming/specialization.g4
 *     grammar/macros/macros.g4
 *     grammar/macros/invocations.g4
 *     grammar/macros/expansion.g4
 *     grammar/macros/hygiene.g4
 *     grammar/macros/syntax-tree.g4
 *     grammar/macros/token-stream.g4
 *     grammar/compile/code-generation.g4
 *     grammar/DESIGN.md
 *
 * The existing architecture establishes:
 *
 *     generation.g4
 *         source-level generation
 *
 *     compile/code-generation.g4
 *         compiler/backend code-generation intent
 *
 * This file MUST NOT collapse those two concerns into one grammar.
 *
 * ============================================================================
 * CANONICAL LEXER CONTRACT
 * ============================================================================
 *
 * This grammar consumes the repository's canonical lexer vocabulary:
 *
 *     tokenVocab = ZamaniLexer
 *
 * No lexer rules are defined here.
 *
 * This file MUST NOT invent tokens for:
 *
 *     GENERATE
 *     CODE_GENERATION
 *     GENERATE_EXPRESSION
 *     GENERATE_TYPE
 *     GENERATE_DECLARATION
 *     TARGET_CODE
 *     MACHINE_CODE
 *     QUANTUM_CODE
 *     GPU_CODE
 *     FPGA_CODE
 *     CPU_CODE
 *
 * Existing lexical syntax is reused.
 *
 * In particular, the existing generation grammar uses:
 *
 *     SYNTHESIZE
 *
 * as the source-generation boundary.
 *
 * This file therefore composes the existing `generation` rule instead of
 * introducing another generation keyword.
 *
 * ============================================================================
 * NO DUPLICATE LANGUAGE
 * ============================================================================
 *
 * This file MUST NOT redefine:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     typeExpression
 *     statement
 *     item
 *     block
 *     attribute
 *     pattern
 *     genericParameters
 *     parameterList
 *     argumentList
 *
 * Those belong to the canonical parser composition layer.
 *
 * Likewise, this file MUST NOT redefine the complete:
 *
 *     generation
 *
 * rule.
 *
 * `generation.g4` already owns that source-generation syntax.
 *
 * ============================================================================
 * SOURCE GENERATION VERSUS COMPILER CODE GENERATION
 * ============================================================================
 *
 * This distinction is mandatory.
 *
 * METAPROGRAMMING:
 *
 *     synthesize ...
 *
 * means:
 *
 *     produce language structure.
 *
 * COMPILER:
 *
 *     codeGenerationDeclaration
 *
 * in:
 *
 *     grammar/compile/code-generation.g4
 *
 * means:
 *
 *     request or control production of downstream compilation artifacts.
 *
 * Therefore this file MUST NOT duplicate rules such as:
 *
 *     codeGenerationArtifact
 *     codeGenerationOutput
 *     codeGenerationEntryPoint
 *     codeGenerationLinkage
 *     codeGenerationInterface
 *     codeGenerationLayout
 *
 * from compile/code-generation.g4.
 *
 * If metaprogramming needs those concepts, they must cross the semantic
 * boundary rather than importing backend implementation syntax into the
 * metaprogramming grammar.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * The language goal remains:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Metaprogramming code generation must therefore produce portable language
 * meaning rather than binding generated source to one physical machine.
 *
 * Generated source may describe:
 *
 *     classical computation;
 *     quantum computation;
 *     hybrid computation;
 *     HDL;
 *     hardware intent;
 *     distributed computation;
 *     AI;
 *     tensor/data computation;
 *     networking;
 *     security;
 *     future domains.
 *
 * The generated source is subsequently interpreted according to the normal
 * semantic rules of that domain.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar contains no artificial language-level capacity limits.
 *
 * In particular, it MUST NOT define:
 *
 *     MAX_GENERATED_ITEMS
 *     MAX_GENERATED_EXPRESSIONS
 *     MAX_GENERATION_DEPTH
 *     MAX_GENERATION_RESULTS
 *     MAX_GENERATORS
 *     MAX_TEMPLATES
 *     MAX_PARAMETERS
 *     MAX_QUOTATIONS
 *     MAX_UNQUOTATIONS
 *     MAX_MACROS
 *     MAX_SPECIALIZATIONS
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_DEVICES
 *
 * Recursive/repeated grammar structure represents arbitrary cardinality.
 *
 * Compiler resource limits are operational policy.
 *
 * They may include:
 *
 *     memory budgets;
 *     evaluation budgets;
 *     cancellation;
 *     timeout policy;
 *     expansion budgets;
 *     generated-source budgets;
 *     diagnostic budgets;
 *     recursion protection.
 *
 * Such implementation limits MUST NOT become Zamani syntax limits.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing this grammar performs no code generation.
 *
 * The parser MUST NOT:
 *
 *     execute generated source;
 *     execute compile-time functions;
 *     access files;
 *     access networks;
 *     access credentials;
 *     inspect arbitrary host memory;
 *     inspect hardware;
 *     access QPUs;
 *     access GPUs;
 *     access FPGAs;
 *     spawn processes;
 *     mutate compiler state.
 *
 * Any such capability, if explicitly supported by the language, must be
 * authorized by semantic capability/effect analysis.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing is deterministic.
 *
 * Generation determinism is a semantic property.
 *
 * A deterministic generator should produce equivalent canonical source
 * structures for equivalent semantic inputs.
 *
 * If a generator depends on:
 *
 *     time;
 *     randomness;
 *     environment;
 *     filesystem;
 *     network;
 *     target capabilities;
 *     resource availability;
 *
 * that dependency must be visible to the semantic/effect/capability system.
 *
 * Merely using this grammar MUST NOT grant those capabilities.
 *
 * ============================================================================
 * PROVENANCE
 * ============================================================================
 *
 * Generated structures must remain traceable to their source.
 *
 * At minimum, the semantic layer must be able to associate generated
 * structures with:
 *
 *     generating source span;
 *     generation site;
 *     generator identity;
 *     generator version where applicable;
 *     source inputs;
 *     transformation phase;
 *     parent generated structure.
 *
 * This grammar does not implement provenance storage.
 *
 * It only provides an explicit generation provenance boundary.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend must lower this parser representation into the canonical AST.
 *
 * The AST representation must preserve:
 *
 *     source span;
 *     generation site;
 *     generation category;
 *     generation operand;
 *     generation options;
 *     explicit attributes;
 *     provenance;
 *     source ordering where relevant;
 *     phase information where required.
 *
 * This file MUST NOT define Rust AST types.
 *
 * It MUST NOT create a metaprogramming-only AST that competes with:
 *
 *     src/frontend/ast/
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     whether generation is legal in the current phase;
 *     whether the generator is callable;
 *     whether its inputs are valid;
 *     whether generated structure is valid;
 *     whether required capabilities exist;
 *     whether effects are permitted;
 *     whether generated names are hygienic;
 *     whether generated source is deterministic where required;
 *     whether provenance is complete;
 *     whether generated source is recursively validated.
 *
 * Syntax alone does not establish any of these properties.
 *
 * ============================================================================
 * GENERATED SOURCE VALIDATION
 * ============================================================================
 *
 * Generated source MUST re-enter the normal language pipeline.
 *
 * Conceptually:
 *
 *     generator
 *        |
 *        v
 *     generated source structure
 *        |
 *        v
 *     canonical AST
 *        |
 *        v
 *     name resolution
 *        |
 *        v
 *     type checking
 *        |
 *        v
 *     effect checking
 *        |
 *        v
 *     capability checking
 *        |
 *        v
 *     resource validation
 *        |
 *        v
 *     semantic model
 *        |
 *        v
 *     canonical IR
 *
 * Generated code MUST NOT bypass validation merely because it was produced by
 * the compiler itself.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Code generation may generate quantum source structure.
 *
 * It MUST NOT generate or directly construct:
 *
 *     quantum::ir;
 *     physical qubits;
 *     physical topology;
 *     QPU identifiers;
 *     routing plans;
 *     schedules;
 *     QEC operations;
 *     ZQN operations.
 *
 * Example generated source:
 *
 *     apply custom_gate to q;
 *
 * remains ordinary Zamani quantum source.
 *
 * It subsequently follows:
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
 *     optimization
 *          |
 *          v
 *     routing
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     QEC / ZQN where applicable
 *          |
 *          v
 *     HAL
 *
 * There is no metaprogramming quantum IR.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Generated HDL or hardware-intent source follows the normal HDL/hardware
 * pipeline.
 *
 * This grammar does not introduce:
 *
 *     fixed register widths;
 *     fixed memory sizes;
 *     physical addresses;
 *     fixed FPGA resources;
 *     ASIC cell identifiers;
 *     GPU identifiers;
 *     CPU identifiers;
 *     QPU identifiers.
 *
 * Hardware intent remains parameterized and target-independent.
 *
 * ============================================================================
 * CLASSICAL / AI / DATA / DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * No domain-specific generation syntax is required here.
 *
 * Generation produces canonical Zamani structures.
 *
 * Those structures are interpreted by:
 *
 *     classical/
 *     quantum/
 *     hybrid/
 *     hdl/
 *     hardware/
 *     distributed/
 *     ai/
 *     data/
 *     networking/
 *     security/
 *
 * and future domains through their normal semantic contracts.
 *
 * ============================================================================
 * MACRO INTEGRATION
 * ============================================================================
 *
 * Macro declaration, invocation, expansion, and hygiene remain owned by:
 *
 *     grammar/macros/
 *
 * This file may compose with macro-generated structures, but MUST NOT redefine
 * macro syntax.
 *
 * Macro expansion must preserve source provenance and hygiene.
 *
 * Generated code resulting from a macro must still enter normal semantic
 * validation.
 *
 * ============================================================================
 * QUOTATION / UNQUOTATION INTEGRATION
 * ============================================================================
 *
 * Quotation remains owned by:
 *
 *     grammar/metaprogramming/quotation.g4
 *
 * Unquotation remains owned by:
 *
 *     grammar/metaprogramming/unquotation.g4
 *
 * This file provides composition boundaries around generated source without
 * recreating:
 *
 *     quote
 *     unquote
 *     splice
 *
 * syntax.
 *
 * ============================================================================
 * COMPILE-TIME EXECUTION INTEGRATION
 * ============================================================================
 *
 * Compile-time execution remains owned by:
 *
 *     grammar/metaprogramming/compile-time-execution.g4
 *
 * This file does not execute anything.
 *
 * The relationship is:
 *
 *     compile-time computation
 *             |
 *             v
 *     generation request
 *             |
 *             v
 *     generated source structure
 *
 * not:
 *
 *     parser
 *       |
 *       v
 *     execute code
 *
 * ============================================================================
 * REFLECTION INTEGRATION
 * ============================================================================
 *
 * Reflection may inspect canonical program structures.
 *
 * Reflection remains owned by:
 *
 *     grammar/metaprogramming/reflection.g4
 *
 * Generated structures may be based on reflection results.
 *
 * Reflection does not become a second generation grammar.
 *
 * ============================================================================
 * SPECIALIZATION INTEGRATION
 * ============================================================================
 *
 * Specialization remains owned by:
 *
 *     grammar/metaprogramming/specialization.g4
 *
 * Specialization may consume generated structures.
 *
 * This file does not choose specialization strategies.
 *
 * ============================================================================
 * INTROSPECTION INTEGRATION
 * ============================================================================
 *
 * Introspection may expose semantic information about:
 *
 *     capabilities;
 *     resources;
 *     target properties;
 *     deployment;
 *     execution context.
 *
 * It does not change this grammar's target independence.
 *
 * Any target-dependent information must remain explicitly classified by the
 * semantic capability/resource system.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Generated source may contain ordinary Zamani resource or capability
 * requirements.
 *
 * Example semantic intent:
 *
 *     requires qubits >= n
 *
 * or:
 *
 *     requires capability("quantum.measurement")
 *
 * or:
 *
 *     requires capability("tensor.compute")
 *
 * This file does not define resource semantics.
 *
 * The generated source re-enters the canonical resource/capability pipeline.
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * Generation itself does not implicitly grant effects.
 *
 * Effects remain owned by:
 *
 *     grammar/effects/
 *
 * If a generator requires an effect, semantic analysis must verify it.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * This file MUST NOT encode:
 *
 *     cpu::x86
 *     gpu::device0
 *     fpga::board0
 *     qpu::device0
 *     physical_qubit(0)
 *     memory_bank(0)
 *     core(0)
 *
 * merely as generation semantics.
 *
 * Target-specific source may exist in explicitly target-specific dialects or
 * downstream compilation layers, but that is not the portable metaprogramming
 * contract.
 *
 * ============================================================================
 * PUBLIC RULES
 * ============================================================================
 *
 * Public integration rules owned by this file:
 *
 *     codeGeneration
 *     codeGenerationDeclaration
 *     codeGenerationExpression
 *     codeGenerationStatement
 *     codeGenerationItem
 *     codeGenerationProgram
 *     codeGenerationContext
 *     codeGenerationResult
 *
 * These are wrappers around canonical source-generation facilities.
 *
 * ============================================================================
 * 1. CODE-GENERATION DECLARATION
 * ============================================================================
 *
 * This rule is the declaration-level integration point.
 *
 * It deliberately delegates actual source generation to the existing
 * `generation` rule.
 *
 * Optional metadata is attached through generation-context syntax.
 *
 * Example conceptual form:
 *
 *     synthesize ...
 *
 *     synthesize ... with ...
 *
 * The exact keyword surface for `with` is not introduced here.
 *
 * Therefore context clauses use existing identifiers and delimiters rather
 * than inventing another keyword.
 */
codeGenerationDeclaration
    : generation codeGenerationContext?
    ;


/*
 * ============================================================================
 * 2. GENERAL CODE-GENERATION ENTRY
 * ============================================================================
 *
 * This is the neutral composition rule used by the canonical metaprogramming
 * parser when the syntactic position is not yet known to be declaration,
 * expression, statement, or item.
 *
 * The generated structure remains owned by generation.g4.
 */
codeGeneration
    : generation
    ;


/*
 * ============================================================================
 * 3. EXPRESSION CONTEXT
 * ============================================================================
 *
 * This wrapper allows the canonical parser to identify that a generation
 * construct occurs in expression position.
 *
 * The actual generation syntax remains owned by generation.g4.
 */
codeGenerationExpression
    : generation
    ;


/*
 * ============================================================================
 * 4. STATEMENT CONTEXT
 * ============================================================================
 *
 * The surrounding parser decides whether the generation construct is legal
 * as a statement.
 *
 * This file does not duplicate statement syntax.
 */
codeGenerationStatement
    : generation
    ;


/*
 * ============================================================================
 * 5. ITEM / DECLARATION CONTEXT
 * ============================================================================
 *
 * Generated declarations/items are produced by the existing generation
 * facility.
 *
 * This wrapper exists solely for parser composition and AST classification.
 */
codeGenerationItem
    : generation
    ;


/*
 * ============================================================================
 * 6. PROGRAM-FRAGMENT CONTEXT
 * ============================================================================
 *
 * A generated program fragment is still canonical generation syntax.
 *
 * No separate program grammar is introduced here.
 */
codeGenerationProgram
    : generation
    ;


/*
 * ============================================================================
 * 7. GENERATION CONTEXT
 * ============================================================================
 *
 * Generation metadata is optional.
 *
 * The context grammar intentionally remains generic and semantic-data-driven.
 *
 * This avoids making every future generation policy a new lexer keyword.
 *
 * Examples of semantic property names may include:
 *
 *     provenance
 *     visibility
 *     deterministic
 *     validate
 *     hygiene
 *     phase
 *     capabilities
 *     effects
 *     domain
 *
 * These names are NOT reserved by this grammar.
 *
 * Their legality is determined by semantic analysis and the relevant
 * specification.
 *
 * Because this grammar must not invent a second keyword system, the context
 * is introduced by the existing `WITH` token only if that token is already
 * part of the canonical lexer.
 *
 * The current repository architecture uses identifier-driven extension points
 * where appropriate. Therefore this grammar uses an ordinary identifier as
 * the context introducer.
 */
codeGenerationContext
    : identifier
      codeGenerationContextBody
    ;


/*
 * ============================================================================
 * 8. CONTEXT BODY
 * ============================================================================
 *
 * The body is deliberately structured rather than an arbitrary token sink.
 *
 * This preserves parse-tree structure for AST conversion and diagnostics.
 */
codeGenerationContextBody
    : LPAREN codeGenerationContextEntries? RPAREN
    | LBRACE codeGenerationContextEntries? RBRACE
    ;


/*
 * ============================================================================
 * 9. CONTEXT ENTRIES
 * ============================================================================
 *
 * No finite number of context entries is imposed.
 */
codeGenerationContextEntries
    : codeGenerationContextEntry
      (COMMA codeGenerationContextEntry)*
    ;


/*
 * ============================================================================
 * 10. CONTEXT ENTRY
 * ============================================================================
 *
 * Context entries are semantic metadata.
 *
 * The grammar permits either:
 *
 *     name = expression
 *
 * or:
 *
 *     name
 *
 * or:
 *
 *     name(expression)
 *
 * without enumerating future generation policies.
 */
codeGenerationContextEntry
    : identifier
      codeGenerationContextValue?
    ;


/*
 * ============================================================================
 * 11. CONTEXT VALUE
 * ============================================================================
 *
 * The canonical expression grammar remains authoritative.
 */
codeGenerationContextValue
    : ASSIGN expression
    | LPAREN expression RPAREN
    ;


/*
 * ============================================================================
 * 12. RESULT CLASSIFICATION
 * ============================================================================
 *
 * This rule does not create a new type system.
 *
 * It gives the semantic layer a stable parser boundary for describing the
 * expected class of generated source.
 *
 * The actual category remains semantic data.
 *
 * Example conceptual categories:
 *
 *     expression
 *     type
 *     statement
 *     item
 *     program
 *     module
 *     quantum
 *     hdl
 *     hardware
 *
 * These are not finite grammar enums.
 *
 * The generated structure itself remains validated by the canonical grammar
 * and semantic system.
 */
codeGenerationResult
    : identifier
    ;


/*
 * ============================================================================
 * 13. GENERATION TARGET CATEGORY
 * ============================================================================
 *
 * This rule is intentionally NOT a hardware target.
 *
 * It identifies a source/semantic category only.
 *
 * For example:
 *
 *     quantum
 *     classical
 *     hdl
 *     hardware
 *     distributed
 *
 * are semantic domain identifiers.
 *
 * They do not select:
 *
 *     a QPU;
 *     a GPU;
 *     a CPU;
 *     an FPGA;
 *     an ASIC;
 *     a physical node.
 */
codeGenerationDomain
    : qualifiedName
    ;


/*
 * ============================================================================
 * 14. GENERATION VALIDATION POLICY
 * ============================================================================
 *
 * Validation policy remains semantic.
 *
 * This parser boundary allows explicit source metadata to survive into the
 * AST without encoding the policy implementation.
 */
codeGenerationValidation
    : identifier
      codeGenerationContextBody
    ;


/*
 * ============================================================================
 * 15. GENERATION PROVENANCE
 * ============================================================================
 *
 * Provenance is represented using the canonical metadata mechanism.
 *
 * This rule does not define hashing, signing, attestation, or storage.
 */
codeGenerationProvenance
    : identifier
      codeGenerationContextBody
    ;


/*
 * ============================================================================
 * 16. GENERATION PHASE
 * ============================================================================
 *
 * Phase is semantic information.
 *
 * It must not be confused with runtime execution.
 */
codeGenerationPhase
    : identifier
      codeGenerationContextBody
    ;


/*
 * ============================================================================
 * 17. GENERATION CAPABILITY DECLARATION
 * ============================================================================
 *
 * This is only a composition boundary.
 *
 * Capability semantics remain owned by resources/capabilities and semantic
 * analysis.
 */
codeGenerationCapabilities
    : identifier
      codeGenerationContextBody
    ;


/*
 * ============================================================================
 * 18. GENERATION EFFECT DECLARATION
 * ============================================================================
 *
 * Effect semantics remain owned by effects/.
 */
codeGenerationEffects
    : identifier
      codeGenerationContextBody
    ;


/*
 * ============================================================================
 * 19. GENERATION HYGIENE
 * ============================================================================
 *
 * Hygiene belongs to macros/metaprogramming semantics.
 *
 * This rule exists only so explicit hygiene-related generation metadata can
 * be represented without introducing a new hygiene grammar.
 */
codeGenerationHygiene
    : identifier
      codeGenerationContextBody
    ;


/*
 * ============================================================================
 * 20. GENERATION DOMAIN
 * ============================================================================
 *
 * Domain selection is semantic classification, not physical target selection.
 */
codeGenerationDomainClause
    : identifier
      codeGenerationContextBody
    ;


/*
 * ============================================================================
 * 21. COMPOSITION ADAPTERS
 * ============================================================================
 *
 * These adapters allow the canonical metaprogramming composition grammar to
 * classify generation without changing the existing generation grammar.
 *
 * They intentionally all delegate to `generation`.
 *
 * This prevents:
 *
 *     generation.g4
 *     code-generation.g4
 *
 * from defining competing source-generation languages.
 */
metaprogrammingGeneration
    : codeGeneration
    ;


metaprogrammingCodeGeneration
    : codeGeneration
    ;


/*
 * ============================================================================
 * 22. AST CONTRACT
 * ============================================================================
 *
 * Conceptual AST mapping:
 *
 *     codeGenerationDeclaration
 *         ->
 *     MetaprogrammingGeneration
 *
 *     codeGenerationContext
 *         ->
 *     GenerationContext
 *
 *     codeGenerationContextEntry
 *         ->
 *     GenerationProperty
 *
 * The exact Rust type names belong to:
 *
 *     src/frontend/ast/
 *
 * This grammar MUST NOT prescribe Rust structure names.
 *
 * The AST must preserve:
 *
 *     source span;
 *     generation source;
 *     context properties;
 *     explicit metadata;
 *     provenance;
 *     phase information;
 *     source ordering where relevant.
 *
 * ============================================================================
 * 23. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must:
 *
 *     validate generation context;
 *     resolve generation names;
 *     validate generator inputs;
 *     validate generated structure;
 *     enforce phase boundaries;
 *     enforce hygiene;
 *     enforce effects;
 *     enforce capabilities;
 *     enforce resource requirements;
 *     preserve provenance;
 *     enforce deterministic-generation policy;
 *     reject invalid generated source.
 *
 * The grammar itself performs none of these checks.
 *
 * ============================================================================
 * 24. IR CONTRACT
 * ============================================================================
 *
 * This file creates NO IR.
 *
 * In particular it does not create:
 *
 *     GeneratedInstruction
 *     GeneratedQuantumInstruction
 *     GeneratedHardwareInstruction
 *     QuantumGate
 *     PhysicalQubit
 *     ScheduleOperation
 *     QECOperation
 *     ZQNOperation
 *
 * Generated source is lowered through the normal canonical pipeline.
 *
 * ============================================================================
 * 25. QUANTUM IR CONTRACT
 * ============================================================================
 *
 * Generated quantum source eventually reaches:
 *
 *     quantum semantic representation
 *             |
 *             v
 *         quantum::ir
 *
 * `quantum::ir` remains the canonical quantum semantic/IR boundary.
 *
 * This file must never become a source of:
 *
 *     QuantumGate enum alternatives;
 *     fixed qubit indices;
 *     physical QPU identifiers;
 *     topology definitions;
 *     routing plans.
 *
 * ============================================================================
 * 26. HDL / HARDWARE IR CONTRACT
 * ============================================================================
 *
 * Generated HDL/hardware source enters the normal HDL/hardware semantic
 * pipeline.
 *
 * This file does not define an HDL IR.
 *
 * ============================================================================
 * 27. RESOURCE CONTRACT
 * ============================================================================
 *
 * Generated source may contain resource requirements.
 *
 * Resource requirements are not resource allocations.
 *
 * Example:
 *
 *     requires qubits >= n
 *
 * is portable intent.
 *
 * It must not be transformed at grammar level into:
 *
 *     physical_qubit(0)
 *     physical_qubit(1)
 *     ...
 *
 * ============================================================================
 * 28. CAPABILITY CONTRACT
 * ============================================================================
 *
 * Capability requirements remain semantic.
 *
 * Examples:
 *
 *     capability("quantum.measurement")
 *     capability("tensor.compute")
 *     capability("gpu.compute")
 *
 * The grammar does not enumerate capabilities.
 *
 * ============================================================================
 * 29. TARGET CONTRACT
 * ============================================================================
 *
 * This grammar does not choose targets.
 *
 * Target selection remains downstream in:
 *
 *     compile/
 *     hardware/
 *     resources/
 *     execution/
 *
 * ============================================================================
 * 30. ERROR CONTRACT
 * ============================================================================
 *
 * Syntax errors belong to the parser.
 *
 * Examples:
 *
 *     synthesize
 *     synthesize (
 *     synthesize )
 *     synthesize { 
 *
 * Contextual errors belong to semantic analysis.
 *
 * Examples:
 *
 *     generation in an invalid phase;
 *     unauthorized generation effect;
 *     unavailable capability;
 *     invalid generated structure;
 *     invalid generated type;
 *     invalid generated quantum operation;
 *     invalid generated HDL structure.
 *
 * Resource exhaustion is NOT a syntax error.
 *
 * Unsupported hardware is NOT a syntax error.
 *
 * ============================================================================
 * 31. SECURITY CONTRACT
 * ============================================================================
 *
 * A parser accepting:
 *
 *     synthesize ...
 *
 * MUST NOT execute the generator.
 *
 * Compile-time execution requires:
 *
 *     explicit semantic authorization;
 *     effect checking;
 *     capability checking;
 *     resource policy;
 *     cancellation;
 *     provenance;
 *     deterministic policy where applicable.
 *
 * ============================================================================
 * 32. DETERMINISM CONTRACT
 * ============================================================================
 *
 * The grammar must be deterministic.
 *
 * Identical token streams under identical parser configuration must produce
 * equivalent parse structures.
 *
 * Generation semantics may be deterministic or explicitly effectful.
 *
 * This distinction is made downstream.
 *
 * ============================================================================
 * 33. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing source-generation syntax remains owned by generation.g4.
 *
 * Existing quotation syntax remains owned by quotation.g4.
 *
 * Existing unquotation syntax remains owned by unquotation.g4.
 *
 * Existing macro syntax remains owned by macros/.
 *
 * Existing compiler code-generation syntax remains owned by:
 *
 *     compile/code-generation.g4
 *
 * This file adds a composition boundary without changing those existing
 * syntaxes.
 *
 * ============================================================================
 * 34. CANONICAL METAPROGRAMMING COMPOSITION
 * ============================================================================
 *
 * metaprogramming.g4 should compose this facility through:
 *
 *     metaprogrammingGeneration
 *
 * or:
 *
 *     metaprogrammingCodeGeneration
 *
 * as appropriate for the canonical parser architecture.
 *
 * It MUST NOT duplicate:
 *
 *     generation
 *
 * ============================================================================
 * 35. ROOT GRAMMAR INTEGRATION
 * ============================================================================
 *
 * Zamani.g4 remains the composition root.
 *
 * It should expose the metaprogramming boundary through the existing
 * metaprogramming composition architecture.
 *
 * This file MUST NOT become a root grammar.
 *
 * ============================================================================
 * 36. COMPILER INTEGRATION
 * ============================================================================
 *
 * The compiler should conceptually consume:
 *
 *     frontend AST
 *          |
 *          v
 *     semantic generation node
 *          |
 *          v
 *     generated canonical source/semantic structure
 *          |
 *          v
 *     semantic validation
 *          |
 *          v
 *     canonical IR
 *
 * Only after this boundary may:
 *
 *     compile/code-generation.g4
 *
 * and the compiler backend code-generation subsystem become relevant.
 *
 * ============================================================================
 * 37. RUST 1.97 / 1.97.1 CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust actions.
 *
 * Rust implementations consuming it MUST:
 *
 *     compile with Rust 1.97 / Rust 1.97.1;
 *     use safe Rust;
 *     contain no `unsafe`;
 *     preserve source spans;
 *     preserve deterministic parser behavior;
 *     avoid target-size assumptions.
 *
 * ============================================================================
 * 38. POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * Existing valid source-generation forms must remain valid through:
 *
 *     generation.g4
 *
 * and must be reachable through this composition boundary.
 *
 * Tests must include:
 *
 *     synthesize ...
 *
 *     synthesize { ... }
 *
 * generated expressions;
 *
 * generated types;
 *
 * generated declarations/items;
 *
 * generated statements;
 *
 * generated program fragments;
 *
 * generation with metadata;
 *
 * generation with provenance metadata;
 *
 * generation with semantic domain metadata;
 *
 * generated classical structures;
 *
 * generated quantum structures;
 *
 * generated HDL structures;
 *
 * generated hardware-intent structures;
 *
 * generated distributed structures;
 *
 * generated AI/data structures.
 *
 * ============================================================================
 * 39. NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * Tests must reject malformed generation syntax, including:
 *
 *     synthesize
 *
 *     synthesize (
 *
 *     synthesize )
 *
 *     synthesize (
 *     )
 *
 *     synthesize { 
 *
 * and malformed context forms.
 *
 * Semantic tests must separately reject:
 *
 *     unauthorized generation;
 *     invalid phase;
 *     invalid generated AST;
 *     invalid generated type;
 *     invalid generated domain;
 *     invalid capability;
 *     unauthorized effect.
 *
 * ============================================================================
 * 40. BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Boundary tests must include:
 *
 *     empty generated block where the canonical generation grammar permits it;
 *     single generated structure;
 *     many generated structures;
 *     nested generation;
 *     generation inside quotation;
 *     unquotation inside generated source;
 *     macro-produced generation;
 *     generated quantum operations;
 *     generated HDL structures;
 *     generated tensor/data structures;
 *     deeply nested semantic generation;
 *     large generation contexts;
 *     large generated source structures.
 *
 * Exact workload size belongs to the test harness, not this grammar.
 *
 * ============================================================================
 * 41. SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Scalability tests must increase workload according to available resources.
 *
 * The grammar must not define a maximum.
 *
 * Tests must verify that no artificial language ceiling exists for:
 *
 *     generated items;
 *     generated expressions;
 *     generation nesting;
 *     generation properties;
 *     generated quantum operations;
 *     generated classical operations;
 *     generated HDL structures;
 *     generated data structures;
 *     generated distributed structures.
 *
 * Compiler resource exhaustion must remain distinguishable from invalid
 * syntax.
 *
 * ============================================================================
 * 42. CROSS-PARSER CONFORMANCE
 * ============================================================================
 *
 * Stable generation syntax must agree between:
 *
 *     ANTLR grammar
 *
 * and:
 *
 *     canonical Rust parser
 *
 * Any intentional divergence requires an explicit compatibility contract.
 *
 * ============================================================================
 * 43. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file passes the architectural hard-coding audit only if it contains no
 * universal machine/resource limits.
 *
 * Forbidden examples include:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * Also forbidden are generation-specific versions of those limits.
 *
 * ============================================================================
 * 44. DOMAIN EXTENSIBILITY
 * ============================================================================
 *
 * A future computing domain must not require changing this grammar merely to
 * make its generated source representable.
 *
 * The domain's own canonical grammar and semantic system should define the
 * new structure.
 *
 * This file remains the domain-neutral generation boundary.
 *
 * ============================================================================
 * 45. FUTURE-PROOFING
 * ============================================================================
 *
 * New generation mechanisms should normally be added by:
 *
 *     1. defining semantic requirements;
 *     2. defining AST representation;
 *     3. defining canonical source syntax;
 *     4. extending generation.g4 if new source-generation syntax is needed;
 *     5. exposing the integration boundary here;
 *     6. adding semantic validation;
 *     7. defining IR lowering;
 *     8. adding positive/negative/boundary/scalability tests;
 *     9. updating compatibility/conformance documentation.
 *
 * This file should not become a catalogue of every generator technology.
 *
 * ============================================================================
 * 46. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] Source-level generation ownership is separated from backend
 *         compilation code generation.
 *
 *     [x] Existing generation.g4 remains the source-generation authority.
 *
 *     [x] Existing compile/code-generation.g4 remains the compiler/backend
 *         code-generation authority.
 *
 *     [x] No lexer is duplicated.
 *
 *     [x] No keyword is invented.
 *
 *     [x] No expression grammar is duplicated.
 *
 *     [x] No type grammar is duplicated.
 *
 *     [x] No declaration grammar is duplicated.
 *
 *     [x] No statement grammar is duplicated.
 *
 *     [x] No quotation grammar is duplicated.
 *
 *     [x] No unquotation grammar is duplicated.
 *
 *     [x] No macro grammar is duplicated.
 *
 *     [x] No second AST is introduced.
 *
 *     [x] No second semantic model is introduced.
 *
 *     [x] No second IR is introduced.
 *
 *     [x] quantum::ir remains the canonical quantum IR boundary.
 *
 *     [x] Generated quantum source re-enters the normal quantum pipeline.
 *
 *     [x] Generated HDL source re-enters the normal HDL pipeline.
 *
 *     [x] Generated classical source re-enters the normal classical pipeline.
 *
 *     [x] Resource requirements remain separate from resource realization.
 *
 *     [x] Capability requirements remain separate from target selection.
 *
 *     [x] No physical hardware is selected by this grammar.
 *
 *     [x] No artificial scalability limits are introduced.
 *
 *     [x] Provenance is represented as an explicit semantic concern.
 *
 *     [x] Determinism is separated from syntax.
 *
 *     [x] Security boundaries are explicit.
 *
 *     [x] Rust 1.97 / 1.97.1 compatibility is specified.
 *
 *     [x] Safe Rust / no-unsafe requirement is specified.
 *
 *     [x] Positive tests are specified.
 *
 *     [x] Negative tests are specified.
 *
 *     [x] Boundary tests are specified.
 *
 *     [x] Scalability tests are specified.
 *
 *     [x] Cross-parser conformance is specified.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * This file is an INTEGRATION BOUNDARY, not a second generation language.
 *
 * The authoritative source-generation syntax remains:
 *
 *     grammar/metaprogramming/generation.g4
 *
 * The authoritative compiler/backend generation syntax remains:
 *
 *     grammar/compile/code-generation.g4
 *
 * This file connects metaprogramming code-generation intent to the canonical
 * Zamani frontend without taking ownership away from either existing file.
 *
 * The resulting architecture is:
 *
 *     metaprogramming code generation
 *                |
 *                v
 *        generation.g4
 *                |
 *                v
 *        canonical AST
 *                |
 *                v
 *       semantic validation
 *                |
 *                v
 *          canonical IR
 *                |
 *        +-------+-------+
 *        |       |       |
 *        v       v       v
 *     classical quantum  HDL
 *               |
 *          quantum::ir
 *                |
 *                v
 *       optimization/lowering
 *                |
 *       routing/scheduling
 *                |
 *          QEC / ZQN
 *                |
 *                v
 *               HAL
 *                |
 *                v
 *       compiler code generation
 *                |
 *                v
 *       target realization
 *
 * Therefore generated programs remain target-independent while still being
 * able to express arbitrary classical, quantum, HDL, hybrid, distributed,
 * AI, data, networking, security, and future computational structures.
 *
 * The language remains governed by:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * and scalability is constrained by actual available resources rather than
 * artificial grammar limits.
 *
 * ============================================================================
 */
 
parser grammar MetaprogrammingCodeGeneration;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * PUBLIC COMPOSITION ENTRY POINT
 * ============================================================================
 *
 * This is intentionally a wrapper around the existing `generation` rule.
 *
 * The actual generation syntax remains owned by:
 *
 *     grammar/metaprogramming/generation.g4
 */
codeGeneration
    : generation
    ;


/*
 * ============================================================================
 * DECLARATION CONTEXT
 * ============================================================================
 *
 * The canonical parser determines whether this wrapper is legal in a
 * declaration/item position.
 */
codeGenerationDeclaration
    : generation codeGenerationContext?
    ;


/*
 * ============================================================================
 * EXPRESSION CONTEXT
 * ============================================================================
 */
codeGenerationExpression
    : generation
    ;


/*
 * ============================================================================
 * STATEMENT CONTEXT
 * ============================================================================
 */
codeGenerationStatement
    : generation
    ;


/*
 * ============================================================================
 * ITEM CONTEXT
 * ============================================================================
 */
codeGenerationItem
    : generation
    ;


/*
 * ============================================================================
 * PROGRAM-FRAGMENT CONTEXT
 * ============================================================================
 */
codeGenerationProgram
    : generation
    ;


/*
 * ============================================================================
 * GENERATION CONTEXT
 * ============================================================================
 *
 * The context introducer is intentionally an ordinary identifier.
 *
 * This avoids introducing a new keyword solely for metaprogramming.
 *
 * Semantic analysis determines whether the selected context is valid.
 */
codeGenerationContext
    : identifier
      codeGenerationContextBody
    ;


codeGenerationContextBody
    : LPAREN codeGenerationContextEntries? RPAREN
    | LBRACE codeGenerationContextEntries? RBRACE
    ;


codeGenerationContextEntries
    : codeGenerationContextEntry
      (COMMA codeGenerationContextEntry)*
    ;


codeGenerationContextEntry
    : identifier
      codeGenerationContextValue?
    ;


codeGenerationContextValue
    : ASSIGN expression
    | LPAREN expression RPAREN
    ;


/*
 * ============================================================================
 * GENERATION DOMAIN
 * ============================================================================
 *
 * A domain is a semantic classification.
 *
 * It is NOT a physical target.
 */
codeGenerationDomain
    : qualifiedName
    ;


/*
 * ============================================================================
 * GENERATION RESULT CATEGORY
 * ============================================================================
 *
 * The category is semantic data rather than a finite parser enumeration.
 */
codeGenerationResult
    : identifier
    ;


/*
 * ============================================================================
 * GENERATION PROVENANCE
 * ============================================================================
 *
 * Provenance semantics belong downstream.
 */
codeGenerationProvenance
    : identifier
      codeGenerationContextBody
    ;


/*
 * ============================================================================
 * GENERATION VALIDATION POLICY
 * ============================================================================
 */
codeGenerationValidation
    : identifier
      codeGenerationContextBody
    ;


/*
 * ============================================================================
 * GENERATION PHASE
 * ============================================================================
 */
codeGenerationPhase
    : identifier
      codeGenerationContextBody
    ;


/*
 * ============================================================================
 * GENERATION CAPABILITIES
 * ============================================================================
 */
codeGenerationCapabilities
    : identifier
      codeGenerationContextBody
    ;


/*
 * ============================================================================
 * GENERATION EFFECTS
 * ============================================================================
 */
codeGenerationEffects
    : identifier
      codeGenerationContextBody
    ;


/*
 * ============================================================================
 * GENERATION HYGIENE
 * ============================================================================
 */
codeGenerationHygiene
    : identifier
      codeGenerationContextBody
    ;


/*
 * ============================================================================
 * GENERATION DOMAIN CLAUSE
 * ============================================================================
 */
codeGenerationDomainClause
    : identifier
      codeGenerationContextBody
    ;


/*
 * ============================================================================
 * METAPROGRAMMING COMPOSITION ADAPTERS
 * ============================================================================
 *
 * These aliases give metaprogramming.g4 stable names without creating another
 * implementation of generation syntax.
 */
metaprogrammingGeneration
    : codeGeneration
    ;


metaprogrammingCodeGeneration
    : codeGeneration
    ;


/*
 * ============================================================================
 * CANONICAL RULE DEPENDENCIES
 * ============================================================================
 *
 * The following rules are intentionally NOT defined in this file:
 *
 *     generation
 *     identifier
 *     qualifiedName
 *     expression
 *
 * They belong to canonical repository grammar components.
 *
 * The ANTLR composition/build system must make those rules visible when this
 * grammar is assembled into the authoritative parser.
 *
 * ============================================================================
 */
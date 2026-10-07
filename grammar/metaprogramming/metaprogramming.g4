/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/metaprogramming/metaprogramming.g4
 *
 * GRAMMAR
 * -------
 * Metaprogramming
 *
 * STATUS
 * ------
 * Production composition/orchestration grammar
 *
 * COMPILER BASELINE
 * -----------------
 * Rust 1.97 or later
 * Rust 2021 edition
 * Safe Rust only
 * No unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the SINGLE COMPOSITION ROOT for the grammar/metaprogramming/
 * subsystem.
 *
 * It owns orchestration only.
 *
 * It does NOT own the detailed syntax of any metaprogramming facility.
 *
 * The leaf grammars remain the authorities for their respective syntax:
 *
 *     capabilities.g4
 *     code-generation.g4
 *     compile-time-execution.g4
 *     compile-time.g4
 *     generation.g4
 *     introspection.g4
 *     quotation.g4
 *     reflection.g4
 *     schemas.g4
 *     specialization.g4
 *     syntax-tree.g4
 *     type-level.g4
 *     unquotation.g4
 *
 * Macro syntax remains owned by:
 *
 *     grammar/macros/
 *
 * Expression-level metaprogramming dispatch remains owned by:
 *
 *     grammar/expressions/metaprogramming.g4
 *
 * This file connects those facilities without creating a second language
 * hierarchy.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     canonical lexer
 *       |
 *       v
 *     canonical parser
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +--> type analysis
 *       +--> effect analysis
 *       +--> capability analysis
 *       +--> resource analysis
 *       +--> policy analysis
 *       +--> contract analysis
 *       +--> provenance
 *       |
 *       v
 *     metaprogramming analysis
 *       |
 *       +--> compile-time execution
 *       +--> reflection
 *       +--> introspection
 *       +--> quotation
 *       +--> unquotation
 *       +--> source generation
 *       +--> specialization
 *       +--> type-level computation
 *       +--> schema transformation
 *       +--> metaprogramming capability requests
 *       +--> syntax-tree operations
 *       |
 *       v
 *     generated/transformed canonical source
 *       |
 *       v
 *     canonical frontend again
 *       |
 *       v
 *     semantic model
 *       |
 *       +--> classical representation
 *       +--> quantum::ir
 *       +--> HDL/hardware representation
 *       +--> AI/data representation
 *       +--> distributed representation
 *       +--> networking representation
 *       +--> future domain representations
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     lowering
 *       |
 *       v
 *     routing / scheduling / resilience
 *       |
 *       v
 *     ZQN / HAL / target realization
 *
 * This grammar MUST NOT bypass any stage of that pipeline.
 *
 * ============================================================================
 * ARCHITECTURAL AUTHORITY
 * ============================================================================
 *
 * Universal grammar architecture:
 *
 *     grammar/DESIGN.md
 *
 * Complete-language composition:
 *
 *     grammar/Zamani.g4
 *
 * Canonical parser composition:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * Canonical lexical vocabulary:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Canonical lexer-token grammar:
 *
 *     grammar/lexer/tokens.g4
 *
 * Metaprogramming documentation:
 *
 *     grammar/metaprogramming/README.md
 *
 * Metaprogramming specification:
 *
 *     grammar/spec/
 *     grammar/specification/
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * This file MUST remain a COMPOSITION grammar.
 *
 * It MUST NOT become the owner of:
 *
 *     - lexer rules;
 *     - keyword spelling;
 *     - identifier syntax;
 *     - qualified-name syntax;
 *     - path syntax;
 *     - ordinary expression syntax;
 *     - expression precedence;
 *     - ordinary statement syntax;
 *     - ordinary declaration syntax;
 *     - ordinary type syntax;
 *     - pattern syntax;
 *     - macro syntax;
 *     - macro expansion;
 *     - macro hygiene;
 *     - compile-time evaluation;
 *     - reflection implementation;
 *     - introspection implementation;
 *     - code-generation implementation;
 *     - specialization algorithms;
 *     - type checking;
 *     - schema validation;
 *     - semantic analysis;
 *     - capability authorization;
 *     - resource resolution;
 *     - policy evaluation;
 *     - provenance implementation;
 *     - AST construction;
 *     - IR construction;
 *     - quantum lowering;
 *     - HDL lowering;
 *     - hardware selection;
 *     - target selection;
 *     - routing;
 *     - scheduling;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar defines NO language-level capacity ceilings.
 *
 * In particular, it MUST NOT define fixed limits for:
 *
 *     macros
 *     generated declarations
 *     generated expressions
 *     generated statements
 *     generated source
 *     syntax-tree nodes
 *     quotation depth
 *     unquotation depth
 *     specialization count
 *     type-level complexity
 *     compile-time computation
 *     reflection requests
 *     introspection requests
 *     schema members
 *     schema transformations
 *     capabilities
 *     resource requirements
 *     quantum resources
 *     classical resources
 *     HDL resources
 *     distributed resources
 *     AI/data resources
 *
 * There must be no language constants corresponding to machine capacity.
 *
 * Compiler implementations MAY impose configurable operational safeguards
 * for memory, execution time, recursion, expansion, diagnostics, or other
 * implementation resources.
 *
 * Such safeguards are implementation policy.
 *
 * They MUST NOT become source-language ceilings.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Metaprogramming MUST preserve:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Therefore this grammar MUST NOT encode:
 *
 *     CPU identity
 *     GPU identity
 *     FPGA identity
 *     ASIC identity
 *     QPU identity
 *     physical qubit identity
 *     fixed device counts
 *     fixed memory sizes
 *     fixed register widths
 *     fixed topology sizes
 *     vendor-specific machine layouts
 *
 * A metaprogram may generate source containing semantic requirements such as:
 *
 *     requires capability(...)
 *     requires memory >= ...
 *     requires topology(...)
 *
 * but the realization of those requirements belongs downstream to semantic
 * resource/capability negotiation and target realization.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Parsing is NEVER execution.
 *
 * Successful parsing of a metaprogramming construct MUST NOT imply permission
 * to:
 *
 *     - read arbitrary files;
 *     - access arbitrary directories;
 *     - access networks;
 *     - read credentials;
 *     - read secrets;
 *     - inspect protected environment state;
 *     - invoke subprocesses;
 *     - access devices;
 *     - access hardware;
 *     - invoke a QPU;
 *     - invoke a GPU;
 *     - invoke an FPGA;
 *     - mutate compiler state.
 *
 * Such operations require explicit downstream semantic authorization through
 * the repository's effect, capability, resource, policy, and security
 * systems.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing is deterministic with respect to:
 *
 *     source token sequence
 *     grammar composition
 *     parser configuration relevant to syntax
 *
 * This file MUST NOT execute:
 *
 *     - compile-time programs;
 *     - reflection;
 *     - introspection;
 *     - macros;
 *     - generated code.
 *
 * Determinism and reproducibility of those operations belong to semantic and
 * compiler infrastructure.
 *
 * ============================================================================
 * IMPORT COMPOSITION
 * ============================================================================
 *
 * Every grammar imported below is a parser grammar belonging to the
 * metaprogramming subsystem.
 *
 * The imports intentionally compose LEAF FACILITIES rather than reproducing
 * their rules.
 *
 * Imported grammar ownership:
 *
 *     MetaprogrammingCapabilities
 *         capabilities.g4
 *
 *     MetaprogrammingCodeGeneration
 *         code-generation.g4
 *
 *     CompileTimeExecution
 *         compile-time-execution.g4
 *
 *     CompileTime
 *         compile-time.g4
 *
 *     Generation
 *         generation.g4
 *
 *     Introspection
 *         introspection.g4
 *
 *     Quotation
 *         quotation.g4
 *
 *     Reflection
 *         reflection.g4
 *
 *     ZamaniMetaSchemas
 *         schemas.g4
 *
 *     Specialization
 *         specialization.g4
 *
 *     syntaxTree
 *         syntax-tree.g4
 *
 *     TypeLevelMetaprogramming
 *         type-level.g4
 *
 *     Unquotation
 *         unquotation.g4
 *
 * Macro grammar is NOT imported here because grammar/macros/ is independently
 * composed by the canonical parser and must remain a separate owner.
 *
 * ============================================================================
 */

parser grammar Metaprogramming;

options {
    tokenVocab = ZamaniLexer;
}

import
    MetaprogrammingCapabilities,
    MetaprogrammingCodeGeneration,
    CompileTimeExecution,
    CompileTime,
    Generation,
    Introspection,
    Quotation,
    Reflection,
    ZamaniMetaSchemas,
    Specialization,
    syntaxTree,
    TypeLevelMetaprogramming,
    Unquotation
;


/*
 * ============================================================================
 * 1. PRIMARY METAPROGRAMMING DECLARATION BOUNDARY
 * ============================================================================
 *
 * This is the stable declaration-level integration point consumed by the
 * canonical parser.
 *
 * Each alternative delegates to exactly one authoritative facility.
 *
 * No detailed syntax is defined here.
 */

metaprogrammingDeclaration
    : compileTimeDeclaration
    | codeGenerationDeclaration
    | specializationDeclaration
    | metaSchemaDeclaration
    | typeLevelDeclaration
    ;


/*
 * ============================================================================
 * 2. PRIMARY METAPROGRAMMING STATEMENT BOUNDARY
 * ============================================================================
 *
 * This is the stable statement-level integration point consumed by the
 * canonical parser.
 *
 * Statement-producing facilities are delegated to their owning grammars.
 */

metaprogrammingStatement
    : compileTimeStatement
    | codeGenerationStatement
    | metaSchemaStatement
    ;


/*
 * ============================================================================
 * 3. METAPROGRAMMING EXPRESSION BOUNDARY
 * ============================================================================
 *
 * IMPORTANT:
 *
 * The expression-level grammar:
 *
 *     grammar/expressions/metaprogramming.g4
 *
 * already owns the public rule:
 *
 *     metaprogrammingExpression
 *
 * Therefore this composition grammar MUST NOT define another rule with that
 * name.
 *
 * Instead, this subsystem exports the distinct facility boundary below.
 *
 * The canonical expression-level component can consume the corresponding
 * facility expressions when its repository-wide composition is finalized.
 */

metaprogrammingFacilityExpression
    : compileTimeExpression
    | generationExpression
    | codeGenerationExpression
    | reflectionExpression
    | introspectionExpressionCore
    | specializationExpression
    | metaSchemaExpression
    | typeLevelExpression
    | quoteExpressionCore
    | unquotationExpression
    ;


/*
 * ============================================================================
 * 4. COMPILE-TIME FACILITY
 * ============================================================================
 *
 * compile-time.g4 is the composition boundary between metaprogramming and
 * compile-time execution.
 *
 * compile-time-execution.g4 owns the actual explicit compile-time execution
 * syntax.
 *
 * This file merely exposes the stable subsystem entry.
 */

metaprogrammingCompileTime
    : compileTimeDeclaration
    | compileTimeExpression
    | compileTimeStatement
    ;


/*
 * ============================================================================
 * 5. SOURCE-GENERATION FACILITY
 * ============================================================================
 *
 * Generation and code-generation intentionally remain separate concepts:
 *
 * generation.g4
 *     owns source-level generation requests.
 *
 * code-generation.g4
 *     owns metaprogramming-side generation composition/context.
 *
 * Neither grammar emits backend instructions.
 */

metaprogrammingGeneration
    : generationExpression
    | generationDeclaration
    | generationStatement
    | codeGenerationDeclaration
    | codeGenerationExpression
    | codeGenerationStatement
    ;


/*
 * ============================================================================
 * 6. REFLECTION / INTROSPECTION FACILITY
 * ============================================================================
 *
 * Reflection and introspection are related but distinct:
 *
 * reflection
 *     operates on language structures/semantic entities according to the
 *     reflection subsystem.
 *
 * introspection
 *     requests explicit information about a subject/environment according
 *     to introspection semantics.
 *
 * Neither operation is performed by parsing.
 */

metaprogrammingReflection
    : reflectionExpression
    | reflectionDeclaration
    | reflectionStatement
    ;

metaprogrammingIntrospection
    : introspectionExpressionCore
    ;


/*
 * ============================================================================
 * 7. QUOTATION / UNQUOTATION FACILITY
 * ============================================================================
 *
 * quotation.g4 owns quotation core syntax.
 *
 * unquotation.g4 owns the unquotation integration boundary.
 *
 * This file only composes them.
 */

metaprogrammingQuotation
    : quoteExpressionCore
    | metaQuoteCore
    ;

metaprogrammingUnquotation
    : unquoteExpressionCore
    | metaSpliceCore
    | unquotationExpression
    | unquotationSplice
    ;


/*
 * ============================================================================
 * 8. SPECIALIZATION FACILITY
 * ============================================================================
 *
 * Specialization is an explicit semantic request.
 *
 * It does not mean immediate backend code generation.
 *
 * The semantic/compiler pipeline decides whether and how specialization is
 * performed.
 */

metaprogrammingSpecialization
    : specializationRequest
    | specializationExpression
    | specializationDeclaration
    | specializationRequestWithMetadata
    | specializationRequestWithBody
    | completeSpecializationRequest
    | completeSpecializationDeclaration
    ;


/*
 * ============================================================================
 * 9. TYPE-LEVEL FACILITY
 * ============================================================================
 *
 * Type-level computation remains distinct from ordinary runtime expression
 * evaluation.
 *
 * The type-level grammar does not define another ordinary type system.
 */

metaprogrammingTypeLevel
    : typeLevelMetaprogramming
    | typeLevelDeclaration
    | typeLevelExpression
    ;


/*
 * ============================================================================
 * 10. SCHEMA METAPROGRAMMING FACILITY
 * ============================================================================
 *
 * Schema transformation remains a metaprogramming concern, while ordinary
 * schema syntax remains owned by the ordinary data/declaration grammar.
 */

metaprogrammingSchema
    : metaSchemaDeclaration
    | metaSchemaExpression
    | metaSchemaStatement
    | metaSchemaTransformation
    | metaSchemaGeneration
    | metaSchemaValidation
    | metaSchemaCompatibility
    ;


/*
 * ============================================================================
 * 11. CAPABILITY FACILITY
 * ============================================================================
 *
 * Capability syntax describes semantic intent.
 *
 * Parsing does NOT grant capabilities.
 *
 * Capability resolution/authorization belongs downstream.
 */

metaprogrammingCapability
    : metaprogrammingCapabilityDeclaration
    | metaprogrammingCapabilityRequirement
    | metaprogrammingCapabilityRestriction
    | metaprogrammingCapabilityExpression
    ;


/*
 * ============================================================================
 * 12. SYNTAX-TREE FACILITY
 * ============================================================================
 *
 * syntax-tree.g4 is an adapter over the canonical macro token-tree system.
 *
 * This file does not redefine token-tree structure.
 */

metaprogrammingSyntaxTree
    : syntaxTree
    | syntaxTreeNode
    ;


/*
 * ============================================================================
 * 13. UNIFIED METAPROGRAMMING ELEMENT
 * ============================================================================
 *
 * This is the broadest metaprogramming composition boundary.
 *
 * It exists for consumers that need to process any metaprogramming-specific
 * construct without knowing which leaf facility owns it.
 *
 * IMPORTANT:
 *
 * This is NOT the complete-language `sourceElement` rule.
 *
 * The canonical parser remains responsible for deciding where this category
 * is legal.
 */

metaprogrammingElement
    : metaprogrammingDeclaration
    | metaprogrammingStatement
    | metaprogrammingFacilityExpression
    | metaprogrammingCapability
    | metaprogrammingSyntaxTree
    ;


/*
 * ============================================================================
 * 14. TRANSFORMATION BOUNDARY
 * ============================================================================
 *
 * All transformations eventually produce canonical Zamani source/semantic
 * structures.
 *
 * This grammar does not execute the transformation.
 */

metaprogrammingTransformation
    : metaprogrammingFacilityExpression
    | metaprogrammingDeclaration
    | metaprogrammingStatement
    ;


/*
 * ============================================================================
 * 15. META-VALUE ADAPTERS
 * ============================================================================
 *
 * These adapters deliberately reuse canonical language rules.
 *
 * They MUST NOT create another expression, type, name, pattern, or declaration
 * hierarchy.
 *
 * Canonical rules are supplied by the complete parser composition.
 */

metaprogrammingValue
    : expression
    | typeExpression
    | pattern
    | qualifiedName
    ;

metaprogrammingSource
    : expression
    | statement
    | item
    ;

metaprogrammingName
    : identifier
    ;

metaprogrammingPath
    : qualifiedName
    ;

metaprogrammingType
    : typeExpression
    ;

metaprogrammingPattern
    : pattern
    ;


/*
 * ============================================================================
 * 16. GENERATED-SOURCE BOUNDARY
 * ============================================================================
 *
 * Generated declarations/statements/expressions are SOURCE STRUCTURES.
 *
 * They MUST NOT be interpreted as backend instructions by this grammar.
 *
 * Generated source must re-enter:
 *
 *     lexer
 *       ->
 *     parser
 *       ->
 *     AST
 *       ->
 *     semantic analysis
 *       ->
 *     canonical semantic model
 *       ->
 *     domain IR
 *
 * including:
 *
 *     quantum::ir
 *
 * where the generated source describes quantum computation.
 */

metaprogrammingGeneratedSource
    : generatedSource
    | generatedDeclaration
    | generatedStatement
    | generatedExpression
    | generatedType
    ;


/*
 * ============================================================================
 * 17. CANONICAL SOURCE-REENTRY CONTRACT
 * ============================================================================
 *
 * This rule represents a metaprogram's intent to produce ordinary Zamani
 * source structure.
 *
 * The parser only records syntax.
 *
 * The compiler is responsible for:
 *
 *     1. validating the produced structure;
 *     2. assigning source/generated provenance;
 *     3. performing name resolution;
 *     4. performing type checking;
 *     5. performing effect checking;
 *     6. checking capabilities;
 *     7. checking resource requirements;
 *     8. checking policies;
 *     9. validating contracts;
 *    10. lowering to canonical semantic representation.
 *
 * There is no direct generated-source-to-backend shortcut.
 */

metaprogrammingSourceReentry
    : generatedSource
    ;


/*
 * ============================================================================
 * 18. CAPABILITY / RESOURCE / EFFECT INTEGRATION CONTRACT
 * ============================================================================
 *
 * Metaprogramming facilities may participate semantically in:
 *
 *     capabilities
 *     resources
 *     effects
 *     policies
 *     contracts
 *     provenance
 *
 * This grammar does not implement those analyses.
 *
 * In particular, compile-time execution, reflection, introspection,
 * generation, specialization, and adaptation must not silently acquire
 * privileged access merely because their syntax was accepted.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 19. DOMAIN-NEUTRAL INTEGRATION CONTRACT
 * ============================================================================
 *
 * Metaprogramming can generate or transform constructs belonging to:
 *
 *     classical computing
 *     quantum computing
 *     hybrid computing
 *     HDL
 *     hardware
 *     AI/ML
 *     data/tensor computing
 *     distributed computing
 *     networking
 *     cryptography
 *     accelerators
 *     embedded systems
 *     HPC
 *     cloud execution
 *     future computational domains
 *
 * No domain-specific metaprogramming grammar is required merely because a
 * generated construct belongs to a new domain.
 *
 * Domain semantics remain owned by the corresponding domain grammar and
 * semantic subsystem.
 *
 * ============================================================================
 * 20. QUANTUM INTEGRATION CONTRACT
 * ============================================================================
 *
 * Metaprogramming MUST NOT define:
 *
 *     quantum gate lists
 *     physical qubits
 *     device-specific qubits
 *     routing tables
 *     calibration data
 *     hardware topology
 *     QEC implementation
 *
 * If generated source describes quantum computation, the normal path is:
 *
 *     generated source
 *       ->
 *     canonical AST
 *       ->
 *     quantum semantic analysis
 *       ->
 *     quantum::ir
 *       ->
 *     optimization
 *       ->
 *     decomposition
 *       ->
 *     routing
 *       ->
 *     scheduling
 *       ->
 *     resilience / QEC
 *       ->
 *     ZQN
 *       ->
 *     HAL
 *       ->
 *     target realization
 *
 * There is no metaprogramming-specific quantum IR.
 *
 * ============================================================================
 * 21. HDL / HARDWARE INTEGRATION CONTRACT
 * ============================================================================
 *
 * Generated HDL or hardware-intent source follows the ordinary HDL/hardware
 * semantic pipeline.
 *
 * This file MUST NOT encode:
 *
 *     fixed bus widths
 *     fixed register widths
 *     fixed device counts
 *     fixed FPGA resources
 *     fixed ASIC resources
 *     fixed clock frequencies
 *     fixed memory capacities
 *     fixed topology dimensions
 *
 * Hardware realization is downstream.
 *
 * ============================================================================
 * 22. DISTRIBUTED / NETWORK / AI / DATA INTEGRATION
 * ============================================================================
 *
 * Generated constructs belonging to distributed computing, networking,
 * AI/ML, data processing, or other domains re-enter their normal semantic
 * owners.
 *
 * Metaprogramming does not create:
 *
 *     a second actor system;
 *     a second resource model;
 *     a second capability model;
 *     a second effect model;
 *     a second data model;
 *     a second execution model.
 *
 * ============================================================================
 * 23. PROVENANCE CONTRACT
 * ============================================================================
 *
 * Metaprogramming transformations should preserve enough information for
 * downstream infrastructure to establish:
 *
 *     source origin
 *     transformation origin
 *     generated artifact origin
 *     transformation sequence
 *     transformation inputs
 *     policy context
 *     capability context
 *     semantic decisions
 *
 * The grammar only preserves syntactic structure.
 *
 * Provenance storage and verification belong downstream.
 *
 * ============================================================================
 * 24. DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics are syntactic.
 *
 * Examples:
 *
 *     malformed delimiters
 *     malformed arguments
 *     malformed source structure
 *     malformed specialization syntax
 *     malformed quotation
 *     malformed type-level expression
 *
 * The parser MUST NOT attempt to diagnose:
 *
 *     unavailable hardware
 *     unavailable resources
 *     missing capabilities
 *     unauthorized reflection
 *     unauthorized introspection
 *     invalid specialization semantics
 *     failed type-level normalization
 *     nondeterministic compile-time evaluation
 *     policy violations
 *     semantic type errors
 *
 * Those belong to downstream phases.
 *
 * ============================================================================
 * 25. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing filenames remain stable.
 *
 * The public metaprogramming composition boundary is:
 *
 *     metaprogrammingDeclaration
 *     metaprogrammingStatement
 *     metaprogrammingElement
 *
 * The expression-level public boundary remains owned by:
 *
 *     grammar/expressions/metaprogramming.g4
 *
 * This separation prevents duplicate ownership of:
 *
 *     metaprogrammingExpression
 *
 * Internal leaf rules may evolve without changing this composition boundary
 * provided the public delegated rule contracts remain stable.
 *
 * ============================================================================
 * 26. FORBIDDEN HARD-CODING
 * ============================================================================
 *
 * This grammar MUST NOT contain artificial computational capacity constants.
 *
 * In particular, none of the following may appear as language-level limits:
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
 * It must also not encode fixed:
 *
 *     machine sizes
 *     memory sizes
 *     processor counts
 *     accelerator counts
 *     qubit counts
 *     topology sizes
 *     vendor device identifiers
 *     physical addresses
 *
 * ============================================================================
 * 27. SAFE-RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no embedded target-language actions.
 *
 * Therefore this grammar itself introduces no unsafe Rust requirement.
 *
 * Compiler/runtime implementation associated with these parser contexts MUST
 * use safe Rust under the repository's Rust 1.97-or-later baseline.
 *
 * No `unsafe` implementation is required by this grammar contract.
 *
 * ============================================================================
 * 28. INTEGRATION WITH CANONICAL AST
 * ============================================================================
 *
 * Parser contexts produced here are mapped into the repository's existing
 * domain-neutral frontend AST.
 *
 * The AST mapping must preserve:
 *
 *     source spans
 *     source ordering
 *     nesting
 *     facility identity
 *     names
 *     paths
 *     arguments
 *     attributes
 *     quotation structure
 *     generated-source provenance
 *
 * This grammar MUST NOT introduce:
 *
 *     MetaprogrammingAst
 *     MetaprogrammingIR
 *     QuantumMetaprogrammingIR
 *     HardwareMetaprogrammingIR
 *
 * ============================================================================
 * 29. IR CONTRACT
 * ============================================================================
 *
 * This grammar generates NO IR.
 *
 * The canonical IR architecture remains downstream.
 *
 * Generated classical constructs use the canonical classical semantic/IR
 * pipeline.
 *
 * Generated quantum constructs use:
 *
 *     quantum::ir
 *
 * Generated HDL/hardware constructs use the canonical HDL/hardware semantic
 * representation.
 *
 * There is no metaprogramming-specific IR.
 *
 * ============================================================================
 * 30. TARGET-INDEPENDENCE CONTRACT
 * ============================================================================
 *
 * This grammar must remain independent of:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     simulator
 *     cluster
 *     supercomputer
 *     cloud provider
 *     vendor
 *
 * Target-dependent specialization is a downstream semantic/compiler decision.
 *
 * ============================================================================
 * 31. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [ ] It is a parser grammar.
 *     [ ] It uses the canonical ZamaniLexer vocabulary.
 *     [ ] All imported grammars exist.
 *     [ ] Every imported grammar has one owner.
 *     [ ] No leaf grammar is reimplemented here.
 *     [ ] No phantom `*Core` dependency is introduced.
 *     [ ] Macro syntax remains owned by grammar/macros/.
 *     [ ] Expression-level metaprogramming remains owned by the expression
 *         metaprogramming component.
 *     [ ] Compile-time facilities are composed.
 *     [ ] Generation facilities are composed.
 *     [ ] Reflection is composed.
 *     [ ] Introspection is composed.
 *     [ ] Quotation is composed.
 *     [ ] Unquotation is composed.
 *     [ ] Specialization is composed.
 *     [ ] Type-level metaprogramming is composed.
 *     [ ] Schema metaprogramming is composed.
 *     [ ] Capability syntax is composed.
 *     [ ] Syntax-tree support is composed.
 *     [ ] Code-generation composition is composed.
 *     [ ] Generated source re-enters the canonical frontend.
 *     [ ] No second AST is introduced.
 *     [ ] No second IR is introduced.
 *     [ ] quantum::ir remains the canonical quantum representation.
 *     [ ] No target-specific syntax is introduced.
 *     [ ] No fixed computational capacity is introduced.
 *     [ ] No parser action executes code.
 *     [ ] Parser diagnostics remain syntactic.
 *     [ ] Semantic authorization remains downstream.
 *     [ ] Resource resolution remains downstream.
 *     [ ] Policy evaluation remains downstream.
 *     [ ] Provenance remains downstream.
 *     [ ] Safe Rust remains sufficient.
 *     [ ] Rust 1.97 or later remains supported.
 *
 * ============================================================================
 * 32. FINAL ARCHITECTURAL INVARIANT
 * ============================================================================
 *
 * This file is the ORCHESTRATOR, not the implementation of every facility.
 *
 * The invariant is:
 *
 *     one subsystem root
 *         ->
 *     many independently owned facilities
 *         ->
 *     one canonical frontend
 *         ->
 *     one domain-neutral AST
 *         ->
 *     one semantic model
 *         ->
 *     canonical domain IRs
 *         ->
 *     target-independent lowering
 *         ->
 *     target realization
 *
 * Metaprogramming therefore expands Zamani's expressive power without
 * fragmenting the language architecture.
 *
 * ============================================================================
 */
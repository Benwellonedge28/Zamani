/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/compile/code-generation.g4
 *
 * Grammar:
 *     CompileCodeGeneration
 *
 * Status:
 *     Production parser fragment
 *
 * Purpose:
 *     Defines source-level CODE-GENERATION INTENT.
 *
 * Architectural position:
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     canonical semantic IR
 *       |
 *       +--> optimization
 *       +--> routing
 *       +--> scheduling
 *       +--> hardware lowering
 *       +--> backend selection
 *       |
 *       v
 *     code generation
 *       |
 *       v
 *     executable / object / deployable artifact
 *
 * ============================================================================
 * FUNDAMENTAL RULE
 * ============================================================================
 *
 * THIS FILE DESCRIBES WHAT CODE GENERATION IS REQUESTED.
 *
 * IT DOES NOT GENERATE CODE.
 *
 * It does not:
 *
 *   - emit machine instructions;
 *   - select a CPU;
 *   - select a GPU;
 *   - select an FPGA;
 *   - select an ASIC;
 *   - select a QPU;
 *   - select a physical quantum device;
 *   - select a physical qubit;
 *   - allocate memory;
 *   - discover hardware;
 *   - perform optimization;
 *   - perform routing;
 *   - perform scheduling;
 *   - perform QEC;
 *   - define ZQN/noise semantics;
 *   - execute programs;
 *   - access files;
 *   - access networks;
 *   - execute host-language code.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Code-generation intent MUST remain separate from physical realization.
 *
 * In particular:
 *
 *     code-generation intent != machine code
 *     artifact kind != device
 *     target family != physical target
 *     backend preference != backend requirement
 *     ABI requirement != processor architecture
 *     calling convention != machine identity
 *     optimization preference != semantic transformation
 *
 * A source program therefore remains portable while downstream compilation
 * maps its canonical semantics to the resources actually available.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar contains NO fixed resource limits.
 *
 * It MUST NOT encode:
 *
 *     MAX_INSTRUCTIONS
 *     MAX_REGISTERS
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_ARTIFACTS
 *     MAX_SECTIONS
 *     MAX_SYMBOLS
 *     MAX_BACKENDS
 *     MAX_TARGETS
 *
 * Repetition and recursive grammar structures are used instead.
 *
 * Actual resource limits belong to:
 *
 *     - compilation context;
 *     - resource model;
 *     - hardware capability model;
 *     - backend;
 *     - scheduler;
 *     - runtime;
 *     - deployment environment.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This grammar contains no embedded Rust actions.
 *
 * Compiler implementations consuming this grammar MUST use:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * and MUST use safe Rust.
 *
 * `unsafe` is not required and is prohibited by the Zamani compiler contract.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - source-level code-generation intent;
 *   - artifact requests;
 *   - representation requests;
 *   - code-generation profiles;
 *   - emission policies;
 *   - symbol-emission intent;
 *   - linkage intent;
 *   - entry-point intent;
 *   - interface/ABI intent;
 *   - debug/provenance emission intent;
 *   - source-map intent;
 *   - section/layout intent at the semantic level;
 *   - backend-independent generation preferences;
 *   - generated-artifact relationships.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - target selection semantics;
 *   - resource requirements;
 *   - hardware description;
 *   - target discovery;
 *   - optimization;
 *   - scheduling;
 *   - routing;
 *   - quantum semantics;
 *   - quantum IR;
 *   - QEC;
 *   - ZQN;
 *   - runtime dispatch;
 *   - deployment;
 *   - machine instruction encodings;
 *   - linker implementation;
 *   - assembler implementation;
 *   - object-file implementation;
 *   - ABI implementation;
 *   - platform-specific binary formats.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * Canonical lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This parser fragment therefore uses:
 *
 *     tokenVocab = ZamaniLexer
 *
 * The canonical parser assembly integrates `codeGenerationDeclaration`
 * into the appropriate compilation/declaration surface.
 *
 * The grammar MUST NOT create a second lexer.
 *
 * ============================================================================
 *
 * Integration boundaries:
 *
 *     compile.g4
 *         general compilation intent
 *
 *     compile-time.g4
 *         compile-time evaluation
 *
 *     conditional-compilation.g4
 *         compile-time selection
 *
 *     feature-selection.g4
 *         feature selection
 *
 *     target.g4
 *         target intent
 *
 *     optimization.g4
 *         optimization intent
 *
 *     code-generation.g4
 *         code-generation intent
 *
 *     execution/*
 *         execution/deployment intent
 *
 *     interoperability/*
 *         foreign/ABI interoperability
 *
 *     hardware/*
 *         hardware semantics
 *
 *     quantum/*
 *         quantum semantics
 *
 *     canonical IR
 *         semantic representation
 *
 * ============================================================================
 *
 * IMPORTANT:
 *
 * This file does not import or redefine the canonical quantum IR.
 *
 * Quantum source syntax is lowered through:
 *
 *     frontend AST
 *         ->
 *     quantum semantic analysis
 *         ->
 *     quantum::ir
 *
 * Code generation consumes the resulting semantic representation downstream.
 *
 * ============================================================================
 */

parser grammar CompileCodeGeneration;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. TOP-LEVEL CODE-GENERATION DECLARATION
 * ============================================================================
 *
 * Canonical integration entry point.
 *
 * The surrounding parser owns placement/order.
 *
 * This rule owns only the syntax of a code-generation request.
 */
codeGenerationDeclaration
    : codeGenerationDirective
    | codeGenerationProfile
    | codeGenerationArtifact
    | codeGenerationOutput
    | codeGenerationEntryPoint
    | codeGenerationSymbolPolicy
    | codeGenerationLinkage
    | codeGenerationInterface
    | codeGenerationDebug
    | codeGenerationProvenance
    | codeGenerationLayout
    | codeGenerationEmission
    ;


/*
 * ============================================================================
 * 2. GENERIC CODE-GENERATION DIRECTIVE
 * ============================================================================
 *
 * Generic extensibility point.
 *
 * The directive identity remains semantic data rather than a finite list of
 * future compiler technologies.
 */
codeGenerationDirective
    : CODE identifier codeGenerationDirectiveBody?
    ;


codeGenerationDirectiveBody
    : ASSIGN expression
    | LPAREN codeGenerationArgumentList? RPAREN
    | LBRACE codeGenerationProperty* RBRACE
    ;


codeGenerationArgumentList
    : codeGenerationArgument
      (COMMA codeGenerationArgument)*
    ;


codeGenerationArgument
    : identifier ASSIGN expression
    | expression
    ;


/*
 * ============================================================================
 * 3. CODE-GENERATION PROFILE
 * ============================================================================
 *
 * A profile groups generation intent.
 *
 * It is NOT a machine configuration.
 *
 * It may be interpreted by the compiler/build system and lowered into a
 * target-independent compilation plan.
 */
codeGenerationProfile
    : GENERATIVE identifier
      LBRACE codeGenerationProfileEntry* RBRACE
    ;


codeGenerationProfileEntry
    : codeGenerationProperty
    | codeGenerationArtifact
    | codeGenerationOutput
    | codeGenerationSymbolPolicy
    | codeGenerationLinkage
    | codeGenerationInterface
    | codeGenerationDebug
    | codeGenerationProvenance
    | codeGenerationLayout
    | codeGenerationEmission
    ;


/*
 * ============================================================================
 * 4. ARTIFACT REQUEST
 * ============================================================================
 *
 * Describes WHAT representation is requested.
 *
 * Examples are intentionally semantic rather than machine-specific:
 *
 *     source
 *     ir
 *     object
 *     executable
 *     library
 *     package
 *     deployable
 *     firmware
 *     hardware-description
 *
 * The grammar does not enumerate these values so future artifact forms do not
 * require grammar changes.
 */
codeGenerationArtifact
    : codeGenerationArtifactKeyword
      codeGenerationArtifactSpecification
    ;


codeGenerationArtifactKeyword
    : CODE
    | SYNTHESIZE
    ;


codeGenerationArtifactSpecification
    : identifier
    | qualifiedIdentifier
    | STRING
    | codeGenerationArtifactBlock
    ;


codeGenerationArtifactBlock
    : LBRACE codeGenerationArtifactEntry* RBRACE
    ;


codeGenerationArtifactEntry
    : identifier ASSIGN expression SEMICOLON
    | identifier COLON expression SEMICOLON
    ;


/*
 * ============================================================================
 * 5. OUTPUT REQUEST
 * ============================================================================
 *
 * Describes the desired output relationship.
 *
 * Output identity remains semantic.
 *
 * A concrete filesystem path, deployment endpoint, or device address must
 * not become part of the permanent semantic model merely because one compiler
 * invocation has such a destination.
 */
codeGenerationOutput
    : identifier codeGenerationOutputSpecification
    ;


codeGenerationOutputSpecification
    : ARROW expression
    | ASSIGN expression
    | LBRACE codeGenerationOutputEntry* RBRACE
    ;


codeGenerationOutputEntry
    : identifier ASSIGN expression SEMICOLON
    | identifier COLON expression SEMICOLON
    ;


/*
 * ============================================================================
 * 6. ENTRY POINT
 * ============================================================================
 *
 * Entry-point intent identifies a semantic program entry.
 *
 * It does not encode:
 *
 *     - an operating-system ABI;
 *     - a processor address;
 *     - a fixed symbol name;
 *     - a linker implementation.
 */
codeGenerationEntryPoint
    : identifier identifier
      SEMICOLON?
    ;


codeGenerationEntryPointBlock
    : LBRACE codeGenerationEntryPointProperty* RBRACE
    ;


codeGenerationEntryPointProperty
    : identifier ASSIGN expression SEMICOLON
    | identifier COLON expression SEMICOLON
    ;


/*
 * ============================================================================
 * 7. SYMBOL EMISSION POLICY
 * ============================================================================
 *
 * Describes which semantic symbols should be emitted or retained.
 *
 * This is deliberately open-ended.
 */
codeGenerationSymbolPolicy
    : identifier codeGenerationSymbolPolicyBody
    ;


codeGenerationSymbolPolicyBody
    : expression
    | LBRACE codeGenerationSymbolPolicyEntry* RBRACE
    ;


codeGenerationSymbolPolicyEntry
    : identifier ASSIGN expression SEMICOLON
    | identifier COLON expression SEMICOLON
    ;


/*
 * ============================================================================
 * 8. LINKAGE INTENT
 * ============================================================================
 *
 * Linkage is represented semantically.
 *
 * Platform-specific linkage mechanisms are downstream concerns.
 */
codeGenerationLinkage
    : identifier codeGenerationLinkageBody
    ;


codeGenerationLinkageBody
    : expression
    | LBRACE codeGenerationLinkageEntry* RBRACE
    ;


codeGenerationLinkageEntry
    : identifier ASSIGN expression SEMICOLON
    | identifier COLON expression SEMICOLON
    ;


/*
 * ============================================================================
 * 9. INTERFACE / ABI INTENT
 * ============================================================================
 *
 * ABI requests describe interoperability requirements.
 *
 * They do not select a particular processor.
 *
 * They do not implement an ABI.
 *
 * They are consumed later by interoperability/backend lowering.
 */
codeGenerationInterface
    : identifier codeGenerationInterfaceBody
    ;


codeGenerationInterfaceBody
    : expression
    | LBRACE codeGenerationInterfaceEntry* RBRACE
    ;


codeGenerationInterfaceEntry
    : identifier ASSIGN expression SEMICOLON
    | identifier COLON expression SEMICOLON
    ;


/*
 * ============================================================================
 * 10. DEBUG INFORMATION
 * ============================================================================
 *
 * Debug intent may request preservation of source/semantic information.
 *
 * It must not affect program semantics.
 */
codeGenerationDebug
    : identifier codeGenerationDebugBody
    ;


codeGenerationDebugBody
    : expression
    | LBRACE codeGenerationDebugEntry* RBRACE
    ;


codeGenerationDebugEntry
    : identifier ASSIGN expression SEMICOLON
    | identifier COLON expression SEMICOLON
    ;


/*
 * ============================================================================
 * 11. PROVENANCE
 * ============================================================================
 *
 * Provenance is especially important for POCO-REAF and reproducible builds.
 *
 * The grammar describes WHAT provenance should be represented.
 *
 * It does not prescribe a particular hashing, signing, storage, or attestation
 * implementation.
 */
codeGenerationProvenance
    : identifier codeGenerationProvenanceBody
    ;


codeGenerationProvenanceBody
    : expression
    | LBRACE codeGenerationProvenanceEntry* RBRACE
    ;


codeGenerationProvenanceEntry
    : identifier ASSIGN expression SEMICOLON
    | identifier COLON expression SEMICOLON
    ;


/*
 * ============================================================================
 * 12. LAYOUT INTENT
 * ============================================================================
 *
 * Represents semantic output layout intent.
 *
 * This must NOT become a hard-coded physical memory map.
 *
 * Physical addresses, device registers, bus locations and machine-specific
 * section placement belong downstream to target/hardware lowering.
 */
codeGenerationLayout
    : identifier codeGenerationLayoutBody
    ;


codeGenerationLayoutBody
    : expression
    | LBRACE codeGenerationLayoutEntry* RBRACE
    ;


codeGenerationLayoutEntry
    : identifier ASSIGN expression SEMICOLON
    | identifier COLON expression SEMICOLON
    ;


/*
 * ============================================================================
 * 13. EMISSION POLICY
 * ============================================================================
 *
 * Describes whether/how semantic artifacts should be emitted.
 *
 * Examples of semantic concerns include:
 *
 *     deterministic
 *     reproducible
 *     debuggable
 *     inspectable
 *     portable
 *     canonical
 *
 * These remain identifiers rather than hard-coded finite enums.
 */
codeGenerationEmission
    : identifier codeGenerationEmissionBody
    ;


codeGenerationEmissionBody
    : expression
    | LBRACE codeGenerationEmissionEntry* RBRACE
    ;


codeGenerationEmissionEntry
    : identifier ASSIGN expression SEMICOLON
    | identifier COLON expression SEMICOLON
    ;


/*
 * ============================================================================
 * 14. GENERIC PROPERTY
 * ============================================================================
 *
 * Generic property syntax prevents this grammar from becoming a catalogue of
 * every backend technology that may exist in the future.
 */
codeGenerationProperty
    : identifier
      (
          ASSIGN expression
        | COLON expression
        | expression
      )
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 15. SYMBOL / ARTIFACT COLLECTIONS
 * ============================================================================
 *
 * No fixed cardinality is encoded.
 */
codeGenerationIdentifierList
    : identifier
      (COMMA identifier)*
    ;


codeGenerationExpressionList
    : expression
      (COMMA expression)*
    ;


/*
 * ============================================================================
 * 16. QUALIFIED IDENTIFIER
 * ============================================================================
 *
 * This is a local parser contract only.
 *
 * If the canonical parser assembly already provides an equivalent shared
 * `qualifiedIdentifier`, it MUST use that canonical rule rather than retain a
 * duplicate semantic definition.
 *
 * This fragment deliberately keeps its own adapter rule so it can be compiled
 * independently by ANTLR during grammar validation.
 */
qualifiedIdentifier
    : identifier
      (
          DCOLON identifier
        | DOT identifier
      )*
    ;


identifier
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * 17. CODE-GENERATION PROPERTY EXPRESSION
 * ============================================================================
 *
 * This adapter keeps code-generation grammar independent of the expression
 * implementation while permitting integration with the canonical expression
 * grammar.
 *
 * Canonical parser assembly should bind `expression` to the repository's
 * authoritative expression rule.
 *
 * This file MUST NOT create a second expression language.
 */


/*
 * ============================================================================
 * 18. INTEGRATION CONTRACT — AST
 * ============================================================================
 *
 * The parser AST produced from this file MUST preserve:
 *
 *   - source span;
 *   - declaration kind;
 *   - property names;
 *   - expressions;
 *   - ordering where semantically relevant;
 *   - source-level identifiers;
 *   - explicit-vs-default intent;
 *   - provenance metadata supplied by the frontend.
 *
 * The AST MUST NOT contain:
 *
 *   - generated machine instructions;
 *   - physical device handles;
 *   - hardware addresses;
 *   - mutable runtime state;
 *   - backend-owned compiler objects.
 *
 * ============================================================================
 * 19. INTEGRATION CONTRACT — SEMANTIC ANALYSIS
 * ============================================================================
 *
 * Semantic analysis validates:
 *
 *   - artifact compatibility;
 *   - output compatibility;
 *   - entry-point validity;
 *   - symbol-policy validity;
 *   - linkage semantics;
 *   - interface/ABI compatibility;
 *   - provenance requirements;
 *   - debug/provenance interactions;
 *   - layout legality;
 *   - emission-policy legality;
 *   - target compatibility;
 *   - capability requirements.
 *
 * Invalid combinations MUST produce structured diagnostics.
 *
 * The grammar must never encode semantic validation as parser actions.
 *
 * ============================================================================
 * 20. INTEGRATION CONTRACT — IR
 * ============================================================================
 *
 * This grammar lowers indirectly:
 *
 *     code-generation syntax
 *         ->
 *     frontend AST
 *         ->
 *     semantic model
 *         ->
 *     canonical compilation representation
 *         ->
 *     backend/code-generation planning
 *
 * It MUST NOT introduce:
 *
 *     CodeGenerationIR
 *
 * merely to duplicate the canonical compiler representation.
 *
 * Where the repository already has a canonical compilation/backend IR, this
 * syntax lowers into that existing representation.
 *
 * For quantum programs:
 *
 *     quantum syntax
 *         ->
 *     frontend AST
 *         ->
 *     quantum semantic analysis
 *         ->
 *     quantum::ir
 *
 * remains authoritative.
 *
 * ============================================================================
 * 21. INTEGRATION CONTRACT — OPTIMIZATION
 * ============================================================================
 *
 * Optimization consumes semantic IR.
 *
 * Code-generation intent may provide:
 *
 *   - optimization preferences;
 *   - emission requirements;
 *   - artifact requirements.
 *
 * It must not invoke optimization algorithms.
 *
 * `grammar/compile/optimization.g4` owns optimization intent.
 *
 * ============================================================================
 * 22. INTEGRATION CONTRACT — TARGET
 * ============================================================================
 *
 * Target selection remains owned by:
 *
 *     grammar/compile/target.g4
 *
 * Code generation may reference target intent but MUST NOT redefine target
 * semantics.
 *
 * Target resolution happens downstream.
 *
 * ============================================================================
 * 23. INTEGRATION CONTRACT — HARDWARE
 * ============================================================================
 *
 * Hardware-specific implementation belongs to:
 *
 *     grammar/hardware/*
 *
 * and downstream hardware abstraction/lowering.
 *
 * This grammar MUST NOT define:
 *
 *     CPU register counts
 *     GPU counts
 *     FPGA resources
 *     ASIC cell counts
 *     QPU topology
 *     qubit count limits
 *     physical addresses
 *     memory maps
 *
 * ============================================================================
 * 24. INTEGRATION CONTRACT — QUANTUM
 * ============================================================================
 *
 * Quantum compilation requests may refer to quantum semantic artifacts, but
 * this file MUST NOT define:
 *
 *     qubits
 *     gates
 *     circuits
 *     measurements
 *     QEC algorithms
 *     noise models
 *     physical topology
 *
 * Those remain owned by the quantum grammar/domain and downstream IR.
 *
 * `quantum::ir` remains the canonical semantic boundary.
 *
 * ============================================================================
 * 25. INTEGRATION CONTRACT — RUNTIME
 * ============================================================================
 *
 * Runtime consumes compiled/deployable artifacts.
 *
 * Runtime dispatch MUST NOT depend directly on this parser grammar.
 *
 * The normal dependency direction is:
 *
 *     grammar
 *       ->
 *     AST
 *       ->
 *     semantic model
 *       ->
 *     IR / compilation plan
 *       ->
 *     generated artifact
 *       ->
 *     runtime
 *
 * ============================================================================
 * 26. INTEGRATION CONTRACT — INTEROPERABILITY
 * ============================================================================
 *
 * ABI, FFI, C/C++, Python, OpenQASM, Verilog and other interoperability
 * mechanisms are resolved by their respective interoperability layers.
 *
 * This grammar may express intent to expose or consume such an interface but
 * does not implement it.
 *
 * ============================================================================
 * 27. DETERMINISM
 * ============================================================================
 *
 * Parsing the same source with the same grammar version MUST produce the same
 * parse structure.
 *
 * This file contains:
 *
 *   - no semantic actions;
 *   - no random behavior;
 *   - no time-dependent behavior;
 *   - no environment-dependent behavior;
 *   - no filesystem access;
 *   - no network access.
 *
 * ============================================================================
 * 28. COMPATIBILITY
 * ============================================================================
 *
 * New artifact kinds, profiles, policies and semantic property names SHOULD
 * normally be introduced as identifier-level extensions.
 *
 * Existing syntax MUST NOT silently change meaning.
 *
 * Breaking syntax changes require a language-version/compatibility decision.
 *
 * ============================================================================
 * 29. HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden in this file:
 *
 *   - finite architecture lists used as the authoritative universe;
 *   - fixed resource counts;
 *   - fixed register counts;
 *   - fixed instruction sets;
 *   - fixed memory sizes;
 *   - fixed device identifiers;
 *   - physical addresses;
 *   - fixed topology;
 *   - fixed quantum dimensions;
 *   - fixed accelerator counts;
 *   - fixed deployment sizes.
 *
 * Open-ended identifiers and repeated grammar constructs are intentional.
 *
 * ============================================================================
 * 30. TEST CONTRACT
 * ============================================================================
 *
 * Positive tests MUST cover:
 *
 *   - artifact requests;
 *   - output requests;
 *   - profiles;
 *   - entry points;
 *   - symbol policies;
 *   - linkage;
 *   - interfaces;
 *   - debug information;
 *   - provenance;
 *   - layout intent;
 *   - emission policies;
 *   - generic directives;
 *   - nested properties;
 *   - arbitrarily long lists.
 *
 * Negative tests MUST cover:
 *
 *   - malformed artifact declarations;
 *   - malformed output declarations;
 *   - malformed profiles;
 *   - malformed property blocks;
 *   - invalid separators;
 *   - incomplete expressions;
 *   - unterminated blocks;
 *   - invalid qualified identifiers.
 *
 * Boundary/scalability tests MUST verify that the grammar itself imposes no
 * artificial limits on:
 *
 *   - number of artifacts;
 *   - number of symbols;
 *   - number of properties;
 *   - number of profiles;
 *   - number of interfaces;
 *   - expression size;
 *   - program size.
 *
 * Cross-domain tests MUST include combinations involving:
 *
 *   classical
 *   quantum
 *   hybrid
 *   HDL
 *   hardware
 *   distributed
 *   AI
 *   accelerators
 *
 * ============================================================================
 * 31. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when:
 *
 *   1. It compiles as an ANTLR parser fragment with ZamaniLexer.
 *
 *   2. It contains no embedded host-language actions.
 *
 *   3. It contains no unsafe implementation.
 *
 *   4. It introduces no machine-size limits.
 *
 *   5. It does not redefine quantum IR.
 *
 *   6. It does not duplicate target semantics.
 *
 *   7. It does not duplicate optimization semantics.
 *
 *   8. It does not perform code generation.
 *
 *   9. Its AST contract is stable.
 *
 *  10. Its integration point with compile.g4 is explicit.
 *
 *  11. Its integration point with target.g4 is explicit.
 *
 *  12. Its integration point with optimization.g4 is explicit.
 *
 *  13. Its integration with canonical expressions/identifiers is explicit.
 *
 *  14. Positive, negative and scalability tests pass.
 *
 *  15. The same source remains semantically independent of the eventual
 *      physical machine.
 *
 * ============================================================================
 */
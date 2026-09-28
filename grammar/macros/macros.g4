/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/macros/macros.g4
 *
 * GRAMMAR IDENTITY
 * ----------------
 * Macros
 *
 * STATUS
 * ------
 * CANONICAL MACRO-SUBSYSTEM COMPOSITION ROOT
 *
 * LANGUAGE
 * --------
 * Zamani
 *
 * GRAMMAR TECHNOLOGY
 * ------------------
 * ANTLR4 parser grammar
 *
 * COMPILER BASELINE
 * -----------------
 * Rust 1.97 / Rust 1.97.1
 * Rust 2021
 *
 * SAFETY
 * ------
 * Safe Rust only.
 *
 * This grammar:
 *
 *   - contains no embedded Rust actions;
 *   - contains no target-language actions;
 *   - contains no semantic predicates;
 *   - performs no I/O;
 *   - performs no filesystem access;
 *   - performs no network access;
 *   - performs no process execution;
 *   - performs no hardware discovery;
 *   - performs no runtime execution;
 *   - requires no unsafe Rust.
 *
 * PORTABILITY OBJECTIVE
 * ---------------------
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Abbreviation:
 *
 *   POCO-REAF
 *
 * SCALABILITY OBJECTIVE
 * ---------------------
 * The macro language has no language-defined finite maximum for:
 *
 *   - macro declarations;
 *   - macro parameters;
 *   - macro arguments;
 *   - namespace/path depth;
 *   - token-tree size;
 *   - token-tree nesting;
 *   - macro body size;
 *   - annotations;
 *   - expansion result size;
 *   - source-unit size;
 *   - generated declarations;
 *   - generated expressions;
 *   - generated statements;
 *   - generated domain constructs.
 *
 * Actual compiler/resource limits are implementation admission policies and
 * MUST NOT be encoded here as language semantics.
 *
 * ============================================================================
 * 1. PURPOSE
 * ============================================================================
 *
 * This file is the SINGLE composition boundary for the Zamani macro grammar.
 *
 * It composes independently owned macro grammar components without copying
 * their productions.
 *
 * The component ownership model is:
 *
 *   MacroDeclarations
 *       -> macro declaration syntax
 *
 *   MacroParameters
 *       -> macro parameter syntax
 *
 *   invocations
 *       -> macro paths
 *       -> macro invocations
 *       -> macro expressions
 *
 *   expansion
 *       -> expansion-related source metadata
 *
 *   hygiene
 *       -> hygiene-related source metadata
 *
 *   diagnostics
 *       -> macro diagnostic metadata
 *
 *   syntaxTree
 *       -> syntax-tree/token-tree integration
 *
 * The composition root itself owns only the macro-subsystem dispatch rule:
 *
 *   macroElement
 *
 * It MUST NOT redefine the productions owned by the imported components.
 *
 * ============================================================================
 * 2. ARCHITECTURAL POSITION
 * ============================================================================
 *
 * The macro subsystem participates in the normal Zamani compilation pipeline:
 *
 *
 *                         Zamani source
 *                              |
 *                              v
 *                     canonical Zamani lexer
 *                              |
 *                              v
 *                     canonical Zamani parser
 *                              |
 *                              v
 *                         macro syntax
 *                              |
 *              +---------------+----------------+
 *              |               |                |
 *              v               v                v
 *        declarations     invocations       metadata
 *              |               |                |
 *              |               |       +--------+--------+
 *              |               |       |        |        |
 *              |               |       v        v        v
 *              |               |   expansion hygiene diagnostics
 *              |               |
 *              +---------------+----------------+
 *                              |
 *                              v
 *                       frontend AST
 *                              |
 *                              v
 *                       macro resolution
 *                              |
 *                              v
 *                    controlled macro expansion
 *                              |
 *                              v
 *                    hygiene / provenance
 *                              |
 *                              v
 *                   structural + semantic analysis
 *                              |
 *                              v
 *                    canonical semantic model
 *                              |
 *             +----------------+----------------+
 *             |                |                |
 *             v                v                v
 *        classical        quantum::ir       HDL/hardware
 *             |                |                |
 *             +----------------+----------------+
 *                              |
 *                              v
 *                    optimization / lowering
 *                              |
 *                     routing / scheduling
 *                              |
 *                   resilience / QEC / ZQN
 *                              |
 *                             HAL
 *                              |
 *                       target realization
 *
 *
 * Macro expansion is therefore a compiler/frontend operation.
 *
 * It is NOT an ANTLR parser action.
 *
 * ============================================================================
 * 3. SINGLE-AUTHORITY CONTRACT
 * ============================================================================
 *
 * Every macro production has exactly one owner.
 *
 * ---------------------------------------------------------------------------
 * MACRO DECLARATIONS
 * ---------------------------------------------------------------------------
 *
 * Owner:
 *
 *   MacroDeclarations
 *
 * Canonical productions:
 *
 *   macroDeclaration
 *
 * Parameter productions are intended to be owned by:
 *
 *   MacroParameters
 *
 * Canonical productions:
 *
 *   macroParameterList
 *   macroParameter
 *   macroParameterDefault
 *
 * ---------------------------------------------------------------------------
 * MACRO INVOCATIONS
 * ---------------------------------------------------------------------------
 *
 * Owner:
 *
 *   invocations
 *
 * Canonical productions:
 *
 *   macroPath
 *   macroInvocation
 *   macroExpression
 *
 * ---------------------------------------------------------------------------
 * EXPANSION METADATA
 * ---------------------------------------------------------------------------
 *
 * Owner:
 *
 *   expansion
 *
 * ---------------------------------------------------------------------------
 * HYGIENE METADATA
 * ---------------------------------------------------------------------------
 *
 * Owner:
 *
 *   hygiene
 *
 * ---------------------------------------------------------------------------
 * DIAGNOSTIC METADATA
 * ---------------------------------------------------------------------------
 *
 * Owner:
 *
 *   diagnostics
 *
 * ---------------------------------------------------------------------------
 * TOKEN-TREE / SYNTAX-TREE INTEGRATION
 * ---------------------------------------------------------------------------
 *
 * Owner:
 *
 *   syntaxTree
 *
 * The syntaxTree component is the composition path to the canonical
 * token-stream/token-tree representation.
 *
 * ---------------------------------------------------------------------------
 * MACRO COMPOSITION
 * ---------------------------------------------------------------------------
 *
 * Owner:
 *
 *   Macros
 *
 * This file owns only:
 *
 *   macroElement
 *
 * No other macro production is duplicated here.
 *
 * ============================================================================
 * 4. NO SPECULATIVE macroStatement
 * ============================================================================
 *
 * A previous design referenced a production named:
 *
 *   macroStatement
 *
 * The inspected repository does not provide a canonical owner for that rule.
 *
 * This file therefore deliberately DOES NOT invent:
 *
 *   macroStatement
 *
 * Macro syntax currently enters the universal parser through:
 *
 *   macroDeclaration
 *   macroExpression
 *
 * If Zamani later requires a dedicated macro statement form, that feature
 * MUST first receive:
 *
 *   specification
 *   AST contract
 *   semantic contract
 *   grammar owner
 *   integration contract
 *   conformance tests
 *
 * before being added here.
 *
 * This prevents speculative syntax from becoming part of the language merely
 * because a composition root references it.
 *
 * ============================================================================
 * 5. LEXER CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * The canonical production lexer is:
 *
 *   grammar/antlr/ZamaniLexer.g4
 *
 * The parser vocabulary is therefore:
 *
 *   ZamaniLexer
 *
 * This file MUST NOT:
 *
 *   - define lexer rules;
 *   - define token aliases;
 *   - redefine identifiers;
 *   - redefine keywords;
 *   - redefine punctuation;
 *   - redefine operators;
 *   - redefine delimiters;
 *   - introduce macro-specific lexer modes;
 *   - introduce macro-specific lexer channels.
 *
 * In particular, this file MUST NOT define tokens such as:
 *
 *   MACRO_DECLARATION
 *   MACRO_INVOCATION
 *   HYGIENE
 *   CAPTURE
 *   FRESH
 *   DEF_SITE
 *   CALL_SITE
 *   TOKEN_TREE
 *
 * unless the canonical lexical specification independently establishes them.
 *
 * Macro semantics are not lexical token identity.
 *
 * ============================================================================
 * 6. SHARED GRAMMAR CONTRACT
 * ============================================================================
 *
 * Imported macro components may consume canonical rules supplied by the
 * broader Zamani parser composition.
 *
 * Examples include:
 *
 *   identifier
 *   qualifiedName
 *   visibilityModifier
 *   genericParameters
 *   typeExpression
 *   expression
 *   argumentList
 *   blockExpression
 *   annotation
 *
 * These rules are NOT redefined here.
 *
 * Their ownership remains with their respective canonical grammar components.
 *
 * This composition root must remain a composition layer rather than becoming
 * a second shared-language grammar.
 *
 * ============================================================================
 * 7. ANTLR IMPORT CONTRACT
 * ============================================================================
 *
 * ANTLR grammar imports are grammar-identity imports.
 *
 * Therefore this file imports canonical grammar identities rather than
 * filesystem paths.
 *
 * The intended component identities are:
 *
 *   MacroDeclarations
 *   invocations
 *   expansion
 *   hygiene
 *   diagnostics
 *   syntaxTree
 *
 * The repository's canonical parser build is responsible for placing all
 * imported grammars in the ANTLR grammar source path.
 *
 * This file MUST NOT use imports such as:
 *
 *   import grammar/macros/declarations;
 *   import grammar.macros.declarations;
 *   import MacroDeclarations.g4;
 *
 * The grammar identity is the integration contract.
 *
 * ============================================================================
 * 8. PARAMETER OWNERSHIP INTEGRATION
 * ============================================================================
 *
 * The repository contains:
 *
 *   grammar/macros/parameters.g4
 *
 * with grammar identity:
 *
 *   MacroParameters
 *
 * MacroParameters is intended to own:
 *
 *   macroParameterList
 *   macroParameter
 *   macroParameterDefault
 *
 * The declaration component should consume those canonical productions
 * through its own import/composition contract.
 *
 * This composition root therefore MUST NOT import MacroParameters directly
 * when MacroDeclarations already provides that composition path.
 *
 * The intended dependency direction is:
 *
 *   MacroParameters
 *          |
 *          v
 *   MacroDeclarations
 *          |
 *          v
 *       Macros
 *
 * This avoids creating two independent import paths for the same productions.
 *
 * IMPORTANT:
 *
 * If MacroDeclarations currently contains duplicate local definitions of
 * parameter productions, that duplication must be removed in
 * MacroDeclarations itself.
 *
 * It is intentionally NOT "fixed" by redefining the rules here.
 *
 * ============================================================================
 * 9. TOKEN-TREE INTEGRATION
 * ============================================================================
 *
 * The repository contains:
 *
 *   grammar/macros/token-stream.g4
 *
 * and:
 *
 *   grammar/macros/syntax-tree.g4
 *
 * The token-stream grammar owns the canonical balanced token-tree structure.
 *
 * The syntax-tree grammar provides the macro-facing composition/adaptation
 * boundary.
 *
 * Therefore this root imports:
 *
 *   syntaxTree
 *
 * rather than independently importing tokenStream.
 *
 * Intended dependency:
 *
 *   tokenStream
 *        |
 *        v
 *   syntaxTree
 *        |
 *        v
 *      Macros
 *
 * This prevents multiple macro composition paths from independently exposing
 * the same token-tree productions.
 *
 * ============================================================================
 * 10. DOMAIN NEUTRALITY
 * ============================================================================
 *
 * Macros are not domain-specific.
 *
 * The same macro mechanism may generate source for:
 *
 *   classical computing
 *   systems programming
 *   scientific computing
 *   numerical computing
 *   symbolic computing
 *   vector/matrix/tensor computing
 *   quantum computing
 *   hybrid quantum-classical computing
 *   HDL
 *   hardware/software co-design
 *   FPGA/ASIC intent
 *   accelerator computation
 *   distributed computing
 *   HPC
 *   AI/ML
 *   data processing
 *   networking
 *   security
 *   cryptography
 *   embedded computing
 *   temporal computation
 *   nano-oriented computation
 *   future Zamani domains
 *
 * This file MUST NOT introduce domain-specific macro categories such as:
 *
 *   quantumMacro
 *   classicalMacro
 *   gpuMacro
 *   qpuMacro
 *   fpgaMacro
 *   hdlMacro
 *   hardwareMacro
 *   aiMacro
 *   distributedMacro
 *
 * merely to distinguish generated source.
 *
 * Generated source remains ordinary Zamani source.
 *
 * ============================================================================
 * 11. QUANTUM INTEGRATION
 * ============================================================================
 *
 * A macro may generate quantum source.
 *
 * This grammar does NOT:
 *
 *   - enumerate gates;
 *   - enumerate physical qubits;
 *   - allocate physical qubits;
 *   - select a QPU;
 *   - select a native gate set;
 *   - select topology;
 *   - perform routing;
 *   - perform scheduling;
 *   - perform calibration;
 *   - perform QEC;
 *   - implement ZQN;
 *   - construct quantum::ir.
 *
 * The canonical downstream path remains:
 *
 *   macro expansion
 *        |
 *        v
 *   semantic analysis
 *        |
 *        v
 *   quantum::ir
 *        |
 *        v
 *   optimization
 *        |
 *        v
 *   routing / scheduling
 *        |
 *        v
 *   resilience / QEC / ZQN
 *        |
 *        v
 *   HAL
 *        |
 *        v
 *   target realization
 *
 * Macro expansion must therefore not create a second quantum IR.
 *
 * ============================================================================
 * 12. HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * A macro may generate HDL or hardware-intent constructs.
 *
 * This file does not encode:
 *
 *   - register widths;
 *   - bus widths;
 *   - memory sizes;
 *   - device counts;
 *   - FPGA capacities;
 *   - ASIC dimensions;
 *   - clock counts;
 *   - pipeline limits;
 *   - physical pin counts;
 *   - physical topology;
 *   - implementation-specific resources.
 *
 * Such information belongs to the appropriate downstream semantic,
 * resource, hardware, compilation, and backend layers.
 *
 * A macro is therefore allowed to generate parameterized hardware intent
 * without coupling the macro grammar to a particular implementation.
 *
 * ============================================================================
 * 13. RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Macro syntax does not grant resources or capabilities.
 *
 * The presence of a macro invocation MUST NOT itself imply:
 *
 *   - CPU availability;
 *   - GPU availability;
 *   - FPGA availability;
 *   - QPU availability;
 *   - qubit availability;
 *   - memory availability;
 *   - network availability;
 *   - filesystem access;
 *   - process execution;
 *   - privileged operations;
 *   - cryptographic secrets;
 *   - compiler authority.
 *
 * Resource and capability requirements are semantic contracts.
 *
 * For example, generated source may eventually contain constructs equivalent
 * to:
 *
 *   requires qubits >= n
 *
 * or:
 *
 *   requires capability("quantum.measurement")
 *
 * The macro parser does not evaluate those requirements.
 *
 * The compiler's semantic/resource/capability layers do so later.
 *
 * ============================================================================
 * 14. MACRO EXPANSION BOUNDARY
 * ============================================================================
 *
 * This grammar defines source structure only.
 *
 * Expansion itself belongs downstream.
 *
 * The expansion implementation is responsible for concerns such as:
 *
 *   - macro registration;
 *   - macro resolution;
 *   - argument binding;
 *   - expansion ordering;
 *   - recursion detection;
 *   - expansion provenance;
 *   - controlled expansion;
 *   - generated-source validation;
 *   - configurable compiler admission policies.
 *
 * The grammar MUST NOT encode:
 *
 *   max_expansion_depth
 *   max_expansion_size
 *
 * as language syntax.
 *
 * If the compiler has such safety/admission policies, they are runtime/build
 * configuration rather than language semantics.
 *
 * ============================================================================
 * 15. HYGIENE BOUNDARY
 * ============================================================================
 *
 * Hygiene is a semantic/compiler concern.
 *
 * The grammar may preserve explicit hygiene metadata through the canonical
 * hygiene component.
 *
 * The parser does NOT:
 *
 *   - create binding identities;
 *   - resolve identifiers;
 *   - generate fresh symbols;
 *   - determine captures;
 *   - create lexical scopes;
 *   - bypass visibility;
 *   - bypass ownership;
 *   - bypass type checking;
 *   - bypass effect checking;
 *   - bypass capability checking;
 *   - bypass resource checking;
 *   - authorize privileged access.
 *
 * Hygiene and provenance must survive through the frontend AST and controlled
 * expansion pipeline.
 *
 * ============================================================================
 * 16. PROVENANCE BOUNDARY
 * ============================================================================
 *
 * Macro-generated constructs must remain diagnosable.
 *
 * Downstream representations should therefore preserve enough source
 * provenance to distinguish, where supported by the compiler:
 *
 *   - macro definition source;
 *   - invocation source;
 *   - argument source;
 *   - generated source;
 *   - expansion nesting/context.
 *
 * This grammar does not implement source maps.
 *
 * It merely ensures that the parser does not destroy the source structure
 * needed by the downstream provenance system.
 *
 * ============================================================================
 * 17. AST INTEGRATION
 * ============================================================================
 *
 * Macro syntax must map into the existing domain-neutral frontend AST.
 *
 * The repository already contains a canonical macro expression AST:
 *
 *   src/frontend/ast/node/expressions/macro.rs
 *
 * That representation models a macro invocation structurally rather than
 * pretending that the macro has already executed.
 *
 * The intended conceptual mapping is:
 *
 *   macroExpression
 *          |
 *          v
 *   MacroExpression AST node
 *          |
 *          v
 *   macro resolution
 *          |
 *          v
 *   controlled expansion
 *          |
 *          v
 *   ordinary Zamani AST / semantic model
 *
 * The grammar MUST NOT create:
 *
 *   - a second MacroExpression AST;
 *   - a macro-specific quantum AST;
 *   - a macro-specific HDL AST;
 *   - a macro-specific hardware AST;
 *   - a second universal IR.
 *
 * ============================================================================
 * 18. AST SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * Macro declarations, invocations, metadata, and token-tree structures must
 * remain traceable to source locations.
 *
 * The grammar must therefore preserve parser contexts required by the
 * frontend's source-span/lowering infrastructure.
 *
 * Source-span implementation remains outside this grammar.
 *
 * The grammar MUST NOT replace source structure with opaque parser actions.
 *
 * ============================================================================
 * 19. CANONICAL IR CONTRACT
 * ============================================================================
 *
 * Macro syntax is not an IR.
 *
 * After expansion and semantic validation, generated source participates in
 * the ordinary canonical semantic pipeline.
 *
 * Conceptually:
 *
 *   macro source
 *        |
 *        v
 *   macro AST
 *        |
 *        v
 *   controlled expansion
 *        |
 *        v
 *   domain-neutral AST
 *        |
 *        v
 *   semantic analysis
 *        |
 *        v
 *   canonical semantic model
 *        |
 *        +----------------------+----------------------+
 *        |                      |                      |
 *        v                      v                      v
 *   classical IR          quantum::ir          HDL/hardware
 *        |                      |                      |
 *        +----------------------+----------------------+
 *                               |
 *                               v
 *                     optimization / lowering
 *
 * The macro subsystem MUST NOT introduce a second IR boundary.
 *
 * ============================================================================
 * 20. POCO-REAF CONTRACT
 * ============================================================================
 *
 * Macro syntax must remain target-independent.
 *
 * The same macro source may generate portable source constructs that can
 * subsequently be compiled for:
 *
 *   tiny embedded targets;
 *   CPUs;
 *   multicore systems;
 *   GPUs;
 *   FPGAs;
 *   ASIC-oriented flows;
 *   accelerators;
 *   quantum simulators;
 *   quantum processors;
 *   HPC systems;
 *   clusters;
 *   distributed systems;
 *   cloud environments;
 *   future computational targets.
 *
 * This does NOT mean every target can satisfy every program requirement.
 *
 * Target feasibility remains a downstream resource/capability question.
 *
 * The macro grammar must never solve target mismatch by changing source
 * semantics.
 *
 * ============================================================================
 * 21. HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This file contains no language-level hardware limits.
 *
 * In particular, it MUST NOT encode any of:
 *
 *   MAX_QUBITS
 *   MAX_CPUS
 *   MAX_GPUS
 *   MAX_FPGAS
 *   MAX_NODES
 *   MAX_MEMORY
 *   MAX_THREADS
 *   MAX_TENSOR_RANK
 *   MAX_REGISTER_WIDTH
 *   MAX_NETWORK_SIZE
 *   MAX_DEVICE_COUNT
 *
 * Nor may it encode equivalent universal limits through disguised grammar
 * alternatives.
 *
 * Examples of prohibited architecture include:
 *
 *   qubit0
 *   qubit1
 *   qubit2
 *
 * as the complete language resource model, or:
 *
 *   macroCount <= fixedValue
 *
 * as a grammar restriction.
 *
 * Ordinary numeric literals remain valid program data.
 *
 * The prohibition concerns implementation ceilings masquerading as language
 * semantics.
 *
 * ============================================================================
 * 22. SCALABILITY CONTRACT
 * ============================================================================
 *
 * Grammar repetition constructs such as:
 *
 *   *
 *   +
 *
 * represent unbounded language structure from the language specification's
 * perspective.
 *
 * This file deliberately does not introduce finite bounds for:
 *
 *   macro declarations;
 *   parameters;
 *   arguments;
 *   path segments;
 *   metadata;
 *   token trees;
 *   nested groups;
 *   expansion output.
 *
 * The implementation may still require configurable admission controls for
 * hostile or resource-exhausting input.
 *
 * Those controls MUST be:
 *
 *   - implementation-level;
 *   - configurable where appropriate;
 *   - observable;
 *   - diagnosable;
 *   - independent of source-language meaning;
 *   - free of hardware-specific assumptions.
 *
 * ============================================================================
 * 23. DETERMINISM CONTRACT
 * ============================================================================
 *
 * For:
 *
 *   identical source;
 *   identical lexer vocabulary;
 *   identical grammar version;
 *   identical parser configuration;
 *
 * parsing must produce structurally equivalent parse results.
 *
 * This grammar contains no:
 *
 *   - randomness;
 *   - wall-clock dependency;
 *   - environment inspection;
 *   - filesystem ordering;
 *   - network state;
 *   - hardware state;
 *   - backend availability dependency.
 *
 * ============================================================================
 * 24. ERROR-RECOVERY CONTRACT
 * ============================================================================
 *
 * This composition root must not hide malformed macro syntax.
 *
 * Errors such as:
 *
 *   malformed macro declarations;
 *   malformed invocations;
 *   malformed paths;
 *   malformed delimiter groups;
 *   invalid component composition;
 *
 * must remain visible to the parser diagnostic system.
 *
 * Error recovery belongs to the canonical parser infrastructure.
 *
 * This file does not add semantic actions merely to recover from malformed
 * source.
 *
 * ============================================================================
 * 25. CROSS-DOMAIN INTEGRATION
 * ============================================================================
 *
 * Macro-generated source must be able to re-enter the ordinary Zamani grammar
 * for all supported domains.
 *
 * Required integration coverage includes generated forms for:
 *
 *   classical;
 *   quantum;
 *   hybrid;
 *   HDL;
 *   hardware;
 *   distributed;
 *   AI;
 *   data;
 *   networking;
 *   security;
 *   interoperability;
 *   future domain extensions.
 *
 * The macro grammar itself remains unchanged when a new downstream domain is
 * added.
 *
 * This is a key extensibility invariant.
 *
 * Adding a new target or domain should not require creating:
 *
 *   new macro syntax;
 *   new macro invocation syntax;
 *   new macro parser;
 *   new macro IR.
 *
 * The new domain integrates into the ordinary Zamani language pipeline.
 *
 * ============================================================================
 * 26. INTEROPERABILITY CONTRACT
 * ============================================================================
 *
 * Macro-generated source may ultimately participate in interoperability with:
 *
 *   foreign languages;
 *   foreign ABIs;
 *   OpenQASM;
 *   QIR;
 *   HDL formats;
 *   WebAssembly;
 *   other Zamani dialects;
 *   future interchange representations.
 *
 * Such formats remain downstream interoperability concerns.
 *
 * This grammar does not make any external representation the canonical
 * Zamani semantic representation.
 *
 * In particular:
 *
 *   OpenQASM != Zamani AST
 *   QIR       != Zamani AST
 *   HDL       != Zamani AST
 *
 * Quantum lowering continues through:
 *
 *   quantum::ir
 *
 * ============================================================================
 * 27. MACRO SAFETY CONTRACT
 * ============================================================================
 *
 * Parsing a macro invocation MUST NOT itself execute anything.
 *
 * In particular, parser construction of:
 *
 *   foo!(...)
 *
 * must not:
 *
 *   execute foo;
 *   inspect the filesystem;
 *   access the network;
 *   invoke a process;
 *   inspect hardware;
 *   allocate physical resources;
 *   allocate physical qubits;
 *   open devices;
 *   access secrets;
 *   change compiler configuration.
 *
 * Parsing produces structure.
 *
 * Execution belongs downstream.
 *
 * ============================================================================
 * 28. MACRO ELEMENT DISPATCH
 * ============================================================================
 *
 * This is the ONLY production owned directly by this composition root.
 *
 * A macro element may be:
 *
 *   - a macro declaration;
 *   - a macro expression/invocation.
 *
 * The actual declaration and invocation syntax is delegated to their
 * respective owners.
 *
 * ============================================================================
 */

macroElement
    : macroDeclaration
    | macroExpression
    ;


/*
 * ============================================================================
 * 29. COMPOSITION IMPORTS
 * ============================================================================
 *
 * These imports intentionally occur after the architectural contract so the
 * ownership relationship is explicit and auditable.
 *
 * IMPORTANT:
 *
 *   MacroDeclarations
 *       is responsible for composing MacroParameters where required.
 *
 * Therefore MacroParameters is not independently imported here.
 *
 * Likewise:
 *
 *   syntaxTree
 *       is responsible for reaching tokenStream.
 *
 * Therefore tokenStream is not independently imported here.
 *
 * This keeps the dependency graph single-path and avoids duplicate grammar
 * ownership.
 * ============================================================================
 */

parser grammar Macros;

options {
    tokenVocab = ZamaniLexer;
}

import
    MacroDeclarations,
    invocations,
    expansion,
    hygiene,
    diagnostics,
    syntaxTree
;


/*
 * ============================================================================
 * 30. INTEGRATION CHECKLIST
 * ============================================================================
 *
 * This file is considered compositionally complete only when all of the
 * following repository contracts are satisfied.
 *
 * CANONICAL COMPOSITION
 *
 * [x] Grammar identity is Macros.
 * [x] Canonical token vocabulary is ZamaniLexer.
 * [x] MacroDeclarations is imported exactly once.
 * [x] invocations is imported exactly once.
 * [x] expansion is imported exactly once.
 * [x] hygiene is imported exactly once.
 * [x] diagnostics is imported exactly once.
 * [x] syntaxTree is imported exactly once.
 * [x] MacroParameters is not redundantly imported through a second path.
 * [x] tokenStream is not redundantly imported through a second path.
 *
 * OWNERSHIP
 *
 * [x] macroElement is the only production owned here.
 * [x] macroDeclaration remains owned by MacroDeclarations.
 * [x] macroPath remains owned by invocations.
 * [x] macroInvocation remains owned by invocations.
 * [x] macroExpression remains owned by invocations.
 * [x] expansion metadata remains owned by expansion.
 * [x] hygiene metadata remains owned by hygiene.
 * [x] diagnostic metadata remains owned by diagnostics.
 * [x] token-tree structure remains owned by tokenStream through syntaxTree.
 *
 * SAFETY
 *
 * [x] No embedded Rust actions.
 * [x] No semantic predicates.
 * [x] No I/O.
 * [x] No filesystem access.
 * [x] No network access.
 * [x] No process execution.
 * [x] No hardware discovery.
 * [x] No runtime execution.
 * [x] No unsafe Rust requirement.
 *
 * PORTABILITY
 *
 * [x] No CPU-specific syntax.
 * [x] No GPU-specific syntax.
 * [x] No FPGA-specific syntax.
 * [x] No ASIC-specific syntax.
 * [x] No QPU-specific syntax.
 * [x] No physical-qubit mapping.
 * [x] No physical topology.
 * [x] No target-specific macro language.
 *
 * SCALABILITY
 *
 * [x] No universal macro-count limit.
 * [x] No universal parameter-count limit.
 * [x] No universal argument-count limit.
 * [x] No universal path-depth limit.
 * [x] No universal token-tree-size limit.
 * [x] No universal nesting limit.
 * [x] No universal expansion-size language limit.
 * [x] No hardware capacity encoded.
 *
 * QUANTUM
 *
 * [x] No fixed gate enumeration.
 * [x] No physical-qubit enumeration.
 * [x] No QPU enumeration.
 * [x] No topology selection.
 * [x] No routing.
 * [x] No scheduling.
 * [x] No QEC implementation.
 * [x] No ZQN implementation.
 * [x] No second quantum IR.
 * [x] quantum::ir remains the canonical quantum IR boundary.
 *
 * DOMAIN INTEGRATION
 *
 * [x] Classical source can be generated.
 * [x] Quantum source can be generated.
 * [x] Hybrid source can be generated.
 * [x] HDL source can be generated.
 * [x] Hardware intent can be generated.
 * [x] Distributed source can be generated.
 * [x] AI/data source can be generated.
 * [x] Networking source can be generated.
 * [x] Security source can be generated.
 *
 * AST
 *
 * [x] Macro invocation remains representable by the existing frontend
 *     MacroExpression abstraction.
 * [x] Macro expansion remains separate from parsing.
 * [x] No second macro AST is introduced by this file.
 *
 * ============================================================================
 * 31. REQUIRED EXTERNAL INTEGRATION FIXES
 * ============================================================================
 *
 * The following items are intentionally NOT implemented by this file because
 * they belong to their existing owners.
 *
 * ---------------------------------------------------------------------------
 * A. MacroParameters ownership
 * ---------------------------------------------------------------------------
 *
 * File:
 *
 *   grammar/macros/parameters.g4
 *
 * Must remain the canonical owner of:
 *
 *   macroParameterList
 *   macroParameter
 *   macroParameterDefault
 *
 * File:
 *
 *   grammar/macros/declarations.g4
 *
 * must consume those rules through its canonical import/composition path and
 * must not retain duplicate local definitions.
 *
 * ---------------------------------------------------------------------------
 * B. Canonical parser integration
 * ---------------------------------------------------------------------------
 *
 * File:
 *
 *   grammar/antlr/ZamaniParser.g4
 *
 * must import:
 *
 *   Macros
 *
 * and expose macro declarations/expressions through the universal source
 * composition.
 *
 * It must not define a competing macroDeclaration, macroInvocation, or
 * macroExpression rule.
 *
 * ---------------------------------------------------------------------------
 * C. Expression integration
 * ---------------------------------------------------------------------------
 *
 * File:
 *
 *   grammar/expressions/macros.g4
 *
 * remains the expression-side integration contract.
 *
 * It must consume the canonical macroExpression rather than redefine it.
 *
 * ---------------------------------------------------------------------------
 * D. AST integration
 * ---------------------------------------------------------------------------
 *
 * Existing:
 *
 *   src/frontend/ast/node/expressions/macro.rs
 *
 * remains the canonical macro-expression AST representation.
 *
 * Macro declarations require their corresponding declaration-side AST
 * representation and lowering contract.
 *
 * ---------------------------------------------------------------------------
 * E. Macro engine
 * ---------------------------------------------------------------------------
 *
 * Existing:
 *
 *   src/compiler/macro_engine.rs
 *
 * remains downstream from parsing.
 *
 * Its configurable expansion/resource policies must not be copied into this
 * grammar.
 *
 * ---------------------------------------------------------------------------
 * F. ANTLR duplicate ownership
 * ---------------------------------------------------------------------------
 *
 * Any older macro productions in:
 *
 *   grammar/antlr/Core.g4
 *   grammar/antlr/Meta.g4
 *   grammar/antlr/ZamaniParser.g4
 *
 * must be removed or converted to references to the canonical macro
 * composition.
 *
 * This file does not duplicate those rules in order to compensate for stale
 * definitions elsewhere.
 *
 * ============================================================================
 * 32. REQUIRED CONFORMANCE TEST MATRIX
 * ============================================================================
 *
 * The macro subsystem should be tested at these layers.
 *
 * LEXICAL
 *
 *   macro
 *   identifiers
 *   punctuation
 *   invocation delimiters
 *   path separators
 *   annotation syntax
 *
 * SYNTAX
 *
 *   macro empty() { }
 *   macro one(value) { }
 *   macro many(a, b, c) { }
 *   macro trailing(a, b,) { }
 *   macro typed(value: T) { }
 *   macro defaulted(value = expression) { }
 *   macro typedDefaulted(value: T = expression) { }
 *   macro generic<T>(value: T) { }
 *
 * INVOCATION
 *
 *   build!()
 *   build!(value)
 *   build!(a, b)
 *   math::build!(value)
 *   package::math::build!(a, b)
 *
 * TOKEN-TREE
 *
 *   ()
 *   []
 *   {}
 *   ({})
 *   ([{}])
 *   { call([value]) }
 *
 * CROSS-DOMAIN
 *
 *   macro-generated classical source
 *   macro-generated quantum source
 *   macro-generated hybrid source
 *   macro-generated HDL source
 *   macro-generated hardware-intent source
 *   macro-generated distributed source
 *   macro-generated AI/data source
 *   macro-generated networking source
 *   macro-generated security source
 *
 * NEGATIVE
 *
 *   malformed declaration
 *   malformed parameter list
 *   malformed invocation
 *   malformed path
 *   unmatched delimiters
 *   crossed delimiters
 *   malformed metadata
 *   duplicate parameter names
 *   invalid default syntax
 *
 * SCALABILITY
 *
 *   many declarations
 *   many parameters
 *   many arguments
 *   deep namespace paths
 *   deeply nested token trees
 *   large macro bodies
 *   large generated source
 *
 * Tests may be resource-bounded by the test environment, but must not turn
 * those environmental bounds into language-level grammar limits.
 *
 * ============================================================================
 * 33. FINAL ARCHITECTURAL GUARANTEE
 * ============================================================================
 *
 * This file establishes the following invariant:
 *
 *
 *   MACROS ARE SOURCE TRANSFORMATION SYNTAX,
 *   NOT TARGET-SPECIFIC COMPUTATION.
 *
 *
 * Therefore:
 *
 *   macro syntax
 *       |
 *       v
 *   domain-neutral AST
 *       |
 *       v
 *   controlled expansion
 *       |
 *       v
 *   semantic analysis
 *       |
 *       v
 *   canonical semantic model
 *       |
 *       +---------------------+---------------------+
 *       |                     |                     |
 *       v                     v                     v
 *   classical             quantum::ir          HDL/hardware
 *       |                     |                     |
 *       +---------------------+---------------------+
 *                             |
 *                             v
 *                       optimization
 *                             |
 *                    routing / scheduling
 *                             |
 *                    resilience / QEC / ZQN
 *                             |
 *                            HAL
 *                             |
 *                       target realization
 *
 *
 * The macro grammar therefore remains:
 *
 *   deterministic
 *   compositional
 *   domain-neutral
 *   target-independent
 *   resource-independent
 *   capability-neutral
 *   provenance-preserving
 *   scalable
 *   safe
 *
 * and compatible with:
 *
 *   Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * ============================================================================
 * END OF grammar/macros/macros.g4
 * ============================================================================
 */
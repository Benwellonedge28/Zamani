/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/metaprogramming/compile-time.g4
 *
 * Grammar:
 *     CompileTime
 *
 * Status:
 *     Production parser-composition boundary
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the compile-time METAPROGRAMMING composition boundary.
 *
 * It connects the explicit compile-time execution facility with the broader
 * metaprogramming subsystem without creating a second compile-time language.
 *
 * The architecture is:
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
 *     domain-neutral AST
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +--> compile-time eligibility
 *       +--> effect analysis
 *       +--> capability analysis
 *       +--> resource analysis
 *       +--> policy analysis
 *       +--> provenance
 *       +--> deterministic/reproducible evaluation
 *       |
 *       v
 *     compile-time evaluation / transformation
 *       |
 *       v
 *     canonical semantic model
 *       |
 *       +--> classical
 *       +--> quantum::ir
 *       +--> HDL / hardware
 *       +--> AI / data
 *       +--> distributed
 *       +--> networking
 *       +--> future domains
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
 * This grammar defines syntax and composition only.
 *
 * It MUST NOT:
 *
 *     - execute compile-time code;
 *     - evaluate expressions;
 *     - inspect the compiler host;
 *     - inspect hardware;
 *     - select a device;
 *     - allocate resources;
 *     - access the filesystem;
 *     - access the network;
 *     - access credentials;
 *     - execute subprocesses;
 *     - invoke GPUs;
 *     - invoke FPGAs;
 *     - invoke QPUs;
 *     - perform routing;
 *     - perform scheduling;
 *     - perform QEC;
 *     - construct IR;
 *     - construct quantum::ir;
 *     - implement runtime semantics.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - compile-time metaprogramming composition;
 *     - compile-time declaration/expression/statement adapters;
 *     - the stable public boundary consumed by metaprogramming.g4;
 *     - the distinction between compile-time metaprogramming syntax and
 *       ordinary runtime syntax;
 *     - integration of compile-time execution into metaprogramming.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer tokens;
 *     - keywords;
 *     - identifiers;
 *     - qualified names;
 *     - ordinary expressions;
 *     - expression precedence;
 *     - ordinary statements;
 *     - declarations;
 *     - types;
 *     - patterns;
 *     - functions;
 *     - macros;
 *     - macro expansion;
 *     - quotation;
 *     - unquotation;
 *     - reflection;
 *     - source generation;
 *     - specialization algorithms;
 *     - conditional compilation;
 *     - resource requirements;
 *     - capabilities;
 *     - policies;
 *     - effects;
 *     - provenance implementation;
 *     - AST implementation;
 *     - semantic evaluation;
 *     - canonical IR;
 *     - quantum::ir;
 *     - HDL IR;
 *     - target selection;
 *     - hardware realization.
 *
 * ============================================================================
 * AUTHORITATIVE OWNERS
 * ============================================================================
 *
 * Lexical authority:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Canonical parser composition:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * Metaprogramming composition:
 *
 *     grammar/metaprogramming/metaprogramming.g4
 *
 * Explicit compile-time execution:
 *
 *     grammar/metaprogramming/compile-time-execution.g4
 *
 * Expression-level compile-time syntax:
 *
 *     grammar/expressions/compile-time.g4
 *
 * Compilation-control syntax:
 *
 *     grammar/compile/compile-time.g4
 *
 * Compile-time function syntax:
 *
 *     grammar/functions/compile-time-functions.g4
 *
 * Reflection:
 *
 *     grammar/metaprogramming/reflection.g4
 *
 * Generation:
 *
 *     grammar/metaprogramming/generation.g4
 *
 * Specialization:
 *
 *     grammar/metaprogramming/specialization.g4
 *
 * Quotation:
 *
 *     grammar/metaprogramming/quotation.g4
 *
 * ============================================================================
 * IMPORTANT ARCHITECTURAL DISTINCTION
 * ============================================================================
 *
 * There are multiple compile-time concepts in Zamani:
 *
 *     1. compile-time METAPROGRAMMING
 *     2. compile-time EXECUTION
 *     3. compile-time EXPRESSIONS
 *     4. compilation CONTROL
 *     5. compile-time FUNCTIONS
 *
 * They must not become one giant grammar.
 *
 * This file owns only #1:
 *
 *     compile-time metaprogramming composition.
 *
 * It delegates #2 to compile-time-execution.g4.
 *
 * It does not duplicate #3 or #4.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Compile-time syntax must describe program intent rather than accidentally
 * freezing properties of the machine performing compilation.
 *
 * This grammar therefore contains no universal assumptions about:
 *
 *     CPU count
 *     core count
 *     thread count
 *     GPU count
 *     FPGA count
 *     accelerator count
 *     QPU count
 *     qubit count
 *     memory capacity
 *     storage capacity
 *     register width
 *     vector width
 *     tensor rank
 *     tensor dimensions
 *     node count
 *     network size
 *     topology size
 *     device count
 *     physical addresses
 *     vendor identifiers
 *
 * Compile-time code MAY semantically inspect explicitly authorized
 * capabilities/resources.
 *
 * That information belongs to the semantic resource/capability systems and
 * must never become an implicit permanent source-level machine dependency.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar imposes no language-level finite limit on:
 *
 *     compile-time declarations
 *     compile-time expressions
 *     compile-time statements
 *     nesting
 *     generated structures
 *     specialization candidates
 *     metadata
 *     source size
 *     semantic object size
 *
 * ANTLR repetition and ordinary recursive grammar structure represent
 * unbounded language structure.
 *
 * Compiler resource controls MAY independently impose:
 *
 *     memory budgets
 *     execution budgets
 *     cancellation
 *     timeout policies
 *     recursion protection
 *     generated-output budgets
 *     evaluation budgets
 *
 * Such limits are implementation/resource policy.
 *
 * They MUST NOT become grammar constants.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Parsing a compile-time construct grants NO capability.
 *
 * In particular, syntax alone must not grant:
 *
 *     filesystem access
 *     network access
 *     environment access
 *     credential access
 *     process execution
 *     native execution
 *     device discovery
 *     GPU access
 *     FPGA access
 *     QPU access
 *     cloud access
 *
 * Authorization is determined downstream through:
 *
 *     effects
 *       +
 *     capabilities
 *       +
 *     resources
 *       +
 *     policies
 *       +
 *     security rules
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Compile-time evaluation may have effects.
 *
 * The grammar does not classify them.
 *
 * Semantic analysis must determine whether a compile-time computation is:
 *
 *     pure
 *     deterministic
 *     reproducible
 *     effectful
 *     capability-dependent
 *     resource-dependent
 *     externally state-dependent
 *
 * Compile-time context MUST NOT silently erase effects.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Compile-time transformations must preserve enough information to explain:
 *
 *     source
 *     transformation
 *     generated artifact
 *     evaluation context
 *     dependencies
 *     evidence
 *     policy
 *     capability decisions
 *     resource decisions
 *
 * Generated source MUST re-enter the ordinary validation pipeline.
 *
 * It MUST NOT bypass:
 *
 *     type checking
 *     name resolution
 *     effect checking
 *     capability checking
 *     resource checking
 *     policy checking
 *     security checking
 *     provenance checking
 *     domain validation
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar owns no AST implementation.
 *
 * The frontend must map the exported compile-time syntax into the existing
 * domain-neutral AST.
 *
 * The resulting AST must preserve at minimum:
 *
 *     source span
 *     source order
 *     nesting
 *     syntactic category
 *     explicit compile-time intent
 *     child expression/declaration/statement structure
 *     attributes where present
 *
 * No compile-time-specific domain IR is permitted here.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar produces NO IR.
 *
 * It must never construct:
 *
 *     ClassicalInstruction
 *     QuantumGate
 *     Qubit
 *     PhysicalQubit
 *     HDLInstruction
 *     HardwareInstruction
 *     RoutingOperation
 *     SchedulingOperation
 *     QECOperation
 *
 * If compile-time computation produces quantum source, the result must still
 * follow:
 *
 *     source
 *       -> AST
 *       -> semantic analysis
 *       -> quantum::ir
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * ============================================================================
 * DOMAIN INTEGRATION
 * ============================================================================
 *
 * Compile-time metaprogramming is domain-neutral.
 *
 * It may produce or transform source representing:
 *
 *     classical computation
 *     quantum computation
 *     hybrid computation
 *     HDL
 *     hardware intent
 *     AI/data computation
 *     distributed computation
 *     networking
 *     accelerator computation
 *     future domains
 *
 * This file MUST NOT enumerate domain-specific operations.
 *
 * A new quantum operation, hardware capability, accelerator, tensor facility,
 * AI model, HDL construct, or future execution domain must not require this
 * grammar to be modified merely because the new domain exists.
 *
 * ============================================================================
 * PUBLIC COMPOSITION CONTRACT
 * ============================================================================
 *
 * The public rules exported by this grammar are:
 *
 *     compileTimeDeclarationCore
 *     compileTimeExpressionCore
 *     compileTimeStatementCore
 *
 * These are the ONLY metaprogramming compile-time adapter rules that
 * `metaprogramming.g4` should consume from this file.
 *
 * Do not add a second set of declaration/statement/expression aliases.
 *
 * ============================================================================
 */

parser grammar CompileTime;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * PUBLIC COMPILE-TIME DECLARATION CORE
 * ============================================================================
 *
 * Explicit compile-time execution syntax remains owned by
 * compile-time-execution.g4.
 *
 * This adapter intentionally does not reproduce that grammar.
 *
 * The exact declaration/result legality is determined downstream.
 * ============================================================================
 */

compileTimeDeclarationCore
    : compileTimeExecution
    ;

/*
 * ============================================================================
 * PUBLIC COMPILE-TIME EXPRESSION CORE
 * ============================================================================
 *
 * Expression-level compile-time syntax has its own canonical owner:
 *
 *     grammar/expressions/compile-time.g4
 *
 * This metaprogramming boundary consumes that facility rather than defining
 * another `COMPTIME` or `CONST` expression syntax.
 *
 * ============================================================================
 */

compileTimeExpressionCore
    : compileTimeExpression
    ;

/*
 * ============================================================================
 * PUBLIC COMPILE-TIME STATEMENT CORE
 * ============================================================================
 *
 * Explicit compile-time execution syntax remains owned by
 * compile-time-execution.g4.
 * ============================================================================
 */

compileTimeStatementCore
    : compileTimeExecution
    ;

/*
 * ============================================================================
 * TOOLING / SEMANTIC CONTEXT BOUNDARY
 * ============================================================================
 *
 * These rules intentionally expose the semantic category without defining a
 * second syntax.
 *
 * They are useful to parser composition and tooling, but they MUST NOT be
 * consumed as alternative language forms by the root parser.
 *
 * ============================================================================
 */

compileTimeMetaprogram
    : compileTimeExpressionCore
    | compileTimeStatementCore
    | compileTimeDeclarationCore
    ;

/*
 * ============================================================================
 * PHASE BOUNDARY
 * ============================================================================
 *
 * This grammar does not establish that a construct MUST be evaluated during
 * compilation.
 *
 * The semantic/compiler subsystem determines:
 *
 *     whether evaluation is permitted;
 *     whether evaluation is required;
 *     whether evaluation is deferred;
 *     whether specialization is performed;
 *     whether generation occurs;
 *     whether the result is cached;
 *     whether evaluation is deterministic;
 *     which capabilities are required.
 *
 * ============================================================================
 */

compileTimePhaseBoundary
    : compileTimeMetaprogram
    ;

/*
 * ============================================================================
 * GENERATED-SOURCE BOUNDARY
 * ============================================================================
 *
 * Generated source is not a special AST universe.
 *
 * It returns to the normal Zamani pipeline.
 *
 * No generated-source syntax is duplicated here.
 * ============================================================================
 */

compileTimeGeneratedSource
    : compileTimeMetaprogram
    ;

/*
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Explicitly forbidden:
 *
 *     MAX_COMPTIME_OPERATIONS
 *     MAX_COMPTIME_DEPTH
 *     MAX_GENERATED_ITEMS
 *     MAX_GENERATED_TYPES
 *     MAX_GENERATED_EXPRESSIONS
 *     MAX_SPECIALIZATIONS
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
 * This grammar contains none of these limits.
 *
 * ============================================================================
 * SAFE-RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no Rust actions;
 *     no semantic predicates;
 *     no embedded executable code;
 *     no host callbacks;
 *     no filesystem access;
 *     no network access;
 *     no unsafe code.
 *
 * The consuming implementation must remain compatible with:
 *
 *     Rust 1.97+
 *     Rust 2021
 *     safe Rust only
 *
 * ============================================================================
 * ERROR-RECOVERY CONTRACT
 * ============================================================================
 *
 * The grammar must preserve normal ANTLR error recovery.
 *
 * It must not use semantic predicates or target-language actions to implement
 * semantic validation.
 *
 * Invalid cases such as:
 *
 *     malformed compile-time blocks
 *     malformed compile-time bindings
 *     malformed compile-time expressions
 *     invalid nesting
 *     missing delimiters
 *
 * are diagnosed by the parser and/or downstream semantic validation.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive:
 *
 *     compile-time expression
 *     compile-time execution block
 *     compile-time value production
 *     compile-time declaration generation
 *     compile-time statement execution
 *
 * Cross-domain:
 *
 *     compile-time classical source
 *     compile-time quantum source
 *     compile-time hybrid source
 *     compile-time HDL source
 *     compile-time hardware intent
 *     compile-time AI/data source
 *     compile-time distributed source
 *
 * Negative:
 *
 *     malformed compile-time construct
 *     missing compile-time body
 *     malformed nested construct
 *     invalid delimiter
 *     incomplete construct
 *
 * Scalability:
 *
 *     increasing nesting
 *     increasing generated structure
 *     increasing expression complexity
 *     increasing declaration count
 *
 * Tests MUST NOT define a universal maximum.
 *
 * Determinism:
 *
 *     identical source + identical grammar configuration
 *     ->
 *     equivalent parse structure and source spans
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * This file is consumed by:
 *
 *     grammar/metaprogramming/metaprogramming.g4
 *
 * Its three public core rules are:
 *
 *     compileTimeDeclarationCore
 *     compileTimeExpressionCore
 *     compileTimeStatementCore
 *
 * The following files remain separate authorities:
 *
 *     grammar/metaprogramming/compile-time-execution.g4
 *     grammar/expressions/compile-time.g4
 *     grammar/compile/compile-time.g4
 *     grammar/functions/compile-time-functions.g4
 *
 * The canonical parser remains responsible for determining where
 * metaprogramming constructs are legal in source structure.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * DONE means:
 *
 *     [x] One ownership boundary.
 *     [x] No duplicate compile-time execution grammar.
 *     [x] No duplicate expression grammar.
 *     [x] No local lexer vocabulary.
 *     [x] No machine/resource limits.
 *     [x] No hardware assumptions.
 *     [x] No quantum-gate enumeration.
 *     [x] No IR creation.
 *     [x] No semantic execution.
 *     [x] No host-resource access.
 *     [x] Safe-Rust compatible.
 *     [x] Public integration rules are explicit.
 *     [x] AST ownership is downstream.
 *     [x] Semantic ownership is downstream.
 *     [x] Effects/capabilities/resources remain downstream.
 *     [x] Provenance remains preserved downstream.
 *     [x] Generated source returns to the canonical pipeline.
 *     [x] Cross-domain operation remains open-ended.
 *     [x] POCO-REAF constraints are preserved.
 *
 * ============================================================================
 */
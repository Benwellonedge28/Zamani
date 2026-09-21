/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/statements/statements.g4
 *
 * STATUS
 * ------
 * CANONICAL UNIVERSAL STATEMENT COMPOSITION GRAMMAR
 *
 * VERSION
 * -------
 * Zamani language grammar architecture
 *
 * GRAMMAR TECHNOLOGY
 * ------------------
 * ANTLR4 parser grammar
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 1.97 / Rust 1.97.1
 * Rust 2021
 * Safe Rust only.
 *
 * This grammar contains:
 *
 *     - no embedded Rust actions;
 *     - no semantic predicates;
 *     - no unsafe Rust;
 *     - no I/O;
 *     - no filesystem access;
 *     - no networking;
 *     - no hardware discovery;
 *     - no runtime execution;
 *     - no mutable global state;
 *     - no target-specific implementation;
 *     - no machine-size constants.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the SINGLE AUTHORITATIVE UNIVERSAL STATEMENT COMPOSITION
 * BOUNDARY for Zamani.
 *
 * It owns exactly one public language-wide entry point:
 *
 *     statement
 *
 * Concrete statement syntax remains owned by specialized statement grammars.
 *
 * This file MUST compose those grammars rather than reimplement them.
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * Zamani source
 *      |
 *      v
 * canonical lexer
 *      |
 *      v
 * canonical parser
 *      |
 *      v
 * statement composition                 <-- THIS FILE
 *      |
 *      +--> declarations
 *      +--> assignments
 *      +--> assertions
 *      +--> control flow
 *      +--> concurrency
 *      +--> effects
 *      +--> resources
 *      +--> domain statements
 *      +--> blocks
 *      +--> empty statements
 *      +--> expression statements
 *      |
 *      v
 * domain-neutral frontend AST
 *      |
 *      v
 * structural validation
 *      |
 *      v
 * semantic analysis
 *      |
 *      +--> names
 *      +--> types
 *      +--> effects
 *      +--> ownership
 *      +--> capabilities
 *      +--> resources
 *      +--> control flow
 *      +--> domain semantics
 *      |
 *      v
 * canonical semantic representations
 *      |
 *      +--> classical representation
 *      +--> quantum::ir
 *      +--> HDL / hardware representation
 *      +--> distributed representation
 *      +--> accelerator representation
 *      +--> future-domain representations
 *      |
 *      v
 * optimization
 *      |
 *      v
 * routing / scheduling / lowering
 *      |
 *      v
 * target realization
 *      |
 *      v
 * runtime / hardware
 *
 * THIS FILE MUST NOT BYPASS THAT PIPELINE.
 *
 * ============================================================================
 * OWNERSHIP CONTRACT
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 *     statement
 *
 *     The universal composition relationship between statement families.
 *
 *     The distinction between:
 *
 *         specialized statement
 *         empty statement
 *         generic expression statement
 *
 *     The stable parser boundary through which all statement families enter
 *     the language.
 *
 * THIS FILE DOES NOT OWN
 * ----------------------
 *
 *     lexer rules
 *     token definitions
 *     keyword definitions
 *     identifiers
 *     names
 *     paths
 *     expressions
 *     expression precedence
 *     types
 *     declarations
 *     assignments
 *     bindings
 *     assertions
 *     blocks
 *     conditionals
 *     loops
 *     pattern matching
 *     break
 *     continue
 *     return
 *     exceptions
 *     unsafe statements
 *     concurrency syntax
 *     effect syntax
 *     resource syntax
 *     quantum syntax
 *     classical syntax
 *     hybrid syntax
 *     HDL syntax
 *     hardware syntax
 *     distributed syntax
 *     AI syntax
 *     data syntax
 *     networking syntax
 *     security syntax
 *     compilation syntax
 *     execution syntax
 *     domain-specific operations
 *     QEC
 *     ZQN
 *     routing
 *     scheduling
 *     calibration
 *     HAL
 *     target selection
 *     physical device selection
 *     runtime execution
 *
 * ============================================================================
 * SINGLE-SOURCE-OF-TRUTH RULE
 * ============================================================================
 *
 * There MUST be exactly one authoritative effective `statement` rule in the
 * assembled production parser.
 *
 * This grammar owns that rule.
 *
 * Imported grammars MUST NOT define another universal:
 *
 *     statement
 *
 * rule.
 *
 * Specialized grammars may define their own public entry points, for example:
 *
 *     declarationStatement
 *     assignmentStatement
 *     assertionStatement
 *     controlFlowStatement
 *     concurrencyStatementAdapter
 *     effectStatement
 *     resourceStatement
 *     domainStatement
 *     blockExpression
 *     expressionStatement
 *
 * but none of them may replace the universal `statement` owner.
 *
 * ============================================================================
 * DOMAIN-NEUTRALITY
 * ============================================================================
 *
 * The statement layer is intentionally independent of computational target.
 *
 * The same source-level statement composition must be capable of representing
 * programs involving:
 *
 *     classical computing
 *     quantum computing
 *     hybrid computing
 *     HDL
 *     hardware/software co-design
 *     embedded systems
 *     parallel computing
 *     distributed computing
 *     HPC
 *     AI/ML
 *     data processing
 *     networking
 *     cryptography
 *     scientific computing
 *     accelerators
 *     edge/cloud execution
 *     nano-scale computation
 *     future computational domains
 *
 * Domain meaning is supplied by the domain grammars and semantic layers.
 *
 * This file therefore MUST NOT contain alternatives such as:
 *
 *     cpuStatement
 *     gpuStatement
 *     fpgaStatement
 *     qpuStatement
 *     asicStatement
 *     clusterStatement
 *
 * merely because different targets exist.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * The statement grammar expresses source-level program meaning.
 *
 * It MUST NOT encode current hardware capacity as language syntax.
 *
 * The grammar therefore contains no universal limits for:
 *
 *     CPUs
 *     cores
 *     threads
 *     GPUs
 *     FPGAs
 *     ASICs
 *     QPUs
 *     qubits
 *     registers
 *     memory
 *     storage
 *     accelerators
 *     nodes
 *     devices
 *     network links
 *     tensor dimensions
 *     timelines
 *     processes
 *     tasks
 *
 * A source program may therefore be compiled for a tiny target or a much
 * larger target according to available resources and declared semantic
 * requirements.
 *
 * This is the grammar-level foundation for:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * ============================================================================
 * SEMANTIC REQUIREMENT VS IMPLEMENTATION DECISION
 * ============================================================================
 *
 * Statement syntax must preserve the distinction between:
 *
 *     semantic requirement
 *     capability requirement
 *     constraint
 *     preference
 *     hint
 *     implementation decision
 *
 * Examples of portable intent belong to the resource/capability layers:
 *
 *     requires capability("quantum.measurement")
 *     requires capability("gpu.compute")
 *
 * The statement composition layer MUST NOT convert such intent into:
 *
 *     physical device 0
 *     CPU core 7
 *     GPU 3
 *     physical qubit 17
 *     FPGA region 2
 *
 * Those are downstream realization concerns.
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * Quantum statements enter through:
 *
 *     domainStatement
 *
 * and ultimately the quantum statement adapter.
 *
 * This file does NOT enumerate:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     CNOT
 *     SWAP
 *     or any other finite gate inventory.
 *
 * It does NOT define:
 *
 *     qubit limits
 *     physical qubits
 *     topology
 *     routing
 *     scheduling
 *     calibration
 *     QEC
 *     ZQN
 *     resilience
 *     HAL
 *     backend selection
 *
 * The downstream quantum path remains:
 *
 *     source
 *       -> lexer
 *       -> parser
 *       -> domain-neutral AST
 *       -> semantic quantum representation
 *       -> quantum::ir
 *       -> optimization
 *       -> decomposition
 *       -> routing
 *       -> scheduling
 *       -> QEC / resilience / ZQN
 *       -> HAL
 *       -> target realization
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * ============================================================================
 * HDL / HARDWARE CONTRACT
 * ============================================================================
 *
 * HDL and hardware statements enter through the domain composition boundary.
 *
 * This file does NOT encode:
 *
 *     fixed bus widths
 *     fixed register widths
 *     fixed memory sizes
 *     fixed clock counts
 *     fixed pipeline depths
 *     fixed device counts
 *     physical addresses
 *     FPGA resources
 *     ASIC resources
 *     vendor devices
 *     physical topology
 *
 * Hardware intent is resolved downstream through:
 *
 *     capabilities
 *     resources
 *     constraints
 *     target descriptions
 *     synthesis
 *     placement
 *     routing
 *     scheduling
 *     lowering
 *
 * ============================================================================
 * CONTROL-FLOW CONTRACT
 * ============================================================================
 *
 * Control-flow composition is owned by:
 *
 *     grammar/statements/control-flow.g4
 *
 * This file MUST NOT recreate:
 *
 *     ifStatement
 *     loopStatement
 *     matchStatement
 *     breakStatement
 *     continueStatement
 *     returnStatement
 *     throwStatement
 *     tryStatement
 *
 * Doing so would create competing ownership.
 *
 * ============================================================================
 * DOMAIN COMPOSITION CONTRACT
 * ============================================================================
 *
 * Domain statements are composed through:
 *
 *     grammar/statements/domains.g4
 *
 * That grammar owns:
 *
 *     domainStatement
 *
 * and delegates to the domain-specific statement adapters.
 *
 * This file therefore does not directly import every domain grammar.
 *
 * The dependency direction is:
 *
 *     Statements
 *          |
 *          v
 *     Domains
 *          |
 *          +--> classical
 *          +--> quantum
 *          +--> HDL
 *          +--> hybrid
 *          +--> distributed
 *          +--> AI
 *          +--> data
 *          +--> networking
 *          +--> security
 *          +--> resources
 *          +--> compilation
 *          +--> execution
 *          +--> hardware
 *
 * This keeps the universal statement composition independent of individual
 * domain implementations.
 *
 * ============================================================================
 * CONCURRENCY CONTRACT
 * ============================================================================
 *
 * Concurrency is composed through:
 *
 *     grammar/statements/concurrency.g4
 *
 * This file must not duplicate:
 *
 *     async
 *     await
 *     spawn
 *     task
 *     actor
 *     channel
 *     parallel
 *     synchronization
 *     cancellation
 *     distributed-concurrency
 *
 * syntax.
 *
 * Concurrency semantics must not encode a fixed number of:
 *
 *     threads
 *     tasks
 *     workers
 *     actors
 *     cores
 *     nodes
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Effects are composed through:
 *
 *     grammar/statements/effects.g4
 *
 * Effect semantics remain downstream.
 *
 * This file does not define:
 *
 *     effect declarations
 *     effect operations
 *     handlers
 *     capability implementation
 *     resource implementation
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Resource intent is composed through:
 *
 *     grammar/statements/resource.g4
 *
 * Resource syntax remains owned by:
 *
 *     grammar/resources/
 *
 * This file does not define resource quantities, capacities, inventories,
 * placement, acquisition, reservation, scheduling or allocation.
 *
 * ============================================================================
 * ASSIGNMENT CONTRACT
 * ============================================================================
 *
 * Assignment syntax is owned by:
 *
 *     grammar/statements/assignments.g4
 *
 * The universal statement layer merely admits:
 *
 *     assignmentStatement
 *
 * Assignment expressions remain owned by the expression grammar.
 *
 * This distinction prevents:
 *
 *     assignment expression
 *
 * from being confused with:
 *
 *     assignment statement.
 *
 * ============================================================================
 * DECLARATION CONTRACT
 * ============================================================================
 *
 * Declaration statement syntax is owned by:
 *
 *     grammar/statements/declarations.g4
 *
 * This file only admits:
 *
 *     declarationStatement
 *
 * It must not duplicate:
 *
 *     let
 *     var
 *     const
 *     struct
 *     class
 *     enum
 *     resource
 *     capability
 *
 * declaration syntax.
 *
 * ============================================================================
 * ASSERTION CONTRACT
 * ============================================================================
 *
 * Assertion syntax is owned by:
 *
 *     grammar/statements/assertions.g4
 *
 * This file only admits:
 *
 *     assertionStatement
 *
 * Assertion evaluation, proof, optimization and runtime behavior remain
 * downstream concerns.
 *
 * ============================================================================
 * BLOCK CONTRACT
 * ============================================================================
 *
 * Block syntax is owned by:
 *
 *     grammar/statements/blocks.g4
 *
 * This file must not recreate:
 *
 *     {
 *     }
 *     blockExpression
 *     blockElement
 *
 * Block contents ultimately recurse into this universal `statement` rule
 * through the block grammar.
 *
 * ============================================================================
 * EMPTY STATEMENT CONTRACT
 * ============================================================================
 *
 * A standalone statement terminator is a valid empty statement:
 *
 *     ;
 *
 * It has no expression.
 *
 * Any warning, linting, optimization or style policy is downstream.
 *
 * ============================================================================
 * EXPRESSION STATEMENT CONTRACT
 * ============================================================================
 *
 * A canonical expression may appear in statement position:
 *
 *     expression ;
 *
 * The expression grammar owns expression syntax.
 *
 * This file owns only the statement-position wrapper.
 *
 * Domain-specific operation names MUST NOT be enumerated here.
 *
 * Examples that may ultimately be represented by the expression layer include:
 *
 *     function calls
 *     mathematical operations
 *     tensor operations
 *     quantum operations
 *     accelerator operations
 *     networking operations
 *     data operations
 *     future domain operations
 *
 * Their semantic interpretation is downstream.
 *
 * ============================================================================
 * AMBIGUITY CONTRACT
 * ============================================================================
 *
 * The grammar must not solve semantic ambiguity by embedding semantic
 * predicates or target-specific knowledge.
 *
 * When multiple statement families can begin with a common token sequence,
 * ANTLR's parser prediction and the delegated grammar structure must determine
 * the syntactic alternative.
 *
 * Semantic distinctions belong downstream.
 *
 * In particular, this file MUST NOT use:
 *
 *     embedded actions
 *     semantic predicates
 *     target inspection
 *     symbol-table lookup
 *     type lookup
 *     capability lookup
 *     resource lookup
 *     runtime lookup
 *
 * to choose a statement.
 *
 * ============================================================================
 * ERROR BOUNDARY
 * ============================================================================
 *
 * Parser errors include:
 *
 *     unexpected token
 *     malformed statement
 *     missing delimiter
 *     incomplete expression
 *     malformed declaration
 *     malformed assignment
 *     malformed block
 *     malformed control-flow construct
 *     malformed domain construct
 *
 * Semantic errors remain downstream:
 *
 *     unknown name
 *     invalid type
 *     invalid ownership
 *     invalid effect
 *     invalid capability
 *     unavailable resource
 *     impossible resource requirement
 *     invalid quantum operation
 *     invalid hardware requirement
 *     invalid control-flow context
 *
 * This separation is mandatory for deterministic frontend architecture.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates parser contexts only.
 *
 * The frontend AST must map:
 *
 *     statement
 *
 * to the domain-neutral statement representation.
 *
 * Each concrete statement must retain sufficient source information for:
 *
 *     source span
 *     source ordering
 *     statement kind
 *     child relationships
 *     syntactic attributes
 *
 * The AST must remain independent of:
 *
 *     LLVM
 *     QIR
 *     MLIR
 *     vendor IRs
 *     physical topology
 *     QEC implementation
 *     backend-specific hardware.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file creates NO IR.
 *
 * The pipeline remains:
 *
 *     grammar
 *       -> parser
 *       -> AST
 *       -> semantic analysis
 *       -> canonical semantic model
 *       -> domain IR
 *       -> optimization
 *       -> lowering
 *       -> target realization
 *
 * For quantum:
 *
 *     AST
 *       -> quantum semantics
 *       -> quantum::ir
 *
 * No second quantum IR may be introduced by this statement layer.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * The grammar itself contains no Rust implementation code.
 *
 * Generated parser code and the consuming frontend must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *     safe Rust
 *
 * This grammar does not require `unsafe`.
 *
 * A Zamani source-language construct named `unsafe`, if supported by the
 * language, is unrelated to Rust implementation safety and remains owned by:
 *
 *     grammar/statements/unsafe.g4
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing must depend only on:
 *
 *     source tokens
 *     grammar version
 *     parser configuration
 *
 * It must NOT depend on:
 *
 *     current time
 *     randomness
 *     filesystem state
 *     network state
 *     hardware state
 *     device discovery
 *     scheduler state
 *     runtime state
 *     backend availability
 *
 * Identical canonical token streams under identical parser configuration must
 * produce equivalent parse-tree structure.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * There is no grammar-defined finite limit for:
 *
 *     statement count
 *     block count
 *     block depth
 *     branch count
 *     loop nesting
 *     match-arm count
 *     declaration count
 *     task count
 *     process count
 *     resource count
 *     device count
 *     node count
 *     qubit count
 *     CPU count
 *     GPU count
 *     FPGA count
 *     accelerator count
 *     memory capacity
 *     tensor dimensions
 *
 * Repetition is represented structurally.
 *
 * Actual parser/compiler resource exhaustion is an implementation/resource
 * concern, not a language-level semantic limit.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden in this file:
 *
 *     MAX_STATEMENTS
 *     MAX_BLOCKS
 *     MAX_BRANCHES
 *     MAX_LOOPS
 *     MAX_MATCH_ARMS
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_STORAGE
 *     MAX_REGISTER_WIDTH
 *     MAX_VECTOR_WIDTH
 *     MAX_TENSOR_RANK
 *     physical device identifiers
 *     physical addresses
 *     fixed topology
 *     vendor-specific hardware
 *
 * Numeric values appearing in source programs remain program semantics.
 *
 * ============================================================================
 * VERSIONING / COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Adding a new statement family requires:
 *
 *     1. dedicated grammar owner;
 *     2. stable public entry rule;
 *     3. AST contract;
 *     4. semantic contract;
 *     5. IR contract where applicable;
 *     6. compiler/runtime integration where applicable;
 *     7. positive tests;
 *     8. negative tests;
 *     9. boundary tests;
 *    10. scalability tests;
 *    11. determinism tests;
 *    12. compatibility documentation;
 *    13. hard-coding audit.
 *
 * Existing statement syntax must not be silently redefined.
 *
 * ============================================================================
 * REQUIRED IMPORTS
 * ============================================================================
 *
 * These imports are statement-family owners, not duplicate implementations.
 *
 * Domains is deliberately imported as one boundary rather than importing every
 * computational domain directly here.
 *
 * ============================================================================
 */

parser grammar Statements;

options {
    tokenVocab = ZamaniLexer;
}

import
    AssertionsParser,
    Assignments,
    Blocks,
    BreakStatements,
    ConditionalsParser,
    ContinueStatements,
    Declarations,
    Domains,
    ExceptionsParser,
    Expressions,
    EffectStatements,
    ConcurrencyStatements,
    Loops,
    PatternMatching,
    ResourceStatements,
    ReturnStatements,
    UnsafeStatementsParser
    ;

/*
 * ============================================================================
 * UNIVERSAL STATEMENT
 * ============================================================================
 *
 * This is the ONLY universal `statement` production.
 *
 * Ordering follows ownership boundaries rather than implementation targets.
 *
 * Specialized statement families are admitted before the generic expression
 * statement so dedicated statement constructs remain first-class parser
 * contexts.
 *
 * ============================================================================
 */

statement
    : declarationStatement
    | assignmentStatement
    | assertionStatement
    | controlFlowStatement
    | concurrencyStatementAdapter
    | effectStatement
    | resourceStatement
    | domainStatement
    | blockExpression
    | emptyStatement
    | expressionStatement
    ;

/*
 * ============================================================================
 * CONTROL-FLOW
 * ============================================================================
 *
 * ControlFlow owns this production.
 *
 * Do NOT recreate:
 *
 *     if
 *     loop
 *     match
 *     break
 *     continue
 *     return
 *     throw
 *     try
 *
 * syntax here.
 *
 * ============================================================================
 */

/*
 * `controlFlowStatement` is intentionally NOT defined here.
 *
 * It is imported from:
 *
 *     grammar/statements/control-flow.g4
 *
 * through the control-flow composition grammar.
 *
 * ============================================================================
 * EMPTY STATEMENT
 * ============================================================================
 *
 * Empty statements are intentionally separate from expression statements.
 *
 *     ;
 *
 * contains no expression.
 *
 * ============================================================================
 */

emptyStatement
    : SEMICOLON
    ;

/*
 * ============================================================================
 * GENERIC EXPRESSION STATEMENT
 * ============================================================================
 *
 * This is the universal fallback for an expression used in statement
 * position.
 *
 * The expression grammar owns:
 *
 *     expression
 *
 * and all expression precedence/associativity.
 *
 * This rule owns only:
 *
 *     expression + statement terminator
 *
 * ============================================================================
 */

expressionStatement
    : expression SEMICOLON
    ;

/*
 * ============================================================================
 * INTEGRATION NOTES
 * ============================================================================
 *
 * The imported grammar contracts are:
 *
 * AssertionsParser
 *     -> assertionStatement
 *
 * Assignments
 *     -> assignmentStatement
 *
 * Blocks
 *     -> blockExpression
 *
 * BreakStatements
 *     -> breakStatement
 *
 * ConditionalsParser
 *     -> ifStatement
 *
 * ContinueStatements
 *     -> continueStatement
 *
 * Declarations
 *     -> declarationStatement
 *
 * Domains
 *     -> domainStatement
 *
 * ExceptionsParser
 *     -> throwStatement / try-related syntax
 *
 * Expressions
 *     -> expression
 *
 * EffectStatements
 *     -> effectStatement
 *
 * ConcurrencyStatements
 *     -> concurrencyStatementAdapter
 *
 * Loops
 *     -> loopStatement
 *
 * PatternMatching
 *     -> matchStatement and pattern syntax
 *
 * ResourceStatements
 *     -> resourceStatement
 *
 * ReturnStatements
 *     -> returnStatement
 *
 * UnsafeStatementsParser
 *     -> unsafeStatement
 *
 * Control-flow composition:
 *
 *     grammar/statements/control-flow.g4
 *
 * owns:
 *
 *     controlFlowStatement
 *
 * and composes the appropriate imported control-flow families.
 *
 * ============================================================================
 * DOMAIN INTEGRATION
 * ============================================================================
 *
 * Domains remains the statement-level domain adapter.
 *
 * Its dependency direction is:
 *
 *     Statements
 *         |
 *         v
 *     Domains
 *         |
 *         +--> classical
 *         +--> quantum
 *         +--> hybrid
 *         +--> HDL
 *         +--> distributed
 *         +--> AI
 *         +--> data
 *         +--> networking
 *         +--> security
 *         +--> resources
 *         +--> compilation
 *         +--> execution
 *         +--> hardware
 *
 * No domain implementation is copied into this file.
 *
 * ============================================================================
 * FUTURE DOMAIN EXTENSION
 * ============================================================================
 *
 * A future computational domain must not require a rewrite of the universal
 * statement grammar merely because a new target technology exists.
 *
 * Instead it must provide:
 *
 *     domain grammar
 *     public statement entry rule
 *     statement adapter
 *     lexer integration if needed
 *     AST mapping
 *     semantic mapping
 *     IR mapping where applicable
 *     compiler integration
 *     runtime integration where applicable
 *     tests
 *     compatibility metadata
 *
 * Then Domains becomes the domain-composition integration point.
 *
 * ============================================================================
 * CROSS-DOMAIN INTEGRATION
 * ============================================================================
 *
 * The statement layer intentionally permits composition such as:
 *
 *     classical statement
 *         -> quantum statement
 *         -> measurement
 *         -> classical control
 *         -> resource requirement
 *         -> execution intent
 *
 * and:
 *
 *     classical computation
 *         -> accelerator intent
 *         -> hardware/HDL construct
 *         -> distributed execution
 *
 * The parser does not decide whether such a composition is semantically valid.
 *
 * That belongs to semantic analysis.
 *
 * ============================================================================
 * SEMANTIC INTEGRATION
 * ============================================================================
 *
 * Every statement must eventually be mapped into the domain-neutral frontend
 * AST.
 *
 * Semantic analysis then determines:
 *
 *     name resolution
 *     type validity
 *     ownership
 *     borrowing
 *     effects
 *     capabilities
 *     resources
 *     control-flow legality
 *     domain compatibility
 *     portability
 *
 * ============================================================================
 * IR INTEGRATION
 * ============================================================================
 *
 * Statements do not directly construct IR.
 *
 * Downstream mapping is determined by semantic domain:
 *
 *     classical
 *         -> classical representation
 *
 *     quantum
 *         -> quantum::ir
 *
 *     HDL/hardware
 *         -> canonical HDL/hardware representation
 *
 *     distributed
 *         -> distributed representation
 *
 *     hybrid
 *         -> appropriate combined semantic representation
 *
 *     future domains
 *         -> their canonical semantic representation
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * The same statement tree must remain valid independently of whether the
 * eventual realization uses:
 *
 *     embedded hardware
 *     CPU
 *     multicore CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     simulator
 *     accelerator
 *     cluster
 *     HPC system
 *     cloud system
 *     heterogeneous system
 *     future architecture
 *
 * Resource availability and capabilities determine realization downstream.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE:
 *
 *     ;
 *     expression;
 *     assignment;
 *     declaration;
 *     assertion;
 *     if statement;
 *     loop statement;
 *     match statement;
 *     break;
 *     continue;
 *     return;
 *     exception statement;
 *     concurrency statement;
 *     effect statement;
 *     resource statement;
 *     quantum statement;
 *     classical statement;
 *     hybrid statement;
 *     HDL statement;
 *     hardware statement;
 *     distributed statement;
 *     AI/data statement;
 *     networking statement;
 *     security statement;
 *     compile statement;
 *     execution statement.
 *
 * NEGATIVE:
 *
 *     malformed declaration;
 *     malformed assignment;
 *     malformed assertion;
 *     malformed block;
 *     malformed control flow;
 *     malformed domain statement;
 *     malformed resource statement;
 *     malformed expression;
 *     missing statement terminator where required.
 *
 * SEMANTIC NEGATIVES:
 *
 *     break outside valid loop context;
 *     continue outside valid loop context;
 *     return outside callable context;
 *     invalid type;
 *     invalid ownership;
 *     unavailable capability;
 *     unsatisfiable resource requirement;
 *     invalid quantum operation;
 *     invalid hardware requirement.
 *
 * These semantic cases MUST be rejected downstream, not by this grammar.
 *
 * BOUNDARY:
 *
 *     empty statement sequence;
 *     large statement sequences;
 *     deeply nested blocks;
 *     deeply nested control flow;
 *     large match structures;
 *     large declaration sets;
 *     large expressions;
 *     many independent domain statements.
 *
 * CROSS-DOMAIN:
 *
 *     classical + quantum;
 *     quantum + classical;
 *     classical + HDL;
 *     quantum + hardware;
 *     AI + accelerator;
 *     distributed + networking;
 *     resource + quantum;
 *     resource + hardware;
 *     compile + hardware;
 *     execution + distributed.
 *
 * DETERMINISM:
 *
 *     identical source
 *       + identical lexer configuration
 *       + identical parser configuration
 *       => equivalent parse-tree structure.
 *
 * ROUND-TRIP:
 *
 *     source
 *       -> lexer
 *       -> parser
 *       -> AST
 *       -> formatter
 *       -> parser
 *
 * must preserve source semantics.
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests MUST NOT establish artificial machine limits.
 *
 * They must verify that statement composition remains independent of:
 *
 *     CPU count
 *     GPU count
 *     FPGA count
 *     QPU count
 *     qubit count
 *     node count
 *     memory capacity
 *     accelerator count
 *     topology
 *
 * Repetition and nesting must remain structurally expressible until actual
 * parser/compiler resource limits are reached.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains no:
 *
 *     MAX_*
 *     fixed CPU count
 *     fixed GPU count
 *     fixed FPGA count
 *     fixed QPU count
 *     fixed qubit count
 *     fixed node count
 *     fixed device count
 *     fixed memory capacity
 *     fixed register width
 *     fixed tensor dimension
 *     fixed topology
 *     physical address
 *     vendor device identifier
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [ ] It is the sole universal `statement` owner.
 *
 *     [ ] All concrete statement families remain independently owned.
 *
 *     [ ] Control-flow composition comes from control-flow.g4.
 *
 *     [ ] Domain composition comes from domains.g4.
 *
 *     [ ] Concurrency composition is reachable.
 *
 *     [ ] Effect composition is reachable.
 *
 *     [ ] Resource composition is reachable.
 *
 *     [ ] Assignment syntax is not duplicated.
 *
 *     [ ] Declaration syntax is not duplicated.
 *
 *     [ ] Block syntax is not duplicated.
 *
 *     [ ] Expression syntax is not duplicated.
 *
 *     [ ] Quantum syntax is not duplicated.
 *
 *     [ ] HDL syntax is not duplicated.
 *
 *     [ ] Hardware realization is not encoded.
 *
 *     [ ] QEC is not encoded.
 *
 *     [ ] ZQN is not encoded.
 *
 *     [ ] Routing is not encoded.
 *
 *     [ ] Scheduling is not encoded.
 *
 *     [ ] Calibration is not encoded.
 *
 *     [ ] Target selection is not encoded.
 *
 *     [ ] Runtime execution is not encoded.
 *
 *     [ ] No machine-size limits exist.
 *
 *     [ ] No embedded actions exist.
 *
 *     [ ] No semantic predicates exist.
 *
 *     [ ] No unsafe Rust is required.
 *
 *     [ ] Rust 1.97 / 1.97.1 integration passes.
 *
 *     [ ] ANTLR generation passes.
 *
 *     [ ] All imported grammar identities resolve.
 *
 *     [ ] No imported grammar introduces another universal `statement`.
 *
 *     [ ] Positive tests pass.
 *
 *     [ ] Negative tests pass.
 *
 *     [ ] Boundary tests pass.
 *
 *     [ ] Scalability tests pass.
 *
 *     [ ] Cross-domain tests pass.
 *
 *     [ ] Determinism tests pass.
 *
 *     [ ] Round-trip tests pass.
 *
 * ============================================================================
 */
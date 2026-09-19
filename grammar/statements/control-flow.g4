/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/statements/control-flow.g4
 *
 * STATUS
 * ------
 * CANONICAL CONTROL-FLOW COMPOSITION GRAMMAR
 *
 * VERSION BASELINE
 * ----------------
 * Zamani grammar architecture
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 1.97 / Rust 1.97.1
 *
 * GRAMMAR TECHNOLOGY
 * ------------------
 * ANTLR4 parser grammar
 *
 * SAFETY
 * ------
 * This grammar contains:
 *
 *   - no embedded Rust actions;
 *   - no semantic predicates;
 *   - no unsafe Rust;
 *   - no I/O;
 *   - no filesystem access;
 *   - no networking;
 *   - no process execution;
 *   - no device discovery;
 *   - no hardware inspection;
 *   - no runtime execution;
 *   - no mutable global state;
 *   - no target-specific implementation;
 *   - no machine-size constants.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the SINGLE COMPOSITION OWNER for statement-level control flow.
 *
 * It does not implement the concrete syntax of individual control-flow
 * constructs. Those responsibilities remain in their dedicated grammar files.
 *
 * This file composes:
 *
 *     conditionals.g4
 *     loops.g4
 *     pattern-matching.g4
 *     breaks.g4
 *     continues.g4
 *     returns.g4
 *     exceptions.g4
 *
 * into one canonical:
 *
 *     controlFlowStatement
 *
 * entry point.
 *
 * The purpose of this layer is to prevent:
 *
 *     - multiple controlFlowStatement definitions;
 *     - duplicated control-flow dispatch;
 *     - duplicated try adapters;
 *     - domain-specific control-flow forks;
 *     - accidental grammar authority conflicts.
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * Control flow expresses SOURCE-LEVEL PROGRAM SEMANTICS.
 *
 * It does not express target realization.
 *
 * The grammar therefore knows nothing about:
 *
 *     CPUs
 *     cores
 *     threads
 *     GPUs
 *     FPGAs
 *     ASICs
 *     QPUs
 *     accelerators
 *     nodes
 *     memory capacities
 *     registers
 *     vector widths
 *     physical qubits
 *     physical addresses
 *     hardware topology
 *     scheduler queues
 *     device identifiers
 *     calibration
 *     routing
 *     QEC implementation
 *     ZQN implementation
 *
 * Those concerns remain downstream.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Zamani's control flow participates in:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * A conditional, loop, match, return, break, continue, or exception describes
 * program behavior. The compiler and runtime determine how that behavior is
 * realized on available resources.
 *
 * The same source-level control flow may therefore participate in:
 *
 *     embedded systems
 *     CPUs
 *     multicore systems
 *     GPUs
 *     FPGAs
 *     ASIC-oriented systems
 *     quantum-classical systems
 *     QPUs
 *     simulators
 *     accelerators
 *     distributed systems
 *     HPC systems
 *     cloud systems
 *     heterogeneous systems
 *     future architectures
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - controlFlowStatement;
 *     - composition of control-flow statement families;
 *     - stable tryStatement adapter;
 *     - control-flow grammar dependency boundaries.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - ifStatement;
 *     - elseIfClause;
 *     - elseClause;
 *     - loopStatement;
 *     - whileStatement;
 *     - doWhileStatement;
 *     - forStatement;
 *     - matchStatement;
 *     - breakStatement;
 *     - continueStatement;
 *     - returnStatement;
 *     - throwStatement;
 *     - tryCatchFinallyStatement;
 *     - catchClause;
 *     - finallyClause;
 *     - expression syntax;
 *     - block syntax;
 *     - declaration syntax;
 *     - type syntax;
 *     - lexer rules;
 *     - AST construction;
 *     - semantic analysis;
 *     - control-flow graph construction;
 *     - IR construction;
 *     - optimization;
 *     - routing;
 *     - scheduling;
 *     - QEC;
 *     - ZQN;
 *     - hardware selection;
 *     - runtime execution.
 *
 * ============================================================================
 * DEPENDENCY GRAPH
 * ============================================================================
 *
 *                         ControlFlow
 *                              |
 *             +----------------+----------------+
 *             |        |       |       |        |
 *             v        v       v       v        v
 *        Conditionals Loops Pattern Break   Continue
 *                                  |        |
 *             +--------------------+--------+
 *             |
 *             +-----------> Return
 *             |
 *             +-----------> Exceptions
 *
 * ControlFlow provides composition only.
 *
 * Concrete syntax remains owned by the imported grammar component.
 *
 * ============================================================================
 * GRAMMAR IMPORTS
 * ============================================================================
 *
 * Each imported grammar must expose its own public rule.
 *
 * Required public rules:
 *
 *     ConditionalsParser
 *         -> ifStatement
 *
 *     Loops
 *         -> loopStatement
 *
 *     PatternMatching
 *         -> matchStatement
 *
 *     BreakStatements
 *         -> breakStatement
 *
 *     ContinueStatements
 *         -> continueStatement
 *
 *     ReturnStatements
 *         -> returnStatement
 *
 *     ExceptionsParser
 *         -> throwStatement
 *         -> tryCatchFinallyStatement
 *
 * No concrete rule is copied into this file.
 *
 * ============================================================================
 * IMPORTANT ANTLR OWNERSHIP RULE
 * ============================================================================
 *
 * There must be exactly ONE effective `controlFlowStatement` rule in the
 * assembled production grammar.
 *
 * Therefore:
 *
 *     grammar/statements/statements.g4
 *
 * MUST NOT continue defining its own `controlFlowStatement` after this file
 * becomes authoritative.
 *
 * `statements.g4` should import this grammar and delegate:
 *
 *     statement
 *         -> controlFlowStatement
 *
 * to this file.
 *
 * ============================================================================
 */

parser grammar ControlFlow;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * IMPORTED CONTROL-FLOW COMPONENTS
 * ============================================================================
 *
 * These imports establish composition without duplicating concrete grammar.
 *
 * The imported grammars remain individually responsible for their own syntax.
 * ============================================================================
 */

import
    ConditionalsParser,
    Loops,
    PatternMatching,
    BreakStatements,
    ContinueStatements,
    ReturnStatements,
    ExceptionsParser;


/*
 * ============================================================================
 * CANONICAL CONTROL-FLOW ENTRY POINT
 * ============================================================================
 *
 * This is the only authoritative control-flow dispatcher.
 *
 * It deliberately contains no semantic predicates.
 *
 * It also deliberately contains no target-specific alternatives.
 *
 * The parser recognizes SOURCE-LEVEL control flow only.
 *
 * ============================================================================
 */

controlFlowStatement
    : ifStatement
    | loopStatement
    | matchStatement
    | breakStatement
    | continueStatement
    | returnStatement
    | throwStatement
    | tryStatement
    ;


/*
 * ============================================================================
 * TRY STATEMENT ADAPTER
 * ============================================================================
 *
 * exceptions.g4 owns the concrete exception grammar:
 *
 *     tryCatchFinallyStatement
 *
 * The statement composition layer needs one stable control-flow name:
 *
 *     tryStatement
 *
 * This adapter provides that name without duplicating the exception syntax.
 *
 * ============================================================================
 */

tryStatement
    : tryCatchFinallyStatement
    ;


/*
 * ============================================================================
 * CONTROL-FLOW FAMILY DEFINITIONS
 * ============================================================================
 *
 * The following conceptual ownership is normative:
 *
 *     conditional control
 *         -> conditionals.g4
 *
 *     iteration
 *         -> loops.g4
 *
 *     pattern dispatch
 *         -> pattern-matching.g4
 *
 *     loop/control transfer
 *         -> breaks.g4
 *         -> continues.g4
 *
 *     function control transfer
 *         -> returns.g4
 *
 *     exceptional control transfer
 *         -> exceptions.g4
 *
 * This file must not reproduce any of those rules.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * STATEMENT / EXPRESSION SEPARATION
 * ============================================================================
 *
 * Statement-level:
 *
 *     if condition {
 *         work();
 *     }
 *
 * Expression-level:
 *
 *     let value =
 *         if condition {
 *             a
 *         } else {
 *             b
 *         };
 *
 * Statement-level `if` is owned by:
 *
 *     conditionals.g4
 *
 * Conditional expressions are owned by:
 *
 *     grammar/expressions/conditionals.g4
 *
 * This file must never import or redefine conditional-expression syntax.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * BLOCK INTEGRATION
 * ============================================================================
 *
 * Individual control-flow grammars consume the canonical block abstraction
 * exposed by:
 *
 *     grammar/statements/blocks.g4
 *
 * This file does not redefine:
 *
 *     blockExpression
 *     blockElement
 *
 * A control-flow construct therefore receives the same universal statement
 * universe as every other statement.
 *
 * Conceptually:
 *
 *     controlFlowStatement
 *          |
 *          +--> conditional
 *          |       |
 *          |       +--> canonical expression
 *          |       +--> canonical block
 *          |
 *          +--> loop
 *          |       |
 *          |       +--> canonical expression
 *          |       +--> canonical block
 *          |
 *          +--> match
 *          |       |
 *          |       +--> canonical expression/pattern system
 *          |       +--> canonical block/arm system
 *          |
 *          +--> exception
 *          |       |
 *          |       +--> canonical expression
 *          |       +--> canonical block
 *          |
 *          +--> transfer statements
 *
 * This prevents domain-specific control-flow duplication.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * EXPRESSION INTEGRATION
 * ============================================================================
 *
 * Conditions, loop expressions, match subjects, return values, throw values,
 * and guards remain owned by the canonical expression grammar.
 *
 * This file must never introduce:
 *
 *     booleanExpression
 *     classicalCondition
 *     quantumCondition
 *     gpuCondition
 *     hardwareCondition
 *     distributedCondition
 *
 * merely because the eventual computation domain differs.
 *
 * Semantic analysis determines whether a particular expression is valid in
 * the relevant context.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * CLASSICAL / QUANTUM / HDL / HARDWARE NEUTRALITY
 * ============================================================================
 *
 * Control flow is intentionally domain-neutral.
 *
 * The same control-flow structure may surround:
 *
 *     classical operations;
 *     vector/matrix/tensor operations;
 *     quantum operations;
 *     hybrid operations;
 *     HDL operations;
 *     hardware-intent operations;
 *     accelerator operations;
 *     distributed operations;
 *     AI/ML operations;
 *     networking operations;
 *     security operations;
 *     scientific computation;
 *     future domain operations.
 *
 * This file therefore must not define:
 *
 *     quantumIfStatement
 *     gpuIfStatement
 *     fpgaIfStatement
 *     cpuLoopStatement
 *     qpuLoopStatement
 *     clusterMatchStatement
 *
 * solely because the eventual target differs.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A control-flow statement may surround or otherwise control quantum
 * operations.
 *
 * Example conceptual source:
 *
 *     if ready {
 *         measure(q);
 *         apply operation(parameter) to q;
 *     }
 *
 * The grammar does not determine:
 *
 *     qubit count;
 *     logical qubit count;
 *     physical qubit mapping;
 *     gate inventory;
 *     topology;
 *     calibration;
 *     pulse scheduling;
 *     routing;
 *     decomposition;
 *     QEC;
 *     ZQN;
 *     HAL;
 *     target device.
 *
 * The downstream pipeline remains:
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
 *     semantic analysis
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     decomposition/routing
 *       |
 *       v
 *     scheduling
 *       |
 *       v
 *     QEC / resilience / ZQN
 *       |
 *       v
 *     HAL
 *       |
 *       v
 *     target realization
 *
 * This grammar participates only in the first parser stage.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Control flow may appear in hardware/software co-design and HDL-oriented
 * source where the relevant domain grammars permit it.
 *
 * This composition layer does not encode:
 *
 *     bus width;
 *     register count;
 *     clock count;
 *     FPGA resource count;
 *     ASIC dimensions;
 *     memory capacity;
 *     physical addresses;
 *     device topology.
 *
 * Such information belongs to:
 *
 *     grammar/hdl/
 *     grammar/hardware/
 *     grammar/resources/
 *
 * and downstream semantic/compiler layers.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * CONCURRENCY / DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Control flow may surround concurrent or distributed operations.
 *
 * This grammar does not impose:
 *
 *     maximum task count;
 *     maximum worker count;
 *     maximum thread count;
 *     maximum process count;
 *     maximum node count;
 *     maximum channel count;
 *     maximum device count.
 *
 * Those are resource/runtime concerns.
 *
 * The source grammar describes semantic relationships only.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * RESOURCE AND CAPABILITY INDEPENDENCE
 * ============================================================================
 *
 * No control-flow grammar rule may contain machine-size constants.
 *
 * Forbidden architectural concepts include:
 *
 *     MAX_BRANCHES
 *     MAX_NESTING
 *     MAX_LOOPS
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *
 * Likewise, the grammar must not encode physical identifiers such as:
 *
 *     cpu0
 *     gpu0
 *     qpu0
 *     qubit0
 *     node0
 *
 * as language-level hardware resources.
 *
 * Ordinary user identifiers with such spelling remain ordinary identifiers.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * The grammar uses recursive/repetitive structure supplied by its component
 * grammars.
 *
 * It imposes no semantic upper bound on:
 *
 *     number of conditional branches;
 *     number of nested control-flow constructs;
 *     number of loop iterations;
 *     number of match arms;
 *     number of handlers;
 *     source-program size.
 *
 * Practical parser resource limits MAY exist for:
 *
 *     memory protection;
 *     denial-of-service protection;
 *     stack protection;
 *     implementation resource management.
 *
 * Such limits are external implementation policies.
 *
 * They MUST NOT be represented as language-semantic maxima in this grammar.
 *
 * Therefore:
 *
 *     tiny program
 *          |
 *          v
 *     same grammar
 *          |
 *          v
 *     very large program
 *          |
 *          v
 *     arbitrarily scalable source structure
 *
 * subject only to available implementation resources.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This composition grammar has no semantic actions and no external state.
 *
 * Parsing the same token stream under the same grammar/parser version must
 * produce the same structural parse.
 *
 * Parsing must not depend on:
 *
 *     CPU count;
 *     GPU availability;
 *     QPU availability;
 *     FPGA availability;
 *     memory size;
 *     network topology;
 *     calibration state;
 *     scheduler state;
 *     runtime state;
 *     deployment state.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * ERROR RECOVERY
 * ============================================================================
 *
 * This file deliberately contains no embedded error actions.
 *
 * Parser diagnostics remain owned by the parser/frontend diagnostic layer.
 *
 * Expected diagnostics include recognition of malformed control-flow structure
 * such as:
 *
 *     if
 *     if condition
 *     else
 *     else if
 *     while
 *     for
 *     match
 *     break
 *     continue
 *     return
 *     try
 *     throw
 *
 * with missing required structural components.
 *
 * The parser should preserve source spans for the failing construct.
 *
 * This grammar must not encode human-language diagnostic strings.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This file produces parser contexts only.
 *
 * It must not define Rust AST structures.
 *
 * The frontend AST layer must preserve:
 *
 *     - statement kind;
 *     - source span;
 *     - source order;
 *     - child relationships;
 *     - branch ordering;
 *     - loop structure;
 *     - match-arm ordering;
 *     - transfer targets where syntactically present;
 *     - exception handler ordering.
 *
 * IMPORTANT:
 *
 * The current repository AST has:
 *
 *     While
 *     For
 *     Match
 *     Break
 *     Continue
 *     Return
 *
 * but does not currently expose a dedicated statement-level `If` variant.
 *
 * Therefore this grammar establishes the parser contract, but frontend AST
 * integration must add the corresponding canonical statement representation
 * before statement-level `if` can be considered fully implementation-complete.
 *
 * That AST change belongs in:
 *
 *     src/ast/mod.rs
 *
 * and is NOT embedded in this grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Syntactic recognition does not imply semantic validity.
 *
 * Examples:
 *
 *     break outside a loop
 *     continue outside a loop
 *     return outside a function
 *     invalid return type
 *     invalid condition type
 *     invalid loop binding
 *     non-exhaustive match
 *     unreachable match arm
 *     invalid exception handler
 *     invalid capability use
 *     invalid resource requirement
 *
 * are semantic/control-flow-analysis concerns.
 *
 * This grammar must not attempt to encode arbitrary nesting or semantic
 * context using grammar duplication.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * CONTROL-FLOW GRAPH CONTRACT
 * ============================================================================
 *
 * This grammar does not construct a CFG.
 *
 * After AST and semantic analysis, downstream control-flow analysis may derive:
 *
 *     basic blocks;
 *     branch edges;
 *     loop back-edges;
 *     exceptional edges;
 *     return edges;
 *     break edges;
 *     continue edges;
 *     reachability;
 *     dominance;
 *     post-dominance;
 *     termination properties.
 *
 * None of those representations belong in the parser grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * EFFECTS / OWNERSHIP / TYPES
 * ============================================================================
 *
 * Control-flow syntax interacts with:
 *
 *     types;
 *     effects;
 *     ownership;
 *     borrowing;
 *     capabilities;
 *     resources;
 *     determinism;
 *     concurrency.
 *
 * The grammar only establishes structural syntax.
 *
 * Semantic analysis performs the corresponding checks.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * MACRO / METAPROGRAMMING CONTRACT
 * ============================================================================
 *
 * Macro expansion may generate control-flow syntax only by passing the
 * generated source/token/tree representation through the canonical parser and
 * semantic pipeline.
 *
 * Macro expansion must not bypass:
 *
 *     syntax validation;
 *     AST construction;
 *     semantic validation;
 *     type checking;
 *     effect checking;
 *     capability checking;
 *     resource validation.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * DIALECT CONTRACT
 * ============================================================================
 *
 * Dialects may extend control flow only through the repository's declared
 * dialect mechanism.
 *
 * A dialect must not silently redefine:
 *
 *     controlFlowStatement
 *     ifStatement
 *     loopStatement
 *     matchStatement
 *
 * or change core precedence/associativity.
 *
 * A dialect extension must declare:
 *
 *     dialect identity;
 *     version;
 *     feature gate;
 *     grammar extension;
 *     AST mapping;
 *     semantic mapping;
 *     IR mapping;
 *     compatibility behavior;
 *     positive tests;
 *     negative tests;
 *     boundary tests;
 *     scalability tests.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * INTEROPERABILITY CONTRACT
 * ============================================================================
 *
 * External formats may contain control flow.
 *
 * Examples:
 *
 *     C
 *     C++
 *     Rust
 *     Python
 *     WebAssembly
 *     QIR
 *     OpenQASM
 *     HDL formats
 *     other future formats
 *
 * Imported source syntax must be translated into Zamani's canonical
 * domain-neutral AST/semantic model.
 *
 * External control-flow syntax must not become a competing Zamani control-flow
 * grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * LEGACY COMPATIBILITY
 * ============================================================================
 *
 * The existing repository architecture already defines:
 *
 *     statements.g4
 *         -> controlFlowStatement
 *
 * and currently contains the following alternatives:
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
 * This file preserves that public composition contract.
 *
 * Migration therefore does NOT require source-language renaming.
 *
 * Required structural migration:
 *
 *     OLD:
 *
 *         statements.g4
 *             -> owns controlFlowStatement
 *
 *     NEW:
 *
 *         control-flow.g4
 *             -> owns controlFlowStatement
 *
 *         statements.g4
 *             -> imports ControlFlow
 *             -> delegates to controlFlowStatement
 *
 * Concrete statement rules remain in their existing files.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * REQUIRED INTEGRATION WITH statements.g4
 * ============================================================================
 *
 * After this file is introduced, statements.g4 should change its import list
 * from individual control-flow components to the single composition grammar.
 *
 * Replace the control-flow-related imports:
 *
 *     ConditionalsParser,
 *     Loops,
 *     PatternMatching,
 *     BreakStatements,
 *     ContinueStatements,
 *     ExceptionsParser,
 *     ReturnStatements,
 *
 * with:
 *
 *     ControlFlow,
 *
 * The statement dispatcher remains:
 *
 *     statement
 *         : declarationStatement
 *         | assignmentStatement
 *         | assertionStatement
 *         | controlFlowStatement
 *         | unsafeStatement
 *         | blockExpression
 *         | emptyStatement
 *         | expressionStatement
 *         ;
 *
 * BUT the following local rule MUST be removed from statements.g4:
 *
 *     controlFlowStatement
 *         : ifStatement
 *         | loopStatement
 *         | matchStatement
 *         | breakStatement
 *         | continueStatement
 *         | returnStatement
 *         | throwStatement
 *         | tryStatement
 *         ;
 *
 * Likewise remove the local:
 *
 *     tryStatement
 *         : tryCatchFinallyStatement
 *         ;
 *
 * because both now belong to ControlFlow.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * NO NEW ROOT GRAMMAR
 * ============================================================================
 *
 * This file is NOT a new root grammar.
 *
 * It is subordinate to:
 *
 *     grammar/Zamani.g4
 *
 * through the canonical parser composition hierarchy.
 *
 * It must not become an independently competing language entry point.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * HANDWRITTEN RUST FRONTEND CONTRACT
 * ============================================================================
 *
 * The repository currently contains a handwritten parser in:
 *
 *     src/parser.rs
 *
 * Its current statement dispatcher recognizes:
 *
 *     let/var
 *     const
 *     fn
 *     return
 *     break
 *     continue
 *     while
 *     for
 *     match
 *     declarations
 *     quantum constructs
 *     exceptions/handlers
 *     unsafe
 *     other existing Zamani constructs
 *
 * but does not currently dispatch:
 *
 *     KeywordIf
 *
 * as a statement-level `if`.
 *
 * Therefore this grammar file cannot, by itself, make the handwritten Rust
 * frontend production-complete.
 *
 * The frontend integration work must:
 *
 *     1. add statement-level `if` parsing;
 *     2. preserve the distinction between statement-level and expression-level
 *        `if`;
 *     3. construct the canonical AST representation;
 *     4. preserve source spans;
 *     5. preserve ordered else-if branches;
 *     6. preserve optional else branch;
 *     7. feed semantic analysis rather than constructing IR directly.
 *
 * This work belongs to src/parser.rs and src/ast/mod.rs.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * RUST SAFETY CONTRACT
 * ============================================================================
 *
 * Nothing in this grammar requires Rust `unsafe`.
 *
 * The fact that Zamani may expose a source-language `unsafe` construct elsewhere
 * does not authorize unsafe Rust in the compiler implementation.
 *
 * Rust implementation requirements:
 *
 *     Rust 1.97 / Rust 1.97.1
 *     safe Rust only
 *     no unsafe blocks
 *     no unsafe traits
 *     no unsafe functions
 *     no unsafe extern blocks
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
 *
 *     maximum branch count;
 *     maximum loop count;
 *     maximum nesting depth;
 *     maximum match-arm count;
 *     maximum handler count;
 *     CPU count;
 *     core count;
 *     thread count;
 *     GPU count;
 *     FPGA count;
 *     QPU count;
 *     qubit count;
 *     memory size;
 *     register width;
 *     tensor dimension;
 *     node count;
 *     device count;
 *     topology size.
 *
 * Repetition and recursion are used where the concrete control-flow grammars
 * require arbitrary cardinality.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * The following tests are required for this composition layer.
 *
 * POSITIVE
 * --------
 *
 *     if condition {}
 *
 *     if condition {} else {}
 *
 *     if a {} else if b {} else {}
 *
 *     while condition {}
 *
 *     do {} while condition
 *
 *     for item in iterable {}
 *
 *     for ; ; {}
 *
 *     match value { ... }
 *
 *     break;
 *
 *     continue;
 *
 *     return value;
 *
 *     return;
 *
 *     throw error;
 *
 *     try {} catch (...) {}
 *
 *     try {} finally {}
 *
 *     try {} catch (...) {} finally {}
 *
 * NEGATIVE
 * --------
 *
 *     if {}
 *
 *     if condition
 *
 *     else {}
 *
 *     else if condition {}
 *
 *     while
 *
 *     for
 *
 *     match
 *
 *     break
 *
 *     continue
 *
 *     return
 *
 *     try
 *
 *     catch
 *
 *     finally
 *
 * These negative tests are grammar-context tests. Semantic invalidity remains
 * the responsibility of semantic analysis.
 *
 * BOUNDARY
 * --------
 *
 * Test:
 *
 *     - long else-if chains;
 *     - deeply nested control flow;
 *     - large match structures;
 *     - large loop bodies;
 *     - large handler structures;
 *     - large source units.
 *
 * SCALABILITY
 * ----------
 *
 * Verify that grammar behavior does not depend on artificial constants.
 *
 * CROSS-DOMAIN
 * ------------
 *
 * Verify control flow containing:
 *
 *     classical operations;
 *     quantum operations;
 *     hybrid operations;
 *     HDL operations;
 *     hardware intent;
 *     distributed operations;
 *     AI/data operations;
 *     networking operations;
 *     accelerator operations.
 *
 * DETERMINISM
 * -----------
 *
 * Parse identical source repeatedly and verify identical parse structure.
 *
 * ROUND TRIP
 * ----------
 *
 * Verify:
 *
 *     source
 *       -> lexer
 *       -> parser
 *       -> AST
 *       -> formatter
 *       -> parser
 *
 * preserves control-flow structure and source ordering.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE when:
 *
 * [ ] ControlFlow is the sole owner of controlFlowStatement composition.
 *
 * [ ] ControlFlow is the sole owner of tryStatement composition.
 *
 * [ ] Concrete conditional syntax remains in conditionals.g4.
 *
 * [ ] Concrete loop syntax remains in loops.g4.
 *
 * [ ] Concrete match syntax remains in pattern-matching.g4.
 *
 * [ ] Break syntax remains in breaks.g4.
 *
 * [ ] Continue syntax remains in continues.g4.
 *
 * [ ] Return syntax remains in returns.g4.
 *
 * [ ] Exception syntax remains in exceptions.g4.
 *
 * [ ] statements.g4 imports ControlFlow.
 *
 * [ ] statements.g4 no longer duplicates controlFlowStatement.
 *
 * [ ] statements.g4 no longer duplicates tryStatement.
 *
 * [ ] No competing control-flow composition grammar exists.
 *
 * [ ] No lexer rules exist here.
 *
 * [ ] No embedded Rust actions exist.
 *
 * [ ] No semantic predicates exist.
 *
 * [ ] No unsafe Rust is required.
 *
 * [ ] No target-specific rules exist.
 *
 * [ ] No machine-size constants exist.
 *
 * [ ] No quantum IR is created.
 *
 * [ ] quantum::ir remains downstream.
 *
 * [ ] QEC remains downstream.
 *
 * [ ] ZQN remains downstream.
 *
 * [ ] routing remains downstream.
 *
 * [ ] scheduling remains downstream.
 *
 * [ ] hardware discovery remains downstream.
 *
 * [ ] runtime execution remains downstream.
 *
 * [ ] AST mapping is documented.
 *
 * [ ] semantic mapping is documented.
 *
 * [ ] parser diagnostics are handled by the frontend.
 *
 * [ ] positive tests exist.
 *
 * [ ] negative tests exist.
 *
 * [ ] boundary tests exist.
 *
 * [ ] scalability tests exist.
 *
 * [ ] cross-domain tests exist.
 *
 * [ ] determinism tests exist.
 *
 * [ ] round-trip tests exist.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL RULE
 * ============================================================================
 *
 *     CONTROL-FLOW GRAMMAR
 *              |
 *              v
 *        PARSE STRUCTURE
 *              |
 *              v
 *        DOMAIN-NEUTRAL AST
 *              |
 *              v
 *        SEMANTIC ANALYSIS
 *              |
 *              v
 *       CONTROL-FLOW MODEL
 *              |
 *       +------+------+----------------+
 *       |             |                |
 *       v             v                v
 *   Classical      quantum::ir     HDL/Hardware
 *       |             |                |
 *       +-------------+----------------+
 *                     |
 *                     v
 *               Optimization
 *                     |
 *              Routing/Scheduling
 *                     |
 *                     v
 *                Target Lowering
 *                     |
 *                     v
 *              Runtime / Hardware
 *
 * Control flow describes WHAT the program does.
 *
 * It does not decide WHERE or HOW a particular machine executes it.
 *
 * ============================================================================
 */
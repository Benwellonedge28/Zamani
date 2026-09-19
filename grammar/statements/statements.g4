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
 * CANONICAL STATEMENT COMPOSITION GRAMMAR
 *
 * GRAMMAR TECHNOLOGY
 * ------------------
 * ANTLR4 parser grammar
 *
 * RUST BASELINE
 * -------------
 * Rust 1.97 / Rust 1.97.1
 *
 * SAFETY
 * ------
 * This grammar:
 *
 *   - contains no embedded Rust actions;
 *   - contains no semantic predicates;
 *   - contains no unsafe Rust;
 *   - performs no I/O;
 *   - performs no filesystem access;
 *   - performs no networking;
 *   - performs no process execution;
 *   - performs no hardware discovery;
 *   - performs no runtime execution;
 *   - contains no mutable global state;
 *   - contains no target-specific implementation;
 *   - contains no machine-size constants.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the SINGLE AUTHORITATIVE COMPOSITION OWNER for the Zamani
 * statement grammar.
 *
 * It owns the language-wide statement entry point:
 *
 *     statement
 *
 * and composes statement families owned by dedicated grammar files.
 *
 * It does NOT implement those statement families itself.
 *
 * Concrete statement syntax remains owned by:
 *
 *     assignments.g4
 *     assertions.g4
 *     blocks.g4
 *     breaks.g4
 *     conditionals.g4
 *     continues.g4
 *     declarations.g4
 *     exceptions.g4
 *     loops.g4
 *     pattern-matching.g4
 *     returns.g4
 *     unsafe.g4
 *
 * Expression syntax remains owned by:
 *
 *     grammar/expressions/
 *
 * Type syntax remains owned by:
 *
 *     grammar/types/
 *
 * Names and shared source-language infrastructure remain owned by:
 *
 *     grammar/core/
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
 *     parser
 *          |
 *          v
 *     statement composition                 <-- THIS FILE
 *          |
 *          +--> declaration statements
 *          +--> assignment statements
 *          +--> assertion statements
 *          +--> control-flow statements
 *          +--> unsafe statements
 *          +--> block statements
 *          +--> empty statements
 *          +--> expression statements
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> names
 *          +--> types
 *          +--> effects
 *          +--> capabilities
 *          +--> ownership
 *          +--> control flow
 *          +--> resources
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          +--> classical representation
 *          +--> quantum::ir
 *          +--> HDL / hardware representation
 *          +--> distributed representation
 *          +--> accelerator representation
 *          +--> future-domain representation
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing / scheduling / lowering
 *          |
 *          v
 *     target realization
 *          |
 *          v
 *     runtime / hardware
 *
 * This grammar MUST NOT bypass this pipeline.
 *
 * ============================================================================
 * OWNERSHIP CONTRACT
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 *   - statement
 *   - statement-family dispatch
 *   - control-flow composition
 *   - generic expression-statement admission
 *   - empty-statement admission
 *   - composition adapters required to connect delegated grammars
 *
 * THIS FILE DOES NOT OWN
 * ----------------------
 *
 *   - lexer rules
 *   - token spelling
 *   - identifiers
 *   - names
 *   - paths
 *   - expressions
 *   - expression precedence
 *   - assignment-expression semantics
 *   - concrete assignment syntax
 *   - declarations
 *   - functions
 *   - modules
 *   - types
 *   - blocks
 *   - conditionals
 *   - loops
 *   - pattern matching
 *   - break
 *   - continue
 *   - return
 *   - exceptions
 *   - unsafe-region syntax
 *   - assertion syntax
 *   - quantum operations
 *   - HDL operations
 *   - hardware realization
 *   - resource allocation
 *   - capability evaluation
 *   - QEC
 *   - ZQN
 *   - resilience
 *   - optimization
 *   - routing
 *   - scheduling
 *   - calibration
 *   - target selection
 *   - runtime execution
 *   - physical device selection
 *
 * ============================================================================
 * SINGLE-SOURCE-OF-TRUTH CONTRACT
 * ============================================================================
 *
 * There MUST be exactly ONE authoritative effective `statement` rule in the
 * assembled Zamani parser.
 *
 * This file is that owner within the modular statement grammar.
 *
 * No imported delegate grammar may redefine:
 *
 *     statement
 *
 * No legacy parser composition layer may remain in the authoritative generated
 * parser with a competing statement definition.
 *
 * In particular, the migration path must eventually remove/deactivate competing
 * statement productions from:
 *
 *     grammar/antlr/ZamaniParser.g4
 *     grammar/antlr/Core.g4
 *
 * when those productions overlap this modular owner.
 *
 * The repository may retain legacy definitions temporarily for migration or
 * compatibility analysis, but they must not participate as competing
 * authoritative definitions in production parser generation.
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * Each imported grammar owns a distinct part of the statement language.
 *
 * This file composes those owners.
 *
 * It MUST NOT copy their concrete productions.
 *
 * ============================================================================
 * EXPRESSION / ASSIGNMENT CONTRACT
 * ============================================================================
 *
 * Zamani has both:
 *
 *     assignmentExpression
 *
 * and:
 *
 *     assignmentStatement
 *
 * They are intentionally distinct.
 *
 * assignmentExpression
 *     -> owned by grammar/expressions/
 *
 * assignmentStatement
 *     -> owned by assignments.g4
 *
 * This dispatcher explicitly admits assignmentStatement before the generic
 * expression statement.
 *
 * This is important because:
 *
 *     expressionStatement
 *         : expression SEMICOLON
 *
 * alone is not a sufficient composition contract for the dedicated
 * assignmentStatement rule.
 *
 * The assignment grammar already defines:
 *
 *     assignmentStatement
 *         : assignmentOperation statementTerminator
 *
 * and:
 *
 *     assignmentOperation
 *         : assignmentTarget assignmentOperator assignmentExpression
 *
 * Therefore this file must consume that canonical rule instead of recreating
 * assignment syntax.
 *
 * ============================================================================
 * BLOCK CONTRACT
 * ============================================================================
 *
 * Block syntax is owned by:
 *
 *     blocks.g4
 *
 * This file MUST NOT define:
 *
 *     {
 *     }
 *     blockExpression
 *     blockElement
 *
 * A block recursively contains the canonical `statement` rule.
 *
 * This creates the intentional grammar relationship:
 *
 *     statement
 *         |
 *         v
 *     blockExpression
 *         |
 *         v
 *     blockElement*
 *         |
 *         v
 *     statement
 *
 * This is grammar recursion, not a semantic subsystem dependency cycle.
 *
 * ============================================================================
 * CONTROL-FLOW CONTRACT
 * ============================================================================
 *
 * Control-flow syntax is delegated to:
 *
 *     conditionals.g4
 *     loops.g4
 *     pattern-matching.g4
 *     breaks.g4
 *     continues.g4
 *     returns.g4
 *     exceptions.g4
 *
 * This file merely composes them.
 *
 * No finite limit is imposed on:
 *
 *     - branches;
 *     - loop nesting;
 *     - match arms;
 *     - catch clauses;
 *     - statement nesting;
 *     - block nesting;
 *     - program size.
 *
 * ============================================================================
 * DECLARATION CONTRACT
 * ============================================================================
 *
 * declarations.g4 owns:
 *
 *     declarationStatement
 *
 * including local declarations and declaration-family dispatch.
 *
 * This file does not duplicate declaration syntax.
 *
 * ============================================================================
 * ASSERTION CONTRACT
 * ============================================================================
 *
 * assertions.g4 owns:
 *
 *     assertionStatement
 *
 * This file merely admits it in statement position.
 *
 * Proof checking, theorem proving, compile-time evaluation and runtime
 * assertion policy remain downstream concerns.
 *
 * ============================================================================
 * UNSAFE CONTRACT
 * ============================================================================
 *
 * unsafe.g4 owns:
 *
 *     unsafeStatement
 *
 * The presence of this rule does NOT mean the grammar implementation uses
 * Rust unsafe.
 *
 * `unsafe` is a Zamani source-language construct whose semantics are determined
 * downstream.
 *
 * The Rust implementation remains required to use safe Rust.
 *
 * ============================================================================
 * EXCEPTION CONTRACT
 * ============================================================================
 *
 * exceptions.g4 owns:
 *
 *     throwStatement
 *     tryCatchFinallyStatement
 *     catchClause
 *     finallyClause
 *
 * This dispatcher provides the stable:
 *
 *     tryStatement
 *
 * adapter so that the exception implementation remains replaceable without
 * changing the overall statement-dispatch contract.
 *
 * A try statement is syntactically valid only when its delegated grammar
 * supplies a catch and/or finally handler according to exceptions.g4.
 *
 * ============================================================================
 * EMPTY STATEMENT CONTRACT
 * ============================================================================
 *
 * A standalone semicolon is a syntactically valid empty statement:
 *
 *     ;
 *
 * It is deliberately distinct from:
 *
 *     expressionStatement
 *
 * because an empty statement contains no expression.
 *
 * Whether an empty statement produces a warning, lint, style diagnostic, or
 * optimization is downstream policy.
 *
 * ============================================================================
 * GENERIC EXPRESSION STATEMENT CONTRACT
 * ============================================================================
 *
 * Any canonical expression may occupy statement position when followed by the
 * canonical statement terminator.
 *
 * Examples include:
 *
 *     compute();
 *     value;
 *     foo();
 *     measure(q);
 *     tensor_operation();
 *     distributed_operation();
 *     hardware_operation();
 *
 * Domain-specific meaning is determined downstream.
 *
 * This file MUST NOT enumerate:
 *
 *     quantum operations;
 *     GPU operations;
 *     CPU operations;
 *     FPGA operations;
 *     AI operations;
 *     networking operations;
 *     vendor operations.
 *
 * ============================================================================
 * DOMAIN-NEUTRALITY CONTRACT
 * ============================================================================
 *
 * The same statement composition must work across:
 *
 *     classical computing
 *     quantum computing
 *     hybrid computing
 *     HDL
 *     hardware/software co-design
 *     embedded computing
 *     parallel computing
 *     distributed computing
 *     HPC
 *     AI/ML
 *     accelerator programming
 *     networking
 *     scientific computing
 *     systems programming
 *     cryptography
 *     data processing
 *     future computational domains
 *
 * This dispatcher therefore MUST NOT introduce target-specific alternatives
 * such as:
 *
 *     cpuStatement
 *     gpuStatement
 *     fpgaStatement
 *     qpuStatement
 *     asicStatement
 *     clusterStatement
 *
 * merely because the final target differs.
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * This file defines NO quantum operation.
 *
 * Quantum syntax is owned by the quantum grammar hierarchy.
 *
 * The statement layer therefore has no knowledge of:
 *
 *     - qubit count;
 *     - logical-qubit count;
 *     - physical-qubit count;
 *     - gate inventory;
 *     - topology;
 *     - calibration;
 *     - physical mapping;
 *     - QEC strategy;
 *     - ZQN noise model;
 *     - routing;
 *     - scheduling;
 *     - backend selection.
 *
 * The canonical downstream path remains:
 *
 *     statement AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum semantic model
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
 *     QEC / resilience / ZQN / HAL
 *          |
 *          v
 *     target realization
 *
 * ============================================================================
 * HARDWARE / HDL CONTRACT
 * ============================================================================
 *
 * This file does not encode:
 *
 *     - CPU count;
 *     - core count;
 *     - thread count;
 *     - GPU count;
 *     - FPGA count;
 *     - register width;
 *     - memory capacity;
 *     - accelerator count;
 *     - bus width;
 *     - topology;
 *     - device identity;
 *     - clock count;
 *     - pipeline depth.
 *
 * Hardware and HDL intent belongs to their respective grammar and semantic
 * layers.
 *
 * ============================================================================
 * DISTRIBUTED / CONCURRENT CONTRACT
 * ============================================================================
 *
 * Statement composition does not impose any finite limit on:
 *
 *     - tasks;
 *     - actors;
 *     - processes;
 *     - workers;
 *     - channels;
 *     - nodes;
 *     - devices;
 *     - communication paths.
 *
 * Those are source semantics, resource requirements, capability constraints,
 * compiler decisions, runtime decisions, or target properties.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * The statement grammar expresses portable source-level computation.
 *
 * It MUST remain independent of the eventual machine scale.
 *
 * A valid source program can therefore contain statements that ultimately
 * execute on:
 *
 *     - a tiny processor;
 *     - a single CPU;
 *     - many CPUs;
 *     - GPUs;
 *     - FPGAs;
 *     - ASICs;
 *     - QPUs;
 *     - simulators;
 *     - heterogeneous machines;
 *     - clusters;
 *     - supercomputers;
 *     - cloud systems;
 *     - future architectures.
 *
 * No grammar-level statement limit is used to represent machine capability.
 *
 * Practical parser/compiler limits remain implementation/resource policies and
 * are not language semantics.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no actions;
 *     - no semantic predicates;
 *     - no randomness;
 *     - no time dependence;
 *     - no filesystem dependence;
 *     - no network dependence;
 *     - no hardware dependence;
 *     - no runtime dependence;
 *     - no scheduler dependence;
 *     - no backend dependence;
 *     - no mutable global state.
 *
 * Given identical source tokens, grammar version, lexer vocabulary and parser
 * configuration, the structural parse result must be deterministic.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser/frontend diagnostics are responsible for syntax failures such as:
 *
 *     - malformed statement;
 *     - unexpected token;
 *     - missing expression;
 *     - missing semicolon;
 *     - malformed declaration;
 *     - malformed assignment;
 *     - malformed conditional;
 *     - malformed loop;
 *     - malformed match;
 *     - malformed exception;
 *     - malformed block.
 *
 * Semantic diagnostics remain downstream.
 *
 * Examples:
 *
 *     break outside loop;
 *     continue outside loop;
 *     return outside callable;
 *     invalid type;
 *     invalid ownership;
 *     unavailable capability;
 *     impossible resource requirement;
 *     invalid quantum operation;
 *     invalid hardware requirement.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates no Rust AST values.
 *
 * The frontend AST layer must map each statement context to the repository's
 * canonical domain-neutral AST representation.
 *
 * Every statement node must preserve enough information for:
 *
 *     - source span;
 *     - source ordering;
 *     - statement kind;
 *     - child relationships;
 *     - relevant syntactic metadata.
 *
 * Target-specific information must not be injected merely because a statement
 * may later lower to a particular backend.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates no IR.
 *
 * The statement layer must remain upstream of all canonical IRs.
 *
 * The downstream architecture is:
 *
 *     source
 *       -> lexer
 *       -> parser
 *       -> AST
 *       -> semantic analysis
 *       -> canonical semantic representation
 *       -> domain IR
 *       -> optimization
 *       -> routing / scheduling / lowering
 *       -> target
 *
 * For quantum:
 *
 *     AST
 *       -> semantic quantum representation
 *       -> quantum::ir
 *
 * No second quantum IR is introduced here.
 *
 * ============================================================================
 * VERSIONING CONTRACT
 * ============================================================================
 *
 * This file is intentionally a stable composition boundary.
 *
 * Adding a new statement family requires:
 *
 *     1. a dedicated grammar owner;
 *     2. an explicit import here;
 *     3. an explicit composition alternative here;
 *     4. AST integration;
 *     5. semantic integration;
 *     6. IR integration where applicable;
 *     7. compiler/runtime integration where applicable;
 *     8. positive tests;
 *     9. negative tests;
 *    10. boundary/scalability tests;
 *    11. compatibility documentation.
 *
 * Existing syntax must not be silently redefined.
 *
 * ============================================================================
 * RUST INTEGRATION
 * ============================================================================
 *
 * This grammar generates parser code consumed by the Zamani Rust frontend.
 *
 * Compatibility target:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * The generated/consuming implementation must remain safe Rust.
 *
 * This grammar itself has no Rust implementation actions and therefore cannot
 * introduce Rust unsafe code.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE
 * --------
 *
 *     ;
 *
 *     expression;
 *
 *     assignment = expression;
 *
 *     let x = expression;
 *
 *     if condition { }
 *
 *     while condition { }
 *
 *     for item in iterable { }
 *
 *     match value {
 *         pattern => expression,
 *     }
 *
 *     break;
 *
 *     continue;
 *
 *     return;
 *
 *     return expression;
 *
 *     throw expression;
 *
 *     try {
 *         work();
 *     } catch (error) {
 *         recover();
 *     }
 *
 *     unsafe {
 *         operation();
 *     }
 *
 * NEGATIVE
 * --------
 *
 *     malformed statement;
 *     malformed assignment;
 *     malformed declaration;
 *     malformed conditional;
 *     malformed loop;
 *     malformed match;
 *     malformed exception;
 *     malformed block;
 *     incomplete expression;
 *     missing required delimiter;
 *
 * SEMANTIC-NEGATIVE
 * -----------------
 *
 * These must be rejected downstream rather than by this grammar:
 *
 *     break outside loop;
 *     continue outside loop;
 *     return outside callable;
 *     invalid type;
 *     invalid ownership;
 *     invalid capability;
 *     unavailable resource;
 *     invalid quantum operation;
 *     invalid target requirement.
 *
 * BOUNDARY
 * --------
 *
 * Tests must cover:
 *
 *     - empty statements;
 *     - arbitrarily long statement sequences;
 *     - deeply nested blocks;
 *     - deeply nested control flow;
 *     - long conditional chains;
 *     - many match arms;
 *     - many catch clauses;
 *     - large expressions;
 *     - large declarations;
 *     - large programs.
 *
 * CROSS-DOMAIN
 * ------------
 *
 * Tests must demonstrate statement composition containing:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     distributed
 *     AI/data
 *     networking
 *     accelerator
 *
 * constructs wherever those constructs are independently legal.
 *
 * SCALABILITY
 * -----------
 *
 * No grammar-defined finite limit may exist for:
 *
 *     - statement count;
 *     - block count;
 *     - nesting;
 *     - branches;
 *     - loop iterations;
 *     - match arms;
 *     - catch clauses;
 *     - declarations;
 *     - devices;
 *     - qubits;
 *     - CPUs;
 *     - GPUs;
 *     - FPGAs;
 *     - nodes;
 *     - memory;
 *     - accelerators.
 *
 * DETERMINISM
 * -----------
 *
 * Repeated parsing of identical input under identical parser configuration
 * must produce equivalent parse-tree structure.
 *
 * ROUND-TRIP
 * ----------
 *
 *     source
 *       -> lexer
 *       -> parser
 *       -> AST
 *       -> formatter
 *       -> parser
 *
 * must preserve statement meaning.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden in this composition grammar:
 *
 *     MAX_STATEMENTS
 *     MAX_BLOCKS
 *     MAX_BRANCHES
 *     MAX_LOOP_DEPTH
 *     MAX_MATCH_ARMS
 *     MAX_CATCHES
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
 *     MAX_TENSOR_RANK
 *
 * Numeric literals in a source program remain program semantics; they are not
 * parser-level resource limits.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when:
 *
 *     [ ] It is the sole owner of `statement`.
 *
 *     [ ] Concrete statement syntax remains in dedicated grammar owners.
 *
 *     [ ] `assignmentStatement` is explicitly reachable.
 *
 *     [ ] `emptyStatement` is explicitly reachable.
 *
 *     [ ] `declarationStatement` is explicitly reachable.
 *
 *     [ ] `assertionStatement` is explicitly reachable.
 *
 *     [ ] `controlFlowStatement` is explicitly reachable.
 *
 *     [ ] `unsafeStatement` is explicitly reachable.
 *
 *     [ ] `blockExpression` is explicitly reachable.
 *
 *     [ ] Generic `expressionStatement` is explicitly reachable.
 *
 *     [ ] `tryStatement` is only a composition adapter.
 *
 *     [ ] No concrete statement family is duplicated here.
 *
 *     [ ] All imports resolve.
 *
 *     [ ] No imported grammar redefines `statement`.
 *
 *     [ ] No competing authoritative statement rule remains in the production
 *         parser.
 *
 *     [ ] Lexer ownership remains external.
 *
 *     [ ] Expression ownership remains external.
 *
 *     [ ] Type ownership remains external.
 *
 *     [ ] Block ownership remains external.
 *
 *     [ ] AST ownership remains external.
 *
 *     [ ] Semantic validation remains downstream.
 *
 *     [ ] IR construction remains downstream.
 *
 *     [ ] `quantum::ir` remains the canonical quantum semantic boundary.
 *
 *     [ ] QEC remains downstream.
 *
 *     [ ] ZQN remains downstream.
 *
 *     [ ] Routing remains downstream.
 *
 *     [ ] Scheduling remains downstream.
 *
 *     [ ] Optimization remains downstream.
 *
 *     [ ] Hardware discovery remains downstream.
 *
 *     [ ] Runtime execution remains downstream.
 *
 *     [ ] No machine-size constants exist.
 *
 *     [ ] No target-specific statement families exist merely for hardware
 *         targets.
 *
 *     [ ] No embedded Rust actions exist.
 *
 *     [ ] No unsafe Rust is required.
 *
 *     [ ] Rust 1.97 / 1.97.1 integration passes.
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
 * CANONICAL PARSER GRAMMAR
 * ============================================================================
 */

parser grammar Statements;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * DELEGATE GRAMMAR IMPORTS
 * ============================================================================
 *
 * These grammars own their concrete productions.
 *
 * This file owns only their composition.
 * ============================================================================
 */

import
    AssertionsParser,
    Assignments,
    Blocks,
    BreakStatements,
    ConditionalsParser,
    ContinueStatements,
    Declarations,
    ExceptionsParser,
    Expressions,
    Loops,
    PatternMatching,
    ReturnStatements,
    UnsafeStatementsParser;

/*
 * ============================================================================
 * CANONICAL STATEMENT ENTRY POINT
 * ============================================================================
 *
 * There is exactly one authoritative `statement` rule here.
 *
 * Ordering is intentionally structural:
 *
 *     declaration
 *     assignment
 *     assertion
 *     control flow
 *     unsafe
 *     block
 *     empty
 *     expression
 *
 * The concrete syntax is owned by imported grammars.
 *
 * No semantic predicate is required.
 * ============================================================================
 */

statement
    : declarationStatement
    | assignmentStatement
    | assertionStatement
    | controlFlowStatement
    | unsafeStatement
    | blockExpression
    | emptyStatement
    | expressionStatement
    ;

/*
 * ============================================================================
 * CONTROL-FLOW COMPOSITION
 * ============================================================================
 *
 * Concrete control-flow syntax remains delegated.
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
 * EXCEPTION COMPOSITION ADAPTER
 * ============================================================================
 *
 * exceptions.g4 owns:
 *
 *     tryCatchFinallyStatement
 *
 * This adapter provides a stable statement-composition name without duplicating
 * exception syntax.
 * ============================================================================
 */

tryStatement
    : tryCatchFinallyStatement
    ;

/*
 * ============================================================================
 * EMPTY STATEMENT
 * ============================================================================
 *
 * A standalone semicolon is syntactically valid.
 *
 * Semantic/lint policy is downstream.
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
 * Expressions are owned by the canonical Expressions grammar.
 *
 * Assignment statements are dispatched separately above so the dedicated
 * assignmentStatement contract remains reachable and unambiguous at the
 * statement-composition level.
 *
 * ============================================================================
 */

expressionStatement
    : expression SEMICOLON
    ;
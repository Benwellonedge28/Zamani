/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/statements/continues.g4
 *
 * Status:
 *     Production parser-grammar component
 *
 * Purpose:
 *     Own the canonical source syntax for the Zamani `continue` statement.
 *
 * Architectural position:
 *
 *     Source
 *       |
 *       v
 *     Canonical Lexer
 *       |
 *       |-- K_CONTINUE
 *       |-- SEMICOLON
 *       v
 *     continues.g4
 *       |
 *       v
 *     statements.g4
 *       |
 *       v
 *     Parser
 *       |
 *       v
 *     Frontend AST
 *       |
 *       v
 *     Structural Validation
 *       |
 *       v
 *     Semantic Analysis
 *       |
 *       v
 *     Control-Flow Representation
 *       |
 *       v
 *     ZUIR / Canonical Semantic IR
 *       |
 *       +--> Classical lowering
 *       +--> Quantum lowering
 *       +--> HDL lowering
 *       +--> Hardware lowering
 *       +--> Distributed lowering
 *       +--> Future-domain lowering
 *       |
 *       v
 *     Optimization
 *       |
 *       v
 *     Scheduling / Routing / Target Lowering
 *       |
 *       v
 *     Runtime / Hardware
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - the canonical parser production for `continue`;
 *     - the required source terminator;
 *     - the syntactic boundary of the continue statement;
 *     - the parser-level continue statement node boundary.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer tokens;
 *     - identifier syntax;
 *     - labels;
 *     - loop syntax;
 *     - loop nesting;
 *     - control-flow validity;
 *     - reachability analysis;
 *     - AST storage;
 *     - semantic analysis;
 *     - type checking;
 *     - effects;
 *     - capabilities;
 *     - resources;
 *     - scheduling;
 *     - routing;
 *     - optimization;
 *     - quantum IR;
 *     - QEC;
 *     - ZQN;
 *     - hardware discovery;
 *     - calibration;
 *     - runtime execution.
 *
 * ============================================================================
 * CANONICAL LEXER CONTRACT
 * ============================================================================
 *
 * `K_CONTINUE` is owned by:
 *
 *     grammar/lexer/tokens.g4
 *
 * and represents:
 *
 *     'continue'
 *
 * `SEMICOLON` is owned by:
 *
 *     grammar/lexer/punctuation.g4
 *
 * and represents:
 *
 *     ';'
 *
 * This file MUST NOT redefine either token.
 *
 * The parser therefore consumes:
 *
 *     K_CONTINUE SEMICOLON
 *
 * rather than literal strings such as:
 *
 *     'continue' ';'
 *
 * This preserves the repository's lexer ownership boundary and prevents
 * duplicate token definitions when the modular grammar is assembled.
 *
 * ============================================================================
 * CURRENT LANGUAGE SYNTAX
 * ============================================================================
 *
 * The canonical current source form is:
 *
 *     continue;
 *
 * No expression follows `continue`.
 *
 * No implicit target follows `continue`.
 *
 * No machine-specific information is encoded.
 *
 * ============================================================================
 * LABEL POLICY
 * ============================================================================
 *
 * The repository's AST already has an optional source-level label reference
 * on ContinueStatement.
 *
 * However, the current authoritative grammar surface does not establish a
 * labeled-continue syntax.
 *
 * Therefore this grammar intentionally accepts ONLY:
 *
 *     continue;
 *
 * It MUST NOT invent syntax such as:
 *
 *     continue label;
 *
 * until labeled control flow has been formally specified by the language
 * specification and integrated with the canonical label grammar.
 *
 * The AST can therefore remain forward-compatible without prematurely
 * expanding the language syntax.
 *
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * The parser accepts:
 *
 *     continue;
 *
 * regardless of its surrounding semantic context.
 *
 * Semantic analysis determines whether the statement occurs inside an
 * applicable loop.
 *
 * For example:
 *
 *     while condition {
 *         continue;
 *     }
 *
 * may be semantically valid.
 *
 * Whereas:
 *
 *     fn compute() {
 *         continue;
 *     }
 *
 * is syntactically recognizable but semantically invalid because there is no
 * enclosing applicable loop.
 *
 * This distinction is intentional.
 *
 * The grammar MUST NOT attempt to count loop nesting or inspect enclosing
 * scopes.
 *
 * ============================================================================
 * WHY LOOP VALIDITY IS NOT PARSER LOGIC
 * ============================================================================
 *
 * Keeping loop-context validation out of this file prevents an architectural
 * dependency such as:
 *
 *     grammar -> semantic analysis
 *
 * and preserves the intended compiler direction:
 *
 *     grammar
 *        |
 *        v
 *     AST
 *        |
 *        v
 *     semantic analysis
 *        |
 *        v
 *     canonical semantic representation
 *
 * The existing ContinueStatement AST contract follows the same separation:
 * local structural validation belongs to the AST layer while legality inside
 * a loop belongs to semantic/control-flow analysis.
 *
 * ============================================================================
 * CONTROL-FLOW SEMANTICS
 * ============================================================================
 *
 * At the language level:
 *
 *     continue;
 *
 * means:
 *
 *     transfer control to the continuation point of the applicable enclosing
 *     loop.
 *
 * The exact continuation point depends on the enclosing loop construct.
 *
 * Examples include:
 *
 *     while
 *     do-while
 *     for
 *     foreach
 *     future loop forms
 *
 * This grammar does not distinguish those cases because the statement itself
 * has the same source syntax.
 *
 * The enclosing control-flow representation determines the correct target.
 *
 * ============================================================================
 * NO FIXED LOOP LIMITS
 * ============================================================================
 *
 * This grammar MUST NOT impose limits such as:
 *
 *     MAX_LOOP_DEPTH
 *     MAX_CONTINUE_COUNT
 *     MAX_STATEMENTS
 *     MAX_NESTING
 *
 * A source program can contain as many `continue` statements as permitted by
 * the available compiler resources and explicit compiler policy.
 *
 * Language syntax itself imposes no machine-size ceiling.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * The meaning of:
 *
 *     continue;
 *
 * is independent of the eventual execution substrate.
 *
 * The same source-level construct can participate in programs targeting:
 *
 *     - a tiny embedded processor;
 *     - a CPU;
 *     - a multicore CPU;
 *     - a GPU;
 *     - an FPGA;
 *     - an ASIC;
 *     - a quantum processor;
 *     - a quantum simulator;
 *     - a heterogeneous accelerator;
 *     - a distributed system;
 *     - a cluster;
 *     - a supercomputer;
 *     - a cloud environment;
 *     - future computational architectures.
 *
 * The grammar therefore describes control-flow intent rather than its
 * eventual machine implementation.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * `continue` is not a quantum operation.
 *
 * It may nevertheless occur in a loop whose body contains quantum or
 * hybrid computation.
 *
 * Example:
 *
 *     while condition {
 *         quantum_operation();
 *
 *         if condition {
 *             continue;
 *         }
 *
 *         classical_operation();
 *     }
 *
 * This file does not inspect or understand the quantum operation.
 *
 * Quantum syntax is owned by the quantum grammar.
 *
 * Quantum semantic lowering occurs downstream.
 *
 * This file MUST NOT depend directly on:
 *
 *     quantum::ir
 *
 * QEC, ZQN, routing, scheduling, hardware mapping, and backend selection are
 * all downstream concerns.
 *
 * ============================================================================
 * HARDWARE / HDL INTEGRATION
 * ============================================================================
 *
 * A `continue` statement does not mean:
 *
 *     - clock skip;
 *     - pipeline flush;
 *     - hardware branch;
 *     - state-machine transition;
 *     - processor instruction;
 *     - FPGA control signal;
 *     - ASIC control path.
 *
 * A downstream HDL or hardware lowering may choose an appropriate
 * implementation, but the parser must preserve only the source semantics.
 *
 * ============================================================================
 * DISTRIBUTED / CONCURRENT INTEGRATION
 * ============================================================================
 *
 * `continue` MUST NOT implicitly mean:
 *
 *     - cancel all tasks;
 *     - cancel all workers;
 *     - terminate a node;
 *     - terminate a service;
 *     - abort a distributed operation;
 *     - synchronize devices;
 *     - cancel sibling tasks.
 *
 * Those are separate concurrency/distributed semantics.
 *
 * `continue` only expresses structured control flow for its enclosing loop.
 *
 * ============================================================================
 * EFFECTS AND CAPABILITIES
 * ============================================================================
 *
 * This file does not assign effects or capabilities to `continue`.
 *
 * The surrounding semantic context determines whether control flow interacts
 * with constructs that have effects, resources, synchronization, or other
 * constraints.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser must lower the recognized syntax into the repository's existing
 * ContinueStatement AST representation.
 *
 * For:
 *
 *     continue;
 *
 * the AST representation is conceptually:
 *
 *     Continue {
 *         label: None
 *     }
 *
 * The grammar itself does not construct Rust values.
 *
 * The parser/frontend owns AST construction.
 *
 * ============================================================================
 * SOURCE SPAN CONTRACT
 * ============================================================================
 *
 * The resulting AST node must cover the complete source statement:
 *
 *     continue;
 *     ^^^^^^^^^
 *
 * Source-location information belongs to the common AST Node/span system.
 *
 * This grammar must not invent an independent source-location representation.
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * Parser-level malformed forms include:
 *
 *     continue
 *
 *     continue value;
 *
 *     continue value value;
 *
 *     continue();
 *
 *     continue(whatever);
 *
 *     continue [target];
 *
 *     continue label;
 *
 * when labeled continue has not been specified.
 *
 * The parser/frontend diagnostic layer owns the final diagnostic wording,
 * source span, and recovery strategy.
 *
 * This grammar must not contain embedded Rust actions to manufacture errors.
 *
 * ============================================================================
 * ERROR RECOVERY
 * ============================================================================
 *
 * The rule intentionally has no custom error-recovery action.
 *
 * Parser recovery belongs to the parser/frontend infrastructure so that all
 * statements receive consistent:
 *
 *     - synchronization;
 *     - diagnostic;
 *     - source-span;
 *     - recovery;
 *     - error-budget;
 *     - determinism
 *
 * behavior.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar rule:
 *
 *     - contains no actions;
 *     - contains no semantic predicates;
 *     - contains no mutable global state;
 *     - performs no I/O;
 *     - performs no filesystem access;
 *     - performs no networking;
 *     - performs no hardware discovery;
 *     - performs no randomness;
 *     - performs no time-dependent behavior.
 *
 * Therefore identical token streams produce identical syntactic results.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar introduces no:
 *
 *     - unsafe Rust;
 *     - raw pointers;
 *     - filesystem access;
 *     - network access;
 *     - command execution;
 *     - environment-variable reads;
 *     - hardware discovery;
 *     - dynamic code execution.
 *
 * Rust 1.97 / 1.97.1 consumers must continue to compile with unsafe code
 * forbidden.
 *
 * ============================================================================
 * INTEGRATION WITH STATEMENTS.G4
 * ============================================================================
 *
 * `statements.g4` is the composition owner.
 *
 * It should reference this rule through:
 *
 *     continueStatement
 *
 * and MUST NOT contain another independent implementation of:
 *
 *     continueStatement
 *
 * The authoritative modular dependency is:
 *
 *     statements.g4
 *          |
 *          +--> breaks.g4
 *          |
 *          +--> continues.g4
 *          |
 *          +--> loops.g4
 *          |
 *          +--> returns.g4
 *          |
 *          +--> conditionals.g4
 *          |
 *          +--> pattern-matching.g4
 *          |
 *          v
 *     canonical statement grammar
 *
 * `continues.g4` owns the continue production.
 *
 * `loops.g4` owns loop syntax.
 *
 * Neither file should duplicate the other's semantics.
 *
 * ============================================================================
 * INTEGRATION WITH LOOPS.G4
 * ============================================================================
 *
 * `loops.g4` owns the syntax of:
 *
 *     while
 *     do-while
 *     for
 *     foreach
 *     parallel/future loop constructs
 *
 * where those constructs are part of the authoritative language specification.
 *
 * It MUST NOT redefine:
 *
 *     continueStatement
 *
 * Instead, loop syntax simply admits the general `statement` production in
 * its body.
 *
 * Semantic analysis later establishes that:
 *
 *     ContinueStatement
 *
 * is enclosed by a valid loop.
 *
 * ============================================================================
 * INTEGRATION WITH BREAKS.G4
 * ============================================================================
 *
 * `break` and `continue` are sibling control-transfer statements.
 *
 * Their grammar components should remain separate:
 *
 *     breaks.g4
 *     continues.g4
 *
 * They share:
 *
 *     - lexer ownership;
 *     - statement composition;
 *     - semantic control-flow analysis;
 *     - AST architecture;
 *     - POCO-REAF principles.
 *
 * They must not share a generic parser production merely to reduce file count.
 *
 * This preserves independent evolution.
 *
 * ============================================================================
 * INTEGRATION WITH AST
 * ============================================================================
 *
 * Existing repository AST support already provides:
 *
 *     ContinueStatement
 *
 * with an optional label field.
 *
 * The current grammar intentionally constructs the unlabeled case only.
 *
 * The parser should use the AST's canonical constructor for an unlabeled
 * continue node rather than creating a competing representation.
 *
 * ============================================================================
 * INTEGRATION WITH SEMANTIC ANALYSIS
 * ============================================================================
 *
 * Semantic analysis consumes the AST and determines:
 *
 *     - whether an enclosing loop exists;
 *     - which loop is targeted;
 *     - whether control-flow transfer is legal;
 *     - whether the resulting control-flow graph remains valid;
 *     - whether unreachable/reachable regions must be updated;
 *     - whether other semantic rules are affected.
 *
 * None of those checks belong in this grammar.
 *
 * ============================================================================
 * INTEGRATION WITH IR
 * ============================================================================
 *
 * This file does not directly create IR.
 *
 * The intended flow is:
 *
 *     continue;
 *         |
 *         v
 *     parser AST
 *         |
 *         v
 *     semantic control-flow representation
 *         |
 *         v
 *     canonical semantic IR
 *
 * For quantum programs, any eventual quantum representation must be reached
 * through the repository's canonical quantum semantic boundary.
 *
 * This grammar MUST NOT import or depend directly on `quantum::ir`.
 *
 * ============================================================================
 * INTEGRATION WITH OPTIMIZATION
 * ============================================================================
 *
 * Optimization may later:
 *
 *     - simplify control flow;
 *     - remove unreachable code;
 *     - merge compatible control-flow paths;
 *     - lower or transform loop structure.
 *
 * Such transformations MUST preserve the source-level meaning of `continue`.
 *
 * None of those transformations belong in this file.
 *
 * ============================================================================
 * INTEGRATION WITH SCHEDULING
 * ============================================================================
 *
 * Scheduling sees downstream semantic operations rather than parser rules.
 *
 * A `continue` does not directly specify:
 *
 *     - a timestamp;
 *     - a duration;
 *     - a scheduling slot;
 *     - a resource;
 *     - a device;
 *     - a topology.
 *
 * Therefore this grammar has no scheduling dependency.
 *
 * ============================================================================
 * INTEGRATION WITH HARDWARE
 * ============================================================================
 *
 * Hardware capability discovery occurs after parsing and semantic analysis.
 *
 * This file must not depend on:
 *
 *     CPU count
 *     GPU count
 *     FPGA count
 *     ASIC topology
 *     QPU topology
 *     memory capacity
 *     device identifiers
 *     physical addresses
 *
 * ============================================================================
 * INTEGRATION WITH QEC / ZQN
 * ============================================================================
 *
 * There is no direct dependency.
 *
 * QEC and ZQN describe downstream quantum reliability/fault semantics.
 *
 * A source-level `continue` statement does not itself describe:
 *
 *     - a quantum error;
 *     - a correction;
 *     - noise;
 *     - mitigation;
 *     - fault;
 *     - recovery action.
 *
 * ============================================================================
 * INTEGRATION WITH RUNTIME
 * ============================================================================
 *
 * Runtime execution is downstream from semantic lowering.
 *
 * This grammar does not prescribe:
 *
 *     - program counters;
 *     - branch instructions;
 *     - runtime stacks;
 *     - scheduler state;
 *     - backend execution.
 *
 * ============================================================================
 * INTEROPERABILITY
 * ============================================================================
 *
 * Other source languages or foreign frontends may translate their loop-control
 * constructs into the canonical ContinueStatement AST.
 *
 * This file does not directly import:
 *
 *     C
 *     C++
 *     Python
 *     OpenQASM
 *     Verilog
 *     SystemVerilog
 *     vendor-specific languages
 *
 * Interoperability belongs to the appropriate frontend/translation layer.
 *
 * ============================================================================
 * VERSIONING
 * ============================================================================
 *
 * The core syntax:
 *
 *     continue;
 *
 * is intentionally minimal and stable.
 *
 * Any future labeled syntax must be introduced through an explicit language
 * specification and compatibility policy.
 *
 * Adding syntax must not change the meaning of existing:
 *
 *     continue;
 *
 * programs.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This rule has O(1) syntactic structure for each continue statement.
 *
 * It does not encode any finite machine limit.
 *
 * The grammar therefore remains valid for programs whose total size is bounded
 * only by available compiler resources and explicitly configured resource
 * policies.
 *
 * No:
 *
 *     MAX_CONTINUE
 *     MAX_LOOP
 *     MAX_DEPTH
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_DEVICES
 *     MAX_NODES
 *
 * may appear here.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive syntax:
 *
 *     continue;
 *
 * Positive nested-loop syntax:
 *
 *     while condition {
 *         while other_condition {
 *             continue;
 *         }
 *     }
 *
 * Positive mixed control flow:
 *
 *     while condition {
 *         if other_condition {
 *             continue;
 *         }
 *     }
 *
 * Positive cross-domain syntax must be exercised by integration fixtures,
 * including loops containing:
 *
 *     classical operations
 *     quantum operations
 *     hybrid operations
 *     HDL/hardware constructs where the language permits them
 *     distributed operations
 *
 * Negative syntax:
 *
 *     continue
 *
 *     continue value;
 *
 *     continue value value;
 *
 *     continue();
 *
 *     continue(value);
 *
 *     continue label;
 *
 *     continue:;
 *
 * Semantic negative:
 *
 *     fn example() {
 *         continue;
 *     }
 *
 * The final example must parse as a continue statement and then fail semantic
 * control-flow validation, rather than being rejected by the grammar merely
 * because it is outside a loop.
 *
 * ============================================================================
 * DETERMINISM TESTS
 * ============================================================================
 *
 * Parse the same source repeatedly and verify identical syntax/AST structure.
 *
 * The result must not depend on:
 *
 *     - machine size;
 *     - CPU count;
 *     - thread count;
 *     - hardware;
 *     - quantum backend;
 *     - network state;
 *     - filesystem state;
 *     - current time.
 *
 * ============================================================================
 * ROUND-TRIP TESTS
 * ============================================================================
 *
 * Where the frontend formatter/serializer supports this construct:
 *
 *     source
 *       -> lexer
 *       -> parser
 *       -> AST
 *       -> formatter
 *       -> parser
 *
 * must preserve:
 *
 *     continue;
 *
 * semantically.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [ ] `continue;` is accepted.
 *     [ ] `continue` without `;` is rejected by the parser.
 *     [ ] Arguments/expressions after `continue` are rejected.
 *     [ ] The rule consumes canonical K_CONTINUE.
 *     [ ] The rule consumes canonical SEMICOLON.
 *     [ ] No lexer token is redefined here.
 *     [ ] No loop syntax is duplicated here.
 *     [ ] No label syntax is invented here.
 *     [ ] No semantic loop validation exists here.
 *     [ ] No machine-size limit exists here.
 *     [ ] No hardware assumption exists here.
 *     [ ] No quantum IR dependency exists here.
 *     [ ] No QEC dependency exists here.
 *     [ ] No ZQN dependency exists here.
 *     [ ] No scheduling dependency exists here.
 *     [ ] No routing dependency exists here.
 *     [ ] No runtime dependency exists here.
 *     [ ] The rule can be composed by statements.g4.
 *     [ ] The resulting parser maps to ContinueStatement.
 *     [ ] Semantic analysis remains responsible for "continue outside loop".
 *     [ ] Positive tests pass.
 *     [ ] Negative syntax tests pass.
 *     [ ] Semantic negative tests pass.
 *     [ ] Determinism tests pass.
 *     [ ] Round-trip tests pass.
 *
 * ============================================================================
 */

/*
 * Canonical continue statement.
 *
 * Current language syntax:
 *
 *     continue;
 *
 * Contextual legality is intentionally deferred to semantic/control-flow
 * analysis.
 */
continueStatement
    : K_CONTINUE SEMICOLON
    ;
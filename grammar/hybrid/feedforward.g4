/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hybrid/feedforward.g4
 *
 * Grammar:
 *     FeedForward
 *
 * Status:
 *     PRODUCTION HYBRID FEED-FORWARD BOUNDARY
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar defines the HYBRID SOURCE-SYNTAX boundary for classical
 * feed-forward into quantum computation.
 *
 * Feed-forward expresses a semantic dependency:
 *
 *     quantum computation
 *          |
 *          v
 *     classical value / measurement result
 *          |
 *          v
 *     classical expression / predicate
 *          |
 *          v
 *     later quantum operation(s)
 *
 * Canonical examples:
 *
 *     when result
 *         apply X(q);
 *
 *     when result == 1
 *         apply X(q);
 *
 *     when syndrome == expected
 *         apply correction(data);
 *
 *     when result && enabled {
 *         apply correction_a(data);
 *         apply correction_b(auxiliary);
 *     }
 *
 * The condition is an ordinary Zamani expression.
 *
 * The quantum operation is the existing canonical quantum operation grammar.
 *
 * This file therefore adds the HYBRID composition boundary without creating:
 *
 *     - another expression grammar;
 *     - another quantum operation grammar;
 *     - another quantum IR;
 *     - another AST;
 *     - another lexer;
 *     - another hardware model;
 *     - another scheduling model.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - feed-forward composition syntax;
 *     - the relationship:
 *
 *           classical condition -> quantum operation(s)
 *
 *     - feed-forward single-operation bodies;
 *     - feed-forward multi-operation bodies;
 *     - feed-forward block structure;
 *     - stable hybrid parser entry point.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer tokens;
 *     - keywords;
 *     - identifiers;
 *     - names;
 *     - expressions;
 *     - expression precedence;
 *     - types;
 *     - declarations;
 *     - general if/else syntax;
 *     - quantum operation syntax;
 *     - quantum gates;
 *     - quantum targets;
 *     - quantum measurement syntax;
 *     - resource requirements;
 *     - capabilities;
 *     - hardware;
 *     - device selection;
 *     - routing;
 *     - scheduling;
 *     - calibration;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - runtime execution;
 *     - canonical IR.
 *
 * ============================================================================
 * SINGLE-OWNER RULE
 * ============================================================================
 *
 * There must be one effective source-level owner for the feed-forward
 * relationship.
 *
 * This file is the HYBRID public composition surface.
 *
 * The existing:
 *
 *     grammar/quantum/classical-feedforward.g4
 *
 * already contains a substantially equivalent implementation.
 *
 * To avoid two competing implementations, the canonical composition must
 * eventually expose ONE implementation only.
 *
 * Migration rule:
 *
 *     grammar/hybrid/feedforward.g4
 *              |
 *              v
 *     public hybrid feed-forward boundary
 *
 * Existing quantum/classical-feedforward.g4 may be retained temporarily as a
 * compatibility/reference surface, but MUST NOT be independently dispatched
 * by the canonical parser alongside this grammar.
 *
 * The older overlapping:
 *
 *     grammar/hybrid/quantum-classical.g4
 *     grammar/hybrid/classical-quantum.g4
 *     grammar/hybrid/quantum-classical-control.g4
 *
 * MUST NOT introduce another independent:
 *
 *     WHEN expression quantum-operation
 *
 * implementation.
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * This grammar consumes the canonical:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * which in turn composes:
 *
 *     grammar/lexer/tokens.g4
 *
 * The canonical feed-forward keyword is:
 *
 *     WHEN
 *
 * whose spelling is:
 *
 *     when
 *
 * Quantum operations use:
 *
 *     APPLY
 *
 * through the imported quantum operation grammar.
 *
 * This grammar MUST NOT introduce:
 *
 *     K_WHEN
 *     K_APPLY
 *     FEEDBACK
 *     FEEDFORWARD
 *     FEEDBACK_TOKEN
 *     FEEDFORWARD_TOKEN
 *
 * merely to implement this feature.
 *
 * Feed-forward is a semantic category, not a required new keyword.
 *
 * ============================================================================
 * ANTLR CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It contains:
 *
 *     - parser rules only;
 *     - no lexer rules;
 *     - no embedded Rust;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no runtime calls;
 *     - no hardware discovery.
 *
 * Dependencies:
 *
 *     Expressions
 *         -> canonical expression syntax
 *
 *     QuantumOperations
 *         -> canonical quantum operation syntax
 *
 * Importing QuantumOperations is intentional.
 *
 * It ensures that:
 *
 *     quantumOperationStatement
 *
 * is supplied by the grammar that actually owns quantum operations rather
 * than being redefined here.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     ZamaniParser
 *          |
 *          v
 *     FeedForward
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--------------------------+
 *          |                          |
 *          v                          v
 *     classical semantics       quantum semantics
 *          |                          |
 *          |                          v
 *          |                     quantum::ir
 *          |                          |
 *          +-------------+------------+
 *                        |
 *                        v
 *               canonical semantic model
 *                        |
 *             +----------+----------+
 *             |          |          |
 *             v          v          v
 *         optimize    routing    scheduling
 *                                   |
 *                                   v
 *                              resilience
 *                                   |
 *                                   v
 *                                QEC / ZQN
 *                                   |
 *                                   v
 *                                  HAL
 *                                   |
 *                                   v
 *                            target realization
 *                                   |
 *                                   v
 *                                runtime
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Feed-forward expresses WHAT dependency exists.
 *
 * It does NOT express:
 *
 *     which CPU evaluates the condition;
 *     which GPU evaluates the condition;
 *     which FPGA evaluates the condition;
 *     which QPU executes the operation;
 *     which physical qubit is selected;
 *     which classical processor receives the result;
 *     which memory bank stores the result;
 *     which network node carries the value;
 *     which control wire transports the value;
 *     which pulse sequence implements the operation;
 *     which scheduler slot executes it;
 *     which routing path is used.
 *
 * Therefore the same source-level dependency may be realized by:
 *
 *     dynamic-circuit hardware;
 *     classical control electronics;
 *     host-side execution;
 *     FPGA control;
 *     accelerator control;
 *     compiler-generated control;
 *     simulation;
 *     distributed execution;
 *     future computational substrates.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar imposes NO artificial universal capacity.
 *
 * It contains no:
 *
 *     MAX_QUBITS
 *     MAX_CLASSICAL_BITS
 *     MAX_FEEDFORWARD
 *     MAX_FEEDBACK
 *     MAX_BRANCHES
 *     MAX_OPERATIONS
 *     MAX_TARGETS
 *     MAX_MEASUREMENTS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_THREADS
 *     MAX_MEMORY
 *     MAX_TIMELINES
 *     MAX_CIRCUIT_DEPTH
 *
 * It also contains no finite gate list.
 *
 * Repetition is represented structurally through ANTLR repetition operators.
 *
 * Therefore:
 *
 *     one condition
 *     one operation
 *
 * and:
 *
 *     arbitrarily many conditions
 *     arbitrarily many operations
 *     arbitrarily nested surrounding program constructs
 *
 * remain possible, subject only to program representation, compiler,
 * runtime, target, and resource constraints outside this grammar.
 *
 * "Infinity" therefore means:
 *
 *     no artificial finite language-level ceiling;
 *
 * not:
 *
 *     physically infinite execution.
 *
 * ============================================================================
 * SEMANTIC REQUIREMENT VS REALIZATION
 * ============================================================================
 *
 * The grammar permits semantic analysis to establish relationships such as:
 *
 *     measurement result
 *          ->
 *     classical value
 *          ->
 *     predicate
 *          ->
 *     quantum operation
 *
 * Resource/capability checks remain downstream:
 *
 *     requires capability("quantum.dynamic_control")
 *     requires capability("quantum.mid_circuit_measurement")
 *     requires memory >= required_memory
 *     requires qubits >= required_qubits
 *
 * These are NOT grammar-level hardware limits.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar must lower into the repository's existing domain-neutral
 * frontend AST.
 *
 * The feed-forward construct should conceptually preserve:
 *
 *     condition
 *     body
 *     body cardinality
 *     source span
 *
 * It MUST NOT create:
 *
 *     PhysicalFeedForwardNode
 *     QPUFeedForwardNode
 *     HostFeedForwardNode
 *     GPUFeedForwardNode
 *     VendorFeedForwardNode
 *
 * A representative semantic shape is:
 *
 *     FeedForward {
 *         condition,
 *         body,
 *         source_span
 *     }
 *
 * The actual Rust AST type remains owned by:
 *
 *     src/frontend/ast/
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes only:
 *
 *     WHEN <expression> <quantum operation body>
 *
 * Semantic analysis must determine:
 *
 *     - whether the condition is valid;
 *     - whether it is classical;
 *     - whether it is initialized;
 *     - whether it depends on a measurement;
 *     - whether its values are available at this point;
 *     - whether the condition may control quantum execution;
 *     - whether each quantum operation is valid;
 *     - whether the operation parameters are valid;
 *     - whether targets are valid;
 *     - whether required capabilities exist;
 *     - whether required resources exist;
 *     - whether the target supports dynamic control;
 *     - whether host-side or device-side realization is required;
 *     - whether compilation must transform the dependency.
 *
 * None of these checks belong in this grammar.
 *
 * ============================================================================
 * QUANTUM IR CONTRACT
 * ============================================================================
 *
 * This grammar does not define quantum IR.
 *
 * After semantic validation:
 *
 *     feed-forward dependency
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          v
 *     quantum::ir
 *
 * where appropriate.
 *
 * `quantum::ir` remains the single canonical quantum semantic IR boundary.
 *
 * This grammar MUST NOT create:
 *
 *     FeedForwardIR
 *     HybridQuantumIR
 *     QuantumFeedForwardIR
 *     DynamicCircuitIR
 *
 * as competing IRs.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * The grammar has:
 *
 *     no actions;
 *     no predicates;
 *     no randomness;
 *     no hardware discovery;
 *     no environment inspection;
 *     no filesystem access;
 *     no network access;
 *     no runtime calls.
 *
 * Therefore parsing is determined by:
 *
 *     source text
 *     +
 * canonical lexer
 *     +
 * imported grammar definitions.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar is a pure source-processing boundary.
 *
 * It cannot:
 *
 *     execute commands;
 *     access files;
 *     access networks;
 *     inspect credentials;
 *     inspect hardware;
 *     allocate devices;
 *     schedule execution;
 *     invoke a backend.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * The `.g4` file contains no Rust code.
 *
 * The generated/reference implementation must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and safe Rust only.
 *
 * No `unsafe` implementation is required by this grammar.
 *
 * ============================================================================
 */

parser grammar FeedForward;

options {
    tokenVocab = ZamaniLexer;
}

import
    Expressions,
    QuantumOperations
;


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * This is the ONLY public feed-forward parser entry point.
 *
 * The canonical Hybrid dispatcher should reference:
 *
 *     feedForward
 *
 * exactly once.
 *
 * ============================================================================
 */

feedForward
    : feedForwardStatement
    ;


/*
 * ============================================================================
 * 2. FEED-FORWARD STATEMENT
 * ============================================================================
 *
 * Canonical form:
 *
 *     when condition
 *         apply operation(target);
 *
 * or:
 *
 *     when condition {
 *         apply operation_a(target);
 *         apply operation_b(target);
 *     }
 *
 * ============================================================================
 */

feedForwardStatement
    : WHEN
      feedForwardCondition
      feedForwardBody
    ;


/*
 * ============================================================================
 * 3. CONDITION
 * ============================================================================
 *
 * The condition is deliberately the canonical Zamani expression grammar.
 *
 * No special feed-forward Boolean language is introduced.
 *
 * Examples:
 *
 *     when result
 *     when result == 1
 *     when syndrome == expected
 *     when parity != zero
 *     when ready && valid
 *     when predicate(result, state)
 *     when classical_state.value > threshold
 *
 * Semantic analysis determines whether a particular expression is valid as
 * quantum feed-forward control.
 *
 * ============================================================================
 */

feedForwardCondition
    : expression
    ;


/*
 * ============================================================================
 * 4. BODY
 * ============================================================================
 *
 * A feed-forward body contains quantum operation statements only.
 *
 * This deliberately avoids turning feed-forward into a second general-purpose
 * `if` statement.
 *
 * General classical branching remains owned by the general statement grammar.
 *
 * ============================================================================
 */

feedForwardBody
    : feedForwardOperation
    | feedForwardBlock
    ;


/*
 * ============================================================================
 * 5. SINGLE QUANTUM OPERATION
 * ============================================================================
 *
 * `quantumOperationStatement` is imported from QuantumOperations.
 *
 * This grammar therefore does not duplicate:
 *
 *     apply
 *     operation designators
 *     operation parameters
 *     operation targets
 *     controls
 *     adjoints
 *     inverses
 *     gate inventories
 *
 * ============================================================================
 */

feedForwardOperation
    : quantumOperationStatement
    ;


/*
 * ============================================================================
 * 6. MULTI-OPERATION FEED-FORWARD BLOCK
 * ============================================================================
 *
 * A single classical predicate may control any number of quantum operations.
 *
 * There is deliberately no fixed maximum.
 *
 * ============================================================================
 */

feedForwardBlock
    : LBRACE
      feedForwardOperation*
      RBRACE
    ;


/*
 * ============================================================================
 * 7. EXPLICIT EXTENSION BOUNDARY
 * ============================================================================
 *
 * Future feed-forward-specific syntax must extend through semantic contracts
 * rather than replacing the canonical expression or quantum-operation
 * grammars.
 *
 * Potential semantic extensions include:
 *
 *     measurement-dependent predicates
 *     syndrome-dependent correction
 *     computed classical predicates
 *     deferred predicates
 *     distributed classical predicates
 *     asynchronous result dependencies
 *     hybrid control regions
 *
 * Such extensions must preserve:
 *
 *     syntax       -> dependency structure
 *     semantics    -> legality
 *     compiler     -> realization
 *
 * ============================================================================
 */

feedForwardExtension
    : feedForwardStatement
    ;


/*
 * ============================================================================
 * 8. SOURCE-SCOPE CONTRACT
 * ============================================================================
 *
 * A feed-forward condition is evaluated conceptually before its controlled
 * quantum body.
 *
 * This is a semantic dependency, not a scheduling instruction.
 *
 * The grammar does not establish:
 *
 *     execution latency;
 *     synchronization latency;
 *     transport latency;
 *     hardware clock;
 *     pulse timing;
 *     host/device transfer mechanism.
 *
 * ============================================================================
 */

feedForwardSourceDependency
    : feedForwardCondition
      feedForwardBody
    ;


/*
 * ============================================================================
 * 9. COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is COMPLETE when:
 *
 *     [x] parser grammar only
 *     [x] canonical ZamaniLexer vocabulary
 *     [x] canonical expression grammar reused
 *     [x] canonical quantum operation grammar reused
 *     [x] no K_* token aliases
 *     [x] no new feed-forward lexer keyword
 *     [x] no fixed gate inventory
 *     [x] no fixed target count
 *     [x] no fixed branch count
 *     [x] no hardware IDs
 *     [x] no physical topology
 *     [x] no resource ceiling
 *     [x] no embedded Rust
 *     [x] no unsafe
 *     [x] deterministic syntax
 *     [x] domain-neutral AST contract
 *     [x] semantic contract defined
 *     [x] quantum::ir remains canonical
 *     [x] compiler/runtime realization remains downstream
 *
 * Repository integration required outside this file:
 *
 *     [ ] expose `feedForward` exactly once from grammar/hybrid/hybrid.g4
 *     [ ] ensure the canonical parser composition reaches Hybrid
 *     [ ] remove/deactivate competing feed-forward dispatcher paths
 *     [ ] update compatibility/conformance tests
 *     [ ] ensure grammar/quantum/classical-feedforward.g4 is not independently
 *         composed alongside this public hybrid boundary
 *
 * ============================================================================
 */
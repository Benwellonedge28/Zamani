/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/statements/breaks.g4
 *
 * Status:
 *     Production grammar component
 *
 * Purpose:
 *     Canonical source grammar for the Zamani `break` statement.
 *
 * Language architecture:
 *
 *     Source
 *       -> Lexer
 *       -> Parser
 *       -> AST
 *       -> Semantic Analysis
 *       -> Canonical IR
 *       -> Optimization
 *       -> Routing / Scheduling / Lowering
 *       -> Target
 *       -> Runtime / Hardware
 *
 * This file belongs exclusively to the parser/syntax layer.
 *
 * It MUST NOT contain:
 *
 *     - Rust code
 *     - embedded actions
 *     - semantic predicates
 *     - filesystem access
 *     - network access
 *     - hardware discovery
 *     - target selection
 *     - runtime execution
 *     - resource discovery
 *     - machine-size assumptions
 *     - quantum IR construction
 *     - QEC logic
 *     - ZQN logic
 *     - scheduling logic
 *     - routing logic
 *
 * Rust compatibility:
 *
 *     Rust 1.97 / Rust 1.97.1
 *     Edition 2021
 *     unsafe code is forbidden in the consuming implementation.
 *
 * ============================================================================
 * ARCHITECTURAL OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - the syntactic form of `break`;
 *     - optional break-label syntax, if the core language exposes labels;
 *     - the required statement terminator;
 *     - parser-level composition of a break statement.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - loops;
 *     - loop validity;
 *     - loop nesting;
 *     - labeled-block validity;
 *     - control-flow graphs;
 *     - unreachable-code analysis;
 *     - return semantics;
 *     - continue semantics;
 *     - function semantics;
 *     - expression syntax;
 *     - labels as general names;
 *     - identifiers;
 *     - types;
 *     - effects;
 *     - capabilities;
 *     - resources;
 *     - hardware;
 *     - quantum computation;
 *     - classical computation;
 *     - HDL;
 *     - distributed execution;
 *     - runtime behavior.
 *
 * ============================================================================
 * SINGLE SOURCE OF TRUTH
 * ============================================================================
 *
 * There MUST be exactly one authoritative `breakStatement` parser rule in
 * the assembled Zamani grammar.
 *
 * A legacy/duplicate `breakStatement` rule in:
 *
 *     grammar/statements/statements.g4
 *
 * MUST NOT remain authoritative.
 *
 * `statements.g4` should compose statement categories and import/reference
 * this rule rather than redefine it.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This parser grammar consumes tokens owned by the canonical Zamani lexer.
 *
 * The keyword token for:
 *
 *     break
 *
 * MUST be supplied by the canonical lexer.
 *
 * This file MUST NOT redefine the keyword token.
 *
 * Likewise, statement termination is owned by the canonical lexer.
 *
 * The expected canonical token names are:
 *
 *     K_BREAK
 *     SEMICOLON
 *
 * If the repository's authoritative lexer uses different names, those names
 * must be reconciled at the lexer-authority stage rather than introducing
 * aliases or duplicate lexer tokens here.
 *
 * ============================================================================
 * WHY `break` IS SMALL
 * ============================================================================
 *
 * The grammar intentionally keeps `break` syntactically small.
 *
 * A break operation means:
 *
 *     transfer control out of an enclosing breakable construct.
 *
 * Which construct is targeted is a semantic/control-flow question.
 *
 * Therefore the grammar does NOT encode:
 *
 *     loop depth
 *     loop count
 *     nesting maximum
 *     machine resources
 *     thread count
 *     processor count
 *     quantum resources
 *     hardware topology
 *
 * ============================================================================
 * CORE FORM
 * ============================================================================
 *
 * The core form is:
 *
 *     break;
 *
 * This is the portable baseline.
 *
 * ============================================================================
 * LABEL SUPPORT
 * ============================================================================
 *
 * Labeled breaks are a language-design decision and must only be enabled if
 * labels are part of the authoritative Zamani syntax model.
 *
 * This file therefore deliberately does NOT invent a label syntax merely to
 * anticipate a future feature.
 *
 * If the language specification establishes labeled control flow, the
 * production rule can be extended through the canonical label grammar without
 * changing the meaning of the core break statement.
 *
 * The intended architecture is:
 *
 *     breakStatement
 *         : K_BREAK breakTarget? SEMICOLON
 *         ;
 *
 * where:
 *
 *     breakTarget
 *
 * is supplied by the authoritative control-flow/label grammar.
 *
 * Until that contract exists, the production-safe core form is:
 *
 *     breakStatement
 *         : K_BREAK SEMICOLON
 *         ;
 *
 * This avoids creating an accidental language feature.
 *
 * ============================================================================
 * SEMANTIC VALIDATION
 * ============================================================================
 *
 * The parser accepts:
 *
 *     break;
 *
 * Semantic analysis MUST subsequently determine whether the statement occurs
 * inside a valid breakable construct.
 *
 * Examples:
 *
 *     while condition {
 *         break;
 *     }
 *
 *     loop {
 *         break;
 *     }
 *
 *     for item in items {
 *         break;
 *     }
 *
 * are potentially valid.
 *
 * Whereas:
 *
 *     fn compute() {
 *         break;
 *     }
 *
 * is syntactically a break statement but semantically invalid if no
 * breakable construct encloses it.
 *
 * The grammar MUST NOT attempt to determine this.
 *
 * ============================================================================
 * WHY CONTEXT MUST NOT BE IN THE GRAMMAR
 * ============================================================================
 *
 * Making the grammar determine whether a `break` is inside a loop would
 * couple parsing to semantic context.
 *
 * That would create undesirable dependencies such as:
 *
 *     grammar -> control-flow analysis
 *
 * and potentially:
 *
 *     grammar -> AST -> semantic analyzer -> grammar
 *
 * The intended dependency direction is:
 *
 *     grammar
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     semantic analysis
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser must produce a structure that can be lowered to the repository's
 * canonical break-statement AST representation.
 *
 * For:
 *
 *     break;
 *
 * the semantic AST representation should contain:
 *
 *     - statement kind = break
 *     - optional target = none
 *
 * The exact Rust AST type remains owned by the frontend AST implementation.
 *
 * This grammar MUST NOT duplicate the AST definition.
 *
 * ============================================================================
 * CONTROL-FLOW CONTRACT
 * ============================================================================
 *
 * Semantic analysis associates the break statement with its enclosing
 * breakable control-flow construct.
 *
 * This may include:
 *
 *     - loop identity;
 *     - labeled target;
 *     - control-flow graph edge;
 *     - scope information;
 *     - reachability information.
 *
 * None of those concepts belong in this parser grammar.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * `break` is a language-level control-flow construct.
 *
 * It must remain independent of whether the enclosing computation contains:
 *
 *     classical operations
 *     quantum operations
 *     hybrid operations
 *     hardware operations
 *     distributed operations
 *     accelerator operations
 *
 * For example, a quantum loop may eventually contain:
 *
 *     while condition {
 *         quantum_operation(...);
 *
 *         if condition {
 *             break;
 *         }
 *     }
 *
 * The grammar does not need to understand the quantum operation.
 *
 * The expression/statement grammar parses it.
 *
 * Semantic analysis establishes control-flow meaning.
 *
 * Quantum lowering subsequently maps quantum semantics to:
 *
 *     quantum::ir
 *
 * This file must never depend directly on `quantum::ir`.
 *
 * ============================================================================
 * HARDWARE / HDL INTEGRATION
 * ============================================================================
 *
 * `break` does not describe:
 *
 *     clock cycles
 *     hardware pipeline stages
 *     FPGA resources
 *     ASIC resources
 *     CPU cores
 *     GPU lanes
 *     accelerator counts
 *
 * Hardware-specific control-flow interpretation belongs downstream.
 *
 * A hardware implementation may lower a control-flow construct into a state
 * machine, pipeline control, generated logic, software control, or another
 * representation.
 *
 * That is not a parser concern.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * A break statement does not imply:
 *
 *     node termination
 *     process termination
 *     cluster cancellation
 *     distributed transaction rollback
 *     task cancellation
 *
 * Those are separate semantics.
 *
 * If a future language feature needs distributed cancellation, it must have
 * its own explicit syntax and semantic contract.
 *
 * `break` remains ordinary structured control flow.
 *
 * ============================================================================
 * CONCURRENCY INTEGRATION
 * ============================================================================
 *
 * `break` exits the enclosing language-level breakable construct.
 *
 * It MUST NOT implicitly mean:
 *
 *     cancel all tasks
 *     cancel all threads
 *     stop all actors
 *     terminate all workers
 *     synchronize all devices
 *
 * Concurrency semantics belong to the concurrency subsystem.
 *
 * ============================================================================
 * RESOURCE / SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar contains no fixed limits.
 *
 * It MUST NOT introduce:
 *
 *     MAX_LOOP_DEPTH
 *     MAX_BREAK_DEPTH
 *     MAX_BREAKS
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_QUBITS
 *     MAX_DEVICES
 *     MAX_NODES
 *
 * A source program may contain any number of break statements permitted by
 * available implementation resources.
 *
 * Language syntax does not impose a machine-size ceiling.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * The semantic meaning of:
 *
 *     break;
 *
 * is independent of:
 *
 *     CPU architecture
 *     GPU architecture
 *     FPGA architecture
 *     ASIC architecture
 *     quantum processor
 *     quantum simulator
 *     embedded processor
 *     cluster
 *     supercomputer
 *     cloud platform
 *     future execution substrate
 *
 * Therefore the same source-level control-flow meaning can be preserved
 * while downstream compilation selects an appropriate implementation.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no mutable parser state;
 *     - no randomness;
 *     - no time-dependent behavior;
 *     - no I/O;
 *     - no hardware inspection;
 *     - no semantic predicates;
 *     - no embedded Rust actions.
 *
 * Consequently, identical token sequences produce identical parse results.
 *
 * ============================================================================
 * ERROR OWNERSHIP
 * ============================================================================
 *
 * Parser-owned errors include:
 *
 *     break
 *
 * when an explicit statement terminator is required;
 *
 *     break something;
 *
 * when no break-target grammar exists;
 *
 *     break;;
 *
 * when an additional statement terminator cannot be consumed as another
 * valid statement.
 *
 * Semantic errors include:
 *
 *     break;
 *
 * outside a breakable construct.
 *
 *     break;
 *
 * targeting an invalid label.
 *
 * Those semantic errors must be reported by semantic/control-flow analysis,
 * not by this grammar.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * The parser/frontend diagnostics layer should report:
 *
 *     - unexpected tokens after `break`;
 *     - missing statement terminator;
 *     - malformed future break targets;
 *     - source span;
 *     - token location;
 *     - parser context.
 *
 * This grammar does not manufacture diagnostic strings through embedded
 * actions.
 *
 * ============================================================================
 * VERSIONING
 * ============================================================================
 *
 * `break` is a core control-flow keyword.
 *
 * Its core syntax:
 *
 *     break;
 *
 * must remain stable across compatible language versions.
 *
 * Any future extension such as labeled breaks must:
 *
 *     1. be specified first;
 *     2. be assigned a language-version policy;
 *     3. update compatibility documentation;
 *     4. add positive and negative tests;
 *     5. preserve existing `break;` programs.
 *
 * ============================================================================
 * INTEROPERABILITY
 * ============================================================================
 *
 * This grammar does not directly depend on:
 *
 *     C
 *     C++
 *     Python
 *     OpenQASM
 *     Verilog
 *     SystemVerilog
 *     vendor-specific hardware languages
 *
 * Interoperability layers may translate their control-flow constructs into
 * Zamani's canonical semantic representation.
 *
 * Such translation belongs outside this file.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Required positive tests:
 *
 *     break;
 *
 *     while condition {
 *         break;
 *     }
 *
 *     loop {
 *         break;
 *     }
 *
 *     for item in items {
 *         break;
 *     }
 *
 * Required negative syntax tests:
 *
 *     break
 *
 *     break value;
 *
 *     break value value;
 *
 *     break;;
 *
 *     break (
 *
 * Required semantic tests:
 *
 *     break;
 *     // outside any breakable construct
 *
 * Required nesting tests:
 *
 *     while outer {
 *         while inner {
 *             break;
 *         }
 *     }
 *
 * Required cross-domain tests:
 *
 *     classical loop + break
 *     quantum-capable loop + break
 *     hybrid loop + break
 *     hardware-oriented control flow + break
 *     distributed task body + break
 *
 * Required scalability tests:
 *
 *     - many nested breakable constructs;
 *     - many break statements;
 *     - large source files;
 *     - large surrounding expressions;
 *     - no artificial source-level machine limits.
 *
 * Required determinism tests:
 *
 *     parse identical source repeatedly;
 *     compare resulting syntax structure;
 *     ensure no nondeterministic result.
 *
 * Required round-trip tests:
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
 *     break;
 *
 * ============================================================================
 * INTEGRATION GRAPH
 * ============================================================================
 *
 *     lexer
 *       |
 *       | K_BREAK
 *       | SEMICOLON
 *       v
 *     breaks.g4
 *       |
 *       v
 *     statements.g4
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
 *       +--> scope analysis
 *       +--> control-flow analysis
 *       +--> label resolution
 *       +--> reachability
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +--> classical lowering
 *       +--> quantum lowering
 *       +--> hybrid lowering
 *       +--> HDL lowering
 *       +--> distributed lowering
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     scheduling / routing / target lowering
 *       |
 *       v
 *     runtime
 *
 * ============================================================================
 * FORBIDDEN DEPENDENCIES
 * ============================================================================
 *
 * This file MUST NOT import or depend directly on:
 *
 *     src/quantum/ir
 *     QEC
 *     ZQN
 *     scheduling
 *     routing
 *     hardware discovery
 *     calibration
 *     runtime
 *     backend-specific implementations
 *
 * Those layers consume semantic representations produced downstream.
 *
 * ============================================================================
 * NO CIRCULAR DEPENDENCY
 * ============================================================================
 *
 * Correct:
 *
 *     breaks.g4
 *          |
 *          v
 *     parser AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     IR
 *
 * Incorrect:
 *
 *     breaks.g4 -> IR -> breaks.g4
 *
 * Incorrect:
 *
 *     breaks.g4 -> runtime -> breaks.g4
 *
 * Incorrect:
 *
 *     breaks.g4 -> quantum::ir -> hardware -> breaks.g4
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE when:
 *
 *     [ ] K_BREAK is owned by the canonical lexer.
 *     [ ] SEMICOLON is owned by the canonical lexer.
 *     [ ] breakStatement has one authoritative definition.
 *     [ ] no duplicate breakStatement exists in the assembled grammar.
 *     [ ] `break;` parses deterministically.
 *     [ ] malformed break syntax is rejected.
 *     [ ] semantic context is not encoded in parser actions.
 *     [ ] AST mapping is defined.
 *     [ ] semantic validation ownership is defined.
 *     [ ] no hardware assumptions exist.
 *     [ ] no machine-size limits exist.
 *     [ ] no quantum-specific assumptions exist.
 *     [ ] no unsafe Rust is introduced.
 *     [ ] no embedded Rust actions are introduced.
 *     [ ] positive tests exist.
 *     [ ] negative tests exist.
 *     [ ] boundary tests exist.
 *     [ ] cross-domain tests exist.
 *     [ ] determinism tests exist.
 *     [ ] round-trip tests exist.
 *     [ ] compatibility policy is documented.
 *
 * ============================================================================
 * PRODUCTION RULE
 * ============================================================================
 *
 * Keep this grammar intentionally small.
 *
 * Complexity belongs in the appropriate downstream layer rather than being
 * pushed into the grammar.
 *
 * ============================================================================
 */

parser grammar BreakStatements;

options {
    tokenVocab = ZamaniTokens;
}


/*
 * ============================================================================
 * BREAK STATEMENT
 * ============================================================================
 *
 * Canonical core form:
 *
 *     break;
 *
 * The statement contains no expression and no implicit target.
 *
 * Contextual validity is checked by semantic analysis.
 */
breakStatement
    : K_BREAK SEMICOLON
    ;
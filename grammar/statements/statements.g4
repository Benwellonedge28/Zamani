/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/statements/statements.g4
 *
 * Status:
 *     CANONICAL production statement-composition grammar.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains:
 *
 *       - no embedded Rust actions;
 *       - no semantic predicates;
 *       - no unsafe Rust;
 *       - no filesystem access;
 *       - no networking;
 *       - no process execution;
 *       - no hardware discovery;
 *       - no runtime execution;
 *       - no mutable global state;
 *       - no target-specific implementation;
 *       - no machine-size constants.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the SINGLE COMPOSITION OWNER for Zamani statements.
 *
 * It answers only:
 *
 *     "Which syntactic constructs are admitted where a statement is expected?"
 *
 * It does NOT implement the individual statement families.
 *
 * Individual syntax remains owned by dedicated grammar components:
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
 * This separation is intentional.
 *
 * ============================================================================
 * ARCHITECTURAL RULE
 * ============================================================================
 *
 * The dependency direction is:
 *
 *     source
 *       |
 *       v
 *     canonical lexer
 *       |
 *       v
 *     statement composition              <-- THIS FILE
 *       |
 *       +--> statement-family grammars
 *       |
 *       v
 *     frontend parser / AST
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +--> names
 *       +--> types
 *       +--> effects
 *       +--> capabilities
 *       +--> ownership
 *       +--> control flow
 *       +--> resources
 *       |
 *       v
 *     canonical semantic representations
 *       |
 *       +--> classical representation
 *       +--> quantum::ir
 *       +--> HDL / hardware representation
 *       +--> distributed representation
 *       +--> accelerator representation
 *       +--> future-domain representations
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     routing / scheduling / lowering
 *       |
 *       v
 *     target realization
 *       |
 *       v
 *     runtime / hardware
 *
 * This grammar MUST NOT depend directly on:
 *
 *     quantum::ir
 *     QEC
 *     ZQN
 *     resilience
 *     routing
 *     scheduling
 *     optimization
 *     hardware discovery
 *     calibration
 *     runtime
 *     backend selection
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - statement
 *     - statement-family composition
 *     - canonical statement dispatch
 *     - generic expression-statement admission
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer rules
 *     - keyword spelling
 *     - punctuation spelling
 *     - expressions
 *     - expression precedence
 *     - assignment operators
 *     - declarations
 *     - functions
 *     - modules
 *     - types
 *     - blocks
 *     - loops
 *     - if/else syntax
 *     - match syntax
 *     - break syntax
 *     - continue syntax
 *     - return syntax
 *     - exception syntax
 *     - unsafe syntax
 *     - assertion syntax
 *     - quantum syntax
 *     - HDL syntax
 *     - hardware syntax
 *     - resource semantics
 *     - capability semantics
 *     - effect semantics
 *     - AST implementation
 *     - semantic analysis
 *     - IR construction
 *     - machine selection
 *     - scheduling
 *     - routing
 *     - runtime execution
 *
 * ============================================================================
 * SINGLE-SOURCE-OF-TRUTH CONTRACT
 * ============================================================================
 *
 * There MUST be exactly ONE authoritative `statement` rule in the assembled
 * Zamani parser.
 *
 * The legacy monolithic statement rule in:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * and any competing statement rule in:
 *
 *     grammar/antlr/Core.g4
 *
 * MUST NOT remain part of the authoritative parser generation path once this
 * modular grammar becomes canonical.
 *
 * Those files may remain temporarily for migration/documentation purposes, but
 * generated production parsers MUST expose this composition boundary exactly
 * once.
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * Every imported grammar is a delegate grammar.
 *
 * Delegate grammars own their concrete productions.
 *
 * This file MUST NOT copy those productions.
 *
 * The import graph is deliberately one-way:
 *
 *     Statements
 *       |
 *       +--> Assignments
 *       +--> AssertionsParser
 *       +--> Blocks
 *       +--> BreakStatements
 *       +--> ConditionalsParser
 *       +--> ContinueStatements
 *       +--> Declarations
 *       +--> ExceptionsParser
 *       +--> Loops
 *       +--> PatternMatching
 *       +--> ReturnStatements
 *       +--> UnsafeStatementsParser
 *
 * Delegate grammars may consume canonical rules exposed by the assembled
 * grammar, such as:
 *
 *     statement
 *     expression
 *     identifier
 *     blockExpression
 *
 * but MUST NOT redefine this file's `statement` rule.
 *
 * ============================================================================
 * EXPRESSION / ASSIGNMENT OWNERSHIP
 * ============================================================================
 *
 * Zamani has two related constructs:
 *
 *     assignmentExpression
 *
 * and:
 *
 *     assignmentStatement
 *
 * The expression grammar owns assignment expressions.
 *
 * The statement grammar owns statement composition.
 *
 * IMPORTANT:
 *
 * The canonical generic expression statement below intentionally consumes the
 * canonical `expression` rule.
 *
 * This means assignment syntax remains valid in statement position through:
 *
 *     expression
 *         |
 *         v
 *     expressionStatement
 *         |
 *         v
 *     statement
 *
 * The separate `assignmentStatement` rule supplied by Assignments remains
 * available to grammar contexts that specifically require a statement-level
 * assignment boundary.
 *
 * It is NOT duplicated here.
 *
 * This avoids two competing implementations of assignment semantics.
 *
 * ============================================================================
 * BLOCK OWNERSHIP
 * ============================================================================
 *
 * Blocks are owned by:
 *
 *     grammar/statements/blocks.g4
 *
 * This file MUST NOT define:
 *
 *     {
 *     }
 *     blockExpression
 *     blockElement
 *
 * A block consumes the canonical `statement` rule supplied here.
 *
 * Therefore the relationship is:
 *
 *     Statements
 *         |
 *         v
 *     statement
 *         |
 *         v
 *     Blocks.blockElement
 *         |
 *         v
 *     statement
 *
 * This is an intentional recursive grammar relationship.
 *
 * It does NOT represent a dependency cycle between semantic subsystems.
 *
 * ============================================================================
 * CONTROL-FLOW OWNERSHIP
 * ============================================================================
 *
 * This file composes control-flow families but does not implement them.
 *
 * The concrete owners are:
 *
 *     conditionals.g4
 *     loops.g4
 *     pattern-matching.g4
 *     breaks.g4
 *     continues.g4
 *     returns.g4
 *     exceptions.g4
 *
 * The dispatcher is therefore:
 *
 *     statement
 *       |
 *       +--> controlFlowStatement
 *              |
 *              +--> conditional
 *              +--> loop
 *              +--> match
 *              +--> break
 *              +--> continue
 *              +--> return
 *              +--> throw
 *              +--> try
 *
 * No control-flow semantics are evaluated by this grammar.
 *
 * ============================================================================
 * DECLARATION OWNERSHIP
 * ============================================================================
 *
 * Declarations are owned by:
 *
 *     declarations.g4
 *
 * This file merely admits `declarationStatement` at statement position.
 *
 * The declaration grammar itself delegates further to:
 *
 *     functions
 *     modules
 *     types
 *     core
 *     domain-specific declarations
 *
 * ============================================================================
 * ASSERTION OWNERSHIP
 * ============================================================================
 *
 * Statement assertions are owned by:
 *
 *     assertions.g4
 *
 * Compile-time assertions remain expression/compile-time constructs and MUST
 * NOT be duplicated here.
 *
 * ============================================================================
 * UNSAFE OWNERSHIP
 * ============================================================================
 *
 * Unsafe-region syntax is owned by:
 *
 *     unsafe.g4
 *
 * `unsafe` in Zamani is a source-language semantic boundary.
 *
 * It is NOT Rust `unsafe`.
 *
 * The presence of:
 *
 *     unsafeStatement
 *
 * in this dispatcher does not permit the grammar implementation itself to use
 * unsafe Rust.
 *
 * ============================================================================
 * GENERIC EXPRESSION STATEMENTS
 * ============================================================================
 *
 * A generic expression may appear in statement position.
 *
 * Examples:
 *
 *     compute();
 *     value;
 *     measure(q);
 *     hardware_operation();
 *     distributed_operation();
 *     tensor_operation();
 *
 * The expression grammar determines what constitutes an expression.
 *
 * This file determines only that an expression can occupy statement position
 * when followed by the canonical statement terminator.
 *
 * No expression precedence is repeated here.
 *
 * ============================================================================
 * TERMINATION OWNERSHIP
 * ============================================================================
 *
 * Statement termination is inherited from the canonical statement ecosystem.
 *
 * The current repository uses explicit semicolon termination for ordinary
 * expression statements.
 *
 * This file therefore uses:
 *
 *     SEMICOLON
 *
 * directly for its generic expression-statement boundary.
 *
 * Delegate statement grammars may use their own canonical termination
 * abstraction where they own a more specific statement form.
 *
 * Automatic semicolon insertion MUST NOT be introduced here without an
 * explicit language-specification and compatibility decision.
 *
 * ============================================================================
 * DOMAIN NEUTRALITY
 * ============================================================================
 *
 * Statements are classified by language semantics, not by target hardware.
 *
 * This grammar MUST NOT introduce alternatives such as:
 *
 *     cpuStatement
 *     gpuStatement
 *     fpgaStatement
 *     asicStatement
 *     qpuStatement
 *     embeddedStatement
 *     clusterStatement
 *
 * merely because the eventual execution target differs.
 *
 * The same statement grammar must remain usable for:
 *
 *     classical computing
 *     quantum computing
 *     hybrid computing
 *     HDL
 *     hardware/software co-design
 *     embedded computing
 *     distributed computing
 *     parallel computing
 *     HPC
 *     AI/ML
 *     accelerators
 *     networking
 *     scientific computing
 *     future computational domains
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * This file does not define quantum operations.
 *
 * Quantum syntax is owned by the quantum grammar family.
 *
 * A statement may contain or control quantum computation without this dispatcher
 * knowing:
 *
 *     qubit count
 *     logical-qubit count
 *     physical-qubit count
 *     topology
 *     gate inventory
 *     backend
 *     calibration
 *     QEC strategy
 *     ZQN noise model
 *     schedule
 *     routing
 *
 * The downstream semantic pipeline remains:
 *
 *     statement AST
 *        |
 *        v
 *     semantic analysis
 *        |
 *        v
 *     canonical quantum semantics
 *        |
 *        v
 *     quantum::ir
 *
 * This grammar never constructs `quantum::ir`.
 *
 * ============================================================================
 * HDL / HARDWARE CONTRACT
 * ============================================================================
 *
 * Hardware and HDL statements are admitted through their own domain grammar
 * layers when those layers become statement-producing grammar components.
 *
 * This file must not hard-code:
 *
 *     registers
 *     cores
 *     lanes
 *     devices
 *     memory sizes
 *     FPGA resources
 *     ASIC dimensions
 *     clock counts
 *     topology
 *
 * Hardware realization belongs downstream.
 *
 * ============================================================================
 * DISTRIBUTED / CONCURRENT CONTRACT
 * ============================================================================
 *
 * Statement composition does not imply:
 *
 *     a fixed number of threads
 *     a fixed number of tasks
 *     a fixed number of nodes
 *     a fixed number of workers
 *     a fixed number of devices
 *
 * Those are resource/capability/runtime concerns.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * A statement is source-level semantic structure.
 *
 * The same statement syntax must remain valid whether the resulting program
 * executes on:
 *
 *     one tiny processor
 *     one CPU
 *     many CPUs
 *     GPUs
 *     FPGAs
 *     ASICs
 *     QPUs
 *     simulators
 *     heterogeneous systems
 *     clusters
 *     supercomputers
 *     cloud systems
 *     future architectures
 *
 * The grammar contains NO language-level limits for:
 *
 *     - number of statements;
 *     - number of blocks;
 *     - number of branches;
 *     - loop count;
 *     - nesting count;
 *     - devices;
 *     - qubits;
 *     - cores;
 *     - threads;
 *     - GPUs;
 *     - FPGAs;
 *     - nodes;
 *     - memory;
 *     - accelerators.
 *
 * Practical parser/compiler limits belong to explicit resource policies and
 * implementation configuration, never to source-language machine constants.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains no:
 *
 *     - actions;
 *     - semantic predicates;
 *     - randomness;
 *     - time-dependent behavior;
 *     - I/O;
 *     - filesystem access;
 *     - network access;
 *     - hardware inspection;
 *     - runtime execution;
 *     - mutable global state.
 *
 * Identical token streams under identical grammar/token versions must produce
 * identical parse structures.
 *
 * ============================================================================
 * ERROR BOUNDARY
 * ============================================================================
 *
 * Parser errors belong to the parser/frontend diagnostic layer.
 *
 * Examples include:
 *
 *     missing statement terminator
 *     malformed statement keyword
 *     incomplete block
 *     malformed control-flow construct
 *     unexpected token
 *     incomplete declaration
 *
 * Semantic errors do NOT belong here.
 *
 * Examples:
 *
 *     break outside a loop
 *     return outside a callable
 *     invalid quantum operation
 *     unavailable capability
 *     impossible resource requirement
 *     illegal ownership
 *     invalid type
 *
 * Those are downstream semantic diagnostics.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates no Rust AST values.
 *
 * The frontend parser/AST layer maps parser contexts to the repository's
 * canonical AST nodes.
 *
 * Every statement node must preserve:
 *
 *     - source span;
 *     - source order;
 *     - child relationships;
 *     - statement kind;
 *     - relevant syntax metadata.
 *
 * No machine-specific information is added merely because a statement may
 * eventually lower to a particular target.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates no IR.
 *
 * The intended path is:
 *
 *     source
 *       -> lexer
 *       -> parser
 *       -> AST
 *       -> semantic analysis
 *       -> canonical semantic representation
 *       -> domain-specific IR
 *       -> optimization
 *       -> routing/scheduling/lowering
 *       -> target
 *
 * For quantum programs:
 *
 *     AST
 *       -> semantic quantum representation
 *       -> quantum::ir
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * ============================================================================
 * VERSIONING
 * ============================================================================
 *
 * This file is a composition boundary and should therefore be comparatively
 * stable.
 *
 * Adding a new statement family should normally require:
 *
 *     1. a dedicated grammar owner;
 *     2. a parser grammar import;
 *     3. one new alternative in the appropriate composition category;
 *     4. AST integration;
 *     5. semantic integration;
 *     6. tests;
 *     7. compatibility documentation.
 *
 * Existing statement syntax MUST NOT be silently redefined.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive:
 *
 *     expression;
 *     assignment;
 *     declaration;
 *     assertion;
 *     if/else;
 *     loop;
 *     match;
 *     break;
 *     continue;
 *     return;
 *     return expression;
 *     throw expression;
 *     try/catch;
 *     try/finally;
 *     unsafe { ... }
 *
 * Negative:
 *
 *     incomplete statement
 *     missing terminator where required
 *     malformed declaration
 *     malformed loop
 *     malformed conditional
 *     malformed match
 *     malformed exception
 *     malformed unsafe region
 *
 * Boundary:
 *
 *     empty statement
 *     deeply nested statements
 *     very large statement sequences
 *     very large expressions
 *     very large blocks
 *
 * Cross-domain:
 *
 *     classical + quantum
 *     classical + HDL
 *     quantum + hardware
 *     quantum + distributed
 *     AI + quantum
 *     AI + hardware
 *     classical + quantum + distributed
 *     classical + quantum + HDL + hardware
 *
 * Scalability:
 *
 * No source-level machine-size limit may be introduced by this dispatcher.
 *
 * Determinism:
 *
 * Repeated parsing of identical source must produce equivalent parse trees.
 *
 * Round-trip:
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
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE only when:
 *
 *     [ ] It is the sole owner of `statement`.
 *     [ ] Every concrete statement family has one external owner.
 *     [ ] No statement implementation is duplicated here.
 *     [ ] All imports resolve.
 *     [ ] The assembled parser has no duplicate rule definitions.
 *     [ ] The assembled parser has no undefined statement references.
 *     [ ] Lexer ownership remains external.
 *     [ ] Expression ownership remains external.
 *     [ ] Block ownership remains external.
 *     [ ] AST ownership remains external.
 *     [ ] Semantic validation remains downstream.
 *     [ ] No machine-size constants exist.
 *     [ ] No target-specific syntax has leaked into generic statements.
 *     [ ] No embedded Rust exists.
 *     [ ] No unsafe Rust exists.
 *     [ ] Rust 1.97 / 1.97.1 integration passes.
 *     [ ] Positive tests pass.
 *     [ ] Negative tests pass.
 *     [ ] Boundary tests pass.
 *     [ ] Cross-domain tests pass.
 *     [ ] Determinism tests pass.
 *     [ ] Round-trip tests pass.
 *     [ ] Legacy grammar no longer competes with this dispatcher.
 *
 * ============================================================================
 * CANONICAL PARSER
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
 * Concrete statement syntax remains in its dedicated grammar.
 *
 * The import list is intentionally explicit. A new statement family must not
 * be silently introduced by modifying this file's internal syntax.
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
 * This is the ONLY authoritative `statement` rule in the modular statement
 * grammar.
 *
 * Ordering:
 *
 *     1. declarations
 *     2. assertions / safety boundaries
 *     3. structured control flow
 *     4. blocks
 *     5. generic expressions
 *
 * ANTLR's adaptive prediction resolves the concrete alternatives using the
 * token stream. No semantic predicate is required.
 */
statement
    : declarationStatement
    | assertionStatement
    | controlFlowStatement
    | unsafeStatement
    | blockExpression
    | expressionStatement
    ;


/*
 * ============================================================================
 * CONTROL-FLOW COMPOSITION
 * ============================================================================
 *
 * Concrete syntax remains delegated to the individual grammar owners.
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
 * EXCEPTION ADAPTER
 * ============================================================================
 *
 * exceptions.g4 owns the concrete try/catch/finally syntax.
 *
 * This small adapter prevents the dispatcher from depending on the internal
 * representation of exception syntax.
 */
tryStatement
    : tryCatchFinallyStatement
    ;


/*
 * ============================================================================
 * GENERIC EXPRESSION STATEMENT
 * ============================================================================
 *
 * Expression syntax is owned by Expressions.
 *
 * This rule intentionally does NOT redefine:
 *
 *     assignmentExpression
 *     binaryExpression
 *     unaryExpression
 *     conditionalExpression
 *     call syntax
 *     indexing syntax
 *     member syntax
 *     quantum expressions
 *     hardware expressions
 *
 * Therefore:
 *
 *     foo();
 *     value;
 *     result = compute();
 *     measure(q);
 *
 * all consume the canonical expression grammar.
 *
 * Whether an expression is semantically valid as a standalone statement is
 * determined by semantic/effect analysis.
 */
expressionStatement
    : expression SEMICOLON
    ;


/*
 * ============================================================================
 * EMPTY STATEMENT
 * ============================================================================
 *
 * A standalone semicolon is a valid syntactic statement.
 *
 * It is deliberately represented separately from expressionStatement because
 * an empty statement contains no expression AST node.
 *
 * Whether empty statements should produce a warning/lint diagnostic is
 * downstream policy.
 */
emptyStatement
    : SEMICOLON
    ;
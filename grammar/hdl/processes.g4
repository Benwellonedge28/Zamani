parser grammar processes;

options {
    tokenVocab = ZamaniTokens;
}

/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hdl/processes.g4
 *
 * Purpose:
 *     Production parser delegate for HDL process declarations.
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no embedded Rust, actions, predicates,
 *     filesystem access, network access, process execution, or unsafe code.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          v
 *     HDL composition
 *          |
 *          +--> processes.g4
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> type checking
 *          +--> process legality
 *          +--> clock/event validation
 *          +--> sequential/combinational classification
 *          +--> resource/capability analysis
 *          +--> CDC analysis
 *          |
 *          v
 *     canonical hardware/HDL semantic representation
 *          |
 *          +--> optimization
 *          +--> scheduling
 *          +--> routing
 *          +--> synthesis
 *          +--> target lowering
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - HDL process declaration syntax;
 *     - process identity/name syntax;
 *     - process sensitivity/event-list syntax;
 *     - process-local configuration bindings;
 *     - process-local attributes;
 *     - process body boundary;
 *     - process-local statement composition;
 *     - process-local nested blocks;
 *     - process-local event grouping syntax.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - identifiers;
 *     - general expressions;
 *     - general statements;
 *     - general blocks;
 *     - types;
 *     - ports;
 *     - signals;
 *     - wires/nets;
 *     - registers;
 *     - clocks;
 *     - timing constraints;
 *     - combinational semantics;
 *     - sequential semantics;
 *     - state machines;
 *     - memories;
 *     - pipelines;
 *     - hardware interfaces;
 *     - hardware resources;
 *     - target selection;
 *     - placement;
 *     - routing;
 *     - scheduling;
 *     - synthesis;
 *     - simulation;
 *     - calibration;
 *     - runtime execution;
 *     - AST construction;
 *     - canonical IR construction.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * A process describes a logical hardware behavior.
 *
 * It MUST NOT inherently select:
 *
 *     - a CPU;
 *     - a GPU;
 *     - an FPGA;
 *     - an ASIC;
 *     - a QPU;
 *     - a board;
 *     - a vendor;
 *     - a physical clock tree;
 *     - a physical pin;
 *     - a physical address;
 *     - a routing topology;
 *     - a fixed resource count;
 *     - a fixed machine size.
 *
 * A process may express semantic requirements such as:
 *
 *     clock
 *     reset
 *     enable
 *     event
 *     sensitivity
 *     process mode
 *     implementation-independent attributes
 *
 * Physical realization is a downstream concern.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar intentionally contains no finite resource limits.
 *
 * There is no:
 *
 *     MAX_PROCESSES
 *     MAX_SENSITIVITY_ITEMS
 *     MAX_EVENTS
 *     MAX_STATEMENTS
 *     MAX_NESTING
 *     MAX_CLOCKS
 *     MAX_RESETS
 *     MAX_SIGNALS
 *     MAX_REGISTERS
 *     MAX_MODULES
 *     MAX_DEVICES
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_QUBITS
 *
 * Repetition is represented through ANTLR's unbounded `*` and `+`
 * operators.
 *
 * Actual resource limits belong to parser/resource policy, semantic
 * analysis, compilation, scheduling, synthesis, deployment, and runtime.
 *
 * ============================================================================
 * ANTLR COMPOSITION
 * ============================================================================
 *
 * This is a parser delegate grammar.
 *
 * The canonical lexer vocabulary is the sole lexical authority.
 *
 * Shared grammar contracts consumed by this file include:
 *
 *     identifier
 *     hdlKeyword
 *     hdlExpression
 *     hdlStatement
 *     hdlBlock
 *     hdlAttributeList
 *
 * Those rules MUST be supplied by the canonical/shared HDL parser
 * composition layer.
 *
 * This file MUST NOT redefine them.
 *
 * ============================================================================
 * CONTEXTUAL HDL VOCABULARY
 * ============================================================================
 *
 * The existing Zamani HDL architecture intentionally treats HDL-specific
 * terminology as contextual rather than creating an independent lexer.
 *
 * Consequently this grammar does not create a second lexer vocabulary for:
 *
 *     process
 *     always
 *     event
 *     sensitivity
 *     clock
 *     reset
 *     enable
 *     asynchronous
 *     synchronous
 *     combinational
 *     sequential
 *
 * Their semantic validity is checked by the HDL semantic layer.
 *
 * IMPORTANT:
 *
 * The canonical lexer/token vocabulary remains authoritative.
 *
 * If the language specification later reserves dedicated tokens for any
 * process keyword, this grammar may consume those canonical tokens, but
 * those tokens MUST be introduced in the canonical lexer rather than here.
 *
 * ============================================================================
 */


/* ============================================================================
 * PUBLIC ENTRY POINT
 * ========================================================================== */

/*
 * A process consists of:
 *
 *     process declaration
 *     optional logical name
 *     optional sensitivity/event specification
 *     optional process attributes
 *     process body
 *
 * Examples of the semantic shapes supported by this syntax include:
 *
 *     process {
 *         ...
 *     }
 *
 *     process counter {
 *         ...
 *     }
 *
 *     process counter(clk) {
 *         ...
 *     }
 *
 *     process counter(clk, reset) {
 *         ...
 *     }
 *
 * The semantic layer determines whether the process is combinational,
 * sequential, asynchronous, clocked, event-driven, or another supported
 * process category.
 */
hdlProcessDeclaration
    : hdlProcessKeyword
      identifier?
      hdlProcessSensitivity?
      hdlProcessAttributes?
      hdlProcessBody
    ;


/* ============================================================================
 * PROCESS KEYWORD
 * ========================================================================== */

/*
 * Process terminology is contextual in the current HDL architecture.
 *
 * Keeping this as a dedicated grammar rule gives the semantic/compiler
 * integration layer a stable process boundary without duplicating lexer
 * vocabulary.
 *
 * A future canonical reserved token can replace this rule internally without
 * changing the process AST contract.
 */
hdlProcessKeyword
    : hdlKeyword
    ;


/* ============================================================================
 * PROCESS ATTRIBUTES
 * ========================================================================== */

/*
 * Attributes are process-local metadata.
 *
 * They do not directly select hardware.
 */
hdlProcessAttributes
    : hdlAttributeList
    ;


/* ============================================================================
 * PROCESS SENSITIVITY
 * ========================================================================== */

/*
 * Parenthesized sensitivity/event specification.
 *
 * Examples:
 *
 *     (clk)
 *     (clk, reset)
 *     (clock_event)
 *     (signal_a, signal_b, signal_c)
 *
 * The grammar deliberately does not hard-code a finite number of events.
 */
hdlProcessSensitivity
    : LPAREN
      hdlProcessSensitivityList?
      RPAREN
    ;


/*
 * An arbitrary number of sensitivity items is permitted.
 */
hdlProcessSensitivityList
    : hdlProcessSensitivityItem
      (
          COMMA
          hdlProcessSensitivityItem
      )*
      COMMA?
    ;


/*
 * A sensitivity item is represented by the shared expression system.
 *
 * This is intentional.
 *
 * It permits future semantic forms without repeatedly modifying the
 * process grammar for every new event representation.
 *
 * Examples that can therefore be represented by the expression layer include:
 *
 *     clk
 *     reset
 *     enable
 *     event(clk)
 *     rising(clk)
 *     falling(clk)
 *     condition
 *
 * Whether a particular form is legal for a particular process kind is a
 * semantic question, not a parsing question.
 */
hdlProcessSensitivityItem
    : hdlExpression
    ;


/* ============================================================================
 * PROCESS BODY
 * ========================================================================== */

/*
 * The body owns process-local composition but delegates ordinary statements
 * to the shared statement grammar.
 *
 * This prevents a second procedural language from being created inside HDL.
 */
hdlProcessBody
    : LBRACE
      hdlProcessItem*
      RBRACE
    ;


/*
 * Process items may be:
 *
 *     - process configuration bindings;
 *     - ordinary shared statements;
 *     - nested process blocks;
 *     - process-local attributes.
 *
 * Generic statements remain owned by the shared statement grammar.
 */
hdlProcessItem
    : hdlProcessBinding
    | hdlStatement
    | hdlProcessBlock
    | hdlAttributeList
    ;


/* ============================================================================
 * PROCESS BINDINGS
 * ========================================================================== */

/*
 * Process bindings provide an extensible syntax for process-local semantic
 * properties without hard-coding every possible future process concept into
 * this grammar.
 *
 * Examples:
 *
 *     clock: clk;
 *     reset: reset_signal;
 *     enable: enable_signal;
 *     mode: sequential;
 *     reset_mode: asynchronous;
 *     edge: rising;
 *
 * The identifier on the left is a logical semantic property name.
 *
 * The semantic layer owns:
 *
 *     - which properties are legal;
 *     - which types they require;
 *     - whether they may repeat;
 *     - whether combinations are valid;
 *     - whether they conflict;
 *     - whether they have target-specific realizations.
 */
hdlProcessBinding
    : identifier
      COLON
      hdlExpression
      SEMICOLON
    ;


/* ============================================================================
 * NESTED PROCESS BLOCKS
 * ========================================================================== */

/*
 * Named nested blocks are useful for structured process organization while
 * remaining target-independent.
 *
 * Example:
 *
 *     process controller {
 *
 *         reset {
 *             ...
 *         }
 *
 *         update {
 *             ...
 *         }
 *     }
 *
 * The semantic layer decides which named blocks are meaningful.
 *
 * This grammar does not invent a second state-machine, timing, or scheduling
 * model.
 */
hdlProcessBlock
    : identifier
      hdlProcessBlockArguments?
      LBRACE
      hdlProcessItem*
      RBRACE
    ;


/*
 * Optional named-block arguments.
 *
 * Example:
 *
 *     event(clk) {
 *         ...
 *     }
 *
 *     update(mode = next) {
 *         ...
 *     }
 */
hdlProcessBlockArguments
    : LPAREN
      hdlProcessArgumentList?
      RPAREN
    ;


hdlProcessArgumentList
    : hdlProcessArgument
      (
          COMMA
          hdlProcessArgument
      )*
      COMMA?
    ;


hdlProcessArgument
    : identifier
      COLON
      hdlExpression
    | identifier
      ASSIGN
      hdlExpression
    | hdlExpression
    ;


/* ============================================================================
 * PROCESS EVENT GROUP
 * ========================================================================== */

/*
 * Event grouping is deliberately syntax-only.
 *
 * This allows a process to describe an event relationship without encoding
 * a particular hardware implementation.
 */
hdlProcessEvent
    : identifier
      LPAREN
      hdlProcessSensitivityList?
      RPAREN
    ;


/* ============================================================================
 * PROCESS BODY EXTENSION POINT
 * ========================================================================== */

/*
 * A process event may contain a nested process body.
 *
 * This rule is intentionally separate from hdlProcessEvent so the semantic
 * layer can distinguish:
 *
 *     process event declaration
 *
 * from:
 *
 *     process body execution.
 */
hdlProcessEventBody
    : hdlProcessEvent
      LBRACE
      hdlProcessItem*
      RBRACE
    ;


/* ============================================================================
 * PROCESS SEMANTIC PROPERTY BLOCK
 * ========================================================================== */

/*
 * Structured property blocks allow future process metadata to be represented
 * without changing the core process declaration.
 *
 * Example:
 *
 *     process counter {
 *         timing {
 *             ...
 *         }
 *     }
 *
 *     process controller {
 *         reset {
 *             ...
 *         }
 *     }
 *
 * The semantic owner determines which property blocks are recognized.
 */
hdlProcessPropertyBlock
    : identifier
      LBRACE
      hdlProcessItem*
      RBRACE
    ;


/* ============================================================================
 * PROCESS ITEM EXTENSION
 * ========================================================================== */

/*
 * Central process-item extension rule.
 *
 * Keep this rule small.
 *
 * Domain-specific semantics must be added by their owning grammar rather than
 * by duplicating them here.
 */
hdlProcessExtension
    : hdlProcessBinding
    | hdlProcessEventBody
    | hdlProcessPropertyBlock
    ;


/* ============================================================================
 * INTEGRATION CONTRACT
 * ========================================================================== */

/*
 * REQUIRED IMPORT/COMPOSITION CONTRACT
 *
 * The canonical HDL parser must import this grammar and expose:
 *
 *     hdlProcessDeclaration
 *
 * to its HDL module-member dispatcher.
 *
 * The canonical HDL composition must NOT retain another independent
 * hdlProcessDeclaration definition.
 *
 *
 * REQUIRED SHARED RULE CONTRACTS
 *
 * This grammar consumes:
 *
 *     identifier
 *     hdlKeyword
 *     hdlExpression
 *     hdlStatement
 *     hdlAttributeList
 *
 * and must not redefine them.
 *
 *
 * REQUIRED OWNERSHIP RELATIONSHIPS
 *
 * clocks.g4
 *     owns clock declarations.
 *
 * timing.g4
 *     owns timing constraints and timing semantics.
 *
 * sequential.g4
 *     owns sequential-specific structural syntax.
 *
 * combinational.g4
 *     owns combinational-specific structural syntax.
 *
 * state-machines.g4
 *     owns state/transition semantics.
 *
 * registers.g4
 *     owns register declarations and register-specific structure.
 *
 * signals.g4
 *     owns signal declarations.
 *
 * wires.g4
 *     owns wire/net declarations.
 *
 * processes.g4
 *     owns process structure and process sensitivity/event syntax.
 *
 * No child component may redefine another component's ownership.
 */


/* ============================================================================
 * SEMANTIC INTEGRATION CONTRACT
 * ========================================================================== */

/*
 * The AST layer should lower this grammar to a process node containing at
 * least the semantic categories:
 *
 *     ProcessId
 *     optional ProcessName
 *     Sensitivity/Event specification
 *     ProcessAttributes
 *     ProcessBindings
 *     ProcessBody
 *
 * The AST must preserve source spans for diagnostics.
 *
 * The grammar itself does not construct the AST.
 *
 *
 * Semantic analysis must determine:
 *
 *     - whether the process keyword/name is valid;
 *     - whether sensitivity expressions are valid;
 *     - whether event expressions refer to valid signals/clocks/events;
 *     - whether bindings are legal;
 *     - whether bindings conflict;
 *     - whether reset semantics are valid;
 *     - whether enable semantics are valid;
 *     - whether process behavior is combinational;
 *     - whether process behavior is sequential;
 *     - whether multiple drivers exist;
 *     - whether assignments are legal;
 *     - whether clock-domain crossings are valid;
 *     - whether the process is synthesizable;
 *     - whether target capabilities can realize the process.
 *
 * None of these checks belong in this grammar.
 */


/* ============================================================================
 * HARDWARE INTEGRATION CONTRACT
 * ========================================================================== */

/*
 * A process may reference hardware-level symbols, but this grammar does not
 * resolve them.
 *
 * Resolution occurs downstream:
 *
 *     process syntax
 *          |
 *          v
 *     semantic symbol resolution
 *          |
 *          +--> signal
 *          +--> wire
 *          +--> register
 *          +--> clock
 *          +--> reset
 *          +--> state
 *          |
 *          v
 *     canonical HDL/hardware semantic model
 *
 * Physical realization then proceeds through:
 *
 *     scheduling
 *     placement
 *     routing
 *     synthesis
 *     target lowering
 *
 * This prevents a process declaration from becoming a hidden physical
 * hardware description.
 */


/* ============================================================================
 * QUANTUM / HYBRID INTEGRATION CONTRACT
 * ========================================================================== */

/*
 * A process may participate in a hybrid quantum/classical/hardware design,
 * but processes.g4 must not define quantum operations.
 *
 * If a process body contains quantum syntax, that syntax belongs to the
 * quantum grammar and its semantic integration layer.
 *
 * Quantum semantics ultimately cross the canonical quantum::ir boundary.
 *
 * This grammar therefore has no:
 *
 *     qubit count
 *     quantum device ID
 *     QPU topology
 *     fixed quantum register size
 *     quantum gate implementation
 *
 * Any quantum/hardware interaction is resolved by the frontend semantic and
 * canonical IR layers.
 */


/* ============================================================================
 * RESOURCE / CAPABILITY CONTRACT
 * ========================================================================== */

/*
 * Process syntax may express logical requirements through bindings or
 * attributes.
 *
 * It must not convert them into physical resource selections.
 *
 * For example:
 *
 *     clock: clk;
 *
 * identifies a logical clock.
 *
 * It does not mean:
 *
 *     use physical clock #0
 *
 * or:
 *
 *     use a fixed clock frequency.
 *
 * Likewise:
 *
 *     requires: property
 *
 * must not imply:
 *
 *     use device X.
 *
 * Resource discovery and capability matching belong downstream.
 */


/* ============================================================================
 * DETERMINISM CONTRACT
 * ========================================================================== */

/*
 * Parsing this grammar must be deterministic for a fixed:
 *
 *     source
 *     canonical token vocabulary
 *     grammar version
 *
 * No semantic state, mutable global state, environment lookup, filesystem
 * lookup, network lookup, or hardware discovery may influence parsing.
 */


/* ============================================================================
 * SAFETY CONTRACT
 * ========================================================================== */

/*
 * This grammar contains no:
 *
 *     @members
 *     @header
 *     Rust actions
 *     Rust predicates
 *     embedded executable code
 *     unsafe code
 *     filesystem access
 *     network access
 *     environment-dependent behavior
 *     hardware discovery
 *
 * Generated Rust must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * Zamani-owned Rust source must continue to enforce safe Rust.
 */


/* ============================================================================
 * SCALABILITY CONTRACT
 * ========================================================================== */

/*
 * Process size is unbounded by language grammar.
 *
 * The following are intentionally represented with repetition:
 *
 *     sensitivity items
 *     process statements
 *     process bindings
 *     nested process blocks
 *     process arguments
 *     attributes
 *
 * No grammar production introduces an artificial finite ceiling.
 *
 * Very large programs remain subject only to:
 *
 *     available memory
 *     parser implementation limits
 *     compiler resource policy
 *     semantic analysis resources
 *     build configuration
 *     deployment resources
 *
 * Those are not language-level semantic limits.
 */


/* ============================================================================
 * ERROR/DIAGNOSTIC CONTRACT
 * ========================================================================== */

/*
 * Syntax diagnostics originate from the parser.
 *
 * Semantic diagnostics must originate from the semantic layer.
 *
 * The grammar must never:
 *
 *     silently discard invalid process constructs;
 *     encode errors as comments;
 *     print to stdout;
 *     execute recovery logic;
 *     invent target-specific defaults.
 *
 * Error recovery must preserve source locations so downstream diagnostics can
 * identify:
 *
 *     process
 *     sensitivity item
 *     binding
 *     nested block
 *     process statement
 *
 * precisely.
 */
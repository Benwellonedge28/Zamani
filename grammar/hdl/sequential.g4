parser grammar sequential;

options {
    tokenVocab = ZamaniTokens;
}

/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hdl/sequential.g4
 *
 * Purpose:
 *     Production HDL sequential-behavior grammar delegate.
 *
 * Baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no embedded Rust code.
 *     Generated/compiler/runtime Rust MUST remain safe Rust.
 *     Rust `unsafe` is prohibited.
 *
 * ============================================================================
 * ARCHITECTURAL ROLE
 * ============================================================================
 *
 * Sequential hardware describes behavior whose semantic interpretation may
 * depend on state, storage, clock/event boundaries, reset behavior, enables,
 * or other explicitly declared sequential conditions.
 *
 * This grammar describes SOURCE SYNTAX ONLY.
 *
 * It does not:
 *
 *     - create hardware;
 *     - allocate registers;
 *     - select a clock tree;
 *     - select an FPGA;
 *     - select an ASIC;
 *     - select a QPU;
 *     - determine physical placement;
 *     - determine physical routing;
 *     - perform timing closure;
 *     - schedule operations;
 *     - allocate resources;
 *     - synthesize gates;
 *     - select vendor primitives;
 *     - create a second IR.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - sequential declaration syntax;
 *     - sequential body syntax;
 *     - sequential event specifications;
 *     - clock/reset/enable relationships at source level;
 *     - sequential assignments;
 *     - sequential conditionals;
 *     - sequential selection;
 *     - sequential local declarations;
 *     - sequential nested blocks;
 *     - sequential assertions;
 *     - sequential generation syntax;
 *     - source-level sequential attributes.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - general expressions;
 *     - general types;
 *     - signal declarations;
 *     - register declarations;
 *     - memory declarations;
 *     - clock declarations;
 *     - timing constraints;
 *     - combinational logic;
 *     - state-machine semantics;
 *     - pipeline semantics;
 *     - process ownership;
 *     - hardware capabilities;
 *     - hardware resources;
 *     - physical topology;
 *     - synthesis;
 *     - scheduling;
 *     - routing;
 *     - placement;
 *     - optimization;
 *     - runtime execution.
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 *     canonical lexer
 *          |
 *          v
 *     ZamaniTokens
 *          |
 *          v
 *     HDL parser
 *          |
 *          +--> clocks.g4
 *          +--> timing.g4
 *          +--> registers.g4
 *          +--> combinational.g4
 *          +--> THIS FILE
 *          +--> processes.g4
 *          +--> state-machines.g4
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> name resolution
 *          +--> type checking
 *          +--> width/shape checking
 *          +--> driver analysis
 *          +--> sequential legality
 *          +--> state analysis
 *          +--> clock/reset analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          |
 *          v
 *     canonical hardware semantic representation
 *          |
 *          +--> optimization
 *          +--> synthesis
 *          +--> scheduling
 *          +--> placement
 *          +--> routing
 *          |
 *          v
 *     target lowering
 *
 * The grammar MUST NOT depend on any downstream compiler or runtime subsystem.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Sequential source describes semantic state transitions.
 *
 * It MUST NOT encode permanent assumptions about:
 *
 *     - number of registers;
 *     - number of bits;
 *     - number of clocks;
 *     - number of state elements;
 *     - clock-tree topology;
 *     - FPGA family;
 *     - ASIC family;
 *     - process technology;
 *     - physical register location;
 *     - physical pin;
 *     - physical address;
 *     - device identifier;
 *     - routing fabric;
 *     - hardware capacity.
 *
 * All such information belongs to downstream capability/resource/target
 * descriptions when it is required for realization.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * No finite grammar-level limits are permitted.
 *
 * There is deliberately no:
 *
 *     MAX_REGISTERS
 *     MAX_STATES
 *     MAX_CLOCKS
 *     MAX_EVENTS
 *     MAX_BRANCHES
 *     MAX_ASSIGNMENTS
 *     MAX_NESTING
 *     MAX_WIDTH
 *     MAX_BITS
 *     MAX_PROCESSES
 *     MAX_MODULES
 *
 * Repetition is expressed structurally with ANTLR operators:
 *
 *     *
 *     +
 *     ?
 *
 * Physical limits are evaluated by semantic analysis, compilation, scheduling,
 * synthesis, deployment, or runtime resource policies.
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * This grammar uses the canonical ZamaniTokens vocabulary.
 *
 * A dedicated:
 *
 *     K_SEQUENTIAL
 *
 * token MUST exist in the canonical token vocabulary.
 *
 * The grammar deliberately does not define its own lexer rules.
 *
 * Contextual sequential terms such as:
 *
 *     clock
 *     reset
 *     enable
 *     sync
 *     async
 *     rising
 *     falling
 *     edge
 *     state
 *     next
 *     default
 *
 * are interpreted by semantic analysis unless the language-wide lexer promotes
 * a particular term to a reserved keyword.
 *
 * ============================================================================
 */


/* ============================================================================
 * PUBLIC ENTRY POINT
 * ========================================================================== */

/*
 * Named or anonymous sequential hardware behavior.
 *
 * Examples:
 *
 *     sequential {
 *         ...
 *     }
 *
 *     sequential controller {
 *         ...
 *     }
 *
 * The name is a logical source-level name.
 * It is not a physical device identifier.
 */
hdlSequentialDeclaration
    : hdlSequentialModifiers*
      K_SEQUENTIAL
      identifier?
      hdlSequentialAttributes?
      hdlSequentialEventSpecification?
      hdlSequentialBody
    ;


/* ============================================================================
 * MODIFIERS
 * ========================================================================== */

/*
 * Modifiers are intentionally contextual.
 *
 * Semantic analysis determines whether a modifier is legal for a particular
 * sequential construct.
 */
hdlSequentialModifiers
    : hdlKeyword
    ;


/* ============================================================================
 * ATTRIBUTES
 * ========================================================================== */

hdlSequentialAttributes
    : hdlAttribute+
    ;


/*
 * Sequential attributes reuse the repository-wide attribute mechanism.
 *
 * The grammar does not assign implementation meaning to attributes.
 */
hdlSequentialAttribute
    : AT
      identifier
      (
          LPAREN
          hdlSequentialArgumentList?
          RPAREN
      )?
    ;


hdlSequentialArgumentList
    : hdlSequentialArgument
      (
          COMMA
          hdlSequentialArgument
      )*
      COMMA?
    ;


hdlSequentialArgument
    : identifier
      (
          ASSIGN
          hdlExpression
      )?
    | hdlExpression
    ;


/* ============================================================================
 * EVENT / CLOCK SPECIFICATION
 * ========================================================================== */

/*
 * Sequential behavior may optionally declare the source-level events that
 * establish its state-transition boundary.
 *
 * Examples:
 *
 *     sequential @(clock) { ... }
 *
 *     sequential @(rising clock) { ... }
 *
 *     sequential @(falling clock) { ... }
 *
 *     sequential @(clock, reset) { ... }
 *
 * The actual legality of event combinations is semantic.
 */
hdlSequentialEventSpecification
    : LPAREN
      hdlSequentialEventList?
      RPAREN
    ;


hdlSequentialEventList
    : hdlSequentialEvent
      (
          COMMA
          hdlSequentialEvent
      )*
      COMMA?
    ;


hdlSequentialEvent
    : hdlSequentialEdge?
      hdlSequentialEventSource
      hdlSequentialEventQualifier*
    ;


hdlSequentialEdge
    : hdlKeyword
    ;


hdlSequentialEventSource
    : qualifiedHdlName
    | identifier
    | hdlExpression
    ;


hdlSequentialEventQualifier
    : hdlKeyword
    | hdlAttribute
    ;


/* ============================================================================
 * BODY
 * ========================================================================== */

hdlSequentialBody
    : LBRACE
      hdlSequentialStatement*
      RBRACE
    ;


hdlSequentialStatement
    : hdlSequentialAssignment
    | hdlSequentialIf
    | hdlSequentialCase
    | hdlSequentialLocalDeclaration
    | hdlSequentialExpressionStatement
    | hdlSequentialAssertion
    | hdlSequentialBlock
    | hdlSequentialGenerate
    ;


/* ============================================================================
 * ASSIGNMENT
 * ========================================================================== */

/*
 * Sequential assignment is syntactically distinct from combinational
 * assignment by its containing sequential semantic region.
 *
 * This grammar intentionally uses the canonical ASSIGN token because the
 * current Zamani token vocabulary must remain the sole operator authority.
 *
 * If Zamani later standardizes a distinct sequential/non-blocking assignment
 * operator, that operator MUST be introduced centrally in the canonical lexer
 * and operator grammar first; this file must not invent a private token.
 */
hdlSequentialAssignment
    : hdlLValue
      ASSIGN
      hdlExpression
      SEMICOLON
    ;


/*
 * Attribute-bearing sequential assignment.
 */
hdlSequentialAnnotatedAssignment
    : hdlSequentialAttributes
      hdlSequentialAssignment
    ;


/* ============================================================================
 * CONDITIONAL STATE TRANSITIONS
 * ========================================================================== */

/*
 * Conditional state update.
 *
 * Semantic analysis determines:
 *
 *     - whether all state elements are properly defined;
 *     - whether branches conflict;
 *     - whether reset dominates normal operation;
 *     - whether enables are legal;
 *     - whether a construct infers storage;
 *     - whether the resulting state transition is deterministic.
 */
hdlSequentialIf
    : K_IF
      LPAREN
      hdlExpression
      RPAREN
      hdlSequentialBody
      hdlSequentialElseClause?
    ;


hdlSequentialElseClause
    : K_ELSE
      (
          hdlSequentialIf
        | hdlSequentialBody
      )
    ;


/* ============================================================================
 * CASE / STATE SELECTION
 * ========================================================================== */

/*
 * Case-style state transition selection.
 *
 * Exhaustiveness, overlap, duplicate patterns, and default semantics belong
 * to semantic analysis.
 */
hdlSequentialCase
    : K_CASE
      LPAREN
      hdlExpression
      RPAREN
      LBRACE
      hdlSequentialCaseItem*
      RBRACE
    ;


hdlSequentialCaseItem
    : hdlSequentialCasePattern
      COLON
      hdlSequentialBody
    | K_DEFAULT
      COLON
      hdlSequentialBody
    ;


hdlSequentialCasePattern
    : hdlExpression
      (
          COMMA
          hdlExpression
      )*
    ;


/* ============================================================================
 * LOCAL DECLARATIONS
 * ========================================================================== */

/*
 * Local values inside a sequential region do not automatically imply
 * persistent hardware storage.
 *
 * Whether a local becomes state, combinational logic, or an implementation
 * artifact is determined downstream.
 */
hdlSequentialLocalDeclaration
    : hdlKeyword
      identifier
      (
          COLON
          hdlTypeExpression
      )?
      (
          ASSIGN
          hdlExpression
      )?
      SEMICOLON
    ;


/* ============================================================================
 * EXPRESSION STATEMENTS
 * ========================================================================== */

hdlSequentialExpressionStatement
    : hdlExpression
      SEMICOLON
    ;


/* ============================================================================
 * NESTED BLOCKS
 * ========================================================================== */

hdlSequentialBlock
    : LBRACE
      hdlSequentialStatement*
      RBRACE
    ;


/* ============================================================================
 * RESET
 * ========================================================================== */

/*
 * Reset is represented as a source-level state-transition condition.
 *
 * It does not select a physical reset network.
 */
hdlSequentialResetClause
    : hdlKeyword
      (
          LPAREN
          hdlExpression
          RPAREN
      )?
    ;


/*
 * Explicit reset-oriented sequential region.
 *
 * Example conceptual form:
 *
 *     reset {
 *         state = initial;
 *     }
 *
 * The contextual keyword is interpreted semantically.
 */
hdlSequentialResetRegion
    : hdlKeyword
      hdlSequentialBody
    ;


/* ============================================================================
 * ENABLE
 * ========================================================================== */

/*
 * Enables are source-level predicates controlling whether a sequential
 * transition is applied.
 */
hdlSequentialEnableClause
    : hdlKeyword
      (
          LPAREN
          hdlExpression
          RPAREN
      )?
    ;


/* ============================================================================
 * DEFAULT / HOLD SEMANTICS
 * ========================================================================== */

/*
 * A sequential implementation may intentionally retain the current state
 * when no transition assignment occurs.
 *
 * The grammar does not introduce a machine-specific "hold register"
 * operation. Absence of an assignment is interpreted by semantic analysis.
 */
hdlSequentialDefaultClause
    : K_DEFAULT
      COLON
      hdlSequentialBody
    ;


/* ============================================================================
 * GENERATION
 * ========================================================================== */

/*
 * Sequential generation is source-level elaboration syntax.
 *
 * It must never imply a fixed number of physical state elements.
 *
 * Cardinality may depend on:
 *
 *     - generic parameters;
 *     - compile-time expressions;
 *     - type-level values;
 *     - semantic elaboration.
 */
hdlSequentialGenerate
    : hdlKeyword
      (
          hdlSequentialGenerateBody
        | hdlExpression
          hdlSequentialGenerateBody
      )
    ;


hdlSequentialGenerateBody
    : hdlSequentialBody
    ;


/* ============================================================================
 * ASSERTIONS
 * ========================================================================== */

/*
 * Assertions may feed:
 *
 *     - simulation;
 *     - formal verification;
 *     - synthesis-time validation;
 *     - equivalence checking;
 *     - verification tooling.
 *
 * They do not directly determine implementation.
 */
hdlSequentialAssertion
    : hdlKeyword
      hdlExpression
      SEMICOLON
    ;


/* ============================================================================
 * STATE UPDATE GROUP
 * ========================================================================== */

/*
 * Explicit grouping of state updates.
 *
 * This allows future semantic analysis to preserve source-level update
 * boundaries without hard-coding the number of updates.
 */
hdlSequentialUpdateGroup
    : hdlKeyword
      hdlSequentialBody
    ;


/* ============================================================================
 * CLOCK / RESET RELATIONSHIPS
 * ========================================================================== */

/*
 * Source-level relationship declarations are intentionally generic.
 *
 * Examples of concepts that semantic analysis may recognize:
 *
 *     clock
 *     reset
 *     enable
 *     synchronizer
 *     asynchronous
 *     synchronous
 *
 * The grammar does not impose a fixed clock topology.
 */
hdlSequentialControlClause
    : hdlKeyword
      (
          LPAREN
          hdlExpression
          RPAREN
      )?
    ;


/* ============================================================================
 * SEQUENTIAL VALUE SELECTION
 * ========================================================================== */

/*
 * A source-level value transition can be expressed as an ordinary expression.
 *
 * This rule exists as an explicit semantic boundary so future sequential
 * expression extensions do not require changing the enclosing declaration.
 */
hdlSequentialValue
    : hdlExpression
    ;


/* ============================================================================
 * SEQUENTIAL STATE TARGET
 * ========================================================================== */

/*
 * State targets use the common HDL lvalue contract.
 *
 * The grammar deliberately does not introduce:
 *
 *     physical_register
 *     register_number
 *     flip_flop_id
 *     hardware_address
 *
 * or any equivalent machine-specific identifier.
 */
hdlSequentialStateTarget
    : hdlLValue
    ;


/* ============================================================================
 * SEQUENTIAL TRANSITION
 * ========================================================================== */

/*
 * A transition is a source-level update from a current semantic state to a
 * next semantic value.
 *
 * The grammar does not construct the state-transition IR.
 */
hdlSequentialTransition
    : hdlSequentialStateTarget
      ASSIGN
      hdlSequentialValue
      SEMICOLON
    ;


/* ============================================================================
 * MULTIPLE STATE TRANSITIONS
 * ========================================================================== */

/*
 * Arbitrarily many transitions are supported.
 */
hdlSequentialTransitionList
    : hdlSequentialTransition+
    ;


/* ============================================================================
 * SEQUENTIAL REGION WITH EXPLICIT CONTROLS
 * ========================================================================== */

/*
 * This form provides an explicit structural place for clock/reset/enable
 * metadata without embedding physical implementation.
 *
 * Example conceptual syntax:
 *
 *     sequential controller
 *         clock(clk)
 *         reset(reset_n)
 *         enable(enable)
 *     {
 *         ...
 *     }
 */
hdlSequentialControlledDeclaration
    : hdlSequentialModifiers*
      K_SEQUENTIAL
      identifier?
      hdlSequentialControlClause*
      hdlSequentialAttributes?
      hdlSequentialBody
    ;


/* ============================================================================
 * SEQUENTIAL PROCESS COMPATIBILITY BOUNDARY
 * ========================================================================== */

/*
 * Processes belong to processes.g4.
 *
 * This rule is intentionally only a syntactic delegation boundary.
 *
 * The parent HDL grammar may use this rule when a process is explicitly
 * classified as sequential.
 */
hdlSequentialProcessBody
    : hdlSequentialBody
    ;


/* ============================================================================
 * FORBIDDEN OWNERSHIP
 * ========================================================================== */

/*
 * The following concepts MUST NOT be implemented as grammar-owned hardware
 * semantics here:
 *
 *     - physical flip-flop type;
 *     - FPGA slice;
 *     - LUT;
 *     - ASIC cell;
 *     - physical register number;
 *     - physical clock-tree node;
 *     - physical reset network;
 *     - device ID;
 *     - pin number;
 *     - physical timing delay;
 *     - routing resource;
 *     - placement coordinate;
 *     - hardware address.
 *
 * Those belong to target/resource/capability models.
 */


/* ============================================================================
 * SEMANTIC VALIDATION CONTRACT
 * ========================================================================== */

/*
 * Downstream semantic analysis MUST validate at least:
 *
 *     - identifiers resolve;
 *     - assignment targets are legal;
 *     - assignment types are compatible;
 *     - widths/shapes are compatible;
 *     - state targets are writable;
 *     - clock/event sources are valid;
 *     - event combinations are legal;
 *     - reset semantics are valid;
 *     - synchronous/asynchronous classification is legal;
 *     - enable predicates are valid;
 *     - conflicting state updates are diagnosed;
 *     - unreachable transitions are diagnosed where required;
 *     - incomplete state behavior is diagnosed where required;
 *     - illegal combinational/sequential mixing is diagnosed;
 *     - unsupported constructs are diagnosed;
 *     - generated cardinality is resource-checked downstream;
 *     - timing requirements are checked by timing analysis;
 *     - target realizability is checked outside this grammar.
 *
 * None of these checks belong in parser actions or predicates.
 */


/* ============================================================================
 * AST CONTRACT
 * ========================================================================== */

/*
 * The parser must expose enough structure for the frontend AST to represent:
 *
 *     SequentialDeclaration
 *         - name
 *         - modifiers
 *         - attributes
 *         - events
 *         - body
 *
 *     SequentialEvent
 *         - edge
 *         - source
 *         - qualifiers
 *
 *     SequentialAssignment
 *         - target
 *         - value
 *
 *     SequentialIf
 *         - condition
 *         - then
 *         - else
 *
 *     SequentialCase
 *         - selector
 *         - cases
 *
 *     SequentialLocal
 *         - name
 *         - type
 *         - initializer
 *
 *     SequentialAssertion
 *         - condition/expression
 *
 * The grammar itself MUST NOT construct these AST objects.
 */


/* ============================================================================
 * IR CONTRACT
 * ========================================================================== */

/*
 * This grammar MUST NOT define a sequential IR.
 *
 * AST lowering belongs to the frontend semantic layer.
 *
 * The resulting semantic representation may then be consumed by:
 *
 *     - HDL/hardware IR;
 *     - optimization;
 *     - scheduling;
 *     - synthesis;
 *     - timing analysis;
 *     - placement;
 *     - routing;
 *     - target lowering.
 *
 * Quantum syntax and quantum::ir remain separate.
 *
 * This file MUST NOT depend on quantum::ir.
 */


/* ============================================================================
 * RUNTIME CONTRACT
 * ========================================================================== */

/*
 * Runtime MUST NOT depend directly on this grammar.
 *
 * Runtime receives compiled/lowered representations rather than reparsing
 * source-level sequential declarations.
 */


/* ============================================================================
 * DETERMINISM CONTRACT
 * ========================================================================== */

/*
 * Given the same token stream and parser configuration, this grammar must
 * produce deterministic parse structure.
 *
 * No:
 *
 *     - random behavior;
 *     - timestamps;
 *     - environment reads;
 *     - filesystem access;
 *     - network access;
 *     - hardware discovery;
 *     - runtime state;
 *
 * may affect parsing.
 */


/* ============================================================================
 * SAFETY CONTRACT
 * ========================================================================== */

/*
 * No embedded target-language actions are permitted.
 *
 * In particular:
 *
 *     - no Rust action blocks;
 *     - no Rust semantic predicates;
 *     - no unsafe;
 *     - no FFI;
 *     - no filesystem operations;
 *     - no process execution;
 *     - no network access.
 *
 * Rust 1.97 / Rust 1.97.1 compatibility is established by the generated
 * frontend/compiler integration, not by embedding Rust into this grammar.
 */


/* ============================================================================
 * COMPATIBILITY CONTRACT
 * ========================================================================== */

/*
 * This file owns stable parser rule names beginning with:
 *
 *     hdlSequential
 *
 * Existing callers must migrate to these names rather than retaining a
 * duplicate sequential grammar in hdl.g4.
 *
 * Deprecated sequential rules must be removed only after all parser imports
 * and generated parser references have been migrated.
 */


/* ============================================================================
 * INTEGRATION CONTRACT
 * ========================================================================== */

/*
 * hdl.g4 MUST:
 *
 *     1. import this grammar as a delegate;
 *     2. retain the single public HDL module/design entry point;
 *     3. remove its duplicate hdlSequentialDeclaration rule;
 *     4. route sequential constructs to hdlSequentialDeclaration;
 *     5. retain ownership of module composition rather than sequential
 *        implementation details.
 *
 * combinational.g4 MUST NOT import sequential semantics.
 *
 * clocks.g4 owns clock declaration syntax.
 *
 * timing.g4 owns timing constraint syntax.
 *
 * registers.g4 owns register declaration syntax.
 *
 * processes.g4 owns generic process syntax.
 *
 * state-machines.g4 owns explicit state-machine syntax.
 *
 * No cyclic grammar dependency is permitted.
 */


/* ============================================================================
 * INTEGRATION WITH OTHER ZAMANI SUBSYSTEMS
 * ========================================================================== */

/*
 * Lexer:
 *     canonical ZamaniTokens only.
 *
 * AST:
 *     parser structure is lowered into the frontend AST.
 *
 * Semantic analysis:
 *     owns sequential legality and meaning.
 *
 * Type system:
 *     validates values, targets, widths and shapes.
 *
 * Capability system:
 *     determines whether a requested sequential semantic can be realized.
 *
 * Resource system:
 *     determines resource requirements.
 *
 * Timing:
 *     determines whether clock/event behavior satisfies timing constraints.
 *
 * Scheduling:
 *     may schedule implementation operations after semantic lowering.
 *
 * Hardware abstraction:
 *     provides target-independent capabilities and target-specific facts.
 *
 * Optimization:
 *     may transform the lowered semantic representation while preserving
 *     sequential meaning.
 *
 * Synthesis:
 *     determines physical implementation.
 *
 * Runtime:
 *     consumes lowered/compiled artifacts.
 *
 * quantum::ir:
 *     NO dependency.
 *
 * QEC:
 *     NO dependency.
 *
 * ZQN:
 *     NO dependency.
 *
 * Those systems are downstream or independent semantic domains and must not
 * become parser dependencies.
 */


/* ============================================================================
 * SCALABILITY INVARIANTS
 * ========================================================================== */

/*
 * The grammar imposes no fixed limit on:
 *
 *     - sequential declarations;
 *     - state targets;
 *     - assignments;
 *     - events;
 *     - branches;
 *     - cases;
 *     - nesting;
 *     - generated regions;
 *     - width;
 *     - state count;
 *     - clocks;
 *     - resets;
 *     - enables.
 *
 * All collections use grammar repetition rather than fixed cardinality.
 *
 * No source-level construct may imply:
 *
 *     "this machine has N registers"
 *
 * unless N is itself explicitly part of the program's semantic model.
 */


/* ============================================================================
 * COMPLETION CRITERIA
 * ========================================================================== */

/*
 * This file is COMPLETE only when:
 *
 *     [ ] canonical ZamaniTokens contains K_SEQUENTIAL;
 *
 *     [ ] hdl.g4 imports this delegate;
 *
 *     [ ] duplicate hdlSequentialDeclaration in hdl.g4 is removed;
 *
 *     [ ] no second sequential lexer exists;
 *
 *     [ ] all referenced shared HDL rules resolve;
 *
 *     [ ] parser generation succeeds;
 *
 *     [ ] sequential positive tests pass;
 *
 *     [ ] invalid sequential syntax tests pass;
 *
 *     [ ] semantic-invalid sequential constructs are rejected downstream;
 *
 *     [ ] clock/reset/enable combinations are tested;
 *
 *     [ ] nested sequential regions are tested;
 *
 *     [ ] arbitrarily repeated assignments/cases are tested;
 *
 *     [ ] large generated sequential descriptions are tested;
 *
 *     [ ] deterministic parsing is verified;
 *
 *     [ ] round-trip parsing is verified where supported;
 *
 *     [ ] no machine-size constants exist;
 *
 *     [ ] no physical hardware identifiers are embedded;
 *
 *     [ ] no target-specific actions exist;
 *
 *     [ ] no Rust unsafe code is introduced;
 *
 *     [ ] Rust 1.97 / Rust 1.97.1 integration succeeds;
 *
 *     [ ] AST lowering succeeds without grammar changes;
 *
 *     [ ] downstream hardware semantic lowering succeeds;
 *
 *     [ ] no dependency cycle exists.
 */
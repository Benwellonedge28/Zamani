parser grammar sequential;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hdl/sequential.g4
 *
 * Status:
 *     Canonical HDL sequential-behavior parser component.
 *
 * Purpose:
 *     Defines target-independent sequential hardware behavior:
 *
 *         - sequential regions;
 *         - event/sensitivity specifications;
 *         - sequential assignments;
 *         - conditional state transitions;
 *         - case-based state transitions;
 *         - sequential local values;
 *         - sequential assertions;
 *         - nested sequential blocks.
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Edition 2021
 *
 * Safety:
 *     This grammar contains no Rust actions or unsafe code.
 *     Zamani compiler/runtime implementation MUST remain safe Rust.
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * Sequential HDL syntax describes STATE-TRANSITION INTENT.
 *
 * It does NOT describe:
 *
 *     - physical registers;
 *     - flip-flop cell types;
 *     - physical clock trees;
 *     - physical reset networks;
 *     - FPGA primitives;
 *     - ASIC cells;
 *     - physical pins;
 *     - placement;
 *     - routing;
 *     - timing closure;
 *     - vendor APIs;
 *     - target device identifiers;
 *     - resource allocation;
 *     - scheduling implementation;
 *     - synthesis implementation.
 *
 * Those concerns belong to downstream semantic analysis, hardware IR,
 * resource/capability analysis, synthesis, scheduling, routing, placement,
 * and target lowering.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - sequential declaration syntax;
 *     - sequential body syntax;
 *     - sequential event/sensitivity syntax;
 *     - sequential assignments;
 *     - sequential conditionals;
 *     - sequential case selection;
 *     - sequential local-value declarations;
 *     - sequential assertion statements;
 *     - nested sequential blocks.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer rules;
 *     - identifiers;
 *     - literals;
 *     - general expressions;
 *     - general types;
 *     - HDL ports;
 *     - HDL signals;
 *     - HDL nets;
 *     - physical/logical register declarations;
 *     - memories;
 *     - clock declarations;
 *     - timing constraints;
 *     - combinational behavior;
 *     - state-machine declarations;
 *     - pipeline declarations;
 *     - generate/elaboration constructs;
 *     - process declarations;
 *     - resource declarations;
 *     - capability declarations;
 *     - synthesis;
 *     - scheduling;
 *     - routing;
 *     - placement;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime behavior.
 *
 * ============================================================================
 * SHARED CONTRACTS
 * ============================================================================
 *
 * The following rules are supplied by the canonical HDL composition:
 *
 *     identifier
 *     qualifiedHdlName
 *     hdlExpression
 *     hdlTypeExpression
 *     hdlLValue
 *
 * They MUST have one authoritative owner.
 *
 * This file MUST NOT define another expression, type, identifier, or lvalue
 * grammar.
 *
 * ============================================================================
 * ATTRIBUTE OWNERSHIP
 * ============================================================================
 *
 * Declaration-level attributes are owned by the enclosing HDL declaration
 * dispatcher.
 *
 * Therefore this file intentionally DOES NOT put:
 *
 *     hdlAttribute*
 *
 * inside `hdlSequentialDeclaration`.
 *
 * The enclosing composition rule is responsible for attaching attributes to
 * the sequential declaration AST node.
 *
 * ============================================================================
 * STRUCTURAL OWNERSHIP
 * ============================================================================
 *
 * `generate.g4` owns structural/elaboration-time repetition.
 *
 * `state-machines.g4` owns explicit state-machine declarations.
 *
 * `pipelines.g4` owns pipeline declarations.
 *
 * `processes.g4` owns generic process declarations.
 *
 * `clocks.g4` owns clock declarations.
 *
 * `reset.g4` owns standalone reset declarations/intent where applicable.
 *
 * `timing.g4` owns timing constraints.
 *
 * Sequential behavior may REFER to these constructs through shared expressions
 * and names, but must not duplicate their declarations.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * This grammar contains no universal hardware capacities.
 *
 * It does NOT define:
 *
 *     MAX_REGISTERS
 *     MAX_BITS
 *     MAX_WIDTH
 *     MAX_STATES
 *     MAX_CLOCKS
 *     MAX_PROCESSES
 *     MAX_EVENTS
 *     MAX_BRANCHES
 *     MAX_MEMORY
 *     MAX_DEVICES
 *     MAX_PIPELINE_STAGES
 *     MAX_MODULES
 *
 * Nor does it assume:
 *
 *     32-bit registers;
 *     64-bit registers;
 *     fixed clock counts;
 *     fixed reset counts;
 *     fixed state counts;
 *     fixed FPGA resources;
 *     fixed ASIC resources;
 *     fixed CPU/GPU resources;
 *     fixed QPU resources.
 *
 * Program cardinality is constrained only by available implementation
 * resources and explicit semantic requirements.
 *
 * ============================================================================
 * SEMANTIC MODEL
 * ============================================================================
 *
 * The intended lowering is:
 *
 *     sequential declaration
 *             |
 *             v
 *     domain-neutral HDL AST
 *             |
 *             v
 *     sequential semantic analysis
 *             |
 *             v
 *     canonical hardware semantic representation / IR
 *             |
 *             +--> optimization
 *             +--> synthesis
 *             +--> scheduling
 *             +--> timing analysis
 *             +--> placement
 *             +--> routing
 *             +--> target lowering
 *
 * This grammar does NOT introduce a second sequential IR.
 *
 * ============================================================================
 */


/* ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 */

/*
 * Named or anonymous sequential behavior.
 *
 * Examples:
 *
 *     sequential {
 *         q = d;
 *     }
 *
 *     sequential controller {
 *         state = next_state;
 *     }
 *
 * Event/sensitivity information is optional because a sequential construct may
 * also be associated with timing/process metadata by its enclosing HDL
 * composition or semantic model.
 */
hdlSequentialDeclaration
    : SEQUENTIAL
      identifier?
      hdlSequentialEventSpecification?
      hdlSequentialBody
    ;


/* ============================================================================
 * EVENT / SENSITIVITY SPECIFICATION
 * ============================================================================
 *
 * Event syntax is deliberately explicit rather than using `hdlKeyword`.
 *
 * This prevents arbitrary keywords from silently becoming legal clock/reset
 * syntax.
 *
 * Canonical source form:
 *
 *     sequential @(clock) {
 *         ...
 *     }
 *
 *     sequential @(posedge clock) {
 *         ...
 *     }
 *
 *     sequential @(negedge reset_n, posedge clock) {
 *         ...
 *     }
 *
 * The semantic layer determines whether a particular event combination is
 * legal for the selected sequential model.
 */
hdlSequentialEventSpecification
    : AT
      LPAREN
      hdlSequentialEventList
      RPAREN
    ;


hdlSequentialEventList
    : hdlSequentialEvent
      (
          COMMA
          hdlSequentialEvent
      )*
    ;


hdlSequentialEvent
    : hdlSequentialEdge?
      hdlSequentialEventSource
      hdlSequentialEventQualifier*
    ;


/*
 * Edge vocabulary is deliberately small and semantic.
 *
 * It describes an event relationship rather than a particular implementation.
 */
hdlSequentialEdge
    : POSEDGE
    | NEGEDGE
    ;


/*
 * The event source is an HDL expression so symbolic and parameterized
 * references remain possible.
 *
 * Semantic analysis must require an event-compatible expression.
 */
hdlSequentialEventSource
    : hdlExpression
    ;


/*
 * Existing language-level synchronization vocabulary can be used as an event
 * qualifier where the surrounding semantic model permits it.
 *
 * No arbitrary keyword is accepted here.
 */
hdlSequentialEventQualifier
    : ASYNC
    | SYNC
    ;


/* ============================================================================
 * BODY
 * ============================================================================
 */

hdlSequentialBody
    : LBRACE
      hdlSequentialStatement*
      RBRACE
    ;


hdlSequentialStatement
    : hdlSequentialAssignment
    | hdlSequentialConditional
    | hdlSequentialCase
    | hdlSequentialLocalDeclaration
    | hdlSequentialAssertion
    | hdlSequentialBlock
    ;


/* ============================================================================
 * SEQUENTIAL ASSIGNMENT
 * ============================================================================
 *
 * The assignment operator is deliberately the canonical ASSIGN token.
 *
 * Sequential semantics are determined by the containing sequential region.
 *
 * A future non-blocking/clocked assignment operator must first be added to
 * the canonical lexer/operator contract. It must NOT be invented locally in
 * this grammar.
 *
 * The semantic layer determines:
 *
 *     - current-state read;
 *     - next-state write;
 *     - update ordering;
 *     - multiple-driver legality;
 *     - write conflicts;
 *     - reset dominance;
 *     - enable behavior;
 *     - state inference;
 *     - deterministic transition semantics.
 */
hdlSequentialAssignment
    : hdlLValue
      ASSIGN
      hdlExpression
      SEMICOLON
    ;


/* ============================================================================
 * CONDITIONAL STATE TRANSITION
 * ============================================================================
 */

hdlSequentialConditional
    : IF
      LPAREN
      hdlExpression
      RPAREN
      hdlSequentialBody
      hdlSequentialElseClause?
    ;


hdlSequentialElseClause
    : ELSE
      (
          hdlSequentialConditional
        | hdlSequentialBody
      )
    ;


/* ============================================================================
 * CASE-BASED STATE TRANSITION
 * ============================================================================
 *
 * Case matching and coverage are semantic concerns.
 *
 * The grammar permits arbitrary numbers of case items.
 *
 * Semantic analysis must detect:
 *
 *     - duplicate/default conflicts;
 *     - overlapping patterns where prohibited;
 *     - unreachable branches;
 *     - incomplete coverage where required;
 *     - incompatible pattern types.
 */
hdlSequentialCase
    : CASE
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
    | DEFAULT
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
 * LOCAL SEQUENTIAL VALUES
 * ============================================================================
 *
 * Local declarations are restricted to immutable/explicitly initialized
 * values so the grammar does not silently introduce another state-storage
 * mechanism.
 *
 * Persistent state must be represented by the appropriate HDL state/register/
 * memory semantics elsewhere and then referenced through hdlLValue.
 *
 * Semantic analysis must still verify:
 *
 *     - initialization;
 *     - type compatibility;
 *     - lifetime;
 *     - use-before-definition;
 *     - absence of unintended storage.
 */
hdlSequentialLocalDeclaration
    : (LET | CONST)
      identifier
      (
          COLON
          hdlTypeExpression
      )?
      ASSIGN
      hdlExpression
      SEMICOLON
    ;


/* ============================================================================
 * ASSERTION
 * ============================================================================
 *
 * This is the sequential statement form of an assertion.
 *
 * The dedicated HDL assertion grammar may own declaration/property forms;
 * this rule only owns the procedural sequential statement boundary.
 */
hdlSequentialAssertion
    : ASSERT
      LPAREN
      hdlExpression
      RPAREN
      SEMICOLON
    ;


/* ============================================================================
 * NESTED BLOCK
 * ============================================================================
 */

hdlSequentialBlock
    : LBRACE
      hdlSequentialStatement*
      RBRACE
    ;


/*
 * ============================================================================
 * SEMANTIC INVARIANTS
 * ============================================================================
 *
 * These are NOT parser actions. They are mandatory downstream semantic
 * contracts for every successfully parsed sequential construct.
 *
 * 1. EVENT VALIDITY
 *
 *    An event specification must resolve to valid event sources.
 *
 * 2. CLOCK VALIDITY
 *
 *    A clock event must resolve to a declared/valid clock or event-capable
 *    semantic source.
 *
 * 3. EDGE VALIDITY
 *
 *    POSEDGE/NEGEDGE may only apply where the semantic event model permits
 *    edge-triggered behavior.
 *
 * 4. RESET VALIDITY
 *
 *    Reset behavior is semantic. It must not be inferred from arbitrary
 *    identifiers solely because their names contain "reset".
 *
 * 5. STATE VALIDITY
 *
 *    State targets must be legal sequentially writable objects.
 *
 * 6. SINGLE-DRIVER VALIDITY
 *
 *    Conflicting sequential drivers must be diagnosed according to the HDL
 *    semantic model.
 *
 * 7. UPDATE VALIDITY
 *
 *    Sequential assignments must lower to deterministic state transitions.
 *
 * 8. READ/WRITE ANALYSIS
 *
 *    The compiler must distinguish current-state reads from next-state writes.
 *
 * 9. RESET/ENABLE DOMINANCE
 *
 *    Where reset and enable conditions coexist, their precedence must be
 *    determined by explicit semantic rules rather than source ordering alone.
 *
 * 10. COMBINATIONAL/SEQUENTIAL SEPARATION
 *
 *     This construct must not silently become a combinational region.
 *
 * 11. CLOCK-DOMAIN VALIDATION
 *
 *     Cross-clock state interactions require explicit downstream validation.
 *
 * 12. LATCH/STATE SEMANTICS
 *
 *     Missing assignments in a sequential region represent state retention
 *     only when the sequential semantic model explicitly permits that
 *     behavior. The parser does not infer this.
 *
 * 13. WIDTH/SHAPE VALIDATION
 *
 *     Assignment compatibility is checked semantically.
 *
 * 14. RESOURCE VALIDATION
 *
 *     Register/state/memory requirements are evaluated downstream against
 *     available resources and capabilities.
 *
 * 15. PORTABILITY
 *
 *     No semantic rule may depend on a particular FPGA, ASIC, CPU, GPU, QPU,
 *     vendor, board, package, or physical topology unless explicitly selected
 *     through downstream target realization.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST should represent these concepts generically:
 *
 *     SequentialRegion
 *     SequentialEvent
 *     SequentialAssignment
 *     SequentialConditional
 *     SequentialCase
 *     SequentialLocal
 *     SequentialAssertion
 *     SequentialBlock
 *
 * Source spans MUST be retained.
 *
 * Attributes attached by the enclosing HDL dispatcher MUST remain associated
 * with the SequentialRegion rather than being reparsed here.
 *
 * The AST must not contain:
 *
 *     PhysicalRegisterId
 *     FpgaRegisterId
 *     VendorFlipFlop
 *     PhysicalClockPin
 *     RoutingResourceId
 *
 * unless such target-specific objects are introduced later during target
 * lowering, outside the frontend AST.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * The sequential AST lowers into the repository's canonical HDL/hardware
 * semantic representation.
 *
 * Conceptually:
 *
 *     SequentialRegion
 *          |
 *          +--> event boundary
 *          +--> state read dependencies
 *          +--> next-state assignments
 *          +--> conditional selection
 *          +--> case selection
 *          +--> assertions
 *          |
 *          v
 *     Hardware/HDL IR
 *
 * No second "SequentialIR" is introduced by this grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * COMPILER / RUNTIME CONTRACT
 * ============================================================================
 *
 * Compiler responsibilities:
 *
 *     - name resolution;
 *     - type checking;
 *     - width/shape checking;
 *     - event validation;
 *     - clock-domain analysis;
 *     - reset analysis;
 *     - state-transition construction;
 *     - driver analysis;
 *     - dependency analysis;
 *     - optimization;
 *     - synthesis/lowering;
 *     - scheduling;
 *     - timing analysis;
 *     - placement/routing;
 *     - target realization.
 *
 * Runtime responsibilities are relevant only for simulation, emulation,
 * hardware control, or generated executable environments.
 *
 * This grammar itself has no runtime behavior.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden in this grammar:
 *
 *     MAX_REGISTERS
 *     MAX_BITS
 *     MAX_STATES
 *     MAX_CLOCKS
 *     MAX_EVENTS
 *     MAX_PROCESSES
 *     MAX_WIDTH
 *     MAX_MEMORY
 *     MAX_DEVICES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *
 * Also forbidden:
 *
 *     physical register numbers;
 *     physical clock IDs;
 *     vendor primitive names;
 *     FPGA-specific resource counts;
 *     ASIC cell names;
 *     fixed bus widths;
 *     fixed clock counts;
 *     fixed state counts.
 *
 * Numeric values remain legal when they are actual program semantics, such as:
 *
 *     counter = 1024;
 *     width = 4096;
 *
 * The prohibition is against making such values universal grammar/compiler
 * limits.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] sequential declaration has one canonical owner;
 * [x] canonical ZamaniLexer vocabulary is consumed;
 * [x] no K_* pseudo-token dependency exists;
 * [x] no hdlKeyword dependency exists;
 * [x] no duplicate attribute grammar exists;
 * [x] no duplicate expression grammar exists;
 * [x] no duplicate type grammar exists;
 * [x] no generate ownership exists here;
 * [x] no state-machine ownership exists here;
 * [x] no pipeline ownership exists here;
 * [x] no process ownership exists here;
 * [x] no physical implementation is encoded;
 * [x] no artificial capacity exists;
 * [x] event syntax is explicit;
 * [x] sequential assignment is explicit;
 * [x] conditional transitions are explicit;
 * [x] case transitions are explicit;
 * [x] local values are explicit;
 * [x] assertions are explicit;
 * [x] nested blocks are explicit;
 * [x] AST mapping is predetermined;
 * [x] IR mapping is predetermined;
 * [x] semantic obligations are predetermined;
 * [x] downstream ownership is predetermined;
 * [x] POCO-REAF constraints are preserved;
 * [x] Rust implementation remains outside this grammar and safe.
 *
 * ============================================================================
 */
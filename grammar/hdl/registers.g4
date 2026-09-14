/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hdl/registers.g4
 *
 * Purpose:
 *     Production HDL register/storage declaration grammar.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no embedded Rust code.
 *     No unsafe Rust is used or required.
 *
 * Language objective:
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     canonical/modular parser
 *          |
 *          +--> HDL module grammar
 *                  |
 *                  +--> ports
 *                  +--> signals
 *                  +--> wires
 *                  +--> registers <--- THIS FILE
 *                  +--> clocks
 *                  +--> timing
 *                  +--> combinational logic
 *                  +--> sequential logic
 *                  +--> processes
 *                  +--> state machines
 *                  +--> memories
 *                  +--> pipelines
 *                  |
 *                  v
 *              frontend AST
 *                  |
 *                  v
 *              semantic analysis
 *                  |
 *                  +--> name resolution
 *                  +--> type checking
 *                  +--> state/storage analysis
 *                  +--> clock-domain analysis
 *                  +--> reset analysis
 *                  +--> sequential validation
 *                  +--> resource analysis
 *                  |
 *                  v
 *              canonical HDL/hardware semantics
 *                  |
 *                  +--> optimization
 *                  +--> scheduling
 *                  +--> routing
 *                  +--> synthesis
 *                  +--> target lowering
 *                  |
 *                  v
 *              hardware/runtime realization
 *
 * Grammar establishes syntax.
 * Semantic analysis establishes meaning.
 * Sequential/timing analysis establishes temporal behavior.
 * Hardware compilation establishes physical realization.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - HDL register declarations;
 *     - HDL register declaration lists;
 *     - logical register names;
 *     - logical register types;
 *     - register dimensions;
 *     - register declaration modifiers;
 *     - register declaration attributes;
 *     - optional source-level initialization syntax;
 *     - register references;
 *     - register indexing;
 *     - register slicing;
 *     - register member selection;
 *     - register-specific declaration metadata;
 *     - syntax-level storage identity for HDL registers.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - general identifiers;
 *     - general expressions;
 *     - general types;
 *     - ports;
 *     - signals;
 *     - wires;
 *     - clocks;
 *     - timing;
 *     - sequential transition equations;
 *     - combinational logic;
 *     - processes;
 *     - state machines;
 *     - memories;
 *     - pipelines;
 *     - module declarations;
 *     - hardware interfaces;
 *     - physical registers;
 *     - CPU register allocation;
 *     - FPGA flip-flop/LUT allocation;
 *     - ASIC cell selection;
 *     - placement;
 *     - routing;
 *     - scheduling;
 *     - synthesis;
 *     - target selection;
 *     - calibration;
 *     - hardware discovery;
 *     - runtime dispatch;
 *     - resource allocation;
 *     - quantum registers;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN.
 *
 * ============================================================================
 * REGISTER SEMANTIC MODEL
 * ============================================================================
 *
 * An HDL register is a LOGICAL STATE-HOLDING object.
 *
 * It represents storage whose value may persist across semantic evaluation
 * boundaries according to the sequential/timing model.
 *
 * A register does NOT inherently mean:
 *
 *     - one CPU architectural register;
 *     - one physical flip-flop;
 *     - one FPGA flip-flop;
 *     - one ASIC storage cell;
 *     - one physical latch;
 *     - one memory bit;
 *     - one device address;
 *     - one physical hardware resource;
 *     - one fixed-width machine register.
 *
 * The compiler may realize a logical register as:
 *
 *     - one or more flip-flops;
 *     - latches;
 *     - distributed storage;
 *     - block memory;
 *     - CPU registers;
 *     - memory;
 *     - optimized state representation;
 *     - replicated state;
 *     - another target-specific storage mechanism.
 *
 * The realization must preserve the semantic contract.
 *
 * ============================================================================
 * SIGNAL / WIRE / REGISTER DISTINCTION
 * ============================================================================
 *
 * SIGNAL:
 *
 *     A logical value-bearing HDL object.
 *
 * WIRE:
 *
 *     Logical connectivity between compatible endpoints.
 *
 * REGISTER:
 *
 *     Logical persistent state/storage.
 *
 * Therefore:
 *
 *     signals.g4
 *         owns signal declarations.
 *
 *     wires.g4
 *         owns logical connectivity.
 *
 *     registers.g4
 *         owns logical state/storage declarations.
 *
 *     sequential.g4
 *         owns sequential transition behavior.
 *
 *     clocks.g4
 *         owns clock declarations and clock semantics.
 *
 *     timing.g4
 *         owns timing constraints and temporal relationships.
 *
 * A register MUST NOT become a second signal or wire abstraction.
 *
 * ============================================================================
 * QUANTUM REGISTER DISTINCTION
 * ============================================================================
 *
 * This file owns HDL/classical hardware registers only.
 *
 * It does NOT own:
 *
 *     Qubit registers
 *     quantum registers
 *     quantum states
 *     quantum operations
 *     quantum measurement
 *
 * Those concepts belong to the quantum grammar and ultimately lower through
 * the canonical quantum semantic boundary, quantum::ir.
 *
 * A source spelling such as:
 *
 *     register
 *
 * inside an HDL context therefore has different semantic ownership from:
 *
 *     quantum register
 *
 * in a quantum context.
 *
 * ============================================================================
 * POCO-REAF PRINCIPLE
 * ============================================================================
 *
 * A register describes logical state, not physical implementation.
 *
 * This grammar therefore deliberately contains NO:
 *
 *     MAX_REGISTERS
 *     MAX_WIDTH
 *     MAX_BITS
 *     MAX_STATE_ELEMENTS
 *     MAX_DIMENSIONS
 *     MAX_BANKS
 *     MAX_PHYSICAL_REGISTERS
 *     MAX_FLIP_FLOPS
 *     MAX_LANES
 *
 * It also contains NO:
 *
 *     CPU register names;
 *     FPGA identifiers;
 *     ASIC cell identifiers;
 *     device IDs;
 *     physical addresses;
 *     package pins;
 *     routing coordinates;
 *     vendor-specific storage resources;
 *     fixed hardware topology.
 *
 * Any actual limitation belongs to:
 *
 *     parser resource policy;
 *     semantic validation;
 *     compiler resource policy;
 *     target capability analysis;
 *     synthesis;
 *     placement;
 *     routing;
 *     deployment;
 *     runtime resources.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * Register collections use repetition rather than fixed cardinality.
 *
 * A program may therefore describe:
 *
 *     one register;
 *     many registers;
 *     parameterized register arrays;
 *     generated register collections;
 *     arbitrarily large logical state structures.
 *
 * The grammar itself imposes no machine-size ceiling.
 *
 * Widths and dimensions are represented through the shared type/expression
 * system rather than fixed numeric ranges.
 *
 * ============================================================================
 * CANONICAL LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexer is authoritative.
 *
 * This grammar consumes:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * through:
 *
 *     options {
 *         tokenVocab = ZamaniLexer;
 *     }
 *
 * This file MUST NOT define lexer rules.
 *
 * The current repository architecture contains both reserved lexical words
 * and contextual HDL vocabulary. Register syntax therefore uses a contextual
 * register keyword adapter rather than silently creating another lexer.
 *
 * If the language specification later promotes `register` to a permanently
 * reserved keyword, the canonical lexer may provide a dedicated REGISTER token.
 *
 * Such promotion belongs to the canonical lexer/specification layer, not here.
 *
 * ============================================================================
 * SHARED PARSER CONTRACT
 * ============================================================================
 *
 * The composed canonical parser supplies shared rules including:
 *
 *     identifier
 *     expression
 *     typeExpr
 *     qualifiedName
 *
 * This grammar consumes those rules rather than creating another general
 * expression or type system.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Parsing MUST provide source-location-aware syntax sufficient for the frontend
 * AST to represent at least:
 *
 *     RegisterDeclaration
 *     RegisterDeclarator
 *     RegisterType
 *     RegisterDimension
 *     RegisterInitializer
 *     RegisterAttribute
 *     RegisterReference
 *     RegisterSelection
 *
 * The grammar itself MUST NOT construct AST objects.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for determining:
 *
 *     - whether the register name is declared;
 *     - whether a register shadows another declaration illegally;
 *     - whether its type is valid;
 *     - whether dimensions are valid;
 *     - whether dimensions are compile-time evaluable where required;
 *     - whether initialization is legal;
 *     - whether the register participates in sequential logic;
 *     - which clock/domain governs its state transitions;
 *     - whether reset behavior is valid;
 *     - whether enable behavior is valid;
 *     - whether state transitions are deterministic where required;
 *     - whether multiple assignments/drivers are legal;
 *     - whether a state update is compatible with the timing model;
 *     - whether the resulting logical state is realizable by the target.
 *
 * None of those decisions are encoded as physical assumptions in this grammar.
 *
 * ============================================================================
 * CLOCK / RESET BOUNDARY
 * ============================================================================
 *
 * Registers may participate in clocked and resettable behavior.
 *
 * However:
 *
 *     clocks.g4
 *
 * owns clock declarations.
 *
 *     timing.g4
 *
 * owns timing relationships.
 *
 *     sequential.g4
 *
 * owns sequential state-transition semantics.
 *
 *     processes.g4
 *
 * owns procedural/process semantics.
 *
 * Therefore this file does NOT define a second clock language or reset language.
 *
 * Register attributes may carry source-level metadata when the language
 * specification explicitly defines such attributes, but semantic clock/reset
 * interpretation remains outside this grammar.
 *
 * ============================================================================
 * INITIALIZATION BOUNDARY
 * ============================================================================
 *
 * A register initializer is a SOURCE-LEVEL INITIAL VALUE.
 *
 * It does NOT automatically imply:
 *
 *     FPGA configuration initialization;
 *     ASIC power-up state;
 *     physical reset state;
 *     hardware memory initialization;
 *     runtime startup behavior.
 *
 * The semantic and target-lowering layers decide whether and how initialization
 * can be realized.
 *
 * ============================================================================
 * NO PHYSICAL REGISTER ALLOCATION
 * ============================================================================
 *
 * The following are explicitly forbidden as register grammar semantics:
 *
 *     register = CPU_RAX
 *     register = FPGA_FF_123
 *     register = ASIC_CELL_42
 *     address = 0x...
 *     bank = 3
 *     device = "..."
 *     placement = "..."
 *
 * Such information belongs to hardware targets, resource constraints,
 * placement, deployment, or target-specific dialects.
 *
 * ============================================================================
 */

parser grammar registers;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. PUBLIC REGISTER DECLARATION
 * ============================================================================
 *
 * Canonical conceptual forms:
 *
 *     register state: logic;
 *
 *     register counter: uint;
 *
 *     register state: Vector<logic, WIDTH>;
 *
 *     register state: logic[WIDTH];
 *
 *     register state: logic = INITIAL_VALUE;
 *
 * The exact type and expression syntax is supplied by the shared grammar.
 */
hdlRegisterDeclaration
    : hdlRegisterAttributes?
      hdlRegisterModifiers*
      hdlRegisterKeyword
      hdlRegisterDeclaratorList
      hdlRegisterTerminator
    ;


/*
 * ============================================================================
 * 2. DECLARATION LIST
 * ============================================================================
 *
 * No fixed number of registers is permitted.
 */
hdlRegisterDeclaratorList
    : hdlRegisterDeclarator
      (
          COMMA
          hdlRegisterDeclarator
      )*
      COMMA?
    ;


hdlRegisterDeclarator
    : hdlRegisterName
      hdlRegisterType
      hdlRegisterDimensions*
      hdlRegisterInitializer?
      hdlRegisterDeclarationMetadata*
    ;


/*
 * ============================================================================
 * 3. DECLARATION TERMINATOR
 * ============================================================================
 *
 * The normal HDL declaration boundary is a semicolon.
 */
hdlRegisterTerminator
    : SEMICOLON
    ;


/*
 * ============================================================================
 * 4. CONTEXTUAL REGISTER KEYWORD
 * ============================================================================
 *
 * `register` is treated as contextual HDL vocabulary unless the canonical
 * lexer promotes it to a reserved token.
 *
 * Semantic validation MUST verify the canonical spelling.
 */
hdlRegisterKeyword
    : identifier
    ;


/*
 * ============================================================================
 * 5. REGISTER MODIFIERS
 * ============================================================================
 *
 * Modifiers describe source-level storage properties.
 *
 * They must not select physical hardware.
 */
hdlRegisterModifiers
    : hdlRegisterModifier
    ;


hdlRegisterModifier
    : MUT
    | CONST
    | VOLATILE
    | INTERNAL
    | hdlRegisterContextualModifier
    ;


hdlRegisterContextualModifier
    : identifier
    ;


/*
 * ============================================================================
 * 6. REGISTER NAME
 * ============================================================================
 *
 * A register name is a logical source symbol.
 */
hdlRegisterName
    : identifier
    ;


/*
 * ============================================================================
 * 7. REGISTER TYPE
 * ============================================================================
 *
 * Register types are delegated to the shared Zamani type system.
 *
 * The grammar imposes no width or representation limit.
 */
hdlRegisterType
    : COLON
      typeExpr
    ;


/*
 * ============================================================================
 * 8. REGISTER DIMENSIONS
 * ============================================================================
 *
 * Dimensions describe logical storage shape.
 *
 * Examples:
 *
 *     register state: logic[WIDTH];
 *
 *     register matrix: logic[ROWS][COLS];
 *
 * WIDTH, ROWS and COLS may be generic parameters, compile-time expressions,
 * symbolic values, or other semantically valid dimension expressions.
 *
 * No fixed dimension count is imposed.
 */
hdlRegisterDimensions
    : LBRACKET
      hdlRegisterDimensionExpression?
      RBRACKET
    ;


hdlRegisterDimensionExpression
    : expression
    ;


/*
 * ============================================================================
 * 9. REGISTER INITIALIZER
 * ============================================================================
 *
 * Initial values are semantic values.
 *
 * They do not directly prescribe physical startup implementation.
 */
hdlRegisterInitializer
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * 10. REGISTER ATTRIBUTES
 * ============================================================================
 *
 * Attributes are metadata and source-level declarations.
 *
 * They must not silently become physical placement or routing directives.
 */
hdlRegisterAttributes
    : hdlRegisterAttribute+
    ;


hdlRegisterAttribute
    : AT
      identifier
      (
          LPAREN
          hdlRegisterArgumentList?
          RPAREN
      )?
    ;


hdlRegisterArgumentList
    : hdlRegisterArgument
      (
          COMMA
          hdlRegisterArgument
      )*
      COMMA?
    ;


hdlRegisterArgument
    : identifier
      ASSIGN
      expression
    | expression
    ;


/*
 * ============================================================================
 * 11. REGISTER DECLARATION METADATA
 * ============================================================================
 *
 * Declaration metadata is intentionally separate from general register
 * modifiers so that future language evolution can distinguish storage
 * properties from annotations/contracts.
 *
 * Interpretation remains a semantic-layer responsibility.
 */
hdlRegisterDeclarationMetadata
    : hdlRegisterMetadata
    ;


hdlRegisterMetadata
    : AT
      identifier
      (
          LPAREN
          hdlRegisterArgumentList?
          RPAREN
      )?
    ;


/*
 * ============================================================================
 * 12. REGISTER REFERENCE
 * ============================================================================
 *
 * A reference identifies a logical HDL register.
 */
hdlRegisterReference
    : hdlRegisterQualifiedReference
    | hdlRegisterName
    ;


hdlRegisterQualifiedReference
    : identifier
      (
          DOUBLE_COLON
          identifier
      )+
    ;


/*
 * ============================================================================
 * 13. REGISTER INDEX
 * ============================================================================
 *
 * Indexing is logical selection.
 *
 * Semantic analysis determines:
 *
 *     - indexability;
 *     - bounds;
 *     - resulting type;
 *     - shape;
 *     - legality of writes.
 */
hdlRegisterIndex
    : LBRACKET
      expression
      RBRACKET
    ;


/*
 * ============================================================================
 * 14. REGISTER SLICE
 * ============================================================================
 *
 * A slice selects a logical range.
 *
 * No machine width is encoded.
 */
hdlRegisterSlice
    : LBRACKET
      hdlRegisterRangeExpression
      RBRACKET
    ;


hdlRegisterRangeExpression
    : expression
      hdlRegisterRangeOperator
      expression?
    ;


hdlRegisterRangeOperator
    : COLON
    | DOT_DOT
    | DOT_DOT_EQ
    ;


/*
 * ============================================================================
 * 15. REGISTER SELECTION
 * ============================================================================
 *
 * A register may be selected by index or slice.
 */
hdlRegisterSelection
    : hdlRegisterReference
      (
          hdlRegisterIndex
        | hdlRegisterSlice
      )+
    ;


/*
 * ============================================================================
 * 16. REGISTER MEMBER ACCESS
 * ============================================================================
 *
 * Structured register values may expose members through the common logical
 * member-access model.
 */
hdlRegisterMemberSelection
    : hdlRegisterReference
      (
          DOT
          identifier
      )+
    ;


/*
 * ============================================================================
 * 17. REGISTER ACCESS
 * ============================================================================
 */
hdlRegisterAccess
    : hdlRegisterSelection
    | hdlRegisterMemberSelection
    | hdlRegisterReference
    ;


/*
 * ============================================================================
 * 18. REGISTER DECLARATION GROUP
 * ============================================================================
 *
 * Allows a module grammar or HDL declaration container to consume multiple
 * register declarations without imposing a count.
 */
hdlRegisterDeclarations
    : hdlRegisterDeclaration*
    ;


/*
 * ============================================================================
 * 19. REGISTER BLOCK
 * ============================================================================
 *
 * This rule is deliberately a declaration container only.
 *
 * It does not define sequential behavior.
 */
hdlRegisterBlock
    : LBRACE
      hdlRegisterDeclaration*
      RBRACE
    ;
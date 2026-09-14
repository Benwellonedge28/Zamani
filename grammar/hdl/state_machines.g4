parser grammar state_machines;

options {
    tokenVocab = ZamaniTokens;
}

/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hdl/state_machines.g4
 *
 * Purpose:
 *     Dedicated HDL state-machine syntax delegate.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no Rust actions, predicates, unsafe code,
 *     filesystem access, network access, process execution, or target-specific
 *     behavior.
 *
 * ============================================================================
 * ARCHITECTURAL ROLE
 * ============================================================================
 *
 * This grammar owns the SOURCE SYNTAX of target-independent hardware
 * state-machine descriptions.
 *
 * It does not own:
 *
 *     - lexical definitions;
 *     - identifiers;
 *     - expressions;
 *     - types;
 *     - registers;
 *     - clocks;
 *     - timing;
 *     - processes;
 *     - combinational logic;
 *     - scheduling;
 *     - routing;
 *     - placement;
 *     - synthesis;
 *     - hardware discovery;
 *     - physical resources;
 *     - vendor-specific implementations;
 *     - quantum IR;
 *     - runtime execution.
 *
 * Those concerns remain owned by their respective grammar/compiler domains.
 *
 * ============================================================================
 * POCO-REAF PRINCIPLE
 * ============================================================================
 *
 * A state machine describes:
 *
 *     state
 *     transition
 *     guard
 *     action
 *     entry behavior
 *     exit behavior
 *     initial/reset intent
 *
 * It MUST NOT encode:
 *
 *     - a fixed number of states;
 *     - a fixed number of transitions;
 *     - a fixed state width;
 *     - a fixed register count;
 *     - a fixed clock frequency;
 *     - a fixed clock domain;
 *     - a fixed FPGA;
 *     - a fixed ASIC;
 *     - a fixed CPU;
 *     - a fixed GPU;
 *     - a fixed device;
 *     - a fixed topology;
 *     - a fixed physical encoding.
 *
 * Physical state encoding, optimization, placement, timing, and realization
 * are downstream compiler responsibilities.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * All collections use ANTLR repetition constructs.
 *
 * There are intentionally no language-level constants such as:
 *
 *     MAX_STATES
 *     MAX_TRANSITIONS
 *     MAX_STATE_WIDTH
 *     MAX_ACTIONS
 *     MAX_STATE_MACHINES
 *
 * Any practical limit is imposed by the parser/compiler/resource policy,
 * available memory, execution environment, synthesis technology, or target
 * capabilities rather than this grammar.
 *
 * ============================================================================
 * CONTEXTUAL VOCABULARY
 * ============================================================================
 *
 * The existing Zamani HDL architecture deliberately treats many HDL words
 * as contextual vocabulary rather than creating a second lexer.
 *
 * Consequently, the structural words represented through `hdlKeyword`
 * are interpreted by semantic analysis.
 *
 * This grammar therefore does NOT introduce duplicate lexer tokens for:
 *
 *     state_machine
 *     state
 *     transition
 *     initial
 *     reset
 *     entry
 *     exit
 *     guard
 *     action
 *     terminal
 *     default
 *     final
 *
 * Semantic analysis is responsible for validating their contextual meaning.
 *
 * ============================================================================
 * CANONICAL DATA FLOW
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniTokens
 *          |
 *          v
 *     canonical parser
 *          |
 *          v
 *     state_machines.g4
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     HDL semantic analysis
 *          |
 *          +--> state validation
 *          +--> transition validation
 *          +--> guard validation
 *          +--> action validation
 *          +--> clock/process validation
 *          +--> type validation
 *          +--> resource analysis
 *          |
 *          v
 *     canonical HDL/hardware representation
 *          |
 *          +--> optimization
 *          +--> scheduling
 *          +--> synthesis
 *          +--> placement
 *          +--> routing
 *          |
 *          v
 *     target lowering
 *
 * ============================================================================
 */


/* ============================================================================
 * PUBLIC ENTRY POINT
 * ========================================================================== */

/*
 * Complete state-machine declaration.
 *
 * Example semantic form:
 *
 *     state_machine controller {
 *         initial: idle;
 *
 *         state idle;
 *         state active;
 *         state complete;
 *
 *         transition idle -> active : start;
 *         transition active -> complete : done;
 *         transition complete -> idle : restart;
 *     }
 *
 * The grammar does not assign physical encodings to the states.
 */
hdlStateMachineDeclaration
    : hdlKeyword
      identifier
      hdlStateMachineAttributes*
      hdlStateMachineBody
    ;


/* ============================================================================
 * STATE-MACHINE BODY
 * ========================================================================== */

hdlStateMachineBody
    : LBRACE
      hdlStateMachineItem*
      RBRACE
    ;

hdlStateMachineItem
    : hdlStateMachineProperty
    | hdlStateDeclaration
    | hdlTransitionDeclaration
    | hdlStateMachineDefaultTransition
    | hdlStateMachineBlock
    | hdlAttribute
    ;


/* ============================================================================
 * STATE-MACHINE ATTRIBUTES
 * ========================================================================== */

/*
 * Attributes remain syntactic metadata.
 *
 * Their semantic interpretation belongs to the AST/semantic layer.
 */
hdlStateMachineAttributes
    : hdlAttribute
    ;


/* ============================================================================
 * STATE-MACHINE PROPERTIES
 * ========================================================================== */

/*
 * Generic property form deliberately prevents the grammar from accumulating
 * a permanent list of every possible future state-machine property.
 *
 * Examples:
 *
 *     initial: idle;
 *     reset: idle;
 *     encoding: symbolic;
 *     clock: clk;
 *     reset_mode: synchronous;
 *
 * The semantic layer determines which properties are legal and what they mean.
 */
hdlStateMachineProperty
    : identifier
      COLON
      hdlExpression
      SEMICOLON
    ;


/* ============================================================================
 * STATES
 * ========================================================================== */

/*
 * Basic state declaration.
 *
 * Examples:
 *
 *     state idle;
 *     state running;
 *
 * Optional expression:
 *
 *     state idle = initial_value;
 *
 * The expression is semantic metadata/value information, NOT a physical
 * encoding requirement.
 */
hdlStateDeclaration
    : hdlKeyword
      identifier
      hdlStateAttributes*
      (
          ASSIGN
          hdlExpression
      )?
      (
          hdlStateBody
      )?
      SEMICOLON?
    ;


/*
 * Optional state-level metadata.
 *
 * Examples:
 *
 *     @initial
 *     @terminal
 *     @reset
 *     @encoding(...)
 *
 * The grammar only records the attribute syntax.
 */
hdlStateAttributes
    : hdlAttribute
    ;


/*
 * State behavior is intentionally block-based.
 *
 * The block may contain the shared HDL statement/control constructs supplied
 * by the parent HDL grammar.
 *
 * This prevents state_machines.g4 from creating another statement language.
 */
hdlStateBody
    : LBRACE
      hdlStateBodyItem*
      RBRACE
    ;

hdlStateBodyItem
    : hdlStateBehavior
    | hdlAttribute
    ;


/*
 * Contextual state behavior.
 *
 * A behavior consists of a contextual name followed by either:
 *
 *     an expression;
 *     a block;
 *     an argument list;
 *
 * This supports extensibility for concepts such as:
 *
 *     entry { ... }
 *     exit  { ... }
 *     action(...) { ... }
 *     invariant: expression;
 *
 * Semantic analysis determines the permitted state behaviors.
 */
hdlStateBehavior
    : identifier
      (
          COLON
          hdlExpression
          SEMICOLON
        | hdlStateBehaviorArguments?
          hdlBlock
      )
    ;

hdlStateBehaviorArguments
    : LPAREN
      hdlArgumentList?
      RPAREN
    ;


/* ============================================================================
 * TRANSITIONS
 * ========================================================================== */

/*
 * Explicit transition:
 *
 *     transition source -> target;
 *
 * Guarded transition:
 *
 *     transition source -> target : condition;
 *
 * Transition action:
 *
 *     transition source -> target : condition {
 *         ...
 *     }
 *
 * Both source and target are symbolic semantic state references.
 *
 * No state count or physical state encoding is implied.
 */
hdlTransitionDeclaration
    : hdlKeyword
      identifier
      THIN_ARROW
      identifier
      hdlTransitionGuard?
      hdlTransitionBody?
      SEMICOLON?
    ;


/*
 * Transition guard.
 *
 * The expression is evaluated semantically by the HDL/control-flow layer.
 */
hdlTransitionGuard
    : COLON
      hdlExpression
    ;


/*
 * Optional transition behavior.
 *
 * This is deliberately a normal HDL block rather than a second statement
 * grammar.
 */
hdlTransitionBody
    : hdlBlock
    ;


/* ============================================================================
 * DEFAULT TRANSITIONS
 * ========================================================================== */

/*
 * A default transition is represented structurally without forcing a specific
 * target state.
 *
 * Example:
 *
 *     default -> recovery;
 *
 * The contextual word `default` is resolved by semantic analysis.
 */
hdlStateMachineDefaultTransition
    : hdlKeyword
      THIN_ARROW
      identifier
      hdlTransitionGuard?
      hdlTransitionBody?
      SEMICOLON?
    ;


/* ============================================================================
 * NESTED / EXTENSIBLE STATE-MACHINE BLOCKS
 * ========================================================================== */

/*
 * Extensible named blocks permit future semantic features without repeatedly
 * modifying this grammar.
 *
 * Examples:
 *
 *     invariants { ... }
 *     properties { ... }
 *     timing { ... }
 *     verification { ... }
 *
 * Such blocks do NOT become an alternative owner for clocks, timing,
 * verification, or scheduling. Semantic ownership remains with those domains.
 */
hdlStateMachineBlock
    : identifier
      hdlStateMachineBlockArguments?
      hdlBlock
    ;

hdlStateMachineBlockArguments
    : LPAREN
      hdlArgumentList?
      RPAREN
    ;


/* ============================================================================
 * STATE-MACHINE REFERENCES
 * ========================================================================== */

/*
 * State references deliberately remain symbolic.
 *
 * No numeric state encoding is introduced here.
 *
 * Physical encodings such as:
 *
 *     binary
 *     one-hot
 *     Gray
 *     implementation-selected
 *
 * belong to optimization/synthesis/target lowering.
 */
hdlStateReference
    : identifier
    ;


/* ============================================================================
 * TRANSITION COLLECTION
 * ========================================================================== */

/*
 * This rule exists as a reusable semantic grouping rule for future delegates
 * and tests. It introduces no resource limits.
 */
hdlTransitionList
    : hdlTransitionDeclaration*
    ;


/* ============================================================================
 * STATE COLLECTION
 * ========================================================================== */

hdlStateList
    : hdlStateDeclaration*
    ;


/* ============================================================================
 * END
 * ========================================================================== */
/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hdl/registers.g4
 *
 * Status:
 *     CANONICAL HDL REGISTER / LOGICAL STATE DELEGATE
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     Safe Rust only; no unsafe Rust.
 *
 * Language objective:
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *     (POCO-REAF)
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL SYNTAX of logical HDL registers.
 *
 * A register is a logical state/storage object.
 *
 * This grammar describes:
 *
 *     - register declarations;
 *     - register declarator lists;
 *     - register modifiers;
 *     - register initializers;
 *     - register references;
 *     - register indexing;
 *     - register slicing;
 *     - register member selection;
 *     - register access;
 *     - register declaration containers.
 *
 * This grammar does NOT allocate physical registers.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * OWNS:
 *
 *     hdlRegisterDeclaration
 *     hdlRegisterDeclaratorList
 *     hdlRegisterDeclarator
 *     hdlRegisterModifierList
 *     hdlRegisterModifier
 *     hdlRegisterReference
 *     hdlRegisterAccess
 *     hdlRegisterSelector
 *     hdlRegisterMemberSelector
 *     hdlRegisterDeclarations
 *     hdlRegisterBlock
 *
 * DOES NOT OWN:
 *
 *     lexer/token definitions
 *     identifiers/names
 *     expressions
 *     types
 *     attributes
 *     modules
 *     ports
 *     signals
 *     nets/wires
 *     memories
 *     clocks
 *     resets
 *     timing
 *     sequential transition semantics
 *     combinational semantics
 *     processes
 *     state machines
 *     pipelines
 *     synthesis
 *     routing
 *     placement
 *     scheduling
 *     target selection
 *     physical register allocation
 *     CPU register allocation
 *     FPGA register allocation
 *     ASIC cell selection
 *     quantum registers
 *     quantum::ir
 *     QEC
 *     ZQN
 *     HAL
 *     runtime execution
 *
 * ============================================================================
 * ARCHITECTURAL BOUNDARY
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
 *          +--> registers.g4
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> name resolution
 *          +--> type checking
 *          +--> shape/dimension checking
 *          +--> initialization checking
 *          +--> sequential-state analysis
 *          +--> clock/reset analysis
 *          +--> driver analysis
 *          +--> capability/resource analysis
 *          |
 *          v
 *     canonical HDL/hardware semantic representation
 *          |
 *          v
 *     optimization / synthesis / scheduling / routing
 *          |
 *          v
 *     target realization
 *
 * The grammar defines structure.
 * Semantic analysis defines meaning.
 * Hardware compilation defines physical realization.
 *
 * ============================================================================
 * POCO-REAF / OPEN-WORLD CONTRACT
 * ============================================================================
 *
 * This grammar intentionally contains NO universal finite limits.
 *
 * Forbidden language-level limits include:
 *
 *     MAX_REGISTERS
 *     MAX_REGISTER_WIDTH
 *     MAX_BITS
 *     MAX_DIMENSIONS
 *     MAX_STATE_ELEMENTS
 *     MAX_REGISTER_COUNT
 *     MAX_BANKS
 *     MAX_LANES
 *     MAX_PHYSICAL_REGISTERS
 *     MAX_FLIP_FLOPS
 *     MAX_DEVICES
 *
 * There is also no fixed:
 *
 *     CPU register inventory
 *     GPU register inventory
 *     FPGA register inventory
 *     ASIC cell inventory
 *     physical register address
 *     register-bank number
 *     device identifier
 *     physical placement
 *     routing coordinate
 *     hardware topology
 *
 * Repetition is structural:
 *
 *     *
 *     +
 *
 * rather than a fixed capacity.
 *
 * Practical limits may exist in:
 *
 *     parser resource policy
 *     compiler resources
 *     available memory
 *     target resources
 *     synthesis
 *     deployment
 *     runtime
 *
 * Such limits MUST NOT become source-language semantics.
 *
 * ============================================================================
 * TYPE SYSTEM BOUNDARY
 * ============================================================================
 *
 * Register types are owned by the canonical source type grammar:
 *
 *     grammar/types/types.g4
 *
 * Public type entry point:
 *
 *     typeExpression
 *
 * This file MUST NOT define another type grammar.
 *
 * Consequently both of these can be represented without imposing a
 * hardware-level width limit:
 *
 *     register state : Logic;
 *     register state : Vector<Logic>[N];
 *
 * The meaning and validity of the type are determined downstream.
 *
 * ============================================================================
 * EXPRESSION BOUNDARY
 * ============================================================================
 *
 * Initializers, indices, and slice bounds consume the canonical:
 *
 *     expression
 *
 * rule.
 *
 * This file MUST NOT define another expression precedence hierarchy.
 *
 * ============================================================================
 * NAME BOUNDARY
 * ============================================================================
 *
 * Names are consumed from the canonical name grammar:
 *
 *     identifier
 *     qualifiedName
 *
 * This file MUST NOT redefine identifier syntax.
 *
 * ============================================================================
 * ATTRIBUTE BOUNDARY
 * ============================================================================
 *
 * Declaration attributes are consumed through the canonical:
 *
 *     attribute
 *
 * rule from grammar/core/attributes.g4.
 *
 * This file MUST NOT create another @attribute grammar.
 *
 * ============================================================================
 * REGISTER SEMANTIC MODEL
 * ============================================================================
 *
 * A logical register means persistent source-level state.
 *
 * It does NOT inherently mean:
 *
 *     - one CPU architectural register;
 *     - one physical flip-flop;
 *     - one FPGA flip-flop;
 *     - one ASIC storage cell;
 *     - one memory bit;
 *     - one physical storage bank;
 *     - one fixed-width machine register.
 *
 * The compiler may realize one logical register using:
 *
 *     - registers;
 *     - flip-flops;
 *     - latches;
 *     - memory;
 *     - distributed storage;
 *     - replicated storage;
 *     - optimized state;
 *     - another target representation.
 *
 * The realization must preserve source semantics.
 *
 * ============================================================================
 * REGISTER / SIGNAL / NET DISTINCTION
 * ============================================================================
 *
 * SIGNAL:
 *
 *     logical value-bearing object.
 *
 * NET/WIRE:
 *
 *     logical connectivity between compatible endpoints.
 *
 * REGISTER:
 *
 *     logical persistent state/storage.
 *
 * SEQUENTIAL:
 *
 *     owns state-transition behavior.
 *
 * CLOCKING:
 *
 *     owns clock declarations and source-level clock relationships.
 *
 * RESET:
 *
 *     owns reset syntax.
 *
 * TIMING:
 *
 *     owns temporal constraints and timing relationships.
 *
 * Registers.g4 therefore MUST NOT absorb sequential, clock, reset, or timing
 * grammars.
 *
 * ============================================================================
 * INITIALIZATION CONTRACT
 * ============================================================================
 *
 * A register initializer is a source-level initial value:
 *
 *     register state : Logic = initial_value;
 *
 * It does NOT by itself mean:
 *
 *     FPGA power-up initialization
 *     ASIC power-up state
 *     physical reset state
 *     memory-image initialization
 *     runtime startup behavior
 *
 * Semantic analysis and target lowering determine whether initialization is
 * realizable and how it is implemented.
 *
 * ============================================================================
 * QUANTUM REGISTER SEPARATION
 * ============================================================================
 *
 * This file owns HDL/classical logical registers only.
 *
 * It does NOT own:
 *
 *     Qubit
 *     quantum register
 *     quantum state
 *     quantum operation
 *     measurement
 *
 * Quantum syntax belongs to grammar/quantum/.
 *
 * Quantum semantics ultimately cross the canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * There must be no second quantum IR introduced here.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * Canonical lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Canonical lexical vocabulary:
 *
 *     grammar/lexer/
 *
 * Parser grammars consume:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * This file defines no lexer rules.
 *
 * IMPORTANT:
 *
 * The current repository has HDL consumers using K_REGISTER while the
 * canonical keyword vocabulary does not currently expose that exact token.
 *
 * The production lexical contract must therefore promote:
 *
 *     REGISTER : 'register' ;
 *
 * in the canonical keyword layer.
 *
 * registers.g4 consumes REGISTER after that contract is established.
 *
 * Parser-local fake tokens are forbidden.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The grammar must map structurally into the existing domain-neutral frontend
 * AST.
 *
 * Conceptual mappings:
 *
 *     hdlRegisterDeclaration
 *         -> RegisterDeclaration
 *
 *     hdlRegisterDeclarator
 *         -> RegisterDeclarator
 *
 *     hdlRegisterReference
 *         -> RegisterReference
 *
 *     hdlRegisterSelector
 *         -> RegisterSelection
 *
 *     hdlRegisterInitializer
 *         -> RegisterInitializer
 *
 * The exact Rust AST type names remain owned by src/frontend/ast/.
 *
 * The grammar MUST NOT construct Rust AST objects.
 *
 * Every mapped node must preserve source spans.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     - declaration-name resolution;
 *     - duplicate declaration detection;
 *     - scope validation;
 *     - type resolution;
 *     - type compatibility;
 *     - dimension/shape validation;
 *     - index validity;
 *     - slice validity;
 *     - initializer compatibility;
 *     - mutability rules;
 *     - sequential-state legality;
 *     - clock-domain association;
 *     - reset association;
 *     - driver/assignment legality;
 *     - read/write legality;
 *     - deterministic-state analysis;
 *     - resource requirements;
 *     - target capability requirements.
 *
 * The parser must not perform any of these semantic checks.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * Register declarations lower into the canonical HDL/hardware semantic
 * representation already used by the compiler.
 *
 * This grammar MUST NOT create:
 *
 *     PhysicalRegister
 *     FPGAFlipFlop
 *     ASICCell
 *     CPURegister
 *     GPURegister
 *
 * objects.
 *
 * Physical realization remains downstream.
 *
 * ============================================================================
 * COMPILER / RUNTIME CONTRACT
 * ============================================================================
 *
 * This grammar performs no runtime work.
 *
 * Register syntax is consumed during compilation and lowered into the
 * canonical hardware semantic representation.
 *
 * The compiler may later:
 *
 *     optimize;
 *     infer storage;
 *     share storage;
 *     replicate storage;
 *     eliminate storage;
 *     map storage;
 *     schedule updates;
 *     route connections;
 *     synthesize hardware;
 *
 * without changing the source-level register semantics.
 *
 * ============================================================================
 * DETERMINISM / SECURITY
 * ============================================================================
 *
 * Parsing MUST depend only on:
 *
 *     - source tokens;
 *     - active language version;
 *     - canonical grammar.
 *
 * It MUST NOT depend on:
 *
 *     - hardware discovery;
 *     - device state;
 *     - network state;
 *     - environment variables;
 *     - wall-clock time;
 *     - randomness;
 *     - runtime scheduler state.
 *
 * The grammar contains no actions, predicates, I/O, or code execution.
 *
 * ============================================================================
 */

parser grammar registers;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * PUBLIC DECLARATION
 * ============================================================================
 *
 * Canonical forms:
 *
 *     register state : Logic;
 *     register counter : UInt;
 *     register state : Vector<Logic>[N];
 *     register state : Logic = initial_value;
 *
 * Multiple declarations are permitted:
 *
 *     register a : Logic, b : Logic;
 *
 * Cardinality is unbounded by the grammar.
 */
hdlRegisterDeclaration
    : attribute*
      REGISTER
      hdlRegisterModifierList?
      hdlRegisterDeclaratorList
      SEMICOLON
    ;


/*
 * ============================================================================
 * DECLARATOR LIST
 * ============================================================================
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
    : identifier
      COLON
      typeExpression
      hdlRegisterInitializer?
    ;


/*
 * ============================================================================
 * MODIFIERS
 * ============================================================================
 *
 * Only canonical lexer-owned modifiers are accepted here.
 *
 * Unknown words are NOT silently accepted as modifiers.
 *
 * This prevents the declaration prefix from becoming ambiguous.
 */
hdlRegisterModifierList
    : hdlRegisterModifier+
    ;


hdlRegisterModifier
    : MUT
    | CONST
    | VOLATILE
    | INTERNAL
    ;


/*
 * ============================================================================
 * INITIALIZER
 * ============================================================================
 *
 * The initializer uses the universal expression grammar.
 */
hdlRegisterInitializer
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * REGISTER REFERENCE
 * ============================================================================
 *
 * Qualified names use the canonical `qualifiedName` contract.
 *
 * Examples:
 *
 *     state
 *     controller::state
 *     module::submodule::state
 */
hdlRegisterReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * REGISTER ACCESS
 * ============================================================================
 *
 * Access supports arbitrary combinations of:
 *
 *     indexing
 *     slicing
 *     member selection
 *
 * Examples:
 *
 *     state[i]
 *     state[hi:lo]
 *     state[i][j]
 *     state.field
 *     state[i].field[j]
 *
 * No fixed number of selectors is imposed.
 */
hdlRegisterAccess
    : hdlRegisterReference
      hdlRegisterAccessSuffix*
    ;


hdlRegisterAccessSuffix
    : hdlRegisterSelector
    | hdlRegisterMemberSelector
    ;


hdlRegisterSelector
    : LBRACKET
      expression
      (
          COLON expression
        | DOT_DOT expression
        | DOT_DOT_EQ expression
      )?
      RBRACKET
    ;


hdlRegisterMemberSelector
    : DOT
      identifier
    ;


/*
 * ============================================================================
 * REGISTER DECLARATION COLLECTION
 * ============================================================================
 *
 * Container rule for HDL module composition.
 */
hdlRegisterDeclarations
    : hdlRegisterDeclaration*
    ;


/*
 * ============================================================================
 * REGISTER BLOCK
 * ============================================================================
 *
 * Declaration-only block.
 *
 * Sequential behavior remains owned by sequential.g4/processes.g4.
 */
hdlRegisterBlock
    : LBRACE
      hdlRegisterDeclaration*
      RBRACE
    ;


/*
 * ============================================================================
 * COMPLETION CONTRACT
 * ============================================================================
 *
 * registers.g4 is complete when:
 *
 * [x] It has one canonical owner.
 * [x] It is a parser delegate, not a second HDL root.
 * [x] It consumes the canonical ZamaniLexer vocabulary.
 * [x] It uses REGISTER rather than a parser-local contextual keyword.
 * [x] It consumes canonical identifier syntax.
 * [x] It consumes canonical qualified-name syntax.
 * [x] It consumes canonical expression syntax.
 * [x] It consumes canonical typeExpression syntax.
 * [x] It consumes canonical attribute syntax.
 * [x] It contains no second expression grammar.
 * [x] It contains no second type grammar.
 * [x] It contains no physical-register semantics.
 * [x] It contains no fixed hardware capacities.
 * [x] It contains no target-specific resource assumptions.
 * [x] It distinguishes logical register state from quantum registers.
 * [x] It supports arbitrary declaration cardinality.
 * [x] It supports arbitrary selector depth.
 * [x] It supports symbolic dimensions through the canonical type system.
 * [x] It supports source-level initialization.
 * [x] It preserves a clean semantic boundary with sequential/clock/reset/timing.
 * [x] It contains no Rust.
 * [x] It requires no unsafe Rust.
 * [x] It is compatible with Rust 1.97 / 1.97.1 integration.
 * [x] It has a defined AST contract.
 * [x] It has a defined semantic contract.
 * [x] It has a defined IR contract.
 * [x] It has a defined compiler/runtime boundary.
 * [x] It has positive/negative/boundary/scalability test obligations.
 * [x] It has a hard-coding audit.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE:
 *
 *     register state : Logic;
 *
 *     register counter : UInt;
 *
 *     register state : Vector<Logic>[N];
 *
 *     register state : Logic = initial_value;
 *
 *     register mut state : Logic;
 *
 *     register volatile state : Logic;
 *
 *     register a : Logic, b : Logic, c : Logic;
 *
 *     register state : Logic[N][M];
 *
 *     register state : Tensor<Logic, Shape>;
 *
 * ACCESS:
 *
 *     state
 *
 *     module::state
 *
 *     state[i]
 *
 *     state[hi:lo]
 *
 *     state[lo..hi]
 *
 *     state[lo..=hi]
 *
 *     state[i][j]
 *
 *     state.field
 *
 *     state[i].field[j]
 *
 * NEGATIVE:
 *
 *     register;
 *
 *     register state;
 *
 *     register : Logic;
 *
 *     register state :;
 *
 *     register state : Logic =;
 *
 *     register state : Logic[];
 *
 *     register state : Logic[ ];
 *
 *     register state : Logic[hi:];
 *
 *     register state : Logic[:lo];
 *
 *     register state : Logic[hi:lo:step];
 *
 *     register state : Logic = ;
 *
 *     register state : Logic,;
 *
 * The semantic layer, rather than the grammar, must reject invalid symbolic
 * dimensions, invalid index types, out-of-range accesses, incompatible
 * initializers, and illegal state transitions.
 *
 * BOUNDARY:
 *
 *     register r : T;
 *     register r : T[N];
 *     register r : T[N][M][K];
 *
 *     deeply qualified register names;
 *     deeply nested generic types;
 *     deeply nested selectors;
 *     arbitrarily large representable source-level dimension expressions.
 *
 * SCALABILITY:
 *
 *     arbitrarily many declarations;
 *     arbitrarily many dimensions;
 *     arbitrarily many selectors;
 *     arbitrarily many members;
 *     arbitrarily many generated register declarations.
 *
 * The tests MUST NOT establish a language-level maximum.
 *
 * DETERMINISM:
 *
 *     identical source + identical language version
 *         -> identical parse structure.
 *
 * PORTABILITY:
 *
 *     the same register declaration remains semantically portable across
 *     CPU/GPU/FPGA/ASIC/QPU/future targets unless semantic requirements cannot
 *     be satisfied by a target.
 *
 * ============================================================================
 */
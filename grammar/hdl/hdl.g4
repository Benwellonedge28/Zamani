parser grammar hdl;

options { tokenVocab=ZamaniTokens; }

/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hdl/hdl.g4
 *
 * Status:
 *     Production HDL parser grammar.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     Generated/compiler/runtime Rust MUST use safe Rust only.
 *     Rust `unsafe` is prohibited.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar owns the syntax of Zamani HDL once the canonical parser has
 * selected the HDL domain boundary.
 *
 * HDL syntax describes:
 *
 *     - hardware modules;
 *     - ports;
 *     - interfaces;
 *     - signals;
 *     - nets;
 *     - registers;
 *     - memories;
 *     - clocks;
 *     - resets;
 *     - combinational behavior;
 *     - sequential behavior;
 *     - processes;
 *     - state machines;
 *     - pipelines;
 *     - instances;
 *     - generated hardware;
 *     - assertions;
 *     - target-independent hardware structure.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - HDL structural syntax;
 *     - HDL module structure;
 *     - HDL port structure;
 *     - HDL declaration structure;
 *     - HDL process structure;
 *     - HDL state-machine structure;
 *     - HDL pipeline structure;
 *     - HDL instance structure;
 *     - HDL generation structure;
 *     - HDL-local expression syntax;
 *     - HDL parser-facing composition boundary.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical token definitions;
 *     - general Zamani lexical rules;
 *     - general language expressions;
 *     - general language statements;
 *     - general type semantics;
 *     - AST construction;
 *     - type checking;
 *     - hardware discovery;
 *     - target selection;
 *     - FPGA selection;
 *     - ASIC selection;
 *     - CPU selection;
 *     - GPU selection;
 *     - QPU selection;
 *     - physical pins;
 *     - physical addresses;
 *     - process technology;
 *     - placement;
 *     - routing;
 *     - synthesis;
 *     - timing closure;
 *     - scheduling;
 *     - resource allocation;
 *     - runtime dispatch.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
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
 *     hdl.g4
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> type analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> hardware legality
 *          +--> domain validation
 *          |
 *          v
 *     canonical hardware/HDL semantic representation
 *          |
 *          +--> optimization
 *          +--> synthesis
 *          +--> scheduling
 *          +--> placement
 *          +--> routing
 *          |
 *          v
 *     target lowering
 *          |
 *          +--> FPGA
 *          +--> ASIC
 *          +--> embedded hardware
 *          +--> heterogeneous hardware
 *          +--> future targets
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * HDL source MUST describe hardware semantics and intent.
 *
 * It MUST NOT permanently encode:
 *
 *     - a particular FPGA;
 *     - a particular ASIC;
 *     - a process node;
 *     - a physical pin;
 *     - a physical address;
 *     - a vendor device identifier;
 *     - a fixed number of LUTs;
 *     - a fixed number of registers;
 *     - a fixed number of BRAMs;
 *     - a fixed number of DSP units;
 *     - a fixed number of I/O banks;
 *     - a fixed clock-tree topology;
 *     - a fixed routing fabric;
 *     - a fixed accelerator count;
 *     - a fixed machine size.
 *
 * The source may express requirements and constraints when those requirements
 * are genuinely part of program semantics.
 *
 * Target realizability is a downstream concern.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are intentionally NO language-level finite limits here.
 *
 * No:
 *
 *     MAX_MODULES
 *     MAX_PORTS
 *     MAX_SIGNALS
 *     MAX_REGISTERS
 *     MAX_MEMORY
 *     MAX_STATES
 *     MAX_TRANSITIONS
 *     MAX_PIPELINE_STAGES
 *     MAX_INSTANCES
 *     MAX_GENERATION_COUNT
 *     MAX_WIDTH
 *     MAX_BITS
 *     MAX_CLOCKS
 *     MAX_PROCESSES
 *     MAX_HARDWARE_BLOCKS
 *
 * Repetition is represented structurally through ANTLR repetition operators.
 *
 * Actual limits belong to:
 *
 *     - parser resource policy;
 *     - compiler resource policy;
 *     - semantic analysis;
 *     - hardware capability analysis;
 *     - synthesis;
 *     - scheduling;
 *     - deployment;
 *     - runtime;
 *     - physical resource availability.
 *
 * ============================================================================
 * CONTEXTUAL HDL VOCABULARY
 * ============================================================================
 *
 * The canonical lexer remains the sole lexical authority.
 *
 * Therefore HDL-specific words such as:
 *
 *     input
 *     output
 *     inout
 *     wire
 *     logic
 *     signal
 *     net
 *     register
 *     memory
 *     clock
 *     reset
 *     process
 *     always
 *     combinational
 *     sequential
 *     state
 *     transition
 *     pipeline
 *     stage
 *     instance
 *     generate
 *     parameter
 *     localparam
 *     signed
 *     unsigned
 *     rising
 *     falling
 *     edge
 *     posedge
 *     negedge
 *     enable
 *     sync
 *     async
 *     latch
 *     tri
 *     pullup
 *     pulldown
 *     highz
 *     drive
 *     resolve
 *
 * are contextual HDL names rather than duplicated lexer keywords.
 *
 * Semantic analysis MUST validate the permitted contextual vocabulary.
 *
 * This deliberately prevents the HDL grammar from creating a second
 * language-wide lexer.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no Rust actions;
 *     - no Rust predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no process execution;
 *     - no mutable global state;
 *     - no unsafe code;
 *     - no target-specific code.
 *
 * ============================================================================
 */

/* ============================================================================
 * PUBLIC ENTRY POINT
 * ========================================================================== */

/*
 * Complete HDL source region.
 *
 * A design may contain arbitrarily many modules.
 */
hdlDesign
    : hdlModuleDeclaration+
    ;

/*
 * Stable parser-facing HDL module boundary.
 *
 * `module` is already owned by the canonical lexer.
 *
 * The canonical parser MUST invoke this rule only when the source has been
 * classified as an HDL domain construct, preventing collision with the
 * ordinary Zamani software-module declaration.
 */
hdlModuleDeclaration
    : K_MODULE
      identifier
      hdlGenericParameters?
      hdlPortList?
      hdlModuleBody
    ;

/* ============================================================================
 * GENERICS
 * ========================================================================== */

hdlGenericParameters
    : LT
      hdlGenericParameterList
      GT
    ;

hdlGenericParameterList
    : hdlGenericParameter
      (
          COMMA
          hdlGenericParameter
      )*
      COMMA?
    ;

hdlGenericParameter
    : identifier
      (
          COLON
          hdlTypeExpression
      )?
      (
          ASSIGN
          hdlExpression
      )?
    ;

/* ============================================================================
 * PORTS
 * ========================================================================== */

hdlPortList
    : LPAREN
      hdlPortDeclarationList?
      RPAREN
    ;

hdlPortDeclarationList
    : hdlPortDeclaration
      (
          COMMA
          hdlPortDeclaration
      )*
      COMMA?
    ;

/*
 * Port syntax is intentionally structural.
 *
 * Direction/modifier names remain contextual identifiers.
 */
hdlPortDeclaration
    : hdlAttributes*
      hdlPortDirection?
      hdlPortModifier*
      identifier
      (
          COLON
          hdlTypeExpression
      )?
      hdlPortConstraint*
    ;

hdlPortDirection
    : hdlKeyword
    ;

hdlPortModifier
    : hdlKeyword
    ;

hdlPortConstraint
    : LBRACKET
      hdlRangeExpression
      RBRACKET
    | hdlAttribute
    ;

/* ============================================================================
 * MODULE BODY
 * ========================================================================== */

hdlModuleBody
    : LBRACE
      hdlModuleMember*
      RBRACE
    ;

hdlModuleMember
    : hdlAttributes*
      (
          hdlParameterDeclaration
        | hdlTypeDeclaration
        | hdlPortDeclarationStatement
        | hdlSignalDeclaration
        | hdlNetDeclaration
        | hdlRegisterDeclaration
        | hdlMemoryDeclaration
        | hdlClockDeclaration
        | hdlAssignment
        | hdlProcessDeclaration
        | hdlAlwaysDeclaration
        | hdlCombinationalDeclaration
        | hdlSequentialDeclaration
        | hdlStateMachineDeclaration
        | hdlPipelineDeclaration
        | hdlInstanceDeclaration
        | hdlGenerateDeclaration
        | hdlBlockDeclaration
        | hdlAssertion
        | hdlExpressionStatement
      )
    ;

/* ============================================================================
 * ATTRIBUTES
 * ========================================================================== */

hdlAttributes
    : hdlAttribute+
    ;

hdlAttribute
    : AT
      identifier
    | AT
      identifier
      LPAREN
      hdlArgumentList?
      RPAREN
    ;

hdlArgumentList
    : hdlExpression
      (
          COMMA
          hdlExpression
      )*
      COMMA?
    ;

/* ============================================================================
 * PARAMETERS / TYPE DECLARATIONS
 * ========================================================================== */

hdlParameterDeclaration
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

hdlTypeDeclaration
    : K_TYPE
      identifier
      hdlGenericParameters?
      ASSIGN
      hdlTypeExpression
      SEMICOLON?
    ;

hdlPortDeclarationStatement
    : hdlKeyword
      hdlPortDeclarationList
      SEMICOLON?
    ;

/* ============================================================================
 * HDL TYPES
 * ========================================================================== */

/*
 * Hardware types are structural.
 *
 * Examples that can be represented by this grammar include semantic forms
 * such as:
 *
 *     logic
 *     bit
 *     Signal<T>
 *     Clock
 *     Bus<T>
 *     Vector<T, N>
 *     Memory<T, N>
 *
 * Semantic ownership remains outside this grammar.
 */
hdlTypeExpression
    : hdlTypePrimary
      hdlTypeSuffix*
    ;

hdlTypePrimary
    : identifier
    | qualifiedHdlName
    | LPAREN
      hdlTypeExpression
      RPAREN
    ;

hdlTypeSuffix
    : LBRACKET
      hdlRangeExpression?
      RBRACKET
    | LT
      hdlTypeArgumentList?
      GT
    ;

hdlTypeArgumentList
    : hdlTypeArgument
      (
          COMMA
          hdlTypeArgument
      )*
      COMMA?
    ;

hdlTypeArgument
    : hdlTypeExpression
    | hdlExpression
    ;

qualifiedHdlName
    : identifier
      (
          DOUBLE_COLON
          identifier
      )*
    ;

/* ============================================================================
 * SIGNALS
 * ========================================================================== */

hdlSignalDeclaration
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
 * NETS
 * ========================================================================== */

hdlNetDeclaration
    : hdlKeyword
      identifier
      (
          COLON
          hdlTypeExpression
      )?
      (
          COMMA
          identifier
      )*
      SEMICOLON
    ;

/* ============================================================================
 * REGISTERS
 * ========================================================================== */

hdlRegisterDeclaration
    : hdlKeyword
      identifier
      (
          COLON
          hdlTypeExpression
      )?
      hdlResetClause?
      (
          ASSIGN
          hdlExpression
      )?
      SEMICOLON
    ;

/* ============================================================================
 * MEMORY
 * ========================================================================== */

hdlMemoryDeclaration
    : hdlKeyword
      identifier
      COLON
      hdlTypeExpression
      hdlMemoryDimension+
      SEMICOLON
    ;

hdlMemoryDimension
    : LBRACKET
      hdlRangeExpression
      RBRACKET
    ;

/* ============================================================================
 * CLOCKS
 * ========================================================================== */

hdlClockDeclaration
    : hdlKeyword
      identifier
      (
          COLON
          hdlTypeExpression
      )?
      hdlClockProperty*
      SEMICOLON
    ;

hdlClockProperty
    : hdlKeyword
      (
          ASSIGN
          hdlExpression
      )?
    | hdlAttribute
    ;

hdlResetClause
    : hdlKeyword
      (
          LPAREN
          hdlExpression
          RPAREN
      )?
    ;

/* ============================================================================
 * PROCESSES
 * ========================================================================== */

hdlProcessDeclaration
    : hdlKeyword
      hdlProcessSensitivity?
      hdlBlock
    ;

hdlAlwaysDeclaration
    : hdlKeyword
      hdlProcessSensitivity?
      hdlBlock
    ;

hdlCombinationalDeclaration
    : hdlKeyword
      hdlBlock
    ;

hdlSequentialDeclaration
    : hdlKeyword
      hdlProcessSensitivity?
      hdlBlock
    ;

hdlProcessSensitivity
    : LPAREN
      hdlSensitivityList?
      RPAREN
    ;

hdlSensitivityList
    : hdlSensitivityItem
      (
          COMMA
          hdlSensitivityItem
      )*
      COMMA?
    ;

hdlSensitivityItem
    : hdlKeyword
    | qualifiedHdlName
    | hdlExpression
    ;

/* ============================================================================
 * HDL BLOCKS
 * ========================================================================== */

hdlBlock
    : LBRACE
      hdlBlockItem*
      RBRACE
    ;

hdlBlockItem
    : hdlAttributes*
      (
          hdlVariableDeclaration
        | hdlAssignment
        | hdlIfStatement
        | hdlCaseStatement
        | hdlForStatement
        | hdlGenerateDeclaration
        | hdlInstanceDeclaration
        | hdlProcessDeclaration
        | hdlAlwaysDeclaration
        | hdlCombinationalDeclaration
        | hdlSequentialDeclaration
        | hdlAssertion
        | hdlExpressionStatement
        | hdlBlock
      )
    ;

hdlVariableDeclaration
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
 * ASSIGNMENTS
 * ========================================================================== */

hdlAssignment
    : hdlLValue
      hdlAssignmentOperator
      hdlExpression
      SEMICOLON?
    ;

hdlAssignmentOperator
    : ASSIGN
    | PLUS_ASSIGN
    | MINUS_ASSIGN
    | STAR_ASSIGN
    | SLASH_ASSIGN
    | PERCENT_ASSIGN
    | AMP_ASSIGN
    | PIPE_ASSIGN
    | CARET_ASSIGN
    ;

hdlLValue
    : qualifiedHdlName
      hdlIndexSuffix*
    ;

/* ============================================================================
 * CONDITIONALS
 * ========================================================================== */

hdlIfStatement
    : hdlKeyword
      hdlExpression
      hdlBlock
      (
          hdlKeyword
          hdlExpression
          hdlBlock
      )*
      (
          hdlKeyword
          hdlBlock
      )?
    ;

/* ============================================================================
 * CASE / SWITCH
 * ========================================================================== */

hdlCaseStatement
    : hdlKeyword
      hdlExpression
      LBRACE
      hdlCaseArm*
      RBRACE
    ;

hdlCaseArm
    : hdlCaseLabel+
      FAT_ARROW
      (
          hdlBlock
        | hdlExpressionStatement
      )
    ;

hdlCaseLabel
    : hdlExpression
    | hdlKeyword
    ;

/* ============================================================================
 * FOR / GENERATION
 * ========================================================================== */

hdlForStatement
    : hdlKeyword
      hdlForInitializer?
      SEMICOLON
      hdlExpression?
      SEMICOLON
      hdlExpression?
      hdlBlock
    ;

hdlForInitializer
    : hdlVariableDeclaration
    | hdlAssignment
    ;

/* ============================================================================
 * STATE MACHINES
 * ========================================================================== */

hdlStateMachineDeclaration
    : hdlKeyword
      identifier
      hdlBlock
    ;

hdlStateDeclaration
    : hdlKeyword
      identifier
      (
          ASSIGN
          hdlExpression
      )?
      SEMICOLON?
    ;

hdlTransitionDeclaration
    : hdlKeyword
      identifier
      THIN_ARROW
      identifier
      (
          COLON
          hdlExpression
      )?
      SEMICOLON?
    ;

/* ============================================================================
 * PIPELINES
 * ========================================================================== */

hdlPipelineDeclaration
    : hdlKeyword
      identifier?
      hdlPipelineStage+
    ;

hdlPipelineStage
    : hdlKeyword
      identifier?
      hdlBlock
    ;

/*
 * Pipeline depth is unbounded at the language level.
 *
 * No maximum number of stages is encoded.
 */

/* ============================================================================
 * INSTANCES
 * ========================================================================== */

hdlInstanceDeclaration
    : hdlKeyword
      qualifiedHdlName
      (
          LPAREN
          hdlArgumentList?
          RPAREN
      )?
      (
          LBRACE
          hdlInstanceBinding*
          RBRACE
      )?
      SEMICOLON?
    ;

hdlInstanceBinding
    : identifier
      ASSIGN
      hdlExpression
      SEMICOLON?
    ;

/* ============================================================================
 * GENERATION
 * ========================================================================== */

hdlGenerateDeclaration
    : hdlKeyword
      (
          hdlGenerateFor
        | hdlGenerateIf
        | hdlGenerateCase
      )
    ;

hdlGenerateFor
    : LPAREN
      hdlExpression?
      SEMICOLON
      hdlExpression?
      SEMICOLON
      hdlExpression?
      RPAREN
      hdlBlock
    ;

hdlGenerateIf
    : LPAREN
      hdlExpression
      RPAREN
      hdlBlock
      (
          hdlKeyword
          hdlBlock
      )?
    ;

hdlGenerateCase
    : LPAREN
      hdlExpression
      RPAREN
      LBRACE
      hdlCaseArm*
      RBRACE
    ;

/* ============================================================================
 * GENERIC BLOCKS
 * ========================================================================== */

hdlBlockDeclaration
    : hdlKeyword
      identifier?
      hdlBlock
    ;

/* ============================================================================
 * ASSERTIONS
 * ========================================================================== */

hdlAssertion
    : hdlKeyword
      LPAREN
      hdlExpression
      RPAREN
      SEMICOLON?
    ;

hdlExpressionStatement
    : hdlExpression
      SEMICOLON?
    ;

/* ============================================================================
 * RANGES / DIMENSIONS
 * ========================================================================== */

hdlRangeExpression
    : hdlExpression
      (
          COLON
          hdlExpression
          (
              COLON
              hdlExpression
          )?
      )?
    ;

hdlIndexSuffix
    : LBRACKET
      hdlRangeExpression
      RBRACKET
    ;

/* ============================================================================
 * EXPRESSIONS
 * ========================================================================== */

/*
 * This expression subsystem exists so hdl.g4 can be independently parsed and
 * tested.
 *
 * It does NOT create a second semantic expression system.
 *
 * During canonical frontend integration, the resulting expression nodes MUST
 * map to the repository's canonical expression AST.
 */

hdlExpression
    : hdlAssignmentExpression
    ;

hdlAssignmentExpression
    : hdlLogicalOrExpression
      (
          ASSIGN
          hdlAssignmentExpression
      )?
    ;

hdlLogicalOrExpression
    : hdlLogicalAndExpression
      (
          LOGICAL_OR
          hdlLogicalAndExpression
      )*
    ;

hdlLogicalAndExpression
    : hdlBitOrExpression
      (
          LOGICAL_AND
          hdlBitOrExpression
      )*
    ;

hdlBitOrExpression
    : hdlBitXorExpression
      (
          BIT_OR
          hdlBitXorExpression
      )*
    ;

hdlBitXorExpression
    : hdlBitAndExpression
      (
          CARET
          hdlBitAndExpression
      )*
    ;

hdlBitAndExpression
    : hdlEqualityExpression
      (
          BIT_AND
          hdlEqualityExpression
      )*
    ;

hdlEqualityExpression
    : hdlRelationalExpression
      (
          EQUAL_EQUAL
        | NOT_EQUAL
      )
      hdlRelationalExpression
    | hdlRelationalExpression
    ;

hdlRelationalExpression
    : hdlShiftExpression
      (
          LESS
        | GREATER
        | LESS_EQUAL
        | GREATER_EQUAL
      )
      hdlShiftExpression
    | hdlShiftExpression
    ;

hdlShiftExpression
    : hdlAdditiveExpression
      (
          LEFT_SHIFT
        | RIGHT_SHIFT
      )
      hdlAdditiveExpression
    | hdlAdditiveExpression
    ;

hdlAdditiveExpression
    : hdlMultiplicativeExpression
      (
          PLUS
        | MINUS
      )
      hdlMultiplicativeExpression
    | hdlMultiplicativeExpression
    ;

hdlMultiplicativeExpression
    : hdlUnaryExpression
      (
          STAR
        | SLASH
        | PERCENT
      )
      hdlUnaryExpression
    | hdlUnaryExpression
    ;

hdlUnaryExpression
    : (
          PLUS
        | MINUS
        | BIT_NOT
        | LOGICAL_NOT
      )
      hdlUnaryExpression
    | hdlPrimaryExpression
    ;

hdlPrimaryExpression
    : literal
    | qualifiedHdlName
      hdlPostfix*
    | LPAREN
      hdlExpression
      RPAREN
    ;

hdlPostfix
    : hdlIndexSuffix
    | DOT
      identifier
    | LPAREN
      hdlArgumentList?
      RPAREN
    ;

/* ============================================================================
 * LITERALS
 * ========================================================================== */

literal
    : INTEGER_LITERAL
    | FLOAT_LITERAL
    | STRING_LITERAL
    | CHAR_LITERAL
    | QUANTUM_LITERAL
    | K_TRUE
    | K_FALSE
    | K_NIL
    | K_NULL
    ;

/* ============================================================================
 * CONTEXTUAL HDL VOCABULARY
 * ========================================================================== */

/*
 * HDL contextual keywords deliberately remain identifiers.
 *
 * The semantic layer MUST validate their permitted meanings.
 *
 * This includes, but is not limited to:
 *
 *     input
 *     output
 *     inout
 *     wire
 *     logic
 *     bit
 *     signal
 *     net
 *     register
 *     memory
 *     clock
 *     reset
 *     process
 *     always
 *     combinational
 *     sequential
 *     state
 *     transition
 *     pipeline
 *     stage
 *     instance
 *     generate
 *     parameter
 *     localparam
 *     signed
 *     unsigned
 *     rising
 *     falling
 *     edge
 *     posedge
 *     negedge
 *     enable
 *     sync
 *     async
 *     latch
 *     tri
 *     pullup
 *     pulldown
 *     highz
 *     drive
 *     resolve
 */
hdlKeyword
    : identifier
    ;

identifier
    : IDENTIFIER
    ;

/* ============================================================================
 * COMPLETION CONTRACT
 * ========================================================================== */

/*
 * This file is complete when the following integration contract is satisfied:
 *
 * 1. TOKEN OWNERSHIP
 *
 *    ZamaniTokens remains the sole lexical authority.
 *
 * 2. PARSER OWNERSHIP
 *
 *    The canonical parser invokes hdlModuleDeclaration only at an HDL domain
 *    boundary. Ordinary software modules remain owned by the ordinary parser.
 *
 * 3. AST
 *
 *    Every public HDL rule maps to an existing or explicitly approved HDL
 *    AST node.
 *
 *    The AST MUST preserve:
 *
 *        - source span;
 *        - module identity;
 *        - ports;
 *        - declarations;
 *        - expressions;
 *        - processes;
 *        - state machines;
 *        - pipeline structure;
 *        - instances;
 *        - generation;
 *        - attributes.
 *
 *    The AST MUST NOT manufacture:
 *
 *        PhysicalDeviceId
 *        PhysicalPinId
 *        PhysicalRegisterId
 *        HardwareAddress
 *        Placement
 *        RoutingSlot
 *        ScheduleSlot
 *
 * 4. SEMANTICS
 *
 *    Semantic analysis validates:
 *
 *        - contextual HDL vocabulary;
 *        - declaration meaning;
 *        - type compatibility;
 *        - signal direction;
 *        - clock/reset legality;
 *        - process semantics;
 *        - state-machine legality;
 *        - pipeline legality;
 *        - instance compatibility;
 *        - parameter constraints;
 *        - hardware capability requirements.
 *
 * 5. RESOURCE MODEL
 *
 *    Resource requirements are represented semantically and checked against
 *    available target capabilities.
 *
 *    This grammar never imposes a finite hardware limit.
 *
 * 6. SYNTHESIS
 *
 *    Synthesis consumes semantic HDL representation.
 *
 *    This grammar does not synthesize hardware.
 *
 * 7. SCHEDULING
 *
 *    Scheduling consumes semantic dependencies and timing requirements.
 *
 *    This grammar does not choose physical execution times.
 *
 * 8. PLACEMENT / ROUTING
 *
 *    Placement and routing consume target-independent hardware semantics.
 *
 *    This grammar does not select physical resources.
 *
 * 9. TARGET LOWERING
 *
 *    Target lowering determines the actual FPGA, ASIC, embedded or other
 *    implementation.
 *
 * 10. POCO-REAF
 *
 *     The same HDL source semantics must remain portable across targets where
 *     the target can satisfy the declared semantic requirements.
 *
 * 11. TESTING
 *
 *     The grammar requires:
 *
 *         - positive parser tests;
 *         - negative parser tests;
 *         - empty/large structural tests;
 *         - expression tests;
 *         - module tests;
 *         - port tests;
 *         - memory tests;
 *         - process tests;
 *         - state-machine tests;
 *         - pipeline tests;
 *         - generation tests;
 *         - cross-domain tests;
 *         - determinism tests;
 *         - hard-coding/scalability tests.
 *
 * 12. HARD-CODING AUDIT
 *
 *     This grammar MUST remain free of:
 *
 *         MAX_MODULES
 *         MAX_PORTS
 *         MAX_BITS
 *         MAX_REGISTERS
 *         MAX_STATES
 *         MAX_PIPELINE_STAGES
 *         MAX_INSTANCES
 *         MAX_MEMORY
 *         MAX_DEVICES
 *         MAX_CLOCKS
 *         MAX_FPGAS
 *         MAX_ASICS
 *         MAX_LUTS
 *         MAX_DSP
 *         MAX_BRAM
 *
 * 13. SAFETY
 *
 *     No unsafe Rust, embedded Rust, runtime calls, I/O, network access,
 *     filesystem access or mutable global state may be introduced into this
 *     grammar.
 */
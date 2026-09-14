parser grammar hdl;

options {
    tokenVocab = ZamaniTokens;
}

/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hdl/hdl.g4
 *
 * Authority:
 *     Canonical HDL parser grammar.
 *
 * Purpose:
 *     Defines the syntactic structure of Zamani hardware-description and
 *     hardware/software-co-design constructs.
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no target-language actions, predicates,
 *     filesystem access, networking, process execution, mutable global state,
 *     or unsafe code.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - HDL source structure;
 *   - hardware-module declarations;
 *   - HDL interfaces and ports;
 *   - HDL signals and nets;
 *   - registers;
 *   - memories;
 *   - clocks and resets;
 *   - combinational processes;
 *   - sequential processes;
 *   - procedural HDL;
 *   - state machines;
 *   - pipelines;
 *   - module instances;
 *   - generate constructs;
 *   - HDL assertions;
 *   - HDL timing declarations;
 *   - HDL-local expressions;
 *   - HDL-local type syntax;
 *   - target-independent hardware intent syntax.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexical recognition;
 *   - AST implementation;
 *   - semantic type checking;
 *   - hardware discovery;
 *   - target selection;
 *   - physical placement;
 *   - physical routing;
 *   - synthesis;
 *   - timing closure;
 *   - scheduling;
 *   - resource allocation;
 *   - FPGA/ASIC implementation;
 *   - CPU/GPU/QPU selection;
 *   - quantum IR;
 *   - QEC;
 *   - ZQN;
 *   - runtime execution.
 *
 * ============================================================================
 * ARCHITECTURE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniTokens
 *          |
 *          v
 *     canonical parser/domain dispatcher
 *          |
 *          v
 *     hdlDesign
 *          |
 *          v
 *     HDL frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> type checking
 *          +--> width checking
 *          +--> capability checking
 *          +--> resource checking
 *          +--> clock-domain analysis
 *          +--> hardware legality
 *          |
 *          v
 *     canonical hardware semantic representation
 *          |
 *          +--> optimization
 *          +--> synthesis
 *          +--> scheduling
 *          +--> placement
 *          +--> routing
 *          +--> target lowering
 *          |
 *          v
 *     hardware / runtime / deployment
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * HDL source expresses:
 *
 *     - what hardware means;
 *     - what interfaces exist;
 *     - what behavior is required;
 *     - what timing relationships are semantic;
 *     - what resource/capability requirements are genuine.
 *
 * HDL source does NOT implicitly select:
 *
 *     - a specific FPGA;
 *     - a specific ASIC;
 *     - a particular fabrication node;
 *     - a particular vendor;
 *     - a particular package;
 *     - physical pins;
 *     - physical addresses;
 *     - a fixed LUT count;
 *     - a fixed BRAM count;
 *     - a fixed DSP count;
 *     - a fixed register count;
 *     - a fixed routing topology;
 *     - a fixed machine size.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * No grammar-level maximum is imposed on:
 *
 *     modules
 *     ports
 *     signals
 *     nets
 *     registers
 *     memories
 *     states
 *     transitions
 *     pipeline stages
 *     instances
 *     generated instances
 *     widths
 *     dimensions
 *     processes
 *     clocks
 *     hardware blocks
 *
 * ANTLR repetition operators represent arbitrary cardinality.
 *
 * Resource limits belong to:
 *
 *     parser resource policy
 *     compiler resource policy
 *     semantic analysis
 *     synthesis
 *     target capability analysis
 *     deployment
 *     runtime
 *     physical resource availability
 *
 * ============================================================================
 * IMPORTANT INTEGRATION RULE
 * ============================================================================
 *
 * HDL-specific keywords are owned by ZamaniTokens.
 *
 * DO NOT create a second lexer in this file.
 *
 * The canonical lexer therefore needs stable HDL tokens such as:
 *
 *     K_MODULE
 *     K_INPUT
 *     K_OUTPUT
 *     K_INOUT
 *     K_INTERFACE
 *     K_SIGNAL
 *     K_WIRE
 *     K_NET
 *     K_LOGIC
 *     K_REGISTER
 *     K_MEMORY
 *     K_CLOCK
 *     K_RESET
 *     K_PROCESS
 *     K_ALWAYS
 *     K_COMBINATIONAL
 *     K_SEQUENTIAL
 *     K_STATE
 *     K_TRANSITION
 *     K_PIPELINE
 *     K_STAGE
 *     K_INSTANCE
 *     K_GENERATE
 *     K_PARAMETER
 *     K_LOCALPARAM
 *     K_SIGNED
 *     K_UNSIGNED
 *     K_RISING
 *     K_FALLING
 *     K_POSEDGE
 *     K_NEGEDGE
 *     K_ENABLE
 *     K_SYNCHRONOUS
 *     K_ASYNCHRONOUS
 *     K_LATCH
 *     K_TRI
 *     K_PULLUP
 *     K_PULLDOWN
 *     K_HIGHZ
 *     K_DRIVE
 *     K_RESOLVE
 *     K_ASSERT
 *     K_ASSUME
 *     K_COVER
 *     K_FOREVER
 *     K_REPEAT
 *     K_CONNECT
 *     K_MAP
 *     K_TO
 *     K_FROM
 *     K_DOMAIN
 *     K_TIMING
 *     K_LATENCY
 *     K_THROUGHPUT
 *     K_FREQUENCY
 *     K_DUTY
 *     K_PHASE
 *
 * These are lexical-language compatibility changes, not semantic hardware
 * assumptions.
 *
 * ============================================================================
 */

/* ============================================================================
 * PUBLIC ENTRY POINT
 * ========================================================================== */

hdlDesign
    : hdlModuleDeclaration+
    EOF
    ;

/* ============================================================================
 * MODULES
 * ========================================================================== */

hdlModuleDeclaration
    : K_MODULE
      identifier
      hdlGenericParameterBlock?
      hdlPortBlock?
      hdlModuleBody
    ;

hdlModuleBody
    : LBRACE
      hdlModuleMember*
      RBRACE
    ;

hdlModuleMember
    : hdlAttributes*
      (
          hdlParameterDeclaration
        | hdlLocalParameterDeclaration
        | hdlTypeDeclaration
        | hdlInterfaceDeclaration
        | hdlPortDeclarationStatement
        | hdlSignalDeclaration
        | hdlNetDeclaration
        | hdlRegisterDeclaration
        | hdlMemoryDeclaration
        | hdlClockDeclaration
        | hdlResetDeclaration
        | hdlAssignment
        | hdlProcessDeclaration
        | hdlAlwaysDeclaration
        | hdlCombinationalDeclaration
        | hdlSequentialDeclaration
        | hdlStateMachineDeclaration
        | hdlPipelineDeclaration
        | hdlInstanceDeclaration
        | hdlGenerateDeclaration
        | hdlTimingDeclaration
        | hdlAssertion
        | hdlBlockDeclaration
        | hdlExpressionStatement
      )
    ;

/* ============================================================================
 * GENERICS / PARAMETERS
 * ========================================================================== */

hdlGenericParameterBlock
    : LESS_THAN
      hdlGenericParameterList
      GREATER_THAN
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
      hdlConstraintClause*
    ;

hdlParameterDeclaration
    : K_PARAMETER
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

hdlLocalParameterDeclaration
    : K_LOCALPARAM
      identifier
      (
          COLON
          hdlTypeExpression
      )?
      ASSIGN
      hdlExpression
      SEMICOLON
    ;

hdlConstraintClause
    : K_REQUIRES
      hdlExpression
    | K_WHERE
      hdlExpression
    ;

/* ============================================================================
 * PORTS
 * ========================================================================== */

hdlPortBlock
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
    : K_INPUT
    | K_OUTPUT
    | K_INOUT
    ;

hdlPortModifier
    : K_SIGNED
    | K_UNSIGNED
    | K_TRI
    | K_HIGHZ
    ;

hdlPortConstraint
    : LBRACKET
      hdlRangeExpression
      RBRACKET
    | hdlAttribute
    ;

/*
 * A declaration form is also permitted inside a module body.
 */
hdlPortDeclarationStatement
    : hdlPortDirection
      hdlPortDeclarationList
      SEMICOLON
    ;

/* ============================================================================
 * INTERFACES
 * ========================================================================== */

hdlInterfaceDeclaration
    : K_INTERFACE
      identifier
      hdlGenericParameterBlock?
      hdlInterfaceBody
    ;

hdlInterfaceBody
    : LBRACE
      hdlInterfaceMember*
      RBRACE
    ;

hdlInterfaceMember
    : hdlAttributes*
      (
          hdlInterfacePort
        | hdlInterfaceSignal
        | hdlInterfaceParameter
        | hdlTypeDeclaration
      )
    ;

hdlInterfacePort
    : hdlPortDirection
      identifier
      (
          COLON
          hdlTypeExpression
      )?
      SEMICOLON
    ;

hdlInterfaceSignal
    : K_SIGNAL
      identifier
      (
          COLON
          hdlTypeExpression
      )?
      SEMICOLON
    ;

hdlInterfaceParameter
    : K_PARAMETER
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
 * ATTRIBUTES
 * ========================================================================== */

hdlAttributes
    : hdlAttribute+
    ;

hdlAttribute
    : AT
      identifier
      (
          LPAREN
          hdlArgumentList?
          RPAREN
      )?
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
 * TYPES
 * ========================================================================== */

hdlTypeDeclaration
    : K_TYPE
      identifier
      hdlGenericParameterBlock?
      ASSIGN
      hdlTypeExpression
      SEMICOLON
    ;

hdlTypeExpression
    : hdlTypePrimary
      hdlTypeSuffix*
    ;

hdlTypePrimary
    : identifier
    | hdlQualifiedName
    | hdlBuiltinType
    | LPAREN
      hdlTypeExpression
      RPAREN
    ;

hdlBuiltinType
    : K_LOGIC
    | K_BIT
    | K_BOOL
    | K_INT
    | K_UINT
    | K_SIGNED
    | K_UNSIGNED
    ;

hdlTypeSuffix
    : LBRACKET
      hdlRangeExpression?
      RBRACKET
    | LESS_THAN
      hdlTypeArgumentList
      GREATER_THAN
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

/* ============================================================================
 * SIGNALS / NETS
 * ========================================================================== */

hdlSignalDeclaration
    : K_SIGNAL
      hdlSignalDeclaratorList
      SEMICOLON
    ;

hdlSignalDeclaratorList
    : hdlSignalDeclarator
      (
          COMMA
          hdlSignalDeclarator
      )*
      COMMA?
    ;

hdlSignalDeclarator
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

hdlNetDeclaration
    : K_NET
      hdlNetKind?
      hdlNetDeclaratorList
      SEMICOLON
    ;

hdlNetKind
    : K_WIRE
    | K_LOGIC
    | K_TRI
    ;

hdlNetDeclaratorList
    : hdlNetDeclarator
      (
          COMMA
          hdlNetDeclarator
      )*
      COMMA?
    ;

hdlNetDeclarator
    : identifier
      (
          COLON
          hdlTypeExpression
      )?
    ;

/* ============================================================================
 * REGISTERS
 * ========================================================================== */

hdlRegisterDeclaration
    : K_REGISTER
      identifier
      (
          COLON
          hdlTypeExpression
      )?
      hdlRegisterProperty*
      (
          ASSIGN
          hdlExpression
      )?
      SEMICOLON
    ;

hdlRegisterProperty
    : hdlResetClause
    | hdlClockReference
    | hdlAttribute
    ;

/* ============================================================================
 * MEMORY
 * ========================================================================== */

hdlMemoryDeclaration
    : K_MEMORY
      identifier
      COLON
      hdlTypeExpression
      hdlMemoryDimension+
      hdlMemoryProperty*
      SEMICOLON
    ;

hdlMemoryDimension
    : LBRACKET
      hdlRangeExpression
      RBRACKET
    ;

hdlMemoryProperty
    : K_READ
      hdlMemoryAccessMode?
    | K_WRITE
      hdlMemoryAccessMode?
    | hdlAttribute
    ;

hdlMemoryAccessMode
    : K_SYNC
    | K_ASYNC
    ;

/* ============================================================================
 * CLOCKS / RESETS
 * ========================================================================== */

hdlClockDeclaration
    : K_CLOCK
      identifier
      (
          COLON
          hdlTypeExpression
      )?
      hdlClockProperty*
      SEMICOLON
    ;

hdlClockProperty
    : K_FREQUENCY
      ASSIGN
      hdlExpression
    | K_DUTY
      ASSIGN
      hdlExpression
    | K_PHASE
      ASSIGN
      hdlExpression
    | hdlAttribute
    ;

hdlResetDeclaration
    : K_RESET
      identifier
      (
          COLON
          hdlTypeExpression
      )?
      hdlResetProperty*
      SEMICOLON
    ;

hdlResetProperty
    : K_SYNCHRONOUS
    | K_ASYNCHRONOUS
    | K_ACTIVE_HIGH
    | K_ACTIVE_LOW
    | hdlAttribute
    ;

hdlResetClause
    : K_RESET
      (
          LPAREN
          hdlExpression
          RPAREN
      )?
    ;

hdlClockReference
    : K_CLOCK
      LPAREN
      hdlExpression
      RPAREN
    ;

/* ============================================================================
 * PROCESSES
 * ========================================================================== */

hdlProcessDeclaration
    : K_PROCESS
      hdlProcessSensitivity?
      hdlBlock
    ;

hdlAlwaysDeclaration
    : K_ALWAYS
      hdlProcessSensitivity?
      hdlBlock
    ;

hdlCombinationalDeclaration
    : K_COMBINATIONAL
      hdlBlock
    ;

hdlSequentialDeclaration
    : K_SEQUENTIAL
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
    : K_POSEDGE
      hdlExpression
    | K_NEGEDGE
      hdlExpression
    | K_RISING
      hdlExpression
    | K_FALLING
      hdlExpression
    | hdlExpression
    ;

/* ============================================================================
 * BLOCKS
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
        | hdlWhileStatement
        | hdlRepeatStatement
        | hdlGenerateDeclaration
        | hdlInstanceDeclaration
        | hdlProcessDeclaration
        | hdlAlwaysDeclaration
        | hdlCombinationalDeclaration
        | hdlSequentialDeclaration
        | hdlAssertion
        | hdlBlockDeclaration
        | hdlExpressionStatement
      )
    ;

hdlBlockDeclaration
    : identifier
      hdlBlock
    ;

/* ============================================================================
 * LOCAL VARIABLES
 * ========================================================================== */

hdlVariableDeclaration
    : hdlVariableKind
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

hdlVariableKind
    : K_LET
    | K_VAR
    | K_CONST
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
    : hdlQualifiedName
      hdlIndexSuffix*
    ;

/* ============================================================================
 * CONDITIONALS
 * ========================================================================== */

hdlIfStatement
    : K_IF
      hdlExpression
      hdlBlock
      (
          K_ELSE
          (
              K_IF
              hdlExpression
              hdlBlock
            | hdlBlock
          )
      )*
    ;

/* ============================================================================
 * CASE / SWITCH
 * ========================================================================== */

hdlCaseStatement
    : K_CASE
      LPAREN
      hdlExpression
      RPAREN
      LBRACE
      hdlCaseItem*
      RBRACE
    ;

hdlCaseItem
    : K_DEFAULT
      COLON
      hdlBlock
    | hdlCaseExpressionList
      COLON
      hdlBlock
    ;

hdlCaseExpressionList
    : hdlExpression
      (
          COMMA
          hdlExpression
      )*
    ;

/* ============================================================================
 * LOOPS / GENERATION
 * ========================================================================== */

hdlForStatement
    : K_FOR
      LPAREN
      hdlForInitializer?
      SEMICOLON
      hdlExpression?
      SEMICOLON
      hdlExpression?
      RPAREN
      hdlBlock
    ;

hdlForInitializer
    : hdlVariableDeclarationNoTerminator
    | hdlAssignment
    ;

hdlVariableDeclarationNoTerminator
    : hdlVariableKind
      identifier
      (
          COLON
          hdlTypeExpression
      )?
      (
          ASSIGN
          hdlExpression
      )?
    ;

hdlWhileStatement
    : K_WHILE
      hdlExpression
      hdlBlock
    ;

hdlRepeatStatement
    : K_REPEAT
      hdlExpression
      hdlBlock
    ;

hdlGenerateDeclaration
    : K_GENERATE
      (
          hdlGenerateFor
        | hdlGenerateIf
        | hdlGenerateBlock
      )
    ;

hdlGenerateFor
    : K_FOR
      LPAREN
      hdlVariableDeclarationNoTerminator
      SEMICOLON
      hdlExpression
      SEMICOLON
      hdlExpression
      RPAREN
      hdlBlock
    ;

hdlGenerateIf
    : K_IF
      hdlExpression
      hdlBlock
      (
          K_ELSE
          hdlBlock
      )?
    ;

hdlGenerateBlock
    : hdlBlock
    ;

/* ============================================================================
 * INSTANCES / CONNECTIONS
 * ========================================================================== */

hdlInstanceDeclaration
    : K_INSTANCE
      hdlQualifiedName
      identifier
      hdlInstanceParameterOverrides?
      hdlInstancePortConnections?
      SEMICOLON
    ;

hdlInstanceParameterOverrides
    : LESS_THAN
      hdlNamedArgumentList?
      GREATER_THAN
    ;

hdlInstancePortConnections
    : LPAREN
      hdlConnectionList?
      RPAREN
    ;

hdlConnectionList
    : hdlConnection
      (
          COMMA
          hdlConnection
      )*
      COMMA?
    ;

hdlConnection
    : identifier
      COLON
      hdlExpression
    | hdlExpression
    ;

hdlNamedArgumentList
    : hdlNamedArgument
      (
          COMMA
          hdlNamedArgument
      )*
      COMMA?
    ;

hdlNamedArgument
    : identifier
      ASSIGN
      hdlExpression
    ;

/* ============================================================================
 * STATE MACHINES
 * ========================================================================== */

hdlStateMachineDeclaration
    : K_STATE
      K_MACHINE?
      identifier
      hdlStateMachineTypeParameters?
      hdlStateMachineBody
    ;

hdlStateMachineTypeParameters
    : LESS_THAN
      hdlTypeArgumentList
      GREATER_THAN
    ;

hdlStateMachineBody
    : LBRACE
      hdlStateDeclaration*
      hdlInitialStateDeclaration?
      hdlTransitionDeclaration*
      RBRACE
    ;

hdlStateDeclaration
    : K_STATE
      identifier
      (
          ASSIGN
          hdlExpression
      )?
      SEMICOLON
    ;

hdlInitialStateDeclaration
    : K_INITIAL
      identifier
      SEMICOLON
    ;

hdlTransitionDeclaration
    : K_TRANSITION
      identifier
      (
          K_TO
          identifier
      )?
      hdlTransitionGuard?
      hdlTransitionAction?
      SEMICOLON
    ;

hdlTransitionGuard
    : K_WHEN
      hdlExpression
    ;

hdlTransitionAction
    : K_DO
      hdlExpression
    ;

/* ============================================================================
 * PIPELINES
 * ========================================================================== */

hdlPipelineDeclaration
    : K_PIPELINE
      identifier?
      hdlPipelineProperty*
      hdlPipelineBody
    ;

hdlPipelineProperty
    : K_LATENCY
      ASSIGN
      hdlExpression
    | K_THROUGHPUT
      ASSIGN
      hdlExpression
    | hdlAttribute
    ;

hdlPipelineBody
    : LBRACE
      hdlPipelineStage+
      RBRACE
    ;

hdlPipelineStage
    : K_STAGE
      identifier?
      hdlPipelineStageProperty*
      hdlBlock
    ;

hdlPipelineStageProperty
    : K_LATENCY
      ASSIGN
      hdlExpression
    | K_ENABLE
      ASSIGN
      hdlExpression
    | hdlAttribute
    ;

/* ============================================================================
 * TIMING
 * ========================================================================== */

hdlTimingDeclaration
    : K_TIMING
      hdlTimingTarget?
      hdlTimingProperty+
      SEMICOLON
    ;

hdlTimingTarget
    : identifier
    | hdlQualifiedName
    ;

hdlTimingProperty
    : K_LATENCY
      ASSIGN
      hdlExpression
    | K_THROUGHPUT
      ASSIGN
      hdlExpression
    | K_FREQUENCY
      ASSIGN
      hdlExpression
    | K_PHASE
      ASSIGN
      hdlExpression
    | hdlAttribute
    ;

/* ============================================================================
 * ASSERTIONS
 * ========================================================================== */

hdlAssertion
    : hdlAssertionKind
      LPAREN
      hdlExpression
      RPAREN
      SEMICOLON
    ;

hdlAssertionKind
    : K_ASSERT
    | K_ASSUME
    | K_COVER
    ;

/* ============================================================================
 * RANGE / WIDTH EXPRESSIONS
 * ========================================================================== */

hdlRangeExpression
    : hdlExpression
      hdlRangeOperator
      hdlExpression
    | hdlExpression
    ;

hdlRangeOperator
    : COLON
    | DOT_DOT
    | DOT_DOT_EQ
    ;

hdlIndexSuffix
    : LBRACKET
      hdlRangeExpression
      RBRACKET
    ;

/* ============================================================================
 * EXPRESSIONS
 *
 * Expressions are deliberately local to the HDL parser. They produce parser
 * structure only; semantic lowering determines the canonical expression/type
 * representation.
 * ========================================================================== */

hdlExpression
    : hdlLogicalOrExpression
    ;

hdlLogicalOrExpression
    : hdlLogicalAndExpression
      (
          LOGICAL_OR
          hdlLogicalAndExpression
      )*
    ;

hdlLogicalAndExpression
    : hdlBitwiseOrExpression
      (
          LOGICAL_AND
          hdlBitwiseOrExpression
      )*
    ;

hdlBitwiseOrExpression
    : hdlBitwiseXorExpression
      (
          PIPE
          hdlBitwiseXorExpression
      )*
    ;

hdlBitwiseXorExpression
    : hdlBitwiseAndExpression
      (
          CARET
          hdlBitwiseAndExpression
      )*
    ;

hdlBitwiseAndExpression
    : hdlEqualityExpression
      (
          AMPERSAND
          hdlEqualityExpression
      )*
    ;

hdlEqualityExpression
    : hdlRelationalExpression
      (
          EQUAL_EQUAL
          hdlRelationalExpression
        | NOT_EQUAL
          hdlRelationalExpression
      )*
    ;

hdlRelationalExpression
    : hdlShiftExpression
      (
          LESS_EQUAL
          hdlShiftExpression
        | GREATER_EQUAL
          hdlShiftExpression
        | LESS_THAN
          hdlShiftExpression
        | GREATER_THAN
          hdlShiftExpression
      )*
    ;

hdlShiftExpression
    : hdlAdditiveExpression
      (
          LEFT_SHIFT
          hdlAdditiveExpression
        | RIGHT_SHIFT
          hdlAdditiveExpression
      )*
    ;

hdlAdditiveExpression
    : hdlMultiplicativeExpression
      (
          PLUS
          hdlMultiplicativeExpression
        | MINUS
          hdlMultiplicativeExpression
      )*
    ;

hdlMultiplicativeExpression
    : hdlUnaryExpression
      (
          STAR
          hdlUnaryExpression
        | SLASH
          hdlUnaryExpression
        | PERCENT
          hdlUnaryExpression
      )*
    ;

hdlUnaryExpression
    : PLUS
      hdlUnaryExpression
    | MINUS
      hdlUnaryExpression
    | EXCLAMATION
      hdlUnaryExpression
    | TILDE
      hdlUnaryExpression
    | K_NOT
      hdlUnaryExpression
    | hdlPostfixExpression
    ;

hdlPostfixExpression
    : hdlPrimaryExpression
      (
          hdlIndexSuffix
        | DOT
          identifier
        | QUESTION_DOT
          identifier
        | LPAREN
          hdlArgumentList?
          RPAREN
      )*
    ;

hdlPrimaryExpression
    : identifier
    | hdlLiteral
    | hdlQualifiedName
    | LPAREN
      hdlExpression
      RPAREN
    ;

hdlLiteral
    : INTEGER_LITERAL
    | HEX_INTEGER
    | BINARY_INTEGER
    | OCTAL_INTEGER
    | FLOAT_LITERAL
    | CHAR_LITERAL
    | STRING_LITERAL
    | QUANTUM_LITERAL
    | K_TRUE
    | K_FALSE
    | K_NULL
    | K_NIL
    ;

/* ============================================================================
 * QUALIFIED NAMES
 * ========================================================================== */

identifier
    : IDENTIFIER
    ;

hdlQualifiedName
    : identifier
      (
          DOUBLE_COLON
          identifier
      )*
    ;

/* ============================================================================
 * EXPRESSION STATEMENTS
 * ========================================================================== */

hdlExpressionStatement
    : hdlExpression
      SEMICOLON
    ;
/**
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/interoperability/verilog.g4
 *
 * Authority:
 *     Canonical external-Verilog interoperability grammar.
 *
 * Purpose:
 *     Parse IEEE 1364-2005 Verilog source as an external input format for
 *     Zamani's hardware/software co-design and interoperability pipeline.
 *
 * Scope:
 *     - Verilog-2001 / Verilog-2005 source syntax
 *     - module declarations
 *     - ports
 *     - parameters
 *     - nets and variables
 *     - continuous assignments
 *     - procedural assignments
 *     - always / initial
 *     - event controls
 *     - timing controls
 *     - functions
 *     - tasks
 *     - generate constructs
 *     - module instantiation
 *     - primitive instantiation
 *     * UDP declarations
 *     * specify blocks
 *     * timing paths
 *     * attributes
 *     * compiler directives
 *     * system tasks/functions
 *     * Verilog expressions and literals
 *
 * IMPORTANT ARCHITECTURAL BOUNDARY
 * ---------------------------------
 *
 * This grammar is an interoperability grammar.
 *
 * It is NOT the canonical Zamani grammar.
 *
 * It is NOT the canonical Zamani HDL semantic model.
 *
 * It is NOT an IR.
 *
 * It does NOT select:
 *     - FPGA
 *     - ASIC
 *     - CPU
 *     - GPU
 *     - QPU
 *     - device
 *     - package
 *     - pin
 *     - address
 *     - topology
 *     - routing
 *     - placement
 *     - resource count
 *
 * Pipeline:
 *
 *     Verilog source
 *          |
 *          v
 *     Verilog lexer/parser
 *          |
 *          v
 *     Verilog interoperability AST
 *          |
 *          v
 *     Verilog semantic normalization
 *          |
 *          v
 *     Zamani HDL semantic model / canonical hardware IR
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
 * The grammar must never directly construct or depend on:
 *
 *     quantum::ir
 *     QEC
 *     ZQN
 *     scheduler state
 *     routing state
 *     hardware discovery
 *     runtime state
 *
 * Those systems consume normalized semantic representations downstream.
 *
 * ============================================================================
 * RUST / SAFETY
 * ============================================================================
 *
 * Generated parser target:
 *
 *     Rust 1.97 / Rust 1.97.1
 *
 * This grammar contains:
 *
 *     - no Rust actions
 *     - no semantic predicates
 *     - no filesystem access
 *     - no network access
 *     - no process execution
 *     - no mutable global state
 *     - no unsafe code
 *
 * The generated parser/runtime must be compiled under the repository's
 * unsafe-code prohibition.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Verilog syntax is preserved as source-format syntax.
 *
 * Physical resource realization is deliberately outside this grammar.
 *
 * Therefore this grammar imposes no fixed maximum on:
 *
 *     modules
 *     ports
 *     parameters
 *     nets
 *     registers
 *     memories
 *     instances
 *     hierarchy depth
 *     generated instances
 *     signal widths
 *     expression depth
 *     procedural statements
 *     processes
 *     states
 *     transitions
 *
 * Any practical limit is a parser/compiler/runtime resource policy rather
 * than a language grammar limit.
 *
 * ============================================================================
 * VERSION POLICY
 * ============================================================================
 *
 * This file targets Verilog-2001/2005 syntax.
 *
 * SystemVerilog syntax is intentionally NOT silently mixed into this grammar.
 *
 * SystemVerilog interoperability belongs in a future dedicated grammar,
 * for example:
 *
 *     grammar/interoperability/systemverilog.g4
 *
 * This prevents accidental acceptance of a construct whose semantics are
 * different from IEEE 1364 Verilog.
 *
 * ============================================================================
 */

grammar verilog;

options {
    language = Rust;
}


/* ============================================================================
 * PUBLIC ENTRY POINT
 * ========================================================================== */

source
    : compilerDirective*
      designUnit*
      compilerDirective*
      EOF
    ;


/* ============================================================================
 * DESIGN UNITS
 * ========================================================================== */

designUnit
    : moduleDeclaration
    | primitiveDeclaration
    ;


/* ============================================================================
 * MODULES
 * ========================================================================== */

moduleDeclaration
    : attributeInstance*
      (MODULE | MACROMODULE)
      moduleIdentifier
      moduleParameterPortList?
      modulePortList?
      SEMICOLON
      moduleItem*
      ENDMODULE
    ;

moduleIdentifier
    : IDENTIFIER
    ;

moduleParameterPortList
    : HASH LPAREN parameterPortDeclarationList? RPAREN
    ;

parameterPortDeclarationList
    : parameterPortDeclaration
      (COMMA parameterPortDeclaration)*
      COMMA?
    ;

parameterPortDeclaration
    : parameterDeclaration
    | localParameterDeclaration
    | parameterPortTypeDeclaration
    ;

parameterPortTypeDeclaration
    : dataType
      listOfParameterAssignments
    ;

modulePortList
    : LPAREN
      portDeclarationList?
      RPAREN
    ;

portDeclarationList
    : port
      (COMMA port)*
      COMMA?
    ;

port
    : portExpression
    | ansiPortDeclaration
    ;

portExpression
    : portIdentifier
    | DOT portIdentifier LPAREN portExpression RPAREN
    ;

portIdentifier
    : IDENTIFIER
    ;

ansiPortDeclaration
    : attributeInstance*
      portDirection?
      netPortType?
      variablePortType?
      portIdentifier
      unpackedDimension*
      (ASSIGN expression)?
    ;

portDirection
    : INPUT
    | OUTPUT
    | INOUT
    ;

netPortType
    : netType
      signing?
      packedDimension*
    ;

variablePortType
    : variableDataType
    ;

moduleItem
    : attributeInstance*
      (
          moduleOrGenerateItem
        | portDeclaration
        | nonPortModuleItem
      )
    ;

moduleOrGenerateItem
    : moduleOrGenerateItemDeclaration
    | localParameterDeclaration
    | parameterOverride
    | continuousAssignment
    | gateInstantiation
    | udpInstantiation
    | moduleInstantiation
    | initialConstruct
    | alwaysConstruct
    ;

nonPortModuleItem
    : generateRegion
    | specifyBlock
    | parameterDeclaration
    | specparamDeclaration
    | taskDeclaration
    | functionDeclaration
    ;


/* ============================================================================
 * PORT DECLARATIONS
 * ========================================================================== */

portDeclaration
    : inputDeclaration
    | outputDeclaration
    | inoutDeclaration
    ;

inputDeclaration
    : INPUT
      netOrVariableType?
      listOfPortIdentifiers
      SEMICOLON
    ;

outputDeclaration
    : OUTPUT
      netOrVariableType?
      listOfPortIdentifiers
      SEMICOLON
    ;

inoutDeclaration
    : INOUT
      netType?
      signing?
      packedDimension*
      listOfPortIdentifiers
      SEMICOLON
    ;

listOfPortIdentifiers
    : portIdentifier
      unpackedDimension*
      (COMMA portIdentifier unpackedDimension*)*
    ;

netOrVariableType
    : netPortType
    | variablePortType
    ;


/* ============================================================================
 * DECLARATIONS
 * ========================================================================== */

moduleOrGenerateItemDeclaration
    : netDeclaration
    | regDeclaration
    | integerDeclaration
    | timeDeclaration
    | realDeclaration
    | realtimeDeclaration
    | eventDeclaration
    | genvarDeclaration
    | blockItemDeclaration
    | memoryDeclaration
    ;

blockItemDeclaration
    : regDeclaration
    | integerDeclaration
    | timeDeclaration
    | realDeclaration
    | realtimeDeclaration
    | eventDeclaration
    | parameterDeclaration
    | localParameterDeclaration
    | netDeclaration
    ;

parameterDeclaration
    : PARAMETER
      parameterType?
      listOfParameterAssignments
      SEMICOLON
    ;

localParameterDeclaration
    : LOCALPARAM
      parameterType?
      listOfParameterAssignments
      SEMICOLON
    ;

parameterType
    : signedNumberType
    | integerVectorType
    | realType
    | realtimeType
    | timeType
    ;

listOfParameterAssignments
    : parameterAssignment
      (COMMA parameterAssignment)*
    ;

parameterAssignment
    : parameterIdentifier ASSIGN constantExpression
    ;

parameterIdentifier
    : IDENTIFIER
    ;

specparamDeclaration
    : SPECPARAM
      listOfSpecparamAssignments
      SEMICOLON
    ;

listOfSpecparamAssignments
    : specparamAssignment
      (COMMA specparamAssignment)*
    ;

specparamAssignment
    : IDENTIFIER ASSIGN constantMintypmaxExpression
    ;


/* ============================================================================
 * NETS
 * ========================================================================== */

netDeclaration
    : netType
      driveStrength?
      chargeStrength?
      netDelayValue?
      netDeclaratorList
      SEMICOLON
    ;

netType
    : SUPPLY0
    | SUPPLY1
    | TRI
    | TRI0
    | TRI1
    | TRIAND
    | TRIOR
    | TRIREG
    | WIRE
    | WAND
    | WOR
    | UWIRE
    ;

netDeclaratorList
    : netDeclarator
      (COMMA netDeclarator)*
    ;

netDeclarator
    : netIdentifier
      unpackedDimension*
      (ASSIGN expression)?
    ;

netIdentifier
    : IDENTIFIER
    ;


/* ============================================================================
 * VARIABLES
 * ========================================================================== */

regDeclaration
    : REG
      signing?
      packedDimension*
      listOfVariableAssignments
      SEMICOLON
    ;

integerDeclaration
    : INTEGER
      listOfVariableIdentifiers
      SEMICOLON
    ;

timeDeclaration
    : TIME
      listOfVariableIdentifiers
      SEMICOLON
    ;

realDeclaration
    : REAL
      listOfRealVariables
      SEMICOLON
    ;

realtimeDeclaration
    : REALTIME
      listOfRealVariables
      SEMICOLON
    ;

eventDeclaration
    : EVENT
      listOfEventIdentifiers
      SEMICOLON
    ;

listOfVariableAssignments
    : variableAssignment
      (COMMA variableAssignment)*
    ;

variableAssignment
    : variableIdentifier
      unpackedDimension*
      (ASSIGN expression)?
    ;

variableIdentifier
    : IDENTIFIER
    ;

listOfVariableIdentifiers
    : variableIdentifier
      unpackedDimension*
      (COMMA variableIdentifier unpackedDimension*)*
    ;

listOfRealVariables
    : variableIdentifier
      (COMMA variableIdentifier)*
    ;

listOfEventIdentifiers
    : IDENTIFIER
      (COMMA IDENTIFIER)*
    ;

memoryDeclaration
    : REG
      signing?
      packedDimension+
      memoryVariableDeclarationList
      SEMICOLON
    ;

memoryVariableDeclarationList
    : memoryVariableDeclaration
      (COMMA memoryVariableDeclaration)*
    ;

memoryVariableDeclaration
    : IDENTIFIER
      unpackedDimension+
      (ASSIGN expression)?
    ;


/* ============================================================================
 * DATA TYPES
 * ========================================================================== */

dataType
    : integerVectorType
    | integerAtomType
    | realType
    | realtimeType
    | timeType
    ;

variableDataType
    : integerVectorType
    | integerAtomType
    | realType
    | realtimeType
    | timeType
    ;

integerVectorType
    : (REG | LOGIC_COMPAT?)
      signing?
      packedDimension+
    ;

integerAtomType
    : INTEGER
    | TIME
    ;

realType
    : REAL
    ;

realtimeType
    : REALTIME
    ;

timeType
    : TIME
    ;

signedNumberType
    : INTEGER
    | TIME
    ;

signing
    : SIGNED
    | UNSIGNED
    ;

packedDimension
    : LBRACK constantRangeExpression RBRACK
    ;

unpackedDimension
    : LBRACK constantRangeExpression RBRACK
    | LBRACK constantExpression RBRACK
    ;


/* ============================================================================
 * ASSIGNMENTS
 * ========================================================================== */

continuousAssignment
    : ASSIGN
      driveStrength?
      netDelayValue?
      continuousAssignmentList
      SEMICOLON
    ;

continuousAssignmentList
    : continuousAssignmentItem
      (COMMA continuousAssignmentItem)*
    ;

continuousAssignmentItem
    : netLvalue ASSIGN expression
    ;


/* ============================================================================
 * PARAMETER OVERRIDES
 * ========================================================================== */

parameterOverride
    : DEFPARAM
      listOfDefparamAssignments
      SEMICOLON
    ;

listOfDefparamAssignments
    : defparamAssignment
      (COMMA defparamAssignment)*
    ;

defparamAssignment
    : hierarchicalParameterIdentifier ASSIGN constantExpression
    ;

hierarchicalParameterIdentifier
    : hierarchicalIdentifier
    ;


/* ============================================================================
 * MODULE INSTANTIATION
 * ========================================================================== */

moduleInstantiation
    : moduleIdentifier
      parameterValueAssignment?
      moduleInstanceList
      SEMICOLON
    ;

parameterValueAssignment
    : HASH LPAREN
      (
          expressionList?
        | namedParameterAssignmentList
      )
      RPAREN
    ;

namedParameterAssignmentList
    : namedParameterAssignment
      (COMMA namedParameterAssignment)*
      COMMA?
    ;

namedParameterAssignment
    : DOT parameterIdentifier LPAREN expression RPAREN
    ;

moduleInstanceList
    : moduleInstance
      (COMMA moduleInstance)*
    ;

moduleInstance
    : moduleInstanceIdentifier
      moduleInstanceArrayRange?
      LPAREN
      moduleConnectionList?
      RPAREN
    ;

moduleInstanceIdentifier
    : IDENTIFIER
    ;

moduleInstanceArrayRange
    : unpackedDimension+
    ;

moduleConnectionList
    : orderedModuleConnectionList
    | namedModuleConnectionList
    ;

orderedModuleConnectionList
    : moduleConnection
      (COMMA moduleConnection)*
      COMMA?
    ;

moduleConnection
    : expression?
    ;

namedModuleConnectionList
    : namedModuleConnection
      (COMMA namedModuleConnection)*
      COMMA?
    ;

namedModuleConnection
    : DOT portIdentifier LPAREN expression? RPAREN
    ;


/* ============================================================================
 * GATE PRIMITIVES
 * ========================================================================== */

gateInstantiation
    : gateType
      driveStrength?
      gateDelay?
      gateInstanceList
      SEMICOLON
    ;

gateType
    : AND
    | NAND
    | NOR
    | OR
    | XOR
    | XNOR
    | BUF
    | NOT
    | BUFIF0
    | BUFIF1
    | NOTIF0
    | NOTIF1
    | PULLUP
    | PULLDOWN
    | NMOS
    | PMOS
    | RNMOS
    | RPMOS
    | CMOS
    | RCMOS
    | TRAN
    | RTRAN
    | TRANIF0
    | TRANIF1
    | RTRANIF0
    | RTRANIF1
    ;

gateInstanceList
    : gateInstance
      (COMMA gateInstance)*
    ;

gateInstance
    : gateInstanceIdentifier?
      gateTerminalList
    ;

gateInstanceIdentifier
    : IDENTIFIER
    ;

gateTerminalList
    : LPAREN expressionList RPAREN
    ;


/* ============================================================================
 * UDP INSTANTIATION
 * ========================================================================== */

udpInstantiation
    : udpIdentifier
      driveStrength?
      outputTerminal
      inputTerminalList
      udpInstanceList
      SEMICOLON
    ;

udpIdentifier
    : IDENTIFIER
    ;

outputTerminal
    : LPAREN netLvalue RPAREN
    ;

inputTerminalList
    : LPAREN expressionList RPAREN
    ;

udpInstanceList
    : udpInstance
      (COMMA udpInstance)*
    ;

udpInstance
    : IDENTIFIER
      udpInstanceArrayRange?
    ;

udpInstanceArrayRange
    : unpackedDimension+
    ;


/* ============================================================================
 * PROCEDURAL CONSTRUCTS
 * ========================================================================== */

initialConstruct
    : INITIAL
      statement
    ;

alwaysConstruct
    : ALWAYS
      statement
    ;

statement
    : attributeInstance*
      (
          proceduralAssignmentStatement
        | proceduralContinuousAssignment
        | proceduralTimingControlStatement
        | conditionalStatement
        | caseStatement
        | loopStatement
        | waitStatement
        | eventTriggerStatement
        | blockStatement
        | taskEnableStatement
        | disableStatement
        | proceduralForceStatement
        | proceduralReleaseStatement
        | proceduralDeassignStatement
        | proceduralAssignStatement
        | nullStatement
        | systemTaskEnable
      )
    ;

nullStatement
    : SEMICOLON
    ;

blockStatement
    : namedBlock
    | sequentialBlock
    | parallelBlock
    ;

namedBlock
    : BEGIN COLON IDENTIFIER
      blockItemDeclaration*
      statement*
      END
    ;

sequentialBlock
    : BEGIN
      blockItemDeclaration*
      statement*
      END
    ;

parallelBlock
    : FORK
      blockItemDeclaration*
      statement*
      JOIN
    ;

proceduralAssignmentStatement
    : variableAssignmentOperator
      SEMICOLON
    ;

variableAssignmentOperator
    : blockingAssignment
    | nonblockingAssignment
    ;

blockingAssignment
    : variableLvalue ASSIGN
      expression
    ;

nonblockingAssignment
    : variableLvalue NONBLOCKING_ASSIGN
      timingControl?
      expression
    ;

proceduralContinuousAssignment
    : ASSIGN variableLvalue ASSIGN expression SEMICOLON
    ;

proceduralAssignStatement
    : ASSIGN variableLvalue ASSIGN expression SEMICOLON
    ;

proceduralDeassignStatement
    : DEASSIGN variableLvalue SEMICOLON
    ;

proceduralForceStatement
    : FORCE variableLvalue ASSIGN expression SEMICOLON
    ;

proceduralReleaseStatement
    : RELEASE variableLvalue SEMICOLON
    ;


/* ============================================================================
 * CONTROL FLOW
 * ========================================================================== */

conditionalStatement
    : IF LPAREN expression RPAREN statement
      (ELSE statement)?
    ;

caseStatement
    : CASE LPAREN expression RPAREN caseItem*
      ENDCASE
    | CASEX LPAREN expression RPAREN caseItem*
      ENDCASE
    | CASEZ LPAREN expression RPAREN caseItem*
      ENDCASE
    ;

caseItem
    : expressionList COLON statement
    | DEFAULT
      (COLON statement)?
    ;

loopStatement
    : FOREVER statement
    | REPEAT LPAREN expression RPAREN statement
    | WHILE LPAREN expression RPAREN statement
    | FOR LPAREN
          proceduralAssignmentExpression?
          SEMICOLON
          expression?
          SEMICOLON
          proceduralAssignmentExpression?
      RPAREN
      statement
    ;

proceduralAssignmentExpression
    : variableLvalue ASSIGN expression
    | variableLvalue NONBLOCKING_ASSIGN expression
    ;

waitStatement
    : WAIT LPAREN expression RPAREN statement
    ;

eventTriggerStatement
    : ARROW eventIdentifier SEMICOLON
    ;

eventIdentifier
    : hierarchicalIdentifier
    ;

disableStatement
    : DISABLE hierarchicalIdentifier SEMICOLON
    ;


/* ============================================================================
 * TIMING / EVENT CONTROL
 * ========================================================================== */

proceduralTimingControlStatement
    : timingControl statement
    ;

timingControl
    : delayControl
    | eventControl
    | cycleDelayControl
    ;

delayControl
    : HASH delayValue
    ;

delayValue
    : constantPrimary
    | LPAREN mintypmaxExpression RPAREN
    ;

eventControl
    : AT eventExpression
    | AT ASTERISK
    ;

eventExpression
    : LPAREN eventExpressionList? RPAREN
    | hierarchicalEventIdentifier
    ;

eventExpressionList
    : eventExpressionItem
      (OR eventExpressionItem)*
    ;

eventExpressionItem
    : edgeIdentifier?
      expression
    ;

edgeIdentifier
    : POSEDGE
    | NEGEDGE
    | EDGE
    ;

hierarchicalEventIdentifier
    : hierarchicalIdentifier
    ;

cycleDelayControl
    : HASHHASH constantExpression
    ;


/* ============================================================================
 * WAIT / TASKS / SYSTEM TASKS
 * ========================================================================== */

taskEnableStatement
    : hierarchicalTaskIdentifier
      argumentExpressionList?
      SEMICOLON
    ;

hierarchicalTaskIdentifier
    : hierarchicalIdentifier
    ;

argumentExpressionList
    : LPAREN expressionList? RPAREN
    ;

systemTaskEnable
    : SYSTEM_IDENTIFIER
      argumentExpressionList?
      SEMICOLON
    ;


/* ============================================================================
 * FUNCTIONS
 * ========================================================================== */

functionDeclaration
    : FUNCTION
      functionAutomatic?
      functionReturnType?
      functionIdentifier
      functionPortList?
      SEMICOLON
      blockItemDeclaration*
      statement*
      ENDFUNCTION
    ;

functionAutomatic
    : AUTOMATIC
    ;

functionReturnType
    : integerVectorType
    | integerAtomType
    | realType
    | realtimeType
    | timeType
    ;

functionIdentifier
    : IDENTIFIER
    ;

functionPortList
    : LPAREN functionPortItemList? RPAREN
    ;

functionPortItemList
    : functionPortItem
      (COMMA functionPortItem)*
    ;

functionPortItem
    : inputDeclaration
    | parameterDeclaration
    | functionPortIdentifier
    ;

functionPortIdentifier
    : IDENTIFIER
    ;


/* ============================================================================
 * TASKS
 * ========================================================================== */

taskDeclaration
    : TASK
      taskAutomatic?
      taskIdentifier
      taskPortList?
      SEMICOLON
      blockItemDeclaration*
      statement*
      ENDTASK
    ;

taskAutomatic
    : AUTOMATIC
    ;

taskIdentifier
    : IDENTIFIER
    ;

taskPortList
    : LPAREN taskPortItemList? RPAREN
    ;

taskPortItemList
    : taskPortItem
      (COMMA taskPortItem)*
    ;

taskPortItem
    : inputDeclaration
    | outputDeclaration
    | inoutDeclaration
    | parameterDeclaration
    | IDENTIFIER
    ;


/* ============================================================================
 * GENERATE
 * ========================================================================== */

generateRegion
    : GENERATE
      generateItem*
      ENDGENERATE
    ;

generateItem
    : generateConditional
    | generateLoop
    | generateCase
    | moduleOrGenerateItem
    | blockGenerateItem
    ;

generateConditional
    : IF LPAREN constantExpression RPAREN
      generateBlock
      (ELSE generateBlock)?
    ;

generateLoop
    : FOR LPAREN
      genvarInitialization
      SEMICOLON
      constantExpression
      SEMICOLON
      genvarIteration
      RPAREN
      generateBlock
    ;

genvarInitialization
    : GENVAR? IDENTIFIER ASSIGN constantExpression
    ;

genvarIteration
    : IDENTIFIER ASSIGN constantExpression
    ;

generateCase
    : CASE LPAREN constantExpression RPAREN
      generateCaseItem*
      ENDCASE
    ;

generateCaseItem
    : constantExpressionList COLON generateBlock
    | DEFAULT COLON generateBlock
    ;

generateBlock
    : blockGenerateItem
    | BEGIN
      (COLON IDENTIFIER)?
      generateItem*
      END
    ;

blockGenerateItem
    : moduleOrGenerateItem
    | generateConditional
    | generateLoop
    | generateCase
    ;


/* ============================================================================
 * SPECIFY / TIMING
 * ========================================================================== */

specifyBlock
    : SPECIFY
      specifyItem*
      ENDSPECIFY
    ;

specifyItem
    : specparamDeclaration
    | specifyPathDeclaration
    | systemTimingCheck
    | pulsestyleDeclaration
    | showcancelledDeclaration
    ;

specifyPathDeclaration
    : parallelPathDescription
      pathDelayValue
      SEMICOLON
    | fullPathDescription
      pathDelayValue
      SEMICOLON
    ;

parallelPathDescription
    : LPAREN specifyInputPath RPAREN
      PATH_ARROW
      specifyOutputPath
    ;

fullPathDescription
    : LPAREN specifyInputPath RPAREN
      PATH_FULL_ARROW
      specifyOutputPath
    ;

specifyInputPath
    : specifyTerminalDescriptor
    ;

specifyOutputPath
    : specifyTerminalDescriptor
    ;

specifyTerminalDescriptor
    : IDENTIFIER
      specifyRange?
    ;

specifyRange
    : LBRACK constantExpression COLON constantExpression RBRACK
    ;

pathDelayValue
    : delayValue
    | LPAREN
      mintypmaxExpression
      (
          COMMA mintypmaxExpression
      )*
      RPAREN
    ;

systemTimingCheck
    : SYSTEM_IDENTIFIER
      LPAREN
      specifyTimingArgumentList?
      RPAREN
      SEMICOLON
    ;

specifyTimingArgumentList
    : expression
      (COMMA expression)*
    ;

pulsestyleDeclaration
    : PULSESTYLE_ONEVENT
      pathList
      SEMICOLON
    | PULSESTYLE_ONEVENT
      specifyTerminalList
      SEMICOLON
    ;

showcancelledDeclaration
    : SHOWCANCELLED
      specifyTerminalList
      SEMICOLON
    ;

specifyTerminalList
    : specifyTerminalDescriptor
      (COMMA specifyTerminalDescriptor)*
    ;

pathList
    : specifyTerminalDescriptor
      (COMMA specifyTerminalDescriptor)*
    ;


/* ============================================================================
 * UDP DECLARATIONS
 * ========================================================================== */

primitiveDeclaration
    : attributeInstance*
      PRIMITIVE
      primitiveIdentifier
      primitivePortList
      SEMICOLON
      primitiveDeclarationItem*
      TABLE
      udpTableEntry*
      ENDTABLE
      ENDPRIMITIVE
    ;

primitiveIdentifier
    : IDENTIFIER
    ;

primitivePortList
    : LPAREN
      primitiveOutputPort
      (COMMA primitiveInputPort)*
      RPAREN
    ;

primitiveOutputPort
    : IDENTIFIER
    ;

primitiveInputPort
    : IDENTIFIER
    ;

primitiveDeclarationItem
    : outputDeclaration
    | inputDeclaration
    | regDeclaration
    | parameterDeclaration
    ;

udpTableEntry
    : udpInputList COLON udpOutputList SEMICOLON
    ;

udpInputList
    : udpSymbol+
    ;

udpOutputList
    : udpSymbol+
    ;

udpSymbol
    : ZERO
    | ONE
    | X
    | QUESTION
    | DASH
    | EDGE_SYMBOL
    | LPAREN
      edgeSymbolExpression
      RPAREN
    ;

edgeSymbolExpression
    : udpSymbol
    ;


/* ============================================================================
 * LVALUES
 * ========================================================================== */

netLvalue
    : hierarchicalIdentifier
      bitSelect*
      partSelect*
    ;

variableLvalue
    : hierarchicalIdentifier
      bitSelect*
      partSelect*
    ;

bitSelect
    : LBRACK expression RBRACK
    ;

partSelect
    : LBRACK expression COLON expression RBRACK
    ;

memoryWordSelect
    : LBRACK expression RBRACK
    ;


/* ============================================================================
 * EXPRESSIONS
 * ========================================================================== */

expression
    : conditionalExpression
    ;

conditionalExpression
    : logicalOrExpression
      (QUESTION logicalOrExpression COLON conditionalExpression)?
    ;

logicalOrExpression
    : logicalAndExpression
      (OROR logicalAndExpression)*
    ;

logicalAndExpression
    : bitwiseOrExpression
      (ANDAND bitwiseOrExpression)*
    ;

bitwiseOrExpression
    : bitwiseXorExpression
      (BAR bitwiseXorExpression)*
    ;

bitwiseXorExpression
    : bitwiseAndExpression
      (CARET bitwiseAndExpression)*
    ;

bitwiseAndExpression
    : equalityExpression
      (AMPERSAND equalityExpression)*
    ;

equalityExpression
    : relationalExpression
      (
          EQUAL_EQUAL relationalExpression
        | NOT_EQUAL relationalExpression
        | CASE_EQUAL relationalExpression
        | CASE_NOT_EQUAL relationalExpression
      )*
    ;

relationalExpression
    : shiftExpression
      (
          LESS_THAN shiftExpression
        | LESS_EQUAL shiftExpression
        | GREATER_THAN shiftExpression
        | GREATER_EQUAL shiftExpression
      )*
    ;

shiftExpression
    : additiveExpression
      (
          LEFT_SHIFT additiveExpression
        | RIGHT_SHIFT additiveExpression
        | ARITHMETIC_LEFT_SHIFT additiveExpression
        | ARITHMETIC_RIGHT_SHIFT additiveExpression
      )*
    ;

additiveExpression
    : multiplicativeExpression
      (
          PLUS multiplicativeExpression
        | MINUS multiplicativeExpression
      )*
    ;

multiplicativeExpression
    : unaryExpression
      (
          STAR unaryExpression
        | SLASH unaryExpression
        | PERCENT unaryExpression
      )*
    ;

unaryExpression
    : PLUS unaryExpression
    | MINUS unaryExpression
    | EXCLAMATION unaryExpression
    | TILDE unaryExpression
    | TILDE_AMPERSAND unaryExpression
    | TILDE_BAR unaryExpression
    | TILDE_CARET unaryExpression
    | unaryReductionExpression
    ;

unaryReductionExpression
    : primaryExpression
    ;

primaryExpression
    : primary
      bitSelect*
      partSelect*
    ;

primary
    : NUMBER
    | REAL_NUMBER
    | STRING_LITERAL
    | IDENTIFIER
    | SYSTEM_IDENTIFIER
    | concatenation
    | multipleConcatenation
    | functionCall
    | systemFunctionCall
    | LPAREN expression RPAREN
    ;

functionCall
    : hierarchicalIdentifier
      LPAREN
      expressionList?
      RPAREN
    ;

systemFunctionCall
    : SYSTEM_IDENTIFIER
      LPAREN
      expressionList?
      RPAREN
    ;

concatenation
    : LBRACE
      expressionList
      RBRACE
    ;

multipleConcatenation
    : LBRACE
      expression
      concatenation
      RBRACE
    ;

expressionList
    : expression
      (COMMA expression)*
      COMMA?
    ;

constantExpression
    : expression
    ;

constantExpressionList
    : constantExpression
      (COMMA constantExpression)*
    ;

constantPrimary
    : primary
    ;

constantRangeExpression
    : constantExpression COLON constantExpression
    ;

constantMintypmaxExpression
    : mintypmaxExpression
    ;

mintypmaxExpression
    : expression
      (
          COLON expression
          (
              COLON expression
          )?
      )?
    ;


/* ============================================================================
 * HIERARCHICAL NAMES
 * ========================================================================== */

hierarchicalIdentifier
    : IDENTIFIER
      hierarchySegment*
    ;

hierarchySegment
    : DOT IDENTIFIER
      bitSelect*
    ;

hierarchicalParameterIdentifier
    : hierarchicalIdentifier
    ;


/* ============================================================================
 * DRIVE / CHARGE / DELAY
 * ========================================================================== */

driveStrength
    : LPAREN driveStrengthValue COMMA driveStrengthValue RPAREN
    ;

driveStrengthValue
    : STRONG0
    | STRONG1
    | PULL0
    | PULL1
    | WEAK0
    | WEAK1
    | HIGHZ0
    | HIGHZ1
    ;

chargeStrength
    : SMALL
    | MEDIUM
    | LARGE
    ;

netDelayValue
    : HASH delayValue
    ;

gateDelay
    : HASH delayValue
    ;

delay3
    : delayValue
    | LPAREN
      delayValue COMMA
      delayValue COMMA
      delayValue
      RPAREN
    ;


/* ============================================================================
 * ATTRIBUTES
 * ========================================================================== */

attributeInstance
    : ATTRIBUTE_START
      attributeSpecificationList?
      ATTRIBUTE_END
    ;

attributeSpecificationList
    : attributeSpecification
      (COMMA attributeSpecification)*
      COMMA?
    ;

attributeSpecification
    : IDENTIFIER
      (ASSIGN expression)?
    ;


/* ============================================================================
 * COMPILER DIRECTIVES
 *
 * Known directives are retained structurally.
 *
 * Unknown directives are preserved as opaque source-line directives instead
 * of being rejected by the grammar. This is deliberate interoperability
 * behavior: vendor/preprocessor policy belongs above the grammar.
 * ========================================================================== */

compilerDirective
    : includeDirective
    | timescaleDirective
    | defineDirective
    | undefDirective
    | resetallDirective
    | defaultNettypeDirective
    | genericCompilerDirective
    ;

includeDirective
    : DIRECTIVE_INCLUDE
      STRING_LITERAL
    ;

timescaleDirective
    : DIRECTIVE_TIMESCALE
      timeLiteral
      SLASH
      timeLiteral
    ;

defineDirective
    : DIRECTIVE_DEFINE
      IDENTIFIER
      compilerDirectivePayload*
    ;

undefDirective
    : DIRECTIVE_UNDEF
      IDENTIFIER
    ;

resetallDirective
    : DIRECTIVE_RESETALL
    ;

defaultNettypeDirective
    : DIRECTIVE_DEFAULT_NETTYPE
      IDENTIFIER
    ;

genericCompilerDirective
    : DIRECTIVE_LINE
    ;

compilerDirectivePayload
    : IDENTIFIER
    | NUMBER
    | REAL_NUMBER
    | STRING_LITERAL
    | CHARACTER
    | HASH
    | PLUS
    | MINUS
    | STAR
    | SLASH
    | PERCENT
    | AMPERSAND
    | BAR
    | CARET
    | EXCLAMATION
    | TILDE
    | LPAREN
    | RPAREN
    | LBRACK
    | RBRACK
    | LBRACE
    | RBRACE
    | COMMA
    | DOT
    | COLON
    | QUESTION
    ;


/* ============================================================================
 * TIME LITERALS
 * ========================================================================== */

timeLiteral
    : NUMBER TIME_UNIT?
    | REAL_NUMBER TIME_UNIT?
    ;

TIME_UNIT
    : 's'
    | 'ms'
    | 'us'
    | 'ns'
    | 'ps'
    | 'fs'
    ;


/* ============================================================================
 * LEXER: VERILOG KEYWORDS
 *
 * These are intentionally local to the interoperability grammar.
 *
 * They must not be added to Zamani's core lexical vocabulary merely because
 * Verilog happens to use them.
 * ========================================================================== */

MODULE          : 'module' ;
MACROMODULE     : 'macromodule' ;
ENDMODULE       : 'endmodule' ;

INPUT           : 'input' ;
OUTPUT          : 'output' ;
INOUT           : 'inout' ;

WIRE            : 'wire' ;
WAND            : 'wand' ;
WOR             : 'wor' ;
TRI             : 'tri' ;
TRI0            : 'tri0' ;
TRI1            : 'tri1' ;
TRIAND          : 'triand' ;
TRIOR           : 'trior' ;
TRIREG          : 'trireg' ;
UWIRE           : 'uwire' ;
SUPPLY0         : 'supply0' ;
SUPPLY1         : 'supply1' ;

REG             : 'reg' ;
INTEGER         : 'integer' ;
TIME            : 'time' ;
REAL            : 'real' ;
REALTIME        : 'realtime' ;
EVENT           : 'event' ;

PARAMETER       : 'parameter' ;
LOCALPARAM      : 'localparam' ;
DEFPARAM        : 'defparam' ;
SPECPARAM       : 'specparam' ;

ASSIGN          : 'assign' ;
DEASSIGN        : 'deassign' ;
FORCE           : 'force' ;
RELEASE         : 'release' ;

INITIAL         : 'initial' ;
ALWAYS          : 'always' ;

BEGIN           : 'begin' ;
END             : 'end' ;
FORK            : 'fork' ;
JOIN            : 'join' ;

IF              : 'if' ;
ELSE            : 'else' ;
CASE            : 'case' ;
CASEX           : 'casex' ;
CASEZ           : 'casez' ;
ENDCASE         : 'endcase' ;
DEFAULT         : 'default' ;

FOREVER         : 'forever' ;
REPEAT          : 'repeat' ;
WHILE           : 'while' ;
FOR             : 'for' ;
WAIT            : 'wait' ;

DISABLE         : 'disable' ;

FUNCTION        : 'function' ;
ENDFUNCTION     : 'endfunction' ;
TASK            : 'task' ;
ENDTASK         : 'endtask' ;
AUTOMATIC       : 'automatic' ;

GENERATE        : 'generate' ;
ENDGENERATE     : 'endgenerate' ;
GENVAR          : 'genvar' ;

PRIMITIVE       : 'primitive' ;
ENDPRIMITIVE    : 'endprimitive' ;
TABLE           : 'table' ;
ENDTABLE        : 'endtable' ;

SPECIFY         : 'specify' ;
ENDSPECIFY      : 'endspecify' ;

POSEDGE         : 'posedge' ;
NEGEDGE         : 'negedge' ;
EDGE            : 'edge' ;

AND             : 'and' ;
NAND            : 'nand' ;
NOR             : 'nor' ;
OR              : 'or' ;
XOR             : 'xor' ;
XNOR            : 'xnor' ;
BUF             : 'buf' ;
NOT             : 'not' ;
BUFIF0          : 'bufif0' ;
BUFIF1          : 'bufif1' ;
NOTIF0          : 'notif0' ;
NOTIF1          : 'notif1' ;
PULLUP          : 'pullup' ;
PULLDOWN        : 'pulldown' ;
NMOS            : 'nmos' ;
PMOS            : 'pmos' ;
RNMOS           : 'rnmos' ;
RPMOS           : 'rpmos' ;
CMOS            : 'cmos' ;
RCMOS           : 'rcmos' ;
TRAN            : 'tran' ;
RTRAN           : 'rtran' ;
TRANIF0         : 'tranif0' ;
TRANIF1         : 'tranif1' ;
RTRANIF0        : 'rtranif0' ;
RTRANIF1        : 'rtranif1' ;

SIGNED          : 'signed' ;
UNSIGNED        : 'unsigned' ;

STRONG0         : 'strong0' ;
STRONG1         : 'strong1' ;
PULL0           : 'pull0' ;
PULL1           : 'pull1' ;
WEAK0           : 'weak0' ;
WEAK1           : 'weak1' ;
HIGHZ0          : 'highz0' ;
HIGHZ1          : 'highz1' ;

SMALL           : 'small' ;
MEDIUM          : 'medium' ;
LARGE           : 'large' ;

PULSESTYLE_ONEVENT
                : 'pulsestyle_onevent'
                ;

SHOWCANCELLED   : 'showcancelled' ;

LOGIC_COMPAT    : 'logic' ;


/* ============================================================================
 * LEXER: COMPILER DIRECTIVES
 *
 * Verilog directives begin with `.
 *
 * Known directives receive structured tokens.
 * Unknown directives remain opaque so vendor extensions do not force grammar
 * changes.
 * ========================================================================== */

DIRECTIVE_INCLUDE
    : '`include'
    ;

DIRECTIVE_TIMESCALE
    : '`timescale'
    ;

DIRECTIVE_DEFINE
    : '`define'
    ;

DIRECTIVE_UNDEF
    : '`undef'
    ;

DIRECTIVE_RESETALL
    : '`resetall'
    ;

DIRECTIVE_DEFAULT_NETTYPE
    : '`default_nettype'
    ;

DIRECTIVE_LINE
    : '`'
      ~[\r\n]*
    ;


/* ============================================================================
 * LEXER: IDENTIFIERS
 *
 * IEEE Verilog identifiers use a broad ASCII-compatible form.
 *
 * Escaped identifiers are preserved as their own lexical category.
 * ========================================================================== */

ESCAPED_IDENTIFIER
    : '\\'
      ~[ \t\r\n]+
    ;

IDENTIFIER
    : [a-zA-Z_]
      [a-zA-Z0-9_$]*
    ;

SYSTEM_IDENTIFIER
    : '$'
      [a-zA-Z_]
      [a-zA-Z0-9_$]*
    ;


/* ============================================================================
 * LEXER: NUMERIC LITERALS
 *
 * No magnitude/width limit is imposed by the grammar.
 *
 * Semantic layers determine representability.
 * ========================================================================== */

NUMBER
    : UNSIZED_NUMBER
    | SIZED_NUMBER
    ;

SIZED_NUMBER
    : SIZE
      APOSTROPHE
      SIGNEDNESS?
      BASE
      DIGIT_SEQUENCE
    ;

UNSIZED_NUMBER
    : DIGIT_SEQUENCE
    ;

SIZE
    : DIGIT_SEQUENCE
    ;

SIGNEDNESS
    : [sS]
    ;

BASE
    : [bBoOdDhH]
    ;

DIGIT_SEQUENCE
    : [0-9a-fA-FxXzZ_]+
    ;

REAL_NUMBER
    : DIGIT_SEQUENCE
      DOT
      DIGIT_SEQUENCE
      EXPONENT_PART?
    | DIGIT_SEQUENCE
      EXPONENT_PART
    ;

EXPONENT_PART
    : [eE]
      [+-]?
      DIGIT_SEQUENCE
    ;


/* ============================================================================
 * LEXER: STRINGS / CHARACTERS
 * ========================================================================== */

STRING_LITERAL
    : '"'
      (
          ESCAPED_CHARACTER
        | ~["\\\r\n]
      )*
      '"'
    ;

CHARACTER
    : '\''
      (
          ESCAPED_CHARACTER
        | ~['\\\r\n]
      )
      '\''
    ;

ESCAPED_CHARACTER
    : '\\'
      (
          ['"\\nrtfvab]
        | [0-7] [0-7]? [0-7]?
        | 'x' HEX_DIGIT HEX_DIGIT
      )
    ;


/* ============================================================================
 * LEXER: UDP / FOUR-STATE SYMBOLS
 * ========================================================================== */

ZERO
    : '0'
    ;

ONE
    : '1'
    ;

X
    : 'x'
    ;

QUESTION
    : '?'
    ;

DASH
    : '-'
    ;

EDGE_SYMBOL
    : [rRfFpPnN*]
    ;


/* ============================================================================
 * LEXER: OPERATORS
 *
 * Longer operators precede their prefixes.
 * ========================================================================== */

NONBLOCKING_ASSIGN
    : '<='
    ;

CASE_EQUAL
    : '==='
    ;

CASE_NOT_EQUAL
    : '!=='
    ;

ARITHMETIC_LEFT_SHIFT
    : '<<<'
    ;

ARITHMETIC_RIGHT_SHIFT
    : '>>>'
    ;

LEFT_SHIFT
    : '<<'
    ;

RIGHT_SHIFT
    : '>>'
    ;

OROR
    : '||'
    ;

ANDAND
    : '&&'
    ;

HASHHASH
    : '##'
    ;

PATH_FULL_ARROW
    : '*>'
    ;

PATH_ARROW
    : '=>'
    ;

TILDE_AMPERSAND
    : '~&'
    ;

TILDE_BAR
    : '~|'
    ;

TILDE_CARET
    : '~^'
    ;

PLUS
    : '+'
    ;

MINUS
    : '-'
    ;

STAR
    : '*'
    ;

SLASH
    : '/'
    ;

PERCENT
    : '%'
    ;

AMPERSAND
    : '&'
    ;

BAR
    : '|'
    ;

CARET
    : '^'
    ;

TILDE
    : '~'
    ;

EXCLAMATION
    : '!'
    ;

LESS_THAN
    : '<'
    ;

GREATER_THAN
    : '>'
    ;

EQUAL_EQUAL
    : '=='
    ;

EQUALS
    : '='
    ;

QUESTION_MARK
    : '?'
    ;

HASH
    : '#'
    ;

AT
    : '@'
    ;

ARROW
    : '->'
    ;


/* ============================================================================
 * LEXER: PUNCTUATION
 * ========================================================================== */

LPAREN
    : '('
    ;

RPAREN
    : ')'
    ;

LBRACE
    : '{'
    ;

RBRACE
    : '}'
    ;

LBRACK
    : '['
    ;

RBRACK
    : ']'
    ;

COMMA
    : ','
    ;

DOT
    : '.'
    ;

COLON
    : ':'
    ;

SEMICOLON
    : ';'
    ;

APOSTROPHE
    : '\''
    ;


/* ============================================================================
 * LEXER: COMMENTS / WHITESPACE
 * ========================================================================== */

BLOCK_COMMENT
    : '/*'
      .*?
      '*/'
      -> channel(HIDDEN)
    ;

LINE_COMMENT
    : '//'
      ~[\r\n]*
      -> channel(HIDDEN)
    ;

WS
    : [ \t\r\n\f]+
      -> channel(HIDDEN)
    ;


/* ============================================================================
 * LEXER FRAGMENTS
 * ========================================================================== */

fragment HEX_DIGIT
    : [0-9a-fA-F]
    ;
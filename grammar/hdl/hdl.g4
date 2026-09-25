/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hdl/hdl.g4
 *
 * Status:
 *     Canonical / production HDL parser composition root.
 *
 * Language:
 *     Zamani
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust edition 2021
 *     Zamani-owned Rust MUST remain safe Rust.
 *
 * ============================================================================
 * AUTHORITY
 * ============================================================================
 *
 * This file is the canonical HDL parser composition root.
 *
 * Normative architecture:
 *
 *     grammar/DESIGN.md
 *          |
 *          v
 *     grammar/spec/hdl.md
 *          |
 *          v
 *     grammar/hdl/hdl.g4
 *          |
 *          +--> HDL parser delegates
 *          |
 *          v
 *     grammar/Zamani.g4
 *          |
 *          v
 *     canonical lexer / parser
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     canonical hardware semantic representation / IR
 *          |
 *          +--> optimization
 *          +--> verification
 *          +--> scheduling
 *          +--> synthesis
 *          +--> placement
 *          +--> routing
 *          +--> target lowering
 *
 * This file owns syntax composition.
 *
 * It DOES NOT own:
 *
 *     - lexical definitions;
 *     - a second lexer;
 *     - AST implementation;
 *     - semantic analysis;
 *     - hardware discovery;
 *     - resource allocation;
 *     - physical placement;
 *     - physical routing;
 *     - synthesis implementation;
 *     - timing closure;
 *     - target selection;
 *     - vendor APIs;
 *     - FPGA/ASIC implementation;
 *     - CPU/GPU/QPU selection;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Zamani HDL describes PORTABLE HARDWARE INTENT.
 *
 * It describes:
 *
 *     WHAT
 *     - a hardware module means;
 *     - interfaces exist;
 *     - signals communicate;
 *     - storage is required;
 *     - behavior occurs;
 *     - timing relationships matter;
 *     - protocols apply;
 *     - capabilities are required;
 *     - resources are required;
 *     - correctness properties must hold.
 *
 * It does NOT implicitly describe:
 *
 *     WHICH FPGA
 *     WHICH ASIC
 *     WHICH CPU
 *     WHICH GPU
 *     WHICH QPU
 *     WHICH physical region
 *     WHICH physical pin
 *     WHICH physical register
 *     WHICH physical memory block
 *     WHICH vendor primitive
 *     WHICH routing path
 *     WHICH placement
 *     WHICH fabrication node
 *
 * Those are downstream realization decisions.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar imposes no language-level maximum on:
 *
 *     modules
 *     interfaces
 *     ports
 *     signals
 *     nets
 *     registers
 *     memories
 *     dimensions
 *     states
 *     transitions
 *     pipeline stages
 *     instances
 *     generated instances
 *     processes
 *     clocks
 *     clock domains
 *     widths
 *     array dimensions
 *     topology size
 *     hardware blocks
 *     design hierarchy
 *
 * Repetition is represented by ANTLR repetition operators.
 *
 * Resource exhaustion is an implementation concern, not a language rule.
 *
 * A compiler MAY impose operational safeguards for:
 *
 *     - memory exhaustion;
 *     - parser exhaustion;
 *     - pathological generated designs;
 *     - macro/generate expansion;
 *     - compile-time evaluation;
 *     - denial-of-service protection.
 *
 * Such safeguards MUST NOT become semantic limits of Zamani HDL.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This grammar MUST NOT define universal constants such as:
 *
 *     MAX_MODULES
 *     MAX_PORTS
 *     MAX_SIGNALS
 *     MAX_REGISTERS
 *     MAX_MEMORIES
 *     MAX_STATES
 *     MAX_PIPELINE_STAGES
 *     MAX_WIDTH
 *     MAX_DEVICES
 *     MAX_LUTS
 *     MAX_BRAMS
 *     MAX_DSPS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_NODES
 *
 * It MUST NOT encode assumptions such as:
 *
 *     wire [31:0]
 *     register<32>
 *     memory<64GB>
 *     FPGA_WITH_100000_LUTS
 *
 * unless the quantity is explicitly part of the user's program semantics.
 *
 * A width such as:
 *
 *     width = 32
 *
 * is valid program data.
 *
 * A language rule saying:
 *
 *     widths <= 32
 *
 * is not valid.
 *
 * ============================================================================
 * REQUIREMENT / CAPABILITY / PREFERENCE / REALIZATION
 * ============================================================================
 *
 * HDL source must distinguish:
 *
 *     REQUIREMENT
 *         semantic condition that MUST be satisfied.
 *
 *     CAPABILITY
 *         property a target must provide.
 *
 *     PREFERENCE
 *         optimization guidance that may be ignored if semantics remain valid.
 *
 *     HINT
 *         non-binding implementation guidance.
 *
 *     REALIZATION
 *         target-specific downstream decision.
 *
 * Examples:
 *
 *     requires capability("streaming")
 *     requires memory >= required_memory
 *     requires timing(...)
 *     prefer accelerator(...)
 *     hint(...)
 *
 * Physical realization belongs downstream.
 *
 * ============================================================================
 * CANONICAL LEXER
 * ============================================================================
 *
 * This parser MUST consume the repository's canonical token vocabulary.
 *
 * There MUST NOT be an HDL-specific lexer.
 *
 * The canonical lexer owns:
 *
 *     identifiers
 *     literals
 *     operators
 *     punctuation
 *     keywords
 *     comments
 *     source positions
 *
 * HDL parser rules consume those tokens.
 *
 * If an HDL token is missing from the canonical lexer, that is a lexer
 * conformance issue and MUST NOT be solved by adding a second lexer here.
 *
 * ============================================================================
 * DELEGATE INTEGRATION
 * ============================================================================
 *
 * The following files are subordinate HDL grammar components:
 *
 *     hardware-modules.g4
 *     hardware-generics.g4
 *     hardware-parameters.g4
 *     hardware-interfaces.g4
 *     ports.g4
 *     signals.g4
 *     wires.g4
 *     registers.g4
 *     memories.g4
 *     clocks.g4
 *     timing.g4
 *     processes.g4
 *     combinational.g4
 *     sequential.g4
 *     state_machines.g4
 *     pipelines.g4
 *     hardware-dialects.g4
 *
 * Their responsibility is feature-local syntax.
 *
 * This file owns:
 *
 *     - canonical HDL entry;
 *     - HDL source-unit composition;
 *     - member-category composition;
 *     - cross-domain HDL composition;
 *     - shared HDL syntax that cannot be owned by one delegate;
 *     - integration points.
 *
 * A delegate MUST NOT become an independent HDL root.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every accepted construct MUST map through the repository's domain-neutral
 * frontend AST.
 *
 * The grammar MUST NOT construct:
 *
 *     netlists
 *     vendor primitives
 *     FPGA placement objects
 *     routing graphs
 *     physical memories
 *     physical clocks
 *     QPU objects
 *     QEC objects
 *     HAL objects
 *
 * The semantic pipeline is:
 *
 *     grammar
 *         |
 *         v
 *     domain-neutral AST
 *         |
 *         v
 *     semantic validation
 *         |
 *         v
 *     canonical hardware semantic representation / IR
 *         |
 *         v
 *     downstream realization
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * HDL may describe control, timing, interfaces, memory, and hardware
 * structures surrounding quantum computation.
 *
 * HDL MUST NOT create a competing quantum IR.
 *
 * Where a construct has quantum semantics:
 *
 *     HDL/hybrid source
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     existing quantum::ir
 *
 * Quantum operation meaning remains owned by the quantum subsystem.
 *
 * QEC remains owned by the QEC subsystem.
 *
 * ZQN remains owned by ZQN.
 *
 * Routing remains owned by routing.
 *
 * Scheduling remains owned by scheduling.
 *
 * HAL remains downstream.
 *
 * ============================================================================
 * CROSS-DOMAIN INTEGRATION
 * ============================================================================
 *
 * HDL may compose with:
 *
 *     classical
 *     quantum
 *     hybrid
 *     AI
 *     data
 *     networking
 *     distributed
 *     security
 *     resources
 *     hardware
 *     execution
 *     compile
 *     interoperability
 *
 * Domain semantics remain owned by those domains.
 *
 * HDL describes the hardware-facing structure and intent.
 *
 * ============================================================================
 * TIMING
 * ============================================================================
 *
 * Timing syntax is declarative.
 *
 * The grammar may accept:
 *
 *     period
 *     frequency
 *     duty cycle
 *     phase
 *     latency
 *     throughput
 *     setup intent
 *     hold intent
 *     uncertainty
 *     jitter
 *     clock relationships
 *     timing constraints
 *
 * Timing analysis and closure are downstream.
 *
 * ============================================================================
 * CLOCKS
 * ============================================================================
 *
 * Clock syntax describes source-level clock intent.
 *
 * The grammar does not select:
 *
 *     physical oscillator
 *     PLL
 *     clock tree
 *     clock buffer
 *     physical clock pin
 *
 * Clock-domain analysis is semantic.
 *
 * ============================================================================
 * MEMORY
 * ============================================================================
 *
 * HDL memory declarations describe logical storage.
 *
 * They may express:
 *
 *     element type
 *     dimensions
 *     address domain
 *     read/write intent
 *     synchronization intent
 *     initialization
 *     latency requirements
 *     bandwidth requirements
 *     capabilities
 *
 * They MUST NOT imply a fixed physical memory technology.
 *
 * ============================================================================
 * HARDWARE ARRAYS
 * ============================================================================
 *
 * Repetition may be:
 *
 *     explicit;
 *     parameterized;
 *     generated;
 *     symbolic.
 *
 * Physical replication is downstream.
 *
 * ============================================================================
 * GENERATION
 * ============================================================================
 *
 * Generate constructs may describe parameterized hardware families.
 *
 * Generation semantics MUST be deterministic for a given semantic input.
 *
 * Resource limits for elaboration are implementation policies.
 *
 * ============================================================================
 * PROCEDURAL HDL
 * ============================================================================
 *
 * Process, combinational, and sequential behavior is source syntax only.
 *
 * Semantic analysis determines:
 *
 *     combinational legality
 *     sequential legality
 *     clock relationships
 *     reset behavior
 *     driver conflicts
 *     read/write legality
 *     CDC conditions
 *     timing constraints
 *
 * ============================================================================
 * STATE MACHINES
 * ============================================================================
 *
 * State machines are unbounded in language semantics.
 *
 * The grammar imposes no fixed number of:
 *
 *     states
 *     transitions
 *     actions
 *     guards
 *
 * ============================================================================
 * PIPELINES
 * ============================================================================
 *
 * Pipelines describe semantic stage relationships.
 *
 * The grammar imposes no fixed pipeline depth.
 *
 * Physical retiming, balancing, placement, and scheduling are downstream.
 *
 * ============================================================================
 * VERIFICATION
 * ============================================================================
 *
 * HDL may express:
 *
 *     assert
 *     assume
 *     cover
 *
 * and other repository-supported verification constructs.
 *
 * Verification semantics remain downstream.
 *
 * ============================================================================
 * DIALECTS
 * ============================================================================
 *
 * Hardware dialects are open-world.
 *
 * The core grammar MUST NOT enumerate every future vendor, accelerator,
 * fabrication technology, protocol, or hardware family.
 *
 * Dialect identity is data.
 *
 * Dialect compatibility is semantic.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * The parser MUST preserve source locations through the normal ANTLR parse
 * tree/token stream.
 *
 * Semantic diagnostics MUST be able to point to:
 *
 *     module
 *     port
 *     signal
 *     register
 *     memory
 *     process
 *     timing constraint
 *     state
 *     transition
 *     pipeline stage
 *     instance
 *     generate construct
 *
 * ============================================================================
 * SAFE RUST
 * ============================================================================
 *
 * This grammar contains no embedded Rust actions or predicates.
 *
 * Rust implementations consuming this grammar MUST:
 *
 *     - target Rust 1.97 / 1.97.1;
 *     - use Rust 2021;
 *     - use safe Rust;
 *     - contain no `unsafe`;
 *     - avoid target-size constants;
 *     - preserve source spans;
 *     - use structured diagnostics;
 *     - remain deterministic where required.
 *
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * hdlDesign is the canonical standalone HDL entry rule.
 *
 * grammar/Zamani.g4 MUST invoke the HDL domain through this boundary rather
 * than duplicating the HDL grammar.
 *
 * ============================================================================
 */

/*
 * ============================================================================
 * ROOT
 * ============================================================================
 */

hdlDesign
    : hdlSourceUnit* EOF
    ;

hdlSourceUnit
    : hdlModuleDeclaration
    | hdlInterfaceDeclaration
    | hdlPackageDeclaration
    | hdlDialectDeclaration
    ;

/*
 * ============================================================================
 * MODULE
 *
 * The detailed module syntax is intentionally delegated to the existing
 * hardware-modules grammar.
 *
 * These rules remain the stable composition contract.
 * ============================================================================
 */

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
    : hdlAttribute*
      hdlModuleMemberCore
    ;

hdlModuleMemberCore
    : hdlParameterDeclaration
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
    ;

/*
 * ============================================================================
 * GENERICS
 * ============================================================================
 */

hdlGenericParameterBlock
    : LESS_THAN
      hdlGenericParameterList
      GREATER_THAN
    ;

hdlGenericParameterList
    : hdlGenericParameter
      (COMMA hdlGenericParameter)*
      COMMA?
    ;

hdlGenericParameter
    : identifier
      (
          COLON hdlTypeExpression
      )?
      (
          ASSIGN hdlExpression
      )?
      hdlConstraintClause*
    ;

hdlConstraintClause
    : K_REQUIRES hdlExpression
    | K_WHERE hdlExpression
    ;

/*
 * ============================================================================
 * PARAMETERS
 * ============================================================================
 */

hdlParameterDeclaration
    : K_PARAMETER
      identifier
      (
          COLON hdlTypeExpression
      )?
      (
          ASSIGN hdlExpression
      )?
      SEMICOLON
    ;

hdlLocalParameterDeclaration
    : K_LOCALPARAM
      identifier
      (
          COLON hdlTypeExpression
      )?
      ASSIGN
      hdlExpression
      SEMICOLON
    ;

/*
 * ============================================================================
 * PORTS
 *
 * Logical ports only. Physical pins belong downstream.
 * ============================================================================
 */

hdlPortBlock
    : LPAREN
      hdlPortDeclarationList?
      RPAREN
    ;

hdlPortDeclarationList
    : hdlPortDeclaration
      (COMMA hdlPortDeclaration)*
      COMMA?
    ;

hdlPortDeclaration
    : hdlAttribute*
      hdlPortDirection?
      hdlPortModifier*
      identifier
      (
          COLON hdlTypeExpression
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
    : LBRACKET hdlRangeExpression RBRACKET
    | hdlAttribute
    ;

hdlPortDeclarationStatement
    : hdlPortDirection
      hdlPortDeclarationList
      SEMICOLON
    ;

/*
 * ============================================================================
 * INTERFACES
 * ============================================================================
 */

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
    : hdlAttribute*
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
          COLON hdlTypeExpression
      )?
      SEMICOLON
    ;

hdlInterfaceSignal
    : K_SIGNAL
      identifier
      (
          COLON hdlTypeExpression
      )?
      SEMICOLON
    ;

hdlInterfaceParameter
    : K_PARAMETER
      identifier
      (
          COLON hdlTypeExpression
      )?
      (
          ASSIGN hdlExpression
      )?
      SEMICOLON
    ;

/*
 * ============================================================================
 * ATTRIBUTES
 * ============================================================================
 */

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
      (COMMA hdlExpression)*
      COMMA?
    ;

/*
 * ============================================================================
 * TYPES
 *
 * Widths and dimensions are expressions, not fixed constants.
 * ============================================================================
 */

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
    | LPAREN hdlTypeExpression RPAREN
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
      (COMMA hdlTypeArgument)*
      COMMA?
    ;

hdlTypeArgument
    : hdlTypeExpression
    | hdlExpression
    ;

/*
 * ============================================================================
 * SIGNALS
 * ============================================================================
 */

hdlSignalDeclaration
    : K_SIGNAL
      hdlSignalDeclaratorList
      SEMICOLON
    ;

hdlSignalDeclaratorList
    : hdlSignalDeclarator
      (COMMA hdlSignalDeclarator)*
      COMMA?
    ;

hdlSignalDeclarator
    : identifier
      (
          COLON hdlTypeExpression
      )?
      (
          ASSIGN hdlExpression
      )?
    ;

/*
 * ============================================================================
 * NETS / WIRES
 * ============================================================================
 */

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
      (COMMA hdlNetDeclarator)*
      COMMA?
    ;

hdlNetDeclarator
    : identifier
      (
          COLON hdlTypeExpression
      )?
    ;

/*
 * ============================================================================
 * REGISTERS
 * ============================================================================
 */

hdlRegisterDeclaration
    : K_REGISTER
      identifier
      (
          COLON hdlTypeExpression
      )?
      hdlRegisterProperty*
      (
          ASSIGN hdlExpression
      )?
      SEMICOLON
    ;

hdlRegisterProperty
    : hdlResetClause
    | hdlClockReference
    | hdlAttribute
    ;

/*
 * ============================================================================
 * MEMORY
 *
 * Dimensions are semantic expressions.
 * No physical memory technology is selected here.
 * ============================================================================
 */

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
    : LBRACKET hdlRangeExpression RBRACKET
    ;

hdlMemoryProperty
    : K_READ hdlMemoryAccessMode?
    | K_WRITE hdlMemoryAccessMode?
    | hdlAttribute
    ;

hdlMemoryAccessMode
    : K_SYNC
    | K_ASYNC
    ;

/*
 * ============================================================================
 * CLOCKS / RESETS
 * ============================================================================
 */

hdlClockDeclaration
    : K_CLOCK
      identifier
      (
          COLON hdlTypeExpression
      )?
      hdlClockProperty*
      SEMICOLON
    ;

hdlClockProperty
    : K_FREQUENCY ASSIGN hdlExpression
    | K_DUTY ASSIGN hdlExpression
    | K_PHASE ASSIGN hdlExpression
    | hdlAttribute
    ;

hdlResetDeclaration
    : K_RESET
      identifier
      (
          COLON hdlTypeExpression
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
          LPAREN hdlExpression RPAREN
      )?
    ;

hdlClockReference
    : K_CLOCK
      LPAREN
      hdlExpression
      RPAREN
    ;

/*
 * ============================================================================
 * PROCESSES
 * ============================================================================
 */

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
      (COMMA hdlSensitivityItem)*
      COMMA?
    ;

hdlSensitivityItem
    : K_POSEDGE hdlExpression
    | K_NEGEDGE hdlExpression
    | K_RISING hdlExpression
    | K_FALLING hdlExpression
    | hdlExpression
    ;

/*
 * ============================================================================
 * PROCEDURAL BLOCK
 * ============================================================================
 */

hdlBlock
    : LBRACE
      hdlBlockItem*
      RBRACE
    ;

hdlBlockItem
    : hdlAttribute*
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

/*
 * ============================================================================
 * LOCAL VARIABLES
 * ============================================================================
 */

hdlVariableDeclaration
    : hdlVariableKind
      identifier
      (
          COLON hdlTypeExpression
      )?
      (
          ASSIGN hdlExpression
      )?
      SEMICOLON
    ;

hdlVariableKind
    : K_LET
    | K_VAR
    | K_CONST
    ;

/*
 * ============================================================================
 * ASSIGNMENTS
 * ============================================================================
 */

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

/*
 * ============================================================================
 * CONTROL FLOW
 * ============================================================================
 */

hdlIfStatement
    : K_IF
      hdlExpression
      hdlBlock
      (
          K_ELSE
          (
              K_IF hdlExpression hdlBlock
            | hdlBlock
          )
      )*
    ;

hdlCaseStatement
    : K_CASE
      LPAREN hdlExpression RPAREN
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
      (COMMA hdlExpression)*
    ;

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
          COLON hdlTypeExpression
      )?
      (
          ASSIGN hdlExpression
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

/*
 * ============================================================================
 * GENERATION
 * ============================================================================
 */

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
    : identifier?
      hdlBlock
    ;

/*
 * ============================================================================
 * INSTANCES
 * ============================================================================
 */

hdlInstanceDeclaration
    : K_INSTANCE
      hdlQualifiedName
      identifier
      hdlGenericArgumentBlock?
      hdlConnectionBlock?
      SEMICOLON
    ;

hdlGenericArgumentBlock
    : LESS_THAN
      hdlGenericArgumentList
      GREATER_THAN
    ;

hdlGenericArgumentList
    : hdlGenericArgument
      (COMMA hdlGenericArgument)*
      COMMA?
    ;

hdlGenericArgument
    : identifier
      COLON
      hdlExpression
    | hdlExpression
    ;

hdlConnectionBlock
    : LPAREN
      hdlConnectionList?
      RPAREN
    ;

hdlConnectionList
    : hdlConnection
      (COMMA hdlConnection)*
      COMMA?
    ;

hdlConnection
    : identifier
      COLON
      hdlExpression
    | hdlExpression
    ;

/*
 * ============================================================================
 * TIMING
 * ============================================================================
 */

hdlTimingDeclaration
    : K_TIMING
      identifier?
      hdlTimingBody
    ;

hdlTimingBody
    : LBRACE
      hdlTimingItem*
      RBRACE
    ;

hdlTimingItem
    : K_LATENCY ASSIGN hdlExpression SEMICOLON
    | K_THROUGHPUT ASSIGN hdlExpression SEMICOLON
    | K_FREQUENCY ASSIGN hdlExpression SEMICOLON
    | K_DUTY ASSIGN hdlExpression SEMICOLON
    | K_PHASE ASSIGN hdlExpression SEMICOLON
    | hdlAttribute
    ;

/*
 * ============================================================================
 * ASSERTIONS
 * ============================================================================
 */

hdlAssertion
    : hdlAssertionKind
      hdlExpression
      SEMICOLON
    ;

hdlAssertionKind
    : K_ASSERT
    | K_ASSUME
    | K_COVER
    ;

/*
 * ============================================================================
 * STATE MACHINES
 * ============================================================================
 */

hdlStateMachineDeclaration
    : K_STATE
      K_MACHINE?
      identifier
      hdlStateMachineBody
    ;

hdlStateMachineBody
    : LBRACE
      hdlStateDeclaration*
      hdlTransitionDeclaration*
      RBRACE
    ;

hdlStateDeclaration
    : K_STATE
      identifier
      hdlStateProperty*
      SEMICOLON
    ;

hdlStateProperty
    : K_INITIAL
    | K_DEFAULT
    | K_ENTRY hdlBlock
    | K_EXIT hdlBlock
    | hdlAttribute
    ;

hdlTransitionDeclaration
    : K_TRANSITION
      identifier?
      hdlTransitionSource?
      hdlTransitionArrow
      hdlTransitionTarget
      hdlTransitionGuard?
      hdlTransitionAction?
      SEMICOLON
    ;

hdlTransitionSource
    : identifier
    ;

hdlTransitionTarget
    : identifier
    ;

hdlTransitionArrow
    : ARROW
    | FAT_ARROW
    ;

hdlTransitionGuard
    : K_WHEN
      hdlExpression
    ;

hdlTransitionAction
    : K_DO
      hdlBlock
    ;

/*
 * ============================================================================
 * PIPELINES
 * ============================================================================
 */

hdlPipelineDeclaration
    : K_PIPELINE
      identifier
      hdlPipelineParameterBlock?
      hdlPipelineBody
    ;

hdlPipelineParameterBlock
    : LPAREN
      hdlPipelineParameterList?
      RPAREN
    ;

hdlPipelineParameterList
    : hdlPipelineParameter
      (COMMA hdlPipelineParameter)*
      COMMA?
    ;

hdlPipelineParameter
    : identifier
      (
          COLON hdlTypeExpression
      )?
      (
          ASSIGN hdlExpression
      )?
    ;

hdlPipelineBody
    : LBRACE
      hdlPipelineItem*
      RBRACE
    ;

hdlPipelineItem
    : K_STAGE
      identifier
      hdlPipelineStageProperty*
      hdlBlock?
      SEMICOLON?
    | hdlAttribute
    ;

hdlPipelineStageProperty
    : K_LATENCY ASSIGN hdlExpression
    | K_THROUGHPUT ASSIGN hdlExpression
    | K_REQUIRES hdlExpression
    | K_WHERE hdlExpression
    | hdlAttribute
    ;

/*
 * ============================================================================
 * PACKAGE / DIALECT
 * ============================================================================
 *
 * These are intentionally lightweight composition boundaries.
 * Their complete semantics belong to the corresponding repository domains.
 * ============================================================================
 */

hdlPackageDeclaration
    : K_PACKAGE
      hdlQualifiedName
      hdlPackageBody
    ;

hdlPackageBody
    : LBRACE
      hdlPackageMember*
      RBRACE
    ;

hdlPackageMember
    : hdlAttribute*
      (
          hdlTypeDeclaration
        | hdlParameterDeclaration
        | hdlInterfaceDeclaration
        | hdlModuleDeclaration
      )
    ;

hdlDialectDeclaration
    : K_DIALECT
      hdlQualifiedName
      hdlDialectBody?
      SEMICOLON?
    ;

hdlDialectBody
    : LBRACE
      hdlDialectMember*
      RBRACE
    ;

hdlDialectMember
    : hdlAttribute
    | hdlDialectProperty
    ;

hdlDialectProperty
    : identifier
      (
          COLON hdlExpression
      )?
      SEMICOLON
    ;

/*
 * ============================================================================
 * EXPRESSIONS
 *
 * HDL expressions intentionally remain open-ended.
 *
 * The repository's canonical expression/type grammar should ultimately own
 * general expression semantics. These local rules are retained as the HDL
 * composition boundary until the canonical expression dispatcher is invoked
 * by grammar/Zamani.g4.
 * ============================================================================
 */

hdlExpression
    : hdlAssignmentExpression
    ;

hdlAssignmentExpression
    : hdlConditionalExpression
      (
          hdlAssignmentOperator
          hdlAssignmentExpression
      )?
    ;

hdlConditionalExpression
    : hdlLogicalOrExpression
      (
          QUESTION
          hdlExpression
          COLON
          hdlExpression
      )?
    ;

hdlLogicalOrExpression
    : hdlLogicalAndExpression
      (LOGICAL_OR hdlLogicalAndExpression)*
    ;

hdlLogicalAndExpression
    : hdlBitwiseOrExpression
      (LOGICAL_AND hdlBitwiseOrExpression)*
    ;

hdlBitwiseOrExpression
    : hdlBitwiseXorExpression
      (PIPE hdlBitwiseXorExpression)*
    ;

hdlBitwiseXorExpression
    : hdlBitwiseAndExpression
      (CARET hdlBitwiseAndExpression)*
    ;

hdlBitwiseAndExpression
    : hdlEqualityExpression
      (AMP hdlEqualityExpression)*
    ;

hdlEqualityExpression
    : hdlRelationalExpression
      (
          (
              EQUAL
            | NOT_EQUAL
          )
          hdlRelationalExpression
      )*
    ;

hdlRelationalExpression
    : hdlShiftExpression
      (
          (
              LESS_THAN
            | LESS_EQUAL
            | GREATER_THAN
            | GREATER_EQUAL
          )
          hdlShiftExpression
      )*
    ;

hdlShiftExpression
    : hdlAdditiveExpression
      (
          (
              SHIFT_LEFT
            | SHIFT_RIGHT
          )
          hdlAdditiveExpression
      )*
    ;

hdlAdditiveExpression
    : hdlMultiplicativeExpression
      (
          (
              PLUS
            | MINUS
          )
          hdlMultiplicativeExpression
      )*
    ;

hdlMultiplicativeExpression
    : hdlUnaryExpression
      (
          (
              STAR
            | SLASH
            | PERCENT
          )
          hdlUnaryExpression
      )*
    ;

hdlUnaryExpression
    : (
          PLUS
        | MINUS
        | BANG
        | TILDE
      )
      hdlUnaryExpression
    | hdlPostfixExpression
    ;

hdlPostfixExpression
    : hdlPrimaryExpression
      hdlPostfixSuffix*
    ;

hdlPostfixSuffix
    : LPAREN
      hdlArgumentList?
      RPAREN
    | LBRACKET
      hdlExpression
      RBRACKET
    | DOT
      identifier
    ;

hdlPrimaryExpression
    : identifier
    | hdlQualifiedName
    | integerLiteral
    | floatingLiteral
    | stringLiteral
    | characterLiteral
    | booleanLiteral
    | LPAREN hdlExpression RPAREN
    | hdlArrayLiteral
    ;

hdlArrayLiteral
    : LBRACKET
      (
          hdlExpression
          (COMMA hdlExpression)*
          COMMA?
      )?
      RBRACKET
    ;

hdlExpressionStatement
    : hdlExpression
      SEMICOLON
    ;

/*
 * ============================================================================
 * NAMES
 * ============================================================================
 */

hdlQualifiedName
    : identifier
      (
          DOT identifier
      )*
    ;

identifier
    : IDENTIFIER
    ;

integerLiteral
    : INTEGER_LITERAL
    ;

floatingLiteral
    : FLOAT_LITERAL
    ;

stringLiteral
    : STRING_LITERAL
    ;

characterLiteral
    : CHARACTER_LITERAL
    ;

booleanLiteral
    : TRUE
    | FALSE
    ;

/*
 * ============================================================================
 * INDEX / RANGE
 * ============================================================================
 */

hdlIndexSuffix
    : LBRACKET
      hdlExpression
      RBRACKET
    ;

hdlRangeExpression
    : hdlRangeEndpoint
      (
          RANGE
          hdlRangeEndpoint
      )?
    ;

hdlRangeEndpoint
    : hdlExpression
    ;

/*
 * ============================================================================
 * END
 * ============================================================================
 *
 * No lexer rules appear in this file.
 *
 * No semantic actions appear in this file.
 *
 * No unsafe implementation exists in this file.
 *
 * No hardware-size limits exist in this file.
 *
 * No vendor-specific hardware is encoded in this file.
 *
 * No physical placement is encoded in this file.
 *
 * No competing quantum IR is encoded in this file.
 *
 * ============================================================================
 */
/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/hdl/hdl.g4
 *
 * STATUS
 * ------
 * CANONICAL PRODUCTION HDL COMPOSITION ROOT
 *
 * LANGUAGE
 * --------
 * Zamani
 *
 * GRAMMAR TECHNOLOGY
 * ------------------
 * ANTLR4 parser grammar
 *
 * RUST BASELINE
 * -------------
 * Rust 1.97 / Rust 1.97.1
 * Rust edition 2021
 *
 * SAFETY
 * ------
 * This grammar contains:
 *
 *   - no embedded Rust;
 *   - no semantic actions;
 *   - no semantic predicates;
 *   - no unsafe implementation;
 *   - no hardware discovery;
 *   - no filesystem access;
 *   - no network access;
 *   - no runtime execution.
 *
 * The Rust implementation consuming this grammar MUST remain safe Rust.
 *
 * ============================================================================
 * AUTHORITY
 * ============================================================================
 *
 * Normative architecture:
 *
 *     grammar/DESIGN.md
 *          |
 *          v
 *     grammar/specification/
 *          |
 *          v
 *     grammar/spec/hdl.md
 *          |
 *          v
 *     grammar/hdl/hdl.g4
 *          |
 *          +------------------------------------------------------+
 *          |                                                      |
 *          v                                                      v
 *     HDL subordinate grammars                            canonical lexer
 *          |                                                      |
 *          +--------------------------+---------------------------+
 *                                     |
 *                                     v
 *                              ZamaniParser
 *                                     |
 *                                     v
 *                              domain-neutral AST
 *                                     |
 *                                     v
 *                              semantic analysis
 *                                     |
 *                                     v
 *                         canonical hardware semantic model
 *                                     |
 *                                     v
 *                                canonical IR
 *                                     |
 *                +--------------------+---------------------+
 *                |                    |                     |
 *                v                    v                     v
 *             verify              optimize              schedule
 *                |                    |                     |
 *                +--------------------+---------------------+
 *                                     |
 *                                     v
 *                               placement/routing
 *                                     |
 *                                     v
 *                                  synthesis
 *                                     |
 *                                     v
 *                              target realization
 *
 * ============================================================================
 * SINGLE HDL AUTHORITY
 * ============================================================================
 *
 * This file is the ONE HDL COMPOSITION ROOT.
 *
 * It owns:
 *
 *   - HDL parser grammar identity;
 *   - HDL parser imports;
 *   - HDL source-unit composition;
 *   - HDL declaration dispatch;
 *   - HDL statement dispatch;
 *   - HDL expression integration;
 *   - HDL cross-domain integration;
 *   - the public standalone HDL entry point;
 *   - stable integration boundaries for ZamaniParser.
 *
 * It DOES NOT own the implementation of:
 *
 *   - modules;
 *   - generics;
 *   - parameters;
 *   - interfaces;
 *   - ports;
 *   - signals;
 *   - wires;
 *   - registers;
 *   - memories;
 *   - clocks;
 *   - timing;
 *   - processes;
 *   - combinational logic;
 *   - sequential logic;
 *   - state machines;
 *   - pipelines;
 *   - generation;
 *   - verification;
 *   - hardware resources;
 *   - capabilities;
 *   - target selection;
 *   - placement;
 *   - routing;
 *   - synthesis;
 *   - vendor primitives;
 *   - physical pins;
 *   - physical memories;
 *   - physical clock trees;
 *   - quantum::ir;
 *   - QEC;
 *   - ZQN;
 *   - HAL;
 *   - runtime execution.
 *
 * Those responsibilities belong to their owning subsystem.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Zamani HDL expresses PORTABLE HARDWARE INTENT.
 *
 * A source program describes:
 *
 *   WHAT:
 *
 *     - hardware structure means;
 *     - interfaces exist;
 *     - signals communicate;
 *     - storage exists;
 *     - computation behaves;
 *     - timing relationships exist;
 *     - protocols apply;
 *     - verification properties hold;
 *     - capabilities are required;
 *     - resources are required;
 *     - implementation preferences exist.
 *
 * It does not inherently describe:
 *
 *   WHICH:
 *
 *     - FPGA;
 *     - ASIC;
 *     - CPU;
 *     - GPU;
 *     - QPU;
 *     - accelerator;
 *     - board;
 *     - package;
 *     - physical pin;
 *     - physical register;
 *     - physical memory block;
 *     - routing path;
 *     - clock tree;
 *     - fabrication node;
 *     - vendor primitive.
 *
 * Those are downstream realization decisions.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This composition root imposes NO language-level maximum on:
 *
 *   - modules;
 *   - interfaces;
 *   - ports;
 *   - signals;
 *   - nets;
 *   - registers;
 *   - memories;
 *   - dimensions;
 *   - states;
 *   - transitions;
 *   - pipeline stages;
 *   - instances;
 *   - generated instances;
 *   - processes;
 *   - clock domains;
 *   - timing constraints;
 *   - hierarchy depth;
 *   - generic parameters;
 *   - generic arguments;
 *   - design elements;
 *   - source-unit elements.
 *
 * Repetition is represented by ANTLR repetition operators and by the
 * subordinate grammars.
 *
 * "Infinity" means:
 *
 *     NO ARTIFICIAL LANGUAGE-LEVEL HARDWARE CEILING.
 *
 * It does NOT mean:
 *
 *     infinite physical memory;
 *     infinite compile time;
 *     infinite synthesis capacity;
 *     infinite target resources.
 *
 * Practical resource exhaustion belongs to:
 *
 *     compiler policy
 *     semantic analysis
 *     elaboration
 *     synthesis
 *     scheduling
 *     routing
 *     runtime
 *     deployment
 *     target capabilities
 *
 * and MUST NOT be converted into language grammar limits.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This file MUST NOT define:
 *
 *     MAX_MODULES
 *     MAX_PORTS
 *     MAX_SIGNALS
 *     MAX_NETS
 *     MAX_REGISTERS
 *     MAX_MEMORIES
 *     MAX_STATES
 *     MAX_TRANSITIONS
 *     MAX_PIPELINE_STAGES
 *     MAX_INSTANCES
 *     MAX_WIDTH
 *     MAX_LANES
 *     MAX_CLOCKS
 *     MAX_DEVICES
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_GPUS
 *     MAX_CPUS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_MEMORY
 *
 * It MUST NOT encode:
 *
 *     wire [31:0]
 *     register<32>
 *     memory<64GB>
 *     FPGA_WITH_100000_LUTS
 *     DEVICE_COUNT == 8
 *
 * as language-wide restrictions.
 *
 * A source program may legitimately contain:
 *
 *     width = 32
 *
 *     depth = 1024
 *
 *     lanes = 8
 *
 * because those are program values.
 *
 * What is forbidden is making those values universal language limits.
 *
 * ============================================================================
 * REQUIREMENT / CAPABILITY / PREFERENCE / HINT / REALIZATION
 * ============================================================================
 *
 * HDL participates in the universal Zamani resource model.
 *
 * The semantic distinction is:
 *
 *     REQUIREMENT
 *         A condition required for successful realization.
 *
 *     CAPABILITY
 *         A capability a target must provide.
 *
 *     PREFERENCE
 *         Non-binding optimization guidance.
 *
 *     HINT
 *         Non-binding implementation information.
 *
 *     REALIZATION
 *         A downstream target-specific implementation decision.
 *
 * Examples:
 *
 *     requires capability("streaming")
 *     requires capability("hardware.pipeline")
 *     requires memory >= required_memory
 *     prefer accelerator("compute")
 *     hint(...)
 *
 * This file merely composes syntax.
 *
 * Whether a target satisfies a requirement belongs downstream.
 *
 * ============================================================================
 * CANONICAL LEXER
 * ============================================================================
 *
 * HDL MUST consume:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * through:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * There MUST NOT be an HDL-specific lexer.
 *
 * This is especially important because the existing repository has already
 * established one canonical lexer boundary.
 *
 * Hardware domain names remain ordinary lexical names unless explicitly
 * reserved by the language specification.
 *
 * For example:
 *
 *     cpu
 *     gpu
 *     fpga
 *     qpu
 *     accelerator
 *     node
 *     device
 *
 * are semantic names, not automatic physical allocations.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every accepted HDL construct MUST enter the domain-neutral frontend AST.
 *
 * The parser MUST NOT create:
 *
 *     FpgaNode
 *     GpuNode
 *     CpuNode
 *     QpuNode
 *     VendorPrimitiveNode
 *     PhysicalPinNode
 *     PhysicalMemoryNode
 *     RoutingNode
 *     PlacementNode
 *     ClockTreeNode
 *     NetlistNode
 *
 * The expected pipeline is:
 *
 *     HDL syntax
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic validation
 *          |
 *          v
 *     canonical hardware semantic model / IR
 *          |
 *          v
 *     target-independent optimization
 *          |
 *          v
 *     target realization
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * HDL may surround or interact with quantum computation.
 *
 * HDL MUST NOT create a second quantum IR.
 *
 * Quantum meaning remains:
 *
 *     Zamani source
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic quantum model
 *          |
 *          v
 *     quantum::ir
 *
 * HDL may provide:
 *
 *     - control structures;
 *     - interfaces;
 *     - timing intent;
 *     - memory;
 *     - hardware boundaries;
 *     - accelerator structure;
 *     - classical control around quantum computation.
 *
 * Quantum operation semantics remain owned by the quantum subsystem.
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
 *     hardware
 *     resources
 *     memory
 *     concurrency
 *     distributed
 *     AI
 *     data
 *     networking
 *     security
 *     compile
 *     execution
 *     interoperability
 *     dialects
 *
 * This grammar does not duplicate those domains.
 *
 * It provides integration points through the canonical Zamani parser.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing MUST depend only on:
 *
 *     - source text;
 *     - canonical lexical vocabulary;
 *     - grammar version;
 *     - explicitly selected dialect configuration.
 *
 * Parsing MUST NOT depend on:
 *
 *     - CPU count;
 *     - GPU availability;
 *     - FPGA availability;
 *     - QPU availability;
 *     - target topology;
 *     - filesystem state;
 *     - network state;
 *     - environment variables;
 *     - wall-clock time;
 *     - randomness;
 *     - runtime state.
 *
 * ============================================================================
 * SAFE RUST
 * ============================================================================
 *
 * This file contains no Rust code.
 *
 * The Rust frontend generated/consuming this grammar MUST target:
 *
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * and MUST remain safe Rust.
 *
 * Recommended crate-level enforcement:
 *
 *     #![deny(unsafe_code)]
 *
 * No HDL grammar construct requires unsafe Rust.
 *
 * ============================================================================
 * ANTLR COMPOSITION
 * ============================================================================
 *
 * The subordinate grammars are the actual feature owners.
 *
 * The architecture is:
 *
 *     HDL
 *       |
 *       +--> HardwareModules
 *       +--> HardwareGenerics
 *       +--> Ports
 *       +--> Signals
 *       +--> Wires
 *       +--> Registers
 *       +--> Memories
 *       +--> Clocks
 *       +--> Timing
 *       +--> Processes
 *       +--> Combinational
 *       +--> Sequential
 *       +--> StateMachines
 *       +--> Pipelines
 *       +--> Generate
 *       +--> HDL dialect/extension grammars
 *
 * This composition root MUST NOT reproduce those grammars.
 *
 * ============================================================================
 */

parser grammar HDL;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * CANONICAL HDL SUB-GRAMMAR IMPORTS
 * ============================================================================
 *
 * These imports intentionally follow the repository's existing ownership tree.
 *
 * Each imported grammar is a parser grammar.
 *
 * The imports are composition dependencies, not competing authorities.
 *
 * ============================================================================
 */

import
    HardwareModules,
    HardwareGenerics,
    Ports,
    Signals,
    Wires,
    Registers,
    Memories,
    Clocks,
    Timing,
    Processes,
    Combinational,
    Sequential,
    StateMachines,
    Pipelines,
    Generate
;

/*
 * ============================================================================
 * PUBLIC HDL ENTRY POINT
 * ============================================================================
 *
 * This is the stable standalone HDL parser boundary.
 *
 * grammar/antlr/ZamaniParser.g4 consumes the HDL domain through its HDL
 * composition grammar.
 *
 * ============================================================================
 */

hdlDesign
    : hdlSourceElement* EOF
    ;

/*
 * ============================================================================
 * HDL SOURCE ELEMENT
 * ============================================================================
 *
 * Top-level HDL constructs remain intentionally open-ended through the
 * subordinate grammar ownership model.
 *
 * No fixed number of elements is imposed.
 * ============================================================================
 */

hdlSourceElement
    : hdlDeclaration
    | hdlStatement
    | hdlExpression
    ;

/*
 * ============================================================================
 * HDL DECLARATION DISPATCH
 * ============================================================================
 *
 * This is the stable integration point used by ZamaniParser.
 *
 * Feature syntax remains owned by the subordinate grammar.
 *
 * ============================================================================
 */

hdlDeclaration
    : hdlModuleDeclaration
    | hdlInterfaceDeclaration
    | hdlPackageDeclaration
    | hdlDialectDeclaration
    | hdlParameterDeclaration
    | hdlLocalParameterDeclaration
    | hdlTypeDeclaration
    | hdlPortDeclaration
    | hdlSignalDeclaration
    | hdlNetDeclaration
    | hdlRegisterDeclaration
    | hdlMemoryDeclaration
    | hdlClockDeclaration
    | hdlResetDeclaration
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
    ;

/*
 * ============================================================================
 * HDL STATEMENT DISPATCH
 * ============================================================================
 *
 * HDL statements are deliberately structural.
 *
 * General expression semantics remain owned by the canonical expression
 * subsystem.
 * ============================================================================
 */

hdlStatement
    : hdlAssignment
    | hdlIfStatement
    | hdlCaseStatement
    | hdlForStatement
    | hdlWhileStatement
    | hdlRepeatStatement
    | hdlExpressionStatement
    ;

/*
 * ============================================================================
 * HDL EXPRESSION INTEGRATION
 * ============================================================================
 *
 * The HDL domain must not become a second universal expression language.
 *
 * The canonical expression subsystem remains the semantic authority.
 *
 * This façade provides a stable HDL integration name.
 *
 * ============================================================================
 */

hdlExpression
    : expression
    ;

/*
 * ============================================================================
 * HDL TYPE INTEGRATION
 * ============================================================================
 *
 * The canonical type subsystem remains authoritative.
 *
 * HDL-specific width, shape, storage, signal and hardware semantics are
 * represented by ordinary type/value expressions and interpreted downstream.
 *
 * ============================================================================
 */

hdlTypeExpression
    : typeExpression
    ;

/*
 * ============================================================================
 * HDL ATTRIBUTE INTEGRATION
 * ============================================================================
 *
 * Attributes remain metadata.
 *
 * Vendor-specific attributes must not be hard-coded into this root.
 *
 * ============================================================================
 */

hdlAttribute
    : attribute
    ;

/*
 * ============================================================================
 * HDL BLOCK INTEGRATION
 * ============================================================================
 *
 * A block is a canonical Zamani block.
 *
 * HDL-specific semantics are assigned downstream.
 * ============================================================================
 */

hdlBlock
    : block
    ;

/*
 * ============================================================================
 * HDL NAME INTEGRATION
 * ============================================================================
 *
 * HDL names remain logical source-level names.
 *
 * They do not identify physical devices unless a downstream semantic layer
 * explicitly assigns such meaning.
 *
 * ============================================================================
 */

hdlIdentifier
    : identifier
    ;

hdlQualifiedName
    : qualifiedName
    ;

/*
 * ============================================================================
 * HDL RANGE INTEGRATION
 * ============================================================================
 *
 * Ranges are semantic expressions.
 *
 * No width limit is imposed.
 * ============================================================================
 */

hdlRangeExpression
    : rangeExpression
    ;

/*
 * ============================================================================
 * HDL RESOURCE/CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Resource and capability intent remains owned by grammar/resources/ and
 * grammar/hardware/.
 *
 * HDL only exposes the integration point.
 *
 * ============================================================================
 */

hdlResourceIntent
    : requiresClause
    | constraintClause
    | preferenceClause
    | hintClause
    ;

/*
 * ============================================================================
 * RESOURCE REQUIREMENT
 * ============================================================================
 *
 * The actual resource grammar owns semantic structure.
 *
 * This façade exists so HDL members can consume resource intent without
 * introducing an HDL-specific resource language.
 * ============================================================================
 */

requiresClause
    : REQUIRES expression
    ;

/*
 * ============================================================================
 * CONSTRAINT
 * ============================================================================
 */

constraintClause
    : CONSTRAINT expression
    ;

/*
 * ============================================================================
 * PREFERENCE
 * ============================================================================
 */

preferenceClause
    : PREFER expression
    ;

/*
 * ============================================================================
 * HINT
 * ============================================================================
 */

hintClause
    : HINT expression
    ;

/*
 * ============================================================================
 * HDL PORTABLE HARDWARE CONTRACT
 * ============================================================================
 *
 * This rule exists as an explicit architectural boundary.
 *
 * It does not allocate hardware.
 *
 * It expresses source-level intent that may later be analyzed by:
 *
 *     resources
 *     capabilities
 *     compilation
 *     scheduling
 *     placement
 *     routing
 *     synthesis
 *
 * ============================================================================
 */

hdlHardwareIntent
    : hdlResourceIntent
    | hdlAttribute
    | hdlExpression
    ;

/*
 * ============================================================================
 * CROSS-DOMAIN QUANTUM HARDWARE CONTRACT
 * ============================================================================
 *
 * HDL may contain source-level references to quantum constructs through the
 * canonical Zamani expression/declaration system.
 *
 * No quantum gate vocabulary is introduced here.
 *
 * No physical qubit numbering is introduced here.
 *
 * No QEC/ZQN/HAL representation is introduced here.
 * ============================================================================
 */

hdlQuantumHardwareIntent
    : hdlHardwareIntent
    ;

/*
 * ============================================================================
 * DIALECT INTEGRATION
 * ============================================================================
 *
 * Hardware dialects remain open-world.
 *
 * This root does not enumerate:
 *
 *     Xilinx
 *     AMD
 *     Intel
 *     NVIDIA
 *     ARM
 *     RISC-V
 *     IBM
 *     Google
 *     Quantinuum
 *     Rigetti
 *     or any other vendor/fabrication family.
 *
 * Such identities remain data/dialect metadata.
 * ============================================================================
 */

hdlDialectDeclaration
    : DIALECT hdlQualifiedName hdlDialectBody?
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
    : hdlIdentifier
      (
          ASSIGN hdlExpression
      )?
      SEMICOLON
    ;

/*
 * ============================================================================
 * PACKAGE INTEGRATION
 * ============================================================================
 *
 * HDL packages remain logical namespaces.
 *
 * They do not select target hardware.
 * ============================================================================
 */

hdlPackageDeclaration
    : PACKAGE hdlQualifiedName hdlPackageBody
    ;

hdlPackageBody
    : LBRACE
      hdlPackageMember*
      RBRACE
    ;

hdlPackageMember
    : hdlAttribute
      hdlPackageMemberCore
    | hdlPackageMemberCore
    ;

hdlPackageMemberCore
    : hdlTypeDeclaration
    | hdlParameterDeclaration
    | hdlLocalParameterDeclaration
    | hdlInterfaceDeclaration
    | hdlModuleDeclaration
    ;

/*
 * ============================================================================
 * SHARED HDL MEMBER COMPOSITION
 * ============================================================================
 *
 * This rule is the critical integration contract between module syntax and the
 * existing HDL feature grammars.
 *
 * It is intentionally located in the composition root.
 *
 * The individual grammars remain feature owners.
 *
 * ============================================================================
 */

hdlModuleMember
    : hdlAttribute*
      hdlModuleMemberCore
    ;

hdlModuleMemberCore
    : hdlParameterDeclaration
    | hdlLocalParameterDeclaration
    | hdlTypeDeclaration
    | hdlInterfaceDeclaration
    | hdlPortDeclaration
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
 * HDL BLOCK DECLARATION
 * ============================================================================
 *
 * A generic logical HDL block is not a physical resource.
 * ============================================================================
 */

hdlBlockDeclaration
    : hdlIdentifier
      hdlBlock
    ;

/*
 * ============================================================================
 * EXPRESSION STATEMENT
 * ============================================================================
 */

hdlExpressionStatement
    : hdlExpression
      SEMICOLON
    ;

/*
 * ============================================================================
 * HDL ASSIGNMENT
 * ============================================================================
 *
 * Assignment semantics are resolved downstream.
 *
 * No target-specific register or net semantics are embedded here.
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
    | MODULO_ASSIGN
    | AMPERSAND_ASSIGN
    | PIPE_ASSIGN
    | CARET_ASSIGN
    ;

hdlLValue
    : hdlQualifiedName
      hdlIndexSuffix*
    ;

/*
 * ============================================================================
 * HDL INDEX
 * ============================================================================
 */

hdlIndexSuffix
    : LBRACKET
      hdlExpression
      RBRACKET
    ;

/*
 * ============================================================================
 * CONTROL-FLOW INTEGRATION
 * ============================================================================
 *
 * These forms remain source-level procedural syntax.
 *
 * Their hardware interpretation belongs to semantic analysis.
 * ============================================================================
 */

hdlIfStatement
    : IF
      hdlExpression
      hdlBlock
      (
          ELSE
          (
              IF hdlExpression hdlBlock
            | hdlBlock
          )
      )?
    ;

hdlCaseStatement
    : CASE
      LPAREN hdlExpression RPAREN
      LBRACE
      hdlCaseItem*
      RBRACE
    ;

hdlCaseItem
    : DEFAULT
      COLON
      hdlBlock
    | hdlCaseExpressionList
      COLON
      hdlBlock
    ;

hdlCaseExpressionList
    : hdlExpression
      (
          COMMA hdlExpression
      )*
    ;

hdlForStatement
    : FOR
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
    : LET
      hdlIdentifier
      (
          COLON hdlTypeExpression
      )?
      (
          ASSIGN hdlExpression
      )?
    | VAR
      hdlIdentifier
      (
          COLON hdlTypeExpression
      )?
      (
          ASSIGN hdlExpression
      )?
    | CONST
      hdlIdentifier
      (
          COLON hdlTypeExpression
      )?
      (
          ASSIGN hdlExpression
      )?
    ;

hdlWhileStatement
    : WHILE
      hdlExpression
      hdlBlock
    ;

hdlRepeatStatement
    : REPEAT
      hdlExpression
      hdlBlock
    ;

/*
 * ============================================================================
 * SHARED HDL TYPE DECLARATION ADAPTER
 * ============================================================================
 *
 * This façade exists because several existing HDL delegates consume
 * `typeExpression`, while HDL consumers historically referenced
 * `hdlTypeExpression`.
 *
 * No second type system is introduced.
 * ============================================================================
 */

hdlTypeAliasDeclaration
    : TYPE
      hdlIdentifier
      (
          hardwareGenericParameters
      )?
      ASSIGN
      hdlTypeExpression
      SEMICOLON
    ;

/*
 * ============================================================================
 * SOURCE-LEVEL HDL VERIFICATION
 * ============================================================================
 *
 * Verification semantics remain downstream.
 *
 * The grammar intentionally does not encode a finite assertion language.
 * ============================================================================
 */

hdlAssertion
    : hdlAssertionKind
      hdlExpression
      SEMICOLON
    ;

hdlAssertionKind
    : ASSERT
    | ASSUME
    | COVER
    ;

/*
 * ============================================================================
 * GENERIC HDL VERIFICATION HOOK
 * ============================================================================
 *
 * This allows future verification dialects to attach through ordinary
 * expressions/attributes without requiring this universal root to enumerate
 * every verification technology.
 * ============================================================================
 */

hdlVerificationIntent
    : hdlAssertion
    | hdlAttribute
    ;

/*
 * ============================================================================
 * HARDWARE CAPABILITY HOOK
 * ============================================================================
 *
 * Capability names are ordinary semantic names.
 *
 * The grammar does not know whether a target provides them.
 * ============================================================================
 */

hdlCapabilityReference
    : hdlIdentifier
    ;

hdlCapabilityInvocation
    : hdlCapabilityReference
      LPAREN
      argumentList?
      RPAREN
    ;

/*
 * ============================================================================
 * RESOURCE SCALING HOOK
 * ============================================================================
 *
 * Scaling is expressed symbolically.
 *
 * Examples of valid semantic values:
 *
 *     N
 *     width
 *     depth
 *     lanes
 *     rows * cols
 *     available_capacity
 *
 * No maximum is encoded.
 * ============================================================================
 */

hdlScalingExpression
    : hdlExpression
    ;

/*
 * ============================================================================
 * RANGE ADAPTER
 * ============================================================================
 */

hdlRange
    : LBRACKET
      hdlRangeExpression
      RBRACKET
    ;

/*
 * ============================================================================
 * COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] It is the single HDL parser composition root.
 *
 * [x] It uses `parser grammar HDL`.
 *
 * [x] It consumes `ZamaniLexer`.
 *
 * [x] It does not define lexer rules.
 *
 * [x] It does not define a second hardware lexer.
 *
 * [x] It does not enumerate vendors.
 *
 * [x] It does not enumerate FPGA families.
 *
 * [x] It does not enumerate ASIC families.
 *
 * [x] It does not enumerate CPU/GPU/QPU devices.
 *
 * [x] It does not encode physical topology.
 *
 * [x] It does not encode physical placement.
 *
 * [x] It does not encode routing.
 *
 * [x] It does not encode synthesis implementation.
 *
 * [x] It does not encode QEC.
 *
 * [x] It does not encode ZQN.
 *
 * [x] It does not create a quantum IR.
 *
 * [x] It does not impose hardware-size limits.
 *
 * [x] It composes existing HDL feature grammars.
 *
 * [x] Module syntax remains owned by HardwareModules.
 *
 * [x] Generic syntax remains owned by HardwareGenerics.
 *
 * [x] Port syntax remains owned by Ports.
 *
 * [x] Signal syntax remains owned by Signals.
 *
 * [x] Net syntax remains owned by Wires.
 *
 * [x] Register syntax remains owned by Registers.
 *
 * [x] Memory syntax remains owned by Memories.
 *
 * [x] Clock syntax remains owned by Clocks.
 *
 * [x] Timing syntax remains owned by Timing.
 *
 * [x] Process syntax remains owned by Processes.
 *
 * [x] Combinational syntax remains owned by Combinational.
 *
 * [x] Sequential syntax remains owned by Sequential.
 *
 * [x] State-machine syntax remains owned by StateMachines.
 *
 * [x] Pipeline syntax remains owned by Pipelines.
 *
 * [x] Generate syntax remains owned by Generate.
 *
 * [x] Expressions remain integrated with the canonical expression subsystem.
 *
 * [x] Types remain integrated with the canonical type subsystem.
 *
 * [x] Attributes remain integrated with the canonical attribute subsystem.
 *
 * [x] Names remain source-level logical names.
 *
 * [x] Resource requirements remain separate from target realization.
 *
 * [x] Capability requirements remain semantic intent.
 *
 * [x] Preferences remain non-binding.
 *
 * [x] Hardware realization remains downstream.
 *
 * [x] Parsing remains deterministic.
 *
 * [x] No embedded Rust exists.
 *
 * [x] No unsafe implementation is required.
 *
 * [x] Rust 1.97 / 1.97.1 integration is preserved.
 *
 * [x] Domain-neutral AST integration is defined.
 *
 * [x] Canonical hardware IR integration is defined.
 *
 * [x] quantum::ir remains the only canonical quantum IR.
 *
 * ============================================================================
 * INTEGRATION REQUIREMENTS FOR OTHER FILES
 * ============================================================================
 *
 * The following existing files are the predefined integration contracts:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *         -> canonical token vocabulary
 *
 *     grammar/antlr/ZamaniParser.g4
 *         -> universal parser composition
 *
 *     grammar/hdl/hardware-modules.g4
 *         -> module ownership
 *
 *     grammar/hdl/hardware-generics.g4
 *         -> generic ownership
 *
 *     grammar/hdl/ports.g4
 *         -> port ownership
 *
 *     grammar/hdl/signals.g4
 *         -> signal ownership
 *
 *     grammar/hdl/wires.g4
 *         -> net/wire ownership
 *
 *     grammar/hdl/registers.g4
 *         -> register ownership
 *
 *     grammar/hdl/memories.g4
 *         -> memory ownership
 *
 *     grammar/hdl/clocks.g4
 *         -> clock ownership
 *
 *     grammar/hdl/timing.g4
 *         -> timing ownership
 *
 *     grammar/hdl/processes.g4
 *         -> process ownership
 *
 *     grammar/hdl/combinational.g4
 *         -> combinational ownership
 *
 *     grammar/hdl/sequential.g4
 *         -> sequential ownership
 *
 *     grammar/hdl/state-machines.g4
 *         -> state-machine ownership
 *
 *     grammar/hdl/pipelines.g4
 *         -> pipeline ownership
 *
 *     grammar/hdl/generate.g4
 *         -> generation ownership
 *
 *     grammar/types/types.g4
 *         -> canonical type syntax
 *
 *     grammar/expressions/expressions.g4
 *         -> canonical expression syntax
 *
 *     grammar/core/*
 *         -> canonical names, attributes, blocks and shared syntax
 *
 *     grammar/resources/*
 *         -> resource/capability intent
 *
 *     grammar/hardware/*
 *         -> target-independent hardware intent
 *
 *     grammar/validation/*
 *         -> grammar validation and conformance
 *
 *     grammar/tests/hdl/*
 *         -> HDL positive/negative/boundary/scalability tests
 *
 * No downstream file should need to modify this file merely because a target
 * has changed.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL INVARIANT
 * ============================================================================
 *
 * Zamani HDL therefore follows:
 *
 *     Program Once
 *          |
 *          v
 *     Compile Once
 *          |
 *          v
 *     Run Everywhere
 *          |
 *          v
 *     Run Anywhere
 *          |
 *          v
 *     Forever
 *
 * subject to:
 *
 *     program semantics
 *     explicit requirements
 *     target capabilities
 *     available resources
 *     implementation policies
 *
 * and WITHOUT introducing artificial language-level hardware ceilings.
 *
 * ============================================================================
 */
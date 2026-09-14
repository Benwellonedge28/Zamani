/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hdl/ports.g4
 *
 * Purpose:
 *     Production parser grammar for HDL/hardware port declarations.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust integration baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no embedded Rust.
 *     No Rust `unsafe` code is used or required.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
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
 *          +--> HDL module grammar
 *          |
 *          +--> HardwareModules
 *          |
 *          +--> HardwarePorts  <--- THIS FILE
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> type checking
 *          +--> direction checking
 *          +--> interface checking
 *          +--> capability checking
 *          +--> resource analysis
 *          +--> target-independent validation
 *          |
 *          v
 *     canonical hardware semantic representation
 *          |
 *          +--> optimization
 *          +--> scheduling
 *          +--> routing
 *          +--> synthesis
 *          +--> target lowering
 *          |
 *          v
 *     hardware/runtime realization
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - HDL port declaration syntax;
 *     - port direction syntax;
 *     - port mode syntax;
 *     - port type attachment;
 *     - port width/range syntax;
 *     - port array/dimensional syntax;
 *     - port default/value syntax;
 *     - port attributes;
 *     - port constraints that are semantic source-level constraints;
 *     - port grouping syntax;
 *     - port-list syntax;
 *     - interface-facing port declarations;
 *     - port connection syntax where that syntax belongs to the port model;
 *     - parser-facing port composition contracts.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - identifier definitions;
 *     - general expressions;
 *     - general type semantics;
 *     - general declarations;
 *     - hardware modules;
 *     - signals;
 *     - wires;
 *     - registers;
 *     - clocks;
 *     - timing;
 *     - memories;
 *     - pipelines;
 *     - processes;
 *     - state machines;
 *     - hardware discovery;
 *     - physical pins;
 *     - board layouts;
 *     - device IDs;
 *     - physical addresses;
 *     - vendor-specific placement;
 *     - routing;
 *     - scheduling;
 *     - synthesis;
 *     - calibration;
 *     - runtime dispatch;
 *     - resource allocation;
 *     - canonical IR construction.
 *
 * ============================================================================
 * POCO-REAF PRINCIPLE
 * ============================================================================
 *
 * A port describes a logical hardware boundary.
 *
 * It does NOT inherently identify:
 *
 *     - a physical FPGA pin;
 *     - an ASIC pad;
 *     - a package ball;
 *     - a device address;
 *     - a bus number;
 *     - a machine;
 *     - a board;
 *     - a vendor;
 *     - a physical connector;
 *     - a fixed hardware topology.
 *
 * Physical realization belongs to target/resource/deployment configuration.
 *
 * Therefore this grammar deliberately contains no:
 *
 *     MAX_PORTS
 *     MAX_WIDTH
 *     MAX_LANES
 *     MAX_DIMENSIONS
 *     MAX_INTERFACES
 *     MAX_CONNECTIONS
 *
 * and contains no fixed physical-resource identifiers.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * Port collections use repetition rather than fixed cardinality.
 *
 * Examples:
 *
 *     one port
 *     many ports
 *     parameterized port groups
 *     arbitrarily large generated port sets
 *
 * are represented by the same syntax.
 *
 * Any actual implementation limit belongs to:
 *
 *     parser resource policy;
 *     semantic analysis;
 *     compiler resource policy;
 *     target capabilities;
 *     synthesis;
 *     deployment;
 *     runtime resources.
 *
 * ============================================================================
 * CANONICAL LEXER CONTRACT
 * ============================================================================
 *
 * This parser consumes the canonical lexer vocabulary.
 *
 * Required stable tokens:
 *
 *     IDENTIFIER
 *     K_INPUT
 *     K_OUTPUT
 *     K_INOUT
 *     K_IN
 *     K_OUT
 *     K_REF
 *     K_CONST
 *     K_MUT
 *
 * plus the canonical punctuation/operator tokens:
 *
 *     COLON
 *     COMMA
 *     SEMICOLON
 *     ASSIGN
 *     LPAREN
 *     RPAREN
 *     LBRACKET
 *     RBRACKET
 *     LBRACE
 *     RBRACE
 *     LT
 *     GT
 *     DOT
 *     DOUBLE_COLON
 *
 * IMPORTANT:
 *
 * The repository's canonical lexer is authoritative.
 *
 * If a particular contextual HDL word is currently lexed as IDENTIFIER,
 * this grammar may accept it through the contextual rules below.
 *
 * This file MUST NOT create a second lexer.
 *
 * ============================================================================
 * SHARED-PARSER CONTRACT
 * ============================================================================
 *
 * When imported by the canonical HDL parser, the importing grammar supplies
 * shared rules such as:
 *
 *     identifier
 *     expression
 *     typeExpr
 *     qualifiedName
 *     attribute
 *
 * To keep this component independently testable, this file provides the
 * minimum HDL-port-specific expression/type adapters required by the port
 * grammar without defining a second general-purpose type system.
 *
 * ============================================================================
 */

parser grammar HardwarePorts;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Parse one complete HDL port declaration.
 *
 * Examples:
 *
 *     input data: logic;
 *
 *     output result: logic;
 *
 *     inout bus: logic[WIDTH];
 *
 *     input valid: bool;
 *
 * ============================================================================
 */

hdlPortDeclaration
    : hdlPortAttributes?
      hdlPortDirection
      hdlPortMode*
      hdlPortName
      hdlPortType?
      hdlPortDimensions*
      hdlPortDefaultValue?
      hdlPortConstraints*
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 2. PORT LIST
 * ============================================================================
 *
 * Used by module declarations and interface declarations.
 *
 * There is deliberately no fixed port count.
 * ============================================================================
 */

hdlPortList
    : LPAREN
      hdlPortDeclarationItems?
      RPAREN
    ;


hdlPortDeclarationItems
    : hdlPortDeclarationItem
      (
          COMMA
        | SEMICOLON
      )
      hdlPortDeclarationItem
      (
          (
              COMMA
            | SEMICOLON
          )
          hdlPortDeclarationItem
      )*
      (
          COMMA
        | SEMICOLON
      )?
    ;


hdlPortDeclarationItem
    : hdlPortAttributes?
      hdlPortDirection
      hdlPortMode*
      hdlPortName
      hdlPortType?
      hdlPortDimensions*
      hdlPortDefaultValue?
      hdlPortConstraints*
    ;


/*
 * ============================================================================
 * 3. PORT GROUP
 * ============================================================================
 *
 * Port groups allow a logical collection of related ports without imposing
 * a machine-specific number of ports.
 *
 * Example:
 *
 *     input {
 *         valid: bool;
 *         ready: bool;
 *     }
 *
 * The group name is optional because some module/interface syntaxes use
 * anonymous direction groups.
 * ============================================================================
 */

hdlPortGroup
    : hdlPortAttributes?
      hdlPortGroupName?
      hdlPortGroupDirection?
      LBRACE
      hdlPortGroupMember*
      RBRACE
    ;


hdlPortGroupName
    : hdlPortName
    ;


hdlPortGroupDirection
    : hdlPortDirection
    ;


hdlPortGroupMember
    : hdlPortDeclaration
    | hdlPortGroup
    ;


/*
 * ============================================================================
 * 4. PORT DIRECTION
 * ============================================================================
 *
 * Direction is a semantic property of the logical interface.
 *
 * It is not a physical pin direction.
 *
 * The preferred canonical vocabulary is:
 *
 *     input
 *     output
 *     inout
 *
 * Additional directional forms may be introduced through the canonical
 * language-versioning process.
 * ============================================================================
 */

hdlPortDirection
    : hdlInputDirection
    | hdlOutputDirection
    | hdlInoutDirection
    ;


hdlInputDirection
    : K_INPUT
    | hdlContextualKeywordInput
    ;


hdlOutputDirection
    : K_OUTPUT
    | hdlContextualKeywordOutput
    ;


hdlInoutDirection
    : K_INOUT
    | hdlContextualKeywordInout
    ;


/*
 * Contextual fallbacks are intentionally isolated.
 *
 * They exist for compatibility with repositories in which some HDL vocabulary
 * is still lexed as IDENTIFIER.
 *
 * Semantic validation must ensure these names have the intended meaning.
 */

hdlContextualKeywordInput
    : IDENTIFIER
    ;


hdlContextualKeywordOutput
    : IDENTIFIER
    ;


hdlContextualKeywordInout
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * 5. PORT MODES
 * ============================================================================
 *
 * A direction answers:
 *
 *     "Which logical flow does this port represent?"
 *
 * A mode answers additional semantic questions.
 *
 * Examples:
 *
 *     const
 *     mutable
 *     ref
 *
 * Mode semantics are validated outside the grammar.
 * ============================================================================
 */

hdlPortMode
    : K_CONST
    | K_MUT
    | K_REF
    | hdlContextualPortMode
    ;


hdlContextualPortMode
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * 6. PORT NAME
 * ============================================================================
 *
 * Port names are logical source symbols.
 *
 * They are not:
 *
 *     physical pins;
 *     addresses;
 *     device IDs;
 *     board locations.
 * ============================================================================
 */

hdlPortName
    : identifier
    ;


/*
 * ============================================================================
 * 7. PORT TYPE
 * ============================================================================
 *
 * A port may have an explicit logical type.
 *
 * The type is not a physical implementation type.
 *
 * For example:
 *
 *     logic
 *     bool
 *     bit
 *     Bus<T>
 *     Vector<T, WIDTH>
 *
 * may be represented here.
 *
 * Semantic analysis determines whether the type is valid for the selected
 * HDL dialect and context.
 * ============================================================================
 */

hdlPortType
    : COLON
      hdlPortTypeExpression
    ;


hdlPortTypeExpression
    : hdlPortTypePrimary
      hdlPortTypeSuffix*
    ;


hdlPortTypePrimary
    : identifier
    | hdlPortQualifiedTypeName
    | LPAREN
      hdlPortTypeExpression
      RPAREN
    ;


hdlPortQualifiedTypeName
    : identifier
      (
          DOUBLE_COLON
          identifier
      )*
    ;


hdlPortTypeSuffix
    : LT
      hdlPortTypeArgumentList?
      GT
    | LBRACKET
      hdlPortRangeExpression?
      RBRACKET
    ;


hdlPortTypeArgumentList
    : hdlPortTypeArgument
      (
          COMMA
          hdlPortTypeArgument
      )*
      COMMA?
    ;


hdlPortTypeArgument
    : hdlPortTypeExpression
    | hdlPortExpression
    ;


/*
 * ============================================================================
 * 8. PORT DIMENSIONS
 * ============================================================================
 *
 * Dimensions describe logical data shape.
 *
 * They do not describe physical memory size or hardware resource capacity.
 *
 * Examples:
 *
 *     data[WIDTH]
 *     matrix[ROWS][COLS]
 *     bus[MSB:LSB]
 *
 * No dimension count is fixed.
 * ============================================================================
 */

hdlPortDimensions
    : LBRACKET
      hdlPortRangeExpression?
      RBRACKET
    ;


hdlPortRangeExpression
    : hdlPortRangeEndpoint
      hdlPortRangeOperator?
      hdlPortRangeEndpoint?
    ;


hdlPortRangeEndpoint
    : hdlPortExpression
    ;


hdlPortRangeOperator
    : COLON
    | DOT_DOT
    | DOT_DOT_EQ
    ;


/*
 * ============================================================================
 * 9. DEFAULT PORT VALUES
 * ============================================================================
 *
 * Defaults are source-level semantic values.
 *
 * They do not imply a physical pull-up, pull-down, FPGA initialization mode,
 * ASIC power-up value, or other target-specific implementation unless a
 * downstream semantic rule explicitly establishes such meaning.
 * ============================================================================
 */

hdlPortDefaultValue
    : ASSIGN
      hdlPortExpression
    ;


/*
 * ============================================================================
 * 10. PORT CONSTRAINTS
 * ============================================================================
 *
 * These are logical/source-level constraints.
 *
 * They are intentionally separate from physical placement constraints.
 *
 * Examples:
 *
 *     width-related constraints
 *     protocol constraints
 *     direction constraints
 *     timing-relevant semantic contracts
 *
 * Physical constraints such as:
 *
 *     PIN_A7
 *     FPGA_BANK_3
 *     package_ball_17
 *
 * do not belong in this grammar.
 *
 * Those belong to target/deployment constraint systems.
 * ============================================================================
 */

hdlPortConstraints
    : hdlPortConstraint
    ;


hdlPortConstraint
    : hdlPortAttributeConstraint
    | hdlPortExpressionConstraint
    ;


hdlPortAttributeConstraint
    : AT
      identifier
      (
          LPAREN
          hdlPortArgumentList?
          RPAREN
      )?
    ;


hdlPortExpressionConstraint
    : LBRACE
      hdlPortConstraintExpressionList?
      RBRACE
    ;


hdlPortConstraintExpressionList
    : hdlPortExpression
      (
          COMMA
          hdlPortExpression
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 11. PORT ATTRIBUTES
 * ============================================================================
 *
 * Attributes are metadata.
 *
 * They must not silently become physical-target selectors.
 * ============================================================================
 */

hdlPortAttributes
    : hdlPortAttribute+
    ;


hdlPortAttribute
    : AT
      identifier
      (
          LPAREN
          hdlPortArgumentList?
          RPAREN
      )?
    ;


hdlPortArgumentList
    : hdlPortArgument
      (
          COMMA
          hdlPortArgument
      )*
      COMMA?
    ;


hdlPortArgument
    : identifier
      ASSIGN
      hdlPortExpression
    | hdlPortExpression
    ;


/*
 * ============================================================================
 * 12. PORT EXPRESSION ADAPTER
 * ============================================================================
 *
 * Port expressions are intentionally bounded to the expression forms needed
 * by port declarations:
 *
 *     constants
 *     generic parameters
 *     arithmetic width expressions
 *     symbolic references
 *     ranges
 *     compile-time expressions
 *
 * Runtime side effects do not belong in port declarations.
 *
 * Examples:
 *
 *     WIDTH
 *     WIDTH + 1
 *     2 * LANES
 *     DATA_WIDTH / 8
 *
 * The semantic/compiler layer determines whether an expression is
 * compile-time evaluable where required.
 * ============================================================================
 */

hdlPortExpression
    : hdlPortLogicalOrExpression
    ;


hdlPortLogicalOrExpression
    : hdlPortLogicalAndExpression
      (
          LOGICAL_OR
          hdlPortLogicalAndExpression
      )*
    ;


hdlPortLogicalAndExpression
    : hdlPortBitwiseOrExpression
      (
          LOGICAL_AND
          hdlPortBitwiseOrExpression
      )*
    ;


hdlPortBitwiseOrExpression
    : hdlPortBitwiseXorExpression
      (
          PIPE
          hdlPortBitwiseXorExpression
      )*
    ;


hdlPortBitwiseXorExpression
    : hdlPortBitwiseAndExpression
      (
          CARET
          hdlPortBitwiseAndExpression
      )*
    ;


hdlPortBitwiseAndExpression
    : hdlPortEqualityExpression
      (
          AMPERSAND
          hdlPortEqualityExpression
      )*
    ;


hdlPortEqualityExpression
    : hdlPortRelationalExpression
      (
          (
              EQUAL_EQUAL
            | NOT_EQUAL
          )
          hdlPortRelationalExpression
      )*
    ;


hdlPortRelationalExpression
    : hdlPortRangeArithmeticExpression
      (
          (
              LT
            | GT
            | LESS_EQUAL
            | GREATER_EQUAL
          )
          hdlPortRangeArithmeticExpression
      )*
    ;


hdlPortRangeArithmeticExpression
    : hdlPortAdditiveExpression
      (
          (
              DOT_DOT
            | DOT_DOT_EQ
          )
          hdlPortAdditiveExpression
      )?
    ;


hdlPortAdditiveExpression
    : hdlPortMultiplicativeExpression
      (
          (
              PLUS
            | MINUS
          )
          hdlPortMultiplicativeExpression
      )*
    ;


hdlPortMultiplicativeExpression
    : hdlPortUnaryExpression
      (
          (
              STAR
            | SLASH
            | PERCENT
          )
          hdlPortUnaryExpression
      )*
    ;


hdlPortUnaryExpression
    : (
          PLUS
        | MINUS
        | EXCLAMATION
        | TILDE
      )
      hdlPortUnaryExpression
    | hdlPortPrimaryExpression
    ;


hdlPortPrimaryExpression
    : identifier
    | INTEGER
    | FLOAT
    | STRING
    | hdlPortBooleanLiteral
    | LPAREN
      hdlPortExpression
      RPAREN
    | hdlPortQualifiedReference
    ;


hdlPortQualifiedReference
    : identifier
      (
          DOUBLE_COLON
          identifier
      )*
    ;


hdlPortBooleanLiteral
    : TRUE
    | FALSE
    ;


/*
 * ============================================================================
 * 13. PORT CONNECTION
 * ============================================================================
 *
 * A logical connection binds a port to a source-level expression.
 *
 * It does not perform physical routing.
 * ============================================================================
 */

hdlPortConnection
    : hdlPortName
      ASSIGN
      hdlPortExpression
    ;


hdlPortConnectionList
    : hdlPortConnection
      (
          COMMA
          hdlPortConnection
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 14. NAMED PORT CONNECTION
 * ============================================================================
 *
 * Supports explicit source-level association:
 *
 *     input_data = data
 *     output_data = result
 *
 * ============================================================================
 */

hdlNamedPortConnection
    : hdlPortName
      ASSIGN
      hdlPortExpression
    ;


hdlNamedPortConnectionList
    : hdlNamedPortConnection
      (
          COMMA
          hdlNamedPortConnection
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 15. POSITIONAL PORT CONNECTIONS
 * ============================================================================
 *
 * Positional connections are supported for compact structural descriptions.
 *
 * They remain source-level associations and do not imply physical ordering.
 * ============================================================================
 */

hdlPositionalPortConnections
    : LPAREN
      hdlPositionalPortConnectionList?
      RPAREN
    ;


hdlPositionalPortConnectionList
    : hdlPortExpression
      (
          COMMA
          hdlPortExpression
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 16. MIXED PORT CONNECTIONS
 * ============================================================================
 *
 * Named and positional connection semantics must be validated by semantic
 * analysis.
 *
 * This grammar only captures the syntax.
 * ============================================================================
 */

hdlMixedPortConnections
    : LPAREN
      hdlMixedPortConnectionList?
      RPAREN
    ;


hdlMixedPortConnectionList
    : hdlMixedPortConnection
      (
          COMMA
          hdlMixedPortConnection
      )*
      COMMA?
    ;


hdlMixedPortConnection
    : hdlNamedPortConnection
    | hdlPortExpression
    ;


/*
 * ============================================================================
 * 17. PORT MAP
 * ============================================================================
 *
 * Explicit port mapping is useful when composing reusable hardware modules.
 *
 * Example:
 *
 *     port map {
 *         input_data = data;
 *         output_data = result;
 *     }
 *
 * The containing module/instance grammar owns the surrounding construct.
 * ============================================================================
 */

hdlPortMap
    : LBRACE
      hdlNamedPortConnectionList?
      RBRACE
    ;


/*
 * ============================================================================
 * 18. PORT DECLARATION GROUPS
 * ============================================================================
 *
 * Grouping supports reusable interface declarations without requiring a
 * particular hardware width, count, or topology.
 * ============================================================================
 */

hdlPortDeclarationGroup
    : hdlPortGroupHeader
      LBRACE
      hdlPortDeclarationItem*
      RBRACE
    ;


hdlPortGroupHeader
    : hdlPortAttributes?
      identifier
      (
          COLON
          hdlPortTypeExpression
      )?
    ;


/*
 * ============================================================================
 * 19. INTERFACE PORT REFERENCE
 * ============================================================================
 *
 * Allows a module/interface to refer to a named logical port contract.
 *
 * Resolution belongs to interface/module semantic analysis.
 * ============================================================================
 */

hdlPortReference
    : hdlPortQualifiedReference
    ;


/*
 * ============================================================================
 * 20. PORT ALIAS
 * ============================================================================
 *
 * An alias is a source-level logical name.
 *
 * It does not create a physical alias or hardware resource.
 * ============================================================================
 */

hdlPortAlias
    : identifier
      ASSIGN
      hdlPortReference
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 21. PORT DECLARATION SEQUENCES
 * ============================================================================
 *
 * Useful as the parser-facing body consumed by HardwareModules and
 * HardwareInterfaces.
 * ============================================================================
 */

hdlPortDeclarationSequence
    : hdlPortDeclaration*
    ;


/*
 * ============================================================================
 * 22. CONTEXTUAL HARDWARE TYPE / MODE SUPPORT
 * ============================================================================
 *
 * These rules deliberately accept identifiers because the canonical lexer
 * must remain the sole lexical authority.
 *
 * Semantic analysis is responsible for determining whether a contextual word
 * is actually a permitted HDL construct.
 * ============================================================================
 */

hdlContextualPortTypeName
    : identifier
    ;


hdlContextualPortKeyword
    : identifier
    ;


/*
 * ============================================================================
 * 23. SEMANTIC BOUNDARY NOTES
 * ============================================================================
 *
 * This grammar MUST NOT attempt to determine:
 *
 *     - whether a width fits a device;
 *     - whether a port maps to a physical pin;
 *     - whether a bus fits available routing;
 *     - whether a clock frequency is realizable;
 *     - whether a voltage is supported;
 *     - whether a target has enough I/O;
 *     - whether a quantum/classical interface is executable;
 *     - whether a port requires a particular vendor;
 *     - whether an implementation satisfies timing closure.
 *
 * Those questions belong to semantic analysis, capability analysis, resource
 * analysis, hardware abstraction, compilation, scheduling, placement,
 * routing, synthesis, and runtime/deployment layers.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 24. INTEGRATION CONTRACT
 * ============================================================================
 *
 * HardwareModules
 *     |
 *     +--> hdlPortDeclaration
 *     +--> hdlPortList
 *     +--> hdlPortMap
 *     +--> hdlPortReference
 *
 * HardwareInterfaces
 *     |
 *     +--> hdlPortDeclaration
 *     +--> hdlPortGroup
 *     +--> hdlPortReference
 *
 * Signals / Wires / Registers / Clocks
 *     |
 *     +--> may consume port references
 *
 * Hardware semantic analysis
 *     |
 *     +--> validates direction
 *     +--> validates types
 *     +--> validates dimensions
 *     +--> validates defaults
 *     +--> validates constraints
 *     +--> validates connections
 *
 * Resource/capability layer
 *     |
 *     +--> determines realizability
 *
 * Compiler
 *     |
 *     +--> lowers validated port semantics
 *
 * Hardware IR
 *     |
 *     +--> represents canonical logical interface
 *
 * Target lowering
 *     |
 *     +--> maps logical ports to physical implementation
 *
 * Runtime/deployment
 *     |
 *     +--> applies target-specific configuration
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 25. FORBIDDEN DEPENDENCIES
 * ============================================================================
 *
 * This grammar must never depend on:
 *
 *     hardware discovery
 *     target enumeration
 *     device drivers
 *     physical pin databases
 *     FPGA databases
 *     ASIC libraries
 *     calibration
 *     scheduling
 *     routing
 *     optimization
 *     runtime state
 *     quantum backend state
 *     QEC state
 *     ZQN state
 *
 * Quantum-aware ports may eventually reference quantum semantic types, but
 * this grammar must not import or redefine quantum::ir.
 *
 * quantum::ir remains the canonical quantum semantic boundary.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 26. HARD-CODING AUDIT
 * ============================================================================
 *
 * Deliberately absent:
 *
 *     MAX_PORTS
 *     MAX_WIDTH
 *     MAX_LANES
 *     MAX_BUS_WIDTH
 *     MAX_DIMENSIONS
 *     MAX_INTERFACES
 *     MAX_CONNECTIONS
 *     MAX_MODULES
 *     DEVICE_ID
 *     PIN_ID
 *     FPGA_ID
 *     ASIC_ID
 *     BOARD_ID
 *     ADDRESS
 *     TOPOLOGY
 *
 * Numeric literals used in source programs are data/semantic expressions,
 * not grammar-level machine limits.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 27. DETERMINISM
 * ============================================================================
 *
 * The grammar contains:
 *
 *     no actions;
 *     no semantic predicates;
 *     no mutable state;
 *     no external I/O;
 *     no target discovery.
 *
 * Therefore parsing is deterministic with respect to the supplied token
 * stream and ANTLR parser configuration.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 28. TEST CONTRACT
 * ============================================================================
 *
 * Positive tests MUST cover:
 *
 *     - single input port;
 *     - single output port;
 *     - bidirectional port;
 *     - typed port;
 *     - untyped port where permitted;
 *     - symbolic widths;
 *     - arithmetic widths;
 *     - multidimensional ports;
 *     - generic port types;
 *     - default values;
 *     - port attributes;
 *     - port groups;
 *     - named connections;
 *     - positional connections;
 *     - mixed connections;
 *     - aliases;
 *     - arbitrarily large port lists generated by tests.
 *
 * Negative tests MUST cover:
 *
 *     - missing port name;
 *     - malformed direction;
 *     - malformed type;
 *     - malformed dimension;
 *     - malformed range;
 *     - malformed default;
 *     - malformed attribute;
 *     - malformed connection;
 *     - unmatched delimiters;
 *     - invalid separator structure.
 *
 * Boundary tests MUST demonstrate:
 *
 *     - one port;
 *     - many ports;
 *     - zero-width expression rejection at semantic level where applicable;
 *     - very large symbolic dimensions without grammar limits;
 *     - deeply qualified logical names within parser resource policy.
 *
 * Cross-domain tests MUST include:
 *
 *     classical data port;
 *     quantum data/control port;
 *     hybrid quantum-classical port;
 *     accelerator port;
 *     distributed interface port;
 *     AI/tensor interface port;
 *     memory interface port.
 *
 * These tests validate syntax only. Physical realizability belongs downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 29. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE when:
 *
 *     [ ] canonical lexer integration is validated;
 *     [ ] token names match the authoritative lexer;
 *     [ ] no duplicate lexer is introduced;
 *     [ ] all public port rules have stable names;
 *     [ ] port direction syntax is defined;
 *     [ ] port type syntax is defined;
 *     [ ] port dimensions are unbounded by language-level constants;
 *     [ ] attributes are defined;
 *     [ ] constraints are syntactically represented;
 *     [ ] defaults are defined;
 *     [ ] logical connections are defined;
 *     [ ] port groups are defined;
 *     [ ] interface integration is defined;
 *     [ ] module integration is defined;
 *     [ ] no physical hardware assumptions exist;
 *     [ ] no device IDs exist;
 *     [ ] no fixed resource counts exist;
 *     [ ] no Rust actions exist;
 *     [ ] no unsafe code is required;
 *     [ ] semantic responsibilities are not implemented in grammar;
 *     [ ] quantum::ir is not duplicated;
 *     [ ] positive tests exist;
 *     [ ] negative tests exist;
 *     [ ] boundary tests exist;
 *     [ ] scalability tests exist;
 *     [ ] cross-domain tests exist;
 *     [ ] parser integration tests pass;
 *     [ ] documentation describes ownership;
 *     [ ] compatibility tests cover the public syntax.
 *
 * ============================================================================
 */
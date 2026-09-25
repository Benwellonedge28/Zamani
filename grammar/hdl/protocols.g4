/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hdl/protocols.g4
 *
 * Grammar:
 *     HdlProtocols
 *
 * Status:
 *     CANONICAL HDL PROTOCOL SYNTAX DELEGATE
 *
 * Purpose:
 *     Defines target-independent logical protocol contracts used by Zamani
 *     HDL interfaces and hardware descriptions.
 *
 * Architectural position:
 *
 *     Zamani Source
 *          |
 *          v
 *     grammar/Zamani.g4
 *          |
 *          v
 *     grammar/hdl/hdl.g4
 *          |
 *          +--> grammar/hdl/interfaces.g4
 *          |
 *          +--> grammar/hdl/protocols.g4  <--- THIS FILE
 *          |
 *          +--> grammar/hdl/ports.g4
 *          |
 *          +--> grammar/hdl/signals.g4
 *          |
 *          +--> grammar/hdl/clocks.g4
 *          |
 *          +--> grammar/hdl/clocking.g4
 *          |
 *          +--> grammar/hdl/timing.g4
 *          |
 *          v
 *     Domain-neutral semantic model
 *          |
 *          v
 *     HDL / hardware IR
 *          |
 *          v
 *     synthesis / simulation / verification / backend
 *
 * OWNERSHIP:
 *     This file owns the syntax of HDL logical protocol contracts.
 *
 * This file DOES own:
 *     - HDL protocol declarations;
 *     - protocol identity;
 *     - protocol generic parameters;
 *     - protocol inheritance/composition references;
 *     - protocol roles;
 *     - protocol message references;
 *     - protocol operation declarations;
 *     - protocol usage/reference declarations;
 *     - protocol requirements;
 *     - protocol provided capabilities/contracts;
 *     - protocol attributes;
 *     - protocol-level semantic constraints expressed as Zamani expressions.
 *
 * This file DOES NOT own:
 *     - generic networking protocol semantics;
 *     - transport implementations;
 *     - physical wires;
 *     - pins;
 *     - FPGA/ASIC placement;
 *     - routing;
 *     - clock implementation;
 *     - reset implementation;
 *     - timing implementation;
 *     - port declarations;
 *     - signal declarations;
 *     - message field definitions;
 *     - packet serialization;
 *     - device discovery;
 *     - runtime protocol execution;
 *     - vendor-specific protocols;
 *     - fixed hardware capacities.
 *
 * Those concerns remain owned by their respective grammar/semantic domains.
 *
 * IMPORTANT:
 *     This grammar is intentionally OPEN-WORLD.
 *
 *     It must never enumerate:
 *         - vendor protocols;
 *         - bus families;
 *         - transport technologies;
 *         - fixed lane counts;
 *         - fixed message counts;
 *         - fixed role counts;
 *         - fixed operation counts;
 *         - fixed interface widths;
 *         - fixed clock frequencies;
 *         - fixed device counts.
 *
 *     Protocol names are semantic identifiers, not a closed enumeration.
 *
 * PORTABILITY:
 *     A protocol declaration expresses logical communication/interaction
 *     intent. It does not select a physical realization.
 *
 *     For example, a protocol may ultimately be realized through:
 *
 *         CPU interconnect
 *         GPU interconnect
 *         FPGA fabric
 *         ASIC interconnect
 *         QPU control interface
 *         memory interface
 *         distributed transport
 *         future hardware
 *
 *     without changing the source-level protocol contract.
 *
 * SCALABILITY:
 *     Repetition operators (* and +) are used instead of fixed capacities.
 *     Cardinality is a semantic/resource concern rather than a grammar limit.
 *
 * POCO-REAF:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 *     Protocol syntax must describe WHAT communication/interaction contract
 *     exists rather than WHICH physical implementation realizes it.
 *
 * RESOURCE MODEL:
 *     Requirements such as:
 *
 *         requires capability(...)
 *         requires expression
 *
 *     are contracts.
 *
 *     They are not compiler-wide hardware limits.
 *
 * SAFETY:
 *     This grammar introduces no Rust implementation code and therefore
 *     introduces no unsafe Rust.
 *
 *     The Zamani implementation consuming this grammar MUST remain compatible
 *     with Rust 1.97 / Rust 1.97.1 and MUST NOT use unsafe.
 *
 * CANONICAL TOKEN VOCABULARY:
 *     ZamaniLexer
 *
 * No HDL-specific lexer is permitted here.
 *
 * SHARED RULES:
 *     The composition root supplies shared rules such as:
 *
 *         identifier
 *         qualifiedName
 *         attribute
 *         visibility
 *         expression
 *         genericParameters
 *         typeExpression
 *
 *     This file must not duplicate those rules.
 *
 * INTEGRATION CONTRACT:
 *
 *     grammar/hdl/hdl.g4 MUST import this grammar and expose:
 *
 *         hdlProtocolDeclaration
 *         hdlProtocolReference
 *         hdlProtocolMember
 *
 *     grammar/hdl/interfaces.g4 MUST delegate protocol members to:
 *
 *         hdlProtocolReference
 *         hdlProtocolContractReference
 *
 *     grammar/hdl/hardware-interfaces.g4 MUST NOT maintain an independent
 *     competing implementation of protocol declaration syntax.
 *
 * AST:
 *
 *     hdlProtocolDeclaration
 *         ->
 *     HdlProtocolDecl
 *
 *     hdlProtocolReference
 *         ->
 *     HdlProtocolRef
 *
 *     hdlProtocolRoleDeclaration
 *         ->
 *     HdlProtocolRole
 *
 *     hdlProtocolMessageReference
 *         ->
 *     HdlProtocolMessageRef
 *
 *     hdlProtocolOperationDeclaration
 *         ->
 *     HdlProtocolOperation
 *
 *     hdlProtocolUseDeclaration
 *         ->
 *     HdlProtocolUse
 *
 *     hdlProtocolRequirementClause
 *         ->
 *     HdlProtocolRequirement
 *
 *     hdlProtocolProvidesClause
 *         ->
 *     HdlProtocolProvides
 *
 * SEMANTIC MODEL:
 *     Protocol declarations are logical contracts.
 *
 *     Semantic analysis is responsible for:
 *         - name resolution;
 *         - duplicate-member detection;
 *         - protocol compatibility;
 *         - role consistency;
 *         - operation/message compatibility;
 *         - requirement validation;
 *         - capability validation;
 *         - interface compatibility;
 *         - protocol composition;
 *         - domain-specific legality.
 *
 *     None of those checks belong in the parser.
 *
 * IR:
 *     This grammar MUST NOT introduce a protocol-specific IR.
 *
 *     Protocol semantics lower through the existing canonical semantic model
 *     and then into the appropriate HDL/hardware/domain IR.
 *
 * QUANTUM:
 *     Quantum-facing protocols remain target-independent here.
 *
 *     Any quantum semantics eventually required by a protocol must flow through
 *     the existing quantum semantic pipeline and, where applicable, the
 *     canonical quantum::ir boundary.
 *
 * TESTING:
 *     Required tests:
 *
 *         - empty protocol;
 *         - one-role protocol;
 *         - many-role protocol;
 *         - one-message protocol;
 *         - many-message protocol;
 *         - one-operation protocol;
 *         - many-operation protocol;
 *         - parameterized protocol;
 *         - protocol composition;
 *         - protocol reference;
 *         - requirement clauses;
 *         - capability/provision clauses;
 *         - arbitrary protocol names;
 *         - qualified protocol names;
 *         - Unicode identifiers where supported;
 *         - deeply nested semantic expressions;
 *         - large protocol declarations;
 *         - malformed declarations;
 *         - duplicate semantic members;
 *         - missing identifiers;
 *         - invalid delimiters.
 *
 * HARD-CODING AUDIT:
 *     This grammar contains no universal:
 *
 *         MAX_PROTOCOLS
 *         MAX_ROLES
 *         MAX_MESSAGES
 *         MAX_OPERATIONS
 *         MAX_LANES
 *         MAX_CHANNELS
 *         MAX_PORTS
 *         MAX_DEVICES
 *         MAX_NODES
 *         MAX_WIDTH
 *         MAX_PROTOCOL_DEPTH
 *
 *     Any such limits belong, if needed, to an explicitly documented compiler
 *     resource policy and MUST NOT become language grammar limits.
 *
 * ============================================================================
 */

parser grammar HdlProtocols;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * PUBLIC ENTRY POINTS
 * ============================================================================
 *
 * These are the stable rules exported to grammar/hdl/hdl.g4 and the HDL
 * interface grammar.
 * ============================================================================
 */

/**
 * Complete HDL protocol declaration.
 *
 * Example:
 *
 *     protocol StreamProtocol<T> {
 *         role producer;
 *         role consumer;
 *
 *         message data;
 *         message completion;
 *
 *         operation transfer<T>;
 *
 *         requires capability("streaming");
 *         provides capability("flow_control");
 *     }
 */
hdlProtocolDeclaration
    : attribute*
      visibility?
      PROTOCOL
      identifier
      genericParameters?
      hdlProtocolInheritanceClause?
      hdlProtocolBody
    ;


/**
 * Reference to an already-declared HDL protocol.
 *
 * This does not redeclare the protocol.
 *
 * Example:
 *
 *     protocol network::StreamProtocol;
 */
hdlProtocolReference
    : attribute*
      USES
      qualifiedName
      SEMI
    ;


/**
 * Protocol body.
 *
 * The body is deliberately open-ended through repetition.
 */
hdlProtocolBody
    : LBRACE
      hdlProtocolMember*
      RBRACE
    ;


/**
 * Protocol members.
 *
 * Member ownership remains explicit.
 */
hdlProtocolMember
    : hdlProtocolRoleDeclaration
    | hdlProtocolMessageReference
    | hdlProtocolOperationDeclaration
    | hdlProtocolUseDeclaration
    | hdlProtocolRequirementClause
    | hdlProtocolProvidesClause
    ;


/*
 * ============================================================================
 * PROTOCOL COMPOSITION
 * ============================================================================
 */

/**
 * Protocol inheritance/composition.
 *
 * Multiple protocols may be composed.
 *
 * No fixed inheritance count is imposed.
 *
 * Example:
 *
 *     protocol Streaming
 *         extends BaseProtocol,
 *                 ReliabilityProtocol
 *     {
 *         ...
 *     }
 */
hdlProtocolInheritanceClause
    : K_EXTENDS
      hdlProtocolInheritanceTarget
      (
          COMMA
          hdlProtocolInheritanceTarget
      )*
    ;


hdlProtocolInheritanceTarget
    : qualifiedName
    ;


/*
 * ============================================================================
 * ROLES
 * ============================================================================
 *
 * Roles identify logical participants in a protocol.
 *
 * They do not identify physical devices, cores, nodes, pins, lanes, or
 * placement locations.
 * ============================================================================
 */

/**
 * Example:
 *
 *     role producer;
 *     role consumer : stream.Participant;
 */
hdlProtocolRoleDeclaration
    : attribute*
      ROLE
      identifier
      hdlProtocolRoleTypeClause?
      SEMI
    ;


hdlProtocolRoleTypeClause
    : COLON
      qualifiedName
    ;


/*
 * ============================================================================
 * MESSAGE REFERENCES
 * ============================================================================
 *
 * Message field layout belongs to the data/message grammar.
 *
 * This file only establishes that a protocol uses a message.
 * ============================================================================
 */

/**
 * Example:
 *
 *     message stream::Data;
 *     message stream::Completion : producer -> consumer;
 */
hdlProtocolMessageReference
    : attribute*
      MESSAGE
      qualifiedName
      hdlProtocolMessageDirectionClause?
      SEMI
    ;


hdlProtocolMessageDirectionClause
    : COLON
      hdlProtocolMessageEndpoint
      ARROW
      hdlProtocolMessageEndpoint
    ;


hdlProtocolMessageEndpoint
    : identifier
    | qualifiedName
    ;


/*
 * ============================================================================
 * OPERATIONS
 * ============================================================================
 *
 * Operations express logical protocol actions.
 *
 * They are not implementation functions and do not select a transport.
 * ============================================================================
 */

/**
 * Example:
 *
 *     operation transfer<T>;
 *     operation acknowledge;
 *
 * Parameters/return semantics may be attached by the semantic layer and
 * canonical function/type contracts rather than duplicated here.
 */
hdlProtocolOperationDeclaration
    : attribute*
      OPERATION
      identifier
      genericParameters?
      hdlProtocolOperationSignature?
      SEMI
    ;


hdlProtocolOperationSignature
    : LPAREN
      hdlProtocolOperationParameterList?
      RPAREN
      hdlProtocolOperationResultClause?
    ;


hdlProtocolOperationParameterList
    : hdlProtocolOperationParameter
      (
          COMMA
          hdlProtocolOperationParameter
      )*
    ;


hdlProtocolOperationParameter
    : identifier
      (
          COLON
          typeExpression
      )?
      (
          ASSIGN
          expression
      )?
    ;


hdlProtocolOperationResultClause
    : COLON
      typeExpression
    ;


/*
 * ============================================================================
 * PROTOCOL USE / COMPOSITION
 * ============================================================================
 *
 * This allows an HDL protocol to incorporate another protocol contract
 * without copying its declaration.
 * ============================================================================
 */

hdlProtocolUseDeclaration
    : attribute*
      USES
      qualifiedName
      SEMI
    ;


/*
 * ============================================================================
 * SEMANTIC REQUIREMENTS
 * ============================================================================
 *
 * Requirements describe conditions that must hold for a valid realization.
 *
 * Examples:
 *
 *     requires capability("flow_control");
 *     requires width >= payload_width;
 *     requires expression;
 *
 * The parser does not evaluate the expression.
 * ============================================================================
 */

hdlProtocolRequirementClause
    : attribute*
      K_REQUIRES
      expression
      SEMI
    ;


/*
 * ============================================================================
 * PROVIDED CAPABILITIES / CONTRACTS
 * ============================================================================
 *
 * Provides expresses semantic guarantees offered by the protocol.
 *
 * It does not bind those guarantees to a physical implementation.
 * ============================================================================
 */

hdlProtocolProvidesClause
    : attribute*
      K_PROVIDES
      expression
      SEMI
    ;


/*
 * ============================================================================
 * PROTOCOL CONTRACT REFERENCE
 * ============================================================================
 *
 * This narrow rule is intended for grammar/hdl/interfaces.g4 so an interface
 * can attach a protocol contract without redeclaring the protocol grammar.
 *
 * Example:
 *
 *     protocol network::stream;
 *
 * The exact interface-level marker remains owned by interfaces.g4.
 * ============================================================================
 */

hdlProtocolContractReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * END
 * ============================================================================
 */
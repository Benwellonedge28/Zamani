/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hdl/interfaces.g4
 *
 * Status:
 *     CANONICAL HDL LOGICAL-INTERFACE DELEGATE
 *
 * Purpose:
 *     Define target-independent logical hardware interface syntax.
 *
 * Language objective:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *     (POCO-REAF)
 *
 * Rust baseline:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *     Safe Rust only
 *     No unsafe Rust required
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
 *          v
 *     HDL composition
 *          |
 *          +--> interfaces.g4       <--- THIS FILE
 *          +--> ports.g4
 *          +--> signals.g4
 *          +--> wires.g4
 *          +--> registers.g4
 *          +--> memories.g4
 *          +--> clocks.g4
 *          +--> timing.g4
 *          +--> processes.g4
 *          +--> hardware-modules.g4
 *          +--> hardware-generics.g4
 *          +--> hardware-parameters.g4
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> name resolution
 *          +--> type checking
 *          +--> interface compatibility
 *          +--> protocol compatibility
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> portability validation
 *          |
 *          v
 *     canonical hardware semantic representation / IR
 *          |
 *          +--> optimization
 *          +--> verification
 *          +--> scheduling
 *          +--> routing
 *          +--> synthesis
 *          +--> target lowering
 *          |
 *          v
 *     target realization
 *
 * Grammar establishes syntax.
 * Semantic analysis establishes meaning.
 * Hardware compilation establishes realization.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - logical HDL interface declarations;
 *     - interface identity;
 *     - interface generic parameters;
 *     - interface specialization arguments;
 *     - interface inheritance/extension;
 *     - interface members;
 *     - interface-facing ports;
 *     - interface-facing signals;
 *     - interface parameters;
 *     - interface protocol declarations;
 *     - interface requirements;
 *     - interface guarantees;
 *     - interface capability declarations;
 *     - interface contract declarations;
 *     - interface implementation declarations;
 *     - logical interface references;
 *     - logical interface bindings;
 *     - logical interface connections;
 *     - interface composition.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer rules;
 *     - identifiers;
 *     - universal expressions;
 *     - universal types;
 *     - attributes;
 *     - physical pins;
 *     - package balls;
 *     - board locations;
 *     - device identifiers;
 *     - physical addresses;
 *     - FPGA regions;
 *     - ASIC coordinates;
 *     - routing;
 *     - placement;
 *     - scheduling;
 *     - synthesis;
 *     - calibration;
 *     - hardware discovery;
 *     - resource allocation;
 *     - runtime dispatch;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN.
 *
 * ============================================================================
 * IMPORTANT DISTINCTION
 * ============================================================================
 *
 * This file describes a LOGICAL interface contract.
 *
 * It does not describe a physical interface.
 *
 * Therefore:
 *
 *     interface Stream { ... }
 *
 * does not mean:
 *
 *     physical connector;
 *     package pin;
 *     FPGA I/O bank;
 *     ASIC pad;
 *     bus number;
 *     device address;
 *     physical routing path;
 *     vendor primitive.
 *
 * Those decisions belong downstream.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * An interface must remain valid independently of the eventual target.
 *
 * The same source-level interface may therefore be lowered to:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU-facing control
 *     embedded hardware
 *     distributed hardware
 *     future hardware
 *
 * provided the target can satisfy the semantic contract.
 *
 * ============================================================================
 * OPEN-WORLD / SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar imposes NO language-level maximum on:
 *
 *     interfaces;
 *     members;
 *     ports;
 *     signals;
 *     parameters;
 *     inherited interfaces;
 *     generic parameters;
 *     specialization arguments;
 *     protocol items;
 *     requirements;
 *     guarantees;
 *     capabilities;
 *     bindings;
 *     connections;
 *     interface instances;
 *     dimensions;
 *     widths;
 *     lanes;
 *     channels;
 *     nesting depth;
 *     composition breadth.
 *
 * Repetition is represented structurally with ANTLR repetition operators.
 *
 * "Infinity" means:
 *
 *     no artificial semantic ceiling is encoded by this grammar.
 *
 * Actual execution remains bounded by:
 *
 *     available resources;
 *     compiler resource policy;
 *     target capabilities;
 *     synthesis capacity;
 *     deployment constraints;
 *     runtime resources.
 *
 * Those limits MUST NOT be converted into language-level constants.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This file MUST NOT contain:
 *
 *     MAX_INTERFACES
 *     MAX_PORTS
 *     MAX_SIGNALS
 *     MAX_MEMBERS
 *     MAX_WIDTH
 *     MAX_LANES
 *     MAX_CHANNELS
 *     MAX_DEVICES
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *
 * It also MUST NOT encode:
 *
 *     wire [31:0]
 *     register<32>
 *     memory<64GB>
 *     FPGA_WITH_N_LUTS
 *     PHYSICAL_PIN_0
 *     QUBIT_0
 *
 * unless such values are explicitly supplied as ordinary program data.
 *
 * ============================================================================
 * REQUIREMENT / CAPABILITY / PREFERENCE / HINT
 * ============================================================================
 *
 * The grammar permits the syntax needed to express semantic intent.
 *
 * Semantic analysis distinguishes:
 *
 *     requirement
 *     capability
 *     constraint
 *     preference
 *     hint
 *     guarantee
 *     realization
 *
 * Example:
 *
 *     requires capability("streaming")
 *
 * means:
 *
 *     the eventual target must provide streaming capability.
 *
 * It does NOT mean:
 *
 *     use device X.
 *
 * ============================================================================
 * CANONICAL LEXER
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It consumes:
 *
 *     ZamaniLexer
 *
 * and MUST NOT define a second lexer.
 *
 * The lexical authority remains:
 *
 *     grammar/lexer/
 *     grammar/antlr/ZamaniLexer.g4
 *
 * ============================================================================
 * SHARED GRAMMAR CONTRACT
 * ============================================================================
 *
 * Universal syntax is imported from the canonical parser grammars:
 *
 *     Names
 *     Types
 *     Expressions
 *     Attributes
 *
 * This file therefore MUST NOT redefine:
 *
 *     identifier
 *     qualifiedName
 *     typeExpression
 *     expression
 *     attribute
 *     literals
 *     operator precedence
 *     lexical tokens.
 *
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * This grammar produces syntax only.
 *
 * It does not:
 *
 *     discover hardware;
 *     evaluate capabilities;
 *     select devices;
 *     allocate resources;
 *     route connections;
 *     schedule transfers;
 *     synthesize RTL;
 *     perform timing closure;
 *     access QPU state;
 *     execute runtime operations.
 *
 * ============================================================================
 */

parser grammar HdlInterfaces;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Types, Expressions, Attributes;


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINTS
 * ============================================================================
 *
 * These are the only public HDL interface entry points.
 *
 * The canonical HDL composition grammar should consume these rules.
 *
 * Public rules:
 *
 *     hdlInterfaceDeclaration
 *     hdlInterfaceImplementation
 *     hdlInterfaceReference
 *     hdlInterfaceBinding
 *     hdlInterfaceConnection
 *
 * ============================================================================
 */

hdlInterfaceDeclaration
    : hdlInterfaceAttribute*
      hdlInterfaceModifier*
      K_INTERFACE
      identifier
      hdlInterfaceGenericParameters?
      hdlInterfaceExtendsClause?
      hdlInterfaceWhereClause?
      hdlInterfaceBody
    ;


hdlInterfaceImplementation
    : hdlInterfaceAttribute*
      hdlInterfaceImplementationModifier*
      K_IMPLEMENTS
      hdlInterfaceImplementationTarget
      hdlInterfaceImplementationGenericArguments?
      hdlInterfaceImplementationBody
    ;


hdlInterfaceReference
    : qualifiedName
      hdlInterfaceGenericArguments?
    ;


hdlInterfaceBinding
    : hdlInterfaceAttribute*
      K_BIND
      hdlInterfaceBindingName?
      hdlInterfaceReference
      hdlInterfaceBindingBody?
      SEMICOLON
    ;


hdlInterfaceConnection
    : hdlInterfaceConnectionEndpoint
      hdlInterfaceConnectionOperator
      hdlInterfaceConnectionEndpoint
      hdlInterfaceConnectionAttribute*
      SEMICOLON
    ;


/*
 * ============================================================================
 * 2. INTERFACE DECLARATION
 * ============================================================================
 */

hdlInterfaceModifier
    : K_PUBLIC
    | K_PRIVATE
    | K_PROTECTED
    | K_INTERNAL
    | K_ABSTRACT
    | K_SEALED
    ;


hdlInterfaceAttribute
    : attribute
    ;


hdlInterfaceGenericParameters
    : LT
      hdlInterfaceGenericParameterList
      GT
    ;


hdlInterfaceGenericParameterList
    : hdlInterfaceGenericParameter
      (
          COMMA
          hdlInterfaceGenericParameter
      )*
      COMMA?
    ;


hdlInterfaceGenericParameter
    : identifier
      hdlInterfaceGenericParameterBound?
      hdlInterfaceGenericParameterDefault?
    ;


hdlInterfaceGenericParameterBound
    : COLON
      typeExpression
      (
          PLUS
          typeExpression
      )*
    ;


hdlInterfaceGenericParameterDefault
    : ASSIGN
      expression
    ;


hdlInterfaceExtendsClause
    : K_EXTENDS
      hdlInterfaceParentList
    ;


hdlInterfaceParentList
    : hdlInterfaceParent
      (
          COMMA
          hdlInterfaceParent
      )*
      COMMA?
    ;


hdlInterfaceParent
    : qualifiedName
      hdlInterfaceGenericArguments?
    ;


hdlInterfaceGenericArguments
    : LT
      hdlInterfaceGenericArgumentList
      GT
    ;


hdlInterfaceGenericArgumentList
    : hdlInterfaceGenericArgument
      (
          COMMA
          hdlInterfaceGenericArgument
      )*
      COMMA?
    ;


hdlInterfaceGenericArgument
    : typeExpression
    | expression
    ;


/*
 * ============================================================================
 * 3. WHERE CONSTRAINTS
 * ============================================================================
 *
 * Constraints are structural syntax.
 *
 * Their satisfiability is semantic.
 * ============================================================================
 */

hdlInterfaceWhereClause
    : K_WHERE
      hdlInterfaceWhereConstraintList
    ;


hdlInterfaceWhereConstraintList
    : hdlInterfaceWhereConstraint
      (
          COMMA
          hdlInterfaceWhereConstraint
      )*
      COMMA?
    ;


hdlInterfaceWhereConstraint
    : identifier
      COLON
      hdlInterfaceWhereBoundList
    ;


hdlInterfaceWhereBoundList
    : typeExpression
      (
          PLUS
          typeExpression
      )*
    ;


/*
 * ============================================================================
 * 4. INTERFACE BODY
 * ============================================================================
 */

hdlInterfaceBody
    : LBRACE
      hdlInterfaceMember*
      RBRACE
    ;


hdlInterfaceMember
    : hdlInterfaceMemberAttribute*
      hdlInterfaceMemberCore
    ;


hdlInterfaceMemberAttribute
    : attribute
    ;


hdlInterfaceMemberCore
    : hdlInterfacePort
    | hdlInterfaceSignal
    | hdlInterfaceParameter
    | hdlInterfaceProtocol
    | hdlInterfaceRequirement
    | hdlInterfaceGuarantee
    | hdlInterfaceCapability
    | hdlInterfaceContract
    | hdlInterfaceNestedInterface
    ;


/*
 * ============================================================================
 * 5. INTERFACE PORT
 * ============================================================================
 *
 * Port syntax itself remains owned by ports.g4.
 *
 * This adapter consumes the port contract rather than duplicating it.
 *
 * The canonical HDL parser assembly must make `hdlPortDeclarationItem`
 * available from HardwarePorts.
 * ============================================================================
 */

hdlInterfacePort
    : hdlPortDeclarationItem
      SEMICOLON
    ;


/*
 * ============================================================================
 * 6. INTERFACE SIGNAL
 * ============================================================================
 *
 * Signal semantics remain owned by signals.g4.
 *
 * The interface grammar only provides the interface-facing declaration
 * boundary.
 * ============================================================================
 */

hdlInterfaceSignal
    : K_SIGNAL
      identifier
      (
          COLON
          typeExpression
      )?
      hdlInterfaceDimension*
      hdlInterfaceInitializer?
      SEMICOLON
    ;


hdlInterfaceDimension
    : LBRACKET
      expression?
      RBRACKET
    ;


hdlInterfaceInitializer
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * 7. INTERFACE PARAMETERS
 * ============================================================================
 */

hdlInterfaceParameter
    : K_PARAMETER
      identifier
      (
          COLON
          typeExpression
      )?
      (
          ASSIGN
          expression
      )?
      SEMICOLON
    ;


/*
 * ============================================================================
 * 8. PROTOCOLS
 * ============================================================================
 *
 * Protocol names are intentionally open-world.
 *
 * The grammar does not enumerate:
 *
 *     AXI
 *     AMBA
 *     PCIe
 *     Ethernet
 *     custom protocols
 *
 * as universal language constructs.
 *
 * Protocol identity is semantic data.
 * ============================================================================
 */

hdlInterfaceProtocol
    : K_PROTOCOL
      qualifiedName
      hdlInterfaceProtocolParameters?
      hdlInterfaceProtocolBody?
      SEMICOLON?
    ;


hdlInterfaceProtocolParameters
    : LPAREN
      hdlInterfaceProtocolParameterList?
      RPAREN
    ;


hdlInterfaceProtocolParameterList
    : hdlInterfaceProtocolParameter
      (
          COMMA
          hdlInterfaceProtocolParameter
      )*
      COMMA?
    ;


hdlInterfaceProtocolParameter
    : identifier
      (
          ASSIGN
          expression
      )?
    ;


hdlInterfaceProtocolBody
    : LBRACE
      hdlInterfaceProtocolItem*
      RBRACE
    ;


hdlInterfaceProtocolItem
    : hdlInterfaceProtocolAttribute*
      hdlInterfaceProtocolItemCore
    ;


hdlInterfaceProtocolAttribute
    : attribute
    ;


hdlInterfaceProtocolItemCore
    : hdlInterfaceProtocolProperty
    | hdlInterfaceProtocolRequirement
    | hdlInterfaceProtocolGuarantee
    | hdlInterfaceProtocolMessage
    ;


hdlInterfaceProtocolProperty
    : identifier
      ASSIGN
      expression
      SEMICOLON
    ;


hdlInterfaceProtocolRequirement
    : K_REQUIRES
      expression
      SEMICOLON
    ;


hdlInterfaceProtocolGuarantee
    : K_GUARANTEES
      expression
      SEMICOLON
    ;


hdlInterfaceProtocolMessage
    : K_MESSAGE
      identifier
      (
          COLON
          typeExpression
      )?
      SEMICOLON
    ;


/*
 * ============================================================================
 * 9. REQUIREMENTS
 * ============================================================================
 *
 * Requirements express semantic conditions.
 *
 * They do not select a physical target.
 * ============================================================================
 */

hdlInterfaceRequirement
    : K_REQUIRES
      hdlInterfaceRequirementBody
    ;


hdlInterfaceRequirementBody
    : LPAREN
      expression
      RPAREN
      SEMICOLON
    | expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 10. GUARANTEES
 * ============================================================================
 */

hdlInterfaceGuarantee
    : K_GUARANTEES
      hdlInterfaceGuaranteeBody
    ;


hdlInterfaceGuaranteeBody
    : LPAREN
      expression
      RPAREN
      SEMICOLON
    | expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 11. CAPABILITIES
 * ============================================================================
 *
 * Capability names are open-world.
 *
 * This grammar does not enumerate GPU/FPGA/QPU/vendor capabilities.
 * ============================================================================
 */

hdlInterfaceCapability
    : K_CAPABILITY
      qualifiedName
      hdlInterfaceCapabilityArguments?
      SEMICOLON
    ;


hdlInterfaceCapabilityArguments
    : LPAREN
      hdlInterfaceCapabilityArgumentList?
      RPAREN
    ;


hdlInterfaceCapabilityArgumentList
    : hdlInterfaceCapabilityArgument
      (
          COMMA
          hdlInterfaceCapabilityArgument
      )*
      COMMA?
    ;


hdlInterfaceCapabilityArgument
    : expression
    ;


/*
 * ============================================================================
 * 12. CONTRACTS
 * ============================================================================
 *
 * Contracts remain source-level expressions.
 *
 * Evaluation and proof belong downstream.
 * ============================================================================
 */

hdlInterfaceContract
    : K_CONTRACT
      LBRACE
      hdlInterfaceContractItem*
      RBRACE
    ;


hdlInterfaceContractItem
    : hdlInterfaceRequiresClause
    | hdlInterfaceEnsuresClause
    | hdlInterfaceInvariantClause
    ;


hdlInterfaceRequiresClause
    : K_REQUIRES
      LPAREN
      expression
      RPAREN
      SEMICOLON
    ;


hdlInterfaceEnsuresClause
    : K_ENSURES
      LPAREN
      expression
      RPAREN
      SEMICOLON
    ;


hdlInterfaceInvariantClause
    : K_INVARIANT
      LPAREN
      expression
      RPAREN
      SEMICOLON
    ;


/*
 * ============================================================================
 * 13. NESTED INTERFACES
 * ============================================================================
 *
 * Nested logical interfaces are legal.
 *
 * No nesting depth is imposed by this grammar.
 * ============================================================================
 */

hdlInterfaceNestedInterface
    : hdlInterfaceDeclaration
    ;


/*
 * ============================================================================
 * 14. IMPLEMENTATIONS
 * ============================================================================
 *
 * An implementation says that a hardware construct conforms to an interface
 * contract.
 *
 * It does not describe physical placement.
 * ============================================================================
 */

hdlInterfaceImplementationModifier
    : K_PUBLIC
    | K_PRIVATE
    | K_INTERNAL
    ;


hdlInterfaceImplementationTarget
    : qualifiedName
    ;


hdlInterfaceImplementationGenericArguments
    : hdlInterfaceGenericArguments
    ;


hdlInterfaceImplementationBody
    : LBRACE
      hdlInterfaceImplementationItem*
      RBRACE
    ;


hdlInterfaceImplementationItem
    : hdlInterfaceImplementationAttribute*
      hdlInterfaceImplementationItemCore
    ;


hdlInterfaceImplementationAttribute
    : attribute
    ;


hdlInterfaceImplementationItemCore
    : hdlInterfaceBinding
    | hdlInterfaceConnection
    | hdlInterfaceRequirement
    | hdlInterfaceGuarantee
    | hdlInterfaceContract
    ;


/*
 * ============================================================================
 * 15. BINDINGS
 * ============================================================================
 *
 * Bindings associate a logical interface with another logical interface
 * instance/reference.
 *
 * They do NOT perform physical placement.
 * ============================================================================
 */

hdlInterfaceBindingName
    : identifier
    ;


hdlInterfaceBindingBody
    : LBRACE
      hdlInterfaceBindingItem*
      RBRACE
    ;


hdlInterfaceBindingItem
    : hdlInterfaceBindingAssociation
    | hdlInterfaceBindingAttribute
    ;


hdlInterfaceBindingAssociation
    : identifier
      ASSIGN
      expression
      SEMICOLON
    ;


hdlInterfaceBindingAttribute
    : attribute
    ;


/*
 * ============================================================================
 * 16. CONNECTIONS
 * ============================================================================
 *
 * A connection represents logical connectivity.
 *
 * It does NOT represent:
 *
 *     physical route;
 *     wire segment;
 *     package pin;
 *     FPGA resource;
 *     board trace.
 *
 * The semantic layer resolves compatibility.
 * ============================================================================
 */

hdlInterfaceConnectionEndpoint
    : qualifiedName
    ;


hdlInterfaceConnectionOperator
    : ASSIGN
    | K_CONNECT
    ;


hdlInterfaceConnectionAttribute
    : attribute
    ;


/*
 * ============================================================================
 * 17. SOURCE-LEVEL INTERFACE ADAPTER
 * ============================================================================
 *
 * The following rule gives the HDL composition layer a single declaration
 * boundary when it needs an interface as a generic HDL member.
 * ============================================================================
 */

hdlInterfaceItem
    : hdlInterfaceDeclaration
    | hdlInterfaceImplementation
    | hdlInterfaceBinding
    | hdlInterfaceConnection
    ;


/*
 * ============================================================================
 * 18. CROSS-DOMAIN INTEGRATION
 * ============================================================================
 *
 * Classical:
 *
 *     Interfaces may expose classical data and control types.
 *
 * Quantum:
 *
 *     Interfaces may expose quantum-facing semantic types/capabilities.
 *
 * Hybrid:
 *
 *     Interfaces may connect classical and quantum components.
 *
 * Distributed:
 *
 *     Interfaces may describe logical communication contracts.
 *
 * AI/data:
 *
 *     Interfaces may expose tensor/data abstractions.
 *
 * Hardware:
 *
 *     Hardware realization consumes the logical contract.
 *
 * None of these domains may introduce physical realization into this grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 19. QUANTUM BOUNDARY
 * ============================================================================
 *
 * This grammar MUST NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     quantum gates
 *     circuits
 *     quantum operations
 *     QEC structures
 *     ZQN structures.
 *
 * If an interface uses a quantum type, capability, or requirement:
 *
 *     HDL interface syntax
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     canonical quantum semantic representation
 *          |
 *          v
 *     quantum::ir
 *
 * There is no second quantum IR.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 20. RESOURCE BOUNDARY
 * ============================================================================
 *
 * Interface requirements such as:
 *
 *     requires memory >= required_memory;
 *
 *     requires capability("streaming");
 *
 *     requires capability("quantum.measurement");
 *
 * remain semantic requirements.
 *
 * They are NOT:
 *
 *     device selectors;
 *     physical addresses;
 *     topology selectors;
 *     fixed hardware mappings.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 21. TARGET INDEPENDENCE
 * ============================================================================
 *
 * This grammar does not contain:
 *
 *     CPU identifiers;
 *     GPU identifiers;
 *     FPGA identifiers;
 *     QPU identifiers;
 *     physical pin numbers;
 *     board identifiers;
 *     vendor placement;
 *     routing coordinates;
 *     fixed topology.
 *
 * Such information belongs to target/deployment descriptions downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 22. DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source text;
 *     grammar version;
 *     canonical lexer;
 *     parser configuration;
 *     explicitly selected language/dialect configuration.
 *
 * Parsing does not inspect:
 *
 *     hardware;
 *     filesystem;
 *     network;
 *     environment;
 *     runtime;
 *     wall clock;
 *     randomness.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 23. DIAGNOSTICS
 * ============================================================================
 *
 * The parser must preserve normal ANTLR token/source locations.
 *
 * Semantic diagnostics should be able to identify:
 *
 *     interface;
 *     member;
 *     port;
 *     signal;
 *     protocol;
 *     requirement;
 *     guarantee;
 *     capability;
 *     binding;
 *     connection;
 *     implementation.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 24. SAFETY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no embedded Rust;
 *     no actions;
 *     no semantic predicates;
 *     no filesystem access;
 *     no network access;
 *     no hardware access;
 *     no runtime execution;
 *     no unsafe code.
 *
 * Rust consumers remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *     safe Rust only.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 25. COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] Logical interface syntax has one owner.
 * [x] No lexer rules are defined here.
 * [x] Canonical ZamaniLexer is consumed.
 * [x] Canonical Names grammar is reused.
 * [x] Canonical Types grammar is reused.
 * [x] Canonical Expressions grammar is reused.
 * [x] Canonical Attributes grammar is reused.
 * [x] Interface generics are open-ended.
 * [x] Interface inheritance is open-ended.
 * [x] Interface members are open-ended.
 * [x] Interface ports are delegated to ports.g4.
 * [x] No physical pins are represented.
 * [x] No physical routing is represented.
 * [x] No device identifiers are required.
 * [x] Protocol identities are open-world.
 * [x] Capability identities are open-world.
 * [x] Requirements remain semantic.
 * [x] Guarantees remain semantic.
 * [x] Connections remain logical.
 * [x] Implementations remain logical.
 * [x] Quantum semantics remain outside this grammar.
 * [x] quantum::ir remains the canonical quantum boundary.
 * [x] No fixed machine/resource limits exist.
 * [x] No vendor implementation is embedded.
 * [x] No runtime behavior exists.
 * [x] No unsafe Rust dependency exists.
 *
 * Remaining integration work is deliberately external to this file:
 *
 *     hdl.g4
 *     hardware-interfaces.g4
 *     declarations/declarations.g4
 *     tests/hdl/
 *
 * must each consume this file according to the ownership contract.
 *
 * ============================================================================
 */
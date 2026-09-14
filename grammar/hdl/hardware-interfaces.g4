/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hdl/hardware-interfaces.g4
 *
 * Purpose:
 *     Canonical parser delegate for logical HDL/hardware interface
 *     declarations, interface members, interface composition, interface
 *     implementations, interface references, interface bindings, and
 *     interface-facing connections.
 *
 * Language objective:
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no embedded Rust code.
 *     No unsafe Rust is used or required.
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
 *          +--> ports.g4
 *          +--> signals.g4
 *          +--> hardware-interfaces.g4   <--- THIS FILE
 *          +--> hardware-generics.g4
 *          +--> hardware-parameters.g4
 *          +--> clocks.g4
 *          +--> timing.g4
 *          +--> processes.g4
 *          +--> ...
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> name resolution
 *          +--> type checking
 *          +--> interface compatibility
 *          +--> protocol compatibility
 *          +--> direction checking
 *          +--> capability checking
 *          +--> resource analysis
 *          +--> target-independent validation
 *          |
 *          v
 *     canonical hardware semantic representation / IR
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
 *     - logical hardware interface declarations;
 *     - interface identity;
 *     - interface generic/specialization syntax;
 *     - interface-level semantic requirements;
 *     - interface-level semantic guarantees;
 *     - interface attributes;
 *     - interface member composition;
 *     - interface inheritance/extension syntax;
 *     - interface implementation declarations;
 *     - interface references;
 *     - interface instance/binding syntax;
 *     - interface connection syntax;
 *     - logical interface contracts;
 *     - interface-facing grouping syntax;
 *     - interface protocol annotations at syntax level.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer/token definitions;
 *     - identifiers;
 *     - universal types;
 *     - universal expressions;
 *     - generic parameter semantics;
 *     - hardware module declarations;
 *     - ports;
 *     - signals;
 *     - wires;
 *     - registers;
 *     - clocks;
 *     - timing;
 *     - memories;
 *     - pipelines;
 *     - processes;
 *     - state machines;
 *     - physical pins;
 *     - physical addresses;
 *     - board mappings;
 *     - device identifiers;
 *     - vendor identifiers;
 *     - topology;
 *     - placement;
 *     - routing;
 *     - scheduling;
 *     - synthesis;
 *     - calibration;
 *     - hardware discovery;
 *     - resource allocation;
 *     - runtime dispatch;
 *     - quantum::ir;
 *     - QEC algorithms;
 *     - ZQN noise semantics.
 *
 * Those concepts remain owned by their respective repository subsystems.
 *
 * ============================================================================
 * POCO-REAF PRINCIPLE
 * ============================================================================
 *
 * An interface describes a LOGICAL CONTRACT.
 *
 * It does not inherently describe:
 *
 *     - a physical connector;
 *     - a package pin;
 *     - a board;
 *     - an FPGA bank;
 *     - an ASIC pad;
 *     - a device address;
 *     - a bus number;
 *     - a particular processor;
 *     - a particular quantum processor;
 *     - a particular vendor;
 *     - a particular topology;
 *     - a fixed machine size.
 *
 * Physical realization is selected by downstream target/resource/deployment
 * systems.
 *
 * ============================================================================
 * SCALABILITY INVARIANT
 * ============================================================================
 *
 * This grammar intentionally has NO limits on:
 *
 *     number of interfaces;
 *     number of members;
 *     number of ports;
 *     number of inherited interfaces;
 *     number of implementations;
 *     number of bindings;
 *     number of connections;
 *     number of generic parameters;
 *     number of dimensions;
 *     number of protocol properties;
 *     interface nesting depth;
 *     interface composition breadth.
 *
 * Repetition is used wherever cardinality is naturally unbounded.
 *
 * Actual resource limits belong to:
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
 * IMPORTANT DISTINCTION
 * ============================================================================
 *
 * These concepts are intentionally different:
 *
 *     interface
 *     interface member
 *     requirement
 *     capability
 *     constraint
 *     preference
 *     hint
 *     target
 *     placement
 *     physical binding
 *
 * An interface requirement MUST NOT silently become a physical target
 * selection.
 *
 * For example:
 *
 *     requires protocol::stream
 *
 * does not mean:
 *
 *     use device X
 *     use board Y
 *     use pin Z
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * This is a PARSER grammar.
 *
 * It consumes the canonical lexer vocabulary.
 *
 * The canonical parser/build system must determine the authoritative token
 * vocabulary. This delegate assumes the canonical vocabulary is exposed as:
 *
 *     ZamaniLexer
 *
 * If the repository's final lexer name differs, the parser build configuration
 * MUST change the tokenVocab declaration consistently across the grammar
 * assembly. This file must not introduce a second lexer.
 *
 * ============================================================================
 * SHARED-RULE CONTRACT
 * ============================================================================
 *
 * The final canonical parser should supply shared rules for:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     typeExpr
 *     genericParameters
 *     genericArguments
 *     attribute
 *     visibility
 *
 * This delegate therefore keeps interface-specific rules separate from those
 * universal language concepts.
 *
 * Where an independently testable adapter is necessary, it is deliberately
 * named `hdlInterface...` so it cannot be mistaken for the canonical
 * universal rule.
 *
 * ============================================================================
 */

parser grammar HardwareInterfaces;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINTS
 * ============================================================================
 *
 * The canonical parser may import these rules:
 *
 *     hdlInterfaceDeclaration
 *     hdlInterfaceImplementation
 *     hdlInterfaceReference
 *     hdlInterfaceBinding
 *
 * No other HDL grammar is permitted to redefine these entry points.
 * ============================================================================
 */

hdlInterfaceDeclaration
    : hdlInterfaceHeader
      hdlInterfaceBody
    ;


hdlInterfaceImplementation
    : hdlInterfaceImplementationHeader
      hdlInterfaceImplementationBody
    ;


hdlInterfaceReference
    : hdlInterfaceQualifiedName
      hdlInterfaceSpecializationArguments?
    ;


hdlInterfaceBinding
    : hdlInterfaceBindingHeader
      hdlInterfaceBindingBody?
      SEMICOLON
    ;


/*
 * ============================================================================
 * 2. INTERFACE DECLARATION
 * ============================================================================
 *
 * Conceptual forms:
 *
 *     interface Stream {
 *         ...
 *     }
 *
 *     hdl interface Stream {
 *         ...
 *     }
 *
 *     interface Bus<T> {
 *         ...
 *     }
 *
 * The exact domain introducer policy is resolved by the canonical HDL parser.
 *
 * This delegate uses the dedicated INTERFACE token when available.
 * A contextual identifier fallback is intentionally NOT included here because
 * unrestricted IDENTIFIER alternatives create ambiguity and can consume
 * unrelated declarations.
 * ============================================================================
 */

hdlInterfaceHeader
    : INTERFACE
      hdlInterfaceName
      hdlInterfaceGenerics?
      hdlInterfaceParameters?
      hdlInterfaceExtends?
      hdlInterfaceRequirements?
      hdlInterfaceGuarantees?
    ;


hdlInterfaceName
    : identifier
    ;


hdlInterfaceQualifiedName
    : qualifiedName
    ;


/*
 * ============================================================================
 * 3. INTERFACE GENERICS
 * ============================================================================
 *
 * Interface generics express structural/type-level abstraction.
 *
 * They do not establish physical hardware capacity.
 *
 * Examples:
 *
 *     interface Stream<T> { ... }
 *     interface Bus<WIDTH> { ... }
 *     interface Matrix<ROWS, COLS, ELEMENT> { ... }
 *
 * No finite generic count is imposed.
 *
 * The preferred production integration is to reuse the universal generic
 * parameter rule supplied by the canonical type/declaration grammar.
 * ============================================================================
 */

hdlInterfaceGenerics
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
      (
          COLON
          typeExpr
      )?
      (
          ASSIGN
          expression
      )?
    ;


/*
 * ============================================================================
 * 4. INTERFACE PARAMETERS
 * ============================================================================
 *
 * Parameters configure an interface declaration without embedding a target.
 *
 * Example:
 *
 *     interface Bus(parameter WIDTH = width) { ... }
 *
 * A parameter may describe logical structure, protocol configuration, or
 * compile-time specialization.
 *
 * It does not automatically represent a physical machine property.
 * ============================================================================
 */

hdlInterfaceParameters
    : LPAREN
      hdlInterfaceParameterList?
      RPAREN
    ;


hdlInterfaceParameterList
    : hdlInterfaceParameter
      (
          COMMA
          hdlInterfaceParameter
      )*
      COMMA?
    ;


hdlInterfaceParameter
    : PARAMETER
      identifier
      (
          COLON
          typeExpr
      )?
      (
          ASSIGN
          expression
      )?
    ;


/*
 * ============================================================================
 * 5. INTERFACE INHERITANCE / EXTENSION
 * ============================================================================
 *
 * Interface extension is logical contract composition.
 *
 * It does not imply physical inheritance or hardware duplication.
 *
 * Example:
 *
 *     interface AdvancedStream extends Stream, Control { ... }
 *
 * No inheritance-depth limit is encoded.
 * ============================================================================
 */

hdlInterfaceExtends
    : EXTENDS
      hdlInterfaceBaseList
    ;


hdlInterfaceBaseList
    : hdlInterfaceReference
      (
          COMMA
          hdlInterfaceReference
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 6. REQUIREMENTS
 * ============================================================================
 *
 * Requirements state properties needed by the interface.
 *
 * They are semantic constraints, not hardware selection commands.
 *
 * Examples:
 *
 *     requires {
 *         width >= WIDTH;
 *         protocol == "stream";
 *     }
 *
 * The expression itself is interpreted by semantic analysis.
 * ============================================================================
 */

hdlInterfaceRequirements
    : REQUIRES
      LBRACE
      hdlInterfaceRequirementEntry*
      RBRACE
    ;


hdlInterfaceRequirementEntry
    : expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 7. GUARANTEES
 * ============================================================================
 *
 * Guarantees state logical properties promised by an interface.
 *
 * They do not automatically promise:
 *
 *     performance;
 *     latency;
 *     power;
 *     throughput;
 *     physical timing.
 *
 * Such properties must be represented explicitly by their appropriate
 * semantic/resource models.
 * ============================================================================
 */

hdlInterfaceGuarantees
    : ENSURES
      LBRACE
      hdlInterfaceGuaranteeEntry*
      RBRACE
    ;


hdlInterfaceGuaranteeEntry
    : expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 8. INTERFACE BODY
 * ============================================================================
 *
 * The interface body contains logical interface members.
 *
 * This file delegates concrete member declarations to their owning grammars.
 *
 * IMPORTANT:
 *
 * The exact token/rule names below are integration points. The canonical HDL
 * parser MUST assemble these delegates so that each rule has exactly one owner.
 *
 * ============================================================================
 */

hdlInterfaceBody
    : LBRACE
      hdlInterfaceMember*
      RBRACE
    ;


hdlInterfaceMember
    : hdlInterfaceAttribute
    | hdlPortDeclaration
    | hdlSignalDeclaration
    | hdlInterfaceRequirementMember
    | hdlInterfaceMethod
    | hdlInterfaceTypeMember
    | hdlInterfaceNestedInterface
    | hdlInterfaceReferenceMember
    ;


/*
 * ============================================================================
 * 9. INTERFACE ATTRIBUTES
 * ============================================================================
 *
 * Attributes are metadata.
 *
 * They must not silently select:
 *
 *     vendor;
 *     device;
 *     board;
 *     physical pin;
 *     address;
 *     topology.
 *
 * Any attribute with target-specific semantics must be validated and lowered
 * through the target/deployment contract rather than interpreted by this
 * grammar.
 * ============================================================================
 */

hdlInterfaceAttribute
    : AT
      identifier
      (
          LPAREN
          hdlInterfaceAttributeArgumentList?
          RPAREN
      )?
    ;


hdlInterfaceAttributeArgumentList
    : hdlInterfaceAttributeArgument
      (
          COMMA
          hdlInterfaceAttributeArgument
      )*
      COMMA?
    ;


hdlInterfaceAttributeArgument
    : identifier
      ASSIGN
      expression
    | expression
    ;


/*
 * ============================================================================
 * 10. REQUIREMENT MEMBERS
 * ============================================================================
 *
 * An interface may expose named semantic requirements as members.
 *
 * This is deliberately distinct from the interface-level `requires { ... }`
 * block:
 *
 *     interface declaration contract
 *
 * versus:
 *
 *     reusable named requirement member.
 * ============================================================================
 */

hdlInterfaceRequirementMember
    : REQUIRE
      identifier
      (
          COLON
          typeExpr
      )?
      (
          ASSIGN
          expression
      )?
      SEMICOLON
    ;


/*
 * ============================================================================
 * 11. INTERFACE METHODS / BEHAVIORAL CONTRACTS
 * ============================================================================
 *
 * An interface may describe operations expected at the boundary.
 *
 * This grammar describes syntax only.
 *
 * It does not decide:
 *
 *     implementation;
 *     scheduling;
 *     latency;
 *     execution device;
 *     calling convention;
 *     synthesis strategy.
 * ============================================================================
 */

hdlInterfaceMethod
    : hdlInterfaceMethodModifiers*
      FN
      identifier
      hdlInterfaceMethodGenerics?
      LPAREN
      hdlInterfaceParameterListWithoutParameterKeyword?
      RPAREN
      (
          ARROW
          typeExpr
      )?
      hdlInterfaceMethodEffects?
      SEMICOLON
    ;


hdlInterfaceMethodModifiers
    : identifier
    ;


hdlInterfaceMethodGenerics
    : LT
      hdlInterfaceGenericParameterList
      GT
    ;


hdlInterfaceParameterListWithoutParameterKeyword
    : hdlInterfaceMethodParameter
      (
          COMMA
          hdlInterfaceMethodParameter
      )*
      COMMA?
    ;


hdlInterfaceMethodParameter
    : identifier
      (
          COLON
          typeExpr
      )?
      (
          ASSIGN
          expression
      )?
    ;


hdlInterfaceMethodEffects
    : WITH
      EFFECTS
      LBRACE
      hdlInterfaceEffectNameList?
      RBRACE
    ;


hdlInterfaceEffectNameList
    : identifier
      (
          COMMA
          identifier
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 12. INTERFACE TYPE MEMBERS
 * ============================================================================
 *
 * Type members allow an interface to expose associated semantic types.
 *
 * This is not a second type system.
 *
 * The universal type system owns type semantics.
 * ============================================================================
 */

hdlInterfaceTypeMember
    : TYPE
      identifier
      (
          ASSIGN
          typeExpr
      )?
      SEMICOLON
    ;


/*
 * ============================================================================
 * 13. NESTED INTERFACES
 * ============================================================================
 *
 * Nested interfaces are logical namespace/contract composition.
 *
 * They do not create physical hierarchy.
 * ============================================================================
 */

hdlInterfaceNestedInterface
    : hdlInterfaceDeclaration
    ;


/*
 * ============================================================================
 * 14. REFERENCE MEMBERS
 * ============================================================================
 *
 * A named interface reference allows composition without duplicating the
 * referenced interface declaration.
 * ============================================================================
 */

hdlInterfaceReferenceMember
    : INTERFACE
      identifier
      COLON
      hdlInterfaceReference
      SEMICOLON
    ;


/*
 * ============================================================================
 * 15. INTERFACE IMPLEMENTATION
 * ============================================================================
 *
 * Conceptual form:
 *
 *     implements Stream for StreamEngine {
 *         ...
 *     }
 *
 * The implementation target is a LOGICAL MODULE/type name.
 *
 * It is not a device selector.
 * ============================================================================
 */

hdlInterfaceImplementationHeader
    : IMPLEMENTS
      hdlInterfaceReference
      FOR
      qualifiedName
      hdlInterfaceImplementationGenerics?
    ;


hdlInterfaceImplementationGenerics
    : LT
      hdlInterfaceSpecializationArgumentList
      GT
    ;


hdlInterfaceImplementationBody
    : LBRACE
      hdlInterfaceImplementationMember*
      RBRACE
    ;


hdlInterfaceImplementationMember
    : hdlInterfaceAttribute
    | hdlInterfaceMethodImplementation
    | hdlInterfaceBinding
    | hdlInterfaceImplementationRequirement
    ;


hdlInterfaceMethodImplementation
    : FN
      identifier
      LPAREN
      hdlInterfaceMethodArgumentList?
      RPAREN
      (
          ARROW
          typeExpr
      )?
      block
    ;


hdlInterfaceMethodArgumentList
    : hdlInterfaceMethodArgument
      (
          COMMA
          hdlInterfaceMethodArgument
      )*
      COMMA?
    ;


hdlInterfaceMethodArgument
    : identifier
      (
          COLON
          typeExpr
      )?
      (
          ASSIGN
          expression
      )?
    ;


hdlInterfaceImplementationRequirement
    : REQUIRES
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 16. INTERFACE SPECIALIZATION
 * ============================================================================
 *
 * Specialization arguments configure a declared interface.
 *
 * Examples:
 *
 *     Stream<DATA>
 *     Bus<WIDTH = DATA_WIDTH>
 *
 * No fixed argument count is imposed.
 * ============================================================================
 */

hdlInterfaceSpecializationArguments
    : LT
      hdlInterfaceSpecializationArgumentList
      GT
    ;


hdlInterfaceSpecializationArgumentList
    : hdlInterfaceSpecializationArgument
      (
          COMMA
          hdlInterfaceSpecializationArgument
      )*
      COMMA?
    ;


hdlInterfaceSpecializationArgument
    : identifier
      ASSIGN
      expression
    | expression
    ;


/*
 * ============================================================================
 * 17. INTERFACE BINDING
 * ============================================================================
 *
 * A binding establishes a source-level relationship between a logical
 * interface and another logical interface/member.
 *
 * It does NOT establish physical placement or routing.
 *
 * Examples:
 *
 *     bind producer to consumer;
 *
 *     bind stream_if to engine.stream;
 *
 * Physical realization remains downstream.
 * ============================================================================
 */

hdlInterfaceBindingHeader
    : BIND
      hdlInterfaceBindingSource
      TO
      hdlInterfaceBindingTarget
    ;


hdlInterfaceBindingSource
    : hdlInterfaceReference
    | hdlInterfaceAccess
    ;


hdlInterfaceBindingTarget
    : hdlInterfaceReference
    | hdlInterfaceAccess
    ;


hdlInterfaceBindingBody
    : LBRACE
      hdlInterfaceBindingEntry*
      RBRACE
    ;


hdlInterfaceBindingEntry
    : identifier
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 18. INTERFACE ACCESS
 * ============================================================================
 *
 * Interface access permits a logical interface instance/member to be
 * addressed without exposing physical representation.
 * ============================================================================
 */

hdlInterfaceAccess
    : hdlInterfaceReference
    | hdlInterfaceMemberAccess
    ;


hdlInterfaceMemberAccess
    : hdlInterfaceQualifiedName
      (
          DOT
          identifier
      )+
    ;


/*
 * ============================================================================
 * 19. INTERFACE CONNECTIONS
 * ============================================================================
 *
 * Connection syntax represents logical compatibility/binding.
 *
 * It is deliberately not called "routing".
 *
 * Routing belongs to downstream hardware realization.
 * ============================================================================
 */

hdlInterfaceConnection
    : hdlInterfaceConnectionSource
      CONNECT
      hdlInterfaceConnectionTarget
      SEMICOLON?
    ;


hdlInterfaceConnectionSource
    : hdlInterfaceAccess
    | hdlPortDeclarationReference
    ;


hdlInterfaceConnectionTarget
    : hdlInterfaceAccess
    | hdlPortDeclarationReference
    ;


/*
 * ============================================================================
 * 20. PORT REFERENCE ADAPTER
 * ============================================================================
 *
 * The port grammar owns port declarations.
 *
 * This adapter only provides the interface-facing reference form.
 *
 * It must not redefine hdlPortDeclaration.
 * ============================================================================
 */

hdlPortDeclarationReference
    : hdlInterfaceQualifiedName
      (
          DOT
          identifier
      )*
    ;


/*
 * ============================================================================
 * 21. PROTOCOL CONTRACT
 * ============================================================================
 *
 * Protocol is intentionally represented as a logical contract.
 *
 * Examples:
 *
 *     protocol "stream"
 *     protocol StreamProtocol
 *     protocol::stream
 *
 * The actual protocol semantics belong to semantic analysis/dialect
 * registration, not to this grammar.
 * ============================================================================
 */

hdlInterfaceProtocol
    : PROTOCOL
      hdlInterfaceProtocolReference
      hdlInterfaceProtocolArguments?
      SEMICOLON
    ;


hdlInterfaceProtocolReference
    : hdlInterfaceQualifiedName
    | STRING_LITERAL
    ;


hdlInterfaceProtocolArguments
    : LPAREN
      hdlInterfaceProtocolArgumentList?
      RPAREN
    ;


hdlInterfaceProtocolArgumentList
    : hdlInterfaceProtocolArgument
      (
          COMMA
          hdlInterfaceProtocolArgument
      )*
      COMMA?
    ;


hdlInterfaceProtocolArgument
    : identifier
      ASSIGN
      expression
    | expression
    ;


/*
 * ============================================================================
 * 22. INTERFACE CONTRACT MEMBER
 * ============================================================================
 *
 * A contract member makes protocol/compatibility requirements explicit while
 * remaining independent of physical realization.
 * ============================================================================
 */

hdlInterfaceContract
    : CONTRACT
      identifier
      (
          LPAREN
          hdlInterfaceContractArgumentList?
          RPAREN
      )?
      SEMICOLON
    ;


hdlInterfaceContractArgumentList
    : hdlInterfaceContractArgument
      (
          COMMA
          hdlInterfaceContractArgument
      )*
      COMMA?
    ;


hdlInterfaceContractArgument
    : identifier
      ASSIGN
      expression
    | expression
    ;


/*
 * ============================================================================
 * 23. INTERFACE CAPABILITY DECLARATION
 * ============================================================================
 *
 * Capability syntax identifies a semantic capability required or exposed by
 * an interface.
 *
 * IMPORTANT:
 *
 * This does NOT discover hardware.
 *
 * It does NOT query the runtime.
 *
 * It does NOT select a device.
 *
 * Capability resolution belongs to semantic/resource/target analysis.
 * ============================================================================
 */

hdlInterfaceCapability
    : CAPABILITY
      hdlInterfaceCapabilityReference
      (
          LPAREN
          hdlInterfaceCapabilityArgumentList?
          RPAREN
      )?
      SEMICOLON
    ;


hdlInterfaceCapabilityReference
    : hdlInterfaceQualifiedName
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
    : identifier
      ASSIGN
      expression
    | expression
    ;


/*
 * ============================================================================
 * 24. INTERFACE RESOURCE REQUIREMENT
 * ============================================================================
 *
 * Resource requirements are logical declarations.
 *
 * Examples:
 *
 *     resource throughput >= required_rate;
 *     resource latency <= allowed_latency;
 *
 * This grammar only parses the declaration.
 *
 * Resource semantics belong to grammar/resources and the repository's
 * resource/capability semantic layer.
 *
 * There is deliberately no machine count or physical resource identifier.
 * ============================================================================
 */

hdlInterfaceResourceRequirement
    : RESOURCE
      identifier
      (
          COLON
          typeExpr
      )?
      (
          ASSIGN
          expression
      )?
      SEMICOLON
    ;


/*
 * ============================================================================
 * 25. INTERFACE PREFERENCE
 * ============================================================================
 *
 * Preferences are non-mandatory guidance.
 *
 * They must never be treated as semantic requirements merely because they
 * occur in the same interface declaration.
 * ============================================================================
 */

hdlInterfacePreference
    : PREFERENCE
      identifier
      (
          ASSIGN
          expression
      )?
      SEMICOLON
    ;


/*
 * ============================================================================
 * 26. INTERFACE HINT
 * ============================================================================
 *
 * Hints are optional implementation guidance.
 *
 * A hint is not a guarantee and is not a hard requirement.
 * ============================================================================
 */

hdlInterfaceHint
    : HINT
      identifier
      (
          ASSIGN
          expression
      )?
      SEMICOLON
    ;


/*
 * ============================================================================
 * 27. INTERFACE MEMBER EXTENSION POINT
 * ============================================================================
 *
 * This explicit extension point permits future HDL dialects to introduce
 * interface members without modifying this core grammar every time a new
 * semantic category is standardized.
 *
 * Dialect resolution belongs to dialect registration/semantic validation.
 *
 * The identifier is intentionally namespace-qualified rather than being a
 * free-form arbitrary token sequence.
 * ============================================================================
 */

hdlInterfaceDialectMember
    : hdlInterfaceQualifiedName
      (
          LPAREN
          hdlInterfaceDialectArgumentList?
          RPAREN
      )?
      SEMICOLON
    ;


hdlInterfaceDialectArgumentList
    : hdlInterfaceDialectArgument
      (
          COMMA
          hdlInterfaceDialectArgument
      )*
      COMMA?
    ;


hdlInterfaceDialectArgument
    : identifier
      ASSIGN
      expression
    | expression
    ;


/*
 * ============================================================================
 * 28. COMPLETE INTERFACE MEMBER COMPOSITION
 * ============================================================================
 *
 * These members are included here only where their semantic ownership is
 * clearly interface-specific.
 *
 * Concrete port/signal declarations remain owned by their respective grammar
 * delegates.
 * ============================================================================
 */

hdlInterfaceOwnedMember
    : hdlInterfaceAttribute
    | hdlInterfaceProtocol
    | hdlInterfaceContract
    | hdlInterfaceCapability
    | hdlInterfaceResourceRequirement
    | hdlInterfacePreference
    | hdlInterfaceHint
    | hdlInterfaceRequirementMember
    | hdlInterfaceMethod
    | hdlInterfaceTypeMember
    | hdlInterfaceNestedInterface
    | hdlInterfaceReferenceMember
    | hdlInterfaceDialectMember
    ;


/*
 * ============================================================================
 * 29. INTEGRATED MEMBER ROOT
 * ============================================================================
 *
 * The canonical parser should use this composition root when it assembles
 * all HDL delegates.
 *
 * Rules supplied by other delegates:
 *
 *     hdlPortDeclaration       <- ports.g4
 *     hdlSignalDeclaration     <- signals.g4
 *     hdlWireDeclaration       <- wires.g4
 *     hdlRegisterDeclaration   <- registers.g4
 *     hdlClockDeclaration      <- clocks.g4
 *     hdlTimingDeclaration     <- timing.g4
 *     hdlProcessDeclaration    <- processes.g4
 *     hdlMemoryDeclaration     <- memories.g4
 *     hdlPipelineDeclaration   <- pipelines.g4
 *
 * No implementation details from those files are duplicated here.
 * ============================================================================
 */

hdlInterfaceIntegratedMember
    : hdlInterfaceOwnedMember
    | hdlPortDeclaration
    | hdlSignalDeclaration
    | hdlWireDeclaration
    | hdlRegisterDeclaration
    | hdlClockDeclaration
    | hdlTimingDeclaration
    | hdlProcessDeclaration
    | hdlMemoryDeclaration
    | hdlPipelineDeclaration
    ;


/*
 * ============================================================================
 * 30. PRODUCTION INTERFACE BODY
 * ============================================================================
 *
 * This rule is the preferred body rule for final parser assembly.
 *
 * If the canonical HDL parser already has a separate member composition root,
 * that root may replace this rule, but ownership must remain unchanged.
 * ============================================================================
 */

hdlInterfaceProductionBody
    : LBRACE
      hdlInterfaceIntegratedMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 31. INTERFACE DECLARATION USING PRODUCTION BODY
 * ============================================================================
 *
 * Canonical production form.
 * ============================================================================
 */

hdlInterfaceProductionDeclaration
    : hdlInterfaceHeader
      hdlInterfaceProductionBody
    ;


/*
 * ============================================================================
 * 32. INTERFACE INSTANCE REFERENCE
 * ============================================================================
 *
 * Logical interface instances can be referenced without identifying a
 * physical resource.
 * ============================================================================
 */

hdlInterfaceInstanceReference
    : hdlInterfaceReference
      (
          DOT
          identifier
      )*
    ;


/*
 * ============================================================================
 * 33. INTERFACE CONNECTION GROUP
 * ============================================================================
 *
 * Connection groups support many logical connections without fixed cardinality.
 * ============================================================================
 */

hdlInterfaceConnectionGroup
    : CONNECT
      LBRACE
      hdlInterfaceConnection*
      RBRACE
    ;


/*
 * ============================================================================
 * 34. INTERFACE COMPATIBILITY ASSERTION
 * ============================================================================
 *
 * This syntax expresses an assertion about compatibility.
 *
 * It does not perform the compatibility check.
 *
 * Semantic analysis owns:
 *
 *     type compatibility;
 *     direction compatibility;
 *     protocol compatibility;
 *     capability compatibility;
 *     parameter compatibility.
 * ============================================================================
 */

hdlInterfaceCompatibilityAssertion
    : ASSERT
      COMPATIBLE
      hdlInterfaceAccess
      WITH
      hdlInterfaceAccess
      SEMICOLON
    ;


/*
 * ============================================================================
 * 35. INTERFACE ADAPTER DECLARATION
 * ============================================================================
 *
 * Adapters describe logical conversion between two interfaces.
 *
 * The adapter does not dictate:
 *
 *     placement;
 *     routing;
 *     clocking;
 *     physical implementation;
 *     device choice.
 * ============================================================================
 */

hdlInterfaceAdapter
    : ADAPTER
      identifier
      COLON
      hdlInterfaceReference
      TO
      hdlInterfaceReference
      hdlInterfaceAdapterBody
    ;


hdlInterfaceAdapterBody
    : LBRACE
      hdlInterfaceAdapterMember*
      RBRACE
    ;


hdlInterfaceAdapterMember
    : hdlInterfaceBinding
    | hdlInterfaceMethodImplementation
    | hdlInterfaceCompatibilityAssertion
    | hdlInterfaceAttribute
    ;


/*
 * ============================================================================
 * 36. INTERFACE COMPOSITION
 * ============================================================================
 *
 * Composition combines logical interfaces.
 *
 * It is not physical aggregation.
 * ============================================================================
 */

hdlInterfaceComposition
    : COMPOSE
      hdlInterfaceReferenceList
      SEMICOLON
    ;


hdlInterfaceReferenceList
    : hdlInterfaceReference
      (
          COMMA
          hdlInterfaceReference
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 37. INTERFACE EXPORT / EXPOSURE
 * ============================================================================
 *
 * Exporting an interface makes a logical contract visible outside its local
 * declaration scope.
 *
 * It does not expose physical hardware details.
 * ============================================================================
 */

hdlInterfaceExport
    : EXPORT
      hdlInterfaceReference
      SEMICOLON
    ;


/*
 * ============================================================================
 * 38. INTERFACE IMPORT / REQUIREMENT
 * ============================================================================
 *
 * Imports resolve through the module/package system.
 *
 * This grammar does not perform filesystem or network access.
 * ============================================================================
 */

hdlInterfaceImport
    : IMPORT
      hdlInterfaceQualifiedName
      SEMICOLON
    ;


/*
 * ============================================================================
 * 39. INTEGRATION CONTRACT
 * ============================================================================
 *
 * DOWNSTREAM CONSUMERS
 *
 *     Frontend AST
 *         |
 *         +--> semantic/name resolution
 *         +--> type checking
 *         +--> interface compatibility
 *         +--> protocol validation
 *         +--> capability validation
 *         +--> resource analysis
 *         |
 *         v
 *     canonical hardware semantic representation / IR
 *         |
 *         +--> optimization
 *         +--> scheduling
 *         +--> routing
 *         +--> synthesis
 *         +--> target lowering
 *         |
 *         v
 *     hardware abstraction / runtime
 *
 * THIS FILE MUST NOT:
 *
 *     -> construct IR;
 *     -> call runtime APIs;
 *     -> discover hardware;
 *     -> choose devices;
 *     -> schedule;
 *     -> route;
 *     -> synthesize;
 *     -> calibrate;
 *     -> query QPU state;
 *     -> access filesystem/network resources;
 *     -> perform compilation itself.
 *
 * ============================================================================
 * 40. QUANTUM INTEGRATION BOUNDARY
 * ============================================================================
 *
 * An interface may describe a logical quantum-facing boundary.
 *
 * However:
 *
 *     quantum syntax
 *         -> quantum frontend/AST
 *         -> quantum semantic analysis
 *         -> quantum::ir
 *
 * remains the canonical quantum path.
 *
 * This grammar MUST NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     quantum gate representation
 *     quantum circuit IR
 *     QEC representation
 *     ZQN noise representation.
 *
 * If an interface exposes quantum capabilities, the semantic layer maps those
 * declarations to the appropriate quantum subsystem.
 *
 * ============================================================================
 * 41. RESOURCE INTEGRATION BOUNDARY
 * ============================================================================
 *
 * Interface resource declarations are syntax only.
 *
 * Resource semantics belong to:
 *
 *     grammar/resources/
 *     semantic resource analysis
 *     hardware capability analysis
 *     target selection
 *     scheduling
 *     deployment
 *
 * This grammar must never convert:
 *
 *     resource requirement
 *
 * into:
 *
 *     fixed device;
 *     fixed device count;
 *     fixed topology;
 *     fixed address.
 *
 * ============================================================================
 * 42. HARDWARE HAL BOUNDARY
 * ============================================================================
 *
 * Hardware abstraction owns actual machine facts.
 *
 * This grammar owns only logical interface syntax.
 *
 * Therefore there is intentionally no dependency:
 *
 *     hardware-interfaces.g4 -> hardware discovery
 *
 * The dependency is:
 *
 *     parsed interface
 *         -> semantic interface model
 *         -> hardware capability matching
 *         -> HAL
 *         -> target realization
 *
 * ============================================================================
 * 43. SCHEDULING BOUNDARY
 * ============================================================================
 *
 * Interface declarations do not establish a schedule.
 *
 * Timing requirements may be expressed as semantic constraints, but:
 *
 *     ordering
 *     resource allocation
 *     start times
 *     alignment
 *     latency realization
 *     dynamic scheduling
 *
 * belong to scheduling/timing layers.
 *
 * ============================================================================
 * 44. ROUTING / PLACEMENT BOUNDARY
 * ============================================================================
 *
 * Logical interface connections do not establish physical routing.
 *
 * Therefore this grammar contains no:
 *
 *     pin numbers
 *     routing coordinates
 *     FPGA locations
 *     ASIC coordinates
 *     physical bus numbers
 *     fixed topology.
 *
 * ============================================================================
 * 45. OPTIMIZATION BOUNDARY
 * ============================================================================
 *
 * The grammar does not optimize interface declarations.
 *
 * Optimization consumes semantic representations after parsing.
 *
 * ============================================================================
 * 46. DETERMINISM
 * ============================================================================
 *
 * Parsing must be deterministic for a fixed source, lexer vocabulary, grammar
 * version, and parser configuration.
 *
 * No semantic lookup, hardware query, random choice, runtime inspection, or
 * external I/O is permitted inside this grammar.
 *
 * ============================================================================
 * 47. ERROR HANDLING
 * ============================================================================
 *
 * Syntax errors are reported by ANTLR's parser/error strategy.
 *
 * Semantic errors belong to semantic analysis.
 *
 * Examples of semantic errors that MUST NOT be encoded as parser actions:
 *
 *     incompatible interface types;
 *     incompatible directions;
 *     unavailable capability;
 *     impossible resource requirement;
 *     unsupported protocol;
 *     unsatisfied implementation contract;
 *     unavailable target.
 *
 * ============================================================================
 * 48. SCALABILITY
 * ============================================================================
 *
 * There is deliberately no:
 *
 *     MAX_INTERFACES
 *     MAX_MEMBERS
 *     MAX_PORTS
 *     MAX_SIGNALS
 *     MAX_PARAMETERS
 *     MAX_CONNECTIONS
 *     MAX_PROTOCOLS
 *     MAX_GENERIC_PARAMETERS
 *     MAX_INHERITANCE_DEPTH
 *     MAX_INTERFACE_WIDTH
 *
 * Any implementation limit is external to language semantics.
 *
 * Large programs may therefore be represented using:
 *
 *     repetition;
 *     generics;
 *     symbolic expressions;
 *     composition;
 *     modules;
 *     generated declarations;
 *     dialects.
 *
 * "Infinity" here means the grammar imposes no artificial finite semantic
 * ceiling; actual execution remains bounded by available implementation
 * resources, compiler resource policy, and target capabilities.
 *
 * ============================================================================
 * 49. SECURITY
 * ============================================================================
 *
 * This grammar performs no:
 *
 *     filesystem access;
 *     network access;
 *     process execution;
 *     device access;
 *     runtime evaluation;
 *     hardware discovery.
 *
 * Expressions appearing in declarations are syntax trees only.
 *
 * Evaluation belongs to a controlled semantic/compile-time subsystem.
 *
 * ============================================================================
 * 50. NO UNSAFE
 * ============================================================================
 *
 * This file contains no Rust code.
 *
 * The Rust frontend/compiler integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * and must not require unsafe Rust for parsing or interface representation.
 *
 * ============================================================================
 * 51. COMPATIBILITY
 * ============================================================================
 *
 * Interface syntax is versioned through the language-version and grammar
 * compatibility system.
 *
 * Existing valid interface forms must be preserved or migrated through an
 * explicit compatibility mechanism.
 *
 * New interface members should preferably be introduced through:
 *
 *     dialects;
 *     versioned grammar extensions;
 *     semantic capability registration.
 *
 * ============================================================================
 * 52. TEST REQUIREMENTS
 * ============================================================================
 *
 * Positive tests MUST cover:
 *
 *     simple interface;
 *     parameterized interface;
 *     generic interface;
 *     interface inheritance;
 *     interface composition;
 *     interface methods;
 *     protocol declarations;
 *     capability declarations;
 *     resource requirements;
 *     logical bindings;
 *     interface implementations;
 *     interface adapters;
 *     interface compatibility assertions;
 *     nested interfaces;
 *     qualified names;
 *     symbolic widths;
 *     symbolic dimensions.
 *
 * Negative tests MUST cover:
 *
 *     missing interface name;
 *     malformed generic arguments;
 *     malformed specialization;
 *     malformed inheritance;
 *     malformed bindings;
 *     missing delimiters;
 *     invalid member syntax;
 *     malformed protocol declarations;
 *     malformed capability declarations;
 *     malformed implementation declarations.
 *
 * Boundary tests MUST cover:
 *
 *     zero interface members;
 *     one member;
 *     many members;
 *     deeply composed interfaces;
 *     large generic lists;
 *     large connection lists;
 *     symbolic expressions of arbitrary practical size.
 *
 * Scalability tests MUST verify that source grammar contains no fixed
 * interface/member/resource/device limits.
 *
 * Cross-domain tests MUST cover:
 *
 *     classical + HDL interface;
 *     quantum + HDL interface;
 *     hybrid quantum/classical interface;
 *     accelerator interface;
 *     distributed interface;
 *     HDL + hardware capability interface.
 *
 * ============================================================================
 * 53. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when:
 *
 * [ ] It parses every interface form assigned to this grammar's ownership.
 *
 * [ ] It does not redefine universal names/types/expressions.
 *
 * [ ] It does not redefine port/signal/wire/register/clock/timing semantics.
 *
 * [ ] It has no machine-size constants.
 *
 * [ ] It has no physical device identifiers.
 *
 * [ ] It has no topology assumptions.
 *
 * [ ] It has no runtime/hardware discovery.
 *
 * [ ] It has no semantic actions.
 *
 * [ ] It has no unsafe Rust dependency.
 *
 * [ ] Its lexer contract is satisfied by the canonical lexer.
 *
 * [ ] Its shared-rule contract is satisfied by the canonical parser assembly.
 *
 * [ ] Its AST contract is documented and stable.
 *
 * [ ] Interface requirements/capabilities/resources remain semantic data rather
 *     than target selectors.
 *
 * [ ] Logical bindings remain separate from routing/placement.
 *
 * [ ] Quantum-facing declarations terminate at the quantum semantic boundary
 *     and do not create a second quantum IR.
 *
 * [ ] Resource declarations integrate with the universal resource model.
 *
 * [ ] Interface declarations can be lowered to the canonical hardware
 *     semantic representation.
 *
 * [ ] Positive, negative, boundary, determinism, round-trip, scalability and
 *     cross-domain tests exist.
 *
 * [ ] ANTLR generation succeeds with the repository's canonical lexer/parser
 *     assembly.
 *
 * [ ] Rust 1.97 / 1.97.1 repository builds succeed.
 *
 * [ ] No downstream file requires changing this grammar's ownership model.
 *
 * ============================================================================
 */
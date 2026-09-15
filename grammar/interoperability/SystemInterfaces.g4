/*
 * ============================================================================
 * Zamani Universal System Interfaces Grammar
 * ============================================================================
 *
 * Canonical source:
 *     grammar/interoperability/SystemInterfaces.g4
 *
 * Compatibility/source-tree alias:
 *     grammar/interoperability/system-interfaces.g4
 *
 * IMPORTANT:
 * ANTLR requires the grammar name to correspond to the .g4 filename.
 *
 * Architectural role
 * ------------------
 *
 * Zamani source
 *      |
 *      v
 * canonical lexer/parser
 *      |
 *      v
 * system-interface syntax
 *      |
 *      v
 * semantic analysis
 *      |
 *      +-------------------+
 *      |                   |
 *      v                   v
 * capabilities         resources
 * effects              targets
 *      |                   |
 *      +---------+---------+
 *                |
 *                v
 *          canonical semantic IR
 *                |
 *        +-------+--------+
 *        |                |
 *        v                v
 *     compiler          runtime
 *
 * OWNERSHIP
 * ---------
 *
 * This grammar owns:
 *
 *   - abstract system-interface declarations;
 *   - services;
 *   - abstract operations;
 *   - events;
 *   - interrupt-like semantic events;
 *   - opaque handles;
 *   - abstract resources;
 *   - capabilities;
 *   - requirements;
 *   - effects references;
 *   - lifecycle declarations;
 *   - system properties;
 *   - implementation-neutral bindings;
 *   - interface metadata.
 *
 * This grammar does NOT own:
 *
 *   - syscall numbers;
 *   - syscall instruction encodings;
 *   - CPU registers;
 *   - physical addresses;
 *   - MMIO addresses;
 *   - interrupt-vector numbers;
 *   - PCI/device identifiers;
 *   - processor identifiers;
 *   - CPU/core/thread counts;
 *   - GPU/FPGA/ASIC counts;
 *   - machine topology;
 *   - scheduling;
 *   - routing;
 *   - placement;
 *   - calibration;
 *   - ABI layout;
 *   - linker semantics;
 *   - filesystem access;
 *   - network access;
 *   - dynamic library loading;
 *   - runtime discovery;
 *   - hardware discovery;
 *   - QEC;
 *   - ZQN;
 *   - resilience;
 *   - canonical quantum IR;
 *   - canonical classical IR.
 *
 * POCO-REAF
 * ---------
 *
 * This grammar expresses WHAT an interface provides or requires.
 *
 * It deliberately does not permanently encode HOW or WHERE the interface
 * is implemented.
 *
 * Consequently:
 *
 *   requires capability("quantum.execute")
 *
 * is not:
 *
 *   use quantum device N
 *
 * and:
 *
 *   requires capability("memory.allocate")
 *
 * is not:
 *
 *   use address A
 *
 * Physical implementation belongs to target/resource/backend/runtime layers.
 *
 * SCALABILITY
 * -----------
 *
 * No finite source-level maximum is imposed on:
 *
 *   interfaces
 *   services
 *   operations
 *   parameters
 *   resources
 *   events
 *   handles
 *   capabilities
 *   nodes
 *   devices
 *   processors
 *   qubits
 *   memory
 *   storage
 *   execution contexts
 *
 * Any real limit is supplied by semantic validation, target description,
 * resource discovery, compilation, scheduling, deployment, or runtime.
 *
 * SECURITY
 * --------
 *
 * This grammar performs no external action.
 *
 * Parsing MUST NOT:
 *
 *   - execute a syscall;
 *   - access memory;
 *   - open a file;
 *   - open a socket;
 *   - enumerate devices;
 *   - inspect hardware;
 *   - load a library;
 *   - resolve a process;
 *   - contact a runtime;
 *   - discover resources.
 *
 * RUST
 * ----
 *
 * Intended repository environment:
 *
 *   Rust 1.97 / 1.97.1
 *   Edition 2021
 *   antlr-rust 0.3.0-beta
 *   unsafe forbidden
 *
 * No embedded Rust actions, predicates, or unsafe code are used.
 *
 * ============================================================================
 */

grammar SystemInterfaces;

options {
    language = Rust;
}


/*
 * ============================================================================
 * TOP LEVEL
 * ============================================================================
 */

compilationUnit
    : systemInterfaceDeclaration* EOF
    ;


systemInterfaceDeclaration
    : annotation*
      visibilityModifier?
      SYSTEM INTERFACE
      qualifiedName
      genericParameterClause?
      interfaceVersionClause?
      interfaceMetadata*
      LBRACE
      systemInterfaceMember*
      RBRACE
    ;


interfaceVersionClause
    : VERSION stringLiteral
    ;


interfaceMetadata
    : REQUIREMENT systemRequirementExpression
    | EFFECTS systemEffectBlock
    | ATTRIBUTES LBRACE systemAttribute* RBRACE
    ;


/*
 * ============================================================================
 * INTERFACE MEMBERS
 * ============================================================================
 */

systemInterfaceMember
    : systemServiceDeclaration
    | systemOperationDeclaration
    | systemEventDeclaration
    | systemInterruptDeclaration
    | systemHandleDeclaration
    | systemResourceDeclaration
    | systemCapabilityDeclaration
    | systemRequirementDeclaration
    | systemConstantDeclaration
    | systemTypeDeclaration
    | systemLifecycleDeclaration
    | systemPropertyDeclaration
    | systemBindingDeclaration
    ;


/*
 * ============================================================================
 * SERVICES
 * ============================================================================
 */

systemServiceDeclaration
    : annotation*
      visibilityModifier?
      SERVICE
      identifier
      genericParameterClause?
      serviceMetadata*
      LBRACE
      systemServiceMember*
      RBRACE
    ;


serviceMetadata
    : REQUIREMENT systemRequirementExpression
    | EFFECTS systemEffectBlock
    | ATTRIBUTES LBRACE systemAttribute* RBRACE
    ;


systemServiceMember
    : systemOperationDeclaration
    | systemEventDeclaration
    | systemInterruptDeclaration
    | systemHandleDeclaration
    | systemResourceDeclaration
    | systemCapabilityDeclaration
    | systemRequirementDeclaration
    | systemConstantDeclaration
    | systemPropertyDeclaration
    | systemLifecycleDeclaration
    ;


/*
 * ============================================================================
 * OPERATIONS
 * ============================================================================
 *
 * Operations are abstract callable contracts.
 *
 * They may ultimately lower to:
 *
 *   syscall
 *   hypercall
 *   RPC
 *   kernel message
 *   monitor service
 *   embedded service
 *   device API
 *   accelerator API
 *   quantum-runtime operation
 *   distributed service
 *   simulator interface
 *   future execution mechanism
 *
 * The choice is NOT made by this grammar.
 */

systemOperationDeclaration
    : annotation*
      visibilityModifier?
      operationModifier*
      ASYNC?
      FN
      identifier
      genericParameterClause?
      LPAREN
      parameterList?
      RPAREN
      returnClause?
      effectClause?
      requirementClause?
      availabilityClause?
      operationMetadata*
      SEMICOLON
    ;


operationModifier
    : BLOCKING
    | NONBLOCKING
    | IDEMPOTENT
    | CANCELLABLE
    | STREAMING
    | TRANSACTIONAL
    | PURE
    ;


returnClause
    : ARROW typeExpression
    ;


effectClause
    : EFFECTS systemEffectBlock
    ;


requirementClause
    : REQUIREMENT systemRequirementExpression
    ;


availabilityClause
    : AVAILABLE WHEN expression
    ;


operationMetadata
    : ATTRIBUTES LBRACE systemAttribute* RBRACE
    ;


/*
 * ============================================================================
 * EVENTS
 * ============================================================================
 */

systemEventDeclaration
    : annotation*
      EVENT
      identifier
      eventPayload?
      effectClause?
      requirementClause?
      availabilityClause?
      SEMICOLON
    ;


eventPayload
    : LPAREN
      parameterList?
      RPAREN
    ;


/*
 * ============================================================================
 * INTERRUPT-LIKE SEMANTIC EVENTS
 * ============================================================================
 *
 * No physical IRQ/vector/device number is allowed here.
 */

systemInterruptDeclaration
    : annotation*
      INTERRUPT
      identifier
      eventPayload?
      interruptTrigger?
      effectClause?
      requirementClause?
      availabilityClause?
      SEMICOLON
    ;


interruptTrigger
    : ON expression
    ;


/*
 * ============================================================================
 * HANDLES
 * ============================================================================
 */

systemHandleDeclaration
    : annotation*
      HANDLE
      identifier
      handleType?
      handleOwnership?
      handleMetadata*
      SEMICOLON
    ;


handleType
    : COLON typeExpression
    ;


handleOwnership
    : OWNED
    | SHARED
    | BORROWED
    | SCOPED
    | OPAQUE
    ;


handleMetadata
    : ATTRIBUTES LBRACE systemAttribute* RBRACE
    ;


/*
 * ============================================================================
 * RESOURCES
 * ============================================================================
 *
 * Resources are semantic resources, not physical inventory declarations.
 */

systemResourceDeclaration
    : annotation*
      RESOURCE
      identifier
      resourceType?
      resourceConstraint*
      requirementClause?
      resourceMetadata*
      SEMICOLON
    ;


resourceType
    : COLON typeExpression
    ;


resourceConstraint
    : WHERE expression
    ;


resourceMetadata
    : ATTRIBUTES LBRACE systemAttribute* RBRACE
    ;


/*
 * ============================================================================
 * CAPABILITIES
 * ============================================================================
 *
 * Capability vocabulary is open-ended.
 *
 * No finite enum of hardware/software capabilities is embedded in the grammar.
 */

systemCapabilityDeclaration
    : annotation*
      CAPABILITY
      qualifiedName
      capabilityParameters?
      capabilityMetadata*
      SEMICOLON
    ;


capabilityParameters
    : LPAREN
      argumentList?
      RPAREN
    ;


capabilityMetadata
    : requirementClause
    | effectClause
    | availabilityClause
    | attributeClause
    ;


/*
 * ============================================================================
 * REQUIREMENTS
 * ============================================================================
 */

systemRequirementDeclaration
    : annotation*
      REQUIREMENT
      systemRequirementExpression
      SEMICOLON
    ;


systemRequirementExpression
    : capabilityRequirement
    | resourceRequirement
    | featureRequirement
    | targetIndependentConstraint
    | expression
    ;


capabilityRequirement
    : CAPABILITY LPAREN
      qualifiedName
      optionalArgumentList
      RPAREN
    ;


resourceRequirement
    : RESOURCE LPAREN
      qualifiedName
      optionalArgumentList
      RPAREN
    ;


featureRequirement
    : FEATURE LPAREN
      qualifiedName
      optionalArgumentList
      RPAREN
    ;


targetIndependentConstraint
    : CONSTRAINT LPAREN
      expression
      RPAREN
    ;


optionalArgumentList
    : COMMA argumentList
    | /* empty */
    ;


/*
 * ============================================================================
 * EFFECTS
 * ============================================================================
 *
 * Effect names remain extensible.
 *
 * The canonical effects subsystem remains the semantic owner.
 */

systemEffectBlock
    : LBRACE
      systemEffectExpression*
      RBRACE
    ;


systemEffectExpression
    : qualifiedName
      optionalCallArguments
      SEMICOLON?
    ;


optionalCallArguments
    : LPAREN
      argumentList?
      RPAREN
    | /* empty */
    ;


/*
 * ============================================================================
 * CONSTANTS
 * ============================================================================
 */

systemConstantDeclaration
    : annotation*
      CONST
      identifier
      COLON
      typeExpression
      EQUAL
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * OPAQUE SYSTEM TYPES
 * ============================================================================
 *
 * This does not replace Zamani's canonical type system.
 *
 * It introduces an interface-level declaration that can later lower to the
 * canonical type representation.
 */

systemTypeDeclaration
    : annotation*
      TYPE
      identifier
      genericParameterClause?
      typeAliasClause?
      typeMetadata*
      SEMICOLON
    ;


typeAliasClause
    : EQUAL
      typeExpression
    ;


typeMetadata
    : ATTRIBUTES LBRACE systemAttribute* RBRACE
    ;


/*
 * ============================================================================
 * LIFECYCLE
 * ============================================================================
 */

systemLifecycleDeclaration
    : annotation*
      LIFECYCLE
      identifier
      LBRACE
      lifecycleOperation*
      RBRACE
    ;


lifecycleOperation
    : ACQUIRE
      optionalLifecycleSignature
      SEMICOLON
    | RELEASE
      optionalLifecycleSignature
      SEMICOLON
    | START
      optionalLifecycleSignature
      SEMICOLON
    | STOP
      optionalLifecycleSignature
      SEMICOLON
    | RESET
      optionalLifecycleSignature
      SEMICOLON
    | SUSPEND
      optionalLifecycleSignature
      SEMICOLON
    | RESUME
      optionalLifecycleSignature
      SEMICOLON
    ;


optionalLifecycleSignature
    : LPAREN
      parameterList?
      RPAREN
      returnClause?
    | /* empty */
    ;


/*
 * ============================================================================
 * PROPERTIES
 * ============================================================================
 */

systemPropertyDeclaration
    : annotation*
      PROPERTY
      identifier
      COLON
      typeExpression
      propertyMetadata*
      SEMICOLON
    ;


propertyMetadata
    : READONLY
    | VOLATILE
    | ATOMIC
    | OPTIONAL
    | REQUIRED
    | ATTRIBUTES LBRACE systemAttribute* RBRACE
    ;


/*
 * ============================================================================
 * IMPLEMENTATION-NEUTRAL BINDINGS
 * ============================================================================
 *
 * The binding references an external ABI/interface contract.
 *
 * ABI details remain owned by interoperability/abi.g4.
 */

systemBindingDeclaration
    : annotation*
      BIND
      qualifiedName
      TO
      qualifiedName
      bindingMetadata*
      SEMICOLON
    ;


bindingMetadata
    : ABI qualifiedName
    | FOREIGN
    | NATIVE
    | EXTERNAL
    | ATTRIBUTES LBRACE systemAttribute* RBRACE
    ;


/*
 * ============================================================================
 * GENERICS
 * ============================================================================
 */

genericParameterClause
    : LT
      genericParameter
      (COMMA genericParameter)*
      GT
    ;


genericParameter
    : identifier
      genericParameterConstraint?
    ;


genericParameterConstraint
    : COLON
      qualifiedName
    ;


/*
 * ============================================================================
 * PARAMETERS
 * ============================================================================
 *
 * Parameter count is unbounded by the grammar.
 */

parameterList
    : parameter
      (COMMA parameter)*
    ;


parameter
    : annotation*
      parameterMode?
      identifier
      COLON
      typeExpression
      parameterDefault?
    ;


parameterMode
    : IN
    | OUT
    | INOUT
    | BORROW
    | MOVE
    | REF
    ;


parameterDefault
    : EQUAL
      expression
    ;


/*
 * ============================================================================
 * TYPES
 * ============================================================================
 *
 * This grammar only defines the syntactic reference required by system
 * interfaces. Semantic type ownership remains with grammar/types and the
 * canonical Zamani type system.
 */

typeExpression
    : qualifiedName
      genericTypeArguments?
      typeSuffix*
    ;


genericTypeArguments
    : LT
      typeExpression
      (COMMA typeExpression)*
      GT
    ;


typeSuffix
    : LBRACK
      optionalExpression
      RBRACK
    | QUESTION
    ;


optionalExpression
    : expression
    | /* empty */
    ;


/*
 * ============================================================================
 * ARGUMENTS
 * ============================================================================
 */

argumentList
    : argument
      (COMMA argument)*
    ;


argument
    : namedArgument
    | expression
    ;


namedArgument
    : identifier
      EQUAL
      expression
    ;


/*
 * ============================================================================
 * ATTRIBUTES / ANNOTATIONS
 * ============================================================================
 */

attributeClause
    : ATTRIBUTES
      LBRACE
      systemAttribute*
      RBRACE
    ;


systemAttribute
    : annotation
    | identifier
      optionalAttributeValue
      SEMICOLON
    ;


optionalAttributeValue
    : EQUAL
      expression
    | /* empty */
    ;


annotation
    : AT
      qualifiedName
      optionalCallArguments
    ;


/*
 * ============================================================================
 * VISIBILITY
 * ============================================================================
 */

visibilityModifier
    : PUBLIC
    | PRIVATE
    | INTERNAL
    | PROTECTED
    ;


/*
 * ============================================================================
 * NAMES
 * ============================================================================
 *
 * These rules are intentionally compatible with the repository's canonical
 * naming layer while remaining independently parseable.
 *
 * Semantic lowering MUST canonicalize them through core/names.g4.
 */

qualifiedName
    : identifier
      (DOT identifier)*
    ;


identifier
    : IDENTIFIER
    | ESCAPED_IDENTIFIER
    ;


/*
 * ============================================================================
 * EXPRESSIONS
 * ============================================================================
 *
 * This is deliberately a compact expression surface for interface metadata,
 * constraints, defaults, and capability arguments.
 *
 * Full Zamani expression semantics remain owned by expressions/*.
 */

expression
    : logicalOrExpression
    ;


logicalOrExpression
    : logicalAndExpression
      (OROR logicalAndExpression)*
    ;


logicalAndExpression
    : equalityExpression
      (ANDAND equalityExpression)*
    ;


equalityExpression
    : relationalExpression
      ((EQUAL_EQUAL | NOT_EQUAL) relationalExpression)*
    ;


relationalExpression
    : additiveExpression
      ((LT | LE | GT | GE) additiveExpression)*
    ;


additiveExpression
    : multiplicativeExpression
      ((PLUS | MINUS) multiplicativeExpression)*
    ;


multiplicativeExpression
    : unaryExpression
      ((STAR | SLASH | PERCENT) unaryExpression)*
    ;


unaryExpression
    : (NOT | PLUS | MINUS | TILDE) unaryExpression
    | primaryExpression
    ;


primaryExpression
    : literal
    | qualifiedName
    | callExpression
    | LPAREN expression RPAREN
    ;


callExpression
    : qualifiedName
      LPAREN
      argumentList?
      RPAREN
    ;


literal
    : integerLiteral
    | floatingLiteral
    | stringLiteral
    | characterLiteral
    | TRUE
    | FALSE
    | NULL
    ;


integerLiteral
    : DECIMAL_INTEGER
    | HEX_INTEGER
    | OCTAL_INTEGER
    | BINARY_INTEGER
    ;


floatingLiteral
    : DECIMAL_FLOAT
    ;


stringLiteral
    : STRING_LITERAL
    ;


characterLiteral
    : CHARACTER_LITERAL
    ;


/*
 * ============================================================================
 * LEXER
 * ============================================================================
 *
 * No embedded actions.
 * No predicates.
 * No machine-specific token tables.
 * No hard-coded resource limits.
 */


/* ---------------- Keywords ---------------- */

SYSTEM       : 'system' ;
INTERFACE    : 'interface' ;
SERVICE      : 'service' ;
FN           : 'fn' ;
ASYNC        : 'async' ;
BLOCKING     : 'blocking' ;
NONBLOCKING  : 'nonblocking' ;
IDEMPOTENT   : 'idempotent' ;
CANCELLABLE  : 'cancellable' ;
STREAMING    : 'streaming' ;
TRANSACTIONAL: 'transactional' ;
PURE         : 'pure' ;

EVENT        : 'event' ;
INTERRUPT    : 'interrupt' ;
ON           : 'on' ;

HANDLE       : 'handle' ;
OWNED        : 'owned' ;
SHARED       : 'shared' ;
BORROWED     : 'borrowed' ;
SCOPED       : 'scoped' ;
OPAQUE       : 'opaque' ;

RESOURCE     : 'resource' ;
CAPABILITY   : 'capability' ;
REQUIREMENT  : 'requires' ;
FEATURE      : 'feature' ;
CONSTRAINT   : 'constraint' ;

EFFECTS      : 'effects' ;
ATTRIBUTES   : 'attributes' ;
AVAILABLE    : 'available' ;
WHEN         : 'when' ;

CONST        : 'const' ;
TYPE         : 'type' ;

LIFECYCLE    : 'lifecycle' ;
ACQUIRE      : 'acquire' ;
RELEASE      : 'release' ;
START        : 'start' ;
STOP         : 'stop' ;
RESET        : 'reset' ;
SUSPEND      : 'suspend' ;
RESUME       : 'resume' ;

PROPERTY     : 'property' ;
READONLY     : 'readonly' ;
VOLATILE     : 'volatile' ;
ATOMIC       : 'atomic' ;
OPTIONAL     : 'optional' ;
REQUIRED     : 'required' ;

BIND         : 'bind' ;
TO           : 'to' ;
ABI          : 'abi' ;
FOREIGN      : 'foreign' ;
NATIVE       : 'native' ;
EXTERNAL     : 'external' ;

VERSION      : 'version' ;

IN           : 'in' ;
OUT          : 'out' ;
INOUT        : 'inout' ;
BORROW       : 'borrow' ;
MOVE         : 'move' ;
REF          : 'ref' ;

PUBLIC       : 'public' ;
PRIVATE      : 'private' ;
INTERNAL     : 'internal' ;
PROTECTED    : 'protected' ;

TRUE         : 'true' ;
FALSE        : 'false' ;
NULL         : 'null' ;


/* ---------------- Punctuation ---------------- */

ARROW        : '->' ;
AT           : '@' ;
LBRACE       : '{' ;
RBRACE       : '}' ;
LPAREN       : '(' ;
RPAREN       : ')' ;
LBRACK       : '[' ;
RBRACK       : ']' ;
LT           : '<' ;
GT           : '>' ;
COLON        : ':' ;
SEMICOLON    : ';' ;
COMMA        : ',' ;
DOT          : '.' ;
QUESTION     : '?' ;
EQUAL        : '=' ;


/* ---------------- Operators ---------------- */

OROR         : '||' ;
ANDAND       : '&&' ;
EQUAL_EQUAL  : '==' ;
NOT_EQUAL    : '!=' ;
LE           : '<=' ;
GE           : '>=' ;
PLUS         : '+' ;
MINUS        : '-' ;
STAR         : '*' ;
SLASH        : '/' ;
PERCENT      : '%' ;
NOT          : '!' ;
TILDE        : '~' ;


/* ---------------- Numeric literals ---------------- */

DECIMAL_INTEGER
    : DIGIT+
    ;


HEX_INTEGER
    : '0' [xX] HEX_DIGIT+
    ;


OCTAL_INTEGER
    : '0' [oO] OCTAL_DIGIT+
    ;


BINARY_INTEGER
    : '0' [bB] [01]+
    ;


DECIMAL_FLOAT
    : DIGIT+ '.' DIGIT* EXPONENT?
    | '.' DIGIT+ EXPONENT?
    | DIGIT+ EXPONENT
    ;


fragment EXPONENT
    : [eE] [+-]? DIGIT+
    ;


fragment DIGIT
    : [0-9]
    ;


fragment HEX_DIGIT
    : [0-9a-fA-F]
    ;


fragment OCTAL_DIGIT
    : [0-7]
    ;


/* ---------------- Strings ---------------- */

STRING_LITERAL
    : '"' STRING_CHARACTER* '"'
    ;


fragment STRING_CHARACTER
    : ESCAPE_SEQUENCE
    | ~["\\\r\n]
    ;


CHARACTER_LITERAL
    : '\'' CHARACTER_CHARACTER '\''
    ;


fragment CHARACTER_CHARACTER
    : ESCAPE_SEQUENCE
    | ~['\\\r\n]
    ;


fragment ESCAPE_SEQUENCE
    : '\\'
      (
          ['"\\]
        | 'n'
        | 'r'
        | 't'
        | '0'
        | 'b'
        | 'f'
        | 'v'
        | 'x' HEX_DIGIT HEX_DIGIT
        | 'u' HEX_DIGIT HEX_DIGIT HEX_DIGIT HEX_DIGIT
      )
    ;


/* ---------------- Identifiers ---------------- */

IDENTIFIER
    : [a-zA-Z_]
      [a-zA-Z0-9_]*
    ;


ESCAPED_IDENTIFIER
    : '`'
      (~[`\\\r\n] | ESCAPE_SEQUENCE)*
      '`'
    ;


/* ---------------- Comments / whitespace ---------------- */

LINE_COMMENT
    : '//' ~[\r\n]* -> channel(HIDDEN)
    ;


BLOCK_COMMENT
    : '/*' .*? '*/' -> channel(HIDDEN)
    ;


WS
    : [ \t\r\n\u000B\u000C]+ -> channel(HIDDEN)
    ;
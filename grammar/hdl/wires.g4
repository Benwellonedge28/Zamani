/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hdl/wires.g4
 *
 * Purpose:
 *     Canonical HDL wire/net declaration and logical connectivity grammar.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no embedded Rust code.
 *     It requires no unsafe Rust.
 *
 * Language objective:
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     canonical/modular parser
 *          |
 *          +--> HDL module grammar
 *          |       |
 *          |       +--> ports
 *          |       +--> signals
 *          |       +--> wires <--- THIS FILE
 *          |       +--> registers
 *          |       +--> clocks
 *          |       +--> processes
 *          |       +--> combinational logic
 *          |       +--> sequential logic
 *          |       +--> state machines
 *          |       +--> memories
 *          |       +--> pipelines
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> name resolution
 *          +--> type checking
 *          +--> connectivity checking
 *          +--> driver analysis
 *          +--> width/shape checking
 *          +--> direction checking
 *          +--> resolution analysis
 *          +--> capability/resource analysis
 *          |
 *          v
 *     canonical HDL/hardware semantic representation
 *          |
 *          +--> optimization
 *          +--> scheduling
 *          +--> routing
 *          +--> placement
 *          +--> synthesis
 *          +--> target lowering
 *          |
 *          v
 *     hardware/runtime realization
 *
 * Grammar establishes syntax.
 * Semantic analysis establishes meaning.
 * Hardware compilation establishes physical realization.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - logical wire/net declarations;
 *     - wire/net declaration lists;
 *     - wire names;
 *     - wire logical types;
 *     - wire dimensions;
 *     - wire declaration modifiers;
 *     - wire attributes;
 *     - logical wire constraints;
 *     - wire references;
 *     - qualified wire references;
 *     - wire endpoint references;
 *     - logical wire connection syntax;
 *     - logical wire disconnection syntax where supported;
 *     - logical connectivity grouping;
 *     - wire resolution/driver-policy syntax where explicitly part of the
 *       language's logical wire abstraction.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - ports;
 *     - signals;
 *     - registers;
 *     - variables;
 *     - memories;
 *     - clocks;
 *     - timing;
 *     - processes;
 *     - combinational semantics;
 *     - sequential semantics;
 *     - state machines;
 *     - pipelines;
 *     - hardware modules;
 *     - physical nets;
 *     - physical pins;
 *     - FPGA routing resources;
 *     - ASIC metal routing;
 *     - placement;
 *     - routing algorithms;
 *     - device discovery;
 *     - hardware calibration;
 *     - target selection;
 *     - synthesis;
 *     - scheduling;
 *     - resource allocation;
 *     - runtime execution;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN.
 *
 * ============================================================================
 * WIRE SEMANTIC MODEL
 * ============================================================================
 *
 * A wire is a LOGICAL CONNECTIVITY abstraction.
 *
 * A wire does NOT inherently mean:
 *
 *     - one physical conductor;
 *     - one FPGA routing track;
 *     - one ASIC metal segment;
 *     - one package pin;
 *     - one device address;
 *     - one memory cell;
 *     - one CPU register;
 *     - one network connection;
 *     - one fixed hardware resource.
 *
 * The compiler may realize one logical wire as:
 *
 *     - a physical net;
 *     - multiple routed segments;
 *     - replicated infrastructure;
 *     - an optimized/eliminated connection;
 *     - a target-specific interconnect;
 *     - another implementation satisfying the same semantics.
 *
 * ============================================================================
 * SIGNAL / WIRE DISTINCTION
 * ============================================================================
 *
 * A signal represents a logical value-bearing HDL object.
 *
 * A wire represents logical connectivity between compatible endpoints.
 *
 * Therefore:
 *
 *     signals.g4
 *         owns signal declaration semantics.
 *
 *     wires.g4
 *         owns logical connectivity/net semantics.
 *
 *     ports.g4
 *         owns external/interface boundary semantics.
 *
 *     registers.g4
 *         owns storage/state semantics.
 *
 * A wire MUST NOT silently become a second signal abstraction.
 *
 * A signal declaration MUST NOT be duplicated here.
 *
 * ============================================================================
 * HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * This grammar deliberately contains no:
 *
 *     MAX_WIRES
 *     MAX_NETS
 *     MAX_FANOUT
 *     MAX_DRIVERS
 *     MAX_WIDTH
 *     MAX_CONNECTIONS
 *     MAX_ENDPOINTS
 *     MAX_MODULES
 *     MAX_LANES
 *     MAX_ROUTING_RESOURCES
 *
 * It also contains no:
 *
 *     FPGA identifiers
 *     ASIC identifiers
 *     board identifiers
 *     package-pin numbers
 *     routing coordinates
 *     device addresses
 *     vendor-specific topology
 *     physical net identifiers
 *     fixed interconnect counts
 *
 * Any practical limit belongs to:
 *
 *     parser resource policy;
 *     semantic validation;
 *     compiler resource policy;
 *     hardware capabilities;
 *     synthesis;
 *     placement;
 *     routing;
 *     deployment;
 *     runtime resources.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexer is the sole lexical authority.
 *
 * This parser consumes:
 *
 *     ZamaniLexer
 *
 * through:
 *
 *     options {
 *         tokenVocab = ZamaniLexer;
 *     }
 *
 * This file MUST NOT define lexer rules.
 *
 * The word "wire" is currently treated as a contextual HDL keyword rather than
 * forcing a second lexer vocabulary. The semantic layer MUST validate that the
 * contextual identifier occupying the wire-keyword position has the canonical
 * spelling "wire".
 *
 * The same principle applies to other contextual HDL concepts such as:
 *
 *     net
 *     connect
 *     disconnect
 *     resolve
 *     tri
 *     pullup
 *     pulldown
 *     highz
 *
 * They remain extensible contextual vocabulary unless promoted to reserved
 * language keywords through the canonical lexer specification.
 *
 * ============================================================================
 * SHARED GRAMMAR CONTRACT
 * ============================================================================
 *
 * The canonical parser architecture provides shared concepts such as:
 *
 *     identifier
 *     expression
 *     typeExpr
 *     qualifiedName
 *
 * This grammar MUST consume those shared concepts rather than defining a
 * second general-purpose expression/type system.
 *
 * In a composed parser, these rules are supplied by the shared grammar layer.
 *
 * The wire-specific adapter rules below exist only to establish the HDL wire
 * boundary.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Parsing MUST produce source-location-aware syntax information sufficient for
 * the frontend AST to represent:
 *
 *     WireDeclaration
 *     WireDeclarator
 *     WireReference
 *     WireEndpoint
 *     WireConnection
 *     WireDisconnection
 *     WireConstraint
 *     WireAttribute
 *
 * The grammar itself MUST NOT construct AST objects.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for determining:
 *
 *     - whether a wire name is declared;
 *     - whether a referenced endpoint exists;
 *     - whether an endpoint is signal/port/register-compatible;
 *     - whether source and destination types are compatible;
 *     - whether widths/shapes are compatible;
 *     - whether direction is legal;
 *     - whether multiple drivers are legal;
 *     - whether a resolution policy is required;
 *     - whether a connection is cyclic where prohibited;
 *     - whether a connection crosses an illegal clock/domain boundary;
 *     - whether a connection satisfies declared constraints;
 *     - whether the selected hardware can realize the connectivity.
 *
 * None of those decisions belong in this grammar.
 *
 * ============================================================================
 * NO PHYSICAL ROUTING
 * ============================================================================
 *
 * The following must NEVER become wire grammar semantics:
 *
 *     pin = A7
 *     route = x,y
 *     FPGA_BANK = 3
 *     device = "vendor-device"
 *     switchbox = ...
 *     routing_track = ...
 *
 * Such information belongs to target/deployment/hardware constraint layers.
 *
 * ============================================================================
 */

parser grammar wires;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. PUBLIC DECLARATION ENTRY POINT
 * ============================================================================
 *
 * Complete logical wire declaration.
 *
 * Canonical conceptual forms:
 *
 *     wire data: logic;
 *     wire bus: logic[WIDTH];
 *     wire a, b, c: logic;
 *
 * The declaration does not allocate a physical resource.
 */
hdlWireDeclaration
    : hdlWireAttributes?
      hdlWireModifiers*
      hdlWireKeyword
      hdlWireDeclaratorList
      hdlWireTerminator
    ;


/*
 * ============================================================================
 * 2. DECLARATION LIST
 * ============================================================================
 *
 * Arbitrary declaration cardinality.
 *
 * No fixed number of wires is encoded.
 */
hdlWireDeclaratorList
    : hdlWireDeclarator
      (
          COMMA
          hdlWireDeclarator
      )*
      COMMA?
    ;


hdlWireDeclarator
    : hdlWireName
      hdlWireType?
      hdlWireDimensions*
      hdlWireInitializer?
      hdlWireConstraints*
    ;


/*
 * ============================================================================
 * 3. TERMINATION
 * ============================================================================
 *
 * The enclosing HDL module grammar may decide whether a declaration is a
 * member statement or a declaration item.
 *
 * A semicolon is therefore accepted as the normal statement boundary.
 *
 * No newline-sensitive physical behavior is encoded here.
 */
hdlWireTerminator
    : SEMICOLON
    ;


/*
 * ============================================================================
 * 4. CONTEXTUAL WIRE KEYWORD
 * ============================================================================
 *
 * "wire" is currently contextual vocabulary.
 *
 * The parser intentionally does not create a second lexer token.
 *
 * Semantic validation MUST check the normalized spelling.
 */
hdlWireKeyword
    : identifier
    ;


/*
 * ============================================================================
 * 5. WIRE MODIFIERS
 * ============================================================================
 *
 * Modifiers express logical/source-level properties.
 *
 * They do not select physical hardware.
 */
hdlWireModifiers
    : hdlWireModifier
    ;


hdlWireModifier
    : MUT
    | CONST
    | INTERNAL
    | hdlWireContextualModifier
    ;


hdlWireContextualModifier
    : identifier
    ;


/*
 * ============================================================================
 * 6. WIRE NAME
 * ============================================================================
 *
 * Logical source-level symbol.
 */
hdlWireName
    : identifier
    ;


/*
 * ============================================================================
 * 7. WIRE TYPE
 * ============================================================================
 *
 * Type syntax is delegated to the shared type system.
 *
 * A wire may carry scalar, vector, aggregate, structured, parameterized,
 * generic, or future extensible values as permitted by the type system.
 *
 * No fixed width or shape is encoded.
 */
hdlWireType
    : COLON
      typeExpr
    ;


/*
 * ============================================================================
 * 8. WIRE DIMENSIONS
 * ============================================================================
 *
 * Dimensions describe logical shape.
 *
 * They do not describe physical resource capacity.
 *
 * Example:
 *
 *     wire bus: logic[WIDTH];
 *
 * WIDTH may be a generic, parameter, compile-time expression, type-level
 * expression, or another semantic value.
 */
hdlWireDimensions
    : LBRACKET
      hdlWireDimensionExpression?
      RBRACKET
    ;


hdlWireDimensionExpression
    : expression
    ;


/*
 * ============================================================================
 * 9. INITIALIZATION
 * ============================================================================
 *
 * Initialization is optional source-level value initialization.
 *
 * Whether initialization is meaningful for a selected HDL dialect/target is
 * determined by semantic analysis.
 *
 * This syntax does NOT imply:
 *
 *     FPGA initialization;
 *     ASIC power-up state;
 *     physical pull-up;
 *     physical pull-down;
 *     timing behavior.
 */
hdlWireInitializer
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * 10. ATTRIBUTES
 * ============================================================================
 *
 * Wire attributes are metadata.
 *
 * Attributes must not silently become physical routing directives.
 */
hdlWireAttributes
    : hdlWireAttribute+
    ;


hdlWireAttribute
    : AT
      identifier
      (
          LPAREN
          hdlWireArgumentList?
          RPAREN
      )?
    ;


hdlWireArgumentList
    : hdlWireArgument
      (
          COMMA
          hdlWireArgument
      )*
      COMMA?
    ;


hdlWireArgument
    : identifier
      ASSIGN
      expression
    | expression
    ;


/*
 * ============================================================================
 * 11. LOGICAL WIRE CONSTRAINTS
 * ============================================================================
 *
 * Constraints describe source-level semantic requirements.
 *
 * They are not physical placement/routing constraints.
 */
hdlWireConstraints
    : hdlWireConstraint
    ;


hdlWireConstraint
    : hdlWireConstraintAttribute
    | hdlWireConstraintExpression
    ;


hdlWireConstraintAttribute
    : AT
      identifier
      (
          LPAREN
          hdlWireArgumentList?
          RPAREN
      )?
    ;


hdlWireConstraintExpression
    : LBRACE
      hdlWireConstraintExpressionList?
      RBRACE
    ;


hdlWireConstraintExpressionList
    : expression
      (
          COMMA
          expression
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 12. WIRE REFERENCE
 * ============================================================================
 *
 * A wire reference identifies a logical wire.
 */
hdlWireReference
    : hdlWireName
    | hdlWireQualifiedReference
    ;


hdlWireQualifiedReference
    : identifier
      (
          DOUBLE_COLON
          identifier
      )+
    ;


/*
 * ============================================================================
 * 13. WIRE INDEXING / SLICING
 * ============================================================================
 *
 * Indexing is logical selection.
 *
 * Semantic analysis determines:
 *
 *     - indexability;
 *     - bounds;
 *     - shape;
 *     - resulting type.
 *
 * No index width limit exists.
 */
hdlWireIndex
    : LBRACKET
      expression
      RBRACKET
    ;


hdlWireSlice
    : LBRACKET
      hdlWireRangeExpression
      RBRACKET
    ;


hdlWireRangeExpression
    : expression
      (
          hdlWireRangeOperator
          expression?
      )?
    ;


hdlWireRangeOperator
    : COLON
    | DOT_DOT
    | DOT_DOT_EQ
    ;


hdlWireSelection
    : hdlWireReference
      (
          hdlWireIndex
        | hdlWireSlice
      )*
    ;


/*
 * ============================================================================
 * 14. MEMBER SELECTION
 * ============================================================================
 *
 * Supports structured wire values without defining a new type system.
 */
hdlWireMemberSelection
    : hdlWireReference
      (
          DOT
          identifier
      )+
    ;


/*
 * ============================================================================
 * 15. WIRE ACCESS
 * ============================================================================
 */
hdlWireAccess
    : hdlWireSelection
    | hdlWireMemberSelection
    | hdlWireReference
    ;


/*
 * ============================================================================
 * 16. CONNECTION ENDPOINT
 * ============================================================================
 *
 * A wire endpoint is a logical source-level connection endpoint.
 *
 * An endpoint may be:
 *
 *     a wire;
 *     a signal;
 *     a port;
 *     a register;
 *     a structured member;
 *     an indexed/sliced value.
 *
 * Semantic analysis determines which endpoint categories are legal.
 */
hdlWireEndpoint
    : hdlWireAccess
    ;


/*
 * ============================================================================
 * 17. CONNECTION
 * ============================================================================
 *
 * Canonical conceptual form:
 *
 *     source -> destination;
 *
 * The arrow expresses logical connectivity.
 *
 * It does NOT mean:
 *
 *     physical route;
 *     placement;
 *     routing coordinate;
 *     device-to-device path;
 *     network path.
 *
 * The hardware backend is free to realize the connection in any semantically
 * equivalent manner.
 */
hdlWireConnection
    : hdlWireEndpoint
      THIN_ARROW
      hdlWireEndpoint
      hdlWireConnectionProperties*
      hdlWireTerminator
    ;


/*
 * ============================================================================
 * 18. BIDIRECTIONAL CONNECTION
 * ============================================================================
 *
 * A bidirectional logical relationship can be represented explicitly where
 * the HDL semantic model permits it.
 *
 * Directionality and multiple-driver legality remain semantic concerns.
 */
hdlWireBidirectionalConnection
    : hdlWireEndpoint
      hdlWireBidirectionalOperator
      hdlWireEndpoint
      hdlWireConnectionProperties*
      hdlWireTerminator
    ;


hdlWireBidirectionalOperator
    : ARROW
    ;


/*
 * ============================================================================
 * 19. DISCONNECTION
 * ============================================================================
 *
 * Disconnection is optional source-level topology mutation.
 *
 * It does not perform runtime physical routing changes by itself.
 */
hdlWireDisconnection
    : hdlWireEndpoint
      hdlWireDisconnectOperator
      hdlWireEndpoint
      hdlWireTerminator
    ;


hdlWireDisconnectOperator
    : MINUS_ARROW
    ;


/*
 * ============================================================================
 * 20. CONNECTION PROPERTIES
 * ============================================================================
 *
 * Properties describe logical connection intent.
 *
 * They must not encode physical routing decisions.
 */
hdlWireConnectionProperties
    : hdlWireConnectionAttribute
    ;


hdlWireConnectionAttribute
    : AT
      identifier
      (
          LPAREN
          hdlWireArgumentList?
          RPAREN
      )?
    ;


/*
 * ============================================================================
 * 21. CONNECTIVITY GROUP
 * ============================================================================
 *
 * A group expresses that several endpoints participate in one logical
 * connectivity declaration.
 *
 * Example conceptual form:
 *
 *     connect {
 *         a;
 *         b;
 *         c;
 *     };
 *
 * The semantic layer determines whether the resulting topology permits:
 *
 *     one driver;
 *     multiple drivers;
 *     resolved drivers;
 *     tri-state behavior;
 *     broadcast/fanout;
 *     another legal connectivity model.
 */
hdlWireConnectivityGroup
    : hdlWireGroupKeyword
      LBRACE
      hdlWireEndpointList?
      RBRACE
      hdlWireTerminator
    ;


hdlWireGroupKeyword
    : identifier
    ;


hdlWireEndpointList
    : hdlWireEndpoint
      (
          COMMA
          hdlWireEndpoint
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 22. RESOLUTION POLICY
 * ============================================================================
 *
 * Resolution is a semantic policy for situations in which multiple logical
 * drivers participate in a wire/net.
 *
 * The grammar does not define the actual resolution algorithm.
 *
 * A semantic/type system may later define policies such as:
 *
 *     wired-or;
 *     wired-and;
 *     tri-state;
 *     priority;
 *     custom resolution;
 *
 * without changing the basic wire abstraction.
 */
hdlWireResolution
    : hdlWireResolutionKeyword
      hdlWireResolutionValue?
      hdlWireTerminator
    ;


hdlWireResolutionKeyword
    : identifier
    ;


hdlWireResolutionValue
    : identifier
    | expression
    ;


/*
 * ============================================================================
 * 23. DRIVER POLICY
 * ============================================================================
 *
 * Driver policy is logical semantic metadata.
 *
 * It is NOT a routing/resource allocation directive.
 */
hdlWireDriverPolicy
    : hdlWireDriverPolicyKeyword
      hdlWireDriverPolicyValue?
      hdlWireTerminator
    ;


hdlWireDriverPolicyKeyword
    : identifier
    ;


hdlWireDriverPolicyValue
    : identifier
    | expression
    ;


/*
 * ============================================================================
 * 24. WIRE DECLARATION + CONNECTIVITY COMPOSITION
 * ============================================================================
 *
 * This rule is useful as the stable HDL module-member boundary.
 *
 * It allows the parent HDL grammar to consume wire-specific members without
 * needing to know their internal representation.
 */
hdlWireMember
    : hdlWireDeclaration
    | hdlWireConnection
    | hdlWireBidirectionalConnection
    | hdlWireDisconnection
    | hdlWireConnectivityGroup
    | hdlWireResolution
    | hdlWireDriverPolicy
    ;


/*
 * ============================================================================
 * 25. WIRE DESIGN REGION
 * ============================================================================
 *
 * Standalone wire grammar testing can parse an arbitrary number of wire
 * members.
 *
 * No finite machine-size limit is encoded.
 */
hdlWireRegion
    : hdlWireMember*
    ;


/*
 * ============================================================================
 * 26. WIRE EXPRESSION ADAPTERS
 * ============================================================================
 *
 * These adapters intentionally delegate general expressions to the canonical
 * `expression` rule.
 *
 * They exist to provide stable semantic names for future HDL AST lowering.
 *
 * No second expression grammar is introduced here.
 */
hdlWireValueExpression
    : expression
    ;


hdlWireCompileTimeExpression
    : expression
    ;


/*
 * ============================================================================
 * 27. SEMANTIC CONNECTION ASSERTION
 * ============================================================================
 *
 * A connection assertion states a source-level requirement about connectivity.
 *
 * It does not inspect physical routing.
 */
hdlWireConnectionAssertion
    : hdlWireAssertionKeyword
      hdlWireEndpoint
      hdlWireAssertionOperator
      hdlWireEndpoint
      hdlWireTerminator
    ;


hdlWireAssertionKeyword
    : identifier
    ;


hdlWireAssertionOperator
    : EQUAL_EQUAL
    | NOT_EQUAL
    ;


/*
 * ============================================================================
 * 28. COMPATIBILITY ADAPTER
 * ============================================================================
 *
 * Some existing HDL grammar code may expect a generic "net" concept.
 *
 * The wire grammar provides a parser-facing alias boundary without creating a
 * second implementation.
 *
 * Semantic normalization must map this to the same logical connectivity
 * abstraction when the language defines "net" as a wire synonym.
 */
hdlNetDeclaration
    : hdlWireDeclaration
    ;


hdlNetReference
    : hdlWireReference
    ;


hdlNetConnection
    : hdlWireConnection
    ;


/*
 * ============================================================================
 * 29. SOURCE-LEVEL CONNECTIVITY STATEMENT
 * ============================================================================
 *
 * This is the preferred composition entry point for HDL module bodies.
 */
hdlConnectivityStatement
    : hdlWireConnection
    | hdlWireBidirectionalConnection
    | hdlWireDisconnection
    | hdlWireConnectivityGroup
    ;


/*
 * ============================================================================
 * 30. CANONICAL PUBLIC HDL WIRE API
 * ============================================================================
 *
 * Stable public rules for consumers:
 *
 *     hdlWireDeclaration
 *     hdlWireDeclaratorList
 *     hdlWireReference
 *     hdlWireEndpoint
 *     hdlWireConnection
 *     hdlWireBidirectionalConnection
 *     hdlWireDisconnection
 *     hdlWireMember
 *     hdlConnectivityStatement
 *
 * Downstream grammar files should depend on these boundaries rather than on
 * implementation details of the wire grammar.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */
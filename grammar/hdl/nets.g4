/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hdl/nets.g4
 *
 * Status:
 *     CANONICAL HDL NET DECLARATION GRAMMAR
 *
 * Purpose:
 *     Define target-independent logical HDL net declarations and the
 *     parser-facing net abstraction used by the HDL composition layer.
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     This grammar contains no embedded Rust actions, predicates, unsafe
 *     code, filesystem access, network access, hardware discovery, runtime
 *     execution, or target-dependent behavior.
 *
 * Language objective:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *     (POCO-REAF)
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
 *          +--> modules
 *          +--> ports
 *          +--> signals
 *          +--> nets       <-- THIS FILE
 *          +--> wires
 *          +--> registers
 *          +--> memories
 *          +--> clocks
 *          +--> timing
 *          +--> processes
 *          +--> pipelines
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> name resolution
 *          +--> type checking
 *          +--> shape/width checking
 *          +--> connectivity analysis
 *          +--> driver analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          |
 *          v
 *     canonical hardware semantic representation / IR
 *          |
 *          +--> optimization
 *          +--> scheduling
 *          +--> routing
 *          +--> synthesis
 *          +--> placement
 *          +--> target lowering
 *          |
 *          v
 *     target realization
 *
 * ============================================================================
 * AUTHORITY
 * ============================================================================
 *
 * This file is a LEAF parser grammar.
 *
 * The canonical HDL composition root is:
 *
 *     grammar/hdl/hdl.g4
 *
 * The canonical global parser composition root is:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * The canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This file MUST NOT become another HDL root.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - logical net declarations;
 *     - net declarator lists;
 *     - net names;
 *     - net type attachment;
 *     - logical net dimensions;
 *     - net declaration initializers where supported;
 *     - net modifiers;
 *     - net attributes;
 *     - net-level logical constraints;
 *     - logical net references;
 *     - net selections;
 *     - stable parser-facing net declaration contracts.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical definitions;
 *     - identifiers;
 *     - general expressions;
 *     - the universal type system;
 *     - ports;
 *     - signals;
 *     - wire connectivity algorithms;
 *     - physical routing;
 *     - placement;
 *     - timing closure;
 *     - scheduling;
 *     - synthesis;
 *     - device selection;
 *     - target discovery;
 *     - resource allocation;
 *     - runtime execution;
 *     - quantum operations;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN.
 *
 * ============================================================================
 * NET SEMANTIC MODEL
 * ============================================================================
 *
 * A net is a LOGICAL CONNECTIVITY/VALUE-CARRYING HDL object.
 *
 * It does not inherently represent:
 *
 *     - one physical conductor;
 *     - one FPGA routing track;
 *     - one ASIC metal segment;
 *     - one package pin;
 *     - one physical switch;
 *     - one fixed interconnect;
 *     - one device address;
 *     - one CPU resource;
 *     - one GPU resource;
 *     - one FPGA resource.
 *
 * A backend may realize one logical net using:
 *
 *     - one physical connection;
 *     - multiple physical segments;
 *     - buffered connectivity;
 *     - replicated connectivity;
 *     - optimized/eliminated connectivity;
 *     - target-specific interconnect;
 *     - another semantically equivalent representation.
 *
 * The realization is deliberately outside this grammar.
 *
 * ============================================================================
 * NET / WIRE DISTINCTION
 * ============================================================================
 *
 * `net` and `wire` must not become two unrelated connectivity systems.
 *
 * The intended semantic relationship is:
 *
 *     logical net
 *          |
 *          +--> wire-style realization
 *          +--> other supported net kinds
 *
 * `nets.g4` owns the generic net declaration boundary.
 *
 * `wires.g4` owns wire-specific connectivity syntax and wire-oriented
 * compatibility forms.
 *
 * `wires.g4` MUST NOT redefine the canonical semantic meaning of
 * `hdlNetDeclaration`.
 *
 * ============================================================================
 * POCO-REAF / OPEN-WORLD PRINCIPLE
 * ============================================================================
 *
 * There are intentionally no universal constants such as:
 *
 *     MAX_NETS
 *     MAX_WIRES
 *     MAX_ENDPOINTS
 *     MAX_DRIVERS
 *     MAX_FANOUT
 *     MAX_WIDTH
 *     MAX_DIMENSIONS
 *     MAX_CONNECTIONS
 *     MAX_MODULES
 *     MAX_DEVICES
 *     MAX_LANES
 *
 * The grammar contains no physical resource ceiling.
 *
 * A source program may contain:
 *
 *     one net;
 *     many nets;
 *     parameterized nets;
 *     generated net collections;
 *     arbitrarily large logical designs.
 *
 * Actual limits are determined downstream by:
 *
 *     - compiler resources;
 *     - semantic resource policies;
 *     - target capabilities;
 *     - synthesis resources;
 *     - routing resources;
 *     - deployment resources;
 *     - runtime resources.
 *
 * These are not language-level syntax limits.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Net types belong to the canonical Zamani type system.
 *
 * This file MUST NOT create another type grammar.
 *
 * The canonical parser composition layer provides:
 *
 *     typeExpr
 *
 * Net declarations consume that rule.
 *
 * Examples include conceptually:
 *
 *     net data: logic;
 *     net bus: Vector<logic, WIDTH>;
 *     net tensor: Tensor<T, SHAPE>;
 *
 * A width or shape is program/type semantics.
 *
 * It is NOT a machine capacity.
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * Expressions belong to the canonical expression system.
 *
 * This file consumes:
 *
 *     expression
 *
 * It does NOT define:
 *
 *     additiveExpression
 *     multiplicativeExpression
 *     shiftExpression
 *     logicalExpression
 *     conditionalExpression
 *     functionCallExpression
 *
 * locally.
 *
 * This prevents the HDL directory from developing a second expression
 * precedence hierarchy.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The production lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Parser grammars consume:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * This grammar therefore contains no lexer rules.
 *
 * IMPORTANT REPOSITORY INTEGRATION:
 *
 * The current repository's HDL composition grammar references:
 *
 *     K_NET
 *
 * and:
 *
 *     K_WIRE
 *
 * but the current keyword vocabulary does not yet expose those tokens as
 * canonical reserved keywords.
 *
 * The production migration MUST establish one lexical owner for `net`.
 *
 * Recommended canonical lexical contract:
 *
 *     K_NET : 'net' ;
 *
 * in the canonical keyword vocabulary.
 *
 * That token must then flow through:
 *
 *     ZamaniKeywords
 *         ->
 *     ZamaniTokens
 *         ->
 *     ZamaniLexer
 *         ->
 *     HDL parser
 *
 * `nets.g4` must not create a second lexer.
 *
 * ============================================================================
 * RESERVED KEYWORD POLICY
 * ============================================================================
 *
 * `net` is a structural HDL declaration keyword.
 *
 * It should be lexically distinguishable from an arbitrary identifier.
 *
 * This is preferable to:
 *
 *     hdlNetKeyword
 *         : identifier
 *         ;
 *
 * because an arbitrary identifier fallback creates declaration/expression
 * ambiguity and makes the parser dependent on semantic spelling checks.
 *
 * Therefore the canonical production form is:
 *
 *     hdlNetKeyword
 *         : K_NET
 *         ;
 *
 * ============================================================================
 * PUBLIC DECLARATION
 * ============================================================================
 *
 * Canonical conceptual forms:
 *
 *     net data: logic;
 *
 *     net bus: logic[WIDTH];
 *
 *     net a, b, c: logic;
 *
 *     net data: Vector<T, WIDTH>;
 *
 *     net data[ROWS][COLS]: logic;
 *
 * The exact semantic validity of these forms is determined downstream.
 * ============================================================================
 */

parser grammar nets;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. PUBLIC NET DECLARATION
 * ============================================================================
 *
 * Complete source-level net declaration.
 *
 * The semicolon is owned here because this rule is also a reusable declaration
 * boundary for HDL module members.
 */
hdlNetDeclaration
    : hdlNetAttributes?
      hdlNetModifiers*
      hdlNetKeyword
      hdlNetDeclaratorList
      hdlNetTerminator
    ;


/*
 * ============================================================================
 * 2. NET KEYWORD
 * ============================================================================
 *
 * `net` is a canonical structural keyword.
 *
 * Do not replace this with IDENTIFIER.
 */
hdlNetKeyword
    : K_NET
    ;


/*
 * ============================================================================
 * 3. NET DECLARATOR LIST
 * ============================================================================
 *
 * Arbitrary cardinality.
 *
 * There is no fixed number of net declarations.
 */
hdlNetDeclaratorList
    : hdlNetDeclarator
      (
          COMMA
          hdlNetDeclarator
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 4. NET DECLARATOR
 * ============================================================================
 *
 * A declarator contains the logical identity and optional source-level
 * properties of one net.
 */
hdlNetDeclarator
    : hdlNetName
      hdlNetType?
      hdlNetDimensions*
      hdlNetInitializer?
      hdlNetConstraints*
    ;


/*
 * ============================================================================
 * 5. NET NAME
 * ============================================================================
 *
 * Names are owned by the canonical identifier/name grammar.
 */
hdlNetName
    : identifier
    ;


/*
 * ============================================================================
 * 6. NET TYPE
 * ============================================================================
 *
 * The universal type system owns type semantics.
 */
hdlNetType
    : COLON
      typeExpr
    ;


/*
 * ============================================================================
 * 7. NET DIMENSIONS
 * ============================================================================
 *
 * Logical dimensions are expressions.
 *
 * No fixed number of dimensions exists.
 *
 * Examples:
 *
 *     net bus: logic[WIDTH];
 *
 *     net matrix: logic[ROWS][COLS];
 *
 *     net tensor: T[D0][D1][D2];
 *
 * The actual validity of a dimension is semantic.
 */
hdlNetDimensions
    : LBRACKET
      hdlNetDimensionExpression?
      RBRACKET
    ;


hdlNetDimensionExpression
    : expression
    ;


/*
 * ============================================================================
 * 8. NET INITIALIZER
 * ============================================================================
 *
 * Initialization is source-level semantic initialization.
 *
 * It does not prescribe:
 *
 *     FPGA configuration;
 *     ASIC power-up;
 *     physical pull-up;
 *     physical pull-down;
 *     routing;
 *     timing;
 *     placement.
 */
hdlNetInitializer
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * 9. NET MODIFIERS
 * ============================================================================
 *
 * Modifiers must remain source-level semantic properties.
 *
 * They must not encode physical resources.
 */
hdlNetModifiers
    : hdlNetModifier
    ;


hdlNetModifier
    : MUT
    | CONST
    | INTERNAL
    ;


/*
 * ============================================================================
 * 10. NET ATTRIBUTES
 * ============================================================================
 *
 * Attributes are metadata.
 *
 * The attribute grammar itself belongs to the shared/core grammar layer.
 */
hdlNetAttributes
    : hdlNetAttribute+
    ;


hdlNetAttribute
    : AT
      identifier
      (
          LPAREN
          hdlNetArgumentList?
          RPAREN
      )?
    ;


hdlNetArgumentList
    : hdlNetArgument
      (
          COMMA
          hdlNetArgument
      )*
      COMMA?
    ;


hdlNetArgument
    : identifier
      ASSIGN
      expression
    | expression
    ;


/*
 * ============================================================================
 * 11. NET CONSTRAINTS
 * ============================================================================
 *
 * Constraints are source-level semantic metadata.
 *
 * They do not select physical routing or placement.
 */
hdlNetConstraints
    : hdlNetConstraint
    ;


hdlNetConstraint
    : hdlNetConstraintAttribute
    | hdlNetConstraintExpression
    ;


hdlNetConstraintAttribute
    : AT
      identifier
      (
          LPAREN
          hdlNetArgumentList?
          RPAREN
      )?
    ;


hdlNetConstraintExpression
    : LBRACE
      hdlNetConstraintExpressionList?
      RBRACE
    ;


hdlNetConstraintExpressionList
    : expression
      (
          COMMA
          expression
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 12. NET TERMINATOR
 * ============================================================================
 */
hdlNetTerminator
    : SEMICOLON
    ;


/*
 * ============================================================================
 * 13. NET REFERENCE
 * ============================================================================
 *
 * References identify logical nets.
 */
hdlNetReference
    : hdlNetName
    | hdlNetQualifiedReference
    ;


hdlNetQualifiedReference
    : identifier
      (
          DOUBLE_COLON
          identifier
      )+
    ;


/*
 * ============================================================================
 * 14. NET INDEX
 * ============================================================================
 *
 * Logical indexing.
 *
 * Bounds and type compatibility are semantic concerns.
 */
hdlNetIndex
    : LBRACKET
      expression
      RBRACKET
    ;


/*
 * ============================================================================
 * 15. NET SLICE
 * ============================================================================
 *
 * Logical range selection.
 *
 * The range expressions are ordinary Zamani expressions.
 */
hdlNetSlice
    : LBRACKET
      hdlNetRangeExpression
      RBRACKET
    ;


hdlNetRangeExpression
    : expression
      (
          hdlNetRangeOperator
          expression?
      )?
    ;


hdlNetRangeOperator
    : COLON
    | DOT_DOT
    | DOT_DOT_EQ
    ;


/*
 * ============================================================================
 * 16. NET SELECTION
 * ============================================================================
 */
hdlNetSelection
    : hdlNetReference
      (
          hdlNetIndex
        | hdlNetSlice
      )*
    ;


/*
 * ============================================================================
 * 17. NET MEMBER SELECTION
 * ============================================================================
 *
 * Structured net values may expose members through the common source-level
 * member model.
 */
hdlNetMemberSelection
    : hdlNetReference
      (
          DOT
          identifier
      )+
    ;


/*
 * ============================================================================
 * 18. NET ACCESS
 * ============================================================================
 */
hdlNetAccess
    : hdlNetSelection
    | hdlNetMemberSelection
    | hdlNetReference
    ;


/*
 * ============================================================================
 * 19. NET DECLARATION REGION
 * ============================================================================
 *
 * Useful for standalone grammar conformance tests and HDL composition.
 */
hdlNetDeclarations
    : hdlNetDeclaration*
    ;


/*
 * ============================================================================
 * 20. NET MEMBER
 * ============================================================================
 *
 * This is deliberately declaration-only.
 *
 * Connectivity belongs to wires.g4 / the connectivity layer.
 */
hdlNetMember
    : hdlNetDeclaration
    ;


/*
 * ============================================================================
 * 21. NET VALUE EXPRESSION ADAPTER
 * ============================================================================
 *
 * Stable HDL-facing semantic adapter.
 *
 * It does not create another expression grammar.
 */
hdlNetValueExpression
    : expression
    ;


/*
 * ============================================================================
 * 22. NET COMPILE-TIME EXPRESSION ADAPTER
 * ============================================================================
 *
 * Compile-time legality is semantic.
 */
hdlNetCompileTimeExpression
    : expression
    ;


/*
 * ============================================================================
 * 23. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing HDL code may historically refer to:
 *
 *     hdlNetDeclaration
 *     hdlNetReference
 *
 * These remain the canonical names.
 *
 * No second:
 *
 *     netDeclaration
 *
 * or:
 *
 *     genericNetDeclaration
 *
 * authority should be introduced.
 */


/*
 * ============================================================================
 * 24. AST CONTRACT
 * ============================================================================
 *
 * This grammar provides enough structure for the frontend to represent:
 *
 *     HdlNetDeclaration
 *     HdlNetDeclarator
 *     HdlNetName
 *     HdlNetType
 *     HdlNetDimension
 *     HdlNetInitializer
 *     HdlNetModifier
 *     HdlNetAttribute
 *     HdlNetConstraint
 *     HdlNetReference
 *     HdlNetSelection
 *
 * The grammar constructs no AST objects.
 *
 * Source spans are preserved by the parser/frontend infrastructure.
 */


/*
 * ============================================================================
 * 25. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must determine:
 *
 *     - whether the net name is legal;
 *     - whether the name conflicts with another declaration;
 *     - whether the net type exists;
 *     - whether the type is legal in HDL;
 *     - whether dimensions are valid;
 *     - whether dimensions are evaluable where required;
 *     - whether initializer type matches net type;
 *     - whether attributes are recognized;
 *     - whether constraints are meaningful;
 *     - whether the net may connect to a given endpoint;
 *     - whether widths/shapes are compatible;
 *     - whether multiple drivers are legal;
 *     - whether a resolution policy is required;
 *     - whether a target can realize the logical net.
 *
 * None of these are parser-level hardware decisions.
 */


/*
 * ============================================================================
 * 26. HARDWARE / RESOURCE BOUNDARY
 * ============================================================================
 *
 * This grammar does not decide:
 *
 *     how many physical nets exist;
 *     how many routing tracks exist;
 *     how many FPGA resources exist;
 *     how many ASIC routing layers exist;
 *     which physical device is used;
 *     which pin is used;
 *     where a net is placed;
 *     how the net is routed;
 *     how the net is buffered;
 *     how timing closure is achieved.
 *
 * Those decisions belong downstream.
 */


/*
 * ============================================================================
 * 27. CROSS-DOMAIN INTEGRATION
 * ============================================================================
 *
 * Classical:
 *
 *     Nets may carry classical values.
 *
 * Quantum:
 *
 *     A net may participate in classical control around quantum hardware,
 *     subject to semantic validation.
 *
 *     This grammar does not define quantum operations or quantum::ir.
 *
 * Hybrid:
 *
 *     Nets can form source-level boundaries between classical and quantum
 *     components.
 *
 * AI/data:
 *
 *     Parameterized types may describe structured/tensor data when permitted
 *     by the shared type system.
 *
 * Distributed:
 *
 *     A logical net may later lower to distributed communication only through
 *     explicit semantic/deployment mechanisms.
 *
 * Hardware:
 *
 *     Hardware realization consumes the logical net contract.
 */


/*
 * ============================================================================
 * 28. QUANTUM BOUNDARY
 * ============================================================================
 *
 * This file MUST NOT define:
 *
 *     quantum gates;
 *     qubits;
 *     measurement;
 *     QEC;
 *     noise;
 *     QZN/ZQN behavior;
 *     physical quantum connectivity;
 *     coupling maps;
 *     calibration;
 *     physical qubit IDs.
 *
 * Quantum semantics remain downstream and ultimately use the existing
 * `quantum::ir` boundary.
 */


/*
 * ============================================================================
 * 29. DETERMINISM
 * ============================================================================
 *
 * Given identical:
 *
 *     source;
 *     language version;
 *     lexer version;
 *     parser grammar;
 *
 * this grammar must produce the same syntactic structure.
 *
 * It must not depend on:
 *
 *     hardware;
 *     target availability;
 *     network state;
 *     filesystem state;
 *     environment variables;
 *     runtime state;
 *     randomness.
 */


/*
 * ============================================================================
 * 30. SAFETY
 * ============================================================================
 *
 * No embedded Rust actions are used.
 *
 * Therefore the grammar itself requires no unsafe Rust.
 *
 * Compiler/frontend implementations integrating this grammar must remain:
 *
 *     Rust 1.97 / 1.97.1
 *     Rust 2021
 *     safe Rust
 *
 * No HDL construct may require unsafe Rust merely to parse or represent it.
 */


/*
 * ============================================================================
 * 31. DIAGNOSTIC BOUNDARY
 * ============================================================================
 *
 * Syntax diagnostics belong to parser infrastructure.
 *
 * Semantic diagnostics belong to semantic analysis.
 *
 * Capability errors belong to capability analysis.
 *
 * Resource errors belong to resource analysis.
 *
 * Target realization errors belong to backend/deployment.
 *
 * This distinction is required for POCO-REAF.
 */


/*
 * ============================================================================
 * 32. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file intentionally contains no:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * It also contains no:
 *
 *     fixed bus width;
 *     fixed net count;
 *     fixed fanout;
 *     fixed driver count;
 *     fixed endpoint count;
 *     fixed routing capacity;
 *     fixed topology;
 *     physical device identifier.
 *
 * A literal such as:
 *
 *     32
 *
 * remains legal when it is explicitly part of program semantics.
 *
 * The grammar never turns it into a universal hardware limit.
 */


/*
 * ============================================================================
 * 33. PERFORMANCE / RESOURCE POLICY
 * ============================================================================
 *
 * Implementations may impose configurable operational safeguards against:
 *
 *     pathological source;
 *     excessive nesting;
 *     excessive generated input;
 *     compiler memory exhaustion;
 *     denial-of-service inputs.
 *
 * Such safeguards are implementation policies.
 *
 * They MUST NOT become semantic limits encoded in this grammar.
 */


/*
 * ============================================================================
 * 34. PUBLIC INTEGRATION API
 * ============================================================================
 *
 * The canonical public rules supplied by this file are:
 *
 *     hdlNetDeclaration
 *     hdlNetDeclaratorList
 *     hdlNetDeclarator
 *     hdlNetName
 *     hdlNetType
 *     hdlNetDimensions
 *     hdlNetReference
 *     hdlNetSelection
 *     hdlNetAccess
 *     hdlNetMember
 *     hdlNetDeclarations
 *
 * Other HDL grammars should consume these boundaries rather than duplicate
 * their implementation.
 */


/*
 * ============================================================================
 * 35. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] net declaration syntax is owned here;
 *     [x] net names use the shared identifier contract;
 *     [x] net types use the shared type contract;
 *     [x] net dimensions use the shared expression contract;
 *     [x] net attributes are source metadata;
 *     [x] net constraints remain semantic;
 *     [x] references are represented;
 *     [x] selections are represented;
 *     [x] no physical routing is represented;
 *     [x] no fixed hardware capacity is represented;
 *     [x] no quantum IR is duplicated;
 *     [x] no expression grammar is duplicated;
 *     [x] no type grammar is duplicated;
 *     [x] no second lexer is created;
 *     [x] Rust 1.97/1.97.1 compatibility is preserved;
 *     [x] safe-Rust architecture is preserved;
 *     [x] POCO-REAF is preserved;
 *     [x] cross-domain ownership is explicit;
 *     [x] integration boundaries are explicit.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */
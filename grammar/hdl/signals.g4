/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hdl/signals.g4
 *
 * Role:
 *     HDL signal declaration and signal-reference grammar.
 *
 * Language objective:
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no embedded Rust code and requires no `unsafe`.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     Canonical lexer
 *          |
 *          v
 *     Canonical/modular parser
 *          |
 *          v
 *     HDL grammar
 *          |
 *          +--> ports.g4
 *          +--> signals.g4  <--- THIS FILE
 *          +--> wires.g4
 *          +--> registers.g4
 *          +--> clocks.g4
 *          +--> processes.g4
 *          +--> ...
 *          |
 *          v
 *     Frontend AST
 *          |
 *          v
 *     Semantic analysis
 *          |
 *          +--> name resolution
 *          +--> type checking
 *          +--> width/shape checking
 *          +--> driver analysis
 *          +--> direction checking
 *          +--> clock/domain analysis
 *          +--> capability/resource analysis
 *          |
 *          v
 *     Canonical hardware semantic representation / IR
 *          |
 *          v
 *     optimization / scheduling / routing / synthesis
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
 *     - logical HDL signal declarations;
 *     - signal declaration lists;
 *     - signal names;
 *     - signal type attachment;
 *     - signal dimensions;
 *     - signal initialization syntax;
 *     - signal attributes;
 *     - signal source-level constraints;
 *     - signal references;
 *     - qualified signal references;
 *     - signal collections;
 *     - signal aliases where explicitly part of the signal abstraction;
 *     - signal assignment/reference syntax where needed by the signal model;
 *     - signal declaration modifiers.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - ports;
 *     - wires;
 *     - registers;
 *     - clocks;
 *     - memories;
 *     - pipelines;
 *     - processes;
 *     - combinational blocks;
 *     - sequential blocks;
 *     - state machines;
 *     - hardware modules;
 *     - hardware interfaces;
 *     - hardware targets;
 *     - physical pins;
 *     - physical nets;
 *     - placement;
 *     - routing;
 *     - synthesis;
 *     - timing closure;
 *     - hardware discovery;
 *     - resource allocation;
 *     - calibration;
 *     - device IDs;
 *     - machine topology;
 *     - runtime state;
 *     - quantum::ir;
 *     - quantum error-correction algorithms;
 *     - ZQN noise semantics.
 *
 * ============================================================================
 * SIGNAL SEMANTICS
 * ============================================================================
 *
 * A signal is a LOGICAL hardware communication/storage-independent value
 * carrier in the HDL semantic model.
 *
 * A signal does not inherently mean:
 *
 *     - one physical wire;
 *     - one FPGA routing resource;
 *     - one ASIC metal segment;
 *     - one package pin;
 *     - one device address;
 *     - one clock domain;
 *     - one memory cell;
 *     - one CPU register.
 *
 * Those are downstream implementation decisions unless explicitly introduced
 * as part of a target/deployment contract.
 *
 * ============================================================================
 * POCO-REAF AND SCALABILITY
 * ============================================================================
 *
 * There are intentionally no grammar-level limits on:
 *
 *     number of signals;
 *     signal width;
 *     number of dimensions;
 *     number of signal groups;
 *     number of modules;
 *     number of signal references;
 *     number of drivers.
 *
 * Repetition is used wherever cardinality is naturally unbounded.
 *
 * A declaration such as:
 *
 *     signal data: logic[WIDTH];
 *
 * expresses a logical parameterized signal.
 *
 * WIDTH may be resolved from:
 *
 *     a generic;
 *     a parameter;
 *     compile-time computation;
 *     a type-level expression;
 *     another semantic source.
 *
 * It is NOT a machine-capacity declaration.
 *
 * ============================================================================
 * HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * This file must never encode:
 *
 *     MAX_SIGNALS
 *     MAX_WIDTH
 *     MAX_BITS
 *     MAX_LANES
 *     MAX_DEVICES
 *     MAX_MODULES
 *     MAX_FANOUT
 *     FPGA family identifiers
 *     ASIC identifiers
 *     board identifiers
 *     physical pin numbers
 *     physical addresses
 *     routing coordinates
 *     topology assumptions.
 *
 * Such information belongs to hardware/resource/target/deployment layers.
 *
 * ============================================================================
 * LEXER INTEGRATION
 * ============================================================================
 *
 * This grammar must use the repository's canonical lexer vocabulary.
 *
 * The exact token-vocabulary name MUST be the one selected as authoritative
 * by grammar/lexer/ and the parser build.
 *
 * No second lexer is defined here.
 *
 * The following lexical categories are expected from the canonical lexer:
 *
 *     identifier
 *     integer/numeric literals
 *     string literals
 *     boolean literals
 *     punctuation
 *     operators
 *     annotation markers
 *
 * HDL keywords may either be canonical lexer tokens or contextual identifiers,
 * according to the repository's final keyword policy.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * PARSER GRAMMAR
 * ============================================================================
 *
 * NOTE:
 *
 * The grammar name must match the actual ANTLR grammar filename according to
 * the repository's ANTLR build convention.
 *
 * If the repository retains hyphenated source filenames, the build system must
 * map them explicitly to valid ANTLR grammar identifiers. Prefer valid
 * underscore/camel-case filenames for independently compilable ANTLR grammars.
 * ============================================================================
 */

parser grammar HardwareSignals;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. PUBLIC SIGNAL DECLARATION ENTRY POINT
 * ============================================================================
 *
 * A complete logical signal declaration.
 *
 * Canonical conceptual forms:
 *
 *     signal data: logic;
 *     signal valid: bool;
 *     signal data: logic[WIDTH];
 *     signal matrix: logic[ROWS][COLS];
 *
 * The semicolon is accepted as an optional parser boundary so the surrounding
 * HDL grammar can own statement termination policy where appropriate.
 * ============================================================================
 */

hdlSignalDeclaration
    : hdlSignalAttributes?
      hdlSignalModifiers*
      hdlSignalKeyword
      hdlSignalDeclaratorList
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 2. SIGNAL DECLARATION LIST
 * ============================================================================
 *
 * Multiple signals may share a declaration context.
 *
 * No fixed cardinality is imposed.
 * ============================================================================
 */

hdlSignalDeclaratorList
    : hdlSignalDeclarator
      (
          COMMA
          hdlSignalDeclarator
      )*
      COMMA?
    ;


hdlSignalDeclarator
    : hdlSignalName
      hdlSignalType?
      hdlSignalDimensions*
      hdlSignalInitializer?
      hdlSignalConstraints*
    ;


/*
 * ============================================================================
 * 3. SIGNAL KEYWORD
 * ============================================================================
 *
 * The canonical lexer should eventually provide a dedicated HDL signal token
 * where that is part of Zamani's lexical specification.
 *
 * The contextual form is retained to permit migration from an identifier-based
 * HDL vocabulary without embedding lexer policy into this parser component.
 * ============================================================================
 */

hdlSignalKeyword
    : hdlSignalCanonicalKeyword
    | hdlContextualSignalKeyword
    ;


hdlSignalCanonicalKeyword
    : SIGNAL
    ;


hdlContextualSignalKeyword
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * 4. SIGNAL MODIFIERS
 * ============================================================================
 *
 * Modifiers describe logical source-level semantics.
 *
 * They do not specify physical implementation.
 * ============================================================================
 */

hdlSignalModifiers
    : hdlSignalModifier
    ;


hdlSignalModifier
    : hdlSignalConstModifier
    | hdlSignalMutableModifier
    | hdlSignalReferenceModifier
    | hdlSignalInternalModifier
    | hdlSignalExternalModifier
    | hdlContextualSignalModifier
    ;


hdlSignalConstModifier
    : CONST
    ;


hdlSignalMutableModifier
    : MUT
    ;


hdlSignalReferenceModifier
    : REF
    ;


hdlSignalInternalModifier
    : INTERNAL
    ;


hdlSignalExternalModifier
    : EXTERNAL
    ;


hdlContextualSignalModifier
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * 5. SIGNAL NAME
 * ============================================================================
 *
 * Signal names are logical source symbols.
 *
 * They are not physical identifiers.
 * ============================================================================
 */

hdlSignalName
    : identifier
    ;


/*
 * ============================================================================
 * 6. SIGNAL TYPE
 * ============================================================================
 *
 * Signal type syntax is deliberately compatible with the shared type system.
 *
 * This grammar does not establish whether a type is semantically valid.
 *
 * Examples:
 *
 *     logic
 *     bool
 *     bit
 *     integer
 *     Vector<T>
 *     Bus<T, WIDTH>
 *
 * Type ownership remains in grammar/types/ and the HDL type semantic layer.
 * ============================================================================
 */

hdlSignalType
    : COLON
      hdlSignalTypeExpression
    ;


hdlSignalTypeExpression
    : hdlSignalTypePrimary
      hdlSignalTypeSuffix*
    ;


hdlSignalTypePrimary
    : identifier
    | hdlSignalQualifiedType
    | LPAREN
      hdlSignalTypeExpression
      RPAREN
    ;


hdlSignalQualifiedType
    : identifier
      (
          DOUBLE_COLON
          identifier
      )*
    ;


hdlSignalTypeSuffix
    : LT
      hdlSignalTypeArguments?
      GT
    | LBRACKET
      hdlSignalDimensionExpression?
      RBRACKET
    ;


hdlSignalTypeArguments
    : hdlSignalTypeArgument
      (
          COMMA
          hdlSignalTypeArgument
      )*
      COMMA?
    ;


hdlSignalTypeArgument
    : hdlSignalTypeExpression
    | hdlSignalExpression
    ;


/*
 * ============================================================================
 * 7. SIGNAL DIMENSIONS
 * ============================================================================
 *
 * Dimensions represent logical shape.
 *
 * They do not represent:
 *
 *     memory capacity;
 *     physical wiring;
 *     hardware resource count.
 * ============================================================================
 */

hdlSignalDimensions
    : LBRACKET
      hdlSignalDimensionExpression?
      RBRACKET
    ;


hdlSignalDimensionExpression
    : hdlSignalRangeExpression
    ;


hdlSignalRangeExpression
    : hdlSignalExpression
      hdlSignalRangeOperator?
      hdlSignalExpression?
    ;


hdlSignalRangeOperator
    : COLON
    | DOT_DOT
    | DOT_DOT_EQ
    ;


/*
 * ============================================================================
 * 8. INITIALIZATION
 * ============================================================================
 *
 * Signal initialization is source-level initialization semantics.
 *
 * Whether the target technology supports the resulting initialization is
 * determined later.
 * ============================================================================
 */

hdlSignalInitializer
    : ASSIGN
      hdlSignalExpression
    ;


/*
 * ============================================================================
 * 9. SIGNAL ATTRIBUTES
 * ============================================================================
 *
 * Attributes provide extensible source-level metadata.
 *
 * They must not silently imply target-specific behavior.
 * ============================================================================
 */

hdlSignalAttributes
    : hdlSignalAttribute+
    ;


hdlSignalAttribute
    : AT
      identifier
      (
          LPAREN
          hdlSignalArgumentList?
          RPAREN
      )?
    ;


hdlSignalArgumentList
    : hdlSignalArgument
      (
          COMMA
          hdlSignalArgument
      )*
      COMMA?
    ;


hdlSignalArgument
    : identifier
      ASSIGN
      hdlSignalExpression
    | hdlSignalExpression
    ;


/*
 * ============================================================================
 * 10. SIGNAL CONSTRAINTS
 * ============================================================================
 *
 * These are source-level/logical constraints.
 *
 * Physical constraints must be represented in the target/deployment
 * constraint system rather than embedded into logical signal syntax.
 * ============================================================================
 */

hdlSignalConstraints
    : hdlSignalConstraint
    ;


hdlSignalConstraint
    : hdlSignalAttributeConstraint
    | hdlSignalExpressionConstraint
    ;


hdlSignalAttributeConstraint
    : AT
      identifier
      (
          LPAREN
          hdlSignalArgumentList?
          RPAREN
      )?
    ;


hdlSignalExpressionConstraint
    : LBRACE
      hdlSignalConstraintExpressions?
      RBRACE
    ;


hdlSignalConstraintExpressions
    : hdlSignalExpression
      (
          COMMA
          hdlSignalExpression
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 11. SIGNAL REFERENCE
 * ============================================================================
 *
 * A signal reference is a logical reference to a declared signal.
 * ============================================================================
 */

hdlSignalReference
    : hdlSignalName
    | hdlSignalQualifiedReference
    ;


hdlSignalQualifiedReference
    : identifier
      (
          DOUBLE_COLON
          identifier
      )+
    ;


/*
 * ============================================================================
 * 12. SIGNAL INDEXING
 * ============================================================================
 *
 * Indexing selects part of a logical signal value.
 *
 * Semantic analysis determines:
 *
 *     - whether the signal is indexable;
 *     - whether the index is in range;
 *     - whether the resulting type is valid.
 * ============================================================================
 */

hdlSignalIndex
    : LBRACKET
      hdlSignalExpression
      RBRACKET
    ;


hdlSignalSlice
    : LBRACKET
      hdlSignalRangeExpression
      RBRACKET
    ;


hdlSignalSelection
    : hdlSignalReference
      (
          hdlSignalIndex
        | hdlSignalSlice
      )*
    ;


/*
 * ============================================================================
 * 13. SIGNAL MEMBER SELECTION
 * ============================================================================
 *
 * Allows structured/record-like signal values to expose logical members.
 * ============================================================================
 */

hdlSignalMemberSelection
    : hdlSignalReference
      (
          DOT
          identifier
      )+
    ;


/*
 * ============================================================================
 * 14. SIGNAL ACCESS
 * ============================================================================
 *
 * Unified source-level signal access.
 * ============================================================================
 */

hdlSignalAccess
    : hdlSignalSelection
    | hdlSignalMemberSelection
    | hdlSignalReference
    ;


/*
 * ============================================================================
 * 15. SIGNAL ASSIGNMENT
 * ============================================================================
 *
 * This rule describes logical assignment syntax only.
 *
 * It does not determine:
 *
 *     combinational vs sequential semantics;
 *     clocking;
 *     scheduling;
 *     propagation delay;
 *     physical routing;
 *     synthesis strategy.
 *
 * Those are owned by the appropriate HDL semantic grammars.
 * ============================================================================
 */

hdlSignalAssignment
    : hdlSignalAccess
      ASSIGN
      hdlSignalExpression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 16. SIGNAL CONNECTION
 * ============================================================================
 *
 * A signal connection establishes a source-level relationship.
 *
 * Physical netlist construction is downstream.
 * ============================================================================
 */

hdlSignalConnection
    : hdlSignalAccess
      ASSIGN
      hdlSignalAccess
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 17. SIGNAL ALIAS
 * ============================================================================
 *
 * Alias declarations introduce an additional logical name.
 *
 * They do not create an additional physical resource.
 * ============================================================================
 */

hdlSignalAlias
    : hdlSignalAliasKeyword
      hdlSignalName
      ASSIGN
      hdlSignalAccess
      SEMICOLON?
    ;


hdlSignalAliasKeyword
    : ALIAS
    | hdlContextualAliasKeyword
    ;


hdlContextualAliasKeyword
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * 18. SIGNAL GROUP
 * ============================================================================
 *
 * Signal groups provide logical grouping without a fixed number of members.
 *
 * Example conceptual form:
 *
 *     signal_group control {
 *         signal valid: bool;
 *         signal ready: bool;
 *     }
 *
 * The containing HDL semantic layer determines whether the grouping has
 * structural significance.
 * ============================================================================
 */

hdlSignalGroup
    : hdlSignalGroupKeyword
      hdlSignalGroupName?
      hdlSignalGroupType?
      LBRACE
      hdlSignalGroupMember*
      RBRACE
    ;


hdlSignalGroupKeyword
    : SIGNAL_GROUP
    | hdlContextualSignalGroupKeyword
    ;


hdlContextualSignalGroupKeyword
    : IDENTIFIER
    ;


hdlSignalGroupName
    : identifier
    ;


hdlSignalGroupType
    : COLON
      hdlSignalTypeExpression
    ;


hdlSignalGroupMember
    : hdlSignalDeclaration
    | hdlSignalGroup
    ;


/*
 * ============================================================================
 * 19. SIGNAL DECLARATION SEQUENCE
 * ============================================================================
 *
 * Used by HDL modules, interfaces, architectures and other HDL contexts.
 * ============================================================================
 */

hdlSignalDeclarationSequence
    : (
          hdlSignalDeclaration
        | hdlSignalGroup
      )*
    ;


/*
 * ============================================================================
 * 20. SIGNAL REFERENCE LIST
 * ============================================================================
 */

hdlSignalReferenceList
    : hdlSignalReference
      (
          COMMA
          hdlSignalReference
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 21. SIGNAL EXPRESSION
 * ============================================================================
 *
 * This expression grammar intentionally supports the compile-time/logical
 * expression forms needed by signal dimensions, initializers and constraints.
 *
 * It is NOT intended to replace Zamani's universal expression grammar.
 *
 * The final integrated parser should reuse the canonical expression rules
 * wherever parser composition permits.
 * ============================================================================
 */

hdlSignalExpression
    : hdlSignalLogicalOrExpression
    ;


hdlSignalLogicalOrExpression
    : hdlSignalLogicalAndExpression
      (
          LOGICAL_OR
          hdlSignalLogicalAndExpression
      )*
    ;


hdlSignalLogicalAndExpression
    : hdlSignalBitwiseOrExpression
      (
          LOGICAL_AND
          hdlSignalBitwiseOrExpression
      )*
    ;


hdlSignalBitwiseOrExpression
    : hdlSignalBitwiseXorExpression
      (
          PIPE
          hdlSignalBitwiseXorExpression
      )*
    ;


hdlSignalBitwiseXorExpression
    : hdlSignalBitwiseAndExpression
      (
          CARET
          hdlSignalBitwiseAndExpression
      )*
    ;


hdlSignalBitwiseAndExpression
    : hdlSignalEqualityExpression
      (
          AMPERSAND
          hdlSignalEqualityExpression
      )*
    ;


hdlSignalEqualityExpression
    : hdlSignalRelationalExpression
      (
          (
              EQUAL_EQUAL
            | NOT_EQUAL
          )
          hdlSignalRelationalExpression
      )*
    ;


hdlSignalRelationalExpression
    : hdlSignalAdditiveExpression
      (
          (
              LT
            | GT
            | LESS_EQUAL
            | GREATER_EQUAL
          )
          hdlSignalAdditiveExpression
      )*
    ;


hdlSignalAdditiveExpression
    : hdlSignalMultiplicativeExpression
      (
          (
              PLUS
            | MINUS
          )
          hdlSignalMultiplicativeExpression
      )*
    ;


hdlSignalMultiplicativeExpression
    : hdlSignalUnaryExpression
      (
          (
              STAR
            | SLASH
            | PERCENT
          )
          hdlSignalUnaryExpression
      )*
    ;


hdlSignalUnaryExpression
    : (
          PLUS
        | MINUS
        | EXCLAMATION
        | TILDE
      )
      hdlSignalUnaryExpression
    | hdlSignalPrimaryExpression
    ;


hdlSignalPrimaryExpression
    : hdlSignalReference
    | INTEGER
    | FLOAT
    | STRING
    | TRUE
    | FALSE
    | LPAREN
      hdlSignalExpression
      RPAREN
    ;


/*
 * ============================================================================
 * 22. SIGNAL VECTOR CONSTRUCTION
 * ============================================================================
 *
 * Logical construction of aggregate signal values.
 *
 * This does not dictate a target-specific implementation.
 * ============================================================================
 */

hdlSignalAggregate
    : LBRACE
      hdlSignalAggregateElements?
      RBRACE
    ;


hdlSignalAggregateElements
    : hdlSignalExpression
      (
          COMMA
          hdlSignalExpression
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 23. SIGNAL CAST
 * ============================================================================
 *
 * Source-level conversion syntax.
 *
 * Semantic/type analysis determines legality.
 * ============================================================================
 */

hdlSignalCast
    : LPAREN
      hdlSignalTypeExpression
      RPAREN
      hdlSignalExpression
    ;


/*
 * ============================================================================
 * 24. SIGNAL CONSTANT EXPRESSION
 * ============================================================================
 *
 * Used where a declaration requires a compile-time evaluable expression.
 *
 * The grammar does not guarantee compile-time evaluation.
 * That is a semantic/compiler responsibility.
 * ============================================================================
 */

hdlSignalConstantExpression
    : hdlSignalExpression
    ;


/*
 * ============================================================================
 * 25. SIGNAL WIDTH EXPRESSION
 * ============================================================================
 *
 * Width is a semantic quantity and may be symbolic.
 *
 * Examples:
 *
 *     WIDTH
 *     DATA_WIDTH
 *     LANES * ELEMENT_WIDTH
 *
 * No machine limit is embedded here.
 * ============================================================================
 */

hdlSignalWidthExpression
    : hdlSignalConstantExpression
    ;


/*
 * ============================================================================
 * 26. SIGNAL RANGE
 * ============================================================================
 */

hdlSignalRange
    : LBRACKET
      hdlSignalRangeExpression
      RBRACKET
    ;


/*
 * ============================================================================
 * 27. SIGNAL DECLARATION WITH EXPLICIT RANGE
 * ============================================================================
 *
 * Convenience composition rule for callers that need to distinguish an
 * explicitly ranged signal from a scalar declaration.
 * ============================================================================
 */

hdlRangedSignalDeclarator
    : hdlSignalName
      hdlSignalType?
      hdlSignalRange+
      hdlSignalInitializer?
      hdlSignalConstraints*
    ;


/*
 * ============================================================================
 * 28. SIGNAL COLLECTION
 * ============================================================================
 *
 * A named collection is logical source structure, not a hardware count.
 * ============================================================================
 */

hdlSignalCollection
    : hdlSignalCollectionKeyword
      hdlSignalName
      (
          COLON
          hdlSignalTypeExpression
      )?
      LBRACE
      hdlSignalCollectionMember*
      RBRACE
    ;


hdlSignalCollectionKeyword
    : SIGNALS
    | hdlContextualSignalCollectionKeyword
    ;


hdlContextualSignalCollectionKeyword
    : IDENTIFIER
    ;


hdlSignalCollectionMember
    : hdlSignalDeclaration
    | hdlSignalGroup
    | hdlSignalAlias
    ;


/*
 * ============================================================================
 * 29. PUBLIC REFERENCE CONTRACT
 * ============================================================================
 *
 * The following rules are intended as stable integration points:
 *
 *     hdlSignalDeclaration
 *     hdlSignalDeclarationSequence
 *     hdlSignalDeclarator
 *     hdlSignalReference
 *     hdlSignalAccess
 *     hdlSignalAssignment
 *     hdlSignalConnection
 *     hdlSignalGroup
 *
 * Other HDL grammars should consume these rules instead of duplicating signal
 * syntax.
 * ============================================================================
 */


/*
 * ============================================================================
 * 30. INTEGRATION WITH PORTS
 * ============================================================================
 *
 * Ports and signals are distinct concepts.
 *
 * PORT:
 *
 *     module/interface boundary.
 *
 * SIGNAL:
 *
 *     internal or logically declared HDL value carrier.
 *
 * A signal may be associated with a port by semantic analysis or an explicit
 * connection construct.
 *
 * ports.g4 MUST NOT duplicate these signal declaration rules.
 *
 * signals.g4 MUST NOT duplicate port declarations.
 * ============================================================================
 */


/*
 * ============================================================================
 * 31. INTEGRATION WITH WIRES
 * ============================================================================
 *
 * signals.g4:
 *
 *     logical signal declaration/reference.
 *
 * wires.g4:
 *
 *     connectivity/net semantics.
 *
 * A wire may connect signals, ports and other HDL objects.
 *
 * signals.g4 must not determine physical connectivity.
 * ============================================================================
 */


/*
 * ============================================================================
 * 32. INTEGRATION WITH REGISTERS
 * ============================================================================
 *
 * registers.g4 owns storage/state semantics.
 *
 * A register may expose or consume signals.
 *
 * signals.g4 does not define:
 *
 *     clock edge semantics;
 *     reset semantics;
 *     state retention;
 *     sequential scheduling.
 * ============================================================================
 */


/*
 * ============================================================================
 * 33. INTEGRATION WITH CLOCKS
 * ============================================================================
 *
 * clocks.g4 owns clock declarations and clock-domain semantics.
 *
 * A signal may be associated with a clock domain semantically, but this file
 * does not create clock definitions or timing models.
 * ============================================================================
 */


/*
 * ============================================================================
 * 34. INTEGRATION WITH PROCESSES
 * ============================================================================
 *
 * processes.g4 owns process execution semantics.
 *
 * Processes may read/write/reference signals through:
 *
 *     hdlSignalReference
 *     hdlSignalAccess
 *     hdlSignalAssignment
 *
 * This prevents process grammar from creating duplicate signal syntax.
 * ============================================================================
 */


/*
 * ============================================================================
 * 35. INTEGRATION WITH COMBINATIONAL/SEQUENTIAL GRAMMARS
 * ============================================================================
 *
 * combinational.g4 and sequential.g4 may consume signal references and
 * assignments.
 *
 * They determine the semantic execution class.
 *
 * signals.g4 does not decide whether an assignment is combinational or
 * sequential merely from its syntax.
 * ============================================================================
 */


/*
 * ============================================================================
 * 36. INTEGRATION WITH STATE MACHINES
 * ============================================================================
 *
 * state-machines.g4 may use signal references for:
 *
 *     state inputs;
 *     transition conditions;
 *     outputs;
 *     control signals.
 *
 * State-machine semantics remain owned by state-machines.g4.
 * ============================================================================
 */


/*
 * ============================================================================
 * 37. INTEGRATION WITH HARDWARE MODULES
 * ============================================================================
 *
 * hardware-modules.g4 consumes:
 *
 *     hdlSignalDeclaration
 *     hdlSignalDeclarationSequence
 *     hdlSignalGroup
 *     hdlSignalReference
 *
 * A hardware module owns containment.
 *
 * This file owns signal syntax.
 * ============================================================================
 */


/*
 * ============================================================================
 * 38. INTEGRATION WITH HARDWARE INTERFACES
 * ============================================================================
 *
 * hardware-interfaces.g4 may expose signal contracts as part of an interface.
 *
 * Interface semantics remain separate from internal signal implementation.
 * ============================================================================
 */


/*
 * ============================================================================
 * 39. INTEGRATION WITH HARDWARE PARAMETERS / GENERICS
 * ============================================================================
 *
 * Signal dimensions and widths may reference generic/parameter symbols.
 *
 * Example:
 *
 *     signal data: logic[WIDTH];
 *
 * WIDTH resolution belongs to generic/parameter semantic analysis.
 *
 * signals.g4 must not define generic evaluation.
 * ============================================================================
 */


/*
 * ============================================================================
 * 40. INTEGRATION WITH RESOURCE MODEL
 * ============================================================================
 *
 * A signal declaration does not reserve physical resources.
 *
 * Resource analysis may later derive requirements such as:
 *
 *     width;
 *     fanout;
 *     bandwidth;
 *     storage;
 *     connectivity;
 *     timing;
 *
 * Those are downstream interpretations.
 *
 * This file must never convert a signal into a fixed physical allocation.
 * ============================================================================
 */


/*
 * ============================================================================
 * 41. INTEGRATION WITH HARDWARE TARGETS
 * ============================================================================
 *
 * Hardware target information is external to this grammar.
 *
 * Examples:
 *
 *     FPGA family
 *     ASIC technology
 *     package
 *     board
 *     device
 *     physical I/O
 *
 * must be represented by target/deployment models rather than source signal
 * syntax unless the user explicitly expresses a semantic target requirement.
 * ============================================================================
 */


/*
 * ============================================================================
 * 42. QUANTUM / HYBRID INTEGRATION
 * ============================================================================
 *
 * Signals may carry classical control/data associated with quantum operations.
 *
 * This grammar MUST NOT define quantum operations.
 *
 * Quantum syntax remains owned by:
 *
 *     grammar/quantum/
 *
 * Quantum semantic lowering remains connected to:
 *
 *     quantum::ir
 *
 * Hybrid grammar may compose:
 *
 *     classical expression
 *          +
 *     signal reference
 *          +
 *     quantum semantic operation
 *
 * without making signals a second quantum representation.
 *
 * QEC and ZQN remain outside this grammar.
 * ============================================================================
 */


/*
 * ============================================================================
 * 43. AST CONTRACT
 * ============================================================================
 *
 * The frontend AST representation generated from this grammar should preserve:
 *
 *     declaration span;
 *     declaration modifiers;
 *     signal name;
 *     type syntax;
 *     dimensions;
 *     initializer;
 *     attributes;
 *     constraints;
 *     source ordering;
 *     source locations;
 *     qualified references.
 *
 * The parser must not manufacture:
 *
 *     physical IDs;
 *     device IDs;
 *     resource allocations;
 *     scheduling decisions;
 *     routing decisions.
 *
 * AST nodes should retain symbolic expressions rather than prematurely
 * evaluating target-dependent quantities.
 * ============================================================================
 */


/*
 * ============================================================================
 * 44. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must subsequently determine:
 *
 *     - duplicate signal names;
 *     - visibility;
 *     - type validity;
 *     - dimension validity;
 *     - width compatibility;
 *     - initialization legality;
 *     - driver legality;
 *     - read/write legality;
 *     - signal/port compatibility;
 *     - clock-domain legality;
 *     - process interaction;
 *     - alias validity;
 *     - constraint validity;
 *     - dialect-specific restrictions.
 *
 * The parser must not perform those semantic decisions.
 * ============================================================================
 */


/*
 * ============================================================================
 * 45. IR CONTRACT
 * ============================================================================
 *
 * signals.g4 does not construct hardware IR.
 *
 * The frontend lowers the validated AST into the repository's canonical
 * hardware semantic representation.
 *
 * If the repository already has a hardware IR, that IR remains authoritative.
 *
 * If a canonical hardware IR does not yet exist, its design must be established
 * downstream rather than introducing an ad-hoc IR in this grammar directory.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 46. COMPILER CONTRACT
 * ============================================================================
 *
 * Compiler stages may use signal semantics for:
 *
 *     optimization;
 *     dependency analysis;
 *     width inference;
 *     scheduling;
 *     routing;
 *     synthesis;
 *     resource estimation;
 *     target lowering.
 *
 * None of those decisions are encoded as parser actions.
 * ============================================================================
 */


/*
 * ============================================================================
 * 47. RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime does not depend directly on this grammar.
 *
 * Runtime consumes compiled/validated representations.
 *
 * Therefore:
 *
 *     grammar -> runtime
 *
 * is NOT a direct architectural dependency.
 *
 * The correct direction is:
 *
 *     grammar
 *        ->
 *     AST
 *        ->
 *     semantic model
 *        ->
 *     compiler/IR
 *        ->
 *     executable/runtime representation.
 * ============================================================================
 */


/*
 * ============================================================================
 * 48. TOOLING CONTRACT
 * ============================================================================
 *
 * Tooling may consume parser/AST information for:
 *
 *     syntax highlighting;
 *     formatting;
 *     navigation;
 *     symbol indexing;
 *     diagnostics;
 *     language servers;
 *     refactoring;
 *     documentation generation.
 *
 * Public rule names should therefore remain stable after release.
 * ============================================================================
 */


/*
 * ============================================================================
 * 49. DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no embedded actions;
 *     no filesystem access;
 *     no network access;
 *     no target discovery;
 *     no runtime state;
 *     no mutable semantic state.
 *
 * Parsing therefore depends only on:
 *
 *     source token stream;
 *     grammar version;
 *     parser configuration.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 50. NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The test suite must reject malformed forms including:
 *
 *     signal;
 *     signal : logic;
 *     signal data: ;
 *     signal data[;
 *     signal data: logic[];
 *     signal data: logic[WIDTH;
 *     signal data = ;
 *     signal data: logic = ;
 *     signal data, : logic;
 *     signal data: logic,;
 *     malformed attributes;
 *     malformed ranges;
 *     malformed qualified names;
 *     unmatched delimiters.
 *
 * Semantic-invalid but syntactically valid examples must be tested separately.
 * ============================================================================
 */


/*
 * ============================================================================
 * 51. POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * Tests must cover:
 *
 *     scalar signals;
 *     typed signals;
 *     symbolic widths;
 *     arithmetic widths;
 *     multidimensional signals;
 *     parameterized signal types;
 *     initializers;
 *     attributes;
 *     constraints;
 *     qualified references;
 *     indexing;
 *     slicing;
 *     member selection;
 *     assignments;
 *     connections;
 *     aliases;
 *     signal groups;
 *     declaration lists;
 *     large generated signal collections.
 * ============================================================================
 */


/*
 * ============================================================================
 * 52. SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Generate test sources containing:
 *
 *     one signal;
 *     many signals;
 *     very large symbolic widths;
 *     many dimensions;
 *     large declaration sequences;
 *     deeply nested logical grouping within parser-resource limits.
 *
 * The tests must verify that no grammar rule introduces an artificial machine
 * limit.
 *
 * "Infinity" here means unbounded by the language grammar; actual execution
 * remains constrained only by available compiler/parser/runtime resources.
 * ============================================================================
 */


/*
 * ============================================================================
 * 53. CROSS-DOMAIN TEST CONTRACT
 * ============================================================================
 *
 * Test signal syntax in contexts involving:
 *
 *     classical computation;
 *     quantum-control data;
 *     hybrid quantum/classical computation;
 *     accelerator interfaces;
 *     distributed hardware interfaces;
 *     AI/tensor data paths;
 *     memory systems;
 *     networking hardware;
 *     embedded systems.
 *
 * The tests verify composition, not that signals acquire ownership of those
 * other domains.
 * ============================================================================
 */


/*
 * ============================================================================
 * 54. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing valid signal syntax must be:
 *
 *     preserved;
 *     migrated;
 *     or explicitly deprecated.
 *
 * No existing feature may be silently removed.
 *
 * Grammar versioning and migration policy belong to:
 *
 *     grammar/compatibility/
 * ============================================================================
 */


/*
 * ============================================================================
 * 55. HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden:
 *
 *     MAX_SIGNALS
 *     MAX_WIDTH
 *     MAX_DIMENSIONS
 *     MAX_GROUP_SIZE
 *     MAX_FANOUT
 *     MAX_MODULES
 *     MAX_CONNECTIONS
 *     DEVICE_ID
 *     PIN_ID
 *     FPGA_ID
 *     ASIC_ID
 *     BOARD_ID
 *     fixed topology
 *     fixed physical address
 *     fixed hardware count.
 *
 * Numeric literals occurring in source expressions are values, not language
 * resource limits.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 56. SECURITY / SAFETY
 * ============================================================================
 *
 * This grammar must not:
 *
 *     execute expressions;
 *     access files;
 *     access networks;
 *     inspect hardware;
 *     allocate hardware;
 *     invoke drivers;
 *     invoke runtime APIs.
 *
 * Rust integration must remain safe Rust.
 *
 * No `unsafe` code is introduced by this grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 57. COMPLETION CRITERIA
 * ============================================================================
 *
 * signals.g4 is COMPLETE only when:
 *
 *     [ ] canonical lexer vocabulary is confirmed;
 *     [ ] canonical token names are confirmed;
 *     [ ] signal declaration syntax is stable;
 *     [ ] signal references are stable;
 *     [ ] signal indexing/slicing is stable;
 *     [ ] signal groups are stable;
 *     [ ] signal attributes are stable;
 *     [ ] signal constraints are stable;
 *     [ ] signal initialization is stable;
 *     [ ] signal assignment syntax is stable;
 *     [ ] no port grammar is duplicated;
 *     [ ] no wire grammar is duplicated;
 *     [ ] no register grammar is duplicated;
 *     [ ] no clock grammar is duplicated;
 *     [ ] no process grammar is duplicated;
 *     [ ] no general type system is duplicated;
 *     [ ] no general expression system is permanently duplicated;
 *     [ ] no physical hardware assumptions exist;
 *     [ ] no fixed machine limits exist;
 *     [ ] no device identifiers exist;
 *     [ ] no resource allocation occurs;
 *     [ ] no scheduling occurs;
 *     [ ] no routing occurs;
 *     [ ] no synthesis occurs;
 *     [ ] no quantum::ir duplication exists;
 *     [ ] no QEC ownership is introduced;
 *     [ ] no ZQN ownership is introduced;
 *     [ ] AST integration is defined;
 *     [ ] semantic integration is defined;
 *     [ ] hardware IR integration is defined;
 *     [ ] compiler integration is defined;
 *     [ ] runtime non-dependency is documented;
 *     [ ] tooling integration is defined;
 *     [ ] positive tests pass;
 *     [ ] negative tests pass;
 *     [ ] boundary tests pass;
 *     [ ] scalability tests pass;
 *     [ ] cross-domain tests pass;
 *     [ ] determinism tests pass;
 *     [ ] compatibility tests pass.
 *
 * ============================================================================
 */
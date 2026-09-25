/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hdl/signals.g4
 *
 * Status:
 *     CANONICAL HDL SIGNAL DELEGATE
 *
 * Purpose:
 *     Define target-independent logical HDL signal syntax.
 *
 * Language objective:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *     (POCO-REAF)
 *
 * Rust baseline:
 *
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *
 *     This grammar contains no embedded Rust actions, semantic predicates,
 *     filesystem access, network access, runtime callbacks, hardware access,
 *     or unsafe code.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical Zamani lexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          v
 *     HDL composition root
 *          |
 *          +--> modules
 *          +--> ports
 *          +--> signals       <--- THIS FILE
 *          +--> wires
 *          +--> registers
 *          +--> memories
 *          +--> clocks
 *          +--> timing
 *          +--> processes
 *          +--> pipelines
 *          +--> verification
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> name resolution
 *          +--> type analysis
 *          +--> shape/width analysis
 *          +--> driver analysis
 *          +--> assignment analysis
 *          +--> clock-domain analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          |
 *          v
 *     canonical hardware semantic representation / IR
 *          |
 *          v
 *     optimization / verification / scheduling / synthesis / routing
 *          |
 *          v
 *     target realization
 *
 * Grammar defines syntax.
 * Semantic analysis defines meaning.
 * Hardware compilation defines realization.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - logical signal declarations;
 *     - signal declaration lists;
 *     - signal declarators;
 *     - signal names;
 *     - signal type attachment;
 *     - logical signal dimensions;
 *     - signal initializers;
 *     - signal metadata attachment points;
 *     - signal constraints;
 *     - logical signal references;
 *     - logical signal selections;
 *     - signal indexing/slicing syntax;
 *     - logical signal member access;
 *     - logical signal assignment syntax;
 *     - signal aliases;
 *     - signal groups.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical token definitions;
 *     - identifiers;
 *     - qualified names;
 *     - general expressions;
 *     - general types;
 *     - attributes;
 *     - ports;
 *     - wires/nets;
 *     - registers;
 *     - memories;
 *     - clocks;
 *     - timing;
 *     - processes;
 *     - combinational semantics;
 *     - sequential semantics;
 *     - state machines;
 *     - pipelines;
 *     - modules;
 *     - interfaces;
 *     - physical routing;
 *     - placement;
 *     - synthesis;
 *     - target selection;
 *     - hardware discovery;
 *     - resource allocation;
 *     - calibration;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - runtime execution.
 *
 * ============================================================================
 * SINGLE-SOURCE CONTRACT
 * ============================================================================
 *
 * This grammar deliberately consumes shared language contracts instead of
 * recreating them.
 *
 * Identifier/name ownership:
 *
 *     grammar/core/
 *
 * Expression ownership:
 *
 *     grammar/expressions/
 *
 * Type ownership:
 *
 *     grammar/types/
 *
 * Attribute ownership:
 *
 *     grammar/core/attributes.g4
 *
 * Range ownership:
 *
 *     grammar/expressions/
 *
 * The signal grammar must not introduce a second:
 *
 *     identifier grammar;
 *     qualified-name grammar;
 *     expression precedence hierarchy;
 *     type system;
 *     attribute syntax.
 *
 * ============================================================================
 * CANONICAL LEXER
 * ============================================================================
 *
 * The parser consumes:
 *
 *     ZamaniLexer
 *
 * through the canonical lexer composition:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * No lexer is defined here.
 *
 * The HDL lexical vocabulary MUST be established by the canonical lexer.
 *
 * In particular, the production HDL composition expects the canonical
 * signal keyword token:
 *
 *     K_SIGNAL
 *
 * The token must be owned by the canonical keyword vocabulary rather than
 * being recreated in this parser grammar.
 *
 * ============================================================================
 * IMPORTANT LEXER INTEGRATION NOTE
 * ============================================================================
 *
 * The current repository state contains HDL parser consumers of:
 *
 *     K_SIGNAL
 *     K_INPUT
 *     K_OUTPUT
 *     K_INOUT
 *
 * while the current keyword grammar does not yet expose the complete K_*
 * HDL keyword family.
 *
 * That is a repository-wide lexical conformance issue.
 *
 * It MUST be fixed in:
 *
 *     grammar/lexer/keywords.g4
 *
 * and propagated through:
 *
 *     grammar/lexer/tokens.g4
 *     grammar/antlr/ZamaniLexer.g4
 *
 * It MUST NOT be solved by adding a second lexer or by defining parser-local
 * fake tokens.
 *
 * This file therefore consumes the intended canonical token:
 *
 *     K_SIGNAL
 *
 * ============================================================================
 * POCO-REAF / OPEN-WORLD PRINCIPLE
 * ============================================================================
 *
 * A signal is a logical value-bearing HDL object.
 *
 * The grammar imposes no universal limit on:
 *
 *     number of signals;
 *     signal width;
 *     number of dimensions;
 *     number of signal groups;
 *     number of signal references;
 *     number of aliases;
 *     number of declarations;
 *     number of modules;
 *     number of generated signals.
 *
 * Repetition is represented by grammar repetition operators.
 *
 * There is intentionally no:
 *
 *     MAX_SIGNALS
 *     MAX_WIDTH
 *     MAX_BITS
 *     MAX_LANES
 *     MAX_DIMENSIONS
 *     MAX_MODULES
 *     MAX_FANOUT
 *     MAX_DEVICES
 *
 * A practical implementation may run out of memory or compilation resources,
 * but such implementation limits are not language-level semantic limits.
 *
 * ============================================================================
 * HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * This grammar MUST NOT encode:
 *
 *     physical pin numbers;
 *     FPGA routing identifiers;
 *     ASIC metal identifiers;
 *     package locations;
 *     board identifiers;
 *     physical addresses;
 *     vendor primitives;
 *     fixed device topology;
 *     fixed register widths;
 *     fixed memory sizes;
 *     fixed accelerator counts.
 *
 * For example:
 *
 *     signal data: logic[WIDTH];
 *
 * is valid logical source intent.
 *
 * The grammar does not decide whether WIDTH is realized by:
 *
 *     one wire;
 *     multiple wires;
 *     a bus;
 *     a vector register;
 *     an accelerator interface;
 *     distributed communication;
 *     another target-specific mechanism.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST must be able to preserve at least:
 *
 *     SignalDeclaration
 *     SignalDeclarator
 *     SignalName
 *     SignalType
 *     SignalDimension
 *     SignalInitializer
 *     SignalConstraint
 *     SignalReference
 *     SignalSelection
 *     SignalIndex
 *     SignalSlice
 *     SignalMemberAccess
 *     SignalAssignment
 *     SignalAlias
 *     SignalGroup
 *
 * Exact Rust AST type names belong to:
 *
 *     src/frontend/ast/
 *
 * This grammar does not construct AST objects.
 *
 * Source spans must remain available through the parser/frontend pipeline.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     - declaration/name uniqueness;
 *     - signal scope;
 *     - type validity;
 *     - dimension validity;
 *     - dimension evaluation;
 *     - width compatibility;
 *     - shape compatibility;
 *     - initializer compatibility;
 *     - assignment legality;
 *     - read/write legality;
 *     - driver analysis;
 *     - alias validity;
 *     - group validity;
 *     - clock-domain rules;
 *     - reset rules;
 *     - protocol rules;
 *     - capability requirements;
 *     - resource requirements;
 *     - target feasibility.
 *
 * Parsing must not perform these checks.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY BOUNDARY
 * ============================================================================
 *
 * The grammar describes logical signal requirements.
 *
 * It does not decide whether a target has:
 *
 *     enough routing;
 *     enough I/O;
 *     enough storage;
 *     enough registers;
 *     enough memory;
 *     enough lanes;
 *     enough bandwidth;
 *     enough devices.
 *
 * Such decisions belong downstream to:
 *
 *     semantic resource analysis;
 *     hardware capabilities;
 *     compilation;
 *     scheduling;
 *     placement;
 *     routing;
 *     synthesis;
 *     deployment;
 *     runtime.
 *
 * ============================================================================
 * CROSS-DOMAIN INTEGRATION
 * ============================================================================
 *
 * Classical:
 *
 *     Signals may carry classical values.
 *
 * Quantum:
 *
 *     A signal may participate in a quantum/classical control boundary when
 *     explicitly permitted by the semantic model.
 *
 *     This grammar does not define quantum operations.
 *
 * Hybrid:
 *
 *     Signals may connect classical control and quantum-facing HDL structures.
 *
 * Hardware:
 *
 *     Hardware realization consumes the logical signal contract.
 *
 * Distributed:
 *
 *     A signal may eventually be lowered to a communication mechanism, but
 *     this grammar does not turn a signal into a network endpoint.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no actions;
 *     - no semantic predicates;
 *     - no runtime callbacks;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware discovery;
 *     - no randomness.
 *
 * For a fixed token stream and grammar version, parsing is deterministic.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Signal expressions and attributes are untrusted source input.
 *
 * Parsing must never:
 *
 *     execute expressions;
 *     access files;
 *     access devices;
 *     access networks;
 *     discover hardware;
 *     execute commands.
 *
 * Evaluation belongs to controlled semantic/compile-time subsystems.
 *
 * ============================================================================
 * PUBLIC RULE CONTRACT
 * ============================================================================
 *
 * Stable public rules supplied by this delegate:
 *
 *     hdlSignalDeclaration
 *     hdlSignalDeclarationList
 *     hdlSignalDeclarationItem
 *     hdlSignalDeclarator
 *     hdlSignalName
 *     hdlSignalType
 *     hdlSignalDimension
 *     hdlSignalDimensions
 *     hdlSignalInitializer
 *     hdlSignalConstraint
 *     hdlSignalReference
 *     hdlSignalSelection
 *     hdlSignalIndex
 *     hdlSignalSlice
 *     hdlSignalMemberAccess
 *     hdlSignalAssignment
 *     hdlSignalAlias
 *     hdlSignalGroup
 *     hdlSignalGroupMember
 *     hdlSignalDeclarationSequence
 *
 * Shared rules such as:
 *
 *     identifier
 *     qualifiedName
 *     typeExpression
 *     expression
 *     rangeExpression
 *     attribute
 *
 * are intentionally NOT redefined here.
 *
 * ============================================================================
 */

parser grammar HardwareSignals;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. SIGNAL DECLARATION
 * ============================================================================
 *
 * Canonical logical forms:
 *
 *     signal data: logic;
 *     signal valid: bool;
 *     signal data: Vector<bit, WIDTH>;
 *     signal matrix: Matrix<logic, ROWS, COLS>;
 *
 * Type semantics belong to grammar/types/ and semantic analysis.
 *
 * The semicolon is mandatory for a complete declaration.
 *
 * The enclosing HDL module/interface grammar may consume this rule directly.
 * ============================================================================
 */

hdlSignalDeclaration
    : attribute*
      K_SIGNAL
      hdlSignalDeclarationList
      SEMICOLON
    ;


/*
 * ============================================================================
 * 2. SIGNAL DECLARATION LIST
 * ============================================================================
 *
 * A declaration list may contain arbitrarily many declarators.
 *
 * Example:
 *
 *     signal a: logic, b: logic, c: logic;
 *
 * No finite cardinality is imposed.
 * ============================================================================
 */

hdlSignalDeclarationList
    : hdlSignalDeclarationItem
      (
          COMMA
          hdlSignalDeclarationItem
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 3. SIGNAL DECLARATION ITEM
 * ============================================================================
 *
 * This rule deliberately excludes the signal keyword and terminator.
 *
 * That makes it safe for declaration-list composition.
 * ============================================================================
 */

hdlSignalDeclarationItem
    : hdlSignalDeclarator
    ;


/*
 * ============================================================================
 * 4. SIGNAL DECLARATOR
 * ============================================================================
 *
 * Canonical forms:
 *
 *     data
 *     data: logic
 *     data: logic[WIDTH]
 *     data: Vector<bit, WIDTH>
 *     data: logic[ROWS][COLS] = initial_value
 *
 * ============================================================================
 */

hdlSignalDeclarator
    : hdlSignalName
      (
          COLON
          hdlSignalType
      )?
      hdlSignalDimensions*
      hdlSignalInitializer?
      hdlSignalConstraint*
    ;


/*
 * ============================================================================
 * 5. SIGNAL NAME
 * ============================================================================
 *
 * Names are owned by the shared core name system.
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
 * The type system owns type syntax.
 *
 * This adapter exists only to make the ownership boundary explicit.
 * ============================================================================
 */

hdlSignalType
    : typeExpression
    ;


/*
 * ============================================================================
 * 7. SIGNAL DIMENSION
 * ============================================================================
 *
 * A dimension is a logical shape/indexing declaration.
 *
 * It is not a physical resource declaration.
 *
 * Examples:
 *
 *     [WIDTH]
 *     [ROWS]
 *     [0..WIDTH-1]
 *     [LOW..HIGH]
 *
 * The canonical range/expression grammar owns the actual expression syntax.
 * ============================================================================
 */

hdlSignalDimension
    : LBRACKET
      (
          rangeExpression
        | expression
      )?
      RBRACKET
    ;


hdlSignalDimensions
    : hdlSignalDimension
    ;


/*
 * ============================================================================
 * 8. SIGNAL INITIALIZER
 * ============================================================================
 *
 * Initialization is structural syntax.
 *
 * Whether initialization is legal for a particular HDL target is semantic.
 * ============================================================================
 */

hdlSignalInitializer
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * 9. SIGNAL CONSTRAINT
 * ============================================================================
 *
 * A signal constraint is source-level metadata/contract syntax.
 *
 * It does not perform target analysis.
 *
 * Attribute syntax itself belongs to grammar/core/attributes.g4.
 * ============================================================================
 */

hdlSignalConstraint
    : attribute
    ;


/*
 * ============================================================================
 * 10. SIGNAL REFERENCE
 * ============================================================================
 *
 * A reference uses the canonical name system.
 *
 * Examples:
 *
 *     data
 *     block.data
 *     module::data
 * ============================================================================
 */

hdlSignalReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 11. SIGNAL INDEX
 * ============================================================================
 *
 * Index expressions are canonical expressions.
 *
 * Example:
 *
 *     data[i]
 *
 * Bounds and indexability are semantic checks.
 * ============================================================================
 */

hdlSignalIndex
    : LBRACKET
      expression
      RBRACKET
    ;


/*
 * ============================================================================
 * 12. SIGNAL SLICE
 * ============================================================================
 *
 * Slice/range syntax is owned by the canonical range expression grammar.
 *
 * Example:
 *
 *     data[high..low]
 *
 * The exact range operator is determined by the canonical expression
 * specification.
 * ============================================================================
 */

hdlSignalSlice
    : LBRACKET
      rangeExpression
      RBRACKET
    ;


/*
 * ============================================================================
 * 13. SIGNAL SELECTION
 * ============================================================================
 *
 * Selection may contain indexing and slicing.
 *
 * Examples:
 *
 *     data[i]
 *     data[high..low]
 *     block.data[i]
 *
 * ============================================================================
 */

hdlSignalSelection
    : hdlSignalReference
      (
          hdlSignalIndex
        | hdlSignalSlice
      )*
    ;


/*
 * ============================================================================
 * 14. SIGNAL MEMBER ACCESS
 * ============================================================================
 *
 * Member access uses canonical expression/name punctuation.
 *
 * This rule exists as an explicit HDL semantic boundary.
 * ============================================================================
 */

hdlSignalMemberAccess
    : hdlSignalReference
      (
          DOT
          identifier
      )+
    ;


/*
 * ============================================================================
 * 15. SIGNAL ACCESS
 * ============================================================================
 *
 * Unified source-level signal access.
 * ============================================================================
 */

hdlSignalAccess
    : hdlSignalSelection
    | hdlSignalMemberAccess
    | hdlSignalReference
    ;


/*
 * ============================================================================
 * 16. SIGNAL ASSIGNMENT
 * ============================================================================
 *
 * Assignment establishes a logical source-level relationship.
 *
 * It does not decide:
 *
 *     combinational behavior;
 *     sequential behavior;
 *     clocking;
 *     scheduling;
 *     propagation;
 *     physical routing.
 *
 * ============================================================================
 */

hdlSignalAssignment
    : hdlSignalAccess
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 17. SIGNAL ALIAS
 * ============================================================================
 *
 * An alias introduces another logical name.
 *
 * It does not create another physical hardware resource.
 * ============================================================================
 */

hdlSignalAlias
    : ALIAS
      hdlSignalName
      ASSIGN
      hdlSignalAccess
      SEMICOLON
    ;


/*
 * ============================================================================
 * 18. SIGNAL GROUP
 * ============================================================================
 *
 * Groups are logical source-level structures.
 *
 * They impose no fixed member count.
 *
 * ============================================================================
 */

hdlSignalGroup
    : SIGNAL_GROUP
      hdlSignalGroupName?
      (
          COLON
          typeExpression
      )?
      LBRACE
      hdlSignalGroupMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 19. SIGNAL GROUP NAME
 * ============================================================================
 */

hdlSignalGroupName
    : identifier
    ;


/*
 * ============================================================================
 * 20. SIGNAL GROUP MEMBER
 * ============================================================================
 *
 * A group may contain signals or nested groups.
 *
 * The semantic layer determines whether recursive grouping is permitted.
 * ============================================================================
 */

hdlSignalGroupMember
    : hdlSignalDeclaration
    | hdlSignalGroup
    ;


/*
 * ============================================================================
 * 21. SIGNAL DECLARATION SEQUENCE
 * ============================================================================
 *
 * Used by HDL modules, interfaces and other HDL declaration contexts.
 *
 * No fixed number of declarations is imposed.
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
 * 22. SIGNAL REFERENCE LIST
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
 * 23. SIGNAL ASSIGNMENT LIST
 * ============================================================================
 */

hdlSignalAssignmentList
    : hdlSignalAssignment*
    ;


/*
 * ============================================================================
 * 24. SIGNAL SELECTION LIST
 * ============================================================================
 */

hdlSignalSelectionList
    : hdlSignalSelection
      (
          COMMA
          hdlSignalSelection
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 25. SIGNAL ACCESS LIST
 * ============================================================================
 */

hdlSignalAccessList
    : hdlSignalAccess
      (
          COMMA
          hdlSignalAccess
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 26. COMPLETION / INTEGRATION CONTRACT
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] Signal syntax has one owner.
 *
 * [x] General identifiers are delegated to core.
 *
 * [x] Qualified names are delegated to core.
 *
 * [x] General expressions are delegated to expressions.
 *
 * [x] General types are delegated to types.
 *
 * [x] Attribute syntax is delegated to core attributes.
 *
 * [x] No duplicate expression precedence hierarchy exists here.
 *
 * [x] No duplicate type system exists here.
 *
 * [x] No duplicate attribute system exists here.
 *
 * [x] No physical hardware assumption exists here.
 *
 * [x] No machine-size limit exists here.
 *
 * [x] No quantum gate vocabulary exists here.
 *
 * [x] No quantum IR exists here.
 *
 * [x] No QEC implementation exists here.
 *
 * [x] No ZQN implementation exists here.
 *
 * [x] No routing/placement implementation exists here.
 *
 * [x] No scheduling implementation exists here.
 *
 * [x] No hardware discovery exists here.
 *
 * [x] No runtime behavior exists here.
 *
 * [x] No unsafe Rust dependency exists here.
 *
 * [x] Signal source spans can be preserved by the parser/frontend.
 *
 * [x] Signal declarations can be consumed by the HDL composition root.
 *
 * [x] Signal syntax can be reused by HDL interfaces/groups without creating
 *     another signal language.
 *
 * [x] Signal widths/shapes remain semantic expressions rather than machine
 *     capacity limits.
 *
 * [x] Logical signal semantics remain separate from physical realization.
 *
 * ============================================================================
 * REQUIRED REPOSITORY INTEGRATION
 * ============================================================================
 *
 * The following contracts must be satisfied by the surrounding repository:
 *
 *     grammar/core/
 *         identifier
 *         qualifiedName
 *
 *     grammar/expressions/
 *         expression
 *         rangeExpression
 *
 *     grammar/types/
 *         typeExpression
 *
 *     grammar/core/attributes.g4
 *         attribute
 *
 *     grammar/antlr/ZamaniLexer.g4
 *         K_SIGNAL
 *         canonical punctuation/operator tokens
 *
 *     grammar/hdl/hdl.g4
 *         hdlSignalDeclaration
 *         hdlSignalGroup
 *
 *     grammar/hdl/wires.g4
 *         signal references where logical connectivity is required
 *
 *     grammar/hdl/registers.g4
 *         signal references where storage interfaces are required
 *
 *     grammar/hdl/processes.g4
 *         hdlSignalAssignment
 *
 *     grammar/hdl/combinational.g4
 *         hdlSignalAccess
 *
 *     grammar/hdl/sequential.g4
 *         hdlSignalAccess
 *
 *     grammar/hdl/interfaces.g4
 *         signal declarations inside logical interfaces
 *
 *     grammar/hardware/
 *         capability/resource/realization analysis
 *
 *     grammar/resources/
 *         resource requirements and constraints
 *
 *     src/frontend/ast/
 *         domain-neutral AST representation
 *
 *     semantic analysis
 *         signal/type/shape/driver validation
 *
 *     canonical hardware semantic representation / IR
 *         logical signal representation
 *
 * ============================================================================
 * CRITICAL COMPATIBILITY RULE
 * ============================================================================
 *
 * This file intentionally does NOT provide compatibility aliases for the old
 * locally duplicated rules:
 *
 *     hdlSignalExpression
 *     hdlSignalRangeExpression
 *     hdlSignalRangeOperator
 *     hdlSignalTypeExpression
 *     hdlSignalTypePrimary
 *     hdlSignalTypeSuffix
 *     hdlSignalTypeArguments
 *     hdlSignalTypeArgument
 *
 * They were private duplicates of language-wide concepts and are not used by
 * the repository's canonical signal consumers.
 *
 * If another legacy grammar is found to depend on one of these names, that
 * grammar should be migrated to:
 *
 *     expression
 *     rangeExpression
 *     typeExpression
 *     qualifiedName
 *
 * rather than restoring a second signal-specific expression/type system.
 *
 * ============================================================================
 * FINAL PRINCIPLE
 * ============================================================================
 *
 * A Zamani HDL signal describes:
 *
 *     logical value
 *     logical shape
 *     logical relationship
 *
 * It does NOT describe:
 *
 *     physical wire count
 *     FPGA routing track
 *     ASIC metal segment
 *     package pin
 *     device number
 *     machine topology
 *
 * Therefore the same signal grammar can scale from a tiny implementation to
 * arbitrarily large heterogeneous hardware, subject to actual resources and
 * semantic requirements at realization time.
 *
 * ============================================================================
 */
/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hdl/ports.g4
 *
 * Status:
 *     CANONICAL HDL PORT SYNTAX DELEGATE
 *
 * Purpose:
 *     Define the target-independent syntax for logical HDL port declarations,
 *     port lists, port groups, port references, and logical port mappings.
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
 *     This file contains no embedded Rust actions, predicates, unsafe code,
 *     filesystem access, network access, hardware discovery, runtime
 *     execution, randomness, or environment-dependent behavior.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     canonical lexer
 *       |
 *       v
 *     canonical parser
 *       |
 *       +--> HDL composition
 *               |
 *               +--> modules
 *               +--> ports  <--- THIS FILE
 *               +--> interfaces
 *               +--> signals
 *               +--> wires
 *               +--> registers
 *               +--> clocks
 *               +--> timing
 *               +--> processes
 *               +--> memories
 *               +--> pipelines
 *               +--> verification
 *               |
 *               v
 *           domain-neutral AST
 *               |
 *               v
 *           semantic analysis
 *               |
 *               +--> name resolution
 *               +--> type checking
 *               +--> direction checking
 *               +--> shape/width checking
 *               +--> interface checking
 *               +--> capability analysis
 *               +--> resource analysis
 *               |
 *               v
 *           canonical hardware semantic model / IR
 *               |
 *               +--> optimization
 *               +--> scheduling
 *               +--> routing
 *               +--> synthesis
 *               +--> placement
 *               +--> target lowering
 *               |
 *               v
 *           target realization
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - logical HDL port declaration syntax;
 *     - port direction;
 *     - port mode/modifier syntax;
 *     - port names;
 *     - port type attachment;
 *     - logical port dimensions;
 *     - source-level port defaults;
 *     - port metadata/attributes;
 *     - logical port contracts;
 *     - port declaration lists;
 *     - logical port groups;
 *     - logical port references;
 *     - logical named/positional port associations;
 *     - source-level port maps.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer rules;
 *     - identifiers;
 *     - general expressions;
 *     - general types;
 *     - declarations outside ports;
 *     - modules;
 *     - interfaces;
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
 *     - package balls;
 *     - board locations;
 *     - device IDs;
 *     - physical addresses;
 *     - vendor placement;
 *     - routing;
 *     - scheduling;
 *     - synthesis;
 *     - calibration;
 *     - hardware discovery;
 *     - resource allocation;
 *     - runtime execution;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN.
 *
 * ============================================================================
 * POCO-REAF / OPEN-WORLD PRINCIPLE
 * ============================================================================
 *
 * A port is a logical interface.
 *
 * It MUST NOT encode a universal physical hardware assumption.
 *
 * Therefore this grammar contains no:
 *
 *     MAX_PORTS
 *     MAX_WIDTH
 *     MAX_LANES
 *     MAX_DIMENSIONS
 *     MAX_INTERFACES
 *     MAX_CONNECTIONS
 *     MAX_FANOUT
 *     MAX_PINS
 *
 * It also contains no:
 *
 *     FPGA pin identifiers
 *     ASIC pad identifiers
 *     package-ball identifiers
 *     device identifiers
 *     board identifiers
 *     fixed bus numbers
 *     fixed machine topology
 *
 * A finite implementation limit may exist downstream, but such a limit is
 * never a language-level limit.
 *
 * ============================================================================
 * IMPORTANT DESIGN RULE
 * ============================================================================
 *
 * This grammar MUST NOT reproduce the canonical expression grammar or the
 * canonical type grammar.
 *
 * Expressions belong to:
 *
 *     grammar/expressions/
 *
 * Types belong to:
 *
 *     grammar/types/
 *
 * Identifiers/names belong to:
 *
 *     grammar/core/
 *
 * Port syntax merely consumes those contracts.
 *
 * This prevents a second expression precedence hierarchy and a second type
 * system from developing inside the HDL grammar.
 *
 * ============================================================================
 * CANONICAL TOKEN VOCABULARY
 * ============================================================================
 *
 * This is a parser delegate.
 *
 * It consumes the canonical lexer vocabulary selected by the parser
 * composition layer.
 *
 * The authoritative production lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * The lexical source hierarchy is:
 *
 *     grammar/lexer/
 *
 * No lexer rules are defined here.
 *
 * ============================================================================
 *
 * KEYWORD POLICY
 * ============================================================================
 *
 * Port direction is intentionally represented by canonical lexical tokens:
 *
 *     K_INPUT
 *     K_OUTPUT
 *     K_INOUT
 *
 * This file deliberately DOES NOT provide:
 *
 *     direction : IDENTIFIER
 *
 * because doing so makes every identifier a potential direction and creates
 * ambiguity with the required port name.
 *
 * If future compatibility requires contextual direction words, that change
 * must be made in the canonical lexical/specification layer with an explicit
 * compatibility policy. It must not be hidden inside this grammar.
 *
 * ============================================================================
 * PORT SEMANTICS
 * ============================================================================
 *
 * Direction:
 *
 *     input
 *     output
 *     inout
 *
 * is logical information.
 *
 * It does not directly mean:
 *
 *     physical input pin
 *     physical output pin
 *     tri-state package connection
 *     FPGA I/O bank
 *     ASIC pad
 *
 * Those meanings require downstream target semantics.
 *
 * ============================================================================
 * TYPE / WIDTH SEMANTICS
 * ============================================================================
 *
 * Examples such as:
 *
 *     logic
 *     bool
 *     Vector<T, WIDTH>
 *     Tensor<T, Shape>
 *     Bus<T>
 *
 * are source-level type expressions.
 *
 * A width such as:
 *
 *     WIDTH
 *
 * or:
 *
 *     2 * LANES
 *
 * is a program/type-level expression.
 *
 * It is not a declaration of physical machine capacity.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser must preserve sufficient source structure for the frontend AST
 * to represent at least:
 *
 *     HdlPortDeclaration
 *     HdlPortDirection
 *     HdlPortMode
 *     HdlPortType
 *     HdlPortDimension
 *     HdlPortDefault
 *     HdlPortAttribute
 *     HdlPortGroup
 *     HdlPortReference
 *     HdlPortAssociation
 *     HdlPortMap
 *
 * The grammar MUST NOT construct AST nodes.
 *
 * Source spans are supplied by the parser/frontend infrastructure.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     - whether a port name is valid;
 *     - whether names collide;
 *     - whether direction is legal in the containing declaration;
 *     - whether the type is valid;
 *     - whether dimensions are valid;
 *     - whether dimensions are evaluable where required;
 *     - whether defaults are legal;
 *     - whether attributes are recognized;
 *     - whether port groups are legal;
 *     - whether associations match declarations;
 *     - whether directions are compatible;
 *     - whether connected types are compatible;
 *     - whether widths/shapes are compatible;
 *     - whether a target can realize the resulting interface.
 *
 * None of these decisions are performed by this grammar.
 *
 * ============================================================================
 * RESOURCE / HARDWARE BOUNDARY
 * ============================================================================
 *
 * This grammar MUST NOT determine:
 *
 *     - whether a target has enough I/O;
 *     - whether a bus fits available routing;
 *     - whether a width fits an FPGA;
 *     - whether a port maps to a physical pin;
 *     - whether a voltage is supported;
 *     - whether a clock can meet timing;
 *     - whether a package has enough pads;
 *     - whether a device supports the interface;
 *     - which device is selected.
 *
 * Those belong downstream to hardware, resources, capabilities, compilation,
 * placement, routing, synthesis, deployment, and runtime systems.
 *
 * ============================================================================
 * CROSS-DOMAIN CONTRACT
 * ============================================================================
 *
 * Classical:
 *
 *     A port may expose classical data/control values.
 *
 * Quantum:
 *
 *     A port may expose quantum/classical interface semantics only where the
 *     surrounding HDL/quantum semantic model permits it.
 *
 *     This file does NOT define quantum operations or quantum::ir.
 *
 * Hybrid:
 *
 *     Port declarations may form the boundary between classical and quantum
 *     components.
 *
 * Hardware:
 *
 *     Hardware realization consumes the logical port contract.
 *
 * Distributed:
 *
 *     A logical port may later be lowered to a communication endpoint, but
 *     this grammar does not turn ports into network addresses.
 *
 * ============================================================================
 * CONNECTION OWNERSHIP
 * ============================================================================
 *
 * This file owns source-level port association syntax.
 *
 * It does NOT own:
 *
 *     physical routing
 *     placement
 *     wiring algorithms
 *     topology
 *     scheduling
 *
 * Therefore:
 *
 *     data_in = source
 *
 * means logical association.
 *
 * It does not mean:
 *
 *     route physical pin X to physical resource Y.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * Hardware module grammar may consume:
 *
 *     hdlPortDeclaration
 *     hdlPortList
 *     hdlPortDeclarationItem
 *     hdlPortGroup
 *     hdlPortMap
 *
 * Interface grammar may consume:
 *
 *     hdlPortDeclaration
 *     hdlPortList
 *     hdlPortGroup
 *     hdlPortReference
 *
 * Hardware semantic analysis consumes the resulting AST.
 *
 * ============================================================================
 */

parser grammar HardwarePorts;

options {
    /*
     * Canonical generated lexer vocabulary.
     *
     * This grammar does not define a lexer.
     */
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. COMPLETE PORT DECLARATION
 * ============================================================================
 *
 * Canonical forms:
 *
 *     input data: logic;
 *     output result: logic;
 *     inout bus: logic[WIDTH];
 *
 * A port has exactly one direction and exactly one logical name.
 *
 * The port name is intentionally placed immediately after direction/modes so
 * that arbitrary identifiers cannot be consumed as contextual directions.
 *
 * The surrounding HDL module/interface grammar owns whether a semicolon is
 * required. This delegate accepts the declaration terminator because the same
 * rule is useful in standalone HDL declaration contexts.
 * ============================================================================
 */

hdlPortDeclaration
    : hdlPortAttributes?
      hdlPortDirection
      hdlPortMode*
      hdlPortName
      hdlPortType?
      hdlPortDimension*
      hdlPortDefaultValue?
      hdlPortAttributeConstraint*
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 2. PORT DECLARATION ITEM
 * ============================================================================
 *
 * A reusable form without a declaration terminator.
 *
 * This is the preferred rule for comma/semicolon-separated lists owned by
 * containing grammars.
 * ============================================================================
 */

hdlPortDeclarationItem
    : hdlPortAttributes?
      hdlPortDirection
      hdlPortMode*
      hdlPortName
      hdlPortType?
      hdlPortDimension*
      hdlPortDefaultValue?
      hdlPortAttributeConstraint*
    ;


/*
 * ============================================================================
 * 3. PORT LIST
 * ============================================================================
 *
 * No fixed cardinality is imposed.
 *
 * Both empty and arbitrarily long lists are syntactically representable.
 *
 * The containing module/interface grammar may impose its own contextual rule
 * about whether an empty list is meaningful.
 * ============================================================================
 */

hdlPortList
    : LPAREN
      hdlPortDeclarationList?
      RPAREN
    ;


hdlPortDeclarationList
    : hdlPortDeclarationItem
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


/*
 * ============================================================================
 * 4. PORT DIRECTION
 * ============================================================================
 *
 * IMPORTANT:
 *
 * There is deliberately no IDENTIFIER fallback here.
 *
 * This removes the ambiguity present in the previous implementation where:
 *
 *     IDENTIFIER
 *
 * could mean input, output, inout, mode, or port name.
 *
 * If the canonical lexer later changes these words from reserved tokens to
 * contextual keywords, the lexer/specification contract must change together
 * with this rule.
 * ============================================================================
 */

hdlPortDirection
    : K_INPUT
    | K_OUTPUT
    | K_INOUT
    ;


/*
 * ============================================================================
 * 5. PORT MODES
 * ============================================================================
 *
 * Modes are intentionally limited to canonical lexical tokens.
 *
 * Do NOT use IDENTIFIER here.
 *
 * An arbitrary IDENTIFIER mode would make:
 *
 *     input data: logic;
 *
 * ambiguous because "data" could be consumed as a mode instead of the port
 * name.
 *
 * Additional universal modes must be promoted through the lexical/specification
 * authority before being added here.
 *
 * Vendor-specific modes belong to dialects rather than becoming arbitrary
 * identifiers in the universal grammar.
 * ============================================================================
 */

hdlPortMode
    : K_CONST
    | K_MUT
    | K_REF
    ;


/*
 * ============================================================================
 * 6. PORT NAME
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
 * The type system is owned by grammar/types/.
 *
 * The exact canonical type-entry rule is supplied by the parser composition
 * grammar.
 *
 * This delegate therefore uses the canonical `typeExpr` contract rather than
 * defining another type grammar.
 *
 * If the canonical parser uses a different exported rule name, the adapter
 * in grammar/hdl/ composition must map it once at the composition boundary.
 * ============================================================================
 */

hdlPortType
    : COLON
      typeExpr
    ;


/*
 * ============================================================================
 * 8. PORT DIMENSIONS
 * ============================================================================
 *
 * Dimensions are source-level shape/indexing information.
 *
 * The expressions inside dimensions use the canonical expression system.
 *
 * No maximum dimension count exists.
 * ============================================================================
 */

hdlPortDimension
    : LBRACKET
      expression?
      RBRACKET
    ;


/*
 * ============================================================================
 * 9. PORT DEFAULT
 * ============================================================================
 *
 * Default values are source-level expressions.
 *
 * Whether a default is legal for a particular direction/type is a semantic
 * question.
 * ============================================================================
 */

hdlPortDefaultValue
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * 10. PORT ATTRIBUTES
 * ============================================================================
 *
 * Attribute syntax is shared with the canonical attribute system.
 *
 * If the parser composition layer exports an `attribute` rule, it should be
 * consumed directly.
 *
 * This local rule is retained as an explicit adapter so this file has a stable
 * port-facing contract.
 * ============================================================================
 */

hdlPortAttributes
    : attribute+
    ;


/*
 * ============================================================================
 * 11. PORT ATTRIBUTE CONSTRAINT
 * ============================================================================
 *
 * Port-level constraints are represented as attributes rather than a second
 * constraint language.
 *
 * Examples:
 *
 *     @width(...)
 *     @protocol(...)
 *     @timing(...)
 *
 * Their semantics are determined downstream.
 *
 * Physical-placement attributes are NOT made universal by this rule.
 * ============================================================================
 */

hdlPortAttributeConstraint
    : attribute
    ;


/*
 * ============================================================================
 * 12. LOGICAL PORT GROUP
 * ============================================================================
 *
 * Groups provide reusable logical organization.
 *
 * Cardinality is unbounded by the grammar.
 *
 * A group is not a physical bank or hardware I/O block.
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
    : identifier
    ;


hdlPortGroupDirection
    : hdlPortDirection
    ;


hdlPortGroupMember
    : hdlPortDeclarationItem
    | hdlPortGroup
    ;


/*
 * ============================================================================
 * 13. NAMED PORT ASSOCIATION
 * ============================================================================
 *
 * Logical source-level association:
 *
 *     input_data = data
 *
 * It does not select physical routing.
 * ============================================================================
 */

hdlNamedPortConnection
    : hdlPortName
      ASSIGN
      expression
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
 * 14. POSITIONAL PORT ASSOCIATION
 * ============================================================================
 *
 * Positional association is retained as source syntax.
 *
 * Semantic analysis determines whether positional association is legal for
 * the containing interface/module and whether ordering is compatible.
 *
 * ============================================================================
 */

hdlPositionalPortConnections
    : LPAREN
      hdlPositionalPortConnectionList?
      RPAREN
    ;


hdlPositionalPortConnectionList
    : expression
      (
          COMMA
          expression
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 15. MIXED PORT ASSOCIATION
 * ============================================================================
 *
 * The grammar captures syntax only.
 *
 * Semantic analysis must determine whether mixing named and positional
 * associations is permitted and how ordering is interpreted.
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
    | expression
    ;


/*
 * ============================================================================
 * 16. PORT MAP
 * ============================================================================
 *
 * This is a logical source-level map.
 *
 * The containing module/instance grammar owns the surrounding declaration or
 * instantiation context.
 * ============================================================================
 */

hdlPortMap
    : LBRACE
      hdlNamedPortConnectionList?
      RBRACE
    ;


/*
 * ============================================================================
 * 17. PORT REFERENCE
 * ============================================================================
 *
 * A port reference uses the canonical name/path system.
 *
 * Physical identifiers are intentionally excluded.
 * ============================================================================
 */

hdlPortReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 18. PORT ALIAS
 * ============================================================================
 *
 * This rule represents a logical source-level alias.
 *
 * It does not create physical aliasing.
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
 * 19. PORT DECLARATION SEQUENCE
 * ============================================================================
 *
 * Used by containing HDL/interface grammar where the surrounding construct
 * owns declaration boundaries.
 * ============================================================================
 */

hdlPortDeclarationSequence
    : hdlPortDeclarationItem*
    ;


/*
 * ============================================================================
 * 20. PORT CONTRACT
 * ============================================================================
 *
 * A named logical contract can be referenced by an interface/module grammar.
 *
 * The actual declaration/ownership of interfaces remains outside this file.
 *
 * ============================================================================
 */

hdlPortContractReference
    : hdlPortReference
    ;


/*
 * ============================================================================
 * 21. SEMANTIC BOUNDARY
 * ============================================================================
 *
 * The following are intentionally NOT grammar decisions:
 *
 *     physical pin mapping
 *     package mapping
 *     board mapping
 *     FPGA bank selection
 *     ASIC pad selection
 *     device selection
 *     voltage assignment
 *     electrical standards
 *     routing
 *     placement
 *     timing closure
 *     synthesis
 *     resource feasibility
 *     hardware discovery
 *
 * They belong to downstream systems.
 *
 * ============================================================================
 * 22. INTEGRATION MATRIX
 * ============================================================================
 *
 * OWNER                         CONSUMER
 * ---------------------------------------------------------------------------
 *
 * grammar/lexer/               lexical tokens
 * grammar/core/                identifiers / names / attributes
 * grammar/expressions/         expressions
 * grammar/types/               type expressions
 * grammar/hdl/ports.g4         logical port syntax
 * grammar/hdl/                 HDL composition
 * grammar/hardware/            hardware intent / target semantics
 * grammar/resources/           resource requirements/capabilities
 * grammar/compile/             compilation intent
 * grammar/execution/           execution intent
 * src/frontend/ast/            domain-neutral AST
 * semantic analysis            port/type/direction legality
 * hardware IR                  logical hardware representation
 * synthesis                    implementation realization
 * routing                      physical connectivity
 * placement                    physical location
 * HAL                          target capability/device boundary
 * runtime                      execution/deployment
 *
 * ============================================================================
 * 23. AST MAPPING CONTRACT
 * ============================================================================
 *
 * Rule                          Expected semantic node
 * ---------------------------------------------------------------------------
 *
 * hdlPortDeclaration            HdlPortDeclaration
 * hdlPortDirection              HdlPortDirection
 * hdlPortMode                   HdlPortMode
 * hdlPortName                   Identifier
 * hdlPortType                   TypeExpression
 * hdlPortDimension              DimensionExpression
 * hdlPortDefaultValue           Expression
 * hdlPortAttributes             AttributeList
 * hdlPortGroup                  HdlPortGroup
 * hdlPortReference              HdlPortReference
 * hdlNamedPortConnection        HdlPortAssociation
 * hdlPortMap                    HdlPortMap
 *
 * The actual AST type names are owned by src/frontend/ast/.
 *
 * If the implementation uses different concrete names, the semantic mapping
 * must preserve the same concepts without requiring this grammar to change.
 *
 * ============================================================================
 * 24. QUANTUM / HYBRID INTEGRATION
 * ============================================================================
 *
 * A port may carry a type whose semantic domain is quantum, classical, or
 * hybrid if the canonical type and semantic systems permit it.
 *
 * This file MUST NOT define:
 *
 *     quantum operations
 *     gates
 *     qubit allocation
 *     measurement
 *     quantum routing
 *     QEC
 *     ZQN
 *
 * Quantum meaning ultimately reaches:
 *
 *     semantic analysis
 *          |
 *          v
 *     quantum::ir
 *
 * The port grammar therefore remains independent of physical QPU topology.
 *
 * ============================================================================
 * 25. SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar uses:
 *
 *     *
 *     +
 *     optional constructs
 *     symbolic expressions
 *     generic types
 *     canonical names
 *
 * rather than fixed cardinalities.
 *
 * Therefore source syntax does not impose limits on:
 *
 *     number of ports
 *     number of groups
 *     number of dimensions
 *     width values
 *     number of connections
 *     number of modules
 *     number of interfaces
 *
 * Actual limits are implementation/resource constraints, not grammar limits.
 *
 * ============================================================================
 * 26. HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden language-level concepts in this file include:
 *
 *     MAX_PORTS
 *     MAX_WIDTH
 *     MAX_LANES
 *     MAX_DIMENSIONS
 *     MAX_CONNECTIONS
 *     MAX_INTERFACES
 *     MAX_PINS
 *
 * Also forbidden:
 *
 *     fixed physical pin names
 *     fixed FPGA banks
 *     fixed ASIC pads
 *     fixed board identifiers
 *     fixed device identifiers
 *     fixed bus counts
 *
 * Numeric literals appearing inside user expressions remain program data and
 * are not interpreted as language-wide limits.
 *
 * ============================================================================
 * 27. DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Syntax errors:
 *
 *     malformed direction
 *     missing port name
 *     malformed type attachment
 *     malformed dimension
 *     malformed default expression
 *     malformed association
 *
 * Semantic errors:
 *
 *     duplicate port name
 *     incompatible direction
 *     invalid type
 *     incompatible connected type
 *     incompatible dimensions
 *     invalid default
 *
 * Resource/target errors:
 *
 *     target lacks required capability
 *     target lacks sufficient resources
 *     target cannot realize requested interface
 *
 * The parser MUST NOT report downstream resource feasibility as a syntax error.
 *
 * ============================================================================
 * 28. DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing this grammar depends only on:
 *
 *     source text
 *     selected grammar/version
 *     canonical lexical vocabulary
 *     canonical parser composition
 *
 * It MUST NOT depend on:
 *
 *     hardware availability
 *     target discovery
 *     current time
 *     randomness
 *     filesystem state
 *     network state
 *     runtime state
 *
 * ============================================================================
 * 29. TEST CONTRACT
 * ============================================================================
 *
 * Required positive tests:
 *
 *     input data: logic;
 *     output result: logic;
 *     inout bus: logic[WIDTH];
 *     input data: Vector<T, WIDTH>;
 *     input matrix: Tensor<T, Shape>;
 *     input data: logic[WIDTH][LANES];
 *     input a: logic, b: logic, c: logic;
 *     input {
 *         valid: bool;
 *         ready: bool;
 *     }
 *
 * Required symbolic/scalability tests:
 *
 *     input data: logic[WIDTH];
 *     input data: logic[2 * LANES];
 *     input data: logic[problem_size];
 *     input data: Tensor<T, Shape>;
 *
 * Required association tests:
 *
 *     port_map { input_data = data; }
 *
 *     port_map {
 *         input_data = data,
 *         output_data = result
 *     }
 *
 * Required negative tests:
 *
 *     input;
 *     output;
 *     inout;
 *     input 123;
 *     input data:;
 *     input data: logic[;
 *     input data: logic] ;
 *
 * Required ambiguity regression tests:
 *
 *     input data: logic;
 *     output output_data: logic;
 *     input input_data: logic;
 *     inout inout_data: logic;
 *
 * These tests specifically ensure that direction keywords cannot be
 * accidentally consumed as arbitrary identifiers/modes and that port names
 * remain recoverable.
 *
 * Required cross-domain tests:
 *
 *     HDL + classical type
 *     HDL + quantum type
 *     HDL + hybrid type
 *     HDL + hardware capability
 *     HDL + resource requirement
 *     HDL + distributed endpoint
 *
 * Required hard-coding test:
 *
 *     Search this file and generated grammar sources for forbidden universal
 *     capacity constants and physical identifiers.
 *
 * ============================================================================
 * 30. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] Port syntax has one owner.
 *     [x] Direction has one unambiguous representation.
 *     [x] Arbitrary IDENTIFIER direction fallbacks are absent.
 *     [x] Arbitrary IDENTIFIER modes are absent.
 *     [x] General expressions are delegated to canonical expression syntax.
 *     [x] General types are delegated to canonical type syntax.
 *     [x] Identifiers are delegated to canonical name syntax.
 *     [x] Port dimensions are target-independent.
 *     [x] Port groups are target-independent.
 *     [x] Port associations are logical.
 *     [x] Physical placement is excluded.
 *     [x] Hardware discovery is excluded.
 *     [x] No universal resource limits exist.
 *     [x] Quantum semantics remain outside this file.
 *     [x] quantum::ir remains the canonical quantum boundary.
 *     [x] Resource/capability analysis remains downstream.
 *     [x] AST mappings are documented.
 *     [x] Semantic responsibilities are documented.
 *     [x] Compiler responsibilities are documented.
 *     [x] Runtime responsibilities are documented.
 *     [x] Positive tests are specified.
 *     [x] Negative tests are specified.
 *     [x] Scalability tests are specified.
 *     [x] Ambiguity regression tests are specified.
 *     [x] Cross-domain tests are specified.
 *     [x] Rust integration requires no unsafe code.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 *     PORT = LOGICAL INTERFACE CONTRACT
 *
 *     PORT != PHYSICAL PIN
 *     PORT != DEVICE
 *     PORT != RESOURCE
 *     PORT != ROUTE
 *     PORT != PLACEMENT
 *     PORT != SCHEDULING DECISION
 *     PORT != TARGET SELECTION
 *
 * A Zamani port therefore remains portable from:
 *
 *     tiny hardware
 *     embedded systems
 *     FPGA
 *     ASIC
 *     CPU
 *     GPU
 *     accelerator
 *     distributed hardware
 *     heterogeneous systems
 *     future computing targets
 *
 * subject only to the resources and capabilities available to the downstream
 * realization.
 *
 * ============================================================================
 */
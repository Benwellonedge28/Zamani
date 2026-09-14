/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hdl/hardware-parameters.g4
 *
 * Purpose:
 *     Canonical parser grammar for HDL/hardware parameter declarations,
 *     parameter domains, parameter constraints, parameter relationships,
 *     and parameter bindings used by Zamani hardware descriptions.
 *
 * Architectural position:
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          +--> HardwareParameters
 *          |
 *          v
 *     syntax AST
 *          |
 *          v
 *     semantic analysis / elaboration
 *          |
 *          +--> HDL semantic model
 *          +--> classical IR where applicable
 *          +--> hardware/resource model
 *          +--> quantum::ir where quantum semantics are actually present
 *          |
 *          v
 *     optimization / scheduling / routing / lowering
 *          |
 *          v
 *     target realization
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This is an ANTLR parser grammar.
 *     It contains no embedded Rust.
 *     No unsafe Rust is required by this file.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - HDL parameter declarations
 *     - HDL parameter kinds
 *     - HDL parameter type annotations
 *     - HDL parameter defaults
 *     - HDL parameter bounds
 *     - HDL parameter constraints
 *     - HDL parameter relationships
 *     - HDL parameter domains
 *     - HDL parameter aliases
 *     - HDL parameter references
 *     - HDL parameter binding syntax
 *     - HDL parameter override syntax
 *     - HDL parameter specialization syntax
 *     - parameter-level semantic metadata
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens
 *     - identifiers
 *     - generic expression syntax
 *     - general type syntax
 *     - general declarations
 *     - module declarations
 *     - ports
 *     - signals
 *     - wires
 *     - registers
 *     - clocks
 *     - timing
 *     - combinational logic
 *     - sequential logic
 *     - processes
 *     - state machines
 *     - memories
 *     - pipelines
 *     - hardware interfaces
 *     - physical devices
 *     - device discovery
 *     - placement
 *     - routing
 *     - scheduling
 *     - calibration
 *     - synthesis
 *     - simulation
 *     - optimization
 *     - vendor selection
 *     - backend selection
 *     - runtime execution
 *     - canonical IR construction
 *
 * ============================================================================
 * POCO-REAF PRINCIPLE
 * ============================================================================
 *
 * A parameter is a source-level abstraction.
 *
 * A parameter may describe:
 *
 *     - width
 *     - depth
 *     - count
 *     - latency
 *     - precision
 *     - structural choice
 *     - algorithmic configuration
 *     - interface configuration
 *     - implementation preference
 *     - logical resource requirement
 *
 * It MUST NOT implicitly mean:
 *
 *     - a fixed physical machine size
 *     - a fixed number of physical devices
 *     - a fixed board
 *     - a fixed FPGA
 *     - a fixed ASIC
 *     - a fixed CPU
 *     - a fixed GPU
 *     - a fixed quantum processor
 *     - a fixed topology
 *     - a fixed physical address
 *     - a fixed vendor
 *
 * Parameter values are therefore source-level values.
 *
 * Whether a parameterized design is realizable is determined later by:
 *
 *     semantic analysis
 *     elaboration
 *     capability analysis
 *     resource analysis
 *     compilation
 *     scheduling
 *     hardware abstraction
 *     target lowering
 *     runtime
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar intentionally contains no:
 *
 *     MAX_PARAMETERS
 *     MAX_GENERICS
 *     MAX_WIDTH
 *     MAX_DEPTH
 *     MAX_INSTANCES
 *     MAX_LANES
 *     MAX_PORTS
 *     MAX_DEVICES
 *     MAX_CORES
 *     MAX_MEMORY
 *     MAX_CLOCKS
 *     MAX_FREQUENCY
 *     MAX_QUANTUM_RESOURCES
 *
 * Repetition is represented through grammar operators rather than finite
 * enumerations.
 *
 * Actual numeric limits, if any, are implementation/resource concerns and
 * MUST NOT be encoded as arbitrary parser limits.
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * This is a PARSER grammar.
 *
 * The canonical lexer supplies the tokens.
 *
 * Canonical shared grammar supplies:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     typeExpr
 *     attribute
 *
 * This grammar must therefore NOT redefine those concepts.
 *
 * ============================================================================
 */

parser grammar HardwareParameters;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. PUBLIC PARAMETER DECLARATION
 * ============================================================================
 *
 * This is the primary rule consumed by hardware module/generic grammar.
 *
 * Examples:
 *
 *     parameter WIDTH: integer = 64;
 *
 *     parameter DEPTH: integer;
 *
 *     parameter LANES: integer
 *         where LANES > 0;
 *
 *     parameter DATA_WIDTH: integer = WIDTH;
 *
 *     parameter ENABLE_PARITY: boolean = true;
 */
hdlParameterDeclaration
    : PARAMETER hdlParameterName
      hdlParameterType?
      hdlParameterDefault?
      hdlParameterDomain?
      hdlParameterConstraints?
      SEMICOLON
    ;


/*
 * ============================================================================
 * 2. PARAMETER NAME
 * ============================================================================
 *
 * Parameter identity is logical source identity.
 *
 * It is not:
 *
 *     a physical address
 *     a device identifier
 *     a register number
 *     a vendor identifier
 */
hdlParameterName
    : identifier
    ;


/*
 * ============================================================================
 * 3. PARAMETER TYPE
 * ============================================================================
 *
 * The parameter grammar consumes the canonical type expression.
 *
 * It does not create a second hardware-specific type system.
 */
hdlParameterType
    : COLON typeExpr
    ;


/*
 * ============================================================================
 * 4. DEFAULT VALUE
 * ============================================================================
 *
 * A default is an expression evaluated during the appropriate semantic or
 * elaboration phase.
 *
 * This grammar does not decide whether evaluation occurs at:
 *
 *     parse time
 *     compile time
 *     elaboration time
 *     runtime
 *
 * That decision belongs to semantic/compiler infrastructure.
 */
hdlParameterDefault
    : ASSIGN expression
    ;


/*
 * ============================================================================
 * 5. PARAMETER DOMAIN
 * ============================================================================
 *
 * A domain describes the legal value space of a parameter.
 *
 * Examples:
 *
 *     parameter WIDTH: integer
 *         in 1..WIDTH_LIMIT;
 *
 *     parameter MODE: Mode
 *         in {fast, balanced, low_power};
 *
 * The grammar only describes the syntax.
 * Semantic validation determines whether the domain is meaningful.
 */
hdlParameterDomain
    : hdlParameterRangeDomain
    | hdlParameterSetDomain
    | hdlParameterPredicateDomain
    ;


/*
 * ============================================================================
 * 6. RANGE DOMAIN
 * ============================================================================
 */
hdlParameterRangeDomain
    : IN hdlRangeExpression
    ;


hdlRangeExpression
    : expression RANGE expression
    ;


/*
 * ============================================================================
 * 7. SET DOMAIN
 * ============================================================================
 *
 * The set is represented by expressions rather than by a fixed enumeration.
 */
hdlParameterSetDomain
    : IN LBRACE hdlParameterSetElementList? RBRACE
    ;


hdlParameterSetElementList
    : expression
      (COMMA expression)*
      COMMA?
    ;


/*
 * ============================================================================
 * 8. PREDICATE DOMAIN
 * ============================================================================
 *
 * Allows a parameter domain to be expressed through a semantic predicate.
 *
 * Example:
 *
 *     parameter WIDTH: integer
 *         where WIDTH % 8 == 0;
 *
 * The actual predicate semantics are checked outside the grammar.
 */
hdlParameterPredicateDomain
    : WHERE expression
    ;


/*
 * ============================================================================
 * 9. PARAMETER CONSTRAINTS
 * ============================================================================
 *
 * Constraints are distinct from defaults.
 *
 * Default:
 *
 *     what value is used when no explicit value is supplied.
 *
 * Constraint:
 *
 *     what values are legally permitted.
 */
hdlParameterConstraints
    : WHERE hdlParameterConstraintList
    ;


hdlParameterConstraintList
    : hdlParameterConstraint
      (COMMA hdlParameterConstraint)*
      COMMA?
    ;


hdlParameterConstraint
    : expression
    ;


/*
 * ============================================================================
 * 10. PARAMETER BLOCK
 * ============================================================================
 *
 * A parameter block allows a module or hardware declaration to group
 * parameter declarations.
 *
 * Example:
 *
 *     parameters {
 *         parameter WIDTH: integer = 64;
 *         parameter LANES: integer = 4;
 *     }
 */
hdlParameterBlock
    : PARAMETERS LBRACE hdlParameterDeclaration* RBRACE
    ;


/*
 * ============================================================================
 * 11. PARAMETER REFERENCE
 * ============================================================================
 *
 * Parameter references use ordinary expressions/identifiers.
 *
 * This rule exists as a semantic integration point rather than introducing
 * a separate parameter identifier type.
 */
hdlParameterReference
    : identifier
    ;


/*
 * ============================================================================
 * 12. PARAMETER ALIAS
 * ============================================================================
 *
 * An alias gives another source-level name to an existing parameter.
 *
 * Example:
 *
 *     parameter alias WORD_SIZE = DATA_WIDTH;
 *
 * The alias does not create a second physical resource.
 */
hdlParameterAliasDeclaration
    : PARAMETER ALIAS hdlParameterName ASSIGN hdlParameterReference SEMICOLON
    ;


/*
 * ============================================================================
 * 13. PARAMETER BINDING
 * ============================================================================
 *
 * Binding connects a caller/module instance value to a parameter.
 *
 * Example:
 *
 *     WIDTH = 128
 *
 * Binding is source-level specialization.
 * It is not hardware placement.
 */
hdlParameterBinding
    : hdlParameterName ASSIGN expression
    ;


/*
 * ============================================================================
 * 14. PARAMETER BINDING LIST
 * ============================================================================
 */
hdlParameterBindingList
    : hdlParameterBinding
      (COMMA hdlParameterBinding)*
      COMMA?
    ;


/*
 * ============================================================================
 * 15. PARAMETER OVERRIDE
 * ============================================================================
 *
 * Override syntax is deliberately explicit.
 *
 * Example:
 *
 *     with {
 *         WIDTH = 128,
 *         LANES = 8
 *     }
 *
 * The semantic layer decides whether overriding the selected parameter is
 * legal.
 */
hdlParameterOverride
    : WITH LBRACE hdlParameterBindingList? RBRACE
    ;


/*
 * ============================================================================
 * 16. PARAMETER SPECIALIZATION
 * ============================================================================
 *
 * A specialization supplies parameter values to a parameterized declaration.
 *
 * Example:
 *
 *     <WIDTH = 128, LANES = 8>
 *
 * No finite number of parameters is assumed.
 */
hdlParameterSpecialization
    : LT hdlParameterBindingList? GT
    ;


/*
 * ============================================================================
 * 17. PARAMETER VALUE
 * ============================================================================
 *
 * A parameter value is intentionally represented by the canonical expression
 * grammar.
 *
 * This permits:
 *
 *     constants
 *     arithmetic
 *     references
 *     compile-time expressions
 *     symbolic expressions
 *     generic expressions
 *
 * without duplicating expression grammar here.
 */
hdlParameterValue
    : expression
    ;


/*
 * ============================================================================
 * 18. PARAMETER RELATION
 * ============================================================================
 *
 * A parameter relation expresses a relationship between parameters.
 *
 * Examples:
 *
 *     WIDTH = DATA_WIDTH;
 *
 *     TOTAL = ROWS * COLS;
 *
 *     LANES <= WIDTH;
 *
 * This remains syntax-level information.
 */
hdlParameterRelation
    : expression
    ;


/*
 * ============================================================================
 * 19. PARAMETER RELATION BLOCK
 * ============================================================================
 *
 * Example:
 *
 *     relations {
 *         TOTAL = ROWS * COLS;
 *         LANES <= WIDTH;
 *     }
 */
hdlParameterRelationBlock
    : RELATIONS LBRACE hdlParameterRelationEntry* RBRACE
    ;


hdlParameterRelationEntry
    : hdlParameterRelation SEMICOLON
    ;


/*
 * ============================================================================
 * 20. PARAMETER REQUIREMENT
 * ============================================================================
 *
 * Parameter requirements express semantic requirements without binding the
 * design to a physical target.
 *
 * Example:
 *
 *     requires {
 *         WIDTH > 0;
 *         LANES <= WIDTH;
 *     }
 */
hdlParameterRequirementBlock
    : REQUIRES LBRACE hdlParameterRequirementEntry* RBRACE
    ;


hdlParameterRequirementEntry
    : expression SEMICOLON
    ;


/*
 * ============================================================================
 * 21. PARAMETER GUARANTEE
 * ============================================================================
 *
 * Guarantees describe properties that successful elaboration must preserve.
 */
hdlParameterGuaranteeBlock
    : ENSURES LBRACE hdlParameterGuaranteeEntry* RBRACE
    ;


hdlParameterGuaranteeEntry
    : expression SEMICOLON
    ;


/*
 * ============================================================================
 * 22. PARAMETER ATTRIBUTES
 * ============================================================================
 *
 * Parameter attributes remain metadata.
 *
 * They must not silently turn into target-selection mechanisms.
 */
hdlParameterAttribute
    : AT identifier
      (
          LPAREN hdlParameterAttributeArguments? RPAREN
      )?
    ;


hdlParameterAttributeArguments
    : hdlParameterAttributeArgument
      (COMMA hdlParameterAttributeArgument)*
      COMMA?
    ;


hdlParameterAttributeArgument
    : identifier ASSIGN expression
    | expression
    ;


/*
 * ============================================================================
 * 23. COMPLETE PARAMETER DECLARATION WITH ATTRIBUTES
 * ============================================================================
 *
 * This rule is provided as the preferred integration entry point for HDL
 * declaration grammars that need metadata around parameters.
 */
hdlAnnotatedParameterDeclaration
    : hdlParameterAttribute*
      hdlParameterDeclaration
    ;


/*
 * ============================================================================
 * 24. PARAMETER GROUP
 * ============================================================================
 *
 * A parameter group permits multiple declarations and relationships to be
 * represented as one syntactic unit.
 */
hdlParameterGroup
    : PARAMETER_GROUP hdlParameterGroupName?
      LBRACE
          hdlParameterGroupMember*
      RBRACE
    ;


hdlParameterGroupName
    : identifier
    ;


hdlParameterGroupMember
    : hdlParameterAttribute*
      (
          hdlParameterDeclaration
        | hdlParameterAliasDeclaration
        | hdlParameterRelationBlock
        | hdlParameterRequirementBlock
        | hdlParameterGuaranteeBlock
      )
    ;


/*
 * ============================================================================
 * 25. PARAMETER EXPRESSION
 * ============================================================================
 *
 * Named integration rule for downstream grammars.
 *
 * It intentionally delegates to the canonical expression grammar.
 */
hdlParameterExpression
    : expression
    ;


/*
 * ============================================================================
 * 26. PARAMETER TYPE/DOMAIN CONTRACT
 * ============================================================================
 *
 * This rule provides a single syntactic unit for tools that need to inspect
 * the parameter's declaration contract.
 *
 * It does not perform semantic validation.
 */
hdlParameterContract
    : hdlParameterType?
      hdlParameterDefault?
      hdlParameterDomain?
      hdlParameterConstraints?
    ;


/*
 * ============================================================================
 * 27. COMPLETE PARAMETER ITEM
 * ============================================================================
 *
 * Common integration point for module/generic grammar.
 */
hdlParameterItem
    : hdlAnnotatedParameterDeclaration
    | hdlParameterAliasDeclaration
    | hdlParameterGroup
    ;


/*
 * ============================================================================
 * 28. PARAMETER LIST
 * ============================================================================
 *
 * Used by grammar components that need a list without introducing another
 * parameter declaration construct.
 */
hdlParameterList
    : hdlParameterDeclaration
      (hdlParameterDeclaration)*
    ;


/*
 * ============================================================================
 * 29. PARAMETER MAP
 * ============================================================================
 *
 * Explicit named parameter bindings.
 *
 * Example:
 *
 *     parameters {
 *         WIDTH = 128,
 *         LANES = 8
 *     }
 *
 * This rule is syntactic only.
 */
hdlParameterMap
    : PARAMETERS LBRACE hdlParameterBindingList? RBRACE
    ;


/*
 * ============================================================================
 * 30. ELABORATION CONTRACT
 * ============================================================================
 *
 * This rule provides an explicit syntax boundary for constructs whose values
 * are resolved during elaboration.
 *
 * Example:
 *
 *     elaborates {
 *         WIDTH = BASE_WIDTH * 2;
 *     }
 *
 * The actual elaboration engine is outside this grammar.
 */
hdlParameterElaborationBlock
    : ELABORATES LBRACE hdlParameterElaborationEntry* RBRACE
    ;


hdlParameterElaborationEntry
    : hdlParameterBinding SEMICOLON
    ;


/*
 * ============================================================================
 * 31. PARAMETER REQUIREMENT/CONSTRAINT EXPRESSION
 * ============================================================================
 *
 * Shared semantic expression boundary.
 */
hdlParameterConstraintExpression
    : expression
    ;


/*
 * ============================================================================
 * 32. RESOURCE PARAMETER
 * ============================================================================
 *
 * Resource parameters remain logical.
 *
 * Examples:
 *
 *     parameter lanes: integer;
 *     parameter memory_capacity: size;
 *     parameter queue_depth: integer;
 *
 * No physical resource is allocated by this grammar.
 */
hdlResourceParameterDeclaration
    : PARAMETER hdlParameterName
      COLON typeExpr
      hdlParameterDefault?
      hdlParameterConstraints?
      SEMICOLON
    ;


/*
 * ============================================================================
 * 33. HARDWARE CAPABILITY PARAMETER
 * ============================================================================
 *
 * Capability values are source-level requirements/configuration values.
 *
 * They do not perform capability discovery.
 */
hdlCapabilityParameterDeclaration
    : PARAMETER hdlParameterName
      COLON typeExpr
      hdlParameterDefault?
      hdlParameterConstraints?
      SEMICOLON
    ;


/*
 * ============================================================================
 * 34. TARGET PARAMETER
 * ============================================================================
 *
 * A target parameter can describe an abstract target property.
 *
 * It must not encode a mandatory physical device.
 *
 * Example:
 *
 *     parameter target_class: TargetClass;
 *
 * Semantic target selection belongs to compilation/deployment.
 */
hdlTargetParameterDeclaration
    : PARAMETER hdlParameterName
      COLON typeExpr
      hdlParameterDefault?
      hdlParameterConstraints?
      SEMICOLON
    ;


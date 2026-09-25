/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hdl/parameters.g4
 *
 * Status:
 *     CANONICAL HDL PARAMETER GRAMMAR
 *
 * Replaces:
 *     grammar/hdl/hardware-parameters.g4
 *
 * Purpose:
 *     Defines target-independent, parameterized HDL source syntax.
 *
 * Architectural position:
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     ZamaniParser
 *          |
 *          v
 *     HDLParameters
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis / elaboration
 *          |
 *          +--> type analysis
 *          +--> constant/symbolic evaluation
 *          +--> resource analysis
 *          +--> capability analysis
 *          +--> HDL semantic model
 *          +--> hardware semantic model
 *          |
 *          v
 *     canonical semantic IR / HDL IR
 *          |
 *          +--> optimization
 *          +--> synthesis
 *          +--> scheduling
 *          +--> routing
 *          +--> placement
 *          +--> target lowering
 *          |
 *          v
 *     actual CPU / GPU / FPGA / ASIC / accelerator / QPU / future target
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     Safe Rust
 *     No unsafe Rust
 *
 * IMPORTANT:
 *
 *     This file contains grammar only.
 *
 *     It contains:
 *         - no Rust actions;
 *         - no semantic predicates implemented in Rust;
 *         - no hardware discovery;
 *         - no filesystem access;
 *         - no network access;
 *         - no target selection;
 *         - no physical resource allocation.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - HDL parameter declarations
 *     - HDL local parameter declarations
 *     - parameter type clauses
 *     - parameter default expressions
 *     - parameter constraints
 *     - parameter domains
 *     - parameter sets
 *     - parameter relationships
 *     - parameter aliases
 *     - parameter blocks
 *     - parameter maps
 *     - parameter bindings
 *     - parameter overrides
 *     - parameter specialization
 *     - parameter elaboration blocks
 *     - parameter requirements
 *     - parameter guarantees
 *     - parameter attributes
 *     - parameter groups
 *     - parameter expressions
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer rules
 *     - identifiers
 *     - general expressions
 *     - general types
 *     - modules
 *     - ports
 *     - signals
 *     - nets
 *     - registers
 *     - clocks
 *     - timing
 *     - memories
 *     - pipelines
 *     - generate constructs
 *     - generic parameter declarations
 *     - resource declarations
 *     - capability declarations
 *     - physical devices
 *     - physical addresses
 *     - placement
 *     - routing
 *     - scheduling
 *     - calibration
 *     - synthesis implementation
 *     - runtime execution
 *     - vendor selection
 *     - target selection
 *     - canonical IR construction
 *
 * ============================================================================
 * PARAMETER VS GENERIC
 * ============================================================================
 *
 * HDL PARAMETERS:
 *
 *     Source-level configuration values belonging to an HDL declaration,
 *     module, interface, package, or parameterized hardware construct.
 *
 * HDL GENERICS:
 *
 *     Compile-time structural/type/value abstraction owned by
 *     hardware-generics.g4.
 *
 * They must not become duplicate syntaxes.
 *
 * Parameter syntax is owned here.
 *
 * Generic syntax is owned by:
 *
 *     grammar/hdl/hardware-generics.g4
 *
 * ============================================================================
 * PARAMETER VS RESOURCE REQUIREMENT
 * ============================================================================
 *
 * A parameter:
 *
 *     WIDTH
 *
 * is a source-level symbolic value.
 *
 * A resource requirement:
 *
 *     requires memory >= required_memory
 *
 * is a semantic requirement on realization.
 *
 * A parameter MUST NOT silently become a physical resource allocation.
 *
 * For example:
 *
 *     parameter LANES: uint = 8;
 *
 * does NOT mean:
 *
 *     use physical lane 0 through 7
 *
 * unless a downstream semantic/lowering phase explicitly derives that
 * realization.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Parameters exist to make hardware descriptions reusable across targets.
 *
 * The grammar MUST NOT encode universal physical limits.
 *
 * Forbidden language-level limits include:
 *
 *     MAX_PARAMETERS
 *     MAX_WIDTH
 *     MAX_DEPTH
 *     MAX_LANES
 *     MAX_PORTS
 *     MAX_INSTANCES
 *     MAX_DEVICES
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_MEMORY
 *     MAX_REGISTER_WIDTH
 *     MAX_TENSOR_RANK
 *     MAX_NETWORK_SIZE
 *
 * A numeric value in source code remains program semantics.
 *
 * For example:
 *
 *     parameter WIDTH: uint = 1024;
 *
 * is valid source-level information.
 *
 * The grammar MUST NOT interpret 1024 as a compiler-wide maximum.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * These are portable:
 *
 *     parameter WIDTH: uint = DATA_WIDTH;
 *     parameter LANES: uint = requested_lanes;
 *     parameter DEPTH: uint = Rows * Columns;
 *
 * These are not owned by this grammar:
 *
 *     physical_gpu(0)
 *     physical_qubit(17)
 *     fpga_bank(3)
 *     cpu_core(7)
 *
 * Physical realization belongs downstream.
 *
 * ============================================================================
 * ANTLR CONTRACT
 * ============================================================================
 *
 * This is a PARSER grammar.
 *
 * Canonical lexical authority:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This file MUST NOT define lexer rules.
 *
 * Canonical shared parser concepts:
 *
 *     identifier
 *     expression
 *     typeExpr
 *     attribute
 *
 * remain owned by the corresponding canonical parser composition layers.
 *
 * The HDL composition layer supplies:
 *
 *     hdlExpression
 *     hdlTypeExpression
 *
 * where those names are part of the existing HDL grammar contract.
 *
 * ============================================================================
 */

parser grammar HDLParameters;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. PARAMETER DECLARATION
 * ============================================================================
 *
 * Canonical HDL parameter declaration.
 *
 * Examples:
 *
 *     parameter WIDTH: uint;
 *
 *     parameter WIDTH: uint = 64;
 *
 *     parameter WIDTH: uint = DATA_WIDTH;
 *
 *     parameter LANES: uint = WIDTH / 8;
 *
 * The grammar does not evaluate the expressions.
 */

hdlParameterDeclaration
    : K_PARAMETER
      identifier
      hdlParameterTypeClause?
      hdlParameterDefaultClause?
      hdlParameterConstraintClause*
      SEMICOLON
    ;


/*
 * ============================================================================
 * 2. LOCAL PARAMETER
 * ============================================================================
 *
 * A local parameter is source-local configuration.
 *
 * It does not represent a physical resource.
 */

hdlLocalParameterDeclaration
    : K_LOCALPARAM
      identifier
      hdlParameterTypeClause?
      ASSIGN
      hdlExpression
      hdlParameterConstraintClause*
      SEMICOLON
    ;


/*
 * ============================================================================
 * 3. PARAMETER TYPE
 * ============================================================================
 *
 * HDL type syntax remains owned by the canonical HDL type grammar.
 *
 * This file does not create a second type system.
 */

hdlParameterTypeClause
    : COLON hdlTypeExpression
    ;


/*
 * ============================================================================
 * 4. PARAMETER DEFAULT
 * ============================================================================
 *
 * Defaults are expressions.
 *
 * They may be:
 *
 *     constants
 *     parameter references
 *     generic references
 *     arithmetic expressions
 *     symbolic expressions
 *     type-level values
 *     domain expressions
 *
 * Evaluation belongs downstream.
 */

hdlParameterDefaultClause
    : ASSIGN hdlExpression
    ;


/*
 * ============================================================================
 * 5. PARAMETER CONSTRAINT
 * ============================================================================
 *
 * Constraints are syntactic expressions.
 *
 * Semantic analysis determines:
 *
 *     - type validity;
 *     - satisfiability;
 *     - dependency ordering;
 *     - constant/symbolic evaluation;
 *     - resource implications.
 */

hdlParameterConstraintClause
    : K_REQUIRES hdlExpression
    | K_WHERE hdlExpression
    ;


/*
 * ============================================================================
 * 6. PARAMETER DOMAIN
 * ============================================================================
 *
 * A domain constrains the legal source-level value space.
 *
 * It does not perform hardware discovery.
 */

hdlParameterDomain
    : hdlParameterRangeDomain
    | hdlParameterSetDomain
    | hdlParameterPredicateDomain
    ;


/*
 * ============================================================================
 * 7. RANGE DOMAIN
 * ============================================================================
 *
 * Examples:
 *
 *     in 1..WIDTH
 *     in MIN..MAX
 *     in lower..upper
 *
 * Range endpoints remain expressions.
 */

hdlParameterRangeDomain
    : K_IN hdlParameterRangeExpression
    ;

hdlParameterRangeExpression
    : hdlExpression RANGE hdlExpression
    ;


/*
 * ============================================================================
 * 8. SET DOMAIN
 * ============================================================================
 *
 * Example:
 *
 *     in {fast, balanced, low_power}
 *
 * There is no fixed enumeration encoded here.
 *
 * Trailing commas are intentionally rejected to remain consistent with the
 * canonical array-expression syntax in expressions/arrays.g4.
 */

hdlParameterSetDomain
    : K_IN
      LBRACE
      hdlParameterSetElementList?
      RBRACE
    ;

hdlParameterSetElementList
    : hdlExpression
      (COMMA hdlExpression)*
    ;


/*
 * ============================================================================
 * 9. PREDICATE DOMAIN
 * ============================================================================
 *
 * Example:
 *
 *     where WIDTH % 8 == 0
 *
 * Semantic validation belongs outside the parser.
 */

hdlParameterPredicateDomain
    : K_WHERE hdlExpression
    ;


/*
 * ============================================================================
 * 10. PARAMETER BLOCK
 * ============================================================================
 *
 * A parameter block groups source-level parameter declarations.
 *
 * No finite parameter count is encoded.
 */

hdlParameterBlock
    : K_PARAMETERS
      LBRACE
      hdlParameterBlockItem*
      RBRACE
    ;

hdlParameterBlockItem
    : hdlParameterAttribute*
      (
          hdlParameterDeclaration
        | hdlLocalParameterDeclaration
        | hdlParameterAliasDeclaration
        | hdlParameterGroup
      )
    ;


/*
 * ============================================================================
 * 11. PARAMETER REFERENCE
 * ============================================================================
 *
 * Parameter references intentionally reuse ordinary HDL identifiers.
 *
 * This avoids creating a second identifier category.
 */

hdlParameterReference
    : identifier
    ;


/*
 * ============================================================================
 * 12. PARAMETER ALIAS
 * ============================================================================
 *
 * Example:
 *
 *     parameter alias WORD_SIZE = DATA_WIDTH;
 *
 * The semantic layer verifies that the referenced parameter exists and that
 * aliasing is legal.
 */

hdlParameterAliasDeclaration
    : K_PARAMETER
      K_ALIAS
      identifier
      ASSIGN
      hdlParameterReference
      SEMICOLON
    ;


/*
 * ============================================================================
 * 13. PARAMETER BINDING
 * ============================================================================
 *
 * A binding associates a parameter name with a source-level expression.
 *
 * It does NOT select a physical resource.
 */

hdlParameterBinding
    : identifier
      ASSIGN
      hdlExpression
    ;


/*
 * ============================================================================
 * 14. PARAMETER BINDING LIST
 * ============================================================================
 *
 * Arbitrarily many bindings are represented structurally.
 *
 * No fixed number of parameters is encoded.
 *
 * Trailing commas are accepted here because binding/argument lists in the
 * existing HDL grammar already use that compatibility convention.
 */

hdlParameterBindingList
    : hdlParameterBinding
      (COMMA hdlParameterBinding)*
      COMMA?
    ;


/*
 * ============================================================================
 * 15. PARAMETER MAP
 * ============================================================================
 *
 * Explicit named parameter values.
 *
 * Example:
 *
 *     parameters {
 *         WIDTH = 128,
 *         LANES = 8
 *     }
 */

hdlParameterMap
    : K_PARAMETERS
      LBRACE
      hdlParameterBindingList?
      RBRACE
    ;


/*
 * ============================================================================
 * 16. PARAMETER OVERRIDE
 * ============================================================================
 *
 * Example:
 *
 *     with {
 *         WIDTH = 128,
 *         LANES = 8
 *     }
 *
 * The semantic layer determines whether each override is permitted.
 */

hdlParameterOverride
    : K_WITH
      LBRACE
      hdlParameterBindingList?
      RBRACE
    ;


/*
 * ============================================================================
 * 17. PARAMETER SPECIALIZATION
 * ============================================================================
 *
 * Named parameter specialization.
 *
 * Example:
 *
 *     <WIDTH = 128, LANES = 8>
 *
 * Generic specialization remains separately owned by hardware-generics.g4.
 */

hdlParameterSpecialization
    : LESS_THAN
      hdlParameterBindingList?
      GREATER_THAN
    ;


/*
 * ============================================================================
 * 18. PARAMETER VALUE
 * ============================================================================
 *
 * Stable integration boundary for consumers that need a parameter value.
 */

hdlParameterValue
    : hdlExpression
    ;


/*
 * ============================================================================
 * 19. PARAMETER EXPRESSION
 * ============================================================================
 *
 * Stable semantic-analysis boundary.
 */

hdlParameterExpression
    : hdlExpression
    ;


/*
 * ============================================================================
 * 20. PARAMETER RELATION
 * ============================================================================
 *
 * Relationships express source-level relationships among parameters.
 *
 * Examples:
 *
 *     TOTAL = ROWS * COLS;
 *     LANES <= WIDTH;
 *     ADDRESS_BITS = ceil_log2(DEPTH);
 *
 * The grammar accepts the expression; semantic analysis interprets it.
 */

hdlParameterRelation
    : hdlExpression
    ;

hdlParameterRelationBlock
    : K_RELATIONS
      LBRACE
      hdlParameterRelationEntry*
      RBRACE
    ;

hdlParameterRelationEntry
    : hdlParameterRelation
      SEMICOLON
    ;


/*
 * ============================================================================
 * 21. PARAMETER REQUIREMENTS
 * ============================================================================
 *
 * These are parameter/value constraints.
 *
 * They must not be confused with target resource requirements.
 *
 * For example:
 *
 *     requires WIDTH > 0;
 *
 * is a parameter constraint.
 *
 * A requirement such as:
 *
 *     requires capability("gpu.compute")
 *
 * belongs to the resource/capability grammar.
 */

hdlParameterRequirementBlock
    : K_REQUIRES
      LBRACE
      hdlParameterRequirementEntry*
      RBRACE
    ;

hdlParameterRequirementEntry
    : hdlExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 22. PARAMETER GUARANTEES
 * ============================================================================
 *
 * Guarantees are source-level properties that downstream elaboration must
 * preserve.
 */

hdlParameterGuaranteeBlock
    : K_ENSURES
      LBRACE
      hdlParameterGuaranteeEntry*
      RBRACE
    ;

hdlParameterGuaranteeEntry
    : hdlExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 23. PARAMETER ATTRIBUTES
 * ============================================================================
 *
 * Metadata only.
 *
 * Attributes must not silently select vendors, devices, boards, or physical
 * resources.
 */

hdlParameterAttribute
    : AT
      identifier
      (
          LPAREN
          hdlParameterAttributeArgumentList?
          RPAREN
      )?
    ;

hdlParameterAttributeArgumentList
    : hdlParameterAttributeArgument
      (COMMA hdlParameterAttributeArgument)*
      COMMA?
    ;

hdlParameterAttributeArgument
    : identifier
      ASSIGN
      hdlExpression
    | hdlExpression
    ;


/*
 * ============================================================================
 * 24. ANNOTATED PARAMETER
 * ============================================================================
 */

hdlAnnotatedParameterDeclaration
    : hdlParameterAttribute*
      (
          hdlParameterDeclaration
        | hdlLocalParameterDeclaration
      )
    ;


/*
 * ============================================================================
 * 25. PARAMETER GROUP
 * ============================================================================
 *
 * Groups allow related parameters and semantic contracts to remain together.
 */

hdlParameterGroup
    : K_PARAMETER_GROUP
      identifier?
      LBRACE
      hdlParameterGroupMember*
      RBRACE
    ;

hdlParameterGroupMember
    : hdlParameterAttribute*
      (
          hdlParameterDeclaration
        | hdlLocalParameterDeclaration
        | hdlParameterAliasDeclaration
        | hdlParameterRelationBlock
        | hdlParameterRequirementBlock
        | hdlParameterGuaranteeBlock
      )
    ;


/*
 * ============================================================================
 * 26. PARAMETER ELABORATION
 * ============================================================================
 *
 * Elaboration expressions are source-level semantic inputs.
 *
 * This grammar does not execute them.
 */

hdlParameterElaborationBlock
    : K_ELABORATES
      LBRACE
      hdlParameterElaborationEntry*
      RBRACE
    ;

hdlParameterElaborationEntry
    : hdlParameterBinding
      SEMICOLON
    ;


/*
 * ============================================================================
 * 27. PARAMETER CONTRACT
 * ============================================================================
 *
 * Stable inspection boundary for tooling and semantic analysis.
 */

hdlParameterContract
    : hdlParameterTypeClause?
      hdlParameterDefaultClause?
      hdlParameterConstraintClause*
    ;


/*
 * ============================================================================
 * 28. PARAMETER ITEM
 * ============================================================================
 *
 * General integration point for HDL module/interface/package consumers.
 */

hdlParameterItem
    : hdlAnnotatedParameterDeclaration
    | hdlParameterAliasDeclaration
    | hdlParameterGroup
    | hdlParameterRelationBlock
    | hdlParameterRequirementBlock
    | hdlParameterGuaranteeBlock
    ;


/*
 * ============================================================================
 * 29. PARAMETER LIST
 * ============================================================================
 *
 * Declaration list with no artificial cardinality limit.
 */

hdlParameterList
    : hdlParameterItem*
    ;


/*
 * ============================================================================
 * 30. RESOURCE PARAMETER
 * ============================================================================
 *
 * These remain logical source parameters.
 *
 * Resource requirements themselves are NOT owned here.
 */

hdlResourceParameterDeclaration
    : K_PARAMETER
      identifier
      COLON
      hdlTypeExpression
      hdlParameterDefaultClause?
      hdlParameterConstraintClause*
      SEMICOLON
    ;


/*
 * ============================================================================
 * 31. CAPABILITY PARAMETER
 * ============================================================================
 *
 * Capability values are source-level values.
 *
 * Capability discovery remains downstream.
 */

hdlCapabilityParameterDeclaration
    : K_PARAMETER
      identifier
      COLON
      hdlTypeExpression
      hdlParameterDefaultClause?
      hdlParameterConstraintClause*
      SEMICOLON
    ;


/*
 * ============================================================================
 * 32. TARGET-ABSTRACT PARAMETER
 * ============================================================================
 *
 * A target parameter may describe an abstract target property.
 *
 * It does not identify a physical device.
 */

hdlTargetParameterDeclaration
    : K_PARAMETER
      identifier
      COLON
      hdlTypeExpression
      hdlParameterDefaultClause?
      hdlParameterConstraintClause*
      SEMICOLON
    ;


/*
 * ============================================================================
 * 33. PARAMETERIZED HDL DECLARATION CONTRACT
 * ============================================================================
 *
 * Consumers can use this rule as a stable boundary without knowing how the
 * parameter declaration is represented internally.
 */

hdlParameterizedDeclarationParameters
    : hdlParameterItem*
    ;


/*
 * ============================================================================
 * 34. SAFETY / SCALABILITY INVARIANTS
 * ============================================================================
 *
 * No grammar-level hardware capacities are encoded.
 *
 * The following concepts remain source-level expressions:
 *
 *     width
 *     depth
 *     count
 *     lanes
 *     ports
 *     memory size
 *     vector width
 *     tensor dimensions
 *     pipeline depth
 *     latency
 *     precision
 *     quantum resource quantities
 *
 * Any physical limit belongs downstream to:
 *
 *     resource analysis
 *     capability analysis
 *     target discovery
 *     compilation
 *     scheduling
 *     routing
 *     placement
 *     synthesis
 *     deployment
 *
 * ============================================================================
 * END
 * ============================================================================
 */
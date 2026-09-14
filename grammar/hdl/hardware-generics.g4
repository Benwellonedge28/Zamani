/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hdl/hardware-generics.g4
 *
 * Purpose:
 *     Canonical parser grammar for hardware/HDL generic declarations,
 *     generic constraints, generic defaults, generic specialization, and
 *     generic-aware hardware composition.
 *
 * Architectural role:
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     ZamaniParser / HDL parser
 *       |
 *       v
 *     HardwareGenerics
 *       |
 *       v
 *     frontend AST
 *       |
 *       +--> generic/type semantic analysis
 *       +--> constant/compile-time evaluation
 *       +--> capability/resource analysis
 *       +--> hardware/HDL semantic model
 *       |
 *       v
 *     canonical IR
 *       |
 *       +--> optimization
 *       +--> scheduling
 *       +--> placement/routing
 *       +--> synthesis/lowering
 *       +--> target realization
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This is an ANTLR parser grammar.
 *     It contains no embedded Rust.
 *     It uses no Rust `unsafe`.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - HDL generic parameter-list syntax;
 *   - HDL generic parameter declarations;
 *   - hardware type generic parameters;
 *   - hardware value generic parameters;
 *   - hardware constant generic parameters;
 *   - generic parameter defaults;
 *   - generic parameter constraints attached syntactically to a generic;
 *   - generic parameter groups;
 *   - generic specialization syntax when that specialization is explicitly
 *     represented as an HDL generic specialization;
 *   - generic argument lists;
 *   - named generic arguments;
 *   - positional generic arguments;
 *   - generic argument ordering;
 *   - optional trailing commas;
 *   - syntactic generic metadata/attributes where supported by the shared
 *     attribute grammar.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexical tokens;
 *   - identifiers;
 *   - expressions;
 *   - general types;
 *   - general-purpose language generics;
 *   - function generics;
 *   - module declarations;
 *   - hardware parameters;
 *   - ports;
 *   - signals;
 *   - wires;
 *   - registers;
 *   - clocks;
 *   - timing;
 *   - memories;
 *   - pipelines;
 *   - interfaces;
 *   - hardware targets;
 *   - devices;
 *   - boards;
 *   - FPGA resources;
 *   - ASIC cells;
 *   - CPU cores;
 *   - GPU counts;
 *   - QPU topology;
 *   - resource discovery;
 *   - scheduling;
 *   - routing;
 *   - placement;
 *   - optimization;
 *   - synthesis;
 *   - runtime execution;
 *   - canonical IR;
 *   - semantic evaluation;
 *   - resource availability.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Hardware generics describe PARAMETRIC HARDWARE SEMANTICS.
 *
 * They do not describe a fixed physical machine.
 *
 * Therefore this grammar MUST NOT encode:
 *
 *     MAX_WIDTH
 *     MAX_LANES
 *     MAX_PORTS
 *     MAX_PARAMETERS
 *     MAX_GENERICS
 *     MAX_INSTANCES
 *     MAX_CORES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_QUANTUM_RESOURCES
 *     MAX_FREQUENCY
 *     MAX_PIPELINE_DEPTH
 *     MAX_FPGA_RESOURCES
 *     MAX_ASIC_CELLS
 *
 * Repetition is represented structurally using `*`, `+`, and optional rules.
 *
 * Any eventual physical limit belongs to:
 *
 *     semantic analysis
 *     capability analysis
 *     resource analysis
 *     compilation
 *     scheduling
 *     hardware abstraction
 *     target lowering
 *     runtime
 *
 * A generic such as:
 *
 *     WIDTH
 *
 * means that WIDTH participates in the module's semantic parameterization.
 *
 * It does NOT mean that a target has a particular physical width.
 *
 * ============================================================================
 * GENERIC VS PARAMETER
 * ============================================================================
 *
 * Generic:
 *
 *     A compile-time structural/type/value abstraction that participates in
 *     specialization and elaboration.
 *
 * Parameter:
 *
 *     A separately owned module-level configuration/value mechanism.
 *
 * This grammar owns the former.
 *
 * `hardware-parameters.g4`, when introduced, owns the latter.
 *
 * The two grammars MUST NOT duplicate one another.
 *
 * ============================================================================
 * GENERIC VS RESOURCE REQUIREMENT
 * ============================================================================
 *
 * A generic:
 *
 *     WIDTH
 *
 * is a symbolic program-level parameter.
 *
 * A resource requirement:
 *
 *     requires ...
 *
 * is a statement about what successful realization requires.
 *
 * A generic MUST NOT silently become a hardware requirement.
 *
 * For example:
 *
 *     <LANES>
 *
 * does not mean:
 *
 *     requires LANES physical execution lanes
 *
 * unless semantic analysis explicitly derives that requirement from the
 * hardware model.
 *
 * ============================================================================
 * GENERIC VS TARGET
 * ============================================================================
 *
 * Generic syntax must never encode:
 *
 *     a vendor
 *     a board
 *     a device identifier
 *     a physical address
 *     a topology
 *     a fixed accelerator
 *     a fixed QPU
 *     a fixed CPU
 *
 * Such information belongs to target/resource/deployment layers.
 *
 * ============================================================================
 * ANTLR CONTRACT
 * ============================================================================
 *
 * This is a PARSER grammar.
 *
 * It must consume the canonical Zamani lexer vocabulary:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * It MUST NOT redefine lexical tokens.
 *
 * Shared rules such as:
 *
 *     identifier
 *     expression
 *     typeExpr
 *     qualifiedName
 *     attribute
 *
 * are supplied by the canonical parser composition layer.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * Expected consumers include:
 *
 *     grammar/hdl/hardware-modules.g4
 *     grammar/hdl/hdl.g4
 *     grammar/hdl/hardware-interfaces.g4
 *     grammar/types/generic-types.g4
 *     grammar/functions/generics.g4
 *     frontend AST generic nodes
 *     semantic generic/type analysis
 *     HDL elaboration
 *     hardware capability analysis
 *     resource analysis
 *     hardware/HDL IR lowering
 *
 * This grammar must remain independent of target realization.
 *
 * ============================================================================
 */

parser grammar HardwareGenerics;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * `hardwareGenericParameters` is the canonical HDL generic declaration list.
 *
 * Typical use:
 *
 *     <WIDTH>
 *
 *     <WIDTH: uint = 32>
 *
 *     <WIDTH: uint where WIDTH > 0>
 *
 *     <T: SomeHardwareType>
 *
 *     <LANES: uint = 1, WIDTH: uint = 32>
 *
 * No finite number of generic parameters is imposed.
 * ============================================================================
 */

hardwareGenericParameters
    : LT hardwareGenericParameterList? GT
    ;


/*
 * ============================================================================
 * 2. GENERIC PARAMETER LIST
 * ============================================================================
 */

hardwareGenericParameterList
    : hardwareGenericParameter
      (COMMA hardwareGenericParameter)*
      COMMA?
    ;


/*
 * ============================================================================
 * 3. GENERIC PARAMETER
 * ============================================================================
 *
 * A parameter consists of:
 *
 *     optional attributes
 *     parameter name
 *     optional type
 *     optional constraint
 *     optional default
 *
 * The semantic layer determines whether the declaration is valid.
 *
 * Examples:
 *
 *     WIDTH
 *
 *     WIDTH: uint
 *
 *     WIDTH: uint = 32
 *
 *     WIDTH: uint where WIDTH > 0
 *
 *     WIDTH: uint where WIDTH > 0 = 32
 *
 * The parser accepts the structure; semantic analysis validates ordering,
 * typing, satisfiability and dependency rules.
 * ============================================================================
 */

hardwareGenericParameter
    : hardwareGenericAttributes?
      identifier
      hardwareGenericTypeClause?
      hardwareGenericConstraintClause?
      hardwareGenericDefaultClause?
    ;


/*
 * ============================================================================
 * 4. GENERIC TYPE CLAUSE
 * ============================================================================
 *
 * The type expression is supplied by the canonical type grammar.
 *
 * This file does not create a second hardware type system.
 * ============================================================================
 */

hardwareGenericTypeClause
    : COLON typeExpr
    ;


/*
 * ============================================================================
 * 5. GENERIC CONSTRAINT CLAUSE
 * ============================================================================
 *
 * Constraints restrict legal generic substitutions.
 *
 * They are semantic constraints, not physical machine limits.
 *
 * Example:
 *
 *     WIDTH: uint where WIDTH > 0
 *
 *     LANES: uint where LANES is power_of_two
 *
 * The grammar intentionally treats the constraint body as an expression.
 *
 * The semantic layer decides:
 *
 *     - whether the expression is valid;
 *     - whether it is evaluable;
 *     - whether it is satisfiable;
 *     - whether it is compile-time evaluable;
 *     - whether it refers only to permitted generic symbols.
 * ============================================================================
 */

hardwareGenericConstraintClause
    : WHERE hardwareGenericConstraintExpression
    ;


hardwareGenericConstraintExpression
    : expression
    ;


/*
 * ============================================================================
 * 6. GENERIC DEFAULT
 * ============================================================================
 *
 * Defaults are source-level specialization defaults.
 *
 * They do not establish a universal hardware default.
 *
 * Example:
 *
 *     WIDTH: uint = 32
 *
 * means only:
 *
 *     if WIDTH is omitted during specialization, the semantic elaborator may
 *     use the declared default 32.
 *
 * It does NOT mean:
 *
 *     all Zamani hardware has width 32.
 * ============================================================================
 */

hardwareGenericDefaultClause
    : ASSIGN expression
    ;


/*
 * ============================================================================
 * 7. GENERIC ATTRIBUTES
 * ============================================================================
 *
 * Attributes remain metadata.
 *
 * This rule intentionally does not enumerate vendor attributes.
 *
 * Future attributes can therefore be introduced through the canonical
 * attribute system without modifying this grammar for every target.
 * ============================================================================
 */

hardwareGenericAttributes
    : attribute+
    ;


/*
 * ============================================================================
 * 8. HARDWARE GENERIC ARGUMENTS
 * ============================================================================
 *
 * Used when specializing a generic hardware declaration.
 *
 * Examples:
 *
 *     <32>
 *
 *     <WIDTH = 32>
 *
 *     <WIDTH = 32, LANES = 8>
 *
 *     <32, 8>
 *
 * The semantic layer determines whether positional and named arguments are
 * compatible with the declaration.
 *
 * No fixed argument count is imposed here.
 * ============================================================================
 */

hardwareGenericArguments
    : LT hardwareGenericArgumentList? GT
    ;


hardwareGenericArgumentList
    : hardwareGenericArgument
      (COMMA hardwareGenericArgument)*
      COMMA?
    ;


/*
 * ============================================================================
 * 9. GENERIC ARGUMENT
 * ============================================================================
 *
 * Named:
 *
 *     WIDTH = 32
 *
 * Positional:
 *
 *     32
 *
 * The grammar deliberately does not require every specialization to use one
 * style exclusively.
 *
 * Semantic analysis is responsible for rejecting invalid mixtures or ordering
 * according to the language specification.
 * ============================================================================
 */

hardwareGenericArgument
    : identifier ASSIGN expression
    | expression
    ;


/*
 * ============================================================================
 * 10. GENERIC BINDINGS
 * ============================================================================
 *
 * A binding is useful when a generic specialization is represented separately
 * from the angle-bracket argument list.
 *
 * Example:
 *
 *     bind WIDTH = 32;
 *
 * This rule is intentionally NOT exposed as a general statement.
 *
 * It exists only as a reusable parser component for HDL grammar owners that
 * explicitly choose to expose generic binding syntax.
 * ============================================================================
 */

hardwareGenericBinding
    : identifier ASSIGN expression
    ;


/*
 * ============================================================================
 * 11. GENERIC BINDING LIST
 * ============================================================================
 */

hardwareGenericBindingList
    : hardwareGenericBinding
      (COMMA hardwareGenericBinding)*
      COMMA?
    ;


/*
 * ============================================================================
 * 12. GENERIC REFERENCE
 * ============================================================================
 *
 * A generic reference is represented through the universal identifier grammar.
 *
 * This rule gives HDL grammar composition a named semantic hook without
 * creating a second identifier representation.
 * ============================================================================
 */

hardwareGenericReference
    : identifier
    ;


/*
 * ============================================================================
 * 13. GENERIC TYPE ARGUMENT
 * ============================================================================
 *
 * Explicit type-level specialization is kept separate from ordinary value
 * expressions.
 *
 * The canonical type grammar remains authoritative.
 * ============================================================================
 */

hardwareGenericTypeArgument
    : typeExpr
    ;


/*
 * ============================================================================
 * 14. GENERIC VALUE ARGUMENT
 * ============================================================================
 *
 * Hardware value expressions use the canonical expression grammar.
 *
 * No machine-width restriction is imposed.
 * ============================================================================
 */

hardwareGenericValueArgument
    : expression
    ;


/*
 * ============================================================================
 * 15. GENERIC SPECIALIZATION
 * ============================================================================
 *
 * A complete reusable specialization construct.
 *
 * Examples:
 *
 *     GenericModule<WIDTH = 32>
 *
 *     GenericModule<32>
 *
 *     GenericModule<WIDTH = 32, LANES = 8>
 *
 * This rule owns only the generic specialization syntax.
 *
 * Module/reference ownership remains with the module grammar.
 * ============================================================================
 */

hardwareGenericSpecialization
    : hardwareGenericArguments
    ;


/*
 * ============================================================================
 * 16. NAMED GENERIC SPECIALIZATION
 * ============================================================================
 *
 * Convenience composition rule for consumers that require named-only
 * specialization.
 *
 * Example:
 *
 *     <WIDTH = 32, LANES = 8>
 *
 * This is intentionally a parser-level distinction.
 *
 * Semantic analysis remains responsible for:
 *
 *     duplicate names
 *     unknown names
 *     missing required generics
 *     type mismatches
 *     invalid dependency order
 *     unsatisfied constraints
 * ============================================================================
 */

hardwareNamedGenericArguments
    : LT hardwareNamedGenericArgumentList? GT
    ;


hardwareNamedGenericArgumentList
    : hardwareNamedGenericArgument
      (COMMA hardwareNamedGenericArgument)*
      COMMA?
    ;


hardwareNamedGenericArgument
    : identifier ASSIGN expression
    ;


/*
 * ============================================================================
 * 17. POSITIONAL GENERIC SPECIALIZATION
 * ============================================================================
 *
 * Convenience composition rule for consumers that require positional-only
 * specialization.
 *
 * Semantic validation still belongs outside the parser.
 * ============================================================================
 */

hardwarePositionalGenericArguments
    : LT hardwarePositionalGenericArgumentList? GT
    ;


hardwarePositionalGenericArgumentList
    : hardwarePositionalGenericArgument
      (COMMA hardwarePositionalGenericArgument)*
      COMMA?
    ;


hardwarePositionalGenericArgument
    : expression
    ;


/*
 * ============================================================================
 * 18. GENERIC DECLARATION GROUP
 * ============================================================================
 *
 * Some HDL constructs may want a named generic group without creating a new
 * generic semantic model.
 *
 * Example:
 *
 *     generics <WIDTH: uint, LANES: uint>;
 *
 * This rule is a composition primitive only.
 *
 * Whether the `GENERICS` keyword is part of the canonical Zamani language is
 * determined by the lexer/specification.
 *
 * Consequently this production deliberately does not introduce a new lexer
 * token here.
 * ============================================================================
 */

/*
 * NOTE:
 *
 * A keyword such as `generics` MUST NOT be represented here as an identifier
 * workaround.
 *
 * If named generic-group syntax becomes part of the language, the canonical
 * lexer must first own the keyword and the canonical parser must establish its
 * compatibility policy.
 *
 * Therefore no speculative `GENERICS` token is referenced in this file.
 */


/*
 * ============================================================================
 * 19. GENERIC WHERE-CLAUSE
 * ============================================================================
 *
 * Reusable form for HDL constructs that expose a generic constraint block.
 *
 * Example:
 *
 *     where WIDTH > 0, LANES > 0
 *
 * This rule remains expression-based.
 * ============================================================================
 */

hardwareGenericWhereClause
    : WHERE hardwareGenericWhereEntryList
    ;


hardwareGenericWhereEntryList
    : hardwareGenericWhereEntry
      (COMMA hardwareGenericWhereEntry)*
      COMMA?
    ;


hardwareGenericWhereEntry
    : expression
    ;


/*
 * ============================================================================
 * 20. GENERIC DECLARATION REFERENCE
 * ============================================================================
 *
 * Used by semantic-facing parser compositions that need to identify a generic
 * declaration without duplicating its declaration structure.
 * ============================================================================
 */

hardwareGenericDeclarationReference
    : identifier
    ;


/*
 * ============================================================================
 * 21. GENERIC VALUE DECLARATION
 * ============================================================================
 *
 * Explicit value generic.
 *
 * Example:
 *
 *     WIDTH: uint
 *
 * This is intentionally equivalent in semantic structure to the general
 * generic declaration; the distinction is supplied by the type expression.
 *
 * No special hardware integer width is imposed.
 * ============================================================================
 */

hardwareValueGenericParameter
    : hardwareGenericAttributes?
      identifier
      hardwareGenericTypeClause
      hardwareGenericConstraintClause?
      hardwareGenericDefaultClause?
    ;


/*
 * ============================================================================
 * 22. GENERIC TYPE DECLARATION
 * ============================================================================
 *
 * Explicit type generic.
 *
 * Example:
 *
 *     T: SomeHardwareType
 *
 * Whether a type expression denotes a hardware type is a semantic question.
 * ============================================================================
 */

hardwareTypeGenericParameter
    : hardwareGenericAttributes?
      identifier
      COLON typeExpr
    ;


/*
 * ============================================================================
 * 23. GENERIC CONSTANT DECLARATION
 * ============================================================================
 *
 * Explicit compile-time constant generic.
 *
 * Example:
 *
 *     DEPTH: uint = 1024
 *
 * This grammar does not impose a maximum value.
 *
 * The semantic evaluator and target/resource layers determine realizability.
 * ============================================================================
 */

hardwareConstantGenericParameter
    : hardwareGenericAttributes?
      identifier
      hardwareGenericTypeClause?
      hardwareGenericConstraintClause?
      hardwareGenericDefaultClause?
    ;


/*
 * ============================================================================
 * 24. GENERIC PARAMETER CATEGORY
 * ============================================================================
 *
 * Reusable classification hook.
 *
 * All forms ultimately map into the canonical generic AST model.
 *
 * This rule MUST NOT result in separate semantic representations merely
 * because syntax gives them convenient names.
 * ============================================================================
 */

hardwareGenericParameterKind
    : hardwareTypeGenericParameter
    | hardwareValueGenericParameter
    | hardwareConstantGenericParameter
    ;


/*
 * ============================================================================
 * 25. GENERIC DEPENDENCY REFERENCE
 * ============================================================================
 *
 * Generic defaults and constraints may reference previously declared symbols.
 *
 * The parser merely recognizes the expression.
 *
 * Dependency ordering and legality belong to semantic analysis.
 *
 * No artificial dependency-depth limit is imposed.
 * ============================================================================
 */

hardwareGenericDependencyReference
    : identifier
    ;


/*
 * ============================================================================
 * 26. GENERIC CONTRACT
 * ============================================================================
 *
 * A generic contract is represented by an expression.
 *
 * This gives HDL generic systems a future-compatible extension point without
 * hard-coding target-specific concepts into the grammar.
 * ============================================================================
 */

hardwareGenericContract
    : expression
    ;


/*
 * ============================================================================
 * 27. GENERIC CONTRACT LIST
 * ============================================================================
 */

hardwareGenericContractList
    : hardwareGenericContract
      (COMMA hardwareGenericContract)*
      COMMA?
    ;


/*
 * ============================================================================
 * 28. GENERIC CONTRACT CLAUSE
 * ============================================================================
 *
 * Generic contracts are semantic constraints, not resource discovery.
 * ============================================================================
 */

hardwareGenericContractClause
    : WHERE hardwareGenericContractList
    ;


/*
 * ============================================================================
 * 29. GENERIC SPECIALIZATION REFERENCE
 * ============================================================================
 *
 * This rule intentionally separates the logical name from its specialization.
 *
 * Example:
 *
 *     VectorUnit<WIDTH = 64>
 *
 * The module/reference grammar owns `VectorUnit`.
 * This file owns `<WIDTH = 64>`.
 * ============================================================================
 */

hardwareGenericSpecializationReference
    : hardwareGenericSpecialization
    ;


/*
 * ============================================================================
 * 30. GENERIC ELABORATION INPUT
 * ============================================================================
 *
 * Parser-level aggregate consumed by elaboration-facing HDL rules.
 *
 * It does not execute elaboration.
 * ============================================================================
 */

hardwareGenericElaborationInput
    : hardwareGenericParameters?
      hardwareGenericSpecialization?
    ;
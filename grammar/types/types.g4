/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/types.g4
 *
 * Grammar:
 *     Types
 *
 * Status:
 *     CANONICAL PRODUCTION TYPE-SYSTEM ORCHESTRATOR
 *
 * Compiler baseline:
 *     Rust 1.97+
 *     Rust 2021
 *     safe Rust only
 *     no unsafe
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar is the SINGLE PUBLIC ORCHESTRATION BOUNDARY for Zamani
 * source-level types.
 *
 * It owns type composition.
 *
 * It does NOT own the implementation details of individual type families.
 *
 * Specialized grammars under grammar/types/ own their respective
 * constructors and semantic syntax.
 *
 * The architecture is:
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     Types.typeExpression
 *          |
 *          +-----------------------------+
 *          |                             |
 *          v                             v
 *     specialized type delegates    type modifiers/postfixes
 *          |                             |
 *          +-------------+---------------+
 *                        |
 *                        v
 *                  TypeExpr AST
 *                        |
 *                        v
 *               structural validation
 *                        |
 *             +----------+----------+
 *             |          |          |
 *             v          v          v
 *           types      effects   constraints
 *             |          |          |
 *             +----------+----------+
 *                        |
 *                        v
 *                 semantic type model
 *                        |
 *        +---------------+----------------+
 *        |               |                |
 *        v               v                v
 *    classical       quantum::ir      HDL/hardware
 *        |               |                |
 *        +---------------+----------------+
 *                        |
 *                        v
 *                 canonical semantic IR
 *                        |
 *                        v
 *            optimization / specialization
 *                        |
 *                        v
 *                 lowering / routing
 *                        |
 *                        v
 *                    scheduling
 *                        |
 *                        v
 *                 resilience / QEC
 *                        |
 *                        v
 *                    ZQN / HAL
 *
 * ============================================================================
 * SINGLE-OWNER RULE
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     typeExpression
 *     typeCore
 *     typePrefix
 *     typePostfix
 *     parenthesizedType
 *     typeValueExpression
 *     typeExtension
 *
 * THIS FILE DOES NOT OWN:
 *
 *     primitive type definitions
 *     named type definitions
 *     generic argument implementation
 *     tuple implementation
 *     array implementation
 *     slice implementation
 *     map implementation
 *     option implementation
 *     result implementation
 *     record implementation
 *     sum implementation
 *     union implementation
 *     function implementation
 *     reference implementation
 *     pointer implementation
 *     linear implementation
 *     affine implementation
 *     dependent implementation
 *     associated-type implementation
 *     type-class implementation
 *     existential-type implementation
 *     classical-domain implementation
 *     quantum-domain implementation
 *     hardware-domain implementation
 *     resource-type implementation
 *     capability-type implementation
 *     temporal-type implementation
 *     effect-qualified type implementation
 *     advanced type-level computation implementation
 *
 * It also does not own:
 *
 *     name resolution
 *     type inference
 *     unification
 *     substitution
 *     trait/type-class resolution
 *     ownership analysis
 *     lifetime analysis
 *     resource discovery
 *     capability discovery
 *     effect checking
 *     policy checking
 *     target selection
 *     hardware discovery
 *     quantum routing
 *     quantum scheduling
 *     QEC
 *     ZQN
 *     HAL
 *     runtime layout
 *     ABI layout
 *
 * ============================================================================
 * AUTHORITATIVE PUBLIC ENTRY POINT
 * ============================================================================
 *
 * All source-level type consumers must enter through:
 *
 *     typeExpression
 *
 * Other grammars may import Types and consume typeExpression.
 *
 * No other grammar under grammar/types/ may define a competing universal
 * type-expression entry point.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY CONTRACT
 * ============================================================================
 *
 * A type describes program meaning.
 *
 * A type MUST NOT encode an implementation-specific physical capacity.
 *
 * This grammar therefore contains NO universal limits for:
 *
 *     CPUs
 *     cores
 *     threads
 *     GPUs
 *     FPGAs
 *     ASIC resources
 *     accelerators
 *     QPUs
 *     qubits
 *     nodes
 *     devices
 *     memory
 *     registers
 *     register width
 *     tensor rank
 *     tensor dimensions
 *     network size
 *     topology size
 *     program size
 *
 * In particular, this file MUST NOT introduce:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_REGISTER_WIDTH
 *     MAX_TENSOR_RANK
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * Symbolic quantities remain symbolic.
 *
 * Examples:
 *
 *     N
 *     M
 *     Rows
 *     Columns
 *     RequiredMemory
 *     RequiredWidth
 *     Batch
 *     Dimension
 *
 * Physical feasibility is determined after parsing through:
 *
 *     resource requirements
 *     capability negotiation
 *     target discovery
 *     specialization
 *     lowering
 *     scheduling
 *     runtime realization
 *
 * ============================================================================
 * EXTENSIBILITY CONTRACT
 * ============================================================================
 *
 * New domains MUST NOT require a new universal type branch merely because
 * the domain has new user-defined types.
 *
 * The preferred extension mechanisms are:
 *
 *     named types
 *     qualified names
 *     generic applications
 *     value parameters
 *     associated types
 *     type classes
 *     dialects
 *     domain registrations
 *     semantic capabilities
 *
 * Therefore types such as:
 *
 *     Tensor<T>
 *     Tensor<T>[N, M]
 *     Model<T>
 *     Dataset<T>
 *     Distribution<T>
 *     Evidence<T>
 *     QuantumState<T>
 *     LogicalQubit
 *     QRegister<N>
 *     Signal<T>
 *     Accelerator<T>
 *     Resource<T>
 *     Capability<T>
 *     DistributedState<T>
 *
 * remain expressible without creating a finite universal inventory.
 *
 * ============================================================================
 * DOMAIN INTEGRATION
 * ============================================================================
 *
 * The universal type grammar must be capable of carrying types used by:
 *
 *     classical computing
 *     numerical computing
 *     tensor computing
 *     AI/ML
 *     symbolic reasoning
 *     knowledge systems
 *     probabilistic systems
 *     quantum computing
 *     hybrid quantum-classical computing
 *     HDL
 *     hardware description
 *     accelerators
 *     distributed computing
 *     networking
 *     embedded systems
 *     HPC
 *     future computational domains
 *
 * Domain semantics are resolved downstream.
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * Quantum types are source-level semantic types only.
 *
 * This grammar MUST NOT:
 *
 *     allocate physical qubits
 *     choose QPUs
 *     select coupling maps
 *     perform routing
 *     schedule operations
 *     select calibration
 *     choose pulse implementations
 *     select QEC codes
 *     construct ZQN
 *     bind physical device identifiers
 *
 * The semantic path is:
 *
 *     quantum source type
 *          |
 *          v
 *     TypeExpr
 *          |
 *          v
 *     semantic quantum type
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     decomposition
 *          |
 *          v
 *     routing
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     resilience / QEC
 *          |
 *          v
 *     ZQN
 *          |
 *          v
 *     HAL
 *
 * ============================================================================
 * HDL / HARDWARE CONTRACT
 * ============================================================================
 *
 * Hardware-related types describe source-level abstractions.
 *
 * They do not select a particular machine.
 *
 * Examples:
 *
 *     Hardware<Compute>
 *     Memory<T>
 *     Accelerator<Model>
 *     Interconnect<Message>
 *     Signal<T>
 *
 * Hardware realization belongs downstream.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY CONTRACT
 * ============================================================================
 *
 * Resource and capability types are semantic abstractions.
 *
 * For example:
 *
 *     Resource<T>
 *     Capability<T>
 *
 * do not themselves acquire, reserve or discover a resource.
 *
 * Requirements such as:
 *
 *     requires capability("quantum.measurement")
 *     requires capability("gpu.compute")
 *     requires capability("tensor.compute")
 *     requires qubits >= N
 *     requires memory >= RequiredMemory
 *
 * belong to the resource/capability subsystem.
 *
 * This grammar merely permits the corresponding types to participate in
 * source programs.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Types do not execute effects.
 *
 * Type syntax may carry an effect-qualified type form where supported by
 * Effectful, but effect semantics remain owned by:
 *
 *     grammar/effects/
 *
 * This file does not redefine the effect vocabulary.
 *
 * ============================================================================
 * CONTRACT / POLICY CONTRACT
 * ============================================================================
 *
 * Type syntax may participate in:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *     policy
 *
 * but this grammar does not implement those systems.
 *
 * Validation, policy and authorization remain downstream.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * This grammar creates no timestamps, hashes, IDs, machine information or
 * environment-derived values.
 *
 * Source locations are preserved by the generated parser/frontend.
 *
 * Provenance is attached by the frontend/compiler semantic pipeline.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This grammar contains NO lexer rules.
 *
 * All tokens originate from:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This grammar must therefore never introduce token aliases that duplicate
 * canonical lexical ownership.
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * The imports below represent the canonical delegate set.
 *
 * A delegate must:
 *
 *     1. be a parser grammar;
 *     2. use tokenVocab = ZamaniLexer;
 *     3. own only its specialized syntax;
 *     4. not define typeExpression;
 *     5. not import Types;
 *     6. not create a competing universal type system;
 *     7. not contain target-specific implementation actions.
 *
 * Files that currently violate those rules require normalization before they
 * can be imported by this orchestrator.
 *
 * ============================================================================
 * CANONICAL DELEGATE IMPORTS
 * ============================================================================
 */

parser grammar Types;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ---------------------------------------------------------------------------
 * Core scalar and named forms
 * ---------------------------------------------------------------------------
 */

import
    PrimitiveTypes,
    NamedTypes,
    Generic,
    Function,
    Tuple,
    Array,
    Slice,
    Result,
    RecordTypes,
    Sum,
    Union,
    Reference,
    PointerTypes,
    LinearTypes,
    AffineTypes,
    DependentTypes,
    Associated,
    TypeClasses,
    ExistentialTypes,
    ClassicalTypes,
    Quantum,
    ZamaniHardwareTypesParser,
    Resource,
    CapabilityTypes,
    TemporalTypes,
    Effectful,
    TypeBounds,
    TypeConstraints,
    AdvancedTypeLevelComputation;


/*
 * ============================================================================
 * 1. PUBLIC TYPE ENTRY POINT
 * ============================================================================
 *
 * This is the only source-level universal type entry point.
 *
 * The grammar intentionally separates:
 *
 *     prefixes
 *     core constructors
 *     postfixes
 *
 * This makes the type system extensible without requiring specialized
 * grammars to know how the complete type-expression graph is assembled.
 */

typeExpression
    : typePrefix*
      typeCore
      typePostfix*
    ;


/*
 * ============================================================================
 * 2. TYPE PREFIXES
 * ============================================================================
 *
 * Prefixes are ownership/semantic modifiers whose syntax is specialized.
 *
 * Their delegates MUST NOT consume the complete typeExpression.
 */

typePrefix
    : linearTypePrefix
    | affineTypePrefix
    | referenceTypePrefix
    | pointerTypePrefix
    | existentialTypePrefix
    ;


/*
 * ---------------------------------------------------------------------------
 * Linear types
 * ---------------------------------------------------------------------------
 */

linearTypePrefix
    : linearQualifier
    ;


/*
 * ---------------------------------------------------------------------------
 * Affine types
 * ---------------------------------------------------------------------------
 */

affineTypePrefix
    : affineQualifier
    ;


/*
 * ---------------------------------------------------------------------------
 * References
 * ---------------------------------------------------------------------------
 *
 * Reference owns only the reference prefix.
 *
 * The referenced type is supplied by this orchestrator.
 */

referenceTypePrefix
    : referencePrefix
    ;


/*
 * ---------------------------------------------------------------------------
 * Pointers
 * ---------------------------------------------------------------------------
 */

pointerTypePrefix
    : pointerQualifier
      pointerMutability?
      pointerVolatility?
    ;


/*
 * ---------------------------------------------------------------------------
 * Existential type prefix
 * ---------------------------------------------------------------------------
 *
 * The exact existential constructor remains owned by ExistentialTypes.
 */

existentialTypePrefix
    : existentialBinderList
    ;


/*
 * ============================================================================
 * 3. TYPE CORE
 * ============================================================================
 *
 * The core is intentionally domain-neutral.
 *
 * Specific type constructors are supplied by delegates.
 */

typeCore
    : primitiveType
    | namedType
    | genericType
    | functionType
    | tupleType
    | arrayType
    | sliceType
    | mapType
    | optionType
    | resultType
    | recordType
    | sumType
    | unionType
    | dependentType
    | associatedType
    | typeClassProjection
    | existentialType
    | classicalType
    | quantumType
    | hardwareType
    | resourceType
    | capabilityType
    | temporalType
    | effectfulType
    | parenthesizedType
    | advancedTypeLevelComputation
    ;


/*
 * ============================================================================
 * 4. NAMED / GENERIC TYPES
 * ============================================================================
 *
 * Generic application is a specialized extension of a named/path-based type.
 *
 * The Generic delegate owns argument syntax.
 * The NamedTypes delegate owns path syntax.
 */

genericType
    : namedType
      genericTypeApplicationSuffix+
    ;


/*
 * ============================================================================
 * 5. MAP TYPE
 * ============================================================================
 *
 * Map is included as a first-class universal collection constructor.
 *
 * The canonical Map delegate must expose mapType without importing Types.
 */

mapType
    : MAP
      LESS
      typeExpression
      COMMA
      typeExpression
      GREATER
    ;


/*
 * ============================================================================
 * 6. OPTION TYPE
 * ============================================================================
 *
 * The canonical optional representation is a postfix type constructor.
 *
 * Examples:
 *
 *     T?
 *     Result<T, E>?
 *     quantum::State?
 *
 * The specialized Option delegate owns the marker implementation.
 */

optionType
    : optionalTypePostfix
    ;


/*
 * ============================================================================
 * 7. RESULT TYPE
 * ============================================================================
 *
 * Result<T, E> is represented by the specialized Result delegate.
 */

resultType
    : RESULT
      LESS
      typeExpression
      COMMA
      typeExpression
      GREATER
    ;


/*
 * ============================================================================
 * 8. PARENTHESIZED TYPES
 * ============================================================================
 *
 * Parentheses are structural grouping and do not create a new semantic type.
 */

parenthesizedType
    : LPAREN
      typeExpression
      RPAREN
    ;


/*
 * ============================================================================
 * 9. TYPE POSTFIXES
 * ============================================================================
 *
 * Postfixes operate on a complete preceding type.
 *
 * This permits future postfix constructors to be introduced without
 * modifying every existing type family.
 */

typePostfix
    : typeApplicationPostfix
    | typeConstraintPostfix
    | typeEffectPostfix
    | typeProjectionPostfix
    ;


/*
 * ---------------------------------------------------------------------------
 * Generic/application postfix
 * ---------------------------------------------------------------------------
 *
 * The exact generic syntax remains owned by Generic.
 */

typeApplicationPostfix
    : genericTypeApplicationSuffix
    ;


/*
 * ---------------------------------------------------------------------------
 * Constraint postfix
 * ---------------------------------------------------------------------------
 *
 * Constraints are syntactic relationships.
 * Constraint solving remains semantic.
 */

typeConstraintPostfix
    : typeBoundClause
    | typeConstraintClause
    ;


/*
 * ---------------------------------------------------------------------------
 * Effect postfix
 * ---------------------------------------------------------------------------
 */

typeEffectPostfix
    : effectfulTypeClause
    ;


/*
 * ---------------------------------------------------------------------------
 * Associated/projection postfix
 * ---------------------------------------------------------------------------
 */

typeProjectionPostfix
    : associatedTypeProjectionSuffix
    ;


/*
 * ============================================================================
 * 10. VALUE-LEVEL TYPE PARAMETERS
 * ============================================================================
 *
 * Value parameters are required for scalable forms such as:
 *
 *     Vector<T, N>
 *     Tensor<T, Rows, Columns>
 *     QRegister<N>
 *     Array<T, N>
 *
 * The grammar does not assign a machine integer width to those values.
 *
 * They remain source-level expressions.
 */

typeValueExpression
    : typeValueBitwiseOr
    ;


typeValueBitwiseOr
    : typeValueBitwiseXor
      (PIPE typeValueBitwiseXor)*
    ;


typeValueBitwiseXor
    : typeValueBitwiseAnd
      (CARET typeValueBitwiseAnd)*
    ;


typeValueBitwiseAnd
    : typeValueShift
      (AMPERSAND typeValueShift)*
    ;


typeValueShift
    : typeValueAdditive
      (
          LEFT_SHIFT typeValueAdditive
        | RIGHT_SHIFT typeValueAdditive
      )*
    ;


typeValueAdditive
    : typeValueMultiplicative
      (
          PLUS typeValueMultiplicative
        | MINUS typeValueMultiplicative
      )*
    ;


typeValueMultiplicative
    : typeValueUnary
      (
          STAR typeValueUnary
        | SLASH typeValueUnary
        | MODULO typeValueUnary
      )*
    ;


typeValueUnary
    : PLUS typeValueUnary
    | MINUS typeValueUnary
    | TILDE typeValueUnary
    | typeValuePrimary
    ;


typeValuePrimary
    : INTEGER
    | IDENTIFIER
    | typeValueQualifiedPath
    | parenthesizedTypeValue
    ;


typeValueQualifiedPath
    : IDENTIFIER
      (
          DOUBLE_COLON
          IDENTIFIER
      )*
    ;


parenthesizedTypeValue
    : LPAREN
      typeValueExpression
      RPAREN
    ;


/*
 * ============================================================================
 * 11. ARRAY TYPE INTEGRATION
 * ============================================================================
 *
 * Array itself is owned by Array.
 *
 * This façade is intentionally thin.
 *
 * The delegate must expose:
 *
 *     arrayType
 *
 * and consume:
 *
 *     typeExpression
 *     typeValueExpression
 *
 * without importing Types.
 */


/*
 * ============================================================================
 * 12. FUNCTION TYPE INTEGRATION
 * ============================================================================
 *
 * Function owns:
 *
 *     functionType
 *     functionTypeParameterList
 *     functionTypeParameter
 *     functionTypeReturn
 *
 * Function parameters and return values may recursively use typeExpression.
 *
 * No function implementation details belong here.
 */


/*
 * ============================================================================
 * 13. TUPLE TYPE INTEGRATION
 * ============================================================================
 *
 * Tuple owns tuple structure.
 *
 * Its canonical grammar name MUST be:
 *
 *     Tuple
 *
 * It MUST NOT be named Types because this file is the sole Types grammar.
 */


/*
 * ============================================================================
 * 14. ALGEBRAIC TYPES
 * ============================================================================
 *
 * Sum and Union are distinct semantic constructions.
 *
 * Sum:
 *
 *     A | B
 *
 * where supported by the canonical syntax.
 *
 * Union:
 *
 *     union-specific representation
 *
 * Exact semantic interpretation is downstream.
 */


/*
 * ============================================================================
 * 15. DEPENDENT TYPES
 * ============================================================================
 *
 * DependentTypes may refer to:
 *
 *     type expressions
 *     value expressions
 *     symbolic dimensions
 *     dependent binders
 *
 * It must not impose a fixed dimension/rank/width ceiling.
 */


/*
 * ============================================================================
 * 16. ASSOCIATED TYPES / TYPE CLASSES
 * ============================================================================
 *
 * Associated types and type-class references participate in the same universal
 * type expression.
 *
 * Resolution is semantic.
 *
 * This grammar does not perform:
 *
 *     trait lookup
 *     instance selection
 *     coherence checking
 *     specialization
 *     type unification
 */


/*
 * ============================================================================
 * 17. CLASSICAL TYPES
 * ============================================================================
 *
 * ClassicalTypes supplies domain-specific mathematical/computational types.
 *
 * Examples include symbolic:
 *
 *     scalar
 *     vector
 *     matrix
 *     tensor
 *     complex
 *     rational
 *     numeric
 *
 * No fixed tensor rank or machine width is encoded here.
 */


/*
 * ============================================================================
 * 18. QUANTUM TYPES
 * ============================================================================
 *
 * Quantum supplies source-level quantum type constructors.
 *
 * Examples may include:
 *
 *     Qubit
 *     LogicalQubit
 *     QRegister<N>
 *     QuantumState<T>
 *
 * The grammar does not impose physical qubit limits.
 */


/*
 * ============================================================================
 * 19. HARDWARE TYPES
 * ============================================================================
 *
 * Hardware supplies abstract hardware types.
 *
 * It may describe:
 *
 *     compute
 *     memory
 *     accelerator
 *     device
 *     interface
 *     port
 *     interconnect
 *     topology
 *     capability
 *     timing
 *     power
 *     thermal
 *     reliability
 *
 * It must not bind a type to a particular physical machine.
 */


/*
 * ============================================================================
 * 20. RESOURCE / CAPABILITY TYPES
 * ============================================================================
 *
 * These provide type-level representation of abstract resources and
 * capabilities.
 *
 * Resource discovery and negotiation remain outside the parser.
 */


/*
 * ============================================================================
 * 21. TEMPORAL TYPES
 * ============================================================================
 *
 * TemporalTypes provides temporal type construction.
 *
 * Time/resource semantics remain downstream.
 */


/*
 * ============================================================================
 * 22. EFFECTFUL TYPES
 * ============================================================================
 *
 * Effectful supplies effect-qualified type syntax.
 *
 * The effect system itself remains owned by grammar/effects/.
 */


/*
 * ============================================================================
 * 23. TYPE BOUNDS / CONSTRAINTS
 * ============================================================================
 *
 * Bounds and constraints are syntax only.
 *
 * Examples:
 *
 *     T: Numeric
 *     T: Ordered
 *     N: Positive
 *
 * Actual satisfiability is semantic.
 */


/*
 * ============================================================================
 * 24. ADVANCED TYPE-LEVEL COMPUTATION
 * ============================================================================
 *
 * Advanced type-level computation is deliberately treated as an extension of
 * the same type system.
 *
 * It does not create a second type language.
 *
 * Any compile-time computation must remain subject to:
 *
 *     determinism
 *     resource policy
 *     diagnostics
 *     provenance
 *     compatibility
 *
 * Runtime execution does not occur in the grammar.
 */


/*
 * ============================================================================
 * 25. TYPE EXTENSIONS
 * ============================================================================
 *
 * Future type features can be attached through an explicitly registered
 * extension boundary.
 *
 * This is intentionally identifier-driven rather than keyword-enumerated.
 *
 * A future domain must not require an ever-growing universal keyword list.
 */

typeExtension
    : AT
      IDENTIFIER
      (
          LPAREN
          typeExtensionArgumentList?
          RPAREN
      )?
    ;


typeExtensionArgumentList
    : typeExtensionArgument
      (
          COMMA
          typeExtensionArgument
      )*
      COMMA?
    ;


typeExtensionArgument
    : IDENTIFIER
      (
          COLON
          typeExpression
      )?
      (
          ASSIGN
          typeValueExpression
      )?
    | typeExpression
    ;


/*
 * ============================================================================
 * 26. TYPE APPLICATION SAFETY
 * ============================================================================
 *
 * The parser accepts structurally valid applications.
 *
 * It does NOT decide whether:
 *
 *     T has the correct number of parameters
 *     a type argument satisfies a bound
 *     a value parameter is legal
 *     a capability exists
 *     a resource exists
 *     a quantum device supports a type
 *     hardware can realize a type
 *
 * Those are semantic diagnostics.
 */


/*
 * ============================================================================
 * 27. DETERMINISM
 * ============================================================================
 *
 * Type parsing depends only on:
 *
 *     source text
 *     canonical lexer vocabulary
 *     grammar version
 *     explicitly selected grammar configuration
 *
 * It must not depend on:
 *
 *     CPU count
 *     GPU availability
 *     QPU availability
 *     memory availability
 *     filesystem state
 *     network state
 *     wall-clock time
 *     randomness
 *     environment variables
 *     scheduler state
 *     deployment topology
 */


/*
 * ============================================================================
 * 28. SAFETY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no embedded Rust
 *     no semantic predicates
 *     no target-language actions
 *     no filesystem access
 *     no network access
 *     no hardware discovery
 *     no runtime execution
 *     no unsafe implementation
 *
 * Rust implementation requirements remain:
 *
 *     Rust 1.97+
 *     Rust 2021
 *     safe Rust
 *     no unsafe
 */


/*
 * ============================================================================
 * 29. AST CONTRACT
 * ============================================================================
 *
 * Every successful typeExpression parse must map into the repository's
 * existing domain-neutral TypeExpr representation.
 *
 * This grammar MUST NOT introduce:
 *
 *     QuantumTypeExpr
 *     HardwareTypeExpr
 *     AITypeExpr
 *     UniversalTypeExpr
 *     TypeExpr2
 *     DomainTypeIR
 *
 * Domain-specific semantics are represented downstream.
 */


/*
 * ============================================================================
 * 30. SEMANTIC CONTRACT
 * ============================================================================
 *
 * After parsing:
 *
 *     TypeExpr
 *        |
 *        +--> name resolution
 *        +--> generic substitution
 *        +--> inference
 *        +--> unification
 *        +--> bounds
 *        +--> constraints
 *        +--> ownership
 *        +--> lifetimes
 *        +--> effects
 *        +--> capabilities
 *        +--> resources
 *        +--> contracts
 *        +--> policies
 *        +--> provenance
 *        |
 *        v
 *     canonical semantic type
 *
 * The semantic type may subsequently participate in:
 *
 *     classical IR
 *     quantum::ir
 *     HDL semantics
 *     hardware semantics
 *     AI/data semantics
 *     distributed semantics
 *     networking semantics
 */


/*
 * ============================================================================
 * 31. NO PHYSICAL REALIZATION
 * ============================================================================
 *
 * This grammar never decides:
 *
 *     CPU vs GPU
 *     FPGA vs ASIC
 *     accelerator vs CPU
 *     QPU vs simulator
 *     local vs distributed
 *     embedded vs HPC
 *     cloud vs cluster
 *
 * Those decisions belong to compilation/execution/resource negotiation.
 */


/*
 * ============================================================================
 * 32. POCO-REAF INVARIANT
 * ============================================================================
 *
 * The same source-level type must retain the same semantic meaning when
 * realized on different target scales.
 *
 * Target realization may change:
 *
 *     representation
 *     layout
 *     specialization
 *     routing
 *     scheduling
 *     execution strategy
 *
 * It must not silently change:
 *
 *     source type meaning
 *     type safety
 *     declared contracts
 *     declared effects
 *     declared resource requirements
 *     declared capabilities
 *
 * If a target cannot satisfy the semantic requirements, the compiler/runtime
 * must report or negotiate the condition through the resource/capability
 * system rather than silently changing the program's type semantics.
 */


/*
 * ============================================================================
 * 33. REQUIRED INTEGRATION CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/lexer/tokens.g4
 *     canonical type delegates under grammar/types/
 *
 * EXPORTS:
 *
 *     typeExpression
 *     typeCore
 *     typePrefix
 *     typePostfix
 *     typeValueExpression
 *
 * CONSUMED_BY:
 *
 *     grammar/Zamani.g4
 *     grammar/declarations/
 *     grammar/functions/
 *     grammar/expressions/
 *     grammar/statements/
 *     grammar/classical/
 *     grammar/quantum/
 *     grammar/hybrid/
 *     grammar/hdl/
 *     grammar/ai/
 *     grammar/data/
 *     grammar/distributed/
 *     grammar/networking/
 *     grammar/interoperability/
 *     grammar/metaprogramming/
 *
 * AST_OWNER:
 *
 *     existing frontend TypeExpr model
 *
 * SEMANTIC_OWNER:
 *
 *     existing semantic type system
 *
 * IR_OWNER:
 *
 *     canonical semantic IR
 *     classical IR
 *     quantum::ir where applicable
 *
 * SPEC_OWNER:
 *
 *     grammar/specification/
 *     grammar/spec/
 *
 * TEST_OWNER:
 *
 *     grammar/tests/types/
 *
 * COMPATIBILITY_OWNER:
 *
 *     grammar/compatibility/
 *
 * RESOURCE_OWNER:
 *
 *     grammar/resources/
 *
 * EFFECT_OWNER:
 *
 *     grammar/effects/
 *
 * POLICY_OWNER:
 *
 *     grammar/policies/
 *
 * PROVENANCE_OWNER:
 *
 *     compiler/frontend provenance subsystem
 */


/*
 * ============================================================================
 * 34. REQUIRED DELEGATE NORMALIZATION
 * ============================================================================
 *
 * The current repository contains several transitional delegates whose grammar
 * names/import relationships prevent direct production composition.
 *
 * Before this orchestrator can be generated as one ANTLR grammar, the following
 * normalization is required:
 *
 *     grammar/types/tuple.g4
 *         parser grammar Tuple;
 *
 *     grammar/types/map.g4
 *         MUST NOT import Types;
 *
 *     grammar/types/option.g4
 *         parser grammar Option;
 *
 *     grammar/types/reference.g4
 *         parser grammar Reference;
 *
 *     grammar/types/dependent.g4
 *         parser grammar DependentTypes;
 *
 *     grammar/types/constraints.g4
 *         parser grammar TypeConstraints;
 *
 *     grammar/types/result.g4
 *         must expose the complete resultType constructor rather than only
 *         resultTypeArguments if that is the selected canonical owner;
 *
 *     grammar/types/array.g4
 *         must remain the sole owner of arrayType/arrayLength;
 *
 *     grammar/types/function.g4
 *         must remain the sole owner of functionType;
 *
 *     grammar/types/reference.g4
 *         must remain the sole owner of referencePrefix and lifetime syntax;
 *
 *     grammar/types/pointer.g4
 *         must remain the sole owner of pointer syntax;
 *
 *     grammar/types/linear.g4
 *         must remain the sole owner of linearQualifier;
 *
 *     grammar/types/affine.g4
 *         must remain the sole owner of affineQualifier.
 *
 * These changes are architectural normalization, not additions to the type
 * language.
 */


/*
 * ============================================================================
 * 35. IMPORTANT CYCLIC-DEPENDENCY RULE
 * ============================================================================
 *
 * NO imported delegate may import Types.
 *
 * Forbidden:
 *
 *     Types -> Map -> Types
 *     Types -> Generic -> Types
 *     Types -> Effectful -> Types
 *     Types -> Reference -> Types
 *
 * Delegates may consume:
 *
 *     typeExpression
 *     typeCore
 *     typeValueExpression
 *
 * as rules supplied by the composing grammar.
 *
 * They must not import the composing grammar to obtain those rules.
 */


/*
 * ============================================================================
 * 36. LEGACY FILE POLICY
 * ============================================================================
 *
 * Files with overlapping names such as:
 *
 *     array-types.g4
 *     associated types.g4
 *     composite-types.g4
 *     generic-types.g4
 *     map-types.g4
 *     option-types.g4
 *     existential-types.g4
 *
 * must not silently become additional authorities.
 *
 * Each must be classified as one of:
 *
 *     canonical delegate
 *     compatibility façade
 *     historical reference
 *     deprecated
 *     experimental
 *
 * Only one canonical implementation of each rule may participate in the
 * production import graph.
 */


/*
 * ============================================================================
 * 37. TEST CONTRACT
 * ============================================================================
 *
 * Positive:
 *
 *     int
 *     bool
 *     String
 *     User
 *     module::User
 *     Vec<T>
 *     Map<K, V>
 *     Result<T, E>
 *     Option<T>
 *     (A, B)
 *     [T; N]
 *     [T]
 *     fn(A, B) -> C
 *     &T
 *     &mut T
 *     *T
 *     linear T
 *     affine T
 *     QuantumState<T>
 *     QRegister<N>
 *     Tensor<T>
 *     Tensor<T>[N, M]
 *     Hardware<Compute>
 *     Resource<T>
 *     Capability<T>
 *     Distribution<T>
 *     Model<T>
 *
 * Hybrid:
 *
 *     Result<QuantumState<T>, Error>
 *     Tensor<QuantumState<T>>
 *     Vec<Hardware<Accelerator>>
 *     Model<Tensor<float>>
 *     QRegister<N>
 *
 * Nested:
 *
 *     &&T
 *     &Vec<T>
 *     &mut Result<T, E>
 *     Vec<Result<T, E>>
 *     Map<K, Vec<V>>
 *     Tensor<Vec<T>>
 *
 * Symbolic:
 *
 *     Vector<T, N>
 *     Matrix<T, Rows, Columns>
 *     Tensor<T, Shape>
 *     QRegister<N>
 *
 * Negative:
 *
 *     incomplete generic arguments
 *     incomplete tuple
 *     incomplete reference
 *     incomplete function type
 *     malformed array
 *     malformed map
 *     malformed result
 *
 * Boundary:
 *
 *     every canonical type family nested inside every composite family where
 *     semantically legal.
 *
 * Scalability:
 *
 *     generated symbolic dimensions
 *     generated nested generic structures
 *     generated nested composite types
 *     large qualified paths
 *     large type argument lists
 *
 * Scalability tests MUST NOT encode an arbitrary language maximum.
 */


/*
 * ============================================================================
 * 38. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no physical resource limits.
 *
 * It contains no:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_REGISTER_WIDTH
 *     MAX_TENSOR_RANK
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * Numeric type parameters remain symbolic.
 *
 * Compiler protection limits, if required for hostile input, belong to an
 * explicitly configurable compiler resource policy and are not language
 * semantics.
 */


/*
 * ============================================================================
 * 39. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     1. The grammar name is exactly `Types`.
 *
 *     2. `typeExpression` is the only universal public type entry point.
 *
 *     3. The root grammar imports Types.
 *
 *     4. No other grammar defines a competing universal typeExpression.
 *
 *     5. Specialized delegates own their specialized constructors.
 *
 *     6. No delegate imported here imports Types.
 *
 *     7. No delegate creates a second type system.
 *
 *     8. All delegates consume ZamaniLexer.
 *
 *     9. All canonical token ownership remains in ZamaniLexer.
 *
 *    10. Named and generic types remain open-ended.
 *
 *    11. Symbolic dimensions remain open-ended.
 *
 *    12. Quantum types remain target-neutral.
 *
 *    13. Hardware types remain target-neutral.
 *
 *    14. Resource/capability types remain target-neutral.
 *
 *    15. No physical capacity constants exist.
 *
 *    16. No Rust code exists inside the grammar.
 *
 *    17. No unsafe implementation exists.
 *
 *    18. Rust 1.97+ compatibility remains an implementation requirement.
 *
 *    19. Positive tests exist.
 *
 *    20. Negative tests exist.
 *
 *    21. Boundary tests exist.
 *
 *    22. Scalability tests exist.
 *
 *    23. Cross-domain tests exist.
 *
 *    24. AST integration is verified.
 *
 *    25. Semantic type integration is verified.
 *
 *    26. Classical IR integration is verified.
 *
 *    27. quantum::ir integration is verified where applicable.
 *
 *    28. HDL/hardware semantic integration is verified.
 *
 *    29. Effects/capabilities/resources remain downstream.
 *
 *    30. Contracts/policies/provenance remain downstream.
 *
 *    31. ANTLR generation succeeds without grammar cycles.
 *
 *    32. The generated parser has one canonical type-expression hierarchy.
 *
 * ============================================================================
 */
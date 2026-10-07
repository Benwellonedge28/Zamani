/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/type-families.g4
 *
 * Grammar:
 *     TypeFamilies
 *
 * Status:
 *     PRODUCTION TYPE-SYSTEM DELEGATE
 *
 * Compiler baseline:
 *     Rust 1.97+
 *     Rust 2021
 *     safe Rust only
 *     no unsafe
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * Define the source-level declaration and equation syntax for type families.
 *
 * A type family is a type-level mapping from one or more type-level inputs
 * to a type-level result.
 *
 * Conceptually:
 *
 *     type family Element<T> : *;
 *
 *     type family Element<T> : * {
 *         Element<List<T>> = T;
 *     }
 *
 *     type family Transform<F, T> : *;
 *
 *     type family Transform<F, T> : * {
 *         Transform<Identity, T> = T;
 *     }
 *
 * The grammar represents structure only.
 *
 * Type-family reduction, overlap checking, termination, injectivity,
 * confluence, kind checking, normalization, substitution and specialization
 * belong to semantic analysis.
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
 *     type-family syntax
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     type-family semantic model
 *          |
 *          +----------------------+
 *          |                      |
 *          v                      v
 *     type normalization     kind analysis
 *          |                      |
 *          +----------+-----------+
 *                     |
 *                     v
 *              semantic type model
 *                     |
 *          +----------+-----------+
 *          |                      |
 *          v                      v
 *      classical              quantum semantics
 *                                 |
 *                                 v
 *                             quantum::ir
 *                     |
 *                     v
 *              canonical IR
 *                     |
 *                     v
 *             target-independent
 *                optimization
 *                     |
 *                     v
 *             target realization
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns ONLY:
 *
 *     typeFamilyDeclaration
 *     typeFamilyInstance
 *     typeFamilyEquation
 *     typeFamilyName
 *     typeFamilyParameterList
 *     typeFamilyParameter
 *     typeFamilyParameterKind
 *     typeFamilyResultKind
 *     typeFamilyEquationArguments
 *
 * It owns the source syntax distinguishing:
 *
 *     family declaration
 *     family equation
 *     family instance
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     typeExpression
 *     genericTypeArguments
 *     genericArgumentList
 *     genericParameter
 *     typeBound
 *     type-class declarations
 *     trait declarations
 *     associated-type declarations
 *     associated-type projections
 *     kind-expression implementation
 *     type inference
 *     kind inference
 *     kind unification
 *     type unification
 *     family reduction
 *     family normalization
 *     family overlap checking
 *     family termination checking
 *     family confluence checking
 *     injectivity checking
 *     specialization
 *     monomorphization
 *     resource negotiation
 *     capability negotiation
 *     hardware selection
 *     quantum allocation
 *     routing
 *     scheduling
 *     QEC
 *     ZQN
 *     HAL
 *
 * ============================================================================
 * SINGLE-OWNER RULE
 * ============================================================================
 *
 * Ordinary generic application:
 *
 *     F<T>
 *     F<A, B>
 *
 * remains owned by:
 *
 *     grammar/types/generic.g4
 *
 * A family application is therefore NOT given a second application grammar.
 *
 * Example:
 *
 *     Element<List<T>>
 *
 * is parsed using the existing generic/type-application machinery.
 *
 * Whether `Element` is a type family is semantic information.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Existing TypeExpr already provides:
 *
 *     TypeExpr::Generic
 *     TypeExpr::TypeApplication
 *     TypeExpr::Hkt
 *
 * Family APPLICATIONS must lower to the existing type-application structure.
 *
 * For example:
 *
 *     Element<List<T>>
 *
 * becomes structurally equivalent to:
 *
 *     TypeExpr::TypeApplication {
 *         constructor: Element,
 *         arguments: [List<T>],
 *     }
 *
 * or the repository's canonical generic representation where that is the
 * established source representation.
 *
 * This grammar MUST NOT create:
 *
 *     TypeFamilyTypeExpr
 *     TypeFamilyApplication
 *     TypeFamilyIR
 *     FamilyTypeExpr
 *
 * as a competing AST hierarchy.
 *
 * ============================================================================
 * AST REQUIREMENT FOR DECLARATIONS
 * ============================================================================
 *
 * The current TypeExpr enum represents TYPE EXPRESSIONS, not declarations.
 *
 * Therefore a production frontend requires declaration-level AST support for:
 *
 *     type family declaration
 *     type family parameter
 *     type family equation
 *     type family instance
 *
 * That support must be added to the declaration AST owner rather than encoded
 * inside this grammar.
 *
 * Recommended semantic structure:
 *
 *     TypeFamilyDecl
 *         name
 *         parameters
 *         result_kind
 *         mode
 *         source_span
 *         provenance
 *
 *     TypeFamilyEquation
 *         family
 *         arguments
 *         result
 *         source_span
 *         provenance
 *
 *     TypeFamilyMode
 *         Open
 *         Closed
 *
 * The exact Rust representation is owned by the frontend declaration AST and
 * semantic type-system implementation.
 *
 * ============================================================================
 * KIND CONTRACT
 * ============================================================================
 *
 * Explicit family result kinds use the existing higher-kinded kind grammar.
 *
 * Examples:
 *
 *     *
 *
 *     * -> *
 *
 *     * -> * -> *
 *
 *     (* -> *) -> *
 *
 * Parameter kinds may also be explicitly specified:
 *
 *     type family Map<F, (T : *)> : *;
 *
 *     type family Apply<(F : * -> *), T> : *;
 *
 * The actual kind structure is supplied by:
 *
 *     grammar/types/higher-kinded-types.g4
 *
 * This file MUST NOT redefine kind expressions.
 *
 * ============================================================================
 * PARAMETER CONTRACT
 * ============================================================================
 *
 * A family parameter is source-level metadata.
 *
 * It may have an optional explicit kind:
 *
 *     T
 *
 *     T : *
 *
 *     F : * -> *
 *
 * The semantic layer determines:
 *
 *     inferred kind;
 *     explicit kind;
 *     kind compatibility;
 *     kind unification;
 *     whether the family parameter is valid.
 *
 * The grammar does not infer kinds.
 *
 * ============================================================================
 * FAMILY ARITY
 * ============================================================================
 *
 * Family arity is semantic declaration metadata.
 *
 * It MUST NOT be represented by a finite set of grammar alternatives.
 *
 * Correct:
 *
 *     <T>
 *     <T, U>
 *     <T, U, V>
 *     ...
 *
 * Incorrect architecture:
 *
 *     family0
 *     family1
 *     family2
 *     family3
 *     ...
 *
 * No universal family-arity ceiling exists.
 *
 * ============================================================================
 * FAMILY MODES
 * ============================================================================
 *
 * Two source-level modes are supported:
 *
 * 1. Open family declaration:
 *
 *     type family F<T> : *;
 *
 *    followed by separately declared family instances.
 *
 * 2. Closed family declaration:
 *
 *     type family F<T> : * {
 *         F<Int> = Bool;
 *         F<String> = Text;
 *     }
 *
 * Closed equations are ordered source declarations.
 *
 * The semantic layer determines:
 *
 *     overlap;
 *     compatibility;
 *     reduction order;
 *     completeness;
 *     termination;
 *     confluence.
 *
 * ============================================================================
 * OPEN FAMILY INSTANCES
 * ============================================================================
 *
 * Open-family equations are declared using:
 *
 *     type family instance F<Int> = Bool;
 *
 *     type family instance F<List<T>> = T;
 *
 * The grammar preserves the equation.
 *
 * The semantic system verifies that:
 *
 *     F
 *
 * is an existing open family;
 *
 * the argument count is correct;
 *
 * the argument kinds are valid;
 *
 * the pattern is legal;
 *
 * the equation is compatible with existing instances;
 *
 * the result is well formed.
 *
 * ============================================================================
 * CLOSED FAMILY EQUATIONS
 * ============================================================================
 *
 * Closed-family equations occur inside the family declaration:
 *
 *     type family F<T> : * {
 *         F<Int> = Bool;
 *         F<String> = Text;
 *         F<T> = Unknown<T>;
 *     }
 *
 * Source ordering is significant.
 *
 * The grammar therefore uses an ordered repeated production.
 *
 * ============================================================================
 * EQUATION LEFT-HAND SIDE
 * ============================================================================
 *
 * The left-hand side contains:
 *
 *     family name
 *     optional generic arguments
 *
 * Examples:
 *
 *     F
 *
 *     F<Int>
 *
 *     F<List<T>>
 *
 *     F<Map<K, V>>
 *
 * The grammar deliberately permits structurally valid type expressions.
 *
 * Semantic analysis determines whether a particular expression is an
 * admissible family pattern.
 *
 * This is important for extensibility because the grammar must not maintain
 * a closed list of legal pattern constructors.
 *
 * ============================================================================
 * EQUATION RIGHT-HAND SIDE
 * ============================================================================
 *
 * The right-hand side is a canonical:
 *
 *     typeExpression
 *
 * It may therefore compose with:
 *
 *     named types
 *     generic types
 *     tuples
 *     arrays
 *     functions
 *     references
 *     pointers
 *     dependent types
 *     associated types
 *     higher-kinded applications
 *     quantum types
 *     hardware-neutral types
 *     resource types
 *     capability types
 *     future domain-neutral types
 *
 * No domain receives a special family RHS grammar.
 *
 * ============================================================================
 * TYPE FAMILY APPLICATION
 * ============================================================================
 *
 * This file MUST NOT define:
 *
 *     typeFamilyApplication
 *
 * as a competing generic application grammar.
 *
 * Existing syntax:
 *
 *     F<T>
 *
 * is already handled by:
 *
 *     grammar/types/generic.g4
 *
 * The semantic resolver determines whether:
 *
 *     F
 *
 * denotes:
 *
 *     ordinary type;
 *     generic type constructor;
 *     type family;
 *     higher-kinded constructor;
 *     another extensible type-level entity.
 *
 * ============================================================================
 * ASSOCIATED TYPE INTEGRATION
 * ============================================================================
 *
 * Associated types and type families are related semantic concepts but have
 * different declaration ownership.
 *
 * Associated projections remain owned by:
 *
 *     grammar/types/associated.g4
 *
 * This file MUST NOT redefine:
 *
 *     T::Item
 *
 * or:
 *
 *     T::Output
 *
 * A family may participate in the semantic resolution of an associated type,
 * but that is downstream.
 *
 * ============================================================================
 * HIGHER-KINDED TYPE INTEGRATION
 * ============================================================================
 *
 * Higher-kinded syntax remains owned by:
 *
 *     grammar/types/higher-kinded-types.g4
 *
 * Type families may have higher-kinded parameters and results.
 *
 * Example:
 *
 *     type family Transform<F : * -> *, T : *> : *;
 *
 * The kind syntax itself is not reimplemented here.
 *
 * ============================================================================
 * DEPENDENT TYPE INTEGRATION
 * ============================================================================
 *
 * A family result may participate in dependent types where the canonical
 * dependent grammar permits it.
 *
 * Example conceptual form:
 *
 *     type family BufferType<N : *> : *;
 *
 *     [BufferType<N>; N]
 *
 * This grammar does not define dependent cardinality syntax.
 *
 * ============================================================================
 * TYPE-LEVEL COMPUTATION
 * ============================================================================
 *
 * A type family is a semantic type-level computation.
 *
 * It MUST NOT be confused with:
 *
 *     runtime function execution;
 *     compile-time arbitrary code execution;
 *     metaprogramming side effects.
 *
 * Family reduction must be:
 *
 *     deterministic;
 *     pure with respect to source semantics;
 *     reproducible;
 *     explicitly bounded by compiler resource policy where necessary.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Type-family declaration and reduction are effect-free at the language
 * semantic level.
 *
 * They do not imply:
 *
 *     IO
 *     network
 *     mutation
 *     randomness
 *     measurement
 *     learning
 *     adaptation
 *     reflection
 *     foreign execution
 *     device access
 *
 * Compile-time implementation work may consume compiler resources, but this
 * is an implementation concern rather than a source-language effect.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * A type family does not request a runtime capability merely by being
 * declared or reduced.
 *
 * Family semantics must not inspect:
 *
 *     CPU capability;
 *     GPU capability;
 *     FPGA capability;
 *     QPU capability;
 *     network capability;
 *     device capability;
 *     hardware topology.
 *
 * Capability requirements belong to the capability subsystem.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Type-family syntax imposes no physical resource requirement.
 *
 * A family may describe resource-related types such as:
 *
 *     Resource<Device>
 *
 *     Buffer<T>
 *
 *     Accelerator<Model>
 *
 * but the grammar does not allocate or discover those resources.
 *
 * ============================================================================
 * CONTRACT / POLICY CONTRACT
 * ============================================================================
 *
 * Family declarations may be subject to:
 *
 *     requires
 *     ensures
 *     invariant
 *     property
 *     policy
 *
 * but this grammar does not duplicate those systems.
 *
 * Any applicable contract or policy is attached and validated by the
 * corresponding declaration/semantic subsystem.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Source spans must be preserved for:
 *
 *     family keyword;
 *     family name;
 *     parameter list;
 *     parameter kinds;
 *     result kind;
 *     equation family name;
 *     equation arguments;
 *     equation result.
 *
 * Semantic provenance should additionally record:
 *
 *     declaration;
 *     equation;
 *     substitution;
 *     reduction;
 *     normalization;
 *     specialization;
 *     verification;
 *     diagnostic cause.
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * Type families may describe abstractions involving quantum types:
 *
 *     type family Logical<T> : *;
 *
 *     type family StateOf<T> : *;
 *
 *     type family RegisterOf<T> : *;
 *
 * They must remain independent of:
 *
 *     physical qubit count;
 *     physical qubit identity;
 *     QPU topology;
 *     calibration;
 *     routing;
 *     scheduling;
 *     QEC;
 *     vendor gate sets.
 *
 * Family results that participate in quantum semantics eventually flow through
 * the existing quantum semantic boundary:
 *
 *     TypeExpr
 *        |
 *        v
 *     semantic quantum model
 *        |
 *        v
 *     quantum::ir
 *
 * ============================================================================
 * HDL / HARDWARE CONTRACT
 * ============================================================================
 *
 * Families may describe hardware-neutral relationships:
 *
 *     type family SignalOf<T> : *;
 *
 *     type family StorageOf<T> : *;
 *
 *     type family InterfaceOf<T> : *;
 *
 * The grammar MUST NOT encode:
 *
 *     fixed register widths;
 *     fixed memory;
 *     fixed number of devices;
 *     fixed number of ports;
 *     fixed topology;
 *     fixed accelerator count.
 *
 * ============================================================================
 * AI / DATA CONTRACT
 * ============================================================================
 *
 * Families may express domain-neutral relationships such as:
 *
 *     type family FeatureOf<T> : *;
 *
 *     type family ModelInput<M> : *;
 *
 *     type family ModelOutput<M> : *;
 *
 *     type family DistributionOf<T> : *;
 *
 * They do not introduce AI-specific type keywords or algorithm catalogs.
 *
 * ============================================================================
 * DISTRIBUTED / NETWORK CONTRACT
 * ============================================================================
 *
 * Families may express:
 *
 *     StreamOf<T>
 *     MessageOf<T>
 *     ReplicaOf<T>
 *     PartitionOf<T>
 *
 * without encoding:
 *
 *     node count;
 *     network size;
 *     topology;
 *     transport;
 *     deployment count.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar imposes NO language-level maximum on:
 *
 *     family declarations;
 *     family parameters;
 *     family equations;
 *     family argument count;
 *     type nesting;
 *     kind nesting;
 *     namespace depth;
 *     source program size;
 *     domain count;
 *     target count;
 *     hardware size;
 *     quantum size.
 *
 * It MUST NOT define:
 *
 *     MAX_TYPE_FAMILIES
 *     MAX_FAMILY_PARAMETERS
 *     MAX_FAMILY_ARITY
 *     MAX_FAMILY_EQUATIONS
 *     MAX_FAMILY_DEPTH
 *     MAX_KIND_DEPTH
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_DEVICE_COUNT
 *
 * Compiler denial-of-service protection may use an explicit configurable
 * resource policy.
 *
 * Such a policy is NOT a language semantic ceiling.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing is deterministic for identical:
 *
 *     source;
 *     language version;
 *     grammar version;
 *     lexical configuration;
 *     dialect configuration.
 *
 * Family equation ordering must be preserved exactly.
 *
 * No parser rule may depend on:
 *
 *     wall-clock time;
 *     randomness;
 *     filesystem state;
 *     network state;
 *     hardware availability;
 *     runtime state.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing syntax must retain its meaning:
 *
 *     type Foo = Bar;
 *     Vec<T>
 *     Result<T, E>
 *     T::Item
 *     F<T>
 *
 * A type family must be distinguished lexically by:
 *
 *     family
 *
 * rather than by heuristic reinterpretation of an ordinary `type`
 * declaration.
 *
 * This prevents existing type aliases/declarations from becoming ambiguous.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no embedded Rust;
 *     no semantic predicates;
 *     no filesystem access;
 *     no network access;
 *     no subprocess execution;
 *     no runtime execution;
 *     no hardware access;
 *     no target probing.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This grammar itself contains no Rust implementation.
 *
 * The consuming compiler must remain compatible with:
 *
 *     Rust 1.97+
 *     Rust 2021
 *     safe Rust
 *
 * and must not require `unsafe`.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/types/generic.g4
 *     grammar/types/higher-kinded-types.g4
 *     canonical typeExpression owner
 *
 * EXPORTS:
 *
 *     typeFamilyDeclaration
 *     typeFamilyInstance
 *     typeFamilyEquation
 *
 * CONSUMED_BY:
 *
 *     declaration composition
 *     type-system semantic analysis
 *     frontend AST construction
 *
 * AST_OWNER:
 *
 *     frontend declaration AST
 *
 * SEMANTIC_OWNER:
 *
 *     type-family semantic/type-resolution subsystem
 *
 * IR_OWNER:
 *
 *     canonical semantic type model
 *
 *     No family-specific IR is introduced.
 *
 * TEST_OWNER:
 *
 *     grammar/tests/types/
 *     frontend type-family tests
 *     semantic type-family tests
 *
 * SPEC_OWNER:
 *
 *     grammar/specification/types.md
 *     grammar/specification/type-families.md
 *
 * COMPATIBILITY_OWNER:
 *
 *     grammar/compatibility/
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * This file MUST be imported by the declaration-level parser composition,
 * not by `typeExpression` itself.
 *
 * Correct:
 *
 *     declarations
 *         |
 *         +--> typeFamilyDeclaration
 *
 *     types
 *         |
 *         +--> generic application
 *
 * Incorrect:
 *
 *     typeExpression
 *         |
 *         +--> typeFamilyDeclaration
 *
 * A declaration is not a type expression.
 *
 * The canonical type-expression grammar remains responsible for consuming
 * family APPLICATIONS through ordinary generic/type-application syntax.
 *
 * ============================================================================
 * REQUIRED EXTERNAL INTEGRATION
 * ============================================================================
 *
 * 1. LEXER
 *
 * Add:
 *
 *     FAMILY : 'family'
 *     INSTANCE : 'instance'
 *
 * to the canonical keyword authority.
 *
 * They must be added to the canonical lexer assembly and token registry.
 *
 *
 * 2. DECLARATION COMPOSITION
 *
 * Import:
 *
 *     TypeFamilies
 *
 * into the canonical declaration parser.
 *
 * The declaration dispatcher should expose:
 *
 *     typeFamilyDeclaration
 *     typeFamilyInstance
 *
 * at declaration position.
 *
 *
 * 3. TYPE EXPRESSION COMPOSITION
 *
 * Do NOT add family declarations to `typeExpression`.
 *
 * Family applications continue through:
 *
 *     generic.g4
 *
 *
 * 4. AST
 *
 * Add declaration-level source AST support for:
 *
 *     TypeFamilyDecl
 *     TypeFamilyParameter
 *     TypeFamilyEquation
 *     TypeFamilyMode
 *
 * without modifying `TypeExpr` to contain declarations.
 *
 *
 * 5. SEMANTICS
 *
 * Implement one family environment containing:
 *
 *     family identity;
 *     parameters;
 *     parameter kinds;
 *     result kind;
 *     openness/closedness;
 *     equations;
 *     source provenance.
 *
 * Family resolution must reuse the existing type environment rather than
 * creating a second independent type resolver.
 *
 *
 * 6. KIND CHECKING
 *
 * Reuse:
 *
 *     grammar/types/higher-kinded-types.g4
 *
 * and the existing semantic kind system.
 *
 * Do not implement kind checking in parser predicates.
 *
 *
 * 7. TYPE REDUCTION
 *
 * Family reduction must occur in semantic type analysis.
 *
 * The reducer must preserve:
 *
 *     source provenance;
 *     deterministic equation ordering;
 *     substitution information;
 *     diagnostics.
 *
 *
 * 8. IR
 *
 * Type-family declarations do not create a new runtime IR.
 *
 * After normalization they contribute to the canonical semantic type model.
 *
 * Quantum-related results eventually continue through:
 *
 *     quantum::ir
 *
 *
 * 9. RESOURCE / CAPABILITY
 *
 * Family resolution must not perform target discovery.
 *
 * Resource and capability requirements remain downstream.
 *
 *
 * 10. TESTS
 *
 * Add:
 *
 *     parser positive tests;
 *     parser negative tests;
 *     AST tests;
 *     kind tests;
 *     reduction tests;
 *     overlap tests;
 *     recursion/termination tests;
 *     provenance tests;
 *     deterministic tests;
 *     scalability tests;
 *     compatibility tests;
 *     cross-domain tests.
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar TypeFamilies;

options {
    tokenVocab = ZamaniLexer;
}

import HigherKindedTypes;


/*
 * ============================================================================
 * PUBLIC DECLARATION
 * ============================================================================
 *
 * Open family:
 *
 *     type family Element<T> : *;
 *
 * Closed family:
 *
 *     type family Element<T> : * {
 *         Element<List<T>> = T;
 *     }
 *
 * Result kind is optional at the syntax level.
 *
 * Semantic analysis determines whether kind inference succeeds when it is
 * omitted.
 */
typeFamilyDeclaration
    : TYPE
      FAMILY
      typeFamilyName
      typeFamilyParameterList?
      typeFamilyResultKind?
      (
          LBRACE
          typeFamilyEquation*
          RBRACE
        | SEMICOLON
      )
    ;


/*
 * ============================================================================
 * OPEN FAMILY INSTANCE
 * ============================================================================
 *
 * Example:
 *
 *     type family instance Element<List<Int>> = Int;
 */
typeFamilyInstance
    : TYPE
      FAMILY
      INSTANCE
      typeFamilyEquation
    ;


/*
 * ============================================================================
 * FAMILY NAME
 * ============================================================================
 *
 * A family name is a declaration identifier.
 *
 * Qualified declaration names are intentionally not accepted here. Module
 * qualification belongs to the declaration/name-resolution architecture.
 */
typeFamilyName
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * FAMILY PARAMETER LIST
 * ============================================================================
 *
 * Examples:
 *
 *     <T>
 *     <T, U>
 *     <T, U, V>
 *
 * Explicit kinds:
 *
 *     <T : *>
 *     <F : * -> *>
 *     <F : * -> *, T : *>
 *
 * There is no finite parameter limit.
 */
typeFamilyParameterList
    : LESS
      typeFamilyParameter
      (
          COMMA
          typeFamilyParameter
      )*
      COMMA?
      GREATER
    ;


/*
 * ============================================================================
 * FAMILY PARAMETER
 * ============================================================================
 *
 * The parameter itself remains an ordinary source-level type parameter.
 *
 * Kind syntax is delegated to HigherKindedTypes.
 */
typeFamilyParameter
    : IDENTIFIER
      typeFamilyParameterKind?
    ;


/*
 * ============================================================================
 * PARAMETER KIND
 * ============================================================================
 *
 * Example:
 *
 *     T : *
 *
 *     F : * -> *
 */
typeFamilyParameterKind
    : COLON
      higherKindedKind
    ;


/*
 * ============================================================================
 * FAMILY RESULT KIND
 * ============================================================================
 *
 * Example:
 *
 *     type family F<T> : *;
 *
 *     type family F<F : * -> *, T : *> : *;
 */
typeFamilyResultKind
    : COLON
      higherKindedKind
    ;


/*
 * ============================================================================
 * FAMILY EQUATION
 * ============================================================================
 *
 * Examples:
 *
 *     F<Int> = Bool;
 *
 *     F<List<T>> = T;
 *
 *     F<Map<K, V>> = Result<K, V>;
 *
 *     F = Int;
 *
 * The semantic layer validates whether the equation is legal for the
 * declared family arity and kind.
 */
typeFamilyEquation
    : typeFamilyName
      typeFamilyEquationArguments?
      ASSIGN
      typeExpression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * EQUATION ARGUMENTS
 * ============================================================================
 *
 * Reuses the canonical generic application syntax.
 *
 * This is deliberately NOT a duplicate generic argument grammar.
 */
typeFamilyEquationArguments
    : genericTypeArguments
    ;
/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/higher-kinded-types.g4
 *
 * Grammar:
 *     HigherKindedTypes
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
 * PURPOSE
 * ============================================================================
 *
 * This grammar provides the narrow source-language syntax required to express
 * higher-kinded type information without creating a second type system,
 * generic-application grammar, type-expression root, or AST hierarchy.
 *
 * A higher-kinded type is a type-level abstraction whose parameter is itself a
 * type constructor rather than only a concrete type.
 *
 * Conceptual examples:
 *
 *     F
 *     F<T>
 *     F<A, B>
 *
 *     F : * -> *
 *     F : * -> * -> *
 *
 * The grammar deliberately separates:
 *
 *     1. kind syntax;
 *     2. higher-kinded parameter metadata;
 *     3. higher-kinded type references;
 *
 * from ordinary generic type application.
 *
 * Ordinary application remains owned by:
 *
 *     grammar/types/generic.g4
 *
 * The complete type-expression composition remains owned by:
 *
 *     grammar/types/types.g4
 *
 * This file therefore provides only the additional HKT-specific structure
 * needed by those canonical owners.
 *
 * ============================================================================
 * ARCHITECTURAL OBJECTIVE
 * ============================================================================
 *
 * The type architecture is:
 *
 *     source
 *       |
 *       v
 *     canonical lexer
 *       |
 *       v
 *     canonical typeExpression
 *       |
 *       +-----------------------------+
 *       |                             |
 *       v                             v
 *     ordinary types             higher-kinded metadata
 *       |                             |
 *       v                             v
 *     TypeExpr                   semantic kind model
 *       |                             |
 *       +-------------+---------------+
 *                     |
 *                     v
 *              semantic type system
 *                     |
 *                     v
 *             canonical semantic model
 *                     |
 *          +----------+----------+
 *          |                     |
 *          v                     v
 *      classical             quantum::ir
 *          |                     |
 *          +----------+----------+
 *                     |
 *                     v
 *              target-independent
 *                    lowering
 *                     |
 *                     v
 *                realization
 *
 * HKT syntax MUST NOT directly select:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     accelerator
 *     node
 *     memory device
 *     physical qubit
 *     topology
 *     routing
 *     scheduling
 *     calibration
 *     QEC
 *     ZQN
 *     HAL
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     higherKindedKind
 *     higherKindedKindAtom
 *     higherKindedKindArrow
 *     higherKindedParameterKind
 *     higherKindedParameterKindClause
 *     higherKindedParameter
 *     higherKindedParameterList
 *     higherKindedTypeReference
 *     higherKindedTypeReferenceBase
 *     higherKindedTypeReferenceArguments
 *
 * These rules describe HKT-specific source structure.
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file MUST NOT own:
 *
 *     typeExpression
 *     typeCore
 *     typePath
 *     identifier
 *     qualifiedName
 *     genericTypeArguments
 *     genericArgumentList
 *     genericParameterList
 *     genericParameter
 *     typeBound
 *     typeBoundList
 *     associatedTypeProjection
 *     dependentType
 *     functionType
 *     tupleType
 *     arrayType
 *     referenceType
 *     pointerType
 *     quantumType
 *     hardwareType
 *     resourceType
 *     capabilityType
 *
 * It also does not own:
 *
 *     type inference
 *     unification
 *     substitution
 *     trait resolution
 *     type-class resolution
 *     specialization
 *     monomorphization
 *     normalization
 *     coherence
 *     proof checking
 *     resource negotiation
 *     capability negotiation
 *     target selection
 *     lowering
 *     routing
 *     scheduling
 *     QEC
 *     ZQN
 *     HAL
 *
 * ============================================================================
 * CRITICAL SINGLE-OWNER RULE
 * ============================================================================
 *
 * Ordinary generic application such as:
 *
 *     Vec<T>
 *     Option<T>
 *     Result<T, E>
 *     Map<K, V>
 *
 * is owned by:
 *
 *     grammar/types/generic.g4
 *
 * This file MUST NOT redefine genericTypeArguments or genericArgumentList.
 *
 * Higher-kinded application may therefore consume the canonical generic
 * application representation rather than creating another angle-bracket
 * grammar.
 *
 * ============================================================================
 * WHY THIS FILE EXISTS
 * ============================================================================
 *
 * Ordinary generic parameters describe parameters ranging over types:
 *
 *     T
 *
 * A higher-kinded parameter describes a parameter ranging over type
 * constructors:
 *
 *     F
 *
 * with a kind such as:
 *
 *     F : *
 *
 *     F : * -> *
 *
 *     F : * -> * -> *
 *
 * The grammar preserves this distinction without embedding semantic
 * interpretation into the parser.
 *
 * ============================================================================
 * KIND MODEL
 * ============================================================================
 *
 * The canonical source-level kind syntax is:
 *
 *     *
 *     * -> *
 *     * -> * -> *
 *     (* -> *) -> *
 *
 * `*` denotes the kind of ordinary fully-applied types.
 *
 * `->` denotes kind-level constructor application.
 *
 * Parentheses are permitted to make nested kind structure explicit.
 *
 * The grammar deliberately does not assign a finite number of kinds.
 *
 * For example:
 *
 *     *
 *
 *     * -> *
 *
 *     * -> * -> *
 *
 *     (* -> *) -> *
 *
 *     (* -> * -> *) -> *
 *
 * are structurally expressible.
 *
 * No:
 *
 *     MAX_KIND_ARITY
 *     MAX_KIND_DEPTH
 *     MAX_TYPE_CONSTRUCTOR_ARITY
 *
 * is defined.
 *
 * ============================================================================
 * KIND SEMANTICS
 * ============================================================================
 *
 * The grammar recognizes kind structure only.
 *
 * Semantic analysis determines:
 *
 *     - whether a kind is well formed;
 *     - whether a constructor has that kind;
 *     - whether a supplied type argument has the required kind;
 *     - whether kind inference succeeds;
 *     - whether kind unification succeeds;
 *     - whether a higher-kinded application is saturated;
 *     - whether partial application is legal;
 *     - whether variance/coherence rules apply;
 *     - whether specialization is legal.
 *
 * None of these are parser decisions.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar MUST NOT introduce:
 *
 *     HktTypeExpr
 *     KindExpr
 *     HigherKindedTypeExpr
 *     HigherKindedTypeIR
 *     QuantumKindExpr
 *     HardwareKindExpr
 *
 * as competing frontend AST hierarchies.
 *
 * Existing source AST support is:
 *
 *     TypeExpr::Hkt
 *
 * and:
 *
 *     TypeExpr::TypeApplication
 *
 * The preferred semantic direction for HKT applications is:
 *
 *     source HKT application
 *          |
 *          v
 *     existing TypeExpr representation
 *          |
 *          v
 *     semantic kind analysis
 *
 * `TypeExpr::Hkt` remains a compatibility representation for the existing
 * single-argument HKT vocabulary.
 *
 * Multi-argument and nested applications SHOULD use the existing
 * `TypeExpr::TypeApplication` representation rather than extending
 * `TypeExpr::Hkt` into a second application hierarchy.
 *
 * ============================================================================
 * KIND AST BOUNDARY
 * ============================================================================
 *
 * The current frontend AST does not expose a dedicated first-class source
 * `KindExpr` node.
 *
 * Therefore the grammar MUST NOT claim that parsed kind annotations are
 * automatically represented by a new Rust AST node.
 *
 * The parser integration layer must choose one of the repository-approved
 * metadata mechanisms:
 *
 *     - generic-parameter metadata;
 *     - declaration metadata;
 *     - semantic-side source metadata;
 *     - an explicitly versioned AST extension.
 *
 * Such an extension is a coordinated AST change and is outside this grammar.
 *
 * Until that AST contract is implemented, this grammar provides the
 * structural parse boundary while semantic integration remains responsible
 * for preserving the kind information.
 *
 * This prevents the grammar from silently losing:
 *
 *     F : * -> *
 *
 * during AST lowering.
 *
 * A production parser MUST reject or report an integration error if a parsed
 * kind annotation cannot be preserved by the active frontend AST contract.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/types/types.g4
 *     grammar/types/generic.g4
 *     grammar/functions/generics.g4
 *     grammar/types/bounds.g4
 *     grammar/types/type-constraints.g4
 *
 * Consumed lexical vocabulary:
 *
 *     IDENTIFIER
 *     LESS
 *     GREATER
 *     COMMA
 *     COLON
 *     THIN_ARROW
 *     STAR
 *     LPAREN
 *     RPAREN
 *
 * This file declares NO lexer rules.
 *
 * ============================================================================
 * TOKEN AUTHORITY
 * ============================================================================
 *
 * All tokens MUST come from the canonical Zamani lexer.
 *
 * In particular, this grammar MUST use:
 *
 *     LESS
 *     GREATER
 *
 * rather than legacy:
 *
 *     LESS_THAN
 *     GREATER_THAN
 *
 * and:
 *
 *     THIN_ARROW
 *
 * rather than introducing a new HKT-specific arrow token.
 *
 * If the canonical lexer uses another already-standardized spelling for the
 * multiplication/star token, this grammar must be adapted to that existing
 * token rather than defining a duplicate lexer rule.
 *
 * ============================================================================
 * KIND ATOM
 * ============================================================================
 *
 * A kind atom is either:
 *
 *     *
 *
 * or an explicitly named kind reference.
 *
 * Named kinds are open-world.
 *
 * Examples:
 *
 *     *
 *     Type
 *     Kind
 *     domain::Kind
 *
 * The grammar does not reserve a closed keyword list for kinds.
 *
 * ============================================================================
 * KIND EXPRESSION
 * ============================================================================
 *
 * A kind expression is right-associative:
 *
 *     A -> B -> C
 *
 * means:
 *
 *     A -> (B -> C)
 *
 * This matches the conventional constructor-kind interpretation and avoids
 * unnecessary ambiguity in semantic kind checking.
 *
 * Parentheses may override grouping:
 *
 *     (A -> B) -> C
 *
 * ============================================================================
 * KIND GRAMMAR
 * ============================================================================
 *
 * The rules below deliberately use recursive structure rather than finite
 * alternatives.
 *
 * Therefore the grammar expresses:
 *
 *     arbitrary finite kind structures
 *
 * without imposing a universal language ceiling.
 *
 * ============================================================================
 * HIGHER-KINDED PARAMETER
 * ============================================================================
 *
 * A higher-kinded parameter is a generic parameter with an explicit kind.
 *
 * Conceptual syntax:
 *
 *     F : * -> *
 *
 *     G : * -> * -> *
 *
 *     H : (* -> *) -> *
 *
 * The generic parameter's ordinary bounds remain owned by the generic/bounds
 * grammar.
 *
 * This file attaches only the optional kind clause.
 *
 * Therefore it MUST NOT redefine:
 *
 *     genericParameter
 *
 * Instead, it provides:
 *
 *     higherKindedParameter
 *
 * as a reusable specialized parameter form.
 *
 * ============================================================================
 * PARAMETER LIST
 * ============================================================================
 *
 * This file provides a specialized HKT parameter-list boundary for grammar
 * components that explicitly need HKT declarations.
 *
 * It does NOT replace the canonical generic parameter list globally.
 *
 * A declaration grammar that wants to expose HKT parameters must explicitly
 * integrate this rule.
 *
 * This prevents every generic declaration from accidentally becoming a
 * second HKT declaration language.
 *
 * ============================================================================
 * HIGHER-KINDED TYPE REFERENCE
 * ============================================================================
 *
 * An HKT reference identifies a type constructor and its canonical generic
 * arguments.
 *
 * Conceptually:
 *
 *     F<T>
 *
 *     F<A, B>
 *
 *     F<Vec<T>>
 *
 * The actual generic argument syntax remains owned by `generic.g4`.
 *
 * This rule therefore delegates to:
 *
 *     genericTypeArguments
 *
 * instead of duplicating:
 *
 *     < ... >
 *
 * syntax.
 *
 * ============================================================================
 * IMPORTANT APPLICATION RULE
 * ============================================================================
 *
 * This file does NOT redefine:
 *
 *     genericTypeArguments
 *
 * or:
 *
 *     genericArgumentList
 *
 * An HKT application is an ordinary type application whose constructor is
 * known, after semantic analysis, to have a higher kind.
 *
 * Thus:
 *
 *     F<T>
 *
 * is syntactically a generic application.
 *
 * Whether `F` is higher-kinded is semantic information.
 *
 * This distinction is essential.
 *
 * ============================================================================
 * PARTIAL APPLICATION
 * ============================================================================
 *
 * A constructor may be partially applied where the semantic type system
 * permits it.
 *
 * Example:
 *
 *     F
 *
 * where:
 *
 *     F : * -> *
 *
 * remains a constructor-level entity.
 *
 * Conversely:
 *
 *     F<T>
 *
 * may be fully saturated or may itself remain higher-kinded depending on the
 * declared kind of `F`.
 *
 * The grammar does not determine saturation.
 *
 * Semantic kind checking does.
 *
 * ============================================================================
 * KIND INFERENCE
 * ============================================================================
 *
 * Explicit kind syntax is supported by this grammar.
 *
 * Kind inference is intentionally not encoded in parser predicates.
 *
 * For source such as:
 *
 *     F
 *
 * the semantic system may infer its kind from:
 *
 *     declaration metadata;
 *     bounds;
 *     applications;
 *     associated types;
 *     constraints;
 *     usage.
 *
 * The parser must not attempt to infer kinds.
 *
 * ============================================================================
 * KIND UNIFICATION
 * ============================================================================
 *
 * Kind unification belongs to semantic analysis.
 *
 * Example:
 *
 *     F : K1 -> K2
 *
 * and:
 *
 *     F<T>
 *
 * require semantic verification that:
 *
 *     kind(T) = K1
 *
 * and that the resulting constructor has kind:
 *
 *     K2
 *
 * The grammar only preserves:
 *
 *     F
 *     T
 *     F<T>
 *
 * ============================================================================
 * TYPE-CLASS / TRAIT INTEGRATION
 * ============================================================================
 *
 * Higher-kinded parameters may participate in trait/type-class constraints.
 *
 * Example conceptual form:
 *
 *     F : * -> *
 *
 *     F extends Functor
 *
 * The exact bound syntax remains owned by:
 *
 *     grammar/types/bounds.g4
 *
 * and:
 *
 *     grammar/types/type-class.g4
 *
 * This file must not redefine trait declarations or type-class references.
 *
 * Semantic analysis determines whether a trait/type-class contract is
 * applicable to a higher-kinded constructor.
 *
 * ============================================================================
 * ASSOCIATED-TYPE INTEGRATION
 * ============================================================================
 *
 * Higher-kinded constructors may participate in associated projections:
 *
 *     F::Item
 *
 *     F<T>::Item
 *
 * The projection syntax remains owned by:
 *
 *     grammar/types/associated.g4
 *
 * This file must not define another `::` projection grammar.
 *
 * ============================================================================
 * DEPENDENT-TYPE INTEGRATION
 * ============================================================================
 *
 * Higher-kinded constructors may appear in dependent types:
 *
 *     F<T>[N]
 *
 * or in other type forms where the canonical dependent grammar permits them.
 *
 * This file does not define:
 *
 *     type-level values;
 *     dependent products;
 *     dependent functions;
 *     type-level arithmetic.
 *
 * Those remain owned by:
 *
 *     grammar/types/dependent.g4
 *
 * ============================================================================
 * TYPE-CLASS KINDING
 * ============================================================================
 *
 * A trait/type-class reference itself may be parameterized by a constructor.
 *
 * Example semantic concept:
 *
 *     Functor<F>
 *
 * where:
 *
 *     F : * -> *
 *
 * This grammar only preserves the source structure.
 *
 * The type-class semantic layer determines:
 *
 *     - whether `Functor` exists;
 *     - its required kind;
 *     - whether `F` satisfies that kind;
 *     - whether implementation coherence holds.
 *
 * ============================================================================
 * GENERIC INTEGRATION
 * ============================================================================
 *
 * `grammar/functions/generics.g4` owns ordinary generic declaration syntax.
 *
 * Therefore this file does not replace:
 *
 *     functionGenericParameters
 *
 * or:
 *
 *     genericParameter
 *
 * A function grammar may integrate HKT parameters by adding an explicit
 * HKT-aware parameter alternative at its composition boundary.
 *
 * That integration must be performed once by the function/declaration owner.
 *
 * The HKT grammar itself remains independent of function declarations.
 *
 * ============================================================================
 * AST INTEGRATION
 * ============================================================================
 *
 * Existing source AST:
 *
 *     TypeExpr::Generic
 *     TypeExpr::TypeApplication
 *     TypeExpr::Hkt
 *     TypeExpr::GenericParameter
 *
 * Recommended lowering:
 *
 *     F<T>
 *         ->
 *     TypeExpr::TypeApplication
 *
 * or, where compatibility requires:
 *
 *     TypeExpr::Hkt
 *
 * for the existing single-argument representation.
 *
 * New multi-argument HKT applications MUST NOT be forced into the existing
 * unary `TypeExpr::Hkt` form.
 *
 * The canonical multi-argument representation is:
 *
 *     TypeExpr::TypeApplication {
 *         constructor,
 *         arguments,
 *     }
 *
 * This preserves arbitrary finite arity and avoids an AST-level capacity
 * restriction.
 *
 * ============================================================================
 * KIND METADATA INTEGRATION
 * ============================================================================
 *
 * Kind annotations require semantic preservation.
 *
 * The integration boundary is:
 *
 *     higherKindedParameter
 *             |
 *             v
 *     generic parameter AST / metadata
 *             |
 *             v
 *     semantic kind environment
 *
 * The semantic kind environment associates a source parameter or constructor
 * with its kind.
 *
 * It must preserve:
 *
 *     source declaration;
 *     source span;
 *     parameter identity;
 *     kind structure;
 *     declaration version;
 *     provenance.
 *
 * The grammar does not create the semantic environment.
 *
 * ============================================================================
 * SEMANTIC KIND MODEL
 * ============================================================================
 *
 * The semantic layer should conceptually distinguish:
 *
 *     TypeKind
 *
 *     ConstructorKind {
 *         inputs: ordered kinds,
 *         output: kind
 *     }
 *
 * or an equivalent open representation.
 *
 * This file deliberately does not prescribe the Rust semantic representation.
 *
 * The semantic model must not encode a finite maximum number of inputs.
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * Parser-level errors include:
 *
 *     malformed kind expression;
 *     missing kind after COLON;
 *     malformed kind arrow;
 *     unmatched kind parentheses;
 *     missing parameter name;
 *     malformed HKT parameter list;
 *     malformed HKT application delegation.
 *
 * Semantic diagnostics include:
 *
 *     unknown kind;
 *     invalid kind application;
 *     kind mismatch;
 *     unsatisfied kind constraint;
 *     invalid partial application;
 *     invalid saturation;
 *     unresolved constructor kind;
 *     incompatible higher-kinded bound;
 *     ambiguous kind;
 *     failed kind inference;
 *     failed kind unification.
 *
 * Parser predicates MUST NOT implement these semantic checks.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Type/kind syntax is effect-free.
 *
 * Parsing:
 *
 *     F : * -> *
 *
 * performs no runtime operation.
 *
 * Kind checking performs no execution effect.
 *
 * Any compile-time computation associated with type-level evaluation remains
 * governed by the metaprogramming/compile-time effect and capability systems.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * HKT syntax does not request physical capabilities.
 *
 * A constructor kind such as:
 *
 *     * -> *
 *
 * does not mean:
 *
 *     CPU capability
 *     GPU capability
 *     quantum capability
 *     hardware capability
 *
 * Capability requirements remain owned by the resource/capability system.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * HKT syntax consumes no physical resource.
 *
 * A type constructor may eventually denote a resource-aware abstraction, but
 * that meaning is semantic.
 *
 * For example:
 *
 *     Resource<T>
 *
 * does not allocate a resource merely because it appears in a type.
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * HKT mechanisms are domain-neutral and may be used to express abstractions
 * over quantum types.
 *
 * Examples:
 *
 *     Q : * -> *
 *
 *     Q<Qubit>
 *
 *     Q<LogicalQubit>
 *
 *     Functor<QuantumState>
 *
 * These remain source-level abstractions.
 *
 * This grammar does not:
 *
 *     allocate qubits;
 *     identify physical qubits;
 *     choose QPUs;
 *     select topology;
 *     route operations;
 *     schedule operations;
 *     choose calibration;
 *     perform QEC;
 *     construct ZQN;
 *     access HAL.
 *
 * The semantic path remains:
 *
 *     HKT source
 *       |
 *       v
 *     TypeExpr
 *       |
 *       v
 *     semantic type/kind model
 *       |
 *       v
 *     quantum semantics
 *       |
 *       v
 *     quantum::ir
 *
 * ============================================================================
 * HDL / HARDWARE CONTRACT
 * ============================================================================
 *
 * HKT abstractions may parameterize hardware-neutral types:
 *
 *     Signal<T>
 *     Buffer<T>
 *     Register<T>
 *     Module<T>
 *
 * The HKT grammar does not encode:
 *
 *     fixed signal width;
 *     fixed register width;
 *     fixed memory;
 *     fixed device count;
 *     fixed topology;
 *     fixed accelerator count.
 *
 * Those remain semantic/resource concerns.
 *
 * ============================================================================
 * AI / DATA CONTRACT
 * ============================================================================
 *
 * HKT abstractions can support generic data/model structures:
 *
 *     F<T>
 *
 *     Model<F<Input>, Output>
 *
 *     Dataset<F<Record>>
 *
 *     Distribution<F<Value>>
 *
 * The grammar remains algorithm-neutral.
 *
 * No machine-learning algorithm is encoded as a keyword.
 *
 * ============================================================================
 * DISTRIBUTED / NETWORK CONTRACT
 * ============================================================================
 *
 * HKT abstractions may parameterize:
 *
 *     Channel<T>
 *     Stream<T>
 *     Future<T>
 *     Replica<T>
 *     Partition<T>
 *
 * No node count, topology, transport or network capacity is encoded here.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * HKT syntax expresses abstraction over types and type constructors.
 *
 * It must remain independent of:
 *
 *     machine size;
 *     hardware vendor;
 *     device identity;
 *     target topology;
 *     physical resource allocation;
 *     deployment size.
 *
 * Consequently:
 *
 *     F : * -> *
 *
 * remains meaningful whether the eventual realization is:
 *
 *     tiny embedded system
 *     CPU
 *     multicore CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     simulator
 *     HPC system
 *     cluster
 *     distributed system
 *     cloud
 *     future execution substrate
 *
 * provided that the semantic program requirements can be satisfied.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar MUST NOT contain:
 *
 *     MAX_HKT_PARAMETERS
 *     MAX_KIND_ARITY
 *     MAX_KIND_DEPTH
 *     MAX_TYPE_CONSTRUCTOR_ARITY
 *     MAX_HKT_APPLICATIONS
 *     MAX_GENERIC_ARGUMENTS
 *     MAX_TYPE_DEPTH
 *
 * Nor any renamed equivalent.
 *
 * Repetition and recursive productions are used for arbitrary finite source
 * structures.
 *
 * Compiler resource limits may exist as explicit:
 *
 *     parsing policy;
 *     validation policy;
 *     semantic-analysis policy;
 *     compilation resource policy.
 *
 * Such operational limits must be:
 *
 *     configurable;
 *     observable;
 *     diagnosable;
 *     independent of target hardware;
 *
 * and MUST NOT become source-language semantic ceilings.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing must depend only on:
 *
 *     source;
 *     canonical lexical configuration;
 *     grammar version;
 *     language version;
 *     dialect configuration.
 *
 * It must not depend on:
 *
 *     time;
 *     randomness;
 *     filesystem state;
 *     network state;
 *     target hardware;
 *     runtime state.
 *
 * Source ordering MUST be preserved.
 *
 * Kind input ordering MUST be preserved.
 *
 * Generic argument ordering MUST be preserved.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * The parser must preserve source spans for:
 *
 *     parameter;
 *     kind annotation;
 *     kind atom;
 *     kind arrow;
 *     HKT application;
 *     constructor;
 *     argument list.
 *
 * Semantic analysis should be able to record:
 *
 *     source declaration;
 *     inferred kind;
 *     explicit kind;
 *     kind unification;
 *     kind constraint;
 *     specialization;
 *     diagnostic provenance.
 *
 * This grammar itself performs no provenance storage outside the normal parse
 * tree/source-span mechanism.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing ordinary generic syntax remains valid.
 *
 * Existing generic parameter syntax remains valid.
 *
 * Existing type-bound syntax remains valid.
 *
 * Existing dependent-type syntax remains valid.
 *
 * Existing associated-type syntax remains valid.
 *
 * Existing type-class/trait syntax remains valid.
 *
 * Existing `TypeExpr::Hkt` compatibility semantics remain available.
 *
 * This file MUST NOT change the meaning of:
 *
 *     Vec<T>
 *     Result<T, E>
 *     Option<T>
 *     T::Item
 *     T: Bound
 *
 * merely because HKT support exists.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * No physical capacity is represented.
 *
 * No machine size is represented.
 *
 * No fixed generic arity is represented.
 *
 * No fixed kind arity is represented.
 *
 * No fixed nesting depth is represented.
 *
 * No quantum resource ceiling is represented.
 *
 * No hardware topology is represented.
 *
 * No target-specific representation is selected.
 *
 * Forbidden universal constants include:
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
 *     MAX_KIND_ARITY
 *     MAX_HKT_ARITY
 *
 * None are defined by this grammar.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive structural cases
 * -------------------------
 *
 *     F : *
 *
 *     F : * -> *
 *
 *     F : * -> * -> *
 *
 *     F : (* -> *) -> *
 *
 *     F : ((* -> *) -> *) -> *
 *
 *     <F : * -> *>
 *
 *     <F : * -> *, G : *>
 *
 *     F<T>
 *
 *     F<A, B>
 *
 *     F<Vec<T>>
 *
 *     F<Map<K, V>>
 *
 *     Functor<F>
 *
 * These examples describe structural possibilities. Exact declaration syntax
 * is admitted only where the integrating declaration grammar exposes the
 * corresponding HKT rule.
 *
 * Negative structural cases
 * -------------------------
 *
 *     F :
 *
 *     F : ->
 *
 *     F : * ->
 *
 *     F : -> *
 *
 *     F : (* -> *
 *
 *     F : * -> *)
 *
 *     F : (* *)
 *
 *     < : * >
 *
 *     <F : >
 *
 *     <F : * -> >
 *
 * Semantic negative cases
 * -----------------------
 *
 *     application of a kind-0 constructor as a constructor;
 *
 *     too few arguments where saturation is required;
 *
 *     too many arguments for a constructor kind;
 *
 *     argument kind mismatch;
 *
 *     incompatible kind bounds;
 *
 *     unresolved constructor kind;
 *
 *     contradictory explicit/inferred kinds;
 *
 *     invalid higher-kinded trait constraint.
 *
 * These semantic cases MUST be tested outside the parser grammar.
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Test combinations of:
 *
 *     HKT + ordinary generics
 *     HKT + traits
 *     HKT + bounds
 *     HKT + associated types
 *     HKT + dependent types
 *     HKT + function types
 *     HKT + tuples
 *     HKT + arrays
 *     HKT + quantum types
 *     HKT + hardware-neutral types
 *     HKT + resource-aware types
 *     HKT + effect-qualified types
 *     HKT + metaprogramming
 *     HKT + type-level values
 *
 * The purpose is to verify that HKT syntax composes with the existing type
 * architecture rather than becoming a parallel language.
 *
 * ============================================================================
 * CROSS-DOMAIN TEST CONTRACT
 * ============================================================================
 *
 * At minimum, the conformance suite should exercise generic constructors
 * representing:
 *
 *     classical data;
 *     tensors;
 *     probabilistic values;
 *     AI models;
 *     quantum states;
 *     quantum registers;
 *     hardware-neutral signals;
 *     distributed streams;
 *     resources;
 *     capabilities.
 *
 * The HKT grammar itself must remain domain-neutral.
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests must use source-defined structures rather than hard-coded maxima.
 *
 * Required classes include:
 *
 *     one kind atom;
 *     nested kind arrows;
 *     many kind inputs;
 *     deeply nested parenthesized kinds;
 *     nested generic applications;
 *     multiple HKT parameters;
 *     multiple HKT arguments;
 *     mixed ordinary/HKT constructors.
 *
 * The test generator may choose sizes dynamically according to the test
 * resource budget.
 *
 * The grammar must not define a largest legal case.
 *
 * ============================================================================
 * INTEGRATION MATRIX
 * ============================================================================
 *
 * This file
 *     |
 *     +--> canonical lexer
 *     |
 *     +--> generic.g4
 *     |
 *     +--> functions/generics.g4
 *     |
 *     +--> bounds.g4
 *     |
 *     +--> type-class.g4
 *     |
 *     +--> associated.g4
 *     |
 *     +--> dependent.g4
 *     |
 *     +--> types.g4
 *     |
 *     v
 * frontend TypeExpr
 *     |
 *     v
 * semantic kind analysis
 *     |
 *     +--> type inference
 *     +--> type substitution
 *     +--> trait/type-class resolution
 *     +--> associated-type resolution
 *     +--> specialization
 *     |
 *     v
 * semantic type model
 *     |
 *     +--> classical
 *     +--> quantum
 *     +--> HDL/hardware
 *     +--> AI/data
 *     +--> distributed
 *     |
 *     v
 * canonical IR
 *     |
 *     +--> classical IR
 *     +--> quantum::ir
 *     |
 *     v
 * target-independent optimization
 *     |
 *     v
 * lowering / routing / scheduling / resilience
 *     |
 *     v
 * target realization
 *
 * ============================================================================
 * IMPORTANT INTEGRATION ACTIONS OUTSIDE THIS FILE
 * ============================================================================
 *
 * Adding this file alone does NOT activate HKT semantics.
 *
 * The following integration work must be performed by their respective owners:
 *
 * 1. `grammar/types/types.g4`
 *
 *    Import this grammar only if the canonical type composition requires the
 *    HKT-specific reference/metadata rules.
 *
 *    Do NOT import it in a way that creates:
 *
 *        Types -> HigherKindedTypes -> Types
 *
 *    circular parser composition.
 *
 * 2. `grammar/functions/generics.g4`
 *
 *    If function generic declarations are to accept explicit kind clauses,
 *    integrate `higherKindedParameter` at the function generic composition
 *    boundary.
 *
 *    Do not duplicate the kind grammar.
 *
 * 3. `grammar/declarations/traits.g4`
 *
 *    Trait declarations may use HKT-aware generic parameters through the
 *    canonical generic/declaration composition.
 *
 *    This file remains the trait declaration owner.
 *
 * 4. `grammar/types/type-class.g4`
 *
 *    Type-class references may consume HKT parameters, but this file remains
 *    only a type-level reference grammar.
 *
 * 5. `grammar/types/bounds.g4`
 *
 *    Bounds may constrain higher-kinded parameters.
 *
 *    Bound syntax remains owned by bounds.g4.
 *
 * 6. `src/frontend/ast/node/types/type_expr.rs`
 *
 *    Existing:
 *
 *        TypeExpr::Hkt
 *        TypeExpr::TypeApplication
 *
 *    must remain the source-level representation.
 *
 *    If explicit kind annotations need persistent AST representation, extend
 *    the canonical generic-parameter representation in a separately reviewed
 *    AST change. Do not hide kind metadata in this grammar.
 *
 * 7. Semantic type analysis
 *
 *    Add the actual kind environment, kind checking, kind inference and kind
 *    unification there.
 *
 * 8. Type inference
 *
 *    HKT constraints must participate in the existing type-inference system
 *    rather than creating a second inference engine.
 *
 * 9. Monomorphization/specialization
 *
 *    Specialization must consume the semantic kind information and must not
 *    infer it again independently.
 *
 * 10. Tests
 *
 *     Add lexical/parser/AST/semantic/conformance tests under the existing
 *     test ownership structure.
 *
 * ============================================================================
 * NO RUNTIME EFFECT
 * ============================================================================
 *
 * This grammar does not:
 *
 *     execute code;
 *     allocate resources;
 *     communicate over networks;
 *     access files;
 *     access devices;
 *     perform quantum measurement;
 *     perform learning;
 *     perform adaptation;
 *     perform reflection;
 *     mutate compiler global state.
 *
 * ============================================================================
 * SAFE RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no embedded target-language actions.
 *
 * The generated parser integration must therefore remain compatible with:
 *
 *     Rust 1.97+
 *     Rust 2021
 *     safe Rust
 *
 * The Rust implementation MUST NOT require:
 *
 *     unsafe;
 *     raw pointer manipulation;
 *     unchecked memory access;
 *     target-specific intrinsics.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [ ] It is the sole HKT-specific grammar delegate.
 * [ ] It contains no lexer rules.
 * [ ] It contains no typeExpression rule.
 * [ ] It contains no genericArgumentList rule.
 * [ ] It contains no genericParameter rule.
 * [ ] It contains no typeBound rule.
 * [ ] It contains no trait declaration grammar.
 * [ ] It contains no associated-type projection grammar.
 * [ ] It contains no dependent-type grammar.
 * [ ] It uses canonical ZamaniLexer tokens.
 * [ ] It uses LESS/GREATER rather than legacy generic delimiters.
 * [ ] It uses THIN_ARROW rather than inventing another arrow token.
 * [ ] Kind expressions are recursively compositional.
 * [ ] Kind arrows are structurally right-associative.
 * [ ] No kind arity ceiling exists.
 * [ ] No HKT arity ceiling exists.
 * [ ] No type nesting ceiling exists.
 * [ ] Ordinary generic application remains owned by generic.g4.
 * [ ] HKT application delegates to genericTypeArguments.
 * [ ] Existing TypeExpr::TypeApplication remains the multi-argument target.
 * [ ] Existing TypeExpr::Hkt remains compatibility-compatible.
 * [ ] Explicit kind metadata is not silently discarded.
 * [ ] Kind semantics remain downstream.
 * [ ] Kind inference remains downstream.
 * [ ] Kind unification remains downstream.
 * [ ] Trait/type-class resolution remains downstream.
 * [ ] Resource resolution remains downstream.
 * [ ] Capability resolution remains downstream.
 * [ ] Quantum realization remains downstream.
 * [ ] Hardware realization remains downstream.
 * [ ] Positive tests exist.
 * [ ] Negative tests exist.
 * [ ] Boundary tests exist.
 * [ ] Scalability tests exist.
 * [ ] Determinism tests exist.
 * [ ] Compatibility tests exist.
 * [ ] Rust 1.97+ integration is verified.
 * [ ] No unsafe Rust is required.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * Higher-kinded types are a TYPE-SYSTEM CAPABILITY.
 *
 * They are not:
 *
 *     a second generic language;
 *     a second type-expression language;
 *     a second AST;
 *     a second IR;
 *     a quantum-specific type system;
 *     a hardware-specific type system;
 *     a runtime mechanism;
 *     a physical resource allocator.
 *
 * The production architecture is:
 *
 *     HKT source structure
 *          |
 *          v
 *     canonical TypeExpr / generic metadata
 *          |
 *          v
 *     semantic kind system
 *          |
 *          v
 *     semantic type system
 *          |
 *          v
 *     canonical IR
 *          |
 *          +------------------+
 *          |                  |
 *          v                  v
 *      classical          quantum::ir
 *          |                  |
 *          +---------+--------+
 *                    |
 *                    v
 *              target-independent
 *                 optimization
 *                    |
 *                    v
 *             lowering / realization
 *
 * This preserves POCO-REAF while allowing higher-kinded abstractions to scale
 * from the smallest program to arbitrarily large programs permitted by the
 * available compiler and execution resources.
 *
 * ============================================================================
 */

parser grammar HigherKindedTypes;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * 1. KIND EXPRESSIONS
 * ============================================================================
 *
 * Examples:
 *
 *     *
 *     * -> *
 *     * -> * -> *
 *     (* -> *) -> *
 *
 * The grammar is right-associative:
 *
 *     A -> B -> C
 *
 * is parsed structurally as:
 *
 *     A -> (B -> C)
 *
 * Parentheses may override that structure.
 */
higherKindedKind
    : higherKindedKindAtom
      (
          THIN_ARROW
          higherKindedKind
      )?
    ;


/*
 * ============================================================================
 * 2. KIND ATOMS
 * ============================================================================
 *
 * `*` is the ordinary type kind.
 *
 * Named kind references remain open-world so future type-system extensions
 * do not require modifying this grammar.
 */
higherKindedKindAtom
    : STAR
    | qualifiedName
    | LPAREN
      higherKindedKind
      RPAREN
    ;


/*
 * ============================================================================
 * 3. EXPLICIT PARAMETER KIND CLAUSE
 * ============================================================================
 *
 * Examples:
 *
 *     F : *
 *
 *     F : * -> *
 *
 *     F : * -> * -> *
 *
 *     F : (* -> *) -> *
 */
higherKindedParameterKindClause
    : COLON
      higherKindedKind
    ;


/*
 * ============================================================================
 * 4. SPECIALIZED HIGHER-KINDED PARAMETER
 * ============================================================================
 *
 * This is intentionally NOT named `genericParameter`.
 *
 * Ordinary generic declaration ownership remains with the canonical generic
 * declaration grammar.
 *
 * A declaration owner can use this specialized rule when explicit HKT syntax
 * is requested.
 *
 * Bounds remain owned by the generic/bounds subsystem and may be integrated
 * at the declaration composition boundary.
 */
higherKindedParameter
    : identifier
      higherKindedParameterKindClause
    ;


/*
 * ============================================================================
 * 5. SPECIALIZED HKT PARAMETER LIST
 * ============================================================================
 *
 * This is not the universal genericParameterList.
 *
 * It is an explicit integration point for declaration grammars that elect to
 * support HKT parameters.
 */
higherKindedParameterList
    : LESS
      higherKindedParameter
      (
          COMMA
          higherKindedParameter
      )*
      COMMA?
      GREATER
    ;


/*
 * ============================================================================
 * 6. HKT REFERENCE
 * ============================================================================
 *
 * The actual angle-bracket application is delegated to Generic.
 *
 * This rule therefore does not duplicate generic argument syntax.
 *
 * Examples:
 *
 *     F<T>
 *     F<A, B>
 *     F<Vec<T>>
 */
higherKindedTypeReference
    : higherKindedTypeReferenceBase
      genericTypeArguments
    ;


/*
 * ============================================================================
 * 7. HKT REFERENCE BASE
 * ============================================================================
 *
 * The constructor itself is a source-level name.
 *
 * Whether it is actually higher-kinded is determined semantically.
 */
higherKindedTypeReferenceBase
    : qualifiedName
    ;


/*
 * ============================================================================
 * 8. HKT REFERENCE ARGUMENT BOUNDARY
 * ============================================================================
 *
 * This rule exists as an explicit integration boundary for consumers that
 * need to identify the generic application portion of an HKT reference.
 *
 * It delegates completely to Generic.
 */
higherKindedTypeReferenceArguments
    : genericTypeArguments
    ;
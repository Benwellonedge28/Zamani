/**
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/type-class.g4
 *
 * Grammar:
 *     TypeClasses
 *
 * Status:
 *     PRODUCTION TYPE-SYSTEM COMPOSITION COMPONENT
 *
 * Language baseline:
 *     Rust 1.97+
 *     Rust 2021
 *     safe Rust only
 *     no unsafe Rust required
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar provides the TYPE-SYSTEM composition boundary for type-class
 * and trait references.
 *
 * Zamani does NOT maintain a second declaration language for type classes.
 *
 * The canonical declaration abstraction is owned by:
 *
 *     grammar/declarations/traits.g4
 *
 * That file owns:
 *
 *     trait declarations
 *     trait members
 *     associated types
 *     associated constants
 *     trait inheritance
 *     implementation declarations
 *     implementation members
 *     declaration-level generic parameters
 *     declaration-level bounds
 *
 * This file is concerned only with the reusable TYPE-LEVEL reference to such
 * a contract.
 *
 * The intended architecture is:
 *
 *     trait declaration
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic type-class interpretation
 *          |
 *          +--> generic constraint solving
 *          +--> associated-type resolution
 *          +--> implementation selection
 *          +--> coherence checking
 *          +--> specialization
 *          +--> capability/resource analysis
 *          |
 *          v
 *     canonical semantic type model
 *
 * ============================================================================
 * IMPORTANT OWNERSHIP RULE
 * ============================================================================
 *
 * This file MUST remain a composition component.
 *
 * It MUST NOT become the owner of:
 *
 *     typeExpression
 *     typeCore
 *     typePath
 *     typePathSegment
 *     typeArguments
 *     genericArgumentList
 *     genericArgument
 *     typeBound
 *     typeBoundList
 *     typeBoundClause
 *     associatedType
 *     associatedTypeProjectionSuffix
 *     traitDeclaration
 *     implementationDeclaration
 *     traitMember
 *     implementationMember
 *
 * Those constructs already have canonical owners elsewhere in the repository.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/lexer/
 *         Canonical lexical vocabulary through ZamaniLexer.
 *
 *     grammar/types/types.g4
 *         Canonical type-expression orchestration.
 *
 *     grammar/types/generic.g4
 *         Canonical generic type application.
 *
 *     grammar/types/bounds.g4
 *         Canonical type-bound representation.
 *
 *     grammar/types/associated.g4
 *         Canonical associated-type projection.
 *
 *     grammar/declarations/traits.g4
 *         Canonical trait/type-class declaration semantics.
 *
 * This file deliberately does NOT import or consume a complete
 * `typeExpression` in its own public rules. That prevents a dependency cycle:
 *
 *     types.g4
 *          |
 *          v
 *     type-class.g4
 *          |
 *          X
 *     types.g4
 *
 * ============================================================================
 * EXPORTS
 * ============================================================================
 *
 * Public reusable rules:
 *
 *     typeClassReference
 *     typeClassReferenceArguments
 *     typeClassReferenceArgumentList
 *     typeClassReferenceArgument
 *     typeClassReferenceList
 *
 * These rules describe references, not declarations or semantic satisfaction.
 *
 * ============================================================================
 * CONSUMERS
 * ============================================================================
 *
 * The exported rules may be consumed by:
 *
 *     grammar/types/bounds.g4
 *     grammar/types/type-constraints.g4
 *     grammar/types/generic.g4
 *     grammar/declarations/traits.g4
 *     grammar/declarations/implementations.g4
 *     grammar/validation/
 *     grammar/expressions/
 *     grammar/statements/
 *     domain-specific semantic adapters
 *
 * A consumer MUST NOT redefine these rules merely to specialize them.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar introduces NO new AST hierarchy.
 *
 * A parsed type-class reference must ultimately map into the existing
 * domain-neutral type representation.
 *
 * Depending on semantic context, the frontend may represent a reference as:
 *
 *     TypeExpr::Identifier
 *
 * or:
 *
 *     TypeExpr::Generic
 *
 * or another existing canonical type representation.
 *
 * The semantic type system is responsible for determining whether the
 * referenced declaration denotes:
 *
 *     a trait
 *     an interface
 *     a type-class contract
 *     another type-level abstraction
 *
 * This grammar must never introduce:
 *
 *     TypeClassExpr
 *     UniversalTypeClassExpr
 *     QuantumTypeClassExpr
 *     HardwareTypeClassExpr
 *     AITypeClassExpr
 *     TypeClassIR
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing answers:
 *
 *     "What type-class reference was written?"
 *
 * Semantic analysis answers:
 *
 *     "What declaration does this reference denote?"
 *     "Are its arguments valid?"
 *     "Is the constraint satisfied?"
 *     "Which implementation satisfies it?"
 *     "Are the implementations coherent?"
 *     "Can specialization occur?"
 *     "What associated types follow from it?"
 *
 * None of those decisions belong in this grammar.
 *
 * ============================================================================
 * GENERIC CONTRACT
 * ============================================================================
 *
 * Generic application syntax remains owned by the canonical type grammar.
 *
 * This file intentionally mirrors the repository's canonical generic
 * application shape only at the type-class-reference boundary.
 *
 * It MUST NOT create another:
 *
 *     genericArguments
 *     genericArgumentList
 *     genericArgument
 *
 * implementation.
 *
 * ============================================================================
 * ASSOCIATED-TYPE CONTRACT
 * ============================================================================
 *
 * Associated-type projection remains owned by:
 *
 *     grammar/types/associated.g4
 *
 * Therefore this file MUST NOT define:
 *
 *     T::Item
 *     Iterator::Item
 *     Collection<T>::Element
 *
 * as another independent projection grammar.
 *
 * When a type-class reference participates in an associated-type projection,
 * the canonical associated-type grammar consumes the appropriate type
 * expression/reference.
 *
 * ============================================================================
 * BOUNDS CONTRACT
 * ============================================================================
 *
 * Type bounds remain owned by:
 *
 *     grammar/types/bounds.g4
 *
 * A source construct such as:
 *
 *     T: Numeric
 *
 * is a TYPE BOUND.
 *
 * The left-hand type and the bound composition therefore belong to the
 * canonical bound machinery.
 *
 * `typeClassReference` supplies the reusable representation of the referenced
 * contract when the semantic system identifies that bound as a type-class or
 * trait requirement.
 *
 * This prevents two competing definitions of:
 *
 *     typeBound
 *     typeBoundList
 *     typeBoundClause
 *
 * ============================================================================
 * OPEN-WORLD CONTRACT
 * ============================================================================
 *
 * Type classes are NOT an enumerated built-in catalogue.
 *
 * This grammar MUST NOT contain rules such as:
 *
 *     : NUMERIC
 *     | COMPARABLE
 *     | ITERABLE
 *     | SERIALIZABLE
 *     | QUANTUM
 *     | TENSOR
 *     | HARDWARE
 *
 * New contracts are represented by ordinary qualified names.
 *
 * Examples:
 *
 *     Numeric
 *     Comparable
 *     Iterable
 *     data::Serializable
 *     quantum::Observable
 *     hardware::Signal
 *     tensor::Computable
 *     future::computing::Property
 *
 * Adding a new type class must therefore NOT require changing this grammar.
 *
 * This is essential for POCO-REAF and long-term language evolution.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar contains no language-level capacity constants.
 *
 * It does NOT limit:
 *
 *     number of type classes
 *     number of references
 *     number of generic parameters
 *     number of generic arguments
 *     number of bounds
 *     number of implementations
 *     number of associated types
 *     inheritance depth
 *     type-expression depth
 *     module count
 *     source size
 *     quantum resources
 *     CPU resources
 *     GPU resources
 *     FPGA resources
 *     accelerator resources
 *     memory
 *     tensor rank
 *     network size
 *     distributed-node count
 *
 * The grammar uses repetition operators rather than finite enumerations.
 *
 * Runtime/compiler resource protection, if required, belongs to tooling or
 * compiler configuration and MUST NOT become part of the language semantics.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * A type-class reference is target-independent.
 *
 * This grammar MUST NOT know:
 *
 *     CPU count
 *     GPU count
 *     FPGA count
 *     ASIC structure
 *     accelerator topology
 *     QPU topology
 *     qubit count
 *     physical memory
 *     register width
 *     tensor hardware
 *     network topology
 *     scheduling policy
 *     routing policy
 *     calibration
 *     error-correction configuration
 *     physical placement
 *
 * Those concerns are resolved after semantic analysis.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum contracts are ordinary type-level contracts.
 *
 * For example, a semantic environment may define:
 *
 *     quantum::Observable
 *     quantum::Unitary
 *     quantum::Parameterized
 *
 * This grammar does not need to know those names.
 *
 * A type-class reference can therefore participate in quantum source programs
 * without coupling the grammar to:
 *
 *     physical qubits
 *     gate sets
 *     coupling maps
 *     calibration
 *     routing
 *     scheduling
 *     QEC
 *     ZQN
 *     HAL
 *
 * The canonical quantum pipeline remains:
 *
 *     source
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     semantic model
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     decomposition
 *       |
 *       v
 *     routing
 *       |
 *       v
 *     scheduling
 *       |
 *       v
 *     resilience / QEC
 *       |
 *       v
 *     ZQN
 *       |
 *       v
 *     HAL
 *       |
 *       v
 *     target realization
 *
 * ============================================================================
 * CLASSICAL / HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * The same open-world reference mechanism applies to:
 *
 *     classical contracts
 *     numerical contracts
 *     tensor contracts
 *     HDL contracts
 *     hardware contracts
 *     embedded contracts
 *     distributed contracts
 *     networking contracts
 *     AI/model contracts
 *     interoperability contracts
 *
 * Domain semantics are registered downstream.
 *
 * The grammar remains unchanged when a new domain is introduced.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing a type-class reference has no execution effect.
 *
 * It MUST NOT:
 *
 *     execute code
 *     perform IO
 *     inspect hardware
 *     query resources
 *     access a network
 *     invoke foreign functions
 *     perform learning
 *     perform adaptation
 *     perform reflection
 *
 * Semantic analysis may later discover effects associated with an
 * implementation or operation constrained by the type class.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * A type-class reference does not itself select a physical capability.
 *
 * Semantic analysis may derive capability requirements from:
 *
 *     trait semantics
 *     associated operations
 *     implementation metadata
 *     domain contracts
 *
 * Those requirements flow through the existing capability system.
 *
 * The flow is:
 *
 *     type-class reference
 *          |
 *          v
 *     semantic resolution
 *          |
 *          v
 *     capability requirements
 *          |
 *          v
 *     resource/capability negotiation
 *          |
 *          v
 *     execution planning
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * No resource requirement is resolved by this grammar.
 *
 * A type class may semantically imply a requirement such as:
 *
 *     capability("tensor.compute")
 *
 * or:
 *
 *     capability("quantum.measurement")
 *
 * but that information belongs to semantic/resource metadata.
 *
 * This grammar remains independent of physical capacity.
 *
 * ============================================================================
 * CONTRACT / POLICY CONTRACT
 * ============================================================================
 *
 * A type-class reference may participate in:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *     policy
 *
 * constructs through their canonical owners.
 *
 * This file must not redefine those constructs.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * The parser must preserve source spans through the normal parser/AST
 * pipeline.
 *
 * Provenance of:
 *
 *     declaration resolution
 *     implementation selection
 *     specialization
 *     associated-type normalization
 *
 * is a semantic/compiler concern.
 *
 * This file introduces no independent provenance representation.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file has NO direct backend IR ownership.
 *
 * Type-class references are resolved into the canonical semantic type model.
 *
 * From there they may influence:
 *
 *     generic specialization
 *     overload resolution
 *     operation legality
 *     capability requirements
 *     resource requirements
 *     effect validation
 *     contract validation
 *     quantum semantic validation
 *     HDL semantic validation
 *
 * They must not directly emit:
 *
 *     LLVM IR
 *     machine code
 *     HDL netlists
 *     QASM
 *     QIR
 *     vendor-specific IR
 *     physical qubit maps
 *     device-specific schedules
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * This grammar is deterministic with respect to:
 *
 *     token stream
 *     selected language version
 *     selected compatibility mode
 *
 * It must not depend on:
 *
 *     hardware
 *     current time
 *     random state
 *     filesystem state
 *     network state
 *     resource availability
 *     target selection
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * `typeClassReference` is an additive/open-world abstraction.
 *
 * Existing trait names and qualified names remain valid identifiers.
 *
 * No new reserved keyword is required for a type class.
 *
 * Therefore existing programs do not become invalid merely because new
 * type-class declarations are introduced into libraries.
 *
 * Deprecated or renamed declarations are handled by:
 *
 *     semantic compatibility
 *     module/version metadata
 *     migration tooling
 *
 * and not by changing this grammar.
 *
 * ============================================================================
 * DIAGNOSTICS CONTRACT
 * ============================================================================
 *
 * Parser diagnostics for this file are limited to structural errors, such as:
 *
 *     missing type-class name
 *     malformed qualification
 *     malformed generic argument list
 *     missing closing generic delimiter
 *     malformed separator
 *
 * Semantic diagnostics belong downstream, including:
 *
 *     unknown type class
 *     invalid type-class arguments
 *     wrong generic arity
 *     unsatisfied constraint
 *     conflicting implementations
 *     ambiguous implementation
 *     invalid associated type
 *     invalid specialization
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive examples:
 *
 *     Numeric
 *     Comparable
 *     data::Serializable
 *     quantum::Observable
 *     hardware::Signal
 *     tensor::Computable
 *
 * Generic references:
 *
 *     Collection<T>
 *     Iterable<Item>
 *     Relation<A, B>
 *
 * Qualified generic references:
 *
 *     data::Collection<T>
 *     quantum::Operator<Observable>
 *     hardware::Signal<Scalar>
 *
 * Negative examples:
 *
 *     <Numeric
 *     Numeric>
 *     data::
 *     ::Numeric
 *     Numeric<
 *     Numeric<T
 *
 * The semantic suite must separately test:
 *
 *     unknown references
 *     invalid arguments
 *     unsatisfied constraints
 *     ambiguous implementations
 *     coherence failures
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Test combinations with:
 *
 *     generic parameters
 *     associated types
 *     dependent types
 *     linear types
 *     affine types
 *     quantum types
 *     hardware types
 *     resource types
 *     capability types
 *     effect-qualified types
 *     contracts
 *     policies
 *
 * Parser tests must verify that domain names remain open-ended identifiers.
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests must use generated source rather than hard-coded finite ceilings.
 *
 * The test generator should vary:
 *
 *     qualification depth
 *     generic nesting
 *     argument count
 *     source size
 *
 * within available test resources.
 *
 * No test may establish a language maximum merely because the test machine
 * has finite memory or stack capacity.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [x] It has exactly one grammar implementation.
 * [x] It contains no embedded Rust actions.
 * [x] It contains no unsafe implementation requirement.
 * [x] It has no semantic predicates.
 * [x] It owns no declaration syntax.
 * [x] It owns no complete type-expression syntax.
 * [x] It owns no generic-argument implementation.
 * [x] It owns no associated-type projection implementation.
 * [x] It owns no type-bound implementation.
 * [x] It contains no physical resource limits.
 * [x] It contains no target-specific assumptions.
 * [x] It is open-world.
 * [x] It can represent arbitrarily named type-class contracts.
 * [x] It uses the canonical lexical vocabulary.
 * [x] It can be composed with the existing type grammar without a
 *     type-expression dependency cycle.
 * [x] It has a defined AST/semantic/IR boundary.
 * [x] It has explicit integration contracts.
 *
 * ============================================================================
 */

parser grammar TypeClasses;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * TYPE-CLASS REFERENCE
 * ============================================================================
 *
 * A type-class reference is an open-world qualified type name with optional
 * canonical type arguments.
 *
 * Examples:
 *
 *     Numeric
 *     Comparable
 *     data::Serializable
 *     quantum::Observable
 *     Collection<T>
 *     data::Collection<T>
 *
 * `typePath` is owned by the canonical type grammar.
 *
 * `typeClassReferenceArguments` is intentionally a narrow composition rule
 * using the canonical generic argument representation.
 *
 * Semantic analysis determines whether the referenced declaration is actually
 * a type class/trait and whether its arguments are valid.
 */

typeClassReference
    : typePath
      typeClassReferenceArguments?
    ;


/* ============================================================================
 * TYPE-CLASS REFERENCE ARGUMENTS
 * ============================================================================
 *
 * This is the type-class-reference-specific wrapper around the canonical
 * generic argument list.
 *
 * The underlying argument syntax remains type-system syntax.
 *
 * No finite arity is imposed.
 */

typeClassReferenceArguments
    : LESS_THAN
      typeClassReferenceArgumentList
      GREATER_THAN
    ;


typeClassReferenceArgumentList
    : typeClassReferenceArgument
      (COMMA typeClassReferenceArgument)*
      COMMA?
    ;


typeClassReferenceArgument
    : typeExpression
    ;


/* ============================================================================
 * TYPE-CLASS REFERENCE LIST
 * ============================================================================
 *
 * Provides a reusable list for grammar consumers that need several
 * type-class references without creating another constraint language.
 *
 * Examples:
 *
 *     Numeric, Comparable
 *
 *     data::Serializable, data::Hashable
 *
 * The meaning of combination is determined by the consuming construct.
 *
 * This grammar does not assume whether the combination means:
 *
 *     conjunction
 *     disjunction
 *     alternatives
 *     overload candidates
 *     implementation candidates
 *
 * Semantic analysis determines that from context.
 */

typeClassReferenceList
    : typeClassReference
      (COMMA typeClassReference)*
      COMMA?
    ;


/* ============================================================================
 * INTEGRATION NOTE
 * ============================================================================
 *
 * Type-class constraints themselves belong to the canonical bound/constraint
 * machinery.
 *
 * Conceptually:
 *
 *     T: Numeric
 *
 * is represented by:
 *
 *     left-hand type expression
 *          +
 *     canonical type-bound
 *          +
 *     type-class reference semantics
 *
 * Therefore this file intentionally does NOT define:
 *
 *     typeClassConstraint
 *     typeClassConstraintList
 *     typeBound
 *     typeBoundList
 *     typeBoundClause
 *
 * The canonical owner remains:
 *
 *     grammar/types/bounds.g4
 *
 * ============================================================================
 */


/* ============================================================================
 * ASSOCIATED-TYPE INTEGRATION NOTE
 * ============================================================================
 *
 * Associated projections such as:
 *
 *     T::Item
 *     Iterator::Item
 *     Collection<T>::Element
 *
 * belong to:
 *
 *     grammar/types/associated.g4
 *
 * This file intentionally does NOT redefine those productions.
 *
 * A type-class reference can be used as the base of such a projection through
 * the canonical type-expression system.
 *
 * ============================================================================
 */


/* ============================================================================
 * GENERIC INTEGRATION NOTE
 * ============================================================================
 *
 * Generic declarations and generic application remain owned by:
 *
 *     grammar/types/generic.g4
 *
 * and the canonical type orchestrator:
 *
 *     grammar/types/types.g4
 *
 * This file only provides a type-class-reference boundary.
 *
 * ============================================================================
 */


/* ============================================================================
 * DECLARATION INTEGRATION NOTE
 * ============================================================================
 *
 * Trait declarations and implementation declarations remain owned by:
 *
 *     grammar/declarations/traits.g4
 *
 * This file does not introduce:
 *
 *     typeclassDeclaration
 *     typeClassBody
 *     typeClassMember
 *     implementationBody
 *
 * A declaration becomes a type-class contract through semantic interpretation.
 *
 * ============================================================================
 */
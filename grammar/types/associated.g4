/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/associated.g4
 *
 * Grammar:
 *     Associated
 *
 * Status:
 *     CANONICAL SOURCE-LEVEL ASSOCIATED-TYPE / TYPE-PROJECTION DELEGATE
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Rust edition:
 *     2021
 *
 * Safety:
 *     This grammar contains no Rust actions and imposes no unsafe-Rust
 *     requirement on the compiler implementation.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines the reusable source-level grammar for associated-type
 * projections.
 *
 * An associated type is a type selected from another type, generic parameter,
 * interface/trait-like abstraction, or other type-level entity.
 *
 * Conceptually:
 *
 *     Base::Member
 *
 * Examples:
 *
 *     Iterator::Item
 *     Collection::Element
 *     T::Output
 *     T::Error
 *     quantum::Circuit::State
 *     hardware::Module::Signal
 *
 * The grammar preserves source structure only.
 *
 * It does NOT decide whether the final member is:
 *
 *     - an associated type;
 *     - a nested type;
 *     - a namespace-qualified type;
 *     - a module member;
 *     - a type alias;
 *     - another language-level type entity.
 *
 * That distinction is resolved by semantic name/type resolution.
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
 *     Zamani parser
 *          |
 *          v
 *     canonical TypeExpr
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     name/type resolution
 *          |
 *          +------------------------------+
 *          |                              |
 *          v                              v
 *     ordinary qualified type       associated projection
 *                                      |
 *                                      v
 *                              TypeExpr::Associated
 *                                      |
 *                                      v
 *                              semantic type model
 *                                      |
 *                                      v
 *                                canonical IR
 *                                      |
 *                   +------------------+------------------+
 *                   |                  |                  |
 *                   v                  v                  v
 *              classical         quantum::ir       HDL/hardware
 *                   |                  |                  |
 *                   +------------------+------------------+
 *                                      |
 *                                      v
 *                              lowering/optimization
 *                                      |
 *                              target realization
 *
 * This file therefore remains entirely above target-specific compilation.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - the reusable syntax for an associated-type projection;
 *     - the separator between a projection base and projected member;
 *     - the projected member identifier;
 *     - source ordering of projection components;
 *     - the parser-level projection shape consumed by type composition.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer rules;
 *     - identifiers;
 *     - `DOUBLE_COLON` token definition;
 *     - ordinary type expressions;
 *     - generic type application;
 *     - generic declarations;
 *     - generic parameter declarations;
 *     - type bounds;
 *     - where clauses;
 *     - trait/interface declarations;
 *     - type aliases;
 *     - name resolution;
 *     - overload resolution;
 *     - trait/interface satisfaction;
 *     - associated-type lookup;
 *     - associated-type equality solving;
 *     - type inference;
 *     - generic substitution;
 *     - specialization;
 *     - monomorphization;
 *     - type-level evaluation;
 *     - resource discovery;
 *     - capability negotiation;
 *     - hardware discovery;
 *     - quantum allocation;
 *     - physical qubit mapping;
 *     - routing;
 *     - scheduling;
 *     - calibration;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution.
 *
 * ============================================================================
 * EXISTING AST CONTRACT
 * ============================================================================
 *
 * The repository already provides the canonical source-level representation:
 *
 *     src/frontend/ast/node/types/type_expr.rs
 *
 * Its TypeExpr vocabulary already contains:
 *
 *     TypeExpr::Associated {
 *         base: Box<TypeExpr>,
 *         member: TypeName,
 *     }
 *
 * This grammar MUST lower to that existing representation.
 *
 * It MUST NOT introduce:
 *
 *     AssociatedTypeExpr
 *     AssociatedTypeAst
 *     TypeProjectionAst
 *     AssociatedTypeIR
 *     TypeProjectionIR
 *
 * as competing representations.
 *
 * The canonical AST remains TypeExpr.
 *
 * ============================================================================
 * IMPORTANT: QUALIFIED-NAME AMBIGUITY
 * ============================================================================
 *
 * Zamani already supports ordinary qualified type paths such as:
 *
 *     module::Type
 *     quantum::State
 *     hardware::Signal
 *
 * An associated projection can have the same surface shape:
 *
 *     T::Item
 *     Iterator::Item
 *
 * Consequently, parsing `::` alone MUST NOT force the final segment to be an
 * associated type.
 *
 * The semantic resolver determines whether:
 *
 *     A::B
 *
 * denotes:
 *
 *     1. an ordinary qualified type path,
 *     2. a type/member namespace path,
 *     3. an associated-type projection,
 *     4. another valid type-level entity.
 *
 * This is essential for backwards compatibility and for an open-world
 * language architecture.
 *
 * The parser must preserve sufficient structure for the semantic layer to
 * make that determination without requiring a closed list of type names.
 *
 * ============================================================================
 * CANONICAL TYPE-EXPRESSION AUTHORITY
 * ============================================================================
 *
 * Ordinary type syntax remains owned by:
 *
 *     grammar/types/types.g4
 *
 * This file MUST consume the canonical:
 *
 *     typeExpression
 *
 * rule.
 *
 * It MUST NOT redefine:
 *
 *     typeExpression
 *     typeCore
 *     namedType
 *     typePath
 *     typePathSegment
 *     genericType
 *     tupleType
 *     arrayType
 *     sliceType
 *     functionType
 *     referenceType
 *     pointerType
 *     dependentType
 *     quantumType
 *     temporalType
 *
 * ============================================================================
 * GENERIC APPLICATION AUTHORITY
 * ============================================================================
 *
 * Generic type application remains owned by:
 *
 *     grammar/types/generic.g4
 *
 * Examples:
 *
 *     Iterator<T>::Item
 *     Collection<T>::Element
 *     Container<Key, Value>::Entry
 *
 * are semantically meaningful combinations of:
 *
 *     generic type application
 *
 * followed by:
 *
 *     associated/member projection.
 *
 * This file does not duplicate generic argument syntax.
 *
 * ============================================================================
 * GENERIC PARAMETER AUTHORITY
 * ============================================================================
 *
 * Generic declarations remain owned by their existing declaration/function
 * grammars.
 *
 * Examples:
 *
 *     <T>
 *     <T extends Iterable>
 *
 * remain outside this file.
 *
 * This file merely permits the canonical type expression used as a projection
 * base to be a generic parameter or any other valid source-level type.
 *
 * ============================================================================
 * TYPE-CONSTRAINT INTEGRATION
 * ============================================================================
 *
 * Generic/type constraints remain owned by:
 *
 *     grammar/types/type-constraints.g4
 *
 * For example:
 *
 *     T: Iterator
 *
 *     T: Iterable
 *
 * does not belong here.
 *
 * Associated-type equality or capability constraints may consume the
 * projection produced by this grammar downstream.
 *
 * Examples of semantic relationships include:
 *
 *     T::Item = U
 *     T::Output: Serializable
 *
 * Such constraint semantics do not belong in this parser delegate.
 *
 * ============================================================================
 * BASIC PROJECTION
 * ============================================================================
 *
 * Canonical conceptual form:
 *
 *     Base::Member
 *
 * where:
 *
 *     Base
 *
 * is a canonical source-level type expression and:
 *
 *     Member
 *
 * is an ordinary source identifier representing the projected member.
 *
 * The grammar deliberately does not reserve names such as:
 *
 *     Item
 *     Output
 *     Error
 *     Element
 *     State
 *
 * because associated-type names are open-world and user-defined.
 *
 * ============================================================================
 * PUBLIC RULE
 * ============================================================================
 *
 * `associatedType` is the public reusable rule.
 *
 * It is intentionally a suffix/projection form rather than an independent
 * type universe.
 *
 * The surrounding type-composition grammar determines where it is admitted.
 *
 * ============================================================================
 */

parser grammar Associated;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. ASSOCIATED TYPE PROJECTION
 * ============================================================================
 *
 * Canonical conceptual syntax:
 *
 *     Base::Member
 *
 * The base is an ordinary canonical type expression.
 *
 * The member is an identifier.
 *
 * IMPORTANT:
 *
 * Because canonical `typeExpression` may itself contain qualified names,
 * generic applications, projections, postfixes, and other recursive forms,
 * the parser composition layer must avoid introducing a second recursive type
 * system through this rule.
 *
 * The rule therefore represents one projection step.
 *
 * Semantic analysis may subsequently construct:
 *
 *     TypeExpr::Associated {
 *         base,
 *         member,
 *     }
 *
 * ============================================================================
 */

associatedType
    : associatedTypeBase
      DOUBLE_COLON
      associatedTypeMember
    ;


/*
 * ============================================================================
 * 2. PROJECTION BASE
 * ============================================================================
 *
 * The base must be a canonical type expression.
 *
 * This permits associated projections from:
 *
 *     named types
 *     qualified types
 *     generic applications
 *     generic parameters
 *     tuples
 *     references
 *     pointers
 *     dependent types
 *     quantum types
 *     hybrid types
 *     hardware types
 *     future domain types
 *
 * without this file knowing those domains.
 *
 * The exact parser-composition placement of this rule is controlled by the
 * canonical type-expression grammar.
 *
 * ============================================================================
 */

associatedTypeBase
    : typeExpression
    ;


/*
 * ============================================================================
 * 3. PROJECTED MEMBER
 * ============================================================================
 *
 * The projected member is deliberately an IDENTIFIER.
 *
 * No fixed associated-type catalogue is permitted.
 *
 * Valid examples include:
 *
 *     Item
 *     Output
 *     Error
 *     Element
 *     State
 *     Measurement
 *     Signal
 *     Memory
 *     Accelerator
 *     CustomFutureType
 *
 * These names acquire meaning only through semantic resolution.
 *
 * ============================================================================
 */

associatedTypeMember
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * 4. PROJECTION CHAIN
 * ============================================================================
 *
 * A semantic type may contain multiple projection steps.
 *
 * Conceptually:
 *
 *     A::B::C
 *
 * can represent:
 *
 *     (A::B)::C
 *
 * when semantic resolution establishes the first projection.
 *
 * This file intentionally does not recursively redefine `typeExpression`.
 *
 * A complete projection chain is therefore composed by the canonical type
 * grammar/parser rather than by introducing a competing recursive type
 * grammar here.
 *
 * ============================================================================
 */

associatedTypeProjectionSuffix
    : DOUBLE_COLON
      associatedTypeMember
    ;


/*
 * ============================================================================
 * 5. SOURCE-LEVEL PROJECTION SHAPE
 * ============================================================================
 *
 * The parser preserves the following conceptual structure:
 *
 *     associatedType
 *         |
 *         +-- associatedTypeBase
 *         |
 *         +-- DOUBLE_COLON
 *         |
 *         +-- associatedTypeMember
 *
 * The semantic lowering layer may construct:
 *
 *     TypeExpr::Associated {
 *         base: Box<TypeExpr>,
 *         member: TypeName,
 *     }
 *
 * The parser does not perform that construction itself.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Input:
 *
 *     Base::Member
 *
 * Structural representation:
 *
 *     base
 *     member
 *
 * Canonical frontend representation:
 *
 *     TypeExpr::Associated {
 *         base: Box<TypeExpr>,
 *         member: TypeName,
 *     }
 *
 * The member's source spelling must be preserved.
 *
 * The base must retain its complete source structure.
 *
 * No resolved symbol ID belongs in TypeExpr.
 *
 * No semantic type belongs in TypeExpr.
 *
 * No hardware identity belongs in TypeExpr.
 *
 * No resource allocation belongs in TypeExpr.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     - whether the base type exists;
 *     - whether the projected member exists;
 *     - whether the member is an associated type;
 *     - which declaration owns the associated type;
 *     - whether the projection is legal for the base;
 *     - whether required generic parameters are available;
 *     - whether associated-type constraints are satisfied;
 *     - whether associated-type equality constraints are satisfied;
 *     - whether the resulting type is well formed.
 *
 * The grammar performs none of these operations.
 *
 * Example:
 *
 *     T::Item
 *
 * is syntactically valid if:
 *
 *     T
 *
 * is a valid source-level type expression and:
 *
 *     Item
 *
 * is a valid identifier.
 *
 * Whether `T` actually has an associated type named `Item` is semantic.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * OPEN-WORLD TYPE MODEL
 * ============================================================================
 *
 * The grammar MUST NOT enumerate associated types.
 *
 * It must not contain alternatives such as:
 *
 *     ITEM
 *     OUTPUT
 *     ERROR
 *     ELEMENT
 *     VALUE
 *     STATE
 *
 * because future domains may define arbitrary associated types.
 *
 * For example, all of the following remain extensible:
 *
 *     quantum::Circuit::State
 *     quantum::Circuit::Measurement
 *     hardware::Module::Signal
 *     hardware::Module::Clock
 *     tensor::Tensor::Element
 *     tensor::Tensor::Shape
 *     distributed::Node::Message
 *     future::Domain::Result
 *
 * No grammar modification is required merely because a new associated type
 * is introduced.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * GENERIC INTEGRATION
 * ============================================================================
 *
 * Associated projections may be applied to generic types:
 *
 *     Iterator<T>::Item
 *     Container<T>::Element
 *     Map<K, V>::Entry
 *
 * The generic argument structure remains owned by:
 *
 *     grammar/types/generic.g4
 *
 * This file only owns the projection component.
 *
 * The semantic pipeline is:
 *
 *     generic source type
 *          |
 *          v
 *     generic TypeExpr
 *          |
 *          v
 *     associated projection
 *          |
 *          v
 *     semantic associated-type resolution
 *
 * Generic substitution is performed downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * GENERIC PARAMETER INTEGRATION
 * ============================================================================
 *
 * A generic parameter can be a projection base:
 *
 *     T::Item
 *     T::Output
 *     T::Error
 *
 * This does not require `T` to be a built-in type.
 *
 * The semantic layer determines whether the declaration introducing `T`
 * establishes the required associated-type contract.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * ASSOCIATED-TYPE EQUALITY
 * ============================================================================
 *
 * The grammar itself does not solve equality.
 *
 * A semantic system may establish relationships such as:
 *
 *     T::Item = U
 *
 *     T::Output = Result
 *
 *     T::Error = E
 *
 *     T::Element: Serializable
 *
 * The parser only supplies the type expression required by the surrounding
 * constraint grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * TYPE-LEVEL METAPROGRAMMING INTEGRATION
 * ============================================================================
 *
 * The repository's type-level metaprogramming system already recognizes
 * associated-type concepts.
 *
 * This grammar MUST remain compatible with:
 *
 *     grammar/metaprogramming/type-level.g4
 *
 * Type-level operations may consume:
 *
 *     TypeExpr::Associated
 *
 * but must not create a second associated-type representation.
 *
 * A type-level computation may therefore:
 *
 *     inspect
 *     normalize
 *     compare
 *     substitute
 *     specialize
 *
 * associated projections only after normal semantic validation.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * REFLECTION INTEGRATION
 * ============================================================================
 *
 * Reflection may expose metadata about an associated type.
 *
 * Reflection remains owned by:
 *
 *     grammar/metaprogramming/reflection.g4
 *
 * This file does not define reflection syntax.
 *
 * Example semantic flow:
 *
 *     source projection
 *          |
 *          v
 *     TypeExpr::Associated
 *          |
 *          v
 *     semantic resolution
 *          |
 *          v
 *     reflection metadata
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * DEPENDENT TYPE INTEGRATION
 * ============================================================================
 *
 * An associated type may participate in dependent types.
 *
 * Examples:
 *
 *     Buffer<T::Item>
 *
 *     Array<T::Element; N>
 *
 *     Tensor<T::Scalar, Shape>
 *
 * The dependent/value syntax remains owned by:
 *
 *     grammar/types/dependent.g4
 *     grammar/types/types.g4
 *
 * This file does not duplicate dependent-type syntax.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * LINEAR / AFFINE INTEGRATION
 * ============================================================================
 *
 * Associated types may resolve to linear or affine resource types.
 *
 * Examples:
 *
 *     T::Resource
 *
 *     T::Handle
 *
 * The semantic layer determines whether the resolved type carries linear or
 * affine ownership semantics.
 *
 * This grammar does not impose ownership rules.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Associated types are domain-neutral and may describe quantum abstractions.
 *
 * Examples:
 *
 *     QuantumBackend::Qubit
 *     Circuit::State
 *     Circuit::Measurement
 *     QubitRegister::Element
 *
 * This grammar does not:
 *
 *     - enumerate quantum operations;
 *     - allocate qubits;
 *     - identify physical qubits;
 *     - select a QPU;
 *     - select a gate set;
 *     - inspect topology;
 *     - perform routing;
 *     - perform scheduling;
 *     - select calibration;
 *     - perform QEC;
 *     - generate ZQN;
 *     - access HAL.
 *
 * After semantic resolution, quantum-specific meaning continues through the
 * canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Associated types may represent hardware-intent abstractions:
 *
 *     Module::Signal
 *     Module::Clock
 *     Module::Memory
 *     Accelerator::Interface
 *     Device::Capability
 *
 * The grammar does not assign:
 *
 *     physical pins
 *     register widths
 *     device IDs
 *     memory-bank IDs
 *     FPGA resources
 *     ASIC resources
 *
 * Those are target realization concerns.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * AI / DATA / DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Associated types are equally applicable to:
 *
 *     learned-model abstractions;
 *     data abstractions;
 *     graph abstractions;
 *     agent abstractions;
 *     distributed abstractions;
 *     networking abstractions;
 *     security abstractions;
 *     future computational domains.
 *
 * Examples:
 *
 *     Model::Input
 *     Model::Output
 *     Dataset::Element
 *     Graph::Node
 *     Agent::Message
 *     Service::Request
 *
 * No application-specific vocabulary is required.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Associated types may resolve to semantic resource/capability abstractions.
 *
 * Example:
 *
 *     Device::Capability
 *
 *     Backend::Resource
 *
 *     Accelerator::Memory
 *
 * The grammar does not determine whether a capability exists on the current
 * target.
 *
 * Resource/capability negotiation belongs downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Associated types describe source-level relationships.
 *
 * They MUST NOT encode target-specific implementation limits.
 *
 * This file therefore contains no language-level limits for:
 *
 *     CPUs
 *     cores
 *     threads
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     QPUs
 *     qubits
 *     nodes
 *     devices
 *     memory
 *     storage
 *     register width
 *     vector width
 *     tensor rank
 *     topology
 *     network size
 *
 * It MUST contain no constants or rules such as:
 *
 *     MAX_ASSOCIATED_TYPES
 *     MAX_PROJECTION_DEPTH
 *     MAX_TYPE_PARAMETERS
 *     MAX_GENERIC_ARITY
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *
 * Recursive and repeated structures are represented using grammar recursion
 * and repetition rather than finite enumerations.
 *
 * Practical compiler limits, where necessary for denial-of-service protection,
 * belong to explicit implementation policy and MUST NOT become language
 * semantics.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * This grammar does not select:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     simulator
 *     cluster
 *     cloud
 *     vendor
 *     ABI
 *     calling convention
 *
 * An associated type therefore remains portable across target realizations.
 *
 * The semantic compiler may later determine how the resolved type is
 * represented on a particular target.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing this construct is deterministic for a fixed:
 *
 *     source
 *     lexer configuration
 *     language version
 *     parser configuration
 *
 * The grammar performs:
 *
 *     no I/O
 *     no randomness
 *     no hardware discovery
 *     no network access
 *     no environment inspection
 *     no runtime execution
 *     no mutable global state
 *
 * Source ordering is preserved.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * The frontend must preserve source spans for:
 *
 *     associatedType
 *     associatedTypeBase
 *     DOUBLE_COLON
 *     associatedTypeMember
 *
 * This enables:
 *
 *     precise diagnostics;
 *     IDE navigation;
 *     rename support;
 *     formatting;
 *     source maps;
 *     provenance;
 *     semantic diagnostics;
 *     compatibility tooling.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * Structurally incomplete projection forms must be rejected:
 *
 *     T::
 *     T::+
 *     T::<
 *     T::()
 *
 * A syntactically valid but unresolved projection must remain parseable:
 *
 *     T::Unknown
 *
 *     FutureType::FutureMember
 *
 * because existence and compatibility are semantic questions.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This is a new file:
 *
 *     grammar/types/associated.g4
 *
 * Therefore no existing filename is renamed.
 *
 * Existing TypeExpr::Associated support in:
 *
 *     src/frontend/ast/node/types/type_expr.rs
 *
 * is reused.
 *
 * Existing qualified-name syntax remains valid.
 *
 * Existing generic application syntax remains owned by:
 *
 *     grammar/types/generic.g4
 *
 * Existing type constraints remain owned by:
 *
 *     grammar/types/type-constraints.g4
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * CRITICAL INTEGRATION NOTE
 * ============================================================================
 *
 * This delegate must be imported exactly once by the canonical parser
 * composition hierarchy.
 *
 * The canonical parser boundary remains:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * The complete-program root remains:
 *
 *     grammar/Zamani.g4
 *
 * The type-composition authority remains:
 *
 *     grammar/types/types.g4
 *
 * The integration MUST NOT create another:
 *
 *     typeExpression
 *
 * authority.
 *
 * ============================================================================
 *
 * RECOMMENDED COMPOSITION
 * ============================================================================
 *
 * Conceptually:
 *
 *     parser grammar Types;
 *
 *     options {
 *         tokenVocab = ZamaniLexer;
 *     }
 *
 *     ...
 *
 *     typeCore
 *         : ...
 *         | associatedType
 *         | ...
 *         ;
 *
 * However, the repository's existing `typePath` production already accepts
 * arbitrary qualified names:
 *
 *     A::B::C
 *
 * Therefore the semantic/frontend adapter MUST distinguish:
 *
 *     qualified type path
 *
 * from:
 *
 *     associated projection
 *
 * using declaration/symbol information rather than treating every final
 * `::Identifier` as an associated type.
 *
 * This is a deliberate semantic disambiguation boundary.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * REQUIRED SEMANTIC DISAMBIGUATION
 * ============================================================================
 *
 * Given:
 *
 *     A::B
 *
 * the resolver considers:
 *
 *     A::B as a qualified type path
 *
 * and:
 *
 *     A::B as an associated projection
 *
 * according to the resolved symbol/type context.
 *
 * The resolver MUST NOT depend on a finite list of known associated-type names.
 *
 * The resolver should prefer an explicitly declared associated-type/member
 * relation where one exists and otherwise retain ordinary qualified-name
 * interpretation according to the language's name-resolution rules.
 *
 * This preserves extensibility for:
 *
 *     modules
 *     packages
 *     interfaces
 *     traits
 *     generic types
 *     quantum domains
 *     hardware domains
 *     AI/data domains
 *     future domains.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * NO SEMANTIC PREDICATES
 * ============================================================================
 *
 * This grammar MUST NOT use semantic predicates to inspect:
 *
 *     symbol tables
 *     types
 *     declarations
 *     capabilities
 *     hardware
 *     resources
 *     runtime state
 *
 * The grammar remains structurally deterministic and target-independent.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no embedded Rust;
 *     no target-language actions;
 *     no semantic predicates;
 *     no filesystem operations;
 *     no network operations;
 *     no subprocess execution;
 *     no environment inspection;
 *     no hardware access;
 *     no credentials;
 *     no dynamic code execution.
 *
 * Generated Rust code therefore remains subject to the repository's normal
 * safe-Rust requirements.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * The grammar itself is language-independent.
 *
 * The consuming Zamani frontend MUST remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and must use:
 *
 *     safe Rust only
 *
 * with:
 *
 *     no unsafe code.
 *
 * The grammar does not require nightly Rust features.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * The following cases must be represented in parser/conformance tests.
 *
 * ---------------------------------------------------------------------------
 * BASIC POSITIVE
 * ---------------------------------------------------------------------------
 *
 *     T::Item
 *     T::Output
 *     Iterator::Item
 *     Collection::Element
 *
 * ---------------------------------------------------------------------------
 * QUALIFIED BASE
 * ---------------------------------------------------------------------------
 *
 *     quantum::Circuit::State
 *     hardware::Module::Signal
 *     data::Collection::Element
 *
 * ---------------------------------------------------------------------------
 * GENERIC BASE
 * ---------------------------------------------------------------------------
 *
 *     Iterator<T>::Item
 *     Collection<Key, Value>::Element
 *
 * ---------------------------------------------------------------------------
 * NESTED / COMPOSED
 * ---------------------------------------------------------------------------
 *
 *     Option<Iterator<T>::Item>
 *
 *     Result<T::Output, T::Error>
 *
 *     Array<T::Element; N>
 *
 * ---------------------------------------------------------------------------
 * UNKNOWN BUT SYNTACTICALLY VALID
 * ---------------------------------------------------------------------------
 *
 *     FutureType::FutureMember
 *
 *     T::Unknown
 *
 * These must not be rejected merely because the semantic declaration does not
 * currently exist.
 *
 * ---------------------------------------------------------------------------
 * NEGATIVE
 * ---------------------------------------------------------------------------
 *
 *     T::
 *     T::+
 *     T::<
 *     T::()
 *
 * ---------------------------------------------------------------------------
 * QUALIFIED-NAME COMPATIBILITY
 * ---------------------------------------------------------------------------
 *
 *     module::Type
 *     module::nested::Type
 *     quantum::State
 *     hardware::Signal
 *
 * must continue to parse according to the existing qualified-name/type-path
 * rules.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SEMANTIC TEST CONTRACT
 * ============================================================================
 *
 * Parser tests are insufficient by themselves.
 *
 * Semantic tests must cover at least:
 *
 *     1. valid associated type;
 *     2. unknown associated member;
 *     3. invalid projection base;
 *     4. projection from generic parameter;
 *     5. projection after generic substitution;
 *     6. associated-type equality;
 *     7. associated-type bounds;
 *     8. nested associated projections;
 *     9. qualified-name versus associated-type resolution;
 *    10. ambiguous-looking names;
 *    11. quantum-associated types;
 *    12. HDL/hardware-associated types;
 *    13. dependent use of associated types;
 *    14. type-level metaprogramming use;
 *    15. deterministic diagnostics.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Scalability tests must NOT use hard-coded language maxima.
 *
 * They should generate source according to available test resources and verify
 * that the grammar preserves correctness as structures grow.
 *
 * Examples:
 *
 *     deeply qualified projection paths;
 *     deeply nested generic bases;
 *     large source-defined bound sets;
 *     large numbers of independent projections;
 *     nested associated projections;
 *     associated types participating in dependent types.
 *
 * The test harness may impose an execution/resource budget for safety.
 *
 * That budget is an implementation/test policy and is not a language limit.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * CROSS-DOMAIN TEST CONTRACT
 * ============================================================================
 *
 * At least one conformance suite must verify that the same associated-type
 * mechanism works across:
 *
 *     classical
 *     numerical
 *     AI
 *     data
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     accelerator
 *     distributed
 *     networking
 *     security
 *     future extensible domains.
 *
 * No domain gets a private associated-type syntax.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Associated-type syntax must preserve:
 *
 *     source span;
 *     base source structure;
 *     projected member spelling;
 *     declaration/reference provenance once resolved.
 *
 * Semantic provenance may later record:
 *
 *     resolved declaration;
 *     substitution;
 *     normalization;
 *     specialization;
 *     transformation;
 *     verification.
 *
 * This grammar itself records none of those semantic facts.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * POCO-REAF INTEGRATION
 * ============================================================================
 *
 * Associated types contribute to program portability by describing abstract
 * relationships rather than physical implementation.
 *
 * For example:
 *
 *     fn process<T>(value: T::Item) -> T::Output
 *
 * describes relationships between types.
 *
 * It does not require:
 *
 *     a particular CPU;
 *     a particular GPU;
 *     a particular QPU;
 *     a particular FPGA;
 *     a particular number of devices;
 *     a particular memory size;
 *     a particular register width;
 *     a particular topology.
 *
 * Semantic/resource analysis may later determine whether the program can be
 * realized on a particular target.
 *
 * The source-level projection remains unchanged.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] It is a parser grammar, not a lexer grammar.
 *     [x] It uses tokenVocab = ZamaniLexer.
 *     [x] It defines no lexer tokens.
 *     [x] It defines no second typeExpression.
 *     [x] It reuses the canonical TypeExpr representation.
 *     [x] It maps conceptually to TypeExpr::Associated.
 *     [x] It does not create a second associated-type AST.
 *     [x] It does not duplicate generic syntax.
 *     [x] It does not duplicate type constraints.
 *     [x] It does not duplicate qualified-name token definitions.
 *     [x] It permits user-defined associated member names.
 *     [x] It remains open-world.
 *     [x] It contains no machine-capacity limits.
 *     [x] It contains no quantum-capacity limits.
 *     [x] It contains no target selection.
 *     [x] It contains no resource discovery.
 *     [x] It contains no hardware assumptions.
 *     [x] It contains no semantic predicates.
 *     [x] It contains no embedded Rust.
 *     [x] It requires no unsafe Rust.
 *     [x] It preserves source structure for semantic resolution.
 *     [x] It remains compatible with generic and dependent types.
 *     [x] It remains compatible with quantum and HDL/hardware types.
 *     [x] It remains compatible with type-level metaprogramming.
 *     [x] It has positive, negative, boundary, scalability and cross-domain
 *         test requirements.
 *
 * ============================================================================
 * FINAL INTEGRATION CONTRACT
 * ============================================================================
 *
 * UPSTREAM:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *          |
 *          v
 *     canonical token vocabulary
 *          |
 *          v
 *     grammar/types/associated.g4
 *
 * TYPE INTEGRATION:
 *
 *     grammar/types/types.g4
 *          |
 *          +--> typeExpression
 *          |
 *          +--> generic.g4
 *          |
 *          +--> associated.g4
 *          |
 *          +--> dependent.g4
 *          |
 *          +--> type-constraints.g4
 *
 * AST:
 *
 *     associated projection
 *          |
 *          v
 *     src/frontend/ast/node/types/type_expr.rs
 *          |
 *          v
 *     TypeExpr::Associated
 *
 * SEMANTICS:
 *
 *     TypeExpr::Associated
 *          |
 *          v
 *     name/type resolution
 *          |
 *          v
 *     associated-type validation
 *          |
 *          v
 *     generic substitution / normalization
 *          |
 *          v
 *     semantic type model
 *
 * METAPROGRAMMING:
 *
 *     grammar/metaprogramming/type-level.g4
 *          |
 *          v
 *     canonical TypeExpr
 *
 * DOMAIN INTEGRATION:
 *
 *     classical
 *     quantum::ir
 *     hybrid
 *     HDL
 *     hardware
 *     AI
 *     data
 *     distributed
 *     networking
 *     future domains
 *
 * TARGET INTEGRATION:
 *
 *     semantic type
 *          |
 *          v
 *     canonical IR
 *          |
 *          v
 *     optimization/lowering
 *          |
 *          v
 *     target realization
 *
 * There is deliberately no direct:
 *
 *     associated.g4 -> quantum backend
 *     associated.g4 -> hardware
 *     associated.g4 -> runtime
 *
 * dependency.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */
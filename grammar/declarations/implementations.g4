/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/declarations/implementations.g4
 *
 * Grammar:
 *     ZamaniImplementations
 *
 * Status:
 *     Production parser delegate.
 *
 * Purpose:
 *     Canonical source syntax for implementation declarations:
 *
 *         impl Type {
 *             ...
 *         }
 *
 *         impl Trait for Type {
 *             ...
 *         }
 *
 * The implementation grammar is deliberately source/semantic-domain neutral.
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
 *     parser
 *          |
 *          v
 *     implementationDeclaration
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> name resolution
 *          +--> type checking
 *          +--> generic constraint solving
 *          +--> trait/interface resolution
 *          +--> implementation coherence
 *          +--> member compatibility
 *          +--> effect checking
 *          +--> capability/resource analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical semantics / IR
 *          +--> quantum semantics / quantum::ir
 *          +--> HDL / hardware semantics
 *          +--> distributed / AI / data / networking semantics
 *          |
 *          v
 *     optimization / lowering
 *          |
 *          +--> routing
 *          +--> scheduling
 *          +--> resilience
 *          +--> QEC
 *          +--> ZQN
 *          +--> HAL
 *          |
 *          v
 *     target realization
 *
 * This file MUST NOT bypass the frontend AST/semantic boundary.
 *
 * In particular:
 *
 *     implementations.g4 -> quantum::ir
 *
 * is forbidden.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - implementationDeclaration;
 *     - implementationHead;
 *     - implementationBody;
 *     - implementationMember;
 *     - implementation method syntax;
 *     - concrete associated-type implementation syntax;
 *     - concrete associated-constant implementation syntax;
 *     - implementation-specific composition of generic parameters;
 *     - implementation-specific composition of where clauses.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer tokens;
 *     - identifiers;
 *     - qualified-name syntax;
 *     - generic parameter syntax;
 *     - where-clause syntax;
 *     - type-expression syntax;
 *     - expression syntax;
 *     - parameter syntax;
 *     - return-type syntax;
 *     - block syntax;
 *     - attributes;
 *     - modifiers;
 *     - effects;
 *     - contracts;
 *     - trait declarations;
 *     - interface declarations;
 *     - class declarations;
 *     - type declarations;
 *     - resource allocation;
 *     - capability discovery;
 *     - hardware discovery;
 *     - topology;
 *     - physical qubit mapping;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - calibration;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution;
 *     - backend selection.
 *
 * ============================================================================
 * CANONICAL SYNTAX
 * ============================================================================
 *
 * Normative implementation syntax is structurally:
 *
 *     ImplDeclaration ::=
 *         "impl"
 *         [GenericParameterList]
 *         ImplementationHead
 *         [WhereClause]
 *         ImplementationBody ;
 *
 *     ImplementationHead ::=
 *           TypeExpression
 *         | TypeExpression "for" TypeExpression ;
 *
 * Therefore:
 *
 *     impl Type { ... }
 *
 * represents an inherent implementation.
 *
 *     impl Trait for Type { ... }
 *
 * represents a contract/trait/interface implementation candidate.
 *
 * Semantic analysis determines whether the first type expression denotes a
 * trait, interface, or another implementation contract.
 *
 * The parser MUST NOT hard-code a finite list of trait/interface names.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Implementations describe source-level behavior attached to semantic types.
 *
 * They MUST NOT encode:
 *
 *     MAX_METHODS
 *     MAX_ASSOCIATED_TYPES
 *     MAX_ASSOCIATED_CONSTANTS
 *     MAX_GENERIC_PARAMETERS
 *     MAX_WHERE_PREDICATES
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_DEVICES
 *     MAX_TENSOR_RANK
 *
 * There is no language-level finite implementation size.
 *
 * Repetition is represented by ANTLR repetition operators.
 *
 * Practical parser/compiler/resource budgets remain implementation policy and
 * MUST NOT become language semantics.
 *
 * ============================================================================
 * DOMAIN NEUTRALITY
 * ============================================================================
 *
 * The implemented type may represent:
 *
 *     classical values;
 *     quantum values;
 *     hybrid values;
 *     HDL abstractions;
 *     hardware resources;
 *     accelerators;
 *     distributed values;
 *     AI/data values;
 *     networking abstractions;
 *     security abstractions;
 *     future computational domains.
 *
 * No domain-specific implementation rule is required here.
 *
 * Examples are therefore structurally possible:
 *
 *     impl Numeric for Tensor<T> { ... }
 *
 *     impl QuantumOperation for MyOperation { ... }
 *
 *     impl HardwareResource for Accelerator<T> { ... }
 *
 *     impl DistributedValue for Sharded<T> { ... }
 *
 * without this grammar knowing what those names mean.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * A quantum implementation is still an ordinary source-level implementation.
 *
 * This grammar does NOT determine:
 *
 *     - qubit count;
 *     - physical qubit identifiers;
 *     - logical-to-physical mapping;
 *     - gate availability;
 *     - topology;
 *     - calibration;
 *     - noise;
 *     - QEC;
 *     - ZQN;
 *     - routing;
 *     - scheduling;
 *     - QPU selection.
 *
 * Quantum semantics eventually cross the established:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * ============================================================================
 * HARDWARE / HDL BOUNDARY
 * ============================================================================
 *
 * An implementation may implement a hardware/HDL semantic abstraction, but
 * this grammar does not select:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     accelerator
 *     node
 *     memory bank
 *     physical address
 *
 * Target realization remains downstream.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The canonical frontend AST already models an implementation declaration as:
 *
 *     generic_parameters: Vec<NodeId>
 *     trait_target: Option<NodeId>
 *     self_type: NodeId
 *     constraints: Vec<NodeId>
 *     members: Vec<NodeId>
 *
 * Source mapping is therefore:
 *
 *     impl <G> C for T where W { M }
 *       |    |   |     |       |    |
 *       |    |   |     |       |    +--> members
 *       |    |   |     |       +-------> constraints
 *       |    |   |     +---------------> self_type
 *       |    |   +---------------------> trait_target
 *       |    +--------------------------> generic_parameters
 *       +-------------------------------> implementation declaration
 *
 * For:
 *
 *     impl T { ... }
 *
 * `trait_target` is absent.
 *
 * For:
 *
 *     impl Trait for T { ... }
 *
 * `trait_target` refers to `Trait`.
 *
 * The parser does not instantiate Rust AST structures itself. The frontend
 * adapter constructs the canonical AST graph using NodeId references.
 *
 * ============================================================================
 * MEMBER AST CONTRACT
 * ============================================================================
 *
 * Implementation members are represented as canonical AST child nodes.
 *
 * The current implementation AST explicitly permits ordered member NodeIds.
 *
 * Concrete members supplied by this grammar are:
 *
 *     function/method
 *     associated type
 *     associated constant
 *
 * Each member retains:
 *
 *     source span;
 *     attributes;
 *     modifiers/visibility where applicable;
 *     generic parameters;
 *     type expressions;
 *     expressions;
 *     body;
 *     source ordering.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing does NOT determine:
 *
 *     - whether the implemented type exists;
 *     - whether the contract exists;
 *     - whether the contract is a trait;
 *     - whether the contract is an interface;
 *     - whether the contract is applicable;
 *     - whether generic parameters are legal;
 *     - whether bounds are satisfiable;
 *     - whether the implementation is coherent;
 *     - whether orphan/coherence rules are satisfied;
 *     - whether duplicate implementations exist;
 *     - whether required members are implemented;
 *     - whether method signatures match;
 *     - whether effects are compatible;
 *     - whether associated types match;
 *     - whether associated constants match;
 *     - whether visibility is legal;
 *     - whether capabilities/resources are available.
 *
 * These are semantic responsibilities.
 *
 * ============================================================================
 * GENERIC INTEGRATION
 * ============================================================================
 *
 * Generic parameter syntax is owned by:
 *
 *     grammar/core/generics.g4
 *
 * This file consumes:
 *
 *     genericParameterList
 *
 * It MUST NOT redefine:
 *
 *     genericParameter
 *     genericParameterList
 *     genericBound
 *     whereClause
 *     wherePredicate
 *
 * This permits:
 *
 *     impl<T: Numeric> Trait for Type<T> { ... }
 *
 *     impl<const N: int> Trait<N> for Buffer<N> { ... }
 *
 *     impl<T, U> Converter<T, U> for Pair<T, U> { ... }
 *
 * without imposing a finite generic arity.
 *
 * ============================================================================
 * TYPE INTEGRATION
 * ============================================================================
 *
 * Type syntax is owned by:
 *
 *     grammar/types/types.g4
 *
 * The canonical rule is:
 *
 *     typeExpression
 *
 * This grammar MUST NOT use a legacy `typeReference` rule.
 *
 * This permits arbitrary source-level type structure without creating a
 * second type system.
 *
 * ============================================================================
 * WHERE INTEGRATION
 * ============================================================================
 *
 * The canonical where syntax is:
 *
 *     where Predicate, Predicate
 *
 * with no mandatory trailing semicolon.
 *
 * The canonical where grammar is supplied by:
 *
 *     grammar/core/generics.g4
 *
 * This file consumes:
 *
 *     whereClause
 *
 * only as an optional implementation suffix.
 *
 * ============================================================================
 * FUNCTION INTEGRATION
 * ============================================================================
 *
 * Implementation methods reuse the canonical callable components:
 *
 *     parameterList
 *     functionReturnClause
 *     effectClause
 *     block
 *
 * They do NOT reuse the complete `functionDeclaration` rule because an
 * implementation member has a different semantic context and must require a
 * concrete body.
 *
 * ============================================================================
 * CONCRETE METHOD CONTRACT
 * ============================================================================
 *
 * A method inside an implementation MUST have a body:
 *
 *     impl Trait for Type {
 *         fn run(value: Value) -> Result {
 *             ...
 *         }
 *     }
 *
 * The grammar therefore deliberately does NOT accept:
 *
 *     fn run(value: Value) -> Result;
 *
 * inside an implementation.
 *
 * A body-less declaration belongs to:
 *
 *     trait/interface/function-signature/foreign declaration
 *
 * syntax.
 *
 * Whether a concrete method may be empty is a semantic/contextual question;
 * `{}` remains syntactically valid.
 *
 * ============================================================================
 * ASSOCIATED TYPE CONTRACT
 * ============================================================================
 *
 * A concrete implementation supplies an associated type with:
 *
 *     type Item = Element;
 *
 * The implementation does NOT repeat the abstract bound declaration.
 *
 * Semantic analysis verifies that the supplied type satisfies the contract.
 *
 * No finite associated-type count is imposed.
 *
 * ============================================================================
 * ASSOCIATED CONSTANT CONTRACT
 * ============================================================================
 *
 * A concrete implementation supplies an associated constant with:
 *
 *     const VERSION: Version = value;
 *
 * The parser records the expression.
 *
 * It does NOT evaluate the expression.
 *
 * Constant evaluation, type checking, purity, determinism and compile-time
 * validity are semantic/compiler responsibilities.
 *
 * ============================================================================
 * ATTRIBUTES
 * ============================================================================
 *
 * Attributes are owned by:
 *
 *     grammar/core/attributes.g4
 *
 * This grammar consumes:
 *
 *     attributes
 *
 * and does not create implementation-specific attribute syntax.
 *
 * ============================================================================
 * MODIFIERS / VISIBILITY
 * ============================================================================
 *
 * Shared modifier and visibility syntax is owned by:
 *
 *     grammar/core/modifiers.g4
 *     grammar/core/visibility.g4
 *
 * This grammar consumes:
 *
 *     visibilityModifier
 *     modifierList
 *
 * Semantic validation determines which modifiers are legal for:
 *
 *     implementation declarations;
 *     implementation methods;
 *     associated members.
 *
 * ============================================================================
 * EFFECTS
 * ============================================================================
 *
 * Effect syntax is owned by:
 *
 *     grammar/effects/effects.g4
 *
 * This grammar consumes:
 *
 *     effectClause
 *
 * It does not define effect names or effect semantics.
 *
 * Quantum, hardware, distributed and future effects remain open-world.
 *
 * ============================================================================
 * CONTRACTS
 * ============================================================================
 *
 * Contract syntax is deliberately not duplicated here.
 *
 * If implementation-level pre/postconditions are supported, they must use
 * the same canonical contract grammar consumed by ordinary functions.
 *
 * A local implementation-only contract syntax MUST NOT be introduced.
 *
 * ============================================================================
 * DECLARATION DISPATCH
 * ============================================================================
 *
 * grammar/declarations/declarations.g4 must remain a dispatcher/composition
 * boundary.
 *
 * It should contain only the integration:
 *
 *     | implementationDeclaration
 *
 * and MUST NOT reproduce the implementation rules from this file.
 *
 * ============================================================================
 * TRAIT / INTERFACE INTEGRATION
 * ============================================================================
 *
 * This file does NOT import or embed trait/interface declarations.
 *
 * The dependency is intentionally one-directional:
 *
 *     traits.g4 / interfaces.g4
 *             |
 *             v
 *     semantic contract identity
 *             |
 *             v
 *     implementations.g4
 *
 * The implementation grammar only parses the source reference to the
 * contract.
 *
 * This avoids circular parser dependencies.
 *
 * ============================================================================
 * QUANTUM / CLASSICAL / HDL / FUTURE DOMAIN INTEGRATION
 * ============================================================================
 *
 * No implementation-domain enumeration is permitted.
 *
 * The same implementation syntax applies to:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     distributed
 *     AI
 *     data
 *     networking
 *     security
 *     future
 *
 * The implemented type determines semantic domain.
 *
 * ============================================================================
 * SOURCE ORDER
 * ============================================================================
 *
 * The parser must preserve:
 *
 *     generic parameter order;
 *     bound order;
 *     where predicate order;
 *     member order;
 *     method modifier order;
 *     attribute order;
 *     source spans.
 *
 * Semantic normalization/sorting/deduplication belongs downstream.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no embedded actions;
 *     no semantic predicates;
 *     no filesystem access;
 *     no network access;
 *     no runtime calls;
 *     no hardware discovery;
 *     no randomness;
 *     no mutable parser-global state.
 *
 * Identical source/token streams therefore produce identical parse structure.
 *
 * ============================================================================
 * ERROR BOUNDARY
 * ============================================================================
 *
 * Syntax errors include:
 *
 *     impl { ... }
 *     impl for Type { ... }
 *     impl Trait for { ... }
 *     impl<T:> Trait for Type { ... }
 *     impl Trait for Type
 *     impl Trait for Type { fn f(); }
 *     impl Trait for Type { type Item; }
 *     impl Trait for Type { const VALUE: Int; }
 *
 * Semantic errors include:
 *
 *     unknown contract;
 *     unknown self type;
 *     duplicate implementation;
 *     incoherent implementation;
 *     unsatisfied generic bound;
 *     missing required member;
 *     incompatible member signature;
 *     incompatible associated type;
 *     incompatible associated constant;
 *     illegal modifier combination;
 *     illegal visibility;
 *     unsupported effect;
 *     unavailable capability/resource.
 *
 * The parser MUST NOT attempt to diagnose these semantic conditions.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * These productions deliberately use:
 *
 *     *
 *     +
 *     ?
 *
 * rather than finite expansions.
 *
 * No artificial upper bound exists for:
 *
 *     implementations;
 *     generic parameters;
 *     bounds;
 *     where predicates;
 *     members;
 *     methods;
 *     associated types;
 *     associated constants;
 *     method parameters;
 *     type-expression complexity.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust implementation code.
 *
 * Generated compiler/parser integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and safe Rust only.
 *
 * No Rust `unsafe` is required or permitted.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_DEVICES
 *     physical-qubit identifiers
 *     device identifiers
 *     vendor-specific implementation categories
 *     backend-specific implementation rules.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive:
 *
 *     impl Number {
 *     }
 *
 *     impl<T: Numeric> Number for Wrapper<T> {
 *     }
 *
 *     impl<T, U> Converter<T, U> for Pair<T, U> {
 *     }
 *
 *     impl<T: Numeric + Ordered> Trait for Container<T>
 *     where T: Serializable
 *     {
 *     }
 *
 *     impl Iterator for Collection {
 *         type Item = Element;
 *     }
 *
 *     impl Configurable for Device {
 *         const VERSION: Version = 1;
 *     }
 *
 *     impl Trait for Type {
 *         fn run(value: Value) -> Result {
 *             value
 *         }
 *     }
 *
 *     impl Trait for Type {
 *         #[metadata]
 *         pub async fn_name(value: Value) -> Result {
 *             value
 *         }
 *     }
 *
 * Cross-domain structural tests:
 *
 *     impl quantum::Operation for MyOperation { ... }
 *
 *     impl hardware::Resource for Accelerator { ... }
 *
 *     impl hdl::Module for MyModule { ... }
 *
 *     impl distributed::Service for Service { ... }
 *
 *     impl ai::Model for Model { ... }
 *
 * These tests verify syntax openness only; semantic availability is tested
 * downstream.
 *
 * Negative:
 *
 *     impl { }
 *
 *     impl for Type { }
 *
 *     impl Trait for { }
 *
 *     impl<T:> Trait for Type { }
 *
 *     impl<T: A +> Trait for Type { }
 *
 *     impl Trait for Type
 *
 *     impl Trait for Type {
 *         fn run();
 *     }
 *
 *     impl Trait for Type {
 *         type Item;
 *     }
 *
 *     impl Trait for Type {
 *         const VALUE: Int;
 *     }
 *
 * Boundary:
 *
 *     empty implementation body;
 *     one member;
 *     many members;
 *     many generic parameters;
 *     many bounds;
 *     many where predicates;
 *     deeply nested generic types;
 *     deeply qualified names;
 *     large associated constant expressions.
 *
 * Scalability:
 *
 *     source-level implementation count limited only by implementation
 *     resource policy, never by grammar cardinality.
 *
 * ============================================================================
 * PRODUCTION COMPLETION CRITERIA
 * ============================================================================
 *
 * [x] Canonical ZamaniLexer vocabulary.
 * [x] No lexer rules.
 * [x] No unsafe Rust.
 * [x] Rust 1.97 / 1.97.1 compatible implementation contract.
 * [x] Inherent implementations.
 * [x] Contract implementations.
 * [x] Generic implementations.
 * [x] Open-ended type expressions.
 * [x] Canonical where clauses.
 * [x] Concrete methods.
 * [x] Associated types.
 * [x] Associated constants.
 * [x] Shared attributes.
 * [x] Shared modifiers.
 * [x] Shared visibility.
 * [x] Shared parameters.
 * [x] Shared return clauses.
 * [x] Shared effects.
 * [x] Shared blocks.
 * [x] No property-specific syntax without an AST/spec contract.
 * [x] No machine/resource limits.
 * [x] No hardware-specific syntax.
 * [x] No quantum gate enumeration.
 * [x] No quantum IR duplication.
 * [x] No QEC/ZQN/HAL coupling.
 * [x] Deterministic source-order preservation.
 * [x] Explicit AST mapping.
 * [x] Explicit semantic boundary.
 * [x] Explicit IR boundary.
 * [x] Positive/negative/boundary/scalability tests defined.
 *
 * ============================================================================
 */

parser grammar ZamaniImplementations;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * Shared parser delegates.
 *
 * These are integration dependencies only. Their concrete syntax remains
 * owned by their respective files.
 */
import
    Attributes,
    Types,
    Expressions,
    Parameters,
    Returns,
    Effects,
    Blocks,
    Modifiers,
    Visibility,
    Generics
    ;


/* ============================================================================
 * IMPLEMENTATION DECLARATION
 * ========================================================================== */

implementationDeclaration
    : attributes?
      visibilityModifier?
      modifierList?
      IMPL
      genericParameterList?
      implementationHead
      whereClause?
      implementationBody
    ;


/* ============================================================================
 * IMPLEMENTATION HEAD
 * ========================================================================== */

/*
 * Canonical source forms:
 *
 *     impl Type
 *
 *     impl Trait for Type
 *
 * Both sides are type expressions rather than merely identifiers. This permits
 * generic/constructed/qualified types without introducing a second type-name
 * grammar.
 *
 * Semantic analysis determines whether the first type expression is actually
 * an implementation contract.
 */
implementationHead
    : typeExpression
      (
          FOR
          typeExpression
      )?
    ;


/* ============================================================================
 * IMPLEMENTATION BODY
 * ========================================================================== */

implementationBody
    : LBRACE
      implementationMember*
      RBRACE
    ;


/* ============================================================================
 * IMPLEMENTATION MEMBER
 * ========================================================================== */

implementationMember
    : attributes?
      implementationMemberCore
    ;

implementationMemberCore
    : implementationMethodDeclaration
    | implementationAssociatedTypeDeclaration
    | implementationAssociatedConstantDeclaration
    ;


/* ============================================================================
 * IMPLEMENTATION METHOD
 * ========================================================================== */

/*
 * Concrete implementation methods deliberately require a block.
 *
 * A declaration-only:
 *
 *     fn f(...);
 *
 * belongs to trait/interface/signature/foreign syntax rather than an
 * implementation body.
 */
implementationMethodDeclaration
    : visibilityModifier?
      modifierList?
      FN
      identifier
      genericParameterList?
      LPAREN
      parameterList?
      RPAREN
      functionReturnClause?
      effectClause?
      block
    ;


/* ============================================================================
 * ASSOCIATED TYPE IMPLEMENTATION
 * ========================================================================== */

/*
 * Concrete associated type:
 *
 *     type Item = Element;
 *
 * Bounds belong to the abstract trait/interface declaration. The implementation
 * supplies the actual type and semantic analysis checks that it satisfies the
 * declared contract.
 */
implementationAssociatedTypeDeclaration
    : visibilityModifier?
      TYPE
      identifier
      ASSIGN
      typeExpression
      SEMICOLON
    ;


/* ============================================================================
 * ASSOCIATED CONSTANT IMPLEMENTATION
 * ========================================================================== */

/*
 * Concrete associated constant:
 *
 *     const VERSION: Version = 1;
 *
 * The expression is parsed but not evaluated here.
 */
implementationAssociatedConstantDeclaration
    : visibilityModifier?
      CONST
      identifier
      COLON
      typeExpression
      ASSIGN
      expression
      SEMICOLON
    ;
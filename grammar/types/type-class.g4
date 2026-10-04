/*
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
 *     CANONICAL TYPE-SYSTEM TYPE-CLASS COMPOSITION DELEGATE
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file establishes the canonical TYPE-SYSTEM view of type classes.
 *
 * IMPORTANT:
 *
 * Zamani already has a canonical source declaration grammar:
 *
 *     grammar/declarations/traits.g4
 *
 * That grammar owns the source syntax for:
 *
 *     trait declarations
 *     trait methods
 *     associated types
 *     associated constants
 *     supertraits
 *     generic parameters
 *     where clauses
 *     trait members
 *
 * Therefore this file MUST NOT create a second declaration syntax such as:
 *
 *     typeclass Foo { ... }
 *
 * or:
 *
 *     class Foo implements ...
 *
 * or any other competing type-class declaration language.
 *
 * In Zamani, the source-level abstraction is represented by the existing
 * extensible trait/interface contract and is interpreted by the semantic
 * type-class system.
 *
 * This file provides the type-system composition boundary for that concept.
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * Source declaration:
 *
 *     trait Numeric<T> {
 *         ...
 *     }
 *
 * is owned by:
 *
 *     grammar/declarations/traits.g4
 *
 * Type-class interpretation:
 *
 *     "Numeric is a constraint-bearing type abstraction"
 *
 * belongs to semantic analysis.
 *
 * This file bridges those concepts without duplicating either syntax or AST.
 *
 * The architecture is therefore:
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     Traits
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     type-class semantic interpretation
 *       |
 *       +-------------------------+
 *       |                         |
 *       v                         v
 * generic/type checking       associated types
 *       |                         |
 *       +------------+------------+
 *                    |
 *                    v
 *             canonical semantic model
 *                    |
 *          +---------+---------+
 *          |         |         |
 *          v         v         v
 *      classical   quantum    HDL/hardware
 *          |         |         |
 *          +---------+---------+
 *                    |
 *                    v
 *              canonical IR
 *
 * No physical target information enters this grammar.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - typeClassDeclaration as a type-system semantic alias of the canonical
 *       trait declaration;
 *
 *     - typeClassReference as the canonical type-expression view used by
 *       type-system integration points;
 *
 *     - typeClassConstraint as the canonical type-expression payload used
 *       when a semantic constraint is interpreted as type-class satisfaction;
 *
 *     - typeClassConstraintList as an ordered conjunction of such constraints;
 *
 *     - typeClassProjection as the source-level projection shape for an
 *       associated member when a dedicated type-system context needs to
 *       distinguish it structurally;
 *
 *     - typeClassAssociatedType as the source-level associated-type projection
 *       form;
 *
 *     - type-class-specific integration boundaries.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer rules;
 *     - keyword definitions;
 *     - identifier syntax;
 *     - typeExpression;
 *     - named types;
 *     - generic type application;
 *     - generic parameter declaration;
 *     - generic parameter bounds;
 *     - trait declaration bodies;
 *     - trait methods;
 *     - trait associated-type declarations;
 *     - trait associated constants;
 *     - where clauses;
 *     - expressions;
 *     - statements;
 *     - implementation blocks;
 *     - type inference;
 *     - type substitution;
 *     - unification;
 *     - trait coherence;
 *     - specialization;
 *     - monomorphization;
 *     - resource discovery;
 *     - capability negotiation;
 *     - hardware discovery;
 *     - quantum allocation;
 *     - routing;
 *     - scheduling;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution.
 *
 * ============================================================================
 * SINGLE SOURCE-OF-TRUTH RULE
 * ============================================================================
 *
 * There is exactly one source-level declaration owner:
 *
 *     grammar/declarations/traits.g4
 *
 * This file MUST NOT redefine:
 *
 *     traitDeclaration
 *     traitBody
 *     traitMember
 *     traitMethodDeclaration
 *     traitAssociatedTypeDeclaration
 *     traitAssociatedConstantDeclaration
 *     traitInheritanceClause
 *     traitWhereClause
 *
 * If a type-class construct needs those declarations, it references the
 * canonical trait declaration rather than copying it.
 *
 * ============================================================================
 * SINGLE TYPE-EXPRESSION AUTHORITY
 * ============================================================================
 *
 * The canonical source type expression is owned by:
 *
 *     grammar/types/types.g4
 *
 * This file MUST NOT redefine:
 *
 *     typeExpression
 *     typeCore
 *     namedType
 *     typePath
 *     genericType
 *     tupleType
 *     arrayType
 *     sliceType
 *     functionType
 *     referenceType
 *     pointerType
 *     optionalType
 *     resultType
 *     dependentType
 *     quantumType
 *
 * Type-class references are therefore represented through the existing
 * canonical TypeExpr structure.
 *
 * ============================================================================
 * WHY THIS FILE EXISTS
 * ============================================================================
 *
 * A type class is a semantic abstraction, not a second kind of machine.
 *
 * It allows a source program to state that a type participates in a reusable
 * contract.
 *
 * Examples:
 *
 *     T: Numeric
 *
 *     T: Comparable
 *
 *     T: Numeric + Comparable
 *
 *     T: quantum::Observable
 *
 *     T: hardware::Signal
 *
 *     T: data::Serializable
 *
 * The grammar does not determine whether a named constraint denotes:
 *
 *     trait satisfaction
 *     interface satisfaction
 *     capability satisfaction
 *     refinement
 *     subtype compatibility
 *     domain property
 *
 * Semantic analysis decides that.
 *
 * ============================================================================
 * OPEN-WORLD DESIGN
 * ============================================================================
 *
 * This grammar intentionally does not enumerate built-in type classes.
 *
 * It does NOT contain a finite list such as:
 *
 *     Numeric
 *     Comparable
 *     Iterable
 *     Serializable
 *     Quantum
 *     Hardware
 *     Tensor
 *     Neural
 *
 * Those names are ordinary source-level types/contracts.
 *
 * Consequently a future domain can introduce:
 *
 *     future::computing::Property
 *
 * without modifying this file.
 *
 * This is mandatory for POCO-REAF.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Type classes describe SOURCE-LEVEL TYPE INTENT.
 *
 * They must not describe physical implementation limits.
 *
 * This grammar contains no limits for:
 *
 *     type classes
 *     implementations
 *     generic parameters
 *     bounds
 *     associated types
 *     associated members
 *     inheritance depth
 *     type-expression depth
 *     quantum resources
 *     classical resources
 *     hardware resources
 *     distributed resources
 *
 * It MUST NOT introduce:
 *
 *     MAX_TYPE_CLASSES
 *     MAX_TYPE_CLASS_BOUNDS
 *     MAX_GENERIC_ARITY
 *     MAX_ASSOCIATED_TYPES
 *     MAX_ASSOCIATED_MEMBERS
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
 *
 * Practical parser/compiler resource protection is implementation policy.
 *
 * Such protection MUST:
 *
 *     - remain outside language semantics;
 *     - be explicit;
 *     - be configurable;
 *     - be diagnosable;
 *     - never change the meaning of valid source syntax.
 *
 * ============================================================================
 * SOURCE DECLARATION INTEGRATION
 * ============================================================================
 *
 * Canonical source form:
 *
 *     trait Numeric<T> {
 *         ...
 *     }
 *
 * is owned by:
 *
 *     grammar/declarations/traits.g4
 *
 * This file exposes it through:
 *
 *     typeClassDeclaration
 *
 * This is intentionally an alias/delegation rule.
 *
 * It does not introduce another keyword.
 *
 * The language therefore does NOT require both:
 *
 *     trait Numeric ...
 *
 * and:
 *
 *     typeclass Numeric ...
 *
 * to represent the same concept.
 *
 * One source declaration, one AST, one semantic model.
 *
 * ============================================================================
 * TYPE-CLASS REFERENCE
 * ============================================================================
 *
 * A type-class reference is represented by the canonical source type system.
 *
 * Examples:
 *
 *     Numeric
 *
 *     Comparable
 *
 *     quantum::Observable
 *
 *     data::Serializable
 *
 *     collections::Iterable<T>
 *
 *     future::domain::Capability
 *
 * The grammar delegates the actual type structure to `typeExpression`.
 *
 * This is essential because a type-class reference may itself be:
 *
 *     qualified;
 *     generic;
 *     parameterized;
 *     dependent;
 *     associated;
 *     domain-specific;
 *     future-defined.
 *
 * No closed-world class catalogue is allowed.
 *
 * ============================================================================
 * TYPE-CLASS CONSTRAINT
 * ============================================================================
 *
 * A type-class constraint is structurally a type expression.
 *
 * Examples:
 *
 *     Numeric
 *
 *     Comparable
 *
 *     quantum::Observable
 *
 *     Numeric<T>
 *
 * The semantic layer determines whether the referenced type expression is
 * actually a type-class contract in the current scope.
 *
 * The parser does NOT resolve it.
 *
 * ============================================================================
 * CONSTRAINT LIST
 * ============================================================================
 *
 * The canonical conjunction syntax remains:
 *
 *     Numeric + Comparable
 *
 *     quantum::Observable + quantum::Measurable
 *
 * This file preserves source order.
 *
 * It does not:
 *
 *     sort;
 *     deduplicate;
 *     normalize;
 *     resolve;
 *     simplify;
 *     prove;
 *
 * the constraints.
 *
 * Those are semantic operations.
 *
 * ============================================================================
 * ASSOCIATED TYPE PROJECTIONS
 * ============================================================================
 *
 * Existing trait declarations may contain:
 *
 *     type Item;
 *
 * A consuming type context may need to refer to that associated type.
 *
 * The source-level projection shape is:
 *
 *     Iterator::Item
 *
 * or:
 *
 *     T::Item
 *
 * or:
 *
 *     collection::Iterator::Item
 *
 * The grammar preserves this source structure.
 *
 * It does not determine whether the final segment is:
 *
 *     a namespace;
 *     a type;
 *     an associated type;
 *     an associated constant;
 *     a module;
 *     another semantic entity.
 *
 * Resolution is downstream.
 *
 * ============================================================================
 * IMPORTANT ASSOCIATED-TYPE RULE
 * ============================================================================
 *
 * `typePath` already supports:
 *
 *     T::Item
 *
 * Therefore the normal type-expression parser may represent this as a
 * qualified type path.
 *
 * The dedicated projection rule in this file exists only for contexts where
 * the parser composition explicitly needs the semantic distinction.
 *
 * It MUST NOT replace or compete with `typePath`.
 *
 * The canonical AST remains the existing domain-neutral type representation.
 *
 * ============================================================================
 * GENERIC INTEGRATION
 * ============================================================================
 *
 * Type classes interact with generics through existing generic declarations.
 *
 * Example:
 *
 *     trait Numeric<T> {
 *         ...
 *     }
 *
 *     fn sum<T: Numeric>(value: T) -> T {
 *         ...
 *     }
 *
 *     fn compare<T>
 *     where
 *         T: Numeric + Comparable
 *     {
 *         ...
 *     }
 *
 * Generic declaration syntax remains owned by the generic-declaration
 * grammar.
 *
 * Type-class constraints are consumed as type constraints.
 *
 * This file MUST NOT redefine:
 *
 *     genericParameter
 *     genericParameterList
 *     genericArgumentList
 *     genericType
 *
 * ============================================================================
 * TYPE-CONSTRAINT INTEGRATION
 * ============================================================================
 *
 * The repository already has:
 *
 *     grammar/types/type-constraints.g4
 *
 * as the intended reusable type-bound grammar.
 *
 * Therefore this file MUST NOT create another competing colon/bound grammar.
 *
 * The canonical relationship is:
 *
 *     generic parameter
 *          |
 *          v
 *     type constraint clause
 *          |
 *          v
 *     type expression
 *          |
 *          v
 *     semantic type-class interpretation
 *
 * This file provides the type-class semantic view of the resulting
 * type-expression constraint.
 *
 * The existing `type-constraints.g4` should ultimately be migrated to the
 * canonical public lexer vocabulary:
 *
 *     tokenVocab = ZamaniLexer
 *
 * rather than directly naming an internal lexical composition vocabulary.
 *
 * ============================================================================
 * TRAIT INTEGRATION
 * ============================================================================
 *
 * Trait declarations remain owned by:
 *
 *     grammar/declarations/traits.g4
 *
 * That grammar already supports:
 *
 *     trait name
 *     generic parameters
 *     supertraits
 *     where clauses
 *     methods
 *     associated types
 *     associated constants
 *
 * Consequently this file must not duplicate any of those rules.
 *
 * Type-class semantics are obtained by interpreting a trait declaration as a
 * reusable type contract.
 *
 * ============================================================================
 * ASSOCIATED-TYPE INTEGRATION
 * ============================================================================
 *
 * Trait:
 *
 *     trait Collection {
 *         type Item;
 *     }
 *
 * Consumer:
 *
 *     T::Item
 *
 * The semantic model must retain:
 *
 *     declaring trait;
 *     implementing type;
 *     associated member name;
 *     associated type value;
 *     generic substitutions;
 *     source provenance.
 *
 * The grammar itself stores only source structure.
 *
 * ============================================================================
 * ASSOCIATED CONSTANT INTEGRATION
 * ============================================================================
 *
 * Trait:
 *
 *     trait Configuration {
 *         const Value: Parameter;
 *     }
 *
 * Associated constants may participate in type-level or compile-time
 * reasoning only where the semantic/type-level subsystem explicitly permits
 * them.
 *
 * This grammar does not evaluate associated constants.
 *
 * It does not turn constants into resource limits.
 *
 * ============================================================================
 * SUPERTRAIT INTEGRATION
 * ============================================================================
 *
 * Existing trait inheritance:
 *
 *     trait Ordered extends Comparable {
 *         ...
 *     }
 *
 * means that a semantic type-class implementation of `Ordered` may need to
 * satisfy `Comparable`.
 *
 * This is a semantic relationship.
 *
 * The parser does not:
 *
 *     resolve inheritance;
 *     detect cycles;
 *     prove satisfaction;
 *     construct dictionaries;
 *     generate dispatch;
 *     specialize implementations.
 *
 * ============================================================================
 * COHERENCE
 * ============================================================================
 *
 * Coherence is NOT a grammar property.
 *
 * The grammar must allow source declarations that are syntactically valid.
 *
 * Semantic analysis determines whether multiple implementations conflict.
 *
 * This separation is necessary because coherence depends on:
 *
 *     modules;
 *     imports;
 *     scopes;
 *     generic substitutions;
 *     visibility;
 *     implementation ownership;
 *     specialization policy;
 *     compatibility version.
 *
 * ============================================================================
 * SPECIALIZATION
 * ============================================================================
 *
 * This file does not introduce specialization syntax.
 *
 * If specialization becomes a supported language feature, it must receive:
 *
 *     specification;
 *     AST contract;
 *     semantic rules;
 *     coherence rules;
 *     compatibility rules;
 *     IR/lowering rules;
 *     tests.
 *
 * It must not be silently introduced here as a grammar-only feature.
 *
 * ============================================================================
 * HIGHER-KINDED TYPES
 * ============================================================================
 *
 * A type-class constraint may eventually involve a type constructor rather
 * than a fully applied type.
 *
 * Example conceptual form:
 *
 *     F: Functor
 *
 * or:
 *
 *     F<T>: Functor
 *
 * The grammar must not hard-code a particular higher-kinded representation.
 *
 * Existing type-level/metaprogramming facilities remain the owners of the
 * canonical higher-kinded representation.
 *
 * This file only preserves source-level type expressions.
 *
 * ============================================================================
 * DEPENDENT TYPE INTEGRATION
 * ============================================================================
 *
 * A type-class contract may be used with dependent/value-parameterized types.
 *
 * Examples:
 *
 *     Tensor<T>[N]
 *
 *     Matrix<T>[Rows, Cols]
 *
 * The type-class grammar does not evaluate:
 *
 *     N
 *     Rows
 *     Cols
 *
 * and does not establish physical limits for them.
 *
 * Type-level value semantics remain owned by the dependent/type-level system.
 *
 * ============================================================================
 * LINEAR / AFFINE INTEGRATION
 * ============================================================================
 *
 * A type-class constraint may appear on a linear or affine type.
 *
 * For example, semantic constructs may eventually express that a resource
 * abstraction satisfies a reusable contract.
 *
 * The grammar does not decide whether a particular implementation is:
 *
 *     linear;
 *     affine;
 *     unrestricted;
 *     copyable;
 *     movable;
 *     borrowable.
 *
 * Those are semantic/type-system properties.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Type classes are domain-neutral and may constrain quantum abstractions.
 *
 * Examples:
 *
 *     trait Observable {
 *         ...
 *     }
 *
 *     trait QuantumOperation<Q> {
 *         ...
 *     }
 *
 *     fn execute<T: quantum::Operation>(value: T) -> Result {
 *         ...
 *     }
 *
 * The grammar does not:
 *
 *     allocate qubits;
 *     identify physical qubits;
 *     enumerate gates;
 *     select a QPU;
 *     inspect coupling topology;
 *     choose calibration;
 *     perform decomposition;
 *     perform routing;
 *     perform scheduling;
 *     select QEC;
 *     construct ZQN;
 *     access HAL state.
 *
 * Required semantic direction:
 *
 *     source type-class contract
 *          |
 *          v
 *     semantic type model
 *          |
 *          v
 *     quantum semantic model
 *          |
 *          v
 *     quantum::ir
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Type classes can express reusable classical contracts:
 *
 *     Numeric
 *     Comparable
 *     Iterable
 *     Serializable
 *     Hashable
 *
 * These remain source-level semantic abstractions.
 *
 * They do not force:
 *
 *     CPU;
 *     instruction set;
 *     register width;
 *     memory representation;
 *     calling convention.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Type classes may constrain HDL/hardware abstractions:
 *
 *     hdl::Signal
 *     hardware::Memory
 *     hardware::Accelerator
 *     hardware::Module
 *
 * The type-class grammar does not encode:
 *
 *     bus width;
 *     register count;
 *     memory capacity;
 *     pipeline count;
 *     clock frequency;
 *     device count;
 *     topology.
 *
 * Those properties belong to hardware/resource semantics.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * A type class is not itself a physical resource requirement.
 *
 * Distinguish:
 *
 *     T: quantum::Observable
 *
 * from:
 *
 *     requires capability("quantum.measurement");
 *
 * and:
 *
 *     requires memory >= required_memory;
 *
 * The first is a type constraint.
 *
 * The second is a capability requirement.
 *
 * The third is a resource requirement.
 *
 * The grammar must preserve those distinctions.
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * A type-class declaration itself does not introduce an effect.
 *
 * A method declared by a trait may carry effects through the existing effect
 * grammar.
 *
 * Therefore:
 *
 *     type class membership != execution effect
 *
 * A type can satisfy a type class without performing any operation merely by
 * being named.
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Type-class methods may have:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * contracts.
 *
 * Contract semantics remain owned by the validation/contract subsystem.
 *
 * Generic/type-class substitutions may be used during semantic contract
 * checking.
 *
 * This grammar does not evaluate contracts.
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * A type-class implementation may be subject to:
 *
 *     visibility;
 *     authorization;
 *     capability;
 *     security;
 *     deployment;
 *     resource;
 *     compatibility;
 *
 * policies.
 *
 * Policy semantics remain downstream.
 *
 * ============================================================================
 * PROVENANCE
 * ============================================================================
 *
 * Type-class semantic processing must preserve provenance for:
 *
 *     declaration source;
 *     constraint source;
 *     associated-type source;
 *     implementation source;
 *     substitution source;
 *     specialization source, if supported;
 *     generated semantic artifacts.
 *
 * The grammar itself creates no provenance records.
 *
 * Source spans must remain available to the AST/frontend layer.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * IMPORTANT:
 *
 * No new `TypeClassAst` or equivalent parallel AST is introduced here.
 *
 * Canonical mappings are:
 *
 *     typeClassDeclaration
 *         ->
 *     existing Trait AST
 *
 *     typeClassReference
 *         ->
 *     existing TypeExpr
 *
 *     typeClassConstraint
 *         ->
 *     existing TypeExpr
 *
 *     typeClassConstraintList
 *         ->
 *     ordered collection of existing TypeExpr nodes
 *
 *     typeClassAssociatedType
 *         ->
 *     existing TypeExpr / TypePath representation
 *
 * The frontend adapter remains the owner of concrete Rust AST construction.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     - declaration resolution;
 *     - type-class identity;
 *     - generic parameter binding;
 *     - bound satisfaction;
 *     - associated-type resolution;
 *     - associated-constant resolution;
 *     - supertrait resolution;
 *     - inheritance-cycle detection;
 *     - implementation conformance;
 *     - coherence;
 *     - specialization;
 *     - substitution;
 *     - inference;
 *     - ambiguity diagnostics;
 *     - visibility;
 *     - capability interpretation;
 *     - resource interpretation;
 *     - portability.
 *
 * None of these are parser decisions.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * Type-class semantics may contribute information to:
 *
 *     canonical semantic type model
 *     canonical classical IR
 *     quantum::ir
 *     HDL/hardware semantic representation
 *     distributed semantic representation
 *     future domain IR
 *
 * The lowering path remains:
 *
 *     source
 *       ->
 *     AST
 *       ->
 *     structural validation
 *       ->
 *     semantic type-class resolution
 *       ->
 *     canonical semantic model
 *       ->
 *     domain-specific lowering
 *
 * A type class must never directly choose:
 *
 *     CPU;
 *     GPU;
 *     FPGA;
 *     ASIC;
 *     QPU;
 *     accelerator;
 *     node;
 *     memory device;
 *     network topology.
 *
 * ============================================================================
 * QUANTUM IR BOUNDARY
 * ============================================================================
 *
 * If a type-class contract constrains quantum computation:
 *
 *     type class
 *         |
 *         v
 *     semantic quantum type
 *         |
 *         v
 *     quantum::ir
 *
 * There is no direct:
 *
 *     type-class.g4 -> quantum::ir
 *
 * dependency.
 *
 * ============================================================================
 * HDL BOUNDARY
 * ============================================================================
 *
 * If a type-class contract constrains HDL/hardware computation:
 *
 *     type class
 *         |
 *         v
 *     semantic hardware/HDL type
 *         |
 *         v
 *     HDL/hardware semantic representation
 *
 * No physical synthesis decision belongs here.
 *
 * ============================================================================
 * RESOURCE BOUNDARY
 * ============================================================================
 *
 * Type-class satisfaction and resource feasibility are separate:
 *
 *     type constraint
 *          |
 *          v
 *     semantic satisfaction
 *          |
 *          v
 *     capability/resource requirements
 *          |
 *          v
 *     target feasibility
 *
 * A type class cannot silently become a hardware requirement.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no actions;
 *     no semantic predicates;
 *     no randomness;
 *     no I/O;
 *     no filesystem access;
 *     no network access;
 *     no hardware discovery;
 *     no runtime execution;
 *     no mutable global state.
 *
 * Identical source token streams under identical grammar versions produce
 * identical parse structures.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar must remain declarative.
 *
 * It MUST NOT:
 *
 *     execute Rust;
 *     execute source programs;
 *     inspect the environment;
 *     inspect hardware;
 *     access the filesystem;
 *     access the network;
 *     access secrets;
 *     perform dynamic loading.
 *
 * Rust implementation remains:
 *
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * The canonical source declaration remains `trait`.
 *
 * This file does not reserve a new `typeclass` keyword.
 *
 * This is intentional.
 *
 * Existing source remains valid:
 *
 *     trait Numeric<T> { ... }
 *
 * while the semantic layer can classify the declaration as a type-class
 * contract where appropriate.
 *
 * Historical/legacy type-class syntax, if present elsewhere in the repository,
 * must be handled through:
 *
 *     grammar/compatibility/
 *
 * and must not be recreated as a second normative grammar here.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/declarations/traits.g4
 *     grammar/types/types.g4
 *
 * INDIRECTLY:
 *
 *     canonical identifiers
 *     canonical type paths
 *     canonical generic syntax
 *     canonical constraints
 *     canonical attributes
 *
 * EXPORTS:
 *
 *     typeClassDeclaration
 *     typeClassReference
 *     typeClassConstraint
 *     typeClassConstraintList
 *     typeClassProjection
 *     typeClassAssociatedType
 *
 * AST_OWNER:
 *
 *     existing frontend Trait / TypeExpr structures
 *
 * SEMANTIC_OWNER:
 *
 *     type semantic analysis
 *     trait/type-class resolution
 *     coherence
 *     associated-type resolution
 *
 * IR_OWNER:
 *
 *     canonical semantic model
 *     classical IR
 *     quantum::ir where applicable
 *     HDL/hardware semantic IR where applicable
 *
 * TEST_OWNER:
 *
 *     grammar/tests/types/
 *     grammar/tests/semantic/
 *     grammar/tests/quantum/
 *     grammar/tests/hybrid/
 *     grammar/tests/hdl/
 *
 * SPEC_OWNER:
 *
 *     grammar/specification/types.md
 *     grammar/specification/poco-reaf.md
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * Canonical parser composition must import this grammar at the type-system
 * composition layer only after the underlying declaration/type dependencies
 * are available.
 *
 * Recommended direction:
 *
 *     Zamani.g4
 *         |
 *         v
 *     ZamaniParser.g4
 *         |
 *         +--> Types
 *         |     |
 *         |     +--> TypeClasses
 *         |
 *         +--> Traits
 *
 * However, because `Traits` is already the declaration owner and already
 * imports `Types`, the repository must ensure that TypeClasses is imported
 * only once in the final ANTLR composition graph.
 *
 * The final graph MUST NOT contain competing definitions of:
 *
 *     typeExpression
 *     traitDeclaration
 *     typeConstraintClause
 *     typePath
 *
 * ============================================================================
 * REQUIRED INTEGRATION WITH TRAITS
 * ============================================================================
 *
 * `grammar/declarations/traits.g4` remains the syntax owner.
 *
 * No changes are required to its declaration rules merely to support the
 * type-class semantic model.
 *
 * The semantic layer should consume:
 *
 *     Trait
 *
 * and classify/resolve it as a type-class contract where the declaration is
 * used as such.
 *
 * This avoids reopening `traits.g4` whenever type-class semantics evolve.
 *
 * ============================================================================
 * REQUIRED INTEGRATION WITH TYPES
 * ============================================================================
 *
 * `grammar/types/types.g4` remains the public `typeExpression` owner.
 *
 * It MUST NOT import this file merely to redefine `typeExpression`.
 *
 * If a dedicated type-class parse context is required, the composing grammar
 * may expose:
 *
 *     typeClassReference
 *
 * through this delegate.
 *
 * The actual payload remains:
 *
 *     typeExpression
 *
 * ============================================================================
 * REQUIRED INTEGRATION WITH TYPE CONSTRAINTS
 * ============================================================================
 *
 * `grammar/types/type-constraints.g4` remains the owner of:
 *
 *     typeConstraintClause
 *     typeConstraintBoundList
 *     typeConstraintBound
 *
 * This file MUST NOT redefine those rules.
 *
 * The semantic pipeline should interpret a parsed bound as a type-class
 * constraint only after semantic resolution establishes that the referenced
 * type is a type-class/trait contract.
 *
 * ============================================================================
 * REQUIRED INTEGRATION WITH GENERICS
 * ============================================================================
 *
 * Generic declaration owners consume:
 *
 *     typeConstraintClause
 *
 * from the canonical constraint grammar.
 *
 * They do not need a second type-class-specific bound syntax.
 *
 * Example:
 *
 *     fn sum<T: Numeric>(value: T) -> T;
 *
 * is therefore:
 *
 *     generic parameter
 *         +
 *     type constraint
 *         +
 *     semantic type-class resolution
 *
 * ============================================================================
 * REQUIRED INTEGRATION WITH ASSOCIATED TYPES
 * ============================================================================
 *
 * Existing trait declarations already own:
 *
 *     type Item;
 *
 * The type system may consume:
 *
 *     T::Item
 *
 * through the canonical type-path representation.
 *
 * No new associated-type declaration syntax is introduced here.
 *
 * ============================================================================
 * REQUIRED INTEGRATION WITH SEMANTICS
 * ============================================================================
 *
 * The semantic layer must build a reusable relation:
 *
 *     ImplementingType
 *          |
 *          v
 *     TypeClassContract
 *          |
 *          +--> required methods
 *          +--> associated types
 *          +--> associated constants
 *          +--> supercontracts
 *          +--> type constraints
 *          +--> effects
 *          +--> capabilities
 *          +--> contracts
 *
 * This relation must remain independent of target hardware.
 *
 * ============================================================================
 * TYPE-CLASS DECLARATION ALIAS
 * ============================================================================
 *
 * This rule intentionally delegates to the canonical trait declaration.
 *
 * It exists so type-system integration points can refer to the semantic
 * category without introducing a second source declaration syntax.
 */

typeClassDeclaration
    : traitDeclaration
    ;


/*
 * ============================================================================
 * TYPE-CLASS REFERENCE
 * ============================================================================
 *
 * A type-class reference is an ordinary canonical type expression.
 *
 * Examples:
 *
 *     Numeric
 *     Comparable
 *     quantum::Observable
 *     collections::Iterable<T>
 *     future::computing::Capability
 *
 * Semantic analysis determines whether the referenced type denotes a
 * type-class contract.
 */

typeClassReference
    : typeExpression
    ;


/*
 * ============================================================================
 * SINGLE TYPE-CLASS CONSTRAINT
 * ============================================================================
 *
 * The payload is exactly one canonical type expression.
 *
 * This rule deliberately does not introduce:
 *
 *     traitName
 *     interfaceName
 *     capabilityName
 *     quantumClassName
 *     hardwareClassName
 *
 * because those distinctions belong to semantic resolution.
 */

typeClassConstraint
    : typeExpression
    ;


/*
 * ============================================================================
 * TYPE-CLASS CONSTRAINT LIST
 * ============================================================================
 *
 * Canonical conjunction:
 *
 *     Numeric + Comparable + Serializable
 *
 * Source order is preserved.
 *
 * No finite bound count is encoded.
 */

typeClassConstraintList
    : typeClassConstraint
      (
          PLUS
          typeClassConstraint
      )*
    ;


/*
 * ============================================================================
 * ASSOCIATED-TYPE PROJECTION
 * ============================================================================
 *
 * Canonical source-level projection:
 *
 *     T::Item
 *
 *     Iterator::Item
 *
 *     collection::Iterator::Item
 *
 * The first portion is represented by a canonical type path.
 *
 * The final member is represented by an identifier.
 *
 * Semantic resolution determines whether the projection is a valid associated
 * type.
 */

typeClassProjection
    : typePath
      DOUBLE_COLON
      identifier
    ;


/*
 * ============================================================================
 * ASSOCIATED TYPE REFERENCE
 * ============================================================================
 *
 * This rule exists as the explicit semantic-facing name for an associated
 * type projection.
 *
 * It deliberately delegates to the projection rule above.
 */

typeClassAssociatedType
    : typeClassProjection
    ;


/*
 * ============================================================================
 * SOURCE-ORDER CONTRACT
 * ============================================================================
 *
 * The parser/frontend adapter MUST preserve:
 *
 *     - type-class declaration source order;
 *     - generic parameter order;
 *     - constraint order;
 *     - projection path order;
 *     - source spans.
 *
 * The parser MUST NOT:
 *
 *     - sort constraints;
 *     - deduplicate constraints;
 *     - resolve type-class identities;
 *     - substitute generic parameters;
 *     - evaluate associated types;
 *     - select implementations.
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * Syntactic errors belong to parsing.
 *
 * Examples of malformed type-class constraints:
 *
 *     :
 *     +
 *     A +
 *     + A
 *     A + +
 *
 * Semantic errors remain downstream:
 *
 *     UnknownClass
 *     T: UnknownClass
 *     T::Missing
 *     T::NotAnAssociatedType
 *
 * A syntactically valid unknown name MUST NOT be rejected merely because
 * semantic resolution cannot currently find it.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar uses repetition and canonical recursive type expressions.
 *
 * There is no language-level ceiling on:
 *
 *     number of type classes;
 *     number of constraints;
 *     number of generic parameters;
 *     number of implementations;
 *     number of associated types;
 *     qualification depth;
 *     generic nesting;
 *     domain count;
 *     target count;
 *     hardware scale.
 *
 * Any operational compiler budget belongs outside the grammar.
 *
 * ============================================================================
 * CROSS-DOMAIN CONTRACT
 * ============================================================================
 *
 * A type class may constrain:
 *
 *     classical types;
 *     quantum types;
 *     hybrid types;
 *     HDL types;
 *     hardware abstractions;
 *     accelerator abstractions;
 *     distributed abstractions;
 *     networking abstractions;
 *     data abstractions;
 *     AI/ML abstractions;
 *     security abstractions;
 *     future computational abstractions.
 *
 * The grammar does not enumerate these domains.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE
 * --------
 *
 * Canonical declaration:
 *
 *     trait Numeric<T> {
 *         fn add(lhs: T, rhs: T) -> T;
 *     }
 *
 * Type-class reference:
 *
 *     Numeric
 *
 * Qualified reference:
 *
 *     numeric::Numeric
 *
 * Generic reference:
 *
 *     collections::Iterable<Value>
 *
 * Constraint:
 *
 *     Numeric + Comparable
 *
 * Quantum constraint:
 *
 *     quantum::Observable + quantum::Measurable
 *
 * Hardware constraint:
 *
 *     hardware::Signal
 *
 * Associated type:
 *
 *     T::Item
 *
 * Qualified associated type:
 *
 *     collections::Iterator::Item
 *
 * Nested generic:
 *
 *     collections::Iterable<collections::Iterable<T>>
 *
 * NEGATIVE
 * --------
 *
 * The following are structurally invalid when parsed through a
 * typeClassConstraintList context:
 *
 *     :
 *     +
 *     A +
 *     + A
 *     A + +
 *
 * Malformed projections:
 *
 *     ::Item
 *     T::
 *     T:::Item
 *
 * SEMANTIC-NEGATIVE
 * -----------------
 *
 * These may parse but must fail semantic validation when no corresponding
 * declaration exists:
 *
 *     UnknownClass
 *
 *     UnknownClass<T>
 *
 *     T::Missing
 *
 *     UnknownNamespace::Class
 *
 * BOUNDARY
 * --------
 *
 *     one constraint;
 *     multiple constraints;
 *     deeply qualified names;
 *     nested generic references;
 *     associated-type projections;
 *     quantum type-class references;
 *     HDL type-class references;
 *     resource-related type abstractions;
 *     hybrid source types.
 *
 * SCALABILITY
 * -----------
 *
 * Tests may increase:
 *
 *     constraint count;
 *     generic nesting;
 *     qualification depth;
 *     declaration count;
 *     associated-type count;
 *
 * without changing the grammar.
 *
 * No test may assert a universal machine/resource ceiling.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Given:
 *
 *     identical source;
 *     identical lexical vocabulary;
 *     identical grammar version;
 *     identical parser configuration;
 *
 * the parser must produce an equivalent parse tree.
 *
 * No runtime state participates.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden:
 *
 *     MAX_TYPE_CLASSES
 *     MAX_TYPE_CLASS_BOUNDS
 *     MAX_ASSOCIATED_TYPES
 *     MAX_ASSOCIATED_CONSTANTS
 *     MAX_IMPLEMENTATIONS
 *     MAX_GENERIC_ARITY
 *     MAX_GENERIC_DEPTH
 *     MAX_TYPE_DEPTH
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
 *
 * Also forbidden:
 *
 *     vendor-specific type-class lists;
 *     fixed hardware inventories;
 *     finite quantum resource inventories;
 *     physical device identifiers;
 *     physical topology;
 *     backend selection;
 *     implementation-specific ABI rules.
 *
 * ============================================================================
 * RUST INTEGRATION
 * ============================================================================
 *
 * This file contains no Rust code.
 *
 * Rust 1.97 / 1.97.1 is the compiler/frontend implementation baseline.
 *
 * Generated parser integration must remain safe Rust.
 *
 * No unsafe implementation is required or permitted as part of this grammar.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [x] It uses the canonical ZamaniLexer vocabulary.
 *
 * [x] It contains parser rules only.
 *
 * [x] It contains no lexer rules.
 *
 * [x] It does not introduce a new type-class keyword.
 *
 * [x] It does not duplicate trait declaration syntax.
 *
 * [x] It does not duplicate typeExpression.
 *
 * [x] It does not duplicate generic declaration syntax.
 *
 * [x] It does not duplicate type constraint syntax.
 *
 * [x] It does not duplicate associated-type declaration syntax.
 *
 * [x] It preserves open-world type-class references.
 *
 * [x] It preserves source order.
 *
 * [x] It introduces no hardware limits.
 *
 * [x] It introduces no quantum limits.
 *
 * [x] It introduces no machine-size limits.
 *
 * [x] It introduces no resource-size limits.
 *
 * [x] It introduces no target selection.
 *
 * [x] It introduces no IR construction.
 *
 * [x] It introduces no unsafe Rust requirement.
 *
 * [x] It provides explicit AST integration.
 *
 * [x] It provides explicit semantic integration.
 *
 * [x] It provides explicit IR boundaries.
 *
 * [x] It provides explicit quantum/HDL boundaries.
 *
 * [x] It provides explicit diagnostics expectations.
 *
 * [x] It provides positive/negative/boundary/scalability tests.
 *
 * [x] It defines its dependency and ownership boundaries.
 *
 * ============================================================================
 * FINAL RULE
 * ============================================================================
 *
 * Type classes in Zamani are NOT a second language.
 *
 * They are a semantic interpretation of reusable source-level type contracts.
 *
 * The canonical source declaration is:
 *
 *     trait
 *
 * The canonical type representation is:
 *
 *     TypeExpr
 *
 * The canonical type constraint representation is:
 *
 *     type constraint -> TypeExpr
 *
 * The semantic system determines whether that contract behaves as a type
 * class, interface, capability contract, refinement, or another supported
 * type relation.
 *
 * This keeps the language:
 *
 *     domain-neutral;
 *     open-world;
 *     target-independent;
 *     quantum-safe;
 *     hardware-neutral;
 *     scalable;
 *     deterministic;
 *     POCO-REAF compatible.
 *
 * ============================================================================
 */

parser grammar TypeClasses;

options {
    tokenVocab = ZamaniLexer;
}

import
    Traits;
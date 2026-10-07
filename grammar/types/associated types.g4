/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/associated-types.g4
 *
 * Grammar:
 *     AssociatedTypes
 *
 * Status:
 *     PRODUCTION SOURCE-LEVEL ASSOCIATED-TYPE PROJECTION DELEGATE
 *
 * Compiler baseline:
 *     Rust 1.97+
 *
 * Rust edition:
 *     2021
 *
 * Safety:
 *     This grammar contains no target-language actions.
 *     It contains no embedded Rust.
 *     It contains no semantic predicates.
 *     It contains no filesystem, network, runtime, or hardware access.
 *     Rust implementation consuming this grammar MUST use safe Rust only.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the canonical parser delegate for ONE associated-type
 * projection suffix.
 *
 * Canonical source form:
 *
 *     Base::Member
 *
 * This file owns only:
 *
 *     :: Member
 *
 * It deliberately does NOT own `Base`.
 *
 * The complete base is supplied by the canonical type-expression composition
 * layer.
 *
 * This separation is required to prevent recursive grammar ownership such as:
 *
 *     TypeExpression
 *         -> AssociatedTypes
 *             -> TypeExpression
 *
 * and to prevent multiple grammars from defining competing type-expression
 * systems.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     canonical lexer
 *       |
 *       v
 *     Zamani parser
 *       |
 *       v
 *     canonical typeExpression
 *       |
 *       +-----------------------------+
 *       |                             |
 *       v                             v
 *     type base                associated suffix
 *       |                             |
 *       +-------------+---------------+
 *                     |
 *                     v
 *             domain-neutral TypeExpr
 *                     |
 *                     v
 *             semantic type resolution
 *                     |
 *              +------+------+
 *              |             |
 *              v             v
 *        ordinary type   associated projection
 *                            |
 *                            v
 *                    semantic type model
 *                            |
 *                            v
 *                    canonical semantic IR
 *                            |
 *              +-------------+-------------+
 *              |             |             |
 *              v             v             v
 *          classical     quantum::ir    HDL/hardware
 *              |             |             |
 *              +-------------+-------------+
 *                            |
 *                            v
 *                    target-independent
 *                       optimization
 *                            |
 *                         lowering
 *                            |
 *                    routing/scheduling
 *                            |
 *                    resilience/QEC
 *                            |
 *                         ZQN/HAL
 *                            |
 *                    target realization
 *
 * This grammar never crosses into target realization.
 *
 * ============================================================================
 * OWNERSHIP CONTRACT
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - the reusable associated-type projection suffix;
 *     - the `DOUBLE_COLON` projection separator at the parser level;
 *     - the projected member name;
 *     - the structural source order of one projection step;
 *     - the parser boundary consumed by canonical type composition.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - the canonical `typeExpression`;
 *     - type constructors;
 *     - type paths;
 *     - ordinary qualified names;
 *     - generic declarations;
 *     - generic parameters;
 *     - generic argument lists;
 *     - type bounds;
 *     - where clauses;
 *     - type equality constraints;
 *     - type classes;
 *     - traits/interfaces;
 *     - type aliases;
 *     - type inference;
 *     - unification;
 *     - substitution;
 *     - associated-type resolution;
 *     - associated-type equality solving;
 *     - specialization;
 *     - monomorphization;
 *     - type-level evaluation;
 *     - capabilities;
 *     - resources;
 *     - effects;
 *     - contracts;
 *     - policies;
 *     - provenance;
 *     - quantum allocation;
 *     - physical qubit mapping;
 *     - HDL realization;
 *     - hardware placement;
 *     - routing;
 *     - scheduling;
 *     - calibration;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     ZamaniLexer
 *     canonical identifier vocabulary
 *     canonical DOUBLE_COLON token
 *
 * EXPORTS:
 *
 *     associatedTypeProjectionSuffix
 *     associatedTypeMember
 *
 * CONSUMED_BY:
 *
 *     grammar/types/types.g4
 *     canonical type-expression composition
 *     any parser delegate requiring one associated-type projection step
 *
 * AST_OWNER:
 *
 *     existing frontend TypeExpr
 *
 * SEMANTIC_OWNER:
 *
 *     semantic type-resolution / associated-type subsystem
 *
 * IR_OWNER:
 *
 *     canonical semantic type representation
 *     quantum::ir where a resolved associated type participates in quantum
 *     semantics
 *
 * TEST_OWNER:
 *
 *     grammar/tests/types/
 *     parser conformance tests
 *     AST/type-resolution tests
 *
 * SPEC_OWNER:
 *
 *     grammar/specification/types.md
 *     grammar/specification/poco-reaf.md
 *
 * ============================================================================
 * SINGLE TYPE-EXPRESSION AUTHORITY
 * ============================================================================
 *
 * The sole public universal source-level type expression is:
 *
 *     typeExpression
 *
 * owned by:
 *
 *     grammar/types/types.g4
 *
 * This file MUST NOT define:
 *
 *     typeExpression
 *     typeCore
 *     typePath
 *     typePathSegment
 *     namedType
 *     genericType
 *     tupleType
 *     arrayType
 *     sliceType
 *     functionType
 *     dependentType
 *     quantumType
 *     hardwareType
 *
 * The projection suffix is intentionally independent of those constructors.
 *
 * ============================================================================
 * WHY THIS FILE DOES NOT CONSUME typeExpression
 * ============================================================================
 *
 * A tempting implementation is:
 *
 *     associatedType
 *         : typeExpression DOUBLE_COLON IDENTIFIER
 *         ;
 *
 * That design is not production-safe in the current architecture because
 * `typeExpression` already orchestrates the specialized type grammars.
 *
 * It can introduce dependency cycles and duplicate recursive type ownership.
 *
 * Instead this file defines:
 *
 *     associatedTypeProjectionSuffix
 *
 * as:
 *
 *     DOUBLE_COLON associatedTypeMember
 *
 * The canonical type-expression owner attaches the suffix to an already
 * parsed type base.
 *
 * Therefore:
 *
 *     Base::Member
 *
 * is structurally assembled as:
 *
 *     typeExpression
 *          |
 *          v
 *     Base
 *          |
 *          v
 *     associatedTypeProjectionSuffix
 *          |
 *          v
 *     Member
 *
 * This is the correct compositional architecture.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This file declares NO lexer rules.
 *
 * Required tokens are supplied by the canonical lexer:
 *
 *     DOUBLE_COLON
 *     IDENTIFIER
 *
 * This file MUST NOT define:
 *
 *     DOUBLE_COLON : '::' ;
 *
 * or any identifier rule.
 *
 * Lexical ownership remains centralized.
 *
 * ============================================================================
 * BASIC ASSOCIATED TYPE
 * ============================================================================
 *
 * Canonical form:
 *
 *     T::Item
 *
 * Structural interpretation:
 *
 *     base = T
 *     member = Item
 *
 * This grammar owns only:
 *
 *     ::Item
 *
 * The semantic layer determines whether `Item` actually denotes:
 *
 *     - an associated type;
 *     - a nested type;
 *     - a member type;
 *     - a type-level projection;
 *     - a namespace member;
 *     - another valid type-level entity.
 *
 * ============================================================================
 * OPEN-WORLD MEMBER MODEL
 * ============================================================================
 *
 * Associated-type member names are open-ended.
 *
 * The grammar MUST NOT enumerate:
 *
 *     Item
 *     Output
 *     Error
 *     Element
 *     Value
 *     State
 *     Signal
 *     Memory
 *     Measurement
 *
 * or any other finite catalogue.
 *
 * A member is represented by:
 *
 *     IDENTIFIER
 *
 * Therefore a new library, domain, hardware abstraction, quantum abstraction,
 * scientific type, data abstraction, or future computational system can
 * introduce an associated type without changing this grammar.
 *
 * ============================================================================
 * QUALIFIED-NAME AMBIGUITY
 * ============================================================================
 *
 * Zamani already permits qualified type paths such as:
 *
 *     module::Type
 *     quantum::State
 *     hardware::Signal
 *     data::Dataset
 *
 * The same source punctuation may occur in:
 *
 *     T::Item
 *
 * Consequently this grammar MUST NOT claim that every `::` sequence is
 * semantically an associated-type projection.
 *
 * Parsing preserves source structure.
 *
 * Semantic resolution determines the meaning.
 *
 * This is essential for an open-world language.
 *
 * ============================================================================
 * PROJECTION CHAIN
 * ============================================================================
 *
 * Multiple projection steps are supported by repetition in the canonical
 * type-expression composition layer.
 *
 * Conceptual source:
 *
 *     A::B::C
 *
 * Canonical structural interpretation:
 *
 *     A
 *       +-- ::B
 *       +-- ::C
 *
 * Semantic resolution may normalize this into:
 *
 *     Associated(
 *         Associated(A, B),
 *         C
 *     )
 *
 * or another canonical internal representation defined by the semantic
 * type system.
 *
 * This file intentionally owns only one suffix step.
 *
 * It MUST NOT define a second recursive projection language.
 *
 * ============================================================================
 * GENERIC BASE INTEGRATION
 * ============================================================================
 *
 * Generic types can be projection bases.
 *
 * Examples:
 *
 *     Iterator<T>::Item
 *     Collection<T>::Element
 *     Container<Key, Value>::Entry
 *
 * Generic application remains owned by:
 *
 *     grammar/types/generic.g4
 *
 * The composition is:
 *
 *     generic type
 *         |
 *         v
 *     canonical typeExpression
 *         |
 *         v
 *     associatedTypeProjectionSuffix
 *
 * This file does not parse generic arguments.
 *
 * ============================================================================
 * GENERIC PARAMETER INTEGRATION
 * ============================================================================
 *
 * Generic parameters may be projection bases:
 *
 *     T::Item
 *     T::Output
 *     T::Error
 *
 * Whether `T` actually provides that member is a semantic question.
 *
 * The declaration of `T` and its bounds are owned elsewhere.
 *
 * For example:
 *
 *     T: Iterable
 *
 * may semantically imply that:
 *
 *     T::Item
 *
 * is available.
 *
 * This grammar does not establish that relationship.
 *
 * ============================================================================
 * ASSOCIATED-TYPE BOUNDS
 * ============================================================================
 *
 * Associated-type constraints belong to the type-constraint subsystem.
 *
 * Examples:
 *
 *     T::Item: Serializable
 *
 *     T::Output: Numeric
 *
 *     T::Error: Recoverable
 *
 * The parser component responsible for constraints consumes a complete
 * canonical type expression.
 *
 * This file contributes only the projection structure.
 *
 * ============================================================================
 * ASSOCIATED-TYPE EQUALITY
 * ============================================================================
 *
 * Relationships such as:
 *
 *     T::Item = U
 *
 *     T::Output = Result
 *
 *     T::Error = E
 *
 * are semantic constraints.
 *
 * They MUST NOT be solved by this grammar.
 *
 * The parser preserves enough structure for the constraint subsystem to
 * construct the appropriate semantic relation.
 *
 * ============================================================================
 * ASSOCIATED TYPES AND TYPE CLASSES
 * ============================================================================
 *
 * Type classes, traits, interfaces, or equivalent abstractions may declare
 * associated types.
 *
 * This file does not own those declarations.
 *
 * The semantic resolver determines:
 *
 *     declaration owner;
 *     implementation;
 *     substitution;
 *     inherited requirements;
 *     associated-type defaults;
 *     equality constraints;
 *     satisfiability.
 *
 * ============================================================================
 * DEPENDENT TYPES
 * ============================================================================
 *
 * An associated type may participate in dependent/value-parameterized types.
 *
 * Examples of semantic possibilities include:
 *
 *     Buffer<T::Element>[N]
 *
 *     Matrix<T::Scalar>[Rows, Columns]
 *
 *     Register<T::State>
 *
 * The value-level expression and dependent-type syntax remain owned by the
 * canonical dependent-type grammar.
 *
 * This file does not interpret:
 *
 *     N
 *     Rows
 *     Columns
 *
 * as machine-sized integers.
 *
 * ============================================================================
 * LINEAR AND AFFINE TYPES
 * ============================================================================
 *
 * Associated projections may appear inside linear or affine types.
 *
 * Examples:
 *
 *     linear Resource<T::Handle>
 *
 *     affine Buffer<T::Element>
 *
 * Ownership/usage checking remains semantic.
 *
 * This grammar does not track ownership or consumption.
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * Type projection itself introduces no execution effect.
 *
 * It does not:
 *
 *     perform IO;
 *     access a network;
 *     measure a quantum state;
 *     allocate a resource;
 *     invoke an external function;
 *     execute learning;
 *     perform adaptation;
 *     execute reflection.
 *
 * Any effectful semantic operation involving the resolved type is handled by
 * the existing effect system.
 *
 * ============================================================================
 * CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Associated types can represent capability-related semantic types.
 *
 * Example:
 *
 *     T::Capability
 *
 * does not itself acquire or verify a capability.
 *
 * Capability resolution remains downstream.
 *
 * This distinction is mandatory:
 *
 *     syntax
 *         !=
 *     capability satisfaction
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * An associated type may describe a resource-related abstraction:
 *
 *     T::Memory
 *     T::Accelerator
 *     T::Buffer
 *
 * The grammar does not inspect the target environment.
 *
 * It does not determine:
 *
 *     available memory;
 *     processor count;
 *     accelerator count;
 *     device count;
 *     network capacity;
 *     QPU capacity;
 *     physical topology.
 *
 * Resource analysis is downstream.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum types can contain or produce associated projections.
 *
 * Conceptual examples:
 *
 *     quantum::Circuit::State
 *     Circuit<T>::Measurement
 *     Register<T>::LogicalState
 *
 * The grammar does not enumerate quantum types or operations.
 *
 * It does not:
 *
 *     allocate qubits;
 *     identify physical qubits;
 *     choose a QPU;
 *     choose topology;
 *     perform routing;
 *     perform decomposition;
 *     schedule operations;
 *     select calibration;
 *     select QEC.
 *
 * After semantic resolution, a quantum-relevant type may participate in:
 *
 *     semantic quantum model
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
 * No quantum implementation detail belongs in this grammar.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Associated projections may represent abstract hardware/software interfaces:
 *
 *     hardware::Module::Signal
 *     hardware::Module::Clock
 *     Accelerator::Memory
 *     Device::Interface
 *
 * The grammar remains independent of:
 *
 *     physical pin numbers;
 *     fixed bus widths;
 *     device counts;
 *     topology sizes;
 *     vendor implementation details.
 *
 * ============================================================================
 * AI / DATA INTEGRATION
 * ============================================================================
 *
 * Associated types can naturally represent:
 *
 *     Model::Input
 *     Model::Output
 *     Dataset::Element
 *     Distribution::Sample
 *     Knowledge::Evidence
 *
 * These remain ordinary type-level constructs.
 *
 * No application-specific keyword catalogue is needed.
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Associated projections may occur inside:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * expressions where the surrounding grammar accepts a type expression.
 *
 * This file does not own contracts.
 *
 * Contract semantics remain under:
 *
 *     grammar/validation/
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Policies may refer semantically to types containing associated projections.
 *
 * This file does not define:
 *
 *     permissions;
 *     prohibitions;
 *     preferences;
 *     fallbacks;
 *     deployment policies.
 *
 * Those belong to the policy subsystem.
 *
 * ============================================================================
 * PROVENANCE INTEGRATION
 * ============================================================================
 *
 * The source span of:
 *
 *     ::Member
 *
 * MUST be preserved by the frontend.
 *
 * Provenance systems may then identify:
 *
 *     source declaration;
 *     projected member;
 *     resolution decision;
 *     substitution;
 *     normalization;
 *     specialization.
 *
 * This grammar does not generate provenance records.
 *
 * ============================================================================
 * METAPROGRAMMING INTEGRATION
 * ============================================================================
 *
 * Reflection and type-level metaprogramming may inspect associated types.
 *
 * Examples:
 *
 *     reflect(T::Item)
 *
 *     type_of(T::Output)
 *
 * Such operations belong to:
 *
 *     grammar/metaprogramming/
 *
 * and the semantic type system.
 *
 * They MUST consume the canonical TypeExpr representation rather than create
 * another associated-type AST.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar contributes the structural information required for the
 * existing canonical AST representation.
 *
 * Existing semantic/frontend representation:
 *
 *     TypeExpr::Associated {
 *         base: Box<TypeExpr>,
 *         member: TypeName,
 *     }
 *
 * The associated suffix itself MUST NOT introduce:
 *
 *     AssociatedTypeExpr
 *     AssociatedTypeAst
 *     TypeProjectionAst
 *     AssociatedTypeIR
 *     TypeProjectionIR
 *
 * as competing representations.
 *
 * The canonical frontend remains TypeExpr.
 *
 * ============================================================================
 * AST LOWERING
 * ============================================================================
 *
 * Given:
 *
 *     Base::Member
 *
 * the canonical type-expression composer provides:
 *
 *     Base
 *
 * and this delegate provides:
 *
 *     Member
 *
 * The semantic/AST lowering layer combines them into the existing:
 *
 *     TypeExpr::Associated
 *
 * representation.
 *
 * The grammar does not construct Rust values directly.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     - whether the base type is valid;
 *     - whether the member exists;
 *     - whether the member is an associated type;
 *     - which declaration owns the member;
 *     - whether the projection is permitted;
 *     - whether generic substitutions are available;
 *     - whether associated bounds are satisfied;
 *     - whether equality constraints are satisfied;
 *     - whether normalization succeeds;
 *     - whether the resulting type is well formed.
 *
 * Syntactic acceptance MUST NOT imply semantic resolution.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * This file is part of the type-language syntax.
 *
 * It supports compositional type expressions without imposing:
 *
 *     fixed generic arity;
 *     fixed projection depth;
 *     fixed namespace depth;
 *     fixed type nesting depth;
 *     fixed associated-member count.
 *
 * The grammar uses repetition in the enclosing type composition rather than
 * finite enumerations.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * There is no language-level maximum for:
 *
 *     projection-chain length;
 *     generic nesting;
 *     type nesting;
 *     namespace depth;
 *     number of declarations;
 *     number of associated types;
 *     number of domains;
 *     number of target types.
 *
 * This grammar MUST NOT introduce:
 *
 *     MAX_ASSOCIATED_TYPES
 *     MAX_PROJECTION_DEPTH
 *     MAX_TYPE_DEPTH
 *     MAX_GENERIC_ARITY
 *
 * or any equivalent universal ceiling.
 *
 * Practical compiler protection against resource exhaustion may exist outside
 * the grammar as explicit implementation configuration.
 *
 * Such protection MUST:
 *
 *     - be configurable where appropriate;
 *     - be diagnosable;
 *     - not redefine source-language semantics;
 *     - not depend on a particular physical target;
 *     - not silently truncate source structure.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Associated types describe source-level abstraction.
 *
 * They MUST remain independent of physical realization.
 *
 * The same source-level type relationship can therefore participate in
 * compilation for:
 *
 *     tiny systems;
 *     embedded systems;
 *     CPUs;
 *     multicore systems;
 *     GPUs;
 *     FPGAs;
 *     ASICs;
 *     accelerators;
 *     QPUs;
 *     simulators;
 *     HPC systems;
 *     clusters;
 *     distributed systems;
 *     cloud systems;
 *     future computational substrates.
 *
 * A target's inability to realize a resolved type requirement is a semantic,
 * resource, capability, or target-feasibility issue—not an associated-type
 * grammar error.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source;
 *     lexical configuration;
 *     language version;
 *     grammar version.
 *
 * It MUST NOT depend on:
 *
 *     target hardware;
 *     runtime state;
 *     wall-clock time;
 *     randomness;
 *     filesystem state;
 *     network state;
 *     resource availability.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * This delegate owns structural errors involving the projection suffix.
 *
 * Invalid:
 *
 *     T::
 *     T::+
 *     T::123
 *     T::*
 *
 * when those tokens do not constitute a valid identifier.
 *
 * The following are NOT grammar errors:
 *
 *     T::Unknown
 *     Unknown<T>::Member
 *     future::Domain::Member
 *
 * if their syntax is otherwise valid.
 *
 * Those are resolved semantically.
 *
 * ============================================================================
 * BOUNDARY CONDITIONS
 * ============================================================================
 *
 * Valid structural projection:
 *
 *     T::Item
 *
 * Valid chained projection:
 *
 *     T::Item::Output
 *
 * Valid qualified projection:
 *
 *     module::Type::Item
 *
 * Valid generic-base projection:
 *
 *     Container<T>::Item
 *
 * Valid nested generic projection:
 *
 *     Container<Result<T, E>>::Item
 *
 * Valid domain-neutral projection:
 *
 *     future::domain::Type::Associated
 *
 * Invalid:
 *
 *     T::
 *
 * Invalid:
 *
 *     T::+
 *
 * Invalid:
 *
 *     T::123
 *
 * Invalid:
 *
 *     T::Item::
 *
 * when the enclosing type-expression requires another complete member.
 *
 * ============================================================================
 * IMPORTANT COMPOSITION RULE
 * ============================================================================
 *
 * The enclosing canonical type grammar MUST decide whether a projection
 * suffix is allowed after the preceding type expression.
 *
 * This delegate MUST NOT consume arbitrary preceding expressions.
 *
 * In particular, it MUST NOT use:
 *
 *     .* 
 *
 * or another catch-all construct to locate the base.
 *
 * ============================================================================
 * NO SEMANTIC PREDICATES
 * ============================================================================
 *
 * This grammar MUST remain context-neutral.
 *
 * It MUST NOT use semantic predicates to ask questions such as:
 *
 *     "Is this identifier a trait?"
 *     "Does this type have an associated member?"
 *     "Is this namespace a module?"
 *     "Does this hardware target provide the type?"
 *
 * Those questions belong to semantic analysis.
 *
 * ============================================================================
 * NO TARGET COUPLING
 * ============================================================================
 *
 * This grammar must never inspect:
 *
 *     CPU count;
 *     GPU count;
 *     FPGA availability;
 *     accelerator availability;
 *     memory;
 *     QPU count;
 *     qubit availability;
 *     topology;
 *     network size;
 *     deployment state;
 *     runtime state.
 *
 * ============================================================================
 * NO FIXED DOMAIN INVENTORY
 * ============================================================================
 *
 * The grammar MUST NOT enumerate:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     tensor systems
 *     AI models
 *     database systems
 *     network systems
 *     future hardware
 *
 * Domain types remain open through normal type names and semantic
 * registration.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing source forms such as:
 *
 *     module::Type
 *     quantum::State
 *     hardware::Signal
 *
 * MUST remain parseable.
 *
 * Introducing this delegate MUST NOT force existing qualified names to become
 * semantically associated types.
 *
 * Semantic resolution preserves that distinction.
 *
 * The existing:
 *
 *     grammar/types/associated.g4
 *
 * MUST NOT remain an independent competing implementation.
 *
 * Migration policy:
 *
 *     associated.g4
 *          |
 *          v
 *     compatibility/deprecated facade
 *          |
 *          v
 *     associated-types.g4
 *
 * If no active consumer requires the old grammar name, it should be removed
 * after the compatibility window defined by the repository version policy.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * Canonical integration into grammar/types/types.g4:
 *
 *     typeExpression
 *         : typePrefix*
 *           typeCore
 *           typePostfix*
 *         ;
 *
 * The canonical type composition layer should attach:
 *
 *     associatedTypeProjectionSuffix
 *
 * through `typePostfix` or the repository's equivalent canonical postfix
 * boundary.
 *
 * Conceptually:
 *
 *     typeExpression
 *         : typePrefix*
 *           typeCore
 *           typePostfix*
 *         ;
 *
 *     typePostfix
 *         : associatedTypeProjectionSuffix
 *         | QUESTION_MARK
 *         ;
 *
 * This makes:
 *
 *     T::Item
 *
 * structurally:
 *
 *     T
 *     ::Item
 *
 * without creating a second type-expression grammar.
 *
 * ============================================================================
 * GENERIC INTEGRATION
 * ============================================================================
 *
 * `generic.g4` remains the owner of generic argument syntax.
 *
 * Do NOT duplicate:
 *
 *     genericArgumentList
 *     genericTypeArguments
 *     genericTypeApplicationSuffix
 *
 * here.
 *
 * ============================================================================
 * CONSTRAINT INTEGRATION
 * ============================================================================
 *
 * `type-constraints.g4` remains the owner of reusable type-bound syntax.
 *
 * Associated projections may appear as the type expression within a bound:
 *
 *     T::Item: Serializable
 *
 * or as a constraint subject where supported.
 *
 * This file does not import or redefine the constraint language.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * A projection can participate in type relationships involving:
 *
 *     requirements;
 *     capabilities;
 *     resources;
 *     policies;
 *     contracts.
 *
 * Example semantic relationship:
 *
 *     T::Buffer: ResourceBacked
 *
 * The grammar does not resolve ResourceBacked or inspect available resources.
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * Type projection has no execution effect.
 *
 * Effect declarations remain owned by:
 *
 *     grammar/effects/
 *
 * ============================================================================
 * IR INTEGRATION
 * ============================================================================
 *
 * This grammar produces no IR.
 *
 * Its semantic output eventually contributes to the canonical semantic type
 * representation.
 *
 * If the resolved type is relevant to quantum computation, the semantic layer
 * may carry it toward:
 *
 *     quantum::ir
 *
 * No second quantum IR is introduced here.
 *
 * ============================================================================
 * HARDWARE / HDL BOUNDARY
 * ============================================================================
 *
 * Hardware and HDL semantic consumers may use associated projections for
 * abstract interfaces:
 *
 *     Module::Signal
 *     Module::Clock
 *     Device::Memory
 *
 * They must not interpret this grammar as physical allocation or realization.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive parser fixtures:
 *
 *     T::Item
 *     T::Output
 *     Iterator::Item
 *     Container<T>::Item
 *     Container<Key, Value>::Entry
 *     module::Type::Member
 *     quantum::Circuit::State
 *     hardware::Module::Signal
 *     future::domain::Type::Associated
 *
 * Negative parser fixtures:
 *
 *     T::
 *     T::+
 *     T::123
 *     T::*
 *     T::Item::
 *
 * Semantic-negative fixtures:
 *
 *     T::Unknown
 *
 * when `Unknown` is not declared.
 *
 * These MUST parse structurally and fail at semantic resolution rather than
 * at lexical/parser level.
 *
 * Boundary fixtures:
 *
 *     A::B::C
 *     A<B>::C
 *     A<B<C>>::D
 *     module::A<B>::C
 *     quantum::Circuit<T>::State
 *     hardware::Module<T>::Signal
 *
 * Scalability fixtures:
 *
 *     long projection chains;
 *     deeply nested generic bases;
 *     symbolic type arguments;
 *     source-defined associated member names;
 *     large cross-domain type compositions.
 *
 * Tests MUST be generated/configured according to available test resources.
 * No source-language test may assume a fixed universal maximum.
 *
 * Determinism:
 *
 * identical source + identical language configuration
 *     =>
 * identical parse structure.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [x] It owns only the associated projection suffix.
 *     [x] It does not define `typeExpression`.
 *     [x] It does not recursively consume `typeExpression`.
 *     [x] It does not define lexer rules.
 *     [x] It uses the canonical identifier token.
 *     [x] It uses the canonical DOUBLE_COLON token.
 *     [x] It permits arbitrary source-defined member names.
 *     [x] It supports composition with generic types.
 *     [x] It supports composition with qualified types.
 *     [x] It supports arbitrary projection chains through the enclosing
 *         canonical type grammar.
 *     [x] It imposes no hardware ceiling.
 *     [x] It imposes no quantum ceiling.
 *     [x] It imposes no generic arity ceiling.
 *     [x] It imposes no projection-depth ceiling.
 *     [x] It introduces no semantic predicates.
 *     [x] It introduces no target coupling.
 *     [x] It introduces no execution effects.
 *     [x] It introduces no resource allocation.
 *     [x]] It introduces no competing AST.
 *     [x] It introduces no competing IR.
 *     [x] It preserves the domain-neutral TypeExpr boundary.
 *     [x] It is compatible with Rust 1.97+ safe-Rust implementation.
 *     [x] It has explicit downstream integration contracts.
 *
 * Repository-level completion additionally requires:
 *
 *     [ ] types.g4 imports this delegate exactly once.
 *     [ ] types.g4 attaches the suffix through the canonical postfix boundary.
 *     [ ] the old associated.g4 no longer competes as an active owner.
 *     [ ] duplicate associated projection rules are removed from active
 *         grammars.
 *     [ ] AST lowering targets the existing TypeExpr::Associated node.
 *     [ ] semantic associated-type resolution is implemented.
 *     [ ] associated-type equality constraints are implemented where specified.
 *     [ ] positive/negative/boundary/scalability tests pass.
 *     [ ] ANTLR generation succeeds.
 *     [ ] Rust 1.97+ frontend conformance succeeds.
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar AssociatedTypes;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * PUBLIC RULE
 * ============================================================================
 *
 * Exactly ONE projection step.
 *
 * The preceding type is owned by the canonical type-expression grammar.
 *
 * Example:
 *
 *     T::Item
 *      ^
 *      |
 *      this rule consumes "::Item"
 *
 * ============================================================================
 */

associatedTypeProjectionSuffix
    : DOUBLE_COLON associatedTypeMember
    ;


/*
 * ============================================================================
 * PROJECTED MEMBER
 * ============================================================================
 *
 * Associated-type names are open-world identifiers.
 *
 * No predefined associated-type catalogue is permitted.
 *
 * ============================================================================
 */

associatedTypeMember
    : IDENTIFIER
    ;
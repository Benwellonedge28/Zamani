/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/path-dependent-types.g4
 *
 * Grammar:
 *     PathDependentTypes
 *
 * Status:
 *     PRODUCTION TYPE-SYSTEM PATH-DEPENDENT-TYPE COMPONENT
 *
 * Compiler baseline:
 *     Rust 1.97+
 *     Rust 2021
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * Define the reusable source syntax for path-dependent / singleton types.
 *
 * Canonical forms:
 *
 *     value.type
 *     self.type
 *     this.type
 *     object.field.type
 *     module::value.type
 *     module::object.field.type
 *
 * A path-dependent type represents a type whose identity or meaning depends
 * upon a particular stable source-level value path.
 *
 * This is intentionally different from:
 *
 *     T::Item
 *
 * which is an associated-type/member projection.
 *
 * It is also different from:
 *
 *     value.field
 *
 * which is ordinary expression member access.
 *
 * The explicit `.type` terminator provides a syntactic boundary that lets
 * semantic analysis distinguish a path-dependent type from ordinary member
 * access without adding another lexer token.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * The intended pipeline is:
 *
 *     source
 *       |
 *       v
 *     canonical lexer
 *       |
 *       v
 *     ANTLR parser
 *       |
 *       v
 *     pathDependentType
 *       |
 *       v
 *     domain-neutral TypeExpr
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     value/path resolution
 *       |
 *       v
 *     path-dependent type resolution
 *       |
 *       v
 *     semantic type model
 *       |
 *       +---------------------------+
 *       |                           |
 *       v                           v
 *   classical                  quantum semantics
 *       |                           |
 *       +-------------+-------------+
 *                     |
 *                     v
 *              canonical semantic IR
 *                     |
 *             +-------+-------+
 *             |               |
 *             v               v
 *        classical IR     quantum::ir
 *             |
 *             v
 *       target-independent
 *          optimization
 *             |
 *             v
 *       lowering / routing
 *             |
 *             v
 *          scheduling
 *             |
 *             v
 *       resilience / recovery
 *             |
 *             v
 *             ZQN
 *             |
 *             v
 *             HAL
 *             |
 *             v
 *       target realization
 *
 * Path-dependent types are entirely source/type-system constructs.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     pathDependentType
 *     pathDependentValuePath
 *     pathDependentPathRoot
 *     pathDependentPathSegment
 *     pathDependentMemberSegment
 *
 * It owns the source-level syntactic boundary:
 *
 *     <stable-value-path>.type
 *
 * THIS FILE DOES NOT OWN:
 *
 *     typeExpression
 *     typeCore
 *     typePath
 *     typePathSegment
 *     qualifiedName
 *     genericParameter
 *     genericArgument
 *     genericTypeApplication
 *     associatedType
 *     associatedTypeProjectionSuffix
 *     memberAccessSuffix
 *     expression
 *     postfixExpression
 *     value declarations
 *     variable declarations
 *     trait declarations
 *     type-class declarations
 *     dependent Pi/Sigma declarations
 *     type families
 *     type inference
 *     name resolution
 *     path stability analysis
 *     ownership analysis
 *     lifetime analysis
 *     trait resolution
 *     generic substitution
 *     unification
 *     specialization
 *     coherence
 *     resource negotiation
 *     capability negotiation
 *     effects
 *     policies
 *     contracts
 *     quantum operations
 *     HDL constructs
 *     backend lowering
 *     target realization
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     canonical identifier vocabulary
 *     canonical TYPE token
 *     canonical DOT token
 *     canonical DOUBLE_COLON token
 *     grammar/types/types.g4
 *     grammar/types/associated.g4
 *     grammar/expressions/member-access.g4
 *
 * EXPORTS:
 *
 *     pathDependentType
 *     pathDependentValuePath
 *     pathDependentPathRoot
 *     pathDependentPathSegment
 *     pathDependentMemberSegment
 *
 * CONSUMED_BY:
 *
 *     grammar/types/types.g4
 *     canonical type-expression composition
 *     semantic type resolution
 *     frontend AST construction
 *     type validation
 *
 * AST_OWNER:
 *
 *     src/frontend/ast/node/types/type_expr.rs
 *
 * SEMANTIC_OWNER:
 *
 *     canonical semantic type/path-resolution subsystem
 *
 * IR_OWNER:
 *
 *     canonical semantic type model and downstream domain IR lowering
 *
 * TEST_OWNER:
 *
 *     grammar/tests/types/
 *     semantic/type tests
 *     frontend AST tests
 *
 * SPEC_OWNER:
 *
 *     grammar/specification/types.md
 *     grammar/specification/dependent-types.md
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This file contains NO lexer rules.
 *
 * It consumes only existing canonical lexical tokens:
 *
 *     IDENTIFIER
 *     SELF
 *     THIS
 *     TYPE
 *     DOT
 *     DOUBLE_COLON
 *
 * The spelling:
 *
 *     .type
 *
 * uses the existing:
 *
 *     TYPE
 *
 * keyword.
 *
 * No new token such as:
 *
 *     PATH_TYPE
 *     SINGLETON_TYPE
 *     PATH_DEPENDENT_TYPE
 *     TYPE_OF_PATH
 *
 * may be introduced by this file.
 *
 * ============================================================================
 * WHY `.type`
 * ============================================================================
 *
 * Zamani already uses:
 *
 *     ::   for qualified paths / associated projections
 *     .    for ordinary member access
 *
 * Therefore a path-dependent type cannot safely be introduced as another
 * unrestricted `Base::Member` form without colliding with:
 *
 *     associated.g4
 *     typePath
 *     member-access.g4
 *
 * The canonical distinction is:
 *
 *     T::Item
 *         -> associated/member type projection
 *
 *     value.field
 *         -> ordinary expression member access
 *
 *     value.field.type
 *         -> path-dependent / singleton type
 *
 * This keeps each existing syntax owner intact.
 *
 * ============================================================================
 * PATH MODEL
 * ============================================================================
 *
 * The path-dependent base is deliberately a restricted stable path.
 *
 * It is NOT an arbitrary expression.
 *
 * Valid structural examples:
 *
 *     value
 *     self
 *     this
 *     value.field
 *     object.field.subfield
 *     module::value
 *     module::object.field
 *     module::object.field.subfield
 *
 * Invalid structural examples include arbitrary computations:
 *
 *     (a + b).type
 *     make_value().type
 *     if condition { a } else { b }.type
 *     [a, b][0].type
 *
 * Whether a particular stable path is semantically eligible is determined
 * downstream.
 *
 * The grammar deliberately preserves the distinction between:
 *
 *     syntactic path
 *
 * and:
 *
 *     semantic stability.
 *
 * ============================================================================
 * PATH ROOT
 * ============================================================================
 *
 * A path may begin with:
 *
 *     IDENTIFIER
 *     SELF
 *     THIS
 *
 * `SELF` and `THIS` are already canonical Zamani lexer tokens.
 *
 * No fixed list of variable names is encoded.
 *
 * ============================================================================
 * PATH SEGMENTS
 * ============================================================================
 *
 * A path may contain arbitrarily many source-level segments:
 *
 *     a
 *     a.b
 *     a.b.c
 *     a.b.c.d
 *
 * or qualified components:
 *
 *     module::a
 *     module::a.b
 *     module::a.b.c
 *
 * No maximum path depth is defined.
 *
 * Practical parser/compiler protection remains an implementation/resource
 * policy rather than a language-level limit.
 *
 * ============================================================================
 * IMPORTANT: NO GENERIC SYNTAX HERE
 * ============================================================================
 *
 * This file does not define:
 *
 *     <T>
 *     Vec<T>
 *     Map<K, V>
 *
 * Generic syntax remains owned by:
 *
 *     grammar/types/generic.g4
 *
 * A future extension may permit generic/member paths where the canonical type
 * and expression systems can represent them without creating a duplicate
 * generic grammar.
 *
 * Such an extension must be integrated through the canonical path/type
 * composition layer rather than by copying genericArgumentList here.
 *
 * ============================================================================
 * IMPORTANT: NO ASSOCIATED-TYPE DUPLICATION
 * ============================================================================
 *
 * This file MUST NOT define:
 *
 *     Base::Member
 *
 * as a path-dependent type.
 *
 * That syntax already belongs to:
 *
 *     grammar/types/associated.g4
 *     grammar/types/types.g4
 *     grammar/expressions/member-access.g4
 *
 * The distinction is:
 *
 *     Base::Member
 *         -> qualified/associated selection
 *
 *     value.type
 *         -> path-dependent/singleton type
 *
 * Semantic analysis may later determine that a particular associated type is
 * path-dependent, but that does not change parser ownership.
 *
 * ============================================================================
 * IMPORTANT: NO EXPRESSION DUPLICATION
 * ============================================================================
 *
 * This grammar MUST NOT import or redefine the full expression grammar.
 *
 * In particular, it must not define:
 *
 *     expression
 *     postfixExpression
 *     primaryExpression
 *     callExpression
 *     memberAccessExpression
 *     indexingExpression
 *
 * A path-dependent type uses a restricted source-level value path precisely
 * to avoid creating a parser cycle:
 *
 *     Type
 *       -> PathDependent
 *       -> Expression
 *       -> Type
 *
 * ============================================================================
 * PUBLIC RULE
 * ============================================================================
 *
 * The principal reusable rule is:
 *
 *     pathDependentType
 *
 * Conceptual syntax:
 *
 *     <path>.type
 *
 * Examples:
 *
 *     value.type
 *     self.type
 *     this.type
 *     object.field.type
 *     module::value.type
 *
 * ============================================================================
 */

parser grammar PathDependentTypes;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. PATH-DEPENDENT TYPE
 * ============================================================================
 *
 * Canonical syntax:
 *
 *     path.type
 *
 * The `.type` suffix is mandatory.
 *
 * The path is syntactically restricted to a stable-path-shaped sequence.
 */
pathDependentType
    : pathDependentValuePath
      DOT
      TYPE
    ;


/*
 * ============================================================================
 * 2. VALUE PATH
 * ============================================================================
 *
 * A value path contains one or more source-level path components.
 *
 * Examples:
 *
 *     value
 *     object.field
 *     module::object.field
 *
 * The path grammar is intentionally structural.
 *
 * It does not decide whether the referenced value exists or is stable.
 */
pathDependentValuePath
    : pathDependentPathRoot
      pathDependentPathSegment*
    ;


/*
 * ============================================================================
 * 3. PATH ROOT
 * ============================================================================
 *
 * The first component may be an ordinary identifier or one of the canonical
 * self-reference keywords.
 */
pathDependentPathRoot
    : IDENTIFIER
    | SELF
    | THIS
    ;


/*
 * ============================================================================
 * 4. PATH SEGMENT
 * ============================================================================
 *
 * A segment is either:
 *
 *     ::name
 *
 * or:
 *
 *     .name
 *
 * The final `.type` is NOT consumed here.
 *
 * Therefore:
 *
 *     object.field.type
 *
 * is parsed as:
 *
 *     pathDependentValuePath
 *         object
 *         .field
 *
 * followed by:
 *
 *     .type
 */
pathDependentPathSegment
    : DOUBLE_COLON
      pathDependentMemberSegment
    | DOT
      pathDependentMemberSegment
    ;


/*
 * ============================================================================
 * 5. MEMBER SEGMENT
 * ============================================================================
 *
 * Path segments use the canonical identifier vocabulary.
 *
 * Reserved words are not silently accepted as identifiers.
 */
pathDependentMemberSegment
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar is source syntax only.
 *
 * A production frontend requires a domain-neutral AST representation for the
 * distinction between:
 *
 *     ordinary member access
 *
 * and:
 *
 *     path-dependent type.
 *
 * The recommended canonical representation is:
 *
 *     TypeExpr::PathDependent {
 *         path: ValuePath,
 *     }
 *
 * where `ValuePath` is a source-level, unresolved path representation.
 *
 * The representation MUST preserve:
 *
 *     - root spelling;
 *     - ordered path segments;
 *     - separator kind;
 *     - complete source span;
 *     - `.type` terminator span;
 *     - source ordering.
 *
 * It MUST NOT contain:
 *
 *     resolved symbol IDs;
 *     runtime addresses;
 *     object references;
 *     physical resources;
 *     hardware IDs;
 *     QPU IDs;
 *     backend IDs;
 *     memory addresses;
 *     runtime values.
 *
 * IMPORTANT:
 *
 * The current frontend `TypeExpr` already has:
 *
 *     TypeExpr::Associated
 *
 * but does not currently expose a dedicated path-dependent/singleton variant.
 *
 * Therefore this feature is NOT complete merely by adding this `.g4` file.
 *
 * The AST owner must add the dedicated source-level representation before
 * marking this feature `AST_IMPLEMENTED`.
 *
 * Reusing `TypeExpr::Associated` would lose the semantic distinction between:
 *
 *     T::Item
 *
 * and:
 *
 *     value.type
 *
 * and is therefore NOT recommended.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns all meaning beyond the source shape.
 *
 * For:
 *
 *     value.type
 *
 * semantic analysis must determine:
 *
 *     1. whether `value` resolves;
 *     2. whether every member segment resolves;
 *     3. whether the complete path denotes a value;
 *     4. whether the path is stable enough for a path-dependent type;
 *     5. whether the referenced value has a type identity that can participate
 *        in type checking;
 *     6. whether the path-dependent type is well formed;
 *     7. whether equality with another path-dependent type can be established;
 *     8. whether generic substitutions preserve path identity;
 *     9. whether lifetime/ownership constraints permit the dependency;
 *     10. whether the type can be normalized;
 *     11. whether the type participates in trait/type-class resolution;
 *     12. whether associated-type constraints interact with the result.
 *
 * The grammar MUST NOT perform any of these operations.
 *
 * ============================================================================
 * STABILITY CONTRACT
 * ============================================================================
 *
 * Not every expression path is automatically a valid path-dependent type
 * witness.
 *
 * The semantic system must distinguish stable paths from transient values.
 *
 * For example:
 *
 *     stable_variable.type
 *
 * may be valid.
 *
 * Whereas:
 *
 *     make().type
 *
 * is intentionally not accepted by this grammar because it is not a stable
 * path-shaped source construct.
 *
 * More sophisticated stability rules may consider:
 *
 *     immutable bindings;
 *     ownership;
 *     borrowing;
 *     lifetime;
 *     lexical scope;
 *     region;
 *     object identity;
 *     persistent resources;
 *     actor state;
 *     capability-scoped state.
 *
 * Those are semantic concerns.
 *
 * ============================================================================
 * GENERIC INTEGRATION
 * ============================================================================
 *
 * Path-dependent types may participate in generic declarations:
 *
 *     <T>
 *
 * and generic constraints.
 *
 * Example conceptual semantic usage:
 *
 *     fn use_type<T>(value: T)
 *         where ...
 *
 *     value.type
 *
 * Generic declaration syntax remains owned by:
 *
 *     grammar/functions/generics.g4
 *     grammar/types/generic.g4
 *
 * This grammar must never redefine generic parameters or arguments.
 *
 * ============================================================================
 * ASSOCIATED-TYPE INTEGRATION
 * ============================================================================
 *
 * Path-dependent types and associated types are related but distinct.
 *
 * Example:
 *
 *     value.type
 *
 * versus:
 *
 *     T::Item
 *
 * The first identifies a type dependent on a value path.
 *
 * The second selects an associated/type member from a type-level owner.
 *
 * Semantic analysis may combine both concepts.
 *
 * For example, an associated member may itself participate in a
 * path-dependent type relation.
 *
 * Nevertheless:
 *
 *     associated.g4
 *
 * remains the sole syntax owner of associated-type projection.
 *
 * ============================================================================
 * DEPENDENT-TYPE INTEGRATION
 * ============================================================================
 *
 * Path-dependent types complement the existing dependent-type machinery:
 *
 *     grammar/types/dependent.g4
 *
 * Existing dependent forms include source-level type/value relationships.
 *
 * This file adds the value-path identity dimension without redefining:
 *
 *     Pi
 *     Sigma
 *     dependent value expressions
 *     symbolic dimensions
 *
 * The semantic type system may use path-dependent types as components inside:
 *
 *     Pi types
 *     Sigma types
 *     generic types
 *     arrays
 *     records
 *     function signatures
 *     associated-type constraints
 *     type identities
 *     contracts
 *
 * ============================================================================
 * TYPE-FAMILY INTEGRATION
 * ============================================================================
 *
 * Type-family syntax remains owned by:
 *
 *     grammar/types/type-families.g4
 *
 * A path-dependent type may appear as an input/output of a type-family
 * computation if the semantic type system permits it.
 *
 * This grammar does not define type-family reduction rules.
 *
 * ============================================================================
 * HIGHER-KINDED TYPE INTEGRATION
 * ============================================================================
 *
 * Higher-kinded type syntax remains owned by:
 *
 *     grammar/types/higher-kinded-types.g4
 *
 * Path-dependent types do not need to know the kind of their resolved type.
 *
 * Semantic kind checking happens after path resolution.
 *
 * ============================================================================
 * FUNCTIONAL-DEPENDENCY INTEGRATION
 * ============================================================================
 *
 * Functional dependencies remain owned by:
 *
 *     grammar/types/functional-dependencies.g4
 *
 * or the repository's canonical normalized functional-dependency filename.
 *
 * A path-dependent type may participate in semantic type relationships that
 * are constrained by functional dependencies.
 *
 * This grammar does not duplicate:
 *
 *     A -> B
 *
 * syntax.
 *
 * ============================================================================
 * TYPE-CLASS / TRAIT INTEGRATION
 * ============================================================================
 *
 * A path-dependent type may be subject to:
 *
 *     trait bounds;
 *     type-class constraints;
 *     associated-type constraints;
 *     implementation selection;
 *     coherence;
 *     specialization.
 *
 * These remain semantic responsibilities.
 *
 * The parser does not attempt to resolve a trait implementation.
 *
 * ============================================================================
 * LINEAR / AFFINE INTEGRATION
 * ============================================================================
 *
 * Path-dependent types may occur inside:
 *
 *     Linear<T>
 *     Affine<T>
 *
 * or equivalent canonical source-level type constructs.
 *
 * Ownership and usage analysis remains downstream.
 *
 * This grammar does not encode resource consumption.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Path-dependent types remain target-neutral and may describe type identity
 * associated with quantum-level abstractions.
 *
 * Examples of semantic possibilities include:
 *
 *     register.type
 *     circuit.output.type
 *     measurement.type
 *
 * These names do not allocate:
 *
 *     physical qubits;
 *     logical qubits;
 *     QPUs;
 *     couplers;
 *     calibration data;
 *     measurement hardware.
 *
 * Quantum semantic lowering remains:
 *
 *     source
 *       ->
 *     domain-neutral AST
 *       ->
 *     quantum semantic model
 *       ->
 *     quantum::ir
 *       ->
 *     routing
 *       ->
 *     scheduling
 *       ->
 *     resilience / QEC
 *       ->
 *     ZQN
 *       ->
 *     HAL
 *
 * No quantum operation catalogue belongs here.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Path-dependent types may describe type identity associated with hardware
 * abstractions:
 *
 *     module.output.type
 *     interface.signal.type
 *     device.configuration.type
 *
 * These are source-level abstractions.
 *
 * They do not encode:
 *
 *     register widths;
 *     physical pins;
 *     clock frequency;
 *     FPGA capacity;
 *     ASIC cell counts;
 *     memory-bank counts;
 *     topology size;
 *     device counts.
 *
 * Hardware realization remains downstream.
 *
 * ============================================================================
 * DATA / AI / REASONING INTEGRATION
 * ============================================================================
 *
 * Path-dependent types can express type relationships around:
 *
 *     model.output.type
 *     dataset.schema.type
 *     tensor.element.type
 *     agent.state.type
 *
 * Application semantics remain outside this grammar.
 *
 * The grammar does not introduce application-specific keywords.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Declaring or referencing a path-dependent type has no runtime effect.
 *
 * It does not itself perform:
 *
 *     I/O;
 *     networking;
 *     randomness;
 *     measurement;
 *     learning;
 *     adaptation;
 *     reflection;
 *     code generation;
 *     simulation;
 *     native calls;
 *     foreign calls.
 *
 * If the path ultimately denotes an effectful runtime object, the semantic
 * type system must still distinguish type identity from execution effects.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Path-dependent type syntax does not request capabilities.
 *
 * Capability analysis remains owned by the capability subsystem.
 *
 * A type depending on:
 *
 *     device.configuration
 *
 * does not itself access a device.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Path-dependent type syntax does not allocate resources.
 *
 * It cannot imply:
 *
 *     memory allocation;
 *     qubit allocation;
 *     CPU allocation;
 *     GPU allocation;
 *     FPGA allocation;
 *     accelerator allocation;
 *     network allocation;
 *     device allocation.
 *
 * Resource requirements remain separately expressible through the resource
 * subsystem.
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * Path-dependent types may appear in:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * expressions and declarations.
 *
 * Contract semantics remain owned by the validation/contract subsystem.
 *
 * This file does not define contract syntax.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Policies may constrain declarations or uses involving path-dependent types.
 *
 * This grammar does not define:
 *
 *     allow
 *     forbid
 *     prefer
 *     constrain
 *     fallback
 *
 * Policy ownership remains in the policy subsystem.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * The parser must preserve source spans for:
 *
 *     path root;
 *     every path segment;
 *     separators;
 *     `.type`;
 *     complete path-dependent type.
 *
 * Semantic provenance may record:
 *
 *     source declaration;
 *     resolved path;
 *     type identity;
 *     normalization;
 *     substitution;
 *     specialization;
 *     diagnostic;
 *     derived semantic relationship.
 *
 * The grammar itself performs no provenance recording.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar emits NO direct backend IR.
 *
 * It must not emit:
 *
 *     LLVM IR;
 *     machine code;
 *     QIR;
 *     QASM;
 *     HDL netlists;
 *     FPGA placement;
 *     ASIC layout;
 *     physical qubit mappings;
 *     device schedules;
 *     vendor instructions.
 *
 * Path-dependent types first become part of the canonical semantic type model.
 *
 * Only after semantic resolution may their consequences influence:
 *
 *     classical IR;
 *     quantum::ir;
 *     HDL/hardware representations;
 *     distributed IR;
 *     data/AI semantic representations.
 *
 * ============================================================================
 * BACKEND / HAL CONTRACT
 * ============================================================================
 *
 * This grammar has no direct backend dependency.
 *
 * Backend realization may use a resolved path-dependent type for:
 *
 *     representation selection;
 *     layout;
 *     specialization;
 *     dispatch;
 *     ABI decisions;
 *     optimization.
 *
 * Those decisions must occur downstream.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Path-dependent types describe relationships in program meaning.
 *
 * They must remain independent of physical target capacity.
 *
 * The grammar therefore contains no limits for:
 *
 *     path depth;
 *     generic arity;
 *     type nesting;
 *     number of dependent types;
 *     number of variables;
 *     number of objects;
 *     number of quantum resources;
 *     number of CPUs;
 *     number of GPUs;
 *     number of FPGAs;
 *     number of nodes;
 *     amount of memory;
 *     tensor rank;
 *     network size;
 *     device count.
 *
 * There must be no constants such as:
 *
 *     MAX_PATH_DEPTH
 *     MAX_PATH_DEPENDENCIES
 *     MAX_SINGLETON_TYPES
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
 * "Scale to infinity" means that the language grammar introduces no artificial
 * finite semantic ceiling. Actual compilation remains bounded by available
 * implementation resources and explicit configurable compiler resource
 * policies.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no semantic predicates;
 *     no actions;
 *     no I/O;
 *     no randomness;
 *     no environment inspection;
 *     no hardware discovery;
 *     no network access;
 *     no runtime execution.
 *
 * Identical source token streams under the same language/grammar version must
 * produce equivalent parse structures.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This file contains no Rust code.
 *
 * Generated Zamani parser integration must remain compatible with:
 *
 *     Rust 1.97+
 *     Rust 2021
 *     safe Rust
 *     no unsafe
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Structural syntax errors include:
 *
 *     .type
 *     value.
 *     value..
 *     value.type.extra
 *     value::type
 *     value::field.
 *     value::field..type
 *
 * The following are syntactically valid but may be semantically invalid:
 *
 *     unknown.type
 *     temporary.type
 *     mutable_value.type
 *     inaccessible.type
 *
 * Semantic diagnostics include:
 *
 *     unresolved path;
 *     path denotes a type rather than a value;
 *     unstable path;
 *     invalid lifetime;
 *     invalid ownership relationship;
 *     invalid path-dependent identity;
 *     incompatible path-dependent types;
 *     invalid substitution;
 *     invalid specialization;
 *     invalid trait/type-class constraint.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing constructs remain unaffected:
 *
 *     T::Item
 *     module::Type
 *     value.field
 *     value::member
 *     [T; N]
 *     Pi<...>
 *     Sigma<...>
 *
 * The new production rule is intentionally distinguished by:
 *
 *     .type
 *
 * Therefore it does not require changing the meaning of existing:
 *
 *     :: projections;
 *     qualified names;
 *     ordinary member access.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE STRUCTURAL TESTS
 * -------------------------
 *
 *     value.type
 *
 *     self.type
 *
 *     this.type
 *
 *     object.field.type
 *
 *     object.field.subfield.type
 *
 *     module::value.type
 *
 *     module::object.field.type
 *
 *     module::object.field.subfield.type
 *
 * NEGATIVE STRUCTURAL TESTS
 * -------------------------
 *
 *     .type
 *
 *     value.
 *
 *     value..type
 *
 *     value::type
 *
 *     value::field.type.extra
 *
 *     value::field..type
 *
 *     value.type.type
 *
 *     (value).type
 *
 *     make_value().type
 *
 *     a + b.type
 *
 * The last three are intentionally excluded from this narrow stable-path
 * grammar.
 *
 * SEMANTIC NEGATIVE TESTS
 * -----------------------
 *
 *     unresolved path;
 *     non-value path;
 *     unstable path;
 *     invalid ownership;
 *     invalid lifetime;
 *     inaccessible path;
 *     path whose type identity cannot be established.
 *
 * ============================================================================
 * BOUNDARY TESTS
 * ============================================================================
 *
 * Test:
 *
 *     one path segment;
 *     many path segments;
 *     mixed `::` and `.`;
 *     self;
 *     this;
 *     generic declarations containing path-dependent types;
 *     trait constraints;
 *     associated types;
 *     dependent Pi/Sigma types;
 *     type families;
 *     higher-kinded types;
 *     linear types;
 *     affine types;
 *     quantum types;
 *     HDL types;
 *     hardware-neutral types;
 *     resource-aware types;
 *     contracts;
 *     policies;
 *     provenance;
 *     metaprogramming.
 *
 * ============================================================================
 * SCALABILITY TESTS
 * ============================================================================
 *
 * Generated tests must vary:
 *
 *     path length;
 *     number of path-dependent types;
 *     nesting depth;
 *     generic nesting;
 *     declaration size;
 *     source-unit size.
 *
 * No test may convert an observed implementation/resource limit into a
 * language-level constant.
 *
 * The required language guarantee is:
 *
 *     arbitrary finite source structures are grammatically representable,
 *     subject only to explicit implementation/resource policies.
 *
 * ============================================================================
 * CROSS-DOMAIN TESTS
 * ============================================================================
 *
 * The same grammar must work for semantic declarations involving:
 *
 *     classical computation;
 *     numerical computation;
 *     tensor/data computation;
 *     AI/model types;
 *     probabilistic types;
 *     quantum abstractions;
 *     hybrid computation;
 *     HDL abstractions;
 *     hardware abstractions;
 *     distributed systems;
 *     networking abstractions;
 *     interoperability types;
 *     future computational domains.
 *
 * No domain-specific grammar alternative belongs here.
 *
 * ============================================================================
 * INTEGRATION WITH `grammar/types/types.g4`
 * ============================================================================
 *
 * `types.g4` is the canonical type orchestrator.
 *
 * It must import:
 *
 *     PathDependentTypes
 *
 * after the grammar file is established as a parser component.
 *
 * The canonical `typeCore` must expose:
 *
 *     pathDependentType
 *
 * as one of its specialized alternatives.
 *
 * Conceptually:
 *
 *     typeCore
 *         : ...
 *         | dependentType
 *         | pathDependentType
 *         | associatedType
 *         | ...
 *         ;
 *
 * This file does NOT own `typeExpression`.
 *
 * ============================================================================
 * INTEGRATION WITH `grammar/types/associated.g4`
 * ============================================================================
 *
 * No grammar change is required merely because path-dependent types exist.
 *
 * `associated.g4` remains the owner of:
 *
 *     Base::Member
 *
 * Path-dependent types remain:
 *
 *     path.type
 *
 * The semantic type subsystem may integrate both representations.
 *
 * ============================================================================
 * INTEGRATION WITH `grammar/expressions/member-access.g4`
 * ============================================================================
 *
 * No replacement of member-access syntax is required.
 *
 * Ordinary:
 *
 *     value.field
 *
 * remains expression/member syntax.
 *
 * A path-dependent type is distinguished only when the complete syntactic
 * sequence terminates in:
 *
 *     .type
 *
 * The semantic/type parser context determines that:
 *
 *     value.field.type
 *
 * is a type expression rather than an ordinary runtime member chain.
 *
 * ============================================================================
 * INTEGRATION WITH `grammar/types/dependent.g4`
 * ============================================================================
 *
 * No duplicate dependent-type grammar is introduced.
 *
 * `dependent.g4` remains the owner of its existing dependent/value-parameter
 * forms.
 *
 * This file contributes only path-dependent type identity.
 *
 * ============================================================================
 * INTEGRATION WITH `grammar/types/type-families.g4`
 * ============================================================================
 *
 * No type-family syntax is duplicated.
 *
 * A path-dependent type may be consumed semantically as a family input/output
 * where permitted by the canonical type system.
 *
 * ============================================================================
 * INTEGRATION WITH GENERICS
 * ============================================================================
 *
 * `grammar/types/generic.g4` remains the sole owner of generic application.
 *
 * `grammar/functions/generics.g4` remains the owner of function-generic
 * declaration syntax where applicable.
 *
 * This file never declares:
 *
 *     <T>
 *     <T, U>
 *     <T: Bound>
 *
 * ============================================================================
 * INTEGRATION WITH TYPE CLASSES / TRAITS
 * ============================================================================
 *
 * Trait declarations remain owned by:
 *
 *     grammar/declarations/traits.g4
 *
 * Type-class references remain owned by:
 *
 *     grammar/types/type-class.g4
 *
 * Path-dependent types may appear in their constraints and members, but this
 * file does not redefine those constructs.
 *
 * ============================================================================
 * INTEGRATION WITH FUNCTIONAL DEPENDENCIES
 * ============================================================================
 *
 * Functional dependencies remain an independent constraint mechanism.
 *
 * They may semantically relate generic parameters whose resolved types
 * include path-dependent types.
 *
 * This grammar does not import or redefine functional-dependency syntax.
 *
 * ============================================================================
 * INTEGRATION WITH CONTRACTS / POLICIES / PROVENANCE
 * ============================================================================
 *
 * Contract syntax remains under validation.
 *
 * Policy syntax remains under policy ownership.
 *
 * Provenance remains under the provenance subsystem.
 *
 * Path-dependent types simply remain valid type expressions wherever the
 * canonical type-expression entry point is accepted.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * PASS CONDITIONS:
 *
 *     [ ] No machine-capacity constants.
 *     [ ] No quantum-capacity constants.
 *     [ ] No path-depth constants.
 *     [ ] No generic-arity constants.
 *     [ ] No target enumeration.
 *     [ ] No vendor enumeration.
 *     [ ] No application-specific keyword inventory.
 *     [ ] No runtime resource allocation.
 *     [ ] No hardware discovery.
 *     [ ] No semantic resolution in parser actions.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [ ] The file is named `path-dependent-types.g4`.
 *
 *     [ ] Grammar name is `PathDependentTypes`.
 *
 *     [ ] It is a parser grammar, not a combined grammar.
 *
 *     [ ] It consumes only the canonical `ZamaniLexer`.
 *
 *     [ ] It introduces no lexer tokens.
 *
 *     [ ] It defines `pathDependentType`.
 *
 *     [ ] It defines only the restricted stable-path syntax required by the
 *         feature.
 *
 *     [ ] It uses the existing `TYPE` token.
 *
 *     [ ] It uses the existing `DOT` token.
 *
 *     [ ] It uses the existing `DOUBLE_COLON` token.
 *
 *     [ ] It supports `self.type` and `this.type`.
 *
 *     [ ] It supports arbitrarily long finite paths.
 *
 *     [ ] It does not redefine `typeExpression`.
 *
 *     [ ] It does not redefine `typePath`.
 *
 *     [ ] It does not redefine associated-type syntax.
 *
 *     [ ] It does not redefine member-access syntax.
 *
 *     [ ] It does not import the full expression grammar.
 *
 *     [ ] It introduces no runtime effects.
 *
 *     [ ] It introduces no capabilities.
 *
 *     [ ] It allocates no resources.
 *
 *     [ ] It selects no hardware.
 *
 *     [ ] It emits no backend IR.
 *
 *     [ ] It preserves source spans.
 *
 *     [ ] It has positive parser tests.
 *
 *     [ ] It has negative parser tests.
 *
 *     [ ] It has semantic tests.
 *
 *     [ ] It has cross-domain tests.
 *
 *     [ ] It has scalability tests.
 *
 *     [ ] It has determinism tests.
 *
 *     [ ] AST integration is explicitly completed.
 *
 *     [ ] `types.g4` integration is completed.
 *
 *     [ ] Rust 1.97+ generated-parser integration is completed.
 *
 *     [ ] No unsafe Rust is required.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL INVARIANT
 * ============================================================================
 *
 * Path-dependent types are a source-level type-system feature.
 *
 * They are NOT:
 *
 *     runtime reflection;
 *     runtime object inspection;
 *     resource discovery;
 *     hardware selection;
 *     quantum allocation;
 *     an associated-type replacement;
 *     a second expression grammar;
 *     a second type grammar;
 *     a backend IR.
 *
 * The canonical architecture remains:
 *
 *     path.type
 *          |
 *          v
 *     domain-neutral TypeExpr
 *          |
 *          v
 *     path/name resolution
 *          |
 *          v
 *     semantic path-dependent type
 *          |
 *          v
 *     canonical semantic type model
 *          |
 *       +--+----------------+
 *       |                   |
 *       v                   v
 *   classical          quantum::ir
 *       |                   |
 *       +---------+---------+
 *                 |
 *                 v
 *       target-independent
 *          optimization
 *                 |
 *                 v
 *       lowering / realization
 *
 * ============================================================================
 */

parser grammar PathDependentTypes;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Canonical examples:
 *
 *     value.type
 *     self.type
 *     this.type
 *     object.field.type
 *     module::value.type
 *     module::object.field.type
 */
pathDependentType
    : pathDependentValuePath
      DOT
      TYPE
    ;


/*
 * ============================================================================
 * VALUE PATH
 * ============================================================================
 *
 * The path is intentionally narrower than a general expression.
 */
pathDependentValuePath
    : pathDependentPathRoot
      pathDependentPathSegment*
    ;


/*
 * ============================================================================
 * PATH ROOT
 * ============================================================================
 */
pathDependentPathRoot
    : IDENTIFIER
    | SELF
    | THIS
    ;


/*
 * ============================================================================
 * PATH SEGMENT
 * ============================================================================
 *
 * The final `.type` terminator is deliberately excluded from this rule.
 */
pathDependentPathSegment
    : DOUBLE_COLON
      pathDependentMemberSegment
    | DOT
      pathDependentMemberSegment
    ;


/*
 * ============================================================================
 * PATH MEMBER
 * ============================================================================
 */
pathDependentMemberSegment
    : IDENTIFIER
    ;
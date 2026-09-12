/*
 * Zamani Programming Language
 * Grammar: Resource Types
 *
 * File:
 *   grammar/types/resource-types.g4
 *
 * Status:
 *   Production grammar component.
 *
 * Purpose:
 *   Defines source-level syntax for types that describe computational
 *   resources, resource-bearing values, resource classes, and
 *   resource-parameterized types.
 *
 * Architectural position:
 *
 *   Source
 *      |
 *      v
 *   ZamaniLexer
 *      |
 *      v
 *   Zamani parser grammars
 *      |
 *      v
 *   Frontend AST
 *      |
 *      v
 *   Semantic analysis
 *      |
 *      +--> capability/resource validation
 *      |
 *      +--> canonical IR
 *      |
 *      +--> optimization
 *      |
 *      +--> routing / scheduling
 *      |
 *      +--> hardware / target lowering
 *      |
 *      v
 *   Runtime / deployment
 *
 * IMPORTANT:
 *
 * This grammar describes SOURCE SYNTAX.
 *
 * It does not define:
 *   - a resource allocator
 *   - a resource manager
 *   - hardware discovery
 *   - hardware topology
 *   - scheduling
 *   - placement
 *   - device selection
 *   - runtime resource state
 *   - resource accounting
 *   - capability inference
 *   - optimization
 *   - quantum IR
 *   - classical IR
 *   - hardware IR
 *   - ABI layout
 *   - physical addresses
 *   - machine sizes
 *   - fixed resource limits
 *
 * POCO-REAF:
 *
 *   Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Resource types describe semantic requirements or classifications.
 * They must not accidentally bind a program to a particular machine.
 *
 * Examples of concepts that belong elsewhere:
 *
 *   "requires quantum"
 *       -> source-level resource/capability requirement
 *
 *   "requires 128 qubits"
 *       -> semantic/resource requirement, represented by the appropriate
 *          resource expression/type parameter rather than a grammar limit
 *
 *   "run on device qpu-7"
 *       -> target/deployment policy, NOT a resource type
 *
 *   "use exactly 64 hardware threads"
 *       -> target/resource constraint, NOT a parser-level maximum
 *
 *   "GPU memory = 80 GiB"
 *       -> hardware/runtime capability, NOT grammar semantics
 *
 * No finite resource limit is encoded by this file.
 *
 * Rust implementation contract:
 *
 *   - Rust 2021
 *   - Rust 1.97 / 1.97.1
 *   - no unsafe Rust
 *
 * This .g4 file itself contains no executable Rust code.
 *
 * Integration contract:
 *
 *   - Uses ZamaniLexer as the canonical lexer vocabulary.
 *   - References the canonical typeExpression rule supplied by the
 *     containing/composed type grammar.
 *   - References canonical identifier/path syntax rather than defining
 *     another identifier grammar.
 *   - Must not introduce a second QubitId, ResourceId, DeviceId, etc.
 *
 * Grammar ownership:
 *
 *   resource-type syntax only.
 *
 * Semantic ownership belongs to the compiler/type/resource systems.
 *
 * --------------------------------------------------------------------------
 * DESIGN PRINCIPLES
 * --------------------------------------------------------------------------
 *
 * 1. A resource type is a TYPE-LEVEL DESCRIPTION.
 *
 * 2. Resource types are not runtime resources.
 *
 * 3. Resource types do not allocate resources.
 *
 * 4. Resource types do not identify physical devices.
 *
 * 5. Resource types do not imply a scheduling strategy.
 *
 * 6. Resource types do not imply a hardware topology.
 *
 * 7. Resource types may be parameterized.
 *
 * 8. Resource parameters may be symbolic.
 *
 * 9. Concrete resource feasibility is checked after parsing.
 *
 * 10. Target-specific realization is downstream.
 *
 * 11. No parser rule contains an artificial finite maximum.
 *
 * 12. The grammar must remain extensible for future computational models.
 *
 * --------------------------------------------------------------------------
 * ANTLR COMPOSITION
 * --------------------------------------------------------------------------
 *
 * This is intentionally a parser grammar.
 *
 * Lexer tokens such as IDENT, LT, GT, COMMA, COLON, etc. are supplied by
 * ZamaniLexer.
 *
 * This file does NOT redefine those tokens.
 *
 * The canonical typeExpression and identifier rules are owned by the
 * appropriate higher-level type/core grammar and must not be duplicated
 * here.
 */

parser grammar ResourceTypes;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * --------------------------------------------------------------------------
 * RESOURCE TYPE ROOT
 * --------------------------------------------------------------------------
 *
 * resourceType
 *
 * Represents a source-level type whose semantic interpretation is associated
 * with computational resources.
 *
 * The concrete semantic meaning is resolved later.
 *
 * Examples:
 *
 *   resource
 *   resource<quantum>
 *   resource<cpu>
 *   resource<gpu>
 *   resource<accelerator>
 *   resource<network>
 *   resource<memory>
 *
 * The identifiers above are examples of possible vocabulary, not a closed
 * enumeration. New resource domains must not require changing this grammar
 * merely because a new machine technology appears.
 */
resourceType
    : RESOURCE
      resourceTypeArguments?
    ;

/*
 * --------------------------------------------------------------------------
 * PARAMETERIZED RESOURCE TYPE
 * --------------------------------------------------------------------------
 *
 * Resource parameters are intentionally syntactic expressions rather than
 * parser-enforced constants.
 *
 * This permits:
 *
 *   resource<quantum, N>
 *   resource<gpu, capacity>
 *   resource<memory, amount>
 *   resource<accelerator, width>
 *
 * without imposing a finite range at grammar level.
 *
 * Semantic validation determines whether a parameter is meaningful for the
 * selected resource kind.
 */
resourceTypeArguments
    : LT
      resourceTypeArgumentList?
      GT
    ;

resourceTypeArgumentList
    : resourceTypeArgument
      (COMMA resourceTypeArgument)*
      COMMA?
    ;

resourceTypeArgument
    : typeExpression
    | resourceTypeValueArgument
    | resourceTypeNamedArgument
    ;

/*
 * --------------------------------------------------------------------------
 * RESOURCE TYPE VALUE ARGUMENT
 * --------------------------------------------------------------------------
 *
 * A resource type may carry a symbolic or literal value.
 *
 * The grammar does not impose a maximum or minimum.
 *
 * Examples:
 *
 *   resource<quantum, 128>
 *   resource<quantum, qubit_count>
 *   resource<memory, required_memory>
 *
 * The semantic layer decides whether the value represents a count,
 * quantity, capacity, width, duration, or another domain-specific concept.
 */
resourceTypeValueArgument
    : expression
    ;

/*
 * --------------------------------------------------------------------------
 * NAMED RESOURCE TYPE ARGUMENT
 * --------------------------------------------------------------------------
 *
 * Named arguments preserve source intent when a resource has multiple
 * independently meaningful parameters.
 *
 * Examples:
 *
 *   resource<quantum, qubits: N>
 *   resource<memory, capacity: amount>
 *   resource<accelerator, kind: accelerator_kind>
 *
 * The actual semantic vocabulary is NOT fixed here.
 */
resourceTypeNamedArgument
    : identifier
      COLON
      expression
    ;

/*
 * --------------------------------------------------------------------------
 * RESOURCE TYPE QUALIFICATION
 * --------------------------------------------------------------------------
 *
 * A resource type can be qualified by semantic properties such as:
 *
 *   shared
 *   exclusive
 *   local
 *   remote
 *   logical
 *   physical
 *
 * These are syntactic qualifiers only.
 *
 * Their legality and meaning are determined by semantic analysis.
 *
 * This intentionally does not encode deployment topology.
 */
qualifiedResourceType
    : resourceType
      resourceTypeQualifierList?
    ;

resourceTypeQualifierList
    : resourceTypeQualifier+
    ;

resourceTypeQualifier
    : resourceSharingQualifier
    | resourceLocationQualifier
    | resourceAbstractionQualifier
    ;

/*
 * --------------------------------------------------------------------------
 * RESOURCE SHARING
 * --------------------------------------------------------------------------
 *
 * These describe source-level resource ownership/usage intent.
 *
 * They do not perform allocation.
 */
resourceSharingQualifier
    : SHARED
    | EXCLUSIVE
    ;

/*
 * --------------------------------------------------------------------------
 * RESOURCE LOCATION
 * --------------------------------------------------------------------------
 *
 * Location qualifiers are intentionally abstract.
 *
 * "local" does not mean a particular NUMA node, CPU socket, process, machine,
 * rack, cloud region, or physical address.
 *
 * "remote" does not identify a particular network endpoint.
 */
resourceLocationQualifier
    : LOCAL
    | REMOTE
    ;

/*
 * --------------------------------------------------------------------------
 * RESOURCE ABSTRACTION
 * --------------------------------------------------------------------------
 *
 * Allows source programs to distinguish semantic resource abstraction levels
 * where that distinction is meaningful.
 *
 * Examples:
 *
 *   logical
 *   physical
 *
 * In quantum computing, for example, logical versus physical qubits are
 * semantically important, but mapping a logical qubit to a physical qubit is
 * owned by routing/hardware compilation, not this grammar.
 */
resourceAbstractionQualifier
    : LOGICAL
    | PHYSICAL
    ;

/*
 * --------------------------------------------------------------------------
 * RESOURCE TYPE REFERENCE
 * --------------------------------------------------------------------------
 *
 * A named resource type may be supplied by a library, module, dialect, or
 * domain-specific extension.
 *
 * Canonical path/name grammar remains owned by the core language grammar.
 *
 * This rule intentionally references identifier/path syntax instead of
 * defining another identifier representation.
 */
resourceTypeReference
    : qualifiedName
    ;

/*
 * --------------------------------------------------------------------------
 * RESOURCE TYPE EXPRESSION
 * --------------------------------------------------------------------------
 *
 * This rule is the integration point for resource-bearing type expressions.
 *
 * A resource can be:
 *
 *   - a standalone resource type
 *   - a named resource type
 *   - a parameterized resource
 *   - a qualified resource
 *
 * Generic/composite/reference/pointer/tuple/etc. composition remains owned
 * by the general type system.
 */
resourceTypeExpression
    : resourceType
    | qualifiedResourceType
    | resourceTypeReference
    ;

/*
 * --------------------------------------------------------------------------
 * RESOURCE-BEARING TYPE
 * --------------------------------------------------------------------------
 *
 * Allows a normal type to be associated with a resource type.
 *
 * This is intentionally structural syntax.
 *
 * The semantic layer determines whether the association is legal.
 *
 * Examples of possible source semantics:
 *
 *   value: T @ resource<R>
 *   buffer: Tensor<T> @ resource<memory>
 *
 * The exact surface notation is controlled by the higher-level type grammar.
 */
resourceBoundType
    : typeExpression
      AT
      resourceTypeExpression
    ;

/*
 * --------------------------------------------------------------------------
 * RESOURCE KIND
 * --------------------------------------------------------------------------
 *
 * Resource kinds are open-ended semantic names.
 *
 * The grammar deliberately does NOT enumerate every possible resource kind.
 *
 * This is necessary for future-proofing.
 *
 * A future accelerator, quantum technology, memory technology, interconnect,
 * processing architecture, or computational substrate must not require the
 * language grammar to be rewritten simply because its resource category is
 * new.
 */
resourceKind
    : identifier
    ;

/*
 * --------------------------------------------------------------------------
 * RESOURCE TYPE DECLARATION REFERENCE
 * --------------------------------------------------------------------------
 *
 * Resource type declarations themselves belong to declaration/type-definition
 * grammar.
 *
 * This rule only provides the reusable resource-type shape needed by those
 * declarations.
 */
resourceTypeParameter
    : identifier
    ;

/*
 * --------------------------------------------------------------------------
 * RESOURCE TYPE CONSTRAINT REFERENCE
 * --------------------------------------------------------------------------
 *
 * Constraints are semantic expressions evaluated downstream.
 *
 * The grammar permits an expression but does not determine:
 *
 *   - feasibility
 *   - resource availability
 *   - scheduling
 *   - hardware selection
 *   - placement
 *   - performance
 */
resourceTypeConstraint
    : identifier
      COLON
      expression
    ;

/*
 * --------------------------------------------------------------------------
 * RESOURCE TYPE LIST
 * --------------------------------------------------------------------------
 *
 * No fixed number of resources is permitted.
 */
resourceTypeList
    : resourceTypeExpression
      (COMMA resourceTypeExpression)*
      COMMA?
    ;

/*
 * --------------------------------------------------------------------------
 * RESOURCE TYPE SET
 * --------------------------------------------------------------------------
 *
 * Represents syntactic grouping of resource types.
 *
 * Set semantics, uniqueness, compatibility, and satisfiability are semantic
 * responsibilities.
 */
resourceTypeSet
    : LBRACE
      resourceTypeList?
      RBRACE
    ;

/*
 * --------------------------------------------------------------------------
 * RESOURCE TYPE REQUIREMENT
 * --------------------------------------------------------------------------
 *
 * This is a type-level syntactic representation of a required resource.
 *
 * It does NOT mean:
 *
 *   allocate immediately
 *   select a device
 *   reserve hardware
 *   schedule execution
 *   reject a target
 *
 * Those decisions belong downstream.
 */
resourceTypeRequirement
    : REQUIRES
      resourceTypeExpression
    ;

/*
 * --------------------------------------------------------------------------
 * RESOURCE TYPE PREFERENCE
 * --------------------------------------------------------------------------
 *
 * Preferences are weaker than requirements.
 *
 * The distinction is semantic and important for POCO-REAF:
 *
 * requirement != preference != hint != constraint
 */
resourceTypePreference
    : PREFER
      resourceTypeExpression
    ;

/*
 * --------------------------------------------------------------------------
 * RESOURCE TYPE HINT
 * --------------------------------------------------------------------------
 *
 * A hint is non-binding source information.
 *
 * A compiler or runtime may ignore a hint when necessary.
 */
resourceTypeHint
    : HINT
      resourceTypeExpression
    ;

/*
 * --------------------------------------------------------------------------
 * RESOURCE TYPE CAPABILITY
 * --------------------------------------------------------------------------
 *
 * A capability describes what a resource may support.
 *
 * It is not equivalent to a concrete device.
 */
resourceTypeCapability
    : CAPABILITY
      resourceTypeExpression
    ;

/*
 * --------------------------------------------------------------------------
 * RESOURCE TYPE REQUIREMENT SET
 * --------------------------------------------------------------------------
 *
 * Requirements can be grouped without imposing a machine-specific limit.
 */
resourceTypeRequirementSet
    : REQUIREMENTS
      resourceTypeSet
    ;

/*
 * --------------------------------------------------------------------------
 * RESOURCE TYPE CAPABILITY SET
 * --------------------------------------------------------------------------
 *
 * Capability sets are descriptive.
 */
resourceTypeCapabilitySet
    : CAPABILITIES
      resourceTypeSet
    ;
/*
 * ============================================================================
 * ZAMANI PROGRAMMING LANGUAGE
 * ============================================================================
 *
 * File:
 *     grammar/effects/reflection.g4
 *
 * Grammar:
 *     ReflectionEffects
 *
 * Status:
 *     CANONICAL REFLECTION-EFFECT DOMAIN INTEGRATION GRAMMAR
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     - No embedded Rust actions.
 *     - No semantic predicates.
 *     - No filesystem access.
 *     - No network access.
 *     - No runtime execution.
 *     - No hardware discovery.
 *     - No resource discovery.
 *     - No target selection.
 *     - No capability discovery.
 *     - No randomness.
 *     - No unsafe implementation.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the canonical parser-level integration boundary for the
 * REFLECTION EFFECT DOMAIN.
 *
 * Reflection is a source-language/metaprogramming facility whose semantic
 * execution may have an effect.
 *
 * This file connects that effect to Zamani's generic effect subsystem.
 *
 * It DOES NOT define the source-level reflection expression.
 *
 * Source-level reflection syntax is owned by:
 *
 *     grammar/metaprogramming/reflection.g4
 *
 * In particular, that grammar owns:
 *
 *     reflectionExpressionCore
 *     reflectionSubject
 *     reflectionValueSubject
 *     reflectionTypeSubject
 *     reflectionProjection
 *     reflectionSelector
 *
 * This file instead provides reusable effect-domain adapters for semantic
 * effect analysis.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * The intended architecture is:
 *
 *     source
 *       |
 *       v
 *     canonical lexer
 *       |
 *       v
 *     canonical parser
 *       |
 *       +------------------------------+
 *       |                              |
 *       v                              v
 *     reflection syntax          generic effects
 *       |                              |
 *       v                              v
 *     domain-neutral AST         effect model
 *       |                              |
 *       +---------------+--------------+
 *                       |
 *                       v
 *                semantic analysis
 *                       |
 *             +---------+----------+
 *             |         |          |
 *             v         v          v
 *           types   capabilities resources
 *             |         |          |
 *             +---------+----------+
 *                       |
 *                       v
 *                    policies
 *                       |
 *                       v
 *                   provenance
 *                       |
 *                       v
 *              canonical semantic model
 *                       |
 *             +---------+----------+
 *             |                    |
 *             v                    v
 *        classical              quantum::ir
 *             |                    |
 *             +---------+----------+
 *                       |
 *                       v
 *                 target-independent
 *                   optimization
 *                       |
 *                       v
 *              lowering / scheduling
 *                       |
 *                       v
 *                  target realization
 *
 * Reflection is therefore an effect-bearing frontend capability, not a
 * backend-specific feature.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     reflectionEffect
 *     reflectionEffectReference
 *     reflectionEffectReferenceList
 *     reflectionEffectSet
 *     reflectionEffectOperationReference
 *     reflectionEffectInvocation
 *     reflectionEffectOperationUse
 *
 * These are effect-domain adapters around the canonical generic effect
 * subsystem.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     reflectionExpressionCore
 *     reflectionExpression
 *     reflectionSubject
 *     reflectionValueSubject
 *     reflectionTypeSubject
 *     reflectionProjection
 *     reflectionSelector
 *     identifier
 *     qualifiedName
 *     typeExpression
 *     expression
 *     effectDeclaration
 *     effectOperationDeclaration
 *     effectReference
 *     effectReferenceList
 *     effectSet
 *     effectOperationReference
 *     effectInvocation
 *     effectOperationUse
 *     effectHandler
 *     effectPolymorphism
 *     effect composition
 *     metaprogramming
 *     macro expansion
 *     source generation
 *     specialization
 *     compile-time evaluation
 *     capability discovery
 *     resource discovery
 *     policy authorization
 *     provenance recording
 *     runtime reflection
 *     hardware discovery
 *     quantum routing
 *     quantum scheduling
 *     quantum::ir
 *     HDL IR
 *     classical IR
 *     runtime execution
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There MUST be exactly one owner for reflection SOURCE SYNTAX:
 *
 *     grammar/metaprogramming/reflection.g4
 *
 * There MUST be exactly one owner for generic EFFECT SYNTAX:
 *
 *     grammar/effects/effect-declarations.g4
 *     grammar/effects/effect-sets.g4
 *     grammar/effects/effect-operations.g4
 *     grammar/effects/effect-handling.g4
 *     grammar/effects/effect-types.g4
 *     grammar/effects/effect-polymorphism.g4
 *     grammar/effects/effect-composition.g4
 *     grammar/effects/custom-effects.g4
 *
 * This file MUST NOT create another reflection expression grammar.
 *
 * In particular, it MUST NOT define:
 *
 *     reflect
 *     reflectionExpression
 *     reflectionSubject
 *     reflectionProjection
 *
 * and it MUST NOT import the complete reflection expression grammar merely
 * to classify an effect.
 *
 * ============================================================================
 * CRITICAL ARCHITECTURAL DISTINCTION
 * ============================================================================
 *
 * SOURCE SYNTAX:
 *
 *     reflect(MyType).members
 *
 * belongs to:
 *
 *     grammar/metaprogramming/reflection.g4
 *
 * EFFECT CLASSIFICATION:
 *
 *     reflection
 *
 * belongs to the semantic effect system and may use the adapters in this file.
 *
 * Therefore:
 *
 *     reflection syntax
 *          |
 *          v
 *     canonical AST
 *          |
 *          v
 *     semantic classification
 *          |
 *          v
 *     reflection effect
 *
 * The parser must not infer the effect merely from spelling.
 *
 * A valid reflection expression is not automatically authorized.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     Core
 *     Expressions
 *     EffectOperations
 *     EffectSets
 *
 * Core supplies canonical language-wide name infrastructure.
 *
 * Expressions supplies canonical expression infrastructure where required by
 * generic effect invocation.
 *
 * EffectOperations supplies:
 *
 *     effectOperationReference
 *     effectInvocation
 *     effectOperationUse
 *
 * EffectSets supplies:
 *
 *     effectReference
 *     effectReferenceList
 *     effectSet
 *
 * This grammar MUST NOT import:
 *
 *     Metaprogramming
 *     Reflection
 *     Statements
 *     AI
 *     Quantum
 *     HDL
 *     Hardware
 *     Runtime
 *
 * merely to obtain syntax.
 *
 * This keeps the grammar dependency graph acyclic.
 *
 * ============================================================================
 * EXPORTS
 * ============================================================================
 *
 * PUBLIC:
 *
 *     reflectionEffect
 *
 * REUSABLE:
 *
 *     reflectionEffectReference
 *     reflectionEffectReferenceList
 *     reflectionEffectSet
 *     reflectionEffectOperationReference
 *     reflectionEffectInvocation
 *     reflectionEffectOperationUse
 *
 * ============================================================================
 * CONSUMED BY
 * ============================================================================
 *
 * Semantic effect classification may consume these parser contexts through
 * the normal frontend AST mapping.
 *
 * Potential semantic consumers:
 *
 *     reflection semantic analysis
 *     effect inference
 *     effect checking
 *     capability checking
 *     policy checking
 *     provenance analysis
 *     compile-time evaluation
 *     metaprogramming validation
 *     execution planning
 *
 * The generic effect composition root:
 *
 *     grammar/effects/effects.g4
 *
 * MUST remain domain-neutral and MUST NOT import this file merely to enumerate
 * reflection effects.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar produces parser contexts only.
 *
 * It does not construct AST objects.
 *
 * The frontend AST must preserve, where applicable:
 *
 *     source span
 *     source order
 *     effect identity
 *     qualified-name segments
 *     operation identity
 *     invocation arguments
 *     effect-set membership
 *     surrounding syntactic context
 *
 * Reflection source syntax is mapped separately through:
 *
 *     grammar/metaprogramming/reflection.g4
 *
 * The effect AST representation, if required by the frontend, must remain
 * domain-neutral.
 *
 * This grammar MUST NOT introduce:
 *
 *     ReflectionObject
 *     ReflectionRuntimeObject
 *     HardwareReflectionObject
 *     QuantumReflectionObject
 *     ReflectionIR
 *     ReflectionRuntimeHandle
 *     DeviceReflectionHandle
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing proves only structural validity.
 *
 * Semantic analysis determines:
 *
 *     - whether a reflection operation resolves;
 *     - whether the referenced operation is declared;
 *     - whether reflection is permitted in the current phase;
 *     - whether the subject is reflectable;
 *     - whether the requested metadata is visible;
 *     - whether the operation is compile-time or runtime;
 *     - whether capabilities are required;
 *     - whether resources are required;
 *     - whether policies authorize access;
 *     - whether provenance must be recorded;
 *     - whether the result is deterministic;
 *     - whether the result is portable;
 *     - whether the operation exposes implementation-specific information;
 *     - whether the operation crosses a domain boundary.
 *
 * Parser acceptance MUST NOT imply semantic authorization.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Reflection is an effect-domain concept because reflection can observe or
 * expose semantic state.
 *
 * Depending on the semantic subject, reflection may be:
 *
 *     pure/static
 *     compile-time
 *     metadata-observing
 *     capability-sensitive
 *     resource-sensitive
 *     runtime-observing
 *     externally stateful
 *
 * The grammar does not decide which category applies.
 *
 * Semantic analysis determines the actual effect set.
 *
 * A reflection expression MUST NOT silently acquire external-state access
 * merely because it is syntactically valid.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Capability requirements are semantic.
 *
 * Examples of possible semantic capabilities include:
 *
 *     reflection.read
 *     reflection.metadata
 *     reflection.runtime
 *     reflection.external_state
 *
 * These names are examples of semantic capability identifiers only.
 *
 * This grammar does NOT define a closed capability enumeration.
 *
 * A future capability may be introduced through the capability subsystem
 * without modifying this file.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Reflection may consume resources for:
 *
 *     metadata traversal
 *     semantic lookup
 *     compile-time evaluation
 *     runtime inspection
 *     provenance collection
 *     generated metadata
 *
 * Resource requirements are resolved outside this grammar.
 *
 * This grammar MUST NOT define:
 *
 *     maximum reflected entities
 *     maximum projections
 *     maximum metadata
 *     maximum type complexity
 *     maximum declarations
 *     maximum members
 *     maximum reflection depth
 *
 * Compiler memory, time, recursion, cancellation, or execution budgets are
 * implementation policies and are not language semantics.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Reflection may be restricted by policy.
 *
 * Policies may govern:
 *
 *     visibility
 *     phase
 *     external-state access
 *     implementation-detail exposure
 *     runtime inspection
 *     source inspection
 *     generated metadata
 *     sensitive metadata
 *     foreign/native boundaries
 *
 * This grammar does not implement policy evaluation.
 *
 * Parser acceptance is not authorization.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Reflection results may need provenance.
 *
 * Semantic provenance may record:
 *
 *     subject
 *     selector
 *     source
 *     phase
 *     derivation
 *     transformation
 *     evidence
 *     policy decision
 *     capability decision
 *     target context
 *
 * This grammar preserves source structure but does not create provenance
 * records.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing is deterministic.
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware
 *     CPU
 *     GPU
 *     FPGA
 *     QPU
 *     filesystem
 *     network
 *     wall-clock time
 *     environment variables
 *     runtime state
 *     deployment topology
 *     resource availability
 *     randomness
 *
 * Reflection RESULT determinism is a semantic concern.
 *
 * Static source metadata should be reproducible.
 *
 * External-state reflection must be explicitly represented by semantic
 * effects/capabilities/policies and must never be inferred solely from the
 * existence of this grammar.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Reflection must preserve portable program meaning.
 *
 * The grammar therefore imposes no limit on:
 *
 *     reflected declarations
 *     reflected types
 *     reflected operations
 *     reflected domains
 *     projection chains
 *     namespaces
 *     qualified-name depth
 *     metadata structures
 *     program size
 *
 * It MUST NOT encode:
 *
 *     CPU counts
 *     GPU counts
 *     FPGA counts
 *     accelerator counts
 *     QPU counts
 *     qubit capacities
 *     memory capacities
 *     register widths
 *     tensor dimensions
 *     node counts
 *     device counts
 *     physical addresses
 *     vendor-specific hardware
 *     topology sizes
 *
 * Reflection must not turn target discovery into permanent source semantics.
 *
 * A program may be reflected over a semantic capability model, but the
 * realization of that capability belongs downstream.
 *
 * ============================================================================
 * OPEN-WORLD CONTRACT
 * ============================================================================
 *
 * Reflection effect identities are open-world.
 *
 * The grammar MUST NOT enumerate:
 *
 *     reflection.read
 *     reflection.type
 *     reflection.member
 *     reflection.attribute
 *     reflection.effect
 *     reflection.capability
 *     reflection.resource
 *     reflection.hardware
 *     reflection.quantum
 *
 * as a finite set of built-ins.
 *
 * Qualified names remain semantic identifiers.
 *
 * This allows future reflection domains to be added without changing the
 * universal grammar.
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * Reflection may inspect semantic descriptions of:
 *
 *     quantum declarations
 *     quantum types
 *     quantum operations
 *     logical qubits
 *     circuits
 *     measurements
 *     observables
 *     quantum capabilities
 *     quantum requirements
 *
 * However, this grammar MUST NOT:
 *
 *     - enumerate quantum gates;
 *     - assign physical qubits;
 *     - inspect physical calibration implicitly;
 *     - choose a QPU;
 *     - perform routing;
 *     - perform scheduling;
 *     - perform QEC;
 *     - create quantum IR.
 *
 * If reflected information participates in quantum compilation, the canonical
 * quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * The pipeline remains:
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
 *     routing
 *       |
 *       v
 *     scheduling
 *       |
 *       v
 *     resilience / QEC / ZQN
 *       |
 *       v
 *     HAL
 *
 * This file introduces no alternate quantum representation.
 *
 * ============================================================================
 * HDL / HARDWARE CONTRACT
 * ============================================================================
 *
 * Reflection may inspect semantic descriptions of:
 *
 *     HDL modules
 *     signals
 *     ports
 *     interfaces
 *     hardware capabilities
 *     resource requirements
 *     accelerator intent
 *
 * It must not implicitly inspect:
 *
 *     physical addresses
 *     physical device state
 *     implementation-specific register widths
 *     physical topology
 *     vendor-specific implementation state
 *
 * unless a separately authorized semantic capability permits it.
 *
 * ============================================================================
 * DISTRIBUTED CONTRACT
 * ============================================================================
 *
 * Reflection may describe distributed semantic entities:
 *
 *     services
 *     actors
 *     messages
 *     channels
 *     tasks
 *     capabilities
 *     topology abstractions
 *
 * It must not infer a fixed node count or deployment topology from source
 * syntax.
 *
 * ============================================================================
 * AI / KNOWLEDGE CONTRACT
 * ============================================================================
 *
 * Reflection may inspect semantic descriptions of:
 *
 *     models
 *     reasoning operations
 *     knowledge structures
 *     evidence
 *     provenance
 *     learning operations
 *     policies
 *     decisions
 *
 * The AI subsystem remains the owner of AI semantics.
 *
 * This grammar does not define:
 *
 *     model architectures
 *     learning algorithms
 *     inference algorithms
 *     knowledge graph formats
 *     probability implementations
 *
 * ============================================================================
 * METAPROGRAMMING CONTRACT
 * ============================================================================
 *
 * Source-level reflection remains owned by:
 *
 *     grammar/metaprogramming/reflection.g4
 *
 * That grammar exposes:
 *
 *     reflectionExpressionCore
 *
 * This file MUST NOT import that grammar as a dependency.
 *
 * The semantic integration is instead:
 *
 *     reflectionExpressionCore
 *             |
 *             v
 *        canonical AST
 *             |
 *             v
 *      semantic reflection
 *             |
 *             v
 *        effect analysis
 *             |
 *             v
 *       reflection effect
 *
 * This direction prevents:
 *
 *     Effects -> Reflection -> Metaprogramming -> Expressions -> Effects
 *
 * circularity.
 *
 * ============================================================================
 * COMPILE-TIME CONTRACT
 * ============================================================================
 *
 * Compile-time reflection is permitted only through the ordinary compile-time
 * semantic subsystem.
 *
 * This grammar does not execute reflection.
 *
 * The compile-time system must determine:
 *
 *     phase
 *     capability
 *     effect
 *     resource
 *     policy
 *     provenance
 *     determinism
 *
 * before evaluation.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime reflection is NOT implied by this file.
 *
 * If runtime reflection is supported, semantic analysis must explicitly model:
 *
 *     runtime effect
 *     authorization
 *     capability
 *     resource use
 *     portability
 *     determinism
 *     security
 *     provenance
 *
 * The parser must not make runtime inspection implicit.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Parsing this grammar MUST NOT:
 *
 *     - inspect hardware;
 *     - inspect memory;
 *     - inspect processes;
 *     - inspect files;
 *     - inspect credentials;
 *     - inspect environment variables;
 *     - inspect network state;
 *     - contact devices;
 *     - contact QPUs;
 *     - contact GPUs;
 *     - execute host code;
 *     - execute generated code.
 *
 * Reflection security is enforced downstream.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This file introduces a NEW effect-domain adapter:
 *
 *     grammar/effects/reflection.g4
 *
 * It does not replace:
 *
 *     grammar/metaprogramming/reflection.g4
 *
 * Existing reflection source syntax remains owned by the metaprogramming
 * grammar.
 *
 * No legacy reflection syntax is duplicated here.
 *
 * If compatibility aliases are required, they belong to:
 *
 *     grammar/compatibility/
 *
 * rather than this canonical effect grammar.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics are limited to malformed generic effect structures.
 *
 * Examples:
 *
 *     malformed effect reference
 *     malformed effect reference list
 *     malformed effect set
 *     malformed effect operation reference
 *     malformed effect invocation
 *     malformed effect operation use
 *
 * Semantic diagnostics include:
 *
 *     unknown reflection effect
 *     unknown reflection operation
 *     unauthorized reflection
 *     inaccessible reflected entity
 *     unavailable capability
 *     unavailable resource
 *     forbidden runtime inspection
 *     forbidden implementation-detail exposure
 *     non-portable reflection
 *     phase violation
 *     policy violation
 *
 * Semantic diagnostics belong outside this grammar.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Recommended test owner:
 *
 *     grammar/tests/effects/reflection/
 *
 * Required groups:
 *
 *     positive/
 *     negative/
 *     boundary/
 *     scalability/
 *     determinism/
 *     cross-domain/
 *     compatibility/
 *     round-trip/
 *
 * ============================================================================
 * POSITIVE TEST SHAPES
 * ============================================================================
 *
 * These are semantic/source-shape examples, not built-in effect declarations:
 *
 *     reflection
 *
 *     reflection::read
 *
 *     reflection::metadata
 *
 *     semantic::reflection::inspect
 *
 *     future::reflection::operation
 *
 *     vendor::extension::reflection
 *
 * Generic effect invocation examples:
 *
 *     reflection::read(value)
 *
 *     reflection::metadata(type_info)
 *
 *     semantic::reflection::inspect(subject, selector)
 *
 * These names remain open-world.
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * This adapter must not accept malformed generic effect syntax.
 *
 * Examples include malformed forms equivalent to:
 *
 *     missing qualified name
 *     missing operation name
 *     missing invocation delimiter
 *     malformed argument structure
 *     malformed effect set
 *
 * It must NOT attempt to turn semantic errors into parser alternatives.
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Test:
 *
 *     one-segment effect names
 *     deeply qualified effect names
 *     large effect sets
 *     deeply nested generic expressions
 *     long argument lists
 *     long qualified names
 *     Unicode identifiers where supported
 *     comments and whitespace
 *     source spans at EOF
 *     mixed reflection/classical effects
 *     mixed reflection/quantum effects
 *     mixed reflection/HDL effects
 *     mixed reflection/distributed effects
 *     mixed reflection/AI effects
 *
 * No fixed boundary value is part of language semantics.
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * The grammar must remain structurally capable of representing:
 *
 *     reflection
 *     reflection::a
 *     reflection::a::b
 *     reflection::a::b::c
 *     ...
 *
 * and:
 *
 *     reflection::inspect(x)
 *     reflection::inspect(x, y)
 *     reflection::inspect(x, y, z)
 *     ...
 *
 * and effect sets containing arbitrary numbers of semantically valid entries.
 *
 * The test suite may generate progressively larger programs until available
 * implementation resources are exhausted.
 *
 * Such exhaustion is an implementation/resource condition, not a language
 * ceiling.
 *
 * ============================================================================
 * CROSS-DOMAIN TEST CONTRACT
 * ============================================================================
 *
 * Reflection effects must be composable with:
 *
 *     learning
 *     adaptation
 *     randomness
 *     mutation
 *     IO
 *     network
 *     distributed
 *     native
 *     foreign
 *     quantum
 *     measurement
 *     hardware
 *     security
 *     simulation
 *     future effects
 *
 * Example semantic composition:
 *
 *     reflection
 *     +
 *     learning
 *     +
 *     quantum::measurement
 *     +
 *     distributed::message
 *
 * The grammar does not determine whether such a combination is authorized or
 * realizable.
 *
 * ============================================================================
 * ROUND-TRIP CONTRACT
 * ============================================================================
 *
 * If the formatter/printer supports effect syntax, parsing and formatting
 * must preserve:
 *
 *     effect identity
 *     qualified-name structure
 *     operation identity
 *     argument order
 *     effect-set membership order where source order is semantically relevant
 *
 * The parser must not normalize semantic identities prematurely.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar MUST remain free of:
 *
 *     MAX_REFLECTION_EFFECTS
 *     MAX_REFLECTION_OPERATIONS
 *     MAX_REFLECTION_ARGUMENTS
 *     MAX_REFLECTION_DEPTH
 *     MAX_REFLECTIONS
 *     MAX_METADATA
 *     MAX_DECLARATIONS
 *     MAX_TYPES
 *     MAX_MEMBERS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * It must also remain free of fixed:
 *
 *     CPU identifiers
 *     GPU identifiers
 *     FPGA identifiers
 *     QPU identifiers
 *     physical qubit identifiers
 *     node identifiers
 *     device identifiers
 *     vendor-specific hardware identifiers
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar ReflectionEffects;

options {
    tokenVocab = ZamaniLexer;
}

import
    Core,
    Expressions,
    EffectOperations,
    EffectSets
    ;


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * This is the sole public reflection-effect integration rule.
 *
 * It does not enumerate reflection operations.
 */

reflectionEffect
    : reflectionEffectReference
    | reflectionEffectReferenceList
    | reflectionEffectSet
    | reflectionEffectInvocation
    | reflectionEffectOperationUse
    | reflectionEffectOperationReference
    ;


/*
 * ============================================================================
 * 2. REFLECTION EFFECT REFERENCE
 * ============================================================================
 *
 * Delegates effect identity to the generic effect subsystem.
 *
 * Examples of semantic source shapes:
 *
 *     reflection
 *     reflection::metadata
 *     semantic::reflection::inspect
 *     future::reflection::operation
 *
 * The names are NOT built-in declarations.
 *
 * Semantic analysis decides whether a resolved identity is a reflection
 * effect.
 */

reflectionEffectReference
    : effectReference
    ;


/*
 * ============================================================================
 * 3. REFLECTION EFFECT REFERENCE LIST
 * ============================================================================
 *
 * Delegates list structure to the generic effect subsystem.
 *
 * Source order remains available to the frontend.
 */

reflectionEffectReferenceList
    : effectReferenceList
    ;


/*
 * ============================================================================
 * 4. REFLECTION EFFECT SET
 * ============================================================================
 *
 * Delegates effect-set syntax to the canonical generic effect system.
 *
 * Example source shape:
 *
 *     {
 *         reflection,
 *         learning,
 *         provenance::record
 *     }
 *
 * No effect-count ceiling is encoded.
 */

reflectionEffectSet
    : effectSet
    ;


/*
 * ============================================================================
 * 5. REFLECTION EFFECT OPERATION REFERENCE
 * ============================================================================
 *
 * Delegates operation identity to the generic effect-operation subsystem.
 *
 * No reflection-operation catalogue is introduced.
 */

reflectionEffectOperationReference
    : effectOperationReference
    ;


/*
 * ============================================================================
 * 6. REFLECTION EFFECT INVOCATION
 * ============================================================================
 *
 * Delegates invocation structure to the generic effect-operation subsystem.
 *
 * Example source shapes:
 *
 *     reflection::read(subject)
 *
 *     reflection::metadata(subject)
 *
 *     semantic::reflection::inspect(subject, selector)
 *
 * These names are semantic examples only.
 */

reflectionEffectInvocation
    : effectInvocation
    ;


/*
 * ============================================================================
 * 7. REFLECTION EFFECT OPERATION USE
 * ============================================================================
 *
 * This is the preferred adapter when another grammar already knows that the
 * occurrence is an effect operation.
 *
 * Generic operation-use syntax remains authoritative.
 */

reflectionEffectOperationUse
    : effectOperationUse
    ;


/*
 * ============================================================================
 * 8. FINAL INTEGRATION INVARIANTS
 * ============================================================================
 *
 * INVARIANT 1
 *
 * This grammar defines no lexer rules.
 *
 *
 * INVARIANT 2
 *
 * This grammar does not define `reflect`.
 *
 *
 * INVARIANT 3
 *
 * This grammar does not define `reflectionExpression`.
 *
 *
 * INVARIANT 4
 *
 * This grammar does not define `reflectionExpressionCore`.
 *
 *
 * INVARIANT 5
 *
 * Source-level reflection remains owned by:
 *
 *     grammar/metaprogramming/reflection.g4
 *
 *
 * INVARIANT 6
 *
 * Generic effect syntax remains owned by the generic effect subsystem.
 *
 *
 * INVARIANT 7
 *
 * No reflection operation catalogue is embedded.
 *
 *
 * INVARIANT 8
 *
 * No reflection selector catalogue is embedded.
 *
 *
 * INVARIANT 9
 *
 * No hardware capability catalogue is embedded.
 *
 *
 * INVARIANT 10
 *
 * No quantum operation catalogue is embedded.
 *
 *
 * INVARIANT 11
 *
 * No runtime inspection is performed by parsing.
 *
 *
 * INVARIANT 12
 *
 * No capability is discovered by parsing.
 *
 *
 * INVARIANT 13
 *
 * No resource is allocated by parsing.
 *
 *
 * INVARIANT 14
 *
 * No policy is evaluated by parsing.
 *
 *
 * INVARIANT 15
 *
 * No provenance record is created by parsing.
 *
 *
 * INVARIANT 16
 *
 * No IR is created by this grammar.
 *
 *
 * INVARIANT 17
 *
 * Quantum semantics remain downstream through quantum::ir.
 *
 *
 * INVARIANT 18
 *
 * HDL/hardware realization remains downstream.
 *
 *
 * INVARIANT 19
 *
 * Distributed realization remains downstream.
 *
 *
 * INVARIANT 20
 *
 * Reflection remains open-world.
 *
 *
 * INVARIANT 21
 *
 * No physical machine capacity is represented as a language limit.
 *
 *
 * INVARIANT 22
 *
 * Parsing is deterministic.
 *
 *
 * INVARIANT 23
 *
 * The grammar contains no embedded unsafe implementation.
 *
 *
 * INVARIANT 24
 *
 * Rust 1.97 / 1.97.1 and Rust 2021 remain the consumer baseline.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [x] The filename is retained as grammar/effects/reflection.g4.
 *
 * [x] Grammar identity is ReflectionEffects.
 *
 * [x] Canonical ZamaniLexer is consumed.
 *
 * [x] Generic effect references are delegated.
 *
 * [x] Generic effect sets are delegated.
 *
 * [x] Generic operation references are delegated.
 *
 * [x] Generic effect invocations are delegated.
 *
 * [x] Generic operation uses are delegated.
 *
 * [x] No reflection expression syntax is duplicated.
 *
 * [x] No reflection selector list is hard-coded.
 *
 * [x] No reflection operation list is hard-coded.
 *
 * [x] No finite resource capacity is encoded.
 *
 * [x] No hardware topology is encoded.
 *
 * [x] No physical device is encoded.
 *
 * [x] No quantum gate set is encoded.
 *
 * [x] No quantum IR is introduced.
 *
 * [x] No HDL IR is introduced.
 *
 * [x] No runtime implementation is introduced.
 *
 * [x] No unsafe Rust requirement exists.
 *
 * [x] Rust 1.97 / 1.97.1 compatibility is documented.
 *
 * [x] AST ownership is documented.
 *
 * [x] semantic ownership is documented.
 *
 * [x] capability ownership is documented.
 *
 * [x] resource ownership is documented.
 *
 * [x] policy ownership is documented.
 *
 * [x] provenance ownership is documented.
 *
 * [x] quantum boundary is documented.
 *
 * [x] HDL/hardware boundary is documented.
 *
 * [x] distributed boundary is documented.
 *
 * [x] metaprogramming boundary is documented.
 *
 * [x] test ownership is documented.
 *
 * [x] scalability is documented.
 *
 * [x] determinism is documented.
 *
 * [x] compatibility is documented.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL INVARIANT
 * ============================================================================
 *
 * This file answers:
 *
 *     "How can reflection participate in Zamani's generic effect system?"
 *
 * It does NOT answer:
 *
 *     "How is reflection written?"
 *     "What reflection selectors exist?"
 *     "What entity is reflected?"
 *     "What hardware exists?"
 *     "Which QPU should execute something?"
 *     "Which resource should be allocated?"
 *     "Which policy should authorize it?"
 *     "Which backend should be selected?"
 *
 * Those responsibilities remain downstream or with their existing owners.
 *
 * The permanent relationship is:
 *
 *     reflection source syntax
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic reflection
 *          |
 *          v
 *     reflection effect
 *          |
 *          +--> capabilities
 *          +--> resources
 *          +--> policies
 *          +--> provenance
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical
 *          +--> quantum::ir
 *          +--> HDL/hardware
 *          +--> distributed
 *          +--> accelerator
 *          +--> future domains
 *
 * This preserves POCO-REAF while allowing reflection to participate in the
 * universal effect system without creating a second language or a target-
 * specific reflection model.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */
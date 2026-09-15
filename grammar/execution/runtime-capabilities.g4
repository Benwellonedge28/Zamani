/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/execution/runtime-capabilities.g4
 *
 * Grammar:
 *     RuntimeCapabilities
 *
 * Status:
 *     Production-ready execution/runtime capability-intent parser grammar
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar defines SOURCE-LEVEL RUNTIME CAPABILITY INTENT.
 *
 * It allows a Zamani program to express:
 *
 *     - capabilities it requires;
 *     - capabilities it permits;
 *     - capabilities it prefers;
 *     - capabilities it can optionally use;
 *     - capability relationships;
 *     - capability conditions;
 *     - capability fallback intent;
 *     - capability compatibility intent;
 *     - capability version intent;
 *     - capability negotiation intent;
 *     - runtime-service requirements;
 *     - execution-environment requirements.
 *
 * It does NOT discover, allocate, select, or control physical resources.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     Core / domain parser
 *          |
 *          v
 *     frontend AST
 *          |
 *          +--> name resolution
 *          +--> type analysis
 *          +--> effect analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> target resolution
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL / hardware representation
 *          +--> distributed representation
 *          +--> capability metadata
 *          |
 *          v
 *     optimization / lowering
 *          |
 *          +--> routing
 *          +--> scheduling
 *          +--> resilience
 *          +--> hardware HAL
 *          |
 *          v
 *     runtime capability negotiation
 *          |
 *          v
 *     dispatch / execution
 *
 * ============================================================================
 * CORE PRINCIPLE
 * ============================================================================
 *
 * This grammar describes:
 *
 *     WHAT runtime capability is required or desired.
 *
 * It does NOT describe:
 *
 *     HOW that capability is implemented.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - runtime capability requirement syntax;
 *     - runtime capability preference syntax;
 *     - runtime capability optionality syntax;
 *     - runtime capability negotiation intent;
 *     - runtime capability compatibility intent;
 *     - runtime capability version intent;
 *     - runtime capability condition syntax;
 *     - capability fallback intent;
 *     - capability dependency relationships;
 *     - capability conflict declarations;
 *     - capability property expressions;
 *     - capability requirement composition.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer tokens;
 *     - identifiers;
 *     - qualified names;
 *     - ordinary expressions;
 *     - types;
 *     - generic resource syntax;
 *     - physical resources;
 *     - hardware discovery;
 *     - hardware calibration;
 *     - hardware topology;
 *     - placement;
 *     - routing;
 *     - scheduling;
 *     - dispatch implementation;
 *     - runtime implementation;
 *     - device selection;
 *     - resource allocation;
 *     - capability probing;
 *     - capability enforcement;
 *     - authentication;
 *     - authorization;
 *     - quantum IR;
 *     - classical IR;
 *     - QEC;
 *     - ZQN;
 *     - optimization.
 *
 * ============================================================================
 * OPEN-WORLD CAPABILITY MODEL
 * ============================================================================
 *
 * Runtime capabilities are intentionally OPEN-ENDED.
 *
 * The grammar MUST NOT contain a finite enumeration such as:
 *
 *     cpu
 *     gpu
 *     qpu
 *     fpga
 *     ...
 *
 * as the complete capability vocabulary.
 *
 * A capability identity is represented through canonical names and expressions.
 *
 * Examples of semantic capability identities include:
 *
 *     quantum
 *     dynamic_quantum_control
 *     mid_circuit_measurement
 *     tensor_compute
 *     distributed_execution
 *     remote_execution
 *     realtime
 *     hardware_synthesis
 *     secure_execution
 *     custom.future.capability
 *
 * New capabilities therefore do not require changing this grammar.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * A capability declaration MUST NOT bind a program to a physical machine.
 *
 * For example:
 *
 *     requires quantum;
 *
 * means that the computation requires quantum capability.
 *
 * It does NOT mean:
 *
 *     use QPU 0
 *     use 127 qubits
 *     use IBM device X
 *     use topology Y
 *
 * Those decisions belong downstream.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * No finite execution-resource limits are encoded here.
 *
 * This grammar contains no:
 *
 *     MAX_CAPABILITIES
 *     MAX_DEVICES
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_RESOURCES
 *
 * Repeated capability declarations are represented structurally.
 *
 * Actual limits belong to:
 *
 *     - semantic analysis;
 *     - resource analysis;
 *     - runtime policy;
 *     - operating-system limits;
 *     - target capabilities;
 *     - provider limits;
 *     - deployment policy.
 *
 * ============================================================================
 * SEMANTIC VS PHYSICAL CAPABILITIES
 * ============================================================================
 *
 * A capability may describe an abstract ability:
 *
 *     quantum
 *     floating_point
 *     vectorization
 *     dynamic_control
 *
 * A resource describes an available quantity:
 *
 *     memory
 *     processing capacity
 *     storage
 *     communication capacity
 *
 * A target identifies an execution environment.
 *
 * A device identifies a concrete implementation only when the downstream
 * target/deployment model explicitly requires such identity.
 *
 * This grammar MUST NOT collapse these concepts.
 *
 * ============================================================================
 * CAPABILITY NEGOTIATION
 * ============================================================================
 *
 * The grammar expresses negotiation intent.
 *
 * The runtime determines:
 *
 *     - what capabilities are actually available;
 *     - whether requirements can be satisfied;
 *     - whether preferences can be honored;
 *     - whether fallback is possible;
 *     - whether execution must be rejected.
 *
 * The parser never performs negotiation.
 *
 * ============================================================================
 * CAPABILITY OBSERVABILITY
 * ============================================================================
 *
 * A capability can be queried through expressions where the language/runtime
 * provides an appropriate capability-query semantic operation.
 *
 * This grammar does not introduce a second runtime API.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no semantic actions;
 *     - no Rust code;
 *     - no I/O;
 *     - no network access;
 *     - no filesystem access;
 *     - no hardware discovery;
 *     - no random behavior;
 *     - no global mutable state.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * The generated parser is consumed by the Rust implementation.
 *
 * Runtime/compiler implementation requirements:
 *
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust
 *     no unsafe
 *
 * ============================================================================
 */

parser grammar RuntimeCapabilities;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * This is the rule imported/exposed by the execution grammar.
 *
 * It deliberately does not require a newly invented lexer keyword such as
 * RUNTIME or CAPABILITY.
 *
 * This prevents the capability vocabulary from becoming a closed lexical
 * namespace.
 *
 * ============================================================================
 */

runtimeCapabilities
    : runtimeCapabilityClause+
    ;


/*
 * ============================================================================
 * CAPABILITY CLAUSES
 * ============================================================================
 */

runtimeCapabilityClause
    : runtimeCapabilityRequirement
    | runtimeCapabilityPreference
    | runtimeCapabilityOptional
    | runtimeCapabilityNegotiation
    | runtimeCapabilityCompatibility
    | runtimeCapabilityDependency
    | runtimeCapabilityConflict
    | runtimeCapabilityFallback
    | runtimeCapabilityCondition
    | runtimeCapabilityAssertion
    | runtimeCapabilityProperty
    ;


/*
 * ============================================================================
 * REQUIRED CAPABILITY
 * ============================================================================
 *
 * Canonical semantic form:
 *
 *     requires capability;
 *
 *     requires capability(argument);
 *
 *     requires capability { ... };
 *
 * The capability itself is an open semantic name.
 *
 * ============================================================================
 */

runtimeCapabilityRequirement
    : REQUIRES runtimeCapabilitySpec runtimeCapabilityTerminator?
    ;


runtimeCapabilitySpec
    : runtimeCapabilityReference
    | runtimeCapabilityInvocation
    | runtimeCapabilityPropertyBlock
    ;


runtimeCapabilityReference
    : qualifiedName
    ;


runtimeCapabilityInvocation
    : qualifiedName LPAREN argumentList? RPAREN
    ;


/*
 * ============================================================================
 * PREFERRED CAPABILITY
 * ============================================================================
 *
 * Preferences do not make an execution invalid merely because the runtime
 * cannot satisfy them.
 *
 * The semantic layer decides whether a preference can be honored.
 * ============================================================================
 */

runtimeCapabilityPreference
    : WITH runtimeCapabilityPreferenceSpec runtimeCapabilityTerminator?
    ;


runtimeCapabilityPreferenceSpec
    : runtimeCapabilityReference
    | runtimeCapabilityInvocation
    | runtimeCapabilityPropertyBlock
    ;


/*
 * ============================================================================
 * OPTIONAL CAPABILITY
 * ============================================================================
 *
 * An optional capability may be used when available but MUST NOT become an
 * unconditional execution requirement.
 *
 * ============================================================================
 */

runtimeCapabilityOptional
    : runtimeCapabilityOptionalMarker
      runtimeCapabilitySpec
      runtimeCapabilityTerminator?
    ;


runtimeCapabilityOptionalMarker
    : identifier
    ;


/*
 * ============================================================================
 * NEGOTIATION
 * ============================================================================
 *
 * Negotiation describes intent to resolve a capability against the runtime
 * capability environment.
 *
 * The actual negotiation algorithm belongs to runtime infrastructure.
 *
 * ============================================================================
 */

runtimeCapabilityNegotiation
    : runtimeCapabilityNegotiationMarker
      runtimeCapabilityNegotiationBody
      runtimeCapabilityTerminator?
    ;


runtimeCapabilityNegotiationMarker
    : identifier
    ;


runtimeCapabilityNegotiationBody
    : LBRACE runtimeCapabilityNegotiationClause* RBRACE
    ;


runtimeCapabilityNegotiationClause
    : runtimeCapabilityRequirement
    | runtimeCapabilityPreference
    | runtimeCapabilityOptional
    | runtimeCapabilityCompatibility
    | runtimeCapabilityFallback
    | runtimeCapabilityProperty
    ;


/*
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Capability compatibility is expressed as semantic properties rather than a
 * fixed vendor/backend matrix.
 *
 * ============================================================================
 */

runtimeCapabilityCompatibility
    : runtimeCapabilityCompatibilityMarker
      runtimeCapabilityCompatibilityValue
      runtimeCapabilityTerminator?
    ;


runtimeCapabilityCompatibilityMarker
    : identifier
    ;


runtimeCapabilityCompatibilityValue
    : expression
    | runtimeCapabilityPropertyBlock
    ;


/*
 * ============================================================================
 * CAPABILITY DEPENDENCIES
 * ============================================================================
 *
 * A capability may depend on another capability.
 *
 * Example semantic form:
 *
 *     capabilityA requires capabilityB
 *
 * The actual semantic interpretation belongs to capability analysis.
 *
 * ============================================================================
 */

runtimeCapabilityDependency
    : runtimeCapabilityReference
      REQUIRES
      runtimeCapabilityReference
      runtimeCapabilityTerminator?
    ;


/*
 * ============================================================================
 * CAPABILITY CONFLICTS
 * ============================================================================
 *
 * Expresses that two capability intents cannot simultaneously be satisfied
 * under the same semantic context.
 *
 * ============================================================================
 */

runtimeCapabilityConflict
    : runtimeCapabilityConflictMarker
      runtimeCapabilityReferenceList
      runtimeCapabilityTerminator?
    ;


runtimeCapabilityConflictMarker
    : identifier
    ;


runtimeCapabilityReferenceList
    : runtimeCapabilityReference
      (COMMA runtimeCapabilityReference)*
    ;


/*
 * ============================================================================
 * FALLBACK
 * ============================================================================
 *
 * Fallback expresses an alternative capability realization.
 *
 * It does not itself choose which alternative will execute.
 *
 * ============================================================================
 */

runtimeCapabilityFallback
    : runtimeCapabilityFallbackMarker
      runtimeCapabilityAlternative
      runtimeCapabilityFallbackAlternative?
      runtimeCapabilityTerminator?
    ;


runtimeCapabilityFallbackMarker
    : identifier
    ;


runtimeCapabilityAlternative
    : runtimeCapabilityReference
    | runtimeCapabilityInvocation
    | runtimeCapabilityPropertyBlock
    ;


runtimeCapabilityFallbackAlternative
    : identifier
      runtimeCapabilityAlternativeValue
    ;


runtimeCapabilityAlternativeValue
    : runtimeCapabilityReference
    | runtimeCapabilityInvocation
    | runtimeCapabilityPropertyBlock
    ;


/*
 * ============================================================================
 * CONDITIONAL CAPABILITY INTENT
 * ============================================================================
 *
 * Capability use can depend on a semantic condition.
 *
 * The condition is an ordinary Zamani expression.
 *
 * ============================================================================
 */

runtimeCapabilityCondition
    : WHEN expression
      runtimeCapabilityConditionalBody?
      runtimeCapabilityTerminator?
    ;


runtimeCapabilityConditionalBody
    : runtimeCapabilitySpec
    | runtimeCapabilityPropertyBlock
    ;


/*
 * ============================================================================
 * CAPABILITY ASSERTION
 * ============================================================================
 *
 * Assertions describe conditions that must hold for the capability contract.
 *
 * The grammar does not evaluate the assertion.
 *
 * ============================================================================
 */

runtimeCapabilityAssertion
    : ASSERT runtimeCapabilityPredicate runtimeCapabilityTerminator?
    ;


runtimeCapabilityPredicate
    : expression
    ;


runtimeCapabilityProperty
    : runtimeCapabilityPropertyName
      COLON
      runtimeCapabilityPropertyValue
      runtimeCapabilityTerminator?
    ;


runtimeCapabilityPropertyName
    : identifier
    ;


runtimeCapabilityPropertyValue
    : expression
    | runtimeCapabilityPropertyBlock
    ;


/*
 * ============================================================================
 * PROPERTY BLOCK
 * ============================================================================
 *
 * Property names remain open identifiers.
 *
 * This allows future capability metadata without changing the grammar.
 *
 * ============================================================================
 */

runtimeCapabilityPropertyBlock
    : LBRACE runtimeCapabilityPropertyEntry* RBRACE
    ;


runtimeCapabilityPropertyEntry
    : identifier
      COLON
      runtimeCapabilityPropertyValue
      runtimeCapabilityTerminator?
    ;


/*
 * ============================================================================
 * VERSION / FEATURE EXPRESSIONS
 * ============================================================================
 *
 * Version values intentionally remain expressions.
 *
 * The grammar does not impose a universal versioning scheme.
 *
 * Semantic layers may interpret:
 *
 *     semantic versions;
 *     capability revisions;
 *     feature levels;
 *     compatibility ranges;
 *     provider capability versions;
 *     implementation revisions.
 *
 * ============================================================================
 */

runtimeCapabilityVersion
    : runtimeCapabilityVersionMarker
      expression
    ;


runtimeCapabilityVersionMarker
    : identifier
    ;


/*
 * ============================================================================
 * CAPABILITY REQUIREMENT EXPRESSIONS
 * ============================================================================
 *
 * A capability requirement may carry structured semantic properties.
 *
 * Examples:
 *
 *     requires quantum {
 *         mode: dynamic;
 *     };
 *
 *     requires distributed {
 *         consistency: model;
 *     };
 *
 * No fixed machine property is implied.
 *
 * ============================================================================
 */

runtimeCapabilityRequirementWithProperties
    : REQUIRES runtimeCapabilityReference runtimeCapabilityPropertyBlock
      runtimeCapabilityTerminator?
    ;


/*
 * ============================================================================
 * CAPABILITY QUERY REFERENCE
 * ============================================================================
 *
 * Runtime capability observation is represented as a semantic reference.
 *
 * The runtime owns the actual query mechanism.
 *
 * ============================================================================
 */

runtimeCapabilityQuery
    : runtimeCapabilityQueryMarker
      runtimeCapabilityReference
      runtimeCapabilityTerminator?
    ;


runtimeCapabilityQueryMarker
    : identifier
    ;


/*
 * ============================================================================
 * CAPABILITY SET
 * ============================================================================
 *
 * Capability collections remain dynamically sized.
 *
 * ============================================================================
 */

runtimeCapabilitySet
    : LBRACKET
      runtimeCapabilityReferenceList?
      RBRACKET
    ;


/*
 * ============================================================================
 * CAPABILITY EXPRESSION
 * ============================================================================
 *
 * This rule provides a reusable capability expression for execution grammar
 * composition.
 *
 * ============================================================================
 */

runtimeCapabilityExpression
    : runtimeCapabilityReference
    | runtimeCapabilityInvocation
    | runtimeCapabilitySet
    | runtimeCapabilityPropertyBlock
    | expression
    ;


/*
 * ============================================================================
 * GENERIC CAPABILITY ATTRIBUTE
 * ============================================================================
 *
 * This provides extensibility without forcing future capability concepts into
 * the lexer.
 *
 * ============================================================================
 */

runtimeCapabilityAttribute
    : identifier
      (ASSIGN expression)?
    ;


/*
 * ============================================================================
 * ATTRIBUTE LIST
 * ============================================================================
 */

runtimeCapabilityAttributeList
    : runtimeCapabilityAttribute
      (COMMA runtimeCapabilityAttribute)*
      COMMA?
    ;


/*
 * ============================================================================
 * CAPABILITY DECLARATION BODY
 * ============================================================================
 *
 * This is useful to higher-level execution grammars that want to embed a
 * capability contract without duplicating the property grammar.
 *
 * ============================================================================
 */

runtimeCapabilityDeclarationBody
    : LBRACE
      runtimeCapabilityDeclarationEntry*
      RBRACE
    ;


runtimeCapabilityDeclarationEntry
    : runtimeCapabilityRequirement
    | runtimeCapabilityPreference
    | runtimeCapabilityOptional
    | runtimeCapabilityCompatibility
    | runtimeCapabilityDependency
    | runtimeCapabilityConflict
    | runtimeCapabilityFallback
    | runtimeCapabilityCondition
    | runtimeCapabilityAssertion
    | runtimeCapabilityProperty
    ;


/*
 * ============================================================================
 * TERMINATOR
 * ============================================================================
 *
 * SEMI is the canonical statement terminator.
 *
 * The optional form allows composition inside larger execution constructs
 * where the surrounding grammar owns termination.
 *
 * ============================================================================
 */

runtimeCapabilityTerminator
    : SEMI
    ;
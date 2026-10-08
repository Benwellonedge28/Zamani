/*
 * ============================================================================
 * ZAMANI UNIVERSAL PROGRAMMING LANGUAGE
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/networking/policies.g4
 *
 * GRAMMAR
 * -------
 * NetworkingPolicies
 *
 * STATUS
 * ------
 * PRODUCTION NETWORKING-POLICY ADAPTER GRAMMAR
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 1.97+
 * Rust 2021
 * ANTLR4
 * Safe Rust only
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file provides the networking-domain integration boundary for Zamani's
 * canonical policy system.
 *
 * It does NOT define another policy language.
 *
 * The universal policy language is owned by:
 *
 *     grammar/policies/policy.g4
 *
 * The policy-domain composition boundary is owned by:
 *
 *     grammar/policies/policies.g4
 *
 * This file merely adapts that canonical policy model into the networking
 * grammar so that networking can expose policy intent without duplicating
 * policy syntax.
 *
 *
 * NETWORKING POLICY MAY GOVERN
 * ----------------------------
 *
 * Networking policy may semantically influence:
 *
 *     service selection
 *     service admission
 *     endpoint selection
 *     protocol selection
 *     channel selection
 *     route selection
 *     stream behavior
 *     request behavior
 *     response behavior
 *     discovery behavior
 *     capability negotiation
 *     resource requirements
 *     communication security
 *     reliability
 *     ordering
 *     delivery requirements
 *     retry
 *     recovery
 *     fallback
 *     simulation
 *     deterministic execution
 *     reproducibility
 *     adaptation
 *     provenance
 *     distributed communication
 *
 * The grammar does not perform any of those operations.
 *
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
 *     ZamaniParser
 *          |
 *          v
 *     Networking
 *          |
 *          v
 *     NetworkingPolicies
 *          |
 *          v
 *     canonical Policy grammar
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     structural validation
 *          |
 *          +--> type analysis
 *          +--> effect analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> contract analysis
 *          +--> policy analysis
 *          +--> provenance analysis
 *          +--> security analysis
 *          |
 *          v
 *     networking semantic model
 *          |
 *          v
 *     target-independent planning
 *          |
 *          +--> service resolution
 *          +--> endpoint resolution
 *          +--> protocol selection
 *          +--> route planning
 *          +--> scheduling
 *          +--> resilience
 *          |
 *          v
 *     canonical IR / domain representation
 *          |
 *          v
 *     target realization
 *
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     networkingPolicyConstruct
 *     networkingPolicyDeclaration
 *     networkingPolicyReference
 *     networkingPolicyBody
 *     networkingPolicyPropertyReference
 *
 * These are NETWORKING ADAPTER RULES.
 *
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     policyDeclaration
 *     policyBody
 *     policyMember
 *     policyRule
 *     policyRequirement
 *     policyConstraint
 *     policyCapability
 *     policyResource
 *     policyPermission
 *     policyProhibition
 *     policyPreference
 *     policyFallback
 *     policySelection
 *     policyNegotiation
 *     policyRetry
 *     policyRecovery
 *     policyEscalation
 *     policyRejection
 *     policySimulation
 *     policyAdaptation
 *     policySandbox
 *     policyDeterminism
 *     policyReproducibility
 *     policyEffect
 *     policyProvenance
 *     policyContractReference
 *     policyComposition
 *     policyProperty
 *
 * Those remain owned by:
 *
 *     grammar/policies/policy.g4
 *
 *
 * It also does NOT own:
 *
 *     services
 *     endpoints
 *     addresses
 *     protocols
 *     channels
 *     messages
 *     requests
 *     responses
 *     sockets
 *     streams
 *     routing
 *     discovery
 *     network capabilities
 *     resource allocation
 *     scheduling
 *     placement
 *     transport implementation
 *     authentication
 *     authorization enforcement
 *     cryptography
 *     distributed execution
 *     hardware realization
 *     quantum operations
 *     quantum topology
 *     quantum::ir
 *     classical IR
 *     HDL IR
 *     ZQN
 *     HAL
 *     runtime behavior
 *
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There MUST be exactly one source-level policy syntax authority.
 *
 * That authority is:
 *
 *     grammar/policies/policy.g4
 *
 * This file MUST NEVER copy the policy grammar into networking.
 *
 * Therefore a change to:
 *
 *     when
 *     requires
 *     capability
 *     resource
 *     allow
 *     forbid
 *     prefer
 *     fallback
 *     adapt
 *     sandbox
 *     simulate
 *     provenance
 *     reproducible
 *
 * belongs to the canonical policy subsystem, not this file.
 *
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/policies/policy.g4
 *     grammar/core/names.g4
 *     grammar/expressions/
 *     grammar/antlr/ZamaniLexer.g4
 *
 *
 * IMPORTS:
 *
 *     Policy
 *     Names
 *     Expressions
 *
 *
 * EXPORTS:
 *
 *     networkingPolicyConstruct
 *     networkingPolicyDeclaration
 *     networkingPolicyReference
 *     networkingPolicyBody
 *     networkingPolicyPropertyReference
 *
 *
 * CONSUMED_BY:
 *
 *     grammar/networking/networking.g4
 *     networking semantic frontend
 *
 *
 * AST_OWNER:
 *
 *     networking frontend AST adapter
 *
 *
 * SEMANTIC_OWNER:
 *
 *     networking semantic layer plus canonical policy semantic model
 *
 *
 * IR_OWNER:
 *
 *     canonical semantic/IR lowering layer
 *
 *
 * TEST_OWNER:
 *
 *     grammar/tests/networking/policies/
 *
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/networking.md
 *     grammar/spec/policies.md
 *
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * The canonical Policy grammar is imported rather than copied.
 *
 * This allows networking to reuse:
 *
 *     policyDeclaration
 *     policyBody
 *     policyMember
 *     policyExpression
 *     policyProperty
 *
 * without establishing another policy authority.
 *
 *
 * ============================================================================
 * POLICY DECLARATION ADAPTER
 * ============================================================================
 *
 * A complete policy declaration is owned by the universal policy grammar.
 *
 * This adapter gives the networking aggregate a stable networking-specific
 * entry point for that declaration.
 *
 * Example:
 *
 *     policy networking.communication {
 *         requires capability("network.communication");
 *         prefer network::reliable;
 *     }
 *
 * The policy itself remains a universal policy.
 *
 * Its networking interpretation occurs downstream.
 *
 * ============================================================================
 */

parser grammar NetworkingPolicies;

options {
    tokenVocab = ZamaniLexer;
}

import
    Policy,
    Names,
    Expressions
;


/*
 * ============================================================================
 * PUBLIC COMPOSITION ENTRY POINT
 * ============================================================================
 *
 * This is the only rule that the networking aggregate needs to consume.
 *
 * It intentionally delegates the actual policy syntax to Policy.
 * ============================================================================
 */

networkingPolicyConstruct
    : networkingPolicyDeclaration
    ;


/*
 * ============================================================================
 * POLICY DECLARATION ADAPTER
 * ============================================================================
 *
 * IMPORTANT:
 *
 * This is NOT a second policyDeclaration.
 *
 * It is a networking-domain classification wrapper around the canonical
 * universal policy declaration.
 * ============================================================================
 */

networkingPolicyDeclaration
    : policyDeclaration
    ;


/*
 * ============================================================================
 * POLICY REFERENCE
 * ============================================================================
 *
 * Networking constructs frequently need to refer to an already-declared
 * policy, for example:
 *
 *     policy: networking.communication;
 *
 * The generic policy language remains responsible for declaring the policy.
 *
 * This rule only represents the networking-side reference.
 *
 * The reference is deliberately a qualified name so that namespaces,
 * modules, packages, dialects, and future policy registries remain open.
 * ============================================================================
 */

networkingPolicyReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * POLICY BODY ADAPTER
 * ============================================================================
 *
 * This rule exposes the canonical policy body where a networking semantic
 * consumer needs to process a policy body as a nested semantic object.
 *
 * No networking-specific policy members are introduced here.
 * ============================================================================
 */

networkingPolicyBody
    : policyBody
    ;


/*
 * ============================================================================
 * POLICY PROPERTY REFERENCE
 * ============================================================================
 *
 * Networking components may preserve an explicitly named policy-related
 * property as an expression.
 *
 * The property itself remains open-world.
 *
 * Examples of semantic names that MAY be resolved downstream include:
 *
 *     reliability
 *     ordering
 *     delivery
 *     latency
 *     locality
 *     security
 *     retry
 *     recovery
 *     determinism
 *     reproducibility
 *
 * This grammar deliberately does not enumerate those names.
 * ============================================================================
 */

networkingPolicyPropertyReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * NETWORKING POLICY LIST
 * ============================================================================
 *
 * An unbounded collection of policy declarations is useful for independent
 * networking grammar validation and tooling.
 *
 * No language-level maximum exists.
 * ============================================================================
 */

networkingPolicyDeclarationList
    : networkingPolicyDeclaration*
    ;


/*
 * ============================================================================
 * NETWORKING POLICY REFERENCE LIST
 * ============================================================================
 *
 * An unbounded collection of references permits arbitrarily large policy
 * sets, subject only to implementation and target resources.
 * ============================================================================
 */

networkingPolicyReferenceList
    : networkingPolicyReference
      (COMMA networkingPolicyReference)*
    ;


/*
 * ============================================================================
 * NETWORKING POLICY PROPERTY LIST
 * ============================================================================
 *
 * This is a structural helper only.
 *
 * It does not define property semantics.
 * ============================================================================
 */

networkingPolicyPropertyReferenceList
    : networkingPolicyPropertyReference
      (COMMA networkingPolicyPropertyReference)*
    ;


/*
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * After parsing:
 *
 *     networkingPolicyDeclaration
 *          |
 *          v
 *     canonical Policy AST
 *          |
 *          v
 *     networking policy binding
 *          |
 *          v
 *     semantic policy model
 *
 * Semantic analysis determines:
 *
 *     policy identity
 *     policy scope
 *     policy applicability
 *     policy requirements
 *     policy constraints
 *     policy capabilities
 *     policy resources
 *     policy effects
 *     policy permissions
 *     policy prohibitions
 *     policy preferences
 *     policy fallbacks
 *     policy adaptation
 *     policy simulation
 *     policy provenance
 *
 * This grammar performs none of those operations.
 *
 *
 * ============================================================================
 * NETWORKING INTEGRATION BOUNDARY
 * ============================================================================
 *
 * Networking policy MAY be consumed by:
 *
 *     services.g4
 *     requests.g4
 *     responses.g4
 *     protocols.g4
 *     channels.g4
 *     endpoints.g4
 *     sockets.g4
 *     routing.g4
 *     streaming/streams.g4
 *     service-discovery.g4
 *     network-capabilities.g4
 *
 * However, those files MUST NOT import this grammar merely to recreate their
 * existing local policy wrappers.
 *
 * Their existing local rules remain responsible for their own structural
 * syntax.
 *
 * The semantic layer is responsible for associating those references with
 * canonical policies.
 *
 *
 * ============================================================================
 * POLICY ATTACHMENT MODEL
 * ============================================================================
 *
 * A networking component may contain a policy reference using its own
 * component-specific syntax.
 *
 * Conceptually:
 *
 *     service
 *         |
 *         +--> policy reference
 *
 *     request
 *         |
 *         +--> policy reference
 *
 *     protocol
 *         |
 *         +--> policy reference
 *
 *     route
 *         |
 *         +--> policy reference
 *
 *     stream
 *         |
 *         +--> policy reference
 *
 *     discovery
 *         |
 *         +--> policy reference
 *
 * Those component-specific wrappers remain owned by their files.
 *
 * The canonical policy identity is resolved against:
 *
 *     networkingPolicyReference
 *
 * or the equivalent qualified-name representation in the component AST.
 *
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Networking policies may semantically express:
 *
 *     requires capability("network.reliable")
 *     requires capability("network.discovery")
 *     requires memory >= required_memory
 *     prefer network::low_latency
 *     constrain network::topology
 *
 * but this file does not define resource semantics.
 *
 * Resource requirements remain governed by the canonical resource/requirement
 * subsystem.
 *
 * No physical capacity is encoded here.
 *
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Capability identity remains open-world.
 *
 * This grammar does NOT enumerate:
 *
 *     TCP
 *     UDP
 *     QUIC
 *     HTTP
 *     MQTT
 *     gRPC
 *     RDMA
 *     InfiniBand
 *     vendor transports
 *     cloud providers
 *     NICs
 *     routers
 *     switches
 *
 * A new networking capability must not require this grammar to change merely
 * because a new technology becomes available.
 *
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Policy syntax itself introduces no runtime effect.
 *
 * When a networking policy governs an operation that performs:
 *
 *     network I/O
 *     distributed communication
 *     discovery
 *     mutation
 *     adaptation
 *     native execution
 *     foreign execution
 *     simulation
 *
 * the corresponding effects are assigned by semantic analysis.
 *
 * Parsing a policy does not execute it.
 *
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Networking policy may constrain security behavior.
 *
 * This file does not implement:
 *
 *     authentication
 *     authorization
 *     credential validation
 *     cryptographic verification
 *     key management
 *     identity management
 *     trust evaluation
 *     sandbox enforcement
 *
 * Security semantics remain owned by the security subsystem.
 *
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Policy declarations and networking policy references must retain source
 * locations and identity information in the AST.
 *
 * Downstream provenance should be able to trace:
 *
 *     source
 *       ->
 *     policy declaration
 *       ->
 *     networking attachment/reference
 *       ->
 *     semantic policy decision
 *       ->
 *     execution plan
 *       ->
 *     realization
 *
 * This grammar does not generate provenance records itself.
 *
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Policy conditions may interact semantically with:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * but this grammar does not redefine those constructs.
 *
 * Contract ownership remains in:
 *
 *     grammar/validation/
 *
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Networking policies may govern communication involving quantum resources,
 * QPUs, simulators, quantum services, or hybrid execution.
 *
 * This grammar does NOT define:
 *
 *     qubits
 *     quantum operations
 *     measurements
 *     quantum states
 *     circuits
 *     coupling maps
 *     QEC
 *     quantum routing
 *     quantum scheduling
 *
 * Those remain quantum semantic concerns.
 *
 * The downstream boundary is:
 *
 *     networking policy
 *          |
 *          v
 *     hybrid/quantum semantic analysis
 *          |
 *          v
 *     quantum::ir
 *
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Networking policies may constrain communication with hardware or HDL
 * components.
 *
 * This file does not encode:
 *
 *     FPGA dimensions
 *     ASIC resources
 *     register widths
 *     processor counts
 *     memory sizes
 *     physical links
 *     device identifiers
 *     fixed topology sizes
 *
 * Hardware feasibility remains downstream.
 *
 *
 * ============================================================================
 * DISTRIBUTED BOUNDARY
 * ============================================================================
 *
 * Networking policy may participate in:
 *
 *     distributed execution
 *     actor communication
 *     service placement
 *     communication reliability
 *     consistency
 *     failure handling
 *     retry
 *     recovery
 *
 * It does not define:
 *
 *     node counts
 *     cluster sizes
 *     fixed network sizes
 *     physical topology limits
 *
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar contains NO hard-coded physical or logical capacity limit.
 *
 * It does not define:
 *
 *     MAX_POLICIES
 *     MAX_POLICY_RULES
 *     MAX_NETWORK_POLICIES
 *     MAX_NETWORK_NODES
 *     MAX_ENDPOINTS
 *     MAX_SERVICES
 *     MAX_ROUTES
 *     MAX_CHANNELS
 *     MAX_STREAMS
 *     MAX_CONNECTIONS
 *     MAX_CAPABILITIES
 *     MAX_RESOURCES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_DEVICES
 *
 * All collections use ANTLR repetition operators.
 *
 * Actual limitations are determined by:
 *
 *     compiler resources
 *     memory
 *     target resources
 *     capability availability
 *     deployment constraints
 *     runtime resources
 *     physical feasibility
 *
 * None of those limitations become grammar-level ceilings.
 *
 *
 * ============================================================================
 * OPEN-WORLD CONTRACT
 * ============================================================================
 *
 * The networking policy adapter deliberately avoids a closed list of:
 *
 *     transports
 *     providers
 *     protocols
 *     network devices
 *     topology types
 *     routing algorithms
 *     discovery registries
 *     service classes
 *     deployment platforms
 *     hardware targets
 *
 * Future technologies therefore remain representable through the canonical
 * qualified-name, expression, capability, resource, and policy systems.
 *
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no actions
 *     no semantic predicates
 *     no embedded Rust
 *     no filesystem access
 *     no network access
 *     no hardware access
 *     no runtime callbacks
 *     no randomness
 *     no environment inspection
 *
 * Identical input and parser configuration must yield structurally equivalent
 * parse trees.
 *
 *
 * ============================================================================
 * SAFE-RUST CONTRACT
 * ============================================================================
 *
 * This file contains no Rust implementation code.
 *
 * The downstream implementation must remain compatible with:
 *
 *     Rust 1.97+
 *     Rust 2021
 *     safe Rust only
 *
 * No unsafe block, unsafe trait, unsafe function, raw-pointer requirement, or
 * unsafe FFI assumption is introduced by this grammar.
 *
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics cover structural errors.
 *
 * Examples:
 *
 *     malformed policy declaration
 *     missing policy name
 *     missing policy body
 *     malformed qualified name
 *     malformed policy reference
 *     malformed policy property reference
 *
 * Semantic diagnostics belong downstream.
 *
 * Examples:
 *
 *     unknown policy
 *     inaccessible policy
 *     conflicting policy
 *     unsatisfied requirement
 *     unavailable capability
 *     forbidden operation
 *     incompatible networking policy
 *     impossible networking constraint
 *     target infeasibility
 *
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This file introduces no new universal keyword.
 *
 * Existing canonical policy tokens remain owned by the lexer/token registry
 * and canonical policy grammar.
 *
 * Existing networking files remain source-compatible because this adapter does
 * not replace their local policy-reference rules.
 *
 * In particular, existing constructs such as:
 *
 *     service {
 *         policy: security::authenticated;
 *     }
 *
 *     protocol {
 *         policy communication_policy;
 *     }
 *
 *     route compute {
 *         policy {
 *             strategy: adaptive;
 *         }
 *     }
 *
 * continue to belong to their existing networking grammars.
 *
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE TESTS
 * --------------
 *
 *     policy networking.communication {
 *         requires capability("network.communication");
 *     }
 *
 *     policy networking.reliable {
 *         prefer network::reliable;
 *     }
 *
 *     policy networking.adaptive {
 *         fallback execution::simulation;
 *         adapt networking::strategy;
 *     }
 *
 *     policy networking.quantum {
 *         requires capability("quantum.communication");
 *         prefer quantum::resilience;
 *     }
 *
 *     policy networking.hybrid {
 *         requires capability("network.communication");
 *         requires capability("tensor.compute");
 *         reproducible;
 *     }
 *
 *
 * REFERENCE TESTS
 * ---------------
 *
 *     networkingPolicyReference
 *         -> networking.communication
 *
 *     networkingPolicyReference
 *         -> organization.networking.reliable
 *
 *     networkingPolicyReference
 *         -> future::networking::policy
 *
 *
 * NEGATIVE TESTS
 * --------------
 *
 *     policy;
 *
 *     policy networking.communication;
 *
 *     policy networking.communication {
 *
 *     networkingPolicyReference
 *         -> ;
 *
 *     networkingPolicyReference
 *         -> malformed::
 *
 *
 * BOUNDARY TESTS
 * --------------
 *
 * Test policy integration with:
 *
 *     services
 *     requests
 *     responses
 *     protocols
 *     routes
 *     streams
 *     sockets
 *     endpoints
 *     discovery
 *     network capabilities
 *     security
 *     distributed execution
 *     simulation
 *     quantum/hybrid execution
 *
 *
 * SCALABILITY TESTS
 * -----------------
 *
 * Generate arbitrarily large valid policy declarations and policy reference
 * sets.
 *
 * Test:
 *
 *     many policy members
 *     many rules
 *     many requirements
 *     many capabilities
 *     many properties
 *     deeply qualified names
 *     deeply nested expressions
 *     many networking policy references
 *
 * The test suite MUST NOT convert its chosen test size into a language limit.
 *
 *
 * DETERMINISM TESTS
 * -----------------
 *
 * Parse identical networking policy input repeatedly and verify equivalent
 * parse-tree structure.
 *
 *
 * CROSS-DOMAIN TESTS
 * ------------------
 *
 * Verify policies can govern networking relationships involving:
 *
 *     classical computation
 *     quantum computation
 *     HDL
 *     hardware
 *     AI/model execution
 *     distributed execution
 *     simulation
 *     accelerators
 *     future capabilities
 *
 * without changing this grammar.
 *
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * REQUIRED CHANGE 1
 * -----------------
 *
 * Update:
 *
 *     grammar/networking/networking.g4
 *
 * to import:
 *
 *     NetworkingPolicies
 *
 *
 * REQUIRED CHANGE 2
 * -----------------
 *
 * Add the networking policy adapter to the aggregate construct dispatch:
 *
 *     | networkingPolicy
 *
 *
 * REQUIRED CHANGE 3
 * -----------------
 *
 * Add the stable aggregate adapter:
 *
 *     networkingPolicy
 *         : networkingPolicyConstruct
 *         ;
 *
 *
 * REQUIRED CHANGE 4
 * -----------------
 *
 * Do NOT add policy syntax directly to:
 *
 *     networking.g4
 *
 * The policy syntax remains delegated to this file and ultimately to
 * grammar/policies/policy.g4.
 *
 *
 * REQUIRED CHANGE 5
 * -----------------
 *
 * Existing networking component grammars must NOT be rewritten merely to
 * consume this adapter.
 *
 * Their existing local policy wrappers remain valid.
 *
 * The semantic frontend should normalize:
 *
 *     network-local policy reference
 *          |
 *          v
 *     canonical PolicyReference
 *
 *
 * REQUIRED CHANGE 6
 * -----------------
 *
 * The canonical policy semantic model must remain the only semantic authority.
 *
 * Networking contributes:
 *
 *     subject/domain
 *     attachment location
 *     source span
 *
 * but not a second policy model.
 *
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The networking adapter should produce an AST representation equivalent to:
 *
 *     NetworkingPolicy
 *         declaration: Policy
 *
 * or:
 *
 *     NetworkingPolicyReference
 *         name: QualifiedName
 *
 * Source spans must be preserved.
 *
 * The AST MUST NOT lose:
 *
 *     policy identity
 *     policy member order
 *     policy source locations
 *     qualified names
 *     expressions
 *     attributes
 *     policy composition
 *
 *
 * ============================================================================
 * SEMANTIC NORMALIZATION
 * ============================================================================
 *
 * Networking semantic analysis should normalize:
 *
 *     networkingPolicyDeclaration
 *         ->
 *     canonical Policy
 *
 * and:
 *
 *     networkingPolicyReference
 *         ->
 *     canonical PolicyReference
 *
 * A networking policy reference may then be attached semantically to:
 *
 *     service
 *     request
 *     response
 *     protocol
 *     endpoint
 *     socket
 *     channel
 *     stream
 *     route
 *     discovery
 *
 * The exact attachment is determined by the owning networking AST node.
 *
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar emits NO IR.
 *
 * Policy semantics may influence:
 *
 *     canonical semantic representation
 *     classical IR
 *     quantum::ir
 *     distributed plans
 *     hardware plans
 *     scheduling
 *     routing
 *     resilience
 *
 * but the networking policy grammar MUST NOT directly emit:
 *
 *     machine instructions
 *     network packets
 *     socket handles
 *     device handles
 *     physical routes
 *     physical qubit assignments
 *     HDL signals
 *     scheduler commands
 *     QEC operations
 *
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * THIS FILE IS COMPLETE WHEN:
 *
 *     [x] It has one clear networking-policy ownership boundary.
 *
 *     [x] It does not duplicate universal policy syntax.
 *
 *     [x] It imports the canonical Policy grammar.
 *
 *     [x] It provides a stable networkingPolicyConstruct rule.
 *
 *     [x] It provides a stable networkingPolicyDeclaration adapter.
 *
 *     [x] It provides qualified-name policy references.
 *
 *     [x] It has no networking technology catalogue.
 *
 *     [x] It has no hardware limits.
 *
 *     [x] It has no network-size limits.
 *
 *     [x] It has no quantum limits.
 *
 *     [x] It has no embedded Rust.
 *
 *     [x] It requires no unsafe Rust.
 *
 *     [x] It creates no IR.
 *
 *     [x] It performs no policy evaluation.
 *
 *     [x] It performs no network discovery.
 *
 *     [x] It performs no routing.
 *
 *     [x] It performs no scheduling.
 *
 *     [x] It performs no resource allocation.
 *
 *     [ ] networking.g4 imports NetworkingPolicies.
 *
 *     [ ] networking.g4 dispatches networkingPolicy.
 *
 *     [ ] parser generation succeeds.
 *
 *     [ ] Rust generation succeeds.
 *
 *     [ ] positive tests pass.
 *
 *     [ ] negative tests pass.
 *
 *     [ ] boundary tests pass.
 *
 *     [ ] scalability tests pass.
 *
 *     [ ] determinism tests pass.
 *
 *     [ ] cross-domain tests pass.
 *
 *     [ ] compatibility tests pass.
 *
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * Networking policy is GOVERNING INTENT applied to networking.
 *
 * It is not a networking implementation.
 *
 * The dependency direction is:
 *
 *     canonical policy
 *          |
 *          v
 *     networking policy adapter
 *          |
 *          v
 *     networking semantic model
 *          |
 *          v
 *     requirements / capabilities / resources / effects / contracts
 *          |
 *          v
 *     target-independent planning
 *          |
 *          v
 *     routing / scheduling / resilience
 *          |
 *          v
 *     target realization
 *
 * This preserves:
 *
 *     Program_Once
 *          ->
 *     Compile_Once
 *          ->
 *     Run_Everywhere
 *          ->
 *     Run_Anywhere
 *          ->
 *     Forever
 *
 * ============================================================================
 */
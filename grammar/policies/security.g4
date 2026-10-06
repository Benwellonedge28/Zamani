/*
 * ============================================================================
 * ZAMANI UNIVERSAL COMPUTING LANGUAGE
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/policies/security.g4
 *
 * GRAMMAR
 * -------
 * PolicySecurity
 *
 * STATUS
 * ------
 * CANONICAL SECURITY-POLICY INTEGRATION BOUNDARY
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * ANTLR4 parser grammar
 * Rust 2021
 * Rust 1.97+
 * Safe Rust only
 * No unsafe Rust required
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file defines the policy-layer integration boundary between:
 *
 *     universal Zamani policy expressions
 *
 * and:
 *
 *     security-domain semantic analysis.
 *
 * This file intentionally does NOT create a second security policy language.
 *
 * The universal policy syntax is owned by:
 *
 *     grammar/expressions/policy.g4
 *
 * whose parser grammar is:
 *
 *     PolicyExpressions
 *
 * Its public entry point is:
 *
 *     policyExpression
 *
 * Security-specific policy declaration syntax is owned by:
 *
 *     grammar/security/policies.g4
 *
 * This file therefore acts as the bridge:
 *
 *     universal policy syntax
 *             |
 *             v
 *     security policy context
 *             |
 *             v
 *     security semantic analysis
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * Complete source pipeline:
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     ZamaniParser
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     structural validation
 *       |
 *       +-------------------------------+
 *       |                               |
 *       v                               v
 *     universal policy             security context
 *       |                               |
 *       +---------------+---------------+
 *                       |
 *                       v
 *               security semantic model
 *                       |
 *             +---------+---------+
 *             |         |         |
 *             v         v         v
 *          identity  authority  trust
 *             |         |         |
 *             +---------+---------+
 *                       |
 *                       v
 *              capability/effect/resource
 *                       |
 *                       v
 *                contract/policy
 *                       |
 *                       v
 *              canonical semantic model
 *                       |
 *             +---------+---------+
 *             |         |         |
 *             v         v         v
 *        classical   quantum::ir  HDL/hardware
 *             |         |         |
 *             +---------+---------+
 *                       |
 *                       v
 *                optimization
 *                       |
 *                 lowering/planning
 *                       |
 *               routing/scheduling
 *                       |
 *                 resilience
 *                       |
 *                    ZQN/HAL
 *                       |
 *                 target runtime
 *
 * This grammar participates only at the source parsing boundary.
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns ONLY the security-policy integration boundary:
 *
 *     securityPolicyConstruct
 *     securityPolicyReference
 *
 * These rules establish stable names that security-domain grammars and
 * semantic consumers can depend on without importing the complete universal
 * policy implementation directly.
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     lexer rules
 *     lexer tokens
 *     identifiers
 *     qualified names
 *     general expressions
 *     policy expressions
 *     policy directives
 *     policy bodies
 *     policy declarations
 *     security policy declarations
 *     permissions
 *     authorization
 *     authentication
 *     identities
 *     principals
 *     roles
 *     credentials
 *     trust evaluation
 *     capabilities
 *     resource requirements
 *     resource constraints
 *     effects
 *     contracts
 *     sandbox enforcement
 *     cryptographic algorithms
 *     key management
 *     privacy enforcement
 *     audit implementation
 *     provenance storage
 *     hardware discovery
 *     target selection
 *     device selection
 *     topology
 *     routing
 *     scheduling
 *     quantum operations
 *     physical qubits
 *     QEC
 *     ZQN
 *     HAL
 *     runtime enforcement
 *     policy evaluation algorithms
 *     policy conflict resolution
 *
 * Those responsibilities remain with their existing authoritative owners.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There is ONE universal policy-expression syntax.
 *
 * Its authority is:
 *
 *     grammar/expressions/policy.g4
 *
 * There is ONE security-specific policy declaration grammar.
 *
 * Its authority is:
 *
 *     grammar/security/policies.g4
 *
 * This file MUST NOT redefine either language.
 *
 * In particular, this file MUST NOT define another:
 *
 *     policy
 *     policyBody
 *     policyClause
 *     policyDirective
 *     policyDirectiveName
 *     policySelector
 *     policyCondition
 *     policyValue
 *
 * Doing so would create competing policy authorities.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 * Direct:
 *
 *     grammar/expressions/policy.g4
 *
 * Parser grammar:
 *
 *     PolicyExpressions
 *
 * Public rule consumed:
 *
 *     policyExpression
 *
 * Lexical vocabulary:
 *
 *     ZamaniLexer
 *
 * The imported policy grammar owns all policy lexical and structural details.
 *
 * ============================================================================
 * EXPORTS
 * ============================================================================
 *
 * Public parser rules:
 *
 *     securityPolicyConstruct
 *     securityPolicyReference
 *
 * These are semantic integration boundaries, not new policy syntax.
 *
 * ============================================================================
 * CONSUMED_BY
 * ============================================================================
 *
 * Potential consumers include:
 *
 *     grammar/security/security.g4
 *     grammar/security/sandbox.g4
 *     grammar/security/authorization.g4
 *     grammar/security/permissions.g4
 *     grammar/security/trust.g4
 *     grammar/security/privacy.g4
 *     grammar/security/secure-computation.g4
 *     grammar/security/zero-knowledge.g4
 *
 * and security-aware policy consumers under:
 *
 *     grammar/policies/
 *
 * AI-domain policy consumers may continue to use:
 *
 *     grammar/ai/policies.g4
 *
 * directly through PolicyExpressions.
 *
 * No consumer should recreate policy syntax merely to consume security
 * semantics.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar introduces NO mandatory security-specific AST type.
 *
 * The parse tree for:
 *
 *     securityPolicyConstruct
 *
 * contains the canonical:
 *
 *     policyExpression
 *
 * structure.
 *
 * The domain-neutral AST should therefore preserve:
 *
 *     policy identity
 *     selector
 *     clauses
 *     directive names
 *     directive arguments
 *     bindings
 *     conditions
 *     source spans
 *     source order
 *     lexical/source provenance
 *     enclosing security context
 *
 * The AST MUST NOT introduce:
 *
 *     physical device identifiers
 *     processor identifiers
 *     QPU identifiers
 *     physical qubit identifiers
 *     hardware topology
 *     scheduler state
 *     routing state
 *     calibration state
 *     backend-specific security structures
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing a security policy does NOT establish:
 *
 *     authorization
 *     authentication
 *     trust
 *     permission
 *     capability ownership
 *     resource availability
 *     policy validity
 *     policy satisfiability
 *     runtime enforcement
 *
 * Semantic analysis must resolve the policy in its security context.
 *
 * Conceptually:
 *
 *     securityPolicyConstruct
 *             |
 *             v
 *     PolicyExpression
 *             |
 *             v
 *     security semantic context
 *             |
 *             +--> identity resolution
 *             +--> principal resolution
 *             +--> permission analysis
 *             +--> capability analysis
 *             +--> trust analysis
 *             +--> resource analysis
 *             +--> effect analysis
 *             +--> contract analysis
 *             +--> provenance analysis
 *             +--> authorization analysis
 *             +--> policy conflict analysis
 *             |
 *             v
 *     security policy semantic model
 *
 * Parser acceptance MUST NEVER be treated as authorization.
 *
 * ============================================================================
 * SECURITY MODEL
 * ============================================================================
 *
 * Security policy expressions may semantically describe:
 *
 *     permission requirements
 *     prohibitions
 *     authorization requirements
 *     authentication requirements
 *     trust requirements
 *     capability requirements
 *     resource restrictions
 *     effect restrictions
 *     privacy requirements
 *     confidentiality requirements
 *     integrity requirements
 *     availability requirements
 *     isolation requirements
 *     sandbox requirements
 *     audit requirements
 *     provenance requirements
 *     secure-computation requirements
 *     adaptation restrictions
 *     reflection restrictions
 *     native-call restrictions
 *     foreign-call restrictions
 *     network restrictions
 *     data-access restrictions
 *     execution restrictions
 *
 * None of these are hard-coded as a closed list of policy directives here.
 *
 * The universal policy grammar deliberately represents policy directives as
 * extensible identifiers/qualified names.
 *
 * ============================================================================
 * OPEN-WORLD SECURITY
 * ============================================================================
 *
 * Security mechanisms are open-ended.
 *
 * The grammar MUST remain capable of representing future security concepts
 * without modification merely because a new semantic capability is invented.
 *
 * Examples of semantically possible namespaces include:
 *
 *     security.authorization
 *     security.authentication
 *     security.confidentiality
 *     security.integrity
 *     security.isolation
 *     security.trust
 *     security.audit
 *     security.provenance
 *     security.privacy
 *     security.secure_execution
 *     security.future_mechanism
 *
 * These names remain semantic identifiers.
 *
 * This grammar does NOT enumerate them.
 *
 * Therefore adding a new security mechanism normally requires:
 *
 *     semantic registration
 *     capability registration
 *     policy specification
 *     implementation
 *     tests
 *
 * rather than modification of this parser boundary.
 *
 * ============================================================================
 * POLICY EXPRESSION BOUNDARY
 * ============================================================================
 *
 * Canonical universal policy grammar:
 *
 *     grammar/expressions/policy.g4
 *
 * Grammar:
 *
 *     PolicyExpressions
 *
 * Entry point:
 *
 *     policyExpression
 *
 * Security integration:
 *
 *     policyExpression
 *          |
 *          v
 *     securityPolicyConstruct
 *          |
 *          v
 *     security semantic model
 *
 * This direction is intentional.
 *
 * The universal policy language does not depend on security.
 *
 * Security consumes the universal policy language.
 *
 * ============================================================================
 * SECURITY DECLARATION BOUNDARY
 * ============================================================================
 *
 * The repository already contains:
 *
 *     grammar/security/policies.g4
 *
 * That file owns source-level security policy declarations.
 *
 * It may define structures such as:
 *
 *     policy declaration
 *     policy header
 *     policy rule
 *     policy condition
 *     policy subject
 *     policy resource
 *     policy action
 *     authorization effect
 *     policy requirement
 *     policy obligation
 *     policy default
 *     policy priority
 *     policy duration
 *     policy import
 *
 * This file MUST NOT reproduce those rules.
 *
 * Instead:
 *
 *     grammar/security/policies.g4
 *             |
 *             v
 *     security policy declaration
 *             |
 *             v
 *     security semantic model
 *
 * while:
 *
 *     grammar/policies/security.g4
 *             |
 *             v
 *     universal policy expression
 *             |
 *             v
 *     security semantic context
 *
 * These are complementary boundaries, not competing grammars.
 *
 * ============================================================================
 * AUTHORIZATION BOUNDARY
 * ============================================================================
 *
 * Authorization remains owned by:
 *
 *     grammar/security/authorization.g4
 *
 * Permissions remain owned by:
 *
 *     grammar/security/permissions.g4
 *
 * This file does not define:
 *
 *     allow
 *     deny
 *     grant
 *     revoke
 *
 * as security-policy grammar rules.
 *
 * Such directive names remain part of the universal policy-expression
 * mechanism and are interpreted by the security semantic layer where
 * appropriate.
 *
 * ============================================================================
 * IDENTITY BOUNDARY
 * ============================================================================
 *
 * Identity syntax remains owned by the security identity subsystem.
 *
 * This file does not define:
 *
 *     identity
 *     principal
 *     subject
 *     role
 *     credential
 *
 * as new security-policy syntax.
 *
 * A policy may semantically reference identity constructs through the generic
 * policy expression/value mechanism.
 *
 * ============================================================================
 * CAPABILITY BOUNDARY
 * ============================================================================
 *
 * Generic capabilities remain owned by the resource/capability subsystem.
 *
 * Security capabilities remain owned by:
 *
 *     grammar/security/capabilities.g4
 *
 * A security policy may semantically require or prohibit a capability.
 *
 * The parser does not determine whether that capability exists or is
 * available.
 *
 * Example semantic intent:
 *
 *     require capability("security.isolated_execution")
 *
 * The grammar does not need to know whether the capability is provided by:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     simulator
 *     operating system
 *     runtime
 *     distributed environment
 *     future computational substrate
 *
 * ============================================================================
 * RESOURCE BOUNDARY
 * ============================================================================
 *
 * Security policy may constrain resources through the universal policy model.
 *
 * Resource ownership remains under:
 *
 *     grammar/resources/
 *
 * Security MUST NOT define universal resource capacities.
 *
 * The grammar MUST NOT impose limits on:
 *
 *     memory
 *     storage
 *     processors
 *     threads
 *     nodes
 *     devices
 *     channels
 *     accelerators
 *     QPUs
 *     qubits
 *     network endpoints
 *     security domains
 *     principals
 *     policies
 *     rules
 *
 * Resource feasibility belongs downstream.
 *
 * ============================================================================
 * EFFECT BOUNDARY
 * ============================================================================
 *
 * Security policies may semantically govern effects such as:
 *
 *     io
 *     network
 *     native
 *     foreign
 *     mutation
 *     randomness
 *     distributed
 *     measurement
 *     quantum
 *     learning
 *     adaptation
 *     reflection
 *     code_generation
 *     simulation
 *
 * Effect ownership remains in:
 *
 *     grammar/effects/
 *
 * This file does not redefine the effect grammar.
 *
 * ============================================================================
 * SANDBOX BOUNDARY
 * ============================================================================
 *
 * Sandbox syntax remains owned by:
 *
 *     grammar/security/sandbox.g4
 *
 * and its statement-level integration.
 *
 * Security policy may semantically govern sandbox configuration, but this
 * file does not redefine sandbox statements or sandbox directives.
 *
 * This prevents:
 *
 *     policy sandbox syntax
 *
 * from becoming a second:
 *
 *     sandbox language.
 *
 * ============================================================================
 * TRUST BOUNDARY
 * ============================================================================
 *
 * Trust semantics remain owned by:
 *
 *     grammar/security/trust.g4
 *
 * The parser does not establish trust.
 *
 * A source declaration such as a trust requirement remains only an intent
 * until semantic analysis verifies:
 *
 *     identity
 *     authority
 *     trust relationship
 *     evidence
 *     provenance
 *     applicable policy
 *     runtime enforcement capability
 *
 * ============================================================================
 * PROVENANCE BOUNDARY
 * ============================================================================
 *
 * Security-policy semantics should preserve provenance sufficient to answer:
 *
 *     which source policy produced this requirement?
 *     which directive produced this restriction?
 *     which semantic transformation changed it?
 *     which evidence supported the decision?
 *     which authority approved it?
 *     which policy version was active?
 *
 * Provenance storage and representation remain owned by the provenance
 * subsystem.
 *
 * This grammar introduces no security-specific provenance format.
 *
 * ============================================================================
 * AUDIT BOUNDARY
 * ============================================================================
 *
 * Audit intent may be represented through the universal policy-expression
 * mechanism.
 *
 * Actual audit collection, storage, integrity, retention, transport, and
 * verification are runtime/tooling/security concerns.
 *
 * Parser acceptance MUST NOT imply that an audit record has been created.
 *
 * ============================================================================
 * CRYPTOGRAPHY BOUNDARY
 * ============================================================================
 *
 * Cryptographic syntax and semantics remain owned by the cryptography
 * subsystem.
 *
 * This grammar MUST NOT enumerate:
 *
 *     algorithms
 *     key sizes
 *     curves
 *     providers
 *     ciphers
 *     hashes
 *     signature schemes
 *     hardware security modules
 *
 * Security policy may express cryptographic requirements through generic
 * policy names, values, capabilities, requirements, and constraints.
 *
 * ============================================================================
 * PRIVACY BOUNDARY
 * ============================================================================
 *
 * Privacy syntax remains owned by:
 *
 *     grammar/security/privacy.g4
 *
 * This file provides only the policy integration boundary.
 *
 * Privacy semantics may consume security policy expressions without requiring
 * this grammar to know every future privacy model.
 *
 * ============================================================================
 * SECURE COMPUTATION BOUNDARY
 * ============================================================================
 *
 * Secure-computation semantics remain owned by the existing security
 * subsystem.
 *
 * This policy boundary may express requirements for secure computation but
 * does not implement:
 *
 *     enclaves
 *     isolation
 *     secure memory
 *     trusted execution
 *     confidential computing
 *     cryptographic protocols
 *     attestation
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Security policies may govern quantum computation.
 *
 * Examples of semantic requirements include:
 *
 *     quantum execution authorization
 *     measurement restrictions
 *     protected quantum data
 *     trusted quantum execution
 *     secure classical/quantum communication
 *     QPU access authorization
 *     simulator fallback
 *     provenance requirements
 *     audit requirements
 *
 * However, this grammar MUST NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     gate catalogues
 *     topology
 *     calibration
 *     pulse data
 *     QEC codes
 *     routing
 *     scheduling
 *
 * The canonical quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * Security semantics may influence the path toward quantum::ir and its
 * realization, but this grammar does not create a security-specific quantum
 * IR.
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Security policies may apply to:
 *
 *     HDL designs
 *     hardware intent
 *     secure memory
 *     secure channels
 *     hardware isolation
 *     trusted execution
 *     device capabilities
 *     accelerator access
 *     synthesis restrictions
 *
 * HDL remains owned by:
 *
 *     grammar/hdl/
 *
 * Hardware realization remains owned by:
 *
 *     grammar/hardware/
 *
 * This file does not encode:
 *
 *     bus widths
 *     register widths
 *     device counts
 *     memory capacities
 *     topology limits
 *     clock limits
 *     pipeline limits
 *
 * ============================================================================
 * DISTRIBUTED BOUNDARY
 * ============================================================================
 *
 * Security policies may govern arbitrarily many:
 *
 *     processes
 *     tasks
 *     actors
 *     services
 *     nodes
 *     regions
 *     channels
 *     endpoints
 *     participants
 *
 * No language-level maximum is defined.
 *
 * Distribution semantics remain owned by:
 *
 *     grammar/distributed/
 *     grammar/networking/
 *     grammar/concurrency/
 *
 * ============================================================================
 * AI / LEARNING / ADAPTATION BOUNDARY
 * ============================================================================
 *
 * Security policies may govern:
 *
 *     inference
 *     reasoning
 *     learning
 *     adaptation
 *     model access
 *     knowledge access
 *     agent authority
 *     provenance
 *     explainability
 *     external resources
 *     native execution
 *     foreign execution
 *
 * AI syntax remains owned by:
 *
 *     grammar/ai/
 *
 * The AI policy boundary remains:
 *
 *     grammar/ai/policies.g4
 *
 * This file must not duplicate AI policy syntax.
 *
 * ============================================================================
 * METAPROGRAMMING BOUNDARY
 * ============================================================================
 *
 * Security policies may constrain:
 *
 *     reflection
 *     introspection
 *     code generation
 *     compile-time execution
 *     dynamic loading
 *     metaprogramming
 *
 * Metaprogramming syntax remains owned by:
 *
 *     grammar/metaprogramming/
 *
 * This file only provides policy-context integration.
 *
 * ============================================================================
 * FFI / ABI BOUNDARY
 * ============================================================================
 *
 * Security policies may constrain:
 *
 *     FFI
 *     ABI transitions
 *     native calls
 *     foreign calls
 *     external libraries
 *     external data
 *
 * Interoperability remains owned by:
 *
 *     grammar/interoperability/
 *
 * The policy grammar does not define ABI syntax.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Security policy syntax is target-independent.
 *
 * The same source-level security policy may apply to:
 *
 *     tiny embedded systems
 *     single processors
 *     multicore processors
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     QPUs
 *     simulators
 *     HPC systems
 *     clusters
 *     distributed systems
 *     cloud environments
 *     heterogeneous systems
 *     future computational substrates
 *
 * The grammar MUST NOT force a source program to select a physical target
 * merely because it expresses a security policy.
 *
 * Target feasibility and enforcement are downstream concerns.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar contains NO fixed limits on:
 *
 *     policies
 *     security policy contexts
 *     clauses
 *     directives
 *     arguments
 *     namespaces
 *     identities
 *     principals
 *     roles
 *     resources
 *     capabilities
 *     effects
 *     devices
 *     processors
 *     nodes
 *     channels
 *     QPUs
 *     qubits
 *     memory
 *     storage
 *
 * Repetition and extensibility are inherited from PolicyExpressions.
 *
 * Practical parser limits are implementation/resource limits, not language
 * semantics.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains:
 *
 *     NO maximum policy count.
 *     NO maximum security-domain count.
 *     NO maximum identity count.
 *     NO maximum principal count.
 *     NO maximum role count.
 *     NO maximum capability count.
 *     NO maximum resource count.
 *     NO maximum node count.
 *     NO maximum device count.
 *     NO maximum processor count.
 *     NO maximum accelerator count.
 *     NO maximum QPU count.
 *     NO maximum qubit count.
 *     NO maximum memory capacity.
 *     NO maximum storage capacity.
 *     NO maximum namespace depth.
 *     NO vendor enumeration.
 *     NO cryptographic algorithm enumeration.
 *     NO hardware enumeration.
 *     NO fixed authorization action enumeration.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing must be deterministic.
 *
 * The parse result depends only on:
 *
 *     source text
 *     grammar version
 *     lexer vocabulary
 *     parser configuration
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware
 *     current time
 *     randomness
 *     filesystem state
 *     network state
 *     environment variables
 *     runtime state
 *     scheduler state
 *     target availability
 *
 * Security-policy evaluation is a downstream semantic operation and may have
 * separate determinism/reproducibility requirements.
 *
 * ============================================================================
 * SAFETY CONTRACT
 * ============================================================================
 *
 * This grammar:
 *
 *     contains no embedded Rust actions;
 *     contains no executable semantic predicates;
 *     performs no filesystem access;
 *     performs no network access;
 *     performs no hardware discovery;
 *     performs no authorization;
 *     performs no credential handling;
 *     performs no secret handling;
 *     performs no runtime execution.
 *
 * Generated Rust integration must remain:
 *
 *     Rust 2021
 *     Rust 1.97+
 *     safe Rust
 *
 * No unsafe implementation is required by this grammar.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser-level diagnostics belong to malformed universal policy syntax.
 *
 * This grammar should therefore allow the canonical policy grammar to report
 * errors for:
 *
 *     malformed policy body
 *     malformed directive
 *     malformed directive arguments
 *     malformed selector
 *     malformed condition
 *     malformed binding
 *
 * Semantic security diagnostics belong downstream.
 *
 * Examples:
 *
 *     unknown security policy directive
 *     unauthorized principal
 *     unavailable capability
 *     unsatisfied security requirement
 *     conflicting security policies
 *     invalid trust relationship
 *     insufficient evidence
 *     invalid provenance
 *     prohibited effect
 *     unavailable enforcement mechanism
 *     incompatible target
 *
 * These MUST NOT be encoded as parser alternatives.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This boundary is intentionally stable.
 *
 * New security policy directives should normally NOT require changes to this
 * file.
 *
 * A new semantic directive can be represented by the existing universal
 * policy expression grammar through:
 *
 *     policyDirectiveName
 *     policyDirectiveArguments
 *     policyCondition
 *     policySelector
 *
 * Compatibility aliases belong under:
 *
 *     grammar/compatibility/
 *
 * This file must not duplicate historical lexer aliases.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file creates NO IR.
 *
 * Security policy semantics may influence:
 *
 *     canonical semantic policy model
 *     classical IR metadata
 *     quantum::ir metadata/constraints where applicable
 *     HDL/hardware semantic representation
 *     distributed execution plans
 *     networking plans
 *     deployment plans
 *     execution plans
 *
 * But this grammar MUST NOT define:
 *
 *     SecurityIR
 *     AuthorizationIR
 *     TrustIR
 *     CryptoIR
 *     QuantumSecurityIR
 *     HardwareSecurityIR
 *
 * merely because security semantics affect those domains.
 *
 * ============================================================================
 * BACKEND CONTRACT
 * ============================================================================
 *
 * Backend realization may enforce security semantics using available:
 *
 *     capabilities
 *     resources
 *     operating-system mechanisms
 *     runtime mechanisms
 *     hardware mechanisms
 *     cryptographic mechanisms
 *     distributed mechanisms
 *     quantum execution mechanisms
 *     sandbox mechanisms
 *     external security providers
 *
 * The grammar remains unchanged when a new backend or enforcement mechanism
 * is introduced.
 *
 * ============================================================================
 * PUBLIC RULE 1
 * ============================================================================
 *
 * Security policy construct.
 *
 * This is the canonical parser boundary for using universal policy syntax in
 * a security semantic context.
 *
 * It is deliberately a direct delegation.
 *
 * There is no second policy syntax here.
 */

securityPolicyConstruct
    : policyExpression
    ;


/*
 * ============================================================================
 * PUBLIC RULE 2
 * ============================================================================
 *
 * Security policy reference.
 *
 * This stable named boundary is useful for security-domain grammars that need
 * to state that a construct accepts a policy without depending directly on
 * the full policy grammar's internal rule structure.
 *
 * It remains a direct delegation and therefore cannot diverge from the
 * universal policy language.
 */

securityPolicyReference
    : securityPolicyConstruct
    ;


/*
 * ============================================================================
 * SEMANTIC CONTEXT CONTRACT
 * ============================================================================
 *
 * The enclosing security semantic consumer determines what the policy means.
 *
 * Possible contexts include:
 *
 *     authorization
 *     authentication
 *     identity
 *     trust
 *     permission
 *     capability
 *     sandbox
 *     privacy
 *     secure computation
 *     audit
 *     provenance
 *     resource access
 *     network access
 *     native execution
 *     foreign execution
 *     reflection
 *     adaptation
 *     quantum execution
 *     hardware access
 *     distributed execution
 *
 * This grammar deliberately does not encode these contexts as a closed enum.
 *
 * ============================================================================
 * CROSS-DOMAIN CONTRACT
 * ============================================================================
 *
 * A security policy must remain semantically attachable to:
 *
 *     declarations
 *     expressions
 *     statements
 *     functions
 *     modules
 *     classical operations
 *     tensor operations
 *     quantum operations
 *     hybrid operations
 *     HDL intent
 *     hardware intent
 *     actors
 *     tasks
 *     distributed operations
 *     network operations
 *     AI operations
 *     learning
 *     adaptation
 *     FFI
 *     ABI
 *     simulation
 *     metaprogramming
 *
 * without changing this grammar.
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * This file must NOT accept malformed input merely because it is intended for
 * security.
 *
 * Invalid forms remain invalid according to PolicyExpressions.
 *
 * Examples to test through the complete parser:
 *
 *     policy
 *     policy {
 *     policy {
 *         require(
 *     }
 *
 *     policy security {
 *         ...
 *     }
 *
 * where the embedded policy expression itself is malformed.
 *
 * Security semantic failures such as:
 *
 *     unknown principal
 *     unauthorized action
 *     unavailable capability
 *     invalid trust
 *
 * are NOT parser-negative tests.
 *
 * They are semantic-negative tests.
 *
 * ============================================================================
 * POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * At minimum, security consumers must verify that this boundary can consume
 * canonical policy expressions such as:
 *
 *     policy security {
 *         require capability("security.authorization");
 *     }
 *
 *     policy security {
 *         forbid capability("native.execute");
 *     }
 *
 *     policy security {
 *         require capability("secure.execution");
 *         prefer execution.deterministic;
 *     }
 *
 *     policy security {
 *         require security.confidentiality;
 *         require security.integrity;
 *         forbid effect("network");
 *     }
 *
 *     policy security {
 *         require security.trusted_execution;
 *         require provenance.required;
 *     }
 *
 * The exact semantic validity of these directives belongs downstream.
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Test this boundary in combination with:
 *
 *     declarations
 *     functions
 *     modules
 *     contracts
 *     requirements
 *     constraints
 *     capabilities
 *     effects
 *     resources
 *     sandbox
 *     authorization
 *     permissions
 *     trust
 *     privacy
 *     provenance
 *     classical computation
 *     quantum computation
 *     hybrid computation
 *     HDL
 *     hardware intent
 *     distributed computation
 *     networking
 *     AI
 *     learning
 *     adaptation
 *     FFI
 *     ABI
 *     simulation
 *     metaprogramming
 *
 * The policy grammar must remain unchanged across all such contexts.
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Test policy expressions containing:
 *
 *     many clauses
 *     many directives
 *     many arguments
 *     deeply qualified names
 *     nested policy values
 *     large requirement expressions
 *     large capability expressions
 *     large resource expressions
 *     large security policy sets
 *
 * Test sizes are test parameters.
 *
 * They MUST NOT be interpreted as language limits.
 *
 * No test may assert a universal maximum such as:
 *
 *     maximum policies
 *     maximum rules
 *     maximum principals
 *     maximum resources
 *     maximum devices
 *     maximum nodes
 *     maximum QPUs
 *
 * ============================================================================
 * DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * Given identical:
 *
 *     source
 *     lexer version
 *     grammar version
 *     parser configuration
 *
 * parsing must produce equivalent structure and source spans.
 *
 * Security evaluation may have a separate determinism contract.
 *
 * ============================================================================
 * ROUND-TRIP CONTRACT
 * ============================================================================
 *
 * Where the repository provides formatting/serialization:
 *
 *     source
 *       ->
 *     lexer
 *       ->
 *     parser
 *       ->
 *     AST
 *       ->
 *     formatter
 *       ->
 *     parser
 *
 * must preserve policy semantics.
 *
 * This file must not introduce a structure that causes policy directives,
 * arguments, selectors, conditions, or ordering information to be silently
 * discarded.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * REQUIRED EXISTING INTEGRATION
 * -----------------------------
 *
 * 1. Universal policy expressions:
 *
 *     grammar/expressions/policy.g4
 *
 *     PolicyExpressions
 *
 *     policyExpression
 *
 * This file already owns the actual policy syntax.
 *
 *
 * 2. Security policy declarations:
 *
 *     grammar/security/policies.g4
 *
 * This remains the security declaration authority.
 *
 *
 * 3. Security composition:
 *
 *     grammar/security/security.g4
 *
 * The security parser composition may expose this boundary where security
 * semantic contexts consume universal policies.
 *
 *
 * 4. Sandbox:
 *
 *     grammar/security/sandbox.g4
 *
 * Sandbox syntax remains independently owned.
 *
 *
 * 5. Authorization:
 *
 *     grammar/security/authorization.g4
 *
 * Authorization semantics remain independently owned.
 *
 *
 * 6. Permissions:
 *
 *     grammar/security/permissions.g4
 *
 * Permission syntax remains independently owned.
 *
 *
 * 7. Trust:
 *
 *     grammar/security/trust.g4
 *
 * Trust semantics remain independently owned.
 *
 *
 * 8. AI policy integration:
 *
 *     grammar/ai/policies.g4
 *
 * AI policy syntax continues to consume PolicyExpressions rather than this
 * security-specific adapter.
 *
 *
 * 9. Root parser:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * The root parser composition should include the security composition boundary
 * exactly once.
 *
 *
 * 10. Lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * No new lexer token is required by this file.
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * This grammar imports exactly the canonical universal policy-expression
 * grammar.
 *
 * It deliberately does NOT import:
 *
 *     grammar/security/policies.g4
 *     grammar/security/authorization.g4
 *     grammar/security/permissions.g4
 *     grammar/security/trust.g4
 *     grammar/security/sandbox.g4
 *
 * merely to expose this boundary.
 *
 * This avoids:
 *
 *     circular imports
 *     duplicate policy syntax
 *     parser ambiguity
 *     competing security policy authorities
 *
 * Security-specific grammars consume this boundary at their composition
 * points rather than this file importing every security subsystem.
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 * Correct:
 *
 *     PolicyExpressions
 *            |
 *            v
 *     PolicySecurity
 *            |
 *            v
 *     Security semantic consumers
 *
 * Incorrect:
 *
 *     Security
 *          |
 *          v
 *     PolicySecurity
 *          |
 *          v
 *     Security
 *
 * The latter creates circular ownership.
 *
 * ============================================================================
 * AST / SEMANTIC OWNERSHIP
 * ============================================================================
 *
 * AST_OWNER:
 *
 *     src/ast/
 *
 * The AST must preserve the universal policy structure.
 *
 * SEMANTIC_OWNER:
 *
 *     security semantic subsystem
 *
 * The semantic layer resolves:
 *
 *     directive identity
 *     policy applicability
 *     identity
 *     authority
 *     capability
 *     resource
 *     effect
 *     trust
 *     authorization
 *     provenance
 *     contracts
 *
 * ============================================================================
 * RESOURCE / CAPABILITY / EFFECT INTEGRATION
 * ============================================================================
 *
 * Security policy analysis must use the existing universal semantic systems:
 *
 *     policy
 *       |
 *       +--> requirements
 *       +--> constraints
 *       +--> capabilities
 *       +--> resources
 *       +--> effects
 *       +--> contracts
 *       +--> provenance
 *
 * It must not create security-specific copies of those universal models.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [x] It exists as a policy/security integration boundary.
 *
 * [x] It does not create a second security policy language.
 *
 * [x] It consumes PolicyExpressions.
 *
 * [x] It exposes a stable securityPolicyConstruct rule.
 *
 * [x] It exposes a stable securityPolicyReference rule.
 *
 * [x] It defines no lexer rules.
 *
 * [x] It defines no security-specific lexer tokens.
 *
 * [x] It defines no universal policy syntax.
 *
 * [x] It defines no authorization implementation.
 *
 * [x] It defines no identity implementation.
 *
 * [x] It defines no trust implementation.
 *
 * [x] It defines no capability implementation.
 *
 * [x] It defines no resource implementation.
 *
 * [x] It defines no runtime enforcement.
 *
 * [x] It defines no hardware selection.
 *
 * [x] It defines no quantum hardware assumptions.
 *
 * [x] It defines no HDL assumptions.
 *
 * [x] It defines no fixed capacity.
 *
 * [x] It defines no vendor-specific mechanism.
 *
 * [x] It requires no unsafe Rust.
 *
 * [x] It remains valid for Rust 1.97+ generated-parser integration.
 *
 * [ ] ANTLR generation succeeds in the repository's actual build.
 *
 * [ ] ZamaniParser imports/exposes the rule through the canonical security
 *     composition boundary where required.
 *
 * [ ] Security semantic tests consume the rule.
 *
 * [ ] Positive tests pass.
 *
 * [ ] Negative parser tests pass.
 *
 * [ ] Semantic-negative tests pass downstream.
 *
 * [ ] Cross-domain tests pass.
 *
 * [ ] Scalability tests pass within available test resources.
 *
 * [ ] Determinism tests pass.
 *
 * [ ] Round-trip tests pass where formatting is available.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * This file is deliberately small at the parser-rule level.
 *
 * That is a feature, not a missing capability.
 *
 * Security policy capability belongs to the universal policy language and
 * security semantic system.
 *
 * This file supplies a stable integration boundary so security-aware grammar
 * components can consume that policy model without creating a competing
 * language.
 *
 * Therefore:
 *
 *     universal policy syntax
 *             |
 *             v
 *     security policy context
 *             |
 *             v
 *     identity / authorization / trust / capability / resource / effect
 *             |
 *             v
 *     security semantic model
 *             |
 *             v
 *     canonical semantic model
 *             |
 *             +--> classical
 *             +--> quantum::ir
 *             +--> HDL/hardware
 *             +--> distributed
 *             +--> networking
 *             +--> AI
 *             +--> future domains
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
 * without turning security into a closed, target-specific language.
 * ============================================================================
 */

parser grammar PolicySecurity;

options {
    tokenVocab = ZamaniLexer;
}

import PolicyExpressions;


/*
 * ============================================================================
 * PUBLIC SECURITY-POLICY CONSTRUCT
 * ============================================================================
 *
 * Canonical universal policy syntax is delegated directly to
 * PolicyExpressions.
 *
 * No security-specific syntax is duplicated here.
 */

securityPolicyConstruct
    : policyExpression
    ;


/*
 * ============================================================================
 * PUBLIC SECURITY-POLICY REFERENCE
 * ============================================================================
 *
 * Stable adapter boundary for security grammars that need to consume a
 * policy-valued construct.
 */

securityPolicyReference
    : securityPolicyConstruct
    ;
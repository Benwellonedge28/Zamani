/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/security/secure-computation.g4
 *
 * ROLE
 * ----
 * Canonical parser-level grammar for secure-computation intent.
 *
 * STATUS
 * ------
 * PRODUCTION SECURITY-DOMAIN GRAMMAR
 *
 * ============================================================================
 * LANGUAGE / IMPLEMENTATION BASELINE
 * ============================================================================
 *
 * Language:
 *     Zamani
 *
 * Grammar:
 *     ANTLR4 parser grammar
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     This grammar contains no Rust actions.
 *     No unsafe Rust is required.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar describes PORTABLE SECURE-COMPUTATION INTENT.
 *
 * It covers the source-level expression of:
 *
 *     - secure-computation domains;
 *     - secure-computation protocols;
 *     - computation sessions;
 *     - abstract participants;
 *     - public inputs;
 *     - private inputs;
 *     - shared inputs;
 *     - outputs;
 *     - secret sharing intent;
 *     - threshold requirements;
 *     - secure operations;
 *     - secure calls;
 *     - reveal/declassification intent;
 *     - privacy requirements;
 *     - integrity requirements;
 *     - isolation requirements;
 *     - attestation requirements;
 *     - capability requirements;
 *     - resource requirements;
 *     - constraints;
 *     - preferences;
 *     - protocol/provider/implementation references;
 *     - metadata.
 *
 * It does NOT implement secure computation.
 *
 * ============================================================================
 * NON-RESPONSIBILITIES
 * ============================================================================
 *
 * This grammar MUST NOT:
 *
 *     - execute MPC;
 *     - execute homomorphic computation;
 *     - execute secret sharing;
 *     - execute oblivious computation;
 *     - execute confidential computation;
 *     - perform secure aggregation;
 *     - evaluate cryptographic protocols;
 *     - generate keys;
 *     - store secrets;
 *     - access credentials;
 *     - perform attestation;
 *     - discover trusted hardware;
 *     - select CPUs;
 *     - select GPUs;
 *     - select FPGAs;
 *     - select ASICs;
 *     - select QPUs;
 *     - select nodes;
 *     - select providers;
 *     - perform scheduling;
 *     - perform routing;
 *     - perform QEC;
 *     - perform ZQN;
 *     - create a second IR;
 *     - create a second security IR.
 *
 * Those responsibilities belong downstream.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical ZamaniLexer
 *          |
 *          v
 *     canonical ZamaniParser
 *          |
 *          v
 *     Security composition
 *          |
 *          v
 *     THIS GRAMMAR
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic security analysis
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          +-----------------------+
 *          |                       |
 *          v                       v
 *     classical semantics     quantum::ir metadata
 *          |                       |
 *          +-----------+-----------+
 *                      |
 *                      v
 *                 optimization
 *                      |
 *             +--------+--------+
 *             |        |        |
 *             v        v        v
 *          routing  scheduling resilience
 *                                  |
 *                                  v
 *                                 ZQN
 *                                  |
 *                                  v
 *                                 HAL
 *                                  |
 *                                  v
 *                               target
 *
 * ============================================================================
 * OPEN-WORLD PRINCIPLE
 * ============================================================================
 *
 * Secure-computation mechanisms are semantic names.
 *
 * This grammar MUST NOT enumerate:
 *
 *     MPC
 *     SMPC
 *     SPDZ
 *     GMW
 *     BGW
 *     Yao
 *     garbled_circuit
 *     secret_sharing_scheme
 *     FHE
 *     BFV
 *     BGV
 *     CKKS
 *     TFHE
 *     TEE
 *     SGX
 *     SEV
 *     TrustZone
 *     TPM
 *     HSM
 *
 * as a finite language catalogue.
 *
 * These remain qualified semantic names.
 *
 * Examples:
 *
 *     secure::computation::protocol::custom
 *     secure::computation::protocol::future
 *     secure::computation::sharing::scheme
 *     secure::computation::implementation::provider
 *
 * Semantic analysis determines whether the referenced mechanism exists and
 * whether it satisfies the program's requirements.
 *
 * ============================================================================
 * SCALABILITY / POCO-REAF
 * ============================================================================
 *
 * This grammar imposes no language-level maximum on:
 *
 *     participants
 *     parties
 *     inputs
 *     outputs
 *     secrets
 *     shares
 *     operations
 *     sessions
 *     protocols
 *     rounds
 *     thresholds
 *     resources
 *     devices
 *     nodes
 *     processors
 *     memory
 *     storage
 *     quantum resources
 *     classical resources
 *     secure-computation domains
 *
 * The following MUST NOT appear as language limits:
 *
 *     MAX_PARTIES
 *     MAX_PARTICIPANTS
 *     MAX_SECRETS
 *     MAX_SHARES
 *     MAX_ROUNDS
 *     MAX_PROTOCOLS
 *     MAX_SESSIONS
 *     MAX_OPERATIONS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *
 * Any actual limit is a property of:
 *
 *     program semantics;
 *     compiler resources;
 *     target capabilities;
 *     resource availability;
 *     deployment policy;
 *     runtime environment.
 *
 * It is not a grammar limit.
 *
 * ============================================================================
 * REQUIREMENT / CONSTRAINT / PREFERENCE
 * ============================================================================
 *
 * These are deliberately separate semantic categories.
 *
 * REQUIREMENT:
 *     mandatory property that must be preserved/satisfied.
 *
 * CONSTRAINT:
 *     restriction on valid realizations.
 *
 * PREFERENCE:
 *     desired property which may be traded off when realization requires it.
 *
 * The parser preserves this distinction.
 *
 * Semantic analysis determines satisfiability.
 *
 * ============================================================================
 * SECRET-MATERIAL BOUNDARY
 * ============================================================================
 *
 * This grammar does not introduce secret literals.
 *
 * Source may reference a secret object:
 *
 *     private_input secret::data
 *
 * but actual secret material belongs to the secret/key-management subsystem.
 *
 * This grammar MUST NOT introduce:
 *
 *     private_key_literal
 *     secret_literal
 *     password_literal
 *     credential_literal
 *     token_literal
 *
 * ============================================================================
 * CROSS-DOMAIN PRINCIPLE
 * ============================================================================
 *
 * Secure computation is not a separate programming language.
 *
 * It composes with:
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
 *     interoperability
 *
 * Secure-computation syntax describes security intent around those domains.
 *
 * ============================================================================
 */

parser grammar SecureComputation;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * CANONICAL IMPORT CONTRACT
 * ============================================================================
 *
 * Reuse canonical language syntax.
 *
 * Do NOT recreate:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     typeExpression
 *
 * here.
 *
 * These are supplied by the canonical parser composition hierarchy.
 *
 * ============================================================================
 */

import
    Core,
    Types,
    Expressions,
    Effects;


/*
 * ============================================================================
 * 1. STANDALONE ENTRY POINT
 * ============================================================================
 *
 * This is used for focused grammar validation and secure-computation
 * conformance tests.
 *
 * The complete Zamani compiler normally reaches this grammar through:
 *
 *     Zamani.g4
 *       ->
 *     ZamaniParser.g4
 *       ->
 *     Security
 *       ->
 *     SecureComputation
 *
 * ============================================================================
 */

secureComputationFile
    : secureComputationEntry* EOF
    ;


secureComputationEntry
    : secureComputationDeclaration
    | secureComputationOperation
    | secureComputationRequirement
    | secureComputationConstraint
    | secureComputationPreference
    ;


/*
 * ============================================================================
 * 2. SECURE-COMPUTATION DECLARATION
 * ============================================================================
 *
 * `secure` is the only dedicated lexical marker introduced for this domain.
 *
 * The remainder remains open-world.
 *
 * Examples:
 *
 *     secure computation::protocol::example { ... }
 *
 *     secure computation::session::session_name { ... }
 *
 *     secure computation::domain::analytics { ... }
 *
 * The semantic layer determines the declared category.
 *
 * ============================================================================
 */

secureComputationDeclaration
    : attributes*
      visibility?
      SECURE
      qualifiedName
      genericParameters?
      secureComputationBody?
      SEMI?
    ;


secureComputationBody
    : LBRACE
      secureComputationMember*
      RBRACE
    ;


secureComputationMember
    : secureComputationField
    | secureComputationParty
    | secureComputationInput
    | secureComputationOutput
    | secureComputationOperation
    | secureComputationRequirement
    | secureComputationConstraint
    | secureComputationPreference
    | secureComputationProperty
    | secureComputationCapability
    | secureComputationMetadata
    ;


/*
 * ============================================================================
 * 3. GENERIC DECLARATION FIELDS
 * ============================================================================
 *
 * Field names remain identifiers/qualified names.
 *
 * This prevents the grammar from becoming a closed dictionary.
 *
 * Examples:
 *
 *     protocol: secure::computation::protocol::future;
 *     sharing: secure::computation::sharing::scheme;
 *     implementation: vendor::implementation;
 *     purpose: analytics::private;
 *
 * ============================================================================
 */

secureComputationField
    : qualifiedName
      COLON
      expression
      SEMI?
    ;


secureComputationProperty
    : qualifiedName
      COLON
      expression
      SEMI?
    ;


secureComputationMetadata
    : HASH
      LBRACKET
      qualifiedName
      (ASSIGN expression)?
      RBRACKET
    ;


/*
 * ============================================================================
 * 4. PARTICIPANTS
 * ============================================================================
 *
 * Participants are logical identities, not physical nodes.
 *
 * Valid conceptual examples:
 *
 *     party alice;
 *     party service::analytics;
 *     participant organization::A;
 *
 * The grammar does not limit the number of participants.
 *
 * Physical realization belongs downstream.
 * ============================================================================
 */

secureComputationParty
    : secureComputationPartyKeyword
      qualifiedName
      secureComputationPartyBody?
      SEMI?
    ;


secureComputationPartyKeyword
    : IDENTIFIER
    ;


secureComputationPartyBody
    : LBRACE
      secureComputationPartyMember*
      RBRACE
    ;


secureComputationPartyMember
    : secureComputationField
    | secureComputationRequirement
    | secureComputationConstraint
    | secureComputationPreference
    ;


/*
 * ============================================================================
 * 5. INPUTS
 * ============================================================================
 *
 * Inputs are classified by semantic intent.
 *
 * The grammar does not embed data or secrets.
 *
 * ============================================================================
 */

secureComputationInput
    : secureComputationInputKeyword
      qualifiedName
      secureComputationBinding?
      secureComputationInputBody?
      SEMI?
    ;


secureComputationInputKeyword
    : IDENTIFIER
    ;


secureComputationBinding
    : COLON
      typeExpression
    ;


secureComputationInputBody
    : LBRACE
      secureComputationMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 6. OUTPUTS
 * ============================================================================
 */

secureComputationOutput
    : secureComputationOutputKeyword
      qualifiedName
      secureComputationBinding?
      secureComputationOutputBody?
      SEMI?
    ;


secureComputationOutputKeyword
    : IDENTIFIER
    ;


secureComputationOutputBody
    : LBRACE
      secureComputationMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 7. SECURE OPERATIONS
 * ============================================================================
 *
 * Secure operations use the universal Zamani `apply` operation model.
 *
 * Examples:
 *
 *     apply secure::computation::operation(value);
 *
 *     apply secure::computation::operation(a, b);
 *
 *     apply secure::computation::protocol::operation(input) {
 *         requires capability("secure.compute");
 *     }
 *
 * No operation catalogue is encoded here.
 *
 * ============================================================================
 */

secureComputationOperation
    : APPLY
      qualifiedName
      secureComputationArguments?
      secureComputationOperationBody?
      SEMI?
    ;


secureComputationArguments
    : LPAREN
      secureComputationArgumentList?
      RPAREN
    ;


secureComputationArgumentList
    : expression
      (
          COMMA
          expression
      )*
      COMMA?
    ;


secureComputationOperationBody
    : LBRACE
      secureComputationOperationMember*
      RBRACE
    ;


secureComputationOperationMember
    : secureComputationField
    | secureComputationRequirement
    | secureComputationConstraint
    | secureComputationPreference
    | secureComputationProperty
    | secureComputationCapability
    ;


/*
 * ============================================================================
 * 8. SECRET / PUBLIC / SHARED INTENT
 * ============================================================================
 *
 * These are intentionally contextual identifiers rather than dedicated
 * keywords.
 *
 * The semantic layer recognizes categories such as:
 *
 *     public
 *     private
 *     secret
 *     shared
 *     masked
 *     committed
 *     replicated
 *
 * without forcing those words into the global keyword namespace.
 *
 * ============================================================================
 */

secureComputationDataClassification
    : qualifiedName
    ;


secureComputationDataDeclaration
    : secureComputationDataClassification
      qualifiedName
      (COLON typeExpression)?
      SEMI?
    ;


/*
 * ============================================================================
 * 9. REQUIREMENTS
 * ============================================================================
 *
 * A requirement is mandatory.
 *
 * Examples:
 *
 *     requires capability("secure.compute");
 *
 *     requires secure::computation::privacy;
 *
 *     requires secure::computation::threshold >= threshold_value;
 *
 *     requires secure::computation::isolation;
 *
 * The actual satisfiability check is semantic.
 * ============================================================================
 */

secureComputationRequirement
    : REQUIRES
      secureComputationRequirementExpression
      SEMI?
    ;


secureComputationRequirementExpression
    : secureComputationRequirementDisjunction
    ;


secureComputationRequirementDisjunction
    : secureComputationRequirementConjunction
      (
          OR
          secureComputationRequirementConjunction
      )*
    ;


secureComputationRequirementConjunction
    : secureComputationRequirementUnary
      (
          AND
          secureComputationRequirementUnary
      )*
    ;


secureComputationRequirementUnary
    : NOT
      secureComputationRequirementUnary
    | secureComputationRequirementPrimary
    ;


secureComputationRequirementPrimary
    : qualifiedName
    | expression
    | LPAREN
      secureComputationRequirementExpression
      RPAREN
    ;


/*
 * ============================================================================
 * 10. CONSTRAINTS
 * ============================================================================
 *
 * Constraints restrict realization but do not themselves choose a target.
 *
 * ============================================================================
 */

secureComputationConstraint
    : secureComputationConstraintKeyword
      secureComputationConstraintExpression
      SEMI?
    ;


secureComputationConstraintKeyword
    : IDENTIFIER
    ;


secureComputationConstraintExpression
    : expression
    | qualifiedName
    | LPAREN
      secureComputationConstraintExpression
      RPAREN
    ;


/*
 * ============================================================================
 * 11. PREFERENCES
 * ============================================================================
 *
 * Preferences are non-mandatory implementation intent.
 *
 * They MUST NOT be interpreted as hard requirements by the parser.
 * ============================================================================
 */

secureComputationPreference
    : secureComputationPreferenceKeyword
      secureComputationPreferenceExpression
      SEMI?
    ;


secureComputationPreferenceKeyword
    : IDENTIFIER
    ;


secureComputationPreferenceExpression
    : expression
    | qualifiedName
    | LPAREN
      secureComputationPreferenceExpression
      RPAREN
    ;


/*
 * ============================================================================
 * 12. CAPABILITIES
 * ============================================================================
 *
 * Capability names are open-world.
 *
 * Examples:
 *
 *     capability("secure.compute")
 *     capability("threshold.compute")
 *     capability("confidential.execution")
 *     capability("attestation")
 *     capability("secure.aggregation")
 *
 * The grammar does not decide whether a target supplies them.
 * ============================================================================
 */

secureComputationCapability
    : CAPABILITY
      LPAREN
      expression
      RPAREN
      SEMI?
    ;


/*
 * ============================================================================
 * 13. SECURE REVEAL / RELEASE INTENT
 * ============================================================================
 *
 * A reveal is a semantic security boundary.
 *
 * It does not mean that the parser or runtime automatically releases data.
 *
 * Example:
 *
 *     apply secure::computation::reveal(secret_value);
 *
 * The semantic security analyzer determines whether such a release is
 * permitted.
 *
 * ============================================================================
 */

secureComputationReveal
    : APPLY
      qualifiedName
      LPAREN
      expression
      RPAREN
      SEMI?
    ;


/*
 * ============================================================================
 * 14. THRESHOLD / SHARING PARAMETERS
 * ============================================================================
 *
 * Thresholds remain source-level values.
 *
 * No maximum threshold is imposed.
 *
 * Example semantic metadata:
 *
 *     threshold: required_threshold;
 *     shares: share_count;
 *
 * The grammar never turns them into machine limits.
 * ============================================================================
 */

secureComputationThreshold
    : qualifiedName
      COLON
      expression
      SEMI?
    ;


secureComputationSharing
    : qualifiedName
      COLON
      qualifiedName
      SEMI?
    ;


/*
 * ============================================================================
 * 15. SECURITY PROPERTIES
 * ============================================================================
 *
 * Properties remain open-world.
 *
 * Examples:
 *
 *     confidentiality
 *     privacy
 *     integrity
 *     non_interference
 *     obliviousness
 *     provenance
 *     auditability
 *     isolation
 *     forward_secrecy
 *
 * No finite property vocabulary is imposed here.
 * ============================================================================
 */

secureComputationPropertyReference
    : qualifiedName
    ;


secureComputationPropertyDeclaration
    : secureComputationPropertyReference
      (COLON expression)?
      SEMI?
    ;


/*
 * ============================================================================
 * 16. RESOURCE REQUIREMENTS
 * ============================================================================
 *
 * Secure computation may require resources, but resource semantics belong to
 * the common resource/capability system.
 *
 * This grammar does not create:
 *
 *     MAX_MEMORY
 *     MAX_PARTIES
 *     MAX_NODES
 *     MAX_THREADS
 *
 * Example:
 *
 *     requires resource::memory >= required_memory;
 *
 * ============================================================================
 */

secureComputationResourceRequirement
    : REQUIRES
      qualifiedName
      (
          EQUAL_EQUAL
        | LESS_EQUAL
        | GREATER_EQUAL
        | LESS
        | GREATER
      )?
      expression
      SEMI?
    ;


/*
 * ============================================================================
 * 17. SECURE-COMPUTATION CONTRACT
 * ============================================================================
 *
 * This rule provides a structural contract for tooling.
 *
 * It deliberately does not introduce a new contract language.
 * ============================================================================
 */

secureComputationContract
    : CONTRACT
      qualifiedName
      LBRACE
      secureComputationContractMember*
      RBRACE
      SEMI?
    ;


secureComputationContractMember
    : secureComputationRequirement
    | secureComputationConstraint
    | secureComputationPreference
    | secureComputationCapability
    | secureComputationProperty
    | secureComputationField
    ;


/*
 * ============================================================================
 * 18. GENERIC PARAMETER INTEGRATION
 * ============================================================================
 *
 * Generic parameters are supplied by the canonical core grammar.
 *
 * This file does not redefine them.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 19. DOMAIN-NEUTRAL AST CONTRACT
 * ============================================================================
 *
 * The parser must lower structurally into the existing domain-neutral
 * frontend AST.
 *
 * Recommended semantic mappings:
 *
 *     secureComputationDeclaration
 *         -> domain-neutral declaration/operation node
 *
 *     secureComputationOperation
 *         -> generic Operation node
 *
 *     secureComputationParty
 *         -> declaration/binding node
 *
 *     secureComputationInput
 *         -> declaration/binding node
 *
 *     secureComputationOutput
 *         -> declaration/binding node
 *
 *     secureComputationRequirement
 *         -> requirement node
 *
 *     secureComputationConstraint
 *         -> constraint node
 *
 *     secureComputationPreference
 *         -> preference node
 *
 *     secureComputationCapability
 *         -> capability requirement node
 *
 * No:
 *
 *     SecureComputationIR
 *     MPCIR
 *     SecretSharingIR
 *     HomomorphicIR
 *
 * is introduced.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 20. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     - name resolution;
 *     - protocol resolution;
 *     - participant validation;
 *     - input/output classification;
 *     - secret-flow validation;
 *     - information-flow analysis;
 *     - reveal/declassification checks;
 *     - capability satisfaction;
 *     - resource feasibility;
 *     - threshold validity;
 *     - protocol compatibility;
 *     - cryptographic compatibility;
 *     - trust requirements;
 *     - privacy guarantees;
 *     - integrity guarantees;
 *     - target capability matching.
 *
 * The parser does none of these.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 21. CRYPTOGRAPHY INTEGRATION
 * ============================================================================
 *
 * Cryptographic primitives are owned by:
 *
 *     grammar/security/cryptography.g4
 *
 * Hashing is owned by:
 *
 *     grammar/security/hashes.g4
 *
 * Secure computation may reference those mechanisms through qualified names.
 *
 * This grammar MUST NOT duplicate:
 *
 *     hash syntax;
 *     signature syntax;
 *     key syntax;
 *     cryptographic algorithm syntax.
 *
 * Example:
 *
 *     requires cryptography::property::constant_time;
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 22. SECRETS INTEGRATION
 * ============================================================================
 *
 * Secret lifecycle belongs to:
 *
 *     grammar/security/secrets.g4
 *
 * Key management belongs to:
 *
 *     grammar/security/key-management.g4
 *
 * This grammar may reference symbolic names owned by those subsystems but
 * must not duplicate their declarations.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 23. IDENTITY / AUTHORIZATION INTEGRATION
 * ============================================================================
 *
 * Participant names may resolve to identities defined by:
 *
 *     grammar/security/identifiers.g4
 *
 * Authorization is owned by:
 *
 *     grammar/security/authorization.g4
 *     grammar/security/permissions.g4
 *     grammar/security/capabilities.g4
 *
 * Secure computation does not decide authorization.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 24. DISTRIBUTED-COMPUTING INTEGRATION
 * ============================================================================
 *
 * Secure computation can be distributed across arbitrary logical locations.
 *
 * Node placement, process placement, topology, replication and communication
 * are owned by:
 *
 *     grammar/distributed/
 *     grammar/networking/
 *     grammar/resources/
 *     grammar/hardware/
 *
 * This grammar does not encode:
 *
 *     node0
 *     node1
 *     node2
 *
 * or any fixed topology.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 25. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Secure computation may protect or operate over quantum-related data.
 *
 * Quantum semantics remain owned by:
 *
 *     grammar/quantum/
 *
 * and the canonical semantic boundary:
 *
 *     quantum::ir
 *
 * This grammar does NOT define:
 *
 *     qubits;
 *     physical qubits;
 *     gates;
 *     topology;
 *     routing;
 *     scheduling;
 *     QEC;
 *     ZQN.
 *
 * Security metadata may accompany quantum::ir downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 26. HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Secure-computation requirements may constrain hardware/software co-design.
 *
 * Hardware realization belongs to:
 *
 *     grammar/hardware/
 *     grammar/hdl/
 *
 * This grammar does not specify:
 *
 *     register width;
 *     bus width;
 *     memory capacity;
 *     accelerator count;
 *     device count;
 *     topology.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 27. AI / DATA INTEGRATION
 * ============================================================================
 *
 * Secure computation may protect:
 *
 *     model data;
 *     datasets;
 *     inference;
 *     training;
 *     aggregation;
 *     distributed learning.
 *
 * AI semantics remain owned by:
 *
 *     grammar/ai/
 *
 * Data semantics remain owned by:
 *
 *     grammar/data/
 *
 * This grammar expresses security intent around those constructs rather than
 * redefining them.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 28. NETWORKING INTEGRATION
 * ============================================================================
 *
 * Secure channels, endpoints and communication requirements are resolved by:
 *
 *     grammar/networking/
 *
 * This grammar may reference abstract capabilities such as:
 *
 *     secure::channel
 *     authenticated::transport
 *     confidential::transport
 *
 * but does not implement networking.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 29. RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Resource discovery is downstream.
 *
 * Examples:
 *
 *     requires capability("secure.compute");
 *
 *     requires capability("attestation");
 *
 *     requires capability("confidential.execution");
 *
 *     requires resource::memory >= required_memory;
 *
 * These are requirements, not hard-coded capacities.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 30. COMPILATION / EXECUTION INTEGRATION
 * ============================================================================
 *
 * Compilation may:
 *
 *     resolve protocols;
 *     specialize secure operations;
 *     select available implementations;
 *     preserve security requirements;
 *     attach security metadata;
 *     lower operations into canonical representations.
 *
 * Runtime may:
 *
 *     establish secure sessions;
 *     validate credentials;
 *     evaluate policy;
 *     verify attestation;
 *     execute secure protocols;
 *     enforce information-flow policies.
 *
 * None of those actions occur during parsing.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 31. CANONICAL IR CONTRACT
 * ============================================================================
 *
 * Secure computation does NOT create a separate IR.
 *
 * Conceptually:
 *
 *     SecureComputationSyntax
 *             |
 *             v
 *     domain-neutral AST
 *             |
 *             v
 *     semantic secure-computation operation
 *             |
 *             +------------------------+
 *             |                        |
 *             v                        v
 *       classical representation   quantum::ir metadata
 *             |                        |
 *             +------------+-----------+
 *                          |
 *                          v
 *                     canonical IR
 *                          |
 *             +------------+------------+
 *             |            |            |
 *             v            v            v
 *          optimize      route       schedule
 *                                      |
 *                                      v
 *                                   resilience
 *                                      |
 *                                      v
 *                                     ZQN
 *                                      |
 *                                      v
 *                                     HAL
 *
 * No second secure-computation IR is permitted.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 32. DETERMINISM CONTRACT
 * ============================================================================
 *
 * Given identical:
 *
 *     source;
 *     language version;
 *     lexer version;
 *     parser version;
 *
 * this grammar must produce the same parse structure.
 *
 * Parsing must not depend upon:
 *
 *     hardware;
 *     available providers;
 *     network;
 *     filesystem;
 *     wall-clock time;
 *     randomness;
 *     runtime state;
 *     secret material.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 33. SECURITY CONTRACT
 * ============================================================================
 *
 * The grammar itself performs no:
 *
 *     authentication;
 *     authorization;
 *     cryptographic execution;
 *     secret access;
 *     network communication;
 *     hardware discovery;
 *     environment inspection.
 *
 * No parser action or semantic predicate is permitted.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 34. SAFE-RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust code.
 *
 * The compiler/frontend consuming it must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and must remain safe Rust.
 *
 * The implementation should enforce:
 *
 *     #![forbid(unsafe_code)]
 *
 * at the applicable Rust crate boundary.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 35. HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden:
 *
 *     MAX_PARTIES
 *     MAX_PARTICIPANTS
 *     MAX_SECRETS
 *     MAX_SHARES
 *     MAX_ROUNDS
 *     MAX_SESSIONS
 *     MAX_PROTOCOLS
 *     MAX_OPERATIONS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *
 * Forbidden physical assumptions:
 *
 *     cpu0
 *     gpu0
 *     fpga0
 *     qpu0
 *     node0
 *     memory_bank0
 *
 * Program values such as:
 *
 *     let threshold = 3;
 *
 * remain valid source semantics.
 *
 * The distinction is:
 *
 *     program value
 *         !=
 *     compiler limit
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 36. POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * The validation suite must cover forms equivalent to:
 *
 *     secure computation::protocol::example {
 *         party alice;
 *         party bob;
 *
 *         input secret_data: Data;
 *         output result: Data;
 *
 *         requires capability("secure.compute");
 *     }
 *
 *     apply secure::computation::operation(value);
 *
 *     apply secure::computation::operation(a, b);
 *
 *     apply secure::computation::future::operation(input) {
 *         requires capability("confidential.execution");
 *     }
 *
 *     secure computation::session::example {
 *         requires secure::computation::privacy;
 *         requires capability("attestation");
 *     }
 *
 * These names are semantic examples, not reserved algorithms/protocols.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 37. NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * Syntax tests must reject malformed forms such as:
 *
 *     secure
 *
 *     secure computation::protocol::x {
 *
 *     apply secure::computation::operation(
 *
 *     requires
 *
 *     capability(
 *
 * Semantic tests must separately cover:
 *
 *     unresolved protocol;
 *     unresolved participant;
 *     invalid secret flow;
 *     unauthorized reveal;
 *     unavailable capability;
 *     insufficient resource;
 *     incompatible protocol;
 *     conflicting security requirements;
 *     invalid threshold semantics.
 *
 * The parser must not attempt to solve these semantic problems.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 38. BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Test:
 *
 *     zero secure-computation declarations;
 *     one participant;
 *     many participants;
 *     one input;
 *     many inputs;
 *     one output;
 *     many outputs;
 *     nested secure-computation bodies;
 *     deeply qualified names;
 *     symbolic thresholds;
 *     symbolic resource requirements;
 *     very large source values;
 *     classical/secure composition;
 *     quantum/secure composition;
 *     hybrid/secure composition;
 *     distributed/secure composition;
 *     AI/data secure computation;
 *     networking secure computation;
 *     HDL/hardware secure computation.
 *
 * No test may establish a universal maximum.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 39. SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * The test suite must exercise progressively larger programs while ensuring
 * that no grammar rule introduces an artificial semantic ceiling.
 *
 * Scale dimensions include:
 *
 *     declaration count;
 *     operation count;
 *     participant count;
 *     input count;
 *     output count;
 *     requirement count;
 *     constraint count;
 *     preference count;
 *     qualified-name depth;
 *     nested body depth;
 *     expression complexity.
 *
 * These are test dimensions, not language constants.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 40. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing canonical lexer/parser vocabulary remains authoritative.
 *
 * This file introduces only the dedicated `SECURE` lexical marker.
 *
 * No existing token is renamed.
 *
 * No existing grammar file is renamed.
 *
 * No alternate parser root is introduced.
 *
 * `grammar/Zamani.g4` remains the program root.
 *
 * `grammar/antlr/ZamaniParser.g4` remains the parser composition root.
 *
 * `grammar/security/security.g4` remains the security composition root.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 41. VALIDATION CONTRACT
 * ============================================================================
 *
 * grammar/validation must verify:
 *
 *     - this file is a parser grammar;
 *     - tokenVocab is ZamaniLexer;
 *     - no lexer rules exist here;
 *     - no Rust actions exist;
 *     - no semantic predicates exist;
 *     - no fixed protocol catalogue exists;
 *     - no fixed party count exists;
 *     - no fixed share count exists;
 *     - no fixed round count exists;
 *     - no hardware limits exist;
 *     - no physical device IDs are required;
 *     - no second security IR exists;
 *     - no secret literal is introduced;
 *     - canonical Core/Types/Expressions ownership is reused;
 *     - Security imports this grammar exactly once;
 *     - ZamaniParser imports Security rather than this file directly;
 *     - Zamani.g4 remains the only complete-program root.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 42. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [x] Secure computation has one grammar owner.
 * [x] Canonical ZamaniLexer remains the lexer authority.
 * [x] General names are delegated to Core.
 * [x] General expressions are delegated to Expressions.
 * [x] General types are delegated to Types.
 * [x] Effects remain delegated to Effects.
 * [x] Secure-computation mechanisms are open-world.
 * [x] Protocols are not enumerated.
 * [x] Secret-sharing schemes are not enumerated.
 * [x] Homomorphic schemes are not enumerated.
 * [x] TEE technologies are not enumerated.
 * [x] Participant count is unbounded by grammar.
 * [x] Operation count is unbounded by grammar.
 * [x] Session count is unbounded by grammar.
 * [x] Resource capacity is not hard-coded.
 * [x] Hardware identity is not hard-coded.
 * [x] Quantum resources are not hard-coded.
 * [x] Secret material is not embedded.
 * [x] Cryptography ownership remains in cryptography.g4.
 * [x] Secret ownership remains in secrets.g4.
 * [x] Key-management ownership remains in key-management.g4.
 * [x] Authorization ownership remains in authorization/permissions.
 * [x] Distributed ownership remains in distributed/.
 * [x] Quantum ownership remains in quantum/.
 * [x] Hardware ownership remains in hardware/.
 * [x] No second IR is created.
 * [x] Canonical quantum::ir remains the quantum boundary.
 * [x] Parsing is deterministic.
 * [x] No parser actions exist.
 * [x] No semantic predicates exist.
 * [x] Rust 1.97 / 1.97.1 compatibility is documented.
 * [x] Safe-Rust requirement is preserved.
 * [x] Positive tests are specified.
 * [x] Negative tests are specified.
 * [x] Boundary tests are specified.
 * [x] Scalability tests are specified.
 * [x] Compatibility tests are specified.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * Secure computation is a portable semantic capability of Zamani.
 *
 * It is NOT:
 *
 *     a protocol catalogue;
 *     a cryptographic implementation;
 *     a secret store;
 *     a hardware model;
 *     a distributed-runtime implementation;
 *     a quantum IR;
 *     a compiler backend.
 *
 * Therefore:
 *
 *     Program Once
 *          ->
 *     Compile Once
 *          ->
 *     Run Everywhere
 *          ->
 *     Run Anywhere
 *          ->
 *     Forever
 *
 * remains the architectural objective, subject to actual program semantics,
 * security requirements, available capabilities, available resources and
 * target realization.
 *
 * ============================================================================
 */
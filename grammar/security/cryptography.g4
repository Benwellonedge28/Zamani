/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/security/cryptography.g4
 *
 * ROLE
 * ----
 * Canonical parser-level cryptography-domain grammar.
 *
 * STATUS
 * ------
 * PRODUCTION TARGET
 *
 * LANGUAGE
 * --------
 * Zamani
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 1.97 / Rust 1.97.1
 * Rust 2021
 * Safe Rust only.
 * No unsafe Rust is required.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar defines portable cryptographic intent.
 *
 * It describes:
 *
 *   - cryptographic operations;
 *   - cryptographic objects;
 *   - algorithm references;
 *   - primitive references;
 *   - key references;
 *   - protocol references;
 *   - security properties;
 *   - requirements;
 *   - constraints;
 *   - preferences;
 *   - provider references;
 *   - implementation references;
 *   - metadata;
 *   - hash delegation;
 *   - cryptographic composition.
 *
 * It does NOT implement cryptography.
 *
 * It does NOT:
 *
 *   - execute cryptographic operations;
 *   - discover providers;
 *   - discover hardware;
 *   - resolve keys;
 *   - access secrets;
 *   - select CPUs;
 *   - select GPUs;
 *   - select FPGAs;
 *   - select QPUs;
 *   - perform routing;
 *   - perform scheduling;
 *   - perform QEC;
 *   - perform ZQN;
 *   - create a second IR.
 *
 * ============================================================================
 * AUTHORITY
 * ============================================================================
 *
 * Language specification:
 *
 *     grammar/specification/
 *
 * Canonical parser composition:
 *
 *     grammar/Zamani.g4
 *
 * Canonical ANTLR lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Security composition:
 *
 *     grammar/security/security.g4
 *
 * Hash-specific syntax:
 *
 *     grammar/security/hashes.g4
 *
 * Frontend AST:
 *
 *     src/frontend/ast/
 *
 * Canonical quantum semantic boundary:
 *
 *     quantum::ir
 *
 * This file must never become a competing root grammar.
 *
 * ============================================================================
 * IMPORTANT LEXICAL DESIGN
 * ============================================================================
 *
 * The previous cryptography grammar depended on numerous lexer tokens that
 * are not part of the current canonical Zamani lexical vocabulary.
 *
 * This implementation deliberately does NOT require tokens such as:
 *
 *     ALGORITHM
 *     PRIMITIVE
 *     KEY_REFERENCE
 *     KEY_ROLE
 *     PROVIDER
 *     IMPLEMENTATION
 *     PARAMETER
 *     METADATA
 *     REQUIREMENT
 *     PREFERENCE
 *     PURPOSE
 *     PROTOCOL
 *     PARTICIPANT
 *     LIFECYCLE
 *     STORAGE
 *     ROLE
 *     CRYPTOGRAPHIC_OPERATION
 *
 * merely because these concepts exist semantically.
 *
 * Cryptographic field names are represented by ordinary identifiers and
 * qualified names.
 *
 * This keeps the grammar compatible with the repository's open-world lexical
 * architecture.
 *
 * A future language-version change may reserve selected words, but such a
 * change belongs to the canonical lexical specification and compatibility
 * process rather than being silently introduced here.
 *
 * ============================================================================
 * OPEN-WORLD PRINCIPLE
 * ============================================================================
 *
 * Cryptographic algorithms are semantic names.
 *
 * This grammar MUST NOT enumerate:
 *
 *     AES
 *     AES-GCM
 *     ChaCha20
 *     RSA
 *     ECC
 *     SHA-256
 *     SHA-3
 *     BLAKE2
 *     BLAKE3
 *     Ed25519
 *     ML-KEM
 *     ML-DSA
 *     SLH-DSA
 *
 * or any other finite algorithm catalogue.
 *
 * All such names are represented by:
 *
 *     qualifiedName
 *
 * Examples:
 *
 *     cryptography::hash::sha256
 *     cryptography::signature::ed25519
 *     future::cryptography::algorithm
 *     vendor::cryptography::implementation
 *
 * The semantic layer decides whether the reference exists and whether it is
 * valid.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Cryptographic programs must remain portable across:
 *
 *     embedded systems
 *     CPUs
 *     multicore CPUs
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     QPUs
 *     simulators
 *     HPC systems
 *     clusters
 *     distributed systems
 *     cloud systems
 *     future computational substrates
 *
 * This grammar imposes NO universal limits on:
 *
 *     algorithms
 *     keys
 *     operations
 *     protocols
 *     participants
 *     inputs
 *     outputs
 *     properties
 *     requirements
 *     constraints
 *     devices
 *     nodes
 *     processors
 *     memory
 *     storage
 *     qubits
 *     threads
 *
 * It MUST NOT define:
 *
 *     MAX_KEYS
 *     MAX_ALGORITHMS
 *     MAX_CRYPTO_OPERATIONS
 *     MAX_PROTOCOLS
 *     MAX_PARTICIPANTS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *
 * "Infinity" means that the language itself imposes no arbitrary semantic
 * ceiling. Actual execution remains bounded by available resources and
 * implementation capabilities.
 *
 * ============================================================================
 * REQUIREMENT / CONSTRAINT / PREFERENCE
 * ============================================================================
 *
 * These are different semantic categories.
 *
 * REQUIREMENT
 *     A mandatory condition.
 *
 * CONSTRAINT
 *     A condition restricting permitted realizations.
 *
 * PREFERENCE
 *     A desired but non-mandatory property.
 *
 * The parser preserves the distinction.
 *
 * Semantic analysis determines satisfiability.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * This grammar does not encode:
 *
 *     cpu0
 *     gpu0
 *     fpga0
 *     qpu0
 *     hsm0
 *     node0
 *     memory_bank0
 *
 * as special language constructs.
 *
 * Physical realization belongs downstream.
 *
 * ============================================================================
 * SECRET-MATERIAL BOUNDARY
 * ============================================================================
 *
 * Cryptographic source may reference protected material but this grammar does
 * not provide special literal syntax for:
 *
 *     private keys
 *     secret keys
 *     passwords
 *     API keys
 *     credentials
 *     secret seeds
 *
 * A string literal is syntactically an ordinary expression.
 *
 * Whether a particular expression is permitted as cryptographic input is a
 * semantic/security decision.
 *
 * Secret storage and lifecycle remain owned by the security secret subsystem.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     canonical lexer
 *       |
 *       v
 *     canonical parser
 *       |
 *       v
 *     cryptographic syntax
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic security analysis
 *       |
 *       v
 *     canonical semantic model
 *       |
 *       +-------------------------+
 *       |                         |
 *       v                         v
 *   classical representation   quantum::ir metadata
 *       |                         |
 *       +------------+------------+
 *                    |
 *                    v
 *               optimization
 *                    |
 *          +---------+---------+
 *          |         |         |
 *          v         v         v
 *       routing  scheduling resilience
 *                              |
 *                              v
 *                             ZQN
 *                              |
 *                              v
 *                             HAL
 *                              |
 *                              v
 *                           target
 *
 * ============================================================================
 */

parser grammar Cryptography;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * PUBLIC ENTRY POINTS
 * ========================================================================== */

/*
 * Standalone cryptography grammar entry point.
 *
 * The complete language normally enters cryptography through:
 *
 *     grammar/security/security.g4
 *
 * and ultimately:
 *
 *     grammar/Zamani.g4
 */
cryptographyFile
    : cryptographicEntry* EOF
    ;


/*
 * A cryptographic entry is deliberately structural rather than algorithm-
 * specific.
 */
cryptographicEntry
    : cryptographicDeclaration
    | cryptographicOperation
    | cryptographicRequirementDeclaration
    | cryptographicConstraintDeclaration
    | cryptographicPreferenceDeclaration
    | cryptographicObjectDeclaration
    ;


/* ============================================================================
 * CRYPTOGRAPHIC DECLARATIONS
 * ========================================================================== */

/*
 * A cryptographic declaration is introduced by the canonical cryptography
 * namespace marker.
 *
 * The exact semantic category is carried by the qualified name.
 *
 * Conceptual forms:
 *
 *     cryptography::algorithm::...
 *     cryptography::protocol::...
 *     cryptography::key::...
 *     cryptography::primitive::...
 *
 * The grammar intentionally does not enumerate those categories.
 *
 * The surrounding security composition is responsible for exposing the
 * cryptographic declaration entry point.
 */
cryptographicDeclaration
    : cryptographicDeclarationHead
      cryptographicBody?
      SEMICOLON?
    ;


cryptographicDeclarationHead
    : cryptographicKeywordReference
      qualifiedName
    ;


/*
 * The lexical spelling `cryptography` remains an ordinary identifier unless
 * the canonical lexical specification explicitly promotes it to a reserved
 * keyword.
 *
 * This rule therefore represents the namespace structurally rather than
 * depending on a missing CRYPTOGRAPHY lexer token.
 */
cryptographicKeywordReference
    : identifier
    ;


cryptographicBody
    : LBRACE
      cryptographicMember*
      RBRACE
    ;


cryptographicMember
    : cryptographicField
    | cryptographicOperation
    | cryptographicRequirement
    | cryptographicConstraint
    | cryptographicPreference
    | cryptographicMetadata
    | hashDelegation
    ;


/* ============================================================================
 * GENERIC CRYPTOGRAPHIC FIELDS
 * ========================================================================== */

/*
 * Generic fields prevent cryptography.g4 from becoming a closed dictionary of
 * algorithms and security mechanisms.
 *
 * Examples:
 *
 *     algorithm: cryptography::hash::sha256;
 *     primitive: cryptography::digest;
 *     purpose: application::identity;
 *     property: collision_resistance;
 *     provider: vendor::provider;
 *     implementation: platform::accelerator;
 *
 * The field name is syntax.
 *
 * Its meaning is semantic.
 */
cryptographicField
    : qualifiedName
      COLON
      expression
      SEMICOLON?
    ;


/*
 * A name-only cryptographic field is useful for references that do not require
 * an expression wrapper.
 */
cryptographicReferenceField
    : qualifiedName
      COLON
      qualifiedName
      SEMICOLON?
    ;


/* ============================================================================
 * CRYPTOGRAPHIC OPERATIONS
 * ========================================================================== */

/*
 * Generic cryptographic operations use Zamani's existing APPLY vocabulary.
 *
 * Conceptual forms:
 *
 *     apply cryptography::encrypt(data);
 *
 *     apply cryptography::sign(message, signing_key);
 *
 *     apply cryptography::verify(message, signature, key);
 *
 *     apply cryptography::hash(data);
 *
 * The operation name remains open-world.
 */
cryptographicOperation
    : APPLY
      qualifiedName
      cryptographicArgumentList?
      cryptographicOperationBody?
      SEMICOLON?
    ;


cryptographicArgumentList
    : LPAREN
      cryptographicArguments?
      RPAREN
    ;


cryptographicArguments
    : expression
      (
          COMMA
          expression
      )*
    ;


cryptographicOperationBody
    : LBRACE
      cryptographicOperationMember*
      RBRACE
    ;


cryptographicOperationMember
    : cryptographicField
    | cryptographicReferenceField
    | cryptographicRequirement
    | cryptographicConstraint
    | cryptographicPreference
    | cryptographicMetadata
    | hashDelegation
    ;


/* ============================================================================
 * REQUIREMENTS
 * ========================================================================== */

/*
 * Mandatory cryptographic requirement.
 *
 * Examples:
 *
 *     requires capability("cryptography.compute");
 *
 *     requires cryptography::property::collision_resistance;
 *
 *     requires cryptography::algorithm::approved;
 *
 *     requires memory >= required_memory;
 *
 * Resource and capability expressions remain semantic expressions.
 */
cryptographicRequirement
    : REQUIRES
      expression
      SEMICOLON?
    ;


cryptographicRequirementDeclaration
    : CONTRACT
      qualifiedName
      cryptographicContractBody?
      SEMICOLON?
    ;


cryptographicContractBody
    : LBRACE
      cryptographicContractMember*
      RBRACE
    ;


cryptographicContractMember
    : cryptographicRequirement
    | cryptographicConstraint
    | cryptographicPreference
    | cryptographicField
    | cryptographicMetadata
    ;


/* ============================================================================
 * CONSTRAINTS
 * ========================================================================== */

cryptographicConstraint
    : CONSTRAINT
      expression
      SEMICOLON?
    ;


cryptographicConstraintDeclaration
    : CONSTRAINT
      qualifiedName
      cryptographicContractBody?
      SEMICOLON?
    ;


/* ============================================================================
 * PREFERENCES
 * ========================================================================== */

cryptographicPreference
    : PREFER
      expression
      SEMICOLON?
    ;


cryptographicPreferenceDeclaration
    : PREFER
      qualifiedName
      cryptographicContractBody?
      SEMICOLON?
    ;


/* ============================================================================
 * CRYPTOGRAPHIC OBJECT REFERENCES
 * ========================================================================== */

/*
 * This is an intentionally generic object declaration.
 *
 * The semantic namespace determines whether it denotes:
 *
 *     key
 *     protocol
 *     primitive
 *     credential handle
 *     provider
 *     implementation
 *     certificate reference
 *     cryptographic service
 *     future object category
 */
cryptographicObjectDeclaration
    : qualifiedName
      cryptographicObjectBody?
      SEMICOLON?
    ;


cryptographicObjectBody
    : LBRACE
      cryptographicObjectMember*
      RBRACE
    ;


cryptographicObjectMember
    : cryptographicField
    | cryptographicRequirement
    | cryptographicConstraint
    | cryptographicPreference
    | cryptographicMetadata
    ;


/* ============================================================================
 * ALGORITHM / PRIMITIVE / KEY / PROTOCOL REFERENCES
 * ========================================================================== */

/*
 * Open-world references.
 *
 * No finite algorithm catalogue is embedded.
 */
algorithmReference
    : qualifiedName
    ;


algorithmReferenceList
    : algorithmReference
      (
          COMMA
          algorithmReference
      )*
      COMMA?
    ;


primitiveReference
    : qualifiedName
    ;


primitiveReferenceList
    : primitiveReference
      (
          COMMA
          primitiveReference
      )*
      COMMA?
    ;


keyReference
    : qualifiedName
    ;


keyReferenceList
    : keyReference
      (
          COMMA
          keyReference
      )*
      COMMA?
    ;


protocolReference
    : qualifiedName
    ;


protocolReferenceList
    : protocolReference
      (
          COMMA
          protocolReference
      )*
      COMMA?
    ;


/* ============================================================================
 * PROVIDER / IMPLEMENTATION REFERENCES
 * ========================================================================== */

providerReference
    : qualifiedName
    ;


implementationReference
    : qualifiedName
    ;


/*
 * Provider and implementation names are opaque semantic references.
 *
 * Parsing them does not:
 *
 *     discover providers;
 *     load implementations;
 *     inspect hardware;
 *     access the network;
 *     access the filesystem.
 */
providerReferenceClause
    : qualifiedName
      COLON
      providerReference
      SEMICOLON?
    ;


implementationReferenceClause
    : qualifiedName
      COLON
      implementationReference
      SEMICOLON?
    ;


/* ============================================================================
 * PURPOSE / PROPERTY / PARAMETER REFERENCES
 * ========================================================================== */

cryptographicPurpose
    : qualifiedName
    ;


cryptographicProperty
    : qualifiedName
    ;


cryptographicParameter
    : qualifiedName
    ;


cryptographicPurposeList
    : cryptographicPurpose
      (
          COMMA
          cryptographicPurpose
      )*
      COMMA?
    ;


cryptographicPropertyList
    : cryptographicProperty
      (
          COMMA
          cryptographicProperty
      )*
      COMMA?
    ;


/* ============================================================================
 * METADATA
 * ========================================================================== */

/*
 * Metadata remains generic.
 *
 * This prevents cryptography from creating a second metadata language.
 */
cryptographicMetadata
    : HASH
      LBRACE
      cryptographicMetadataEntry*
      RBRACE
    ;


cryptographicMetadataEntry
    : qualifiedName
      COLON
      expression
      SEMICOLON?
    ;


/*
 * IMPORTANT:
 *
 * HASH above is the existing lexical punctuation token `#`, not a hash
 * algorithm keyword.
 *
 * Therefore metadata may use the canonical attribute/directive spelling:
 *
 *     #[...]
 *
 * only where the surrounding grammar permits it.
 *
 * If metadata is not represented with `#` in the canonical security
 * composition, this rule should be delegated to the canonical core metadata
 * grammar rather than expanded here.
 */


/* ============================================================================
 * HASH INTEGRATION
 * ========================================================================== */

/*
 * Hashing is a specialized cryptographic domain.
 *
 * Ownership:
 *
 *     cryptography.g4
 *         cryptographic composition
 *
 *     hashes.g4
 *         hash-specific syntax
 *
 * This file therefore exposes only a delegation boundary.
 *
 * It must not copy the hash grammar.
 *
 * IMPORTANT:
 *
 * Because the current lexical vocabulary does not reserve HASH as a keyword,
 * hashes.g4 must remain namespace/operation based unless HASH is deliberately
 * promoted through the canonical lexer specification.
 */
hashDelegation
    : APPLY
      hashQualifiedOperation
      cryptographicArgumentList?
      hashDelegatedBody?
      SEMICOLON?
    ;


hashQualifiedOperation
    : qualifiedName
    ;


hashDelegatedBody
    : LBRACE
      cryptographicField*
      RBRACE
    ;


/*
 * The semantic layer recognizes the hash namespace:
 *
 *     cryptography::hash
 *
 * and delegates the operation contract to hashes.g4.
 *
 * This keeps algorithms open-world.
 */


/* ============================================================================
 * HASH REFERENCE
 * ========================================================================== */

hashReference
    : qualifiedName
    ;


hashReferenceList
    : hashReference
      (
          COMMA
          hashReference
      )*
      COMMA?
    ;


/* ============================================================================
 * SECURITY PROPERTIES
 * ========================================================================== */

/*
 * Security properties are symbolic semantic references.
 *
 * Examples:
 *
 *     collision_resistance
 *     preimage_resistance
 *     second_preimage_resistance
 *     domain_separation
 *     canonical_representation
 *     integrity
 *
 * The grammar does not decide whether a property is actually provided.
 */
securityPropertyReference
    : qualifiedName
    ;


securityPropertyList
    : securityPropertyReference
      (
          COMMA
          securityPropertyReference
      )*
      COMMA?
    ;


/* ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ========================================================================== */

/*
 * Cryptographic requirements may refer to the repository-wide capability and
 * resource model.
 *
 * Examples:
 *
 *     requires capability("cryptography.hash");
 *
 *     requires capability("constant_time");
 *
 *     requires memory >= required_memory;
 *
 *     requires availability >= required_availability;
 *
 * The grammar does not determine whether the requirement is satisfiable.
 */
cryptographicCapabilityRequirement
    : REQUIRES
      CAPABILITY
      LPAREN
      expression
      RPAREN
      SEMICOLON?
    ;


cryptographicResourceRequirement
    : REQUIRES
      RESOURCE
      expression
      SEMICOLON?
    ;


cryptographicTargetRequirement
    : REQUIRES
      TARGET
      expression
      SEMICOLON?
    ;


/* ============================================================================
 * PORTABILITY / SCALABILITY
 * ========================================================================== */

cryptographicPortabilityClause
    : PORTABILITY
      expression
      SEMICOLON?
    ;


cryptographicScalabilityClause
    : SCALABILITY
      expression
      SEMICOLON?
    ;


/* ============================================================================
 * CROSS-DOMAIN ATTACHMENT
 * ========================================================================== */

/*
 * Security metadata may be associated with symbolic targets belonging to:
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
 *     embedded
 *     accelerator
 *     future domains
 *
 * No domain-specific AST is created here.
 */
cryptographicAttachment
    : AT
      qualifiedName
      cryptographicAttachmentBody?
    ;


cryptographicAttachmentBody
    : LBRACE
      cryptographicMember*
      RBRACE
    ;


/* ============================================================================
 * QUANTUM INTEGRATION
 * ========================================================================== */

/*
 * Cryptography may accompany quantum computation, but this grammar does not
 * define quantum operations.
 *
 * It must never introduce:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     QuantumHashIR
 *     SecurityQuantumIR
 *
 * Hash/security metadata reaches the canonical:
 *
 *     quantum::ir
 *
 * boundary through semantic lowering.
 */
cryptographicQuantumAttachment
    : QUANTUM
      qualifiedName
      cryptographicAttachmentBody?
    ;


/* ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ========================================================================== */

cryptographicHardwareAttachment
    : TARGET
      qualifiedName
      cryptographicAttachmentBody?
    ;


/*
 * The target name is symbolic.
 *
 * It does not identify a physical device unless downstream semantics explicitly
 * give it that meaning.
 */


/* ============================================================================
 * DISTRIBUTED INTEGRATION
 * ========================================================================== */

cryptographicDistributedAttachment
    : qualifiedName
      cryptographicAttachmentBody
    ;


/*
 * Distributed topology remains outside this grammar.
 *
 * No node count or topology size is encoded.
 */


/* ============================================================================
 * GENERAL CRYPTOGRAPHIC SPECIFICATION
 * ========================================================================== */

cryptographicSpecification
    : cryptographicMember+
    ;


cryptographicMemberList
    : cryptographicMember+
    ;


/* ============================================================================
 * SEMANTIC PRESERVATION CONTRACT
 * ========================================================================== */

/*
 * The parser-to-AST layer MUST preserve:
 *
 *     source spans
 *     operation names
 *     namespace components
 *     argument expressions
 *     object references
 *     algorithm references
 *     primitive references
 *     key references
 *     protocol references
 *     requirements
 *     constraints
 *     preferences
 *     properties
 *     metadata
 *     target attachments
 *
 * No cryptographic meaning may be silently discarded.
 */


/* ============================================================================
 * AST CONTRACT
 * ========================================================================== */

/*
 * The AST must remain domain-neutral.
 *
 * This grammar MUST NOT require backend-specific nodes such as:
 *
 *     Aes256Node
 *     Sha256Node
 *     RsaNode
 *     QpuHashNode
 *     HsmNode
 *     GpuCryptoNode
 *
 * Instead the frontend should preserve generic semantic information:
 *
 *     operation
 *     qualified name
 *     operands
 *     parameters
 *     properties
 *     requirements
 *     constraints
 *     preferences
 *     metadata
 *     source span
 *
 * Semantic analysis determines cryptographic meaning.
 */


/* ============================================================================
 * SEMANTIC ANALYSIS CONTRACT
 * ========================================================================== */

/*
 * Semantic analysis owns:
 *
 *     algorithm resolution
 *     primitive resolution
 *     key-reference validation
 *     protocol validation
 *     property validation
 *     requirement satisfiability
 *     constraint consistency
 *     preference interpretation
 *     provider availability
 *     implementation availability
 *     security-policy validation
 *     secret-material validation
 *     cryptographic misuse detection
 *     capability validation
 *     resource validation
 *
 * None of these operations occur during parsing.
 */


/* ============================================================================
 * IR CONTRACT
 * ========================================================================== */

/*
 * Cryptography MUST NOT introduce a competing cryptographic IR.
 *
 * The flow is:
 *
 *     cryptographic syntax
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic cryptographic model
 *          |
 *          +-----------------------+
 *          |                       |
 *          v                       v
 *     classical lowering       quantum::ir metadata
 *
 * followed by the normal compiler pipeline.
 *
 * No:
 *
 *     CryptoIR
 *     QuantumCryptoIR
 *     HashIR
 *     SecurityQuantumIR
 *
 * may be introduced merely to support this grammar.
 */


/* ============================================================================
 * OPTIMIZATION CONTRACT
 * ========================================================================== */

/*
 * Optimizations may change implementation strategy only if mandatory
 * cryptographic semantics remain preserved.
 *
 * An optimization MUST NOT:
 *
 *     weaken a mandatory security property;
 *     violate a mandatory requirement;
 *     violate a constraint;
 *     substitute an incompatible algorithm;
 *     expose protected material;
 *     silently change canonicalization;
 *     silently change digest semantics.
 */


/* ============================================================================
 * ROUTING / SCHEDULING CONTRACT
 * ========================================================================== */

/*
 * Cryptography has no routing or scheduling responsibility.
 *
 * Routing/scheduling may select an implementation based on:
 *
 *     capabilities
 *     resources
 *     topology
 *     performance
 *     resilience
 *     deployment policy
 *
 * while preserving semantic cryptographic requirements.
 */


/* ============================================================================
 * HARDWARE CONTRACT
 * ========================================================================== */

/*
 * Hardware realization may involve:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     HSM
 *     TPM
 *     secure element
 *     enclave
 *     QPU-associated classical processor
 *     future device
 *
 * None is hard-coded here.
 */


/* ============================================================================
 * RUNTIME CONTRACT
 * ========================================================================== */

/*
 * Runtime may resolve:
 *
 *     key handles
 *     credential handles
 *     provider references
 *     implementation references
 *     capabilities
 *     resource availability
 *
 * Parsing performs none of these operations.
 */


/* ============================================================================
 * DETERMINISM CONTRACT
 * ========================================================================== */

/*
 * Parsing must depend only on:
 *
 *     source
 *     language version
 *     lexical vocabulary
 *     grammar
 *     selected dialect
 *
 * Parsing must NOT depend on:
 *
 *     hardware
 *     available keys
 *     provider availability
 *     filesystem state
 *     network state
 *     environment variables
 *     wall-clock time
 *     randomness
 *     runtime state
 */


/* ============================================================================
 * SECURITY CONTRACT
 * ========================================================================== */

/*
 * This grammar performs no:
 *
 *     filesystem access
 *     network access
 *     provider loading
 *     key-store access
 *     credential access
 *     command execution
 *     cryptographic execution
 *     hardware discovery
 *
 * It contains no embedded Rust actions.
 *
 * Therefore no unsafe Rust is introduced by this grammar.
 */


/* ============================================================================
 * COMPATIBILITY CONTRACT
 * ========================================================================== */

/*
 * This rewrite deliberately avoids silently introducing the previously
 * missing cryptography-specific lexer vocabulary.
 *
 * Therefore the canonical lexical contract remains stable.
 *
 * If the language specification later promotes a cryptography word such as:
 *
 *     cryptography
 *     hash
 *     algorithm
 *     key
 *     protocol
 *
 * into a reserved keyword, the change MUST proceed through:
 *
 *     grammar/specification/
 *          |
 *          v
 *     grammar/lexer/keywords.g4
 *          |
 *          v
 *     grammar/lexer/tokens.g4
 *          |
 *          v
 *     grammar/antlr/ZamaniLexer.g4
 *          |
 *          v
 *     parser composition
 *          |
 *          v
 *     compatibility tests
 *
 * No parser grammar may privately invent such tokens.
 */


/* ============================================================================
 * VALIDATION CONTRACT
 * ========================================================================== */

/*
 * grammar/validation/ must verify:
 *
 *   - tokenVocab is ZamaniLexer;
 *   - no lexer rules exist here;
 *   - no Rust actions exist;
 *   - no semantic predicates exist;
 *   - no finite algorithm catalogue exists;
 *   - no machine-size constants exist;
 *   - no physical device assumptions exist;
 *   - no second expression grammar exists;
 *   - no second capability system exists;
 *   - no second resource system exists;
 *   - no second quantum IR exists;
 *   - hash syntax is delegated to hashes.g4;
 *   - security composition owns this grammar;
 *   - the canonical root remains Zamani.g4.
 */


/* ============================================================================
 * HARD-CODING AUDIT
 * ========================================================================== */

/*
 * Forbidden language-level limits include:
 *
 *     MAX_KEYS
 *     MAX_ALGORITHMS
 *     MAX_PRIMITIVES
 *     MAX_PROTOCOLS
 *     MAX_PARTICIPANTS
 *     MAX_CRYPTO_OPERATIONS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_MEMORY
 *     MAX_STORAGE
 *
 * No such limits are defined here.
 *
 * Program data such as:
 *
 *     let n = 1024;
 *
 * remains valid program semantics.
 *
 * A compiler implementation limit must never be converted into a language
 * grammar limit.
 */


/* ============================================================================
 * SCALABILITY TEST CONTRACT
 * ========================================================================== */

/*
 * Tests must cover:
 *
 *   - tiny cryptographic programs;
 *   - many cryptographic operations;
 *   - deeply nested expressions;
 *   - large requirement sets;
 *   - large property sets;
 *   - large metadata sets;
 *   - deeply qualified names;
 *   - large source files;
 *   - classical + cryptography;
 *   - quantum + cryptography;
 *   - hybrid + cryptography;
 *   - HDL + cryptography;
 *   - distributed + cryptography;
 *   - AI/data + cryptography;
 *   - networking + cryptography;
 *   - future dialect references.
 *
 * No test may define an artificial maximum.
 */


/* ============================================================================
 * POSITIVE TEST CONTRACT
 * ========================================================================== */

/*
 * Representative valid forms:
 *
 *     apply cryptography::hash(data);
 *
 *     apply cryptography::sign(message, signing_key);
 *
 *     apply cryptography::verify(message, signature, verification_key);
 *
 *     apply cryptography::encrypt(data, key);
 *
 *     apply cryptography::decrypt(ciphertext, key);
 *
 *     apply cryptography::future::operation(value);
 *
 *     contract cryptography::integrity {
 *         requires capability("cryptography.compute");
 *         prefer capability("constant_time");
 *     }
 *
 *     cryptography::algorithm::future {
 *         algorithm: future::cryptography::algorithm;
 *         property: cryptography::collision_resistance;
 *     }
 *
 * These names are illustrative semantic names rather than a closed catalogue.
 */


/* ============================================================================
 * NEGATIVE / SEMANTIC TEST CONTRACT
 * ========================================================================== */

/*
 * Tests must distinguish syntax errors from semantic errors.
 *
 * Semantic failures include:
 *
 *     unresolved algorithm
 *     unavailable provider
 *     unavailable implementation
 *     invalid key reference
 *     unsupported property
 *     unsatisfied capability
 *     insufficient resources
 *     conflicting constraints
 *     prohibited secret-material use
 *     incompatible target capability
 *
 * The parser should not attempt to diagnose these as lexical failures.
 */


/* ============================================================================
 * CROSS-DOMAIN CONTRACT
 * ========================================================================== */

/*
 * Cryptographic intent must be composable with:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     distributed
 *     networking
 *     AI
 *     data
 *     embedded
 *     accelerator
 *
 * The cryptography grammar remains domain-neutral.
 */


/* ============================================================================
 * COMPLETION CRITERIA
 * ========================================================================== */

/*
 * This file is complete when:
 *
 * [x] Cryptography has one grammar owner.
 * [x] The canonical lexer remains ZamaniLexer.
 * [x] Missing cryptography-specific lexer tokens are not silently assumed.
 * [x] Algorithms are open-world.
 * [x] Providers are open-world.
 * [x] Implementations are open-world.
 * [x] Cryptographic properties are open-world.
 * [x] Requirements are distinct from constraints.
 * [x] Constraints are distinct from preferences.
 * [x] No machine capacity is hard-coded.
 * [x] No physical device identity is required.
 * [x] No secret-material literal category is created.
 * [x] General expressions remain owned by the expression grammar.
 * [x] General names remain owned by the core/name grammar.
 * [x] Cryptography does not create a second IR.
 * [x] Quantum-related cryptographic semantics use quantum::ir downstream.
 * [x] Hashing has a dedicated delegation boundary.
 * [x] Classical integration is defined.
 * [x] Quantum integration is defined.
 * [x] HDL integration is defined.
 * [x] Distributed integration is defined.
 * [x] AI/data integration is defined.
 * [x] Networking integration is defined.
 * [x] Runtime responsibilities are separated.
 * [x] Resource/capability responsibilities are separated.
 * [x] Determinism is defined.
 * [x] Compatibility is defined.
 * [x] Safe Rust 1.97/1.97.1 integration is defined.
 * [x] Validation requirements are defined.
 * [x] Scalability requirements are defined.
 * [x] Positive tests are defined.
 * [x] Negative tests are defined.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL INVARIANT
 * ============================================================================
 *
 * Cryptography is a semantic capability of Zamani.
 *
 * It is not:
 *
 *     a fixed algorithm dictionary;
 *     a hardware description;
 *     a provider registry;
 *     a key store;
 *     a runtime;
 *     a quantum IR;
 *     a compiler backend.
 *
 * The architecture therefore remains:
 *
 *     Program Once
 *          ->
 *     Compile Once
 *          ->
 *     Run Everywhere
 *          ->
 *     Run Anywhere
 *          ->
 *     Run Forever
 *
 * subject to program semantics, security requirements, available capabilities,
 * available resources, and target realization.
 *
 * ============================================================================
 */
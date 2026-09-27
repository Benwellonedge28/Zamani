/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/security/cryptography.g4
 *
 * GRAMMAR
 * -------
 * Cryptography
 *
 * STATUS
 * ------
 * PRODUCTION SECURITY-DOMAIN GRAMMAR
 *
 * PURPOSE
 * -------
 * This file defines the parser-level syntax owned by Zamani's cryptography
 * domain.
 *
 * The grammar describes portable cryptographic INTENT and REFERENCES.
 *
 * It does not implement cryptography.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     grammar/antlr/ZamaniLexer.g4
 *          |
 *          v
 *     grammar/antlr/ZamaniParser.g4
 *          |
 *          v
 *     security composition
 *          |
 *          +-----------------------------+
 *          |                             |
 *          v                             v
 *     Cryptography                  other security domains
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic security analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +-----------------------------+
 *          |                             |
 *          v                             v
 *     classical lowering             quantum lowering
 *                                        |
 *                                        v
 *                                   quantum::ir
 *                                        |
 *                                        v
 *                              optimization / routing /
 *                              scheduling / resilience /
 *                              ZQN / HAL / target
 *
 * Cryptography never creates a second quantum IR.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - cryptographic declarations;
 *   - cryptographic operation intent;
 *   - cryptographic algorithm references;
 *   - primitive references;
 *   - key references;
 *   - key roles;
 *   - cryptographic purposes;
 *   - cryptographic properties;
 *   - protocol references and protocol intent;
 *   - cryptographic requirements;
 *   - cryptographic constraints;
 *   - cryptographic preferences;
 *   - provider/implementation references as opaque names;
 *   - cryptographic metadata attachments;
 *   - cryptographic source-level composition.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - identity;
 *   - authentication;
 *   - authorization;
 *   - permissions;
 *   - policy containers;
 *   - trust;
 *   - certificates as trust evidence;
 *   - secret storage;
 *   - passwords;
 *   - private-key material;
 *   - plaintext secret values;
 *   - key generation;
 *   - key rotation implementation;
 *   - key destruction implementation;
 *   - random-number generation;
 *   - HSM/TPM/secure-element implementation;
 *   - hardware discovery;
 *   - provider discovery;
 *   - algorithm implementation;
 *   - cryptographic execution;
 *   - timing enforcement;
 *   - side-channel enforcement;
 *   - runtime enforcement;
 *   - resource discovery;
 *   - capability discovery;
 *   - quantum error correction;
 *   - ZQN;
 *   - routing;
 *   - scheduling;
 *   - optimization;
 *   - target selection;
 *   - deployment.
 *
 * Ownership of adjacent concepts remains:
 *
 *     security/identifiers.g4
 *         identity and principal syntax
 *
 *     security/capabilities.g4
 *         security capability syntax
 *
 *     security/permissions.g4
 *         permission syntax
 *
 *     security/authorization.g4
 *         authorization syntax
 *
 *     security/policies.g4
 *         policy syntax
 *
 *     security/secrets.g4
 *         secret references and lifecycle intent
 *
 *     security/privacy.g4
 *         privacy syntax
 *
 *     security/trust.g4
 *         trust syntax
 *
 *     security/security-constraints.g4
 *         general security constraints
 *
 *     security/security.g4
 *         security composition
 *
 * Cryptography may REFER to concepts owned by those grammars, but must not
 * redefine them.
 *
 * ============================================================================
 * CRITICAL LEXICAL CORRECTION
 * ============================================================================
 *
 * The previous implementation attempted to consume cryptography-specific
 * lexer tokens such as:
 *
 *     CRYPTOGRAPHY
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
 *
 * Those names are not currently part of the canonical lexical vocabulary.
 *
 * This grammar therefore does NOT silently assume their existence.
 *
 * The production integration contract is:
 *
 *     grammar/security/cryptography.g4
 *             |
 *             v
 *     grammar/antlr/ZamaniParser.g4
 *             |
 *             v
 *     canonical security vocabulary
 *
 * Cryptography-specific reserved words MUST be added exactly once to the
 * canonical lexical hierarchy before this grammar is enabled by the complete
 * parser composition.
 *
 * The required lexical additions are listed at the end of this file.
 *
 * They MUST NOT be duplicated in this parser grammar.
 *
 * ============================================================================
 * OPEN-WORLD CRYPTOGRAPHY
 * ============================================================================
 *
 * Cryptographic algorithms, primitives, schemes, protocols, providers and
 * implementations are OPEN-WORLD semantic names.
 *
 * Do NOT encode a finite list such as:
 *
 *     AES
 *     ChaCha20
 *     RSA
 *     ECC
 *     SHA256
 *     SHA3
 *     Ed25519
 *     ML-KEM
 *     ML-DSA
 *     SLH-DSA
 *     ...
 *
 * as parser alternatives.
 *
 * Such names are values in the language's semantic namespace.
 *
 * This allows future algorithms and implementations to be represented without
 * modifying the grammar.
 *
 * Example:
 *
 *     algorithm "future.namespace.algorithm"
 *
 * is structurally equivalent to:
 *
 *     algorithm "existing.namespace.algorithm"
 *
 * The semantic layer determines whether a referenced algorithm exists,
 * satisfies the requested properties, is approved by policy, and can be
 * realized on the selected target.
 *
 * ============================================================================
 * SECRET-MATERIAL INVARIANT
 * ============================================================================
 *
 * This grammar MUST NEVER provide syntax whose semantic purpose is embedding
 * secret/private cryptographic material directly in ordinary source.
 *
 * Forbidden conceptual forms include:
 *
 *     private_key = "..."
 *     secret_key = "..."
 *     password = "..."
 *     seed = "..."
 *
 * A STRING is not inherently a secret, so the parser cannot reject every
 * string globally.
 *
 * Instead:
 *
 *     cryptographic key references
 *     credential references
 *     secret references
 *     key-store references
 *
 * must remain references.
 *
 * The semantic/security layer is responsible for preventing a reference from
 * being misused as embedded secret material.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Cryptographic source must remain portable across:
 *
 *     embedded systems
 *     CPUs
 *     multicore systems
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     QPUs
 *     simulators
 *     clusters
 *     distributed systems
 *     cloud systems
 *     future computational substrates
 *
 * This grammar therefore contains no universal limits on:
 *
 *     keys
 *     algorithms
 *     operations
 *     participants
 *     protocols
 *     security properties
 *     cryptographic objects
 *     devices
 *     nodes
 *     CPUs
 *     GPUs
 *     FPGAs
 *     QPUs
 *     memory
 *     storage
 *     threads
 *     qubits
 *
 * No:
 *
 *     MAX_KEYS
 *     MAX_ALGORITHMS
 *     MAX_CRYPTO_OPERATIONS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_MEMORY
 *
 * may become language-level constraints.
 *
 * Actual resource limitations belong to semantic resource analysis, target
 * capabilities, compilation, scheduling, deployment and runtime.
 *
 * ============================================================================
 * REQUIREMENT / CONSTRAINT / PREFERENCE
 * ============================================================================
 *
 * These are intentionally distinct semantic categories.
 *
 * REQUIREMENT:
 *
 *     must be satisfied.
 *
 * CONSTRAINT:
 *
 *     limits an otherwise valid realization.
 *
 * PREFERENCE:
 *
 *     desirable but not necessarily mandatory.
 *
 * A preference MUST NOT be silently promoted to a requirement.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * This grammar must not require:
 *
 *     CPU identity
 *     GPU identity
 *     FPGA identity
 *     QPU identity
 *     HSM identity
 *     TPM identity
 *     memory-bank identity
 *     physical device index
 *     network-node index
 *
 * Hardware-specific realization belongs downstream.
 *
 * ============================================================================
 * ANTLR CONTRACT
 * ============================================================================
 */

parser grammar Cryptography;

options {
    tokenVocab = ZamaniLexer;
}


/* ========================================================================= */
/* PUBLIC ENTRY POINTS                                                      */
/* ========================================================================= */

/*
 * Standalone cryptography grammar entry point.
 *
 * This is useful for grammar conformance tests.
 *
 * The complete Zamani parser normally enters cryptography through the security
 * composition root.
 */
cryptographyFile
    : cryptographyEntry* EOF
    ;

cryptographyEntry
    : cryptographicDeclaration
    | cryptographicRequirementDeclaration
    | cryptographicConstraintDeclaration
    | cryptographicPreferenceDeclaration
    | cryptographicOperation
    ;


/* ========================================================================= */
/* DECLARATIONS                                                              */
/* ========================================================================= */

/*
 * A cryptography declaration names a cryptographic semantic scope/object.
 *
 * The declaration name is an ordinary Zamani identifier.
 *
 * The cryptographic meaning is determined downstream.
 */
cryptographicDeclaration
    : CRYPTOGRAPHY identifier cryptographicBody? SEMI?
    ;

cryptographicBody
    : LBRACE cryptographicMember* RBRACE
    ;

cryptographicMember
    : cryptographicAlgorithmDeclaration
    | cryptographicPrimitiveDeclaration
    | cryptographicKeyDeclaration
    | cryptographicProtocolDeclaration
    | cryptographicRequirementMember
    | cryptographicConstraintMember
    | cryptographicPreferenceMember
    | cryptographicOperation
    | cryptographicMetadata
    ;


/* ========================================================================= */
/* ALGORITHM                                                                 */
/* ========================================================================= */

cryptographicAlgorithmDeclaration
    : ALGORITHM cryptographicReferenceName cryptographicDescriptor?
    ;

cryptographicDescriptor
    : LBRACE cryptographicDescriptorMember* RBRACE
    ;

cryptographicDescriptorMember
    : propertyClause
    | purposeClause
    | requirementClause
    | constraintClause
    | preferenceClause
    | parameterClause
    | implementationReference
    | providerReference
    | metadataClause
    ;


/*
 * Open-world algorithm reference.
 *
 * No algorithm enumeration is permitted here.
 */
algorithmReference
    : cryptographicReferenceName
    ;

algorithmReferenceList
    : algorithmReference (COMMA algorithmReference)*
    ;


/* ========================================================================= */
/* PRIMITIVE                                                                 */
/* ========================================================================= */

cryptographicPrimitiveDeclaration
    : PRIMITIVE cryptographicReferenceName cryptographicDescriptor?
    ;

primitiveReference
    : cryptographicReferenceName
    ;

primitiveReferenceList
    : primitiveReference (COMMA primitiveReference)*
    ;


/* ========================================================================= */
/* KEY DECLARATIONS                                                         */
/* ========================================================================= */

cryptographicKeyDeclaration
    : KEY identifier keyDescriptor?
    ;

keyDescriptor
    : LBRACE keyMember* RBRACE
    ;

keyMember
    : keyRoleClause
    | keyReferenceClause
    | keyAlgorithmClause
    | keyPurposeClause
    | keyPropertyClause
    | keyRequirementClause
    | keyConstraintClause
    | keyPreferenceClause
    | keyStorageReference
    | keyLifecycleIntent
    | cryptographicMetadata
    ;


/*
 * The name after KEY is a symbolic identifier.
 *
 * It does not represent secret material.
 */
keyReferenceClause
    : KEY_REFERENCE referenceValue
    ;

keyAlgorithmClause
    : ALGORITHM algorithmReference
    ;

keyPurposeClause
    : PURPOSE expression
    ;

keyPropertyClause
    : PROPERTY expression
    ;

keyRequirementClause
    : REQUIRE requirementExpression
    ;

keyConstraintClause
    : CONSTRAIN constraintExpression
    ;

keyPreferenceClause
    : PREFER preferenceExpression
    ;

keyStorageReference
    : STORAGE cryptographicReferenceName
    ;

keyLifecycleIntent
    : LIFECYCLE expression
    ;

keyRoleClause
    : ROLE expression
    ;


/* ========================================================================= */
/* KEY ROLE                                                                  */
/* ========================================================================= */

keyRoleDeclaration
    : KEY_ROLE identifier keyRoleBody?
    ;

keyRoleBody
    : LBRACE keyRoleMember* RBRACE
    ;

keyRoleMember
    : purposeClause
    | propertyClause
    | requirementClause
    | constraintClause
    | preferenceClause
    | cryptographicMetadata
    ;

keyRoleReference
    : cryptographicReferenceName
    ;


/* ========================================================================= */
/* PROTOCOL                                                                  */
/* ========================================================================= */

cryptographicProtocolDeclaration
    : PROTOCOL cryptographicReferenceName protocolDescriptor?
    ;

protocolDescriptor
    : LBRACE protocolMember* RBRACE
    ;

protocolMember
    : algorithmClause
    | primitiveClause
    | keyReferenceClause
    | purposeClause
    | propertyClause
    | requirementClause
    | constraintClause
    | preferenceClause
    | participantClause
    | parameterClause
    | cryptographicMetadata
    ;

algorithmClause
    : ALGORITHM algorithmReference
    ;

primitiveClause
    : PRIMITIVE primitiveReference
    ;

participantClause
    : PARTICIPANT expression
    ;

participantList
    : participantExpression (COMMA participantExpression)*
    ;

participantExpression
    : expression
    ;


/* ========================================================================= */
/* CRYPTOGRAPHIC OPERATIONS                                                  */
/* ========================================================================= */

/*
 * A cryptographic operation is intentionally generic.
 *
 * This avoids turning the grammar into a finite catalogue of algorithms.
 *
 * Conceptual examples:
 *
 *     cryptographic_operation signing with key signing_key
 *     cryptographic_operation hashing with algorithm digest
 *     cryptographic_operation encrypting with key session_key
 *
 * The exact semantic operation vocabulary is extensible through the operation
 * name and descriptor rather than a closed grammar enumeration.
 */
cryptographicOperation
    : CRYPTOGRAPHIC_OPERATION cryptographicOperationName
      cryptographicOperationArguments?
      cryptographicOperationBody?
      SEMI?
    ;

cryptographicOperationName
    : identifier
    ;

cryptographicOperationArguments
    : LPAREN argumentList? RPAREN
    ;

argumentList
    : expression (COMMA expression)*
    ;

cryptographicOperationBody
    : LBRACE cryptographicOperationMember* RBRACE
    ;

cryptographicOperationMember
    : algorithmClause
    | primitiveClause
    | keyReferenceClause
    | purposeClause
    | propertyClause
    | requirementClause
    | constraintClause
    | preferenceClause
    | parameterClause
    | implementationReference
    | providerReference
    | cryptographicMetadata
    ;


/* ========================================================================= */
/* PURPOSES                                                                  */
/* ========================================================================= */

purposeClause
    : PURPOSE expression
    ;

purposeList
    : purposeExpression (COMMA purposeExpression)*
    ;

purposeExpression
    : expression
    ;


/* ========================================================================= */
/* PROPERTIES                                                                */
/* ========================================================================= */

propertyClause
    : PROPERTY expression
    ;

propertyList
    : propertyExpression (COMMA propertyExpression)*
    ;

propertyExpression
    : expression
    ;


/* ========================================================================= */
/* PARAMETERS                                                                */
/* ========================================================================= */

parameterClause
    : PARAMETER identifier ASSIGN expression
    ;

parameterList
    : parameterClause (COMMA parameterClause)*
    ;


/* ========================================================================= */
/* IMPLEMENTATION / PROVIDER REFERENCES                                     */
/* ========================================================================= */

/*
 * These are opaque semantic references.
 *
 * They do not cause provider discovery or implementation loading.
 */
implementationReference
    : IMPLEMENTATION cryptographicReferenceName
    ;

providerReference
    : PROVIDER cryptographicReferenceName
    ;


/* ========================================================================= */
/* REQUIREMENTS                                                              */
/* ========================================================================= */

cryptographicRequirementDeclaration
    : REQUIREMENT identifier requirementBody?
    ;

requirementBody
    : LBRACE requirementMember* RBRACE
    ;

requirementMember
    : algorithmClause
    | primitiveClause
    | purposeClause
    | propertyClause
    | requirementClause
    | constraintClause
    | preferenceClause
    | keyRequirementClause
    | protocolReferenceClause
    | parameterClause
    | cryptographicMetadata
    ;

requirementClause
    : REQUIRE requirementExpression
    ;

requirementExpression
    : expression
    ;

keyRequirementClause
    : REQUIRE requirementExpression
    ;

protocolReferenceClause
    : PROTOCOL cryptographicReferenceName
    ;


/* ========================================================================= */
/* CONSTRAINTS                                                               */
/* ========================================================================= */

cryptographicConstraintDeclaration
    : CONSTRAINT identifier constraintBody?
    ;

constraintBody
    : LBRACE constraintMember* RBRACE
    ;

constraintMember
    : algorithmClause
    | primitiveClause
    | purposeClause
    | propertyClause
    | requirementClause
    | constraintClause
    | preferenceClause
    | parameterClause
    | protocolReferenceClause
    | cryptographicMetadata
    ;

constraintClause
    : CONSTRAIN constraintExpression
    ;

constraintExpression
    : expression
    ;


/* ========================================================================= */
/* PREFERENCES                                                               */
/* ========================================================================= */

cryptographicPreferenceDeclaration
    : PREFERENCE identifier preferenceBody?
    ;

preferenceBody
    : LBRACE preferenceMember* RBRACE
    ;

preferenceMember
    : algorithmClause
    | primitiveClause
    | purposeClause
    | propertyClause
    | requirementClause
    | constraintClause
    | preferenceClause
    | implementationReference
    | providerReference
    | parameterClause
    | protocolReferenceClause
    | cryptographicMetadata
    ;

preferenceClause
    : PREFER preferenceExpression
    ;

preferenceExpression
    : expression
    ;


/* ========================================================================= */
/* GENERIC CRYPTOGRAPHIC REFERENCES                                         */
/* ========================================================================= */

/*
 * Cryptographic references are semantic names.
 *
 * They may represent:
 *
 *     algorithm
 *     primitive
 *     key handle
 *     credential handle
 *     provider
 *     implementation
 *     protocol
 *     cryptographic service
 *     external cryptographic object
 *     future cryptographic abstraction
 *
 * They are NOT automatically resolved by parsing.
 */
cryptographicReferenceName
    : qualifiedName
    ;


/*
 * Reference values are deliberately restricted to names or ordinary source
 * strings used as identifiers/handles.
 *
 * Security semantics determine whether a string is an allowed external
 * reference. Secret material must never be interpreted as valid key material
 * merely because it is lexically a STRING.
 */
referenceValue
    : qualifiedName
    | stringLiteral
    ;


/* ========================================================================= */
/* METADATA                                                                  */
/* ========================================================================= */

cryptographicMetadata
    : METADATA metadataBlock
    ;

metadataBlock
    : LBRACE metadataEntry* RBRACE
    ;

metadataEntry
    : identifier ASSIGN expression
    ;

metadataClause
    : cryptographicMetadata
    ;


/* ========================================================================= */
/* SHARED CRYPTOGRAPHIC MEMBER CONTRACTS                                    */
/* ========================================================================= */

/*
 * These façade rules intentionally delegate to the canonical Zamani
 * expression/name contracts.
 *
 * They MUST NOT be replaced by cryptography-specific identifier or expression
 * grammars.
 *
 * Expected canonical contracts:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     stringLiteral
 *     argumentList
 *
 * Ownership remains in the corresponding core/expression grammar.
 */


/* ========================================================================= */
/* COMPOSITION HELPERS                                                       */
/* ========================================================================= */

cryptographicRequirementMember
    : algorithmClause
    | primitiveClause
    | purposeClause
    | propertyClause
    | requirementClause
    | constraintClause
    | preferenceClause
    | keyRequirementClause
    | protocolReferenceClause
    | parameterClause
    | cryptographicMetadata
    ;

cryptographicConstraintMember
    : algorithmClause
    | primitiveClause
    | purposeClause
    | propertyClause
    | requirementClause
    | constraintClause
    | preferenceClause
    | parameterClause
    | protocolReferenceClause
    | cryptographicMetadata
    ;

cryptographicPreferenceMember
    : algorithmClause
    | primitiveClause
    | purposeClause
    | propertyClause
    | requirementClause
    | constraintClause
    | preferenceClause
    | implementationReference
    | providerReference
    | parameterClause
    | protocolReferenceClause
    | cryptographicMetadata
    ;


/* ========================================================================= */
/* INTEGRATION CONTRACT                                                      */
/* ========================================================================= */

/*
 * SECURITY COMPOSITION
 * --------------------
 *
 * grammar/security/security.g4 owns the security composition boundary.
 *
 * It should import/compose Cryptography rather than copying these rules.
 *
 * Intended relationship:
 *
 *     Security
 *        |
 *        +--> Identity
 *        +--> Capabilities
 *        +--> Permissions
 *        +--> Authorization
 *        +--> Policies
 *        +--> Cryptography
 *        +--> Privacy
 *        +--> Trust
 *        +--> SecurityConstraints
 *
 * Cryptography owns only its own syntax.
 *
 *
 * LEXER
 * -----
 *
 * The canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * It consumes:
 *
 *     ZamaniTokens
 *
 * Cryptography MUST use:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * and MUST NOT consume:
 *
 *     ZamaniTokens
 *
 * directly.
 *
 *
 * PARSER
 * ------
 *
 * The canonical parser composition root is:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * That parser is responsible for importing/composing security.
 *
 *
 * AST
 * ---
 *
 * Cryptography MUST lower into the domain-neutral AST owned by:
 *
 *     src/frontend/ast/
 *
 * This grammar does not prescribe Rust struct names.
 *
 * The AST contract must preserve at minimum:
 *
 *     source span
 *     declaration identity
 *     cryptographic category
 *     symbolic references
 *     arguments
 *     properties
 *     purposes
 *     requirements
 *     constraints
 *     preferences
 *     metadata
 *
 * No cryptography-specific backend AST is required.
 *
 *
 * SEMANTIC ANALYSIS
 * -----------------
 *
 * Semantic analysis owns:
 *
 *     reference resolution
 *     algorithm existence
 *     primitive validity
 *     key-reference validity
 *     purpose compatibility
 *     property interpretation
 *     requirement satisfiability
 *     constraint consistency
 *     preference ordering
 *     provider availability
 *     implementation availability
 *     policy compliance
 *     secret-material restrictions
 *     cryptographic misuse detection
 *
 * None of these are parser actions.
 *
 *
 * IR
 * --
 *
 * Cryptography MUST NOT create a competing cryptographic IR.
 *
 * Cryptographic semantics are lowered into the repository's canonical semantic
 * and IR architecture.
 *
 * Classical cryptographic computation may lower into the appropriate classical
 * representation.
 *
 * Quantum-related cryptographic computation MUST use the canonical:
 *
 *     quantum::ir
 *
 * boundary where applicable.
 *
 * Security metadata must remain traceable through lowering.
 *
 *
 * OPTIMIZATION
 * ------------
 *
 * Optimization may transform implementation details only when semantic
 * cryptographic requirements remain preserved.
 *
 * An optimization must not:
 *
 *     remove a mandatory property;
 *     weaken a mandatory requirement;
 *     violate a cryptographic constraint;
 *     substitute an incompatible primitive;
 *     silently replace a required algorithm;
 *     expose secret material.
 *
 *
 * ROUTING / SCHEDULING
 * --------------------
 *
 * Cryptography grammar has no routing or scheduling responsibility.
 *
 * Routing and scheduling may select an implementation after semantic analysis,
 * subject to preserved cryptographic requirements and target capabilities.
 *
 *
 * HARDWARE
 * --------
 *
 * HSMs, secure elements, TPMs, enclaves, CPUs, GPUs, FPGAs, ASICs, QPUs and
 * other devices are target capabilities, not grammar-level cryptographic
 * implementations.
 *
 *
 * RUNTIME
 * -------
 *
 * Runtime systems resolve:
 *
 *     references
 *     providers
 *     keys
 *     credentials
 *     cryptographic services
 *
 * subject to security policy.
 *
 * Parsing does none of this.
 *
 *
 * RUST
 * ----
 *
 * This grammar contains no Rust actions.
 *
 * Generated and handwritten Rust integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and must use safe Rust only.
 *
 * This grammar introduces no requirement for:
 *
 *     unsafe
 *     unsafe fn
 *     unsafe impl
 *     unsafe trait
 *     unsafe block
 *
 *
 * ============================================================================
 * CROSS-DOMAIN CONTRACT
 * ============================================================================
 *
 * Cryptography is intentionally usable with:
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
 *     cloud
 *     future domains
 *
 * The cryptographic grammar remains domain-neutral.
 *
 * For example, the same cryptographic requirement may apply to:
 *
 *     a classical computation;
 *     a quantum workload;
 *     a hybrid quantum/classical workflow;
 *     an HDL implementation;
 *     a distributed computation.
 *
 * The target domain is determined by surrounding semantic constructs.
 *
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * All repeated structures use zero-or-more or one-or-more recursive/list forms
 * rather than arbitrary fixed cardinalities.
 *
 * There is no universal limit on:
 *
 *     declarations
 *     algorithms
 *     primitives
 *     keys
 *     protocols
 *     participants
 *     properties
 *     requirements
 *     constraints
 *     preferences
 *     metadata entries
 *     operation arguments
 *
 * Practical parser/compiler limits remain implementation/resource limits.
 *
 * They MUST NOT become Zamani language semantics.
 *
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source
 *     language version
 *     lexical vocabulary
 *     parser grammar
 *     explicitly selected dialect
 *
 * It must not depend on:
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
 *
 * Security resolution happens after parsing.
 *
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains NO:
 *
 *     MAX_KEYS
 *     MAX_ALGORITHMS
 *     MAX_PRIMITIVES
 *     MAX_PROTOCOLS
 *     MAX_PARTICIPANTS
 *     MAX_CRYPTOGRAPHIC_OPERATIONS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_QUBITS
 *
 * It also contains no:
 *
 *     CPU0
 *     GPU0
 *     FPGA0
 *     QPU0
 *     HSM0
 *     DEVICE0
 *
 * as grammar-level physical assumptions.
 *
 * Numeric literals appearing in expressions remain ordinary program data.
 *
 *
 * ============================================================================
 * SECRET / CREDENTIAL AUDIT
 * ============================================================================
 *
 * This grammar does not define:
 *
 *     passwordLiteral
 *     privateKeyLiteral
 *     secretKeyLiteral
 *     seedLiteral
 *     credentialLiteral
 *
 * Cryptographic objects are referenced symbolically.
 *
 * Secret storage belongs to security/secrets.g4 and downstream secret
 * management.
 *
 *
 * ============================================================================
 * OPEN-WORLD AUDIT
 * ============================================================================
 *
 * No finite algorithm list exists in this file.
 *
 * Future cryptographic mechanisms therefore do not require grammar changes
 * merely because a new algorithm is introduced.
 *
 * Semantic registries, provider metadata and dialects may evolve independently
 * of this parser grammar.
 *
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Syntax diagnostics are generated by the parser infrastructure.
 *
 * Semantic diagnostics should distinguish at minimum:
 *
 *     unresolved cryptographic reference
 *     invalid cryptographic reference
 *     unavailable algorithm
 *     unavailable primitive
 *     invalid key reference
 *     invalid purpose
 *     unsatisfied requirement
 *     conflicting constraint
 *     incompatible preference
 *     unavailable provider
 *     unavailable implementation
 *     prohibited secret material
 *     policy violation
 *     unsupported target capability
 *
 * The parser must not fabricate these semantic conclusions.
 *
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * This grammar preserves the conceptual API of the previous cryptography
 * grammar:
 *
 *     cryptographicDeclaration
 *     algorithmReference
 *     primitiveReference
 *     keyClause
 *     keyReferenceClause
 *     keyRoleDeclaration
 *     purposeClause
 *     propertyClause
 *     protocolClause
 *     parameterClause
 *     requirementClause
 *     constraintClause
 *     preferenceClause
 *     referenceValue
 *     metadataClause
 *
 * Where a previous rule was structurally unsafe or depended on nonexistent
 * lexer tokens, the corrected production contract takes precedence.
 *
 * Token renames must be handled through:
 *
 *     grammar/compatibility/
 *
 * and:
 *
 *     grammar/spec/compatibility.md
 *
 *
 * ============================================================================
 * VALIDATION CONTRACT
 * ============================================================================
 *
 * grammar/validation/ must verify:
 *
 *     - parser grammar name matches Cryptography;
 *     - tokenVocab is ZamaniLexer;
 *     - no lexer rules are embedded here;
 *     - no Rust actions exist;
 *     - no semantic predicates are required;
 *     - no duplicate cryptographic ownership exists;
 *     - no finite algorithm enumeration exists;
 *     - no finite key enumeration exists;
 *     - no hardware limit exists;
 *     - no machine-size limit exists;
 *     - all shared name/expression rules resolve through canonical grammar;
 *     - security composition imports this grammar exactly once;
 *     - AST mapping exists;
 *     - semantic mapping exists;
 *     - IR mapping exists;
 *     - negative tests exist;
 *     - boundary tests exist;
 *     - scalability tests exist.
 *
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive tests must cover:
 *
 *     cryptographic declarations
 *     algorithms
 *     primitives
 *     keys
 *     key references
 *     key roles
 *     protocols
 *     participants
 *     purposes
 *     properties
 *     parameters
 *     requirements
 *     constraints
 *     preferences
 *     providers
 *     implementations
 *     metadata
 *     generic cryptographic operations
 *     qualified names
 *     cross-domain use
 *
 * Negative tests must cover:
 *
 *     malformed declarations
 *     malformed references
 *     malformed lists
 *     malformed descriptors
 *     malformed operations
 *     malformed metadata
 *     missing required syntax
 *     malformed qualified names
 *     malformed secret-reference syntax
 *
 * Security semantic tests must additionally reject attempts to treat ordinary
 * source strings as secret/private cryptographic material where the semantic
 * contract prohibits them.
 *
 *
 * Boundary tests must cover:
 *
 *     one key
 *     many keys
 *     one algorithm
 *     many algorithms
 *     deeply nested descriptors
 *     long qualified names
 *     large parameter sets
 *     large metadata sets
 *     large operation sequences
 *     large source units
 *
 * The test environment may impose practical resource limits.
 * Those limits are not language-level limits.
 *
 *
 * ============================================================================
 * POCO-REAF ACCEPTANCE
 * ============================================================================
 *
 * A cryptographic program remains semantically portable when:
 *
 *     source cryptographic intent
 *
 * is unchanged while target realization changes between:
 *
 *     tiny embedded target
 *     CPU
 *     multicore CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     simulator
 *     cluster
 *     cloud
 *     future target
 *
 * The grammar must not change solely because the target changes.
 *
 *
 * ============================================================================
 * REQUIRED CANONICAL LEXER INTEGRATION
 * ============================================================================
 *
 * Before this grammar is enabled in the canonical parser composition, the
 * canonical lexical hierarchy must provide these reserved tokens exactly once:
 *
 *     CRYPTOGRAPHY
 *     ALGORITHM
 *     PRIMITIVE
 *     KEY
 *     KEY_REFERENCE
 *     KEY_ROLE
 *     PROTOCOL
 *     PARTICIPANT
 *     PARAMETER
 *     PROVIDER
 *     IMPLEMENTATION
 *     LIFECYCLE
 *     PREFERENCE
 *     REQUIREMENT
 *     PURPOSE
 *     METADATA
 *     STORAGE
 *     CRYPTOGRAPHIC_OPERATION
 *
 * Existing generic tokens such as:
 *
 *     PROPERTY
 *     CONSTRAINT
 *     REQUIRE
 *     CONSTRAIN
 *     PREFER
 *     ROLE
 *
 * should be reused when they already exist.
 *
 * These tokens belong in:
 *
 *     grammar/lexer/keywords.g4
 *
 * and consequently enter:
 *
 *     grammar/lexer/tokens.g4
 *          |
 *          v
 *     grammar/antlr/ZamaniLexer.g4
 *
 * They MUST NOT be defined here.
 *
 *
 * ============================================================================
 * REQUIRED COMPOSITION INTEGRATION
 * ============================================================================
 *
 * grammar/security/security.g4
 *
 * must compose:
 *
 *     Cryptography
 *
 * rather than copying:
 *
 *     cryptographicDeclaration
 *     cryptographicOperation
 *     cryptographicKeyDeclaration
 *     cryptographicAlgorithmDeclaration
 *     cryptographicProtocolDeclaration
 *     cryptographicRequirementDeclaration
 *     cryptographicConstraintDeclaration
 *     cryptographicPreferenceDeclaration
 *
 * The composition root remains the sole security-wide integration boundary.
 *
 *
 * ============================================================================
 * AST COMPLETION CONTRACT
 * ============================================================================
 *
 * The corresponding AST implementation must preserve enough structure that
 * this file never needs to be modified merely because a new backend is added.
 *
 * Minimum semantic fields:
 *
 *     category
 *     name/reference
 *     arguments
 *     parameters
 *     keys/references
 *     algorithms/references
 *     primitives/references
 *     purposes
 *     properties
 *     requirements
 *     constraints
 *     preferences
 *     metadata
 *     source span
 *
 * Backend-specific fields do not belong in this grammar.
 *
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * This file defines:
 *
 *     WHAT cryptographic intent can be expressed.
 *
 * It does not define:
 *
 *     WHICH cryptographic implementation executes it.
 *
 * Therefore:
 *
 *     cryptographic syntax
 *          !=
 *     cryptographic implementation
 *
 *     algorithm reference
 *          !=
 *     algorithm implementation
 *
 *     key reference
 *          !=
 *     secret material
 *
 *     provider reference
 *          !=
 *     provider discovery
 *
 *     security requirement
 *          !=
 *     hardware requirement
 *
 *     quantum cryptography syntax
 *          !=
 *     quantum::ir implementation
 *
 * The permanent pipeline remains:
 *
 *     Source
 *       ->
 *     Lexer
 *       ->
 *     Parser
 *       ->
 *     Domain-neutral AST
 *       ->
 *     Semantic Security Analysis
 *       ->
 *     Canonical IR
 *       ->
 *     Optimization
 *       ->
 *     Routing / Scheduling / Resilience / ZQN
 *       ->
 *     HAL
 *       ->
 *     Target realization
 *
 * subject to actual program semantics and available resources, without an
 * artificial language-level hardware ceiling.
 *
 * ============================================================================
 */
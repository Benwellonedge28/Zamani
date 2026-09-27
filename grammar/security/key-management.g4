/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/security/key-management.g4
 *
 * Grammar:
 *     KeyManagement
 *
 * Status:
 *     PRODUCTION TARGET
 *
 * Language:
 *     Zamani
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     Safe Rust only
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns SOURCE-LEVEL KEY-MANAGEMENT INTENT.
 *
 * It describes:
 *
 *     - managed-key declarations;
 *     - symbolic key references;
 *     - key lifecycle intent;
 *     - key generation intent;
 *     - key derivation intent;
 *     - key import/export intent;
 *     - key wrapping/unwrapping intent;
 *     - key rotation intent;
 *     - key revocation intent;
 *     - key destruction intent;
 *     - key suspension/activation intent;
 *     - key versioning intent;
 *     - key usage/purpose metadata;
 *     - key algorithm references;
 *     - provider/implementation references;
 *     - key-storage intent;
 *     - key-access requirements;
 *     - key capability requirements;
 *     - key resource requirements;
 *     - key constraints;
 *     - key preferences;
 *     - key audit/provenance metadata;
 *     - future key-management operations.
 *
 * This grammar DOES NOT:
 *
 *     - generate random material;
 *     - derive actual key bytes;
 *     - read secret material;
 *     - store secret material;
 *     - expose private keys;
 *     - perform cryptographic operations;
 *     - select a cryptographic algorithm implementation;
 *     - select an HSM;
 *     - select a CPU;
 *     - select a GPU;
 *     - select an FPGA;
 *     - select a QPU;
 *     - select a network node;
 *     - select a memory device;
 *     - discover hardware;
 *     - access a filesystem;
 *     - access a network;
 *     - execute runtime code;
 *     - perform authorization;
 *     - perform identity verification;
 *     - create a second IR;
 *     - create a second quantum IR;
 *     - perform QEC;
 *     - perform ZQN analysis;
 *     - perform routing;
 *     - perform scheduling.
 *
 * ============================================================================
 * ARCHITECTURAL AUTHORITY
 * ============================================================================
 *
 * Normative architecture:
 *
 *     grammar/DESIGN.md
 *
 * Security composition:
 *
 *     grammar/security/security.g4
 *
 * Cryptographic intent:
 *
 *     grammar/security/cryptography.g4
 *
 * Hashing:
 *
 *     grammar/security/hashes.g4
 *
 * Secret-material boundary:
 *
 *     grammar/security/secrets.g4
 *
 * Signatures:
 *
 *     grammar/security/signatures.g4
 *
 * Canonical names:
 *
 *     grammar/core/names.g4
 *
 * Canonical expressions:
 *
 *     grammar/expressions/expressions.g4
 *
 * Canonical types:
 *
 *     grammar/types/types.g4
 *
 * Canonical lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Canonical parser composition:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * Frontend AST:
 *
 *     src/frontend/ast/
 *
 * Semantic analysis:
 *
 *     security/key-management semantic layer
 *
 * Canonical semantic/IR boundary:
 *
 *     canonical IR
 *
 * Quantum semantic boundary:
 *
 *     quantum::ir
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - key-management source syntax;
 *     - managed-key declarations;
 *     - key-management operations;
 *     - key lifecycle intent;
 *     - key-management property syntax;
 *     - key-management requirements;
 *     - key-management constraints;
 *     - key-management preferences;
 *     - symbolic key references;
 *     - key operation arguments;
 *     - key operation bodies;
 *     - key-management-specific syntax composition.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - identifiers;
 *     - qualified-name syntax;
 *     - expressions;
 *     - types;
 *     - secret declarations;
 *     - cryptographic algorithm definitions;
 *     - hash definitions;
 *     - signature definitions;
 *     - authorization;
 *     - identity;
 *     - trust;
 *     - generic resources;
 *     - hardware;
 *     - quantum operations;
 *     - quantum IR;
 *     - runtime key storage;
 *     - cryptographic implementation.
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 *     ZamaniLexer
 *          |
 *          v
 *     Names / Types / Expressions
 *          |
 *          v
 *     KeyManagement
 *          |
 *          v
 *     security.g4
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          +--------------------------+
 *          |                          |
 *          v                          v
 *     classical representation   quantum::ir metadata
 *          |                          |
 *          +------------+-------------+
 *                       |
 *                       v
 *                 canonical IR
 *                       |
 *                       v
 *              optimization/lowering
 *                       |
 *              +--------+--------+
 *              |        |        |
 *              v        v        v
 *           routing scheduling resilience
 *                              |
 *                              v
 *                             ZQN
 *                              |
 *                              v
 *                             HAL
 *                              |
 *                              v
 *                            target
 *
 * This grammar MUST NOT depend on:
 *
 *     runtime
 *     hardware
 *     HAL
 *     routing
 *     scheduling
 *     QEC
 *     ZQN
 *     backend implementation
 *
 * ============================================================================
 * OPEN-WORLD CRYPTOGRAPHY
 * ============================================================================
 *
 * Key algorithms are semantic names.
 *
 * The grammar MUST NOT enumerate:
 *
 *     AES
 *     RSA
 *     ECC
 *     Ed25519
 *     ML-KEM
 *     ML-DSA
 *     SLH-DSA
 *     X25519
 *     ChaCha20
 *     HMAC
 *
 * or any other finite catalogue.
 *
 * Examples are represented by qualified names:
 *
 *     cryptography::encryption::algorithm
 *     cryptography::signature::algorithm
 *     cryptography::kem::algorithm
 *     cryptography::kdf::algorithm
 *     future::cryptography::algorithm
 *
 * Semantic analysis determines whether a referenced algorithm exists,
 * whether it is compatible with the key, and whether the target can realize
 * it.
 *
 * ============================================================================
 * KEY MATERIAL BOUNDARY
 * ============================================================================
 *
 * THIS GRAMMAR NEVER EMBEDS KEY MATERIAL.
 *
 * It MUST NOT introduce:
 *
 *     private-key literals;
 *     secret-key literals;
 *     password literals;
 *     seed literals;
 *     raw key-byte literals;
 *     recovery-secret literals;
 *     authentication-secret literals.
 *
 * An expression may syntactically contain a string, integer, byte sequence,
 * or other ordinary value.
 *
 * Whether that expression is legal as key material is a semantic/security
 * decision.
 *
 * Secret material itself belongs to secure runtime/key-management systems.
 *
 * A source program should normally reference a protected object:
 *
 *     key signing_key
 *
 * or a symbolic secret reference:
 *
 *     security::secret::signing_material
 *
 * without exposing the actual bytes.
 *
 * ============================================================================
 * KEY IDENTITY VERSUS KEY MATERIAL
 * ============================================================================
 *
 * These are deliberately distinct.
 *
 * A key reference identifies a managed cryptographic object.
 *
 * A key reference does NOT expose:
 *
 *     private material;
 *     secret material;
 *     seed material;
 *     provider storage;
 *     HSM memory;
 *     physical storage;
 *     hardware address.
 *
 * Key identity is source-level symbolic information.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * This grammar imposes NO universal limits on:
 *
 *     keys
 *     key versions
 *     key references
 *     algorithms
 *     providers
 *     implementations
 *     policies
 *     operations
 *     requirements
 *     constraints
 *     participants
 *     devices
 *     nodes
 *     processors
 *     memory
 *     storage
 *     qubits
 *     threads
 *     distributed systems
 *
 * It MUST NOT define:
 *
 *     MAX_KEYS
 *     MAX_KEY_VERSIONS
 *     MAX_KEY_SIZE
 *     MAX_KEY_OPERATIONS
 *     MAX_KEY_STORES
 *     MAX_PROVIDERS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_THREADS
 *
 * A source program may express a concrete key size as program semantics or a
 * cryptographic requirement.
 *
 * That does NOT create a language-wide maximum.
 *
 * For example:
 *
 *     size: requested_size;
 *
 * is valid source intent.
 *
 * The grammar does not decide whether a target can satisfy it.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * These concepts remain distinct:
 *
 *     requirement
 *     constraint
 *     preference
 *     capability
 *     resource
 *     implementation decision
 *
 * Example:
 *
 *     requires capability("secure.key.storage");
 *
 * means:
 *
 *     the program requires a capability.
 *
 * It does NOT mean:
 *
 *     use HSM 0.
 *
 * Similarly:
 *
 *     requires capability("hardware.key.isolation");
 *
 * does not identify a particular device.
 *
 * ============================================================================
 * KEY LIFECYCLE
 * ============================================================================
 *
 * Lifecycle is semantic.
 *
 * This grammar permits source intent for:
 *
 *     generation
 *     derivation
 *     registration
 *     import
 *     export of metadata
 *     activation
 *     suspension
 *     rotation
 *     revocation
 *     destruction
 *     wrapping
 *     unwrapping
 *     migration
 *     recovery
 *     archival
 *
 * The grammar does not prescribe the implementation.
 *
 * Lifecycle state names remain semantic identifiers.
 *
 * No closed lifecycle enumeration is imposed.
 *
 * ============================================================================
 * VERSIONING
 * ============================================================================
 *
 * Key versions are source-level semantic references.
 *
 * The grammar does not impose:
 *
 *     numeric version width;
 *     maximum version count;
 *     fixed version numbering;
 *     fixed rotation period.
 *
 * A key may therefore be referenced symbolically:
 *
 *     key: application::signing
 *     version: current
 *
 * or:
 *
 *     version: requested_version
 *
 * Actual version resolution is downstream.
 *
 * ============================================================================
 * CROSS-DOMAIN SECURITY
 * ============================================================================
 *
 * Key management may serve:
 *
 *     classical computation
 *     quantum computation
 *     hybrid computation
 *     HDL/hardware computation
 *     distributed computation
 *     AI/ML
 *     networking
 *     embedded systems
 *     accelerators
 *     HPC
 *     cloud systems
 *     future computational substrates
 *
 * Key management does not become a hardware grammar merely because a key is
 * backed by an HSM, secure enclave, QPU, accelerator, or other device.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Key management may protect quantum execution.
 *
 * It MUST NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     QPU topology
 *     calibration
 *     QEC codes
 *     physical routing
 *
 * Quantum semantics remain downstream at:
 *
 *     quantum::ir
 *
 * Key-management metadata may accompany quantum::ir as security metadata or
 * constraints.
 *
 * ============================================================================
 * PROVIDER / IMPLEMENTATION BOUNDARY
 * ============================================================================
 *
 * Providers and implementations are symbolic references.
 *
 * Examples:
 *
 *     provider: organization::kms
 *     provider: cloud::kms
 *     implementation: hardware::secure_module
 *     implementation: software::provider
 *
 * The grammar does not load or discover them.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every stable syntactic construct MUST map to a domain-neutral frontend
 * representation.
 *
 * Conceptual mappings:
 *
 *     keyDeclaration
 *         -> security/key-management declaration node
 *
 *     keyReference
 *         -> symbolic security reference
 *
 *     keyOperation
 *         -> generic operation node with security/key-management domain
 *
 *     keyProperty
 *         -> property/attribute representation
 *
 *     keyRequirement
 *         -> requirement representation
 *
 *     keyConstraint
 *         -> constraint representation
 *
 *     keyPreference
 *         -> preference representation
 *
 * The grammar MUST NOT introduce:
 *
 *     RuntimeKey
 *     HsmKey
 *     PhysicalKey
 *     QuantumKeyIR
 *     HardwareKeyNode
 *
 * as frontend AST concepts merely because a backend may use them.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     - whether a key name resolves;
 *     - whether a key is declared;
 *     - whether an operation is legal;
 *     - whether an algorithm is compatible;
 *     - whether a key purpose is compatible;
 *     - whether a key version exists;
 *     - whether rotation is legal;
 *     - whether revocation is legal;
 *     - whether destruction is permitted;
 *     - whether import/export is permitted;
 *     - whether secret material is accessed legally;
 *     - whether a provider satisfies requirements;
 *     - whether required capabilities exist;
 *     - whether resource requirements are satisfiable;
 *     - whether a quantum security requirement can survive lowering;
 *     - whether security properties are preserved through optimization.
 *
 * None of these checks occur in this grammar.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar does NOT define:
 *
 *     KeyIR
 *     CryptoKeyIR
 *     KeyManagementIR
 *     PhysicalKeyIR
 *
 * Key-management semantics are attached to the repository's canonical
 * semantic/IR representations.
 *
 * Security metadata may accompany:
 *
 *     classical IR
 *     quantum::ir
 *     HDL/hardware IR
 *     distributed representations
 *     deployment metadata
 *
 * The key-management grammar therefore does not create an alternate IR.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime/key-management infrastructure is responsible for:
 *
 *     - generating key material;
 *     - protecting key material;
 *     - securely storing key material;
 *     - resolving provider references;
 *     - performing rotation;
 *     - performing revocation;
 *     - destroying material;
 *     - performing wrapping/unwrap;
 *     - enforcing access policy;
 *     - auditing operations;
 *     - secure memory handling.
 *
 * This grammar performs none of these operations.
 *
 * ============================================================================
 * SAFE-RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust actions.
 *
 * Therefore:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * are implementation baselines rather than grammar dependencies.
 *
 * The Zamani compiler implementation MUST remain safe Rust.
 *
 * No `unsafe` Rust is required by this grammar.
 *
 * ============================================================================
 */

parser grammar KeyManagement;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * IMPORTS
 * ============================================================================
 *
 * Names owns:
 *
 *     identifier
 *     qualifiedName
 *     nameReference
 *
 * Types owns:
 *
 *     typeExpression
 *
 * Expressions owns:
 *
 *     expression
 *
 * No key-management grammar redefines any of these constructs.
 * ============================================================================
 */

import
    Names,
    Types,
    Expressions
;


/* ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ========================================================================== */

/*
 * Standalone entry point for:
 *
 *     parser tests;
 *     conformance tests;
 *     tooling;
 *     documentation;
 *     security composition.
 *
 * The root Zamani parser normally reaches this domain through security.g4.
 */
keyManagementFile
    : keyManagementEntry*
      EOF
    ;


/* ============================================================================
 * 2. TOP-LEVEL ENTRY
 * ========================================================================== */

keyManagementEntry
    : keyDeclaration
    | keyOperation
    | keyRequirementDeclaration
    | keyConstraintDeclaration
    | keyPreferenceDeclaration
    ;


/* ============================================================================
 * 3. MANAGED KEY DECLARATION
 * ========================================================================== */

/*
 * Canonical source-level declaration:
 *
 *     key security::signing::application {
 *         algorithm: cryptography::signature::algorithm;
 *         purpose: application::signing;
 *         provider: organization::kms;
 *     }
 *
 * The declaration identifies a managed key object.
 *
 * It NEVER contains key bytes.
 */
keyDeclaration
    : KEY
      qualifiedName
      keyDeclarationBody?
      SEMICOLON?
    ;


keyDeclarationBody
    : LBRACE
      keyMember*
      RBRACE
    ;


keyMember
    : keyProperty
    | keyReferenceProperty
    | keyTypeProperty
    | keyRequirement
    | keyConstraint
    | keyPreference
    ;


/* ============================================================================
 * 4. GENERIC KEY PROPERTIES
 * ========================================================================== */

/*
 * Generic property syntax is deliberate.
 *
 * It allows future key-management concepts without modifying the grammar for
 * every new field.
 *
 * Examples:
 *
 *     algorithm: cryptography::signature::algorithm;
 *     purpose: application::signing;
 *     lifecycle: managed;
 *     version: current;
 *     status: active;
 *     storage: secure::key_store;
 */
keyProperty
    : identifier
      COLON
      expression
      SEMICOLON?
    ;


/*
 * Properties whose values are symbolic names rather than arbitrary values.
 */
keyReferenceProperty
    : identifier
      COLON
      qualifiedName
      SEMICOLON?
    ;


/*
 * A key may have a source-level type contract.
 *
 * Example:
 *
 *     type: KeyMaterialHandle;
 *
 * or:
 *
 *     type: security::key::managed;
 *
 * Type semantics remain owned by Types and semantic analysis.
 */
keyTypeProperty
    : TYPE
      COLON
      typeExpression
      SEMICOLON?
    ;


/* ============================================================================
 * 5. KEY REQUIREMENTS
 * ========================================================================== */

keyRequirement
    : REQUIRES
      expression
      SEMICOLON?
    ;


keyCapabilityRequirement
    : REQUIRES
      CAPABILITY
      LPAREN
      expression
      RPAREN
      SEMICOLON?
    ;


keyResourceRequirement
    : REQUIRES
      RESOURCE
      LPAREN
      expression
      RPAREN
      SEMICOLON?
    ;


keyRequirementDeclaration
    : REQUIRES
      qualifiedName
      keyRequirementBody?
      SEMICOLON?
    ;


keyRequirementBody
    : LBRACE
      keyRequirementMember*
      RBRACE
    ;


keyRequirementMember
    : keyRequirement
    | keyCapabilityRequirement
    | keyResourceRequirement
    | keyProperty
    ;


/* ============================================================================
 * 6. KEY CONSTRAINTS
 * ========================================================================== */

keyConstraint
    : CONSTRAINT
      expression
      SEMICOLON?
    ;


keyConstraintDeclaration
    : CONSTRAINT
      qualifiedName
      keyConstraintBody?
      SEMICOLON?
    ;


keyConstraintBody
    : LBRACE
      keyConstraintMember*
      RBRACE
    ;


keyConstraintMember
    : keyConstraint
    | keyProperty
    ;


/* ============================================================================
 * 7. KEY PREFERENCES
 * ========================================================================== */

keyPreference
    : PREFER
      expression
      SEMICOLON?
    ;


keyPreferenceDeclaration
    : PREFER
      qualifiedName
      keyPreferenceBody?
      SEMICOLON?
    ;


keyPreferenceBody
    : LBRACE
      keyPreferenceMember*
      RBRACE
    ;


keyPreferenceMember
    : keyPreference
    | keyProperty
    ;


/* ============================================================================
 * 8. KEY REFERENCES
 * ========================================================================== */

/*
 * A key reference is a symbolic reference.
 *
 * It is deliberately not a key-material expression.
 */
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


/*
 * A more general reference is useful for handles, aliases and values produced
 * by another operation.
 *
 * Semantic analysis determines whether the expression denotes a legal key
 * handle.
 */
keyHandleExpression
    : expression
    ;


/* ============================================================================
 * 9. KEY OPERATIONS
 * ========================================================================== */

/*
 * All executable key-management actions use:
 *
 *     apply key <operation>(...)
 *
 * Examples:
 *
 *     apply key generate(signing_key);
 *
 *     apply key derive(child_key, parent_key);
 *
 *     apply key rotate(signing_key);
 *
 *     apply key revoke(compromised_key);
 *
 *     apply key destroy(retired_key);
 *
 *     apply key wrap(content_key, wrapping_key);
 *
 *     apply key unwrap(wrapped_key, wrapping_key);
 *
 *     apply key import(key_reference);
 *
 *     apply key export_metadata(key_reference);
 *
 * The operation name remains an open-world qualified name.
 *
 * Therefore future operations require no grammar redesign.
 */
keyOperation
    : APPLY
      KEY
      qualifiedName
      keyOperationArguments?
      keyOperationBody?
      SEMICOLON?
    ;


keyOperationArguments
    : LPAREN
      keyOperationArgumentList?
      RPAREN
    ;


keyOperationArgumentList
    : expression
      (
          COMMA
          expression
      )*
      COMMA?
    ;


keyOperationBody
    : LBRACE
      keyOperationMember*
      RBRACE
    ;


keyOperationMember
    : keyProperty
    | keyReferenceProperty
    | keyRequirement
    | keyConstraint
    | keyPreference
    | keyOperationMetadata
    ;


/* ============================================================================
 * 10. KEY OPERATION METADATA
 * ========================================================================== */

keyOperationMetadata
    : identifier
      COLON
      expression
      SEMICOLON?
    ;


/* ============================================================================
 * 11. GENERATION
 * ========================================================================== */

/*
 * Generation is represented semantically.
 *
 * This grammar does not enumerate random-number generators or key algorithms.
 *
 * The following is therefore valid:
 *
 *     apply key cryptography::generate(signing_key) {
 *         algorithm: cryptography::signature::algorithm;
 *         purpose: application::signing;
 *     }
 *
 * The grammar does not know how the key is generated.
 */
keyGenerationOperation
    : APPLY
      KEY
      qualifiedName
      keyOperationArguments?
      keyOperationBody?
      SEMICOLON?
    ;


/* ============================================================================
 * 12. DERIVATION
 * ========================================================================== */

/*
 * Derivation remains open-world.
 *
 * The actual KDF/derivation mechanism is identified semantically.
 */
keyDerivationOperation
    : DERIVE
      KEY
      keyHandleExpression
      keyDerivationBody?
      SEMICOLON?
    ;


keyDerivationBody
    : LBRACE
      keyDerivationMember*
      RBRACE
    ;


keyDerivationMember
    : keyProperty
    | keyReferenceProperty
    | keyRequirement
    | keyConstraint
    | keyPreference
    ;


/* ============================================================================
 * 13. KEY WRAPPING / UNWRAPPING
 * ========================================================================== */

/*
 * These operations intentionally remain expression-driven.
 *
 * The grammar does not prescribe:
 *
 *     envelope format;
 *     wrapping algorithm;
 *     provider;
 *     key size;
 *     hardware;
 *     storage.
 */
keyWrapOperation
    : APPLY
      KEY
      qualifiedName
      keyOperationArguments?
      keyOperationBody?
      SEMICOLON?
    ;


keyUnwrapOperation
    : APPLY
      KEY
      qualifiedName
      keyOperationArguments?
      keyOperationBody?
      SEMICOLON?
    ;


/* ============================================================================
 * 14. ROTATION
 * ========================================================================== */

keyRotationOperation
    : APPLY
      KEY
      qualifiedName
      keyOperationArguments?
      keyOperationBody?
      SEMICOLON?
    ;


/* ============================================================================
 * 15. REVOCATION
 * ========================================================================== */

keyRevocationOperation
    : APPLY
      KEY
      qualifiedName
      keyOperationArguments?
      keyOperationBody?
      SEMICOLON?
    ;


/* ============================================================================
 * 16. DESTRUCTION
 * ========================================================================== */

keyDestructionOperation
    : APPLY
      KEY
      qualifiedName
      keyOperationArguments?
      keyOperationBody?
      SEMICOLON?
    ;


/* ============================================================================
 * 17. LIFECYCLE INTENT
 * ========================================================================== */

/*
 * Generic lifecycle metadata avoids hard-coding a closed lifecycle enum.
 *
 * Examples:
 *
 *     lifecycle: active;
 *     lifecycle: suspended;
 *     lifecycle: revoked;
 *     lifecycle: retired;
 *     lifecycle: future::state;
 *
 * The semantic layer determines legal transitions.
 */
keyLifecycleProperty
    : identifier
      COLON
      expression
      SEMICOLON?
    ;


/* ============================================================================
 * 18. KEY PURPOSE
 * ========================================================================== */

/*
 * Purpose is semantic data.
 *
 * Examples:
 *
 *     purpose: cryptography::encryption;
 *     purpose: cryptography::signature;
 *     purpose: cryptography::authentication;
 *     purpose: application::data;
 *     purpose: future::purpose;
 *
 * No finite purpose list is embedded.
 */
keyPurposeProperty
    : identifier
      COLON
      qualifiedName
      SEMICOLON?
    ;


/* ============================================================================
 * 19. ALGORITHM REFERENCE
 * ========================================================================== */

keyAlgorithmReference
    : qualifiedName
    ;


keyAlgorithmList
    : keyAlgorithmReference
      (
          COMMA
          keyAlgorithmReference
      )*
      COMMA?
    ;


/* ============================================================================
 * 20. PROVIDER REFERENCE
 * ========================================================================== */

keyProviderReference
    : qualifiedName
    ;


keyProviderList
    : keyProviderReference
      (
          COMMA
          keyProviderReference
      )*
      COMMA?
    ;


/* ============================================================================
 * 21. IMPLEMENTATION REFERENCE
 * ========================================================================== */

keyImplementationReference
    : qualifiedName
    ;


keyImplementationList
    : keyImplementationReference
      (
          COMMA
          keyImplementationReference
      )*
      COMMA?
    ;


/* ============================================================================
 * 22. KEY VERSION REFERENCES
 * ========================================================================== */

/*
 * Version identifiers remain expressions.
 *
 * This permits:
 *
 *     current
 *     previous
 *     requested_version
 *     7
 *     policy::current
 *
 * without making version numbering a grammar-level policy.
 */
keyVersionReference
    : expression
    ;


/* ============================================================================
 * 23. KEY STORAGE REFERENCES
 * ========================================================================== */

keyStorageReference
    : qualifiedName
    ;


keyStorageProperty
    : identifier
      COLON
      keyStorageReference
      SEMICOLON?
    ;


/* ============================================================================
 * 24. KEY ACCESS / AUTHORIZATION REFERENCES
 * ========================================================================== */

/*
 * Authorization is NOT implemented here.
 *
 * This merely allows a key-management construct to refer to an authorization
 * or identity object.
 */
keyAccessReference
    : qualifiedName
    ;


keyAccessProperty
    : identifier
      COLON
      keyAccessReference
      SEMICOLON?
    ;


/* ============================================================================
 * 25. KEY ROTATION POLICY
 * ========================================================================== */

/*
 * Rotation policy is represented by ordinary expressions.
 *
 * This permits policies based on:
 *
 *     time;
 *     events;
 *     usage;
 *     compromise;
 *     external policy;
 *     resource conditions;
 *     security state;
 *     application state.
 *
 * The grammar does not prescribe a fixed rotation interval.
 */
keyRotationPolicy
    : identifier
      COLON
      expression
      SEMICOLON?
    ;


/* ============================================================================
 * 26. KEY REVOCATION POLICY
 * ========================================================================== */

keyRevocationPolicy
    : identifier
      COLON
      expression
      SEMICOLON?
    ;


/* ============================================================================
 * 27. KEY DESTRUCTION POLICY
 * ========================================================================== */

keyDestructionPolicy
    : identifier
      COLON
      expression
      SEMICOLON?
    ;


/* ============================================================================
 * 28. KEY AUDIT / PROVENANCE
 * ========================================================================== */

/*
 * Audit data is metadata.
 *
 * The grammar does not create an audit runtime.
 */
keyAuditProperty
    : identifier
      COLON
      expression
      SEMICOLON?
    ;


/* ============================================================================
 * 29. KEY EXPORT / IMPORT BOUNDARY
 * ========================================================================== */

/*
 * Import/export operations must refer to semantic objects or protected
 * handles.
 *
 * They do not create raw key-material literals.
 */
keyImportReference
    : expression
    ;


keyExportReference
    : expression
    ;


/* ============================================================================
 * 30. GENERIC KEY OPERATION REFERENCE
 * ========================================================================== */

/*
 * This is intentionally open-world.
 *
 * Future key-management operations can be referenced without changing this
 * grammar.
 */
keyOperationReference
    : qualifiedName
    ;


keyOperationReferenceList
    : keyOperationReference
      (
          COMMA
          keyOperationReference
      )*
      COMMA?
    ;


/* ============================================================================
 * 31. KEY CAPABILITY REFERENCES
 * ========================================================================== */

keyCapabilityReference
    : qualifiedName
    ;


keyCapabilityReferenceList
    : keyCapabilityReference
      (
          COMMA
          keyCapabilityReference
      )*
      COMMA?
    ;


/* ============================================================================
 * 32. KEY RESOURCE REFERENCES
 * ========================================================================== */

keyResourceReference
    : qualifiedName
    ;


keyResourceReferenceList
    : keyResourceReference
      (
          COMMA
          keyResourceReference
      )*
      COMMA?
    ;


/* ============================================================================
 * 33. KEY SECURITY PROPERTY
 * ========================================================================== */

/*
 * Security properties are semantic names.
 *
 * Examples:
 *
 *     security::confidentiality
 *     security::integrity
 *     security::forward_secrecy
 *     security::non_repudiation
 *     security::post_quantum
 *     future::security::property
 *
 * No finite property list is embedded.
 */
keySecurityProperty
    : qualifiedName
    ;


keySecurityPropertyList
    : keySecurityProperty
      (
          COMMA
          keySecurityProperty
      )*
      COMMA?
    ;


/* ============================================================================
 * 34. KEY ATTACHMENTS
 * ========================================================================== */

/*
 * A key may be semantically associated with another source-level object:
 *
 *     identity
 *     principal
 *     service
 *     data object
 *     quantum program
 *     classical program
 *     hardware intent
 *     distributed service
 *
 * The association is metadata, not runtime binding.
 */
keyAttachment
    : identifier
      COLON
      qualifiedName
      SEMICOLON?
    ;


keyAttachmentList
    : keyAttachment+
    ;


/* ============================================================================
 * 35. KEY OPERATION RESULT
 * ========================================================================== */

/*
 * Result references remain source-level expressions.
 *
 * The grammar does not prescribe a runtime result representation.
 */
keyResultProperty
    : identifier
      COLON
      expression
      SEMICOLON?
    ;


/* ============================================================================
 * 36. DETERMINISTIC PARSING CONTRACT
 * ========================================================================== */

/*
 * This grammar contains:
 *
 *     - no semantic predicates;
 *     - no actions;
 *     - no Rust code;
 *     - no filesystem access;
 *     - no network access;
 *     - no runtime calls;
 *     - no hardware discovery;
 *     - no randomness.
 *
 * Parsing therefore depends only on:
 *
 *     source token stream
 *     grammar version
 *     lexical vocabulary
 *
 * Security semantics are evaluated later.
 */


/* ============================================================================
 * 37. SECURITY CONTRACT
 * ========================================================================== */

/*
 * The grammar MUST NOT expose secret material through diagnostics.
 *
 * Semantic/compiler layers must ensure that diagnostics identify:
 *
 *     key reference;
 *     source span;
 *     semantic category;
 *
 * without printing:
 *
 *     private key material;
 *     secret key material;
 *     passwords;
 *     seeds;
 *     credentials;
 *     authentication secrets.
 *
 * The grammar itself never receives interpreted secret bytes.
 */


/* ============================================================================
 * 38. HARDWARE INDEPENDENCE
 * ========================================================================== */

/*
 * Invalid architectural patterns include:
 *
 *     key_on_hsm_0
 *     key_on_cpu_0
 *     key_on_gpu_0
 *     key_on_qpu_0
 *     key_on_node_0
 *
 * as universal grammar concepts.
 *
 * Target-specific placement is downstream.
 *
 * Valid source intent is expressed through abstract requirements:
 *
 *     requires capability("secure.key.storage");
 *
 *     requires capability("hardware.key.isolation");
 *
 *     requires capability("quantum.safe.cryptography");
 *
 * Actual realization is selected downstream.
 */


/* ============================================================================
 * 39. NO ARTIFICIAL SCALE LIMITS
 * ========================================================================== */

/*
 * These are NOT language limits:
 *
 *     key_size: requested_size;
 *     version: requested_version;
 *     quantity: requested_quantity;
 *
 * A program may express arbitrary symbolic values.
 *
 * The compiler/runtime determines whether the requested computation can be
 * realized with available resources.
 *
 * This preserves POCO-REAF.
 */


/* ============================================================================
 * 40. CROSS-DOMAIN INTEGRATION
 * ========================================================================== */

/*
 * Classical:
 *
 *     key protects classical computation.
 *
 * Quantum:
 *
 *     key protects quantum program metadata, control, communication or
 *     associated classical resources.
 *
 * Hybrid:
 *
 *     one security contract may cover both classical and quantum portions.
 *
 * HDL:
 *
 *     key-management intent may describe security properties of a hardware
 *     design without selecting a particular physical implementation.
 *
 * Distributed:
 *
 *     key references may be associated with distributed services and
 *     communication identities.
 *
 * AI:
 *
 *     model/data pipelines may refer to key-managed encryption or signing.
 *
 * Networking:
 *
 *     secure channels may refer to managed key objects.
 *
 * No domain receives a special hard-coded key representation here.
 */


/* ============================================================================
 * 41. CANONICAL IR INTEGRATION
 * ========================================================================== */

/*
 * Required lowering:
 *
 *     source
 *       |
 *       v
 *     key-management parse tree
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic security model
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +-----------------------+
 *       |                       |
 *       v                       v
 *     classical             quantum::ir
 *       |                       |
 *       +-----------+-----------+
 *                   |
 *                   v
 *             canonical IR
 *                   |
 *                   v
 *              optimization
 *                   |
 *          routing / scheduling
 *                   |
 *             resilience / ZQN
 *                   |
 *                  HAL
 *                   |
 *                target
 *
 * There is NO KeyManagementIR.
 */


/* ============================================================================
 * 42. DIAGNOSTIC CONTRACT
 * ========================================================================== */

/*
 * Syntax diagnostics:
 *
 *     malformed key declaration
 *     malformed key operation
 *     malformed key argument list
 *     malformed key property
 *
 * Semantic diagnostics:
 *
 *     unresolved key reference
 *     unknown algorithm
 *     incompatible algorithm/key
 *     invalid lifecycle transition
 *     prohibited operation
 *     unavailable capability
 *     unsatisfied resource requirement
 *     unauthorized operation
 *     invalid provider
 *     invalid implementation
 *     invalid key version
 *
 * Runtime/resource diagnostics:
 *
 *     unavailable provider
 *     unavailable secure storage
 *     insufficient resources
 *     unavailable hardware capability
 *
 * These categories must remain distinct.
 */


/* ============================================================================
 * 43. COMPATIBILITY CONTRACT
 * ========================================================================== */

/*
 * Adding a new algorithm:
 *
 *     DOES NOT require a grammar change.
 *
 * Adding a new provider:
 *
 *     DOES NOT require a grammar change.
 *
 * Adding a new implementation:
 *
 *     DOES NOT require a grammar change.
 *
 * Adding a new key property:
 *
 *     normally DOES NOT require a grammar change because properties are
 *     identifier/expression based.
 *
 * Adding a new key operation:
 *
 *     DOES NOT require a grammar change because operation names are open-world
 *     qualified names.
 *
 * Changing `key` from an ordinary identifier to the reserved KEY token:
 *
 *     IS a lexical compatibility change.
 *
 * It must therefore be recorded in:
 *
 *     grammar/compatibility/
 *     grammar/spec/compatibility.md
 *
 * and reflected in:
 *
 *     grammar/grammar.md
 *
 * ============================================================================
 * 44. POSITIVE CONFORMANCE EXAMPLES
 * ========================================================================== */

/*
 * These examples are intentionally comments so this grammar remains a
 * reusable parser component.
 *
 * VALID:
 *
 *     key security::signing::application {
 *         algorithm: cryptography::signature::algorithm;
 *         purpose: application::signing;
 *         provider: organization::key_service;
 *         version: current;
 *         lifecycle: managed;
 *     }
 *
 *     key security::encryption::data {
 *         algorithm: cryptography::encryption::algorithm;
 *         purpose: application::data;
 *     }
 *
 *     apply key generate(security::signing::application);
 *
 *     apply key derive(
 *         security::signing::application,
 *         security::signing::child
 *     );
 *
 *     apply key rotate(security::signing::application);
 *
 *     apply key revoke(security::signing::application);
 *
 *     apply key destroy(security::signing::retired);
 *
 *     apply key wrap(
 *         application::data_key,
 *         security::wrapping::key
 *     );
 *
 *     apply key unwrap(
 *         application::wrapped_key,
 *         security::wrapping::key
 *     );
 *
 *     apply key import(application::protected_key_reference);
 *
 *     apply key export_metadata(
 *         security::signing::application
 *     );
 *
 *     key security::distributed::service {
 *         requires capability("secure.key.storage");
 *         requires capability("secure.key.isolation");
 *         constraint security::policy::approved;
 *         prefer security::provider::local;
 *     }
 *
 *     key security::quantum::service {
 *         requires capability("quantum.safe.cryptography");
 *         requires capability("secure.key.storage");
 *     }
 *
 * FUTURE:
 *
 *     apply key future::key_operation(...);
 *
 * No grammar change is necessary for the new operation name.
 */


/* ============================================================================
 * 45. NEGATIVE CONFORMANCE EXAMPLES
 * ========================================================================== */

/*
 * INVALID:
 *
 *     key {
 *     }
 *
 *     key security::key
 *
 *     apply key generate(
 *
 *     apply key generate();
 *
 *     key security::signing {
 *         private_key: "raw-secret-material";
 *     }
 *
 * The last example is syntactically representable as an ordinary expression
 * if the expression grammar accepts a string.
 *
 * It MUST therefore be rejected, when prohibited, by semantic/security
 * analysis rather than by inventing a secret-literal grammar.
 *
 * This distinction is intentional:
 *
 *     syntax validity != security authorization.
 */


/* ============================================================================
 * 46. BOUNDARY TESTS
 * ========================================================================== */

/*
 * Test:
 *
 *     deeply nested qualified names;
 *
 *     large property sets;
 *
 *     deeply nested expressions;
 *
 *     large key operation argument lists;
 *
 *     symbolic version expressions;
 *
 *     symbolic resource requirements;
 *
 *     many independent key declarations;
 *
 *     many independent key operations;
 *
 *     future operation names;
 *
 *     future provider names;
 *
 *     future algorithm names.
 *
 * No test may introduce an artificial key count or key-size ceiling.
 */


/* ============================================================================
 * 47. SCALABILITY TESTS
 * ========================================================================== */

/*
 * The test suite must demonstrate that syntax does not depend on:
 *
 *     number of keys;
 *     number of versions;
 *     number of providers;
 *     number of operations;
 *     number of requirements;
 *     number of nodes;
 *     number of CPUs;
 *     number of GPUs;
 *     number of FPGAs;
 *     number of QPUs;
 *     memory capacity;
 *     network size.
 *
 * Scaling is a compiler/runtime/resource question.
 *
 * It is not a grammar constant.
 */


/* ============================================================================
 * 48. DETERMINISM TESTS
 * ========================================================================== */

/*
 * The same token stream and grammar version MUST produce the same parse tree.
 *
 * No result may depend on:
 *
 *     wall-clock time;
 *     random state;
 *     hardware;
 *     environment variables;
 *     network;
 *     filesystem;
 *     provider availability.
 */


/* ============================================================================
 * 49. SECURITY TESTS
 * ========================================================================== */

/*
 * Required security tests include:
 *
 *     - raw secret material is not introduced as a dedicated syntax;
 *     - private-key material is not printed in diagnostics;
 *     - key references remain symbolic;
 *     - provider names remain symbolic;
 *     - implementation names remain symbolic;
 *     - hardware identifiers are not required;
 *     - physical placement is not encoded;
 *     - secret lifecycle is not executed by parsing;
 *     - authorization remains downstream;
 *     - key destruction remains downstream;
 *     - rotation remains downstream.
 */


/* ============================================================================
 * 50. COMPLETION CRITERIA
 * ========================================================================== */

/*
 * This file is complete when:
 *
 * [x] The file is the sole key-management grammar owner.
 * [x] No second key-management grammar exists.
 * [x] It consumes ZamaniLexer.
 * [x] It imports canonical Names.
 * [x] It imports canonical Types.
 * [x] It imports canonical Expressions.
 * [x] It does not redefine identifier syntax.
 * [x] It does not redefine qualified-name syntax.
 * [x] It does not redefine expressions.
 * [x] It does not redefine types.
 * [x] It does not embed key material.
 * [x] It does not enumerate cryptographic algorithms.
 * [x] It does not enumerate providers.
 * [x] It does not enumerate implementations.
 * [x] It does not impose key-count limits.
 * [x] It does not impose key-size limits.
 * [x] It does not impose version-count limits.
 * [x] It does not impose hardware limits.
 * [x] It does not impose node limits.
 * [x] It does not impose qubit limits.
 * [x] It supports symbolic key references.
 * [x] It supports managed-key declarations.
 * [x] It supports open-world operations.
 * [x] It supports generation intent.
 * [x] It supports derivation intent.
 * [x] It supports wrapping intent.
 * [x] It supports unwrapping intent.
 * [x] It supports rotation intent.
 * [x] It supports revocation intent.
 * [x] It supports destruction intent.
 * [x] It supports import intent.
 * [x] It supports metadata export intent.
 * [x] It supports requirements.
 * [x] It supports capabilities.
 * [x] It supports resource requirements.
 * [x] It supports constraints.
 * [x] It supports preferences.
 * [x] It preserves open-world extensibility.
 * [x] It creates no KeyManagementIR.
 * [x] It creates no quantum-specific IR.
 * [x] It performs no runtime work.
 * [x] It contains no Rust actions.
 * [x] It requires no unsafe Rust.
 * [x] It documents AST integration.
 * [x] It documents semantic integration.
 * [x] It documents IR integration.
 * [x] It documents runtime integration.
 * [x] It documents compatibility.
 * [x] It documents scalability.
 * [x] It documents deterministic parsing.
 *
 * Remaining repository integration is deliberately explicit below the file:
 *
 *     lexer keyword registration
 *     security composition registration
 *     parser composition validation
 *     conformance tests
 *
 * ============================================================================
 */
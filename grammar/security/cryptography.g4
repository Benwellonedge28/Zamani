/*
 * Zamani Cryptography Grammar
 *
 * File:
 *   grammar/security/cryptography.g4
 *
 * Purpose:
 *   Production parser grammar for source-level cryptographic declarations,
 *   cryptographic requirements, algorithm intent, key references, primitive
 *   references, protocol intent, security properties, cryptographic policies,
 *   backend-independent cryptographic constraints, and cryptographic
 *   implementation preferences.
 *
 * Language:
 *   Zamani
 *
 * Parser target:
 *   Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *   The generated Rust integration must remain unsafe-free.
 *   This grammar contains no Rust target actions and no unsafe code.
 *
 * ---------------------------------------------------------------------------
 * OWNERSHIP
 * ---------------------------------------------------------------------------
 *
 * This grammar owns:
 *
 *   - cryptographic source syntax
 *   - cryptographic declarations
 *   - cryptographic intent
 *   - algorithm references
 *   - primitive references
 *   - key references
 *   - key roles
 *   - cryptographic purpose
 *   - cryptographic properties
 *   - protocol intent
 *   - cryptographic requirements
 *   - cryptographic constraints
 *   - cryptographic preferences
 *   - cryptographic backend/implementation references
 *   - cryptographic metadata
 *
 * This grammar does NOT own:
 *
 *   - actual cryptographic algorithms
 *   - key generation
 *   - key storage
 *   - private-key material
 *   - secret keys
 *   - passwords
 *   - authentication implementation
 *   - certificate validation
 *   - trust evaluation
 *   - permission enforcement
 *   - capability enforcement
 *   - randomness generation
 *   - cryptographic hardware discovery
 *   - hardware topology
 *   - provider-specific execution
 *   - runtime key management
 *   - quantum IR
 *   - QEC
 *   - ZQN
 *   - scheduling
 *   - optimization
 *   - resource discovery
 *
 * ---------------------------------------------------------------------------
 * SECURITY PRINCIPLE
 * ---------------------------------------------------------------------------
 *
 * Source syntax may identify a key, credential, certificate, secret,
 * algorithm, or cryptographic object by reference.
 *
 * It MUST NOT embed the secret/private value itself.
 *
 * Examples of forbidden source-level semantics:
 *
 *   secret = "actual-secret"
 *   private_key = "actual-private-key"
 *   password = "actual-password"
 *
 * References such as:
 *
 *   key "signing-key"
 *   key "device.identity"
 *   credential "session-key"
 *
 * are syntactically representable, but their resolution and security policy
 * belong to downstream semantic/runtime systems.
 *
 * ---------------------------------------------------------------------------
 * POCO-REAF
 * ---------------------------------------------------------------------------
 *
 * Cryptographic source code describes semantic security intent rather than
 * a particular implementation.
 *
 * Therefore the grammar MUST NOT require:
 *
 *   - a fixed cryptographic provider
 *   - a fixed CPU
 *   - a fixed accelerator
 *   - a fixed HSM
 *   - a fixed FPGA
 *   - a fixed instruction set
 *   - a fixed memory size
 *   - a fixed key-store implementation
 *   - a fixed number of cryptographic operations
 *   - a fixed number of keys
 *   - a fixed deployment topology
 *
 * Physical realization is supplied by target capabilities, resources,
 * compilation, deployment, and runtime systems.
 *
 * ---------------------------------------------------------------------------
 * INTEGRATION
 * ---------------------------------------------------------------------------
 *
 * This is a parser grammar.
 *
 * It consumes the canonical Zamani lexer vocabulary.
 *
 * Semantic analysis must lower its parse tree into the repository's canonical
 * security semantic representation.
 *
 * It must not create a competing cryptographic IR.
 *
 * ---------------------------------------------------------------------------
 * ANTLR
 * ---------------------------------------------------------------------------
 *
 * This file intentionally contains no target-language actions.
 *
 * Keep semantic validation outside the grammar so that:
 *
 *   grammar -> AST -> semantic analysis -> IR/backend
 *
 * remains independent of the generated Rust parser implementation.
 */

parser grammar Cryptography;

options {
    tokenVocab = ZamaniLexer;
}


/* ========================================================================= */
/* ENTRY POINT                                                               */
/* ========================================================================= */

/*
 * Standalone entry point used by grammar tests and tooling.
 *
 * The main Zamani parser may import/reuse the declaration rules instead of
 * using this EOF-consuming entry point.
 */
cryptographyFile
    : cryptographyEntry* EOF
    ;

cryptographyEntry
    : cryptographicDeclaration
    | cryptographicRequirementDeclaration
    | cryptographicConstraintDeclaration
    | cryptographicPreferenceDeclaration
    ;


/* ========================================================================= */
/* CRYPTOGRAPHIC DECLARATIONS                                                */
/* ========================================================================= */

cryptographicDeclaration
    : CRYPTOGRAPHY identifier cryptographicBody? SEMI?
    ;

cryptographicBody
    : LBRACE cryptographicMember* RBRACE
    ;

cryptographicMember
    : algorithmClause
    | primitiveClause
    | keyClause
    | keyReferenceClause
    | keyRoleClause
    | purposeClause
    | propertyClause
    | protocolClause
    | parameterClause
    | requirementClause
    | constraintClause
    | preferenceClause
    | implementationClause
    | providerClause
    | namespaceClause
    | metadataClause
    ;


/* ========================================================================= */
/* ALGORITHMS                                                                */
/* ========================================================================= */

algorithmClause
    : ALGORITHM qualifiedName
    ;

algorithmReference
    : qualifiedName
    ;

algorithmList
    : algorithmReference (COMMA algorithmReference)*
    ;


/* ========================================================================= */
/* CRYPTOGRAPHIC PRIMITIVES                                                  */
/* ========================================================================= */

primitiveClause
    : PRIMITIVE qualifiedName
    ;

primitiveReference
    : qualifiedName
    ;

primitiveList
    : primitiveReference (COMMA primitiveReference)*
    ;


/* ========================================================================= */
/* KEYS                                                                      */
/* ========================================================================= */

keyClause
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
    | keyStorageClause
    | keyLifecycleClause
    | metadataClause
    ;

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
    : CONSTRAINT constraintExpression
    ;

keyPreferenceClause
    : PREFER preferenceExpression
    ;

keyStorageClause
    : STORAGE qualifiedName
    ;

keyLifecycleClause
    : LIFECYCLE expression
    ;

keyRoleClause
    : ROLE expression
    ;


/* ========================================================================= */
/* KEY ROLES                                                                 */
/* ========================================================================= */

keyRoleDeclaration
    : KEY_ROLE identifier keyRoleBody?
    ;

keyRoleBody
    : LBRACE
        keyRoleMember*
      RBRACE
    ;

keyRoleMember
    : purposeClause
    | propertyClause
    | requirementClause
    | constraintClause
    | preferenceClause
    | metadataClause
    ;

keyRoleReference
    : qualifiedName
    ;


/* ========================================================================= */
/* PURPOSE                                                                    */
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
/* SECURITY PROPERTIES                                                        */
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
/* PROTOCOLS                                                                  */
/* ========================================================================= */

protocolClause
    : PROTOCOL qualifiedName protocolBody?
    ;

protocolBody
    : LBRACE protocolMember* RBRACE
    ;

protocolMember
    : algorithmClause
    | primitiveClause
    | keyClause
    | keyReferenceClause
    | purposeClause
    | propertyClause
    | requirementClause
    | constraintClause
    | preferenceClause
    | participantClause
    | parameterClause
    | metadataClause
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
/* PARAMETERS                                                                 */
/* ========================================================================= */

parameterClause
    : PARAMETER identifier ASSIGN expression
    ;

parameterList
    : parameterClause (COMMA parameterClause)*
    ;


/* ========================================================================= */
/* IMPLEMENTATION / PROVIDER REFERENCES                                      */
/* ========================================================================= */

/*
 * These are references/intent only.
 *
 * They do not cause the grammar to depend on a concrete cryptographic
 * library, vendor, device, CPU, accelerator, HSM, or operating system.
 */
implementationClause
    : IMPLEMENTATION qualifiedName
    ;

providerClause
    : PROVIDER qualifiedName
    ;


/* ========================================================================= */
/* NAMESPACE                                                                   */
/* ========================================================================= */

namespaceClause
    : NAMESPACE qualifiedName
    ;


/* ========================================================================= */
/* REQUIREMENTS                                                               */
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
    | protocolClause
    | parameterClause
    | metadataClause
    ;

requirementClause
    : REQUIRE requirementExpression
    ;

requirementExpression
    : expression
    ;


/* ========================================================================= */
/* CONSTRAINTS                                                                */
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
    | protocolClause
    | metadataClause
    ;

constraintClause
    : CONSTRAIN constraintExpression
    ;

constraintExpression
    : expression
    ;


/* ========================================================================= */
/* PREFERENCES                                                                */
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
    | implementationClause
    | providerClause
    | parameterClause
    | metadataClause
    ;

preferenceClause
    : PREFER preferenceExpression
    ;

preferenceExpression
    : expression
    ;


/* ========================================================================= */
/* REFERENCE VALUES                                                           */
/* ========================================================================= */

/*
 * A reference is deliberately not a secret literal.
 *
 * The semantic layer decides whether a referenced object is:
 *
 *   - a key handle
 *   - a certificate reference
 *   - a credential reference
 *   - a secure-element object
 *   - an HSM object
 *   - a runtime-managed object
 *   - a remote key-management object
 *   - another supported security resource
 */
referenceValue
    : qualifiedName
    | stringLiteral
    ;


/* ========================================================================= */
/* METADATA                                                                   */
/* ========================================================================= */

metadataClause
    : METADATA LBRACE metadataEntry* RBRACE
    ;

metadataEntry
    : identifier ASSIGN expression
    ;


/* ========================================================================= */
/* COMMON LISTS                                                               */
/* ========================================================================= */

cryptographicDeclarationList
    : cryptographicDeclaration
      (cryptographicDeclaration)*
    ;

algorithmClauseList
    : algorithmClause*
    ;

primitiveClauseList
    : primitiveClause*
    ;

keyClauseList
    : keyClause*
    ;

protocolClauseList
    : protocolClause*
    ;


/* ========================================================================= */
/* SHARED CORE CONTRACT                                                       */
/* ========================================================================= */

/*
 * These rule names are intentionally semantic contracts with the canonical
 * Zamani grammar foundation.
 *
 * They MUST resolve through the repository's canonical imported grammar
 * structure rather than being redefined here.
 *
 * Expected shared contracts:
 *
 *   identifier
 *   qualifiedName
 *   expression
 *   stringLiteral
 *
 * If the repository uses different canonical rule names, the import adapter
 * must map them centrally rather than creating cryptography-specific copies.
 */
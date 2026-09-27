parser grammar ZamaniSecrets;

options {
    tokenVocab = ZamaniLexer;
}

@header {
    //! Zamani secrets grammar.
    //!
    //! This grammar defines portable syntax for declaring and referring to
    //! secret material and for expressing secret-handling intent.
    //!
    //! Ownership:
    //! - This grammar owns secret declarations, references, metadata,
    //!   lifecycle intent, handling requirements, and secret attributes.
    //!
    //! This grammar does NOT own:
    //! - cryptographic algorithms or implementations;
    //! - key-generation or key-management implementation;
    //! - identity verification;
    //! - authorization enforcement;
    //! - secret storage backends;
    //! - HSM/TEE/TPM/device-specific behavior;
    //! - networking or transport implementation;
    //! - resource scheduling;
    //! - hardware topology;
    //! - fixed secret sizes or capacities;
    //! - runtime secret values.
    //!
    //! Secret semantics are resolved after parsing by the semantic/security
    //! layers and lowered into the repository's canonical security model.
    //!
    //! No artificial limits are encoded here. Secret count, size, lifetime,
    //! scope, storage capacity, rotation frequency, or number of consumers
    //! are determined by program semantics and available resources.
    //!
    //! Rust compatibility:
    //! Generated parser integration targets Rust 1.97 / 1.97.1.
    //! This grammar introduces no unsafe Rust code.
}

// -----------------------------------------------------------------------------
// Secret declarations
// -----------------------------------------------------------------------------
//
// A secret declaration introduces a named secret resource.
//
// Example:
//
// secret DatabaseCredential {
//     description "Credential used by the database service";
// }
//
// The declaration describes intent. It does not place secret bytes into the
// source program or prescribe a particular storage mechanism.
//

secretDeclaration
    : SECRET Identifier secretHeader* LBRACE secretMember* RBRACE
    ;

// -----------------------------------------------------------------------------
// Secret declaration headers
// -----------------------------------------------------------------------------
//
// Headers provide extensible declaration-level metadata without introducing
// target-specific syntax.
//

secretHeader
    : secretVersion
    | secretDescription
    | secretScope
    | secretLifecycle
    | secretClassification
    | secretAttribute
    ;

secretVersion
    : VERSION expression SEMICOLON
    ;

secretDescription
    : DESCRIPTION STRING_LITERAL SEMICOLON
    ;

secretScope
    : SCOPE expression SEMICOLON
    ;

secretLifecycle
    : LIFECYCLE expression SEMICOLON
    ;

secretClassification
    : CLASSIFICATION expression SEMICOLON
    ;

secretAttribute
    : ATTRIBUTE Identifier ASSIGN expression SEMICOLON
    ;

// -----------------------------------------------------------------------------
// Secret declaration body
// -----------------------------------------------------------------------------

secretMember
    : secretReference
    | secretRequirement
    | secretConstraint
    | secretRotation
    | secretExpiration
    | secretRevocation
    | secretAudit
    | secretExportPolicy
    | secretAttribute
    ;

// -----------------------------------------------------------------------------
// Secret references
// -----------------------------------------------------------------------------
//
// A reference identifies secret material without embedding the material itself.
//
// The actual resolution of the reference belongs to the semantic/runtime
// security system.
//

secretReference
    : SECRET_REF Identifier SEMICOLON
    ;

// -----------------------------------------------------------------------------
// Secret requirements
// -----------------------------------------------------------------------------
//
// Requirements express properties needed by a secret.
//
// They are semantic requirements, not implementation limits.
//
// Examples:
//
// requires capability("secure.secret_storage");
// requires capability("hardware_isolated_secret");
// requires storage("confidential");
//

secretRequirement
    : REQUIRES expression SEMICOLON
    ;

// -----------------------------------------------------------------------------
// Secret constraints
// -----------------------------------------------------------------------------
//
// Constraints express semantic restrictions.
//
// Examples:
//
// constraint expression
//
// The expression language remains the canonical Zamani expression language.
// No second security-specific expression language is introduced.
//

secretConstraint
    : CONSTRAINT expression SEMICOLON
    ;

// -----------------------------------------------------------------------------
// Secret rotation
// -----------------------------------------------------------------------------
//
// Rotation expresses lifecycle intent.
//
// The expression may describe a duration, event, policy, or other semantic
// condition. The grammar does not impose a fixed unit, interval, or count.
//

secretRotation
    : ROTATE expression SEMICOLON
    ;

// -----------------------------------------------------------------------------
// Secret expiration
// -----------------------------------------------------------------------------
//
// Expiration describes when secret material should no longer be considered
// valid.
//
// The actual clock/time semantics belong to the semantic/runtime layers.
//

secretExpiration
    : EXPIRES expression SEMICOLON
    ;

// -----------------------------------------------------------------------------
// Secret revocation
// -----------------------------------------------------------------------------
//
// Revocation expresses invalidation intent.
//
// It does not implement revocation or determine how dependent systems react.
//

secretRevocation
    : REVOKE expression SEMICOLON
    ;

// -----------------------------------------------------------------------------
// Secret auditing
// -----------------------------------------------------------------------------
//
// Audit expresses an audit requirement without exposing secret contents.
//
// The audit backend and redaction behavior belong to the security/runtime
// implementation.
//

secretAudit
    : AUDIT expression SEMICOLON
    ;

// -----------------------------------------------------------------------------
// Secret export / disclosure policy
// -----------------------------------------------------------------------------
//
// This expresses whether and under which semantic conditions a secret may
// cross a security boundary.
//
// Actual authorization is owned by authorization.g4 and the semantic
// authorization subsystem.
//

secretExportPolicy
    : EXPORT expression SEMICOLON
    ;

// -----------------------------------------------------------------------------
// Secret-use expressions
// -----------------------------------------------------------------------------
//
// A secret can be referenced from ordinary expressions without requiring
// secret bytes to become literals in the grammar.
//
// Example:
//
// use secret DatabaseCredential
//
// The actual expression/type representation is resolved by the semantic
// layer.
//

secretUse
    : USE SECRET Identifier
    ;

// -----------------------------------------------------------------------------
// Secret metadata / policy references
// -----------------------------------------------------------------------------
//
// These rules allow other security domains to refer to secret declarations
// without duplicating the declaration grammar.
//

secretIdentifier
    : Identifier
    ;
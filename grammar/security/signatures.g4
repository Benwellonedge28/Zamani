/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/security/signatures.g4
 *
 * ROLE
 * ----
 * Canonical parser grammar for cryptographic SIGNATURE intent.
 *
 * STATUS
 * ------
 * PRODUCTION SECURITY-DOMAIN GRAMMAR
 *
 * ============================================================================
 * IMPLEMENTATION BASELINE
 * ============================================================================
 *
 * Language:
 *     Zamani
 *
 * Grammar:
 *     ANTLR4 parser grammar
 *
 * Rust frontend/compiler:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Rust edition:
 *     Rust 2021
 *
 * Safety:
 *     Safe Rust only.
 *
 * This file contains no Rust actions, semantic predicates, or native-code
 * hooks. The grammar therefore imposes no unsafe-code requirement.
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
 *          +-------------------------------+
 *          |                               |
 *          v                               v
 *     cryptography                    other security domains
 *          |
 *          v
 *     signatures.g4
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic security analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *       +--+-------------------------------+
 *       |                                  |
 *       v                                  v
 * classical lowering                 quantum metadata
 *       |                                  |
 *       |                                  v
 *       |                              quantum::ir
 *       |                                  |
 *       +------------------+---------------+
 *                          |
 *                          v
 *                 optimization / lowering
 *                          |
 *                routing / scheduling
 *                          |
 *                    resilience / ZQN
 *                          |
 *                          v
 *                         HAL
 *                          |
 *                          v
 *                    target realization
 *
 * This grammar defines source-level signature intent.
 *
 * It does NOT implement digital signatures.
 *
 * ============================================================================
 * SINGLE-LEXER INVARIANT
 * ============================================================================
 *
 * This file is a parser grammar only.
 *
 * The canonical lexical authority remains:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * No lexer rules are defined here.
 *
 * In particular this file MUST NOT introduce:
 *
 *     SIGNATURE
 *     VERIFY
 *     SIGN
 *     ALGORITHM
 *     PRIVATE_KEY
 *     PUBLIC_KEY
 *     KEY_REFERENCE
 *     PROVIDER
 *     IMPLEMENTATION
 *     DOMAIN
 *     DIGEST
 *
 * as lexer tokens.
 *
 * Signature-specific concepts are represented by ordinary Zamani names,
 * qualified names, expressions, attributes, and semantic references.
 *
 * ============================================================================
 * OPEN-WORLD SIGNATURE MODEL
 * ============================================================================
 *
 * Signature mechanisms are OPEN-WORLD.
 *
 * This grammar MUST NOT enumerate:
 *
 *     RSA
 *     ECDSA
 *     Ed25519
 *     Ed448
 *     ML-DSA
 *     SLH-DSA
 *     future signature algorithms
 *
 * or any other finite algorithm catalogue.
 *
 * Instead, an algorithm is a semantic name.
 *
 * Examples:
 *
 *     cryptography::signature::algorithm
 *     cryptography::signature::ed25519
 *     cryptography::signature::future_algorithm
 *     organization::signature::scheme
 *     vendor::signature::implementation
 *
 * All have the same syntactic form.
 *
 * Semantic analysis determines:
 *
 *     - whether the mechanism exists;
 *     - whether it is approved;
 *     - whether it supports the requested operation;
 *     - whether its security properties are sufficient;
 *     - whether the required key material is available;
 *     - whether the target can realize it.
 *
 * ============================================================================
 * SECRET-MATERIAL INVARIANT
 * ============================================================================
 *
 * This grammar MUST NOT provide source syntax whose semantic purpose is
 * embedding private-key or secret-key material.
 *
 * The following conceptual constructs are NOT owned by this grammar:
 *
 *     private key literals
 *     secret key literals
 *     password literals
 *     seed literals
 *     credential literals
 *     authentication-token literals
 *
 * Signature operations refer to key objects or protected references.
 *
 * The security subsystem determines how those references are resolved.
 *
 * A parser MUST NOT retrieve, inspect, dereference, or execute key material.
 *
 * ============================================================================
 * SIGNING / VERIFICATION SEPARATION
 * ============================================================================
 *
 * Signing and verification are distinct semantic operations.
 *
 * Signing:
 *
 *     message
 *         +
 *     signing key reference
 *         +
 *     signature algorithm
 *         ->
 *     signature
 *
 * Verification:
 *
 *     message
 *         +
 *     signature
 *         +
 *     verification key reference
 *         +
 *     signature algorithm
 *         ->
 *     verification result
 *
 * The grammar keeps these operations structurally distinguishable.
 *
 * ============================================================================
 * HASH / SIGNATURE SEPARATION
 * ============================================================================
 *
 * A signature MAY consume:
 *
 *     a message;
 *     a digest;
 *     a canonicalized representation;
 *     a structured value;
 *     another semantic artifact.
 *
 * Hashing itself belongs to:
 *
 *     security/hashes.g4
 *
 * This grammar MUST NOT duplicate hash grammar.
 *
 * For example, the semantic pipeline may be:
 *
 *     value
 *       |
 *       v
 *     canonical representation
 *       |
 *       v
 *     hash
 *       |
 *       v
 *     signature
 *
 * but this file owns only the signature side of that pipeline.
 *
 * ============================================================================
 * SIGNATURE OBJECT MODEL
 * ============================================================================
 *
 * A signature operation may describe:
 *
 *     algorithm
 *     message
 *     digest
 *     signing key
 *     verification key
 *     signature output
 *     verification result
 *     encoding
 *     domain separation
 *     context
 *     canonicalization
 *     parameters
 *     properties
 *     requirements
 *     constraints
 *     preferences
 *     provider reference
 *     implementation reference
 *     metadata
 *
 * The semantic layer determines whether the combination is valid.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Signature syntax is target-independent.
 *
 * It imposes no universal limits on:
 *
 *     signatures
 *     messages
 *     algorithms
 *     keys
 *     key references
 *     verification operations
 *     providers
 *     implementations
 *     participants
 *     devices
 *     nodes
 *     processors
 *     accelerators
 *     memory
 *     storage
 *     threads
 *     quantum resources
 *
 * This grammar MUST NOT define:
 *
 *     MAX_SIGNATURES
 *     MAX_KEYS
 *     MAX_MESSAGES
 *     MAX_ALGORITHMS
 *     MAX_OPERATIONS
 *     MAX_NODES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_MEMORY
 *
 * as language-level limits.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * Source syntax MUST NOT require:
 *
 *     CPU identity
 *     GPU identity
 *     FPGA identity
 *     ASIC identity
 *     QPU identity
 *     HSM identity
 *     TPM identity
 *     secure-element identity
 *     physical device index
 *     physical memory-bank identity
 *     network-node identity
 *
 * Those belong to target realization.
 *
 * ============================================================================
 * REQUIREMENT / CONSTRAINT / PREFERENCE
 * ============================================================================
 *
 * Signature declarations distinguish:
 *
 *     requirement
 *     constraint
 *     preference
 *
 * A requirement is mandatory.
 *
 * A constraint limits permitted realizations.
 *
 * A preference expresses desirable realization characteristics.
 *
 * A preference MUST NOT silently become a requirement.
 *
 * ============================================================================
 * CANONICAL AST CONTRACT
 * ============================================================================
 *
 * Every signature construct must lower to a domain-neutral representation.
 *
 * Conceptually:
 *
 *     signature operation
 *         |
 *         v
 *     generic operation node
 *         |
 *         v
 *     semantic signature operation
 *
 * Suggested semantic fields:
 *
 *     operation_kind
 *     algorithm
 *     message
 *     digest
 *     signing_key
 *     verification_key
 *     output
 *     result
 *     encoding
 *     domain
 *     context
 *     parameters
 *     properties
 *     requirements
 *     constraints
 *     preferences
 *     provider
 *     implementation
 *     metadata
 *     source_span
 *
 * The exact Rust AST type belongs to the frontend AST contract.
 *
 * This grammar does not define Rust structs.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar does NOT define an IR.
 *
 * The intended lowering is:
 *
 *     SignatureSyntax
 *          |
 *          v
 *     Frontend AST
 *          |
 *          v
 *     Semantic Signature Operation
 *          |
 *          v
 *     canonical semantic IR
 *          |
 *          +----------------------+
 *          |                      |
 *          v                      v
 *     classical target       quantum metadata
 *
 * If signature metadata accompanies quantum computation, the canonical
 * quantum boundary remains:
 *
 *     quantum::ir
 *
 * This grammar MUST NOT create:
 *
 *     SignatureIR
 *     QuantumSignatureIR
 *     SecurityQuantumIR
 *
 * as competing intermediate representations.
 *
 * ============================================================================
 * STANDALONE ENTRY POINT
 * ============================================================================
 *
 * This rule exists for grammar-level tests and tooling.
 *
 * The complete Zamani parser MUST enter signature syntax through the security
 * composition root rather than requiring `signaturesFile` as its normal
 * program entry point.
 */

parser grammar Signatures;

options {
    tokenVocab = ZamaniLexer;
}


/* ========================================================================= */
/* GRAMMAR COMPOSITION                                                      */
/* ========================================================================= */

/*
 * These imports are semantic dependencies only.
 *
 * Core:
 *     names / qualified names / shared syntax
 *
 * Types:
 *     type references where signature output/result types require them
 *
 * Expressions:
 *     messages, parameters, requirements, constraints, properties, etc.
 *
 * No security grammar imports this file back.
 *
 * In particular:
 *
 *     signatures -> cryptography
 *
 * MUST NOT be introduced because cryptography is the composition owner.
 *
 * The intended direction is:
 *
 *     cryptography
 *         |
 *         +--> hashes
 *         |
 *         +--> signatures
 *
 * rather than:
 *
 *     signatures <--> cryptography
 *
 * =========================================================================
 */

/*
 * NOTE:
 *
 * The repository's canonical parser composition must resolve these imports
 * against its actual ANTLR grammar names.
 *
 * If the composition root already re-exports Core/Types/Expressions rules,
 * the generated parser may omit redundant imports at the composition layer.
 *
 * This file deliberately references only the existing canonical shared
 * grammar concepts and does not define replacement versions of them.
 */

import Core, Types, Expressions;


/* ========================================================================= */
/* PUBLIC ENTRY POINT                                                        */
/* ========================================================================= */

signaturesFile
    : signatureEntry* EOF
    ;

signatureEntry
    : signatureDeclaration
    | signatureOperation
    ;


/* ========================================================================= */
/* SIGNATURE DECLARATION                                                     */
/* ========================================================================= */

/*
 * A declaration creates a named semantic signature contract.
 *
 * Examples:
 *
 *     apply cryptography::signature::scheme {
 *         algorithm: cryptography::signature::algorithm;
 *     };
 *
 *     apply cryptography::signature::sign(message, key) {
 *         algorithm: cryptography::signature::scheme;
 *     };
 *
 * No algorithm is reserved by the grammar.
 */

signatureDeclaration
    : APPLY signatureQualifiedName
      signatureArgumentList?
      signatureBody?
      SEMICOLON?
    ;


/* ========================================================================= */
/* SIGNATURE OPERATION                                                       */
/* ========================================================================= */

/*
 * The operation name remains an open-world qualified name.
 *
 * This supports:
 *
 *     cryptography::signature::sign
 *     cryptography::signature::verify
 *     future::signature::operation
 *     vendor::signature::operation
 *
 * without modifying this grammar.
 *
 * Semantic analysis distinguishes signing, verification, aggregation,
 * derivation, or future operations according to the operation's registered
 * semantic contract.
 */

signatureOperation
    : APPLY signatureQualifiedName
      signatureArgumentList
      signatureBody?
      SEMICOLON?
    ;


/* ========================================================================= */
/* SIGNATURE NAME                                                            */
/* ========================================================================= */

/*
 * Qualified names remain owned by the core/name grammar.
 *
 * This rule intentionally does not enumerate:
 *
 *     sign
 *     verify
 *     aggregate
 *     batch
 *     threshold
 *
 * as lexer or parser keywords.
 */

signatureQualifiedName
    : qualifiedName
    ;


/* ========================================================================= */
/* ARGUMENTS                                                                 */
/* ========================================================================= */

signatureArgumentList
    : LPAREN
      signatureArgument*
      RPAREN
    ;

signatureArgument
    : expression
    ;

signatureArgumentListNonEmpty
    : expression
      (COMMA expression)*
    ;


/* ========================================================================= */
/* SIGNATURE BODY                                                            */
/* ========================================================================= */

signatureBody
    : LBRACE
      signatureMember*
      RBRACE
    ;

signatureMember
    : signatureAlgorithmClause
    | signatureMessageClause
    | signatureDigestClause
    | signatureSigningKeyClause
    | signatureVerificationKeyClause
    | signatureOutputClause
    | signatureResultClause
    | signatureEncodingClause
    | signatureDomainClause
    | signatureContextClause
    | signatureCanonicalizationClause
    | signatureParameterClause
    | signaturePropertyClause
    | signatureRequirementClause
    | signatureConstraintClause
    | signaturePreferenceClause
    | signatureProviderClause
    | signatureImplementationClause
    | signatureMetadataClause
    ;


/* ========================================================================= */
/* ALGORITHM                                                                 */
/* ========================================================================= */

/*
 * The algorithm is an open-world semantic reference.
 *
 * Example:
 *
 *     algorithm: cryptography::signature::future_scheme
 *
 * The grammar does not know whether the scheme exists.
 */

signatureAlgorithmClause
    : identifier COLON qualifiedName
    ;


/* ========================================================================= */
/* MESSAGE                                                                   */
/* ========================================================================= */

/*
 * The message is an ordinary Zamani expression.
 *
 * No message-size ceiling exists here.
 */

signatureMessageClause
    : identifier COLON expression
    ;


/* ========================================================================= */
/* DIGEST                                                                    */
/* ========================================================================= */

/*
 * A signature may operate over a digest supplied by another semantic
 * operation.
 *
 * This rule does not reimplement hashing.
 */

signatureDigestClause
    : identifier COLON expression
    ;


/* ========================================================================= */
/* SIGNING KEY                                                               */
/* ========================================================================= */

/*
 * A signing key is a semantic reference.
 *
 * It is not key material.
 */

signatureSigningKeyClause
    : identifier COLON expression
    ;


/* ========================================================================= */
/* VERIFICATION KEY                                                         */
/* ========================================================================= */

/*
 * A verification key is a semantic reference.
 *
 * It is not key material.
 */

signatureVerificationKeyClause
    : identifier COLON expression
    ;


/* ========================================================================= */
/* SIGNATURE OUTPUT                                                          */
/* ========================================================================= */

/*
 * Output intent remains an expression.
 *
 * The semantic layer determines whether the selected operation can produce
 * the requested representation.
 */

signatureOutputClause
    : identifier COLON expression
    ;


/* ========================================================================= */
/* VERIFICATION RESULT                                                      */
/* ========================================================================= */

/*
 * Verification result is deliberately semantic rather than a hard-coded
 * boolean grammar.
 *
 * This permits richer verification models such as:
 *
 *     valid
 *     invalid
 *     indeterminate
 *     policy-rejected
 *     unsupported
 *
 * without embedding those states into the parser.
 */

signatureResultClause
    : identifier COLON expression
    ;


/* ========================================================================= */
/* ENCODING                                                                  */
/* ========================================================================= */

/*
 * Encoding is an open semantic value.
 *
 * No fixed encoding catalogue is imposed.
 */

signatureEncodingClause
    : identifier COLON expression
    ;


/* ========================================================================= */
/* DOMAIN SEPARATION                                                         */
/* ========================================================================= */

/*
 * Domain separation is semantic intent.
 *
 * The domain may be a qualified semantic name or another expression.
 */

signatureDomainClause
    : identifier COLON expression
    ;


/* ========================================================================= */
/* CONTEXT                                                                   */
/* ========================================================================= */

signatureContextClause
    : identifier COLON expression
    ;


/* ========================================================================= */
/* CANONICALIZATION                                                          */
/* ========================================================================= */

/*
 * Canonicalization determines the semantic representation that is signed or
 * verified.
 *
 * Serialization implementation belongs elsewhere.
 */

signatureCanonicalizationClause
    : identifier COLON expression
    ;


/* ========================================================================= */
/* PARAMETERS                                                                */
/* ========================================================================= */

/*
 * Signature parameters remain expressions.
 *
 * No fixed parameter vocabulary is imposed.
 */

signatureParameterClause
    : identifier COLON expression
    ;


/* ========================================================================= */
/* SECURITY PROPERTIES                                                       */
/* ========================================================================= */

/*
 * Properties are semantic assertions/requests.
 *
 * Examples:
 *
 *     property: cryptography::signature::property
 *     property: security::required_strength
 *
 * The grammar does not decide whether a property is satisfied.
 */

signaturePropertyClause
    : identifier COLON expression
    ;


/* ========================================================================= */
/* REQUIREMENTS                                                              */
/* ========================================================================= */

/*
 * Requirement semantics belong to semantic/resource analysis.
 *
 * Examples:
 *
 *     requires: capability("signature.verify")
 *     requires: capability("constant_time")
 *     requires: capability("post_quantum")
 *
 * The exact capability vocabulary remains open-world.
 */

signatureRequirementClause
    : REQUIRES COLON expression
    ;


/* ========================================================================= */
/* CONSTRAINTS                                                               */
/* ========================================================================= */

/*
 * Constraints limit realizations without identifying a specific physical
 * target.
 */

signatureConstraintClause
    : identifier COLON expression
    ;


/* ========================================================================= */
/* PREFERENCES                                                               */
/* ========================================================================= */

/*
 * Preferences are advisory.
 *
 * Semantic analysis MUST preserve the distinction between preference and
 * requirement.
 */

signaturePreferenceClause
    : identifier COLON expression
    ;


/* ========================================================================= */
/* PROVIDER                                                                  */
/* ========================================================================= */

/*
 * Provider references are opaque semantic references.
 *
 * They do not perform provider discovery.
 */

signatureProviderClause
    : identifier COLON qualifiedName
    ;


/* ========================================================================= */
/* IMPLEMENTATION                                                            */
/* ========================================================================= */

/*
 * Implementation references are opaque semantic references.
 *
 * They do not load or execute an implementation.
 */

signatureImplementationClause
    : identifier COLON qualifiedName
    ;


/* ========================================================================= */
/* METADATA                                                                  */
/* ========================================================================= */

/*
 * Metadata remains ordinary syntax.
 *
 * Interpretation belongs to semantic analysis.
 */

signatureMetadataClause
    : identifier COLON expression
    ;


/* ========================================================================= */
/* REUSABLE CONTRACT RULES                                                   */
/* ========================================================================= */

/*
 * These façade rules allow the cryptography/security composition grammars to
 * reference signature concepts without copying their grammar.
 */

signatureAlgorithmReference
    : qualifiedName
    ;

signatureKeyReference
    : expression
    ;

signatureProviderReference
    : qualifiedName
    ;

signatureImplementationReference
    : qualifiedName
    ;

signatureProperty
    : signaturePropertyClause
    ;

signatureRequirement
    : signatureRequirementClause
    ;

signatureConstraint
    : signatureConstraintClause
    ;

signaturePreference
    : signaturePreferenceClause
    ;


/* ========================================================================= */
/* SIGN / VERIFY SEMANTIC CONTRACT                                           */
/* ========================================================================= */

/*
 * The parser intentionally does not distinguish `sign` and `verify` through
 * reserved words.
 *
 * Instead, the semantic registry interprets:
 *
 *     cryptography::signature::sign
 *     cryptography::signature::verify
 *
 * or future equivalent qualified operations.
 *
 * This gives Zamani an open-world signature namespace.
 *
 * The semantic layer MUST nevertheless establish operation-specific contracts.
 *
 * For signing, the semantic contract should establish:
 *
 *     message
 *     signing key
 *     algorithm
 *     output
 *
 * For verification:
 *
 *     message
 *     signature
 *     verification key
 *     algorithm
 *     result
 *
 * Missing or contradictory fields are semantic errors, not parser errors.
 */


/* ========================================================================= */
/* SIGNATURE / HASH INTEGRATION                                              */
/* ========================================================================= */

/*
 * Hashing remains owned by:
 *
 *     security/hashes.g4
 *
 * A signature may consume a digest expression, for example:
 *
 *     apply cryptography::signature::sign(digest) {
 *         algorithm: cryptography::signature::scheme;
 *     };
 *
 * The parser does not determine whether `digest` was produced by:
 *
 *     security/hashes.g4
 *
 * another library operation, an imported artifact, or another valid source.
 *
 * Semantic analysis determines compatibility.
 *
 * This prevents duplicate hash syntax and keeps:
 *
 *     hashes.g4
 *
 * and:
 *
 *     signatures.g4
 *
 * independently maintainable.
 */


/* ========================================================================= */
/* SIGNATURE / IDENTITY INTEGRATION                                           */
/* ========================================================================= */

/*
 * Identity is NOT owned here.
 *
 * A key may be associated semantically with an identity/principal, but this
 * grammar must not redefine identity syntax.
 *
 * Integration direction:
 *
 *     identity
 *         |
 *         v
 *     key reference
 *         |
 *         v
 *     signature operation
 *
 * Identity resolution belongs to the security identity subsystem.
 */


/* ========================================================================= */
/* SIGNATURE / AUTHORIZATION INTEGRATION                                     */
/* ========================================================================= */

/*
 * Authorization is NOT signature semantics.
 *
 * A valid signature does not automatically imply authorization.
 *
 * Authorization policy is owned by the authorization/policy grammars and
 * semantic security layer.
 *
 * This grammar may carry references to such policies through ordinary
 * expressions or metadata.
 */


/* ========================================================================= */
/* SIGNATURE / KEY MANAGEMENT INTEGRATION                                    */
/* ========================================================================= */

/*
 * Key lifecycle is owned by key-management/secrets grammars.
 *
 * This file only consumes key references.
 *
 * The semantic compiler/runtime may resolve:
 *
 *     key handles
 *     public-key references
 *     protected key references
 *     external key objects
 *
 * according to security policy.
 *
 * This grammar never receives raw private-key material.
 */


/* ========================================================================= */
/* SIGNATURE / HARDWARE INTEGRATION                                          */
/* ========================================================================= */

/*
 * A signature may require a capability such as:
 *
 *     capability("secure-key-storage")
 *     capability("hardware-signature")
 *     capability("constant-time")
 *
 * but the grammar does not name:
 *
 *     TPM 2.0 device N
 *     HSM device N
 *     secure element N
 *     CPU N
 *     GPU N
 *
 * Physical realization belongs downstream.
 */


/* ========================================================================= */
/* SIGNATURE / QUANTUM INTEGRATION                                          */
/* ========================================================================= */

/*
 * Signature semantics may participate in quantum-classical programs.
 *
 * Examples include:
 *
 *     signing measurement data;
 *     verifying signed classical control information;
 *     signing circuit provenance;
 *     signing compiled artifacts;
 *     verifying authenticated classical messages used by quantum workflows.
 *
 * The grammar does not define quantum operations.
 *
 * If signature metadata accompanies a quantum computation, it flows through:
 *
 *     semantic model
 *          |
 *          v
 *     quantum::ir
 *
 * No quantum-specific signature IR is introduced.
 */


/* ========================================================================= */
/* SIGNATURE / HDL INTEGRATION                                               */
/* ========================================================================= */

/*
 * Signature intent may describe integrity/provenance of:
 *
 *     firmware
 *     HDL artifacts
 *     FPGA bitstreams
 *     ASIC build artifacts
 *     hardware configuration
 *
 * The grammar does not define HDL constructs.
 *
 * HDL remains owned by:
 *
 *     grammar/hdl/
 *
 * Hardware realization remains owned by:
 *
 *     grammar/hardware/
 */


/* ========================================================================= */
/* SIGNATURE / DISTRIBUTED COMPUTING                                        */
/* ========================================================================= */

/*
 * Signature operations may be executed:
 *
 *     locally;
 *     remotely;
 *     in parallel;
 *     as part of distributed protocols;
 *     through accelerators.
 *
 * The source-level signature meaning must remain independent of topology.
 *
 * No fixed node count, participant count, or communication topology is
 * permitted here.
 */


/* ========================================================================= */
/* SIGNATURE / AI AND DATA                                                  */
/* ========================================================================= */

/*
 * Signature intent may be used for:
 *
 *     model provenance;
 *     dataset provenance;
 *     artifact integrity;
 *     model identity;
 *     cache/artifact verification;
 *     signed training artifacts;
 *     signed inference inputs/outputs.
 *
 * AI/data grammar remains responsible for AI/data structures.
 *
 * This grammar owns only signature semantics.
 */


/* ========================================================================= */
/* SIGNATURE / NETWORKING                                                   */
/* ========================================================================= */

/*
 * Signatures may authenticate:
 *
 *     messages;
 *     requests;
 *     responses;
 *     protocol artifacts;
 *     distributed state.
 *
 * Networking grammar remains responsible for:
 *
 *     endpoints;
 *     sockets;
 *     addresses;
 *     transport;
 *     routing.
 *
 * This grammar does not redefine those concepts.
 */


/* ========================================================================= */
/* SIGNATURE / RESOURCE AND CAPABILITY MODEL                                */
/* ========================================================================= */

/*
 * Resource and capability decisions are downstream.
 *
 * Valid source intent may state requirements without specifying hardware:
 *
 *     requires: capability("signature.sign")
 *     requires: capability("signature.verify")
 *     requires: capability("constant_time")
 *
 * or other registered capabilities.
 *
 * A resource requirement may depend on a symbolic expression.
 *
 * No fixed resource maximum belongs here.
 */


/* ========================================================================= */
/* DETERMINISM                                                              */
/* ========================================================================= */

/*
 * Parsing this grammar MUST be deterministic.
 *
 * It MUST NOT depend on:
 *
 *     current time;
 *     filesystem contents;
 *     network state;
 *     provider availability;
 *     hardware state;
 *     runtime state;
 *     random values.
 *
 * Provider lookup, key resolution, capability discovery, and target selection
 * are semantic/compiler/runtime concerns and occur after parsing.
 */


/* ========================================================================= */
/* SOURCE-SPAN PRESERVATION                                                  */
/* ========================================================================= */

/*
 * The frontend must preserve source locations for:
 *
 *     operation name;
 *     algorithm;
 *     message;
 *     digest;
 *     signing key;
 *     verification key;
 *     signature output;
 *     verification result;
 *     requirements;
 *     constraints;
 *     preferences;
 *     properties;
 *     metadata.
 *
 * Diagnostics must point back to the smallest relevant source construct.
 */


/* ========================================================================= */
/* DIAGNOSTIC CONTRACT                                                       */
/* ========================================================================= */

/*
 * Parser diagnostics:
 *
 *     malformed signature syntax
 *     malformed argument list
 *     malformed body
 *     malformed expression
 *
 * Semantic diagnostics:
 *
 *     unknown signature operation
 *     unknown algorithm
 *     incompatible key
 *     missing signing key
 *     missing verification key
 *     missing signature input
 *     incompatible digest
 *     unsupported encoding
 *     unsupported domain
 *     unsatisfied requirement
 *     conflicting constraint
 *     unavailable capability
 *     unavailable implementation
 *
 * These categories MUST NOT be conflated.
 */


/* ========================================================================= */
/* SECURITY INVARIANTS                                                       */
/* ========================================================================= */

/*
 * This grammar itself:
 *
 *     - never evaluates expressions;
 *     - never accesses keys;
 *     - never accesses credentials;
 *     - never performs signing;
 *     - never performs verification;
 *     - never loads providers;
 *     - never performs network access;
 *     - never discovers hardware;
 *     - never chooses a device;
 *     - never executes external code.
 *
 * Security enforcement belongs to semantic analysis, compilation, runtime,
 * policy, and the appropriate secure execution subsystem.
 */


/* ========================================================================= */
/* SCALABILITY CONTRACT                                                      */
/* ========================================================================= */

/*
 * The grammar uses recursive/list structures rather than fixed capacities.
 *
 * Therefore it does not establish language limits for:
 *
 *     number of signatures;
 *     number of operations;
 *     number of arguments;
 *     number of properties;
 *     number of requirements;
 *     number of constraints;
 *     number of metadata entries;
 *     qualified-name depth;
 *     source-program size.
 *
 * Practical limits are implementation/resource limits.
 *
 * The compiler must use dynamically sized representations and must not turn
 * these grammar constructs into fixed-size arrays solely for convenience.
 */


/* ========================================================================= */
/* NO HARD-CODED ALGORITHM POLICY                                            */
/* ========================================================================= */

/*
 * Adding a new signature scheme MUST NOT require editing this grammar.
 *
 * The normal extension path is:
 *
 *     algorithm registry / semantic specification
 *              |
 *              v
 *     capability contract
 *              |
 *              v
 *     implementation/provider
 *              |
 *              v
 *     compiler/backend
 *
 * not:
 *
 *     edit signatures.g4
 *
 * unless the NEW FEATURE introduces genuinely new language semantics rather
 * than a new cryptographic algorithm.
 */


/* ========================================================================= */
/* COMPATIBILITY CONTRACT                                                    */
/* ========================================================================= */

/*
 * This grammar deliberately introduces no new lexer token.
 *
 * Therefore algorithm evolution is not a lexical compatibility event.
 *
 * If the language specification later chooses to reserve a word such as
 * `signature`, that is a separate language-versioning change:
 *
 *     specification
 *          |
 *          v
 *     lexical contract
 *          |
 *          v
 *     keyword registry
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     parser compatibility
 *
 * This file must not silently change the lexical contract.
 */


/* ========================================================================= */
/* TEST CONTRACT                                                             */
/* ========================================================================= */

/*
 * Positive syntax cases:
 *
 *     apply cryptography::signature::sign(message, key);
 *
 *     apply cryptography::signature::verify(message, signature, public_key);
 *
 *     apply cryptography::signature::sign(message, key) {
 *         algorithm: cryptography::signature::scheme;
 *     };
 *
 *     apply cryptography::signature::verify(message, signature, public_key) {
 *         algorithm: future::signature::algorithm;
 *     };
 *
 *     apply cryptography::signature::sign(message, key) {
 *         digest: cryptography::hash::value;
 *     };
 *
 *     apply cryptography::signature::verify(message, signature, public_key) {
 *         requires: capability("signature.verify");
 *     };
 *
 *     apply cryptography::signature::sign(message, key) {
 *         domain: application::identity;
 *         context: application::protocol;
 *     };
 *
 *     apply organization::future::signature::operation(value) {
 *         implementation: organization::implementation;
 *     };
 *
 * Negative parser cases:
 *
 *     malformed invocation;
 *     malformed argument list;
 *     malformed body;
 *     malformed qualified name;
 *     malformed member expression.
 *
 * Negative semantic cases:
 *
 *     unknown algorithm;
 *     incompatible key;
 *     missing signing key;
 *     missing verification key;
 *     incompatible message;
 *     incompatible digest;
 *     unsupported encoding;
 *     unsupported parameter;
 *     unsatisfied capability;
 *     conflicting requirement/constraint;
 *     unavailable implementation.
 */


/* ========================================================================= */
/* BOUNDARY TEST CONTRACT                                                    */
/* ========================================================================= */

/*
 * Boundary tests must include:
 *
 *     empty signature body;
 *     one argument;
 *     many arguments;
 *     deeply qualified names;
 *     many metadata members;
 *     large symbolic expressions;
 *     nested signature expressions;
 *     composed hash/signature operations;
 *     distributed verification intent;
 *     hardware capability requirements;
 *     quantum-associated metadata.
 *
 * Boundary tests MUST NOT establish an artificial language maximum.
 */


/* ========================================================================= */
/* SCALABILITY TEST CONTRACT                                                */
/* ========================================================================= */

/*
 * Scale tests across:
 *
 *     tiny programs;
 *     many signature operations;
 *     many independent signature declarations;
 *     large expressions;
 *     large qualified names;
 *     large metadata sets;
 *     distributed programs;
 *     heterogeneous target descriptions.
 *
 * "Infinity" means:
 *
 *     no arbitrary language-imposed finite ceiling.
 *
 * It does NOT mean an implementation can consume literally infinite memory.
 */


/* ========================================================================= */
/* HARD-CODING AUDIT                                                         */
/* ========================================================================= */

/*
 * Forbidden universal limits include:
 *
 *     MAX_SIGNATURES
 *     MAX_KEYS
 *     MAX_ALGORITHMS
 *     MAX_MESSAGE_SIZE
 *     MAX_SIGNATURE_SIZE
 *     MAX_OPERATIONS
 *     MAX_NODES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_MEMORY
 *     MAX_THREADS
 *
 * Also forbidden as language-level target assumptions:
 *
 *     cpu0
 *     gpu0
 *     fpga0
 *     qpu0
 *     node0
 *     key0
 *     physical_device0
 *
 * Numeric constants appearing in source expressions are not themselves
 * forbidden. The prohibition applies to compiler/grammar capacity limits.
 */


/* ========================================================================= */
/* SAFE-RUST CONTRACT                                                        */
/* ========================================================================= */

/*
 * The grammar itself contains no Rust.
 *
 * The Rust implementation integrating it must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and must not require:
 *
 *     unsafe
 *     unsafe blocks
 *     unsafe traits
 *     unsafe implementations
 *
 * The surrounding Rust crate should preferably enforce:
 *
 *     #![forbid(unsafe_code)]
 */


/* ========================================================================= */
/* COMPLETION CRITERIA                                                       */
/* ========================================================================= */

/*
 * signatures.g4 is complete when:
 *
 * [x] It is parser-only.
 * [x] It uses the canonical Zamani lexer boundary.
 * [x] It does not create a second lexer.
 * [x] It does not enumerate signature algorithms.
 * [x] It does not encode hardware limits.
 * [x] It does not embed key material.
 * [x] It does not own identity.
 * [x] It does not own authorization.
 * [x] It does not own key management.
 * [x] It does not duplicate hash grammar.
 * [x] It does not create another expression grammar.
 * [x] It does not create another resource system.
 * [x] It does not create another quantum IR.
 * [x] Signing and verification remain semantically distinguishable.
 * [x] Future signature schemes remain representable.
 * [x] Provider names remain open-world.
 * [x] Implementation names remain open-world.
 * [x] Requirements remain separate from preferences.
 * [x] Constraints remain separate from requirements.
 * [x] Source spans can be preserved.
 * [x] Parsing remains deterministic.
 * [x] No Rust actions exist.
 * [x] No semantic predicates exist.
 * [x] No unsafe implementation is required.
 * [x] Classical integration is defined.
 * [x] Quantum integration is defined.
 * [x] HDL/hardware integration is defined.
 * [x] AI/data integration is defined.
 * [x] Networking integration is defined.
 * [x] Distributed integration is defined.
 * [x] Resource/capability integration is defined.
 * [x] Positive tests are defined.
 * [x] Negative tests are defined.
 * [x] Boundary tests are defined.
 * [x] Scalability tests are defined.
 * [x] Compatibility is defined.
 * [x] AST integration is defined.
 * [x] Semantic integration is defined.
 * [x] IR integration is defined.
 * [x] Runtime boundaries are defined.
 */


/* ========================================================================= */
/* FINAL ARCHITECTURAL INVARIANT                                             */
/* ========================================================================= */

/*
 * A Zamani signature describes portable cryptographic intent:
 *
 *     WHAT is signed
 *     WHAT is verified
 *     WHICH semantic algorithm is required
 *     WHICH key reference is used
 *     WHICH properties are required
 *     WHICH capabilities are required
 *     WHICH constraints apply
 *
 * It does NOT dictate:
 *
 *     WHICH CPU
 *     WHICH GPU
 *     WHICH FPGA
 *     WHICH ASIC
 *     WHICH QPU
 *     WHICH HSM
 *     WHICH physical device
 *     WHICH network node
 *
 * Target realization remains downstream.
 *
 * Therefore signature syntax participates in:
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
 * subject to the actual semantic requirements, security policy, available
 * capabilities, and resources of the execution environment.
 *
 * ============================================================================
 */
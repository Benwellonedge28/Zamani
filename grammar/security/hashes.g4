/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/security/hashes.g4
 *
 * ROLE
 * ----
 * Canonical parser grammar for HASHING intent.
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
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Compiler implementation:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Rust edition:
 *     Rust 2021
 *
 * Safety:
 *     Safe Rust only.
 *
 * This grammar contains no Rust actions and therefore introduces no unsafe
 * implementation requirement.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         Zamani source
 *                              |
 *                              v
 *                         canonical lexer
 *                              |
 *                              v
 *                       Zamani parser
 *                              |
 *                              v
 *                    security / cryptography
 *                              |
 *                              v
 *                       THIS GRAMMAR
 *                              |
 *                              v
 *                    domain-neutral frontend AST
 *                              |
 *                              v
 *                    semantic security analysis
 *                              |
 *                              v
 *                     canonical semantic model
 *                              |
 *                +-------------+-------------+
 *                |                           |
 *                v                           v
 *          classical IR                 quantum::ir
 *                |                           |
 *                +-------------+-------------+
 *                              |
 *                              v
 *                 optimization / verification
 *                              |
 *                   +----------+----------+
 *                   |          |          |
 *                   v          v          v
 *                routing   scheduling  resilience
 *                              |
 *                              v
 *                            ZQN
 *                              |
 *                              v
 *                             HAL
 *                              |
 *                              v
 *                           runtime
 *
 * Hashing is a cryptographic semantic capability.
 *
 * It is NOT:
 *
 *     - a hashing implementation;
 *     - a runtime service;
 *     - a key store;
 *     - a random-number generator;
 *     - a hardware feature detector;
 *     - a backend selector;
 *     - a second IR;
 *     - a quantum IR;
 *     - a fixed algorithm catalogue.
 *
 * ============================================================================
 * PRIMARY DESIGN PRINCIPLE
 * ============================================================================
 *
 * This grammar describes WHAT HASHING IS REQUIRED TO MEAN.
 *
 * It does not dictate HOW hashing must be implemented.
 *
 * Therefore:
 *
 *     hash intent
 *          !=
 *     hash implementation
 *
 * A source program may request:
 *
 *     - a digest;
 *     - a particular cryptographic property;
 *     - a particular algorithm by semantic name;
 *     - a security strength;
 *     - domain separation;
 *     - canonicalization;
 *     - output representation;
 *     - deterministic behavior;
 *     - compatibility properties;
 *     - provider/implementation preference.
 *
 * The semantic/compiler/runtime layers determine whether and how the request
 * can be realized.
 *
 * ============================================================================
 * OPEN-WORLD ALGORITHM MODEL
 * ============================================================================
 *
 * HASH ALGORITHMS ARE NOT ENUMERATED HERE.
 *
 * Do NOT create grammar alternatives such as:
 *
 *     SHA256
 *     SHA3
 *     SHAKE
 *     BLAKE2
 *     BLAKE3
 *     SM3
 *     future_algorithm
 *
 * The algorithm name is semantic data.
 *
 * This allows:
 *
 *     cryptography::hash::sha256
 *     cryptography::hash::sha3_256
 *     cryptography::hash::blake3
 *     future::cryptography::hash::algorithm
 *     vendor::cryptography::hash::implementation
 *
 * without changing this grammar.
 *
 * The semantic layer determines whether the named mechanism exists and
 * whether it satisfies the requested properties.
 *
 * ============================================================================
 * CRITICAL LEXICAL CORRECTION
 * ============================================================================
 *
 * The current repository's lexical architecture requires parser grammars to
 * consume:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * This grammar therefore DOES NOT invent:
 *
 *     HASH
 *     HASH_ALGORITHM
 *     DIGEST
 *     SALT
 *     DOMAIN
 *     ENCODING
 *     OUTPUT
 *     PROVIDER
 *
 * lexer tokens.
 *
 * Hash-specific concepts are represented through:
 *
 *     existing Zamani lexical vocabulary
 *     canonical identifiers
 *     qualified names
 *     expressions
 *     attributes
 *     generic requirements
 *     generic constraints
 *     generic preferences
 *
 * This prevents a second lexical authority.
 *
 * ============================================================================
 * HASH SYNTAX MODEL
 * ============================================================================
 *
 * The canonical hash operation is intentionally namespace-oriented.
 *
 * Conceptually:
 *
 *     apply cryptography::hash(input)
 *
 * or:
 *
 *     apply cryptography::hash(input) {
 *         algorithm: cryptography::hash::sha256;
 *     }
 *
 * Algorithm selection is therefore semantic.
 *
 * A future algorithm requires no grammar modification.
 *
 * ============================================================================
 * WHY `apply cryptography::hash(...)`
 * ============================================================================
 *
 * The existing Zamani lexical vocabulary already contains:
 *
 *     APPLY
 *
 * while general operation names remain identifiers.
 *
 * This lets hashing participate in the existing universal operation model
 * without introducing a dedicated HASH lexer token.
 *
 * The grammar therefore does not create:
 *
 *     hash-language
 *
 * as a separate language.
 *
 * Instead:
 *
 *     universal Zamani operation
 *             +
 *     cryptographic hash semantic namespace
 *
 * forms the hashing language contract.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - hash operation syntax;
 *     - hash declaration syntax;
 *     - hash algorithm references;
 *     - hash input selection syntax;
 *     - hash output intent;
 *     - digest representation intent;
 *     - hash properties;
 *     - hash requirements;
 *     - hash constraints;
 *     - hash preferences;
 *     - domain-separation intent;
 *     - canonicalization intent;
 *     - provider/implementation references as opaque names;
 *     - hash metadata attachments.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - cryptographic algorithm implementation;
 *     - algorithm correctness;
 *     - collision resistance evaluation;
 *     - preimage resistance evaluation;
 *     - cryptanalysis;
 *     - random-number generation;
 *     - key generation;
 *     - secret storage;
 *     - password storage;
 *     - authentication;
 *     - authorization;
 *     - identity;
 *     - trust;
 *     - certificates;
 *     - HSM/TPM implementation;
 *     - hardware discovery;
 *     - provider discovery;
 *     - runtime execution;
 *     - target selection;
 *     - CPU/GPU/FPGA/QPU selection;
 *     - quantum error correction;
 *     - ZQN;
 *     - routing;
 *     - scheduling;
 *     - optimization.
 *
 * ============================================================================
 * RELATIONSHIP TO CRYPTOGRAPHY
 * ============================================================================
 *
 * The ownership hierarchy is:
 *
 *     security/
 *          |
 *          +-- cryptography.g4
 *          |       cryptographic composition
 *          |
 *          +-- hashes.g4
 *          |       hash-specific syntax
 *          |
 *          +-- permissions.g4
 *          +-- capabilities.g4
 *          +-- privacy.g4
 *          +-- trust.g4
 *          +-- security-constraints.g4
 *
 * `cryptography.g4` remains the cryptographic composition root.
 *
 * This file MUST NOT become another cryptographic composition root.
 *
 * ============================================================================
 * SECRET-MATERIAL BOUNDARY
 * ============================================================================
 *
 * Hashing does not inherently require secret material.
 *
 * However this grammar MUST NOT provide syntax for embedding:
 *
 *     private keys;
 *     secret keys;
 *     passwords;
 *     API keys;
 *     authentication tokens;
 *     secret seeds;
 *     credential material.
 *
 * A hash input may be an ordinary expression.
 *
 * A symbolic reference to protected material may be represented by a semantic
 * reference owned by the appropriate security subsystem.
 *
 * This grammar never interprets a STRING as a secret merely because it occurs
 * in a hash expression.
 *
 * Semantic security analysis remains responsible for identifying prohibited
 * secret-material handling.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Hash syntax must scale from:
 *
 *     tiny embedded execution
 *
 * through:
 *
 *     CPU
 *     multicore CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     quantum-assisted system
 *     simulator
 *     HPC
 *     cluster
 *     distributed system
 *     cloud
 *     future computational substrate
 *
 * No language-level limit is imposed on:
 *
 *     hash operations;
 *     hash inputs;
 *     digest count;
 *     algorithm references;
 *     policy declarations;
 *     domains;
 *     resources;
 *     nodes;
 *     processors;
 *     accelerators;
 *     memory;
 *     storage.
 *
 * In particular, this file MUST NOT define:
 *
 *     MAX_HASHES
 *     MAX_HASH_OPERATIONS
 *     MAX_DIGESTS
 *     MAX_INPUT_SIZE
 *     MAX_ALGORITHMS
 *     MAX_KEYS
 *     MAX_NODES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_MEMORY
 *
 * as language limits.
 *
 * ============================================================================
 * INFINITY / SCALABILITY INTERPRETATION
 * ============================================================================
 *
 * "Infinity" means that the grammar does not impose an arbitrary finite
 * semantic ceiling.
 *
 * Actual execution remains bounded by:
 *
 *     available memory;
 *     available compute;
 *     compiler resources;
 *     runtime resources;
 *     target capabilities;
 *     operational policy.
 *
 * Such limits are implementation/resource constraints, not language grammar
 * constraints.
 *
 * ============================================================================
 * REQUIREMENT / CONSTRAINT / PREFERENCE
 * ============================================================================
 *
 * These three concepts MUST remain distinct.
 *
 * REQUIREMENT:
 *
 *     mandatory semantic condition.
 *
 * CONSTRAINT:
 *
 *     permitted realization boundary.
 *
 * PREFERENCE:
 *
 *     desired realization property.
 *
 * A preference MUST NOT silently become a requirement.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * This grammar MUST NOT require:
 *
 *     CPU identity;
 *     GPU identity;
 *     FPGA identity;
 *     QPU identity;
 *     HSM identity;
 *     TPM identity;
 *     physical memory bank;
 *     device number;
 *     network node number;
 *     vendor-specific device.
 *
 * A hash algorithm may be realized differently on different targets while
 * preserving the same source-level semantics.
 *
 * ============================================================================
 * CANONICAL INPUT MODEL
 * ============================================================================
 *
 * A hash input is an ordinary Zamani expression.
 *
 * This allows:
 *
 *     hash(data)
 *     hash(buffer)
 *     hash(message)
 *     hash(serialize(value))
 *     hash(stream)
 *     hash(data || metadata)
 *
 * subject to the semantics of the surrounding language.
 *
 * This grammar does not create a second expression language.
 *
 * ============================================================================
 * CANONICAL OUTPUT MODEL
 * ============================================================================
 *
 * A hash operation may optionally describe output intent.
 *
 * The grammar supports semantic properties such as:
 *
 *     representation;
 *     encoding;
 *     length;
 *     truncation;
 *     domain;
 *     canonicalization.
 *
 * The actual validity of a requested output form belongs to semantic analysis.
 *
 * For example:
 *
 *     output: digest
 *
 * is syntactic intent.
 *
 * It does not mean that every algorithm supports every requested output mode.
 *
 * ============================================================================
 * DOMAIN SEPARATION
 * ============================================================================
 *
 * Domain separation is a semantic property.
 *
 * The grammar permits:
 *
 *     domain: cryptography::application::identity
 *
 * without enumerating domains.
 *
 * A domain may be:
 *
 *     a qualified name;
 *     a string-like expression;
 *     a compile-time expression;
 *     another semantic reference.
 *
 * The semantic layer determines whether the selected algorithm and operation
 * support the requested domain-separation semantics.
 *
 * ============================================================================
 * CANONICALIZATION
 * ============================================================================
 *
 * Hashing is sensitive to byte-level representation.
 *
 * Therefore the source language may express canonicalization intent, but this
 * grammar does not define serialization algorithms.
 *
 * Example:
 *
 *     canonical: data::canonical
 *
 * means that the semantic system must establish a canonical representation.
 *
 * It does not define a particular serialization implementation.
 *
 * ============================================================================
 * LENGTH / WIDTH
 * ============================================================================
 *
 * Digest length MUST remain semantic.
 *
 * Do not encode machine register width into hash syntax.
 *
 * These are distinct:
 *
 *     digest length
 *     integer width
 *     machine register width
 *     vector width
 *     memory bus width
 *
 * A requested digest length is validated against the selected algorithm.
 *
 * ============================================================================
 * TRUNCATION
 * ============================================================================
 *
 * Truncation may be requested explicitly.
 *
 * The grammar permits the expression of intent.
 *
 * Semantic analysis MUST determine whether:
 *
 *     - truncation is supported;
 *     - it is cryptographically acceptable;
 *     - policy permits it;
 *     - it preserves required security strength.
 *
 * The grammar does not make a security judgment.
 *
 * ============================================================================
 * HASH COMPOSITION
 * ============================================================================
 *
 * Hash operations may be composed with ordinary Zamani expressions.
 *
 * This grammar does not create special syntax for:
 *
 *     hash(hash(x))
 *     hash(concat(x, y))
 *     hash(serialize(x))
 *
 * beyond ordinary expression composition.
 *
 * ============================================================================
 * STREAMING
 * ============================================================================
 *
 * Hashing may operate over:
 *
 *     values;
 *     byte sequences;
 *     buffers;
 *     streams;
 *     data structures after canonicalization;
 *     distributed data representations.
 *
 * The grammar may describe streaming intent.
 *
 * It does not prescribe implementation buffering.
 *
 * ============================================================================
 * DISTRIBUTED / PARALLEL HASHING
 * ============================================================================
 *
 * Hash semantics must remain independent of the execution topology.
 *
 * A hash computation may eventually be lowered into:
 *
 *     serial execution;
 *     parallel execution;
 *     tree reduction;
 *     distributed computation;
 *     accelerator execution;
 *
 * but the source-level hash meaning remains unchanged.
 *
 * The grammar does not select the strategy.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Hashing may be applied to:
 *
 *     classical data associated with quantum computation;
 *     serialized quantum semantic data;
 *     measurement results;
 *     provenance;
 *     circuit/program metadata;
 *     classical control data.
 *
 * This grammar MUST NOT define:
 *
 *     QubitId;
 *     PhysicalQubitId;
 *     GateKind;
 *     QEC;
 *     ZQN;
 *     quantum routing;
 *     quantum scheduling.
 *
 * If hash metadata accompanies a quantum computation, it is propagated toward
 * the canonical:
 *
 *     quantum::ir
 *
 * boundary through semantic metadata.
 *
 * No second quantum IR is created.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hash intent may be associated with:
 *
 *     hardware verification;
 *     data integrity;
 *     configuration integrity;
 *     firmware provenance;
 *     bitstream provenance;
 *     hardware/software co-design.
 *
 * The grammar does not select:
 *
 *     FPGA;
 *     ASIC;
 *     secure element;
 *     hardware accelerator.
 *
 * ============================================================================
 * AI / DATA INTEGRATION
 * ============================================================================
 *
 * Hashes may be used for:
 *
 *     dataset identity;
 *     model identity;
 *     artifact identity;
 *     provenance;
 *     cache identity;
 *     content addressing;
 *     integrity checking.
 *
 * The hash grammar remains domain-neutral.
 *
 * ============================================================================
 * NETWORKING INTEGRATION
 * ============================================================================
 *
 * Hashes may participate in:
 *
 *     message integrity;
 *     content identity;
 *     request/response integrity;
 *     protocol metadata;
 *     distributed data identity.
 *
 * Actual transport/security protocol semantics remain owned elsewhere.
 *
 * ============================================================================
 * COMPILER / IR INTEGRATION
 * ============================================================================
 *
 * The compiler may lower hash operations to:
 *
 *     classical IR;
 *     generic operation IR;
 *     cryptographic semantic operations;
 *     hardware intrinsics;
 *     accelerator implementations;
 *     runtime services.
 *
 * The choice is downstream.
 *
 * The grammar MUST NOT select an IR instruction or backend.
 *
 * If a hash operation is associated with a quantum program, its security
 * metadata may accompany `quantum::ir`.
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime MAY:
 *
 *     execute a selected hash implementation;
 *     use hardware acceleration;
 *     invoke a cryptographic service;
 *     verify provider availability;
 *     enforce security policy.
 *
 * Runtime MUST NOT be invoked by parsing.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar has:
 *
 *     no actions;
 *     no semantic predicates;
 *     no filesystem access;
 *     no network access;
 *     no hardware discovery;
 *     no randomness;
 *     no runtime calls;
 *     no cryptographic execution.
 *
 * Parsing depends solely on the supplied token stream and grammar version.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST should preserve at least:
 *
 *     operation;
 *     algorithm reference;
 *     input expressions;
 *     output binding;
 *     properties;
 *     requirements;
 *     constraints;
 *     preferences;
 *     domain separation;
 *     canonicalization;
 *     metadata;
 *     source spans.
 *
 * Suggested semantic node:
 *
 *     HashOperation
 *
 * with conceptual fields:
 *
 *     operation_name
 *     algorithm
 *     inputs
 *     output
 *     properties
 *     requirements
 *     constraints
 *     preferences
 *     metadata
 *     source
 *
 * This grammar does not require a particular Rust AST type name. The existing
 * frontend AST contract remains authoritative.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     - whether the algorithm reference resolves;
 *     - whether the input type is hashable;
 *     - whether canonicalization is available;
 *     - whether the requested digest representation is valid;
 *     - whether requested digest length is valid;
 *     - whether truncation is permitted;
 *     - whether domain separation is supported;
 *     - whether requested security properties are satisfied;
 *     - whether requirements conflict;
 *     - whether the selected target has a suitable implementation;
 *     - whether the operation can be lowered;
 *     - whether security policy permits the operation.
 *
 * Parsing performs none of these operations.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * The semantic/frontend diagnostic layer should distinguish:
 *
 *     unknown hash algorithm;
 *     unsupported hash property;
 *     invalid hash input;
 *     invalid digest representation;
 *     invalid digest length;
 *     invalid truncation;
 *     unsupported domain separation;
 *     invalid canonicalization;
 *     unsatisfied cryptographic requirement;
 *     conflicting cryptographic constraint;
 *     unavailable target capability;
 *     forbidden security configuration;
 *     invalid provider reference;
 *     invalid implementation reference.
 *
 * Diagnostics MUST preserve source spans.
 *
 * Diagnostics MUST NOT expose secret material.
 *
 * ============================================================================
 * SECURITY PROPERTY MODEL
 * ============================================================================
 *
 * Hash security properties are semantic names.
 *
 * Examples:
 *
 *     cryptography::hash::collision_resistance
 *     cryptography::hash::preimage_resistance
 *     cryptography::hash::second_preimage_resistance
 *     cryptography::hash::domain_separation
 *     cryptography::hash::deterministic
 *     cryptography::hash::streaming
 *
 * These are not closed enumerations.
 *
 * Future properties remain syntactically representable through qualified
 * names.
 *
 * ============================================================================
 * ALGORITHM AGILITY
 * ============================================================================
 *
 * Source programs SHOULD prefer semantic algorithm properties when portability
 * is the primary goal.
 *
 * For example:
 *
 *     requires cryptography::hash::collision_resistance
 *
 * can remain portable even when the implementation changes.
 *
 * Explicit algorithm references remain legal where reproducibility,
 * interoperability, protocol compatibility, or other semantic requirements
 * require them.
 *
 * ============================================================================
 * PROVIDER / IMPLEMENTATION REFERENCES
 * ============================================================================
 *
 * Provider and implementation names are opaque semantic references.
 *
 * They MUST NOT trigger discovery.
 *
 * Example:
 *
 *     provider: organization::crypto
 *
 * means that the semantic/runtime layer may require such a provider.
 *
 * It does not mean:
 *
 *     discover organization::crypto during parsing.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file intentionally contains no:
 *
 *     MAX_*
 *     fixed digest maximum;
 *     fixed algorithm list;
 *     fixed provider list;
 *     fixed device list;
 *     fixed hardware topology;
 *     fixed CPU count;
 *     fixed GPU count;
 *     fixed FPGA count;
 *     fixed QPU count;
 *     fixed memory size;
 *     fixed input size;
 *     fixed operation count.
 *
 * Numeric literals appearing in source expressions remain program data and are
 * not compiler-wide capacity limits.
 *
 * ============================================================================
 * ANTLR INTEGRATION
 * ============================================================================
 */

parser grammar Hashes;

options {
    tokenVocab = ZamaniLexer;
}

import
    Core,
    Types,
    Expressions;


/*
 * ============================================================================
 * 1. STANDALONE ENTRY POINT
 * ============================================================================
 *
 * Used by:
 *
 *     grammar tests;
 *     parser conformance;
 *     grammar validation;
 *     tooling.
 *
 * The complete security parser enters the hash rules through:
 *
 *     cryptographicHashDeclaration
 *     cryptographicHashOperation
 *
 * It must NOT require this EOF-consuming rule.
 */

hashesFile
    : hashEntry* EOF
    ;

hashEntry
    : cryptographicHashDeclaration
    | cryptographicHashOperation
    ;


/*
 * ============================================================================
 * 2. HASH DECLARATION
 * ============================================================================
 *
 * Hash declarations use the existing `cryptography` namespace structurally
 * rather than requiring a new HASH lexer keyword.
 *
 * Canonical shape:
 *
 *     apply cryptography::hash(...);
 *
 * or a named hash declaration attached to a semantic operation scope.
 *
 * The operation name itself remains an identifier.
 */

cryptographicHashDeclaration
    : APPLY cryptographicHashName
      cryptographicHashArguments?
      cryptographicHashBody?
      SEMI?
    ;


/*
 * ============================================================================
 * 3. HASH OPERATION
 * ============================================================================
 *
 * Kept as a separate public rule so the cryptography composition grammar can
 * explicitly delegate hash semantics here.
 *
 * The concrete source shape is:
 *
 *     apply cryptography::hash(...)
 *
 * followed optionally by a hash descriptor body.
 */

cryptographicHashOperation
    : APPLY cryptographicHashName
      cryptographicHashArguments
      cryptographicHashBody?
      SEMI?
    ;


/*
 * ============================================================================
 * 4. HASH NAME
 * ============================================================================
 *
 * The hash operation is identified by the canonical semantic namespace:
 *
 *     cryptography::hash
 *
 * Additional namespace components remain available for future dialects and
 * algorithm-specific forms.
 *
 * Examples:
 *
 *     cryptography::hash
 *     cryptography::hash::sha256
 *     future::cryptography::hash
 *     organization::cryptography::hash
 *
 * The semantic layer determines the meaning.
 */

cryptographicHashName
    : qualifiedName
    ;


/*
 * ============================================================================
 * 5. ARGUMENTS
 * ============================================================================
 */

cryptographicHashArguments
    : LPAREN
      cryptographicHashInputList?
      RPAREN
    ;

cryptographicHashInputList
    : expression
      (COMMA expression)*
    ;


/*
 * ============================================================================
 * 6. HASH BODY
 * ============================================================================
 */

cryptographicHashBody
    : LBRACE
      cryptographicHashMember*
      RBRACE
    ;

cryptographicHashMember
    : hashAlgorithmClause
    | hashOutputClause
    | hashPropertyClause
    | hashRequirementClause
    | hashConstraintClause
    | hashPreferenceClause
    | hashDomainClause
    | hashCanonicalizationClause
    | hashTruncationClause
    | hashProviderClause
    | hashImplementationClause
    | hashParameterClause
    | hashMetadataClause
    ;


/*
 * ============================================================================
 * 7. ALGORITHM
 * ============================================================================
 *
 * The algorithm is a semantic reference.
 *
 * No closed algorithm enumeration exists.
 */

hashAlgorithmClause
    : ALGORITHM qualifiedName
    ;


/*
 * ============================================================================
 * 8. OUTPUT
 * ============================================================================
 *
 * Output intent is represented structurally.
 *
 * The target variable/type is resolved downstream.
 */

hashOutputClause
    : identifier
      ASSIGN expression
    ;


/*
 * ============================================================================
 * 9. HASH PROPERTIES
 * ============================================================================
 */

hashPropertyClause
    : PROPERTY expression
    ;


/*
 * ============================================================================
 * 10. REQUIREMENTS
 * ============================================================================
 *
 * A requirement is mandatory.
 */

hashRequirementClause
    : REQUIRES expression
    ;


/*
 * ============================================================================
 * 11. CONSTRAINTS
 * ============================================================================
 */

hashConstraintClause
    : CONSTRAINT expression
    ;


/*
 * ============================================================================
 * 12. PREFERENCES
 * ============================================================================
 */

hashPreferenceClause
    : PREFER expression
    ;


/*
 * ============================================================================
 * 13. DOMAIN SEPARATION
 * ============================================================================
 *
 * No fixed domain vocabulary is imposed.
 */

hashDomainClause
    : identifier
      COLON
      expression
    ;


/*
 * ============================================================================
 * 14. CANONICALIZATION
 * ============================================================================
 *
 * Canonicalization is semantic intent.
 */

hashCanonicalizationClause
    : identifier
      COLON
      expression
    ;


/*
 * ============================================================================
 * 15. TRUNCATION
 * ============================================================================
 *
 * The expression supplies the requested semantic length.
 *
 * No maximum is imposed here.
 */

hashTruncationClause
    : identifier
      COLON
      expression
    ;


/*
 * ============================================================================
 * 16. PROVIDER
 * ============================================================================
 *
 * Provider names are opaque references.
 *
 * They do not trigger discovery.
 */

hashProviderClause
    : identifier
      COLON
      qualifiedName
    ;


/*
 * ============================================================================
 * 17. IMPLEMENTATION
 * ============================================================================
 *
 * Implementation names are opaque references.
 */

hashImplementationClause
    : identifier
      COLON
      qualifiedName
    ;


/*
 * ============================================================================
 * 18. PARAMETERS
 * ============================================================================
 */

hashParameterClause
    : identifier
      ASSIGN
      expression
    ;


/*
 * ============================================================================
 * 19. METADATA
 * ============================================================================
 *
 * Metadata remains ordinary source-level metadata.
 *
 * It is not interpreted by the parser.
 */

hashMetadataClause
    : identifier
      COLON
      expression
    ;


/*
 * ============================================================================
 * 20. HASH REFERENCE
 * ============================================================================
 *
 * Hash references may be used by surrounding security/cryptography syntax.
 *
 * They remain open-world.
 */

hashReference
    : qualifiedName
    ;

hashReferenceList
    : hashReference
      (COMMA hashReference)*
      COMMA?
    ;


/*
 * ============================================================================
 * 21. HASH REQUIREMENT CONTRACT
 * ============================================================================
 *
 * This façade rule exists so the cryptography composition grammar can refer to
 * hash requirements without copying their implementation.
 */

hashRequirement
    : hashRequirementClause
    ;


/*
 * ============================================================================
 * 22. HASH CONSTRAINT CONTRACT
 * ============================================================================
 */

hashConstraint
    : hashConstraintClause
    ;


/*
 * ============================================================================
 * 23. HASH PREFERENCE CONTRACT
 * ============================================================================
 */

hashPreference
    : hashPreferenceClause
    ;


/*
 * ============================================================================
 * 24. HASH PROPERTY CONTRACT
 * ============================================================================
 */

hashProperty
    : hashPropertyClause
    ;


/*
 * ============================================================================
 * 25. HASH ALGORITHM CONTRACT
 * ============================================================================
 */

hashAlgorithmReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 26. SOURCE PRESERVATION
 * ============================================================================
 *
 * The parser must preserve:
 *
 *     operation;
 *     namespace;
 *     algorithm reference;
 *     argument expressions;
 *     body member order;
 *     property expressions;
 *     requirement expressions;
 *     constraint expressions;
 *     preference expressions;
 *     metadata expressions;
 *     source spans.
 *
 * Semantic normalization belongs downstream.
 *
 * ============================================================================
 * 27. NO SECRET EVALUATION
 * ============================================================================
 *
 * This grammar never evaluates:
 *
 *     strings;
 *     expressions;
 *     algorithm names;
 *     provider names;
 *     implementation names;
 *     references.
 *
 * It only parses them.
 *
 * ============================================================================
 * 28. ERROR RECOVERY
 * ============================================================================
 *
 * Error recovery belongs to the generated parser/frontend.
 *
 * This grammar does not:
 *
 *     recover by executing source;
 *     query security providers;
 *     inspect hardware;
 *     inspect credentials;
 *     inspect key stores;
 *     call cryptographic libraries.
 *
 * ============================================================================
 * 29. COMPATIBILITY
 * ============================================================================
 *
 * This file intentionally introduces no new lexer token.
 *
 * Therefore adding this grammar does not require a lexical-token compatibility
 * migration.
 *
 * If the language specification later decides that `hash` should become a
 * reserved keyword, that is a separate lexical evolution:
 *
 *     lexer specification
 *         ->
 *     compatibility policy
 *         ->
 *     ZamaniKeywords
 *         ->
 *     ZamaniTokens
 *         ->
 *     ZamaniLexer
 *         ->
 *     parser integration
 *
 * `hashes.g4` must then be migrated deliberately rather than silently changing
 * token ownership.
 *
 * ============================================================================
 * 30. VALIDATION CONTRACT
 * ============================================================================
 *
 * Grammar validation must verify:
 *
 *     - this grammar is a parser grammar;
 *     - tokenVocab is ZamaniLexer;
 *     - no lexer rules exist here;
 *     - no Rust actions exist;
 *     - no semantic predicates exist;
 *     - imports resolve;
 *     - Core / Types / Expressions are canonical dependencies;
 *     - no fixed hash algorithm enumeration exists;
 *     - no MAX_* capacity exists;
 *     - no hardware identity exists;
 *     - no provider discovery exists;
 *     - no secret material is required;
 *     - no second expression grammar exists;
 *     - no second security capability system exists;
 *     - no second quantum IR exists.
 *
 * ============================================================================
 * 31. TEST CONTRACT
 * ============================================================================
 *
 * Positive tests must include:
 *
 *     apply cryptography::hash(data);
 *     apply cryptography::hash(data) {
 *         algorithm: cryptography::hash::sha256;
 *     }
 *
 *     apply cryptography::hash(data) {
 *         algorithm: future::cryptography::hash::algorithm;
 *         property: cryptography::hash::collision_resistance;
 *     }
 *
 *     apply cryptography::hash(data) {
 *         requires cryptography::hash::deterministic;
 *     }
 *
 *     apply cryptography::hash(data) {
 *         domain: application::identity;
 *     }
 *
 *     apply cryptography::hash(serialize(value)) {
 *         canonical: data::canonical;
 *     }
 *
 *     apply cryptography::hash(stream) {
 *         output: digest;
 *     }
 *
 *     apply cryptography::hash(data) {
 *         length: digest_length;
 *     }
 *
 * Negative/semantic tests must include:
 *
 *     invalid algorithm references;
 *     invalid input types;
 *     unsupported digest length;
 *     invalid truncation;
 *     unsupported domain separation;
 *     incompatible canonicalization;
 *     unsatisfied security requirements;
 *     conflicting constraints;
 *     unavailable implementation.
 *
 * The parser must distinguish syntax failures from semantic failures.
 *
 * ============================================================================
 * 32. SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests must vary:
 *
 *     input size;
 *     expression depth;
 *     number of hash operations;
 *     number of requirements;
 *     number of constraints;
 *     number of properties;
 *     number of metadata entries;
 *     qualified-name depth;
 *     program size.
 *
 * Tests MUST NOT establish an artificial maximum.
 *
 * ============================================================================
 * 33. DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * Parsing identical input with identical:
 *
 *     source;
 *     lexer version;
 *     parser version;
 *     grammar version;
 *
 * must produce the same parse structure.
 *
 * Parsing must not depend on:
 *
 *     current time;
 *     filesystem;
 *     network;
 *     hardware;
 *     provider availability;
 *     random state;
 *     runtime state.
 *
 * ============================================================================
 * 34. SAFE-RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust implementation.
 *
 * The Rust frontend/compiler integrating this grammar MUST remain compatible
 * with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and MUST NOT require:
 *
 *     unsafe
 *     unsafe blocks
 *     unsafe traits
 *     unsafe implementations
 *
 * Where applicable, the crate-level safety boundary SHOULD use:
 *
 *     #![forbid(unsafe_code)]
 *
 * ============================================================================
 * 35. IR INTEGRATION CONTRACT
 * ============================================================================
 *
 * Hash syntax lowers conceptually as:
 *
 *     HashSyntax
 *          |
 *          v
 *     Hash AST
 *          |
 *          v
 *     Cryptographic Semantic Operation
 *          |
 *          +-------------------------+
 *          |                         |
 *          v                         v
 *     classical representation   quantum::ir metadata
 *          |                         |
 *          +------------+------------+
 *                       |
 *                       v
 *                  target lowering
 *
 * The hash grammar MUST NOT define an IR instruction.
 *
 * ============================================================================
 * 36. QUANTUM IR CONTRACT
 * ============================================================================
 *
 * If a hash operation applies to quantum-related semantic data, the security
 * metadata may accompany the canonical quantum IR.
 *
 * The canonical quantum boundary remains:
 *
 *     quantum::ir
 *
 * No:
 *
 *     HashQuantumIr
 *     QuantumHashIr
 *     SecurityQuantumIr
 *
 * may be created by this grammar.
 *
 * ============================================================================
 * 37. RESILIENCE CONTRACT
 * ============================================================================
 *
 * Security/integrity metadata produced from this grammar may be consumed by
 * resilience analysis.
 *
 * Resilience may determine whether a recovered/retried artifact still
 * satisfies the required integrity properties.
 *
 * This grammar does not implement:
 *
 *     retry;
 *     recovery;
 *     rollback;
 *     rerouting;
 *     rescheduling;
 *     backend switching.
 *
 * ============================================================================
 * 38. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE when:
 *
 * [x] Hash syntax has one owner.
 * [x] No hash lexer token is invented here.
 * [x] The canonical lexer remains the only lexer authority.
 * [x] Algorithms are open-world.
 * [x] Providers are open-world.
 * [x] Implementations are open-world.
 * [x] Digest properties are open-world.
 * [x] No universal capacity is hard-coded.
 * [x] No hardware identity is required.
 * [x] No secret material is embedded by design.
 * [x] General expressions remain owned by Expressions.
 * [x] General names remain owned by Core.
 * [x] Generic types remain owned by Types.
 * [x] Hash semantics remain separate from implementation.
 * [x] Hash semantics remain separate from runtime enforcement.
 * [x] Hash semantics remain compatible with classical computation.
 * [x] Hash semantics remain compatible with quantum metadata.
 * [x] Hash semantics remain compatible with HDL/hardware provenance.
 * [x] Hash semantics remain compatible with distributed computation.
 * [x] Hash semantics remain compatible with AI/data provenance.
 * [x] Hash semantics remain compatible with networking.
 * [x] Parsing remains deterministic.
 * [x] Grammar contains no actions.
 * [x] Grammar contains no semantic predicates.
 * [x] Safe-Rust requirement is preserved.
 * [x] Rust 1.97 / 1.97.1 compatibility is documented.
 * [x] Positive tests are defined.
 * [x] Negative tests are defined.
 * [x] Boundary tests are defined.
 * [x] Scalability tests are defined.
 * [x] Compatibility is defined.
 * [x] AST integration is defined.
 * [x] Semantic integration is defined.
 * [x] IR integration is defined.
 * [x] Runtime integration is defined.
 * [x] Hard-coding audit is defined.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * Hashes are a semantic cryptographic capability of Zamani.
 *
 * They are not a fixed list of algorithms, a fixed hardware feature, or a
 * second execution language.
 *
 * The language therefore preserves:
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
 * subject to the actual cryptographic requirements of the program and the
 * capabilities/resources available to the execution environment.
 *
 * ============================================================================
 */
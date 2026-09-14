/**
 * Zamani — Universal Serialization Grammar
 *
 * Path:
 *     grammar/data/serialization.g4
 *
 * Grammar:
 *     serialization
 *
 * PURPOSE
 * -------
 * Defines the language-level syntax for expressing serialization and
 * deserialization intent.
 *
 * The grammar describes WHAT representation/conversion is required.
 * It does not prescribe HOW a particular runtime, library, processor,
 * accelerator, storage engine, network, or device implements it.
 *
 *
 * OWNS
 * ----
 * - serialization expressions
 * - deserialization expressions
 * - serialization statements
 * - deserialization statements
 * - representation/format references
 * - schema/type references
 * - serialization options
 * - compatibility/version intent
 * - canonicalization intent
 * - framing intent
 * - compression intent
 * - integrity intent
 * - extensible serialization policies
 * - source-level serialization contracts
 *
 *
 * DOES NOT OWN
 * -------------
 * - general types
 * - expressions
 * - records
 * - schemas
 * - collections
 * - streams
 * - transformations
 * - networking
 * - storage
 * - filesystem access
 * - database engines
 * - compression algorithms
 * - cryptographic implementations
 * - physical memory layout
 * - hardware
 * - device identifiers
 * - quantum IR
 * - quantum-state implementation
 * - runtime allocation
 * - scheduling
 * - deployment
 *
 *
 * CRITICAL ARCHITECTURAL RULE
 * ----------------------------
 * A serialization format is a logical representation contract.
 *
 * It is NOT a hardware target.
 *
 * Therefore this grammar MUST NOT require:
 *
 *     JSON
 *     XML
 *     CBOR
 *     Protobuf
 *     MessagePack
 *
 * or any other finite provider list to be exhaustive.
 *
 * Standard formats may be registered by the standard library, dialect
 * registry, interoperability layer, or runtime.
 *
 * User-defined and future formats remain possible through qualified names.
 *
 *
 * SCALABILITY
 * -----------
 * This grammar imposes no fixed maximum on:
 *
 * - serialized values
 * - fields
 * - records
 * - collections
 * - stream elements
 * - schema members
 * - metadata entries
 * - options
 * - nested values
 * - serialization operations
 * - formats
 * - versions
 * - compatibility rules
 *
 * Any physical limit belongs to the resource/runtime/backend layer.
 *
 *
 * POCO-REAF
 * ---------
 * A program can state:
 *
 *     serialize value using format.some_representation
 *
 * without stating:
 *
 *     use library X
 *     use device Y
 *     allocate N bytes
 *     use machine Z
 *     use network node N
 *
 * The compiler/runtime resolves the implementation available on the
 * execution target.
 *
 *
 * RUST
 * ----
 * This file contains no embedded Rust actions.
 *
 * Generated Rust parser code must integrate with Rust 1.97 / 1.97.1.
 * Zamani-owned Rust integration must remain safe Rust.
 *
 *
 * PARSER ARCHITECTURE
 * -------------------
 * This is intentionally a parser grammar.
 *
 * The shared Zamani lexer owns tokens.
 * This grammar MUST NOT define a second lexer.
 */

parser grammar serialization;

options {
    tokenVocab = Zamani;
}


/* ==========================================================================
 * PUBLIC ENTRY POINT
 * ========================================================================== */

/**
 * Standalone serialization grammar entry point.
 *
 * The root Zamani grammar must NOT use this as its whole-program entry
 * point. It exists for grammar-specific testing and tooling.
 */
serializationUnit
    : serializationConstruct*
      EOF
    ;


/* ==========================================================================
 * PUBLIC SERIALIZATION CONSTRUCTS
 * ========================================================================== */

/**
 * Integration facade consumed by data/data.g4.
 *
 * data.g4 should delegate serialization syntax to this rule rather than
 * redefining serialization alternatives.
 */
serializationConstruct
    : serializationStatement
    | deserializationStatement
    | serializationExpression
    | deserializationExpression
    | serializationContractDeclaration
    ;


/* ==========================================================================
 * SERIALIZATION STATEMENTS
 * ========================================================================== */

/**
 * Legacy-compatible form:
 *
 *     serialize expression to format;
 *
 * Extended production form:
 *
 *     serialize expression using format ...;
 *
 * Both forms are retained during migration.
 */
serializationStatement
    : 'serialize'
      expression
      serializationDestinationClause?
      serializationFormatClause
      serializationOptionClause*
      ';'
    ;


/**
 * Explicit destination support.
 *
 * Destination semantics remain logical.
 *
 * Physical file/network/storage handling belongs elsewhere.
 */
serializationDestinationClause
    : 'to'
      serializationDestination
    ;

serializationDestination
    : expression
    ;


/**
 * The representation itself is intentionally extensible.
 */
serializationFormatClause
    : 'to'
      serializationFormatReference
    | 'using'
      serializationFormatReference
    ;

serializationFormatReference
    : qualifiedName
    ;


/* ==========================================================================
 * DESERIALIZATION STATEMENTS
 * ========================================================================== */

deserializationStatement
    : 'deserialize'
      expression
      deserializationSourceClause?
      serializationFormatClause
      serializationOptionClause*
      deserializationTargetClause?
      ';'
    ;

deserializationSourceClause
    : 'from'
      expression
    ;

deserializationTargetClause
    : 'as'
      typeExpr
    ;


/* ==========================================================================
 * SERIALIZATION EXPRESSIONS
 * ========================================================================== */

/**
 * Expression form permits serialization to participate in:
 *
 *     pipelines
 *     transformations
 *     function arguments
 *     assignments
 *     data movement
 *     interoperability
 *
 * without requiring a statement.
 */
serializationExpression
    : 'serialize'
      '('
      expression
      serializationExpressionFormatClause?
      serializationExpressionOptionList?
      ')'
    ;

serializationExpressionFormatClause
    : 'to'
      serializationFormatReference
    | 'using'
      serializationFormatReference
    ;


/* ==========================================================================
 * DESERIALIZATION EXPRESSIONS
 * ========================================================================== */

deserializationExpression
    : 'deserialize'
      '('
      expression
      serializationExpressionFormatClause?
      serializationExpressionTargetClause?
      serializationExpressionOptionList?
      ')'
    ;

serializationExpressionTargetClause
    : 'as'
      typeExpr
    ;


/* ==========================================================================
 * OPTION LISTS
 * ========================================================================== */

serializationOptionClause
    : serializationOption
    ;

serializationExpressionOptionList
    : '['
      serializationOption
      (',' serializationOption)*
      ']'
    ;

serializationOption
    : serializationSchemaOption
    | serializationVersionOption
    | serializationCompatibilityOption
    | serializationCanonicalOption
    | serializationFramingOption
    | serializationCompressionOption
    | serializationIntegrityOption
    | serializationMetadataOption
    | serializationPropertyOption
    | serializationPolicyOption
    | serializationExtensionOption
    ;


/* ==========================================================================
 * SCHEMA / TYPE OPTIONS
 * ========================================================================== */

/**
 * A serialization schema is a semantic contract, not a physical database
 * schema.
 */
serializationSchemaOption
    : 'schema'
      qualifiedName
    ;


/**
 * The schema may be inferred from the value/type when omitted.
 */
serializationTypeOption
    : 'type'
      typeExpr
    ;


/* ==========================================================================
 * VERSIONING
 * ========================================================================== */

/**
 * Serialization format version is distinct from:
 *
 * - language version
 * - AST schema version
 * - IR version
 * - backend version
 * - protocol version
 */
serializationVersionOption
    : 'version'
      expression
    ;

serializationSchemaVersionOption
    : 'schema_version'
      expression
    ;


/* ==========================================================================
 * COMPATIBILITY
 * ========================================================================== */

serializationCompatibilityOption
    : 'compatibility'
      qualifiedName
      serializationNamedArguments?
    ;

serializationCompatibilityRule
    : 'compatible'
      qualifiedName
      serializationNamedArguments?
    ;


/* ==========================================================================
 * CANONICALIZATION
 * ========================================================================== */

serializationCanonicalOption
    : 'canonical'
    | 'canonical'
      '='
      expression
    ;


/**
 * Canonicalization may be an implementation-specific policy.
 *
 * The grammar only records intent.
 */
serializationCanonicalPolicy
    : 'canonicalization'
      qualifiedName
      serializationNamedArguments?
    ;


/* ==========================================================================
 * FRAMING
 * ========================================================================== */

serializationFramingOption
    : 'framing'
      qualifiedName
      serializationNamedArguments?
    ;


/**
 * Framing describes logical document/message boundaries.
 *
 * It does not define network packet layout.
 */
serializationFrameOption
    : 'frame'
      qualifiedName
      serializationNamedArguments?
    ;


/* ==========================================================================
 * COMPRESSION
 * ========================================================================== */

/**
 * Compression algorithm names are qualified names rather than hard-coded
 * grammar keywords.
 *
 * Examples may be:
 *
 *     compression.zstd
 *     compression.gzip
 *     compression.custom
 *
 * but the grammar does not privilege any particular algorithm.
 */
serializationCompressionOption
    : 'compression'
      qualifiedName
      serializationNamedArguments?
    ;

serializationCompressionLevelOption
    : 'compression_level'
      expression
    ;


/* ==========================================================================
 * INTEGRITY
 * ========================================================================== */

/**
 * Integrity is intentionally separate from encryption/security.
 *
 * Cryptographic policy implementation belongs to security/interoperability.
 */
serializationIntegrityOption
    : 'integrity'
      qualifiedName
      serializationNamedArguments?
    ;


/* ==========================================================================
 * METADATA
 * ========================================================================== */

serializationMetadataOption
    : 'metadata'
      serializationMetadataValue
    ;

serializationMetadataValue
    : expression
    ;


/* ==========================================================================
 * GENERIC PROPERTIES
 * ========================================================================== */

serializationPropertyOption
    : 'property'
      qualifiedName
      '='
      expression
    ;


/* ==========================================================================
 * EXTENSIBLE POLICIES
 * ========================================================================== */

/**
 * Open policy namespace.
 *
 * This prevents the grammar from becoming obsolete every time a new
 * serialization requirement is introduced.
 */
serializationPolicyOption
    : 'policy'
      qualifiedName
      serializationNamedArguments?
    ;


/**
 * Explicit extension point for dialects.
 */
serializationExtensionOption
    : 'extension'
      qualifiedName
      serializationNamedArguments?
    ;


/* ==========================================================================
 * NAMED ARGUMENTS
 * ========================================================================== */

serializationNamedArguments
    : '('
      serializationNamedArgument
      (',' serializationNamedArgument)*
      ')'
    ;

serializationNamedArgument
    : IDENTIFIER
      '='
      expression
    ;


/* ==========================================================================
 * SERIALIZATION CONTRACT DECLARATIONS
 * ========================================================================== */

/**
 * A contract defines reusable serialization intent.
 *
 * Example conceptually:
 *
 *     serialization contract telemetry
 *     {
 *         format telemetry.binary;
 *         schema telemetry.Event;
 *         version 1;
 *         compatibility backward;
 *     }
 *
 * The grammar intentionally does not dictate a particular wire format.
 */
serializationContractDeclaration
    : visibilityModifier?
      'serialization'
      'contract'
      qualifiedName
      serializationContractTypeParameters?
      '{'
      serializationContractMember*
      '}'
    ;

serializationContractTypeParameters
    : '<'
      serializationTypeParameter
      (',' serializationTypeParameter)*
      '>'
    ;

serializationTypeParameter
    : IDENTIFIER
      (':' typeExpr)?
    ;


/* ==========================================================================
 * CONTRACT MEMBERS
 * ========================================================================== */

serializationContractMember
    : serializationContractFormat
    | serializationContractSchema
    | serializationContractVersion
    | serializationContractCompatibility
    | serializationContractCanonicalization
    | serializationContractFraming
    | serializationContractCompression
    | serializationContractIntegrity
    | serializationContractProperty
    | serializationContractPolicy
    | annotation
    ;


/* ==========================================================================
 * CONTRACT FORMAT
 * ========================================================================== */

serializationContractFormat
    : 'format'
      serializationFormatReference
      ';'
    ;


/* ==========================================================================
 * CONTRACT SCHEMA
 * ========================================================================== */

serializationContractSchema
    : 'schema'
      qualifiedName
      ';'
    ;


/* ==========================================================================
 * CONTRACT VERSION
 * ========================================================================== */

serializationContractVersion
    : 'version'
      expression
      ';'
    ;


/* ==========================================================================
 * CONTRACT COMPATIBILITY
 * ========================================================================== */

serializationContractCompatibility
    : 'compatibility'
      qualifiedName
      serializationNamedArguments?
      ';'
    ;


/* ==========================================================================
 * CONTRACT CANONICALIZATION
 * ========================================================================== */

serializationContractCanonicalization
    : 'canonical'
      serializationContractPolicyArguments?
      ';'
    ;

serializationContractPolicyArguments
    : '('
      serializationArgumentList?
      ')'
    ;


/* ==========================================================================
 * CONTRACT FRAMING
 * ========================================================================== */

serializationContractFraming
    : 'framing'
      qualifiedName
      serializationNamedArguments?
      ';'
    ;


/* ==========================================================================
 * CONTRACT COMPRESSION
 * ========================================================================== */

serializationContractCompression
    : 'compression'
      qualifiedName
      serializationNamedArguments?
      ';'
    ;


/* ==========================================================================
 * CONTRACT INTEGRITY
 * ========================================================================== */

serializationContractIntegrity
    : 'integrity'
      qualifiedName
      serializationNamedArguments?
      ';'
    ;


/* ==========================================================================
 * CONTRACT PROPERTIES
 * ========================================================================== */

serializationContractProperty
    : 'property'
      qualifiedName
      '='
      expression
      ';'
    ;


/* ==========================================================================
 * CONTRACT POLICIES
 * ========================================================================== */

serializationContractPolicy
    : 'policy'
      qualifiedName
      serializationNamedArguments?
      ';'
    ;


/* ==========================================================================
 * GENERIC ARGUMENT LIST
 * ========================================================================== */

serializationArgumentList
    : expression
      (',' expression)*
    ;


/* ==========================================================================
 * SCHEMA EVOLUTION
 * ========================================================================== */

/**
 * Schema evolution belongs semantically to schema compatibility, but the
 * serialization contract may carry the policy used during encoding/decoding.
 */
serializationEvolutionOption
    : 'evolution'
      qualifiedName
      serializationNamedArguments?
    ;


/* ==========================================================================
 * NULL / OPTIONAL / DEFAULT REPRESENTATION
 * ========================================================================== */

serializationNullPolicyOption
    : 'null_policy'
      qualifiedName
      serializationNamedArguments?
    ;

serializationDefaultPolicyOption
    : 'default_policy'
      qualifiedName
      serializationNamedArguments?
    ;


/* ==========================================================================
 * FIELD REPRESENTATION
 * ========================================================================== */

/**
 * Field representation is deliberately symbolic.
 *
 * It must not require a particular byte width unless that width is actually
 * part of the language-level serialization contract.
 */
serializationFieldPolicyOption
    : 'field_policy'
      qualifiedName
      serializationNamedArguments?
    ;


/* ==========================================================================
 * ENCODING POLICY
 * ========================================================================== */

/**
 * Character/text encoding is a representation concern.
 *
 * It remains extensible rather than hard-coded to UTF-8/UTF-16/etc.
 */
serializationEncodingOption
    : 'encoding'
      qualifiedName
      serializationNamedArguments?
    ;


/* ==========================================================================
 * BYTE ORDER
 * ========================================================================== */

/**
 * Byte ordering is only meaningful for representations that expose such a
 * property. The semantic layer determines whether the selected format allows
 * it.
 */
serializationByteOrderOption
    : 'byte_order'
      qualifiedName
      serializationNamedArguments?
    ;


/* ==========================================================================
 * DETERMINISM
 * ========================================================================== */

serializationDeterminismOption
    : 'deterministic'
    | 'deterministic'
      '='
      expression
    ;


/* ==========================================================================
 * LOSS / FIDELITY POLICY
 * ========================================================================== */

/**
 * Particularly important for scientific, numerical, AI, quantum, and
 * hardware-adjacent data.
 *
 * The grammar records intent.
 *
 * It does NOT claim that arbitrary quantum states, hardware states, or
 * runtime state can necessarily be reconstructed.
 */
serializationFidelityOption
    : 'fidelity'
      qualifiedName
      serializationNamedArguments?
    ;


/* ==========================================================================
 * QUANTUM-SAFE SEMANTIC BOUNDARY
 * ========================================================================== */

/**
 * Quantum data may be serialized when a backend/domain defines a valid
 * representation.
 *
 * This grammar does NOT define a quantum-state format.
 *
 * It MUST NOT imply that an arbitrary live physical quantum state can be
 * extracted and serialized.
 *
 * Actual semantics are determined by the quantum subsystem, canonical
 * quantum::ir, hardware capabilities, and runtime contract.
 */
serializationQuantumPolicy
    : 'quantum'
      qualifiedName
      serializationNamedArguments?
    ;


/* ==========================================================================
 * HARDWARE / EXECUTION INDEPENDENCE
 * ========================================================================== */

/**
 * These are intentionally not hardware declarations.
 *
 * If serialization requires a capability, it should be expressed through
 * the universal resource/capability system rather than here.
 *
 * Therefore this grammar does NOT define:
 *
 *     device
 *     gpu
 *     qpu
 *     fpga
 *     cpu
 *     memory_size
 *     address
 *     node_count
 *     bandwidth
 *
 * Those belong to their respective grammar domains.
 */


/* ==========================================================================
 * LEGACY FORMAT COMPATIBILITY
 * ========================================================================== */

/**
 * The existing Zamani grammar documents JSON, XML, MessagePack, Protobuf,
 * and CBOR as serialization formats.
 *
 * They remain valid through qualified-name interpretation during migration:
 *
 *     json
 *     xml
 *     messagepack
 *     protobuf
 *     cbor
 *
 * The semantic registry, not this grammar, determines whether a format is
 * available.
 *
 * These rules are compatibility aliases only.
 */
legacySerializationFormat
    : 'json'
    | 'xml'
    | 'messagepack'
    | 'protobuf'
    | 'cbor'
    ;


/**
 * Compatibility adapter for existing code that still expects `dataFormat`.
 *
 * This rule MUST be removed only after all legacy consumers have migrated.
 */
dataFormat
    : legacySerializationFormat
    | qualifiedName
    ;


/**
 * Compatibility adapter for the legacy data statement.
 *
 * Existing syntax:
 *
 *     serialize expression to json;
 *     deserialize expression from json;
 *
 * remains parseable while the modular grammar becomes authoritative.
 */
legacyDataSerializationStatement
    : 'serialize'
      expression
      'to'
      dataFormat
      ';'
    | 'deserialize'
      expression
      'from'
      dataFormat
      ';'
    ;


/* ==========================================================================
 * SEMANTIC VALIDATION BOUNDARY
 * ==========================================================================
 *
 * The following are intentionally semantic checks, NOT grammar checks:
 *
 * 1. Is the selected format registered?
 * 2. Does the format support the value's type?
 * 3. Does the selected schema match the value?
 * 4. Is the version compatible?
 * 5. Is the requested compatibility policy satisfiable?
 * 6. Is canonicalization supported?
 * 7. Is compression supported?
 * 8. Is integrity supported?
 * 9. Is the selected format lossless?
 * 10. Can the selected representation preserve the required precision?
 * 11. Can a quantum value legally be represented?
 * 12. Can the target runtime decode it?
 * 13. Are required capabilities available?
 * 14. Are resource requirements satisfiable?
 *
 * None of these should be encoded as arbitrary parser limits.
 */


/* ==========================================================================
 * OWNERSHIP BOUNDARIES
 * ==========================================================================
 *
 * `serialization.g4`
 *     owns source syntax for serialization intent.
 *
 * `schemas.g4`
 *     owns schema declarations and schema evolution semantics.
 *
 * `records.g4`
 *     owns record declarations.
 *
 * `collections.g4`
 *     owns collection declarations.
 *
 * `streams.g4`
 *     owns logical stream declarations.
 *
 * `transformations.g4`
 *     owns data transformation/query semantics.
 *
 * `types/*.g4`
 *     owns the type system.
 *
 * `expressions/*.g4`
 *     owns general expressions.
 *
 * `security/*.g4`
 *     owns security/cryptographic policy.
 *
 * `networking/*.g4`
 *     owns transport/network protocol semantics.
 *
 * `resources/*.g4`
 *     owns resource/capability requirements.
 *
 * `hardware/*.g4`
 *     owns hardware descriptions.
 *
 * `execution/*.g4`
 *     owns runtime execution/deployment.
 *
 * `quantum/*.g4`
 *     owns quantum source syntax.
 *
 * `quantum::ir`
 *     remains the canonical quantum semantic boundary.
 *
 * This grammar MUST NOT import quantum::ir or define a second quantum IR.
 */


/* ==========================================================================
 * AST CONTRACT
 * ==========================================================================
 *
 * The parser layer should produce source-level serialization AST nodes
 * containing, at minimum:
 *
 *     operation
 *     source expression
 *     destination/source expression
 *     format reference
 *     optional target type
 *     schema reference
 *     version expression
 *     compatibility policy
 *     canonicalization policy
 *     framing policy
 *     compression policy
 *     integrity policy
 *     metadata
 *     source span/provenance
 *
 * The grammar must NOT manufacture:
 *
 *     codec instances
 *     byte buffers
 *     runtime handles
 *     device IDs
 *     network connections
 *     storage handles
 *     hardware objects
 *
 * Those belong downstream.
 */


/* ==========================================================================
 * IR CONTRACT
 * ==========================================================================
 *
 * Serialization syntax lowers into a semantic serialization request/plan.
 *
 * The semantic representation should preserve:
 *
 *     source semantics
 *     target type
 *     format identity
 *     schema identity
 *     version
 *     compatibility requirements
 *     representation policies
 *     provenance
 *
 * It should not embed a concrete codec object.
 *
 * Backend lowering may later select:
 *
 *     JSON codec
 *     binary codec
 *     MessagePack codec
 *     Protobuf codec
 *     custom codec
 *     future codec
 *
 * based on capabilities and compilation context.
 */


/* ==========================================================================
 * VERSIONING CONTRACT
 * ==========================================================================
 *
 * Keep distinct:
 *
 *     Zamani language version
 *     grammar version
 *     AST schema version
 *     serialization contract version
 *     serialization format version
 *     schema version
 *     protocol version
 *     backend version
 *
 * They MUST NOT be collapsed into one integer.
 */


/* ==========================================================================
 * DETERMINISM CONTRACT
 * ==========================================================================
 *
 * Parsing must be deterministic.
 *
 * Serialization determinism is a semantic/runtime property.
 *
 * If a program requests deterministic serialization:
 *
 *     deterministic
 *
 * semantic validation must ensure that the selected representation and
 * implementation can satisfy that requirement.
 *
 * The grammar itself must not implement canonical serialization.
 */


/* ==========================================================================
 * SECURITY CONTRACT
 * ==========================================================================
 *
 * Serialization syntax must not silently imply security.
 *
 * For example:
 *
 *     serialize x using format.foo;
 *
 * does not mean:
 *
 *     encrypted
 *     authenticated
 *     private
 *     trusted
 *
 * Security must be explicitly expressed through the security subsystem or
 * an explicitly named serialization integrity/policy contract.
 */


/* ==========================================================================
 * RESOURCE CONTRACT
 * ==========================================================================
 *
 * Serialization can require resources:
 *
 *     memory
 *     compute
 *     bandwidth
 *     storage
 *     accelerator support
 *
 * The grammar must not encode those limits.
 *
 * They are resolved by:
 *
 *     resources
 *     compile
 *     execution
 *     runtime
 *     hardware
 *     distributed
 *
 * subsystems.
 */


/* ==========================================================================
 * POCO-REAF CONTRACT
 * ==========================================================================
 *
 * Source:
 *
 *     serialize value using representation.application.Event;
 *
 * remains semantically portable.
 *
 * The compiler/runtime can map that intent to whatever implementation exists
 * on the target.
 *
 * Therefore:
 *
 *     one source
 *         ->
 *     one semantic serialization contract
 *         ->
 *     many representations/implementations
 *         ->
 *     many machines
 *         ->
 *     many scales
 *         ->
 *     future platforms
 *
 * without rewriting the source merely because the physical machine changes.
 */


/* ==========================================================================
 * NO HARD-CODED SCALABILITY LIMITS
 * ==========================================================================
 *
 * This grammar intentionally has no:
 *
 *     MAX_FIELDS
 *     MAX_BYTES
 *     MAX_RECORDS
 *     MAX_COLLECTIONS
 *     MAX_STREAMS
 *     MAX_SCHEMA_DEPTH
 *     MAX_METADATA
 *     MAX_FORMATS
 *     MAX_VERSIONS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_NETWORK_SIZE
 *
 * Any such limit must be supplied by an external resource/runtime contract.
 */


/* ==========================================================================
 * NO RUNTIME ACTIONS
 * ==========================================================================
 *
 * This file contains no:
 *
 *     @members
 *     @init
 *     @after
 *     embedded Rust
 *     filesystem operations
 *     network operations
 *     codec calls
 *     device discovery
 *     runtime allocation
 *
 * The grammar remains a pure syntax layer.
 */
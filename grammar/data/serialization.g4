/*
 * ============================================================================
 * Zamani Universal Data Serialization Grammar
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/data/serialization.g4
 *
 * STATUS
 * ------
 * CANONICAL SERIALIZATION / DESERIALIZATION LEAF GRAMMAR
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar defines SOURCE-LEVEL SERIALIZATION INTENT.
 *
 * It describes:
 *
 *   - conversion of Zamani values into external representations;
 *   - conversion of external representations into Zamani values;
 *   - reusable serialization contracts;
 *   - representation/format identity;
 *   - schema identity;
 *   - schema/version intent;
 *   - compatibility intent;
 *   - canonicalization intent;
 *   - framing intent;
 *   - compression intent;
 *   - integrity intent;
 *   - encoding intent;
 *   - byte-order intent;
 *   - null/missing-field intent;
 *   - unknown-field intent;
 *   - duplicate-field intent;
 *   - evolution intent;
 *   - metadata intent;
 *   - extensible format properties;
 *   - serialization policies;
 *   - resource-neutral streaming intent;
 *   - validation intent.
 *
 * This grammar defines WHAT is requested.
 *
 * It does NOT define HOW the request is implemented.
 *
 * ============================================================================
 * ARCHITECTURAL OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 *   - serialization syntax;
 *   - deserialization syntax;
 *   - serialization/deserialization expressions;
 *   - serialization/deserialization statement forms;
 *   - serialization contract declarations;
 *   - serialization contract members;
 *   - format references;
 *   - schema references;
 *   - representation options;
 *   - compatibility options;
 *   - evolution options;
 *   - canonicalization options;
 *   - framing options;
 *   - compression options;
 *   - integrity options;
 *   - encoding options;
 *   - byte-order options;
 *   - field/null/default policies;
 *   - metadata;
 *   - extensible serialization properties;
 *   - logical streaming/chunking policy;
 *   - validation intent associated with representation conversion.
 *
 * THIS FILE DOES NOT OWN
 * ----------------------
 *
 *   - general expressions;
 *   - general types;
 *   - records;
 *   - schemas;
 *   - collections;
 *   - streams;
 *   - tensors;
 *   - datasets;
 *   - transformations;
 *   - queries;
 *   - storage;
 *   - filesystems;
 *   - networking;
 *   - databases;
 *   - compression implementations;
 *   - cryptographic implementations;
 *   - encryption;
 *   - memory allocation;
 *   - hardware;
 *   - device selection;
 *   - topology;
 *   - scheduling;
 *   - routing;
 *   - runtime execution;
 *   - backend/provider selection;
 *   - canonical IR definitions.
 *
 * ============================================================================
 * SINGLE-OWNER RULE
 * ============================================================================
 *
 * Serialization and deserialization are one semantic family.
 *
 * Therefore this file is the canonical leaf grammar for BOTH:
 *
 *     serialization
 *     deserialization
 *
 * `grammar/data/deserialization.g4` MUST NOT become a second independent
 * implementation of deserialization syntax.
 *
 * During migration, deserialization.g4 may remain as a compatibility façade,
 * but its rules must delegate to the canonical rules defined here.
 *
 * No third serialization grammar may be introduced.
 *
 * ============================================================================
 * CANONICAL COMPOSITION
 * ============================================================================
 *
 * Canonical hierarchy:
 *
 *     Zamani.g4
 *          |
 *          v
 *     ZamaniParser.g4
 *          |
 *          v
 *        Data
 *          |
 *          v
 *     serialization
 *          |
 *     +----+-------------------+
 *     |                        |
 * serialization             deserialization
 *     |                        |
 *     +-----------+------------+
 *                 |
 *                 v
 *        domain-neutral AST
 *                 |
 *                 v
 *        semantic analysis
 *                 |
 *                 v
 *        canonical data model / IR
 *                 |
 *       +---------+----------+
 *       |         |          |
 *       v         v          v
 *    storage    network    runtime
 *
 * Serialization MUST NOT create another semantic IR.
 *
 * ============================================================================
 * QUANTUM / CLASSICAL / HDL INTEGRATION
 * ============================================================================
 *
 * Serialization is domain-neutral.
 *
 * It may serialize values originating from:
 *
 *   - classical computation;
 *   - quantum computation;
 *   - hybrid computation;
 *   - tensors;
 *   - AI/ML;
 *   - HDL/hardware co-design;
 *   - distributed computation;
 *   - networking;
 *   - future domains.
 *
 * This grammar does not inspect or encode domain implementation details.
 *
 * Quantum values remain governed by the quantum semantic pipeline:
 *
 *     source
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     quantum::ir
 *
 * Serialization is a separate data/interoperability concern.
 *
 * It MUST NOT create:
 *
 *     serialization -> quantum IR
 *
 * or:
 *
 *     serialization -> physical qubit
 *
 * or:
 *
 *     serialization -> device memory
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Serialization syntax MUST remain independent of:
 *
 *   - CPU count;
 *   - core count;
 *   - thread count;
 *   - GPU count;
 *   - FPGA count;
 *   - accelerator count;
 *   - QPU count;
 *   - qubit count;
 *   - node count;
 *   - device count;
 *   - memory capacity;
 *   - storage capacity;
 *   - network capacity;
 *   - register width;
 *   - tensor rank;
 *   - tensor dimensions;
 *   - buffer capacity;
 *   - partition count;
 *   - replica count.
 *
 * The following are NEVER grammar-level limits:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * A numeric value in a program is program semantics.
 *
 * A machine/resource limit is a downstream resource/capability concern.
 *
 * ============================================================================
 * EXTENSIBILITY
 * ============================================================================
 *
 * Formats are OPEN-WORLD identifiers.
 *
 * The grammar MUST NOT enumerate:
 *
 *     json
 *     xml
 *     cbor
 *     protobuf
 *     messagepack
 *
 * as the complete set of formats.
 *
 * Such formats may exist in standard registries or interoperability packages.
 *
 * Future formats can be represented without changing this grammar.
 *
 * Examples:
 *
 *     json
 *     cbor
 *     vendor::format
 *     organization::format
 *     dialect::format
 *     future::representation
 *
 * Format identity is semantic data.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It contains:
 *
 *   - no target-language actions;
 *   - no semantic predicates;
 *   - no filesystem access;
 *   - no network access;
 *   - no environment access;
 *   - no hardware discovery;
 *   - no runtime execution;
 *   - no unsafe Rust.
 *
 * Generated parser integration MUST remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * and Rust 2021.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *   - source text;
 *   - selected grammar version;
 *   - shared lexical vocabulary;
 *   - explicitly selected dialect configuration.
 *
 * Parsing MUST NOT depend on:
 *
 *   - hardware;
 *   - runtime resources;
 *   - filesystem state;
 *   - network state;
 *   - wall-clock time;
 *   - randomness;
 *   - environment variables.
 *
 * ============================================================================
 * SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * Every construct recognized here must remain representable with its source
 * span by the domain-neutral AST.
 *
 * The grammar itself does not construct AST nodes.
 *
 * The AST contract must preserve at least:
 *
 *   - operation kind;
 *   - source expression;
 *   - target/source expression;
 *   - format reference;
 *   - target type where applicable;
 *   - options;
 *   - contract identity;
 *   - source span.
 *
 * ============================================================================
 * AST / SEMANTIC / IR CONTRACT
 * ============================================================================
 *
 * serializationStatement
 *     -> generic serialization operation AST
 *
 * deserializationStatement
 *     -> generic deserialization operation AST
 *
 * serializationExpression
 *     -> expression-level serialization operation
 *
 * deserializationExpression
 *     -> expression-level deserialization operation
 *
 * serializationContractDeclaration
 *     -> reusable serialization contract AST
 *
 * semantic analysis resolves:
 *
 *     format
 *     schema
 *     version
 *     compatibility
 *     policies
 *     capabilities
 *     resource requirements
 *
 * Lowering produces the repository's canonical data/interoperability
 * representation.
 *
 * This grammar does NOT define that IR.
 *
 * ============================================================================
 */

parser grammar serialization;

options {
    /*
     * IMPORTANT:
     *
     * The repository's canonical lexer composition is:
     *
     *     grammar/antlr/ZamaniLexer.g4
     *
     * Therefore parser grammars must consume ZamaniLexer.
     *
     * Do NOT revert this to:
     *
     *     tokenVocab = Zamani;
     */
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * PUBLIC TEST ENTRY POINT
 * ============================================================================
 *
 * This is for isolated grammar/conformance tests.
 *
 * It is NOT the complete Zamani program entry point.
 * ============================================================================
 */

serializationUnit
    : serializationConstruct* EOF
    ;


/* ============================================================================
 * PUBLIC INTEGRATION FACADE
 * ============================================================================
 *
 * `data.g4` and the canonical Data composition grammar should delegate here.
 *
 * These names are intentionally stable integration points.
 * ============================================================================
 */

dataSerializationStmt
    : serializationStatement
    | deserializationStatement
    ;

dataSerializationExpression
    : serializationExpression
    | deserializationExpression
    ;

dataSerializationContract
    : serializationContractDeclaration
    ;

serializationConstruct
    : dataSerializationStmt
    | dataSerializationExpression
    | dataSerializationContract
    ;


/* ============================================================================
 * SERIALIZATION STATEMENT
 * ============================================================================
 *
 * Canonical forms:
 *
 *     serialize value using format;
 *
 *     serialize value to destination using format;
 *
 *     serialize value using format with options;
 *
 * The destination is optional.
 *
 * Format is explicit in the statement form so that the operation cannot be
 * accidentally confused with an ordinary expression.
 * ============================================================================
 */

serializationStatement
    : SERIALIZE
      expression
      serializationDestinationClause?
      serializationFormatClause
      serializationOptionBlock?
      SEMICOLON
    ;

serializationDestinationClause
    : TO
      expression
    ;

serializationFormatClause
    : USING
      serializationFormatReference
    ;


/* ============================================================================
 * DESERIALIZATION STATEMENT
 * ============================================================================
 *
 * Canonical forms:
 *
 *     deserialize source using format as Type;
 *
 *     deserialize source from source using format as Type;
 *
 * The source expression itself is sufficient in the common case.
 * ============================================================================
 */

deserializationStatement
    : DESERIALIZE
      expression
      deserializationSourceClause?
      serializationFormatClause
      deserializationTargetClause
      serializationOptionBlock?
      SEMICOLON
    ;

deserializationSourceClause
    : FROM
      expression
    ;

deserializationTargetClause
    : AS
      typeExpr
    ;


/* ============================================================================
 * EXPRESSION FORMS
 * ============================================================================
 *
 * Expression forms are useful for:
 *
 *     let encoded = serialize(value) using format;
 *
 *     pipeline(...)
 *
 *     transform(serialize(value) using format)
 *
 *     let value = deserialize(source) using format as Record;
 *
 * The expression grammar owns precedence; this grammar only introduces the
 * serialization operation.
 * ============================================================================
 */

serializationExpression
    : SERIALIZE
      LPAREN
      expression
      serializationExpressionFormatClause?
      serializationExpressionOptions?
      RPAREN
    ;

serializationExpressionFormatClause
    : USING
      serializationFormatReference
    | TO
      serializationFormatReference
    ;

deserializationExpression
    : DESERIALIZE
      LPAREN
      expression
      serializationExpressionFormatClause?
      serializationExpressionTargetClause?
      serializationExpressionOptions?
      RPAREN
    ;

serializationExpressionTargetClause
    : AS
      typeExpr
    ;

serializationExpressionOptions
    : WITH
      serializationOptionList
    ;


/* ============================================================================
 * FORMAT REFERENCES
 * ============================================================================
 *
 * A format is a symbolic semantic identifier.
 *
 * It may be:
 *
 *     json
 *     cbor
 *     protobuf
 *     vendor::format
 *     organization::domain::format
 *
 * No finite registry is embedded in the grammar.
 * ============================================================================
 */

serializationFormatReference
    : qualifiedName
    ;


/* ============================================================================
 * OPTION BLOCK
 * ============================================================================
 *
 * Example:
 *
 *     serialize value using format
 *         with {
 *             schema = telemetry::Event;
 *             version = 2;
 *             compatibility = backward;
 *         };
 *
 * A repeated option list is deliberately unbounded.
 * ============================================================================
 */

serializationOptionBlock
    : WITH
      LBRACE
      serializationOption*
      RBRACE
    ;

serializationOptionList
    : serializationOption
      (COMMA serializationOption)*
    ;


/* ============================================================================
 * OPTION DISPATCH
 * ============================================================================
 */

serializationOption
    : serializationSchemaOption
    | serializationTypeOption
    | serializationVersionOption
    | serializationSchemaVersionOption
    | serializationCompatibilityOption
    | serializationEvolutionOption
    | serializationCanonicalOption
    | serializationFramingOption
    | serializationCompressionOption
    | serializationIntegrityOption
    | serializationEncodingOption
    | serializationByteOrderOption
    | serializationNullPolicyOption
    | serializationMissingPolicyOption
    | serializationUnknownFieldOption
    | serializationDuplicateFieldOption
    | serializationDefaultPolicyOption
    | serializationFieldPolicyOption
    | serializationMetadataOption
    | serializationPropertyOption
    | serializationStreamOption
    | serializationChunkOption
    | serializationValidationOption
    | serializationPolicyOption
    | serializationCapabilityOption
    | serializationRequirementOption
    | serializationPreferenceOption
    | serializationHintOption
    | serializationExtensionOption
    ;


/* ============================================================================
 * SCHEMA
 * ============================================================================
 */

serializationSchemaOption
    : SCHEMA
      qualifiedName
    ;

serializationTypeOption
    : TYPE
      typeExpr
    ;


/* ============================================================================
 * VERSIONING
 * ============================================================================
 *
 * Version values are expressions rather than fixed integer grammar values.
 *
 * This permits:
 *
 *     version = 1;
 *     version = schema_version;
 *     version = negotiated_version;
 *
 * Semantic validation determines whether the resulting value is a valid
 * representation/version identifier.
 * ============================================================================
 */

serializationVersionOption
    : VERSION
      ASSIGN
      expression
    ;

serializationSchemaVersionOption
    : SCHEMA_VERSION
      ASSIGN
      expression
    ;


/* ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Compatibility policy is symbolic and extensible.
 *
 * Examples:
 *
 *     compatibility = backward;
 *     compatibility = forward;
 *     compatibility = vendor::policy;
 *
 * The grammar does not define the actual compatibility algorithm.
 * ============================================================================
 */

serializationCompatibilityOption
    : COMPATIBILITY
      ASSIGN
      serializationPolicyReference
    ;

serializationEvolutionOption
    : EVOLUTION
      ASSIGN
      serializationPolicyReference
    ;

serializationPolicyReference
    : qualifiedName
    ;


/* ============================================================================
 * CANONICALIZATION
 * ============================================================================
 */

serializationCanonicalOption
    : CANONICAL
      serializationBooleanOrPolicyValue?
    ;

serializationBooleanOrPolicyValue
    : ASSIGN
      expression
    | LPAREN
      expression
      RPAREN
    ;


/* ============================================================================
 * FRAMING
 * ============================================================================
 *
 * Framing describes logical representation boundaries.
 *
 * It does NOT define network packets.
 * ============================================================================
 */

serializationFramingOption
    : FRAMING
      ASSIGN
      serializationPolicyReference
    ;


/* ============================================================================
 * COMPRESSION
 * ============================================================================
 */

serializationCompressionOption
    : COMPRESSION
      ASSIGN
      serializationPolicyReference
    ;


/* ============================================================================
 * INTEGRITY
 * ============================================================================
 *
 * Integrity is intentionally distinct from encryption.
 *
 * Encryption/security implementation belongs to the security/interoperability
 * layers.
 * ============================================================================
 */

serializationIntegrityOption
    : INTEGRITY
      ASSIGN
      serializationPolicyReference
    ;


/* ============================================================================
 * TEXT ENCODING
 * ============================================================================
 *
 * Encoding names remain symbolic.
 *
 * The grammar does not privilege a finite encoding list.
 * ============================================================================
 */

serializationEncodingOption
    : ENCODING
      ASSIGN
      serializationPolicyReference
    ;


/* ============================================================================
 * BYTE ORDER
 * ============================================================================
 */

serializationByteOrderOption
    : BYTE_ORDER
      ASSIGN
      serializationPolicyReference
    ;


/* ============================================================================
 * NULL / MISSING VALUES
 * ============================================================================
 */

serializationNullPolicyOption
    : NULL_POLICY
      ASSIGN
      serializationPolicyReference
    ;

serializationMissingPolicyOption
    : MISSING_POLICY
      ASSIGN
      serializationPolicyReference
    ;


/* ============================================================================
 * UNKNOWN FIELDS
 * ============================================================================
 */

serializationUnknownFieldOption
    : UNKNOWN_FIELDS
      ASSIGN
      serializationPolicyReference
    ;


/* ============================================================================
 * DUPLICATE FIELDS
 * ============================================================================
 */

serializationDuplicateFieldOption
    : DUPLICATE_FIELDS
      ASSIGN
      serializationPolicyReference
    ;


/* ============================================================================
 * DEFAULT VALUES
 * ============================================================================
 */

serializationDefaultPolicyOption
    : DEFAULT_POLICY
      ASSIGN
      serializationPolicyReference
    ;


/* ============================================================================
 * FIELD REPRESENTATION
 * ============================================================================
 */

serializationFieldPolicyOption
    : FIELD_POLICY
      ASSIGN
      serializationPolicyReference
    ;


/* ============================================================================
 * METADATA
 * ============================================================================
 *
 * Metadata is an expression so arbitrary structured metadata can be preserved
 * without introducing another metadata language.
 * ============================================================================
 */

serializationMetadataOption
    : METADATA
      ASSIGN
      expression
    ;


/* ============================================================================
 * EXTENSIBLE FORMAT PROPERTY
 * ============================================================================
 *
 * Properties allow a format-specific representation policy without requiring
 * every new format to modify the core grammar.
 *
 * Example:
 *
 *     property = vendor::property(value)
 *
 * ============================================================================
 */

serializationPropertyOption
    : PROPERTY
      serializationPropertyName
      ASSIGN
      expression
    ;

serializationPropertyName
    : qualifiedName
    ;


/* ============================================================================
 * STREAMING
 * ============================================================================
 *
 * Streaming is semantic intent.
 *
 * It does not impose a fixed buffer size, chunk count, record count, or
 * transport implementation.
 * ============================================================================
 */

serializationStreamOption
    : STREAMING
      ASSIGN
      serializationPolicyReference
    ;


/* ============================================================================
 * CHUNKING
 * ============================================================================
 *
 * Chunking is representation intent.
 *
 * A chunk size may be expressed symbolically or numerically.
 *
 * The grammar imposes no maximum or minimum.
 * ============================================================================
 */

serializationChunkOption
    : CHUNKING
      ASSIGN
      expression
    ;


/* ============================================================================
 * VALIDATION
 * ============================================================================
 */

serializationValidationOption
    : VALIDATE
      ASSIGN
      serializationPolicyReference
    ;


/* ============================================================================
 * CAPABILITIES / REQUIREMENTS
 * ============================================================================
 *
 * These are source-level requirements, not backend selections.
 *
 * Examples:
 *
 *     capability = serialization::streaming;
 *     requires = capability("serialization.canonical");
 *
 * Actual capability checking occurs downstream.
 * ============================================================================
 */

serializationCapabilityOption
    : CAPABILITY
      ASSIGN
      expression
    ;

serializationRequirementOption
    : REQUIRES
      ASSIGN
      expression
    ;

serializationPreferenceOption
    : PREFER
      ASSIGN
      expression
    ;

serializationHintOption
    : HINT
      ASSIGN
      expression
    ;


/* ============================================================================
 * GENERIC POLICY
 * ============================================================================
 */

serializationPolicyOption
    : POLICY
      ASSIGN
      serializationPolicyReference
    ;


/* ============================================================================
 * DIALECT / FUTURE EXTENSION
 * ============================================================================
 *
 * Extensions remain qualified and are interpreted by the dialect/semantic
 * layer. The core grammar does not need to know future provider features.
 * ============================================================================
 */

serializationExtensionOption
    : EXTENSION
      serializationExtensionReference
    ;

serializationExtensionReference
    : qualifiedName
    ;


/* ============================================================================
 * SERIALIZATION CONTRACT
 * ============================================================================
 *
 * A contract packages reusable representation intent.
 *
 * Example:
 *
 *     serialization contract telemetry::EventWire {
 *         format = telemetry::binary;
 *         schema = telemetry::Event;
 *         version = 1;
 *         compatibility = backward;
 *     }
 *
 * Contracts do not implement codecs.
 * ============================================================================
 */

serializationContractDeclaration
    : visibilityModifier?
      SERIALIZATION
      CONTRACT
      qualifiedName
      serializationContractTypeParameters?
      LBRACE
      serializationContractMember*
      RBRACE
    ;

serializationContractTypeParameters
    : LESS_THAN
      serializationTypeParameter
      (COMMA serializationTypeParameter)*
      GREATER_THAN
    ;

serializationTypeParameter
    : IDENTIFIER
      (COLON typeExpr)?
    ;


/* ============================================================================
 * CONTRACT MEMBERS
 * ============================================================================
 */

serializationContractMember
    : serializationContractFormat
    | serializationContractSchema
    | serializationContractType
    | serializationContractVersion
    | serializationContractSchemaVersion
    | serializationContractCompatibility
    | serializationContractEvolution
    | serializationContractCanonicalization
    | serializationContractFraming
    | serializationContractCompression
    | serializationContractIntegrity
    | serializationContractEncoding
    | serializationContractByteOrder
    | serializationContractNullPolicy
    | serializationContractMissingPolicy
    | serializationContractUnknownFields
    | serializationContractDuplicateFields
    | serializationContractDefaultPolicy
    | serializationContractFieldPolicy
    | serializationContractMetadata
    | serializationContractProperty
    | serializationContractStreaming
    | serializationContractChunking
    | serializationContractValidation
    | serializationContractPolicy
    | serializationContractCapability
    | serializationContractRequirement
    | serializationContractPreference
    | serializationContractHint
    | serializationContractExtension
    | annotation
    ;


/* ============================================================================
 * CONTRACT FORMAT
 * ============================================================================
 */

serializationContractFormat
    : FORMAT
      ASSIGN
      serializationFormatReference
      SEMICOLON
    ;


/* ============================================================================
 * CONTRACT SCHEMA
 * ============================================================================
 */

serializationContractSchema
    : SCHEMA
      ASSIGN
      qualifiedName
      SEMICOLON
    ;


/* ============================================================================
 * CONTRACT TYPE
 * ============================================================================
 */

serializationContractType
    : TYPE
      ASSIGN
      typeExpr
      SEMICOLON
    ;


/* ============================================================================
 * CONTRACT VERSION
 * ============================================================================
 */

serializationContractVersion
    : VERSION
      ASSIGN
      expression
      SEMICOLON
    ;

serializationContractSchemaVersion
    : SCHEMA_VERSION
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * CONTRACT COMPATIBILITY
 * ============================================================================
 */

serializationContractCompatibility
    : COMPATIBILITY
      ASSIGN
      serializationPolicyReference
      SEMICOLON
    ;

serializationContractEvolution
    : EVOLUTION
      ASSIGN
      serializationPolicyReference
      SEMICOLON
    ;


/* ============================================================================
 * CONTRACT CANONICALIZATION
 * ============================================================================
 */

serializationContractCanonicalization
    : CANONICAL
      serializationBooleanOrPolicyValue?
      SEMICOLON
    ;


/* ============================================================================
 * CONTRACT FRAMING
 * ============================================================================
 */

serializationContractFraming
    : FRAMING
      ASSIGN
      serializationPolicyReference
      SEMICOLON
    ;


/* ============================================================================
 * CONTRACT COMPRESSION
 * ============================================================================
 */

serializationContractCompression
    : COMPRESSION
      ASSIGN
      serializationPolicyReference
      SEMICOLON
    ;


/* ============================================================================
 * CONTRACT INTEGRITY
 * ============================================================================
 */

serializationContractIntegrity
    : INTEGRITY
      ASSIGN
      serializationPolicyReference
      SEMICOLON
    ;


/* ============================================================================
 * CONTRACT ENCODING
 * ============================================================================
 */

serializationContractEncoding
    : ENCODING
      ASSIGN
      serializationPolicyReference
      SEMICOLON
    ;


/* ============================================================================
 * CONTRACT BYTE ORDER
 * ============================================================================
 */

serializationContractByteOrder
    : BYTE_ORDER
      ASSIGN
      serializationPolicyReference
      SEMICOLON
    ;


/* ============================================================================
 * CONTRACT NULL / MISSING POLICY
 * ============================================================================
 */

serializationContractNullPolicy
    : NULL_POLICY
      ASSIGN
      serializationPolicyReference
      SEMICOLON
    ;

serializationContractMissingPolicy
    : MISSING_POLICY
      ASSIGN
      serializationPolicyReference
      SEMICOLON
    ;


/* ============================================================================
 * CONTRACT UNKNOWN / DUPLICATE FIELD POLICY
 * ============================================================================
 */

serializationContractUnknownFields
    : UNKNOWN_FIELDS
      ASSIGN
      serializationPolicyReference
      SEMICOLON
    ;

serializationContractDuplicateFields
    : DUPLICATE_FIELDS
      ASSIGN
      serializationPolicyReference
      SEMICOLON
    ;


/* ============================================================================
 * CONTRACT DEFAULT / FIELD POLICY
 * ============================================================================
 */

serializationContractDefaultPolicy
    : DEFAULT_POLICY
      ASSIGN
      serializationPolicyReference
      SEMICOLON
    ;

serializationContractFieldPolicy
    : FIELD_POLICY
      ASSIGN
      serializationPolicyReference
      SEMICOLON
    ;


/* ============================================================================
 * CONTRACT METADATA
 * ============================================================================
 */

serializationContractMetadata
    : METADATA
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * CONTRACT PROPERTY
 * ============================================================================
 */

serializationContractProperty
    : PROPERTY
      serializationPropertyName
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * CONTRACT STREAMING / CHUNKING
 * ============================================================================
 */

serializationContractStreaming
    : STREAMING
      ASSIGN
      serializationPolicyReference
      SEMICOLON
    ;

serializationContractChunking
    : CHUNKING
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * CONTRACT VALIDATION
 * ============================================================================
 */

serializationContractValidation
    : VALIDATE
      ASSIGN
      serializationPolicyReference
      SEMICOLON
    ;


/* ============================================================================
 * CONTRACT CAPABILITY / REQUIREMENT / PREFERENCE / HINT
 * ============================================================================
 */

serializationContractCapability
    : CAPABILITY
      ASSIGN
      expression
      SEMICOLON
    ;

serializationContractRequirement
    : REQUIRES
      ASSIGN
      expression
      SEMICOLON
    ;

serializationContractPreference
    : PREFER
      ASSIGN
      expression
      SEMICOLON
    ;

serializationContractHint
    : HINT
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * CONTRACT POLICY
 * ============================================================================
 */

serializationContractPolicy
    : POLICY
      ASSIGN
      serializationPolicyReference
      SEMICOLON
    ;


/* ============================================================================
 * CONTRACT EXTENSION
 * ============================================================================
 */

serializationContractExtension
    : EXTENSION
      serializationExtensionReference
      SEMICOLON
    ;


/* ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * This grammar is intentionally dependent only on shared canonical rules:
 *
 *     expression
 *     typeExpr
 *     qualifiedName
 *     visibilityModifier
 *     annotation
 *
 * and canonical ZamaniLexer tokens.
 *
 * It does NOT redefine:
 *
 *     expression
 *     typeExpr
 *     identifier
 *     qualifiedName
 *     literal
 *     block
 *     attributes
 *     source spans
 *
 * Therefore the semantic owner of those constructs remains outside this file.
 *
 * ============================================================================
 */
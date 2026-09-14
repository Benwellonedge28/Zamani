/*
 * ============================================================================
 * Zamani Universal Data Grammar
 * File: grammar/data/deserialization.g4
 * ============================================================================
 *
 * PURPOSE
 * -------
 * Authoritative parser grammar for the SOURCE-LEVEL DESERIALIZATION
 * CONTRACT of Zamani.
 *
 * This grammar describes WHAT a program means when it requests conversion
 * from an encoded/external representation into a Zamani value.
 *
 * It deliberately does NOT implement decoding.
 *
 *
 * ARCHITECTURAL OWNERSHIP
 * -----------------------
 *
 * THIS FILE OWNS:
 *
 *   - deserialization statement syntax
 *   - deserialization expression syntax
 *   - source/format association
 *   - target type intent
 *   - schema intent
 *   - validation intent
 *   - version-selection intent
 *   - encoding/framing intent
 *   - compatibility intent
 *   - canonicalization intent
 *   - unknown-field policy intent
 *   - duplicate-field policy intent
 *   - numeric representation intent
 *   - null/missing-value intent
 *   - reference resolution intent
 *   - resource-neutral decoding requirements
 *   - extensible format identifiers
 *   - format-version identifiers
 *   - dialect identifiers
 *
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - JSON/CBOR/Protobuf/etc. decoders
 *   - serialization libraries
 *   - filesystem access
 *   - network access
 *   - database access
 *   - memory allocation
 *   - buffering implementation
 *   - streaming implementation
 *   - schema registry implementation
 *   - compression implementation
 *   - encryption implementation
 *   - cryptographic verification implementation
 *   - hardware discovery
 *   - hardware resources
 *   - scheduling
 *   - routing
 *   - optimization
 *   - QEC
 *   - ZQN
 *   - quantum IR
 *   - classical IR
 *   - runtime resource discovery
 *   - backend/provider selection
 *
 *
 * POCO-REAF
 * ---------
 *
 * A Zamani program expresses semantic intent.
 *
 * The source program must NOT encode:
 *
 *   - fixed memory capacity
 *   - fixed buffer size
 *   - fixed device count
 *   - fixed node count
 *   - fixed CPU count
 *   - fixed accelerator count
 *   - fixed network capacity
 *   - fixed storage capacity
 *   - fixed maximum record size
 *   - fixed maximum collection size
 *
 * Deserialization requirements are therefore expressed symbolically.
 *
 *
 * SCALABILITY
 * ----------
 *
 * There are intentionally no grammar-level limits on:
 *
 *   - fields
 *   - records
 *   - nested values
 *   - collection elements
 *   - schema members
 *   - input size
 *   - encoded size
 *   - nesting depth
 *   - number of formats
 *   - number of format versions
 *   - number of schemas
 *   - number of decoder options
 *   - number of compatibility rules
 *
 * Any actual resource limitation belongs to semantic validation,
 * compilation, runtime, or resource management.
 *
 *
 * EXTENSIBILITY
 * ------------
 *
 * Built-in formats may be recognized by the root language vocabulary,
 * but this grammar deliberately permits qualified format identifiers.
 *
 * Therefore the language can evolve from:
 *
 *     json
 *     cbor
 *     protobuf
 *
 * toward:
 *
 *     vendor::format
 *     organization::format
 *     dialect::format
 *     format::version
 *
 * without modifying this grammar merely because a new format is introduced.
 *
 *
 * INTEGRATION
 * -----------
 *
 * Intended grammar composition:
 *
 *     Zamani.g4
 *          |
 *          +-- data/data.g4
 *                    |
 *                    +-- data/deserialization.g4
 *
 * The root grammar owns the common lexer vocabulary.
 *
 * This grammar is a PARSER grammar and therefore MUST NOT define a second
 * lexer.
 *
 * The generated parser consumes the shared Zamani token vocabulary.
 *
 *
 * IMPORTANT
 * ---------
 *
 * This grammar contains NO target-language actions.
 *
 * Consequently:
 *
 *   - no unsafe Rust
 *   - no filesystem access
 *   - no network access
 *   - no hidden I/O
 *   - no provider-specific runtime behavior
 *   - no target-language coupling
 *
 * Rust 1.97 / 1.97.1 compatibility is established by the generated-parser
 * integration/build layer, not by embedding Rust code in this grammar.
 *
 * ============================================================================
 */

parser grammar ZamaniDataDeserializationParser;

options {
    tokenVocab = Zamani;
}


/* ============================================================================
 * PUBLIC INTEGRATION ENTRY POINTS
 * ============================================================================
 *
 * `dataDeserializationStmt` is intended to be imported/re-exported by
 * grammar/data/data.g4 and ultimately reached from the root `dataStmt`.
 *
 * `dataDeserializationExpression` is the expression form.
 *
 * Keeping statement and expression forms separate allows:
 *
 *     deserialize source from json;
 *
 * and:
 *
 *     let value = deserialize(source from json);
 *
 * to share the same semantic model without duplicating grammar.
 * ============================================================================
 */

dataDeserializationStmt
    : dataDeserializeStatement
    ;

dataDeserializationExpression
    : dataDeserializeExpression
    ;


/* ============================================================================
 * STATEMENT FORM
 * ============================================================================
 */

dataDeserializeStatement
    : 'deserialize'
      dataDeserializationSource
      'from'
      dataFormatSpec
      dataDeserializationClause*
      ';'
    ;


/* ============================================================================
 * EXPRESSION FORM
 * ============================================================================
 */

dataDeserializeExpression
    : 'deserialize'
      '('
      dataDeserializationSource
      'from'
      dataFormatSpec
      dataDeserializationClause*
      ')'
    ;


/* ============================================================================
 * SOURCE
 * ============================================================================
 *
 * The source is deliberately an expression.
 *
 * It may therefore represent:
 *
 *   - an in-memory byte sequence
 *   - a string
 *   - a stream
 *   - a data source
 *   - a network result
 *   - a storage result
 *   - another expression
 *   - a provider-independent data handle
 *
 * This grammar does not decide where the bytes came from.
 * ============================================================================
 */

dataDeserializationSource
    : expression
    ;


/* ============================================================================
 * FORMAT SPECIFICATION
 * ============================================================================
 *
 * Format is a logical description, not an implementation/provider binding.
 *
 * Examples:
 *
 *     json
 *     cbor
 *     protobuf
 *     messagepack
 *     vendor::format
 *     vendor::format::v2
 *
 * A format may optionally carry a version and dialect.
 * ============================================================================
 */

dataFormatSpec
    : dataFormatReference
      dataFormatVersion?
      dataFormatDialect?
      dataFormatArguments?
    ;


/* ============================================================================
 * FORMAT REFERENCE
 * ============================================================================
 */

dataFormatReference
    : dataBuiltinFormat
    | dataQualifiedFormat
    | dataFormatStringReference
    ;


/*
 * Built-in/common formats remain syntactically convenient.
 *
 * They are NOT the complete universe of supported formats.
 *
 * The semantic layer is responsible for determining whether a decoder for
 * the requested format exists.
 */
dataBuiltinFormat
    : 'json'
    | 'xml'
    | 'yaml'
    | 'toml'
    | 'cbor'
    | 'messagepack'
    | 'msgpack'
    | 'protobuf'
    | 'avro'
    | 'parquet'
    | 'arrow'
    | 'csv'
    | 'tsv'
    | 'ndjson'
    | 'text'
    | 'binary'
    ;


/*
 * Extensible format namespace.
 *
 * No fixed number of namespace components is imposed.
 */
dataQualifiedFormat
    : IDENTIFIER
      ('::' IDENTIFIER)+
    ;


/*
 * String-based format references are useful for dynamically registered
 * formats and compatibility layers.
 *
 * Semantic validation MUST reject unknown or unauthorized formats.
 */
dataFormatStringReference
    : STRING
    ;


/* ============================================================================
 * FORMAT VERSION
 * ============================================================================
 *
 * Versions are semantic metadata.
 *
 * They are not used to encode parser implementation versions.
 * ============================================================================
 */

dataFormatVersion
    : 'version'
      dataVersionValue
    | '@'
      dataVersionValue
    ;

dataVersionValue
    : IDENTIFIER
    | INTEGER
    | STRING
    ;


/* ============================================================================
 * FORMAT DIALECT
 * ============================================================================
 *
 * A dialect identifies a semantic variant of a format without tying Zamani
 * to a particular vendor implementation.
 * ============================================================================
 */

dataFormatDialect
    : 'dialect'
      dataDialectReference
    ;

dataDialectReference
    : IDENTIFIER
    | dataQualifiedName
    | STRING
    ;


/* ============================================================================
 * FORMAT ARGUMENTS
 * ============================================================================
 *
 * Arguments describe logical format parameters.
 *
 * They are NOT arbitrary implementation configuration.
 * ============================================================================
 */

dataFormatArguments
    : '('
      dataFormatArgumentList?
      ')'
    ;

dataFormatArgumentList
    : dataFormatArgument
      (',' dataFormatArgument)*
    ;

dataFormatArgument
    : IDENTIFIER
      '='
      expression
    ;


/* ============================================================================
 * DESERIALIZATION CLAUSES
 * ============================================================================
 *
 * Clauses are intentionally orthogonal.
 *
 * A compiler/semantic analyzer can reject incompatible combinations without
 * requiring the grammar to enumerate every future combination.
 * ============================================================================
 */

dataDeserializationClause
    : dataDeserializeTargetClause
    | dataDeserializeSchemaClause
    | dataDeserializeValidationClause
    | dataDeserializeCompatibilityClause
    | dataDeserializeEncodingClause
    | dataDeserializeFramingClause
    | dataDeserializeUnknownFieldClause
    | dataDeserializeDuplicateFieldClause
    | dataDeserializeNullClause
    | dataDeserializeMissingFieldClause
    | dataDeserializeNumericClause
    | dataDeserializeReferenceClause
    | dataDeserializeCanonicalClause
    | dataDeserializeStrictnessClause
    | dataDeserializeOptionClause
    ;


/* ============================================================================
 * TARGET TYPE
 * ============================================================================
 *
 * `into` establishes the desired logical Zamani type.
 *
 * It does NOT select a machine representation.
 * ============================================================================
 */

dataDeserializeTargetClause
    : 'into'
      typeExpr
    ;


/* ============================================================================
 * SCHEMA
 * ============================================================================
 *
 * A schema reference may identify:
 *
 *   - a local schema
 *   - an imported schema
 *   - a versioned schema
 *   - a registry-neutral logical schema
 *
 * Physical schema registry resolution belongs elsewhere.
 * ============================================================================
 */

dataDeserializeSchemaClause
    : 'schema'
      dataSchemaReference
    ;

dataSchemaReference
    : dataQualifiedName
      dataSchemaVersion?
    | STRING
    ;

dataSchemaVersion
    : 'version'
      dataVersionValue
    ;


/* ============================================================================
 * VALIDATION
 * ============================================================================
 */

dataDeserializeValidationClause
    : 'validate'
      dataValidationMode
    ;

dataValidationMode
    : 'none'
    | 'structural'
    | 'schema'
    | 'semantic'
    | 'strict'
    | 'custom'
      '(' expression ')'
    ;


/* ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Compatibility is a semantic policy.
 *
 * The grammar does not assume that a newer or older schema is always safe.
 * ============================================================================
 */

dataDeserializeCompatibilityClause
    : 'compatibility'
      dataCompatibilityMode
    ;

dataCompatibilityMode
    : 'exact'
    | 'backward'
    | 'forward'
    | 'full'
    | 'permissive'
    | 'custom'
      '(' expression ')'
    ;


/* ============================================================================
 * ENCODING
 * ============================================================================
 */

dataDeserializeEncodingClause
    : 'encoding'
      dataEncodingReference
    ;

dataEncodingReference
    : IDENTIFIER
    | dataQualifiedName
    | STRING
    ;


/* ============================================================================
 * FRAMING
 * ============================================================================
 *
 * Framing is especially important for streams and concatenated encoded
 * values.
 * ============================================================================
 */

dataDeserializeFramingClause
    : 'framing'
      dataFramingMode
    ;

dataFramingMode
    : 'single'
    | 'delimited'
    | 'length_prefixed'
    | 'self_describing'
    | 'stream'
    | 'custom'
      '(' expression ')'
    ;


/* ============================================================================
 * UNKNOWN FIELD POLICY
 * ============================================================================
 */

dataDeserializeUnknownFieldClause
    : 'unknown_fields'
      dataUnknownFieldMode
    ;

dataUnknownFieldMode
    : 'reject'
    | 'ignore'
    | 'preserve'
    | 'capture'
      dataCaptureTarget?
    ;

dataCaptureTarget
    : 'as'
      IDENTIFIER
    ;


/* ============================================================================
 * DUPLICATE FIELD POLICY
 * ============================================================================
 */

dataDeserializeDuplicateFieldClause
    : 'duplicate_fields'
      dataDuplicateFieldMode
    ;

dataDuplicateFieldMode
    : 'reject'
    | 'first'
    | 'last'
    | 'preserve'
    | 'merge'
    | 'custom'
      '(' expression ')'
    ;


/* ============================================================================
 * NULL POLICY
 * ============================================================================
 */

dataDeserializeNullClause
    : 'nulls'
      dataNullMode
    ;

dataNullMode
    : 'allow'
    | 'reject'
    | 'default'
    | 'custom'
      '(' expression ')'
    ;


/* ============================================================================
 * MISSING FIELD POLICY
 * ============================================================================
 */

dataDeserializeMissingFieldClause
    : 'missing'
      dataMissingFieldMode
    ;

dataMissingFieldMode
    : 'reject'
    | 'default'
    | 'optional'
    | 'preserve'
    | 'custom'
      '(' expression ')'
    ;


/* ============================================================================
 * NUMERIC REPRESENTATION
 * ============================================================================
 *
 * Numeric representation must remain semantic.
 *
 * The grammar does not force a fixed machine width.
 * ============================================================================
 */

dataDeserializeNumericClause
    : 'numbers'
      dataNumericMode
    ;

dataNumericMode
    : 'native'
    | 'exact'
    | 'decimal'
    | 'integer'
    | 'floating'
    | 'rational'
    | 'arbitrary'
    | 'custom'
      '(' expression ')'
    ;


/* ============================================================================
 * REFERENCE RESOLUTION
 * ============================================================================
 */

dataDeserializeReferenceClause
    : 'references'
      dataReferenceMode
    ;

dataReferenceMode
    : 'preserve'
    | 'resolve'
    | 'reject'
    | 'defer'
    | 'custom'
      '(' expression ')'
    ;


/* ============================================================================
 * CANONICALIZATION
 * ============================================================================
 *
 * Canonicalization can be important for deterministic compilation,
 * reproducibility, hashing, signing, caching, and provenance.
 * ============================================================================
 */

dataDeserializeCanonicalClause
    : 'canonical'
      dataCanonicalMode
    ;

dataCanonicalMode
    : 'require'
    | 'prefer'
    | 'allow'
    | 'reject'
    ;


/* ============================================================================
 * STRICTNESS
 * ============================================================================
 */

dataDeserializeStrictnessClause
    : 'strictness'
      dataStrictnessMode
    ;

dataStrictnessMode
    : 'strict'
    | 'standard'
    | 'permissive'
    | 'custom'
      '(' expression ')'
    ;


/* ============================================================================
 * EXTENSIBLE DESERIALIZATION OPTIONS
 * ============================================================================
 *
 * Named options provide forward compatibility without requiring grammar
 * changes for every future serialization technology.
 *
 * Example:
 *
 *     option preserve_order = true
 *
 * Semantic validation owns whether an option is legal for a format.
 * ============================================================================
 */

dataDeserializeOptionClause
    : 'option'
      IDENTIFIER
      '='
      expression
    ;


/* ============================================================================
 * SHARED NAME SUPPORT
 * ============================================================================
 *
 * Kept local to this grammar so format/schema/dialect names can be qualified
 * without imposing a new global naming model.
 * ============================================================================
 */

dataQualifiedName
    : IDENTIFIER
      ('::' IDENTIFIER)*
    ;


/* ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * The containing data grammar MUST expose:
 *
 *     dataSerializationStmt
 *
 * only through the data subsystem's canonical statement dispatcher.
 *
 * It MUST NOT duplicate the rules in this file.
 *
 * Recommended data.g4 integration:
 *
 *     dataStmt
 *         : ...
 *         | dataDeserializationStmt
 *         | ...
 *         ;
 *
 * and:
 *
 *     dataPipelineStageExpression
 *         : ...
 *         | dataDeserializationExpression
 *         ;
 *
 *
 * The root grammar MUST NOT implement a second:
 *
 *     'deserialize' ...
 *
 * rule.
 *
 * The root `statement` -> `dataStmt` boundary remains the language-level
 * integration point.
 *
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes only syntactic intent.
 *
 * Semantic analysis MUST subsequently determine:
 *
 *   1. whether the format exists;
 *   2. whether the requested format version exists;
 *   3. whether the dialect exists;
 *   4. whether a decoder is available;
 *   5. whether the source type is compatible;
 *   6. whether the requested target type is compatible;
 *   7. whether schema evolution is legal;
 *   8. whether validation policy is satisfiable;
 *   9. whether unknown-field policy is supported;
 *  10. whether duplicate-field policy is supported;
 *  11. whether numeric conversion is lossless or intentionally lossy;
 *  12. whether reference resolution is permitted;
 *  13. whether canonicalization is available;
 *  14. whether requested capabilities exist;
 *  15. whether security policy permits the operation;
 *  16. whether runtime resources are sufficient.
 *
 * None of those decisions belong in this grammar.
 *
 *
 * ============================================================================
 * RESOURCE / SCALABILITY CONTRACT
 * ============================================================================
 *
 * No production rule contains a fixed cardinality representing:
 *
 *   - records
 *   - fields
 *   - elements
 *   - bytes
 *   - buffers
 *   - nodes
 *   - devices
 *   - cores
 *   - threads
 *   - qubits
 *   - storage capacity
 *
 * Repetition is represented using ANTLR repetition operators or semantic
 * expressions, allowing the implementation to scale until actual available
 * resources are exhausted.
 *
 *
 * ============================================================================
 * SAFETY CONTRACT
 * ============================================================================
 *
 * This grammar contains no actions.
 *
 * Therefore it cannot:
 *
 *   - dereference pointers
 *   - perform unsafe Rust
 *   - access files
 *   - open sockets
 *   - execute processes
 *   - access hardware
 *   - allocate runtime buffers
 *   - decode attacker-controlled payloads
 *
 * All decoding happens after parsing in a separately controlled subsystem.
 *
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * For a fixed token stream and grammar version, parsing must produce the same
 * parse structure independent of:
 *
 *   - hardware
 *   - machine size
 *   - number of CPUs
 *   - memory capacity
 *   - accelerator availability
 *   - runtime provider
 *   - execution location
 *
 *
 * ============================================================================
 * VERSIONING CONTRACT
 * ============================================================================
 *
 * Format version, schema version, dialect version, and Zamani language
 * version are distinct concepts.
 *
 * This grammar MUST NOT conflate them.
 *
 * ============================================================================
 */
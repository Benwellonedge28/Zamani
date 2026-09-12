/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/core/metadata.g4
 *
 * Purpose:
 *     Canonical parser-level grammar for structured, source-level metadata
 *     values used throughout Zamani.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust integration baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no embedded Rust actions and requires no `unsafe`.
 *     Generated compiler/runtime implementation MUST remain safe Rust.
 *
 * ============================================================================
 * ARCHITECTURAL ROLE
 * ============================================================================
 *
 * Metadata is information ABOUT a source construct.
 *
 * Examples include:
 *
 *     documentation metadata
 *     provenance metadata
 *     compilation metadata
 *     language/dialect metadata
 *     capability metadata
 *     resource metadata
 *     portability metadata
 *     reproducibility metadata
 *     verification metadata
 *     security metadata
 *     quantum metadata
 *     hardware intent metadata
 *     interoperability metadata
 *     tooling metadata
 *
 * This file owns the STRUCTURE OF METADATA VALUES.
 *
 * It does NOT own the syntax used to ATTACH metadata to a declaration.
 *
 * For example, if the language uses:
 *
 *     #[...]
 *
 * or:
 *
 *     @name(...)
 *
 * the marker/attachment syntax belongs to the attribute/annotation grammar.
 *
 * This file provides the reusable structured payload consumed by those
 * grammars.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - metadata value structure;
 *     - metadata key structure;
 *     - metadata entry structure;
 *     - metadata maps/objects;
 *     - metadata sequences/arrays;
 *     - metadata tuples;
 *     - metadata references;
 *     - metadata paths;
 *     - metadata tagged values;
 *     - metadata lists;
 *     - metadata optional values;
 *     - metadata null values;
 *     - metadata literal composition;
 *     - metadata nesting.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - @ lexical syntax;
 *     - # lexical syntax;
 *     - attribute attachment;
 *     - annotation attachment;
 *     - identifiers;
 *     - lexical literal definitions;
 *     - qualified-name lexical structure;
 *     - filesystem paths;
 *     - URLs;
 *     - hardware addresses;
 *     - physical device identifiers;
 *     - quantum IR;
 *     - classical IR;
 *     - HDL IR;
 *     - QEC;
 *     - ZQN;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - resilience;
 *     - hardware discovery;
 *     - calibration;
 *     - runtime execution;
 *     - backend selection;
 *     - target selection;
 *     - resource allocation.
 *
 * ============================================================================
 * CORE PRINCIPLE
 * ============================================================================
 *
 * Metadata describes information, intent, declarations, constraints, hints,
 * provenance, or other structured source-level information.
 *
 * Metadata MUST NOT silently become an execution command.
 *
 * For example:
 *
 *     target = "gpu"
 *
 * may be metadata.
 *
 * It does not mean that the grammar selects a GPU.
 *
 * Likewise:
 *
 *     qubits = 1000
 *
 * is syntactically a metadata value.
 *
 * It does NOT impose that a machine contain 1000 qubits, nor does this grammar
 * validate whether such a resource exists.
 *
 * Resource/capability semantics belong downstream.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Metadata syntax is machine-independent.
 *
 * It must not encode assumptions such as:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_DEVICES
 *
 * No finite language-level limit is imposed on:
 *
 *     - metadata entries;
 *     - metadata nesting;
 *     - metadata keys;
 *     - metadata values;
 *     - metadata array elements;
 *     - metadata object fields;
 *     - qualified-name depth.
 *
 * Practical limits are implementation/resource concerns.
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 *     canonical lexer
 *           |
 *           v
 *     names.g4
 *           |
 *           +--------------------+
 *           |                    |
 *           v                    v
 *     metadata.g4          attributes.g4
 *           |                    |
 *           +---------+----------+
 *                     |
 *                     v
 *              declarations/types/
 *              modules/functions/
 *              quantum/hardware/
 *              resources/etc.
 *                     |
 *                     v
 *                  AST
 *                     |
 *                     v
 *              semantic analysis
 *                     |
 *                     +--> capabilities
 *                     +--> resources
 *                     +--> effects
 *                     +--> provenance
 *                     +--> compiler metadata
 *                     |
 *                     v
 *              canonical semantic IR
 *                     |
 *                     +--> quantum::ir
 *                     +--> classical IR
 *                     +--> HDL/hardware representations
 *                     |
 *                     v
 *           optimization / routing /
 *           scheduling / resilience /
 *           ZQN / QEC / lowering
 *
 * Metadata grammar MUST NOT depend on any downstream execution subsystem.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser must preserve:
 *
 *     - metadata key spelling;
 *     - qualified-name segment ordering;
 *     - literal spelling;
 *     - object/list ordering;
 *     - duplicate keys;
 *     - source spans;
 *     - source ordering;
 *     - nesting structure.
 *
 * Duplicate-key rejection is NOT a parser responsibility.
 *
 * This is deliberate because different metadata schemas may legitimately
 * choose different duplicate-key policies:
 *
 *     reject
 *     first-wins
 *     last-wins
 *     merge
 *     accumulate
 *
 * Such policy belongs to semantic/schema validation.
 *
 * ============================================================================
 * SOURCE PRESERVATION
 * ============================================================================
 *
 * The grammar does not normalize:
 *
 *     strings;
 *     names;
 *     numeric spellings;
 *     case;
 *     ordering;
 *     duplicate entries.
 *
 * Canonicalization belongs to semantic/serialization infrastructure.
 *
 * This is required for deterministic diagnostics and source-preserving AST
 * construction.
 *
 * ============================================================================
 * ANTLR COMPOSITION
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It consumes the canonical Zamani lexer vocabulary:
 *
 *     tokenVocab = ZamaniLexer
 *
 * It intentionally does not define lexer tokens.
 *
 * Shared name rules such as:
 *
 *     identifier
 *     qualifiedName
 *
 * are supplied by the canonical core grammar composition.
 *
 * If the repository's parser assembly imports Names separately, the canonical
 * `identifier` and `qualifiedName` rules MUST be reused rather than redefined.
 *
 * ============================================================================
 */

parser grammar Metadata;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. METADATA VALUE
 * ============================================================================
 *
 * The root reusable metadata value.
 *
 * Metadata is deliberately richer than a simple string key/value pair.
 *
 * This allows the language to express structured information without creating
 * a new syntax for every future domain.
 * ============================================================================
 */

metadataValue
    : metadataLiteral
    | metadataReference
    | metadataArray
    | metadataObject
    | metadataTuple
    | metadataTaggedValue
    ;


/*
 * ============================================================================
 * 2. METADATA LITERAL
 * ============================================================================
 *
 * Literal lexical recognition belongs to the lexer.
 *
 * This rule merely composes canonical literal tokens into a metadata value.
 *
 * Domain-specific literal forms are accepted as opaque source-level values.
 *
 * Their semantic validity is determined later.
 * ============================================================================
 */

metadataLiteral
    : INTEGER_LITERAL
    | DECIMAL_LITERAL
    | STRING_LITERAL
    | CHARACTER_LITERAL
    | BOOLEAN_LITERAL
    | QUANTUM_LITERAL
    | HARDWARE_LITERAL
    | DURATION_LITERAL
    | SIZE_LITERAL
    | K_NULL
    | K_NIL
    ;


/*
 * ============================================================================
 * 3. METADATA REFERENCE
 * ============================================================================
 *
 * A metadata value may refer to a source-level name.
 *
 * Examples:
 *
 *     version
 *     build::version
 *     platform::capability
 *     quantum::policy
 *
 * This grammar does not resolve the reference.
 * ============================================================================
 */

metadataReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 4. METADATA KEY
 * ============================================================================
 *
 * Keys may be:
 *
 *     simple/qualified names
 *     string keys
 *
 * String keys support externally defined schemas without requiring every
 * schema key to become a Zamani keyword.
 * ============================================================================
 */

metadataKey
    : qualifiedName
    | STRING_LITERAL
    ;


/*
 * ============================================================================
 * 5. METADATA ENTRY
 * ============================================================================
 *
 * Canonical structural form:
 *
 *     key = value
 *
 * The assignment token is intentionally lexical/parser infrastructure owned
 * by the canonical lexer.
 *
 * Metadata does not interpret the key.
 * ============================================================================
 */

metadataEntry
    : metadataKey ASSIGN metadataValue
    ;


/*
 * ============================================================================
 * 6. METADATA ENTRY LIST
 * ============================================================================
 *
 * No fixed number of metadata entries is permitted or required.
 *
 * The trailing comma is accepted deliberately for source stability and
 * formatter friendliness.
 * ============================================================================
 */

metadataEntryList
    : metadataEntry
      (
          COMMA metadataEntry
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 7. METADATA OBJECT
 * ============================================================================
 *
 * Structured key/value metadata.
 *
 * Example:
 *
 *     {
 *         "language" = "Zamani",
 *         "version" = "1.0"
 *     }
 *
 * The grammar imposes no field-count limit.
 *
 * Duplicate keys are preserved for semantic validation.
 * ============================================================================
 */

metadataObject
    : LBRACE
      metadataEntryList?
      RBRACE
    ;


/*
 * ============================================================================
 * 8. METADATA ARRAY
 * ============================================================================
 *
 * Ordered metadata values.
 *
 * Example:
 *
 *     [
 *         "cpu",
 *         "gpu",
 *         "qpu"
 *     ]
 *
 * The number of elements is unbounded by language grammar.
 * ============================================================================
 */

metadataArray
    : LBRACKET
      metadataValueList?
      RBRACKET
    ;


/*
 * ============================================================================
 * 9. METADATA VALUE LIST
 * ============================================================================
 */

metadataValueList
    : metadataValue
      (
          COMMA metadataValue
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 10. METADATA TUPLE
 * ============================================================================
 *
 * Tuples differ from arrays by explicitly preserving tuple semantics at the
 * AST/semantic boundary.
 *
 * Example:
 *
 *     (value1, value2)
 *
 * A single-element tuple requires the trailing comma.
 * ============================================================================
 */

metadataTuple
    : LPAREN
      metadataTupleElements
      RPAREN
    ;


metadataTupleElements
    : metadataValue COMMA
    | metadataValue
      COMMA metadataValue
      (
          COMMA metadataValue
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 11. METADATA TAGGED VALUE
 * ============================================================================
 *
 * Tagged values permit extensible schemas without adding a new grammar rule
 * for every future metadata category.
 *
 * Examples:
 *
 *     resource(...)
 *     capability(...)
 *     provenance(...)
 *
 * The tag is a normal source-level name.
 *
 * The grammar does not assign domain semantics to it.
 * ============================================================================
 */

metadataTaggedValue
    : qualifiedName
      LPAREN
      metadataValueList?
      RPAREN
    ;


/*
 * ============================================================================
 * 12. OPTIONAL METADATA VALUE
 * ============================================================================
 *
 * Reusable nullable form for consumers that permit absent metadata values.
 * ============================================================================
 */

optionalMetadataValue
    : metadataValue?
    ;


/*
 * ============================================================================
 * 13. METADATA DOCUMENT
 * ============================================================================
 *
 * A metadata document is an ordered sequence of metadata entries.
 *
 * This is useful for:
 *
 *     source metadata;
 *     module metadata;
 *     compilation metadata;
 *     provenance;
 *     generated-artifact metadata;
 *     tooling metadata.
 *
 * It deliberately has no attachment marker.
 *
 * Attachment belongs to the consuming grammar.
 * ============================================================================
 */

metadataDocument
    : metadataEntryList?
    ;


/*
 * ============================================================================
 * 14. METADATA BLOCK
 * ============================================================================
 *
 * Explicit block form for consumers that want a named metadata section.
 *
 * The surrounding declaration determines what the metadata describes.
 * ============================================================================
 */

metadataBlock
    : LBRACE
      metadataEntryList?
      RBRACE
    ;


/*
 * ============================================================================
 * 15. METADATA PROPERTY
 * ============================================================================
 *
 * Alias-style reusable production for downstream grammar components.
 *
 * This keeps consumers from duplicating:
 *
 *     metadataKey ASSIGN metadataValue
 *
 * ============================================================================
 */

metadataProperty
    : metadataEntry
    ;


/*
 * ============================================================================
 * 16. METADATA PROPERTY LIST
 * ============================================================================
 */

metadataPropertyList
    : metadataEntryList
    ;


/*
 * ============================================================================
 * 17. METADATA PATH
 * ============================================================================
 *
 * A metadata path identifies a nested metadata field structurally.
 *
 * Example:
 *
 *     build::provenance::source
 *
 * This is a source-level name path, not a filesystem path.
 *
 * It MUST NOT be interpreted as:
 *
 *     ./foo
 *     /foo
 *     C:\foo
 *     https://...
 *
 * Those belong elsewhere.
 * ============================================================================
 */

metadataPath
    : qualifiedName
    ;


/*
 * ============================================================================
 * 18. METADATA ASSIGNMENT
 * ============================================================================
 *
 * Explicit named metadata assignment.
 *
 * Kept separate from metadataEntry to provide a stable semantic naming
 * boundary for downstream grammar composition.
 * ============================================================================
 */

metadataAssignment
    : metadataPath ASSIGN metadataValue
    ;


/*
 * ============================================================================
 * 19. METADATA ASSIGNMENT LIST
 * ============================================================================
 */

metadataAssignmentList
    : metadataAssignment
      (
          COMMA metadataAssignment
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 20. METADATA SEQUENCE
 * ============================================================================
 *
 * Generic ordered metadata sequence.
 *
 * This is intentionally represented using the canonical array structure.
 * ============================================================================
 */

metadataSequence
    : metadataArray
    ;


/*
 * ============================================================================
 * 21. METADATA MAP
 * ============================================================================
 *
 * Generic key/value metadata map.
 *
 * It is structurally equivalent to metadataObject but receives a distinct
 * parser-level rule so semantic consumers can explicitly state their intended
 * category.
 *
 * The parser does not enforce map uniqueness.
 * ============================================================================
 */

metadataMap
    : metadataObject
    ;


/*
 * ============================================================================
 * 22. METADATA SCALAR
 * ============================================================================
 *
 * Convenience rule for consumers that accept only scalar metadata.
 * ============================================================================
 */

metadataScalar
    : metadataLiteral
    | metadataReference
    ;


/*
 * ============================================================================
 * 23. METADATA SCALAR LIST
 * ============================================================================
 */

metadataScalarList
    : metadataScalar
      (
          COMMA metadataScalar
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 24. METADATA OBJECT ENTRY
 * ============================================================================
 *
 * Explicit object-entry alias for consumers that need to communicate their
 * semantic intent without duplicating syntax.
 * ============================================================================
 */

metadataObjectEntry
    : metadataEntry
    ;


/*
 * ============================================================================
 * 25. METADATA OBJECT ENTRY LIST
 * ============================================================================
 */

metadataObjectEntryList
    : metadataObjectEntry
      (
          COMMA metadataObjectEntry
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 26. METADATA VALUE OR REFERENCE
 * ============================================================================
 *
 * Useful for schema systems that permit either a literal/structured value or
 * a symbolic reference.
 * ============================================================================
 */

metadataValueOrReference
    : metadataValue
    | metadataReference
    ;


/*
 * ============================================================================
 * 27. METADATA NAME VALUE
 * ============================================================================
 *
 * Explicit named metadata property.
 * ============================================================================
 */

metadataNameValue
    : metadataKey ASSIGN metadataValueOrReference
    ;


/*
 * ============================================================================
 * 28. METADATA NAME VALUE LIST
 * ============================================================================
 */

metadataNameValueList
    : metadataNameValue
      (
          COMMA metadataNameValue
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 29. METADATA REFERENCE PATH
 * ============================================================================
 *
 * Kept separate from metadataPath for downstream semantic readability.
 * ============================================================================
 */

metadataReferencePath
    : qualifiedName
    ;


/*
 * ============================================================================
 * 30. METADATA TAG
 * ============================================================================
 *
 * A standalone metadata tag.
 *
 * Example:
 *
 *     provenance
 *
 * Meaning belongs to the consumer/schema.
 * ============================================================================
 */

metadataTag
    : qualifiedName
    ;


/*
 * ============================================================================
 * 31. METADATA TAGGED SCALAR
 * ============================================================================
 */

metadataTaggedScalar
    : metadataTag
      LPAREN
      metadataScalarList?
      RPAREN
    ;


/*
 * ============================================================================
 * 32. METADATA TAGGED OBJECT
 * ============================================================================
 */

metadataTaggedObject
    : metadataTag
      metadataObject
    ;


/*
 * ============================================================================
 * 33. METADATA TAGGED ARRAY
 * ============================================================================
 */

metadataTaggedArray
    : metadataTag
      metadataArray
    ;


/*
 * ============================================================================
 * 34. METADATA TAGGED STRUCTURE
 * ============================================================================
 *
 * A stable umbrella production for semantic consumers that need to accept
 * tagged metadata without duplicating the grammar.
 * ============================================================================
 */

metadataTaggedStructure
    : metadataTaggedScalar
    | metadataTaggedObject
    | metadataTaggedArray
    ;


/*
 * ============================================================================
 * 35. METADATA ENTRY OR TAGGED STRUCTURE
 * ============================================================================
 */

metadataMember
    : metadataEntry
    | metadataTaggedStructure
    ;


/*
 * ============================================================================
 * 36. METADATA MEMBER LIST
 * ============================================================================
 */

metadataMemberList
    : metadataMember
      (
          COMMA metadataMember
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 37. METADATA STRUCTURE
 * ============================================================================
 *
 * General structured metadata root.
 *
 * This is intentionally broad but remains finite and deterministic.
 * ============================================================================
 */

metadataStructure
    : metadataObject
    | metadataArray
    | metadataTuple
    | metadataTaggedStructure
    ;


/*
 * ============================================================================
 * 38. METADATA ROOT
 * ============================================================================
 *
 * Canonical reusable metadata production.
 *
 * Consumers should prefer this rule unless they intentionally need a narrower
 * contract such as metadataScalar or metadataObject.
 * ============================================================================
 */

metadata
    : metadataValue
    ;


/*
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * The following decisions MUST NOT be made by this grammar:
 *
 *     - whether a key is recognized;
 *     - whether a key is deprecated;
 *     - whether a key is duplicated;
 *     - whether a value has the expected type;
 *     - whether a value is within a valid range;
 *     - whether a resource exists;
 *     - whether a capability exists;
 *     - whether a target exists;
 *     - whether a device exists;
 *     - whether a QPU supports an operation;
 *     - whether a hardware resource is available;
 *     - whether a metadata request is satisfiable;
 *     - whether metadata changes compilation;
 *     - whether metadata changes optimization;
 *     - whether metadata changes scheduling;
 *     - whether metadata changes routing;
 *     - whether metadata is propagated into IR.
 *
 * Those decisions belong to semantic analysis and the owning subsystem.
 *
 * ============================================================================
 * IR INTEGRATION
 * ============================================================================
 *
 * Metadata may eventually be lowered into:
 *
 *     frontend AST metadata
 *         |
 *         v
 *     semantic metadata model
 *         |
 *         +--> canonical semantic IR
 *         |
 *         +--> quantum::ir metadata where applicable
 *         +--> classical IR metadata
 *         +--> hardware/HDL metadata
 *         +--> compilation provenance
 *
 * The grammar MUST NOT construct any IR.
 *
 * In particular:
 *
 *     grammar -> quantum::ir
 *
 * is forbidden.
 *
 * The correct direction is:
 *
 *     grammar
 *       -> AST
 *       -> semantic metadata
 *       -> canonical IR
 *       -> domain IR
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum metadata can express source-level information such as:
 *
 *     logical
 *     error_correction
 *     observable
 *     capability
 *     resource
 *     provenance
 *     measurement
 *
 * without this grammar defining the semantics of those concepts.
 *
 * No metadata production contains:
 *
 *     qubit_count
 *     physical_qubit_count
 *     topology_count
 *     gate-set count
 *     device count
 *     backend count
 *
 * as fixed language restrictions.
 *
 * Values containing such names are ordinary metadata until semantic analysis
 * gives them meaning.
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * ============================================================================
 * HARDWARE / HDL INTEGRATION
 * ============================================================================
 *
 * Hardware metadata may describe:
 *
 *     capabilities
 *     interfaces
 *     timing intent
 *     resource requirements
 *     implementation hints
 *     portability requirements
 *
 * without selecting a physical device.
 *
 * Physical mapping belongs to:
 *
 *     hardware abstraction
 *     target selection
 *     placement
 *     routing
 *     scheduling
 *     runtime
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Metadata may carry structured descriptions of:
 *
 *     requirement
 *     constraint
 *     preference
 *     hint
 *     capability
 *     resource
 *
 * These concepts MUST remain semantically distinct.
 *
 * The grammar only represents their structure.
 *
 * ============================================================================
 * PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Metadata is suitable for source-level provenance such as:
 *
 *     source identity
 *     language version
 *     dialect identity
 *     generator identity
 *     transformation information
 *     reproducibility information
 *
 * The grammar does not generate timestamps, hashes, UUIDs, or machine state.
 *
 * Such values are supplied by the appropriate semantic/toolchain layer.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no semantic predicates;
 *     - no embedded actions;
 *     - no filesystem access;
 *     - no network access;
 *     - no runtime discovery;
 *     - no target discovery;
 *     - no random behavior;
 *     - no hardware-dependent branches.
 *
 * Given the same token stream, parsing is deterministic.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are intentionally no grammar-level constants such as:
 *
 *     MAX_METADATA_ENTRIES
 *     MAX_METADATA_DEPTH
 *     MAX_METADATA_KEYS
 *     MAX_METADATA_ARRAY_ELEMENTS
 *     MAX_METADATA_VALUE_SIZE
 *     MAX_METADATA_OBJECT_FIELDS
 *
 * Repetition uses ANTLR repetition operators.
 *
 * Actual implementation limits may be imposed by:
 *
 *     memory availability
 *     compiler resource policy
 *     parser resource policy
 *     operating-system limits
 *
 * Such limits must remain configurable and must not become language semantics.
 *
 * ============================================================================
 * ERROR HANDLING
 * ============================================================================
 *
 * Parser diagnostics should distinguish:
 *
 *     missing metadata key
 *     missing assignment
 *     missing metadata value
 *     malformed object
 *     malformed array
 *     malformed tuple
 *     malformed tagged value
 *     unexpected trailing comma where the surrounding construct forbids it
 *
 * Semantic diagnostics should separately handle:
 *
 *     unknown metadata key
 *     invalid metadata schema
 *     duplicate metadata key
 *     incompatible value type
 *     unsupported metadata feature
 *     invalid resource/capability request
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * This grammar deliberately uses canonical token names from ZamaniLexer rather
 * than inventing metadata-specific lexical tokens.
 *
 * Existing lexer ownership remains authoritative.
 *
 * Existing annotation/attribute syntax can consume this metadata model without
 * changing the lexical architecture.
 *
 * The older `grammar/antlr/Meta.g4` remains a separate metaprogramming grammar
 * and must not become the owner of generic metadata values.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * The metadata test suite must cover:
 *
 * POSITIVE:
 *
 *     key = 1
 *     key = "value"
 *     key = true
 *     key = null
 *     key = namespace::value
 *     key = [1, 2, 3]
 *     key = {a = 1, b = 2}
 *     key = (1, 2)
 *     key = capability(cpu)
 *     key = capability("quantum")
 *     key = resource({kind = "quantum"})
 *
 * CROSS-DOMAIN:
 *
 *     classical metadata
 *     quantum metadata
 *     hybrid metadata
 *     HDL metadata
 *     hardware metadata
 *     distributed metadata
 *     AI/data metadata
 *
 * SCALABILITY:
 *
 *     many entries
 *     many nested structures
 *     many qualified-name segments
 *     many array elements
 *
 * NEGATIVE:
 *
 *     = value
 *     key =
 *     key = {
 *     key = [
 *     key = capability(
 *     { key = 1
 *     [1, 2
 *
 * SOURCE-PRESERVATION:
 *
 *     preserve key spelling
 *     preserve qualified-name order
 *     preserve literal spelling
 *     preserve list/object order
 *     preserve duplicate entries
 *     preserve source spans
 *
 * DETERMINISM:
 *
 *     identical source -> identical token sequence
 *     identical token sequence -> identical parse structure
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * metadata.g4 is complete when:
 *
 *     1. It compiles as an ANTLR parser grammar against the canonical
 *        ZamaniLexer vocabulary.
 *
 *     2. It introduces no lexer rules.
 *
 *     3. It introduces no Rust actions.
 *
 *     4. It requires no unsafe Rust.
 *
 *     5. It owns structured metadata values without owning annotation
 *        attachment syntax.
 *
 *     6. It reuses canonical identifier/qualified-name rules in the final
 *        parser composition.
 *
 *     7. It introduces no machine-size or hardware-size limits.
 *
 *     8. It creates no quantum IR.
 *
 *     9. It creates no classical IR.
 *
 *    10. It does not select a hardware backend.
 *
 *    11. It preserves source structure required by the AST.
 *
 *    12. It supports arbitrarily nested metadata subject only to actual
 *        implementation resources.
 *
 *    13. It has positive, negative, boundary, cross-domain, determinism,
 *        and round-trip tests.
 *
 *    14. Its integration contract is stable enough that downstream grammar
 *        files do not need to redefine metadata syntax.
 *
 * ============================================================================
 */
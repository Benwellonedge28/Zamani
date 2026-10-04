/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/core/metadata.g4
 *
 * GRAMMAR
 * -------
 * Metadata
 *
 * STATUS
 * ------
 * CANONICAL / PRODUCTION
 *
 * LANGUAGE
 * --------
 * Zamani
 *
 * BASELINE
 * --------
 * ANTLR4 parser grammar
 * Rust 1.97 / Rust 1.97.1
 * Rust 2021
 *
 * SAFETY
 * ------
 * This grammar contains no embedded target-language actions, semantic
 * predicates, filesystem access, network access, runtime callbacks, or unsafe
 * implementation requirements.
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file is the canonical parser-level owner of STRUCTURED SOURCE-LEVEL
 * METADATA VALUES.
 *
 * Metadata is information associated with another source construct or
 * represented as a structured source-level value.
 *
 * Examples include:
 *
 *     documentation information
 *     provenance information
 *     compilation information
 *     dialect information
 *     capability information
 *     resource information
 *     portability information
 *     reproducibility information
 *     verification information
 *     security information
 *     quantum information
 *     hardware-intent information
 *     interoperability information
 *     tooling information
 *     AI/model information
 *     data/schema information
 *
 * This grammar defines the STRUCTURE of metadata.
 *
 * It does not define what a metadata key means.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - metadata values;
 *     - metadata literals;
 *     - metadata symbolic references;
 *     - metadata keys;
 *     - metadata entries;
 *     - metadata objects;
 *     - metadata arrays;
 *     - metadata tuples;
 *     - metadata tagged values;
 *     - reusable metadata value lists;
 *     - reusable metadata entry lists.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical token definitions;
 *     - identifier lexical recognition;
 *     - Unicode identifier rules;
 *     - keyword recognition;
 *     - comments;
 *     - whitespace;
 *     - operators;
 *     - punctuation token definitions;
 *     - attribute attachment syntax;
 *     - annotation attachment syntax;
 *     - pragma attachment syntax;
 *     - filesystem paths;
 *     - URLs;
 *     - module resolution;
 *     - namespace resolution;
 *     - symbol resolution;
 *     - type checking;
 *     - effect checking;
 *     - capability resolution;
 *     - resource resolution;
 *     - contract validation;
 *     - policy validation;
 *     - provenance generation;
 *     - runtime execution;
 *     - target selection;
 *     - hardware discovery;
 *     - hardware allocation;
 *     - quantum physical mapping;
 *     - quantum routing;
 *     - scheduling;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - backend selection;
 *     - classical IR;
 *     - quantum::ir;
 *     - HDL IR;
 *     - runtime IR.
 *
 * ============================================================================
 * CRITICAL OWNERSHIP RULE
 * ============================================================================
 *
 * Attribute attachment remains owned by:
 *
 *     grammar/core/attributes.g4
 *
 * Annotation attachment remains owned by:
 *
 *     grammar/core/annotations.g4
 *
 * Metadata values are reusable payloads.
 *
 * A consuming grammar decides where a metadata value is legal.
 *
 * This file must therefore NOT define:
 *
 *     @
 *     #
 *     #[...]
 *     @name(...)
 *
 * or any equivalent attachment syntax.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/core/names.g4
 *
 * `Names` supplies:
 *
 *     identifier
 *     simpleName
 *     nameSegment
 *     qualifiedName
 *
 * This grammar intentionally imports `Names` so the file is independently
 * composable and does not require downstream grammars to provide hidden
 * parser-rule dependencies.
 *
 * EXPORTS:
 *
 *     metadata
 *     metadataValue
 *     metadataLiteral
 *     metadataReference
 *     metadataKey
 *     metadataEntry
 *     metadataEntryList
 *     metadataObject
 *     metadataArray
 *     metadataValueList
 *     metadataTuple
 *     metadataTupleElements
 *     metadataTaggedValue
 *
 * CONSUMED_BY:
 *
 *     grammar/core/attributes.g4
 *     grammar/core/annotations.g4
 *     grammar/core/pragmas.g4
 *     grammar/core/hints.g4
 *     grammar/declarations/
 *     grammar/types/
 *     grammar/functions/
 *     grammar/modules/
 *     grammar/effects/
 *     grammar/resources/
 *     grammar/validation/
 *     grammar/security/
 *     grammar/compile/
 *     grammar/execution/
 *     grammar/classical/
 *     grammar/quantum/
 *     grammar/hybrid/
 *     grammar/hdl/
 *     grammar/hardware/
 *     grammar/distributed/
 *     grammar/ai/
 *     grammar/data/
 *     grammar/networking/
 *     grammar/interoperability/
 *     grammar/dialects/
 *     grammar/macros/
 *     grammar/metaprogramming/
 *
 * AST_OWNER:
 *
 *     canonical domain-neutral frontend AST
 *
 * SEMANTIC_OWNER:
 *
 *     canonical semantic-analysis / metadata-schema subsystem
 *
 * TYPE_OWNER:
 *
 *     canonical type-analysis subsystem
 *
 * EFFECT_OWNER:
 *
 *     canonical effect-analysis subsystem
 *
 * CAPABILITY_OWNER:
 *
 *     canonical capability-analysis subsystem
 *
 * RESOURCE_OWNER:
 *
 *     canonical resource-analysis subsystem
 *
 * POLICY_OWNER:
 *
 *     canonical policy-analysis subsystem
 *
 * PROVENANCE_OWNER:
 *
 *     canonical provenance subsystem
 *
 * IR_OWNER:
 *
 *     downstream semantic/domain IR owners
 *
 * SPEC_OWNER:
 *
 *     grammar/specification/
 *     grammar/spec/
 *
 * TEST_OWNER:
 *
 *     grammar/tests/core/metadata/
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It MUST use:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * It MUST NOT define lexical rules.
 *
 * The canonical lexer owns all token spelling and classification.
 *
 * ============================================================================
 * CANONICAL LITERAL TOKENS
 * ============================================================================
 *
 * The current repository literal architecture provides the following parser
 * visible literal tokens:
 *
 *     INTEGER
 *     FLOAT
 *     STRING_LITERAL
 *     CHARACTER_LITERAL
 *     TRUE
 *     FALSE
 *     QUANTUM_LITERAL
 *     HARDWARE_LITERAL
 *     DURATION_LITERAL
 *     SIZE_LITERAL
 *
 * This file consumes those tokens.
 *
 * It does NOT invent alternate names such as:
 *
 *     INTEGER_LITERAL
 *     DECIMAL_LITERAL
 *     BOOLEAN_LITERAL
 *     K_NULL
 *     K_NIL
 *
 * where those are not the current canonical lexer vocabulary.
 *
 * If a future literal category becomes canonical, its lexer owner and parser
 * integration must be changed as one coordinated language evolution.
 *
 * ============================================================================
 * NAME CONTRACT
 * ============================================================================
 *
 * Metadata keys and symbolic references use the canonical name system.
 *
 * Examples:
 *
 *     version
 *     build::version
 *     quantum::policy
 *     hardware::capability
 *     provenance::source
 *
 * Name resolution does not occur here.
 *
 * The grammar only preserves the source structure.
 *
 * ============================================================================
 * METADATA MODEL
 * ============================================================================
 *
 * The canonical structural model is:
 *
 *     metadata
 *         |
 *         v
 *     metadataValue
 *         |
 *         +----------------+----------------+----------------+
 *         |                |                |                |
 *         v                v                v                v
 *      literal         reference          array            object
 *                                          |
 *                                          +--> tuple
 *                                          |
 *                                          +--> tagged value
 *
 * The parser preserves structure.
 *
 * Semantic analysis determines meaning.
 *
 * ============================================================================
 * SOURCE PRESERVATION CONTRACT
 * ============================================================================
 *
 * The resulting AST must preserve enough information to retain:
 *
 *     - metadata key spelling;
 *     - qualified-name segment order;
 *     - literal token text;
 *     - object-entry order;
 *     - array-element order;
 *     - tuple-element order;
 *     - duplicate object keys;
 *     - tagged-value name;
 *     - nested structure;
 *     - source spans.
 *
 * Duplicate keys MUST NOT be rejected by this grammar.
 *
 * Different metadata schemas may require:
 *
 *     reject
 *     first-wins
 *     last-wins
 *     merge
 *     accumulate
 *
 * Therefore duplicate-key policy belongs to semantic/schema validation.
 *
 * ============================================================================
 * SEMANTIC SEPARATION
 * ============================================================================
 *
 * Parsing metadata does NOT mean:
 *
 *     - the key is recognized;
 *     - the value is semantically valid;
 *     - a capability exists;
 *     - a resource exists;
 *     - a target exists;
 *     - a device exists;
 *     - a QPU supports a requested operation;
 *     - a policy is authorized;
 *     - a contract is satisfied;
 *     - an effect is permitted;
 *     - provenance is trusted;
 *     - a compilation option is accepted.
 *
 * Those decisions belong downstream.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Metadata syntax must remain independent of machine scale.
 *
 * There are NO grammar-level limits on:
 *
 *     metadata entries
 *     metadata values
 *     metadata keys
 *     array elements
 *     object fields
 *     tuple elements
 *     nesting depth
 *     qualified-name depth
 *     source metadata quantity
 *
 * The grammar MUST NOT define:
 *
 *     MAX_METADATA_ENTRIES
 *     MAX_METADATA_DEPTH
 *     MAX_METADATA_KEYS
 *     MAX_METADATA_ARRAY_ELEMENTS
 *     MAX_METADATA_OBJECT_FIELDS
 *     MAX_METADATA_VALUE_SIZE
 *
 * It also MUST NOT encode:
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
 * Practical parser limits, when required for denial-of-service protection,
 * belong to explicitly configurable compiler/parser resource policy and must
 * not become language semantics.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing metadata is deterministic.
 *
 * The parse result must depend only on:
 *
 *     source token stream
 *     grammar version
 *     parser configuration
 *
 * It must not depend on:
 *
 *     current time
 *     randomness
 *     filesystem state
 *     network state
 *     environment variables
 *     hardware availability
 *     runtime state
 *     target selection
 *     resource discovery
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Metadata is untrusted source input until validated.
 *
 * Parsing metadata MUST NOT grant:
 *
 *     permissions
 *     capabilities
 *     hardware access
 *     filesystem access
 *     network access
 *     native execution
 *     FFI access
 *     privileged execution
 *
 * A metadata value such as:
 *
 *     capability = hardware::accelerator
 *
 * is only syntactic data at this stage.
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * Metadata may describe quantum-related intent:
 *
 *     quantum::measurement
 *     quantum::resource
 *     quantum::logical
 *     quantum::error_correction
 *     quantum::provenance
 *
 * but this grammar does not implement quantum semantics.
 *
 * The canonical quantum pipeline remains:
 *
 *     source
 *       ->
 *     domain-neutral AST
 *       ->
 *     semantic analysis
 *       ->
 *     quantum::ir
 *       ->
 *     optimization
 *       ->
 *     decomposition
 *       ->
 *     routing
 *       ->
 *     scheduling
 *       ->
 *     resilience / QEC
 *       ->
 *     ZQN
 *       ->
 *     HAL
 *       ->
 *     target realization
 *
 * Metadata grammar MUST NOT:
 *
 *     - allocate qubits;
 *     - identify physical qubits;
 *     - select a QPU;
 *     - define a gate set;
 *     - define topology;
 *     - define calibration;
 *     - define routing;
 *     - define scheduling;
 *     - create quantum::ir.
 *
 * ============================================================================
 * HDL / HARDWARE CONTRACT
 * ============================================================================
 *
 * Metadata may describe:
 *
 *     hardware intent
 *     interfaces
 *     implementation hints
 *     resource requirements
 *     timing intent
 *     portability requirements
 *     capabilities
 *
 * It does not select physical hardware.
 *
 * Metadata such as:
 *
 *     resource = memory
 *
 * is source-level information only.
 *
 * Hardware realization is downstream.
 *
 * ============================================================================
 * AI / KNOWLEDGE CONTRACT
 * ============================================================================
 *
 * Metadata may describe:
 *
 *     models
 *     datasets
 *     reasoning systems
 *     learning systems
 *     adaptation policies
 *     uncertainty
 *     evidence
 *     explanations
 *     agents
 *     decisions
 *     provenance
 *
 * These remain generic metadata values.
 *
 * The metadata grammar does not create separate application-specific
 * languages for individual AI techniques.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY CONTRACT
 * ============================================================================
 *
 * Metadata may carry structures used later by:
 *
 *     requirements
 *     constraints
 *     capabilities
 *     preferences
 *     hints
 *     budgets
 *     policies
 *
 * This grammar does not decide whether a requirement is satisfiable.
 *
 * For example:
 *
 *     {
 *         capability = quantum::measurement,
 *         resource = memory
 *     }
 *
 * remains structured source metadata until semantic analysis interprets it.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Metadata may represent provenance information such as:
 *
 *     source
 *     derivation
 *     generated
 *     transformed
 *     verified
 *     decision
 *     evidence
 *
 * The grammar does not manufacture:
 *
 *     timestamps
 *     hashes
 *     UUIDs
 *     signatures
 *     machine identity
 *
 * Those values are supplied by the relevant semantic/toolchain subsystem.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This file intentionally avoids obsolete duplicate lexical token names.
 *
 * The stable parser-facing literal contract is:
 *
 *     INTEGER
 *     FLOAT
 *     STRING_LITERAL
 *     CHARACTER_LITERAL
 *     TRUE
 *     FALSE
 *     QUANTUM_LITERAL
 *     HARDWARE_LITERAL
 *     DURATION_LITERAL
 *     SIZE_LITERAL
 *
 * Name syntax is inherited from `Names`.
 *
 * Changing any of these contracts requires coordinated updates to:
 *
 *     canonical lexer
 *     parser composition
 *     AST conversion
 *     semantic validation
 *     grammar/spec/
 *     conformance tests
 *     compatibility tests
 *
 * This file must not silently create lexical aliases.
 *
 * ============================================================================
 * ANTLR COMPOSITION
 * ============================================================================
 *
 * Metadata is a leaf parser grammar.
 *
 * The grammar imports `Names` because it directly consumes `qualifiedName`.
 *
 * This produces:
 *
 *     ZamaniLexer
 *          |
 *          v
 *       Names
 *          |
 *          v
 *      Metadata
 *          |
 *          v
 *     consuming grammar
 *
 * Higher-level grammar composition remains responsible for attaching metadata
 * to declarations, expressions, statements, modules, resources, quantum
 * constructs, HDL constructs, and other source constructs.
 *
 * ============================================================================
 * RULE DESIGN PRINCIPLE
 * ============================================================================
 *
 * Every public rule in this file represents a genuinely useful structural
 * distinction.
 *
 * Redundant aliases are deliberately avoided.
 *
 * In particular, this file does NOT create separate parser rules merely for
 * semantic labels such as:
 *
 *     metadataMap
 *     metadataSequence
 *     metadataProperty
 *     metadataAssignment
 *     metadataStructure
 *     metadataScalarList
 *     metadataObjectEntry
 *     metadataReferencePath
 *
 * unless those forms acquire distinct language semantics.
 *
 * The AST/semantic layer can assign semantic names to the same structural
 * production without multiplying parser authorities.
 *
 * ============================================================================
 * 1. CANONICAL METADATA ROOT
 * ============================================================================
 *
 * `metadata` is the stable general-purpose entry point.
 *
 * Consumers should use it when they accept any metadata value.
 * ============================================================================
 */

parser grammar Metadata;

options {
    tokenVocab = ZamaniLexer;
}

import Names;


/*
 * ============================================================================
 * 2. METADATA
 * ============================================================================
 */

metadata
    : metadataValue
    ;


/*
 * ============================================================================
 * 3. METADATA VALUE
 * ============================================================================
 *
 * A metadata value may be:
 *
 *     literal
 *     symbolic reference
 *     array
 *     object
 *     tuple
 *     tagged value
 *
 * No arbitrary runtime expression is accepted.
 *
 * This is deliberate.
 *
 * Metadata must not accidentally become a second expression language.
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
 * 4. METADATA LITERAL
 * ============================================================================
 */

metadataLiteral
    : INTEGER
    | FLOAT
    | STRING_LITERAL
    | CHARACTER_LITERAL
    | TRUE
    | FALSE
    | QUANTUM_LITERAL
    | HARDWARE_LITERAL
    | DURATION_LITERAL
    | SIZE_LITERAL
    ;


/*
 * ============================================================================
 * 5. METADATA REFERENCE
 * ============================================================================
 *
 * A metadata reference is a source-level qualified name.
 *
 * Examples:
 *
 *     version
 *     build::version
 *     quantum::policy
 *     hardware::capability
 *
 * Resolution is downstream.
 * ============================================================================
 */

metadataReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 6. METADATA KEY
 * ============================================================================
 *
 * Keys may be:
 *
 *     qualified names
 *     string keys
 *
 * String keys allow externally defined schemas without turning every schema
 * key into a language keyword.
 * ============================================================================
 */

metadataKey
    : qualifiedName
    | STRING_LITERAL
    ;


/*
 * ============================================================================
 * 7. METADATA ENTRY
 * ============================================================================
 *
 * Canonical key/value syntax:
 *
 *     key = value
 *
 * The key is deliberately not interpreted here.
 * ============================================================================
 */

metadataEntry
    : metadataKey ASSIGN metadataValue
    ;


/*
 * ============================================================================
 * 8. METADATA ENTRY LIST
 * ============================================================================
 *
 * Non-empty ordered metadata entries.
 *
 * Duplicate keys remain structurally representable.
 *
 * A trailing comma is accepted.
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
 * 9. METADATA OBJECT
 * ============================================================================
 *
 * Structured key/value metadata.
 *
 * Examples:
 *
 *     {}
 *
 *     {
 *         language = "Zamani"
 *     }
 *
 *     {
 *         language = "Zamani",
 *         version = "1"
 *     }
 *
 *     {
 *         "language" = "Zamani"
 *     }
 *
 * Empty objects are valid.
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
 * 10. METADATA ARRAY
 * ============================================================================
 *
 * Ordered metadata values.
 *
 * Examples:
 *
 *     []
 *
 *     [cpu, gpu, accelerator]
 *
 *     [1, 2, 3]
 *
 *     [quantum::measurement, hardware::capability]
 *
 * Empty arrays are valid.
 *
 * A trailing comma is accepted.
 * ============================================================================
 */

metadataArray
    : LBRACKET
      metadataValueList?
      RBRACKET
    ;


/*
 * ============================================================================
 * 11. METADATA VALUE LIST
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
 * 12. METADATA TUPLE
 * ============================================================================
 *
 * Tuples preserve positional structure distinct from arrays.
 *
 * Examples:
 *
 *     ()
 *     (value)
 *     (value,)
 *     (value1, value2)
 *     (value1, value2,)
 *
 * Semantic validation determines tuple arity/type meaning.
 *
 * No finite tuple-size limit is imposed by this grammar.
 * ============================================================================
 */

metadataTuple
    : LPAREN
      metadataTupleElements?
      RPAREN
    ;


/*
 * ============================================================================
 * 13. METADATA TUPLE ELEMENTS
 * ============================================================================
 *
 * The optional trailing comma is accepted.
 *
 * The AST must preserve whether the source contained a trailing comma when
 * source-preserving formatting or diagnostics require that information.
 * ============================================================================
 */

metadataTupleElements
    : metadataValue
      (
          COMMA metadataValue
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 14. METADATA TAGGED VALUE
 * ============================================================================
 *
 * Tagged values provide an extensible semantic envelope without requiring a
 * new parser production for every future metadata category.
 *
 * Examples:
 *
 *     capability(quantum::measurement)
 *
 *     resource(memory)
 *
 *     provenance(source::module)
 *
 *     evidence("measurement")
 *
 *     policy(execution::recovery)
 *
 *     confidence(0.95)
 *
 * The tag is a normal qualified name.
 *
 * The grammar assigns no semantic meaning to the tag.
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
 * STRUCTURAL INTEGRATION CONTRACT
 * ============================================================================
 *
 * The canonical structural forms are now:
 *
 *     metadata
 *         -> metadataValue
 *
 *     metadataValue
 *         -> metadataLiteral
 *         -> metadataReference
 *         -> metadataArray
 *         -> metadataObject
 *         -> metadataTuple
 *         -> metadataTaggedValue
 *
 *     metadataObject
 *         -> metadataEntryList?
 *
 *     metadataEntryList
 *         -> metadataEntry (, metadataEntry)* ,?
 *
 *     metadataArray
 *         -> metadataValueList?
 *
 *     metadataValueList
 *         -> metadataValue (, metadataValue)* ,?
 *
 *     metadataTuple
 *         -> metadataTupleElements?
 *
 *     metadataTupleElements
 *         -> metadataValue (, metadataValue)* ,?
 *
 *     metadataTaggedValue
 *         -> qualifiedName (metadataValueList?)
 *
 * This is the complete structural contract.
 *
 * Downstream grammars should reuse these productions rather than reproducing
 * equivalent key/value/list/map syntax.
 *
 * ============================================================================
 * ATTRIBUTE INTEGRATION
 * ============================================================================
 *
 * `grammar/core/attributes.g4` currently owns generic attribute values and
 * attribute maps.
 *
 * The intended final ownership is:
 *
 *     metadata.g4
 *         |
 *         +--> generic reusable metadata-value structure
 *         |
 *         v
 *     attributes.g4
 *         |
 *         +--> attribute attachment and attribute-specific argument structure
 *
 * The attribute grammar must not create a competing metadata authority.
 *
 * During the composition migration, attribute consumers should use the
 * canonical metadata value productions wherever the semantic contract says
 * that an attribute argument is metadata rather than an expression.
 *
 * Attribute attachment remains exclusively owned by attributes.g4.
 *
 * ============================================================================
 * ANNOTATION INTEGRATION
 * ============================================================================
 *
 * `grammar/core/annotations.g4` remains responsible for annotation syntax and
 * attachment policy.
 *
 * Annotation payloads may consume:
 *
 *     metadata
 *     metadataValue
 *     metadataObject
 *     metadataArray
 *
 * according to the annotation specification.
 *
 * This file must not add annotation markers.
 *
 * ============================================================================
 * PRAGMA INTEGRATION
 * ============================================================================
 *
 * `grammar/core/pragmas.g4` owns pragma syntax.
 *
 * Pragma payloads may use metadata values where specified.
 *
 * Metadata must remain a payload grammar and must not become a second pragma
 * grammar.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Resource and capability grammars may consume metadata to represent
 * extensible source descriptions.
 *
 * Example structural intent:
 *
 *     {
 *         capability = quantum::measurement,
 *         resource = memory
 *     }
 *
 * The metadata parser does not determine:
 *
 *     whether memory exists;
 *     how much memory exists;
 *     which device provides it;
 *     whether quantum measurement is available;
 *     whether a target satisfies the request.
 *
 * Those decisions belong to semantic/resource/capability analysis.
 *
 * ============================================================================
 * CONTRACT / POLICY INTEGRATION
 * ============================================================================
 *
 * Contract and policy systems may store structured metadata containing:
 *
 *     requirements
 *     assumptions
 *     guarantees
 *     constraints
 *     preferences
 *     prohibitions
 *     fallbacks
 *     evidence
 *     provenance
 *
 * The metadata grammar only represents their structural values.
 *
 * Contract and policy semantics remain outside this file.
 *
 * ============================================================================
 * AI / REASONING / LEARNING INTEGRATION
 * ============================================================================
 *
 * The same metadata structure may describe:
 *
 *     reasoning metadata
 *     knowledge metadata
 *     learning metadata
 *     adaptation metadata
 *     uncertainty metadata
 *     evidence metadata
 *     explanation metadata
 *     decision metadata
 *     agent metadata
 *     model metadata
 *     dataset metadata
 *     provenance metadata
 *
 * No application-specific keyword is necessary.
 *
 * New models, algorithms, domains, frameworks, or techniques should generally
 * be represented as ordinary names, qualified names, metadata tags, or
 * semantic registrations rather than new core grammar rules.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum metadata can use:
 *
 *     quantum::...
 *     capability(...)
 *     resource(...)
 *     provenance(...)
 *     evidence(...)
 *
 * without changing this grammar.
 *
 * New quantum operations do not require new metadata rules.
 *
 * The grammar remains independent of:
 *
 *     physical qubit count
 *     logical qubit count
 *     QPU count
 *     topology
 *     calibration
 *     routing
 *     scheduling
 *     error-correction implementation
 *     backend identity
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware and HDL grammars may attach metadata describing:
 *
 *     interfaces
 *     timing intent
 *     implementation intent
 *     capabilities
 *     resource requirements
 *     portability constraints
 *     verification information
 *
 * The metadata grammar does not define:
 *
 *     wire width
 *     register width
 *     device count
 *     memory capacity
 *     topology dimensions
 *     physical placement
 *     synthesis implementation.
 *
 * ============================================================================
 * DISTRIBUTED / NETWORK INTEGRATION
 * ============================================================================
 *
 * Metadata may describe:
 *
 *     nodes
 *     services
 *     endpoints
 *     channels
 *     topology intent
 *     consistency requirements
 *     provenance
 *     security policy
 *
 * The parser imposes no finite node, endpoint, channel, or topology limit.
 *
 * ============================================================================
 * INTEROPERABILITY INTEGRATION
 * ============================================================================
 *
 * Metadata may describe external language or data boundaries:
 *
 *     language
 *     dialect
 *     ABI
 *     calling convention
 *     schema
 *     format
 *     version
 *     provenance
 *
 * SQL, JSON, XML, C, C++, Rust, WASM, OpenQASM and other external formats
 * retain their own grammar or interoperability owners.
 *
 * Metadata does not become those formats.
 *
 * ============================================================================
 * METAPROGRAMMING INTEGRATION
 * ============================================================================
 *
 * Metadata may describe compile-time or reflective intent.
 *
 * It does not itself perform:
 *
 *     reflection
 *     code generation
 *     execution
 *     evaluation
 *     macro expansion
 *
 * Those operations remain owned by metaprogramming/compiler subsystems.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST should map these parser contexts into domain-neutral
 * metadata nodes.
 *
 * Conceptually:
 *
 *     Metadata
 *         value
 *         source_span
 *
 *     MetadataLiteral
 *         token_kind
 *         source_text
 *         source_span
 *
 *     MetadataReference
 *         qualified_name
 *         source_span
 *
 *     MetadataKey
 *         name | string
 *         source_span
 *
 *     MetadataEntry
 *         key
 *         value
 *         source_span
 *
 *     MetadataObject
 *         entries[]
 *         source_span
 *
 *     MetadataArray
 *         values[]
 *         source_span
 *
 *     MetadataTuple
 *         values[]
 *         source_span
 *
 *     MetadataTaggedValue
 *         tag
 *         values[]
 *         source_span
 *
 * The concrete Rust names may differ.
 *
 * The important invariants are:
 *
 *     - source order is retained;
 *     - duplicate entries are retained;
 *     - nested structure is retained;
 *     - literal source text is available;
 *     - qualified-name order is retained;
 *     - source spans are retained.
 *
 * This grammar creates no Rust AST types.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     - metadata schema selection;
 *     - metadata key resolution;
 *     - duplicate-key policy;
 *     - type validation;
 *     - reference resolution;
 *     - tag interpretation;
 *     - capability interpretation;
 *     - resource interpretation;
 *     - policy interpretation;
 *     - contract interpretation;
 *     - provenance validation;
 *     - portability validation;
 *     - target-specific validation.
 *
 * A syntactically valid metadata value may therefore still be semantically
 * invalid.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * The parser does not assign machine representation to literals.
 *
 * For example:
 *
 *     42
 *
 * does not imply:
 *
 *     32-bit
 *     64-bit
 *     native machine integer
 *
 * Likewise:
 *
 *     1.0
 *
 * does not imply a particular floating-point representation.
 *
 * Numeric meaning is established by semantic/type analysis.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Metadata parsing itself introduces no runtime effect.
 *
 * Metadata may later describe a construct that has effects such as:
 *
 *     IO
 *     network
 *     mutation
 *     randomness
 *     measurement
 *     foreign
 *     native
 *     learning
 *     adaptation
 *     reflection
 *     code generation
 *     simulation
 *
 * Those effects belong to the semantic construct being described, not to
 * generic metadata parsing.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * A metadata value containing a capability name does not grant that
 * capability.
 *
 * Example:
 *
 *     capability = quantum::measurement
 *
 * remains source data.
 *
 * Capability resolution and authorization occur downstream.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * A numeric metadata value does not reserve or consume a resource.
 *
 * Example:
 *
 *     memory = 1024
 *
 * remains metadata until a semantic schema assigns meaning to it.
 *
 * Resource feasibility belongs to resource analysis and target realization.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Metadata may carry provenance values.
 *
 * The parser preserves them.
 *
 * It does not assert their truthfulness.
 *
 * Trust, verification, signatures, hashes, timestamps and provenance chains
 * are downstream concerns.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * The permitted pipeline is:
 *
 *     metadata syntax
 *          ->
 *     domain-neutral AST
 *          ->
 *     semantic metadata
 *          ->
 *     canonical semantic model
 *          ->
 *     domain representation
 *
 * Depending on meaning, metadata may eventually contribute to:
 *
 *     classical representation
 *     quantum::ir
 *     HDL/hardware representation
 *     distributed representation
 *     execution metadata
 *     compilation metadata
 *     provenance
 *
 * There is no:
 *
 *     metadata -> quantum::ir
 *
 * shortcut.
 *
 * ============================================================================
 * SOURCE / TARGET SEPARATION
 * ============================================================================
 *
 * Metadata may describe target intent without selecting a target.
 *
 * Example:
 *
 *     target = hardware::accelerator
 *
 * is structurally valid metadata.
 *
 * It does not mean that:
 *
 *     a specific accelerator exists;
 *     the accelerator is selected;
 *     the accelerator is available;
 *     the program must be rewritten for that accelerator.
 *
 * Target realization remains downstream.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * All structural repetition is expressed with ANTLR repetition operators.
 *
 * There are no artificial finite limits on:
 *
 *     entry count
 *     array count
 *     tuple count
 *     nesting
 *     name qualification
 *     metadata values
 *
 * Therefore the language structure remains open-ended with respect to:
 *
 *     tiny programs
 *     large programs
 *     embedded systems
 *     multicore systems
 *     accelerators
 *     GPUs
 *     FPGAs
 *     ASICs
 *     QPUs
 *     simulators
 *     HPC systems
 *     clusters
 *     distributed systems
 *     cloud systems
 *     future computational targets
 *
 * Actual limits are implementation/resource limits, not grammar semantics.
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * Parser-level errors include:
 *
 *     - missing metadata key;
 *     - missing assignment;
 *     - missing metadata value;
 *     - malformed object;
 *     - malformed array;
 *     - malformed tuple;
 *     - malformed tagged value;
 *     - missing closing delimiter;
 *     - invalid separator placement.
 *
 * Semantic errors include:
 *
 *     - unknown metadata key;
 *     - invalid schema;
 *     - duplicate key where prohibited;
 *     - invalid value type;
 *     - unresolved reference;
 *     - invalid tag;
 *     - unsupported metadata version;
 *     - invalid capability;
 *     - invalid resource request;
 *     - invalid policy;
 *     - invalid provenance.
 *
 * Parser and semantic diagnostics must remain distinct.
 *
 * ============================================================================
 * NEGATIVE EXAMPLES
 * ============================================================================
 *
 * These must fail structurally where used as metadata values:
 *
 *     = value
 *     key =
 *     key = {
 *     key = [
 *     key = capability(
 *     { key = 1
 *     [1, 2
 *     (1, 2
 *
 * These are semantic concerns rather than parser concerns:
 *
 *     unknown_key = value
 *     duplicate_key = ...
 *     unavailable_capability = ...
 *     impossible_resource = ...
 *
 * ============================================================================
 * POSITIVE EXAMPLES
 * ============================================================================
 *
 * Scalars:
 *
 *     42
 *     1.25
 *     "Zamani"
 *     'x'
 *     true
 *     false
 *
 * Domain literals:
 *
 *     |0⟩
 *     10GiB
 *     10ms
 *
 * References:
 *
 *     version
 *     build::version
 *     quantum::measurement
 *
 * Arrays:
 *
 *     []
 *     [cpu, gpu, accelerator]
 *     [1, 2, 3,]
 *
 * Objects:
 *
 *     {}
 *     {language = "Zamani"}
 *     {language = "Zamani", version = "1",}
 *
 * String keys:
 *
 *     {"language" = "Zamani"}
 *
 * Tuples:
 *
 *     ()
 *     (value)
 *     (value,)
 *     (value1, value2)
 *
 * Tagged values:
 *
 *     capability(quantum::measurement)
 *     resource(memory)
 *     provenance(source::module)
 *     confidence(0.95)
 *
 * Nested structures:
 *
 *     {
 *         capability = quantum::measurement,
 *         resources = [
 *             memory,
 *             accelerator
 *         ],
 *         provenance = provenance(source::module)
 *     }
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * The test suite must verify:
 *
 *     scalar
 *     reference
 *     array
 *     object
 *     tuple
 *     tagged value
 *     nested values
 *     empty collections
 *     trailing commas
 *     qualified names
 *     string keys
 *     duplicate keys
 *     large symbolic values
 *     cross-domain metadata
 *
 * The test suite must also verify that metadata parsing does not accidentally
 * consume surrounding syntax belonging to:
 *
 *     attributes
 *     annotations
 *     pragmas
 *     declarations
 *     statements
 *     expressions
 *     modules
 *     quantum constructs
 *     HDL constructs
 *     resource constructs
 *     policy constructs
 *     metaprogramming constructs.
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests should generate metadata structures with increasing:
 *
 *     entry counts
 *     array sizes
 *     tuple sizes
 *     nesting
 *     qualified-name segments
 *     tagged-value nesting
 *
 * Test sizes are test parameters.
 *
 * They MUST NOT be converted into language-level constants.
 *
 * The purpose is to demonstrate that increasing resource requirements do not
 * require grammar redesign.
 *
 * ============================================================================
 * DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * Verify:
 *
 *     identical source
 *         ->
 *     identical lexical token sequence
 *         ->
 *     equivalent metadata parse structure
 *
 * No metadata rule may use:
 *
 *     semantic predicates
 *     randomness
 *     external state
 *     mutable global parser state
 *     runtime callbacks.
 *
 * ============================================================================
 * ROUND-TRIP CONTRACT
 * ============================================================================
 *
 * Where a source printer/formatter exists:
 *
 *     source
 *       ->
 *     lexer
 *       ->
 *     parser
 *       ->
 *     AST
 *       ->
 *     printer
 *       ->
 *     parser
 *
 * must preserve metadata semantics.
 *
 * In particular:
 *
 *     - object entry order;
 *     - array order;
 *     - tuple order;
 *     - duplicate entries;
 *     - qualified names;
 *     - tagged values;
 *     - literal values
 *
 * must not be silently discarded.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     NO machine-capacity constants.
 *     NO hardware-size constants.
 *     NO quantum-resource constants.
 *     NO metadata cardinality constants.
 *     NO nesting-depth constants.
 *     NO identifier-length constants.
 *     NO numeric-width assumptions.
 *     NO floating-point-width assumptions.
 *     NO tensor-rank assumptions.
 *     NO topology assumptions.
 *     NO device-count assumptions.
 *     NO backend assumptions.
 *     NO vendor assumptions.
 *
 * The finite literal token alternatives are lexical categories, not resource
 * limits.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This grammar itself contains no Rust.
 *
 * Generated frontend/compiler integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * The implementation must use safe Rust.
 *
 * No `unsafe` implementation is required or permitted for this feature.
 *
 * The grammar must not depend on Rust-specific type widths or memory layout.
 *
 * ============================================================================
 * REPOSITORY INTEGRATION
 * ============================================================================
 *
 * The intended integration chain is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *                     |
 *                     v
 *             canonical token stream
 *                     |
 *                     v
 *              grammar/core/names.g4
 *                     |
 *                     v
 *              grammar/core/metadata.g4
 *                     |
 *          +----------+----------+
 *          |          |          |
 *          v          v          v
 *      attributes annotations pragmas
 *          |          |          |
 *          +----------+----------+
 *                     |
 *                     v
 *             domain-neutral AST
 *                     |
 *                     v
 *             semantic validation
 *                     |
 *       +-------------+-------------+
 *       |             |             |
 *       v             v             v
 *   capabilities   resources    provenance
 *       |             |             |
 *       +-------------+-------------+
 *                     |
 *                     v
 *             canonical semantics
 *                     |
 *          +----------+----------+
 *          |          |          |
 *          v          v          v
 *      classical  quantum::ir  HDL/hardware
 *                     |
 *                     v
 *              target realization
 *
 * The grammar has no dependency on the bottom half of this pipeline.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * `metadata.g4` is COMPLETE when:
 *
 * [x] It is a parser grammar.
 *
 * [x] It uses tokenVocab = ZamaniLexer.
 *
 * [x] It imports Names.
 *
 * [x] It does not define lexer rules.
 *
 * [x] It does not duplicate identifier syntax.
 *
 * [x] It does not duplicate qualified-name syntax.
 *
 * [x] It uses the current canonical numeric token names.
 *
 * [x] It uses the current canonical boolean token names.
 *
 * [x] It uses canonical domain literal tokens.
 *
 * [x] It provides one canonical metadata-value model.
 *
 * [x] It represents objects.
 *
 * [x] It represents arrays.
 *
 * [x] It represents tuples.
 *
 * [x] It represents symbolic references.
 *
 * [x] It represents tagged values.
 *
 * [x] It supports string keys.
 *
 * [x] It supports arbitrary structural repetition.
 *
 * [x] It permits duplicate keys structurally.
 *
 * [x] It permits trailing commas in collection forms.
 *
 * [x] It does not create an expression language.
 *
 * [x] It does not interpret metadata semantics.
 *
 * [x] It does not grant capabilities.
 *
 * [x] It does not allocate resources.
 *
 * [x] It does not select targets.
 *
 * [x] It does not create IR.
 *
 * [x] It does not create quantum::ir.
 *
 * [x] It does not implement QEC.
 *
 * [x] It does not implement ZQN.
 *
 * [x] It does not implement HAL.
 *
 * [x] It contains no machine-size limits.
 *
 * [x] It contains no metadata-size limits.
 *
 * [x] It contains no Rust actions.
 *
 * [x] It requires no unsafe Rust.
 *
 * [x] It is independent of target hardware.
 *
 * [x] It has a defined AST contract.
 *
 * [x] It has a defined semantic contract.
 *
 * [x] It has a defined IR boundary.
 *
 * [x] It has a defined testing contract.
 *
 * [x] It has a defined compatibility contract.
 *
 * [x] It has a defined scalability contract.
 *
 * [x] It has a defined repository integration contract.
 *
 * Remaining repository verification:
 *
 * [ ] ANTLR generation succeeds.
 *
 * [ ] The `Names` import resolves in the configured grammar source path.
 *
 * [ ] `ZamaniParser.g4` receives the metadata rules through the canonical
 *     Core composition grammar.
 *
 * [ ] Attribute integration uses the canonical metadata-value structure where
 *     appropriate.
 *
 * [ ] AST conversion preserves all required source structure.
 *
 * [ ] Positive metadata tests pass.
 *
 * [ ] Negative metadata tests pass.
 *
 * [ ] Boundary tests pass.
 *
 * [ ] Cross-domain tests pass.
 *
 * [ ] Scalability tests pass within available implementation resources.
 *
 * [ ] Determinism tests pass.
 *
 * [ ] Round-trip tests pass where formatter infrastructure exists.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * Metadata is a universal STRUCTURAL DATA mechanism.
 *
 * It is not:
 *
 *     a runtime;
 *     a policy engine;
 *     a capability allocator;
 *     a resource allocator;
 *     an AI language;
 *     a quantum language;
 *     an HDL language;
 *     a hardware backend;
 *     an IR;
 *     a target selector.
 *
 * Its purpose is to provide one extensible, source-preserving structure that
 * can carry information across the complete Zamani architecture without
 * encoding the accidental limitations of a particular implementation or
 * machine.
 *
 * Therefore:
 *
 *     source metadata
 *          ->
 *     domain-neutral AST
 *          ->
 *     semantic interpretation
 *          ->
 *     target-independent semantic representation
 *          ->
 *     target realization
 *
 * remains the invariant.
 *
 * ============================================================================
 */
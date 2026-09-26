/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/ai/datasets.g4
 *
 * Grammar:
 *     AIDatasets
 *
 * Status:
 *     Production AI / ML dataset-domain parser grammar
 *
 * Language/runtime baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust edition 2021
 *
 * Safety:
 *     - parser grammar only;
 *     - no embedded Rust actions;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no runtime execution;
 *     - no hardware discovery;
 *     - no target selection;
 *     - no unsafe Rust.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns SOURCE-LEVEL SYNTAX for logical AI/ML dataset intent.
 *
 * A Zamani dataset is a logical data computation/resource.  It is not
 * intrinsically a filesystem, database, object store, network stream,
 * accelerator buffer, device memory region, or physical storage layout.
 *
 * This grammar therefore describes:
 *
 *   - dataset declarations;
 *   - dataset-local schema boundaries;
 *   - dataset sources;
 *   - fields;
 *   - feature/label/target roles;
 *   - splits;
 *   - partitions;
 *   - transformations;
 *   - projections;
 *   - filters;
 *   - maps;
 *   - joins;
 *   - concatenation;
 *   - batching;
 *   - windows;
 *   - sampling;
 *   - ordering;
 *   - shuffling;
 *   - streaming intent;
 *   - materialization intent;
 *   - caching intent;
 *   - validation intent;
 *   - provenance;
 *   - lineage;
 *   - reproducibility metadata;
 *   - resource requirements;
 *   - capabilities;
 *   - constraints;
 *   - preferences;
 *   - hints;
 *   - execution-region boundaries;
 *   - interoperability boundaries.
 *
 * ============================================================================
 * NON-OWNERSHIP
 * ============================================================================
 *
 * This file does NOT own:
 *
 *   - lexical tokens;
 *   - identifiers;
 *   - general expression syntax;
 *   - general type syntax;
 *   - canonical data schemas;
 *   - storage engines;
 *   - database semantics;
 *   - filesystem semantics;
 *   - network protocols;
 *   - tensor implementation;
 *   - model semantics;
 *   - training algorithms;
 *   - inference algorithms;
 *   - automatic differentiation;
 *   - resource discovery;
 *   - scheduling;
 *   - routing;
 *   - placement;
 *   - optimization;
 *   - runtime execution;
 *   - canonical IR;
 *   - quantum::ir;
 *   - QEC;
 *   - ZQN;
 *   - HAL.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     ZamaniParser
 *       |
 *       v
 *     AIDatasets
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +--------------------+--------------------+
 *       |                    |                    |
 *       v                    v                    v
 *   dataset semantics   resource/capability   provenance
 *       |                    |                    |
 *       +--------------------+--------------------+
 *                            |
 *                            v
 *                     canonical semantic IR
 *                            |
 *              +-------------+-------------+
 *              |             |             |
 *              v             v             v
 *          classical       AI/ML       distributed
 *          lowering       lowering      lowering
 *              |             |             |
 *              +-------------+-------------+
 *                            |
 *                       optimization
 *                            |
 *                       scheduling
 *                            |
 *                       realization
 *                            |
 *                         runtime
 *
 * This file MUST NOT construct IR.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Dataset syntax describes WHAT the program means.
 *
 * It MUST NOT encode universal assumptions about:
 *
 *   CPU count
 *   GPU count
 *   FPGA count
 *   accelerator count
 *   QPU count
 *   node count
 *   worker count
 *   memory capacity
 *   storage capacity
 *   partition count
 *   shard count
 *   batch count
 *   record count
 *   tensor rank
 *   sequence length
 *   device count
 *   topology
 *   physical addresses
 *
 * No constants such as:
 *
 *   MAX_ROWS
 *   MAX_COLUMNS
 *   MAX_FEATURES
 *   MAX_LABELS
 *   MAX_SHARDS
 *   MAX_PARTITIONS
 *   MAX_BATCH_SIZE
 *   MAX_WORKERS
 *   MAX_DATASETS
 *   MAX_STREAMS
 *   MAX_NODES
 *   MAX_DEVICES
 *   MAX_RECORDS
 *   MAX_SEQUENCE_LENGTH
 *
 * are language limits.
 *
 * Program values such as:
 *
 *     batch_size = 1024
 *
 * remain ordinary program semantics when explicitly written by the developer.
 *
 * The grammar MUST NOT reinterpret such values as universal implementation
 * ceilings.
 *
 * ============================================================================
 * REQUIREMENT / CAPABILITY / PREFERENCE SEPARATION
 * ============================================================================
 *
 * Dataset syntax may express:
 *
 *   requirement
 *   constraint
 *   capability
 *   preference
 *   hint
 *   budget
 *
 * These concepts are not interchangeable.
 *
 * Example:
 *
 *     @requires capability("streaming")
 *
 * is a semantic requirement.
 *
 * It does NOT select:
 *
 *     GPU 0
 *     node 7
 *     device 3
 *     provider X
 *
 * Actual realization belongs downstream.
 *
 * ============================================================================
 * STORAGE INDEPENDENCE
 * ============================================================================
 *
 * Dataset source syntax MUST remain provider-neutral.
 *
 * This grammar does not reserve or hard-code:
 *
 *   CSV
 *   JSON
 *   Parquet
 *   Arrow
 *   SQL
 *   S3
 *   HTTP
 *   Kafka
 *   local filesystem
 *   database vendors
 *   cloud providers
 *
 * Such technologies may be represented through ordinary names, dialects, or
 * interoperability constructs and resolved after parsing.
 *
 * ============================================================================
 * DATA GRAMMAR INTEGRATION
 * ============================================================================
 *
 * grammar/data/data.g4 remains the owner of the general logical data model.
 *
 * This file may introduce dataset-specific boundaries needed by AI/ML, but it
 * MUST NOT create a competing universal schema/data type system.
 *
 * Dataset-local schema declarations map semantically to the canonical data
 * schema model.
 *
 * Schema identity, compatibility, evolution, nullability, validation,
 * serialization and persistence remain owned by the data/schema subsystem.
 *
 * ============================================================================
 * TYPE INTEGRATION
 * ============================================================================
 *
 * Canonical type syntax is imported through:
 *
 *     Types
 *
 * This file uses:
 *
 *     typeExpression
 *
 * It does not create:
 *
 *     DatasetType
 *     FeatureType
 *     LabelType
 *     TensorType
 *
 * as competing type systems.
 *
 * ============================================================================
 * EXPRESSION INTEGRATION
 * ============================================================================
 *
 * General expression syntax is imported through:
 *
 *     Expressions
 *
 * This file uses:
 *
 *     expression
 *
 * for:
 *
 *   - source expressions;
 *   - dimensions;
 *   - batch sizes;
 *   - window sizes;
 *   - sampling parameters;
 *   - predicates;
 *   - keys;
 *   - ordering expressions;
 *   - transformation arguments;
 *   - resource values;
 *   - symbolic values;
 *   - compile-time values;
 *   - runtime values.
 *
 * Dataset-specific operation syntax is deliberately kept inside explicitly
 * dataset-scoped declarations/regions so that ordinary expressions do not
 * become ambiguous with dataset constructs.
 *
 * ============================================================================
 * ANTLR COMPOSITION
 * ============================================================================
 *
 * This grammar is a parser grammar.
 *
 * Stable public boundary:
 *
 *     datasetConstruct
 *
 * Canonical dependencies:
 *
 *     Core
 *     Types
 *     Expressions
 *
 * AIDatasets MUST NOT import:
 *
 *     AI
 *     Data
 *     Models
 *     Training
 *     Inference
 *
 * merely to obtain generic syntax.
 *
 * Dependency direction:
 *
 *     shared language grammar
 *            |
 *            v
 *        AIDatasets
 *            |
 *            v
 *       semantic analysis
 *
 * There MUST NOT be:
 *
 *     AI -> AIDatasets -> AI
 *
 * ============================================================================
 * ANNOTATION INTEGRATION
 * ============================================================================
 *
 * The actual lexer owns:
 *
 *     AT
 *
 * for '@'.
 *
 * The existing repository does NOT establish a valid production lexer token
 * named NANO_ANNOTATION in the canonical lexical source.  Therefore this file
 * deliberately uses AT and the shared name grammar instead of inventing or
 * depending on NANO_ANNOTATION.
 *
 * Dataset semantic annotations remain extensible names.
 *
 * Examples:
 *
 *     @dataset
 *     @source
 *     @schema
 *     @feature
 *     @label
 *     @target
 *     @split
 *     @partition
 *     @requires
 *     @capability
 *     @constraint
 *     @prefer
 *     @hint
 *
 * Their semantic meaning is validated downstream.
 *
 * ============================================================================
 */

parser grammar AIDatasets;

options {
    tokenVocab = ZamaniLexer;
}

import Core,
       Types,
       Expressions;


/* ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * IMPORTANT:
 *
 * This rule intentionally does NOT accept a bare `identifier` or arbitrary
 * `expression`.
 *
 * Doing so would make every ordinary expression a potential dataset construct
 * and would create ambiguity with the universal expression grammar.
 *
 * Dataset references remain ordinary expressions when used outside an explicit
 * dataset boundary.
 *
 * The dataset grammar therefore enters through explicit dataset annotations or
 * dataset-owned regions.
 * ========================================================================== */

datasetConstruct
    : datasetDeclaration
    | datasetRegion
    ;


/* ============================================================================
 * 2. DATASET DECLARATION
 * ============================================================================
 *
 * Canonical form:
 *
 *     @dataset training = source;
 *
 *     @dataset training: DatasetSchema = source;
 *
 *     @dataset training {
 *         @schema {
 *             input: Tensor<f32>;
 *             label: int;
 *         }
 *     }
 *
 *     @dataset training = source {
 *         @feature input: x, y;
 *         @label label;
 *     }
 *
 * The annotation name remains syntactically extensible.  Semantic analysis
 * MUST verify that the declaration annotation resolves to the dataset
 * construct.
 * ========================================================================== */

datasetDeclaration
    : datasetDeclarationAnnotation
      identifier
      datasetTypeAnnotation?
      datasetInitializer?
      datasetBody?
      SEMICOLON?
    ;


datasetDeclarationAnnotation
    : AT
      datasetAnnotationName
    ;


datasetAnnotationName
    : identifier
    | datasetAnnotationKeyword
    ;


datasetAnnotationKeyword
    : REQUIRES
    | CAPABILITY
    | CONSTRAINT
    | PREFER
    | HINT
    | RESOURCE
    | TARGET
    | CONTRACT
    | PROFILE
    | PROPERTY
    | PORTABILITY
    | SCALABILITY
    | PERFORMANCE
    | LATENCY
    | THROUGHPUT
    | BANDWIDTH
    | ENERGY
    | POWER
    | RELIABILITY
    | RESILIENCE
    | COST
    | AVAILABILITY
    | CAPACITY
    ;


datasetTypeAnnotation
    : COLON
      typeExpression
    ;


datasetInitializer
    : ASSIGN
      expression
    ;


datasetBody
    : LBRACE
      datasetMember*
      RBRACE
    ;


/* ============================================================================
 * 3. DATASET REGION
 * ============================================================================
 *
 * A region gives a named dataset scope in which dataset operations may be
 * described without polluting the ordinary expression grammar.
 *
 * Example:
 *
 *     @dataset_pipeline training {
 *         @source input = source;
 *         @transform normalized = map(input, normalize);
 *         @split train = split(normalized, ratio);
 *     }
 *
 * The semantic layer decides whether the annotation denotes a dataset,
 * pipeline, view, stream, or dialect extension.
 * ========================================================================== */

datasetRegion
    : AT
      datasetAnnotationName
      identifier
      datasetTypeAnnotation?
      LBRACE
      datasetRegionMember*
      RBRACE
      SEMICOLON?
    ;


datasetRegionMember
    : datasetSourceDeclaration
    | datasetSchemaDeclaration
    | datasetFeatureDeclaration
    | datasetLabelDeclaration
    | datasetTargetDeclaration
    | datasetSplitDeclaration
    | datasetPartitionDeclaration
    | datasetTransformDeclaration
    | datasetProjectionDeclaration
    | datasetFilterDeclaration
    | datasetMapDeclaration
    | datasetJoinDeclaration
    | datasetUnionDeclaration
    | datasetBatchDeclaration
    | datasetWindowDeclaration
    | datasetSamplingDeclaration
    | datasetOrderingDeclaration
    | datasetShuffleDeclaration
    | datasetStreamDeclaration
    | datasetMaterializationDeclaration
    | datasetCacheDeclaration
    | datasetValidationDeclaration
    | datasetProvenanceDeclaration
    | datasetLineageDeclaration
    | datasetReproducibilityDeclaration
    | datasetRequirementDeclaration
    | datasetCapabilityDeclaration
    | datasetConstraintDeclaration
    | datasetPreferenceDeclaration
    | datasetHintDeclaration
    | datasetInteropDeclaration
    | datasetMetadataDeclaration
    | datasetBindingDeclaration
    ;


/* ============================================================================
 * 4. GENERIC DATASET MEMBER
 * ============================================================================
 *
 * Generic metadata remains extensible without requiring a new keyword or
 * lexer token for every future AI/data concept.
 * ========================================================================== */

datasetBindingDeclaration
    : AT
      datasetAnnotationName
      identifier
      datasetTypeAnnotation?
      datasetInitializer?
      SEMICOLON?
    ;


datasetMetadataDeclaration
    : AT
      datasetAnnotationName
      datasetMetadataPayload?
      SEMICOLON?
    ;


datasetMetadataPayload
    : LPAREN datasetArgumentList? RPAREN
    | COLON expression
    | ASSIGN expression
    | LBRACE datasetMetadataEntry* RBRACE
    ;


datasetMetadataEntry
    : identifier
      (COLON expression | ASSIGN expression)?
      SEMICOLON?
    ;


/* ============================================================================
 * 5. SOURCE
 * ============================================================================
 *
 * A source is a logical provider boundary.
 *
 * The expression identifies source intent.
 *
 * It does not imply a filesystem, database, object store, network endpoint,
 * accelerator, or provider.
 * ========================================================================== */

datasetSourceDeclaration
    : AT
      datasetAnnotationName
      identifier
      datasetTypeAnnotation?
      ASSIGN
      expression
      SEMICOLON?
    ;


/* ============================================================================
 * 6. SCHEMA
 * ============================================================================
 *
 * Dataset-local schema syntax is intentionally a boundary into the canonical
 * data/schema semantic system.
 * ========================================================================== */

datasetSchemaDeclaration
    : AT
      datasetAnnotationName
      datasetSchemaBody
      SEMICOLON?
    ;


datasetSchemaBody
    : LBRACE
      datasetFieldDeclaration*
      RBRACE
    ;


datasetFieldDeclaration
    : identifier
      COLON
      typeExpression
      datasetFieldInitializer?
      datasetFieldMetadata*
      SEMICOLON
    ;


datasetFieldInitializer
    : ASSIGN
      expression
    ;


datasetFieldMetadata
    : AT
      datasetAnnotationName
      datasetMetadataPayload?
    ;


/* ============================================================================
 * 7. FIELD ROLES
 * ============================================================================
 *
 * Features, labels and targets refer to logical dataset fields.
 *
 * They do not impose tensor dimensions, model dimensions, storage layout or
 * machine limits.
 * ========================================================================== */

datasetFeatureDeclaration
    : AT
      datasetAnnotationName
      datasetFieldReferenceList
      SEMICOLON?
    ;


datasetLabelDeclaration
    : AT
      datasetAnnotationName
      datasetFieldReferenceList
      SEMICOLON?
    ;


datasetTargetDeclaration
    : AT
      datasetAnnotationName
      datasetFieldReferenceList
      SEMICOLON?
    ;


datasetFieldReferenceList
    : datasetFieldReference
      (COMMA datasetFieldReference)*
      COMMA?
    ;


datasetFieldReference
    : qualifiedName
    ;


/* ============================================================================
 * 8. SPLITS
 * ============================================================================
 *
 * A split is a logical view/subset.
 *
 * No fixed number of splits is encoded.
 * ========================================================================== */

datasetSplitDeclaration
    : AT
      datasetAnnotationName
      identifier
      ASSIGN
      datasetSplitSpecification
      SEMICOLON?
    ;


datasetSplitSpecification
    : datasetReferenceExpression
      datasetSplitSelector?
    ;


datasetSplitSelector
    : LPAREN
      datasetArgumentList?
      RPAREN
    ;


datasetReferenceExpression
    : expression
    ;


/* ============================================================================
 * 9. PARTITIONING
 * ============================================================================
 *
 * Partitioning describes logical partitioning intent.
 *
 * It does NOT prescribe:
 *
 *   - shard count;
 *   - worker count;
 *   - node count;
 *   - topology;
 *   - physical storage.
 * ========================================================================== */

datasetPartitionDeclaration
    : AT
      datasetAnnotationName
      identifier
      ASSIGN
      datasetPartitionSpecification
      SEMICOLON?
    ;


datasetPartitionSpecification
    : datasetReferenceExpression
      datasetPartitionClause*
    ;


datasetPartitionClause
    : datasetPartitionByClause
    | datasetPartitionStrategyClause
    | datasetPartitionConstraintClause
    | datasetPartitionPreferenceClause
    ;


datasetPartitionByClause
    : AT
      datasetAnnotationName
      datasetExpressionList
    ;


datasetPartitionStrategyClause
    : AT
      datasetAnnotationName
      expression
    ;


datasetPartitionConstraintClause
    : AT
      datasetAnnotationName
      expression
    ;


datasetPartitionPreferenceClause
    : AT
      datasetAnnotationName
      expression
    ;


/* ============================================================================
 * 10. TRANSFORMATIONS
 * ============================================================================
 *
 * Transformations are represented as explicit named dataset declarations.
 *
 * This keeps dataset operations out of the universal expression entry point.
 * ========================================================================== */

datasetTransformDeclaration
    : AT
      datasetAnnotationName
      identifier
      ASSIGN
      datasetTransformSpecification
      SEMICOLON?
    ;


datasetTransformSpecification
    : datasetOperationCall
    ;


datasetOperationCall
    : qualifiedName
      LPAREN
      datasetArgumentList?
      RPAREN
    ;


datasetArgumentList
    : expression
      (COMMA expression)*
      COMMA?
    ;


/* ============================================================================
 * 11. PROJECTION
 * ========================================================================== */

datasetProjectionDeclaration
    : AT
      datasetAnnotationName
      identifier
      ASSIGN
      datasetProjectionSpecification
      SEMICOLON?
    ;


datasetProjectionSpecification
    : datasetReferenceExpression
      datasetProjectionClause
    ;


datasetProjectionClause
    : LPAREN
      datasetProjectionItemList?
      RPAREN
    ;


datasetProjectionItemList
    : datasetProjectionItem
      (COMMA datasetProjectionItem)*
      COMMA?
    ;


datasetProjectionItem
    : expression
    ;


/* ============================================================================
 * 12. FILTER
 * ========================================================================== */

datasetFilterDeclaration
    : AT
      datasetAnnotationName
      identifier
      ASSIGN
      datasetFilterSpecification
      SEMICOLON?
    ;


datasetFilterSpecification
    : datasetReferenceExpression
      datasetPredicateClause
    ;


datasetPredicateClause
    : LPAREN
      expression
      RPAREN
    ;


/* ============================================================================
 * 13. MAP
 * ========================================================================== */

datasetMapDeclaration
    : AT
      datasetAnnotationName
      identifier
      ASSIGN
      datasetMapSpecification
      SEMICOLON?
    ;


datasetMapSpecification
    : datasetReferenceExpression
      datasetMapClause
    ;


datasetMapClause
    : LPAREN
      datasetMapArgumentList?
      RPAREN
    ;


datasetMapArgumentList
    : expression
      (COMMA expression)*
      COMMA?
    ;


/* ============================================================================
 * 14. JOIN
 * ============================================================================
 *
 * Joins are logical data operations.
 *
 * Physical join algorithms and distribution are downstream concerns.
 * ========================================================================== */

datasetJoinDeclaration
    : AT
      datasetAnnotationName
      identifier
      ASSIGN
      datasetJoinSpecification
      SEMICOLON?
    ;


datasetJoinSpecification
    : datasetReferenceExpression
      datasetJoinOperator
      datasetReferenceExpression
      datasetJoinCondition?
    ;


datasetJoinOperator
    : LPAREN
      datasetArgumentList?
      RPAREN
    ;


datasetJoinCondition
    : AT
      datasetAnnotationName
      expression
    ;


/* ============================================================================
 * 15. UNION / CONCATENATION
 * ========================================================================== */

datasetUnionDeclaration
    : AT
      datasetAnnotationName
      identifier
      ASSIGN
      datasetUnionSpecification
      SEMICOLON?
    ;


datasetUnionSpecification
    : datasetReferenceExpression
      datasetUnionOperator
      datasetReferenceExpression
      (COMMA datasetReferenceExpression)*
    ;


datasetUnionOperator
    : LPAREN
      datasetArgumentList?
      RPAREN
    ;


/* ============================================================================
 * 16. BATCHING
 * ============================================================================
 *
 * Batch size is an expression, not a grammar constant.
 * ========================================================================== */

datasetBatchDeclaration
    : AT
      datasetAnnotationName
      identifier
      ASSIGN
      datasetBatchSpecification
      SEMICOLON?
    ;


datasetBatchSpecification
    : datasetReferenceExpression
      datasetBatchClause
    ;


datasetBatchClause
    : LPAREN
      datasetArgumentList?
      RPAREN
    ;


/* ============================================================================
 * 17. WINDOWING
 * ============================================================================
 *
 * Window dimensions and steps are expressions.
 * ========================================================================== */

datasetWindowDeclaration
    : AT
      datasetAnnotationName
      identifier
      ASSIGN
      datasetWindowSpecification
      SEMICOLON?
    ;


datasetWindowSpecification
    : datasetReferenceExpression
      datasetWindowClause
    ;


datasetWindowClause
    : LPAREN
      datasetArgumentList?
      RPAREN
    ;


/* ============================================================================
 * 18. SAMPLING
 * ============================================================================
 *
 * Sampling parameters remain semantic expressions.
 *
 * The grammar does not impose a maximum sample size, population size,
 * probability precision, or number of sampling stages.
 * ========================================================================== */

datasetSamplingDeclaration
    : AT
      datasetAnnotationName
      identifier
      ASSIGN
      datasetSamplingSpecification
      SEMICOLON?
    ;


datasetSamplingSpecification
    : datasetReferenceExpression
      datasetSamplingClause
    ;


datasetSamplingClause
    : LPAREN
      datasetArgumentList?
      RPAREN
    ;


/* ============================================================================
 * 19. ORDERING
 * ========================================================================== */

datasetOrderingDeclaration
    : AT
      datasetAnnotationName
      identifier
      ASSIGN
      datasetOrderingSpecification
      SEMICOLON?
    ;


datasetOrderingSpecification
    : datasetReferenceExpression
      datasetOrderingClause
    ;


datasetOrderingClause
    : LPAREN
      datasetOrderingItemList?
      RPAREN
    ;


datasetOrderingItemList
    : datasetOrderingItem
      (COMMA datasetOrderingItem)*
      COMMA?
    ;


datasetOrderingItem
    : expression
    ;


/* ============================================================================
 * 20. SHUFFLING
 * ============================================================================
 *
 * Randomness is semantic metadata.
 *
 * The grammar never silently creates a seed.
 * ========================================================================== */

datasetShuffleDeclaration
    : AT
      datasetAnnotationName
      identifier
      ASSIGN
      datasetShuffleSpecification
      SEMICOLON?
    ;


datasetShuffleSpecification
    : datasetReferenceExpression
      datasetShuffleClause?
    ;


datasetShuffleClause
    : LPAREN
      datasetArgumentList?
      RPAREN
    ;


/* ============================================================================
 * 21. STREAMING
 * ============================================================================
 *
 * Streaming expresses dataflow intent.
 *
 * It does not select a broker, transport, queue, network or buffering
 * implementation.
 * ========================================================================== */

datasetStreamDeclaration
    : AT
      datasetAnnotationName
      identifier
      ASSIGN
      datasetStreamSpecification
      SEMICOLON?
    ;


datasetStreamSpecification
    : datasetReferenceExpression
      datasetStreamClause?
    ;


datasetStreamClause
    : LPAREN
      datasetArgumentList?
      RPAREN
    ;


/* ============================================================================
 * 22. MATERIALIZATION
 * ============================================================================
 *
 * Materialization is an execution/storage intent boundary.
 *
 * It does not select a physical storage technology.
 * ========================================================================== */

datasetMaterializationDeclaration
    : AT
      datasetAnnotationName
      identifier
      ASSIGN
      datasetMaterializationSpecification
      SEMICOLON?
    ;


datasetMaterializationSpecification
    : datasetReferenceExpression
      datasetMaterializationClause?
    ;


datasetMaterializationClause
    : LPAREN
      datasetArgumentList?
      RPAREN
    ;


/* ============================================================================
 * 23. CACHING
 * ========================================================================== */

datasetCacheDeclaration
    : AT
      datasetAnnotationName
      identifier
      ASSIGN
      datasetCacheSpecification
      SEMICOLON?
    ;


datasetCacheSpecification
    : datasetReferenceExpression
      datasetCacheClause?
    ;


datasetCacheClause
    : LPAREN
      datasetArgumentList?
      RPAREN
    ;


/* ============================================================================
 * 24. VALIDATION
 * ============================================================================
 *
 * Validation declarations express validation intent.
 *
 * Validation implementation remains semantic/runtime responsibility.
 * ========================================================================== */

datasetValidationDeclaration
    : AT
      datasetAnnotationName
      datasetValidationSpecification
      SEMICOLON?
    ;


datasetValidationSpecification
    : expression
    ;


/* ============================================================================
 * 25. PROVENANCE
 * ============================================================================
 *
 * Provenance describes logical origin and reproducibility information.
 *
 * It does not expose secrets or force a particular provenance backend.
 * ========================================================================== */

datasetProvenanceDeclaration
    : AT
      datasetAnnotationName
      datasetProvenanceSpecification
      SEMICOLON?
    ;


datasetProvenanceSpecification
    : expression
    ;


/* ============================================================================
 * 26. LINEAGE
 * ============================================================================
 *
 * Lineage identifies semantic relationships between dataset computations.
 * ========================================================================== */

datasetLineageDeclaration
    : AT
      datasetAnnotationName
      datasetLineageSpecification
      SEMICOLON?
    ;


datasetLineageSpecification
    : expression
    ;


/* ============================================================================
 * 27. REPRODUCIBILITY
 * ============================================================================
 *
 * Reproducibility metadata is explicit.
 *
 * The grammar does NOT silently inject:
 *
 *   - random seeds;
 *   - timestamps;
 *   - machine identifiers;
 *   - backend identifiers;
 *   - hardware state.
 *
 * Such information must come from explicit source semantics or downstream
 * provenance systems.
 * ========================================================================== */

datasetReproducibilityDeclaration
    : AT
      datasetAnnotationName
      datasetReproducibilitySpecification
      SEMICOLON?
    ;


datasetReproducibilitySpecification
    : expression
    ;


/* ============================================================================
 * 28. RESOURCE REQUIREMENTS
 * ============================================================================
 *
 * Requirements describe what must be available.
 *
 * They do not select a concrete target.
 * ========================================================================== */

datasetRequirementDeclaration
    : AT
      datasetAnnotationName
      expression
      SEMICOLON?
    ;


/* ============================================================================
 * 29. CAPABILITIES
 * ========================================================================== */

datasetCapabilityDeclaration
    : AT
      datasetAnnotationName
      expression
      SEMICOLON?
    ;


/* ============================================================================
 * 30. CONSTRAINTS
 * ========================================================================== */

datasetConstraintDeclaration
    : AT
      datasetAnnotationName
      expression
      SEMICOLON?
    ;


/* ============================================================================
 * 31. PREFERENCES
 * ========================================================================== */

datasetPreferenceDeclaration
    : AT
      datasetAnnotationName
      expression
      SEMICOLON?
    ;


/* ============================================================================
 * 32. HINTS
 * ========================================================================== */

datasetHintDeclaration
    : AT
      datasetAnnotationName
      expression
      SEMICOLON?
    ;


/* ============================================================================
 * 33. INTEROPERABILITY
 * ============================================================================
 *
 * Foreign formats/providers may be named here, but their implementation is
 * resolved downstream.
 *
 * This grammar does not turn an interoperability name into a core language
 * storage implementation.
 * ========================================================================== */

datasetInteropDeclaration
    : AT
      datasetAnnotationName
      identifier
      datasetInteropPayload?
      SEMICOLON?
    ;


datasetInteropPayload
    : ASSIGN expression
    | COLON typeExpression
    | LPAREN datasetArgumentList? RPAREN
    ;


/* ============================================================================
 * 34. DATASET EXECUTION REGION
 * ============================================================================
 *
 * This is a semantic boundary for execution policy attached to a dataset.
 *
 * It does not encode placement or scheduling.
 * ========================================================================== */

datasetExecutionRegion
    : AT
      datasetAnnotationName
      identifier
      LBRACE
      datasetRegionMember*
      RBRACE
    ;


/* ============================================================================
 * 35. GENERAL DATASET REFERENCE
 * ============================================================================
 *
 * Dataset references are deliberately NOT exposed as the public
 * `datasetConstruct` alternative.
 *
 * They are used internally by dataset-owned declarations.
 *
 * This prevents arbitrary identifiers from becoming dataset constructs.
 * ========================================================================== */

datasetReference
    : qualifiedName
    ;


/* ============================================================================
 * 36. EXPRESSION LIST
 * ============================================================================
 *
 * Local list rule prevents this grammar from depending on a particular
 * argument-list implementation while still reusing canonical expressions.
 * ========================================================================== */

datasetExpressionList
    : expression
      (COMMA expression)*
      COMMA?
    ;


/* ============================================================================
 * 37. INTEGRATION CONTRACT
 * ============================================================================
 *
 * Canonical upstream contracts:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/antlr/ZamaniParser.g4
 *     grammar/core/
 *     grammar/types/
 *     grammar/expressions/
 *
 * Canonical downstream consumers:
 *
 *     frontend AST
 *     semantic analysis
 *     data semantic model
 *     AI semantic model
 *     resource/capability analysis
 *     provenance/lineage
 *     canonical IR
 *     compiler
 *     runtime
 *
 * AI integration:
 *
 *     grammar/ai/ai.g4
 *
 * MUST expose `datasetConstruct` through the AI-domain composition boundary.
 *
 * IMPORTANT:
 *
 * `ai.g4` currently contains a broad generic annotated-declaration path.
 * Production composition must ensure that dataset-owned annotated constructs
 * are dispatched through `datasetConstruct` rather than duplicated by the
 * generic AI declaration alternative.
 *
 * The resolution must be performed structurally in the parser-composition
 * layer, not through semantic predicates in this leaf grammar.
 *
 * This file itself requires no re-edit when that dispatcher is updated.
 *
 * ============================================================================
 * DATA INTEGRATION
 * ============================================================================
 *
 * `grammar/data/data.g4` owns the universal logical data subsystem.
 *
 * Dataset-specific schema declarations in this file are syntax boundaries.
 *
 * Semantic lowering:
 *
 *     AIDatasets field/schema
 *             |
 *             v
 *     canonical data schema model
 *
 * This file must NOT create a second universal schema IR.
 *
 * ============================================================================
 * AI INTEGRATION
 * ============================================================================
 *
 * Dataset semantics may be consumed by:
 *
 *     models
 *     training
 *     inference
 *     pipelines
 *     agents
 *     differentiation
 *     tensors
 *
 * The dependency remains semantic:
 *
 *     dataset
 *        |
 *        v
 *     canonical AI/data model
 *
 * No leaf grammar may create an AI-specific IR.
 *
 * ============================================================================
 * TENSOR INTEGRATION
 * ============================================================================
 *
 * Tensor-valued dataset fields use canonical `typeExpression`.
 *
 * This grammar does not define tensor syntax.
 *
 * Tensor rank, shape, dimensions, element types and layout are semantic/type
 * concerns.
 *
 * No tensor dimension becomes a universal grammar limit.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * AI datasets may participate in hybrid classical/quantum programs through
 * ordinary expressions and semantic bindings.
 *
 * This file does not define:
 *
 *     Qubit
 *     quantum gates
 *     physical qubits
 *     topology
 *     routing
 *     scheduling
 *     QEC
 *     ZQN
 *
 * If a dataset feeds a quantum computation:
 *
 *     dataset syntax
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum semantic representation
 *          |
 *          v
 *     quantum::ir
 *
 * `quantum::ir` remains the canonical quantum boundary.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Partition, stream, cache, materialization and distribution declarations
 * describe logical intent.
 *
 * They do not specify:
 *
 *     worker count;
 *     node count;
 *     device count;
 *     topology;
 *     physical shard identifiers.
 *
 * Distributed/resource systems resolve those later.
 *
 * ============================================================================
 * HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware capabilities may be expressed indirectly through semantic
 * requirements such as:
 *
 *     @requires capability("tensor.compute")
 *
 * or:
 *
 *     @requires memory(required_memory)
 *
 * This grammar does not select:
 *
 *     gpu0
 *     cpu0
 *     qpu0
 *     fpga0
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every public rule maps to a domain-neutral AST construct.
 *
 * Recommended semantic categories:
 *
 *     DatasetDeclaration
 *     DatasetRegion
 *     DatasetSource
 *     DatasetSchema
 *     DatasetField
 *     DatasetRole
 *     DatasetSplit
 *     DatasetPartition
 *     DatasetTransform
 *     DatasetProjection
 *     DatasetFilter
 *     DatasetMap
 *     DatasetJoin
 *     DatasetUnion
 *     DatasetBatch
 *     DatasetWindow
 *     DatasetSampling
 *     DatasetOrdering
 *     DatasetShuffle
 *     DatasetStream
 *     DatasetMaterialization
 *     DatasetCache
 *     DatasetValidation
 *     DatasetProvenance
 *     DatasetLineage
 *     DatasetReproducibility
 *     DatasetRequirement
 *     DatasetCapability
 *     DatasetConstraint
 *     DatasetPreference
 *     DatasetHint
 *     DatasetInterop
 *
 * The AST MUST preserve:
 *
 *     source spans;
 *     names;
 *     expressions;
 *     types;
 *     ordering;
 *     annotations;
 *     semantic arguments;
 *     source-level provenance metadata.
 *
 * The AST MUST NOT resolve:
 *
 *     physical storage;
 *     hardware;
 *     node placement;
 *     accelerator identity;
 *     scheduling;
 *     routing.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     - resolving dataset identities;
 *     - checking field references;
 *     - validating schema compatibility;
 *     - validating feature/label/target roles;
 *     - validating transformations;
 *     - validating partition expressions;
 *     - validating ordering;
 *     - validating sampling semantics;
 *     - validating reproducibility declarations;
 *     - validating provenance;
 *     - validating requirements/capabilities/constraints;
 *     - validating resource availability;
 *     - checking cross-domain compatibility.
 *
 * A syntactically valid dataset program may therefore still fail semantic
 * validation because a target lacks a required capability or resource.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar MUST NOT define IR.
 *
 * Dataset semantics lower into the repository's canonical semantic/data/AI IR
 * according to the existing compiler architecture.
 *
 * If a dataset operation participates in quantum computation, the quantum
 * portion ultimately uses:
 *
 *     quantum::ir
 *
 * There is no dataset-specific quantum IR.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing must depend only on:
 *
 *     source text;
 *     selected grammar/version;
 *     lexical vocabulary;
 *     parser composition;
 *     explicitly selected dialect configuration.
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware;
 *     filesystem state;
 *     network state;
 *     current time;
 *     randomness;
 *     environment variables;
 *     runtime state.
 *
 * In particular, dataset shuffling syntax MUST NOT implicitly generate a
 * random seed.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Structural diagnostics belong to the parser.
 *
 * Semantic diagnostics belong downstream.
 *
 * Diagnostics should preserve:
 *
 *     source span;
 *     construct kind;
 *     offending expression/name;
 *     expected structure.
 *
 * Resource diagnostics should distinguish:
 *
 *     invalid source
 *
 * from:
 *
 *     valid source but unavailable resource/capability.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Dataset grammar processing MUST NOT:
 *
 *     - open files;
 *     - access databases;
 *     - access network endpoints;
 *     - execute provider code;
 *     - access credentials;
 *     - inspect hardware;
 *     - execute expressions.
 *
 * Dataset source expressions are parsed data, not executed instructions.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar uses unbounded structural repetition:
 *
 *     *
 *     +
 *     optional elements
 *
 * rather than artificial finite limits.
 *
 * It contains no language-level maximum for:
 *
 *     datasets;
 *     fields;
 *     partitions;
 *     transformations;
 *     joins;
 *     pipeline stages;
 *     batches;
 *     windows;
 *     samples;
 *     streams;
 *     records;
 *     dimensions;
 *     nodes;
 *     devices;
 *     workers.
 *
 * Practical limits are implementation/resource limits.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
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
 * It also contains no:
 *
 *     GPU 0
 *     CPU 0
 *     QPU 0
 *     FPGA 0
 *     node 0
 *
 * as language-level hardware assumptions.
 *
 * Numeric values appearing inside `expression` are program semantics.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust implementation.
 *
 * Zamani's consuming implementation MUST target:
 *
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * and MUST use safe Rust only.
 *
 * No `unsafe` block, `unsafe fn`, unsafe trait implementation, raw-pointer
 * requirement, or unsafe FFI assumption is introduced by this grammar.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * The corresponding grammar conformance suite should cover:
 *
 * POSITIVE:
 *
 *     @dataset training = source;
 *
 *     @dataset training: DatasetSchema = source;
 *
 *     @dataset training {
 *         @schema {
 *             input: Tensor<f32>;
 *             label: int;
 *         }
 *     }
 *
 *     @dataset training {
 *         @feature input, metadata;
 *         @label label;
 *         @target target;
 *         @split train = training(...);
 *         @partition shards = training(...);
 *         @transform normalized = map(training, normalize);
 *         @filter selected = filter(normalized, predicate);
 *         @batch batches = batch(selected, batch_size);
 *         @window windows = window(batches, window_size, step);
 *         @sample sampled = sample(windows, sampling_policy);
 *         @stream stream = streaming(sampled);
 *         @materialize materialized = materialize(stream);
 *     }
 *
 * RESOURCE:
 *
 *     @dataset training {
 *         @requires capability("tensor.compute");
 *         @requires memory(required_memory);
 *         @prefer capability("accelerator.compute");
 *         @constraint latency_budget;
 *         @hint locality;
 *     }
 *
 * PROVENANCE:
 *
 *     @dataset training {
 *         @provenance provenance_expression;
 *         @lineage lineage_expression;
 *         @reproducibility reproducibility_expression;
 *     }
 *
 * INTEROPERABILITY:
 *
 *     @dataset training {
 *         @interop provider = provider_expression;
 *     }
 *
 * NEGATIVE:
 *
 *     malformed annotation;
 *     missing dataset identifier;
 *     malformed type;
 *     malformed expression;
 *     missing assignment expression where required;
 *     malformed field declaration;
 *     malformed partition expression;
 *     malformed transformation call.
 *
 * SCALABILITY:
 *
 *     arbitrarily many fields;
 *     arbitrarily many dataset members;
 *     arbitrarily many transformations;
 *     arbitrarily many partitions;
 *     arbitrarily many pipeline stages;
 *     arbitrarily large symbolic expressions.
 *
 * The tests must NOT define an arbitrary maximum and call that maximum a
 * language capability.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *   [x] dataset syntax has one public parser boundary;
 *   [x] arbitrary identifiers are not accepted as dataset constructs;
 *   [x] AT is used instead of the nonexistent NANO_ANNOTATION token;
 *   [x] type syntax is delegated to Types;
 *   [x] expression syntax is delegated to Expressions;
 *   [x] dataset schema is separated from universal data semantics;
 *   [x] transformations are explicit dataset-owned declarations;
 *   [x] partitions are logical intent;
 *   [x] streaming is provider-neutral;
 *   [x] materialization is provider-neutral;
 *   [x] caching is provider-neutral;
 *   [x] provenance is explicit;
 *   [x] lineage is explicit;
 *   [x] reproducibility is explicit;
 *   [x] requirements are distinct from preferences;
 *   [x] capabilities are distinct from concrete resources;
 *   [x] no hardware limits are encoded;
 *   [x] no storage vendor is encoded;
 *   [x] no AI framework is encoded;
 *   [x] no runtime behavior is encoded;
 *   [x] no IR is constructed;
 *   [x] quantum::ir remains the canonical quantum boundary;
 *   [x] no QEC/ZQN/routing/scheduling implementation leaks into grammar;
 *   [x] source-level determinism is preserved;
 *   [x] source spans remain representable;
 *   [x] Safe Rust remains sufficient;
 *   [x] Rust 1.97/1.97.1 compatibility is preserved;
 *   [x] no artificial scalability ceiling is introduced.
 *
 * ============================================================================
 */
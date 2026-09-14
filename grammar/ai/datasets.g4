/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/ai/datasets.g4
 *
 * Role:
 *     Production AI / ML dataset-domain parser grammar.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Language/runtime baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust edition 2021
 *
 * Safety:
 *     - Parser grammar only.
 *     - No embedded Rust actions.
 *     - No semantic predicates.
 *     - No target-specific implementation.
 *     - No filesystem access.
 *     - No network access.
 *     - No runtime execution.
 *     - No unsafe implementation.
 *
 * ============================================================================
 *
 * PURPOSE
 * ============================================================================
 *
 * This file owns SOURCE-LEVEL SYNTAX for AI/ML dataset intent.
 *
 * A dataset in Zamani represents a logical computational data resource.
 *
 * The syntax can express:
 *
 *     - dataset declarations;
 *     - dataset references;
 *     - dataset sources;
 *     - dataset schemas;
 *     - dataset fields;
 *     - features;
 *     - labels;
 *     - targets;
 *     - metadata;
 *     - splits;
 *     - partitions;
 *     - transformations;
 *     - projections;
 *     - filters;
 *     - maps;
 *     - batches;
 *     - windows;
 *     - sampling;
 *     - shuffling;
 *     - ordering;
 *     - joins;
 *     - concatenation;
 *     - composition;
 *     - streaming intent;
 *     - materialization intent;
 *     - caching intent;
 *     - provenance;
 *     - reproducibility metadata;
 *     - dataset requirements;
 *     - dataset capabilities;
 *     - dataset constraints;
 *     - dataset preferences;
 *     - dataset execution regions.
 *
 * It deliberately does NOT define:
 *
 *     - filesystem formats;
 *     - database engines;
 *     - object stores;
 *     - network protocols;
 *     - physical storage;
 *     - memory allocation;
 *     - worker counts;
 *     - device counts;
 *     - GPU counts;
 *     - CPU counts;
 *     - cluster sizes;
 *     - shard counts;
 *     - node topology;
 *     - hardware addresses;
 *     - accelerator selection;
 *     - scheduling;
 *     - placement;
 *     - runtime execution.
 *
 * ============================================================================
 *
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          +----------------------------+
 *          |                            |
 *          v                            v
 *     Types / Expressions           AI domain
 *                                       |
 *                                       v
 *                                AIDatasets
 *                                       |
 *                                       v
 *                                  Frontend AST
 *                                       |
 *                                       v
 *                              Semantic Analysis
 *                                       |
 *                   +-------------------+-------------------+
 *                   |                                       |
 *                   v                                       v
 *             Dataset Model                         Resource Metadata
 *                   |                                       |
 *                   +-------------------+-------------------+
 *                                       |
 *                                       v
 *                                 Canonical IR
 *                                       |
 *                         +-------------+-------------+
 *                         |             |             |
 *                         v             v             v
 *                     Classical      AI/ML        Distributed
 *                     computation    lowering      lowering
 *                         |             |             |
 *                         +-------------+-------------+
 *                                       |
 *                                       v
 *                                  Optimization
 *                                       |
 *                                       v
 *                                   Scheduling
 *                                       |
 *                                       v
 *                                Target realization
 *                                       |
 *                                       v
 *                                     Runtime
 *
 * AIDatasets MUST NOT construct IR directly.
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - dataset source syntax;
 *     - dataset declaration syntax;
 *     - dataset reference syntax;
 *     - dataset schema boundary syntax;
 *     - dataset field declarations;
 *     - dataset feature/label/target declarations;
 *     - dataset split declarations;
 *     - dataset partition declarations;
 *     - dataset transformation syntax;
 *     - dataset selection syntax;
 *     - dataset batching syntax;
 *     - dataset windowing syntax;
 *     - dataset sampling syntax;
 *     - dataset ordering syntax;
 *     - dataset composition syntax;
 *     - dataset provenance boundary syntax;
 *     - dataset reproducibility metadata boundary;
 *     - dataset resource/capability/constraint boundary;
 *     - dataset execution-region boundary.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical token definitions;
 *     - identifiers;
 *     - general expressions;
 *     - general statements;
 *     - canonical types;
 *     - canonical data schemas;
 *     - tensor types;
 *     - tensor implementation;
 *     - model definitions;
 *     - training algorithms;
 *     - inference algorithms;
 *     - numerical algorithms;
 *     - storage engines;
 *     - filesystem semantics;
 *     - databases;
 *     - network protocols;
 *     - distributed scheduling;
 *     - resource allocation;
 *     - hardware discovery;
 *     - accelerator selection;
 *     - optimization;
 *     - runtime execution;
 *     - canonical IR;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN.
 *
 * ============================================================================
 *
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Dataset syntax describes WHAT data computation means.
 *
 * It must not silently encode WHERE the data physically lives or HOW it is
 * executed.
 *
 * Examples of portable intent:
 *
 *     @dataset training = source;
 *
 *     @dataset training {
 *         schema {
 *             input: Tensor<f32>;
 *             label: int;
 *         }
 *     }
 *
 *     @dataset batches =
 *         batch(training, batchSize);
 *
 *     @dataset filtered =
 *         filter(training, predicate);
 *
 *     @dataset windows =
 *         window(stream, windowSize, step);
 *
 * The semantic meaning remains independent of:
 *
 *     - CPU;
 *     - GPU;
 *     - TPU;
 *     - NPU;
 *     - FPGA;
 *     - ASIC;
 *     - cluster;
 *     - cloud;
 *     - embedded machine;
 *     - future hardware.
 *
 * ============================================================================
 *
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * No machine-oriented finite limits are encoded here.
 *
 * There is intentionally no:
 *
 *     MAX_ROWS
 *     MAX_COLUMNS
 *     MAX_FEATURES
 *     MAX_LABELS
 *     MAX_SHARDS
 *     MAX_PARTITIONS
 *     MAX_BATCH_SIZE
 *     MAX_WORKERS
 *     MAX_DATASETS
 *     MAX_STREAMS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_RECORDS
 *     MAX_SEQUENCE_LENGTH
 *
 * Repetition is structural.
 *
 * Dimensions, counts, sizes, partitions, batches, windows, workers, and
 * distributed resources are represented by expressions or semantic resource
 * descriptions.
 *
 * Actual limits belong to:
 *
 *     - resource management;
 *     - semantic validation;
 *     - compiler policy;
 *     - scheduling;
 *     - deployment;
 *     - runtime capabilities;
 *     - available resources.
 *
 * ============================================================================
 *
 * TYPE CONTRACT
 * ============================================================================
 *
 * Canonical type syntax belongs to the shared type grammar.
 *
 * This file reuses:
 *
 *     typeExpression
 *
 * It does NOT create:
 *
 *     TensorType
 *     DatasetType
 *     FeatureType
 *     LabelType
 *
 * as competing type systems.
 *
 * A dataset field may therefore use any canonical Zamani type whose semantic
 * meaning is valid for dataset data.
 *
 * ============================================================================
 *
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * General expression syntax belongs to the canonical expression grammar.
 *
 * This file reuses:
 *
 *     expression
 *
 * for:
 *
 *     - source expressions;
 *     - dimensions;
 *     - batch sizes;
 *     - window sizes;
 *     - sampling parameters;
 *     - predicates;
 *     - ordering expressions;
 *     - keys;
 *     - transformation arguments;
 *     - symbolic values;
 *     - compile-time values;
 *     - runtime values where permitted.
 *
 * ============================================================================
 *
 * SCHEMA CONTRACT
 * ============================================================================
 *
 * Dataset-local field declarations are a syntactic boundary only.
 *
 * The canonical data/schema subsystem remains responsible for:
 *
 *     - schema identity;
 *     - schema compatibility;
 *     - schema evolution;
 *     - field identity;
 *     - nullability;
 *     - constraints;
 *     - serialization;
 *     - validation;
 *     - schema versioning.
 *
 * This grammar must not become a second data-schema system.
 *
 * ============================================================================
 *
 * STORAGE CONTRACT
 * ============================================================================
 *
 * Dataset sources are expressed as semantic source specifications.
 *
 * The grammar does not define:
 *
 *     CSV;
 *     Parquet;
 *     Arrow;
 *     SQL;
 *     object storage;
 *     local filesystem;
 *     HTTP;
 *     database;
 *     message broker;
 *     device memory;
 *
 * as hard-coded storage implementations.
 *
 * Those are resolved through semantic source providers / interoperability /
 * data / networking layers.
 *
 * ============================================================================
 *
 * DISTRIBUTED CONTRACT
 * ============================================================================
 *
 * Dataset syntax may express:
 *
 *     partition intent;
 *     sharding intent;
 *     replication preference;
 *     locality preference;
 *     ordering requirements;
 *     consistency requirements.
 *
 * It must not prescribe:
 *
 *     - number of workers;
 *     - number of nodes;
 *     - node identifiers;
 *     - cluster topology;
 *     - network topology.
 *
 * Those belong to distributed/resource/deployment semantics.
 *
 * ============================================================================
 *
 * STREAMING CONTRACT
 * ============================================================================
 *
 * Streaming is represented as semantic intent.
 *
 * It does not imply:
 *
 *     - a particular transport;
 *     - a particular queue;
 *     - a particular broker;
 *     - a particular network;
 *     - a particular buffering implementation.
 *
 * ============================================================================
 *
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Dataset operations that are inherently order-sensitive must preserve their
 * source-level intent.
 *
 * Reproducibility metadata may be expressed, but actual reproducibility is
 * validated downstream.
 *
 * A grammar-level construct must never silently introduce an implementation
 * dependent random seed or ordering.
 *
 * ============================================================================
 *
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * This grammar is parser-composable.
 *
 * Its stable public integration boundary is:
 *
 *     datasetConstruct
 *
 * AI.g4 should import this grammar and delegate its dataset boundary to:
 *
 *     datasetConstruct
 *
 * AI.g4 MUST NOT reproduce these productions.
 *
 * datasets.g4 MUST NOT import AI.g4.
 *
 * This prevents:
 *
 *     AI -> Datasets -> AI
 *
 * circular grammar dependencies.
 *
 * ============================================================================
 */

parser grammar AIDatasets;

options {
    tokenVocab = ZamaniLexer;
}

import Types, Expressions;


/* ============================================================================
 * 1. PUBLIC DATASET ENTRY POINT
 * ========================================================================== */

/**
 * Stable parser boundary for dataset-domain syntax.
 *
 * AI.g4 and other domain grammars should depend on this rule rather than
 * duplicating dataset syntax.
 */
datasetConstruct
    : datasetDeclaration
    | datasetReference
    | datasetExpressionStatement
    | datasetAssignment
    | datasetSchemaExpression
    | datasetTransformExpression
    | datasetSplitExpression
    | datasetPartitionExpression
    | datasetBatchExpression
    | datasetWindowExpression
    | datasetSamplingExpression
    | datasetCompositionExpression
    | datasetExecutionRegion
    ;


/* ============================================================================
 * 2. DATASET DECLARATIONS
 * ========================================================================== */

/**
 * Dataset declaration.
 *
 * Examples:
 *
 *     @dataset training = source;
 *
 *     @dataset training {
 *         schema {
 *             input: Tensor<f32>;
 *             label: int;
 *         }
 *     }
 *
 *     @dataset training: SomeDatasetType = source;
 *
 * The semantic layer validates that the annotation is actually `@dataset`.
 */
datasetDeclaration
    : datasetAnnotation
      identifier
      datasetTypeAnnotation?
      datasetInitializer?
      datasetBody?
      SEMICOLON?
    ;


datasetAnnotation
    : NANO_ANNOTATION
    ;


datasetTypeAnnotation
    : COLON
      typeExpression
    ;


datasetInitializer
    : ASSIGN
      expression
    ;


/**
 * Dataset declaration body.
 *
 * The body is intentionally extensible.
 */
datasetBody
    : LBRACE
      datasetMember*
      RBRACE
    ;


datasetMember
    : datasetSchemaMember
    | datasetFeatureMember
    | datasetLabelMember
    | datasetTargetMember
    | datasetSplitMember
    | datasetPartitionMember
    | datasetSourceMember
    | datasetTransformMember
    | datasetMetadataMember
    | datasetRequirementMember
    | datasetCapabilityMember
    | datasetConstraintMember
    | datasetPreferenceMember
    | datasetProvenanceMember
    | datasetGenericMember
    ;


/* ============================================================================
 * 3. GENERIC DATASET MEMBERS
 * ========================================================================== */

/**
 * Generic extension point.
 *
 * This allows future dataset dialects to introduce metadata without changing
 * the lexical grammar.
 *
 * Semantic validation is responsible for recognizing known member names.
 */
datasetGenericMember
    : NANO_ANNOTATION
      identifier
      datasetMemberPayload?
      SEMICOLON
    ;


datasetMemberPayload
    : COLON
      expression
    | ASSIGN
      expression
    ;


/* ============================================================================
 * 4. SOURCE
 * ========================================================================== */

/**
 * Dataset source declaration.
 *
 * Source identity is semantic rather than storage-specific.
 *
 * Examples:
 *
 *     @source source = expression;
 *
 *     @source source: SourceType = expression;
 */
datasetSourceMember
    : NANO_ANNOTATION
      identifier
      datasetSourceType?
      ASSIGN
      expression
      SEMICOLON
    ;


datasetSourceType
    : COLON
      typeExpression
    ;


/**
 * Standalone dataset source expression.
 */
datasetSourceExpression
    : expression
    ;


/* ============================================================================
 * 5. DATASET REFERENCES
 * ========================================================================== */

datasetReference
    : identifier
    ;


datasetExpressionStatement
    : datasetExpression
      SEMICOLON
    ;


datasetExpression
    : datasetReference
    | datasetSourceExpression
    | datasetTransformExpression
    | datasetSplitExpression
    | datasetPartitionExpression
    | datasetBatchExpression
    | datasetWindowExpression
    | datasetSamplingExpression
    | datasetCompositionExpression
    | datasetSchemaExpression
    | datasetExecutionExpression
    | datasetExpressionInParentheses
    ;


/* ============================================================================
 * 6. ASSIGNMENT
 * ========================================================================== */

datasetAssignment
    : datasetAssignableTarget
      ASSIGN
      datasetExpression
      SEMICOLON
    ;


datasetAssignableTarget
    : identifier
    | datasetFieldReference
    ;


/* ============================================================================
 * 7. SCHEMA
 * ========================================================================== */

/**
 * Dataset-local schema boundary.
 *
 * This is NOT the canonical global data-schema system.
 */
datasetSchemaMember
    : NANO_ANNOTATION
      datasetSchemaKeyword
      datasetSchemaBody
      SEMICOLON?
    ;


datasetSchemaKeyword
    : identifier
    ;


datasetSchemaExpression
    : datasetSchemaReference
    | datasetInlineSchema
    ;


datasetSchemaReference
    : identifier
    ;


datasetInlineSchema
    : LBRACE
      datasetFieldDeclaration*
      RBRACE
    ;


datasetFieldDeclaration
    : identifier
      COLON
      typeExpression
      datasetFieldInitializer?
      SEMICOLON
    ;


datasetFieldInitializer
    : ASSIGN
      expression
    ;


/* ============================================================================
 * 8. FEATURES
 * ========================================================================== */

datasetFeatureMember
    : NANO_ANNOTATION
      identifier
      COLON
      datasetFieldReferenceList
      SEMICOLON
    ;


datasetFeatureExpression
    : datasetFieldReferenceList
    ;


datasetFieldReferenceList
    : datasetFieldReference
      (COMMA datasetFieldReference)*
      COMMA?
    ;


datasetFieldReference
    : identifier
    ;


/* ============================================================================
 * 9. LABELS
 * ========================================================================== */

datasetLabelMember
    : NANO_ANNOTATION
      identifier
      COLON
      datasetFieldReferenceList
      SEMICOLON
    ;


datasetLabelExpression
    : datasetFieldReferenceList
    ;


/* ============================================================================
 * 10. TARGETS
 * ========================================================================== */

datasetTargetMember
    : NANO_ANNOTATION
      identifier
      COLON
      datasetFieldReferenceList
      SEMICOLON
    ;


datasetTargetExpression
    : datasetFieldReferenceList
    ;


/* ============================================================================
 * 11. SPLITS
 * ========================================================================== */

/**
 * Dataset split intent.
 *
 * Counts and proportions remain expressions.
 *
 * No fixed number of splits is imposed.
 */
datasetSplitMember
    : NANO_ANNOTATION
      identifier
      datasetSplitSpecification
      SEMICOLON
    ;


datasetSplitExpression
    : datasetSplitSpecification
    ;


datasetSplitSpecification
    : datasetSplitByName
    | datasetSplitByExpression
    | datasetSplitComposition
    ;


datasetSplitByName
    : identifier
    ;


datasetSplitByExpression
    : expression
    ;


datasetSplitComposition
    : LBRACE
      datasetSplitEntry*
      RBRACE
    ;


datasetSplitEntry
    : identifier
      COLON
      expression
      COMMA?
    ;


/* ============================================================================
 * 12. PARTITIONS
 * ========================================================================== */

/**
 * Partition intent.
 *
 * No partition count or worker count is hard-coded.
 */
datasetPartitionMember
    : NANO_ANNOTATION
      identifier
      datasetPartitionSpecification
      SEMICOLON
    ;


datasetPartitionExpression
    : datasetPartitionSpecification
    ;


datasetPartitionSpecification
    : datasetPartitionByExpression
    | datasetPartitionByKey
    | datasetPartitionByRange
    | datasetPartitionByHash
    | datasetPartitionGeneric
    ;


datasetPartitionByExpression
    : expression
    ;


datasetPartitionByKey
    : datasetFieldReferenceList
    ;


datasetPartitionByRange
    : expression
    ;


datasetPartitionByHash
    : expression
    ;


datasetPartitionGeneric
    : identifier
      LPAREN
      datasetArgumentList?
      RPAREN
    ;


/* ============================================================================
 * 13. TRANSFORMATIONS
 * ========================================================================== */

/**
 * Dataset transformations are semantic operations.
 *
 * The operation vocabulary is intentionally open.
 *
 * Examples:
 *
 *     map(...)
 *     filter(...)
 *     project(...)
 *     normalize(...)
 *     tokenize(...)
 *     augment(...)
 *     encode(...)
 *     decode(...)
 *
 * Semantic analysis determines the operation.
 */
datasetTransformMember
    : NANO_ANNOTATION
      identifier
      ASSIGN
      datasetTransformExpression
      SEMICOLON
    ;


datasetTransformExpression
    : datasetUnaryTransform
    | datasetBinaryTransform
    | datasetPipelineTransform
    | datasetGenericTransform
    ;


datasetUnaryTransform
    : identifier
      LPAREN
      datasetExpressionArgument
      RPAREN
    ;


datasetBinaryTransform
    : identifier
      LPAREN
      datasetExpressionArgument
      COMMA
      datasetExpressionArgument
      COMMA?
      RPAREN
    ;


datasetPipelineTransform
    : datasetExpression
      PIPE
      datasetTransformExpression
    ;


datasetGenericTransform
    : identifier
      LPAREN
      datasetArgumentList?
      RPAREN
    ;


datasetExpressionArgument
    : expression
    | datasetExpression
    ;


datasetArgumentList
    : datasetArgument
      (COMMA datasetArgument)*
      COMMA?
    ;


datasetArgument
    : expression
    | datasetExpression
    ;


/* ============================================================================
 * 14. BATCHING
 * ========================================================================== */

/**
 * Batch semantics.
 *
 * Batch size is an expression rather than a literal constant.
 */
datasetBatchExpression
    : identifier
      LPAREN
      datasetExpressionArgument
      COMMA
      expression
      (COMMA datasetArgumentList)?
      RPAREN
    ;


/* ============================================================================
 * 15. WINDOWING
 * ========================================================================== */

/**
 * Windowing supports finite or runtime-derived values.
 */
datasetWindowExpression
    : identifier
      LPAREN
      datasetExpressionArgument
      COMMA
      expression
      (COMMA expression)*
      COMMA?
      RPAREN
    ;


/* ============================================================================
 * 16. SAMPLING
 * ========================================================================== */

/**
 * Sampling is intentionally open-ended.
 *
 * Examples:
 *
 *     sample(dataset, count)
 *     sample(dataset, probability)
 *     sample(dataset, expression)
 *
 * The semantic layer determines the interpretation.
 */
datasetSamplingExpression
    : identifier
      LPAREN
      datasetArgumentList?
      RPAREN
    ;


/* ============================================================================
 * 17. COMPOSITION
 * ========================================================================== */

/**
 * Dataset composition supports semantic composition without fixing a finite
 * number of inputs.
 */
datasetCompositionExpression
    : datasetConcatExpression
    | datasetJoinExpression
    | datasetZipExpression
    | datasetMergeExpression
    | datasetGenericComposition
    ;


datasetConcatExpression
    : identifier
      LPAREN
      datasetArgumentList?
      RPAREN
    ;


datasetJoinExpression
    : identifier
      LPAREN
      datasetArgumentList?
      RPAREN
    ;


datasetZipExpression
    : identifier
      LPAREN
      datasetArgumentList?
      RPAREN
    ;


datasetMergeExpression
    : identifier
      LPAREN
      datasetArgumentList?
      RPAREN
    ;


datasetGenericComposition
    : identifier
      LPAREN
      datasetArgumentList?
      RPAREN
    ;


/* ============================================================================
 * 18. METADATA
 * ========================================================================== */

datasetMetadataMember
    : NANO_ANNOTATION
      identifier
      COLON
      datasetMetadataValue
      SEMICOLON
    ;


datasetMetadataValue
    : expression
    | datasetMetadataObject
    | datasetMetadataList
    ;


datasetMetadataObject
    : LBRACE
      datasetMetadataEntry*
      RBRACE
    ;


datasetMetadataEntry
    : identifier
      COLON
      expression
      COMMA?
    ;


datasetMetadataList
    : LBRACKET
      datasetMetadataListElement*
      RBRACKET
    ;


datasetMetadataListElement
    : expression
      COMMA?
    ;


/* ============================================================================
 * 19. REQUIREMENTS
 * ========================================================================== */

/**
 * Dataset requirements express semantic requirements, not physical machines.
 *
 * Examples:
 *
 *     @requires deterministic;
 *     @requires streaming;
 *     @requires random_access;
 */
datasetRequirementMember
    : REQUIRES
      datasetRequirementList
      SEMICOLON
    ;


datasetRequirementList
    : datasetRequirement
      (COMMA datasetRequirement)*
      COMMA?
    ;


datasetRequirement
    : identifier
    | expression
    ;


/* ============================================================================
 * 20. CAPABILITIES
 * ========================================================================== */

datasetCapabilityMember
    : NANO_ANNOTATION
      identifier
      COLON
      datasetCapabilityValue
      SEMICOLON
    ;


datasetCapabilityValue
    : expression
    ;


/* ============================================================================
 * 21. CONSTRAINTS
 * ========================================================================== */

/**
 * Constraints describe semantic restrictions.
 *
 * They do not select hardware.
 */
datasetConstraintMember
    : NANO_ANNOTATION
      identifier
      COLON
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 22. PREFERENCES
 * ========================================================================== */

datasetPreferenceMember
    : NANO_ANNOTATION
      identifier
      COLON
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 23. PROVENANCE
 * ========================================================================== */

/**
 * Provenance is intentionally semantic.
 *
 * Examples:
 *
 *     @provenance source;
 *     @provenance expression;
 *     @provenance metadata;
 */
datasetProvenanceMember
    : NANO_ANNOTATION
      identifier
      COLON
      datasetProvenanceValue
      SEMICOLON
    ;


datasetProvenanceValue
    : expression
    | datasetMetadataObject
    ;


/* ============================================================================
 * 24. EXECUTION REGION
 * ========================================================================== */

/**
 * Dataset execution region.
 *
 * This expresses a semantic boundary only.
 *
 * It does not schedule or dispatch work.
 */
datasetExecutionRegion
    : NANO_ANNOTATION
      identifier
      LBRACE
      datasetExecutionMember*
      RBRACE
    ;


datasetExecutionMember
    : datasetMember
    | datasetExpressionStatement
    | datasetAssignment
    ;


/* ============================================================================
 * 25. EXECUTION EXPRESSION
 * ========================================================================== */

datasetExecutionExpression
    : identifier
      LPAREN
      datasetArgumentList?
      RPAREN
    ;


/* ============================================================================
 * 26. PARENTHESES
 * ========================================================================== */

datasetExpressionInParentheses
    : LPAREN
      datasetExpression
      RPAREN
    ;


/* ============================================================================
 * 27. INLINE DATASET VALUE
 * ========================================================================== */

/**
 * Inline dataset values.
 *
 * This is intentionally recursive and therefore has no fixed record count or
 * nesting depth.
 */
datasetInlineValue
    : datasetInlineRecord
    | datasetInlineSequence
    | expression
    ;


datasetInlineRecord
    : LBRACE
      datasetInlineRecordEntry*
      RBRACE
    ;


datasetInlineRecordEntry
    : identifier
      COLON
      datasetInlineValue
      COMMA?
    ;


datasetInlineSequence
    : LBRACKET
      datasetInlineSequenceElement*
      RBRACKET
    ;


datasetInlineSequenceElement
    : datasetInlineValue
      COMMA?
    ;


/* ============================================================================
 * 28. DATASET PIPELINE
 * ========================================================================== */

/**
 * A pipeline can contain an arbitrary number of stages.
 *
 * No fixed stage count is encoded.
 */
datasetPipelineExpression
    : datasetPipelineStage
      (PIPE datasetPipelineStage)*
    ;


datasetPipelineStage
    : datasetExpression
    | datasetTransformExpression
    | datasetBatchExpression
    | datasetWindowExpression
    | datasetSamplingExpression
    | datasetCompositionExpression
    ;


/* ============================================================================
 * 29. DATASET ARGUMENTS
 * ========================================================================== */

/**
 * A dataset argument can carry ordinary expressions or dataset-domain
 * expressions.
 */
datasetArgumentExpression
    : expression
    | datasetExpression
    | datasetInlineValue
    ;


/* ============================================================================
 * 30. ERROR-RECOVERY BOUNDARY
 * ========================================================================== */

/**
 * Deliberately no custom parser actions or recovery code are embedded here.
 *
 * Error reporting is owned by the canonical parser/frontend diagnostic layer.
 */


/* ============================================================================
 * END OF DATASET GRAMMAR
 * ============================================================================
 */
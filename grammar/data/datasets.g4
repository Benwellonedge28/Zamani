/*

* ============================================================================
* Zamani Programming Language
* ============================================================================
* 
* File:
* grammar/data/datasets.g4
* 
* Grammar:
* ZamaniDataDatasets
* 
* Status:
* CANONICAL GENERAL DATASET-DOMAIN PARSER GRAMMAR
* 
* Language/runtime baseline:
* Rust 1.97 / Rust 1.97.1
* Rust edition 2021
* Safe Rust only
* 
* ============================================================================
* PURPOSE
* ============================================================================
* 
* This file defines the canonical SOURCE-LEVEL SYNTAX for logical datasets.
* 
* A dataset is a portable logical data abstraction. It may represent:
* 
* - finite data;
* - streaming data;
* - generated data;
* - persistent data;
* - distributed data;
* - tensor-valued data;
* - classical data;
* - quantum-derived classical data;
* - AI/ML training data;
* - scientific data;
* - simulation data;
* - sensor data;
* - event data;
* - future data representations.
* 
* The grammar describes DATA INTENT.
* 
* It does not describe a particular:
* 
* - filesystem;
* - database;
* - object store;
* - cloud provider;
* - network provider;
* - accelerator;
* - CPU;
* - GPU;
* - FPGA;
* - QPU;
* - storage device;
* - memory bank;
* - physical address;
* - execution node;
* - worker;
* - thread;
* - partition placement;
* - physical topology.
* 
* ============================================================================
* ARCHITECTURAL POSITION
* ============================================================================
* 
* Zamani source
*      |
*      v
* canonical lexer
*      |
*      v
* ZamaniParser
*      |
*      v
* ZamaniDataDatasets
*      |
*      v
* domain-neutral AST
*      |
*      v
* semantic analysis
*      |
*      +----------------------+----------------------+
*      |                      |                      |
*      v                      v                      v
* data semantics       resource/capability       provenance
*      |                      |                      |
*      +----------------------+----------------------+
*                             |
*                             v
*                   canonical semantic model
*                             |
*                             v
*                        canonical IR
*                             |
*            +----------------+----------------+
*            |                |                |
*            v                v                v
*        classical          AI/ML         distributed
*        lowering          lowering       lowering
*            |                |                |
*            +----------------+----------------+
*                             |
*                             v
*                        optimization
*                             |
*                        scheduling
*                             |
*                        execution
*                             |
*                storage/network/compute targets
* 
* This grammar MUST NOT construct IR.
* 
* ============================================================================
* OWNERSHIP
* ============================================================================
* 
* THIS FILE OWNS:
* 
* - logical dataset declarations;
* - dataset references;
* - dataset source intent;
* - dataset schema references;
* - dataset field-role declarations;
* - dataset transformations as invocation boundaries;
* - dataset projections;
* - dataset filtering;
* - dataset partitioning intent;
* - dataset splitting intent;
* - dataset sampling intent;
* - dataset batching intent;
* - dataset windowing intent;
* - dataset ordering intent;
* - dataset shuffling intent;
* - dataset materialization intent;
* - dataset caching intent;
* - dataset validation intent;
* - dataset provenance/lineage metadata;
* - dataset resource/capability intent;
* - dataset interoperability metadata;
* - dataset-local metadata.
* 
* THIS FILE DOES NOT OWN:
* 
* - lexical tokens;
* - identifiers;
* - qualified names;
* - general expressions;
* - general types;
* - tensor types;
* - tensor operations;
* - record declarations;
* - collection declarations;
* - stream declarations;
* - general transformation semantics;
* - model semantics;
* - training algorithms;
* - inference algorithms;
* - automatic differentiation;
* - storage;
* - serialization implementation;
* - databases;
* - network protocols;
* - resource discovery;
* - hardware discovery;
* - scheduling;
* - routing;
* - optimization;
* - placement;
* - runtime execution;
* - canonical IR;
* - quantum::ir;
* - QEC;
* - ZQN;
* - HAL.
* 
* ============================================================================
* SINGLE DATASET AUTHORITY
* ============================================================================
* 
* This file is the canonical GENERAL dataset grammar.
* 
* "grammar/ai/datasets.g4" MUST NOT become a competing general dataset
* grammar. AI-specific dataset syntax should adapt this grammar through
* explicit adapter rules where necessary.
* 
* Likewise:
* 
* grammar/data/data.g4
* 
* MUST delegate dataset syntax to this grammar rather than defining a second
* dataset declaration hierarchy.
* 
* ============================================================================
* ANTLR INTEGRATION
* ============================================================================
* 
* This is a parser grammar.
* 
* The canonical lexical vocabulary is:
* 
* ZamaniLexer
* 
* Shared parser dependencies are:
* 
* Core
* Types
* Expressions
* 
* No lexer rules are declared here.
* 
* No Rust actions are declared here.
* 
* No semantic predicates are declared here.
* 
* ============================================================================
* POCO-REAF
* ============================================================================
* 
* Dataset syntax is target-independent.
* 
* A dataset may express:
* 
* requires capability(...)
* requires memory >= ...
* requires ...
* 
* without selecting:
* 
* GPU 0
* CPU 7
* node 4
* device 2
* storage bank 1
* provider X
* 
* Physical realization is a downstream concern.
* 
* This supports:
* 
* Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
* 
* subject to actual program semantics, implementation capabilities, and
* resources available at realization time.
* 
* ============================================================================
* SCALABILITY
* ============================================================================
* 
* There is NO language-level maximum for:
* 
* datasets;
* records;
* fields;
* samples;
* features;
* labels;
* dimensions;
* partitions;
* shards;
* batches;
* windows;
* transformations;
* pipeline stages;
* sources;
* sinks;
* splits;
* metadata entries;
* lineage entries;
* provenance entries;
* dataset nesting;
* dataset references.
* 
* Repetition is represented by normal ANTLR repetition operators.
* 
* Practical resource limits belong to:
* 
* parser resource policies;
* compiler resources;
* semantic resource analysis;
* runtime resources;
* target capabilities;
* deployment constraints.
* 
* Such limits MUST NOT become language semantics.
* 
* ============================================================================
* HARD-CODING PROHIBITION
* ============================================================================
* 
* This grammar MUST NOT define:
* 
* MAX_DATASET_SIZE
* MAX_DATASETS
* MAX_RECORDS
* MAX_ROWS
* MAX_COLUMNS
* MAX_FEATURES
* MAX_LABELS
* MAX_SHARDS
* MAX_PARTITIONS
* MAX_BATCH_SIZE
* MAX_WORKERS
* MAX_STREAMS
* MAX_NODES
* MAX_DEVICES
* MAX_TENSOR_RANK
* MAX_MEMORY
* MAX_STORAGE
* 
* Nor may it encode:
* 
* fixed CPU counts;
* fixed GPU counts;
* fixed FPGA counts;
* fixed QPU counts;
* fixed node counts;
* fixed worker counts;
* fixed device counts;
* fixed tensor ranks;
* fixed dimensions;
* fixed storage capacities;
* fixed memory capacities.
* 
* A value explicitly written by a programmer is program semantics, not a
* compiler-wide limit.
* 
* ============================================================================
* TYPE INTEGRATION
* ============================================================================
* 
* Dataset element/field types use the canonical:
* 
* typeExpression
* 
* Dataset syntax does NOT define:
* 
* DatasetType
* FeatureType
* LabelType
* TensorType
* 
* as competing type systems.
* 
* Consequently a dataset may semantically contain:
* 
* scalar values
* records
* collections
* tensors
* quantum measurement results
* domain-defined values
* user-defined types
* 
* according to the canonical type system.
* 
* ============================================================================
* EXPRESSION INTEGRATION
* ============================================================================
* 
* General expressions use:
* 
* expression
* 
* from the canonical expression grammar.
* 
* This permits:
* 
* symbolic values;
* runtime values;
* compile-time values;
* dimensions;
* predicates;
* keys;
* expressions;
* resource requirements;
* transformation arguments.
* 
* Dataset syntax does not create another expression hierarchy.
* 
* ============================================================================
* IDENTIFIER INTEGRATION
* ============================================================================
* 
* Names are supplied by:
* 
* identifier
* qualifiedName
* 
* from the canonical core grammar.
* 
* This file does not redefine identifier lexical structure.
* 
* ============================================================================
* EXTENSIBILITY MODEL
* ============================================================================
* 
* Dataset concepts that do not require language-level semantics should remain
* ordinary names or annotations.
* 
* Examples:
* 
* @dataset
* @source
* @schema
* @feature
* @label
* @target
* @split
* @partition
* @sample
* @batch
* @window
* @shuffle
* @cache
* @materialize
* @validate
* @provenance
* @lineage
* @requires
* @capability
* @constraint
* @prefer
* @hint
* 
* Their semantic meaning is validated downstream.
* 
* ============================================================================
  */

parser grammar ZamaniDataDatasets;

options {
tokenVocab = ZamaniLexer;
}

import
Core,
Types,
Expressions
;

/*

* ============================================================================
* 1. PUBLIC DATASET ENTRY POINT
* ============================================================================
* 
* This is the single public dataset-domain entry point.
* 
* It deliberately accepts only explicit dataset constructs.
* 
* A bare expression remains an ordinary Zamani expression.
  */
  datasetConstruct
  : datasetDeclaration
  | datasetRegion
  | datasetExpressionStatement
  ;

/*

* ============================================================================
* 2. DATASET DECLARATION
* ============================================================================
* 
* Canonical extensible form:
* 
* @dataset training = source;
* 
* @dataset training: DatasetSchema = source;
* 
* @dataset training {
*     ...
* }
* 
* The annotation identifies the semantic category while preserving the
* language's extensibility model.
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
: AT
identifier
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

/*

* ============================================================================
* 3. DATASET REGION
* ============================================================================
* 
* A region permits a named dataset scope with explicit members.
* 
* Example:
* 
* @dataset training {
*     @source input = source;
*     @feature x;
*     @label y;
* }
* 
* No physical storage or execution model is implied.
  /
  datasetRegion
  : datasetAnnotation
  identifier
  datasetTypeAnnotation?
  LBRACE
  datasetMember
  RBRACE
  SEMICOLON?
  ;

/*

* ============================================================================
* 4. DATASET MEMBERS
* ============================================================================
* 
* Members are deliberately explicit enough to provide structural validation
* while delegating actual semantics to later compiler stages.
  */
  datasetMember
  : datasetSource
  | datasetSchema
  | datasetField
  | datasetFeature
  | datasetLabel
  | datasetTarget
  | datasetSplit
  | datasetPartition
  | datasetTransform
  | datasetProject
  | datasetFilter
  | datasetJoin
  | datasetUnion
  | datasetSample
  | datasetBatch
  | datasetWindow
  | datasetOrder
  | datasetShuffle
  | datasetCache
  | datasetMaterialize
  | datasetValidate
  | datasetProvenance
  | datasetLineage
  | datasetRequirement
  | datasetCapability
  | datasetConstraint
  | datasetPreference
  | datasetHint
  | datasetInterop
  | datasetMetadata
  | datasetBinding
  ;

/*

* ============================================================================
* 5. GENERIC DATASET BINDING
* ============================================================================
* 
* This provides an extensible named dataset-local value without introducing
* a new type system.
  */
  datasetBinding
  : datasetAnnotation
  identifier
  datasetTypeAnnotation?
  datasetInitializer?
  SEMICOLON?
  ;

/*

* ============================================================================
* 6. SOURCE
* ============================================================================
* 
* A source is logical.
* 
* The expression may eventually resolve to:
* 
* local data
* generated data
* persistent data
* remote data
* streamed data
* another dataset
* a future data provider
* 
* No provider is hard-coded.
  */
  datasetSource
  : datasetAnnotation
  identifier
  datasetTypeAnnotation?
  ASSIGN
  expression
  SEMICOLON?
  ;

/*

* ============================================================================
* 7. SCHEMA REFERENCE
* ============================================================================
* 
* Schema identity remains semantic.
* 
* A schema may be supplied as:
* 
* a named type;
* an expression;
* a logical schema resource;
* a dialect-defined schema reference.

*/
datasetSchema
: datasetAnnotation
datasetSchemaPayload
SEMICOLON?
;

datasetSchemaPayload
: datasetSchemaReference
| datasetSchemaBody
;

datasetSchemaReference
: typeExpression
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
: datasetAnnotation
datasetMetadataPayload?
;

/*

* ============================================================================
* 8. FIELD ROLES
* ============================================================================
* 
* Feature/label/target roles are logical semantic classifications.
* 
* They do not constrain tensor rank, storage format, model architecture,
* memory size, or hardware.
  */
  datasetField
  : datasetAnnotation
  datasetFieldReferenceList
  SEMICOLON?
  ;

datasetFeature
: datasetAnnotation
datasetFieldReferenceList
SEMICOLON?
;

datasetLabel
: datasetAnnotation
datasetFieldReferenceList
SEMICOLON?
;

datasetTarget
: datasetAnnotation
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

/*

* ============================================================================
* 9. SPLITS
* ============================================================================
* 
* A split is a logical dataset view/subset.
* 
* It does not prescribe a fixed number of partitions or records.
  */
  datasetSplit
  : datasetAnnotation
  identifier
  ASSIGN
  datasetReferenceExpression
  datasetSelector?
  SEMICOLON?
  ;

datasetSelector
: LPAREN
datasetArgumentList?
RPAREN
;

datasetReferenceExpression
: expression
;

/*

* ============================================================================
* 10. PARTITIONING
* ============================================================================
* 
* Partitioning describes logical partitioning intent.
* 
* Physical partition count, shard placement, worker assignment, topology and
* storage layout remain downstream concerns.
  */
  datasetPartition
  : datasetAnnotation
  identifier?
  ASSIGN?
  datasetPartitionSpec
  SEMICOLON?
  ;

datasetPartitionSpec
: datasetReferenceExpression
datasetPartitionClause*
;

datasetPartitionClause
: datasetPartitionBy
| datasetPartitionStrategy
| datasetPartitionConstraint
| datasetPartitionPreference
;

datasetPartitionBy
: datasetAnnotation
datasetExpressionList
;

datasetPartitionStrategy
: datasetAnnotation
expression
;

datasetPartitionConstraint
: datasetAnnotation
expression
;

datasetPartitionPreference
: datasetAnnotation
expression
;

/*

* ============================================================================
* 11. TRANSFORMATIONS
* ============================================================================
* 
* These are invocation boundaries.
* 
* Transformation semantics remain owned by the data transformation subsystem.
* 
* Examples:
* 
* @transform normalized = map(data, f);
* @transform filtered = filter(data, predicate);
* @transform result = custom_operation(data);
* 
* No fixed transformation inventory is required here.
  */
  datasetTransform
  : datasetAnnotation
  identifier?
  ASSIGN
  datasetOperationExpression
  SEMICOLON?
  ;

datasetOperationExpression
: qualifiedName
LPAREN
datasetArgumentList?
RPAREN
;

datasetArgumentList
: datasetArgument
(COMMA datasetArgument)*
COMMA?
;

datasetArgument
: datasetNamedArgument
| expression
;

datasetNamedArgument
: identifier
COLON
expression
;

/*

* ============================================================================
* 12. PROJECTION
* ============================================================================
  */
  datasetProject
  : datasetAnnotation
  identifier?
  ASSIGN?
  datasetReferenceExpression
  datasetProjectClause
  SEMICOLON?
  ;

datasetProjectClause
: datasetAnnotation
datasetFieldReferenceList
;

/*

* ============================================================================
* 13. FILTER
* ============================================================================
  */
  datasetFilter
  : datasetAnnotation
  identifier?
  ASSIGN?
  datasetReferenceExpression
  datasetPredicateClause
  SEMICOLON?
  ;

datasetPredicateClause
: datasetAnnotation
expression
;

/*

* ============================================================================
* 14. JOIN
* ============================================================================
* 
* Join strategy is semantic.
* 
* Vendor/database-specific join syntax does not belong here.
  /
  datasetJoin
  : datasetAnnotation
  identifier?
  ASSIGN?
  datasetReferenceExpression
  datasetJoinOperator
  datasetReferenceExpression
  datasetJoinClause
  SEMICOLON?
  ;

datasetJoinOperator
: datasetAnnotation
;

datasetJoinClause
: datasetAnnotation
expression
;

/*

* ============================================================================
* 15. UNION / CONCATENATION
* ============================================================================
  */
  datasetUnion
  : datasetAnnotation
  identifier?
  ASSIGN?
  datasetReferenceList
  SEMICOLON?
  ;

datasetReferenceList
: datasetReferenceExpression
(COMMA datasetReferenceExpression)*
COMMA?
;

/*

* ============================================================================
* 16. SAMPLING
* ============================================================================
* 
* Sample size, probability, seed, stratification and replacement are program
* semantics when explicitly supplied.
* 
* They are not compiler-wide limits.
  /
  datasetSample
  : datasetAnnotation
  identifier?
  ASSIGN?
  datasetReferenceExpression
  datasetSampleClause
  SEMICOLON?
  ;

datasetSampleClause
: datasetAnnotation
expression
;

/*

* ============================================================================
* 17. BATCHING
* ============================================================================
  /
  datasetBatch
  : datasetAnnotation
  identifier?
  ASSIGN?
  datasetReferenceExpression
  datasetBatchClause
  SEMICOLON?
  ;

datasetBatchClause
: datasetAnnotation
expression
;

/*

* ============================================================================
* 18. WINDOWS
* ============================================================================
  /
  datasetWindow
  : datasetAnnotation
  identifier?
  ASSIGN?
  datasetReferenceExpression
  datasetWindowClause
  SEMICOLON?
  ;

datasetWindowClause
: datasetAnnotation
expression
;

/*

* ============================================================================
* 19. ORDERING
* ============================================================================
  /
  datasetOrder
  : datasetAnnotation
  identifier?
  ASSIGN?
  datasetReferenceExpression
  datasetOrderClause
  SEMICOLON?
  ;

datasetOrderClause
: datasetAnnotation
expression
;

/*

* ============================================================================
* 20. SHUFFLING
* ============================================================================
  /
  datasetShuffle
  : datasetAnnotation
  identifier?
  ASSIGN?
  datasetReferenceExpression
  datasetShuffleClause
  SEMICOLON?
  ;

datasetShuffleClause
: datasetAnnotation
expression
;

/*

* ============================================================================
* 21. CACHE
* ============================================================================
* 
* Cache semantics are logical.
* 
* The grammar does not select:
* 
* RAM
* VRAM
* disk
* cache level
* device
* storage provider.

/
datasetCache
: datasetAnnotation
identifier?
ASSIGN?
datasetReferenceExpression
datasetCacheClause
SEMICOLON?
;

datasetCacheClause
: datasetAnnotation
expression
;

/*

* ============================================================================
* 22. MATERIALIZATION
* ============================================================================
  /
  datasetMaterialize
  : datasetAnnotation
  identifier?
  ASSIGN?
  datasetReferenceExpression
  datasetMaterializeClause
  SEMICOLON?
  ;

datasetMaterializeClause
: datasetAnnotation
expression
;

/*

* ============================================================================
* 23. VALIDATION
* ============================================================================
  */
  datasetValidate
  : datasetAnnotation
  datasetValidationPayload
  SEMICOLON?
  ;

datasetValidationPayload
: expression
| LBRACE
datasetValidationMember*
RBRACE
;

datasetValidationMember
: datasetAnnotation
datasetMetadataPayload?
SEMICOLON?
;

/*

* ============================================================================
* 24. PROVENANCE
* ============================================================================
* 
* Provenance is metadata/semantic intent.
* 
* It does not implement history tracking.
  */
  datasetProvenance
  : datasetAnnotation
  datasetMetadataPayload?
  SEMICOLON?
  ;

/*

* ============================================================================
* 25. LINEAGE
* ============================================================================
  */
  datasetLineage
  : datasetAnnotation
  datasetLineagePayload
  SEMICOLON?
  ;

datasetLineagePayload
: datasetReferenceList
| expression
| LBRACE
datasetMetadataEntry*
RBRACE
;

/*

* ============================================================================
* 26. REQUIREMENTS
* ============================================================================
* 
* Requirements describe semantic/resource needs.
* 
* They do not select physical hardware.
  */
  datasetRequirement
  : datasetAnnotation
  datasetRequirementPayload
  SEMICOLON?
  ;

datasetRequirementPayload
: expression
| LPAREN
expression
RPAREN
;

/*

* ============================================================================
* 27. CAPABILITIES
* ============================================================================
  */
  datasetCapability
  : datasetAnnotation
  datasetCapabilityPayload
  SEMICOLON?
  ;

datasetCapabilityPayload
: expression
| LPAREN
expression
RPAREN
;

/*

* ============================================================================
* 28. CONSTRAINTS
* ============================================================================
  */
  datasetConstraint
  : datasetAnnotation
  expression
  SEMICOLON?
  ;

/*

* ============================================================================
* 29. PREFERENCES
* ============================================================================
  */
  datasetPreference
  : datasetAnnotation
  expression
  SEMICOLON?
  ;

/*

* ============================================================================
* 30. HINTS
* ============================================================================
  */
  datasetHint
  : datasetAnnotation
  expression
  SEMICOLON?
  ;

/*

* ============================================================================
* 31. INTEROPERABILITY
* ============================================================================
* 
* Interoperability is represented as semantic metadata.
* 
* Provider/format implementation belongs to interoperability backends.
  */
  datasetInterop
  : datasetAnnotation
  datasetInteropPayload
  SEMICOLON?
  ;

datasetInteropPayload
: expression
| LBRACE
datasetMetadataEntry*
RBRACE
;

/*

* ============================================================================
* 32. METADATA
* ============================================================================
* 
* Metadata remains extensible.
* 
* Future dataset properties therefore do not necessarily require a new lexer
* token or grammar keyword.
  */
  datasetMetadata
  : datasetAnnotation
  datasetMetadataPayload?
  SEMICOLON?
  ;

datasetMetadataPayload
: LPAREN
datasetArgumentList?
RPAREN
| COLON
expression
| ASSIGN
expression
| LBRACE
datasetMetadataEntry*
RBRACE
;

datasetMetadataEntry
: identifier
datasetMetadataValue?
SEMICOLON?
;

datasetMetadataValue
: COLON expression
| ASSIGN expression
;

/*

* ============================================================================
* 33. DATASET EXPRESSIONS
* ============================================================================
* 
* Dataset expressions remain ordinary Zamani expressions wherever possible.
* 
* This wrapper exists only where a data-domain consumer explicitly needs a
* stable dataset boundary.
  */
  datasetExpression
  : datasetReferenceExpression
  | datasetOperationExpression
  | datasetCollectionExpression
  ;

datasetCollectionExpression
: LBRACKET
datasetExpressionList?
RBRACKET
;

datasetExpressionList
: expression
(COMMA expression)*
COMMA?
;

datasetExpressionStatement
: datasetExpression
SEMICOLON
;

/*

* ============================================================================
* 34. DATASET-LOCAL MEMBER DISPATCH
* ============================================================================
* 
* This rule provides a stable extension point for future dataset constructs
* without allowing arbitrary expressions to become dataset members.
  */
  datasetExtensionMember
  : datasetAnnotation
  identifier
  datasetMetadataPayload?
  SEMICOLON?
  ;

/*

* ============================================================================
* 35. COMMON DATASET REFERENCE
* ============================================================================
* 
* A dataset reference is intentionally a normal expression.
* 
* The semantic layer determines whether the referenced value is a dataset.
  */
  datasetReference
  : qualifiedName
  ;

/*

* ============================================================================
* 36. RESOURCE-SAFE DATASET SEMANTICS
* ============================================================================
* 
* The following distinctions MUST remain semantic:
* 
* requirement
* capability
* constraint
* preference
* hint
* implementation decision
* 
* For example:
* 
* @requires capability("streaming");
* 
* is not equivalent to:
* 
* @use device(0);
* 
* The former expresses portable intent.
* 
* The latter, if supported by a downstream dialect, is a realization detail
* and must not become a universal dataset requirement.
* 
* ============================================================================
* 37. TENSOR INTEGRATION
* ============================================================================
* 
* Dataset fields may use the canonical type system:
* 
* Tensor<T>
* Tensor<T, Shape>
* 
* where supported by the type subsystem.
* 
* This file MUST NOT import:
* 
* grammar/classical/tensor.g4
* 
* merely to duplicate tensor syntax.
* 
* Tensor operations remain owned by:
* 
* grammar/classical/tensor.g4
* 
* and the canonical expression system.
* 
* Therefore dataset syntax can carry tensor-valued data without becoming a
* second tensor language.
* 
* ============================================================================
* 38. AI INTEGRATION
* ============================================================================
* 
* AI/ML may consume this grammar for:
* 
* training datasets
* validation datasets
* test datasets
* inference datasets
* streaming datasets
* feature datasets
* label datasets
* 
* AI-specific semantics remain outside this file.
* 
* "grammar/ai/datasets.g4" should adapt to this canonical grammar instead of
* redefining the complete dataset model.
* 
* ============================================================================
* 39. CLASSICAL INTEGRATION
* ============================================================================
* 
* Dataset values may feed:
* 
* classical computation
* numerical computation
* statistics
* signal processing
* tensor computation
* symbolic computation
* optimization
* 
* No classical execution strategy is selected here.
* 
* ============================================================================
* 40. QUANTUM / HYBRID INTEGRATION
* ============================================================================
* 
* Dataset values may contain or be derived from:
* 
* measurement results
* classical feed-forward values
* simulation results
* parameter sets
* calibration-independent logical data
* 
* Quantum semantics remain owned by the quantum subsystem and ultimately use
* the canonical:
* 
* quantum::ir
* 
* boundary.
* 
* This file creates no quantum IR.
* 
* ============================================================================
* 41. DISTRIBUTED INTEGRATION
* ============================================================================
* 
* A dataset may express logical:
* 
* partitioning
* replication
* distribution
* ordering
* consistency
* streaming
* 
* without specifying:
* 
* node count
* worker count
* topology
* physical placement
* network device
* storage device.
* 
* Distribution is resolved downstream.
* 
* ============================================================================
* 42. HARDWARE / ACCELERATOR INTEGRATION
* ============================================================================
* 
* Dataset syntax can carry requirements or capabilities such as:
* 
* @requires capability("tensor.compute");
* @requires capability("distributed.data");
* @requires capability("streaming");
* 
* It MUST NOT directly select:
* 
* CPU
* GPU
* FPGA
* ASIC
* accelerator
* QPU
* 
* Hardware realization belongs to resource analysis, compilation, scheduling,
* routing, HAL and runtime layers.
* 
* ============================================================================
* 43. SERIALIZATION / STORAGE INTEGRATION
* ============================================================================
* 
* Serialization and persistence are represented through logical metadata or
* interoperability expressions.
* 
* This grammar does not reserve:
* 
* CSV
* JSON
* Parquet
* Arrow
* SQL
* S3
* Kafka
* 
* as universal language semantics.
* 
* Such names may remain ordinary identifiers or dialect/interoperability
* constructs.
* 
* ============================================================================
* 44. AST CONTRACT
* ============================================================================
* 
* Every public rule maps through the repository's domain-neutral AST.
* 
* The intended mapping is:
* 
* datasetDeclaration
*     ->
* generic declaration / dataset declaration semantic node
* 
* datasetSource
*     ->
* data source intent node
* 
* datasetSchema
*     ->
* schema reference/body node
* 
* datasetTransform
*     ->
* generic operation/invocation node
* 
* datasetPartition
*     ->
* resource/data partition intent node
* 
* datasetRequirement
*     ->
* requirement node
* 
* datasetCapability
*     ->
* capability requirement node
* 
* datasetMetadata
*     ->
* metadata/attribute node
* 
* No dataset-specific AST implementation is created by this grammar.
* 
* ============================================================================
* 45. SEMANTIC CONTRACT
* ============================================================================
* 
* Semantic analysis is responsible for:
* 
* - determining whether @dataset denotes a dataset;
* - resolving names;
* - resolving dataset types;
* - validating schema compatibility;
* - validating field roles;
* - validating transformation types;
* - validating shape/type relationships;
* - validating resource requirements;
* - validating capabilities;
* - validating constraints;
* - validating provenance;
* - validating interoperability metadata;
* - detecting invalid provider-specific assumptions;
* - determining whether operations are legal.
* 
* The parser MUST NOT perform those decisions.
* 
* ============================================================================
* 46. IR CONTRACT
* ============================================================================
* 
* This grammar introduces NO dataset IR.
* 
* The required pipeline is:
* 
* dataset syntax
*     ->
* domain-neutral AST
*     ->
* semantic dataset model
*     ->
* existing canonical data/computation IR
* 
* The exact IR type is owned by the repository's canonical IR subsystem.
* 
* This file must therefore never introduce:
* 
* DatasetIR
* AIDatasetIR
* TensorDatasetIR
* DistributedDatasetIR
* 
* merely as parser artifacts.
* 
* ============================================================================
* 47. COMPILER CONTRACT
* ============================================================================
* 
* The compiler may lower the same dataset program differently depending on
* available resources and capabilities.
* 
* Possible realization strategies include:
* 
* in-memory
* streamed
* distributed
* persistent
* accelerator-backed
* fused
* lazy
* eager
* materialized
* generated
* 
* Such choices MUST NOT change the source grammar.
* 
* ============================================================================
* 48. RUNTIME CONTRACT
* ============================================================================
* 
* Runtime systems may determine:
* 
* actual storage;
* actual transport;
* actual partitioning;
* actual placement;
* actual buffering;
* actual scheduling;
* actual resource usage.
* 
* The grammar performs none of these actions.
* 
* ============================================================================
* 49. DETERMINISM CONTRACT
* ============================================================================
* 
* Parsing of identical source with identical grammar/version/configuration
* MUST produce identical:
* 
* token interpretation;
* parse structure;
* source spans;
* syntax diagnostics.
* 
* Parsing MUST NOT depend on:
* 
* CPU availability;
* GPU availability;
* QPU availability;
* filesystem state;
* network state;
* wall-clock time;
* randomness;
* runtime resource state;
* deployment topology.
* 
* ============================================================================
* 50. SECURITY CONTRACT
* ============================================================================
* 
* This grammar contains:
* 
* no Rust actions;
* no semantic predicates;
* no filesystem operations;
* no network operations;
* no process execution;
* no environment inspection;
* no hardware discovery;
* no secret access;
* no runtime execution.
* 
* It therefore remains a pure source-processing boundary.
* 
* ============================================================================
* 51. DIAGNOSTIC CONTRACT
* ============================================================================
* 
* Malformed dataset syntax must fail structurally rather than being silently
* converted into unrelated ordinary syntax.
* 
* Examples that must be rejected by the parser/semantic pipeline include:
* 
* @dataset
* 
* @dataset training {
* 
* @dataset training {
*     @feature
* }
* 
* @dataset training {
*     field:
* }
* 
* @dataset training {
*     field: Type
*     another: Type
* }
* 
* @dataset training {
*     @source input =
* }
* 
* Exact diagnostic wording belongs to the canonical diagnostics subsystem.
* 
* ============================================================================
* 52. POSITIVE TEST CONTRACT
* ============================================================================
* 
* The test suite must cover at minimum:
* 
* @dataset data = source;
* 
* @dataset data: SomeType = source;
* 
* @dataset data {
*     @source input = source;
* }
* 
* @dataset data {
*     @schema {
*         value: Tensor<f64>;
*         label: int;
*     }
* }
* 
* @dataset training {
*     @feature input;
*     @label target;
* }
* 
* @dataset training {
*     @split train = source;
*     @split validation = source;
*     @split test = source;
* }
* 
* @dataset training {
*     @partition shards = source;
* }
* 
* @dataset training {
*     @transform normalized = normalize(source);
* }
* 
* @dataset training {
*     @sample sampled = source;
* }
* 
* @dataset training {
*     @batch batched = source;
* }
* 
* @dataset training {
*     @window windows = source;
* }
* 
* @dataset training {
*     @requires capability("streaming");
* }
* 
* @dataset training {
*     @requires capability("tensor.compute");
* }
* 
* @dataset training {
*     @provenance {
*         origin: source;
*     }
* }
* 
* ============================================================================
* 53. NEGATIVE TEST CONTRACT
* ============================================================================
* 
* Test malformed:
* 
* missing dataset name;
* missing type;
* missing initializer;
* malformed schema;
* malformed field;
* malformed annotation;
* malformed argument;
* malformed partition expression;
* malformed metadata;
* malformed nested dataset body;
* missing delimiters;
* duplicated punctuation.
* 
* ============================================================================
* 54. BOUNDARY TEST CONTRACT
* ============================================================================
* 
* Test boundaries involving:
* 
* annotation + identifier;
* identifier + colon;
* type + assignment;
* nested braces;
* nested expressions;
* generic types;
* tensor-valued fields;
* symbolic dimensions;
* qualified names;
* trailing commas;
* empty dataset bodies;
* empty metadata bodies.
* 
* ============================================================================
* 55. SCALABILITY TEST CONTRACT
* ============================================================================
* 
* Generated tests must demonstrate correct parsing of arbitrarily large
* FINITE datasets subject only to test/compiler resources.
* 
* Test dimensions include:
* 
* many fields;
* many feature references;
* many labels;
* many transformations;
* many partitions;
* many splits;
* many metadata entries;
* deeply nested expressions;
* large logical pipeline descriptions;
* very large numeric values;
* symbolic resource values.
* 
* No generated test may define a language maximum.
* 
* Any test-environment limit must be documented as a test-resource limit only.
* 
* ============================================================================
* 56. COMPATIBILITY CONTRACT
* ============================================================================
* 
* The addition of this file must not require changing stable lexical token
* definitions merely to support future dataset concepts.
* 
* Dataset concepts should preferentially use:
* 
* AT + identifier
* 
* for extensible semantic annotations.
* 
* Existing parser users should receive the canonical:
* 
* datasetConstruct
* 
* entry point through the Data composition grammar.
* 
* AI consumers should adapt through:
* 
* grammar/ai/datasets.g4
* 
* rather than creating another universal dataset authority.
* 
* ============================================================================
* 57. HARD-CODING AUDIT
* ============================================================================
* 
* This grammar contains no universal:
* 
* CPU limit;
* GPU limit;
* FPGA limit;
* QPU limit;
* accelerator limit;
* node limit;
* device limit;
* memory limit;
* storage limit;
* tensor-rank limit;
* tensor-dimension limit;
* record limit;
* field limit;
* partition limit;
* shard limit;
* worker limit;
* dataset limit.
* 
* Numeric values remain ordinary expressions.
* 
* ============================================================================
* 58. RUST CONTRACT
* ============================================================================
* 
* This grammar contains no Rust code.
* 
* The consuming implementation must remain compatible with:
* 
* Rust 1.97
* Rust 1.97.1
* Rust 2021
* 
* and safe Rust only.
* 
* No "unsafe" implementation is required or implied by this grammar.
* 
* ============================================================================
* 59. REQUIRED REPOSITORY INTEGRATION
* ============================================================================
* 
* The following integration must be performed as a separate repository
* composition change:
* 
* grammar/data/data.g4
* 
* must expose:
* 
* datasetConstruct
* 
* through its data dispatcher.
* 
* The preferred relationship is:
* 
* ZamaniParser
*      |
*      +--> Data
*               |
*               +--> ZamaniDataDatasets
* 
* and NOT:
* 
* ZamaniParser
*      |
*      +--> Data
*      +--> Dataset
* 
* as competing top-level dataset authorities.
* 
* "grammar/ai/datasets.g4" should subsequently replace duplicated general
* dataset structures with a thin adapter to this canonical grammar.
* 
* ============================================================================
* 60. INTEGRATION RULE FOR DATA.G4
* ============================================================================
* 
* "grammar/data/data.g4" should eventually:
* 
* import ZamaniDataDatasets;
* 
* and add exactly one dataset-domain entry to its dispatcher:
* 
* | datasetConstruct
* 
* It must then remove or delegate any competing dataset declaration rules.
* 
* The rest of data.g4 remains responsible for the data constructs it
* legitimately owns.
* 
* ============================================================================
* 61. INTEGRATION RULE FOR AI/DATASETS.G4
* ============================================================================
* 
* "grammar/ai/datasets.g4" currently contains a broad AI/ML dataset grammar.
* 
* It should be migrated toward:
* 
* parser grammar AIDatasets;
* 
* options {
*     tokenVocab = ZamaniLexer;
* }
* 
* import
*     ZamaniDataDatasets;
* 
* and expose an AI adapter such as:
* 
* aiDatasetConstruct
*     : datasetConstruct
*     ;
* 
* AI-specific training semantics remain in:
* 
* grammar/ai/training.g4
* 
* Model semantics remain in:
* 
* grammar/ai/model.g4
* 
* Inference remains in:
* 
* grammar/ai/inference.g4
* 
* This prevents AI from creating a second dataset language.
* 
* ============================================================================
* 62. INTEGRATION WITH COLLECTIONS
* ============================================================================
* 
* Dataset collection semantics may consume:
* 
* grammar/data/collections.g4
* 
* but this file must not duplicate collection syntax.
* 
* Dataset values may semantically be collection values.
* 
* ============================================================================
* 63. INTEGRATION WITH STREAMS
* ============================================================================
* 
* Streaming datasets may consume:
* 
* grammar/data/streams.g4
* 
* but this file must not duplicate stream syntax.
* 
* A stream may semantically provide dataset values, and a dataset may
* semantically be evaluated as a stream.
* 
* The distinction is semantic rather than a reason to create competing
* grammars.
* 
* ============================================================================
* 64. INTEGRATION WITH TRANSFORMATIONS
* ============================================================================
* 
* General transformation syntax belongs to:
* 
* grammar/data/transformations.g4
* 
* Dataset transformation members therefore use operation/invocation
* boundaries rather than reproducing the full transformation language.
* 
* ============================================================================
* 65. INTEGRATION WITH SCHEMAS / RECORDS
* ============================================================================
* 
* Schema and record declarations remain owned by their canonical grammar
* components.
* 
* Dataset schema members may refer to those canonical types and declarations.
* 
* This file must not create a second record or schema type system.
* 
* ============================================================================
* 66. INTEGRATION WITH TENSORS
* ============================================================================
* 
* Tensor syntax remains owned by:
* 
* grammar/classical/tensor.g4
* 
* Dataset fields can carry tensor types through:
* 
* typeExpression
* 
* No tensor-specific AST or IR is introduced here.
* 
* ============================================================================
* 67. INTEGRATION WITH QUANTUM
* ============================================================================
* 
* Dataset syntax may represent data generated by quantum programs, including
* measurement results.
* 
* The dataset grammar has no dependency on:
* 
* quantum::ir
* 
* and must not introduce a quantum IR.
* 
* The semantic pipeline remains:
* 
* dataset / quantum source
*      ->
* domain-neutral AST
*      ->
* semantic analysis
*      ->
* quantum::ir where quantum semantics apply
*      ->
* downstream realization.
* 
* ============================================================================
* 68. INTEGRATION WITH HARDWARE
* ============================================================================
* 
* Dataset requirements may be consumed by:
* 
* resources
* hardware
* compile
* execution
* 
* but this grammar performs no resource discovery.
* 
* ============================================================================
* 69. INTEGRATION WITH DISTRIBUTED COMPUTING
* ============================================================================
* 
* Dataset partition/distribution intent may be lowered to distributed
* execution.
* 
* The source grammar remains independent of:
* 
* node count;
* worker count;
* topology;
* network hardware;
* storage topology.
* 
* ============================================================================
* 70. COMPLETION CRITERIA
* ============================================================================
* 
* This file is complete when:
* 
* [x] It exists as the canonical general dataset grammar.
* 
* [x] Its filename is retained as "datasets.g4".
* 
* [x] It uses the canonical ZamaniLexer vocabulary.
* 
* [x] It reuses Core, Types and Expressions.
* 
* [x] It does not define lexer rules.
* 
* [x] It does not define a competing expression grammar.
* 
* [x] It does not define a competing type grammar.
* 
* [x] It does not define tensor syntax.
* 
* [x] It does not define collection syntax.
* 
* [x] It does not define stream syntax.
* 
* [x] It does not define record syntax.
* 
* [x] It does not define storage implementation.
* 
* [x] It does not define database implementation.
* 
* [x] It does not define network implementation.
* 
* [x] It does not define hardware implementation.
* 
* [x] It does not define resource discovery.
* 
* [x] It does not define scheduling.
* 
* [x] It does not define routing.
* 
* [x] It does not define optimization.
* 
* [x] It does not define runtime execution.
* 
* [x] It does not define canonical IR.
* 
* [x] It does not define quantum::ir.
* 
* [x] It contains no unsafe Rust.
* 
* [x] It contains no embedded Rust actions.
* 
* [x] It contains no semantic predicates.
* 
* [x] It contains no fixed hardware capacities.
* 
* [x] It contains no fixed dataset cardinality.
* 
* [x] It contains no fixed tensor rank.
* 
* [x] It supports extensible dataset annotations.
* 
* [x] It supports tensor-valued dataset fields through the canonical type
* system.
* 
* [x] It supports classical, AI, distributed, streaming and hybrid consumers.
* 
* [x] It defines a stable "datasetConstruct" integration boundary.
* 
* [x] Its downstream AST contract is specified.
* 
* [x] Its semantic contract is specified.
* 
* [x] Its IR contract is specified.
* 
* [x] Its compiler/runtime boundaries are specified.
* 
* [x] Its compatibility boundary is specified.
* 
* [x] Its scalability contract is specified.
* 
* [x] Its determinism contract is specified.
* 
* [x] Its hard-coding audit is specified.
* 
* ============================================================================
* FINAL ARCHITECTURAL INVARIANT
* ============================================================================
* 
* A dataset is a logical computation/data abstraction.
* 
* It is NOT a machine.
* 
* It is NOT a storage device.
* 
* It is NOT a database vendor.
* 
* It is NOT a GPU buffer.
* 
* It is NOT a cluster.
* 
* It is NOT a fixed-size tensor.
* 
* It is NOT a physical memory region.
* 
* The source describes:
* 
* WHAT data exists
* WHAT structure it has
* WHAT transformations are intended
* WHAT properties are required
* WHAT capabilities are required
* WHAT constraints apply
* 
* Downstream systems determine:
* 
* WHERE it lives
* HOW it is partitioned
* HOW it is stored
* HOW it is transported
* HOW it is scheduled
* WHICH hardware realizes it
* WHICH runtime executes it
* 
* Therefore:
* 
* dataset syntax
*      !=
* storage implementation
* 
* dataset size
*      !=
* language maximum
* 
* logical partitioning
*      !=
* physical node count
* 
* tensor-valued data
*      !=
* tensor grammar ownership
* 
* source intent
*      !=
* target realization
* 
* This separation is required for:
* 
* Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
* 
* while allowing Zamani programs to scale from the smallest practical
* computation to arbitrarily large finite computations permitted by the
* resources available at realization time.
* 
* ============================================================================
  */
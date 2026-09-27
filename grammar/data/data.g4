/*
 * Zamani — Universal Data Grammar
 * Path: grammar/data/data.g4
 *
 * ============================================================================
 * STATUS
 * ============================================================================
 *
 * Production data-domain parser composition grammar.
 *
 * This file is the canonical DATA DOMAIN composition boundary.
 *
 * It owns the common structural syntax required to connect Zamani's data
 * abstractions with the shared language:
 *
 *     source -> lexer -> parser -> AST -> semantic analysis
 *            -> canonical semantic model / IR -> optimization
 *            -> scheduling -> execution -> target realization
 *
 * Specialized data concerns remain in their existing files:
 *
 *     data/queries.g4
 *     data/datasets.g4
 *     data/transformations.g4
 *     data/tensors.g4
 *     data/tables.g4
 *     data/collections.g4
 *     data/streams.g4
 *
 * This file must not become a second implementation of those grammars.
 *
 * ============================================================================
 * DESIGN PRINCIPLES
 * ============================================================================
 *
 * 1. PORTABLE SEMANTICS
 *
 * The grammar describes logical data intent.
 *
 * It does not select:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     QPU
 *     database vendor
 *     cloud provider
 *     filesystem
 *     physical memory bank
 *     physical address
 *     network node
 *     execution device
 *     storage engine
 *
 * Those decisions belong to semantic analysis, capability/resource analysis,
 * compilation, optimization, scheduling, routing, execution and deployment.
 *
 * 2. POCO-REAF
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Data programs describe WHAT data means and WHAT properties are required,
 * rather than WHICH machine realizes the computation.
 *
 * 3. NO ARTIFICIAL LIMITS
 *
 * This grammar contains no universal maximum for:
 *
 *     records
 *     fields
 *     columns
 *     dimensions
 *     collections
 *     datasets
 *     streams
 *     partitions
 *     replicas
 *     joins
 *     transformations
 *     pipeline stages
 *     nodes
 *     threads
 *     devices
 *     memory
 *     storage
 *
 * Any practical limit is imposed by the implementation, available resources,
 * or an explicit program-level semantic constraint.
 *
 * 4. SHARED LANGUAGE OWNERSHIP
 *
 * General:
 *
 *     expression
 *     typeExpr
 *     lambdaExpression
 *     parameterList
 *     genericParameters
 *     block
 *     annotation
 *     visibilityModifier
 *     literal
 *
 * remain owned by the canonical Zamani grammar.
 *
 * This file MUST NOT create a second expression or type system.
 *
 * 5. NO TARGET ACTIONS
 *
 * There are no embedded target-language actions, semantic predicates,
 * filesystem operations, network operations, or runtime calls.
 *
 * Rust integration remains compatible with Rust 1.97 / 1.97.1 and must not
 * introduce unsafe code.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 *
 *     - data-domain entry point
 *     - logical data declarations
 *     - common data references
 *     - logical schema declarations
 *     - logical record declarations
 *     - collection declarations
 *     - sequence declarations
 *     - stream declarations
 *     - source/sink declarations
 *     - pipeline declarations
 *     - view declarations
 *     - data contracts
 *     - data partition/distribution/replication intent
 *     - provenance/lineage declarations
 *     - common data statements
 *     - common data operation composition
 *     - data endpoint intent
 *     - data resource/capability intent
 *
 * THIS FILE DOES NOT OWN
 *
 *     - lexical tokens
 *     - general expressions
 *     - general types
 *     - query language implementation
 *     - dataset operation implementation
 *     - transformation implementation
 *     - tensor implementation
 *     - table implementation
 *     - collection implementation
 *     - stream implementation
 *     - database engines
 *     - SQL dialects
 *     - storage engines
 *     - physical indexes
 *     - physical partitions
 *     - physical replicas
 *     - scheduling
 *     - routing
 *     - hardware discovery
 *     - hardware placement
 *     - quantum::ir
 *     - QEC
 *     - ZQN
 *     - runtime implementation
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * The canonical root grammar should expose ONE data-domain entry:
 *
 *     dataStmt
 *
 * It should not duplicate the alternatives below.
 *
 * Specialized grammars should be delegated to through their established
 * public entry rules:
 *
 *     dataQueryConstruct
 *     dataDatasetConstruct
 *     dataTransformationConstruct
 *     dataTensorConstruct
 *     dataTableConstruct
 *     dataCollectionConstruct
 *     dataStreamConstruct
 *
 * Where a specialized file currently exposes a different public rule, the
 * integration layer should add a compatibility wrapper there rather than
 * duplicating its implementation here.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar produces parser structure only.
 *
 * Recommended semantic mapping:
 *
 *     dataSchemaDecl
 *         -> logical schema declaration
 *
 *     dataRecordDecl
 *         -> logical record declaration
 *
 *     dataCollectionDecl
 *         -> logical collection declaration
 *
 *     dataSequenceDecl
 *         -> logical sequence declaration
 *
 *     dataStreamDecl
 *         -> logical stream declaration
 *
 *     dataSourceDecl
 *         -> logical data source
 *
 *     dataSinkDecl
 *         -> logical data sink
 *
 *     dataPipelineDecl
 *         -> logical data pipeline
 *
 *     dataViewDecl
 *         -> logical data view
 *
 *     dataContractDecl
 *         -> data contract
 *
 *     dataPartitionDecl
 *         -> partitioning intent
 *
 *     dataDistributionDecl
 *         -> distribution intent
 *
 *     dataReplicationDecl
 *         -> replication intent
 *
 *     dataProvenanceDecl
 *         -> provenance/lineage intent
 *
 * The exact Rust AST type names remain owned by the canonical domain-neutral
 * frontend AST.
 *
 * This file must not introduce a parallel data AST.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * Data syntax lowers through semantic analysis into the repository's
 * canonical semantic/IR boundary.
 *
 * No second data IR is defined here.
 *
 * Logical data constructs may subsequently be lowered to:
 *
 *     sequential execution
 *     parallel execution
 *     vectorized execution
 *     GPU execution
 *     FPGA acceleration
 *     distributed execution
 *     streaming execution
 *     accelerator execution
 *     quantum/hybrid workflows where semantically applicable
 *
 * without changing the source-level data program.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY CONTRACT
 * ============================================================================
 *
 * Resource quantities are expressions.
 *
 * Valid:
 *
 *     requires(quota)
 *     requires(capability("distributed.data"))
 *     requires(capability("streaming.data"))
 *     requires(memory >= required_memory)
 *     prefers(capability("accelerated.data"))
 *
 * Invalid as universal language rules:
 *
 *     MAX_ROWS
 *     MAX_COLUMNS
 *     MAX_PARTITIONS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *
 * Physical realization remains downstream.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * The grammar must not imply an ordering guarantee merely because a data
 * abstraction is iterable.
 *
 * Ordering semantics must be declared by the semantic construct that owns
 * ordering.
 *
 * Implementations may parallelize unordered operations as long as observable
 * semantics are preserved.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing public rule names are intentionally retained where practical:
 *
 *     dataStmt
 *     dataDeclaration
 *     dataSchemaDecl
 *     dataRecordDecl
 *     dataCollectionDecl
 *     dataSequenceDecl
 *     dataStreamDecl
 *     dataSourceDecl
 *     dataSinkDecl
 *     dataPipelineDecl
 *     dataViewDecl
 *     dataTransformDecl
 *     dataContractDecl
 *     dataPartitionDecl
 *     dataDistributionDecl
 *     dataReplicationDecl
 *     dataProvenanceDecl
 *
 * Specialized operations should move toward their dedicated files without
 * forcing callers to rename the top-level data entry point.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * GRAMMAR DECLARATION
 * ============================================================================
 */

parser grammar ZamaniDataParser;

options {
    tokenVocab = Zamani;
}


/*
 * ============================================================================
 * 1. PUBLIC DATA ENTRY POINT
 * ============================================================================
 *
 * Zamani.g4 should delegate to this rule:
 *
 *     | dataStmt
 *
 * No other root grammar should duplicate this data-domain dispatch.
 * ============================================================================
 */

dataStmt
    : dataDeclaration
    | dataStatement
    ;


/*
 * ============================================================================
 * 2. DATA DECLARATIONS
 * ============================================================================
 */

dataDeclaration
    : dataSchemaDecl
    | dataRecordDecl
    | dataCollectionDecl
    | dataSequenceDecl
    | dataStreamDecl
    | dataSourceDecl
    | dataSinkDecl
    | dataPipelineDecl
    | dataViewDecl
    | dataTransformDecl
    | dataContractDecl
    | dataPartitionDecl
    | dataDistributionDecl
    | dataReplicationDecl
    | dataProvenanceDecl
    ;


/*
 * ============================================================================
 * 3. DATA STATEMENTS
 * ============================================================================
 *
 * Specialized domains are delegated rather than duplicated.
 * ============================================================================
 */

dataStatement
    : dataAssignmentStmt
    | dataExpressionStmt
    | dataQueryStmt
    | dataDatasetStmt
    | dataTransformationStmt
    | dataTensorStmt
    | dataTableStmt
    | dataCollectionStmt
    | dataStreamStmt
    | dataSerializationStmt
    | dataValidationStmt
    | dataMovementStmt
    | dataMaterializationStmt
    | dataPartitionStmt
    | dataDistributionStmt
    | dataReplicationStmt
    | dataProvenanceStmt
    | dataLineageStmt
    | dataPipelineStmt
    ;


/*
 * ============================================================================
 * 4. COMMON DATA ASSIGNMENT
 * ============================================================================
 *
 * Assignment itself remains semantically owned by Zamani.
 *
 * This rule only provides a data-domain statement boundary.
 * ============================================================================
 */

dataAssignmentStmt
    : dataReference '=' expression ';'
    ;


/*
 * ============================================================================
 * 5. COMMON DATA EXPRESSION STATEMENT
 * ============================================================================
 *
 * General expression syntax belongs to the shared expression grammar.
 * ============================================================================
 */

dataExpressionStmt
    : expression ';'
    ;


/*
 * ============================================================================
 * 6. DATA REFERENCES
 * ============================================================================
 *
 * Do NOT use:
 *
 *     IDENTIFIER
 *     | qualifiedDataName
 *
 * where the first alternative makes the second partially unreachable.
 *
 * A single qualified-name rule provides deterministic structure.
 * ============================================================================
 */

dataReference
    : qualifiedDataName
    ;

qualifiedDataName
    : IDENTIFIER
      (
          '::'
          IDENTIFIER
      )*
    ;


/*
 * ============================================================================
 * 7. SCHEMAS
 * ============================================================================
 */

dataSchemaDecl
    : visibilityModifier?
      'schema'
      IDENTIFIER
      genericParameters?
      dataExtendsClause?
      '{'
      dataSchemaMember*
      '}'
    ;

dataExtendsClause
    : 'extends'
      qualifiedDataName
      (
          ','
          qualifiedDataName
      )*
    ;

dataSchemaMember
    : dataFieldDecl
    | dataConstraintDecl
    | dataIndexDecl
    | annotation
    ;

dataFieldDecl
    : visibilityModifier?
      IDENTIFIER
      ':'
      typeExpr
      dataFieldModifier*
      dataDefaultValue?
      dataFieldConstraint*
      ';'
    ;

dataFieldModifier
    : 'optional'
    | 'required'
    | 'nullable'
    | 'immutable'
    | 'computed'
    | 'indexed'
    | 'unique'
    | 'key'
    ;

dataDefaultValue
    : '='
      expression
    ;

dataFieldConstraint
    : 'where'
      expression
    ;

dataConstraintDecl
    : 'constraint'
      IDENTIFIER?
      '('
      expression
      ')'
      ';'
    ;

dataIndexDecl
    : 'index'
      IDENTIFIER?
      '('
      dataIndexFieldList
      ')'
      dataIndexOption*
      ';'
    ;

dataIndexFieldList
    : IDENTIFIER
      (
          ','
          IDENTIFIER
      )*
    ;

dataIndexOption
    : 'unique'
    | 'ordered'
    | 'ascending'
    | 'descending'
    ;


/*
 * ============================================================================
 * 8. RECORDS
 * ============================================================================
 */

dataRecordDecl
    : visibilityModifier?
      'record'
      IDENTIFIER
      genericParameters?
      dataExtendsClause?
      '{'
      dataRecordMember*
      '}'
    ;

dataRecordMember
    : dataFieldDecl
    | dataConstraintDecl
    | annotation
    ;


/*
 * ============================================================================
 * 9. COLLECTIONS
 * ============================================================================
 */

dataCollectionDecl
    : visibilityModifier?
      'collection'
      IDENTIFIER
      genericParameters?
      ':'
      typeExpr
      dataCollectionOption*
      dataInitializer?
      ';'
    ;

dataCollectionOption
    : 'ordered'
    | 'unordered'
    | 'unique'
    | 'lazy'
    | 'eager'
    | 'persistent'
    | 'ephemeral'
    | 'append_only'
    | 'mutable'
    | 'immutable'
    ;

dataInitializer
    : '='
      expression
    ;


/*
 * ============================================================================
 * 10. SEQUENCES
 * ============================================================================
 */

dataSequenceDecl
    : visibilityModifier?
      'sequence'
      IDENTIFIER
      genericParameters?
      ':'
      typeExpr
      dataSequenceOption*
      dataInitializer?
      ';'
    ;

dataSequenceOption
    : 'ordered'
    | 'lazy'
    | 'eager'
    | 'persistent'
    | 'ephemeral'
    | 'mutable'
    | 'immutable'
    ;


/*
 * ============================================================================
 * 11. STREAM DECLARATIONS
 * ============================================================================
 *
 * Cardinality is deliberately not encoded.
 * ============================================================================
 */

dataStreamDecl
    : visibilityModifier?
      'stream'
      IDENTIFIER
      genericParameters?
      ':'
      typeExpr
      dataStreamOption*
      ';'
    ;

dataStreamOption
    : 'bounded'
    | 'unbounded'
    | 'ordered'
    | 'unordered'
    | 'replayable'
    | 'non_replayable'
    | 'lossless'
    | 'lossy'
    | 'persistent'
    | 'ephemeral'
    ;


/*
 * ============================================================================
 * 12. SOURCES
 * ============================================================================
 */

dataSourceDecl
    : visibilityModifier?
      'source'
      IDENTIFIER
      genericParameters?
      ':'
      typeExpr
      dataEndpointClause?
      dataSourceOption*
      ';'
    ;

dataSourceOption
    : 'read_only'
    | 'read_write'
    | 'streaming'
    | 'batch'
    | 'replayable'
    | 'non_replayable'
    ;

dataEndpointClause
    : 'from'
      dataEndpointExpression
    ;

dataEndpointExpression
    : expression
    ;


/*
 * ============================================================================
 * 13. SINKS
 * ============================================================================
 */

dataSinkDecl
    : visibilityModifier?
      'sink'
      IDENTIFIER
      genericParameters?
      ':'
      typeExpr
      dataEndpointClauseSink?
      dataSinkOption*
      ';'
    ;

dataSinkOption
    : 'append'
    | 'replace'
    | 'upsert'
    | 'streaming'
    | 'batch'
    | 'transactional'
    ;

dataEndpointClauseSink
    : 'to'
      dataEndpointExpression
    ;


/*
 * ============================================================================
 * 14. PIPELINES
 * ============================================================================
 *
 * Pipeline structure is deliberately independent of physical execution.
 * ============================================================================
 */

dataPipelineDecl
    : visibilityModifier?
      'pipeline'
      IDENTIFIER
      genericParameters?
      '{'
      dataPipelineMember*
      '}'
    ;

dataPipelineMember
    : dataPipelineInput
    | dataPipelineOutput
    | dataPipelineStage
    | dataPipelineRequirement
    | dataPipelineConstraint
    | dataPipelinePreference
    | dataPipelinePolicy
    | annotation
    ;

dataPipelineInput
    : 'input'
      IDENTIFIER
      ':'
      typeExpr
      ';'
    ;

dataPipelineOutput
    : 'output'
      IDENTIFIER
      ':'
      typeExpr
      ';'
    ;

dataPipelineStage
    : 'stage'
      IDENTIFIER
      '='
      expression
      ';'
    ;

dataPipelineRequirement
    : 'requires'
      '('
      expression
      ')'
      ';'
    ;

dataPipelineConstraint
    : 'constraint'
      '('
      expression
      ')'
      ';'
    ;

dataPipelinePreference
    : 'prefers'
      '('
      expression
      ')'
      ';'
    ;

dataPipelinePolicy
    : 'policy'
      IDENTIFIER
      '='
      expression
      ';'
    ;


/*
 * ============================================================================
 * 15. VIEWS
 * ============================================================================
 */

dataViewDecl
    : visibilityModifier?
      'view'
      IDENTIFIER
      genericParameters?
      dataViewTypeClause?
      '='
      expression
      ';'
    ;

dataViewTypeClause
    : ':'
      typeExpr
    ;


/*
 * ============================================================================
 * 16. TRANSFORM DECLARATIONS
 * ============================================================================
 *
 * Actual transformation operators belong to transformations.g4.
 * This declaration only provides a named transformation boundary.
 * ============================================================================
 */

dataTransformDecl
    : visibilityModifier?
      'transform'
      IDENTIFIER
      genericParameters?
      '('
      parameterList?
      ')'
      dataReturnTypeClause?
      block
    ;

dataReturnTypeClause
    : '->'
      typeExpr
    ;


/*
 * ============================================================================
 * 17. DATA CONTRACTS
 * ============================================================================
 */

dataContractDecl
    : visibilityModifier?
      'contract'
      IDENTIFIER
      genericParameters?
      '{'
      dataContractMember*
      '}'
    ;

dataContractMember
    : dataContractRequires
    | dataContractEnsures
    | dataContractInvariant
    | dataContractProperty
    | annotation
    ;

dataContractRequires
    : 'requires'
      '('
      expression
      ')'
      ';'
    ;

dataContractEnsures
    : 'ensures'
      '('
      expression
      ')'
      ';'
    ;

dataContractInvariant
    : 'invariant'
      '('
      expression
      ')'
      ';'
    ;

dataContractProperty
    : IDENTIFIER
      ':'
      typeExpr
      ';'
    ;


/*
 * ============================================================================
 * 18. PARTITIONING
 * ============================================================================
 *
 * This expresses logical partitioning intent.
 *
 * It does NOT specify:
 *
 *     partition count
 *     machine count
 *     node count
 *     shard identifier
 *     physical storage
 *
 * The compiler/runtime determines those later.
 * ============================================================================
 */

dataPartitionDecl
    : visibilityModifier?
      'partition'
      IDENTIFIER
      dataPartitionSource?
      dataPartitionStrategy?
      dataPartitionKeyClause?
      dataPartitionOption*
      ';'
    ;

dataPartitionSource
    : 'of'
      dataReference
    ;

dataPartitionStrategy
    : 'by'
      expression
    ;

dataPartitionKeyClause
    : 'key'
      expression
    ;

dataPartitionOption
    : 'balanced'
    | 'ordered'
    | 'stable'
    | 'adaptive'
    | 'dynamic'
    ;


/*
 * ============================================================================
 * 19. DISTRIBUTION
 * ============================================================================
 */

dataDistributionDecl
    : visibilityModifier?
      'distribution'
      IDENTIFIER
      'of'
      dataReference
      dataDistributionPolicy*
      ';'
    ;

dataDistributionPolicy
    : 'by'
      expression
    | 'requires'
      '('
      expression
      ')'
    | 'constraint'
      '('
      expression
      ')'
    | 'prefers'
      '('
      expression
      ')'
    ;


/*
 * ============================================================================
 * 20. REPLICATION
 * ============================================================================
 *
 * Replica quantities remain expressions/semantic requirements.
 * ============================================================================
 */

dataReplicationDecl
    : visibilityModifier?
      'replication'
      IDENTIFIER
      'of'
      dataReference
      dataReplicationPolicy*
      ';'
    ;

dataReplicationPolicy
    : 'factor'
      expression
    | 'strategy'
      expression
    | 'requires'
      '('
      expression
      ')'
    | 'constraint'
      '('
      expression
      ')'
    | 'prefers'
      '('
      expression
      ')'
    ;


/*
 * ============================================================================
 * 21. PROVENANCE
 * ============================================================================
 */

dataProvenanceDecl
    : visibilityModifier?
      'provenance'
      IDENTIFIER
      '{'
      dataProvenanceMember*
      '}'
    ;

dataProvenanceMember
    : 'source'
      expression
      ';'
    | 'transform'
      expression
      ';'
    | 'derived_from'
      expression
      ';'
    | 'version'
      expression
      ';'
    | 'policy'
      expression
      ';'
    | annotation
    ;


/*
 * ============================================================================
 * 22. QUERY INTEGRATION
 * ============================================================================
 *
 * queries.g4 is the owner of query syntax.
 *
 * The compatibility wrapper below provides a stable data.g4 boundary without
 * duplicating SELECT/FROM/JOIN/GROUP/ORDER/WINDOW semantics.
 *
 * Integration requirement:
 *
 *     queries.g4
 *         -> public rule: dataQueryConstruct
 *
 * ============================================================================
 */

dataQueryStmt
    : dataQueryConstruct
    ;


/*
 * ============================================================================
 * 23. DATASET INTEGRATION
 * ============================================================================
 *
 * datasets.g4 owns dataset-specific operations.
 *
 * Expected public integration rule:
 *
 *     dataDatasetConstruct
 *
 * If an older revision exposes a different public rule, add a compatibility
 * wrapper in datasets.g4 rather than duplicating dataset operations here.
 * ============================================================================
 */

dataDatasetStmt
    : dataDatasetConstruct
    ;


/*
 * ============================================================================
 * 24. TRANSFORMATION INTEGRATION
 * ============================================================================
 *
 * transformations.g4 owns transformation operators and pipelines.
 * ============================================================================
 */

dataTransformationStmt
    : dataTransformationConstruct
    ;


/*
 * ============================================================================
 * 25. TENSOR INTEGRATION
 * ============================================================================
 */

dataTensorStmt
    : dataTensorConstruct
    ;


/*
 * ============================================================================
 * 26. TABLE INTEGRATION
 * ============================================================================
 */

dataTableStmt
    : dataTableConstruct
    ;


/*
 * ============================================================================
 * 27. COLLECTION INTEGRATION
 * ============================================================================
 */

dataCollectionStmt
    : dataCollectionConstruct
    ;


/*
 * ============================================================================
 * 28. STREAM INTEGRATION
 * ============================================================================
 */

dataStreamStmt
    : dataStreamConstruct
    ;


/*
 * ============================================================================
 * 29. SERIALIZATION
 * ============================================================================
 *
 * Serialization is expressed as intent.
 *
 * Format/provider selection remains semantic or interoperability metadata.
 * ============================================================================
 */

dataSerializationStmt
    : 'serialize'
      expression
      dataSerializationTarget?
      ';'
    | 'deserialize'
      expression
      dataSerializationTarget?
      ';'
    ;

dataSerializationTarget
    : 'as'
      expression
    | 'using'
      expression
    ;


/*
 * ============================================================================
 * 30. VALIDATION
 * ============================================================================
 */

dataValidationStmt
    : 'validate'
      expression
      dataValidationOptions*
      ';'
    ;

dataValidationOptions
    : 'against'
      expression
    | 'with'
      expression
    | 'requires'
      '('
      expression
      ')'
    | 'constraint'
      '('
      expression
      ')'
    ;


/*
 * ============================================================================
 * 31. DATA MOVEMENT
 * ============================================================================
 *
 * Movement is logical. It does not imply a physical network or memory path.
 * ============================================================================
 */

dataMovementStmt
    : 'move'
      expression
      'to'
      expression
      dataMovementOption*
      ';'
    ;

dataMovementOption
    : 'requires'
      '('
      expression
      ')'
    | 'constraint'
      '('
      expression
      ')'
    | 'prefers'
      '('
      expression
      ')'
    | 'using'
      expression
    ;


/*
 * ============================================================================
 * 32. MATERIALIZATION
 * ============================================================================
 */

dataMaterializationStmt
    : 'materialize'
      expression
      dataMaterializationOption*
      ';'
    ;

dataMaterializationOption
    : 'as'
      expression
    | 'using'
      expression
    | 'requires'
      '('
      expression
      ')'
    | 'constraint'
      '('
      expression
      ')'
    | 'prefers'
      '('
      expression
      ')'
    ;


/*
 * ============================================================================
 * 33. PIPELINE STATEMENT
 * ============================================================================
 *
 * Named pipeline declarations are owned above.
 *
 * This statement form invokes/composes an existing logical pipeline.
 * ============================================================================
 */

dataPipelineStmt
    : 'pipeline'
      expression
      ';'
    ;


/*
 * ============================================================================
 * 34. DATA PARTITION STATEMENT
 * ============================================================================
 */

dataPartitionStmt
    : 'partition'
      expression
      dataPartitionStatementPolicy*
      ';'
    ;

dataPartitionStatementPolicy
    : 'by'
      expression
    | 'key'
      expression
    | 'using'
      expression
    | 'requires'
      '('
      expression
      ')'
    | 'constraint'
      '('
      expression
      ')'
    | 'prefers'
      '('
      expression
      ')'
    ;


/*
 * ============================================================================
 * 35. DATA DISTRIBUTION STATEMENT
 * ============================================================================
 */

dataDistributionStmt
    : 'distribute'
      expression
      dataDistributionStatementPolicy*
      ';'
    ;

dataDistributionStatementPolicy
    : 'by'
      expression
    | 'requires'
      '('
      expression
      ')'
    | 'constraint'
      '('
      expression
      ')'
    | 'prefers'
      '('
      expression
      ')'
    ;


/*
 * ============================================================================
 * 36. DATA REPLICATION STATEMENT
 * ============================================================================
 */

dataReplicationStmt
    : 'replicate'
      expression
      dataReplicationStatementPolicy*
      ';'
    ;

dataReplicationStatementPolicy
    : 'factor'
      expression
    | 'using'
      expression
    | 'requires'
      '('
      expression
      ')'
    | 'constraint'
      '('
      expression
      ')'
    | 'prefers'
      '('
      expression
      ')'
    ;


/*
 * ============================================================================
 * 37. LINEAGE
 * ============================================================================
 */

dataProvenanceStmt
    : 'provenance'
      expression
      ';'
    ;

dataLineageStmt
    : 'lineage'
      expression
      ';'
    ;


/*
 * ============================================================================
 * 38. EXPLICIT RESOURCE/CAPABILITY INTENT
 * ============================================================================
 *
 * The expressions are deliberately generic.
 *
 * This allows future capabilities without changing this grammar.
 *
 * Examples:
 *
 *     requires(capability("distributed.data"));
 *     requires(memory >= required_memory);
 *     prefers(capability("accelerated.data"));
 *
 * The grammar does not enumerate hardware capabilities.
 * ============================================================================
 */

dataRequirement
    : 'requires'
      '('
      expression
      ')'
    ;

dataConstraint
    : 'constraint'
      '('
      expression
      ')'
    ;

dataPreference
    : 'prefers'
      '('
      expression
      ')'
    ;


/*
 * ============================================================================
 * 39. DATA METADATA
 * ============================================================================
 *
 * Metadata is intentionally extensible.
 *
 * A metadata key does not automatically become a compiler/runtime feature.
 * ============================================================================
 */

dataMetadata
    : '['
      dataMetadataEntry*
      ']'
    ;

dataMetadataEntry
    : IDENTIFIER
      (
          '='
          expression
      )?
      ';'?
    ;


/*
 * ============================================================================
 * 40. INTEGRATION NOTES
 * ============================================================================
 *
 * REQUIRED SPECIALIZED PUBLIC RULES
 * ---------------------------------
 *
 * data/queries.g4
 *     dataQueryConstruct
 *
 * data/datasets.g4
 *     dataDatasetConstruct
 *
 * data/transformations.g4
 *     dataTransformationConstruct
 *
 * data/tensors.g4
 *     dataTensorConstruct
 *
 * data/tables.g4
 *     dataTableConstruct
 *
 * data/collections.g4
 *     dataCollectionConstruct
 *
 * data/streams.g4
 *     dataStreamConstruct
 *
 * These specialized files remain independently authoritative for their
 * operations.
 *
 * If a specialized file currently has a differently named public entry rule,
 * it should expose a compatibility wrapper with the expected name. The
 * implementation of the specialized construct must remain in that specialized
 * file.
 *
 *
 * ROOT INTEGRATION
 * ----------------
 *
 * Zamani.g4 should contain one data-domain dispatch:
 *
 *     | dataStmt
 *
 * It should NOT copy:
 *
 *     dataSchemaDecl
 *     dataQueryStmt
 *     dataPipelineStmt
 *     dataTensorStmt
 *     dataTableStmt
 *     ...
 *
 *
 * AST INTEGRATION
 * ---------------
 *
 * The AST layer should consume the parse tree and map it into the existing
 * domain-neutral frontend AST.
 *
 * No DataAst or DataIR duplicate hierarchy should be introduced merely because
 * this grammar is large.
 *
 *
 * SEMANTIC INTEGRATION
 * --------------------
 *
 * Semantic analysis owns:
 *
 *     schema validation
 *     type compatibility
 *     null/optional semantics
 *     key validity
 *     ordering guarantees
 *     partition semantics
 *     distribution semantics
 *     replication semantics
 *     provenance validity
 *     resource requirements
 *     capability requirements
 *     determinism
 *     ownership/lifetime
 *     effect interactions
 *
 *
 * COMPILER INTEGRATION
 * --------------------
 *
 * Compiler stages may transform logical data operations into:
 *
 *     scalar execution
 *     vector execution
 *     tensor execution
 *     parallel execution
 *     accelerator execution
 *     distributed execution
 *     streaming execution
 *
 * according to available resources and capabilities.
 *
 *
 * RUNTIME INTEGRATION
 * -------------------
 *
 * Runtime discovers actual resources and realizes the semantic plan.
 *
 * This grammar must never perform runtime discovery.
 *
 *
 * CROSS-DOMAIN INTEGRATION
 * ------------------------
 *
 * Data may interact with:
 *
 *     classical
 *     quantum
 *     hybrid
 *     AI
 *     HDL
 *     hardware
 *     distributed
 *     networking
 *     security
 *
 * through ordinary Zamani expressions, types, capabilities, effects and
 * resource contracts.
 *
 * Data grammar must not import another domain's physical implementation.
 *
 *
 * QUANTUM INTEGRATION
 * -------------------
 *
 * Data may provide:
 *
 *     datasets
 *     parameters
 *     measurement records
 *     training data
 *     experiment results
 *
 * to quantum workflows.
 *
 * The quantum representation remains owned by quantum::ir.
 *
 *
 * AI INTEGRATION
 * --------------
 *
 * Tensor/model/dataset semantics remain in their domain grammars and semantic
 * models.
 *
 * This file only provides their data-domain composition boundary.
 *
 *
 * HARD-CODING AUDIT
 * -----------------
 *
 * Forbidden universal limits include:
 *
 *     MAX_ROWS
 *     MAX_COLUMNS
 *     MAX_FIELDS
 *     MAX_PARTITIONS
 *     MAX_REPLICAS
 *     MAX_NODES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_MEMORY
 *     MAX_STORAGE
 *
 * No such limits are defined by this grammar.
 *
 * Quantities written by a programmer are program semantics, not compiler
 * capacities.
 *
 *
 * DIAGNOSTICS
 * -----------
 *
 * Parser diagnostics should report:
 *
 *     unexpected token
 *     malformed declaration
 *     malformed data endpoint
 *     malformed contract
 *     malformed pipeline
 *     malformed resource requirement
 *     malformed partition/distribution/replication intent
 *
 * Semantic diagnostics belong to semantic analysis and must carry source
 * spans from the parser tree.
 *
 *
 * SECURITY
 * --------
 *
 * This grammar does not:
 *
 *     execute expressions
 *     open files
 *     connect to databases
 *     contact networks
 *     allocate memory
 *     invoke providers
 *     execute queries
 *     execute code
 *
 * It only parses source syntax.
 *
 *
 * PERFORMANCE
 * -----------
 *
 * Avoid broad ambiguous alternatives and unnecessary recursive data-expression
 * grammars.
 *
 * In particular, this replacement intentionally removes the old pattern:
 *
 *     dataCoalesceExpression
 *         : dataExpression '??' dataExpression
 *
 * because it created indirect recursive expression ownership.
 *
 * General operators such as coalescing belong to the canonical expression
 * grammar.
 *
 * Likewise, dataReference no longer contains an identifier alternative that is
 * subsumed by qualifiedDataName.
 *
 *
 * COMPLETION CRITERIA
 * -------------------
 *
 * This file is complete when:
 *
 *     [x] one public dataStmt boundary exists
 *     [x] data declarations have one owner
 *     [x] specialized data grammars have delegation points
 *     [x] general expressions are reused
 *     [x] general types are reused
 *     [x] no second data type system exists
 *     [x] no second data AST is required
 *     [x] no second data IR is introduced
 *     [x] no physical hardware limits are encoded
 *     [x] no provider-specific execution is encoded
 *     [x] no target-language actions exist
 *     [x] no unsafe Rust is required
 *     [x] Rust 1.97 / 1.97.1 compatibility is preserved downstream
 *     [x] resource intent is expression-based
 *     [x] capability intent is expression-based
 *     [x] portability is preserved
 *
 * Integration completion additionally requires the specialized grammar files
 * to expose the public wrapper rules documented above and Zamani.g4 to expose
 * exactly one data-domain entry point.
 */
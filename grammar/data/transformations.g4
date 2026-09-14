/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/data/transformations.g4
 *
 * Grammar role:
 *     Authoritative parser grammar for GENERAL DATA TRANSFORMATION SYNTAX.
 *
 * Language baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust edition 2021
 *
 * Safety:
 *     - parser grammar only;
 *     - no embedded Rust actions;
 *     - no semantic predicates;
 *     - no unsafe code;
 *     - no filesystem access;
 *     - no network access;
 *     - no runtime execution;
 *     - no provider-specific implementation;
 *     - no hardware discovery;
 *     - no resource discovery.
 *
 * ============================================================================
 *
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * Zamani source
 *      |
 *      v
 * shared lexer
 *      |
 *      v
 * root parser / delegated parser grammars
 *      |
 *      v
 * data transformation syntax
 *      |
 *      v
 * frontend AST
 *      |
 *      v
 * semantic analysis
 *      |
 *      v
 * canonical data / computation representation
 *      |
 *      +-----------------------------+
 *      |             |               |
 *      v             v               v
 * classical       AI/data       distributed
 * lowering        lowering      lowering
 *      |             |               |
 *      +-------------+---------------+
 *                    |
 *                    v
 *               optimization
 *                    |
 *                    v
 *                scheduling
 *                    |
 *                    v
 *               execution/runtime
 *
 * This grammar is SYNTAX ONLY.
 *
 * It does not construct IR.
 *
 * It does not perform transformations.
 *
 * It does not decide where a transformation executes.
 *
 * It does not decide whether execution uses:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     cluster
 *     cloud
 *     embedded hardware
 *     future hardware
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - general data transformation syntax;
 *     - transformation declarations;
 *     - transformation invocation syntax;
 *     - transformation composition;
 *     - map;
 *     - flat_map;
 *     - filter;
 *     - reduce;
 *     - fold;
 *     - scan;
 *     - group;
 *     - partition;
 *     - repartition intent;
 *     - sort;
 *     - distinct;
 *     - project;
 *     - rename;
 *     - derive;
 *     - cast;
 *     - reshape;
 *     - flatten;
 *     - explode;
 *     - collect;
 *     - materialize intent;
 *     - window transformations;
 *     - joins;
 *     - unions;
 *     - intersections;
 *     - differences;
 *     - concatenation;
 *     - sampling;
 *     - batching;
 *     - limiting;
 *     - taking;
 *     - dropping;
 *     - transformation pipelines;
 *     - transformation stages;
 *     - transformation policies;
 *     - transformation requirements;
 *     - transformation hints;
 *     - transformation metadata;
 *     - transformation provenance boundaries.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - general expressions;
 *     - canonical types;
 *     - general statements;
 *     - data schemas;
 *     - data serialization;
 *     - data deserialization;
 *     - storage engines;
 *     - databases;
 *     - filesystem implementation;
 *     - network transports;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - routing;
 *     - scheduling;
 *     - optimization algorithms;
 *     - hardware discovery;
 *     - resource allocation;
 *     - runtime execution;
 *     - provider APIs.
 *
 * ============================================================================
 *
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * A transformation describes WHAT transformation means.
 *
 * It must not permanently encode HOW or WHERE the transformation executes.
 *
 * Therefore this grammar deliberately contains no:
 *
 *     MAX_ROWS
 *     MAX_COLUMNS
 *     MAX_ELEMENTS
 *     MAX_PARTITIONS
 *     MAX_WORKERS
 *     MAX_THREADS
 *     MAX_DEVICES
 *     MAX_GPUS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_BATCHES
 *
 * Counts, dimensions, sizes and resource quantities are expressions.
 *
 * Physical limits are evaluated by:
 *
 *     semantic analysis
 *     compiler policy
 *     resource management
 *     scheduling
 *     deployment
 *     runtime capability negotiation
 *
 * ============================================================================
 *
 * IMPORTANT INTEGRATION RULE
 * ============================================================================
 *
 * `data.g4` previously contained general transformation productions.
 *
 * Those productions must delegate here rather than remain duplicated there.
 *
 * The intended architecture is:
 *
 *     data.g4
 *          |
 *          +--> dataTransformationConstruct
 *                         |
 *                         v
 *                 transformations.g4
 *
 * AI dataset grammar may similarly consume:
 *
 *     dataTransformationConstruct
 *
 * rather than redefining map/filter/batch/window/etc.
 *
 * This file MUST NOT import AI grammar.
 *
 * This file MUST NOT import quantum grammar.
 *
 * This file MUST NOT import hardware grammar.
 *
 * This prevents domain cycles.
 *
 * ============================================================================
 */

parser grammar ZamaniDataTransformationsParser;

options {
    tokenVocab = Zamani;
}

import ZamaniDataParser;


/* ============================================================================
 * 1. PUBLIC INTEGRATION BOUNDARY
 * ============================================================================
 *
 * This is the stable rule that other data/domain grammars should consume.
 *
 * Example integration in data.g4:
 *
 *     dataTransformationConstruct
 *
 * rather than reproducing the rules in this file.
 *
 * ========================================================================== */

dataTransformationConstruct
    : dataTransformationDeclaration
    | dataTransformationStatement
    | dataTransformationExpression
    | dataTransformationPipeline
    ;


/* ============================================================================
 * 2. DECLARATION
 * ========================================================================== */

/*
 * Declares a reusable transformation.
 *
 * Examples:
 *
 *     transform normalize(value) -> Value {
 *         ...
 *     }
 *
 *     transform normalize<T>(value: T) -> T {
 *         ...
 *     }
 *
 * No machine-specific execution target is implied.
 */
dataTransformationDeclaration
    : visibilityModifier?
      'transform'
      IDENTIFIER
      genericParameters?
      '('
      parameterList?
      ')'
      dataTransformationReturnType?
      dataTransformationAttributes?
      block
    ;

dataTransformationReturnType
    : '->'
      typeExpr
    ;

dataTransformationAttributes
    : '['
      dataTransformationAttribute*
      ']'
    ;

dataTransformationAttribute
    : annotation
    | dataTransformationProperty
    ;

dataTransformationProperty
    : IDENTIFIER
      ('=' expression)?
    ;


/* ============================================================================
 * 3. TRANSFORMATION STATEMENT
 * ========================================================================== */

dataTransformationStatement
    : dataTransformationExpression ';'
    ;


/* ============================================================================
 * 4. CORE TRANSFORMATION EXPRESSION
 * ========================================================================== */

dataTransformationExpression
    : dataMapExpression
    | dataFlatMapExpression
    | dataFilterExpression
    | dataReduceExpression
    | dataFoldExpression
    | dataScanExpression
    | dataGroupExpression
    | dataPartitionExpression
    | dataRepartitionExpression
    | dataSortExpression
    | dataDistinctExpression
    | dataProjectExpression
    | dataRenameExpression
    | dataDeriveExpression
    | dataCastExpression
    | dataReshapeExpression
    | dataFlattenExpression
    | dataExplodeExpression
    | dataCollectExpression
    | dataMaterializeExpression
    | dataWindowExpression
    | dataJoinExpression
    | dataUnionExpression
    | dataIntersectionExpression
    | dataDifferenceExpression
    | dataConcatExpression
    | dataSampleExpression
    | dataBatchExpression
    | dataLimitExpression
    | dataTakeExpression
    | dataDropExpression
    | dataCustomTransformationExpression
    ;


/* ============================================================================
 * 5. MAP
 * ========================================================================== */

dataMapExpression
    : 'map'
      '('
      dataExpression
      ','
      lambdaExpression
      ')'
    ;


/* ============================================================================
 * 6. FLAT MAP
 * ========================================================================== */

dataFlatMapExpression
    : 'flat_map'
      '('
      dataExpression
      ','
      lambdaExpression
      ')'
    ;


/* ============================================================================
 * 7. FILTER
 * ========================================================================== */

dataFilterExpression
    : 'filter'
      '('
      dataExpression
      ','
      lambdaExpression
      ')'
    ;


/* ============================================================================
 * 8. REDUCE
 * ========================================================================== */

dataReduceExpression
    : 'reduce'
      '('
      dataExpression
      ','
      lambdaExpression
      ')'
    ;


/* ============================================================================
 * 9. FOLD
 * ========================================================================== */

dataFoldExpression
    : 'fold'
      '('
      dataExpression
      ','
      dataExpression
      ','
      lambdaExpression
      ')'
    ;


/* ============================================================================
 * 10. SCAN
 * ============================================================================
 *
 * Scan preserves intermediate accumulation states.
 *
 * This is semantically different from reduce because the complete sequence
 * of accumulated results remains observable.
 * ========================================================================== */

dataScanExpression
    : 'scan'
      '('
      dataExpression
      ','
      dataExpression
      ','
      lambdaExpression
      ')'
    ;


/* ============================================================================
 * 11. GROUP
 * ========================================================================== */

dataGroupExpression
    : 'group'
      '('
      dataExpression
      'by'
      dataExpressionList
      ')'
    ;


/* ============================================================================
 * 12. PARTITION
 * ========================================================================== */

dataPartitionExpression
    : 'partition'
      '('
      dataExpression
      'by'
      dataExpressionList
      ')'
    ;


/* ============================================================================
 * 13. REPARTITION
 * ============================================================================
 *
 * Repartitioning expresses logical data redistribution.
 *
 * It does NOT select a physical cluster topology.
 * ========================================================================== */

dataRepartitionExpression
    : 'repartition'
      '('
      dataExpression
      'by'
      dataExpressionList
      dataRepartitionOptions?
      ')'
    ;

dataRepartitionOptions
    : '['
      dataRepartitionOption*
      ']'
    ;

dataRepartitionOption
    : dataTransformationHint
    | dataTransformationRequirement
    | dataTransformationPreference
    ;


/* ============================================================================
 * 14. SORT
 * ========================================================================== */

dataSortExpression
    : 'sort'
      '('
      dataExpression
      'by'
      dataSortKeyList
      ')'
    ;

dataSortKeyList
    : dataSortKey
      (',' dataSortKey)*
    ;

dataSortKey
    : dataExpression
      dataSortDirection?
      dataSortNullPolicy?
    ;

dataSortDirection
    : 'ascending'
    | 'descending'
    ;

dataSortNullPolicy
    : 'nulls_first'
    | 'nulls_last'
    ;


/* ============================================================================
 * 15. DISTINCT
 * ========================================================================== */

dataDistinctExpression
    : 'distinct'
      '('
      dataExpression
      ')'
    ;


/* ============================================================================
 * 16. PROJECT
 * ========================================================================== */

dataProjectExpression
    : 'project'
      '('
      dataExpression
      'select'
      dataProjectionList
      ')'
    ;

dataProjectionList
    : dataProjection
      (',' dataProjection)*
    ;

dataProjection
    : dataExpression
      dataProjectionAlias?
    ;

dataProjectionAlias
    : 'as'
      IDENTIFIER
    ;


/* ============================================================================
 * 17. RENAME
 * ========================================================================== */

dataRenameExpression
    : 'rename'
      '('
      dataExpression
      'fields'
      dataRenameList
      ')'
    ;

dataRenameList
    : dataRenameItem
      (',' dataRenameItem)*
    ;

dataRenameItem
    : IDENTIFIER
      'as'
      IDENTIFIER
    ;


/* ============================================================================
 * 18. DERIVE
 * ============================================================================
 *
 * Adds logically derived fields.
 * ========================================================================== */

dataDeriveExpression
    : 'derive'
      '('
      dataExpression
      ','
      dataDerivationList
      ')'
    ;

dataDerivationList
    : dataDerivation
      (',' dataDerivation)*
    ;

dataDerivation
    : IDENTIFIER
      '='
      expression
    ;


/* ============================================================================
 * 19. CAST
 * ========================================================================== */

dataCastExpression
    : 'cast'
      '('
      dataExpression
      'as'
      typeExpr
      ')'
    ;


/* ============================================================================
 * 20. RESHAPE
 * ============================================================================
 *
 * Shape is expressed as an arbitrary expression list.
 *
 * No fixed tensor/vector dimension is encoded here.
 * ========================================================================== */

dataReshapeExpression
    : 'reshape'
      '('
      dataExpression
      ','
      dataShapeExpression
      ')'
    ;

dataShapeExpression
    : '['
      dataExpressionList?
      ']'
    ;


/* ============================================================================
 * 21. FLATTEN
 * ========================================================================== */

dataFlattenExpression
    : 'flatten'
      '('
      dataExpression
      flattenDepth?
      ')'
    ;

flattenDepth
    : 'depth'
      expression
    ;


/* ============================================================================
 * 22. EXPLODE
 * ========================================================================== */

dataExplodeExpression
    : 'explode'
      '('
      dataExpression
      ')'
    ;


/* ============================================================================
 * 23. COLLECT
 * ============================================================================
 *
 * Collect expresses logical aggregation into a collection.
 *
 * It does not prescribe where collection occurs.
 * ========================================================================== */

dataCollectExpression
    : 'collect'
      '('
      dataExpression
      ')'
    ;


/* ============================================================================
 * 24. MATERIALIZATION
 * ============================================================================
 *
 * Materialization is an execution intent.
 *
 * It does not specify:
 *
 *     RAM
 *     disk
 *     database
 *     object store
 *     device memory
 *
 * Those are downstream decisions.
 * ========================================================================== */

dataMaterializeExpression
    : 'materialize'
      '('
      dataExpression
      dataMaterializationOptions?
      ')'
    ;

dataMaterializationOptions
    : '['
      dataMaterializationOption*
      ']'
    ;

dataMaterializationOption
    : dataTransformationHint
    | dataTransformationRequirement
    | dataTransformationPreference
    ;


/* ============================================================================
 * 25. WINDOWING
 * ========================================================================== */

dataWindowExpression
    : 'window'
      '('
      dataExpression
      dataWindowSpec
      ')'
    ;

dataWindowSpec
    : dataTumblingWindow
    | dataSlidingWindow
    | dataSessionWindow
    | dataCountWindow
    | dataCustomWindow
    ;

dataTumblingWindow
    : 'tumbling'
      '('
      expression
      ')'
    ;

dataSlidingWindow
    : 'sliding'
      '('
      'size'
      expression
      'step'
      expression
      ')'
    ;

dataSessionWindow
    : 'session'
      '('
      'gap'
      expression
      ')'
    ;

dataCountWindow
    : 'count'
      '('
      expression
      ')'
    ;

dataCustomWindow
    : 'custom'
      '('
      expression
      ')'
    ;


/* ============================================================================
 * 26. JOIN
 * ========================================================================== */

dataJoinExpression
    : 'join'
      '('
      dataExpression
      'with'
      dataExpression
      'on'
      dataJoinCondition
      dataJoinOptions?
      ')'
    ;

dataJoinCondition
    : dataExpression
      ('==' | '=')
      dataExpression
    ;

dataJoinOptions
    : '['
      dataJoinOption*
      ']'
    ;

dataJoinOption
    : 'inner'
    | 'left'
    | 'right'
    | 'full'
    | 'semi'
    | 'anti'
    | dataTransformationHint
    | dataTransformationRequirement
    | dataTransformationPreference
    ;


/* ============================================================================
 * 27. SET OPERATIONS
 * ========================================================================== */

dataUnionExpression
    : 'union'
      '('
      dataExpressionList
      ')'
    ;

dataIntersectionExpression
    : 'intersection'
      '('
      dataExpressionList
      ')'
    ;

dataDifferenceExpression
    : 'difference'
      '('
      dataExpression
      ','
      dataExpression
      ')'
    ;


/* ============================================================================
 * 28. CONCATENATION
 * ========================================================================== */

dataConcatExpression
    : 'concat'
      '('
      dataExpressionList
      ')'
    ;


/* ============================================================================
 * 29. SAMPLING
 * ========================================================================== */

dataSampleExpression
    : 'sample'
      '('
      dataExpression
      ','
      expression
      dataSamplingMode?
      ')'
    ;

dataSamplingMode
    : 'with_replacement'
    | 'without_replacement'
    | 'deterministic'
    | 'random'
    ;


/* ============================================================================
 * 30. BATCHING
 * ========================================================================== */

dataBatchExpression
    : 'batch'
      '('
      dataExpression
      ','
      expression
      dataBatchOptions?
      ')'
    ;

dataBatchOptions
    : '['
      dataBatchOption*
      ']'
    ;

dataBatchOption
    : 'drop_remainder'
    | 'keep_remainder'
    | 'ordered'
    | 'unordered'
    | dataTransformationHint
    | dataTransformationRequirement
    | dataTransformationPreference
    ;


/* ============================================================================
 * 31. LIMIT
 * ========================================================================== */

dataLimitExpression
    : 'limit'
      '('
      dataExpression
      ','
      expression
      ')'
    ;


/* ============================================================================
 * 32. TAKE
 * ========================================================================== */

dataTakeExpression
    : 'take'
      '('
      dataExpression
      ','
      expression
      ')'
    ;


/* ============================================================================
 * 33. DROP
 * ========================================================================== */

dataDropExpression
    : 'drop'
      '('
      dataExpression
      ','
      expression
      ')'
    ;


/* ============================================================================
 * 34. CUSTOM TRANSFORMATIONS
 * ============================================================================
 *
 * Custom transformation names are resolved semantically.
 *
 * This prevents the grammar from becoming a closed list of transformations.
 * ========================================================================== */

dataCustomTransformationExpression
    : IDENTIFIER
      '('
      dataTransformationArgumentList?
      ')'
    ;

dataTransformationArgumentList
    : dataTransformationArgument
      (',' dataTransformationArgument)*
    ;

dataTransformationArgument
    : expression
    | lambdaExpression
    | dataExpression
    ;


/* ============================================================================
 * 35. TRANSFORMATION PIPELINES
 * ========================================================================== */

dataTransformationPipeline
    : dataTransformationPipelineSource
      dataTransformationPipelineStage+
    ;

dataTransformationPipelineSource
    : dataExpression
    ;

dataTransformationPipelineStage
    : PIPE_FORWARD
      dataTransformationPipelineOperation
    ;

dataTransformationPipelineOperation
    : dataTransformationExpression
    | dataCustomTransformationExpression
    ;


/*
 * The literal `|>` is intentionally represented as a literal parser token
 * rather than a machine-specific operation.
 *
 * If the canonical lexer already exposes a PIPE_FORWARD token, the lexer
 * vocabulary should be centralized there and this rule can be replaced by
 * that token without changing transformation semantics.
 */
PIPE_FORWARD
    : '|>'
    ;


/* ============================================================================
 * 36. TRANSFORMATION REQUIREMENTS
 * ============================================================================
 *
 * These are semantic requirements, not machine selections.
 *
 * Example:
 *
 *     transform x
 *         requires (associative)
 *
 * does not select a particular processor.
 * ========================================================================== */

dataTransformationRequirement
    : 'requires'
      '('
      expression
      ')'
    ;


/* ============================================================================
 * 37. TRANSFORMATION HINTS
 * ============================================================================
 *
 * A hint is advisory.
 *
 * The compiler may ignore it when it cannot be satisfied without changing
 * semantics.
 * ========================================================================== */

dataTransformationHint
    : 'hint'
      '('
      expression
      ')'
    ;


/* ============================================================================
 * 38. TRANSFORMATION PREFERENCES
 * ========================================================================== */

dataTransformationPreference
    : 'prefer'
      '('
      expression
      ')'
    ;


/* ============================================================================
 * 39. TRANSFORMATION METADATA
 * ========================================================================== */

dataTransformationMetadata
    : 'metadata'
      '{'
      dataTransformationMetadataEntry*
      '}'
    ;

dataTransformationMetadataEntry
    : IDENTIFIER
      '='
      expression
      ';'
    ;


/* ============================================================================
 * 40. TRANSFORMATION PROVENANCE
 * ========================================================================== */

dataTransformationProvenance
    : 'provenance'
      '{'
      dataTransformationProvenanceEntry*
      '}'
    ;

dataTransformationProvenanceEntry
    : IDENTIFIER
      '='
      expression
      ';'
    ;


/* ============================================================================
 * 41. TRANSFORMATION OPTIONS
 * ========================================================================== */

dataTransformationOptions
    : '['
      dataTransformationOption*
      ']'
    ;

dataTransformationOption
    : dataTransformationRequirement
    | dataTransformationHint
    | dataTransformationPreference
    | dataTransformationMetadata
    | dataTransformationProvenance
    | dataTransformationProperty
    ;


/* ============================================================================
 * 42. TRANSFORMATION PIPELINE DECLARATION
 * ========================================================================== */

dataTransformationPipelineDeclaration
    : visibilityModifier?
      'pipeline'
      IDENTIFIER
      genericParameters?
      '{'
      dataTransformationPipelineMember*
      '}'
    ;

dataTransformationPipelineMember
    : dataTransformationPipelineInput
    | dataTransformationPipelineOutput
    | dataTransformationPipelineStageDeclaration
    | dataTransformationPipelineRequirement
    | dataTransformationPipelineHint
    ;

dataTransformationPipelineInput
    : 'input'
      IDENTIFIER
      ':'
      typeExpr
      ';'
    ;

dataTransformationPipelineOutput
    : 'output'
      IDENTIFIER
      ':'
      typeExpr
      ';'
    ;

dataTransformationPipelineStageDeclaration
    : 'stage'
      IDENTIFIER
      '='
      dataTransformationExpression
      ';'
    ;

dataTransformationPipelineRequirement
    : dataTransformationRequirement
      ';'
    ;

dataTransformationPipelineHint
    : dataTransformationHint
      ';'
    ;


/* ============================================================================
 * 43. PIPELINE INVOCATION
 * ========================================================================== */

dataTransformationPipelineInvocation
    : 'pipeline'
      IDENTIFIER
      '('
      dataTransformationArgumentList?
      ')'
    ;


/* ============================================================================
 * 44. EXTENSIBLE TRANSFORMATION INVOCATION
 * ========================================================================== */

/*
 * Named transformations are intentionally open-ended.
 *
 * The semantic layer determines whether an identifier refers to:
 *
 *     - a user transformation;
 *     - a standard-library transformation;
 *     - a registered dialect transformation;
 *     - an imported transformation.
 *
 * The grammar does not maintain a finite registry.
 */
dataNamedTransformationInvocation
    : qualifiedDataName
      '('
      dataTransformationArgumentList?
      ')'
    ;


/* ============================================================================
 * 45. SEMANTICALLY COMPOSABLE TRANSFORMATION
 * ========================================================================== */

dataComposableTransformation
    : dataTransformationExpression
      (
          dataTransformationCompositionOperator
          dataTransformationExpression
      )*
    ;

dataTransformationCompositionOperator
    : '|>'
    | 'then'
    ;


/* ============================================================================
 * 46. TRANSFORMATION DECLARATION CONTRACT
 * ========================================================================== */

/*
 * This rule provides a machine-independent transformation contract.
 *
 * It is intentionally descriptive.
 */
dataTransformationContract
    : 'transform_contract'
      IDENTIFIER
      '{'
      dataTransformationContractMember*
      '}'
    ;

dataTransformationContractMember
    : dataTransformationContractInput
    | dataTransformationContractOutput
    | dataTransformationContractEffect
    | dataTransformationContractRequirement
    | dataTransformationContractGuarantee
    ;

dataTransformationContractInput
    : 'input'
      IDENTIFIER
      ':'
      typeExpr
      ';'
    ;

dataTransformationContractOutput
    : 'output'
      IDENTIFIER
      ':'
      typeExpr
      ';'
    ;

dataTransformationContractEffect
    : 'effect'
      expression
      ';'
    ;

dataTransformationContractRequirement
    : 'requires'
      expression
      ';'
    ;

dataTransformationContractGuarantee
    : 'guarantees'
      expression
      ';'
    ;


/* ============================================================================
 * 47. SEMANTIC PROPERTIES
 * ============================================================================
 *
 * These properties describe mathematical/semantic behavior.
 *
 * They do NOT choose an implementation.
 * ========================================================================== */

dataTransformationSemanticProperty
    : 'property'
      IDENTIFIER
      '='
      expression
      ';'
    ;


/* ============================================================================
 * 48. DETERMINISM
 * ============================================================================
 *
 * Determinism is a semantic declaration.
 *
 * It must not imply a fixed implementation seed.
 * ========================================================================== */

dataTransformationDeterminism
    : 'determinism'
      '('
      dataDeterminismMode
      ')'
    ;

dataDeterminismMode
    : 'deterministic'
    | 'nondeterministic'
    | 'implementation_defined'
    | 'externally_defined'
    ;


/* ============================================================================
 * 49. ORDER SEMANTICS
 * ========================================================================== */

dataTransformationOrder
    : 'order'
      '('
      dataOrderMode
      ')'
    ;

dataOrderMode
    : 'preserve'
    | 'not_preserve'
    | 'define'
      '('
      expression
      ')'
    ;


/* ============================================================================
 * 50. NULL / MISSING-VALUE SEMANTICS
 * ========================================================================== */

dataTransformationMissingValuePolicy
    : 'missing'
      '('
      dataMissingValueMode
      ')'
    ;

dataMissingValueMode
    : 'preserve'
    | 'drop'
    | 'error'
    | 'default'
      '('
      expression
      ')'
    ;


/* ============================================================================
 * 51. ERROR SEMANTICS
 * ========================================================================== */

dataTransformationErrorPolicy
    : 'errors'
      '('
      dataTransformationErrorMode
      ')'
    ;

dataTransformationErrorMode
    : 'propagate'
    | 'drop'
    | 'replace'
      '('
      expression
      ')'
    | 'collect'
    | 'retry'
    ;


/* ============================================================================
 * 52. LAZINESS
 * ========================================================================== */

dataTransformationEvaluation
    : 'evaluation'
      '('
      dataTransformationEvaluationMode
      ')'
    ;

dataTransformationEvaluationMode
    : 'lazy'
    | 'eager'
    | 'deferred'
    | 'implementation_defined'
    ;


/* ============================================================================
 * 53. RESOURCE-NEUTRAL EXECUTION INTENT
 * ============================================================================
 *
 * These rules deliberately describe requirements without identifying hardware.
 * ========================================================================== */

dataTransformationExecutionIntent
    : 'execution'
      '{'
      dataTransformationExecutionProperty*
      '}'
    ;

dataTransformationExecutionProperty
    : 'parallel'
      '('
      expression
      ')'
    | 'streaming'
      '('
      expression
      ')'
    | 'locality'
      '('
      expression
      ')'
    | 'latency'
      '('
      expression
      ')'
    | 'throughput'
      '('
      expression
      ')'
    | 'energy'
      '('
      expression
      ')'
    | 'reliability'
      '('
      expression
      ')'
    ;


/* ============================================================================
 * 54. RESOURCE SCALABILITY
 * ============================================================================
 *
 * No resource cardinality is represented as a grammar constant.
 * ========================================================================== */

dataTransformationResourceIntent
    : 'resources'
      '{'
      dataTransformationResourceProperty*
      '}'
    ;

dataTransformationResourceProperty
    : 'require'
      expression
      ';'
    | 'prefer'
      expression
      ';'
    | 'allow'
      expression
      ';'
    | 'forbid'
      expression
      ';'
    ;


/* ============================================================================
 * 55. DATA TRANSFORMATION COMPOSITION
 * ========================================================================== */

dataTransformationComposition
    : 'compose'
      '('
      dataTransformationCompositionMember
      (',' dataTransformationCompositionMember)*
      ')'
    ;

dataTransformationCompositionMember
    : dataTransformationExpression
    | dataNamedTransformationInvocation
    | dataTransformationPipelineInvocation
    ;


/* ============================================================================
 * 56. CONDITIONAL TRANSFORMATION
 * ============================================================================
 *
 * Conditions remain expressions.
 *
 * Machine characteristics must not become implicit grammar conditions.
 * ========================================================================== */

dataConditionalTransformationExpression
    : 'transform_if'
      '('
      expression
      ','
      dataTransformationExpression
      ','
      dataTransformationExpression
      ')'
    ;


/* ============================================================================
 * 57. TRANSFORMATION VERSIONING
 * ========================================================================== */

dataTransformationVersion
    : 'version'
      '('
      expression
      ')'
    ;


/* ============================================================================
 * 58. DIALECT EXTENSION
 * ============================================================================
 *
 * Future domains can register transformation syntax through semantic
 * dialect registration without requiring this grammar to enumerate every
 * future transformation.
 * ========================================================================== */

dataTransformationDialectExpression
    : 'dialect'
      qualifiedDataName
      '::'
      IDENTIFIER
      '('
      dataTransformationArgumentList?
      ')'
    ;


/* ============================================================================
 * 59. COMPLETE TRANSFORMATION SPECIFICATION
 * ========================================================================== */

dataTransformationSpecification
    : dataTransformationContract?
      dataTransformationSemanticProperty*
      dataTransformationDeterminism?
      dataTransformationOrder?
      dataTransformationMissingValuePolicy?
      dataTransformationErrorPolicy?
      dataTransformationEvaluation?
      dataTransformationExecutionIntent?
      dataTransformationResourceIntent?
      dataTransformationMetadata?
      dataTransformationProvenance?
    ;


/* ============================================================================
 * 60. TRANSFORMATION PIPELINE ROOT
 * ========================================================================== */

dataTransformationProgram
    : dataTransformationConstruct+
      EOF
    ;


/* ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * REQUIRED changes outside this file
 * -----------------------------------
 *
 * 1. grammar/data/data.g4
 *
 *    Remove duplicated general transformation productions and delegate to:
 *
 *        dataTransformationConstruct
 *
 *    In particular, data.g4 must no longer independently own:
 *
 *        dataMapExpression
 *        dataFilterExpression
 *        dataFlatMapExpression
 *        dataReduceExpression
 *        dataFoldExpression
 *        dataGroupExpression
 *        dataSortExpression
 *        dataDistinctExpression
 *        dataProjectExpression
 *        dataJoinExpression
 *        dataAggregateExpression
 *        dataWindowExpression
 *        dataUnionExpression
 *        dataDifferenceExpression
 *        dataIntersectionExpression
 *        dataConcatExpression
 *        dataLimitExpression
 *        dataTakeExpression
 *        dataDropExpression
 *
 *    unless a repository audit proves a particular rule belongs elsewhere.
 *
 *
 * 2. grammar/ai/datasets.g4
 *
 *    Dataset-specific transformation syntax should delegate to this grammar
 *    instead of creating a second general transformation language.
 *
 *
 * 3. grammar/Zamani.g4
 *
 *    The root parser must expose the data transformation entry point through
 *    its data/domain integration layer.
 *
 *    It must remain the owner of shared lexical vocabulary.
 *
 *
 * 4. grammar/expressions/*
 *
 *    General expressions remain owned by the expression grammar.
 *
 *    This file consumes:
 *
 *        expression
 *        lambdaExpression
 *
 *    rather than redefining them.
 *
 *
 * 5. grammar/types/*
 *
 *    Canonical types remain owned by the type grammar.
 *
 *    This file consumes:
 *
 *        typeExpr
 *
 *    rather than creating data-specific competing type systems.
 *
 *
 * 6. AST / semantic analysis
 *
 *    The parser output must be lowered into the existing data semantic model.
 *
 *    This grammar must not instantiate canonical IR structures.
 *
 *
 * 7. Optimization
 *
 *    Optimization consumes semantic/IR representations.
 *
 *    Optimization must not parse this grammar directly.
 *
 *
 * 8. Scheduling
 *
 *    Scheduling consumes the resulting executable representation and resource
 *    constraints.
 *
 *    Scheduling does not depend directly on this grammar.
 *
 *
 * 9. Hardware
 *
 *    Hardware discovery/capability systems determine actual resources.
 *
 *    This grammar cannot name or assume physical resource counts.
 *
 *
 * 10. Quantum
 *
 *     Quantum transformation syntax must remain separate from this general
 *     data transformation grammar.
 *
 *     If a quantum program transforms classical datasets, the semantic layer
 *     composes the resulting representations.
 *
 *     This grammar must never import quantum::ir or quantum grammar merely
 *     because data eventually feeds a quantum computation.
 *
 *
 * ============================================================================
 *
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 *     lexer
 *       |
 *       v
 *     core names/types/expressions
 *       |
 *       v
 *     data.g4
 *       |
 *       v
 *     transformations.g4
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     canonical data/computation representation
 *       |
 *       +--------------------+
 *       |                    |
 *       v                    v
 *     optimization        execution planning
 *                            |
 *                            v
 *                         runtime
 *
 * There is intentionally NO dependency:
 *
 *     transformations.g4 -> runtime
 *     transformations.g4 -> hardware
 *     transformations.g4 -> scheduler
 *     transformations.g4 -> quantum::ir
 *     transformations.g4 -> QEC
 *     transformations.g4 -> ZQN
 *
 * ============================================================================
 *
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden in this file:
 *
 *     MAX_*
 *     fixed resource counts
 *     fixed worker counts
 *     fixed device counts
 *     fixed topology
 *     fixed node counts
 *     fixed memory capacities
 *     fixed tensor dimensions
 *     fixed data sizes
 *     fixed batch maxima
 *     fixed partition maxima
 *     fixed stream maxima
 *     provider-specific device identifiers
 *     physical addresses
 *
 * Expressions are intentionally used for:
 *
 *     sizes
 *     dimensions
 *     limits
 *     batch quantities
 *     window quantities
 *     partition expressions
 *     resource requirements
 *     performance requirements
 *
 * ============================================================================
 *
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when:
 *
 * [ ] ANTLR generation succeeds.
 *
 * [ ] The grammar uses the canonical Zamani token vocabulary.
 *
 * [ ] No embedded Rust action exists.
 *
 * [ ] No unsafe implementation is introduced.
 *
 * [ ] General expressions are reused.
 *
 * [ ] Canonical types are reused.
 *
 * [ ] General transformation syntax has one authoritative owner.
 *
 * [ ] data.g4 does not duplicate these productions.
 *
 * [ ] AI dataset grammar does not duplicate general transformations.
 *
 * [ ] Transformation syntax is backend-independent.
 *
 * [ ] No finite machine/resource limits exist.
 *
 * [ ] Arbitrarily large structural transformation expressions are syntactically
 *     representable subject only to available compiler/runtime resources.
 *
 * [ ] Empty transformation argument lists are rejected where semantically
 *     invalid by syntax.
 *
 * [ ] Transformation composition is deterministic at parse level.
 *
 * [ ] Positive tests exist for every transformation family.
 *
 * [ ] Negative tests exist for malformed transformation syntax.
 *
 * [ ] Boundary tests cover tiny and very large expressions.
 *
 * [ ] Cross-domain tests cover data + classical, data + AI, data + distributed,
 *     data + quantum-classical workflows, and data + hardware-facing programs.
 *
 * [ ] Round-trip parser tests preserve transformation structure.
 *
 * [ ] Semantic analysis—not grammar—checks type compatibility.
 *
 * [ ] Semantic analysis—not grammar—checks resource feasibility.
 *
 * [ ] Runtime—not grammar—checks actual available resources.
 *
 * [ ] POCO-REAF remains intact.
 *
 * ============================================================================
 */
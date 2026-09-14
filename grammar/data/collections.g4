/*
 * ============================================================================
 * Zamani — Universal Data Collections Grammar
 * ============================================================================
 *
 * File:
 *   grammar/data/collections.g4
 *
 * Purpose:
 *   Authoritative syntax for logical collections in Zamani.
 *
 * Architectural position:
 *
 *   Source
 *      |
 *      v
 *   Lexer
 *      |
 *      v
 *   Parser
 *      |
 *      v
 *   collections.g4
 *      |
 *      v
 *   AST
 *      |
 *      v
 *   Semantic analysis
 *      |
 *      v
 *   Data semantic model / IR
 *      |
 *      +--> classical execution
 *      +--> quantum/classical workflows
 *      +--> AI/ML
 *      +--> distributed execution
 *      +--> accelerator execution
 *      +--> storage backends
 *      +--> future execution models
 *
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - logical collection declarations
 *   - collection type relationships
 *   - collection construction syntax
 *   - collection access syntax
 *   - collection update syntax
 *   - collection iteration syntax
 *   - collection transformation syntax
 *   - collection cardinality intent
 *   - collection ordering intent
 *   - collection uniqueness intent
 *   - collection mutability intent
 *   - collection evaluation intent
 *   - logical collection requirements
 *   - logical collection constraints
 *   - collection-level annotations
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - the universal type system
 *   - expressions
 *   - lambdas
 *   - statements
 *   - records
 *   - schemas
 *   - streams
 *   - serialization codecs
 *   - databases
 *   - filesystems
 *   - memory allocation
 *   - physical storage
 *   - physical layout
 *   - CPU/GPU/FPGA/ASIC selection
 *   - quantum hardware
 *   - quantum IR
 *   - classical IR
 *   - scheduling
 *   - optimization
 *   - routing
 *   - hardware discovery
 *   - runtime resource discovery
 *
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * A collection expresses logical data semantics.
 *
 * It MUST NOT require rewriting a program merely because the collection is
 * executed using:
 *
 *   - local memory
 *   - distributed memory
 *   - persistent storage
 *   - a database
 *   - a GPU
 *   - an FPGA
 *   - an accelerator
 *   - a cluster
 *   - a cloud service
 *   - a future storage/execution architecture
 *
 * No fixed machine limits are encoded here.
 *
 * There is deliberately no:
 *
 *   MAX_ELEMENTS
 *   MAX_COLLECTION_SIZE
 *   MAX_PARTITIONS
 *   MAX_NODES
 *   MAX_REPLICAS
 *   MAX_MEMORY
 *   MAX_DEVICES
 *
 * Runtime and resource systems determine actual capacity.
 *
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This grammar contains no target-language actions.
 *
 * It performs:
 *
 *   - no filesystem access
 *   - no network access
 *   - no process execution
 *   - no hardware discovery
 *   - no runtime resource discovery
 *
 * Zamani's Rust integration remains compatible with:
 *
 *   Rust 1.97
 *   Rust 1.97.1
 *
 * and uses no unsafe Rust in Zamani-owned integration code.
 *
 *
 * ============================================================================
 * GRAMMAR INTEGRATION
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It consumes the authoritative Zamani lexer vocabulary.
 *
 * It MUST NOT define a second lexer.
 *
 * Existing canonical rules such as:
 *
 *   typeExpr
 *   expression
 *   lambdaExpression
 *   argumentList
 *   genericParameters
 *   annotation
 *   visibilityModifier
 *   block
 *
 * remain owned by their respective grammar subsystems.
 *
 *
 * ============================================================================
 */

parser grammar collections;

options {
    tokenVocab = Zamani;
}


/*
 * ============================================================================
 * PUBLIC INTEGRATION RULE
 * ============================================================================
 *
 * `dataCollectionDeclaration` is the canonical collection declaration rule.
 *
 * `data.g4` MUST delegate to this rule instead of redefining collection
 * declarations.
 * ============================================================================
 */

dataCollectionDeclaration
    : collectionAnnotation*
      visibilityModifier?
      'collection'
      collectionName
      genericParameters?
      collectionElementType?
      collectionModifier*
      collectionRequirementClause*
      collectionConstraintClause*
      collectionAttribute*
      collectionInitializer?
      ';'
    ;


/*
 * ============================================================================
 * COLLECTION NAME
 * ============================================================================
 */

collectionName
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * ELEMENT TYPE
 * ============================================================================
 *
 * The collection grammar deliberately reuses the canonical type system.
 *
 * It does not define:
 *
 *   array types
 *   map types
 *   tensor types
 *   quantum types
 *   hardware types
 *
 * itself.
 *
 * Those remain owned by grammar/types/.
 * ============================================================================
 */

collectionElementType
    : ':'
      typeExpr
    ;


/*
 * ============================================================================
 * ANNOTATIONS
 * ============================================================================
 */

collectionAnnotation
    : annotation
    ;


/*
 * ============================================================================
 * COLLECTION MODIFIERS
 * ============================================================================
 *
 * These describe logical behavior.
 *
 * They do not prescribe physical implementation.
 * ============================================================================
 */

collectionModifier
    : 'ordered'
    | 'unordered'
    | 'unique'
    | 'multiset'
    | 'mutable'
    | 'immutable'
    | 'persistent'
    | 'ephemeral'
    | 'lazy'
    | 'eager'
    | 'append_only'
    | 'replaceable'
    | 'versioned'
    | 'replayable'
    | 'non_replayable'
    ;


/*
 * ============================================================================
 * REQUIREMENTS
 * ============================================================================
 *
 * Requirements express what a valid implementation must provide.
 *
 * They do NOT select a concrete machine.
 * ============================================================================
 */

collectionRequirementClause
    : 'requires'
      collectionRequirementExpression
      ';'
    ;

collectionRequirementExpression
    : expression
    ;


/*
 * ============================================================================
 * CONSTRAINTS
 * ============================================================================
 *
 * Constraints are semantic conditions.
 *
 * Their enforcement is decided downstream.
 * ============================================================================
 */

collectionConstraintClause
    : 'constraint'
      collectionConstraintName?
      '('
      expression
      ')'
      ';'
    ;

collectionConstraintName
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * ATTRIBUTES
 * ============================================================================
 *
 * Open-ended attributes provide extensibility without forcing every future
 * collection feature into the core grammar.
 * ============================================================================
 */

collectionAttribute
    : 'attribute'
      IDENTIFIER
      collectionAttributeValue?
      ';'
    ;

collectionAttributeValue
    : '='
      expression
    ;


/*
 * ============================================================================
 * INITIALIZATION
 * ============================================================================
 */

collectionInitializer
    : '='
      collectionExpression
    ;


/*
 * ============================================================================
 * COLLECTION EXPRESSIONS
 * ============================================================================
 */

collectionExpression
    : collectionReference
    | collectionLiteral
    | collectionRange
    | collectionAccess
    | collectionSlice
    | collectionMap
    | collectionFilter
    | collectionFlatMap
    | collectionReduce
    | collectionFold
    | collectionScan
    | collectionGroup
    | collectionPartition
    | collectionSort
    | collectionDistinct
    | collectionProject
    | collectionJoin
    | collectionAggregate
    | collectionWindow
    | collectionUnion
    | collectionDifference
    | collectionIntersection
    | collectionConcat
    | collectionTake
    | collectionDrop
    | collectionReverse
    | collectionFlatten
    | collectionZip
    | collectionEnumerate
    | collectionMaterialize
    | collectionCollect
    | collectionTransform
    | collectionExpression
      '('
      argumentList?
      ')'
    ;


/*
 * ============================================================================
 * COLLECTION REFERENCE
 * ============================================================================
 */

collectionReference
    : IDENTIFIER
    | collectionQualifiedName
    ;

collectionQualifiedName
    : IDENTIFIER
      (
          '::'
          IDENTIFIER
      )*
    ;


/*
 * ============================================================================
 * COLLECTION LITERALS
 * ============================================================================
 *
 * There is no fixed element count.
 * ============================================================================
 */

collectionLiteral
    : '['
      collectionElementList?
      ']'
    ;

collectionElementList
    : collectionExpression
      (
          ','
          collectionExpression
      )*
    ;


/*
 * ============================================================================
 * COLLECTION RANGE
 * ============================================================================
 */

collectionRange
    : 'range'
      '('
      collectionRangeBound
      (
          ','
          collectionRangeBound
      )?
      (
          ','
          collectionRangeBound
      )?
      ')'
    ;

collectionRangeBound
    : expression
    ;


/*
 * ============================================================================
 * ACCESS
 * ============================================================================
 */

collectionAccess
    : collectionExpression
      '['
      expression
      ']'
    ;


/*
 * ============================================================================
 * SLICE
 * ============================================================================
 *
 * Supports open-ended logical slices.
 * ============================================================================
 */

collectionSlice
    : collectionExpression
      '['
      expression?
      ':'
      expression?
      (
          ':'
          expression?
      )?
      ']'
    ;


/*
 * ============================================================================
 * MAP
 * ============================================================================
 *
 * Lambda semantics remain owned by the canonical expression grammar.
 * ============================================================================
 */

collectionMap
    : 'map'
      '('
      collectionExpression
      ','
      lambdaExpression
      ')'
    ;


/*
 * ============================================================================
 * FILTER
 * ============================================================================
 */

collectionFilter
    : 'filter'
      '('
      collectionExpression
      ','
      lambdaExpression
      ')'
    ;


/*
 * ============================================================================
 * FLAT MAP
 * ============================================================================
 */

collectionFlatMap
    : 'flat_map'
      '('
      collectionExpression
      ','
      lambdaExpression
      ')'
    ;


/*
 * ============================================================================
 * REDUCE
 * ============================================================================
 */

collectionReduce
    : 'reduce'
      '('
      collectionExpression
      ','
      lambdaExpression
      ')'
    ;


/*
 * ============================================================================
 * FOLD
 * ============================================================================
 */

collectionFold
    : 'fold'
      '('
      collectionExpression
      ','
      expression
      ','
      lambdaExpression
      ')'
    ;


/*
 * ============================================================================
 * SCAN
 * ============================================================================
 *
 * Unlike reduce/fold, scan preserves intermediate results.
 * ============================================================================
 */

collectionScan
    : 'scan'
      '('
      collectionExpression
      ','
      expression
      ','
      lambdaExpression
      ')'
    ;


/*
 * ============================================================================
 * GROUP
 * ============================================================================
 */

collectionGroup
    : 'group'
      '('
      collectionExpression
      'by'
      collectionExpressionList
      ')'
    ;


/*
 * ============================================================================
 * PARTITION
 * ============================================================================
 *
 * Logical partitioning only.
 *
 * It does not determine:
 *
 *   - number of machines
 *   - number of shards
 *   - network topology
 *   - physical locations
 *   - device IDs
 * ============================================================================
 */

collectionPartition
    : 'partition'
      '('
      collectionExpression
      'by'
      collectionExpressionList
      ')'
    ;


/*
 * ============================================================================
 * SORT
 * ============================================================================
 */

collectionSort
    : 'sort'
      '('
      collectionExpression
      'by'
      collectionSortKeyList
      ')'
    ;

collectionSortKeyList
    : collectionSortKey
      (
          ','
          collectionSortKey
      )*
    ;

collectionSortKey
    : expression
      collectionSortDirection?
    ;

collectionSortDirection
    : 'ascending'
    | 'descending'
    ;


/*
 * ============================================================================
 * DISTINCT
 * ============================================================================
 */

collectionDistinct
    : 'distinct'
      '('
      collectionExpression
      ')'
    ;


/*
 * ============================================================================
 * PROJECT
 * ============================================================================
 */

collectionProject
    : 'project'
      '('
      collectionExpression
      'select'
      collectionProjectionList
      ')'
    ;

collectionProjectionList
    : collectionProjection
      (
          ','
          collectionProjection
      )*
    ;

collectionProjection
    : expression
      (
          'as'
          IDENTIFIER
      )?
    ;


/*
 * ============================================================================
 * JOIN
 * ============================================================================
 *
 * Join cardinality and execution strategy are semantic concerns.
 * ============================================================================
 */

collectionJoin
    : 'join'
      '('
      collectionExpression
      'with'
      collectionExpression
      'on'
      collectionJoinCondition
      collectionJoinOption*
      ')'
    ;

collectionJoinCondition
    : expression
      (
          '='
          | '=='
      )
      expression
    ;

collectionJoinOption
    : 'inner'
    | 'left'
    | 'right'
    | 'full'
    | 'outer'
    | 'semi'
    | 'anti'
    ;


/*
 * ============================================================================
 * AGGREGATION
 * ============================================================================
 */

collectionAggregate
    : 'aggregate'
      '('
      collectionExpression
      collectionAggregateGrouping?
      'using'
      collectionAggregateList
      ')'
    ;

collectionAggregateGrouping
    : 'by'
      collectionExpressionList
    ;

collectionAggregateList
    : collectionAggregateFunction
      (
          ','
          collectionAggregateFunction
      )*
    ;

collectionAggregateFunction
    : IDENTIFIER
      '('
      expression
      ')'
    ;


/*
 * ============================================================================
 * WINDOWING
 * ============================================================================
 *
 * Windows are logical data semantics.
 *
 * Timing and runtime scheduling remain downstream concerns.
 * ============================================================================
 */

collectionWindow
    : 'window'
      '('
      collectionExpression
      collectionWindowSpecification
      ')'
    ;

collectionWindowSpecification
    : 'tumbling'
      '('
      expression
      ')'
    | 'sliding'
      '('
      expression
      ','
      expression
      ')'
    | 'session'
      '('
      expression
      ')'
    | 'count'
      '('
      expression
      ')'
    | 'custom'
      '('
      expression
      ')'
    ;


/*
 * ============================================================================
 * SET-LIKE OPERATIONS
 * ============================================================================
 */

collectionUnion
    : 'union'
      '('
      collectionExpressionList
      ')'
    ;

collectionDifference
    : 'difference'
      '('
      collectionExpression
      ','
      collectionExpression
      ')'
    ;

collectionIntersection
    : 'intersection'
      '('
      collectionExpressionList
      ')'
    ;


/*
 * ============================================================================
 * CONCATENATION
 * ============================================================================
 */

collectionConcat
    : 'concat'
      '('
      collectionExpressionList
      ')'
    ;


/*
 * ============================================================================
 * TAKE / DROP
 * ============================================================================
 *
 * The count is an expression rather than a grammar-level constant.
 * ============================================================================
 */

collectionTake
    : 'take'
      '('
      collectionExpression
      ','
      expression
      ')'
    ;

collectionDrop
    : 'drop'
      '('
      collectionExpression
      ','
      expression
      ')'
    ;


/*
 * ============================================================================
 * REVERSE
 * ============================================================================
 */

collectionReverse
    : 'reverse'
      '('
      collectionExpression
      ')'
    ;


/*
 * ============================================================================
 * FLATTEN
 * ============================================================================
 */

collectionFlatten
    : 'flatten'
      '('
      collectionExpression
      ')'
    ;


/*
 * ============================================================================
 * ZIP
 * ============================================================================
 */

collectionZip
    : 'zip'
      '('
      collectionExpressionList
      ')'
    ;


/*
 * ============================================================================
 * ENUMERATE
 * ============================================================================
 */

collectionEnumerate
    : 'enumerate'
      '('
      collectionExpression
      ')'
    ;


/*
 * ============================================================================
 * MATERIALIZATION
 * ============================================================================
 *
 * Materialization is semantic intent.
 *
 * It does NOT dictate:
 *
 *   RAM
 *   disk
 *   database
 *   cache
 *   GPU memory
 *   distributed storage
 * ============================================================================
 */

collectionMaterialize
    : 'materialize'
      '('
      collectionExpression
      collectionMaterializationOption*
      ')'
    ;

collectionMaterializationOption
    : 'persistent'
    | 'ephemeral'
    | 'checkpointed'
    | 'replayable'
    | 'lazy'
    | 'eager'
    ;


/*
 * ============================================================================
 * COLLECT
 * ============================================================================
 *
 * Converts a logical data flow into a collection value.
 * ============================================================================
 */

collectionCollect
    : 'collect'
      '('
      collectionExpression
      ')'
    ;


/*
 * ============================================================================
 * GENERIC TRANSFORMATION
 * ============================================================================
 *
 * This provides an extensibility point for future data paradigms without
 * forcing every new operation to modify the core collection grammar.
 * ============================================================================
 */

collectionTransform
    : 'transform'
      '('
      collectionExpression
      'using'
      lambdaExpression
      ')'
    ;


/*
 * ============================================================================
 * COLLECTION EXPRESSION LIST
 * ============================================================================
 */

collectionExpressionList
    : collectionExpression
      (
          ','
          collectionExpression
      )*
    ;


/*
 * ============================================================================
 * COLLECTION STATEMENTS
 * ============================================================================
 */

collectionStatement
    : collectionDeclarationStatement
    | collectionInsertStatement
    | collectionAppendStatement
    | collectionRemoveStatement
    | collectionUpdateStatement
    | collectionClearStatement
    | collectionConsumeStatement
    | collectionProduceStatement
    | collectionTransformStatement
    | collectionMaterializeStatement
    ;

collectionDeclarationStatement
    : dataCollectionDeclaration
    ;


/*
 * ============================================================================
 * INSERT
 * ============================================================================
 */

collectionInsertStatement
    : 'insert'
      collectionExpression
      'into'
      collectionReference
      ';'
    ;


/*
 * ============================================================================
 * APPEND
 * ============================================================================
 */

collectionAppendStatement
    : 'append'
      collectionExpression
      'to'
      collectionReference
      ';'
    ;


/*
 * ============================================================================
 * REMOVE
 * ============================================================================
 */

collectionRemoveStatement
    : 'remove'
      collectionExpression
      'from'
      collectionReference
      ';'
    ;


/*
 * ============================================================================
 * UPDATE
 * ============================================================================
 */

collectionUpdateStatement
    : 'update'
      collectionReference
      'where'
      expression
      'set'
      collectionUpdateList
      ';'
    ;

collectionUpdateList
    : collectionUpdateItem
      (
          ','
          collectionUpdateItem
      )*
    ;

collectionUpdateItem
    : IDENTIFIER
      '='
      expression
    ;


/*
 * ============================================================================
 * CLEAR
 * ============================================================================
 */

collectionClearStatement
    : 'clear'
      collectionReference
      ';'
    ;


/*
 * ============================================================================
 * CONSUME
 * ============================================================================
 */

collectionConsumeStatement
    : 'consume'
      collectionExpression
      ';'
    ;


/*
 * ============================================================================
 * PRODUCE
 * ============================================================================
 */

collectionProduceStatement
    : 'produce'
      collectionExpression
      ';'
    ;


/*
 * ============================================================================
 * TRANSFORM STATEMENT
 * ============================================================================
 */

collectionTransformStatement
    : 'transform'
      collectionExpression
      ';'
    ;


/*
 * ============================================================================
 * MATERIALIZATION STATEMENT
 * ============================================================================
 */

collectionMaterializeStatement
    : 'materialize'
      collectionExpression
      ';'
    ;


/*
 * ============================================================================
 * COLLECTION PIPELINE
 * ============================================================================
 *
 * Pipeline composition is intentionally logical.
 *
 * Scheduling, parallelization and placement are decided later.
 * ============================================================================
 */

collectionPipeline
    : collectionPipelineSource
      collectionPipelineStage+
    ;

collectionPipelineSource
    : collectionExpression
    ;

collectionPipelineStage
    : '|>'
      collectionPipelineOperation
    ;

collectionPipelineOperation
    : collectionMap
    | collectionFilter
    | collectionFlatMap
    | collectionReduce
    | collectionFold
    | collectionScan
    | collectionGroup
    | collectionPartition
    | collectionSort
    | collectionDistinct
    | collectionProject
    | collectionJoin
    | collectionAggregate
    | collectionWindow
    | collectionUnion
    | collectionDifference
    | collectionIntersection
    | collectionConcat
    | collectionTake
    | collectionDrop
    | collectionReverse
    | collectionFlatten
    | collectionZip
    | collectionEnumerate
    | collectionMaterialize
    | collectionCollect
    | collectionTransform
    ;


/*
 * ============================================================================
 * COLLECTION ITERATION
 * ============================================================================
 *
 * Iteration describes logical traversal.
 *
 * The compiler/runtime decides whether that becomes:
 *
 *   sequential execution
 *   parallel execution
 *   vectorization
 *   GPU execution
 *   FPGA execution
 *   distributed execution
 *   streaming execution
 *   another future mechanism
 * ============================================================================
 */

collectionIteration
    : 'for'
      IDENTIFIER
      'in'
      collectionExpression
      block
    ;


/*
 * ============================================================================
 * COLLECTION PARALLELISM INTENT
 * ============================================================================
 *
 * This is NOT a thread-count declaration.
 *
 * The program expresses parallelism semantically.
 * Resource allocation belongs to the execution/resource subsystem.
 * ============================================================================
 */

collectionParallelExpression
    : 'parallel'
      '('
      collectionExpression
      ')'
    ;


/*
 * ============================================================================
 * COLLECTION RESOURCE / CAPABILITY INTENT
 * ============================================================================
 *
 * These hooks permit integration with the universal resource model without
 * hard-coding machine characteristics.
 * ============================================================================
 */

collectionResourceClause
    : 'resource'
      IDENTIFIER
      (
          '='
          expression
      )?
      ';'
    ;

collectionCapabilityClause
    : 'capability'
      IDENTIFIER
      (
          '='
          expression
      )?
      ';'
    ;


/*
 * ============================================================================
 * COLLECTION PLACEMENT INTENT
 * ============================================================================
 *
 * This is deliberately symbolic.
 *
 * It MUST NOT require physical device IDs, addresses or topology.
 * ============================================================================
 */

collectionPlacementClause
    : 'placement'
      expression
      ';'
    ;


/*
 * ============================================================================
 * COLLECTION PERFORMANCE INTENT
 * ============================================================================
 */

collectionPerformanceClause
    : 'performance'
      expression
      ';'
    ;


/*
 * ============================================================================
 * COLLECTION LATENCY INTENT
 * ============================================================================
 */

collectionLatencyClause
    : 'latency'
      expression
      ';'
    ;


/*
 * ============================================================================
 * COLLECTION ENERGY INTENT
 * ============================================================================
 */

collectionEnergyClause
    : 'energy'
      expression
      ';'
    ;


/*
 * ============================================================================
 * COLLECTION RELIABILITY INTENT
 * ============================================================================
 */

collectionReliabilityClause
    : 'reliability'
      expression
      ';'
    ;


/*
 * ============================================================================
 * COLLECTION PORTABILITY INTENT
 * ============================================================================
 */

collectionPortabilityClause
    : 'portability'
      expression
      ';'
    ;


/*
 * ============================================================================
 * COLLECTION VERSIONING INTENT
 * ============================================================================
 */

collectionVersionClause
    : 'version'
      expression
      ';'
    ;


/*
 * ============================================================================
 * COLLECTION COMPATIBILITY
 * ============================================================================
 */

collectionCompatibilityClause
    : 'compatibility'
      expression
      ';'
    ;


/*
 * ============================================================================
 * COLLECTION PROVENANCE
 * ============================================================================
 *
 * Provenance describes semantic lineage.
 *
 * It does not perform provenance recording itself.
 * ============================================================================
 */

collectionProvenanceClause
    : 'provenance'
      collectionProvenanceExpression
      ';'
    ;

collectionProvenanceExpression
    : expression
    ;


/*
 * ============================================================================
 * COLLECTION VALIDATION
 * ============================================================================
 */

collectionValidationClause
    : 'validate'
      expression
      ';'
    ;


/*
 * ============================================================================
 * COLLECTION DECLARATION EXTENSIONS
 * ============================================================================
 *
 * These are optional members intended to integrate the universal resource,
 * capability and portability models without putting those models inside the
 * collection grammar.
 * ============================================================================
 */

collectionDeclarationMember
    : collectionResourceClause
    | collectionCapabilityClause
    | collectionPlacementClause
    | collectionPerformanceClause
    | collectionLatencyClause
    | collectionEnergyClause
    | collectionReliabilityClause
    | collectionPortabilityClause
    | collectionVersionClause
    | collectionCompatibilityClause
    | collectionProvenanceClause
    | collectionValidationClause
    | collectionAttribute
    ;


/*
 * ============================================================================
 * CANONICAL COLLECTION DECLARATION WITH EXTENSIONS
 * ============================================================================
 *
 * This rule is the preferred future integration form once the resource,
 * capability and metadata grammars are delegated from the core data grammar.
 * ============================================================================
 */

dataCollectionDefinition
    : collectionAnnotation*
      visibilityModifier?
      'collection'
      collectionName
      genericParameters?
      collectionElementType?
      collectionModifier*
      '{'
      collectionDeclarationMember*
      '}'
    ;
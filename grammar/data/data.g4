/*
 * Zamani — Universal Data Grammar
 * Path: grammar/data/data.g4
 *
 * Copyright / project ownership:
 *   Benwellonedge28/Zamani
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines the parser-level syntax for Zamani's portable data
 * programming model.
 *
 * It describes DATA INTENT, not a database engine, storage engine, network
 * provider, memory allocator, accelerator, machine topology, or deployment.
 *
 * The grammar is deliberately independent of:
 *
 *   - CPU count
 *   - GPU count
 *   - FPGA count
 *   - node count
 *   - memory size
 *   - storage size
 *   - record count
 *   - collection cardinality
 *   - stream cardinality
 *   - partition count
 *   - replica count
 *   - network topology
 *   - database vendor
 *   - cloud provider
 *   - device identifier
 *   - physical address
 *   - filesystem layout
 *   - execution placement
 *
 * Those concerns belong to semantic analysis, resource/capability models,
 * compilation, scheduling, execution, deployment, and runtime systems.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 *
 *   - data declarations
 *   - logical schemas
 *   - logical records
 *   - logical collections
 *   - logical streams
 *   - logical sources and sinks
 *   - data expressions
 *   - data transformations
 *   - logical queries
 *   - pipelines
 *   - validation intent
 *   - serialization intent
 *   - data movement intent
 *   - materialization intent
 *   - partitioning intent
 *   - distribution intent
 *   - replication intent
 *   - consistency intent
 *   - provenance/lineage declarations
 *   - logical resource requirements/preferences/hints for data operations
 *
 * THIS FILE DOES NOT OWN
 *
 *   - lexer definitions
 *   - token definitions
 *   - AST implementation
 *   - semantic type checking
 *   - database implementations
 *   - SQL engines
 *   - filesystem implementations
 *   - network implementations
 *   - serialization implementations
 *   - compression implementations
 *   - storage engines
 *   - memory allocation
 *   - hardware discovery
 *   - hardware topology
 *   - scheduling
 *   - routing
 *   - optimization algorithms
 *   - runtime resource discovery
 *   - cloud-provider APIs
 *   - canonical IR definitions
 *   - execution engines
 *
 * ============================================================================
 * ARCHITECTURAL BOUNDARY
 * ============================================================================
 *
 * Source
 *   -> shared Zamani lexer
 *   -> Zamani parser
 *   -> this data parser grammar
 *   -> syntax AST
 *   -> semantic analysis
 *   -> data semantic model / canonical IR
 *   -> optimization
 *   -> scheduling
 *   -> execution
 *   -> storage/network/hardware/runtime backends
 *
 * The grammar never directly selects a physical backend.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * The grammar supports:
 *
 *   Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * by ensuring that source-level data constructs describe portable semantics.
 *
 * A source program may therefore express:
 *
 *   requires distributed execution
 *
 * without expressing:
 *
 *   use 64 machines
 *   use provider X
 *   use device Y
 *
 * Likewise:
 *
 *   partition by key
 *
 * is semantic intent.
 *
 * The actual partition count, topology, placement and transport are decided
 * later by compilation/execution/resource systems.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are NO grammar-level maximums for:
 *
 *   records
 *   fields
 *   collection elements
 *   stream elements
 *   partitions
 *   replicas
 *   nodes
 *   dimensions
 *   datasets
 *   pipelines
 *   transformations
 *   data sources
 *   data sinks
 *
 * Any practical limit comes from parser implementation, available memory,
 * compilation resources, runtime resources, or explicit semantic constraints.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This grammar contains no target-language actions.
 * This grammar performs no filesystem access.
 * This grammar performs no network access.
 * This grammar performs no runtime resource discovery.
 * This grammar requires no unsafe Rust.
 *
 * Zamani-owned Rust integration MUST compile with:
 *
 *   Rust 1.97 / 1.97.1
 *
 * and must not introduce `unsafe`.
 *
 * ============================================================================
 * TOKEN INTEGRATION
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * The shared Zamani lexer owns tokens.
 *
 * The long-term production architecture should provide a stable shared lexer
 * vocabulary, preferably through a dedicated Zamani lexer grammar.
 *
 * This grammar intentionally does NOT define lexer rules.
 *
 * ============================================================================
 * IMPORTANT INTEGRATION RULE
 * ============================================================================
 *
 * `dataStmt` is the root integration rule.
 *
 * The root parser must consume:
 *
 *     dataStmt
 *
 * rather than duplicate the data alternatives in another grammar.
 *
 * Existing root-level rules such as:
 *
 *     dataStmt
 *     databaseOp
 *     webService
 *
 * must be migrated so that data semantics have one authoritative owner.
 *
 * Database/network/provider-specific syntax must not be reintroduced here.
 * Such operations must be represented as logical data endpoint/resource
 * semantics and lowered later.
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
    /*
     * Shared Zamani token vocabulary.
     *
     * During the lexer/parser split migration this should become the stable
     * generated lexer vocabulary (for example ZamaniLexer).
     *
     * Until that migration is complete, the repository integration layer may
     * generate the shared vocabulary from the existing Zamani grammar.
     */
    tokenVocab = Zamani;
}


/*
 * ============================================================================
 * TOP-LEVEL DATA DECLARATIONS
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
 * ROOT STATEMENT INTEGRATION
 * ============================================================================
 *
 * The root Zamani statement rule should contain exactly one data entry point:
 *
 *     | dataStmt
 *
 * This grammar owns the alternatives beneath that entry point.
 * ============================================================================
 */

dataStmt
    : dataDeclaration
    | dataExpressionStmt
    | dataTransformationStmt
    | dataQueryStmt
    | dataPipelineStmt
    | dataSerializationStmt
    | dataValidationStmt
    | dataMovementStmt
    | dataMaterializationStmt
    | dataStreamStmt
    ;


/*
 * ============================================================================
 * DATA EXPRESSIONS
 * ============================================================================
 *
 * Expression precedence is deliberately delegated to the shared Zamani
 * expression system where possible.
 *
 * Data-specific operations are represented as explicit forms instead of
 * allowing unrestricted recursive invocation of arbitrary data expressions.
 * This prevents accidental grammar ambiguity and makes semantic lowering
 * deterministic.
 * ============================================================================
 */

dataExpression
    : dataPrimaryExpression dataPostfix*
    ;

dataPrimaryExpression
    : dataReference
    | dataLiteral
    | dataCollectionLiteral
    | dataRecordLiteral
    | dataTransformExpression
    | dataQueryExpression
    | dataPipelineExpression
    | dataSerializationExpression
    | dataMaterializationExpression
    | dataCastExpression
    | dataCoalesceExpression
    | dataConditionalExpression
    | '(' dataExpression ')'
    ;

dataPostfix
    : dataFieldAccessSuffix
    | dataIndexAccessSuffix
    | dataSliceAccessSuffix
    | dataCallSuffix
    ;

dataFieldAccessSuffix
    : '.' IDENTIFIER
    ;

dataIndexAccessSuffix
    : '[' expression ']'
    ;

dataSliceAccessSuffix
    : '[' expression? ':' expression? ']'
    ;

dataCallSuffix
    : '(' argumentList? ')'
    ;

dataExpressionStmt
    : dataExpression ';'
    ;

dataReference
    : IDENTIFIER
    | qualifiedDataName
    ;

qualifiedDataName
    : IDENTIFIER
      ('::' IDENTIFIER)*
    ;

dataLiteral
    : literal
    ;

dataCollectionLiteral
    : '[' dataExpressionList? ']'
    ;

dataRecordLiteral
    : '{' dataFieldInitializerList? '}'
    ;

dataFieldInitializerList
    : dataFieldInitializer
      (',' dataFieldInitializer)*
    ;

dataFieldInitializer
    : IDENTIFIER ':' dataExpression
    ;

dataExpressionList
    : dataExpression
      (',' dataExpression)*
    ;

dataCastExpression
    : 'cast'
      '('
      dataExpression
      'as'
      typeExpr
      ')'
    ;

dataCoalesceExpression
    : dataExpression
      '??'
      dataExpression
    ;

dataConditionalExpression
    : 'if'
      expression
      'then'
      dataExpression
      'else'
      dataExpression
    ;


/*
 * ============================================================================
 * SCHEMAS
 * ============================================================================
 *
 * A schema is logical structure.
 *
 * It does not imply:
 *
 *   - SQL table
 *   - filesystem layout
 *   - memory layout
 *   - network packet layout
 *   - physical database index
 *
 * Those interpretations belong to later semantic/target stages.
 * ============================================================================
 */

dataSchemaDecl
    : visibilityModifier?
      'schema'
      IDENTIFIER
      genericParameters?
      dataSchemaExtends?
      '{'
      dataSchemaMember*
      '}'
    ;

dataSchemaExtends
    : 'extends'
      qualifiedDataName
      (',' qualifiedDataName)*
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
    : '=' dataExpression
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
      (',' IDENTIFIER)*
    ;

dataIndexOption
    : 'unique'
    | 'ordered'
    | 'ascending'
    | 'descending'
    ;


/*
 * ============================================================================
 * RECORDS
 * ============================================================================
 */

dataRecordDecl
    : visibilityModifier?
      'record'
      IDENTIFIER
      genericParameters?
      dataSchemaExtends?
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
 * COLLECTIONS
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
      dataExpression
    ;


/*
 * ============================================================================
 * SEQUENCES
 * ============================================================================
 *
 * A sequence is an ordered logical data abstraction.
 *
 * It is intentionally distinct from a physical array/vector/buffer.
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
 * STREAMS
 * ============================================================================
 *
 * A stream may be bounded or unbounded.
 *
 * No stream cardinality is encoded into the grammar.
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

dataStreamStmt
    : 'stream'
      dataExpression
      dataStreamOperator*
      ';'
    ;

dataStreamOperator
    : 'map' lambdaExpression
    | 'filter' lambdaExpression
    | 'flat_map' lambdaExpression
    | 'window' dataWindowSpec
    | 'batch' dataBatchSpec
    | 'buffer' dataExpression
    | 'throttle' dataExpression
    | 'sample' dataExpression
    | 'deduplicate'
    | 'checkpoint'
    ;


/*
 * ============================================================================
 * SOURCES AND SINKS
 * ============================================================================
 *
 * Endpoints are logical.
 *
 * A string endpoint is intentionally opaque to the grammar. Semantic analysis
 * determines whether it represents a logical resource, URI, symbolic endpoint,
 * provider-independent identifier, or another supported endpoint abstraction.
 * ============================================================================
 */

dataSourceDecl
    : visibilityModifier?
      'source'
      IDENTIFIER
      genericParameters?
      ':'
      typeExpr
      dataEndpointSpec?
      ';'
    ;

dataSinkDecl
    : visibilityModifier?
      'sink'
      IDENTIFIER
      genericParameters?
      ':'
      typeExpr
      dataEndpointSpec?
      ';'
    ;

dataEndpointSpec
    : 'from'
      dataEndpointExpression
    | 'to'
      dataEndpointExpression
    ;

dataEndpointExpression
    : STRING
    | dataReference
    | dataExpression
    ;


/*
 * ============================================================================
 * PIPELINES
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
    | dataPipelinePolicy
    | dataPipelineRequirement
    | dataPipelineConstraint
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
      dataTransformExpression
      ';'
    ;

dataPipelinePolicy
    : 'policy'
      IDENTIFIER
      '='
      dataExpression
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

dataPipelineStmt
    : 'pipeline'
      dataExpression
      ';'
    ;

dataPipelineExpression
    : dataExpression
      ('|>' dataPipelineStageExpression)+
    ;

dataPipelineStageExpression
    : dataTransformExpression
    | dataQueryExpression
    | dataSerializationExpression
    | dataMaterializationExpression
    ;


/*
 * ============================================================================
 * TRANSFORMS
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
      ('->' typeExpr)?
      block
    ;

dataTransformationStmt
    : dataTransformExpression
      ';'
    ;

dataTransformExpression
    : dataMapExpression
    | dataFilterExpression
    | dataFlatMapExpression
    | dataReduceExpression
    | dataFoldExpression
    | dataGroupExpression
    | dataSortExpression
    | dataDistinctExpression
    | dataProjectExpression
    | dataJoinExpression
    | dataAggregateExpression
    | dataWindowExpression
    | dataUnionExpression
    | dataDifferenceExpression
    | dataIntersectionExpression
    | dataConcatExpression
    | dataLimitExpression
    | dataTakeExpression
    | dataDropExpression
    ;

dataMapExpression
    : 'map'
      '('
      dataExpression
      ','
      lambdaExpression
      ')'
    ;

dataFilterExpression
    : 'filter'
      '('
      dataExpression
      ','
      lambdaExpression
      ')'
    ;

dataFlatMapExpression
    : 'flat_map'
      '('
      dataExpression
      ','
      lambdaExpression
      ')'
    ;

dataReduceExpression
    : 'reduce'
      '('
      dataExpression
      ','
      lambdaExpression
      ')'
    ;

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

dataGroupExpression
    : 'group'
      '('
      dataExpression
      'by'
      dataExpressionList
      ')'
    ;

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
      ('ascending' | 'descending')?
    ;

dataDistinctExpression
    : 'distinct'
      '('
      dataExpression
      ')'
    ;

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
      ('as' IDENTIFIER)?
    ;

dataJoinExpression
    : 'join'
      '('
      dataExpression
      'with'
      dataExpression
      'on'
      dataJoinCondition
      ')'
    ;

dataJoinCondition
    : dataExpression
      ('==' | '=')
      dataExpression
    ;

dataAggregateExpression
    : 'aggregate'
      '('
      dataExpression
      'by'
      dataExpressionList?
      'using'
      dataAggregateList
      ')'
    ;

dataAggregateList
    : dataAggregate
      (',' dataAggregate)*
    ;

dataAggregate
    : IDENTIFIER
      '('
      dataExpression
      ')'
    ;

dataWindowExpression
    : 'window'
      '('
      dataExpression
      dataWindowSpec
      ')'
    ;

dataWindowSpec
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

dataBatchSpec
    : 'batch'
      '('
      expression
      ')'
    ;

dataUnionExpression
    : 'union'
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

dataIntersectionExpression
    : 'intersection'
      '('
      dataExpressionList
      ')'
    ;

dataConcatExpression
    : 'concat'
      '('
      dataExpressionList
      ')'
    ;

dataLimitExpression
    : 'limit'
      '('
      dataExpression
      ','
      expression
      ')'
    ;

dataTakeExpression
    : 'take'
      '('
      dataExpression
      ','
      expression
      ')'
    ;

dataDropExpression
    : 'drop'
      '('
      dataExpression
      ','
      expression
      ')'
    ;


/*
 * ============================================================================
 * QUERY LANGUAGE
 * ============================================================================
 *
 * This is intentionally a logical query language.
 *
 * It is NOT SQL.
 *
 * A compiler may lower it to SQL, an in-memory execution plan, distributed
 * execution, accelerator execution, streaming execution, or another backend.
 * ============================================================================
 */

dataQueryStmt
    : dataQueryExpression
      ';'
    ;

dataQueryExpression
    : dataSelectExpression
    ;

dataSelectExpression
    : 'select'
      dataProjectionList
      'from'
      dataExpression
      dataQueryClause*
    ;

dataQueryClause
    : 'where'
      expression
    | 'group'
      'by'
      dataExpressionList
    | 'having'
      expression
    | 'order'
      'by'
      dataSortKeyList
    | 'limit'
      expression
    | 'offset'
      expression
    | 'distinct'
    ;


/*
 * ============================================================================
 * VIEWS
 * ============================================================================
 */

dataViewDecl
    : visibilityModifier?
      'view'
      IDENTIFIER
      genericParameters?
      ':'
      typeExpr?
      '='
      dataQueryExpression
      ';'
    ;


/*
 * ============================================================================
 * VALIDATION
 * ============================================================================
 */

dataValidationStmt
    : 'validate'
      dataExpression
      dataValidationClause*
      ';'
    ;

dataValidationClause
    : 'against'
      qualifiedDataName
    | 'with'
      dataValidationOptions
    ;

dataValidationOptions
    : '{'
      dataValidationOption*
      '}'
    ;

dataValidationOption
    : IDENTIFIER
      '='
      dataExpression
      ';'
    ;


/*
 * ============================================================================
 * DATA CONTRACTS
 * ============================================================================
 *
 * Contracts describe semantic guarantees and requirements.
 *
 * They do not directly execute validation.
 * ============================================================================
 */

dataContractDecl
    : visibilityModifier?
      'data_contract'
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
    | dataContractSchema
    | dataContractAttribute
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

dataContractSchema
    : 'schema'
      qualifiedDataName
      ';'
    ;

dataContractAttribute
    : annotation
    ;


/*
 * ============================================================================
 * SERIALIZATION
 * ============================================================================
 *
 * Serialization is expressed as semantic intent.
 *
 * Concrete formats are symbolic identifiers rather than hard-coded provider
 * implementations.
 *
 * This permits:
 *
 *   JSON
 *   XML
 *   CBOR
 *   MessagePack
 *   Protocol Buffers
 *   future formats
 *
 * without making the grammar dependent on serialization libraries.
 * ============================================================================
 */

dataSerializationStmt
    : 'serialize'
      dataExpression
      dataSerializationTarget?
      dataFormatSpec?
      ';'
    | 'deserialize'
      dataSerializationInput
      dataFormatSpec?
      'as'
      typeExpr
      ';'
    ;

dataSerializationExpression
    : 'serialize'
      '('
      dataExpression
      dataFormatSpec?
      ')'
    | 'deserialize'
      '('
      dataSerializationInput
      dataFormatSpec?
      ')'
    ;

dataSerializationTarget
    : 'to'
      dataEndpointExpression
    ;

dataSerializationInput
    : dataEndpointExpression
    | dataExpression
    ;

dataFormatSpec
    : 'format'
      dataFormatName
      dataFormatOptionBlock?
    ;

dataFormatName
    : IDENTIFIER
    | qualifiedDataName
    ;

dataFormatOptionBlock
    : '{'
      dataFormatOption*
      '}'
    ;

dataFormatOption
    : IDENTIFIER
      '='
      dataExpression
      ';'
    ;


/*
 * ============================================================================
 * MATERIALIZATION
 * ============================================================================
 */

dataMaterializationStmt
    : 'materialize'
      dataExpression
      dataMaterializationTarget?
      dataMaterializationOptions?
      ';'
    ;

dataMaterializationExpression
    : 'materialize'
      '('
      dataExpression
      dataMaterializationTarget?
      dataMaterializationOptions?
      ')'
    ;

dataMaterializationTarget
    : 'to'
      dataEndpointExpression
    ;

dataMaterializationOptions
    : '{'
      dataMaterializationOption*
      '}'
    ;

dataMaterializationOption
    : IDENTIFIER
      '='
      dataExpression
      ';'
    ;


/*
 * ============================================================================
 * DATA MOVEMENT
 * ============================================================================
 *
 * Movement is a semantic operation.
 *
 * The compiler/runtime chooses:
 *
 *   local memory
 *   distributed memory
 *   storage
 *   network transport
 *   accelerator transfer
 *   another supported mechanism
 *
 * based on capabilities and constraints.
 * ============================================================================
 */

dataMovementStmt
    : 'move'
      dataExpression
      'to'
      dataEndpointExpression
      dataMovementOptions?
      ';'
    ;

dataMovementOptions
    : '{'
      dataMovementOption*
      '}'
    ;

dataMovementOption
    : IDENTIFIER
      '='
      dataExpression
      ';'
    ;


/*
 * ============================================================================
 * PARTITIONING
 * ============================================================================
 *
 * Partition count is never a grammar constant.
 *
 * The user may specify a semantic partitioning expression, such as:
 *
 *   partition by key
 *
 * without specifying the number of physical partitions.
 * ============================================================================
 */

dataPartitionDecl
    : visibilityModifier?
      'partition'
      IDENTIFIER
      'of'
      dataExpression
      dataPartitionSpec
      ';'
    ;

dataPartitionSpec
    : 'by'
      dataPartitionKeyList
      dataPartitionOptionBlock?
    ;

dataPartitionKeyList
    : dataExpression
      (',' dataExpression)*
    ;

dataPartitionOptionBlock
    : '{'
      dataPartitionOption*
      '}'
    ;

dataPartitionOption
    : IDENTIFIER
      '='
      dataExpression
      ';'
    ;


/*
 * ============================================================================
 * DISTRIBUTION
 * ============================================================================
 *
 * Distribution describes logical placement intent.
 *
 * It does not name a fixed number of nodes.
 * ============================================================================
 */

dataDistributionDecl
    : visibilityModifier?
      'distribution'
      IDENTIFIER
      'of'
      dataExpression
      dataDistributionSpec
      ';'
    ;

dataDistributionSpec
    : 'by'
      dataDistributionPolicy
      dataDistributionOptionBlock?
    ;

dataDistributionPolicy
    : IDENTIFIER
    | qualifiedDataName
    ;

dataDistributionOptionBlock
    : '{'
      dataDistributionOption*
      '}'
    ;

dataDistributionOption
    : IDENTIFIER
      '='
      dataExpression
      ';'
    ;


/*
 * ============================================================================
 * REPLICATION
 * ============================================================================
 *
 * Replication factor is deliberately an expression rather than a grammar
 * constant. The semantic layer determines whether the requested value can
 * actually be satisfied.
 * ============================================================================
 */

dataReplicationDecl
    : visibilityModifier?
      'replicate'
      IDENTIFIER
      'of'
      dataExpression
      dataReplicationSpec
      ';'
    ;

dataReplicationSpec
    : 'with'
      dataReplicationPolicy
      dataReplicationOptionBlock?
    ;

dataReplicationPolicy
    : IDENTIFIER
    | qualifiedDataName
    ;

dataReplicationOptionBlock
    : '{'
      dataReplicationOption*
      '}'
    ;

dataReplicationOption
    : IDENTIFIER
      '='
      dataExpression
      ';'
    ;


/*
 * ============================================================================
 * PROVENANCE / LINEAGE
 * ============================================================================
 */

dataProvenanceDecl
    : visibilityModifier?
      'provenance'
      IDENTIFIER
      'for'
      dataExpression
      '{'
      dataProvenanceMember*
      '}'
    ;

dataProvenanceMember
    : dataLineageDecl
    | dataProvenanceAttribute
    ;

dataLineageDecl
    : 'derived_from'
      dataExpressionList
      ';'
    ;

dataProvenanceAttribute
    : IDENTIFIER
      '='
      dataExpression
      ';'
    ;


/*
 * ============================================================================
 * RESOURCE / CAPABILITY INTENT
 * ============================================================================
 *
 * These constructs are intentionally generic.
 *
 * A data program may state:
 *
 *   requires(...)
 *   prefers(...)
 *   hints(...)
 *
 * but the grammar never converts these into fixed machine assumptions.
 * ============================================================================
 */

dataResourceClause
    : dataRequiresClause
    | dataConstraintClause
    | dataPreferenceClause
    | dataHintClause
    ;

dataRequiresClause
    : 'requires'
      '('
      expression
      ')'
    ;

dataConstraintClause
    : 'constraint'
      '('
      expression
      ')'
    ;

dataPreferenceClause
    : 'prefers'
      '('
      expression
      ')'
    ;

dataHintClause
    : 'hint'
      '('
      expression
      ')'
    ;


/*
 * ============================================================================
 * DATA OPTIONS
 * ============================================================================
 *
 * Generic options preserve forward compatibility.
 *
 * Unknown options must be rejected or accepted according to semantic
 * capability/version policy rather than silently changing meaning.
 * ============================================================================
 */

dataOptionBlock
    : '{'
      dataOption*
      '}'
    ;

dataOption
    : IDENTIFIER
      '='
      dataExpression
      ';'
    ;


/*
 * ============================================================================
 * BACKWARD-COMPATIBILITY BRIDGE
 * ============================================================================
 *
 * Existing Zamani source historically contains forms represented by rules
 * such as:
 *
 *   Database::...
 *   stream ...
 *   serialization operations
 *
 * The production grammar deliberately does NOT reintroduce a separate
 * provider-specific `databaseOp` grammar.
 *
 * Database operations must be represented through logical sources, sinks,
 * queries, transformations, materialization, and endpoint/resource semantics.
 *
 * Migration mapping:
 *
 *   old database operation
 *       ->
 *   logical data source/sink/query operation
 *
 * The compatibility layer outside this grammar may recognize deprecated
 * syntax and lower it into the new semantic model.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * EXTENSIBILITY
 * ============================================================================
 *
 * Future data-domain constructs should be added through explicit grammar
 * rules or registered dialects.
 *
 * They must NOT be introduced through:
 *
 *   catch-all token rules
 *   arbitrary text
 *   provider-specific lexer hacks
 *   fixed resource assumptions
 *   embedded target-language actions
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * UPSTREAM
 *
 *   Shared lexer
 *       |
 *       v
 *   Zamani parser
 *       |
 *       v
 *   dataStmt / dataDeclaration / dataExpression
 *
 *
 * DOWNSTREAM
 *
 *   syntax AST
 *       |
 *       v
 *   semantic analysis
 *       |
 *       +--> type checking
 *       |
 *       +--> capability checking
 *       |
 *       +--> effect checking
 *       |
 *       +--> resource validation
 *       |
 *       v
 *   canonical data semantic representation / IR
 *       |
 *       +--> optimization
 *       |
 *       +--> scheduling
 *       |
 *       +--> routing
 *       |
 *       +--> hardware/resource realization
 *       |
 *       v
 *   execution/runtime
 *
 *
 * CROSS-DOMAIN
 *
 * Classical
 *   data types and expressions integrate with the common type/expression
 *   systems.
 *
 * Quantum
 *   quantum results may be represented as data, but quantum semantics remain
 *   owned by the quantum subsystem and `quantum::ir`.
 *
 * HDL
 *   hardware data interfaces may consume data contracts and schemas, but
 *   physical signal semantics remain owned by HDL/hardware grammar and IR.
 *
 * Distributed
 *   partition/distribution/replication are logical intents; node placement
 *   remains outside this grammar.
 *
 * AI
 *   tensors/datasets may use data declarations and schemas; model semantics
 *   remain owned by the AI subsystem.
 *
 * Networking
 *   endpoints remain logical; transport protocols remain owned by networking.
 *
 * Security
 *   data contracts may be checked against security policies, but cryptographic
 *   implementations remain outside this grammar.
 *
 *
 * ============================================================================
 * NO CIRCULAR DEPENDENCY CONTRACT
 * ============================================================================
 *
 * This grammar MUST NOT depend on:
 *
 *   runtime
 *   scheduler
 *   optimizer
 *   hardware discovery
 *   storage implementation
 *   network implementation
 *   quantum execution
 *   canonical IR implementation
 *
 * Those systems consume the grammar's AST/semantic output.
 *
 * ============================================================================
 */
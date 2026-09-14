/*
 * Zamani — Universal Data Grammar
 * Path: grammar/data/data.g4
 *
 * Purpose
 * -------
 * Authoritative parser grammar for Zamani's language-level data model.
 *
 * This grammar describes:
 *
 *   - data declarations
 *   - schemas
 *   - fields
 *   - records
 *   - collections
 *   - sequences
 *   - streams
 *   - transformations
 *   - projections
 *   - filtering
 *   - grouping
 *   - aggregation
 *   - joins
 *   - sorting
 *   - windowing
 *   - mapping
 *   - folding/reduction
 *   - serialization/deserialization intent
 *   - format selection
 *   - data contracts
 *   - data validation
 *   - data lineage/provenance declarations
 *   - partitioning
 *   - distribution
 *   - replication intent
 *   - consistency requirements
 *   - data movement
 *   - pipelines
 *   - bounded/unbounded data
 *   - lazy/eager evaluation intent
 *   - resource-neutral data requirements
 *
 * Architectural ownership
 * -----------------------
 *
 * THIS FILE OWNS:
 *   - syntax for expressing data semantics and intent
 *   - syntax for data transformations
 *   - syntax for data contracts
 *   - syntax for data-flow composition
 *   - syntax for portable serialization intent
 *   - syntax for logical partitioning and distribution intent
 *
 * THIS FILE DOES NOT OWN:
 *   - concrete database implementations
 *   - filesystem implementation
 *   - network transports
 *   - serialization libraries
 *   - compression implementations
 *   - memory allocation
 *   - CPU/GPU/FPGA/quantum resources
 *   - scheduling
 *   - routing
 *   - hardware discovery
 *   - runtime resource discovery
 *   - concrete storage engines
 *   - concrete cloud providers
 *   - physical topology
 *   - canonical IR definitions
 *
 * Integration boundary
 * --------------------
 *
 * Source
 *   -> Zamani lexer
 *   -> Zamani parser
 *   -> this delegated parser grammar
 *   -> AST
 *   -> semantic analysis
 *   -> data semantic model / IR
 *   -> optimization
 *   -> scheduling
 *   -> execution/runtime
 *   -> storage/network/hardware backends
 *
 * The grammar MUST remain backend independent.
 *
 * Scalability
 * -----------
 *
 * There are deliberately no fixed:
 *
 *   - record counts
 *   - field counts
 *   - collection sizes
 *   - stream sizes
 *   - partition counts
 *   - node counts
 *   - replica counts
 *   - tensor/data dimensions
 *   - memory limits
 *   - device counts
 *   - database counts
 *   - storage capacities
 *
 * Resource availability is a semantic/runtime concern, not a grammar limit.
 *
 * Safety
 * ------
 *
 * No target-language actions are used.
 * No unsafe operations are required.
 * No filesystem/network access is performed by the grammar.
 *
 * Rust integration
 * ----------------
 *
 * Generated parser code MUST be generated with the repository's Rust ANTLR
 * toolchain and compiled under Rust 1.97 / 1.97.1 without unsafe code in
 * Zamani-owned integration code.
 *
 * Token integration
 * -----------------
 *
 * This is intentionally a parser grammar.
 *
 * The root Zamani grammar MUST provide the shared lexer/token vocabulary.
 * This file MUST NOT define a second lexer.
 *
 * The root grammar should delegate/import this grammar after the shared
 * data-related lexical vocabulary has been centralized.
 */

parser grammar ZamaniDataParser;

options {
    tokenVocab = Zamani;
}


/* ==========================================================================
 * TOP-LEVEL DATA DECLARATIONS
 * ========================================================================== */

dataDeclaration
    : dataSchemaDecl
    | dataRecordDecl
    | dataCollectionDecl
    | dataStreamDecl
    | dataPipelineDecl
    | dataContractDecl
    | dataViewDecl
    | dataSourceDecl
    | dataSinkDecl
    | dataTransformDecl
    | dataPartitionDecl
    | dataDistributionDecl
    ;


/* ==========================================================================
 * DATA STATEMENTS
 *
 * This rule is the integration boundary consumed by the root `statement`
 * rule. The root grammar should expose:
 *
 *     | dataStmt
 *
 * without duplicating the alternatives below.
 * ========================================================================== */

dataStmt
    : dataDeclaration
    | dataExpressionStmt
    | dataSerializationStmt
    | dataTransformationStmt
    | dataQueryStmt
    | dataPipelineStmt
    | dataValidationStmt
    | dataMovementStmt
    | dataMaterializationStmt
    | dataStreamStmt
    ;


/* ==========================================================================
 * DATA EXPRESSIONS
 * ========================================================================== */

dataExpression
    : dataReference
    | dataLiteral
    | dataCollectionLiteral
    | dataRecordLiteral
    | dataFieldAccess
    | dataIndexAccess
    | dataSliceExpression
    | dataTransformExpression
    | dataQueryExpression
    | dataPipelineExpression
    | dataCastExpression
    | dataCoalesceExpression
    | dataConditionalExpression
    | dataExpression '(' argumentList? ')'
    ;

dataExpressionStmt
    : dataExpression ';'
    ;

dataReference
    : IDENTIFIER
    | qualifiedDataName
    ;

qualifiedDataName
    : IDENTIFIER ('::' IDENTIFIER)*
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
    : dataFieldInitializer (',' dataFieldInitializer)*
    ;

dataFieldInitializer
    : IDENTIFIER ':' dataExpression
    ;

dataExpressionList
    : dataExpression (',' dataExpression)*
    ;

dataFieldAccess
    : dataExpression '.' IDENTIFIER
    ;

dataIndexAccess
    : dataExpression '[' expression ']'
    ;

dataSliceExpression
    : dataExpression '[' expression? ':' expression? ']'
    ;

dataCastExpression
    : 'cast' '(' dataExpression 'as' typeExpr ')'
    ;

dataCoalesceExpression
    : dataExpression '??' dataExpression
    ;

dataConditionalExpression
    : 'if' expression 'then' dataExpression 'else' dataExpression
    ;


/* ==========================================================================
 * SCHEMAS
 *
 * A schema describes logical data structure.
 *
 * It does NOT describe physical database layout, machine memory layout,
 * network packets, or hardware storage.
 * ========================================================================== */

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
    : 'extends' qualifiedDataName (',' qualifiedDataName)*
    ;

dataSchemaMember
    : dataFieldDecl
    | dataConstraintDecl
    | dataIndexDecl
    | dataSchemaAttribute
    ;

dataSchemaAttribute
    : annotation
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
    : 'where' expression
    ;

dataConstraintDecl
    : 'constraint'
      IDENTIFIER?
      '(' expression ')'
      ';'
    ;

dataIndexDecl
    : 'index'
      IDENTIFIER?
      '(' dataIndexFieldList ')'
      dataIndexOption*
      ';'
    ;

dataIndexFieldList
    : IDENTIFIER (',' IDENTIFIER)*
    ;

dataIndexOption
    : 'unique'
    | 'ordered'
    | 'descending'
    | 'ascending'
    ;


/* ==========================================================================
 * RECORDS
 * ========================================================================== */

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


/* ==========================================================================
 * COLLECTIONS
 *
 * Collection cardinality is intentionally expressed semantically rather than
 * through fixed grammar limits.
 * ========================================================================== */

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
    : '=' dataExpression
    ;


/* ==========================================================================
 * STREAMS
 *
 * Streams may be bounded or unbounded.
 * The grammar must not encode an artificial maximum size.
 * ========================================================================== */

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
      ('pipe' dataExpression)?
      dataStreamOperator*
      ';'
    ;

dataStreamOperator
    : 'map' lambdaExpression
    | 'filter' lambdaExpression
    | 'window' dataWindowSpec
    | 'batch' dataBatchSpec
    | 'buffer' dataExpression
    | 'throttle' dataExpression
    | 'sample' dataExpression
    | 'deduplicate'
    | 'checkpoint'
    ;


/* ==========================================================================
 * SOURCES AND SINKS
 *
 * These describe logical endpoints. They do not hard-code providers.
 * ========================================================================== */

dataSourceDecl
    : visibilityModifier?
      'source'
      IDENTIFIER
      ':'
      typeExpr
      dataEndpointSpec?
      ';'
    ;

dataSinkDecl
    : visibilityModifier?
      'sink'
      IDENTIFIER
      ':'
      typeExpr
      dataEndpointSpec?
      ';'
    ;

dataEndpointSpec
    : 'from' dataEndpointExpression
    | 'to' dataEndpointExpression
    ;

dataEndpointExpression
    : STRING
    | dataReference
    | dataExpression
    ;


/* ==========================================================================
 * DATA PIPELINES
 * ========================================================================== */

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
    | dataPipelineConstraint
    ;

dataPipelineInput
    : 'input' IDENTIFIER ':' typeExpr ';'
    ;

dataPipelineOutput
    : 'output' IDENTIFIER ':' typeExpr ';'
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

dataPipelineConstraint
    : 'requires'
      '(' expression ')'
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


/* ==========================================================================
 * TRANSFORMATIONS
 * ========================================================================== */

dataTransformDecl
    : visibilityModifier?
      'transform'
      IDENTIFIER
      genericParameters?
      '(' parameterList? ')'
      ('->' typeExpr)?
      block
    ;

dataTransformationStmt
    : dataTransformExpression ';'
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
    : 'map' '(' dataExpression ',' lambdaExpression ')'
    ;

dataFilterExpression
    : 'filter' '(' dataExpression ',' lambdaExpression ')'
    ;

dataFlatMapExpression
    : 'flat_map' '(' dataExpression ',' lambdaExpression ')'
    ;

dataReduceExpression
    : 'reduce' '(' dataExpression ',' lambdaExpression ')'
    ;

dataFoldExpression
    : 'fold' '(' dataExpression ',' dataExpression ',' lambdaExpression ')'
    ;

dataGroupExpression
    : 'group' '(' dataExpression 'by' dataExpressionList ')'
    ;

dataSortExpression
    : 'sort' '(' dataExpression 'by' dataSortKeyList ')'
    ;

dataSortKeyList
    : dataSortKey (',' dataSortKey)*
    ;

dataSortKey
    : dataExpression ('ascending' | 'descending')?
    ;

dataDistinctExpression
    : 'distinct' '(' dataExpression ')'
    ;

dataProjectExpression
    : 'project' '(' dataExpression 'select' dataProjectionList ')'
    ;

dataProjectionList
    : dataProjection (',' dataProjection)*
    ;

dataProjection
    : dataExpression ('as' IDENTIFIER)?
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
    : dataAggregate (',' dataAggregate)*
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
    : 'tumbling' '(' expression ')'
    | 'sliding' '(' expression ',' expression ')'
    | 'session' '(' expression ')'
    | 'count' '(' expression ')'
    | 'custom' '(' expression ')'
    ;

dataUnionExpression
    : 'union' '(' dataExpressionList ')'
    ;

dataDifferenceExpression
    : 'difference' '(' dataExpression ',' dataExpression ')'
    ;

dataIntersectionExpression
    : 'intersection' '(' dataExpressionList ')'
    ;

dataConcatExpression
    : 'concat' '(' dataExpressionList ')'
    ;

dataLimitExpression
    : 'limit' '(' dataExpression ',' expression ')'
    ;

dataTakeExpression
    : 'take' '(' dataExpression ',' expression ')'
    ;

dataDropExpression
    : 'drop' '(' dataExpression ',' expression ')'
    ;


/* ==========================================================================
 * QUERY EXPRESSIONS
 *
 * These are logical operations, not database-provider syntax.
 * ========================================================================== */

dataQueryStmt
    : dataQueryExpression ';'
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
    : 'where' expression
    | 'group' 'by' dataExpressionList
    | 'having' expression
    | 'order' 'by' dataSortKeyList
    | 'limit' expression
    | 'offset' expression
    ;


/* ==========================================================================
 * SERIALIZATION
 *
 * Serialization is expressed as intent.
 *
 * A format name does NOT select a particular implementation library.
 * The compiler/runtime resolves the format through capability negotiation.
 * ========================================================================== */

dataSerializationStmt
    : 'serialize'
      dataExpression
      'to'
      dataFormatSpec
      dataSerializationOption*
      ';'
    | 'deserialize'
      dataExpression
      'from'
      dataFormatSpec
      dataSerializationOption*
      ';'
    ;

dataSerializationExpression
    : 'serialize'
      '('
      dataExpression
      'to'
      dataFormatSpec
      ')'
    | 'deserialize'
      '('
      dataExpression
      'from'
      dataFormatSpec
      ')'
    ;

dataFormatSpec
    : dataBuiltinFormat
    | dataNamedFormat
    ;

dataBuiltinFormat
    : 'json'
    | 'xml'
    | 'messagepack'
    | 'protobuf'
    | 'cbor'
    | 'text'
    | 'binary'
    | 'csv'
    | 'tsv'
    ;

dataNamedFormat
    : IDENTIFIER
    ;

dataSerializationOption
    : 'canonical'
    | 'compact'
    | 'pretty'
    | 'deterministic'
    | 'lossless'
    | 'lossy'
    | 'schema' dataReference
    | 'version' expression
    | 'encoding' STRING
    | 'compression' IDENTIFIER
    ;


/* ==========================================================================
 * VALIDATION AND DATA CONTRACTS
 * ========================================================================== */

dataContractDecl
    : visibilityModifier?
      'data_contract'
      IDENTIFIER
      '{'
      dataContractMember*
      '}'
    ;

dataContractMember
    : dataContractRequires
    | dataContractEnsures
    | dataContractInvariant
    | dataContractSchema
    | dataContractVersion
    | dataContractCompatibility
    ;

dataContractRequires
    : 'requires' '(' expression ')' ';'
    ;

dataContractEnsures
    : 'ensures' '(' expression ')' ';'
    ;

dataContractInvariant
    : 'invariant' '(' expression ')' ';'
    ;

dataContractSchema
    : 'schema' dataReference ';'
    ;

dataContractVersion
    : 'version' expression ';'
    ;

dataContractCompatibility
    : 'compatible' '(' expression ')' ';'
    ;

dataValidationStmt
    : 'validate'
      dataExpression
      ('against' dataReference)?
      ('with' expression)?
      ';'
    ;


/* ==========================================================================
 * VIEWS
 *
 * A view is a logical derived data representation.
 * ========================================================================== */

dataViewDecl
    : visibilityModifier?
      'view'
      IDENTIFIER
      genericParameters?
      ':'
      typeExpr
      '='
      dataQueryExpression
      ';'
    ;


/* ==========================================================================
 * PARTITIONING
 *
 * Partitioning is logical.
 *
 * No fixed partition count is encoded here.
 * ========================================================================== */

dataPartitionDecl
    : visibilityModifier?
      'partition'
      IDENTIFIER
      'of'
      dataReference
      dataPartitionStrategy
      ';'
    ;

dataPartitionStrategy
    : 'by' dataExpressionList
    | 'hash' dataExpressionList
    | 'range' dataExpressionList
    | 'key' dataExpressionList
    | 'custom' dataExpression
    ;


/* ==========================================================================
 * DISTRIBUTION
 *
 * Distribution expresses intent. It does not select machines or nodes.
 * ========================================================================== */

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
    : 'partitioned'
    | 'replicated'
    | 'sharded'
    | 'local'
    | 'remote'
    | 'distributed'
    | 'elastic'
    | 'placement' dataExpression
    | 'consistency' dataConsistencyModel
    | 'durability' dataExpression
    ;

dataConsistencyModel
    : IDENTIFIER
    ;


/* ==========================================================================
 * DATA MOVEMENT
 * ========================================================================== */

dataMovementStmt
    : 'move'
      dataExpression
      ('to' | 'from')
      dataEndpointExpression
      dataMovementOption*
      ';'
    ;

dataMovementOption
    : 'streaming'
    | 'buffered'
    | 'lazy'
    | 'eager'
    | 'lossless'
    | 'lossy'
    | 'ordered'
    | 'unordered'
    ;


/* ==========================================================================
 * MATERIALIZATION
 * ========================================================================== */

dataMaterializationStmt
    : 'materialize'
      dataExpression
      dataMaterializationOption*
      ';'
    ;

dataMaterializationExpression
    : 'materialize'
      '('
      dataExpression
      dataMaterializationOption*
      ')'
    ;

dataMaterializationOption
    : 'persistent'
    | 'ephemeral'
    | 'cached'
    | 'lazy'
    | 'eager'
    | 'incremental'
    ;


/* ==========================================================================
 * BATCHING
 * ========================================================================== */

dataBatchSpec
    : 'fixed' '(' expression ')'
    | 'adaptive' '(' expression ')'
    | 'until' '(' expression ')'
    ;


/* ==========================================================================
 * DATA OPERATIONS
 *
 * These preserve compatibility with the original language-level data
 * operations while expanding them into the production data model.
 * ========================================================================== */

databaseOp
    : 'Database'
      '::'
      IDENTIFIER
      '('
      argumentList?
      ')'
      ';'
    ;

webService
    : 'HTTP'
      '::'
      IDENTIFIER
      '('
      argumentList?
      ')'
      ';'
    ;


/* ==========================================================================
 * DATA NAMING
 *
 * Data identifiers deliberately reuse the language-wide identifier model.
 * There is no separate data-specific identifier namespace.
 * ========================================================================== */

dataName
    : IDENTIFIER
    ;

dataQualifiedName
    : dataName ('::' dataName)*
    ;


/* ==========================================================================
 * ARGUMENTS
 *
 * Reuse the language-wide argumentList where available.
 *
 * The fallback rule below exists only as an explicit integration contract
 * for parser delegation. The root grammar must ultimately expose the
 * canonical argumentList rule and this rule must not be duplicated there.
 * ========================================================================== */

dataArgumentList
    : dataArgument (',' dataArgument)*
    ;

dataArgument
    : dataExpression
    | IDENTIFIER '=' dataExpression
    ;


/* ==========================================================================
 * INTEGRATION NOTES
 * ========================================================================== */

/*
 * ROOT GRAMMAR INTEGRATION
 * ------------------------
 *
 * The root Zamani grammar must:
 *
 *   1. import/delegate this parser grammar;
 *   2. expose dataDeclaration where declarations are accepted;
 *   3. expose dataStmt where statements are accepted;
 *   4. reuse the root `expression`, `typeExpr`, `literal`,
 *      `parameterList`, `lambdaExpression`, `argumentList`,
 *      `block`, `annotation`, and visibility rules;
 *   5. NOT copy these rules into Zamani.g4.
 *
 *
 * LEXER INTEGRATION
 * -----------------
 *
 * The shared lexer must own the keywords introduced by this grammar.
 *
 * It must NOT create a separate data lexer.
 *
 * Data keywords must therefore be added to the authoritative lexical
 * vocabulary in the root/shared lexer layer during grammar modularization.
 *
 *
 * AST INTEGRATION
 * ---------------
 *
 * The parser must produce syntax nodes that are lowered into the existing
 * Zamani AST/data semantic model.
 *
 * The grammar must not instantiate runtime data structures.
 *
 *
 * SEMANTIC ANALYSIS
 * -----------------
 *
 * Semantic analysis must validate:
 *
 *   - schema references
 *   - field existence
 *   - field types
 *   - generic constraints
 *   - nullability
 *   - data contracts
 *   - transformation compatibility
 *   - stream boundedness requirements
 *   - serialization compatibility
 *   - format capability requirements
 *   - distribution requirements
 *   - consistency requirements
 *
 * None of these semantic checks belong inside this grammar.
 *
 *
 * RESOURCE INTEGRATION
 * --------------------
 *
 * Expressions such as:
 *
 *   distribution
 *   partition
 *   materialize
 *   stream
 *   placement
 *
 * describe requirements/intent only.
 *
 * They MUST be resolved later against the resource/capability model.
 *
 * No grammar rule may introduce:
 *
 *   MAX_RECORDS
 *   MAX_FIELDS
 *   MAX_PARTITIONS
 *   MAX_STREAM_ITEMS
 *   MAX_NODES
 *   MAX_BYTES
 *   MAX_DATABASES
 *   MAX_CONNECTIONS
 *
 * or equivalent fixed limits.
 *
 *
 * HARDWARE INTEGRATION
 * --------------------
 *
 * This grammar does not refer directly to:
 *
 *   CPU
 *   GPU
 *   FPGA
 *   ASIC
 *   QPU
 *   physical memory
 *   device IDs
 *   topology
 *   hardware addresses
 *
 * Hardware realization belongs to the compiler/runtime/resource layers.
 *
 *
 * QUANTUM INTEGRATION
 * -------------------
 *
 * Data expressions may be used by quantum/classical hybrid programs through
 * the common expression/type system.
 *
 * This grammar does not define quantum state, qubit, gate, QEC, or ZQN
 * semantics.
 *
 * Those remain owned by the quantum language/IR and corresponding
 * subsystems.
 *
 *
 * HDL INTEGRATION
 * ---------------
 *
 * Data declarations may describe logical data exchanged with HDL modules,
 * but this grammar does not define signals, clocks, ports, timing, or
 * hardware layout.
 *
 *
 * DISTRIBUTED INTEGRATION
 * ----------------------
 *
 * Distribution is an intent.
 *
 * Mapping data to concrete nodes is a compiler/runtime responsibility.
 *
 *
 * SECURITY INTEGRATION
 * --------------------
 *
 * Access control, identity, encryption implementation, key management, and
 * trust policy remain outside this grammar.
 *
 * The grammar only permits the semantic layer to attach those policies
 * through the repository's common capability/effect/security model.
 *
 *
 * DETERMINISM
 * -----------
 *
 * Parsing must be deterministic.
 *
 * Canonical serialization is represented by:
 *
 *   serialize <expr> to <format> deterministic;
 *
 * but determinism of the produced representation is a semantic/backend
 * contract, not a parser guarantee.
 *
 *
 * POCO-REAF
 * ---------
 *
 * A valid data program must describe logical data semantics rather than
 * binding itself to a specific implementation.
 *
 * Therefore:
 *
 *   source program
 *       |
 *       v
 *   logical data semantics
 *       |
 *       +--> local execution
 *       +--> distributed execution
 *       +--> accelerated execution
 *       +--> heterogeneous execution
 *       +--> future execution
 *
 * without source-level rewriting merely because available resources change.
 *
 *
 * COMPATIBILITY
 * -------------
 *
 * Existing:
 *
 *   serialize
 *   deserialize
 *   json
 *   xml
 *   messagepack
 *   protobuf
 *   cbor
 *   stream
 *   pipe
 *   Database::...
 *   HTTP::...
 *
 * forms are preserved at the grammar boundary.
 *
 * Their implementations must be migrated behind the new data semantic
 * abstraction rather than removed silently.
 *
 *
 * ERROR HANDLING
 * --------------
 *
 * Invalid data syntax must result in normal parser diagnostics.
 *
 * The grammar must never:
 *
 *   - print errors
 *   - perform I/O
 *   - panic
 *   - access the filesystem
 *   - access the network
 *   - select a backend
 *   - silently recover by changing program meaning
 *
 *
 * TEST REQUIREMENTS
 * -----------------
 *
 * This grammar requires corresponding tests under:
 *
 *   grammar/tests/data/
 *
 * including:
 *
 *   positive/
 *   negative/
 *   scalability/
 *   serialization/
 *   streaming/
 *   transformations/
 *   schemas/
 *   contracts/
 *   distribution/
 *   cross-domain/
 *   compatibility/
 *
 * Minimum semantic coverage must include:
 *
 *   - empty collection
 *   - singleton collection
 *   - arbitrarily large logical collection
 *   - bounded stream
 *   - unbounded stream
 *   - schema inheritance
 *   - optional fields
 *   - nullable fields
 *   - nested records
 *   - nested collections
 *   - transformations
 *   - joins
 *   - aggregation
 *   - windows
 *   - serialization
 *   - deserialization
 *   - pipelines
 *   - partitioning
 *   - distribution
 *   - contracts
 *   - malformed syntax
 *   - cross-domain classical/data programs
 *   - cross-domain quantum/data programs
 *   - cross-domain HDL/data programs
 *
 *
 * COMPLETION CRITERIA
 * -------------------
 *
 * This file is complete when:
 *
 *   1. It is imported by the authoritative Zamani parser grammar.
 *   2. No data rule is duplicated in the root grammar.
 *   3. Shared lexer tokens are centrally defined.
 *   4. Existing serialization syntax remains parse-compatible.
 *   5. Data declarations and expressions have deterministic parse trees.
 *   6. No machine-size limit exists in the grammar.
 *   7. No provider-specific implementation is encoded.
 *   8. No filesystem/network side effects exist.
 *   9. Rust-generated parser integration compiles under Rust 1.97/1.97.1.
 *  10. Zamani-owned integration code contains no unsafe.
 *  11. Positive, negative, boundary, compatibility, and cross-domain tests
 *      pass.
 *  12. AST/semantic/IR ownership is documented and stable.
 */
/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/data/queries.g4
 *
 * Grammar role:
 *     Canonical parser grammar for logical DATA QUERY syntax.
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
 *     - no database-specific execution;
 *     - no hardware discovery;
 *     - no resource discovery.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the syntax of Zamani's logical query model.
 *
 * A query describes WHAT data should be selected, related, filtered,
 * grouped, ordered, transformed, aggregated, or combined.
 *
 * A query does NOT describe HOW or WHERE the query is physically executed.
 *
 * The same query may therefore be lowered to:
 *
 *     - in-memory execution;
 *     - local collections;
 *     - streaming execution;
 *     - distributed execution;
 *     - database execution;
 *     - accelerator execution;
 *     - tensor execution;
 *     - scientific-data execution;
 *     - quantum/classical result processing;
 *     - heterogeneous execution;
 *     - future execution substrates.
 *
 * The physical realization is determined after parsing by semantic analysis,
 * optimization, resource/capability analysis, scheduling, and lowering.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * Zamani source
 *      |
 *      v
 * shared Zamani lexer
 *      |
 *      v
 * Zamani root parser
 *      |
 *      v
 * data query grammar
 *      |
 *      v
 * syntax AST
 *      |
 *      v
 * semantic analysis
 *      |
 *      +--> type checking
 *      +--> effect checking
 *      +--> capability checking
 *      +--> resource checking
 *      +--> determinism analysis
 *      +--> ownership/lifetime analysis
 *      |
 *      v
 * canonical data semantic model
 *      |
 *      v
 * canonical IR / data IR boundary
 *      |
 *      +--> optimization
 *      +--> parallelization
 *      +--> distribution
 *      +--> vectorization
 *      +--> acceleration
 *      +--> scheduling
 *      +--> routing
 *      |
 *      v
 * target realization
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 *
 *     - query declarations;
 *     - query statements;
 *     - query expressions;
 *     - SELECT projection syntax;
 *     - FROM source syntax;
 *     - JOIN syntax;
 *     - WHERE predicates;
 *     - GROUP BY;
 *     - HAVING;
 *     - ORDER BY;
 *     - LIMIT;
 *     - OFFSET;
 *     - DISTINCT;
 *     - QUALIFY;
 *     - WINDOW;
 *     - WITH / common table expressions;
 *     - recursive query intent;
 *     - VALUES query sources;
 *     - set operations;
 *     - EXISTS / quantified predicates;
 *     - subqueries;
 *     - scalar subqueries;
 *     - row/array membership predicates;
 *     - logical query aliases;
 *     - query parameters;
 *     - query-level contracts;
 *     - query-level requirements;
 *     - query-level constraints;
 *     - query-level preferences;
 *     - query-level hints;
 *     - query-level metadata;
 *     - query composition;
 *     - query ordering semantics at the syntax boundary.
 *
 * THIS FILE DOES NOT OWN
 *
 *     - lexer definitions;
 *     - identifier definitions;
 *     - general expression precedence;
 *     - universal type formation;
 *     - general statements;
 *     - general functions;
 *     - collection declarations;
 *     - schema declarations;
 *     - record declarations;
 *     - stream declarations;
 *     - transformation implementation;
 *     - database engines;
 *     - SQL dialects;
 *     - storage engines;
 *     - query planners;
 *     - query optimizers;
 *     - physical indexes;
 *     - physical partitions;
 *     - physical replicas;
 *     - node placement;
 *     - network topology;
 *     - hardware selection;
 *     - CPU/GPU/FPGA/QPU selection;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - runtime execution.
 *
 * ============================================================================
 * NON-OWNERSHIP RULE
 * ============================================================================
 *
 * Query syntax MUST NOT become a hidden database language.
 *
 * For example:
 *
 *     select x from source where x.value > threshold;
 *
 * is Zamani query syntax.
 *
 * It is NOT implicitly:
 *
 *     SQL
 *     PostgreSQL
 *     SQLite
 *     MongoDB
 *     Spark SQL
 *     CUDA
 *     GPU execution
 *     CPU execution
 *     distributed execution
 *
 * A backend may lower the logical query into any suitable implementation
 * while preserving the declared semantics.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Query syntax must remain portable.
 *
 * It must not encode universal limits such as:
 *
 *     MAX_ROWS
 *     MAX_COLUMNS
 *     MAX_FIELDS
 *     MAX_QUERY_DEPTH
 *     MAX_JOINS
 *     MAX_GROUPS
 *     MAX_RESULTS
 *     MAX_PARTITIONS
 *     MAX_REPLICAS
 *     MAX_NODES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_QUBITS
 *     MAX_TENSOR_RANK
 *     MAX_MEMORY
 *
 * Counts, dimensions, limits, thresholds, offsets, window sizes,
 * replication requirements, partition requirements, and similar quantities
 * are expressions or semantic requirements.
 *
 * Example:
 *
 *     limit result_count
 *
 * is legal.
 *
 * A grammar-level rule such as:
 *
 *     limit <= 1024
 *
 * is forbidden.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * Query syntax distinguishes logical intent from physical realization.
 *
 * Requirement:
 *
 *     requires(capability("distributed.query"))
 *
 * Capability:
 *
 *     requires(capability("index.lookup"))
 *
 * Preference:
 *
 *     prefers(capability("accelerated.query"))
 *
 * Constraint:
 *
 *     constraint(latency < target_latency)
 *
 * Hint:
 *
 *     hint(locality(key))
 *
 * None of these selects a physical device.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Query ordering is explicit.
 *
 * A query MUST NOT imply a deterministic row order unless:
 *
 *     ORDER BY
 *
 * or another semantic ordering contract establishes one.
 *
 * Implementations may execute unordered portions in any valid order provided
 * that observable semantics remain unchanged.
 *
 * ============================================================================
 * NULL / OPTIONAL SEMANTICS
 * ============================================================================
 *
 * This grammar does not invent a second null/type system.
 *
 * NULL, optional values, absence, invalidity, and three-valued or
 * type-specific logical semantics are determined by the canonical Zamani
 * type/semantic system.
 *
 * Query syntax only provides structural positions in which those semantics
 * may be expressed.
 *
 * ============================================================================
 * TYPE SYSTEM INTEGRATION
 * ============================================================================
 *
 * This grammar reuses:
 *
 *     typeExpr
 *     expression
 *     lambdaExpression
 *     parameterList
 *     genericParameters
 *     block
 *     annotation
 *     visibilityModifier
 *     qualified names
 *
 * It MUST NOT define a second type system.
 *
 * ============================================================================
 * EXPRESSION INTEGRATION
 * ============================================================================
 *
 * General expressions remain owned by the canonical expression grammar.
 *
 * Query expressions use:
 *
 *     expression
 *
 * for:
 *
 *     predicates
 *     computed projections
 *     aggregate arguments
 *     limits
 *     offsets
 *     window parameters
 *     query requirements
 *     query hints
 *     query constraints
 *
 * Query-specific syntax is only introduced where a structural distinction
 * is required by the data model.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser produces syntax nodes that are lowered into the existing
 * domain-neutral AST/semantic pipeline.
 *
 * The recommended semantic mapping is:
 *
 *     dataQueryDeclaration
 *         -> DataQueryDeclaration
 *
 *     dataQueryExpression
 *         -> DataQuery
 *
 *     dataProjection
 *         -> DataProjection
 *
 *     dataQuerySource
 *         -> DataSourceRef
 *
 *     dataJoin
 *         -> DataJoin
 *
 *     dataPredicate
 *         -> Predicate
 *
 *     dataGrouping
 *         -> Grouping
 *
 *     dataOrdering
 *         -> Ordering
 *
 *     dataSetOperation
 *         -> SetOperation
 *
 *     dataCommonTableExpression
 *         -> QueryBinding
 *
 * The exact Rust AST names remain owned by the canonical AST implementation.
 *
 * This grammar MUST NOT introduce a parallel Rust AST merely for queries.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * Query syntax lowers to the canonical data semantic representation and
 * subsequently to the repository's canonical IR boundary.
 *
 * The query grammar does NOT define a second query IR.
 *
 * Recommended logical lowering:
 *
 *     query
 *       |
 *       +--> source
 *       +--> bindings
 *       +--> projection
 *       +--> predicate
 *       +--> grouping
 *       +--> aggregation
 *       +--> ordering
 *       +--> windowing
 *       +--> set operations
 *       |
 *       v
 *     logical data plan
 *       |
 *       v
 *     canonical IR
 *
 * ============================================================================
 * ANTLR INTEGRATION
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * The shared Zamani lexer owns tokens.
 *
 * The grammar therefore uses:
 *
 *     tokenVocab = Zamani;
 *
 * No lexer rules are defined here.
 *
 * ============================================================================
 */

parser grammar ZamaniDataQueriesParser;

options {
    tokenVocab = Zamani;
}


/*
 * ============================================================================
 * 1. PUBLIC INTEGRATION BOUNDARY
 * ============================================================================
 *
 * This is the single public rule consumed by data.g4 and other domains.
 *
 * data.g4 MUST delegate query syntax to:
 *
 *     dataQueryConstruct
 *
 * It MUST NOT duplicate the query alternatives defined below.
 *
 * ============================================================================
 */

dataQueryConstruct
    : dataQueryDeclaration
    | dataQueryStatement
    | dataQueryExpression
    ;


/*
 * ============================================================================
 * 2. QUERY DECLARATIONS
 * ============================================================================
 *
 * A query declaration gives reusable logical query semantics a name.
 *
 * It does not create a database view or physical execution plan by itself.
 * ============================================================================
 */

dataQueryDeclaration
    : visibilityModifier?
      'query'
      IDENTIFIER
      genericParameters?
      dataQueryParameterList?
      dataQueryReturnType?
      dataQueryAttributes?
      '='
      dataQueryExpression
      ';'
    ;

dataQueryParameterList
    : '('
      parameterList?
      ')'
    ;

dataQueryReturnType
    : '->'
      typeExpr
    ;

dataQueryAttributes
    : '['
      dataQueryAttribute*
      ']'
    ;

dataQueryAttribute
    : annotation
    | dataQueryProperty
    ;

dataQueryProperty
    : IDENTIFIER
      ('=' expression)?
    ;


/*
 * ============================================================================
 * 3. QUERY STATEMENTS
 * ============================================================================
 */

dataQueryStatement
    : dataQueryExpression
      ';'
    ;


/*
 * ============================================================================
 * 4. QUERY EXPRESSION
 * ============================================================================
 *
 * WITH is represented separately so recursive/common-table-expression
 * semantics remain explicit.
 * ============================================================================
 */

dataQueryExpression
    : dataWithQueryExpression
    | dataSetQueryExpression
    | dataSelectQueryExpression
    ;


/*
 * ============================================================================
 * 5. WITH / COMMON TABLE EXPRESSIONS
 * ============================================================================
 *
 * A CTE is a logical binding.
 *
 * It does not imply:
 *
 *     materialization
 *     storage
 *     caching
 *     persistence
 *     physical temporary tables
 *
 * The optimizer may materialize or inline it when semantic equivalence
 * permits.
 * ============================================================================
 */

dataWithQueryExpression
    : 'with'
      dataRecursiveModifier?
      dataCommonTableExpression
      (',' dataCommonTableExpression)*
      dataQueryBody
    ;

dataRecursiveModifier
    : 'recursive'
    ;

dataCommonTableExpression
    : IDENTIFIER
      dataQueryColumnAliasList?
      'as'
      '('
      dataQueryExpression
      ')'
    ;

dataQueryColumnAliasList
    : '('
      IDENTIFIER
      (',' IDENTIFIER)*
      ')'
    ;

dataQueryBody
    : dataSetQueryExpression
    | dataSelectQueryExpression
    ;


/*
 * ============================================================================
 * 6. SELECT QUERY
 * ============================================================================
 */

dataSelectQueryExpression
    : 'select'
      dataSelectModifier*
      dataProjectionList
      dataFromClause?
      dataQueryClause*
    ;

dataSelectModifier
    : 'distinct'
    | 'all'
    ;


/*
 * ============================================================================
 * 7. PROJECTION
 * ============================================================================
 *
 * Projection expressions are ordinary Zamani expressions.
 *
 * The wildcard form is represented explicitly as a query projection item.
 * ============================================================================
 */

dataProjectionList
    : dataProjection
      (',' dataProjection)*
    ;

dataProjection
    : dataProjectionWildcard
    | dataProjectionExpression
    ;

dataProjectionWildcard
    : '*'
    | dataQualifiedWildcard
    ;

dataQualifiedWildcard
    : qualifiedDataQueryName
      '.'
      '*'
    ;

dataProjectionExpression
    : expression
      dataProjectionAlias?
    ;

dataProjectionAlias
    : 'as'
      IDENTIFIER
    ;


/*
 * ============================================================================
 * 8. FROM
 * ============================================================================
 */

dataFromClause
    : 'from'
      dataFromSource
      (',' dataFromSource)*
    ;

dataFromSource
    : dataQuerySourceExpression
      dataQuerySourceAlias?
      dataJoinClause*
    ;

dataQuerySourceAlias
    : ('as')?
      IDENTIFIER
    ;


/*
 * ============================================================================
 * 9. QUERY SOURCES
 * ============================================================================
 *
 * A source is a logical data expression.
 *
 * It may represent:
 *
 *     collection
 *     stream
 *     dataset
 *     view
 *     source
 *     pipeline
 *     subquery
 *     table-like logical abstraction
 *     future data abstraction
 *
 * Physical source interpretation belongs downstream.
 * ============================================================================
 */

dataQuerySourceExpression
    : dataQuerySourceReference
    | dataQuerySubquerySource
    | dataQueryValuesSource
    | dataQueryFunctionSource
    | dataQuerySourceParenthesized
    ;

dataQuerySourceReference
    : qualifiedDataQueryName
    ;

dataQuerySubquerySource
    : '('
      dataQueryExpression
      ')'
    ;

dataQueryValuesSource
    : 'values'
      dataValuesRowList
    ;

dataQueryFunctionSource
    : qualifiedDataQueryName
      '('
      argumentList?
      ')'
    ;

dataQuerySourceParenthesized
    : '('
      dataQuerySourceExpression
      ')'
    ;


/*
 * ============================================================================
 * 10. VALUES
 * ============================================================================
 *
 * VALUES is a logical relation/value source.
 *
 * It is not a fixed-size table.
 * ============================================================================
 */

dataValuesRowList
    : dataValuesRow
      (',' dataValuesRow)*
    ;

dataValuesRow
    : '('
      dataValuesItemList?
      ')'
    ;

dataValuesItemList
    : expression
      (',' expression)*
    ;


/*
 * ============================================================================
 * 11. JOIN
 * ============================================================================
 *
 * Join semantics are logical.
 *
 * Physical join strategy:
 *
 *     hash join
 *     merge join
 *     nested-loop join
 *     distributed join
 *     accelerator join
 *     future strategy
 *
 * is determined downstream.
 * ============================================================================
 */

dataJoinClause
    : dataJoinType?
      'join'
      dataQuerySourceExpression
      dataQuerySourceAlias?
      dataJoinCondition?
    ;

dataJoinType
    : 'inner'
    | 'left'
    | 'right'
    | 'full'
    | 'cross'
    | 'semi'
    | 'anti'
    ;

dataJoinCondition
    : 'on'
      expression
    | 'using'
      '('
      dataJoinColumnList
      ')'
    ;

dataJoinColumnList
    : IDENTIFIER
      (',' IDENTIFIER)*
    ;


/*
 * ============================================================================
 * 12. QUERY CLAUSES
 * ============================================================================
 */

dataQueryClause
    : dataWhereClause
    | dataGroupByClause
    | dataHavingClause
    | dataWindowClause
    | dataQualifyClause
    | dataOrderByClause
    | dataLimitClause
    | dataOffsetClause
    | dataQueryRequirementClause
    | dataQueryConstraintClause
    | dataQueryPreferenceClause
    | dataQueryHintClause
    ;


/*
 * ============================================================================
 * 13. WHERE
 * ============================================================================
 */

dataWhereClause
    : 'where'
      expression
    ;


/*
 * ============================================================================
 * 14. GROUP BY
 * ============================================================================
 */

dataGroupByClause
    : 'group'
      'by'
      dataGroupByItemList
      dataGroupingOptions?
    ;

dataGroupByItemList
    : expression
      (',' expression)*
    ;

dataGroupingOptions
    : '['
      dataGroupingOption*
      ']'
    ;

dataGroupingOption
    : 'rollup'
    | 'cube'
    | 'grouping_sets'
    | dataQueryOption
    ;


/*
 * ============================================================================
 * 15. HAVING
 * ============================================================================
 */

dataHavingClause
    : 'having'
      expression
    ;


/*
 * ============================================================================
 * 16. WINDOW
 * ============================================================================
 *
 * Window declarations are logical query semantics.
 * ============================================================================
 */

dataWindowClause
    : 'window'
      dataQueryWindowDefinitionList
    ;

dataQueryWindowDefinitionList
    : dataQueryWindowDefinition
      (',' dataQueryWindowDefinition)*
    ;

dataQueryWindowDefinition
    : IDENTIFIER
      'as'
      dataQueryWindowSpecification
    ;

dataQueryWindowSpecification
    : dataQueryWindowPartition?
      dataQueryWindowOrder?
      dataQueryWindowFrame?
    ;

dataQueryWindowPartition
    : 'partition'
      'by'
      dataQueryExpressionList
    ;

dataQueryWindowOrder
    : 'order'
      'by'
      dataSortKeyList
    ;

dataQueryWindowFrame
    : 'rows'
      dataQueryWindowFrameExtent
    | 'range'
      dataQueryWindowFrameExtent
    | 'groups'
      dataQueryWindowFrameExtent
    ;

dataQueryWindowFrameExtent
    : 'between'
      dataQueryWindowFrameBound
      'and'
      dataQueryWindowFrameBound
    | dataQueryWindowFrameBound
    ;

dataQueryWindowFrameBound
    : 'unbounded'
      dataQueryWindowFrameDirection?
    | 'current'
      'row'
    | expression
      dataQueryWindowFrameDirection
    ;

dataQueryWindowFrameDirection
    : 'preceding'
    | 'following'
    ;


/*
 * ============================================================================
 * 17. QUALIFY
 * ============================================================================
 *
 * QUALIFY filters after window expressions but before final projection/order
 * realization according to the semantic query model.
 *
 * Exact evaluation order belongs to semantics, not parser structure.
 * ============================================================================
 */

dataQualifyClause
    : 'qualify'
      expression
    ;


/*
 * ============================================================================
 * 18. ORDER BY
 * ============================================================================
 */

dataOrderByClause
    : 'order'
      'by'
      dataSortKeyList
    ;

dataSortKeyList
    : dataSortKey
      (',' dataSortKey)*
    ;

dataSortKey
    : expression
      dataSortDirection?
      dataSortNullPolicy?
    ;

dataSortDirection
    : 'ascending'
    | 'descending'
    | 'asc'
    | 'desc'
    ;

dataSortNullPolicy
    : 'nulls_first'
    | 'nulls_last'
    ;


/*
 * ============================================================================
 * 19. LIMIT / OFFSET
 * ============================================================================
 *
 * These are expressions rather than fixed grammar constants.
 * ============================================================================
 */

dataLimitClause
    : 'limit'
      expression
    ;

dataOffsetClause
    : 'offset'
      expression
    ;


/*
 * ============================================================================
 * 20. QUERY RESOURCE / CAPABILITY CONTRACTS
 * ============================================================================
 *
 * Query-level resource intent is represented as ordinary expressions.
 *
 * This allows:
 *
 *     requires(capability("distributed.query"))
 *
 *     prefers(capability("accelerated.query"))
 *
 *     constraint(latency < target)
 *
 * without introducing hardware-specific grammar.
 * ============================================================================
 */

dataQueryRequirementClause
    : 'requires'
      '('
      expression
      ')'
    ;

dataQueryConstraintClause
    : 'constraint'
      '('
      expression
      ')'
    ;

dataQueryPreferenceClause
    : 'prefers'
      '('
      expression
      ')'
    ;

dataQueryHintClause
    : 'hint'
      '('
      expression
      ')'
    ;


/*
 * ============================================================================
 * 21. SET OPERATIONS
 * ============================================================================
 *
 * Set operations are represented structurally so semantic analysis can
 * validate schema compatibility.
 *
 * No implementation strategy is implied.
 * ============================================================================
 */

dataSetQueryExpression
    : dataSelectQueryExpression
      dataSetOperation*
    ;

dataSetOperation
    : dataSetOperationKind
      dataSetOperationModifier?
      dataSelectQueryExpression
    ;

dataSetOperationKind
    : 'union'
    | 'intersect'
    | 'except'
    | 'difference'
    ;

dataSetOperationModifier
    : 'all'
    | 'distinct'
    ;


/*
 * ============================================================================
 * 22. QUERY NAMES
 * ============================================================================
 *
 * Query names use the shared identifier model.
 *
 * No separate lexer is introduced.
 * ============================================================================
 */

qualifiedDataQueryName
    : IDENTIFIER
      ('::' IDENTIFIER)*
    ;


/*
 * ============================================================================
 * 23. QUERY EXPRESSION LIST
 * ============================================================================
 */

dataQueryExpressionList
    : expression
      (',' expression)*
    ;


/*
 * ============================================================================
 * 24. PREDICATE / SUBQUERY OPERATORS
 * ============================================================================
 *
 * These rules expose structural query constructs without replacing the
 * canonical expression system.
 *
 * Implementations may lower these forms into ordinary semantic expressions.
 * ============================================================================
 */

dataPredicateExpression
    : dataExistsPredicate
    | dataInPredicate
    | dataBetweenPredicate
    | dataLikePredicate
    | dataIsPredicate
    | dataQuantifiedPredicate
    ;

dataExistsPredicate
    : 'exists'
      '('
      dataQueryExpression
      ')'
    ;

dataInPredicate
    : expression
      'in'
      '('
      dataInPredicateSource
      ')'
    ;

dataInPredicateSource
    : dataQueryExpression
    | dataExpressionList
    ;

dataBetweenPredicate
    : expression
      'between'
      expression
      'and'
      expression
    ;

dataLikePredicate
    : expression
      'like'
      expression
    ;

dataIsPredicate
    : expression
      'is'
      dataIsPredicateValue
    ;

dataIsPredicateValue
    : 'null'
    | 'true'
    | 'false'
    | 'unknown'
    ;

dataQuantifiedPredicate
    : expression
      dataQuantifier
      '('
      dataQueryExpression
      ')'
    ;

dataQuantifier
    : 'any'
    | 'some'
    | 'all'
    ;


/*
 * ============================================================================
 * 25. SCALAR / TABLE SUBQUERIES
 * ============================================================================
 *
 * Subqueries remain logical.
 *
 * Their execution may be:
 *
 *     inline
 *     cached
 *     distributed
 *     materialized
 *     streamed
 *     optimized away
 *
 * when semantics permit.
 * ============================================================================
 */

dataScalarSubquery
    : '('
      dataQueryExpression
      ')'
    ;

dataQuerySubqueryExpression
    : dataScalarSubquery
    ;


/*
 * ============================================================================
 * 26. QUERY OPTIONS
 * ============================================================================
 *
 * Generic options provide forward compatibility without embedding provider
 * implementations.
 *
 * Unknown options must be handled by semantic/version policy.
 * They must never silently alter semantics.
 * ============================================================================
 */

dataQueryOptionBlock
    : '['
      dataQueryOption*
      ']'
    ;

dataQueryOption
    : IDENTIFIER
      ('=' expression)?
    ;


/*
 * ============================================================================
 * 27. QUERY CONTRACTS
 * ============================================================================
 *
 * Query contracts describe semantic requirements/guarantees.
 *
 * They do not execute validation.
 * ============================================================================
 */

dataQueryContractBlock
    : '{'
      dataQueryContractMember*
      '}'
    ;

dataQueryContractMember
    : dataQueryRequiresContract
    | dataQueryEnsuresContract
    | dataQueryInvariantContract
    | dataQueryContractAttribute
    ;

dataQueryRequiresContract
    : 'requires'
      '('
      expression
      ')'
      ';'
    ;

dataQueryEnsuresContract
    : 'ensures'
      '('
      expression
      ')'
      ';'
    ;

dataQueryInvariantContract
    : 'invariant'
      '('
      expression
      ')'
      ';'
    ;

dataQueryContractAttribute
    : annotation
    ;


/*
 * ============================================================================
 * 28. QUERY PARAMETERS
 * ============================================================================
 *
 * Query parameters use the canonical function/type parameter model.
 *
 * No parameter-count limit is encoded.
 * ============================================================================
 */

dataQueryParameterExpression
    : ':'
      IDENTIFIER
    ;

dataQueryParameterListExpression
    : '('
      dataQueryParameterExpression
      (',' dataQueryParameterExpression)*
      ')'
    ;


/*
 * ============================================================================
 * 29. DATA QUERY COMPOSITION
 * ============================================================================
 *
 * A query may consume a logical result of another data computation.
 *
 * This rule intentionally remains generic.
 * ============================================================================
 */

dataQueryComposition
    : dataQueryExpression
      ('|>' dataQueryQueryStage)+
    ;

dataQueryQueryStage
    : dataSelectQueryExpression
    | dataQueryExpression
    ;


/*
 * ============================================================================
 * 30. QUERY MATERIALIZATION INTENT
 * ============================================================================
 *
 * Materialization is deliberately represented as an explicit operation.
 *
 * It does not mean:
 *
 *     RAM
 *     disk
 *     database
 *     object store
 *     GPU memory
 *     QPU memory
 *
 * Physical realization remains downstream.
 * ============================================================================
 */

dataQueryMaterialization
    : 'materialize'
      '('
      dataQueryExpression
      dataQueryMaterializationOptions?
      ')'
    ;

dataQueryMaterializationOptions
    : '['
      dataQueryOption*
      ']'
    ;


/*
 * ============================================================================
 * 31. QUERY VALIDATION INTENT
 * ============================================================================
 */

dataQueryValidation
    : 'validate'
      '('
      dataQueryExpression
      dataQueryValidationOptions?
      ')'
    ;

dataQueryValidationOptions
    : '['
      dataQueryOption*
      ']'
    ;


/*
 * ============================================================================
 * 32. QUERY PROVENANCE / LINEAGE INTENT
 * ============================================================================
 *
 * Provenance is logical metadata.
 *
 * It must not expose secrets merely because a query references them.
 * ============================================================================
 */

dataQueryProvenance
    : 'provenance'
      '('
      dataQueryExpression
      ')'
    ;


/*
 * ============================================================================
 * 33. QUERY FAILURE / RESILIENCE INTENT
 * ============================================================================
 *
 * Failure handling is expressed semantically.
 *
 * Actual retry/recovery execution remains downstream.
 * ============================================================================
 */

dataQueryFailurePolicy
    : 'on_failure'
      '('
      expression
      ')'
    ;


/*
 * ============================================================================
 * 34. DATA EXPRESSION BRIDGE
 * ============================================================================
 *
 * This bridge permits query grammar to consume the existing data expression
 * model without redefining that model.
 *
 * The root data grammar may map this rule to its canonical dataExpression.
 *
 * Until that facade migration is complete, semantic integration should map
 * logical data references and expressions through the existing data expression
 * contract.
 * ============================================================================
 */

dataQueryValueExpression
    : expression
    ;


/*
 * ============================================================================
 * 35. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing query syntax in data.g4 must be migrated into this file.
 *
 * Specifically, the following conceptual rule currently exists in data.g4:
 *
 *     dataQueryStmt
 *         -> dataQueryExpression
 *         -> dataSelectExpression
 *         -> dataQueryClause
 *
 * The production migration is:
 *
 *     data.g4
 *          |
 *          v
 *     dataQueryConstruct
 *          |
 *          v
 *     ZamaniDataQueriesParser
 *
 * The old duplicated query productions must then be removed from data.g4.
 *
 * No public source construct should require a source-level rewrite solely
 * because the query rules moved from the integration facade into this
 * specialized grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 36. DATABASE COMPATIBILITY
 * ============================================================================
 *
 * SQL-compatible lowering is permitted.
 *
 * SQL is NOT the canonical Zamani query language.
 *
 * Therefore:
 *
 *     Zamani query
 *         |
 *         v
 * logical query semantics
 *         |
 *         +--> SQL
 *         +--> in-memory plan
 *         +--> distributed plan
 *         +--> accelerator plan
 *         +--> streaming plan
 *         +--> future backend
 *
 * A SQL backend may reject constructs that cannot be represented by its
 * capability set. That is a backend capability issue, not a language limit.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 37. CLASSICAL COMPUTING INTEGRATION
 * ============================================================================
 *
 * Query expressions may consume classical data:
 *
 *     scalars
 *     vectors
 *     matrices
 *     tensors
 *     records
 *     collections
 *     streams
 *
 * Numeric semantics remain owned by the canonical type system and classical
 * data subsystem.
 *
 * The query grammar does not define numeric representation widths.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 38. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum results may become queryable data.
 *
 * Examples include:
 *
 *     measurement results
 *     experiment records
 *     parameter sweeps
 *     optimization results
 *     calibration observations
 *     statistical results
 *
 * Quantum execution remains owned by the quantum subsystem.
 *
 * Quantum semantics lower through the canonical:
 *
 *     quantum::ir
 *
 * Query grammar MUST NOT define a second quantum IR.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 39. AI / ML INTEGRATION
 * ============================================================================
 *
 * Query syntax may consume:
 *
 *     datasets
 *     tensors
 *     embeddings
 *     model outputs
 *     feature records
 *     training data
 *     inference results
 *
 * AI-specific model semantics remain owned by grammar/ai/.
 *
 * The query grammar does not define neural-network architecture syntax.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 40. HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Query results may feed hardware/software co-design.
 *
 * The query grammar MUST NOT define:
 *
 *     register width
 *     bus width
 *     FPGA count
 *     accelerator count
 *     physical memory
 *     physical address
 *
 * Those are owned by hardware/HDL/resource semantics.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 41. DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Queries may be distributed.
 *
 * The query grammar does not specify:
 *
 *     number of nodes
 *     number of workers
 *     partition count
 *     network topology
 *     replica count
 *
 * Distribution semantics are expressed through logical requirements and
 * lowered later.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 42. SECURITY INTEGRATION
 * ============================================================================
 *
 * Query syntax may participate in capability and policy checking.
 *
 * Examples:
 *
 *     requires(capability("data.read"))
 *     requires(capability("data.query"))
 *
 * Actual authorization remains outside the parser.
 *
 * Diagnostics must not reveal secret data merely because a query failed.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 43. RESOURCE EXHAUSTION
 * ============================================================================
 *
 * A query may be arbitrarily large at the language level.
 *
 * Implementations may reject execution because resources are unavailable.
 *
 * That rejection is a runtime/compiler/resource decision, not a grammar
 * maximum.
 *
 * Example:
 *
 *     query too large for current compilation budget
 *
 * is different from:
 *
 *     Zamani queries may never contain more than N joins.
 *
 * The latter is forbidden.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 44. ERROR / DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Syntax diagnostics should identify:
 *
 *     - unexpected token;
 *     - missing clause;
 *     - malformed source;
 *     - malformed join;
 *     - malformed grouping;
 *     - malformed ordering;
 *     - malformed window;
 *     - malformed set operation;
 *     - malformed subquery;
 *     - malformed CTE.
 *
 * Semantic diagnostics belong downstream and include:
 *
 *     - unknown source;
 *     - unknown field;
 *     - incompatible schemas;
 *     - invalid join types;
 *     - invalid grouping;
 *     - invalid aggregation;
 *     - invalid ordering;
 *     - invalid window;
 *     - invalid type;
 *     - unavailable capability;
 *     - unsatisfied resource requirement;
 *     - invalid effect;
 *     - ownership/lifetime violation;
 *     - nondeterministic behavior where determinism is required.
 *
 * The grammar MUST NOT attempt to perform those semantic checks.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 45. SOURCE SPANS
 * ============================================================================
 *
 * Every parsed query construct must remain traceable to its source span.
 *
 * This is supplied by the shared parser/AST infrastructure.
 *
 * This grammar does not define a second span type.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 46. NO HARD-CODED SCALE
 * ============================================================================
 *
 * The following must remain unbounded at the grammar level:
 *
 *     projection count
 *     source count
 *     join count
 *     grouping key count
 *     ordering key count
 *     CTE count
 *     set-operation count
 *     expression depth
 *     parameter count
 *     window definitions
 *     query stages
 *     query nesting
 *
 * Practical limits are determined by:
 *
 *     parser resources
 *     compiler resources
 *     runtime resources
 *     explicit semantic requirements
 *     available capabilities
 *     deployment environment
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 47. DETERMINISM AND OPTIMIZATION
 * ============================================================================
 *
 * Optimizers may:
 *
 *     reorder predicates;
 *     fuse operations;
 *     eliminate redundant projections;
 *     push filters;
 *     push projections;
 *     eliminate unused CTEs;
 *     inline CTEs;
 *     choose alternate join algorithms;
 *     distribute operations;
 *     vectorize operations;
 *     specialize operations;
 *
 * only when semantic equivalence is preserved.
 *
 * Query syntax therefore describes semantics, not implementation order,
 * except where explicit ordering is observable.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 48. SECURITY / TRUST BOUNDARY
 * ============================================================================
 *
 * This grammar is declarative and must remain side-effect free.
 *
 * No parser action may:
 *
 *     - execute a query;
 *     - connect to a database;
 *     - open a file;
 *     - contact a network endpoint;
 *     - discover hardware;
 *     - allocate runtime resources;
 *     - access credentials;
 *     - inspect secrets.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 49. TEST CONTRACT
 * ============================================================================
 *
 * Production tests must cover at least:
 *
 * POSITIVE
 *
 *     simple select
 *     select expression
 *     select wildcard
 *     qualified wildcard
 *     aliases
 *     from source
 *     multiple sources
 *     inner join
 *     outer joins
 *     semi join
 *     anti join
 *     cross join
 *     where
 *     group by
 *     having
 *     order by
 *     limit
 *     offset
 *     distinct
 *     qualify
 *     windows
 *     CTE
 *     recursive CTE
 *     subquery
 *     scalar subquery
 *     exists
 *     in
 *     between
 *     like
 *     set operations
 *     values
 *     query declaration
 *     query parameters
 *     resource requirements
 *     constraints
 *     preferences
 *     hints
 *
 * NEGATIVE
 *
 *     malformed SELECT
 *     missing FROM source where required by semantic rules
 *     malformed JOIN
 *     malformed ON
 *     malformed USING
 *     malformed GROUP BY
 *     malformed ORDER BY
 *     malformed window
 *     malformed CTE
 *     malformed set operation
 *     malformed subquery
 *     malformed VALUES
 *     malformed aliases
 *
 * BOUNDARY
 *
 *     empty source
 *     empty projection
 *     singleton projection
 *     nested query
 *     deeply nested query
 *     many joins
 *     many grouping keys
 *     many ordering keys
 *     many CTEs
 *     symbolic limits
 *     symbolic offsets
 *     symbolic window sizes
 *
 * SCALABILITY
 *
 *     tiny query
 *     large query
 *     generated query
 *     large logical schema
 *     large logical result
 *     distributed result
 *     heterogeneous result
 *
 * DETERMINISM
 *
 *     ordered query
 *     unordered query
 *     equivalent optimizer transformations
 *     distributed execution preserving declared order
 *
 * PORTABILITY
 *
 *     local
 *     distributed
 *     accelerator
 *     database-backed
 *     streaming
 *     quantum-result-backed
 *     future backend
 *
 * SECURITY
 *
 *     capability denial
 *     protected source
 *     protected field
 *     sensitive diagnostic handling
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 50. FEATURE TRACEABILITY
 * ============================================================================
 *
 * Query feature lifecycle:
 *
 *     grammar/spec/data.md
 *          |
 *          v
 *     grammar/data/queries.g4
 *          |
 *          v
 *     grammar/data/data.g4
 *          |
 *          v
 *     grammar/Zamani.g4
 *          |
 *          v
 *     parser / AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     canonical data semantic model
 *          |
 *          v
 *     canonical IR
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     runtime
 *
 * No feature is considered production-complete until every required boundary
 * has a defined owner.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 51. INTEGRATION ACTIONS REQUIRED OUTSIDE THIS FILE
 * ============================================================================
 *
 * This file is independently complete as the canonical query syntax contract.
 *
 * The following repository integrations are required, but MUST NOT be
 * performed by modifying the ownership of this file:
 *
 * 1. grammar/data/data.g4
 *
 * Replace its duplicated query implementation with delegation to:
 *
 *     dataQueryConstruct
 *
 * The old:
 *
 *     dataQueryStmt
 *     dataQueryExpression
 *     dataSelectExpression
 *     dataQueryClause
 *
 * implementation must not remain as a competing query grammar.
 *
 *
 * 2. grammar/Zamani.g4
 *
 * The root grammar must expose one data entry point.
 *
 * Conceptually:
 *
 *     statement
 *         : ...
 *         | dataStmt
 *         | ...
 *
 * Data query syntax must then enter through:
 *
 *     dataStmt
 *         -> dataQueryConstruct
 *
 *
 * 3. grammar/grammar.md
 *
 * Add the query rules to implementation-conformance tracking:
 *
 *     SPECIFIED
 *     IMPLEMENTED
 *     PARTIALLY IMPLEMENTED
 *     PLANNED
 *     DEPRECATED
 *
 *
 * 4. grammar/spec/data.md
 *
 * Maintain semantic authority there.
 *
 * This file owns syntax, not semantic interpretation.
 *
 *
 * 5. AST
 *
 * Map query parse nodes into the existing domain-neutral AST.
 *
 * Do not introduce a vendor-specific or database-specific AST.
 *
 *
 * 6. Semantic analysis
 *
 * Resolve:
 *
 *     source names
 *     fields
 *     aliases
 *     types
 *     joins
 *     grouping
 *     aggregation
 *     windows
 *     capabilities
 *     resources
 *     effects
 *     determinism
 *
 *
 * 7. Canonical IR
 *
 * Lower query semantics into the canonical data representation.
 *
 * Do not introduce a second query IR.
 *
 *
 * 8. Compiler
 *
 * Query optimization may choose:
 *
 *     local
 *     parallel
 *     distributed
 *     vectorized
 *     accelerator
 *     database
 *     streaming
 *     future
 *
 * implementations.
 *
 *
 * 9. Runtime
 *
 * Runtime executes the lowered plan and handles:
 *
 *     resource availability
 *     capability availability
 *     source/sink failures
 *     cancellation
 *     checkpointing
 *     recovery
 *     observability
 *
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 52. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [x] query syntax has one canonical owner;
 * [x] parser grammar is declarative;
 * [x] no unsafe Rust is required;
 * [x] no target implementation is embedded;
 * [x] no database vendor is embedded;
 * [x] no physical topology is embedded;
 * [x] no universal resource maximum is embedded;
 * [x] canonical type system is reused;
 * [x] canonical expression system is reused;
 * [x] query declarations exist;
 * [x] SELECT exists;
 * [x] FROM exists;
 * [x] JOIN exists;
 * [x] WHERE exists;
 * [x] GROUP BY exists;
 * [x] HAVING exists;
 * [x] WINDOW exists;
 * [x] QUALIFY exists;
 * [x] ORDER BY exists;
 * [x] LIMIT exists;
 * [x] OFFSET exists;
 * [x] CTE exists;
 * [x] recursive CTE syntax exists;
 * [x] subqueries exist;
 * [x] EXISTS exists;
 * [x] IN exists;
 * [x] BETWEEN exists;
 * [x] LIKE exists;
 * [x] set operations exist;
 * [x] VALUES exists;
 * [x] requirements exist;
 * [x] constraints exist;
 * [x] preferences exist;
 * [x] hints exist;
 * [x] portability boundary exists;
 * [x] quantum boundary is preserved;
 * [x] AI boundary is preserved;
 * [x] HDL boundary is preserved;
 * [x] distributed boundary is preserved;
 * [x] security boundary is preserved;
 * [x] AST contract is defined;
 * [x] IR contract is defined;
 * [x] compiler integration is defined;
 * [x] runtime integration is defined;
 * [x] test contract is defined;
 * [x] hard-coding audit is defined.
 *
 * The remaining repository work is integration/testing, not redesign of this
 * file.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 53. FINAL ARCHITECTURAL INVARIANT
 * ============================================================================
 *
 * A Zamani query describes:
 *
 *     WHAT data is wanted
 *     WHAT relationships are required
 *     WHAT transformations are required
 *     WHAT ordering is observable
 *     WHAT correctness constraints apply
 *     WHAT capabilities are required
 *     WHAT resource properties matter
 *
 * It does NOT prescribe:
 *
 *     WHICH CPU
 *     WHICH GPU
 *     WHICH FPGA
 *     WHICH QPU
 *     WHICH node
 *     WHICH database
 *     WHICH storage device
 *     WHICH memory bank
 *     WHICH physical partition
 *     WHICH network path
 *
 * Therefore:
 *
 *     QUERY INTENT
 *          |
 *          v
 *     SEMANTIC DATA MODEL
 *          |
 *          v
 *     CANONICAL IR
 *          |
 *          v
 *     OPTIMIZATION
 *          |
 *          v
 *     RESOURCE / CAPABILITY REALIZATION
 *          |
 *          v
 *     TARGET EXECUTION
 *
 * remains compatible with:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * ============================================================================
 */
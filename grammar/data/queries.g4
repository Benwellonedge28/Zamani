/*
 * ============================================================================
 * ZAMANI PROGRAMMING LANGUAGE
 * ============================================================================
 *
 * File:
 *     grammar/data/queries.g4
 *
 * Grammar:
 *     ZamaniDataQueriesParser
 *
 * Status:
 *     CANONICAL / PRODUCTION DATA-QUERY LEAF GRAMMAR
 *
 * Baseline:
 *     Rust 1.97+
 *     Rust 2021
 *     Safe Rust only
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the SINGLE SYNTAX AUTHORITY for Zamani's logical data-query
 * model.
 *
 * A query expresses logical data intent:
 *
 *     projection
 *     source selection
 *     joins
 *     filtering
 *     grouping
 *     aggregation
 *     ordering
 *     windowing
 *     pagination
 *     common-table expressions
 *     recursive query intent
 *     subqueries
 *     set composition
 *     quantified predicates
 *     query parameters
 *     logical requirements
 *     logical constraints
 *     logical preferences
 *     logical hints
 *     logical metadata
 *
 * This file does NOT define:
 *
 *     SQL
 *     database engines
 *     storage engines
 *     indexes
 *     physical partitions
 *     replicas
 *     workers
 *     threads
 *     CPU/GPU/FPGA/QPU selection
 *     network topology
 *     scheduling
 *     routing
 *     optimization algorithms
 *     execution engines
 *     hardware realization
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * A query is portable logical intent.
 *
 * The same source query may be lowered to:
 *
 *     in-memory execution
 *     collection execution
 *     table execution
 *     stream execution
 *     dataset execution
 *     graph execution
 *     tensor/data execution
 *     classical execution
 *     distributed execution
 *     accelerator execution
 *     heterogeneous execution
 *     quantum-derived data processing
 *     hybrid execution
 *     future execution substrates
 *
 * without changing the source-level query semantics.
 *
 * The grammar therefore contains NO universal capacity constants.
 *
 * Forbidden examples include:
 *
 *     MAX_ROWS
 *     MAX_COLUMNS
 *     MAX_FIELDS
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
 * Programmer-written numeric values are program data, not implementation
 * ceilings.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     dataQueryConstruct
 *     dataQueryDeclaration
 *     dataQueryStatement
 *     dataQueryExpression
 *     SELECT-style logical query structure
 *     projection
 *     FROM/source structure
 *     JOIN structure
 *     WHERE structure
 *     GROUP BY structure
 *     HAVING structure
 *     WINDOW structure
 *     QUALIFY structure
 *     ORDER BY structure
 *     LIMIT structure
 *     OFFSET structure
 *     WITH/CTE structure
 *     recursive query syntax
 *     VALUES source syntax
 *     set-query composition
 *     subquery structure
 *     quantified query predicates
 *     membership predicates
 *     query-level requirements
 *     query-level constraints
 *     query-level preferences
 *     query-level hints
 *     query-level metadata
 *     query parameters
 *
 * THIS FILE DOES NOT OWN:
 *
 *     lexer rules
 *     identifier syntax
 *     qualified-name syntax generally
 *     universal expressions
 *     universal types
 *     functions
 *     lambdas
 *     pattern matching
 *     contracts generally
 *     policies generally
 *     capabilities generally
 *     resources generally
 *     provenance implementation
 *     graph implementation
 *     collection implementation
 *     stream implementation
 *     tensor implementation
 *     SQL dialects
 *     JSON
 *     XML
 *     database implementations
 *     execution plans
 *     canonical IR
 *     classical IR
 *     quantum::ir
 *     ZQN
 *     HAL
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There is exactly ONE implementation of logical query syntax:
 *
 *     grammar/data/queries.g4
 *
 * grammar/data/data.g4 MUST delegate to:
 *
 *     dataQueryConstruct
 *
 * grammar/expressions/query.g4 MUST delegate to:
 *
 *     dataQueryExpression
 *
 * grammar/statements/query.g4, when retained, MUST remain a thin adapter and
 * MUST NOT implement query syntax independently.
 *
 * SQL, JSON, XML and provider-specific query syntaxes belong under the
 * interoperability/dialect architecture.
 *
 * ============================================================================
 * EXPRESSION INTEGRATION
 * ============================================================================
 *
 * Query values use the canonical Zamani expression model.
 *
 * This file MUST NOT define:
 *
 *     expression
 *     binaryExpression
 *     unaryExpression
 *     callExpression
 *     literalExpression
 *     lambdaExpression
 *
 * Query positions consume the canonical expression surface supplied by the
 * parser composition architecture.
 *
 * The repository must resolve the current dependency direction so that:
 *
 *     query grammar -> canonical expression surface
 *
 * and:
 *
 *     expression grammar -> query expression adapter
 *
 * do not create a circular grammar import.
 *
 * The architectural solution is a shared expression composition boundary,
 * not a second query-specific expression language.
 *
 * ============================================================================
 * TYPE INTEGRATION
 * ============================================================================
 *
 * Query declarations consume the canonical type system.
 *
 * This file does NOT define:
 *
 *     QueryType
 *     RelationType
 *     TableType
 *     DatasetType
 *
 * as competing type systems.
 *
 * Query result typing belongs to semantic analysis.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser produces parse-tree structure only.
 *
 * Recommended semantic mapping:
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
 *     dataJoinClause
 *         -> DataJoin
 *
 *     dataWhereClause
 *         -> QueryPredicate
 *
 *     dataGroupByClause
 *         -> QueryGrouping
 *
 *     dataOrderByClause
 *         -> QueryOrdering
 *
 *     dataQueryWindowDefinition
 *         -> QueryWindow
 *
 *     dataCommonTableExpression
 *         -> QueryBinding
 *
 *     dataSetOperation
 *         -> QuerySetOperation
 *
 *     dataQueryRequirement
 *         -> Requirement
 *
 *     dataQueryConstraint
 *         -> Constraint
 *
 *     dataQueryPreference
 *         -> Preference
 *
 *     dataQueryHint
 *         -> Hint
 *
 * Exact Rust AST structures remain owned by the domain-neutral frontend AST.
 *
 * This grammar MUST NOT introduce a query-specific parallel AST.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     name resolution
 *     source resolution
 *     schema compatibility
 *     field resolution
 *     type checking
 *     aggregation checking
 *     grouping validation
 *     ordering validation
 *     window validation
 *     cardinality reasoning
 *     null/absence semantics
 *     capability checking
 *     resource checking
 *     effect checking
 *     policy checking
 *     provenance checking
 *     determinism checking
 *     portability analysis
 *
 * This grammar performs none of these operations.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY CONTRACT
 * ============================================================================
 *
 * Query requirements describe intent.
 *
 * They do NOT select physical implementations.
 *
 * Examples:
 *
 *     requires(capability("data.query"))
 *     requires(capability("distributed.query"))
 *     requires(capability("tensor.query"))
 *     requires(memory >= required_memory)
 *     requires(topology(required_topology))
 *
 * Preferences and hints remain non-semantic implementation guidance unless
 * explicitly promoted to semantic constraints by the owning policy system.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing has no runtime effect.
 *
 * Semantic analysis MAY derive effects such as:
 *
 *     data.read
 *     data.write
 *     io
 *     network
 *     distributed
 *     external
 *     measurement
 *
 * according to the resolved source and semantic model.
 *
 * This grammar must not blindly assign every possible effect to every query.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Query execution may be subject to:
 *
 *     security policy
 *     data-access policy
 *     privacy policy
 *     resource policy
 *     execution policy
 *     provenance policy
 *     sandbox policy
 *
 * Authorization is downstream.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Every query construct remains source-traceable through the normal parser
 * and AST source-span infrastructure.
 *
 * Provenance may later record:
 *
 *     source
 *     transformation
 *     derivation
 *     evidence
 *     decision
 *     verification
 *     semantic version
 *
 * The parser itself does not manufacture runtime provenance.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Query parsing is deterministic.
 *
 * Query result ordering is NOT deterministic merely because a source is
 * iterable.
 *
 * Observable ordering requires an explicit semantic ordering construct.
 *
 * In particular:
 *
 *     ORDER BY
 *
 * establishes ordering intent.
 *
 * Physical execution may otherwise reorder independent/unordered portions
 * without changing observable semantics.
 *
 * ============================================================================
 * SQL BOUNDARY
 * ============================================================================
 *
 * Zamani query syntax is SQL-compatible in places where useful, but it is NOT
 * SQL.
 *
 * SQL dialects remain external representations.
 *
 * Canonical direction:
 *
 *     Zamani query
 *          |
 *          v
 *     canonical query semantics
 *          |
 *          +--> SQL dialect
 *          +--> in-memory plan
 *          +--> stream plan
 *          +--> distributed plan
 *          +--> accelerator plan
 *          +--> future backend
 *
 * A backend may reject a query because the target lacks a required capability.
 *
 * That is not a language restriction.
 *
 * ============================================================================
 * ANTLR / RUST SAFETY
 * ============================================================================
 *
 * This file contains:
 *
 *     no embedded Rust
 *     no semantic predicates
 *     no target-language actions
 *     no filesystem access
 *     no network access
 *     no runtime execution
 *     no hardware discovery
 *     no randomness
 *     no unsafe implementation
 *
 * Generated Rust remains subject to the repository's:
 *
 *     Rust 1.97+
 *     Rust 2021
 *     safe Rust only
 *
 * requirements.
 *
 * ============================================================================
 */

parser grammar ZamaniDataQueriesParser;

options {
    /*
     * Canonical lexical authority.
     *
     * The repository's current parser composition consistently uses
     * ZamaniLexer. The older `tokenVocab = Zamani` form is deliberately not
     * used here.
     */
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. PUBLIC QUERY BOUNDARY
 * ============================================================================
 *
 * data.g4 delegates here.
 *
 * This rule is intentionally small and stable.
 * ============================================================================
 */

dataQueryConstruct
    : dataQueryDeclaration
    | dataQueryStatement
    | dataQueryExpression
    ;


/*
 * ============================================================================
 * 2. QUERY DECLARATION
 * ============================================================================
 *
 * A query declaration gives reusable logical query intent a name.
 *
 * It does not imply:
 *
 *     database view
 *     materialized view
 *     physical cache
 *     persistent storage
 *     execution plan
 *
 * ============================================================================
 */

dataQueryDeclaration
    : visibilityModifier?
      'query'
      IDENTIFIER
      genericParameters?
      dataQueryParameters?
      dataQueryReturnType?
      dataQueryAttributes?
      '='
      dataQueryExpression
      ';'
    ;

dataQueryParameters
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
    | dataQueryAttributeAssignment
    ;

dataQueryAttributeAssignment
    : IDENTIFIER
      ('=' expression)?
    ;


/*
 * ============================================================================
 * 3. QUERY STATEMENT
 * ============================================================================
 *
 * This is the canonical logical query statement.
 *
 * The terminating semicolon belongs here rather than being duplicated by
 * statement-layer adapters.
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
 * A query expression is either:
 *
 *     WITH query
 *     set query
 *     SELECT query
 *
 * The grammar is structured so set operations compose recursively without
 * creating duplicate SELECT implementations.
 * ============================================================================
 */

dataQueryExpression
    : dataWithQueryExpression
    | dataSetQueryExpression
    ;


/*
 * ============================================================================
 * 5. WITH / CTE
 * ============================================================================
 */

dataWithQueryExpression
    : 'with'
      dataRecursiveModifier?
      dataCommonTableExpression
      (',' dataCommonTableExpression)*
      dataSetQueryExpression
    ;

dataRecursiveModifier
    : 'recursive'
    ;

dataCommonTableExpression
    : IDENTIFIER
      dataQueryColumnAliasList?
      'as'
      '('
      dataSetQueryExpression
      ')'
    ;

dataQueryColumnAliasList
    : '('
      IDENTIFIER
      (',' IDENTIFIER)*
      ')'
    ;


/*
 * ============================================================================
 * 6. SET QUERY
 * ============================================================================
 *
 * Set operations are left-associative at the semantic level.
 *
 * Parenthesized query operands preserve explicit grouping.
 *
 * This rule does not encode an artificial maximum nesting depth.
 * ============================================================================
 */

dataSetQueryExpression
    : dataSetQueryOperand
      dataSetOperation*
    ;

dataSetQueryOperand
    : dataSelectQueryExpression
    | dataParenthesizedQueryExpression
    ;

dataParenthesizedQueryExpression
    : '('
      dataSetQueryExpression
      ')'
    ;

dataSetOperation
    : dataSetOperationKind
      dataSetOperationModifier?
      dataSetQueryOperand
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
 * 7. SELECT QUERY
 * ============================================================================
 *
 * SELECT is used here as a logical projection query, not as a declaration of
 * a database implementation.
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
 * 8. PROJECTION
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
    : dataQueryName
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
 * 9. FROM
 * ============================================================================
 *
 * A FROM clause is optional because a logical query may project expressions
 * without a relation-like source.
 *
 * Example:
 *
 *     select expression;
 *
 * Semantic validation determines whether a particular expression is valid in
 * a source-free query.
 * ============================================================================
 */

dataFromClause
    : 'from'
      dataFromSource
      (
          ','
          dataFromSource
      )*
    ;

dataFromSource
    : dataQuerySourceExpression
      dataQuerySourceAlias?
      dataJoinClause*
    ;

dataQuerySourceAlias
    : 'as'?
      IDENTIFIER
    ;


/*
 * ============================================================================
 * 10. QUERY SOURCE
 * ============================================================================
 *
 * Sources remain logical.
 *
 * A source may denote:
 *
 *     collection
 *     table
 *     dataset
 *     stream
 *     graph
 *     view
 *     pipeline result
 *     logical source
 *     function-produced relation
 *     subquery
 *     values
 *     future data abstraction
 *
 * Physical interpretation belongs downstream.
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
    : dataQueryName
    ;

dataQuerySubquerySource
    : '('
      dataSetQueryExpression
      ')'
    ;

dataQueryValuesSource
    : 'values'
      dataValuesRowList
    ;

dataQueryFunctionSource
    : dataQueryName
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
 * 11. VALUES SOURCE
 * ============================================================================
 *
 * No fixed row or column capacity is encoded.
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
 * 12. JOIN
 * ============================================================================
 *
 * Join strategy is semantic/backend policy.
 *
 * The grammar describes only logical join intent.
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
 * 13. QUERY CLAUSES
 * ============================================================================
 *
 * Clause ordering is intentionally expressed structurally rather than by
 * allowing arbitrary keyword sequences.
 *
 * This prevents syntactically valid but semantically meaningless duplicate
 * clauses while leaving semantic evaluation order to the semantic layer.
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
 * 14. WHERE
 * ============================================================================
 */

dataWhereClause
    : 'where'
      expression
    ;


/*
 * ============================================================================
 * 15. GROUP BY
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
 * 16. HAVING
 * ============================================================================
 */

dataHavingClause
    : 'having'
      expression
    ;


/*
 * ============================================================================
 * 17. WINDOW
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
 * 18. QUALIFY
 * ============================================================================
 */

dataQualifyClause
    : 'qualify'
      expression
    ;


/*
 * ============================================================================
 * 19. ORDER BY
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
 * 20. LIMIT / OFFSET
 * ============================================================================
 *
 * Both operands are ordinary expressions.
 *
 * This permits:
 *
 *     limit result_count
 *     offset starting_position
 *
 * without introducing language-level numeric ceilings.
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
 * 21. QUERY REQUIREMENTS
 * ============================================================================
 *
 * These are logical requirements.
 *
 * They are intentionally expressed through the universal expression system.
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
 * 22. QUERY PREDICATE STRUCTURES
 * ============================================================================
 *
 * These rules are query-specific structural forms.
 *
 * They are NOT automatically inserted into the universal expression
 * precedence hierarchy.
 *
 * The semantic/query-expression integration layer may lower them into the
 * canonical expression model.
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
      dataSetQueryExpression
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
    : dataSetQueryExpression
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
      dataSetQueryExpression
      ')'
    ;

dataQuantifier
    : 'any'
    | 'some'
    | 'all'
    ;


/*
 * ============================================================================
 * 23. QUERY VALUE LIST
 * ============================================================================
 */

dataExpressionList
    : expression
      (',' expression)*
    ;


/*
 * ============================================================================
 * 24. QUERY NAMES
 * ============================================================================
 *
 * Query names are intentionally open-world.
 *
 * They may identify:
 *
 *     user declarations
 *     imported data abstractions
 *     collections
 *     tables
 *     streams
 *     datasets
 *     graphs
 *     views
 *     functions
 *     dialect-provided logical sources
 *     future data abstractions
 *
 * No vendor catalogue is encoded.
 * ============================================================================
 */

dataQueryName
    : IDENTIFIER
      (
          '::'
          IDENTIFIER
      )*
    ;


/*
 * ============================================================================
 * 25. QUERY OPTIONS
 * ============================================================================
 *
 * Options are intentionally name-based.
 *
 * An unknown option is NOT silently given semantics by this grammar.
 *
 * Semantic/version policy determines whether a given option is:
 *
 *     recognized
 *     deprecated
 *     experimental
 *     unsupported
 *     invalid
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
 * 26. QUERY PARAMETERS
 * ============================================================================
 *
 * Query declarations use the canonical parameter grammar.
 *
 * The named expression form is retained for query-specific parameter
 * references where required by semantic lowering.
 * ============================================================================
 */

dataQueryParameterExpression
    : ':'
      IDENTIFIER
    ;


/*
 * ============================================================================
 * 27. QUERY COMPOSITION
 * ============================================================================
 *
 * Composition is deliberately generic.
 *
 * It does not enumerate transformation algorithms.
 *
 * Pipeline/transformation semantics remain owned by their respective
 * grammars.
 * ============================================================================
 */

dataQueryComposition
    : dataQueryExpression
      (
          '|>'
          dataQueryStage
      )+
    ;

dataQueryStage
    : dataQueryName
      (
          '('
          argumentList?
          ')'
      )?
    ;


/*
 * ============================================================================
 * 28. QUERY MATERIALIZATION
 * ============================================================================
 *
 * This is logical intent only.
 *
 * It does not select RAM, storage, database, accelerator memory, or any
 * physical resource.
 * ============================================================================
 */

dataQueryMaterialization
    : 'materialize'
      '('
      dataSetQueryExpression
      dataQueryOptionBlock?
      ')'
    ;


/*
 * ============================================================================
 * 29. QUERY VALIDATION
 * ============================================================================
 */

dataQueryValidation
    : 'validate'
      '('
      dataSetQueryExpression
      dataQueryOptionBlock?
      ')'
    ;


/*
 * ============================================================================
 * 30. QUERY PROVENANCE
 * ============================================================================
 *
 * Provenance is a semantic metadata request.
 *
 * Actual provenance generation belongs to the semantic/compiler/runtime
 * provenance infrastructure.
 * ============================================================================
 */

dataQueryProvenance
    : 'provenance'
      '('
      dataSetQueryExpression
      ')'
    ;


/*
 * ============================================================================
 * 31. QUERY FAILURE INTENT
 * ============================================================================
 *
 * Failure policy is logical metadata.
 *
 * Retry/recovery implementation remains downstream.
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
 * 32. QUERY CONTRACT METADATA
 * ============================================================================
 *
 * Query contracts are represented here only when attached directly to query
 * syntax. General contracts remain owned by grammar/validation/.
 *
 * The universal contract system remains the semantic authority.
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
    | annotation
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


/*
 * ============================================================================
 * 33. FINAL QUERY COMPOSITION INVARIANTS
 * ============================================================================
 *
 * 1. dataQueryConstruct is the canonical data-domain query entry point.
 *
 * 2. dataQueryExpression is the canonical logical query expression.
 *
 * 3. SELECT/FROM/JOIN/etc. are owned only here.
 *
 * 4. data.g4 delegates instead of duplicating query syntax.
 *
 * 5. expressions/query.g4 delegates instead of duplicating query syntax.
 *
 * 6. statements/query.g4, if retained, is an adapter only.
 *
 * 7. SQL is not the canonical language.
 *
 * 8. Provider names are not grammar-level query constructs.
 *
 * 9. Query operations remain open-world.
 *
 * 10. Query source names remain open-world.
 *
 * 11. General expressions remain owned by the universal expression system.
 *
 * 12. General types remain owned by the universal type system.
 *
 * 13. Resource availability is not parser validity.
 *
 * 14. Capability discovery is not parser validity.
 *
 * 15. Hardware selection is not parser validity.
 *
 * 16. Query size has no language-defined universal ceiling.
 *
 * 17. No physical execution strategy is encoded.
 *
 * 18. No quantum-specific implementation is encoded.
 *
 * 19. Quantum-derived data remains ordinary semantic data until the semantic
 *     pipeline identifies quantum provenance/requirements.
 *
 * 20. No second data IR is created.
 *
 * 21. No query-specific Rust implementation is embedded in this grammar.
 *
 * 22. Generated Rust remains safe Rust.
 *
 * 23. Rust 1.97+ remains supported.
 *
 * 24. The grammar remains target-independent.
 *
 * 25. The grammar remains suitable for classical, AI, tensor, graph,
 *     distributed, hybrid and quantum-derived data.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] one canonical query grammar exists;
 *     [x] data.g4 delegates to it;
 *     [x] expression/query delegates to it;
 *     [x] statement/query does not duplicate it;
 *     [x] the lexer remains the single lexical authority;
 *     [x] general expressions remain outside this file;
 *     [x] general types remain outside this file;
 *     [x] SQL remains outside the core grammar;
 *     [x] physical execution remains outside the grammar;
 *     [x] no machine-capacity constants exist;
 *     [x] no domain-specific resource ceilings exist;
 *     [x] no embedded Rust exists;
 *     [x] no unsafe implementation is required;
 *     [x] source-level query semantics remain target-independent;
 *     [x] the query model can participate in classical/AI/graph/tensor/
 *         distributed/hybrid/quantum-derived computation;
 *     [x] downstream semantic, capability, resource, effect, policy and
 *         provenance analysis remain authoritative.
 *
 * ============================================================================
 */
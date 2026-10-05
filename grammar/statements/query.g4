/*
 * ============================================================================
 * ZAMANI UNIVERSAL COMPUTING LANGUAGE
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/statements/query.g4
 *
 * STATUS
 * ------
 * CANONICAL STATEMENT-LEVEL QUERY ADAPTER
 *
 * GRAMMAR TECHNOLOGY
 * ------------------
 * ANTLR4 parser grammar
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 1.97 / Rust 1.97.1
 * Rust 2021
 * Safe Rust only.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the statement-level QUERY ADAPTER boundary.
 *
 * IMPORTANT:
 *
 * This file does NOT implement a second query language.
 *
 * The canonical logical data-query syntax is owned by:
 *
 *     grammar/data/queries.g4
 *
 * The canonical expression-level query adapter is owned by:
 *
 *     grammar/expressions/query.g4
 *
 * Knowledge-oriented query operations are owned by:
 *
 *     grammar/expressions/knowledge.g4
 *
 * This file exists to provide a stable statement-layer name:
 *
 *     queryStatement
 *
 * without duplicating SELECT, FROM, JOIN, WHERE, GROUP BY, ORDER BY,
 * knowledge-query, SQL, graph-query, or provider-specific syntax.
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * Query syntax expresses logical computational intent.
 *
 * It does NOT select:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     accelerator
 *     database engine
 *     storage engine
 *     cluster
 *     cloud provider
 *     network node
 *     physical partition
 *     physical index
 *     physical memory
 *     scheduler
 *     routing strategy
 *
 * Those decisions belong downstream.
 *
 * The intended pipeline is:
 *
 *     source
 *       |
 *       v
 *     canonical lexer
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     queryStatement
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     structural validation
 *       |
 *       +--> names
 *       +--> types
 *       +--> effects
 *       +--> capabilities
 *       +--> resources
 *       +--> contracts
 *       +--> policies
 *       +--> provenance
 *       |
 *       v
 *     query semantic model
 *       |
 *       v
 *     canonical IR
 *       |
 *       +--> local execution
 *       +--> parallel execution
 *       +--> distributed execution
 *       +--> streaming
 *       +--> tensor/data acceleration
 *       +--> heterogeneous execution
 *       +--> quantum/classical result processing
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     lowering
 *       |
 *       v
 *     scheduling / routing / execution
 *       |
 *       v
 *     target realization
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * Provide a stable statement-layer entry point for a logical data query.
 *
 * OWNS
 * ----
 *
 *     queryStatement
 *
 * PRIVATE RULES
 * -------------
 *
 *     none
 *
 * DOES NOT OWN
 * -------------
 *
 *     dataQueryConstruct
 *     dataQueryDeclaration
 *     dataQueryExpression
 *     dataQueryStatement
 *     SELECT
 *     FROM
 *     JOIN
 *     WHERE
 *     GROUP BY
 *     HAVING
 *     ORDER BY
 *     LIMIT
 *     OFFSET
 *     WINDOW
 *     QUALIFY
 *     WITH
 *     VALUES
 *     UNION
 *     INTERSECT
 *     EXCEPT
 *     EXISTS
 *     quantified predicates
 *     subqueries
 *     query source syntax
 *     query projection syntax
 *     query planning
 *     query optimization
 *     database execution
 *     knowledge storage
 *     knowledge retrieval
 *     graph execution
 *     SQL dialects
 *     provider-specific query syntax
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 *     grammar/data/queries.g4
 *
 * Required imported public rule:
 *
 *     dataQueryStatement
 *
 * EXPORTS
 * -------
 *
 *     queryStatement
 *
 * CONSUMED_BY
 * ----------
 *
 * This rule is available to a statement-composition grammar that explicitly
 * needs a named logical-query statement boundary.
 *
 * IMPORTANT:
 *
 * The current universal statement root already reaches data queries through:
 *
 *     statement
 *       -> domainStatement
 *       -> dataDomainStatement
 *       -> dataStatement
 *       -> dataQueryStmt
 *       -> dataQueryConstruct
 *
 * Therefore this adapter MUST NOT also be inserted into the universal
 * `statement` alternatives unless the existing data-domain query path is
 * deliberately replaced.
 *
 * Otherwise the same source construct would have two competing statement
 * paths.
 *
 * AST_OWNER
 * ---------
 *
 * Existing domain-neutral frontend AST.
 *
 * This grammar introduces no query-specific Rust AST.
 *
 * SEMANTIC_OWNER
 * --------------
 *
 * Canonical data/query semantic subsystem.
 *
 * TYPE_OWNER
 * ----------
 *
 * Canonical Zamani type subsystem.
 *
 * EFFECT_OWNER
 * ------------
 *
 * Canonical effect subsystem.
 *
 * CAPABILITY_OWNER
 * ---------------
 *
 * Canonical resource/capability subsystem.
 *
 * RESOURCE_OWNER
 * --------------
 *
 * Canonical resource subsystem.
 *
 * CONTRACT_OWNER
 * --------------
 *
 * Canonical validation/contract subsystem.
 *
 * POLICY_OWNER
 * ------------
 *
 * Canonical policy subsystem.
 *
 * PROVENANCE_OWNER
 * ----------------
 *
 * Canonical provenance subsystem.
 *
 * IR_OWNER
 * --------
 *
 * Canonical semantic/IR data-query pipeline.
 *
 * TEST_OWNER
 * ----------
 *
 *     grammar/tests/parser/
 *     grammar/tests/data/
 *     grammar/tests/semantic/
 *     grammar/tests/portability/
 *     grammar/tests/scalability/
 *     grammar/tests/boundary/
 *
 * SPEC_OWNER
 * ----------
 *
 *     grammar/spec/data.md
 *
 * ============================================================================
 * ANTLR CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It contains:
 *
 *     no lexer rules;
 *     no embedded Rust;
 *     no actions;
 *     no semantic predicates;
 *     no runtime execution;
 *     no filesystem access;
 *     no networking;
 *     no hardware discovery;
 *     no resource discovery.
 *
 * The canonical lexer authority is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * The parser vocabulary must ultimately be normalized to:
 *
 *     ZamaniLexer
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There must be exactly one owner for actual logical query syntax.
 *
 * That owner is:
 *
 *     grammar/data/queries.g4
 *
 * There must be exactly one owner for the expression-layer query adapter.
 *
 * That owner is:
 *
 *     grammar/expressions/query.g4
 *
 * This file owns only:
 *
 *     queryStatement
 *
 * Consequently this file MUST NOT reproduce:
 *
 *     dataSelectQueryExpression
 *     dataFromClause
 *     dataJoinClause
 *     dataWhereClause
 *     dataGroupByClause
 *     dataHavingClause
 *     dataOrderByClause
 *     dataLimitClause
 *     dataOffsetClause
 *     dataWindowClause
 *     dataQualifyClause
 *     dataWithQueryExpression
 *     dataSetQueryExpression
 *
 * ============================================================================
 * KNOWLEDGE QUERY SEPARATION
 * ============================================================================
 *
 * Knowledge queries already have an expression-level representation:
 *
 *     knowledge query(...)
 *
 * owned by:
 *
 *     grammar/expressions/knowledge.g4
 *
 * They therefore remain ordinary expressions and can already reach statement
 * position through the generic expression-statement mechanism.
 *
 * This file MUST NOT define:
 *
 *     knowledgeQueryStatement
 *
 * because doing so would create a second owner for knowledge-query syntax.
 *
 * The semantic relationship is:
 *
 *     knowledge query expression
 *          |
 *          v
 *     expression statement
 *          |
 *          v
 *     domain-neutral AST
 *
 * ============================================================================
 * DATA QUERY RELATIONSHIP
 * ============================================================================
 *
 * The canonical data query grammar already exposes:
 *
 *     dataQueryStatement
 *
 * Therefore this adapter is intentionally:
 *
 *     queryStatement
 *         :
 *             dataQueryStatement
 *         ;
 *
 * The actual query remains owned by:
 *
 *     grammar/data/queries.g4
 *
 * This guarantees that adding a new logical query clause does not require
 * editing this file.
 *
 * For example, if the data query grammar later gains:
 *
 *     a new grouping construct
 *     a new window construct
 *     a new logical set operation
 *     a new query constraint
 *     a new query preference
 *     a new query capability requirement
 *
 * this file remains unchanged as long as the public
 * `dataQueryStatement` boundary remains stable.
 *
 * ============================================================================
 * STATEMENT TERMINATION
 * ============================================================================
 *
 * `dataQueryStatement` owns the query terminator.
 *
 * Therefore this adapter MUST NOT append another:
 *
 *     SEMICOLON
 *
 * Doing so would require:
 *
 *     select ...;;
 *
 * and would duplicate statement termination ownership.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser context should be mapped conceptually as:
 *
 *     queryStatement
 *         |
 *         v
 *     existing data-query statement representation
 *
 * No parallel:
 *
 *     QueryStatementAst
 *
 * should be introduced merely because this adapter exists.
 *
 * The AST must preserve, through the existing query AST path:
 *
 *     source span
 *     query structure
 *     source ordering
 *     query expression
 *     projection
 *     source relations
 *     predicates
 *     grouping
 *     ordering
 *     windowing
 *     set operations
 *     query requirements
 *     query constraints
 *     query preferences
 *     query hints
 *     query metadata
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * A query statement represents logical data intent.
 *
 * Semantic analysis determines:
 *
 *     source validity
 *     name resolution
 *     type validity
 *     schema compatibility
 *     predicate validity
 *     aggregation validity
 *     ordering semantics
 *     capability requirements
 *     resource requirements
 *     effect requirements
 *     policy constraints
 *     provenance requirements
 *     determinism properties
 *     portability
 *
 * This grammar performs none of those operations.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Query results use the canonical Zamani type system.
 *
 * They may semantically represent:
 *
 *     scalar
 *     tuple
 *     record
 *     collection
 *     relation
 *     stream
 *     dataset
 *     graph
 *     tensor
 *     measurement result
 *     hybrid result
 *     future data abstraction
 *
 * This file imposes no:
 *
 *     row-count limit
 *     field-count limit
 *     nesting limit
 *     tensor-rank limit
 *     collection-size limit
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing a query does not itself perform an effect.
 *
 * Semantic analysis may derive effects such as:
 *
 *     data.read
 *     network
 *     distributed
 *     io
 *     external
 *     measurement
 *     randomness
 *
 * according to the logical source and active semantic/policy model.
 *
 * This file MUST NOT automatically declare every query to have every effect.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Query semantics may require capabilities such as:
 *
 *     data.query
 *     data.read
 *     distributed.query
 *     stream.query
 *     graph.query
 *     tensor.query
 *     accelerated.query
 *
 * Capability identities remain open-world.
 *
 * This grammar does not select a provider or physical target.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * No universal machine capacity is encoded.
 *
 * In particular, this file contains no limits for:
 *
 *     rows
 *     columns
 *     joins
 *     groups
 *     partitions
 *     replicas
 *     nodes
 *     workers
 *     threads
 *     devices
 *     GPUs
 *     QPUs
 *     memory
 *     tensor dimensions
 *
 * Resource requirements are semantic expressions.
 *
 * Examples:
 *
 *     requires(capability("distributed.query"))
 *
 *     requires(memory >= required_memory)
 *
 *     requires(topology(required_topology))
 *
 * Resolution occurs downstream.
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * Queries can participate in:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *     assertion
 *
 * The syntax of those constructs is owned by the validation subsystem.
 *
 * This file merely provides the query statement as an input to that semantic
 * system.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * A valid query is not automatically authorized.
 *
 * Query execution may be constrained by:
 *
 *     data-access policies
 *     security policies
 *     privacy policies
 *     resource policies
 *     execution policies
 *     deployment policies
 *     sandbox policies
 *     provenance policies
 *
 * Policy evaluation occurs downstream.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Query source information must remain available to provenance analysis.
 *
 * The frontend must be able to associate the query with:
 *
 *     source span
 *     logical query identity
 *     derived value
 *     transformation
 *     evidence
 *     decision
 *     verification
 *
 * The parser does not manufacture timestamps, runtime identities, hardware
 * identities or external records.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing is deterministic.
 *
 * Identical token streams under identical grammar/parser configuration must
 * produce equivalent parser structure.
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware
 *     clock
 *     randomness
 *     network
 *     filesystem
 *     database state
 *     scheduler state
 *     resource availability.
 *
 * Query execution may depend on external state; parsing does not.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A query may consume logical data derived from:
 *
 *     quantum measurement
 *     quantum experiments
 *     hybrid computation
 *     quantum simulation
 *
 * This file does not define quantum syntax.
 *
 * It does not enumerate gates, qubits, physical topology, routing, calibration,
 * scheduling, QEC, ZQN, or HAL behavior.
 *
 * If quantum-derived data enters a query:
 *
 *     query
 *       |
 *       v
 *     semantic data model
 *       |
 *       v
 *     hybrid/quantum semantic boundary
 *       |
 *       v
 *     quantum::ir where applicable
 *
 * `quantum::ir` remains the canonical quantum IR boundary.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Queries may process logical hardware/HDL data.
 *
 * They do not define:
 *
 *     register width
 *     bus width
 *     device count
 *     physical address
 *     physical topology
 *     FPGA region
 *     ASIC resource
 *
 * Hardware realization remains downstream.
 *
 * ============================================================================
 * AI / KNOWLEDGE INTEGRATION
 * ============================================================================
 *
 * Query results may feed:
 *
 *     infer
 *     deduce
 *     reason
 *     learn
 *     adapt
 *     explain
 *     decision
 *
 * Query syntax remains independent of those semantic operations.
 *
 * Knowledge querying remains owned by the knowledge-expression subsystem.
 *
 * General data querying remains owned by the data-query subsystem.
 *
 * ============================================================================
 * DISTRIBUTED / PARALLEL INTEGRATION
 * ============================================================================
 *
 * A logical query may be realized as:
 *
 *     local
 *     parallel
 *     distributed
 *     streaming
 *     accelerator
 *     heterogeneous
 *
 * without source modification.
 *
 * The query statement does not specify:
 *
 *     worker count
 *     node count
 *     thread count
 *     partition count
 *     replica count
 *     accelerator count.
 *
 * ============================================================================
 * INTEROPERABILITY
 * ============================================================================
 *
 * SQL, JSON, XML and vendor query languages remain dialect/interoperability
 * concerns.
 *
 * They must not be copied into this statement adapter.
 *
 * Their logical results may be lowered into the canonical data/query semantic
 * model.
 *
 * ============================================================================
 * METAPROGRAMMING
 * ============================================================================
 *
 * Query syntax may be inspected by controlled metaprogramming only through
 * the canonical AST/reflection infrastructure.
 *
 * This grammar never executes a query during:
 *
 *     parsing
 *     AST construction
 *     grammar prediction
 *     type checking.
 *
 * Compile-time query execution, if eventually supported, must be explicitly
 * represented by the metaprogramming/effect/capability system.
 *
 * ============================================================================
 * SECURITY / SANDBOX
 * ============================================================================
 *
 * Query syntax does not grant:
 *
 *     filesystem access
 *     network access
 *     database access
 *     native execution
 *     hardware access
 *     QPU access.
 *
 * Capability, policy, effect, authorization and sandbox analysis remain
 * downstream.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This file imposes no artificial language ceiling.
 *
 * It contains no universal maximum for:
 *
 *     query statements
 *     query depth
 *     projections
 *     joins
 *     groups
 *     sources
 *     fields
 *     rows
 *     partitions
 *     nodes
 *     devices
 *     workers
 *     threads
 *     memory
 *     tensor rank
 *     qubits
 *
 * "Scale to infinity" means that the language does not impose an artificial
 * finite capacity ceiling. Actual execution remains bounded by available
 * resources and implementation limits.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden:
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
 * Also forbidden:
 *
 *     physical device identifiers
 *     fixed topology
 *     vendor-specific target names
 *     fixed query capacity
 *     fixed worker count
 *     fixed partition count
 *
 * This grammar contains none of these as language semantics.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics:
 *
 *     malformed query statement
 *     unexpected token
 *     missing query structure
 *     malformed query expression
 *
 * Semantic diagnostics:
 *
 *     unknown source
 *     unknown field
 *     invalid type
 *     invalid aggregation
 *     incompatible schemas
 *     unsupported capability
 *     insufficient resources
 *     policy violation
 *     effect violation
 *     provenance violation
 *
 * Those semantic diagnostics MUST NOT be encoded as parser predicates.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing logical query syntax remains owned by:
 *
 *     grammar/data/queries.g4
 *
 * Existing expression query syntax remains owned by:
 *
 *     grammar/expressions/query.g4
 *
 * Existing knowledge query syntax remains owned by:
 *
 *     grammar/expressions/knowledge.g4
 *
 * This file adds a stable statement adapter without redefining those syntaxes.
 *
 * The public rule:
 *
 *     queryStatement
 *
 * can therefore remain stable even if the internal data-query grammar evolves.
 *
 * ============================================================================
 * REQUIRED REPOSITORY INTEGRATION
 * ============================================================================
 *
 * 1. grammar/data/queries.g4
 *
 * Must expose the stable:
 *
 *     dataQueryStatement
 *
 * rule.
 *
 * It already does so.
 *
 * 2. grammar/expressions/query.g4
 *
 * Continues to own:
 *
 *     queryExpression
 *
 * and must remain the expression-layer adapter.
 *
 * 3. grammar/expressions/knowledge.g4
 *
 * Continues to own:
 *
 *     knowledgeQueryExpression
 *
 * No query syntax should be copied into this file.
 *
 * 4. grammar/data/data.g4
 *
 * Continues to own:
 *
 *     dataQueryStmt
 *
 * which delegates to:
 *
 *     dataQueryConstruct
 *
 * This is currently the canonical path used by the data domain.
 *
 * 5. grammar/statements/statements.g4
 *
 * DO NOT add `queryStatement` to the universal `statement` alternatives
 * while `domainStatement -> dataDomainStatement -> dataStatement ->
 * dataQueryStmt` remains active.
 *
 * Otherwise query statements acquire two competing parser paths.
 *
 * If the repository later promotes query statements to a universal statement
 * family, the migration must be atomic:
 *
 *     a. add Query to Statements imports;
 *     b. add queryStatement to statement;
 *     c. remove the duplicated data-domain statement path;
 *     d. retain dataQueryConstruct as the canonical logical query owner.
 *
 * This file itself does not require that migration.
 *
 * ============================================================================
 * REQUIRED DATA-QUERY TOKEN-VOCABULARY NORMALIZATION
 * ============================================================================
 *
 * The inspected repository currently contains data grammars using:
 *
 *     tokenVocab = Zamani;
 *
 * while the canonical statement/domain grammars use:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * These must be normalized at the repository integration layer.
 *
 * The canonical lexical authority should be:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * and parser grammars should consume that vocabulary consistently.
 *
 * This is an integration correction outside the ownership of this adapter.
 *
 * The query adapter deliberately does not create replacement lexer tokens.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE
 * --------
 *
 * Every valid construct accepted by `dataQueryStatement` must be accepted
 * through `queryStatement`.
 *
 * Examples:
 *
 *     select value
 *     from source;
 *
 *     select value
 *     from source
 *     where value > threshold;
 *
 *     select category, aggregate(value)
 *     from source
 *     group by category;
 *
 *     select value
 *     from source
 *     order by value descending;
 *
 *     select value
 *     from source
 *     limit result_count;
 *
 *     select value
 *     from source
 *     offset start_position;
 *
 *     select value
 *     from source
 *     requires(capability("data.query"));
 *
 * The exact lexical spelling remains owned by the data-query grammar.
 *
 * NEGATIVE
 * --------
 *
 *     malformed SELECT
 *     malformed FROM
 *     malformed WHERE
 *     malformed JOIN
 *     malformed GROUP BY
 *     malformed ORDER BY
 *     malformed LIMIT
 *     malformed OFFSET
 *     malformed query requirement
 *     malformed query expression
 *     missing required query terminator
 *
 * BOUNDARY
 * --------
 *
 *     empty query
 *     nested subqueries
 *     nested query sources
 *     large projection lists
 *     large join structures
 *     large grouping structures
 *     large ordering structures
 *     deeply nested logical query composition
 *     query over stream
 *     query over tensor-derived data
 *     query over graph-derived data
 *     query over quantum measurement data
 *     query over hybrid computation data
 *
 * SCALABILITY
 * ----------
 *
 * Verify that no test assumes:
 *
 *     fixed row count
 *     fixed column count
 *     fixed source count
 *     fixed join count
 *     fixed node count
 *     fixed worker count
 *     fixed device count
 *     fixed memory capacity.
 *
 * DETERMINISM
 * -----------
 *
 * Identical source and parser configuration must produce equivalent parse
 * structure.
 *
 * CROSS-DOMAIN
 * ------------
 *
 *     data + classical
 *     data + AI
 *     data + reasoning
 *     data + learning
 *     data + adaptation
 *     data + quantum
 *     data + hybrid
 *     data + distributed
 *     data + networking
 *     data + hardware metadata
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [x] It owns exactly one public rule: queryStatement.
 *
 *     [x] It does not duplicate logical query syntax.
 *
 *     [x] It does not duplicate knowledge-query syntax.
 *
 *     [x] It does not create a new query keyword.
 *
 *     [x] It does not create a second expression hierarchy.
 *
 *     [x] It does not create a query-specific AST.
 *
 *     [x] It does not create a query-specific IR.
 *
 *     [x] It does not select physical targets.
 *
 *     [x] It contains no resource ceilings.
 *
 *     [x] It contains no target-specific actions.
 *
 *     [x] It contains no semantic predicates.
 *
 *     [x] It contains no unsafe Rust.
 *
 *     [x] It is independent of hardware size.
 *
 *     [x] It preserves the canonical data-query ownership boundary.
 *
 *     [x] It preserves the canonical knowledge-query ownership boundary.
 *
 *     [x] It preserves the canonical expression-query ownership boundary.
 *
 *     [x] Its downstream integration is explicitly defined.
 *
 *     [x] Its migration path is defined if universal query statements are
 *         promoted later.
 *
 *     [x] Its tests are defined independently of physical resource limits.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * This file answers only:
 *
 *     "How does an already-defined logical data query enter the statement
 *      layer?"
 *
 * It does NOT answer:
 *
 *     "How is the query executed?"
 *
 *     "Where is it executed?"
 *
 *     "Which machine executes it?"
 *
 *     "Which database executes it?"
 *
 *     "Which GPU executes it?"
 *
 *     "Which QPU executes it?"
 *
 *     "How many workers are used?"
 *
 *     "How is it routed?"
 *
 *     "How is it scheduled?"
 *
 * Those decisions remain downstream.
 *
 * ============================================================================
 */

parser grammar Query;

options {
    tokenVocab = ZamaniLexer;
}

import ZamaniDataQueriesParser;


/*
 * ============================================================================
 * PUBLIC STATEMENT BOUNDARY
 * ============================================================================
 *
 * This adapter deliberately delegates to the canonical data-query statement.
 *
 * No semicolon is added here because dataQueryStatement owns termination.
 * ============================================================================
 */

queryStatement
    : dataQueryStatement
    ;
/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/ai/queries.g4
 *
 * GRAMMAR
 * -------
 * AIQueries
 *
 * STATUS
 * ------
 * Production AI query composition boundary.
 *
 * LANGUAGE BASELINE
 * -----------------
 * Rust 1.97 or later
 * Rust edition 2021
 * Safe Rust only
 * No unsafe Rust
 *
 * GRAMMAR TECHNOLOGY
 * ------------------
 * ANTLR4 parser grammar
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file provides the AI-domain composition boundary for query
 * capabilities already defined by Zamani's universal query and knowledge
 * systems.
 *
 * IMPORTANT:
 *
 * This file DOES NOT define a new query language.
 *
 * It DOES NOT duplicate:
 *
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
 *     subqueries
 *
 * It also DOES NOT duplicate:
 *
 *     knowledge query
 *     knowledge patterns
 *     knowledge terms
 *     knowledge relations
 *
 * Those constructs already have canonical owners.
 *
 * The purpose of this file is composition only.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         Zamani source
 *                              |
 *                              v
 *                            lexer
 *                              |
 *                              v
 *                           parser
 *                              |
 *                 +------------+-------------+
 *                 |                          |
 *                 v                          v
 *       universal data query       universal knowledge query
 *                 |                          |
 *                 v                          v
 *       dataQueryExpression       knowledgeQueryExpression
 *                 |                          |
 *                 v                          v
 *          queryExpression       AIQueries composition
 *                 |                          |
 *                 +-------------+------------+
 *                               |
 *                               v
 *                      domain-neutral AST
 *                               |
 *                               v
 *                      semantic analysis
 *                               |
 *              +----------------+----------------+
 *              |                |                |
 *              v                v                v
 *           types           effects         capabilities
 *              |                |                |
 *              +----------------+----------------+
 *                               |
 *                          resources
 *                               |
 *                           policies
 *                               |
 *                         provenance
 *                               |
 *                         canonical IR
 *                               |
 *                  +------------+-------------+
 *                  |                          |
 *                  v                          v
 *             classical                  quantum::ir
 *                  |                          |
 *                  +------------+-------------+
 *                               |
 *                         target-independent
 *                         optimization/lowering
 *                               |
 *                         execution planning
 *                               |
 *                              HAL
 *                               |
 *                         target realization
 *
 * AI is therefore a consumer/composition domain, not a second query language.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 * -------------
 *
 *     aiQueryConstruct
 *
 * This is the sole AI-specific query composition boundary.
 *
 *
 * THIS FILE DOES NOT OWN
 * ---------------------
 *
 *     - lexer rules;
 *     - keyword definitions;
 *     - identifiers;
 *     - literals;
 *     - operators;
 *     - punctuation;
 *     - general expression precedence;
 *     - data query syntax;
 *     - SELECT syntax;
 *     - FROM syntax;
 *     - JOIN syntax;
 *     - WHERE syntax;
 *     - GROUP BY syntax;
 *     - HAVING syntax;
 *     - ORDER BY syntax;
 *     - LIMIT syntax;
 *     - OFFSET syntax;
 *     - WINDOW syntax;
 *     - QUALIFY syntax;
 *     - WITH syntax;
 *     - VALUES syntax;
 *     - set-operation syntax;
 *     - data query planning;
 *     - knowledge query syntax;
 *     - knowledge patterns;
 *     - knowledge terms;
 *     - knowledge relations;
 *     - knowledge storage;
 *     - inference;
 *     - deduction;
 *     - induction;
 *     - abduction;
 *     - learning;
 *     - adaptation;
 *     - model execution;
 *     - agent execution;
 *     - database execution;
 *     - graph execution;
 *     - distributed execution;
 *     - hardware selection;
 *     - quantum operations;
 *     - quantum routing;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - AST implementation;
 *     - semantic implementation;
 *     - type checking;
 *     - effect checking;
 *     - capability resolution;
 *     - resource negotiation;
 *     - policy enforcement;
 *     - provenance implementation;
 *     - IR construction;
 *     - optimization;
 *     - lowering;
 *     - scheduling;
 *     - runtime execution.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * The ownership chain is:
 *
 *     grammar/data/queries.g4
 *              |
 *              v
 *     dataQueryExpression
 *              |
 *              v
 *     grammar/expressions/query.g4
 *              |
 *              v
 *     queryExpression
 *              |
 *              v
 *     grammar/ai/queries.g4
 *              |
 *              v
 *     aiQueryConstruct
 *
 * Knowledge queries have a separate canonical path:
 *
 *     grammar/expressions/knowledge.g4
 *              |
 *              v
 *     knowledgeQueryExpression
 *              |
 *              v
 *     aiQueryConstruct
 *
 * Consequently there MUST NOT be another implementation here of:
 *
 *     dataQueryExpression
 *     queryExpression
 *     knowledgeQueryExpression
 *
 * This prevents divergent query semantics.
 *
 * ============================================================================
 * QUERY DOMAINS
 * ============================================================================
 *
 * AI code can query several kinds of logical information:
 *
 *     data
 *     knowledge
 *     datasets
 *     observations
 *     model outputs
 *     experiment results
 *     telemetry
 *     scientific data
 *     graph-like information
 *     distributed data
 *     quantum measurement results
 *     hybrid computation results
 *     hardware observations
 *     resource information
 *     provenance information
 *
 * The grammar does not create a separate syntax for each one.
 *
 * The semantic/type system determines the source kind.
 *
 * ============================================================================
 * DATA QUERY INTEGRATION
 * ============================================================================
 *
 * General logical data querying is owned by:
 *
 *     grammar/data/queries.g4
 *
 * Its expression-level adapter is:
 *
 *     grammar/expressions/query.g4
 *
 * with:
 *
 *     queryExpression
 *         : dataQueryExpression
 *         ;
 *
 * AIQueries consumes that boundary.
 *
 * Therefore AI code can consume a normal Zamani query without introducing
 * an AI-specific SELECT/FROM/JOIN language.
 *
 * ============================================================================
 * KNOWLEDGE QUERY INTEGRATION
 * ============================================================================
 *
 * Knowledge querying is owned by:
 *
 *     grammar/expressions/knowledge.g4
 *
 * through:
 *
 *     knowledgeQueryExpression
 *
 * AIQueries consumes this canonical rule directly.
 *
 * This permits:
 *
 *     AI
 *       |
 *       +--> data query
 *       |
 *       +--> knowledge query
 *
 * while preserving one source of truth for each query family.
 *
 * ============================================================================
 * REASONING INTEGRATION
 * ============================================================================
 *
 * Query results may be consumed by:
 *
 *     infer
 *     deduce
 *     reason
 *     explain
 *     decide
 *
 * AIQueries does not define those operations.
 *
 * The semantic relationship is:
 *
 *     query
 *       |
 *       v
 *     result
 *       |
 *       +--> reasoning
 *       +--> inference
 *       +--> learning
 *       +--> adaptation
 *       +--> explanation
 *       +--> decision
 *
 * ============================================================================
 * LEARNING INTEGRATION
 * ============================================================================
 *
 * Query results may become:
 *
 *     training data
 *     features
 *     labels
 *     observations
 *     evaluation data
 *     model inputs
 *
 * AIQueries does not define learning syntax.
 *
 * Learning remains owned by its existing AI grammar and semantic subsystem.
 *
 * The same query source can therefore feed multiple consumers without
 * changing the query grammar.
 *
 * ============================================================================
 * KNOWLEDGE / DATA SEPARATION
 * ============================================================================
 *
 * A knowledge query and a data query are related but are not automatically
 * identical semantic operations.
 *
 * DATA QUERY
 * ----------
 *
 * Primarily describes logical selection/transformation of data.
 *
 * KNOWLEDGE QUERY
 * --------------
 *
 * Primarily describes retrieval/matching of logical knowledge.
 *
 * Their implementations may eventually share infrastructure, but this
 * grammar does not collapse them into one semantic category.
 *
 * ============================================================================
 * OPEN-WORLD EXTENSIBILITY
 * ============================================================================
 *
 * The query grammar must remain open-world.
 *
 * It MUST NOT enumerate application concepts such as:
 *
 *     image
 *     sentiment
 *     robot
 *     vehicle
 *     payment
 *     legal_case
 *     blockchain
 *     virtual_reality
 *     medical_case
 *     financial_account
 *
 * Such concepts are represented by:
 *
 *     identifiers
 *     types
 *     schemas
 *     libraries
 *     dialects
 *     semantic registries
 *     application data
 *
 * New application domains therefore do not require modifications to this
 * grammar.
 *
 * ============================================================================
 * AI MODEL INTEGRATION
 * ============================================================================
 *
 * AI models may be query sources or query consumers.
 *
 * Examples:
 *
 *     model outputs
 *     embeddings
 *     predictions
 *     evaluation records
 *     training records
 *     model metadata
 *
 * AIQueries does not define model syntax.
 *
 * Model semantics remain owned by the AI model subsystem.
 *
 * ============================================================================
 * AGENT INTEGRATION
 * ============================================================================
 *
 * An AI agent may execute or request a query.
 *
 * The architecture is:
 *
 *     agent
 *       |
 *       v
 *     query construct
 *       |
 *       v
 *     semantic query
 *       |
 *       v
 *     capability/resource/policy analysis
 *       |
 *       v
 *     execution
 *
 * This file does not create a second actor or agent execution model.
 *
 * Existing concurrency/actor infrastructure remains authoritative.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Queries may consume quantum-derived information such as:
 *
 *     measurement results
 *     experiment results
 *     simulation results
 *     hybrid computation results
 *
 * AIQueries does not define quantum syntax.
 *
 * The quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * Query processing may occur before or after quantum lowering depending on
 * semantic dependencies.
 *
 * No query rule may contain:
 *
 *     physical qubit IDs
 *     coupling maps
 *     calibration data
 *     routing decisions
 *     QEC parameters
 *     QPU identifiers
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Queries may consume:
 *
 *     hardware observations
 *     telemetry
 *     simulation results
 *     synthesis information
 *     resource information
 *     performance information
 *
 * This grammar does not select hardware.
 *
 * Hardware realization remains downstream.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * A logical query may eventually be executed:
 *
 *     locally
 *     in parallel
 *     on an accelerator
 *     across a cluster
 *     across distributed nodes
 *     through a service
 *     in a cloud environment
 *     on a future execution substrate
 *
 * AIQueries does not encode:
 *
 *     node count
 *     worker count
 *     partition count
 *     replica count
 *     device count
 *
 * Such decisions belong to resource analysis, optimization and execution
 * planning.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * AIQueries introduces NO AI-specific query type.
 *
 * Query result types are determined by the canonical Zamani type system.
 *
 * Possible semantic result categories include:
 *
 *     scalar
 *     tuple
 *     record
 *     option
 *     result
 *     collection
 *     stream
 *     relation
 *     graph
 *     dataset
 *     tensor
 *     knowledge set
 *     domain-defined value
 *
 * The grammar does not constrain:
 *
 *     result cardinality
 *     row count
 *     field count
 *     tensor rank
 *     dimensionality
 *     nesting depth
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing this grammar introduces no runtime effect.
 *
 * Semantic query execution may produce effects such as:
 *
 *     data.read
 *     io
 *     network
 *     distributed
 *     external
 *     measurement
 *
 * The actual effect set is determined downstream.
 *
 * This grammar does not create a second effect system.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Query execution may require capabilities such as:
 *
 *     data.query
 *     data.read
 *     knowledge.query
 *     distributed.query
 *     stream.query
 *     graph.query
 *     tensor.query
 *
 * Capability names remain semantic data.
 *
 * This file does not grant or resolve capabilities.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * No resource ceiling is encoded here.
 *
 * The grammar MUST NOT define limits for:
 *
 *     rows
 *     columns
 *     fields
 *     joins
 *     groups
 *     results
 *     partitions
 *     replicas
 *     workers
 *     nodes
 *     threads
 *     GPUs
 *     QPUs
 *     memory
 *     tensor dimensions
 *     tensor rank
 *     network size
 *
 * A query may participate in source-level requirements such as:
 *
 *     requires capability("distributed.query");
 *     requires memory >= required_memory;
 *     requires topology(required_topology);
 *
 * Resource feasibility is determined downstream.
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Query constructs may be governed by:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * Contract syntax belongs to:
 *
 *     grammar/validation/
 *
 * AIQueries does not duplicate contract rules.
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Queries may be constrained by:
 *
 *     access policy
 *     privacy policy
 *     security policy
 *     resource policy
 *     deployment policy
 *     sandbox policy
 *     provenance policy
 *
 * A successful parse MUST NOT imply authorization.
 *
 * Policy evaluation occurs downstream.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Query syntax must remain compatible with provenance tracking.
 *
 * Downstream provenance may record:
 *
 *     source
 *     query
 *     inputs
 *     transformation
 *     result
 *     evidence
 *     decision
 *     verification
 *
 * The parser itself records only source-structure information supplied by the
 * ANTLR parse tree.
 *
 * It must not invent:
 *
 *     timestamps
 *     machine identifiers
 *     hardware identifiers
 *     runtime identities
 *     external provenance records.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing must be deterministic.
 *
 * The parse tree must depend only on:
 *
 *     source tokens
 *     grammar version
 *     parser configuration
 *
 * Parsing must not depend on:
 *
 *     hardware
 *     database state
 *     network state
 *     filesystem state
 *     randomness
 *     wall-clock time
 *     resource availability
 *     QPU state
 *     scheduler state
 *
 * Query EXECUTION may depend on external state; parsing does not.
 *
 * ============================================================================
 * PORTABILITY / POCO-REAF CONTRACT
 * ============================================================================
 *
 * A source-level query must remain independent of its eventual execution
 * substrate.
 *
 * The same query may be lowered to:
 *
 *     embedded execution
 *     CPU execution
 *     multicore execution
 *     GPU execution
 *     FPGA execution
 *     ASIC execution
 *     accelerator execution
 *     simulator execution
 *     QPU-assisted execution
 *     HPC execution
 *     cluster execution
 *     distributed execution
 *     cloud execution
 *     future execution substrates
 *
 * The grammar contains no physical-machine assumptions.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This file MUST NOT contain universal limits such as:
 *
 *     MAX_QUERIES
 *     MAX_RESULTS
 *     MAX_ROWS
 *     MAX_COLUMNS
 *     MAX_FIELDS
 *     MAX_JOINS
 *     MAX_QUERY_DEPTH
 *     MAX_GROUPS
 *     MAX_NODES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_QPUS
 *     MAX_MEMORY
 *     MAX_TENSOR_RANK
 *     MAX_DEVICES
 *
 * It MUST also not encode:
 *
 *     fixed database engines
 *     fixed graph engines
 *     fixed ML engines
 *     fixed hardware
 *     fixed topology
 *     fixed accelerator
 *     fixed QPU
 *
 * Scalability is determined by available resources and downstream
 * implementation feasibility, not by grammar constants.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar introduces NO AI-specific AST node.
 *
 * `aiQueryConstruct` is a composition context.
 *
 * The canonical semantic structures remain:
 *
 *     data query
 *     knowledge query
 *
 * The frontend AST must preserve the underlying query meaning rather than
 * replacing it with an AI-specific representation.
 *
 * The AST must preserve sufficient source information for:
 *
 *     diagnostics
 *     provenance
 *     semantic analysis
 *     tooling
 *     formatting
 *     refactoring
 *
 * The AST must not contain:
 *
 *     hardware placement
 *     physical device IDs
 *     physical qubit IDs
 *     routing
 *     scheduling
 *     calibration
 *     QEC implementation
 *     database-engine decisions
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * `aiQueryConstruct` means only:
 *
 *     "a canonical query capability is being composed in an AI-domain
 *      context."
 *
 * It does NOT mean:
 *
 *     query execution has occurred;
 *     authorization exists;
 *     resources exist;
 *     capabilities exist;
 *     a model exists;
 *     a database exists;
 *     a knowledge provider exists;
 *     a hardware target exists.
 *
 * Semantic analysis resolves those questions.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates no IR.
 *
 * Query semantics lower through the existing canonical semantic pipeline.
 *
 * Conceptually:
 *
 *     aiQueryConstruct
 *             |
 *             v
 *     canonical query semantics
 *             |
 *             v
 *     domain-neutral semantic model
 *             |
 *             v
 *     canonical IR
 *
 * If a query ultimately consumes quantum-derived data, any quantum computation
 * follows the established:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * AIQueries MUST NOT create:
 *
 *     AIQueryIR
 *     AIKnowledgeQueryIR
 *     AIQueryPlanIR
 *
 * merely because the query originated in an AI context.
 *
 * ============================================================================
 * ERROR / DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * This adapter delegates syntax errors to its canonical imported grammars.
 *
 * It MUST NOT invent alternate recovery productions that accept malformed
 * queries.
 *
 * Diagnostics should ultimately identify:
 *
 *     malformed query syntax
 *     malformed knowledge query
 *     invalid expression
 *     invalid identifier
 *     invalid type
 *
 * Semantic diagnostics such as:
 *
 *     capability unavailable
 *     resource unavailable
 *     unauthorized query
 *     provider unavailable
 *
 * belong downstream.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Tests should be maintained under the repository's existing grammar test
 * hierarchy, with an AI query-specific suite added if not already present:
 *
 *     grammar/tests/ai/queries/
 *
 * Recommended categories:
 *
 *     positive/
 *     negative/
 *     integration/
 *     cross-domain/
 *     scalability/
 *     determinism/
 *     portability/
 *     compatibility/
 *
 * ============================================================================
 * POSITIVE TESTS
 * ============================================================================
 *
 * At minimum, verify AI composition accepts:
 *
 *     a canonical data query expression;
 *     a canonical knowledge query expression;
 *     a query returning a model input;
 *     a query returning model output;
 *     a query over a dataset;
 *     a query over a knowledge source;
 *     a query over scientific data;
 *     a query over measurement-derived data;
 *     a query used as an input to reasoning;
 *     a query used as an input to learning;
 *     a query used by an AI agent.
 *
 * Exact source syntax must be inherited from the canonical query grammars.
 *
 * ============================================================================
 * NEGATIVE TESTS
 * ============================================================================
 *
 * Verify this grammar does NOT independently accept malformed query syntax.
 *
 * Examples:
 *
 *     malformed SELECT
 *     malformed FROM
 *     malformed JOIN
 *     malformed WHERE
 *     malformed knowledge query
 *     missing query source
 *     malformed expression
 *
 * Invalid syntax must fail through the canonical owner.
 *
 * ============================================================================
 * CROSS-DOMAIN TESTS
 * ============================================================================
 *
 * Verify queries can semantically participate with:
 *
 *     classical computation
 *     numerical computation
 *     scientific computation
 *     AI inference
 *     learning
 *     adaptation
 *     uncertainty
 *     provenance
 *     policies
 *     concurrency
 *     distributed computation
 *     networking
 *     quantum measurement
 *     hybrid computation
 *     HDL simulation
 *     hardware observations
 *     accelerators
 *     simulation
 *
 * The query grammar itself must remain unchanged.
 *
 * ============================================================================
 * SCALABILITY TESTS
 * ============================================================================
 *
 * Test increasingly large:
 *
 *     source programs;
 *     query expressions;
 *     projection lists;
 *     predicates;
 *     nested queries;
 *     knowledge patterns;
 *     result schemas;
 *     symbolic expressions.
 *
 * Scaling tests must never require changing this grammar.
 *
 * There is no grammar-defined "maximum query size".
 *
 * ============================================================================
 * DETERMINISM TESTS
 * ============================================================================
 *
 * Parse identical source repeatedly and verify equivalent:
 *
 *     parse-tree structure;
 *     rule selection;
 *     token sequence;
 *     source spans.
 *
 * ============================================================================
 * PORTABILITY TESTS
 * ============================================================================
 *
 * The same source query must remain syntactically valid regardless of whether
 * semantic execution is eventually realized on:
 *
 *     tiny embedded hardware;
 *     CPU;
 *     multicore CPU;
 *     GPU;
 *     FPGA;
 *     ASIC;
 *     accelerator;
 *     simulator;
 *     QPU;
 *     HPC;
 *     cluster;
 *     distributed infrastructure;
 *     cloud;
 *     future hardware.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This file introduces an AI composition boundary without changing the
 * canonical query syntax.
 *
 * Therefore:
 *
 *     data query syntax remains owned by data/queries.g4;
 *     query expression syntax remains owned by expressions/query.g4;
 *     knowledge query syntax remains owned by expressions/knowledge.g4.
 *
 * Changes to those canonical grammars must preserve their public exported
 * rule contracts or deliberately go through the repository's compatibility
 * process.
 *
 * AIQueries should not be modified merely because a backend implementation,
 * database provider, model provider, accelerator, QPU, or execution strategy
 * changes.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 *     grammar/data/queries.g4
 *         public boundary:
 *             dataQueryExpression
 *
 *     grammar/expressions/query.g4
 *         public boundary:
 *             queryExpression
 *
 *     grammar/expressions/knowledge.g4
 *         public boundary:
 *             knowledgeQueryExpression
 *
 *     grammar/antlr/ZamaniLexer.g4
 *         canonical lexical vocabulary
 *
 *
 * EXPORTS
 * -------
 *
 *     aiQueryConstruct
 *
 *
 * CONSUMED_BY
 * ----------
 *
 *     grammar/ai/ai.g4
 *
 *
 * AST_OWNER
 * ---------
 *
 * Existing domain-neutral frontend AST.
 *
 *
 * SEMANTIC_OWNER
 * --------------
 *
 * Canonical data-query and knowledge-query semantic systems.
 *
 *
 * TYPE_OWNER
 * ----------
 *
 * Canonical Zamani type system.
 *
 *
 * EFFECT_OWNER
 * ------------
 *
 * Canonical Zamani effect system.
 *
 *
 * CAPABILITY_OWNER
 * ---------------
 *
 * grammar/resources/
 * and downstream semantic capability resolution.
 *
 *
 * RESOURCE_OWNER
 * --------------
 *
 * grammar/resources/
 * and downstream resource analysis.
 *
 *
 * POLICY_OWNER
 * -----------
 *
 * grammar/policies/
 * and grammar/security/
 * with downstream policy enforcement.
 *
 *
 * PROVENANCE_OWNER
 * ----------------
 *
 * Canonical provenance subsystem.
 *
 *
 * IR_OWNER
 * --------
 *
 * Canonical semantic/IR pipeline.
 *
 *
 * QUANTUM_IR_OWNER
 * ----------------
 *
 * quantum::ir
 *
 *
 * TEST_OWNER
 * ----------
 *
 * grammar/tests/ai/queries/
 *
 *
 * SPEC_OWNER
 * ----------
 *
 * grammar/spec/ai.md
 * grammar/spec/data.md
 * grammar/spec/policies.md
 * grammar/spec/resources.md
 * grammar/spec/effects.md
 *
 * ============================================================================
 * ANTLR CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It contains:
 *
 *     - no lexer rules;
 *     - no embedded Rust;
 *     - no semantic predicates;
 *     - no target-language actions;
 *     - no filesystem access;
 *     - no network access;
 *     - no runtime execution;
 *     - no hardware access;
 *     - no resource discovery.
 *
 * The parser consumes the canonical Zamani lexer vocabulary.
 *
 * ============================================================================
 * FUTURE EXTENSIBILITY
 * ============================================================================
 *
 * Adding a new:
 *
 *     database backend
 *     knowledge provider
 *     graph provider
 *     model provider
 *     accelerator
 *     QPU
 *     hardware target
 *     distributed execution strategy
 *     query optimizer
 *     storage engine
 *     AI algorithm
 *
 * MUST NOT require modification to this file unless the language-level public
 * query composition contract itself changes.
 *
 * New application domains should likewise remain outside this grammar.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when all of the following are true:
 *
 *     [x] It is an ANTLR4 parser grammar.
 *
 *     [x] Its grammar name is AIQueries.
 *
 *     [x] It uses the canonical Zamani lexer vocabulary.
 *
 *     [x] It imports canonical query boundaries rather than duplicating them.
 *
 *     [x] It imports the canonical knowledge-query boundary.
 *
 *     [x] It owns only aiQueryConstruct.
 *
 *     [x] It introduces no alternate query language.
 *
 *     [x] It introduces no AI-specific query AST.
 *
 *     [x] It introduces no AI-specific query IR.
 *
 *     [x] It introduces no lexer rules.
 *
 *     [x] It introduces no semantic predicates.
 *
 *     [x] It introduces no embedded Rust.
 *
 *     [x] It introduces no hardware assumptions.
 *
 *     [x] It introduces no resource ceilings.
 *
 *     [x] It introduces no capability resolution.
 *
 *     [x] It introduces no policy enforcement.
 *
 *     [x] It introduces no runtime execution.
 *
 *     [x] It preserves the domain-neutral AST boundary.
 *
 *     [x] It preserves the canonical semantic query boundary.
 *
 *     [x] It preserves the canonical quantum::ir boundary.
 *
 *     [x] It preserves POCO-REAF source portability.
 *
 *     [x] It requires no unsafe Rust.
 *
 *     [x] It is compatible with Rust 1.97 or later downstream.
 *
 *     [ ] AI composition imports this grammar.
 *
 *     [ ] Positive AI-query tests pass.
 *
 *     [ ] Negative AI-query tests pass.
 *
 *     [ ] Data-query integration tests pass.
 *
 *     [ ] Knowledge-query integration tests pass.
 *
 *     [ ] Reasoning integration tests pass.
 *
 *     [ ] Learning integration tests pass.
 *
 *     [ ] Agent integration tests pass.
 *
 *     [ ] Policy integration tests pass.
 *
 *     [ ] Provenance integration tests pass.
 *
 *     [ ] Cross-domain tests pass.
 *
 *     [ ] Scalability tests pass.
 *
 *     [ ] Determinism tests pass.
 *
 *     [ ] Portability tests pass.
 *
 *     [ ] Compatibility tests pass.
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar AIQueries;

options {
    tokenVocab = ZamaniLexer;
}

import
    ZamaniExpressionQuery,
    KnowledgeExpressions
;


/*
 * ============================================================================
 * PUBLIC AI QUERY COMPOSITION BOUNDARY
 * ============================================================================
 *
 * This is the ONLY grammar rule owned by AIQueries.
 *
 * Data queries enter through:
 *
 *     queryExpression
 *
 * Knowledge queries enter through:
 *
 *     knowledgeQueryExpression
 *
 * Neither syntax is duplicated here.
 *
 * ============================================================================
 */

aiQueryConstruct
    : queryExpression
    | knowledgeQueryExpression
    ;


/*
 * ============================================================================
 * ARCHITECTURAL INVARIANT
 * ============================================================================
 *
 * DO NOT add query implementation rules here.
 *
 * In particular, do not add:
 *
 *     selectQuery
 *     fromClause
 *     joinClause
 *     whereClause
 *     groupByClause
 *     orderByClause
 *     limitClause
 *     knowledgeQuery
 *     knowledgePattern
 *     knowledgeTerm
 *
 * Those belong to their canonical owners.
 *
 * ============================================================================
 */
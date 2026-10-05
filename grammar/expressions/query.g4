/*

* ============================================================================
* Zamani Programming Language
* ============================================================================
* 
* File:
* grammar/expressions/query.g4
* 
* Status:
* Production expression-layer query integration boundary.
* 
* Grammar technology:
* ANTLR4 parser grammar
* 
* Rust implementation baseline:
* Rust 1.97 / Rust 1.97.1
* Edition 2021
* Safe Rust only.
* 
* ============================================================================
* PURPOSE
* ============================================================================
* 
* This file owns the EXPRESSION-LEVEL INTEGRATION of query semantics.
* 
* It does NOT own the complete logical query language.
* 
* The canonical logical data-query syntax is owned by:
* 
* grammar/data/queries.g4
* 
* That file owns:
* 
* - SELECT;
* - FROM;
* - JOIN;
* - WHERE;
* - GROUP BY;
* - HAVING;
* - ORDER BY;
* - LIMIT;
* - OFFSET;
* - WINDOW;
* - QUALIFY;
* - WITH;
* - VALUES;
* - set operations;
* - subqueries;
* - query declarations;
* - query-level requirements;
* - query-level constraints;
* - query-level preferences;
* - query-level hints;
* - query composition.
* 
* This file MUST NOT duplicate those rules.
* 
* Its purpose is to establish the expression boundary:
* 
* primaryExpression
*       |
*       +--> queryExpression
*                |
*                v
*         dataQueryExpression
* 
* This permits a logical query to participate in ordinary Zamani expression
* contexts without creating a second query grammar.
* 
* ============================================================================
* ARCHITECTURAL POSITION
* ============================================================================
* 
* source
*   |
*   v
* ZamaniLexer
*   |
*   v
* parser
*   |
*   v
* domain-neutral AST
*   |
*   v
* structural validation
*   |
*   +------------------------------+
*   |                              |
*   v                              v
* ordinary expression          query expression
*                                  |
*                                  v
*                           logical data query
*                                  |
*                                  v
*                           semantic query model
*                                  |
*                                  v
*                           canonical semantic IR
*                                  |
*                +-----------------+------------------+
*                |                 |                  |
*                v                 v                  v
*            local data       distributed data    accelerator data
*                |                 |                  |
*                +-----------------+------------------+
*                                  |
*                                  v
*                              execution plan
*                                  |
*                                  v
*                               target
* 
* Query syntax therefore describes LOGICAL INTENT.
* 
* It does not select:
* 
* CPU
* GPU
* FPGA
* ASIC
* QPU
* database engine
* storage engine
* cluster
* network node
* physical index
* physical partition
* scheduler
* simulator
* 
* ============================================================================
* OWNERSHIP
* ============================================================================
* 
* THIS FILE OWNS:
* 
* queryExpression
* 
* The expression-layer semantic boundary by which a logical query enters
* the universal expression hierarchy.
* 
* THIS FILE MAY OWN:
* 
* minimal expression-level dispatch necessary to expose query syntax
* without duplicating the canonical query grammar.
* 
* THIS FILE DOES NOT OWN:
* 
* dataQueryExpression
* dataQueryDeclaration
* dataQueryStatement
* SELECT syntax
* FROM syntax
* JOIN syntax
* WHERE syntax
* GROUP BY syntax
* HAVING syntax
* ORDER BY syntax
* LIMIT syntax
* OFFSET syntax
* WINDOW syntax
* QUALIFY syntax
* WITH syntax
* VALUES syntax
* set-operation syntax
* query source syntax
* query planner syntax
* query optimizer syntax
* database syntax
* SQL dialect syntax
* knowledge-store implementation
* graph execution
* distributed execution
* physical execution
* resource negotiation
* capability discovery
* hardware selection
* quantum routing
* QEC
* ZQN
* HAL
* 
* ============================================================================
* SINGLE-AUTHORITY RULE
* ============================================================================
* 
* There must be exactly ONE canonical owner for logical query syntax:
* 
* grammar/data/queries.g4
* 
* There must be exactly ONE expression-layer integration point:
* 
* grammar/expressions/query.g4
* 
* There must NOT be:
* 
* grammar/expressions/queries.g4
* 
* implementing a competing query grammar.
* 
* If a legacy plural expression-query file exists, it must be reduced to a
* compatibility/delegation boundary or removed through the repository's normal
* compatibility process. It MUST NOT introduce another implementation of
* query syntax.
* 
* ============================================================================
* DEPENDENCY CONTRACT
* ============================================================================
* 
* DEPENDS_ON:
* 
* grammar/antlr/ZamaniLexer.g4
* grammar/data/queries.g4
* grammar/expressions/expressions.g4
* grammar/data/data.g4
* grammar/types/
* grammar/validation/
* grammar/resources/
* grammar/effects/
* grammar/policies/
* grammar/spec/data.md
* grammar/spec/resources.md
* grammar/spec/effects.md
* grammar/spec/policies.md
* 
* EXPORTS:
* 
* queryExpression
* 
* CONSUMED_BY:
* 
* grammar/expressions/expressions.g4
* any future expression composition grammar that explicitly imports this
* integration boundary
* 
* AST_OWNER:
* 
* existing domain-neutral frontend AST
* 
* SEMANTIC_OWNER:
* 
* canonical data/query semantic layer
* 
* IR_OWNER:
* 
* canonical data semantic/IR pipeline
* 
* TEST_OWNER:
* 
* grammar/tests/parser/
* grammar/tests/ast/
* grammar/tests/semantic/
* grammar/tests/data/
* grammar/tests/ai/
* grammar/tests/scalability/
* grammar/tests/portability/
* 
* SPEC_OWNER:
* 
* grammar/spec/data.md
* 
* ============================================================================
* ANTLR CONTRACT
* ============================================================================
* 
* This is a parser grammar.
* 
* It consumes the canonical lexer vocabulary:
* 
* tokenVocab = ZamaniLexer;
* 
* It contains:
* 
* - no lexer rules;
* - no embedded Rust;
* - no semantic predicates;
* - no target-language actions;
* - no runtime execution;
* - no filesystem access;
* - no network access;
* - no hardware access.
* 
* ============================================================================
* QUERY EXPRESSION BOUNDARY
* ============================================================================
* 
* A query expression is deliberately defined as an adapter to the canonical
* logical query expression.
* 
* The canonical query grammar provides:
* 
* dataQueryExpression
* 
* Therefore:
* 
* queryExpression
*     : dataQueryExpression
*     ;
* 
* is the only rule required here.
* 
* This intentionally avoids duplicating:
* 
* select
* from
* where
* join
* group
* order
* limit
* with
* values
* union
* intersect
* except
* exists
* subquery
* 
* ============================================================================
* WHY THIS BOUNDARY EXISTS
* ============================================================================
* 
* Query is both:
* 
* 1. a data-domain construct;
* 2. an expression-producing construct.
* 
* A query can therefore produce a value that participates in ordinary
* expression composition.
* 
* Conceptually:
* 
* let result = query ...;
* 
* transform(query ...);
* 
* consume(query ...);
* 
* combine(query ..., other_value);
* 
* The exact legal contexts remain controlled by the surrounding expression,
* declaration, statement and type grammars.
* 
* This file does not decide whether a particular query result is:
* 
* scalar
* row
* record
* collection
* stream
* relation
* tensor
* graph
* dataset
* future data abstraction.
* 
* That is semantic/type-system responsibility.
* 
* ============================================================================
* DATA / KNOWLEDGE INTEGRATION
* ============================================================================
* 
* The same query expression boundary can support multiple logical knowledge
* and data sources.
* 
* Examples include:
* 
* collections
* datasets
* streams
* records
* graphs
* knowledge bases
* model outputs
* scientific data
* telemetry
* distributed data
* accelerator data
* quantum measurement results
* hybrid computation results.
* 
* The grammar MUST NOT create separate query syntax for each source.
* 
* Instead:
* 
* query expression
*      |
*      v
* logical source
*      |
*      v
* semantic source type
*      |
*      v
* capability/resource analysis
*      |
*      v
* execution realization
* 
* ============================================================================
* KNOWLEDGE QUERY INTEGRATION
* ============================================================================
* 
* Knowledge operations such as:
* 
* assert
* retract
* query
* 
* belong to the broader knowledge/semantic architecture.
* 
* A query expression MUST therefore be capable of being interpreted against
* knowledge-oriented sources without requiring a separate knowledge-query
* grammar.
* 
* The query grammar does not define:
* 
* inference algorithms
* deduction engines
* graph engines
* theorem provers
* probabilistic engines
* retrieval engines
* database engines.
* 
* Those are semantic/runtime implementations.
* 
* ============================================================================
* REASONING INTEGRATION
* ============================================================================
* 
* Query results may become inputs to:
* 
* infer
* deduce
* reason
* learn
* adapt
* explain
* decision
* 
* This file does not define those constructs.
* 
* It only ensures that query results are ordinary expression values.
* 
* Conceptual pipeline:
* 
* query
*   |
*   v
* result
*   |
*   +--> reasoning
*   +--> learning
*   +--> adaptation
*   +--> validation
*   +--> decision
*   +--> classical computation
*   +--> quantum/classical control
* 
* ============================================================================
* TYPE CONTRACT
* ============================================================================
* 
* "queryExpression" is type-checked by the canonical type system.
* 
* This file does not define a query-specific type hierarchy.
* 
* Query result types may be:
* 
* scalar
* tuple
* record
* option
* result
* array
* slice
* map
* collection
* stream
* relation
* graph
* tensor
* domain-specific semantic value
* 
* subject to the existing type system.
* 
* The grammar introduces no fixed result width, row count, field count,
* dimensionality, nesting depth, or cardinality.
* 
* ============================================================================
* EFFECT CONTRACT
* ============================================================================
* 
* Query syntax itself is non-executing.
* 
* Semantic analysis may assign effects according to the query source and
* execution policy.
* 
* Possible effects include:
* 
* data.read
* data.write
* network
* distributed
* io
* randomness
* external
* measurement
* 
* A query expression MUST NOT automatically imply every possible effect.
* 
* Effects are determined from semantic source, operation and policy
* information.
* 
* ============================================================================
* CAPABILITY CONTRACT
* ============================================================================
* 
* A query may require capabilities such as:
* 
* capability("data.query")
* capability("data.read")
* capability("distributed.query")
* capability("stream.query")
* capability("graph.query")
* capability("tensor.query")
* capability("accelerated.query")
* 
* These are semantic capabilities.
* 
* This grammar MUST NOT require a particular provider or device.
* 
* The source may therefore remain portable while the compiler determines
* whether a target can satisfy the requirements.
* 
* ============================================================================
* RESOURCE CONTRACT
* ============================================================================
* 
* Query expressions MUST NOT contain language-level machine limits.
* 
* Forbidden concepts include fixed universal ceilings for:
* 
* rows
* columns
* fields
* joins
* groups
* partitions
* replicas
* nodes
* threads
* devices
* GPUs
* QPUs
* memory
* tensor dimensions
* tensor rank.
* 
* Query quantities are values or semantic requirements.
* 
* Examples of portable semantic intent include:
* 
* requires capability("distributed.query")
* 
* requires memory >= required_memory
* 
* requires topology(required_topology)
* 
* The actual resource resolution belongs downstream.
* 
* ============================================================================
* CONTRACT INTEGRATION
* ============================================================================
* 
* Query expressions may participate in:
* 
* requires
* ensures
* invariant
* assume
* guarantee
* property
* assert
* 
* Example semantic relationship:
* 
* query result
*     |
*     v
* condition
*     |
*     v
* contract
* 
* This file does not define contract syntax.
* 
* Contract syntax remains owned by:
* 
* grammar/validation/
* 
* ============================================================================
* POLICY INTEGRATION
* ============================================================================
* 
* Query execution may be constrained by:
* 
* security policies
* resource policies
* data-access policies
* privacy policies
* deployment policies
* execution policies
* sandbox policies
* provenance policies.
* 
* Policy evaluation occurs after parsing.
* 
* A syntactically valid query MUST NOT be interpreted as an authorization
* grant.
* 
* ============================================================================
* PROVENANCE CONTRACT
* ============================================================================
* 
* Query expressions must preserve enough source information for downstream
* provenance systems to establish:
* 
* source
* query identity
* derived value
* transformation
* evidence
* decision
* verification
* 
* The parser must preserve:
* 
* source spans
* syntactic nesting
* source ordering
* identifiers
* query structure
* 
* The grammar does not generate timestamps, hardware identifiers, runtime
* identities or external provenance records.
* 
* Those belong to semantic/compiler/runtime provenance systems.
* 
* ============================================================================
* DETERMINISM CONTRACT
* ============================================================================
* 
* Parsing a query expression MUST be deterministic.
* 
* The parse result MUST depend only on:
* 
* source tokens
* grammar version
* language compatibility configuration.
* 
* Parsing MUST NOT depend on:
* 
* clock time
* randomness
* hardware
* network state
* filesystem state
* database state
* QPU state
* scheduler state
* resource availability.
* 
* Query execution may naturally depend on external state, but that is not
* parsing semantics.
* 
* ============================================================================
* QUANTUM INTEGRATION
* ============================================================================
* 
* A query may consume or produce quantum-derived data.
* 
* Examples include logical query processing over:
* 
* measurement results
* experiment results
* quantum-generated datasets
* hybrid computation results.
* 
* This file MUST NOT define quantum operations.
* 
* It MUST NOT define:
* 
* H
* X
* Y
* Z
* CNOT
* RX
* RY
* RZ
* physical qubit topology
* calibration
* routing.
* 
* If a query participates in quantum computation:
* 
* query expression
*      |
*      v
* semantic analysis
*      |
*      v
* quantum/classical boundary
*      |
*      v
* quantum::ir
* 
* "quantum::ir" remains the canonical quantum IR boundary.
* 
* ============================================================================
* HYBRID INTEGRATION
* ============================================================================
* 
* Query expressions may participate in hybrid computation:
* 
* classical data
*      |
*      v
*   query
*      |
*      v
* classical decision
*      |
*      v
* quantum operation
*      |
*      v
* measurement
*      |
*      v
*   query
* 
* The grammar does not encode a fixed execution topology.
* 
* The hybrid semantic layer decides the valid data/control boundary.
* 
* ============================================================================
* AI / LEARNING INTEGRATION
* ============================================================================
* 
* Query results may feed:
* 
* learning
* inference
* reasoning
* adaptation
* model evaluation
* evidence generation
* decision records.
* 
* The query grammar does not enumerate ML algorithms.
* 
* Algorithms remain semantic/library/runtime capabilities.
* 
* This preserves extensibility for future models and algorithms without
* changing the universal grammar.
* 
* ============================================================================
* DISTRIBUTED / PARALLEL INTEGRATION
* ============================================================================
* 
* The query expression does not specify:
* 
* number of workers
* number of nodes
* number of threads
* partition count
* replica count
* GPU count.
* 
* The same logical query may be lowered into:
* 
* local execution
* parallel execution
* distributed execution
* streaming execution
* accelerator execution
* heterogeneous execution.
* 
* Resource and capability analysis determines the realization.
* 
* ============================================================================
* HDL / HARDWARE INTEGRATION
* ============================================================================
* 
* Queries may be used to describe logical selection or transformation of
* hardware-related data.
* 
* The grammar does not encode:
* 
* register width
* bus width
* fixed memory size
* device count
* physical topology.
* 
* Hardware intent remains owned by:
* 
* grammar/hdl/
* grammar/hardware/
* 
* Query semantics can consume their logical data representations.
* 
* ============================================================================
* INTEROPERABILITY
* ============================================================================
* 
* SQL, JSON, XML and vendor-specific query languages MUST NOT be copied into
* this universal expression grammar.
* 
* They belong to:
* 
* grammar/dialects/
* grammar/interoperability/
* 
* Their logical results may be adapted into the canonical Zamani query/data
* semantic model.
* 
* Conceptually:
* 
* external dialect
*      |
*      v
* dialect parser
*      |
*      v
* logical data/query model
*      |
*      v
* Zamani semantic pipeline
* 
* This file remains dialect-neutral.
* 
* ============================================================================
* METAPROGRAMMING INTEGRATION
* ============================================================================
* 
* Query expressions may be inspected by controlled reflection or
* compile-time tooling only where explicitly permitted by the metaprogramming
* capability/effect system.
* 
* This file does not execute queries during:
* 
* parsing
* AST construction
* macro expansion
* type checking.
* 
* Compile-time query execution, where supported, is a separate semantic
* capability and must remain explicitly controlled.
* 
* ============================================================================
* SANDBOX / SECURITY
* ============================================================================
* 
* A query expression must not bypass:
* 
* capability checks
* access policies
* sandbox policies
* effect checks
* provenance requirements
* data-access controls.
* 
* A query's source syntax does not grant:
* 
* filesystem access
* network access
* database access
* hardware access
* QPU access.
* 
* ============================================================================
* SCALABILITY / POCO-REAF
* ============================================================================
* 
* This file imposes NO artificial finite language-level limits.
* 
* In particular, it contains no universal maximum for:
* 
* query depth
* query clauses
* projections
* joins
* grouping keys
* ordering keys
* result rows
* source count
* fields
* dimensions
* nodes
* workers
* devices
* memory
* threads
* qubits
* tensor rank.
* 
* Repetition is represented structurally by the canonical query grammar.
* 
* Actual limits are implementation/resource constraints, not language
* constants.
* 
* "Scale to infinity" therefore means:
* 
* no artificial language ceiling is introduced here.
* 
* It does not claim that a finite machine has infinite resources.
* 
* ============================================================================
* HARD-CODING AUDIT
* ============================================================================
* 
* This file MUST contain:
* 
* no hardware IDs
* no provider IDs
* no fixed device counts
* no fixed node counts
* no fixed worker counts
* no fixed memory limits
* no fixed query limits
* no fixed quantum limits
* no fixed tensor limits
* no fixed register widths
* no physical topology.
* 
* The following classes of identifiers are explicitly forbidden as language
* constants:
* 
* MAX_QUBITS
* MAX_CPUS
* MAX_GPUS
* MAX_FPGAS
* MAX_NODES
* MAX_MEMORY
* MAX_THREADS
* MAX_TENSOR_RANK
* MAX_REGISTER_WIDTH
* MAX_NETWORK_SIZE
* MAX_DEVICE_COUNT
* 
* ============================================================================
* ERROR / DIAGNOSTIC CONTRACT
* ============================================================================
* 
* Structural syntax errors are parser diagnostics.
* 
* Semantic errors belong downstream.
* 
* Examples:
* 
* PARSER ERROR:
* 
* malformed query structure
* 
* SEMANTIC ERROR:
* 
* query references an unknown source
* 
* TYPE ERROR:
* 
* predicate expects a boolean-compatible value
* 
* EFFECT ERROR:
* 
* query requires an undeclared effect under the active policy
* 
* CAPABILITY ERROR:
* 
* target lacks a required query capability
* 
* RESOURCE ERROR:
* 
* required resources cannot be satisfied
* 
* POLICY ERROR:
* 
* query violates an execution/data policy
* 
* PROVENANCE ERROR:
* 
* required provenance cannot be established.
* 
* These error classes MUST NOT be collapsed into one parser rule.
* 
* ============================================================================
* INTEGRATION WITH grammar/expressions/expressions.g4
* ============================================================================
* 
* The canonical expression composition file:
* 
* grammar/expressions/expressions.g4
* 
* owns:
* 
* primaryExpression
* 
* It must integrate this file by adding:
* 
* | queryExpression
* 
* to the existing primary-expression alternatives.
* 
* The resulting conceptual rule is:
* 
* primaryExpression
*     : identifierExpression
*     | literalExpression
*     | parenthesizedExpression
*     | tupleExpression
*     | arrayExpression
*     | mapExpression
*     | lambdaExpression
*     | newExpression
*     | thisExpression
*     | superExpression
*     | queryExpression
*     ;
* 
* IMPORTANT:
* 
* "expressions.g4" remains the owner of the public "expression" rule and
* precedence hierarchy.
* 
* "query.g4" remains the owner of the query-expression integration rule.
* 
* "data/queries.g4" remains the owner of actual logical query syntax.
* 
* ============================================================================
* INTEGRATION WITH grammar/data/queries.g4
* ============================================================================
* 
* "queryExpression" delegates directly to:
* 
* dataQueryExpression
* 
* No query rule is copied here.
* 
* Therefore a future change to:
* 
* SELECT
* FROM
* JOIN
* WHERE
* GROUP BY
* ORDER BY
* WITH
* VALUES
* set operations
* 
* is made in:
* 
* grammar/data/queries.g4
* 
* rather than here.
* 
* This is intentional.
* 
* It means this file should not need to be reopened merely because the
* logical query language gains another clause that remains inside the
* canonical data-query owner.
* 
* ============================================================================
* INTEGRATION WITH grammar/data/data.g4
* ============================================================================
* 
* "data.g4" remains the data-domain facade.
* 
* It may expose:
* 
* dataQueryConstruct
* 
* from:
* 
* grammar/data/queries.g4
* 
* The expression boundary does not replace the data facade.
* 
* The two boundaries are:
* 
* dataQueryConstruct
*     -> data-domain integration
* 
* queryExpression
*     -> universal expression integration
* 
* Both ultimately reference the same logical query grammar.
* 
* ============================================================================
* INTEGRATION WITH grammar/spec/data.md
* ============================================================================
* 
* "grammar/spec/data.md" owns normative query semantics.
* 
* This file must remain aligned with:
* 
* logical query intent
* query result semantics
* query portability
* query composition
* query execution separation.
* 
* It must not duplicate the complete specification.
* 
* ============================================================================
* AST CONTRACT
* ============================================================================
* 
* This grammar does not define Rust AST structures.
* 
* The parser should preserve the syntactic query structure required by the
* existing domain-neutral AST.
* 
* Recommended conceptual mapping:
* 
* queryExpression
*     -> QueryExpression
* 
* dataQueryExpression
*     -> DataQuery
* 
* The exact Rust type names are owned by the existing frontend AST.
* 
* If the AST uses a generic expression node containing a domain-specific
* payload, query semantics must fit that existing model rather than creating
* an incompatible parallel AST.
* 
* ============================================================================
* IR CONTRACT
* ============================================================================
* 
* This grammar creates no IR.
* 
* Query expressions ultimately lower through the canonical data semantic
* pipeline.
* 
* They may subsequently participate in:
* 
* classical IR
* data IR
* distributed execution planning
* accelerator lowering
* hybrid execution
* quantum/classical integration.
* 
* A query expression MUST NOT create a second query IR merely because it
* entered through the expression layer.
* 
* ============================================================================
* COMPATIBILITY CONTRACT
* ============================================================================
* 
* This file introduces the rule:
* 
* queryExpression
* 
* Compatibility concerns therefore primarily involve the placement of query
* expressions in the universal expression grammar.
* 
* Changes to the logical query syntax remain owned by:
* 
* grammar/data/queries.g4
* 
* Changes to keyword reservation remain owned by:
* 
* grammar/lexer/
* 
* Changes to historical syntax remain owned by:
* 
* grammar/compatibility/
* 
* ============================================================================
* TEST CONTRACT
* ============================================================================
* 
* This file is complete only when the following classes of tests exist.
* 
* ---
* A. GRAMMAR STRUCTURE
* ---
* 
* Verify:
* 
* - this grammar compiles;
* - tokenVocab resolves to ZamaniLexer;
* - dataQueryExpression resolves through the configured ANTLR grammar
*   library/import mechanism;
* - no lexer rules exist here;
* - no duplicate query syntax exists here.
* 
* ---
* B. POSITIVE EXPRESSION TESTS
* ---
* 
* Query expressions must be accepted wherever the surrounding expression
* grammar permits a primary expression.
* 
* Examples should cover:
* 
* query-expression in an initializer;
* query-expression as a function argument;
* query-expression in a parenthesized expression;
* query-expression combined with ordinary operators where the type system
* permits it;
* query-expression used as input to reasoning;
* query-expression used as input to learning;
* query-expression used as input to adaptation.
* 
* ---
* C. QUERY DELEGATION TESTS
* ---
* 
* The same logical query must have one syntax definition.
* 
* Test:
* 
* dataQueryExpression
* 
* through:
* 
* dataQueryConstruct
* 
* and:
* 
* queryExpression
* 
* and verify that the two paths represent the same logical query semantics.
* 
* ---
* D. NEGATIVE TESTS
* ---
* 
* Reject malformed query syntax through the canonical data-query grammar.
* 
* Examples:
* 
* incomplete SELECT;
* malformed FROM;
* malformed JOIN;
* malformed predicate;
* malformed grouping;
* malformed ordering;
* malformed subquery.
* 
* This file must not add a second set of inconsistent diagnostics.
* 
* ---
* E. TYPE TESTS
* ---
* 
* Verify that query results participate in the existing type system.
* 
* Test:
* 
* scalar results;
* record results;
* collection results;
* stream results;
* optional results;
* result/error values;
* tensor/data values;
* 
* where supported by the semantic implementation.
* 
* ---
* F. EFFECT TESTS
* ---
* 
* Verify that query semantics can be checked for:
* 
* data access;
* network;
* distributed execution;
* external access;
* measurement where applicable.
* 
* ---
* G. CAPABILITY TESTS
* ---
* 
* Verify that logical requirements such as:
* 
* capability("data.query")
* 
* can be checked without embedding target/provider information in this
* grammar.
* 
* ---
* H. RESOURCE TESTS
* ---
* 
* Verify that:
* 
* memory
* concurrency
* distributed capacity
* accelerator availability
* 
* are semantic/resource concerns and not grammar constants.
* 
* ---
* I. POLICY TESTS
* ---
* 
* Verify that query expressions respect:
* 
* access policies;
* sandbox policies;
* resource policies;
* execution policies.
* 
* ---
* J. PROVENANCE TESTS
* ---
* 
* Verify that query source spans and structural identity are preserved for
* downstream provenance.
* 
* ---
* K. CROSS-DOMAIN TESTS
* ---
* 
* Query results must be usable, where semantically legal, with:
* 
* classical computation;
* AI reasoning;
* learning;
* adaptation;
* distributed computation;
* networking;
* hybrid computation;
* quantum measurement results;
* HDL/hardware data;
* simulation.
* 
* ---
* L. SCALABILITY TESTS
* ---
* 
* Test increasingly large logical queries without introducing grammar-level
* limits.
* 
* Scaling dimensions include:
* 
* projection count;
* source count;
* predicate complexity;
* nesting depth;
* grouping keys;
* ordering keys;
* query composition;
* source naming depth.
* 
* Tests must be bounded by test-environment resources, but those bounds must
* never become language constants.
* 
* ---
* M. DETERMINISM TESTS
* ---
* 
* The same source and grammar configuration must produce the same parse
* structure and source spans.
* 
* ---
* N. COMPATIBILITY TESTS
* ---
* 
* Verify that existing canonical data-query programs continue to parse.
* 
* Adding expression-level query integration must not change the meaning of
* existing non-query expressions.
* 
* ============================================================================
* COMPLETION CRITERIA
* ============================================================================
* 
* THIS FILE IS DONE when:
* 
* [ ] It is the only expression-level query integration boundary.
* 
* [ ] It does not duplicate logical query syntax.
* 
* [ ] "data/queries.g4" remains the sole owner of logical query syntax.
* 
* [ ] "expressions.g4" remains the sole owner of universal expression
* precedence and the public `expression` entry point.
* 
* [ ] "queryExpression" is integrated into "primaryExpression".
* 
* [ ] The ANTLR grammar resolves all required imports.
* 
* [ ] The canonical Zamani lexer is used.
* 
* [ ] No embedded Rust exists.
* 
* [ ] No unsafe Rust is required.
* 
* [ ] No semantic predicate is required.
* 
* [ ] No runtime operation occurs during parsing.
* 
* [ ] No physical target is selected.
* 
* [ ] No hardware capacity is encoded.
* 
* [ ] No quantum hardware limit is encoded.
* 
* [ ] No query-size ceiling is encoded.
* 
* [ ] Query results use the existing type system.
* 
* [ ] Effects are checked downstream.
* 
* [ ] Capabilities are checked downstream.
* 
* [ ] Resources are checked downstream.
* 
* [ ] Contracts are checked downstream.
* 
* [ ] Policies are checked downstream.
* 
* [ ] Provenance is preserved downstream.
* 
* [ ] SQL remains a dialect/interoperability concern.
* 
* [ ] JSON/XML remain interoperability/data concerns.
* 
* [ ] Knowledge querying can consume the same logical query model.
* 
* [ ] Reasoning can consume query results.
* 
* [ ] Learning can consume query results.
* 
* [ ] Adaptive execution can consume query results.
* 
* [ ] Distributed execution can lower the same logical query.
* 
* [ ] Accelerator execution can lower the same logical query.
* 
* [ ] Quantum-derived data can participate without creating a second query
* language.
* 
* [ ] "quantum::ir" remains the canonical quantum boundary.
* 
* [ ] Positive tests pass.
* 
* [ ] Negative tests pass.
* 
* [ ] Boundary tests pass.
* 
* [ ] Scalability tests pass.
* 
* [ ] Determinism tests pass.
* 
* [ ] Compatibility tests pass.
* 
* ============================================================================
* FINAL RULE
* ============================================================================
* 
* The purpose of this file is deliberately narrow:
* 
* ONE expression boundary
*      |
*      v
* ONE canonical logical query grammar
*      |
*      v
* ONE semantic query model
*      |
*      v
* TARGET-INDEPENDENT execution planning
* 
* Query syntax describes WHAT is wanted.
* 
* The compiler/runtime decides HOW and WHERE it can be realized.
* 
* Therefore the same logical source can remain unchanged while being lowered
* to tiny, local, parallel, distributed, accelerated, heterogeneous, quantum,
* simulated, or future execution environments, subject to semantic
* correctness, available resources, capabilities, policies and physical
* feasibility.
* 
* ============================================================================
  */

parser grammar ZamaniExpressionQuery;

options {
tokenVocab = ZamaniLexer;
}

/*

* ============================================================================
* PUBLIC EXPRESSION INTEGRATION RULE
* ============================================================================
* 
* This is intentionally a one-rule adapter.
* 
* The complete query grammar is owned by:
* 
* grammar/data/queries.g4
* 
* Do not copy its rules into this file.
* ============================================================================
  */

queryExpression
: dataQueryExpression
;
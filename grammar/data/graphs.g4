/*
 * ============================================================================
 * ZAMANI UNIVERSAL COMPUTING LANGUAGE
 * ============================================================================
 *
 * File:
 *     grammar/data/graphs.g4
 *
 * Grammar:
 *     Graphs
 *
 * Status:
 *     PRODUCTION-READY GRAPH DATA DOMAIN GRAMMAR
 *
 * Purpose:
 *     Define the portable source syntax for logical graph data structures,
 *     graph nodes, graph relations, graph properties, graph patterns, graph
 *     assertions/retractions, and graph-local constraints.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         Zamani source
 *                              |
 *                              v
 *                           lexer
 *                              |
 *                              v
 *                            parser
 *                              |
 *                              v
 *                    domain-neutral AST
 *                              |
 *                              v
 *                       semantic analysis
 *                              |
 *                +-------------+-------------+
 *                |                           |
 *                v                           v
 *             data model              graph semantics
 *                |                           |
 *                +-------------+-------------+
 *                              |
 *                              v
 *                     canonical semantic model
 *                              |
 *                              v
 *                         canonical IR
 *                              |
 *                 +------------+------------+
 *                 |            |            |
 *                 v            v            v
 *              classical    distributed   accelerator
 *                              |
 *                              v
 *                        target realization
 *
 * Graphs are DATA semantics.
 *
 * This file does not define:
 *
 *     graph storage engines
 *     graph databases
 *     memory layouts
 *     adjacency-list layouts
 *     hash-table layouts
 *     indexes
 *     partition placement
 *     physical devices
 *     CPUs
 *     GPUs
 *     FPGAs
 *     ASICs
 *     QPUs
 *     network nodes
 *     thread counts
 *     memory capacities
 *     traversal algorithms
 *     graph algorithms
 *     vendor APIs
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Graph source describes logical graph intent.
 *
 * The same graph program may be realized as:
 *
 *     in-memory data
 *     persistent data
 *     streaming data
 *     distributed data
 *     partitioned data
 *     replicated data
 *     CPU execution
 *     GPU execution
 *     FPGA execution
 *     accelerator execution
 *     heterogeneous execution
 *     distributed execution
 *     quantum/classical workflows where semantically applicable
 *
 * without changing this grammar.
 *
 * There are NO universal graph-size constants.
 *
 * In particular this file does not define:
 *
 *     MAX_NODES
 *     MAX_EDGES
 *     MAX_VERTICES
 *     MAX_PROPERTIES
 *     MAX_LABELS
 *     MAX_PATH_LENGTH
 *     MAX_GRAPH_DEPTH
 *     MAX_GRAPH_SIZE
 *     MAX_RELATIONS
 *     MAX_PATTERNS
 *     MAX_HOPS
 *     MAX_PARTITIONS
 *     MAX_REPLICAS
 *     MAX_NODES_PER_PARTITION
 *
 * Any finite limit is an implementation/resource/target concern.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     graph declaration syntax
 *     graph-local options
 *     logical node declarations
 *     logical relation declarations
 *     graph property declarations
 *     graph property assignments
 *     graph patterns
 *     graph pattern nodes
 *     graph pattern relations
 *     graph pattern property constraints
 *     graph pattern guards
 *     graph assertions
 *     graph retractions
 *     graph-local graph expressions
 *     graph-domain composition entry points
 *
 * THIS FILE DOES NOT OWN:
 *
 *     identifiers
 *     qualified names
 *     general expressions
 *     types
 *     collections
 *     records
 *     schemas
 *     tables
 *     datasets
 *     streams
 *     queries
 *     provenance
 *     resources
 *     capabilities
 *     policies
 *     effects
 *     execution
 *     distributed placement
 *     hardware
 *     quantum operations
 *     quantum::ir
 *     graph algorithms
 *     graph storage
 *     graph database implementations
 *
 * ============================================================================
 * SHARED LANGUAGE OWNERSHIP
 * ============================================================================
 *
 * General expressions remain owned by:
 *
 *     grammar/expressions/
 *
 * This file consumes:
 *
 *     expression
 *
 * It MUST NOT define:
 *
 *     expression
 *     binaryExpression
 *     unaryExpression
 *     callExpression
 *     literalExpression
 *     identifierExpression
 *
 * or another expression hierarchy.
 *
 * ============================================================================
 * LEXICAL OWNERSHIP
 * ============================================================================
 *
 * This grammar consumes the canonical Zamani lexer vocabulary.
 *
 * Existing graph-related lexical tokens include:
 *
 *     GRAPH
 *     NODE
 *     RELATION
 *     PROPERTY
 *     PATTERN
 *     GUARD
 *     ASSERT
 *     RETRACT
 *
 * Graph algorithms, graph labels, graph relationship types, graph property
 * names, vendor names, and future graph concepts remain identifiers or
 * expressions unless they genuinely require a reserved language keyword.
 *
 * This is deliberate.
 *
 * The grammar must remain open-world.
 *
 * ============================================================================
 * ANTLR / RUST SAFETY
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It contains:
 *
 *     no embedded Rust
 *     no semantic predicates
 *     no target-language actions
 *     no filesystem operations
 *     no network operations
 *     no hardware discovery
 *     no runtime execution
 *     no randomness
 *     no unsafe code
 *
 * Generated Rust must remain compatible with:
 *
 *     Rust 1.97+
 *     Rust 2021
 *
 * and the Zamani Rust implementation must remain safe Rust.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates parse-tree structure only.
 *
 * Recommended semantic mappings:
 *
 *     graphDeclaration
 *         -> GraphDeclaration
 *
 *     graphNodeDeclaration
 *         -> GraphNodeDeclaration
 *
 *     graphRelationDeclaration
 *         -> GraphRelationDeclaration
 *
 *     graphPropertyDeclaration
 *         -> GraphPropertyDeclaration
 *
 *     graphPatternDeclaration
 *         -> GraphPatternDeclaration
 *
 *     graphPatternNode
 *         -> GraphPatternNode
 *
 *     graphPatternRelation
 *         -> GraphPatternRelation
 *
 *     graphPatternGuard
 *         -> GraphPatternGuard
 *
 *     graphAssertion
 *         -> GraphAssertion
 *
 *     graphRetraction
 *         -> GraphRetraction
 *
 * The exact Rust AST structures remain owned by the canonical domain-neutral
 * frontend AST.
 *
 * This file MUST NOT create a graph-specific competing AST package.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * A graph is a logical data object.
 *
 * A node represents an entity/value participating in the graph.
 *
 * A relation connects graph terms.
 *
 * A property associates a graph element with a value.
 *
 * A pattern describes structural and value constraints over graph elements.
 *
 * A guard is an ordinary Zamani expression evaluated by semantic analysis.
 *
 * Assertions and retractions describe logical graph-state intent.
 *
 * Whether these operations are:
 *
 *     mutable
 *     persistent
 *     transactional
 *     distributed
 *     replicated
 *     eventually consistent
 *     strongly consistent
 *     streamed
 *     simulated
 *
 * is NOT determined by this grammar.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Graph labels and property values are represented using ordinary Zamani
 * expressions.
 *
 * This deliberately avoids creating a graph-specific type system.
 *
 * A semantic implementation may subsequently establish:
 *
 *     node type
 *     relation type
 *     property type
 *     label type
 *     graph schema
 *
 * through the canonical type/schema systems.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Graph constructs do not contain physical resource limits.
 *
 * Resource requirements belong to the existing resource subsystem.
 *
 * Examples that remain semantically possible:
 *
 *     requires capability("distributed.data");
 *     requires capability("graph.compute");
 *     requires memory >= required_memory;
 *
 * Graphs.g4 does not parse or evaluate those resource requirements itself.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing graph syntax produces no runtime effect.
 *
 * Semantic analysis may classify operations such as:
 *
 *     assertion
 *     retraction
 *     persistence
 *     external graph access
 *     network access
 *     distributed mutation
 *
 * according to the canonical effect system.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Graph access, mutation, persistence, distribution, security and privacy
 * policies remain owned by the policy/security subsystems.
 *
 * This file does not define graph-specific policy semantics.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Graph constructs preserve their source locations through the normal parser
 * context and AST pipeline.
 *
 * Provenance is consumed downstream by:
 *
 *     grammar/data/provenance.g4
 *
 * and the canonical semantic provenance subsystem.
 *
 * Graph-specific provenance syntax is intentionally not duplicated here.
 *
 * ============================================================================
 * QUERY CONTRACT
 * ============================================================================
 *
 * Graphs.g4 does NOT create a competing query language.
 *
 * Graph patterns are reusable logical pattern structures.
 *
 * Query composition remains owned by:
 *
 *     grammar/data/queries.g4
 *
 * A future/current query grammar can consume:
 *
 *     graphPatternDeclaration
 *     graphPattern
 *     graphPatternNode
 *     graphPatternRelation
 *
 * without creating a second graph-query language.
 *
 * ============================================================================
 * ALGORITHM CONTRACT
 * ============================================================================
 *
 * Graph algorithms are NOT grammar keywords.
 *
 * Operations such as:
 *
 *     traversal
 *     shortest_path
 *     reachability
 *     connected_components
 *     centrality
 *     matching
 *     clustering
 *     ranking
 *     graph_learning
 *
 * remain semantic/library capabilities.
 *
 * They may be represented by ordinary Zamani expressions and resolved by
 * semantic analysis.
 *
 * This prevents the language from requiring a grammar change whenever a new
 * graph algorithm is introduced.
 *
 * ============================================================================
 * HARDWARE / QUANTUM CONTRACT
 * ============================================================================
 *
 * A graph may participate in:
 *
 *     classical computation
 *     AI
 *     symbolic reasoning
 *     distributed computation
 *     tensor computation
 *     quantum/classical hybrid computation
 *     HDL/hardware workflows
 *
 * but this grammar does not encode those physical realizations.
 *
 * If graph computation contributes to a quantum workflow, the semantic
 * pipeline remains:
 *
 *     graph source
 *         ->
 *     domain-neutral AST
 *         ->
 *     semantic model
 *         ->
 *     quantum semantic operations where applicable
 *         ->
 *     quantum::ir
 *
 * This file never constructs quantum::ir directly.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source tokens
 *     grammar version
 *     parser configuration
 *     imported grammar definitions
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware
 *     memory availability
 *     graph size at runtime
 *     filesystem state
 *     network state
 *     randomness
 *     wall-clock time
 *     target availability
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Public integration rules:
 *
 *     dataGraphConstruct
 *     graphDeclaration
 *     graphStatement
 *     graphPattern
 *     graphPatternDeclaration
 *
 * are stable composition boundaries.
 *
 * Internal rules may evolve provided the public integration rules retain their
 * semantics or receive an explicit compatibility transition.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * Repetition is expressed using ANTLR's unbounded repetition operators:
 *
 *     *
 *     +
 *
 * No finite language-level graph size is encoded.
 *
 * This includes:
 *
 *     graph members
 *     nodes
 *     relations
 *     properties
 *     patterns
 *     pattern elements
 *     guards
 *     graph options
 *
 * Compiler/runtime resource exhaustion is not a language-level graph limit.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * Immediate consumer:
 *
 *     grammar/data/data.g4
 *
 * Required integration:
 *
 *     data.g4
 *         imports Graphs
 *              |
 *              v
 *         dataGraphConstruct
 *
 * The data dispatcher should expose:
 *
 *     dataGraphConstruct
 *
 * as a data-domain declaration/construct.
 *
 * Query integration:
 *
 *     grammar/data/queries.g4
 *
 * may consume:
 *
 *     graphPattern
 *
 * for graph-aware query sources.
 *
 * Schema integration:
 *
 *     grammar/data/schemas.g4
 *
 * remains authoritative for reusable schema/type semantics.
 *
 * Expression integration:
 *
 *     grammar/expressions/expressions.g4
 *
 * remains authoritative for expressions.
 *
 * Provenance integration:
 *
 *     grammar/data/provenance.g4
 *
 * remains authoritative for provenance.
 *
 * Resource integration:
 *
 *     grammar/resources/
 *
 * remains authoritative for resource requirements and capabilities.
 *
 * ============================================================================
 * INTEGRATION RULE
 * ============================================================================
 *
 * The data dispatcher must import this grammar by grammar name:
 *
 *     Graphs
 *
 * and consume:
 *
 *     dataGraphConstruct
 *
 * It must NOT copy graph rules into data.g4.
 *
 * ============================================================================
 */

parser grammar Graphs;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * PUBLIC GRAPH COMPOSITION
 * ============================================================================
 *
 * This is the primary integration rule for grammar/data/data.g4.
 *
 * A graph construct is a declaration-level graph object.
 *
 * Graph statements and patterns are contained inside the graph declaration.
 */
dataGraphConstruct
    : graphDeclaration
    ;


/*
 * ============================================================================
 * GRAPH DECLARATION
 * ============================================================================
 *
 * Example:
 *
 *     graph social {
 *         ...
 *     }
 *
 * The graph name is intentionally an ordinary identifier.
 *
 * ============================================================================
 */

graphDeclaration
    : GRAPH
      IDENTIFIER
      graphOptionBlock?
      LBRACE
      graphMember*
      RBRACE
    ;


/*
 * ============================================================================
 * GRAPH OPTIONS
 * ============================================================================
 *
 * Options are open-world.
 *
 * New graph implementation capabilities do not require new universal grammar
 * keywords.
 *
 * Example:
 *
 *     graph social(
 *         mode = distributed,
 *         consistency = eventual
 *     ) {
 *         ...
 *     }
 *
 * The values remain ordinary Zamani expressions.
 *
 * ============================================================================
 */

graphOptionBlock
    : LPAREN
      graphOptionList?
      RPAREN
    ;

graphOptionList
    : graphOption
      (
          COMMA
          graphOption
      )*
      COMMA?
    ;

graphOption
    : IDENTIFIER
      (
          ASSIGN
          expression
      )?
    ;


/*
 * ============================================================================
 * GRAPH MEMBERS
 * ============================================================================
 */

graphMember
    : graphNodeDeclaration
    | graphRelationDeclaration
    | graphPropertyDeclaration
    | graphPatternDeclaration
    | graphStatement
    ;


/*
 * ============================================================================
 * NODE DECLARATION
 * ============================================================================
 *
 * A node declaration can express:
 *
 *     a named logical node
 *     a node with a semantic label/type
 *     a node with properties
 *
 * Examples:
 *
 *     node alice;
 *
 *     node alice : Person;
 *
 *     node alice : Person {
 *         name = "Alice";
 *         age = 42;
 *     }
 *
 * The semantic layer decides whether a node declaration is:
 *
 *     schema-like
 *     instance-like
 *     symbolic
 *     persistent
 *     transient
 *
 * ============================================================================
 */

graphNodeDeclaration
    : NODE
      IDENTIFIER
      graphNodeTypeClause?
      graphPropertyBlock?
      SEMICOLON
    ;

graphNodeTypeClause
    : COLON
      expression
    ;


/*
 * ============================================================================
 * RELATION DECLARATION
 * ============================================================================
 *
 * Relations are deliberately generic.
 *
 * The syntax does not define a fixed edge/relationship catalogue.
 *
 * Example:
 *
 *     relation knows;
 *
 *     relation alice -> bob : knows;
 *
 *     relation alice -> bob : knows {
 *         since = 2026;
 *     }
 *
 * The relationship type is an expression so future semantic relation systems
 * do not require a grammar change.
 *
 * ============================================================================
 */

graphRelationDeclaration
    : RELATION
      graphRelationBody
      SEMICOLON
    ;

graphRelationBody
    : graphRelationTypeOnly
    | graphRelationInstance
    ;

graphRelationTypeOnly
    : IDENTIFIER
      graphRelationTypeClause?
      graphPropertyBlock?
    ;

graphRelationInstance
    : expression
      THIN_ARROW
      expression
      graphRelationTypeClause?
      graphPropertyBlock?
    ;

graphRelationTypeClause
    : COLON
      expression
    ;


/*
 * ============================================================================
 * GRAPH PROPERTY DECLARATIONS
 * ============================================================================
 *
 * Property declarations provide graph-level property vocabulary.
 *
 * The actual type system remains outside this file.
 *
 * Examples:
 *
 *     property name;
 *
 *     property age : integer;
 *
 *     property active = true;
 *
 * The semantic layer determines whether a property declaration describes:
 *
 *     a schema property
 *     a default
 *     a constraint
 *     a graph-level metadata field
 *
 * ============================================================================
 */

graphPropertyDeclaration
    : PROPERTY
      IDENTIFIER
      graphPropertyTypeClause?
      graphPropertyDefaultClause?
      SEMICOLON
    ;

graphPropertyTypeClause
    : COLON
      expression
    ;

graphPropertyDefaultClause
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * PROPERTY BLOCK
 * ============================================================================
 *
 * Property blocks are deliberately value-oriented.
 *
 * They do not create a second object/map grammar.
 *
 * Example:
 *
 *     {
 *         name = "Alice";
 *         score = 0.95;
 *     }
 *
 * ============================================================================
 */

graphPropertyBlock
    : LBRACE
      graphPropertyAssignment*
      RBRACE
    ;

graphPropertyAssignment
    : graphPropertyKey
      ASSIGN
      expression
      SEMICOLON?
    ;

graphPropertyKey
    : IDENTIFIER
    | STRING
    ;


/*
 * ============================================================================
 * GRAPH STATEMENTS
 * ============================================================================
 */

graphStatement
    : graphAssertion
    | graphRetraction
    | graphExpressionStatement
    ;


/*
 * ============================================================================
 * GRAPH ASSERTION
 * ============================================================================
 *
 * Assertions describe logical graph-state additions.
 *
 * They do NOT imply:
 *
 *     physical insertion
 *     database mutation
 *     network mutation
 *     persistence
 *     transaction commit
 *
 * Those semantics belong downstream.
 *
 * ============================================================================
 */

graphAssertion
    : ASSERT
      graphFact
      SEMICOLON
    ;

graphFact
    : graphNodeFact
    | graphRelationFact
    ;

graphNodeFact
    : NODE
      IDENTIFIER
      graphNodeTypeClause?
      graphPropertyBlock?
    ;

graphRelationFact
    : RELATION
      expression
      THIN_ARROW
      expression
      graphRelationTypeClause?
      graphPropertyBlock?
    ;


/*
 * ============================================================================
 * GRAPH RETRACTION
 * ============================================================================
 *
 * Retraction has the same logical shape as assertion.
 *
 * The runtime decides how the operation is realized.
 *
 * ============================================================================
 */

graphRetraction
    : RETRACT
      graphFact
      SEMICOLON
    ;


/*
 * ============================================================================
 * GRAPH EXPRESSION STATEMENT
 * ============================================================================
 *
 * Graph algorithms and graph operations that are not part of the graph data
 * model remain ordinary Zamani expressions.
 *
 * Examples of semantic/library-level operations that can therefore be
 * represented without adding grammar keywords include:
 *
 *     traverse(graph, start)
 *     shortest_path(graph, source, target)
 *     connected_components(graph)
 *     centrality(graph)
 *     match(graph, pattern)
 *
 * The grammar does not need to know which algorithms exist.
 *
 * ============================================================================
 */

graphExpressionStatement
    : expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * GRAPH PATTERNS
 * ============================================================================
 *
 * Patterns are reusable graph structures.
 *
 * They are intentionally separated from query execution.
 *
 * A query grammar may consume these structures later.
 *
 * Example:
 *
 *     pattern friendship {
 *         node person;
 *         relation person -> friend : knows;
 *         guard person != friend;
 *     }
 *
 * ============================================================================
 */

graphPatternDeclaration
    : PATTERN
      IDENTIFIER
      LBRACE
      graphPatternElement*
      RBRACE
    ;

graphPattern
    : PATTERN
      graphPatternBody
    ;

graphPatternBody
    : LBRACE
      graphPatternElement*
      RBRACE
    ;


/*
 * ============================================================================
 * GRAPH PATTERN ELEMENTS
 * ============================================================================
 */

graphPatternElement
    : graphPatternNode
    | graphPatternRelation
    | graphPatternPropertyConstraint
    | graphPatternGuard
    | graphPatternAssertion
    ;


/*
 * ============================================================================
 * PATTERN NODE
 * ============================================================================
 *
 * A pattern node has a symbolic binding.
 *
 * Example:
 *
 *     node person : Person;
 *
 * ============================================================================
 */

graphPatternNode
    : NODE
      IDENTIFIER
      graphNodeTypeClause?
      graphPatternPropertyBlock?
      SEMICOLON
    ;


/*
 * ============================================================================
 * PATTERN RELATION
 * ============================================================================
 *
 * Example:
 *
 *     relation person -> friend : knows;
 *
 * The direction operator is syntactic structure only.
 *
 * The semantic model may later represent:
 *
 *     directed
 *     undirected
 *     bidirectional
 *     constrained direction
 *     implementation-specific traversal
 *
 * without changing this grammar.
 *
 * ============================================================================
 */

graphPatternRelation
    : RELATION
      expression
      THIN_ARROW
      expression
      graphRelationTypeClause?
      graphPatternPropertyBlock?
      SEMICOLON
    ;


/*
 * ============================================================================
 * PATTERN PROPERTIES
 * ============================================================================
 *
 * Property constraints use ordinary expressions.
 *
 * Examples:
 *
 *     property age = 18;
 *
 *     property score = score > threshold;
 *
 * ============================================================================
 */

graphPatternPropertyBlock
    : LBRACE
      graphPatternPropertyConstraint*
      RBRACE
    ;

graphPatternPropertyConstraint
    : PROPERTY
      graphPropertyKey
      (
          ASSIGN
          expression
      )?
      SEMICOLON?
    ;


/*
 * ============================================================================
 * PATTERN GUARDS
 * ============================================================================
 *
 * Guards are ordinary Zamani expressions.
 *
 * This grammar does not define a graph-specific boolean language.
 *
 * Example:
 *
 *     guard person != friend;
 *
 * ============================================================================
 */

graphPatternGuard
    : GUARD
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * PATTERN ASSERTION
 * ============================================================================
 *
 * Allows a pattern to contain an explicitly asserted logical graph fact.
 *
 * ============================================================================
 */

graphPatternAssertion
    : ASSERT
      graphFact
      SEMICOLON
    ;


/*
 * ============================================================================
 * REUSABLE GRAPH PATTERN REFERENCE
 * ============================================================================
 *
 * This rule intentionally uses an expression rather than introducing a second
 * name/reference system.
 *
 * A semantic resolver can determine whether the expression denotes a graph
 * pattern, graph value, graph variable, or other compatible object.
 *
 * ============================================================================
 */

graphPatternReference
    : expression
    ;


/*
 * ============================================================================
 * GRAPH DATA SOURCE
 * ============================================================================
 *
 * This bridge allows a graph to participate in ordinary Zamani data
 * composition without defining a storage-specific source grammar.
 *
 * ============================================================================
 */

graphDataExpression
    : expression
    ;


/*
 * ============================================================================
 * OPEN-WORLD GRAPH TERM
 * ============================================================================
 *
 * Graph terms are expressions.
 *
 * This is important for universal computation because a graph endpoint may
 * eventually be:
 *
 *     scalar
 *     symbolic value
 *     record
 *     collection member
 *     tensor-derived value
 *     stream value
 *     distributed value
 *     quantum measurement result
 *     hybrid value
 *
 * The graph grammar does not need a separate grammar for every such domain.
 *
 * ============================================================================
 */

graphTerm
    : expression
    ;


/*
 * ============================================================================
 * GRAPH TERM LIST
 * ============================================================================
 */

graphTermList
    : graphTerm
      (
          COMMA
          graphTerm
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * GRAPH PROPERTY VALUE
 * ============================================================================
 *
 * Values remain canonical Zamani expressions.
 *
 * ============================================================================
 */

graphPropertyValue
    : expression
    ;


/*
 * ============================================================================
 * GRAPH RELATION ENDPOINTS
 * ============================================================================
 *
 * This named rule provides a stable semantic boundary without defining a new
 * endpoint type system.
 *
 * ============================================================================
 */

graphRelationEndpoints
    : graphTerm
      THIN_ARROW
      graphTerm
    ;


/*
 * ============================================================================
 * GRAPH CONSTRAINT EXPRESSION
 * ============================================================================
 *
 * Constraints remain ordinary Zamani expressions.
 *
 * A future semantic validator may interpret the expression as a graph
 * constraint without changing this grammar.
 *
 * ============================================================================
 */

graphConstraintExpression
    : expression
    ;


/*
 * ============================================================================
 * GRAPH MATCH INPUT
 * ============================================================================
 *
 * Query systems can consume graph patterns through this stable rule.
 *
 * ============================================================================
 */

graphMatchInput
    : graphPatternReference
    ;


/*
 * ============================================================================
 * GRAPH SEMANTIC VALUE
 * ============================================================================
 *
 * A semantic graph value is represented by the canonical expression system.
 *
 * ============================================================================
 */

graphValue
    : expression
    ;


/*
 * ============================================================================
 * COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is COMPLETE when:
 *
 * [x] Graphs is a parser grammar.
 *
 * [x] tokenVocab is ZamaniLexer.
 *
 * [x] No lexer tokens are defined here.
 *
 * [x] No embedded Rust exists.
 *
 * [x] No unsafe implementation is required.
 *
 * [x] Rust 1.97+ compatibility is preserved.
 *
 * [x] General expressions are delegated to expression.
 *
 * [x] No second expression grammar exists.
 *
 * [x] No second type system exists.
 *
 * [x] No second name system exists.
 *
 * [x] No graph-specific IR exists.
 *
 * [x] No graph-specific AST is mandated.
 *
 * [x] Graph size is not bounded.
 *
 * [x] Node count is not bounded.
 *
 * [x] Relation count is not bounded.
 *
 * [x] Property count is not bounded.
 *
 * [x] Pattern size is not bounded.
 *
 * [x] Path length is not bounded by grammar constants.
 *
 * [x] Algorithm names are not hard-coded.
 *
 * [x] Storage engines are not hard-coded.
 *
 * [x] Hardware is not hard-coded.
 *
 * [x] Quantum hardware is not hard-coded.
 *
 * [x] Distributed topology is not hard-coded.
 *
 * [x] Vendor APIs are not hard-coded.
 *
 * [x] Graph queries do not create a competing query language.
 *
 * [x] Graph schema semantics remain compatible with data/schemas.g4.
 *
 * [x] Provenance remains compatible with data/provenance.g4.
 *
 * [x] Resource semantics remain downstream.
 *
 * [x] Capability semantics remain downstream.
 *
 * [x] Policy semantics remain downstream.
 *
 * [x] Effect semantics remain downstream.
 *
 * [x] Canonical IR remains downstream.
 *
 * [x] quantum::ir remains the canonical quantum boundary.
 *
 * ============================================================================
 * REQUIRED TEST CATEGORIES
 * ============================================================================
 *
 * The repository test suite must cover this grammar independently before
 * integrating it into the data dispatcher.
 *
 * POSITIVE:
 *
 *     graph social {
 *         node alice;
 *         node bob;
 *         relation alice -> bob : knows;
 *     }
 *
 *     graph social {
 *         node alice : Person {
 *             name = "Alice";
 *             age = 42;
 *         }
 *
 *         relation alice -> bob : knows {
 *             since = 2026;
 *         }
 *     }
 *
 *     graph social {
 *         property name;
 *         property age : integer;
 *         property active = true;
 *     }
 *
 *     graph social {
 *         pattern friendship {
 *             node person : Person;
 *             node friend : Person;
 *             relation person -> friend : knows;
 *             guard person != friend;
 *         }
 *     }
 *
 *     graph knowledge {
 *         assert node fact;
 *         assert relation source -> target : relation;
 *         retract node obsolete;
 *     }
 *
 * NEGATIVE:
 *
 *     graph;
 *
 *     graph 123;
 *
 *     graph social {
 *         node;
 *     }
 *
 *     graph social {
 *         relation -> bob;
 *     }
 *
 *     graph social {
 *         relation alice ->;
 *     }
 *
 *     graph social {
 *         property;
 *     }
 *
 *     graph social {
 *         pattern {
 *         }
 *     }
 *
 * BOUNDARY:
 *
 *     deeply nested expressions;
 *     large property blocks;
 *     large pattern bodies;
 *     large graph member sequences;
 *     long identifiers;
 *     Unicode identifiers accepted by the canonical lexer;
 *     large expression values;
 *     nested graph-related expressions;
 *     graph references across modules;
 *     graph values crossing classical/data/AI domains;
 *     graph values participating in hybrid computation.
 *
 * SCALABILITY:
 *
 * Tests must increase:
 *
 *     graph member count;
 *     node count;
 *     relation count;
 *     property count;
 *     pattern element count;
 *     nesting depth;
 *     identifier length;
 *     expression complexity;
 *
 * without changing this grammar.
 *
 * CROSS-DOMAIN:
 *
 *     graph + classical
 *     graph + AI
 *     graph + reasoning
 *     graph + knowledge
 *     graph + distributed
 *     graph + networking
 *     graph + tensor
 *     graph + quantum/classical
 *     graph + HDL/hardware intent
 *
 * The graph grammar must remain unchanged when the target domain changes.
 *
 * DETERMINISM:
 *
 * The same source and same grammar configuration must produce the same parse
 * structure regardless of:
 *
 *     hardware
 *     graph size
 *     target availability
 *     runtime state
 *     filesystem state
 *     network state
 *     randomness
 *
 * ============================================================================
 * DOWNSTREAM INTEGRATION MATRIX
 * ============================================================================
 *
 * graphDeclaration
 *     |
 *     +--> domain-neutral AST GraphDeclaration
 *     |
 *     +--> semantic graph model
 *     |
 *     +--> data semantic representation
 *     |
 *     +--> canonical IR
 *
 * graphNodeDeclaration
 *     |
 *     +--> graph node semantic object
 *
 * graphRelationDeclaration
 *     |
 *     +--> graph relation semantic object
 *
 * graphPropertyDeclaration
 *     |
 *     +--> graph property/schema semantic object
 *
 * graphPatternDeclaration
 *     |
 *     +--> graph pattern semantic object
 *     |
 *     +--> data query planner
 *
 * graphAssertion
 *     |
 *     +--> logical graph mutation intent
 *     |
 *     +--> effect analysis
 *     |
 *     +--> policy analysis
 *     |
 *     +--> provenance
 *
 * graphRetraction
 *     |
 *     +--> logical graph mutation intent
 *     |
 *     +--> effect analysis
 *     |
 *     +--> policy analysis
 *     +--> provenance
 *
 * ============================================================================
 * FINAL ARCHITECTURAL GUARANTEE
 * ============================================================================
 *
 * Graphs remain one domain of Zamani rather than a second language.
 *
 * The complete path is:
 *
 *     graph source
 *          |
 *          v
 *     Zamani lexer
 *          |
 *          v
 *     Graphs parser grammar
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     name/type/effect/resource/capability analysis
 *          |
 *          v
 *     graph semantic model
 *          |
 *          v
 *     canonical semantic/IR layer
 *          |
 *          +-------------------------------+
 *          |               |               |
 *          v               v               v
 *       classical      distributed      accelerator
 *          |               |               |
 *          +---------------+---------------+
 *                          |
 *                          v
 *                   target realization
 *
 * Quantum participation remains:
 *
 *     graph semantics
 *          |
 *          v
 *     hybrid semantic model
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     resilience / QEC
 *          |
 *          v
 *     ZQN
 *          |
 *          v
 *     HAL
 *
 * No graph-specific physical assumptions are introduced.
 *
 * ============================================================================
 */
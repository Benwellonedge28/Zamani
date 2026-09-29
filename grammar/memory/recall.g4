/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/memory/recall.g4
 *
 * Grammar:
 *     Recall
 *
 * Grammar kind:
 *     ANTLR4 parser grammar
 *
 * Domain:
 *     Memory / Sankofa
 *
 * Status:
 *     CANONICAL RECALL SYNTAX COMPONENT
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Rust edition:
 *     2021
 *
 * Safety:
 *     Safe Rust only.
 *
 *     This grammar contains no embedded Rust actions and requires no
 *     `unsafe` Rust.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the source-level syntax of the Zamani `recall` construct.
 *
 * `recall` expresses an intent to retrieve, query, recover, or otherwise
 * obtain information from a semantic source.
 *
 * The semantic source may represent, depending on semantic analysis:
 *
 *     memory
 *     knowledge
 *     a memory key
 *     a query
 *     temporal information
 *     a provenance-aware source
 *     another valid Sankofa source
 *     an implementation-independent memory abstraction
 *
 * The grammar deliberately does NOT decide which of those meanings applies.
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns:
 *
 *     recallStatement
 *     recallExpression
 *     recallFromClause
 *     recallWithClause
 *     recallArgumentList
 *     recallArgument
 *     recallNamedArgument
 *
 * It owns the syntactic relationship between:
 *
 *     recall
 *     source/query expression
 *     optional `from` expression
 *     optional `with (...)` arguments
 *     statement terminator
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     remember
 *     learn
 *     infer
 *     wisdom
 *     MTS temporal type syntax
 *     zamani/sasa temporal block syntax
 *     generic memory syntax
 *     memory allocation
 *     ownership
 *     borrowing
 *     lifetimes
 *     memory regions
 *     address spaces
 *     persistence
 *     distributed memory
 *     accelerator memory
 *     quantum memory
 *     provenance implementation
 *     retrieval algorithms
 *     databases
 *     caches
 *     indexing
 *     search algorithms
 *     learning algorithms
 *     inference algorithms
 *     temporal execution
 *     timeline management
 *     branch management
 *     scheduling
 *     routing
 *     resource discovery
 *     hardware selection
 *     QEC
 *     ZQN
 *     HAL
 *     runtime execution
 *
 * Those responsibilities remain in their respective language or implementation
 * layers.
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * `recall.g4` is a leaf grammar.
 *
 * It MUST NOT become another parser root.
 *
 * The composition relationship is:
 *
 *     grammar/memory/recall.g4
 *                  |
 *                  v
 *     grammar/memory/sankofa.g4
 *                  |
 *                  v
 *     grammar/antlr/ZamaniParser.g4
 *                  |
 *                  v
 *             Zamani parser
 *
 * `Zamani.g4` remains the canonical public grammar composition root.
 *
 * ============================================================================
 * AUTHORITY
 * ============================================================================
 *
 * Normative syntax:
 *
 *     grammar/spec/syntax.md
 *
 * Memory architecture:
 *
 *     grammar/memory/README.md
 *
 * Sankofa composition:
 *
 *     grammar/memory/sankofa.g4
 *
 * Generic memory foundation:
 *
 *     grammar/memory/memory.g4
 *
 * Canonical expression syntax:
 *
 *     grammar/expressions/
 *
 * Canonical names:
 *
 *     grammar/core/
 *     grammar/expressions/
 *
 * Canonical types:
 *
 *     grammar/types/
 *
 * Canonical lexical vocabulary:
 *
 *     grammar/lexer/
 *
 * Canonical parser composition:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * Executable frontend:
 *
 *     src/lexer.rs
 *     src/parser.rs
 *     src/ast/
 *     src/semantic.rs
 *     src/ir_gen.rs
 *
 * ============================================================================
 * TOKEN AUTHORITY
 * ============================================================================
 *
 * This parser grammar consumes:
 *
 *     ZamaniLexer
 *
 * It does not define lexer tokens.
 *
 * The following existing canonical tokens are used:
 *
 *     RECALL
 *     FROM
 *     WITH
 *     LPAREN
 *     RPAREN
 *     COMMA
 *     ASSIGN
 *     SEMICOLON
 *
 * Names and expressions are delegated to their existing grammar owners.
 *
 * No recall-specific lexical tokens are introduced here.
 *
 * ============================================================================
 * OPEN-WORLD PRINCIPLE
 * ============================================================================
 *
 * Recall MUST NOT contain an exhaustive list of memory technologies,
 * databases, retrieval algorithms, knowledge systems, or temporal mechanisms.
 *
 * This is deliberately NOT:
 *
 *     recallSource
 *         : MEMORY
 *         | DATABASE
 *         | CACHE
 *         | TIMELINE
 *         | ...
 *         ;
 *
 * Instead, the operand is represented using the canonical expression grammar.
 *
 * This allows semantic analysis to resolve future sources without requiring
 * this grammar to change whenever a new memory technology appears.
 *
 * ============================================================================
 * CANONICAL SOURCE FORMS
 * ============================================================================
 *
 * The established source forms are:
 *
 *     recall expression;
 *
 *     recall expression from expression;
 *
 *     recall expression with (...);
 *
 *     recall expression from expression with (...);
 *
 * Examples:
 *
 *     recall key;
 *
 *     recall query;
 *
 *     recall memory;
 *
 *     recall key from memory;
 *
 *     recall query with (limit = n);
 *
 *     recall query from history with (policy = policy);
 *
 * These examples describe syntax only.
 *
 * Whether a particular expression represents a memory, key, query, temporal
 * source, policy, or another semantic object is determined downstream.
 *
 * ============================================================================
 * EXPRESSION REUSE
 * ============================================================================
 *
 * This grammar MUST reuse the canonical `expression` rule.
 *
 * It must not define:
 *
 *     recallExpressionLanguage
 *     recallQueryLanguage
 *     recallMemoryExpression
 *
 * as competing expression systems.
 *
 * This guarantees that recall participates in the same:
 *
 *     precedence
 *     associativity
 *     literals
 *     identifiers
 *     qualified names
 *     calls
 *     indexing
 *     member access
 *     operators
 *     generic expression composition
 *
 * as the rest of Zamani.
 *
 * ============================================================================
 * FROM CLAUSE
 * ============================================================================
 *
 * `from` supplies an optional semantic source.
 *
 * Syntax:
 *
 *     from expression
 *
 * The expression is intentionally unrestricted at grammar level.
 *
 * Examples:
 *
 *     recall key from memory;
 *     recall query from history;
 *     recall value from zamani;
 *     recall state from source;
 *
 * Whether the source is valid is a semantic question.
 *
 * The grammar must not hard-code:
 *
 *     memory names
 *     database names
 *     timeline names
 *     node names
 *     device names
 *     physical addresses
 *     storage technologies
 *
 * ============================================================================
 * WITH CLAUSE
 * ============================================================================
 *
 * `with (...)` carries optional recall parameters.
 *
 * It is deliberately open-ended.
 *
 * Examples:
 *
 *     recall query with (limit = n);
 *
 *     recall query with (
 *         policy = policy,
 *         consistency = consistency
 *     );
 *
 *     recall query from history with (
 *         temporal = reference,
 *         provenance = requirement
 *     );
 *
 * Named arguments use ordinary Zamani identifiers.
 *
 * There is no closed property list.
 *
 * Therefore future semantic policies do not require grammar changes merely
 * because a new named parameter is introduced.
 *
 * ============================================================================
 * TRAILING COMMA
 * ============================================================================
 *
 * A trailing comma is accepted in a `with` argument list:
 *
 *     recall query with (
 *         policy = p,
 *     );
 *
 * This follows the existing Sankofa argument-list convention.
 *
 * It is a syntactic convenience only.
 *
 * ============================================================================
 * STATEMENT / EXPRESSION BOUNDARY
 * ============================================================================
 *
 * Two public forms are provided:
 *
 *     recallStatement
 *
 * and:
 *
 *     recallExpression
 *
 * `recallStatement` owns the statement terminator.
 *
 * `recallExpression` does not.
 *
 * This permits the composition grammar to choose the correct surrounding
 * syntactic context without duplicating recall syntax.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This file creates parser contexts only.
 *
 * The domain-neutral frontend AST must preserve sufficient information to
 * represent:
 *
 *     source span
 *     recall operation identity
 *     requested expression
 *     optional source expression
 *     optional named arguments
 *     positional arguments
 *     source ordering
 *     syntactic metadata
 *
 * The existing Rust AST already contains:
 *
 *     Expression::Recall(Span, Box<Expression>)
 *
 * Therefore this grammar MUST remain compatible with that existing
 * representation while allowing the AST to evolve to preserve additional
 * syntax that the current compatibility representation does not yet retain.
 *
 * In particular, the optional:
 *
 *     from
 *
 * and:
 *
 *     with (...)
 *
 * information must not be silently discarded when the frontend evolves.
 *
 * No backend-specific AST node may be required.
 *
 * Forbidden examples include:
 *
 *     DatabaseRecallNode
 *     RedisRecallNode
 *     GPURecallNode
 *     QPURecallNode
 *     PhysicalMemoryRecallNode
 *     TimelineRecallNode
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     whether the recall operation is valid;
 *     what the requested expression denotes;
 *     whether the source expression is valid;
 *     whether source and query types are compatible;
 *     whether named arguments are valid;
 *     whether positional arguments are valid;
 *     whether requested information exists;
 *     whether temporal relationships are valid;
 *     whether provenance requirements are valid;
 *     whether ownership/lifetime requirements are satisfied;
 *     whether required capabilities exist;
 *     whether required resources are available;
 *     whether the operation is deterministic where required;
 *     whether the operation is permitted by security policy.
 *
 * None of those checks belong in this grammar.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY CONTRACT
 * ============================================================================
 *
 * Recall may semantically require capabilities such as:
 *
 *     memory.read
 *     memory.persistence
 *     memory.temporal
 *     memory.provenance
 *     memory.query
 *     distributed.communication
 *     knowledge.retrieval
 *
 * Such requirements are resolved by the resource/capability subsystem.
 *
 * The grammar MUST NOT convert capabilities into hardware assumptions.
 *
 * Valid conceptual intent:
 *
 *     requires capability("memory.temporal")
 *
 * does not select:
 *
 *     node 0
 *     device 0
 *     memory bank 0
 *     database 0
 *
 * ============================================================================
 * TEMPORAL INTEGRATION
 * ============================================================================
 *
 * Recall can participate in temporal/Sankofa computation.
 *
 * Temporal meaning is semantic.
 *
 * This file therefore does NOT define:
 *
 *     TIMELINE
 *     FORK
 *     MERGE
 *     REWIND
 *     SNAPSHOT
 *     HISTORY
 *
 * as parser keywords.
 *
 * The current canonical lexer does not provide those dedicated tokens, and
 * introducing them here would create a lexical/grammar authority violation.
 *
 * Future temporal operations can be represented by existing expression and
 * qualified-name mechanisms where the surrounding language permits them.
 *
 * The existing temporal type constructor:
 *
 *     MTS<T>
 *
 * is owned by:
 *
 *     grammar/types/temporal.g4
 *
 * It is NOT redefined here.
 *
 * `zamani`, `sasa`, and `mts` remain canonical lexical vocabulary owned by the
 * lexer. Their semantic interpretation remains downstream.
 *
 * ============================================================================
 * SANKOFA INTEGRATION
 * ============================================================================
 *
 * `recall.g4` is a specialized component of:
 *
 *     grammar/memory/sankofa.g4
 *
 * Sankofa owns the higher-level composition:
 *
 *     sankofaStatement
 *     sankofaExpression
 *
 * The integration must be:
 *
 *     sankofaStatement
 *         |
 *         +--> sankofaRemember
 *         +--> recallStatement
 *         +--> sankofaLearn
 *         +--> sankofaInfer
 *         +--> sankofaWisdom
 *
 * and:
 *
 *     sankofaExpression
 *         |
 *         +--> recallExpression
 *         +--> sankofaLearnExpression
 *         +--> sankofaInferExpression
 *
 * `sankofa.g4` must not duplicate the actual recall productions after this
 * file is integrated.
 *
 * ============================================================================
 * MEMORY INTEGRATION
 * ============================================================================
 *
 * Recall consumes generic expressions rather than redefining memory places.
 *
 * Therefore it remains compatible with:
 *
 *     grammar/memory/memory.g4
 *     grammar/memory/references.g4
 *     grammar/memory/ownership.g4
 *     grammar/memory/borrowing.g4
 *     grammar/memory/lifetimes.g4
 *     grammar/memory/regions.g4
 *     grammar/memory/persistence.g4
 *     grammar/memory/shared-memory.g4
 *     grammar/memory/distributed-memory.g4
 *     grammar/memory/accelerator-memory.g4
 *     grammar/memory/quantum-memory.g4
 *
 * without importing all of those grammars here.
 *
 * This avoids circular grammar dependencies.
 *
 * ============================================================================
 * TYPE INTEGRATION
 * ============================================================================
 *
 * The recall operand is an ordinary `expression`.
 *
 * Result typing is semantic.
 *
 * Recall may therefore eventually produce:
 *
 *     scalar values
 *     structured values
 *     collections
 *     tensors
 *     classical states
 *     quantum-associated classical information
 *     temporal values
 *     knowledge values
 *     user-defined types
 *
 * The grammar must not enumerate those result types.
 *
 * Type validation belongs to:
 *
 *     grammar/types/
 *
 * and semantic/type analysis.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Recall may participate in hybrid quantum-classical programs.
 *
 * For example, semantic systems may use recall to obtain classical information
 * later consumed by quantum computation.
 *
 * This grammar does not define quantum semantics.
 *
 * In particular it does not own:
 *
 *     qubit allocation
 *     quantum state
 *     measurement
 *     gate semantics
 *     routing
 *     scheduling
 *     QEC
 *     ZQN
 *     calibration
 *     physical qubit mapping
 *
 * If a recalled value participates in quantum computation, the downstream
 * architecture remains:
 *
 *     source
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     routing / scheduling / resilience
 *       |
 *       v
 *     QEC / ZQN
 *       |
 *       v
 *     HAL
 *
 * No recall-specific quantum IR is introduced.
 *
 * ============================================================================
 * CLASSICAL / HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Recall remains domain-neutral.
 *
 * A recalled value may semantically participate in:
 *
 *     classical computation
 *     AI/ML
 *     tensor/data processing
 *     distributed computation
 *     networking
 *     HDL/hardware co-design
 *     accelerator computation
 *     quantum-classical computation
 *
 * The recall grammar does not select the eventual target.
 *
 * ============================================================================
 * OPEN EXTENSION CONTRACT
 * ============================================================================
 *
 * Future semantic recall facilities must prefer:
 *
 *     existing expression syntax
 *     qualified names
 *     named arguments
 *     semantic registration
 *     capability declarations
 *     explicit dialect mechanisms
 *
 * rather than introducing another reserved keyword for every new retrieval
 * technology.
 *
 * Conceptual future examples may include:
 *
 *     recall query from sankofa::history;
 *     recall value from sankofa::provenance;
 *     recall state from domain::source with (policy = p);
 *
 * Whether such forms are valid depends on the canonical expression grammar
 * and semantic registration.
 *
 * This grammar deliberately does not hard-code the operation universe.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar defines no universal finite limit for:
 *
 *     recall statements
 *     recall expressions
 *     nested expressions
 *     named arguments
 *     positional arguments
 *     memory records
 *     history records
 *     temporal states
 *     knowledge items
 *     provenance records
 *     memory domains
 *     distributed sources
 *     timelines
 *     branches
 *     nodes
 *     devices
 *     accelerators
 *     qubits
 *     CPUs
 *     GPUs
 *     FPGAs
 *     QPUs
 *     tensor dimensions
 *     memory capacity
 *
 * Repetition is represented structurally through ordinary grammar repetition.
 *
 * Actual limits imposed by:
 *
 *     parser implementation
 *     compiler memory
 *     operating system
 *     runtime
 *     target hardware
 *     resource availability
 *
 * are implementation/resource constraints, not language grammar limits.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This file MUST NOT define:
 *
 *     MAX_RECALLS
 *     MAX_RECALL_DEPTH
 *     MAX_RECALL_ARGUMENTS
 *     MAX_MEMORY
 *     MAX_MEMORY_ITEMS
 *     MAX_HISTORY
 *     MAX_TIMELINES
 *     MAX_BRANCHES
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_ACCELERATORS
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *
 * It must also not encode:
 *
 *     physical addresses
 *     physical memory banks
 *     fixed node IDs
 *     fixed device IDs
 *     vendor-specific memory systems
 *     fixed database engines
 *     fixed retrieval engines
 *
 * Numeric literals appearing inside expressions remain ordinary program data.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * The same recall source syntax must remain semantically expressible across:
 *
 *     tiny embedded targets
 *     CPUs
 *     multicore systems
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     QPUs
 *     quantum simulators
 *     HPC systems
 *     clusters
 *     distributed systems
 *     cloud systems
 *     future computational substrates
 *
 * Target realization may differ.
 *
 * Source syntax does not.
 *
 * The compiler/runtime may select different:
 *
 *     memory representations
 *     indexes
 *     storage systems
 *     communication mechanisms
 *     accelerators
 *     execution strategies
 *
 * without requiring the source language to introduce a new `recall` syntax
 * for each target.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing of recall constructs depends only on:
 *
 *     token stream
 *     grammar version
 *     parser configuration
 *
 * It must not depend on:
 *
 *     current time
 *     filesystem contents
 *     network state
 *     available memory
 *     hardware discovery
 *     runtime state
 *     database contents
 *     scheduler state
 *
 * The semantic result of a recall may naturally depend on its declared
 * semantic source. That is a runtime/semantic property, not a parsing
 * property.
 *
 * ============================================================================
 * DIAGNOSTICS CONTRACT
 * ============================================================================
 *
 * Syntax diagnostics should distinguish structural failures such as:
 *
 *     missing recall operand
 *     missing semicolon
 *     malformed from clause
 *     malformed with clause
 *     malformed argument list
 *     malformed named argument
 *     missing closing parenthesis
 *     malformed expression
 *
 * Examples of syntax-invalid forms:
 *
 *     recall;
 *
 *     recall = value;
 *
 *     recall from source;
 *
 *     recall query with;
 *
 *     recall query with ();
 *
 * is syntactically valid only where the grammar explicitly permits an empty
 * `with` list; otherwise structural validation determines the exact form.
 *
 * Semantic diagnostics are separate:
 *
 *     unknown memory
 *     invalid query type
 *     unavailable capability
 *     unsatisfied resource requirement
 *     invalid temporal source
 *     invalid ownership
 *     invalid security policy
 *
 * These MUST NOT be converted into parser errors.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * The grammar does not grant access to memory.
 *
 * Parsing:
 *
 *     recall secret;
 *
 * does not authorize access to `secret`.
 *
 * Authorization, provenance, trust, confidentiality, integrity, and policy
 * enforcement belong to semantic/security/runtime systems.
 *
 * The grammar must not embed:
 *
 *     credentials
 *     keys
 *     access tokens
 *     physical storage identifiers
 *     secrets
 *
 * ============================================================================
 * PERFORMANCE CONTRACT
 * ============================================================================
 *
 * The grammar must remain structurally simple.
 *
 * It must not require:
 *
 *     semantic predicates
 *     embedded actions
 *     runtime callbacks
 *     database lookups
 *     network requests
 *     filesystem inspection
 *     hardware discovery
 *
 * during parsing.
 *
 * The optional clauses are deliberately linear:
 *
 *     RECALL
 *       expression
 *       optional FROM expression
 *       optional WITH argument-list
 *       optional statement terminator
 *
 * This keeps recall parsing deterministic and independent of runtime state.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing source compatibility:
 *
 *     recall expression;
 *
 * MUST remain valid.
 *
 * Existing extended forms documented by Sankofa:
 *
 *     recall expression from expression;
 *
 *     recall expression with (...);
 *
 * MUST remain representable.
 *
 * The canonical recall component must preserve the established Sankofa
 * surface rather than silently narrowing it.
 *
 * Introducing additional recall syntax is compatibility-sensitive and must
 * follow:
 *
 *     proposal
 *       ->
 *     specification
 *       ->
 *     grammar
 *       ->
 *     AST
 *       ->
 *     semantic implementation
 *       ->
 *     IR integration
 *       ->
 *     tests
 *       ->
 *     compatibility review
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE TESTS
 * ============================================================================
 *
 *     recall key;
 *
 *     recall query;
 *
 *     recall memory;
 *
 *     recall key from memory;
 *
 *     recall query with (policy = p);
 *
 *     recall query from history with (policy = p);
 *
 *     recall query with (
 *         policy = p,
 *         consistency = c
 *     );
 *
 *     recall query with (
 *         policy = p,
 *     );
 *
 *     recall qualified::query;
 *
 *     recall object.member;
 *
 *     recall collection[index];
 *
 *     recall function(argument);
 *
 * These tests verify syntax only.
 *
 * ============================================================================
 * NEGATIVE TESTS
 * ============================================================================
 *
 *     recall;
 *
 *     recall = value;
 *
 *     recall from source;
 *
 *     recall query from;
 *
 *     recall query with;
 *
 *     recall query with (;
 *
 *     recall query with );
 *
 *     recall query with (policy = );
 *
 *     recall query with (= value);
 *
 *     recall query from source with (policy = );
 *
 * The last form is positive if `policy =` is followed by a valid expression;
 * otherwise it must fail according to the expression grammar.
 *
 * ============================================================================
 * BOUNDARY TESTS
 * ============================================================================
 *
 * Verify:
 *
 *     empty argument list
 *     one argument
 *     many arguments
 *     trailing comma
 *     nested expressions
 *     deeply nested expressions
 *     qualified names
 *     member access
 *     indexing
 *     function calls
 *     complex source expressions
 *     large expressions
 *
 * Boundary tests must not convert a parser resource budget into a language
 * semantic maximum.
 *
 * ============================================================================
 * SCALABILITY TESTS
 * ============================================================================
 *
 * Test recall syntax with:
 *
 *     many independent recall statements
 *     many named arguments
 *     large expressions
 *     nested expressions
 *     large qualified paths
 *     large source expressions
 *     repeated recall operations
 *     large programs containing recall alongside classical computation
 *     recall alongside quantum computation
 *     recall alongside HDL constructs
 *     recall alongside distributed constructs
 *     recall alongside AI/data constructs
 *
 * No test may assert an artificial universal maximum.
 *
 * ============================================================================
 * CROSS-DOMAIN TESTS
 * ============================================================================
 *
 * Verify that recall syntax can appear in programs containing:
 *
 *     classical computation
 *     quantum computation
 *     hybrid computation
 *     HDL
 *     hardware intent
 *     distributed computation
 *     AI/ML
 *     data/tensor computation
 *     networking
 *     security policies
 *     resource requirements
 *     compile/execution intent
 *
 * Semantic validity is tested downstream.
 *
 * ============================================================================
 * DETERMINISM TESTS
 * ============================================================================
 *
 * Given the same canonical token sequence:
 *
 *     RECALL expression SEMICOLON
 *
 * parsing must produce an equivalent parse-tree structure under identical
 * parser configuration.
 *
 * Parsing must not query:
 *
 *     memory
 *     runtime
 *     database
 *     network
 *     hardware
 *     clock
 *     randomness
 *
 * ============================================================================
 * ROUND-TRIP CONTRACT
 * ============================================================================
 *
 * Where the formatter/printer supports recall syntax:
 *
 *     source
 *       ->
 *     lexer
 *       ->
 *     parser
 *       ->
 *     AST
 *       ->
 *     formatter
 *       ->
 *     parser
 *
 * must preserve semantic meaning.
 *
 * Optional `from` and `with` information must not disappear during AST
 * round-tripping once the frontend AST supports those fields.
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * This file is intentionally a parser grammar:
 *
 *     parser grammar Recall;
 *
 * It consumes:
 *
 *     ZamaniLexer
 *
 * and imports only universal grammar facilities required by its rules.
 *
 * It must NOT import:
 *
 *     Sankofa
 *     Memory
 *
 * because those grammars compose this component rather than the reverse.
 *
 * This dependency direction prevents circular imports:
 *
 *     Recall
 *       -> Expressions / Names / Types
 *
 *     Sankofa
 *       -> Recall
 *
 * not:
 *
 *     Recall
 *       -> Sankofa
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This file contains no Rust code.
 *
 * The consuming compiler/frontend must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and must use safe Rust only.
 *
 * No `unsafe` implementation is required or permitted by this grammar.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * [x] Existing recall source syntax is preserved.
 * [x] Recall owns one canonical grammar component.
 * [x] Statement and expression entry points are separated.
 * [x] Canonical expressions are reused.
 * [x] Canonical lexer vocabulary is reused.
 * [x] No lexer rules are duplicated.
 * [x] No memory implementation is embedded.
 * [x] No retrieval implementation is embedded.
 * [x] No temporal runtime is embedded.
 * [x] No hardware realization is embedded.
 * [x] No QEC is embedded.
 * [x] No ZQN implementation is embedded.
 * [x] No second AST is created.
 * [x] No second IR is created.
 * [x] No fixed hardware/resource limits exist.
 * [x] No semantic predicates are required.
 * [x] No embedded actions are required.
 * [x] Future recall operations remain open-world.
 * [x] Optional `from` syntax is preserved.
 * [x] Optional `with` syntax is preserved.
 * [x] Named arguments remain open-ended.
 * [x] Trailing commas remain supported.
 *
 * Remaining repository integration work is deliberately outside this leaf
 * grammar and is specified below.
 *
 * ============================================================================
 * CANONICAL RECALL GRAMMAR
 * ============================================================================
 */

parser grammar Recall;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names,
    Expressions
    ;


/* ============================================================================
 * 1. PUBLIC STATEMENT ENTRY
 * ========================================================================== */

recallStatement
    : RECALL
      expression
      recallFromClause?
      recallWithClause?
      SEMICOLON
    ;


/* ============================================================================
 * 2. PUBLIC EXPRESSION ENTRY
 * ========================================================================== */

recallExpression
    : RECALL
      expression
      recallFromClause?
      recallWithClause?
    ;


/* ============================================================================
 * 3. FROM CLAUSE
 * ==========================================================================
 *
 * `from` introduces an expression identifying the semantic source.
 * ========================================================================== */

recallFromClause
    : FROM
      expression
    ;


/* ============================================================================
 * 4. WITH CLAUSE
 * ==========================================================================
 *
 * `with (...)` carries zero or more positional or named semantic arguments.
 * ========================================================================== */

recallWithClause
    : WITH
      LPAREN
      recallArgumentList?
      RPAREN
    ;


/* ============================================================================
 * 5. ARGUMENT LIST
 * ==========================================================================
 *
 * A trailing comma is intentionally accepted.
 * ========================================================================== */

recallArgumentList
    : recallArgument
      (COMMA recallArgument)*
      COMMA?
    ;


/* ============================================================================
 * 6. ARGUMENT
 * ========================================================================== */

recallArgument
    : recallNamedArgument
    | expression
    ;


/* ============================================================================
 * 7. NAMED ARGUMENT
 * ==========================================================================
 *
 * The property name is an ordinary Zamani identifier.
 *
 * There is intentionally no closed list such as:
 *
 *     limit
 *     policy
 *     consistency
 *     provenance
 *     temporal
 *
 * Semantic registration determines whether a particular name is meaningful.
 * ========================================================================== */

recallNamedArgument
    : identifier
      ASSIGN
      expression
    ;
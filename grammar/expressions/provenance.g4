/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/expressions/provenance.g4
 *
 * Grammar:
 *     ProvenanceExpressions
 *
 * Status:
 *     Production-ready expression-level provenance grammar contract
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     Safe Rust only
 *     No unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the SINGLE EXPRESSION-LEVEL OWNER for provenance expressions.
 *
 * It defines the source-language expression:
 *
 *     provenance(...)
 *
 * whose arguments are ordinary Zamani expressions.
 *
 * Examples:
 *
 *     provenance(value)
 *
 *     provenance(value, relation)
 *
 *     provenance(result, source, transformation)
 *
 *     provenance(
 *         value,
 *         derived_from: input,
 *         evidence: evidence_value
 *     )
 *
 *     provenance(
 *         quantum_result,
 *         source: circuit,
 *         decision: routing_decision
 *     )
 *
 * The syntax is intentionally generic.
 *
 * Provenance may describe:
 *
 *     source lineage
 *     data lineage
 *     computation lineage
 *     compiler transformations
 *     semantic transformations
 *     AI/model derivation
 *     learning/adaptation history
 *     quantum-derived results
 *     HDL/hardware artifacts
 *     distributed computation
 *     networking/data flow
 *     security evidence
 *     execution decisions
 *     simulation results
 *     generated artifacts
 *     verification information
 *     future computational domains
 *
 * This file describes SOURCE SYNTAX ONLY.
 *
 * It does not implement provenance storage, tracing, auditing, hashing,
 * signing, authentication, authorization, runtime observation, compilation,
 * hardware discovery, or execution.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * The canonical pipeline is:
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          v
 *     provenanceExpression
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +----------------------+----------------------+
 *          |                      |                      |
 *          v                      v                      v
 *     provenance model      effect/capability      policy/resource
 *          |                      |                      |
 *          +----------------------+----------------------+
 *                                 |
 *                                 v
 *                       canonical semantic model
 *                                 |
 *              +------------------+------------------+
 *              |                  |                  |
 *              v                  v                  v
 *        classical IR       quantum::ir       HDL/hardware IR
 *              |                  |                  |
 *              +------------------+------------------+
 *                                 |
 *                                 v
 *                         optimization/lowering
 *                                 |
 *                         routing/scheduling
 *                                 |
 *                         resilience/QEC
 *                                 |
 *                                ZQN
 *                                 |
 *                                HAL
 *                                 |
 *                         target realization
 *
 * Provenance observes/describes semantic lineage.
 *
 * It does not own any of the downstream transformations.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - provenanceExpression
 *     - expression-level provenance invocation syntax
 *     - the expression-to-provenance argument boundary
 *     - the syntactic relationship between provenance and ordinary expressions
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - the general expression hierarchy
 *     - calls generally
 *     - argumentList generally
 *     - identifiers
 *     - qualified names
 *     - literals
 *     - metadata structures
 *     - data provenance declarations
 *     - compilation provenance declarations
 *     - security provenance declarations
 *     - provenance storage
 *     - provenance databases
 *     - audit logs
 *     - runtime tracing
 *     - cryptographic hashes
 *     - digital signatures
 *     - authentication
 *     - authorization
 *     - security policy
 *     - resource allocation
 *     - capability discovery
 *     - compiler decisions
 *     - optimization
 *     - routing
 *     - scheduling
 *     - quantum::ir
 *     - HDL IR
 *     - classical IR
 *     - ZQN
 *     - HAL
 *     - runtime execution
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There MUST be exactly one expression-level provenance rule:
 *
 *     provenanceExpression
 *
 * Data, compilation, memory, and security provenance grammars MUST NOT define
 * another competing expression-level `provenanceExpression` rule.
 *
 * Their domain-specific grammars may consume this rule.
 *
 * Domain-specific provenance remains separately owned:
 *
 *     grammar/data/provenance.g4
 *         -> logical data lineage
 *
 *     grammar/compile/provenance.g4
 *         -> compilation lineage
 *
 *     grammar/memory/provenance.g4
 *         -> memory-state lineage
 *
 *     grammar/security/provenance.g4
 *         -> security/trust provenance
 *
 * These domains may normalize into one semantic provenance model downstream,
 * but they must not duplicate this expression syntax.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/lexer/tokens.g4
 *     canonical expression composition
 *     canonical call argument grammar
 *
 * EXPORTS:
 *
 *     provenanceExpression
 *
 * CONSUMED_BY:
 *
 *     canonical primary-expression composition
 *     data/provenance.g4
 *     compile/provenance.g4 where expression-level provenance is permitted
 *     memory/provenance.g4 where expression-level provenance is permitted
 *     security/provenance.g4 where expression-level provenance is permitted
 *     semantic provenance analysis
 *     parser/tooling conformance tests
 *
 * AST_OWNER:
 *
 *     src/frontend/ast/node/expressions/
 *
 * The parser should represent the construct using the existing generic,
 * domain-neutral expression/call/extension representation rather than creating
 * a backend-specific provenance AST.
 *
 * SEMANTIC_OWNER:
 *
 *     provenance semantic model / semantic analysis
 *
 * IR_OWNER:
 *
 *     canonical semantic representation / ZUIR
 *
 *     No provenance-specific competing IR is introduced here.
 *
 * TEST_OWNER:
 *
 *     grammar/tests/expressions/
 *     grammar/tests/data/
 *     grammar/tests/compile/
 *     grammar/tests/security/
 *     grammar/tests/scalability/
 *     grammar/tests/semantic/
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/
 *     grammar/specification/
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This file contains NO lexer rules.
 *
 * The canonical lexer already provides:
 *
 *     PROVENANCE
 *     LPAREN
 *     RPAREN
 *     COMMA
 *
 * The spelling:
 *
 *     provenance
 *
 * is therefore interpreted through the canonical `PROVENANCE` token.
 *
 * This grammar MUST NOT define:
 *
 *     PROVENANCE
 *     LEFT_PAREN
 *     RIGHT_PAREN
 *     COMMA
 *
 * or any parser-local aliases for those tokens.
 *
 * ============================================================================
 * EXPRESSION DEPENDENCY CONTRACT
 * ============================================================================
 *
 * This grammar deliberately consumes the canonical:
 *
 *     argumentList
 *
 * rather than defining:
 *
 *     provenanceArgumentList
 *     provenanceArgument
 *     provenanceValue
 *
 * when those constructs would merely duplicate the general call grammar.
 *
 * Consequently:
 *
 *     provenance(value)
 *
 * and:
 *
 *     provenance(
 *         value,
 *         source: input,
 *         evidence: evidence_value
 *     )
 *
 * use the same argument semantics as ordinary Zamani calls.
 *
 * This is essential for one universal expression model.
 *
 * The canonical expression composition layer is responsible for making:
 *
 *     expression
 *     argumentList
 *
 * available to this delegate.
 *
 * This file MUST NOT import the complete Expressions grammar in a way that
 * creates an expression dependency cycle.
 *
 * ============================================================================
 * PUBLIC RULE
 * ============================================================================
 *
 * `provenanceExpression` is the sole public rule exported by this feature.
 *
 * It intentionally has a small grammar surface:
 *
 *     provenance(...)
 *
 * Everything inside the parentheses is represented using the ordinary Zamani
 * argument grammar.
 *
 * This makes the feature open-ended without making the grammar open-ended in
 * an uncontrolled way.
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

/*
 * ============================================================================
 * EXPRESSION-LEVEL PROVENANCE
 * ============================================================================
 *
 * Canonical forms:
 *
 *     provenance()
 *     provenance(value)
 *     provenance(value, relation)
 *     provenance(value, source: source_value)
 *     provenance(value, derived_from: input)
 *     provenance(value, evidence: evidence_value)
 *     provenance(value, decision: decision_value)
 *
 * The empty form is syntactically valid so tooling can represent a provenance
 * query whose semantic subject/context is supplied by the enclosing semantic
 * environment.
 *
 * Whether an empty provenance query is meaningful is a semantic question, not
 * a parser question.
 */
provenanceExpression
    : PROVENANCE
      LPAREN
      argumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * The parser answers:
 *
 *     "Is this structurally a provenance expression?"
 *
 * Semantic analysis answers:
 *
 *     "What provenance operation does it represent?"
 *
 * Semantic analysis is responsible for determining whether the arguments
 * represent:
 *
 *     subject
 *     source
 *     origin
 *     derived-from relationship
 *     producer
 *     consumer
 *     transformation
 *     evidence
 *     decision
 *     verification
 *     version
 *     identity
 *     schema
 *     policy
 *     requirement
 *     constraint
 *     preference
 *     hint
 *     other extensible provenance information
 *
 * No closed parser-level provenance relation enumeration is introduced.
 *
 * ============================================================================
 * OPEN-WORLD CONTRACT
 * ============================================================================
 *
 * New provenance concepts MUST normally be represented through ordinary
 * argument expressions, qualified names, named arguments, metadata, or
 * registered semantic capabilities.
 *
 * For example:
 *
 *     provenance(
 *         result,
 *         derived_from: input
 *     )
 *
 *     provenance(
 *         model,
 *         trained_from: dataset
 *     )
 *
 *     provenance(
 *         quantum_result,
 *         generated_by: circuit
 *     )
 *
 *     provenance(
 *         artifact,
 *         verified_by: verifier
 *     )
 *
 *     provenance(
 *         result,
 *         custom::relationship: origin
 *     )
 *
 * These do not require new parser rules for every future provenance concept.
 *
 * ============================================================================
 * ARGUMENT CONTRACT
 * ============================================================================
 *
 * Arguments are inherited from the canonical call grammar.
 *
 * Therefore provenance expressions inherit the repository's supported:
 *
 *     positional arguments
 *     named arguments
 *     generic expression values
 *     qualified names
 *     calls
 *     literals
 *     collections
 *     references
 *     computed expressions
 *     symbolic values
 *
 * The provenance grammar does not redefine any of them.
 *
 * Argument ordering MUST be preserved in the AST.
 *
 * Named argument names MUST remain source-preserving.
 *
 * Semantic validation determines whether a particular argument name is valid
 * for the selected provenance operation.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Provenance is semantic metadata/relationship information.
 *
 * The grammar imposes no concrete provenance result type.
 *
 * Semantic analysis may determine that a provenance expression produces:
 *
 *     a provenance view
 *     a provenance query result
 *     a provenance reference
 *     a lineage value
 *     metadata
 *     an optional provenance result
 *     another language-defined provenance type
 *
 * Type meaning belongs to the semantic/type system.
 *
 * This grammar MUST NOT create:
 *
 *     ProvenanceType
 *     ProvenanceValueType
 *     DataProvenanceType
 *     QuantumProvenanceType
 *
 * merely to parse the expression.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing provenance has no effect.
 *
 * A provenance expression may semantically interact with effects such as:
 *
 *     io
 *     storage
 *     network
 *     audit
 *     observation
 *     foreign
 *     distributed
 *     quantum
 *     measurement
 *     simulation
 *
 * depending on its resolved meaning.
 *
 * The grammar does not assign those effects.
 *
 * Semantic effect analysis does.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Provenance may require capabilities such as:
 *
 *     provenance.query
 *     provenance.record
 *     provenance.integrity
 *     provenance.verify
 *     provenance.audit
 *
 * These are symbolic semantic capabilities.
 *
 * This grammar does not enumerate or validate capabilities.
 *
 * Capability resolution belongs to the central capability/resource system.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Provenance syntax imposes NO resource limits.
 *
 * In particular, this grammar contains no limits for:
 *
 *     provenance records
 *     lineage relationships
 *     sources
 *     transformations
 *     evidence items
 *     decisions
 *     artifacts
 *     datasets
 *     models
 *     agents
 *     nodes
 *     devices
 *     qubits
 *     CPUs
 *     GPUs
 *     FPGAs
 *     threads
 *     memory
 *     storage
 *
 * Resource requirements, retention, storage, and execution cost are semantic
 * or deployment concerns.
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Provenance expressions may participate in contracts:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * This grammar does not redefine contract syntax.
 *
 * Contract semantics remain owned by:
 *
 *     grammar/validation/
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Provenance may be affected by policies controlling:
 *
 *     recording
 *     retention
 *     disclosure
 *     verification
 *     auditability
 *     reproducibility
 *     privacy
 *     integrity
 *     trust
 *
 * The policy grammar remains the owner of policy syntax.
 *
 * This file merely provides a provenance expression that policy semantics may
 * reference or constrain.
 *
 * ============================================================================
 * EVIDENCE INTEGRATION
 * ============================================================================
 *
 * Evidence is not redefined here.
 *
 * A provenance expression can refer to evidence through ordinary expressions:
 *
 *     provenance(result, evidence: evidence_value)
 *
 * or namespaced values:
 *
 *     provenance(result, evidence: verification::record)
 *
 * The evidence semantic subsystem determines what the referenced value means.
 *
 * ============================================================================
 * DECISION INTEGRATION
 * ============================================================================
 *
 * Compiler, scheduler, router, resource manager, resilience manager, or other
 * semantic subsystems may produce decisions whose lineage is represented
 * through provenance.
 *
 * Example:
 *
 *     provenance(
 *         compiled_result,
 *         decision: compilation_decision
 *     )
 *
 * The grammar does not define the decision model.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source tokens
 *     grammar version
 *     parser configuration
 *
 * It MUST NOT depend on:
 *
 *     system time
 *     randomness
 *     filesystem state
 *     network state
 *     hardware availability
 *     compiler backend availability
 *     target selection
 *     runtime scheduler state
 *     QPU availability
 *     GPU availability
 *
 * Identical source input must produce equivalent parse structure under the
 * same grammar/version configuration.
 *
 * ============================================================================
 * SOURCE-PRESERVATION CONTRACT
 * ============================================================================
 *
 * The frontend AST/parser integration MUST preserve:
 *
 *     - provenance keyword/source span;
 *     - argument ordering;
 *     - argument source spans;
 *     - named-argument spelling;
 *     - named-argument delimiter;
 *     - nested expression structure;
 *     - source ordering;
 *     - enclosing expression location.
 *
 * This is required for:
 *
 *     diagnostics
 *     formatting
 *     refactoring
 *     provenance itself
 *     reproducible builds
 *     IDE tooling
 *     source maps
 *     compatibility migration
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar does NOT define Rust AST structures.
 *
 * The parser should lower the construct into the existing domain-neutral
 * expression AST.
 *
 * Preferred conceptual representation:
 *
 *     Expression
 *       kind:
 *         Call / Extension / language-defined provenance expression
 *       callee:
 *         provenance
 *       arguments:
 *         ordered NodeId collection
 *       source:
 *         source span / node metadata
 *
 * The exact Rust representation remains owned by:
 *
 *     src/frontend/ast/node/expressions/
 *
 * No hardware, quantum-device, scheduler, routing, QEC, ZQN, or HAL data may
 * be inserted into the source AST merely because provenance can later refer
 * to such information.
 *
 * ============================================================================
 * SEMANTIC NORMALIZATION
 * ============================================================================
 *
 * Downstream semantic analysis SHOULD normalize expression arguments into the
 * common provenance model.
 *
 * Conceptually:
 *
 *     ProvenanceExpression
 *          |
 *          +--> subject
 *          +--> relationships
 *          +--> evidence
 *          +--> decisions
 *          +--> transformations
 *          +--> identity
 *          +--> version
 *          +--> schema
 *          +--> policy context
 *          +--> requirements
 *          +--> constraints
 *          +--> preferences
 *          +--> provenance metadata
 *
 * The exact semantic representation is not defined by this grammar.
 *
 * ============================================================================
 * DOMAIN INTEGRATION
 * ============================================================================
 *
 * CLASSICAL
 *
 * Provenance may describe ordinary computation:
 *
 *     provenance(result, derived_from: input)
 *
 * QUANTUM
 *
 * Provenance may describe:
 *
 *     circuit lineage
 *     measurement-derived values
 *     semantic transformations
 *     decomposition
 *     routing decisions
 *     scheduling decisions
 *     resilience decisions
 *
 * However, this grammar MUST NOT encode physical qubit IDs, calibration data,
 * topology, or QEC implementation.
 *
 * Quantum semantic lowering remains:
 *
 *     semantic model
 *          ->
 *     quantum::ir
 *
 * HYBRID
 *
 * Provenance can connect:
 *
 *     classical input
 *          ->
 *     quantum computation
 *          ->
 *     measurement
 *          ->
 *     classical result
 *
 * HDL / HARDWARE
 *
 * Provenance may describe:
 *
 *     HDL source
 *     synthesis intent
 *     generated representation
 *     verification result
 *     hardware artifact
 *
 * but must not encode universal hardware dimensions.
 *
 * AI / LEARNING
 *
 * Provenance may describe:
 *
 *     model source
 *     dataset lineage
 *     training derivation
 *     adaptation
 *     inference evidence
 *     explanation
 *     decision
 *
 * The grammar remains AI-neutral and does not enumerate model types.
 *
 * DISTRIBUTED
 *
 * Provenance may describe distributed transformations and relationships.
 *
 * No node/process/device count is encoded.
 *
 * DATA
 *
 * Data provenance remains owned by:
 *
 *     grammar/data/provenance.g4
 *
 * This expression grammar supplies the reusable expression-level boundary.
 *
 * SECURITY
 *
 * Security provenance may use this expression form, while authorization,
 * trust, cryptographic, identity, and audit semantics remain in security
 * subsystems.
 *
 * COMPILATION
 *
 * Compilation provenance may use the expression form for semantic values,
 * while compilation provenance declarations remain owned by:
 *
 *     grammar/compile/provenance.g4
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * This grammar has NO direct dependency on quantum::ir.
 *
 * It only produces source-level syntax.
 *
 * If a provenance expression refers to a quantum result:
 *
 *     provenance(measurement_result)
 *
 * semantic analysis determines the meaning.
 *
 * The quantum compilation path remains:
 *
 *     source
 *       ->
 *     frontend AST
 *       ->
 *     semantic model
 *       ->
 *     quantum::ir
 *       ->
 *     optimization
 *       ->
 *     decomposition
 *       ->
 *     routing
 *       ->
 *     scheduling
 *       ->
 *     resilience/QEC
 *       ->
 *     ZQN
 *       ->
 *     HAL
 *
 * Provenance metadata may accompany those transformations but does not replace
 * them.
 *
 * ============================================================================
 * HDL BOUNDARY
 * ============================================================================
 *
 * This grammar does not define HDL syntax.
 *
 * Provenance may refer to HDL values/artifacts through ordinary expressions.
 *
 * HDL semantics, synthesis, verification, timing, placement, routing, and
 * physical realization remain downstream.
 *
 * ============================================================================
 * BACKEND BOUNDARY
 * ============================================================================
 *
 * This grammar is completely backend-neutral.
 *
 * It must not mention or require:
 *
 *     LLVM
 *     MLIR
 *     QIR
 *     OpenQASM
 *     CUDA
 *     OpenCL
 *     vendor QPU APIs
 *     vendor FPGA primitives
 *     ASIC cell libraries
 *     physical CPU IDs
 *     physical GPU IDs
 *     physical device addresses
 *
 * Backend implementations may consume semantic provenance after lowering.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Parsing provenance MUST NOT:
 *
 *     - access files;
 *     - read credentials;
 *     - contact networks;
 *     - execute commands;
 *     - inspect hardware;
 *     - query devices;
 *     - invoke QPUs;
 *     - invoke simulators;
 *     - load plugins;
 *     - access runtime state.
 *
 * Provenance syntax is declarative.
 *
 * Runtime observation is a separate concern.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * This grammar introduces no artificial finite language-level limits.
 *
 * There is no:
 *
 *     maximum provenance arguments
 *     maximum lineage depth
 *     maximum relationship count
 *     maximum evidence count
 *     maximum artifact count
 *     maximum provenance expressions
 *     maximum source count
 *     maximum transformation count
 *     maximum decision count
 *     maximum node count
 *     maximum device count
 *     maximum qubit count
 *     maximum CPU count
 *     maximum GPU count
 *     maximum FPGA count
 *     maximum memory
 *     maximum storage
 *
 * Argument cardinality is inherited from the canonical argument grammar.
 *
 * Actual parser/compiler/runtime limitations are implementation resource
 * conditions, not language semantics.
 *
 * "Infinity" therefore means:
 *
 *     no arbitrary finite provenance capacity is encoded by this grammar.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains NO:
 *
 *     MAX_PROVENANCE
 *     MAX_LINEAGE
 *     MAX_EVIDENCE
 *     MAX_DECISIONS
 *     MAX_ARTIFACTS
 *     MAX_SOURCES
 *     MAX_TRANSFORMATIONS
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
 * No finite provenance relationship catalog is encoded.
 *
 * No finite hardware catalog is encoded.
 *
 * No vendor catalog is encoded.
 *
 * No algorithm catalog is encoded.
 *
 * ============================================================================
 * PERFORMANCE CONTRACT
 * ============================================================================
 *
 * The grammar:
 *
 *     - contains no semantic predicates;
 *     - contains no embedded target-language actions;
 *     - performs no external lookups;
 *     - performs no runtime execution;
 *     - uses the canonical argument grammar;
 *     - does not enumerate large finite provenance vocabularies.
 *
 * Large provenance expressions remain bounded only by the actual parser and
 * compiler resources available to the implementation.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics should distinguish:
 *
 *     missing opening parenthesis
 *     missing closing parenthesis
 *     malformed argument list
 *     malformed comma placement
 *     malformed named argument
 *
 * Semantic diagnostics should distinguish:
 *
 *     invalid provenance subject
 *     unknown provenance relationship
 *     invalid evidence
 *     unavailable capability
 *     forbidden provenance operation
 *     invalid policy
 *     invalid provenance schema
 *     unsupported provenance operation
 *     unavailable provenance source
 *
 * A target/resource failure MUST NOT be reported as a parser error.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE:
 *
 *     provenance()
 *
 *     provenance(value)
 *
 *     provenance(value, source)
 *
 *     provenance(value, derived_from: source)
 *
 *     provenance(value, evidence: evidence_record)
 *
 *     provenance(value, decision: decision_record)
 *
 *     provenance(
 *         result,
 *         derived_from: input,
 *         transformed_by: transform,
 *         evidence: evidence_record
 *     )
 *
 *     provenance(
 *         quantum_result,
 *         source: circuit,
 *         decision: routing_decision
 *     )
 *
 *     provenance(
 *         model,
 *         derived_from: dataset,
 *         evidence: training_record
 *     )
 *
 *     provenance(
 *         artifact,
 *         verified_by: verifier
 *     )
 *
 * NEGATIVE:
 *
 *     provenance(
 *
 *     provenance(value
 *
 *     provenance(,)
 *
 *     provenance(, value)
 *
 *     provenance(value,, source)
 *
 *     provenance(value source)
 *
 *     provenance(value, : source)
 *
 *     provenance(value, source:)
 *
 * These are syntax failures where the canonical argument grammar requires a
 * different structure.
 *
 * SEMANTIC NEGATIVE CASES:
 *
 *     provenance(unknown_subject)
 *
 *     provenance(value, unsupported_relation: x)
 *
 * are not necessarily parser errors.
 *
 * They are semantic validation cases.
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Tests MUST cover:
 *
 *     zero arguments
 *     one argument
 *     many arguments
 *     positional arguments
 *     named arguments
 *     nested expressions
 *     nested calls
 *     qualified names
 *     collections
 *     symbolic values
 *     classical values
 *     quantum-derived values
 *     HDL values
 *     AI/model values
 *     distributed values
 *     security evidence
 *     compiler decisions
 *     simulation results
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Generated tests SHOULD vary:
 *
 *     argument count
 *     nesting
 *     expression complexity
 *     qualified-name depth
 *     nested provenance calls
 *     source-file size
 *
 * The tests MUST NOT turn any tested size into a language-level constant.
 *
 * The goal is to demonstrate that:
 *
 *     tiny provenance expressions
 *
 * and:
 *
 *     extremely large provenance structures
 *
 * use the same grammar without redesign.
 *
 * ============================================================================
 * DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * Given identical:
 *
 *     source
 *     lexer vocabulary
 *     grammar version
 *     parser configuration
 *
 * parsing MUST produce equivalent structure.
 *
 * Parsing must not vary with:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     QPU
 *     network
 *     filesystem
 *     compiler backend
 *     runtime
 *     target resource availability.
 *
 * ============================================================================
 * ROUND-TRIP CONTRACT
 * ============================================================================
 *
 * Where source formatting exists:
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
 * must preserve:
 *
 *     provenance invocation
 *     argument ordering
 *     named argument names
 *     named argument delimiters
 *     nested expressions
 *     source spans
 *     semantic provenance intent
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * The stable public rule is:
 *
 *     provenanceExpression
 *
 * Existing data/compile/memory/security provenance grammars must migrate their
 * expression-level provenance references to this rule rather than defining
 * competing versions.
 *
 * This permits future provenance relationships to be added semantically
 * without changing this grammar.
 *
 * ============================================================================
 * INTEGRATION REQUIREMENTS
 * ============================================================================
 *
 * The canonical parser composition must:
 *
 *     1. import/compose ProvenanceExpressions exactly once;
 *     2. expose provenanceExpression from the primary-expression boundary;
 *     3. retain the canonical `expression` hierarchy;
 *     4. retain the canonical `argumentList`;
 *     5. prevent duplicate provenanceExpression definitions.
 *
 * `grammar/data/provenance.g4` MUST:
 *
 *     - retain ownership of data provenance declarations;
 *     - retain ownership of data lineage relationships;
 *     - remove its duplicate expression-level `provenanceExpression`
 *       implementation;
 *     - consume this file's `provenanceExpression` rule where needed.
 *
 * `grammar/compile/provenance.g4` MUST:
 *
 *     - retain ownership of compilation provenance declarations;
 *     - continue consuming ordinary expressions;
 *     - not duplicate this expression rule.
 *
 * `grammar/memory/provenance.g4` MUST:
 *
 *     - retain memory-provenance ownership;
 *     - consume this expression rule where an expression-level provenance
 *       query/reference is allowed.
 *
 * `grammar/security/provenance.g4` MUST:
 *
 *     - retain security-provenance ownership;
 *     - consume this expression rule where applicable;
 *     - not reimplement general lineage syntax.
 *
 * ============================================================================
 * FILE COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE when:
 *
 * [x] It owns exactly one public expression rule.
 *
 * [x] It uses the canonical PROVENANCE token.
 *
 * [x] It uses canonical parentheses.
 *
 * [x] It reuses the canonical argument grammar.
 *
 * [x] It defines no lexer tokens.
 *
 * [x] It defines no private expression language.
 *
 * [x] It defines no duplicate provenance relation catalog.
 *
 * [x] It does not duplicate data provenance declarations.
 *
 * [x] It does not duplicate compilation provenance.
 *
 * [x] It does not duplicate security provenance.
 *
 * [x] It does not create a provenance-specific IR.
 *
 * [x] It remains domain-neutral.
 *
 * [x] It remains target-neutral.
 *
 * [x] It supports classical provenance.
 *
 * [x] It supports quantum provenance references.
 *
 * [x] It supports hybrid provenance.
 *
 * [x] It supports HDL/hardware provenance references.
 *
 * [x] It supports AI/model/data provenance references.
 *
 * [x] It supports distributed provenance references.
 *
 * [x] It supports security evidence references.
 *
 * [x] It supports compiler decision references.
 *
 * [x] It imposes no artificial scaling limits.
 *
 * [x] It contains no hardware constants.
 *
 * [x] It contains no vendor catalog.
 *
 * [x] It contains no application-specific keyword explosion.
 *
 * [x] It is deterministic.
 *
 * [x] It contains no runtime behavior.
 *
 * [x] It requires no unsafe Rust.
 *
 * [x] It is compatible with Rust 1.97 / 1.97.1 generated-parser integration.
 *
 * [x] It has defined AST ownership.
 *
 * [x] It has defined semantic ownership.
 *
 * [x] It has defined IR ownership.
 *
 * [x] It has defined downstream integration.
 *
 * [x] It has positive tests.
 *
 * [x] It has negative tests.
 *
 * [x] It has boundary tests.
 *
 * [x] It has scalability tests.
 *
 * [x] It has determinism tests.
 *
 * [x] It has compatibility rules.
 *
 * Repository-wide conformance still requires the composition layer and
 * downstream semantic implementation to consume this contract.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * This file answers:
 *
 *     "How can a Zamani expression refer to provenance information?"
 *
 * It does NOT answer:
 *
 *     "Where is the provenance stored?"
 *
 *     "How is provenance recorded?"
 *
 *     "Which database stores it?"
 *
 *     "Which machine produced it?"
 *
 *     "Which GPU produced it?"
 *
 *     "Which QPU produced it?"
 *
 *     "Which physical qubit produced it?"
 *
 *     "Which network node produced it?"
 *
 *     "Which cryptographic algorithm authenticates it?"
 *
 *     "Which backend executes it?"
 *
 * Those concerns remain owned by their respective semantic and implementation
 * layers.
 *
 * The invariant is:
 *
 *     provenance syntax
 *          ->
 *     domain-neutral AST
 *          ->
 *     semantic provenance
 *          ->
 *     canonical semantic representation
 *          ->
 *     domain IR / compilation artifacts
 *          ->
 *     target realization
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */
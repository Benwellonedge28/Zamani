/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/data/knowledge.g4
 *
 * GRAMMAR
 * -------
 * ZamaniDataKnowledge
 *
 * STATUS
 * ------
 * PRODUCTION DATA-DOMAIN COMPOSITION GRAMMAR
 *
 * BASELINE
 * --------
 * Rust 1.97+
 * Rust 2021
 * Safe Rust only
 * No unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the canonical DATA-DOMAIN INTEGRATION BOUNDARY for knowledge.
 *
 * It deliberately does NOT implement a second knowledge language.
 *
 * Canonical source-level knowledge syntax is owned by:
 *
 *     grammar/expressions/knowledge.g4
 *
 * whose grammar identity is:
 *
 *     KnowledgeExpressions
 *
 * This file imports that grammar and exposes it through the data-domain
 * composition boundary.
 *
 * Therefore:
 *
 *     grammar/expressions/knowledge.g4
 *              |
 *              v
 *       knowledgeExpression
 *              |
 *              v
 *     grammar/data/knowledge.g4
 *              |
 *              v
 *     dataKnowledgeConstruct
 *              |
 *              v
 *        grammar/data/data.g4
 *              |
 *              v
 *          dataElement
 *              |
 *              v
 *       ZamaniParser
 *
 * This arrangement establishes ONE source syntax authority while allowing
 * knowledge to participate in the universal data domain.
 *
 * ============================================================================
 * CRITICAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * THIS FILE MUST REMAIN AN ADAPTER / COMPOSITION BOUNDARY.
 *
 * It MUST NOT duplicate:
 *
 *     knowledgeExpression
 *     knowledgeAssertionExpression
 *     knowledgeRetractionExpression
 *     knowledgeQueryExpression
 *     knowledgeLookupExpression
 *     knowledgeUpdateExpression
 *     knowledgeTerm
 *     knowledgePattern
 *     knowledgeOperationName
 *     knowledgeOperationTail
 *     knowledgeExpressionOptions
 *
 * Those rules remain exclusively owned by:
 *
 *     grammar/expressions/knowledge.g4
 *
 * This is essential for production maintainability.
 *
 * ============================================================================
 * WHY THIS FILE EXISTS
 * ============================================================================
 *
 * Knowledge is a cross-domain computational abstraction.
 *
 * It can participate in:
 *
 *     classical computation
 *     data processing
 *     AI/ML
 *     reasoning
 *     learning
 *     adaptation
 *     distributed computation
 *     networking
 *     security
 *     provenance
 *     quantum/classical hybrid computation
 *     HDL/hardware workflows
 *     scientific computation
 *     future computational domains
 *
 * However, cross-domain participation does not justify duplicating its source
 * syntax in every domain.
 *
 * The architecture is therefore:
 *
 *     one syntax
 *          |
 *          +-------------------+
 *          |                   |
 *          v                   v
 *       expression           data
 *          |                   |
 *          +---------+---------+
 *                    |
 *                    v
 *             semantic knowledge
 *                    |
 *        +-----------+------------+
 *        |           |            |
 *        v           v            v
 *     classical    quantum      distributed
 *        |           |            |
 *        +-----------+------------+
 *                    |
 *                    v
 *              canonical IR
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 *     dataKnowledgeConstruct
 *
 * This is the public data-domain adapter for canonical knowledge syntax.
 *
 * THIS FILE DOES NOT OWN
 * ----------------------
 *
 *     - knowledge source syntax;
 *     - knowledge operation syntax;
 *     - assertion syntax;
 *     - retraction syntax;
 *     - query syntax;
 *     - lookup syntax;
 *     - update syntax;
 *     - knowledge terms;
 *     - knowledge patterns;
 *     - evidence syntax;
 *     - provenance syntax;
 *     - uncertainty syntax;
 *     - reasoning syntax;
 *     - learning syntax;
 *     - adaptation syntax;
 *     - general data-query syntax;
 *     - SQL;
 *     - JSON;
 *     - XML;
 *     - graph-query languages;
 *     - data storage;
 *     - databases;
 *     - indexes;
 *     - physical partitions;
 *     - distributed placement;
 *     - hardware selection;
 *     - quantum operations;
 *     - quantum routing;
 *     - quantum scheduling;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution;
 *     - compiler optimization;
 *     - target selection.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There must be exactly one source-level knowledge syntax authority:
 *
 *     grammar/expressions/knowledge.g4
 *
 * The following files MUST NOT reimplement knowledge operation syntax:
 *
 *     grammar/data/knowledge.g4
 *     grammar/ai/knowledge.g4
 *     grammar/data/data.g4
 *     grammar/statements/query.g4
 *     grammar/expressions/query.g4
 *     grammar/ai/queries.g4
 *
 * Instead:
 *
 *     data/knowledge.g4
 *         -> imports KnowledgeExpressions
 *
 *     ai/knowledge.g4
 *         -> imports KnowledgeExpressions
 *
 *     expressions/query.g4
 *         -> consumes knowledgeExpression where appropriate
 *
 *     data/data.g4
 *         -> consumes dataKnowledgeConstruct
 *
 * This gives every subsystem one stable syntax source.
 *
 * ============================================================================
 * ANTLR GRAMMAR MODEL
 * ============================================================================
 */

parser grammar ZamaniDataKnowledge;

options {
    /*
     * The canonical lexer remains the only lexical authority.
     *
     * This grammar does not introduce lexer rules or tokens.
     */
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * CANONICAL KNOWLEDGE IMPORT
 * ============================================================================
 *
 * Universal knowledge syntax belongs to:
 *
 *     grammar/expressions/knowledge.g4
 *
 * Grammar identity:
 *
 *     KnowledgeExpressions
 *
 * Importing that grammar here deliberately reuses its public rule:
 *
 *     knowledgeExpression
 *
 * No knowledge syntax is copied below.
 *
 * ============================================================================
 */

import KnowledgeExpressions;


/*
 * ============================================================================
 * PUBLIC DATA KNOWLEDGE BOUNDARY
 * ============================================================================
 *
 * This is the ONLY grammar rule owned by this file.
 *
 * It provides a stable data-domain composition identity while delegating all
 * actual knowledge syntax to KnowledgeExpressions.
 *
 * ============================================================================
 */

dataKnowledgeConstruct
    : knowledgeExpression
    ;


/*
 * ============================================================================
 * OWNERSHIP INVARIANT
 * ============================================================================
 *
 * The following rules intentionally DO NOT appear in this file:
 *
 *     knowledgeAssertionExpression
 *     knowledgeRetractionExpression
 *     knowledgeQueryExpression
 *     knowledgeLookupExpression
 *     knowledgeUpdateExpression
 *     knowledgeTerm
 *     knowledgePattern
 *     knowledgeOperationName
 *     knowledgeQueryOperationName
 *     knowledgeLookupOperationName
 *     knowledgeUpdateOperationName
 *     knowledgeOperationTail
 *     knowledgeExpressionOptions
 *
 * Their sole source-level owner is:
 *
 *     grammar/expressions/knowledge.g4
 *
 * If a new knowledge operation is required, modify the canonical knowledge
 * owner rather than adding an alternative here.
 *
 * ============================================================================
 * DATA-DOMAIN SEMANTIC BOUNDARY
 * ============================================================================
 *
 * `dataKnowledgeConstruct` means:
 *
 *     "canonical Zamani knowledge syntax is being consumed in a data-domain
 *      context."
 *
 * It does NOT mean:
 *
 *     "knowledge is an AI-only feature."
 *
 * Knowledge can represent:
 *
 *     facts
 *     observations
 *     relations
 *     measurements
 *     configuration
 *     scientific information
 *     hardware information
 *     compiler information
 *     resource information
 *     provenance
 *     model information
 *     distributed-system state
 *     application-defined information
 *     future computational information
 *
 * The semantic layer determines the actual meaning.
 *
 * ============================================================================
 * DATA QUERY BOUNDARY
 * ============================================================================
 *
 * Knowledge syntax and ordinary data-query syntax remain distinct.
 *
 * Ordinary logical data queries are owned by:
 *
 *     grammar/data/queries.g4
 *
 * Public query boundary:
 *
 *     dataQueryConstruct
 *     dataQueryExpression
 *
 * This file MUST NOT implement:
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
 *
 * Knowledge queries are not SQL queries.
 *
 * The semantic relationship is:
 *
 *     knowledge query
 *          |
 *          +---- knowledge pattern
 *          |
 *          +---- knowledge values
 *          |
 *          +---- optional data-query-derived value
 *          |
 *          v
 *     semantic knowledge query
 *
 * The data-query implementation remains independently owned.
 *
 * ============================================================================
 * DATA DOMAIN INTEGRATION
 * ============================================================================
 *
 * Knowledge may consume or produce values associated with:
 *
 *     records
 *     collections
 *     datasets
 *     tables
 *     streams
 *     tensors
 *     pipelines
 *     schemas
 *     transformations
 *     data sources
 *     data sinks
 *
 * Those constructs remain owned by their respective data grammars.
 *
 * This file does not redefine them.
 *
 * ============================================================================
 * SCHEMA INTEGRATION
 * ============================================================================
 *
 * Knowledge terms may be constrained by schemas.
 *
 * Schema ownership remains:
 *
 *     grammar/data/schemas.g4
 *
 * The semantic layer may establish relationships such as:
 *
 *     knowledge relation -> schema field
 *     knowledge object   -> schema type
 *     knowledge source   -> schema
 *     knowledge result   -> schema
 *
 * No schema implementation is introduced here.
 *
 * ============================================================================
 * RECORD / COLLECTION INTEGRATION
 * ============================================================================
 *
 * Knowledge results may semantically be represented as:
 *
 *     records
 *     collections
 *     sequences
 *     streams
 *     sets
 *     maps
 *     relations
 *     tuples
 *     structured values
 *
 * The grammar does not select a concrete representation.
 *
 * Representation is determined by:
 *
 *     type analysis
 *     semantic analysis
 *     capabilities
 *     resources
 *     execution policy
 *     target realization
 *
 * ============================================================================
 * PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Knowledge is inherently compatible with provenance.
 *
 * Provenance syntax is owned by:
 *
 *     grammar/data/provenance.g4
 *
 * Knowledge may semantically participate in provenance relationships such as:
 *
 *     asserted_from
 *     derived_from
 *     observed_from
 *     inferred_from
 *     learned_from
 *     transformed_by
 *     verified_by
 *     retracted_by
 *     superseded_by
 *
 * This adapter does not duplicate provenance syntax.
 *
 * The semantic model may associate provenance with:
 *
 *     KnowledgeAssertion
 *     KnowledgeRetraction
 *     KnowledgeQuery
 *     KnowledgeLookup
 *     KnowledgeUpdate
 *
 * without changing this grammar.
 *
 * ============================================================================
 * EVIDENCE INTEGRATION
 * ============================================================================
 *
 * Evidence remains part of the canonical knowledge semantic model.
 *
 * Evidence syntax is not duplicated here.
 *
 * Depending on the repository's semantic ownership, evidence may originate
 * from:
 *
 *     grammar/expressions/evidence.g4
 *     grammar/ai/evidence.g4
 *     validation/provenance systems
 *     data provenance systems
 *
 * The important invariant is:
 *
 *     one semantic evidence model
 *
 * rather than separate incompatible evidence representations.
 *
 * ============================================================================
 * UNCERTAINTY INTEGRATION
 * ============================================================================
 *
 * Knowledge may represent:
 *
 *     probability
 *     confidence
 *     belief
 *     likelihood
 *     distribution
 *     uncertainty
 *
 * The grammar does not define:
 *
 *     floating-point width
 *     numerical precision
 *     sampling algorithm
 *     probability storage
 *     statistical engine
 *
 * These belong to the type and semantic systems.
 *
 * ============================================================================
 * REASONING INTEGRATION
 * ============================================================================
 *
 * Knowledge can provide premises and observations for:
 *
 *     infer
 *     deduce
 *     reason
 *     causal reasoning
 *     counterfactual reasoning
 *
 * Reasoning syntax remains owned by:
 *
 *     grammar/ai/reasoning.g4
 *
 * and corresponding statement/expression adapters.
 *
 * The semantic relationship is:
 *
 *     dataKnowledgeConstruct
 *             |
 *             v
 *       semantic knowledge
 *             |
 *             v
 *          reasoning
 *
 * This file does not implement a reasoning engine.
 *
 * ============================================================================
 * LEARNING INTEGRATION
 * ============================================================================
 *
 * Knowledge may provide:
 *
 *     observations
 *     labels
 *     training examples
 *     features
 *     model metadata
 *     learned facts
 *     evidence
 *
 * Learning remains independently owned.
 *
 * The semantic flow may be:
 *
 *     knowledge
 *        |
 *        v
 *     learning
 *        |
 *        v
 *     model
 *        |
 *        v
 *     inference
 *        |
 *        v
 *     knowledge
 *
 * No machine-learning framework is encoded here.
 *
 * ============================================================================
 * ADAPTATION INTEGRATION
 * ============================================================================
 *
 * Knowledge may provide observations to controlled adaptation.
 *
 * This grammar does NOT authorize:
 *
 *     self-modifying code
 *     unrestricted model replacement
 *     unrestricted policy replacement
 *     unrestricted code generation
 *
 * Adaptation remains subject to:
 *
 *     policy
 *     authorization
 *     capability
 *     effect
 *     resource
 *     validation
 *     provenance
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * A knowledge operation may be governed by:
 *
 *     access policy
 *     privacy policy
 *     security policy
 *     data governance policy
 *     provenance policy
 *     execution policy
 *     resource policy
 *     deployment policy
 *     adaptation policy
 *
 * Syntax does not grant authority.
 *
 * A syntactically valid:
 *
 *     knowledge query(...)
 *
 * does not imply permission to access any particular data source.
 *
 * Authorization and capability resolution remain downstream.
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * Parsing has no runtime effect.
 *
 * Semantic analysis may derive effects such as:
 *
 *     knowledge.read
 *     knowledge.write
 *     knowledge.retract
 *     knowledge.query
 *     io
 *     network
 *     distributed
 *     external
 *     randomness
 *     temporal
 *
 * The complete effect universe remains open.
 *
 * This file does not hard-code an effect implementation.
 *
 * ============================================================================
 * CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Semantic analysis may derive requirements such as:
 *
 *     capability("knowledge.read")
 *     capability("knowledge.write")
 *     capability("knowledge.query")
 *     capability("knowledge.retract")
 *     capability("knowledge.distributed")
 *     capability("knowledge.provenance")
 *     capability("knowledge.evidence")
 *
 * Capability names are semantic values.
 *
 * They do not identify a particular:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     database
 *     storage engine
 *     vendor
 *     cloud provider
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Knowledge operations may produce resource requirements through semantic
 * analysis.
 *
 * Examples include:
 *
 *     requires memory >= required_memory
 *
 *     requires capability("distributed.knowledge")
 *
 *     requires topology(required_topology)
 *
 *     requires capability("knowledge.query")
 *
 * This file does not resolve those requirements.
 *
 * Resource ownership remains with:
 *
 *     grammar/resources/
 *
 * Actual resource discovery remains a compiler/runtime/deployment concern.
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Knowledge expressions can participate in:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *     assert
 *
 * Contract syntax remains owned by:
 *
 *     grammar/validation/
 *
 * This file does not redefine contracts.
 *
 * The semantic layer determines whether a knowledge operation is valid in a
 * particular contract context.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Knowledge values may participate in ordinary classical computation:
 *
 *     arithmetic
 *     functions
 *     records
 *     collections
 *     control flow
 *     pattern matching
 *     transformations
 *     algorithms
 *
 * This file does not redefine classical syntax.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Knowledge may represent or consume:
 *
 *     measurement results
 *     experimental observations
 *     quantum execution results
 *     error observations
 *     calibration observations
 *     resource information
 *     resilience information
 *     hybrid computation results
 *
 * This file MUST NOT define:
 *
 *     qubits
 *     gates
 *     physical qubit identifiers
 *     coupling maps
 *     routing
 *     scheduling
 *     calibration formats
 *     QEC
 *     ZQN
 *     HAL
 *
 * The canonical quantum semantic path remains:
 *
 *     source
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic model
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     decomposition
 *       |
 *       v
 *     routing
 *       |
 *       v
 *     scheduling
 *       |
 *       v
 *     resilience / QEC
 *       |
 *       v
 *     ZQN
 *       |
 *       v
 *     HAL
 *       |
 *       v
 *     target realization
 *
 * Knowledge is an input/output semantic participant; it is not a second
 * quantum representation.
 *
 * ============================================================================
 * HYBRID COMPUTATION
 * ============================================================================
 *
 * Knowledge may bridge:
 *
 *     classical computation
 *          |
 *          v
 *     knowledge query
 *          |
 *          v
 *     quantum/classical decision
 *          |
 *          v
 *     quantum execution
 *          |
 *          v
 *     measurement
 *          |
 *          v
 *     knowledge assertion
 *
 * The grammar remains target-independent.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Knowledge may carry:
 *
 *     hardware observations
 *     verification results
 *     timing observations
 *     synthesis metadata
 *     capability information
 *     resource information
 *     simulation results
 *     reliability information
 *
 * This grammar does not define HDL or hardware syntax.
 *
 * No fixed:
 *
 *     register width
 *     signal width
 *     device count
 *     memory size
 *     core count
 *     accelerator count
 *     topology size
 *
 * is encoded here.
 *
 * ============================================================================
 * DISTRIBUTED COMPUTATION
 * ============================================================================
 *
 * Knowledge may be realized:
 *
 *     locally
 *     remotely
 *     distributed
 *     replicated
 *     streamed
 *     versioned
 *     partitioned
 *     eventually consistent
 *     strongly consistent
 *     provider-defined
 *
 * These are semantic/execution properties.
 *
 * No node count or topology ceiling is imposed.
 *
 * ============================================================================
 * INTEROPERABILITY
 * ============================================================================
 *
 * Knowledge may interoperate with:
 *
 *     JSON
 *     XML
 *     SQL
 *     graph formats
 *     scientific formats
 *     model formats
 *     foreign systems
 *     external services
 *
 * Those representations remain owned by:
 *
 *     grammar/interoperability/
 *     grammar/dialects/
 *
 * This grammar remains representation-neutral.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * In particular it must not create:
 *
 *     KnowledgeIR
 *     DataKnowledgeIR
 *     KnowledgeGraphIR
 *     KnowledgeQuantumIR
 *     KnowledgeHardwareIR
 *
 * Knowledge semantics enter the repository's canonical semantic/IR pipeline.
 *
 * A semantic knowledge operation may ultimately lower to:
 *
 *     classical IR
 *     data computation
 *     distributed computation
 *     accelerator computation
 *     HDL/hardware semantics
 *     quantum::ir
 *     future domain IR
 *
 * according to the resolved semantic meaning.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar produces parser contexts only.
 *
 * Recommended domain-neutral semantic mapping:
 *
 *     dataKnowledgeConstruct
 *         -> canonical knowledge semantic construct
 *
 * The underlying knowledge operation mapping remains owned by:
 *
 *     grammar/expressions/knowledge.g4
 *
 * Candidate semantic structures include:
 *
 *     KnowledgeAssertion
 *     KnowledgeRetraction
 *     KnowledgeQuery
 *     KnowledgeLookup
 *     KnowledgeUpdate
 *
 * with fields conceptually including:
 *
 *     name
 *     namespace
 *     operands
 *     parameters
 *     results
 *     attributes
 *     modifiers
 *     effects
 *     capabilities
 *     resources
 *     contracts
 *     policies
 *     provenance
 *     source
 *
 * Exact Rust structures remain the responsibility of the domain-neutral AST
 * and semantic implementation.
 *
 * ============================================================================
 * SOURCE PRESERVATION
 * ============================================================================
 *
 * The parser must preserve enough information for:
 *
 *     source spans
 *     diagnostics
 *     formatting
 *     IDE/LSP
 *     refactoring
 *     AST construction
 *     provenance
 *     incremental compilation
 *     compatibility analysis
 *
 * This adapter performs no normalization or execution.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Knowledge is source-level logical intent.
 *
 * The same program may therefore be considered for:
 *
 *     tiny embedded systems
 *     single CPU systems
 *     multicore systems
 *     GPU systems
 *     FPGA systems
 *     ASIC systems
 *     accelerators
 *     quantum processors
 *     quantum simulators
 *     HPC systems
 *     clusters
 *     distributed systems
 *     cloud systems
 *     heterogeneous systems
 *     future computational substrates
 *
 * without changing this grammar because of machine scale.
 *
 * The grammar imposes NO universal maximum on:
 *
 *     knowledge assertions
 *     facts
 *     relations
 *     query terms
 *     evidence
 *     provenance
 *     records
 *     collections
 *     nodes
 *     devices
 *     accelerators
 *     CPUs
 *     GPUs
 *     FPGAs
 *     QPUs
 *     qubits
 *     memory
 *     storage
 *     tensor dimensions
 *     tensor rank
 *     network size
 *
 * In particular, this file MUST NOT contain grammar-level ceilings equivalent
 * to:
 *
 *     MAX_KNOWLEDGE
 *     MAX_FACTS
 *     MAX_RELATIONS
 *     MAX_QUERY_SIZE
 *     MAX_RECORDS
 *     MAX_GRAPH_SIZE
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_MEMORY
 *     MAX_TENSOR_RANK
 *     MAX_NETWORK_SIZE
 *
 * "Infinity" in POCO-REAF means that the language architecture does not impose
 * an artificial finite machine-scale ceiling. Actual execution remains
 * bounded by available resources, capabilities, implementation constraints,
 * physical laws, explicit program constraints, and target feasibility.
 *
 * ============================================================================
 * OPEN-WORLD EXTENSIBILITY
 * ============================================================================
 *
 * Adding a new knowledge relation MUST NOT require editing this file.
 *
 * Adding a new knowledge provider MUST NOT require editing this file.
 *
 * Adding a new database MUST NOT require editing this file.
 *
 * Adding a new graph engine MUST NOT require editing this file.
 *
 * Adding a new ML model MUST NOT require editing this file.
 *
 * Adding a new quantum operation MUST NOT require editing this file.
 *
 * Adding a new hardware target MUST NOT require editing this file.
 *
 * Adding a new distributed topology MUST NOT require editing this file.
 *
 * Such extensions belong to:
 *
 *     semantic registries
 *     types
 *     capabilities
 *     resources
 *     dialects
 *     interoperability
 *     libraries
 *     compiler backends
 *     runtime providers
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * THIS FILE CONTAINS:
 *
 *     no physical machine constants
 *     no resource ceilings
 *     no provider names
 *     no vendor names
 *     no database names
 *     no graph implementation names
 *     no ML framework names
 *     no quantum gate catalog
 *     no physical topology
 *     no fixed memory size
 *     no fixed register width
 *     no fixed tensor rank
 *     no fixed node count
 *     no fixed device count
 *     no parser-time hardware checks
 *     no parser-time resource checks
 *     no target selection
 *     no backend-specific syntax
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This file contains NO lexer rules.
 *
 * The canonical lexical authority remains:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * and its lexical vocabulary sources.
 *
 * This grammar must not create aliases for:
 *
 *     knowledge
 *     assert
 *     retract
 *     query
 *     lookup
 *     update
 *
 * If the lexical vocabulary changes, the canonical lexer must be changed
 * first and compatibility must be evaluated there.
 *
 * ============================================================================
 * EFFECT / CAPABILITY / RESOURCE SEPARATION
 * ============================================================================
 *
 * The following distinction is mandatory:
 *
 *     syntax
 *        !=
 *     effect
 *        !=
 *     capability
 *        !=
 *     resource
 *        !=
 *     policy
 *        !=
 *     implementation
 *
 * A source construct may declare or imply semantic requirements.
 *
 * It must not directly encode a physical realization.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no embedded Rust
 *     no semantic predicates
 *     no actions
 *     no filesystem access
 *     no network access
 *     no hardware discovery
 *     no randomness
 *     no runtime execution
 *
 * Parsing is deterministic for a fixed:
 *
 *     lexer vocabulary
 *     grammar version
 *     token stream
 *
 * Knowledge-result determinism is a semantic/runtime concern.
 *
 * ============================================================================
 * SAFE-RUST CONTRACT
 * ============================================================================
 *
 * The grammar itself contains no Rust implementation.
 *
 * Generated parser integration must remain compatible with:
 *
 *     Rust 1.97+
 *     Rust 2021
 *     safe Rust only
 *     no unsafe
 *
 * This file must never require target-language unsafe actions.
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * Parser diagnostics belong to structural syntax errors.
 *
 * Examples include:
 *
 *     malformed imported knowledge expression
 *     invalid delimiter structure
 *     invalid expression structure
 *     malformed knowledge operation
 *
 * Semantic diagnostics remain downstream.
 *
 * Examples:
 *
 *     unknown knowledge operation
 *     invalid relation
 *     invalid term type
 *     unavailable capability
 *     insufficient resources
 *     unauthorized access
 *     invalid provenance
 *     invalid policy
 *     invalid evidence
 *     invalid deterministic-execution requirement
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * This adapter should have a small, focused conformance suite.
 *
 * POSITIVE
 * --------
 *
 * Every canonical knowledge expression accepted by:
 *
 *     KnowledgeExpressions
 *
 * must remain reachable through:
 *
 *     dataKnowledgeConstruct
 *
 * Examples include:
 *
 *     knowledge assert(subject, relation, object)
 *
 *     knowledge retract(subject, relation, object)
 *
 *     knowledge query(pattern)
 *
 *     knowledge lookup(pattern)
 *
 *     knowledge update(subject, relation, object)
 *
 * The exact accepted surface remains determined by
 * `grammar/expressions/knowledge.g4`.
 *
 * NEGATIVE
 * -------
 *
 * Verify that malformed knowledge syntax remains rejected by the canonical
 * knowledge grammar and is not accidentally accepted by this adapter.
 *
 * BOUNDARY
 * --------
 *
 * Test composition with:
 *
 *     dataQueryConstruct
 *     data declarations
 *     data records
 *     collections
 *     datasets
 *     streams
 *     schemas
 *     transformations
 *     provenance
 *     classical expressions
 *     reasoning
 *     learning
 *     adaptation
 *     policies
 *     contracts
 *     distributed execution
 *     quantum-derived values
 *
 * SCALABILITY
 * ----------
 *
 * Verify that this adapter introduces no finite limit on:
 *
 *     number of knowledge operations
 *     number of knowledge terms
 *     number of relations
 *     number of assertions
 *     number of query results
 *     number of data sources
 *     number of records
 *     data volume
 *     distributed nodes
 *     devices
 *     accelerators
 *
 * These limits are implementation/resource concerns, not grammar concerns.
 *
 * DETERMINISM
 * ----------
 *
 * The adapter must introduce no nondeterministic parser behavior.
 *
 * COMPATIBILITY
 * -------------
 *
 * Existing source-level knowledge syntax must remain owned by
 * `KnowledgeExpressions`.
 *
 * No duplicate token path may be introduced.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * UPSTREAM
 * --------
 *
 *     grammar/expressions/knowledge.g4
 *     grammar/antlr/ZamaniLexer.g4
 *     canonical expression composition
 *
 * IMPORT
 * ------
 *
 *     KnowledgeExpressions
 *
 * PUBLIC EXPORT
 * -------------
 *
 *     dataKnowledgeConstruct
 *
 * CONSUMER
 * --------
 *
 *     grammar/data/data.g4
 *
 * The data dispatcher should expose:
 *
 *     dataKnowledgeConstruct
 *
 * through its data-domain statement/declaration composition as appropriate to
 * the repository's existing data-dispatch model.
 *
 * UNIVERSAL ROOT
 * -------------
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * must continue to consume only the canonical:
 *
 *     Data
 *
 * composition grammar.
 *
 * The root parser should NOT import this leaf directly.
 *
 * ============================================================================
 * REQUIRED DATA.G4 INTEGRATION
 * ============================================================================
 *
 * `grammar/data/data.g4` should import this grammar through its data-domain
 * composition layer.
 *
 * The required architectural direction is:
 *
 *     Data
 *       |
 *       +--> DataQueries
 *       +--> DataSets
 *       +--> DataTransformations
 *       +--> DataTensors
 *       +--> DataTables
 *       +--> DataCollections
 *       +--> DataStreams
 *       +--> ZamaniDataKnowledge
 *
 * and then:
 *
 *     dataStatement
 *          |
 *          +--> dataKnowledgeConstruct
 *
 * or the equivalent existing data-domain dispatch point.
 *
 * The exact placement should follow the current public dispatch structure of
 * `data.g4`; the knowledge syntax itself must not be copied there.
 *
 * ============================================================================
 * REQUIRED AI INTEGRATION
 * ============================================================================
 *
 * `grammar/ai/knowledge.g4` should remain an adapter to:
 *
 *     KnowledgeExpressions
 *
 * rather than importing this data-domain adapter as a replacement.
 *
 * This prevents an ownership inversion:
 *
 *     AI -> Data -> Knowledge
 *
 * when the correct architecture is:
 *
 *                 KnowledgeExpressions
 *                  /             \
 *                 /               \
 *                v                 v
 *             Data adapter      AI adapter
 *
 * The universal knowledge syntax therefore remains reusable by multiple
 * domains without making one domain subordinate to another.
 *
 * ============================================================================
 * REQUIRED EXPRESSION INTEGRATION
 * ============================================================================
 *
 * `grammar/expressions/knowledge.g4` remains the canonical source syntax
 * owner.
 *
 * Its public rule:
 *
 *     knowledgeExpression
 *
 * remains unchanged by this adapter.
 *
 * This file must never modify the meaning of that rule.
 *
 * ============================================================================
 * REQUIRED QUERY INTEGRATION
 * ============================================================================
 *
 * Knowledge query semantics must remain distinct from:
 *
 *     dataQueryExpression
 *
 * A knowledge query may consume or produce values associated with data-query
 * execution, but the two grammars remain independently owned.
 *
 * ============================================================================
 * REQUIRED PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Provenance remains owned by:
 *
 *     grammar/data/provenance.g4
 *
 * This file consumes that semantic capability without copying provenance
 * syntax.
 *
 * ============================================================================
 * REQUIRED QUANTUM INTEGRATION
 * ============================================================================
 *
 * Knowledge participating in quantum workflows remains target-independent.
 *
 * The only canonical quantum semantic boundary is:
 *
 *     quantum::ir
 *
 * This file must never introduce a knowledge-specific quantum IR.
 *
 * ============================================================================
 * REQUIRED COMPILER INTEGRATION
 * ============================================================================
 *
 * The compiler pipeline remains:
 *
 *     source
 *       |
 *       v
 *     lexer
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     semantic model
 *       |
 *       +--> types
 *       +--> effects
 *       +--> capabilities
 *       +--> resources
 *       +--> contracts
 *       +--> policies
 *       +--> provenance
 *       |
 *       v
 *     canonical IR
 *       |
 *       +--> classical
 *       +--> quantum::ir
 *       +--> data
 *       +--> distributed
 *       +--> hardware/HDL
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     lowering
 *       |
 *       v
 *     routing / scheduling
 *       |
 *       v
 *     resilience
 *       |
 *       v
 *     ZQN / HAL where applicable
 *       |
 *       v
 *     target realization
 *
 * This file participates only at the parser/data-domain boundary.
 *
 * ============================================================================
 * REQUIRED RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime responsibilities remain outside this grammar:
 *
 *     resource discovery
 *     capability discovery
 *     authorization
 *     storage selection
 *     provider selection
 *     distributed placement
 *     scheduling
 *     retry
 *     recovery
 *     adaptation
 *     monitoring
 *     provenance recording
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [x] It contains exactly one owned public rule.
 *
 *     [x] It imports KnowledgeExpressions.
 *
 *     [x] It does not duplicate knowledge syntax.
 *
 *     [x] It introduces no lexer rules.
 *
 *     [x] It introduces no new keyword.
 *
 *     [x] It introduces no Rust code.
 *
 *     [x] It introduces no unsafe code.
 *
 *     [x] It introduces no runtime behavior.
 *
 *     [x] It introduces no physical hardware assumptions.
 *
 *     [x] It introduces no capacity ceilings.
 *
 *     [x] It introduces no provider-specific implementation.
 *
 *     [x] It preserves the domain-neutral AST boundary.
 *
 *     [x] It preserves the canonical knowledge expression owner.
 *
 *     [x] It can be consumed by the canonical Data composition grammar.
 *
 *     [x] It can coexist with data queries without duplicating query syntax.
 *
 *     [x] It can coexist with provenance without duplicating provenance syntax.
 *
 *     [x] It can coexist with AI knowledge without creating competing syntax.
 *
 *     [x] Quantum participation remains downstream through quantum::ir.
 *
 *     [x] POCO-REAF remains independent of machine scale.
 *
 *     [x] Future knowledge providers do not require grammar modification.
 *
 *     [x] Future hardware targets do not require grammar modification.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL INVARIANT
 * ============================================================================
 *
 * This file answers exactly one question:
 *
 *     "How does canonical Zamani knowledge syntax enter the data-domain
 *      composition boundary?"
 *
 * It does NOT answer:
 *
 *     "How is knowledge stored?"
 *     "How is knowledge queried physically?"
 *     "How does reasoning work?"
 *     "How does learning work?"
 *     "How does adaptation work?"
 *     "Which database is used?"
 *     "Which graph engine is used?"
 *     "Which CPU executes it?"
 *     "Which GPU executes it?"
 *     "Which QPU executes it?"
 *     "How many nodes exist?"
 *     "How is quantum routing performed?"
 *     "How is QEC performed?"
 *
 * Those responsibilities remain downstream.
 *
 * The architectural invariant is:
 *
 *     ONE KNOWLEDGE SYNTAX
 *             |
 *       +-----+-----+
 *       |           |
 *       v           v
 *      DATA         AI
 *       |           |
 *       +-----+-----+
 *             |
 *             v
 *      DOMAIN-NEUTRAL AST
 *             |
 *             v
 *       SEMANTIC MODEL
 *             |
 *       +-----+------+
 *       |            |
 *       v            v
 *   CLASSICAL    quantum::ir
 *       |            |
 *       +-----+------+
 *             |
 *             v
 *       TARGET-INDEPENDENT
 *          REALIZATION
 *
 * This is the required architecture for scalable Zamani knowledge support.
 *
 * ============================================================================
 */
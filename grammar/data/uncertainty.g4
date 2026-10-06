/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/data/uncertainty.g4
 *
 * GRAMMAR
 * -------
 * ZamaniDataUncertainty
 *
 * STATUS
 * ------
 * PRODUCTION DATA-DOMAIN COMPOSITION ADAPTER
 *
 * IMPLEMENTATION BASELINE
 * ------------------------
 * Rust 1.97+
 * Rust 2021+
 * Safe Rust only
 * No unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the DATA-DOMAIN COMPOSITION BOUNDARY for canonical Zamani
 * uncertainty expressions.
 *
 * It does NOT define a second uncertainty language.
 *
 * The single source-level uncertainty syntax authority is:
 *
 *     grammar/expressions/uncertainty.g4
 *
 * whose public rule is:
 *
 *     uncertaintyExpression
 *
 * This file exposes that canonical construct to the data domain through:
 *
 *     dataUncertaintyConstruct
 *
 * The architecture is therefore:
 *
 *     grammar/expressions/uncertainty.g4
 *                 |
 *                 v
 *       uncertaintyExpression
 *                 |
 *                 v
 *     grammar/data/uncertainty.g4
 *                 |
 *                 v
 *       dataUncertaintyConstruct
 *                 |
 *                 v
 *          data-domain semantic context
 *
 * There is ONE uncertainty syntax authority.
 *
 * ============================================================================
 * WHY THIS FILE EXISTS
 * ============================================================================
 *
 * Uncertainty is cross-domain.
 *
 * It may occur in:
 *
 *     classical computation
 *     numerical computation
 *     statistics
 *     data processing
 *     data queries
 *     knowledge systems
 *     reasoning
 *     learning
 *     adaptation
 *     distributed computation
 *     networking
 *     simulation
 *     quantum/classical computation
 *     hardware observation
 *     HDL verification
 *     scientific computation
 *     future computational domains
 *
 * Cross-domain use does NOT justify copying the uncertainty grammar into
 * every domain.
 *
 * Instead:
 *
 *     one canonical syntax
 *             |
 *       +-----+-------------------------------+
 *       |             |            |           |
 *       v             v            v           v
 *     expressions    data         AI         quantum
 *       |             |            |           |
 *       +-------------+------------+-----------+
 *                         |
 *                         v
 *                common semantic model
 *
 * This file exists only to establish the DATA DOMAIN boundary.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * The following rule belongs exclusively to:
 *
 *     grammar/expressions/uncertainty.g4
 *
 *     uncertaintyExpression
 *     uncertainValueExpression
 *     uncertaintyValueExpression
 *     uncertaintyValue
 *     uncertaintyArgumentList
 *     uncertaintyArgument
 *     uncertaintyNamedArgument
 *     uncertaintyPositionalArgument
 *     uncertaintyFieldName
 *
 * THIS FILE MUST NOT redefine any of them.
 *
 * In particular, DO NOT add:
 *
 *     uncertaintyExpression
 *     dataUncertaintyExpression
 *     dataUncertainExpression
 *     dataProbabilityExpression
 *
 * merely to reproduce the same syntax.
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns exactly one language-domain composition boundary:
 *
 *     dataUncertaintyConstruct
 *
 * The rule means:
 *
 *     canonical Zamani uncertainty syntax consumed in a data-domain context.
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     - uncertainty expression syntax;
 *     - uncertain(...) syntax;
 *     - uncertainty(...) syntax;
 *     - probability syntax;
 *     - confidence syntax;
 *     - belief syntax;
 *     - likelihood syntax;
 *     - distribution syntax;
 *     - evidence syntax;
 *     - provenance syntax;
 *     - reasoning syntax;
 *     - knowledge syntax;
 *     - learning syntax;
 *     - adaptation syntax;
 *     - query syntax;
 *     - schema syntax;
 *     - data schema syntax;
 *     - general expressions;
 *     - general types;
 *     - literals;
 *     - identifiers;
 *     - qualified names;
 *     - effects;
 *     - capabilities;
 *     - resources;
 *     - contracts;
 *     - policies;
 *     - statistical algorithms;
 *     - probability algorithms;
 *     - sampling algorithms;
 *     - distribution implementations;
 *     - machine-learning algorithms;
 *     - quantum operations;
 *     - quantum routing;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - hardware selection;
 *     - runtime execution;
 *     - storage;
 *     - database engines;
 *     - SQL;
 *     - JSON;
 *     - XML;
 *     - canonical IR.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 * Canonical lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Canonical uncertainty grammar:
 *
 *     grammar/expressions/uncertainty.g4
 *
 * Grammar identity:
 *
 *     UncertaintyExpressions
 *
 * Public imported rule:
 *
 *     uncertaintyExpression
 *
 * This file intentionally depends on the canonical uncertainty grammar rather
 * than reproducing its rules.
 *
 * ============================================================================
 * EXPORTS
 * ============================================================================
 *
 * Primary public rule:
 *
 *     dataUncertaintyConstruct
 *
 * No other public uncertainty syntax is exported from this file.
 *
 * ============================================================================
 * CONSUMED BY
 * ============================================================================
 *
 * Potential consumers include:
 *
 *     grammar/data/data.g4
 *     grammar/data/knowledge.g4
 *     grammar/data/queries.g4
 *     grammar/data/schemas.g4
 *     grammar/data/datasets.g4
 *     grammar/data/transformations.g4
 *     grammar/data/streams.g4
 *     grammar/ai/
 *     grammar/hybrid/
 *     grammar/quantum/
 *     grammar/classical/
 *
 * Consumers MUST consume:
 *
 *     dataUncertaintyConstruct
 *
 * rather than reproducing uncertainty syntax.
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * The canonical lexer remains:
 *
 *     ZamaniLexer
 *
 * This file contains no lexer rules.
 *
 * The canonical uncertainty parser grammar is imported through:
 *
 *     import UncertaintyExpressions;
 *
 * No token aliases are introduced.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This adapter creates parser structure only.
 *
 * It MUST NOT create a second data-specific uncertainty AST.
 *
 * The canonical domain-neutral frontend AST remains responsible for the
 * representation of:
 *
 *     uncertaintyExpression
 *
 * The semantic representation should retain enough information to distinguish:
 *
 *     source-level uncertainty value
 *
 * from:
 *
 *     data-domain contextual use.
 *
 * Context may be represented by the surrounding AST/semantic node rather than
 * by creating:
 *
 *     DataUncertaintyValue
 *
 * as a separate universal type.
 *
 * The AST must preserve information supplied by the canonical uncertainty
 * grammar, including:
 *
 *     - source span;
 *     - uncertain/uncertainty introducer;
 *     - wrapped value expression;
 *     - ordered arguments;
 *     - named arguments;
 *     - positional arguments;
 *     - field names;
 *     - named-argument separator;
 *     - source ordering.
 *
 * This adapter does not reinterpret any of those fields.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns the meaning of uncertainty.
 *
 * For a data-domain uncertainty expression, semantic analysis may determine:
 *
 *     - the type of the underlying data value;
 *     - the uncertainty type;
 *     - probability semantics;
 *     - confidence semantics;
 *     - distribution compatibility;
 *     - evidence compatibility;
 *     - provenance compatibility;
 *     - metadata compatibility;
 *     - duplicate metadata;
 *     - conflicting metadata;
 *     - schema compatibility;
 *     - query compatibility;
 *     - transformation compatibility;
 *     - data lineage;
 *     - reproducibility requirements;
 *     - deterministic execution requirements;
 *     - required effects;
 *     - required capabilities;
 *     - required resources;
 *     - applicable policies.
 *
 * The parser performs none of these semantic operations.
 *
 * ============================================================================
 * DATA SEMANTIC INTEGRATION
 * ============================================================================
 *
 * Uncertainty may annotate or describe:
 *
 *     data values
 *     records
 *     collections
 *     sequences
 *     streams
 *     datasets
 *     tables
 *     tensors
 *     query results
 *     transformations
 *     measurements
 *     observations
 *     model outputs
 *     knowledge results
 *     reasoning results
 *     simulation results
 *     distributed results
 *     hardware observations
 *
 * The concrete representation is determined by:
 *
 *     type analysis
 *     semantic analysis
 *     execution planning
 *     capabilities
 *     resources
 *     policies
 *     target realization
 *
 * This grammar does not choose any representation.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Uncertainty is compatible with the universal type system.
 *
 * Conceptually, semantic analysis may represent:
 *
 *     uncertain(T)
 *
 * as an uncertainty-bearing semantic type such as:
 *
 *     Uncertain<T>
 *
 * or another canonical type representation.
 *
 * This grammar does NOT define:
 *
 *     probability precision;
 *     numeric representation;
 *     floating-point width;
 *     fixed-point width;
 *     arbitrary-precision implementation;
 *     interval representation;
 *     symbolic probability representation;
 *     distribution representation.
 *
 * Type representation remains owned by:
 *
 *     grammar/types/
 *
 * and the frontend semantic type system.
 *
 * ============================================================================
 * SCHEMA CONTRACT
 * ============================================================================
 *
 * Data uncertainty may interact with logical schemas.
 *
 * Schema ownership remains:
 *
 *     grammar/data/schemas.g4
 *
 * Examples of semantic relationships include:
 *
 *     schema field -> uncertain value
 *     uncertain value -> schema field
 *     query result -> uncertainty
 *     measurement -> uncertainty
 *     observation -> uncertainty
 *
 * This adapter does not redefine schema syntax.
 *
 * ============================================================================
 * QUERY CONTRACT
 * ============================================================================
 *
 * Query syntax remains owned by:
 *
 *     grammar/data/queries.g4
 *
 * This file does not define:
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
 *     recursive query syntax
 *
 * An uncertainty expression may nevertheless occur in query expressions where
 * the canonical expression grammar permits it.
 *
 * Example semantic use:
 *
 *     uncertainty(query_result, confidence: confidence_value)
 *
 * The query grammar remains the owner of query structure.
 *
 * ============================================================================
 * KNOWLEDGE CONTRACT
 * ============================================================================
 *
 * Knowledge syntax remains owned by:
 *
 *     grammar/expressions/knowledge.g4
 *
 * Uncertainty may wrap knowledge results:
 *
 *     uncertainty(knowledge_result, confidence: confidence_value)
 *
 * or provide uncertainty metadata for knowledge-derived data.
 *
 * This file does NOT define:
 *
 *     assert
 *     retract
 *     query
 *     knowledge terms
 *     knowledge patterns
 *     knowledge operations.
 *
 * ============================================================================
 * REASONING CONTRACT
 * ============================================================================
 *
 * Reasoning syntax remains owned by the reasoning subsystem.
 *
 * Uncertainty may describe:
 *
 *     inference results
 *     deduction results
 *     evidence
 *     hypotheses
 *     conclusions
 *     confidence
 *     belief
 *
 * The data uncertainty adapter does not redefine reasoning syntax.
 *
 * ============================================================================
 * LEARNING CONTRACT
 * ============================================================================
 *
 * Learning syntax remains owned by the AI/learning subsystem.
 *
 * Learning may produce data values carrying uncertainty:
 *
 *     uncertainty(model_result, confidence: model_confidence)
 *
 * The grammar does not enumerate:
 *
 *     model families
 *     learning algorithms
 *     optimizers
 *     architectures
 *     providers
 *     frameworks.
 *
 * Those remain semantic, library, dialect, capability, or implementation
 * concerns.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Uncertainty may preserve:
 *
 *     source
 *     derivation
 *     transformation
 *     evidence
 *     verification
 *     model origin
 *     query origin
 *     measurement origin
 *     version
 *
 * Provenance expression syntax remains owned by:
 *
 *     grammar/expressions/provenance.g4
 *
 * Data provenance declarations remain owned by:
 *
 *     grammar/data/provenance.g4
 *
 * This file does not define another provenance system.
 *
 * Semantic provenance must be normalized into the repository-wide provenance
 * model.
 *
 * ============================================================================
 * EVIDENCE CONTRACT
 * ============================================================================
 *
 * Evidence remains a semantic/canonical concept.
 *
 * An uncertainty expression may carry:
 *
 *     evidence: value
 *
 * but this adapter does not define evidence storage, evidence databases,
 * evidence formats, signatures, verification algorithms, or audit systems.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * The existence of an uncertainty expression does NOT automatically imply a
 * particular effect.
 *
 * For example:
 *
 *     uncertainty(value)
 *
 * does not necessarily imply:
 *
 *     randomness
 *
 * If the underlying value originates from:
 *
 *     measure(...)
 *
 * semantic analysis may determine that measurement is required.
 *
 * If the underlying value originates from:
 *
 *     model.predict(...)
 *
 * semantic analysis may determine model/inference effects.
 *
 * If the value originates from an external service, semantic analysis may
 * determine:
 *
 *     network
 *     IO
 *     foreign
 *
 * effects.
 *
 * The adapter does not infer effects.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Capability analysis may determine requirements such as:
 *
 *     probabilistic.compute
 *     statistical.compute
 *     model.inference
 *     quantum.measurement
 *     data.query
 *     provenance.record
 *     provenance.verify
 *
 * Capability names remain OPEN-WORLD.
 *
 * This file does not enumerate the capability universe.
 *
 * A future capability must not require modification of this adapter merely
 * because the capability exists.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Uncertainty may require resources such as:
 *
 *     compute
 *     memory
 *     storage
 *     communication
 *     accelerator resources
 *     quantum resources
 *     model resources
 *
 * Those requirements are expressed and resolved through the canonical
 * resource system.
 *
 * This file introduces no resource quantities and no resource limits.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Policies may constrain:
 *
 *     uncertainty sources
 *     evidence sources
 *     external services
 *     randomness
 *     reproducibility
 *     provenance
 *     privacy
 *     model usage
 *     data handling
 *     execution strategy
 *
 * Syntax validity does not imply policy authorization.
 *
 * Policy evaluation remains outside this grammar.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing is deterministic.
 *
 * It depends only on:
 *
 *     source text
 *     token stream
 *     grammar version
 *     parser configuration
 *     explicitly selected language configuration
 *
 * It MUST NOT depend on:
 *
 *     hardware
 *     resource availability
 *     filesystem state
 *     network state
 *     wall-clock time
 *     random state
 *     runtime state
 *     installed statistical libraries
 *     model availability
 *     quantum backend availability
 *
 * Runtime uncertainty may be nondeterministic.
 *
 * That is a semantic/runtime property and is not a parser property.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Parsing is non-executing.
 *
 * This grammar MUST NOT:
 *
 *     - sample distributions;
 *     - generate random values;
 *     - invoke models;
 *     - query databases;
 *     - contact network services;
 *     - inspect hardware;
 *     - access credentials;
 *     - invoke foreign functions;
 *     - execute generated code;
 *     - invoke a QPU;
 *     - invoke a simulator.
 *
 * Any such operation belongs downstream to explicitly authorized execution
 * semantics.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * This adapter introduces no machine-dependent assumptions.
 *
 * A data value carrying uncertainty may participate in execution on:
 *
 *     embedded systems
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
 *     future execution substrates
 *
 * subject to semantic feasibility and available resources.
 *
 * The source-level uncertainty syntax remains unchanged.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This file imposes NO language-level finite limit on:
 *
 *     - uncertainty expressions;
 *     - metadata fields;
 *     - positional arguments;
 *     - named arguments;
 *     - nested expressions;
 *     - nested uncertainty;
 *     - data values;
 *     - datasets;
 *     - records;
 *     - collections;
 *     - streams;
 *     - tensors;
 *     - evidence;
 *     - provenance;
 *     - query results;
 *     - distributed participants;
 *     - quantum-derived data;
 *     - hardware observations.
 *
 * The canonical uncertainty grammar uses open-ended ANTLR repetition.
 *
 * "Infinity" here means:
 *
 *     the language introduces no artificial finite machine-capacity ceiling.
 *
 * It does NOT claim that:
 *
 *     hardware;
 *     parser memory;
 *     compiler memory;
 *     runtime memory;
 *     storage;
 *     network capacity
 *
 * are literally infinite.
 *
 * Actual limits are implementation and resource constraints.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST NOT introduce any equivalent of:
 *
 *     MAX_PROBABILITY_BITS
 *     MAX_CONFIDENCE_BITS
 *     MAX_OUTCOMES
 *     MAX_DISTRIBUTION_SIZE
 *     MAX_SAMPLES
 *     MAX_RANDOM_VARIABLES
 *     MAX_UNCERTAINTY_DEPTH
 *     MAX_EVIDENCE
 *     MAX_PROVENANCE
 *     MAX_TENSOR_RANK
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_DEVICE_COUNT
 *
 * It also MUST NOT encode equivalent indirect limits through finite parser
 * alternatives.
 *
 * Programmer-written values remain program semantics.
 *
 * For example:
 *
 *     uncertainty(value, confidence: 0.95)
 *
 * contains a program value.
 *
 * `0.95` is not a language capacity.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Uncertainty can originate from quantum computation.
 *
 * Example:
 *
 *     uncertainty(measure(q), probability: p)
 *
 * This grammar does not define:
 *
 *     - quantum operations;
 *     - physical qubits;
 *     - coupling topology;
 *     - calibration;
 *     - routing;
 *     - scheduling;
 *     - QEC;
 *     - resilience;
 *     - QPU selection.
 *
 * Quantum lowering remains:
 *
 *     source
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic quantum model
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
 *     target
 *
 * No uncertainty-specific quantum IR is introduced.
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Data uncertainty may describe:
 *
 *     sensor values
 *     timing observations
 *     simulation results
 *     hardware measurements
 *     reliability information
 *     verification results
 *
 * It does not define:
 *
 *     bus width
 *     register width
 *     device count
 *     memory capacity
 *     clock count
 *     topology size.
 *
 * Hardware realization remains downstream.
 *
 * ============================================================================
 * DISTRIBUTED BOUNDARY
 * ============================================================================
 *
 * An uncertain data value may originate from:
 *
 *     actor
 *     task
 *     node
 *     service
 *     stream
 *     remote observation
 *     distributed query
 *
 * Distributed syntax remains owned by:
 *
 *     grammar/distributed/
 *     grammar/concurrency/
 *     grammar/networking/
 *
 * This adapter introduces no node or participant limit.
 *
 * ============================================================================
 * INTEROPERABILITY BOUNDARY
 * ============================================================================
 *
 * External uncertainty representations may originate from:
 *
 *     SQL
 *     JSON
 *     XML
 *     foreign APIs
 *     ABI boundaries
 *     external data providers
 *
 * Those grammars remain owned by:
 *
 *     grammar/interoperability/
 *     grammar/dialects/
 *
 * This file does not implement external formats.
 *
 * ============================================================================
 * METAPROGRAMMING BOUNDARY
 * ============================================================================
 *
 * Reflection, introspection, quotation, generation, and compile-time
 * computation may inspect uncertainty constructs through the canonical AST.
 *
 * This file performs no metaprogramming.
 *
 * Any reflective or generated use remains subject to the canonical:
 *
 *     effect
 *     capability
 *     resource
 *     policy
 *     provenance
 *
 * systems.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file creates NO IR.
 *
 * The intended semantic path is:
 *
 *     dataUncertaintyConstruct
 *              |
 *              v
 *     canonical uncertainty AST
 *              |
 *              v
 *     semantic uncertainty model
 *              |
 *              v
 *     canonical semantic representation
 *              |
 *        +-----+---------------------+
 *        |                           |
 *        v                           v
 *   classical/domain IR          quantum::ir
 *        |                           |
 *        +-------------+-------------+
 *                      |
 *                      v
 *             optimization/lowering
 *                      |
 *                 scheduling
 *                      |
 *                 realization
 *
 * There is no:
 *
 *     dataUncertaintyIR
 *     probabilityIR
 *     quantumUncertaintyIR
 *
 * introduced by this file.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE
 * --------
 *
 * The canonical uncertainty grammar owns the detailed syntax tests.
 *
 * This adapter must additionally test:
 *
 *     dataUncertaintyConstruct
 *
 * with canonical uncertainty expressions such as:
 *
 *     uncertain(value)
 *
 *     uncertainty(value)
 *
 *     uncertainty(value, confidence: confidence_value)
 *
 *     uncertainty(value, probability: probability_value)
 *
 *     uncertainty(query_result, evidence: evidence_value)
 *
 *     uncertainty(measurement_result, provenance: source)
 *
 *     uncertainty(tensor_value, distribution: distribution_value)
 *
 *     uncertainty(model_result, confidence: model_confidence)
 *
 *     uncertainty(distributed_result, evidence: evidence_value)
 *
 * NEGATIVE
 * --------
 *
 * The adapter must reject constructs that are invalid under the imported
 * canonical uncertainty grammar.
 *
 * Examples include malformed canonical forms such as:
 *
 *     uncertain()
 *
 *     uncertainty()
 *
 *     uncertain(value,)
 *
 *     uncertain(value, confidence:)
 *
 *     uncertain(value, : confidence)
 *
 *     uncertainty(value, confidence: c confidence: d)
 *
 * Semantic invalidity such as an impossible probability value belongs to
 * semantic tests, not parser tests.
 *
 * BOUNDARY
 * --------
 *
 * Test uncertainty over:
 *
 *     scalar data
 *     records
 *     tuples
 *     collections
 *     streams
 *     tensors
 *     query results
 *     transformation results
 *     knowledge results
 *     reasoning results
 *     learning results
 *     simulation results
 *     distributed results
 *     hardware observations
 *     quantum measurements
 *
 * Also test:
 *
 *     nested uncertainty;
 *     arbitrary metadata extension names;
 *     Unicode identifiers where supported by the canonical lexer;
 *     named metadata;
 *     positional metadata;
 *     provenance metadata;
 *     evidence metadata.
 *
 * SCALABILITY
 * ----------
 *
 * Conformance tests must verify that this adapter introduces no finite limit
 * on:
 *
 *     metadata count
 *     expression size
 *     nested expressions
 *     data size
 *     tensor rank
 *     evidence count
 *     provenance depth
 *     distributed participants
 *     quantum resources
 *     hardware resources.
 *
 * The tests themselves must use finite inputs because actual test execution
 * consumes finite resources.
 *
 * DETERMINISM
 * -----------
 *
 * Repeated parsing of the same token stream with the same parser configuration
 * must produce equivalent parser structure.
 *
 * The parser must not inspect:
 *
 *     hardware
 *     resource availability
 *     network state
 *     wall-clock time
 *     randomness.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This file does not introduce a new source keyword.
 *
 * All uncertainty keywords remain owned by:
 *
 *     grammar/lexer/keywords.g4
 *     grammar/lexer/tokens.g4
 *
 * Therefore adding this adapter does not create a second lexical vocabulary.
 *
 * Existing source forms remain governed by:
 *
 *     grammar/expressions/uncertainty.g4
 *
 * Compatibility changes to uncertainty syntax belong to that canonical
 * expression grammar and the compatibility subsystem.
 *
 * ============================================================================
 * SPECIFICATION CONTRACT
 * ============================================================================
 *
 * The normative uncertainty specification should be:
 *
 *     grammar/spec/uncertainty.md
 *
 * The normative data specification should include this integration through:
 *
 *     grammar/spec/
 *
 * The overall architecture is governed by:
 *
 *     grammar/DESIGN.md
 *
 * The overall POCO-REAF contract is governed by:
 *
 *     grammar/specification/poco-reaf.md
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * UPSTREAM
 * --------
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/lexer/tokens.g4
 *     grammar/lexer/keywords.g4
 *     grammar/expressions/uncertainty.g4
 *
 * THIS FILE
 * ---------
 *
 * Exports:
 *
 *     dataUncertaintyConstruct
 *
 * DOWNSTREAM
 * ----------
 *
 *     data-domain parser composition
 *     domain-neutral AST
 *     semantic analysis
 *     type analysis
 *     effect analysis
 *     capability analysis
 *     resource analysis
 *     policy analysis
 *     provenance analysis
 *     canonical semantic representation
 *
 * DOMAIN CONSUMERS
 * ----------------
 *
 *     data
 *     classical
 *     AI
 *     hybrid
 *     quantum-derived data
 *     distributed
 *     hardware/HDL observations
 *
 * TEST OWNERS
 * -----------
 *
 *     grammar/tests/data/
 *     grammar/tests/expressions/
 *     grammar/tests/semantic/
 *     grammar/tests/scalability/
 *     grammar/tests/portability/
 *     grammar/tests/quantum/
 *     grammar/tests/hybrid/
 *     grammar/tests/ai/
 *     grammar/tests/distributed/
 *     grammar/tests/boundary/
 *     grammar/tests/negative/
 *
 * AST OWNER
 * ---------
 *
 *     existing domain-neutral frontend AST
 *
 * SEMANTIC OWNER
 * --------------
 *
 *     canonical uncertainty semantic model
 *
 * IR OWNER
 * --------
 *
 *     canonical semantic representation
 *
 *     quantum::ir where quantum lowering is applicable
 *
 * SPEC OWNER
 * ----------
 *
 *     grammar/spec/uncertainty.md
 *     grammar/spec/resources.md
 *     grammar/spec/effects.md
 *     grammar/spec/provenance.md
 *     grammar/spec/policies.md
 *     grammar/specification/poco-reaf.md
 *
 * ============================================================================
 * ROOT-GRAMMAR INTEGRATION RULE
 * ============================================================================
 *
 * The canonical root/parser composition MUST NOT copy uncertainty syntax.
 *
 * It should consume:
 *
 *     dataUncertaintyConstruct
 *
 * only where an explicit data-domain boundary is required.
 *
 * IMPORTANT:
 *
 * `grammar/data/data.g4` already has:
 *
 *     dataExpressionStmt
 *         : expression ';'
 *         ;
 *
 * Since `expression` already includes:
 *
 *     uncertaintyExpression
 *
 * adding:
 *
 *     dataUncertaintyStmt
 *
 * as another alternative to `dataStatement` would create two parser paths
 * for the same source-level uncertainty expression.
 *
 * Therefore NO such duplicate alternative should be added merely to make this
 * adapter exist.
 *
 * If the parser architecture later introduces explicit domain-context
 * dispatch, that dispatcher should consume:
 *
 *     dataUncertaintyConstruct
 *
 * without copying its implementation.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * THIS FILE IS DONE WHEN:
 *
 * [x] It is a parser grammar.
 *
 * [x] It uses ZamaniLexer as its lexical authority.
 *
 * [x] It imports the canonical UncertaintyExpressions grammar.
 *
 * [x] It defines exactly one data-domain uncertainty boundary.
 *
 * [x] It does not redefine uncertaintyExpression.
 *
 * [x] It does not create uncertainty-specific data syntax.
 *
 * [x] It does not create a second uncertainty AST.
 *
 * [x] It does not create a second uncertainty semantic model.
 *
 * [x] It does not create a second uncertainty IR.
 *
 * [x] It introduces no machine-capacity limit.
 *
 * [x] It introduces no provider catalog.
 *
 * [x] It introduces no distribution catalog.
 *
 * [x] It introduces no numerical precision requirement.
 *
 * [x] It introduces no hardware selection.
 *
 * [x] It introduces no quantum backend selection.
 *
 * [x] It introduces no runtime execution.
 *
 * [x] It contains no Rust actions.
 *
 * [x] It requires no unsafe Rust.
 *
 * [x] It documents AST integration.
 *
 * [x] It documents semantic integration.
 *
 * [x] It documents type integration.
 *
 * [x] It documents effect integration.
 *
 * [x] It documents capability integration.
 *
 * [x] It documents resource integration.
 *
 * [x] It documents policy integration.
 *
 * [x] It documents provenance integration.
 *
 * [x] It documents quantum integration.
 *
 * [x] It documents HDL/hardware integration.
 *
 * [x] It documents distributed integration.
 *
 * [x] It documents interoperability.
 *
 * [x] It documents deterministic parsing.
 *
 * [x] It documents scalability.
 *
 * [x] It documents hard-coding prohibition.
 *
 * [x] It documents compatibility.
 *
 * [x] It documents tests.
 *
 * [ ] The imported UncertaintyExpressions grammar successfully participates in
 *     the repository's canonical parser composition.
 *
 * [ ] The frontend AST has exactly one semantic representation for uncertainty.
 *
 * [ ] Semantic uncertainty analysis is implemented.
 *
 * [ ] Type checking is implemented.
 *
 * [ ] Effect checking is implemented.
 *
 * [ ] Capability checking is implemented.
 *
 * [ ] Resource checking is implemented.
 *
 * [ ] Policy checking is implemented.
 *
 * [ ] Provenance propagation is implemented.
 *
 * [ ] Canonical IR lowering is implemented.
 *
 * [ ] Required conformance tests pass.
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar ZamaniDataUncertainty;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * CANONICAL UNCERTAINTY IMPORT
 * ============================================================================
 *
 * The complete uncertainty expression syntax remains owned by:
 *
 *     grammar/expressions/uncertainty.g4
 *
 * Grammar:
 *
 *     UncertaintyExpressions
 *
 * Public rule:
 *
 *     uncertaintyExpression
 *
 * ============================================================================
 */

import UncertaintyExpressions;


/*
 * ============================================================================
 * PUBLIC DATA-DOMAIN BOUNDARY
 * ============================================================================
 *
 * This is intentionally the ONLY rule owned by this file.
 *
 * It delegates directly to the canonical uncertainty expression.
 * ============================================================================
 */

dataUncertaintyConstruct
    : uncertaintyExpression
    ;
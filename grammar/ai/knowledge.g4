/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/ai/knowledge.g4
 *
 * Grammar:
 *     AIKnowledge
 *
 * Status:
 *     CANONICAL AI-DOMAIN KNOWLEDGE COMPOSITION ADAPTER
 *
 * Implementation baseline:
 *     Rust 1.97 or later
 *     Rust 2021
 *     Safe Rust only
 *     No unsafe Rust
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the AI-domain COMPOSITION BOUNDARY for computational
 * knowledge.
 *
 * IMPORTANT:
 *
 * This file does NOT define a second knowledge language.
 *
 * The canonical source-level knowledge syntax is owned by:
 *
 *     grammar/expressions/knowledge.g4
 *
 * whose parser grammar is:
 *
 *     KnowledgeExpressions
 *
 * This file imports that grammar and exposes its canonical public boundary
 * to the AI composition grammar.
 *
 * Therefore the architecture is:
 *
 *     grammar/expressions/knowledge.g4
 *                    |
 *                    v
 *             KnowledgeExpressions
 *                    |
 *                    v
 *             knowledgeExpression
 *                    |
 *                    v
 *        grammar/ai/knowledge.g4
 *                    |
 *                    v
 *             aiKnowledgeConstruct
 *                    |
 *                    v
 *             grammar/ai/ai.g4
 *                    |
 *                    v
 *              Zamani parser
 *
 * This preserves a SINGLE knowledge syntax authority.
 *
 * ============================================================================
 * CORE ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * Knowledge is a GENERAL COMPUTATIONAL ABSTRACTION.
 *
 * It is not inherently tied to:
 *
 *     - machine learning;
 *     - neural networks;
 *     - symbolic AI;
 *     - databases;
 *     - graph databases;
 *     - theorem provers;
 *     - a particular storage engine;
 *     - a particular hardware target;
 *     - classical computation;
 *     - quantum computation;
 *     - HDL;
 *     - distributed computation.
 *
 * Knowledge may represent:
 *
 *     - facts;
 *     - relations;
 *     - observations;
 *     - scientific data;
 *     - configuration;
 *     - compiler information;
 *     - resource information;
 *     - capability information;
 *     - security information;
 *     - provenance;
 *     - model information;
 *     - quantum experiment results;
 *     - hardware observations;
 *     - distributed-system state;
 *     - application-defined information;
 *     - future computational information.
 *
 * The semantic meaning of those values belongs downstream.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 * -------------
 *
 *     aiKnowledgeConstruct
 *
 * and the AI-domain composition boundary by which canonical knowledge
 * expressions become visible to:
 *
 *     grammar/ai/ai.g4
 *
 * THIS FILE DOES NOT OWN
 * ---------------------
 *
 *     - keyword definitions;
 *     - identifiers;
 *     - literals;
 *     - operators;
 *     - punctuation;
 *     - expression precedence;
 *     - primaryExpression;
 *     - knowledge assertion syntax;
 *     - knowledge retraction syntax;
 *     - knowledge query syntax;
 *     - knowledge lookup syntax;
 *     - knowledge update syntax;
 *     - knowledge patterns;
 *     - knowledge terms;
 *     - data query syntax;
 *     - SQL;
 *     - graph query syntax;
 *     - inference;
 *     - deduction;
 *     - induction;
 *     - abduction;
 *     - causal reasoning;
 *     - learning;
 *     - adaptation;
 *     - probability algorithms;
 *     - model algorithms;
 *     - storage;
 *     - databases;
 *     - knowledge engines;
 *     - AST implementation;
 *     - semantic implementation;
 *     - type checking;
 *     - effect checking;
 *     - capability resolution;
 *     - resource resolution;
 *     - policy enforcement;
 *     - authorization;
 *     - provenance implementation;
 *     - IR implementation;
 *     - optimization;
 *     - lowering;
 *     - scheduling;
 *     - routing;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - target selection;
 *     - runtime execution.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There MUST be exactly one canonical source-level knowledge grammar:
 *
 *     grammar/expressions/knowledge.g4
 *
 * There MUST be exactly one canonical expression-level knowledge boundary:
 *
 *     knowledgeExpression
 *
 * This file MUST NOT recreate:
 *
 *     knowledgeExpression
 *     knowledgeAssertionExpression
 *     knowledgeRetractionExpression
 *     knowledgeQueryExpression
 *     knowledgeLookupExpression
 *     knowledgeUpdateExpression
 *
 * Those rules belong to:
 *
 *     grammar/expressions/knowledge.g4
 *
 * This prevents the AI subsystem from becoming a competing language.
 *
 * ============================================================================
 * WHY THIS FILE EXISTS
 * ============================================================================
 *
 * The existence of an AI-domain adapter is intentional.
 *
 * AI constructs need to be composed through:
 *
 *     grammar/ai/ai.g4
 *
 * while knowledge itself is a universal expression capability.
 *
 * The adapter therefore provides:
 *
 *     universal knowledge syntax
 *             |
 *             v
 *       AI composition
 *
 * without changing the universal knowledge syntax.
 *
 * This allows future non-AI consumers to use the same knowledge expression
 * boundary without importing the AI subsystem.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 *     grammar/expressions/knowledge.g4
 *         |
 *         +--> KnowledgeExpressions
 *         |
 *         +--> knowledgeExpression
 *
 *     grammar/antlr/ZamaniLexer.g4
 *         |
 *         +--> canonical token vocabulary
 *
 * This file deliberately does NOT import:
 *
 *     grammar/expressions/query.g4
 *
 * because knowledgeExpressions already owns its knowledge/query distinction
 * and must consume the canonical data/query boundary internally where
 * appropriate.
 *
 * This file also does NOT import:
 *
 *     grammar/ai/reasoning.g4
 *     grammar/ai/learning.g4
 *     grammar/ai/adaptation.g4
 *     grammar/ai/agents.g4
 *
 * Knowledge is an input/state capability that those subsystems may consume;
 * it is not owned by them.
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * ANTLR grammar imports use GRAMMAR NAMES, not filesystem paths.
 *
 * Therefore the import below refers to:
 *
 *     parser grammar KnowledgeExpressions;
 *
 * declared by:
 *
 *     grammar/expressions/knowledge.g4
 *
 * The ANTLR build must make grammar/expressions available on its grammar
 * search/import path.
 *
 * ============================================================================
 * EXPORT CONTRACT
 * ============================================================================
 *
 * EXPORTS
 * -------
 *
 *     aiKnowledgeConstruct
 *
 * The imported canonical rule:
 *
 *     knowledgeExpression
 *
 * remains owned by KnowledgeExpressions.
 *
 * This adapter does not re-export individual knowledge implementation rules
 * as new AI-owned rules.
 *
 * ============================================================================
 * CONSUMED BY
 * ============================================================================
 *
 * Primary consumer:
 *
 *     grammar/ai/ai.g4
 *
 * Expected integration:
 *
 *     import AIKnowledge;
 *
 * followed by:
 *
 *     aiConstruct
 *         : ...
 *         | aiKnowledgeConstruct
 *         | ...
 *         ;
 *
 * IMPORTANT:
 *
 * `AIKnowledge` must be imported only once through the canonical AI
 * composition boundary.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This file creates NO Rust AST type.
 *
 * `aiKnowledgeConstruct` is a parser composition boundary only.
 *
 * The resulting parse tree must ultimately map to the existing
 * domain-neutral frontend AST representation for:
 *
 *     knowledge assertion
 *     knowledge retraction
 *     knowledge query
 *     knowledge lookup
 *     knowledge update
 *
 * The AST MUST NOT contain:
 *
 *     - GPU identifiers;
 *     - CPU identifiers;
 *     - QPU identifiers;
 *     - physical qubit identifiers;
 *     - device identifiers;
 *     - storage-engine identifiers;
 *     - database-engine assumptions;
 *     - vendor-specific placement;
 *     - physical topology;
 *     - scheduling decisions;
 *     - routing decisions.
 *
 * Source spans MUST be preserved by the frontend AST implementation.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * This grammar does not decide what knowledge means operationally.
 *
 * Semantic analysis determines:
 *
 *     - the knowledge operation;
 *     - the subject;
 *     - the relation;
 *     - the object;
 *     - the pattern;
 *     - the knowledge source;
 *     - the evidence;
 *     - the provenance;
 *     - the metadata;
 *     - the applicable policy;
 *     - required capabilities;
 *     - required resources;
 *     - produced effects;
 *     - determinism;
 *     - authorization;
 *     - type compatibility;
 *     - contract validity;
 *     - execution realization.
 *
 * Parser acceptance MUST NOT imply semantic validity.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Knowledge terms use the universal Zamani expression/type system through
 * the canonical knowledge grammar.
 *
 * This adapter introduces NO:
 *
 *     AIKnowledgeType
 *     KnowledgeType
 *     FactType
 *     KnowledgeResultType
 *
 * Semantic results may be represented by existing universal types such as:
 *
 *     scalar
 *     tuple
 *     record
 *     collection
 *     stream
 *     relation
 *     graph value
 *     option
 *     result
 *     tensor value
 *     symbolic value
 *     provider-defined logical value
 *
 * according to the semantic/type system.
 *
 * No fixed result cardinality is imposed.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing this grammar has NO runtime effects.
 *
 * Semantic analysis may classify a knowledge operation with effects including,
 * where applicable:
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
 * The effect universe is owned by the universal effect system.
 *
 * This file MUST NOT define a competing effect taxonomy.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Knowledge operations may require semantic capabilities such as:
 *
 *     knowledge.read
 *     knowledge.write
 *     knowledge.query
 *     knowledge.retract
 *     knowledge.update
 *
 * or future capabilities defined by the capability system.
 *
 * Capability names are semantic data.
 *
 * This file MUST NOT enumerate physical devices or hardware capacities.
 *
 * A capability is not a device identifier.
 *
 * For example, this grammar must never encode:
 *
 *     GPU 0
 *     QPU 1
 *     device 7
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Knowledge processing may consume arbitrary resources.
 *
 * Resource requirements are resolved by:
 *
 *     grammar/resources/
 *
 * and downstream semantic/compiler systems.
 *
 * This file does not define:
 *
 *     - memory ceilings;
 *     - storage ceilings;
 *     - record-count ceilings;
 *     - graph-size ceilings;
 *     - query-size ceilings;
 *     - worker-count ceilings;
 *     - node-count ceilings;
 *     - device-count ceilings;
 *     - tensor-rank ceilings;
 *     - model-size ceilings.
 *
 * No finite universal scaling boundary is encoded in this grammar.
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Knowledge operations may participate in:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * contracts.
 *
 * Contract syntax remains owned by the universal validation subsystem.
 *
 * This adapter neither duplicates contract grammar nor grants contract
 * authority.
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Knowledge operations may be subject to:
 *
 *     security policies;
 *     privacy policies;
 *     evidence policies;
 *     provenance policies;
 *     resource policies;
 *     execution policies;
 *     data-access policies;
 *     adaptation policies.
 *
 * Policy syntax and enforcement remain owned by the policy/security systems.
 *
 * Parsing a knowledge construct MUST NOT grant permission to:
 *
 *     read data;
 *     write data;
 *     retract information;
 *     access a remote source;
 *     execute code;
 *     access hardware;
 *     modify a model;
 *     modify program state.
 *
 * ============================================================================
 * EVIDENCE INTEGRATION
 * ============================================================================
 *
 * Knowledge may carry or reference evidence.
 *
 * Evidence may originate from:
 *
 *     - observations;
 *     - datasets;
 *     - classical computation;
 *     - quantum measurement;
 *     - simulation;
 *     - hardware observations;
 *     - model outputs;
 *     - distributed computation;
 *     - network services;
 *     - scientific computation;
 *     - future domains.
 *
 * Evidence semantics are owned by the universal evidence/provenance systems.
 *
 * This file does not create an AI-specific evidence model.
 *
 * ============================================================================
 * PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Knowledge operations must remain compatible with the universal provenance
 * model.
 *
 * Provenance may record:
 *
 *     source;
 *     derived_from;
 *     generated_by;
 *     transformed_by;
 *     verified_by;
 *     reason;
 *     evidence;
 *     decision;
 *     version;
 *     execution context;
 *     policy context.
 *
 * The exact provenance representation belongs downstream.
 *
 * ============================================================================
 * REASONING INTEGRATION
 * ============================================================================
 *
 * Knowledge is an input/state boundary for reasoning.
 *
 * The relationship is:
 *
 *     knowledge
 *         |
 *         +--> infer
 *         +--> deduce
 *         +--> reason
 *         +--> explain
 *         +--> decide
 *
 * Reasoning syntax remains owned by:
 *
 *     grammar/ai/reasoning.g4
 *
 *     grammar/statements/reason.g4
 *     grammar/statements/infer.g4
 *     grammar/statements/deduce.g4
 *
 * where applicable.
 *
 * This file must not duplicate those constructs.
 *
 * ============================================================================
 * LEARNING INTEGRATION
 * ============================================================================
 *
 * Knowledge may provide:
 *
 *     - training data;
 *     - labels;
 *     - observations;
 *     - features;
 *     - learned facts;
 *     - model metadata;
 *     - model outputs.
 *
 * Learning syntax remains owned by the learning subsystem.
 *
 * The conceptual relationship is:
 *
 *     knowledge
 *         |
 *         v
 *     learning
 *         |
 *         v
 *     model
 *         |
 *         v
 *     inference
 *         |
 *         v
 *     knowledge / decision
 *
 * This adapter does not define learning algorithms.
 *
 * ============================================================================
 * ADAPTATION INTEGRATION
 * ============================================================================
 *
 * Knowledge may provide observations for controlled adaptation.
 *
 * It MUST NOT itself authorize:
 *
 *     self-modifying code;
 *     unrestricted model replacement;
 *     unrestricted policy changes;
 *     unrestricted executable-code generation.
 *
 * Adaptation remains governed by:
 *
 *     policy
 *     capability
 *     effect
 *     resource
 *     validation
 *     provenance
 *
 * ============================================================================
 * UNCERTAINTY INTEGRATION
 * ============================================================================
 *
 * Knowledge may contain or reference:
 *
 *     uncertainty;
 *     probability;
 *     distribution;
 *     confidence;
 *     belief;
 *     likelihood.
 *
 * These concepts are interpreted by the universal type/semantic systems.
 *
 * This grammar does not define:
 *
 *     - a floating-point width;
 *     - a probability representation;
 *     - a sampling algorithm;
 *     - a numerical precision ceiling;
 *     - a finite probability-domain size.
 *
 * ============================================================================
 * DATA INTEGRATION
 * ============================================================================
 *
 * Knowledge and ordinary data queries are related but distinct.
 *
 * Canonical data-query ownership remains:
 *
 *     grammar/data/queries.g4
 *
 * Expression-level query integration remains:
 *
 *     grammar/expressions/query.g4
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
 * nor any competing SQL-like grammar.
 *
 * A semantic knowledge operation may consume a data-query result through the
 * existing universal expression/data-query boundary.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Knowledge values may participate in ordinary classical computation.
 *
 * No classical numerical semantics are defined here.
 *
 * Arithmetic, tensor computation, collections, records, functions, and other
 * classical operations remain owned by their respective universal/domain
 * grammars.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Knowledge may represent or consume:
 *
 *     quantum measurement results;
 *     experimental observations;
 *     circuit metadata;
 *     calibration information;
 *     resource information;
 *     execution results;
 *     uncertainty information.
 *
 * This file MUST NOT define:
 *
 *     qubits;
 *     quantum gates;
 *     physical qubit identifiers;
 *     coupling maps;
 *     routing;
 *     scheduling;
 *     decomposition;
 *     QEC;
 *     ZQN;
 *     HAL.
 *
 * Quantum computation remains owned by the quantum subsystem.
 *
 * When knowledge participates in quantum computation, the downstream
 * semantic path remains:
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
 *     resilience / QEC / ZQN
 *       |
 *       v
 *     HAL
 *       |
 *       v
 *     target realization
 *
 * No quantum implementation details leak into this grammar.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Knowledge may describe or carry:
 *
 *     hardware observations;
 *     verification facts;
 *     synthesis metadata;
 *     timing information;
 *     capability information;
 *     resource information;
 *     simulation results.
 *
 * Hardware realization remains downstream.
 *
 * No fixed:
 *
 *     register width;
 *     signal width;
 *     device count;
 *     memory size;
 *     core count;
 *     accelerator count;
 *     topology size
 *
 * is encoded here.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Knowledge may be:
 *
 *     local;
 *     distributed;
 *     replicated;
 *     streamed;
 *     versioned;
 *     remote;
 *     eventually consistent;
 *     strongly consistent;
 *     provider-defined.
 *
 * These are semantic/execution concerns.
 *
 * The grammar imposes no node-count or topology ceiling.
 *
 * ============================================================================
 * INTEROPERABILITY INTEGRATION
 * ============================================================================
 *
 * Knowledge may interoperate with:
 *
 *     JSON;
 *     XML;
 *     SQL;
 *     graph formats;
 *     scientific formats;
 *     model formats;
 *     foreign systems;
 *     external services.
 *
 * Those formats are owned by:
 *
 *     grammar/interoperability/
 *     grammar/dialects/
 *
 * and their corresponding semantic adapters.
 *
 * This file remains format-neutral.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO AI-specific IR.
 *
 * It does not create:
 *
 *     KnowledgeIR
 *     AIKnowledgeIR
 *     KnowledgeGraphIR
 *
 * unless such a representation is explicitly established as part of the
 * repository's canonical semantic/IR architecture outside the grammar.
 *
 * The intended path is:
 *
 *     knowledge syntax
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic knowledge model
 *          |
 *          +--> classical representation
 *          +--> data representation
 *          +--> distributed representation
 *          +--> quantum::ir where required
 *          +--> HDL/hardware representation where required
 *          +--> future domain representation
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * The compiler may:
 *
 *     - validate knowledge operations;
 *     - specialize them;
 *     - optimize them;
 *     - select an implementation;
 *     - negotiate capabilities;
 *     - negotiate resources;
 *     - lower them;
 *     - distribute them;
 *     - route them;
 *     - schedule them;
 *     - provide fallbacks.
 *
 * None of those decisions belong to this grammar.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime realization may use:
 *
 *     - local memory;
 *     - persistent storage;
 *     - distributed storage;
 *     - graph storage;
 *     - databases;
 *     - indexes;
 *     - remote services;
 *     - learned models;
 *     - accelerators;
 *     - quantum systems;
 *     - future execution systems.
 *
 * This grammar does not select any implementation.
 *
 * Runtime resource discovery MUST remain dynamic.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY CONTRACT
 * ============================================================================
 *
 * Knowledge syntax must remain independent of target scale.
 *
 * The grammar imposes NO universal maximum on:
 *
 *     - knowledge assertions;
 *     - facts;
 *     - relations;
 *     - query terms;
 *     - records;
 *     - graph elements;
 *     - knowledge sources;
 *     - evidence;
 *     - provenance records;
 *     - agents;
 *     - machines;
 *     - nodes;
 *     - devices;
 *     - accelerators;
 *     - qubits;
 *     - CPUs;
 *     - GPUs;
 *     - FPGAs;
 *     - QPUs;
 *     - memory;
 *     - storage;
 *     - tensor rank;
 *     - model size;
 *     - network size.
 *
 * In particular, this file MUST NOT contain language-level constants or
 * grammar structures equivalent to:
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
 * "Infinity" in the POCO-REAF objective means that the language does not
 * impose an artificial finite machine-scale ceiling. Actual execution remains
 * bounded only by the resources, capabilities, policies, implementation
 * limits, and physical laws of the realization.
 *
 * ============================================================================
 * DETERMINISM / REPRODUCIBILITY
 * ============================================================================
 *
 * Knowledge operations may be deterministic or nondeterministic depending on
 * their semantic realization.
 *
 * The grammar does not assume either behavior.
 *
 * Determinism and reproducibility are semantic/execution properties and must
 * be represented downstream where required.
 *
 * Provenance may record sufficient information to reproduce a knowledge
 * transformation when the selected execution model permits reproducibility.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * This grammar does not grant authority.
 *
 * In particular:
 *
 *     knowledge query
 *
 * does not automatically grant:
 *
 *     filesystem access;
 *     database access;
 *     network access;
 *     hardware access;
 *     model access;
 *     secret access;
 *     foreign-function access.
 *
 * Required authorization and capability checks belong downstream.
 *
 * ============================================================================
 * PARSER SAFETY CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no embedded Rust;
 *     - no semantic actions;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware access;
 *     - no runtime execution;
 *     - no unsafe code;
 *     - no parser-time evaluation;
 *     - no semantic predicates for resource availability.
 *
 * Rust implementation requirements:
 *
 *     Rust >= 1.97
 *     Rust 2021
 *     safe Rust only
 *     no unsafe
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This adapter does not introduce new lexical keywords.
 *
 * Existing knowledge vocabulary remains owned by:
 *
 *     grammar/lexer/keywords.g4
 *
 * and the canonical lexer boundary:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Consequently, this file does not reserve:
 *
 *     knowledge;
 *     assert;
 *     retract;
 *     query;
 *     fact;
 *     relation;
 *
 * itself.
 *
 * If lexical vocabulary changes, the change must be made in the canonical
 * lexical owner and compatibility analysis must be performed there.
 *
 * This file should not require modification merely because:
 *
 *     - a new relation is introduced;
 *     - a new knowledge provider is introduced;
 *     - a new database is introduced;
 *     - a new model is introduced;
 *     - a new hardware target is introduced;
 *     - a new quantum operation is introduced;
 *     - a new distributed topology is introduced.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive parser tests:
 *
 *     - AI knowledge assertion;
 *     - AI knowledge query;
 *     - AI knowledge lookup;
 *     - AI knowledge update;
 *     - AI knowledge retraction;
 *     - nested knowledge expressions;
 *     - knowledge values used by AI constructs;
 *     - knowledge values used by ordinary expressions.
 *
 * Negative tests:
 *
 *     - malformed knowledge expressions;
 *     - missing operation arguments;
 *     - malformed delimiters;
 *     - invalid expression structure;
 *     - attempts to inject physical target selection;
 *     - invalid parser-level duplication.
 *
 * Boundary tests:
 *
 *     - knowledge -> reasoning;
 *     - knowledge -> learning;
 *     - knowledge -> adaptation;
 *     - knowledge -> agents;
 *     - knowledge -> data query;
 *     - knowledge -> classical computation;
 *     - knowledge -> quantum measurement;
 *     - knowledge -> distributed execution;
 *     - knowledge -> provenance;
 *     - knowledge -> policy.
 *
 * Scalability tests:
 *
 *     - arbitrarily nested expressions permitted by the parser architecture;
 *     - arbitrarily many source-level knowledge operations subject only to
 *       implementation/resource limits;
 *     - symbolic quantities;
 *     - large structured values;
 *     - future domain names without grammar modification.
 *
 * Compatibility tests:
 *
 *     - canonical lexer tokens remain accepted;
 *     - no duplicate keyword path is introduced;
 *     - existing expression-level knowledge syntax remains unchanged.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains:
 *
 *     NO machine-size constants;
 *     NO device identifiers;
 *     NO vendor identifiers;
 *     NO hardware capacity constants;
 *     NO finite knowledge cardinality constants;
 *     NO fixed tensor-rank assumptions;
 *     NO fixed model-size assumptions;
 *     NO fixed node-count assumptions;
 *     NO fixed memory assumptions;
 *     NO parser-time resource checks;
 *     NO target-selection rules;
 *     NO backend-specific syntax.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when all of the following are true:
 *
 *     [x] It has exactly one responsibility: AI knowledge composition.
 *     [x] It does not duplicate universal knowledge syntax.
 *     [x] It imports KnowledgeExpressions.
 *     [x] It exports aiKnowledgeConstruct.
 *     [x] It introduces no lexer rules.
 *     [x] It introduces no Rust code.
 *     [x] It introduces no runtime behavior.
 *     [x] It introduces no artificial scaling ceiling.
 *     [x] It does not select hardware.
 *     [x] It does not create an AI-specific IR.
 *     [x] It preserves the universal knowledge AST boundary.
 *     [x] It integrates with reasoning.
 *     [x] It integrates with learning.
 *     [x] It integrates with adaptation.
 *     [x] It integrates with data.
 *     [x] It integrates with quantum semantics without redefining them.
 *     [x] It integrates with provenance.
 *     [x] It integrates with policy/security.
 *     [x] It has a defined AI composition consumer.
 *     [x] Its tests can be maintained independently.
 *
 * ============================================================================
 * CANONICAL IMPORT
 * ============================================================================
 */

parser grammar AIKnowledge;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * CANONICAL KNOWLEDGE SOURCE
 * ============================================================================
 *
 * Universal knowledge syntax is owned by:
 *
 *     grammar/expressions/knowledge.g4
 *
 * Grammar:
 *
 *     KnowledgeExpressions
 *
 * Importing it here is deliberate.
 *
 * DO NOT replace this import with a second implementation of the knowledge
 * grammar.
 */

import KnowledgeExpressions;


/*
 * ============================================================================
 * PUBLIC AI KNOWLEDGE BOUNDARY
 * ============================================================================
 *
 * This is the ONLY rule owned by this file.
 *
 * It delegates completely to the universal knowledge expression boundary.
 *
 * This gives grammar/ai/ai.g4 a stable AI-domain integration point while
 * retaining grammar/expressions/knowledge.g4 as the single source-level
 * knowledge syntax authority.
 */

aiKnowledgeConstruct
    : knowledgeExpression
    ;


/*
 * ============================================================================
 * ARCHITECTURAL INVARIANT
 * ============================================================================
 *
 * The following intentionally do NOT exist in this grammar:
 *
 *     knowledgeAssertionExpression
 *     knowledgeRetractionExpression
 *     knowledgeQueryExpression
 *     knowledgeLookupExpression
 *     knowledgeUpdateExpression
 *     knowledgeTerm
 *     knowledgePattern
 *     knowledgeOperationName
 *
 * They remain owned by KnowledgeExpressions.
 *
 * This is essential for avoiding two independently evolving knowledge
 * grammars.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/ai/ai.g4
 * ============================================================================
 *
 * grammar/ai/ai.g4 must import:
 *
 *     AIKnowledge
 *
 * and add:
 *
 *     aiKnowledgeConstruct
 *
 * to its existing aiConstruct alternatives.
 *
 * The intended composition is:
 *
 *     aiConstruct
 *         : aiModelConstruct
 *         | datasetConstruct
 *         | tensorConstruct
 *         | trainingConstruct
 *         | inferenceConstruct
 *         | agentConstruct
 *         | aiDifferentiationConstruct
 *         | pipelineConstruct
 *         | aiAcceleratorConstruct
 *         | deploymentConstruct
 *         | aiCapabilityConstruct
 *         | aiKnowledgeConstruct
 *         ;
 *
 * The exact existing ordering may be retained if the repository's ANTLR
 * composition policy requires another order.
 *
 * No knowledge implementation rules should be copied into ai.g4.
 *
 * ============================================================================
 */
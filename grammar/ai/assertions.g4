/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/ai/assertions.g4
 *
 * GRAMMAR
 * -------
 * AIAssertions
 *
 * STATUS
 * ------
 * CANONICAL AI ASSERTION COMPOSITION / ADAPTER GRAMMAR
 *
 * IMPLEMENTATION BASELINE
 * ------------------------
 * Rust 1.97 or later
 * Rust 2021
 * Safe Rust only
 * No unsafe Rust
 *
 * GRAMMAR TECHNOLOGY
 * ------------------
 * ANTLR4 parser grammar
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file provides the AI-domain composition boundary for assertions.
 *
 * It deliberately does NOT create a second assertion language.
 *
 * Assertions are already owned by the universal statement grammar:
 *
 *     grammar/statements/assertions.g4
 *
 * with grammar identity:
 *
 *     AssertionsParser
 *
 * Knowledge assertions are already owned by the universal expression grammar:
 *
 *     grammar/expressions/knowledge.g4
 *
 * with grammar identity:
 *
 *     KnowledgeExpressions
 *
 * This file connects those canonical constructs to the AI composition layer.
 *
 * The architecture is therefore:
 *
 *     canonical assertion syntax
 *             |
 *             +-----------------------------+
 *             |                             |
 *             v                             v
 *     AssertionsParser             KnowledgeExpressions
 *             |                             |
 *             |                             |
 *             +-------------+---------------+
 *                           |
 *                           v
 *                      AIAssertions
 *                           |
 *                           v
 *                         AI.g4
 *                           |
 *                           v
 *                  domain-neutral AST
 *                           |
 *                           v
 *                   semantic analysis
 *                           |
 *          +----------------+----------------+
 *          |                |                |
 *          v                v                v
 *        types           effects         resources
 *          |                |                |
 *          +----------------+----------------+
 *                           |
 *                           v
 *                    contracts/policies
 *                           |
 *                           v
 *                       provenance
 *                           |
 *                           v
 *                   semantic AI model
 *                           |
 *                           +----------------------+
 *                           |                      |
 *                           v                      v
 *                      classical              quantum::ir
 *                           |                      |
 *                           +----------+-----------+
 *                                      |
 *                                      v
 *                             target-independent
 *                             optimization/lowering
 *                                      |
 *                                      v
 *                              target realization
 *
 * ============================================================================
 * CORE PRINCIPLE
 * ============================================================================
 *
 * An assertion expresses a property, condition, or check.
 *
 * This grammar does not determine:
 *
 *     - whether the assertion is true;
 *     - whether it is statically provable;
 *     - whether it executes at compile time;
 *     - whether it executes at runtime;
 *     - whether it is optimized away;
 *     - whether failure aborts execution;
 *     - whether failure is recoverable;
 *     - whether verification is possible;
 *     - which solver or verifier is used;
 *     - which model is used;
 *     - which CPU is used;
 *     - which GPU is used;
 *     - which FPGA is used;
 *     - which accelerator is used;
 *     - which QPU is used;
 *     - which simulator is used;
 *     - which node executes it;
 *     - which storage system supplies evidence.
 *
 * Those are semantic, policy, resource, capability, verification,
 * compilation, and runtime concerns.
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns ONLY:
 *
 *     aiAssertionConstruct
 *     aiAssertionStatement
 *     aiKnowledgeAssertion
 *
 * These are composition boundaries.
 *
 * It does NOT own the underlying assertion or knowledge assertion syntax.
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     assertionStatement
 *     assertionCondition
 *     assertionExplanation
 *
 * Those belong to:
 *
 *     grammar/statements/assertions.g4
 *
 * This file also does NOT own:
 *
 *     knowledgeExpression
 *     knowledgeAssertionExpression
 *     knowledgeRetractionExpression
 *     knowledgeQueryExpression
 *     knowledgeLookupExpression
 *     knowledgeUpdateExpression
 *
 * Those belong to:
 *
 *     grammar/expressions/knowledge.g4
 *
 * It also does NOT own:
 *
 *     facts
 *     factPattern
 *     factSubject
 *     factRelation
 *     factObject
 *
 * Those belong to:
 *
 *     grammar/ai/facts.g4
 *
 * It does NOT own:
 *
 *     reasoning
 *     inference
 *     deduction
 *     induction
 *     abduction
 *     learning
 *     adaptation
 *     uncertainty
 *     probability
 *     evidence
 *     provenance
 *     explanations
 *     policies
 *     contracts
 *     effects
 *     capabilities
 *     resources
 *     models
 *     datasets
 *     tensors
 *     agents
 *     quantum operations
 *     HDL syntax
 *     hardware syntax
 *     distributed execution
 *     networking
 *     FFI
 *     ABI
 *     metaprogramming
 *     IR
 *     runtime behavior
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There must be exactly one source-level owner for ordinary assertions:
 *
 *     grammar/statements/assertions.g4
 *
 * There must be exactly one source-level owner for knowledge assertions:
 *
 *     grammar/expressions/knowledge.g4
 *
 * This file MUST NOT reproduce their productions.
 *
 * In particular, this file must never contain a second implementation of:
 *
 *     ASSERT LPAREN ...
 *
 * or:
 *
 *     KeywordKnowledge KeywordAssert LPAREN ...
 *
 * The AI layer merely exposes those canonical constructs through a stable
 * composition boundary.
 *
 * ============================================================================
 * WHY AN AI ASSERTION ADAPTER EXISTS
 * ============================================================================
 *
 * Assertions are universal language constructs.
 *
 * They can nevertheless be consumed by AI-oriented semantic analysis for:
 *
 *     model validation;
 *     dataset validation;
 *     inference validation;
 *     training invariants;
 *     learned-state validation;
 *     knowledge consistency;
 *     reasoning conditions;
 *     uncertainty constraints;
 *     agent state validation;
 *     policy validation;
 *     provenance validation;
 *     hybrid computation;
 *     quantum-derived results;
 *     simulation results;
 *     hardware observations.
 *
 * The adapter allows those consumers to use the canonical assertion language
 * without moving assertion ownership into the AI subsystem.
 *
 * ============================================================================
 * ARCHITECTURAL DISTINCTION
 * ============================================================================
 *
 * There are two intentionally different constructs:
 *
 *     assert(condition);
 *
 * and:
 *
 *     knowledge assert(subject, relation [, object]);
 *
 * The first is a program assertion.
 *
 * The second is a knowledge operation.
 *
 * They may both be relevant to AI semantics, but they have different AST
 * meanings and different semantic ownership.
 *
 * This file preserves that distinction.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * REQUIRED PARSER DEPENDENCIES
 * ----------------------------
 *
 *     grammar/statements/assertions.g4
 *         |
 *         +--> AssertionsParser
 *         +--> assertionStatement
 *
 *     grammar/expressions/knowledge.g4
 *         |
 *         +--> KnowledgeExpressions
 *         +--> knowledgeAssertionExpression
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This grammar consumes:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * No lexer rules are defined here.
 *
 * This file MUST NOT define:
 *
 *     ASSERT
 *     KeywordAssert
 *     KeywordKnowledge
 *     punctuation
 *     identifiers
 *     operators
 *     literals
 *
 * If lexical vocabulary changes, the canonical lexer and lexical
 * specification must be updated first.
 *
 * This adapter should then continue to consume the canonical public parser
 * rules without recreating their lexical definitions.
 *
 * ============================================================================
 * GRAMMAR DEPENDENCY DIRECTION
 * ============================================================================
 *
 * Correct:
 *
 *     ZamaniLexer
 *          |
 *          v
 *     canonical leaf grammar
 *          |
 *          v
 *     AIAssertions
 *          |
 *          v
 *     AI
 *
 * Incorrect:
 *
 *     AIAssertions
 *          |
 *          v
 *     private assertion syntax
 *
 * or:
 *
 *     AIAssertions
 *          |
 *          v
 *     private knowledge syntax
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * The only parser grammars imported by this file are:
 *
 *     AssertionsParser
 *     KnowledgeExpressions
 *
 * This intentionally keeps the adapter narrow.
 *
 * `AIFacts` is NOT imported because this file does not expose bare facts as
 * assertions.
 *
 * `AIReasoning` is NOT imported because reasoning is a separate AI capability.
 *
 * `AIEvidence` is NOT imported because evidence is a semantic relationship
 * associated with assertions rather than a replacement for assertion syntax.
 *
 * `AIProvenance` is NOT imported because provenance remains a semantic
 * tracking subsystem.
 *
 * ============================================================================
 * PUBLIC API
 * ============================================================================
 *
 * Public rule:
 *
 *     aiAssertionConstruct
 *
 * This is the rule AI.g4 should consume.
 *
 * Additional public aliases:
 *
 *     aiAssertionStatement
 *     aiKnowledgeAssertion
 *
 * These aliases make the semantic distinction explicit for tooling and
 * future composition without duplicating syntax.
 *
 * ============================================================================
 * ASSERTION COMPOSITION
 * ============================================================================
 *
 * The ordinary assertion path is:
 *
 *     aiAssertionStatement
 *         :
 *     assertionStatement
 *         ;
 *
 * Therefore all canonical assertion forms remain defined by:
 *
 *     grammar/statements/assertions.g4
 *
 * The AI adapter does not alter:
 *
 *     assert(condition);
 *
 * or:
 *
 *     assert(condition, explanation);
 *
 * ============================================================================
 * KNOWLEDGE ASSERTION COMPOSITION
 * ============================================================================
 *
 * The knowledge assertion path is:
 *
 *     aiKnowledgeAssertion
 *         :
 *     knowledgeAssertionExpression
 *         ;
 *
 * Therefore knowledge assertion syntax remains defined by:
 *
 *     grammar/expressions/knowledge.g4
 *
 * The AI adapter does not alter:
 *
 *     knowledge assert(...)
 *
 * semantics or syntax.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This file creates no AST types.
 *
 * The frontend AST remains domain-neutral.
 *
 * The ordinary assertion path must preserve the canonical assertion
 * structure:
 *
 *     condition
 *     explanation?
 *     source span
 *
 * The knowledge assertion path must preserve the canonical knowledge
 * operation structure:
 *
 *     subject
 *     relation
 *     optional object
 *     operation options/tails
 *     source span
 *
 * The AI adapter itself must not introduce:
 *
 *     AIAssertionNode
 *     AIKnowledgeAssertionNode
 *
 * merely because these constructs are consumed by AI semantics.
 *
 * Existing domain-neutral AST ownership remains downstream.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines what an AI assertion means.
 *
 * For an ordinary assertion it may determine:
 *
 *     predicate type;
 *     name resolution;
 *     value availability;
 *     effect legality;
 *     capability requirements;
 *     resource requirements;
 *     policy constraints;
 *     contract interaction;
 *     verification requirements;
 *     provenance;
 *     execution behavior.
 *
 * For a knowledge assertion it may determine:
 *
 *     knowledge subject;
 *     relation;
 *     object;
 *     relation arity;
 *     type compatibility;
 *     knowledge scope;
 *     evidence;
 *     provenance;
 *     confidence;
 *     uncertainty;
 *     policy;
 *     authorization;
 *     persistence requirements;
 *     provider requirements.
 *
 * Parser acceptance is not semantic acceptance.
 *
 * ============================================================================
 * AI SEMANTIC INTEGRATION
 * ============================================================================
 *
 * AI semantic analysis may use assertions for:
 *
 *     model validity;
 *     dataset validity;
 *     tensor constraints;
 *     training invariants;
 *     inference requirements;
 *     knowledge consistency;
 *     reasoning premises;
 *     learning conditions;
 *     adaptation guards;
 *     agent state conditions;
 *     policy conditions;
 *     confidence requirements;
 *     provenance requirements.
 *
 * These meanings are semantic.
 *
 * This grammar provides only the parser composition boundary.
 *
 * ============================================================================
 * KNOWLEDGE INTEGRATION
 * ============================================================================
 *
 * Knowledge assertions must remain distinguishable from ordinary assertions.
 *
 * Conceptual flow:
 *
 *     knowledgeAssertionExpression
 *             |
 *             v
 *     domain-neutral AST
 *             |
 *             v
 *     semantic knowledge assertion
 *             |
 *        +----+----+
 *        |         |
 *        v         v
 *     knowledge  provenance
 *        |
 *        +----------------+
 *        |                |
 *        v                v
 *     reasoning        learning
 *
 * An AI knowledge assertion does not directly execute a database operation.
 *
 * Storage, provider selection, indexing, persistence, and distributed
 * realization remain downstream.
 *
 * ============================================================================
 * REASONING INTEGRATION
 * ============================================================================
 *
 * Assertions may provide premises or verification conditions to reasoning.
 *
 * Example conceptual composition:
 *
 *     assert(condition);
 *
 *     infer conclusion from evidence;
 *
 * The reasoning grammar remains owned by:
 *
 *     grammar/statements/reason.g4
 *
 * This file does not import or duplicate reasoning syntax.
 *
 * Semantic analysis may connect:
 *
 *     assertion
 *         |
 *         v
 *     claim/property
 *         |
 *         v
 *     reasoning/evidence
 *
 * ============================================================================
 * EVIDENCE INTEGRATION
 * ============================================================================
 *
 * An assertion may become a claim requiring evidence.
 *
 * The evidence subsystem determines:
 *
 *     evidence availability;
 *     evidence source;
 *     evidence quality;
 *     confidence;
 *     verification status;
 *     provenance;
 *     trust;
 *     policy compliance.
 *
 * This grammar does not invent:
 *
 *     evidence(...)
 *
 * as an assertion-specific syntax.
 *
 * Existing evidence syntax and semantics remain owned by the evidence
 * subsystem.
 *
 * ============================================================================
 * PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Every parsed assertion retains source structure and source span so semantic
 * infrastructure can record provenance.
 *
 * Possible semantic provenance relationships include:
 *
 *     asserted_from
 *     derived_from
 *     verified_by
 *     supported_by
 *     rejected_by
 *     generated_by
 *     transformed_by
 *
 * The grammar does not generate provenance records.
 *
 * ============================================================================
 * UNCERTAINTY INTEGRATION
 * ============================================================================
 *
 * Assertions may contain expressions whose values are:
 *
 *     probabilities;
 *     distributions;
 *     confidence values;
 *     uncertain values;
 *     observations;
 *     model outputs;
 *     measurement results.
 *
 * The assertion grammar remains unchanged.
 *
 * The semantic type system determines whether the expression is a valid
 * predicate or assertion value.
 *
 * No fixed probability representation is embedded here.
 *
 * ============================================================================
 * LEARNING INTEGRATION
 * ============================================================================
 *
 * Assertions may constrain learning-related computation.
 *
 * Examples of semantic use include:
 *
 *     training data validity;
 *     model invariants;
 *     convergence conditions;
 *     validation properties;
 *     reproducibility conditions;
 *     resource requirements.
 *
 * Learning syntax remains owned by:
 *
 *     grammar/ai/training.g4
 *
 * This file does not import or redefine training syntax.
 *
 * ============================================================================
 * ADAPTATION INTEGRATION
 * ============================================================================
 *
 * Assertions may guard adaptive behavior.
 *
 * Example semantic relationship:
 *
 *     assertion
 *         |
 *         v
 *     condition
 *         |
 *         v
 *     adaptation policy
 *
 * Adaptation itself remains subject to:
 *
 *     policy;
 *     authorization;
 *     capabilities;
 *     effects;
 *     resources;
 *     provenance;
 *     validation.
 *
 * An assertion must never implicitly grant permission to adapt program,
 * model, execution, or hardware state.
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Ordinary assertions are distinct from contracts.
 *
 * Contract syntax remains owned by:
 *
 *     grammar/statements/contract.g4
 *
 * including concepts such as:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * AI assertions may be semantically consumed by contract verification, but
 * this grammar must not duplicate those productions.
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Assertions may participate in policy evaluation.
 *
 * Policy semantics may determine:
 *
 *     whether an assertion may execute;
 *     whether evidence may be accessed;
 *     whether knowledge may be modified;
 *     whether a model may be evaluated;
 *     whether a verification capability may be used;
 *     whether external providers may be contacted.
 *
 * This grammar does not enforce policy.
 *
 * Parser acceptance never constitutes authorization.
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * Assertions themselves have no grammar-defined effect.
 *
 * Semantic analysis determines effects arising from the condition or
 * explanation expressions.
 *
 * For example, an assertion expression may semantically reference:
 *
 *     IO;
 *     network;
 *     measurement;
 *     randomness;
 *     native;
 *     foreign;
 *     distributed;
 *     learning;
 *     adaptation;
 *     reflection;
 *     simulation.
 *
 * The effect subsystem remains authoritative.
 *
 * ============================================================================
 * CAPABILITY INTEGRATION
 * ============================================================================
 *
 * An assertion may semantically require capabilities.
 *
 * Examples:
 *
 *     verification capability;
 *     model evaluation capability;
 *     quantum measurement capability;
 *     tensor computation capability;
 *     hardware observation capability.
 *
 * Capability names remain open-world.
 *
 * This grammar does not enumerate capability names.
 *
 * Capability resolution remains downstream.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Assertion syntax does not allocate resources.
 *
 * Semantic analysis may determine that evaluating an assertion requires:
 *
 *     computation;
 *     memory;
 *     storage;
 *     communication;
 *     accelerator capability;
 *     quantum resources;
 *     verification resources.
 *
 * Resource requirements remain symbolic and target independent.
 *
 * Actual realization is selected downstream from available resources and
 * capabilities.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * The canonical assertion expression may reference any valid classical
 * expression.
 *
 * No separate:
 *
 *     AIAssert
 *     CPUAssert
 *     GPUAssert
 *
 * syntax is required.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Assertions may reference quantum-derived values.
 *
 * Examples include semantic forms equivalent to:
 *
 *     assert(measurement_result);
 *
 *     assert(classical_condition_from_measurement);
 *
 *     knowledge assert(state, relation, value);
 *
 * The assertion grammar does not:
 *
 *     enumerate gates;
 *     enumerate qubits;
 *     assign physical qubits;
 *     define quantum circuits;
 *     define routing;
 *     define scheduling;
 *     define QEC;
 *     define ZQN;
 *     define HAL;
 *     select QPUs.
 *
 * If the assertion's semantic computation contains quantum operations, the
 * established quantum path remains:
 *
 *     AST
 *       ->
 *     quantum semantic model
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
 *     resilience / QEC
 *       ->
 *     ZQN
 *       ->
 *     HAL
 *       ->
 *     target realization
 *
 * ============================================================================
 * HYBRID INTEGRATION
 * ============================================================================
 *
 * The same assertion may validate:
 *
 *     classical values;
 *     quantum-derived values;
 *     model outputs;
 *     tensor results;
 *     hardware observations;
 *     simulation results;
 *     distributed results.
 *
 * No separate hybrid assertion syntax is required.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Assertions may semantically validate:
 *
 *     HDL properties;
 *     simulation outputs;
 *     hardware state;
 *     timing-related values;
 *     resource observations;
 *     verification results.
 *
 * This grammar does not define hardware syntax.
 *
 * Hardware realization remains downstream.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Assertions may reference distributed state or results.
 *
 * The grammar imposes no limit on:
 *
 *     participants;
 *     messages;
 *     nodes;
 *     devices;
 *     resources;
 *     topology;
 *     replicated state.
 *
 * Distributed semantics remain owned by the distributed subsystem.
 *
 * ============================================================================
 * DATA INTEGRATION
 * ============================================================================
 *
 * Assertion expressions may reference:
 *
 *     data values;
 *     records;
 *     collections;
 *     schemas;
 *     query results;
 *     structured interchange values.
 *
 * SQL, JSON, XML, graph-query, and other external formats remain owned by
 * their respective dialect/interoperability grammars.
 *
 * ============================================================================
 * FFI / ABI INTEGRATION
 * ============================================================================
 *
 * An assertion condition may reference values returned by foreign functions.
 *
 * FFI and ABI semantics remain owned by:
 *
 *     grammar/interoperability/
 *
 * This grammar does not define:
 *
 *     calling conventions;
 *     ABI layouts;
 *     foreign type layouts;
 *     native addresses;
 *     library loading.
 *
 * ============================================================================
 * METAPROGRAMMING INTEGRATION
 * ============================================================================
 *
 * Assertions may participate in compile-time validation or generated
 * verification artifacts.
 *
 * Metaprogramming remains owned by:
 *
 *     grammar/metaprogramming/
 *
 * Generated assertions must preserve source and generation provenance
 * downstream.
 *
 * ============================================================================
 * SIMULATION INTEGRATION
 * ============================================================================
 *
 * Assertions may validate simulation results.
 *
 * Simulation may represent:
 *
 *     classical systems;
 *     quantum systems;
 *     hybrid systems;
 *     HDL systems;
 *     hardware;
 *     distributed systems;
 *     AI systems;
 *     future computational systems.
 *
 * Simulation mode is an execution concern, not an assertion grammar concern.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * This file is completely target independent.
 *
 * Identical assertion source may be compiled for:
 *
 *     tiny embedded environments;
 *     CPUs;
 *     multicore systems;
 *     GPUs;
 *     FPGAs;
 *     ASIC-oriented systems;
 *     accelerators;
 *     QPUs;
 *     quantum simulators;
 *     HPC systems;
 *     clusters;
 *     distributed environments;
 *     cloud environments;
 *     future computational architectures.
 *
 * The assertion expresses a property of computation.
 *
 * It does not express a mandatory physical implementation.
 *
 * If a target lacks a capability or resource required to evaluate an
 * assertion, that is a downstream feasibility result.
 *
 * The source grammar remains unchanged.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar introduces NO language-level finite limits for:
 *
 *     assertions;
 *     knowledge assertions;
 *     facts;
 *     expression size;
 *     nesting;
 *     source size;
 *     program size;
 *     models;
 *     datasets;
 *     tensors;
 *     tensor rank;
 *     agents;
 *     nodes;
 *     processors;
 *     accelerators;
 *     devices;
 *     qubits;
 *     memory;
 *     storage;
 *     network size;
 *     topology size.
 *
 * In particular, this file contains no concepts equivalent to:
 *
 *     MAX_ASSERTIONS
 *     MAX_FACTS
 *     MAX_MODELS
 *     MAX_DATASETS
 *     MAX_TENSOR_RANK
 *     MAX_AGENTS
 *     MAX_NODES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * Practical limits may exist in:
 *
 *     parser implementation;
 *     compiler resources;
 *     verifier resources;
 *     runtime resources;
 *     target resources;
 *     explicit user policies.
 *
 * Such limits MUST NOT become hidden language constants.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing is deterministic.
 *
 * Parsing must depend only on:
 *
 *     source;
 *     canonical lexer;
 *     imported grammar definitions;
 *     parser configuration.
 *
 * Parsing must not depend on:
 *
 *     hardware;
 *     runtime state;
 *     resource availability;
 *     network state;
 *     filesystem state;
 *     wall-clock time;
 *     randomness;
 *     scheduler state;
 *     provider state.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Parsing an AI assertion must never:
 *
 *     execute a model;
 *     execute a knowledge provider;
 *     access a database;
 *     access a network;
 *     access a file;
 *     access credentials;
 *     inspect hardware;
 *     allocate devices;
 *     mutate knowledge;
 *     mutate model state;
 *     mutate program state.
 *
 * Authorization, sandboxing, trust, and capability checks occur downstream.
 *
 * ============================================================================
 * DIAGNOSTICS CONTRACT
 * ============================================================================
 *
 * Parser diagnostics are restricted to malformed syntax originating from the
 * canonical imported grammars.
 *
 * Semantic diagnostics may include:
 *
 *     invalid predicate type;
 *     unresolved symbol;
 *     invalid knowledge subject;
 *     invalid knowledge relation;
 *     invalid knowledge object;
 *     invalid operation;
 *     unavailable verification capability;
 *     insufficient resource;
 *     forbidden effect;
 *     policy violation;
 *     invalid evidence;
 *     invalid provenance;
 *     untrusted source;
 *     contradictory knowledge;
 *     unverifiable claim.
 *
 * Such diagnostics are semantic diagnostics and MUST NOT be encoded as
 * parser-specific alternatives.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This file introduces no new lexical keyword.
 *
 * Existing source forms remain owned by their canonical grammars.
 *
 * Therefore:
 *
 *     assert(...)
 *
 * continues to mean the canonical assertion statement.
 *
 *     knowledge assert(...)
 *
 * continues to mean the canonical knowledge assertion expression.
 *
 * This adapter does not reinterpret existing syntax.
 *
 * Legacy syntax must be handled by:
 *
 *     grammar/compatibility/
 *
 * rather than by adding ambiguous alternatives here.
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * This grammar is a parser grammar.
 *
 * It uses:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * and imports:
 *
 *     AssertionsParser
 *     KnowledgeExpressions
 *
 * The public adapter rule is:
 *
 *     aiAssertionConstruct
 *
 * The expected AI composition is:
 *
 *     AI
 *       |
 *       +--> aiAssertionConstruct
 *               |
 *               +--> aiAssertionStatement
 *               |       |
 *               |       +--> assertionStatement
 *               |
 *               +--> aiKnowledgeAssertion
 *                       |
 *                       +--> knowledgeAssertionExpression
 *
 * The canonical AI composition grammar should import:
 *
 *     AIAssertions
 *
 * and include:
 *
 *     aiAssertionConstruct
 *
 * in `aiConstruct` only if AI-specific composition needs an explicit
 * assertion boundary.
 *
 * This file must never replace the universal statement dispatcher.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/ai/ai.g4
 * ============================================================================
 *
 * REQUIRED CHANGE OUTSIDE THIS FILE:
 *
 * Add:
 *
 *     AIAssertions
 *
 * to the canonical AI grammar imports.
 *
 * Then add:
 *
 *     | aiAssertionConstruct
 *
 * to:
 *
 *     aiConstruct
 *
 * The relevant architecture becomes:
 *
 *     AI
 *       |
 *       +--> models
 *       +--> datasets
 *       +--> tensors
 *       +--> training
 *       +--> inference
 *       +--> agents
 *       +--> reasoning
 *       +--> knowledge
 *       +--> assertions
 *       +--> differentiation
 *       +--> pipelines
 *       +--> accelerators
 *       +--> deployment
 *
 * No assertion production is copied into AI.g4.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/statements/assertions.g4
 * ============================================================================
 *
 * This file imports:
 *
 *     AssertionsParser
 *
 * and consumes:
 *
 *     assertionStatement
 *
 * The canonical assertion grammar remains the sole owner of:
 *
 *     assertionStatement
 *     assertionCondition
 *     assertionExplanation
 *
 * No modifications to those productions are required for this adapter.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/expressions/knowledge.g4
 * ============================================================================
 *
 * This file imports:
 *
 *     KnowledgeExpressions
 *
 * and consumes:
 *
 *     knowledgeAssertionExpression
 *
 * The canonical knowledge grammar remains the sole owner of:
 *
 *     knowledgeExpression
 *     knowledgeAssertionExpression
 *     knowledgeRetractionExpression
 *     knowledgeQueryExpression
 *     knowledgeLookupExpression
 *     knowledgeUpdateExpression
 *
 * No knowledge syntax is duplicated here.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/ai/knowledge.g4
 * ============================================================================
 *
 * AIKnowledge already provides:
 *
 *     aiKnowledgeConstruct
 *
 * as a composition boundary for:
 *
 *     knowledgeExpression
 *
 * This assertion adapter deliberately does not replace AIKnowledge.
 *
 * Instead:
 *
 *     AIKnowledge
 *          |
 *          v
 *     knowledgeExpression
 *          |
 *          +--> knowledgeAssertionExpression
 *
 * and:
 *
 *     AIAssertions
 *          |
 *          v
 *     knowledgeAssertionExpression
 *
 * are both valid composition views of the same canonical knowledge syntax.
 *
 * If the AI composition grammar imports both adapters, semantic analysis must
 * deduplicate them by canonical AST/source identity rather than treating them
 * as different language constructs.
 *
 * To avoid duplicate parser alternatives where the enclosing AI grammar cannot
 * distinguish them, the preferred composition is:
 *
 *     AIAssertions
 *         -> ordinary assertionStatement
 *
 * while AIKnowledge remains responsible for knowledgeExpression.
 *
 * In that preferred arrangement, `aiKnowledgeAssertion` remains available as
 * a stable reusable boundary for tooling or future domain composition but
 * should not be added as a second identical alternative beside
 * `aiKnowledgeConstruct` unless the surrounding grammar requires it.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/ai/facts.g4
 * ============================================================================
 *
 * Facts remain separate from assertions.
 *
 * `AIFacts` owns:
 *
 *     fact
 *     factPattern
 *     factTerm
 *     factRelation
 *     factMetadata
 *     factQualifier
 *
 * This adapter does not import or reproduce those structures.
 *
 * Knowledge assertion semantics may consume fact structures downstream.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/ai/evidence.g4
 * ============================================================================
 *
 * Evidence may support an assertion.
 *
 * Evidence syntax and evidence semantics remain evidence-owned.
 *
 * The semantic pipeline is:
 *
 *     assertion
 *        |
 *        v
 *     claim/property
 *        |
 *        v
 *     evidence relation
 *        |
 *        v
 *     verification
 *
 * This file does not introduce evidence-specific assertion syntax.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/ai/provenance.g4
 * ============================================================================
 *
 * Provenance records assertion origin and transformations.
 *
 * This file supplies source structure indirectly through the canonical
 * assertion/knowledge parse trees.
 *
 * Provenance semantics remain downstream.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/ai/reasoning.g4
 * ============================================================================
 *
 * Reasoning may consume assertion-derived claims.
 *
 * No import is required merely to establish this semantic relationship.
 *
 * Reasoning syntax remains owned by:
 *
 *     grammar/statements/reason.g4
 *
 * and composed by:
 *
 *     grammar/ai/reasoning.g4
 *
 * ============================================================================
 * INTEGRATION WITH grammar/validation/
 * ============================================================================
 *
 * Validation is a downstream semantic consumer.
 *
 * In particular:
 *
 *     grammar/validation/assertions.g4
 *
 * remains a validation facade over the canonical assertion grammar.
 *
 * AIAssertions must not become another validation implementation.
 *
 * The relationship is:
 *
 *     AIAssertions
 *          |
 *          v
 *     canonical assertion syntax
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     validation
 *
 * ============================================================================
 * INTEGRATION WITH grammar/effects/
 * ============================================================================
 *
 * Effects are derived downstream from assertion expressions and referenced
 * operations.
 *
 * This file does not declare effect syntax.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/resources/
 * ============================================================================
 *
 * Resource requirements are derived downstream.
 *
 * This file does not encode resource capacities.
 *
 * ============================================================================
 * INTEGRATION WITH grammar/policies/
 * ============================================================================
 *
 * Policy evaluation remains downstream.
 *
 * This file does not authorize an assertion merely because it parses.
 *
 * ============================================================================
 * INTEGRATION WITH CANONICAL AST
 * ============================================================================
 *
 * Existing domain-neutral frontend AST remains authoritative.
 *
 * Required preservation:
 *
 *     source span
 *     construct kind
 *     condition
 *     explanation
 *     knowledge operation structure
 *     source ordering
 *
 * The AI adapter must not require an AI-specific AST hierarchy.
 *
 * ============================================================================
 * INTEGRATION WITH SEMANTIC MODEL
 * ============================================================================
 *
 * Semantic analysis maps the canonical AST into semantic concepts such as:
 *
 *     Assertion
 *     Claim
 *     KnowledgeAssertion
 *     VerificationObligation
 *     EvidenceRequirement
 *     ProvenanceRelation
 *
 * Exact Rust type names remain owned by the implementation.
 *
 * ============================================================================
 * INTEGRATION WITH IR
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * Assertion semantics enter the canonical semantic representation first.
 *
 * Possible downstream destinations include:
 *
 *     classical IR;
 *     tensor/data computation;
 *     distributed computation;
 *     HDL/hardware verification;
 *     quantum::ir;
 *     accelerator representation;
 *     future domain IR.
 *
 * There must not be:
 *
 *     AIAssertionIR
 *     AIKnowledgeAssertionIR
 *     AIQuantumAssertionIR
 *
 * created merely because this file exists.
 *
 * ============================================================================
 * QUANTUM IR BOUNDARY
 * ============================================================================
 *
 * If an assertion's semantic computation requires quantum operations:
 *
 *     domain-neutral AST
 *          |
 *          v
 *     semantic model
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     decomposition
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
 *          |
 *          v
 *     target
 *
 * AIAssertions does not modify this boundary.
 *
 * ============================================================================
 * TOOLING CONTRACT
 * ============================================================================
 *
 * FORMATTER
 * ---------
 *
 * The formatter must use canonical AST structure.
 *
 * It must not format AI assertions according to a second grammar.
 *
 * LSP
 * ---
 *
 * Symbol resolution, completion, hover, references, and diagnostics must use
 * semantic information rather than an AI-specific assertion symbol table.
 *
 * SYNTAX HIGHLIGHTING
 * -------------------
 *
 * Highlighting may identify:
 *
 *     assert
 *     knowledge assertion structure
 *
 * according to the canonical lexer/parser vocabulary.
 *
 * REFACTORING
 * -----------
 *
 * Renaming symbols inside an assertion must use canonical semantic name
 * resolution.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * This file is independently testable as an adapter grammar.
 *
 * POSITIVE TESTS
 * --------------
 *
 * Ordinary assertion:
 *
 *     assert(result.is_valid());
 *
 * Ordinary assertion with explanation:
 *
 *     assert(result.is_valid(), "result must be valid");
 *
 * AI/model-oriented expression:
 *
 *     assert(model.is_valid());
 *
 * Knowledge assertion:
 *
 *     knowledge assert(subject, relation);
 *
 * Knowledge assertion with object:
 *
 *     knowledge assert(subject, relation, object);
 *
 * Knowledge assertion using canonical expression terms:
 *
 *     knowledge assert(model, relation, result);
 *
 * Cross-domain semantic examples:
 *
 *     assert(measurement_result.is_valid());
 *
 *     assert(hardware_state.is_consistent());
 *
 *     assert(distributed_state.is_consistent());
 *
 *     assert(simulation_result.is_valid());
 *
 *     assert(tensor_result.is_valid());
 *
 * Exact semantic validity of these expressions is determined downstream.
 *
 * ============================================================================
 * NEGATIVE TESTS
 * ============================================================================
 *
 * The adapter must not make malformed canonical assertions valid.
 *
 * Examples:
 *
 *     assert
 *     assert()
 *     assert(
 *     assert(condition,
 *
 *     knowledge assert
 *     knowledge assert()
 *     knowledge assert(subject)
 *
 * where the canonical knowledge grammar requires more structure.
 *
 * The exact negative set must follow the canonical imported grammar contracts.
 *
 * The adapter must not accept invented syntax such as:
 *
 *     aiAssert(condition);
 *     modelAssert(condition);
 *     neuralAssert(condition);
 *     quantumAssert(condition);
 *     proofAssert(condition);
 *
 * merely because the source construct is consumed by AI semantics.
 *
 * ============================================================================
 * BOUNDARY TESTS
 * ============================================================================
 *
 * Test:
 *
 *     nested expressions;
 *     deeply nested expressions;
 *     qualified names;
 *     Unicode identifiers accepted by the canonical lexer;
 *     generic values;
 *     structured values;
 *     tensor expressions;
 *     model expressions;
 *     dataset expressions;
 *     probabilistic expressions;
 *     uncertain values;
 *     knowledge expressions;
 *     quantum-derived values;
 *     hybrid values;
 *     HDL-derived values;
 *     hardware-derived values;
 *     distributed values;
 *     simulation results;
 *     foreign-function results;
 *     generated values.
 *
 * The assertion adapter must not change behavior as these domains expand.
 *
 * ============================================================================
 * SCALABILITY TESTS
 * ============================================================================
 *
 * Increase independently:
 *
 *     assertion count;
 *     expression complexity;
 *     expression nesting;
 *     source-unit size;
 *     module size;
 *     program size;
 *     knowledge complexity;
 *     model complexity;
 *     tensor complexity;
 *     cross-domain composition.
 *
 * No scalability test may establish a language-level maximum.
 *
 * Any practical failure must be classified as:
 *
 *     implementation resource;
 *     compiler resource;
 *     verifier resource;
 *     runtime resource;
 *     target resource;
 *     explicit policy.
 *
 * It must not be converted into a grammar constant.
 *
 * ============================================================================
 * DETERMINISM TESTS
 * ============================================================================
 *
 * Parse identical source repeatedly using identical:
 *
 *     lexer version;
 *     grammar version;
 *     parser configuration;
 *     compatibility configuration.
 *
 * Verify equivalent:
 *
 *     token sequence;
 *     parse tree;
 *     source spans;
 *     selected alternatives.
 *
 * ============================================================================
 * PORTABILITY TESTS
 * ============================================================================
 *
 * Parse identical assertion source while varying external:
 *
 *     CPU inventories;
 *     GPU inventories;
 *     FPGA inventories;
 *     accelerator inventories;
 *     QPU inventories;
 *     node inventories;
 *     memory availability;
 *     topology;
 *     execution providers;
 *     simulator availability.
 *
 * Parser structure must remain unchanged.
 *
 * ============================================================================
 * CROSS-DOMAIN TESTS
 * ============================================================================
 *
 * Assertions must compose with:
 *
 *     classical;
 *     numerical;
 *     scientific;
 *     AI;
 *     knowledge;
 *     reasoning;
 *     learning;
 *     adaptation;
 *     uncertainty;
 *     probability;
 *     quantum;
 *     hybrid;
 *     HDL;
 *     hardware;
 *     distributed;
 *     networking;
 *     data;
 *     tensors;
 *     accelerators;
 *     simulation;
 *     interoperability;
 *     metaprogramming.
 *
 * No domain gets a private assertion syntax.
 *
 * ============================================================================
 * SECURITY TESTS
 * ============================================================================
 *
 * Parsing must not:
 *
 *     execute assertions;
 *     execute models;
 *     access knowledge providers;
 *     access databases;
 *     access files;
 *     access networks;
 *     access hardware;
 *     allocate resources;
 *     mutate state.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
 *
 *     machine-capacity constants;
 *     assertion-count constants;
 *     fact-count constants;
 *     model-count constants;
 *     dataset-count constants;
 *     tensor-rank limits;
 *     processor-count limits;
 *     accelerator-count limits;
 *     node-count limits;
 *     device-count limits;
 *     qubit-count limits;
 *     memory limits;
 *     register-width limits;
 *     network-size limits;
 *     topology limits;
 *     vendor catalogues;
 *     hardware catalogues;
 *     model catalogues;
 *     algorithm catalogues;
 *     quantum-gate catalogues.
 *
 * In particular, this file does not introduce any language-level physical
 * capacity ceiling.
 *
 * ============================================================================
 * SAFE RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust implementation.
 *
 * Generated parser/frontend integration must remain compatible with:
 *
 *     Rust 1.97+
 *     Rust 2021
 *     safe Rust only
 *
 * The consuming implementation must not require:
 *
 *     unsafe blocks;
 *     unsafe functions;
 *     unsafe traits;
 *     raw-pointer based APIs;
 *     target-specific unsafe operations.
 *
 * This grammar itself cannot enforce Rust safety, so repository CI must enforce
 * the corresponding implementation policy.
 *
 * ============================================================================
 * REPOSITORY INTEGRATION CHECKLIST
 * ============================================================================
 *
 * Before declaring this file complete:
 *
 * [ ] grammar/ai/assertions.g4 exists.
 *
 * [ ] Grammar identity is AIAssertions.
 *
 * [ ] tokenVocab is ZamaniLexer.
 *
 * [ ] Only AssertionsParser and KnowledgeExpressions are imported.
 *
 * [ ] assertionStatement is delegated to AssertionsParser.
 *
 * [ ] knowledgeAssertionExpression is delegated to KnowledgeExpressions.
 *
 * [ ] No assertion syntax is duplicated.
 *
 * [ ] No knowledge assertion syntax is duplicated.
 *
 * [ ] No lexer rule is defined.
 *
 * [ ] No expression grammar is duplicated.
 *
 * [ ] No type grammar is duplicated.
 *
 * [ ] No contract grammar is duplicated.
 *
 * [ ] No evidence grammar is duplicated.
 *
 * [ ] No provenance grammar is duplicated.
 *
 * [ ] No reasoning grammar is duplicated.
 *
 * [ ] No learning grammar is duplicated.
 *
 * [ ] No adaptation grammar is duplicated.
 *
 * [ ] No AI-specific AST is introduced.
 *
 * [ ] No AI-specific IR is introduced.
 *
 * [ ] quantum::ir remains the canonical quantum IR boundary.
 *
 * [ ] No hardware selection occurs in the grammar.
 *
 * [ ] No resource selection occurs in the grammar.
 *
 * [ ] No capability resolution occurs in the grammar.
 *
 * [ ] No policy enforcement occurs in the grammar.
 *
 * [ ] No runtime execution occurs in the grammar.
 *
 * [ ] No artificial scalability ceiling exists.
 *
 * [ ] Positive tests exist.
 *
 * [ ] Negative tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Scalability tests exist.
 *
 * [ ] Determinism tests exist.
 *
 * [ ] Portability tests exist.
 *
 * [ ] Cross-domain tests exist.
 *
 * [ ] Security tests exist.
 *
 * [ ] Formatter round-trip tests exist where formatter support exists.
 *
 * [ ] LSP integration uses the canonical AST/semantic model.
 *
 * [ ] AI.g4 imports AIAssertions.
 *
 * [ ] AI.g4 exposes aiAssertionConstruct where required.
 *
 * [ ] Canonical Statements composition remains authoritative.
 *
 * [ ] Canonical KnowledgeExpressions composition remains authoritative.
 *
 * [ ] Rust 1.97+ generation succeeds.
 *
 * [ ] Generated frontend remains safe Rust.
 *
 * ============================================================================
 * IMPORTANT COMPOSITION NOTE
 * ============================================================================
 *
 * `aiAssertionConstruct` contains both:
 *
 *     aiAssertionStatement
 *     aiKnowledgeAssertion
 *
 * This is intentional as an independently reusable adapter boundary.
 *
 * However, the canonical AI composition must avoid exposing the same
 * `knowledgeAssertionExpression` through two indistinguishable alternatives.
 *
 * Therefore:
 *
 *     AIKnowledge
 *
 * remains the preferred owner of the AI-wide knowledge expression boundary.
 *
 * If AI.g4 already exposes:
 *
 *     aiKnowledgeConstruct
 *
 * then it should normally expose ordinary assertions through:
 *
 *     aiAssertionConstruct
 *
 * while retaining knowledge assertions through:
 *
 *     aiKnowledgeConstruct
 *
 * The `aiKnowledgeAssertion` rule in this file exists for:
 *
 *     tooling;
 *     isolated adapter testing;
 *     future composition;
 *     semantic boundary documentation.
 *
 * It must not be used to create duplicate parser alternatives in the
 * top-level AI dispatcher.
 *
 * ============================================================================
 * FUTURE EXTENSION CONTRACT
 * ============================================================================
 *
 * If future Zamani versions introduce additional assertion categories, such
 * as a new language-level verification construct, this file must NOT invent
 * syntax locally.
 *
 * The required promotion path is:
 *
 *     specification
 *          |
 *          v
 *     canonical language owner
 *          |
 *          v
 *     lexer, if genuinely required
 *          |
 *          v
 *     canonical grammar
 *          |
 *          v
 *     AST contract
 *          |
 *          v
 *     semantic contract
 *          |
 *          v
 *     AI adapter
 *          |
 *          v
 *     tests
 *          |
 *          v
 *     compatibility decision
 *
 * This keeps AIAssertions stable when the rest of Zamani evolves.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [ ] It is a parser grammar.
 *
 * [ ] Its grammar name is AIAssertions.
 *
 * [ ] It consumes the canonical ZamaniLexer.
 *
 * [ ] It imports only canonical assertion and knowledge-expression owners.
 *
 * [ ] It owns only AI composition boundaries.
 *
 * [ ] It does not duplicate assertion syntax.
 *
 * [ ] It does not duplicate knowledge syntax.
 *
 * [ ] It does not define bare fact syntax.
 *
 * [ ] It does not define reasoning syntax.
 *
 * [ ] It does not define inference syntax.
 *
 * [ ] It does not define learning syntax.
 *
 * [ ] It does not define adaptation syntax.
 *
 * [ ] It does not define evidence syntax.
 *
 * [ ] It does not define provenance syntax.
 *
 * [ ] It does not define policy syntax.
 *
 * [ ] It does not define contract syntax.
 *
 * [ ] It does not define resource syntax.
 *
 * [ ] It does not define capability syntax.
 *
 * [ ] It does not define expression syntax.
 *
 * [ ] It does not define type syntax.
 *
 * [ ] It does not define quantum syntax.
 *
 * [ ] It does not define HDL syntax.
 *
 * [ ] It does not define hardware syntax.
 *
 * [ ] It does not define runtime behavior.
 *
 * [ ] It does not create an AST.
 *
 * [ ] It does not create an IR.
 *
 * [ ] It preserves domain-neutral AST ownership.
 *
 * [ ] It preserves semantic ownership downstream.
 *
 * [ ] It preserves the canonical quantum::ir boundary.
 *
 * [ ] It is deterministic.
 *
 * [ ] It is target independent.
 *
 * [ ] It is safe-Rust compatible downstream.
 *
 * [ ] It contains no artificial capacity limit.
 *
 * [ ] It supports POCO-REAF.
 *
 * [ ] Its positive tests pass.
 *
 * [ ] Its negative tests pass.
 *
 * [ ] Its boundary tests pass.
 *
 * [ ] Its scalability tests pass.
 *
 * [ ] Its determinism tests pass.
 *
 * [ ] Its portability tests pass.
 *
 * [ ] Its cross-domain tests pass.
 *
 * [ ] Its security tests pass.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL GUARANTEE
 * ============================================================================
 *
 * AIAssertions does NOT become a second assertion language.
 *
 * It establishes only:
 *
 *     canonical assertion syntax
 *              |
 *              v
 *        AI composition
 *
 * and:
 *
 *     canonical knowledge assertion
 *              |
 *              v
 *        AI composition
 *
 * The resulting semantic path remains:
 *
 *     source
 *       ->
 *     canonical lexer
 *       ->
 *     canonical parser
 *       ->
 *     domain-neutral AST
 *       ->
 *     structural validation
 *       ->
 *     semantic analysis
 *       ->
 *     types / effects / capabilities / resources
 *       ->
 *     contracts / policies / evidence / provenance
 *       ->
 *     canonical semantic representation
 *       ->
 *     classical / quantum::ir / HDL / distributed / other domain IR
 *       ->
 *     optimization
 *       ->
 *     lowering
 *       ->
 *     routing / scheduling where applicable
 *       ->
 *     resilience where applicable
 *       ->
 *     target realization
 *
 * This preserves source-level portability and allows assertion semantics to
 * participate in computations ranging from extremely small systems to
 * arbitrarily large resource configurations without embedding physical
 * machine limits into the language grammar.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */

parser grammar AIAssertions;

options {
    tokenVocab = ZamaniLexer;
}

import
    AssertionsParser,
    KnowledgeExpressions
;

/*
 * ============================================================================
 * PUBLIC AI ASSERTION COMPOSITION
 * ============================================================================
 *
 * This is the sole top-level rule owned by this file.
 *
 * It delegates all actual assertion syntax to the canonical owners.
 */
aiAssertionConstruct
    : aiAssertionStatement
    | aiKnowledgeAssertion
    ;

/*
 * ============================================================================
 * ORDINARY ASSERTION ADAPTER
 * ============================================================================
 *
 * Canonical source owner:
 *
 *     grammar/statements/assertions.g4
 *
 * Grammar:
 *
 *     AssertionsParser
 */
aiAssertionStatement
    : assertionStatement
    ;

/*
 * ============================================================================
 * KNOWLEDGE ASSERTION ADAPTER
 * ============================================================================
 *
 * Canonical source owner:
 *
 *     grammar/expressions/knowledge.g4
 *
 * Grammar:
 *
 *     KnowledgeExpressions
 *
 * This rule does not reproduce knowledge assertion syntax.
 */
aiKnowledgeAssertion
    : knowledgeAssertionExpression
    ;
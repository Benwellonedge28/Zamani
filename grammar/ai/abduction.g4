/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/ai/abduction.g4
 *
 * Grammar:
 *     Abduction
 *
 * Status:
 *     CANONICAL AI ABDUCTIVE-REASONING COMPOSITION GRAMMAR
 *
 * Implementation baseline:
 *     Rust 1.97+
 *     Rust edition 2021
 *     Safe Rust only
 *     No unsafe Rust
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file is the AI-domain composition boundary for ABDUCTIVE REASONING.
 *
 * Abduction is treated as a general computational reasoning capability:
 *
 *     observations / evidence
 *             |
 *             v
 *        candidate explanation
 *             |
 *             v
 *          hypothesis
 *
 * The source language expresses the portable intent of the operation.
 *
 * This file does NOT implement:
 *
 *     - an abductive algorithm;
 *     - a theorem prover;
 *     - a Bayesian engine;
 *     - a probabilistic engine;
 *     - a machine-learning framework;
 *     - a neural architecture;
 *     - a symbolic reasoning engine;
 *     - a SAT/SMT solver;
 *     - an optimization algorithm;
 *     - a scoring algorithm;
 *     - a confidence algorithm;
 *     - an evidence verifier;
 *     - a provenance database;
 *     - a resource allocator;
 *     - a hardware selector;
 *     - a quantum router;
 *     - a scheduler;
 *     - a runtime.
 *
 * Those responsibilities belong downstream.
 *
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     Zamani parser
 *          |
 *          v
 *     universal AbductionStatements
 *          |
 *          +------------------------------+
 *          |                              |
 *          v                              v
 *     Statements                      AIAbduction
 *          |                              |
 *          +---------------+--------------+
 *                          |
 *                          v
 *                  domain-neutral AST
 *                          |
 *                          v
 *                  structural validation
 *                          |
 *          +---------------+----------------+
 *          |               |                |
 *          v               v                v
 *        types          effects        provenance
 *          |               |                |
 *          +---------------+----------------+
 *                          |
 *                          v
 *                   semantic abduction
 *                          |
 *          +---------------+----------------+
 *          |               |                |
 *          v               v                v
 *       resources      capabilities      policies
 *          |               |                |
 *          +---------------+----------------+
 *                          |
 *                          v
 *                  canonical semantic model
 *                          |
 *             +------------+-------------+
 *             |            |             |
 *             v            v             v
 *        classical     quantum::ir   HDL/hardware
 *             |            |             |
 *             +------------+-------------+
 *                          |
 *                     optimization
 *                          |
 *                      lowering
 *                          |
 *                  routing/scheduling
 *                          |
 *                 resilience/recovery
 *                          |
 *                     ZQN where needed
 *                          |
 *                     HAL where needed
 *                          |
 *                    target realization
 *
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 * Only the AI-domain composition boundary:
 *
 *     aiAbductionConstruct
 *
 *     aiAbductionStatement
 *
 *
 * THIS FILE DOES NOT OWN
 * ---------------------
 *
 *     lexer rules
 *     keyword spelling
 *     identifiers
 *     qualified names
 *     expressions
 *     expression precedence
 *     types
 *     contracts
 *     requirements
 *     capabilities
 *     constraints
 *     preferences
 *     hints
 *     effects
 *     policies
 *     provenance
 *     evidence representation
 *     uncertainty representation
 *     knowledge representation
 *     learning
 *     adaptation
 *     deduction
 *     induction
 *     inference
 *     agents
 *     concurrency
 *     distributed execution
 *     quantum syntax
 *     HDL syntax
 *     hardware syntax
 *     resources
 *     scheduling
 *     routing
 *     optimization
 *     IR
 *     runtime execution
 *
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * The universal source syntax of abduction belongs to:
 *
 *     grammar/statements/abduction.g4
 *
 * This file MUST NOT duplicate that syntax.
 *
 * The reason is architectural:
 *
 *     abduction is not inherently an AI-only operation.
 *
 * It may be used by:
 *
 *     classical computation
 *     scientific computation
 *     verification
 *     security analysis
 *     compiler analysis
 *     data processing
 *     AI
 *     probabilistic computation
 *     quantum/classical hybrid computation
 *     HDL verification
 *     hardware analysis
 *     distributed computation
 *     simulation
 *     future computational domains
 *
 * Therefore the universal statement grammar owns the syntax.
 *
 * This file merely makes that universal construct available at the AI
 * composition boundary.
 *
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/statements/abduction.g4
 *
 * Imported parser grammar:
 *
 *     AbductionStatements
 *
 *
 * EXPORTS:
 *
 *     aiAbductionConstruct
 *     aiAbductionStatement
 *
 *
 * CONSUMED_BY:
 *
 *     grammar/ai/ai.g4
 *
 *
 * AST_OWNER:
 *
 *     Existing domain-neutral frontend AST.
 *
 * This adapter MUST NOT create an AI-specific AST node.
 *
 *
 * SEMANTIC_OWNER:
 *
 *     Universal reasoning semantic analysis
 *     AI reasoning semantic analysis
 *
 *
 * IR_OWNER:
 *
 *     Canonical semantic model
 *
 *     followed by the appropriate downstream representation.
 *
 * Quantum computation MUST cross:
 *
 *     quantum::ir
 *
 * rather than an abduction-specific quantum representation.
 *
 *
 * TEST_OWNER:
 *
 *     grammar/tests/ai/abduction/
 *     grammar/tests/semantic/reasoning/
 *     grammar/tests/boundary/
 *     grammar/tests/scalability/
 *
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/ai.md
 *     grammar/spec/semantics.md
 *     grammar/spec/provenance.md
 *     grammar/spec/resources.md
 *     grammar/spec/effects.md
 *
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This file defines NO lexer rules.
 *
 * The universal lexical vocabulary must provide:
 *
 *     ABDUCE
 *
 * with canonical spelling:
 *
 *     abduce
 *
 * The token belongs to:
 *
 *     grammar/lexer/keywords.g4
 *
 * and is consumed through:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * No AI grammar may define its own ABDUCE token.
 *
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * This adapter introduces no expression syntax.
 *
 * The universal AbductionStatements grammar consumes the canonical:
 *
 *     expression
 *
 * from:
 *
 *     grammar/expressions/expressions.g4
 *
 * Therefore abductive reasoning can operate over arbitrary valid Zamani
 * expressions, including:
 *
 *     scalar values
 *     records
 *     tuples
 *     collections
 *     streams
 *     tensors
 *     graphs
 *     datasets
 *     knowledge values
 *     uncertain values
 *     probabilistic values
 *     model results
 *     classical results
 *     quantum-derived measurements
 *     simulation results
 *     distributed results
 *     hardware observations
 *     future domain values
 *
 * This file does not create an AI-specific expression hierarchy.
 *
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Type syntax remains owned by:
 *
 *     grammar/types/
 *
 * No:
 *
 *     AbductionType
 *     HypothesisType
 *     EvidenceType
 *     ExplanationType
 *
 * is introduced here.
 *
 * Semantic type checking determines whether:
 *
 *     hypothesis
 *
 * and:
 *
 *     evidence / observation source
 *
 * are compatible with the requested abductive operation.
 *
 *
 * ============================================================================
 * REASONING FAMILY CONTRACT
 * ============================================================================
 *
 * Abduction belongs to the same semantic reasoning family as:
 *
 *     inference
 *     deduction
 *     induction
 *     generic reasoning
 *
 * However, the constructs must remain distinguishable at the semantic layer.
 *
 * The source operator:
 *
 *     abduce
 *
 * provides an explicit semantic discriminator.
 *
 * It MUST NOT be implemented as:
 *
 *     AT identifier
 *
 * because:
 *
 *     @infer
 *     @model
 *     @agent
 *     @pipeline
 *     @training
 *     @dataset
 *
 * and future annotations would otherwise create structural ambiguity.
 *
 *
 * ============================================================================
 * EVIDENCE CONTRACT
 * ============================================================================
 *
 * Evidence is represented through ordinary expressions.
 *
 * Evidence may originate from:
 *
 *     observations
 *     datasets
 *     knowledge
 *     simulations
 *     classical computation
 *     quantum measurements
 *     hardware observations
 *     network results
 *     distributed computation
 *     learned models
 *     foreign functions
 *     external services
 *
 * Evidence semantics are NOT implemented here.
 *
 * The semantic system may attach:
 *
 *     source
 *     reliability
 *     confidence
 *     provenance
 *     verification state
 *     temporal information
 *     policy information
 *
 * without changing this grammar.
 *
 *
 * ============================================================================
 * UNCERTAINTY CONTRACT
 * ============================================================================
 *
 * Abductive reasoning may naturally produce uncertain hypotheses.
 *
 * Semantic analysis may therefore represent:
 *
 *     confidence
 *     probability
 *     likelihood
 *     belief
 *     distribution
 *     interval
 *     evidence strength
 *
 * This grammar does not prescribe:
 *
 *     floating-point width
 *     probability precision
 *     probability representation
 *     distribution size
 *     numerical backend
 *     accelerator
 *     hardware
 *
 *
 * ============================================================================
 * EXPLANATION CONTRACT
 * ============================================================================
 *
 * An abductive result may require explanation.
 *
 * Explanation semantics belong to the universal explanation/evidence/
 * provenance subsystem.
 *
 * The semantic representation may retain:
 *
 *     observations considered
 *     evidence used
 *     candidate hypothesis
 *     rejected alternatives
 *     derivation
 *     confidence
 *     policy
 *     decision
 *     verification
 *
 * This file introduces no competing explanation grammar.
 *
 *
 * ============================================================================
 * KNOWLEDGE CONTRACT
 * ============================================================================
 *
 * Abduction may consume or produce knowledge.
 *
 * Example semantic flow:
 *
 *     knowledge
 *        |
 *        v
 *     observations
 *        |
 *        v
 *     abduction
 *        |
 *        v
 *     candidate explanation
 *
 * Knowledge operations remain owned by the knowledge subsystem.
 *
 * In particular:
 *
 *     assert
 *     retract
 *     query
 *
 * MUST NOT be redefined here.
 *
 *
 * ============================================================================
 * LEARNING CONTRACT
 * ============================================================================
 *
 * Abductive results may participate in learning.
 *
 * For example, an abductive hypothesis may become:
 *
 *     a candidate rule
 *     a feature
 *     a training signal
 *     a model constraint
 *     an explanation
 *     a policy input
 *
 * However:
 *
 *     abduction != learning
 *
 * Learning remains independently owned.
 *
 * No optimizer, model architecture, training algorithm, batch-size limit,
 * accelerator, or framework is encoded here.
 *
 *
 * ============================================================================
 * ADAPTATION CONTRACT
 * ============================================================================
 *
 * An abductive result may influence adaptive execution.
 *
 * Parsing abduction MUST NOT authorize:
 *
 *     self-modification
 *     unrestricted model mutation
 *     unrestricted strategy replacement
 *     code generation
 *     resource reallocation
 *     deployment changes
 *
 * Adaptation requires independent:
 *
 *     effects
 *     capabilities
 *     resources
 *     policies
 *     authorization
 *     provenance
 *
 * analysis.
 *
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * This file assigns no effects.
 *
 * Depending on semantic realization, abduction may involve:
 *
 *     computation
 *     knowledge.read
 *     knowledge.write
 *     model.inference
 *     randomness
 *     learning
 *     measurement
 *     network
 *     distributed
 *     simulation
 *     foreign
 *     native
 *     reflection
 *
 * Effect ownership remains with:
 *
 *     grammar/effects/
 *
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Resource syntax is not duplicated here.
 *
 * Requirements such as:
 *
 *     requires capability("reasoning.abduction");
 *     requires capability("probabilistic.compute");
 *     requires memory >= required_memory;
 *     requires topology(required_topology);
 *
 * belong to the canonical resource architecture.
 *
 * This grammar does not know:
 *
 *     how much memory exists
 *     how many processors exist
 *     how many accelerators exist
 *     how many quantum resources exist
 *     how many nodes exist
 *     which devices exist
 *     which vendor is available
 *
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Capability identity remains open-world.
 *
 * A semantic implementation may expose capabilities such as:
 *
 *     reasoning.abduction
 *     reasoning
 *     knowledge.query
 *     probabilistic.compute
 *     model.inference
 *
 * but this grammar does not create a closed capability catalogue.
 *
 * Hardware and vendor capabilities remain downstream.
 *
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * Abduction may be constrained by:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * Contract syntax remains owned by:
 *
 *     grammar/validation/
 *
 * No contract syntax is duplicated here.
 *
 * Parser acceptance does not imply that a contract is satisfied.
 *
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Abduction may be controlled by:
 *
 *     evidence policies
 *     privacy policies
 *     security policies
 *     model policies
 *     execution policies
 *     determinism policies
 *     resource policies
 *     adaptation policies
 *
 * Policy evaluation remains downstream.
 *
 * Valid syntax does not imply authorization.
 *
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Abduction is provenance-sensitive.
 *
 * The semantic provenance system should be capable of recording:
 *
 *     source
 *     observations
 *     evidence
 *     candidate hypothesis
 *     derivation
 *     reasoning operation
 *     model
 *     transformation
 *     verification
 *     decision
 *     policy
 *     language version
 *     semantic version
 *
 * Provenance ownership remains outside this grammar.
 *
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * Quantum-derived observations may be supplied as ordinary expressions.
 *
 * Example:
 *
 *     abduce fault_model from measurements;
 *
 * The quantum subsystem remains responsible for:
 *
 *     quantum syntax
 *     quantum semantic analysis
 *     quantum::ir
 *     optimization
 *     decomposition
 *     routing
 *     scheduling
 *     resilience
 *     QEC
 *     ZQN
 *     HAL
 *
 * This grammar introduces:
 *
 *     no qubit syntax
 *     no gate catalogue
 *     no physical topology
 *     no calibration
 *     no routing
 *     no QEC representation
 *
 *
 * ============================================================================
 * HDL / HARDWARE CONTRACT
 * ============================================================================
 *
 * Abduction may consume:
 *
 *     simulation results
 *     verification results
 *     timing observations
 *     hardware observations
 *     synthesis information
 *     telemetry
 *
 * HDL and hardware syntax remains owned by:
 *
 *     grammar/hdl/
 *     grammar/hardware/
 *
 * No register width, bus width, device count, memory capacity, processor
 * count, or physical address is encoded here.
 *
 *
 * ============================================================================
 * DISTRIBUTED CONTRACT
 * ============================================================================
 *
 * Evidence may originate from:
 *
 *     actors
 *     services
 *     distributed tasks
 *     replicated observations
 *     collective computation
 *
 * Distributed semantics remain owned by:
 *
 *     grammar/concurrency/
 *     grammar/distributed/
 *
 * This adapter introduces no topology syntax.
 *
 *
 * ============================================================================
 * FFI / ABI CONTRACT
 * ============================================================================
 *
 * Foreign-function results may be used as abductive evidence.
 *
 * FFI and ABI syntax remain owned by:
 *
 *     grammar/interoperability/
 *
 * Foreign effects and capabilities are checked independently.
 *
 *
 * ============================================================================
 * METAPROGRAMMING CONTRACT
 * ============================================================================
 *
 * Abduction may operate over compile-time or reflective values when the
 * semantic system permits it.
 *
 * Reflection and compile-time execution remain owned by:
 *
 *     grammar/metaprogramming/
 *
 * This grammar introduces no reflection syntax.
 *
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY CONTRACT
 * ============================================================================
 *
 * This adapter introduces NO finite computational capacity.
 *
 * It contains no limits for:
 *
 *     hypotheses
 *     evidence
 *     observations
 *     source expressions
 *     context expressions
 *     models
 *     datasets
 *     reasoning operations
 *     AI constructs
 *     quantum resources
 *     classical resources
 *     distributed resources
 *     hardware resources
 *
 * No constants such as:
 *
 *     MAX_ABDUCTIONS
 *     MAX_HYPOTHESES
 *     MAX_EVIDENCE
 *     MAX_OBSERVATIONS
 *     MAX_MODELS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * may be introduced.
 *
 * "Infinity" means that the language introduces no artificial finite machine
 * ceiling.
 *
 * Actual feasibility is determined downstream by:
 *
 *     available resources
 *     target capabilities
 *     policies
 *     compiler implementation
 *     runtime
 *     operating environment
 *
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing MUST depend only on:
 *
 *     source
 *     token stream
 *     grammar
 *     parser configuration
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware
 *     available memory
 *     network state
 *     filesystem state
 *     runtime state
 *     scheduler state
 *     randomness
 *     wall-clock time
 *
 * Any probabilistic or nondeterministic abductive realization is a semantic or
 * runtime concern.
 *
 *
 * ============================================================================
 * SAFETY CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no embedded Rust
 *     no semantic predicates
 *     no target actions
 *     no filesystem access
 *     no network access
 *     no hardware access
 *     no runtime execution
 *
 * Generated Rust integration must remain compatible with:
 *
 *     Rust 1.97+
 *     Rust 2021
 *
 * and must use safe Rust only.
 *
 *
 * ============================================================================
 * PRODUCTION COMPOSITION
 * ============================================================================
 */

parser grammar Abduction;

options {
    tokenVocab = ZamaniLexer;
}

import
    AbductionStatements
    ;


/*
 * ============================================================================
 * PUBLIC AI BOUNDARY
 * ============================================================================
 *
 * The AI subsystem consumes the universal abductive-reasoning representation.
 *
 * No syntax is duplicated here.
 * ============================================================================
 */

aiAbductionConstruct
    : aiAbductionStatement
    ;


aiAbductionStatement
    : abductionStatement
    ;
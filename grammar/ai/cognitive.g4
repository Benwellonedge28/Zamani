/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/ai/cognitive.g4
 *
 * GRAMMAR
 * -------
 * Cognitive
 *
 * STATUS
 * ------
 * CANONICAL AI COGNITIVE COMPOSITION / INTEGRATION GRAMMAR
 *
 * LANGUAGE BASELINE
 * -----------------
 * Rust 1.97+
 * Rust edition 2021
 * Safe Rust only
 * No embedded Rust actions
 * No unsafe implementation requirement
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the stable AI-domain composition boundary for Zamani's
 * cognitive-computation facilities.
 *
 * IMPORTANT:
 *
 * The detailed source syntax for a mind/cognitive computation is already
 * owned by:
 *
 *     grammar/ai/mind.g4
 *
 * whose parser grammar identity is:
 *
 *     AIMind
 *
 * This file MUST therefore remain a composition adapter.
 *
 * It MUST NOT reproduce the rules from mind.g4.
 *
 * Its purpose is to make the cognitive subsystem explicitly consumable by:
 *
 *     grammar/ai/ai.g4
 *
 * while preserving the repository's single-authority architecture.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * Source
 *   |
 *   v
 * ZamaniLexer
 *   |
 *   v
 * ZamaniParser
 *   |
 *   +-----------------------------+
 *   |                             |
 *   v                             v
 * universal language          AI domain
 *                                 |
 *                                 v
 *                             Cognitive
 *                                 |
 *                                 v
 *                              AIMind
 *                                 |
 *                                 v
 *                       domain-neutral frontend AST
 *                                 |
 *                                 v
 *                         structural validation
 *                                 |
 *          +----------------------+----------------------+
 *          |                      |                      |
 *          v                      v                      v
 *        types                 effects              resources
 *          |                      |                      |
 *          +----------------------+----------------------+
 *                                 |
 *                                 v
 *                         semantic analysis
 *                                 |
 *          +----------------------+----------------------+
 *          |                      |                      |
 *          v                      v                      v
 *       classical             quantum::ir           hybrid/HDL
 *          |                      |                      |
 *          +----------------------+----------------------+
 *                                 |
 *                                 v
 *                         canonical IR pipeline
 *                                 |
 *                    optimization / specialization
 *                                 |
 *                         lowering / routing
 *                                 |
 *                        scheduling / resilience
 *                                 |
 *                              ZQN / HAL
 *                                 |
 *                                 v
 *                         target realization
 *
 * This file contributes ONLY the grammar composition boundary.
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns:
 *
 *     - the Cognitive parser grammar identity;
 *     - the AI cognitive composition boundary;
 *     - the public cognitiveConstruct rule;
 *     - delegation from AI composition to AIMind;
 *     - the stable integration point consumed by AI composition;
 *     - documentation of the cognitive grammar dependency boundary.
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     - lexer rules;
 *     - keywords;
 *     - identifiers;
 *     - names;
 *     - qualified names;
 *     - expressions;
 *     - expression precedence;
 *     - types;
 *     - statements;
 *     - declarations;
 *     - functions;
 *     - modules;
 *     - agents;
 *     - actors;
 *     - knowledge syntax;
 *     - reasoning syntax;
 *     - learning syntax;
 *     - adaptation syntax;
 *     - uncertainty syntax;
 *     - evidence syntax;
 *     - provenance syntax;
 *     - explanation syntax;
 *     - policy syntax;
 *     - contract syntax;
 *     - resource syntax;
 *     - capability syntax;
 *     - memory implementation;
 *     - model implementation;
 *     - tensor implementation;
 *     - dataset implementation;
 *     - quantum operations;
 *     - quantum topology;
 *     - routing;
 *     - scheduling;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution;
 *     - hardware discovery;
 *     - target selection;
 *     - resource allocation;
 *     - AI algorithms;
 *     - model architectures;
 *     - inference engines;
 *     - learning algorithms;
 *     - planning algorithms;
 *     - decision algorithms;
 *     - a cognitive-specific IR;
 *     - a second AI IR;
 *     - a second quantum IR.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * Detailed cognitive source syntax is owned exclusively by:
 *
 *     grammar/ai/mind.g4
 *
 * Grammar identity:
 *
 *     AIMind
 *
 * The canonical public cognitive entry point in that grammar is:
 *
 *     mindConstruct
 *
 * This file MUST NOT redefine any of the following:
 *
 *     mindConstruct
 *     mindDeclaration
 *     mindBinding
 *     mindInvocation
 *     mindRegion
 *     mindMember
 *     mindNamedMember
 *     mindBindingMember
 *     mindInvocationMember
 *     mindAssignmentMember
 *     mindRegionMember
 *     mindResourceContract
 *     mindCapabilityContract
 *     mindConstraintContract
 *     mindPreferenceContract
 *     mindHintContract
 *
 * Instead:
 *
 *     cognitiveConstruct
 *
 * delegates directly to:
 *
 *     mindConstruct
 *
 * This prevents grammar divergence.
 *
 * ============================================================================
 * WHY AN ADAPTER IS REQUIRED
 * ============================================================================
 *
 * `mind.g4` is a leaf grammar with a broad cognitive source model.
 *
 * `ai.g4` is the AI domain composition grammar.
 *
 * A direct dependency from the complete AI grammar to the leaf grammar is
 * desirable, but the cognitive feature needs a stable named boundary so that
 * future AI composition can consume the cognitive subsystem without knowing
 * the internal rule names of AIMind.
 *
 * Therefore:
 *
 *     AIMind
 *       |
 *       v
 * cognitiveConstruct
 *       |
 *       v
 * AI
 *
 * is the canonical direction.
 *
 * This also permits the internal implementation of `mind.g4` to evolve while
 * preserving the AI-domain composition boundary.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This file defines NO lexer rules.
 *
 * It therefore introduces:
 *
 *     no new keywords;
 *     no new operators;
 *     no new delimiters;
 *     no new literals;
 *     no lexical limits.
 *
 * Cognitive vocabulary remains open and is interpreted through the canonical
 * cognitive grammar and semantic layer.
 *
 * This is important for extensibility and POCO-REAF.
 *
 * New cognitive methods, models, reasoning systems, learning methods,
 * planning mechanisms, memory implementations, or decision strategies must
 * NOT require a universal lexer-keyword expansion merely because they exist.
 *
 * ============================================================================
 * GRAMMAR DEPENDENCIES
 * ============================================================================
 *
 * DIRECT DEPENDENCY:
 *
 *     AIMind
 *
 * AIMind already owns its required dependencies:
 *
 *     Types
 *     Expressions
 *     Statements
 *
 * Therefore this file intentionally does not independently import:
 *
 *     Types
 *     Expressions
 *     Statements
 *
 * unless a future change introduces a direct rule dependency requiring them.
 *
 * This keeps the dependency graph minimal.
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 * Correct:
 *
 *     Cognitive
 *         |
 *         v
 *     AIMind
 *
 *     AI
 *         |
 *         v
 *     Cognitive
 *
 * Incorrect:
 *
 *     AIMind
 *         |
 *         v
 *     Cognitive
 *
 *     Cognitive
 *         |
 *         v
 *     AI
 *
 * The leaf cognitive grammar MUST NOT depend on the AI composition grammar.
 *
 * ============================================================================
 * PUBLIC API
 * ============================================================================
 *
 * This grammar exposes exactly one public composition rule:
 *
 *     cognitiveConstruct
 *
 * That rule is intentionally small.
 *
 * It represents the complete cognitive-domain construct through the canonical
 * AIMind grammar.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates NO new AST hierarchy.
 *
 * The parse tree originates from:
 *
 *     mindConstruct
 *
 * through:
 *
 *     cognitiveConstruct
 *
 * The frontend AST remains domain-neutral.
 *
 * The semantic frontend is responsible for interpreting the resulting
 * structure as a cognitive computation where the source semantics require it.
 *
 * The AST MUST preserve, as applicable:
 *
 *     - source spans;
 *     - annotations;
 *     - identifiers;
 *     - types;
 *     - expressions;
 *     - member ordering;
 *     - nested regions;
 *     - referenced names;
 *     - contracts;
 *     - resource intent;
 *     - capability intent;
 *     - provenance.
 *
 * This file MUST NOT require:
 *
 *     CognitiveNode
 *     CognitiveAST
 *     MindIR
 *     CognitiveIR
 *
 * merely to represent the parser boundary.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Cognitive semantics are resolved downstream.
 *
 * Depending on the source program, semantic analysis may connect cognitive
 * constructs with:
 *
 *     reasoning
 *     knowledge
 *     inference
 *     learning
 *     adaptation
 *     uncertainty
 *     evidence
 *     provenance
 *     explanation
 *     decisions
 *     policies
 *     contracts
 *     memory
 *     models
 *     tensors
 *     agents
 *     concurrency
 *     distributed computation
 *     classical computation
 *     quantum computation
 *     hybrid computation
 *     HDL/hardware intent
 *
 * These are semantic relationships.
 *
 * This grammar does not implement them.
 *
 * ============================================================================
 * REASONING INTEGRATION
 * ============================================================================
 *
 * Generic reasoning remains universal.
 *
 * Its syntax is owned by:
 *
 *     grammar/statements/reason.g4
 *
 * The cognitive subsystem may consume reasoning semantics through the
 * canonical AIMind structure.
 *
 * Cognitive MUST NOT introduce:
 *
 *     CognitiveReasoning
 *
 * as a competing syntax hierarchy.
 *
 * ============================================================================
 * KNOWLEDGE INTEGRATION
 * ============================================================================
 *
 * Knowledge remains owned by its canonical knowledge subsystem.
 *
 * Cognitive constructs may reference knowledge, beliefs, observations and
 * related values through the existing expression/semantic architecture.
 *
 * This file MUST NOT create:
 *
 *     CognitiveKnowledge
 *
 * as a second knowledge language.
 *
 * ============================================================================
 * LEARNING INTEGRATION
 * ============================================================================
 *
 * Learning remains a semantic operation governed by the existing learning
 * subsystem.
 *
 * Cognitive constructs may refer to learning intent.
 *
 * Algorithm selection remains downstream.
 *
 * Therefore this file contains no enumeration of:
 *
 *     neural architectures;
 *     optimization algorithms;
 *     training algorithms;
 *     reinforcement algorithms;
 *     transfer-learning algorithms;
 *     future learning algorithms.
 *
 * ============================================================================
 * ADAPTATION INTEGRATION
 * ============================================================================
 *
 * Adaptation remains controlled semantic behavior.
 *
 * Cognitive syntax does not imply unrestricted self-modification.
 *
 * Any adaptation must remain subject to the existing:
 *
 *     effects
 *     capabilities
 *     resources
 *     contracts
 *     policies
 *     security
 *     provenance
 *
 * subsystems.
 *
 * ============================================================================
 * UNCERTAINTY / EVIDENCE / PROVENANCE
 * ============================================================================
 *
 * Cognitive computation may interact with:
 *
 *     uncertainty;
 *     probability;
 *     confidence;
 *     evidence;
 *     provenance;
 *     explanation;
 *     decision records.
 *
 * These remain shared semantic concepts rather than cognitive-specific
 * implementations.
 *
 * ============================================================================
 * AGENT INTEGRATION
 * ============================================================================
 *
 * A cognitive computation is not automatically an agent.
 *
 * Agent syntax and lifecycle remain owned by the existing agent/concurrency
 * subsystem.
 *
 * The correct relationship is:
 *
 *     cognitive computation
 *          |
 *          v
 *     semantic result/state
 *          |
 *          v
 *     agent/actor
 *          |
 *          v
 *     message / action / external interaction
 *
 * This file does not create a second actor model.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Cognitive computation may participate in hybrid or quantum programs.
 *
 * The boundary remains:
 *
 *     cognitive source intent
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum semantic lowering
 *          |
 *          v
 *     quantum::ir
 *
 * This file MUST NOT:
 *
 *     enumerate quantum gates;
 *     enumerate physical qubits;
 *     encode topology;
 *     encode routing;
 *     encode calibration;
 *     encode QEC implementation;
 *     select a QPU.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Cognitive computation may require hardware capabilities through the existing
 * resource/capability architecture.
 *
 * Example semantic intent:
 *
 *     requires capability("tensor.compute")
 *
 * or:
 *
 *     requires capability("quantum.measurement")
 *
 * or other registered capabilities.
 *
 * This grammar does not decide where computation executes.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY CONTRACT
 * ============================================================================
 *
 * No resource capacity is hard-coded here.
 *
 * The grammar MUST NOT contain:
 *
 *     MAX_COGNITIVE_STATES
 *     MAX_MEMORIES
 *     MAX_MODELS
 *     MAX_REASONING_STEPS
 *     MAX_AGENTS
 *     MAX_CONTEXTS
 *     MAX_DEVICES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_TENSOR_RANK
 *
 * or equivalent limits.
 *
 * Source programs express requirements symbolically.
 *
 * Actual feasibility is determined by:
 *
 *     semantic analysis
 *     capability negotiation
 *     resource management
 *     compilation
 *     scheduling
 *     routing
 *     runtime
 *     deployment
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Cognitive source syntax must remain independent of physical scale.
 *
 * The same source construct must be representable on:
 *
 *     tiny embedded systems
 *     CPUs
 *     multicore systems
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     simulators
 *     QPUs
 *     HPC systems
 *     clusters
 *     distributed systems
 *     cloud systems
 *     future targets
 *
 * subject only to semantic feasibility and available resources/capabilities.
 *
 * The grammar does not promise that every target can execute every program.
 *
 * Instead:
 *
 *     source meaning remains portable;
 *     target feasibility is evaluated downstream.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing through this grammar depends only on:
 *
 *     source text
 *     ZamaniLexer
 *     imported grammar definitions
 *     parser configuration
 *
 * It MUST NOT depend on:
 *
 *     hardware discovery
 *     runtime state
 *     network state
 *     model availability
 *     filesystem state
 *     randomness
 *     current time
 *     target resource availability
 *     deployment state
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This ANTLR grammar contains:
 *
 *     - no embedded Rust actions;
 *     - no embedded target-language code;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no process execution;
 *     - no hardware discovery;
 *     - no runtime execution;
 *     - no randomness;
 *     - no unsafe code.
 *
 * Rust implementations consuming this grammar remain compatible with:
 *
 *     Rust 1.97+
 *     Rust edition 2021
 *     safe Rust
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file produces NO IR.
 *
 * There is no:
 *
 *     CognitiveIR
 *     MindIR
 *     AIIR
 *
 * introduced by this grammar.
 *
 * The semantic pipeline remains:
 *
 *     source
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     semantic model
 *       |
 *       +--------------------+
 *       |                    |
 *       v                    v
 *   classical            quantum::ir
 *       |                    |
 *       +---------+----------+
 *                 |
 *                 v
 *        target-independent
 *        optimization/lowering
 *                 |
 *                 v
 *          routing/scheduling
 *                 |
 *                 v
 *            resilience
 *                 |
 *                 v
 *              ZQN/HAL
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * This grammar itself records no runtime provenance.
 *
 * The frontend/AST/semantic layers must preserve source provenance so that
 * cognitive constructs can participate in the repository-wide provenance
 * system.
 *
 * At minimum, downstream provenance must be able to identify:
 *
 *     source location
 *     originating construct
 *     transformations
 *     semantic decisions
 *     evidence where applicable
 *     generated artifacts
 *     verification information where applicable
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Cognitive constructs remain subject to the repository-wide policy system.
 *
 * Policies may constrain:
 *
 *     reasoning
 *     learning
 *     adaptation
 *     memory access
 *     external interaction
 *     model use
 *     network effects
 *     native/foreign effects
 *     resource consumption
 *     quantum execution
 *     distributed execution
 *
 * Policy enforcement is NOT performed by this grammar.
 *
 * ============================================================================
 * ERROR / DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics for this boundary must report ordinary ANTLR source
 * locations and preserve the underlying AIMind rule context.
 *
 * The adapter MUST NOT swallow or replace errors produced by AIMind.
 *
 * Semantic diagnostics belong to semantic validation.
 *
 * Resource/capability diagnostics belong to resource analysis.
 *
 * Policy diagnostics belong to policy/security analysis.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This file is a stable composition boundary.
 *
 * Internal cognitive syntax changes should normally occur in:
 *
 *     grammar/ai/mind.g4
 *
 * while preserving:
 *
 *     mindConstruct
 *
 * If the public AIMind boundary changes, this adapter must be updated in the
 * same change as the owning grammar and its conformance tests.
 *
 * No downstream AI grammar should directly depend on private AIMind rules.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Tests for this file belong under:
 *
 *     grammar/tests/ai/
 *
 * Recommended groups:
 *
 *     cognitive/
 *     integration/
 *     negative/
 *     portability/
 *     scalability/
 *
 * Minimum positive coverage:
 *
 *     - cognitive declaration;
 *     - cognitive binding;
 *     - cognitive invocation;
 *     - cognitive region;
 *     - cognitive members;
 *     - resource contracts;
 *     - capability contracts;
 *     - ordinary Zamani statements inside a cognitive region.
 *
 * Minimum integration coverage:
 *
 *     cognitive -> AI
 *     cognitive -> reasoning
 *     cognitive -> knowledge
 *     cognitive -> learning
 *     cognitive -> adaptation
 *     cognitive -> uncertainty
 *     cognitive -> evidence
 *     cognitive -> provenance
 *     cognitive -> policy
 *     cognitive -> resources
 *     cognitive -> classical
 *     cognitive -> quantum
 *     cognitive -> hybrid
 *     cognitive -> concurrency
 *
 * Negative coverage must verify that this adapter does not:
 *
 *     - invent a second cognitive syntax;
 *     - accept malformed AIMind constructs;
 *     - bypass ordinary expression/type validation;
 *     - bypass resource/capability analysis.
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests MUST NOT rely on artificial fixed capacity constants.
 *
 * Scalability tests should exercise:
 *
 *     repeated cognitive members;
 *     nested cognitive regions;
 *     large source programs;
 *     large expression graphs;
 *     large knowledge references;
 *     large collections of cognitive constructs;
 *     cross-domain composition.
 *
 * Test scale must be generated/configured by the test harness rather than
 * represented as a language-level maximum.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file intentionally contains:
 *
 *     no physical resource constants;
 *     no hardware identifiers;
 *     no algorithm catalogue;
 *     no model catalogue;
 *     no gate catalogue;
 *     no accelerator catalogue;
 *     no device-count limit;
 *     no tensor-rank limit;
 *     no memory limit;
 *     no node limit;
 *     no thread limit;
 *     no parser recursion constant;
 *     no runtime capacity constant.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [x] It has exactly one ownership responsibility.
 *     [x] It delegates detailed syntax to AIMind.
 *     [x] It introduces no duplicate cognitive grammar.
 *     [x] It introduces no new lexer vocabulary.
 *     [x] It introduces no cognitive-specific IR.
 *     [x] It introduces no physical resource limits.
 *     [x] It composes cleanly with AI.
 *     [x] It preserves the domain-neutral AST boundary.
 *     [x] It preserves the canonical semantic pipeline.
 *     [x] It preserves the quantum::ir boundary.
 *     [x] It is deterministic.
 *     [x] It contains no unsafe implementation.
 *     [x] It has positive, negative, integration and scalability tests.
 *
 * ============================================================================
 * ANTLR GRAMMAR
 * ============================================================================
 */

parser grammar Cognitive;

options {
    tokenVocab = ZamaniLexer;
}

import AIMind;


/*
 * ============================================================================
 * PUBLIC COGNITIVE COMPOSITION ENTRY
 * ============================================================================
 *
 * `mindConstruct` is the canonical source syntax owner.
 *
 * This adapter deliberately does not reproduce any of its alternatives.
 *
 * ============================================================================
 */

cognitiveConstruct
    : mindConstruct
    ;
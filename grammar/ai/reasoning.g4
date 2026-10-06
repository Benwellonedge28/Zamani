/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/ai/reasoning.g4
 *
 * Grammar:
 *     AIReasoning
 *
 * Status:
 *     CANONICAL AI REASONING COMPOSITION / ADAPTER GRAMMAR
 *
 * Implementation baseline:
 *     Rust 1.97+
 *     Rust edition 2021
 *     Safe Rust only
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
 * This file is the AI-domain composition boundary for generic computational
 * reasoning.
 *
 * It integrates the universal reasoning subsystem with the AI subsystem
 * without creating a second reasoning language.
 *
 * Generic reasoning is a universal Zamani capability and may therefore be
 * used by:
 *
 *     classical computation
 *     scientific computation
 *     data processing
 *     knowledge systems
 *     AI / machine learning
 *     probabilistic computation
 *     quantum/classical systems
 *     HDL/hardware systems
 *     distributed systems
 *     networking
 *     simulation
 *     security
 *     optimization
 *     future computational domains
 *
 * The canonical reasoning syntax is owned by:
 *
 *     grammar/statements/reason.g4
 *
 * This file MUST delegate to that owner.
 *
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns ONLY the AI-domain composition boundary:
 *
 *     aiReasoningConstruct
 *
 * It establishes that generic reasoning is a valid AI-domain construct.
 *
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     infer syntax
 *     deduce syntax
 *     reason syntax
 *     reasoning operators
 *     reasoning targets
 *     reasoning source clauses
 *     reasoning context clauses
 *     reasoning options
 *     expressions
 *     expression precedence
 *     identifiers
 *     names
 *     types
 *     patterns
 *     guards
 *     knowledge assertions
 *     knowledge retraction
 *     knowledge queries
 *     learning
 *     adaptation
 *     uncertainty
 *     probability
 *     evidence
 *     provenance
 *     explanations
 *     decisions
 *     policies
 *     contracts
 *     effects
 *     capabilities
 *     resources
 *     models
 *     tensors
 *     datasets
 *     quantum operations
 *     HDL syntax
 *     hardware syntax
 *     distributed execution
 *     networking
 *     FFI
 *     ABI
 *     metaprogramming
 *     IR
 *     runtime execution
 *     backend realization
 *
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     ZamaniParser
 *       |
 *       +-----------------------------+
 *       |                             |
 *       v                             v
 *     Statements                      AI
 *       |                             |
 *       v                             v
 *     ReasonStatements          AIReasoning
 *       |                             |
 *       +-------------+---------------+
 *                     |
 *                     v
 *             domain-neutral AST
 *                     |
 *                     v
 *             structural validation
 *                     |
 *       +-------------+-------------+-------------+
 *       |             |             |             |
 *       v             v             v             v
 *     types        effects      capabilities   resources
 *       |             |             |             |
 *       +-------------+-------------+-------------+
 *                     |
 *                     v
 *              semantic reasoning
 *                     |
 *       +-------------+-------------+-------------+
 *       |             |             |             |
 *       v             v             v             v
 *   classical      quantum         hybrid       AI/model
 *       |             |             |             |
 *       |             v             |             |
 *       |         quantum::ir      |             |
 *       |             |             |             |
 *       +-------------+-------------+-------------+
 *                     |
 *                     v
 *                canonical IR
 *                     |
 *               optimization
 *                     |
 *               specialization
 *                     |
 *                  lowering
 *                     |
 *             routing/scheduling
 *                     |
 *          resilience/recovery
 *                     |
 *                 ZQN / HAL
 *                     |
 *             target realization
 *
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There MUST be exactly one authoritative grammar for generic reasoning
 * statement syntax.
 *
 * That authority is:
 *
 *     grammar/statements/reason.g4
 *
 * Its grammar identity is:
 *
 *     ReasonStatements
 *
 * This file MUST NOT redefine:
 *
 *     reasonStatement
 *     reasoningOperator
 *     reasoningTarget
 *     reasoningSourceClause
 *     reasoningContextClause
 *     reasoningOptionList
 *     reasoningOption
 *
 * Instead it imports ReasonStatements and exposes a stable AI-domain adapter.
 *
 *
 * ============================================================================
 * WHY THIS FILE EXISTS
 * ============================================================================
 *
 * Reasoning is intentionally broader than AI.
 *
 * Consequently, putting all reasoning syntax directly inside:
 *
 *     grammar/ai/
 *
 * would incorrectly make reasoning an AI-only language feature.
 *
 * Conversely, leaving AI composition unaware of reasoning would prevent AI
 * programs from using the universal reasoning subsystem through the canonical
 * AI composition boundary.
 *
 * This file solves both problems:
 *
 *     universal syntax
 *          ->
 *     statements/reason.g4
 *          ->
 *     AI composition adapter
 *          ->
 *     ai.g4
 *
 * Therefore:
 *
 *     reasoning remains universal
 *
 * while:
 *
 *     AI can consume reasoning naturally.
 *
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/statements/reason.g4
 *
 * IMPORTS:
 *
 *     ReasonStatements
 *
 * EXPORTS:
 *
 *     aiReasoningConstruct
 *
 * CONSUMED_BY:
 *
 *     grammar/ai/ai.g4
 *
 * AST_OWNER:
 *
 *     existing domain-neutral frontend AST
 *
 * SEMANTIC_OWNER:
 *
 *     universal reasoning semantic subsystem
 *
 *     plus:
 *
 *         AI semantic analysis
 *         type analysis
 *         effect analysis
 *         capability analysis
 *         resource analysis
 *         contract analysis
 *         policy analysis
 *         provenance
 *
 * IR_OWNER:
 *
 *     canonical semantic/domain IR layers
 *
 * Quantum reasoning ultimately crosses:
 *
 *     quantum::ir
 *
 * TEST_OWNER:
 *
 *     grammar/tests/ai/reasoning/
 *
 *     and universal reasoning tests under:
 *
 *     grammar/tests/statements/reasoning/
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/ai.md
 *     grammar/specification/
 *
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 * The dependency graph MUST remain:
 *
 *     Expressions
 *          ^
 *          |
 *     ReasonStatements
 *          ^
 *          |
 *     AIReasoning
 *          ^
 *          |
 *     AI
 *
 * In particular:
 *
 *     AIReasoning
 *
 * MUST NOT import:
 *
 *     AI
 *     ZamaniParser
 *     Statements
 *
 * because doing so would introduce circular composition.
 *
 * `Statements` already composes the universal reasoning statement grammar.
 *
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This file defines NO lexer rules.
 *
 * Lexer authority remains:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * through:
 *
 *     grammar/lexer/
 *
 * Reasoning vocabulary such as:
 *
 *     infer
 *     deduce
 *     reason
 *     from
 *     with
 *
 * is owned by the canonical lexical system.
 *
 * This file must never redefine those tokens.
 *
 *
 * ============================================================================
 * GRAMMAR DECLARATION
 * ============================================================================
 */

parser grammar AIReasoning;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * IMPORTS
 * ============================================================================
 *
 * ReasonStatements is the universal owner of:
 *
 *     reasonStatement
 *
 * and its supporting reasoning rules.
 *
 * ============================================================================
 */

import ReasonStatements;


/*
 * ============================================================================
 * PUBLIC AI REASONING ENTRY
 * ============================================================================
 *
 * AI reasoning is exactly the universal reasoning construct.
 *
 * There is deliberately no AI-specific reasoning syntax here.
 *
 * This guarantees:
 *
 *     infer
 *     deduce
 *     reason
 *
 * have identical syntax and structural meaning whether they occur in:
 *
 *     classical code
 *     AI code
 *     data code
 *     hybrid code
 *     quantum/classical code
 *     distributed code
 *     simulation
 *     future domains
 *
 * ============================================================================
 */

aiReasoningConstruct
    : reasonStatement
    ;


/*
 * ============================================================================
 * EXPLICIT REUSABLE AI BOUNDARY
 * ============================================================================
 *
 * This alias is useful for AI leaf grammars that need to distinguish an
 * AI-composed reasoning construct from other AI constructs without duplicating
 * the underlying reasoning grammar.
 *
 * It remains structurally identical to reasonStatement.
 *
 * ============================================================================
 */

aiReasoningStatement
    : reasonStatement
    ;


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates NO new reasoning AST hierarchy.
 *
 * The AST must represent:
 *
 *     infer
 *     deduce
 *     reason
 *
 * using the existing domain-neutral reasoning representation produced from:
 *
 *     reasonStatement
 *
 * The AST MUST NOT introduce:
 *
 *     AIReasoningNode
 *     AIInferenceNode
 *     AIDeductionNode
 *     AIReasonNode
 *
 * merely because the construct was encountered through the AI composition
 * boundary.
 *
 * The source-domain context may be preserved separately where required by
 * the semantic model.
 *
 * The AST must preserve, as applicable:
 *
 *     source span
 *     operator kind
 *     target expression
 *     source expression
 *     context expressions
 *     reasoning options
 *     nested expression structure
 *     source ordering
 *     provenance
 *
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     what is being inferred;
 *     what is being deduced;
 *     what reasoning operation is requested;
 *     which evidence is available;
 *     which knowledge sources are referenced;
 *     which models are referenced;
 *     which policies apply;
 *     which capabilities are required;
 *     which resources are required;
 *     which effects occur;
 *     whether the operation is deterministic;
 *     whether the operation is authorized;
 *     whether contracts are satisfied;
 *     whether provenance requirements are satisfied;
 *     how the operation is lowered.
 *
 * The grammar does not decide any of these properties.
 *
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Reasoning targets and sources are ordinary Zamani expressions.
 *
 * Therefore reasoning can operate over:
 *
 *     scalar values
 *     structured values
 *     records
 *     collections
 *     streams
 *     tensors
 *     datasets
 *     graph values
 *     probabilistic values
 *     uncertain values
 *     model outputs
 *     classical results
 *     quantum measurement results
 *     simulation results
 *     distributed results
 *     hardware-derived observations
 *     future domain values
 *
 * Type checking remains the responsibility of the universal type system.
 *
 * This adapter introduces no AI-specific type system.
 *
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Reasoning effects are determined downstream.
 *
 * Depending on the referenced expressions and selected semantic realization,
 * reasoning may involve effects such as:
 *
 *     computation
 *     knowledge.read
 *     model.inference
 *     randomness
 *     network
 *     external.io
 *     measurement
 *     distributed
 *     simulation
 *     foreign
 *     native
 *
 * This grammar neither declares nor assumes those effects.
 *
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * AI reasoning may require capabilities such as:
 *
 *     reasoning
 *     reasoning.inference
 *     reasoning.deduction
 *     knowledge.query
 *     model.inference
 *     probabilistic.compute
 *
 * or capabilities supplied by future semantic extensions.
 *
 * Capability names remain semantic data.
 *
 * This grammar does not enumerate hardware capabilities.
 *
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Reasoning may require arbitrary resources.
 *
 * Examples include:
 *
 *     compute
 *     memory
 *     storage
 *     communication
 *     accelerator resources
 *     quantum resources
 *     distributed resources
 *     model resources
 *
 * Resource requirements are expressed through the universal resource system.
 *
 * This file MUST NOT duplicate:
 *
 *     requires
 *     capability
 *     constraint
 *     prefer
 *     hint
 *
 * grammar.
 *
 * It also MUST NOT introduce resource ceilings.
 *
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * Reasoning may execute under:
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
 * Contract syntax is owned elsewhere.
 *
 * This grammar simply permits the reasoning construct to participate in the
 * universal contract model.
 *
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Reasoning may be constrained by:
 *
 *     execution policy
 *     security policy
 *     evidence policy
 *     privacy policy
 *     resource policy
 *     model policy
 *     determinism policy
 *     adaptation policy
 *
 * Policy evaluation is semantic.
 *
 * Parsing a reasoning construct does not grant authorization.
 *
 *
 * ============================================================================
 * EVIDENCE CONTRACT
 * ============================================================================
 *
 * Reasoning may consume evidence originating from:
 *
 *     data
 *     knowledge
 *     classical computation
 *     quantum measurement
 *     simulation
 *     hardware observation
 *     learned models
 *     distributed computation
 *     network services
 *     scientific computation
 *
 * Evidence grammar and provenance remain owned by their respective
 * subsystems.
 *
 * The reasoning AST must retain enough structure for those systems to attach
 * evidence and provenance after parsing.
 *
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Reasoning must remain explainable and traceable.
 *
 * Downstream provenance may record:
 *
 *     source
 *     premises
 *     evidence
 *     model
 *     transformation
 *     decision
 *     verification
 *     derivation
 *     policy
 *     version
 *
 * This grammar does not invent a separate AI provenance model.
 *
 *
 * ============================================================================
 * KNOWLEDGE INTEGRATION
 * ============================================================================
 *
 * Reasoning may consume knowledge operations supplied by the knowledge
 * subsystem.
 *
 * Conceptually:
 *
 *     assert fact;
 *     infer answer from query(pattern);
 *     deduce conclusion from observation;
 *
 * Knowledge syntax remains owned elsewhere.
 *
 * No knowledge grammar is duplicated here.
 *
 *
 * ============================================================================
 * LEARNING INTEGRATION
 * ============================================================================
 *
 * Reasoning may consume learned results:
 *
 *     infer result from model(input);
 *
 *     reason classification from predictor(value);
 *
 * Learning/training syntax remains owned by:
 *
 *     grammar/ai/learning.g4
 *
 * or the repository's canonical training/learning composition.
 *
 * This adapter does not define model-training algorithms.
 *
 *
 * ============================================================================
 * ADAPTATION INTEGRATION
 * ============================================================================
 *
 * Reasoning may provide information to adaptive execution.
 *
 * For example:
 *
 *     infer strategy from evidence;
 *
 * may produce a semantic value subsequently consumed by adaptation.
 *
 * Reasoning itself does not authorize:
 *
 *     self-modification
 *     unrestricted model mutation
 *     unrestricted strategy replacement
 *     resource reallocation
 *     code mutation
 *
 * Such operations require their own:
 *
 *     effects
 *     capabilities
 *     resources
 *     policies
 *     provenance
 *
 *
 * ============================================================================
 * UNCERTAINTY / PROBABILITY INTEGRATION
 * ============================================================================
 *
 * Reasoning may operate on:
 *
 *     probability
 *     distributions
 *     confidence
 *     belief
 *     uncertain values
 *     intervals
 *     symbolic uncertainty
 *
 * No precision, range, representation, distribution size, or algorithm is
 * hard-coded here.
 *
 *
 * ============================================================================
 * PATTERN / GUARD INTEGRATION
 * ============================================================================
 *
 * Reasoning may consume expressions containing:
 *
 *     patterns
 *     matches
 *     guards
 *     predicates
 *
 * Pattern and guard syntax remains owned by:
 *
 *     grammar/expressions/
 *
 * No AI-specific pattern language is introduced.
 *
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Reasoning can consume quantum-derived values:
 *
 *     infer result from measurement_result;
 *
 *     reason decision from quantum_result;
 *
 *     deduce property from measured_state;
 *
 * The AI reasoning grammar does not define:
 *
 *     qubits
 *     gates
 *     operations
 *     measurement
 *     routing
 *     scheduling
 *     calibration
 *     QEC
 *     ZQN
 *     HAL
 *
 * Quantum semantics remain owned by:
 *
 *     grammar/quantum/
 *
 * and eventually:
 *
 *     quantum::ir
 *
 * There is no AI-specific quantum IR.
 *
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Reasoning may consume values produced by:
 *
 *     HDL
 *     hardware
 *     accelerators
 *     embedded systems
 *     host/device systems
 *
 * The reasoning grammar remains independent of:
 *
 *     register width
 *     bus width
 *     signal count
 *     device count
 *     CPU count
 *     GPU count
 *     FPGA capacity
 *     accelerator capacity
 *     physical topology
 *
 *
 * ============================================================================
 * DISTRIBUTED / MULTI-AGENT BOUNDARY
 * ============================================================================
 *
 * Reasoning can operate within:
 *
 *     actors
 *     agents
 *     tasks
 *     distributed computations
 *     services
 *
 * but this file does not create a second actor or agent model.
 *
 * Existing concurrency/distributed grammars remain authoritative.
 *
 *
 * ============================================================================
 * FFI / ABI BOUNDARY
 * ============================================================================
 *
 * Reasoning may consume results from foreign functions or ABI-backed
 * components.
 *
 * FFI/ABI syntax remains owned by:
 *
 *     grammar/interoperability/
 *
 * Any foreign/native effect is determined semantically.
 *
 *
 * ============================================================================
 * METAPROGRAMMING BOUNDARY
 * ============================================================================
 *
 * Reasoning may appear in source generated or analyzed by controlled
 * metaprogramming.
 *
 * Reflection, quotation, compile-time evaluation and code generation remain
 * owned by:
 *
 *     grammar/metaprogramming/
 *
 * This file introduces no reflective execution.
 *
 *
 * ============================================================================
 * CANONICAL IR CONTRACT
 * ============================================================================
 *
 * This grammar owns NO IR.
 *
 * The reasoning operation enters the domain-neutral semantic model through
 * the existing universal reasoning representation.
 *
 * Later semantic lowering may select:
 *
 *     classical realization
 *     model inference
 *     symbolic reasoning
 *     probabilistic reasoning
 *     distributed reasoning
 *     quantum/classical reasoning
 *     hardware-assisted reasoning
 *     simulation
 *     future realization
 *
 * The selected realization is target-independent until backend lowering.
 *
 *
 * ============================================================================
 * QUANTUM IR CONTRACT
 * ============================================================================
 *
 * If reasoning depends on or causes quantum computation, the quantum portion
 * must cross:
 *
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
 * This file must never introduce:
 *
 *     AIQuantumIR
 *     ReasoningQuantumIR
 *     InferenceQuantumIR
 *
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY CONTRACT
 * ============================================================================
 *
 * This adapter introduces NO finite language-level limits.
 *
 * It must not contain constants or grammar constructs representing limits on:
 *
 *     reasoning operations
 *     reasoning declarations
 *     inference operations
 *     deductions
 *     premises
 *     evidence
 *     contexts
 *     models
 *     datasets
 *     tensors
 *     tensor rank
 *     quantum resources
 *     CPU resources
 *     GPU resources
 *     FPGA resources
 *     accelerator resources
 *     devices
 *     nodes
 *     threads
 *     memory
 *     storage
 *     network size
 *     topology size
 *
 * Explicitly prohibited examples include:
 *
 *     MAX_REASONING
 *     MAX_INFERENCES
 *     MAX_DEDUCTIONS
 *     MAX_PREMISES
 *     MAX_EVIDENCE
 *     MAX_CONTEXT
 *     MAX_MODELS
 *     MAX_TENSOR_RANK
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * Repetition and complexity are therefore bounded only by implementation,
 * compilation, execution and available resources.
 *
 * "Infinity" means no artificial language ceiling, not physically infinite
 * hardware.
 *
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing through this adapter is deterministic.
 *
 * Given identical:
 *
 *     source
 *     lexer version
 *     grammar version
 *     compatibility configuration
 *
 * the resulting parse structure must be equivalent.
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware availability
 *     runtime state
 *     scheduler state
 *     network state
 *     filesystem state
 *     wall-clock time
 *     randomness
 *     environment state
 *     target selection
 *
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Parser diagnostics originate from the canonical ReasonStatements grammar.
 *
 * This adapter must not reinterpret parser errors as AI semantic errors.
 *
 * Semantic diagnostics may include:
 *
 *     unresolved reasoning target
 *     unresolved reasoning source
 *     invalid target type
 *     invalid source type
 *     missing capability
 *     unsatisfied resource requirement
 *     forbidden effect
 *     policy violation
 *     contract violation
 *     provenance violation
 *     nondeterministic operation where deterministic execution is required
 *
 * These are semantic diagnostics, not parser diagnostics.
 *
 *
 * ============================================================================
 * POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * AI composition must accept universal reasoning forms equivalent to:
 *
 *     infer hypothesis;
 *
 *     deduce conclusion;
 *
 *     reason decision;
 *
 *     infer answer from evidence;
 *
 *     deduce result from premises;
 *
 *     reason decision from observations;
 *
 *     infer answer with (model);
 *
 *     reason conclusion with (evidence, policy);
 *
 *     infer result from measurement_result with (confidence);
 *
 *     deduce property from quantum_result;
 *
 *     reason decision from distributed_result;
 *
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The following remain invalid through this adapter when the underlying
 * ReasonStatements grammar rejects them:
 *
 *     infer;
 *
 *     deduce;
 *
 *     reason;
 *
 *     infer from source;
 *
 *     deduce from source;
 *
 *     reason with ();
 *
 *     reason target with (context) from source;
 *
 *     reason target from;
 *
 *     infer target from;
 *
 *     deduce target with ();
 *
 * The adapter must not add alternate spellings that bypass the canonical
 * reasoning grammar.
 *
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Test reasoning composition with:
 *
 *     model expressions
 *     tensor expressions
 *     dataset expressions
 *     knowledge queries
 *     learned results
 *     probabilistic values
 *     uncertain values
 *     evidence
 *     provenance
 *     policy expressions
 *     contract-controlled execution
 *     classical values
 *     quantum measurement values
 *     hybrid values
 *     HDL-derived values
 *     hardware-derived values
 *     distributed values
 *     network-derived values
 *     simulation results
 *     foreign-function results
 *     generated values
 *
 * The same reasoning syntax must remain valid regardless of the eventual
 * realization domain.
 *
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Scale tests across:
 *
 *     reasoning operation count
 *     source expression complexity
 *     target expression complexity
 *     context expression count
 *     nested expression depth
 *     model complexity
 *     data complexity
 *     symbolic complexity
 *     cross-domain composition
 *
 * No test size becomes a language-level constant.
 *
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Canonical reasoning syntax remains defined by:
 *
 *     grammar/statements/reason.g4
 *
 * This adapter does not establish legacy syntax.
 *
 * Historical reasoning spellings must be handled by the compatibility
 * subsystem rather than by adding ambiguous alternatives here.
 *
 *
 * ============================================================================
 * ANTLR / BUILD CONTRACT
 * ============================================================================
 *
 * The generated parser must resolve:
 *
 *     ReasonStatements
 *
 * from the configured ANTLR grammar library path.
 *
 * The canonical build must make available:
 *
 *     grammar/statements/
 *
 * to the ANTLR import search path.
 *
 * No generated source is committed as a substitute for the grammar contract
 * unless repository policy explicitly requires generated artifacts.
 *
 *
 * ============================================================================
 * SAFETY CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no Rust actions
 *     no embedded code
 *     no semantic predicates
 *     no filesystem access
 *     no network access
 *     no hardware access
 *     no runtime execution
 *     no mutable parser-global state
 *
 * The downstream implementation must remain:
 *
 *     Rust 1.97+
 *     Rust 2021
 *     safe Rust only
 *
 *
 * ============================================================================
 * INTEGRATION WITH AI.G4
 * ============================================================================
 *
 * `grammar/ai/ai.g4` should import:
 *
 *     AIReasoning
 *
 * alongside the other canonical AI leaf grammars.
 *
 * Its AI composition should include:
 *
 *     aiReasoningConstruct
 *
 * as one of its domain alternatives.
 *
 * Conceptually:
 *
 *     import
 *         Types,
 *         Expressions,
 *         Statements,
 *         AIModels,
 *         ...
 *         AIReasoning,
 *         ...
 *     ;
 *
 * and:
 *
 *     aiConstruct
 *         : aiModelConstruct
 *         | datasetConstruct
 *         | tensorConstruct
 *         | trainingConstruct
 *         | inferenceConstruct
 *         | aiReasoningConstruct
 *         | agentConstruct
 *         | ...
 *         ;
 *
 * This is the ONLY required AI composition integration.
 *
 *
 * ============================================================================
 * INTEGRATION WITH STATEMENTS.G4
 * ============================================================================
 *
 * No modification to:
 *
 *     grammar/statements/statements.g4
 *
 * is required merely to create this adapter.
 *
 * `Statements` already owns and composes:
 *
 *     ReasonStatements
 *
 * Therefore the universal language remains able to parse reasoning even
 * without entering the AI domain.
 *
 *
 * ============================================================================
 * INTEGRATION WITH INFERENCE.G4
 * ============================================================================
 *
 * `grammar/ai/inference.g4` must NOT import AIReasoning merely to support
 * generic inference.
 *
 * The distinction is:
 *
 *     inference
 *         = model/operation inference capability
 *
 *     reasoning
 *         = generic infer/deduce/reason computational intent
 *
 * They may compose semantically, but their source grammar ownership remains
 * distinct.
 *
 * For example:
 *
 *     infer classifier(input);
 *
 * is an inference operation when accepted by the inference grammar.
 *
 *     infer hypothesis from evidence;
 *
 * is a reasoning operation when accepted by ReasonStatements.
 *
 * If a surface form is intentionally shared in a future language version,
 * that ambiguity must be resolved at the universal composition layer rather
 * than by creating competing AI grammars.
 *
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when all of the following are true:
 *
 *     [x] It has exactly one purpose: AI reasoning composition.
 *     [x] It defines no duplicate reasoning syntax.
 *     [x] It imports ReasonStatements.
 *     [x] It exposes aiReasoningConstruct.
 *     [x] It exposes aiReasoningStatement.
 *     [x] It has no lexer rules.
 *     [x] It has no semantic actions.
 *     [x] It has no target-specific assumptions.
 *     [x] It has no physical capacity limits.
 *     [x] It introduces no AI-specific type system.
 *     [x] It introduces no AI-specific IR.
 *     [x] It preserves the quantum::ir boundary.
 *     [x] It composes with classical computation.
 *     [x] It composes with quantum-derived values.
 *     [x] It composes with hybrid computation.
 *     [x] It composes with distributed computation.
 *     [x] It composes with HDL/hardware-derived values.
 *     [x] It composes with evidence/provenance.
 *     [x] It composes with contracts and policies.
 *     [x] It remains deterministic.
 *     [x] It requires only safe Rust downstream.
 *     [x] It introduces no application-specific keyword explosion.
 *
 *
 * ============================================================================
 * FINAL ARCHITECTURAL INVARIANT
 * ============================================================================
 *
 * The universal reasoning architecture is:
 *
 *     reasonStatement
 *            |
 *            +--------------------------+
 *            |                          |
 *            v                          v
 *       Statements                    AIReasoning
 *            |                          |
 *            |                          v
 *            |                     aiConstruct
 *            |                          |
 *            +------------+-------------+
 *                         |
 *                         v
 *                  domain-neutral AST
 *                         |
 *                         v
 *                  semantic reasoning
 *                         |
 *              +----------+----------+
 *              |          |          |
 *              v          v          v
 *          classical   quantum    hybrid/AI
 *                         |
 *                         v
 *                    quantum::ir
 *                         |
 *                         v
 *                 target-independent
 *                 optimization/lowering
 *                         |
 *                         v
 *                  target realization
 *
 * This preserves one language, one reasoning model, one AST representation,
 * and one semantic path while allowing reasoning to participate in every
 * computational domain supported by Zamani.
 *
 * ============================================================================
 */

aiReasoningConstruct
    : reasonStatement
    ;

aiReasoningStatement
    : reasonStatement
    ;
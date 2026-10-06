/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/ai/evidence.g4
 *
 * Grammar:
 *     AIEvidence
 *
 * Status:
 *     CANONICAL AI-DOMAIN EVIDENCE COMPOSITION ADAPTER
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
 * This file provides the AI-domain composition boundary for the universal
 * Zamani evidence expression.
 *
 * Evidence itself is NOT AI-specific.
 *
 * The canonical source-level evidence syntax is owned by:
 *
 *     grammar/expressions/evidence.g4
 *
 * Grammar:
 *
 *     EvidenceExpressions
 *
 * Public rule:
 *
 *     evidenceExpression
 *
 * This file therefore acts only as an AI-domain adapter.
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
 *     ZamaniParser
 *          |
 *          v
 *     AI
 *          |
 *          v
 *     AIEvidence                    <-- THIS FILE
 *          |
 *          v
 *     EvidenceExpressions
 *          |
 *          v
 *     evidenceExpression
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic validation
 *          |
 *          +------------------------------+
 *          |        |        |             |
 *          v        v        v             v
 *        types   effects capabilities resources
 *          |        |        |             |
 *          +--------+--------+-------------+
 *                   |
 *                   v
 *               contracts
 *                   |
 *                   v
 *                policies
 *                   |
 *                   v
 *              provenance
 *                   |
 *                   v
 *            canonical semantics
 *                   |
 *          +--------+---------+
 *          |                  |
 *          v                  v
 *      classical          quantum semantic
 *                              |
 *                              v
 *                          quantum::ir
 *
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns exactly ONE public parser boundary:
 *
 *     aiEvidenceConstruct
 *
 * It also owns no evidence syntax.
 *
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     evidenceExpression
 *     evidenceArgumentList
 *     evidenceArgument
 *     evidenceNamedArgument
 *     evidencePositionalArgument
 *     evidenceFieldName
 *     expressions
 *     expression precedence
 *     identifiers
 *     qualified names
 *     literals
 *     types
 *     uncertainty
 *     probability
 *     provenance
 *     assertions
 *     contracts
 *     policies
 *     reasoning
 *     inference
 *     knowledge
 *     learning
 *     adaptation
 *     causality
 *     explanations
 *     decisions
 *     verification
 *     proof systems
 *     quantum syntax
 *     HDL syntax
 *     hardware syntax
 *     resource syntax
 *     capability syntax
 *     effect syntax
 *     security syntax
 *     runtime execution
 *     target selection
 *     IR construction.
 *
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * The following ownership is mandatory:
 *
 *     grammar/expressions/evidence.g4
 *         |
 *         +--> universal evidence expression syntax
 *
 *     grammar/ai/evidence.g4
 *         |
 *         +--> AI composition only
 *
 *     grammar/validation/evidence.g4
 *         |
 *         +--> validation/conformance facade only
 *
 *     grammar/expressions/provenance.g4
 *         |
 *         +--> provenance expression syntax
 *
 *     grammar/expressions/uncertainty.g4
 *         |
 *         +--> uncertainty expression syntax
 *
 * No file may create a competing AI-specific evidence syntax.
 *
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/expressions/evidence.g4
 *
 * IMPORTS:
 *
 *     EvidenceExpressions
 *
 * EXPORTS:
 *
 *     aiEvidenceConstruct
 *
 * CONSUMED_BY:
 *
 *     grammar/ai/ai.g4
 *
 * AST_OWNER:
 *
 *     Existing domain-neutral frontend AST.
 *
 * SEMANTIC_OWNER:
 *
 *     Universal semantic evidence subsystem,
 *     with AI semantic analysis as a consumer.
 *
 * IR_OWNER:
 *
 *     Canonical semantic representation.
 *
 *     Quantum-related evidence reaches quantum::ir only after semantic
 *     classification and quantum lowering.
 *
 * TEST_OWNER:
 *
 *     grammar/tests/ai/evidence/
 *     grammar/tests/semantic/evidence/
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/evidence.md
 *     grammar/spec/ai.md
 *
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This adapter creates NO AI-specific evidence AST.
 *
 * The AST must represent the underlying:
 *
 *     evidenceExpression
 *
 * rather than:
 *
 *     AIEvidence
 *     QuantumEvidence
 *     ModelEvidence
 *     KnowledgeEvidence
 *
 * AI composition must not change the canonical evidence representation.
 *
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * AI semantic analysis may interpret evidence associated with:
 *
 *     inference
 *     reasoning
 *     deduction
 *     induction
 *     abduction
 *     knowledge
 *     learning
 *     adaptation
 *     decisions
 *     explanations
 *     agents
 *     models
 *     datasets
 *     uncertainty
 *     causality
 *
 * However, evidence remains semantically usable outside AI.
 *
 * The AI subsystem must therefore consume the universal evidence model rather
 * than define a separate evidence model.
 *
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * This file defines no types.
 *
 * Evidence values use the canonical Zamani type system.
 *
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * This file defines no effects.
 *
 * Semantic resolution may determine effects such as:
 *
 *     observation
 *     IO
 *     network
 *     measurement
 *     simulation
 *     foreign
 *     distributed
 *     native
 *
 * based on the resolved evidence values/providers.
 *
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * AI evidence may require symbolic capabilities such as:
 *
 *     evidence.read
 *     evidence.create
 *     evidence.verify
 *     provenance.read
 *     provenance.record
 *     model.inspect
 *     knowledge.read
 *     observation.read
 *
 * Capability satisfaction remains owned by the central capability/resource
 * system.
 *
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * This adapter introduces no resource limits.
 *
 * It does not encode:
 *
 *     model size
 *     dataset size
 *     evidence count
 *     source count
 *     argument count
 *     memory
 *     CPUs
 *     GPUs
 *     FPGAs
 *     QPUs
 *     nodes
 *     threads
 *     tensor rank
 *     network size.
 *
 * All resource feasibility is downstream.
 *
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * AI evidence may participate in:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *     assert
 *
 * This adapter does not redefine those constructs.
 *
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * AI evidence may be constrained by:
 *
 *     trust policy
 *     provenance policy
 *     model policy
 *     security policy
 *     disclosure policy
 *     verification policy
 *     adaptation policy
 *
 * Policy evaluation is semantic/runtime behavior.
 *
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Evidence may contain or reference canonical provenance expressions.
 *
 * This adapter does not redefine provenance.
 *
 * The canonical provenance expression remains:
 *
 *     grammar/expressions/provenance.g4
 *
 *
 * ============================================================================
 * UNCERTAINTY CONTRACT
 * ============================================================================
 *
 * Evidence may contain canonical uncertainty values.
 *
 * This adapter does not redefine:
 *
 *     uncertaintyExpression
 *
 * That remains owned by:
 *
 *     grammar/expressions/uncertainty.g4
 *
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * AI evidence may refer to:
 *
 *     quantum measurement
 *     quantum inference
 *     quantum learning
 *     quantum verification
 *     hybrid computation
 *
 * This file defines none of that quantum syntax.
 *
 * If an evidence value is quantum-derived, semantic lowering eventually uses:
 *
 *     quantum::ir
 *
 * There is no:
 *
 *     AIQuantumEvidenceIR
 *     QuantumEvidenceIR
 *
 * introduced by this adapter.
 *
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * AI evidence may refer to HDL or hardware observations through ordinary
 * expressions.
 *
 * Hardware realization remains outside this file.
 *
 *
 * ============================================================================
 * BACKEND BOUNDARY
 * ============================================================================
 *
 * This file selects no:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     simulator
 *     cluster
 *     cloud
 *     future device.
 *
 * Target realization remains downstream.
 *
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source
 *     lexer
 *     grammar
 *     parser configuration
 *     language/compatibility version.
 *
 * Parsing MUST NOT depend on:
 *
 *     model execution
 *     hardware availability
 *     resource availability
 *     network state
 *     runtime state
 *     randomness
 *     wall-clock time.
 *
 *
 * ============================================================================
 * SAFETY CONTRACT
 * ============================================================================
 *
 * This file contains:
 *
 *     no embedded Rust
 *     no semantic predicates
 *     no runtime actions
 *     no filesystem access
 *     no network access
 *     no hardware access
 *     no unsafe Rust.
 *
 * Generated Rust must remain compatible with Rust 1.97+ and safe Rust.
 *
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no physical-capacity constants.
 *
 * In particular it does not introduce:
 *
 *     MAX_EVIDENCE
 *     MAX_MODELS
 *     MAX_DATASETS
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_NETWORK_SIZE
 *
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE:
 *
 *     evidence(value)
 *
 *     evidence(claim: value)
 *
 *     evidence(claim: value, source: source_value)
 *
 *     evidence(
 *         claim: result,
 *         source: source_value,
 *         confidence: confidence_value
 *     )
 *
 *     evidence(
 *         model_result,
 *         measurement_source,
 *         provenance_value
 *     )
 *
 * NEGATIVE:
 *
 *     evidence()
 *
 * The underlying expression grammar owns malformed-expression diagnostics.
 *
 * BOUNDARY:
 *
 *     AI inference result
 *     reasoning result
 *     knowledge query result
 *     learning result
 *     adaptation result
 *     uncertainty result
 *     quantum measurement result
 *     hybrid result
 *     distributed result
 *     hardware observation
 *     simulation result
 *
 * SCALABILITY:
 *
 *     arbitrary evidence arguments;
 *     arbitrary nested expressions;
 *     arbitrary semantic evidence fields;
 *     no machine-capacity ceiling;
 *     no fixed model size;
 *     no fixed dataset size;
 *     no fixed evidence count.
 *
 *
 * ============================================================================
 * INTEGRATION
 * ============================================================================
 *
 * Upstream:
 *
 *     grammar/expressions/evidence.g4
 *
 * AI composition:
 *
 *     grammar/ai/ai.g4
 *
 * Universal expression composition:
 *
 *     grammar/expressions/expressions.g4
 *
 * Validation:
 *
 *     grammar/validation/evidence.g4
 *
 * Provenance:
 *
 *     grammar/expressions/provenance.g4
 *
 * Uncertainty:
 *
 *     grammar/expressions/uncertainty.g4
 *
 * Semantic consumers:
 *
 *     grammar/types/
 *     grammar/effects/
 *     grammar/resources/
 *     grammar/validation/
 *     grammar/security/
 *     grammar/policies/
 *     grammar/ai/
 *     grammar/quantum/
 *     grammar/hybrid/
 *     grammar/hdl/
 *     grammar/hardware/
 *     grammar/distributed/
 *     grammar/interoperability/
 *
 * Canonical IR:
 *
 *     canonical semantic representation
 *
 * Quantum IR boundary:
 *
 *     quantum::ir
 *
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [x] It is an ANTLR parser grammar.
 *
 * [x] It exposes exactly one AI public rule.
 *
 * [x] It defines no evidence syntax.
 *
 * [x] It delegates to EvidenceExpressions.
 *
 * [x] It defines no AI-specific evidence AST.
 *
 * [x] It defines no AI-specific evidence IR.
 *
 * [x] It defines no physical resource limits.
 *
 * [x] It defines no target-specific syntax.
 *
 * [x] It defines no quantum-specific syntax.
 *
 * [x] It defines no hardware-specific syntax.
 *
 * [x] It contains no embedded Rust.
 *
 * [x] It contains no unsafe implementation requirement.
 *
 * Repository integration is complete when:
 *
 * [ ] EvidenceExpressions is imported by the expression composition root.
 *
 * [ ] AIEvidence is imported by AI.
 *
 * [ ] aiConstruct contains aiEvidenceConstruct.
 *
 * [ ] The same domain-neutral AST representation is used.
 *
 * [ ] Evidence participates in semantic validation.
 *
 * [ ] Evidence participates in type checking.
 *
 * [ ] Evidence participates in effect analysis.
 *
 * [ ] Evidence participates in capability analysis.
 *
 * [ ] Evidence participates in resource analysis.
 *
 * [ ] Evidence participates in contract analysis.
 *
 * [ ] Evidence participates in policy analysis.
 *
 * [ ] Evidence participates in provenance.
 *
 * [ ] Quantum-derived evidence reaches quantum::ir where applicable.
 *
 * [ ] Positive tests pass.
 *
 * [ ] Negative tests pass.
 *
 * [ ] Boundary tests pass.
 *
 * [ ] Scalability tests pass.
 *
 * ============================================================================
 * PUBLIC RULE
 * ============================================================================
 */

aiEvidenceConstruct
    : evidenceExpression
    ;
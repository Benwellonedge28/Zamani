/*
 * ============================================================================
 * ZAMANI UNIVERSAL COMPUTING LANGUAGE
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/quantum/adaptive-quantum-execution.g4
 *
 * GRAMMAR
 * -------
 * QuantumAdaptiveExecution
 *
 * STATUS
 * ------
 * CANONICAL QUANTUM ADAPTIVE-EXECUTION INTEGRATION GRAMMAR
 *
 * LANGUAGE
 * --------
 * Zamani
 *
 * ANTLR
 * -----
 * ANTLR4 parser grammar
 *
 * RUST IMPLEMENTATION BASELINE
 * -----------------------------
 * Rust 1.97+
 * Rust 2021 edition
 * safe Rust only
 * unsafe Rust is not required and MUST NOT be required
 *
 * ============================================================================
 * 1. PURPOSE
 * ============================================================================
 *
 * This file is the quantum-domain integration boundary for adaptive execution.
 *
 * It does NOT create a new adaptive-execution language.
 *
 * It integrates the already canonical:
 *
 *     grammar/execution/adaptive.g4
 *
 * whose grammar is:
 *
 *     AdaptiveExpressions
 *
 * and the already canonical:
 *
 *     grammar/execution/resilience.g4
 *
 * whose grammar is:
 *
 *     Resilience
 *
 * The source-level adaptive operation remains:
 *
 *     adapt(target)
 *     adapt(target, context)
 *     adapt(target, context_a, context_b, ...)
 *
 * This file gives that universal operation a quantum-domain composition
 * boundary without changing its syntax or semantics.
 *
 * ============================================================================
 * 2. CORE ARCHITECTURAL RULE
 * ============================================================================
 *
 * Quantum adaptive execution means:
 *
 *     adaptive computational intent
 *              |
 *              v
 *     quantum semantic interpretation
 *              |
 *              v
 *     domain-neutral AST
 *              |
 *              v
 *     semantic validation
 *              |
 *       +------+-------+-------+-------+-------+
 *       |      |       |       |       |       |
 *      type  effect capability resource policy provenance
 *       |      |       |       |       |       |
 *       +------+-------+-------+-------+-------+
 *              |
 *              v
 *          quantum::ir
 *              |
 *              v
 *        optimization
 *              |
 *              v
 *        decomposition
 *              |
 *              v
 *          routing
 *              |
 *              v
 *         scheduling
 *              |
 *              v
 *       resilience / QEC
 *              |
 *              v
 *             ZQN
 *              |
 *              v
 *             HAL
 *              |
 *              v
 *       target realization
 *
 * This file MUST NOT implement any stage below the semantic quantum boundary.
 *
 * ============================================================================
 * 3. SINGLE-OWNER RULE
 * ============================================================================
 *
 * This file OWNS:
 *
 *     quantumAdaptiveExecution
 *     quantumAdaptiveExecutionContext
 *     quantumAdaptiveExecutionResilience
 *     quantumAdaptiveExecutionItem
 *
 * This file DOES NOT OWN:
 *
 *     adapt(...)
 *     adaptiveExpression
 *     adaptiveOperation
 *     adaptiveArgumentList
 *     adaptiveTarget
 *     adaptiveContext
 *
 * Those remain owned by:
 *
 *     grammar/execution/adaptive.g4
 *
 * This file also does not own:
 *
 *     resilienceDeclaration
 *     resilienceBlock
 *     resilienceClause
 *
 * Those remain owned by:
 *
 *     grammar/execution/resilience.g4
 *
 * ============================================================================
 * 4. WHY THIS FILE EXISTS
 * ============================================================================
 *
 * A generic adaptive expression may occur in many domains:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     AI/ML
 *     distributed
 *     networking
 *     simulation
 *     compilation
 *
 * The universal adaptation grammar therefore cannot assume that every
 * adaptation is quantum.
 *
 * This file provides the explicit quantum-domain composition point.
 *
 * It allows the semantic/frontend composition layer to recognize:
 *
 *     quantum adaptive execution
 *
 * without adding another spelling of:
 *
 *     adapt(...)
 *
 * and without creating a second quantum execution IR.
 *
 * ============================================================================
 * 5. NO NEW KEYWORD
 * ============================================================================
 *
 * No new universal keyword such as:
 *
 *     ADAPTIVE_QUANTUM_EXECUTION
 *     ADAPTIVE_QUANTUM
 *     QUANTUM_ADAPT
 *     ADAPTIVE_EXECUTION
 *
 * is introduced here.
 *
 * This is deliberate.
 *
 * A keyword for every computational combination would create an unbounded
 * keyword explosion and would make the language less extensible.
 *
 * The domain is established by semantic context, not by multiplying
 * keywords.
 *
 * ============================================================================
 * 6. DEPENDENCIES
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 *     AdaptiveExpressions
 *         grammar/execution/adaptive.g4
 *
 *     Resilience
 *         grammar/execution/resilience.g4
 *
 *     canonical Zamani lexer vocabulary
 *
 *         grammar/antlr/ZamaniLexer.g4
 *
 *     canonical expression hierarchy
 *
 *         grammar/expressions/*
 *
 *     canonical quantum composition
 *
 *         grammar/quantum/quantum.g4
 *
 *     quantum resources/capabilities
 *
 *         grammar/quantum/quantum-resources.g4
 *         grammar/quantum/quantum-capabilities.g4
 *
 * These dependencies are syntactic composition boundaries only.
 *
 * ============================================================================
 * 7. EXPORTS
 * ============================================================================
 *
 * PUBLIC RULES
 * ------------
 *
 *     quantumAdaptiveExecution
 *     quantumAdaptiveExecutionContext
 *     quantumAdaptiveExecutionResilience
 *     quantumAdaptiveExecutionItem
 *
 * The canonical public integration rule is:
 *
 *     quantumAdaptiveExecution
 *
 * Parent quantum grammar should import this grammar exactly once.
 *
 * ============================================================================
 * 8. AST CONTRACT
 * ============================================================================
 *
 * This grammar does not create Rust AST structures.
 *
 * The conceptual domain-neutral representation is equivalent to:
 *
 *     QuantumAdaptiveExecution {
 *         adaptation,
 *         resilience,
 *         source_span
 *     }
 *
 * where:
 *
 *     adaptation
 *
 * is represented by the existing generic adaptive-expression AST.
 *
 * The AST MUST NOT contain:
 *
 *     physical CPU identifiers
 *     physical GPU identifiers
 *     physical FPGA identifiers
 *     physical ASIC identifiers
 *     physical QPU identifiers
 *     physical qubit mappings
 *     coupling maps
 *     routing decisions
 *     scheduler decisions
 *     calibration
 *     pulse information
 *     vendor-specific hardware state
 *     QEC layout
 *
 * Those belong downstream.
 *
 * ============================================================================
 * 9. SEMANTIC CONTRACT
 * ============================================================================
 *
 * The semantic layer determines whether an adaptive expression is actually
 * quantum-related.
 *
 * Examples:
 *
 *     adapt(strategy, measurement_result)
 *
 *     adapt(plan, quantum_result)
 *
 *     adapt(circuit_strategy, observation)
 *
 *     adapt(hybrid_strategy, quantum_measurement)
 *
 * may all become quantum adaptive execution depending on their enclosing
 * semantic context and resolved types/operations.
 *
 * This grammar does not guess that relationship.
 *
 * Semantic analysis MUST determine:
 *
 *     domain
 *     operation meaning
 *     operand compatibility
 *     result type
 *     effects
 *     capabilities
 *     resource requirements
 *     contracts
 *     policies
 *     provenance
 *     determinism requirements
 *     resilience requirements
 *
 * ============================================================================
 * 10. CANONICAL QUANTUM IR
 * ============================================================================
 *
 * There MUST be exactly one canonical quantum semantic IR boundary:
 *
 *     quantum::ir
 *
 * This file MUST NOT introduce:
 *
 *     QuantumAdaptiveExecutionIR
 *     AdaptiveQuantumIR
 *     QuantumAdaptiveIR
 *     QuantumExecutionIR
 *     QuantumResilienceIR
 *
 * as competing semantic representations.
 *
 * The required path is:
 *
 *     source
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic model
 *       |
 *       v
 *     quantum::ir
 *
 * Existing adaptive information may be represented as attributes,
 * operations, control dependencies, effects, requirements, policies,
 * provenance, or other canonical quantum::ir structures as determined by
 * the semantic/IR layer.
 *
 * ============================================================================
 * 11. QUANTUM OPERATIONS REMAIN OPEN-WORLD
 * ============================================================================
 *
 * This file MUST NOT enumerate:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     S
 *     T
 *     CNOT
 *     CX
 *     CZ
 *     SWAP
 *     RX
 *     RY
 *     RZ
 *
 * or any other finite operation catalogue.
 *
 * Quantum operation identity remains owned by:
 *
 *     grammar/quantum/operations.g4
 *
 * and the corresponding semantic operation resolver.
 *
 * New operations therefore do not require modifying this file.
 *
 * ============================================================================
 * 12. RESILIENCE INTEGRATION
 * ============================================================================
 *
 * Existing resilience syntax is reused.
 *
 * Canonical resilience states include:
 *
 *     Unknown
 *     Healthy
 *     Degraded
 *     Unstable
 *     Unavailable
 *     Recovering
 *     Quarantined
 *     Retired
 *
 * Canonical resilience outcomes include:
 *
 *     ACCEPT
 *     DEGRADED_ACCEPT
 *     RETRY
 *     RECOVER
 *     ESCALATE
 *     REJECT
 *
 * These values are semantic concepts.
 *
 * They do not define a fixed hardware topology or resource limit.
 *
 * This grammar does not redefine those values.
 *
 * ============================================================================
 * 13. ADAPTIVE EXECUTION SEMANTICS
 * ============================================================================
 *
 * Adaptive quantum execution MAY use information obtained from:
 *
 *     measurement
 *     observation
 *     classical feed-forward
 *     quantum state analysis
 *     error detection
 *     error correction
 *     resilience state
 *     resource observations
 *     capability observations
 *     simulation
 *     learning
 *     reasoning
 *     evidence
 *     provenance
 *     policy
 *     previous execution results
 *     future domain information
 *
 * None of those mechanisms is redefined here.
 *
 * They are represented by existing expressions and semantic models.
 *
 * ============================================================================
 * 14. CONTROLLED ADAPTATION
 * ============================================================================
 *
 * Adaptive execution MUST NOT mean unrestricted self-modification.
 *
 * Parsing this construct does NOT authorize:
 *
 *     source-code mutation
 *     executable-code mutation
 *     compiler mutation
 *     policy mutation
 *     credential mutation
 *     protected-memory mutation
 *     hardware mutation
 *     unauthorized network operations
 *     unrestricted runtime mutation
 *
 * Authorization remains downstream.
 *
 * Required semantic sequence:
 *
 *     adaptation intent
 *          |
 *          v
 *     type validation
 *          |
 *          v
 *     effect validation
 *          |
 *          v
 *     capability validation
 *          |
 *          v
 *     resource validation
 *          |
 *          v
 *     contract validation
 *          |
 *          v
 *     policy authorization
 *          |
 *          v
 *     provenance requirements
 *          |
 *          v
 *     semantic adaptation
 *          |
 *          v
 *     result validation
 *
 * ============================================================================
 * 15. RESOURCE CONTRACT
 * ============================================================================
 *
 * Adaptive quantum execution may require arbitrary resources.
 *
 * Examples include:
 *
 *     qubits
 *     logical quantum resources
 *     physical quantum resources
 *     classical compute
 *     memory
 *     storage
 *     communication
 *     accelerator resources
 *     simulation resources
 *     error-correction resources
 *     measurement resources
 *     control resources
 *     energy
 *     time
 *     bandwidth
 *     future resource classes
 *
 * This grammar contains no resource limits.
 *
 * It MUST NOT define:
 *
 *     MAX_QUBITS
 *     MAX_LOGICAL_QUBITS
 *     MAX_PHYSICAL_QUBITS
 *     MAX_QPUS
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
 *     MAX_ADAPTATIONS
 *     MAX_RECOVERIES
 *     MAX_RETRIES
 *
 * Program-specific numeric requirements remain valid source semantics.
 *
 * They are not universal implementation ceilings.
 *
 * ============================================================================
 * 16. CAPABILITY CONTRACT
 * ============================================================================
 *
 * Capability requirements remain open-world.
 *
 * Examples may include:
 *
 *     quantum.measurement
 *     quantum.mid_circuit_measurement
 *     quantum.dynamic_control
 *     quantum.feed_forward
 *     quantum.error_detection
 *     quantum.error_correction
 *     quantum.adaptive_execution
 *     quantum.classical_control
 *     tensor.compute
 *     distributed.compute
 *
 * This file does not create a closed capability catalogue.
 *
 * Capability resolution belongs to the resource/capability semantic system.
 *
 * ============================================================================
 * 17. EFFECT CONTRACT
 * ============================================================================
 *
 * Adaptive quantum execution may semantically carry effects such as:
 *
 *     quantum
 *     measurement
 *     adaptation
 *     mutation
 *     randomness
 *     simulation
 *     distributed
 *     network
 *     native
 *     foreign
 *     runtime_control
 *
 * The actual effect set depends on semantic resolution.
 *
 * Parsing performs no runtime effect.
 *
 * ============================================================================
 * 18. CONTRACT INTEGRATION
 * ============================================================================
 *
 * Adaptive quantum execution participates in the universal contract model.
 *
 * Applicable semantic contracts include:
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
 * This file does not duplicate it.
 *
 * ============================================================================
 * 19. POLICY INTEGRATION
 * ============================================================================
 *
 * Adaptive execution may be constrained by:
 *
 *     execution policy
 *     quantum policy
 *     resilience policy
 *     security policy
 *     resource policy
 *     deployment policy
 *     adaptation policy
 *     simulation policy
 *
 * Policy syntax remains owned by:
 *
 *     grammar/policies/
 *     grammar/execution/policies.g4
 *
 * This file does not grant authorization.
 *
 * ============================================================================
 * 20. PROVENANCE CONTRACT
 * ============================================================================
 *
 * Adaptive quantum decisions may depend on:
 *
 *     measurements
 *     observations
 *     learned models
 *     inferred decisions
 *     evidence
 *     policies
 *     resource observations
 *     resilience observations
 *     previous results
 *
 * Therefore provenance MUST remain available to the semantic/execution
 * systems.
 *
 * This file does not create another provenance grammar.
 *
 * The semantic system should be able to record:
 *
 *     source
 *     derived_from
 *     decision
 *     evidence
 *     transformation
 *     verification
 *     execution attempt
 *     resilience state
 *     outcome
 *     implementation identity
 *
 * ============================================================================
 * 21. DETERMINISM
 * ============================================================================
 *
 * Parsing MUST be deterministic.
 *
 * Parsing MUST NOT inspect:
 *
 *     hardware
 *     available QPUs
 *     calibration
 *     runtime state
 *     network state
 *     filesystem state
 *     wall-clock time
 *     randomness
 *     scheduler state
 *     resource availability
 *
 * Those values may influence semantic execution decisions later, but never
 * the syntactic interpretation of the source.
 *
 * ============================================================================
 * 22. POCO-REAF
 * ============================================================================
 *
 * This file is compatible with:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * because it describes computational intent rather than physical realization.
 *
 * The same source structure can therefore be considered for:
 *
 *     minimal quantum systems
 *     embedded quantum-classical systems
 *     simulators
 *     CPU-based simulation
 *     GPU simulation
 *     FPGA acceleration
 *     ASIC realization
 *     QPUs
 *     heterogeneous systems
 *     HPC systems
 *     clusters
 *     distributed systems
 *     cloud systems
 *     future computational substrates
 *
 * without modifying this grammar because machine scale changes.
 *
 * "Infinity" means:
 *
 *     no artificial finite language-level ceiling.
 *
 * It does not claim physically infinite resources.
 *
 * Actual feasibility remains determined by:
 *
 *     target capabilities
 *     available resources
 *     compiler resources
 *     runtime resources
 *     policies
 *     deployment environment
 *
 * ============================================================================
 * 23. SOURCE-LEVEL FORM
 * ============================================================================
 *
 * The canonical source-level adaptive expression remains:
 *
 *     adapt(target)
 *
 *     adapt(target, context)
 *
 *     adapt(target, context_a, context_b, ...)
 *
 * This file deliberately does not introduce another spelling.
 *
 * Quantum semantic context may be established by:
 *
 *     enclosing quantum construct
 *     quantum-typed target
 *     quantum operation result
 *     quantum circuit result
 *     quantum measurement result
 *     quantum capability requirement
 *     quantum resource requirement
 *     semantic domain information
 *
 * ============================================================================
 * 24. QUANTUM ADAPTIVE EXECUTION CONTEXT
 * ============================================================================
 *
 * The context is intentionally generic.
 *
 * It is represented by the existing adaptive expression.
 *
 * This rule exists only as a stable quantum integration point.
 *
 * No new expression precedence hierarchy is introduced.
 */

parser grammar QuantumAdaptiveExecution;

options {
    tokenVocab = ZamaniLexer;
}

import
    AdaptiveExpressions,
    Resilience
;


/*
 * ============================================================================
 * 25. PUBLIC QUANTUM ADAPTIVE EXECUTION ENTRY
 * ============================================================================
 *
 * This is the sole public composition rule owned by this file.
 *
 * The adaptive operation itself remains owned by AdaptiveExpressions.
 *
 * The optional resilience block remains owned by Resilience.
 *
 * Therefore this file composes existing authorities instead of duplicating
 * them.
 *
 * Canonical form:
 *
 *     adapt(target)
 *
 * or:
 *
 *     adapt(target, observation, evidence, ...)
 *
 * with optional resilience intent represented by the canonical resilience
 * grammar when the surrounding parser composition permits it.
 *
 * ============================================================================
 */

quantumAdaptiveExecution
    : adaptiveExpression
      quantumAdaptiveExecutionContext*
    ;


/*
 * ============================================================================
 * 26. ADAPTIVE EXECUTION CONTEXT
 * ============================================================================
 *
 * The context is deliberately extensible.
 *
 * At present the canonical reusable context supplied by this integration
 * boundary is resilience.
 *
 * Future contexts MUST be added through independently owned grammars and
 * composed here rather than redefining existing syntax.
 *
 * ============================================================================
 */

quantumAdaptiveExecutionContext
    : quantumAdaptiveExecutionResilience
    ;


/*
 * ============================================================================
 * 27. RESILIENCE CONTEXT
 * ============================================================================
 *
 * Resilience remains declarative.
 *
 * It does not execute recovery.
 *
 * It does not perform retries.
 *
 * It does not perform routing.
 *
 * It does not schedule.
 *
 * It does not invoke QEC.
 *
 * It does not select hardware.
 *
 * It does not access a QPU.
 *
 * The existing Resilience grammar owns the actual resilience syntax.
 * ============================================================================
 */

quantumAdaptiveExecutionResilience
    : resilienceBlock
    ;


/*
 * ============================================================================
 * 28. SEMANTIC INTEGRATION RULE
 * ============================================================================
 *
 * This rule is intentionally an alias.
 *
 * It provides a stable named boundary for semantic consumers without
 * introducing another syntax tree or IR.
 *
 * ============================================================================
 */

quantumAdaptiveExecutionItem
    : quantumAdaptiveExecution
    ;


/*
 * ============================================================================
 * 29. QUANTUM SEMANTIC CLASSIFICATION
 * ============================================================================
 *
 * The following are semantic examples, not additional grammar productions:
 *
 *     adapt(strategy, measurement_result)
 *
 *     adapt(strategy, quantum_result)
 *
 *     adapt(circuit_plan, observation)
 *
 *     adapt(hybrid_plan, quantum_measurement)
 *
 *     adapt(error_strategy, syndrome_information)
 *
 *     adapt(execution_plan, resilience_state)
 *
 *     adapt(strategy, evidence, provenance)
 *
 * The semantic layer determines whether the expression represents:
 *
 *     quantum adaptive execution
 *
 * rather than this parser grammar making that decision.
 *
 * ============================================================================
 * 30. MEASUREMENT INTEGRATION
 * ============================================================================
 *
 * Measurement syntax remains owned by:
 *
 *     grammar/quantum/measurement.g4
 *
 * Adaptive execution may consume a measurement result through the ordinary
 * expression hierarchy.
 *
 * Example:
 *
 *     adapt(strategy, measurement_result)
 *
 * This file does not redefine measurement.
 *
 * ============================================================================
 * 31. DYNAMIC CONTROL INTEGRATION
 * ============================================================================
 *
 * Dynamic quantum/classical control remains owned by:
 *
 *     grammar/quantum/dynamic-control.g4
 *
 * and related hybrid/control grammars.
 *
 * Adaptive execution may semantically produce or consume decisions that
 * influence dynamic control.
 *
 * This grammar does not duplicate conditional or feed-forward syntax.
 *
 * ============================================================================
 * 32. HYBRID INTEGRATION
 * ============================================================================
 *
 * Hybrid computation remains owned by:
 *
 *     grammar/hybrid/
 *
 * Adaptive quantum execution may therefore participate in:
 *
 *     classical -> quantum
 *     quantum -> classical
 *     measurement -> classical decision
 *     classical decision -> quantum operation
 *     AI -> quantum
 *     quantum -> AI
 *     simulation -> quantum
 *     hardware observation -> quantum strategy
 *
 * The corresponding semantic transformation occurs after parsing.
 *
 * ============================================================================
 * 33. QUANTUM RESOURCE INTEGRATION
 * ============================================================================
 *
 * Quantum resource syntax remains owned by the canonical resource grammars.
 *
 * This file does not define:
 *
 *     qubit counts
 *     register sizes
 *     physical qubit IDs
 *     topology
 *     device count
 *     QPU count
 *     memory capacity
 *
 * Source requirements may instead flow through the existing requirement and
 * capability model.
 *
 * ============================================================================
 * 34. ADAPTIVE RETRY / RECOVERY
 * ============================================================================
 *
 * Retry and recovery are semantic policies.
 *
 * This file does not create a retry loop.
 *
 * A resilience policy may semantically produce:
 *
 *     ACCEPT
 *     DEGRADED_ACCEPT
 *     RETRY
 *     RECOVER
 *     ESCALATE
 *     REJECT
 *
 * The execution/resilience subsystem decides what those outcomes mean.
 *
 * The grammar only preserves the source-level intent.
 *
 * ============================================================================
 * 35. RESILIENCE STATES
 * ============================================================================
 *
 * The canonical semantic resilience state space remains:
 *
 *     Unknown
 *     Healthy
 *     Degraded
 *     Unstable
 *     Unavailable
 *     Recovering
 *     Quarantined
 *     Retired
 *
 * No additional parser-level state machine is created here.
 *
 * Runtime transition legality remains owned by:
 *
 *     src/quantum/resilience/
 *
 * ============================================================================
 * 36. QEC INTEGRATION
 * ============================================================================
 *
 * Adaptive execution may depend upon:
 *
 *     error syndromes
 *     logical reliability
 *     correction results
 *     verification results
 *     resilience observations
 *
 * QEC implementation remains outside grammar.
 *
 * The semantic pipeline remains:
 *
 *     adaptive intent
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     QEC / resilience
 *
 * This file must never create a QEC IR.
 *
 * ============================================================================
 * 37. ROUTING INTEGRATION
 * ============================================================================
 *
 * Adaptive execution may semantically cause a different routing decision.
 *
 * This grammar does not perform routing.
 *
 * Required boundary:
 *
 *     source
 *       |
 *       v
 *     semantic quantum intent
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     routing
 *
 * ============================================================================
 * 38. SCHEDULING INTEGRATION
 * ============================================================================
 *
 * Adaptive execution may change future scheduling decisions.
 *
 * The parser does not schedule.
 *
 * Scheduling remains downstream from:
 *
 *     quantum::ir
 *
 * and the semantic execution plan.
 *
 * ============================================================================
 * 39. HARDWARE INTEGRATION
 * ============================================================================
 *
 * This grammar does not select:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     physical qubit
 *     memory bank
 *     device
 *     vendor
 *
 * Hardware capabilities are negotiated downstream.
 *
 * ============================================================================
 * 40. SIMULATION INTEGRATION
 * ============================================================================
 *
 * The same adaptive quantum source may execute through:
 *
 *     hardware
 *     simulator
 *     emulator
 *     hybrid simulator
 *     distributed simulator
 *
 * Simulation is an execution strategy, not another source language.
 *
 * ============================================================================
 * 41. AI / LEARNING INTEGRATION
 * ============================================================================
 *
 * Adaptive quantum execution may consume:
 *
 *     learning results
 *     inferred strategies
 *     deduced configurations
 *     confidence values
 *     probabilities
 *     evidence
 *     knowledge-query results
 *
 * The corresponding AI grammars remain independent.
 *
 * This file does not create:
 *
 *     quantum learning syntax
 *     quantum reasoning syntax
 *     quantum knowledge syntax
 *
 * It merely provides the quantum adaptive composition boundary.
 *
 * ============================================================================
 * 42. UNCERTAINTY INTEGRATION
 * ============================================================================
 *
 * Adaptive execution may consume uncertainty-bearing expressions.
 *
 * Examples:
 *
 *     adapt(strategy, probability)
 *
 *     adapt(strategy, confidence)
 *
 *     adapt(strategy, distribution)
 *
 *     adapt(strategy, observation)
 *
 * No fixed numerical representation is encoded here.
 *
 * ============================================================================
 * 43. PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Adaptive decisions affecting quantum execution SHOULD preserve sufficient
 * provenance for downstream audit/replay/explanation requirements.
 *
 * Provenance may include:
 *
 *     semantic identity
 *     operation identity
 *     evidence
 *     observation
 *     resilience state
 *     policy
 *     capability snapshot
 *     resource snapshot
 *     selected adaptation
 *     resulting semantic state
 *
 * The grammar itself only preserves the syntactic source structure.
 *
 * ============================================================================
 * 44. EXPLAINABILITY INTEGRATION
 * ============================================================================
 *
 * Adaptive decisions may be explainable through the existing explanation and
 * evidence systems.
 *
 * This grammar does not introduce:
 *
 *     explain_adaptive_quantum
 *
 * or any other specialized keyword.
 *
 * Existing explanation/provenance constructs remain authoritative.
 *
 * ============================================================================
 * 45. SECURITY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no embedded Rust actions
 *     no semantic predicates
 *     no filesystem access
 *     no network access
 *     no hardware discovery
 *     no credential access
 *     no QPU invocation
 *     no simulator invocation
 *     no runtime execution
 *     no randomness
 *     no unsafe code
 *
 * It is purely declarative ANTLR syntax.
 *
 * ============================================================================
 * 46. COMPATIBILITY
 * ============================================================================
 *
 * This grammar uses:
 *
 *     tokenVocab = ZamaniLexer
 *
 * exclusively.
 *
 * It MUST NOT introduce:
 *
 *     ZamaniTokens
 *
 * as an alternative lexical authority.
 *
 * It also does not introduce token aliases for historical token names.
 *
 * ============================================================================
 * 47. CIRCULAR DEPENDENCY PROHIBITION
 * ============================================================================
 *
 * This file MUST NOT be imported by:
 *
 *     grammar/execution/adaptive.g4
 *
 * because AdaptiveExpressions is a dependency of this file.
 *
 * Likewise:
 *
 *     grammar/execution/resilience.g4
 *
 * MUST NOT import this file.
 *
 * The dependency direction is:
 *
 *     AdaptiveExpressions
 *            |
 *            v
 *     Resilience
 *            |
 *            v
 *     QuantumAdaptiveExecution
 *            |
 *            v
 *     quantum composition
 *
 * More precisely, AdaptiveExpressions and Resilience are independent
 * authorities consumed by this integration grammar.
 *
 * ============================================================================
 * 48. PARENT INTEGRATION
 * ============================================================================
 *
 * The canonical quantum composition grammar:
 *
 *     grammar/quantum/quantum.g4
 *
 * should import:
 *
 *     QuantumAdaptiveExecution
 *
 * exactly once.
 *
 * It should expose:
 *
 *     quantumAdaptiveExecution
 *
 * through its quantum-domain dispatch.
 *
 * It MUST NOT recreate:
 *
 *     adapt(...)
 *
 * or:
 *
 *     resilience { ... }
 *
 * syntax.
 *
 * ============================================================================
 * 49. UNIVERSAL PARSER INTEGRATION
 * ============================================================================
 *
 * The universal parser hierarchy remains:
 *
 *     grammar/Zamani.g4
 *             |
 *             v
 *     grammar/antlr/ZamaniParser.g4
 *             |
 *             v
 *     quantum composition
 *             |
 *             v
 *     QuantumAdaptiveExecution
 *
 * This file must not become a second complete-program parser.
 *
 * ============================================================================
 * 50. SOURCE PRESERVATION
 * ============================================================================
 *
 * The parse tree must preserve enough information for:
 *
 *     source spans
 *     diagnostics
 *     AST construction
 *     formatting
 *     IDE/LSP tooling
 *     refactoring
 *     provenance
 *     reproducible compilation
 *     incremental compilation
 *     compatibility analysis
 *
 * ============================================================================
 * 51. DIAGNOSTICS
 * ============================================================================
 *
 * Parser diagnostics cover structural errors only.
 *
 * Examples:
 *
 *     malformed adapt expression
 *     malformed argument list
 *     malformed resilience block
 *     missing delimiters
 *     unexpected token
 *
 * Semantic diagnostics remain downstream:
 *
 *     non-quantum adaptation used in quantum-only context
 *     invalid quantum operand
 *     incompatible quantum types
 *     missing quantum capability
 *     insufficient quantum resources
 *     forbidden effect
 *     forbidden policy
 *     invalid resilience policy
 *     invalid recovery transition
 *     unavailable execution capability
 *     invalid provenance requirement
 *     unsupported target
 *
 * ============================================================================
 * 52. SCALABILITY
 * ============================================================================
 *
 * This grammar imposes no universal finite limit on:
 *
 *     adaptive expressions
 *     adaptive contexts
 *     nested expressions
 *     quantum operations
 *     qubits
 *     logical resources
 *     physical resources
 *     measurements
 *     circuit width
 *     circuit depth
 *     execution regions
 *     resilience clauses
 *     policies
 *     capabilities
 *     requirements
 *     devices
 *     QPUs
 *     CPUs
 *     GPUs
 *     FPGAs
 *     nodes
 *     memory
 *     tensor rank
 *     network size
 *
 * Repetition is expressed using ANTLR repetition operators and ordinary
 * program structure.
 *
 * Concrete parser/runtime limits remain implementation resource limits, not
 * language semantics.
 *
 * ============================================================================
 * 53. NO HARD-CODED PHYSICAL REALIZATION
 * ============================================================================
 *
 * Forbidden in this file:
 *
 *     physical qubit number
 *     physical device number
 *     vendor topology
 *     fixed coupling map
 *     fixed QPU
 *     fixed CPU
 *     fixed GPU
 *     fixed FPGA
 *     fixed memory size
 *     fixed register width
 *     fixed node count
 *
 * A target-specific dialect may represent implementation information elsewhere
 * under an explicitly target-specific contract.
 *
 * That information must never silently become portable quantum semantics.
 *
 * ============================================================================
 * 54. SAFE RUST INTEGRATION
 * ============================================================================
 *
 * The grammar itself contains no Rust code.
 *
 * The generated parser/frontend integration MUST remain compatible with:
 *
 *     Rust 1.97+
 *     Rust 2021
 *     safe Rust
 *
 * No unsafe implementation is required.
 *
 * The grammar must not require:
 *
 *     unsafe blocks
 *     raw-pointer ownership models
 *     FFI-specific parser actions
 *     target-specific native actions
 *
 * ============================================================================
 * 55. TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE
 * --------
 *
 *     adapt(strategy)
 *
 *     adapt(strategy, measurement_result)
 *
 *     adapt(strategy, quantum_result)
 *
 *     adapt(plan, observation, evidence)
 *
 *     adapt(plan, measurement_result, policy)
 *
 *     adapt(strategy, infer(candidate))
 *
 *     adapt(strategy, query_result)
 *
 *     adapt(strategy, learning_result, confidence)
 *
 *     adapt(hybrid_strategy, quantum_result)
 *
 *     adapt(error_strategy, syndrome_information)
 *
 * RESILIENCE
 * ----------
 *
 *     adapt(strategy, measurement_result)
 *     resilience {
 *         policy: fault_tolerant;
 *     }
 *
 *     adapt(strategy, quantum_result)
 *     resilience {
 *         state: Degraded;
 *         outcome: DEGRADED_ACCEPT;
 *     }
 *
 *     adapt(strategy, observation)
 *     resilience {
 *         outcome: RETRY;
 *     }
 *
 *     adapt(strategy, observation)
 *     resilience {
 *         outcome: RECOVER;
 *     }
 *
 * CROSS-DOMAIN
 * ------------
 *
 *     adapt(classical_strategy, quantum_result)
 *
 *     adapt(quantum_strategy, classical_feedback)
 *
 *     adapt(hybrid_strategy, measurement_result)
 *
 *     adapt(quantum_strategy, simulation_result)
 *
 *     adapt(quantum_strategy, distributed_result)
 *
 * NEGATIVE STRUCTURAL CASES
 * -------------------------
 *
 *     adapt()
 *
 *     adapt(,)
 *
 *     adapt(strategy,)
 *
 *     adapt(strategy,,feedback)
 *
 *     adapt(strategy feedback)
 *
 *     adapt(strategy, feedback
 *
 *     adapt(strategy, feedback,)
 *
 * BOUNDARY
 * --------
 *
 *     adapt(adapt(strategy), feedback)
 *
 *     outer(adapt(strategy, measurement_result))
 *
 *     match adapt(strategy, evidence) { ... }
 *
 * SCALABILITY
 * -----------
 *
 *     adapt(target)
 *
 *     adapt(target, a, b, c, d, ...)
 *
 * No grammar-level cardinality ceiling is introduced.
 *
 * ============================================================================
 * 56. CONFORMANCE MATRIX
 * ============================================================================
 *
 * LEXER
 * -----
 *
 * Required canonical tokens are supplied by ZamaniLexer.
 *
 * No tokens are defined here.
 *
 * PARSER
 * ------
 *
 * The following public rule must be reachable from the canonical quantum
 * parser:
 *
 *     quantumAdaptiveExecution
 *
 * AST
 * ---
 *
 * Must preserve:
 *
 *     adaptive expression
 *     optional resilience context
 *     source spans
 *
 * SEMANTICS
 * ---------
 *
 * Must resolve:
 *
 *     quantum domain
 *     adaptation meaning
 *     effects
 *     capabilities
 *     resources
 *     contracts
 *     policies
 *     provenance
 *     resilience requirements
 *
 * IR
 * --
 *
 * Quantum semantics MUST lower through:
 *
 *     quantum::ir
 *
 * No competing quantum adaptive IR is permitted.
 *
 * EXECUTION
 * ---------
 *
 * Downstream systems own:
 *
 *     optimization
 *     decomposition
 *     routing
 *     scheduling
 *     QEC
 *     resilience execution
 *     ZQN
 *     HAL
 *     hardware realization
 *
 * ============================================================================
 * 57. INTEGRATION CHECKLIST
 * ============================================================================
 *
 * [ ] tokenVocab is ZamaniLexer.
 *
 * [ ] AdaptiveExpressions is the sole owner of adapt(...) expression syntax.
 *
 * [ ] Resilience is the sole owner of resilience block syntax.
 *
 * [ ] This file introduces no duplicate adaptive syntax.
 *
 * [ ] This file introduces no duplicate resilience syntax.
 *
 * [ ] This file introduces no quantum operation catalogue.
 *
 * [ ] This file introduces no physical resource model.
 *
 * [ ] This file introduces no hardware allocation.
 *
 * [ ] This file introduces no routing.
 *
 * [ ] This file introduces no scheduling.
 *
 * [ ] This file introduces no QEC implementation.
 *
 * [ ] This file introduces no ZQN implementation.
 *
 * [ ] This file introduces no HAL implementation.
 *
 * [ ] This file introduces no runtime behavior.
 *
 * [ ] This file introduces no competing IR.
 *
 * [ ] quantum::ir remains canonical.
 *
 * [ ] resource analysis remains downstream.
 *
 * [ ] capability analysis remains downstream.
 *
 * [ ] effect analysis remains downstream.
 *
 * [ ] contract analysis remains downstream.
 *
 * [ ] policy analysis remains downstream.
 *
 * [ ] provenance analysis remains downstream.
 *
 * [ ] resilience runtime remains downstream.
 *
 * [ ] no artificial finite hardware ceiling exists.
 *
 * [ ] no artificial finite retry ceiling exists.
 *
 * [ ] no artificial finite adaptation ceiling exists.
 *
 * [ ] no unsafe Rust is required.
 *
 * [ ] deterministic parsing is preserved.
 *
 * [ ] source portability is preserved.
 *
 * [ ] target realization remains downstream.
 *
 * ============================================================================
 * 58. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     1. ANTLR accepts the grammar with the repository's canonical
 *        ZamaniLexer.
 *
 *     2. AdaptiveExpressions resolves without circular grammar imports.
 *
 *     3. Resilience resolves without circular grammar imports.
 *
 *     4. quantum.g4 imports this grammar exactly once.
 *
 *     5. The universal parser can reach quantumAdaptiveExecution.
 *
 *     6. `adapt(...)` remains owned by AdaptiveExpressions.
 *
 *     7. resilience blocks remain owned by Resilience.
 *
 *     8. Quantum operation syntax remains owned by quantum/operations.g4.
 *
 *     9. Measurement syntax remains owned by quantum/measurement.g4.
 *
 *    10. Dynamic-control syntax remains owned by its existing owner.
 *
 *    11. No new universal keyword is required.
 *
 *    12. No machine-specific constant is present.
 *
 *    13. No quantum hardware identifier is present.
 *
 *    14. No competing quantum IR is introduced.
 *
 *    15. Semantic analysis can attach quantum meaning without modifying the
 *        universal adaptive syntax.
 *
 *    16. Resilience states/outcomes remain semantically extensible without
 *        changing this integration grammar.
 *
 *    17. Positive, negative, boundary, cross-domain and scalability tests
 *        exist.
 *
 *    18. Rust integration remains compatible with Rust 1.97+ and safe Rust.
 *
 * ============================================================================
 * 59. FINAL ARCHITECTURAL GUARANTEE
 * ============================================================================
 *
 * This file deliberately stays SMALL in executable grammar responsibility
 * while being complete in architectural responsibility.
 *
 * Its purpose is not to encode every possible adaptive quantum algorithm.
 *
 * Its purpose is to provide one stable integration boundary:
 *
 *     universal adaptive intent
 *              +
 *     quantum semantic context
 *              +
 *     resilience intent
 *              |
 *              v
 *        domain-neutral AST
 *              |
 *              v
 *        semantic analysis
 *              |
 *              v
 *          quantum::ir
 *              |
 *              v
 *     target-independent optimization
 *              |
 *              v
 *       routing / scheduling
 *              |
 *              v
 *        QEC / resilience
 *              |
 *              v
 *             ZQN
 *              |
 *              v
 *             HAL
 *              |
 *              v
 *       available realization
 *
 * Therefore:
 *
 *     source meaning is independent of machine size;
 *     resource availability determines feasibility;
 *     capabilities determine realizability;
 *     policies determine authorization;
 *     resilience determines acceptable execution behavior;
 *     quantum::ir remains the canonical quantum semantic boundary.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */
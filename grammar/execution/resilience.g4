// =============================================================================
// Zamani Language — Universal Resilience Grammar
// =============================================================================
//
// File:
//     grammar/execution/resilience.g4
//
// Grammar:
//     parser grammar Resilience;
//
// Purpose:
//     Define target-independent resilience intent for Zamani programs.
//
// Status:
//     Production-target grammar contract.
//
// -----------------------------------------------------------------------------
// AUTHORITY
// -----------------------------------------------------------------------------
//
// Normative architecture:
//     ../DESIGN.md
//
// Normative language specification:
//     ../specification/
//
// Canonical composition root:
//     ../Zamani.g4
//
// Implementation-conformance reference:
//     ../grammar.md
//
// Historical / extended design reference:
//     ../Zamani-Grammar.md
//
// This file is the syntax authority for resilience declarations only.
//
// It does NOT define:
//     - QEC implementation
//     - physical-device calibration
//     - routing
//     - scheduling implementation
//     - hardware topology
//     - runtime state machines
//     - vendor APIs
//     - ZQN implementation
//     - HAL implementation
//     - compiler resource limits
//
// -----------------------------------------------------------------------------
// DESIGN PRINCIPLES
// -----------------------------------------------------------------------------
//
// 1. Target independent.
// 2. No artificial hardware limits.
// 3. Resource requirements are symbolic/semantic.
// 4. Capabilities are semantic requirements.
// 5. Preferences are not requirements.
// 6. Constraints are not implementation decisions.
// 7. Recovery policy is declarative.
// 8. Failure policy is declarative.
// 9. QEC intent remains separate from QEC implementation.
// 10. ZQN remains separate from parser syntax.
// 11. Scheduling remains separate from resilience.
// 12. Placement remains separate from resilience.
// 13. Runtime remains responsible for actual execution.
// 14. Unknown extensions may be represented syntactically and validated
//     semantically according to the active dialect/profile.
// 15. No target-specific parser actions.
// 16. No unsafe code.
// 17. No Rust implementation code inside this grammar.
// 18. Source order is preserved by the parser.
// 19. Semantic analysis determines which properties are order-independent.
// 20. The grammar imposes no finite resource capacity.
//
// -----------------------------------------------------------------------------
// POCO-REAF
// -----------------------------------------------------------------------------
//
// Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//
// A resilience declaration describes:
//
//     WHAT reliability/correctness properties are required
//     WHAT failures may be tolerated
//     WHAT recovery behavior is acceptable
//     WHAT capabilities are required
//     WHAT resources are required
//     WHAT preferences may be considered
//
// It does NOT prescribe:
//
//     WHICH CPU
//     WHICH GPU
//     WHICH FPGA
//     WHICH QPU
//     WHICH physical qubit
//     WHICH core
//     WHICH memory bank
//     WHICH node
//     WHICH device identifier
//
// Those decisions belong downstream.
//
// -----------------------------------------------------------------------------
// HARD-CODING POLICY
// -----------------------------------------------------------------------------
//
// Forbidden as universal language limits:
//
//     MAX_QUBITS
//     MAX_CPUS
//     MAX_GPUS
//     MAX_FPGAS
//     MAX_NODES
//     MAX_MEMORY
//     MAX_THREADS
//     MAX_TENSOR_RANK
//     MAX_REGISTER_WIDTH
//     MAX_NETWORK_SIZE
//     MAX_DEVICE_COUNT
//
// Also forbidden:
//
//     fixed number of retry attempts in grammar
//     fixed number of replicas in grammar
//     fixed number of recovery branches
//     fixed number of resilience levels
//     fixed number of devices
//     fixed hardware topology
//     fixed physical qubit numbering
//     fixed register width
//     fixed timeline count
//
// A program may still contain a concrete value:
//
//     retries: 3;
//
// because that is program semantics.
//
// The grammar must never transform such a value into a compiler-wide limit.
//
// -----------------------------------------------------------------------------
// SEMANTIC VOCABULARY
// -----------------------------------------------------------------------------
//
// Resilience state vocabulary:
//
//     Unknown
//     Healthy
//     Degraded
//     Unstable
//     Unavailable
//     Recovering
//     Quarantined
//     Retired
//
// Resilience outcome vocabulary:
//
//     ACCEPT
//     DEGRADED_ACCEPT
//     RETRY
//     RECOVER
//     ESCALATE
//     REJECT
//
// These are semantic values, not hardware limits.
//
// -----------------------------------------------------------------------------
// FOUR-WAY INTENT SEPARATION
// -----------------------------------------------------------------------------
//
// Requirement:
//
//     requires: qubits >= n;
//
// Capability:
//
//     requires: capability("quantum.mid_circuit_measurement");
//
// Preference:
//
//     prefer: accelerator("quantum");
//
// Implementation decision:
//
//     map: q0 -> physical_qubit(17);
//
// The first three are portable program intent.
//
// The last is target realization and MUST NOT be interpreted by this grammar
// as a portable physical mapping. Target-specific mappings belong downstream
// in placement/routing/HAL/deployment infrastructure or an explicitly
// target-specific dialect.
//
// -----------------------------------------------------------------------------
// INTEGRATION
// -----------------------------------------------------------------------------
//
// Direct consumers:
//
//     execution/runtime.g4
//     execution/scheduling.g4
//     execution/recovery.g4
//     execution/checkpointing.g4
//     execution/placement.g4
//
// Related domains:
//
//     resources/*.g4
//     hardware/*.g4
//     quantum/*.g4
//     hybrid/*.g4
//     hdl/*.g4
//     distributed/*.g4
//     compile/*.g4
//     concurrency/*.g4
//     security/*.g4
//
// Downstream semantic consumers:
//
//     semantic analysis
//     resource analysis
//     capability analysis
//     resilience planner
//     QEC analysis
//     ZQN
//     scheduler
//     router
//     HAL
//     runtime
//
// -----------------------------------------------------------------------------
// OWNERSHIP
// -----------------------------------------------------------------------------
//
// OWNS:
//
//     resilience declaration syntax
//     resilience target selection syntax
//     resilience clause structure
//     resilience property/value syntax
//     resilience extension syntax
//
// DOES NOT OWN:
//
//     runtime implementation
//     QEC implementation
//     QEC limits
//     physical mapping
//     scheduling
//     routing
//     topology
//     calibration
//     device discovery
//     resource allocation
//     fault injection
//     telemetry
//     recovery execution
//     checkpoint storage
//     compiler optimization
//
// -----------------------------------------------------------------------------
// AST CONTRACT
// -----------------------------------------------------------------------------
//
// This grammar must lower into a domain-neutral AST contract equivalent to:
//
//     ResilienceDeclaration {
//         target: Option<QualifiedName>,
//         clauses: Vec<ResilienceClause>,
//         source_span: SourceSpan
//     }
//
//     ResilienceClause {
//         key: QualifiedName,
//         value: Expression,
//         source_span: SourceSpan
//     }
//
// The parser must NOT create:
//
//     PhysicalQubit
//     CpuId
//     GpuId
//     FpgaId
//     QpuId
//     HardwareTopology
//     QecPlan
//     RuntimeRecoveryPlan
//
// Those belong downstream.
//
// -----------------------------------------------------------------------------
// SEMANTIC CONTRACT
// -----------------------------------------------------------------------------
//
// Semantic analysis classifies resilience clauses into concepts such as:
//
//     policy
//     requirement
//     capability
//     constraint
//     preference
//     hint
//     state handling
//     outcome handling
//     retry policy
//     recovery policy
//     checkpoint policy
//     escalation policy
//     degradation policy
//     fault-tolerance policy
//     error-correction intent
//     noise/reliability requirement
//     timeout/deadline/window
//     extension
//
// Unknown properties may be accepted syntactically but must be resolved by
// the semantic feature/dialect registry. They must never silently acquire
// implementation semantics.
//
// -----------------------------------------------------------------------------
// IR CONTRACT
// -----------------------------------------------------------------------------
//
// This grammar does NOT introduce a second resilience IR.
//
// The lowering path is:
//
//     source
//       |
//       v
//     ResilienceDeclaration AST
//       |
//       v
//     semantic resilience intent
//       |
//       +--------------------+
//       |                    |
//       v                    v
//     resource/capability   resilience/QEC/ZQN model
//       |                    |
//       +---------+----------+
//                 |
//                 v
//          canonical execution/semantic IR
//                 |
//                 v
//       optimization / routing / scheduling
//                 |
//                 v
//                HAL
//
// The canonical quantum IR remains quantum::ir.
//
// -----------------------------------------------------------------------------
// DETERMINISM
// -----------------------------------------------------------------------------
//
// Parsing is deterministic.
//
// Source order of clauses is preserved.
//
// Semantic analysis decides whether repeated properties are:
//
//     singleton
//     repeatable
//     mergeable
//     mutually exclusive
//
// The grammar itself must not impose accidental ordering semantics.
//
// -----------------------------------------------------------------------------
// EXTENSIBILITY
// -----------------------------------------------------------------------------
//
// Resilience property names are qualified identifiers rather than a giant
// list of lexer keywords.
//
// Therefore adding a future property such as:
//
//     resilience_domain.some_future_policy: value;
//
// does not require changing the lexer merely to recognize the property name.
//
// Stable language-level declarations still require specification, semantic,
// AST, IR, and conformance-test promotion.
//
// -----------------------------------------------------------------------------
// DIAGNOSTICS
// -----------------------------------------------------------------------------
//
// Syntax diagnostics are parser responsibility.
//
// Semantic diagnostics include, but are not limited to:
//
//     invalid resilience target
//     unknown resilience property
//     incompatible resilience properties
//     duplicate singleton property
//     invalid resilience value
//     unsupported capability
//     unsatisfied resource requirement
//     impossible constraint
//     incompatible recovery policy
//     incompatible degradation policy
//     invalid retry policy
//     invalid checkpoint policy
//     unsupported extension
//     target-specific implementation leaking into portable profile
//
// Diagnostics must carry source spans.
//
// -----------------------------------------------------------------------------
// SECURITY
// -----------------------------------------------------------------------------
//
// Resilience declarations must not provide parser-level escape hatches.
//
// They cannot execute code.
//
// They cannot invoke runtime operations.
//
// They cannot mutate hardware.
//
// They cannot bypass semantic validation.
//
// They cannot bypass authorization/capability checks.
//
// -----------------------------------------------------------------------------
// PERFORMANCE
// -----------------------------------------------------------------------------
//
// The grammar contains no fixed clause-count limit.
//
// Lists and repeated declarations scale with available implementation
// resources.
//
// The parser must not impose language-level limits on:
//
//     number of resilience clauses
//     number of targets
//     number of requirements
//     number of capabilities
//     number of recovery alternatives
//     number of states
//
// Implementation resource exhaustion remains an implementation/runtime
// concern rather than a language semantic limit.
//
// =============================================================================

parser grammar Resilience;

options {
    tokenVocab = ZamaniLexer;
}

//
// -----------------------------------------------------------------------------
// TOP-LEVEL DECLARATION
// -----------------------------------------------------------------------------
//
// Examples:
//
//     resilience {
//         policy: fault_tolerant;
//     }
//
//     resilience quantum::kernel {
//         requires: capability("quantum.measurement");
//     }
//
//     resilience distributed::service {
//         state: degraded;
//         outcome: DEGRADED_ACCEPT;
//     }
//
// The optional target identifies the program entity to which the policy
// applies. If omitted, semantic analysis resolves the enclosing execution
// scope.
//
resilienceDeclaration
    : RESILIENCE resilienceTarget? resilienceBody
    ;

//
// A resilience target is symbolic.
//
// It is deliberately not a physical device identifier.
//
resilienceTarget
    : qualifiedName
    ;

//
// A single canonical body form is used.
//
// This avoids the old pattern of having multiple structurally equivalent
// brace/block alternatives.
//
resilienceBody
    : LBRACE resilienceClause* RBRACE
    ;

//
// -----------------------------------------------------------------------------
// CLAUSES
// -----------------------------------------------------------------------------
//
// Every clause is:
//
//     qualifiedName : expression ;
//
// This is intentionally generic.
//
// The semantic registry gives well-known properties their meaning while
// preserving extensibility.
//
resilienceClause
    : resilienceProperty COLON expression SEMICOLON
    ;

//
// Property names may be hierarchical:
//
//     policy
//     requires
//     requires.capability
//     recovery.strategy
//     recovery.timeout
//     fault_tolerance.mode
//     quantum.error_correction
//     distributed.replication
//
// No fixed property list is encoded here.
//
resilienceProperty
    : qualifiedName
    ;

//
// -----------------------------------------------------------------------------
// OPTIONAL ASSIGNMENT FORM
// -----------------------------------------------------------------------------
//
// Some existing Zamani designs may use:
//
//     resilience {
//         policy = fault_tolerant;
//     }
//
// Keep this rule available only if the repository's shared syntax contract
// declares assignment-style resilience properties compatible.
//
// It is deliberately separated from the canonical colon form so that the
// semantic layer can distinguish legacy/compatibility syntax if necessary.
//
// If the language specification chooses one spelling permanently, this rule
// can be disabled by the compatibility profile without changing the semantic
// model.
//
resilienceAssignmentClause
    : resilienceProperty ASSIGN expression SEMICOLON
    ;

//
// A compatibility-aware body can accept both forms when the active language
// profile permits them.
//
// The canonical stable syntax remains colon-based.
//
resilienceCompatibilityBody
    : LBRACE resilienceCompatibilityClause* RBRACE
    ;

resilienceCompatibilityClause
    : resilienceProperty COLON expression SEMICOLON
    | resilienceProperty ASSIGN expression SEMICOLON
    ;

//
// -----------------------------------------------------------------------------
// REUSABLE RESILIENCE BLOCK
// -----------------------------------------------------------------------------
//
// This rule allows other execution grammars to embed resilience policy
// without redefining its internal syntax.
//
// Example conceptual use:
//
//     schedule foo {
//         resilience { ... }
//     }
//
// The embedding grammar owns the surrounding construct;
// this grammar owns the resilience body.
//
resilienceBlock
    : RESILIENCE resilienceBody
    ;

//
// -----------------------------------------------------------------------------
// POLICY VALUE HELPERS
// -----------------------------------------------------------------------------
//
// These are intentionally aliases around the canonical expression grammar.
//
// They provide stable integration points for semantic code without creating
// duplicate expression syntax.
//
resilienceValue
    : expression
    ;

resilienceRequirementValue
    : expression
    ;

resilienceCapabilityValue
    : expression
    ;

resiliencePreferenceValue
    : expression
    ;

resilienceConstraintValue
    : expression
    ;

resilienceRecoveryValue
    : expression
    ;

resilienceCheckpointValue
    : expression
    ;

resilienceRetryValue
    : expression
    ;

resilienceStateValue
    : expression
    ;

resilienceOutcomeValue
    : expression
    ;

//
// -----------------------------------------------------------------------------
// WELL-KNOWN SEMANTIC PROPERTY CONTRACTS
// -----------------------------------------------------------------------------
//
// These rules do NOT introduce new keywords.
//
// They document semantic keys that the feature registry may recognize.
//
// They are kept as parser aliases so future semantic implementations can
// refer to stable rule names without duplicating syntax.
//
// The actual property remains a qualifiedName.
//
resiliencePolicyProperty
    : resilienceProperty
    ;

resilienceRequirementProperty
    : resilienceProperty
    ;

resilienceCapabilityProperty
    : resilienceProperty
    ;

resilienceConstraintProperty
    : resilienceProperty
    ;

resiliencePreferenceProperty
    : resilienceProperty
    ;

resilienceHintProperty
    : resilienceProperty
    ;

resilienceStateProperty
    : resilienceProperty
    ;

resilienceOutcomeProperty
    : resilienceProperty
    ;

resilienceRetryProperty
    : resilienceProperty
    ;

resilienceRecoveryProperty
    : resilienceProperty
    ;

resilienceCheckpointProperty
    : resilienceProperty
    ;

resilienceEscalationProperty
    : resilienceProperty
    ;

resilienceDegradationProperty
    : resilienceProperty
    ;

resilienceFaultToleranceProperty
    : resilienceProperty
    ;

resilienceErrorCorrectionProperty
    : resilienceProperty
    ;

resilienceReliabilityProperty
    : resilienceProperty
    ;

//
// -----------------------------------------------------------------------------
// OPTIONAL SEMANTIC CLASSIFICATION ENTRY POINT
// -----------------------------------------------------------------------------
//
// This rule exists for downstream grammar composition.
//
// It does not encode the semantic classification itself.
//
resilienceIntent
    : resilienceProperty COLON resilienceValue SEMICOLON
    ;

//
// -----------------------------------------------------------------------------
// DOCUMENTED SEMANTIC PROPERTY NAMES
// -----------------------------------------------------------------------------
//
// The following names are examples of semantic registry entries, not lexer
// tokens and not parser-enforced finite vocabulary.
//
// policy
// requires
// capability
// constraint
// prefer
// hint
//
// state
// outcome
//
// retry
// retry.limit
// retry.backoff
// retry.condition
//
// recovery
// recovery.strategy
// recovery.condition
// recovery.timeout
//
// checkpoint
// checkpoint.policy
// checkpoint.interval
// checkpoint.storage
//
// escalation
// escalation.policy
// escalation.target
//
// degradation
// degradation.policy
//
// fault_tolerance
// fault_tolerance.mode
// fault_tolerance.threshold
//
// error_correction
// error_correction.code
// error_correction.distance
// error_correction.strategy
//
// reliability
// reliability.target
// reliability.minimum
//
// noise
// noise.budget
// noise.model
//
// These names remain ordinary qualified identifiers.
//
// -----------------------------------------------------------------------------
// RESILIENCE STATE CONTRACT
// -----------------------------------------------------------------------------
//
// The semantic registry recognizes the following values where applicable:
//
//     Unknown
//     Healthy
//     Degraded
//     Unstable
//     Unavailable
//     Recovering
//     Quarantined
//     Retired
//
// The parser deliberately treats them as expressions/identifiers rather than
// hard-coded lexer tokens.
//
// This permits future state extensions without grammar surgery.
//
// -----------------------------------------------------------------------------
// RESILIENCE OUTCOME CONTRACT
// -----------------------------------------------------------------------------
//
// The semantic registry recognizes:
//
//     ACCEPT
//     DEGRADED_ACCEPT
//     RETRY
//     RECOVER
//     ESCALATE
//     REJECT
//
// Again, these remain semantic values.
//
// -----------------------------------------------------------------------------
// RESOURCE/CAPABILITY EXAMPLES
// -----------------------------------------------------------------------------
//
// Valid portable intent:
//
//     resilience quantum::kernel {
//         requires: qubits >= n;
//         requires: capability("quantum.measurement");
//         requires: capability("quantum.mid_circuit_measurement");
//     }
//
// Valid scalable intent:
//
//     resilience workload {
//         requires: memory >= required_memory;
//         requires: capability("tensor.compute");
//     }
//
// Valid preference:
//
//     resilience workload {
//         prefer: accelerator("quantum");
//     }
//
// Valid constraint:
//
//     resilience workload {
//         constraint: reliability >= required_reliability;
//     }
//
// None of these impose a compiler-wide maximum.
//
// -----------------------------------------------------------------------------
// RECOVERY EXAMPLES
// -----------------------------------------------------------------------------
//
//     resilience service {
//         policy: fault_tolerant;
//         retry: retry_policy(max_attempts);
//         recovery: recovery_strategy;
//         checkpoint: checkpoint_policy;
//         outcome: DEGRADED_ACCEPT;
//     }
//
// The expressions may be defined by libraries, semantic capabilities,
// profiles, or dialects.
//
// The grammar does not implement the policies.
//
// -----------------------------------------------------------------------------
// QUANTUM EXAMPLES
// -----------------------------------------------------------------------------
//
//     resilience quantum::kernel {
//         requires: capability("quantum.measurement");
//         requires: capability("quantum.mid_circuit_measurement");
//         error_correction: fault_tolerant;
//         reliability: required_reliability;
//         outcome: RECOVER;
//     }
//
//     resilience quantum::algorithm {
//         requires: qubits >= required_qubits;
//         requires: capability("quantum.logical_qubit");
//         recovery: logical_recovery;
//     }
//
// There is no fixed number of qubits, gates, QPUs, physical qubits, or
// correction rounds encoded in this grammar.
//
// -----------------------------------------------------------------------------
// CLASSICAL EXAMPLES
// -----------------------------------------------------------------------------
//
//     resilience classical::kernel {
//         requires: capability("memory.redundancy");
//         retry: retry_policy;
//         outcome: RETRY;
//     }
//
// -----------------------------------------------------------------------------
// DISTRIBUTED EXAMPLES
// -----------------------------------------------------------------------------
//
//     resilience distributed::service {
//         requires: capability("distributed.replication");
//         constraint: availability >= required_availability;
//         recovery: failover;
//         outcome: DEGRADED_ACCEPT;
//     }
//
// No node count is encoded.
//
// -----------------------------------------------------------------------------
// HDL/HARDWARE EXAMPLES
// -----------------------------------------------------------------------------
//
//     resilience accelerator::module {
//         requires: capability("hardware.reconfiguration");
//         recovery: reconfigure;
//         outcome: RECOVER;
//     }
//
// The grammar does not encode FPGA fabric size, register width, memory size,
// clock count, or physical topology.
//
// -----------------------------------------------------------------------------
// HYBRID EXAMPLE
// -----------------------------------------------------------------------------
//
//     resilience hybrid::workflow {
//         requires: capability("quantum.measurement");
//         requires: capability("classical.feedforward");
//         recovery: restart_quantum_region;
//         outcome: DEGRADED_ACCEPT;
//     }
//
// -----------------------------------------------------------------------------
// EXTENSION EXAMPLES
// -----------------------------------------------------------------------------
//
// A dialect may define:
//
//     resilience_domain.custom_policy: custom_value;
//
// or:
//
//     vendor.example.resilience_mode: vendor_policy;
//
// The parser can represent this without adding a lexer keyword.
//
// Semantic validation determines whether the active portability profile
// permits the extension.
//
// -----------------------------------------------------------------------------
// DUPLICATION SEMANTICS
// -----------------------------------------------------------------------------
//
// These are legal syntactically:
//
//     resilience workload {
//         requires: capability("a");
//         requires: capability("b");
//     }
//
//     resilience workload {
//         constraint: latency < budget_a;
//         constraint: energy < budget_b;
//     }
//
// The semantic registry determines whether repetition means:
//
//     AND
//     OR
//     accumulation
//     override
//     merge
//     conflict
//
// The parser must not silently discard repeated clauses.
//
// -----------------------------------------------------------------------------
// SOURCE-SPAN CONTRACT
// -----------------------------------------------------------------------------
//
// Every resilienceDeclaration and resilienceClause must retain source spans
// through AST construction.
//
// The implementation must be capable of diagnosing the exact property/value
// that caused a semantic error.
//
// -----------------------------------------------------------------------------
// COMPATIBILITY CONTRACT
// -----------------------------------------------------------------------------
//
// Stable syntax:
//
//     resilience [target] { property: expression; ... }
//
// Legacy/experimental assignment syntax:
//
//     resilience [target] { property = expression; ... }
//
// Assignment syntax must only be enabled by an explicit compatibility or
// dialect profile if the normative specification does not select it.
//
// This prevents historical syntax from silently becoming permanent syntax.
//
// -----------------------------------------------------------------------------
// TEST CONTRACT
// -----------------------------------------------------------------------------
//
// Positive:
//
//     resilience {}
//
//     resilience workload {}
//
//     resilience workload {
//         policy: fault_tolerant;
//     }
//
//     resilience workload {
//         requires: capability("quantum.measurement");
//         requires: qubits >= n;
//     }
//
//     resilience workload {
//         prefer: accelerator("quantum");
//         hint: adaptive;
//     }
//
//     resilience workload {
//         retry: retry_policy;
//         recovery: recover;
//         checkpoint: checkpoint_policy;
//     }
//
//     resilience workload {
//         state: Degraded;
//         outcome: DEGRADED_ACCEPT;
//     }
//
//     resilience quantum::kernel {
//         requires: capability("quantum.mid_circuit_measurement");
//         error_correction: fault_tolerant;
//     }
//
//     resilience distributed::service {
//         requires: capability("distributed.replication");
//         recovery: failover;
//     }
//
// Negative:
//
//     resilience
//
//     resilience workload {
//
//     resilience workload {
//         policy
//     }
//
//     resilience workload {
//         policy:;
//     }
//
//     resilience workload {
//         : policy;
//     }
//
//     resilience workload {
//         policy: value
//     }
//
// Boundary:
//
//     resilience {}
//
//     resilience workload {
//         requires: qubits >= n;
//     }
//
//     resilience workload {
//         requires: qubits >= n;
//         requires: memory >= required_memory;
//         requires: capability("tensor.compute");
//     }
//
//     very.deep.resilience.target {
//         // represented through resilience target syntax where declaration
//         // dispatch identifies the resilience construct
//     }
//
// Scalability:
//
//     arbitrarily many clauses subject only to implementation resources
//     arbitrarily many repeated requirements
//     arbitrarily many capability expressions
//     arbitrarily large symbolic resource expressions
//     arbitrarily large target names supported by the lexer
//
// Determinism:
//
//     identical source + identical language/profile/environment contracts
//     produces identical parse structure.
//
// Portability:
//
//     no test establishes a maximum qubit/core/GPU/FPGA/node/memory count.
//
// -----------------------------------------------------------------------------
// HARD-CODING AUDIT
// -----------------------------------------------------------------------------
//
// PASS CONDITIONS:
//
//     no MAX_* resource constants
//     no fixed device counts
//     no fixed QEC distances
//     no fixed retry count
//     no fixed recovery count
//     no fixed topology
//     no fixed physical identifiers
//     no vendor API syntax
//     no QPU-specific grammar
//     no CPU-specific grammar
//     no GPU-specific grammar
//     no FPGA-specific grammar
//     no parser-side hardware decisions
//
// -----------------------------------------------------------------------------
// COMPLETION CRITERIA
// -----------------------------------------------------------------------------
//
// This file is considered complete when:
//
//     [ ] grammar parses with the repository's canonical ANTLR toolchain
//     [ ] token vocabulary is resolved by the composition build
//     [ ] RESILIENCE is present in the canonical lexer vocabulary
//     [ ] no target-specific parser actions exist
//     [ ] no unsafe Rust exists
//     [ ] resilience target syntax is defined
//     [ ] resilience clause syntax is defined
//     [ ] expressions are delegated to canonical expression grammar
//     [ ] source spans are preserved by AST construction
//     [ ] semantic classification is specified
//     [ ] resource requirements remain symbolic
//     [ ] capabilities remain symbolic
//     [ ] preferences remain distinct
//     [ ] implementation decisions remain downstream
//     [ ] QEC remains downstream
//     [ ] ZQN remains downstream
//     [ ] scheduling remains downstream
//     [ ] routing remains downstream
//     [ ] HAL remains downstream
//     [ ] repeated clauses are preserved
//     [ ] extension properties remain representable
//     [ ] no artificial capacity exists
//     [ ] positive tests exist
//     [ ] negative tests exist
//     [ ] boundary tests exist
//     [ ] scalability tests exist
//     [ ] determinism tests exist
//     [ ] portability tests exist
//     [ ] compatibility behavior is specified
//
// =============================================================================

// -----------------------------------------------------------------------------
// Grammar rules
// -----------------------------------------------------------------------------

resilienceDeclaration
    : RESILIENCE resilienceTarget? resilienceBody
    ;

resilienceTarget
    : qualifiedName
    ;

resilienceBody
    : LBRACE resilienceClause* RBRACE
    ;

resilienceClause
    : resilienceProperty COLON resilienceValue SEMICOLON
    ;

resilienceProperty
    : qualifiedName
    ;

resilienceValue
    : expression
    ;

resilienceAssignmentClause
    : resilienceProperty ASSIGN resilienceValue SEMICOLON
    ;

resilienceCompatibilityBody
    : LBRACE resilienceCompatibilityClause* RBRACE
    ;

resilienceCompatibilityClause
    : resilienceClause
    | resilienceAssignmentClause
    ;

resilienceBlock
    : RESILIENCE resilienceBody
    ;

resilienceIntent
    : resilienceProperty COLON resilienceValue SEMICOLON
    ;

resiliencePolicyProperty
    : resilienceProperty
    ;

resilienceRequirementProperty
    : resilienceProperty
    ;

resilienceCapabilityProperty
    : resilienceProperty
    ;

resilienceConstraintProperty
    : resilienceProperty
    ;

resiliencePreferenceProperty
    : resilienceProperty
    ;

resilienceHintProperty
    : resilienceProperty
    ;

resilienceStateProperty
    : resilienceProperty
    ;

resilienceOutcomeProperty
    : resilienceProperty
    ;

resilienceRetryProperty
    : resilienceProperty
    ;

resilienceRecoveryProperty
    : resilienceProperty
    ;

resilienceCheckpointProperty
    : resilienceProperty
    ;

resilienceEscalationProperty
    : resilienceProperty
    ;

resilienceDegradationProperty
    : resilienceProperty
    ;

resilienceFaultToleranceProperty
    : resilienceProperty
    ;

resilienceErrorCorrectionProperty
    : resilienceProperty
    ;

resilienceReliabilityProperty
    : resilienceProperty
    ;
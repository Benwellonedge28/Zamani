/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/expressions/evidence.g4
 *
 * Grammar:
 *     EvidenceExpressions
 *
 * Status:
 *     CANONICAL PRODUCTION SOURCE-LEVEL EVIDENCE EXPRESSION GRAMMAR
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
 * This file is the SINGLE SOURCE-LEVEL OWNER for the universal evidence
 * expression:
 *
 *     evidence(...)
 *
 * Evidence represents structured support, qualification, contradiction,
 * verification, derivation, observation, or other semantic relationship
 * associated with a computational claim or subject.
 *
 * Evidence is intentionally domain-neutral.
 *
 * It may be associated with:
 *
 *     classical computation
 *     scientific computation
 *     numerical computation
 *     AI/model computation
 *     reasoning
 *     knowledge
 *     learning
 *     adaptation
 *     quantum computation
 *     quantum measurement
 *     hybrid computation
 *     HDL verification
 *     hardware observation
 *     simulation
 *     distributed computation
 *     networking
 *     security
 *     compiler transformations
 *     interoperability
 *     future computational domains
 *
 * This grammar expresses STRUCTURED COMPUTATIONAL INTENT.
 *
 * It does not decide whether evidence is:
 *
 *     true
 *     sufficient
 *     trusted
 *     verified
 *     valid
 *     complete
 *     authoritative
 *     reproducible
 *     safe
 *     physically realizable.
 *
 * Those properties belong to semantic, verification, provenance, policy,
 * security, resource, capability, and runtime systems.
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
 *     Expressions
 *          |
 *          v
 *     EvidenceExpressions             <-- THIS FILE
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     structural validation
 *          |
 *          +-------------------------------+
 *          |          |          |          |
 *          v          v          v          v
 *        types     effects   capabilities resources
 *          |          |          |          |
 *          +----------+----------+----------+
 *                     |
 *                     v
 *                 contracts
 *                     |
 *                     v
 *                  policies
 *                     |
 *                     v
 *                 provenance
 *                     |
 *                     v
 *               semantic evidence
 *                     |
 *          +----------+-----------+
 *          |          |           |
 *          v          v           v
 *      classical   quantum      other
 *                    |
 *                    v
 *                quantum::ir
 *                    |
 *                    v
 *              optimization
 *                    |
 *              lowering
 *                    |
 *          routing / scheduling
 *                    |
 *             resilience
 *                    |
 *                ZQN / HAL
 *                    |
 *             target realization
 *
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 *     evidenceExpression
 *     evidenceArgumentList
 *     evidenceArgument
 *     evidenceNamedArgument
 *     evidencePositionalArgument
 *     evidenceFieldName
 *
 * These are the only source-level evidence-expression parser constructs
 * introduced by this file.
 *
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     lexer rules
 *     keywords
 *     punctuation
 *     operators
 *     identifiers
 *     qualified names
 *     general expression precedence
 *     types
 *     uncertainty
 *     probability
 *     distributions
 *     confidence mathematics
 *     assertions
 *     contracts
 *     policies
 *     provenance
 *     knowledge
 *     reasoning
 *     inference
 *     learning
 *     adaptation
 *     causality
 *     explanations
 *     decisions
 *     quantum syntax
 *     HDL syntax
 *     hardware syntax
 *     concurrency
 *     distributed execution
 *     networking
 *     FFI
 *     ABI
 *     simulation
 *     resource allocation
 *     capability resolution
 *     security enforcement
 *     verification algorithms
 *     proof checking
 *     certificate checking
 *     trust computation
 *     evidence storage
 *     evidence combination algorithms
 *     runtime execution
 *     target selection
 *     scheduling
 *     routing
 *     QEC
 *     ZQN
 *     HAL
 *     IR construction.
 *
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * Evidence syntax MUST exist exactly once at the source-expression level.
 *
 * AI, quantum, validation, security, data, compiler, and other domains may
 * consume this expression but MUST NOT redefine it.
 *
 * In particular:
 *
 *     grammar/ai/evidence.g4
 *
 * must be an AI composition adapter only.
 *
 *     grammar/validation/evidence.g4
 *
 * remains a validation facade and must not duplicate this syntax.
 *
 *     grammar/expressions/provenance.g4
 *
 * remains the provenance-expression owner.
 *
 *     grammar/expressions/uncertainty.g4
 *
 * remains the uncertainty-expression owner.
 *
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/core/names.g4
 *     grammar/expressions/expressions.g4
 *
 * IMPORTS:
 *
 *     Names
 *     Expressions
 *
 * EXPORTS:
 *
 *     evidenceExpression
 *
 *     Secondary parser rules are intentionally local.
 *
 * CONSUMED_BY:
 *
 *     grammar/expressions/expressions.g4
 *     grammar/ai/evidence.g4
 *     grammar/ai/knowledge.g4
 *     grammar/ai/reasoning.g4
 *     grammar/ai/assertions.g4
 *     grammar/ai/explanations.g4
 *     grammar/ai/causality.g4
 *     grammar/validation/*
 *     grammar/security/*
 *     grammar/data/*
 *     grammar/quantum/*
 *     grammar/hybrid/*
 *     future domain adapters
 *
 * AST_OWNER:
 *
 *     Existing domain-neutral frontend AST.
 *
 * SEMANTIC_OWNER:
 *
 *     Universal semantic evidence subsystem.
 *
 * IR_OWNER:
 *
 *     Canonical semantic representation.
 *
 *     Quantum-derived evidence crosses:
 *
 *         quantum::ir
 *
 * TEST_OWNER:
 *
 *     grammar/tests/expressions/evidence/
 *     grammar/tests/ai/evidence/
 *     grammar/tests/semantic/evidence/
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/evidence.md
 *     grammar/spec/ai.md
 *     grammar/spec/provenance.md
 *
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The AST must preserve:
 *
 *     source span
 *     evidence expression kind
 *     ordered arguments
 *     positional/named distinction
 *     field names
 *     field source spans
 *     nested expressions
 *     source provenance
 *
 * The AST MUST NOT become:
 *
 *     AIEvidenceAst
 *     QuantumEvidenceAst
 *     HardwareEvidenceAst
 *     ModelEvidenceAst
 *
 * merely because evidence originates from a particular domain.
 *
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis interprets evidence arguments.
 *
 * A canonical evidence expression may conceptually represent:
 *
 *     evidence(
 *         claim: claim_value,
 *         source: source_value,
 *         support: support_value,
 *         confidence: confidence_value,
 *         provenance: provenance_value
 *     )
 *
 * The grammar deliberately does not hard-code a closed field taxonomy.
 *
 * Semantic registries may recognize standardized fields such as:
 *
 *     claim
 *     subject
 *     source
 *     kind
 *     content
 *     confidence
 *     strength
 *     support
 *     contradiction
 *     derivation
 *     verification
 *     provenance
 *     context
 *     scope
 *
 * Future fields remain possible without changing this grammar.
 *
 *
 * ============================================================================
 * OPEN-WORLD FIELD CONTRACT
 * ============================================================================
 *
 * Standard language-wide evidence fields may be reserved by the canonical
 * lexical system where already justified.
 *
 * Other evidence fields remain ordinary identifiers.
 *
 * Therefore a future provider may introduce semantic fields such as:
 *
 *     reliability
 *     calibration
 *     methodology
 *     measurement
 *     experiment
 *     certificate
 *     witness
 *     observation
 *     lineage
 *
 * without requiring a new universal grammar rule merely because the semantic
 * vocabulary expanded.
 *
 *
 * ============================================================================
 * VALUE CONTRACT
 * ============================================================================
 *
 * Every evidence value is an ordinary Zamani expression.
 *
 * Therefore evidence may contain:
 *
 *     literals
 *     identifiers
 *     calls
 *     records
 *     tuples
 *     arrays
 *     tensors
 *     streams
 *     datasets
 *     models
 *     knowledge results
 *     reasoning results
 *     learning results
 *     uncertainty values
 *     probability values
 *     quantum measurement results
 *     simulation results
 *     distributed results
 *     hardware observations
 *     foreign values
 *     future-domain values.
 *
 * This file does not reproduce expression precedence.
 *
 *
 * ============================================================================
 * NAMED ARGUMENT CONTRACT
 * ============================================================================
 *
 * Named evidence fields support both established source forms:
 *
 *     field: expression
 *
 * and:
 *
 *     field = expression
 *
 * The semantic layer determines whether both forms have equivalent meaning
 * under the selected language/compatibility version.
 *
 *
 * ============================================================================
 * POSITIONAL ARGUMENT CONTRACT
 * ============================================================================
 *
 * Positional arguments remain syntactically legal.
 *
 * Their meaning is determined by the semantic operation contract.
 *
 * This supports provider-independent forms such as:
 *
 *     evidence(claim, source)
 *
 * and:
 *
 *     evidence(claim, source, confidence)
 *
 * without forcing every future evidence provider into a fixed grammar.
 *
 *
 * ============================================================================
 * EMPTY ARGUMENT CONTRACT
 * ============================================================================
 *
 * The expression requires at least one argument.
 *
 * Therefore:
 *
 *     evidence()
 *
 * is syntactically invalid.
 *
 * This avoids creating an ambiguous empty evidence object.
 *
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * This grammar creates no evidence-specific type.
 *
 * It MUST NOT introduce:
 *
 *     EvidenceType
 *     AIEvidenceType
 *     QuantumEvidenceType
 *     ProofType
 *     CertificateType
 *     VerificationType
 *
 * as universal grammar-level types.
 *
 * The type system determines the types of:
 *
 *     claim
 *     source
 *     content
 *     confidence
 *     derivation
 *     verification
 *     provenance
 *     context
 *     other fields.
 *
 *
 * ============================================================================
 * UNCERTAINTY INTEGRATION
 * ============================================================================
 *
 * Evidence may consume canonical uncertainty expressions.
 *
 * For example:
 *
 *     evidence(
 *         claim: result,
 *         confidence: uncertain_confidence
 *     )
 *
 * or:
 *
 *     evidence(
 *         claim: result,
 *         probability: probability_value
 *     )
 *
 * This grammar does not redefine uncertainty.
 *
 * The canonical owner remains:
 *
 *     grammar/expressions/uncertainty.g4
 *
 *
 * ============================================================================
 * PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Evidence may consume canonical provenance expressions.
 *
 * For example:
 *
 *     evidence(
 *         claim: result,
 *         provenance: provenance(source)
 *     )
 *
 * This grammar does not redefine provenance.
 *
 * The canonical expression owner remains:
 *
 *     grammar/expressions/provenance.g4
 *
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Evidence expressions may appear inside:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *     assert
 *
 * This grammar does not redefine those constructs.
 *
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Evidence may be subject to:
 *
 *     trust policies
 *     provenance policies
 *     verification policies
 *     disclosure policies
 *     retention policies
 *     security policies
 *     execution policies
 *
 * Policy evaluation remains downstream.
 *
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing evidence has no effect.
 *
 * Semantic resolution may introduce effects depending on the values and
 * providers involved.
 *
 * Possible semantic effects include:
 *
 *     observation
 *     IO
 *     network
 *     storage
 *     measurement
 *     foreign
 *     distributed
 *     simulation
 *     quantum
 *     native
 *
 * The grammar does not infer effects.
 *
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Evidence semantics may require capabilities such as:
 *
 *     evidence.read
 *     evidence.create
 *     evidence.verify
 *     evidence.record
 *     provenance.read
 *     provenance.record
 *     verification.execute
 *     observation.read
 *
 * These are symbolic semantic capabilities.
 *
 * The grammar does not determine whether a target provides them.
 *
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * This grammar introduces NO resource ceilings.
 *
 * It does not define limits for:
 *
 *     evidence count
 *     argument count
 *     provenance depth
 *     source count
 *     claim count
 *     verification depth
 *     confidence precision
 *     memory
 *     storage
 *     CPU count
 *     GPU count
 *     FPGA count
 *     QPU count
 *     node count
 *     thread count
 *     tensor rank
 *     network size
 *
 * Repetition is represented through grammar repetition operators rather than
 * machine-dependent constants.
 *
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Evidence describes semantic information, not physical realization.
 *
 * The same evidence-bearing source may therefore participate in:
 *
 *     tiny execution
 *     embedded execution
 *     CPU execution
 *     multicore execution
 *     GPU execution
 *     FPGA execution
 *     ASIC execution
 *     accelerator execution
 *     QPU execution
 *     simulation
 *     HPC execution
 *     cluster execution
 *     distributed execution
 *     cloud execution
 *     future execution models
 *
 * Target feasibility is determined downstream.
 *
 * "Infinity" means:
 *
 *     no artificial language-level finite ceiling.
 *
 * It does not mean infinite physical resources are assumed.
 *
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Evidence may describe:
 *
 *     quantum measurement
 *     circuit verification
 *     decomposition evidence
 *     routing evidence
 *     scheduling evidence
 *     resilience evidence
 *     experimental evidence
 *
 * The grammar does not define:
 *
 *     qubits
 *     physical qubits
 *     topology
 *     coupling maps
 *     calibration
 *     QEC
 *     gate sets
 *     QPU selection
 *     ZQN
 *     HAL.
 *
 * Quantum semantic lowering remains:
 *
 *     evidence expression
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic evidence
 *          |
 *          v
 *     quantum semantic model
 *          |
 *          v
 *     quantum::ir
 *
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Evidence may describe:
 *
 *     synthesis verification
 *     simulation results
 *     timing observations
 *     hardware observations
 *     implementation checks
 *     physical measurements
 *
 * This grammar does not encode:
 *
 *     wire widths
 *     register widths
 *     device counts
 *     clock counts
 *     memory capacity
 *     physical topology
 *     implementation-specific limits.
 *
 *
 * ============================================================================
 * DISTRIBUTED BOUNDARY
 * ============================================================================
 *
 * Evidence may originate from:
 *
 *     tasks
 *     actors
 *     services
 *     nodes
 *     streams
 *     remote observations
 *     distributed verification.
 *
 * The grammar does not encode a maximum number of distributed participants.
 *
 *
 * ============================================================================
 * INTEROPERABILITY BOUNDARY
 * ============================================================================
 *
 * Evidence may reference:
 *
 *     JSON
 *     XML
 *     SQL results
 *     foreign data
 *     external services
 *     ABI values
 *     FFI results
 *
 * Their syntax remains owned by the relevant interoperability or dialect
 * grammar.
 *
 *
 * ============================================================================
 * METAPROGRAMMING BOUNDARY
 * ============================================================================
 *
 * Evidence expressions may be inspected through the normal AST and reflection
 * facilities.
 *
 * This grammar does not perform reflection or compile-time execution.
 *
 *
 * ============================================================================
 * SECURITY BOUNDARY
 * ============================================================================
 *
 * Evidence may be subject to security policies.
 *
 * This grammar does not:
 *
 *     authenticate evidence
 *     authorize evidence
 *     verify signatures
 *     encrypt evidence
 *     establish trust
 *     enforce disclosure rules.
 *
 * Those are downstream concerns.
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
 *     explicitly selected language/compatibility version.
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware availability
 *     resource availability
 *     network state
 *     filesystem state
 *     runtime state
 *     scheduler state
 *     model execution
 *     randomness
 *     wall-clock time.
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
 *     no target-specific actions
 *     no filesystem access
 *     no network access
 *     no hardware access
 *     no runtime execution
 *     no unsafe Rust.
 *
 * Generated Rust code must remain compatible with:
 *
 *     Rust 1.97+
 *     Rust 2021
 *     safe Rust.
 *
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST NOT contain:
 *
 *     MAX_EVIDENCE
 *     MAX_EVIDENCE_ARGUMENTS
 *     MAX_EVIDENCE_DEPTH
 *     MAX_PROVENANCE
 *     MAX_SOURCES
 *     MAX_CLAIMS
 *     MAX_CONFIDENCE_BITS
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
 * No finite physical capacity is represented by this grammar.
 *
 *
 * ============================================================================
 * PUBLIC RULE
 * ============================================================================
 */

evidenceExpression
    : EVIDENCE
      LPAREN
      evidenceArgumentList
      RPAREN
    ;


/*
 * ============================================================================
 * ARGUMENT LIST
 * ============================================================================
 */

evidenceArgumentList
    : evidenceArgument
      (COMMA evidenceArgument)*
    ;


evidenceArgument
    : evidenceNamedArgument
    | evidencePositionalArgument
    ;


evidenceNamedArgument
    : evidenceFieldName
      (COLON | ASSIGN)
      expression
    ;


evidencePositionalArgument
    : expression
    ;


evidenceFieldName
    : identifier
    ;
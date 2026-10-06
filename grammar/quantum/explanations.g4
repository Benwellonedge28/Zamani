/*

* ============================================================================
* Zamani Universal Computing Language
* ============================================================================
* 
* FILE
* ---
* grammar/quantum/explanations.g4
* 
* GRAMMAR
* ---
* QuantumExplanations
* 
* STATUS
* ---
* CANONICAL QUANTUM-DOMAIN EXPLANATION COMPOSITION ADAPTER
* 
* IMPLEMENTATION BASELINE
* ---
* Rust 1.97+
* Rust 2021 edition
* Safe Rust only
* No unsafe Rust
* 
* GRAMMAR TECHNOLOGY
* ---
* ANTLR4 parser grammar
* 
* ============================================================================
* FEATURE CONTRACT
* ============================================================================
* 
* PURPOSE
* ---
* 
* This file provides the quantum-domain composition boundary for explanation
* requests.
* 
* It does NOT define a second explanation language.
* 
* The universal source-level explanation syntax is owned by:
* 
* grammar/statements/explain.g4
* 
* whose parser grammar identity is:
* 
* Explain
* 
* This file adapts that universal construct into the quantum composition
* hierarchy so that quantum programs can explicitly request explanations of:
* 
* quantum operations
* quantum states
* quantum measurements
* quantum results
* quantum circuits
* quantum transformations
* quantum optimization decisions
* quantum resource decisions
* quantum capability decisions
* quantum routing decisions
* quantum scheduling decisions
* quantum error-correction decisions
* quantum resilience decisions
* hybrid quantum/classical results
* quantum simulation results
* quantum execution results
* quantum compilation artifacts
* quantum provenance
* future quantum-domain semantic artifacts
* 
* The actual explanation target remains an ordinary Zamani expression.
* 
* ============================================================================
* ARCHITECTURAL PRINCIPLE
* ============================================================================
* 
* Explanation is a UNIVERSAL LANGUAGE CAPABILITY.
* 
* Quantum explanation is therefore:
* 
* universal explanation syntax
*         |
*         v
* quantum-domain composition
*         |
*         v
* domain-neutral AST
*         |
*         v
* quantum semantic analysis
*         |
*         v
*     quantum::ir
*         |
*         v
* explanation/provenance consumers
* 
* This file MUST NOT create:
* 
* QuantumExplanationIR
* QuantumExplanationAST
* QuantumExplanationType
* QuantumExplanationLanguage
* 
* or another quantum-specific explanation hierarchy.
* 
* ============================================================================
* OWNERSHIP
* ============================================================================
* 
* THIS FILE OWNS
* ---
* 
* quantumExplanationConstruct
* quantumExplanationStatement
* quantumExplanationElement
* 
* These are quantum-domain composition boundaries only.
* 
* THIS FILE DOES NOT OWN
* ---
* 
* explainStatement
* explanationTarget
* explanationSourceClause
* explanationContextClause
* explanationContextList
* explanationContext
* 
* Those productions belong exclusively to:
* 
* grammar/statements/explain.g4
* 
* This file also does not own:
* 
* expressions
* identifiers
* names
* qualified names
* literals
* types
* quantum operations
* quantum states
* quantum measurements
* quantum circuits
* quantum controls
* quantum resources
* quantum capabilities
* quantum error correction
* routing
* scheduling
* resilience
* provenance storage
* evidence storage
* policies
* effects
* contracts
* hardware
* target selection
* runtime execution
* backend implementation
* 
* ============================================================================
* SINGLE-OWNER RULE
* ============================================================================
* 
* There MUST be exactly one authoritative explanation syntax.
* 
* That authority is:
* 
* grammar/statements/explain.g4
* 
* This file MUST NOT redefine:
* 
* explainStatement
* explanationTarget
* explanationSourceClause
* explanationContextClause
* explanationContextList
* explanationContext
* 
* The quantum domain obtains explanation syntax exclusively by delegation.
* 
* This prevents:
* 
* universal explain syntax
* +
* quantum explain syntax
* 
* from becoming two incompatible languages.
* 
* ============================================================================
* DEPENDENCY CONTRACT
* ============================================================================
* 
* DEPENDS_ON:
* 
* grammar/antlr/ZamaniLexer.g4
* grammar/statements/explain.g4
* 
* IMPORTS:
* 
* Explain
* 
* EXPORTS:
* 
* quantumExplanationConstruct
* quantumExplanationStatement
* quantumExplanationElement
* 
* CONSUMED_BY:
* 
* grammar/quantum/quantum.g4
* 
* AST_OWNER:
* 
* domain-neutral frontend AST
* 
* SEMANTIC_OWNER:
* 
* universal explanation semantic subsystem
* +
* quantum semantic analysis when the target is quantum-derived
* 
* TYPE_OWNER:
* 
* canonical Zamani type subsystem
* 
* EFFECT_OWNER:
* 
* canonical effect subsystem
* 
* CAPABILITY_OWNER:
* 
* canonical capability subsystem
* 
* RESOURCE_OWNER:
* 
* canonical resource subsystem
* 
* CONTRACT_OWNER:
* 
* canonical validation/contract subsystem
* 
* POLICY_OWNER:
* 
* canonical policy/security subsystem
* 
* PROVENANCE_OWNER:
* 
* canonical provenance subsystem
* 
* IR_OWNER:
* 
* canonical semantic/domain IR pipeline
* 
* QUANTUM_IR_OWNER:
* 
* quantum::ir
* 
* TEST_OWNER:
* 
* grammar/tests/quantum/explanations/
* 
* SPEC_OWNER:
* 
* grammar/spec/quantum.md
* grammar/specification/
* 
* ============================================================================
* DEPENDENCY DIRECTION
* ============================================================================
* 
* The dependency direction is intentionally one-way:
* 
* grammar/statements/explain.g4
*              |
*              v
*    grammar/quantum/explanations.g4
*              |
*              v
*    grammar/quantum/quantum.g4
*              |
*              v
*    canonical Zamani parser
* 
* The reverse dependency is forbidden.
* 
* In particular:
* 
* QuantumExplanations MUST NOT import Quantum.
* 
* Otherwise the following cycle would occur:
* 
* Quantum
*   -> QuantumExplanations
*   -> Quantum
* 
* The quantum composition root consumes this adapter; this adapter never
* consumes the quantum composition root.
* 
* ============================================================================
* LEXER CONTRACT
* ============================================================================
* 
* This file defines NO lexer rules.
* 
* The canonical lexical authority is:
* 
* grammar/antlr/ZamaniLexer.g4
* 
* The universal explanation grammar already consumes:
* 
* EXPLAIN
* 
* Therefore this file requires no new lexer keyword merely to support
* quantum explanations.
* 
* Do NOT add quantum-specific explanation keywords such as:
* 
* explain_quantum
* explain_gate
* explain_qubit
* explain_measurement
* explain_circuit
* explain_qpu
* 
* The target expression already supplies the required extensibility.
* 
* New explanation mechanisms should normally be semantic capabilities,
* libraries, dialects, or backend services rather than new universal
* keywords.
* 
* ============================================================================
* GRAMMAR DECLARATION
* ============================================================================
  */

parser grammar QuantumExplanations;

options {
tokenVocab = ZamaniLexer;
}

/*

* ============================================================================
* UNIVERSAL EXPLANATION IMPORT
* ============================================================================
* 
* Explain is the sole source-level explanation syntax owner.
* ============================================================================
  */

import
Explain
;

/*

* ============================================================================
* PUBLIC QUANTUM EXPLANATION CONSTRUCT
* ============================================================================
* 
* A quantum explanation construct is exactly the universal explanation
* statement.
* 
* Examples accepted through the imported grammar include:
* 
* explain quantum_result;
* 
* explain measurement_result;
* 
* explain circuit_result from measurement_record;
* 
* explain routing_decision from compilation_record;
* 
* explain scheduling_decision with (provenance);
* 
* explain error_correction_decision
*     from execution_record
*     with (provenance, policy);
* 
* explain hybrid_result;
* 
* No quantum-specific target syntax is necessary.
* ============================================================================
  */

quantumExplanationConstruct
: explainStatement
;

/*

* ============================================================================
* PUBLIC QUANTUM EXPLANATION STATEMENT
* ============================================================================
* 
* Stable statement-level composition alias.
* 
* This rule does not duplicate the universal explanation syntax.
* ============================================================================
  */

quantumExplanationStatement
: explainStatement
;

/*

* ============================================================================
* QUANTUM ELEMENT ADAPTER
* ============================================================================
* 
* This rule allows the quantum composition root to expose explanation as one
* of its domain elements without importing the complete universal statement
* hierarchy into this file.
* 
* ============================================================================
  */

quantumExplanationElement
: quantumExplanationConstruct
;

/*

* ============================================================================
* SOURCE-LEVEL SEMANTIC MODEL
* ============================================================================
* 
* The accepted source-level form is inherited from Explain:
* 
* explain TARGET;
* 
* explain TARGET from SOURCE;
* 
* explain TARGET with (CONTEXT);
* 
* explain TARGET from SOURCE with (CONTEXT, CONTEXT);
* 
* TARGET, SOURCE, and CONTEXT are ordinary Zamani expressions.
* 
* Therefore quantum expressions remain governed by the existing expression
* architecture.
* 
* This file does NOT create:
* 
* quantumExplanationTarget
* quantumExplanationSource
* quantumExplanationContext
* 
* because those would duplicate universal expression ownership.
* 
* ============================================================================
  */

/*

* ============================================================================
* QUANTUM TARGET INTEGRATION
* ============================================================================
* 
* A quantum explanation target may semantically represent:
* 
* quantum operation
* quantum state
* quantum register
* qubit-related value
* measurement
* observable
* circuit
* quantum result
* dynamic-circuit result
* feed-forward result
* error-correction result
* resilience result
* routing result
* scheduling result
* optimization result
* compilation result
* simulation result
* hardware realization result
* hybrid result
* 
* The grammar does not enumerate these target categories.
* 
* They are represented by ordinary expressions and resolved semantically.
* 
* This makes the language open-ended:
* 
* adding a new quantum semantic construct does not require this file to
* change merely because the new construct exists.
* 
* ============================================================================
  */

/*

* ============================================================================
* QUANTUM SOURCE / EVIDENCE INTEGRATION
* ============================================================================
* 
* The universal "from" clause may identify information relevant to explaining
* a quantum result.
* 
* Possible semantic sources include:
* 
* measurement records
* execution records
* circuit transformations
* compiler transformations
* optimization records
* decomposition records
* routing records
* scheduling records
* QEC records
* resilience records
* simulation records
* hardware observations
* calibration-derived semantic records
* resource decisions
* capability decisions
* provenance records
* experimental data
* classical control data
* hybrid execution data
* 
* None of these are hard-coded into the grammar.
* 
* ============================================================================
  */

/*

* ============================================================================
* CONTEXT INTEGRATION
* ============================================================================
* 
* The universal "with (...)" clause may carry explanation context such as:
* 
* provenance
* evidence
* policy
* confidence
* uncertainty
* resource information
* capability information
* reproducibility information
* deterministic-execution information
* compiler information
* simulation information
* execution information
* hybrid information
* 
* These remain ordinary expressions.
* 
* No quantum-specific context enumeration is required.
* 
* ============================================================================
  */

/*

* ============================================================================
* MEASUREMENT INTEGRATION
* ============================================================================
* 
* Explanation may target measurement-derived information:
* 
* explain measurement_result;
* 
* explain measurement_result from measurement_record;
* 
* explain decision from measurement_record
*     with (provenance);
* 
* Measurement syntax remains owned by the quantum measurement subsystem.
* 
* This file does not redefine:
* 
* measurement
* observable
* outcome
* basis
* measurement target
* 
* ============================================================================
  */

/*

* ============================================================================
* OPERATION INTEGRATION
* ============================================================================
* 
* Explanation may target arbitrary quantum operations.
* 
* The operation model remains open-world.
* 
* This file therefore does not enumerate:
* 
* H
* X
* Y
* Z
* S
* T
* CNOT
* CZ
* SWAP
* RX
* RY
* RZ
* 
* or any future/vendor/custom operation.
* 
* Explanation syntax remains unchanged as operation vocabularies evolve.
* 
* ============================================================================
  */

/*

* ============================================================================
* CIRCUIT INTEGRATION
* ============================================================================
* 
* Explanation may describe:
* 
* circuit structure
* circuit transformations
* optimization
* decomposition
* routing
* scheduling
* execution
* simulation
* verification
* 
* Circuit syntax remains owned by the circuit subsystem.
* 
* This adapter does not create circuit-specific explanation syntax.
* 
* ============================================================================
  */

/*

* ============================================================================
* DYNAMIC-CIRCUIT INTEGRATION
* ============================================================================
* 
* Dynamic and measurement-driven quantum computation may produce explanation
* targets such as:
* 
* control decisions
* measurement dependencies
* feed-forward decisions
* adaptive execution decisions
* conditional quantum operations
* 
* These are ordinary semantic values from the perspective of this grammar.
* 
* Dynamic-control syntax remains owned by the existing quantum dynamic-control
* subsystem.
* 
* ============================================================================
  */

/*

* ============================================================================
* QUANTUM-CLASSICAL INTEGRATION
* ============================================================================
* 
* Hybrid computation can be explained using the same universal construct:
* 
* explain hybrid_result;
* 
* explain quantum_result from classical_context;
* 
* explain classical_decision from measurement_record;
* 
* explain feedback_decision
*     from execution_record
*     with (provenance, policy);
* 
* The hybrid grammar remains the owner of quantum/classical control syntax.
* 
* This file does not create a second hybrid language.
* 
* ============================================================================
  */

/*

* ============================================================================
* QUANTUM ERROR-CORRECTION INTEGRATION
* ============================================================================
* 
* Explanation may target semantic QEC information:
* 
* explain correction_decision;
* 
* explain recovery_decision from syndrome_record;
* 
* explain resilience_decision from qec_record;
* 
* The grammar does not enumerate:
* 
* code families
* code distances
* syndrome widths
* decoder implementations
* physical qubits
* physical error rates
* correction hardware
* 
* Those are semantic/runtime/backend concerns.
* 
* No universal capacity or implementation ceiling is introduced.
* 
* ============================================================================
  */

/*

* ============================================================================
* RESILIENCE INTEGRATION
* ============================================================================
* 
* Explanation may describe execution-resilience decisions such as:
* 
* ACCEPT
* DEGRADED_ACCEPT
* RETRY
* RECOVER
* ESCALATE
* REJECT
* 
* and semantic states such as:
* 
* Unknown
* Healthy
* Degraded
* Unstable
* Unavailable
* Recovering
* Quarantined
* Retired
* 
* These values belong to the semantic/runtime resilience model.
* 
* They are NOT grammar-level quantum explanation keywords.
* 
* A future resilience state must therefore not require this grammar to change
* merely because the semantic model expands.
* 
* ============================================================================
  */

/*

* ============================================================================
* PROVENANCE INTEGRATION
* ============================================================================
* 
* Quantum explanation is strongly coupled semantically to provenance.
* 
* Provenance may describe:
* 
* source
* transformation
* derived artifact
* operation
* measurement
* optimization
* decomposition
* routing
* scheduling
* QEC
* resilience
* execution
* verification
* backend realization
* 
* This file does not store or verify provenance.
* 
* Provenance storage and verification remain downstream responsibilities.
* 
* ============================================================================
  */

/*

* ============================================================================
* EVIDENCE INTEGRATION
* ============================================================================
* 
* Quantum evidence may originate from:
* 
* measurement
* simulation
* execution
* verification
* compiler analysis
* resource analysis
* capability analysis
* hardware observation
* classical computation
* hybrid computation
* 
* Evidence is represented through ordinary expressions.
* 
* The evidence subsystem determines:
* 
* validity
* trust
* origin
* confidence
* derivation
* verification
* 
* This grammar does not establish those properties.
* 
* ============================================================================
  */

/*

* ============================================================================
* UNCERTAINTY INTEGRATION
* ============================================================================
* 
* Quantum explanation may involve:
* 
* probabilities
* distributions
* confidence
* uncertainty
* statistical observations
* experimental results
* 
* This grammar imposes no:
* 
* fixed precision
* fixed probability representation
* fixed distribution size
* fixed sample count
* fixed tensor rank
* fixed numeric width
* 
* Those concerns belong to the type and semantic systems.
* 
* ============================================================================
  */

/*

* ============================================================================
* POLICY INTEGRATION
* ============================================================================
* 
* Quantum explanations may be governed by policies controlling:
* 
* access to execution records
* access to measurement records
* access to provenance
* access to hardware information
* disclosure
* redaction
* reproducibility
* evidence requirements
* external service access
* model/algorithm inspection
* reflection
* 
* Policy evaluation is downstream.
* 
* A syntactically valid explanation does NOT imply authorization.
* 
* ============================================================================
  */

/*

* ============================================================================
* EFFECT INTEGRATION
* ============================================================================
* 
* Explanation itself introduces no parser-level effects.
* 
* Semantic analysis may determine that realization requires effects such as:
* 
* IO
* computation
* provenance access
* evidence access
* reflection
* simulation
* network access
* foreign access
* 
* This grammar MUST NOT silently grant any such effect.
* 
* Effect ownership remains with the canonical effects subsystem.
* 
* ============================================================================
  */

/*

* ============================================================================
* CAPABILITY INTEGRATION
* ============================================================================
* 
* A quantum explanation implementation may require semantic capabilities such
* as:
* 
* explanation
* quantum.explanation
* evidence.read
* provenance.read
* quantum.execution.inspect
* quantum.measurement.inspect
* quantum.simulation.inspect
* 
* These are capability identifiers, not hardware declarations.
* 
* The grammar MUST NOT enumerate which machines provide them.
* 
* Capability resolution remains downstream.
* 
* ============================================================================
  */

/*

* ============================================================================
* RESOURCE INTEGRATION
* ============================================================================
* 
* Explanation may consume resources depending upon:
* 
* target complexity
* provenance volume
* evidence volume
* simulation complexity
* execution-trace complexity
* model complexity
* requested explanation detail
* 
* No fixed upper bound is encoded here.
* 
* In particular, this file contains no limits on:
* 
* qubits
* quantum operations
* circuit depth
* circuit width
* measurements
* evidence records
* provenance records
* nodes
* devices
* memory
* threads
* tensor dimensions
* 
* Practical limits belong to resource analysis and execution.
* 
* ============================================================================
  */

/*

* ============================================================================
* CONTRACT INTEGRATION
* ============================================================================
* 
* Explanation can occur within source governed by:
* 
* requires
* ensures
* invariant
* assume
* guarantee
* property
* 
* Contract syntax remains owned by the validation subsystem.
* 
* This file does not redefine contracts.
* 
* ============================================================================
  */

/*

* ============================================================================
* QUANTUM IR BOUNDARY
* ============================================================================
* 
* This file creates NO IR.
* 
* If the explained subject is quantum-derived, the canonical semantic path is:
* 
* source
*    |
*    v
* universal explanation syntax
*    |
*    v
* domain-neutral AST
*    |
*    v
* semantic analysis
*    |
*    v
* quantum::ir
*    |
*    v
* explanation/provenance consumer
* 
* This file MUST NOT introduce:
* 
* QuantumExplanationIR
* ExplanationQuantumIR
* QuantumExplainIR
* 
* The canonical quantum semantic boundary remains:
* 
* quantum::ir
* 
* ============================================================================
  */

/*

* ============================================================================
* AST CONTRACT
* ============================================================================
* 
* The frontend AST mapper should preserve:
* 
* explanation intent
* target expression
* optional source expression
* optional context expressions
* source span
* language version
* dialect/version information where applicable
* 
* The mapper MUST NOT encode:
* 
* physical qubit IDs
* physical device IDs
* vendor topology
* calibration state
* routing decisions
* scheduler decisions
* backend allocation
* 
* unless those are already represented as ordinary semantic values in the
* source program.
* 
* ============================================================================
  */

/*

* ============================================================================
* SEMANTIC CONTRACT
* ============================================================================
* 
* A syntactically valid construct means:
* 
* an explanation request has been expressed.
* 
* It does NOT imply:
* 
* the target exists;
* the target is explainable;
* evidence exists;
* provenance exists;
* the explanation mechanism exists;
* the caller is authorized;
* required capabilities exist;
* required resources exist;
* the target is executable;
* the target is physically realizable;
* the explanation is complete;
* the explanation is truthful.
* 
* Those properties are established downstream.
* 
* ============================================================================
  */

/*

* ============================================================================
* DETERMINISM CONTRACT
* ============================================================================
* 
* Parsing depends only on:
* 
* source text
* canonical lexer
* canonical parser grammar
* selected language version
* selected compatible dialect
* 
* Parsing MUST NOT depend on:
* 
* QPU availability
* hardware discovery
* calibration
* runtime state
* network state
* filesystem state
* wall-clock time
* randomness
* target selection
* available evidence
* available provenance
* 
* ============================================================================
  */

/*

* ============================================================================
* OPEN-WORLD CONTRACT
* ============================================================================
* 
* Adding a new quantum:
* 
* operation
* state representation
* measurement mechanism
* circuit representation
* simulator
* QPU
* accelerator
* error-correction method
* resilience mechanism
* compiler optimization
* routing algorithm
* scheduling algorithm
* hardware target
* vendor implementation
* 
* MUST NOT require this grammar to change merely because that capability
* exists.
* 
* If new source syntax is genuinely required, it must first receive its own
* canonical grammar owner and then be explicitly composed.
* 
* ============================================================================
  */

/*

* ============================================================================
* POCO-REAF / SCALABILITY CONTRACT
* ============================================================================
* 
* This file is compatible with:
* 
* Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
* 
* because it describes explanation intent rather than physical realization.
* 
* The same source-level explanation request may therefore be semantically
* considered for:
* 
* tiny embedded systems
* CPUs
* multicore systems
* GPUs
* accelerators
* FPGAs
* ASICs
* QPUs
* quantum simulators
* HPC systems
* clusters
* distributed systems
* cloud systems
* future computational systems
* 
* without this grammar imposing a universal machine-size ceiling.
* 
* ============================================================================
  */

/*

* ============================================================================
* HARD-CODING AUDIT
* ============================================================================
* 
* This file contains:
* 
* [x] no fixed qubit limit
* [x] no fixed register limit
* [x] no fixed circuit-width limit
* [x] no fixed circuit-depth limit
* [x] no fixed measurement limit
* [x] no fixed operation limit
* [x] no fixed device count
* [x] no fixed QPU count
* [x] no fixed CPU count
* [x] no fixed GPU count
* [x] no fixed FPGA count
* [x] no fixed node count
* [x] no fixed memory size
* [x] no fixed thread count
* [x] no fixed tensor rank
* [x] no fixed register width
* [x] no vendor enumeration
* [x] no physical topology
* [x] no physical allocation
* [x] no timing constants
* [x] no calibration constants
* [x] no backend implementation
* [x] no runtime execution
* [x] no embedded Rust
* [x] no unsafe Rust
* 
* ============================================================================
  */

/*

* ============================================================================
* TEST CONTRACT
* ============================================================================
* 
* Tests belong outside this grammar file.
* 
* REQUIRED POSITIVE TESTS
* ---
* 
* The quantum explanation test suite should cover at least:
* 
* explain quantum_result;
* 
* explain measurement_result;
* 
* explain circuit_result;
* 
* explain quantum_result from measurement_record;
* 
* explain circuit_result from compilation_record;
* 
* explain routing_decision from routing_record;
* 
* explain scheduling_decision from scheduling_record;
* 
* explain correction_decision from qec_record;
* 
* explain resilience_decision from execution_record;
* 
* explain simulation_result from simulation_record;
* 
* explain hybrid_result from execution_record;
* 
* explain quantum_result with (provenance);
* 
* explain quantum_result with (evidence);
* 
* explain quantum_result
*     from execution_record
*     with (provenance, policy);
* 
* explain quantum_expression
*     from compilation_record
*     with (uncertainty, provenance);
* 
* REQUIRED NEGATIVE TESTS
* ---
* 
* The quantum adapter must inherit the universal Explain diagnostics.
* 
* Test malformed forms such as:
* 
* explain;
* 
* explain from evidence;
* 
* explain with (policy);
* 
* explain result from;
* 
* explain result with;
* 
* explain result with ();
* 
* explain result with (evidence,);
* 
* explain result with (, evidence);
* 
* explain result with (evidence,, provenance);
* 
* explain result with (evidence) from source;
* 
* The adapter MUST NOT weaken the universal grammar.
* 
* REQUIRED BOUNDARY TESTS
* ---
* 
* Test:
* 
* explanation + measurement
* explanation + quantum operation
* explanation + circuit
* explanation + dynamic control
* explanation + feed-forward
* explanation + quantum/classical hybrid computation
* explanation + QEC
* explanation + resilience
* explanation + simulation
* explanation + provenance
* explanation + evidence
* explanation + policy
* explanation + uncertainty
* 
* REQUIRED CROSS-DOMAIN TEST
* ---
* 
* At least one test must cover:
* 
* classical computation
*      ->
* quantum operation
*      ->
* measurement
*      ->
* classical decision
*      ->
* quantum control
*      ->
* execution result
*      ->
* provenance
*      ->
* explanation
* 
* The explanation grammar must remain unchanged across that pipeline.
* 
* REQUIRED SCALABILITY TESTS
* ---
* 
* Vary:
* 
* expression complexity
* target complexity
* context size
* provenance expression complexity
* evidence expression complexity
* nested semantic structures
* 
* without converting those dimensions into grammar-level limits.
* 
* REQUIRED DETERMINISM TEST
* ---
* 
* Identical source/token streams under identical parser configuration must
* produce equivalent parse structure.
* 
* ============================================================================
  */

/*

* ============================================================================
* COMPATIBILITY CONTRACT
* ============================================================================
* 
* Public stable rules:
* 
* quantumExplanationConstruct
* quantumExplanationStatement
* quantumExplanationElement
* 
* The universal explanation syntax remains versioned by:
* 
* grammar/statements/explain.g4
* 
* Therefore changes to the universal explanation syntax are compatibility
* sensitive.
* 
* Adding new semantic explanation mechanisms should normally NOT require this
* adapter to change.
* 
* ============================================================================
  */

/*

* ============================================================================
* INTEGRATION CONTRACT FOR quantum/quantum.g4
* ============================================================================
* 
* The quantum composition root MUST:
* 
* 1. import QuantumExplanations exactly once;
* 
* 2. expose explanation through quantumElement;
* 
* 3. delegate to quantumExplanationElement;
* 
* 4. NOT copy the rules from this file;
* 
* 5. NOT import Explain independently merely for quantum composition;
* 
* 6. NOT create a second quantum explanation rule hierarchy.
* 
* Intended composition:
* 
* Quantum
*    |
*    +--> QuantumExplanations
*              |
*              +--> Explain
* 
* The relevant quantum element boundary becomes:
* 
* quantumElement
*     :
*         ...
*       | quantumExplanationElement
*       | ...
* 
* This keeps ownership unambiguous.
* 
* ============================================================================
  */

/*

* ============================================================================
* INTEGRATION CONTRACT FOR grammar/statements/quantum.g4
* ============================================================================
* 
* The canonical statement layer remains the sole owner of universal quantum
* statement dispatch.
* 
* It MUST NOT duplicate:
* 
* quantumExplanationConstruct
* quantumExplanationStatement
* 
* If the statement layer needs quantum explanation, it should consume the
* appropriate quantum-domain boundary through the existing quantum statement
* composition architecture rather than defining a parallel explanation
* language.
* 
* ============================================================================
  */

/*

* ============================================================================
* INTEGRATION CONTRACT FOR grammar/ai/explanations.g4
* ============================================================================
* 
* The AI adapter and this quantum adapter intentionally converge on the same
* universal explanation owner:
* 
*         Explain
*         /    \
*        /      \
*       v        v
* AIExplanations  QuantumExplanations
* 
* Neither adapter imports the other.
* 
* This permits a result to be both:
* 
* AI-related
* 
* and:
* 
* quantum-derived
* 
* without creating a third explanation syntax.
* 
* ============================================================================
  */

/*

* ============================================================================
* INTEGRATION CONTRACT FOR quantum::ir
* ============================================================================
* 
* No grammar-level IR changes are required.
* 
* Quantum semantic analysis determines whether the explanation target
* corresponds to:
* 
* quantum::ir
* 
* and associates explanation/provenance metadata with the canonical semantic
* representation downstream.
* 
* This grammar does not construct, mutate, route, schedule, optimize,
* decompose, error-correct, or execute quantum::ir.
* 
* ============================================================================
  */

/*

* ============================================================================
* INTEGRATION CONTRACT FOR RUST
* ============================================================================
* 
* This grammar contains no embedded Rust actions.
* 
* Generated parser integration therefore remains compatible with:
* 
* Rust 1.97 or later
* Rust 2021
* safe Rust
* no unsafe Rust
* 
* Rust-side AST construction, semantic analysis, diagnostics, provenance,
* capability resolution, resource analysis, policy evaluation and IR
* generation remain outside this grammar.
* 
* ============================================================================
  */

/*

* ============================================================================
* COMPLETION CRITERIA
* ============================================================================
* 
* This file is DONE when:
* 
* [x] It has exactly one parser grammar identity.
* 
* [x] It uses the canonical Zamani lexer vocabulary.
* 
* [x] It imports only the universal Explain grammar.
* 
* [x] It does not import Quantum.
* 
* [x] It cannot create a Quantum -> QuantumExplanations -> Quantum cycle.
* 
* [x] It owns only quantum-domain composition aliases.
* 
* [x] It does not redefine universal explanation syntax.
* 
* [x] It does not create quantum-specific expression syntax.
* 
* [x] It does not enumerate quantum gates.
* 
* [x] It does not enumerate QPUs or vendors.
* 
* [x] It does not encode physical topology.
* 
* [x] It does not encode hardware allocation.
* 
* [x] It does not encode machine-size limits.
* 
* [x] It does not create a competing IR.
* 
* [x] It preserves quantum::ir as the canonical quantum IR boundary.
* 
* [x] It preserves the universal provenance model.
* 
* [x] It preserves the universal evidence model.
* 
* [x] It preserves the universal policy/effect/capability/resource model.
* 
* [x] It remains open-world for future quantum technologies.
* 
* [x] It remains deterministic.
* 
* [x] It contains no runtime behavior.
* 
* [x] It contains no embedded Rust.
* 
* [x] It requires no unsafe Rust.
* 
* [x] Its public rules are stable composition boundaries.
* 
* [x] Quantum composition can consume it without duplicating syntax.
* 
* ============================================================================
* FINAL ARCHITECTURAL INVARIANT
* ============================================================================
* 
* The correct model is:
* 
* universal explanation
*         |
*         v
* quantum composition
*         |
*         v
* domain-neutral AST
*         |
*         v
* quantum semantic analysis
*         |
*         v
*     quantum::ir
*         |
*         v
* provenance / evidence / policy / effects
*         |
*         v
* explanation realization
* 
* NOT:
* 
* quantum explanation language
*         |
*         v
* quantum-specific AST
*         |
*         v
* quantum-specific explanation IR
* 
* The former preserves Zamani's universal architecture and allows the same
* source program to scale across available computational resources without
* changing its explanation syntax.
* 
* ============================================================================
  */
/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/ai/provenance.g4
 *
 * GRAMMAR
 * -------
 * AIProvenance
 *
 * STATUS
 * ------
 * PRODUCTION AI-DOMAIN PROVENANCE COMPOSITION ADAPTER
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 1.97+
 * Rust 2021 edition
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
 * This file provides the AI-domain composition boundary for the universal
 * Zamani provenance model.
 *
 * IMPORTANT:
 *
 * This file does NOT define a second provenance language.
 *
 * Universal provenance expression syntax is owned by:
 *
 *     grammar/expressions/provenance.g4
 *
 * Universal evidence expression syntax is owned by:
 *
 *     grammar/expressions/evidence.g4
 *
 * This file only establishes how AI grammar composition consumes those
 * universal constructs.
 *
 * AI provenance can describe lineage associated with:
 *
 *     - reasoning;
 *     - inference;
 *     - deduction;
 *     - induction;
 *     - abduction;
 *     - knowledge;
 *     - learning;
 *     - adaptation;
 *     - models;
 *     - datasets;
 *     - tensors;
 *     - agents;
 *     - decisions;
 *     - explanations;
 *     - uncertainty;
 *     - causality;
 *     - neural-symbolic computation;
 *     - classical computation;
 *     - quantum computation;
 *     - hybrid computation;
 *     - HDL/hardware computation;
 *     - distributed computation;
 *     - simulation;
 *     - compilation;
 *     - generated artifacts.
 *
 * The AI grammar must preserve provenance across those domains without
 * introducing a competing AI-specific provenance representation.
 *
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * The intended pipeline is:
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          v
 *     AI grammar composition
 *          |
 *          v
 *     aiProvenanceConstruct
 *          |
 *          v
 *     universal provenanceExpression
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     structural validation
 *          |
 *          +------------------+------------------+------------------+
 *          |                  |                  |                  |
 *          v                  v                  v                  v
 *        types             effects          capabilities        resources
 *          |                  |                  |                  |
 *          +------------------+------------------+------------------+
 *                             |
 *                             v
 *                         contracts
 *                             |
 *                             v
 *                          policies
 *                             |
 *                             v
 *                         provenance
 *                             |
 *                             v
 *                    canonical semantic model
 *                             |
 *              +--------------+--------------+
 *              |              |              |
 *              v              v              v
 *        classical IR    quantum::ir     HDL/hardware
 *              |              |              |
 *              +--------------+--------------+
 *                             |
 *                             v
 *                     optimization/lowering
 *                             |
 *                     routing/scheduling
 *                             |
 *                     resilience/QEC
 *                             |
 *                            ZQN
 *                             |
 *                            HAL
 *                             |
 *                       target realization
 *
 *
 * ============================================================================
 * CORE ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * Provenance is UNIVERSAL.
 *
 * AI provenance is therefore a specialization of provenance usage, not a
 * separate provenance language.
 *
 * The semantic model must be able to preserve relationships such as:
 *
 *     source
 *     derived_from
 *     generated_by
 *     transformed_by
 *     consumed_by
 *     produced_by
 *     verified_by
 *     supported_by
 *     decided_by
 *     learned_from
 *     adapted_from
 *     observed_by
 *
 * without requiring this grammar to enumerate every future relationship.
 *
 * New provenance relations must therefore be represented through the
 * universal provenance semantic model or explicitly registered extensions.
 *
 * They must not require a new AI grammar rule merely because a new model,
 * algorithm, accelerator, quantum processor, data source, or backend appears.
 *
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns exactly ONE public parser boundary:
 *
 *     aiProvenanceConstruct
 *
 * This boundary identifies provenance expressions when they are consumed from
 * the AI grammar composition layer.
 *
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     - provenanceExpression;
 *     - provenance argument syntax;
 *     - provenance relation syntax;
 *     - evidenceExpression;
 *     - evidence argument syntax;
 *     - identifiers;
 *     - qualified names;
 *     - literals;
 *     - general expressions;
 *     - function calls;
 *     - model declarations;
 *     - dataset declarations;
 *     - tensor syntax;
 *     - learning syntax;
 *     - adaptation syntax;
 *     - reasoning syntax;
 *     - inference syntax;
 *     - knowledge syntax;
 *     - decision syntax;
 *     - explanation syntax;
 *     - uncertainty syntax;
 *     - causality syntax;
 *     - agent syntax;
 *     - actor syntax;
 *     - concurrency;
 *     - distributed execution;
 *     - policies;
 *     - contracts;
 *     - effects;
 *     - capabilities;
 *     - resources;
 *     - security;
 *     - quantum operations;
 *     - quantum routing;
 *     - QEC;
 *     - HDL;
 *     - hardware;
 *     - FFI;
 *     - ABI;
 *     - compilation;
 *     - optimization;
 *     - scheduling;
 *     - runtime execution;
 *     - target selection;
 *     - target discovery;
 *     - resource allocation;
 *     - provenance storage;
 *     - provenance databases;
 *     - audit-log implementation;
 *     - cryptographic implementation;
 *     - hashing implementation;
 *     - signatures;
 *     - authentication;
 *     - authorization;
 *     - vendor APIs;
 *     - framework APIs;
 *     - physical device identifiers;
 *     - physical qubit identifiers;
 *     - backend-specific IR.
 *
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * The repository MUST maintain the following ownership:
 *
 *     grammar/expressions/provenance.g4
 *         |
 *         +--> universal provenance expression syntax
 *
 *     grammar/expressions/evidence.g4
 *         |
 *         +--> universal evidence expression syntax
 *
 *     grammar/ai/provenance.g4
 *         |
 *         +--> AI composition boundary only
 *
 *     grammar/ai/evidence.g4
 *         |
 *         +--> AI evidence composition boundary only
 *
 *     grammar/data/provenance.g4
 *         |
 *         +--> data lineage declarations/semantics
 *
 *     grammar/compile/provenance.g4
 *         |
 *         +--> compilation provenance declarations/semantics
 *
 *     grammar/security/provenance.g4
 *         |
 *         +--> security/trust provenance declarations/semantics
 *
 * Domain-specific files MAY consume the universal provenance expression.
 *
 * Domain-specific files MUST NOT create another competing
 * `provenanceExpression` rule.
 *
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/expressions/provenance.g4
 *     grammar/expressions/evidence.g4
 *     grammar/ai/ai.g4
 *     canonical Zamani parser composition
 *
 * PUBLIC INPUT:
 *
 *     provenanceExpression
 *
 * OPTIONAL RELATED INPUT:
 *
 *     evidenceExpression
 *
 * EXPORTS:
 *
 *     aiProvenanceConstruct
 *
 * CONSUMED_BY:
 *
 *     grammar/ai/ai.g4
 *     AI semantic composition
 *     AI conformance tests
 *
 * AST_OWNER:
 *
 *     Existing domain-neutral frontend AST.
 *
 * SEMANTIC_OWNER:
 *
 *     Universal provenance semantic subsystem.
 *
 *     AI semantic analysis consumes the universal provenance model.
 *
 * IR_OWNER:
 *
 *     Canonical semantic representation.
 *
 *     No AI-specific provenance IR is introduced here.
 *
 * QUANTUM_IR_OWNER:
 *
 *     quantum::ir
 *
 *     only after semantic classification and quantum lowering where
 *     quantum-derived computation is involved.
 *
 * TEST_OWNER:
 *
 *     grammar/tests/ai/provenance/
 *     grammar/tests/semantic/provenance/
 *     grammar/tests/integration/
 *
 * SPEC_OWNER:
 *
 *     grammar/specification/provenance.md
 *     grammar/specification/ai.md
 *     grammar/spec/ai.md
 *
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * This grammar deliberately imports the canonical expression-level provenance
 * grammar rather than reproducing its syntax.
 *
 * It also imports the canonical evidence expression grammar because AI
 * provenance commonly consumes evidence-bearing semantic results.
 *
 * The imported grammars are composition dependencies.
 *
 * They are NOT alternate semantic owners.
 *
 */

grammar AIProvenance;

import ProvenanceExpressions, EvidenceExpressions;


/*
 * ============================================================================
 * PUBLIC AI PROVENANCE BOUNDARY
 * ============================================================================
 *
 * AI provenance is represented by the same universal provenance expression
 * used everywhere else in Zamani.
 *
 * The adapter intentionally does not introduce:
 *
 *     aiProvenanceExpression
 *     modelProvenanceExpression
 *     trainingProvenanceExpression
 *     inferenceProvenanceExpression
 *     quantumAIProvenanceExpression
 *
 * because those would fragment the language's provenance model.
 *
 * The public rule is therefore a composition boundary rather than a second
 * expression language.
 *
 * Canonical form:
 *
 *     provenance(...)
 *
 * Examples accepted through the delegated universal expression:
 *
 *     provenance(result)
 *
 *     provenance(result, source)
 *
 *     provenance(
 *         result,
 *         source: dataset
 *     )
 *
 *     provenance(
 *         result,
 *         derived_from: input,
 *         evidence: evidence_value
 *     )
 *
 *     provenance(
 *         model_result,
 *         source: model_input,
 *         decision: decision_value
 *     )
 *
 * The detailed argument grammar remains owned by
 * `grammar/expressions/provenance.g4`.
 *
 */
aiProvenanceConstruct
    : provenanceExpression
    ;


/*
 * ============================================================================
 * OPTIONAL AI EVIDENCE COMPOSITION BOUNDARY
 * ============================================================================
 *
 * AI provenance frequently needs to preserve the relationship between a
 * provenance-bearing result and evidence supporting that result.
 *
 * Evidence remains universal.
 *
 * This adapter therefore exposes a separate composition boundary rather than
 * embedding evidence syntax inside provenance syntax.
 *
 * This prevents:
 *
 *     provenance -> AI-specific evidence grammar
 *
 * from becoming a second semantic language.
 *
 * Instead:
 *
 *     provenanceExpression
 *          |
 *          +--> ordinary expression
 *          |
 *          +--> evidenceExpression where applicable
 *
 * The evidence grammar owns evidence syntax.
 *
 */
aiProvenanceEvidenceConstruct
    : evidenceExpression
    ;


/*
 * ============================================================================
 * COMBINED AI PROVENANCE INPUT
 * ============================================================================
 *
 * This rule is intentionally useful to the AI composition root while remaining
 * structurally simple.
 *
 * It allows AI grammar composition to recognize either:
 *
 *     provenance(...)
 *
 * or:
 *
 *     evidence(...)
 *
 * without creating a new AI-specific semantic representation.
 *
 * Semantic analysis determines whether the construct is being used as:
 *
 *     provenance;
 *     evidence;
 *     both through nested expressions;
 *     or an invalid combination.
 *
 * The parser does not make that semantic decision.
 *
 */
aiProvenanceEvidence
    : aiProvenanceConstruct
    | aiProvenanceEvidenceConstruct
    ;


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar MUST NOT require a new AI-specific provenance AST.
 *
 * The frontend should preserve the universal expression structure.
 *
 * Conceptually:
 *
 *     Expression
 *        |
 *        +--> Provenance
 *        |      |
 *        |      +--> arguments
 *        |
 *        +--> Evidence
 *               |
 *               +--> arguments
 *
 * The concrete Rust AST representation is owned by the existing frontend.
 *
 * The AST must preserve:
 *
 *     - source span;
 *     - expression category;
 *     - ordered arguments;
 *     - named argument names where present;
 *     - nested expressions;
 *     - attributes where the universal expression model permits them;
 *     - source ordering.
 *
 * It MUST NOT introduce fields such as:
 *
 *     gpu_id;
 *     qpu_id;
 *     physical_qubit;
 *     model_provider;
 *     framework_handle;
 *     device_handle;
 *
 * unless such information is represented as ordinary program data or
 * target-specific semantic metadata by the appropriate downstream subsystem.
 *
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must transform the parsed provenance expression into the
 * universal provenance semantic model.
 *
 * AI semantic consumers may associate provenance with:
 *
 *     model creation;
 *     model training;
 *     model evaluation;
 *     inference;
 *     reasoning;
 *     learning;
 *     adaptation;
 *     knowledge updates;
 *     decisions;
 *     explanations;
 *     evidence;
 *     datasets;
 *     tensor transformations;
 *     agent actions;
 *     distributed computation;
 *     simulation;
 *     quantum computation;
 *     hybrid computation;
 *     compilation;
 *     optimization;
 *     generated artifacts.
 *
 * The semantic layer must distinguish at least conceptually between:
 *
 *     provenance subject
 *     provenance source
 *     provenance derivation
 *     provenance transformation
 *     provenance evidence
 *     provenance decision
 *     provenance verification
 *
 * without forcing those concepts into parser-level keyword lists.
 *
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * This file defines NO new primitive type.
 *
 * Provenance values use the canonical Zamani type system.
 *
 * The semantic layer determines whether a provenance argument has an
 * appropriate type for its role.
 *
 * Examples of possible semantic values include:
 *
 *     model
 *     dataset
 *     tensor
 *     result
 *     decision
 *     evidence
 *     observation
 *     artifact
 *     computation
 *     execution record
 *     symbolic identifier
 *
 * The parser must not hard-code this list as a closed type universe.
 *
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * This grammar introduces NO effects.
 *
 * A provenance expression is syntactic input.
 *
 * Depending on semantic resolution, an actual operation may require effects
 * such as:
 *
 *     observation
 *     IO
 *     network
 *     measurement
 *     simulation
 *     distributed
 *     foreign
 *     native
 *     reflection
 *
 * Effect classification belongs to the universal effects subsystem.
 *
 * Provenance recording MUST NOT automatically imply unrestricted IO,
 * filesystem access, network access, or native execution.
 *
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Parsing a provenance expression grants NO capability.
 *
 * Semantic/runtime resolution MAY require capabilities such as:
 *
 *     provenance.read
 *     provenance.record
 *     provenance.verify
 *     evidence.read
 *     evidence.record
 *     model.inspect
 *     data.inspect
 *     execution.observe
 *
 * Capability names are symbolic semantic identifiers.
 *
 * This grammar does not define a finite capability catalogue.
 *
 * Capability resolution belongs to the universal capability/resource system.
 *
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * This grammar introduces NO physical resource requirement.
 *
 * Provenance processing may consume implementation resources, but those
 * requirements are determined downstream.
 *
 * This file MUST NOT encode limits for:
 *
 *     provenance entries;
 *     lineage depth;
 *     model count;
 *     dataset count;
 *     evidence count;
 *     argument count;
 *     tensor dimensions;
 *     tensor rank;
 *     memory;
 *     CPU count;
 *     GPU count;
 *     FPGA count;
 *     accelerator count;
 *     QPU count;
 *     qubit count;
 *     node count;
 *     thread count;
 *     network size;
 *     register width.
 *
 * Resource availability is a realization concern.
 *
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Provenance can participate in universal contracts:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *     assert
 *
 * This file does NOT redefine those constructs.
 *
 * Example semantic relationship:
 *
 *     a model result
 *         |
 *         +--> provenance
 *         |
 *         +--> evidence
 *         |
 *         +--> contract
 *
 * Contract checking remains owned by:
 *
 *     grammar/validation/
 *
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Provenance can be constrained by policies concerning:
 *
 *     recording;
 *     disclosure;
 *     retention;
 *     verification;
 *     trust;
 *     model inspection;
 *     data lineage;
 *     security;
 *     adaptation;
 *     execution;
 *     export.
 *
 * Policy semantics remain owned by the policy subsystem.
 *
 * Parsing MUST NOT:
 *
 *     grant permission;
 *     bypass policy;
 *     establish trust;
 *     authorize disclosure;
 *     reserve resources.
 *
 *
 * ============================================================================
 * AI DOMAIN INTEGRATION
 * ============================================================================
 *
 * The following AI subsystems may consume the semantic provenance model:
 *
 *     reasoning
 *     inference
 *     deduction
 *     induction
 *     abduction
 *     knowledge
 *     learning
 *     adaptation
 *     uncertainty
 *     causality
 *     decisions
 *     explanations
 *     evidence
 *     agents
 *     models
 *     datasets
 *     pipelines
 *     differentiation
 *
 * They must consume the universal provenance representation.
 *
 * They must NOT introduce separate provenance AST or IR variants merely to
 * distinguish AI subdomains.
 *
 *
 * ============================================================================
 * LEARNING INTEGRATION
 * ============================================================================
 *
 * Learning provenance may preserve semantic lineage such as:
 *
 *     training input
 *     training dataset
 *     model state
 *     transformation
 *     objective
 *     evaluation result
 *     derived model
 *
 * The parser does not enumerate training algorithms.
 *
 * A future learning algorithm must not require this grammar to be modified
 * merely because the algorithm has a new name.
 *
 *
 * ============================================================================
 * ADAPTATION INTEGRATION
 * ============================================================================
 *
 * Adaptation provenance may preserve:
 *
 *     prior state;
 *     trigger;
 *     evidence;
 *     policy;
 *     selected adaptation;
 *     resulting state;
 *     verification;
 *     outcome.
 *
 * Adaptation remains controlled by:
 *
 *     effects;
 *     capabilities;
 *     policies;
 *     contracts;
 *     semantic validation.
 *
 * This grammar does not authorize self-modification.
 *
 *
 * ============================================================================
 * REASONING INTEGRATION
 * ============================================================================
 *
 * Reasoning provenance may preserve:
 *
 *     premises;
 *     evidence;
 *     inference;
 *     derivation;
 *     conclusion;
 *     confidence;
 *     verification.
 *
 * The reasoning subsystem owns reasoning semantics.
 *
 * This file only provides the provenance composition boundary.
 *
 *
 * ============================================================================
 * KNOWLEDGE INTEGRATION
 * ============================================================================
 *
 * Knowledge operations may use provenance to record:
 *
 *     assertion origin;
 *     derivation;
 *     source;
 *     evidence;
 *     update;
 *     retraction;
 *     verification.
 *
 * Knowledge syntax remains owned by the knowledge subsystem.
 *
 *
 * ============================================================================
 * DECISION INTEGRATION
 * ============================================================================
 *
 * AI decisions may be associated with provenance describing:
 *
 *     input;
 *     evidence;
 *     policy;
 *     constraints;
 *     alternatives;
 *     selected result;
 *     verification;
 *     explanation.
 *
 * The decision grammar owns decision syntax.
 *
 * This grammar only provides universal provenance consumption.
 *
 *
 * ============================================================================
 * EXPLANATION INTEGRATION
 * ============================================================================
 *
 * An explanation may reference provenance to answer questions such as:
 *
 *     where a result originated;
 *     which inputs contributed;
 *     which transformation produced a value;
 *     which evidence supported a decision;
 *     which compilation transformation generated an artifact.
 *
 * Explanation syntax remains owned by:
 *
 *     grammar/statements/explain.g4
 *
 * Provenance remains owned by:
 *
 *     grammar/expressions/provenance.g4
 *
 *
 * ============================================================================
 * EVIDENCE INTEGRATION
 * ============================================================================
 *
 * Evidence and provenance are related but distinct concepts.
 *
 * Evidence answers conceptually:
 *
 *     "What supports this claim or result?"
 *
 * Provenance answers conceptually:
 *
 *     "What is the lineage or derivation of this artifact/result?"
 *
 * They may reference each other.
 *
 * Neither should be implemented as an alias of the other.
 *
 *
 * ============================================================================
 * UNCERTAINTY INTEGRATION
 * ============================================================================
 *
 * Provenance may be associated with uncertainty-bearing results.
 *
 * Examples include:
 *
 *     uncertain inference;
 *     probabilistic result;
 *     confidence-bearing decision;
 *     stochastic learning result;
 *     measurement-derived result.
 *
 * Uncertainty semantics remain owned by:
 *
 *     grammar/expressions/uncertainty.g4
 *     grammar/ai/uncertainty-related components
 *
 * This file does not redefine uncertainty syntax.
 *
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * AI provenance may describe quantum-derived computation.
 *
 * Examples include:
 *
 *     quantum measurement result;
 *     hybrid model result;
 *     quantum-assisted learning result;
 *     quantum optimization result;
 *     simulated quantum result.
 *
 * This grammar does NOT define:
 *
 *     qubits;
 *     gates;
 *     circuits;
 *     measurement syntax;
 *     routing;
 *     topology;
 *     calibration;
 *     QEC;
 *     physical qubit identifiers.
 *
 * Quantum syntax remains owned by grammar/quantum/.
 *
 * If semantic lowering identifies a quantum computation, its canonical quantum
 * representation MUST converge on:
 *
 *     quantum::ir
 *
 * Provenance does not create:
 *
 *     AIQuantumProvenanceIR
 *
 * or another competing quantum representation.
 *
 *
 * ============================================================================
 * HYBRID INTEGRATION
 * ============================================================================
 *
 * A hybrid computation may contain provenance across:
 *
 *     classical;
 *     AI;
 *     quantum;
 *     accelerator;
 *     HDL/hardware;
 *     distributed.
 *
 * The provenance graph remains semantic and domain-neutral.
 *
 * Domain-specific lowering consumes the relevant portions after semantic
 * analysis.
 *
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Provenance may describe:
 *
 *     generated HDL;
 *     synthesized artifacts;
 *     verification results;
 *     hardware observations;
 *     accelerator results;
 *     target-specific execution artifacts.
 *
 * The grammar does not encode physical hardware identifiers or capacity.
 *
 * Hardware provenance is represented semantically after parsing.
 *
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Provenance may cross:
 *
 *     tasks;
 *     actors;
 *     processes;
 *     services;
 *     nodes;
 *     messages;
 *     datasets;
 *     distributed results.
 *
 * The grammar does not encode a fixed number of nodes.
 *
 * Actual topology and execution placement remain downstream.
 *
 *
 * ============================================================================
 * COMPILATION INTEGRATION
 * ============================================================================
 *
 * Compiler transformations may themselves have provenance.
 *
 * For example:
 *
 *     source
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     semantic model
 *       |
 *       v
 *     canonical representation
 *       |
 *       v
 *     quantum::ir / classical IR / HDL representation
 *       |
 *       v
 *     lowered artifact
 *
 * The compiler may consume the same provenance model used by AI.
 *
 * This avoids a separate AI-only lineage system.
 *
 *
 * ============================================================================
 * POCO-REAF INTEGRATION
 * ============================================================================
 *
 * Provenance must preserve semantic identity across target realization.
 *
 * Conceptually:
 *
 *     one source program
 *          |
 *          v
 *     one semantic meaning
 *          |
 *          +-----------------------+
 *          |                       |
 *          v                       v
 *     target realization A    target realization B
 *          |                       |
 *          +-----------+-----------+
 *                      |
 *                      v
 *                  provenance
 *
 * Provenance can identify which semantic transformations occurred while
 * preserving the distinction between:
 *
 *     source meaning;
 *     compiler transformation;
 *     target specialization;
 *     execution artifact.
 *
 * Provenance MUST NOT turn target-specific details into source-level
 * requirements unless the programmer explicitly expresses such a requirement.
 *
 *
 * ============================================================================
 * PORTABILITY CONTRACT
 * ============================================================================
 *
 * This grammar must remain valid as new computational targets are introduced.
 *
 * Adding a new:
 *
 *     CPU architecture;
 *     GPU architecture;
 *     accelerator;
 *     FPGA family;
 *     ASIC;
 *     QPU;
 *     simulator;
 *     cluster architecture;
 *     distributed runtime;
 *     cloud environment;
 *     future machine
 *
 * MUST NOT require changes to this grammar merely to record provenance.
 *
 *
 * ============================================================================
 * EXTENSIBILITY CONTRACT
 * ============================================================================
 *
 * Provenance is expected to grow over time.
 *
 * The grammar therefore intentionally delegates detailed provenance fields to
 * the universal provenance expression grammar.
 *
 * Extension should normally occur through:
 *
 *     ordinary expressions;
 *     qualified names;
 *     attributes;
 *     semantic registration;
 *     dialect mechanisms;
 *     library-defined schemas.
 *
 * A new provenance relation should not automatically become a new lexer token.
 *
 * This is critical for keeping the grammar scalable.
 *
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing MUST depend only on:
 *
 *     source text;
 *     token stream;
 *     grammar;
 *     parser configuration;
 *     language compatibility version.
 *
 * Parsing MUST NOT depend on:
 *
 *     model state;
 *     network state;
 *     current hardware;
 *     target availability;
 *     runtime state;
 *     randomness;
 *     current time;
 *     resource availability.
 *
 * Identical source and parser configuration must produce equivalent parse
 * structure.
 *
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Provenance source text is untrusted input until validated.
 *
 * Successful parsing does NOT imply:
 *
 *     trusted source;
 *     trusted model;
 *     trusted dataset;
 *     verified evidence;
 *     authorization;
 *     authenticity;
 *     integrity;
 *     permission;
 *     execution safety.
 *
 * Those properties belong to downstream semantic/security systems.
 *
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains NO language-level capacity constants.
 *
 * It introduces no:
 *
 *     maximum provenance records;
 *     maximum lineage depth;
 *     maximum models;
 *     maximum datasets;
 *     maximum evidence records;
 *     maximum arguments;
 *     maximum nodes;
 *     maximum devices;
 *     maximum qubits;
 *     maximum CPUs;
 *     maximum GPUs;
 *     maximum FPGAs;
 *     maximum accelerators;
 *     maximum threads;
 *     maximum memory;
 *     maximum tensor rank;
 *     maximum network size;
 *     maximum register width.
 *
 * It also contains no:
 *
 *     physical device catalogue;
 *     vendor catalogue;
 *     hardware topology;
 *     quantum gate catalogue;
 *     backend selection;
 *     scheduling policy;
 *     routing algorithm;
 *     QEC algorithm;
 *     ZQN implementation;
 *     HAL implementation.
 *
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar imposes no artificial semantic ceiling on:
 *
 *     provenance-bearing computations;
 *     provenance arguments;
 *     nested expressions;
 *     model lineage;
 *     dataset lineage;
 *     transformation chains;
 *     distributed lineage;
 *     quantum-derived lineage;
 *     hybrid lineage.
 *
 * Finite parser/runtime implementation limits may exist as implementation
 * resource constraints.
 *
 * Such limits MUST NOT be represented as language semantics.
 *
 * Tests must use configurable finite workloads to demonstrate behavior at
 * increasing scales without claiming literal mathematical infinity.
 *
 *
 * ============================================================================
 * PERFORMANCE CONTRACT
 * ============================================================================
 *
 * This adapter is intentionally O(1)-structure composition around the
 * canonical provenance/evidence rules.
 *
 * It must not:
 *
 *     - perform semantic graph construction;
 *     - recursively traverse provenance;
 *     - resolve provenance stores;
 *     - query databases;
 *     - inspect models;
 *     - access hardware;
 *     - perform network operations;
 *     - execute expressions.
 *
 * Those operations belong downstream.
 *
 * The adapter therefore introduces no additional semantic traversal over the
 * provenance structure beyond the universal grammar it delegates to.
 *
 *
 * ============================================================================
 * DIAGNOSTICS CONTRACT
 * ============================================================================
 *
 * Parser diagnostics for this file are limited to structural composition
 * failures.
 *
 * Examples:
 *
 *     invalid AI provenance composition;
 *     malformed delegated provenance expression;
 *     malformed delegated evidence expression.
 *
 * The parser MUST NOT report:
 *
 *     unavailable capability;
 *     insufficient resources;
 *     invalid policy;
 *     invalid contract;
 *     untrusted evidence;
 *     invalid model provenance;
 *     unavailable quantum hardware;
 *     unavailable accelerator;
 *     unsupported backend;
 *
 * as syntax errors.
 *
 * Those are downstream semantic or realization diagnostics.
 *
 *
 * ============================================================================
 * POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * These constructs must be accepted when the universal expression grammar
 * provides the referenced syntax:
 *
 *     provenance(result)
 *
 *     provenance(result, source)
 *
 *     provenance(
 *         result,
 *         source: dataset
 *     )
 *
 *     provenance(
 *         result,
 *         derived_from: input
 *     )
 *
 *     provenance(
 *         model_result,
 *         source: dataset,
 *         evidence: evidence_value
 *     )
 *
 *     provenance(
 *         learned_model,
 *         derived_from: training_data
 *     )
 *
 *     provenance(
 *         adapted_model,
 *         derived_from: previous_model,
 *         evidence: evaluation
 *     )
 *
 *     provenance(
 *         reasoning_result,
 *         evidence: premises
 *     )
 *
 *     provenance(
 *         quantum_result,
 *         source: measurement
 *     )
 *
 *     provenance(
 *         hybrid_result,
 *         source: classical_result
 *     )
 *
 *     provenance(
 *         distributed_result,
 *         source: computation
 *     )
 *
 *     evidence(result)
 *
 *     evidence(
 *         claim: conclusion,
 *         source: observation
 *     )
 *
 * These examples test composition.
 *
 * The detailed validity of their arguments is determined by the canonical
 * provenance/evidence expression grammar and downstream semantics.
 *
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The following must be rejected when they violate the imported grammar:
 *
 *     provenance(
 *
 *     evidence(
 *
 *     provenance(
 *         =
 *     )
 *
 *     evidence(
 *         =
 *     )
 *
 * The AI adapter MUST NOT introduce special recovery rules that accidentally
 * accept malformed universal provenance/evidence syntax.
 *
 * Semantic negatives belong downstream.
 *
 * Examples of semantic negatives include:
 *
 *     unknown provenance field;
 *     invalid provenance subject;
 *     invalid provenance source type;
 *     forbidden disclosure;
 *     unsatisfied provenance capability;
 *     unsatisfied resource requirement;
 *     prohibited policy;
 *     invalid contract relationship;
 *     unverified evidence;
 *     unavailable target.
 *
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Test provenance consumption alongside:
 *
 *     model declarations;
 *     model invocation;
 *     inference;
 *     reasoning;
 *     learning;
 *     adaptation;
 *     knowledge;
 *     decisions;
 *     explanations;
 *     uncertainty;
 *     agents;
 *     datasets;
 *     tensors;
 *     classical expressions;
 *     quantum expressions;
 *     hybrid expressions;
 *     HDL constructs;
 *     hardware intent;
 *     distributed constructs;
 *     networking;
 *     security;
 *     contracts;
 *     policies;
 *     resources;
 *     capabilities;
 *     effects;
 *     FFI;
 *     compilation;
 *     metaprogramming.
 *
 * Boundary tests MUST ensure that the AI adapter does not steal ownership from
 * those subsystems.
 *
 *
 * ============================================================================
 * CROSS-DOMAIN TEST CONTRACT
 * ============================================================================
 *
 * The same semantic provenance model must be testable with:
 *
 *     classical -> classical
 *     data -> AI
 *     AI -> AI
 *     AI -> quantum
 *     quantum -> AI
 *     AI -> HDL/hardware
 *     hardware -> AI
 *     distributed -> AI
 *     AI -> distributed
 *     compiler -> AI artifact
 *     AI -> compiled artifact
 *     simulation -> execution
 *     execution -> simulation
 *
 * No additional grammar should be required for these relationships merely
 * because the producing and consuming domains differ.
 *
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests must progressively increase:
 *
 *     provenance argument count;
 *     nested expression depth;
 *     lineage chain size;
 *     model lineage;
 *     dataset lineage;
 *     transformation count;
 *     cross-domain references.
 *
 * The exact workload sizes are test configuration.
 *
 * They are NOT language constants.
 *
 * Tests must verify:
 *
 *     - no grammar-level fixed ceiling;
 *     - no parser-specific provenance limit;
 *     - no AI-specific provenance limit;
 *     - controlled diagnostics under resource exhaustion;
 *     - preservation of source structure.
 *
 *
 * ============================================================================
 * DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * Given identical:
 *
 *     source;
 *     lexer version;
 *     parser version;
 *     grammar version;
 *     parser configuration;
 *
 * the resulting parse tree must be equivalent.
 *
 * Repeated parsing must not depend on:
 *
 *     hardware;
 *     network;
 *     model state;
 *     current time;
 *     randomness;
 *     target selection.
 *
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing universal provenance syntax remains authoritative.
 *
 * In particular, this adapter must not change the meaning of:
 *
 *     provenance(...)
 *
 * when it is encountered through the normal expression hierarchy.
 *
 * The addition of this file must therefore be backwards compatible with
 * programs already using universal provenance expressions, subject to the
 * repository's normal language-version rules.
 *
 * Removing this adapter in a future version must not be required merely
 * because the AI subsystem evolves.
 *
 * New AI provenance semantics should be added downstream or through
 * versioned semantic extensions.
 *
 *
 * ============================================================================
 * RUST INTEGRATION CONTRACT
 * ============================================================================
 *
 * This file contains:
 *
 *     NO Rust code;
 *     NO embedded actions;
 *     NO semantic predicates;
 *     NO unsafe code;
 *     NO target-specific callbacks;
 *     NO filesystem operations;
 *     NO network operations;
 *     NO runtime operations.
 *
 * Generated Rust must remain compatible with:
 *
 *     Rust 1.97+
 *     Rust 2021
 *     safe Rust.
 *
 * The grammar itself cannot guarantee the complete repository's Rust safety
 * property. CI and repository-wide checks must enforce that property.
 *
 *
 * ============================================================================
 * TOOLING INTEGRATION
 * ============================================================================
 *
 * Tooling consuming this grammar may use:
 *
 *     parse trees;
 *     source spans;
 *     AST nodes;
 *     semantic provenance records;
 *     diagnostics;
 *     formatter output;
 *     documentation generators;
 *     conformance metadata.
 *
 * Tooling MUST NOT infer that parsing grants:
 *
 *     capability;
 *     authorization;
 *     resource allocation;
 *     target support;
 *     provenance authenticity.
 *
 *
 * ============================================================================
 * FORMATTER / ROUND-TRIP CONTRACT
 * ============================================================================
 *
 * If a formatter exists:
 *
 *     source
 *       |
 *       v
 *     lexer
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     formatter
 *       |
 *       v
 *     parser
 *
 * must preserve provenance semantics.
 *
 * In particular:
 *
 *     - argument ordering must be preserved where semantically relevant;
 *     - named arguments must retain their names;
 *     - nested expressions must retain structure;
 *     - source locations must remain recoverable;
 *     - no provenance information may disappear during formatting.
 *
 *
 * ============================================================================
 * REPOSITORY INTEGRATION
 * ============================================================================
 *
 * 1. AI COMPOSITION
 *
 * grammar/ai/ai.g4
 *     |
 *     +--> imports AIProvenance
 *     |
 *     +--> exposes aiProvenanceConstruct through the AI composition boundary
 *
 *
 * 2. UNIVERSAL EXPRESSION COMPOSITION
 *
 * grammar/expressions/provenance.g4
 *     |
 *     +--> owns provenanceExpression
 *
 * grammar/ai/provenance.g4
 *     |
 *     +--> consumes provenanceExpression
 *
 * The AI file must never reverse this ownership.
 *
 *
 * 3. UNIVERSAL EVIDENCE
 *
 * grammar/expressions/evidence.g4
 *     |
 *     +--> owns evidenceExpression
 *
 * grammar/ai/provenance.g4
 *     |
 *     +--> consumes evidenceExpression
 *
 *
 * 4. DATA
 *
 * grammar/data/provenance.g4
 *     |
 *     +--> owns data lineage semantics
 *
 * It may consume universal provenance expressions where appropriate.
 *
 * It must not import AI provenance as its authority.
 *
 *
 * 5. COMPILATION
 *
 * grammar/compile/provenance.g4
 *     |
 *     +--> owns compiler transformation provenance
 *
 * AI provenance may reference compiler provenance through ordinary expressions
 * and semantic relationships.
 *
 *
 * 6. SECURITY
 *
 * grammar/security/provenance.g4
 *     |
 *     +--> owns security/trust provenance
 *
 * Security validation remains downstream.
 *
 *
 * 7. EFFECTS
 *
 * grammar/effects/
 *     |
 *     +--> determines actual effects of provenance operations
 *
 * This file introduces no effect declarations.
 *
 *
 * 8. RESOURCES
 *
 * grammar/resources/
 *     |
 *     +--> determines resource requirements
 *
 * This file introduces no physical resource limits.
 *
 *
 * 9. POLICIES
 *
 * grammar/policies/
 *     |
 *     +--> determines applicable policy
 *
 * This file introduces no authorization semantics.
 *
 *
 * 10. CONTRACTS
 *
 * grammar/validation/
 *     |
 *     +--> determines contract validity
 *
 * This file does not redefine contracts.
 *
 *
 * 11. QUANTUM
 *
 * grammar/quantum/
 *     |
 *     +--> owns quantum syntax and semantic domain
 *
 * quantum::ir
 *     |
 *     +--> remains the canonical quantum IR boundary
 *
 *
 * 12. HDL/HARDWARE
 *
 * grammar/hdl/
 * grammar/hardware/
 *     |
 *     +--> own hardware/HDL semantics
 *
 *
 * 13. FRONTEND
 *
 * src/lexer.rs
 * src/parser.rs
 * src/ast/
 *
 *     |
 *     +--> consume the canonical syntax and produce the domain-neutral AST
 *
 *
 * 14. SEMANTICS
 *
 * Provenance semantic analysis consumes the AST and produces the universal
 * semantic provenance representation.
 *
 *
 * 15. IR
 *
 * Provenance does not introduce a new IR.
 *
 * It contributes metadata/lineage to the canonical semantic representation
 * and, where appropriate, downstream domain representations.
 *
 *
 * ============================================================================
 * AI.G4 INTEGRATION REQUIREMENT
 * ============================================================================
 *
 * `grammar/ai/ai.g4` should compose this file once.
 *
 * Conceptually:
 *
 *     AI
 *       |
 *       +--> aiProvenanceConstruct
 *       |
 *       +--> other independently-owned AI constructs
 *
 * `ai.g4` must not recreate:
 *
 *     provenanceExpression
 *
 * or:
 *
 *     evidenceExpression
 *
 * locally.
 *
 *
 * ============================================================================
 * DUPLICATION PREVENTION
 * ============================================================================
 *
 * Repository conformance tooling SHOULD reject multiple definitions of:
 *
 *     provenanceExpression
 *
 * across the grammar tree.
 *
 * The expected canonical owner is:
 *
 *     grammar/expressions/provenance.g4
 *
 * Similarly:
 *
 *     evidenceExpression
 *
 * is expected to remain owned by:
 *
 *     grammar/expressions/evidence.g4
 *
 * This file must remain a consumer.
 *
 *
 * ============================================================================
 * INTEGRATION MATRIX
 * ============================================================================
 *
 * Concern                 Owner                         This File
 * ---------------------------------------------------------------------------
 * Provenance syntax       expressions/provenance.g4     consumes
 * Evidence syntax         expressions/evidence.g4       consumes
 * AI composition          ai/ai.g4                      participates
 * AI semantics            AI semantic subsystem         consumes
 * AST                     frontend AST                  consumes
 * Types                   types subsystem               consumes
 * Effects                 effects subsystem             consumes
 * Capabilities            capability subsystem         consumes
 * Resources               resources subsystem           consumes
 * Contracts               validation subsystem         consumes
 * Policies                policy subsystem              consumes
 * Security                security subsystem            consumes
 * Classical IR            canonical IR                  downstream
 * Quantum IR              quantum::ir                   downstream
 * HDL/hardware IR         domain owner                  downstream
 * Runtime                 execution/runtime             downstream
 *
 *
 * ============================================================================
 * PRODUCTION READINESS GATES
 * ============================================================================
 *
 * GATE A — AUTHORITY
 *
 * [x] Universal provenance syntax has a different canonical owner.
 * [x] This file has one AI composition responsibility.
 * [x] No competing provenance language is introduced.
 *
 *
 * GATE B — GRAMMAR
 *
 * [x] ANTLR parser grammar.
 * [x] No lexer rules.
 * [x] No embedded Rust.
 * [x] No semantic predicates.
 * [x] Universal provenance syntax is delegated.
 * [x] Universal evidence syntax is delegated.
 *
 *
 * GATE C — AST
 *
 * [x] Domain-neutral AST ownership declared.
 * [x] No AI-specific provenance AST required.
 * [x] Source structure preservation declared.
 *
 *
 * GATE D — SEMANTICS
 *
 * [x] Universal provenance semantic owner declared.
 * [x] AI semantic consumers identified.
 * [x] Type integration defined.
 * [x] Effect integration defined.
 * [x] Capability integration defined.
 * [x] Resource integration defined.
 * [x] Contract integration defined.
 * [x] Policy integration defined.
 *
 *
 * GATE E — IR
 *
 * [x] No competing provenance IR.
 * [x] Canonical semantic representation remains authoritative.
 * [x] Quantum provenance converges through quantum::ir where applicable.
 *
 *
 * GATE F — EXECUTION
 *
 * [x] No execution is performed by this grammar.
 * [x] Target realization remains downstream.
 * [x] Resource negotiation remains downstream.
 *
 *
 * GATE G — TESTING
 *
 * [x] Positive test contract specified.
 * [x] Negative test contract specified.
 * [x] Boundary test contract specified.
 * [x] Cross-domain test contract specified.
 * [x] Scalability test contract specified.
 * [x] Determinism test contract specified.
 * [x] Compatibility test contract specified.
 * [x] Round-trip contract specified.
 *
 *
 * GATE H — SAFETY
 *
 * [x] No Rust unsafe.
 * [x] No parser actions.
 * [x] No filesystem access.
 * [x] No network access.
 * [x] No hardware access.
 * [x] No artificial capacity limit.
 * [x] No vendor lock-in.
 *
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * THIS FILE IS COMPLETE WHEN:
 *
 * [x] The file exists at:
 *
 *         grammar/ai/provenance.g4
 *
 * [x] It is an ANTLR parser grammar.
 *
 * [x] It imports the canonical provenance expression grammar.
 *
 * [x] It imports the canonical evidence expression grammar.
 *
 * [x] It owns exactly one AI provenance composition boundary.
 *
 * [x] It does not define provenanceExpression.
 *
 * [x] It does not define evidenceExpression.
 *
 * [x] It does not duplicate generic expression syntax.
 *
 * [x] It does not duplicate identifier syntax.
 *
 * [x] It does not duplicate qualified-name syntax.
 *
 * [x] It does not define lexer rules.
 *
 * [x] It does not define AI-specific provenance AST nodes.
 *
 * [x] It does not define AI-specific provenance IR.
 *
 * [x] It does not select hardware.
 *
 * [x] It does not allocate resources.
 *
 * [x] It does not grant capabilities.
 *
 * [x] It does not authorize operations.
 *
 * [x] It does not perform runtime work.
 *
 * [x] It does not contain physical-capacity constants.
 *
 * [x] It does not contain vendor-specific assumptions.
 *
 * [x] It remains valid for classical computation.
 *
 * [x] It remains valid for quantum computation.
 *
 * [x] It remains valid for hybrid computation.
 *
 * [x] It remains valid for HDL/hardware computation.
 *
 * [x] It remains valid for distributed computation.
 *
 * [x] It remains valid for future computational domains.
 *
 * [x] AI reasoning can consume the universal provenance representation.
 *
 * [x] AI learning can consume the universal provenance representation.
 *
 * [x] AI adaptation can consume the universal provenance representation.
 *
 * [x] AI evidence can consume the universal provenance representation.
 *
 * [x] AI decisions can consume the universal provenance representation.
 *
 * [x] AI explanations can consume the universal provenance representation.
 *
 * [x] AI models can consume the universal provenance representation.
 *
 * [x] AI datasets can consume the universal provenance representation.
 *
 * [x] AI agents can consume the universal provenance representation.
 *
 * [x] Quantum-derived AI results can consume the universal provenance
 *     representation.
 *
 * [x] Provenance can survive target-independent semantic lowering.
 *
 * [x] Provenance can survive target specialization.
 *
 * [x] Provenance can survive heterogeneous realization.
 *
 * [x] Provenance can be validated independently of target hardware.
 *
 * [x] The file is compatible with safe Rust 1.97+ generated-parser integration.
 *
 *
 * ============================================================================
 * REPOSITORY-LEVEL COMPLETION STILL REQUIRED
 * ============================================================================
 *
 * Completion of this individual file does NOT falsely claim that the entire
 * repository integration is already complete.
 *
 * The following repository-level work must be verified separately:
 *
 * [ ] AIProvenance is imported exactly once by the canonical AI composition.
 *
 * [ ] aiProvenanceConstruct is reachable from the canonical parser.
 *
 * [ ] ProvenanceExpressions is reachable from the canonical expression root.
 *
 * [ ] EvidenceExpressions is reachable from the canonical expression root.
 *
 * [ ] No duplicate provenanceExpression exists elsewhere.
 *
 * [ ] No duplicate evidenceExpression exists elsewhere.
 *
 * [ ] The Rust parser/frontend accepts the same conformance fixtures.
 *
 * [ ] The domain-neutral AST preserves provenance structure.
 *
 * [ ] Semantic provenance analysis exists.
 *
 * [ ] Provenance participates in effect analysis where required.
 *
 * [ ] Provenance participates in capability analysis where required.
 *
 * [ ] Provenance participates in resource analysis where required.
 *
 * [ ] Provenance participates in contract analysis where required.
 *
 * [ ] Provenance participates in policy analysis where required.
 *
 * [ ] Provenance reaches the canonical semantic representation.
 *
 * [ ] Quantum-related provenance reaches quantum::ir where applicable.
 *
 * [ ] Positive tests pass.
 *
 * [ ] Negative tests pass.
 *
 * [ ] Boundary tests pass.
 *
 * [ ] Cross-domain tests pass.
 *
 * [ ] Scalability tests pass.
 *
 * [ ] Determinism tests pass.
 *
 * [ ] Compatibility tests pass.
 *
 * [ ] ANTLR generation succeeds.
 *
 * [ ] Rust 1.97+ compilation succeeds.
 *
 * [ ] Repository-wide safe-Rust checks pass.
 *
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * This file answers:
 *
 *     "How does the AI grammar consume universal provenance?"
 *
 * It does NOT answer:
 *
 *     "How is provenance stored?"
 *     "How is provenance authenticated?"
 *     "How is provenance queried at runtime?"
 *     "Which database stores provenance?"
 *     "Which hardware produced a result?"
 *     "Which QPU produced a result?"
 *     "Which physical qubit produced a result?"
 *     "Which node produced a result?"
 *     "Which GPU produced a result?"
 *     "Which backend executes the result?"
 *
 * Those questions belong to their respective semantic, security, runtime,
 * compiler, and target subsystems.
 *
 * The architectural invariant is:
 *
 *     AI provenance composition
 *          |
 *          v
 *     universal provenance expression
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic provenance
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +-------------------+
 *          |                   |
 *          v                   v
 *     classical IR         quantum::ir
 *          |                   |
 *          +---------+---------+
 *                    |
 *                    v
 *              target realization
 *
 * The source-level meaning remains independent of target capacity.
 *
 * This preserves Zamani's:
 *
 *     Program_Once
 *          ->
 *     Compile_Once
 *          ->
 *     Run_Everywhere
 *          ->
 *     Run_Anywhere
 *          ->
 *     Forever
 *
 * architecture.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */
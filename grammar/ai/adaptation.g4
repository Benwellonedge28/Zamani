/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/ai/adaptation.g4
 *
 * GRAMMAR
 * -------
 * AIAdaptation
 *
 * STATUS
 * ------
 * CANONICAL AI-DOMAIN ADAPTATION COMPOSITION ADAPTER
 *
 * LANGUAGE BASELINE
 * -----------------
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
 * This file provides the AI-domain composition boundary for Zamani's
 * universal adaptation operation.
 *
 * IMPORTANT:
 *
 * This file DOES NOT define the source-level `adapt` syntax.
 *
 * The canonical source-level adaptation syntax is owned by:
 *
 *     grammar/statements/adapt.g4
 *
 * whose grammar is:
 *
 *     Adapt
 *
 * and whose public rule is:
 *
 *     adaptStatement
 *
 * This file therefore exists to make the canonical adaptation operation
 * available to the AI composition grammar without creating a second
 * adaptation language.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * The intended architecture is:
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
 *     statement composition
 *          |
 *          v
 *     Adapt.adaptStatement
 *          |
 *          v
 *     AIAdaptation.aiAdaptationConstruct
 *          |
 *          v
 *     AI.aiConstruct
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     structural validation
 *          |
 *          +-------------------+-------------------+
 *          |                   |                   |
 *          v                   v                   v
 *        types             effects            contracts
 *          |                   |                   |
 *          +-------------------+-------------------+
 *                              |
 *                  +-----------+-----------+
 *                  |           |           |
 *                  v           v           v
 *             capabilities  resources   policies
 *                  |           |           |
 *                  +-----------+-----------+
 *                              |
 *                              v
 *                         provenance
 *                              |
 *                              v
 *                    semantic adaptation model
 *                              |
 *              +---------------+----------------+
 *              |               |                |
 *              v               v                v
 *          classical       quantum::ir      other domain IR
 *                              |
 *                              v
 *                         optimization
 *                              |
 *                         specialization
 *                              |
 *                           lowering
 *                              |
 *                     routing / scheduling
 *                              |
 *                     resilience / recovery
 *                              |
 *                         ZQN / HAL
 *                              |
 *                              v
 *                       target realization
 *
 * This file MUST remain above physical realization.
 *
 * ============================================================================
 * CORE ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * Adaptation is a UNIVERSAL COMPUTATIONAL INTENT.
 *
 * It may be used to express controlled changes involving:
 *
 *     - learned models;
 *     - strategies;
 *     - policies;
 *     - execution choices;
 *     - resource decisions;
 *     - distributed strategies;
 *     - hardware-aware strategies;
 *     - quantum/classical hybrid strategies;
 *     - simulation strategies;
 *     - scheduling decisions;
 *     - routing decisions;
 *     - knowledge-derived strategies;
 *     - evidence-driven decisions;
 *     - adaptive algorithms;
 *     - future computational domains.
 *
 * This file does not define those domains.
 *
 * It only exposes the universal adaptation statement to the AI grammar.
 *
 * ============================================================================
 * OWNERSHIP CONTRACT
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 *     aiAdaptationConstruct
 *
 * This is the sole AI-domain composition rule owned by this file.
 *
 * THIS FILE DOES NOT OWN
 * ----------------------
 *
 *     - `adapt` keyword spelling;
 *     - lexer rules;
 *     - token definitions;
 *     - identifiers;
 *     - qualified names;
 *     - expressions;
 *     - expression precedence;
 *     - statements;
 *     - adaptation targets;
 *     - adaptation sources;
 *     - adaptation contexts;
 *     - policies;
 *     - capabilities;
 *     - resources;
 *     - contracts;
 *     - effects;
 *     - provenance;
 *     - authorization;
 *     - learning syntax;
 *     - inference syntax;
 *     - reasoning syntax;
 *     - knowledge syntax;
 *     - uncertainty syntax;
 *     - agent syntax;
 *     - concurrency syntax;
 *     - distributed syntax;
 *     - execution syntax;
 *     - simulation syntax;
 *     - quantum syntax;
 *     - HDL syntax;
 *     - hardware syntax;
 *     - AST implementation;
 *     - semantic implementation;
 *     - IR construction;
 *     - runtime implementation.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There MUST be exactly one canonical source-level adaptation statement.
 *
 * That owner is:
 *
 *     grammar/statements/adapt.g4
 *
 * with:
 *
 *     Adapt.adaptStatement
 *
 * This file MUST NOT define another production such as:
 *
 *     adaptationStatement
 *     aiAdaptationStatement
 *     adaptationExpression
 *     aiAdaptExpression
 *     learningAdaptation
 *     modelAdaptation
 *
 * merely to reproduce the same syntax.
 *
 * This prevents grammar divergence.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 *     grammar/statements/adapt.g4
 *
 * Grammar:
 *
 *     Adapt
 *
 * Public rule:
 *
 *     adaptStatement
 *
 * Adapt already owns the canonical source-level syntax and its expression
 * boundary.
 *
 * This file deliberately does not import individual expression grammars.
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * ANTLR imports grammar NAMES rather than filesystem paths.
 *
 * Therefore the dependency is:
 *
 *     import Adapt;
 *
 * The grammar build must make the statement grammar available to ANTLR.
 *
 * ============================================================================
 * EXPORT CONTRACT
 * ============================================================================
 *
 * EXPORTS
 * -------
 *
 *     aiAdaptationConstruct
 *
 * This is the ONLY public AI-owned rule in this file.
 *
 * The imported:
 *
 *     adaptStatement
 *
 * remains owned by:
 *
 *     Adapt
 *
 * ============================================================================
 * CONSUMER CONTRACT
 * ============================================================================
 *
 * Primary consumer:
 *
 *     grammar/ai/ai.g4
 *
 * AI.g4 MUST import:
 *
 *     AIAdaptation
 *
 * and include:
 *
 *     aiAdaptationConstruct
 *
 * in:
 *
 *     aiConstruct
 *
 * The intended composition is:
 *
 *     aiConstruct
 *         : ...
 *         | aiAdaptationConstruct
 *         | ...
 *         ;
 *
 * AI.g4 MUST NOT directly reproduce:
 *
 *     ADAPT ...
 *
 * syntax.
 *
 * ============================================================================
 * UNIVERSAL ADAPTATION BOUNDARY
 * ============================================================================
 *
 * The underlying adaptation operation is intentionally universal.
 *
 * AI adaptation is therefore not a separate language operation.
 *
 * The same canonical:
 *
 *     adaptStatement
 *
 * may be consumed by AI semantics when the adaptation target or source is
 * semantically associated with:
 *
 *     model
 *     learned strategy
 *     policy
 *     agent
 *     knowledge
 *     inference result
 *     reasoning result
 *     evidence
 *     uncertainty
 *     feedback
 *     prediction
 *     observation
 *     simulation result
 *
 * No AI-specific adaptation syntax is necessary.
 *
 * ============================================================================
 * CONTROLLED ADAPTATION CONTRACT
 * ============================================================================
 *
 * Parsing `adapt` MUST NOT imply unrestricted self-modification.
 *
 * In particular, successful parsing does not automatically authorize changes
 * to:
 *
 *     - source code;
 *     - executable code;
 *     - compiler state;
 *     - security state;
 *     - credentials;
 *     - protected memory;
 *     - policies;
 *     - hardware state;
 *     - external systems;
 *     - network resources;
 *     - deployment state.
 *
 * Adaptation is a semantic operation subject to downstream authorization.
 *
 * The conceptual semantic flow is:
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
 *     provenance recording
 *          |
 *          v
 *     semantic adaptation
 *          |
 *          v
 *     validation of resulting state
 *          |
 *          v
 *     continued execution / recompilation / lowering as required
 *
 * None of those runtime operations are performed by this grammar.
 *
 * ============================================================================
 * LEARNING INTEGRATION
 * ============================================================================
 *
 * Adaptation may consume information produced by learning.
 *
 * Examples:
 *
 *     adapt model from training_result;
 *
 *     adapt strategy from learned_policy;
 *
 *     adapt execution from prediction;
 *
 * Those examples are parsed by Adapt.
 *
 * This file does NOT define:
 *
 *     train
 *     learn
 *     fit
 *     reinforcement
 *     transfer
 *     fine_tune
 *
 * Learning remains owned by:
 *
 *     grammar/statements/learn.g4
 *     grammar/ai/learning.g4
 *
 * The semantic relationship may therefore be:
 *
 *     learning
 *         |
 *         v
 *     learned information
 *         |
 *         v
 *     adaptation
 *
 * Learning does not automatically authorize adaptation.
 *
 * ============================================================================
 * REASONING INTEGRATION
 * ============================================================================
 *
 * Adaptation may consume results from:
 *
 *     infer
 *     deduce
 *     reason
 *
 * through ordinary expressions.
 *
 * This file does not duplicate reasoning syntax.
 *
 * Reasoning remains owned by the canonical reasoning subsystem.
 *
 * ============================================================================
 * KNOWLEDGE INTEGRATION
 * ============================================================================
 *
 * Adaptation may consume knowledge-derived values.
 *
 * For example:
 *
 *     adapt strategy from knowledge_result;
 *
 * Knowledge operations remain owned by their canonical grammar.
 *
 * This file does not define:
 *
 *     assert
 *     retract
 *     query
 *     lookup
 *     knowledge
 *
 * ============================================================================
 * UNCERTAINTY INTEGRATION
 * ============================================================================
 *
 * Adaptation may consume values representing:
 *
 *     probability
 *     confidence
 *     belief
 *     distributions
 *     observations
 *     uncertain results
 *     intervals
 *     statistical evidence
 *
 * No probability representation is defined here.
 *
 * The type and semantic systems remain authoritative.
 *
 * ============================================================================
 * EVIDENCE / EXPLANATION INTEGRATION
 * ============================================================================
 *
 * Adaptation decisions may be associated with:
 *
 *     evidence
 *     explanation
 *     decision records
 *     confidence
 *     derivation
 *     verification
 *     provenance
 *
 * These are semantic relationships.
 *
 * This grammar does not create an AI-specific evidence or provenance
 * language.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing has no runtime effects.
 *
 * Semantic analysis may classify adaptation with effects including:
 *
 *     mutation
 *     adaptation
 *     learning
 *     randomness
 *     reflection
 *     code_generation
 *     native
 *     foreign
 *     network
 *     distributed
 *     measurement
 *     simulation
 *
 * The actual effect set depends on the resolved adaptation.
 *
 * This file does not create or duplicate the universal effect taxonomy.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Adaptation may require semantic capabilities such as:
 *
 *     adaptation
 *     model.update
 *     strategy.update
 *     runtime.adaptation
 *     learning.update
 *     data.read
 *     knowledge.read
 *     network
 *     distributed.compute
 *     quantum.measurement
 *     tensor.compute
 *
 * These are examples of semantic capability identifiers.
 *
 * They are NOT a closed catalogue owned by this file.
 *
 * Capability resolution belongs to:
 *
 *     grammar/resources/
 *     semantic capability analysis
 *     execution planning
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Adaptation may require:
 *
 *     memory
 *     computation
 *     tensor resources
 *     accelerator resources
 *     quantum resources
 *     distributed resources
 *     network resources
 *     storage
 *     energy
 *     bandwidth
 *     other future resources
 *
 * No resource quantity is encoded in this grammar.
 *
 * Requirements are resolved downstream.
 *
 * The source remains valid independently of the eventual machine scale.
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * Adaptation participates in the universal contract system.
 *
 * Relevant semantic constructs include:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * This file does not duplicate any contract grammar.
 *
 * Contract analysis determines:
 *
 *     preconditions;
 *     postconditions;
 *     invariants;
 *     assumptions;
 *     guarantees;
 *     verification obligations.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Adaptation is policy-sensitive.
 *
 * Policies may control:
 *
 *     - what may be adapted;
 *     - who or what may authorize adaptation;
 *     - which effects are permitted;
 *     - which capabilities may be consumed;
 *     - which resources may be consumed;
 *     - whether network access is allowed;
 *     - whether foreign/native operations are allowed;
 *     - whether model state may change;
 *     - whether execution may change;
 *     - whether provenance is mandatory;
 *     - whether rollback is required;
 *     - whether adaptation may continue execution.
 *
 * Policy syntax remains owned by the policy subsystem.
 *
 * A parsed adaptation is NOT automatically policy-authorized.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Adaptation may change semantic state.
 *
 * Downstream provenance should therefore be capable of recording, where
 * applicable:
 *
 *     source
 *     target
 *     adaptation operation
 *     source information
 *     context
 *     evidence
 *     decision
 *     policy
 *     capability decision
 *     resource decision
 *     contract result
 *     resulting state
 *     verification
 *     language version
 *     grammar version
 *     compiler version
 *     target capability snapshot
 *
 * This grammar preserves the parse structure needed for source provenance.
 *
 * It does not create a second provenance model.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This file introduces NO AI-specific AST.
 *
 * The frontend must map:
 *
 *     aiAdaptationConstruct
 *          |
 *          v
 *     adaptStatement
 *          |
 *          v
 *     domain-neutral AdaptationStatement
 *
 * The resulting AST must remain independent of:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     vendor
 *     compiler backend
 *     topology
 *     physical resource identity
 *     routing
 *     scheduling
 *     QEC
 *     calibration
 *
 * The AST represents semantic intent, not physical realization.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for determining:
 *
 *     - what is being adapted;
 *     - whether the target is adaptable;
 *     - what source information is consumed;
 *     - what context applies;
 *     - which types are involved;
 *     - which effects are produced;
 *     - which capabilities are required;
 *     - which resources are required;
 *     - which contracts apply;
 *     - which policies apply;
 *     - whether authorization exists;
 *     - whether provenance is sufficient;
 *     - whether the resulting state is valid;
 *     - whether adaptation preserves program semantics.
 *
 * The parser MUST NOT perform those decisions.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO AI-specific IR.
 *
 * Adaptation first enters the domain-neutral semantic model and then lowers
 * according to the affected computation.
 *
 * Possible downstream destinations include:
 *
 *     classical IR
 *     tensor/data IR
 *     distributed IR
 *     accelerator IR
 *     hardware/HDL IR
 *     quantum::ir
 *     other future domain IR
 *
 * If an adaptation affects quantum computation, the quantum portion MUST
 * ultimately cross the canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * This file must never introduce:
 *
 *     AIAdaptationIR
 *     LearningAdaptationIR
 *     AIQuantumIR
 *     AIHardwareIR
 *
 * as competing representations.
 *
 * ============================================================================
 * QUANTUM / HYBRID CONTRACT
 * ============================================================================
 *
 * AI adaptation may affect hybrid computation.
 *
 * Examples include semantic decisions involving:
 *
 *     classical model
 *     quantum computation
 *     measurement result
 *     adaptive circuit choice
 *     hybrid strategy
 *     quantum resource selection
 *
 * The source grammar remains unchanged.
 *
 * The semantic path is:
 *
 *     adaptation intent
 *          |
 *          v
 *     semantic model
 *          |
 *          +--------------------+
 *          |                    |
 *          v                    v
 *     classical             quantum semantics
 *                               |
 *                               v
 *                          quantum::ir
 *
 * This file does not define quantum operations.
 *
 * ============================================================================
 * DISTRIBUTED / MULTI-AGENT CONTRACT
 * ============================================================================
 *
 * Adaptation may participate in:
 *
 *     actor systems
 *     agent systems
 *     message-driven execution
 *     distributed execution
 *     federated computation
 *     collective computation
 *     fault recovery
 *
 * Actor/message syntax remains owned by:
 *
 *     grammar/concurrency/
 *     grammar/distributed/
 *
 * AI agent semantics remain owned by the AI subsystem.
 *
 * This adapter only exposes the universal adaptation operation to AI
 * composition.
 *
 * ============================================================================
 * EXECUTION CONTRACT
 * ============================================================================
 *
 * Adaptation may result in:
 *
 *     strategy selection
 *     fallback
 *     retry
 *     recovery
 *     rescheduling
 *     rerouting
 *     target re-selection
 *     model update
 *     state update
 *     execution-plan modification
 *
 * These are downstream execution semantics.
 *
 * This grammar does not decide whether adaptation requires:
 *
 *     recompilation
 *     relowering
 *     rerouting
 *     rescheduling
 *     runtime mutation
 *     migration
 *     recovery
 *
 * ============================================================================
 * ADAPTIVE EXECUTION CONTRACT
 * ============================================================================
 *
 * Adaptation integrates with the execution/resilience subsystem.
 *
 * Runtime decisions may result in:
 *
 *     ACCEPT
 *     DEGRADED_ACCEPT
 *     RETRY
 *     RECOVER
 *     ESCALATE
 *     REJECT
 *
 * and resilience states such as:
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
 * These are runtime/semantic concepts, not parser alternatives.
 *
 * ============================================================================
 * SIMULATION CONTRACT
 * ============================================================================
 *
 * Adaptation may be evaluated during:
 *
 *     classical simulation
 *     quantum simulation
 *     hardware simulation
 *     distributed simulation
 *     AI simulation
 *     fault simulation
 *     performance simulation
 *
 * Simulation remains an execution strategy.
 *
 * This grammar does not create a simulation-specific adaptation syntax.
 *
 * ============================================================================
 * HARD-CODING / SCALABILITY CONTRACT
 * ============================================================================
 *
 * This file MUST remain open-ended.
 *
 * It contains:
 *
 *     no fixed model count;
 *     no fixed adaptation count;
 *     no fixed context count;
 *     no fixed target count;
 *     no fixed resource count;
 *     no fixed capability count;
 *     no fixed agent count;
 *     no fixed worker count;
 *     no fixed thread count;
 *     no fixed node count;
 *     no fixed accelerator count;
 *     no fixed device count;
 *     no fixed quantum-resource count;
 *     no fixed tensor rank;
 *     no fixed memory size;
 *     no fixed topology size.
 *
 * In particular, this file MUST NOT introduce universal capacity constants.
 *
 * Source-level adaptation remains independent of machine scale.
 *
 * "Infinity" therefore means **no grammar-level artificial ceiling**; actual
 * execution remains bounded by available resources, implementation limits,
 * target capabilities, declared requirements, policies, and physical
 * feasibility.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * The same source-level adaptation syntax must remain valid whether the
 * eventual realization is:
 *
 *     tiny embedded computation
 *     CPU
 *     multicore
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     simulator
 *     HPC
 *     cluster
 *     distributed system
 *     cloud system
 *     future hardware
 *
 * The adaptation grammar must never encode a particular realization.
 *
 * ============================================================================
 * TARGET / HARDWARE CONTRACT
 * ============================================================================
 *
 * This file does not select:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     node
 *     device
 *     vendor
 *     cloud provider
 *
 * Hardware realization belongs downstream.
 *
 * Semantic resource and capability negotiation determines what realization is
 * feasible.
 *
 * ============================================================================
 * DIALECT CONTRACT
 * ============================================================================
 *
 * Vendor-specific and framework-specific adaptation mechanisms MUST remain
 * outside this grammar.
 *
 * Examples include:
 *
 *     vendor model update APIs
 *     accelerator-specific tuning
 *     provider-specific deployment mechanisms
 *     framework-specific optimizer names
 *     hardware-specific recovery mechanisms
 *
 * Such features belong in:
 *
 *     dialects/
 *     interoperability/
 *     execution/
 *     hardware/
 *     compile/
 *
 * They may consume the universal adaptation semantic model.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Adaptation is security-sensitive because it may alter execution behavior.
 *
 * Security controls may include:
 *
 *     authorization
 *     sandboxing
 *     capability restrictions
 *     effect restrictions
 *     provenance requirements
 *     audit requirements
 *     rollback requirements
 *     trust policies
 *
 * This grammar does not implement security enforcement.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing MUST be deterministic.
 *
 * This grammar contains:
 *
 *     - no embedded actions;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware discovery;
 *     - no resource discovery;
 *     - no random behavior;
 *     - no runtime execution.
 *
 * For a fixed:
 *
 *     source text
 *     lexer version
 *     parser grammar version
 *     enabled dialect configuration
 *
 * parsing must produce deterministic parser structure.
 *
 * Runtime adaptation may be nondeterministic when the semantic/runtime
 * contract explicitly permits it.
 *
 * Reproducibility controls belong downstream in execution/provenance.
 *
 * ============================================================================
 * RUST / SAFETY CONTRACT
 * ============================================================================
 *
 * This file contains no Rust implementation code.
 *
 * Generated frontend code must remain compatible with:
 *
 *     Rust 1.97 or later
 *     Rust 2021
 *     safe Rust only
 *
 * No `unsafe` implementation is required by this grammar.
 *
 * The grammar itself performs no:
 *
 *     I/O
 *     process execution
 *     filesystem access
 *     network communication
 *     device access
 *     accelerator discovery
 *     QPU discovery
 *     resource allocation
 *     model loading
 *     dataset loading.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * The canonical adaptation surface is inherited from:
 *
 *     Adapt.adaptStatement
 *
 * Therefore changes to adaptation syntax MUST be made once in:
 *
 *     grammar/statements/adapt.g4
 *
 * This adapter continues to expose that canonical rule.
 *
 * No AI-specific compatibility syntax should be introduced here.
 *
 * Existing valid forms remain:
 *
 *     adapt target;
 *
 *     adapt target from source;
 *
 *     adapt target with (context);
 *
 *     adapt target from source with (context, context);
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE
 * --------
 *
 * AI composition must accept canonical adaptation forms such as:
 *
 *     adapt model;
 *
 *     adapt model from feedback;
 *
 *     adapt strategy with (policy);
 *
 *     adapt model from training_result with (policy, evidence);
 *
 *     adapt execution_strategy from observation with
 *         (resource_state, policy, provenance);
 *
 * These are tests of composition through the canonical Adapt grammar.
 *
 * NEGATIVE
 * --------
 *
 * The AI adapter must not make invalid adaptation syntax valid.
 *
 * Examples:
 *
 *     adapt;
 *
 *     adapt model from;
 *
 *     adapt model with ();
 *
 *     adapt model with (policy,);
 *
 *     adapt model with (,policy);
 *
 *     adapt model with (policy,,feedback);
 *
 *     adapt model with (policy) from feedback;
 *
 *     adapt;
 *
 * must remain rejected by the canonical statement grammar.
 *
 * BOUNDARY
 * --------
 *
 * Test adaptation with:
 *
 *     learning result
 *     reasoning result
 *     knowledge result
 *     evidence
 *     uncertainty
 *     policy
 *     capability
 *     resource observation
 *     simulation result
 *     quantum-derived value
 *     distributed result
 *     hardware observation
 *     actor/agent state
 *
 * CROSS-DOMAIN
 * ------------
 *
 * Verify adaptation composition with:
 *
 *     classical computation
 *     AI
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     distributed
 *     networking
 *     simulation
 *     execution
 *     security
 *
 * SCALABILITY
 * -----------
 *
 * Verify that the grammar does not introduce ceilings for:
 *
 *     adaptation operations
 *     context entries
 *     semantic domains
 *     model complexity
 *     resource quantities
 *     capability quantities
 *     agent counts
 *     distributed participants
 *     quantum resources
 *     hardware resources
 *
 * DETERMINISM
 * -----------
 *
 * Identical source/token streams must produce deterministic parse structures.
 *
 * PORTABILITY
 * -----------
 *
 * The same source forms must remain syntactically independent of target
 * hardware availability.
 *
 * ============================================================================
 * INTEGRATION TEST PROGRAMS
 * ============================================================================
 *
 * At minimum, AI conformance should include:
 *
 *     grammar/tests/ai/adaptation/
 *
 * with:
 *
 *     valid-minimal.zm
 *     valid-learning.zm
 *     valid-reasoning.zm
 *     valid-policy.zm
 *     valid-evidence.zm
 *     valid-uncertainty.zm
 *     valid-hybrid.zm
 *     valid-quantum.zm
 *     valid-distributed.zm
 *     invalid-missing-target.zm
 *     invalid-empty-context.zm
 *     invalid-trailing-context-comma.zm
 *     invalid-clause-order.zm
 *
 * The tests should exercise the adapter through:
 *
 *     AI
 *
 * and separately exercise:
 *
 *     Adapt
 *
 * as the canonical statement grammar.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * REQUIRED CHANGE 1 — AI COMPOSITION
 * -----------------------------------
 *
 * Update:
 *
 *     grammar/ai/ai.g4
 *
 * Add:
 *
 *     AIAdaptation
 *
 * to the canonical AI imports.
 *
 * Add:
 *
 *     aiAdaptationConstruct
 *
 * to:
 *
 *     aiConstruct
 *
 * For example:
 *
 *     import
 *         Types,
 *         Expressions,
 *         Statements,
 *         AIModels,
 *         AIDatasets,
 *         AITensors,
 *         AITraining,
 *         Inference,
 *         Agents,
 *         AIDifferentiable,
 *         AIPipelines,
 *         AIAccelerators,
 *         ModelDeployment,
 *         AILearning,
 *         AIAdaptation
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
 *         | agentConstruct
 *         | aiDifferentiationConstruct
 *         | pipelineConstruct
 *         | aiAcceleratorConstruct
 *         | deploymentConstruct
 *         | aiLearningConstruct
 *         | aiAdaptationConstruct
 *         | aiCapabilityConstruct
 *         ;
 *
 * The exact existing import ordering should be preserved where practical.
 *
 * REQUIRED CHANGE 2 — CANONICAL STATEMENT GRAMMAR
 * ------------------------------------------------
 *
 * Do NOT modify:
 *
 *     grammar/statements/adapt.g4
 *
 * merely to integrate this AI adapter.
 *
 * It already owns:
 *
 *     adaptStatement
 *
 * The adapter consumes it.
 *
 * REQUIRED CHANGE 3 — LEXER
 * --------------------------
 *
 * Do NOT add or redefine:
 *
 *     ADAPT
 *
 * in this file.
 *
 * The canonical lexer already owns the keyword.
 *
 * REQUIRED CHANGE 4 — ROOT PARSER
 * --------------------------------
 *
 * Do NOT import AIAdaptation directly into:
 *
 *     grammar/Zamani.g4
 *
 * or:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * if the existing architecture continues to route AI through:
 *
 *     AI
 *
 * The intended path is:
 *
 *     ZamaniParser
 *          |
 *          v
 *         AI
 *          |
 *          v
 *     AIAdaptation
 *          |
 *          v
 *     Adapt
 *
 * This prevents root-level grammar coupling.
 *
 * REQUIRED CHANGE 5 — AST
 * ------------------------
 *
 * No AI-specific AST type should be introduced.
 *
 * The AST adapter must consume:
 *
 *     adaptStatement
 *
 * and create the existing domain-neutral adaptation representation.
 *
 * REQUIRED CHANGE 6 — SEMANTICS
 * ------------------------------
 *
 * Semantic analysis must resolve:
 *
 *     target
 *     source
 *     context
 *     type
 *     effects
 *     capabilities
 *     resources
 *     contracts
 *     policies
 *     authorization
 *     provenance
 *
 * No semantic decision belongs in this grammar.
 *
 * REQUIRED CHANGE 7 — EFFECTS
 * ----------------------------
 *
 * Use the existing:
 *
 *     grammar/effects/adaptation.g4
 *
 * as the adaptation-effect boundary.
 *
 * Do not duplicate its effect syntax here.
 *
 * REQUIRED CHANGE 8 — RESOURCES / CAPABILITIES
 * ---------------------------------------------
 *
 * Use:
 *
 *     grammar/resources/
 *
 * for capability/resource semantics.
 *
 * Do not introduce AI-specific resource limits.
 *
 * REQUIRED CHANGE 9 — POLICIES
 * -----------------------------
 *
 * Use the shared policy subsystem for:
 *
 *     authorization
 *     adaptation permissions
 *     resource restrictions
 *     effect restrictions
 *     security restrictions
 *     deployment restrictions
 *
 * Do not create AI-only policy syntax here.
 *
 * REQUIRED CHANGE 10 — PROVENANCE
 * -------------------------------
 *
 * Adaptation provenance must use the repository-wide provenance model.
 *
 * Do not create:
 *
 *     AIAdaptationProvenance
 *
 * as a competing provenance representation.
 *
 * ============================================================================
 * DEPENDENCY GRAPH
 * ============================================================================
 *
 *     grammar/ai/adaptation.g4
 *                 |
 *                 v
 *     grammar/statements/adapt.g4
 *                 |
 *                 v
 *        ReasonStatements
 *                 |
 *                 v
 *            Expressions
 *
 * AI composition:
 *
 *     grammar/ai/ai.g4
 *             |
 *       +-----+------+
 *       |            |
 *       v            v
 * AIAdaptation   AILearning
 *       |            |
 *       v            v
 *     Adapt         Learn
 *       |            |
 *       +-----+------+
 *             |
 *             v
 *       domain-neutral AST
 *
 * Semantic dependencies:
 *
 *     AST
 *      |
 *      +--> Types
 *      +--> Effects
 *      +--> Contracts
 *      +--> Capabilities
 *      +--> Resources
 *      +--> Policies
 *      +--> Provenance
 *      +--> Execution
 *      +--> Security
 *      |
 *      v
 * Semantic adaptation model
 *
 * Downstream:
 *
 *     semantic model
 *          |
 *          +--> classical IR
 *          |
 *          +--> quantum::ir
 *          |
 *          +--> other domain IR
 *
 * ============================================================================
 * FILE COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when all of the following are true:
 *
 * [x] It is a parser grammar.
 *
 * [x] It owns exactly one AI composition rule.
 *
 * [x] It imports the canonical Adapt grammar.
 *
 * [x] It does not duplicate adaptStatement.
 *
 * [x] It does not define lexer rules.
 *
 * [x] It does not define keyword tokens.
 *
 * [x] It does not define an expression hierarchy.
 *
 * [x] It does not define resource syntax.
 *
 * [x] It does not define capability syntax.
 *
 * [x] It does not define policy syntax.
 *
 * [x] It does not define contract syntax.
 *
 * [x] It does not define provenance syntax.
 *
 * [x] It does not define learning syntax.
 *
 * [x] It does not define reasoning syntax.
 *
 * [x] It does not define quantum syntax.
 *
 * [x] It does not define hardware syntax.
 *
 * [x] It does not create an AI-specific AST.
 *
 * [x] It does not create an AI-specific IR.
 *
 * [x] It preserves the canonical Adapt syntax.
 *
 * [x] It is independent of physical machine scale.
 *
 * [x] It introduces no fixed resource capacities.
 *
 * [x] It introduces no vendor assumptions.
 *
 * [x] It contains no embedded Rust.
 *
 * [x] It requires no unsafe Rust.
 *
 * [x] It has deterministic parser behavior.
 *
 * [x] It is compatible with Rust 1.97 or later generated-parser integration.
 *
 * [ ] AI.g4 imports AIAdaptation.
 *
 * [ ] AI.g4 includes aiAdaptationConstruct.
 *
 * [ ] AST adaptation mapping is implemented.
 *
 * [ ] Semantic adaptation analysis is implemented.
 *
 * [ ] Effect analysis consumes the semantic adaptation.
 *
 * [ ] Capability analysis consumes the semantic adaptation.
 *
 * [ ] Resource analysis consumes the semantic adaptation.
 *
 * [ ] Contract analysis consumes the semantic adaptation.
 *
 * [ ] Policy analysis consumes the semantic adaptation.
 *
 * [ ] Provenance analysis consumes the semantic adaptation.
 *
 * [ ] Cross-domain tests pass.
 *
 * [ ] Scalability tests pass.
 *
 * [ ] Determinism tests pass.
 *
 * [ ] Compatibility tests pass.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * This file answers exactly one question:
 *
 *     "How does the canonical universal adaptation operation enter the
 *      Zamani AI composition boundary?"
 *
 * It does NOT answer:
 *
 *     "How is adaptation executed?"
 *
 *     "Which model is modified?"
 *
 *     "Which hardware executes it?"
 *
 *     "Which resources are selected?"
 *
 *     "Which policy authorizes it?"
 *
 *     "Which capability realizes it?"
 *
 *     "How is quantum routing performed?"
 *
 *     "How is scheduling performed?"
 *
 *     "How is QEC performed?"
 *
 *     "How is deployment performed?"
 *
 * Those responsibilities remain downstream.
 *
 * The portability invariant is:
 *
 *     Program Once
 *          ->
 *     Compile Once
 *          ->
 *     Run Everywhere
 *          ->
 *     Run Anywhere
 *          ->
 *     Forever
 *
 * subject to program semantics, declared requirements, capabilities,
 * policies, contracts, implementation resources and target feasibility.
 *
 * ============================================================================
 */

/*
 * ============================================================================
 * PRODUCTION GRAMMAR
 * ============================================================================
 */

parser grammar AIAdaptation;

import Adapt;


/*
 * ============================================================================
 * PUBLIC AI ADAPTATION COMPOSITION BOUNDARY
 * ============================================================================
 *
 * The canonical adaptation syntax remains entirely owned by:
 *
 *     Adapt.adaptStatement
 *
 * This adapter introduces no new syntax.
 * ============================================================================
 */

aiAdaptationConstruct
    : adaptStatement
    ;
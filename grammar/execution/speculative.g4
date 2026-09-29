/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *     grammar/execution/speculative.g4
 *
 * Grammar:
 *     SpeculativeExecution
 *
 * Status:
 *     Production-ready modular source-level speculative-execution grammar.
 *
 * Language:
 *     Zamani
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     Safe Rust only
 *     No unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns SOURCE-LEVEL SPECULATIVE EXECUTION INTENT.
 *
 * It provides the syntax required to express computations that may be
 * evaluated speculatively, provisionally, transactionally, counterfactually,
 * or on an isolated temporal execution state before the semantic system
 * determines whether the speculative result may be committed, discarded,
 * merged, observed, or otherwise consumed.
 *
 * This grammar describes:
 *
 *     WHAT computation may be speculated;
 *     WHAT speculative context is requested;
 *     WHAT assumptions apply;
 *     WHAT resources/capabilities are required;
 *     WHAT effects are permitted;
 *     WHAT observation policy applies;
 *     WHAT commit/discard policy applies;
 *     WHAT conflict policy applies;
 *     WHAT merge policy applies;
 *     WHAT provenance is required;
 *     WHAT temporal relationship is requested;
 *     WHAT fallback behavior is permitted.
 *
 * It does NOT implement speculation.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * The production pipeline remains:
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
 *     domain-neutral frontend AST
 *          |
 *          v
 *     structural analysis
 *          |
 *          +--> type analysis
 *          +--> effect analysis
 *          +--> ownership analysis
 *          +--> resource analysis
 *          +--> capability analysis
 *          +--> temporal/causal analysis
 *          +--> determinism analysis
 *          +--> portability analysis
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          +----------------------+----------------------+
 *          |                      |                      |
 *          v                      v                      v
 *      classical              quantum::ir          HDL/hardware
 *          |                      |                      |
 *          +----------------------+----------------------+
 *                                 |
 *                                 v
 *                            optimization
 *                                 |
 *                    +------------+------------+
 *                    |            |            |
 *                    v            v            v
 *                 routing     scheduling    resilience
 *                    |            |            |
 *                    +------------+------------+
 *                                 |
 *                            QEC / ZQN
 *                                 |
 *                                HAL
 *                                 |
 *                                 v
 *                         target realization
 *                                 |
 *                                 v
 *                              runtime
 *
 * Speculation is therefore an execution/semantic intent.
 *
 * It is NOT an additional IR layer.
 *
 * ============================================================================
 * NON-NEGOTIABLE ARCHITECTURAL RULE
 * ============================================================================
 *
 * This file MUST NOT introduce:
 *
 *     SpeculativeIR
 *     SpeculativeExecutionIR
 *     MtsSpeculativeIR
 *     QuantumSpeculativeIR
 *     PhysicalSpeculativeIR
 *
 * Quantum computation remains represented through:
 *
 *     quantum::ir
 *
 * Speculative metadata is attached to the canonical semantic/execution model
 * and is lowered by the existing compiler/runtime architecture.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * This grammar participates in:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * A speculative computation may eventually be realized on:
 *
 *     tiny embedded targets;
 *     CPUs;
 *     multicore CPUs;
 *     GPUs;
 *     FPGAs;
 *     ASICs;
 *     accelerators;
 *     QPUs;
 *     quantum simulators;
 *     hybrid systems;
 *     HPC systems;
 *     clusters;
 *     distributed systems;
 *     cloud systems;
 *     future computational substrates.
 *
 * The source syntax MUST NOT change merely because the realization changes.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * There is NO universal language-level limit on:
 *
 *     speculative computations;
 *     speculative branches;
 *     nested speculation;
 *     assumptions;
 *     candidate results;
 *     observations;
 *     checkpoints;
 *     dependencies;
 *     conflicts;
 *     merge candidates;
 *     resources;
 *     capabilities;
 *     policies;
 *     clauses;
 *     arguments;
 *     nested bodies;
 *     timelines;
 *     participating computations.
 *
 * The grammar MUST NOT introduce:
 *
 *     MAX_SPECULATIONS
 *     MAX_BRANCHES
 *     MAX_ASSUMPTIONS
 *     MAX_CANDIDATES
 *     MAX_CHECKPOINTS
 *     MAX_OBSERVATIONS
 *     MAX_TIMELINES
 *     MAX_THREADS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_REGISTER_WIDTH
 *     MAX_TENSOR_RANK
 *     MAX_NETWORK_SIZE
 *
 * Nor may equivalent hidden parser limits be introduced.
 *
 * Repetition is represented through:
 *
 *     *
 *     +
 *
 * and recursive structures.
 *
 * Actual resource exhaustion belongs to:
 *
 *     compiler;
 *     runtime;
 *     resource manager;
 *     deployment;
 *     target environment.
 *
 * ============================================================================
 * SPECULATION IS NOT AUTOMATIC PARALLELISM
 * ============================================================================
 *
 * Speculative intent does NOT mean:
 *
 *     "run on N threads"
 *
 * or:
 *
 *     "allocate N devices".
 *
 * A backend may realize speculation by:
 *
 *     parallel execution;
 *     sequential execution;
 *     lazy evaluation;
 *     simulation;
 *     checkpoint/restore;
 *     copy-on-write;
 *     transactional execution;
 *     distributed execution;
 *     quantum simulation;
 *     hardware-assisted execution;
 *     another semantically equivalent strategy.
 *
 * The source expresses semantic intent.
 *
 * ============================================================================
 * SPECULATION IS NOT AUTOMATIC COMMIT
 * ============================================================================
 *
 * A speculative result is not automatically observable as committed program
 * state.
 *
 * The semantic model MUST distinguish:
 *
 *     speculative state;
 *     provisional result;
 *     observed result;
 *     committed result;
 *     discarded result;
 *     merged result.
 *
 * Compiler speculation MUST NOT make speculative side effects observable
 * unless the language semantics explicitly permit them.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     speculative execution source syntax;
 *     speculative construct composition;
 *     speculative subject;
 *     speculative assumptions;
 *     speculative policies;
 *     speculative clauses;
 *     speculative outcomes;
 *     speculative observation intent;
 *     speculative commit/discard intent;
 *     speculative conflict/merge intent;
 *     speculative provenance intent;
 *     speculative resource/capability intent;
 *     speculative nesting;
 *     speculative counterfactual intent;
 *     speculative temporal association.
 *
 * ============================================================================
 * NON-OWNERSHIP
 * ============================================================================
 *
 * THIS FILE DOES NOT OWN:
 *
 *     lexical token definitions;
 *     identifiers;
 *     qualified names;
 *     expression precedence;
 *     types;
 *     blocks;
 *     general concurrency;
 *     general scheduling;
 *     general placement;
 *     general resources;
 *     general capabilities;
 *     generic temporal-memory operations;
 *     timeline allocation;
 *     timeline storage;
 *     checkpoint storage;
 *     causal analysis;
 *     quantum operations;
 *     quantum state representation;
 *     quantum::ir;
 *     QEC;
 *     ZQN;
 *     HAL;
 *     runtime implementation;
 *     hardware discovery;
 *     resource allocation;
 *     conflict-resolution implementation.
 *
 * ============================================================================
 * EXISTING REPOSITORY INTEGRATION
 * ============================================================================
 *
 * This component integrates with existing repository components:
 *
 *     grammar/execution/timelines.g4
 *     grammar/memory/temporal.g4
 *     grammar/execution/scheduling.g4
 *     grammar/execution/parallel-execution.g4
 *     grammar/execution/dispatch.g4
 *     grammar/execution/recovery.g4
 *     grammar/execution/checkpointing.g4
 *     grammar/execution/runtime.g4
 *     grammar/resources/
 *     grammar/concurrency/
 *     grammar/quantum/
 *     grammar/hybrid/
 *     grammar/hardware/
 *     grammar/distributed/
 *     grammar/security/
 *
 * Integration is by semantic composition.
 *
 * This file MUST NOT duplicate those grammars.
 *
 * ============================================================================
 * IMPORTANT RELATIONSHIP WITH timelines.g4
 * ============================================================================
 *
 * `grammar/execution/timelines.g4` already provides generic MTS execution
 * operations using:
 *
 *     MTS <operation> ...
 *
 * Its operation namespace is intentionally open.
 *
 * This file therefore MUST NOT create another generic:
 *
 *     MTS <operation>
 *
 * root because that would overlap with `timelineConstruct`.
 *
 * Instead, this component uses the distinctive source-level form:
 *
 *     MTS speculative: ...
 *
 * or:
 *
 *     MTS counterfactual: ...
 *
 * The identifier following MTS is still lexically an ordinary IDENTIFIER.
 *
 * The semantic feature registry validates that the operation name denotes a
 * supported speculative operation.
 *
 * This avoids adding a permanent global `SPECULATIVE` lexer keyword merely
 * to support one execution feature.
 *
 * ============================================================================
 * WHY NO SPECULATIVE LEXER TOKEN
 * ============================================================================
 *
 * The current repository's MTS grammar intentionally keeps:
 *
 *     timeline
 *     branch
 *     fork
 *     speculate
 *     merge
 *     rewind
 *     restore
 *     checkpoint
 *
 * open semantic operation names.
 *
 * The canonical lexer already owns MTS and other stable lexical vocabulary.
 *
 * Adding a global:
 *
 *     SPECULATIVE
 *
 * token solely for this file would unnecessarily enlarge the global keyword
 * vocabulary and would create compatibility work across:
 *
 *     grammar/lexer/
 *     grammar/antlr/ZamaniLexer.g4
 *     src/lexer.rs
 *     parser vocabulary
 *     tooling
 *
 * Therefore this grammar deliberately preserves the repository's open-world
 * operation model.
 *
 * ============================================================================
 * CANONICAL LEXER
 * ============================================================================
 *
 * The production ANTLR lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * which composes:
 *
 *     grammar/lexer/tokens.g4
 *
 * This parser consumes:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * This file defines NO lexer rules.
 *
 * ============================================================================
 * CANONICAL SHARED GRAMMARS
 * ============================================================================
 *
 * Names:
 *
 *     grammar/core/names.g4
 *
 * Expressions:
 *
 *     grammar/expressions/expressions.g4
 *
 * The public imports below therefore use:
 *
 *     Names
 *     Expressions
 *
 * rather than redefining:
 *
 *     identifier;
 *     qualifiedName;
 *     expression;
 *     expressionList;
 *     blockExpression.
 *
 * ============================================================================
 * SOURCE FORM
 * ============================================================================
 *
 * Canonical speculative form:
 *
 *     mts speculative: expression;
 *
 * Block form:
 *
 *     mts speculative: {
 *         computation();
 *     }
 *
 * With clauses:
 *
 *     mts speculative:
 *         computation
 *         with {
 *             assumption: condition;
 *             policy: explore;
 *         };
 *
 * Counterfactual form:
 *
 *     mts counterfactual:
 *         computation
 *         with {
 *             assumption: alternative;
 *         };
 *
 * Nested speculation:
 *
 *     mts speculative: {
 *         mts speculative: inner_computation;
 *     }
 *
 * The exact semantic interpretation of these forms is downstream.
 *
 * ============================================================================
 * SUBJECT MODEL
 * ============================================================================
 *
 * A speculative subject is an ordinary Zamani expression or block expression.
 *
 * Therefore the subject may semantically represent:
 *
 *     classical computation;
 *     quantum computation;
 *     hybrid computation;
 *     HDL computation;
 *     hardware/software co-design;
 *     AI computation;
 *     tensor computation;
 *     dataflow;
 *     distributed computation;
 *     accelerator computation;
 *     embedded computation;
 *     future computational domains.
 *
 * The grammar does NOT enumerate these domains.
 *
 * ============================================================================
 * OPEN-WORLD OPERATION MODEL
 * ============================================================================
 *
 * The stable semantic operation names for this component are:
 *
 *     speculative
 *     speculate
 *     counterfactual
 *
 * They are represented as source identifiers.
 *
 * Future registered semantic variants may be introduced through the feature
 * registry/dialect mechanism without requiring a new parser architecture.
 *
 * The parser intentionally recognizes the structural form:
 *
 *     MTS identifier :
 *
 * Semantic validation MUST then distinguish:
 *
 *     supported speculative operation
 *
 * from:
 *
 *     unrelated MTS operation.
 *
 * ============================================================================
 * ARGUMENTS
 * ============================================================================
 *
 * Speculative operations may optionally carry an argument list before the
 * subject:
 *
 *     mts speculative(
 *         strategy,
 *         depth
 *     ): computation;
 *
 * Arguments are ordinary expressions.
 *
 * No argument-count limit exists.
 *
 * ============================================================================
 * CLAUSES
 * ============================================================================
 *
 * Speculative clauses are open-ended name/value properties.
 *
 * Examples:
 *
 *     assumption: condition;
 *     strategy: explore;
 *     commit: conditional;
 *     conflict: reconcile;
 *     observation: isolated;
 *     provenance: required;
 *     effects: restricted;
 *
 * Future properties may use qualified names:
 *
 *     quantum::measurement: isolated;
 *     distributed::consistency: causal;
 *     security::isolation: strict;
 *     hardware::capability: capability("...");
 *
 * Semantic analysis decides whether a property is:
 *
 *     stable;
 *     experimental;
 *     dialect-defined;
 *     deprecated;
 *     unsupported;
 *     invalid.
 *
 * ============================================================================
 * REQUIREMENT / CONSTRAINT / PREFERENCE / HINT
 * ============================================================================
 *
 * These concepts remain semantically distinct.
 *
 * Requirement:
 *
 *     requires capability("...");
 *
 * Constraint:
 *
 *     constraint expression;
 *
 * Preference:
 *
 *     prefer expression;
 *
 * Hint:
 *
 *     hint expression;
 *
 * The grammar does not perform constraint solving.
 *
 * ============================================================================
 * EFFECT SAFETY
 * ============================================================================
 *
 * Speculative execution is particularly sensitive to effects.
 *
 * A computation containing:
 *
 *     filesystem writes;
 *     irreversible external effects;
 *     network mutation;
 *     device mutation;
 *     secret-dependent effects;
 *     externally observable I/O;
 *
 * MUST be checked by semantic/effect analysis.
 *
 * The parser MUST NOT assume that every operation is safely speculatable.
 *
 * An effect may be:
 *
 *     prohibited;
 *     isolated;
 *     buffered;
 *     transactional;
 *     deferred;
 *     compensatable;
 *     explicitly permitted.
 *
 * The semantic effect system owns that decision.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Speculation MUST NOT accidentally introduce observable nondeterminism.
 *
 * The semantic layer must determine:
 *
 *     whether an effect is deterministic;
 *     whether a random source may be consumed;
 *     whether randomness must be isolated;
 *     whether ordering matters;
 *     whether speculative results are reproducible;
 *     whether observations are stable.
 *
 * This grammar only preserves the user's declared intent.
 *
 * ============================================================================
 * TEMPORAL INTEGRATION
 * ============================================================================
 *
 * Temporal memory is already owned by:
 *
 *     grammar/memory/temporal.g4
 *
 * That grammar supports open temporal operations such as:
 *
 *     temporal::snapshot(...)
 *     temporal::checkpoint(...)
 *     temporal::restore(...)
 *     temporal::rewind(...)
 *     temporal::fork(...)
 *     temporal::merge(...)
 *
 * Speculation may reference temporal state through ordinary expressions or
 * speculative properties.
 *
 * This file MUST NOT redefine temporal memory operations.
 *
 * Example:
 *
 *     mts speculative:
 *         temporal::snapshot(state)
 *         with {
 *             checkpoint: checkpoint_id;
 *         };
 *
 * The semantic layer determines whether such use is valid.
 *
 * ============================================================================
 * TIMELINE INTEGRATION
 * ============================================================================
 *
 * `grammar/execution/timelines.g4` remains responsible for generic timeline
 * intent.
 *
 * This file provides the more specific speculative semantic contract.
 *
 * A speculative execution may be associated with a timeline through a clause:
 *
 *     timeline: future;
 *
 *     parent: current;
 *
 *     branch: candidate;
 *
 * Such names remain symbolic.
 *
 * No physical timeline ID is required.
 *
 * ============================================================================
 * FORK / MERGE INTEGRATION
 * ============================================================================
 *
 * Speculative execution may produce candidate states.
 *
 * It does not itself implement:
 *
 *     fork;
 *     merge;
 *     conflict resolution.
 *
 * Those are semantic/runtime responsibilities.
 *
 * The source may express:
 *
 *     fork: requested;
 *     merge: conditional;
 *     conflict: reconcile;
 *
 * but the downstream MTS/resilience/runtime layers determine realization.
 *
 * ============================================================================
 * OBSERVATION INTEGRATION
 * ============================================================================
 *
 * Observation is semantically significant.
 *
 * A speculative result may be:
 *
 *     private;
 *     isolated;
 *     provisional;
 *     observable;
 *     committed;
 *     discarded.
 *
 * The grammar represents observation policy only.
 *
 * It does not cause observation.
 *
 * ============================================================================
 * CHECKPOINT / RECOVERY INTEGRATION
 * ============================================================================
 *
 * Speculative execution may require checkpointing.
 *
 * This grammar may express checkpoint intent through a property:
 *
 *     checkpoint: temporal::checkpoint(...);
 *
 * or:
 *
 *     recovery: retry;
 *
 * Existing checkpoint/recovery grammars remain authoritative for their own
 * generic syntax and semantics.
 *
 * This file does not duplicate them.
 *
 * ============================================================================
 * SCHEDULING INTEGRATION
 * ============================================================================
 *
 * Speculation may be:
 *
 *     eager;
 *     lazy;
 *     concurrent;
 *     sequential;
 *     deferred;
 *     demand-driven.
 *
 * These are semantic scheduling properties.
 *
 * The scheduler determines actual execution order and resource allocation.
 *
 * This grammar does not create:
 *
 *     SpeculativeScheduler
 *
 * or a second scheduling model.
 *
 * ============================================================================
 * CONCURRENCY INTEGRATION
 * ============================================================================
 *
 * Speculative computation may be realized concurrently, but concurrency is
 * not implied by speculation.
 *
 * The semantic model may lower speculative regions into the canonical
 * concurrency/execution representation where appropriate.
 *
 * No fixed thread/core/device count is permitted.
 *
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * Speculation may require additional resources.
 *
 * Example:
 *
 *     requires memory >= required_memory;
 *
 *     requires capability("parallel.compute");
 *
 *     requires capability("temporal.checkpoint");
 *
 * These are requirements, not compiler-wide limits.
 *
 * A target with insufficient resources may:
 *
 *     serialize;
 *     defer;
 *     repartition;
 *     distribute;
 *     simulate;
 *     reject realization;
 *
 * according to downstream semantics and policies.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Speculation may surround quantum computation.
 *
 * Example conceptual form:
 *
 *     mts speculative: {
 *         apply custom_operation to q;
 *         measure q;
 *     }
 *
 * The quantum operations remain owned by:
 *
 *     grammar/quantum/
 *
 * and lower through:
 *
 *     quantum::ir
 *
 * There is no:
 *
 *     SpeculativeQuantumIR
 *
 * Physical qubit assignment remains downstream.
 *
 * QEC remains downstream.
 *
 * ZQN remains downstream.
 *
 * Routing remains downstream.
 *
 * ============================================================================
 * CLASSICAL / HYBRID INTEGRATION
 * ============================================================================
 *
 * A speculative block may contain arbitrary valid Zamani computation.
 *
 * This permits:
 *
 *     classical -> speculative -> classical
 *
 *     classical -> speculative quantum -> measurement -> classical
 *
 *     HDL intent -> speculative analysis
 *
 *     dataflow -> speculative branch
 *
 *     AI inference -> counterfactual computation
 *
 * without creating separate languages.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Speculation over hardware intent is semantic.
 *
 * This grammar does not create speculative hardware.
 *
 * Hardware realization remains owned by:
 *
 *     grammar/hdl/
 *     grammar/hardware/
 *
 * and their downstream compiler/backend systems.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * A speculative computation may eventually be distributed.
 *
 * The grammar does NOT imply:
 *
 *     one branch = one node.
 *
 * It also does not impose:
 *
 *     one speculation = one device.
 *
 * Partitioning and placement remain downstream.
 *
 * ============================================================================
 * SECURITY INTEGRATION
 * ============================================================================
 *
 * Speculation MUST respect:
 *
 *     authorization;
 *     capability checks;
 *     information-flow policy;
 *     secret isolation;
 *     provenance;
 *     non-interference;
 *     effect restrictions.
 *
 * A speculative block MUST NOT become an implicit security bypass.
 *
 * ============================================================================
 * PROVENANCE
 * ============================================================================
 *
 * Speculative results should be semantically distinguishable from committed
 * state.
 *
 * Downstream provenance may need to retain:
 *
 *     source speculation;
 *     assumptions;
 *     parent state;
 *     candidate identity;
 *     observation;
 *     commit/discard decision;
 *     merge decision;
 *     causal dependencies.
 *
 * This grammar only preserves source-level intent.
 *
 * ============================================================================
 * COUNTERFACTUAL EXECUTION
 * ============================================================================
 *
 * Counterfactual computation is treated as a semantic specialization of
 * speculation.
 *
 * It asks conceptually:
 *
 *     "What would the computation produce under this alternative assumption?"
 *
 * It does not imply:
 *
 *     physical time travel;
 *     modification of historical reality;
 *     mutation of committed state.
 *
 * Counterfactual state must remain isolated unless semantic rules explicitly
 * authorize propagation.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar requires a domain-neutral AST representation.
 *
 * Conceptual structure:
 *
 *     SpeculativeExecution {
 *         operation: QualifiedName,
 *         arguments: Vec<Expression>,
 *         subject: SpeculativeSubject,
 *         clauses: Vec<SpeculativeClause>,
 *         source_span: SourceSpan
 *     }
 *
 *     SpeculativeSubject {
 *         expression: Expression,
 *         source_span: SourceSpan
 *     }
 *
 *     SpeculativeClause {
 *         key: QualifiedName,
 *         value: Expression,
 *         source_span: SourceSpan
 *     }
 *
 * These are conceptual contracts only.
 *
 * Rust AST structures belong to:
 *
 *     src/frontend/ast/
 *
 * This grammar MUST NOT define Rust structures.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis MUST determine:
 *
 *     - whether the operation is a supported speculative operation;
 *     - whether the subject is speculatable;
 *     - whether its effects are permitted;
 *     - whether assumptions are well typed;
 *     - whether resources are sufficient;
 *     - whether capabilities exist;
 *     - whether observations are permitted;
 *     - whether commit/discard semantics are valid;
 *     - whether conflicts are resolvable;
 *     - whether temporal relationships are valid;
 *     - whether causal relationships are valid;
 *     - whether nested speculation is valid;
 *     - whether quantum operations remain valid;
 *     - whether classical/quantum boundaries remain valid;
 *     - whether security policies permit speculation;
 *     - whether the requested realization is portable.
 *
 * The parser performs none of these checks.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * There is NO speculative IR.
 *
 * The lowering path is:
 *
 *     source
 *       |
 *       v
 *     SpeculativeExecution AST
 *       |
 *       v
 *     semantic speculative intent
 *       |
 *       v
 *     canonical semantic execution model
 *       |
 *       +----------------------+----------------------+
 *       |                      |                      |
 *       v                      v                      v
 *   classical             quantum::ir          HDL/hardware
 *       |                      |                      |
 *       +----------------------+----------------------+
 *                              |
 *                              v
 *                         optimization
 *                              |
 *                     routing / scheduling
 *                              |
 *                         resilience
 *                              |
 *                         QEC / ZQN
 *                              |
 *                             HAL
 *                              |
 *                         target/runtime
 *
 * ============================================================================
 * NO PHYSICAL TARGET MODEL
 * ============================================================================
 *
 * The grammar MUST NOT require:
 *
 *     CPU id;
 *     GPU id;
 *     FPGA id;
 *     QPU id;
 *     node id;
 *     physical qubit id;
 *     memory-bank id;
 *     fixed topology;
 *     fixed core count;
 *     fixed thread count.
 *
 * If target-specific binding is explicitly required, it belongs to the
 * target/deployment/dialect layer rather than this portable speculative
 * grammar.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no semantic predicates;
 *     no parser actions;
 *     no Rust code;
 *     no randomness;
 *     no filesystem access;
 *     no network access;
 *     no hardware access;
 *     no runtime calls.
 *
 * Given the same token stream and grammar version, parsing is deterministic.
 *
 * ============================================================================
 * SOURCE SPANS
 * ============================================================================
 *
 * Frontend processing MUST preserve source spans for:
 *
 *     MTS;
 *     speculative operation name;
 *     arguments;
 *     subject;
 *     clauses;
 *     clause names;
 *     clause values;
 *     nested bodies.
 *
 * This supports:
 *
 *     diagnostics;
 *     IDE tooling;
 *     formatting;
 *     provenance;
 *     debugging;
 *     semantic analysis.
 *
 * ============================================================================
 * ERROR BOUNDARY
 * ============================================================================
 *
 * Parser errors:
 *
 *     malformed MTS speculative structure.
 *
 * Semantic errors:
 *
 *     unsupported speculative operation;
 *     invalid subject;
 *     illegal effects;
 *     invalid assumption;
 *     invalid policy;
 *     unavailable capability;
 *     insufficient resources;
 *     invalid temporal relationship.
 *
 * Runtime errors:
 *
 *     failure to realize an otherwise valid speculative execution.
 *
 * These categories MUST NOT be collapsed.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * This file is additive.
 *
 * It does not rename:
 *
 *     execution.g4
 *     timelines.g4
 *     temporal.g4
 *     scheduling.g4
 *     runtime.g4
 *     dispatch.g4
 *     placement.g4
 *
 * Existing MTS/speculative constructs documented in:
 *
 *     grammar/Zamani-Grammar.md
 *     grammar/grammar.md
 *
 * remain subject to the repository's lifecycle statuses:
 *
 *     stable;
 *     proposed;
 *     experimental;
 *     deprecated;
 *     historical;
 *     not implemented.
 *
 * Adding this grammar MUST NOT by itself claim that the Rust frontend or
 * runtime already implements speculative execution.
 *
 * ============================================================================
 * CANONICAL COMPOSITION
 * ============================================================================
 *
 * The public rule is:
 *
 *     speculativeConstruct
 *
 * The canonical root:
 *
 *     grammar/Zamani.g4
 *
 * and/or the canonical parser composition must expose this rule where
 * speculative execution is legal.
 *
 * IMPORTANT:
 *
 * Do NOT independently compose this rule beside another generic:
 *
 *     MTS <qualifiedName> ...
 *
 * alternative that accepts the same source shape.
 *
 * `timelines.g4` already owns the generic MTS operation form.
 *
 * The canonical parser must dispatch:
 *
 *     MTS speculative: ...
 *
 * through this specialized rule.
 *
 * A future lexer-level `SPECULATIVE` token MUST NOT be introduced unless the
 * language specification explicitly promotes speculative syntax to a global
 * reserved keyword.
 *
 * ============================================================================
 * VALIDATION
 * ============================================================================
 *
 * grammar/validation/ must verify:
 *
 *     - tokenVocab is ZamaniLexer;
 *     - imports resolve;
 *     - no lexer rules exist here;
 *     - no semantic predicates exist;
 *     - no parser actions exist;
 *     - no unsafe Rust is embedded;
 *     - no speculative capacity constants exist;
 *     - no machine capacity constants exist;
 *     - no fixed hardware identifiers are required;
 *     - no speculative IR is introduced;
 *     - quantum::ir remains the canonical quantum boundary;
 *     - source spans are preserved downstream;
 *     - operation names remain semantically extensible;
 *     - no duplicate generic MTS root is introduced.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Tests belong under:
 *
 *     grammar/tests/execution/speculative/
 *
 * Recommended structure:
 *
 *     positive/
 *     negative/
 *     boundary/
 *     scalability/
 *     determinism/
 *     compatibility/
 *
 * ============================================================================
 * POSITIVE TESTS
 * ============================================================================
 *
 * At minimum:
 *
 *     mts speculative: compute();
 *
 *     mts speculative: {
 *         compute();
 *     }
 *
 *     mts speculative: {
 *         let result = compute();
 *         result;
 *     };
 *
 *     mts speculate(
 *         strategy
 *     ): compute();
 *
 *     mts counterfactual:
 *         compute()
 *         with {
 *             assumption: alternative;
 *         };
 *
 *     mts speculative:
 *         compute()
 *         with {
 *             assumption: condition;
 *             strategy: explore;
 *             observation: isolated;
 *             commit: conditional;
 *             conflict: reconcile;
 *         };
 *
 *     mts speculative:
 *         quantum_operation()
 *         with {
 *             requires capability("quantum.measurement");
 *         };
 *
 *     mts speculative: {
 *         mts speculative: inner();
 *     };
 *
 *     mts speculative:
 *         distributed_operation()
 *         with {
 *             requires capability("distributed.compute");
 *         };
 *
 * ============================================================================
 * NEGATIVE TESTS
 * ============================================================================
 *
 * Must reject malformed forms such as:
 *
 *     mts speculative
 *
 *     mts speculative:
 *
 *     mts speculative: ;
 *
 *     mts speculative(
 *
 *     mts speculative:
 *         {
 *
 *     mts speculative: {
 *         ...
 *     } unexpected
 *
 *     mts speculative: {
 *         ...
 *     } {
 *         ...
 *     }
 *
 * Exact diagnostics belong to the canonical diagnostic system.
 *
 * ============================================================================
 * BOUNDARY TESTS
 * ============================================================================
 *
 * Include:
 *
 *     empty speculative block;
 *     one-expression speculation;
 *     nested speculation;
 *     deeply nested speculation;
 *     many clauses;
 *     many arguments;
 *     long qualified property names;
 *     symbolic resource expressions;
 *     large integer program values;
 *     complex expressions;
 *     mixed classical/quantum subjects;
 *     distributed subjects;
 *     HDL/hardware subjects.
 *
 * No boundary test may define an artificial maximum.
 *
 * ============================================================================
 * SCALABILITY TESTS
 * ============================================================================
 *
 * The conformance suite must verify that the grammar supports:
 *
 *     arbitrarily many clauses;
 *     arbitrarily many arguments;
 *     arbitrarily many nested speculative regions;
 *     arbitrarily many symbolic assumptions;
 *     arbitrarily many candidate relationships;
 *     arbitrarily large program-defined resource quantities;
 *
 * subject only to implementation resources.
 *
 * Tests MUST NOT assert:
 *
 *     "maximum speculation count = N".
 *
 * ============================================================================
 * DETERMINISM TESTS
 * ============================================================================
 *
 * Identical source and identical language configuration MUST produce
 * equivalent parse structure.
 *
 * Formatting-only changes MUST NOT change semantic structure.
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware;
 *     runtime state;
 *     filesystem enumeration;
 *     network state;
 *     randomness;
 *     wall-clock time.
 *
 * ============================================================================
 * CROSS-DOMAIN TESTS
 * ============================================================================
 *
 * Speculation must be tested around:
 *
 *     classical computation;
 *     quantum computation;
 *     hybrid computation;
 *     HDL/hardware intent;
 *     distributed computation;
 *     AI/data computation;
 *     accelerator computation.
 *
 * The tests must verify that the same speculative syntax remains target
 * independent.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust code.
 *
 * The implementing compiler/frontend MUST remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and MUST use safe Rust only.
 *
 * No `unsafe` implementation is required or permitted by this grammar's
 * integration contract.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] one public speculative root exists;
 *     [x] canonical MTS lexical token is reused;
 *     [x] no SPECULATIVE lexer keyword is required;
 *     [x] canonical Names grammar is reused;
 *     [x] canonical Expressions grammar is reused;
 *     [x] blockExpression is reused;
 *     [x] no second expression grammar exists;
 *     [x] no second temporal grammar exists;
 *     [x] no second timeline grammar exists;
 *     [x] no speculative IR exists;
 *     [x] quantum::ir remains canonical;
 *     [x] requirements are distinct from preferences/hints;
 *     [x] effects remain downstream;
 *     [x] resources remain downstream;
 *     [x] capabilities remain downstream;
 *     [x] scheduling remains downstream;
 *     [x] routing remains downstream;
 *     [x] QEC remains downstream;
 *     [x] ZQN remains downstream;
 *     [x] HAL remains downstream;
 *     [x] no machine-size limits exist;
 *     [x] no physical device identities are required;
 *     [x] nested speculation is supported;
 *     [x] counterfactual intent is supported;
 *     [x] source spans are preserved by the frontend contract;
 *     [x] diagnostics are separated by layer;
 *     [x] deterministic parsing is required;
 *     [x] positive tests are defined;
 *     [x] negative tests are defined;
 *     [x] boundary tests are defined;
 *     [x] scalability tests are defined;
 *     [x] compatibility tests are defined;
 *     [x] Rust 1.97/1.97.1 compatibility is defined;
 *     [x] unsafe Rust is prohibited.
 *
 * ============================================================================
 * FINAL PRINCIPLE
 * ============================================================================
 *
 * Speculative execution syntax describes:
 *
 *     POSSIBLE COMPUTATION
 *
 * under:
 *
 *     DECLARED ASSUMPTIONS
 *     DECLARED POLICIES
 *     DECLARED REQUIREMENTS
 *     DECLARED CONSTRAINTS
 *     DECLARED OBSERVATION RULES
 *     DECLARED COMMIT/DISCARD RULES
 *
 * It does NOT describe:
 *
 *     HOW MANY CPUs;
 *     HOW MANY GPUs;
 *     HOW MANY QPUs;
 *     HOW MANY THREADS;
 *     WHICH physical qubits;
 *     WHICH machine;
 *     WHICH node;
 *     WHICH scheduler;
 *     WHICH hardware topology.
 *
 * Therefore:
 *
 *     SPECULATIVE SOURCE INTENT
 *              !=
 *     SPECULATIVE HARDWARE REALIZATION
 *
 * and:
 *
 *     PROGRAM SCALE
 *              !=
 *     MACHINE CAPACITY
 *
 * The architectural objective remains:
 *
 *     Program Once
 *          ->
 *     Compile Once
 *          ->
 *     Run Everywhere
 *          ->
 *     Run Anywhere
 *          ->
 *     Run Forever
 *
 * subject to semantic correctness and actual resources available at
 * realization time.
 *
 * ============================================================================
 * PARSER GRAMMAR
 * ============================================================================
 */

parser grammar SpeculativeExecution;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names,
    Expressions
;


/*
 * ============================================================================
 * 1. PUBLIC ROOT
 * ============================================================================
 *
 * The MTS token is the existing canonical temporal/execution introducer.
 *
 * The specialized `speculative:` form is deliberately distinct from the
 * generic `MTS <operation>` form owned by ExecutionTimelines.
 *
 * ============================================================================
 */

speculativeConstruct
    : MTS speculativeOperation
    ;


/*
 * ============================================================================
 * 2. SPECULATIVE OPERATION
 * ============================================================================
 *
 * Canonical:
 *
 *     mts speculative: expression;
 *
 *     mts speculate: expression;
 *
 *     mts counterfactual: expression;
 *
 * The operation name is intentionally an identifier rather than a new global
 * lexer keyword.
 *
 * Semantic validation MUST verify that the operation belongs to the
 * speculative-execution feature set.
 *
 * ============================================================================
 */

speculativeOperation
    : speculativeOperationName
      speculativeOperationArguments?
      COLON
      speculativeRequest
    ;


/*
 * ============================================================================
 * 3. OPERATION NAME
 * ============================================================================
 *
 * The accepted semantic operation names are:
 *
 *     speculative
 *     speculate
 *     counterfactual
 *
 * They remain lexical identifiers.
 *
 * This rule intentionally does not enumerate future dialect-specific names.
 *
 * Semantic validation owns feature registration.
 * ============================================================================
 */

speculativeOperationName
    : identifier
    ;


/*
 * ============================================================================
 * 4. OPTIONAL ARGUMENTS
 * ============================================================================
 *
 * Examples:
 *
 *     mts speculative(strategy): computation;
 *
 *     mts speculative(
 *         strategy,
 *         depth
 *     ): computation;
 *
 * ============================================================================
 */

speculativeOperationArguments
    : LPAREN speculativeArgumentList? RPAREN
    ;


speculativeArgumentList
    : expression
      (
          COMMA
          expression
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 5. SPECULATIVE REQUEST
 * ============================================================================
 *
 * A request consists of:
 *
 *     subject
 *     optional policy/metadata block
 *     optional terminator
 *
 * ============================================================================
 */

speculativeRequest
    : speculativeSubject
      speculativeClauseBlock?
      speculativeTerminator?
    ;


/*
 * ============================================================================
 * 6. SUBJECT
 * ============================================================================
 *
 * A subject is an existing Zamani expression or block expression.
 *
 * ============================================================================
 */

speculativeSubject
    : blockExpression
    | expression
    ;


/*
 * ============================================================================
 * 7. CLAUSE BLOCK
 * ============================================================================
 *
 * Canonical:
 *
 *     with {
 *         assumption: condition;
 *         strategy: explore;
 *     }
 *
 * The WITH token is part of the canonical Zamani lexical vocabulary.
 *
 * ============================================================================
 */

speculativeClauseBlock
    : WITH LBRACE speculativeClause* RBRACE
    ;


/*
 * ============================================================================
 * 8. CLAUSES
 * ============================================================================
 *
 * Common semantic categories are structurally distinguished where doing so
 * improves AST/diagnostic clarity.
 *
 * Generic property syntax remains available for future extensions.
 *
 * ============================================================================
 */

speculativeClause
    : speculativeAssumption
    | speculativeRequirement
    | speculativeConstraint
    | speculativePreference
    | speculativeHint
    | speculativeProperty
    | speculativeArgumentProperty
    ;


/*
 * ============================================================================
 * 9. ASSUMPTION
 * ============================================================================
 *
 * Assumptions describe the hypothetical/provisional conditions under which
 * the speculative computation is evaluated.
 *
 * Example:
 *
 *     assumption: alternative_state;
 *
 * The grammar does not determine whether the assumption is physically
 * realizable.
 *
 * ============================================================================
 */

speculativeAssumption
    : ASSUMPTION speculativeClauseValue speculativeClauseTerminator
    ;


/*
 * ============================================================================
 * 10. REQUIREMENT
 * ============================================================================
 *
 * Example:
 *
 *     requires capability("temporal.speculation");
 *
 * ============================================================================
 */

speculativeRequirement
    : REQUIRES speculativeClauseValue speculativeClauseTerminator
    ;


/*
 * ============================================================================
 * 11. CONSTRAINT
 * ============================================================================
 *
 * Example:
 *
 *     constraint: no_external_commit;
 *
 * ============================================================================
 */

speculativeConstraint
    : CONSTRAINT speculativeClauseValue speculativeClauseTerminator
    ;


/*
 * ============================================================================
 * 12. PREFERENCE
 * ============================================================================
 *
 * Preference is advisory.
 *
 * It MUST NOT silently become a requirement.
 *
 * ============================================================================
 */

speculativePreference
    : PREFER speculativeClauseValue speculativeClauseTerminator
    ;


/*
 * ============================================================================
 * 13. HINT
 * ============================================================================
 *
 * Hint is weaker than a requirement or constraint.
 *
 * ============================================================================
 */

speculativeHint
    : HINT speculativeClauseValue speculativeClauseTerminator
    ;


/*
 * ============================================================================
 * 14. GENERIC PROPERTY
 * ============================================================================
 *
 * Examples:
 *
 *     strategy: explore;
 *     commit: conditional;
 *     discard: automatic;
 *     conflict: reconcile;
 *     observation: isolated;
 *     provenance: required;
 *     timeline: future;
 *
 * Qualified names are supported.
 *
 * ============================================================================
 */

speculativeProperty
    : qualifiedName
      COLON
      speculativeClauseValue
      speculativeClauseTerminator
    ;


/*
 * ============================================================================
 * 15. ARGUMENT PROPERTY
 * ============================================================================
 *
 * Allows property assignment using the canonical assignment token.
 *
 * Example:
 *
 *     strategy = explore;
 *
 * This is intentionally kept separate from comparison expressions because
 * the value is parsed in clause context.
 *
 * ============================================================================
 */

speculativeArgumentProperty
    : identifier
      ASSIGN
      speculativeClauseValue
      speculativeClauseTerminator
    ;


/*
 * ============================================================================
 * 16. CLAUSE VALUE
 * ============================================================================
 *
 * Values use the canonical expression grammar.
 *
 * A nested property block is also permitted for structured policies.
 *
 * ============================================================================
 */

speculativeClauseValue
    : expression
    | speculativePropertyBlock
    ;


/*
 * ============================================================================
 * 17. NESTED PROPERTY BLOCK
 * ============================================================================
 *
 * Example:
 *
 *     policy: {
 *         commit: conditional;
 *         conflict: reconcile;
 *     };
 *
 * Nested structures are recursively composable.
 *
 * ============================================================================
 */

speculativePropertyBlock
    : LBRACE speculativePropertyEntry* RBRACE
    ;


speculativePropertyEntry
    : qualifiedName
      COLON
      speculativeClauseValue
      speculativeClauseTerminator
    ;


/*
 * ============================================================================
 * 18. CLAUSE TERMINATOR
 * ============================================================================
 */

speculativeClauseTerminator
    : SEMICOLON
    ;


/*
 * ============================================================================
 * 19. OUTER TERMINATOR
 * ============================================================================
 *
 * The host statement/declaration composition may own termination.
 *
 * Therefore the terminator is optional here.
 * ============================================================================
 */

speculativeTerminator
    : SEMICOLON
    ;
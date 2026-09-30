/*
 * Zamani — Execution / Rewind Grammar
 *
 * File:
 *   grammar/execution/rewind.g4
 *
 * Status:
 *   Production grammar component / execution-domain syntax
 *
 * Purpose
 * -------
 * Defines target-independent syntax for requesting a rewind of execution
 * state, logical time, an execution timeline, an observation point, a
 * checkpoint, or another semantically rewindable execution boundary.
 *
 * This grammar expresses INTENT.
 *
 * It does not implement:
 *   - state restoration
 *   - checkpoint storage
 *   - timeline reconstruction
 *   - speculative execution
 *   - branch creation
 *   - fork/merge
 *   - scheduling
 *   - placement
 *   - resource allocation
 *   - quantum error correction
 *   - ZQN fault/noise handling
 *   - hardware recovery
 *   - device control
 *
 * Those responsibilities remain downstream.
 *
 * Architectural boundary
 * ----------------------
 *
 *   source
 *      ↓
 *   lexer
 *      ↓
 *   parser / this grammar
 *      ↓
 *   frontend AST
 *      ↓
 *   semantic validation
 *      ↓
 *   canonical semantic execution model
 *      ↓
 *   canonical IR
 *      ↓
 *   checkpoint / recovery / scheduling / resilience
 *      ↓
 *   target realization
 *
 * Rewind MUST remain target-independent.
 *
 * No language-level maximum is encoded here for:
 *   - timelines
 *   - rewind depth
 *   - checkpoints
 *   - branches
 *   - observations
 *   - execution steps
 *   - timestamps
 *   - state size
 *   - qubits
 *   - classical memory
 *   - devices
 *   - nodes
 *   - processors
 *   - accelerators
 *
 * POCO-REAF
 * ---------
 *
 * A rewind request describes what execution state should be revisited
 * and under what semantic conditions. The compiler/runtime determines
 * whether and how that request can be realized on the selected target.
 *
 * Rust implementation requirements
 * --------------------------------
 *
 * The Rust implementation consuming this grammar targets Rust 1.97 /
 * Rust 1.97.1 and MUST use safe Rust only.
 *
 * This grammar itself contains no Rust implementation.
 *
 * Integration ownership
 * ---------------------
 *
 * Shared concepts are intentionally referenced by semantic names rather
 * than redefined here:
 *
 *   timeline       → execution/timelines.g4
 *   observation    → execution/observation.g4
 *   speculation    → execution/speculative.g4
 *   fork/merge     → execution/fork-merge.g4
 *   checkpoint     → execution/checkpointing.g4
 *   recovery       → execution/recovery.g4
 *   scheduling     → execution/scheduling.g4
 *   resources      → resources/
 *   capabilities   → hardware/ + resources/
 *
 * If the canonical root grammar exposes these shared rules under different
 * names, only the composition aliases in Zamani.g4 should change. This
 * grammar must not duplicate their definitions.
 *
 * AST contract
 * ------------
 *
 * Rewind syntax maps to a domain-neutral execution AST node conceptually
 * equivalent to:
 *
 *   Rewind {
 *       target,
 *       boundary,
 *       condition,
 *       mode,
 *       policy,
 *       consistency,
 *       scope,
 *       recovery,
 *       resource_requirements,
 *       capability_requirements,
 *       attributes,
 *       source_span
 *   }
 *
 * The exact Rust AST type belongs to src/frontend/ast/ and is not defined
 * by this grammar.
 *
 * Semantic contract
 * -----------------
 *
 * Semantic analysis MUST:
 *
 *   1. Resolve the rewind target.
 *   2. Verify that the target is rewindable under its declared semantics.
 *   3. Resolve temporal/boundary expressions.
 *   4. Validate conditions and policies.
 *   5. Check resource/capability requirements.
 *   6. Reject contradictory rewind policies.
 *   7. Preserve source spans.
 *   8. Preserve symbolic/unbounded values.
 *   9. Avoid converting semantic requirements into hardware constants.
 *  10. Produce a canonical semantic execution operation.
 *
 * IR contract
 * -----------
 *
 * The grammar MUST NOT introduce a frontend-specific rewind IR.
 *
 * Semantic lowering produces a canonical execution operation such as:
 *
 *   Execution::Rewind
 *
 * containing target/boundary/policy/condition information.
 *
 * Quantum programs remain integrated through the canonical quantum::ir
 * boundary. Rewind does not create a second quantum IR.
 *
 * Scalability contract
 * --------------------
 *
 * Rewind expressions may be:
 *
 *   concrete
 *   symbolic
 *   parameterized
 *   computed
 *   capability-dependent
 *   resource-dependent
 *   dynamically resolved
 *
 * No fixed integer width, maximum rewind distance, maximum number of
 * checkpoints, maximum number of branches, or maximum timeline count
 * belongs in this grammar.
 *
 * Determinism
 * -----------
 *
 * The syntax is deterministic. Semantic/runtime nondeterminism must be
 * represented explicitly through policy/condition semantics and must not
 * be introduced by parser ambiguity.
 *
 * Security
 * --------
 *
 * Rewind does not imply permission to mutate protected state. Authorization,
 * capability checks, provenance, isolation, and trust remain semantic/runtime
 * responsibilities.
 *
 * Compatibility
 * -------------
 *
 * Existing rewind syntax must retain its meaning unless explicitly
 * version-gated by the language compatibility system.
 *
 * New rewind modes should be additive and open-world where possible.
 *
 * Hard-coding audit
 * -----------------
 *
 * Forbidden universal grammar concepts include:
 *
 *   MAX_TIMELINES
 *   MAX_REWIND_DEPTH
 *   MAX_CHECKPOINTS
 *   MAX_BRANCHES
 *   MAX_STEPS
 *   MAX_TIMESTAMP
 *   MAX_STATE_SIZE
 *   MAX_NODES
 *   MAX_DEVICES
 *
 * Values such as 1000 or 1024 remain legal when they are programmer
 * data/requirements. They MUST NOT become grammar-level capacity limits.
 *
 * ------------------------------------------------------------------------
 */


/*
 * ------------------------------------------------------------------------
 * Grammar composition
 * ------------------------------------------------------------------------
 *
 * This is a parser grammar intended to be composed by the canonical
 * Zamani parser grammar.
 *
 * Shared lexical tokens/rules are expected to come from the canonical
 * Zamani lexer/token vocabulary.
 *
 * The root grammar remains responsible for deciding where a rewind
 * statement/expression is legal.
 *
 * ------------------------------------------------------------------------
 */

parser grammar Rewind;


/*
 * Rewind
 * ------
 *
 * Canonical entry point for the rewind construct.
 *
 * Examples of intended surface forms:
 *
 *   rewind to checkpoint cp
 *   rewind to observation obs
 *   rewind to timeline t
 *   rewind to time t
 *   rewind to step n
 *   rewind to before event e
 *   rewind to after event e
 *
 * Optional policy clauses may further constrain realization.
 *
 * The exact shared identifier/path/value rules are supplied by the
 * canonical Zamani grammar.
 */

rewindStatement
    : REWIND rewindTarget rewindClause*
    ;


/*
 * ------------------------------------------------------------------------
 * Target
 * ------------------------------------------------------------------------
 *
 * The target identifies the semantic execution boundary to revisit.
 *
 * Rewind MUST NOT require a physical machine address, physical qubit,
 * processor identifier, memory-bank identifier, GPU ordinal, FPGA slot,
 * or other target-specific location.
 */

rewindTarget
    : rewindCheckpointTarget
    | rewindObservationTarget
    | rewindTimelineTarget
    | rewindTimeTarget
    | rewindStepTarget
    | rewindEventTarget
    | rewindStateTarget
    | rewindBoundaryTarget
    ;


/*
 * Checkpoint
 * ----------
 *
 * Checkpoint ownership belongs to execution/checkpointing.g4.
 *
 * This rule only supplies the rewind-specific reference form.
 */

rewindCheckpointTarget
    : TO CHECKPOINT rewindReference
    ;


/*
 * Observation
 * -----------
 *
 * Observation ownership belongs to execution/observation.g4.
 */

rewindObservationTarget
    : TO OBSERVATION rewindReference
    ;


/*
 * Timeline
 * --------
 *
 * Timeline identity remains semantic.
 *
 * There is no maximum number of timelines.
 */

rewindTimelineTarget
    : TO TIMELINE rewindReference
    ;


/*
 * Logical time
 * ------------
 *
 * A time expression is semantic data.
 *
 * It may be concrete or symbolic.
 *
 * The implementation determines the representation appropriate to the
 * target and execution model.
 */

rewindTimeTarget
    : TO TIME rewindTemporalExpression
    ;


/*
 * Execution step
 * --------------
 *
 * A step expression is not constrained to a machine-sized integer.
 */

rewindStepTarget
    : TO STEP rewindScalarExpression
    ;


/*
 * Event
 * -----
 *
 * Events may originate from the execution/observation/speculation model.
 */

rewindEventTarget
    : TO EVENT rewindReference
    ;


/*
 * State
 * -----
 *
 * A named or otherwise semantically addressable execution state.
 *
 * This is not a physical memory address.
 */

rewindStateTarget
    : TO STATE rewindReference
    ;


/*
 * Generic execution boundary
 * --------------------------
 *
 * Allows future execution-boundary concepts without requiring a new
 * parser architecture.
 */

rewindBoundaryTarget
    : TO BOUNDARY rewindReference
    ;


/*
 * ------------------------------------------------------------------------
 * Clauses
 * ------------------------------------------------------------------------
 *
 * Clauses refine intent without embedding implementation details.
 */

rewindClause
    : rewindConditionClause
    | rewindModeClause
    | rewindPolicyClause
    | rewindConsistencyClause
    | rewindScopeClause
    | rewindRecoveryClause
    | rewindRequirementClause
    | rewindCapabilityClause
    | rewindAttributeClause
    ;


/*
 * Condition
 * ---------
 *
 * Rewind can be conditional.
 *
 * Example:
 *
 *   rewind to checkpoint cp if condition
 *
 * The condition itself uses the canonical Zamani expression system.
 */

rewindConditionClause
    : IF rewindExpression
    ;


/*
 * Mode
 * ----
 *
 * Mode describes the semantic nature of the rewind.
 *
 * The grammar intentionally uses a data-driven mode form rather than
 * hard-coding a closed enumeration of runtime strategies.
 *
 * Examples:
 *
 *   rewind to checkpoint cp mode exact
 *   rewind to checkpoint cp mode logical
 *   rewind to checkpoint cp mode semantic
 *
 * New modes can therefore be introduced through semantic capability
 * registration rather than requiring the grammar to enumerate hardware
 * implementations.
 */

rewindModeClause
    : MODE rewindMode
    ;


rewindMode
    : rewindIdentifier
    | rewindQualifiedName
    | rewindCall
    ;


/*
 * Policy
 * ------
 *
 * Policy is intentionally open-ended.
 *
 * Examples:
 *
 *   policy deterministic
 *   policy best_effort
 *   policy transactional
 *   policy resumable
 *
 * The semantic layer defines which policies exist and which combinations
 * are compatible.
 */

rewindPolicyClause
    : POLICY rewindPolicy
    ;


rewindPolicy
    : rewindIdentifier
    | rewindQualifiedName
    | rewindCall
    ;


/*
 * Consistency
 * -----------
 *
 * Rewind can require a consistency model.
 *
 * Examples:
 *
 *   consistency strong
 *   consistency causal
 *   consistency checkpoint
 *
 * Actual consistency semantics remain outside the grammar.
 */

rewindConsistencyClause
    : CONSISTENCY rewindConsistency
    ;


rewindConsistency
    : rewindIdentifier
    | rewindQualifiedName
    | rewindCall
    ;


/*
 * Scope
 * -----
 *
 * Scope identifies which semantic execution domain is affected.
 *
 * Examples:
 *
 *   scope local
 *   scope task
 *   scope timeline
 *   scope distributed
 *   scope quantum
 *   scope hybrid
 *
 * These are semantic classifications, not hardware selections.
 */

rewindScopeClause
    : SCOPE rewindScope
    ;


rewindScope
    : rewindIdentifier
    | rewindQualifiedName
    | rewindCall
    ;


/*
 * Recovery
 * --------
 *
 * Rewind and recovery are related but not identical.
 *
 * This clause allows the programmer to specify recovery intent without
 * moving recovery implementation into the grammar.
 */

rewindRecoveryClause
    : RECOVERY rewindRecovery
    ;


rewindRecovery
    : rewindIdentifier
    | rewindQualifiedName
    | rewindCall
    ;


/*
 * Resource requirement
 * --------------------
 *
 * Requirements describe resources necessary to satisfy the rewind.
 *
 * They do NOT establish compiler-wide limits.
 *
 * Examples:
 *
 *   requires memory >= required_memory
 *   requires checkpoint_storage >= required_storage
 *
 * Resource semantics belong to resources/ and hardware capability
 * analysis, not this grammar.
 */

rewindRequirementClause
    : REQUIRES rewindRequirement
    ;


rewindRequirement
    : rewindExpression
    ;


/*
 * Capability requirement
 * ----------------------
 *
 * Examples:
 *
 *   requires capability("execution.rewind")
 *   requires capability("checkpoint.restore")
 *   requires capability("timeline.reconstruction")
 *
 * The capability namespace is intentionally open.
 */

rewindCapabilityClause
    : REQUIRES CAPABILITY LPAREN rewindExpression RPAREN
    ;


/*
 * Attributes
 * ----------
 *
 * Attributes provide extensibility without repeatedly modifying the
 * core rewind grammar for metadata that has no structural language
 * significance.
 */

rewindAttributeClause
    : rewindAttribute
    ;


/*
 * ------------------------------------------------------------------------
 * Temporal expressions
 * ------------------------------------------------------------------------
 *
 * These are intentionally delegated to the general expression language
 * conceptually. This local rule provides a stable integration point.
 *
 * A temporal expression may be:
 *
 *   an identifier
 *   a qualified name
 *   a literal
 *   an arithmetic expression
 *   a function call
 *   a symbolic expression
 *   a value produced by another semantic operation
 */

rewindTemporalExpression
    : rewindExpression
    ;


/*
 * ------------------------------------------------------------------------
 * Generic expression bridge
 * ------------------------------------------------------------------------
 *
 * This rule is a deliberate integration boundary.
 *
 * The canonical Zamani expression grammar owns the full expression
 * language. Rewind does not recreate arithmetic, logical, call, member,
 * indexing, lambda, quantum, tensor, or other expression grammars.
 *
 * When integrated into Zamani.g4, this rule MUST be mapped to the
 * canonical expression entry rule.
 *
 * The placeholder form below exists to make this grammar contract
 * explicit without defining a competing expression language.
 */

rewindExpression
    : rewindLiteral
    | rewindReference
    | rewindQualifiedName
    | rewindCall
    ;


/*
 * ------------------------------------------------------------------------
 * References
 * ------------------------------------------------------------------------
 */

rewindReference
    : rewindIdentifier
    | rewindQualifiedName
    ;


/*
 * Identifier
 * ----------
 *
 * The canonical identifier rule is owned by core/identifiers.g4.
 *
 * This local spelling is an integration alias and MUST resolve to the
 * canonical Zamani identifier token/rule during grammar composition.
 */

rewindIdentifier
    : IDENTIFIER
    ;


/*
 * Qualified names
 * ---------------
 *
 * Examples:
 *
 *   timeline.main
 *   execution.checkpoint
 *   quantum.execution
 *   user.domain.boundary
 *
 * Namespace depth is not bounded.
 */

rewindQualifiedName
    : rewindIdentifier (DOT rewindIdentifier)*
    ;


/*
 * Calls
 * -----
 *
 * Generic semantic calls allow policy/mode/capability extensions without
 * requiring every new semantic function to become a grammar keyword.
 */

rewindCall
    : rewindQualifiedName LPAREN rewindArgumentList? RPAREN
    ;


rewindArgumentList
    : rewindExpression (COMMA rewindExpression)*
    ;


/*
 * Literals
 * --------
 *
 * Literal syntax is ultimately owned by lexer/literals.md and the
 * canonical expression grammar.
 *
 * This local rule provides only the minimal integration surface needed
 * for rewind expressions.
 */

rewindLiteral
    : INTEGER_LITERAL
    | FLOAT_LITERAL
    | STRING_LITERAL
    | BOOLEAN_LITERAL
    | NULL_LITERAL
    ;


/*
 * ------------------------------------------------------------------------
 * Attributes
 * ------------------------------------------------------------------------
 *
 * Attribute syntax is owned by core/attributes.g4.
 *
 * This local rule deliberately keeps metadata structurally generic.
 */

rewindAttribute
    : AT rewindQualifiedName
      (LPAREN rewindArgumentList? RPAREN)?
    ;


/*
 * ------------------------------------------------------------------------
 * Integration contract
 * ------------------------------------------------------------------------
 *
 * Zamani.g4 MUST integrate this grammar at the statement/execution
 * dispatch point.
 *
 * Conceptually:
 *
 *   statement
 *       : ...
 *       | rewindStatement
 *       | ...
 *       ;
 *
 * The root grammar remains the sole composition authority.
 *
 * Rewind MUST NOT become independently reachable in a way that bypasses
 * the canonical statement dispatcher.
 *
 * ------------------------------------------------------------------------
 */


/*
 * ------------------------------------------------------------------------
 * AST integration
 * ------------------------------------------------------------------------
 *
 * Parser output must map to the domain-neutral AST execution operation.
 *
 * Required semantic information:
 *
 *   target
 *   boundary
 *   condition
 *   mode
 *   policy
 *   consistency
 *   scope
 *   recovery
 *   requirements
 *   capabilities
 *   attributes
 *   source span
 *
 * No physical target information should be introduced at this layer.
 *
 * For quantum execution, rewind semantics must pass through the normal
 * frontend semantic path and ultimately integrate with canonical
 * quantum::ir where quantum state/execution semantics are involved.
 *
 * No `QuantumRewindIR` or equivalent frontend-specific secondary IR
 * should be created.
 *
 * ------------------------------------------------------------------------
 */


/*
 * ------------------------------------------------------------------------
 * Semantic validation
 * ------------------------------------------------------------------------
 *
 * The semantic layer must reject:
 *
 *   - unknown references when a reference requires resolution
 *   - impossible or contradictory policy combinations
 *   - invalid boundary kinds
 *   - unsupported rewind semantics for a selected execution model
 *   - missing required capabilities
 *   - unsatisfied resource requirements
 *   - invalid scope/target combinations
 *   - illegal cross-timeline operations
 *   - invalid speculative-state transitions
 *   - invalid fork/merge interactions
 *
 * It must NOT reject a program merely because a current physical machine
 * lacks enough resources if the program expresses a legitimate resource
 * requirement that can be satisfied by another valid target.
 *
 * Such conditions belong to capability/resource negotiation and target
 * selection.
 *
 * ------------------------------------------------------------------------
 */


/*
 * ------------------------------------------------------------------------
 * Timeline integration
 * ------------------------------------------------------------------------
 *
 * execution/timelines.g4 owns:
 *
 *   timeline declaration
 *   timeline identity
 *   temporal execution model
 *   timeline relationships
 *
 * rewind.g4 only consumes timeline references.
 *
 * It must not redefine timeline declaration syntax.
 *
 * ------------------------------------------------------------------------
 */


/*
 * ------------------------------------------------------------------------
 * Observation integration
 * ------------------------------------------------------------------------
 *
 * execution/observation.g4 owns:
 *
 *   observation points
 *   observation metadata
 *   observation semantics
 *
 * rewind.g4 may target an observation but must not redefine observation
 * semantics.
 *
 * ------------------------------------------------------------------------
 */


/*
 * ------------------------------------------------------------------------
 * Speculation integration
 * ------------------------------------------------------------------------
 *
 * execution/speculative.g4 owns speculative execution constructs.
 *
 * Rewind may be used to return to a speculative boundary, but the
 * speculative lifecycle remains owned by that grammar/semantic domain.
 *
 * ------------------------------------------------------------------------
 */


/*
 * ------------------------------------------------------------------------
 * Fork / merge integration
 * ------------------------------------------------------------------------
 *
 * execution/fork-merge.g4 owns branch/fork/merge semantics.
 *
 * Rewind may target a branch boundary, but rewind MUST NOT implicitly
 * create, destroy, merge, or select branches unless explicitly expressed
 * by the corresponding execution semantics.
 *
 * ------------------------------------------------------------------------
 */


/*
 * ------------------------------------------------------------------------
 * Checkpoint integration
 * ------------------------------------------------------------------------
 *
 * execution/checkpointing.g4 owns:
 *
 *   checkpoint declaration
 *   checkpoint capture
 *   checkpoint metadata
 *   checkpoint lifecycle
 *
 * rewind only references checkpoints.
 *
 * A rewind to a checkpoint means "restore/reconstruct execution state
 * corresponding to this semantic checkpoint"; it does not prescribe the
 * storage mechanism.
 *
 * ------------------------------------------------------------------------
 */


/*
 * ------------------------------------------------------------------------
 * Recovery integration
 * ------------------------------------------------------------------------
 *
 * execution/recovery.g4 owns recovery strategies and lifecycle.
 *
 * Rewind may request a recovery policy, but recovery execution remains
 * downstream.
 *
 * ------------------------------------------------------------------------
 */


/*
 * ------------------------------------------------------------------------
 * Scheduling integration
 * ------------------------------------------------------------------------
 *
 * Scheduling determines when and where a rewind operation can execute.
 *
 * Rewind grammar must not encode:
 *
 *   core number
 *   GPU ordinal
 *   QPU index
 *   FPGA region
 *   node number
 *   memory bank
 *   fixed execution queue
 *
 * These are target realization concerns.
 *
 * ------------------------------------------------------------------------
 */


/*
 * ------------------------------------------------------------------------
 * Resource / capability integration
 * ------------------------------------------------------------------------
 *
 * Resource requirements are semantic constraints.
 *
 * Capability requirements describe required abilities.
 *
 * Neither creates universal compiler limits.
 *
 * Valid conceptual examples:
 *
 *   requires capability("execution.rewind")
 *   requires capability("checkpoint.restore")
 *   requires capability("timeline.reconstruction")
 *   requires memory >= required_memory
 *
 * Invalid architectural concepts:
 *
 *   MAX_CHECKPOINTS
 *   MAX_TIMELINES
 *   MAX_REWIND_DEPTH
 *
 * ------------------------------------------------------------------------
 */


/*
 * ------------------------------------------------------------------------
 * Quantum integration
 * ------------------------------------------------------------------------
 *
 * Rewind can appear around quantum execution, but this grammar does not
 * define quantum state semantics.
 *
 * Quantum state, logical state, measurement, reset, noise, error
 * correction, routing, scheduling, and physical realization remain
 * downstream concerns.
 *
 * The canonical boundary remains:
 *
 *   Zamani frontend
 *       ↓
 *   canonical semantic model
 *       ↓
 *   quantum::ir
 *       ↓
 *   optimization / routing / scheduling / resilience / QEC / ZQN
 *       ↓
 *   HAL / target
 *
 * Rewind MUST NOT introduce a competing quantum frontend IR.
 *
 * ------------------------------------------------------------------------
 */


/*
 * ------------------------------------------------------------------------
 * HDL / hardware integration
 * ------------------------------------------------------------------------
 *
 * Rewind may describe execution semantics for software/hardware
 * co-designed systems.
 *
 * It must not encode fixed:
 *
 *   register widths
 *   memory capacities
 *   pipeline depths
 *   FPGA resources
 *   ASIC structures
 *   accelerator counts
 *
 * Hardware realization remains downstream.
 *
 * ------------------------------------------------------------------------
 */


/*
 * ------------------------------------------------------------------------
 * Distributed integration
 * ------------------------------------------------------------------------
 *
 * A distributed rewind may require coordination, replication, consistency,
 * provenance, or recovery.
 *
 * Those semantics belong to distributed/, resources/, networking/, and
 * execution/recovery.g4.
 *
 * The grammar does not impose a fixed number of nodes or participants.
 *
 * ------------------------------------------------------------------------
 */


/*
 * ------------------------------------------------------------------------
 * Error handling contract
 * ------------------------------------------------------------------------
 *
 * Parser diagnostics should identify:
 *
 *   - unexpected rewind target
 *   - malformed target
 *   - malformed temporal expression
 *   - malformed clause
 *   - malformed capability expression
 *   - malformed policy/mode
 *
 * Semantic diagnostics should identify:
 *
 *   - unresolved target
 *   - unsupported rewind capability
 *   - unsatisfied resource requirement
 *   - incompatible policy
 *   - invalid temporal boundary
 *   - invalid speculative boundary
 *   - invalid fork/merge interaction
 *   - invalid state transition
 *
 * Diagnostics must preserve source spans.
 *
 * ------------------------------------------------------------------------
 */


/*
 * ------------------------------------------------------------------------
 * Determinism and ambiguity
 * ------------------------------------------------------------------------
 *
 * The parser must not use semantic lookahead to decide between unrelated
 * language constructs.
 *
 * The canonical expression grammar should own expression precedence.
 *
 * The root Zamani grammar should own statement-level disambiguation.
 *
 * ------------------------------------------------------------------------
 */


/*
 * ------------------------------------------------------------------------
 * Compatibility
 * ------------------------------------------------------------------------
 *
 * Rewind syntax is versioned through the canonical compatibility system.
 *
 * New clauses should normally be additive.
 *
 * Removing or changing the meaning of an existing rewind form requires:
 *
 *   specification update
 *   compatibility entry
 *   migration rule where applicable
 *   positive/negative conformance tests
 *
 * ------------------------------------------------------------------------
 */


/*
 * ------------------------------------------------------------------------
 * Production completion checklist
 * ------------------------------------------------------------------------
 *
 * This grammar component is considered complete only when all of the
 * following are true:
 *
 * [ ] Canonical rewind statement is defined.
 * [ ] Target-independent target forms are defined.
 * [ ] Checkpoint integration is defined.
 * [ ] Observation integration is defined.
 * [ ] Timeline integration is defined.
 * [ ] Speculation integration is defined.
 * [ ] Fork/merge integration is defined.
 * [ ] Recovery integration is defined.
 * [ ] Scheduling integration is defined.
 * [ ] Resource integration is defined.
 * [ ] Capability integration is defined.
 * [ ] AST mapping is documented.
 * [ ] Semantic mapping is documented.
 * [ ] Canonical IR mapping is documented.
 * [ ] Quantum integration is documented.
 * [ ] HDL/hardware integration is documented.
 * [ ] Distributed integration is documented.
 * [ ] Source-span requirements are documented.
 * [ ] Diagnostics are documented.
 * [ ] Compatibility is documented.
 * [ ] Determinism requirements are documented.
 * [ ] No fixed capacity is encoded.
 * [ ] No hardware topology is encoded.
 * [ ] No vendor-specific implementation is encoded.
 * [ ] No duplicate quantum IR is introduced.
 * [ ] Rust implementation can remain safe Rust.
 * [ ] Positive tests exist.
 * [ ] Negative tests exist.
 * [ ] Boundary tests exist.
 * [ ] Scalability tests exist.
 * [ ] Determinism tests exist.
 * [ ] Compatibility tests exist.
 *
 * ------------------------------------------------------------------------
 */
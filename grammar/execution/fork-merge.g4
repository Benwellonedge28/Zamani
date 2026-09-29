/*
 * =============================================================================
 * Zamani — Fork / Merge Execution Grammar
 * =============================================================================
 *
 * File:
 *   grammar/execution/fork-merge.g4
 *
 * Status:
 *   Production grammar component.
 *
 * Purpose:
 *   Defines the language-level syntax needed to create, identify, compose,
 *   reconcile, and terminate alternative execution histories.
 *
 * Architectural position:
 *
 *   Zamani source
 *       -> lexer
 *       -> ANTLR parser
 *       -> domain-neutral frontend AST
 *       -> semantic analysis
 *       -> canonical semantic model
 *       -> canonical IR
 *       -> optimization / scheduling / routing / resilience
 *       -> runtime execution
 *
 * This file owns:
 *   - fork-specific syntax
 *   - merge-specific syntax
 *   - fork/merge policies
 *   - fork/merge references
 *   - merge conflict policy syntax
 *   - merge result selection syntax
 *   - fork/merge lifecycle syntax
 *
 * This file does NOT own:
 *   - generic MTS declarations
 *   - generic timeline declarations
 *   - generic timeline operations
 *   - generic blocks
 *   - expressions
 *   - statements
 *   - identifiers
 *   - lexical tokens
 *   - runtime timeline state
 *   - scheduling
 *   - placement
 *   - hardware topology
 *   - quantum routing
 *   - QEC
 *   - ZQN
 *   - HAL/device realization
 *
 * Integration authority:
 *   execution/timelines.g4 remains the generic MTS composition surface.
 *   This grammar is imported by the MTS execution grammar and delegated to
 *   for fork/merge-specific syntax.
 *
 * Scalability:
 *   No fixed number of forks, branches, timelines, states, participants,
 *   resources, timestamps, qubits, CPUs, GPUs, FPGAs, nodes, memories,
 *   registers, tensor dimensions, or devices is encoded here.
 *
 * POCO-REAF:
 *   Fork/merge describes execution semantics and intent.
 *   Physical realization remains a compiler/runtime concern.
 *
 * Rust:
 *   This grammar contains no Rust implementation code and requires no unsafe
 *   code. The corresponding Zamani implementation must remain compatible with
 *   Rust 1.97 / Rust 1.97.1 and must not use `unsafe`.
 *
 * =============================================================================
 */

parser grammar ForkMergeExecution;

options {
    tokenVocab = ZamaniTokens;
}

/*
 * Reuse the repository's existing common grammar contracts.
 *
 * CoreIdentifiers:
 *   identifier, qualified names, and related naming primitives.
 *
 * CoreAttributes:
 *   attributes used to carry extensible policy/metadata without adding a new
 *   reserved keyword for every future execution capability.
 *
 * CoreModifiers:
 *   common language modifiers.
 */
import CoreAttributes, CoreModifiers, CoreIdentifiers;


/*
 * =============================================================================
 * PUBLIC ENTRY POINT
 * =============================================================================
 *
 * This is the only public dispatch rule this file contributes to the MTS
 * grammar.
 *
 * `timelines.g4` should delegate fork/merge-specific constructs here instead
 * of duplicating their syntax.
 */
forkMergeConstruct
    : forkConstruct
    | mergeConstruct
    ;


/*
 * =============================================================================
 * FORK
 * =============================================================================
 *
 * A fork creates an alternative execution history from an existing execution
 * point.
 *
 * The grammar deliberately does not prescribe:
 *
 *   - how many child timelines are created;
 *   - how much memory a fork consumes;
 *   - how many branches may exist;
 *   - which CPU/GPU/QPU executes a branch;
 *   - how branch identifiers are physically represented;
 *   - how snapshots are stored.
 *
 * Those are semantic/runtime responsibilities.
 *
 * Supported conceptual forms include:
 *
 *   fork { ... }
 *
 *   fork name { ... }
 *
 *   fork(name) { ... }
 *
 *   fork(name, expression) { ... }
 *
 * Additional policies can be supplied through the existing attribute system.
 */
forkConstruct
    : FORK forkAttributes* forkTarget? forkArguments? block
    ;


/*
 * Optional symbolic target of a fork.
 *
 * The target is intentionally an identifier/qualified name rather than a
 * physical timeline number or hardware identifier.
 */
forkTarget
    : identifier
    | qualifiedName
    ;


/*
 * Optional fork arguments.
 *
 * Arguments are expressions because the fork point may depend on program
 * state, configuration, symbolic execution state, or other semantic values.
 */
forkArguments
    : LPAREN expressionList? RPAREN
    ;


/*
 * Fork-specific metadata/policy.
 *
 * Attributes remain extensible and avoid making every future execution policy
 * a new lexer keyword.
 */
forkAttributes
    : attribute
    ;


/*
 * =============================================================================
 * MERGE
 * =============================================================================
 *
 * A merge reconciles one or more alternative execution histories.
 *
 * Merge is intentionally separate from fork:
 *
 *   fork  = create alternative histories
 *   merge = reconcile histories
 *
 * The grammar does not decide whether merge is:
 *
 *   - automatic;
 *   - deterministic;
 *   - conflict-free;
 *   - transactional;
 *   - speculative;
 *   - state-based;
 *   - event-based;
 *   - checkpoint-based;
 *   - application-defined.
 *
 * Those are semantic policies.
 */
mergeConstruct
    : MERGE mergeAttributes* mergeSources? mergeTarget? mergeArguments? mergeBody
    ;


/*
 * Sources being merged.
 *
 * A source can be represented by a symbolic identifier, qualified name, or
 * expression whose semantic value identifies an execution history.
 *
 * There is deliberately no numeric branch/timeline limit.
 */
mergeSources
    : LBRACKET mergeSourceList? RBRACKET
    ;

mergeSourceList
    : mergeSource (COMMA mergeSource)*
    ;

mergeSource
    : identifier
    | qualifiedName
    | expression
    ;


/*
 * Optional destination of a merge.
 *
 * The destination remains symbolic. Mapping it to a physical execution
 * context belongs to semantic analysis/runtime placement.
 */
mergeTarget
    : identifier
    | qualifiedName
    ;


/*
 * Optional merge arguments.
 */
mergeArguments
    : LPAREN expressionList? RPAREN
    ;


/*
 * =============================================================================
 * MERGE BODY
 * =============================================================================
 *
 * A merge may have an explicit block containing merge policy declarations,
 * conflict handling, result selection, or application-defined reconciliation.
 *
 * A semicolon form is allowed for a policy-only merge.
 */
mergeBody
    : block
    | SEMI
    ;


/*
 * =============================================================================
 * MERGE ATTRIBUTES
 * =============================================================================
 *
 * Examples of semantic policies that may be represented through attributes:
 *
 *   @deterministic
 *   @conflict(...)
 *   @prefer(...)
 *   @combine(...)
 *   @validate(...)
 *   @require(...)
 *   @rollback(...)
 *
 * The grammar does not hard-code those policy names.
 */
mergeAttributes
    : attribute
    ;


/*
 * =============================================================================
 * EXPLICIT FORK/MERGE POLICY FORMS
 * =============================================================================
 *
 * These rules provide reusable structure for implementations that want to
 * expose policy clauses without creating a new keyword for every policy.
 *
 * They are deliberately based on existing expressions and identifiers.
 */


/*
 * A symbolic policy name.
 */
forkMergePolicyName
    : identifier
    | qualifiedName
    ;


/*
 * Generic policy value.
 */
forkMergePolicyValue
    : expression
    | identifier
    | qualifiedName
    ;


/*
 * Generic policy invocation.
 *
 * This is intentionally data-driven:
 *
 *   policy(...)
 *
 * rather than:
 *
 *   policyA | policyB | policyC | ...
 *
 * This keeps the language extensible.
 */
forkMergePolicy
    : forkMergePolicyName
      (LPAREN expressionList? RPAREN)?
    ;


/*
 * =============================================================================
 * CONFLICT REPRESENTATION
 * =============================================================================
 *
 * A merge can encounter semantic conflicts. The grammar permits an explicit
 * conflict policy without determining the actual conflict-resolution
 * algorithm.
 */
mergeConflictPolicy
    : forkMergePolicy
    ;


/*
 * =============================================================================
 * RESULT POLICY
 * =============================================================================
 *
 * A merge can expose a symbolic result policy.
 *
 * Examples of semantic policies:
 *
 *   preserve(...)
 *   combine(...)
 *   select(...)
 *   validate(...)
 *   reject(...)
 *
 * Again, the names are semantic data, not a fixed enumeration.
 */
mergeResultPolicy
    : forkMergePolicy
    ;


/*
 * =============================================================================
 * VALIDATION POLICY
 * =============================================================================
 *
 * Validation can be required before a merged state becomes observable.
 */
mergeValidationPolicy
    : forkMergePolicy
    ;


/*
 * =============================================================================
 * REUSABLE POLICY LIST
 * =============================================================================
 */
forkMergePolicyList
    : forkMergePolicy (COMMA forkMergePolicy)*
    ;


/*
 * =============================================================================
 * EXPLICIT MERGE POLICY BLOCK ITEMS
 * =============================================================================
 *
 * These rules are intentionally small. They allow the semantic analyzer to
 * recognize structured policy intent while leaving implementation decisions
 * downstream.
 */
mergePolicyItem
    : mergeConflictPolicy
    | mergeResultPolicy
    | mergeValidationPolicy
    | statement
    | expressionStatement
    ;


/*
 * A specialized policy block can be used by semantic lowering without
 * introducing a second block grammar.
 */
mergePolicyBlock
    : LBRACE mergePolicyItem* RBRACE
    ;


/*
 * =============================================================================
 * FORK/MERGE REFERENCE
 * =============================================================================
 *
 * A reference identifies a symbolic execution history.
 *
 * It is deliberately not a physical timeline index.
 */
forkMergeReference
    : identifier
    | qualifiedName
    ;


/*
 * Multiple references.
 *
 * Open-world: cardinality is determined by source/program resources.
 */
forkMergeReferenceList
    : forkMergeReference (COMMA forkMergeReference)*
    ;


/*
 * =============================================================================
 * OPTIONAL RELATION EXPRESSION
 * =============================================================================
 *
 * This provides a common reusable semantic relation without hard-coding
 * relation names into the grammar.
 */
forkMergeRelation
    : forkMergePolicyName
      (LPAREN forkMergePolicyValue (COMMA forkMergePolicyValue)* RPAREN)?
    ;


/*
 * =============================================================================
 * FORK/MERGE POLICY CLAUSE
 * =============================================================================
 *
 * Generic clause form intended for semantic extension through existing
 * identifiers/attributes rather than an ever-growing keyword vocabulary.
 */
forkMergeClause
    : forkMergePolicyName
      (COLON forkMergePolicyValue)?
    ;


/*
 * =============================================================================
 * EXTENSIBLE POLICY CLAUSE LIST
 * =============================================================================
 */
forkMergeClauseList
    : forkMergeClause (COMMA forkMergeClause)*
    ;


/*
 * =============================================================================
 * SEMANTIC CONTRACT
 * =============================================================================
 *
 * The following comments are part of the file's completion contract.
 *
 * ---------------------------------------------------------------------------
 * AST CONTRACT
 * ---------------------------------------------------------------------------
 *
 * `forkConstruct` must lower to a domain-neutral execution AST node carrying:
 *
 *   - source span;
 *   - symbolic fork target, when present;
 *   - fork arguments;
 *   - attributes/modifiers;
 *   - nested body;
 *   - source-order information.
 *
 * `mergeConstruct` must lower to a domain-neutral execution AST node carrying:
 *
 *   - source span;
 *   - symbolic merge sources;
 *   - symbolic merge target, when present;
 *   - merge arguments;
 *   - attributes/modifiers;
 *   - merge body;
 *   - source-order information.
 *
 * The AST must NOT contain:
 *
 *   - physical CPU identifiers;
 *   - physical GPU identifiers;
 *   - physical QPU identifiers;
 *   - fixed branch counts;
 *   - fixed timeline counts;
 *   - hardware register limits;
 *   - hardware memory limits.
 *
 *
 * ---------------------------------------------------------------------------
 * SEMANTIC CONTRACT
 * ---------------------------------------------------------------------------
 *
 * Semantic analysis must establish:
 *
 *   1. fork source/context validity;
 *   2. fork target identity;
 *   3. scope/lifetime rules;
 *   4. branch identity;
 *   5. dependency relationships;
 *   6. resource requirements;
 *   7. capability requirements;
 *   8. effect restrictions;
 *   9. determinism requirements;
 *  10. external side-effect rules;
 *  11. merge source compatibility;
 *  12. merge target validity;
 *  13. conflict detection;
 *  14. conflict-resolution policy;
 *  15. result validity;
 *  16. observation/visibility rules;
 *  17. cancellation/abandonment semantics;
 *  18. checkpoint/state provenance;
 *  19. security/capability authorization;
 *  20. compatibility/version rules.
 *
 * Semantic analysis, rather than this grammar, determines whether a particular
 * fork or merge is legal.
 *
 *
 * ---------------------------------------------------------------------------
 * CANONICAL IR CONTRACT
 * ---------------------------------------------------------------------------
 *
 * Fork/merge constructs must lower into the existing canonical execution
 * semantic representation.
 *
 * The IR representation should preserve:
 *
 *   - execution-history identity;
 *   - parent/child relationships;
 *   - dependency edges;
 *   - state/provenance information;
 *   - merge inputs;
 *   - merge destination;
 *   - conflict policy;
 *   - validation policy;
 *   - effect information;
 *   - resource/capability requirements;
 *   - determinism requirements;
 *   - source spans/diagnostic provenance.
 *
 * Do NOT introduce a second frontend-only MTS IR.
 *
 * Quantum programs remain lowered through the canonical `quantum::ir`
 * boundary. Fork/merge syntax must not create a competing quantum IR.
 *
 *
 * ---------------------------------------------------------------------------
 * EXECUTION CONTRACT
 * ---------------------------------------------------------------------------
 *
 * Runtime responsibilities include:
 *
 *   - creating execution contexts;
 *   - materializing state;
 *   - sharing state where legal;
 *   - isolating state where required;
 *   - tracking provenance;
 *   - evaluating branch completion;
 *   - detecting conflicts;
 *   - executing merge policy;
 *   - preserving failure/recovery semantics;
 *   - enforcing capability authorization;
 *   - coordinating resources.
 *
 * None of those responsibilities belong in the parser.
 *
 *
 * ---------------------------------------------------------------------------
 * SIDE-EFFECT CONTRACT
 * ---------------------------------------------------------------------------
 *
 * Fork/merge is not automatically safe for arbitrary external side effects.
 *
 * Semantic analysis must classify effects such as:
 *
 *   - pure computation;
 *   - local mutable state;
 *   - shared state;
 *   - durable storage;
 *   - network communication;
 *   - device interaction;
 *   - quantum measurement;
 *   - hardware control;
 *   - external services.
 *
 * Non-reversible effects must not silently acquire reversible fork/merge
 * semantics merely because they occur inside a syntactic fork block.
 *
 *
 * ---------------------------------------------------------------------------
 * QUANTUM CONTRACT
 * ---------------------------------------------------------------------------
 *
 * Fork/merge may surround or contain quantum computation, but this grammar
 * does not define quantum semantics.
 *
 * Quantum-specific semantics remain owned by:
 *
 *   grammar/quantum/
 *   semantic quantum analysis
 *   canonical quantum::ir
 *   optimization
 *   routing
 *   scheduling
 *   QEC
 *   ZQN
 *   HAL/backend
 *
 * In particular, fork/merge must not:
 *
 *   - enumerate quantum gates;
 *   - impose qubit limits;
 *   - impose register-width limits;
 *   - impose measurement limits;
 *   - choose physical qubits.
 *
 *
 * ---------------------------------------------------------------------------
 * HARD-CODING CONTRACT
 * ---------------------------------------------------------------------------
 *
 * This grammar must never introduce universal constants such as:
 *
 *   MAX_QUBITS
 *   MAX_CPUS
 *   MAX_GPUS
 *   MAX_FPGAS
 *   MAX_NODES
 *   MAX_MEMORY
 *   MAX_THREADS
 *   MAX_TENSOR_RANK
 *   MAX_REGISTER_WIDTH
 *   MAX_NETWORK_SIZE
 *   MAX_DEVICE_COUNT
 *
 * Nor may it encode:
 *
 *   timeline 0
 *   timeline 1
 *   branch 0
 *   branch 1
 *   physical device 0
 *
 * as universal language limits.
 *
 *
 * ---------------------------------------------------------------------------
 * RESOURCE CONTRACT
 * ---------------------------------------------------------------------------
 *
 * Resource requirements belong to semantic/resource contracts.
 *
 * Valid concepts include:
 *
 *   requires capability("...")
 *   requires qubits >= n
 *   requires memory >= required_memory
 *   requires topology(...)
 *
 * Fork/merge must remain independent of how those requirements are satisfied.
 *
 *
 * ---------------------------------------------------------------------------
 * POCO-REAF CONTRACT
 * ---------------------------------------------------------------------------
 *
 * The same fork/merge source should remain semantically meaningful when the
 * compiler chooses:
 *
 *   - one CPU;
 *   - many CPU cores;
 *   - GPU execution;
 *   - FPGA execution;
 *   - ASIC execution;
 *   - QPU execution;
 *   - quantum simulation;
 *   - accelerator execution;
 *   - HPC execution;
 *   - cluster execution;
 *   - distributed/cloud execution;
 *   - future execution targets.
 *
 * Physical realization is downstream.
 *
 *
 * ---------------------------------------------------------------------------
 * DETERMINISM CONTRACT
 * ---------------------------------------------------------------------------
 *
 * The parser must be deterministic.
 *
 * Semantic analysis must distinguish:
 *
 *   deterministic fork/merge;
 *   nondeterministic execution;
 *   explicitly speculative execution;
 *   unresolved conflict;
 *   implementation-defined scheduling.
 *
 * A merge must not silently convert nondeterministic state into deterministic
 * program semantics.
 *
 *
 * ---------------------------------------------------------------------------
 * SOURCE-SPAN CONTRACT
 * ---------------------------------------------------------------------------
 *
 * Every public construct and all policy-bearing subconstructs must retain
 * enough source information for diagnostics.
 *
 * At minimum diagnostics must be able to identify:
 *
 *   - fork keyword;
 *   - fork target;
 *   - fork arguments;
 *   - fork body;
 *   - merge keyword;
 *   - merge sources;
 *   - merge target;
 *   - merge arguments;
 *   - merge body;
 *   - conflicting policy clauses.
 *
 *
 * ---------------------------------------------------------------------------
 * ERROR CONTRACT
 * ---------------------------------------------------------------------------
 *
 * The parser should reject malformed syntax such as:
 *
 *   fork
 *   merge
 *   fork(
 *   merge(
 *   fork(name
 *   merge([a,
 *
 * while semantic analysis should diagnose cases such as:
 *
 *   - unknown fork reference;
 *   - duplicate identity;
 *   - invalid merge source;
 *   - incompatible histories;
 *   - unresolved merge conflict;
 *   - illegal side-effect reconciliation;
 *   - unauthorized resource/capability use;
 *   - invalid lifecycle transition.
 *
 *
 * ---------------------------------------------------------------------------
 * SCALABILITY CONTRACT
 * ---------------------------------------------------------------------------
 *
 * Complexity must be proportional to the actual source/program structure and
 * available resources rather than a grammar-imposed universal capacity.
 *
 * The grammar contains:
 *
 *   no fixed branch count;
 *   no fixed merge-source count;
 *   no fixed nesting depth;
 *   no fixed timeline count;
 *   no fixed timestamp count;
 *   no fixed resource count.
 *
 *
 * ---------------------------------------------------------------------------
 * COMPATIBILITY CONTRACT
 * ---------------------------------------------------------------------------
 *
 * Adding a new fork/merge policy should preferentially use:
 *
 *   existing attributes;
 *   existing identifiers;
 *   existing expressions;
 *   existing qualified names;
 *
 * rather than introducing a new reserved keyword.
 *
 * This preserves source compatibility and prevents keyword proliferation.
 *
 *
 * ---------------------------------------------------------------------------
 * TEST CONTRACT
 * ---------------------------------------------------------------------------
 *
 * Required acceptance coverage:
 *
 *   tests/execution/fork-merge/
 *
 * Positive:
 *
 *   fork { ... }
 *   fork name { ... }
 *   fork(name) { ... }
 *   fork(name, expression) { ... }
 *   merge;
 *   merge { ... }
 *   merge(a, b);
 *   merge([a, b]) { ... }
 *   merge([a, b], target) { ... }
 *   fork/merge with attributes;
 *   nested fork/merge;
 *   fork containing classical computation;
 *   fork containing quantum computation;
 *   fork containing HDL intent;
 *   fork containing distributed computation;
 *
 * Negative:
 *
 *   malformed fork;
 *   malformed merge;
 *   missing body;
 *   malformed argument list;
 *   malformed source list;
 *   invalid delimiter placement;
 *
 * Semantic-negative:
 *
 *   unknown history;
 *   incompatible histories;
 *   illegal merge;
 *   conflicting side effects;
 *   unauthorized capability;
 *   unresolved conflict;
 *
 * Boundary:
 *
 *   one fork;
 *   one merge source;
 *   many merge sources;
 *   deeply nested valid structures;
 *   empty policy lists;
 *   symbolic expressions;
 *   very large source lists subject only to available resources.
 *
 * Scalability:
 *
 *   no test may establish a language maximum.
 *
 * Determinism:
 *
 *   equivalent source forms must produce deterministic parse trees.
 *
 * Compatibility:
 *
 *   existing MTS syntax must remain accepted unless deliberately deprecated
 *   through the repository compatibility process.
 *
 * =============================================================================
 */
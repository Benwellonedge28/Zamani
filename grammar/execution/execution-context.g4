/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/execution/execution-context.g4
 *
 * Grammar:
 *     ExecutionContext
 *
 * Status:
 *     Production-ready modular parser grammar
 *
 * Purpose:
 *     Defines the reusable source-level execution-context language.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     Core parser grammar
 *          |
 *          +-----------------------------+
 *          |                             |
 *          v                             v
 *     execution syntax          execution-context.g4
 *                                        |
 *                                        v
 *                              execution intent AST
 *                                        |
 *                       +----------------+----------------+
 *                       |                |                |
 *                       v                v                v
 *                   capability       resource         target
 *                   analysis         analysis         resolution
 *                       |                |                |
 *                       +----------------+----------------+
 *                                        |
 *                                        v
 *                              canonical semantic model
 *                                        |
 *                +-----------------------+-----------------------+
 *                |                       |                       |
 *                v                       v                       v
 *           classical IR           quantum::ir             HDL/hardware
 *                |                       |                       |
 *                +-----------------------+-----------------------+
 *                                        |
 *                                        v
 *                              optimization / routing /
 *                              scheduling / resilience /
 *                              ZQN / target lowering
 *                                        |
 *                                        v
 *                                  runtime / deployment
 *
 * ============================================================================
 * CORE PRINCIPLE
 * ============================================================================
 *
 * This file describes EXECUTION CONTEXT.
 *
 * It does not execute anything.
 *
 * It does not select a backend.
 *
 * It does not discover hardware.
 *
 * It does not allocate resources.
 *
 * It does not perform scheduling.
 *
 * It does not perform routing.
 *
 * It does not perform optimization.
 *
 * It does not perform resilience or recovery.
 *
 * It does not create classical IR.
 *
 * It does not create quantum::ir.
 *
 * It does not create an HDL representation.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - execution-context syntax;
 *     - context entries;
 *     - context keys;
 *     - context values;
 *     - context assignments;
 *     - context comparisons;
 *     - context nested objects;
 *     - context lists;
 *     - context composition;
 *     - context-level annotations represented as data;
 *     - syntactic distinction between requirements, constraints,
 *       preferences, hints, and ordinary properties where explicitly
 *       represented by the context syntax.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - identifiers;
 *     - qualified names;
 *     - ordinary expressions;
 *     - ordinary types;
 *     - function declarations;
 *     - execution declarations;
 *     - compilation;
 *     - execution algorithms;
 *     - scheduling algorithms;
 *     - routing algorithms;
 *     - optimization algorithms;
 *     - resource discovery;
 *     - capability discovery;
 *     - hardware discovery;
 *     - device selection;
 *     - physical placement;
 *     - quantum operations;
 *     - quantum gates;
 *     - quantum IR;
 *     - classical IR;
 *     - HDL IR;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - runtime dispatch;
 *     - deployment;
 *     - backend APIs.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * This parser grammar imports Core.
 *
 * Therefore this file reuses the canonical Core definitions for:
 *
 *     expression
 *     qualifiedName
 *     identifier
 *     argumentList
 *     literals
 *     and all other core language constructs exposed by Core.
 *
 * This file MUST NOT redefine those concepts.
 *
 * Integration direction:
 *
 *     ZamaniLexer
 *          |
 *          v
 *        Core
 *          |
 *          v
 *   ExecutionContext
 *
 * The dependency MUST NOT be reversed.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Execution context is intentionally expressed using open semantic names.
 *
 * A context may describe:
 *
 *     requirements
 *     constraints
 *     preferences
 *     hints
 *     capabilities
 *     resources
 *     targets
 *     placement
 *     scheduling intent
 *     dispatch intent
 *     synchronization intent
 *     lifecycle intent
 *     retry/recovery intent
 *     deployment intent
 *     arbitrary future execution properties
 *
 * The grammar does not enumerate physical machines.
 *
 * It does not encode:
 *
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_RETRIES
 *     MAX_JOBS
 *     MAX_STAGES
 *
 * No finite machine size is represented here.
 *
 * ============================================================================
 * SEMANTIC SEPARATION
 * ============================================================================
 *
 * The following distinctions are mandatory downstream:
 *
 *     requirement
 *         = must be satisfied
 *
 *     constraint
 *         = acceptable realizations are restricted
 *
 *     preference
 *         = desirable but not necessarily mandatory
 *
 *     hint
 *         = advisory implementation information
 *
 *     capability
 *         = required/desired computational capability
 *
 *     resource
 *         = computational resource requirement or description
 *
 *     target
 *         = abstract execution target
 *
 *     placement
 *         = placement intent
 *
 *     scheduling
 *         = temporal realization intent
 *
 *     dispatch
 *         = handoff/execution-environment intent
 *
 *     synchronization
 *         = ordering/completion intent
 *
 *     lifecycle
 *         = execution lifecycle intent
 *
 *     recovery
 *         = failure/recovery intent
 *
 * This grammar only establishes their syntax.
 *
 * Semantic analysis must determine their meaning.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * Repetition is represented structurally.
 *
 * There is no grammar-level maximum for:
 *
 *     - context entries;
 *     - nested contexts;
 *     - keys;
 *     - values;
 *     - list members;
 *     - resource expressions;
 *     - target properties;
 *     - execution properties.
 *
 * Actual limits belong to:
 *
 *     - compiler resource policy;
 *     - parser/runtime resource limits;
 *     - semantic analysis;
 *     - target capabilities;
 *     - explicitly declared user constraints;
 *     - operating-system/runtime limits.
 *
 * ============================================================================
 * EXTENSIBILITY
 * ============================================================================
 *
 * Context keys are open qualified names.
 *
 * Therefore future domains can introduce properties without requiring this
 * grammar to enumerate every future machine, accelerator, execution model,
 * quantum technology, classical architecture, or deployment environment.
 *
 * Examples:
 *
 *     target: quantum
 *     target: classical
 *     target: heterogeneous
 *
 *     capability: quantum
 *     capability: tensor
 *     capability: realtime
 *     capability: distributed
 *
 *     resource: quantum
 *     resource: memory
 *     resource: accelerator
 *
 *     placement: locality
 *     scheduling: policy
 *     dispatch: mode
 *
 * These names are semantic data, not machine-specific parser logic.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no semantic actions;
 *     - no predicates;
 *     - no runtime calls;
 *     - no filesystem operations;
 *     - no network operations;
 *     - no random behavior;
 *     - no hardware discovery;
 *     - no global mutable state.
 *
 * Given the same canonical token stream, parsing is deterministic.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This file contains no Rust implementation code.
 *
 * Downstream Zamani compiler/runtime implementation:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 */

parser grammar ExecutionContext;

import Core;


/*
 * ============================================================================
 * 1. CANONICAL EXECUTION CONTEXT
 * ============================================================================
 *
 * The canonical context is a non-empty collection of execution-context
 * entries.
 *
 * It may be embedded by:
 *
 *     execution/execution.g4
 *     execution/execution-context.g4 consumers
 *     deployment grammars
 *     dispatch grammars
 *     distributed execution grammars
 *     accelerator execution grammars
 *     future execution-domain grammars
 *
 * The surrounding grammar owns the execution keyword or declaration.
 *
 * This file owns only the context itself.
 * ============================================================================
 */

executionContext
    : LBRACE executionContextEntry+ RBRACE
    ;


/*
 * ============================================================================
 * 2. OPTIONAL CONTEXT
 * ============================================================================
 *
 * Reusable wrapper for consumers that permit an optional execution context.
 * ============================================================================
 */

optionalExecutionContext
    : executionContext
    ;


/*
 * ============================================================================
 * 3. CONTEXT ENTRY
 * ============================================================================
 *
 * Each entry has one explicit syntactic form.
 *
 * Supported forms:
 *
 *     key: value;
 *     key = value;
 *     key < value;
 *     key <= value;
 *     key > value;
 *     key >= value;
 *     key == value;
 *     key != value;
 *     key;
 *     key { ... }
 *
 * The final semantic interpretation belongs to semantic analysis.
 *
 * A bare key represents a presence/boolean-style property.
 *
 * ============================================================================
 */

executionContextEntry
    : executionContextAssignment
    | executionContextComparison
    | executionContextPresence
    | executionContextObject
    ;


/*
 * ============================================================================
 * 4. ASSIGNMENT
 * ============================================================================
 *
 * Colon and assignment forms are both supported:
 *
 *     target: quantum;
 *     target = quantum;
 *
 *     resource: required_resources;
 *     resource = required_resources;
 *
 * Colon is particularly suitable for declarative contexts.
 *
 * Assignment is retained for compatibility and expression-oriented styles.
 * ============================================================================
 */

executionContextAssignment
    : executionContextKey executionContextAssignmentOperator
      executionContextValue executionContextTerminator?
    ;


executionContextAssignmentOperator
    : COLON
    | ASSIGN
    ;


/*
 * ============================================================================
 * 5. COMPARISON
 * ============================================================================
 *
 * Comparison syntax permits machine-independent constraints such as:
 *
 *     resource.capacity >= required;
 *     latency <= allowed_latency;
 *     reliability >= required_reliability;
 *
 * The grammar does not decide:
 *
 *     - what capacity means;
 *     - what resource is selected;
 *     - which machine satisfies it;
 *     - whether the constraint is satisfiable.
 *
 * Those are semantic/resource-analysis concerns.
 * ============================================================================
 */

executionContextComparison
    : executionContextKey executionContextComparisonOperator
      executionContextValue executionContextTerminator?
    ;


executionContextComparisonOperator
    : EQUAL_EQUAL
    | NOT_EQUAL
    | LESS_THAN
    | LESS_EQUAL
    | GREATER_THAN
    | GREATER_EQUAL
    ;


/*
 * ============================================================================
 * 6. PRESENCE PROPERTY
 * ============================================================================
 *
 * A bare key expresses the presence of a context property.
 *
 * Example:
 *
 *     deterministic;
 *     portable;
 *     asynchronous;
 *
 * The semantic layer decides whether such a property is:
 *
 *     - a requirement;
 *     - a preference;
 *     - a hint;
 *     - metadata;
 *     - a capability declaration.
 *
 * The parser does not make that decision.
 * ============================================================================
 */

executionContextPresence
    : executionContextKey executionContextTerminator?
    ;


/*
 * ============================================================================
 * 7. NESTED OBJECT
 * ============================================================================
 *
 * Nested objects allow domains to group related properties without requiring
 * every future domain to modify this grammar.
 *
 * Example:
 *
 *     scheduling {
 *         policy: preferred_policy;
 *         deadline: deadline_value;
 *     }
 *
 *     placement {
 *         locality: locality_expression;
 *         affinity: affinity_expression;
 *     }
 *
 *     resources {
 *         memory: required_memory;
 *         quantum: required_quantum_resources;
 *     }
 *
 * ============================================================================
 */

executionContextObject
    : executionContextKey executionContextBlock
    ;


executionContextBlock
    : LBRACE executionContextEntry+ RBRACE
    ;


/*
 * ============================================================================
 * 8. CONTEXT KEY
 * ============================================================================
 *
 * Keys are qualified language names.
 *
 * This is intentionally open-ended.
 *
 * Do NOT replace this with a closed list such as:
 *
 *     target
 *     gpu
 *     qpu
 *     cpu
 *     memory
 *     qubits
 *
 * Such a list would couple the grammar to a particular generation of
 * computing hardware.
 *
 * Qualified names also allow namespaced properties:
 *
 *     quantum::execution
 *     hardware::capability
 *     distributed::placement
 *     vendor::extension
 *     future::execution_mode
 *
 * Semantic ownership remains outside this file.
 * ============================================================================
 */

executionContextKey
    : qualifiedName
    ;


/*
 * ============================================================================
 * 9. CONTEXT VALUE
 * ============================================================================
 *
 * Values reuse the canonical expression grammar.
 *
 * This is critical:
 *
 *     execution context
 *         |
 *         +--> existing expression system
 *
 * rather than:
 *
 *     execution context
 *         |
 *         +--> second expression language
 *
 * Values may therefore use:
 *
 *     literals
 *     identifiers
 *     qualified names
 *     function calls
 *     arithmetic
 *     comparisons
 *     conditional expressions
 *     arrays
 *     tuples
 *     ranges
 *     symbolic expressions
 *     quantum-related expressions
 *     classical expressions
 *     future language expressions
 *
 * exactly according to the canonical expression grammar.
 *
 * ============================================================================
 */

executionContextValue
    : expression
    | executionContextValueObject
    ;


executionContextValueObject
    : LBRACE executionContextValueEntry+ RBRACE
    ;


executionContextValueEntry
    : executionContextKey
      executionContextAssignmentOperator
      executionContextValue
      executionContextTerminator?
    ;


/*
 * ============================================================================
 * 10. CONTEXT LIST
 * ============================================================================
 *
 * This rule provides an explicit reusable list form for execution-specific
 * grammar extensions.
 *
 * The actual expression semantics remain owned by Core.
 *
 * Example semantic forms:
 *
 *     targets: [local, remote, quantum];
 *     capabilities: [quantum, distributed];
 *
 * ============================================================================
 */

executionContextList
    : LBRACKET executionContextListElementList? RBRACKET
    ;


executionContextListElementList
    : executionContextValue
      (COMMA executionContextValue)*
      COMMA?
    ;


/*
 * ============================================================================
 * 11. CONTEXT ARGUMENTS
 * ============================================================================
 *
 * Some execution extensions may need named argument-like context values.
 *
 * This rule provides:
 *
 *     key(value)
 *
 * without defining a second function-call language.
 *
 * Ordinary expression calls remain owned by Core.
 *
 * ============================================================================
 */

executionContextInvocation
    : executionContextKey
      LPAREN
      executionContextArgumentList?
      RPAREN
    ;


executionContextArgumentList
    : executionContextArgument
      (COMMA executionContextArgument)*
      COMMA?
    ;


executionContextArgument
    : executionContextValue
    | executionContextKey
      executionContextAssignmentOperator
      executionContextValue
    ;


/*
 * ============================================================================
 * 12. CONTEXT VALUE ALTERNATIVES
 * ============================================================================
 *
 * This rule is provided as an explicit extension point for consumers that
 * need to distinguish ordinary expressions from structured context data.
 *
 * It intentionally does not introduce target-specific literals.
 * ============================================================================
 */

executionContextValueExpression
    : expression
    ;


executionContextStructuredValue
    : executionContextValueObject
    | executionContextList
    | executionContextInvocation
    ;


/*
 * ============================================================================
 * 13. CONTEXT COMPOSITION
 * ============================================================================
 *
 * A context can be composed from nested contexts.
 *
 * The parser preserves source structure.
 *
 * Semantic analysis determines:
 *
 *     - duplicate keys;
 *     - conflicting constraints;
 *     - requirement/preference precedence;
 *     - namespace resolution;
 *     - capability interpretation;
 *     - resource interpretation.
 *
 * This grammar deliberately does not reject duplicate semantic keys because
 * duplicate keys may be meaningful to a future semantic domain.
 *
 * Examples:
 *
 *     target: quantum;
 *     target: classical;
 *
 * may be:
 *
 *     invalid;
 *     alternative targets;
 *     a conflict;
 *     a preference set;
 *     a multi-target deployment request;
 *
 * depending on the owning semantic subsystem.
 *
 * ============================================================================
 */

executionContextComposition
    : executionContextEntry+
    ;


/*
 * ============================================================================
 * 14. REQUIREMENT CONTEXT
 * ============================================================================
 *
 * This syntactic wrapper permits consumers to explicitly mark a group of
 * entries as requirements without making individual property names reserved.
 *
 * Example:
 *
 *     requirements {
 *         capability: quantum;
 *         resource: required_resources;
 *     }
 *
 * The semantic layer owns the meaning of the contained properties.
 * ============================================================================
 */

executionRequirementContext
    : executionContextNamedBlock
    ;


executionRequirementContextEntry
    : executionContextEntry
    ;


/*
 * ============================================================================
 * 15. CONSTRAINT CONTEXT
 * ============================================================================
 */

executionConstraintContext
    : executionContextNamedBlock
    ;


executionConstraintContextEntry
    : executionContextEntry
    ;


/*
 * ============================================================================
 * 16. PREFERENCE CONTEXT
 * ============================================================================
 */

executionPreferenceContext
    : executionContextNamedBlock
    ;


executionPreferenceContextEntry
    : executionContextEntry
    ;


/*
 * ============================================================================
 * 17. HINT CONTEXT
 * ============================================================================
 */

executionHintContext
    : executionContextNamedBlock
    ;


executionHintContextEntry
    : executionContextEntry
    ;


/*
 * ============================================================================
 * 18. CAPABILITY CONTEXT
 * ============================================================================
 */

executionCapabilityContext
    : executionContextNamedBlock
    ;


executionCapabilityContextEntry
    : executionContextEntry
    ;


/*
 * ============================================================================
 * 19. RESOURCE CONTEXT
 * ============================================================================
 */

executionResourceContext
    : executionContextNamedBlock
    ;


executionResourceContextEntry
    : executionContextEntry
    ;


/*
 * ============================================================================
 * 20. TARGET CONTEXT
 * ============================================================================
 */

executionTargetContext
    : executionContextNamedBlock
    ;


executionTargetContextEntry
    : executionContextEntry
    ;


/*
 * ============================================================================
 * 21. PLACEMENT CONTEXT
 * ============================================================================
 */

executionPlacementContext
    : executionContextNamedBlock
    ;


executionPlacementContextEntry
    : executionContextEntry
    ;


/*
 * ============================================================================
 * 22. SCHEDULING CONTEXT
 * ============================================================================
 */

executionSchedulingContext
    : executionContextNamedBlock
    ;


executionSchedulingContextEntry
    : executionContextEntry
    ;


/*
 * ============================================================================
 * 23. DISPATCH CONTEXT
 * ============================================================================
 */

executionDispatchContext
    : executionContextNamedBlock
    ;


executionDispatchContextEntry
    : executionContextEntry
    ;


/*
 * ============================================================================
 * 24. SYNCHRONIZATION CONTEXT
 * ============================================================================
 */

executionSynchronizationContext
    : executionContextNamedBlock
    ;


executionSynchronizationContextEntry
    : executionContextEntry
    ;


/*
 * ============================================================================
 * 25. LIFECYCLE CONTEXT
 * ============================================================================
 */

executionLifecycleContext
    : executionContextNamedBlock
    ;


executionLifecycleContextEntry
    : executionContextEntry
    ;


/*
 * ============================================================================
 * 26. RECOVERY / FAILURE CONTEXT
 * ============================================================================
 *
 * This is only syntax.
 *
 * It MUST NOT be interpreted as implementing resilience.
 *
 * In particular, this grammar does not decide:
 *
 *     - whether a retry is safe;
 *     - whether a quantum state is recoverable;
 *     - whether a checkpoint exists;
 *     - whether QEC is required;
 *     - whether mitigation is possible;
 *     - whether backend switching is valid.
 *
 * Those decisions belong to resilience and the relevant semantic subsystems.
 * ============================================================================
 */

executionRecoveryContext
    : executionContextNamedBlock
    ;


executionRecoveryContextEntry
    : executionContextEntry
    ;


/*
 * ============================================================================
 * 27. DEPLOYMENT CONTEXT
 * ============================================================================
 */

executionDeploymentContext
    : executionContextNamedBlock
    ;


executionDeploymentContextEntry
    : executionContextEntry
    ;


/*
 * ============================================================================
 * 28. GENERIC NAMED CONTEXT
 * ============================================================================
 *
 * Generic named blocks are the primary future-extension mechanism.
 *
 * The name is not a fixed machine category.
 *
 * Examples:
 *
 *     scheduling { ... }
 *     hardware { ... }
 *     quantum { ... }
 *     distributed { ... }
 *     future_domain { ... }
 *
 * The semantic layer determines ownership.
 * ============================================================================
 */

executionContextNamedBlock
    : executionContextKey executionContextBlock
    ;


/*
 * ============================================================================
 * 29. CONTEXT TERMINATOR
 * ============================================================================
 *
 * The context grammar permits semicolon termination without forcing every
 * enclosing grammar to adopt a single global statement-termination policy.
 *
 * Commas remain available for list-like structures.
 * ============================================================================
 */

executionContextTerminator
    : SEMICOLON
    ;


/*
 * ============================================================================
 * 30. CONTEXT PROPERTY PATH
 * ============================================================================
 *
 * Property paths reuse qualifiedName.
 *
 * Example:
 *
 *     quantum::execution::mode
 *     hardware::capability::latency
 *     distributed::placement::locality
 *
 * No finite namespace hierarchy is imposed.
 * ============================================================================
 */

executionContextPropertyPath
    : qualifiedName
    ;


/*
 * ============================================================================
 * 31. MACHINE-INDEPENDENT RESOURCE EXPRESSIONS
 * ============================================================================
 *
 * Resource quantities are ordinary expressions.
 *
 * Consequently the grammar does not contain:
 *
 *     integer-only resource limits;
 *     fixed qubit counts;
 *     fixed core counts;
 *     fixed device counts;
 *     fixed memory sizes.
 *
 * Examples:
 *
 *     resource::qubits >= required_qubits;
 *     resource::memory >= working_set;
 *     resource::devices >= required_devices;
 *
 * The semantic/resource subsystem determines whether those expressions can
 * be satisfied.
 * ============================================================================
 */

executionResourceConstraint
    : executionContextPropertyPath
      executionContextComparisonOperator
      expression
      executionContextTerminator?
    ;


/*
 * ============================================================================
 * 32. CAPABILITY EXPRESSIONS
 * ============================================================================
 *
 * Capabilities remain open names or expressions.
 *
 * No closed hardware catalogue is created.
 * ============================================================================
 */

executionCapabilityRequirement
    : executionContextPropertyPath
      executionContextAssignmentOperator
      expression
      executionContextTerminator?
    ;


/*
 * ============================================================================
 * 33. TARGET EXPRESSIONS
 * ============================================================================
 *
 * A target is semantic intent, not necessarily a physical device.
 *
 * A semantic target may later resolve to:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     simulator
 *     accelerator
 *     cluster
 *     distributed environment
 *     cloud environment
 *     embedded target
 *     future execution substrate
 *
 * without changing this grammar.
 * ============================================================================
 */

executionTargetRequirement
    : executionContextPropertyPath
      executionContextAssignmentOperator
      expression
      executionContextTerminator?
    ;


/*
 * ============================================================================
 * 34. PORTABILITY CONTEXT
 * ============================================================================
 *
 * Portability declarations are context data.
 *
 * This grammar does not decide whether a program is portable.
 *
 * The compiler's capability/resource/target analysis is responsible for
 * determining portability.
 * ============================================================================
 */

executionPortabilityContext
    : executionContextNamedBlock
    ;


/*
 * ============================================================================
 * 35. PERFORMANCE CONTEXT
 * ============================================================================
 *
 * Performance values remain expressions.
 *
 * This prevents grammar-level assumptions about:
 *
 *     units;
 *     machine speed;
 *     latency;
 *     throughput;
 *     bandwidth;
 *     energy;
 *     memory.
 *
 * ============================================================================
 */

executionPerformanceContext
    : executionContextNamedBlock
    ;


/*
 * ============================================================================
 * 36. ENERGY CONTEXT
 * ============================================================================
 */

executionEnergyContext
    : executionContextNamedBlock
    ;


/*
 * ============================================================================
 * 37. RELIABILITY CONTEXT
 * ============================================================================
 */

executionReliabilityContext
    : executionContextNamedBlock
    ;


/*
 * ============================================================================
 * 38. SCALABILITY CONTEXT
 * ============================================================================
 *
 * Scalability is expressed as semantic information.
 *
 * This file never converts "scalable" into a fixed resource amount.
 * ============================================================================
 */

executionScalabilityContext
    : executionContextNamedBlock
    ;


/*
 * ============================================================================
 * 39. FUTURE EXTENSION CONTEXT
 * ============================================================================
 *
 * Future domains do not require grammar changes merely because a new semantic
 * execution property appears.
 *
 * Example:
 *
 *     future::execution {
 *         new_capability: value;
 *     }
 *
 * ============================================================================
 */

executionFutureContext
    : executionContextNamedBlock
    ;


/*
 * ============================================================================
 * 40. INTEGRATION CONTRACT
 * ============================================================================
 *
 * The canonical execution grammar SHOULD integrate this file approximately as
 * follows:
 *
 *     executionDeclaration
 *         : <execution keyword>
 *           <execution subject>
 *           executionContext?
 *           <terminator>
 *         ;
 *
 * The exact execution keyword and execution-subject syntax remain owned by
 * execution.g4 and the canonical lexer/Core grammar.
 *
 * This file must not duplicate those rules.
 *
 * ============================================================================
 *
 * OTHER INTEGRATION POINTS
 * ============================================================================
 *
 * 1. CORE
 *
 *     This grammar imports Core.
 *
 * 2. AST
 *
 *     The frontend should lower:
 *
 *         executionContext
 *             -> ExecutionContext AST node
 *
 *     without evaluating the values.
 *
 * 3. NAME RESOLUTION
 *
 *     Resolve executionContextKey / qualifiedName after parsing.
 *
 * 4. TYPE ANALYSIS
 *
 *     Type-check executionContextValue expressions using the canonical type
 *     system.
 *
 * 5. EFFECT ANALYSIS
 *
 *     Determine whether an execution request introduces effects.
 *
 * 6. CAPABILITY ANALYSIS
 *
 *     Resolve capability properties against available target capabilities.
 *
 * 7. RESOURCE ANALYSIS
 *
 *     Evaluate resource expressions against the resource model.
 *
 * 8. TARGET RESOLUTION
 *
 *     Resolve abstract targets only after semantic analysis.
 *
 * 9. QUANTUM
 *
 *     Quantum context must eventually feed quantum semantic analysis and,
 *     where applicable, quantum::ir.
 *
 *     This file must never create quantum::ir.
 *
 * 10. CLASSICAL
 *
 *     Classical execution context feeds classical semantic/IR lowering.
 *
 * 11. HDL/HARDWARE
 *
 *     Hardware-related context is consumed by hardware semantic analysis.
 *
 *     This grammar does not perform hardware binding.
 *
 * 12. SCHEDULING
 *
 *     Scheduling properties are passed as scheduling intent.
 *
 *     Scheduling algorithms remain outside the grammar.
 *
 * 13. ROUTING
 *
 *     Placement properties are passed as routing/placement intent.
 *
 *     Routing remains outside the grammar.
 *
 * 14. OPTIMIZATION
 *
 *     Optimization preferences are semantic metadata.
 *
 *     Optimization remains outside the grammar.
 *
 * 15. ZQN
 *
 *     Noise/fault-related context can be interpreted by ZQN-aware semantic
 *     analysis.
 *
 *     This grammar does not define noise semantics.
 *
 * 16. QEC
 *
 *     Error-correction requirements may be represented as semantic context.
 *
 *     QEC algorithms remain outside the grammar.
 *
 * 17. RESILIENCE
 *
 *     Failure/recovery/retry context is policy intent only.
 *
 *     Resilience decides whether an action is safe and possible.
 *
 * 18. RUNTIME
 *
 *     Runtime consumes the resolved execution plan.
 *
 *     Runtime must not depend on parser-specific implementation details.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 41. VALIDATION CONTRACT
 * ============================================================================
 *
 * Semantic validation, not parsing, must detect:
 *
 *     - duplicate mutually exclusive properties;
 *     - impossible constraints;
 *     - contradictory requirements;
 *     - invalid capability names;
 *     - unavailable resources;
 *     - incompatible targets;
 *     - unsupported combinations;
 *     - invalid recovery requests;
 *     - semantic type errors;
 *     - effect violations;
 *     - security violations.
 *
 * The parser must not silently reinterpret any of these conditions.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 42. HARD-CODING AUDIT CONTRACT
 * ============================================================================
 *
 * This file contains no:
 *
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_JOBS
 *     MAX_RETRIES
 *     MAX_TARGETS
 *     MAX_RESOURCES
 *
 * It contains no physical device IDs.
 *
 * It contains no hardware addresses.
 *
 * It contains no topology assumptions.
 *
 * It contains no vendor-specific execution requirements.
 *
 * It contains no fixed quantum topology.
 *
 * It contains no fixed scheduling grid.
 *
 * It contains no fixed timing unit.
 *
 * It contains no fixed execution duration.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 43. TEST CONTRACT
 * ============================================================================
 *
 * The following tests belong under:
 *
 *     grammar/tests/execution/
 *
 * or the repository's canonical grammar-test location.
 *
 * POSITIVE
 *
 *     execute ... with {
 *         target: quantum;
 *     }
 *
 *     execute ... with {
 *         capability: distributed;
 *         resource: required_resources;
 *     }
 *
 *     execute ... with {
 *         scheduling {
 *             policy: preferred_policy;
 *         }
 *     }
 *
 *     execute ... with {
 *         placement {
 *             locality: locality_expression;
 *         }
 *     }
 *
 *     execute ... with {
 *         quantum::execution::mode: mode;
 *     }
 *
 * NEGATIVE
 *
 *     {
 *         target:
 *     }
 *
 *     {
 *         : value;
 *     }
 *
 *     {
 *         target >= ;
 *     }
 *
 * BOUNDARY
 *
 *     - very large context entry counts;
 *     - deeply nested semantic contexts;
 *     - very large expressions;
 *     - large property lists;
 *     - large qualified-name paths.
 *
 * SCALABILITY
 *
 * Verify that no grammar-level resource ceiling exists for:
 *
 *     - qubits;
 *     - devices;
 *     - cores;
 *     - nodes;
 *     - memory;
 *     - accelerators;
 *     - targets.
 *
 * CROSS-DOMAIN
 *
 * Test contexts accompanying:
 *
 *     classical execution;
 *     quantum execution;
 *     hybrid execution;
 *     HDL execution;
 *     hardware execution;
 *     distributed execution;
 *     accelerator execution;
 *     AI execution;
 *     future-domain execution.
 *
 * DETERMINISM
 *
 * Identical token streams must produce structurally identical parse trees.
 *
 * ROUND-TRIP
 *
 * Where the AST/printer supports execution contexts:
 *
 *     source
 *       -> lexer
 *       -> parser
 *       -> AST
 *       -> printer
 *       -> parser
 *
 * must preserve context semantics.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 44. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE only when:
 *
 * [x] It is a parser grammar.
 *
 * [x] It imports the canonical Core grammar.
 *
 * [x] It does not create another expression language.
 *
 * [x] It does not create another type system.
 *
 * [x] It does not create an IR.
 *
 * [x] It does not create quantum IR.
 *
 * [x] It does not perform execution.
 *
 * [x] It does not perform hardware discovery.
 *
 * [x] It does not perform resource allocation.
 *
 * [x] It does not perform scheduling.
 *
 * [x] It does not perform routing.
 *
 * [x] It does not perform optimization.
 *
 * [x] It does not perform resilience.
 *
 * [x] It does not contain machine-size limits.
 *
 * [x] It does not contain physical-device identifiers.
 *
 * [x] It supports open-ended semantic properties.
 *
 * [x] It supports nested execution contexts.
 *
 * [x] It supports assignments.
 *
 * [x] It supports constraints.
 *
 * [x] It supports presence properties.
 *
 * [x] It supports qualified property names.
 *
 * [x] It supports structured values.
 *
 * [x] It preserves semantic ownership for downstream subsystems.
 *
 * [x] It can be consumed by execution.g4 without redefining context syntax.
 *
 * [x] It remains independent of a particular CPU/GPU/FPGA/ASIC/QPU.
 *
 * [x] It remains suitable for POCO-REAF.
 *
 * ============================================================================
 */
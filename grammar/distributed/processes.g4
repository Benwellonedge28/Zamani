/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/distributed/processes.g4
 *
 * Grammar:
 *     Processes
 *
 * Status:
 *     Production distributed-process parser grammar.
 *
 * Baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     - No embedded Rust actions.
 *     - No semantic predicates.
 *     - No unsafe code.
 *     - No filesystem access.
 *     - No network access.
 *     - No hardware access.
 *     - No runtime callbacks.
 *     - No mutable global parser state.
 *     - No randomness.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar owns SOURCE-LEVEL LOGICAL PROCESS SYNTAX for distributed
 * Zamani programs.
 *
 * A process is a logical unit of computation and execution intent.
 *
 * A process is NOT intrinsically:
 *
 *     - a CPU;
 *     - a CPU core;
 *     - a hardware thread;
 *     - an operating-system process;
 *     - a machine;
 *     - a VM;
 *     - a container;
 *     - a GPU;
 *     - an FPGA;
 *     - an ASIC;
 *     - a QPU;
 *     - a physical quantum processor;
 *     - a network endpoint;
 *     - a socket;
 *     - a cloud instance;
 *     - a node;
 *     - a physical device.
 *
 * Those are possible downstream realizations.
 *
 * The source-level process abstraction therefore remains portable across:
 *
 *     embedded systems
 *     single-machine execution
 *     multicore systems
 *     many-core systems
 *     heterogeneous systems
 *     clusters
 *     HPC systems
 *     clouds
 *     federated systems
 *     edge systems
 *     accelerator systems
 *     quantum-classical systems
 *     distributed quantum systems
 *     future computing systems
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical ZamaniLexer
 *          |
 *          v
 *     Processes parser grammar
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          +--> name resolution
 *          +--> type analysis
 *          +--> effect analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> ownership/lifetime analysis
 *          +--> security analysis
 *          +--> distributed semantic analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical computation
 *          +--> quantum computation
 *          |       |
 *          |       +--> quantum::ir
 *          |
 *          +--> HDL/hardware computation
 *          +--> distributed execution metadata
 *          +--> resource requirements
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     placement / routing / scheduling
 *          |
 *          v
 *     target realization
 *          |
 *          v
 *     runtime
 *
 * This grammar does not construct or modify IR.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Distributed process syntax follows:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * A process expresses WHAT logical computation exists.
 *
 * It does not permanently encode WHERE that computation must execute.
 *
 * Therefore this grammar MUST NOT impose universal limits on:
 *
 *     nodes
 *     processes
 *     subprocesses
 *     workers
 *     tasks
 *     actors
 *     threads
 *     CPUs
 *     GPUs
 *     FPGAs
 *     QPUs
 *     devices
 *     memory
 *     storage
 *     channels
 *     network links
 *     process nesting
 *     process parameters
 *     process dependencies
 *     process instances
 *
 * There is deliberately no:
 *
 *     MAX_NODES
 *     MAX_PROCESSES
 *     MAX_WORKERS
 *     MAX_TASKS
 *     MAX_THREADS
 *     MAX_DEVICES
 *     MAX_PROCESS_DEPTH
 *     MAX_PROCESS_ARGUMENTS
 *     MAX_PROCESS_INSTANCES
 *
 * Repetition and recursion are represented by ANTLR's `*` and `+`.
 *
 * Actual resource limitations are downstream implementation/resource
 * constraints and MUST NOT become language-level grammar limits.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - logical process declarations;
 *     - process names;
 *     - process parameter syntax;
 *     - process body composition;
 *     - process-local bindings;
 *     - process invocation syntax;
 *     - process relationships;
 *     - process dependencies;
 *     - process lifecycle intent;
 *     - process execution intent;
 *     - process requirements;
 *     - process constraints;
 *     - process preferences;
 *     - process hints;
 *     - process attributes represented through expressions;
 *     - process-local composition.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - identifier syntax;
 *     - qualified-name syntax;
 *     - general expression syntax;
 *     - general type syntax;
 *     - generic concurrency semantics;
 *     - node discovery;
 *     - service discovery;
 *     - network transport;
 *     - placement algorithms;
 *     - scheduling algorithms;
 *     - routing algorithms;
 *     - replication algorithms;
 *     - consistency algorithms;
 *     - consensus algorithms;
 *     - fault-tolerance algorithms;
 *     - deployment;
 *     - resource discovery;
 *     - resource allocation;
 *     - hardware discovery;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * Canonical lexical authority:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Canonical shared parser components:
 *
 *     grammar/core/names.g4
 *     grammar/expressions/expressions.g4
 *
 * This grammar therefore imports:
 *
 *     Names
 *     Expressions
 *
 * and MUST NOT redefine:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     expressionList
 *
 * or their underlying lexical tokens.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * The public entry point of this grammar is:
 *
 *     distributedProcessDeclaration
 *
 * The aggregate distributed grammar:
 *
 *     grammar/distributed/distributed.g4
 *
 * should import this grammar and delegate process syntax to:
 *
 *     distributedProcessDeclaration
 *
 * The canonical parser composition layer:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * remains the owner of cross-domain parser composition.
 *
 * grammar/Zamani.g4 remains the single complete-language root.
 *
 * This file must not become a second root grammar.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser must preserve the syntactic structure required by the
 * domain-neutral AST.
 *
 * A process declaration must preserve at least:
 *
 *     - process name;
 *     - qualified process kind;
 *     - parameter ordering;
 *     - parameter names;
 *     - parameter type syntax;
 *     - optional parameter initializers;
 *     - process member ordering;
 *     - expression ordering;
 *     - dependency ordering;
 *     - invocation argument ordering;
 *     - lifecycle clause ordering;
 *     - source nesting;
 *     - source spans.
 *
 * This grammar does not prescribe Rust AST implementation types.
 *
 * A recommended semantic mapping is:
 *
 *     distributedProcessDeclaration
 *             |
 *             v
 *     domain-neutral declaration node
 *             |
 *             v
 *     semantic process entity
 *             |
 *             v
 *     distributed execution metadata
 *
 * If a process contains quantum computation:
 *
 *     process syntax
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum semantics
 *          |
 *          v
 *     quantum::ir
 *
 * This grammar never constructs a second quantum IR.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing answers:
 *
 *     "Is this structurally valid process syntax?"
 *
 * Semantic analysis answers:
 *
 *     "What does this process mean?"
 *
 * Semantic analysis determines:
 *
 *     - whether the process kind is recognized;
 *     - whether the process name is valid;
 *     - whether parameters are valid;
 *     - whether parameter types are valid;
 *     - whether bindings are valid;
 *     - whether dependencies resolve;
 *     - whether process relationships are legal;
 *     - whether requested capabilities exist;
 *     - whether resource requirements can be satisfied;
 *     - whether effects are permitted;
 *     - whether ownership/lifetime rules are satisfied;
 *     - whether communication is legal;
 *     - whether placement is feasible;
 *     - whether migration is legal;
 *     - whether execution policies are compatible;
 *     - whether the target can realize the process.
 *
 * None of those decisions are performed by this grammar.
 *
 * ============================================================================
 * OPEN-WORLD PROCESS MODEL
 * ============================================================================
 *
 * The process kind is intentionally represented by a qualified name rather
 * than a closed lexer enumeration.
 *
 * Examples:
 *
 *     distributed::process
 *     distributed::worker
 *     distributed::actor
 *     distributed::task
 *     distributed::service_process
 *     distributed::pipeline
 *     distributed::agent
 *     distributed::quantum_process
 *     distributed::hybrid_process
 *     distributed::future_process
 *
 * Future semantic process categories can therefore be introduced without
 * requiring a new lexer keyword.
 *
 * The parser does not decide whether a process kind is:
 *
 *     stable
 *     proposed
 *     experimental
 *     deprecated
 *     vendor-specific
 *     unknown
 *
 * The semantic feature registry decides that.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * This grammar must not encode:
 *
 *     CPU identifiers
 *     GPU identifiers
 *     FPGA identifiers
 *     QPU identifiers
 *     physical machine identifiers
 *     physical node identifiers
 *     physical memory-bank identifiers
 *     fixed device counts
 *     fixed processor counts
 *     fixed thread counts
 *     fixed topology
 *     fixed network addresses
 *     fixed ports
 *
 * A source program may contain ordinary values that happen to be numeric or
 * textual. Such values are program data unless downstream semantic analysis
 * classifies them otherwise.
 *
 * The grammar itself does not assign physical meaning to those values.
 *
 * ============================================================================
 * REQUIREMENT / CONSTRAINT / PREFERENCE / HINT
 * ============================================================================
 *
 * Process syntax may contain generic process clauses represented by qualified
 * names and expressions.
 *
 * These categories must remain semantically distinct downstream:
 *
 *     requirement
 *     constraint
 *     preference
 *     hint
 *     capability
 *
 * For example, a process may semantically express:
 *
 *     requires capability("distributed.compute")
 *
 * or:
 *
 *     requires memory >= required_memory
 *
 * without converting either expression into a hard-coded compiler capacity.
 *
 * A preference must not silently become a mandatory requirement.
 *
 * A hint must not silently become a semantic constraint.
 *
 * ============================================================================
 * PROCESS IDENTITY
 * ============================================================================
 *
 * A process declaration has:
 *
 *     process kind
 *     process name
 *     optional parameters
 *     process body
 *
 * The process name is a normal Zamani identifier.
 *
 * The grammar does not create specialized lexical categories such as:
 *
 *     ProcessName
 *     WorkerName
 *     NodeProcessName
 *     QPUProcessName
 *
 * ============================================================================
 * PARAMETERS
 * ============================================================================
 *
 * Parameters are ordinary Zamani names with optional type and initializer
 * syntax.
 *
 * The grammar does not impose a maximum number of parameters.
 *
 * Parameter semantics, including:
 *
 *     ownership
 *     borrowing
 *     lifetime
 *     generic constraints
 *     capability requirements
 *     resource requirements
 *
 * are handled downstream.
 *
 * ============================================================================
 * PROCESS BODY
 * ============================================================================
 *
 * The body is a recursive source-level process scope.
 *
 * It supports:
 *
 *     - local bindings;
 *     - process invocations;
 *     - process dependencies;
 *     - relationships;
 *     - lifecycle intents;
 *     - generic process operations;
 *     - nested process declarations;
 *     - nested process scopes.
 *
 * It deliberately does not duplicate the complete Zamani statement grammar.
 *
 * Ordinary language statements should remain owned by the canonical statement
 * subsystem and may be composed around this grammar at the higher parser
 * level.
 *
 * ============================================================================
 * INVOCATION
 * ============================================================================
 *
 * A process invocation uses ordinary qualified names and expressions.
 *
 * Conceptually:
 *
 *     distributed::process_name(arg1, arg2);
 *
 * or:
 *
 *     process_name(arg1, arg2);
 *
 * The parser does not determine whether the target is:
 *
 *     local
 *     remote
 *     replicated
 *     migrated
 *     accelerated
 *     quantum
 *     heterogeneous
 *
 * Semantic analysis and execution planning determine that.
 *
 * ============================================================================
 * DEPENDENCIES
 * ============================================================================
 *
 * Process dependencies express logical ordering/data/control relationships.
 *
 * Conceptually:
 *
 *     process_a -> process_b;
 *
 * means that `process_b` depends on `process_a`.
 *
 * It does not mean:
 *
 *     physical network route
 *     hardware link
 *     physical adjacency
 *     scheduler assignment
 *     machine placement
 *
 * ============================================================================
 * LIFECYCLE
 * ============================================================================
 *
 * Lifecycle intent is represented through an open qualified operation form.
 *
 * This permits semantic concepts such as:
 *
 *     distributed::start(...)
 *     distributed::stop(...)
 *     distributed::pause(...)
 *     distributed::resume(...)
 *     distributed::restart(...)
 *     distributed::migrate(...)
 *     distributed::checkpoint(...)
 *     distributed::recover(...)
 *
 * without forcing every lifecycle concept into the lexical vocabulary.
 *
 * The grammar does not implement lifecycle behavior.
 *
 * ============================================================================
 * COMMUNICATION INTEGRATION
 * ============================================================================
 *
 * Communication is owned by:
 *
 *     grammar/distributed/communication.g4
 *
 * Process syntax may invoke communication operations through generic process
 * operations or through the aggregate distributed grammar.
 *
 * This grammar does not redefine:
 *
 *     send
 *     receive
 *     broadcast
 *     multicast
 *     gather
 *     scatter
 *     publish
 *     subscribe
 *
 * as lexical or semantic authorities.
 *
 * ============================================================================
 * NODE INTEGRATION
 * ============================================================================
 *
 * Logical nodes are owned by:
 *
 *     grammar/distributed/nodes.g4
 *
 * A process may semantically be associated with a node, worker, region, or
 * execution domain.
 *
 * This grammar does not redefine node syntax or node semantics.
 *
 * ============================================================================
 * PLACEMENT INTEGRATION
 * ============================================================================
 *
 * Placement intent is owned by:
 *
 *     grammar/distributed/placement.g4
 *
 * A process may have:
 *
 *     placement requirements;
 *     placement constraints;
 *     placement preferences;
 *     locality intent;
 *     migration intent.
 *
 * Those concepts are semantically integrated downstream.
 *
 * This grammar does not choose a physical location.
 *
 * ============================================================================
 * REMOTE EXECUTION INTEGRATION
 * ============================================================================
 *
 * Remote execution is owned by:
 *
 *     grammar/distributed/remote-execution.g4
 *
 * A process can be the subject of remote execution intent, but this grammar
 * does not implement remote invocation, transport, or deployment.
 *
 * ============================================================================
 * SERVICES INTEGRATION
 * ============================================================================
 *
 * Services are owned by:
 *
 *     grammar/distributed/services.g4
 *
 * A service may contain or invoke processes.
 *
 * The service grammar remains responsible for service-specific syntax.
 *
 * This file remains responsible only for process syntax.
 *
 * ============================================================================
 * REPLICATION / CONSISTENCY INTEGRATION
 * ============================================================================
 *
 * Replication:
 *
 *     grammar/distributed/replication.g4
 *
 * Consistency:
 *
 *     grammar/distributed/consistency.g4
 *
 * A process may participate in replicated or consistency-managed execution.
 *
 * This grammar does not implement:
 *
 *     replica creation;
 *     replica scheduling;
 *     consensus;
 *     quorum selection;
 *     state-machine replication;
 *     consistency algorithms.
 *
 * ============================================================================
 * FAULT-TOLERANCE INTEGRATION
 * ============================================================================
 *
 * Fault-tolerance is owned by:
 *
 *     grammar/distributed/fault-tolerance.g4
 *
 * Process syntax may be a semantic subject of:
 *
 *     retry
 *     recovery
 *     restart
 *     checkpoint
 *     failover
 *
 * but the grammar does not implement the algorithms.
 *
 * ============================================================================
 * CONCURRENCY INTEGRATION
 * ============================================================================
 *
 * Generic concurrency is owned by:
 *
 *     grammar/concurrency/
 *
 * A process is not synonymous with:
 *
 *     thread
 *     task
 *     future
 *     actor
 *     coroutine
 *
 * Semantic analysis determines the relationship between these abstractions.
 *
 * This distinction is necessary for POCO-REAF.
 *
 * ============================================================================
 * NETWORKING INTEGRATION
 * ============================================================================
 *
 * Process communication remains transport-neutral.
 *
 * This grammar does not select:
 *
 *     TCP
 *     UDP
 *     QUIC
 *     MPI
 *     RDMA
 *     InfiniBand
 *     shared memory
 *     vendor-specific transport
 *
 * Networking and runtime systems determine the realization.
 *
 * ============================================================================
 * CLASSICAL / QUANTUM / HDL INTEGRATION
 * ============================================================================
 *
 * A process may contain or invoke:
 *
 *     classical computation;
 *     quantum computation;
 *     hybrid computation;
 *     HDL/hardware computation;
 *     accelerator computation;
 *     AI computation;
 *     data processing;
 *     future computational domains.
 *
 * This grammar remains domain-neutral.
 *
 * If quantum semantics are encountered:
 *
 *     process
 *        |
 *        v
 *     semantic analysis
 *        |
 *        v
 *     quantum::ir
 *
 * No process-specific quantum IR is introduced.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing is a pure function of:
 *
 *     source token stream
 *     grammar version
 *     parser configuration explicitly supplied by the caller
 *
 * It must not depend on:
 *
 *     hardware;
 *     resource availability;
 *     network state;
 *     filesystem state;
 *     environment variables;
 *     wall-clock time;
 *     randomness;
 *     runtime state.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar contains no executable actions.
 *
 * Parsing must not:
 *
 *     execute a process;
 *     contact a node;
 *     start a service;
 *     create a process;
 *     connect to a network;
 *     access credentials;
 *     inspect hardware;
 *     invoke a compiler backend.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * All collections are unbounded by language design.
 *
 * Examples:
 *
 *     process parameters     -> *
 *     process members        -> *
 *     dependencies           -> *
 *     invocations            -> *
 *     nested process scopes  -> *
 *
 * The language therefore imposes no artificial finite process capacity.
 *
 * "Infinity" means:
 *
 *     no language-level finite process-resource ceiling.
 *
 * Actual execution remains bounded by:
 *
 *     available memory;
 *     compiler resources;
 *     runtime resources;
 *     operating-system limits;
 *     target capabilities;
 *     deployment policy;
 *     physical resources.
 *
 * ============================================================================
 */

/*
 * ============================================================================
 * ANTLR PARSER GRAMMAR
 * ============================================================================
 */

parser grammar Processes;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Expressions;


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * This is the only public process-declaration entry point.
 *
 * Aggregate integration must delegate to this rule.
 */
distributedProcessDeclaration
    : processDeclaration
    | processBinding
    | processInvocation
    | processDependency
    ;


/*
 * ============================================================================
 * 2. PROCESS DECLARATION
 * ============================================================================
 *
 * General forms:
 *
 *     distributed::process compute {
 *         ...
 *     }
 *
 *     distributed::worker worker(input: Data) {
 *         ...
 *     }
 *
 *     distributed::actor actor(state: State) {
 *         ...
 *     }
 *
 * The qualified kind is intentionally open-world.
 */
processDeclaration
    : processKind
      identifier
      processParameterClause?
      processBody
    ;


/*
 * ============================================================================
 * 3. PROCESS KIND
 * ============================================================================
 *
 * The semantic layer determines whether the qualified name denotes:
 *
 *     process
 *     worker
 *     actor
 *     task
 *     agent
 *     pipeline
 *     service_process
 *     quantum_process
 *     hybrid_process
 *     future_process
 *
 * or another registered extension.
 *
 * The grammar deliberately does not maintain a closed enumeration.
 */
processKind
    : qualifiedName
    ;


/*
 * ============================================================================
 * 4. PROCESS PARAMETERS
 * ============================================================================
 *
 * Parameters are ordinary Zamani parameter-like bindings.
 *
 * This grammar intentionally keeps the syntax minimal and delegates richer
 * type/generic semantics to the canonical type subsystem.
 *
 * Examples:
 *
 *     process(input: Data)
 *
 *     process(input: Data, count: Count)
 *
 *     process(input)
 *
 * Parameter cardinality is not bounded.
 */
processParameterClause
    : LPAREN processParameterList? RPAREN
    ;


processParameterList
    : processParameter
      (
          COMMA
          processParameter
      )*
      COMMA?
    ;


processParameter
    : identifier
      processParameterType?
      processParameterInitializer?
    ;


processParameterType
    : COLON
      typeExpression
    ;


processParameterInitializer
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * 5. PROCESS BODY
 * ============================================================================
 *
 * The process body is a recursive process-local syntax scope.
 *
 * It does not duplicate the entire statement grammar.
 *
 * Ordinary Zamani statements should be composed by the canonical parser at
 * the appropriate integration boundary.
 */
processBody
    : LBRACE
      processMember*
      RBRACE
    ;


processMember
    : processBinding
    | processInvocation
    | processDependency
    | processRelationship
    | processLifecycleOperation
    | processNestedDeclaration
    | processScopedOperation
    ;


/*
 * ============================================================================
 * 6. PROCESS BINDINGS
 * ============================================================================
 *
 * General form:
 *
 *     distributed::value result = expression;
 *
 * The qualified binding kind is semantic metadata.
 *
 * It does not define a second type system.
 */
processBinding
    : processBindingKind
      identifier
      ASSIGN
      expression
      SEMICOLON
    ;


processBindingKind
    : qualifiedName
    ;


/*
 * ============================================================================
 * 7. PROCESS INVOCATION
 * ============================================================================
 *
 * General forms:
 *
 *     compute();
 *
 *     distributed::compute(value);
 *
 *     worker::run(input, configuration);
 *
 * Invocation target identity remains semantic.
 *
 * The parser does not decide whether invocation is:
 *
 *     local
 *     remote
 *     replicated
 *     migrated
 *     accelerated
 *     quantum
 *     heterogeneous
 */
processInvocation
    : processCallableName
      LPAREN
      optionalExpressionList
      RPAREN
      SEMICOLON
    ;


processCallableName
    : qualifiedName
    ;


/*
 * ============================================================================
 * 8. PROCESS DEPENDENCY
 * ============================================================================
 *
 * The dependency arrow is semantic ordering/data/control intent.
 *
 * It is NOT physical topology.
 */
processDependency
    : qualifiedName
      THIN_ARROW
      qualifiedName
      SEMICOLON
    ;


/*
 * ============================================================================
 * 9. PROCESS RELATIONSHIPS
 * ============================================================================
 *
 * General form:
 *
 *     distributed::depends_on(a, b);
 *
 *     distributed::member_of(process, group);
 *
 *     distributed::associated_with(process, resource);
 *
 * The operation identity is open-world.
 */
processRelationship
    : qualifiedName
      LPAREN
      expressionList
      RPAREN
      SEMICOLON
    ;


/*
 * ============================================================================
 * 10. PROCESS LIFECYCLE OPERATIONS
 * ============================================================================
 *
 * Lifecycle operations use the same open-world operation representation.
 *
 * Examples:
 *
 *     distributed::start(process);
 *     distributed::stop(process);
 *     distributed::pause(process);
 *     distributed::resume(process);
 *     distributed::restart(process);
 *     distributed::migrate(process, target);
 *     distributed::checkpoint(process);
 *     distributed::recover(process);
 *
 * The grammar does not implement any lifecycle behavior.
 */
processLifecycleOperation
    : processLifecycleName
      LPAREN
      optionalExpressionList
      RPAREN
      SEMICOLON
    ;


processLifecycleName
    : qualifiedName
    ;


/*
 * ============================================================================
 * 11. NESTED PROCESS DECLARATIONS
 * ============================================================================
 *
 * Nested process declarations permit hierarchical logical computation.
 *
 * No finite nesting depth is encoded.
 */
processNestedDeclaration
    : processDeclaration
    ;


/*
 * ============================================================================
 * 12. SCOPED PROCESS OPERATIONS
 * ============================================================================
 *
 * General form:
 *
 *     distributed::scope {
 *         ...
 *     }
 *
 * The operation name remains open-world.
 *
 * Examples include semantic concepts such as:
 *
 *     distributed::parallel
 *     distributed::pipeline
 *     distributed::transaction
 *     distributed::coordination
 *     distributed::region
 *     distributed::execution_scope
 *
 * The grammar does not assign implementation semantics to these names.
 */
processScopedOperation
    : processScopeName
      processBody
    ;


processScopeName
    : qualifiedName
    ;


/*
 * ============================================================================
 * 13. OPTIONAL EXPRESSION LIST
 * ============================================================================
 *
 * This wrapper exists only to make zero-argument invocations explicit.
 *
 * The underlying expression grammar remains authoritative.
 */
optionalExpressionList
    : expressionList?
    ;


/*
 * ============================================================================
 * 14. INTEGRATION ASSERTIONS
 * ============================================================================
 *
 * The following concepts intentionally DO NOT appear as rules in this file:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     expressionList
 *     typeExpression
 *
 * They are imported from the canonical shared grammar components.
 *
 * Likewise, this grammar intentionally does not define:
 *
 *     NODE
 *     PROCESS
 *     WORKER
 *     ACTOR
 *     TASK
 *     SERVICE
 *     CPU
 *     GPU
 *     FPGA
 *     QPU
 *     THREAD
 *     MEMORY
 *     NETWORK
 *
 * as lexical tokens.
 *
 * ============================================================================
 * 15. HARD-CODING AUDIT
 * ============================================================================
 *
 * No language-level capacity constants are defined.
 *
 * Forbidden concepts intentionally absent:
 *
 *     MAX_NODES
 *     MAX_PROCESSES
 *     MAX_WORKERS
 *     MAX_TASKS
 *     MAX_THREADS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_MEMORY
 *     MAX_DEVICES
 *     MAX_CHANNELS
 *     MAX_PROCESS_DEPTH
 *     MAX_PROCESS_ARGUMENTS
 *
 * No physical resource identifier is required.
 *
 * No vendor-specific implementation is required.
 *
 * ============================================================================
 * 16. DOWNSTREAM CONTRACT
 * ============================================================================
 *
 * This grammar ends at syntax.
 *
 * Downstream processing must perform:
 *
 *     name resolution
 *          |
 *          v
 *     type analysis
 *          |
 *          v
 *     effect analysis
 *          |
 *          v
 *     capability analysis
 *          |
 *          v
 *     resource analysis
 *          |
 *          v
 *     distributed semantic validation
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical representation
 *          +--> quantum::ir
 *          +--> HDL/hardware representation
 *          +--> distributed execution metadata
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     placement
 *          |
 *          v
 *     routing
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     deployment/runtime
 *
 * ============================================================================
 * 17. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] It has exactly one public process declaration entry point.
 *     [x] It uses the canonical ZamaniLexer.
 *     [x] It imports canonical Names and Expressions grammars.
 *     [x] It does not redefine shared lexical/parser concepts.
 *     [x] Process kinds are open-world qualified names.
 *     [x] Process names are canonical identifiers.
 *     [x] Process parameters are unbounded.
 *     [x] Process members are unbounded.
 *     [x] Nested process declarations are supported.
 *     [x] Process invocations are supported.
 *     [x] Process dependencies are supported.
 *     [x] Process relationships are supported.
 *     [x] Lifecycle intent is supported.
 *     [x] Process scope composition is supported.
 *     [x] No hardware capacity is hard-coded.
 *     [x] No topology is hard-coded.
 *     [x] No transport is hard-coded.
 *     [x] No scheduler is hard-coded.
 *     [x] No deployment provider is hard-coded.
 *     [x] No quantum gate set is hard-coded.
 *     [x] No quantum IR is duplicated.
 *     [x] Classical/quantum/HDL integration remains downstream.
 *     [x] Safe Rust compatibility is preserved because this file contains
 *         no target-language actions.
 *
 * Repository integration still requires the aggregate parser to import this
 * grammar and delegate `distributedProcessDeclaration` to it. That integration
 * belongs to the aggregate grammar and is deliberately not hidden inside this
 * independent file.
 *
 * ============================================================================
 */
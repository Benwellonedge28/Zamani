/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/concurrency/actors.g4
 *
 * GRAMMAR
 * -------
 * Actors
 *
 * STATUS
 * ------
 * PRODUCTION-READY SOURCE-GRAMMAR CONTRACT
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the canonical parser-level owner of Zamani actor syntax.
 *
 * An actor is a logical concurrent computation boundary.
 *
 * An actor is NOT inherently:
 *
 *     - a thread;
 *     - a process;
 *     - a CPU;
 *     - a core;
 *     - a GPU;
 *     - an FPGA;
 *     - an ASIC;
 *     - a QPU;
 *     - an accelerator;
 *     - a network endpoint;
 *     - a distributed node;
 *     - an operating-system process;
 *     - a physical device;
 *     - a scheduler;
 *     - a runtime object.
 *
 * Physical realization is determined after parsing by semantic analysis,
 * resource negotiation, capability resolution, compilation, scheduling,
 * deployment and runtime/target layers.
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * Source actor syntax describes:
 *
 *     WHAT
 *
 * rather than:
 *
 *     WHERE
 *     HOW MANY
 *     ON WHICH DEVICE
 *     ON WHICH CPU
 *     ON WHICH NODE
 *     WITH WHICH THREAD
 *     WITH WHICH PHYSICAL QUEUE
 *
 * This is required for:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * (POCO-REAF).
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     actorConstruct
 *     actorDeclaration
 *     actorDeclarationPrefix
 *     actorInheritanceClause
 *     actorBody
 *     actorMember
 *     actorMemberPrefix
 *     actorStateField
 *     actorHandler
 *     actorHandlerReturnType
 *     actorSpawnExpression
 *     actorSpawnStatement
 *     actorArguments
 *     actorCommand
 *     actorCommandStatement
 *     actorCommandName
 *     actorCommandPayload
 *     actorMessagePayload
 *     actorLifecyclePayload
 *     actorSupervisionPayload
 *     actorSendExpression
 *     actorSendStatement
 *     actorAskExpression
 *     actorAskStatement
 *     actorForwardExpression
 *     actorForwardStatement
 *     actorLifecycleExpression
 *     actorLifecycleStatement
 *     actorSupervisionConstruct
 *     actorSupervisionStatement
 *     actorMessageTarget
 *     actorTarget
 *     actorTargetPath
 *     actorMessageName
 *     actorMessageInvocation
 *     actorStatement
 *     actorExpression
 *
 * THIS FILE DOES NOT OWN:
 *
 *     identifiers
 *     names
 *     qualified names
 *     expressions
 *     types
 *     blocks
 *     attributes
 *     modifiers
 *     visibility
 *     parameters
 *     generic parameter syntax
 *     channels
 *     futures
 *     tasks
 *     async/await
 *     parallelism
 *     cancellation
 *     synchronization
 *     distributed placement
 *     networking
 *     resource requirements
 *     capability negotiation
 *     effects
 *     policies
 *     contracts
 *     provenance semantics
 *     quantum semantics
 *     quantum::ir
 *     HDL semantics
 *     hardware realization
 *     classical IR
 *     runtime implementation
 *     scheduling
 *     routing
 *     QEC
 *     ZQN
 *     HAL
 *
 * Those concerns remain owned by their canonical subsystems.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/lexer/tokens.g4
 *     grammar/core/names.g4
 *     grammar/types/*
 *     grammar/expressions/*
 *     grammar/core/blocks.g4
 *     grammar/core/attributes.g4
 *     grammar/core/modifiers.g4
 *     grammar/core/visibility.g4
 *     grammar/functions/parameters.g4
 *
 * IMPORTED GRAMMARS:
 *
 *     Names
 *     Types
 *     Expressions
 *     ZamaniCoreBlocks
 *     Attributes
 *     Modifiers
 *     Visibility
 *     Parameters
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * This grammar consumes the canonical lexer vocabulary.
 *
 * Required existing tokens include:
 *
 *     ACTOR
 *     SPAWN
 *     EXTENDS
 *     SELF
 *     LPAREN
 *     RPAREN
 *     LBRACE
 *     RBRACE
 *     COMMA
 *     COLON
 *     DOT
 *     SEMICOLON
 *     THIN_ARROW
 *     DOUBLE_COLON
 *     ASSIGN
 *     IDENTIFIER
 *
 * Actor command names such as:
 *
 *     receive
 *     send
 *     ask
 *     forward
 *     stop
 *     restart
 *     supervise
 *
 * intentionally remain ordinary identifiers.
 *
 * The current canonical lexer does not reserve these words as dedicated
 * actor-command tokens.
 *
 * This is deliberate.
 *
 * It prevents the core actor grammar from acquiring a closed finite catalogue
 * of actor commands and permits future semantic command extensions without
 * requiring a universal lexer expansion.
 *
 * Semantic analysis classifies actorCommandName.
 *
 * ============================================================================
 * WHY COMMANDS ARE IDENTIFIERS
 * ============================================================================
 *
 * The actor model is extensible.
 *
 * Therefore:
 *
 *     actor send ...
 *     actor ask ...
 *     actor forward ...
 *     actor stop ...
 *     actor restart ...
 *     actor supervise ...
 *
 * are syntactically represented as:
 *
 *     ACTOR IDENTIFIER ...
 *
 * rather than:
 *
 *     ACTOR SEND ...
 *     ACTOR ASK ...
 *     ACTOR FORWARD ...
 *
 * This avoids a hard-coded actor-command universe.
 *
 * The semantic layer owns the recognized standard command vocabulary and can
 * diagnose unknown commands without making the parser depend on an exhaustive
 * future command list.
 *
 * ============================================================================
 * EXPORT CONTRACT
 * ============================================================================
 *
 * PRIMARY PUBLIC ENTRY:
 *
 *     actorConstruct
 *
 * CONCURRENCY COMPOSITION:
 *
 *     grammar/concurrency/concurrency.g4
 *
 * consumes:
 *
 *     actorConstruct
 *
 * COMPATIBILITY / TOOLING FACADES:
 *
 *     actorDeclaration
 *     actorSpawnExpression
 *     actorCommand
 *     actorSupervisionConstruct
 *     actorStatement
 *     actorExpression
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Parsing must preserve enough source structure for the existing domain-neutral
 * frontend AST to represent:
 *
 *     actor declaration
 *     actor modifiers
 *     actor visibility
 *     actor attributes
 *     actor name
 *     actor generic parameters
 *     actor inheritance
 *     actor members
 *     actor state
 *     state type
 *     state initializer
 *     handler command marker
 *     handler message name
 *     handler parameters
 *     handler return type
 *     handler body
 *     actor construction
 *     actor construction type
 *     actor construction arguments
 *     actor command
 *     command name
 *     message target
 *     message name
 *     message arguments
 *     lifecycle target
 *     supervision target
 *     supervision body
 *     source spans
 *
 * The parser MUST NOT construct runtime objects such as:
 *
 *     ActorId
 *     Mailbox
 *     WorkerId
 *     ThreadHandle
 *     ProcessHandle
 *     Executor
 *     Scheduler
 *     PhysicalNode
 *     DeviceId
 *
 * Those belong downstream.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     - whether an actor declaration is valid;
 *     - whether actor inheritance is legal;
 *     - whether actor state is isolated;
 *     - whether state is mutable;
 *     - whether a handler is a valid actor handler;
 *     - whether the command name is a recognized actor operation;
 *     - whether a target resolves to an actor;
 *     - whether a message exists;
 *     - whether message arguments satisfy the message contract;
 *     - whether an ask operation has a valid result;
 *     - whether forwarding is legal;
 *     - whether lifecycle control is authorized;
 *     - whether supervision is valid;
 *     - whether ownership rules are satisfied;
 *     - whether effects are permitted;
 *     - whether capabilities are available;
 *     - whether resource requirements are satisfiable;
 *     - whether policies permit the operation;
 *     - whether distributed realization is possible;
 *     - whether execution satisfies determinism requirements;
 *     - whether recovery semantics are valid.
 *
 * ============================================================================
 * STANDARD COMMAND SEMANTICS
 * ============================================================================
 *
 * The following command names are reserved by semantic convention:
 *
 *     receive
 *         actor handler declaration
 *
 *     send
 *         asynchronous actor message intent
 *
 *     ask
 *         request/response actor message intent
 *
 *     forward
 *         message forwarding intent
 *
 *     stop
 *         logical actor lifecycle intent
 *
 *     restart
 *         logical actor lifecycle intent
 *
 *     supervise
 *         actor supervision intent
 *
 * The parser does NOT enforce these names.
 *
 * Semantic validation does.
 *
 * This distinction is important because syntax remains extensible while
 * standard language semantics remain precise.
 *
 * ============================================================================
 * SOURCE FORMS
 * ============================================================================
 *
 * Actor declaration:
 *
 *     actor Counter {
 *         value: Int;
 *
 *         actor receive increment(amount: Int) {
 *             value = value + amount;
 *         }
 *     }
 *
 * Actor declaration with inheritance:
 *
 *     actor Counter extends BaseCounter {
 *         value: Int;
 *     }
 *
 * Actor construction:
 *
 *     spawn actor Counter(0);
 *
 * Message:
 *
 *     actor send counter.increment(1);
 *
 * Request/response:
 *
 *     actor ask counter.value();
 *
 * Forwarding:
 *
 *     actor forward worker.process(value);
 *
 * Lifecycle:
 *
 *     actor stop child;
 *
 *     actor restart child;
 *
 * Supervision:
 *
 *     actor supervise child {
 *         recover();
 *     }
 *
 * Dynamic actor target:
 *
 *     actor send (resolve_actor()).message(value);
 *
 * Qualified actor target:
 *
 *     actor send service::worker.process(value);
 *
 * ============================================================================
 * IMPORTANT PARSER CORRECTION
 * ============================================================================
 *
 * The previous architecture represented send, ask, forward and lifecycle
 * payloads with effectively identical grammar alternatives.
 *
 * That created redundant alternatives such as:
 *
 *     actorSendPayload
 *     actorAskPayload
 *     actorForwardPayload
 *
 * where all three consumed the same structure.
 *
 * That does not provide useful structural separation.
 *
 * This production version instead factors actor commands through:
 *
 *     actorCommand
 *         |
 *         +--> actorMessagePayload
 *         |
 *         +--> actorLifecyclePayload
 *         |
 *         +--> actorSupervisionPayload
 *
 * A message payload is structurally identified by:
 *
 *     target . message(arguments)
 *
 * A lifecycle payload is structurally identified by:
 *
 *     target
 *
 * A supervision payload is structurally identified by:
 *
 *     target block
 *
 * The command name remains semantic.
 *
 * This gives the parser useful structure without creating dedicated lexer
 * tokens for every actor command.
 *
 * ============================================================================
 * TARGET MODEL
 * ============================================================================
 *
 * Actor target paths use `::` for qualification.
 *
 * Message selection uses `.`.
 *
 * Therefore:
 *
 *     service::worker.process(value)
 *
 * parses conceptually as:
 *
 *     actorTargetPath
 *         service::worker
 *
 *     DOT
 *
 *     actorMessageName
 *         process
 *
 * This prevents a qualified name from accidentally consuming the message
 * selector.
 *
 * ============================================================================
 * ACTOR TARGETS
 * ============================================================================
 *
 * Static target:
 *
 *     counter
 *
 * Qualified target:
 *
 *     service::counter
 *
 * Deeply qualified target:
 *
 *     domain::service::counter
 *
 * Self target:
 *
 *     self
 *
 * Qualified self target:
 *
 *     self::child
 *
 * Dynamic target:
 *
 *     (expression)
 *
 * The grammar does not encode what the target physically represents.
 *
 * ============================================================================
 * TARGET INVARIANT
 * ============================================================================
 *
 * actorTarget MUST NOT encode:
 *
 *     CPU IDs
 *     GPU IDs
 *     FPGA IDs
 *     QPU IDs
 *     node IDs
 *     process IDs
 *     thread IDs
 *     physical addresses
 *     device IDs
 *     transport IDs
 *     topology coordinates
 *
 * Those concepts may be values in a target-specific semantic model, but they
 * are not universal actor syntax.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar imposes no universal finite limit on:
 *
 *     actors
 *     actor declarations
 *     actor members
 *     handlers
 *     state fields
 *     messages
 *     message arguments
 *     actor instances
 *     supervisors
 *     supervised actors
 *     nesting
 *     target qualification depth
 *     concurrent actors
 *     logical communication relationships
 *
 * ANTLR repetition operators are used instead of finite enumeration.
 *
 * "Infinity" means:
 *
 *     this grammar introduces no artificial finite language ceiling.
 *
 * Actual execution remains subject to:
 *
 *     source size
 *     compiler resources
 *     runtime resources
 *     target capabilities
 *     deployment constraints
 *     semantic feasibility
 *
 * Those constraints MUST NOT become grammar-level capacity constants.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This file MUST NOT define:
 *
 *     MAX_ACTORS
 *     MAX_MESSAGES
 *     MAX_HANDLERS
 *     MAX_CHILDREN
 *     MAX_SUPERVISORS
 *     MAX_MAILBOXES
 *     MAX_THREADS
 *     MAX_WORKERS
 *     MAX_CORES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_QUEUE_DEPTH
 *     MAX_DEVICES
 *
 * Nor may it encode:
 *
 *     cpu0
 *     gpu0
 *     qpu0
 *     node0
 *     core0
 *     worker0
 *
 * as language-level special cases.
 *
 * Numeric values appearing in actor programs are ordinary source values.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Actors express logical computation.
 *
 * Physical resource requirements remain owned by:
 *
 *     grammar/resources/
 *
 * including:
 *
 *     requirements
 *     capabilities
 *     constraints
 *     preferences
 *     hints
 *     negotiation
 *     scaling
 *
 * Conceptually:
 *
 *     requires capability("parallel.compute");
 *
 *     requires capability("distributed.actor");
 *
 *     requires capability("message.passing");
 *
 * Actor syntax does not evaluate or resolve these requirements.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Actor operations may acquire semantic effects such as:
 *
 *     mutation
 *     IO
 *     network
 *     distributed
 *     measurement
 *     native
 *     foreign
 *     randomness
 *     learning
 *     adaptation
 *
 * The grammar does not assign physical effects.
 *
 * For example:
 *
 *     actor send ...
 *
 * does not inherently mean:
 *
 *     network IO
 *
 * because the actor may be realized locally, through shared memory, through a
 * runtime transport, through an accelerator interconnect, or through another
 * implementation.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Actor syntax does not grant capabilities.
 *
 * These are source operations:
 *
 *     spawn
 *     send
 *     ask
 *     forward
 *     stop
 *     restart
 *     supervise
 *
 * Authorization is downstream.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Actor operations may be constrained by:
 *
 *     security policies
 *     execution policies
 *     resource policies
 *     scheduling policies
 *     deployment policies
 *     recovery policies
 *     adaptation policies
 *     reproducibility policies
 *
 * Policy evaluation is not parser behavior.
 *
 * ============================================================================
 * CONTRACT / VALIDATION CONTRACT
 * ============================================================================
 *
 * Actor declarations and operations may participate in the universal:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *     assert
 *
 * systems.
 *
 * This grammar deliberately does not duplicate contract syntax.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Source spans must remain available for:
 *
 *     actor declaration
 *     handler
 *     state
 *     construction
 *     command
 *     target
 *     message
 *     supervision
 *
 * Downstream provenance may then associate:
 *
 *     source
 *       ->
 *     AST
 *       ->
 *     semantic actor model
 *       ->
 *     optimization
 *       ->
 *     lowering
 *       ->
 *     scheduling
 *       ->
 *     deployment
 *       ->
 *     runtime realization
 *
 * with the original source construct.
 *
 * ============================================================================
 * CHANNEL INTEGRATION
 * ============================================================================
 *
 * Actors do not redefine channels.
 *
 * Channel syntax remains owned by:
 *
 *     grammar/concurrency/channels.g4
 *
 * An actor message may eventually be implemented using:
 *
 *     a channel
 *     a queue
 *     shared memory
 *     an event mechanism
 *     a local transport
 *     a distributed transport
 *     another communication substrate
 *
 * without changing actor source syntax.
 *
 * ============================================================================
 * TASK / FUTURE INTEGRATION
 * ============================================================================
 *
 * Actor realization may use:
 *
 *     task
 *     future
 *     async execution
 *     event loop
 *     thread
 *     process
 *     service
 *     distributed executor
 *     another execution substrate
 *
 * These are implementation choices.
 *
 * This grammar does not define a competing task or future model.
 *
 * ============================================================================
 * CANCELLATION / SYNCHRONIZATION INTEGRATION
 * ============================================================================
 *
 * Actor commands may participate in cancellation and synchronization
 * semantics, but those grammars remain independently owned by:
 *
 *     grammar/concurrency/cancellation.g4
 *     grammar/concurrency/synchronization.g4
 *
 * This file does not duplicate those constructs.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * A logical actor may eventually be:
 *
 *     local
 *     remote
 *     migrated
 *     replicated
 *     partitioned
 *     heterogeneous
 *
 * without changing actor syntax.
 *
 * Placement belongs downstream to:
 *
 *     grammar/distributed/
 *     grammar/networking/
 *     grammar/resources/
 *     execution/deployment
 *     semantic analysis
 *
 * `actor send` therefore does NOT mean "send a network packet".
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Actor state and message payloads may contain quantum-domain values where
 * permitted by the type and semantic systems.
 *
 * This grammar does not define:
 *
 *     qubits
 *     physical qubits
 *     quantum gates
 *     gate inventories
 *     quantum topology
 *     routing
 *     calibration
 *     QEC
 *     ZQN
 *     HAL
 *
 * Quantum semantics continue through:
 *
 *     source
 *       ->
 *     domain-neutral AST
 *       ->
 *     semantic model
 *       ->
 *     quantum::ir
 *       ->
 *     optimization
 *       ->
 *     decomposition
 *       ->
 *     routing
 *       ->
 *     scheduling
 *       ->
 *     resilience / QEC
 *       ->
 *     ZQN
 *       ->
 *     HAL
 *       ->
 *     target
 *
 * No actor-specific quantum IR is introduced.
 *
 * ============================================================================
 * CLASSICAL / AI / DATA / HDL INTEGRATION
 * ============================================================================
 *
 * Actor handler bodies use ordinary Zamani expressions and blocks.
 *
 * Therefore handlers may semantically contain:
 *
 *     classical computation
 *     numerical computation
 *     data processing
 *     tensor computation
 *     reasoning
 *     knowledge operations
 *     learning
 *     adaptation
 *     uncertainty
 *     hybrid computation
 *     quantum computation
 *     HDL-related operations
 *     hardware intent
 *
 * No actor-specific grammar is required for those domains.
 *
 * ============================================================================
 * DOMAIN-NEUTRALITY
 * ============================================================================
 *
 * The actor grammar does not distinguish:
 *
 *     CPU actor
 *     GPU actor
 *     QPU actor
 *     AI actor
 *     HDL actor
 *     network actor
 *
 * An actor is a universal concurrency abstraction.
 *
 * Domain meaning is determined by the declarations, types, expressions,
 * effects, capabilities and semantic model contained within the actor.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file introduces NO actor-specific IR.
 *
 * Parsed actors lower into the repository's domain-neutral AST and semantic
 * representation.
 *
 * Conceptually:
 *
 *     Operation {
 *         name,
 *         namespace,
 *         operands,
 *         parameters,
 *         results,
 *         attributes,
 *         modifiers,
 *         effects,
 *         capabilities,
 *         source
 *     }
 *
 * Actor-specific semantic information may be represented as attributes or
 * semantic structures downstream.
 *
 * It must not become a second universal IR.
 *
 * ============================================================================
 * COMPILER / RUNTIME BOUNDARY
 * ============================================================================
 *
 * This grammar stops at source structure.
 *
 * Compiler/runtime components determine:
 *
 *     actor allocation
 *     mailbox implementation
 *     message transport
 *     scheduling
 *     worker allocation
 *     process placement
 *     accelerator use
 *     distributed placement
 *     migration
 *     replication
 *     recovery
 *     persistence
 *
 * No such behavior is encoded here.
 *
 * ============================================================================
 * SAFETY CONTRACT
 * ============================================================================
 *
 * This file contains:
 *
 *     no embedded Rust;
 *     no parser actions;
 *     no semantic predicates;
 *     no filesystem access;
 *     no network access;
 *     no hardware inspection;
 *     no runtime allocation;
 *     no resource allocation;
 *     no target selection.
 *
 * Generated frontend code remains compatible with:
 *
 *     Rust 1.97+
 *     Rust Edition 2021
 *
 * and requires safe Rust only.
 *
 * No `unsafe` implementation is required.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing depends only upon:
 *
 *     source token stream
 *     imported grammar rules
 *     parser configuration
 *
 * Parsing MUST NOT depend upon:
 *
 *     CPU count
 *     GPU availability
 *     QPU availability
 *     node count
 *     memory availability
 *     runtime state
 *     scheduler state
 *     network state
 *     wall-clock time
 *     randomness
 *     target availability
 *
 * Identical source and parser configuration must produce equivalent parse
 * structures.
 *
 * ============================================================================
 * ERROR BOUNDARY
 * ============================================================================
 *
 * Parser diagnostics identify structural errors such as:
 *
 *     malformed actor declaration
 *     missing actor name
 *     malformed inheritance
 *     missing actor body
 *     malformed state declaration
 *     malformed handler
 *     malformed parameter list
 *     malformed spawn expression
 *     malformed message invocation
 *     missing message selector
 *     missing invocation parenthesis
 *     malformed lifecycle target
 *     malformed supervision block
 *
 * Semantic diagnostics identify:
 *
 *     unknown actor command
 *     unknown actor target
 *     unknown message
 *     invalid message arguments
 *     invalid actor inheritance
 *     invalid state access
 *     invalid lifecycle authority
 *     invalid supervision relationship
 *     unsatisfied capability
 *     unsatisfied resource requirement
 *     forbidden effect
 *     policy violation
 *     unsupported target realization
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Required positive tests:
 *
 *     actor Counter {
 *         value: Int;
 *     }
 *
 *     actor Counter {
 *         value: Int;
 *
 *         actor receive increment(amount: Int) {
 *             value = value + amount;
 *         }
 *     }
 *
 *     actor Counter extends BaseCounter {
 *         value: Int;
 *     }
 *
 *     spawn actor Counter(0);
 *
 *     actor send counter.increment(1);
 *
 *     actor ask counter.value();
 *
 *     actor forward worker.process(value);
 *
 *     actor stop child;
 *
 *     actor restart child;
 *
 *     actor supervise child {
 *         recover();
 *     }
 *
 *     actor send service::worker.process(value);
 *
 *     actor send self::child.process(value);
 *
 *     actor send (resolve_actor()).process(value);
 *
 * Required command compatibility tests:
 *
 *     actor receive ...
 *     actor send ...
 *     actor ask ...
 *     actor forward ...
 *     actor stop ...
 *     actor restart ...
 *     actor supervise ...
 *
 * Required cross-domain tests:
 *
 *     actor containing classical computation
 *     actor containing data operations
 *     actor containing reasoning
 *     actor containing learning
 *     actor containing adaptation
 *     actor containing tensor operations
 *     actor containing quantum operations
 *     actor containing hybrid computation
 *     actor containing HDL/hardware intent
 *     actor containing distributed operations
 *
 * Required resource tests:
 *
 *     actor + capability requirement
 *     actor + resource requirement
 *     actor + policy
 *     actor + effect declaration
 *     actor + contract
 *
 * Required negative syntax tests:
 *
 *     actor
 *     actor {
 *     actor Name
 *     actor Name {
 *     actor Name { field; }
 *     actor receive
 *     actor send
 *     actor ask
 *     actor forward
 *     actor stop
 *     actor restart
 *     actor supervise
 *     spawn
 *     spawn actor
 *     spawn actor Name(
 *     actor send target
 *     actor send target.
 *     actor send target.message(
 *     actor supervise target
 *
 * Required scalability tests:
 *
 *     many actor declarations
 *     many actor members
 *     many handlers
 *     many state fields
 *     deeply qualified logical targets
 *     many message arguments
 *     deeply nested handler blocks
 *     many supervision relationships
 *
 * No test may introduce an artificial actor count or resource ceiling.
 *
 * Required determinism tests:
 *
 *     identical token stream
 *         ->
 *     equivalent parse structure
 *
 * Required portability tests:
 *
 *     same actor source
 *         ->
 *     syntactically valid
 *
 * regardless of:
 *
 *     processor count
 *     accelerator count
 *     QPU availability
 *     node count
 *     memory size
 *     topology
 *     deployment environment
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * PASS CONDITIONS:
 *
 *     no MAX_ACTORS
 *     no MAX_MESSAGES
 *     no MAX_HANDLERS
 *     no MAX_CHILDREN
 *     no MAX_SUPERVISORS
 *     no MAX_MAILBOXES
 *     no MAX_THREADS
 *     no MAX_WORKERS
 *     no MAX_CORES
 *     no MAX_CPUS
 *     no MAX_GPUS
 *     no MAX_FPGAS
 *     no MAX_QPUS
 *     no MAX_NODES
 *     no MAX_MEMORY
 *     no MAX_QUEUE_DEPTH
 *     no MAX_DEVICES
 *     no physical placement
 *     no hardware topology
 *     no fixed actor capacity
 *     no fixed mailbox capacity
 *     no runtime allocation
 *     no scheduler implementation
 *     no actor-specific IR
 *     no embedded Rust
 *     no unsafe Rust
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * 1. LEXER
 * ---------------------------------------------------------------------------
 *
 * Canonical lexical source:
 *
 *     grammar/lexer/tokens.g4
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This grammar consumes canonical emitted tokens.
 *
 *
 * 2. CONCURRENCY COMPOSITION
 * ---------------------------------------------------------------------------
 *
 * grammar/concurrency/concurrency.g4
 *
 * consumes:
 *
 *     actorConstruct
 *
 * It MUST NOT duplicate actor syntax.
 *
 *
 * 3. UNIVERSAL STATEMENT DISPATCH
 * ---------------------------------------------------------------------------
 *
 * The universal statement layer may consume:
 *
 *     actorStatement
 *
 * through the established concurrency composition boundary.
 *
 * It must not recreate actor declarations or commands.
 *
 *
 * 4. EXPRESSIONS
 * ---------------------------------------------------------------------------
 *
 * Actor handler bodies and actor arguments consume the canonical:
 *
 *     expression
 *
 * and:
 *
 *     blockExpression
 *
 * rules.
 *
 * This prevents actors from becoming a second expression language.
 *
 *
 * 5. TYPES
 * ---------------------------------------------------------------------------
 *
 * State fields and handler return types consume:
 *
 *     typeExpression
 *
 * from the canonical type grammar.
 *
 *
 * 6. PARAMETERS
 * ---------------------------------------------------------------------------
 *
 * Handler parameters consume:
 *
 *     parameterList
 *
 * from the canonical parameter grammar.
 *
 *
 * 7. ATTRIBUTES / MODIFIERS / VISIBILITY
 * ---------------------------------------------------------------------------
 *
 * Actor declarations and members use the canonical:
 *
 *     attribute
 *     modifier
 *     visibilityModifier
 *
 * rules.
 *
 *
 * 8. CHANNELS
 * ---------------------------------------------------------------------------
 *
 * Actor communication does not redefine:
 *
 *     channel
 *     channel send
 *     channel receive
 *     channel close
 *     channel select
 *
 * Those remain owned by:
 *
 *     grammar/concurrency/channels.g4
 *
 *
 * 9. TASKS / FUTURES
 * ---------------------------------------------------------------------------
 *
 * Actor realization may use tasks/futures, but the grammar does not redefine
 * them.
 *
 *
 * 10. DISTRIBUTED
 * ---------------------------------------------------------------------------
 *
 * Distributed actor semantics remain downstream and/or in:
 *
 *     grammar/distributed/
 *     grammar/networking/
 *
 * This file contains no physical placement syntax.
 *
 *
 * 11. RESOURCES / CAPABILITIES
 * ---------------------------------------------------------------------------
 *
 * Resource requirements remain owned by:
 *
 *     grammar/resources/
 *
 * Actor grammar does not duplicate `requires`, `capability`, resource
 * negotiation or scaling syntax.
 *
 *
 * 12. EFFECTS
 * ---------------------------------------------------------------------------
 *
 * Effects remain owned by:
 *
 *     grammar/effects/
 *
 * Actor commands may acquire effects semantically.
 *
 *
 * 13. POLICIES
 * ---------------------------------------------------------------------------
 *
 * Policies remain owned by the policy/security/execution subsystems.
 *
 *
 * 14. VALIDATION / CONTRACTS
 * ---------------------------------------------------------------------------
 *
 * Contract syntax remains owned by:
 *
 *     grammar/validation/
 *     grammar/statements/
 *     grammar/functions/
 *
 * Actor syntax only provides source constructs that can become contract
 * subjects.
 *
 *
 * 15. PROVENANCE
 * ---------------------------------------------------------------------------
 *
 * Source spans and AST structure feed the repository's provenance system.
 *
 * Actor grammar does not implement provenance storage.
 *
 *
 * 16. QUANTUM
 * ---------------------------------------------------------------------------
 *
 * Quantum semantics remain outside this grammar and ultimately use:
 *
 *     quantum::ir
 *
 * Actor syntax MUST NOT create an actor-specific quantum representation.
 *
 *
 * 17. HDL / HARDWARE
 * ---------------------------------------------------------------------------
 *
 * Hardware realization remains downstream.
 *
 * Actor syntax does not encode:
 *
 *     register widths
 *     bus widths
 *     device counts
 *     FPGA dimensions
 *     CPU counts
 *     GPU counts
 *     QPU counts
 *
 *
 * 18. RUST
 * ---------------------------------------------------------------------------
 *
 * Grammar generation and frontend implementation remain compatible with:
 *
 *     Rust 1.97+
 *     Rust Edition 2021
 *
 * and safe Rust only.
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar Actors;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names,
    Types,
    Expressions,
    ZamaniCoreBlocks,
    Attributes,
    Modifiers,
    Visibility,
    Parameters
    ;


/* ============================================================================
 * PUBLIC COMPOSITION
 * ========================================================================== */

/*
 * Single canonical actor composition root.
 *
 * `concurrency.g4` consumes this rule.
 */
actorConstruct
    : actorDeclaration
    | actorSpawnExpression
    | actorCommand
    ;


/* ============================================================================
 * ACTOR DECLARATION
 * ========================================================================== */

actorDeclaration
    : actorDeclarationPrefix*
      ACTOR
      identifier
      actorGenericParameters?
      actorInheritanceClause?
      actorBody
    ;


actorDeclarationPrefix
    : attribute
    | visibilityModifier
    | modifier
    ;


actorGenericParameters
    : genericParameterClause
    ;


actorInheritanceClause
    : EXTENDS
      typeExpression
      (
          COMMA
          typeExpression
      )*
    ;


actorBody
    : LBRACE
      actorMember*
      RBRACE
    ;


actorMember
    : actorMemberPrefix*
      actorStateField
    | actorMemberPrefix*
      actorHandler
    ;


actorMemberPrefix
    : attribute
    | visibilityModifier
    | modifier
    ;


/* ============================================================================
 * ACTOR STATE
 * ========================================================================== */

actorStateField
    : identifier
      COLON
      typeExpression
      (
          ASSIGN
          expression
      )?
      SEMICOLON
    ;


/* ============================================================================
 * ACTOR HANDLER
 * ========================================================================== */

/*
 * Handler syntax deliberately uses an identifier for the command name.
 *
 * Canonical semantic convention:
 *
 *     actor receive message(...)
 *
 * The parser preserves the command marker and message name separately.
 */
actorHandler
    : ACTOR
      actorCommandName
      actorMessageName
      LPAREN
      parameterList?
      RPAREN
      actorHandlerReturnType?
      blockExpression
    ;


actorHandlerReturnType
    : THIN_ARROW
      typeExpression
    ;


/* ============================================================================
 * ACTOR SPAWN
 * ========================================================================== */

/*
 * Logical actor construction.
 *
 * The source says WHAT actor is to be created.
 *
 * The source does not say:
 *
 *     on which CPU
 *     on which node
 *     on which thread
 *     on which device
 *     with which physical executor
 *
 * Those decisions are downstream.
 */
actorSpawnExpression
    : SPAWN
      ACTOR
      qualifiedName
      actorGenericArguments?
      LPAREN
      actorArguments?
      RPAREN
    ;


actorSpawnStatement
    : actorSpawnExpression
      SEMICOLON
    ;


actorArguments
    : argumentList
    ;


actorGenericArguments
    : genericArgumentClause
    ;


/* ============================================================================
 * ACTOR COMMAND ROOT
 * ========================================================================== */

/*
 * Canonical command structure:
 *
 *     actor <command> <payload>
 *
 * The command name remains an identifier.
 *
 * Payload structure is intentionally factored so that:
 *
 *     target.message(arguments)
 *
 *     target
 *
 *     target { ... }
 *
 * are structurally distinct.
 */
actorCommand
    : ACTOR
      actorCommandName
      actorCommandPayload
    ;


actorCommandStatement
    : actorCommand
      SEMICOLON?
    ;


actorCommandName
    : identifier
    ;


actorCommandPayload
    : actorMessagePayload
    | actorSupervisionPayload
    | actorLifecyclePayload
    ;


/* ============================================================================
 * MESSAGE COMMANDS
 * ========================================================================== */

/*
 * Shared structural representation for:
 *
 *     send
 *     ask
 *     forward
 *
 * Semantic analysis determines which operation the command name denotes.
 *
 * Example:
 *
 *     actor send counter.increment(1);
 *
 *     actor ask counter.value();
 *
 *     actor forward worker.process(value);
 */
actorMessagePayload
    : actorMessageInvocation
    ;


actorMessageInvocation
    : actorMessageTarget
      DOT
      actorMessageName
      LPAREN
      actorArguments?
      RPAREN
      SEMICOLON?
    ;


actorMessageTarget
    : actorTarget
    ;


actorMessageName
    : identifier
    ;


/* ============================================================================
 * COMPATIBILITY MESSAGE FACADES
 * ========================================================================== */

/*
 * These rules are compatibility/tooling entry points.
 *
 * They intentionally delegate to the canonical command representation rather
 * than duplicating message grammar.
 *
 * Semantic analysis verifies that actorCommandName is respectively:
 *
 *     send
 *     ask
 *     forward
 *
 * when these facades are used as isolated validation entry points.
 */
actorSendExpression
    : actorCommand
    ;


actorSendStatement
    : actorSendExpression
      SEMICOLON?
    ;


actorAskExpression
    : actorCommand
    ;


actorAskStatement
    : actorAskExpression
      SEMICOLON?
    ;


actorForwardExpression
    : actorCommand
    ;


actorForwardStatement
    : actorForwardExpression
      SEMICOLON?
    ;


/* ============================================================================
 * LIFECYCLE
 * ========================================================================== */

/*
 * Lifecycle commands include semantic operations such as:
 *
 *     stop
 *     restart
 *
 * Structural form:
 *
 *     actor <command> <target>
 *
 * The command name remains semantic.
 */
actorLifecyclePayload
    : actorTarget
      SEMICOLON?
    ;


actorLifecycleExpression
    : actorCommand
    ;


actorLifecycleStatement
    : actorLifecycleExpression
      SEMICOLON?
    ;


/* ============================================================================
 * SUPERVISION
 * ========================================================================== */

/*
 * Structural distinction:
 *
 *     actor supervise target {
 *         ...
 *     }
 *
 * The trailing block makes supervision structurally different from ordinary
 * lifecycle commands.
 */
actorSupervisionPayload
    : actorTarget
      blockExpression
    ;


actorSupervisionConstruct
    : ACTOR
      actorCommandName
      actorSupervisionPayload
    ;


actorSupervisionStatement
    : actorSupervisionConstruct
      SEMICOLON?
    ;


/* ============================================================================
 * TARGETS
 * ========================================================================== */

/*
 * A target can be:
 *
 *     self
 *     name
 *     qualified::name
 *     (expression)
 *
 * No physical resource identity is represented here.
 */
actorTarget
    : actorTargetPath
    | LPAREN
      expression
      RPAREN
    ;


actorTargetPath
    : SELF
      (
          DOUBLE_COLON
          identifier
      )*
    | identifier
      (
          DOUBLE_COLON
          identifier
      )*
    ;


/* ============================================================================
 * PUBLIC ADAPTERS
 * ========================================================================== */

/*
 * `actorStatement` is a stable actor-domain statement facade.
 *
 * Concrete syntax remains owned above.
 */
actorStatement
    : actorSpawnStatement
    | actorCommandStatement
    | actorSupervisionStatement
    ;


actorExpression
    : actorSpawnExpression
    | actorCommand
    ;


/* ============================================================================
 * SEMANTIC TOOLING ADAPTERS
 * ========================================================================== */

/*
 * These aliases provide stable semantic categories without creating a second
 * actor grammar.
 */

actorDeclarationConstruct
    : actorDeclaration
    ;


actorConstructionConstruct
    : actorSpawnExpression
    ;


actorCommunicationConstruct
    : actorCommand
    ;


actorLifecycleConstruct
    : actorLifecycleExpression
    ;


actorSupervisionConstructRoot
    : actorSupervisionConstruct
    ;


/*
 * ============================================================================
 * END OF ACTORS GRAMMAR
 * ============================================================================
 */
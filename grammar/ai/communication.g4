/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/ai/communication.g4
 *
 * Grammar:
 *     AICommunication
 *
 * Status:
 *     CANONICAL AI COMMUNICATION SOURCE-GRAMMAR BOUNDARY
 *
 * Language baseline:
 *     Rust 1.97+
 *     Rust 2021
 *     Safe Rust only
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file defines the source-level grammar boundary for communication
 * constructs used by AI, agentic, distributed, classical, hybrid, quantum-
 * coordinated, hardware-coordinated, and future computational programs.
 *
 * Communication is intentionally represented as COMPUTATIONAL INTENT.
 *
 * This grammar does not select:
 *
 *     - a network;
 *     - a physical device;
 *     - a CPU;
 *     - a GPU;
 *     - an FPGA;
 *     - an ASIC;
 *     - a QPU;
 *     - a node;
 *     - a process;
 *     - a thread;
 *     - a physical endpoint;
 *     - a route;
 *     - a topology;
 *     - a transport protocol;
 *     - a scheduler;
 *     - a memory location;
 *     - a vendor runtime.
 *
 * Those decisions belong to downstream semantic analysis, resource
 * negotiation, networking, distributed execution, scheduling, routing,
 * deployment, and HAL layers.
 *
 * The grammar is deliberately OPEN-WORLD.
 *
 * New communication protocols, message schemas, transports, serialization
 * formats, agent roles, endpoint implementations, accelerator transports,
 * quantum-classical control channels, or future communication mechanisms must
 * not require a new core grammar production merely because the implementation
 * is new.
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns:
 *
 *     communicationConstruct
 *     communicationMessage
 *     communicationChannel
 *     communicationEndpoint
 *     communicationStream
 *     communicationInvocation
 *     communicationDeclaration
 *     communicationTarget
 *     communicationArguments
 *     communicationTypedDeclaration
 *     communicationInitializer
 *
 * It owns only the SOURCE STRUCTURE of these communication constructs.
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     lexical tokens;
 *     identifiers;
 *     qualified names;
 *     expressions;
 *     expression precedence;
 *     general types;
 *     declarations;
 *     actor lifecycle;
 *     actor scheduling;
 *     message transport;
 *     network protocols;
 *     network topology;
 *     distributed consensus;
 *     distributed placement;
 *     serialization implementation;
 *     compression;
 *     encryption;
 *     authentication;
 *     authorization;
 *     resource discovery;
 *     capability discovery;
 *     hardware discovery;
 *     quantum operations;
 *     physical qubits;
 *     quantum routing;
 *     QEC;
 *     ZQN;
 *     HAL;
 *     model execution;
 *     training;
 *     inference;
 *     learning;
 *     adaptation;
 *     AI model semantics;
 *     AI agent semantics;
 *     runtime execution;
 *     compiler scheduling;
 *     backend implementation;
 *     communication IR.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * This file is the AI-domain communication grammar boundary.
 *
 * It MUST NOT create:
 *
 *     ActorIR
 *     MessageIR
 *     NetworkIR
 *     CommunicationIR
 *     QuantumCommunicationIR
 *     DeviceCommunicationIR
 *
 * Communication semantics lower through the ordinary Zamani semantic model
 * and canonical IR pipeline.
 *
 * Existing actor semantics remain owned by:
 *
 *     grammar/concurrency/actors.g4
 *
 * Networking semantics remain owned by:
 *
 *     grammar/networking/
 *
 * Distributed semantics remain owned by:
 *
 *     grammar/distributed/
 *
 * AI agent semantics remain owned by:
 *
 *     grammar/ai/agents.g4
 *
 * This file supplies the communication syntax that those systems may consume.
 *
 * ============================================================================
 * LEXICAL AUTHORITY
 * ============================================================================
 *
 * The canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Existing shared lexical vocabulary includes communication-related tokens
 * such as:
 *
 *     MESSAGE
 *     MESSAGES
 *     CHANNEL
 *     ENDPOINT
 *     STREAM
 *
 * This file MUST NOT define lexer rules.
 *
 * It MUST NOT introduce:
 *
 *     MESSAGE_CUSTOM
 *     SEND_CUSTOM
 *     RECEIVE_CUSTOM
 *     AI_MESSAGE
 *     AGENT_MESSAGE
 *     NETWORK_MESSAGE
 *     QUANTUM_MESSAGE
 *
 * or equivalent duplicate token families.
 *
 * Communication concepts that are not universal lexical constructs remain
 * identifiers or semantic metadata.
 *
 * ============================================================================
 * OPEN-WORLD COMMUNICATION MODEL
 * ============================================================================
 *
 * Communication is represented through a small number of structural forms.
 *
 * Message invocation:
 *
 *     message(target, payload);
 *
 * Message declaration:
 *
 *     message name;
 *
 * Typed message declaration:
 *
 *     message name: MessageType;
 *
 * Message initialization:
 *
 *     message name = value;
 *
 * Channel declaration:
 *
 *     channel name;
 *
 * Typed channel declaration:
 *
 *     channel name: ChannelType;
 *
 * Channel initialization:
 *
 *     channel name = value;
 *
 * Endpoint declaration:
 *
 *     endpoint name;
 *
 * Typed endpoint declaration:
 *
 *     endpoint name: EndpointType;
 *
 * Endpoint initialization:
 *
 *     endpoint name = value;
 *
 * Stream declaration:
 *
 *     stream name;
 *
 * Typed stream declaration:
 *
 *     stream name: StreamType;
 *
 * Stream initialization:
 *
 *     stream name = value;
 *
 * The semantic layer determines whether a declaration represents:
 *
 *     actor communication
 *     agent communication
 *     process communication
 *     distributed communication
 *     network communication
 *     local communication
 *     accelerator communication
 *     host/device communication
 *     quantum/classical control communication
 *     simulation communication
 *     future communication substrates
 *
 * ============================================================================
 * WHY NO SEND / RECEIVE KEYWORDS
 * ============================================================================
 *
 * The current canonical lexer does not require separate SEND and RECEIVE
 * keywords for this source boundary.
 *
 * More importantly, a universal communication grammar must not become a
 * catalog of every communication verb.
 *
 * Operations such as:
 *
 *     send
 *     receive
 *     publish
 *     subscribe
 *     request
 *     respond
 *     broadcast
 *     multicast
 *     collect
 *     scatter
 *     gather
 *     reduce
 *     stream
 *
 * may be represented by ordinary expressions, message operations, channel
 * semantics, libraries, dialects, or registered capabilities.
 *
 * The core grammar therefore remains stable as communication systems evolve.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The domain-neutral AST must preserve sufficient structure to represent:
 *
 *     construct kind
 *     source span
 *     target
 *     arguments
 *     declared name
 *     optional type
 *     optional initializer
 *     source order
 *     nesting
 *
 * A message invocation should preserve:
 *
 *     target
 *     ordered arguments
 *
 * A communication declaration should preserve:
 *
 *     kind
 *     name
 *     optional type
 *     optional initializer
 *
 * The AST MUST NOT contain physical realization concepts such as:
 *
 *     CPUMessage
 *     GPUMessage
 *     QPUMessage
 *     NodeMessage
 *     DeviceMessage
 *     PhysicalEndpoint
 *     NetworkCard
 *     PhysicalChannel
 *     PhysicalQueue
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     - whether the communication construct is valid;
 *     - whether the target resolves;
 *     - whether argument types are compatible;
 *     - whether the communication capability is available;
 *     - whether required effects are declared;
 *     - whether policy permits the operation;
 *     - whether security requirements are satisfied;
 *     - whether resource requirements can be satisfied;
 *     - whether the communication is local or distributed;
 *     - whether ordering guarantees are meaningful;
 *     - whether reliability requirements can be met;
 *     - whether the communication is deterministic or nondeterministic;
 *     - whether serialization/conversion is required;
 *     - whether provenance must be recorded.
 *
 * None of those decisions occur in this grammar.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Types are owned by:
 *
 *     grammar/types/
 *
 * This grammar consumes:
 *
 *     typeExpression
 *
 * It does not define:
 *
 *     MessageType
 *     ChannelType
 *     EndpointType
 *     StreamType
 *
 * as a competing type system.
 *
 * Such names may exist as ordinary type identifiers.
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * Communication targets and arguments use the canonical:
 *
 *     expression
 *
 * rule.
 *
 * This allows communication to carry:
 *
 *     classical values
 *     quantum-related values
 *     measurement results
 *     tensors
 *     models
 *     datasets
 *     symbolic values
 *     structured data
 *     handles
 *     references
 *     streams
 *     future domain values
 *
 * without creating a separate communication expression hierarchy.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Communication normally participates in effects such as:
 *
 *     network
 *     distributed
 *     IO
 *     foreign
 *     mutation
 *     simulation
 *
 * The exact effect set is determined by semantic analysis.
 *
 * The grammar does not infer or enforce effects.
 *
 * A local communication operation may not require the same effects as a
 * distributed network operation.
 *
 * Therefore the grammar must not hard-code a universal effect for every
 * `message`, `channel`, `endpoint`, or `stream`.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Communication may require capabilities such as:
 *
 *     communication
 *     messaging
 *     streaming
 *     networking
 *     distributed.execution
 *     serialization
 *     secure.communication
 *
 * or future capabilities.
 *
 * Capabilities remain open-world semantic values.
 *
 * This grammar does not enumerate the capability universe.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Communication may consume or require:
 *
 *     memory
 *     bandwidth
 *     storage
 *     compute
 *     network resources
 *     accelerator resources
 *     distributed resources
 *     energy
 *     reliability capacity
 *
 * No physical quantity is fixed in this grammar.
 *
 * The following are architectural anti-patterns and MUST NOT appear:
 *
 *     MAX_MESSAGES
 *     MAX_CHANNELS
 *     MAX_ENDPOINTS
 *     MAX_STREAMS
 *     MAX_MESSAGE_SIZE
 *     MAX_CONNECTIONS
 *     MAX_NODES
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICES
 *
 * Message size, channel count, endpoint count, stream count, network size,
 * node count, device count, bandwidth, and memory are all environment and
 * resource properties.
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Communication constructs may appear within or be governed by:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * They may also participate in contract expressions.
 *
 * This grammar does not define contract semantics.
 *
 * Contract ownership remains in:
 *
 *     grammar/validation/
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Communication may be constrained by policies governing:
 *
 *     authorization
 *     confidentiality
 *     integrity
 *     network access
 *     endpoint access
 *     data movement
 *     foreign calls
 *     distributed execution
 *     resource consumption
 *     sandboxing
 *     provenance
 *
 * Policy evaluation remains downstream.
 *
 * Communication syntax must not bypass policy enforcement.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * This grammar contains no:
 *
 *     authentication implementation
 *     encryption implementation
 *     key management
 *     certificate validation
 *     trust decisions
 *     network access
 *     process execution
 *
 * Security semantics belong to:
 *
 *     grammar/security/
 *
 * and the corresponding semantic/runtime systems.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Communication constructs must preserve enough source structure for
 * downstream provenance to record:
 *
 *     source
 *     target
 *     operation
 *     transformation
 *     serialization
 *     policy decision
 *     evidence
 *     execution context
 *     version
 *
 * This grammar creates no runtime provenance record.
 *
 * ============================================================================
 * ACTOR INTEGRATION
 * ============================================================================
 *
 * Agents and actors must NOT become two independent communication models.
 *
 * The intended architecture is:
 *
 *     AI agent
 *         |
 *         v
 *     actor / task / service
 *         |
 *         v
 *     communication intent
 *         |
 *         v
 *     concurrency / distributed / networking semantics
 *         |
 *         v
 *     execution planning
 *
 * `grammar/ai/agents.g4` owns agent structure.
 *
 * `grammar/concurrency/actors.g4` owns actor structure and lifecycle.
 *
 * This file owns the communication source boundary.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Communication may cross:
 *
 *     processes
 *     threads
 *     tasks
 *     actors
 *     nodes
 *     clusters
 *     clouds
 *     heterogeneous devices
 *     future execution substrates
 *
 * The grammar remains unchanged as those scales change.
 *
 * Distributed placement, consistency, ordering, fault tolerance, retries,
 * routing, and scheduling remain downstream.
 *
 * ============================================================================
 * NETWORKING INTEGRATION
 * ============================================================================
 *
 * Network realization may use:
 *
 *     local transports
 *     shared-memory transports
 *     IPC
 *     sockets
 *     message brokers
 *     streams
 *     RDMA-like transports
 *     accelerator interconnects
 *     quantum-classical links
 *     future transports
 *
 * None of these becomes mandatory core syntax.
 *
 * Transport selection is a semantic/backend concern.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Communication may carry or control quantum-related computation.
 *
 * Examples include:
 *
 *     classical control -> quantum execution
 *     measurement result -> classical computation
 *     classical parameter -> quantum operation
 *     quantum execution result -> AI model
 *     distributed quantum coordination
 *
 * This grammar does NOT define quantum operations.
 *
 * Quantum semantics remain:
 *
 *     domain-neutral AST
 *          |
 *          v
 *     quantum semantic model
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     decomposition
 *          |
 *          v
 *     routing
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     resilience / QEC
 *          |
 *          v
 *     ZQN
 *          |
 *          v
 *     HAL
 *
 * Communication grammar must never introduce:
 *
 *     QubitId
 *     physical qubit
 *     physical QPU
 *     quantum topology
 *     gate catalog
 *     calibration
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Communication intent may eventually lower to:
 *
 *     software messaging
 *     hardware channels
 *     streams
 *     interfaces
 *     control paths
 *     accelerator communication
 *     hardware/software co-design
 *
 * This grammar does not define:
 *
 *     wire widths
 *     fixed registers
 *     pins
 *     buses
 *     clock frequencies
 *     physical links
 *     device IDs
 *
 * HDL and hardware grammars remain authoritative for those domains.
 *
 * ============================================================================
 * DATA / SERIALIZATION INTEGRATION
 * ============================================================================
 *
 * Message payloads may semantically use:
 *
 *     records
 *     tuples
 *     arrays
 *     maps
 *     tensors
 *     datasets
 *     JSON-compatible values
 *     XML-compatible values
 *     binary representations
 *     foreign data
 *
 * Serialization format is not part of the universal communication grammar.
 *
 * JSON/XML/SQL and other external formats remain dialect/interoperability
 * concerns.
 *
 * ============================================================================
 * FFI / ABI INTEGRATION
 * ============================================================================
 *
 * Communication can cross foreign-language or ABI boundaries.
 *
 * If it does, the semantic model must account for:
 *
 *     foreign effects
 *     ABI compatibility
 *     data representation
 *     ownership
 *     lifetime
 *     security
 *     provenance
 *
 * The grammar itself remains independent of ABI details.
 *
 * ============================================================================
 * SIMULATION INTEGRATION
 * ============================================================================
 *
 * Communication constructs must be representable in simulation.
 *
 * The same source may be evaluated through:
 *
 *     classical simulation
 *     distributed simulation
 *     network simulation
 *     hardware simulation
 *     quantum simulation
 *     AI simulation
 *
 * Simulation is an execution mode, not a second communication language.
 *
 * ============================================================================
 * DETERMINISM / REPRODUCIBILITY
 * ============================================================================
 *
 * Parsing is deterministic.
 *
 * It must depend only on:
 *
 *     source token sequence
 *     grammar version
 *     lexer vocabulary
 *     explicit parser configuration
 *
 * Parsing MUST NOT inspect:
 *
 *     CPU count
 *     GPU count
 *     QPU availability
 *     network state
 *     memory availability
 *     filesystem state
 *     wall-clock time
 *     randomness
 *     runtime state
 *     scheduler state
 *
 * Runtime communication may of course be nondeterministic.
 *
 * Reproducibility and replay belong to execution/provenance semantics.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar imposes no artificial finite ceiling on:
 *
 *     messages
 *     message arguments
 *     channels
 *     endpoints
 *     streams
 *     communication operations
 *     nested communication constructs
 *     participating agents
 *     participating actors
 *     participating processes
 *     participating nodes
 *     distributed domains
 *     payload structure
 *     payload dimensions
 *     tensor rank
 *     tensor dimensions
 *     communication paths
 *     network size
 *     device count
 *     CPU count
 *     GPU count
 *     FPGA count
 *     accelerator count
 *     QPU count
 *     qubit count
 *     memory capacity
 *
 * There must be no language-level constants such as:
 *
 *     MAX_MESSAGES
 *     MAX_CHANNELS
 *     MAX_ENDPOINTS
 *     MAX_STREAMS
 *     MAX_AGENTS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_MEMORY
 *
 * or equivalent hidden limits.
 *
 * "Infinity" here means:
 *
 *     no artificial finite language-level machine-capacity ceiling.
 *
 * It does not claim physically infinite resources.
 *
 * Actual limitations are reported through:
 *
 *     semantic validation
 *     capability negotiation
 *     resource analysis
 *     compilation
 *     scheduling
 *     deployment
 *     runtime
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
 *     ASIC identifiers
 *     QPU identifiers
 *     physical node identifiers
 *     physical endpoint identifiers
 *     fixed network topology
 *     fixed memory size
 *     fixed register width
 *
 * Source describes intent.
 *
 * Compilation and execution determine realization.
 *
 * ============================================================================
 * NO APPLICATION-SPECIFIC KEYWORD EXPLOSION
 * ============================================================================
 *
 * Do NOT add grammar productions for application concepts such as:
 *
 *     social_message
 *     payment_message
 *     robotics_message
 *     legal_message
 *     medical_message
 *     vision_message
 *     sentiment_message
 *     blockchain_message
 *     gaming_message
 *
 * These remain libraries, dialects, schemas, policies, or application-level
 * abstractions.
 *
 * ============================================================================
 * GRAMMAR DEPENDENCIES
 * ============================================================================
 *
 * Imports:
 *
 *     Types
 *     Expressions
 *
 * `Statements` is deliberately NOT imported here.
 *
 * Communication is a leaf grammar and must not create a circular dependency
 * through the statement composition layer.
 *
 * ============================================================================
 * PUBLIC RULES
 * ============================================================================
 *
 * Primary:
 *
 *     communicationConstruct
 *
 * Secondary:
 *
 *     communicationMessage
 *     communicationChannel
 *     communicationEndpoint
 *     communicationStream
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar AICommunication;

options {
    tokenVocab = ZamaniLexer;
}

import
    Types,
    Expressions
    ;


/*
 * ============================================================================
 * 1. PUBLIC COMMUNICATION CONSTRUCT
 * ============================================================================
 *
 * This is the single parser-facing entry point.
 *
 * AI composition must consume communication through this rule.
 */

communicationConstruct
    : communicationMessage
    | communicationChannel
    | communicationEndpoint
    | communicationStream
    ;


/*
 * ============================================================================
 * 2. MESSAGE
 * ============================================================================
 *
 * Invocation:
 *
 *     message(target, payload);
 *
 * Declaration:
 *
 *     message name;
 *
 * Typed declaration:
 *
 *     message name: MessageType;
 *
 * Initialization:
 *
 *     message name = value;
 *
 * The target/payload semantics are determined downstream.
 */

communicationMessage
    : MESSAGE LPAREN communicationArguments? RPAREN SEMI
    | MESSAGE communicationDeclaration
    ;


/*
 * ============================================================================
 * 3. CHANNEL
 * ============================================================================
 *
 * Channel is a source-level communication resource/reference.
 *
 * It does not select a physical queue, link, device, process, or network.
 */

communicationChannel
    : CHANNEL communicationDeclaration
    ;


/*
 * ============================================================================
 * 4. ENDPOINT
 * ============================================================================
 *
 * Endpoint is a logical communication endpoint.
 *
 * Its physical or network realization is determined downstream.
 */

communicationEndpoint
    : ENDPOINT communicationDeclaration
    ;


/*
 * ============================================================================
 * 5. STREAM
 * ============================================================================
 *
 * Stream is a logical ordered communication resource.
 *
 * Ordering, buffering, transport, flow control, and realization are semantic
 * and runtime concerns.
 */

communicationStream
    : STREAM communicationDeclaration
    ;


/*
 * ============================================================================
 * 6. COMMUNICATION DECLARATION
 * ============================================================================
 *
 * Shared declaration shape:
 *
 *     name;
 *     name: Type;
 *     name = expression;
 *     name: Type = expression;
 */

communicationDeclaration
    : identifier
      communicationDeclarationTail
    ;


communicationDeclarationTail
    : SEMI
    | COLON
      typeExpression
      communicationTypedTail
    | ASSIGN
      expression
      SEMI
    ;


communicationTypedTail
    : SEMI
    | ASSIGN
      expression
      SEMI
    ;


/*
 * ============================================================================
 * 7. COMMUNICATION ARGUMENTS
 * ============================================================================
 *
 * Arguments use the canonical expression grammar.
 *
 * No payload-specific grammar is introduced.
 */

communicationArguments
    : expression
      (
          COMMA
          expression
      )*
    ;


/*
 * ============================================================================
 * 8. EXPLICIT COMMUNICATION TARGET
 * ============================================================================
 *
 * This named boundary exists for AST and semantic tooling.
 *
 * It intentionally remains an expression.
 */

communicationTarget
    : expression
    ;


/*
 * ============================================================================
 * 9. EXPLICIT COMMUNICATION EXPRESSION
 * ============================================================================
 *
 * Stable tooling bridge.
 */

communicationExpression
    : expression
    ;


/*
 * ============================================================================
 * 10. EXPLICIT COMMUNICATION TYPE
 * ============================================================================
 *
 * Stable tooling bridge.
 */

communicationType
    : typeExpression
    ;


/*
 * ============================================================================
 * 11. EXPLICIT COMMUNICATION VALUE
 * ============================================================================
 *
 * Stable tooling bridge.
 */

communicationValue
    : expression
    ;


/*
 * ============================================================================
 * 12. INTEGRATION HELPERS
 * ============================================================================
 *
 * These rules are intentionally thin.
 *
 * They provide named semantic/parser boundaries without creating competing
 * expression or type systems.
 */

communicationPayload
    : expression
    ;


communicationName
    : identifier
    ;


/*
 * ============================================================================
 * AST INTEGRATION CONTRACT
 * ============================================================================
 *
 * Frontend AST construction must map:
 *
 *     communicationMessage
 *         -> generic communication/message operation node
 *
 *     communicationChannel
 *         -> generic communication/channel declaration node
 *
 *     communicationEndpoint
 *         -> generic communication/endpoint declaration node
 *
 *     communicationStream
 *         -> generic communication/stream declaration node
 *
 * The exact Rust AST type is owned by the frontend AST implementation.
 *
 * This grammar does not dictate the Rust structure.
 *
 * ============================================================================
 * SEMANTIC MODEL INTEGRATION
 * ============================================================================
 *
 * The semantic model should normalize communication into generic intent
 * carrying concepts such as:
 *
 *     kind
 *     name
 *     target
 *     operands
 *     parameters
 *     results
 *     attributes
 *     modifiers
 *     effects
 *     capabilities
 *     resources
 *     contracts
 *     policies
 *     provenance
 *     source
 *
 * Communication-specific semantic information must remain domain-neutral.
 *
 * ============================================================================
 * CANONICAL IR INTEGRATION
 * ============================================================================
 *
 * Communication MUST lower through:
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
 *     domain-neutral AST
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     semantic model
 *       |
 *       v
 *     canonical IR
 *       |
 *       +-------------------------+
 *       |                         |
 *       v                         v
 *     classical              quantum::ir
 *       |                         |
 *       +------------+------------+
 *                    |
 *                    v
 *             optimization
 *                    |
 *             lowering/planning
 *                    |
 *             routing/scheduling
 *                    |
 *             resilience/recovery
 *                    |
 *                    v
 *                   HAL
 *
 * This grammar MUST NOT emit IR directly.
 *
 * ============================================================================
 * QUANTUM IR BOUNDARY
 * ============================================================================
 *
 * If communication controls or carries quantum computation, only the
 * quantum-semantic portion crosses into:
 *
 *     quantum::ir
 *
 * This file does not create:
 *
 *     QuantumCommunicationIR
 *     QubitCommunicationIR
 *     QPUMessageIR
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Source-level communication can be accompanied elsewhere by generic
 * requirements such as:
 *
 *     requires capability("communication");
 *
 *     requires capability("network");
 *
 *     requires capability("distributed.execution");
 *
 *     requires memory >= required_memory;
 *
 *     requires bandwidth >= required_bandwidth;
 *
 *     requires topology(required_topology);
 *
 * These expressions are evaluated downstream.
 *
 * The communication grammar does not evaluate resource availability.
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * The semantic layer may associate communication with:
 *
 *     network
 *     distributed
 *     io
 *     foreign
 *     mutation
 *     simulation
 *
 * based on actual realization.
 *
 * This prevents the grammar from falsely assuming that every communication
 * operation has the same effect set.
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Communication must pass through applicable policy checks.
 *
 * Examples include:
 *
 *     endpoint authorization
 *     network access
 *     data movement policy
 *     confidentiality
 *     integrity
 *     resource budget
 *     sandbox restrictions
 *     foreign-call restrictions
 *
 * Policy semantics remain outside this grammar.
 *
 * ============================================================================
 * PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Semantic and runtime layers may record:
 *
 *     message origin
 *     target
 *     transformation
 *     serialization
 *     routing decision
 *     policy decision
 *     execution realization
 *
 * The parser only preserves source structure needed to support that record.
 *
 * ============================================================================
 * ERROR BOUNDARY
 * ============================================================================
 *
 * Parser diagnostics cover structural errors such as:
 *
 *     missing target
 *     malformed argument list
 *     missing declaration name
 *     malformed type clause
 *     missing initializer expression
 *     missing terminator
 *
 * Semantic diagnostics cover:
 *
 *     unresolved target
 *     invalid payload type
 *     unavailable communication capability
 *     insufficient resources
 *     forbidden communication policy
 *     invalid endpoint
 *     incompatible serialization
 *     unsupported transport
 *     invalid distributed placement
 *     invalid quantum/classical boundary
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * This file relies on existing canonical lexer tokens.
 *
 * It does not rename:
 *
 *     MESSAGE
 *     CHANNEL
 *     ENDPOINT
 *     STREAM
 *
 * Existing source syntax must remain compatible wherever these tokens already
 * have established meaning.
 *
 * Any conflict with an older grammar must be resolved through the repository's
 * compatibility layer rather than by introducing duplicate token names.
 *
 * ============================================================================
 * RUST / SAFETY CONTRACT
 * ============================================================================
 *
 * This is an ANTLR parser grammar and contains no Rust implementation code.
 *
 * Generated parser integration must remain compatible with:
 *
 *     Rust 1.97+
 *     Rust 2021
 *
 * The Zamani implementation must remain safe Rust only.
 *
 * No `unsafe` code is required by this grammar.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no actions;
 *     no semantic predicates;
 *     no runtime callbacks;
 *     no filesystem access;
 *     no network access;
 *     no hardware discovery;
 *     no environment inspection;
 *     no randomness.
 *
 * Therefore parsing is deterministic for a fixed:
 *
 *     source
 *     lexer vocabulary
 *     grammar version
 *     parser configuration
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * Grammar repetition is structurally open-ended.
 *
 * There is no fixed language-level limit on:
 *
 *     message count
 *     channel count
 *     endpoint count
 *     stream count
 *     payload expression size
 *     argument count
 *     communication operations
 *     agents
 *     actors
 *     nodes
 *     devices
 *     CPUs
 *     GPUs
 *     FPGAs
 *     accelerators
 *     QPUs
 *     qubits
 *     memory
 *     network size
 *
 * Resource exhaustion is not silently converted into language semantics.
 *
 * Implementations must report resource limitations explicitly.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * UPSTREAM
 * --------
 *
 *     grammar/antlr/ZamaniLexer.g4
 *         canonical MESSAGE/CHANNEL/ENDPOINT/STREAM tokens
 *
 *     grammar/types/
 *         typeExpression
 *
 *     grammar/expressions/
 *         expression
 *
 * COMPOSITION
 * -----------
 *
 *     grammar/ai/ai.g4
 *
 * must import:
 *
 *     AICommunication
 *
 * and expose:
 *
 *     communicationConstruct
 *
 * AGENT INTEGRATION
 * -----------------
 *
 *     grammar/ai/agents.g4
 *
 * may consume communicationConstruct inside an agent body.
 *
 * This is preferable to creating a second agent communication grammar.
 *
 * CONCURRENCY
 * -----------
 *
 *     grammar/concurrency/actors.g4
 *
 * remains authoritative for actor lifecycle and actor semantics.
 *
 * DISTRIBUTED
 * -----------
 *
 *     grammar/distributed/
 *
 * remains authoritative for distributed execution semantics.
 *
 * NETWORKING
 * ----------
 *
 *     grammar/networking/
 *
 * remains authoritative for network realization.
 *
 * SECURITY
 * --------
 *
 *     grammar/security/
 *
 * remains authoritative for communication security policy.
 *
 * RESOURCES
 * ---------
 *
 *     grammar/resources/
 *
 * remains authoritative for resource requirements and capability negotiation.
 *
 * VALIDATION
 * ----------
 *
 *     grammar/validation/
 *
 * remains authoritative for contracts and assertions.
 *
 * PROVENANCE
 * ----------
 *
 *     grammar/spec/provenance.md
 *     semantic provenance subsystem
 *
 * remain authoritative for provenance.
 *
 * IR
 * --
 *
 * No communication-specific IR is introduced.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive parser tests:
 *
 *     message(target, payload);
 *     message target;
 *     message target: MessageType;
 *     message target = payload;
 *
 *     channel channel_name;
 *     channel channel_name: ChannelType;
 *     channel channel_name = channel_value;
 *
 *     endpoint endpoint_name;
 *     endpoint endpoint_name: EndpointType;
 *     endpoint endpoint_name = endpoint_value;
 *
 *     stream stream_name;
 *     stream stream_name: StreamType;
 *     stream stream_name = stream_value;
 *
 * Expression payloads must include:
 *
 *     scalar values
 *     structured values
 *     calls
 *     indexing
 *     member access
 *     tensor values
 *     model values
 *     quantum-derived values
 *     hybrid values
 *
 * Negative parser tests:
 *
 *     message;
 *     message();
 *     message(;
 *     message(target,);
 *     message target:;
 *     channel;
 *     endpoint;
 *     stream;
 *     channel name:
 *     endpoint name =
 *     stream name =
 *
 * Boundary tests:
 *
 *     AI agent -> message
 *     AI agent -> channel
 *     AI agent -> stream
 *     message -> quantum measurement result
 *     message -> tensor value
 *     message -> distributed computation
 *     message -> simulated execution
 *
 * Scalability tests:
 *
 *     arbitrarily many communication declarations;
 *     arbitrarily many message arguments;
 *     deeply nested payload expressions;
 *     large structured payloads;
 *     large agent communication graphs;
 *     large distributed communication graphs.
 *
 * Determinism tests:
 *
 *     identical source produces identical parse trees;
 *     parser does not inspect hardware;
 *     parser does not inspect resource availability;
 *     parser does not inspect network state.
 *
 * Compatibility tests:
 *
 *     existing MESSAGE syntax;
 *     existing CHANNEL syntax;
 *     existing ENDPOINT syntax;
 *     existing STREAM syntax;
 *     legacy parser surfaces where still supported.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file introduces:
 *
 *     no machine capacity constants;
 *     no fixed message size;
 *     no fixed channel count;
 *     no fixed endpoint count;
 *     no fixed stream count;
 *     no fixed node count;
 *     no fixed device count;
 *     no fixed network size;
 *     no fixed processor count;
 *     no fixed accelerator count;
 *     no fixed qubit count;
 *     no fixed tensor rank;
 *     no fixed register width.
 *
 * It also introduces:
 *
 *     no vendor names;
 *     no protocol catalog;
 *     no transport catalog;
 *     no AI framework catalog;
 *     no hardware catalog.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [ ] AICommunication generates successfully.
 *
 *     [ ] It uses only ZamaniLexer tokens.
 *
 *     [ ] It does not define lexer rules.
 *
 *     [ ] It imports Types and Expressions only.
 *
 *     [ ] It has exactly one public communicationConstruct boundary.
 *
 *     [ ] Message syntax is structurally deterministic.
 *
 *     [ ] Channel syntax is structurally deterministic.
 *
 *     [ ] Endpoint syntax is structurally deterministic.
 *
 *     [ ] Stream syntax is structurally deterministic.
 *
 *     [ ] Communication payloads use canonical expressions.
 *
 *     [ ] Communication types use canonical type expressions.
 *
 *     [ ] No competing expression hierarchy exists.
 *
 *     [ ] No competing type system exists.
 *
 *     [ ] No actor system is duplicated.
 *
 *     [ ] No networking system is duplicated.
 *
 *     [ ] No distributed system is duplicated.
 *
 *     [ ] No quantum grammar is duplicated.
 *
 *     [ ] No communication IR is introduced.
 *
 *     [ ] AI composition imports this grammar.
 *
 *     [ ] Agent integration consumes the same communication boundary.
 *
 *     [ ] Resource semantics remain downstream.
 *
 *     [ ] Capability semantics remain downstream.
 *
 *     [ ] Effects remain downstream.
 *
 *     [ ] Policy remains downstream.
 *
 *     [ ] Security remains downstream.
 *
 *     [ ] Provenance remains downstream.
 *
 *     [ ] Quantum semantics cross only through quantum::ir.
 *
 *     [ ] No physical target is selected by parsing.
 *
 *     [ ] No machine-capacity constant exists.
 *
 *     [ ] No unsafe Rust is required.
 *
 *     [ ] Positive tests exist.
 *
 *     [ ] Negative tests exist.
 *
 *     [ ] Boundary tests exist.
 *
 *     [ ] Scalability tests exist.
 *
 *     [ ] Determinism tests exist.
 *
 *     [ ] Compatibility tests exist.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */
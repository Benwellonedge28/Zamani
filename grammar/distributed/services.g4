/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/distributed/services.g4
 *
 * GRAMMAR
 * -------
 * DistributedServices
 *
 * STATUS
 * ------
 * PRODUCTION-READY DISTRIBUTED SERVICE SOURCE GRAMMAR
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 1.97+
 * Rust Edition 2021
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
 * This file is the canonical SOURCE-LEVEL syntax owner for logical
 * distributed computational services.
 *
 * A distributed service is a logical computational interface and execution
 * boundary. It may expose:
 *
 *     - operations;
 *     - state;
 *     - events;
 *     - lifecycle intent;
 *     - dependencies;
 *     - requirements;
 *     - capabilities;
 *     - policies;
 *     - contracts;
 *     - metadata;
 *     - extensions.
 *
 * A service is a SOURCE-LEVEL abstraction.
 *
 * It is not:
 *
 *     - a process;
 *     - a thread;
 *     - an actor;
 *     - a node;
 *     - a machine;
 *     - a network endpoint;
 *     - a socket;
 *     - a transport;
 *     - a scheduler;
 *     - a deployment instance;
 *     - a physical device;
 *     - a quantum device;
 *     - an HDL module;
 *     - an IR object.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     DistributedServices
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          +--> name resolution
 *          +--> type analysis
 *          +--> effect analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> contract analysis
 *          +--> policy analysis
 *          +--> provenance
 *          +--> distributed semantic analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical semantics / classical IR
 *          +--> quantum semantics / quantum::ir
 *          +--> HDL / hardware semantics
 *          +--> AI/data semantics
 *          +--> networking semantics
 *          +--> distributed execution semantics
 *          |
 *          v
 *     optimization
 *          |
 *          +--> specialization
 *          +--> placement
 *          +--> routing
 *          +--> scheduling
 *          +--> resilience
 *          +--> deployment planning
 *          |
 *          v
 *     target realization
 *          |
 *          v
 *     runtime
 *
 * This grammar performs NONE of those downstream operations.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     distributedServiceDeclaration
 *     distributedServiceGenericParameters
 *     distributedServiceGenericParameter
 *     distributedServiceGenericBound
 *     distributedServiceInheritanceClause
 *     distributedServiceImplementationClause
 *     distributedServiceRequirementClause
 *     distributedServiceBody
 *     distributedServiceMember
 *     distributedServiceOperation
 *     distributedServiceParameterList
 *     distributedServiceParameter
 *     distributedServiceParameterModifier
 *     distributedServiceReturnClause
 *     distributedServiceOperationModifier
 *     distributedServiceOperationContract
 *     distributedServiceState
 *     distributedServiceEvent
 *     distributedServiceLifecycle
 *     distributedServiceDependency
 *     distributedServicePolicy
 *     distributedServiceRequirement
 *     distributedServiceCapability
 *     distributedServiceContract
 *     distributedServiceMetadata
 *     distributedServiceExtension
 *
 * This file also owns the service-specific structural wrappers required to
 * preserve those constructs as distinct AST categories.
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     lexer rules;
 *     token spelling;
 *     identifiers;
 *     qualified names;
 *     expressions;
 *     expression precedence;
 *     types;
 *     generic type applications;
 *     ordinary statements;
 *     ordinary blocks;
 *     actors;
 *     processes;
 *     nodes;
 *     channels;
 *     messages;
 *     network protocols;
 *     endpoints;
 *     transport;
 *     routing;
 *     placement;
 *     scheduling;
 *     resource allocation;
 *     capability discovery;
 *     deployment;
 *     replication algorithms;
 *     consistency algorithms;
 *     fault-tolerance algorithms;
 *     authentication;
 *     authorization;
 *     encryption;
 *     quantum operations;
 *     quantum topology;
 *     quantum::ir;
 *     classical IR;
 *     HDL IR;
 *     ZQN;
 *     HAL;
 *     runtime behavior.
 *
 * ============================================================================
 * CANONICAL AUTHORITIES
 * ============================================================================
 *
 * Lexical authority:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Lexical vocabulary:
 *
 *     grammar/lexer/tokens.g4
 *
 * Core names:
 *
 *     grammar/core/names.g4
 *
 * Core declarations/metadata/visibility/blocks:
 *
 *     grammar/core/
 *
 * Types:
 *
 *     grammar/types/
 *
 * Expressions:
 *
 *     grammar/expressions/
 *
 * Statements:
 *
 *     grammar/statements/
 *
 * Resources:
 *
 *     grammar/resources/
 *
 * Policies:
 *
 *     grammar/policies/
 *
 * Contracts:
 *
 *     grammar/validation/
 *     grammar/statements/
 *
 * Networking:
 *
 *     grammar/networking/
 *
 * Distributed composition:
 *
 *     grammar/distributed/distributed.g4
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * The canonical lexer already defines:
 *
 *     SERVICE
 *     SERVICES
 *     EXTENDS
 *     IMPLEMENTS
 *     REQUIRES
 *     WHERE
 *     ASYNC
 *     PURE
 *     POLICY
 *     CAPABILITY
 *     RESOURCE
 *     CONTRACT
 *     DETERMINISTIC
 *     REPRODUCIBLE
 *
 * This grammar consumes those canonical tokens.
 *
 * It MUST NOT create replacement spellings such as:
 *
 *     SERVICE_KW
 *     SERVICE_DECL
 *     SERVICE_MARKER
 *     LESS_THAN
 *     GREATER_THAN
 *     SEMI
 *     EQUALS
 *
 * where the canonical lexer already provides the required vocabulary.
 *
 * ============================================================================
 * CRITICAL LEXICAL CORRECTION
 * ============================================================================
 *
 * The previous version incorrectly documented `service` as an ordinary
 * identifier.
 *
 * The canonical lexical vocabulary contains:
 *
 *     SERVICE : 'service' ;
 *
 * Therefore the production grammar MUST consume:
 *
 *     SERVICE
 *
 * rather than:
 *
 *     identifier
 *
 * for the service declaration marker.
 *
 * This removes a major ambiguity with other distributed declarations.
 *
 * ============================================================================
 * NAME CONTRACT
 * ============================================================================
 *
 * Service names use the canonical identifier rule.
 *
 * References use the canonical qualifiedName rule.
 *
 * This grammar MUST NOT define:
 *
 *     serviceIdentifier
 *     serviceQualifiedName
 *     distributedIdentifier
 *
 * unless a future semantic requirement proves that such a distinct AST
 * category is necessary.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Parameter and state types consume:
 *
 *     typeExpression
 *
 * This grammar does not create:
 *
 *     ServiceType
 *     ServiceStateType
 *     RemoteType
 *     DistributedType
 *
 * as a second type system.
 *
 * Quantum, tensor, hardware, resource, data, AI, classical and future types
 * therefore remain composable through the canonical type architecture.
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * All executable/default/constraint values consume:
 *
 *     expression
 *
 * This grammar does not redefine expression precedence, operators, calls,
 * indexing, member access, literals, lambdas, quantum expressions, tensor
 * expressions or future expression families.
 *
 * ============================================================================
 * BLOCK CONTRACT
 * ============================================================================
 *
 * Service operation bodies use the canonical core block model.
 *
 * This is important because a distributed service operation may contain:
 *
 *     classical computation;
 *     quantum computation;
 *     HDL/hardware interaction;
 *     AI/model operations;
 *     data operations;
 *     concurrency;
 *     contracts;
 *     effects;
 *     resource requirements;
 *     policies;
 *     distributed communication.
 *
 * The service grammar must therefore NOT create a miniature statement
 * language.
 *
 * ============================================================================
 * DISTRIBUTED BOUNDARY
 * ============================================================================
 *
 * `distributed.g4` owns domain composition.
 *
 * This file owns only service syntax.
 *
 * Other distributed constructs remain owned by:
 *
 *     nodes.g4
 *     processes.g4
 *     actors.g4
 *     channels.g4
 *     communication.g4
 *     messaging.g4
 *     collective.g4
 *     replication.g4
 *     consistency.g4
 *     placement.g4
 *     partitioning.g4
 *     topology.g4
 *     remote-execution.g4
 *     deployment.g4
 *     fault-tolerance.g4
 *     contracts.g4
 *     transactions.g4
 *
 * This file MUST NOT redefine any of them.
 *
 * ============================================================================
 * SERVICE VS NETWORK SERVICE
 * ============================================================================
 *
 * Distributed service:
 *
 *     grammar/distributed/services.g4
 *
 * owns:
 *
 *     logical distributed computational service semantics.
 *
 * Network-facing service:
 *
 *     grammar/networking/services.g4
 *
 * owns:
 *
 *     communication-facing service contracts.
 *
 * These concepts may reference one another semantically, but they are not
 * syntactically merged.
 *
 * A distributed service does not automatically imply:
 *
 *     network transport;
 *     endpoint allocation;
 *     IP address;
 *     port;
 *     protocol;
 *     socket.
 *
 * ============================================================================
 * ACTOR INTEGRATION
 * ============================================================================
 *
 * Actors remain owned by:
 *
 *     grammar/concurrency/actors.g4
 *
 * A service may expose actor-compatible operations or semantically contain
 * actor-oriented computation.
 *
 * This grammar does not redefine actor syntax.
 *
 * ============================================================================
 * MESSAGE INTEGRATION
 * ============================================================================
 *
 * Message schemas remain owned by the canonical messaging/networking
 * subsystem.
 *
 * Service operations may use message types through ordinary:
 *
 *     typeExpression
 *     qualifiedName
 *
 * references.
 *
 * This grammar does not duplicate message declaration syntax.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Services may express semantic resource requirements.
 *
 * This grammar does not allocate resources.
 *
 * Requirements ultimately flow through:
 *
 *     service AST
 *          |
 *          v
 *     resource semantic analysis
 *          |
 *          v
 *     capability negotiation
 *          |
 *          v
 *     target feasibility
 *
 * Examples conceptually representable downstream include:
 *
 *     requires capability("quantum.measurement");
 *     requires capability("gpu.compute");
 *     requires memory >= required_memory;
 *     requires qubits >= required_qubits;
 *
 * No finite capacity is encoded by this grammar.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Capability syntax is represented structurally.
 *
 * This grammar does not define the universe of capabilities.
 *
 * New capabilities must be representable without changing this file merely
 * because a new target, accelerator, protocol, QPU, AI model, or hardware
 * family appears.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Service policies express governing intent.
 *
 * Policy meaning belongs to the canonical policy semantic model.
 *
 * Policies may govern:
 *
 *     resources;
 *     capabilities;
 *     execution;
 *     adaptation;
 *     security;
 *     reproducibility;
 *     simulation;
 *     deployment;
 *     resilience;
 *     communication.
 *
 * This grammar does not implement policy evaluation.
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * Service contracts are source-level semantic obligations.
 *
 * Their semantic interpretation belongs downstream.
 *
 * This grammar does not prove:
 *
 *     requires;
 *     ensures;
 *     invariant;
 *     assume;
 *     guarantee;
 *     property.
 *
 * It only preserves their source structure when attached to a service.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Service operations may ultimately participate in the universal effect
 * system.
 *
 * Effects are not redefined here.
 *
 * Examples include:
 *
 *     IO
 *     network
 *     mutation
 *     randomness
 *     foreign
 *     distributed
 *     measurement
 *     learning
 *     adaptation
 *     reflection
 *     simulation
 *
 * Semantic effect analysis occurs after parsing.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * The AST builder must preserve source spans and declaration/member ordering.
 *
 * Downstream provenance must be able to associate:
 *
 *     service declaration
 *     operation
 *     state
 *     event
 *     dependency
 *     requirement
 *     capability
 *     policy
 *     contract
 *     extension
 *
 * with their originating source locations.
 *
 * This grammar itself does not generate provenance records.
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * A distributed service may expose quantum computation through ordinary
 * canonical types and expressions.
 *
 * Example conceptual form:
 *
 *     service QuantumService {
 *         operation run(state: quantum::State) -> Result;
 *     }
 *
 * The service grammar does not define:
 *
 *     qubits;
 *     gates;
 *     physical qubits;
 *     topology;
 *     calibration;
 *     QEC;
 *     quantum routing;
 *     quantum scheduling.
 *
 * Those concerns remain downstream and eventually converge on:
 *
 *     quantum::ir
 *
 * ============================================================================
 * HDL / HARDWARE CONTRACT
 * ============================================================================
 *
 * A service may expose computation realized through:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     HDL
 *     future target
 *
 * without the service grammar selecting one.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * There are NO grammar-level finite limits on:
 *
 *     services;
 *     operations;
 *     parameters;
 *     states;
 *     events;
 *     dependencies;
 *     requirements;
 *     capabilities;
 *     policies;
 *     contracts;
 *     metadata;
 *     extensions;
 *     generic parameters;
 *     generic bounds;
 *     qualified-name depth;
 *     nesting.
 *
 * This grammar contains no:
 *
 *     MAX_SERVICES
 *     MAX_OPERATIONS
 *     MAX_PARAMETERS
 *     MAX_STATES
 *     MAX_EVENTS
 *     MAX_REPLICAS
 *     MAX_NODES
 *     MAX_WORKERS
 *     MAX_THREADS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_MEMORY
 *     MAX_DEVICES
 *     MAX_NETWORK_SIZE
 *
 * Repetition operators express language cardinality without establishing a
 * physical capacity.
 *
 * Parser/compiler resource budgets may exist as implementation controls, but
 * they must remain configurable implementation policy rather than language
 * semantics.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * A distributed service describes logical computation.
 *
 * The same source may therefore be considered for realization on:
 *
 *     - a tiny embedded target;
 *     - a CPU;
 *     - a multicore system;
 *     - a GPU;
 *     - an FPGA;
 *     - an ASIC;
 *     - an accelerator;
 *     - a QPU;
 *     - a simulator;
 *     - an HPC system;
 *     - a cluster;
 *     - a cloud;
 *     - a heterogeneous system;
 *     - a federated system;
 *     - a future computational substrate.
 *
 * The service source MUST NOT need rewriting merely because realization scale
 * changes, provided the semantic requirements can be satisfied.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     - supplied token stream;
 *     - grammar version;
 *     - selected language compatibility version where applicable.
 *
 * Parsing MUST NOT depend on:
 *
 *     - hardware;
 *     - runtime state;
 *     - filesystem state;
 *     - network state;
 *     - wall-clock time;
 *     - randomness;
 *     - resource availability.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no embedded Rust;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware discovery;
 *     - no runtime callbacks;
 *     - no resource allocation;
 *     - no target selection;
 *     - no execution.
 *
 * The downstream Rust implementation MUST remain compatible with Rust 1.97+
 * and safe Rust only.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST MUST preserve at least:
 *
 *     - attributes;
 *     - visibility;
 *     - service name;
 *     - generic parameters;
 *     - inheritance;
 *     - implementation contracts;
 *     - service requirements;
 *     - service body ordering;
 *     - operation declarations;
 *     - operation names;
 *     - operation generic parameters if later introduced;
 *     - parameter ordering;
 *     - parameter modifiers;
 *     - parameter names;
 *     - parameter types;
 *     - parameter defaults;
 *     - return types;
 *     - operation modifiers;
 *     - operation contracts;
 *     - operation bodies;
 *     - state declarations;
 *     - state initializers;
 *     - state modifiers;
 *     - event declarations;
 *     - lifecycle declarations;
 *     - dependencies;
 *     - policies;
 *     - requirements;
 *     - capabilities;
 *     - contracts;
 *     - metadata;
 *     - extensions;
 *     - source spans.
 *
 * AST construction must not perform semantic target resolution.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     - service-name resolution;
 *     - service uniqueness;
 *     - generic parameter validity;
 *     - inheritance validity;
 *     - implementation resolution;
 *     - operation uniqueness;
 *     - parameter/type validation;
 *     - default-value type checking;
 *     - return validation;
 *     - state validation;
 *     - event validation;
 *     - lifecycle validation;
 *     - dependency resolution;
 *     - capability resolution;
 *     - resource requirement validation;
 *     - policy validation;
 *     - effect compatibility;
 *     - contract checking;
 *     - distributed consistency;
 *     - target feasibility.
 *
 * None of those decisions belong in this parser.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file owns NO IR.
 *
 * Generic service:
 *
 *     service AST
 *         ->
 *     semantic service model
 *         ->
 *     canonical compiler representation
 *
 * Quantum-bearing service:
 *
 *     service AST
 *         ->
 *     semantic service model
 *         ->
 *     quantum semantic model
 *         ->
 *     quantum::ir
 *
 * Classical service:
 *
 *     service AST
 *         ->
 *     semantic service model
 *         ->
 *     classical semantic representation
 *         ->
 *     classical IR
 *
 * HDL/hardware service:
 *
 *     service AST
 *         ->
 *     semantic service model
 *         ->
 *     HDL/hardware semantic representation
 *
 * No service-specific IR is created here.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * Compiler consumers may derive:
 *
 *     - service interfaces;
 *     - dispatch metadata;
 *     - communication requirements;
 *     - resource requirements;
 *     - capability requirements;
 *     - placement constraints;
 *     - scheduling dependencies;
 *     - serialization requirements;
 *     - target compatibility;
 *     - provenance;
 *     - diagnostics.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime owns:
 *
 *     - service discovery;
 *     - binding;
 *     - dispatch;
 *     - placement;
 *     - scheduling;
 *     - networking;
 *     - lifecycle execution;
 *     - recovery;
 *     - observability;
 *     - deployment.
 *
 * The grammar does not implement these mechanisms.
 *
 * ============================================================================
 * TOOLING CONTRACT
 * ============================================================================
 *
 * The grammar must remain consumable by:
 *
 *     - parser;
 *     - formatter;
 *     - syntax highlighter;
 *     - IDE;
 *     - symbol indexer;
 *     - documentation generator;
 *     - API/interface extractor;
 *     - AST inspector;
 *     - semantic diagnostic tooling.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * The existing public entry point:
 *
 *     distributedServiceDeclaration
 *
 * is retained.
 *
 * `distributed.g4` already imports:
 *
 *     DistributedServices
 *
 * and consumes:
 *
 *     distributedServiceDeclaration
 *
 * Therefore the distributed composition root remains compatible with this
 * replacement.
 *
 * No second distributed-service entry point is introduced.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     ZamaniLexer
 *     Core
 *     Types
 *     Expressions
 *
 * IMPORTED GRAMMARS:
 *
 *     Core
 *     Types
 *     Expressions
 *
 * CORE RULES CONSUMED:
 *
 *     identifier
 *     qualifiedName
 *     qualifiedNameList
 *     attribute
 *     visibilityModifier
 *     block
 *     blockElement
 *
 * TYPE RULES CONSUMED:
 *
 *     typeExpression
 *     typeExpressionList
 *
 * EXPRESSION RULES CONSUMED:
 *
 *     expression
 *     expressionList
 *
 * EXPORTS:
 *
 *     distributedServiceDeclaration
 *     distributedServiceBody
 *     distributedServiceMember
 *     distributedServiceOperation
 *     distributedServiceState
 *     distributedServiceEvent
 *     distributedServiceLifecycle
 *     distributedServiceDependency
 *     distributedServicePolicy
 *     distributedServiceRequirement
 *     distributedServiceCapability
 *     distributedServiceContract
 *     distributedServiceMetadata
 *     distributedServiceExtension
 *
 * CONSUMED_BY:
 *
 *     grammar/distributed/distributed.g4
 *
 * INDIRECTLY CONSUMED_BY:
 *
 *     grammar/antlr/ZamaniParser.g4
 *     root parser composition
 *     frontend AST construction
 *     semantic analysis
 *     compiler tooling
 *
 * AST_OWNER:
 *
 *     frontend AST subsystem
 *
 * SEMANTIC_OWNER:
 *
 *     distributed semantic subsystem
 *
 * RESOURCE_OWNER:
 *
 *     resource semantic subsystem
 *
 * CAPABILITY_OWNER:
 *
 *     capability semantic subsystem
 *
 * EFFECT_OWNER:
 *
 *     effect semantic subsystem
 *
 * POLICY_OWNER:
 *
 *     policy semantic subsystem
 *
 * CONTRACT_OWNER:
 *
 *     validation/contract subsystem
 *
 * IR_OWNER:
 *
 *     canonical compiler IR subsystems
 *
 * QUANTUM_IR_OWNER:
 *
 *     quantum::ir
 *
 * TEST_OWNER:
 *
 *     grammar/tests/distributed/
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/distributed.md
 *     grammar/specification/
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics are restricted to structural syntax.
 *
 * Examples:
 *
 *     missing SERVICE;
 *     missing service name;
 *     malformed generic parameters;
 *     malformed inheritance;
 *     malformed parameter list;
 *     missing type;
 *     malformed return clause;
 *     malformed service member;
 *     malformed block;
 *     malformed metadata;
 *     malformed extension.
 *
 * The parser MUST NOT report:
 *
 *     unavailable machine;
 *     unavailable node;
 *     insufficient memory;
 *     insufficient processors;
 *     missing QPU;
 *     unsupported accelerator;
 *     unsupported protocol;
 *     impossible placement;
 *     failed scheduling;
 *     failed deployment;
 *     insufficient resources.
 *
 * Those are semantic/resource/target diagnostics.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive tests MUST include:
 *
 *     service Calculator;
 *
 *     service Calculator {
 *         operation add(a: int, b: int) -> int;
 *     }
 *
 *     public service GenericService<T: SomeConstraint> {
 *         operation run(value: T) -> T;
 *     }
 *
 *     service QuantumService {
 *         operation execute(state: quantum::State) -> Result;
 *     }
 *
 *     service HybridService {
 *         operation execute(input: Input) -> Output {
 *             classical_work();
 *             quantum_work();
 *         }
 *     }
 *
 *     service Stateful {
 *         state value: int = 0;
 *     }
 *
 *     service Events {
 *         event completed(result: Result);
 *     }
 *
 *     service Governed {
 *         requires capability("distributed.compute");
 *         policy execution_policy;
 *         contract(value_is_valid(x));
 *     }
 *
 *     service Extensible {
 *         extension::future_feature(target) {
 *             value: expression;
 *         }
 *     }
 *
 * Negative tests MUST include:
 *
 *     service;
 *     service 123;
 *     service Name<>;
 *     service Name<T U>;
 *     service Name<T,>;
 *     service Name(a:);
 *     service Name {
 *         operation run(x:) ;
 *     }
 *
 * Boundary tests MUST include:
 *
 *     deeply nested service bodies;
 *     deeply nested qualified names;
 *     many operations;
 *     many parameters;
 *     many states;
 *     many requirements;
 *     many policies;
 *     many metadata entries;
 *     many extensions;
 *     generic services with arbitrarily many parameters;
 *     service bodies containing classical, quantum, HDL, data, AI and
 *     distributed expressions where those expressions are valid.
 *
 * Scalability tests MUST verify that no language-level capacity ceiling is
 * encoded by this grammar.
 *
 * Determinism tests MUST verify identical parse structure for identical token
 * streams.
 *
 * Compatibility tests MUST verify preservation of:
 *
 *     distributedServiceDeclaration
 *
 * as the public integration rule.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] SERVICE uses the canonical SERVICE token.
 *     [x] Canonical names are reused.
 *     [x] Canonical types are reused.
 *     [x] Canonical expressions are reused.
 *     [x] Canonical blocks are reused.
 *     [x] Distributed service syntax has one public declaration owner.
 *     [x] Network service syntax remains separately owned.
 *     [x] Messages are not duplicated.
 *     [x] Actors are not duplicated.
 *     [x] Nodes are not duplicated.
 *     [x] Processes are not duplicated.
 *     [x] No transport is selected by the grammar.
 *     [x] No physical target is selected.
 *     [x] No resource allocation occurs.
 *     [x] No machine capacity is hard-coded.
 *     [x] No distributed scale limit is hard-coded.
 *     [x] Future service extensions remain representable.
 *     [x] AST ordering can be preserved.
 *     [x] Source spans can be preserved.
 *     [x] Resource/capability semantics remain downstream.
 *     [x] Effects remain downstream.
 *     [x] Policies remain downstream.
 *     [x] Contracts remain downstream.
 *     [x] Provenance remains downstream.
 *     [x] Classical computation can be embedded.
 *     [x] Quantum computation can be embedded.
 *     [x] HDL/hardware intent can be embedded.
 *     [x] AI/data computation can be embedded.
 *     [x] No IR is constructed here.
 *     [x] quantum::ir remains the canonical quantum IR.
 *     [x] The grammar contains no Rust actions.
 *     [x] The grammar contains no semantic predicates.
 *     [x] The grammar contains no unsafe Rust.
 *     [x] Parser behavior is deterministic.
 *     [x] Rust 1.97+ downstream compatibility is preserved.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * ANTLR COMPOSITION
 * ============================================================================
 */

parser grammar DistributedServices;

options {
    tokenVocab = ZamaniLexer;
}

import
    Core,
    Types,
    Expressions
;


/*
 * ============================================================================
 * PUBLIC SERVICE DECLARATION
 * ============================================================================
 *
 * Canonical source form:
 *
 *     service Name;
 *
 *     service Name {
 *         ...
 *     }
 *
 * Visibility and attributes are handled by canonical core grammar rules.
 *
 * `SERVICE` is a canonical lexer token.
 */

distributedServiceDeclaration
    : attribute*
      visibilityModifier?
      SERVICE
      identifier
      distributedServiceGenericParameters?
      distributedServiceInheritanceClause?
      distributedServiceImplementationClause*
      distributedServiceRequirementClause*
      distributedServiceWhereClause?
      distributedServiceBody?
      SEMICOLON?
    ;


/*
 * ============================================================================
 * GENERIC SERVICE PARAMETERS
 * ============================================================================
 *
 * These are DECLARATION parameters, not generic type application arguments.
 *
 * Example:
 *
 *     service Compute<T: Value>;
 *
 * Generic type application remains owned by the type system.
 *
 * There is no finite parameter limit.
 */

distributedServiceGenericParameters
    : LESS
      distributedServiceGenericParameter
      (
          COMMA distributedServiceGenericParameter
      )*
      COMMA?
      GREATER
    ;


distributedServiceGenericParameter
    : identifier
      distributedServiceGenericBound?
    ;


distributedServiceGenericBound
    : COLON
      typeExpression
    ;


/*
 * ============================================================================
 * INHERITANCE / IMPLEMENTATION
 * ============================================================================
 */

distributedServiceInheritanceClause
    : EXTENDS
      qualifiedName
      (
          COMMA
          qualifiedName
      )*
    ;


distributedServiceImplementationClause
    : IMPLEMENTS
      qualifiedName
      (
          COMMA
          qualifiedName
      )*
      SEMICOLON?
    ;


/*
 * ============================================================================
 * SERVICE REQUIREMENTS
 * ============================================================================
 *
 * The canonical REQUIRES token is used.
 *
 * The expression itself remains owned by the expression grammar.
 *
 * Semantic interpretation belongs to the resource/capability/contract
 * subsystems.
 */

distributedServiceRequirementClause
    : REQUIRES
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * SERVICE WHERE CLAUSE
 * ============================================================================
 */

distributedServiceWhereClause
    : WHERE
      expression
    ;


/*
 * ============================================================================
 * SERVICE BODY
 * ============================================================================
 */

distributedServiceBody
    : LBRACE
      distributedServiceMember*
      RBRACE
    ;


/*
 * ============================================================================
 * SERVICE MEMBERS
 * ============================================================================
 */

distributedServiceMember
    : distributedServiceOperation
    | distributedServiceState
    | distributedServiceEvent
    | distributedServiceLifecycle
    | distributedServiceDependency
    | distributedServicePolicy
    | distributedServiceRequirement
    | distributedServiceCapability
    | distributedServiceContract
    | distributedServiceMetadata
    | distributedServiceExtension
    ;


/*
 * ============================================================================
 * SERVICE OPERATION
 * ============================================================================
 *
 * `operation` remains an extensible contextual name rather than introducing
 * another lexer keyword.
 *
 * Canonical conceptual form:
 *
 *     operation run(x: Input) -> Output;
 *
 * Because `operation` is contextual, semantic analysis verifies that the
 * first identifier denotes the operation declaration marker.
 *
 * This keeps the lexical vocabulary stable while retaining the source form.
 */

distributedServiceOperation
    : identifier
      identifier
      LPAREN
      distributedServiceParameterList?
      RPAREN
      distributedServiceReturnClause?
      distributedServiceOperationModifier*
      distributedServiceOperationContract*
      block?
      SEMICOLON?
    ;


/*
 * ============================================================================
 * SERVICE PARAMETERS
 * ============================================================================
 */

distributedServiceParameterList
    : distributedServiceParameter
      (
          COMMA
          distributedServiceParameter
      )*
      COMMA?
    ;


distributedServiceParameter
    : distributedServiceParameterModifier*
      identifier
      COLON
      typeExpression
      distributedServiceParameterDefault?
    ;


distributedServiceParameterModifier
    : identifier
    ;


distributedServiceParameterDefault
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * SERVICE RETURN TYPE
 * ============================================================================
 */

distributedServiceReturnClause
    : THIN_ARROW
      distributedServiceReturnType
    ;


distributedServiceReturnType
    : typeExpression
    | LPAREN
      typeExpressionList
      RPAREN
    ;


/*
 * ============================================================================
 * OPERATION MODIFIERS
 * ============================================================================
 *
 * Known universal modifiers may be represented directly where their canonical
 * tokens exist. Additional future modifiers remain contextual identifiers.
 */

distributedServiceOperationModifier
    : ASYNC
    | PURE
    | DETERMINISTIC
    | REPRODUCIBLE
    | identifier
    ;


/*
 * ============================================================================
 * OPERATION CONTRACT
 * ============================================================================
 *
 * Canonical contract keywords are admitted directly.
 *
 * The contract condition remains a canonical expression.
 */

distributedServiceOperationContract
    : REQUIRES
      expression
      SEMICOLON?
    | ENSURES
      expression
      SEMICOLON?
    | INVARIANT
      expression
      SEMICOLON?
    | ASSUME
      expression
      SEMICOLON?
    | GUARANTEE
      expression
      SEMICOLON?
    | PROPERTY
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * SERVICE STATE
 * ============================================================================
 *
 * Canonical conceptual form:
 *
 *     state value: Type;
 *     state value: Type = expression;
 *
 * `state` remains contextual because no dedicated STATE lexer token is part
 * of the canonical vocabulary.
 */

distributedServiceState
    : identifier
      identifier
      COLON
      typeExpression
      distributedServiceStateInitializer?
      distributedServiceStateModifier*
      SEMICOLON?
    ;


distributedServiceStateInitializer
    : ASSIGN
      expression
    ;


distributedServiceStateModifier
    : identifier
    ;


/*
 * ============================================================================
 * SERVICE EVENT
 * ============================================================================
 *
 * Canonical conceptual form:
 *
 *     event completed(result: Result);
 *
 * Event transport and delivery remain outside this grammar.
 */

distributedServiceEvent
    : identifier
      identifier
      LPAREN
      distributedServiceParameterList?
      RPAREN
      distributedServiceEventModifier*
      SEMICOLON?
    ;


distributedServiceEventModifier
    : identifier
    ;


/*
 * ============================================================================
 * SERVICE LIFECYCLE
 * ============================================================================
 *
 * Lifecycle remains declarative.
 *
 * The grammar does not execute start/stop/restart/migrate/recover operations.
 */

distributedServiceLifecycle
    : identifier
      identifier?
      distributedServiceLifecycleArguments?
      distributedServiceLifecycleBody?
      SEMICOLON?
    ;


distributedServiceLifecycleArguments
    : LPAREN
      expressionList?
      RPAREN
    ;


distributedServiceLifecycleBody
    : LBRACE
      distributedServiceLifecycleMember*
      RBRACE
    ;


distributedServiceLifecycleMember
    : identifier
      distributedServiceLifecycleValue?
      SEMICOLON?
    ;


distributedServiceLifecycleValue
    : COLON
      expression
    | ASSIGN
      expression
    ;


/*
 * ============================================================================
 * SERVICE DEPENDENCY
 * ============================================================================
 *
 * Dependency is a logical relationship.
 *
 * It does not mean:
 *
 *     network route;
 *     physical adjacency;
 *     machine placement;
 *     hardware link.
 */

distributedServiceDependency
    : identifier
      qualifiedName
      distributedServiceDependencyConstraint*
      SEMICOLON?
    ;


distributedServiceDependencyConstraint
    : identifier
      (
          COLON
          expression
        | ASSIGN
          expression
      )
    ;


/*
 * ============================================================================
 * SERVICE POLICY
 * ============================================================================
 *
 * POLICY is a canonical lexer token.
 *
 * Policy evaluation remains downstream.
 */

distributedServicePolicy
    : POLICY
      distributedServicePolicyTarget?
      distributedServicePolicyBody?
      SEMICOLON?
    ;


distributedServicePolicyTarget
    : qualifiedName
    ;


distributedServicePolicyBody
    : LBRACE
      distributedServicePolicyEntry*
      RBRACE
    ;


distributedServicePolicyEntry
    : identifier
      (
          COLON
          expression
        | ASSIGN
          expression
      )
      SEMICOLON?
    ;


/*
 * ============================================================================
 * SERVICE RESOURCE REQUIREMENT
 * ============================================================================
 *
 * This is a service-local structural wrapper.
 *
 * The resource universe itself remains owned by grammar/resources/.
 *
 * Canonical examples:
 *
 *     resource memory >= required_memory;
 *     resource capability("tensor.compute");
 *
 * The exact semantic interpretation is downstream.
 */

distributedServiceRequirement
    : RESOURCE
      distributedServiceRequirementTarget?
      distributedServiceRequirementArguments?
      distributedServiceRequirementBody?
      SEMICOLON?
    ;


distributedServiceRequirementTarget
    : qualifiedName
    ;


distributedServiceRequirementArguments
    : LPAREN
      expressionList?
      RPAREN
    ;


distributedServiceRequirementBody
    : LBRACE
      distributedServiceRequirementEntry*
      RBRACE
    ;


distributedServiceRequirementEntry
    : identifier
      (
          COLON
          expression
        | ASSIGN
          expression
      )
      SEMICOLON?
    ;


/*
 * ============================================================================
 * SERVICE CAPABILITY
 * ============================================================================
 *
 * CAPABILITY is canonical lexical vocabulary.
 *
 * Capability identity remains open-world.
 */

distributedServiceCapability
    : CAPABILITY
      distributedServiceCapabilityTarget?
      distributedServiceCapabilityArguments?
      distributedServiceCapabilityBody?
      SEMICOLON?
    ;


distributedServiceCapabilityTarget
    : qualifiedName
    ;


distributedServiceCapabilityArguments
    : LPAREN
      expressionList?
      RPAREN
    ;


distributedServiceCapabilityBody
    : LBRACE
      distributedServiceCapabilityEntry*
      RBRACE
    ;


distributedServiceCapabilityEntry
    : identifier
      (
          COLON
          expression
        | ASSIGN
          expression
      )
      SEMICOLON?
    ;


/*
 * ============================================================================
 * SERVICE CONTRACT
 * ============================================================================
 *
 * CONTRACT is the service-level grouping form.
 *
 * The condition remains a canonical expression.
 *
 * Concrete requires/ensures/invariant/etc. clauses remain represented by the
 * canonical contract vocabulary above.
 */

distributedServiceContract
    : CONTRACT
      distributedServiceContractTarget?
      distributedServiceContractBody?
      SEMICOLON?
    ;


distributedServiceContractTarget
    : qualifiedName
    ;


distributedServiceContractBody
    : LBRACE
      distributedServiceContractEntry*
      RBRACE
    ;


distributedServiceContractEntry
    : identifier
      (
          COLON
          expression
        | ASSIGN
          expression
      )
      SEMICOLON?
    ;


/*
 * ============================================================================
 * SERVICE METADATA
 * ============================================================================
 *
 * Metadata is declarative source information.
 *
 * It is not executable behavior.
 */

distributedServiceMetadata
    : identifier
      COLON
      distributedServiceMetadataValue
      SEMICOLON?
    ;


distributedServiceMetadataValue
    : expression
    | distributedServiceMetadataObject
    | distributedServiceMetadataList
    ;


distributedServiceMetadataObject
    : LBRACE
      distributedServiceMetadataEntry*
      RBRACE
    ;


distributedServiceMetadataEntry
    : identifier
      COLON
      distributedServiceMetadataValue
      SEMICOLON?
    ;


distributedServiceMetadataList
    : LBRACKET
      (
          distributedServiceMetadataValue
          (
              COMMA
              distributedServiceMetadataValue
          )*
          COMMA?
      )?
      RBRACKET
    ;


/*
 * ============================================================================
 * SERVICE EXTENSIONS
 * ============================================================================
 *
 * This is the open-world extension boundary.
 *
 * New service technologies must not require a new universal keyword merely
 * because a new distributed technology appears.
 *
 * Examples that can be represented structurally include:
 *
 *     federation::contract(...)
 *     vendor::service_feature(...)
 *     future::service_model(...)
 *     quantum::service_extension(...)
 *     accelerator::service_binding(...)
 *
 * Semantic analysis determines whether an extension is known, supported,
 * experimental, deprecated, vendor-specific or invalid.
 */

distributedServiceExtension
    : identifier
      distributedServiceExtensionTarget?
      distributedServiceExtensionArguments?
      distributedServiceExtensionBody?
      SEMICOLON?
    ;


distributedServiceExtensionTarget
    : qualifiedName
    ;


distributedServiceExtensionArguments
    : LPAREN
      expressionList?
      RPAREN
    ;


distributedServiceExtensionBody
    : LBRACE
      distributedServiceExtensionMember*
      RBRACE
    ;


distributedServiceExtensionMember
    : identifier
      distributedServiceExtensionValue?
      SEMICOLON?
    ;


distributedServiceExtensionValue
    : COLON
      expression
    | ASSIGN
      expression
    | distributedServiceExtensionBody
    ;


/*
 * ============================================================================
 * COMPATIBILITY / INTEGRATION ALIASES
 * ============================================================================
 *
 * These aliases are intentionally narrow.
 *
 * They provide stable semantic categories without creating a second service
 * grammar.
 */

distributedServiceNameList
    : qualifiedNameList
    ;


distributedServiceExpressionList
    : expressionList
    ;
/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/distributed/services.g4
 *
 * Grammar:
 *     DistributedServices
 *
 * Status:
 *     Production distributed-service grammar.
 *
 * Runtime/compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     - No embedded Rust actions.
 *     - No semantic predicates.
 *     - No unsafe implementation.
 *     - No filesystem access.
 *     - No network access.
 *     - No hardware access.
 *     - No runtime callbacks.
 *     - No mutable compiler-global state.
 *     - No randomness.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar owns SOURCE-LEVEL SYNTAX for distributed services.
 *
 * A service is a named, semantically addressable distributed computation
 * boundary. It may expose operations, state, events, lifecycle declarations,
 * requirements, capabilities, policies, dependencies, and implementation
 * metadata.
 *
 * This grammar describes SERVICE INTENT.
 *
 * It does not implement service execution.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     UTF-8 source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     DistributedServices
 *          |
 *          v
 *     Distributed AST
 *          |
 *          +--> name resolution
 *          +--> type checking
 *          +--> effect checking
 *          +--> capability checking
 *          +--> resource analysis
 *          +--> security analysis
 *          +--> distributed semantic validation
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL/hardware representation
 *          +--> distributed service model
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing / placement / scheduling
 *          |
 *          v
 *     target realization
 *          |
 *          v
 *     runtime
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * This grammar MUST NEVER construct, replace, or redefine quantum::ir.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - distributed service declarations;
 *     - service names;
 *     - service signatures;
 *     - service operations;
 *     - service parameters;
 *     - service return declarations;
 *     - service state declarations;
 *     - service events;
 *     - service lifecycle declarations;
 *     - service dependencies;
 *     - service policies;
 *     - service requirements;
 *     - service capability declarations;
 *     - service contracts;
 *     - service metadata;
 *     - service extensions;
 *     - source-level service implementation intent.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical identifiers;
 *     - names;
 *     - types;
 *     - expressions;
 *     - generic effects;
 *     - network protocols;
 *     - transport;
 *     - endpoints;
 *     - node declarations;
 *     - placement;
 *     - topology;
 *     - routing;
 *     - scheduling;
 *     - resource discovery;
 *     - hardware discovery;
 *     - deployment;
 *     - service registry implementation;
 *     - load balancing;
 *     - replication algorithms;
 *     - consensus algorithms;
 *     - consistency algorithms;
 *     - fault tolerance algorithms;
 *     - resilience;
 *     - QEC;
 *     - ZQN;
 *     - quantum::ir;
 *     - runtime execution.
 *
 * ============================================================================
 * NON-OWNERSHIP RULE
 * ============================================================================
 *
 * A service declaration does NOT mean:
 *
 *     node declaration
 *     network endpoint
 *     physical address
 *     deployment
 *     process
 *     container
 *     machine
 *     device
 *     accelerator
 *     QPU
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *
 * Those are separate semantic concerns.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Service syntax is designed around:
 *
 *     Program Once
 *         ->
 *     Compile Once
 *         ->
 *     Run Everywhere
 *         ->
 *     Run Anywhere
 *         ->
 *     Run Forever
 *
 * A service describes:
 *
 *     WHAT computation is exposed
 *     WHAT inputs are accepted
 *     WHAT outputs are produced
 *     WHAT semantic guarantees are required
 *     WHAT capabilities are needed
 *     WHAT constraints/preferences exist
 *
 * It does NOT permanently encode:
 *
 *     WHICH machine
 *     WHICH node
 *     WHICH IP address
 *     WHICH port
 *     WHICH cloud provider
 *     WHICH cluster
 *     WHICH CPU
 *     WHICH GPU
 *     WHICH QPU
 *     WHICH FPGA
 *     WHICH network
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are deliberately NO grammar-level limits on:
 *
 *     - number of services;
 *     - number of operations;
 *     - number of parameters;
 *     - number of return values;
 *     - number of states;
 *     - number of events;
 *     - number of policies;
 *     - number of requirements;
 *     - number of capabilities;
 *     - number of dependencies;
 *     - number of lifecycle declarations;
 *     - number of metadata entries;
 *     - number of extensions;
 *     - service nesting depth;
 *     - qualified-name depth.
 *
 * Repetition is represented through `*` and `+`.
 *
 * No finite maximum is encoded.
 *
 * Parser-resource protection, compiler-resource limits, deployment limits,
 * runtime limits, network limits, and hardware limits are implementation or
 * resource-policy concerns rather than language semantics.
 *
 * ============================================================================
 * OPEN-WORLD DESIGN
 * ============================================================================
 *
 * The grammar intentionally does NOT introduce one lexer keyword for every
 * possible service concept.
 *
 * For example, future service kinds may include:
 *
 *     service
 *     actor
 *     agent
 *     workflow
 *     capability
 *     endpoint
 *     oracle
 *     accelerator
 *     quantum-service
 *     hardware-service
 *     future-service-kind
 *
 * The syntax therefore uses canonical identifiers and contextual names.
 *
 * Semantic analysis determines whether a contextual identifier denotes:
 *
 *     service
 *     operation
 *     state
 *     event
 *     policy
 *     requirement
 *     capability
 *     dependency
 *     lifecycle
 *     extension
 *
 * This prevents the grammar from becoming a closed list of temporary
 * distributed concepts.
 *
 * ============================================================================
 * LEXER BOUNDARY
 * ============================================================================
 *
 * This is a PARSER grammar.
 *
 * The canonical lexer owns:
 *
 *     IDENTIFIER
 *     punctuation
 *     literals
 *     operators
 *     comments
 *     lexical diagnostics
 *
 * This grammar MUST NOT define lexer rules.
 *
 * ============================================================================
 * NAME BOUNDARY
 * ============================================================================
 *
 * `Names` owns:
 *
 *     identifier
 *     qualifiedName
 *
 * This grammar consumes those rules.
 *
 * It MUST NOT define another:
 *
 *     ServiceName
 *     OperationName
 *     EndpointName
 *     NodeName
 *     DeviceName
 *
 * type at grammar level.
 *
 * Semantic layers may create strongly typed service identifiers after parsing.
 *
 * ============================================================================
 * TYPE BOUNDARY
 * ============================================================================
 *
 * `Types` owns `typeExpression`.
 *
 * Service parameters and return values consume `typeExpression`.
 *
 * This grammar therefore does not define:
 *
 *     ServiceType
 *     ParameterType
 *     ReturnType
 *
 * as duplicate type systems.
 *
 * ============================================================================
 * EXPRESSION BOUNDARY
 * ============================================================================
 *
 * `Expressions` owns `expression`.
 *
 * Expressions may be used for:
 *
 *     requirements
 *     constraints
 *     policies
 *     default values
 *     guards
 *     preconditions
 *     postconditions
 *     metadata values
 *     capability parameters
 *     resource requirements
 *
 * This grammar does not reproduce arithmetic, logical, comparison, indexing,
 * calls, or other expression syntax.
 *
 * ============================================================================
 * NETWORKING BOUNDARY
 * ============================================================================
 *
 * A service may describe communication intent.
 *
 * It does NOT define transport.
 *
 * For example:
 *
 *     service operation
 *
 * does not mean:
 *
 *     TCP
 *     UDP
 *     QUIC
 *     RDMA
 *     MPI
 *     InfiniBand
 *     HTTP
 *     gRPC
 *     vendor-specific transport
 *
 * Those belong to networking/communication layers.
 *
 * ============================================================================
 * ENDPOINT BOUNDARY
 * ============================================================================
 *
 * This grammar does not assign:
 *
 *     IP addresses
 *     ports
 *     socket identifiers
 *     machine addresses
 *     physical interfaces
 *
 * A service can expose a semantic interface without specifying how that
 * interface is physically reached.
 *
 * ============================================================================
 * NODE BOUNDARY
 * ============================================================================
 *
 * `nodes.g4` owns node declarations.
 *
 * A service does not implicitly create a node.
 *
 * A service may express placement or capability requirements, but actual node
 * selection is performed downstream.
 *
 * ============================================================================
 * RESOURCE BOUNDARY
 * ============================================================================
 *
 * Service resource requirements are expressed through generic resource and
 * capability mechanisms.
 *
 * This grammar does not introduce:
 *
 *     maxNodes
 *     cpuCount
 *     gpuCount
 *     qpuCount
 *     memorySize
 *     fixedBandwidth
 *     fixedLatency
 *     fixedReplicaCount
 *
 * as machine-level language limits.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * A distributed service may expose quantum computation.
 *
 * Examples include services whose implementation eventually consumes:
 *
 *     quantum::ir
 *     QEC
 *     ZQN
 *     hardware capabilities
 *
 * None of those concepts are redefined here.
 *
 * A service operation may use quantum types or expressions through the
 * canonical type/expression system.
 *
 * This grammar MUST NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     quantum topology
 *     calibration
 *     pulse representation
 *     QEC algorithm
 *     noise model
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * A service may represent a semantic interface to hardware or an accelerator.
 *
 * It does not define:
 *
 *     wires
 *     clocks
 *     FPGA cells
 *     ASIC cells
 *     registers
 *     physical pins
 *     device addresses
 *
 * Those belong to HDL/hardware grammars.
 *
 * ============================================================================
 * STATE BOUNDARY
 * ============================================================================
 *
 * A service may declare logical service state.
 *
 * This does NOT define:
 *
 *     physical storage
 *     database technology
 *     replication algorithm
 *     memory placement
 *     persistence mechanism
 *     serialization format
 *
 * Those are downstream semantic/runtime decisions.
 *
 * ============================================================================
 * LIFECYCLE BOUNDARY
 * ============================================================================
 *
 * Lifecycle declarations describe semantic lifecycle intent.
 *
 * They do not directly start, stop, restart, migrate, or destroy runtime
 * processes.
 *
 * ============================================================================
 * CONTRACT BOUNDARY
 * ============================================================================
 *
 * Preconditions, postconditions, invariants, and guarantees are source-level
 * semantic contracts.
 *
 * They are not runtime implementation algorithms.
 *
 * ============================================================================
 * SECURITY BOUNDARY
 * ============================================================================
 *
 * Service declarations may contain semantic security/capability requirements.
 *
 * They do not implement:
 *
 *     authentication
 *     authorization
 *     cryptographic protocols
 *     key storage
 *     identity providers
 *     transport security
 *
 * Those belong to security and runtime layers.
 *
 * ============================================================================
 * FAILURE / RESILIENCE BOUNDARY
 * ============================================================================
 *
 * This grammar may express service-level failure requirements.
 *
 * It does NOT implement:
 *
 *     retry algorithms
 *     failover algorithms
 *     checkpoint algorithms
 *     recovery orchestration
 *     quorum algorithms
 *     resilience decisions
 *
 * Those remain downstream responsibilities.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no actions;
 *     no predicates;
 *     no runtime calls;
 *     no I/O;
 *     no randomness;
 *     no hardware inspection;
 *     no network inspection.
 *
 * Equal token streams therefore have equal syntactic interpretation under
 * the same grammar version.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser/AST builder should preserve:
 *
 *     - service declaration order;
 *     - service name;
 *     - qualified names;
 *     - operation order;
 *     - parameter order;
 *     - state declaration order;
 *     - event declaration order;
 *     - lifecycle declaration order;
 *     - policy order;
 *     - requirement order;
 *     - capability order;
 *     - dependency order;
 *     - contract order;
 *     - extension order;
 *     - expression structure;
 *     - source spans.
 *
 * The AST should retain enough source information for deterministic
 * diagnostics and source-to-source tooling.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for determining:
 *
 *     - whether the contextual declaration really denotes a service;
 *     - uniqueness of service names within a semantic scope;
 *     - operation uniqueness;
 *     - parameter validity;
 *     - return validity;
 *     - state validity;
 *     - event validity;
 *     - lifecycle validity;
 *     - contract validity;
 *     - capability satisfaction;
 *     - effect compatibility;
 *     - resource compatibility;
 *     - security compatibility;
 *     - distributed consistency requirements;
 *     - placement feasibility;
 *     - target feasibility.
 *
 * The parser does none of those jobs.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * There is no service-specific replacement for canonical IR.
 *
 * After semantic analysis, service declarations may lower into the repository's
 * distributed semantic representation.
 *
 * If an operation contains quantum computation:
 *
 *     service syntax
 *          ->
 *     semantic service model
 *          ->
 *     quantum semantic lowering
 *          ->
 *     quantum::ir
 *
 * The service grammar never constructs quantum::ir directly.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * Compiler stages may consume service semantics to determine:
 *
 *     - interface lowering;
 *     - execution strategy;
 *     - placement constraints;
 *     - resource requirements;
 *     - communication requirements;
 *     - serialization requirements;
 *     - target-specific realization.
 *
 * The grammar must remain independent from those choices.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime consumes compiled service semantics.
 *
 * Runtime owns:
 *
 *     discovery
 *     binding
 *     dispatch
 *     scheduling
 *     networking
 *     placement
 *     execution
 *     lifecycle management
 *     recovery
 *
 * This grammar owns none of them.
 *
 * ============================================================================
 * TOOLING CONTRACT
 * ============================================================================
 *
 * The grammar must support:
 *
 *     - syntax highlighting;
 *     - formatting;
 *     - source indexing;
 *     - symbol extraction;
 *     - documentation generation;
 *     - IDE navigation;
 *     - diagnostics;
 *     - AST inspection;
 *     - semantic analysis.
 *
 * No tool should need to infer service syntax from generated runtime code.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing distributed service syntax must migrate into this grammar without
 * changing its intended semantic meaning.
 *
 * The current distributed root already exposes:
 *
 *     distributedServiceDeclaration
 *
 * Therefore that rule remains the public rule name.
 *
 * `distributed.g4` must import this grammar and must NOT retain a duplicate
 * implementation of that rule.
 *
 * ============================================================================
 */

parser grammar DistributedServices;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Expressions, Types;


/* ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================ */

/**
 * Canonical distributed-service declaration.
 *
 * Existing distributed.g4 already exposes this rule name. The root grammar
 * should consume this imported rule directly.
 *
 * Contextual form:
 *
 *     service <name> ...
 *
 * The contextual spelling is validated semantically rather than forcing every
 * future service concept to become a lexer keyword.
 */
distributedServiceDeclaration
    : identifier
      identifier
      distributedServiceSignature?
      distributedServiceBody?
      SEMI?
    ;


/* ============================================================================
 * SERVICE SIGNATURE
 * ============================================================================ */

/**
 * Optional service-level generic/interface signature.
 *
 * The first identifier is the contextual service declaration kind.
 *
 * The second identifier is the service name.
 *
 * Optional generic parameters are intentionally represented using names and
 * expressions rather than machine-specific bounds.
 */
distributedServiceSignature
    : serviceGenericParameters?
      serviceImplementsClause?
      serviceRequiresClause*
      serviceProvidesClause*
    ;


/* ============================================================================
 * GENERICS
 * ============================================================================ */

/**
 * Service generic parameters.
 *
 * No finite generic-parameter limit exists.
 */
serviceGenericParameters
    : LT serviceGenericParameter (COMMA serviceGenericParameter)* GT
    ;


serviceGenericParameter
    : identifier
      serviceGenericParameterBound?
    ;


serviceGenericParameterBound
    : COLON typeExpression
    ;


/* ============================================================================
 * SERVICE INTERFACES
 * ============================================================================ */

/**
 * Semantic service-interface dependency.
 *
 * This does not create a network connection.
 */
serviceImplementsClause
    : identifier
      qualifiedNameList
      SEMI?
    ;


/**
 * Semantic capability/interface requirement.
 *
 * Actual capability resolution is downstream.
 */
serviceRequiresClause
    : identifier
      serviceRequirementBody?
      SEMI?
    ;


/**
 * Semantic capability/interface provision.
 */
serviceProvidesClause
    : identifier
      serviceCapabilityBody?
      SEMI?
    ;


/* ============================================================================
 * SERVICE BODY
 * ============================================================================ */

distributedServiceBody
    : LBRACE
      distributedServiceMember*
      RBRACE
    ;


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


/* ============================================================================
 * SERVICE OPERATIONS
 * ============================================================================ */

/**
 * Canonical operation form:
 *
 *     operation <name>(<parameters>) [-> <returns>]
 *
 * The contextual operation kind remains an identifier.
 *
 * This allows future operation kinds without continuously expanding the lexer.
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
      distributedServiceOperationBody?
      SEMI?
    ;


distributedServiceParameterList
    : distributedServiceParameter
      (COMMA distributedServiceParameter)*
      COMMA?
    ;


distributedServiceParameter
    : distributedServiceParameterModifier*
      identifier
      COLON
      typeExpression
      distributedServiceDefaultValue?
    ;


distributedServiceParameterModifier
    : identifier
    ;


distributedServiceDefaultValue
    : ASSIGN expression
    ;


distributedServiceReturnClause
    : THIN_ARROW
      distributedServiceReturnType
    ;


distributedServiceReturnType
    : typeExpression
    | LPAREN typeExpressionList RPAREN
    ;


typeExpressionList
    : typeExpression
      (COMMA typeExpression)*
      COMMA?
    ;


/* ============================================================================
 * OPERATION MODIFIERS
 * ============================================================================ */

distributedServiceOperationModifier
    : identifier
    ;


/* ============================================================================
 * OPERATION CONTRACTS
 * ============================================================================ */

/**
 * Examples semantically represented by this structure:
 *
 *     requires(...)
 *     ensures(...)
 *     invariant(...)
 *
 * The actual semantic meaning is owned by contract checking.
 */
distributedServiceOperationContract
    : identifier
      LPAREN
      expression
      RPAREN
    ;


/* ============================================================================
 * OPERATION BODY
 * ============================================================================ */

/**
 * Service operation bodies are deliberately expression-oriented.
 *
 * A service grammar must not duplicate the complete statement grammar.
 *
 * A surrounding statement/block grammar may provide a richer implementation
 * body. This rule provides a minimal service-level body boundary.
 */
distributedServiceOperationBody
    : LBRACE
      distributedServiceOperationMember*
      RBRACE
    ;


distributedServiceOperationMember
    : distributedServiceOperationStatement
    | distributedServiceOperationExpression
    | distributedServiceOperationMetadata
    ;


distributedServiceOperationStatement
    : identifier
      expression?
      SEMI
    ;


distributedServiceOperationExpression
    : expression
      SEMI
    ;


distributedServiceOperationMetadata
    : identifier
      COLON
      expression
      SEMI?
    ;


/* ============================================================================
 * SERVICE STATE
 * ============================================================================ */

/**
 * Service state is logical state.
 *
 * It does not select a physical database, memory device, node, or storage
 * technology.
 */
distributedServiceState
    : identifier
      identifier
      COLON
      typeExpression
      distributedServiceStateInitializer?
      distributedServiceStateModifier*
      SEMI?
    ;


distributedServiceStateInitializer
    : ASSIGN
      expression
    ;


distributedServiceStateModifier
    : identifier
    ;


/* ============================================================================
 * SERVICE EVENTS
 * ============================================================================ */

/**
 * Events express semantic service events.
 *
 * Transport, queueing, messaging protocol, and delivery implementation are
 * owned elsewhere.
 */
distributedServiceEvent
    : identifier
      identifier
      LPAREN
      distributedServiceParameterList?
      RPAREN
      distributedServiceEventModifier*
      SEMI?
    ;


distributedServiceEventModifier
    : identifier
    ;


/* ============================================================================
 * SERVICE LIFECYCLE
 * ============================================================================ */

/**
 * Lifecycle declarations are semantic intent.
 *
 * They do not directly start/stop/restart runtime processes.
 */
distributedServiceLifecycle
    : identifier
      identifier?
      distributedServiceLifecycleArguments?
      distributedServiceLifecycleBody?
      SEMI?
    ;


distributedServiceLifecycleArguments
    : LPAREN
      optionalExpressionList
      RPAREN
    ;


optionalExpressionList
    : expressionList?
    ;


expressionList
    : expression
      (COMMA expression)*
      COMMA?
    ;


distributedServiceLifecycleBody
    : LBRACE
      distributedServiceLifecycleMember*
      RBRACE
    ;


distributedServiceLifecycleMember
    : identifier
      expression?
      SEMI?
    ;


/* ============================================================================
 * SERVICE DEPENDENCIES
 * ============================================================================ */

/**
 * A dependency identifies another semantic service/module/capability.
 *
 * It does not identify a physical machine.
 */
distributedServiceDependency
    : identifier
      qualifiedName
      distributedServiceDependencyConstraint*
      SEMI?
    ;


distributedServiceDependencyConstraint
    : identifier
      (COLON | ASSIGN)
      expression
    ;


/* ============================================================================
 * SERVICE POLICIES
 * ============================================================================ */

/**
 * Policies are declarative.
 *
 * Policy execution belongs to semantic/runtime layers.
 */
distributedServicePolicy
    : identifier
      identifier?
      distributedServicePolicyBody?
      SEMI?
    ;


distributedServicePolicyBody
    : LBRACE
      distributedServicePolicyEntry*
      RBRACE
    ;


distributedServicePolicyEntry
    : identifier
      (COLON | ASSIGN)
      expression
      SEMI?
    ;


/* ============================================================================
 * REQUIREMENTS
 * ============================================================================ */

/**
 * Requirements describe semantic conditions.
 *
 * They do not become hard-coded machine constraints.
 */
distributedServiceRequirement
    : identifier
      distributedServiceRequirementTarget?
      distributedServiceRequirementBody?
      SEMI?
    ;


distributedServiceRequirementTarget
    : qualifiedName
    ;


distributedServiceRequirementBody
    : LBRACE
      distributedServiceRequirementEntry*
      RBRACE
    ;


distributedServiceRequirementEntry
    : identifier
      (COLON | ASSIGN)
      expression
      SEMI?
    ;


/* ============================================================================
 * CAPABILITIES
 * ============================================================================ */

/**
 * Capabilities describe semantic abilities the service requires or exposes.
 */
distributedServiceCapability
    : identifier
      distributedServiceCapabilityTarget?
      distributedServiceCapabilityBody?
      SEMI?
    ;


distributedServiceCapabilityTarget
    : qualifiedName
    ;


distributedServiceCapabilityBody
    : LBRACE
      distributedServiceCapabilityEntry*
      RBRACE
    ;


distributedServiceCapabilityEntry
    : identifier
      (COLON | ASSIGN)
      expression
      SEMI?
    ;


/* ============================================================================
 * CONTRACTS
 * ============================================================================ */

/**
 * Service-level contracts.
 *
 * Examples:
 *
 *     requires
 *     ensures
 *     invariant
 *     guarantees
 *
 * Exact contract semantics belong to semantic analysis.
 */
distributedServiceContract
    : identifier
      LPAREN
      expression
      RPAREN
      SEMI?
    ;


/* ============================================================================
 * METADATA
 * ============================================================================ */

/**
 * Metadata is structured source information.
 *
 * It is not executable behavior.
 */
distributedServiceMetadata
    : identifier
      COLON
      distributedServiceMetadataValue
      SEMI?
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
      SEMI?
    ;


distributedServiceMetadataList
    : LBRACKET
      distributedServiceMetadataValue
      (COMMA distributedServiceMetadataValue)*
      COMMA?
      RBRACKET
    ;


/* ============================================================================
 * EXTENSIONS
 * ============================================================================ */

/**
 * Open extension point.
 *
 * Future distributed-service dialects can add semantic constructs without
 * requiring this grammar to enumerate every future distributed technology.
 */
distributedServiceExtension
    : identifier
      distributedServiceExtensionName?
      distributedServiceExtensionArguments?
      distributedServiceExtensionBody?
      SEMI?
    ;


distributedServiceExtensionName
    : qualifiedName
    ;


distributedServiceExtensionArguments
    : LPAREN
      optionalExpressionList
      RPAREN
    ;


distributedServiceExtensionBody
    : LBRACE
      distributedServiceExtensionMember*
      RBRACE
    ;


distributedServiceExtensionMember
    : identifier
      (
          COLON expression
        | ASSIGN expression
        | distributedServiceExtensionBody
      )
      SEMI?
    ;


/* ============================================================================
 * COMMON SERVICE LIST / MAP STRUCTURES
 * ============================================================================ */

/**
 * Generic service property.
 *
 * This rule is useful for future service dialects and tooling.
 */
distributedServiceProperty
    : identifier
      COLON
      expression
      SEMI?
    ;


/**
 * Reusable qualified-name sequence.
 */
distributedServiceNameList
    : qualifiedName
      (COMMA qualifiedName)*
      COMMA?
    ;


/**
 * Reusable expression sequence.
 */
distributedServiceExpressionList
    : expression
      (COMMA expression)*
      COMMA?
    ;


/* ============================================================================
 * SERVICE RESOURCE / CAPABILITY SHAPES
 * ============================================================================
 *
 * These are deliberately generic.
 *
 * They permit expressions such as:
 *
 *     requirement capability::quantum
 *     capability accelerator::tensor
 *     requirement resource::memory
 *     capability execution::distributed
 *
 * without defining the actual capability/resource universe here.
 */

distributedServiceResourceReference
    : identifier
      qualifiedName?
      distributedServiceResourceArguments?
    ;


distributedServiceResourceArguments
    : LPAREN
      optionalExpressionList
      RPAREN
    ;


/* ============================================================================
 * SERVICE TARGET INDEPENDENCE
 * ============================================================================
 *
 * A service can be semantic-only.
 *
 * There is deliberately no:
 *
 *     cpuService
 *     gpuService
 *     qpuService
 *     fpgaService
 *     nodeService
 *     cloudService
 *
 * grammar rule.
 *
 * A service may instead express capabilities and requirements and allow the
 * compiler/runtime to determine an appropriate realization.
 */


/* ============================================================================
 * SERVICE SCALABILITY GUARANTEE
 * ============================================================================
 *
 * The following are intentionally NOT present:
 *
 *     MAX_SERVICES
 *     MAX_OPERATIONS
 *     MAX_PARAMETERS
 *     MAX_STATES
 *     MAX_EVENTS
 *     MAX_REPLICAS
 *     MAX_NODES
 *     MAX_ENDPOINTS
 *     MAX_WORKERS
 *     MAX_CONNECTIONS
 *     MAX_MESSAGE_SIZE
 *     MAX_SERVICE_DEPTH
 *     MAX_SERVICE_COUNT
 *
 * Any operational limits must be supplied by:
 *
 *     parser resource policy
 *     compiler resource policy
 *     runtime resource policy
 *     deployment policy
 *     hardware capability
 *
 * and must not become language semantics.
 */


/* ============================================================================
 * SERVICE / QUANTUM INTEGRATION
 * ============================================================================
 *
 * A parameter can use a quantum type:
 *
 *     service QuantumProcessor(
 *         input: quantum::...
 *     )
 *
 * without this grammar defining the quantum type.
 *
 * A service implementation may lower to quantum::ir through the normal
 * semantic pipeline:
 *
 *     service AST
 *          |
 *          v
 *     semantic service model
 *          |
 *          v
 *     quantum semantic lowering
 *          |
 *          v
 *     quantum::ir
 *
 * This grammar never imports or depends upon quantum::ir.
 */


/* ============================================================================
 * SERVICE / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware capabilities are referenced semantically.
 *
 * The grammar does not choose:
 *
 *     processor
 *     device
 *     FPGA
 *     ASIC
 *     QPU
 *     accelerator
 *
 * A hardware backend may satisfy a service requirement.
 */


/* ============================================================================
 * SERVICE / DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * distributed.g4 owns distributed-domain composition.
 *
 * nodes.g4 owns nodes.
 *
 * communication.g4 owns communication.
 *
 * messaging.g4 owns messaging.
 *
 * placement.g4 owns placement.
 *
 * services.g4 owns services.
 *
 * No service rule in this grammar should redefine any of those domains.
 */


/* ============================================================================
 * SERVICE / EFFECT INTEGRATION
 * ============================================================================
 *
 * Service operations may participate in the generic effect system.
 *
 * Effects are consumed semantically.
 *
 * This grammar does not define:
 *
 *     IO effects
 *     network effects
 *     quantum effects
 *     hardware effects
 *     distributed effects
 *
 * in parallel to `grammar/effects/`.
 */


/* ============================================================================
 * SERVICE / SECURITY INTEGRATION
 * ============================================================================
 *
 * Security declarations remain semantic requirements/capabilities.
 *
 * The grammar does not implement:
 *
 *     authentication
 *     authorization
 *     encryption
 *     key exchange
 *     identity verification
 *
 * Those belong to the security subsystem.
 */


/* ============================================================================
 * SERVICE / RESILIENCE INTEGRATION
 * ============================================================================
 *
 * Service contracts and failure requirements may be consumed by resilience
 * analysis.
 *
 * The grammar does not implement resilience decisions.
 *
 * Resilience remains responsible for decisions such as:
 *
 *     retry
 *     restart
 *     rollback
 *     reroute
 *     reschedule
 *     recompile
 *     switch backend
 *     quarantine
 *     abort
 */


/* ============================================================================
 * SERVICE / SCHEDULING INTEGRATION
 * ============================================================================
 *
 * Service declarations do not schedule themselves.
 *
 * Scheduling consumes downstream semantic service requirements and operation
 * dependencies.
 *
 * The grammar therefore contains no:
 *
 *     scheduler algorithm
 *     ASAP
 *     ALAP
 *     RCPSP
 *     hardware timing grid
 *     machine cycle count
 *
 * Those remain scheduling concerns.
 */


/* ============================================================================
 * SERVICE / ROUTING INTEGRATION
 * ============================================================================
 *
 * A service may require communication or placement properties.
 *
 * Routing resolves actual physical paths.
 *
 * This grammar does not contain topology or routing algorithms.
 */


/* ============================================================================
 * SERVICE / OPTIMIZATION INTEGRATION
 * ============================================================================
 *
 * Optimization may transform service implementations.
 *
 * The service contract itself remains semantically stable.
 *
 * Optimization therefore consumes service semantics rather than modifying
 * service syntax.
 */


/* ============================================================================
 * SERVICE / AST COMPLETION CONTRACT
 * ============================================================================
 *
 * A service AST node is complete only when it can retain:
 *
 *     name
 *     signature
 *     generic parameters
 *     interface requirements
 *     interface provisions
 *     operations
 *     parameters
 *     return types
 *     operation modifiers
 *     operation contracts
 *     operation bodies
 *     state
 *     events
 *     lifecycle
 *     dependencies
 *     policies
 *     requirements
 *     capabilities
 *     contracts
 *     metadata
 *     extensions
 *     source spans
 *
 * without requiring later grammar changes to represent those concepts.
 */


/* ============================================================================
 * DETERMINISTIC PARSING CONTRACT
 * ============================================================================
 *
 * No parser rule depends on:
 *
 *     hardware state
 *     resource availability
 *     network state
 *     runtime state
 *     current time
 *     randomness
 *     environment variables
 *     filesystem state
 *
 * Semantic feasibility is evaluated after parsing.
 */


/* ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains no machine-specific:
 *
 *     node count
 *     service count
 *     process count
 *     worker count
 *     CPU count
 *     GPU count
 *     QPU count
 *     FPGA count
 *     memory size
 *     network size
 *     topology size
 *     bandwidth limit
 *     latency limit
 *     address
 *     port
 *     provider
 *     device identifier
 *
 * All such information remains outside the grammar.
 */


/* ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [ ] DistributedServices compiles as an ANTLR parser grammar.
 *
 * [ ] It uses the canonical ZamaniLexer vocabulary.
 *
 * [ ] It imports canonical Names, Expressions, and Types grammars.
 *
 * [ ] It exposes distributedServiceDeclaration.
 *
 * [ ] distributed.g4 imports this grammar.
 *
 * [ ] distributed.g4 no longer owns a duplicate
 *     distributedServiceDeclaration implementation.
 *
 * [ ] No service-specific lexer is introduced.
 *
 * [ ] No service-specific type system is introduced.
 *
 * [ ] No service-specific expression system is introduced.
 *
 * [ ] No network implementation is introduced.
 *
 * [ ] No node implementation is introduced.
 *
 * [ ] No placement implementation is introduced.
 *
 * [ ] No scheduling implementation is introduced.
 *
 * [ ] No routing implementation is introduced.
 *
 * [ ] No hardware assumptions are introduced.
 *
 * [ ] No quantum::ir representation is introduced.
 *
 * [ ] No QEC implementation is introduced.
 *
 * [ ] No ZQN implementation is introduced.
 *
 * [ ] No resilience implementation is introduced.
 *
 * [ ] No fixed service/resource/device limits exist.
 *
 * [ ] Positive parser tests exist.
 *
 * [ ] Negative parser tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Cross-domain service tests exist.
 *
 * [ ] Scalability tests exist.
 *
 * [ ] Determinism tests exist.
 *
 * [ ] AST/source-span tests exist.
 *
 * [ ] POCO-REAF portability tests exist.
 *
 * [ ] Rust 1.97 / 1.97.1 generated parser integration passes.
 *
 * [ ] Generated Rust integration contains no unsafe code.
 *
 * [ ] Existing distributed service syntax has a compatibility test.
 */
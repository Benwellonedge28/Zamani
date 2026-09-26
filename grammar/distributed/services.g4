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
 *     PRODUCTION DISTRIBUTED-SERVICE PARSER GRAMMAR
 *
 * Baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust Edition 2021
 *     Safe Rust only; no unsafe Rust.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL SYNTAX of distributed computational
 * services.
 *
 * A service is a logical, named computational interface. A service may expose:
 *
 *     - operations;
 *     - parameters;
 *     - return contracts;
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
 * This grammar describes semantic intent.
 *
 * It does NOT implement:
 *
 *     - service discovery;
 *     - process creation;
 *     - node discovery;
 *     - network transport;
 *     - endpoint allocation;
 *     - routing;
 *     - placement;
 *     - scheduling;
 *     - deployment;
 *     - load balancing;
 *     - replication algorithms;
 *     - consensus algorithms;
 *     - consistency algorithms;
 *     - fault tolerance;
 *     - resilience;
 *     - hardware discovery;
 *     - QEC;
 *     - ZQN;
 *     - quantum::ir;
 *     - runtime execution.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical ZamaniLexer
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
 *          +--> security analysis
 *          +--> distributed semantic analysis
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical representation
 *          +--> quantum semantic representation
 *          |        |
 *          |        +--> quantum::ir
 *          |
 *          +--> HDL/hardware representation
 *          +--> distributed service representation
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
 * The grammar never constructs IR.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Service syntax is target-independent.
 *
 * The same service source may describe computation eventually realized on:
 *
 *     - one machine;
 *     - many machines;
 *     - embedded systems;
 *     - edge systems;
 *     - CPU systems;
 *     - GPU systems;
 *     - FPGA systems;
 *     - ASIC systems;
 *     - quantum systems;
 *     - quantum/classical systems;
 *     - clusters;
 *     - HPC systems;
 *     - clouds;
 *     - federated systems;
 *     - future computational substrates.
 *
 * Source syntax describes:
 *
 *     WHAT the service means.
 *
 * Downstream compilation/runtime determines:
 *
 *     HOW the service is realized.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are NO grammar-level finite limits on:
 *
 *     services
 *     operations
 *     parameters
 *     return values
 *     states
 *     events
 *     dependencies
 *     requirements
 *     capabilities
 *     policies
 *     contracts
 *     metadata entries
 *     extensions
 *     generic parameters
 *     qualified-name depth
 *     service nesting depth
 *
 * This grammar deliberately contains no:
 *
 *     MAX_SERVICES
 *     MAX_OPERATIONS
 *     MAX_PARAMETERS
 *     MAX_STATES
 *     MAX_EVENTS
 *     MAX_REPLICAS
 *     MAX_NODES
 *     MAX_WORKERS
 *     MAX_PROCESSES
 *     MAX_THREADS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_MEMORY
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICES
 *
 * Practical limits belong to parser-resource protection, compiler policy,
 * runtime resources, deployment resources, and actual target availability.
 *
 * ============================================================================
 * OPEN-WORLD DESIGN
 * ============================================================================
 *
 * Service kinds and service properties are intentionally contextual.
 *
 * The grammar does not require a new lexer keyword for every future service
 * abstraction.
 *
 * Examples that remain structurally representable include:
 *
 *     service
 *     actor
 *     agent
 *     workflow
 *     oracle
 *     accelerator
 *     quantum_service
 *     hardware_service
 *     federated_service
 *     future_service_kind
 *
 * Their semantic classification belongs downstream.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * Canonical lexical authority:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Canonical name authority:
 *
 *     grammar/core/names.g4
 *
 * Canonical expression authority:
 *
 *     grammar/expressions/expressions.g4
 *
 * Canonical type authority:
 *
 *     grammar/types/types.g4
 *
 * This file MUST NOT redefine:
 *
 *     IDENTIFIER
 *     identifier
 *     qualifiedName
 *     expression
 *     expressionList
 *     typeExpression
 *     operators
 *     punctuation
 *
 * ============================================================================
 * TOKEN COMPATIBILITY
 * ============================================================================
 *
 * Canonical parser token names are used.
 *
 * Examples:
 *
 *     LPAREN
 *     RPAREN
 *     LBRACE
 *     RBRACE
 *     LBRACKET
 *     RBRACKET
 *     COMMA
 *     COLON
 *     SEMICOLON
 *     ASSIGN
 *     LESS_THAN
 *     GREATER_THAN
 *     DOUBLE_COLON
 *     IDENTIFIER
 *
 * No aliases such as:
 *
 *     LT
 *     GT
 *     SEMI
 *     EQUALS
 *
 * are introduced here.
 *
 * ============================================================================
 * CONTEXTUAL KEYWORDS
 * ============================================================================
 *
 * The current canonical lexical architecture does not establish a dedicated
 * service keyword.
 *
 * Consequently:
 *
 *     service
 *     operation
 *     state
 *     event
 *     lifecycle
 *     requires
 *     provides
 *     capability
 *     policy
 *     contract
 *
 * remain ordinary identifiers at this grammar boundary unless and until the
 * canonical lexer deliberately reserves them.
 *
 * If a spelling becomes reserved later, that change must be coordinated
 * across:
 *
 *     lexer
 *     specification
 *     parser
 *     AST
 *     semantic analysis
 *     compatibility
 *     tests.
 *
 * ============================================================================
 * NAME BOUNDARY
 * ============================================================================
 *
 * `Names` owns:
 *
 *     identifier
 *     qualifiedName
 *     qualifiedNameList
 *
 * Service syntax consumes those rules.
 *
 * Service-specific name categories are deliberately NOT introduced.
 *
 * ============================================================================
 * TYPE BOUNDARY
 * ============================================================================
 *
 * `Types` owns:
 *
 *     typeExpression
 *     typeExpressionList
 *
 * This file consumes those rules and never creates a service-specific type
 * system.
 *
 * Consequently all of the following remain semantic type questions:
 *
 *     classical types
 *     quantum types
 *     tensor types
 *     memory types
 *     resource types
 *     hardware types
 *     future types.
 *
 * ============================================================================
 * EXPRESSION BOUNDARY
 * ============================================================================
 *
 * `Expressions` owns:
 *
 *     expression
 *     expressionList
 *
 * Service grammar consumes those rules directly.
 *
 * It therefore does not duplicate:
 *
 *     arithmetic
 *     comparison
 *     logical operations
 *     calls
 *     indexing
 *     member access
 *     assignment
 *     ranges
 *     quantum expressions
 *     tensor expressions.
 *
 * ============================================================================
 * DISTRIBUTED BOUNDARIES
 * ============================================================================
 *
 * `distributed.g4`
 *     owns distributed-domain composition.
 *
 * `nodes.g4`
 *     owns logical node syntax.
 *
 * `processes.g4`
 *     owns process syntax.
 *
 * `messaging.g4`
 *     owns message syntax.
 *
 * `communication.g4`
 *     owns communication intent.
 *
 * `placement.g4`
 *     owns placement intent.
 *
 * `services.g4`
 *     owns service syntax.
 *
 * No service construct should silently become a node, process, channel,
 * message, endpoint, or physical deployment object.
 *
 * ============================================================================
 * NETWORKING BOUNDARY
 * ============================================================================
 *
 * A distributed service may describe semantic communication requirements.
 *
 * This grammar does NOT select:
 *
 *     TCP
 *     UDP
 *     QUIC
 *     HTTP
 *     gRPC
 *     MPI
 *     RDMA
 *     InfiniBand
 *     vendor transport.
 *
 * Networking grammars and downstream semantic/runtime layers own transport.
 *
 * ============================================================================
 * HARDWARE / HDL BOUNDARY
 * ============================================================================
 *
 * A service may expose computation eventually implemented by:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     HDL module
 *
 * This grammar does not select physical hardware.
 *
 * There is no:
 *
 *     CPUService
 *     GPUService
 *     FPGAService
 *     QPUService
 *     PhysicalService
 *
 * grammar category.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * A service may accept or produce quantum values through the canonical type
 * and expression systems.
 *
 * Example conceptual form:
 *
 *     service QuantumService(
 *         state: quantum::State
 *     )
 *
 * Quantum semantics remain downstream.
 *
 * This file does NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     quantum topology
 *     pulse syntax
 *     calibration
 *     QEC algorithms
 *     noise models
 *     ZQN structures.
 *
 * If a service operation performs quantum computation:
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
 * ============================================================================
 * RESOURCE / CAPABILITY BOUNDARY
 * ============================================================================
 *
 * Requirements and capabilities describe semantic conditions.
 *
 * They are not physical allocation instructions.
 *
 * Examples:
 *
 *     requirement capability::quantum::measurement
 *     requirement resource::memory >= required_memory
 *     capability compute::tensor
 *
 * The actual interpretation belongs to resource/capability semantic systems.
 *
 * ============================================================================
 * SECURITY BOUNDARY
 * ============================================================================
 *
 * Service syntax may represent security requirements or policies.
 *
 * It does not implement:
 *
 *     authentication
 *     authorization
 *     encryption
 *     key management
 *     identity providers
 *     secure transport.
 *
 * ============================================================================
 * LIFECYCLE BOUNDARY
 * ============================================================================
 *
 * Lifecycle declarations express semantic lifecycle intent.
 *
 * They do not execute:
 *
 *     start
 *     stop
 *     restart
 *     migrate
 *     destroy
 *
 * at parse time.
 *
 * ============================================================================
 * CONTRACT BOUNDARY
 * ============================================================================
 *
 * Service contracts are source-level semantic contracts.
 *
 * Examples:
 *
 *     requires(...)
 *     ensures(...)
 *     invariant(...)
 *     guarantees(...)
 *
 * Their actual checking/execution is downstream.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no embedded actions;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware access;
 *     - no runtime callbacks;
 *     - no randomness;
 *     - no environment-dependent parsing.
 *
 * Parsing depends only on the supplied token stream and grammar version.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The AST builder must preserve, at minimum:
 *
 *     - service declaration order;
 *     - service declaration kind;
 *     - service name;
 *     - generic parameters;
 *     - implemented interfaces;
 *     - required capabilities/contracts;
 *     - provided capabilities/contracts;
 *     - operation order;
 *     - parameter order;
 *     - parameter modifiers;
 *     - parameter types;
 *     - default expressions;
 *     - return structure;
 *     - operation modifiers;
 *     - operation contracts;
 *     - operation body structure;
 *     - state declarations;
 *     - state initializers;
 *     - state modifiers;
 *     - events;
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
 * The AST must not resolve hardware or runtime behavior while parsing.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     - service-kind validation;
 *     - service-name uniqueness;
 *     - generic parameter validity;
 *     - interface resolution;
 *     - operation uniqueness;
 *     - parameter validation;
 *     - type resolution;
 *     - default-value checking;
 *     - return validation;
 *     - state validation;
 *     - event validation;
 *     - lifecycle validation;
 *     - dependency resolution;
 *     - capability resolution;
 *     - resource requirement resolution;
 *     - effect compatibility;
 *     - security policy validation;
 *     - contract validation;
 *     - distributed consistency validation;
 *     - target feasibility.
 *
 * The parser performs none of these semantic decisions.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file introduces NO service-specific IR.
 *
 * Parsed service syntax lowers through the repository's established semantic
 * architecture.
 *
 * Generic service:
 *
 *     service AST
 *          ->
 *     semantic service model
 *          ->
 *     canonical compiler representation
 *
 * Quantum-bearing service:
 *
 *     service AST
 *          ->
 *     semantic service model
 *          ->
 *     quantum semantic model
 *          ->
 *     quantum::ir
 *
 * `quantum::ir` remains the only canonical quantum IR.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * Compiler consumers may derive:
 *
 *     - service interfaces;
 *     - dispatch metadata;
 *     - resource requirements;
 *     - capability requirements;
 *     - communication requirements;
 *     - placement constraints;
 *     - scheduling dependencies;
 *     - serialization requirements;
 *     - target-specific lowering.
 *
 * The grammar does not select the realization.
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
 *     - scheduling;
 *     - placement;
 *     - networking;
 *     - lifecycle execution;
 *     - recovery;
 *     - observability;
 *     - deployment.
 *
 * The grammar contains none of these implementations.
 *
 * ============================================================================
 * TOOLING CONTRACT
 * ============================================================================
 *
 * The grammar must remain usable by:
 *
 *     - formatter;
 *     - syntax highlighter;
 *     - parser;
 *     - symbol indexer;
 *     - documentation generator;
 *     - IDE tooling;
 *     - AST inspection;
 *     - semantic diagnostics.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * The public rule:
 *
 *     distributedServiceDeclaration
 *
 * is retained.
 *
 * Existing consumers should migrate to this rule rather than creating another
 * distributed service grammar.
 *
 * `distributed.g4` must import this grammar and remove any duplicate service
 * declaration implementation.
 *
 * ============================================================================
 */

parser grammar DistributedServices;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Expressions, Types;


/* ============================================================================
 * PUBLIC SERVICE DECLARATION
 * ============================================================================
 *
 * Canonical structural form:
 *
 *     <service-kind> <service-name>
 *         <optional-signature>
 *         <optional-body>
 *         ;
 *
 * Examples:
 *
 *     service Calculator;
 *
 *     service Calculator {
 *         ...
 *     }
 *
 *     service QuantumService<T> {
 *         ...
 *     }
 *
 * The word `service` is intentionally contextual rather than a new lexer
 * keyword under the current lexical contract.
 */
distributedServiceDeclaration
    : identifier
      identifier
      distributedServiceSignature?
      distributedServiceBody?
      SEMICOLON?
    ;


/* ============================================================================
 * SERVICE SIGNATURE
 * ============================================================================ */

distributedServiceSignature
    : distributedServiceGenericParameters?
      distributedServiceImplementsClause*
      distributedServiceRequiresClause*
      distributedServiceProvidesClause*
    ;


/* ============================================================================
 * GENERIC PARAMETERS
 * ============================================================================ */

distributedServiceGenericParameters
    : LESS_THAN
      distributedServiceGenericParameter
      (
          COMMA distributedServiceGenericParameter
      )*
      COMMA?
      GREATER_THAN
    ;


distributedServiceGenericParameter
    : identifier
      distributedServiceGenericBound?
    ;


distributedServiceGenericBound
    : COLON
      typeExpression
    ;


/* ============================================================================
 * INTERFACE / IMPLEMENTATION CONTRACTS
 * ============================================================================ */

distributedServiceImplementsClause
    : identifier
      qualifiedNameList
      SEMICOLON?
    ;


distributedServiceRequiresClause
    : identifier
      distributedServiceClauseArguments?
      SEMICOLON?
    ;


distributedServiceProvidesClause
    : identifier
      distributedServiceClauseArguments?
      SEMICOLON?
    ;


distributedServiceClauseArguments
    : LPAREN
      expressionList?
      RPAREN
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
 * SERVICE OPERATION
 * ============================================================================
 *
 * Structural form:
 *
 *     <operation-kind> <name>(<parameters>) [return-clause] ...
 *
 * `operation` is contextual.
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
      SEMICOLON?
    ;


distributedServiceParameterList
    : distributedServiceParameter
      (
          COMMA distributedServiceParameter
      )*
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
    : ASSIGN
      expression
    ;


/* ============================================================================
 * RETURN DECLARATIONS
 * ============================================================================
 *
 * Both a single type and a parenthesized type list are supported.
 *
 * The underlying types remain owned by Types.
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


/* ============================================================================
 * OPERATION MODIFIERS
 * ============================================================================
 *
 * Modifiers are contextual names.
 *
 * Examples:
 *
 *     async
 *     pure
 *     idempotent
 *     streaming
 *     remote
 *     deterministic
 *     transactional
 *     stateful
 *     stateless
 *
 * Their semantic legality is checked downstream.
 */
distributedServiceOperationModifier
    : identifier
    ;


/* ============================================================================
 * OPERATION CONTRACTS
 * ============================================================================ */

distributedServiceOperationContract
    : identifier
      LPAREN
      expression
      RPAREN
      SEMICOLON?
    ;


/* ============================================================================
 * OPERATION BODY
 * ============================================================================
 *
 * The operation body is deliberately a structural service-body boundary.
 *
 * It does not duplicate the complete Zamani statement grammar.
 *
 * A later composition grammar may provide richer implementation-body
 * integration without changing the service declaration model.
 */
distributedServiceOperationBody
    : LBRACE
      distributedServiceOperationMember*
      RBRACE
    ;


distributedServiceOperationMember
    : distributedServiceOperationExpression
    | distributedServiceOperationProperty
    ;


distributedServiceOperationExpression
    : expression
      SEMICOLON
    ;


distributedServiceOperationProperty
    : identifier
      COLON
      expression
      SEMICOLON?
    ;


/* ============================================================================
 * SERVICE STATE
 * ============================================================================
 *
 * State is LOGICAL service state.
 *
 * It does not imply:
 *
 *     RAM
 *     database
 *     persistent storage
 *     node-local storage
 *     replicated storage
 *     physical memory
 *     hardware storage.
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


/* ============================================================================
 * SERVICE EVENTS
 * ============================================================================
 *
 * Event syntax represents a semantic event contract.
 *
 * Transport and queue implementation belong elsewhere.
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


/* ============================================================================
 * SERVICE LIFECYCLE
 * ============================================================================
 *
 * Lifecycle is declarative intent.
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
    : COLON expression
    | ASSIGN expression
    ;


/* ============================================================================
 * SERVICE DEPENDENCIES
 * ============================================================================
 *
 * Dependencies identify semantic dependencies.
 *
 * They do not identify machines or network addresses.
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
          COLON expression
        | ASSIGN expression
      )
    ;


/* ============================================================================
 * SERVICE POLICIES
 * ============================================================================
 */

distributedServicePolicy
    : identifier
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
          COLON expression
        | ASSIGN expression
      )
      SEMICOLON?
    ;


/* ============================================================================
 * SERVICE REQUIREMENTS
 * ============================================================================
 *
 * Requirements are semantic constraints/requirements.
 *
 * They are NOT hard-coded hardware limits.
 */
distributedServiceRequirement
    : identifier
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
          COLON expression
        | ASSIGN expression
      )
      SEMICOLON?
    ;


/* ============================================================================
 * SERVICE CAPABILITIES
 * ============================================================================
 */

distributedServiceCapability
    : identifier
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
          COLON expression
        | ASSIGN expression
      )
      SEMICOLON?
    ;


/* ============================================================================
 * SERVICE CONTRACTS
 * ============================================================================
 */

distributedServiceContract
    : identifier
      LPAREN
      expression
      RPAREN
      SEMICOLON?
    ;


/* ============================================================================
 * SERVICE METADATA
 * ============================================================================
 *
 * Metadata is structured source information.
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
              COMMA distributedServiceMetadataValue
          )*
          COMMA?
      )?
      RBRACKET
    ;


/* ============================================================================
 * SERVICE EXTENSIONS
 * ============================================================================
 *
 * This is the open-world extension point.
 *
 * New distributed technologies should first attempt to use this structural
 * extension point before introducing new core keywords.
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
    : COLON expression
    | ASSIGN expression
    | distributedServiceExtensionBody
    ;


/* ============================================================================
 * GENERIC SERVICE LISTS
 * ============================================================================
 *
 * These wrappers exist only where they provide service-domain semantic
 * context. General name/expression syntax remains owned by canonical grammar.
 */
distributedServiceNameList
    : qualifiedNameList
    ;


distributedServiceExpressionList
    : expressionList
    ;


/* ============================================================================
 * SERVICE RESOURCE / CAPABILITY REFERENCE
 * ============================================================================
 *
 * This is a structural wrapper around canonical names and expressions.
 *
 * It does not define the resource/capability universe.
 */
distributedServiceResourceReference
    : identifier
      qualifiedName?
      distributedServiceResourceArguments?
    ;


distributedServiceResourceArguments
    : LPAREN
      expressionList?
      RPAREN
    ;


/* ============================================================================
 * SERVICE TARGET / PORTABILITY INVARIANT
 * ============================================================================
 *
 * There is deliberately no syntax here for:
 *
 *     physical_cpu
 *     physical_gpu
 *     physical_fpga
 *     physical_qpu
 *     machine_id
 *     node_id
 *     device_id
 *     memory_bank
 *     socket_id
 *     ip_address
 *     mac_address
 *     provider_instance
 *
 * A service may express semantic requirements, capabilities, constraints,
 * preferences, and properties. Physical realization is downstream.
 *
 * ============================================================================
 * CROSS-DOMAIN INTEGRATION
 * ============================================================================
 *
 * Classical:
 *
 *     service -> classical semantic model
 *
 * Quantum:
 *
 *     service -> semantic service model -> quantum semantic lowering
 *             -> quantum::ir
 *
 * HDL:
 *
 *     service -> hardware/interface semantic model
 *
 * AI:
 *
 *     service -> model/agent semantic interface
 *
 * Data:
 *
 *     service -> data/schema semantic interface
 *
 * Networking:
 *
 *     service -> networking service contract
 *
 * Security:
 *
 *     service -> security requirement/capability analysis
 *
 * Distributed:
 *
 *     service -> distributed semantic model
 *
 * Memory/concurrency/effects:
 *
 *     service -> existing generic semantic systems
 *
 * No domain creates a second service grammar authority.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     NO hardware capacity constants.
 *     NO service-count constants.
 *     NO node-count constants.
 *     NO process-count constants.
 *     NO thread-count constants.
 *     NO memory-capacity constants.
 *     NO network-capacity constants.
 *     NO device-count constants.
 *     NO qubit-count constants.
 *     NO tensor-rank limits.
 *     NO register-width limits.
 *     NO physical-device enumeration.
 *
 * Numeric values inside expressions remain ordinary program semantics.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Structural diagnostics should identify:
 *
 *     - malformed service declaration;
 *     - missing service name;
 *     - malformed generic parameter;
 *     - malformed parameter;
 *     - malformed return clause;
 *     - malformed service body;
 *     - malformed contract;
 *     - malformed metadata;
 *     - malformed extension.
 *
 * Semantic diagnostics belong downstream and include:
 *
 *     - unknown service;
 *     - duplicate service;
 *     - unknown type;
 *     - invalid capability;
 *     - unsatisfied requirement;
 *     - invalid effect;
 *     - invalid security policy;
 *     - infeasible deployment.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * The grammar is action-free and does not execute service code.
 *
 * It does not:
 *
 *     - contact endpoints;
 *     - access files;
 *     - query hardware;
 *     - query the network;
 *     - execute commands;
 *     - perform authentication;
 *     - load plugins.
 *
 * ============================================================================
 * PERFORMANCE
 * ============================================================================
 *
 * Repeated service collections use `*` and `+` rather than fixed-size
 * alternatives.
 *
 * No artificial service-size ceiling is encoded.
 *
 * Parser-resource protection for hostile or pathological input must be
 * implemented as explicit tooling/compiler policy rather than grammar
 * semantics.
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
 *     }
 *
 *     service Calculator {
 *         operation add(a: Int, b: Int) -> Int;
 *     }
 *
 *     service QuantumService {
 *         operation execute(input: quantum::State) -> quantum::State;
 *     }
 *
 *     service TensorService<T> {
 *         operation execute(input: Tensor<T>) -> Tensor<T>;
 *     }
 *
 *     service FutureService {
 *         requirement capability::future_compute;
 *     }
 *
 *     service DistributedService {
 *         requires capability::distributed::communication;
 *         requires resource::compute >= required_compute;
 *     }
 *
 * Boundary tests MUST include:
 *
 *     - zero service members;
 *     - one service member;
 *     - many operations;
 *     - many parameters;
 *     - many states;
 *     - many events;
 *     - many contracts;
 *     - deeply qualified names;
 *     - deeply nested metadata;
 *     - deeply nested service extensions;
 *     - large generic parameter lists.
 *
 * Negative tests MUST include:
 *
 *     service;
 *     service Calculator(
 *     service Calculator {
 *     service Calculator {
 *         operation add(a: Int -> Int;
 *     }
 *
 * Portability tests MUST verify that service syntax contains no dependency
 * on a particular:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     machine
 *     node
 *     IP address
 *     port
 *     provider
 *     topology.
 *
 * Determinism tests MUST verify equal token streams produce equivalent parse
 * trees under the same grammar version.
 *
 * AST tests MUST verify source ordering and source spans.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] It is a parser grammar.
 *     [x] It uses tokenVocab = ZamaniLexer.
 *     [x] It imports canonical Names.
 *     [x] It imports canonical Expressions.
 *     [x] It imports canonical Types.
 *     [x] It exposes distributedServiceDeclaration.
 *     [x] It does not redefine identifier.
 *     [x] It does not redefine qualifiedName.
 *     [x] It does not redefine expression.
 *     [x] It does not redefine expressionList.
 *     [x] It does not redefine typeExpression.
 *     [x] It uses canonical token names.
 *     [x] It has no embedded Rust actions.
 *     [x] It has no semantic predicates.
 *     [x] It has no unsafe implementation.
 *     [x] It has no hardware discovery.
 *     [x] It has no network discovery.
 *     [x] It has no runtime execution.
 *     [x] It has no physical placement.
 *     [x] It has no scheduling algorithm.
 *     [x] It has no routing algorithm.
 *     [x] It has no QEC implementation.
 *     [x] It has no ZQN implementation.
 *     [x] It introduces no second quantum IR.
 *     [x] It contains no artificial resource ceilings.
 *     [x] It preserves source-level extensibility.
 *     [x] It provides an AST contract.
 *     [x] It provides a semantic contract.
 *     [x] It provides an IR contract.
 *     [x] It provides compiler integration.
 *     [x] It provides runtime integration.
 *     [x] It provides cross-domain integration.
 *     [x] It defines positive tests.
 *     [x] It defines negative tests.
 *     [x] It defines boundary tests.
 *     [x] It defines scalability tests.
 *     [x] It defines determinism requirements.
 *     [x] It defines portability requirements.
 *
 * ============================================================================
 * INTEGRATION REQUIREMENTS
 * ============================================================================
 *
 * This file is independently complete as the service syntax contract, but
 * repository integration requires the following one-time convergence changes.
 *
 * 1. grammar/distributed/distributed.g4
 *
 *    Import:
 *
 *        DistributedServices
 *
 *    and route the existing distributed service entry point to:
 *
 *        distributedServiceDeclaration
 *
 *    Remove the old duplicate service implementation from distributed.g4.
 *
 * 2. grammar/distributed/README.md
 *
 *    Ensure services.g4 is recorded as the sole owner of distributed service
 *    syntax.
 *
 * 3. grammar/spec/distributed.md
 *
 *    Record:
 *
 *        distributed/services.g4
 *
 *    as the service syntax owner.
 *
 * 4. grammar/spec/syntax.md
 *
 *    Keep:
 *
 *        ServiceDeclaration
 *
 *    as the specification-level concept and map it to:
 *
 *        distributedServiceDeclaration
 *
 * 5. AST
 *
 *    Map this grammar into the existing domain-neutral frontend AST rather
 *    than introducing a parser-specific service AST hierarchy.
 *
 * 6. Semantic analysis
 *
 *    Resolve contextual service kinds, requirements, capabilities, contracts,
 *    effects, resources, security properties, and distributed semantics.
 *
 * 7. IR
 *
 *    Lower service semantics through the existing canonical semantic/IR
 *    pipeline.
 *
 *    Quantum-bearing operations MUST lower through:
 *
 *        quantum::ir
 *
 *    and MUST NOT create another quantum IR.
 *
 * 8. Tests
 *
 *    Add conformance tests under:
 *
 *        grammar/tests/distributed/
 *
 *    covering positive, negative, boundary, scalability, determinism,
 *    portability, AST, semantic, and IR behavior.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * Service grammar defines:
 *
 *     WHAT a distributed service is.
 *
 * Distributed/networking/runtime systems determine:
 *
 *     HOW it is realized.
 *
 * Therefore:
 *
 *     service syntax
 *          ->
 *     portable semantic intent
 *          ->
 *     semantic validation
 *          ->
 *     canonical IR
 *          ->
 *     optimization
 *          ->
 *     placement / routing / scheduling
 *          ->
 *     target realization
 *          ->
 *     runtime
 *
 * This preserves:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * without converting today's hardware or deployment topology into permanent
 * Zamani language limitations.
 *
 * ============================================================================
 */
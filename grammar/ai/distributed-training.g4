/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/ai/distributed-training.g4
 *
 * GRAMMAR
 * -------
 * AIDistributedTraining
 *
 * STATUS
 * ------
 * CANONICAL AI DISTRIBUTED-TRAINING SOURCE-SYNTAX LEAF
 *
 * LANGUAGE BASELINE
 * -----------------
 * Rust 2021
 * Rust 1.97
 * Rust 1.97.1
 *
 * SAFETY
 * ------
 * This file is ANTLR grammar only.
 *
 * It contains:
 *
 *     - no embedded Rust;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware discovery;
 *     - no runtime execution;
 *     - no allocation logic;
 *     - no unsafe Rust requirement.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar defines SOURCE-LEVEL DISTRIBUTED AI TRAINING INTENT.
 *
 * It provides a dedicated syntactic boundary for training workloads that may
 * be realized through:
 *
 *     - data parallelism;
 *     - model parallelism;
 *     - tensor parallelism;
 *     - pipeline parallelism;
 *     - sequence/context parallelism;
 *     - expert/routing parallelism;
 *     - parameter/state sharding;
 *     - replicated computation;
 *     - distributed data;
 *     - distributed checkpoints;
 *     - elastic execution;
 *     - fault-tolerant execution;
 *     - federated or decentralized training;
 *     - heterogeneous execution;
 *     - accelerator-backed execution;
 *     - quantum/classical hybrid training;
 *     - future distributed-training models.
 *
 * The grammar describes WHAT distributed training means at source level.
 *
 * It does NOT decide:
 *
 *     - which machine executes it;
 *     - which CPU executes it;
 *     - which GPU executes it;
 *     - which FPGA executes it;
 *     - which ASIC executes it;
 *     - which QPU executes it;
 *     - which node executes it;
 *     - which worker executes it;
 *     - which network is used;
 *     - which transport is used;
 *     - which accelerator is selected;
 *     - which physical memory is selected;
 *     - how work is physically scheduled;
 *     - how work is physically routed;
 *     - how resources are allocated;
 *     - how checkpoints are physically stored;
 *     - how failures are recovered;
 *     - how gradients are numerically computed.
 *
 * Those responsibilities remain downstream.
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
 *     Zamani parser
 *          |
 *          v
 *     AIDistributedTraining
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *     +----+----------------+------------------+
 *     |                     |                  |
 *     v                     v                  v
 *   AI semantics       resource semantics   distributed semantics
 *     |                     |                  |
 *     +---------------------+------------------+
 *                           |
 *                           v
 *                  canonical semantic model
 *                           |
 *                           v
 *                    canonical compiler IR
 *                           |
 *             +-------------+-------------+
 *             |             |             |
 *             v             v             v
 *         classical     quantum::ir    hardware/
 *             |             |             |
 *             +-------------+-------------+
 *                           |
 *                           v
 *                      optimization
 *                           |
 *                      scheduling
 *                           |
 *                      placement
 *                           |
 *                      routing
 *                           |
 *                    resilience/QEC/ZQN
 *                           |
 *                           v
 *                          HAL
 *                           |
 *                           v
 *                   target realization
 *
 * IMPORTANT:
 *
 * This grammar MUST NOT create:
 *
 *     DistributedTrainingIR
 *     DistributedAIIR
 *     TrainingClusterIR
 *     WorkerIR
 *     DataParallelIR
 *     ModelParallelIR
 *
 * merely because distributed-training syntax exists.
 *
 * Distributed training is source intent.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     distributedTrainingConstruct
 *     distributedTrainingDeclaration
 *     distributedTrainingBody
 *     distributedTrainingMember
 *     distributedTrainingBinding
 *     distributedTrainingInvocation
 *     distributedTrainingRegion
 *     distributedTrainingAnnotation
 *     distributedTrainingRequirement
 *     distributedTrainingCapability
 *     distributedTrainingConstraint
 *     distributedTrainingPreference
 *     distributedTrainingHint
 *     distributedTrainingComposition
 *     distributedTrainingConfiguration
 *     distributedTrainingExtension
 *
 * It provides structural syntax for:
 *
 *     - distributed training declarations;
 *     - distributed-training regions;
 *     - distributed-training metadata;
 *     - logical parallelization intent;
 *     - logical partitioning/sharding intent;
 *     - synchronization intent;
 *     - replication intent;
 *     - consistency intent;
 *     - elasticity intent;
 *     - recovery/checkpoint intent;
 *     - federation/decentralization intent;
 *     - resource/capability contracts;
 *     - open-world future extensions.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - identifiers;
 *     - qualified names;
 *     - expressions;
 *     - types;
 *     - general statements;
 *     - model declarations;
 *     - dataset declarations;
 *     - tensor declarations;
 *     - generic training declarations;
 *     - generic distributed declarations;
 *     - networking;
 *     - concurrency;
 *     - resource semantics;
 *     - hardware semantics;
 *     - accelerator implementation;
 *     - optimizer implementation;
 *     - automatic differentiation;
 *     - gradient computation;
 *     - checkpoint storage implementation;
 *     - scheduling;
 *     - routing;
 *     - deployment;
 *     - quantum IR;
 *     - QEC;
 *     - ZQN;
 *     - HAL.
 *
 * ============================================================================
 * NON-DUPLICATION PRINCIPLE
 * ============================================================================
 *
 * Existing canonical owners remain authoritative:
 *
 *     grammar/ai/training.g4
 *         generic AI training syntax
 *
 *     grammar/ai/models.g4
 *         model declarations
 *
 *     grammar/ai/datasets.g4
 *         AI dataset declarations
 *
 *     grammar/ai/tensors.g4
 *         AI tensor syntax
 *
 *     grammar/distributed/distributed.g4
 *         general distributed-computing composition
 *
 *     grammar/concurrency/
 *         concurrency and parallel execution syntax
 *
 *     grammar/resources/
 *         resource intent and requirements
 *
 *     grammar/hardware/
 *         hardware capabilities and hardware intent
 *
 *     grammar/networking/
 *         networking syntax
 *
 *     grammar/security/
 *         security and authorization
 *
 *     grammar/quantum/
 *         quantum source semantics
 *
 *     grammar/compile/
 *         compilation intent
 *
 *     grammar/execution/
 *         execution intent
 *
 * This file composes those concepts rather than replacing them.
 *
 * ============================================================================
 * LEXICAL AUTHORITY
 * ============================================================================
 *
 * The canonical lexical vocabulary is external to this file.
 *
 * This grammar does NOT define lexer rules.
 *
 * Annotation syntax uses:
 *
 *     AT IDENTIFIER
 *
 * rather than an AI-specific annotation token.
 *
 * Therefore constructs such as:
 *
 *     @distributed_training
 *     @strategy
 *     @partition
 *     @synchronization
 *     @checkpoint
 *
 * remain ordinary lexical identifiers whose semantic meaning is resolved
 * downstream.
 *
 * This prevents the lexer from becoming a closed enumeration of AI concepts.
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * This is a leaf grammar.
 *
 * It imports only foundational parser grammars required to represent the
 * structure of distributed training.
 *
 * Required dependencies:
 *
 *     Types
 *     Expressions
 *     Statements
 *
 * Names are consumed through the expression/type contracts where possible.
 *
 * Resource and distributed composition remains an explicit parent-level
 * integration concern.
 *
 * This avoids creating a dependency cycle such as:
 *
 *     Distributed
 *         ->
 *     AIDistributedTraining
 *         ->
 *     Distributed
 *
 * The distributed-training grammar therefore does NOT import the distributed
 * composition root merely to gain generic distributed concepts.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Type syntax remains owned by:
 *
 *     Types
 *
 * This file consumes:
 *
 *     typeExpression
 *
 * It MUST NOT define:
 *
 *     DistributedTrainingType
 *     WorkerType
 *     ClusterType
 *     ShardType
 *     ReplicaType
 *     TrainingWorkerType
 *
 * as competing type systems.
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * General expressions remain owned by:
 *
 *     Expressions
 *
 * This file consumes:
 *
 *     expression
 *     argumentList
 *
 * Expressions may represent:
 *
 *     - model references;
 *     - dataset references;
 *     - tensor values;
 *     - batch sizes;
 *     - logical partition counts;
 *     - symbolic dimensions;
 *     - resource quantities;
 *     - capability predicates;
 *     - synchronization policies;
 *     - checkpoint policies;
 *     - scaling policies;
 *     - consistency policies;
 *     - user-defined configuration;
 *     - runtime-derived values where supported.
 *
 * This grammar does NOT redefine arithmetic, logical, comparison, indexing,
 * call, assignment, or other general expression syntax.
 *
 * ============================================================================
 * STATEMENT CONTRACT
 * ============================================================================
 *
 * Ordinary Zamani statements remain owned by:
 *
 *     Statements
 *
 * A distributed-training body may contain ordinary statements through:
 *
 *     statement
 *
 * This allows training computation to compose with the rest of Zamani without
 * creating a second statement language.
 *
 * ============================================================================
 * AI TRAINING INTEGRATION
 * ============================================================================
 *
 * Generic training remains owned by:
 *
 *     grammar/ai/training.g4
 *
 * This file specializes the distributed-training portion of that model.
 *
 * The conceptual relationship is:
 *
 *     training
 *         |
 *         +--> local training
 *         |
 *         +--> distributed training
 *                     |
 *                     +--> data parallel
 *                     +--> model parallel
 *                     +--> tensor parallel
 *                     +--> pipeline parallel
 *                     +--> sharded
 *                     +--> replicated
 *                     +--> elastic
 *                     +--> federated
 *                     +--> fault tolerant
 *                     +--> future models
 *
 * `training.g4` remains the owner of generic training constructs.
 *
 * `distributed-training.g4` owns only the distributed-training specialization.
 *
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * The stable public entry point is:
 *
 *     distributedTrainingConstruct
 *
 * It accepts a dedicated distributed-training declaration.
 *
 * This prevents the grammar from swallowing arbitrary ordinary expressions.
 *
 * ============================================================================
 */

parser grammar AIDistributedTraining;

options {
    tokenVocab = ZamaniLexer;
}

import
    Types,
    Expressions,
    Statements
;


/*
 * ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Canonical source shape:
 *
 *     @distributed_training Experiment {
 *         ...
 *     }
 *
 * The annotation spelling is intentionally NOT hard-coded.
 *
 * Semantic analysis determines whether the annotation identifies the
 * distributed-training construct registered by the current language version.
 */
distributedTrainingConstruct
    : distributedTrainingDeclaration
    ;


/*
 * ============================================================================
 * 2. DISTRIBUTED-TRAINING DECLARATION
 * ============================================================================
 *
 * The declaration consists of:
 *
 *     annotation
 *     logical source-level name
 *     optional type
 *     optional initializer
 *     optional body
 *
 * The source-level name identifies the logical training computation.
 *
 * It is NOT:
 *
 *     a node identifier;
 *     a worker identifier;
 *     a machine identifier;
 *     a device identifier;
 *     a process identifier;
 *     a physical accelerator identifier.
 */
distributedTrainingDeclaration
    : distributedTrainingAnnotation
      identifier
      distributedTrainingTypeClause?
      distributedTrainingInitializer?
      distributedTrainingBody?
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 3. DECLARATION ANNOTATION
 * ============================================================================
 *
 * The annotation remains structurally generic.
 *
 * The semantic registry determines whether the annotation represents:
 *
 *     distributed_training
 *     another registered distributed-training dialect
 *     a versioned extension
 *     a future training model
 *
 * The parser does not create a keyword enumeration.
 */
distributedTrainingAnnotation
    : AT
      identifier
    ;


/*
 * ============================================================================
 * 4. OPTIONAL TYPE
 * ============================================================================
 */
distributedTrainingTypeClause
    : COLON
      typeExpression
    ;


/*
 * ============================================================================
 * 5. OPTIONAL INITIALIZER
 * ============================================================================
 */
distributedTrainingInitializer
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * 6. DISTRIBUTED-TRAINING BODY
 * ============================================================================
 *
 * Member order is preserved by the parser.
 *
 * There is no grammar-level maximum for the number of members.
 */
distributedTrainingBody
    : LBRACE
      distributedTrainingMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 7. DISTRIBUTED-TRAINING MEMBER
 * ============================================================================
 *
 * The member categories intentionally preserve semantic distinctions while
 * delegating their internal values to canonical grammar systems.
 */
distributedTrainingMember
    : distributedTrainingBinding
    | distributedTrainingInvocation
    | distributedTrainingRegion
    | distributedTrainingRequirement
    | distributedTrainingCapability
    | distributedTrainingConstraint
    | distributedTrainingPreference
    | distributedTrainingHint
    | distributedTrainingAnnotatedMember
    | distributedTrainingStatement
    ;


/*
 * ============================================================================
 * 8. NAMED DISTRIBUTED-TRAINING BINDING
 * ============================================================================
 *
 * Examples:
 *
 *     @model = model;
 *     @dataset = dataset;
 *     @strategy = strategy;
 *     @partition = partition_policy;
 *     @synchronization = synchronization_policy;
 *
 * The semantic layer determines the role.
 *
 * The parser does not enumerate the possible roles.
 */
distributedTrainingBinding
    : AT
      identifier
      ASSIGN
      expression
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 9. DISTRIBUTED-TRAINING INVOCATION
 * ============================================================================
 *
 * Examples:
 *
 *     @strategy(data_parallel);
 *     @partition(shard_policy);
 *     @synchronization(sync_policy);
 *     @checkpoint(checkpoint_policy);
 *     @scaling(elastic_policy);
 *
 * The operation name remains open-world.
 */
distributedTrainingInvocation
    : AT
      identifier
      LPAREN
      argumentList?
      RPAREN
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 10. DISTRIBUTED-TRAINING REGION
 * ============================================================================
 *
 * A named distributed-training region may contain distributed-training members.
 *
 * Example:
 *
 *     @strategy {
 *         @mode = data_parallel;
 *         @partition = logical_partition;
 *     }
 *
 * The braces define source structure only.
 */
distributedTrainingRegion
    : AT
      identifier
      LBRACE
      distributedTrainingMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 11. REQUIREMENT CONTRACT
 * ============================================================================
 *
 * A requirement states a condition that a realization must satisfy.
 *
 * Examples:
 *
 *     requires capability("distributed.training");
 *     requires capability("collective.reduce");
 *     requires memory >= required_memory;
 *     requires communication >= required_communication;
 *
 * The expression remains canonical.
 *
 * This rule does NOT determine which resource satisfies the requirement.
 */
distributedTrainingRequirement
    : REQUIRES
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 12. CAPABILITY CONTRACT
 * ============================================================================
 *
 * Capability means:
 *
 *     an environment can provide something.
 *
 * It does not mean:
 *
 *     select a specific physical device.
 */
distributedTrainingCapability
    : CAPABILITY
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 13. CONSTRAINT CONTRACT
 * ============================================================================
 *
 * A constraint describes an admissibility condition.
 */
distributedTrainingConstraint
    : CONSTRAINT
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 14. PREFERENCE CONTRACT
 * ============================================================================
 *
 * A preference does not become a mandatory implementation choice.
 */
distributedTrainingPreference
    : PREFER
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 15. HINT CONTRACT
 * ============================================================================
 *
 * Hints provide implementation guidance without becoming semantic
 * requirements unless the semantic specification explicitly defines otherwise.
 */
distributedTrainingHint
    : HINT
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 16. GENERIC ANNOTATED MEMBER
 * ============================================================================
 *
 * This is the principal extensibility mechanism.
 *
 * It supports future distributed-training concepts without requiring a new
 * lexer keyword.
 *
 * Examples:
 *
 *     @strategy ...
 *     @partition ...
 *     @replication ...
 *     @synchronization ...
 *     @consistency ...
 *     @checkpoint ...
 *     @recovery ...
 *     @elastic ...
 *     @federated ...
 *     @secure ...
 *     @heterogeneous ...
 *     @quantum ...
 *     @future_training_model ...
 *
 * Semantic analysis determines their actual meaning.
 */
distributedTrainingAnnotatedMember
    : AT
      identifier
      distributedTrainingPayload
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 17. ANNOTATED PAYLOAD
 * ============================================================================
 *
 * The payload has four structurally distinct forms:
 *
 *     named binding
 *     expression
 *     invocation
 *     nested region
 *
 * This is deliberately structural rather than a closed list of algorithms.
 */
distributedTrainingPayload
    : distributedTrainingPayloadBinding
    | distributedTrainingPayloadExpression
    | distributedTrainingPayloadInvocation
    | distributedTrainingPayloadRegion
    ;


/*
 * ============================================================================
 * 18. ANNOTATED NAMED BINDING
 * ============================================================================
 *
 * Example:
 *
 *     @strategy mode = data_parallel;
 */
distributedTrainingPayloadBinding
    : identifier
      distributedTrainingTypeClause?
      ASSIGN
      expression
    ;


/*
 * ============================================================================
 * 19. ANNOTATED EXPRESSION
 * ============================================================================
 */
distributedTrainingPayloadExpression
    : expression
    ;


/*
 * ============================================================================
 * 20. ANNOTATED INVOCATION
 * ============================================================================
 */
distributedTrainingPayloadInvocation
    : identifier
      LPAREN
      argumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * 21. ANNOTATED REGION
 * ============================================================================
 */
distributedTrainingPayloadRegion
    : LBRACE
      distributedTrainingMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 22. LOGICAL DISTRIBUTED-TRAINING COMPOSITION
 * ============================================================================
 *
 * Distributed training may combine multiple logical strategies.
 *
 * Examples include:
 *
 *     data partitioning
 *     +
 *     tensor partitioning
 *     +
 *     pipeline staging
 *     +
 *     replicated state
 *     +
 *     synchronization
 *
 * This rule gives semantic tooling a stable structural boundary without
 * introducing a strategy enumeration.
 */
distributedTrainingComposition
    : distributedTrainingCompositionMember+
    ;


distributedTrainingCompositionMember
    : distributedTrainingBinding
    | distributedTrainingInvocation
    | distributedTrainingRegion
    | distributedTrainingAnnotatedMember
    ;


/*
 * ============================================================================
 * 23. CONFIGURATION REGION
 * ============================================================================
 *
 * Configuration is represented structurally and semantically.
 *
 * It is NOT a physical deployment manifest.
 */
distributedTrainingConfiguration
    : AT
      identifier
      LBRACE
      distributedTrainingMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 24. FUTURE EXTENSION
 * ============================================================================
 *
 * New distributed-training paradigms must remain expressible without changing
 * the core lexical vocabulary.
 *
 * Examples include future forms of:
 *
 *     parallel learning
 *     federated learning
 *     decentralized learning
 *     swarm learning
 *     split learning
 *     continual distributed learning
 *     distributed reinforcement learning
 *     distributed symbolic learning
 *     distributed probabilistic learning
 *     quantum-assisted distributed learning
 *     hybrid distributed learning
 *     heterogeneous learning
 *     privacy-preserving learning
 *     future computational substrates.
 *
 * The syntax remains annotation-led.
 */
distributedTrainingExtension
    : AT
      identifier
      (
          distributedTrainingPayload
        | LPAREN argumentList? RPAREN
        | LBRACE distributedTrainingMember* RBRACE
      )
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 25. ORDINARY ZAMANI STATEMENT
 * ============================================================================
 *
 * Distributed training is still Zamani.
 *
 * Therefore ordinary statements may occur inside a distributed-training
 * computation region.
 */
distributedTrainingStatement
    : statement
    ;


/*
 * ============================================================================
 * 26. DOMAIN BRIDGES
 * ============================================================================
 *
 * These bridges provide stable integration points for semantic tooling.
 *
 * They do not create duplicate semantic types.
 */
distributedTrainingExpression
    : expression
    ;


distributedTrainingType
    : typeExpression
    ;


distributedTrainingArguments
    : argumentList
    ;


/*
 * ============================================================================
 * 27. MODEL / DATASET / TENSOR REFERENCES
 * ============================================================================
 *
 * These remain expressions rather than AI-specific parser types.
 */
distributedTrainingModelReference
    : expression
    ;


distributedTrainingDatasetReference
    : expression
    ;


distributedTrainingTensorReference
    : expression
    ;


/*
 * ============================================================================
 * 28. PARALLELISM INTENT
 * ============================================================================
 *
 * IMPORTANT:
 *
 * This grammar deliberately does NOT enumerate:
 *
 *     data_parallel
 *     model_parallel
 *     tensor_parallel
 *     pipeline_parallel
 *     sequence_parallel
 *     expert_parallel
 *
 * as lexer tokens or mandatory parser alternatives.
 *
 * They are semantic strategy identifiers.
 *
 * This allows future strategies without changing the lexical language.
 */
distributedTrainingParallelismIntent
    : AT
      identifier
      (
          ASSIGN expression
        | LPAREN argumentList? RPAREN
        | LBRACE distributedTrainingMember* RBRACE
      )
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 29. PARTITIONING / SHARDING INTENT
 * ============================================================================
 *
 * Logical partitioning is different from physical placement.
 *
 * The grammar describes the logical partitioning intent.
 *
 * It does NOT select:
 *
 *     machine;
 *     process;
 *     device;
 *     memory bank;
 *     accelerator;
 *     network link.
 */
distributedTrainingPartitioningIntent
    : AT
      identifier
      (
          ASSIGN expression
        | LPAREN argumentList? RPAREN
        | LBRACE distributedTrainingMember* RBRACE
      )
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 30. SYNCHRONIZATION INTENT
 * ============================================================================
 *
 * Synchronization is source-level intent.
 *
 * Examples of semantic policies may include:
 *
 *     synchronous
 *     asynchronous
 *     bounded-staleness
 *     barrier-based
 *     event-driven
 *     consistency-aware
 *
 * The grammar does not enumerate them.
 */
distributedTrainingSynchronizationIntent
    : AT
      identifier
      (
          ASSIGN expression
        | LPAREN argumentList? RPAREN
        | LBRACE distributedTrainingMember* RBRACE
      )
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 31. REPLICATION INTENT
 * ============================================================================
 *
 * Replication means logical duplication of state or computation.
 *
 * It does not imply a particular number of physical copies.
 *
 * A value such as:
 *
 *     replication_factor
 *
 * is a program/semantic value, not a grammar maximum.
 */
distributedTrainingReplicationIntent
    : AT
      identifier
      (
          ASSIGN expression
        | LPAREN argumentList? RPAREN
        | LBRACE distributedTrainingMember* RBRACE
      )
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 32. ELASTIC-SCALING INTENT
 * ============================================================================
 *
 * Scaling remains resource-parametric.
 *
 * The grammar permits expressions representing:
 *
 *     minimum logical parallelism
 *     desired parallelism
 *     maximum requested parallelism
 *     dynamic scaling policy
 *
 * but does not create:
 *
 *     MAX_WORKERS
 *     MAX_NODES
 *     MAX_DEVICES
 *
 * or equivalent language limits.
 */
distributedTrainingElasticityIntent
    : AT
      identifier
      (
          ASSIGN expression
        | LPAREN argumentList? RPAREN
        | LBRACE distributedTrainingMember* RBRACE
      )
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 33. CHECKPOINT / RECOVERY INTENT
 * ============================================================================
 *
 * Checkpoint syntax describes logical persistence/recovery intent.
 *
 * It does not select:
 *
 *     filesystem;
 *     object store;
 *     database;
 *     cloud provider;
 *     storage device;
 *     memory device.
 */
distributedTrainingCheckpointIntent
    : AT
      identifier
      (
          ASSIGN expression
        | LPAREN argumentList? RPAREN
        | LBRACE distributedTrainingMember* RBRACE
      )
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 34. FAULT-TOLERANCE INTENT
 * ============================================================================
 *
 * Fault tolerance is a semantic policy.
 *
 * Runtime recovery remains downstream.
 */
distributedTrainingFaultToleranceIntent
    : AT
      identifier
      (
          ASSIGN expression
        | LPAREN argumentList? RPAREN
        | LBRACE distributedTrainingMember* RBRACE
      )
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 35. FEDERATED / DECENTRALIZED INTENT
 * ============================================================================
 *
 * Federation may involve independently controlled participants.
 *
 * The grammar does not assume:
 *
 *     participant count;
 *     site count;
 *     region count;
 *     organization count;
 *     node count;
 *     network count.
 */
distributedTrainingFederatedIntent
    : AT
      identifier
      (
          ASSIGN expression
        | LPAREN argumentList? RPAREN
        | LBRACE distributedTrainingMember* RBRACE
      )
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 36. HETEROGENEOUS EXECUTION INTENT
 * ============================================================================
 *
 * The same logical training computation may be realized using different
 * computational resources.
 *
 * Examples:
 *
 *     CPU + accelerator
 *     accelerator + accelerator
 *     classical + quantum
 *     edge + cloud
 *     embedded + remote
 *
 * The grammar does not select a target.
 */
distributedTrainingHeterogeneousIntent
    : AT
      identifier
      (
          ASSIGN expression
        | LPAREN argumentList? RPAREN
        | LBRACE distributedTrainingMember* RBRACE
      )
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 37. QUANTUM / CLASSICAL HYBRID INTENT
 * ============================================================================
 *
 * Distributed AI training may consume quantum-derived values or quantum
 * computation.
 *
 * This grammar does NOT define:
 *
 *     qubits;
 *     gates;
 *     QubitId;
 *     PhysicalQubitId;
 *     topology;
 *     routing;
 *     QEC;
 *     ZQN;
 *     pulse schedules.
 *
 * Those remain owned by the quantum architecture.
 *
 * The canonical quantum boundary remains:
 *
 *     quantum::ir
 */
distributedTrainingQuantumHybridIntent
    : AT
      identifier
      (
          ASSIGN expression
        | LPAREN argumentList? RPAREN
        | LBRACE distributedTrainingMember* RBRACE
      )
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 38. RESOURCE REQUIREMENT BRIDGE
 * ============================================================================
 *
 * This grammar deliberately does not duplicate the resource grammar.
 *
 * The expression payload remains compatible with the repository's canonical
 * resource-expression model.
 *
 * Examples:
 *
 *     requires memory >= required_memory;
 *
 *     requires nodes >= required_nodes;
 *
 *     requires capability("distributed.training");
 *
 *     requires capability("collective.reduce");
 *
 *     requires capability("tensor.compute");
 *
 *     requires capability("network.communication");
 *
 * The semantic resource subsystem decides whether the environment satisfies
 * the requirement.
 */
distributedTrainingResourceRequirement
    : REQUIRES
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 39. CAPABILITY BRIDGE
 * ============================================================================
 */
distributedTrainingCapabilityRequirement
    : CAPABILITY
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 40. PORTABILITY CONTRACT
 * ============================================================================
 *
 * Distributed training is portable when source semantics are independent of
 * physical realization.
 *
 * The grammar therefore permits source-level portability metadata without
 * naming a physical deployment.
 */
distributedTrainingPortability
    : AT
      identifier
      (
          ASSIGN expression
        | LPAREN argumentList? RPAREN
        | LBRACE distributedTrainingMember* RBRACE
      )
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 41. RESOURCE / CAPABILITY DISTINCTION
 * ============================================================================
 *
 * The semantic distinction is:
 *
 *     requirement
 *         =
 *     what must be satisfied
 *
 *     capability
 *         =
 *     what an environment can provide
 *
 *     constraint
 *         =
 *     admissibility condition
 *
 *     preference
 *         =
 *     desired realization
 *
 *     hint
 *         =
 *     implementation guidance
 *
 * This file preserves those categories syntactically.
 *
 * It must never collapse them into:
 *
 *     select device
 *
 * or:
 *
 *     select node
 *
 * ============================================================================
 * 42. OPEN-WORLD STRATEGY MODEL
 * ============================================================================
 *
 * Distributed training strategy names remain semantic identifiers.
 *
 * The language therefore remains extensible to:
 *
 *     data parallelism
 *     model parallelism
 *     tensor parallelism
 *     pipeline parallelism
 *     sequence parallelism
 *     expert parallelism
 *     sharded training
 *     replicated training
 *     asynchronous training
 *     synchronous training
 *     federated training
 *     decentralized training
 *     split learning
 *     continual distributed learning
 *     future strategies.
 *
 * The grammar does not need a new keyword for each strategy.
 *
 * ============================================================================
 * 43. OPEN-WORLD COMMUNICATION MODEL
 * ============================================================================
 *
 * Communication semantics are delegated to:
 *
 *     grammar/networking/
 *     grammar/distributed/
 *
 * This file may express communication requirements and policies but does not
 * define:
 *
 *     TCP;
 *     UDP;
 *     QUIC;
 *     MPI;
 *     RDMA;
 *     InfiniBand;
 *     vendor-specific transports.
 *
 * Transport realization remains downstream.
 *
 * ============================================================================
 * 44. OPEN-WORLD ACCELERATOR MODEL
 * ============================================================================
 *
 * Accelerator semantics are delegated to:
 *
 *     grammar/hardware/
 *     grammar/resources/
 *
 * The source may require:
 *
 *     capability("tensor.compute")
 *
 * but does not have to specify:
 *
 *     GPU 0
 *     accelerator 7
 *     device 3
 *
 * Physical selection remains downstream.
 *
 * ============================================================================
 * 45. DATA-PARALLEL SEMANTICS
 * ============================================================================
 *
 * Data parallelism means that logical training computation may operate over
 * partitions or replicas of data.
 *
 * This file represents the intent structurally.
 *
 * It does not prescribe:
 *
 *     - partition count;
 *     - worker count;
 *     - node count;
 *     - physical shard location;
 *     - network topology.
 *
 * Those values may be represented by source expressions where semantically
 * appropriate.
 *
 * ============================================================================
 * 46. MODEL-PARALLEL SEMANTICS
 * ============================================================================
 *
 * Model parallelism means that a logical model computation may be partitioned
 * across realizations.
 *
 * Partition boundaries remain semantic.
 *
 * Physical placement remains downstream.
 *
 * ============================================================================
 * 47. TENSOR-PARALLEL SEMANTICS
 * ============================================================================
 *
 * Tensor parallelism may partition tensor computation.
 *
 * Tensor shape and dimensions remain owned by the AI/data/type architecture.
 *
 * This grammar does not impose:
 *
 *     maximum tensor rank;
 *     maximum dimension;
 *     maximum shard count;
 *     maximum model size.
 *
 * ============================================================================
 * 48. PIPELINE-PARALLEL SEMANTICS
 * ============================================================================
 *
 * Pipeline parallelism may express logical stages.
 *
 * Stage count is unbounded by language semantics.
 *
 * Physical pipeline scheduling remains downstream.
 *
 * ============================================================================
 * 49. SHARDING SEMANTICS
 * ============================================================================
 *
 * Sharding describes logical partitioning.
 *
 * It is distinct from:
 *
 *     physical placement;
 *     physical memory allocation;
 *     network routing.
 *
 * ============================================================================
 * 50. REPLICATION SEMANTICS
 * ============================================================================
 *
 * Replication describes logical copies.
 *
 * A replication value is not a grammar-level hardware limit.
 *
 * Example:
 *
 *     @replication factor = desired_replication;
 *
 * is source semantics.
 *
 * The runtime determines whether that intent can be realized.
 *
 * ============================================================================
 * 51. SYNCHRONIZATION SEMANTICS
 * ============================================================================
 *
 * Synchronization may be expressed as source intent.
 *
 * The runtime/compiler may lower that intent to:
 *
 *     barriers;
 *     collectives;
 *     messages;
 *     events;
 *     futures;
 *     other mechanisms.
 *
 * This grammar does not select an implementation.
 *
 * ============================================================================
 * 52. ELASTIC EXECUTION
 * ============================================================================
 *
 * Distributed training may scale according to resource availability.
 *
 * The source may express:
 *
 *     desired_parallelism
 *     scaling_policy
 *     resource_requirements
 *     capability_requirements
 *
 * without hard-coding:
 *
 *     worker count;
 *     node count;
 *     device count.
 *
 * If a source program explicitly contains a number, that number is program
 * semantics, not a universal language limit.
 *
 * ============================================================================
 * 53. RESOURCE AVAILABILITY
 * ============================================================================
 *
 * A distributed-training program MAY be evaluated against an environment E.
 *
 * Conceptually:
 *
 *     Requirements(program, E)
 *
 * determines whether the requested execution can be realized.
 *
 * The grammar does not perform that evaluation.
 *
 * If resources are insufficient, downstream systems may report an explicit
 * resource/capability failure rather than changing source semantics.
 *
 * ============================================================================
 * 54. FAILURE / RESILIENCE
 * ============================================================================
 *
 * Distributed training may declare recovery intent.
 *
 * The grammar does not implement recovery.
 *
 * Existing resilience vocabulary remains downstream:
 *
 *     Unknown
 *     Healthy
 *     Degraded
 *     Unstable
 *     Unavailable
 *     Recovering
 *     Quarantined
 *     Retired
 *
 * and outcomes:
 *
 *     ACCEPT
 *     DEGRADED_ACCEPT
 *     RETRY
 *     RECOVER
 *     ESCALATE
 *     REJECT
 *
 * These are semantic/runtime values, not parser decisions.
 *
 * ============================================================================
 * 55. CHECKPOINT SEMANTICS
 * ============================================================================
 *
 * A checkpoint is logical persistent training state.
 *
 * The grammar does not select:
 *
 *     filesystem;
 *     object store;
 *     database;
 *     cloud;
 *     storage device.
 *
 * It only expresses source intent.
 *
 * ============================================================================
 * 56. FEDERATED TRAINING
 * ============================================================================
 *
 * Federated/decentralized training may involve independently controlled
 * participants.
 *
 * The grammar does not require:
 *
 *     a fixed participant count;
 *     a fixed organization count;
 *     a fixed site count;
 *     a fixed node count.
 *
 * Participant identities remain semantic values.
 *
 * ============================================================================
 * 57. SECURITY
 * ============================================================================
 *
 * Distributed training may cross trust boundaries.
 *
 * Security requirements remain integrated through:
 *
 *     grammar/security/
 *     grammar/effects/
 *     grammar/resources/
 *
 * This grammar does not implement:
 *
 *     authentication;
 *     authorization;
 *     encryption;
 *     key management;
 *     secure aggregation;
 *     privacy algorithms.
 *
 * It may express the corresponding capability/requirement intent.
 *
 * ============================================================================
 * 58. AI / DATA INTEGRATION
 * ============================================================================
 *
 * Distributed training consumes:
 *
 *     models;
 *     datasets;
 *     tensors;
 *     training values;
 *     metrics;
 *     checkpoints.
 *
 * Their concrete declarations remain owned by their respective grammar
 * components.
 *
 * This grammar references them through expressions.
 *
 * ============================================================================
 * 59. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Distributed AI training may use:
 *
 *     classical computation;
 *     quantum computation;
 *     hybrid computation.
 *
 * Quantum constructs remain outside this grammar.
 *
 * The semantic path is:
 *
 *     distributed training AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum semantics
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization/routing/scheduling/QEC/ZQN/HAL
 *
 * No second quantum IR is introduced.
 *
 * ============================================================================
 * 60. HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Distributed training may be realized through:
 *
 *     CPUs;
 *     GPUs;
 *     FPGAs;
 *     ASICs;
 *     accelerators;
 *     heterogeneous hardware;
 *     future hardware.
 *
 * The grammar does not enumerate or select them.
 *
 * Hardware realization remains downstream.
 *
 * ============================================================================
 * 61. POCO-REAF
 * ============================================================================
 *
 * This grammar is designed around:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * A distributed-training source program expresses:
 *
 *     WHAT training computation is desired;
 *     WHAT data/model relationships exist;
 *     WHAT parallelization intent exists;
 *     WHAT consistency is required;
 *     WHAT capabilities are required;
 *     WHAT resources are required;
 *     WHAT constraints apply;
 *     WHAT preferences exist;
 *     WHAT recovery guarantees are desired.
 *
 * It does not prescribe:
 *
 *     WHICH machine;
 *     WHICH worker;
 *     WHICH device;
 *     WHICH node;
 *     WHICH accelerator;
 *     WHICH transport;
 *     WHICH physical memory.
 *
 * ============================================================================
 * 62. HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This grammar MUST NOT introduce:
 *
 *     MAX_WORKERS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_ACCELERATORS
 *     MAX_GPUS
 *     MAX_CPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QPUS
 *     MAX_MEMORY
 *     MAX_STORAGE
 *     MAX_BANDWIDTH
 *     MAX_PARAMETERS
 *     MAX_TENSOR_RANK
 *     MAX_TENSOR_DIMENSION
 *     MAX_SHARDS
 *     MAX_REPLICAS
 *     MAX_PARTITIONS
 *     MAX_STAGES
 *     MAX_PARTICIPANTS
 *
 * It also MUST NOT encode equivalent fixed limits.
 *
 * For example, this is NOT a language rule:
 *
 *     workers <= 8
 *
 * if `8` is merely intended as today's hardware capacity.
 *
 * If a programmer explicitly writes:
 *
 *     requires workers >= desired_workers;
 *
 * then `desired_workers` is semantic program intent.
 *
 * ============================================================================
 * 63. NO PHYSICAL IDENTIFIER CONTRACT
 * ============================================================================
 *
 * This grammar MUST NOT require:
 *
 *     worker0
 *     node0
 *     gpu0
 *     qpu0
 *     device0
 *     accelerator0
 *
 * as physical execution identifiers.
 *
 * Logical names remain permitted because logical identity is distinct from
 * physical realization.
 *
 * ============================================================================
 * 64. SCALABILITY CONTRACT
 * ============================================================================
 *
 * All source-level collections use:
 *
 *     *
 *     +
 *
 * where appropriate.
 *
 * There is no grammar-level finite limit for:
 *
 *     - training members;
 *     - training phases;
 *     - logical strategies;
 *     - partitioning clauses;
 *     - replication clauses;
 *     - synchronization clauses;
 *     - checkpoints;
 *     - recovery policies;
 *     - participants;
 *     - datasets;
 *     - model parameters;
 *     - tensors;
 *     - logical partitions;
 *     - logical stages;
 *     - semantic relationships.
 *
 * Practical limits may arise from:
 *
 *     - parser memory;
 *     - compiler memory;
 *     - runtime memory;
 *     - target resources;
 *     - network capacity;
 *     - deployment policy;
 *     - explicit operational limits.
 *
 * Those are NOT language-level semantic limits.
 *
 * ============================================================================
 * 65. DETERMINISM CONTRACT
 * ============================================================================
 *
 * Given:
 *
 *     identical source;
 *     identical language version;
 *     identical parser configuration;
 *     identical dialect selection;
 *
 * parsing MUST produce the same:
 *
 *     - parse structure;
 *     - source ordering;
 *     - token interpretation;
 *     - source spans;
 *     - syntax diagnostics.
 *
 * Parsing MUST NOT depend on:
 *
 *     - CPU count;
 *     - GPU availability;
 *     - node count;
 *     - network state;
 *     - filesystem state;
 *     - wall-clock time;
 *     - random state;
 *     - runtime resource availability.
 *
 * ============================================================================
 * 66. SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * The AST/parser bridge must preserve source spans for:
 *
 *     distributedTrainingDeclaration
 *     distributedTrainingAnnotation
 *     distributedTrainingBody
 *     distributedTrainingMember
 *     distributedTrainingBinding
 *     distributedTrainingInvocation
 *     distributedTrainingRegion
 *     distributedTrainingRequirement
 *     distributedTrainingCapability
 *     distributedTrainingConstraint
 *     distributedTrainingPreference
 *     distributedTrainingHint
 *     distributedTrainingExtension
 *
 * This permits precise diagnostics without changing grammar structure later.
 *
 * ============================================================================
 * 67. AST CONTRACT
 * ============================================================================
 *
 * This grammar must lower into the existing domain-neutral frontend AST.
 *
 * The AST must preserve:
 *
 *     - logical declaration name;
 *     - annotation name;
 *     - type expression;
 *     - initializer expression;
 *     - member ordering;
 *     - nested regions;
 *     - expressions;
 *     - argument ordering;
 *     - resource/capability clauses;
 *     - source spans;
 *     - source-level attributes/modifiers.
 *
 * The AST must NOT contain:
 *
 *     physical worker handles;
 *     physical node handles;
 *     physical device handles;
 *     scheduler state;
 *     network sockets;
 *     GPU contexts;
 *     QPU handles;
 *     memory addresses;
 *     routing state;
 *     checkpoint storage handles.
 *
 * ============================================================================
 * 68. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     - resolving the distributed-training annotation;
 *     - resolving model references;
 *     - resolving dataset references;
 *     - resolving strategy names;
 *     - validating strategy composition;
 *     - validating partitioning semantics;
 *     - validating synchronization semantics;
 *     - validating consistency;
 *     - validating replication;
 *     - validating elasticity;
 *     - validating resource requirements;
 *     - validating capabilities;
 *     - validating security requirements;
 *     - validating portability;
 *     - validating type relationships;
 *     - validating effect relationships;
 *     - validating quantum/classical boundaries.
 *
 * None of those operations occur in this grammar.
 *
 * ============================================================================
 * 69. RESOURCE SEMANTIC CONTRACT
 * ============================================================================
 *
 * Resource feasibility belongs to the resource/compiler/runtime architecture.
 *
 * This grammar may express:
 *
 *     requires ...
 *     capability ...
 *     constraint ...
 *     prefer ...
 *     hint ...
 *
 * but does not determine whether they are satisfiable.
 *
 * Resource insufficiency MUST NOT silently alter program semantics.
 *
 * ============================================================================
 * 70. DISTRIBUTED SEMANTIC CONTRACT
 * ============================================================================
 *
 * Distributed semantics belong to:
 *
 *     grammar/distributed/
 *     distributed semantic analysis
 *
 * This grammar only adds the AI-training-specific structural context.
 *
 * A distributed-training strategy may therefore be lowered into the existing
 * distributed semantic model rather than creating a second distributed
 * semantics system.
 *
 * ============================================================================
 * 71. CONCURRENCY CONTRACT
 * ============================================================================
 *
 * Parallel execution is a semantic concept.
 *
 * This grammar does not redefine:
 *
 *     parallel
 *     task
 *     future
 *     spawn
 *     synchronization primitives
 *
 * Existing concurrency grammar remains authoritative.
 *
 * Distributed training may consume those semantics downstream.
 *
 * ============================================================================
 * 72. NETWORKING CONTRACT
 * ============================================================================
 *
 * Network communication is not transport selection.
 *
 * Distributed training may require:
 *
 *     capability("distributed.communication")
 *
 * without naming:
 *
 *     TCP;
 *     UDP;
 *     QUIC;
 *     MPI;
 *     RDMA;
 *     InfiniBand;
 *     vendor transport.
 *
 * ============================================================================
 * 73. HARDWARE CONTRACT
 * ============================================================================
 *
 * Hardware capability is abstract.
 *
 * Examples:
 *
 *     capability("tensor.compute")
 *     capability("distributed.collective")
 *     capability("accelerator.compute")
 *
 * are semantic requirements.
 *
 * They do not imply:
 *
 *     GPU 0
 *     GPU 1
 *     accelerator 3
 *
 * ============================================================================
 * 74. QUANTUM CONTRACT
 * ============================================================================
 *
 * Quantum training integration must preserve:
 *
 *     quantum::ir
 *
 * as the canonical quantum IR boundary.
 *
 * This grammar does not introduce:
 *
 *     QuantumTrainingIR
 *     DistributedQuantumTrainingIR
 *
 * or another quantum representation.
 *
 * ============================================================================
 * 75. CHECKPOINT / PERSISTENCE CONTRACT
 * ============================================================================
 *
 * Logical checkpoint intent is distinct from physical persistence.
 *
 * A source program may express checkpoint policy.
 *
 * The compiler/runtime determines the actual persistence realization.
 *
 * ============================================================================
 * 76. SECURITY CONTRACT
 * ============================================================================
 *
 * Security semantics are delegated to:
 *
 *     grammar/security/
 *     effects
 *     capability analysis
 *     runtime authorization
 *
 * The parser does not grant capabilities.
 *
 * ============================================================================
 * 77. COMPILER CONTRACT
 * ============================================================================
 *
 * The compiler may perform:
 *
 *     - strategy selection;
 *     - strategy decomposition;
 *     - graph partitioning;
 *     - specialization;
 *     - communication optimization;
 *     - collective selection;
 *     - placement;
 *     - scheduling;
 *     - checkpoint optimization;
 *     - fault-tolerance lowering;
 *     - accelerator lowering;
 *     - heterogeneous lowering.
 *
 * These are downstream decisions.
 *
 * ============================================================================
 * 78. RUNTIME CONTRACT
 * ============================================================================
 *
 * The runtime may determine:
 *
 *     - currently available resources;
 *     - resource health;
 *     - dynamic scaling;
 *     - actual placement;
 *     - actual communication paths;
 *     - checkpoint storage;
 *     - failure recovery.
 *
 * Runtime must not reinterpret source syntax as a different program merely
 * because resource availability changes.
 *
 * ============================================================================
 * 79. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This file is additive.
 *
 * It does not rename:
 *
 *     grammar/ai/training.g4
 *     grammar/ai/ai.g4
 *     grammar/distributed/distributed.g4
 *
 * Existing generic training syntax remains valid.
 *
 * The future composition relationship is:
 *
 *     AI
 *       |
 *       +--> Training
 *               |
 *               +--> AIDistributedTraining
 *
 * or, where the repository's root composition requires it:
 *
 *     Distributed
 *       |
 *       +--> AIDistributedTraining
 *
 * The leaf grammar itself remains independent of both composition roots.
 *
 * ============================================================================
 * 80. REQUIRED PARENT INTEGRATION
 * ============================================================================
 *
 * No existing file needs to be structurally rewritten merely to make this
 * leaf grammar internally complete.
 *
 * When composing it into the canonical AI grammar, the parent should import:
 *
 *     AIDistributedTraining
 *
 * and expose:
 *
 *     distributedTrainingConstruct
 *
 * through the AI/training domain dispatch.
 *
 * The parent must NOT duplicate the rules in this file.
 *
 * In particular, do NOT copy:
 *
 *     distributedTrainingDeclaration
 *     distributedTrainingMember
 *     distributedTrainingInvocation
 *
 * into:
 *
 *     ai.g4
 *     training.g4
 *     distributed.g4
 *
 * The imported rule is the single authority.
 *
 * ============================================================================
 * 81. EXISTING `training.g4` INTEGRATION
 * ============================================================================
 *
 * The repository currently contains:
 *
 *     training.g4
 *
 * with an existing generic:
 *
 *     trainingDistributedIntent
 *
 * That rule should remain the generic training compatibility surface until
 * the parent composition is migrated.
 *
 * Once parent composition is updated, the preferred relationship is:
 *
 *     training.g4
 *          |
 *          +--> AIDistributedTraining
 *
 * rather than maintaining a second complete distributed-training grammar in
 * training.g4.
 *
 * The migration MUST preserve existing source compatibility according to:
 *
 *     grammar/compatibility/
 *
 * This file therefore does not redefine `trainingDistributedIntent`.
 *
 * ============================================================================
 * 82. EXISTING `ai.g4` INTEGRATION
 * ============================================================================
 *
 * The current `ai.g4` contains older annotation-oriented AI constructs.
 *
 * This file deliberately does not depend on:
 *
 *     ai.g4
 *
 * because doing so would reverse the leaf/composition relationship.
 *
 * The AI composition root may import this grammar.
 *
 * ============================================================================
 * 83. EXISTING DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * `grammar/distributed/distributed.g4` remains the distributed composition
 * root.
 *
 * This file does not duplicate:
 *
 *     nodes;
 *     services;
 *     processes;
 *     actors;
 *     channels;
 *     communication;
 *     messaging;
 *     collective;
 *     replication;
 *     consistency;
 *     placement;
 *     partitioning;
 *     topology;
 *     remote execution;
 *     deployment;
 *     fault tolerance;
 *     transactions.
 *
 * Distributed-training semantics consume those concepts downstream.
 *
 * ============================================================================
 * 84. RESOURCE INTEGRATION
 * ============================================================================
 *
 * Resource requirements are intentionally expression-based.
 *
 * This permits:
 *
 *     requires qubits >= logical_qubits;
 *     requires memory >= required_memory;
 *     requires nodes >= required_nodes;
 *     requires capability("tensor.compute");
 *     requires capability("distributed.training");
 *     requires capability("collective.reduce");
 *
 * without creating a new resource grammar.
 *
 * ============================================================================
 * 85. EXAMPLE SOURCE FOR CONFORMANCE TESTS
 * ============================================================================
 *
 * Minimal:
 *
 *     @distributed_training Train {
 *     }
 *
 * Model/data intent:
 *
 *     @distributed_training Train {
 *         @model = model;
 *         @dataset = dataset;
 *     }
 *
 * Data parallel:
 *
 *     @distributed_training Train {
 *         @strategy(data_parallel);
 *         @partition(shard(data));
 *         @synchronization(gradient_sync);
 *     }
 *
 * Model/tensor parallel:
 *
 *     @distributed_training Train {
 *         @strategy(model_parallel);
 *         @tensor_strategy(tensor_parallel);
 *     }
 *
 * Elastic:
 *
 *     @distributed_training Train {
 *         @scaling {
 *             desired = parallelism;
 *             policy = elastic_policy;
 *         }
 *     }
 *
 * Resource-aware:
 *
 *     @distributed_training Train {
 *         requires capability("distributed.training");
 *         requires capability("collective.reduce");
 *         requires memory >= required_memory;
 *         prefer capability("accelerated.tensor.compute");
 *     }
 *
 * Checkpoint/recovery:
 *
 *     @distributed_training Train {
 *         @checkpoint(checkpoint_policy);
 *         @recovery(recovery_policy);
 *     }
 *
 * Federated:
 *
 *     @distributed_training Train {
 *         @federated {
 *             participants = participants;
 *             policy = federation_policy;
 *         }
 *     }
 *
 * Hybrid:
 *
 *     @distributed_training Train {
 *         @quantum(hybrid_training);
 *     }
 *
 * Future strategy:
 *
 *     @distributed_training Train {
 *         @future_strategy(custom_strategy(configuration));
 *     }
 *
 * None of these examples selects a physical worker, node, GPU, QPU, or
 * accelerator.
 *
 * ============================================================================
 * 86. NEGATIVE EXAMPLES
 * ============================================================================
 *
 * The following must NOT become required grammar forms:
 *
 *     use_gpu(0);
 *     use_node(0);
 *     use_worker(0);
 *     use_qpu(0);
 *     assign_gpu(0);
 *     assign_node(0);
 *
 * They may exist only as explicitly target-specific language extensions whose
 * semantics are separately specified.
 *
 * Likewise, the grammar must not establish:
 *
 *     exactly 8 workers
 *     exactly 32 GPUs
 *     exactly 1024 nodes
 *     exactly 24 GB VRAM
 *
 * as universal language rules.
 *
 * ============================================================================
 * 87. AMBIGUITY CONTRACT
 * ============================================================================
 *
 * The declaration starts with:
 *
 *     AT
 *
 * and the distributed-training body members are structurally differentiated by
 * their leading form:
 *
 *     REQUIRES
 *     CAPABILITY
 *     CONSTRAINT
 *     PREFER
 *     HINT
 *     AT
 *     statement
 *
 * Annotated members use:
 *
 *     AT identifier ...
 *
 * and their payload is differentiated by:
 *
 *     ASSIGN
 *     LPAREN
 *     LBRACE
 *     expression
 *
 * The grammar must not use semantic predicates to distinguish those cases.
 *
 * ============================================================================
 * 88. ERROR CONTRACT
 * ============================================================================
 *
 * Syntax errors are handled by the canonical ANTLR parser error strategy.
 *
 * This grammar does not embed error actions.
 *
 * Semantic diagnostics remain downstream.
 *
 * Suggested semantic diagnostic identifiers include:
 *
 *     AI_DISTRIBUTED_TRAINING_INVALID_DECLARATION
 *     AI_DISTRIBUTED_TRAINING_UNKNOWN_STRATEGY
 *     AI_DISTRIBUTED_TRAINING_INVALID_COMPOSITION
 *     AI_DISTRIBUTED_TRAINING_INVALID_PARTITION
 *     AI_DISTRIBUTED_TRAINING_INVALID_SYNCHRONIZATION
 *     AI_DISTRIBUTED_TRAINING_INVALID_REPLICATION
 *     AI_DISTRIBUTED_TRAINING_INVALID_CONSISTENCY
 *     AI_DISTRIBUTED_TRAINING_INVALID_ELASTICITY
 *     AI_DISTRIBUTED_TRAINING_RESOURCE_UNSATISFIABLE
 *     AI_DISTRIBUTED_TRAINING_CAPABILITY_UNSATISFIABLE
 *     AI_DISTRIBUTED_TRAINING_SECURITY_VIOLATION
 *     AI_DISTRIBUTED_TRAINING_PORTABILITY_VIOLATION
 *     AI_DISTRIBUTED_TRAINING_QUANTUM_SEMANTIC_VIOLATION
 *
 * These are diagnostic contracts, not parser tokens.
 *
 * ============================================================================
 * 89. TEST CONTRACT
 * ============================================================================
 *
 * Required positive tests:
 *
 *     - minimal distributed training;
 *     - model reference;
 *     - dataset reference;
 *     - data-parallel intent;
 *     - model-parallel intent;
 *     - tensor-parallel intent;
 *     - pipeline-parallel intent;
 *     - partitioning;
 *     - sharding;
 *     - replication;
 *     - synchronization;
 *     - elasticity;
 *     - checkpointing;
 *     - recovery;
 *     - federated training;
 *     - heterogeneous training;
 *     - quantum/classical training;
 *     - resource requirements;
 *     - capability requirements;
 *     - constraints;
 *     - preferences;
 *     - hints;
 *     - nested extensions;
 *     - arbitrary user-defined strategy names.
 *
 * Required negative tests:
 *
 *     - missing declaration name;
 *     - missing braces;
 *     - malformed annotation;
 *     - malformed invocation;
 *     - malformed assignment;
 *     - empty invocation where prohibited;
 *     - malformed requirement;
 *     - malformed capability;
 *     - malformed nested region.
 *
 * Required boundary tests:
 *
 *     - zero members;
 *     - one member;
 *     - many members;
 *     - deeply nested logical regions;
 *     - long qualified identifiers;
 *     - symbolic resource expressions;
 *     - very large numeric program values;
 *     - very large source structures.
 *
 * Required scalability tests:
 *
 *     - many logical partitions;
 *     - many logical stages;
 *     - many strategy clauses;
 *     - many nested regions;
 *     - large model references;
 *     - large dataset expressions;
 *     - symbolic worker/resource quantities;
 *     - dynamically determined parallelism.
 *
 * None of these tests may establish an artificial language maximum.
 *
 * ============================================================================
 * 90. HARD-CODING AUDIT
 * ============================================================================
 *
 * A production validation pass MUST reject additions that introduce universal
 * language limits such as:
 *
 *     MAX_WORKERS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_GPUS
 *     MAX_CPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_TENSOR_RANK
 *     MAX_PARAMETERS
 *     MAX_SHARDS
 *     MAX_REPLICAS
 *     MAX_PARTITIONS
 *
 * It must also detect semantic equivalents.
 *
 * The following are allowed:
 *
 *     requires workers >= desired_workers;
 *     requires memory >= required_memory;
 *     requires nodes >= required_nodes;
 *     requires capability("distributed.training");
 *
 * because these are program/resource requirements.
 *
 * ============================================================================
 * 91. NO VENDOR LOCK-IN
 * ============================================================================
 *
 * The grammar MUST NOT enumerate:
 *
 *     CUDA
 *     ROCm
 *     TPU
 *     NPU
 *     MPI
 *     NCCL
 *     vendor-specific collective libraries
 *     vendor-specific training frameworks
 *
 * as mandatory syntax.
 *
 * Vendor-specific integration belongs in:
 *
 *     dialects/
 *     interoperability/
 *     hardware/
 *     compile/
 *     target-specific backends.
 *
 * ============================================================================
 * 92. NO FRAMEWORK LOCK-IN
 * ============================================================================
 *
 * The grammar does not enumerate:
 *
 *     PyTorch
 *     TensorFlow
 *     JAX
 *     ONNX
 *
 * or future frameworks.
 *
 * Framework names may remain ordinary semantic/library identifiers.
 *
 * ============================================================================
 * 93. NO ALGORITHM LOCK-IN
 * ============================================================================
 *
 * The grammar does not enumerate a fixed optimizer or distributed-training
 * algorithm list.
 *
 * For example:
 *
 *     Adam
 *     SGD
 *     AdamW
 *     L-BFGS
 *
 * remain semantic/library identifiers rather than language keywords.
 *
 * ============================================================================
 * 94. REPRODUCIBILITY
 * ============================================================================
 *
 * Reproducibility intent may be expressed through ordinary annotations,
 * expressions, and requirements.
 *
 * The parser does not silently introduce:
 *
 *     seeds;
 *     deterministic ordering;
 *     random-state policies;
 *     synchronization policy.
 *
 * Those must be explicit where required by the language semantics.
 *
 * ============================================================================
 * 95. IR INTEGRATION
 * ============================================================================
 *
 * The lowering path is:
 *
 *     AIDistributedTraining parse tree
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic distributed-training model
 *          |
 *          v
 *     canonical compiler representation
 *
 * Distributed metadata must remain attached to canonical semantic operations
 * rather than creating a competing distributed-training IR.
 *
 * If a training operation contains quantum computation:
 *
 *     quantum semantic lowering
 *             |
 *             v
 *         quantum::ir
 *
 * remains canonical.
 *
 * ============================================================================
 * 96. COMPILER / RUNTIME SEPARATION
 * ============================================================================
 *
 * Compiler responsibilities may include:
 *
 *     strategy lowering;
 *     partitioning;
 *     specialization;
 *     communication planning;
 *     placement;
 *     scheduling;
 *     optimization;
 *     target adaptation.
 *
 * Runtime responsibilities may include:
 *
 *     resource discovery;
 *     dynamic scaling;
 *     health observation;
 *     recovery;
 *     checkpoint persistence;
 *     execution monitoring.
 *
 * Neither responsibility belongs in this grammar.
 *
 * ============================================================================
 * 97. RUST INTEGRATION
 * ============================================================================
 *
 * This grammar generates parser artifacts consumed by the Rust implementation.
 *
 * The Rust implementation must support:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and:
 *
 *     NO unsafe
 *
 * This grammar itself contains no Rust target actions.
 *
 * Generated parser code must be treated as generated output.
 *
 * Handwritten semantic/compiler code must use safe Rust.
 *
 * ============================================================================
 * 98. TOOLING CONTRACT
 * ============================================================================
 *
 * The grammar must remain consumable by:
 *
 *     - parser generation;
 *     - syntax highlighting;
 *     - formatter;
 *     - language server;
 *     - source indexing;
 *     - documentation tooling;
 *     - conformance testing.
 *
 * Annotation names must remain recoverable from parse trees.
 *
 * ============================================================================
 * 99. VERSIONING
 * ============================================================================
 *
 * Changes to:
 *
 *     distributedTrainingDeclaration
 *     distributedTrainingMember
 *     annotation structure
 *     requirement structure
 *     expression structure
 *
 * are language compatibility changes if they alter accepted source syntax.
 *
 * Such changes must be recorded through:
 *
 *     grammar/compatibility/
 *     grammar/spec/compatibility.md
 *
 * ============================================================================
 * 100. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] It is a dedicated distributed-training leaf grammar.
 *
 * [x] It does not replace `training.g4`.
 *
 * [x] It does not replace `distributed/distributed.g4`.
 *
 * [x] It does not create a second distributed IR.
 *
 * [x] It does not create a second AI IR.
 *
 * [x] It preserves the canonical frontend AST boundary.
 *
 * [x] It preserves the canonical expression grammar.
 *
 * [x] It preserves the canonical type grammar.
 *
 * [x] It preserves the canonical statement grammar.
 *
 * [x] It preserves the canonical resource model.
 *
 * [x] It preserves the canonical distributed model.
 *
 * [x] It preserves the canonical quantum::ir boundary.
 *
 * [x] It does not enumerate hardware.
 *
 * [x] It does not enumerate workers.
 *
 * [x] It does not enumerate nodes.
 *
 * [x] It does not enumerate accelerators.
 *
 * [x] It does not enumerate AI frameworks.
 *
 * [x] It does not enumerate distributed algorithms.
 *
 * [x] It does not impose machine-size limits.
 *
 * [x] It supports symbolic resource quantities.
 *
 * [x] It supports dynamically determined scaling.
 *
 * [x] It supports future distributed-training concepts.
 *
 * [x] It supports classical training.
 *
 * [x] It supports hybrid training.
 *
 * [x] It supports quantum/classical training integration.
 *
 * [x] It supports fault-tolerance intent.
 *
 * [x] It supports checkpoint intent.
 *
 * [x] It supports federated intent.
 *
 * [x] It supports heterogeneous execution intent.
 *
 * [x] It supports resource/capability contracts.
 *
 * [x] It contains no embedded Rust.
 *
 * [x] It requires no unsafe Rust.
 *
 * [x] It is compatible with Rust 1.97 / 1.97.1 parser integration.
 *
 * [x] It has explicit AST integration.
 *
 * [x] It has explicit semantic integration.
 *
 * [x] It has explicit IR integration.
 *
 * [x] It has explicit compiler integration.
 *
 * [x] It has explicit runtime integration.
 *
 * [x] It has explicit compatibility integration.
 *
 * [x] It has explicit validation and test requirements.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * Distributed training is a property of a logical training computation.
 *
 * It is NOT a synonym for:
 *
 *     GPU;
 *     cluster;
 *     node;
 *     worker;
 *     machine;
 *     network;
 *     accelerator.
 *
 * Therefore:
 *
 *     distributed-training syntax
 *          !=
 *     physical deployment syntax
 *
 * and:
 *
 *     logical parallelism
 *          !=
 *     fixed hardware capacity
 *
 * and:
 *
 *     resource requirement
 *          !=
 *     resource selection
 *
 * and:
 *
 *     capability
 *          !=
 *     physical device identity
 *
 * and:
 *
 *     source syntax
 *          !=
 *     execution strategy.
 *
 * The compiler/runtime remain responsible for transforming the same logical
 * source program into an appropriate realization for the resources and
 * capabilities actually available.
 *
 * ============================================================================
 */
/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/ai/federated-learning.g4
 *
 * Grammar:
 *     AIFederatedLearning
 *
 * Status:
 *     CANONICAL FEDERATED-LEARNING SOURCE-SYNTAX COMPONENT
 *
 * Language baseline:
 *     Rust 1.97+
 *     Rust edition 2021
 *
 * Safety:
 *     - Pure ANTLR4 parser grammar.
 *     - No embedded Rust actions.
 *     - No semantic predicates.
 *     - No unsafe Rust.
 *     - No filesystem access.
 *     - No network access.
 *     - No hardware discovery.
 *     - No runtime execution.
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file owns source-level federated-learning intent.
 *
 * Federated learning is treated as a form of distributed learning in which
 * model/data computation may be coordinated across independently managed
 * execution participants without requiring the source program to encode a
 * particular physical machine topology.
 *
 * This grammar provides structural syntax for:
 *
 *     - federated-learning declarations;
 *     - participants;
 *     - participant roles;
 *     - coordination;
 *     - rounds;
 *     - phases;
 *     - local training;
 *     - aggregation;
 *     - synchronization;
 *     - partitioning;
 *     - evaluation;
 *     - communication intent;
 *     - privacy intent;
 *     - security intent;
 *     - fault/recovery intent;
 *     - adaptation intent;
 *     - reproducibility intent;
 *     *     - provenance intent;
 *     *     - requirements;
 *     *     - capabilities;
 *     *     - constraints;
 *     *     - preferences;
 *     *     - hints;
 *     *     - extensible future federated-learning constructs.
 *
 * The grammar describes portable computation intent.
 *
 * It does NOT implement:
 *
 *     - a federation runtime;
 *     - a networking protocol;
 *     - a cryptographic primitive;
 *     - a privacy mechanism;
 *     - a model;
 *     - a training algorithm;
 *     - an aggregation algorithm;
 *     - a scheduler;
 *     - participant discovery;
 *     - resource allocation;
 *     - hardware selection;
 *     - device selection;
 *     - topology construction;
 *     - distributed consensus;
 *     - fault recovery;
 *     - secure execution;
 *     - quantum execution;
 *     - HDL generation;
 *     - canonical IR construction.
 *
 * ============================================================================
 * ARCHITECTURE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     ZamaniParser
 *          |
 *          v
 *     AIFederatedLearning
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *     +----+---------+----------+-----------+
 *     |              |          |           |
 *     v              v          v           v
 *   AI semantics  Effects   Resources   Capabilities
 *     |              |          |           |
 *     +--------------+----------+-----------+
 *                    |
 *                    v
 *              Policy analysis
 *                    |
 *                    v
 *               Provenance
 *                    |
 *                    v
 *          canonical semantic model
 *                    |
 *          +---------+---------+
 *          |                   |
 *          v                   v
 *     classical lowering   distributed lowering
 *          |                   |
 *          +---------+---------+
 *                    |
 *                    v
 *               execution plan
 *                    |
 *                    v
 *                 runtime
 *
 * If federated computation contains quantum computation, the quantum semantic
 * portion crosses the existing canonical boundary:
 *
 *     semantic model
 *          |
 *          v
 *      quantum::ir
 *          |
 *          v
 *      optimization
 *          |
 *          v
 *      routing / scheduling / resilience
 *          |
 *          v
 *      ZQN / HAL
 *
 * This grammar never creates or owns quantum::ir.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     federatedLearningConstruct
 *     federatedLearningDeclaration
 *     federatedLearningBody
 *     federatedLearningMember
 *     federatedParticipant
 *     federatedParticipantBody
 *     federatedRound
 *     federatedPhase
 *     federatedRequirement
 *     federatedCapability
 *     federatedConstraint
 *     federatedPreference
 *     federatedHint
 *     federatedAnnotatedMember
 *     federatedMemberPayload
 *     federatedBinding
 *     federatedCall
 *     federatedBlock
 *     federatedStatement
 *     federatedExpression
 *     federatedTypeClause
 *     federatedInitializer
 *     federatedAnnotation
 *     federatedAnnotationName
 *
 * THIS FILE DOES NOT OWN:
 *
 *     identifiers
 *     qualified names
 *     expressions
 *     types
 *     statements
 *     generic training syntax
 *     model declarations
 *     dataset declarations
 *     tensor declarations
 *     actor syntax
 *     channel syntax
 *     network syntax
 *     policy semantics
 *     security semantics
 *     capability identity
 *     resource identity
 *     effect identity
 *     provenance representation
 *     quantum syntax
 *     HDL syntax
 *     hardware syntax
 *     distributed-runtime implementation
 *     IR
 *     scheduling
 *     routing
 *     deployment
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/types/types.g4
 *     grammar/expressions/
 *     grammar/statements/
 *     grammar/core/names.g4
 *     grammar/core/capabilities.g4
 *
 * The grammar uses the canonical token vocabulary:
 *
 *     ZamaniLexer
 *
 * It does not define lexer rules.
 *
 * ============================================================================
 * EXPORTS
 * ============================================================================
 *
 * Primary:
 *
 *     federatedLearningConstruct
 *
 * Secondary:
 *
 *     federatedLearningDeclaration
 *     federatedLearningBody
 *     federatedLearningMember
 *     federatedParticipant
 *     federatedRound
 *     federatedPhase
 *     federatedRequirement
 *     federatedCapability
 *     federatedConstraint
 *     federatedPreference
 *     federatedHint
 *
 * ============================================================================
 * CONSUMED BY
 * ============================================================================
 *
 *     grammar/ai/ai.g4
 *
 * Future AI composition layers may consume this grammar through:
 *
 *     federatedLearningConstruct
 *
 * It must not be imported directly by:
 *
 *     grammar/Zamani.g4
 *
 * because domain composition belongs to:
 *
 *     grammar/antlr/ZamaniParser.g4
 *     grammar/ai/ai.g4
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates parser contexts only.
 *
 * The frontend AST must represent federated learning using domain-neutral
 * structures.
 *
 * The AST should preserve, where present:
 *
 *     - declaration identity;
 *     - participant identity;
 *     - participant role;
 *     - round/phase structure;
 *     - model references;
 *     - dataset references;
 *     - expressions;
 *     - requirements;
 *     - capabilities;
 *     - constraints;
 *     - preferences;
 *     - hints;
 *     - source locations;
 *     - annotations;
 *     - source ordering;
 *     - provenance information.
 *
 * The AST MUST NOT contain:
 *
 *     - physical node IDs;
 *     - physical CPU IDs;
 *     - GPU IDs;
 *     - device IDs;
 *     - network addresses as mandatory execution identity;
 *     - fixed worker counts;
 *     - fixed machine topology;
 *     - vendor runtime objects;
 *     - cryptographic implementation objects;
 *     - aggregation implementation objects;
 *     - physical storage locations.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     - whether the declaration is valid;
 *     - whether participants are valid;
 *     - participant role semantics;
 *     - model compatibility;
 *     - dataset compatibility;
 *     - training compatibility;
 *     - aggregation compatibility;
 *     - synchronization semantics;
 *     - privacy requirements;
 *     - security requirements;
 *     - communication effects;
 *     - adaptation effects;
 *     - resource requirements;
 *     - capability requirements;
 *     - policy constraints;
 *     - reproducibility requirements;
 *     - provenance requirements;
 *     - fault/recovery semantics.
 *
 * None of these decisions are performed by this grammar.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Type syntax belongs to the canonical type system.
 *
 * This file consumes:
 *
 *     typeExpression
 *
 * It does not define:
 *
 *     FederatedType
 *     ParticipantType
 *     AggregationType
 *     PrivacyType
 *     FederationType
 *
 * as competing type systems.
 *
 * ============================================================================
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * All values and predicates use the canonical:
 *
 *     expression
 *
 * This permits federated learning to operate over:
 *
 *     classical values;
 *     tensors;
 *     models;
 *     datasets;
 *     distributed values;
 *     probabilistic values;
 *     quantum-derived values;
 *     hardware-backed values;
 *     future domain values.
 *
 * This file MUST NOT define another expression hierarchy.
 *
 * ============================================================================
 * TRAINING INTEGRATION
 * ============================================================================
 *
 * Federated learning is a distributed specialization of learning intent.
 *
 * The existing:
 *
 *     grammar/ai/training.g4
 *
 * remains the canonical owner of generic training declarations.
 *
 * This grammar does NOT copy training syntax.
 *
 * A federated-learning body may refer to:
 *
 *     training(...)
 *
 * or another registered training construct through ordinary expressions and
 * extensible annotations.
 *
 * The semantic layer is responsible for connecting federated learning with
 * AITraining.
 *
 * The relationship is:
 *
 *     federated learning
 *           |
 *           +---- model
 *           |
 *           +---- data
 *           |
 *           +---- local training
 *           |
 *           +---- coordination
 *           |
 *           +---- aggregation
 *           |
 *           +---- evaluation
 *           |
 *           v
 *       distributed learning semantics
 *
 * ============================================================================
 * DISTRIBUTED COMPUTATION CONTRACT
 * ============================================================================
 *
 * Federated learning integrates with the existing distributed subsystem.
 *
 * This grammar does not create another actor/task/channel system.
 *
 * A participant may ultimately be represented semantically using:
 *
 *     actor
 *     task
 *     service
 *     worker
 *     execution context
 *
 * according to the existing distributed/concurrency architecture.
 *
 * This file expresses federated intent only.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Capability identity is owned by:
 *
 *     grammar/core/capabilities.g4
 *
 * This file consumes capability syntax.
 *
 * It does not define AI-specific capability identities.
 *
 * Examples that may be represented through the canonical capability system
 * include:
 *
 *     ai::federated_learning
 *     ai::distributed_training
 *     ai::secure_aggregation
 *     ai::privacy_preserving_computation
 *     distributed::coordination
 *     distributed::communication
 *     cryptography::secure_computation
 *     tensor::compute
 *     quantum::measurement
 *
 * The grammar does not enumerate these as a closed list.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Federated learning may express resource requirements through:
 *
 *     requires expression;
 *
 * capabilities through:
 *
 *     capability expression;
 *
 * constraints through:
 *
 *     constraint expression;
 *
 * preferences through:
 *
 *     prefer expression;
 *
 * hints through:
 *
 *     hint expression;
 *
 * These remain abstract.
 *
 * Examples of valid semantic intent include:
 *
 *     requires capability("distributed.training");
 *     requires capability("tensor.compute");
 *     requires capability("secure.communication");
 *     requires memory >= required_memory;
 *
 * The grammar does not select:
 *
 *     a specific machine;
 *     a specific node;
 *     a specific accelerator;
 *     a specific network;
 *     a specific device.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Federated learning may semantically involve effects such as:
 *
 *     learning
 *     adaptation
 *     network
 *     distributed
 *     randomness
 *     mutation
 *     foreign
 *     native
 *     simulation
 *
 * This grammar does not define effect semantics.
 *
 * Effect analysis determines which effects are produced or required.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Federated learning can be governed by policies controlling:
 *
 *     participation;
 *     authorization;
 *     privacy;
 *     communication;
 *     adaptation;
 *     data use;
 *     aggregation;
 *     deployment;
 *     fault recovery;
 *     resource use;
 *     security.
 *
 * Policy syntax remains owned by the policy/security subsystem.
 *
 * This grammar merely preserves policy-related expressions and annotations.
 *
 * ============================================================================
 * PRIVACY / SECURITY CONTRACT
 * ============================================================================
 *
 * Privacy is an intent and policy concern, not an algorithm catalogue.
 *
 * The grammar therefore does not enumerate:
 *
 *     particular privacy mechanisms;
 *     particular cryptographic schemes;
 *     particular secure-aggregation protocols;
 *     particular threat models.
 *
 * Such mechanisms are represented through:
 *
 *     capabilities;
 *     policies;
 *     library identifiers;
 *     dialects;
 *     semantic metadata;
 *     interoperability contracts.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Federated-learning source constructs must remain traceable.
 *
 * Downstream provenance may record:
 *
 *     declaration;
 *     participant;
 *     data source;
 *     model source;
 *     training operation;
 *     aggregation operation;
 *     policy;
 *     evidence;
 *     decision;
 *     transformation;
 *     execution result.
 *
 * This grammar does not create provenance records.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Federated learning may consume quantum-derived values or quantum
 * computation as part of a hybrid program.
 *
 * This file does NOT define:
 *
 *     qubits;
 *     quantum operations;
 *     quantum registers;
 *     physical qubits;
 *     coupling maps;
 *     routing;
 *     scheduling;
 *     QEC;
 *     calibration;
 *     ZQN;
 *     HAL.
 *
 * If a federated computation uses quantum computation:
 *
 *     federated source
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +----------------+
 *          |                |
 *          v                v
 *     distributed       quantum semantics
 *                           |
 *                           v
 *                       quantum::ir
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Hardware acceleration may be expressed through capabilities and resources.
 *
 * This grammar does not define:
 *
 *     register width;
 *     bus width;
 *     device count;
 *     accelerator count;
 *     memory capacity;
 *     physical topology.
 *
 * Hardware realization belongs downstream.
 *
 * ============================================================================
 * BACKEND CONTRACT
 * ============================================================================
 *
 * This grammar never:
 *
 *     selects a backend;
 *     selects a device;
 *     selects a node;
 *     selects a network;
 *     allocates memory;
 *     schedules work;
 *     routes communication.
 *
 * Backend selection is performed after semantic analysis and capability /
 * resource negotiation.
 *
 * ============================================================================
 * OPEN-WORLD CONTRACT
 * ============================================================================
 *
 * Federated learning concepts remain open-ended.
 *
 * The grammar does not enumerate:
 *
 *     aggregation algorithms;
 *     optimization algorithms;
 *     privacy algorithms;
 *     communication protocols;
 *     model architectures;
 *     dataset formats;
 *     participant counts;
 *     federation sizes;
 *     hardware types;
 *     deployment providers.
 *
 * New concepts should normally be introduced through:
 *
 *     expressions;
 *     identifiers;
 *     capabilities;
 *     policies;
 *     libraries;
 *     dialects;
 *     semantic registries.
 *
 * A new algorithm must not require a new core lexer token.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * There are NO grammar-level limits for:
 *
 *     participants;
 *     rounds;
 *     phases;
 *     local steps;
 *     datasets;
 *     models;
 *     parameters;
 *     aggregation inputs;
 *     metrics;
 *     constraints;
 *     policies;
 *     capabilities;
 *     federation members;
 *     nested constructs;
 *     expression depth;
 *     participant groups.
 *
 * Grammar repetition uses:
 *
 *     *
 *     +
 *
 * where appropriate.
 *
 * Practical limits come only from:
 *
 *     compiler resources;
 *     parser resources;
 *     runtime resources;
 *     available memory;
 *     target resources;
 *     deployment policy;
 *     physical infrastructure.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This file MUST NOT introduce:
 *
 *     MAX_PARTICIPANTS
 *     MAX_CLIENTS
 *     MAX_SERVERS
 *     MAX_ROUNDS
 *     MAX_PHASES
 *     MAX_LOCAL_STEPS
 *     MAX_DATASETS
 *     MAX_MODELS
 *     MAX_PARAMETERS
 *     MAX_AGGREGATORS
 *     MAX_NODES
 *     MAX_WORKERS
 *     MAX_DEVICES
 *     MAX_GPUS
 *     MAX_CPUS
 *     MAX_MEMORY
 *     MAX_NETWORK_SIZE
 *
 * It must not encode:
 *
 *     client 0
 *     node 0
 *     device 0
 *
 * as universal execution assumptions.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no actions;
 *     no predicates;
 *     no I/O;
 *     no network calls;
 *     no hardware inspection;
 *     no randomness;
 *     no environment inspection.
 *
 * Given identical:
 *
 *     source;
 *     lexer version;
 *     grammar version;
 *     dialect configuration;
 *
 * parsing is deterministic.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This is a new canonical grammar component.
 *
 * It must not revive obsolete grammar constructs.
 *
 * Compatibility is maintained through:
 *
 *     grammar/compatibility/
 *
 * Historical spellings should be migrated through compatibility tooling rather
 * than duplicated parser rules.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Required test ownership:
 *
 *     grammar/tests/ai/federated-learning/
 *
 * Required categories:
 *
 *     positive;
 *     negative;
 *     boundary;
 *     scalability;
 *     cross-domain;
 *     compatibility;
 *     determinism;
 *     resource;
 *     capability;
 *     policy;
 *     provenance;
 *     quantum;
 *     hardware;
 *     distributed;
 *     interoperability.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [ ] ANTLR generation succeeds.
 *     [ ] `AIFederatedLearning` resolves with ZamaniLexer.
 *     [ ] `federatedLearningConstruct` is imported by AI composition.
 *     [ ] No lexer changes are required merely to use federated learning.
 *     [ ] No duplicate training grammar exists here.
 *     [ ] No duplicate actor/concurrency grammar exists here.
 *     [ ] No duplicate capability grammar exists here.
 *     [ ] No duplicate resource grammar exists here.
 *     [ ] No duplicate policy grammar exists here.
 *     [ ] No duplicate expression grammar exists here.
 *     [ ] Positive tests pass.
 *     [ ] Negative tests pass.
 *     [ ] Boundary tests pass.
 *     [ ] Scalability tests pass.
 *     [ ] Cross-domain tests pass.
 *     [ ] Determinism tests pass.
 *     [ ] Safe Rust 1.97+ generation/integration passes.
 *
 * ============================================================================
 */

parser grammar AIFederatedLearning;

options {
    tokenVocab = ZamaniLexer;
}

import
    Types,
    Expressions,
    Statements,
    Capabilities;


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Federated learning is deliberately a distinct leaf-domain construct.
 *
 * It does not accept an arbitrary expression as a federated-learning
 * declaration.
 * ============================================================================
 */

federatedLearningConstruct
    : federatedLearningDeclaration
    ;


/*
 * ============================================================================
 * DECLARATION
 * ============================================================================
 *
 * Canonical structural form:
 *
 *     @federated_learning Federation
 *     {
 *         ...
 *     }
 *
 * The annotation spelling is intentionally represented as an identifier.
 *
 * Semantic validation determines whether the annotation identifies the
 * canonical federated-learning construct.
 *
 * This avoids permanently reserving another lexer keyword.
 * ============================================================================
 */

federatedLearningDeclaration
    : federatedLearningAnnotation
      identifier
      federatedTypeClause?
      federatedInitializer?
      federatedLearningBody?
      SEMICOLON?
    ;


federatedLearningAnnotation
    : AT
      federatedAnnotationName
    ;


federatedAnnotationName
    : IDENTIFIER
    ;


federatedTypeClause
    : COLON
      typeExpression
    ;


federatedInitializer
    : ASSIGN
      expression
    ;


/*
 * ============================================================================
 * BODY
 * ============================================================================
 *
 * The body is unbounded.
 *
 * There is deliberately no finite participant/round/member count.
 * ============================================================================
 */

federatedLearningBody
    : LBRACE
      federatedLearningMember*
      RBRACE
    ;


/*
 * ============================================================================
 * MEMBER DISPATCH
 * ============================================================================
 */

federatedLearningMember
    : federatedParticipant
    | federatedRound
    | federatedPhase
    | federatedRequirement
    | federatedCapability
    | federatedConstraint
    | federatedPreference
    | federatedHint
    | federatedAnnotatedMember
    | federatedStatement
    ;


/*
 * ============================================================================
 * PARTICIPANT
 * ============================================================================
 *
 * Participant is a logical federation member.
 *
 * It is NOT:
 *
 *     a physical machine;
 *     a node ID;
 *     a CPU;
 *     a GPU;
 *     a process ID;
 *     a network address.
 *
 * Example:
 *
 *     @participant client_group {
 *         @role client;
 *         @data local_data;
 *     }
 *
 * ============================================================================
 */

federatedParticipant
    : AT
      federatedParticipantKeyword
      identifier
      federatedParticipantBody?
      SEMICOLON?
    ;


federatedParticipantKeyword
    : IDENTIFIER
    ;


federatedParticipantBody
    : LBRACE
      federatedLearningMember*
      RBRACE
    ;


/*
 * ============================================================================
 * ROUND
 * ============================================================================
 *
 * A round is a logical coordination unit.
 *
 * It does not imply:
 *
 *     one network synchronization;
 *     one physical clock;
 *     one hardware execution step;
 *     one fixed number of participants.
 *
 * Those meanings are semantic.
 * ============================================================================
 */

federatedRound
    : AT
      federatedRoundKeyword
      identifier
      federatedRoundPayload?
      SEMICOLON?
    ;


federatedRoundKeyword
    : IDENTIFIER
    ;


federatedRoundPayload
    : federatedBlock
    | ASSIGN expression
    | LPAREN
      argumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * PHASE
 * ============================================================================
 *
 * Phases are generic semantic regions.
 *
 * Examples:
 *
 *     @phase initialization { ... }
 *     @phase local_training { ... }
 *     @phase aggregation { ... }
 *     @phase evaluation { ... }
 *
 * No phase names are reserved by the grammar.
 * ============================================================================
 */

federatedPhase
    : AT
      federatedPhaseKeyword
      identifier
      LBRACE
      federatedLearningMember*
      RBRACE
    ;


federatedPhaseKeyword
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * RESOURCE / CAPABILITY CONTRACTS
 * ============================================================================
 *
 * These use canonical expressions and capability syntax.
 * ============================================================================
 */

federatedRequirement
    : REQUIRES
      expression
      SEMICOLON
    ;


federatedCapability
    : CAPABILITY
      capabilityExpression
      SEMICOLON
    ;


federatedConstraint
    : CONSTRAINT
      expression
      SEMICOLON
    ;


federatedPreference
    : PREFER
      expression
      SEMICOLON
    ;


federatedHint
    : HINT
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * GENERIC FEDERATED ANNOTATION
 * ============================================================================
 *
 * This is the primary extensibility mechanism.
 *
 * Examples:
 *
 *     @model my_model;
 *     @data local_dataset;
 *     @objective objective;
 *     @aggregation aggregator;
 *     @synchronization synchronization_policy;
 *     @privacy privacy_policy;
 *     @security security_policy;
 *     @communication communication_policy;
 *     @evaluation evaluator;
 *     @provenance provenance_policy;
 *     @reproducibility reproducibility_policy;
 *     @adaptation adaptation_policy;
 *
 * The grammar intentionally does not enumerate those names.
 *
 * New semantic concepts therefore do not require:
 *
 *     lexer changes;
 *     parser keyword changes;
 *     a new universal grammar rule.
 *
 * ============================================================================
 */

federatedAnnotatedMember
    : federatedMemberAnnotation
      federatedMemberPayload
      SEMICOLON?
    ;


federatedMemberAnnotation
    : AT
      federatedAnnotationName
    ;


federatedMemberPayload
    : federatedBinding
    | federatedCall
    | federatedExpression
    | federatedBlock
    ;


federatedBinding
    : identifier
      federatedTypeClause?
      ASSIGN
      expression
    ;


federatedCall
    : identifier
      LPAREN
      argumentList?
      RPAREN
    ;


federatedExpression
    : expression
    ;


federatedBlock
    : LBRACE
      federatedLearningMember*
      RBRACE
    ;


/*
 * ============================================================================
 * ORDINARY ZAMANI STATEMENTS
 * ============================================================================
 *
 * Federated-learning bodies may contain ordinary Zamani statements.
 *
 * This permits:
 *
 *     local computation;
 *     classical computation;
 *     tensor operations;
 *     control flow;
 *     function calls;
 *     data processing;
 *     quantum-derived computation;
 *     other language constructs.
 *
 * The statement grammar remains the single statement authority.
 * ============================================================================
 */

federatedStatement
    : statement
    ;


/*
 * ============================================================================
 * REUSABLE REFERENCE BRIDGES
 * ============================================================================
 *
 * These rules intentionally remain expression-based.
 *
 * A reference may semantically denote:
 *
 *     model;
 *     dataset;
 *     tensor;
 *     participant group;
 *     policy;
 *     capability;
 *     resource;
 *     execution context;
 *     quantum result;
 *     classical result;
 *     distributed value;
 *     future-domain value.
 * ============================================================================
 */

federatedModelReference
    : expression
    ;


federatedDatasetReference
    : expression
    ;


federatedParticipantReference
    : expression
    ;


federatedAggregationReference
    : expression
    ;


federatedPolicyReference
    : expression
    ;


federatedCapabilityReference
    : capabilityReference
    ;


/*
 * ============================================================================
 * CROSS-DOMAIN CONTRACT
 * ============================================================================
 *
 * Federated learning may combine:
 *
 *     AI
 *     classical computation
 *     data
 *     distributed computation
 *     networking
 *     security
 *     quantum computation
 *     hardware acceleration
 *     simulation
 *     interoperability
 *     metaprogramming
 *
 * The source construct remains one Zamani program.
 *
 * No domain-specific source language is introduced.
 * ============================================================================
 */
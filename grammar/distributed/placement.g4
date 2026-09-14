/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/distributed/placement.g4
 *
 * Grammar:
 *     Distributed placement
 *
 * Grammar kind:
 *     ANTLR4 parser grammar
 *
 * Target:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     - No embedded Rust actions.
 *     - No semantic predicates.
 *     - No unsafe Rust.
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
 * This file defines SOURCE-LEVEL DISTRIBUTED PLACEMENT INTENT.
 *
 * It answers:
 *
 *     "What placement relationships, requirements, constraints,
 *      preferences, capabilities, and mobility properties does this
 *      distributed computation express?"
 *
 * It does NOT answer:
 *
 *     "Which physical machine will execute it?"
 *
 * Physical realization is downstream.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 *
 *     - distributed placement declarations;
 *     - placement subjects;
 *     - placement domains;
 *     - placement locations;
 *     - placement selectors;
 *     - placement requirements;
 *     - placement constraints;
 *     - placement preferences;
 *     - placement hints;
 *     - placement affinity;
 *     - placement anti-affinity;
 *     - co-location;
 *     - separation;
 *     - locality;
 *     - topology-property intent;
 *     - resource-class intent;
 *     - capability intent;
 *     - placement policies;
 *     - placement groups;
 *     - placement alternatives;
 *     - migration intent;
 *     - elasticity intent;
 *     - replica placement intent;
 *     - placement metadata.
 *
 * THIS FILE DOES NOT OWN
 *
 *     - identifiers;
 *     - qualified names;
 *     - expressions;
 *     - types;
 *     - lexical tokens;
 *     - node discovery;
 *     - service discovery;
 *     - resource discovery;
 *     - hardware discovery;
 *     - topology construction;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - replication algorithms;
 *     - consensus;
 *     - consistency algorithms;
 *     - fault-tolerance algorithms;
 *     - runtime dispatch;
 *     - deployment;
 *     - network transport;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - resilience.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     lexer
 *       |
 *       v
 *     distributed/placement.g4
 *       |
 *       v
 *     placement syntax
 *       |
 *       v
 *     AST
 *       |
 *       +--> name resolution
 *       +--> type checking
 *       +--> capability checking
 *       +--> resource analysis
 *       +--> semantic validation
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +--> resource model
 *       +--> distributed IR
 *       +--> quantum::ir integration where applicable
 *       |
 *       +--> placement
 *       +--> routing
 *       +--> scheduling
 *       +--> hardware realization
 *       |
 *       v
 *     runtime
 *
 * This grammar does NOT call any of those downstream systems.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Distributed placement MUST preserve:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Therefore the grammar MUST NOT require:
 *
 *     - a fixed node count;
 *     - a fixed service count;
 *     - a fixed process count;
 *     - a fixed cluster size;
 *     - a fixed topology;
 *     - a fixed node identifier;
 *     - a fixed hostname;
 *     - a fixed IP address;
 *     - a fixed network port;
 *     - a fixed cloud provider;
 *     - a fixed region;
 *     - a fixed CPU count;
 *     - a fixed GPU count;
 *     - a fixed QPU count;
 *     - a fixed FPGA count;
 *     - a fixed memory capacity;
 *     - a fixed bandwidth;
 *     - a fixed latency.
 *
 * Physical properties may be expressed only as:
 *
 *     - semantic requirements;
 *     - constraints;
 *     - capabilities;
 *     - preferences;
 *     - hints;
 *     - symbolic selectors;
 *     - resource expressions;
 *     - target-independent properties.
 *
 * ============================================================================
 * OPEN-WORLD DESIGN
 * ============================================================================
 *
 * Distributed placement deliberately does not introduce a closed inventory
 * of node/device/provider types.
 *
 * Examples such as:
 *
 *     cpu
 *     gpu
 *     qpu
 *     fpga
 *     accelerator
 *     memory
 *     numa
 *     rack
 *     zone
 *     region
 *     cluster
 *
 * are represented as identifiers.
 *
 * This permits future resources without changing this grammar merely because
 * a new hardware or deployment class appears.
 *
 * ============================================================================
 * CANONICAL DEPENDENCIES
 * ============================================================================
 *
 * Names:
 *
 *     identifier
 *     qualifiedName
 *     nameReference
 *
 * Expressions:
 *
 *     expression
 *     expressionList
 *
 * This grammar MUST NOT redefine those rules.
 *
 * ============================================================================
 * BUILD CONTRACT
 * ============================================================================
 *
 * This parser grammar is intended to be composed with the canonical Zamani
 * lexer and shared parser grammar components.
 *
 * The canonical aggregate grammar MUST expose:
 *
 *     distributedPlacementDeclaration
 *
 * as the public entry rule.
 *
 * The distributed aggregate grammar should delegate its placement alternative
 * to this rule rather than redefining placement syntax.
 *
 * ============================================================================
 * IMPORTANT INTEGRATION RULE
 * ============================================================================
 *
 * grammar/distributed/distributed.g4 already owns the distributed-domain
 * aggregate and already identifies:
 *
 *     distributedPlacementDeclaration
 *
 * as part of its public distributed syntax.
 *
 * Therefore the final aggregate grammar MUST import/include this file and
 * delegate to:
 *
 *     distributedPlacementDeclaration
 *
 * It MUST NOT define a second competing placement grammar.
 *
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * Parsing determines structural validity.
 *
 * Semantic analysis determines:
 *
 *     - what a placement subject means;
 *     - whether a location exists semantically;
 *     - whether a capability is valid;
 *     - whether a requirement is satisfiable;
 *     - whether constraints conflict;
 *     - whether a preference is optional;
 *     - whether a placement is legal;
 *     - whether migration is permitted;
 *     - whether replication placement is compatible with consistency;
 *     - whether quantum resources can satisfy the request;
 *     - whether hardware can realize the request.
 *
 * None of those decisions belong in this grammar.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Distributed placement may refer to logical quantum resources.
 *
 * It MUST NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     quantum topology
 *     calibration
 *     pulse semantics
 *     QEC algorithms
 *     ZQN noise models
 *
 * Any quantum semantic representation ultimately belongs to quantum::ir.
 *
 * Distributed placement merely contributes placement intent.
 *
 * ============================================================================
 * HARDWARE / HDL INTEGRATION
 * ============================================================================
 *
 * Placement may refer symbolically to hardware/resource classes.
 *
 * It MUST NOT define:
 *
 *     wires;
 *     clocks;
 *     FPGA cells;
 *     ASIC cells;
 *     physical addresses;
 *     register files;
 *     device inventory.
 *
 * Those belong to HDL/hardware/resource layers.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * No actions.
 * No predicates.
 * No I/O.
 * No randomness.
 * No runtime calls.
 *
 * Parse results depend only on the token stream.
 *
 * ============================================================================
 */

parser grammar DistributedPlacement;

options {
    tokenVocab = ZamaniLexer;
}

import Names, Expressions;


/* ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Stable public rule consumed by distributed.g4.
 *
 * Canonical shape:
 *
 *     placement <name> { ... }
 *
 * The spelling/classification of the contextual word "placement" is validated
 * by semantic analysis where the repository's open-world identifier model
 * requires contextual keywords.
 */

distributedPlacementDeclaration
    : identifier
      identifier
      distributedPlacementHeader?
      LBRACE
      distributedPlacementMember*
      RBRACE
    ;


/* ============================================================================
 * 2. PLACEMENT HEADER
 * ============================================================================
 */

distributedPlacementHeader
    : distributedPlacementGenericParameters?
      distributedPlacementScopeClause?
      distributedPlacementSubjectClause?
      distributedPlacementAttributeList?
    ;


/* ============================================================================
 * 3. GENERIC PARAMETERS
 * ============================================================================
 *
 * Generic parameters represent semantic parameters.
 *
 * They are not hardware limits.
 *
 * Example:
 *
 *     placement ClusterPolicy<N> { ... }
 *
 * N may later be resolved from program semantics, compilation context,
 * deployment context, or available resources.
 */

distributedPlacementGenericParameters
    : LESS_THAN
      distributedPlacementGenericParameter
      (COMMA distributedPlacementGenericParameter)*
      GREATER_THAN
    ;

distributedPlacementGenericParameter
    : identifier
      distributedPlacementGenericBound?
      (ASSIGN expression)?
    ;

distributedPlacementGenericBound
    : COLON
      qualifiedName
    ;


/* ============================================================================
 * 4. PLACEMENT SCOPE
 * ============================================================================
 */

distributedPlacementScopeClause
    : identifier
      distributedPlacementScopeValue
    ;

distributedPlacementScopeValue
    : qualifiedName
    | expression
    ;


/* ============================================================================
 * 5. PLACEMENT SUBJECT
 * ============================================================================
 */

distributedPlacementSubjectClause
    : identifier
      distributedPlacementSubject
    ;

distributedPlacementSubject
    : qualifiedName
    | distributedPlacementSelector
    | expression
    ;


/* ============================================================================
 * 6. ATTRIBUTE LIST
 * ============================================================================
 */

distributedPlacementAttributeList
    : LPAREN
      distributedPlacementAttribute
      (COMMA distributedPlacementAttribute)*
      RPAREN
    ;

distributedPlacementAttribute
    : qualifiedName
      (
          ASSIGN expression
        | LPAREN expressionList? RPAREN
      )?
    ;


/* ============================================================================
 * 7. PLACEMENT BODY
 * ============================================================================
 */

distributedPlacementMember
    : distributedPlacementBinding
    | distributedPlacementGroup
    | distributedPlacementDomain
    | distributedPlacementRelation
    | distributedPlacementRequirement
    | distributedPlacementConstraint
    | distributedPlacementPreference
    | distributedPlacementHint
    | distributedPlacementPolicy
    | distributedPlacementAlternative
    | distributedPlacementReplica
    | distributedPlacementMigration
    | distributedPlacementElasticity
    | distributedPlacementCapability
    | distributedPlacementProperty
    ;


/* ============================================================================
 * 8. DIRECT PLACEMENT BINDING
 * ============================================================================
 *
 * Examples conceptually include:
 *
 *     place worker to region
 *     bind service to node_class
 *
 * The grammar does not interpret the physical meaning.
 */

distributedPlacementBinding
    : identifier
      distributedPlacementSubject
      distributedPlacementBindingOperator
      distributedPlacementLocation
      SEMICOLON
    ;

distributedPlacementBindingOperator
    : identifier
    ;


/* ============================================================================
 * 9. PLACEMENT LOCATION
 * ============================================================================
 *
 * Locations remain symbolic.
 *
 * A location can represent:
 *
 *     - logical region;
 *     - resource class;
 *     - capability domain;
 *     - topology domain;
 *     - deployment scope;
 *     - backend-provided location.
 */

distributedPlacementLocation
    : qualifiedName
    | distributedPlacementSelector
    | LPAREN expression RPAREN
    ;


/* ============================================================================
 * 10. SELECTORS
 * ============================================================================
 *
 * Selectors are expressions describing a set of possible placement targets.
 *
 * They do not enumerate physical infrastructure.
 */

distributedPlacementSelector
    : LBRACKET
      distributedPlacementSelectorExpression
      RBRACKET
    ;

distributedPlacementSelectorExpression
    : expression
    ;


/* ============================================================================
 * 11. GROUPS
 * ============================================================================
 */

distributedPlacementGroup
    : identifier
      identifier
      distributedPlacementGroupMode?
      distributedPlacementGroupSubjectList
      SEMICOLON?
    ;

distributedPlacementGroupMode
    : identifier
    ;

distributedPlacementGroupSubjectList
    : LBRACE
      distributedPlacementSubjectItem*
      RBRACE
    ;

distributedPlacementSubjectItem
    : distributedPlacementSubject
      SEMICOLON
    ;


/* ============================================================================
 * 12. DOMAINS
 * ============================================================================
 *
 * A domain is a symbolic placement domain.
 *
 * It is not a topology declaration.
 */

distributedPlacementDomain
    : identifier
      identifier
      distributedPlacementDomainAttributes?
      LBRACE
      distributedPlacementDomainMember*
      RBRACE
    ;

distributedPlacementDomainAttributes
    : LPAREN
      distributedPlacementAttributeList?
      RPAREN
    ;

distributedPlacementDomainMember
    : distributedPlacementLocation
      SEMICOLON
    | distributedPlacementRequirement
    | distributedPlacementConstraint
    | distributedPlacementPreference
    | distributedPlacementProperty
    ;


/* ============================================================================
 * 13. RELATIONSHIPS
 * ============================================================================
 */

distributedPlacementRelation
    : distributedPlacementAffinity
    | distributedPlacementAntiAffinity
    | distributedPlacementCoLocation
    | distributedPlacementSeparation
    | distributedPlacementLocality
    | distributedPlacementAdjacency
    | distributedPlacementDistance
    | distributedPlacementAvoidance
    ;


/* ----------------------------------------------------------------------------
 * Affinity
 * ----------------------------------------------------------------------------
 */

distributedPlacementAffinity
    : identifier
      distributedPlacementRelationSubject
      distributedPlacementRelationOperator
      distributedPlacementRelationSubject
      SEMICOLON
    ;


/* ----------------------------------------------------------------------------
 * Anti-affinity
 * ----------------------------------------------------------------------------
 */

distributedPlacementAntiAffinity
    : identifier
      distributedPlacementRelationSubject
      distributedPlacementRelationOperator
      distributedPlacementRelationSubject
      SEMICOLON
    ;


/* ----------------------------------------------------------------------------
 * Co-location
 * ----------------------------------------------------------------------------
 */

distributedPlacementCoLocation
    : identifier
      distributedPlacementRelationSubject
      distributedPlacementRelationOperator
      distributedPlacementRelationSubject
      SEMICOLON
    ;


/* ----------------------------------------------------------------------------
 * Separation
 * ----------------------------------------------------------------------------
 */

distributedPlacementSeparation
    : identifier
      distributedPlacementRelationSubject
      distributedPlacementRelationOperator
      distributedPlacementRelationSubject
      SEMICOLON
    ;


/* ----------------------------------------------------------------------------
 * Locality
 * ----------------------------------------------------------------------------
 */

distributedPlacementLocality
    : identifier
      distributedPlacementRelationSubject
      distributedPlacementRelationOperator
      distributedPlacementLocation
      SEMICOLON
    ;


/* ----------------------------------------------------------------------------
 * Adjacency
 * ----------------------------------------------------------------------------
 */

distributedPlacementAdjacency
    : identifier
      distributedPlacementRelationSubject
      distributedPlacementRelationOperator
      distributedPlacementRelationSubject
      SEMICOLON
    ;


/* ----------------------------------------------------------------------------
 * Distance
 * ----------------------------------------------------------------------------
 */

distributedPlacementDistance
    : identifier
      distributedPlacementRelationSubject
      distributedPlacementDistanceOperator
      expression
      SEMICOLON
    ;

distributedPlacementDistanceOperator
    : LESS_THAN
    | LESS_EQUAL
    | GREATER_THAN
    | GREATER_EQUAL
    | ASSIGN
    ;


/* ----------------------------------------------------------------------------
 * Avoidance
 * ----------------------------------------------------------------------------
 */

distributedPlacementAvoidance
    : identifier
      distributedPlacementRelationSubject
      distributedPlacementRelationOperator
      distributedPlacementRelationSubject
      SEMICOLON
    ;

distributedPlacementRelationSubject
    : distributedPlacementSubject
    | distributedPlacementLocation
    ;

distributedPlacementRelationOperator
    : identifier
    ;


/* ============================================================================
 * 14. REQUIREMENTS
 * ============================================================================
 *
 * Requirements are mandatory semantic conditions.
 */

distributedPlacementRequirement
    : identifier
      distributedPlacementRequirementBody
      SEMICOLON
    ;

distributedPlacementRequirementBody
    : distributedPlacementResourceRequirement
    | distributedPlacementCapabilityRequirement
    | distributedPlacementLocationRequirement
    | distributedPlacementTopologyRequirement
    | distributedPlacementExpressionRequirement
    ;

distributedPlacementResourceRequirement
    : identifier
      distributedPlacementResourceSelector
    ;

distributedPlacementCapabilityRequirement
    : identifier
      distributedPlacementCapabilitySelector
    ;

distributedPlacementLocationRequirement
    : distributedPlacementSubject
      distributedPlacementRequirementOperator
      distributedPlacementLocation
    ;

distributedPlacementTopologyRequirement
    : identifier
      distributedPlacementTopologySelector
    ;

distributedPlacementExpressionRequirement
    : expression
    ;

distributedPlacementRequirementOperator
    : identifier
    ;

distributedPlacementResourceSelector
    : qualifiedName
    | distributedPlacementSelector
    | expression
    ;

distributedPlacementCapabilitySelector
    : qualifiedName
    | distributedPlacementSelector
    | expression
    ;

distributedPlacementTopologySelector
    : qualifiedName
    | distributedPlacementSelector
    | expression
    ;


/* ============================================================================
 * 15. CONSTRAINTS
 * ============================================================================
 *
 * Constraints restrict legal realizations.
 *
 * They do not themselves choose a realization.
 */

distributedPlacementConstraint
    : identifier
      distributedPlacementConstraintBody
      SEMICOLON
    ;

distributedPlacementConstraintBody
    : distributedPlacementConstraintExpression
    | distributedPlacementConstraintRelation
    | distributedPlacementConstraintScope
    ;

distributedPlacementConstraintExpression
    : expression
    ;

distributedPlacementConstraintRelation
    : distributedPlacementRelationSubject
      distributedPlacementConstraintOperator
      distributedPlacementRelationSubject
    ;

distributedPlacementConstraintScope
    : distributedPlacementSubject
      distributedPlacementConstraintOperator
      distributedPlacementLocation
    ;

distributedPlacementConstraintOperator
    : ASSIGN
    | NOT_EQUAL
    | LESS_THAN
    | LESS_EQUAL
    | GREATER_THAN
    | GREATER_EQUAL
    | identifier
    ;


/* ============================================================================
 * 16. PREFERENCES
 * ============================================================================
 *
 * Preferences are non-mandatory.
 *
 * A compiler may satisfy them when possible without treating them as semantic
 * requirements.
 */

distributedPlacementPreference
    : identifier
      distributedPlacementPreferenceBody
      SEMICOLON
    ;

distributedPlacementPreferenceBody
    : distributedPlacementPreferenceExpression
    | distributedPlacementPreferenceRelation
    ;

distributedPlacementPreferenceExpression
    : expression
    ;

distributedPlacementPreferenceRelation
    : distributedPlacementRelationSubject
      distributedPlacementPreferenceOperator
      distributedPlacementLocation
    ;

distributedPlacementPreferenceOperator
    : ASSIGN
    | identifier
    ;


/* ============================================================================
 * 17. HINTS
 * ============================================================================
 *
 * Hints are advisory.
 *
 * A hint MUST NOT become an implicit semantic requirement.
 */

distributedPlacementHint
    : identifier
      distributedPlacementHintBody
      SEMICOLON
    ;

distributedPlacementHintBody
    : expression
    | distributedPlacementHintRelation
    ;

distributedPlacementHintRelation
    : distributedPlacementRelationSubject
      distributedPlacementHintOperator
      distributedPlacementLocation
    ;

distributedPlacementHintOperator
    : ASSIGN
    | identifier
    ;


/* ============================================================================
 * 18. POLICIES
 * ============================================================================
 *
 * Policies describe source-level placement intent.
 *
 * They do not implement placement algorithms.
 */

distributedPlacementPolicy
    : identifier
      identifier
      distributedPlacementPolicyBody
      SEMICOLON?
    ;

distributedPlacementPolicyBody
    : expression
    | LBRACE
      distributedPlacementPolicyItem*
      RBRACE
    ;

distributedPlacementPolicyItem
    : distributedPlacementRequirement
    | distributedPlacementConstraint
    | distributedPlacementPreference
    | distributedPlacementHint
    | distributedPlacementProperty
    ;


/* ============================================================================
 * 19. ALTERNATIVES
 * ============================================================================
 *
 * An alternative represents a set of semantically acceptable placement
 * choices.
 *
 * The grammar does not choose which alternative wins.
 */

distributedPlacementAlternative
    : identifier
      LBRACE
      distributedPlacementAlternativeItem+
      RBRACE
    ;

distributedPlacementAlternativeItem
    : distributedPlacementLocation
      distributedPlacementAlternativeWeight?
      SEMICOLON
    ;

distributedPlacementAlternativeWeight
    : identifier
      expression
    ;


/* ============================================================================
 * 20. REPLICA PLACEMENT
 * ============================================================================
 *
 * This expresses WHERE replicas may/should be placed.
 *
 * It does not implement replication.
 *
 * Replica count is an expression rather than a grammar-level constant.
 */

distributedPlacementReplica
    : identifier
      distributedPlacementReplicaBody
      SEMICOLON
    ;

distributedPlacementReplicaBody
    : distributedPlacementReplicaCount?
      distributedPlacementReplicaLocation?
      distributedPlacementReplicaPolicy*
    ;

distributedPlacementReplicaCount
    : identifier
      expression
    ;

distributedPlacementReplicaLocation
    : identifier
      distributedPlacementLocation
    ;

distributedPlacementReplicaPolicy
    : identifier
      expression?
    ;


/* ============================================================================
 * 21. MIGRATION
 * ============================================================================
 *
 * Migration is intent only.
 *
 * Actual migration belongs to runtime/deployment infrastructure.
 */

distributedPlacementMigration
    : identifier
      distributedPlacementMigrationBody
      SEMICOLON
    ;

distributedPlacementMigrationBody
    : distributedPlacementMigrationMode?
      distributedPlacementMigrationSource?
      distributedPlacementMigrationDestination?
      distributedPlacementMigrationCondition?
    ;

distributedPlacementMigrationMode
    : identifier
    ;

distributedPlacementMigrationSource
    : identifier
      distributedPlacementLocation
    ;

distributedPlacementMigrationDestination
    : identifier
      distributedPlacementLocation
    ;

distributedPlacementMigrationCondition
    : identifier
      expression
    ;


/* ============================================================================
 * 22. ELASTICITY
 * ============================================================================
 *
 * Elasticity expresses whether placement may adapt to resource availability.
 *
 * No finite machine capacity is encoded here.
 */

distributedPlacementElasticity
    : identifier
      distributedPlacementElasticityBody
      SEMICOLON
    ;

distributedPlacementElasticityBody
    : distributedPlacementElasticityMode
      distributedPlacementElasticityCondition*
    ;

distributedPlacementElasticityMode
    : identifier
    ;

distributedPlacementElasticityCondition
    : identifier
      expression
    ;


/* ============================================================================
 * 23. CAPABILITIES
 * ============================================================================
 *
 * Capabilities describe properties that placement may require or prefer.
 *
 * Capability realization belongs to the capability/resource system.
 */

distributedPlacementCapability
    : identifier
      distributedPlacementCapabilityBody
      SEMICOLON
    ;

distributedPlacementCapabilityBody
    : distributedPlacementCapabilityReference
    | distributedPlacementCapabilityAssignment
    ;

distributedPlacementCapabilityReference
    : qualifiedName
    ;

distributedPlacementCapabilityAssignment
    : qualifiedName
      ASSIGN
      expression
    ;


/* ============================================================================
 * 24. PROPERTIES
 * ============================================================================
 *
 * Generic placement properties keep this grammar extensible.
 */

distributedPlacementProperty
    : qualifiedName
      (
          ASSIGN expression
        | LPAREN expressionList? RPAREN
      )
      SEMICOLON
    ;
/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/resources/placement.g4
 *
 * Grammar:
 *     ANTLR4 parser grammar
 *
 * Grammar identity:
 *     ResourcePlacement
 *
 * Baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     Grammar-only.
 *     No embedded Rust.
 *     No semantic predicates.
 *     No parser actions.
 *     No filesystem access.
 *     No network access.
 *     No hardware access.
 *     No runtime execution.
 *     No unsafe Rust requirement.
 *
 * ============================================================================
 * STATUS
 * ============================================================================
 *
 * PRODUCTION RESOURCE-PLACEMENT INTENT GRAMMAR
 *
 * This file is the canonical reusable grammar component for RESOURCE-LAYER
 * PLACEMENT INTENT.
 *
 * It does NOT own:
 *
 *     physical placement;
 *     hardware placement algorithms;
 *     distributed placement algorithms;
 *     quantum physical-qubit mapping;
 *     routing;
 *     scheduling;
 *     allocation;
 *     hardware discovery;
 *     topology construction;
 *     calibration;
 *     QEC;
 *     ZQN;
 *     HAL;
 *     runtime placement execution.
 *
 * Those concerns belong downstream.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          v
 *     Resources
 *          |
 *          v
 *     ResourcePlacement
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic resource model
 *          |
 *     +----+---------+----------+-------------+
 *     |              |          |             |
 *     v              v          v             v
 * requirements   capabilities constraints preferences
 *     |              |          |             |
 *     +--------------+----------+-------------+
 *                    |
 *                    v
 *             canonical semantic model
 *                    |
 *       +------------+-------------+
 *       |            |             |
 *       v            v             v
 *   classical    quantum::ir   HDL/hardware
 *       |            |             |
 *       +------------+-------------+
 *                    |
 *                    v
 *             optimization/lowering
 *                    |
 *          +---------+---------+
 *          |         |         |
 *          v         v         v
 *       placement routing scheduling
 *          |         |         |
 *          +---------+---------+
 *                    |
 *                   ZQN
 *                    |
 *                   HAL
 *                    |
 *             target realization
 *
 * ============================================================================
 * CORE PRINCIPLE
 * ============================================================================
 *
 * Placement intent answers:
 *
 *     WHERE SHOULD A SEMANTIC RESOURCE PREFERABLY/LEGALLY EXIST?
 *
 * It does NOT answer:
 *
 *     WHICH PHYSICAL DEVICE SHOULD THE COMPILER SELECT?
 *
 * Examples of portable intent:
 *
 *     placement::locality = execution_region;
 *
 *     placement::affinity = service_a, service_b;
 *
 *     placement::anti_affinity = replica_a, replica_b;
 *
 *     placement::co_location = stage_a, stage_b;
 *
 *     placement::separation = tenant_a, tenant_b;
 *
 *     placement::scope = execution_region;
 *
 *     placement::elastic = workload_size;
 *
 *     placement::migration = migration_policy;
 *
 * The exact semantic interpretation belongs to semantic analysis.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * This grammar is part of:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Placement therefore describes PORTABLE INTENT rather than today's physical
 * machine layout.
 *
 * A program may express:
 *
 *     affinity;
 *     anti-affinity;
 *     co-location;
 *     separation;
 *     locality;
 *     scope;
 *     elasticity;
 *     mobility;
 *     replication intent;
 *     placement requirements;
 *     placement constraints;
 *     placement preferences;
 *     placement hints;
 *     symbolic target/resource relationships.
 *
 * The compiler, resource system, topology system, routing system, scheduler,
 * hardware abstraction layer, deployment system, and runtime determine the
 * actual realization.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This file MUST NOT define or imply:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *     MAX_PLACEMENTS
 *     MAX_REGIONS
 *     MAX_GROUPS
 *
 * It also MUST NOT encode:
 *
 *     CPU 0
 *     GPU 0
 *     QPU 0
 *     physical qubit 0
 *     node 0
 *     memory bank 0
 *     rack 0
 *     fixed topology dimensions
 *     fixed vendor identifiers
 *     fixed device models
 *
 * A program MAY contain numeric values as semantic program data.
 *
 * For example:
 *
 *     placement::replicas = replica_count;
 *
 * or:
 *
 *     placement::distance = desired_distance;
 *
 * The grammar must never reinterpret those values as universal hardware
 * limits.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     resourcePlacementSpecification
 *     resourcePlacementClause
 *     resourcePlacementAssignment
 *     resourcePlacementRelation
 *     resourcePlacementRelationValue
 *     resourcePlacementScope
 *     resourcePlacementTarget
 *     resourcePlacementRequirement
 *     resourcePlacementConstraint
 *     resourcePlacementPreference
 *     resourcePlacementHint
 *     resourcePlacementPolicy
 *     resourcePlacementReplication
 *     resourcePlacementMobility
 *     resourcePlacementElasticity
 *     resourcePlacementGroup
 *     resourcePlacementProperty
 *     resourcePlacementValue
 *
 * THIS FILE DOES NOT OWN:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     resourceExpression
 *     resourceExpressionList
 *     literals
 *     arithmetic
 *     logical operators
 *     comparison precedence
 *     resource requirements generally
 *     resource constraints generally
 *     resource preferences generally
 *     resource capabilities generally
 *     hardware topology
 *     hardware target declarations
 *     distributed topology
 *     routing
 *     scheduling
 *     allocation
 *     QEC
 *     ZQN
 *     HAL
 *     runtime execution
 *     quantum::ir
 *     classical IR
 *     HDL/hardware IR
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * ResourceExpressions owns:
 *
 *     resourceExpression
 *     resourceExpressionList
 *
 * Names owns:
 *
 *     identifier
 *     qualifiedName
 *
 * This grammar therefore MUST NOT create another expression language or
 * identifier implementation.
 *
 * ============================================================================
 */

parser grammar ResourcePlacement;

options {
    tokenVocab = ZamaniLexer;
}

import ResourceExpressions, Names;


/*
 * ============================================================================
 * 1. PUBLIC PLACEMENT SPECIFICATION
 * ============================================================================
 *
 * This is the primary reusable entry point.
 *
 * A resource-domain parent may introduce it with its own contextual syntax.
 *
 * Example conceptual composition:
 *
 *     placement {
 *         placement::scope = execution_region;
 *         placement::affinity = stage_a, stage_b;
 *     }
 *
 * The concrete placement introducer is intentionally NOT owned here.
 *
 * This avoids inventing a new PLACEMENT lexer token merely for this grammar.
 *
 * ============================================================================
 */

resourcePlacementSpecification
    : LBRACE
      resourcePlacementClause*
      RBRACE
    ;


/*
 * ============================================================================
 * 2. PLACEMENT CLAUSE
 * ============================================================================
 *
 * Placement is deliberately open-world.
 *
 * Standard semantic categories are represented by property namespaces such
 * as:
 *
 *     placement::affinity
 *     placement::anti_affinity
 *     placement::co_location
 *     placement::separation
 *     placement::locality
 *     placement::scope
 *     placement::target
 *     placement::replicas
 *     placement::migration
 *     placement::elasticity
 *
 * New placement dimensions do not require a new lexer keyword.
 *
 * ============================================================================
 */

resourcePlacementClause
    : resourcePlacementRelation
    | resourcePlacementScope
    | resourcePlacementTarget
    | resourcePlacementRequirement
    | resourcePlacementConstraint
    | resourcePlacementPreference
    | resourcePlacementHint
    | resourcePlacementPolicy
    | resourcePlacementReplication
    | resourcePlacementMobility
    | resourcePlacementElasticity
    | resourcePlacementGroup
    | resourcePlacementProperty
    ;


/*
 * ============================================================================
 * 3. GENERIC PLACEMENT ASSIGNMENT
 * ============================================================================
 *
 * This is the principal open-world extension point.
 *
 * It allows future placement properties without requiring a lexer change.
 *
 * Examples:
 *
 *     placement::locality = execution_region;
 *
 *     placement::scope = module_scope;
 *
 *     placement::policy = placement_policy;
 *
 *     vendor::placement::metric = desired_value;
 *
 * Semantic analysis determines whether the property is standard, experimental,
 * dialect-specific, vendor-specific, deprecated, or unknown.
 *
 * ============================================================================
 */

resourcePlacementAssignment
    : qualifiedName
      ASSIGN
      resourcePlacementValue
      SEMICOLON
    ;


/*
 * ============================================================================
 * 4. RELATIONSHIPS
 * ============================================================================
 *
 * Relationships describe semantic relationships between placement subjects.
 *
 * The grammar does not know whether the subjects are:
 *
 *     tasks;
 *     processes;
 *     services;
 *     resources;
 *     logical quantum objects;
 *     memory objects;
 *     accelerator workloads;
 *     HDL modules;
 *     data partitions;
 *     AI operators;
 *     future resource kinds.
 *
 * Their meaning is determined semantically.
 *
 * ============================================================================
 */

resourcePlacementRelation
    : resourcePlacementRelationName
      ASSIGN
      resourcePlacementRelationValue
      SEMICOLON
    ;


resourcePlacementRelationName
    : qualifiedName
    ;


resourcePlacementRelationValue
    : resourceExpressionList
    ;


/*
 * ============================================================================
 * 5. SCOPE
 * ============================================================================
 *
 * Scope describes an abstract placement domain.
 *
 * It MUST NOT identify a physical machine location directly.
 *
 * Examples:
 *
 *     placement::scope = module;
 *
 *     placement::scope = execution_region;
 *
 *     placement::scope = locality_domain;
 *
 *     placement::scope = preferred_region;
 *
 * ============================================================================
 */

resourcePlacementScope
    : resourcePlacementScopeName
      ASSIGN
      resourcePlacementValue
      SEMICOLON
    ;


resourcePlacementScopeName
    : qualifiedName
    ;


/*
 * ============================================================================
 * 6. TARGET INTENT
 * ============================================================================
 *
 * A target here is a symbolic target category or semantic target reference.
 *
 * It does NOT select a concrete device.
 *
 * Examples:
 *
 *     placement::target = quantum;
 *
 *     placement::target = accelerator;
 *
 *     placement::target = distributed;
 *
 *     placement::target = target_class;
 *
 * ============================================================================
 */

resourcePlacementTarget
    : resourcePlacementTargetName
      ASSIGN
      resourcePlacementValue
      SEMICOLON
    ;


resourcePlacementTargetName
    : qualifiedName
    ;


/*
 * ============================================================================
 * 7. REQUIREMENT
 * ============================================================================
 *
 * Placement requirements are mandatory placement-related semantic conditions.
 *
 * They are intentionally represented as named properties here rather than
 * duplicating the universal resource-requirement grammar.
 *
 * The parent resource system remains responsible for the general `requires`
 * statement.
 *
 * Examples:
 *
 *     placement::requirement = locality_requirement;
 *
 *     placement::required_scope = execution_region;
 *
 *     placement::required_affinity = affinity_contract;
 *
 * Semantic analysis MUST preserve:
 *
 *     requirement != constraint != preference != hint
 *
 * ============================================================================
 */

resourcePlacementRequirement
    : resourcePlacementRequirementName
      ASSIGN
      resourcePlacementValue
      SEMICOLON
    ;


resourcePlacementRequirementName
    : qualifiedName
    ;


/*
 * ============================================================================
 * 8. CONSTRAINT
 * ============================================================================
 *
 * Placement constraints restrict legal realizations.
 *
 * They do not choose the realization.
 *
 * Examples:
 *
 *     placement::constraint = locality_constraint;
 *
 *     placement::avoid = forbidden_domain;
 *
 *     placement::distance = maximum_distance;
 *
 * ============================================================================
 */

resourcePlacementConstraint
    : resourcePlacementConstraintName
      ASSIGN
      resourcePlacementValue
      SEMICOLON
    ;


resourcePlacementConstraintName
    : qualifiedName
    ;


/*
 * ============================================================================
 * 9. PREFERENCE
 * ============================================================================
 *
 * Placement preferences are advisory.
 *
 * They MUST NOT silently become requirements.
 *
 * Examples:
 *
 *     placement::preference = preferred_region;
 *
 *     placement::prefer = low_latency_region;
 *
 *     placement::priority = placement_priority;
 *
 * ============================================================================
 */

resourcePlacementPreference
    : resourcePlacementPreferenceName
      ASSIGN
      resourcePlacementValue
      SEMICOLON
    ;


resourcePlacementPreferenceName
    : qualifiedName
    ;


/*
 * ============================================================================
 * 10. HINT
 * ============================================================================
 *
 * Hints are advisory information that may be ignored by downstream systems.
 *
 * They are not requirements and are not constraints.
 *
 * ============================================================================
 */

resourcePlacementHint
    : resourcePlacementHintName
      ASSIGN
      resourcePlacementValue
      SEMICOLON
    ;


resourcePlacementHintName
    : qualifiedName
    ;


/*
 * ============================================================================
 * 11. POLICY
 * ============================================================================
 *
 * A policy is a symbolic placement-policy reference or expression.
 *
 * This grammar does not implement the policy algorithm.
 *
 * ============================================================================
 */

resourcePlacementPolicy
    : resourcePlacementPolicyName
      ASSIGN
      resourcePlacementValue
      SEMICOLON
    ;


resourcePlacementPolicyName
    : qualifiedName
    ;


/*
 * ============================================================================
 * 12. REPLICATION INTENT
 * ============================================================================
 *
 * Replication describes placement intent for multiple semantic instances.
 *
 * The replication quantity is an expression.
 *
 * No fixed replication count is encoded.
 *
 * Examples:
 *
 *     placement::replicas = replica_count;
 *
 *     placement::replica_scope = independent_failure_domain;
 *
 * ============================================================================
 */

resourcePlacementReplication
    : resourcePlacementReplicationName
      ASSIGN
      resourcePlacementValue
      SEMICOLON
    ;


resourcePlacementReplicationName
    : qualifiedName
    ;


/*
 * ============================================================================
 * 13. MOBILITY / MIGRATION
 * ============================================================================
 *
 * Mobility describes whether placement may change over time.
 *
 * This grammar does not perform migration.
 *
 * Examples:
 *
 *     placement::migration = migration_policy;
 *
 *     placement::mobility = dynamic;
 *
 *     placement::source_scope = source_domain;
 *
 *     placement::destination_scope = destination_domain;
 *
 * ============================================================================
 */

resourcePlacementMobility
    : resourcePlacementMobilityName
      ASSIGN
      resourcePlacementValue
      SEMICOLON
    ;


resourcePlacementMobilityName
    : qualifiedName
    ;


/*
 * ============================================================================
 * 14. ELASTICITY
 * ============================================================================
 *
 * Elasticity allows the placement realization to adapt to available
 * resources.
 *
 * The expression is semantic data.
 *
 * No fixed machine capacity is encoded.
 *
 * Examples:
 *
 *     placement::elasticity = workload_size;
 *
 *     placement::scale_with = available_resources;
 *
 *     placement::elastic = true;
 *
 * ============================================================================
 */

resourcePlacementElasticity
    : resourcePlacementElasticityName
      ASSIGN
      resourcePlacementValue
      SEMICOLON
    ;


resourcePlacementElasticityName
    : qualifiedName
    ;


/*
 * ============================================================================
 * 15. PLACEMENT GROUP
 * ============================================================================
 *
 * Groups are semantic collections.
 *
 * They are NOT:
 *
 *     hardware clusters;
 *     CPU core groups;
 *     GPU groups;
 *     QPU groups;
 *     physical node groups;
 *     topology declarations.
 *
 * Group membership remains symbolic.
 *
 * ============================================================================
 */

resourcePlacementGroup
    : resourcePlacementGroupName
      ASSIGN
      resourcePlacementGroupValue
      SEMICOLON
    ;


resourcePlacementGroupName
    : qualifiedName
    ;


resourcePlacementGroupValue
    : resourceExpressionList
    ;


/*
 * ============================================================================
 * 16. GENERIC PROPERTY
 * ============================================================================
 *
 * Generic properties provide the long-term extensibility boundary.
 *
 * Examples:
 *
 *     placement::latency = latency_budget;
 *
 *     placement::energy = energy_budget;
 *
 *     placement::reliability = reliability_goal;
 *
 *     placement::portability = portability_goal;
 *
 *     placement::scalability = scalability_goal;
 *
 *     quantum::placement::fidelity = desired_fidelity;
 *
 *     accelerator::placement::throughput = desired_throughput;
 *
 * No property list is closed in this grammar.
 *
 * ============================================================================
 */

resourcePlacementProperty
    : resourcePlacementPropertyName
      ASSIGN
      resourcePlacementValue
      SEMICOLON
    ;


resourcePlacementPropertyName
    : qualifiedName
    ;


/*
 * ============================================================================
 * 17. PLACEMENT VALUE
 * ============================================================================
 *
 * All placement values use the canonical resource-expression system.
 *
 * This gives placement access to the existing expression architecture without
 * creating a placement-specific expression language.
 *
 * Placement values may therefore eventually represent:
 *
 *     identifiers;
 *     qualified names;
 *     literals;
 *     arithmetic;
 *     comparisons;
 *     logical expressions;
 *     function-like resource expressions;
 *     indexing;
 *     member access;
 *     ranges;
 *     symbolic resource values;
 *     computed resource quantities.
 *
 * ============================================================================
 */

resourcePlacementValue
    : resourceExpression
    | resourceExpressionList
    ;


/*
 * ============================================================================
 * 18. OPTIONAL PLACEMENT SPECIFICATION
 * ============================================================================
 */

optionalResourcePlacementSpecification
    : resourcePlacementSpecification?
    ;


/*
 * ============================================================================
 * 19. PLACEMENT CLAUSE LIST
 * ============================================================================
 *
 * Unbounded at the language level.
 *
 * ============================================================================
 */

resourcePlacementClauseList
    : resourcePlacementClause*
    ;


/*
 * ============================================================================
 * 20. SEMANTIC CATEGORY CONTRACT
 * ============================================================================
 *
 * Semantic analysis should classify placement properties into categories such
 * as:
 *
 *     Binding
 *     Relationship
 *     Requirement
 *     Constraint
 *     Preference
 *     Hint
 *     Scope
 *     Policy
 *     Replication
 *     Mobility
 *     Elasticity
 *     Property
 *
 * The parser intentionally does not perform this classification based on
 * arbitrary property spelling.
 *
 * This keeps the grammar open-world.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 21. REQUIREMENT / CONSTRAINT / PREFERENCE / HINT SEPARATION
 * ============================================================================
 *
 * The canonical semantic distinctions remain:
 *
 *     requirement
 *         MUST be satisfied.
 *
 *     constraint
 *         restricts valid realizations.
 *
 *     preference
 *         SHOULD be satisfied where practical.
 *
 *     hint
 *         MAY guide implementation and MAY be ignored.
 *
 * The placement grammar must never silently convert one category into
 * another.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 22. PHYSICAL PLACEMENT SEPARATION
 * ============================================================================
 *
 * This grammar MUST NOT parse or establish semantic ownership for:
 *
 *     physical CPU identifiers;
 *     physical GPU identifiers;
 *     physical FPGA locations;
 *     physical ASIC cells;
 *     physical QPU identifiers;
 *     physical qubit identifiers;
 *     memory-bank identifiers;
 *     physical network-node identifiers;
 *     physical rack identifiers;
 *     physical addresses.
 *
 * If a downstream implementation needs such information, it is represented
 * after semantic analysis in target-specific data structures.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 23. TOPOLOGY SEPARATION
 * ============================================================================
 *
 * `hardware/topology.g4` remains the topology syntax owner.
 *
 * This grammar may reference topology-related semantic names through
 * resourceExpression.
 *
 * It must NOT redefine:
 *
 *     node
 *     edge
 *     link
 *     topology
 *     connectivity
 *     routing path
 *
 * Topology discovery and validation remain downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 24. ROUTING SEPARATION
 * ============================================================================
 *
 * Placement says WHERE a semantic object should preferably/legally exist.
 *
 * Routing says HOW communication or movement is physically realized.
 *
 * Therefore this grammar must not contain:
 *
 *     pathfinding;
 *     route selection;
 *     SWAP insertion;
 *     connectivity traversal;
 *     network path algorithms;
 *     physical qubit movement.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 25. SCHEDULING SEPARATION
 * ============================================================================
 *
 * Placement is spatial/logical intent.
 *
 * Scheduling is temporal/resource-order realization.
 *
 * Scheduling therefore consumes semantic placement information but is not
 * implemented by this grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 26. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum placement may refer semantically to:
 *
 *     logical qubits;
 *     logical registers;
 *     quantum workloads;
 *     logical circuits;
 *     execution regions;
 *     quantum memory;
 *     quantum accelerators;
 *     capability domains.
 *
 * The grammar MUST NOT define:
 *
 *     QubitId;
 *     PhysicalQubitId;
 *     quantum topology;
 *     coupling maps;
 *     SWAP operations;
 *     QEC placement algorithms.
 *
 * The canonical quantum boundary remains:
 *
 *     quantum::ir
 *
 * Any eventual physical mapping is downstream of the semantic model.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 27. CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical placement may describe:
 *
 *     locality;
 *     affinity;
 *     anti-affinity;
 *     memory locality;
 *     accelerator preference;
 *     execution domain;
 *     workload grouping;
 *     elasticity.
 *
 * It MUST NOT impose:
 *
 *     CPU count;
 *     core count;
 *     register width;
 *     cache size;
 *     physical CPU identity.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 28. GPU / FPGA / ACCELERATOR INTEGRATION
 * ============================================================================
 *
 * Placement may refer to abstract resource classes and capabilities:
 *
 *     gpu::compute;
 *     fpga::compute;
 *     accelerator::tensor;
 *     accelerator::quantum;
 *
 * It must not select:
 *
 *     GPU 0;
 *     FPGA 1;
 *     vendor_model;
 *     physical accelerator address.
 *
 * Target realization belongs to hardware/capability analysis and lowering.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 29. DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Distributed placement grammars already exist under:
 *
 *     grammar/distributed/placement.g4
 *
 * That grammar owns distributed-domain placement structures.
 *
 * This resource grammar MUST NOT copy its distributed topology, replication,
 * migration, or node-placement model.
 *
 * Instead:
 *
 *     resource placement intent
 *             |
 *             v
 *     distributed semantic analysis
 *             |
 *             v
 *     distributed placement realization
 *
 * This keeps the resource layer and distributed layer composable.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 30. HARDWARE INTEGRATION
 * ============================================================================
 *
 * Existing:
 *
 *     grammar/hardware/placement.g4
 *
 * remains responsible for hardware-domain placement intent.
 *
 * This file is the RESOURCE-domain counterpart.
 *
 * The two MUST NOT become competing definitions of the same semantic layer.
 *
 * Hardware placement may consume resource-placement intent after semantic
 * lowering.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 31. EXECUTION INTEGRATION
 * ============================================================================
 *
 * Existing:
 *
 *     grammar/execution/placement.g4
 *
 * remains responsible for execution-layer placement intent.
 *
 * It should consume the canonical semantic representation produced from this
 * resource layer rather than reparsing or redefining resource placement.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 32. RESOURCE INTEGRATION
 * ============================================================================
 *
 * `grammar/resources/resources.g4` remains the universal resource composition
 * owner.
 *
 * It should import this grammar when a concrete resource-level placement
 * entry point is introduced.
 *
 * Recommended ownership:
 *
 *     resources.g4
 *         owns concrete resource statement/dispatch.
 *
 *     placement.g4
 *         owns reusable placement payload.
 *
 * The concrete placement introducer MUST be selected by the canonical
 * resources composition grammar rather than inventing a duplicate keyword
 * here.
 *
 * Because the current repository does not establish a canonical PLACEMENT
 * lexer token, this file intentionally does NOT invent one.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 33. PREFERENCES INTEGRATION
 * ============================================================================
 *
 * `grammar/resources/preferences.g4` remains the owner of reusable resource
 * preference payload syntax.
 *
 * Placement preferences may ultimately lower into the same semantic preference
 * model.
 *
 * This file does not duplicate:
 *
 *     resourcePreferenceSpecification
 *     resourcePreferenceClause
 *     resourcePreferenceProperty
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 34. REQUIREMENTS INTEGRATION
 * ============================================================================
 *
 * `grammar/resources/requirements.g4` remains the canonical owner of general
 * resource requirement statements.
 *
 * Placement-specific requirements represented here as properties must be
 * lowered into the same semantic requirement category.
 *
 * This prevents:
 *
 *     placement requirement
 *
 * from becoming a second incompatible requirement type.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 35. CONSTRAINTS INTEGRATION
 * ============================================================================
 *
 * `grammar/resources/constraints.g4` remains the general resource-constraint
 * owner.
 *
 * Placement-specific constraint properties must lower into the same semantic
 * constraint representation.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 36. CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Capabilities remain open-world semantic entities.
 *
 * Placement may refer to them through:
 *
 *     resourceExpression
 *
 * or qualified resource properties.
 *
 * This grammar does not enumerate:
 *
 *     CPU capabilities;
 *     GPU capabilities;
 *     FPGA capabilities;
 *     QPU capabilities;
 *     vendor capabilities;
 *     future accelerator capabilities.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 37. AST CONTRACT
 * ============================================================================
 *
 * The parser output must map into the domain-neutral frontend AST.
 *
 * Conceptually:
 *
 *     resourcePlacementSpecification
 *         -> PlacementSpecification
 *
 *     resourcePlacementRelation
 *         -> PlacementRelation
 *
 *     resourcePlacementProperty
 *         -> PlacementProperty
 *
 *     resourcePlacementValue
 *         -> canonical expression/resource-expression node
 *
 * The AST must preserve:
 *
 *     source span;
 *     property name;
 *     ordered values;
 *     nesting;
 *     source attributes;
 *     syntactic category where required.
 *
 * The grammar MUST NOT depend on concrete AST Rust types.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 38. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis resolves:
 *
 *     placement subject;
 *     resource identity;
 *     resource kind;
 *     scope;
 *     locality;
 *     affinity;
 *     anti-affinity;
 *     co-location;
 *     separation;
 *     mobility;
 *     elasticity;
 *     capability references;
 *     requirement severity;
 *     constraint severity;
 *     preference priority;
 *     hint advisory status.
 *
 * It also verifies that placement intent is compatible with:
 *
 *     resource requirements;
 *     resource constraints;
 *     capabilities;
 *     target policy;
 *     portability requirements.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 39. IR CONTRACT
 * ============================================================================
 *
 * This grammar does NOT define an IR.
 *
 * Placement information must first become part of the canonical semantic
 * resource model.
 *
 * From there it may be consumed by:
 *
 *     classical IR;
 *     quantum::ir;
 *     HDL/hardware IR;
 *     distributed execution IR;
 *     compiler lowering;
 *     optimization;
 *     routing;
 *     scheduling;
 *     deployment.
 *
 * No second placement IR is created here.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 40. QUANTUM IR SAFETY
 * ============================================================================
 *
 * Placement syntax MUST NOT create a second quantum IR.
 *
 * The established path remains:
 *
 *     quantum source
 *         |
 *         v
 *     frontend AST
 *         |
 *         v
 *     semantic quantum model
 *         |
 *         v
 *     quantum::ir
 *         |
 *         v
 *     routing / scheduling / QEC / ZQN / HAL
 *
 * Placement is metadata/intent consumed by those downstream phases.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 41. SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * Every placement construct must retain enough parser context for downstream
 * diagnostics to identify:
 *
 *     placement specification;
 *     placement property;
 *     property name;
 *     value;
 *     relationship;
 *     nested placement group.
 *
 * The Rust frontend must therefore preserve source spans when converting the
 * parse tree into the AST.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 42. DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Syntax diagnostics belong to the parser.
 *
 * Semantic diagnostics include:
 *
 *     unknown placement property;
 *     invalid placement subject;
 *     incompatible placement scope;
 *     unsatisfied placement requirement;
 *     conflicting placement constraints;
 *     invalid capability reference;
 *     unsupported target;
 *     infeasible placement;
 *     unavailable resource;
 *     incompatible portability policy.
 *
 * Resource availability errors MUST NOT be reported as parser errors.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 43. DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no semantic predicates;
 *     no parser actions;
 *     no filesystem access;
 *     no network access;
 *     no hardware access;
 *     no runtime callbacks;
 *     no randomness.
 *
 * Identical token streams therefore produce identical parser behavior.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 44. SECURITY CONTRACT
 * ============================================================================
 *
 * Parsing placement syntax MUST NOT:
 *
 *     enumerate hardware;
 *     inspect devices;
 *     open files;
 *     open network connections;
 *     access physical addresses;
 *     allocate resources;
 *     reserve devices;
 *     execute commands;
 *     load drivers;
 *     bypass capability checks;
 *     mutate runtime state.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 45. SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar contains no finite limits on:
 *
 *     placement clauses;
 *     relationship values;
 *     resource expressions;
 *     groups;
 *     nesting;
 *     placement properties;
 *     placement specifications.
 *
 * Repetition uses:
 *
 *     *
 *
 * or recursive semantic structures.
 *
 * Therefore the language architecture scales from:
 *
 *     one resource
 *
 * to:
 *
 *     many resources
 *
 * to:
 *
 *     arbitrarily large resource sets
 *
 * subject only to actual compiler, runtime, deployment, and hardware
 * availability.
 *
 * "Infinity" means no artificial language-level capacity limit.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 46. POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * Minimum positive examples:
 *
 *     {
 *         placement::scope = execution_region;
 *     }
 *
 *     {
 *         placement::affinity = stage_a, stage_b;
 *     }
 *
 *     {
 *         placement::anti_affinity = replica_a, replica_b;
 *     }
 *
 *     {
 *         placement::co_location = producer, consumer;
 *     }
 *
 *     {
 *         placement::separation = tenant_a, tenant_b;
 *     }
 *
 *     {
 *         placement::target = accelerator;
 *     }
 *
 *     {
 *         placement::replicas = replica_count;
 *     }
 *
 *     {
 *         placement::migration = migration_policy;
 *     }
 *
 *     {
 *         placement::elasticity = available_resources;
 *     }
 *
 *     {
 *         quantum::placement::fidelity = desired_fidelity;
 *     }
 *
 *     {
 *         vendor::future::placement::metric = desired_value;
 *     }
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 47. NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * Negative tests MUST include:
 *
 *     {}
 *
 * only when an empty specification is semantically forbidden by its consumer;
 *
 *     {
 *         placement::scope
 *     }
 *
 *     {
 *         placement::scope =
 *     }
 *
 *     {
 *         placement::scope = ;
 *     }
 *
 *     {
 *         ::scope = value;
 *     }
 *
 *     {
 *         scope:: = value;
 *     }
 *
 *     {
 *         scope::::region = value;
 *     }
 *
 *     {
 *         placement::scope = value
 *     }
 *
 * when the terminating semicolon is required by the consumer.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 48. BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Boundary tests MUST include:
 *
 *     one placement clause;
 *     many placement clauses;
 *     deeply qualified property names;
 *     large expression lists;
 *     deeply nested placement specifications where a consumer permits them;
 *     symbolic quantities;
 *     very large numeric program values;
 *     empty optional placement specifications;
 *     repeated placement categories.
 *
 * None of these tests may establish a language-level hardware maximum.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 49. SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * The test suite must verify that:
 *
 *     one placement target works;
 *     many placement targets work;
 *     one relation works;
 *     many relations work;
 *     one resource works;
 *     arbitrarily large resource expressions remain grammatically valid;
 *     namespace depth is not fixed;
 *     property count is not fixed;
 *     group count is not fixed;
 *     replication quantities are semantic expressions;
 *     no device-count limit exists.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 50. COMPATIBILITY TEST CONTRACT
 * ============================================================================
 *
 * Compatibility tests must verify:
 *
 *     resources.g4
 *         -> ResourcePlacement
 *
 *     hardware placement
 *         -> canonical semantic placement model
 *
 *     distributed placement
 *         -> canonical semantic placement model
 *
 *     execution placement
 *         -> canonical semantic placement model
 *
 * and must ensure these layers do not create incompatible duplicate AST or IR
 * representations.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 51. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *     MAX_PLACEMENTS
 *     physical device IDs
 *     vendor IDs
 *     fixed topology sizes
 *     fixed qubit IDs
 *     fixed machine counts.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 52. RUST INTEGRATION
 * ============================================================================
 *
 * This grammar contains no Rust.
 *
 * Generated parser integration must:
 *
 *     - compile with Rust 1.97 / Rust 1.97.1;
 *     - use Rust 2021;
 *     - require no unsafe Rust;
 *     - preserve source spans;
 *     - remain deterministic;
 *     - avoid target-specific parser behavior.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 53. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] it is an independent ANTLR parser grammar;
 *     [x] canonical ZamaniLexer vocabulary is consumed;
 *     [x] canonical Names grammar is reused;
 *     [x] canonical ResourceExpressions grammar is reused;
 *     [x] no expression grammar is duplicated;
 *     [x] no identifier grammar is duplicated;
 *     [x] placement intent is represented as open-world semantic properties;
 *     [x] affinity is representable;
 *     [x] anti-affinity is representable;
 *     [x] co-location is representable;
 *     [x] separation is representable;
 *     [x] locality/scope is representable;
 *     [x] target intent is representable;
 *     [x] requirements are distinguishable semantically;
 *     [x] constraints are distinguishable semantically;
 *     [x] preferences are distinguishable semantically;
 *     [x] hints are distinguishable semantically;
 *     [x] policy intent is representable;
 *     [x] replication intent is representable;
 *     [x] migration/mobility intent is representable;
 *     [x] elasticity is representable;
 *     [x] arbitrary future placement properties are representable;
 *     [x] no hardware topology is duplicated;
 *     [x] no routing algorithm is embedded;
 *     [x] no scheduling algorithm is embedded;
 *     [x] no physical resource IDs are embedded;
 *     [x] no fixed resource capacities are embedded;
 *     [x] no fixed number of placement objects is embedded;
 *     [x] no second quantum IR is introduced;
 *     [x] AST/semantic/IR integration is specified;
 *     [x] Rust 1.97/1.97.1 compatibility is specified;
 *     [x] unsafe Rust is not required;
 *     [x] deterministic parsing is preserved;
 *     [x] scalability is unbounded by grammar design.
 *
 * ============================================================================
 */
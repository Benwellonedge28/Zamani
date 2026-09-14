/**
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *   grammar/hardware/placement.g4
 *
 * Grammar kind:
 *   ANTLR4 parser grammar
 *
 * Target:
 *   Rust 1.97 / Rust 1.97.1
 *   Rust 2021
 *
 * Safety:
 *   No embedded target-language actions.
 *   No semantic predicates.
 *   No generated-code customization.
 *   No unsafe Rust.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar defines the SOURCE-LEVEL PLACEMENT LANGUAGE.
 *
 * Placement describes where a computation, resource, logical object, hardware
 * object, or execution unit is intended or permitted to reside.
 *
 * Placement is NOT:
 *
 *   - hardware discovery;
 *   - hardware enumeration;
 *   - topology discovery;
 *   - routing;
 *   - scheduling;
 *   - optimization;
 *   - calibration;
 *   - device selection by hidden compiler policy;
 *   - runtime dispatch;
 *   - physical resource validation;
 *   - quantum-state movement;
 *   - SWAP insertion;
 *   - QEC;
 *   - ZQN/noise modelling.
 *
 * Placement is an intent/constraint/association layer.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 *
 *   - placement declarations;
 *   - placement bindings;
 *   - placement scopes;
 *   - placement regions;
 *   - placement groups;
 *   - placement relationships;
 *   - affinity/anti-affinity;
 *   - co-location/separation;
 *   - placement requirements;
 *   - placement constraints;
 *   - placement preferences;
 *   - placement hints;
 *   - placement policies;
 *   - placement replication intent;
 *   - migration intent;
 *   - elasticity intent;
 *   - placement target references;
 *   - placement capability references;
 *   - symbolic placement selectors;
 *   - placement metadata.
 *
 * THIS FILE DOES NOT OWN
 *
 *   - lexical definitions;
 *   - identifiers;
 *   - literal recognition;
 *   - general expression semantics;
 *   - type checking;
 *   - hardware discovery;
 *   - hardware capabilities;
 *   - canonical resource identities;
 *   - topology discovery;
 *   - route generation;
 *   - path finding;
 *   - scheduling;
 *   - optimization;
 *   - quantum IR;
 *   - QEC;
 *   - ZQN;
 *   - runtime execution;
 *   - physical validation.
 *
 * ============================================================================
 * ARCHITECTURAL BOUNDARY
 * ============================================================================
 *
 * Source
 *   |
 *   v
 * Lexer
 *   |
 *   v
 * placement.g4
 *   |
 *   v
 * Placement syntax tree
 *   |
 *   v
 * Semantic analysis
 *   |
 *   +-------------------------------+
 *   |                               |
 *   v                               v
 * Placement intent              Resource requirements
 *   |                               |
 *   +---------------+---------------+
 *                   |
 *                   v
 *             canonical IR /
 *             resource model
 *                   |
 *          +--------+---------+
 *          |                  |
 *          v                  v
 *       routing           scheduling
 *          |                  |
 *          +--------+---------+
 *                   |
 *                   v
 *              hardware HAL
 *                   |
 *                   v
 *                runtime
 *
 * There is deliberately NO:
 *
 *   placement -> routing implementation
 *   placement -> scheduler implementation
 *   placement -> hardware implementation
 *   placement -> runtime implementation
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Placement must preserve:
 *
 *   Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Therefore source placement MUST NOT require:
 *
 *   - a fixed number of resources;
 *   - a fixed number of devices;
 *   - a fixed topology;
 *   - a fixed device identifier;
 *   - a fixed hardware address;
 *   - a fixed CPU count;
 *   - a fixed GPU count;
 *   - a fixed FPGA count;
 *   - a fixed QPU count;
 *   - a fixed qubit count;
 *   - a fixed memory size;
 *   - a fixed cluster size;
 *   - a fixed node count.
 *
 * A placement may describe:
 *
 *   - semantic requirements;
 *   - symbolic locations;
 *   - resource classes;
 *   - capabilities;
 *   - topology properties;
 *   - locality;
 *   - affinity;
 *   - anti-affinity;
 *   - constraints;
 *   - preferences;
 *   - policies;
 *   - migration permissions;
 *   - replication requirements.
 *
 * Actual physical realization belongs downstream.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are NO MAX_* constants.
 *
 * There are NO fixed:
 *
 *   qubits
 *   nodes
 *   devices
 *   cores
 *   threads
 *   accelerators
 *   regions
 *   placement groups
 *   dimensions
 *   topology sizes
 *
 * Repetition uses ANTLR repetition operators.
 *
 * Quantities use expressions.
 *
 * A quantity may therefore be:
 *
 *   symbolic;
 *   compile-time;
 *   runtime-derived;
 *   target-derived;
 *   resource-negotiated.
 *
 * The grammar does not decide which one it is.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains no unordered grammar construct.
 *
 * Deterministic parse structure is preserved by:
 *
 *   - explicit alternatives;
 *   - deterministic token vocabulary;
 *   - no semantic predicates;
 *   - no embedded actions;
 *   - no target-language code.
 *
 * ============================================================================
 * TOKEN CONTRACT
 * ============================================================================
 *
 * This grammar requires the following stable placement vocabulary in:
 *
 *   grammar/lexer/tokens.g4
 *
 * Core:
 *
 *   K_PLACEMENT
 *   K_PLACE
 *   K_BIND
 *   K_TO
 *   K_ON
 *   K_AT
 *   K_IN
 *
 * Relationship:
 *
 *   K_AFFINITY
 *   K_ANTI_AFFINITY
 *   K_COLOCATE
 *   K_SEPARATE
 *   K_NEAR
 *   K_FAR
 *   K_ADJACENT
 *   K_AVOID
 *
 * Grouping:
 *
 *   K_GROUP
 *   K_REGION
 *   K_DOMAIN
 *   K_SCOPE
 *
 * Resource intent:
 *
 *   K_RESOURCE
 *   K_TARGET
 *   K_CAPABILITY
 *   K_REQUIRE
 *   K_CONSTRAINT
 *   K_PREFERENCE
 *   K_HINT
 *
 * Policy:
 *
 *   K_POLICY
 *   K_STRATEGY
 *   K_PRIORITY
 *
 * Placement policy:
 *
 *   K_PACK
 *   K_SPREAD
 *   K_BALANCED
 *   K_DENSE
 *   K_SPARSE
 *
 * Lifetime/mobility:
 *
 *   K_MIGRATABLE
 *   K_IMMIGRATABLE
 *   K_REPLICATED
 *   K_REPLICA
 *   K_ELASTIC
 *   K_FIXED
 *
 * Scope:
 *
 *   K_LOCAL
 *   K_REMOTE
 *   K_ANY
 *   K_EXCLUSIVE
 *   K_SHARED
 *
 * Location/resource properties:
 *
 *   K_NODE
 *   K_DEVICE
 *   K_CPU
 *   K_GPU
 *   K_FPGA
 *   K_ASIC
 *   K_QPU
 *   K_ACCELERATOR
 *   K_MEMORY
 *   K_SOCKET
 *   K_CORE
 *   K_THREAD
 *   K_RACK
 *   K_ZONE
 *   K_CLUSTER
 *   K_NUMA
 *
 * Comparison/selection:
 *
 *   K_MATCH
 *   K_WHERE
 *   K_BY
 *   K_BEST
 *   K_FIRST
 *   K_MIN
 *   K_MAX
 *
 * The exact spelling of these tokens is part of the language specification.
 *
 * No vendor name, device name, topology name, machine ID, or hardware count
 * belongs in the lexer vocabulary.
 *
 * ============================================================================
 */

parser grammar ZamaniHardwarePlacementParser;

options {
    tokenVocab = ZamaniTokens;
}


// ============================================================================
// 1. PUBLIC PLACEMENT ENTRY POINT
// ============================================================================
//
// This is the rule consumed by hardware.g4:
//
//     hardwarePlacementDecl
//
// It is deliberately named exactly that way so the existing hardware grammar
// can delegate placement syntax without duplicating placement rules.
//

hardwarePlacementDecl
    : placementAnnotation*
      placementVisibility?
      K_PLACEMENT
      IDENTIFIER
      placementGenericParameters?
      placementScopeClause?
      placementRequirementClause?
      placementCapabilityClause?
      LBRACE
      placementItem*
      RBRACE
    ;


// ============================================================================
// 2. VISIBILITY
// ============================================================================

placementVisibility
    : K_PUBLIC
    | K_PRIVATE
    | K_PROTECTED
    | K_INTERNAL
    ;


// ============================================================================
// 3. ANNOTATIONS
// ============================================================================
//
// Annotations are metadata.
// They do not perform placement.
//

placementAnnotation
    : AT placementQualifiedName
      (
          LPAREN
          placementArgumentList?
          RPAREN
      )?
    ;

placementArgumentList
    : placementArgument
      (COMMA placementArgument)*
    ;

placementArgument
    : placementQualifiedName
    | STRING_LITERAL
    | INTEGER_LITERAL
    | FLOAT_LITERAL
    | placementExpression
    ;


// ============================================================================
// 4. GENERIC PARAMETERS
// ============================================================================
//
// Generic parameters allow placement descriptions to scale.
//
// Example:
//
//     placement ClusterPlacement<N, Capacity> { ... }
//
// N is not a machine limit.
// It is a semantic parameter.
//

placementGenericParameters
    : LESS_THAN
      placementGenericParameter
      (COMMA placementGenericParameter)*
      GREATER_THAN
    ;

placementGenericParameter
    : IDENTIFIER
      placementGenericBound?
      (
          ASSIGN
          placementExpression
      )?
    ;

placementGenericBound
    : COLON
      placementTypeReference
    ;


// ============================================================================
// 5. PLACEMENT BODY
// ============================================================================

placementItem
    : placementBinding
    | placementGroup
    | placementRegion
    | placementRelation
    | placementRequirement
    | placementConstraint
    | placementPreference
    | placementHint
    | placementPolicy
    | placementReplication
    | placementMigration
    | placementElasticity
    | placementTarget
    | placementCapability
    | placementScope
    | placementProperty
    ;


// ============================================================================
// 6. DIRECT PLACEMENT BINDING
// ============================================================================
//
// These are semantic associations.
//
// They do NOT mean that the compiler must physically move anything.
//

placementBinding
    : placementBindingVerb
      placementSubject
      placementBindingOperator
      placementLocation
      SEMICOLON
    ;

placementBindingVerb
    : K_PLACE
    | K_BIND
    ;

placementBindingOperator
    : K_TO
    | K_ON
    | K_AT
    | K_IN
    ;


// ============================================================================
// 7. PLACEMENT SUBJECT
// ============================================================================
//
// A subject may identify:
//
//   - a computation;
//   - a module;
//   - a task;
//   - a logical qubit;
//   - a logical register;
//   - a data object;
//   - a hardware component;
//   - a resource class;
//   - a user-defined semantic object.
//
// The grammar does not impose a physical interpretation.
//

placementSubject
    : placementQualifiedName
    | placementSubjectSelector
    ;

placementSubjectSelector
    : LBRACKET
      placementSelectorExpression
      RBRACKET
    ;


// ============================================================================
// 8. PLACEMENT LOCATION
// ============================================================================
//
// A location is symbolic.
//
// It can refer to:
//
//   - a logical region;
//   - a resource class;
//   - a target capability;
//   - a deployment scope;
//   - a topology region;
//   - a backend-provided resource;
//   - a semantic location.
//
// Physical resolution is downstream.
//

placementLocation
    : placementQualifiedName
    | placementLocationSelector
    | placementLocationExpression
    ;

placementLocationSelector
    : LBRACKET
      placementSelectorExpression
      RBRACKET
    ;

placementLocationExpression
    : LPAREN
      placementExpression
      RPAREN
    ;


// ============================================================================
// 9. GROUPS
// ============================================================================
//
// Groups allow a collection of subjects to share placement semantics.
//

placementGroup
    : K_GROUP
      IDENTIFIER
      placementGroupMode?
      LBRACE
      placementGroupMember*
      RBRACE
    ;

placementGroupMode
    : K_PACK
    | K_SPREAD
    | K_BALANCED
    | K_DENSE
    | K_SPARSE
    ;

placementGroupMember
    : placementSubject
      SEMICOLON
    ;


// ============================================================================
// 10. REGIONS
// ============================================================================
//
// A region is a symbolic placement domain.
//
// It is NOT a hardware topology definition.
//
// Actual region discovery is performed downstream.
//

placementRegion
    : K_REGION
      IDENTIFIER
      placementRegionAttributes?
      LBRACE
      placementRegionItem*
      RBRACE
    ;

placementRegionAttributes
    : LPAREN
      placementPropertyList?
      RPAREN
    ;

placementRegionItem
    : placementLocation
      SEMICOLON
    | placementRequirement
    | placementConstraint
    | placementPreference
    | placementProperty
    ;


// ============================================================================
// 11. RELATIONSHIPS
// ============================================================================
//
// Relations express semantic relationships without choosing an algorithm.
//

placementRelation
    : placementAffinity
    | placementAntiAffinity
    | placementCoLocation
    | placementSeparation
    | placementDistance
    | placementAdjacency
    | placementAvoidance
    ;

placementAffinity
    : K_AFFINITY
      placementRelationSubject
      placementRelationOperator
      placementRelationSubject
      SEMICOLON
    ;

placementAntiAffinity
    : K_ANTI_AFFINITY
      placementRelationSubject
      placementRelationOperator
      placementRelationSubject
      SEMICOLON
    ;

placementCoLocation
    : K_COLOCATE
      placementRelationSubject
      placementRelationOperator
      placementRelationSubject
      SEMICOLON
    ;

placementSeparation
    : K_SEPARATE
      placementRelationSubject
      placementRelationOperator
      placementRelationSubject
      SEMICOLON
    ;

placementDistance
    : placementDistanceKeyword
      placementRelationSubject
      placementDistanceOperator
      placementExpression
      SEMICOLON
    ;

placementDistanceKeyword
    : K_NEAR
    | K_FAR
    ;

placementDistanceOperator
    : LESS_THAN
    | LESS_EQUAL
    | GREATER_THAN
    | GREATER_EQUAL
    | ASSIGN
    ;

placementAdjacency
    : K_ADJACENT
      placementRelationSubject
      placementRelationOperator
      placementRelationSubject
      SEMICOLON
    ;

placementAvoidance
    : K_AVOID
      placementRelationSubject
      placementRelationOperator
      placementRelationSubject
      SEMICOLON
    ;

placementRelationSubject
    : placementSubject
    | placementLocation
    ;

placementRelationOperator
    : K_TO
    | K_ON
    | K_IN
    | K_WITH
    ;


// ============================================================================
// 12. REQUIREMENTS
// ============================================================================
//
// Requirements are mandatory semantic conditions.
//
// They are NOT automatically interpreted as a particular physical device.
//

placementRequirement
    : K_REQUIRE
      placementRequirementBody
      SEMICOLON
    ;

placementRequirementBody
    : placementRequirementExpression
    | placementResourceRequirement
    | placementCapabilityRequirement
    | placementLocationRequirement
    ;

placementResourceRequirement
    : K_RESOURCE
      placementResourceSelector
    ;

placementCapabilityRequirement
    : K_CAPABILITY
      placementCapabilitySelector
    ;

placementLocationRequirement
    : placementSubject
      placementRequirementOperator
      placementLocation
    ;

placementRequirementExpression
    : placementExpression
    ;

placementRequirementOperator
    : K_TO
    | K_ON
    | K_IN
    | K_WITH
    ;


// ============================================================================
// 13. CONSTRAINTS
// ============================================================================
//
// Constraints restrict valid placement solutions.
//
// They do not prescribe how a router or scheduler must satisfy them.
//

placementConstraint
    : K_CONSTRAINT
      placementConstraintBody
      SEMICOLON
    ;

placementConstraintBody
    : placementExpression
    | placementRelation
    | placementResourceConstraint
    | placementCapacityConstraint
    | placementLocationConstraint
    ;

placementResourceConstraint
    : K_RESOURCE
      placementResourceSelector
    ;

placementCapacityConstraint
    : placementCapacityName
      placementComparisonOperator
      placementExpression
    ;

placementCapacityName
    : K_MEMORY
    | K_CPU
    | K_GPU
    | K_FPGA
    | K_QPU
    | K_ACCELERATOR
    | K_CORE
    | K_THREAD
    | placementQualifiedName
    ;

placementLocationConstraint
    : placementSubject
      placementComparisonOperator
      placementLocation
    ;

placementComparisonOperator
    : ASSIGN
    | EQUAL_EQUAL
    | NOT_EQUAL
    | LESS_THAN
    | LESS_EQUAL
    | GREATER_THAN
    | GREATER_EQUAL
    ;


// ============================================================================
// 14. PREFERENCES
// ============================================================================
//
// Preferences are advisory.
//
// A preference may be rejected by the compilation/runtime environment when
// satisfying it would violate mandatory requirements or constraints.
//

placementPreference
    : K_PREFERENCE
      placementPreferenceBody
      SEMICOLON
    ;

placementPreferenceBody
    : placementExpression
    | placementPreferenceProperty
    | placementPreferenceStrategy
    ;

placementPreferenceProperty
    : placementPropertyName
      placementComparisonOperator
      placementExpression
    ;

placementPreferenceStrategy
    : K_STRATEGY
      placementStrategy
    ;

placementStrategy
    : K_PACK
    | K_SPREAD
    | K_BALANCED
    | K_DENSE
    | K_SPARSE
    | K_BEST
    | K_FIRST
    ;


// ============================================================================
// 15. HINTS
// ============================================================================
//
// Hints are weaker than preferences.
//
// They must never silently become mandatory constraints.
//

placementHint
    : K_HINT
      placementHintBody
      SEMICOLON
    ;

placementHintBody
    : placementExpression
    | placementProperty
    | placementLocation
    ;


// ============================================================================
// 16. POLICIES
// ============================================================================
//
// Policies express permitted placement behavior.
//
// Policy interpretation belongs to semantic/compiler layers.
//

placementPolicy
    : K_POLICY
      IDENTIFIER?
      LBRACE
      placementPolicyItem*
      RBRACE
    ;

placementPolicyItem
    : placementPolicyProperty
      SEMICOLON
    | placementPolicyRule
      SEMICOLON
    ;

placementPolicyProperty
    : placementPropertyName
      ASSIGN
      placementExpression
    ;

placementPolicyRule
    : placementExpression
    | placementRelation
    ;


// ============================================================================
// 17. REPLICATION
// ============================================================================
//
// Replication is expressed symbolically.
//
// There is no fixed replica count.
//

placementReplication
    : K_REPLICATED
      placementReplicationBody
      SEMICOLON
    ;

placementReplicationBody
    : placementSubject
      placementReplicationCount?
    | placementGroupReference
      placementReplicationCount?
    ;

placementReplicationCount
    : placementExpression
    ;

placementGroupReference
    : K_GROUP
      placementQualifiedName
    ;


// ============================================================================
// 18. MIGRATION
// ============================================================================
//
// Migration specifies whether a placement may change.
//
// It does NOT perform migration.
//

placementMigration
    : placementMigrationMode
      placementMigrationSubject
      SEMICOLON
    ;

placementMigrationMode
    : K_MIGRATABLE
    | K_IMMIGRATABLE
    ;

placementMigrationSubject
    : placementSubject
    | placementGroupReference
    ;


// ============================================================================
// 19. ELASTICITY
// ============================================================================
//
// Elastic placement permits resource cardinality to be determined by the
// compilation/execution environment.
//

placementElasticity
    : K_ELASTIC
      placementElasticityBody
      SEMICOLON
    ;

placementElasticityBody
    : placementSubject
    | placementGroupReference
    | placementExpression
    ;


// ============================================================================
// 20. TARGET REFERENCES
// ============================================================================
//
// A target is a semantic target description.
//
// It is NOT a physical device ID.
//

placementTarget
    : K_TARGET
      placementTargetReference
      placementTargetQualifier*
      SEMICOLON
    ;

placementTargetReference
    : placementQualifiedName
    | placementTargetSelector
    ;

placementTargetSelector
    : LBRACKET
      placementSelectorExpression
      RBRACKET
    ;

placementTargetQualifier
    : K_REQUIRE
      placementExpression
    | K_PREFERENCE
      placementExpression
    | K_CONSTRAINT
      placementExpression
    ;


// ============================================================================
// 21. CAPABILITY REFERENCES
// ============================================================================
//
// Capabilities describe what an implementation must provide or prefer.
//
// The grammar never turns a capability into a specific vendor/device.
//

placementCapability
    : K_CAPABILITY
      placementCapabilitySelector
      placementCapabilityQualifier*
      SEMICOLON
    ;

placementCapabilitySelector
    : placementQualifiedName
    | LBRACKET
      placementSelectorExpression
      RBRACKET
    ;

placementCapabilityQualifier
    : K_REQUIRE
      placementExpression
    | K_PREFERENCE
      placementExpression
    | K_CONSTRAINT
      placementExpression
    ;


// ============================================================================
// 22. SCOPES
// ============================================================================
//
// Scope describes the semantic deployment locality.
//
// It does not enumerate physical nodes.
//

placementScope
    : K_SCOPE
      placementScopeValue
      SEMICOLON
    ;

placementScopeClause
    : K_SCOPE
      placementScopeValue
    ;

placementScopeValue
    : K_LOCAL
    | K_REMOTE
    | K_ANY
    | placementQualifiedName
    ;


// ============================================================================
// 23. PROPERTY DECLARATIONS
// ============================================================================
//
// Properties remain symbolic and target-independent.
//

placementProperty
    : placementPropertyName
      ASSIGN
      placementExpression
      SEMICOLON
    ;

placementPropertyList
    : placementPropertyAssignment
      (COMMA placementPropertyAssignment)*
    ;

placementPropertyAssignment
    : placementPropertyName
      ASSIGN
      placementExpression
    ;

placementPropertyName
    : K_PRIORITY
    | K_MEMORY
    | K_CPU
    | K_GPU
    | K_FPGA
    | K_QPU
    | K_ACCELERATOR
    | K_SOCKET
    | K_CORE
    | K_THREAD
    | K_NODE
    | K_DEVICE
    | K_RACK
    | K_ZONE
    | K_CLUSTER
    | K_NUMA
    | K_DOMAIN
    | placementQualifiedName
    ;


// ============================================================================
// 24. RESOURCE SELECTORS
// ============================================================================
//
// Resource selectors identify classes of resources.
//
// They do not enumerate discovered hardware.
//

placementResourceSelector
    : placementResourceClass
    | placementQualifiedName
    | LBRACKET
      placementSelectorExpression
      RBRACKET
    ;

placementResourceClass
    : K_CPU
    | K_GPU
    | K_FPGA
    | K_ASIC
    | K_QPU
    | K_ACCELERATOR
    | K_MEMORY
    | K_NODE
    | K_DEVICE
    ;


// ============================================================================
// 25. SELECTOR EXPRESSIONS
// ============================================================================
//
// Selectors are intentionally symbolic.
//
// Example semantic forms:
//
//     [capability = "quantum"]
//     [memory >= required_memory]
//     [region = execution_region]
//
// The grammar does not decide whether the selector is evaluated at compile
// time, deployment time, or runtime.
//

placementSelectorExpression
    : placementSelectorTerm
      (
          placementSelectorOperator
          placementSelectorTerm
      )*
    ;

placementSelectorTerm
    : placementQualifiedName
    | STRING_LITERAL
    | INTEGER_LITERAL
    | FLOAT_LITERAL
    | placementSelectorGroup
    | LPAREN
      placementSelectorExpression
      RPAREN
    ;

placementSelectorGroup
    : LBRACE
      placementSelectorExpression
      RBRACE
    ;

placementSelectorOperator
    : ASSIGN
    | EQUAL_EQUAL
    | NOT_EQUAL
    | LESS_THAN
    | LESS_EQUAL
    | GREATER_THAN
    | GREATER_EQUAL
    | LOGICAL_AND
    | LOGICAL_OR
    ;


// ============================================================================
// 26. GENERAL PLACEMENT EXPRESSIONS
// ============================================================================
//
// This is deliberately a placement-level expression surface rather than a
// second type system.
//
// The canonical semantic expression system remains authoritative.
//
// When this grammar is imported by the root parser, these expressions should
// be lowered into the canonical expression AST rather than represented as a
// second expression implementation.
//

placementExpression
    : placementExpressionAtom
      (
          placementExpressionOperator
          placementExpressionAtom
      )*
    ;

placementExpressionAtom
    : placementQualifiedName
    | INTEGER_LITERAL
    | FLOAT_LITERAL
    | STRING_LITERAL
    | LPAREN
      placementExpression
      RPAREN
    ;

placementExpressionOperator
    : PLUS
    | MINUS
    | STAR
    | SLASH
    | PERCENT
    | EQUAL_EQUAL
    | NOT_EQUAL
    | LESS_THAN
    | LESS_EQUAL
    | GREATER_THAN
    | GREATER_EQUAL
    | LOGICAL_AND
    | LOGICAL_OR
    ;


// ============================================================================
// 27. TYPE REFERENCES
// ============================================================================
//
// Placement does not define a second type system.
//

placementTypeReference
    : placementQualifiedName
    ;


// ============================================================================
// 28. QUALIFIED NAMES
// ============================================================================
//
// Qualified names are used for semantic references.
//
// Examples:
//
//     logical.q0
//     resource.gpu
//     target.quantum
//     topology.region
//
// No physical device identity is implied.
//

placementQualifiedName
    : IDENTIFIER
      (
          DOUBLE_COLON
          IDENTIFIER
      )*
    ;


// ============================================================================
// 29. SAFE INTEGRATION ADAPTER
// ============================================================================
//
// Hardware grammar can delegate:
//
//     hardwarePlacementDecl
//
// directly to this grammar.
//
// No rule here references:
//
//     routing
//     scheduling
//     quantum::ir
//     QEC
//     ZQN
//     hardware HAL
//
// Those are downstream semantic consumers.
//
// ============================================================================


// ============================================================================
// 30. HARD-CODING INVARIANTS
// ============================================================================
//
// This grammar MUST NEVER introduce:
//
//     MAX_QUBITS
//     MAX_DEVICES
//     MAX_NODES
//     MAX_CORES
//     MAX_GPUS
//     MAX_FPGAS
//     MAX_MEMORY
//     MAX_REGIONS
//     MAX_PLACEMENTS
//     fixed topology dimensions
//     fixed hardware addresses
//     fixed vendor identifiers
//     fixed device identifiers
//
// Any such requirement belongs to:
//
//     semantic validation
//     resource policy
//     target description
//     hardware capability model
//     deployment configuration
//     runtime resource manager
//
// ============================================================================


// ============================================================================
// 31. SEMANTIC INTEGRATION CONTRACT
// ============================================================================
//
// The parser output must be lowered into the canonical Zamani AST.
//
// Semantic analysis must classify every placement statement as one of:
//
//     Binding
//     Requirement
//     Constraint
//     Preference
//     Hint
//     Relationship
//     Policy
//     Scope
//     Mobility
//     Replication
//     Elasticity
//
// Semantic analysis MUST preserve the distinction:
//
//     requirement != constraint != preference != hint
//
// and:
//
//     placement intent != routing decision
//
// and:
//
//     placement intent != scheduling decision
//
// and:
//
//     placement intent != hardware discovery
//
// ============================================================================


// ============================================================================
// 32. QUANTUM INTEGRATION CONTRACT
// ============================================================================
//
// Quantum placement syntax may reference logical/physical semantic objects.
//
// The grammar itself MUST NOT define:
//
//     QubitId
//     PhysicalQubitId
//
// The canonical quantum IR remains the owner of those identities.
//
// Therefore:
//
//     placement.g4
//         -> syntax
//         -> semantic placement intent
//         -> canonical IR/resource mapping
//
// and NOT:
//
//     placement.g4
//         -> second quantum mapping model
//
// ============================================================================


// ============================================================================
// 33. ROUTING INTEGRATION CONTRACT
// ============================================================================
//
// Routing consumes placement intent and resource/topology information.
//
// Routing is responsible for deciding:
//
//     - mapping realization;
//     - path selection;
//     - movement;
//     - SWAP insertion;
//     - connectivity realization;
//     - remapping.
//
// placement.g4 does none of those things.
//
// ============================================================================


// ============================================================================
// 34. SCHEDULING INTEGRATION CONTRACT
// ============================================================================
//
// Scheduling consumes placement/resource state.
//
// Scheduling decides:
//
//     - ordering;
//     - timing;
//     - resource occupancy;
//     - synchronization;
//     - temporal constraints.
//
// placement.g4 does not schedule anything.
//
// ============================================================================


// ============================================================================
// 35. HARDWARE HAL INTEGRATION CONTRACT
// ============================================================================
//
// Hardware/HAL resolves:
//
//     target
//     capability
//     resource
//     topology
//     availability
//     calibration
//     physical identity
//
// Placement syntax merely expresses the program's intent.
//
// ============================================================================


// ============================================================================
// 36. RUNTIME INTEGRATION CONTRACT
// ============================================================================
//
// Runtime may consume placement information after compilation.
//
// Runtime MUST NOT reinterpret source placement syntax independently.
//
// The semantic representation produced by compilation is authoritative.
//
// ============================================================================


// ============================================================================
// 37. SECURITY CONTRACT
// ============================================================================
//
// Placement syntax cannot:
//
//     - open devices;
//     - access physical addresses;
//     - enumerate hardware;
//     - access files;
//     - access networks;
//     - execute commands;
//     - load drivers;
//     - bypass capability checks.
//
// Placement is declarative syntax only.
//
// ============================================================================


// ============================================================================
// 38. ERROR-BOUNDARY CONTRACT
// ============================================================================
//
// Syntax errors belong to parser diagnostics.
//
// Semantic placement errors belong to semantic analysis.
//
// Resource unavailability belongs to resource resolution.
//
// Hardware incompatibility belongs to hardware validation.
//
// Routing failure belongs to routing.
//
// Scheduling failure belongs to scheduling.
//
// Runtime failure belongs to runtime.
//
// This grammar must not encode backend/provider error codes.
//
// ============================================================================


// ============================================================================
// 39. COMPLETION INVARIANT
// ============================================================================
//
// placement.g4 is complete when:
//
//   1. placement declarations parse deterministically;
//   2. placement binding parses;
//   3. placement groups parse;
//   4. placement regions parse;
//   5. affinity parses;
//   6. anti-affinity parses;
//   7. co-location parses;
//   8. separation parses;
//   9. locality parses;
//  10. requirements parse;
//  11. constraints parse;
//  12. preferences parse;
//  13. hints parse;
//  14. policies parse;
//  15. replication parses;
//  16. migration intent parses;
//  17. elasticity parses;
//  18. target references parse;
//  19. capability references parse;
//  20. symbolic resource selectors parse;
//  21. symbolic expressions parse;
//  22. qualified names parse;
//  23. no hardware count is hard-coded;
//  24. no topology is hard-coded;
//  25. no device ID is hard-coded;
//  26. no vendor is hard-coded;
//  27. no quantum ID type is redefined;
//  28. no routing algorithm is embedded;
//  29. no scheduler policy is embedded;
//  30. no runtime behavior is embedded;
//  31. no unsafe Rust is introduced;
//  32. the grammar integrates with hardware.g4;
//  33. semantic lowering can distinguish requirements,
//      constraints, preferences, and hints;
//  34. downstream routing can consume placement intent;
//  35. downstream scheduling can consume resolved placement;
//  36. downstream hardware validation can resolve targets;
//  37. POCO-REAF remains intact;
//  38. the same syntax can describe one resource or arbitrarily large
//      resource sets subject only to actual semantic/resource limits.
//
// ============================================================================
/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/hardware/placement.g4
 *
 * GRAMMAR
 * -------
 * ZamaniHardwarePlacementParser
 *
 * STATUS
 * ------
 * PRODUCTION-TARGET HARDWARE PLACEMENT GRAMMAR
 *
 * RUST BASELINE
 * -------------
 * Rust 1.97+
 * Rust 2021
 * Safe Rust only
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This grammar owns SOURCE-LEVEL HARDWARE PLACEMENT INTENT.
 *
 * Placement expresses where a computation, logical object, resource class,
 * execution unit, hardware abstraction, or other semantic object is intended,
 * permitted, preferred, constrained, or required to reside.
 *
 * Placement is intentionally target-independent.
 *
 *
 * THIS FILE OWNS
 * --------------
 *
 *   placement declarations
 *   placement bindings
 *   placement groups
 *   placement regions
 *   placement relationships
 *   affinity
 *   anti-affinity
 *   co-location
 *   separation
 *   locality/distance intent
 *   placement requirements
 *   placement constraints
 *   placement preferences
 *   placement hints
 *   placement policies
 *   replication intent
 *   migration intent
 *   elasticity intent
 *   target references
 *   capability references
 *   placement scopes
 *   open-world placement properties
 *   symbolic placement selectors
 *   placement annotations
 *
 *
 * THIS FILE DOES NOT OWN
 * ----------------------
 *
 *   lexical definitions
 *   identifiers
 *   qualified-name syntax
 *   expression precedence
 *   types
 *   resource declarations
 *   resource discovery
 *   capability discovery
 *   target discovery
 *   physical device enumeration
 *   physical addresses
 *   topology discovery
 *   topology construction
 *   routing
 *   scheduling
 *   optimization
 *   calibration
 *   physical allocation
 *   runtime migration
 *   QEC
 *   ZQN
 *   HAL implementation
 *   quantum operation syntax
 *   quantum::ir
 *   classical IR
 *
 *
 * ============================================================================
 * ARCHITECTURAL BOUNDARY
 * ============================================================================
 *
 * Zamani source
 *      |
 *      v
 * canonical lexer
 *      |
 *      v
 * this grammar
 *      |
 *      v
 * domain-neutral AST
 *      |
 *      v
 * placement semantic model
 *      |
 *      +-----------------------------+
 *      |                             |
 *      v                             v
 * resource analysis            capability analysis
 *      |                             |
 *      +-------------+---------------+
 *                    |
 *                    v
 *             realization planning
 *                    |
 *          +---------+---------+
 *          |         |         |
 *          v         v         v
 *       routing  scheduling  optimization
 *                    |
 *                    v
 *              target realization
 *                    |
 *                    v
 *                  HAL
 *
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Placement must preserve:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Therefore this grammar MUST NOT encode universal physical limits.
 *
 * It MUST NOT define:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_DEVICES
 *     MAX_ACCELERATORS
 *     MAX_REGIONS
 *     MAX_PLACEMENTS
 *     MAX_REPLICAS
 *
 * It also MUST NOT encode:
 *
 *     fixed device IDs
 *     physical addresses
 *     vendor IDs
 *     fixed topology sizes
 *     fixed machine sizes
 *     fixed node counts
 *     fixed hardware dimensions
 *
 * All quantities are expressions.
 *
 * A quantity may therefore be:
 *
 *     literal
 *     symbolic
 *     compile-time derived
 *     runtime derived
 *     workload derived
 *     resource derived
 *     target negotiated
 *
 * Physical feasibility is downstream.
 *
 *
 * ============================================================================
 * OPEN-WORLD EXTENSIBILITY
 * ============================================================================
 *
 * Placement MUST remain extensible without adding a parser rule for every
 * future hardware technology.
 *
 * Standard semantic categories have dedicated syntax where that syntax provides
 * real language value.
 *
 * Arbitrary future/vendor/domain properties use:
 *
 *     qualifiedName = expression;
 *
 * or:
 *
 *     [expression]
 *
 * and are interpreted semantically.
 *
 * This allows future resources such as:
 *
 *     optical
 *     neuromorphic
 *     molecular
 *     biological
 *     photonic
 *     reconfigurable
 *     accelerator
 *     future quantum substrates
 *
 * without changing the universal placement grammar.
 *
 *
 * ============================================================================
 * REQUIREMENT / CONSTRAINT / PREFERENCE / HINT SEPARATION
 * ============================================================================
 *
 * These categories are deliberately distinct.
 *
 * REQUIREMENT
 *     Mandatory semantic condition.
 *
 * CONSTRAINT
 *     Mandatory restriction on valid realization.
 *
 * PREFERENCE
 *     Advisory ranking guidance.
 *
 * HINT
 *     Weaker advisory information.
 *
 * A parser context must preserve the distinction so semantic analysis can
 * never accidentally turn a preference or hint into a requirement.
 *
 *
 * ============================================================================
 * RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * Placement may reference resources and capabilities.
 *
 * Placement does NOT define their semantics.
 *
 * Resource meaning belongs to:
 *
 *     grammar/resources/
 *
 * Capability meaning belongs to:
 *
 *     grammar/resources/
 *     grammar/hardware/
 *
 * Placement only attaches those semantic conditions to placement intent.
 *
 *
 * ============================================================================
 * TOPOLOGY SEPARATION
 * ============================================================================
 *
 * Placement may reference topology objects and topology properties.
 *
 * It does NOT define topology.
 *
 * Topology owns:
 *
 *     nodes
 *     edges
 *     connectivity
 *     directionality
 *     topology relationships
 *
 * Placement consumes topology intent only where required.
 *
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Placement may reference logical quantum objects.
 *
 * It MUST NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     quantum gate sets
 *     SWAP operations
 *     routing algorithms
 *     QEC
 *
 * Quantum semantic lowering remains:
 *
 *     frontend AST
 *          ->
 *     quantum semantic model
 *          ->
 *     quantum::ir
 *
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Placement may describe hardware realization intent used by HDL and hardware
 * compilation.
 *
 * It does not define:
 *
 *     RTL primitives
 *     physical floorplanning algorithms
 *     placement algorithms
 *     timing closure
 *     routing algorithms
 *     manufacturing constraints
 *
 * Those belong to downstream HDL/hardware semantic and backend layers.
 *
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Placement syntax itself produces no runtime effect.
 *
 * It is declarative.
 *
 * Runtime effects such as:
 *
 *     migration
 *     allocation
 *     device access
 *     network access
 *     synchronization
 *
 * are represented and checked by downstream semantic/effect systems.
 *
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Capability references are symbolic.
 *
 * Example:
 *
 *     capability quantum::measurement;
 *
 * or:
 *
 *     requires capability::quantum::measurement;
 *
 * The grammar does not decide whether the capability exists.
 *
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Resource quantities are represented by canonical expressions.
 *
 * Example:
 *
 *     requires resource::memory >= required_memory;
 *
 *     requires resource::qubits >= required_qubits;
 *
 * The grammar does not impose a physical upper bound.
 *
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Placement policies may express:
 *
 *     allow
 *     forbid
 *     prefer
 *     require
 *     fallback
 *
 * but policy evaluation belongs to the policy semantic layer.
 *
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * The parser preserves source structure and source ordering.
 *
 * AST lowering must retain source spans for:
 *
 *     declaration
 *     binding
 *     relationship
 *     requirement
 *     constraint
 *     preference
 *     hint
 *     policy
 *     replication
 *     migration
 *     elasticity
 *     property
 *
 * Provenance generation belongs to the frontend/compiler semantic layer.
 *
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar creates ANTLR parser contexts only.
 *
 * The frontend AST should provide semantic nodes corresponding to:
 *
 *     PlacementDeclaration
 *     PlacementBinding
 *     PlacementGroup
 *     PlacementRegion
 *     PlacementRelationship
 *     PlacementRequirement
 *     PlacementConstraint
 *     PlacementPreference
 *     PlacementHint
 *     PlacementPolicy
 *     PlacementReplication
 *     PlacementMigration
 *     PlacementElasticity
 *     PlacementTarget
 *     PlacementCapability
 *     PlacementScope
 *     PlacementProperty
 *     PlacementSelector
 *
 * The exact Rust AST type names belong to the frontend AST implementation.
 *
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis MUST preserve:
 *
 *     placement intent != physical allocation
 *     placement intent != routing
 *     placement intent != scheduling
 *     placement intent != discovery
 *
 * It must also preserve:
 *
 *     requirement != constraint
 *     constraint != preference
 *     preference != hint
 *
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates no IR.
 *
 * Placement lowers into the canonical semantic resource/hardware placement
 * model.
 *
 * It MUST NOT create:
 *
 *     PlacementIR
 *     QuantumPlacementIR
 *     DevicePlacementIR
 *
 * as a competing universal IR.
 *
 * Quantum placement information eventually participates in:
 *
 *     quantum::ir
 *
 * through the normal semantic lowering pipeline.
 *
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Syntax diagnostics belong to the parser.
 *
 * Semantic diagnostics include:
 *
 *     duplicate placement declaration
 *     unresolved placement subject
 *     unresolved placement location
 *     unresolved target
 *     unresolved capability
 *     invalid resource expression
 *     contradictory constraints
 *     contradictory mobility declarations
 *     invalid replication expression
 *     invalid selector
 *
 * Resource availability errors belong to resource analysis.
 *
 * Routing errors belong to routing.
 *
 * Scheduling errors belong to scheduling.
 *
 * Physical realization errors belong to the target/HAL layer.
 *
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/lexer/tokens.g4
 *     grammar/expressions/expressions.g4
 *     grammar/core/names.g4
 *
 * EXPORTS:
 *
 *     placementDeclaration
 *     placementItem
 *     placementBinding
 *     placementGroup
 *     placementRegion
 *     placementRelationship
 *     placementRequirement
 *     placementConstraint
 *     placementPreference
 *     placementHint
 *     placementPolicy
 *     placementReplication
 *     placementMigration
 *     placementElasticity
 *     placementTarget
 *     placementCapability
 *     placementScope
 *     placementProperty
 *     placementSelector
 *
 * CONSUMED_BY:
 *
 *     grammar/hardware/hardware.g4
 *     future hardware-domain composition grammars
 *     frontend parser integration
 *
 * AST_OWNER:
 *
 *     frontend AST implementation
 *
 * SEMANTIC_OWNER:
 *
 *     hardware/resource placement semantic analysis
 *
 * IR_OWNER:
 *
 *     canonical semantic resource/hardware model
 *     quantum::ir where quantum semantics require it
 *
 * TEST_OWNER:
 *
 *     grammar/tests/hardware/
 *     grammar/tests/semantic/
 *     grammar/tests/scalability/
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/hardware.md
 *     grammar/spec/resources.md
 *     grammar/specification/
 *
 * COMPATIBILITY_OWNER:
 *
 *     grammar/compatibility/
 *
 *
 * ============================================================================
 */

parser grammar ZamaniHardwarePlacementParser;

options {
    tokenVocab = ZamaniLexer;
}

import
    Expressions,
    Names
;


/* ============================================================================
 * 1. PUBLIC DECLARATION
 * ============================================================================
 *
 * Canonical hardware.g4 integration rule:
 *
 *     placementDeclaration
 *
 * Example:
 *
 *     placement compute_placement {
 *         place workload to accelerator;
 *     }
 */

placementDeclaration
    : placementAnnotation*
      placementVisibility?
      PLACEMENT
      qualifiedName
      placementParameterList?
      LBRACE
      placementItem*
      RBRACE
    ;


/* ============================================================================
 * 2. DECLARATION SUPPORT
 * ========================================================================== */

placementVisibility
    : PUBLIC
    | PRIVATE
    | PROTECTED
    | INTERNAL
    ;


placementAnnotation
    : AT
      qualifiedName
      (
          LPAREN
          expressionList?
          RPAREN
      )?
    ;


placementParameterList
    : LESS
      placementParameter
      (COMMA placementParameter)*
      GREATER
    ;


placementParameter
    : qualifiedName
      (
          ASSIGN
          expression
      )?
    ;


/* ============================================================================
 * 3. PLACEMENT BODY
 * ========================================================================== */

placementItem
    : placementBinding
    | placementGroup
    | placementRegion
    | placementRelationship
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


/* ============================================================================
 * 4. DIRECT PLACEMENT BINDING
 * ============================================================================
 *
 * Portable source intent:
 *
 *     place computation to accelerator;
 *     bind qpu_workload to quantum_backend;
 *
 * No physical device is selected by this syntax.
 */

placementBinding
    : placementBindingVerb
      qualifiedName
      TO
      placementLocation
      SEMICOLON
    ;


placementBindingVerb
    : PLACE
    | BIND
    ;


placementLocation
    : qualifiedName
      placementSelector?
    | LPAREN
      expression
      RPAREN
    ;


/* ============================================================================
 * 5. SUBJECT SELECTORS
 * ========================================================================== */

placementSubject
    : qualifiedName
      placementSelector?
    ;


placementSelector
    : LBRACKET
      expression
      RBRACKET
    ;


placementSelectorList
    : placementSelector+
    ;


/* ============================================================================
 * 6. GROUPS
 * ============================================================================
 *
 * Groups are logical placement sets.
 *
 * Group cardinality is unrestricted by the grammar.
 */

placementGroup
    : GROUP
      qualifiedName
      placementGroupMode?
      LBRACE
      placementGroupMember*
      RBRACE
    ;


placementGroupMode
    : PACK
    | SPREAD
    | BALANCED
    | DENSE
    | SPARSE
    ;


placementGroupMember
    : placementSubject
      SEMICOLON
    ;


/* ============================================================================
 * 7. SYMBOLIC REGIONS
 * ============================================================================
 *
 * A region is a logical placement domain.
 *
 * It is not a topology definition.
 */

placementRegion
    : REGION
      qualifiedName
      LBRACE
      placementRegionItem*
      RBRACE
    ;


placementRegionItem
    : placementProperty
    | placementRequirement
    | placementConstraint
    | placementPreference
    | placementHint
    ;


/* ============================================================================
 * 8. PLACEMENT RELATIONSHIPS
 * ========================================================================== */

placementRelationship
    : placementAffinity
    | placementAntiAffinity
    | placementCoLocation
    | placementSeparation
    | placementNear
    | placementFar
    | placementAdjacency
    | placementAvoidance
    ;


placementAffinity
    : AFFINITY
      placementSubject
      WITH
      placementSubject
      SEMICOLON
    ;


placementAntiAffinity
    : ANTI_AFFINITY
      placementSubject
      WITH
      placementSubject
      SEMICOLON
    ;


placementCoLocation
    : COLOCATE
      placementSubject
      WITH
      placementSubject
      SEMICOLON
    ;


placementSeparation
    : SEPARATE
      placementSubject
      WITH
      placementSubject
      SEMICOLON
    ;


placementNear
    : NEAR
      placementSubject
      WITH
      placementSubject
      SEMICOLON
    ;


placementFar
    : FAR
      placementSubject
      WITH
      placementSubject
      SEMICOLON
    ;


placementAdjacency
    : ADJACENT
      placementSubject
      WITH
      placementSubject
      SEMICOLON
    ;


placementAvoidance
    : AVOID
      placementSubject
      WITH
      placementSubject
      SEMICOLON
    ;


/* ============================================================================
 * 9. REQUIREMENTS
 * ============================================================================
 *
 * Requirements consume the canonical expression language.
 *
 * Examples:
 *
 *     requires resource::memory >= required_memory;
 *     requires capability::quantum::measurement;
 *     requires topology::connected;
 *
 * No resource implementation is embedded here.
 */

placementRequirement
    : REQUIRES
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 10. CONSTRAINTS
 * ========================================================================== */

placementConstraint
    : CONSTRAINT
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 11. PREFERENCES
 * ========================================================================== */

placementPreference
    : PREFER
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 12. HINTS
 * ========================================================================== */

placementHint
    : HINT
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 13. POLICIES
 * ============================================================================
 *
 * Policy syntax is intentionally declarative.
 *
 * Policy evaluation belongs to the canonical policy semantic subsystem.
 */

placementPolicy
    : POLICY
      qualifiedName?
      LBRACE
      placementPolicyItem*
      RBRACE
    ;


placementPolicyItem
    : ALLOW
      expression
      SEMICOLON
    | FORBID
      expression
      SEMICOLON
    | PREFER
      expression
      SEMICOLON
    | REQUIRES
      expression
      SEMICOLON
    | CONSTRAINT
      expression
      SEMICOLON
    | HINT
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 14. REPLICATION
 * ============================================================================
 *
 * Replication cardinality is an expression.
 *
 * It may therefore depend on workload size, data size, availability,
 * policy, or another semantic quantity.
 */

placementReplication
    : REPLICATED
      qualifiedName
      placementReplicationCount?
      SEMICOLON
    ;


placementReplicationCount
    : expression
    ;


/* ============================================================================
 * 15. MOBILITY / MIGRATION
 * ============================================================================
 *
 * This grammar declares mobility intent.
 *
 * It does not execute migration.
 */

placementMigration
    : MIGRATABLE
      qualifiedName
      SEMICOLON
    | NON_MIGRATABLE
      qualifiedName
      SEMICOLON
    ;


/* ============================================================================
 * 16. ELASTICITY
 * ============================================================================
 *
 * Elasticity allows the realization to determine an appropriate placement
 * cardinality or expansion strategy subject to semantic requirements.
 */

placementElasticity
    : ELASTIC
      qualifiedName
      placementElasticityExpression?
      SEMICOLON
    ;


placementElasticityExpression
    : expression
    ;


/* ============================================================================
 * 17. TARGET REFERENCES
 * ============================================================================
 *
 * Target names are semantic target classes, not physical machine IDs.
 */

placementTarget
    : TARGET
      qualifiedName
      placementQualifierList?
      SEMICOLON
    ;


placementQualifierList
    : placementQualifier+
    ;


placementQualifier
    : REQUIRES
      expression
    | CONSTRAINT
      expression
    | PREFER
      expression
    | HINT
      expression
    ;


/* ============================================================================
 * 18. CAPABILITY REFERENCES
 * ========================================================================== */

placementCapability
    : CAPABILITY
      qualifiedName
      placementQualifierList?
      SEMICOLON
    ;


/* ============================================================================
 * 19. SCOPE
 * ============================================================================
 *
 * Scope values remain expressions so future scope models do not require new
 * lexical vocabulary.
 *
 * Examples:
 *
 *     scope local;
 *     scope cluster;
 *     scope execution::domain;
 */

placementScope
    : SCOPE
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 20. OPEN-WORLD PLACEMENT PROPERTIES
 * ============================================================================
 *
 * Examples:
 *
 *     placement::locality = execution_region;
 *     placement::priority = priority_value;
 *     vendor::placement::metric = metric_value;
 *
 * The parser does not maintain a finite property catalogue.
 */

placementProperty
    : qualifiedName
      ASSIGN
      expression
      SEMICOLON
    ;


/* ============================================================================
 * 21. OPTIONAL LIST BOUNDARIES
 * ========================================================================== */

placementItemList
    : placementItem*
    ;


placementSubjectList
    : placementSubject
      (COMMA placementSubject)*
    ;


placementQualifiedNameList
    : qualifiedName
      (COMMA qualifiedName)*
    ;


/* ============================================================================
 * 22. SEMANTIC INTEGRATION ADAPTERS
 * ============================================================================
 *
 * These aliases provide stable semantic naming boundaries without creating
 * duplicate expression/name implementations.
 */

placementCondition
    : expression
    ;


placementValue
    : expression
    ;


placementQuantity
    : expression
    ;


placementPredicate
    : expression
    ;


/* ============================================================================
 * 23. HARDWARE / RESOURCE INTEGRATION
 * ============================================================================
 *
 * Hardware placement consumes semantic resource/capability information.
 *
 * The following are intentionally NOT parser-level enumerations:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     TPU
 *     NPU
 *     DSP
 *     optical
 *     neuromorphic
 *     future accelerator
 *
 * They remain names/capabilities/resource classes.
 *
 * Therefore:
 *
 *     target::gpu
 *     target::qpu
 *     resource::memory
 *     capability::tensor::compute
 *
 * are represented by qualified names rather than a finite hardware catalogue.
 *
 *
 * ============================================================================
 * 24. TOPOLOGY INTEGRATION
 * ============================================================================
 *
 * Placement may reference topology semantics:
 *
 *     topology::region
 *     topology::connected
 *     topology::local
 *
 * but does not define:
 *
 *     nodes
 *     edges
 *     paths
 *     routing
 *     connectivity algorithms
 *
 * The topology grammar remains authoritative for topology declarations.
 *
 *
 * ============================================================================
 * 25. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Valid semantic references may include:
 *
 *     quantum::logical
 *     quantum::resource
 *     quantum::measurement
 *     quantum::region
 *
 * The grammar does not define quantum IDs or physical qubit allocation.
 *
 * Quantum realization remains downstream through:
 *
 *     semantic quantum model
 *          ->
 *     quantum::ir
 *          ->
 *     routing
 *          ->
 *     scheduling
 *          ->
 *     QEC/resilience
 *          ->
 *     ZQN
 *          ->
 *     HAL
 *
 *
 * ============================================================================
 * 26. HDL INTEGRATION
 * ============================================================================
 *
 * HDL semantic layers may consume:
 *
 *     placement::region
 *     placement::affinity
 *     placement::separation
 *     placement::resource
 *     placement::target
 *
 * Physical synthesis and placement algorithms remain downstream.
 *
 *
 * ============================================================================
 * 27. DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Distributed execution may consume:
 *
 *     placement groups
 *     regions
 *     affinity
 *     anti-affinity
 *     replication
 *     mobility
 *     elasticity
 *     locality
 *
 * The grammar does not define a maximum node or process count.
 *
 *
 * ============================================================================
 * 28. AI / DATA / HYBRID INTEGRATION
 * ============================================================================
 *
 * AI, data, classical, quantum and hybrid workloads can all use the same
 * placement model.
 *
 * Examples of semantic references:
 *
 *     model
 *     dataset
 *     tensor
 *     quantum::circuit
 *     classical::kernel
 *     accelerator
 *
 * These remain ordinary qualified semantic names.
 *
 *
 * ============================================================================
 * 29. EFFECT INTEGRATION
 * ============================================================================
 *
 * Placement declarations themselves are effect-free.
 *
 * However, downstream operations such as:
 *
 *     migration
 *     allocation
 *     remote execution
 *     device access
 *
 * may introduce effects.
 *
 * The semantic layer must attach those effects to the actual operation rather
 * than treating placement syntax itself as execution.
 *
 *
 * ============================================================================
 * 30. PROVENANCE INTEGRATION
 * ============================================================================
 *
 * Every placement semantic node should retain:
 *
 *     source span
 *     declaration identity
 *     property identity
 *     source ordering
 *     originating module
 *     compatibility/version context
 *
 * Provenance records are created by the frontend/compiler, not this grammar.
 *
 *
 * ============================================================================
 * 31. SECURITY BOUNDARY
 * ============================================================================
 *
 * Placement MUST NOT provide:
 *
 *     physical address access
 *     device opening
 *     driver loading
 *     DMA programming
 *     MMIO access
 *     privileged execution
 *     hardware enumeration
 *     filesystem access
 *     network access
 *
 * Security policy and authorization are enforced downstream.
 *
 *
 * ============================================================================
 * 32. DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no actions
 *     no semantic predicates
 *     no target-language code
 *     no runtime state
 *     no hardware discovery
 *     no filesystem access
 *     no network access
 *
 * Parsing therefore depends only on:
 *
 *     source
 *     lexer vocabulary
 *     grammar
 *     selected language/compatibility version
 *
 *
 * ============================================================================
 * 33. SCALABILITY
 * ============================================================================
 *
 * Every collection uses unbounded grammar repetition:
 *
 *     *
 *     +
 *
 * No grammar production defines an artificial maximum.
 *
 * This applies to:
 *
 *     placement declarations
 *     groups
 *     group members
 *     regions
 *     relationships
 *     requirements
 *     constraints
 *     preferences
 *     hints
 *     policies
 *     properties
 *     replicas
 *     selectors
 *     parameters
 *
 * Actual limits are implementation/resource limits, never language semantics.
 *
 *
 * ============================================================================
 * 34. ERROR BOUNDARIES
 * ============================================================================
 *
 * Parser:
 *     malformed syntax
 *
 * AST validation:
 *     malformed structural representation
 *
 * Semantic analysis:
 *     unresolved names
 *     invalid relationships
 *     contradictory declarations
 *
 * Resource analysis:
 *     unavailable resources
 *
 * Capability analysis:
 *     unavailable capabilities
 *
 * Policy analysis:
 *     prohibited realization
 *
 * Routing:
 *     connectivity/route failure
 *
 * Scheduling:
 *     temporal/resource scheduling failure
 *
 * HAL:
 *     physical realization failure
 *
 *
 * ============================================================================
 * 35. COMPATIBILITY
 * ============================================================================
 *
 * The canonical public declaration rule is:
 *
 *     placementDeclaration
 *
 * Existing hardware.g4 already dispatches this rule.
 *
 * No compatibility alias is created because there are no repository consumers
 * of the obsolete `hardwarePlacementDecl` rule.
 *
 * This avoids maintaining two public placement authorities.
 *
 *
 * ============================================================================
 * 36. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *   [x] canonical ZamaniLexer vocabulary is used;
 *   [x] canonical expression grammar is used;
 *   [x] canonical names grammar is used;
 *   [x] placementDeclaration matches hardware.g4;
 *   [x] no K_* token aliases remain;
 *   [x] no duplicate expression grammar exists;
 *   [x] no duplicate qualified-name grammar exists;
 *   [x] no fixed hardware limit exists;
 *   [x] no physical device IDs are encoded;
 *   [x] no physical addresses are encoded;
 *   [x] no vendor catalogue is encoded;
 *   [x] resource semantics remain downstream;
 *   [x] capability semantics remain downstream;
 *   [x] topology semantics remain downstream;
 *   [x] routing remains downstream;
 *   [x] scheduling remains downstream;
 *   [x] quantum::ir remains canonical;
 *   [x] HDL semantics remain downstream;
 *   [x] open-world properties are supported;
 *   [x] symbolic quantities are supported;
 *   [x] requirements are distinct from constraints;
 *   [x] constraints are distinct from preferences;
 *   [x] preferences are distinct from hints;
 *   [x] mobility is declarative;
 *   [x] replication is symbolic;
 *   [x] elasticity is symbolic;
 *   [x] safe Rust integration is preserved;
 *   [x] no embedded Rust exists;
 *   [x] no semantic predicates exist;
 *   [x] no runtime behavior exists;
 *   [ ] generated parser compilation passes;
 *   [ ] repository-wide grammar compilation passes;
 *   [ ] lexical tests pass;
 *   [ ] positive placement tests pass;
 *   [ ] negative placement tests pass;
 *   [ ] scalability tests pass;
 *   [ ] cross-domain tests pass;
 *   [ ] compatibility tests pass.
 *
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */
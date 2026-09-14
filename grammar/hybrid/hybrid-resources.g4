/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hybrid/hybrid-resources.g4
 *
 * Status:
 *     Production hybrid-resource composition grammar.
 *
 * Grammar:
 *     ANTLR4 parser grammar
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     No embedded Rust.
 *     No semantic predicates.
 *     No unsafe implementation.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns SOURCE-LEVEL RESOURCE REQUIREMENTS for HYBRID COMPUTATION.
 *
 * Hybrid computation may combine:
 *
 *     classical computation
 *     quantum computation
 *     accelerators
 *     HDL/hardware computation
 *     distributed computation
 *     future computational domains
 *
 * This grammar expresses what a hybrid program:
 *
 *     requires
 *     constrains
 *     prefers
 *     hints
 *     targets
 *     exposes as a capability
 *
 * without selecting a particular physical machine.
 *
 * ============================================================================
 * FUNDAMENTAL RULE
 * ============================================================================
 *
 * Zamani describes:
 *
 *     computation
 *     semantics
 *     intent
 *     requirements
 *     constraints
 *     capabilities
 *     preferences
 *     hints
 *
 * It does NOT make temporary hardware characteristics part of permanent
 * source semantics.
 *
 * Therefore this grammar MUST NOT encode universal:
 *
 *     CPU counts
 *     core counts
 *     thread counts
 *     GPU counts
 *     FPGA counts
 *     ASIC counts
 *     QPU counts
 *     qubit counts
 *     memory capacities
 *     device IDs
 *     topology
 *     addresses
 *     deployment locations
 *     network inventories
 *     accelerator inventories
 *
 * unless a specific hardware dialect explicitly makes such information part
 * of the programmer's intended hardware semantics.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * The language model is:
 *
 *     Program Once
 *          |
 *          v
 *     Stable semantic meaning
 *          |
 *          v
 *     Compile Once
 *          |
 *          v
 *     Target realization
 *          |
 *          v
 *     Available capabilities/resources
 *          |
 *          v
 *     Execute Everywhere / Anywhere
 *
 * "Forever" means the semantic representation remains versionable and
 * extensible as implementations evolve.
 *
 * It does NOT mean that a compiled binary is literally executable on every
 * future architecture without recompilation.
 *
 * This distinction is important:
 *
 *     source semantic portability
 *
 * is different from:
 *
 *     binary portability.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - hybrid resource requirement expressions;
 *     - hybrid capability requirements;
 *     - hybrid resource constraints;
 *     - hybrid resource preferences;
 *     - hybrid implementation hints;
 *     - hybrid target-class requirements;
 *     - hybrid resource-property requirements;
 *     - hybrid resource composition;
 *     - resource requirement grouping;
 *     - logical hybrid placement intent;
 *     - hybrid scalability intent.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - physical hardware discovery;
 *     - hardware inventory;
 *     - device selection;
 *     - physical allocation;
 *     - scheduling;
 *     - routing;
 *     - calibration;
 *     - optimization;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - runtime dispatch;
 *     - canonical quantum IR;
 *     - classical IR;
 *     - quantum gate syntax;
 *     - general type syntax;
 *     - general expression precedence;
 *     - general statement syntax;
 *     - lexer token definitions.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Source
 *       |
 *       v
 *     Lexer
 *       |
 *       v
 *     Grammar
 *       |
 *       v
 *     Frontend AST
 *       |
 *       v
 *     Semantic analysis
 *       |
 *       +-----------------------------+
 *       |                             |
 *       v                             v
 *     Requirements                 Capabilities
 *       |                             |
 *       +-------------+---------------+
 *                     |
 *                     v
 *               Canonical semantic IR
 *                     |
 *        +------------+-------------+
 *        |            |             |
 *        v            v             v
 *      Quantum     Classical      Hardware
 *       IR          IR/model       model
 *        |            |             |
 *        +------------+-------------+
 *                     |
 *                     v
 *              Optimization
 *                     |
 *                  Routing
 *                     |
 *                 Scheduling
 *                     |
 *             Runtime / deployment
 *
 * ============================================================================
 * CANONICAL QUANTUM BOUNDARY
 * ============================================================================
 *
 * Quantum resource semantics MUST eventually integrate with:
 *
 *     quantum::ir
 *
 * This file MUST NOT define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     QuantumGate
 *     QuantumInstruction
 *     QuantumCircuit
 *     QuantumRegister
 *
 * The existing quantum resource grammar remains responsible for quantum-
 * specific resource syntax. This file composes the hybrid meaning around it.
 *
 * ============================================================================
 * RESOURCE TAXONOMY
 * ============================================================================
 *
 * This grammar deliberately distinguishes:
 *
 *     resource
 *     capability
 *     requirement
 *     constraint
 *     preference
 *     hint
 *     target
 *     placement
 *
 * They have different semantic meanings.
 *
 * RESOURCE
 *     Something that can participate in realization.
 *
 * CAPABILITY
 *     Something a target can do or provide.
 *
 * REQUIREMENT
 *     A condition that MUST be satisfied.
 *
 * CONSTRAINT
 *     A restriction on valid realization.
 *
 * PREFERENCE
 *     An advisory desired property.
 *
 * HINT
 *     Non-authoritative implementation guidance.
 *
 * TARGET
 *     A logical execution class/environment.
 *
 * PLACEMENT
 *     A logical-to-resource realization concern.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * All lists use repetition operators.
 *
 * There are intentionally no rules such as:
 *
 *     exactlyFourResources
 *     maximumEightAccelerators
 *     maximumThirtyTwoQubits
 *     maximumOneThousandThreads
 *
 * Practical parser/compiler limits belong to implementation resource policy.
 *
 * ============================================================================
 * GRAMMAR DEPENDENCIES
 * ============================================================================
 *
 * The following concepts are consumed from canonical grammar layers:
 *
 *     expression
 *     typeExpression
 *     identifier
 *     qualifiedName
 *
 * This file MUST NOT redefine their semantics.
 *
 * If the repository's final composition grammar exposes a differently named
 * canonical rule, the composition layer MUST provide an adapter rather than
 * duplicating the underlying grammar here.
 *
 * ============================================================================
 */

parser grammar HybridResources;

options {
    tokenVocab = ZamaniLexer;
}

import Expressions, Types;


/*
 * ============================================================================
 * 1. ROOT
 * ============================================================================
 *
 * This is the standalone hybrid-resource composition entry point.
 *
 * The root program grammar decides where these constructs are legal.
 */
hybridResourceConstruct
    : hybridResourceDeclaration
    | hybridCapabilityDeclaration
    | hybridRequirementDeclaration
    | hybridConstraintDeclaration
    | hybridPreferenceDeclaration
    | hybridHintDeclaration
    | hybridTargetDeclaration
    | hybridPlacementDeclaration
    ;


/*
 * ============================================================================
 * 2. RESOURCE DECLARATION
 * ============================================================================
 *
 * Declares a LOGICAL resource role.
 *
 * It does not allocate a physical resource.
 *
 * Example:
 *
 *     resource accelerator compute;
 *
 *     resource quantum q;
 *
 *     resource classical host;
 *
 * The names are symbolic.
 */
hybridResourceDeclaration
    : RESOURCE
      hybridResourceKind
      identifier
      hybridResourcePropertyClause*
      SEMICOLON
    ;


hybridResourceKind
    : identifier
    ;


/*
 * ============================================================================
 * 3. RESOURCE PROPERTY CLAUSES
 * ============================================================================
 *
 * Properties describe requirements/preferences/constraints around a resource.
 *
 * They do not describe a discovered hardware inventory.
 */
hybridResourcePropertyClause
    : capacityClause
    | availabilityClause
    | performanceClause
    | latencyClause
    | reliabilityClause
    | energyClause
    | scalabilityClause
    | portabilityClause
    | precisionClause
    | connectivityClause
    | memoryClause
    ;


/*
 * ============================================================================
 * 4. CAPABILITY DECLARATION
 * ============================================================================
 *
 * A capability declaration defines a logical capability requirement/contract.
 *
 * Capability implementation belongs downstream.
 */
hybridCapabilityDeclaration
    : CAPABILITY
      hybridCapabilityName
      hybridCapabilityBody?
      SEMICOLON?
    ;


hybridCapabilityName
    : qualifiedName
    ;


hybridCapabilityBody
    : LBRACE
      hybridCapabilityMember*
      RBRACE
    ;


hybridCapabilityMember
    : capabilityProperty
    | capabilityRequirement
    | capabilityConstraint
    | capabilityParameter
    ;


capabilityProperty
    : identifier
      ASSIGN
      expression
      SEMICOLON
    ;


capabilityRequirement
    : REQUIRES
      expression
      SEMICOLON
    ;


capabilityConstraint
    : CONSTRAINED
      BY
      expression
      SEMICOLON
    ;


capabilityParameter
    : identifier
      COLON
      typeExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 5. REQUIREMENT DECLARATION
 * ============================================================================
 *
 * A requirement is mandatory.
 *
 * Example:
 *
 *     requires capability("quantum");
 *
 *     requires resource("memory") >= amount;
 *
 * The actual satisfiability check belongs to semantic analysis.
 */
hybridRequirementDeclaration
    : REQUIRES
      hybridRequirementExpression
      SEMICOLON
    ;


hybridRequirementExpression
    : hybridRequirementTerm
      (
          hybridRequirementOperator
          hybridRequirementTerm
      )*
    ;


hybridRequirementTerm
    : hybridCapabilityRequirement
    | hybridResourceRequirement
    | hybridTargetRequirement
    | hybridPropertyRequirement
    | LPAREN
      hybridRequirementExpression
      RPAREN
    ;


hybridRequirementOperator
    : AND
    | OR
    ;


/*
 * ============================================================================
 * 6. CAPABILITY REQUIREMENT
 * ============================================================================
 */

hybridCapabilityRequirement
    : CAPABILITY
      LPAREN
      expression
      RPAREN
    ;


/*
 * ============================================================================
 * 7. RESOURCE REQUIREMENT
 * ============================================================================
 */

hybridResourceRequirement
    : RESOURCE
      LPAREN
      expression
      RPAREN
    ;


/*
 * ============================================================================
 * 8. TARGET REQUIREMENT
 * ============================================================================
 *
 * A target is a class/semantic execution environment.
 *
 * It does not identify a physical device.
 *
 * Valid examples may include:
 *
 *     quantum
 *     classical
 *     accelerator
 *     distributed
 *     embedded
 *
 * Future target classes may be introduced without modifying the fundamental
 * resource model.
 */
hybridTargetRequirement
    : TARGET
      LPAREN
      expression
      RPAREN
    ;


/*
 * ============================================================================
 * 9. PROPERTY REQUIREMENT
 * ============================================================================
 *
 * Property names are intentionally symbolic.
 *
 * The semantic model determines which properties exist.
 */
hybridPropertyRequirement
    : PROPERTY
      LPAREN
      expression
      RPAREN
      comparisonOperator
      expression
    ;


/*
 * ============================================================================
 * 10. CONSTRAINT DECLARATION
 * ============================================================================
 *
 * Constraints restrict valid implementations.
 */
hybridConstraintDeclaration
    : CONSTRAINED
      BY
      hybridConstraintExpression
      SEMICOLON
    ;


hybridConstraintExpression
    : expression
    ;


/*
 * ============================================================================
 * 11. PREFERENCE DECLARATION
 * ============================================================================
 *
 * Preferences are advisory.
 *
 * A compiler/runtime MAY relax them when permitted by policy.
 */
hybridPreferenceDeclaration
    : PREFER
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 12. HINT DECLARATION
 * ============================================================================
 *
 * Hints are non-authoritative implementation guidance.
 */
hybridHintDeclaration
    : HINT
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 13. TARGET DECLARATION
 * ============================================================================
 *
 * Declares a logical target class.
 *
 * It does NOT bind the program to a physical machine.
 */
hybridTargetDeclaration
    : TARGET
      identifier
      targetSpecificationBody?
      SEMICOLON?
    ;


targetSpecificationBody
    : LBRACE
      targetSpecificationMember*
      RBRACE
    ;


targetSpecificationMember
    : targetCapabilityRequirement
    | targetResourceRequirement
    | targetConstraint
    | targetPreference
    ;


targetCapabilityRequirement
    : REQUIRES
      CAPABILITY
      LPAREN
      expression
      RPAREN
      SEMICOLON
    ;


targetResourceRequirement
    : REQUIRES
      RESOURCE
      LPAREN
      expression
      RPAREN
      SEMICOLON
    ;


targetConstraint
    : CONSTRAINED
      BY
      expression
      SEMICOLON
    ;


targetPreference
    : PREFER
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 14. PLACEMENT DECLARATION
 * ============================================================================
 *
 * Placement describes LOGICAL placement intent.
 *
 * It is deliberately not physical routing.
 *
 * Example:
 *
 *     placement computation
 *         near communication_peer
 *         prefer locality;
 *
 * The routing/scheduling/hardware layers decide the realization.
 */
hybridPlacementDeclaration
    : PLACEMENT
      hybridPlacementSubject
      hybridPlacementClause*
      SEMICOLON
    ;


hybridPlacementSubject
    : expression
    ;


hybridPlacementClause
    : placementNearClause
    | placementLocalityClause
    | placementPreferenceClause
    | placementConstraintClause
    ;


placementNearClause
    : NEAR
      expression
    ;


placementLocalityClause
    : LOCALITY
      expression
    ;


placementPreferenceClause
    : PREFER
      expression
    ;


placementConstraintClause
    : CONSTRAINED
      BY
      expression
    ;


/*
 * ============================================================================
 * 15. RESOURCE PROPERTY EXPRESSIONS
 * ============================================================================
 *
 * These rules provide semantic names for common resource dimensions.
 *
 * The actual property values remain expressions.
 *
 * No unit/capacity maximum is encoded.
 */
capacityClause
    : CAPACITY
      comparisonOperator
      expression
    ;


availabilityClause
    : AVAILABILITY
      comparisonOperator
      expression
    ;


performanceClause
    : PERFORMANCE
      comparisonOperator
      expression
    ;


latencyClause
    : LATENCY
      comparisonOperator
      expression
    ;


reliabilityClause
    : RELIABILITY
      comparisonOperator
      expression
    ;


energyClause
    : ENERGY
      comparisonOperator
      expression
    ;


scalabilityClause
    : SCALABILITY
      comparisonOperator
      expression
    ;


portabilityClause
    : PORTABILITY
      comparisonOperator
      expression
    ;


precisionClause
    : PRECISION
      comparisonOperator
      expression
    ;


connectivityClause
    : CONNECTIVITY
      comparisonOperator
      expression
    ;


memoryClause
    : MEMORY
      comparisonOperator
      expression
    ;


/*
 * ============================================================================
 * 16. COMPARISON OPERATORS
 * ============================================================================
 *
 * These are structural operators.
 *
 * Their semantic legality belongs to type/semantic analysis.
 */
comparisonOperator
    : EQ
    | NEQ
    | LT
    | LTE
    | GT
    | GTE
    ;


/*
 * ============================================================================
 * 17. CANONICAL IDENTIFIER ADAPTER
 * ============================================================================
 *
 * This rule is intentionally tiny.
 *
 * If the repository exposes `identifier` from the canonical expression/core
 * grammar, this delegates to that rule.
 */
identifier
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * 18. CANONICAL QUALIFIED NAME ADAPTER
 * ============================================================================
 *
 * Symbolic resource/capability/target names may span namespaces.
 *
 * There is no finite depth limit.
 */
qualifiedName
    : identifier
      (
          DOUBLE_COLON
          identifier
      )*
    ;


/*
 * ============================================================================
 * 19. COMPLETION / INTEGRATION CONTRACT
 * ============================================================================
 *
 * REQUIRED LEXER VOCABULARY
 * -------------------------
 *
 * The canonical lexer must own the keywords represented by symbolic tokens
 * above, including the vocabulary for:
 *
 *     resource
 *     capability
 *     requires
 *     constrained
 *     by
 *     prefer
 *     hint
 *     target
 *     property
 *     placement
 *     near
 *     locality
 *     capacity
 *     availability
 *     performance
 *     latency
 *     reliability
 *     energy
 *     scalability
 *     portability
 *     precision
 *     connectivity
 *     memory
 *
 * If the repository's canonical lexer instead uses a different token naming
 * convention, this grammar must follow that existing convention.
 *
 * No lexer rules belong in this parser grammar.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST should preserve the distinction between:
 *
 *     ResourceDeclaration
 *     CapabilityDeclaration
 *     Requirement
 *     Constraint
 *     Preference
 *     Hint
 *     TargetRequirement
 *     PlacementIntent
 *     ResourceProperty
 *
 * These MUST NOT be collapsed into one generic "hardware constraint" node.
 *
 * Source locations MUST be retained by the frontend for diagnostics.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis MUST determine:
 *
 *     whether resource names exist;
 *     whether capability names exist;
 *     whether properties are valid;
 *     whether expressions have compatible types;
 *     whether requirements are satisfiable;
 *     whether constraints are contradictory;
 *     whether preferences are relaxable;
 *     whether hints are permitted;
 *     whether placement intent is realizable;
 *     whether target capabilities satisfy requirements.
 *
 * The grammar does not perform these checks.
 *
 * ============================================================================
 * HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware discovery MUST occur downstream.
 *
 * Conceptually:
 *
 *     source requirement
 *          |
 *          v
 *     semantic requirement
 *          |
 *          v
 *     hardware capability model
 *          |
 *          v
 *     available resources
 *          |
 *          v
 *     target selection
 *
 * This grammar MUST NOT query hardware.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum requirements may be expressed here at the HYBRID level.
 *
 * Quantum-specific resource concepts remain owned by:
 *
 *     grammar/quantum/quantum-resources.g4
 *
 * and eventually map through:
 *
 *     frontend semantic model
 *           |
 *           v
 *     quantum::ir
 *
 * No quantum resource IDs are created here.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical resource requirements remain target-independent.
 *
 * For example, a source program may express a computational requirement
 * without specifying a fixed CPU/core/thread count.
 *
 * Compiler/runtime policy may later determine:
 *
 *     scalar execution
 *     vector execution
 *     multicore execution
 *     GPU execution
 *     distributed execution
 *
 * without changing source semantics.
 *
 * ============================================================================
 * ACCELERATOR INTEGRATION
 * ============================================================================
 *
 * Accelerator interoperability is composed through symbolic capabilities and
 * requirements.
 *
 * This file MUST NOT duplicate:
 *
 *     accelerator invocation syntax
 *     accelerator interface syntax
 *     accelerator operation syntax
 *
 * Those belong to:
 *
 *     grammar/hybrid/accelerator-interoperability.g4
 *
 * The two grammars integrate at the semantic requirement/capability layer.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware/HDL grammars may consume resource/capability semantics produced
 * here.
 *
 * This grammar does not own:
 *
 *     ports
 *     wires
 *     clocks
 *     registers
 *     pipelines
 *     hardware modules
 *
 * Those remain HDL/hardware concerns.
 *
 * ============================================================================
 * SCHEDULING INTEGRATION
 * ============================================================================
 *
 * Scheduling consumes semantic resource requirements and constraints.
 *
 * This file MUST NOT express:
 *
 *     cycle numbers
 *     pulse times
 *     schedule slots
 *     machine-specific timing grids
 *
 * Scheduling owns those concerns.
 *
 * ============================================================================
 * ROUTING INTEGRATION
 * ============================================================================
 *
 * Placement intent MAY be consumed by routing.
 *
 * Routing determines the actual realization.
 *
 * The source grammar must not become a physical topology language.
 *
 * ============================================================================
 * OPTIMIZATION INTEGRATION
 * ============================================================================
 *
 * Preferences and hints may inform optimization.
 *
 * Optimization MUST NOT violate mandatory requirements or program semantics.
 *
 * ============================================================================
 * QEC / ZQN / RESILIENCE INTEGRATION
 * ============================================================================
 *
 * This file does not define:
 *
 *     error correction;
 *     noise;
 *     faults;
 *     mitigation;
 *     recovery;
 *     resilience policy.
 *
 * Where resource/capability requirements concern fault tolerance, they are
 * represented semantically and consumed by the corresponding subsystem.
 *
 * ============================================================================
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime MAY evaluate dynamic resource availability.
 *
 * If resources are unavailable, runtime/compiler policy may:
 *
 *     wait
 *     retry
 *     choose another valid realization
 *     scale down
 *     scale up
 *     migrate
 *     defer
 *     fail explicitly
 *
 * It MUST NOT silently change mandatory source semantics.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing must be deterministic for the same token stream.
 *
 * No external state may affect parsing.
 *
 * Resource discovery is deliberately excluded from parsing so that:
 *
 *     same source + same tokens
 *
 * always produces the same syntax tree.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Expressions in this grammar are data/syntax.
 *
 * They MUST NOT cause:
 *
 *     filesystem access
 *     network access
 *     process execution
 *     environment mutation
 *     hardware access
 *
 * Evaluation belongs to a controlled semantic/compiler/runtime subsystem.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar has no language-level finite resource-count ceiling.
 *
 * These are intentionally absent:
 *
 *     MAX_RESOURCES
 *     MAX_ACCELERATORS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_THREADS
 *     MAX_MEMORY
 *
 * Repeated structures are represented with `*` and `+`.
 *
 * "Infinity" therefore means:
 *
 *     no arbitrary grammar-imposed finite ceiling.
 *
 * Actual execution remains limited by:
 *
 *     available resources
 *     representation
 *     implementation
 *     operating system
 *     runtime
 *     hardware
 *     physical constraints
 *     execution policy
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Adding a new resource property should normally be possible through the
 * canonical property vocabulary without changing existing resource semantics.
 *
 * New target classes MUST NOT require a new fixed enumeration in this grammar
 * merely because a new machine architecture appears.
 *
 * Vendor-specific constructs belong in dialects.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive tests MUST cover:
 *
 *     resource declaration
 *     capability declaration
 *     requirement declaration
 *     capability requirement
 *     resource requirement
 *     target requirement
 *     property requirement
 *     constraint
 *     preference
 *     hint
 *     target declaration
 *     placement declaration
 *     compound requirements
 *     arbitrarily long requirement lists
 *     namespaced resources
 *
 * Negative tests MUST cover:
 *
 *     malformed requirements
 *     missing resource names
 *     malformed comparisons
 *     malformed capability expressions
 *     invalid declaration termination
 *     malformed placement
 *     malformed target declarations
 *
 * Boundary tests MUST verify:
 *
 *     one resource
 *     many resources
 *     deeply namespaced resources
 *     large requirement expressions
 *     large property expressions
 *     large source programs
 *
 * Scalability tests MUST verify absence of grammar-level assumptions about:
 *
 *     CPU count
 *     GPU count
 *     FPGA count
 *     accelerator count
 *     qubit count
 *     node count
 *     memory size
 *
 * Cross-domain tests MUST include:
 *
 *     classical + quantum
 *     classical + accelerator
 *     quantum + accelerator
 *     quantum + HDL
 *     classical + HDL
 *     quantum + distributed
 *     classical + quantum + accelerator
 *     classical + quantum + HDL + accelerator
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     1. It compiles as an ANTLR4 parser grammar.
 *
 *     2. All referenced lexer tokens exist in the canonical lexer.
 *
 *     3. Imported expression/type rules resolve through the canonical grammar
 *        composition layer.
 *
 *     4. No duplicate canonical expression/type system is introduced.
 *
 *     5. No physical hardware selection is performed.
 *
 *     6. No fixed scalable-resource maximum exists.
 *
 *     7. AST mapping exists for every construct.
 *
 *     8. Semantic analysis has defined ownership for every construct.
 *
 *     9. Hardware capability/resource models consume the semantic output.
 *
 *    10. Quantum requirements can ultimately reach `quantum::ir` without this
 *        grammar creating a competing quantum representation.
 *
 *    11. Scheduling consumes constraints without this grammar owning schedule
 *        realization.
 *
 *    12. Routing consumes placement intent without this grammar owning physical
 *        topology.
 *
 *    13. Optimization can consume preferences/hints without treating them as
 *        mandatory semantics.
 *
 *    14. Runtime can react to resource availability without changing program
 *        meaning.
 *
 *    15. Positive, negative, boundary, cross-domain, determinism and
 *        scalability tests pass.
 *
 *    16. Rust 1.97 / 1.97.1 integration builds without `unsafe` Rust.
 *
 * ============================================================================
 */
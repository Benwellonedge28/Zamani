/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/resources/resources.g4
 *
 * GRAMMAR
 * -------
 * parser grammar Resources
 *
 * STATUS
 * ------
 * CANONICAL RESOURCE ORCHESTRATOR
 *
 * PURPOSE
 * -------
 * This file is the SINGLE source-level orchestration grammar for the
 * `grammar/resources/` subsystem.
 *
 * It composes the independent resource grammar components into one coherent
 * resource-language surface.
 *
 * This file is intentionally the ORCHESTRATOR.
 *
 * It is therefore responsible for:
 *
 *   - the universal resource entry point;
 *   - resource-item dispatch;
 *   - resource declaration composition;
 *   - universal resource clause composition;
 *   - integration of specialized resource components;
 *   - stable parser-level ownership boundaries.
 *
 * It is NOT responsible for:
 *
 *   - lexical definitions;
 *   - identifiers;
 *   - general expressions;
 *   - type definitions;
 *   - hardware discovery;
 *   - physical allocation;
 *   - placement algorithms;
 *   - scheduling;
 *   - routing;
 *   - optimization;
 *   - QEC;
 *   - ZQN;
 *   - HAL;
 *   - runtime resource management;
 *   - backend implementation;
 *   - resource availability computation.
 *
 * ============================================================================
 * IMPLEMENTATION BASELINE
 * ============================================================================
 *
 * ANTLR4 parser grammar
 * Rust 2021
 * Rust 1.97 / Rust 1.97.1
 *
 * This grammar contains:
 *
 *   - no embedded Rust;
 *   - no parser actions;
 *   - no semantic predicates;
 *   - no unsafe code;
 *   - no filesystem access;
 *   - no network access;
 *   - no hardware access.
 *
 * The generated parser must remain usable by the safe-Rust frontend.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                     Zamani source
 *                           |
 *                           v
 *                      ZamaniLexer
 *                           |
 *                           v
 *                    parser composition
 *                           |
 *                           v
 *              +-------------------------+
 *              |       Resources         |
 *              |   THIS ORCHESTRATOR     |
 *              +-------------------------+
 *                           |
 *       +-------------------+-------------------+
 *       |                   |                   |
 *       v                   v                   v
 * ResourceExpressions     Names       specialized resource
 *       |                                      grammars
 *       |                                           |
 *       +-------------------+-----------------------+
 *                           |
 *                           v
 *                    Domain-neutral AST
 *                           |
 *                           v
 *                  structural validation
 *                           |
 *                           v
 *                    semantic analysis
 *                           |
 *             +-------------+-------------+
 *             |             |             |
 *             v             v             v
 *         resources     capabilities   constraints
 *             |             |             |
 *             +-------------+-------------+
 *                           |
 *                           v
 *                  canonical semantic model
 *                           |
 *             +-------------+-------------+
 *             |             |             |
 *             v             v             v
 *         classical     quantum::ir    HDL/hardware
 *             |             |             |
 *             +-------------+-------------+
 *                           |
 *                           v
 *                   optimization/lowering
 *                           |
 *              +------------+------------+
 *              |            |            |
 *              v            v            v
 *           routing     scheduling   resilience
 *              |            |            |
 *              +------------+------------+
 *                           |
 *                          ZQN
 *                           |
 *                          HAL
 *                           |
 *                    target realization
 *
 * ============================================================================
 * AUTHORITY
 * ============================================================================
 *
 * Normative language meaning:
 *
 *     grammar/specification/
 *
 * Focused resource contracts:
 *
 *     grammar/spec/
 *
 * Canonical syntax:
 *
 *     grammar/Zamani.g4
 *     grammar/resources/resources.g4
 *
 * Resource expression foundation:
 *
 *     grammar/resources/resource-expressions.g4
 *
 * Name foundation:
 *
 *     grammar/core/names.g4
 *
 * Current implementation conformance:
 *
 *     grammar/grammar.md
 *
 * Historical / extended design:
 *
 *     grammar/Zamani-Grammar.md
 *
 * This file MUST NOT become a second semantic or IR authority.
 *
 * ============================================================================
 * SINGLE-SOURCE-OF-TRUTH RULE
 * ============================================================================
 *
 * THIS FILE OWNS THE UNIVERSAL RESOURCE COMPOSITION BOUNDARY.
 *
 * It owns the resource-language entry points that other resource grammars
 * consume.
 *
 * Specialized resource grammars own their specialized internal syntax.
 *
 * A specialized grammar MUST NOT redefine a rule owned here.
 *
 * In particular, specialized grammars MUST NOT independently redefine:
 *
 *     resources
 *     resourceItem
 *     resourceDeclaration
 *     resourceKindClause
 *     resourceSpecification
 *     resourceClause
 *     resourceRequirement
 *     resourceConstraint
 *     resourcePreference
 *     resourceHint
 *     resourceCapability
 *     resourceTarget
 *     resourceReservation
 *     resourceAcquisition
 *     resourceRelease
 *     resourceGroup
 *     resourceContract
 *     resourceProfile
 *
 * If a specialized grammar currently declares one of these names, it must be
 * converted into a delegate/component whose public entry rule has a unique
 * specialized name.
 *
 * This avoids ANTLR imported-rule collisions and prevents parallel resource
 * language authorities.
 *
 * ============================================================================
 * IMPORT POLICY
 * ============================================================================
 *
 * ResourceExpressions and Names are foundational dependencies.
 *
 * Specialized resource grammars are imported only when their public entry
 * rules are independent of the universal rules owned by this file.
 *
 * The intended dependency direction is:
 *
 *     Names
 *       |
 *       v
 *     Expressions
 *       |
 *       v
 *     ResourceExpressions
 *       |
 *       +-------------------------------+
 *       |                               |
 *       v                               v
 * specialized resource grammars     Resources
 *                                       |
 *                                       v
 *                                Zamani composition root
 *
 * No child resource grammar may import this file.
 *
 * This prevents:
 *
 *     resources.g4 <-> child.g4
 *
 * import cycles.
 *
 * ============================================================================
 * OPEN-WORLD RESOURCE MODEL
 * ============================================================================
 *
 * Resource kinds are semantic names.
 *
 * The grammar MUST NOT enumerate a closed hardware vocabulary.
 *
 * Therefore this grammar does not establish a finite list of:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     accelerator
 *     memory device
 *     storage device
 *     network device
 *     node
 *     future architecture
 *
 * A resource kind is represented by source-level name syntax.
 *
 * Examples:
 *
 *     compute
 *     memory
 *     accelerator
 *     quantum::logical_qubit
 *     quantum::physical_qubit
 *     tensor::compute
 *     hardware::accelerator
 *     future::architecture::resource
 *
 * Semantic analysis determines whether a named resource kind exists and what
 * it means.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Resource syntax is part of:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * The source program expresses WHAT it needs.
 *
 * It does not implicitly express WHICH physical resource realizes it.
 *
 * Examples of portable intent:
 *
 *     requires qubits >= logical_qubits;
 *
 *     requires memory >= required_memory;
 *
 *     requires capability("quantum.measurement");
 *
 *     requires capability("tensor.compute");
 *
 *     requires nodes >= required_nodes;
 *
 *     prefer latency <= latency_budget;
 *
 * Such values are program semantics.
 *
 * They are NOT compiler-wide limits.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This grammar MUST NOT encode:
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
 *
 * It also MUST NOT encode equivalent indirect limits such as:
 *
 *     exactly 32 CPUs
 *     exactly 1024 qubits
 *     exactly 64 GB memory
 *     exactly 24 GB VRAM
 *     exactly 32-bit registers
 *     exactly N nodes
 *
 * Numeric values in source programs remain program values.
 *
 * For example:
 *
 *     requires qubits >= 1024;
 *
 * is valid semantic intent.
 *
 * It does not create:
 *
 *     MAX_QUBITS = 1024
 *
 * ============================================================================
 * UNBOUNDED LANGUAGE SCALABILITY
 * ============================================================================
 *
 * The grammar imposes no language-level maximum on:
 *
 *     resources
 *     resource items
 *     resource clauses
 *     requirements
 *     constraints
 *     capabilities
 *     preferences
 *     hints
 *     properties
 *     resource groups
 *     contracts
 *     profiles
 *     expressions
 *     name-path depth
 *     nested resource structures
 *
 * Repetition uses ANTLR's:
 *
 *     *
 *     +
 *
 * and recursive composition where appropriate.
 *
 * "Infinity" means:
 *
 *     no artificial language-level ceiling.
 *
 * It does not mean that physical hardware, memory, parser stacks, compiler
 * memory, operating-system limits, or deployment infrastructure are infinite.
 *
 * Those are implementation and target constraints.
 *
 * ============================================================================
 * RESOURCE SEMANTIC CATEGORIES
 * ============================================================================
 *
 * The resource subsystem distinguishes:
 *
 *     declaration
 *     requirement
 *     constraint
 *     capability
 *     preference
 *     hint
 *     target intent
 *     reservation
 *     acquisition
 *     release
 *     derivation
 *     group
 *     contract
 *     profile
 *
 * These concepts MUST NOT be collapsed.
 *
 * In particular:
 *
 *     requirement != capability
 *     capability != preference
 *     preference != hint
 *     requirement != implementation decision
 *     resource intent != physical allocation
 *
 * ============================================================================
 * RESOURCE VS HARDWARE
 * ============================================================================
 *
 * This grammar may describe:
 *
 *     compute
 *     memory
 *     storage
 *     accelerator
 *     quantum resources
 *     network resources
 *     interconnect
 *     energy
 *     power
 *     performance
 *     reliability
 *     resilience
 *
 * It does not choose:
 *
 *     CPU 0
 *     GPU 0
 *     FPGA 0
 *     QPU 0
 *     physical qubit 0
 *     memory bank 0
 *     network node 0
 *
 * Physical realization is downstream.
 *
 * ============================================================================
 * RESOURCE DECLARATION
 * ============================================================================
 *
 * A resource declaration introduces a symbolic resource concept.
 *
 * The name is logical unless a later, explicitly target-specific construct
 * states otherwise.
 *
 * Examples:
 *
 *     resource compute;
 *
 *     resource memory: memory;
 *
 *     resource accelerator: accelerator;
 *
 *     resource qpu: quantum::qpu;
 *
 * A declaration does not allocate a device.
 *
 * ============================================================================
 */

parser grammar Resources;

options {
    tokenVocab = ZamaniLexer;
}

import ResourceExpressions, Names;


/*
 * ============================================================================
 * UNIVERSAL RESOURCE ENTRY POINT
 * ============================================================================
 *
 * This is the single complete resource subsystem entry point.
 *
 * It deliberately contains no fixed number of items.
 * ============================================================================
 */

resources
    : resourceItem*
    ;


/*
 * ============================================================================
 * UNIVERSAL RESOURCE DISPATCH
 * ============================================================================
 *
 * Every complete resource construct enters through resourceItem.
 *
 * Specialized grammars integrate through their unique public entry rules.
 *
 * ============================================================================
 */

resourceItem
    : resourceDeclaration
    | resourceRequirement
    | resourceConstraint
    | resourcePreference
    | resourceHint
    | resourceCapability
    | resourceTarget
    | resourceReservation
    | resourceAcquisition
    | resourceRelease
    | resourceDerivation
    | resourceGroup
    | resourceContract
    | resourceProfile
    ;


/*
 * ============================================================================
 * RESOURCE DECLARATION
 * ============================================================================
 *
 * Syntax:
 *
 *     resource <name> [ : <kind> ] [ <specification> ] ;
 *
 * The symbolic resource name is not a physical identifier.
 * ============================================================================
 */

resourceDeclaration
    : resourceAttributes?
      RESOURCE
      identifier
      resourceKindClause?
      resourceSpecification?
      SEMICOLON
    ;


/*
 * ============================================================================
 * RESOURCE KIND
 * ============================================================================
 *
 * Resource kinds are open-world qualified semantic names.
 *
 * The semantic layer decides whether a kind is known, imported, provided by a
 * dialect, or supplied by a target capability registry.
 *
 * ============================================================================
 */

resourceKindClause
    : COLON resourceNamePath
    ;


resourceNamePath
    : resourceNameSegment
      (DOUBLE_COLON resourceNameSegment)*
    ;


/*
 * ============================================================================
 * RESOURCE NAME SEGMENT
 * ============================================================================
 *
 * Normally a resource kind is an identifier.
 *
 * Some Zamani domain words may already be reserved lexical tokens. They may
 * therefore be accepted as resource-name segments where the language's
 * lexical contract permits their use as semantic resource names.
 *
 * This list is intentionally kept minimal.
 *
 * It MUST NOT become a resource-kind enumeration.
 *
 * ============================================================================
 */

resourceNameSegment
    : identifier
    | QUANTUM
    | NANO
    | MEMORY
    | CAPABILITY
    | TARGET
    | RESOURCE
    | RESOURCES
    ;


/*
 * ============================================================================
 * RESOURCE SPECIFICATION
 * ============================================================================
 *
 * A specification is an unbounded collection of resource clauses.
 * ============================================================================
 */

resourceSpecification
    : LBRACE
      resourceBodyItem*
      RBRACE
    ;


resourceBodyItem
    : resourceAttributes?
      resourceClause
    ;


/*
 * ============================================================================
 * RESOURCE ATTRIBUTES
 * ============================================================================
 *
 * Attributes are metadata.
 *
 * They do not perform resource allocation.
 * ============================================================================
 */

resourceAttributes
    : resourceAttribute+
    ;


resourceAttribute
    : AT
      qualifiedName
      resourceAttributeArguments?
    ;


resourceAttributeArguments
    : LPAREN
      resourceExpressionList?
      RPAREN
    ;


/*
 * ============================================================================
 * UNIVERSAL RESOURCE CLAUSE
 * ============================================================================
 *
 * This is the principal internal dispatch boundary.
 *
 * Specialized resource domains are represented by unique specialized public
 * rules once their component grammars are normalized.
 *
 * ============================================================================
 */

resourceClause
    : resourceQuantityClause
    | resourceReferenceClause
    | resourceRequirementClause
    | resourceConstraintClause
    | resourcePreferenceClause
    | resourceHintClause
    | resourceCapabilityClause
    | resourceTargetClause
    | resourceCapacityClause
    | resourceAvailabilityClause
    | resourcePortabilityClause
    | resourceScalabilityClause
    | resourcePerformanceClause
    | resourceLatencyClause
    | resourceThroughputClause
    | resourceBandwidthClause
    | resourceEnergyClause
    | resourcePowerClause
    | resourceReliabilityClause
    | resourceResilienceClause
    | resourceCostClause
    | resourceReservationClause
    | resourceAcquisitionClause
    | resourceReleaseClause
    | resourceDerivationClause
    | resourceGroupClause
    | resourceContractClause
    | resourceProfileClause
    | resourcePropertyClause
    ;


/*
 * ============================================================================
 * GENERIC RESOURCE QUANTITY
 * ============================================================================
 *
 * A quantity is an expression.
 *
 * It may be:
 *
 *     literal
 *     symbolic
 *     computed
 *     generic
 *     dependent
 *     input-derived
 *     runtime-derived
 *
 * The grammar never evaluates it.
 * ============================================================================
 */

resourceQuantityClause
    : QUANTITY
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * RESOURCE REFERENCE
 * ============================================================================
 *
 * A resource reference identifies a logical resource expression.
 * ============================================================================
 */

resourceReferenceClause
    : REFERENCE
      ASSIGN
      resourceSelector
      SEMICOLON
    ;


/*
 * ============================================================================
 * REQUIREMENT
 * ============================================================================
 *
 * Requirements state what must be satisfied.
 *
 * They are not hardware-selection commands.
 * ============================================================================
 */

resourceRequirement
    : REQUIRES
      resourceRequirementExpression
      SEMICOLON
    ;


resourceRequirementExpression
    : resourceExpression
    ;


resourceRequirementClause
    : REQUIRES
      resourceRequirementExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * CONSTRAINT
 * ============================================================================
 *
 * Constraints state conditions that must hold.
 * ============================================================================
 */

resourceConstraint
    : CONSTRAINT
      resourceConstraintExpression
      SEMICOLON
    ;


resourceConstraintExpression
    : resourceExpression
    ;


resourceConstraintClause
    : CONSTRAINT
      resourceConstraintExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * PREFERENCE
 * ============================================================================
 *
 * Preferences influence target realization without becoming requirements.
 * ============================================================================
 */

resourcePreference
    : PREFER
      resourcePreferenceExpression
      SEMICOLON
    ;


resourcePreferenceExpression
    : resourceExpression
    ;


resourcePreferenceClause
    : PREFER
      resourcePreferenceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * HINT
 * ============================================================================
 *
 * Hints communicate non-binding implementation guidance.
 * ============================================================================
 */

resourceHint
    : HINT
      resourceHintExpression
      SEMICOLON
    ;


resourceHintExpression
    : resourceExpression
    ;


resourceHintClause
    : HINT
      resourceHintExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * CAPABILITY
 * ============================================================================
 *
 * Capability syntax remains open-world.
 *
 * The identity may refer to a compiler-known capability, target capability,
 * dialect capability, domain capability, or future capability.
 * ============================================================================
 */

resourceCapability
    : CAPABILITY
      resourceCapabilityValue
      SEMICOLON
    ;


resourceCapabilityValue
    : resourceExpression
    ;


resourceCapabilityClause
    : CAPABILITY
      resourceCapabilityCallArguments?
      resourceCapabilityValue
      SEMICOLON
    ;


resourceCapabilityCallArguments
    : LPAREN
      resourceExpressionList?
      RPAREN
    ;


/*
 * ============================================================================
 * TARGET INTENT
 * ============================================================================
 *
 * Target intent identifies an abstract execution domain.
 *
 * It does not select a physical device.
 * ============================================================================
 */

resourceTarget
    : TARGET
      resourceTargetExpression
      SEMICOLON
    ;


resourceTargetExpression
    : resourceTargetSymbol
    | resourceExpression
    ;


resourceTargetSymbol
    : identifier
    | qualifiedName
    ;


resourceTargetClause
    : TARGET
      resourceTargetExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * CAPACITY
 * ============================================================================
 */

resourceCapacityClause
    : CAPACITY
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * AVAILABILITY
 * ============================================================================
 *
 * Availability is a semantic condition.
 *
 * The parser does not inspect the actual machine.
 * ============================================================================
 */

resourceAvailabilityClause
    : AVAILABILITY
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * PORTABILITY
 * ============================================================================
 */

resourcePortabilityClause
    : PORTABILITY
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * Scaling remains semantic and unbounded.
 * ============================================================================
 */

resourceScalabilityClause
    : SCALABILITY
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * PERFORMANCE
 * ============================================================================
 */

resourcePerformanceClause
    : PERFORMANCE
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * LATENCY
 * ============================================================================
 */

resourceLatencyClause
    : LATENCY
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * THROUGHPUT
 * ============================================================================
 */

resourceThroughputClause
    : THROUGHPUT
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * BANDWIDTH
 * ============================================================================
 */

resourceBandwidthClause
    : BANDWIDTH
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * ENERGY
 * ============================================================================
 */

resourceEnergyClause
    : ENERGY
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * POWER
 * ============================================================================
 */

resourcePowerClause
    : POWER
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * RELIABILITY
 * ============================================================================
 */

resourceReliabilityClause
    : RELIABILITY
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * RESILIENCE
 * ============================================================================
 *
 * Resilience states and outcomes remain semantic vocabulary.
 *
 * This grammar does not perform recovery.
 * ============================================================================
 */

resourceResilienceClause
    : RESILIENCE
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * COST
 * ============================================================================
 */

resourceCostClause
    : COST
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * RESERVATION
 * ============================================================================
 *
 * Reservation syntax is declarative intent.
 * ============================================================================
 */

resourceReservation
    : RESERVE
      resourceExpression
      SEMICOLON
    ;


resourceReservationClause
    : RESERVATION
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * ACQUISITION
 * ============================================================================
 */

resourceAcquisition
    : ACQUIRE
      resourceExpression
      SEMICOLON
    ;


resourceAcquisitionClause
    : ACQUISITION
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * RELEASE
 * ============================================================================
 */

resourceRelease
    : RELEASE
      resourceExpression
      SEMICOLON
    ;


resourceReleaseClause
    : RELEASE_KW
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * DERIVATION
 * ============================================================================
 *
 * Resource relationships may be derived from program expressions.
 * ============================================================================
 */

resourceDerivation
    : DERIVE
      resourceExpression
      SEMICOLON
    ;


resourceDerivationClause
    : DERIVE
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * RESOURCE GROUP
 * ============================================================================
 */

resourceGroup
    : RESOURCE_GROUP
      resourceGroupClauseBody
      SEMICOLON
    ;


resourceGroupClause
    : RESOURCE_GROUP
      resourceGroupClauseBody
      SEMICOLON
    ;


resourceGroupClauseBody
    : resourceExpression
    ;


/*
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * A contract is a declarative collection of resource semantics.
 * ============================================================================
 */

resourceContract
    : CONTRACT
      resourceContractClauseBody
      SEMICOLON
    ;


resourceContractClause
    : CONTRACT
      resourceContractClauseBody
      SEMICOLON
    ;


resourceContractClauseBody
    : resourceExpression
    ;


/*
 * ============================================================================
 * RESOURCE PROFILE
 * ============================================================================
 */

resourceProfile
    : PROFILE
      resourceProfileClauseBody
      SEMICOLON
    ;


resourceProfileClause
    : PROFILE
      resourceProfileClauseBody
      SEMICOLON
    ;


resourceProfileClauseBody
    : resourceExpression
    ;


/*
 * ============================================================================
 * GENERIC PROPERTY
 * ============================================================================
 *
 * Property names remain open-world.
 *
 * The grammar does not enumerate:
 *
 *     cpu_count
 *     gpu_count
 *     qubit_count
 *     memory_gb
 *     register_width
 *     tensor_rank
 *
 * Such properties are semantic data.
 * ============================================================================
 */

resourcePropertyClause
    : qualifiedName
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * SPECIALIZED COMPONENT INTEGRATION
 * ============================================================================
 *
 * The following resource components exist as independent files and MUST be
 * integrated without duplicating universal ownership:
 *
 *     budgets.g4
 *     capabilities.g4
 *     constraints.g4
 *     energy.g4
 *     hints.g4
 *     latency.g4
 *     negotiation.g4
 *     performance.g4
 *     placement.g4
 *     portability.g4
 *     preferences.g4
 *     reliability.g4
 *     requirements.g4
 *     resource-expressions.g4
 *     resource.g4
 *     scalability.g4
 *
 * The integration invariant is:
 *
 *     Resources
 *        |
 *        +--> universal entry points
 *        |
 *        +--> specialized component entry points
 *
 * Specialized files MUST expose unique public entry rules.
 *
 * They MUST NOT redefine a universal rule owned above.
 *
 * ============================================================================
 * COMPONENT OWNERSHIP CONTRACT
 * ============================================================================
 *
 * budgets.g4
 *
 *     Owns specialized resource-budget syntax.
 *
 * capabilities.g4
 *
 *     Owns detailed capability composition/predicate syntax.
 *
 * constraints.g4
 *
 *     Owns detailed constraint forms.
 *
 * energy.g4
 *
 *     Owns detailed energy-domain syntax.
 *
 * hints.g4
 *
 *     Owns detailed hint metadata and grouping.
 *
 * latency.g4
 *
 *     Owns detailed latency syntax.
 *
 * negotiation.g4
 *
 *     Owns resource negotiation syntax.
 *
 * performance.g4
 *
 *     Owns detailed performance objectives and metrics.
 *
 * placement.g4
 *
 *     Owns abstract placement intent.
 *
 * portability.g4
 *
 *     Owns portability contracts.
 *
 * preferences.g4
 *
 *     Owns detailed preference syntax.
 *
 * reliability.g4
 *
 *     Owns detailed reliability syntax.
 *
 * requirements.g4
 *
 *     Owns detailed requirement expressions.
 *
 * resource-expressions.g4
 *
 *     Owns resource-expression composition.
 *
 * resource.g4
 *
 *     Owns singular reusable ResourceIntent syntax only.
 *
 * scalability.g4
 *
 *     Owns detailed scaling declarations and relationships.
 *
 * None of these components owns the complete resource subsystem.
 *
 * ============================================================================
 * RESOURCE-INTENT INTEGRATION
 * ============================================================================
 *
 * `resource.g4` is intentionally a leaf component.
 *
 * Its grammar identity MUST be unique, e.g.:
 *
 *     ResourceIntent
 *
 * It MUST NOT declare:
 *
 *     Resources
 *
 * It MUST NOT define:
 *
 *     resources
 *     resourceItem
 *     resourceDeclaration
 *
 * The orchestrator remains the owner of those rules.
 *
 * ============================================================================
 * EXPRESSION INTEGRATION
 * ============================================================================
 *
 * Every resource-valued expression must ultimately flow through:
 *
 *     resourceExpression
 *
 * from:
 *
 *     resource-expressions.g4
 *
 * Resource grammars MUST NOT introduce a competing arithmetic or logical
 * expression language.
 *
 * General expressions remain owned by:
 *
 *     grammar/expressions/
 *
 * ============================================================================
 * NAME INTEGRATION
 * ============================================================================
 *
 * Names are imported from:
 *
 *     grammar/core/names.g4
 *
 * This file MUST NOT redefine:
 *
 *     identifier
 *     qualifiedName
 *
 * ============================================================================
 * TYPE INTEGRATION
 * ============================================================================
 *
 * Resource declarations may refer semantically to resource-related types.
 *
 * Type syntax remains owned by:
 *
 *     grammar/types/
 *
 * This grammar must not duplicate:
 *
 *     typeExpression
 *     genericType
 *     resourceType
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical resource requirements may express:
 *
 *     compute
 *     memory
 *     storage
 *     vectorization
 *     parallelism
 *     accelerator capability
 *     latency
 *     throughput
 *
 * The resource grammar does not decide whether a CPU, GPU, accelerator, or
 * other target realizes those requirements.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum resource intent may express semantic requirements such as:
 *
 *     quantum::logical_qubit
 *     quantum::physical_qubit
 *     quantum::measurement
 *     quantum::dynamic_control
 *     quantum::error_correction
 *     quantum::coherence
 *
 * No physical qubit numbering is encoded.
 *
 * No fixed qubit count is encoded.
 *
 * No gate list is encoded.
 *
 * No coupling topology is encoded.
 *
 * Quantum semantics ultimately cross the canonical:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * This grammar MUST NOT create:
 *
 *     QuantumResourceIR
 *     ResourceIR
 *     QuantumCapabilityIR
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Resource syntax may express:
 *
 *     accelerator
 *     programmable logic
 *     memory
 *     interconnect
 *     timing
 *     power
 *     reliability
 *
 * HDL syntax remains responsible for:
 *
 *     modules
 *     ports
 *     signals
 *     nets
 *     registers
 *     pipelines
 *     timing
 *     state machines
 *
 * Hardware realization remains downstream.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * Resource syntax may express:
 *
 *     nodes
 *     communication
 *     bandwidth
 *     locality
 *     replication
 *     storage
 *     availability
 *
 * No fixed node count is encoded.
 *
 * ============================================================================
 * AI / DATA INTEGRATION
 * ============================================================================
 *
 * Resource expressions may depend on:
 *
 *     workload size
 *     tensor shape
 *     model size
 *     dataset size
 *     batch size
 *     accelerator capability
 *
 * These are semantic values rather than compiler hardware limits.
 *
 * ============================================================================
 * RESILIENCE INTEGRATION
 * ============================================================================
 *
 * Resource semantics may participate in resilience analysis.
 *
 * Supported semantic states include:
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
 * Supported semantic outcomes include:
 *
 *     ACCEPT
 *     DEGRADED_ACCEPT
 *     RETRY
 *     RECOVER
 *     ESCALATE
 *     REJECT
 *
 * The grammar only carries declarative information.
 *
 * Recovery remains a downstream responsibility.
 *
 * ============================================================================
 * TARGET INDEPENDENCE
 * ============================================================================
 *
 * A resource declaration is target-independent unless an explicit
 * target-specific language construct is used.
 *
 * The compiler may later resolve:
 *
 *     resource intent
 *         ->
 *     available capabilities
 *         ->
 *     target candidates
 *         ->
 *     resource realization
 *         ->
 *     placement
 *         ->
 *     scheduling
 *         ->
 *     execution
 *
 * This separation is fundamental to POCO-REAF.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Resource syntax maps to the domain-neutral frontend AST.
 *
 * The AST must preserve, where applicable:
 *
 *     resource kind
 *     resource name
 *     resource expression
 *     requirement
 *     constraint
 *     capability
 *     preference
 *     hint
 *     target intent
 *     property
 *     source span
 *     attributes
 *
 * The AST MUST NOT contain:
 *
 *     physical device handles
 *     physical addresses
 *     scheduler state
 *     routing state
 *     calibration state
 *     QEC state
 *     ZQN state
 *     HAL handles
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     name resolution
 *     kind resolution
 *     expression typing
 *     capability resolution
 *     requirement checking
 *     constraint checking
 *     preference preservation
 *     hint preservation
 *     resource relationship checking
 *     target compatibility
 *     satisfiability
 *     availability analysis
 *
 * Syntax alone must not determine target availability.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates no IR.
 *
 * Resource syntax is lowered through the canonical semantic model.
 *
 * Resource information may contribute to:
 *
 *     classical IR
 *     quantum::ir
 *     HDL/hardware semantic representation
 *     distributed execution representation
 *
 * according to the semantic domain.
 *
 * There is no:
 *
 *     ResourceIR
 *
 * created merely because this grammar exists.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * The compiler consumes semantic resource intent to determine whether a
 * candidate target can satisfy the program.
 *
 * It may perform:
 *
 *     capability discovery
 *     resource planning
 *     target selection
 *     placement
 *     lowering
 *     scheduling
 *     routing
 *     resilience planning
 *
 * Those operations are downstream and must not alter source-level meaning.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime systems may inspect resource availability and execute the compiled
 * realization.
 *
 * Runtime state must never be used to reinterpret the source grammar.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing must depend only on:
 *
 *     source text
 *     lexer vocabulary
 *     grammar composition
 *     grammar version
 *     explicitly selected dialect
 *
 * Parsing must NOT depend on:
 *
 *     CPU count
 *     GPU count
 *     QPU availability
 *     filesystem state
 *     network state
 *     current time
 *     randomness
 *     runtime resource state
 *     hardware discovery
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Syntax diagnostics belong to the parser.
 *
 * Semantic diagnostics include:
 *
 *     unknown resource kind
 *     invalid resource property
 *     invalid expression type
 *     unsatisfied requirement
 *     unsupported capability
 *     incompatible constraint
 *     unavailable target
 *
 * Runtime diagnostics include:
 *
 *     resource exhaustion
 *     runtime unavailability
 *     deployment failure
 *     recovery failure
 *
 * These categories must not be conflated.
 *
 * ============================================================================
 * SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * Source spans must remain recoverable for:
 *
 *     resource declarations
 *     resource names
 *     kinds
 *     requirements
 *     constraints
 *     capabilities
 *     preferences
 *     hints
 *     target expressions
 *     properties
 *     values
 *
 * This supports:
 *
 *     diagnostics
 *     IDE tooling
 *     formatting
 *     refactoring
 *     provenance
 *     compatibility analysis
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Parsing resource syntax must not:
 *
 *     discover hardware
 *     open devices
 *     access the network
 *     allocate memory on behalf of a target
 *     execute external commands
 *     evaluate untrusted resource expressions
 *
 * Evaluation occurs in controlled semantic/compiler/runtime phases.
 *
 * ============================================================================
 * PERFORMANCE CONTRACT
 * ============================================================================
 *
 * The grammar must remain compositional and deterministic.
 *
 * Resource lists use parser repetition rather than generated fixed-size
 * alternatives.
 *
 * Resource expressions delegate to the canonical expression grammar.
 *
 * Resource kinds use recursive qualified-name composition rather than a fixed
 * vocabulary.
 *
 * ============================================================================
 * VALIDATION CONTRACT
 * ============================================================================
 *
 * grammar/validation/ must verify:
 *
 *     - grammar identity is Resources;
 *     - tokenVocab is ZamaniLexer;
 *     - ResourceExpressions is imported;
 *     - Names is imported;
 *     - no universal resource rule is duplicated by a child grammar;
 *     - no second Resources grammar exists;
 *     - no Resource rule is introduced here;
 *     - no fixed hardware capacity is encoded;
 *     - no physical device enumeration is encoded;
 *     - no physical qubit numbering is encoded;
 *     - no fixed topology is encoded;
 *     - no fixed tensor rank is encoded;
 *     - no fixed register width is encoded;
 *     - no semantic predicates exist;
 *     - no parser actions exist;
 *     - parser entry points are composable;
 *     - resource expressions use the canonical resource expression boundary;
 *     - names use the canonical name boundary;
 *     - source spans remain recoverable;
 *     - parser behavior is deterministic.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Required positive resource tests include:
 *
 *     resource compute;
 *
 *     resource memory: memory;
 *
 *     resource accelerator: accelerator;
 *
 *     resource qpu: quantum::qpu;
 *
 *     requires qubits >= logical_qubits;
 *
 *     requires memory >= required_memory;
 *
 *     requires capability("quantum.measurement");
 *
 *     requires capability("tensor.compute");
 *
 *     prefer latency <= latency_budget;
 *
 *     hint capability("gpu.compute");
 *
 *     target quantum;
 *
 *     target classical;
 *
 *     target future::architecture;
 *
 * Required scalable forms include:
 *
 *     resource future::domain::resource;
 *
 *     resource quantum::logical_qubit;
 *
 *     resource tensor::compute;
 *
 *     resource hardware::accelerator;
 *
 *     resource distributed::communication;
 *
 * Required boundary tests include:
 *
 *     one resource;
 *     many resources;
 *     large property sets;
 *     deeply qualified names;
 *     symbolic quantities;
 *     computed quantities;
 *     nested resource specifications.
 *
 * Required negative tests include:
 *
 *     missing resource name;
 *     missing kind;
 *     malformed qualified name;
 *     missing expression;
 *     missing semicolon;
 *     malformed property assignment;
 *     malformed requirement;
 *     malformed constraint;
 *     malformed target.
 *
 * Required portability tests must verify that adding a new semantic resource
 * kind does not require changing this grammar.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no universal hardware-size constants.
 *
 * Forbidden concepts include:
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
 *
 * No resource grammar rule may be introduced later that encodes an equivalent
 * fixed limit.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing stable resource syntax remains valid unless an explicit language
 * compatibility process changes it.
 *
 * New resource kinds should normally be introduced as semantic names rather
 * than new lexer keywords.
 *
 * Therefore:
 *
 *     future::resource
 *
 * can be represented without modifying the universal resource grammar.
 *
 * Introducing a new reserved keyword requires:
 *
 *     lexical specification update
 *     compatibility review
 *     lexer update
 *     parser conformance
 *     documentation update
 *     tests
 *
 * ============================================================================
 * NO-REWORK COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete as the resource ORCHESTRATOR when all of the following
 * are true:
 *
 * [ ] Resources is the only universal resource parser grammar.
 *
 * [ ] The file is the only owner of `resources`.
 *
 * [ ] The file is the only owner of `resourceItem`.
 *
 * [ ] Universal resource categories have one owner.
 *
 * [ ] ResourceExpressions is the expression boundary.
 *
 * [ ] Names is the name boundary.
 *
 * [ ] Specialized resource grammars expose unique public entry rules.
 *
 * [ ] No imported grammar redefines a universal rule owned here.
 *
 * [ ] `resource.g4` has a unique grammar identity such as ResourceIntent.
 *
 * [ ] No child resource grammar imports Resources.
 *
 * [ ] No resource grammar introduces a second resource IR.
 *
 * [ ] No resource grammar contains physical hardware allocation semantics.
 *
 * [ ] No resource grammar contains fixed machine-size limits.
 *
 * [ ] Quantum resource intent remains compatible with canonical quantum::ir.
 *
 * [ ] Classical, quantum, HDL, distributed, AI, data, networking and future
 *     domains can express resource requirements through semantic names.
 *
 * [ ] Source spans are preserved by the frontend AST.
 *
 * [ ] Semantic validation is downstream of parsing.
 *
 * [ ] Target availability is downstream of semantic analysis.
 *
 * [ ] Routing, scheduling, resilience, QEC, ZQN and HAL remain downstream.
 *
 * [ ] Positive tests exist.
 *
 * [ ] Negative tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Scalability tests exist.
 *
 * [ ] Determinism tests exist.
 *
 * [ ] Compatibility tests exist.
 *
 * [ ] Hard-coding audit passes.
 *
 * [ ] ANTLR generation succeeds.
 *
 * [ ] Rust generated-parser integration succeeds on Rust 1.97 / 1.97.1.
 *
 * [ ] Safe Rust only is used downstream.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * This file answers:
 *
 *     "What resource-language construct is this, and which specialized
 *      resource component owns its internal structure?"
 *
 * It does NOT answer:
 *
 *     "Which physical resource should execute it?"
 *
 *     "How should the resource be allocated?"
 *
 *     "How should the computation be routed?"
 *
 *     "How should it be scheduled?"
 *
 *     "How should QEC be performed?"
 *
 *     "Which QPU, CPU, GPU, FPGA or node should be selected?"
 *
 * Those decisions remain downstream.
 *
 * ============================================================================
 */
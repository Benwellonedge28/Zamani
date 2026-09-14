/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/resources/capabilities.g4
 *
 * Grammar kind:
 *     ANTLR4 parser grammar
 *
 * Rust implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines the RESOURCE-SIDE SYNTAX for expressing capability
 * requirements, capability constraints, capability preferences, capability
 * hints, capability availability, and capability relationships.
 *
 * It deliberately does NOT define the capability identity model itself.
 *
 * Canonical capability identity and version syntax are owned by:
 *
 *     grammar/core/capabilities.g4
 *
 * Universal resource intent is owned by:
 *
 *     grammar/resources/resources.g4
 *
 * This file connects those two boundaries.
 *
 * ============================================================================
 * ARCHITECTURAL ROLE
 * ============================================================================
 *
 *                         Zamani source
 *                              |
 *                              v
 *                           lexer
 *                              |
 *                              v
 *                           parser
 *                              |
 *              +---------------+----------------+
 *              |                                |
 *              v                                v
 *       capability syntax                resource syntax
 *       core/capabilities.g4             resources/resources.g4
 *              |                                |
 *              +---------------+----------------+
 *                              |
 *                              v
 *                    resource capability intent
 *                              |
 *                              v
 *                     semantic analysis
 *                              |
 *             +----------------+----------------+
 *             |                |                |
 *             v                v                v
 *        capability       resource model    requirement
 *        resolution       resolution        analysis
 *             |                |                |
 *             +----------------+----------------+
 *                              |
 *                              v
 *                    canonical semantic model
 *                              |
 *          +-------------------+--------------------+
 *          |                   |                    |
 *          v                   v                    v
 *      classical IR       quantum::ir         hardware/HDL
 *                              |
 *                              v
 *                  optimization / routing /
 *                  scheduling / runtime
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - resource capability requirements;
 *   - resource capability constraints;
 *   - resource capability preferences;
 *   - resource capability hints;
 *   - resource capability availability expressions;
 *   - resource capability compatibility clauses;
 *   - resource capability implication syntax;
 *   - resource capability exclusion syntax;
 *   - resource capability composition syntax;
 *   - resource capability association with abstract resources;
 *   - resource capability association with abstract targets;
 *   - resource capability association with resource groups;
 *   - resource capability property predicates;
 *   - source-level capability/resource intent composition.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - capability identifiers;
 *   - qualified-name syntax;
 *   - capability versions;
 *   - semantic-version compatibility;
 *   - capability registry;
 *   - capability discovery;
 *   - resource identifiers;
 *   - general expressions;
 *   - general types;
 *   - hardware discovery;
 *   - hardware topology;
 *   - device selection;
 *   - physical qubit selection;
 *   - QubitId;
 *   - PhysicalQubitId;
 *   - QEC;
 *   - ZQN;
 *   - quantum::ir;
 *   - routing;
 *   - scheduling;
 *   - optimization;
 *   - calibration;
 *   - runtime allocation;
 *   - backend implementation;
 *   - deployment implementation;
 *   - authorization.
 *
 * ============================================================================
 * CANONICAL OWNERSHIP BOUNDARY
 * ============================================================================
 *
 * Capability identity belongs to:
 *
 *     grammar/core/capabilities.g4
 *
 * Resource intent belongs to:
 *
 *     grammar/resources/resources.g4
 *
 * Resource capability composition belongs here.
 *
 * Therefore:
 *
 *     capabilityReference
 *
 * is consumed from the canonical capability grammar and MUST NOT be
 * reimplemented here.
 *
 * Likewise:
 *
 *     expression
 *
 * is consumed from the canonical expression grammar and MUST NOT be
 * recreated
 * here.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * This grammar is designed for:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Capability requirements describe semantic requirements rather than a
 * particular implementation.
 *
 * For example:
 *
 *     requires capability zamani::quantum::dynamic_control;
 *
 * means that the execution environment must provide the requested
 * capability.
 *
 * It does NOT mean:
 *
 *     use a particular QPU;
 *     use a particular device;
 *     use a particular topology;
 *     use a particular number of qubits;
 *     use a particular CPU;
 *     use a particular GPU;
 *     use a particular FPGA.
 *
 * Those decisions belong to later compilation, resource selection,
 * scheduling, routing, HAL, and runtime layers.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are intentionally NO grammar-level limits on:
 *
 *     capability count
 *     resource count
 *     target count
 *     capability relationships
 *     resource groups
 *     capability predicates
 *     capability expressions
 *     program size
 *     machine size
 *     device count
 *     qubit count
 *     CPU count
 *     GPU count
 *     FPGA count
 *     node count
 *     memory capacity
 *
 * No:
 *
 *     MAX_CAPABILITIES
 *     MAX_RESOURCES
 *     MAX_DEVICES
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *
 * may be introduced here.
 *
 * Repetition is represented structurally using ANTLR repetition operators.
 *
 * ============================================================================
 * SEMANTIC SEPARATION
 * ============================================================================
 *
 * CAPABILITY
 *     A property or facility that an environment can provide.
 *
 * REQUIREMENT
 *     A mandatory capability condition.
 *
 * CONSTRAINT
 *     A mandatory condition governing valid realization.
 *
 * PREFERENCE
 *     A non-mandatory optimization preference.
 *
 * HINT
 *     Advisory information that may be ignored.
 *
 * AVAILABILITY
 *     An expression describing whether/when a capability is available.
 *
 * IMPLICATION
 *     A semantic relationship where one capability condition implies another.
 *
 * EXCLUSION
 *     A semantic relationship expressing incompatibility.
 *
 * COMPOSITION
 *     A source-level combination of capability conditions.
 *
 * NONE of these are hardware allocation decisions.
 *
 * ============================================================================
 * IMPORTANT: NO CLOSED CAPABILITY ENUMERATION
 * ============================================================================
 *
 * This file MUST NOT contain rules such as:
 *
 *     quantumCapability
 *     cpuCapability
 *     gpuCapability
 *     fpgaCapability
 *     qpuCapability
 *
 * as closed enumerations.
 *
 * Capability identities remain open-world and namespaced.
 *
 * Examples include:
 *
 *     zamani::compute::parallel
 *     zamani::quantum::dynamic_control
 *     zamani::quantum::mid_circuit_measurement
 *     zamani::hardware::fpga
 *     zamani::hardware::gpu
 *     zamani::network::rdma
 *     zamani::security::post_quantum_crypto
 *     future::compute::new_architecture
 *
 * without requiring this file to change.
 *
 * ============================================================================
 * IMPORT BOUNDARY
 * ============================================================================
 *
 * The canonical capability grammar owns:
 *
 *     capabilityReference
 *     capabilityName
 *     capabilityVersionClause
 *
 * The canonical expression grammar owns:
 *
 *     expression
 *
 * The canonical lexer owns all tokens.
 *
 * ============================================================================
 */

parser grammar ResourceCapabilities;

options {
    tokenVocab = ZamaniLexer;
}

import Capabilities, Expressions;


/*
 * ============================================================================
 * 1. PUBLIC RESOURCE-CAPABILITY ENTRY POINT
 * ============================================================================
 *
 * A resource capability section may contain any number of capability intent
 * clauses.
 *
 * There is intentionally no finite limit.
 */
resourceCapabilities
    : resourceCapabilityItem*
    ;


/*
 * ============================================================================
 * 2. RESOURCE-CAPABILITY ITEM
 * ============================================================================
 */

resourceCapabilityItem
    : resourceCapabilityRequirement
    | resourceCapabilityConstraint
    | resourceCapabilityPreference
    | resourceCapabilityHint
    | resourceCapabilityAvailability
    | resourceCapabilityImplication
    | resourceCapabilityExclusion
    | resourceCapabilityComposition
    | resourceCapabilityAssertion
    ;


/*
 * ============================================================================
 * 3. REQUIREMENT
 * ============================================================================
 *
 * A requirement is mandatory.
 *
 * Examples:
 *
 *     requires capability zamani::quantum::dynamic_control;
 *
 *     requires capability
 *         zamani::quantum::mid_circuit_measurement
 *         version >= 1.0;
 *
 *     requires resource capability
 *         zamani::compute::parallel;
 *
 * The semantic layer determines whether the requirement can be satisfied.
 */

resourceCapabilityRequirement
    : K_REQUIRES
      resourceCapabilityRequirementBody
      SEMI
    ;


resourceCapabilityRequirementBody
    : K_CAPABILITY
      capabilityReference
    | K_RESOURCE
      K_CAPABILITY
      capabilityReference
    | K_RESOURCE
      capabilityReference
    ;


/*
 * ============================================================================
 * 4. CONSTRAINT
 * ============================================================================
 *
 * Constraints are mandatory conditions.
 *
 * They are stronger than preferences and hints.
 */

resourceCapabilityConstraint
    : K_CONSTRAINT
      resourceCapabilityPredicate
      SEMI
    ;


/*
 * ============================================================================
 * 5. PREFERENCE
 * ============================================================================
 *
 * Preferences do not make an otherwise valid realization invalid.
 *
 * They are optimization intent.
 */

resourceCapabilityPreference
    : K_PREFERENCE
      resourceCapabilityPredicate
      SEMI
    ;


/*
 * ============================================================================
 * 6. HINT
 * ============================================================================
 *
 * Hints are advisory.
 */

resourceCapabilityHint
    : K_HINT
      resourceCapabilityPredicate
      SEMI
    ;


/*
 * ============================================================================
 * 7. AVAILABILITY
 * ============================================================================
 *
 * Availability is a semantic expression and is intentionally not tied to
 * wall-clock time, a machine identifier, or a particular runtime.
 *
 * Example:
 *
 *     capability zamani::quantum::dynamic_control
 *     available when execution_context.supports_dynamic_control;
 *
 * The expression is interpreted downstream.
 */

resourceCapabilityAvailability
    : K_CAPABILITY
      capabilityReference
      K_AVAILABLE
      resourceCapabilityAvailabilityCondition
      SEMI
    ;


/*
 * ============================================================================
 * 8. IMPLICATION
 * ============================================================================
 *
 * Example:
 *
 *     capability A implies capability B;
 *
 * This is semantic relationship syntax.
 *
 * It does not mean that the compiler should physically allocate anything.
 */

resourceCapabilityImplication
    : K_CAPABILITY
      capabilityReference
      K_IMPLIES
      capabilityReference
      SEMI
    ;


/*
 * ============================================================================
 * 9. EXCLUSION
 * ============================================================================
 *
 * Example:
 *
 *     capability A excludes capability B;
 *
 * Semantic validation determines whether the exclusion is meaningful and
 * whether the resulting requirement set is satisfiable.
 */

resourceCapabilityExclusion
    : K_CAPABILITY
      capabilityReference
      K_EXCLUDES
      capabilityReference
      SEMI
    ;


/*
 * ============================================================================
 * 10. COMPOSITION
 * ============================================================================
 *
 * A capability composition allows several capability requirements to be
 * expressed as one resource-level condition.
 *
 * The grammar preserves structure; semantic analysis determines satisfiability.
 */

resourceCapabilityComposition
    : K_CAPABILITY
      LBRACE
      resourceCapabilityPredicateList?
      RBRACE
      SEMI
    ;


/*
 * ============================================================================
 * 11. ASSERTION
 * ============================================================================
 *
 * Assertion is intentionally distinct from requirement.
 *
 * An assertion expresses a condition that semantic analysis may verify.
 *
 * It does not allocate a resource.
 */

resourceCapabilityAssertion
    : K_ASSERT
      resourceCapabilityPredicate
      SEMI
    ;


/*
 * ============================================================================
 * 12. CAPABILITY PREDICATE
 * ============================================================================
 *
 * A predicate may refer to:
 *
 *     capability identity
 *     capability version
 *     resource expressions
 *     general expressions
 *     capability relationships
 *
 * The actual meaning is determined semantically.
 */

resourceCapabilityPredicate
    : resourceCapabilityAtomicPredicate
    | LPAREN resourceCapabilityPredicate RPAREN
    | resourceCapabilityPredicate AND resourceCapabilityPredicate
    | resourceCapabilityPredicate OR resourceCapabilityPredicate
    | NOT resourceCapabilityPredicate
    ;


resourceCapabilityAtomicPredicate
    : resourceCapabilityReferencePredicate
    | resourceCapabilityVersionPredicate
    | resourceCapabilityPropertyPredicate
    | resourceCapabilityExpressionPredicate
    ;


/*
 * ============================================================================
 * 13. CAPABILITY REFERENCE PREDICATE
 * ============================================================================
 */

resourceCapabilityReferencePredicate
    : K_CAPABILITY
      capabilityReference
    | capabilityReference
    ;


/*
 * ============================================================================
 * 14. CAPABILITY VERSION PREDICATE
 * ============================================================================
 *
 * Version syntax itself is owned by core/capabilities.g4.
 *
 * This rule only attaches the canonical version clause to a capability
 * reference in a resource predicate.
 */

resourceCapabilityVersionPredicate
    : K_CAPABILITY
      capabilityReference
      capabilityVersionClause
    ;


/*
 * ============================================================================
 * 15. CAPABILITY PROPERTY PREDICATE
 * ============================================================================
 *
 * Property names remain open through qualified names.
 *
 * Examples:
 *
 *     capability quantum.dynamic_control.mode == "dynamic";
 *
 *     capability accelerator.vector_width >= requested_width;
 *
 * The semantic layer decides whether a property exists.
 */

resourceCapabilityPropertyPredicate
    : K_CAPABILITY
      capabilityReference
      DOT
      qualifiedName
      resourceCapabilityComparisonOperator
      resourceCapabilityValue
    ;


/*
 * ============================================================================
 * 16. GENERAL EXPRESSION PREDICATE
 * ============================================================================
 *
 * This provides an escape hatch for resource/capability conditions without
 * duplicating the expression grammar.
 *
 * Semantic analysis MUST distinguish arbitrary program expressions from
 * capability/resource properties where required.
 */

resourceCapabilityExpressionPredicate
    : expression
    ;


/*
 * ============================================================================
 * 17. COMPARISON OPERATOR
 * ============================================================================
 */

resourceCapabilityComparisonOperator
    : EQ
    | NE
    | LT
    | LE
    | GT
    | GE
    ;


/*
 * ============================================================================
 * 18. CAPABILITY VALUE
 * ============================================================================
 *
 * Values are deliberately expressed using the general expression grammar.
 *
 * This avoids creating a second literal/type system.
 */

resourceCapabilityValue
    : expression
    ;


/*
 * ============================================================================
 * 19. PREDICATE LIST
 * ============================================================================
 */

resourceCapabilityPredicateList
    : resourceCapabilityPredicate
      (COMMA resourceCapabilityPredicate)*
    ;


/*
 * ============================================================================
 * 20. RESOURCE-SCOPED CAPABILITY REQUIREMENT
 * ============================================================================
 *
 * This form associates a capability with an abstract resource expression.
 *
 * Example:
 *
 *     requires resource compute capability zamani::compute::parallel;
 *
 * The resource expression remains symbolic.
 *
 * It does not identify a concrete machine resource.
 */

resourceScopedCapabilityRequirement
    : K_REQUIRES
      K_RESOURCE
      resourceCapabilityResourceReference
      K_CAPABILITY
      capabilityReference
      SEMI
    ;


/*
 * ============================================================================
 * 21. RESOURCE-SCOPED CAPABILITY CONSTRAINT
 * ============================================================================
 */

resourceScopedCapabilityConstraint
    : K_CONSTRAINT
      K_RESOURCE
      resourceCapabilityResourceReference
      K_CAPABILITY
      capabilityReference
      SEMI
    ;


/*
 * ============================================================================
 * 22. ABSTRACT RESOURCE REFERENCE
 * ============================================================================
 *
 * Resource identity is intentionally symbolic.
 *
 * No physical resource identifier is allowed to be implied by this grammar.
 */

resourceCapabilityResourceReference
    : qualifiedName
    | IDENTIFIER
    ;


/*
 * ============================================================================
 * 23. CAPABILITY GROUP
 * ============================================================================
 *
 * A group allows multiple capabilities to be associated with the same
 * abstract resource context.
 *
 * Example:
 *
 *     resource capability_group {
 *         requires capability A;
 *         requires capability B;
 *     }
 *
 * The semantic layer determines whether the group is conjunctive,
 * alternative, conditional, or otherwise meaningful.
 */

resourceCapabilityGroup
    : K_RESOURCE
      K_CAPABILITY
      LBRACE
      resourceCapabilityItem*
      RBRACE
    ;


/*
 * ============================================================================
 * 24. ALTERNATIVE CAPABILITY SET
 * ============================================================================
 *
 * This represents semantic alternatives rather than a fixed backend list.
 *
 * Example:
 *
 *     capability {
 *         A
 *         OR B
 *         OR C
 *     }
 *
 * The implementation remains free to choose any satisfying capability.
 */

resourceCapabilityAlternative
    : K_CAPABILITY
      LBRACE
      capabilityReference
      (OR capabilityReference)+
      RBRACE
      SEMI
    ;


/*
 * ============================================================================
 * 25. ALL-OF CAPABILITY SET
 * ============================================================================
 */

resourceCapabilityAllOf
    : K_CAPABILITY
      LBRACE
      capabilityReference
      (COMMA capabilityReference)*
      RBRACE
      SEMI
    ;


/*
 * ============================================================================
 * 26. OPTIONAL CAPABILITY
 * ============================================================================
 *
 * Optional capabilities are advisory unless a downstream semantic rule
 * explicitly promotes them into a requirement.
 */

resourceCapabilityOptional
    : K_OPTIONAL
      K_CAPABILITY
      capabilityReference
      SEMI
    ;


/*
 * ============================================================================
 * 27. NEGATED CAPABILITY
 * ============================================================================
 *
 * This does not mean that a backend must physically remove a capability.
 *
 * It means that a valid execution context must not depend on the capability
 * in the manner defined by semantic analysis.
 */

resourceCapabilityNegation
    : NOT
      K_CAPABILITY
      capabilityReference
      SEMI
    ;


/*
 * ============================================================================
 * 28. CAPABILITY CONTRACT
 * ============================================================================
 *
 * A contract binds capability intent to an abstract resource context.
 *
 * It intentionally contains no physical placement information.
 */

resourceCapabilityContract
    : K_CAPABILITY
      K_CONTRACT
      qualifiedName
      LBRACE
      resourceCapabilityItem*
      RBRACE
    ;


/*
 * ============================================================================
 * 29. CAPABILITY PROPERTY ACCESS
 * ============================================================================
 *
 * Open-world property names prevent the grammar from having to be changed
 * every time a new computing domain introduces a capability property.
 */

resourceCapabilityPropertyAccess
    : capabilityReference
      DOT
      qualifiedName
    ;


/*
 * ============================================================================
 * 30. CAPABILITY REQUIREMENT EXPRESSION
 * ============================================================================
 *
 * This is the canonical resource-level expression form.
 *
 * It is intentionally structural rather than hardware-specific.
 */

resourceCapabilityRequirementExpression
    : resourceCapabilityPredicate
    ;


/*
 * ============================================================================
 * 31. SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar itself imposes no finite resource/capability limit.
 *
 * Expressions may derive capability requirements from:
 *
 *     input size
 *     problem size
 *     workload
 *     program parameters
 *     execution context
 *     compilation context
 *     available capacity
 *
 * Example semantic forms:
 *
 *     required_capability_count
 *     capability_requirement_for(problem_size)
 *
 * are represented through expressions and resolved downstream.
 *
 * This grammar MUST NOT evaluate such expressions.
 */


/*
 * ============================================================================
 * 32. HARDWARE INDEPENDENCE CONTRACT
 * ============================================================================
 *
 * This grammar MUST NOT contain:
 *
 *     device IDs
 *     PCI addresses
 *     CPU IDs
 *     GPU IDs
 *     FPGA IDs
 *     QPU IDs
 *     physical qubit IDs
 *     topology indices
 *     fixed machine sizes
 *
 * A concrete hardware implementation belongs to the hardware abstraction
 * layer and target/compilation infrastructure.
 */


/*
 * ============================================================================
 * 33. QUANTUM INTEGRATION CONTRACT
 * ============================================================================
 *
 * Quantum capability names may occur here:
 *
 *     zamani::quantum::dynamic_control
 *     zamani::quantum::mid_circuit_measurement
 *     zamani::quantum::logical_qubits
 *     zamani::quantum::fault_tolerant_execution
 *
 * They remain capability identities.
 *
 * This file does NOT import or depend upon:
 *
 *     quantum::ir
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     QEC implementation
 *     ZQN implementation
 *
 * Semantic lowering may later connect the requirement to quantum::ir and
 * other quantum compilation layers.
 */


/*
 * ============================================================================
 * 34. CLASSICAL INTEGRATION CONTRACT
 * ============================================================================
 *
 * Classical capabilities may describe:
 *
 *     parallel execution
 *     vector execution
 *     numerical acceleration
 *     symbolic execution
 *     tensor operations
 *     distributed execution
 *
 * No CPU architecture is implied.
 */


/*
 * ============================================================================
 * 35. HDL/HARDWARE INTEGRATION CONTRACT
 * ============================================================================
 *
 * Hardware capabilities may describe:
 *
 *     programmable logic
 *     pipeline support
 *     memory interface
 *     clocking
 *     hardware acceleration
 *     reconfiguration
 *
 * Concrete FPGA/ASIC/device realization is outside this grammar.
 */


/*
 * ============================================================================
 * 36. DISTRIBUTED INTEGRATION CONTRACT
 * ============================================================================
 *
 * Distributed capabilities may describe:
 *
 *     communication
 *     replication
 *     fault tolerance
 *     consistency
 *     remote execution
 *
 * Node count and topology remain runtime/target properties.
 */


/*
 * ============================================================================
 * 37. SECURITY INTEGRATION CONTRACT
 * ============================================================================
 *
 * Security capabilities may be named here, but authorization remains outside
 * the grammar.
 *
 * For example:
 *
 *     zamani::security::post_quantum_crypto
 *
 * is a capability requirement.
 *
 * It is not a runtime authorization token.
 */


/*
 * ============================================================================
 * 38. EFFECT INTEGRATION CONTRACT
 * ============================================================================
 *
 * Effect syntax remains owned by grammar/effects.
 *
 * This grammar may be consumed by effect analysis, but it does not define
 * effects itself.
 */


/*
 * ============================================================================
 * 39. RESOURCE INTEGRATION CONTRACT
 * ============================================================================
 *
 * grammar/resources/resources.g4 owns:
 *
 *     resources
 *     resource declarations
 *     resource quantities
 *     resource constraints
 *     resource preferences
 *     resource requirements
 *
 * This file supplies capability-specific clauses consumed by that model.
 *
 * The two grammars therefore form:
 *
 *     resource semantics
 *          +
 *     capability semantics
 *
 * rather than two competing resource systems.
 */


/*
 * ============================================================================
 * 40. AST CONTRACT
 * ============================================================================
 *
 * The frontend AST representation derived from this grammar should preserve:
 *
 *     source span
 *     capability identity
 *     capability version requirement
 *     resource association
 *     predicate structure
 *     relationship kind
 *     preference/requirement/constraint/hint classification
 *
 * It MUST NOT contain:
 *
 *     physical device state
 *     runtime handles
 *     allocated resources
 *     scheduler state
 *     calibration data
 *     routing state
 *     QEC state
 *     ZQN state
 */


/*
 * ============================================================================
 * 41. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     capability lookup
 *     namespace resolution
 *     version compatibility
 *     capability availability
 *     capability conflicts
 *     requirement satisfiability
 *     resource compatibility
 *     target compatibility
 *     effect compatibility
 *     security policy validation
 *     quantum capability interpretation
 *     hardware capability interpretation
 *
 * This grammar performs none of those operations.
 */


/*
 * ============================================================================
 * 42. COMPILATION CONTRACT
 * ============================================================================
 *
 * After semantic analysis, capability requirements may influence:
 *
 *     target selection
 *     lowering
 *     optimization
 *     routing
 *     scheduling
 *     backend selection
 *     runtime dispatch
 *
 * They remain semantic intent until those later stages deliberately interpret
 * them.
 */


/*
 * ============================================================================
 * 43. DETERMINISM CONTRACT
 * ============================================================================
 *
 * This grammar:
 *
 *     contains no actions;
 *     contains no semantic predicates;
 *     performs no I/O;
 *     performs no network access;
 *     performs no hardware discovery;
 *     performs no runtime calls;
 *     performs no random operations.
 *
 * Parsing therefore depends only on the supplied token stream.
 */


/*
 * ============================================================================
 * 44. SECURITY CONTRACT
 * ============================================================================
 *
 * Parsing capability syntax MUST NOT:
 *
 *     access the filesystem;
 *     access the network;
 *     discover hardware;
 *     execute code;
 *     load plugins;
 *     resolve external capabilities;
 *     grant permissions;
 *     authorize execution.
 *
 * Those operations belong to explicitly controlled later compiler/runtime
 * layers.
 */


/*
 * ============================================================================
 * 45. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing capability declarations and references remain owned by
 *
 *     grammar/core/capabilities.g4
 *
 * Existing resource declarations remain owned by
 *
 *     grammar/resources/resources.g4
 *
 * This file must evolve only by extending resource-capability composition,
 * not by duplicating those grammars.
 */


/*
 * ============================================================================
 * 46. EXTENSIBILITY CONTRACT
 * ============================================================================
 *
 * New capability namespaces require no modification here.
 *
 * New computing domains therefore remain possible:
 *
 *     quantum
 *     classical
 *     hdl
 *     ai
 *     distributed
 *     photonic
 *     neuromorphic
 *     molecular
 *     optical
 *     biological
 *     future
 *
 * provided their capability identity follows the canonical capability grammar.
 */


/*
 * ============================================================================
 * 47. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when:
 *
 * [ ] It compiles with the canonical ZamaniLexer.
 *
 * [ ] It imports the canonical Capabilities grammar.
 *
 * [ ] It imports the canonical Expressions grammar.
 *
 * [ ] It introduces no duplicate identifier grammar.
 *
 * [ ] It introduces no duplicate capability-version grammar.
 *
 * [ ] It introduces no hardware-specific identifiers.
 *
 * [ ] It contains no fixed machine/resource limits.
 *
 * [ ] It contains no Rust actions.
 *
 * [ ] It contains no unsafe code.
 *
 * [ ] It performs no I/O.
 *
 * [ ] It performs no hardware discovery.
 *
 * [ ] It does not depend on quantum::ir.
 *
 * [ ] It does not redefine quantum resources.
 *
 * [ ] It does not redefine hardware resources.
 *
 * [ ] It remains open-world for capability namespaces.
 *
 * [ ] Requirement/constraint/preference/hint semantics remain distinct.
 *
 * [ ] Capability identity remains owned by core/capabilities.g4.
 *
 * [ ] Resource identity remains owned by resources/resources.g4.
 *
 * [ ] Semantic compatibility remains outside the grammar.
 *
 * [ ] Positive parser tests exist.
 *
 * [ ] Negative parser tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Cross-domain tests exist.
 *
 * [ ] POCO-REAF scalability tests exist.
 *
 * [ ] Deterministic parsing tests exist.
 *
 * [ ] Round-trip tests exist where a canonical formatter is available.
 */
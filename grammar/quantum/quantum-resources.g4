/*
 * ============================================================================
 * Zamani Universal Quantum Grammar
 * ============================================================================
 *
 * File:
 *     grammar/quantum/quantum-resources.g4
 *
 * Purpose:
 *     Canonical parser fragment for SOURCE-LEVEL quantum resource intent.
 *
 * Rust integration baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     lexer
 *          |
 *          v
 *     parser
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +-----------------------------+
 *          |                             |
 *          v                             v
 *     canonical IR                resource intent
 *          |                             |
 *          |                 +-----------+-----------+
 *          |                 |           |           |
 *          v                 v           v           v
 *     quantum::ir       capabilities  requirements  constraints
 *          |                 |           |           |
 *          +-----------------+-----------+-----------+
 *                            |
 *                            v
 *                    compilation context
 *                            |
 *                            v
 *                       hardware HAL
 *                            |
 *                            v
 *                      scheduling/routing
 *                            |
 *                            v
 *                         runtime
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - quantum resource requirement syntax;
 *   - quantum resource constraint syntax;
 *   - quantum resource preference syntax;
 *   - quantum resource hint syntax;
 *   - quantum resource references;
 *   - abstract quantum resource quantities;
 *   - resource scopes;
 *   - resource grouping;
 *   - resource expressions;
 *   - resource portability intent;
 *   - resource scalability intent;
 *   - resource latency/energy/reliability intent;
 *   - quantum resource contracts;
 *   - target-independent resource declarations.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - physical qubit allocation;
 *   - physical qubit identifiers;
 *   - topology;
 *   - routing;
 *   - scheduling;
 *   - calibration;
 *   - hardware discovery;
 *   - backend selection;
 *   - device identifiers;
 *   - QEC algorithms;
 *   - QEC limits;
 *   - ZQN noise models;
 *   - simulator implementation;
 *   - runtime allocation;
 *   - canonical quantum IR;
 *   - hardware capability discovery.
 *
 * ============================================================================
 * CORE PRINCIPLE
 * ============================================================================
 *
 * A resource declaration describes WHAT THE PROGRAM REQUIRES.
 *
 * It does not prescribe HOW A MACHINE MUST REALIZE IT.
 *
 * For example:
 *
 *     requires resource qubits >= required_qubits;
 *
 * does NOT mean:
 *
 *     use device X;
 *
 *     use physical qubit 7;
 *
 *     use topology Y;
 *
 *     reserve N particular hardware qubits.
 *
 * Such decisions belong to compilation, routing, scheduling, hardware
 * abstraction, deployment, and runtime layers.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * This grammar deliberately imposes no machine-size limits.
 *
 * There is no:
 *
 *     MAX_QUBITS
 *     MAX_RESOURCES
 *     MAX_DEVICES
 *     MAX_SHOTS
 *     MAX_DISTANCE
 *     MAX_ROUNDS
 *     MAX_MEMORY
 *     MAX_NODES
 *     MAX_ACCELERATORS
 *
 * Resource quantities are expressions.
 *
 * Therefore:
 *
 *     requires resource qubits >= q;
 *
 * and:
 *
 *     requires resource qubits >= expression;
 *
 * remain valid independent of the eventual machine size.
 *
 * Actual feasibility is determined downstream.
 *
 * ============================================================================
 * SEMANTIC SEPARATION
 * ============================================================================
 *
 * REQUIREMENT
 *     Necessary property.
 *
 * CONSTRAINT
 *     Condition that must be respected.
 *
 * PREFERENCE
 *     Desired property that may be traded off.
 *
 * HINT
 *     Advisory information.
 *
 * CAPABILITY
 *     Property supplied by a target/environment.
 *
 * TARGET
 *     Compilation/execution destination.
 *
 * RESOURCE
 *     Abstract computational resource.
 *
 * This file must never collapse those concepts into one syntax category.
 *
 * ============================================================================
 */


/* ============================================================================
 * 1. QUANTUM RESOURCE DECLARATION
 * ============================================================================
 *
 * A resource declaration creates a symbolic source-level resource contract.
 *
 * Example:
 *
 *     resource qubits {
 *         quantity = required_qubits;
 *     }
 *
 * The resource name is symbolic.
 *
 * It is NOT a physical device identifier.
 */

quantumResourceDeclaration
    : K_RESOURCE
      identifier
      quantumResourceType?
      quantumResourceSpecification?
      SEMI?
    ;


/* ============================================================================
 * 2. RESOURCE TYPE
 * ============================================================================
 *
 * Resource kinds remain symbolic rather than being closed over a finite list.
 *
 * This allows future quantum resources without changing the core grammar.
 *
 * Examples:
 *
 *     qubits
 *     logical_qubits
 *     syndrome_capacity
 *     measurement_capacity
 *     quantum_memory
 *     custom_namespace.resource
 */

quantumResourceType
    : qualifiedName
    ;


/* ============================================================================
 * 3. RESOURCE SPECIFICATION
 * ========================================================================== */

quantumResourceSpecification
    : blockExpression
    ;


/* ============================================================================
 * 4. RESOURCE BODY ELEMENT
 * ========================================================================== */

quantumResourceBodyElement
    : attributes*
      quantumResourceClause
    ;


quantumResourceClause
    : quantumResourceQuantityClause
    | quantumResourceRequirementClause
    | quantumResourceConstraintClause
    | quantumResourcePreferenceClause
    | quantumResourceHintClause
    | quantumResourceCapabilityClause
    | quantumResourceScopeClause
    | quantumResourcePortabilityClause
    | quantumResourceScalabilityClause
    | quantumResourcePerformanceClause
    | quantumResourceLatencyClause
    | quantumResourceEnergyClause
    | quantumResourceReliabilityClause
    | quantumResourceReferenceClause
    | quantumResourcePropertyClause
    ;


/* ============================================================================
 * 5. RESOURCE QUANTITY
 * ============================================================================
 *
 * Quantity is an expression.
 *
 * No upper bound is encoded.
 *
 * Examples:
 *
 *     quantity = n;
 *     quantity = required_qubits;
 *     quantity = input_size * factor;
 */

quantumResourceQuantityClause
    : identifier
      ASSIGN
      expression
      SEMI
    ;


/* ============================================================================
 * 6. RESOURCE REQUIREMENTS
 * ============================================================================
 *
 * Requirement means the program cannot be considered feasible unless the
 * semantic condition is satisfied.
 *
 * The grammar does not determine whether the requirement can be satisfied.
 */

quantumResourceRequirementClause
    : K_REQUIRES
      K_RESOURCE
      expression
      SEMI
    ;


/* ============================================================================
 * 7. RESOURCE CONSTRAINTS
 * ============================================================================
 *
 * Constraints are distinct from requirements.
 *
 * Example:
 *
 *     constraint resource.width <= width_limit;
 *
 * The expression remains target-independent.
 */

quantumResourceConstraintClause
    : K_CONSTRAINT
      K_RESOURCE
      expression
      SEMI
    ;


/* ============================================================================
 * 8. RESOURCE PREFERENCES
 * ============================================================================
 *
 * Preferences are advisory optimization objectives.
 *
 * A preference MUST NOT silently become a hard requirement.
 */

quantumResourcePreferenceClause
    : K_PREFERENCE
      K_RESOURCE
      expression
      SEMI
    ;


/* ============================================================================
 * 9. RESOURCE HINTS
 * ============================================================================
 *
 * Hints are advisory compiler/runtime information.
 *
 * Implementations may ignore hints when necessary.
 */

quantumResourceHintClause
    : K_HINT
      K_RESOURCE
      expression
      SEMI
    ;


/* ============================================================================
 * 10. RESOURCE CAPABILITY
 * ============================================================================
 *
 * A capability is a property that an execution environment can provide.
 *
 * This grammar only permits source-level capability requirements/references.
 *
 * Actual capability discovery belongs to the capability/hardware subsystem.
 */

quantumResourceCapabilityClause
    : K_CAPABILITY
      K_RESOURCE
      expression
      SEMI
    ;


/* ============================================================================
 * 11. RESOURCE SCOPE
 * ============================================================================
 *
 * A resource contract may apply to:
 *
 *     a quantum block;
 *     a circuit;
 *     an operation;
 *     a logical qubit;
 *     a register;
 *     a module;
 *     a program;
 *     another semantic scope.
 *
 * The scope expression remains generic so the grammar does not duplicate
 * logical-qubit, circuit, or module reference syntax.
 */

quantumResourceScopeClause
    : K_SCOPE
      expression
      SEMI
    ;


/* ============================================================================
 * 12. PORTABILITY
 * ============================================================================
 *
 * Portability describes whether the resource contract is portable.
 *
 * This is semantic metadata.
 *
 * It does not select a backend.
 */

quantumResourcePortabilityClause
    : K_PORTABLE
      expression
      SEMI
    ;


/* ============================================================================
 * 13. SCALABILITY
 * ============================================================================
 *
 * Scalability describes how a resource requirement behaves as the problem
 * size changes.
 *
 * Examples:
 *
 *     scalability = linear(problem_size);
 *
 *     scalability = symbolic_expression;
 *
 * No fixed scaling catalogue is imposed.
 */

quantumResourceScalabilityClause
    : K_SCALABILITY
      ASSIGN
      expression
      SEMI
    ;


/* ============================================================================
 * 14. PERFORMANCE
 * ========================================================================== */

quantumResourcePerformanceClause
    : K_PERFORMANCE
      ASSIGN
      expression
      SEMI
    ;


/* ============================================================================
 * 15. LATENCY
 * ============================================================================
 *
 * Latency is expressed semantically.
 *
 * Hardware timing realization belongs to scheduling/runtime.
 */

quantumResourceLatencyClause
    : K_LATENCY
      ASSIGN
      expression
      SEMI
    ;


/* ============================================================================
 * 16. ENERGY
 * ========================================================================== */

quantumResourceEnergyClause
    : K_ENERGY
      ASSIGN
      expression
      SEMI
    ;


/* ============================================================================
 * 17. RELIABILITY
 * ============================================================================
 *
 * Reliability is a requirement/property, not a hard-coded hardware guarantee.
 */

quantumResourceReliabilityClause
    : K_RELIABILITY
      ASSIGN
      expression
      SEMI
    ;


/* ============================================================================
 * 18. RESOURCE REFERENCE
 * ============================================================================
 *
 * References an already-declared resource contract.
 *
 * Example:
 *
 *     resource uses quantum.capacity;
 */

quantumResourceReferenceClause
    : K_RESOURCE
      qualifiedName
      SEMI
    ;


/* ============================================================================
 * 19. EXTENSIBLE RESOURCE PROPERTY
 * ============================================================================
 *
 * Future quantum resource concepts must not require modifying the grammar
 * merely because a new semantic property is introduced.
 *
 * The semantic layer owns validation of property names.
 *
 * Dialects may extend the recognized property namespace.
 */

quantumResourcePropertyClause
    : identifier
      ASSIGN
      expression
      SEMI
    ;


/* ============================================================================
 * 20. DIRECT RESOURCE REQUIREMENT
 * ============================================================================
 *
 * Convenience syntax for attaching a resource requirement directly to a
 * quantum construct.
 *
 * Example:
 *
 *     requires resource qubits >= logical_qubits;
 *
 * This is intentionally independent of physical allocation.
 */

quantumResourceRequirement
    : K_REQUIRES
      K_RESOURCE
      expression
      SEMI
    ;


/* ============================================================================
 * 21. RESOURCE CONSTRAINT
 * ========================================================================== */

quantumResourceConstraint
    : K_CONSTRAINT
      K_RESOURCE
      expression
      SEMI
    ;


/* ============================================================================
 * 22. RESOURCE PREFERENCE
 * ========================================================================== */

quantumResourcePreference
    : K_PREFERENCE
      K_RESOURCE
      expression
      SEMI
    ;


/* ============================================================================
 * 23. RESOURCE HINT
 * ========================================================================== */

quantumResourceHint
    : K_HINT
      K_RESOURCE
      expression
      SEMI
    ;


/* ============================================================================
 * 24. RESOURCE CONTRACT
 * ============================================================================
 *
 * A contract groups resource semantics without prescribing realization.
 */

quantumResourceContract
    : K_RESOURCE
      identifier
      blockExpression
    ;


/* ============================================================================
 * 25. RESOURCE SET
 * ============================================================================
 *
 * A set groups independent resource contracts.
 *
 * The number of entries is unbounded by grammar.
 */

quantumResourceSet
    : K_RESOURCES
      LBRACE
      quantumResourceSetElement*
      RBRACE
    ;


quantumResourceSetElement
    : attributes*
      quantumResourceDeclaration
    | quantumResourceRequirement
    | quantumResourceConstraint
    | quantumResourcePreference
    | quantumResourceHint
    ;


/* ============================================================================
 * 26. RESOURCE EXPRESSION
 * ============================================================================
 *
 * Resource expressions are deliberately delegated to the canonical expression
 * grammar.
 *
 * This rule exists as a semantic boundary and MUST NOT introduce a second
 * expression language.
 */

quantumResourceExpression
    : expression
    ;


/* ============================================================================
 * 27. QUANTUM RESOURCE TARGET
 * ============================================================================
 *
 * This means a semantic target category, not a device ID.
 *
 * Examples:
 *
 *     target = quantum;
 *     target = logical_quantum;
 *
 * A concrete backend/device is resolved later.
 */

quantumResourceTarget
    : K_TARGET
      qualifiedName
      SEMI
    ;


/* ============================================================================
 * 28. RESOURCE CAPACITY
 * ============================================================================
 *
 * Capacity is an observed/declared capability.
 *
 * It is not a source-level fixed limit.
 */

quantumResourceCapacity
    : K_CAPACITY
      ASSIGN
      expression
      SEMI
    ;


/* ============================================================================
 * 29. RESOURCE AVAILABILITY
 * ============================================================================
 *
 * Availability may be supplied by compilation/runtime context.
 *
 * It must not be interpreted as a source-level promise.
 */

quantumResourceAvailability
    : K_AVAILABILITY
      ASSIGN
      expression
      SEMI
    ;


/* ============================================================================
 * 30. RESOURCE RESERVATION INTENT
 * ============================================================================
 *
 * Reservation is intent only.
 *
 * The actual reservation belongs to deployment/runtime/resource management.
 */

quantumResourceReservation
    : K_RESERVE
      K_RESOURCE
      expression
      SEMI
    ;


/* ============================================================================
 * 31. RESOURCE ACQUISITION INTENT
 * ============================================================================
 *
 * Acquisition must remain abstract.
 *
 * It cannot directly identify a physical machine.
 */

quantumResourceAcquisition
    : K_ACQUIRE
      K_RESOURCE
      expression
      SEMI
    ;


/* ============================================================================
 * 32. RESOURCE RELEASE INTENT
 * ============================================================================
 *
 * Runtime/resource management owns the actual release operation.
 */

quantumResourceRelease
    : K_RELEASE
      K_RESOURCE
      expression
      SEMI
    ;


/* ============================================================================
 * 33. RESOURCE GROUP
 * ============================================================================
 *
 * Resource groups allow arbitrary numbers of resources.
 *
 * There is deliberately no fixed group size.
 */

quantumResourceGroup
    : K_RESOURCE
      K_GROUP
      identifier
      blockExpression
    ;


/* ============================================================================
 * 34. RESOURCE DERIVATION
 * ============================================================================
 *
 * A resource requirement may be derived from program parameters.
 *
 * Example:
 *
 *     resource logical_capacity = problem_size * code_distance;
 *
 * This remains symbolic until semantic evaluation.
 */

quantumResourceDerivation
    : K_RESOURCE
      identifier
      ASSIGN
      expression
      SEMI
    ;


/* ============================================================================
 * 35. RESOURCE VALIDATION BOUNDARY
 * ============================================================================
 *
 * This grammar does NOT validate:
 *
 *     whether a resource exists;
 *     whether a resource is available;
 *     whether a resource is sufficient;
 *     whether a quantity fits a machine;
 *     whether a topology supports an operation;
 *     whether calibration supports a requirement;
 *     whether a QEC code is implementable;
 *     whether routing is possible;
 *     whether scheduling is possible.
 *
 * Those are semantic/backend responsibilities.
 *
 * The parser only establishes syntactic structure.
 */


/* ============================================================================
 * 36. IMPORTANT NON-DEPENDENCIES
 * ============================================================================
 *
 * This file MUST NOT import or depend semantically on:
 *
 *     src/quantum/hardware/
 *     src/quantum/scheduling/
 *     src/quantum/optimization/
 *     src/quantum/zqn/
 *     src/quantum/resilience/
 *     src/quantum/qec/
 *
 * Those systems consume the semantic resource representation downstream.
 *
 * In particular:
 *
 *     grammar -> hardware
 *
 * is forbidden.
 *
 * Correct direction:
 *
 *     grammar
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     semantic resource contract
 *       |
 *       +---- quantum::ir
 *       +---- compiler
 *       +---- hardware HAL
 *       +---- scheduler
 *       +---- runtime
 */


/* ============================================================================
 * 37. PHYSICAL RESOURCE BOUNDARY
 * ============================================================================
 *
 * This file MUST NOT define:
 *
 *     physicalQubitId
 *     deviceId
 *     hardwareAddress
 *     topologyEdge
 *     physicalPlacement
 *     backendId
 *     calibrationId
 *
 * Those belong to target/hardware layers.
 *
 * A portable quantum program may express:
 *
 *     resource logical_qubits >= n;
 *
 * but not:
 *
 *     resource physical_qubit_17;
 *
 * as a universal source semantic.
 */


/* ============================================================================
 * 38. SCALE BOUNDARY
 * ============================================================================
 *
 * The grammar accepts:
 *
 *     one resource;
 *     many resources;
 *     symbolic resource quantities;
 *     parameter-derived quantities;
 *     arbitrarily large source structures.
 *
 * Any actual limit must be enforced by:
 *
 *     parser implementation limits;
 *     compiler resource limits;
 *     IR limits;
 *     target capabilities;
 *     runtime resources;
 *
 * and must never be encoded as an arbitrary grammar constant.
 */


/* ============================================================================
 * 39. AST CONTRACT
 * ============================================================================
 *
 * Every rule in this file must lower to AST nodes representing:
 *
 *     ResourceReference
 *     ResourceDeclaration
 *     ResourceRequirement
 *     ResourceConstraint
 *     ResourcePreference
 *     ResourceHint
 *     ResourceCapabilityRequirement
 *     ResourceScope
 *     ResourceQuantity
 *     ResourceExpression
 *     ResourceContract
 *
 * These are AST/semantic concepts, not grammar-defined Rust structs.
 *
 * The grammar MUST NOT define Rust code or embedded actions.
 */


/* ============================================================================
 * 40. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis MUST:
 *
 *     - resolve resource names;
 *     - distinguish requirements from preferences;
 *     - type-check resource expressions;
 *     - validate units where applicable;
 *     - validate resource categories;
 *     - resolve capability references;
 *     - detect contradictory constraints;
 *     - detect impossible requirements;
 *     - preserve symbolic quantities;
 *     - preserve provenance;
 *     - preserve source locations;
 *     - preserve dialect/version information;
 *     - reject provider-specific constructs outside an allowed dialect.
 *
 * Semantic analysis MAY determine:
 *
 *     compile-time constants;
 *     symbolic dependencies;
 *     target-specific feasibility;
 *     runtime-resolvable requirements.
 *
 * The grammar itself must not make those decisions.
 */


/* ============================================================================
 * 41. CANONICAL QUANTUM IR CONTRACT
 * ============================================================================
 *
 * Resource syntax lowers into the canonical semantic representation used by
 * the compiler and quantum::ir/resource validation layers.
 *
 * It MUST NOT create a second quantum resource IR owned by this grammar.
 *
 * The lowering boundary is:
 *
 *     parse tree
 *         ->
 *     frontend AST
 *         ->
 *     semantic resource contract
 *         ->
 *     canonical IR / compilation context
 *
 * `quantum::ir` remains authoritative for quantum program semantics.
 */


/* ============================================================================
 * 42. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
 *
 *     MAX_* constants;
 *     fixed qubit counts;
 *     fixed resource counts;
 *     fixed device IDs;
 *     fixed hardware addresses;
 *     fixed topology;
 *     fixed backend names;
 *     fixed vendor names;
 *     fixed accelerator counts;
 *     fixed machine sizes.
 *
 * A future resource category is represented by a symbolic name/qualified name
 * rather than by changing this grammar.
 */


/* ============================================================================
 * 43. TEST CONTRACT
 * ============================================================================
 *
 * Positive:
 *
 *     resource qubits { quantity = n; }
 *
 *     requires resource qubits >= n;
 *
 *     resource logical_capacity {
 *         quantity = problem_size * distance;
 *     }
 *
 *     resource preferred_backend {
 *         portability = portable;
 *     }
 *
 *     resource requirements containing arbitrary symbolic expressions.
 *
 * Negative:
 *
 *     physical device identifiers in portable resource declarations;
 *
 *     hardware topology embedded as resource syntax;
 *
 *     malformed resource expressions;
 *
 *     missing resource expression;
 *
 *     malformed constraints;
 *
 *     duplicate syntactic separators;
 *
 *     invalid block structure.
 *
 * Boundary:
 *
 *     one resource;
 *     many resources;
 *     arbitrarily large resource lists;
 *     symbolic quantities;
 *     nested expressions;
 *     large identifiers;
 *     deeply composed resource contracts;
 *     empty optional resource sections.
 *
 * Cross-domain:
 *
 *     quantum + classical;
 *     quantum + QEC;
 *     quantum + hardware;
 *     quantum + distributed;
 *     quantum + accelerator;
 *     quantum + runtime capability.
 *
 * Determinism:
 *
 *     identical source produces identical parse structure.
 *
 * Round-trip:
 *
 *     source -> lexer -> parser -> AST -> printer -> parser
 *
 * preserves semantic resource intent.
 */


/* ============================================================================
 * 44. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE only when:
 *
 *     [ ] every rule has a unique ownership boundary;
 *     [ ] no lexer rules are duplicated here;
 *     [ ] no second expression grammar exists here;
 *     [ ] no physical resource syntax exists here;
 *     [ ] no machine-size limit exists here;
 *     [ ] requirements/preferences/hints remain distinct;
 *     [ ] capability semantics remain downstream;
 *     [ ] resource expressions remain symbolic;
 *     [ ] AST lowering is defined;
 *     [ ] semantic lowering is defined;
 *     [ ] quantum::ir integration is defined;
 *     [ ] compiler integration is defined;
 *     [ ] runtime integration is defined;
 *     [ ] hardware integration is downstream-only;
 *     [ ] QEC integration is downstream-only;
 *     [ ] ZQN integration is downstream-only;
 *     [ ] scheduling integration is downstream-only;
 *     [ ] routing integration is downstream-only;
 *     [ ] tests cover positive/negative/boundary cases;
 *     [ ] scalability tests contain no artificial maximum;
 *     [ ] deterministic parsing is tested;
 *     [ ] round-trip behavior is tested;
 *     [ ] ANTLR generation succeeds;
 *     [ ] Rust frontend integration succeeds on Rust 1.97/1.97.1;
 *     [ ] no unsafe Rust is required;
 *     [ ] documentation agrees with the grammar;
 *     [ ] no duplicate resource grammar exists elsewhere.
 */
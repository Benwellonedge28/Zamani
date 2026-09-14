/*
 * Zamani Universal Computing Language
 * Resource Reliability Grammar
 *
 * Path:
 *   grammar/resources/reliability.g4
 *
 * Purpose:
 *   Defines the source-level syntax for reliability-related resource
 *   declarations and expressions.
 *
 * Architectural boundary:
 *
 *   Zamani source
 *       |
 *       v
 *   grammar/resources/reliability.g4
 *       |
 *       v
 *   Resource syntax / parse tree
 *       |
 *       v
 *   semantic analysis
 *       |
 *       +--> resource requirements / constraints / preferences
 *       +--> capability matching
 *       +--> target selection
 *       +--> compilation
 *       +--> scheduling
 *       +--> runtime / deployment
 *       +--> resilience
 *
 * This grammar DOES NOT:
 *   - select a physical machine
 *   - select a backend
 *   - implement retry/recovery
 *   - define QEC algorithms
 *   - define ZQN fault models
 *   - define runtime reliability algorithms
 *   - define hardware topology
 *   - define a maximum resource count
 *   - define provider-specific reliability values
 *   - create an IR
 *
 * Reliability is a declarative resource property.
 *
 * IMPORTANT:
 *   This grammar must remain independent of physical machine size.
 *
 *   There are intentionally no:
 *
 *       MAX_DEVICES
 *       MAX_RETRIES
 *       MAX_NODES
 *       MAX_QUBITS
 *       MAX_CORES
 *       MAX_FAILURES
 *
 *   or equivalent source-level scalability ceilings.
 *
 * Rust compatibility:
 *   This grammar contains no embedded target-language actions.
 *   Generated Rust code is therefore kept separate from grammar semantics
 *   and can be generated for the Zamani Rust 1.97 / 1.97.1 toolchain.
 */

parser grammar ZamaniResourceReliabilityParser;

/*
 * Integration contract
 * --------------------
 *
 * resources.g4 is the aggregate resource grammar.
 *
 * It imports this grammar and exposes:
 *
 *     resourceReliabilityClause
 *
 * to the universal resource declaration.
 *
 * resource-expressions.g4 owns the generic expression language.
 * This grammar consumes that expression language; it does not redefine it.
 *
 * The aggregate resources grammar is responsible for connecting this
 * grammar to the project's canonical token vocabulary.
 *
 * Therefore this file deliberately does NOT import resources.g4.
 *
 * Dependency direction:
 *
 *     lexical foundation
 *             |
 *             v
 *     core expressions
 *             |
 *             v
 *     resource-expressions.g4
 *             |
 *             v
 *     reliability.g4
 *             |
 *             v
 *     resources.g4
 *             |
 *             v
 *     universal grammar
 *
 * This avoids a resources.g4 <-> reliability.g4 import cycle.
 */

/* ----------------------------------------------------------------------
 * Entry point
 * ------------------------------------------------------------------ */

/*
 * resourceReliabilityClause
 *
 * Canonical resource-level reliability declaration.
 *
 * The keyword/token used for `reliability` MUST come from the canonical
 * Zamani lexer/token vocabulary.
 *
 * Do not introduce a second reliability keyword in this grammar.
 */
resourceReliabilityClause
    : reliabilitySpecification
    ;

/*
 * A reliability specification can be either:
 *
 *   reliability <expression>
 *
 * or:
 *
 *   reliability {
 *       <property> = <expression>
 *       ...
 *   }
 *
 * The short form is intended for a simple requirement/constraint.
 *
 * The block form provides extensibility without requiring the grammar
 * to be rewritten every time a new reliability property is introduced.
 */
reliabilitySpecification
    : K_RELIABILITY reliabilityExpression
    | K_RELIABILITY LBRACE reliabilityPropertyList? RBRACE
    ;

/* ----------------------------------------------------------------------
 * Reliability properties
 * ------------------------------------------------------------------ */

reliabilityPropertyList
    : reliabilityProperty reliabilityPropertySeparator*
    ;

reliabilityPropertySeparator
    : SEMICOLON
    | COMMA
    ;

reliabilityProperty
    : reliabilityPropertyName ASSIGN reliabilityPropertyValue
    ;

/*
 * Property names are intentionally extensible.
 *
 * Do not turn every future reliability concept into a lexer keyword.
 *
 * Examples of semantic properties that may be recognized by the semantic
 * layer include:
 *
 *   availability
 *   durability
 *   fault_tolerance
 *   recovery
 *   failure_probability
 *   failure_rate
 *   mean_time_between_failures
 *   mean_time_to_failure
 *   mean_time_to_recovery
 *   service_level
 *   confidence
 *   redundancy
 *   survivability
 *   dependability
 *
 * The grammar does not impose a closed list.
 *
 * The semantic layer is responsible for determining which properties are
 * valid in a particular context and which units/domains they require.
 */
reliabilityPropertyName
    : identifier
    ;

/* ----------------------------------------------------------------------
 * Property values
 * ------------------------------------------------------------------ */

reliabilityPropertyValue
    : reliabilityExpression
    ;

/*
 * Reliability expressions reuse the canonical resource expression
 * machinery.
 *
 * This is critical:
 *
 * reliability.g4 MUST NOT create another expression language.
 *
 * Therefore `resourceExpression` is supplied by:
 *
 *     resource-expressions.g4
 *
 * through the aggregate parser grammar.
 */
reliabilityExpression
    : resourceExpression
    ;

/* ----------------------------------------------------------------------
 * Named reliability declarations
 * ------------------------------------------------------------------ */

/*
 * Optional named reliability requirement.
 *
 * Example conceptual syntax:
 *
 *     reliability critical {
 *         availability = ...
 *     }
 *
 * The actual declaration/label syntax remains deliberately generic.
 *
 * Whether a label is legal in a particular resource context is a
 * semantic-validation concern rather than a hardware-size concern.
 */
reliabilityNamedSpecification
    : K_RELIABILITY identifier LBRACE reliabilityPropertyList? RBRACE
    ;

/* ----------------------------------------------------------------------
 * Reliability requirement forms
 * ------------------------------------------------------------------ */

/*
 * Explicit requirement.
 *
 * The semantic layer decides whether the expression represents:
 *
 *   - a minimum
 *   - maximum
 *   - exact target
 *   - interval
 *   - probabilistic requirement
 *   - confidence requirement
 *   - availability objective
 *   - another supported reliability semantic
 *
 * This keeps syntax independent of implementation policy.
 */
reliabilityRequirement
    : K_REQUIRES K_RELIABILITY reliabilityExpression
    ;

/*
 * Explicit constraint.
 *
 * A constraint is different from a capability.
 *
 * Example conceptual meaning:
 *
 *     constraint reliability <threshold>
 *
 * The grammar records the declaration.
 * Semantic analysis determines whether the constraint is satisfiable.
 */
reliabilityConstraint
    : K_CONSTRAINT K_RELIABILITY reliabilityExpression
    ;

/*
 * Explicit preference.
 *
 * A preference is not a hard requirement.
 *
 * This distinction is essential for POCO-REAF:
 *
 *   requirement  -> must be satisfied
 *   constraint   -> limits valid solutions
 *   preference   -> optimization guidance
 */
reliabilityPreference
    : K_PREFER K_RELIABILITY reliabilityExpression
    ;

/*
 * Explicit reliability hint.
 *
 * A hint is advisory and MUST NOT silently become a hard constraint.
 */
reliabilityHint
    : K_HINT K_RELIABILITY reliabilityExpression
    ;

/* ----------------------------------------------------------------------
 * Reliability objective forms
 * ------------------------------------------------------------------ */

/*
 * Objective declaration.
 *
 * The compiler/optimizer may consume this information when selecting
 * among otherwise valid execution strategies.
 */
reliabilityObjective
    : K_OBJECTIVE K_RELIABILITY reliabilityExpression
    ;

/*
 * Reliability target.
 *
 * This is intentionally not a hardware target.
 *
 * A reliability target expresses desired semantic/resource behavior.
 * Mapping it to hardware belongs downstream.
 */
reliabilityTarget
    : K_TARGET K_RELIABILITY reliabilityExpression
    ;

/* ----------------------------------------------------------------------
 * Scope
 * ------------------------------------------------------------------ */

/*
 * Reliability may apply to different semantic scopes.
 *
 * The exact scope grammar should remain owned by the core/resource
 * grammar. This rule therefore uses an identifier rather than introducing
 * a second scope system.
 *
 * Examples of semantic scopes:
 *
 *   program
 *   module
 *   function
 *   task
 *   region
 *   operation
 *   execution
 *   deployment
 *   resource
 *
 * Whether a particular scope is legal is determined semantically.
 */
reliabilityScopedSpecification
    : K_RELIABILITY identifier reliabilitySpecification
    ;

/* ----------------------------------------------------------------------
 * Evidence / observation
 * ------------------------------------------------------------------ */

/*
 * Reliability evidence is declarative metadata.
 *
 * It is NOT a runtime telemetry implementation.
 *
 * The semantic/runtime layers may associate this information with:
 *
 *   calibration
 *   benchmarking
 *   historical measurements
 *   runtime observations
 *   provider capabilities
 *   reliability reports
 *
 * without making the source program dependent on any particular provider.
 */
reliabilityEvidence
    : K_EVIDENCE K_RELIABILITY reliabilityExpression
    ;

/*
 * Confidence describes confidence in a reliability declaration or
 * observation.
 *
 * It must remain distinct from the reliability value itself.
 */
reliabilityConfidence
    : K_CONFIDENCE reliabilityExpression
    ;

/* ----------------------------------------------------------------------
 * Failure-domain declarations
 * ------------------------------------------------------------------ */

/*
 * A failure domain is a semantic grouping, not a physical topology.
 *
 * Examples include:
 *
 *   component
 *   task
 *   resource
 *   service
 *   execution region
 *   deployment unit
 *
 * The semantic layer decides whether a named failure domain corresponds
 * to physical hardware, logical resources, services, or another domain.
 */
reliabilityFailureDomain
    : K_FAILURE_DOMAIN identifier
    ;

/*
 * Correlation information is intentionally declarative.
 *
 * The grammar does not define a particular fault model.
 *
 * ZQN, resilience, hardware, and runtime layers remain owners of their
 * respective fault semantics.
 */
reliabilityCorrelation
    : K_CORRELATED_WITH reliabilityExpression
    ;

/* ----------------------------------------------------------------------
 * Reliability policy references
 * ------------------------------------------------------------------ */

/*
 * A policy reference allows source programs to identify a semantic
 * reliability policy without embedding the implementation of that policy
 * in the grammar.
 *
 * The policy may later be resolved by:
 *
 *   compiler
 *   deployment
 *   runtime
 *   resilience
 *   resource manager
 *
 * depending on the context.
 */
reliabilityPolicy
    : K_POLICY qualifiedIdentifier
    ;

/* ----------------------------------------------------------------------
 * Generic identifier hooks
 * ------------------------------------------------------------------ */

/*
 * These rules intentionally delegate identifier syntax to the canonical
 * grammar.
 *
 * They are included as explicit contracts so this file does not silently
 * invent an identifier representation.
 */
identifier
    : IDENTIFIER
    ;

qualifiedIdentifier
    : identifier
    | qualifiedIdentifier DOT identifier
    ;
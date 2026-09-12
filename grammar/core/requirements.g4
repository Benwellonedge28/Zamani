/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/core/requirements.g4
 *
 * Purpose:
 *     Canonical parser grammar for source-level requirements.
 *
 * Language:
 *     Zamani
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no embedded Rust actions and requires no `unsafe`.
 *     The Zamani compiler implementation MUST use safe Rust only.
 *
 * ============================================================================
 * ARCHITECTURAL ROLE
 * ============================================================================
 *
 * A requirement expresses a condition that a valid realization of a program
 * must satisfy.
 *
 * Examples:
 *
 *     requires quantum::dynamic_control;
 *     requires quantum::dynamic_control version >= 1.2.0;
 *     requires classical::parallel;
 *     requires hardware::fpga;
 *
 * A requirement is SOURCE-LEVEL INTENT.
 *
 * It is NOT:
 *
 *     - a resource allocation;
 *     - a hardware selection;
 *     - a device identifier;
 *     - a physical qubit;
 *     - a CPU;
 *     - a GPU;
 *     - an FPGA;
 *     - an ASIC;
 *     - a QPU;
 *     - a backend selection;
 *     - a routing decision;
 *     - a scheduling decision;
 *     - a calibration record;
 *     - a runtime authorization;
 *     - an execution command.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - requirement declaration syntax;
 *     - requirement reference syntax;
 *     - requirement expressions;
 *     - logical composition of requirements;
 *     - capability-backed requirement references;
 *     - named requirement references;
 *     - optional version constraints attached to requirements;
 *     - requirement grouping;
 *     - requirement lists;
 *     - source-level requirement annotations;
 *     - syntactic requirement modifiers.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - identifiers;
 *     - qualified-name syntax;
 *     - lexical tokens;
 *     - capability declarations;
 *     - capability registry;
 *     - capability discovery;
 *     - resource declarations;
 *     - resource allocation;
 *     - resource expressions;
 *     - hardware discovery;
 *     - target selection;
 *     - target descriptions;
 *     - scheduling;
 *     - routing;
 *     - optimization;
 *     - calibration;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - quantum::ir;
 *     - classical IR;
 *     - HDL IR;
 *     - runtime execution;
 *     - deployment;
 *     - compatibility resolution.
 *
 * ============================================================================
 * FUNDAMENTAL DISTINCTION
 * ============================================================================
 *
 * Capability:
 *
 *     What an environment CAN provide.
 *
 * Requirement:
 *
 *     What a program NEEDS.
 *
 * Resource:
 *
 *     A realizable computational resource.
 *
 * Constraint:
 *
 *     A condition that a realization MUST satisfy.
 *
 * Preference:
 *
 *     A valid realization that is preferred over alternatives.
 *
 * Target:
 *
 *     An execution/compilation context.
 *
 * These concepts MUST remain distinct.
 *
 * For example:
 *
 *     requires quantum::dynamic_control;
 *
 * means:
 *
 *     "the program requires an environment capable of dynamic control."
 *
 * It MUST NOT mean:
 *
 *     "select QPU X."
 *
 * Nor:
 *
 *     "allocate N qubits."
 *
 * Nor:
 *
 *     "use topology Y."
 *
 * Nor:
 *
 *     "route to backend Z."
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Requirements exist specifically to support:
 *
 *     Program Once
 *         ->
 *     Compile Once
 *         ->
 *     Run Everywhere
 *         ->
 *     Run Anywhere
 *         ->
 *     Run Forever
 *
 * A requirement therefore expresses a semantic condition rather than an
 * implementation choice.
 *
 * A source program MAY require:
 *
 *     quantum computation
 *     parallel execution
 *     persistent storage
 *     deterministic execution
 *     dynamic control
 *     secure execution
 *     distributed communication
 *     accelerator support
 *
 * without identifying a particular implementation.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar imposes NO language-level finite limit on:
 *
 *     - number of requirements;
 *     - number of requirement clauses;
 *     - number of alternatives;
 *     - number of conjunctions;
 *     - number of disjunctions;
 *     - number of nested groups;
 *     - qualified-name depth;
 *     - version-component magnitude;
 *     - number of source declarations;
 *     - number of computational domains.
 *
 * Repetition therefore uses:
 *
 *     *
 *     +
 *
 * rather than fixed counts.
 *
 * Practical limits caused by:
 *
 *     - available memory;
 *     - parser implementation;
 *     - source size;
 *     - compiler policy;
 *     - operating-system resources;
 *     - deployment resources
 *
 * are NOT language-level grammar limits.
 *
 * ============================================================================
 * OPEN-WORLD DESIGN
 * ============================================================================
 *
 * Requirement identities MUST be open-ended.
 *
 * This grammar MUST NOT contain closed enumerations such as:
 *
 *     quantumRequirement
 *     gpuRequirement
 *     cpuRequirement
 *     fpgaRequirement
 *     qpuRequirement
 *     vendorRequirement
 *
 * Instead, requirements use canonical qualified names.
 *
 * Examples:
 *
 *     quantum::dynamic_control
 *     quantum::mid_circuit_measurement
 *     classical::parallel
 *     hardware::fpga
 *     future::computing::new_capability
 *
 * New domains can therefore be introduced without modifying this grammar.
 *
 * ============================================================================
 * CAPABILITY BOUNDARY
 * ============================================================================
 *
 * Capability references are consumed from the canonical capability grammar.
 *
 * This file MUST NOT duplicate capability identity syntax.
 *
 * Conceptual dependency:
 *
 *     capabilities.g4
 *          |
 *          v
 *     capabilityReference
 *          |
 *          v
 *     requirements.g4
 *
 * Requirement semantics may later query a capability registry.
 *
 * The parser does not perform that lookup.
 *
 * ============================================================================
 * VERSION BOUNDARY
 * ============================================================================
 *
 * A requirement may carry a version condition:
 *
 *     requires quantum::dynamic_control version >= 1.2.0;
 *
 * Version syntax belongs to the canonical versioning grammar.
 *
 * Version compatibility belongs to semantic analysis.
 *
 * This grammar MUST NOT decide:
 *
 *     - whether 1.2 satisfies 1.1;
 *     - whether major versions are compatible;
 *     - whether a pre-release is acceptable;
 *     - whether a version is available;
 *     - whether a version is deprecated.
 *
 * ============================================================================
 * NAME BOUNDARY
 * ============================================================================
 *
 * Name syntax belongs to:
 *
 *     grammar/core/names.g4
 *
 * This file consumes:
 *
 *     qualifiedName
 *
 * and MUST NOT redefine:
 *
 *     identifier
 *     simpleName
 *     qualifiedName
 *
 * ============================================================================
 * RESOURCE BOUNDARY
 * ============================================================================
 *
 * Requirements MAY refer semantically to resource properties, but this grammar
 * does not invent a second resource language.
 *
 * For example:
 *
 *     requires resources::persistent_memory;
 *
 * may be represented as a named requirement.
 *
 * Numeric resource quantities, capacities, placement and allocation belong to
 * the resource grammar and semantic resource system.
 *
 * Therefore this file MUST NOT contain:
 *
 *     maxQubits
 *     cpuCount
 *     gpuCount
 *     memorySize
 *     topology
 *     deviceId
 *     address
 *
 * or equivalent machine-specific constructs.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Quantum requirements remain abstract.
 *
 * Examples:
 *
 *     requires quantum::measurement;
 *     requires quantum::dynamic_control;
 *     requires quantum::logical_qubits;
 *
 * This grammar does NOT import:
 *
 *     quantum::ir
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     topology
 *     calibration
 *
 * Semantic lowering later determines how the requirement participates in
 * quantum compilation.
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * ============================================================================
 * HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * A requirement may state that some capability is necessary:
 *
 *     requires hardware::accelerated_compute;
 *
 * It MUST NOT directly encode:
 *
 *     use gpu 0
 *     use qpu 7
 *     use fpga 3
 *     use device "specific-device"
 *
 * Hardware selection is downstream.
 *
 * ============================================================================
 * BOOLEAN COMPOSITION
 * ============================================================================
 *
 * Requirements may be composed using:
 *
 *     and
 *     or
 *     not
 *
 * Parentheses explicitly group expressions.
 *
 * Example:
 *
 *     requires (
 *         quantum::dynamic_control
 *         and quantum::measurement
 *     ) or classical::simulation;
 *
 * The parser records structure.
 *
 * Semantic satisfiability is downstream.
 *
 * ============================================================================
 * SATISFIABILITY BOUNDARY
 * ============================================================================
 *
 * The grammar MUST NOT determine whether requirements can be satisfied.
 *
 * For example:
 *
 *     requires quantum::a and quantum::b;
 *
 * is syntactically valid even if no known execution environment currently
 * provides both capabilities.
 *
 * Semantic analysis determines:
 *
 *     SATISFIED
 *     UNSATISFIED
 *     UNKNOWN
 *     CONDITIONAL
 *
 * or the repository's canonical equivalent.
 *
 * ============================================================================
 * UNKNOWN CAPABILITIES
 * ============================================================================
 *
 * Unknown requirement names MUST remain syntactically representable.
 *
 * This is essential for future-proofing.
 *
 * A future capability may be written before the compiler knows its registry
 * definition.
 *
 * Semantic analysis may subsequently report:
 *
 *     unknown capability
 *
 * without requiring a grammar update.
 *
 * ============================================================================
 * NEGATION
 * ============================================================================
 *
 * `not` is syntactic.
 *
 * It MUST NOT be interpreted as:
 *
 *     "the capability must be absent from the machine."
 *
 * Its semantic meaning belongs to the requirement model.
 *
 * In particular, semantic validation MUST define whether:
 *
 *     not capability::x
 *
 * means:
 *
 *     - capability x must not be required;
 *     - capability x must not be selected;
 *     - environment must lack x;
 *     - some other semantic predicate.
 *
 * The grammar merely records negation.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no semantic predicates;
 *     - no embedded actions;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware discovery;
 *     - no runtime calls;
 *     - no random behavior.
 *
 * Parsing therefore depends only on the token stream.
 *
 * ============================================================================
 * SOURCE PRESERVATION
 * ============================================================================
 *
 * The AST/frontend must preserve:
 *
 *     - requirement ordering;
 *     - requirement expression structure;
 *     - identifier spelling;
 *     - qualified-name segment order;
 *     - version-expression structure;
 *     - grouping;
 *     - negation;
 *     - conjunction/disjunction structure;
 *     - source spans.
 *
 * Semantic canonicalization belongs downstream.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The conceptual AST representation is:
 *
 *     RequirementDeclaration
 *         {
 *             expression
 *             attributes
 *             source_span
 *         }
 *
 * RequirementExpression is structurally one of:
 *
 *     CapabilityReference
 *     NamedRequirementReference
 *     RequirementNot
 *     RequirementAll
 *     RequirementAny
 *     RequirementGroup
 *
 * The exact Rust AST types belong to the frontend AST subsystem.
 *
 * The grammar MUST NOT define Rust structures.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * Requirements do not directly become quantum::ir operations.
 *
 * Instead:
 *
 *     source
 *       |
 *       v
 *     requirement AST
 *       |
 *       v
 *     semantic requirement model
 *       |
 *       +--> capability analysis
 *       +--> resource analysis
 *       +--> constraint analysis
 *       +--> target negotiation
 *       +--> compilation policy
 *       |
 *       v
 *     canonical semantic representation
 *
 * If a requirement affects quantum compilation, the semantic layer may
 * influence lowering toward quantum::ir.
 *
 * The requirement grammar itself MUST NOT construct quantum::ir.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * Compilation consumes semantic requirements to determine whether a selected
 * realization is valid.
 *
 * The compiler MAY use requirements to:
 *
 *     - reject impossible compilation contexts;
 *     - select compatible compilation strategies;
 *     - enable required lowering paths;
 *     - request capability negotiation;
 *     - preserve portability metadata.
 *
 * It MUST NOT treat source requirements as direct hardware commands.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime may verify requirements against the actual execution context.
 *
 * Runtime verification is separate from parsing.
 *
 * Example:
 *
 *     source requirement
 *         ->
 *     semantic requirement
 *         ->
 *     runtime capability environment
 *         ->
 *     requirement evaluation
 *
 * A runtime failure MUST NOT be represented as a parser failure.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * Requirements are not authorization.
 *
 * For example:
 *
 *     requires security::secure_execution;
 *
 * does not grant permission to use a secure facility.
 *
 * Authorization belongs to the security subsystem.
 *
 * ============================================================================
 * RESILIENCE CONTRACT
 * ============================================================================
 *
 * Requirements MUST NOT implement retry, recovery, backend switching,
 * quarantine, rollback or other resilience policy.
 *
 * Resilience may consume semantic requirement information when deciding
 * whether a recovery strategy remains valid.
 *
 * The dependency remains:
 *
 *     requirements
 *         ->
 *     semantic model
 *         ->
 *     resilience
 *
 * and never:
 *
 *     requirements
 *         ->
 *     resilience grammar
 *         ->
 *     requirements
 *
 * ============================================================================
 * ANTLR CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It consumes:
 *
 *     tokenVocab = ZamaniLexer
 *
 * The canonical lexer is assembled under:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * The parser grammar intentionally does not define lexer tokens.
 *
 * Shared parser rules such as:
 *
 *     qualifiedName
 *     capabilityReference
 *     capabilityVersionClause
 *
 * are supplied by the canonical parser composition layer.
 *
 * ============================================================================
 * IMPORTANT REPOSITORY INTEGRATION NOTE
 * ============================================================================
 *
 * The repository currently contains multiple parser-layer representations of
 * requirement concepts.
 *
 * The production architecture MUST converge on this ownership:
 *
 *     requirements.g4
 *         -> requirementDeclaration
 *         -> requirementExpression
 *         -> requirementReference
 *
 * The compilation-unit grammar must consume:
 *
 *     requirementDeclaration
 *
 * directly.
 *
 * It MUST NOT invent a token such as:
 *
 *     REQUIREMENT_DECLARATION
 *
 * unless that token is intentionally introduced as a generated parser-layer
 * abstraction, which is not necessary for this architecture.
 *
 * ============================================================================
 * PUBLIC RULES
 * ============================================================================
 *
 * The following rules are public integration points:
 *
 *     requirementDeclaration
 *     requirementExpression
 *     requirementPrimary
 *     requirementReference
 *     requirementReferenceList
 *     optionalRequirementExpression
 *
 * Other rules are implementation details unless explicitly reused by another
 * grammar.
 *
 * ============================================================================
 */

parser grammar Requirements;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * TOP-LEVEL REQUIREMENT DECLARATION
 * ============================================================================
 *
 * Canonical form:
 *
 *     requires quantum::dynamic_control;
 *
 *     requires quantum::dynamic_control and classical::parallel;
 *
 *     requires (
 *         quantum::dynamic_control
 *         or classical::simulation
 *     );
 *
 * The declaration introduces a source-level requirement.
 *
 * It does not allocate or select anything.
 * ============================================================================
 */

requirementDeclaration
    : REQUIRES requirementExpression SEMICOLON
    ;


/* ============================================================================
 * REQUIREMENT EXPRESSION
 * ============================================================================
 *
 * Boolean composition is explicitly represented.
 *
 * Precedence:
 *
 *     not
 *       >
 *     and
 *       >
 *     or
 *
 * Parentheses may override this precedence.
 *
 * This is a syntactic precedence model only.
 * ============================================================================
 */

requirementExpression
    : requirementDisjunction
    ;


requirementDisjunction
    : requirementConjunction
      (OR requirementConjunction)*
    ;


requirementConjunction
    : requirementUnary
      (AND requirementUnary)*
    ;


requirementUnary
    : NOT requirementUnary
    | requirementPrimary
    ;


requirementPrimary
    : requirementReference
    | LPAREN requirementExpression RPAREN
    ;


/* ============================================================================
 * REQUIREMENT REFERENCE
 * ============================================================================
 *
 * A requirement reference is intentionally open-world.
 *
 * It can name:
 *
 *     a capability;
 *     a declared requirement;
 *     a future semantic requirement category.
 *
 * Semantic analysis determines the actual identity.
 *
 * Examples:
 *
 *     quantum::dynamic_control
 *     classical::parallel
 *     resources::persistent_storage
 *     security::trusted_execution
 *     future::capability
 *
 * An optional version clause is consumed when supplied by the canonical
 * versioning grammar.
 * ============================================================================
 */

requirementReference
    : qualifiedName capabilityVersionClause?
    ;


/* ============================================================================
 * REQUIREMENT REFERENCE LIST
 * ============================================================================
 *
 * A non-empty comma-separated list.
 *
 * The list has no finite language-level maximum.
 * ============================================================================
 */

requirementReferenceList
    : requirementReference
      (COMMA requirementReference)*
    ;


/* ============================================================================
 * OPTIONAL REQUIREMENT EXPRESSION
 * ============================================================================
 */

optionalRequirementExpression
    : requirementExpression?
    ;


/* ============================================================================
 * REQUIREMENT EXPRESSION LIST
 * ============================================================================
 *
 * Reusable comma-separated expression list for downstream declaration
 * grammars.
 *
 * Example:
 *
 *     requirementSet(
 *         quantum::measurement,
 *         quantum::dynamic_control
 *     )
 *
 * The surrounding grammar owns the actual invocation/declaration syntax.
 * ============================================================================
 */

requirementExpressionList
    : requirementExpression
      (COMMA requirementExpression)*
    ;


/* ============================================================================
 * OPTIONAL REQUIREMENT EXPRESSION LIST
 * ============================================================================
 */

optionalRequirementExpressionList
    : requirementExpressionList?
    ;


/* ============================================================================
 * REQUIREMENT GROUP
 * ============================================================================
 *
 * Explicit grouping alias.
 *
 * This rule exists as a stable integration point for consumers that need to
 * identify a grouped requirement without duplicating parenthesized expression
 * syntax.
 * ============================================================================
 */

requirementGroup
    : LPAREN requirementExpression RPAREN
    ;


/* ============================================================================
 * REQUIREMENT NEGATION
 * ============================================================================
 *
 * Explicit alias for downstream semantic consumers.
 *
 * Semantic meaning remains outside the grammar.
 * ============================================================================
 */

requirementNegation
    : NOT requirementUnary
    ;


/* ============================================================================
 * REQUIREMENT ALL
 * ============================================================================
 *
 * Syntactic alias for conjunction.
 *
 * This does not imply any evaluation order.
 * ============================================================================
 */

requirementAll
    : requirementConjunction
    ;


/* ============================================================================
 * REQUIREMENT ANY
 * ============================================================================
 *
 * Syntactic alias for disjunction.
 * ============================================================================
 */

requirementAny
    : requirementDisjunction
    ;


/* ============================================================================
 * CAPABILITY-ORIENTED REQUIREMENT
 * ============================================================================
 *
 * A capability requirement is represented structurally as a normal
 * requirementReference.
 *
 * This alias exists so semantic/domain grammars can identify their intent
 * without introducing a second capability identity grammar.
 *
 * ============================================================================
 */

capabilityRequirement
    : requirementReference
    ;


/* ============================================================================
 * NAMED REQUIREMENT
 * ============================================================================
 *
 * Named requirements use the same canonical qualified-name model.
 *
 * Example:
 *
 *     requires application::portable_execution;
 *
 * Whether the referenced name denotes a capability or a named requirement is
 * resolved semantically.
 * ============================================================================
 */

namedRequirement
    : qualifiedName
    ;


/* ============================================================================
 * REQUIREMENT REFERENCE WITH OPTIONAL VERSION
 * ============================================================================
 *
 * Stable explicit wrapper for downstream grammar consumers.
 * ============================================================================
 */

versionedRequirementReference
    : requirementReference
    ;


/* ============================================================================
 * REQUIREMENT SET
 * ============================================================================
 *
 * A set is represented syntactically as a grouped expression.
 *
 * This rule deliberately does not define set semantics such as:
 *
 *     duplicate elimination
 *     ordering
 *     canonicalization
 *     satisfiability
 *
 * Those belong to semantic analysis.
 * ============================================================================
 */

requirementSet
    : requirementGroup
    ;


/* ============================================================================
 * REQUIREMENT PREDICATE
 * ============================================================================
 *
 * Generic semantic predicate boundary.
 *
 * The predicate is represented using an ordinary qualified name.
 *
 * This allows future requirement schemas without modifying this grammar.
 *
 * Example:
 *
 *     requires portability::everywhere;
 *
 *     requires execution::deterministic;
 *
 *     requires quantum::dynamic_control;
 *
 * ============================================================================
 */

requirementPredicate
    : qualifiedName
    ;


/* ============================================================================
 * REQUIREMENT PREDICATE WITH VERSION
 * ============================================================================
 */

versionedRequirementPredicate
    : qualifiedName capabilityVersionClause?
    ;


/* ============================================================================
 * REQUIREMENT LIST
 * ============================================================================
 *
 * Canonical list wrapper for declarations which require one or more
 * requirement expressions.
 * ============================================================================
 */

requirementList
    : requirementExpression
      (COMMA requirementExpression)*
    ;


/* ============================================================================
 * OPTIONAL REQUIREMENT LIST
 * ============================================================================
 */

optionalRequirementList
    : requirementList?
    ;


/* ============================================================================
 * REQUIREMENT CLAUSE
 * ============================================================================
 *
 * This is the reusable non-declaration form.
 *
 * It allows other grammars to attach:
 *
 *     requires <expression>
 *
 * without duplicating the requirement expression itself.
 *
 * The semicolon remains owned by the surrounding declaration/context.
 * ============================================================================
 */

requirementClause
    : REQUIRES requirementExpression
    ;


/* ============================================================================
 * MULTIPLE REQUIREMENT CLAUSES
 * ============================================================================
 *
 * There is intentionally no finite maximum.
 *
 * Semantic analysis may later determine whether repeated clauses are:
 *
 *     equivalent;
 *     additive;
 *     conflicting;
 *     redundant.
 * ============================================================================
 */

requirementClauses
    : requirementClause+
    ;


/* ============================================================================
 * OPTIONAL REQUIREMENT CLAUSES
 * ============================================================================
 */

optionalRequirementClauses
    : requirementClause*
    ;
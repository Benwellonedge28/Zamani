parser grammar ZamaniPolicies;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * Zamani Security Policy Grammar
 * ============================================================================
 *
 * File:
 *     grammar/security/policies.g4
 *
 * Purpose:
 *     Defines the syntactic structure of declarative Zamani security policies.
 *
 * Ownership:
 *     This grammar owns:
 *       - policy declarations
 *       - policy metadata
 *       - policy rules
 *       - policy conditions
 *       - policy scopes
 *       - policy targets
 *       - policy obligations
 *       - policy requirements
 *       - policy composition syntax
 *       - policy-level defaults
 *
 * Does NOT own:
 *       - identities
 *       - credentials
 *       - roles
 *       - permissions
 *       - capability definitions
 *       - cryptographic algorithms
 *       - key management
 *       - trust evaluation
 *       - runtime enforcement
 *       - resource discovery
 *       - hardware topology
 *       - authorization decision algorithms
 *       - policy conflict-resolution semantics
 *
 * Those concerns belong to their respective security/domain layers.
 *
 * Architectural rule:
 *
 *     source
 *       -> lexer
 *       -> parser
 *       -> AST
 *       -> semantic validation
 *       -> authorization/policy model
 *       -> canonical IR
 *       -> security backend/runtime enforcement
 *
 * This grammar therefore describes intent and structure only.
 *
 * Scalability:
 *     No fixed number of policies, rules, subjects, resources, actions,
 *     conditions, obligations, scopes, attributes, or metadata entries is
 *     encoded here.
 *
 * POCO-REAF:
 *     Policy syntax is target-independent. Physical identity, device,
 *     topology, hardware, resource availability, and enforcement mechanisms
 *     are resolved downstream.
 *
 * Hard-coding prohibition:
 *     This grammar MUST NOT encode:
 *
 *       MAX_USERS
 *       MAX_ROLES
 *       MAX_POLICIES
 *       MAX_RULES
 *       MAX_SUBJECTS
 *       MAX_RESOURCES
 *       MAX_ACTIONS
 *       MAX_DEVICES
 *       MAX_NODES
 *       MAX_CPUS
 *       MAX_GPUS
 *       MAX_FPGAS
 *       MAX_QPUS
 *       MAX_MEMORY
 *
 * ============================================================================
 */

/*
 * ----------------------------------------------------------------------------
 * Top-level policy declaration
 * ----------------------------------------------------------------------------
 *
 * Example:
 *
 * policy computation_access
 *     version 1
 *     description "Access policy"
 * {
 *     ...
 * }
 *
 * Policy names are semantic identifiers. Their implementation and storage
 * representation are determined downstream.
 */
policyDeclaration
    : POLICY qualifiedPolicyName policyHeaderItem* LBRACE policyMember* RBRACE
    ;

/*
 * Policy names deliberately use the language's generic naming/path model
 * rather than introducing a security-specific identifier representation.
 */
qualifiedPolicyName
    : Identifier
    | qualifiedName
    ;

/*
 * ----------------------------------------------------------------------------
 * Policy header
 * ----------------------------------------------------------------------------
 *
 * Header items are extensible without changing the fundamental policy model.
 */
policyHeaderItem
    : policyVersion
    | policyDescription
    | policyScope
    | policyMetadata
    | policyAttribute
    ;

/*
 * Version is an expression rather than a fixed numeric representation.
 *
 * This permits semantic versioning, symbolic versions, feature versions,
 * negotiated versions, or implementation-defined version expressions.
 */
policyVersion
    : VERSION expression
    ;

policyDescription
    : DESCRIPTION STRING_LITERAL
    ;

/*
 * ----------------------------------------------------------------------------
 * Policy scope
 * ----------------------------------------------------------------------------
 *
 * Scope describes where/when the policy is intended to apply.
 *
 * The grammar deliberately does not enumerate deployment environments,
 * operating systems, processors, clouds, QPUs, or network types.
 */
policyScope
    : SCOPE expression
    ;

/*
 * Generic policy metadata.
 *
 * Metadata values remain expressions so that semantic validation can decide
 * which values are legal for a particular metadata key.
 */
policyMetadata
    : METADATA Identifier ASSIGN expression
    ;

policyAttribute
    : ATTRIBUTE Identifier ASSIGN expression SEMICOLON
    ;

/*
 * ----------------------------------------------------------------------------
 * Policy body
 * ----------------------------------------------------------------------------
 */
policyMember
    : policyRule
    | policyDefault
    | policyRequirement
    | policyObligationDeclaration
    | policyImport
    ;

/*
 * ----------------------------------------------------------------------------
 * Policy rule
 * ----------------------------------------------------------------------------
 *
 * A rule has:
 *
 *     condition
 *       -> decision/action
 *
 * Optional clauses allow a rule to express subject/resource/action context
 * without forcing every security domain into a separate grammar.
 *
 * Example:
 *
 * when subject == caller
 *     then allow
 *     for resource
 *     action operation
 *     require capability
 *     obligation audit
 * ;
 *
 * The exact meaning is determined by semantic/security layers.
 */
policyRule
    : WHEN expression THEN authorizationAction policyRuleClause* SEMICOLON
    ;

/*
 * ----------------------------------------------------------------------------
 * Rule clauses
 * ----------------------------------------------------------------------------
 */
policyRuleClause
    : policySubjectClause
    | policyResourceClause
    | policyActionClause
    | policyEffectClause
    | policyConditionClause
    | policyRequirement
    | policyObligation
    | policyScope
    | policyPriority
    | policyDuration
    | policyAttribute
    ;

/*
 * Subject is intentionally an expression.
 *
 * This permits:
 *     identity references
 *     role expressions
 *     capability expressions
 *     workload identities
 *     device identities
 *     service identities
 *     composite subjects
 *     dynamically resolved identities
 */
policySubjectClause
    : SUBJECT expression
    ;

/*
 * Resource is intentionally an expression.
 *
 * This keeps the grammar independent from:
 *     files
 *     databases
 *     memory
 *     GPUs
 *     QPUs
 *     hardware
 *     network resources
 *     distributed resources
 *     application resources
 */
policyResourceClause
    : RESOURCE expression
    ;

/*
 * Action is intentionally an expression.
 *
 * This permits generic Zamani operations instead of a fixed action enum.
 */
policyActionClause
    : ACTION expression
    ;

/*
 * Effect is kept explicit because policy semantics commonly distinguish
 * authorization decisions from arbitrary expressions.
 */
policyEffectClause
    : EFFECT authorizationEffect
    ;

/*
 * Additional rule condition.
 *
 * The primary WHEN condition and additional conditions are both semantic
 * expressions. Their combination semantics belong to policy analysis.
 */
policyConditionClause
    : CONDITION expression
    ;

/*
 * ----------------------------------------------------------------------------
 * Authorization effects
 * ----------------------------------------------------------------------------
 *
 * ALLOW/DENY are intentionally the minimum stable authorization effects.
 *
 * More complex outcomes such as:
 *
 *     ACCEPT
 *     DEGRADED_ACCEPT
 *     RETRY
 *     RECOVER
 *     ESCALATE
 *     REJECT
 *
 * belong to the resilience/execution model and must not be silently conflated
 * with authorization effects.
 */
authorizationEffect
    : ALLOW
    | DENY
    ;

/*
 * ----------------------------------------------------------------------------
 * Policy requirements
 * ----------------------------------------------------------------------------
 *
 * REQUIRE expresses a semantic prerequisite.
 *
 * Examples:
 *
 *     require capability("quantum.measurement")
 *     require capability("network.secure")
 *     require trust >= threshold
 *
 * No resource capacity is hard-coded here.
 */
policyRequirement
    : REQUIRE expression SEMICOLON
    ;

/*
 * A requirement may also occur as a rule clause, where the terminating
 * semicolon belongs to the enclosing policyRule.
 */
policyRequirementClause
    : REQUIRE expression
    ;

/*
 * ----------------------------------------------------------------------------
 * Obligations
 * ----------------------------------------------------------------------------
 *
 * Obligations describe work that must accompany a policy decision.
 *
 * Examples:
 *
 *     audit(...)
 *     notify(...)
 *     record(...)
 *
 * The grammar does not prescribe the implementation.
 */
policyObligationDeclaration
    : OBLIGATION Identifier ASSIGN expression SEMICOLON
    ;

policyObligation
    : OBLIGATION expression
    ;

/*
 * ----------------------------------------------------------------------------
 * Default policy behavior
 * ----------------------------------------------------------------------------
 *
 * Defaults are policy semantics, not global language semantics.
 *
 * Example:
 *
 *     default deny;
 *
 * Conflict resolution between explicit rules and defaults belongs to
 * semantic validation/policy evaluation.
 */
policyDefault
    : DEFAULT authorizationEffect SEMICOLON
    ;

/*
 * ----------------------------------------------------------------------------
 * Priority
 * ----------------------------------------------------------------------------
 *
 * Priority is an expression rather than a fixed integer type.
 *
 * The semantic layer determines whether and how priorities participate in
 * conflict resolution.
 */
policyPriority
    : PRIORITY expression
    ;

/*
 * ----------------------------------------------------------------------------
 * Duration / temporal applicability
 * ----------------------------------------------------------------------------
 *
 * No timestamp width, clock model, or maximum duration is encoded.
 */
policyDuration
    : DURATION expression
    ;

/*
 * ----------------------------------------------------------------------------
 * Policy imports
 * ----------------------------------------------------------------------------
 *
 * Policy composition may reference policy modules without embedding module
 * loading semantics in the grammar.
 */
policyImport
    : IMPORT qualifiedName SEMICOLON
    ;
/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/core/policies.g4
 *
 * Grammar:
 *     ANTLR4 parser grammar
 *
 * Grammar name:
 *     ZamaniPolicies
 *
 * Status:
 *     Canonical universal policy syntax
 *
 * Implementation baseline:
 *     Rust 2021
 *     Rust 1.97
 *     Rust 1.97.1
 *     Safe Rust only
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file defines the UNIVERSAL, DOMAIN-NEUTRAL source syntax for
 * declarative policies in Zamani.
 *
 * A policy expresses governing intent over one or more semantic constructs.
 *
 * A policy may describe:
 *
 *     requirements
 *     prohibitions
 *     permissions
 *     preferences
 *     obligations
 *     fallbacks
 *     applicability
 *     conditions
 *     priorities
 *     temporal scope
 *     composition
 *     inheritance
 *     metadata
 *     capabilities
 *     effects
 *     resources
 *     contracts
 *     execution behavior
 *     adaptation behavior
 *
 * Policy syntax is intentionally generic.
 *
 * A policy is NOT:
 *
 *     a resource allocator;
 *     a hardware selector;
 *     a scheduler;
 *     a router;
 *     a quantum compiler;
 *     a QEC implementation;
 *     a security authorization engine;
 *     an AI reasoning engine;
 *     a runtime;
 *     an IR;
 *     a backend.
 *
 * The grammar records source-level policy intent.
 *
 * Semantic analysis determines the meaning, compatibility, conflict
 * resolution, applicability, authorization, satisfiability, and enforcement
 * behavior of that intent.
 *
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * The policy pipeline is:
 *
 *     source
 *       |
 *       v
 *     lexer
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic policy model
 *       |
 *       +--> requirements
 *       +--> constraints
 *       +--> capabilities
 *       +--> effects
 *       +--> resources
 *       +--> contracts
 *       +--> security
 *       +--> execution
 *       +--> adaptation
 *       +--> deployment
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> HDL/hardware representation
 *       +--> distributed representation
 *       +--> other domain IRs
 *       |
 *       v
 *     target realization
 *
 * This file participates only in the source/parser portion of that pipeline.
 *
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns:
 *
 *     policyDeclaration
 *     policyHeader
 *     policyHeaderItem
 *     policyVersion
 *     policyDescription
 *     policyScope
 *     policyMetadata
 *     policyAttribute
 *
 *     policyBody
 *     policyMember
 *
 *     policyRule
 *     policyRuleCondition
 *     policyRuleAction
 *     policyRuleClause
 *
 *     policyRequirement
 *     policyProhibition
 *     policyPermission
 *     policyPreference
 *     policyObligation
 *     policyFallback
 *
 *     policyWhen
 *     policyUnless
 *     policyIf
 *     policyFor
 *     policyOn
 *     policyWithin
 *
 *     policyPriority
 *     policyDuration
 *     policyOrdering
 *
 *     policyComposition
 *     policyExtends
 *     policyIncludes
 *     policyExcludes
 *     policyOverride
 *
 *     policyBinding
 *     policyTarget
 *     policySubject
 *     policyResource
 *     policyAction
 *     policyCondition
 *     policyValue
 *
 *     policyDecision
 *     policyOutcome
 *
 *     policyExpression
 *     policyExpressionList
 *     policyNamedArgument
 *     policyArgumentList
 *
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     lexical tokens
 *     identifiers
 *     qualified names
 *     general expressions
 *     literals
 *     types
 *     generic parameters
 *     capabilities
 *     requirements
 *     constraints
 *     resources
 *     effects
 *     contracts
 *     permissions as a security authorization implementation
 *     identities
 *     credentials
 *     roles
 *     trust
 *     cryptography
 *     key management
 *     security enforcement
 *     hardware discovery
 *     device selection
 *     topology
 *     routing
 *     scheduling
 *     calibration
 *     QEC
 *     ZQN
 *     HAL
 *     quantum operations
 *     quantum::ir
 *     HDL implementation
 *     runtime enforcement
 *     target realization
 *
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/core/names.g4
 *     grammar/core/attributes.g4
 *     grammar/core/versioning.g4
 *     grammar/core/requirements.g4
 *     grammar/core/constraints.g4
 *     grammar/core/capabilities.g4
 *     grammar/expressions/
 *
 *
 * EXPORTS
 * -------
 *
 * Primary:
 *
 *     policyDeclaration
 *     policyHeader
 *     policyBody
 *     policyMember
 *     policyRule
 *     policyRuleClause
 *     policyRequirement
 *     policyProhibition
 *     policyPermission
 *     policyPreference
 *     policyObligation
 *     policyFallback
 *
 * Secondary:
 *
 *     policyScope
 *     policyMetadata
 *     policyAttribute
 *     policyCondition
 *     policySubject
 *     policyResource
 *     policyAction
 *     policyTarget
 *     policyDecision
 *     policyOutcome
 *     policyComposition
 *     policyBinding
 *
 *
 * CONSUMED_BY
 * -----------
 *
 *     grammar/core/source-unit.g4
 *     grammar/core/program.g4
 *     grammar/antlr/ZamaniParser.g4
 *     grammar/declarations/
 *     grammar/modules/
 *     grammar/functions/
 *     grammar/contracts/
 *     grammar/validation/
 *     grammar/resources/
 *     grammar/effects/
 *     grammar/execution/
 *     grammar/security/
 *     grammar/quantum/
 *     grammar/hybrid/
 *     grammar/hdl/
 *     grammar/hardware/
 *     grammar/distributed/
 *     grammar/ai/
 *     grammar/networking/
 *     grammar/interoperability/
 *     grammar/dialects/
 *     grammar/metaprogramming/
 *
 *
 * AST_OWNER
 * ---------
 *
 * Domain-neutral frontend AST.
 *
 * The parser creates parser contexts only.
 *
 * The Rust AST layer is responsible for constructing domain-neutral policy
 * nodes while preserving:
 *
 *     policy identity
 *     declaration order
 *     rule order
 *     policy composition
 *     expressions
 *     source spans
 *     attributes
 *     metadata
 *     provenance-relevant structure
 *
 *
 * SEMANTIC_OWNER
 * --------------
 *
 * Canonical semantic policy subsystem.
 *
 * Semantic analysis is responsible for:
 *
 *     name resolution
 *     policy identity resolution
 *     scope resolution
 *     applicability
 *     requirement resolution
 *     constraint evaluation
 *     capability resolution
 *     conflict detection
 *     precedence
 *     composition
 *     inheritance
 *     authorization interaction
 *     resource interaction
 *     effect interaction
 *     contract interaction
 *     execution interaction
 *     adaptation authorization
 *     provenance
 *     diagnostics
 *
 *
 * IR_OWNER
 * --------
 *
 * This grammar owns no IR.
 *
 * Policy semantics may be represented as metadata and semantic constraints
 * consumed by the canonical semantic representation and downstream domain
 * IRs.
 *
 * A policy MUST NOT directly lower to:
 *
 *     machine instructions
 *     quantum gates
 *     physical signals
 *     routing commands
 *     scheduler commands
 *     QEC operations
 *     device operations
 *
 *
 * TEST_OWNER
 * ----------
 *
 * Repository test locations should follow the existing test hierarchy.
 *
 * Recommended coverage:
 *
 *     grammar/tests/parser/
 *     grammar/tests/semantic/
 *     grammar/tests/policies/
 *     grammar/tests/security/
 *     grammar/tests/resources/
 *     grammar/tests/effects/
 *     grammar/tests/execution/
 *     grammar/tests/quantum/
 *     grammar/tests/hardware/
 *     grammar/tests/distributed/
 *     grammar/tests/ai/
 *     grammar/tests/scalability/
 *     grammar/tests/portability/
 *     grammar/tests/determinism/
 *     grammar/tests/compatibility/
 *
 *
 * SPEC_OWNER
 * ----------
 *
 *     grammar/spec/policies.md
 *     grammar/spec/resources.md
 *     grammar/spec/effects.md
 *     grammar/specification/poco-reaf.md
 *
 *
 * ============================================================================
 * POLICY MODEL
 * ============================================================================
 *
 * Policy is a GOVERNING abstraction.
 *
 * The core distinction is:
 *
 *     REQUIRE
 *         mandatory condition
 *
 *     FORBID
 *         prohibited behavior
 *
 *     ALLOW
 *         permitted behavior
 *
 *     PREFER
 *         preferred behavior among otherwise valid alternatives
 *
 *     OBLIGATE
 *         behavior that must accompany a policy decision
 *
 *     FALLBACK
 *         permitted alternative when the primary realization cannot satisfy
 *         the selected policy path
 *
 * These meanings are semantic.
 *
 * The grammar only preserves their structure.
 *
 *
 * ============================================================================
 * POLICY IS NOT AUTHORIZATION
 * ============================================================================
 *
 * The universal policy language may express permission and prohibition.
 *
 * However:
 *
 *     policy permission
 *
 * is not itself equivalent to:
 *
 *     security authorization
 *
 * Security identity, credentials, roles, trust, authorization decisions, and
 * enforcement remain owned by grammar/security/.
 *
 * The security subsystem may consume this policy representation.
 *
 * The core policy grammar MUST therefore remain usable for:
 *
 *     optimization policies
 *     resource policies
 *     execution policies
 *     deployment policies
 *     adaptation policies
 *     simulation policies
 *     compiler policies
 *     AI policies
 *     quantum policies
 *     hardware policies
 *     distributed policies
 *     data policies
 *     networking policies
 *     security policies
 *
 *
 * ============================================================================
 * POLICY IS NOT A CAPABILITY
 * ============================================================================
 *
 * A capability says:
 *
 *     what an environment can provide.
 *
 * A policy says:
 *
 *     what governing rules permit, prohibit, require, or prefer.
 *
 * Therefore:
 *
 *     capability
 *
 * does not imply:
 *
 *     permission.
 *
 * And:
 *
 *     permission
 *
 * does not imply:
 *
 *     capability availability.
 *
 * The semantic layer must keep these distinctions explicit.
 *
 *
 * ============================================================================
 * POLICY IS NOT A RESOURCE
 * ============================================================================
 *
 * Resource syntax remains owned by:
 *
 *     grammar/resources/
 *
 * A policy can refer to resources through generic policy expressions, but
 * this file does not define resource quantities or resource allocation.
 *
 * Examples of semantic intent:
 *
 *     prefer resource::latency;
 *
 *     require resource::memory >= required_memory;
 *
 *     forbid resource::untrusted;
 *
 * The exact resource expression is resolved by the resource subsystem.
 *
 *
 * ============================================================================
 * POLICY IS NOT A CONTRACT
 * ============================================================================
 *
 * Contracts remain owned by:
 *
 *     grammar/validation/
 *
 * A policy can govern how contracts are applied, but it does not redefine:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * Contract syntax remains owned by its canonical subsystem.
 *
 *
 * ============================================================================
 * OPEN-WORLD CONTRACT
 * ============================================================================
 *
 * Policy identities are OPEN-WORLD.
 *
 * The grammar MUST NOT enumerate:
 *
 *     operating systems
 *     CPU models
 *     GPU models
 *     FPGA models
 *     ASIC models
 *     QPU models
 *     quantum gates
 *     AI models
 *     databases
 *     cloud providers
 *     network vendors
 *     hardware vendors
 *     accelerator models
 *     deployment platforms
 *     future architectures
 *
 * A policy may therefore refer to:
 *
 *     execution::deterministic
 *     quantum::fault_tolerant
 *     hardware::reconfigurable
 *     resource::latency
 *     security::trusted_execution
 *     learning::adaptation
 *     provenance::tracking
 *     future::computing::property
 *
 * without requiring a grammar modification for every new semantic domain.
 *
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Policies MUST remain target-independent by default.
 *
 * A policy may describe:
 *
 *     requirements
 *     constraints
 *     preferences
 *     prohibitions
 *     permitted alternatives
 *     execution intent
 *     adaptation intent
 *
 * without selecting physical resources.
 *
 * Therefore a policy may remain unchanged while the implementation realizes
 * the same semantic intent using:
 *
 *     an embedded processor
 *     a CPU
 *     a multicore CPU
 *     a GPU
 *     an FPGA
 *     an ASIC
 *     an accelerator
 *     a QPU
 *     a simulator
 *     an HPC system
 *     a cluster
 *     a distributed system
 *     a cloud system
 *     a heterogeneous system
 *     a future computational substrate
 *
 * provided semantic requirements and constraints remain satisfied.
 *
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar imposes NO language-level finite limit on:
 *
 *     number of policies
 *     number of rules
 *     number of policy members
 *     number of clauses
 *     number of conditions
 *     number of requirements
 *     number of prohibitions
 *     number of permissions
 *     number of preferences
 *     number of obligations
 *     number of fallbacks
 *     number of scopes
 *     number of metadata entries
 *     number of attributes
 *     number of composition entries
 *     policy nesting
 *     qualified-name depth
 *     expression size
 *
 * ANTLR repetition operators are used instead of fixed cardinalities.
 *
 * Any practical limit is an implementation/resource limit rather than a
 * Zamani language limit.
 *
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This file MUST NOT introduce:
 *
 *     MAX_POLICIES
 *     MAX_RULES
 *     MAX_SUBJECTS
 *     MAX_RESOURCES
 *     MAX_ACTIONS
 *     MAX_TARGETS
 *     MAX_REQUIREMENTS
 *     MAX_FALLBACKS
 *     MAX_ATTRIBUTES
 *     MAX_CAPABILITIES
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * It also MUST NOT indirectly impose equivalent limits through grammar
 * cardinality.
 *
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no embedded actions
 *     no semantic predicates
 *     no runtime calls
 *     no filesystem access
 *     no network access
 *     no hardware discovery
 *     no randomness
 *     no environment inspection
 *
 * Parsing therefore depends only on:
 *
 *     source tokens
 *     grammar definition
 *     parser configuration
 *
 * Policy evaluation is downstream and may have additional determinism
 * requirements defined by the semantic/runtime policy model.
 *
 *
 * ============================================================================
 * SOURCE PRESERVATION CONTRACT
 * ============================================================================
 *
 * The frontend must preserve:
 *
 *     policy declaration order
 *     member order
 *     rule order
 *     clause order
 *     grouping
 *     expressions
 *     metadata
 *     attributes
 *     composition structure
 *     source spans
 *
 * Semantic normalization must not erase source provenance.
 *
 *
 * ============================================================================
 * POLICY DECLARATION
 * ============================================================================
 *
 * Canonical form:
 *
 *     policy name {
 *         ...
 *     }
 *
 * Optional header information may appear between the name and body.
 *
 * The policy name is a generic qualified name.
 *
 * No policy namespace is reserved for any particular hardware, vendor,
 * domain, or application.
 *
 * ============================================================================
 */

parser grammar ZamaniPolicies;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * Reuse canonical generic authorities.
 *
 * Names:
 *     identifier
 *     qualifiedName
 *
 * Attributes:
 *     attributes
 *
 * Versioning:
 *     versionExpression
 *
 * Requirements:
 *     requirementExpression
 *
 * Constraints:
 *     constraintExpression
 *
 * Capabilities:
 *     capabilityExpression
 *
 * General expressions:
 *     expression
 */
import Names, Attributes, Versioning, Requirements, Constraints, Capabilities;


/* ============================================================================
 * 1. POLICY DECLARATION
 * ============================================================================
 */

policyDeclaration
    : POLICY
      qualifiedName
      policyHeader*
      LBRACE
      policyMember*
      RBRACE
    ;


/* ============================================================================
 * 2. POLICY HEADER
 * ============================================================================
 *
 * Header members describe the policy itself.
 *
 * They do not execute the policy.
 * ========================================================================== */

policyHeader
    : policyVersion
    | policyDescription
    | policyScope
    | policyMetadata
    | policyAttribute
    ;


policyVersion
    : VERSION versionExpression
    ;


policyDescription
    : DESCRIPTION STRING_LITERAL
    ;


policyScope
    : SCOPE expression
    ;


policyMetadata
    : METADATA identifier ASSIGN expression SEMICOLON
    ;


policyAttribute
    : attributes
    ;


/* ============================================================================
 * 3. POLICY BODY
 * ============================================================================
 */

policyBody
    : LBRACE policyMember* RBRACE
    ;


policyMember
    : policyRule
    | policyRequirement
    | policyProhibition
    | policyPermission
    | policyPreference
    | policyObligation
    | policyFallback
    | policyComposition
    | policyBinding
    ;


/* ============================================================================
 * 4. POLICY RULE
 * ============================================================================
 *
 * A rule combines a condition with one or more governing actions.
 *
 * Example:
 *
 *     when condition
 *         allow action
 *     ;
 *
 * More elaborate policy semantics are represented by clauses.
 *
 * The grammar deliberately avoids embedding an authorization algorithm.
 * ========================================================================== */

policyRule
    : WHEN policyRuleCondition
      policyRuleAction
      policyRuleClause*
      SEMICOLON
    ;


policyRuleCondition
    : expression
    ;


policyRuleAction
    : policyDecision
    ;


policyRuleClause
    : policyWhenClause
    | policyUnlessClause
    | policyIfClause
    | policyForClause
    | policyOnClause
    | policyWithinClause
    | policySubjectClause
    | policyResourceClause
    | policyActionClause
    | policyTargetClause
    | policyConditionClause
    | policyRequirementClause
    | policyConstraintClause
    | policyCapabilityClause
    | policyPriority
    | policyDuration
    | policyAttributeClause
    ;


/* ============================================================================
 * 5. APPLICABILITY CLAUSES
 * ============================================================================
 *
 * These clauses refine when a policy member applies.
 *
 * They are intentionally expressions rather than enumerated environments.
 * ========================================================================== */

policyWhenClause
    : WHEN expression
    ;


policyUnlessClause
    : UNLESS expression
    ;


policyIfClause
    : IF expression
    ;


policyForClause
    : FOR expression
    ;


policyOnClause
    : ON expression
    ;


policyWithinClause
    : WITHIN expression
    ;


/* ============================================================================
 * 6. POLICY SUBJECT
 * ============================================================================
 *
 * Subject is a semantic expression.
 *
 * It may represent:
 *
 *     caller
 *     computation
 *     task
 *     service
 *     actor
 *     model
 *     process
 *     resource
 *     abstract execution context
 *
 * The security subsystem may attach identity semantics later.
 * ========================================================================== */

policySubjectClause
    : SUBJECT expression
    ;


policySubject
    : policySubjectClause
    ;


/* ============================================================================
 * 7. POLICY RESOURCE
 * ============================================================================
 *
 * Resource remains an expression.
 *
 * This grammar does not define resource quantities or physical resources.
 * ========================================================================== */

policyResourceClause
    : RESOURCE expression
    ;


policyResource
    : policyResourceClause
    ;


/* ============================================================================
 * 8. POLICY ACTION
 * ============================================================================
 *
 * Action is intentionally generic.
 *
 * The language does not maintain an ever-growing list of actions for every
 * future computing domain.
 * ========================================================================== */

policyActionClause
    : ACTION expression
    ;


policyAction
    : policyActionClause
    ;


/* ============================================================================
 * 9. POLICY TARGET
 * ============================================================================
 *
 * Target here means an abstract semantic target/context.
 *
 * It does NOT mean a physical device.
 * ========================================================================== */

policyTargetClause
    : TARGET expression
    ;


policyTarget
    : policyTargetClause
    ;


/* ============================================================================
 * 10. POLICY CONDITIONS
 * ============================================================================
 */

policyConditionClause
    : CONDITION expression
    ;


policyCondition
    : policyConditionClause
    ;


/* ============================================================================
 * 11. REQUIREMENT
 * ============================================================================
 *
 * Universal requirement syntax remains owned by core/requirements.g4.
 *
 * This policy wrapper consumes that semantic structure without redefining
 * requirement syntax.
 *
 * The terminating semicolon belongs to the policy member.
 * ========================================================================== */

policyRequirement
    : REQUIRE requirementExpression SEMICOLON
    ;


policyRequirementClause
    : REQUIRE requirementExpression
    ;


/* ============================================================================
 * 12. PROHIBITION
 * ============================================================================
 *
 * FORBID expresses a semantic prohibition.
 *
 * The forbidden thing remains an expression.
 *
 * It can therefore represent future domains without adding new grammar rules.
 * ========================================================================== */

policyProhibition
    : FORBID policyExpression SEMICOLON
    ;


policyProhibitionClause
    : FORBID policyExpression
    ;


/* ============================================================================
 * 13. PERMISSION
 * ============================================================================
 *
 * ALLOW expresses permitted behavior.
 *
 * Security-specific authorization semantics remain downstream.
 * ========================================================================== */

policyPermission
    : ALLOW policyExpression SEMICOLON
    ;


policyPermissionClause
    : ALLOW policyExpression
    ;


/* ============================================================================
 * 14. PREFERENCE
 * ============================================================================
 *
 * PREFER is advisory unless semantic policy explicitly promotes it to a
 * stronger category.
 *
 * A preference MUST NOT silently become a requirement.
 * ========================================================================== */

policyPreference
    : PREFER policyExpression SEMICOLON
    ;


policyPreferenceClause
    : PREFER policyExpression
    ;


/* ============================================================================
 * 15. OBLIGATION
 * ============================================================================
 *
 * OBLIGATE expresses work that must accompany an applicable policy decision.
 *
 * The grammar does not define the implementation of the obligation.
 * ========================================================================== */

policyObligation
    : OBLIGATE policyExpression SEMICOLON
    ;


policyObligationClause
    : OBLIGATE policyExpression
    ;


/* ============================================================================
 * 16. FALLBACK
 * ============================================================================
 *
 * A fallback represents an explicitly permitted alternative policy path.
 *
 * Fallback selection is semantic/execution policy, not parser behavior.
 * ========================================================================== */

policyFallback
    : FALLBACK policyExpression SEMICOLON
    ;


policyFallbackClause
    : FALLBACK policyExpression
    ;


/* ============================================================================
 * 17. POLICY DECISION
 * ============================================================================
 *
 * These are universal policy decisions.
 *
 * ACCEPT/RETRY/RECOVER/ESCALATE/REJECT are deliberately NOT included here.
 *
 * Those outcomes belong to execution/resilience semantics and must not be
 * conflated with policy authorization.
 * ========================================================================== */

policyDecision
    : policyPermission
    | policyProhibition
    | policyPreference
    | policyObligation
    | policyFallback
    | policyDecisionReference
    ;


policyDecisionReference
    : POLICY expression
    ;


/* ============================================================================
 * 18. POLICY OUTCOME
 * ============================================================================
 *
 * A generic outcome reference permits domain-specific semantic outcomes
 * without creating a fixed universal outcome enumeration.
 * ========================================================================== */

policyOutcome
    : OUTCOME expression
    ;


/* ============================================================================
 * 19. PRIORITY
 * ============================================================================
 *
 * Priority is an expression.
 *
 * It is deliberately not restricted to a fixed integer width or range.
 * ========================================================================== */

policyPriority
    : PRIORITY expression
    ;


/* ============================================================================
 * 20. DURATION
 * ============================================================================
 *
 * Duration is an expression.
 *
 * No timestamp width, clock frequency, or maximum duration is encoded.
 * ========================================================================== */

policyDuration
    : DURATION expression
    ;


/* ============================================================================
 * 21. POLICY CONSTRAINT
 * ============================================================================
 *
 * Constraint semantics remain owned by core/constraints.g4.
 * ========================================================================== */

policyConstraintClause
    : CONSTRAIN constraintExpression
    ;


/* ============================================================================
 * 22. POLICY CAPABILITY
 * ============================================================================
 *
 * Capability semantics remain owned by core/capabilities.g4.
 *
 * A capability is not a permission.
 * ========================================================================== */

policyCapabilityClause
    : CAPABILITY capabilityExpression
    ;


/* ============================================================================
 * 23. POLICY ATTRIBUTE
 * ============================================================================
 */

policyAttributeClause
    : ATTRIBUTE identifier ASSIGN expression
    ;


/* ============================================================================
 * 24. POLICY COMPOSITION
 * ============================================================================
 *
 * Composition references policies rather than copying their contents into
 * the grammar.
 * ========================================================================== */

policyComposition
    : policyExtends
    | policyIncludes
    | policyExcludes
    | policyOverride
    ;


policyExtends
    : EXTENDS qualifiedName SEMICOLON
    ;


policyIncludes
    : INCLUDES qualifiedName SEMICOLON
    ;


policyExcludes
    : EXCLUDES qualifiedName SEMICOLON
    ;


policyOverride
    : OVERRIDE qualifiedName SEMICOLON
    ;


/* ============================================================================
 * 25. POLICY BINDING
 * ============================================================================
 *
 * Binding associates a policy with an abstract semantic subject, resource,
 * action, target, or other expression.
 *
 * It does not perform runtime binding.
 * ========================================================================== */

policyBinding
    : BIND policyBindingSubject TO qualifiedName SEMICOLON
    ;


policyBindingSubject
    : expression
    ;


/* ============================================================================
 * 26. POLICY EXPRESSION
 * ============================================================================
 *
 * General expressions remain owned by grammar/expressions/.
 *
 * This bridge provides a stable policy-specific exported rule without
 * duplicating expression precedence.
 * ========================================================================== */

policyExpression
    : expression
    ;


policyExpressionList
    : policyExpression
      (COMMA policyExpression)*
    ;


policyNamedArgument
    : identifier ASSIGN policyExpression
    ;


policyArgument
    : policyNamedArgument
    | policyExpression
    ;


policyArgumentList
    : policyArgument
      (COMMA policyArgument)*
    ;


/* ============================================================================
 * 27. REUSABLE POLICY BLOCK
 * ============================================================================
 *
 * Some consumers need to attach a policy body without redeclaring policy
 * syntax.
 * ========================================================================== */

policyBlock
    : LBRACE policyMember* RBRACE
    ;


/* ============================================================================
 * 28. SEMANTIC BRIDGES
 * ============================================================================
 *
 * These bridges intentionally do not duplicate the semantic systems that they
 * consume.
 * ========================================================================== */

policyRequirementReference
    : requirementExpression
    ;


policyConstraintReference
    : constraintExpression
    ;


policyCapabilityReference
    : capabilityExpression
    ;


/* ============================================================================
 * 29. OPTIONAL POLICY
 * ============================================================================
 */

optionalPolicy
    : policyDeclaration?
    ;


policyList
    : policyDeclaration*
    ;


/* ============================================================================
 * 30. QUANTUM BOUNDARY
 * ============================================================================
 *
 * Policy syntax remains quantum-neutral.
 *
 * A policy may refer semantically to:
 *
 *     quantum capabilities
 *     quantum requirements
 *     quantum resources
 *     fault tolerance
 *     measurement
 *     resilience
 *     simulation
 *     adaptation
 *
 * but this grammar does NOT define:
 *
 *     qubits
 *     gates
 *     physical qubits
 *     coupling maps
 *     routing
 *     scheduling
 *     calibration
 *     QEC
 *     ZQN
 *     HAL
 *
 * Those remain downstream.
 *
 * A policy affecting quantum compilation can therefore influence:
 *
 *     semantic model
 *         ->
 *     quantum::ir
 *         ->
 *     optimization
 *         ->
 *     decomposition
 *         ->
 *     routing
 *         ->
 *     scheduling
 *         ->
 *     resilience/QEC
 *         ->
 *     ZQN
 *         ->
 *     HAL
 *
 * without this grammar becoming a quantum grammar.
 *
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Policies may govern hardware intent but do not describe physical hardware.
 *
 * The grammar MUST NOT encode:
 *
 *     fixed signal widths
 *     fixed register widths
 *     device counts
 *     processor counts
 *     memory capacities
 *     FPGA resources
 *     ASIC resources
 *     physical placement
 *
 *
 * ============================================================================
 * AI BOUNDARY
 * ============================================================================
 *
 * Policies can govern:
 *
 *     reasoning
 *     learning
 *     adaptation
 *     model execution
 *     agent behavior
 *     uncertainty handling
 *     provenance
 *     explainability
 *     simulation
 *
 * The AI subsystem owns those semantics.
 *
 * This grammar does not enumerate AI algorithms or model families.
 *
 *
 * ============================================================================
 * EFFECT BOUNDARY
 * ============================================================================
 *
 * Policy expressions may refer to effects.
 *
 * Effects remain owned by grammar/effects/.
 *
 * A policy MUST NOT silently create an effect merely because it mentions an
 * action.
 *
 * Semantic analysis determines the relationship between:
 *
 *     policy
 *     action
 *     effect
 *     capability
 *     authorization
 *
 *
 * ============================================================================
 * RESOURCE BOUNDARY
 * ============================================================================
 *
 * Policy expressions may refer to resources.
 *
 * Resource quantities, units, relationships, allocation, discovery, and
 * realization remain owned by grammar/resources/ and downstream semantic
 * resource infrastructure.
 *
 * This prevents policy grammar from becoming a second resource language.
 *
 *
 * ============================================================================
 * SECURITY BOUNDARY
 * ============================================================================
 *
 * grammar/security/policies.g4 remains responsible for security-specific
 * policy constructs.
 *
 * That grammar may consume the generic policy abstraction defined here.
 *
 * It MUST NOT be necessary to duplicate universal policy semantics in the
 * security grammar.
 *
 * Conversely, this file MUST NOT import or duplicate:
 *
 *     credentials
 *     roles
 *     identities
 *     trust
 *     cryptography
 *     key management
 *     security enforcement
 *
 *
 * ============================================================================
 * EXECUTION BOUNDARY
 * ============================================================================
 *
 * Policies can influence execution planning.
 *
 * They do not perform execution.
 *
 * The downstream path is:
 *
 *     policy
 *       ->
 *     semantic validation
 *       ->
 *     policy evaluation/normalization
 *       ->
 *     execution planning
 *       ->
 *     target realization
 *
 *
 * ============================================================================
 * ADAPTATION BOUNDARY
 * ============================================================================
 *
 * A policy may govern adaptation.
 *
 * Adaptation must remain controlled by:
 *
 *     policy
 *     capability
 *     effects
 *     contracts
 *     authorization
 *     provenance
 *
 * This grammar does not permit unrestricted self-modifying execution.
 *
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Policy source structure must remain traceable.
 *
 * Downstream provenance should be able to associate decisions with:
 *
 *     policy identity
 *     policy version
 *     rule
 *     condition
 *     action
 *     requirement
 *     constraint
 *     capability
 *     evidence
 *     source span
 *     semantic transformation
 *     execution decision
 *
 * This grammar itself does not generate provenance records.
 *
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * The core policy model must remain extensible.
 *
 * New policy categories should normally be represented through:
 *
 *     existing policy action structures
 *     qualified names
 *     expressions
 *     attributes
 *     capabilities
 *     requirements
 *     constraints
 *
 * rather than introducing a new keyword for every future policy concept.
 *
 * New semantic policy kinds should therefore normally require:
 *
 *     semantic model
 *     registry/schema
 *     AST mapping
 *     tests
 *
 * rather than modification of this grammar.
 *
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics should identify structural errors such as:
 *
 *     missing policy name
 *     missing policy body
 *     missing closing brace
 *     missing policy expression
 *     malformed policy composition
 *     malformed requirement
 *     malformed prohibition
 *     malformed permission
 *     malformed preference
 *     malformed obligation
 *     malformed fallback
 *     malformed policy binding
 *     malformed policy clause
 *
 * Semantic diagnostics belong downstream:
 *
 *     unknown policy
 *     unknown capability
 *     unavailable capability
 *     unsatisfied requirement
 *     conflicting policy
 *     forbidden policy
 *     invalid scope
 *     invalid inheritance
 *     invalid override
 *     authorization failure
 *     resource failure
 *     effect conflict
 *     target infeasibility
 *
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The parser must reject malformed forms such as:
 *
 *     policy;
 *
 *     policy name
 *
 *     policy name {
 *
 *     policy name { allow; }
 *
 *     policy name { forbid; }
 *
 *     policy name { prefer; }
 *
 *     policy name { obligate; }
 *
 *     policy name { fallback; }
 *
 *     policy name { extends; }
 *
 *     policy name { includes; }
 *
 *     policy name { excludes; }
 *
 *     policy name { override; }
 *
 *     policy name { bind to; }
 *
 *     policy name { priority; }
 *
 *     policy name { duration; }
 *
 * where the required expression or qualified name is absent.
 *
 *
 * ============================================================================
 * POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * At minimum, conformance tests should cover:
 *
 *     policy computation {
 *     }
 *
 *     policy execution {
 *         require execution::deterministic;
 *     }
 *
 *     policy security {
 *         forbid security::untrusted_execution;
 *     }
 *
 *     policy optimization {
 *         prefer resource::latency;
 *     }
 *
 *     policy execution {
 *         allow execution::parallel;
 *     }
 *
 *     policy resilience {
 *         obligate provenance::tracking;
 *     }
 *
 *     policy recovery {
 *         fallback execution::simulation;
 *     }
 *
 *     policy execution {
 *         when execution::deterministic
 *         allow execution::stable
 *         ;
 *     }
 *
 *     policy execution {
 *         priority priority_value;
 *     }
 *
 *     policy execution {
 *         extends base::execution;
 *     }
 *
 *     policy execution {
 *         includes common::provenance;
 *     }
 *
 *     policy execution {
 *         excludes legacy::behavior;
 *     }
 *
 *     policy execution {
 *         override implementation::policy;
 *     }
 *
 *     policy execution {
 *         bind workload to execution::policy;
 *     }
 *
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests must exercise:
 *
 *     many policies
 *     many rules
 *     many clauses
 *     deeply nested expressions
 *     long qualified names
 *     many composition relationships
 *     large metadata sets
 *     large attribute sets
 *     symbolic policy expressions
 *     dynamic policy expressions
 *
 * No test fixture may define its chosen size as a language maximum.
 *
 *
 * ============================================================================
 * CROSS-DOMAIN TEST CONTRACT
 * ============================================================================
 *
 * Policies must be tested with:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     AI
 *     reasoning
 *     learning
 *     adaptation
 *     uncertainty
 *     agents
 *     distributed
 *     networking
 *     data
 *     security
 *     FFI
 *     ABI
 *     simulation
 *     metaprogramming
 *
 * without changing the universal policy grammar merely because the domain
 * changes.
 *
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains:
 *
 *     no universal hardware limits
 *     no quantum limits
 *     no processor limits
 *     no memory limits
 *     no thread limits
 *     no tensor limits
 *     no node limits
 *     no device limits
 *     no vendor catalogue
 *     no finite action catalogue
 *     no finite policy catalogue
 *
 * All collections use unbounded repetition.
 *
 *
 * ============================================================================
 * SAFE-RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust code.
 *
 * Generated Rust integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * The implementation must use safe Rust only.
 *
 * No unsafe block, unsafe trait, unsafe function, raw-pointer requirement,
 * foreign-memory assumption, or unsafe generated integration is part of this
 * grammar contract.
 *
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [ ] It is the canonical generic policy grammar.
 *
 *     [ ] It uses tokenVocab = ZamaniLexer.
 *
 *     [ ] It reuses Names.
 *
 *     [ ] It reuses Attributes.
 *
 *     [ ] It reuses Versioning.
 *
 *     [ ] It reuses Requirements.
 *
 *     [ ] It reuses Constraints.
 *
 *     [ ] It reuses Capabilities.
 *
 *     [ ] It reuses the canonical expression grammar.
 *
 *     [ ] It defines no lexer rules.
 *
 *     [ ] It defines no semantic predicates.
 *
 *     [ ] It contains no embedded Rust.
 *
 *     [ ] It contains no unsafe code.
 *
 *     [ ] It contains no physical hardware selection.
 *
 *     [ ] It contains no quantum topology.
 *
 *     [ ] It contains no gate catalogue.
 *
 *     [ ] It contains no resource allocation.
 *
 *     [ ] It contains no scheduler.
 *
 *     [ ] It contains no router.
 *
 *     [ ] It contains no QEC implementation.
 *
 *     [ ] It contains no ZQN implementation.
 *
 *     [ ] It contains no HAL implementation.
 *
 *     [ ] It contains no backend implementation.
 *
 *     [ ] It contains no artificial capacity limit.
 *
 *     [ ] Requirements remain requirements.
 *
 *     [ ] Constraints remain constraints.
 *
 *     [ ] Capabilities remain capabilities.
 *
 *     [ ] Preferences remain preferences.
 *
 *     [ ] Permissions remain semantically distinct from capabilities.
 *
 *     [ ] Security-specific policy remains owned by grammar/security/.
 *
 *     [ ] Resource-specific syntax remains owned by grammar/resources/.
 *
 *     [ ] Contract syntax remains owned by grammar/validation/.
 *
 *     [ ] Policy source ordering remains preservable.
 *
 *     [ ] Policy composition remains preservable.
 *
 *     [ ] Policy expressions remain open-world.
 *
 *     [ ] Quantum integration terminates at semantic policy handling and
 *         downstream quantum::ir consumers.
 *
 *     [ ] HDL/hardware integration remains target-independent.
 *
 *     [ ] Positive tests pass.
 *
 *     [ ] Negative tests pass.
 *
 *     [ ] Boundary tests pass.
 *
 *     [ ] Scalability tests pass.
 *
 *     [ ] Cross-domain tests pass.
 *
 *     [ ] Determinism tests pass.
 *
 *     [ ] Compatibility tests pass.
 *
 *
 * ============================================================================
 * FINAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * The universal policy grammar follows:
 *
 *     POLICY SYNTAX
 *          |
 *          v
 *     DOMAIN-NEUTRAL AST
 *          |
 *          v
 *     SEMANTIC POLICY MODEL
 *          |
 *     +----+----+----+----+----+
 *     |    |    |    |    |    |
 *     v    v    v    v    v    v
 *   req  cap  res effect contract security
 *     |    |    |    |    |
 *     +----+----+----+----+----+
 *               |
 *               v
 *     CANONICAL SEMANTIC MODEL
 *               |
 *       +-------+-------+
 *       |       |       |
 *       v       v       v
 *   classical quantum  HDL
 *      IR     ::ir    /hardware
 *       |       |       |
 *       +-------+-------+
 *               |
 *               v
 *        target realization
 *
 * The program describes semantic intent.
 *
 * Policy describes governing intent.
 *
 * Capabilities describe available abilities.
 *
 * Requirements describe mandatory needs.
 *
 * Constraints describe conditions.
 *
 * Resources describe realizable computational facilities.
 *
 * Effects describe observable computational behavior.
 *
 * The implementation determines how these are realized.
 *
 * This separation is mandatory for a scalable:
 *
 *     Program_Once
 *          ->
 *     Compile_Once
 *          ->
 *     Run_Everywhere
 *          ->
 *     Run_Anywhere
 *          ->
 *     Forever
 *
 * architecture.
 * ============================================================================
 */
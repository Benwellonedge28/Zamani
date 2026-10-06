/*
 * ============================================================================
 * ZAMANI UNIVERSAL PROGRAMMING LANGUAGE
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/policies/constitution.g4
 *
 * GRAMMAR
 * -------
 * ANTLR4 parser grammar
 *
 * STATUS
 * ------
 * PRODUCTION-READY FOUNDATIONAL GOVERNANCE GRAMMAR
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
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
 * This grammar defines the syntax boundary for a Zamani CONSTITUTION.
 *
 * A constitution is a foundational governance artifact that establishes
 * language-level or program-level governing principles for policies and
 * semantic decisions.
 *
 * A constitution may establish:
 *
 *     principles
 *     invariants
 *     requirements
 *     guarantees
 *     constraints
 *     permissions
 *     prohibitions
 *     preferences
 *     authorities
 *     scopes
 *     precedence
 *     policy relationships
 *     amendment rules
 *     compatibility requirements
 *     provenance requirements
 *     reproducibility requirements
 *     determinism requirements
 *     safety requirements
 *     resource governance
 *     capability governance
 *     effect governance
 *     execution governance
 *     simulation governance
 *     security governance
 *     quantum governance
 *     HDL/hardware governance
 *     distributed governance
 *     interoperability governance
 *
 * This file does NOT define the meaning of those concepts.
 *
 * Their semantic owners remain in the corresponding Zamani subsystems.
 *
 *
 * ============================================================================
 * ARCHITECTURAL PURPOSE
 * ============================================================================
 *
 * The constitution is ABOVE ordinary policy composition in governance
 * precedence, but it is NOT a replacement for the policy system.
 *
 * Conceptually:
 *
 *     constitution
 *          |
 *          v
 *     policy governance
 *          |
 *          v
 *     policy
 *          |
 *          v
 *     semantic intent
 *          |
 *          +-------------------+
 *          |                   |
 *          v                   v
 *     requirements         capabilities
 *          |                   |
 *          +---------+---------+
 *                    |
 *                    v
 *              resource/effect
 *              contract/security
 *                    |
 *                    v
 *              semantic model
 *                    |
 *          +---------+---------+
 *          |                   |
 *          v                   v
 *     classical             quantum::ir
 *                              |
 *                              v
 *                       target-independent
 *                          optimization
 *                              |
 *                    +---------+---------+
 *                    |                   |
 *                    v                   v
 *                  HDL              execution
 *                    |                   |
 *                    +---------+---------+
 *                              |
 *                              v
 *                    routing/scheduling
 *                              |
 *                           resilience
 *                              |
 *                          ZQN / HAL
 *                              |
 *                              v
 *                         realization
 *
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 *     constitutionDeclaration
 *     constitutionBody
 *     constitutionMember
 *     constitutionPrinciple
 *     constitutionInvariant
 *     constitutionRequirement
 *     constitutionConstraint
 *     constitutionGuarantee
 *     constitutionPermission
 *     constitutionProhibition
 *     constitutionPreference
 *     constitutionAuthority
 *     constitutionScope
 *     constitutionPrecedence
 *     constitutionPolicyReference
 *     constitutionExtension
 *     constitutionOverride
 *     constitutionAmendment
 *     constitutionProperty
 *     constitutionPropertyKey
 *     constitutionExpression
 *
 *
 * THIS FILE DOES NOT OWN
 * ----------------------
 *
 *     ordinary policy declarations
 *     policy body syntax
 *     policy members
 *     policy requirements
 *     policy capabilities
 *     policy resources
 *     policy permissions
 *     policy prohibitions
 *     policy preferences
 *     policy fallbacks
 *     policy execution syntax
 *     simulation syntax
 *     security authorization
 *     resource allocation
 *     capability discovery
 *     effect definitions
 *     contract definitions
 *     quantum operations
 *     quantum::ir
 *     HDL syntax
 *     hardware realization
 *     scheduling
 *     routing
 *     QEC
 *     ZQN
 *     HAL
 *     runtime enforcement
 *
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * Ordinary policies remain owned by:
 *
 *     grammar/policies/policy.g4
 *
 * This grammar MUST NOT recreate:
 *
 *     policyDeclaration
 *     policyBody
 *     policyMember
 *     policySimulation
 *     policyRequirement
 *     policyCapability
 *     policyResource
 *
 * Instead, this grammar establishes governance over those semantic objects.
 *
 *
 * ============================================================================
 * CONSTITUTION VS POLICY
 * ============================================================================
 *
 * A CONSTITUTION establishes foundational governance.
 *
 * A POLICY expresses operational governing intent.
 *
 * Therefore:
 *
 *     constitution != policy
 *
 * A constitution may govern policies without becoming another policy grammar.
 *
 *
 * Example semantic relationship:
 *
 *     constitution system_governance
 *     {
 *         principle = portability;
 *         invariant = semantic_stability;
 *         prohibits = physical_binding;
 *         requires = reproducibility_requirement;
 *     }
 *
 * The exact meaning of each property is determined semantically.
 *
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DIRECT LEXICAL DEPENDENCY
 * -------------------------
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * through:
 *
 *     tokenVocab = ZamaniLexer;
 *
 *
 * DIRECT GRAMMAR DEPENDENCIES
 * ---------------------------
 *
 *     grammar/core/names.g4
 *     grammar/expressions/expressions.g4
 *
 *
 * IMPORT RULE
 * -----------
 *
 * This grammar MUST NOT import:
 *
 *     grammar/policies/policy.g4
 *
 * because policy.g4 is the ordinary policy authority and importing it here
 * would create unnecessary coupling and potential composition cycles.
 *
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Parser contexts are converted into domain-neutral AST structures
 * conceptually equivalent to:
 *
 *     Constitution
 *     ConstitutionPrinciple
 *     ConstitutionInvariant
 *     ConstitutionRequirement
 *     ConstitutionConstraint
 *     ConstitutionGuarantee
 *     ConstitutionPermission
 *     ConstitutionProhibition
 *     ConstitutionPreference
 *     ConstitutionAuthority
 *     ConstitutionScope
 *     ConstitutionPrecedence
 *     ConstitutionPolicyReference
 *     ConstitutionExtension
 *     ConstitutionOverride
 *     ConstitutionAmendment
 *     ConstitutionProperty
 *
 * The AST MUST preserve:
 *
 *     constitution identity
 *     member ordering
 *     property ordering
 *     expression structure
 *     policy references
 *     scope expressions
 *     precedence expressions
 *     source spans
 *     source ordering
 *     amendment relationships
 *     provenance-relevant syntax
 *
 * The AST MUST NOT contain:
 *
 *     CPU handles
 *     GPU handles
 *     FPGA handles
 *     ASIC handles
 *     QPU handles
 *     physical qubit mappings
 *     device handles
 *     memory addresses
 *     calibration state
 *     routing state
 *     scheduling state
 *     vendor SDK objects
 *     backend objects
 *
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis determines:
 *
 *     constitution identity
 *     constitution validity
 *     scope
 *     authority
 *     precedence
 *     applicability
 *     policy compatibility
 *     policy conflicts
 *     invariant preservation
 *     requirement satisfaction
 *     constraint compatibility
 *     amendment authorization
 *     provenance
 *     version compatibility
 *     determinism
 *     reproducibility
 *
 * The parser performs none of these operations.
 *
 *
 * ============================================================================
 * GOVERNANCE CONTRACT
 * ============================================================================
 *
 * Constitution semantics should establish a hierarchy such as:
 *
 *     constitutional invariant
 *          >
 *     constitutional prohibition
 *          >
 *     constitutional requirement
 *          >
 *     constitutional guarantee
 *          >
 *     ordinary policy
 *          >
 *     preference
 *
 * The exact precedence model belongs to semantic analysis.
 *
 * The grammar MUST NOT hard-code precedence through parser ordering.
 *
 *
 * ============================================================================
 * PRINCIPLE CONTRACT
 * ============================================================================
 *
 * A principle is a named or expression-based foundational intent.
 *
 * Principles are deliberately open-ended.
 *
 * This prevents the language from requiring a new keyword for every future
 * governance concept.
 *
 * ============================================================================
 */

parser grammar Constitution;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names,
    Expressions
;


/*
 * ============================================================================
 * 1. CONSTITUTION DECLARATION
 * ============================================================================
 *
 * Canonical form:
 *
 *     constitution <name> {
 *         ...
 *     }
 *
 * The spelling `constitution` requires a canonical lexical token.
 *
 * The lexer integration is described below.
 */
constitutionDeclaration
    : CONSTITUTION
      qualifiedName
      constitutionBody
    ;


/*
 * ============================================================================
 * 2. CONSTITUTION BODY
 * ============================================================================
 *
 * Zero or more members are permitted.
 *
 * There is intentionally no fixed member count.
 */
constitutionBody
    : LBRACE
      constitutionMember*
      RBRACE
    ;


/*
 * ============================================================================
 * 3. CONSTITUTION MEMBER
 * ============================================================================
 *
 * Every member represents foundational governance intent.
 *
 * Ordinary operational semantics remain owned by downstream systems.
 */
constitutionMember
    : constitutionPrinciple
    | constitutionInvariant
    | constitutionRequirement
    | constitutionConstraint
    | constitutionGuarantee
    | constitutionPermission
    | constitutionProhibition
    | constitutionPreference
    | constitutionAuthority
    | constitutionScope
    | constitutionPrecedence
    | constitutionPolicyReference
    | constitutionExtension
    | constitutionOverride
    | constitutionAmendment
    | constitutionProperty
    ;


/*
 * ============================================================================
 * 4. PRINCIPLE
 * ============================================================================
 *
 * Example:
 *
 *     principle = portability;
 *
 * The property name remains open-world.
 */
constitutionPrinciple
    : PROPERTY
      constitutionExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 5. INVARIANT
 * ============================================================================
 *
 * An invariant establishes a condition that constitutional governance must
 * preserve.
 */
constitutionInvariant
    : INVARIANT
      constitutionExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 6. REQUIREMENT
 * ============================================================================
 *
 * Requirements express mandatory constitutional conditions.
 *
 * They do not allocate resources.
 */
constitutionRequirement
    : REQUIRES
      constitutionExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 7. CONSTRAINT
 * ============================================================================
 *
 * Constraints restrict permitted semantic choices.
 */
constitutionConstraint
    : CONSTRAINT
      constitutionExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 8. GUARANTEE
 * ============================================================================
 *
 * Guarantees describe conditions constitutional governance promises to
 * preserve, subject to semantic feasibility.
 */
constitutionGuarantee
    : GUARANTEE
      constitutionExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 9. PERMISSION
 * ============================================================================
 *
 * A constitutional permission establishes that a semantic action or
 * relationship is permitted by the constitution.
 *
 * It does not itself constitute security authorization.
 */
constitutionPermission
    : ALLOW
      constitutionExpression
      SEMICOLON
    | PERMIT
      constitutionExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 10. PROHIBITION
 * ============================================================================
 *
 * A constitutional prohibition establishes a forbidden semantic condition.
 */
constitutionProhibition
    : FORBID
      constitutionExpression
      SEMICOLON
    | DENY
      constitutionExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 11. PREFERENCE
 * ============================================================================
 *
 * Preferences remain advisory.
 *
 * They MUST NOT automatically override constitutional invariants,
 * prohibitions, requirements, or constraints.
 */
constitutionPreference
    : PREFER
      constitutionExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 12. AUTHORITY
 * ============================================================================
 *
 * Authority identifies the semantic scope or governing principal to which
 * constitutional rules apply.
 *
 * The identity remains an expression so that future authority systems do not
 * require new grammar keywords.
 */
constitutionAuthority
    : identifier
      ASSIGN
      constitutionExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 13. SCOPE
 * ============================================================================
 *
 * Scope is intentionally expression-based.
 *
 * It may semantically describe:
 *
 *     module
 *     package
 *     program
 *     library
 *     policy
 *     execution context
 *     deployment context
 *     dialect
 *     domain
 *     resource class
 *     capability class
 *     future semantic scope
 */
constitutionScope
    : SCOPE
      constitutionExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 14. PRECEDENCE
 * ============================================================================
 *
 * Precedence expresses governance ordering.
 *
 * The grammar does not impose a fixed numeric hierarchy.
 */
constitutionPrecedence
    : identifier
      ASSIGN
      constitutionExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 15. POLICY REFERENCE
 * ============================================================================
 *
 * A constitution can explicitly govern an existing policy by qualified name.
 *
 * Example:
 *
 *     policy execution::portable;
 */
constitutionPolicyReference
    : POLICY
      qualifiedName
      SEMICOLON
    ;


/*
 * ============================================================================
 * 16. CONSTITUTION EXTENSION
 * ============================================================================
 *
 * A constitution may extend another constitution.
 *
 * This creates a semantic governance relationship.
 */
constitutionExtension
    : EXTENDS
      qualifiedName
      SEMICOLON
    ;


/*
 * ============================================================================
 * 17. CONSTITUTION OVERRIDE
 * ============================================================================
 *
 * Override is explicit.
 *
 * It MUST NOT silently weaken a constitutional invariant or prohibition.
 *
 * Semantic validation determines whether an override is legally valid.
 */
constitutionOverride
    : OVERRIDE
      qualifiedName
      SEMICOLON
    ;


/*
 * ============================================================================
 * 18. AMENDMENT
 * ============================================================================
 *
 * Amendment is represented as an explicit governance operation.
 *
 * The amendment payload is an expression so that future amendment models can
 * evolve without requiring a new grammar for every mechanism.
 *
 * Example:
 *
 *     amend = approved_change;
 *
 * `amend` is intentionally represented through an open property form rather
 * than requiring another reserved keyword.
 */
constitutionAmendment
    : identifier
      ASSIGN
      constitutionExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 19. OPEN-WORLD CONSTITUTIONAL PROPERTY
 * ============================================================================
 *
 * This is the principal extensibility mechanism.
 *
 * Future governance concepts can be expressed through:
 *
 *     name = expression;
 *
 * without requiring:
 *
 *     a new keyword
 *     a new parser branch
 *     a new hardware enumeration
 *     a new domain grammar
 *
 * Semantic validation determines which properties are valid.
 */
constitutionProperty
    : constitutionPropertyKey
      ASSIGN
      constitutionExpression
      SEMICOLON
    ;


constitutionPropertyKey
    : identifier
    ;


/*
 * ============================================================================
 * 20. EXPRESSION BOUNDARY
 * ============================================================================
 *
 * Constitution expressions are ordinary Zamani expressions.
 *
 * There is no second expression language.
 */
constitutionExpression
    : expression
    ;


/*
 * ============================================================================
 * 21. OPTIONAL EXPRESSION
 * ============================================================================
 */

optionalConstitutionExpression
    : constitutionExpression?
    ;


/*
 * ============================================================================
 * 22. CONSTITUTION LIST
 * ============================================================================
 *
 * Reusable composition boundary.
 */
constitutionList
    : constitutionDeclaration*
    ;


constitutionDeclarations
    : constitutionDeclaration+
    ;


optionalConstitution
    : constitutionDeclaration?
    ;


/*
 * ============================================================================
 * 23. GOVERNANCE RELATIONSHIP
 * ============================================================================
 *
 * This rule allows downstream grammar composition to preserve the distinction
 * between:
 *
 *     constitution
 *     policy
 *
 * without defining either system twice.
 */
constitutionGovernanceReference
    : POLICY qualifiedName
    | EXTENDS qualifiedName
    | OVERRIDE qualifiedName
    ;


/*
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Constitution grammar never allocates resources.
 *
 * It may govern semantic resource requirements such as:
 *
 *     memory
 *     compute
 *     storage
 *     bandwidth
 *     latency
 *     energy
 *     accelerator capability
 *     quantum capability
 *     distributed capability
 *
 * Actual feasibility is determined downstream.
 *
 * There is no fixed resource quantity in this grammar.
 *
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Capability names remain open-world expressions.
 *
 * The grammar does not enumerate:
 *
 *     CPU types
 *     GPU types
 *     FPGA families
 *     ASIC families
 *     QPU families
 *     simulator implementations
 *     accelerator vendors
 *
 * New capabilities therefore do not require grammar changes.
 *
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Constitutional governance may semantically govern effects including:
 *
 *     io
 *     network
 *     mutation
 *     randomness
 *     native
 *     foreign
 *     distributed
 *     measurement
 *     learning
 *     adaptation
 *     reflection
 *     code generation
 *     simulation
 *
 * Effect definitions remain owned by:
 *
 *     grammar/effects/
 *
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Constitutional expressions may refer to:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * Contract syntax remains owned by:
 *
 *     grammar/validation/
 *
 * This grammar establishes governance relationships only.
 *
 *
 * ============================================================================
 * POLICY INTEGRATION
 * ============================================================================
 *
 * Ordinary policy syntax remains owned by:
 *
 *     grammar/policies/policy.g4
 *
 * Constitution semantic analysis consumes:
 *
 *     Policy
 *     PolicyRule
 *     PolicyRequirement
 *     PolicyConstraint
 *     PolicyCapability
 *     PolicyResource
 *     PolicyPermission
 *     PolicyProhibition
 *     PolicyPreference
 *     PolicyFallback
 *     PolicySelection
 *     PolicyNegotiation
 *     PolicySimulation
 *     PolicyAdaptation
 *     PolicySandbox
 *     PolicyDeterminism
 *     PolicyReproducibility
 *     PolicyEffect
 *     PolicyProvenance
 *
 * This file does not redefine any of them.
 *
 *
 * ============================================================================
 * EXECUTION INTEGRATION
 * ============================================================================
 *
 * Constitutional governance may govern:
 *
 *     execution
 *     deployment
 *     simulation
 *     adaptation
 *     fallback
 *     recovery
 *     reproducibility
 *     deterministic execution
 *
 * Execution syntax remains owned by:
 *
 *     grammar/execution/
 *
 *
 * ============================================================================
 * SIMULATION INTEGRATION
 * ============================================================================
 *
 * Constitution grammar does NOT define:
 *
 *     SIMULATE
 *     simulationExpression
 *     simulationOperation
 *     simulationTarget
 *
 * Those remain owned by the existing simulation subsystems.
 *
 * A constitution can govern simulation semantically through expressions,
 * requirements, constraints, prohibitions, guarantees, and properties.
 *
 * Therefore this file does not compete with:
 *
 *     grammar/policies/simulation.g4
 *     grammar/execution/simulation.g4
 *     grammar/expressions/simulation.g4
 *     grammar/statements/simulate.g4
 *
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Constitutional governance can constrain or guarantee quantum semantics.
 *
 * Example semantic concerns include:
 *
 *     measurement policy
 *     reproducibility
 *     resilience
 *     error handling
 *     resource requirements
 *     capability requirements
 *     provenance
 *     allowed effects
 *     adaptation
 *
 * The quantum lowering boundary remains:
 *
 *     constitution
 *         |
 *         v
 *     policy semantics
 *         |
 *         v
 *     semantic quantum model
 *         |
 *         v
 *     quantum::ir
 *         |
 *         v
 *     optimization
 *         |
 *         v
 *     decomposition
 *         |
 *         v
 *     routing
 *         |
 *         v
 *     scheduling
 *         |
 *         v
 *     resilience / QEC
 *         |
 *         v
 *     ZQN
 *         |
 *         v
 *     HAL
 *
 * This grammar never represents physical qubit mappings.
 *
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Constitutional governance can apply to HDL and hardware intent.
 *
 * It does not encode:
 *
 *     register width
 *     wire count
 *     memory capacity
 *     FPGA dimensions
 *     ASIC cell counts
 *     processor counts
 *     device counts
 *
 * Hardware realization remains downstream.
 *
 *
 * ============================================================================
 * AI / KNOWLEDGE / REASONING BOUNDARY
 * ============================================================================
 *
 * Constitution may govern generic semantic capabilities such as:
 *
 *     reasoning
 *     learning
 *     adaptation
 *     evidence
 *     provenance
 *     uncertainty
 *     explanation
 *     agents
 *
 * No application-specific AI vocabulary is introduced here.
 *
 *
 * ============================================================================
 * DISTRIBUTED BOUNDARY
 * ============================================================================
 *
 * Constitutional rules may govern:
 *
 *     consistency
 *     communication
 *     fault handling
 *     recovery
 *     resilience
 *     topology requirements
 *     distributed execution
 *
 * No fixed number of nodes, processors, devices, or endpoints is encoded.
 *
 *
 * ============================================================================
 * INTEROPERABILITY BOUNDARY
 * ============================================================================
 *
 * Constitution may govern:
 *
 *     FFI
 *     ABI
 *     foreign effects
 *     external calls
 *     data interchange
 *
 * Actual FFI/ABI syntax remains owned by:
 *
 *     grammar/interoperability/
 *
 *
 * ============================================================================
 * METAPROGRAMMING BOUNDARY
 * ============================================================================
 *
 * Constitutional governance may restrict or permit:
 *
 *     reflection
 *     code generation
 *     compile-time execution
 *     introspection
 *     dynamic adaptation
 *
 * Metaprogramming syntax remains owned by:
 *
 *     grammar/metaprogramming/
 *
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Semantic provenance should preserve:
 *
 *     constitution identity
 *     constitution version
 *     policy references
 *     amendment relationships
 *     governing principle
 *     source location
 *     derived decisions
 *     validation results
 *     semantic transformations
 *
 * This is consumed by:
 *
 *     grammar/policies/provenance.g4
 *     grammar/spec/provenance.md
 *     compiler provenance infrastructure
 *
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no semantic actions
 *     no runtime predicates
 *     no filesystem access
 *     no network access
 *     no hardware access
 *     no randomness
 *     no environment inspection
 *
 * Identical token streams and parser configuration must produce structurally
 * equivalent parse trees.
 *
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar imposes no language-level finite limit on:
 *
 *     constitutions
 *     members
 *     principles
 *     invariants
 *     requirements
 *     constraints
 *     guarantees
 *     permissions
 *     prohibitions
 *     preferences
 *     policies
 *     policy references
 *     amendments
 *     properties
 *     expressions
 *     qualified-name depth
 *     governance scope
 *     resource classes
 *     capability classes
 *     domains
 *     targets
 *     devices
 *     nodes
 *     processors
 *     accelerators
 *     qubits
 *     memory
 *     tensor dimensions
 *     tensor rank
 *     network size
 *
 * Any practical limitation is an implementation/resource limitation, not a
 * language semantic limit.
 *
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden:
 *
 *     MAX_CONSTITUTIONS
 *     MAX_PRINCIPLES
 *     MAX_POLICIES
 *     MAX_RULES
 *     MAX_RESOURCES
 *     MAX_CAPABILITIES
 *     MAX_NODES
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * None are represented by this grammar.
 *
 *
 * ============================================================================
 * SAFETY CONTRACT
 * ============================================================================
 *
 * This file contains no embedded Rust code and no unsafe actions.
 *
 * The compiler implementation target is:
 *
 *     Rust 1.97+
 *     Rust 2021
 *     safe Rust
 *
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics include:
 *
 *     missing constitution name
 *     missing constitution body
 *     malformed member
 *     malformed expression
 *     malformed policy reference
 *     malformed extension
 *     malformed override
 *     missing assignment
 *     missing semicolon
 *     unmatched braces
 *
 * Semantic diagnostics include:
 *
 *     invalid constitutional scope
 *     invalid authority
 *     conflicting invariant
 *     invalid amendment
 *     unauthorized amendment
 *     invalid policy relationship
 *     policy conflict
 *     unsatisfied constitutional requirement
 *     violated constitutional prohibition
 *     incompatible constitutional extension
 *     invalid precedence
 *     incompatible version
 *
 * Semantic failures MUST NOT be converted into parser failures.
 *
 *
 * ============================================================================
 * POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * Minimal:
 *
 *     constitution core {
 *     }
 *
 * Principle:
 *
 *     constitution core {
 *         property = portability;
 *     }
 *
 * Invariant:
 *
 *     constitution core {
 *         invariant semantic_stability;
 *     }
 *
 * Requirement:
 *
 *     constitution core {
 *         requires capability("portable.execution");
 *     }
 *
 * Constraint:
 *
 *     constitution core {
 *         constraint target != physical_binding;
 *     }
 *
 * Guarantee:
 *
 *     constitution core {
 *         guarantee reproducible_execution;
 *     }
 *
 * Permission:
 *
 *     constitution core {
 *         allow effect("simulation");
 *     }
 *
 * Prohibition:
 *
 *     constitution core {
 *         forbid capability("physical.binding");
 *     }
 *
 * Policy reference:
 *
 *     constitution core {
 *         policy execution::portable;
 *     }
 *
 * Extension:
 *
 *     constitution domain {
 *         extends core;
 *     }
 *
 * Override:
 *
 *     constitution domain {
 *         override core;
 *     }
 *
 * Open-world property:
 *
 *     constitution core {
 *         version = language_version;
 *         provenance = required_provenance;
 *         compatibility = compatibility_policy;
 *     }
 *
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * Reject:
 *
 *     constitution
 *
 *     constitution core
 *
 *     constitution core {
 *
 *     }
 *
 *     constitution core {
 *         requires;
 *     }
 *
 *     constitution core {
 *         invariant;
 *     }
 *
 *     constitution core {
 *         property =;
 *     }
 *
 *     constitution core {
 *         policy;
 *     }
 *
 *     constitution core {
 *         extends;
 *     }
 *
 *     constitution core {
 *         override;
 *     }
 *
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Constitution grammar must be tested against:
 *
 *     ordinary policy declarations
 *     execution policies
 *     simulation policies
 *     security policies
 *     resource policies
 *     quantum policies
 *     HDL policies
 *     distributed policies
 *     interoperability policies
 *     adaptation policies
 *     provenance policies
 *
 * There must be no competing parser ownership for these constructs.
 *
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Constitutional syntax is independent of physical realization.
 *
 * The same constitution can govern a program considered for:
 *
 *     atom-scale computation
 *     embedded systems
 *     CPU
 *     multicore
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     simulator
 *     QPU
 *     HPC
 *     cluster
 *     distributed systems
 *     heterogeneous systems
 *     cloud systems
 *     future computational substrates
 *
 * The constitution itself does not change with target scale.
 *
 * Target feasibility is determined by:
 *
 *     requirements
 *     capabilities
 *     resources
 *     constraints
 *     policies
 *     semantic compatibility
 *
 * downstream.
 *
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/core/names.g4
 *     grammar/expressions/expressions.g4
 *
 *
 * EXPORTS:
 *
 *     Constitution
 *     constitutionDeclaration
 *     constitutionBody
 *     constitutionMember
 *     constitutionExpression
 *     constitutionGovernanceReference
 *
 *
 * CONSUMED_BY:
 *
 *     grammar/policies/policy.g4
 *     grammar/policies/scopes.g4
 *     grammar/policies/provenance.g4
 *     grammar/validation/
 *     semantic policy/constitution analysis
 *     compiler governance analysis
 *
 *
 * AST_OWNER:
 *
 *     domain-neutral Zamani AST layer
 *
 *
 * SEMANTIC_OWNER:
 *
 *     constitutional/policy semantic analysis
 *
 *
 * IR_OWNER:
 *
 *     canonical semantic model
 *
 *     No constitution-specific physical IR is permitted.
 *
 *
 * TEST_OWNER:
 *
 *     grammar/tests/policies/
 *     grammar/tests/validation/
 *     grammar/tests/compatibility/
 *
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/policies.md
 *     grammar/specification/
 *
 *
 * ============================================================================
 * INTEGRATION STEPS
 * ============================================================================
 *
 * 1. Add the canonical lexical token:
 *
 *        CONSTITUTION : 'constitution' ;
 *
 *    to the canonical keyword/token authority.
 *
 *
 * 2. The lexical layer must expose CONSTITUTION through ZamaniLexer.
 *
 *
 * 3. Import this grammar from the appropriate policy composition root:
 *
 *        import
 *            Names,
 *            Requirements,
 *            Constraints,
 *            Capabilities,
 *            Expressions,
 *            Constitution
 *        ;
 *
 *
 * 4. Add the constitution declaration to the relevant source-unit/declaration
 *    composition root rather than placing it inside policy.g4's ordinary
 *    policyMember list.
 *
 *
 * 5. If constitutional rules are later permitted inside a policy body, add
 *    only a semantic reference or adapter. Do NOT duplicate the constitution
 *    grammar inside policy.g4.
 *
 *
 * 6. The AST builder must map Constitution parser contexts to the existing
 *    domain-neutral governance representation.
 *
 *
 * 7. Semantic analysis must establish constitutional precedence before normal
 *    policy conflict resolution.
 *
 *
 * 8. Provenance must retain constitution identity and amendment lineage.
 *
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This grammar is additive.
 *
 * It does not redefine:
 *
 *     policy
 *     simulation
 *     execution
 *     security
 *     resources
 *     capabilities
 *     quantum
 *     HDL
 *     hardware
 *
 * Existing source programs remain unaffected until the new `constitution`
 * declaration is used.
 *
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE when:
 *
 * [x] Constitution has one clear grammar authority.
 *
 * [x] Ordinary policy syntax remains owned by policy.g4.
 *
 * [x] Simulation syntax remains owned by simulation subsystems.
 *
 * [x] No physical hardware is enumerated.
 *
 * [x] No fixed machine capacity is encoded.
 *
 * [x] No application-specific keyword catalogue is introduced.
 *
 * [x] Future governance concepts can use open-world expressions/properties.
 *
 * [x] Constitution identity is qualified-name based.
 *
 * [x] Policy relationships are explicit.
 *
 * [x] Amendments are represented without physical assumptions.
 *
 * [x] Provenance can preserve governance lineage.
 *
 * [x] Deterministic parsing is preserved.
 *
 * [x] Rust implementation remains safe-only.
 *
 * [x] Quantum governance terminates at semantic quantum intent and
 *     quantum::ir downstream.
 *
 * [x] HDL governance terminates at semantic hardware intent downstream.
 *
 * [x] Resource resolution remains outside the grammar.
 *
 * [x] Capability resolution remains outside the grammar.
 *
 * [x] Runtime enforcement remains outside the grammar.
 *
 * [x] Target selection remains outside the grammar.
 *
 * [x] The grammar does not impose a language-level scale ceiling.
 *
 *
 * ============================================================================
 * FINAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * Constitution syntax expresses FOUNDATIONAL GOVERNANCE.
 *
 * It does not express:
 *
 *     machine topology
 *     physical capacity
 *     implementation algorithms
 *     simulator implementation
 *     quantum hardware mapping
 *     HDL synthesis
 *     runtime scheduling
 *     resource allocation
 *
 * The resulting architecture is:
 *
 *     constitution
 *          |
 *          v
 *     policy governance
 *          |
 *          v
 *     semantic intent
 *          |
 *          v
 *     requirements / capabilities / resources
 *          |
 *          v
 *     contracts / effects / security / provenance
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          +--------------------+
 *          |                    |
 *          v                    v
 *     classical IR          quantum::ir
 *          |                    |
 *          +---------+----------+
 *                    |
 *                    v
 *              target-independent
 *                 optimization
 *                    |
 *                    v
 *               lowering/planning
 *                    |
 *              routing/scheduling
 *                    |
 *                 resilience
 *                    |
 *                 ZQN/HAL
 *                    |
 *                    v
 *              target realization
 *
 * This preserves the POCO-REAF requirement because constitutional governance
 * describes invariant semantic intent rather than a particular machine.
 *
 * ============================================================================
 */
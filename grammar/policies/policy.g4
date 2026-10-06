/*
 * ============================================================================
 * ZAMANI UNIVERSAL PROGRAMMING LANGUAGE
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/policies/Policy.g4
 *
 * GRAMMAR
 * -------
 * ANTLR4 parser grammar
 *
 * STATUS
 * ------
 * CANONICAL UNIVERSAL POLICY LEAF GRAMMAR
 *
 * IMPLEMENTATION BASELINE
 * -----------------------
 * Rust 2021
 * Rust 1.97+
 * Safe Rust only
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This file owns the source-level syntax for declarative Zamani policies.
 *
 * A policy expresses GOVERNING INTENT.
 *
 * A policy may govern:
 *
 *     requirements
 *     constraints
 *     capabilities
 *     resources
 *     permissions
 *     prohibitions
 *     preferences
 *     fallbacks
 *     selection
 *     negotiation
 *     execution
 *     adaptation
 *     simulation
 *     sandboxing
 *     determinism
 *     reproducibility
 *     effects
 *     provenance
 *     contracts
 *     security-related behavior
 *     quantum behavior
 *     classical behavior
 *     HDL/hardware intent
 *     distributed behavior
 *     AI/model behavior
 *     interoperability
 *
 * This grammar intentionally remains DOMAIN-NEUTRAL.
 *
 * It does not create separate policy languages for:
 *
 *     classical computing
 *     quantum computing
 *     HDL
 *     hardware
 *     AI
 *     distributed systems
 *     networking
 *     security
 *     simulation
 *     deployment
 *
 * Those systems consume the common policy representation.
 *
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * The complete pipeline is:
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     Policy parser rules
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
 *       +--> resources
 *       +--> effects
 *       +--> contracts
 *       +--> security
 *       +--> execution
 *       +--> adaptation
 *       +--> provenance
 *       |
 *       v
 *     canonical semantic model
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> HDL/hardware representation
 *       +--> distributed representation
 *       +--> other domain IRs
 *       |
 *       v
 *     target-independent optimization
 *       |
 *       v
 *     lowering / planning
 *       |
 *       v
 *     routing / scheduling / resilience
 *       |
 *       v
 *     ZQN / HAL where applicable
 *       |
 *       v
 *     target realization
 *
 * This file participates only in source parsing.
 *
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns:
 *
 *     policyDeclaration
 *     policyBody
 *     policyMember
 *
 *     policyRule
 *     policyRuleCondition
 *     policyRuleAction
 *     policyRuleClause
 *
 *     policyRequirement
 *     policyConstraint
 *     policyCapability
 *     policyResource
 *
 *     policyPermission
 *     policyProhibition
 *     policyPreference
 *     policyFallback
 *
 *     policySelection
 *     policyNegotiation
 *     policyRetry
 *     policyRecovery
 *     policyEscalation
 *     policyRejection
 *
 *     policySimulation
 *     policyAdaptation
 *     policySandbox
 *
 *     policyDeterminism
 *     policyReproducibility
 *
 *     policyEffect
 *     policyProvenance
 *     policyContractReference
 *
 *     policyComposition
 *     policyExtension
 *     policyOverride
 *
 *     policyProperty
 *     policyPropertyKey
 *
 *     policyExpression
 *     policyExpressionList
 *     policyArgument
 *     policyArgumentList
 *
 * This file does NOT own general expression syntax.
 *
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     lexer rules
 *     tokens
 *     identifiers
 *     qualified names
 *     general expressions
 *     arithmetic
 *     logical precedence
 *     types
 *     resources
 *     capabilities
 *     requirements
 *     constraints
 *     contracts
 *     effects
 *     security authorization
 *     identities
 *     credentials
 *     roles
 *     trust
 *     hardware discovery
 *     physical devices
 *     placement
 *     routing
 *     scheduling
 *     calibration
 *     QEC
 *     ZQN
 *     HAL
 *     runtime enforcement
 *     classical IR
 *     quantum::ir
 *     HDL IR
 *     backend implementation
 *
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * This file is the policy LEAF authority.
 *
 * It must not be copied into:
 *
 *     grammar/security/
 *     grammar/resources/
 *     grammar/execution/
 *     grammar/quantum/
 *     grammar/ai/
 *     grammar/hardware/
 *
 * Those directories may define policy adapters, bindings, or consumers, but
 * they must consume these policy rules rather than recreate policy syntax.
 *
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DIRECT INPUT
 * ------------
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Canonical lexical vocabulary is supplied through:
 *
 *     tokenVocab = ZamaniLexer;
 *
 *
 * PARSER DEPENDENCIES
 * -------------------
 *
 *     Names
 *     Requirements
 *     Constraints
 *     Capabilities
 *     Expressions
 *
 * These provide:
 *
 *     qualifiedName
 *     identifier
 *     requirementExpression
 *     constraintExpression
 *     capabilityExpression
 *     expression
 *
 *
 * ============================================================================
 * IMPORT CONTRACT
 * ============================================================================
 *
 * This grammar uses ANTLR grammar-name imports.
 *
 * It does NOT use filesystem-style imports.
 *
 * The build system must expose the canonical grammar directories through the
 * ANTLR grammar library path.
 *
 * ============================================================================
 */

parser grammar Policy;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names,
    Requirements,
    Constraints,
    Capabilities,
    Expressions
;


/*
 * ============================================================================
 * 1. POLICY DECLARATION
 * ============================================================================
 *
 * Canonical form:
 *
 *     policy name {
 *         ...
 *     }
 *
 * Policy identity is a qualified name.
 *
 * No target, vendor, hardware, processor, quantum architecture, or AI model
 * is encoded into the grammar.
 */

policyDeclaration
    : POLICY qualifiedName
      policyBody
    ;


/*
 * ============================================================================
 * 2. POLICY BODY
 * ============================================================================
 *
 * The body is an unbounded ordered sequence of policy members.
 *
 * There is deliberately no fixed number of:
 *
 *     rules
 *     requirements
 *     capabilities
 *     resources
 *     properties
 *     clauses
 *
 * This preserves language-level scalability.
 */

policyBody
    : LBRACE
      policyMember*
      RBRACE
    ;


policyMember
    : policyRule
    | policyRequirement
    | policyConstraint
    | policyCapability
    | policyResource
    | policyPermission
    | policyProhibition
    | policyPreference
    | policyFallback
    | policySelection
    | policyNegotiation
    | policyRetry
    | policyRecovery
    | policyEscalation
    | policyRejection
    | policySimulation
    | policyAdaptation
    | policySandbox
    | policyDeterminism
    | policyReproducibility
    | policyEffect
    | policyProvenance
    | policyContractReference
    | policyComposition
    | policyProperty
    ;


/*
 * ============================================================================
 * 3. CONDITIONAL POLICY RULE
 * ============================================================================
 *
 * Canonical form:
 *
 *     when <condition> => <action>;
 *
 * Example:
 *
 *     when execution::deterministic => prefer target;
 *
 * The condition remains a normal Zamani expression.
 *
 * The grammar does not evaluate it.
 */

policyRule
    : WHEN
      policyRuleCondition
      FAT_ARROW
      policyRuleAction
      policyRuleClause*
      SEMICOLON
    ;


policyRuleCondition
    : expression
    ;


policyRuleAction
    : policyPermission
    | policyProhibition
    | policyPreference
    | policyFallback
    | policySelection
    | policyNegotiation
    | policyRetry
    | policyRecovery
    | policyEscalation
    | policyRejection
    | policySimulation
    | policyAdaptation
    | policySandbox
    | policyDeterminism
    | policyReproducibility
    | policyEffect
    | policyProvenance
    | policyContractReference
    | policyExpression
    ;


policyRuleClause
    : policyRuleConditionClause
    | policyRulePropertyClause
    ;


policyRuleConditionClause
    : IF expression
    ;


policyRulePropertyClause
    : policyPropertyKey
      ASSIGN
      expression
    ;


/*
 * ============================================================================
 * 4. REQUIREMENTS
 * ============================================================================
 *
 * Requirement semantics remain owned by:
 *
 *     grammar/core/requirements.g4
 *
 * This file only establishes the policy boundary around them.
 */

policyRequirement
    : REQUIRES
      requirementExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 5. CONSTRAINTS
 * ============================================================================
 *
 * Constraint semantics remain owned by:
 *
 *     grammar/core/constraints.g4
 */

policyConstraint
    : CONSTRAINT
      constraintExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 6. CAPABILITIES
 * ============================================================================
 *
 * Capability identity and capability-expression semantics remain owned by:
 *
 *     grammar/core/capabilities.g4
 */

policyCapability
    : CAPABILITY
      capabilityExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 7. RESOURCES
 * ============================================================================
 *
 * A resource reference remains an expression.
 *
 * This grammar does not enumerate resource categories.
 *
 * Therefore future resources can be represented without changing this file.
 */

policyResource
    : RESOURCE
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 8. PERMISSION
 * ============================================================================
 *
 * ALLOW and PERMIT are policy-level permissions.
 *
 * They do not themselves constitute security authorization.
 *
 * Security authorization remains owned by grammar/security/.
 */

policyPermission
    : ALLOW
      policyExpression
      SEMICOLON
    | PERMIT
      policyExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 9. PROHIBITION
 * ============================================================================
 */

policyProhibition
    : FORBID
      policyExpression
      SEMICOLON
    | DENY
      policyExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 10. PREFERENCE
 * ============================================================================
 *
 * A preference is advisory.
 *
 * It MUST NOT silently become a requirement.
 *
 * Semantic policy resolution determines how competing preferences are handled.
 */

policyPreference
    : PREFER
      policyExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 11. FALLBACK
 * ============================================================================
 *
 * A fallback identifies an alternative semantic path.
 *
 * It does not select hardware or perform runtime recovery itself.
 */

policyFallback
    : FALLBACK
      policyExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 12. SELECTION
 * ============================================================================
 *
 * Selection expresses intent to choose among semantically valid alternatives.
 *
 * It does NOT select a physical device at parse time.
 */

policySelection
    : SELECT
      policyExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 13. NEGOTIATION
 * ============================================================================
 *
 * Negotiation describes declarative policy intent.
 *
 * Actual capability/resource negotiation is downstream.
 */

policyNegotiation
    : NEGOTIATE
      policyExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 14. RETRY
 * ============================================================================
 */

policyRetry
    : RETRY
      policyExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 15. RECOVERY
 * ============================================================================
 */

policyRecovery
    : RECOVER
      policyExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 16. ESCALATION
 * ============================================================================
 */

policyEscalation
    : ESCALATE
      policyExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 17. REJECTION
 * ============================================================================
 *
 * REJECT is policy intent.
 *
 * It is not the same as a runtime execution result.
 */

policyRejection
    : REJECT
      policyExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 18. SIMULATION
 * ============================================================================
 *
 * Simulation is an execution-policy concern.
 *
 * It may govern:
 *
 *     classical simulation
 *     quantum simulation
 *     HDL simulation
 *     hardware simulation
 *     distributed simulation
 *     AI/model simulation
 *     fault simulation
 *     performance simulation
 *
 * The grammar remains domain-neutral.
 */

policySimulation
    : SIMULATE
      policyExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 19. ADAPTATION
 * ============================================================================
 *
 * Adaptation is explicitly declarative.
 *
 * It does not grant unrestricted self-modification.
 *
 * Authorization, effects, resource constraints, provenance, and validation
 * are determined downstream.
 */

policyAdaptation
    : ADAPT
      policyExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 20. SANDBOX
 * ============================================================================
 *
 * A sandbox policy can govern:
 *
 *     effects
 *     capabilities
 *     resources
 *     network
 *     native calls
 *     foreign calls
 *     reflection
 *     code generation
 *     adaptation
 *
 * Security semantics remain owned by grammar/security/.
 */

policySandbox
    : SANDBOX
      policyExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 21. DETERMINISM
 * ============================================================================
 *
 * This is policy intent.
 *
 * The parser does not prove determinism.
 */

policyDeterminism
    : DETERMINISTIC
      policyExpression?
      SEMICOLON
    ;


/*
 * ============================================================================
 * 22. REPRODUCIBILITY
 * ============================================================================
 */

policyReproducibility
    : REPRODUCIBLE
      policyExpression?
      SEMICOLON
    ;


/*
 * ============================================================================
 * 23. EFFECT POLICY
 * ============================================================================
 *
 * Effects themselves are owned by grammar/effects/.
 *
 * This rule only states that a policy may govern an effect expression.
 */

policyEffect
    : EFFECT
      policyExpression
      SEMICOLON
    | EFFECTS
      policyExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 24. PROVENANCE POLICY
 * ============================================================================
 *
 * Provenance syntax remains open-world.
 *
 * The semantic provenance subsystem determines the actual provenance model.
 */

policyProvenance
    : PROVENANCE
      policyExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 25. CONTRACT REFERENCE
 * ============================================================================
 *
 * Contracts themselves remain owned by grammar/validation/.
 *
 * This rule does NOT redefine:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * A policy may reference a contract semantically.
 *
 * Example:
 *
 *     contract execution::correctness;
 */

policyContractReference
    : CONTRACT
      policyExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 26. POLICY COMPOSITION
 * ============================================================================
 *
 * Policy inheritance/composition is intentionally limited to lexical forms
 * already present in the repository.
 *
 * EXTENDS is a policy relationship.
 *
 * OVERRIDE identifies an explicitly named policy relationship.
 *
 * INCLUDE/EXCLUDE/BIND/TO are intentionally NOT invented here because those
 * are not currently canonical lexer tokens.
 *
 * Future composition constructs can be represented through policy properties
 * or added to the canonical lexer only after language-level approval.
 */

policyComposition
    : policyExtension
    | policyOverride
    ;


policyExtension
    : EXTENDS
      qualifiedName
      SEMICOLON
    ;


policyOverride
    : OVERRIDE
      qualifiedName
      SEMICOLON
    ;


/*
 * ============================================================================
 * 27. OPEN-WORLD POLICY PROPERTIES
 * ============================================================================
 *
 * This is one of the most important production-readiness mechanisms.
 *
 * Instead of adding a new reserved keyword for every future policy concept,
 * Zamani permits an extensible property form:
 *
 *     scope = expression;
 *     priority = expression;
 *     description = expression;
 *     provenance = expression;
 *     evidence = expression;
 *     explanation = expression;
 *     fallback_strategy = expression;
 *     adaptation_policy = expression;
 *
 * New property names remain ordinary identifiers.
 *
 * This prevents policy syntax from becoming an ever-growing keyword catalogue.
 *
 * Semantic validation determines which properties are legal in which policy
 * contexts.
 */

policyProperty
    : policyPropertyKey
      ASSIGN
      expression
      SEMICOLON
    ;


policyPropertyKey
    : identifier
    ;


/*
 * ============================================================================
 * 28. POLICY EXPRESSIONS
 * ============================================================================
 *
 * This bridge guarantees that policy expressions use the same expression
 * semantics as the rest of Zamani.
 *
 * There is no second policy expression language.
 */

policyExpression
    : expression
    ;


policyExpressionList
    : policyExpression
      (COMMA policyExpression)*
    ;


policyArgument
    : policyExpression
    | policyNamedArgument
    ;


policyNamedArgument
    : identifier
      ASSIGN
      policyExpression
    ;


policyArgumentList
    : policyArgument
      (COMMA policyArgument)*
    ;


/*
 * ============================================================================
 * 29. POLICY LISTS
 * ============================================================================
 *
 * Reusable composition boundary.
 */

policyList
    : policyDeclaration*
    ;


policyDeclarations
    : policyDeclaration+
    ;


/*
 * ============================================================================
 * 30. OPTIONAL POLICY
 * ============================================================================
 */

optionalPolicy
    : policyDeclaration?
    ;


/*
 * ============================================================================
 * 31. SOURCE PRESERVATION CONTRACT
 * ============================================================================
 *
 * The frontend AST must preserve:
 *
 *     policy identity
 *     member ordering
 *     rule ordering
 *     clause ordering
 *     property ordering
 *     composition relationships
 *     expressions
 *     source spans
 *     source text where required by tooling
 *
 * Semantic normalization must not destroy provenance information needed by:
 *
 *     diagnostics
 *     reproducibility
 *     audit
 *     explanation
 *     compilation provenance
 *     policy conflict reporting
 *
 *
 * ============================================================================
 * 32. AST CONTRACT
 * ============================================================================
 *
 * Parser contexts are converted by the domain-neutral AST layer.
 *
 * The AST should represent policy structures conceptually as:
 *
 *     Policy
 *     PolicyRule
 *     PolicyAction
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
 *     PolicyRetry
 *     PolicyRecovery
 *     PolicyEscalation
 *     PolicyRejection
 *     PolicySimulation
 *     PolicyAdaptation
 *     PolicySandbox
 *     PolicyDeterminism
 *     PolicyReproducibility
 *     PolicyEffect
 *     PolicyProvenance
 *     PolicyContractReference
 *     PolicyComposition
 *     PolicyProperty
 *
 * The AST MUST remain independent of:
 *
 *     LLVM
 *     QIR
 *     MLIR
 *     vendor SDKs
 *     physical devices
 *     physical qubits
 *     routing state
 *     scheduling state
 *     calibration state
 *     HAL handles
 *
 *
 * ============================================================================
 * 33. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     policy identity resolution
 *     property validation
 *     scope resolution
 *     applicability
 *     requirement satisfaction
 *     constraint validation
 *     capability resolution
 *     resource feasibility
 *     policy conflict detection
 *     policy precedence
 *     composition
 *     inheritance
 *     override semantics
 *     effect compatibility
 *     contract interaction
 *     security interaction
 *     adaptation authorization
 *     provenance
 *     determinism
 *     reproducibility
 *
 * The parser does NONE of these.
 *
 *
 * ============================================================================
 * 34. CAPABILITY CONTRACT
 * ============================================================================
 *
 * A capability means:
 *
 *     what an environment can provide.
 *
 * A policy means:
 *
 *     what governing behavior is desired or permitted.
 *
 * Therefore:
 *
 *     capability != permission
 *
 * and:
 *
 *     permission != capability availability
 *
 * Both must remain distinct in semantic analysis.
 *
 *
 * ============================================================================
 * 35. RESOURCE CONTRACT
 * ============================================================================
 *
 * Policy expressions may refer to resources.
 *
 * Resource semantics remain owned by:
 *
 *     grammar/resources/
 *
 * The policy grammar never allocates resources.
 *
 * It never chooses:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     node
 *     device
 *     memory bank
 *     physical qubit
 *     network link
 *
 *
 * ============================================================================
 * 36. SECURITY CONTRACT
 * ============================================================================
 *
 * ALLOW/PERMIT/FORBID/DENY are universal policy constructs.
 *
 * They are NOT automatically security authorization.
 *
 * Security-specific interpretation is performed by:
 *
 *     grammar/security/
 *
 * and its semantic subsystem.
 *
 * This permits the same policy grammar to govern:
 *
 *     optimization
 *     execution
 *     deployment
 *     resource selection
 *     simulation
 *     adaptation
 *     security
 *     quantum execution
 *     distributed execution
 *
 *
 * ============================================================================
 * 37. QUANTUM CONTRACT
 * ============================================================================
 *
 * Quantum policy expressions remain quantum-neutral.
 *
 * A policy can semantically refer to:
 *
 *     quantum measurement
 *     dynamic control
 *     resilience
 *     fault tolerance
 *     error correction
 *     routing preferences
 *     scheduling preferences
 *     QPU capabilities
 *     simulation
 *
 * but this file does not define:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     CNOT
 *     physical qubit maps
 *     coupling maps
 *     calibration
 *     QEC implementation
 *
 * Quantum lowering remains:
 *
 *     policy semantics
 *       |
 *       v
 *     quantum semantic model
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     decomposition
 *       |
 *       v
 *     routing
 *       |
 *       v
 *     scheduling
 *       |
 *       v
 *     resilience / QEC
 *       |
 *       v
 *     ZQN
 *       |
 *       v
 *     HAL
 *
 *
 * ============================================================================
 * 38. HDL / HARDWARE CONTRACT
 * ============================================================================
 *
 * Hardware policy describes intent.
 *
 * It does not encode:
 *
 *     register widths
 *     fixed device counts
 *     physical wire counts
 *     physical memory capacities
 *     fixed FPGA dimensions
 *     fixed ASIC dimensions
 *     fixed processor counts
 *
 * Hardware realization remains downstream.
 *
 *
 * ============================================================================
 * 39. DISTRIBUTED CONTRACT
 * ============================================================================
 *
 * Policy can govern:
 *
 *     distributed execution
 *     consistency
 *     communication
 *     placement preference
 *     failure handling
 *     retry
 *     recovery
 *     resilience
 *     negotiation
 *
 * No node count or topology size is hard-coded.
 *
 *
 * ============================================================================
 * 40. AI / MODEL CONTRACT
 * ============================================================================
 *
 * Policy can govern:
 *
 *     learning
 *     adaptation
 *     inference
 *     model selection
 *     evidence
 *     provenance
 *     explainability
 *     reproducibility
 *     resource selection
 *
 * AI semantics remain downstream.
 *
 * This grammar does not create an application-specific AI keyword catalogue.
 *
 *
 * ============================================================================
 * 41. POCO-REAF CONTRACT
 * ============================================================================
 *
 * A policy is portable when its semantic intent remains independent of a
 * particular physical realization.
 *
 * The same policy may therefore be evaluated against:
 *
 *     tiny embedded systems
 *     CPUs
 *     multicore systems
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     QPUs
 *     simulators
 *     HPC systems
 *     clusters
 *     distributed systems
 *     heterogeneous systems
 *     cloud systems
 *     future computational substrates
 *
 * provided the target satisfies the semantic requirements and constraints.
 *
 * Policy syntax never selects a physical resource merely because a resource
 * exists.
 *
 *
 * ============================================================================
 * 42. SCALABILITY CONTRACT
 * ============================================================================
 *
 * There is NO language-level finite limit on:
 *
 *     policy count
 *     member count
 *     rule count
 *     property count
 *     clause count
 *     expression size
 *     qualified-name depth
 *     composition depth
 *     requirement count
 *     capability count
 *     resource count
 *     fallback count
 *     policy nesting
 *
 * ANTLR repetition is used instead of fixed cardinalities.
 *
 * Practical limits are implementation/resource limits.
 *
 * They are not language limits.
 *
 *
 * ============================================================================
 * 43. HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This grammar contains no:
 *
 *     MAX_POLICIES
 *     MAX_RULES
 *     MAX_RESOURCES
 *     MAX_CAPABILITIES
 *     MAX_TARGETS
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
 * No equivalent indirect grammar cardinality is introduced.
 *
 *
 * ============================================================================
 * 44. DETERMINISM CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no actions
 *     no semantic predicates
 *     no Rust code
 *     no filesystem access
 *     no network access
 *     no hardware access
 *     no runtime execution
 *     no randomness
 *     no environment inspection
 *
 * Identical source token streams and parser configuration must produce
 * structurally equivalent parse trees.
 *
 *
 * ============================================================================
 * 45. SAFETY CONTRACT
 * ============================================================================
 *
 * This grammar requires no unsafe implementation.
 *
 * The downstream Zamani implementation remains:
 *
 *     Rust 2021
 *     Rust 1.97+
 *     safe Rust only
 *
 * This file contains no Rust actions or embedded target-language code.
 *
 *
 * ============================================================================
 * 46. ERROR CONTRACT
 * ============================================================================
 *
 * Parser diagnostics cover structural errors such as:
 *
 *     missing policy name
 *     missing policy body
 *     malformed rule
 *     malformed action
 *     malformed requirement
 *     malformed constraint
 *     malformed capability
 *     malformed resource
 *     missing assignment
 *     missing semicolon
 *     malformed composition
 *
 * Semantic diagnostics belong downstream:
 *
 *     unknown policy
 *     conflicting policy
 *     unsatisfied requirement
 *     unavailable capability
 *     impossible resource requirement
 *     unauthorized action
 *     invalid adaptation
 *     invalid effect
 *     incompatible policy composition
 *     impossible target
 *
 *
 * ============================================================================
 * 47. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This grammar intentionally avoids introducing new mandatory lexical tokens
 * for:
 *
 *     scope
 *     priority
 *     description
 *     metadata
 *     subject
 *     outcome
 *     duration
 *     provenance attributes
 *     explanation attributes
 *     evidence attributes
 *     future policy concepts
 *
 * Such information can use:
 *
 *     identifier = expression;
 *
 * until a future language version explicitly promotes a concept to a
 * first-class keyword.
 *
 * This reduces keyword collisions and preserves source compatibility.
 *
 *
 * ============================================================================
 * 48. OPEN-WORLD CONTRACT
 * ============================================================================
 *
 * Policy values are expressions.
 *
 * Policy property names are identifiers.
 *
 * Consequently the grammar does not enumerate:
 *
 *     vendors
 *     processors
 *     GPUs
 *     QPUs
 *     operating systems
 *     schedulers
 *     routing algorithms
 *     learning algorithms
 *     models
 *     databases
 *     protocols
 *     cloud platforms
 *     deployment platforms
 *     future architectures
 *
 * New semantic policy concepts can therefore be added without expanding the
 * universal keyword catalogue.
 *
 *
 * ============================================================================
 * 49. INTEGRATION CONTRACT
 * ============================================================================
 *
 * THIS FILE
 * ---------
 *
 *     grammar/policies/Policy.g4
 *
 * is the leaf policy grammar.
 *
 *
 * CORE COMPOSITION
 * ----------------
 *
 * `grammar/core/core.g4` currently imports:
 *
 *     ZamaniPolicies
 *
 * The production integration must replace that policy component with the
 * canonical Policy grammar through the policy-directory dispatcher described
 * below.
 *
 *
 * RECOMMENDED POLICY DISPATCHER
 * -----------------------------
 *
 * Create:
 *
 *     grammar/policies/Policies.g4
 *
 * containing:
 *
 *     parser grammar Policies;
 *
 *     options {
 *         tokenVocab = ZamaniLexer;
 *     }
 *
 *     import Policy;
 *
 * This makes:
 *
 *     Policies
 *
 * the directory composition root and:
 *
 *     Policy
 *
 * the leaf authority.
 *
 *
 * CORE INTEGRATION
 * ----------------
 *
 * Change the Core import from:
 *
 *     ZamaniPolicies
 *
 * to:
 *
 *     Policies
 *
 * and remove the old duplicated policy implementation from:
 *
 *     grammar/core/policies.g4
 *
 * or convert that old file into a compatibility wrapper.
 *
 *
 * ROOT PARSER
 * -----------
 *
 * `grammar/antlr/ZamaniParser.g4` already imports:
 *
 *     Core
 *
 * Therefore no policy rule should be added directly to ZamaniParser.
 *
 *
 * EXECUTION
 * ---------
 *
 *     grammar/execution/policies.g4
 *
 * remains an adapter/consumer.
 *
 * It must not redefine:
 *
 *     policyDeclaration
 *     policyRule
 *     policyPermission
 *     policyProhibition
 *     policyPreference
 *     policyFallback
 *
 *
 * SECURITY
 * --------
 *
 *     grammar/security/policies.g4
 *
 * remains a security-specific consumer.
 *
 * It may interpret:
 *
 *     allow
 *     permit
 *     forbid
 *     deny
 *     sandbox
 *
 * but does not become the universal policy grammar.
 *
 *
 * RESOURCES
 * ---------
 *
 *     grammar/resources/
 *
 * consumes:
 *
 *     policyRequirement
 *     policyCapability
 *     policyResource
 *
 * through semantic integration.
 *
 *
 * VALIDATION
 * ----------
 *
 *     grammar/validation/
 *
 * consumes policy-related:
 *
 *     requirements
 *     constraints
 *     contracts
 *     properties
 *     evidence
 *
 * without redefining policy syntax.
 *
 *
 * EFFECTS
 * -------
 *
 *     grammar/effects/
 *
 * consumes:
 *
 *     policyEffect
 *     policyAdaptation
 *     policySimulation
 *
 * while retaining ownership of effect semantics.
 *
 *
 * QUANTUM
 * -------
 *
 *     grammar/quantum/
 *
 * consumes policy intent before the semantic quantum model reaches:
 *
 *     quantum::ir
 *
 *
 * ============================================================================
 * 50. AST INTEGRATION
 * ============================================================================
 *
 * Required AST mapping:
 *
 *     policyDeclaration
 *         -> Policy
 *
 *     policyRule
 *         -> PolicyRule
 *
 *     policyPermission
 *         -> PolicyPermission
 *
 *     policyProhibition
 *         -> PolicyProhibition
 *
 *     policyPreference
 *         -> PolicyPreference
 *
 *     policyFallback
 *         -> PolicyFallback
 *
 *     policyRequirement
 *         -> PolicyRequirement
 *
 *     policyConstraint
 *         -> PolicyConstraint
 *
 *     policyCapability
 *         -> PolicyCapability
 *
 *     policyResource
 *         -> PolicyResource
 *
 *     policyProperty
 *         -> PolicyProperty
 *
 *     policyComposition
 *         -> PolicyComposition
 *
 * All nodes preserve source spans.
 *
 *
 * ============================================================================
 * 51. SEMANTIC INTEGRATION
 * ============================================================================
 *
 * The semantic policy model should normalize all policy actions into a common
 * representation rather than creating unrelated execution systems.
 *
 * Conceptually:
 *
 *     Policy
 *       |
 *       +--> Applicability
 *       +--> Conditions
 *       +--> Requirements
 *       +--> Constraints
 *       +--> Capabilities
 *       +--> Resources
 *       +--> Effects
 *       +--> Permissions
 *       +--> Prohibitions
 *       +--> Preferences
 *       +--> Fallbacks
 *       +--> Adaptation
 *       +--> Provenance
 *       +--> Composition
 *
 *
 * ============================================================================
 * 52. IR INTEGRATION
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * Policy semantics may influence:
 *
 *     canonical semantic representation
 *     classical IR
 *     quantum::ir
 *     HDL/hardware representation
 *     distributed representation
 *
 * but policy syntax must never directly emit:
 *
 *     machine instructions
 *     quantum gates
 *     physical signals
 *     physical qubit assignments
 *     routing commands
 *     scheduler commands
 *     calibration commands
 *     device handles
 *
 *
 * ============================================================================
 * 53. TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE TESTS
 * --------------
 *
 *     policy execution {
 *         requires capability("execution.deterministic");
 *         deterministic;
 *     }
 *
 *     policy portable {
 *         resource memory;
 *         target execution;
 *         prefer execution::deterministic;
 *         fallback simulation;
 *     }
 *
 *     policy quantum_execution {
 *         requires capability("quantum.measurement");
 *         capability quantum::dynamic_control;
 *         prefer quantum::resilience;
 *         fallback simulation;
 *     }
 *
 *     policy hybrid_ai {
 *         requires capability("tensor.compute");
 *         resource accelerator;
 *         adapt model;
 *         provenance = execution::trace;
 *     }
 *
 *     policy governed {
 *         allow execution;
 *         forbid network;
 *         sandbox execution::restricted;
 *         reproducible;
 *     }
 *
 *     policy conditional {
 *         when workload::size > threshold
 *             => prefer execution::parallel;
 *         when capability::quantum::measurement
 *             => prefer quantum::execution;
 *     }
 *
 *
 * NEGATIVE TESTS
 * --------------
 *
 *     policy;
 *
 *     policy name;
 *
 *     policy name { requires; }
 *
 *     policy name { capability; }
 *
 *     policy name { constraint; }
 *
 *     policy name { when => allow x; }
 *
 *     policy name { allow; }
 *
 *     policy name { extends; }
 *
 *
 * BOUNDARY TESTS
 * --------------
 *
 *     classical policy
 *     quantum policy
 *     hybrid policy
 *     HDL policy
 *     hardware policy
 *     AI policy
 *     distributed policy
 *     networking policy
 *     security policy
 *     simulation policy
 *     mixed-domain policy
 *
 *
 * SCALABILITY TESTS
 * -----------------
 *
 * Generate policies containing increasingly large numbers of:
 *
 *     members
 *     rules
 *     requirements
 *     capabilities
 *     properties
 *     expressions
 *     composition relationships
 *
 * No grammar-defined finite threshold is permitted.
 *
 *
 * PORTABILITY TESTS
 * -----------------
 *
 * The same policy source must be semantically usable where appropriate for:
 *
 *     embedded
 *     CPU
 *     multicore
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     simulator
 *     HPC
 *     cluster
 *     distributed
 *     cloud
 *     heterogeneous
 *     future target
 *
 * Target feasibility is a semantic/backend concern, not a parser concern.
 *
 *
 * DETERMINISM TESTS
 * -----------------
 *
 * Identical source plus identical parser configuration must yield equivalent
 * parse structure.
 *
 *
 * COMPATIBILITY TESTS
 * -------------------
 *
 * Existing policy constructs represented by the current repository must remain
 * expressible where their semantics are still supported.
 *
 * In particular:
 *
 *     policy
 *     requires
 *     constraint
 *     capability
 *     resource
 *     allow
 *     permit
 *     forbid
 *     deny
 *     prefer
 *     fallback
 *     select
 *     negotiate
 *     retry
 *     recover
 *     escalate
 *     reject
 *     simulate
 *     adapt
 *     sandbox
 *     deterministic
 *     reproducible
 *
 * remain canonical.
 *
 *
 * ============================================================================
 * 54. COMPLETION CRITERIA
 * ============================================================================
 *
 * THIS FILE IS COMPLETE WHEN:
 *
 *     [x] It is parser-only.
 *     [x] It uses the canonical ZamaniLexer vocabulary.
 *     [x] It has one policy declaration authority.
 *     [x] It does not redefine expressions.
 *     [x] It does not redefine requirements.
 *     [x] It does not redefine constraints.
 *     [x] It does not redefine capabilities.
 *     [x] It does not define resource allocation.
 *     [x] It does not define security enforcement.
 *     [x] It does not define runtime behavior.
 *     [x] It does not define IR.
 *     [x] It contains no embedded Rust.
 *     [x] It requires no unsafe Rust.
 *     [x] It contains no fixed machine-capacity limits.
 *     [x] It is open-world.
 *     [x] It preserves POCO-REAF.
 *     [x] It supports classical policy intent.
 *     [x] It supports quantum policy intent.
 *     [x] It supports hybrid policy intent.
 *     [x] It supports HDL/hardware policy intent.
 *     [x] It supports distributed policy intent.
 *     [x] It supports AI/model policy intent.
 *     [x] It supports security policy consumption.
 *     [x] It supports simulation.
 *     [x] It supports adaptation.
 *     [x] It supports provenance.
 *     [x] It supports reproducibility.
 *
 * VERIFICATION REQUIRED BY THE BUILD:
 *
 *     [ ] ANTLR generation succeeds.
 *     [ ] All imports resolve.
 *     [ ] No duplicate rule names exist.
 *     [ ] No undefined token references exist.
 *     [ ] No unreachable policy rules exist.
 *     [ ] No ambiguity warnings remain.
 *     [ ] Core composition succeeds.
 *     [ ] ZamaniParser generation succeeds.
 *     [ ] Rust frontend generation succeeds.
 *     [ ] Positive tests pass.
 *     [ ] Negative tests pass.
 *     [ ] Boundary tests pass.
 *     [ ] Scalability tests pass.
 *     [ ] Determinism tests pass.
 *     [ ] Compatibility tests pass.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * A Zamani policy expresses GOVERNING INTENT.
 *
 * It does not encode the physical universe.
 *
 * The semantic system determines whether and how that intent can be realized.
 *
 * Therefore:
 *
 *     policy
 *       |
 *       v
 *     semantic intent
 *       |
 *       +--> requirements
 *       +--> capabilities
 *       +--> resources
 *       +--> constraints
 *       +--> effects
 *       +--> contracts
 *       +--> security
 *       +--> provenance
 *       |
 *       v
 *     target-independent planning
 *       |
 *       v
 *     target realization
 *
 * This is the policy boundary required for Program_Once_Compile_Once_Run_
 * Everywhere_Anywhere_Forever.
 *
 * ============================================================================
 */
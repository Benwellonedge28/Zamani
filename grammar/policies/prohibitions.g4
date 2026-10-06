/*
 * ============================================================================
 * ZAMANI UNIVERSAL PROGRAMMING LANGUAGE
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/policies/prohibitions.g4
 *
 * GRAMMAR
 * -------
 * ANTLR4 parser grammar
 *
 * STATUS
 * ------
 * CANONICAL POLICY PROHIBITION LEAF GRAMMAR
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
 * This file owns the canonical SOURCE-LEVEL SYNTAX for declarative policy
 * prohibitions.
 *
 * A prohibition expresses a semantic restriction:
 *
 *     "this policy does not permit this action, capability, effect, resource
 *      use, transformation, execution path, deployment choice, or other
 *      semantically represented operation."
 *
 * The prohibition target is intentionally an OPEN-ENDED Zamani expression.
 *
 * Therefore this grammar does not enumerate:
 *
 *     resources
 *     hardware
 *     devices
 *     CPUs
 *     GPUs
 *     FPGAs
 *     QPUs
 *     qubits
 *     networks
 *     effects
 *     capabilities
 *     AI models
 *     quantum operations
 *     HDL operations
 *     vendors
 *     applications
 *     protocols
 *     algorithms
 *
 * New semantic domains remain representable without modifying this grammar.
 *
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * The prohibition participates in the common Zamani policy pipeline:
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
 *     structural validation
 *       |
 *       v
 *     semantic policy model
 *       |
 *       +--> capability analysis
 *       +--> resource analysis
 *       +--> effect analysis
 *       +--> contract analysis
 *       +--> security analysis
 *       +--> provenance
 *       |
 *       v
 *     policy resolution
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
 *     target-independent optimization / lowering
 *       |
 *       v
 *     execution planning
 *       |
 *       v
 *     target realization
 *
 * This grammar participates ONLY in source parsing.
 *
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns:
 *
 *     prohibitionClause
 *     prohibitionKeyword
 *     prohibitionTarget
 *
 * It may also own future purely syntactic prohibition modifiers when those
 * modifiers have been formally specified by the policy specification.
 *
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     lexer definitions
 *     keyword definitions
 *     identifiers
 *     qualified names
 *     general expressions
 *     types
 *     capabilities
 *     resources
 *     requirements
 *     constraints
 *     permissions
 *     preferences
 *     fallbacks
 *     policy scopes
 *     policy inheritance
 *     policy precedence
 *     policy resolution
 *     security authorization
 *     runtime enforcement
 *     effect semantics
 *     contracts
 *     provenance semantics
 *     classical IR
 *     quantum::ir
 *     HDL IR
 *     hardware realization
 *     scheduling
 *     routing
 *     QEC
 *     ZQN
 *     HAL
 *
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * This file is the canonical parser leaf for prohibition syntax.
 *
 * No other grammar file should independently redefine:
 *
 *     FORBID <target> ;
 *     DENY   <target> ;
 *
 * Policy containers consume:
 *
 *     prohibitionClause
 *
 * rather than recreating the syntax.
 *
 *
 * ============================================================================
 * OPEN-WORLD RULE
 * ============================================================================
 *
 * Prohibition targets are expressions.
 *
 * This is intentional.
 *
 * The grammar must remain capable of representing future semantic entities
 * without requiring grammar changes.
 *
 * For example, all of these are syntactically representable when their names
 * and surrounding expressions are valid Zamani:
 *
 *     forbid capability("network.external");
 *
 *     forbid capability("native.execute");
 *
 *     forbid effect("network");
 *
 *     forbid effect("mutation");
 *
 *     forbid resource("memory");
 *
 *     forbid operation("vendor.future_operation");
 *
 *     forbid target("some.execution.substrate");
 *
 *     forbid quantum.operation("future_gate");
 *
 *     forbid hardware.feature("future_feature");
 *
 *     forbid foreign.call("external_function");
 *
 * The grammar does not need to know what any of those entities mean.
 *
 * Semantic validation determines whether the referenced entity exists and
 * whether the prohibition is meaningful.
 *
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar contains NO language-level capacity limits.
 *
 * It does not define maximum:
 *
 *     prohibitions
 *     policy members
 *     policy targets
 *     resources
 *     capabilities
 *     devices
 *     CPUs
 *     GPUs
 *     FPGAs
 *     QPUs
 *     qubits
 *     nodes
 *     actors
 *     threads
 *     memory
 *     storage
 *     tensor dimensions
 *     network topology
 *
 * Repetition and collection size are bounded only by:
 *
 *     source representation
 *     parser implementation
 *     compiler/runtime resources
 *
 * and not by a grammar constant.
 *
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * A prohibition constrains semantic intent rather than a physical target.
 *
 * Therefore the same prohibition can be evaluated against:
 *
 *     embedded systems
 *     CPUs
 *     multicore systems
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     QPUs
 *     quantum simulators
 *     HPC systems
 *     clusters
 *     distributed systems
 *     cloud systems
 *     future execution substrates
 *
 * Target feasibility and policy satisfaction are semantic/backend concerns.
 *
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing this grammar must depend only on:
 *
 *     source characters
 *     lexical vocabulary
 *     grammar version
 *     explicitly selected compatibility version
 *
 * It must NOT depend on:
 *
 *     hardware availability
 *     runtime state
 *     filesystem state
 *     network state
 *     current time
 *     random state
 *     resource availability
 *     target selection
 *
 *
 * ============================================================================
 * LEXER DEPENDENCIES
 * ============================================================================
 *
 * Required canonical tokens:
 *
 *     FORBID
 *     DENY
 *     SEMICOLON
 *
 * These are already part of the repository's canonical policy vocabulary.
 *
 * This file does NOT create lexer tokens.
 *
 *
 * ============================================================================
 * GRAMMAR DEPENDENCIES
 * ============================================================================
 *
 * Required parser dependency:
 *
 *     Expressions
 *
 * Required exported expression rule:
 *
 *     expression
 *
 * The prohibition target is intentionally an expression rather than an
 * identifier or a fixed policy-resource grammar.
 *
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar produces the syntactic representation required to construct
 * one domain-neutral prohibition node.
 *
 * Recommended semantic AST shape:
 *
 *     PolicyProhibition
 *     {
 *         kind,
 *         target,
 *         source
 *     }
 *
 * where:
 *
 *     kind   = FORBID | DENY
 *     target = parsed expression
 *     source = source span
 *
 * The AST MUST NOT resolve:
 *
 *     capabilities
 *     resources
 *     hardware
 *     targets
 *     effects
 *     security identities
 *     physical devices
 *
 * Such resolution belongs to semantic analysis.
 *
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * A prohibition means that a matching semantic action, capability, effect,
 * resource use, operation, or other policy subject is not permitted under the
 * applicable policy.
 *
 * FORBID and DENY are syntactic policy forms.
 *
 * Their semantic relationship MUST be defined by the policy specification.
 *
 * If the language defines them as equivalent, semantic normalization may map
 * both to one canonical prohibition kind.
 *
 * If the language later assigns different semantic strengths, the distinction
 * remains available through prohibitionKeyword.
 *
 * This grammar deliberately does not decide that question.
 *
 *
 * ============================================================================
 * TARGET SEMANTICS
 * ============================================================================
 *
 * The target expression may represent any semantically supported policy
 * subject.
 *
 * Examples include:
 *
 *     capability(...)
 *     resource(...)
 *     effect(...)
 *     operation(...)
 *     target(...)
 *     service(...)
 *     device(...)
 *     foreign(...)
 *     network(...)
 *     quantum(...)
 *     hardware(...)
 *
 * The grammar does not require any particular one of these.
 *
 * This preserves an open semantic universe.
 *
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * A prohibition target is parsed as an expression.
 *
 * Semantic analysis MUST determine whether the resulting expression denotes a
 * valid prohibition subject.
 *
 * Invalid examples include expressions that cannot participate in policy
 * semantics according to the policy/type specification.
 *
 * Type checking belongs outside this grammar.
 *
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing a prohibition produces NO runtime effect.
 *
 * The prohibition itself is declarative.
 *
 * The semantic policy system may use a prohibition to constrain operations
 * carrying effects such as:
 *
 *     IO
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
 *     code_generation
 *     simulation
 *
 * This file does not enumerate or interpret those effects.
 *
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * A prohibition MAY semantically constrain any capability represented by its
 * target expression.
 *
 * Capability resolution belongs to the capability subsystem.
 *
 * This grammar does not enumerate capability names.
 *
 * Therefore future capabilities do not require modification of this file.
 *
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * A prohibition MAY semantically constrain resource usage when the target
 * expression denotes a resource or resource-related operation.
 *
 * This grammar does NOT impose:
 *
 *     minimum resources
 *     maximum resources
 *     fixed capacities
 *     physical sizes
 *     machine-specific quantities
 *
 * Resource feasibility remains owned by resource analysis and negotiation.
 *
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * Prohibitions may participate in policy resolution alongside:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * A prohibition MUST NOT silently rewrite or weaken a contract.
 *
 * Conflict resolution belongs to semantic policy analysis.
 *
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * This file defines the prohibition primitive only.
 *
 * Policy scope, inheritance, composition, precedence, exceptions, and
 * applicability belong to the policy system.
 *
 * In particular, this file does NOT own:
 *
 *     scope
 *     extends
 *     override
 *     inheritance
 *     priority
 *     precedence
 *     fallback
 *     policy selection
 *     policy negotiation
 *
 * A containing policy may therefore apply this primitive to a scope defined
 * elsewhere.
 *
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * The prohibition AST must preserve source provenance sufficient for:
 *
 *     diagnostics
 *     semantic validation
 *     policy-resolution explanations
 *     audit
 *     reproducibility
 *     compilation provenance
 *
 * At minimum the semantic node should retain the source span.
 *
 * Provenance generation and storage are owned by the provenance subsystem.
 *
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar does NOT directly lower to classical IR or quantum::ir.
 *
 * The expected pipeline is:
 *
 *     prohibition syntax
 *       |
 *       v
 *     PolicyProhibition semantic node
 *       |
 *       v
 *     policy resolution
 *       |
 *       +--> semantic constraints
 *       +--> capability constraints
 *       +--> resource constraints
 *       +--> effect restrictions
 *       +--> security restrictions
 *       |
 *       v
 *     canonical semantic model
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> HDL/hardware representation
 *       +--> other domain IRs
 *
 * This separation is mandatory.
 *
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * No quantum-specific syntax is defined here.
 *
 * A prohibition may semantically constrain quantum behavior through an open
 * expression target.
 *
 * Example:
 *
 *     forbid capability("quantum.measurement");
 *
 * or another policy expression defined by the semantic capability model.
 *
 * The resulting restriction is consumed before or during the quantum::ir
 * pipeline.
 *
 * This grammar must never enumerate physical quantum gates or qubit counts.
 *
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * No HDL or hardware syntax is defined here.
 *
 * A prohibition may semantically constrain hardware intent, capabilities,
 * effects, deployment, or realization through the common policy model.
 *
 * Physical realization remains downstream.
 *
 *
 * ============================================================================
 * BACKEND CONTRACT
 * ============================================================================
 *
 * Backends MUST NOT interpret raw parser nodes directly.
 *
 * The required flow is:
 *
 *     parser
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     semantic policy model
 *       |
 *       v
 *     policy resolution
 *       |
 *       v
 *     execution/compilation plan
 *       |
 *       v
 *     backend
 *
 * A backend may reject a plan because a prohibition applies, but that decision
 * must be based on the resolved semantic policy rather than on raw syntax.
 *
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * The parser should report ordinary ANTLR syntax diagnostics for malformed
 * forms.
 *
 * Semantic diagnostics should be produced downstream for conditions such as:
 *
 *     unknown prohibition subject
 *     invalid prohibition target
 *     contradictory policy
 *     unreachable prohibition
 *     conflicting policy rules
 *     unsatisfiable policy
 *     prohibited required capability
 *     prohibited required effect
 *     policy conflict after inheritance
 *
 * This grammar must not attempt to perform those semantic checks.
 *
 *
 * ============================================================================
 * POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * At minimum, the following forms must parse:
 *
 *     forbid capability("network.external");
 *
 *     deny capability("native.execute");
 *
 *     forbid effect("network");
 *
 *     deny effect("mutation");
 *
 *     forbid resource("some.resource");
 *
 *     forbid operation("some.operation");
 *
 *     deny target("some.target");
 *
 *     forbid quantum.operation("some.operation");
 *
 *     deny hardware.feature("some.feature");
 *
 *     forbid foreign.call("some.call");
 *
 * The test suite must treat these as representative open-world examples,
 * not as a closed catalogue.
 *
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The following must be rejected by the parser:
 *
 *     forbid;
 *
 *     deny;
 *
 *     forbid();
 *
 *     deny();
 *
 *     forbid capability("network.external")
 *         // missing semicolon
 *
 *     deny capability("network.external"
 *         // missing closing delimiter
 *
 * The exact diagnostic wording is owned by the diagnostics subsystem.
 *
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Test interaction with:
 *
 *     requirements
 *     constraints
 *     permissions
 *     preferences
 *     fallbacks
 *     scopes
 *     contracts
 *     effects
 *     capabilities
 *     resources
 *     provenance
 *     security
 *     execution
 *     quantum
 *     HDL
 *     distributed execution
 *     interoperability
 *
 * Particularly important semantic conflicts include:
 *
 *     requires capability("X");
 *     forbid capability("X");
 *
 * The parser MUST accept both constructs independently.
 *
 * The semantic policy resolver determines whether their combination is
 * contradictory or otherwise unsatisfiable.
 *
 *
 * ============================================================================
 * SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests must demonstrate that prohibition syntax does not depend on fixed
 * domain cardinalities.
 *
 * Test targets should include:
 *
 *     one prohibition
 *     many prohibitions
 *     deeply qualified names
 *     deeply nested expressions
 *     large symbolic policy expressions
 *     generated policy collections
 *     large resource/capability identifiers
 *
 * No test may establish a universal maximum such as:
 *
 *     MAX_PROHIBITIONS
 *     MAX_POLICY_TARGETS
 *     MAX_CAPABILITIES
 *
 * Any implementation limits must be implementation/resource limits rather
 * than language-semantic constants.
 *
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Canonical spellings:
 *
 *     forbid
 *     deny
 *
 * are represented by:
 *
 *     FORBID
 *     DENY
 *
 * Legacy spellings MUST NOT be introduced here merely for convenience.
 *
 * If compatibility aliases are required in the future, they must be handled
 * through the compatibility subsystem and mapped to the canonical semantic
 * prohibition representation.
 *
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * REQUIRED IMPORT:
 *
 *     Expressions
 *
 * REQUIRED CONSUMERS:
 *
 *     grammar/policies/policy.g4
 *
 * POTENTIAL CONSUMERS:
 *
 *     grammar/policies/scopes.g4
 *     grammar/policies/permissions.g4
 *     grammar/policies/preferences.g4
 *     grammar/policies/fallbacks.g4
 *     grammar/policies/security.g4
 *     grammar/policies/execution.g4
 *     grammar/security/policy.g4
 *     grammar/execution/policies.g4
 *
 * Those consumers MUST use the exported:
 *
 *     prohibitionClause
 *
 * rule rather than duplicating FORBID/DENY syntax.
 *
 *
 * ============================================================================
 * DEPENDENCY DECLARATION
 * ============================================================================
 *
 * DEPENDS_ON:
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/expressions/Expressions.g4
 *
 * EXPORTS:
 *     prohibitionClause
 *     prohibitionKeyword
 *     prohibitionTarget
 *
 * CONSUMED_BY:
 *     grammar/policies/policy.g4
 *     policy composition/adapter grammars
 *
 * AST_OWNER:
 *     Zamani AST / policy semantic model
 *
 * SEMANTIC_OWNER:
 *     policy semantic analysis / policy resolver
 *
 * IR_OWNER:
 *     canonical semantic model and downstream domain IR boundaries
 *
 * TEST_OWNER:
 *     grammar/tests/policies/prohibitions/
 *
 * SPEC_OWNER:
 *     grammar/spec/policies.md
 *
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     1. It compiles as an ANTLR4 parser grammar.
 *
 *     2. It imports only canonical dependencies.
 *
 *     3. It defines prohibition syntax exactly once.
 *
 *     4. FORBID and DENY are consumed from the canonical lexer vocabulary.
 *
 *     5. Prohibition targets remain open-ended expressions.
 *
 *     6. No physical hardware capacity is encoded.
 *
 *     7. No domain-specific operation catalogue is encoded.
 *
 *     8. No resource maximum is encoded.
 *
 *     9. The resulting syntax can be represented by a domain-neutral AST.
 *
 *    10. Policy semantics remain outside the parser.
 *
 *    11. Policy scope remains outside this file.
 *
 *    12. Policy precedence/conflict resolution remains outside this file.
 *
 *    13. Provenance remains available through source spans.
 *
 *    14. Classical and quantum consumers do not require grammar changes.
 *
 *    15. HDL/hardware consumers do not require grammar changes.
 *
 *    16. Future capabilities/resources/operations remain representable.
 *
 *    17. Positive, negative, boundary, scalability, and compatibility tests
 *        exist.
 *
 *    18. The policy container delegates to this rule instead of duplicating
 *        it.
 *
 * ============================================================================
 */

parser grammar Prohibitions;

options {
    tokenVocab = ZamaniLexer;
}

import
    Expressions
;


/*
 * ============================================================================
 * CANONICAL PROHIBITION
 * ============================================================================
 *
 * The semicolon belongs to the prohibition construct.
 *
 * This guarantees that policy containers can consume a complete prohibition
 * without needing to know its internal syntax.
 *
 * Canonical forms:
 *
 *     forbid <expression>;
 *     deny   <expression>;
 */
prohibitionClause
    : prohibitionKeyword
      prohibitionTarget
      SEMICOLON
    ;


/*
 * ============================================================================
 * PROHIBITION KEYWORD
 * ============================================================================
 *
 * FORBID and DENY are intentionally retained as separate lexical forms.
 *
 * Semantic normalization may later map them to one canonical prohibition
 * operation if the specification declares them equivalent.
 */
prohibitionKeyword
    : FORBID
    | DENY
    ;


/*
 * ============================================================================
 * PROHIBITION TARGET
 * ============================================================================
 *
 * The target is deliberately an ordinary Zamani expression.
 *
 * This is the critical open-world/scalability rule.
 *
 * Do NOT replace this with a finite catalogue such as:
 *
 *     capabilityName
 *     resourceName
 *     deviceName
 *     quantumOperation
 *     hardwareFeature
 *
 * Doing so would make the grammar a closed-world policy catalogue.
 */
prohibitionTarget
    : expression
    ;
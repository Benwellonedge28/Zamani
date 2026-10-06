/*
 * ============================================================================
 * ZAMANI UNIVERSAL PROGRAMMING LANGUAGE
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/policies/permissions.g4
 *
 * GRAMMAR
 * -------
 * ANTLR4 parser grammar
 *
 * GRAMMAR NAME
 * ------------
 * PolicyPermissions
 *
 * STATUS
 * ------
 * CANONICAL POLICY PERMISSION LEAF GRAMMAR
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
 * This file owns the source-level syntax for PERMISSION INTENT inside the
 * universal Zamani policy system.
 *
 * A policy permission expresses that a policy permits or authorizes some
 * semantic operation, subject, action, resource, or other policy target.
 *
 * This is SOURCE-LEVEL POLICY INTENT.
 *
 * It is NOT runtime authorization enforcement.
 *
 * It is NOT authentication.
 *
 * It is NOT identity verification.
 *
 * It is NOT credential validation.
 *
 * It is NOT capability discovery.
 *
 * It is NOT resource allocation.
 *
 * It is NOT hardware selection.
 *
 * It is NOT a security runtime.
 *
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * The policy pipeline is:
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     PolicyPermissions
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic policy model
 *          |
 *          +--> permission intent
 *          +--> requirements
 *          +--> constraints
 *          +--> capabilities
 *          +--> effects
 *          +--> contracts
 *          +--> provenance
 *          |
 *          v
 *     policy analysis
 *          |
 *          +--> security analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          +--> execution planning
 *          +--> adaptation
 *          +--> deployment
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL / hardware representation
 *          +--> distributed representation
 *          |
 *          v
 *     target-independent optimization
 *          |
 *          v
 *     lowering / routing / scheduling / resilience
 *          |
 *          v
 *     target realization
 *
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns:
 *
 *     policyPermission
 *     policyPermissionStatement
 *
 *     policyPermissionDecision
 *     policyPermissionTarget
 *
 *     policyPermissionReference
 *
 *     policyPermissionStructuredTarget
 *     policyPermissionSubjectClause
 *     policyPermissionActionClause
 *     policyPermissionResourceClause
 *     policyPermissionConditionClause
 *     policyPermissionObligationClause
 *
 *     policyPermissionSubject
 *     policyPermissionAction
 *     policyPermissionResource
 *     policyPermissionCondition
 *     policyPermissionObligation
 *
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *     lexer rules
 *     token definitions
 *     identifiers
 *     qualified-name syntax
 *     general expressions
 *     arithmetic
 *     logical operators
 *     types
 *
 *     authentication
 *     identity verification
 *     credentials
 *     trust establishment
 *     cryptography
 *     key management
 *
 *     security authorization evaluation
 *     runtime permission enforcement
 *     access-control evaluation
 *
 *     capability discovery
 *     capability satisfaction
 *     resource allocation
 *     resource discovery
 *
 *     physical hardware
 *     CPU selection
 *     GPU selection
 *     FPGA selection
 *     ASIC selection
 *     QPU selection
 *     node selection
 *     topology selection
 *
 *     quantum routing
 *     quantum scheduling
 *     QEC
 *     ZQN
 *     HAL
 *
 *     canonical IR
 *     backend implementation
 *
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * POLICY PERMISSION AUTHORITY
 * ---------------------------
 *
 *     grammar/policies/permissions.g4
 *
 * owns the policy-level permission syntax.
 *
 *
 * SECURITY PERMISSION AUTHORITY
 * -----------------------------
 *
 *     grammar/security/permissions.g4
 *
 * owns security-oriented permission declarations and authorization syntax.
 *
 * The two grammars MUST NOT be merged into one semantic responsibility.
 *
 *
 * POLICY AUTHORITY
 * ----------------
 *
 *     grammar/policies/policy.g4
 *
 * remains the owner of:
 *
 *     policy declarations
 *     policy bodies
 *     policy membership/composition
 *     policy-level integration
 *
 * This file supplies the reusable permission leaf rules consumed by Policy.
 *
 *
 * CAPABILITY AUTHORITY
 * --------------------
 *
 *     grammar/resources/capabilities.g4
 *
 * owns capability syntax and capability semantics.
 *
 * A permission is therefore NOT a capability.
 *
 *
 * RESOURCE AUTHORITY
 * ------------------
 *
 *     grammar/resources/
 *
 * owns resource semantics.
 *
 * A policy permission may refer to a resource expression, but this file does
 * not define what a resource physically is.
 *
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/core/names.g4
 *     grammar/expressions/expressions.g4
 *
 *
 * LEXER DEPENDENCY
 * ----------------
 *
 * This grammar consumes the canonical ZamaniLexer vocabulary.
 *
 * Required existing lexical concepts include:
 *
 *     ALLOW
 *     PERMIT
 *     PERMISSION
 *     SUBJECT
 *     ACTION
 *     RESOURCE
 *     WHEN
 *     OBLIGE
 *     SEMICOLON
 *
 * No lexer rule is declared here.
 *
 * No private keyword vocabulary is introduced here.
 *
 *
 * PARSER DEPENDENCIES
 * -------------------
 *
 *     Names
 *     Expressions
 *
 * Names provides:
 *
 *     identifier
 *     qualifiedName
 *
 * Expressions provides:
 *
 *     expression
 *
 *
 * EXPORTS
 * -------
 *
 *     policyPermission
 *     policyPermissionStatement
 *
 *
 * CONSUMED_BY
 * ----------
 *
 * Primary:
 *
 *     grammar/policies/policy.g4
 *
 * Secondary:
 *
 *     policy composition grammars
 *     execution policy adapters
 *     security policy adapters
 *     deployment policy adapters
 *     semantic policy analysis
 *
 *
 * AST_OWNER
 * ---------
 *
 *     src/ast/
 *
 * This grammar creates parser contexts only.
 *
 *
 * SEMANTIC_OWNER
 * --------------
 *
 *     Zamani policy semantic model
 *
 *
 * SECURITY_SEMANTIC_OWNER
 * -----------------------
 *
 *     Zamani security/authorization semantic subsystem
 *
 * This grammar supplies intent.
 *
 * It does not decide whether a permission is actually authorized.
 *
 *
 * IR_OWNER
 * --------
 *
 *     canonical semantic policy representation
 *
 * Permission intent must remain target-neutral until semantic lowering.
 *
 *
 * TEST_OWNER
 * ----------
 *
 *     grammar/tests/policies/
 *     grammar/tests/parser/
 *     grammar/tests/security/
 *     grammar/tests/negative/
 *     grammar/tests/boundary/
 *     grammar/tests/scalability/
 *
 *
 * SPEC_OWNER
 * ----------
 *
 *     grammar/spec/policies.md
 *     grammar/specification/policies.md
 *
 *
 * ============================================================================
 * PERMISSION VS CAPABILITY
 * ============================================================================
 *
 * These concepts MUST remain distinct.
 *
 * CAPABILITY
 * ----------
 *
 * A capability describes what an environment/target/runtime can provide.
 *
 * Example:
 *
 *     capability("quantum.measurement")
 *
 *
 * PERMISSION
 * ----------
 *
 * A permission describes what policy intent allows.
 *
 * Example:
 *
 *     allow permission quantum::measurement;
 *
 *
 * The relationship is semantic:
 *
 *     permission intent
 *          |
 *          v
 *     policy analysis
 *          |
 *          v
 *     capability/effect/security analysis
 *          |
 *          v
 *     target realization
 *
 * A permission grammar MUST NOT manufacture capability declarations.
 *
 *
 * ============================================================================
 * OPEN-WORLD DESIGN
 * ============================================================================
 *
 * Permission identities are OPEN-WORLD.
 *
 * Valid semantic identities can include:
 *
 *     permission data::read;
 *     permission quantum::execute;
 *     permission hardware::configure;
 *     permission network::connect;
 *     permission model::adapt;
 *     permission future::operation;
 *
 * No finite permission catalogue is encoded here.
 *
 * Future permission domains therefore do not require a grammar change merely
 * because a new semantic permission is introduced.
 *
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar imposes NO universal capacity limits.
 *
 * In particular, it does not encode limits on:
 *
 *     CPUs
 *     cores
 *     threads
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     QPUs
 *     qubits
 *     nodes
 *     processes
 *     tasks
 *     channels
 *     memory
 *     storage
 *     registers
 *     vector width
 *     tensor dimensions
 *     tensor rank
 *     network size
 *     device count
 *     topology size
 *
 * There is no fixed number of:
 *
 *     permissions
 *     subjects
 *     actions
 *     resources
 *     conditions
 *     obligations
 *     policy clauses
 *
 * Source cardinality is represented through ANTLR repetition and ordinary
 * expressions.
 *
 * Practical limits are determined by:
 *
 *     available compiler resources
 *     runtime resources
 *     target capabilities
 *     explicit program requirements
 *     explicit resource policies
 *
 * They are NOT language-level grammar constants.
 *
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing MUST depend only on:
 *
 *     source text
 *     lexer configuration
 *     grammar configuration
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware
 *     runtime state
 *     wall-clock time
 *     randomness
 *     filesystem state
 *     network state
 *     environment variables
 *     target availability
 *     capability discovery
 *
 *
 * ============================================================================
 * SAFETY CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     NO embedded Rust
 *     NO parser actions
 *     NO semantic predicates
 *     NO filesystem access
 *     NO network access
 *     NO hardware access
 *     NO runtime calls
 *     NO unsafe Rust
 *
 * Rust implementation compatibility is therefore delegated to the generated
 * parser integration and the repository's Rust frontend.
 *
 * Required implementation baseline:
 *
 *     Rust 2021
 *     Rust 1.97+
 *     safe Rust only
 *
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser must preserve enough structure for the AST layer to represent:
 *
 *     permission decision
 *     permission target
 *     permission reference
 *     subject expression
 *     action expression
 *     resource expression
 *     condition expression
 *     obligation expression
 *     source span
 *     declaration order
 *
 * Conceptual AST representation:
 *
 *     PolicyPermission {
 *         decision,
 *         target,
 *         source
 *     }
 *
 * where target may contain:
 *
 *     reference
 *     subject
 *     action
 *     resource
 *     condition
 *     obligations
 *
 * The AST MUST NOT resolve:
 *
 *     physical resources
 *     identities
 *     capabilities
 *     hardware
 *     runtime permissions
 *
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * The semantic layer determines:
 *
 *     whether the permission exists;
 *     what the permission denotes;
 *     whether the referenced permission is valid;
 *     whether the subject is valid;
 *     whether the action is valid;
 *     whether the resource is valid;
 *     whether the condition is valid;
 *     whether obligations are valid;
 *     whether the permission conflicts with prohibitions;
 *     whether the permission is authorized;
 *     whether the permission can be realized;
 *     whether required capabilities exist;
 *     whether required resources are available.
 *
 * None of those decisions are parser responsibilities.
 *
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Permission syntax itself has no runtime effect.
 *
 * Semantic interpretation may interact with effects such as:
 *
 *     io
 *     network
 *     native
 *     foreign
 *     mutation
 *     randomness
 *     distributed
 *     quantum
 *     measurement
 *     learning
 *     adaptation
 *     reflection
 *     code_generation
 *     simulation
 *
 * Effect ownership remains under:
 *
 *     grammar/effects/
 *
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Permission targets may refer to expressions whose semantic interpretation
 * requires capabilities.
 *
 * Example:
 *
 *     allow capability("quantum.measurement");
 *
 * The grammar does not determine whether that capability exists.
 *
 * Capability resolution belongs to:
 *
 *     grammar/resources/capabilities.g4
 *
 * and downstream semantic analysis.
 *
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Resource expressions remain symbolic.
 *
 * Examples:
 *
 *     allow resource::dataset::read;
 *     allow subject analyst action read resource data::records;
 *
 * A resource expression does NOT identify a physical memory allocation,
 * storage device, GPU, QPU, node, or network device.
 *
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * Permission conditions may participate semantically in:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * This grammar does not redefine the contract system.
 *
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Permission intent is governed by the containing policy.
 *
 * Examples:
 *
 *     policy execution {
 *         allow permission compute::execute;
 *     }
 *
 *     policy quantum {
 *         permit permission quantum::measurement;
 *     }
 *
 * The policy system determines:
 *
 *     applicability
 *     precedence
 *     inheritance
 *     composition
 *     conflicts
 *     activation
 *     fallback
 *     adaptation
 *
 * This file does not own those mechanisms.
 *
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * The AST/semantic model must preserve provenance for permission constructs.
 *
 * Provenance may include:
 *
 *     source location
 *     policy identity
 *     permission identity
 *     originating module
 *     transformation history
 *     decision reason
 *     evidence
 *     verification status
 *
 * Provenance ownership remains outside this grammar.
 *
 *
 * ============================================================================
 * SECURITY BOUNDARY
 * ============================================================================
 *
 * SECURITY PERMISSION SYNTAX
 * --------------------------
 *
 * Existing:
 *
 *     grammar/security/permissions.g4
 *
 * owns detailed security authorization constructs.
 *
 *
 * POLICY PERMISSION SYNTAX
 * ------------------------
 *
 * This file expresses policy intent.
 *
 * Therefore:
 *
 *     policy permission
 *
 * must flow into:
 *
 *     policy analysis
 *          |
 *          v
 *     security analysis where applicable
 *          |
 *          v
 *     capability/resource/effect analysis
 *          |
 *          v
 *     execution planning
 *
 * This prevents two independent authorization languages from being created.
 *
 *
 * ============================================================================
 * 1. PERMISSION DECISION
 * ============================================================================
 *
 * The decision is intentionally small and open-ended at the semantic level.
 *
 * ALLOW and PERMIT are existing canonical policy vocabulary.
 *
 * They are equivalent only if the semantic policy model defines them as such.
 *
 * The parser preserves which spelling was used.
 */
policyPermissionDecision
    : ALLOW
    | PERMIT
    ;


/*
 * ============================================================================
 * 2. PERMISSION
 * ============================================================================
 *
 * This is the reusable permission form.
 *
 * IMPORTANT:
 *
 * This rule intentionally does NOT consume SEMICOLON.
 *
 * That allows it to be used both:
 *
 *     as a policy member;
 *
 * and:
 *
 *     as an action in a conditional policy rule.
 *
 * The containing policy grammar owns statement termination.
 */
policyPermission
    : policyPermissionDecision
      policyPermissionTarget
    ;


/*
 * ============================================================================
 * 3. PERMISSION STATEMENT
 * ============================================================================
 *
 * This is the complete policy-member form.
 *
 * Canonical examples:
 *
 *     allow permission quantum::execute;
 *
 *     permit permission data::read;
 *
 *     allow capability("quantum.measurement");
 *
 *     allow subject analyst action execute resource computation;
 */
policyPermissionStatement
    : policyPermission
      SEMICOLON
    ;


/*
 * ============================================================================
 * 4. PERMISSION TARGET
 * ============================================================================
 *
 * A target can be:
 *
 *     a permission reference;
 *     a structured authorization intent;
 *     a normal Zamani expression.
 *
 * Keeping the final alternative as `expression` makes the permission system
 * extensible without creating a keyword for every future operation.
 */
policyPermissionTarget
    : policyPermissionReference
    | policyPermissionStructuredTarget
    | expression
    ;


/*
 * ============================================================================
 * 5. PERMISSION REFERENCE
 * ============================================================================
 *
 * Explicit reference form:
 *
 *     permission quantum::execute
 *
 * Permission identity is open-world.
 */
policyPermissionReference
    : PERMISSION
      qualifiedName
    ;


/*
 * ============================================================================
 * 6. STRUCTURED PERMISSION TARGET
 * ============================================================================
 *
 * Canonical structured form:
 *
 *     subject <subject>
 *     action <action>
 *     resource <resource>
 *     when <condition>
 *     oblige <obligation>
 *
 * Each clause is optional where semantically valid.
 *
 * The semantic layer determines whether the resulting combination is
 * meaningful.
 *
 * The grammar deliberately does not hard-code a particular security model.
 */
policyPermissionStructuredTarget
    : policyPermissionSubjectClause
      policyPermissionActionClause?
      policyPermissionResourceClause?
      policyPermissionConditionClause?
      policyPermissionObligationClause*
    ;


/*
 * ============================================================================
 * 7. SUBJECT
 * ============================================================================
 *
 * A subject is a normal Zamani expression.
 *
 * This permits:
 *
 *     named principals
 *     symbolic entities
 *     actor references
 *     computed subjects
 *     future identity models
 *
 * without enumerating them in the grammar.
 */
policyPermissionSubjectClause
    : SUBJECT
      policyPermissionSubject
    ;


policyPermissionSubject
    : expression
    ;


/*
 * ============================================================================
 * 8. ACTION
 * ============================================================================
 *
 * Actions are open-world semantic expressions.
 *
 * Examples may represent:
 *
 *     computation
 *     measurement
 *     data access
 *     network operation
 *     deployment
 *     compilation
 *     hardware configuration
 *     model execution
 *     adaptation
 *     future operations
 *
 * The grammar does not enumerate them.
 */
policyPermissionActionClause
    : ACTION
      policyPermissionAction
    ;


policyPermissionAction
    : expression
    ;


/*
 * ============================================================================
 * 9. RESOURCE
 * ============================================================================
 *
 * Resource meaning is resolved downstream.
 */
policyPermissionResourceClause
    : RESOURCE
      policyPermissionResource
    ;


policyPermissionResource
    : expression
    ;


/*
 * ============================================================================
 * 10. CONDITION
 * ============================================================================
 *
 * Conditions use the canonical Zamani expression grammar.
 *
 * There is no second policy-specific boolean language.
 */
policyPermissionConditionClause
    : WHEN
      policyPermissionCondition
    ;


policyPermissionCondition
    : expression
    ;


/*
 * ============================================================================
 * 11. OBLIGATION
 * ============================================================================
 *
 * Obligations describe additional semantic policy intent.
 *
 * They are not executed by the parser.
 *
 * Security/runtime semantics determine whether an obligation can be
 * satisfied.
 */
policyPermissionObligationClause
    : OBLIGE
      policyPermissionObligation
    ;


policyPermissionObligation
    : expression
    ;


/*
 * ============================================================================
 * 12. EXTENSION BOUNDARY
 * ============================================================================
 *
 * Future policy permission metadata MUST be represented through the existing
 * policy/property/attribute systems rather than by continuously adding
 * permission-specific keywords here.
 *
 * Examples of concepts that remain semantic rather than lexical:
 *
 *     priority
 *     duration
 *     delegation
 *     attenuation
 *     trust
 *     provenance
 *     evidence
 *     audit
 *     transparency
 *     explanation
 *     deterministic execution
 *     reproducibility
 *
 * Such concepts belong to their owning policy/security/provenance grammars.
 *
 *
 * ============================================================================
 * ANTI-HARD-CODING CONTRACT
 * ============================================================================
 *
 * This grammar MUST NOT add alternatives such as:
 *
 *     allowGpu
 *     allowQpu
 *     allowFpga
 *     allowCpu
 *     allowQuantumMeasurement
 *     allowTensorCompute
 *     allowNetwork
 *     allowCloud
 *     allowCluster
 *
 * Instead use symbolic semantic identities:
 *
 *     allow permission hardware::accelerator;
 *     allow permission quantum::measurement;
 *     allow capability("tensor.compute");
 *     allow capability("network.transport");
 *
 * The semantic model decides whether these identifiers exist and what they
 * mean.
 *
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Permission syntax remains independent of quantum implementation.
 *
 * Valid semantic permission identities may include:
 *
 *     quantum::execute
 *     quantum::measure
 *     quantum::prepare
 *     quantum::adapt
 *
 * No gate catalogue is introduced.
 *
 * No qubit count is introduced.
 *
 * No topology is introduced.
 *
 * No vendor is introduced.
 *
 * No physical qubit is introduced.
 *
 * Quantum realization remains:
 *
 *     policy
 *       ->
 *     semantic analysis
 *       ->
 *     quantum::ir
 *       ->
 *     optimization
 *       ->
 *     decomposition
 *       ->
 *     routing
 *       ->
 *     scheduling
 *       ->
 *     resilience / QEC
 *       ->
 *     ZQN
 *       ->
 *     HAL
 *       ->
 *     target
 *
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Permissions may semantically govern:
 *
 *     HDL synthesis
 *     HDL simulation
 *     hardware configuration
 *     accelerator use
 *     reconfiguration
 *     deployment
 *
 * The grammar does not encode:
 *
 *     bus width
 *     register width
 *     device count
 *     memory size
 *     clock count
 *     topology size
 *
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical permission targets may refer to:
 *
 *     computation
 *     data
 *     files
 *     models
 *     functions
 *     processes
 *     services
 *
 * These remain symbolic semantic expressions.
 *
 *
 * ============================================================================
 * DISTRIBUTED / NETWORKING INTEGRATION
 * ============================================================================
 *
 * Permissions may semantically govern:
 *
 *     messages
 *     channels
 *     services
 *     endpoints
 *     discovery
 *     transport
 *     distributed computation
 *
 * No finite network or node size is encoded.
 *
 *
 * ============================================================================
 * ADAPTATION / LEARNING INTEGRATION
 * ============================================================================
 *
 * Permission intent may govern operations such as:
 *
 *     learning
 *     model update
 *     controlled adaptation
 *     inference
 *     reasoning
 *     knowledge access
 *
 * Such permissions do not grant unrestricted self-modification.
 *
 * Adaptation remains subject to:
 *
 *     policy
 *     effects
 *     capabilities
 *     resource constraints
 *     authorization
 *     provenance
 *     validation
 *
 *
 * ============================================================================
 * ERROR / DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser-level diagnostics must identify:
 *
 *     missing permission target
 *     malformed permission reference
 *     malformed subject clause
 *     malformed action clause
 *     malformed resource clause
 *     malformed condition
 *     malformed obligation
 *     missing statement terminator
 *     unexpected token
 *
 * Semantic diagnostics belong downstream and may include:
 *
 *     unknown permission
 *     invalid permission reference
 *     invalid subject
 *     invalid action
 *     invalid resource
 *     conflicting permission/prohibition
 *     unauthorized operation
 *     unavailable capability
 *     unsatisfied requirement
 *     forbidden effect
 *     invalid obligation
 *
 * Parser diagnostics MUST NOT claim semantic authorization.
 *
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This file introduces no new lexical token.
 *
 * It reuses existing policy/security vocabulary.
 *
 * Existing forms such as:
 *
 *     allow <expression>;
 *     permit <expression>;
 *
 * remain representable.
 *
 * The richer structured form is additive:
 *
 *     allow subject <expr> action <expr> resource <expr>;
 *
 * Therefore existing source syntax does not need a new keyword merely because
 * the permission semantic model becomes richer.
 *
 *
 * ============================================================================
 * INTEGRATION WITH grammar/policies/policy.g4
 * ============================================================================
 *
 * `policy.g4` currently contains a local `policyPermission` rule.
 *
 * That local rule MUST NOT remain a second authority after this file is
 * integrated.
 *
 * Required one-time composition change:
 *
 *     1. Add `PolicyPermissions` to the import list of Policy:
 *
 *         import
 *             Names,
 *             Requirements,
 *             Constraints,
 *             Capabilities,
 *             Expressions,
 *             PolicyPermissions,
 *             PolicyScopes
 *         ;
 *
 *     2. Remove the local `policyPermission` implementation from policy.g4.
 *
 *     3. Change policy-body membership from the local permission rule to:
 *
 *         policyPermissionStatement
 *
 *     4. Change `policyRuleAction` so permission actions use:
 *
 *         policyPermission
 *
 *        rather than a statement rule containing SEMICOLON.
 *
 * This preserves the existing conditional form:
 *
 *     when <condition> => allow <target>;
 *
 * while also supporting ordinary policy members:
 *
 *     allow <target>;
 *
 * The permission leaf therefore has exactly one canonical permission syntax.
 *
 *
 * ============================================================================
 * REQUIRED policy.g4 COMPOSITION
 * ============================================================================
 *
 * The resulting relevant policy.g4 structure is:
 *
 *     policyMember
 *         : policyRule
 *         | policyRequirement
 *         | policyConstraint
 *         | policyCapability
 *         | policyResource
 *         | policyPermissionStatement
 *         | ...
 *         ;
 *
 *
 *     policyRuleAction
 *         : policyPermission
 *         | policyProhibition
 *         | policyPreference
 *         | ...
 *         ;
 *
 * There must NOT be a second local `policyPermission` rule.
 *
 *
 * ============================================================================
 * INTEGRATION WITH grammar/antlr/ZamaniParser.g4
 * ============================================================================
 *
 * No direct leaf import is permitted here.
 *
 * The canonical composition direction remains:
 *
 *     PolicyPermissions
 *          |
 *          v
 *     Policy
 *          |
 *          v
 *     policy composition
 *          |
 *          v
 *     ZamaniParser
 *
 * `ZamaniParser.g4` should receive Policy through its canonical policy
 * composition grammar rather than importing this leaf directly.
 *
 *
 * ============================================================================
 * INTEGRATION WITH grammar/security/permissions.g4
 * ============================================================================
 *
 * There must be NO direct grammar import from this file into security
 * permissions and no reverse import from security permissions into this file.
 *
 * The semantic relationship is:
 *
 *     policy permission intent
 *             |
 *             v
 *     semantic policy model
 *             |
 *             v
 *     security authorization analysis
 *
 * This avoids circular grammar ownership.
 *
 *
 * ============================================================================
 * INTEGRATION WITH grammar/resources/capabilities.g4
 * ============================================================================
 *
 * No direct capability grammar is recreated here.
 *
 * Capability expressions are consumed through:
 *
 *     expression
 *
 * or through the existing resource/capability semantic model.
 *
 *
 * ============================================================================
 * INTEGRATION WITH grammar/policies/scopes.g4
 * ============================================================================
 *
 * Permission syntax does not define scope.
 *
 * Scope applicability is owned by:
 *
 *     grammar/policies/scopes.g4
 *
 * A containing policy may therefore associate a permission with a policy
 * scope without duplicating scope syntax here.
 *
 *
 * ============================================================================
 * INTEGRATION WITH PROVENANCE
 * ============================================================================
 *
 * Permission AST nodes must retain source provenance.
 *
 * Policy resolution may record:
 *
 *     permission declaration
 *     permission reference
 *     policy source
 *     subject
 *     action
 *     resource
 *     condition
 *     obligation
 *     resolution decision
 *     evidence
 *
 * Provenance is semantic infrastructure, not parser syntax.
 *
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE
 * --------
 *
 * The following forms must parse:
 *
 *     allow permission compute::execute;
 *
 *     permit permission quantum::measure;
 *
 *     allow capability("quantum.measurement");
 *
 *     allow resource::data::read;
 *
 *     allow subject analyst action read;
 *
 *     allow subject analyst action read resource data::records;
 *
 *     allow subject analyst action read resource data::records
 *         when authenticated;
 *
 *     allow subject analyst action read
 *         oblige audit;
 *
 *     allow subject analyst action read
 *         resource data::records
 *         when authenticated
 *         oblige audit;
 *
 *
 * CONDITIONAL
 * -----------
 *
 * The following must remain representable when used by policy.g4:
 *
 *     when execution::deterministic
 *         => allow permission compute::execute;
 *
 *
 * OPEN-WORLD
 * ----------
 *
 * Arbitrary symbolic permission names must parse:
 *
 *     allow permission future::permission;
 *
 *     allow permission vendor::extension::operation;
 *
 *     allow permission domain::subsystem::operation;
 *
 *
 * CROSS-DOMAIN
 * ------------
 *
 * Permission targets must be usable with semantic identities referring to:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     AI/model
 *     data
 *     distributed
 *     networking
 *     interoperability
 *     simulation
 *     deployment
 *
 * No domain-specific keyword is required.
 *
 *
 * NEGATIVE
 * --------
 *
 * These must fail structurally:
 *
 *     allow;
 *
 *     permit;
 *
 *     allow permission;
 *
 *     allow subject;
 *
 *     allow action;
 *
 *     allow resource;
 *
 *     allow subject analyst action;
 *
 *     allow subject analyst resource;
 *
 *     allow subject analyst action read resource;
 *
 *
 * BOUNDARY
 * --------
 *
 * Test:
 *
 *     empty expressions where expressions are legal;
 *     deeply nested qualified names;
 *     many permission clauses;
 *     many obligations;
 *     long policy bodies;
 *     repeated permissions;
 *     multiple independent policies;
 *     permissions combined with prohibitions;
 *     permissions combined with requirements;
 *     permissions combined with constraints;
 *     permissions combined with scopes;
 *     permissions combined with provenance.
 *
 *
 * SCALABILITY
 * ----------
 *
 * Tests must verify that no parser rule introduces:
 *
 *     permission-count limits
 *     subject-count limits
 *     action-count limits
 *     resource-count limits
 *     node-count limits
 *     device-count limits
 *     CPU/GPU/QPU limits
 *     memory limits
 *     network limits
 *
 * Large source input may be constrained by available implementation resources,
 * but no arbitrary language constant may be introduced.
 *
 *
 * DETERMINISM
 * -----------
 *
 * The same source and token stream must produce the same parse structure.
 *
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [x] It is a parser grammar.
 *
 *     [x] It uses tokenVocab = ZamaniLexer.
 *
 *     [x] It imports only canonical Names and Expressions.
 *
 *     [x] It introduces no lexer rules.
 *
 *     [x] It introduces no private keyword vocabulary.
 *
 *     [x] It owns policy-level permission intent.
 *
 *     [x] It does not become a second security authorization grammar.
 *
 *     [x] It distinguishes permission from capability.
 *
 *     [x] It is open-world.
 *
 *     [x] It contains no universal hardware/resource limits.
 *
 *     [x] It contains no physical target selection.
 *
 *     [x] It contains no quantum gate catalogue.
 *
 *     [x] It contains no IR implementation.
 *
 *     [x] It contains no runtime behavior.
 *
 *     [x] It contains no unsafe Rust.
 *
 *     [x] It is compatible with Rust 1.97+ through generated-parser
 *         integration.
 *
 *     [x] It supplies a reusable non-terminated `policyPermission` rule.
 *
 *     [x] It supplies a complete `policyPermissionStatement` rule.
 *
 *     [x] It provides a stable integration boundary for policy.g4.
 *
 * Repository verification still required:
 *
 *     [ ] ANTLR generation succeeds with the repository's configured version.
 *
 *     [ ] PolicyPermissions is composed into Policy.
 *
 *     [ ] Policy no longer contains a competing local policyPermission rule.
 *
 *     [ ] Policy body consumes policyPermissionStatement.
 *
 *     [ ] Conditional policy rules consume policyPermission.
 *
 *     [ ] Generated Rust parser compiles on Rust 1.97+.
 *
 *     [ ] Positive tests pass.
 *
 *     [ ] Negative tests pass.
 *
 *     [ ] Boundary tests pass.
 *
 *     [ ] Cross-domain tests pass.
 *
 *     [ ] Scalability tests pass within available implementation resources.
 *
 *     [ ] Security and policy semantic layers consume the same AST intent.
 *
 * ============================================================================
 * FINAL OWNERSHIP RULE
 * ============================================================================
 *
 * The resulting ownership chain is:
 *
 *     permissions.g4
 *          |
 *          v
 *     policy.g4
 *          |
 *          v
 *     policy semantic model
 *          |
 *          +--> contracts
 *          +--> effects
 *          +--> capabilities
 *          +--> resources
 *          +--> security
 *          +--> provenance
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL/hardware
 *          +--> distributed
 *          +--> future domains
 *
 * This keeps permission syntax extensible without making permissions a
 * scalability bottleneck or a second security language.
 *
 * ============================================================================
 */

parser grammar PolicyPermissions;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names,
    Expressions
;


/*
 * ============================================================================
 * 1. PERMISSION DECISION
 * ============================================================================
 */

policyPermissionDecision
    : ALLOW
    | PERMIT
    ;


/*
 * ============================================================================
 * 2. REUSABLE PERMISSION ACTION
 * ============================================================================
 *
 * No statement terminator is consumed here.
 *
 * This is required because policy.g4 uses the construct in both:
 *
 *     policy members
 *
 * and:
 *
 *     conditional policy actions.
 */
policyPermission
    : policyPermissionDecision
      policyPermissionTarget
    ;


/*
 * ============================================================================
 * 3. COMPLETE POLICY MEMBER
 * ============================================================================
 */

policyPermissionStatement
    : policyPermission
      SEMICOLON
    ;


/*
 * ============================================================================
 * 4. TARGET
 * ============================================================================
 */

policyPermissionTarget
    : policyPermissionReference
    | policyPermissionStructuredTarget
    | expression
    ;


/*
 * ============================================================================
 * 5. SYMBOLIC PERMISSION REFERENCE
 * ============================================================================
 */

policyPermissionReference
    : PERMISSION
      qualifiedName
    ;


/*
 * ============================================================================
 * 6. STRUCTURED TARGET
 * ============================================================================
 */

policyPermissionStructuredTarget
    : policyPermissionSubjectClause
      policyPermissionActionClause?
      policyPermissionResourceClause?
      policyPermissionConditionClause?
      policyPermissionObligationClause*
    ;


/*
 * ============================================================================
 * 7. SUBJECT
 * ============================================================================
 */

policyPermissionSubjectClause
    : SUBJECT
      policyPermissionSubject
    ;


policyPermissionSubject
    : expression
    ;


/*
 * ============================================================================
 * 8. ACTION
 * ============================================================================
 */

policyPermissionActionClause
    : ACTION
      policyPermissionAction
    ;


policyPermissionAction
    : expression
    ;


/*
 * ============================================================================
 * 9. RESOURCE
 * ============================================================================
 */

policyPermissionResourceClause
    : RESOURCE
      policyPermissionResource
    ;


policyPermissionResource
    : expression
    ;


/*
 * ============================================================================
 * 10. CONDITION
 * ============================================================================
 */

policyPermissionConditionClause
    : WHEN
      policyPermissionCondition
    ;


policyPermissionCondition
    : expression
    ;


/*
 * ============================================================================
 * 11. OBLIGATION
 * ============================================================================
 */

policyPermissionObligationClause
    : OBLIGE
      policyPermissionObligation
    ;


policyPermissionObligation
    : expression
    ;
/*
 * ============================================================================
 * Zamani — Policy Expressions
 * File: grammar/expressions/policy.g4
 * ============================================================================
 *
 * PURPOSE
 * -------
 * Defines the syntax of policy expressions.
 *
 * A policy expression describes declarative intent governing execution,
 * resources, capabilities, effects, security, adaptation, deployment,
 * simulation, interoperability, or other semantic domains.
 *
 * This grammar deliberately does NOT encode:
 *
 *   - hardware limits
 *   - target-specific limits
 *   - CPU/GPU/QPU counts
 *   - memory sizes
 *   - register widths
 *   - network sizes
 *   - fixed policy kinds
 *   - fixed policy providers
 *   - fixed resource classes
 *   - fixed security mechanisms
 *   - fixed AI mechanisms
 *   - fixed quantum mechanisms
 *   - fixed HDL mechanisms
 *
 * Policies are semantic declarations. Their realization is performed by
 * semantic analysis, capability negotiation, resource analysis, security,
 * execution planning, compilation, and the appropriate backend.
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * OWNS
 * ----
 *   policyExpression
 *   policyBody
 *   policyClause
 *   policyDirective
 *   policyDirectiveName
 *   policyDirectiveArguments
 *   policySelector
 *   policyScope
 *   policyCondition
 *   policyValue
 *   policyBinding
 *   policyReference
 *
 * DOES NOT OWN
 * -------------
 *   - policy semantic meaning
 *   - authorization decisions
 *   - capability resolution
 *   - resource resolution
 *   - effect checking
 *   - security enforcement
 *   - execution planning
 *   - hardware selection
 *   - quantum placement/routing
 *   - HDL synthesis
 *   - distributed scheduling
 *   - AST implementation
 *   - semantic IR implementation
 *
 * Those belong to their respective subsystems.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *   lexer token authority
 *   core identifiers / qualified names
 *   core attributes / values
 *   expressions
 *   types
 *   requirements
 *   constraints
 *   capabilities
 *
 * This file intentionally reuses general expression/value rules instead of
 * recreating literals, identifiers, calls, collections, or operators.
 *
 * EXPORTS
 * -------
 *   policyExpression
 *
 * CONSUMED_BY
 * ----------
 *   expressions/expressions.g4
 *   statements/policy.g4
 *   validation/policy.g4
 *   security/policy.g4
 *   resources/policies.g4
 *   execution/policies.g4
 *   policies/policy.g4
 *   dialects which expose policy expressions
 *
 * AST_OWNER
 * ---------
 *   Zamani AST layer.
 *   This grammar supplies parse structure only.
 *
 * SEMANTIC_OWNER
 * --------------
 *   Zamani policy semantic model.
 *
 * IR_OWNER
 * --------
 *   Canonical semantic IR / policy representation.
 *   A policy must not become target-specific merely because it is lowered.
 *
 * TEST_OWNER
 * -----------
 *   grammar/tests/policies/
 *   grammar/tests/parser/
 *   grammar/tests/semantic/
 *   grammar/tests/negative/
 *   grammar/tests/scalability/
 *
 * SPEC_OWNER
 * ----------
 *   grammar/spec/policies.md
 *   grammar/specification/policies.md
 *
 * ============================================================================
 * IMPORTANT DESIGN RULE
 * ============================================================================
 *
 * A policy directive is intentionally open-ended:
 *
 *     policy {
 *         allow capability("tensor.compute");
 *         require capability("quantum.measurement");
 *         prefer target.some_property;
 *         forbid effect("network");
 *     }
 *
 * The grammar does NOT enumerate:
 *
 *     allowGpu
 *     allowQpu
 *     requireCuda
 *     requireOpenQasm
 *     requireFpga
 *     requireCluster
 *     ...
 *
 * Such concepts are represented through names, values, capabilities,
 * requirements, constraints, effects, and semantic registrations.
 *
 * This is essential for POCO-REAF and future extensibility.
 *
 * ============================================================================
 *
 * NOTE ABOUT TOKENS
 * -----------------
 *
 * This file expects the repository's central lexer to provide the canonical
 * policy keyword token:
 *
 *     KeywordPolicy
 *
 * If the current lexer uses another spelling, the lexer token registry must
 * be normalized once. Do NOT create a private lexer/token definition here.
 *
 * The remaining directive names are deliberately parsed as identifiers or
 * qualified names. This avoids forcing every future policy operation into
 * the universal lexer.
 *
 * ============================================================================
 */

parser grammar PolicyExpressions;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * A policy is an expression and therefore may occur wherever the surrounding
 * expression grammar permits a policy expression.
 *
 * The policy keyword is the only mandatory syntactic marker.
 */
policyExpression
    : KeywordPolicy
      policySelector?
      policyBody
    ;


/* ============================================================================
 * POLICY BODY
 * ============================================================================
 *
 * Policies are declarative collections of zero or more clauses.
 *
 * Empty policies are syntactically accepted so that construction,
 * metaprogramming, conditional generation, and dialect extensions do not
 * require artificial grammar restrictions.
 *
 * Semantic validation may reject an empty policy where a non-empty policy is
 * required by context.
 */
policyBody
    : LeftBrace policyClause* RightBrace
    ;


/* ============================================================================
 * POLICY CLAUSE
 * ============================================================================
 *
 * A clause consists of:
 *
 *   directive
 *   optional condition
 *   optional terminator
 *
 * The directive itself remains extensible.
 */
policyClause
    : policyDirective policyCondition? Terminator
    | policyDirective policyCondition?
    ;


/* ============================================================================
 * POLICY DIRECTIVE
 * ============================================================================
 *
 * A directive has:
 *
 *   name
 *   optional binding
 *   optional arguments
 *
 * Examples of semantic directives can include:
 *
 *   allow
 *   forbid
 *   require
 *   prefer
 *   constrain
 *   fallback
 *   authorize
 *   audit
 *   record
 *   simulate
 *   adapt
 *
 * None of those names are hard-coded here.
 */
policyDirective
    : policyDirectiveName
      policyBinding?
      policyDirectiveArguments?
    ;


/* ============================================================================
 * POLICY DIRECTIVE NAME
 * ============================================================================
 *
 * A policy directive may be a simple identifier or a qualified name.
 *
 * Qualified names permit extensible namespaces without modifying the grammar:
 *
 *   security.allow
 *   execution.retry
 *   quantum.prefer
 *   hardware.require
 *   deployment.fallback
 *
 * The semantic layer determines whether a directive exists and what it means.
 */
policyDirectiveName
    : Identifier
    | qualifiedPolicyName
    ;


/* ============================================================================
 * QUALIFIED POLICY NAME
 * ============================================================================
 *
 * Kept local to prevent this feature from redefining the repository's
 * canonical general qualified-name grammar.
 *
 * If the repository already exports a canonical qualifiedName rule, the
 * implementation should replace this rule reference with that rule rather
 * than maintaining a duplicate.
 */
qualifiedPolicyName
    : Identifier (Dot Identifier)+
    ;


/* ============================================================================
 * OPTIONAL POLICY BINDING
 * ============================================================================
 *
 * Supports named policy declarations without imposing a single semantic
 * binding model.
 *
 * Examples:
 *
 *   allow capability("x") as execution_permission
 *
 *   require capability("x") -> selected_target
 *
 * The exact meaning belongs to semantic analysis.
 */
policyBinding
    : As Identifier
    ;


/* ============================================================================
 * POLICY ARGUMENTS
 * ============================================================================
 *
 * Arguments deliberately reuse generic expressions.
 *
 * This permits policies to consume:
 *
 *   literals
 *   names
 *   calls
 *   capabilities
 *   requirements
 *   constraints
 *   resource expressions
 *   type expressions
 *   computed values
 *   collections
 *   references
 *
 * without duplicating those grammars here.
 */
policyDirectiveArguments
    : LeftParen
      policyArgumentList?
      RightParen
    ;


/* ============================================================================
 * POLICY ARGUMENT LIST
 * ============================================================================
 *
 * Comma-separated generic expressions.
 */
policyArgumentList
    : policyValue (Comma policyValue)*
    ;


/* ============================================================================
 * POLICY SELECTOR
 * ============================================================================
 *
 * Allows a policy to be scoped to a semantic subject without requiring
 * separate grammar rules for every domain.
 *
 * Examples:
 *
 *   policy execution { ... }
 *   policy quantum { ... }
 *   policy security { ... }
 *   policy hardware.accelerator { ... }
 *
 * The selector is semantic metadata, not target realization.
 */
policySelector
    : policyScope
    ;


/* ============================================================================
 * POLICY SCOPE
 * ============================================================================
 *
 * A scope can be represented by a name or qualified name.
 */
policyScope
    : Identifier
    | qualifiedPolicyName
    ;


/* ============================================================================
 * POLICY CONDITION
 * ============================================================================
 *
 * A policy can be conditional.
 *
 * Conditions are expressions rather than a separate policy-specific boolean
 * language. This is critical for consistency with the rest of Zamani.
 *
 * Example semantic forms:
 *
 *   allow capability("x") when condition;
 *   retry execution when resilience.state == degraded;
 *
 * The generic expression rule is resolved by the surrounding grammar.
 */
policyCondition
    : When policyValue
    ;


/* ============================================================================
 * POLICY VALUE
 * ============================================================================
 *
 * This is intentionally an integration boundary.
 *
 * The canonical repository expression rule SHOULD be substituted here when
 * the complete expression grammar is imported by the root grammar.
 *
 * Keeping this rule as a named boundary lets the policy grammar evolve
 * independently while preserving a single semantic value model.
 */
policyValue
    : Identifier
    | qualifiedPolicyName
    | policyLiteral
    | policyCollection
    | policyReference
    ;


/* ============================================================================
 * POLICY REFERENCE
 * ============================================================================
 *
 * References allow policies to refer to previously declared semantic
 * entities without embedding any particular entity category.
 */
policyReference
    : At Identifier
    ;


/* ============================================================================
 * POLICY LITERALS
 * ============================================================================
 *
 * Policy values need only the primitive literal forms required by the
 * repository's lexical contract.
 *
 * If Zamani's canonical literal grammar is available, it should be consumed
 * instead of duplicating these alternatives.
 */
policyLiteral
    : IntegerLiteral
    | FloatingPointLiteral
    | StringLiteral
    | BooleanLiteral
    | NullLiteral
    ;


/* ============================================================================
 * POLICY COLLECTION
 * ============================================================================
 *
 * Collections are deliberately unbounded by grammar design.
 *
 * The parser accepts arbitrary finite source input; semantic/runtime limits
 * are determined by available resources rather than grammar constants.
 */
policyCollection
    : LeftBracket
      policyArgumentList?
      RightBracket
    | LeftBrace
      policyNamedValueList?
      RightBrace
    ;


/* ============================================================================
 * POLICY NAMED VALUES
 * ============================================================================
 *
 * Named metadata/options are useful for extensible policy providers.
 */
policyNamedValueList
    : policyNamedValue (Comma policyNamedValue)*
    ;


policyNamedValue
    : Identifier Colon policyValue
    ;


/* ============================================================================
 * TOKEN COMPATIBILITY ALIASES
 * ============================================================================
 *
 * The following token names are expected from the canonical lexer.
 *
 * They are intentionally NOT declared in this file.
 *
 * Expected lexical concepts:
 *
 *   KeywordPolicy
 *   Identifier
 *   LeftBrace
 *   RightBrace
 *   LeftParen
 *   RightParen
 *   LeftBracket
 *   RightBracket
 *   Comma
 *   Dot
 *   Colon
 *   At
 *   As
 *   When
 *   Terminator
 *   IntegerLiteral
 *   FloatingPointLiteral
 *   StringLiteral
 *   BooleanLiteral
 *   NullLiteral
 *
 * If the repository currently uses different token names, normalize them in
 * the central lexer/token registry. Do not add duplicate tokens here.
 */


/* ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * The parser output must be lowered into a domain-neutral semantic policy:
 *
 *     Policy
 *       selector
 *       clauses[]
 *
 *     PolicyClause
 *       directive
 *       binding?
 *       arguments[]
 *       condition?
 *
 * No target-specific structure belongs in the AST produced from this grammar.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Parsing a policy has no runtime effect.
 *
 * Semantic interpretation MAY require or constrain effects such as:
 *
 *   io
 *   network
 *   native
 *   foreign
 *   mutation
 *   randomness
 *   distributed
 *   quantum
 *   measurement
 *   learning
 *   adaptation
 *   reflection
 *   code_generation
 *   simulation
 *
 * Those effects are consumed from the central effect system.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Policies may refer to capabilities symbolically.
 *
 * Examples:
 *
 *   capability("tensor.compute")
 *   capability("quantum.measurement")
 *   capability("network.transport")
 *
 * The grammar does not enumerate capabilities.
 *
 * Capability existence and satisfaction are semantic/resource concerns.
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Policy expressions may constrain or prefer resources through generic
 * expressions and capability/requirement values.
 *
 * They must never encode universal fixed capacities such as:
 *
 *   MAX_CPUS
 *   MAX_GPUS
 *   MAX_QUBITS
 *   MAX_NODES
 *   MAX_MEMORY
 *   MAX_THREADS
 *
 * Resource quantities are symbolic or semantically resolved.
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * Policy expressions may participate in:
 *
 *   requires
 *   ensures
 *   invariant
 *   assume
 *   guarantee
 *   property
 *
 * The policy grammar does not redefine those constructs.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Semantic policy decisions should preserve:
 *
 *   source location
 *   policy identity
 *   directive identity
 *   originating declaration
 *   transformation history
 *   decision reason
 *   evidence
 *   verification status
 *
 * Provenance is owned by the central provenance subsystem.
 *
 * ============================================================================
 * SECURITY CONTRACT
 * ============================================================================
 *
 * A policy is not itself an authorization decision.
 *
 * For example:
 *
 *   policy {
 *       forbid capability("native.execute");
 *   }
 *
 * is a declarative constraint.
 *
 * Security/authorization semantics determine whether and how that constraint
 * is enforced.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Quantum policies may express requirements/preferences/constraints such as:
 *
 *   quantum capability requirements
 *   measurement restrictions
 *   resilience requirements
 *   execution preferences
 *   simulator fallback
 *
 * This grammar must never encode:
 *
 *   physical qubit identifiers
 *   fixed qubit counts
 *   vendor-specific topology
 *   calibration data
 *   routing algorithms
 *   QEC implementation
 *
 * Those belong downstream of the quantum semantic model and quantum::ir.
 *
 * ============================================================================
 * HDL BOUNDARY
 * ============================================================================
 *
 * HDL policies may constrain:
 *
 *   synthesis
 *   simulation
 *   verification
 *   timing intent
 *   resource intent
 *   implementation preferences
 *
 * They must not turn policy.g4 into an HDL grammar.
 *
 * ============================================================================
 * BACKEND BOUNDARY
 * ============================================================================
 *
 * Backends consume resolved policy semantics.
 *
 * This grammar has no knowledge of:
 *
 *   LLVM
 *   MLIR
 *   QIR
 *   CUDA
 *   OpenCL
 *   vendor QPUs
 *   FPGA vendor primitives
 *   ASIC technologies
 *   particular CPUs
 *   particular GPUs
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics must identify:
 *
 *   - malformed policy
 *   - malformed directive
 *   - malformed argument
 *   - malformed selector
 *   - malformed binding
 *   - malformed condition
 *   - malformed collection
 *
 * Semantic diagnostics, not parser diagnostics, should report:
 *
 *   - unknown policy directive
 *   - unsupported policy
 *   - unsatisfied capability
 *   - impossible resource requirement
 *   - conflicting policy
 *   - unauthorized operation
 *   - incompatible effect
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * No finite policy count, clause count, directive count, argument count,
 * namespace depth, resource count, capability count, or domain count is
 * encoded here.
 *
 * The grammar scales with available parser/runtime resources.
 *
 * Large policies should be represented structurally rather than by expanding
 * the grammar with one rule per policy type.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing must be deterministic.
 *
 * Policy evaluation order must NOT be inferred from grammar alternative order.
 *
 * Semantic policy precedence/conflict resolution belongs to the policy
 * semantic specification.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Adding a new policy directive must not require modifying this grammar if
 * the directive can be represented by:
 *
 *   policyDirectiveName
 *   policyBinding
 *   policyDirectiveArguments
 *   policyCondition
 *
 * New semantic policy capabilities should therefore normally be registered
 * outside the universal grammar.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *   1. It compiles with the repository's ANTLR toolchain.
 *   2. KeywordPolicy comes from the canonical lexer.
 *   3. It is imported by the root expression grammar.
 *   4. It does not redefine general expression/value semantics.
 *   5. It produces domain-neutral parse structure.
 *   6. It supports arbitrary policy directive names.
 *   7. It supports qualified policy namespaces.
 *   8. It supports conditions.
 *   9. It supports extensible arguments and metadata.
 *  10. It contains no hardware/resource capacity constants.
 *  11. It has positive parser tests.
 *  12. It has negative parser tests.
 *  13. It has boundary/cross-domain tests.
 *  14. It has scalability tests.
 *  15. Its semantic owner is defined.
 *  16. Its AST owner is defined.
 *  17. Its IR owner is defined.
 *  18. Its provenance/security/resource/effect boundaries are defined.
 *  19. It does not duplicate actor, security, resource, effect, or contract
 *      grammars.
 *  20. New policy capabilities can be added without changing this grammar.
 *
 * ============================================================================
 */
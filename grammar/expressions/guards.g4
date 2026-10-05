/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/expressions/guards.g4
 *
 * GRAMMAR
 * -------
 * Guards
 *
 * STATUS
 * ------
 * CANONICAL SHARED GUARD-SYNTAX GRAMMAR
 *
 * RUST BASELINE
 * -------------
 * Rust 1.97 / Rust 1.97.1
 * Edition 2021
 * Safe Rust only.
 * No unsafe Rust is required or permitted.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the single authoritative source-syntax owner for match guards.
 *
 * A guard is a boolean/predicate expression attached to a pattern-matching
 * arm.
 *
 * Canonical form:
 *
 *     pattern when expression => result
 *
 * Example:
 *
 *     value when value > threshold => value
 *
 * The guard itself is deliberately NOT an independent expression language.
 *
 * It reuses the canonical Zamani:
 *
 *     expression
 *
 * grammar.
 *
 * This means guards automatically remain compatible with the complete
 * expression system, including future expression capabilities.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     lexer
 *       |
 *       v
 *     parser
 *       |
 *       +---------------------+
 *       |                     |
 *       v                     v
 *    pattern              guardClause
 *                             |
 *                             v
 *                         expression
 *                             |
 *                             v
 *                     domain-neutral AST
 *                             |
 *                             v
 *                    structural validation
 *                             |
 *                             v
 *                      semantic analysis
 *                             |
 *       +---------------------+----------------------+
 *       |                     |                      |
 *       v                     v                      v
 *     types                effects              policies
 *       |                     |                      |
 *       +---------------------+----------------------+
 *                             |
 *                             v
 *                     canonical semantic model
 *                             |
 *                 +-----------+-----------+
 *                 |                       |
 *                 v                       v
 *             classical              quantum::ir
 *                 |                       |
 *                 +-----------+-----------+
 *                             |
 *                             v
 *                    optimization/lowering
 *                             |
 *                      routing/scheduling
 *                             |
 *                    resilience / QEC / ZQN
 *                             |
 *                            HAL
 *                             |
 *                       target realization
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     guardClause
 *
 * THIS FILE DOES NOT OWN:
 *
 *     expression
 *     pattern
 *     matchExpression
 *     matchStatement
 *     matchArm
 *     literals
 *     identifiers
 *     names
 *     types
 *     effects
 *     capabilities
 *     resources
 *     contracts
 *     policies
 *     provenance
 *     AST definitions
 *     semantic analysis
 *     IR
 *     optimization
 *     lowering
 *     scheduling
 *     routing
 *     QEC
 *     ZQN
 *     HAL
 *     runtime execution
 *     hardware selection
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There MUST be exactly one authoritative:
 *
 *     guardClause
 *
 * rule in the production parser.
 *
 * No other grammar may redefine:
 *
 *     guardClause
 *
 * In particular:
 *
 *     grammar/statements/pattern-matching.g4
 *
 * MUST consume this rule rather than define another copy.
 *
 * Likewise:
 *
 *     grammar/expressions/match.g4
 *
 * MUST consume this rule rather than define another copy.
 *
 * ============================================================================
 * LEXER AUTHORITY
 * ============================================================================
 *
 * The canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * The keyword:
 *
 *     WHEN
 *
 * is owned by:
 *
 *     grammar/lexer/keywords.g4
 *
 * This grammar MUST NOT define:
 *
 *     WHEN
 *
 * or any other lexer token.
 *
 * The guard condition uses:
 *
 *     expression
 *
 * from:
 *
 *     grammar/expressions/expressions.g4
 *
 * This grammar therefore does not duplicate expression precedence.
 *
 * ============================================================================
 * CANONICAL SYNTAX
 * ============================================================================
 *
 * The canonical guard syntax is:
 *
 *     when <expression>
 *
 * Examples:
 *
 *     when value > threshold
 *
 *     when value == expected
 *
 *     when valid && authorized
 *
 *     when probability >= confidence
 *
 *     when measurement == result
 *
 *     when capability_available
 *
 *     when predicate(value)
 *
 * The expression can contain ordinary Zamani expressions.
 *
 * This permits guards to work uniformly across:
 *
 *     classical computation
 *     quantum/classical computation
 *     HDL-oriented computation
 *     hardware abstraction
 *     distributed computation
 *     AI/ML
 *     data processing
 *     networking
 *     security
 *     resource negotiation
 *     adaptive execution
 *     simulation
 *     future computational domains
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 *
 * The guard has exactly one syntactic component after `when`:
 *
 *     expression
 *
 * Semantic analysis determines whether that expression is a valid guard.
 *
 * This grammar intentionally does NOT require a special Boolean literal or
 * Boolean-only expression grammar.
 *
 * For example:
 *
 *     when predicate(value)
 *
 * is syntactically valid.
 *
 * Whether `predicate(value)` produces an acceptable guard condition is a
 * semantic/type-system responsibility.
 *
 * ============================================================================
 */

parser grammar Guards;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * PUBLIC RULE
 * ========================================================================== */

/*
 * Canonical match guard.
 *
 * Example:
 *
 *     when value > threshold
 *
 * The expression is intentionally delegated to the universal expression
 * grammar.
 */
guardClause
    : WHEN expression
    ;


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar produces parse structure only.
 *
 * The frontend AST must preserve:
 *
 *     - the WHEN keyword/source span;
 *     - the complete guard expression;
 *     - expression source ordering;
 *     - nested expression structure;
 *     - source locations.
 *
 * The grammar does NOT define a Rust AST type.
 *
 * The semantic/frontend layer should represent the guard as the canonical
 * guard component of a MatchArm rather than inventing:
 *
 *     AiGuard
 *     QuantumGuard
 *     HardwareGuard
 *     DistributedGuard
 *     ResourceGuard
 *
 * or any other domain-specific guard node.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     - name resolution;
 *     - expression type checking;
 *     - guard result validation;
 *     - access to pattern bindings;
 *     - effect checking;
 *     - capability checking;
 *     - resource checking;
 *     - policy checking;
 *     - determinism checking where required;
 *     - purity restrictions where required;
 *     - provenance;
 *     - reachability analysis;
 *     - interaction with exhaustiveness analysis.
 *
 * The grammar MUST NOT decide whether a guard is:
 *
 *     true
 *     false
 *     pure
 *     effectful
 *     deterministic
 *     nondeterministic
 *     legal
 *     executable
 *     resource-feasible
 *     capability-feasible
 *
 * Those are semantic properties.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * The guard expression is type-checked by the ordinary Zamani type system.
 *
 * The language may eventually support:
 *
 *     Boolean
 *     predicate
 *     refinement
 *     dependent predicate
 *     symbolic proposition
 *     probabilistic predicate
 *     uncertainty-aware predicate
 *     capability predicate
 *     resource predicate
 *     policy predicate
 *
 * without changing this grammar.
 *
 * The grammar therefore does NOT enumerate guard-result types.
 *
 * The semantic type system decides which types can participate in guard
 * conditions.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * A guard has no hard-coded effect set.
 *
 * Its effects are inherited from its expression.
 *
 * Possible semantic effects include, where supported by the language:
 *
 *     IO
 *     network
 *     mutation
 *     randomness
 *     measurement
 *     foreign
 *     native
 *     distributed
 *     learning
 *     adaptation
 *     simulation
 *     reflection
 *
 * The grammar does not enumerate or enforce those effects.
 *
 * Effect checking occurs after parsing.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * A guard does not automatically require any hardware capability.
 *
 * A guard expression may nevertheless reference semantic capabilities.
 *
 * Examples:
 *
 *     when capability_available
 *
 *     when supports(operation)
 *
 *     when resource_available
 *
 * Capability resolution remains downstream.
 *
 * The grammar MUST NOT contain:
 *
 *     CPU capability limits
 *     GPU limits
 *     FPGA limits
 *     QPU limits
 *     qubit limits
 *     device limits
 *     topology limits
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Guard syntax does not impose resource requirements.
 *
 * A guard may evaluate expressions whose semantics involve:
 *
 *     resources
 *     budgets
 *     memory
 *     execution availability
 *     capabilities
 *     topology
 *     resilience state
 *
 * Resource analysis remains downstream.
 *
 * No resource quantity becomes a grammar-level maximum.
 *
 * ============================================================================
 * CONTRACT CONTRACT
 * ============================================================================
 *
 * Guards may participate in constructs whose semantic model includes:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * However, this file does not define those constructs.
 *
 * For example, a guard may semantically contribute to a conditional decision
 * whose surrounding operation has a contract.
 *
 * Contract checking remains the responsibility of:
 *
 *     grammar/validation/
 *
 * and the corresponding semantic implementation.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Guards may be evaluated under policies controlling:
 *
 *     execution
 *     security
 *     resource selection
 *     adaptation
 *     simulation
 *     distributed execution
 *     quantum execution
 *
 * Policies are not encoded in this grammar.
 *
 * Policy evaluation belongs to the semantic/execution layers.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * A guard must retain enough source provenance for downstream systems to
 * identify:
 *
 *     - the containing match;
 *     - the containing match arm;
 *     - the pattern;
 *     - the guard;
 *     - the guard expression;
 *     - source location;
 *     - transformation history.
 *
 * This permits guard-related decisions to be explained and audited.
 *
 * Provenance may be consumed by:
 *
 *     diagnostics
 *     debugging
 *     optimization explanation
 *     reproducibility
 *     verification
 *     audit
 *     scientific provenance
 *     adaptive execution analysis
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar has NO direct IR representation.
 *
 * The guard is represented in the domain-neutral semantic model as part of a
 * match arm.
 *
 * Later lowering may transform the guard into:
 *
 *     conditional control flow
 *     predicate evaluation
 *     decision-tree conditions
 *     dataflow predicates
 *     distributed predicates
 *     hardware predicates
 *     hybrid classical/quantum control
 *
 * The transformation MUST preserve source semantics.
 *
 * This grammar does not choose the representation.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Guards are permitted in hybrid and quantum/classical programs.
 *
 * For example, a guard may semantically depend on a measurement result.
 *
 * The grammar remains unaware of whether an expression originated from:
 *
 *     classical computation
 *     quantum measurement
 *     quantum simulation
 *     hardware
 *     distributed execution
 *
 * Quantum semantics must eventually pass through:
 *
 *     semantic model
 *          |
 *          v
 *     quantum::ir
 *
 * This file MUST NOT:
 *
 *     - define quantum guards;
 *     - enumerate quantum operations;
 *     - define physical qubits;
 *     - define routing;
 *     - define scheduling;
 *     - define QEC;
 *     - define ZQN;
 *     - define hardware topology.
 *
 * ============================================================================
 * HDL BOUNDARY
 * ============================================================================
 *
 * A guard can participate in hardware-oriented semantic constructs.
 *
 * The grammar does not determine whether it becomes:
 *
 *     combinational predicate
 *     sequential condition
 *     mux selection
 *     state transition
 *     software control
 *     host/device control
 *
 * HDL/hardware lowering decides this after semantic analysis.
 *
 * No fixed:
 *
 *     register width
 *     bus width
 *     signal count
 *     device count
 *     clock count
 *     pipeline depth
 *
 * may be introduced here.
 *
 * ============================================================================
 * BACKEND BOUNDARY
 * ============================================================================
 *
 * This grammar has no backend dependency.
 *
 * A compiler backend may lower a guard to:
 *
 *     CPU conditional control
 *     GPU predicate
 *     accelerator control
 *     FPGA logic
 *     ASIC logic
 *     QPU classical feed-forward
 *     distributed decision
 *     simulator control
 *
 * without modifying this grammar.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Parser-level diagnostics include:
 *
 *     - missing WHEN expression;
 *     - malformed expression after WHEN;
 *     - unexpected end of guard;
 *     - malformed tokens in the guard expression.
 *
 * Semantic diagnostics include:
 *
 *     - invalid guard result type;
 *     - unresolved identifier;
 *     - invalid pattern binding reference;
 *     - forbidden effect;
 *     - missing capability;
 *     - unsatisfied resource requirement;
 *     - policy violation;
 *     - nondeterministic guard where determinism is required.
 *
 * Parser diagnostics MUST NOT report semantic failures as syntax failures.
 *
 * ============================================================================
 * SCALABILITY / POCO-REAF
 * ============================================================================
 *
 * This grammar deliberately contains NO finite universal limit on:
 *
 *     - guard expression size;
 *     - expression nesting;
 *     - pattern binding count;
 *     - match-arm count;
 *     - number of guards in a program;
 *     - number of matches;
 *     - operand count;
 *     - data dimensions;
 *     - tensor rank;
 *     - quantum resources;
 *     - CPU resources;
 *     - GPU resources;
 *     - FPGA resources;
 *     - accelerator resources;
 *     - distributed nodes;
 *     - memory;
 *     - storage;
 *     - network size.
 *
 * There are no constants such as:
 *
 *     MAX_GUARD_DEPTH
 *     MAX_GUARD_OPERANDS
 *     MAX_GUARDS
 *     MAX_MATCH_GUARDS
 *     MAX_PATTERN_BINDINGS
 *
 * "Infinity" means that the language does not artificially cap these
 * structures.
 *
 * Actual finite limits belong to:
 *
 *     compiler resource policy
 *     parser implementation resources
 *     runtime resources
 *     target resources
 *     deployment policy
 *
 * They are not language semantics.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Given:
 *
 *     identical source;
 *     identical lexer version;
 *     identical parser grammar/version;
 *
 * guard parsing must produce equivalent parse structure.
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware;
 *     runtime scheduler state;
 *     wall-clock time;
 *     randomness;
 *     network state;
 *     filesystem state;
 *     environment state;
 *     target availability.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no Rust actions;
 *     - no semantic predicates;
 *     - no executable code;
 *     - no filesystem operations;
 *     - no networking;
 *     - no hardware discovery;
 *     - no runtime execution;
 *     - no unsafe Rust.
 *
 * The Rust frontend consuming the generated parser must remain:
 *
 *     Rust 1.97 / Rust 1.97.1
 *     Edition 2021
 *     safe Rust only.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Canonical syntax:
 *
 *     when <expression>
 *
 * Historical or future alternate guard spellings MUST NOT be introduced here
 * merely to accommodate application-specific DSLs.
 *
 * If a future language version introduces an alternate spelling, it must be
 * explicitly versioned and represented through the compatibility subsystem.
 *
 * ============================================================================
 * POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * The following must parse as part of a valid match arm:
 *
 *     value when value > threshold => result
 *
 *     value when value == expected => result
 *
 *     value when predicate(value) => result
 *
 *     value when ready && authorized => result
 *
 *     value when capability_available => result
 *
 *     value when measurement == expected => result
 *
 *     value when probability >= confidence => result
 *
 *     value when nested(a, b, c) => result
 *
 *     value when (a + b) > c => result
 *
 *     value when condition && (x < y || z == q) => result
 *
 * ============================================================================
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The following must fail structurally when used as guards:
 *
 *     value when
 *
 *     value when =>
 *
 *     value when )
 *
 *     value when ( => result
 *
 *     value when expression-that-is-malformed( => result
 *
 * Semantic-invalid guards must remain parser-valid and be diagnosed later.
 *
 * For example, if the type system rejects a non-Boolean result:
 *
 *     value when numeric_value => result
 *
 * that should be a semantic/type diagnostic rather than a parser diagnostic,
 * assuming `numeric_value` is valid Zamani expression syntax.
 *
 * ============================================================================
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Tests must include:
 *
 *     - nested expressions;
 *     - nested function calls;
 *     - pattern-bound variables;
 *     - tuple/record values;
 *     - generic values;
 *     - symbolic values;
 *     - probabilistic values;
 *     - uncertainty values;
 *     - knowledge/query results;
 *     - reasoning results;
 *     - learning/adaptation results;
 *     - measurement-derived values;
 *     - distributed results;
 *     - resource/capability values;
 *     - hardware abstraction values;
 *     - simulation values.
 *
 * ============================================================================
 * CROSS-DOMAIN TEST CONTRACT
 * ============================================================================
 *
 * The same guard syntax must work without grammar modification in:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     AI
 *     data
 *     distributed
 *     networking
 *     security
 *     simulation
 *     adaptive execution
 *     future dialects
 *
 * Domain semantics are resolved downstream.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/lexer/keywords.g4
 *     grammar/expressions/expressions.g4
 *
 * The expression dependency is a parser-rule dependency:
 *
 *     guardClause -> expression
 *
 * EXPORTS:
 *
 *     guardClause
 *
 * CONSUMED_BY:
 *
 *     grammar/expressions/match.g4
 *     grammar/statements/pattern-matching.g4
 *     future shared match-arm grammar
 *
 * AST_OWNER:
 *
 *     frontend AST / canonical MatchArm representation
 *
 * SEMANTIC_OWNER:
 *
 *     semantic analysis / pattern-matching semantics
 *
 * IR_OWNER:
 *
 *     canonical semantic model
 *
 *     with downstream classical and quantum::ir lowering as applicable
 *
 * TEST_OWNER:
 *
 *     grammar/tests/parser/
 *     grammar/tests/expressions/
 *     grammar/tests/statements/
 *     grammar/tests/negative/
 *     grammar/tests/boundary/
 *     grammar/tests/scalability/
 *     grammar/tests/determinism/
 *
 * SPEC_OWNER:
 *
 *     grammar/specification/
 *     grammar/spec/
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * 1. LEXER
 *
 * The lexer must provide:
 *
 *     WHEN
 *
 * through the canonical keyword vocabulary.
 *
 *
 * 2. EXPRESSION COMPOSITION
 *
 * `expression` remains owned by:
 *
 *     grammar/expressions/expressions.g4
 *
 * This file MUST NOT redefine expression.
 *
 *
 * 3. PATTERN COMPOSITION
 *
 * `pattern` remains owned by:
 *
 *     grammar/expressions/patterns.g4
 *
 * This file MUST NOT define or import a second pattern language.
 *
 *
 * 4. MATCH EXPRESSION
 *
 *     grammar/expressions/match.g4
 *
 * MUST consume:
 *
 *     guardClause
 *
 * rather than defining another guard rule.
 *
 *
 * 5. MATCH STATEMENT
 *
 *     grammar/statements/pattern-matching.g4
 *
 * MUST consume:
 *
 *     guardClause
 *
 * rather than defining another guard rule.
 *
 *
 * 6. DUPLICATE REMOVAL
 *
 * The following competing rule MUST be removed from:
 *
 *     grammar/statements/pattern-matching.g4
 *
 *     guardClause
 *
 * Its canonical owner becomes this file.
 *
 *
 * 7. ROOT COMPOSITION
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * receives guard syntax through the expression/statement composition
 * hierarchy.
 *
 * It must not define `guardClause` itself.
 *
 *
 * 8. LEGACY MONOLITH
 *
 * Any legacy `guardClause` implementation in the historical/monolithic
 * grammar must either:
 *
 *     - be removed from the canonical generated grammar; or
 *     - remain explicitly marked historical/reference material.
 *
 * There must never be two active canonical guard implementations.
 *
 * ============================================================================
 * INTEGRATION ORDER
 * ============================================================================
 *
 * Complete integration in this order:
 *
 *     1. Verify canonical WHEN lexer token.
 *
 *     2. Add this Guards parser grammar.
 *
 *     3. Make the canonical expression composition expose `expression`.
 *
 *     4. Make match-expression composition consume `guardClause`.
 *
 *     5. Make match-statement composition consume `guardClause`.
 *
 *     6. Remove the old `guardClause` definition from
 *        statements/pattern-matching.g4.
 *
 *     7. Ensure patterns.g4 remains the sole pattern owner.
 *
 *     8. Remove duplicate match-arm definitions.
 *
 *     9. Ensure the generated parser has exactly one `guardClause`.
 *
 *    10. Add parser/AST/semantic/conformance tests.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [ ] grammar compiles as part of the canonical parser composition;
 *     [ ] WHEN comes from the canonical lexer;
 *     [ ] guardClause has exactly one production owner;
 *     [ ] expression has exactly one production owner;
 *     [ ] no guard-specific expression hierarchy exists;
 *     [ ] no pattern rules are duplicated here;
 *     [ ] no match-arm rules are duplicated here;
 *     [ ] AST representation is canonical;
 *     [ ] semantic guard checking is downstream;
 *     [ ] effect checking is downstream;
 *     [ ] capability checking is downstream;
 *     [ ] resource checking is downstream;
 *     [ ] policy checking is downstream;
 *     [ ] provenance is preserved;
 *     [ ] no hardware limits exist;
 *     [ ] no quantum limits exist;
 *     [ ] no application-specific limits exist;
 *     [ ] no target-specific syntax exists;
 *     [ ] no unsafe Rust is introduced;
 *     [ ] positive tests pass;
 *     [ ] negative tests pass;
 *     [ ] boundary tests pass;
 *     [ ] scalability tests pass;
 *     [ ] determinism tests pass;
 *     [ ] compatibility tests pass;
 *     [ ] cross-domain tests pass.
 *
 * ============================================================================
 */
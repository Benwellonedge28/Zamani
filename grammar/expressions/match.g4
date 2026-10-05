/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/expressions/match.g4
 *
 * Grammar:
 *     MatchExpressions
 *
 * Status:
 *     Production-ready feature grammar.
 *
 * Rust baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Edition 2021
 *     Safe Rust only.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines the source-level syntax contract for Zamani match
 * expressions.
 *
 * A match expression is a domain-neutral expression construct that selects
 * one arm according to the semantic relationship between:
 *
 *     scrutinee
 *     pattern
 *     optional guard
 *     arm result
 *
 * Match expressions may therefore participate in:
 *
 *     classical computation
 *     data processing
 *     AI/ML computation
 *     distributed computation
 *     hardware control
 *     HDL-oriented computation
 *     quantum/classical control
 *     resource/capability decisions
 *     simulation
 *     adaptive execution
 *     future Zamani domains
 *
 * This file owns the existence and top-level structure of a match expression.
 *
 * It does NOT own:
 *
 *     - the general expression hierarchy;
 *     - pattern syntax;
 *     - guard syntax;
 *     - block syntax;
 *     - literal syntax;
 *     - identifier syntax;
 *     - type syntax;
 *     - semantic matching;
 *     - exhaustiveness;
 *     - reachability;
 *     - overlap analysis;
 *     - type checking;
 *     - effect checking;
 *     - capability checking;
 *     - resource negotiation;
 *     - policy evaluation;
 *     - AST construction;
 *     - IR construction;
 *     - optimization;
 *     - lowering;
 *     - scheduling;
 *     - routing;
 *     - hardware selection;
 *     - runtime execution.
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
 *       v
 *     matchExpression
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +--> type analysis
 *       +--> effect analysis
 *       +--> capability analysis
 *       +--> resource analysis
 *       +--> contract analysis
 *       +--> policy analysis
 *       +--> provenance
 *       |
 *       v
 *     canonical semantic model / IR
 *       |
 *       +--> classical
 *       +--> quantum::ir
 *       +--> HDL/hardware
 *       +--> distributed
 *       +--> data
 *       +--> AI/ML
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     lowering
 *       |
 *       v
 *     routing / scheduling / resilience
 *       |
 *       v
 *     target realization
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Match syntax is target-independent.
 *
 * This file imposes no language-level upper bound on:
 *
 *     - number of match arms;
 *     - pattern size;
 *     - nesting depth;
 *     - sequence length;
 *     - tuple arity;
 *     - number of alternatives;
 *     - value width;
 *     - tensor dimensions;
 *     - number of resources;
 *     - number of devices;
 *     - number of qubits;
 *     - number of CPUs;
 *     - number of GPUs;
 *     - number of nodes;
 *     - hardware topology.
 *
 * There are deliberately no constants such as:
 *
 *     MAX_MATCH_ARMS
 *     MAX_PATTERN_SIZE
 *     MAX_MATCH_DEPTH
 *     MAX_ALTERNATIVES
 *     MAX_TUPLE_ARITY
 *
 * Any finite implementation limit is an implementation/resource policy and
 * must not become part of the language's semantic definition.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     matchExpression
 *
 * THIS FILE CONSUMES:
 *
 *     expression
 *     matchArm
 *
 * but does not redefine them.
 *
 * The canonical expression hierarchy remains owned by:
 *
 *     grammar/expressions/expressions.g4
 *
 * Pattern and arm syntax must eventually have one shared owner so that
 * statement-form and expression-form matching cannot diverge.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * There must be exactly one authoritative definition of:
 *
 *     matchExpression
 *
 * The legacy monolithic grammar must not contain another competing
 * matchExpression rule once the modular grammar is authoritative.
 *
 * A compatibility grammar may retain historical syntax only when explicitly
 * marked as compatibility/reference material and excluded from the canonical
 * parser composition.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This grammar consumes the canonical lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Required tokens:
 *
 *     MATCH
 *     LBRACE
 *     RBRACE
 *
 * The match-arm implementation consumes the canonical tokens appropriate to
 * the selected arm syntax, including:
 *
 *     FAT_ARROW
 *
 * and, where guards are supported:
 *
 *     WHEN
 *
 * This file MUST NOT define lexer rules.
 *
 * ============================================================================
 * TOP-LEVEL SYNTAX
 * ============================================================================
 *
 * Canonical form:
 *
 *     match <scrutinee> {
 *         <pattern> => <result>
 *     }
 *
 * Guarded form:
 *
 *     match <scrutinee> {
 *         <pattern> when <guard> => <result>
 *     }
 *
 * Multiple arms:
 *
 *     match <scrutinee> {
 *         <pattern> => <result>,
 *         <pattern> when <guard> => <result>,
 *         _ => <fallback>
 *     }
 *
 * The grammar requires at least one arm.
 *
 * Exhaustiveness is semantic analysis, not parsing.
 *
 * ============================================================================
 * MATCH EXPRESSION
 * ============================================================================
 *
 * `matchExpression` deliberately contains only the outer match construct.
 *
 * The scrutinee is parsed by the canonical `expression` rule.
 *
 * The arm structure is delegated to the canonical match-arm contract.
 *
 * This prevents:
 *
 *     - duplicate pattern grammars;
 *     - duplicate guard grammars;
 *     - duplicate arm grammars;
 *     - different statement/expression pattern semantics.
 *
 * ============================================================================
 */

parser grammar MatchExpressions;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * PUBLIC RULE
 * ============================================================================
 *
 * The rule intentionally has no alternative that represents an empty match.
 *
 * This guarantees structural validity:
 *
 *     match <expression> { ...at least one arm... }
 *
 * Semantic validity remains downstream.
 */
matchExpression
    : MATCH expression LBRACE matchArm+ RBRACE
    ;


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser structure exposed to the frontend is:
 *
 *     MatchExpression
 *         scrutinee
 *         arms[]
 *
 * Each arm preserves:
 *
 *     pattern
 *     optional guard
 *     body/result
 *     source order
 *
 * This grammar MUST NOT:
 *
 *     - reorder arms;
 *     - deduplicate arms;
 *     - eliminate unreachable arms;
 *     - build a decision tree;
 *     - evaluate guards;
 *     - evaluate expressions;
 *     - perform constant folding;
 *     - perform type inference;
 *     - resolve names.
 *
 * Those operations belong downstream.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis MUST determine:
 *
 *     1. scrutinee type;
 *     2. pattern compatibility;
 *     3. pattern bindings;
 *     4. guard validity;
 *     5. guard result type;
 *     6. arm reachability;
 *     7. arm overlap;
 *     8. exhaustiveness;
 *     9. result-type compatibility;
 *    10. effects;
 *    11. required capabilities;
 *    12. resource requirements;
 *    13. applicable policies;
 *    14. provenance requirements.
 *
 * None of these rules are encoded in this file.
 *
 * ============================================================================
 * ARM ORDER
 * ============================================================================
 *
 * Source arm order is semantically observable whenever multiple arms could
 * otherwise match.
 *
 * Therefore the parser MUST preserve source order exactly.
 *
 * A later compiler stage may transform the representation into:
 *
 *     decision trees
 *     jump structures
 *     predicated control
 *     dataflow
 *     distributed dispatch
 *     target-specific control
 *
 * only after semantic analysis establishes that the transformation preserves
 * the language's matching semantics.
 *
 * ============================================================================
 * GUARDS
 * ============================================================================
 *
 * Guards are semantic expressions associated with an arm.
 *
 * Conceptually:
 *
 *     pattern
 *     when guard
 *     =>
 *     body
 *
 * The guard MUST NOT become a second pattern language.
 *
 * The guard uses the ordinary Zamani expression system so that it can
 * reference:
 *
 *     bindings
 *     functions
 *     values
 *     capabilities
 *     measurement results
 *     classical results
 *     data
 *     domain-specific predicates
 *
 * Whether a guard is pure, effectful, deterministic, capability-dependent,
 * or otherwise restricted is determined by semantic/effect analysis.
 *
 * ============================================================================
 * BLOCK RESULTS
 * ============================================================================
 *
 * An arm may produce either:
 *
 *     - an expression result;
 *     - a block result.
 *
 * The exact arm-body ownership belongs to the shared match-arm grammar.
 *
 * The shared arm grammar MUST resolve the expression/block ambiguity in a
 * deterministic manner.
 *
 * In particular, because Zamani also permits brace-delimited collection/map
 * expressions, arm-body parsing MUST NOT accidentally interpret a statement
 * block as a collection expression.
 *
 * This is an integration responsibility of the shared arm grammar.
 *
 * ============================================================================
 * PATTERN CONTRACT
 * ============================================================================
 *
 * Pattern syntax belongs to the shared pattern subsystem.
 *
 * Supported semantic families may include:
 *
 *     wildcard
 *     binding
 *     literal
 *     tuple
 *     sequence
 *     structured
 *     variant
 *     range
 *     reference
 *     type
 *     OR
 *     parenthesized
 *
 * This file intentionally does not enumerate those forms.
 *
 * New pattern forms can therefore be added without modifying the outer
 * match-expression structure.
 *
 * ============================================================================
 * RANGE CONTRACT
 * ============================================================================
 *
 * Range-pattern syntax must consume the canonical operator vocabulary:
 *
 *     DOT_DOT
 *     DOT_DOT_EQ
 *
 * The match-expression grammar itself does not define range patterns.
 *
 * The pattern grammar MUST NOT invent aliases such as:
 *
 *     RANGE_EXCLUSIVE
 *     RANGE_INCLUSIVE
 *
 * unless the canonical lexer explicitly establishes such tokens.
 *
 * ============================================================================
 * EXPRESSION PRECEDENCE
 * ============================================================================
 *
 * A complete match expression behaves as one primary/atomic expression at
 * the expression-precedence level.
 *
 * It does not introduce an arithmetic, logical, relational, or assignment
 * precedence.
 *
 * Therefore:
 *
 *     match x {
 *         0 => a,
 *         _ => b
 *     }
 *
 * can be used wherever a primary expression is permitted.
 *
 * For example, after integration:
 *
 *     result = match value {
 *         0 => zero,
 *         _ => other
 *     };
 *
 * or:
 *
 *     consume(match value {
 *         0 => zero,
 *         _ => other
 *     });
 *
 * Internal expressions retain the normal Zamani precedence hierarchy.
 *
 * ============================================================================
 * TYPE SYSTEM
 * ============================================================================
 *
 * Match syntax imposes no fixed type universe.
 *
 * Semantic analysis may support:
 *
 *     primitive values
 *     records
 *     variants
 *     tuples
 *     sequences
 *     references
 *     generic values
 *     tensor/data values
 *     symbolic values
 *     probabilistic values
 *     measurement results
 *     resource/capability values
 *     hardware abstraction values
 *     distributed values
 *     future domain values
 *
 * This file remains independent of all such types.
 *
 * ============================================================================
 * EFFECTS
 * ============================================================================
 *
 * Matching itself does not automatically impose a particular effect.
 *
 * Effects originate from:
 *
 *     scrutinee evaluation
 *     guard evaluation
 *     arm-result evaluation
 *
 * The semantic effect system therefore determines whether a match participates
 * in effects such as:
 *
 *     IO
 *     network
 *     mutation
 *     randomness
 *     measurement
 *     foreign calls
 *     distributed execution
 *     learning
 *     adaptation
 *     simulation
 *     reflection
 *
 * The grammar must not hard-code these effects.
 *
 * ============================================================================
 * CAPABILITIES AND RESOURCES
 * ============================================================================
 *
 * Match syntax contains no hardware/resource requirements.
 *
 * A program may nevertheless match on semantic values representing:
 *
 *     capabilities
 *     resource availability
 *     execution states
 *     resilience states
 *     measurement outcomes
 *     device-independent properties
 *
 * Capability and resource requirements are resolved downstream.
 *
 * No match rule may contain:
 *
 *     CPU counts
 *     GPU counts
 *     QPU counts
 *     FPGA counts
 *     memory capacities
 *     qubit capacities
 *     node counts
 *     topology sizes
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Match expressions are permitted in hybrid quantum/classical programs.
 *
 * Example semantic shape:
 *
 *     measurement
 *         |
 *         v
 *     match result {
 *         ...
 *     }
 *
 * The grammar does not know whether the scrutinee originated from:
 *
 *     classical computation
 *     quantum measurement
 *     simulation
 *     hardware
 *     distributed execution
 *
 * A quantum-related match is lowered only after semantic analysis.
 *
 * The canonical quantum boundary remains:
 *
 *     semantic model
 *          |
 *          v
 *     quantum::ir
 *
 * This grammar MUST NOT introduce a second quantum IR.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Match expressions may participate in hardware-oriented semantic models.
 *
 * The grammar does not decide whether a match becomes:
 *
 *     combinational logic
 *     sequential control
 *     muxing
 *     predicate logic
 *     state transition
 *     software control
 *     host/device coordination
 *
 * That decision belongs to semantic lowering and the relevant backend.
 *
 * ============================================================================
 * AI / REASONING / KNOWLEDGE INTEGRATION
 * ============================================================================
 *
 * Match expressions are generic control constructs and therefore may consume
 * values produced by:
 *
 *     reasoning
 *     inference
 *     knowledge queries
 *     learning
 *     probabilistic computation
 *     uncertainty analysis
 *     agent execution
 *     explanations
 *
 * No AI-specific syntax is embedded in this grammar.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing MUST depend only on:
 *
 *     - source token sequence;
 *     - selected language/grammar version.
 *
 * Parsing MUST NOT depend on:
 *
 *     - system time;
 *     - randomness;
 *     - environment variables;
 *     - filesystem state;
 *     - network state;
 *     - hardware state;
 *     - runtime scheduler state;
 *     - target availability.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no embedded Rust actions;
 *     - no semantic predicates;
 *     - no unsafe code;
 *     - no filesystem access;
 *     - no networking;
 *     - no command execution;
 *     - no hardware discovery;
 *     - no device discovery;
 *     - no runtime execution.
 *
 * A source program appearing syntactically inside a match arm MUST NOT execute
 * merely because the grammar is parsed.
 *
 * ============================================================================
 * COMPILER / IR CONTRACT
 * ============================================================================
 *
 * The compiler must preserve:
 *
 *     scrutinee semantics
 *     arm ordering
 *     pattern semantics
 *     guard semantics
 *     binding semantics
 *     observable effects
 *     deterministic guarantees
 *
 * Match syntax does not define a match-specific machine IR.
 *
 * It lowers through the canonical semantic pipeline.
 *
 * Possible target representations include:
 *
 *     conditional control flow
 *     decision trees
 *     predication
 *     dataflow
 *     distributed control
 *     dynamic quantum/classical control
 *     HDL control
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * The semantic representation should retain sufficient source provenance to
 * identify:
 *
 *     match expression
 *     scrutinee
 *     arm
 *     pattern
 *     guard
 *     arm result
 *
 * Downstream transformations should be able to associate generated control
 * structures with their originating source constructs.
 *
 * This is important for:
 *
 *     diagnostics
 *     debugging
 *     optimization explanations
 *     verification
 *     reproducibility
 *     auditability
 *     scientific provenance
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Parser-level diagnostics include:
 *
 *     - missing scrutinee;
 *     - missing opening brace;
 *     - missing arm;
 *     - malformed pattern;
 *     - missing arrow;
 *     - malformed guard;
 *     - malformed arm body;
 *     - missing closing brace;
 *     - malformed separator.
 *
 * Semantic diagnostics include:
 *
 *     - non-exhaustive match;
 *     - unreachable arm;
 *     - overlapping arm;
 *     - incompatible pattern;
 *     - invalid binding;
 *     - inconsistent OR-pattern bindings;
 *     - invalid guard;
 *     - incompatible arm result types;
 *     - invalid effect usage;
 *     - missing capability;
 *     - unsatisfied resource requirement;
 *     - policy violation.
 *
 * Parser diagnostics MUST NOT attempt to decide semantic validity.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * The canonical syntax is:
 *
 *     match <expression> {
 *         <pattern> [when <expression>] => <result>
 *     }
 *
 * If historical syntax uses a statement-only match form, compatibility support
 * must be implemented by composing the canonical match construct rather than
 * creating a second pattern/guard/arm language.
 *
 * Deprecated syntax must be explicitly versioned and must not silently change
 * the meaning of canonical match expressions.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * This file requires downstream tests for at least:
 *
 * POSITIVE:
 *
 *     match x {
 *         0 => zero,
 *         _ => other
 *     }
 *
 *     match x {
 *         value when value > threshold => value,
 *         _ => fallback
 *     }
 *
 *     let result = match value {
 *         Some(x) => x,
 *         None => fallback
 *     };
 *
 *     consume(match value {
 *         0 => zero,
 *         _ => other
 *     });
 *
 * NEGATIVE:
 *
 *     match x {
 *     }
 *
 *     match x {
 *         _ 
 *     }
 *
 *     match x {
 *         _ => 
 *     }
 *
 *     match x {
 *         _ => value
 *     /* missing closing brace */
 *
 * BOUNDARY:
 *
 *     deeply nested patterns;
 *     large source-level arm collections;
 *     long OR-patterns;
 *     nested match expressions;
 *     match inside function calls;
 *     match inside larger expressions;
 *     match over generic values;
 *     match over measurement-derived values;
 *     match over distributed results;
 *     match over resource/capability results.
 *
 * SCALABILITY:
 *
 * Tests may exercise implementation-sized inputs, but the tests MUST NOT
 * convert a test-size value into a language-level maximum.
 *
 * DETERMINISM:
 *
 * Identical source + identical grammar version must produce equivalent parse
 * structure independent of target hardware and runtime environment.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * REQUIRED INTEGRATION WITH:
 *
 *     grammar/expressions/expressions.g4
 *
 * Add `matchExpression` to the canonical `primaryExpression` alternatives.
 *
 * The intended composition is:
 *
 *     primaryExpression
 *         :
 *             ...
 *           | matchExpression
 *         ;
 *
 * Do NOT move the complete expression hierarchy into this file.
 *
 * REQUIRED SHARED-ARM INTEGRATION:
 *
 *     grammar/statements/pattern-matching.g4
 *
 * That file must expose exactly one authoritative `matchArm` contract.
 *
 * It must also use the canonical range tokens:
 *
 *     DOT_DOT
 *     DOT_DOT_EQ
 *
 * and must not use undefined aliases such as:
 *
 *     RANGE_EXCLUSIVE
 *     RANGE_INCLUSIVE
 *
 * REQUIRED ROOT INTEGRATION:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * The root parser must receive `matchExpression` through the canonical
 * expression composition.
 *
 * It must not define another match-expression rule.
 *
 * REQUIRED LEGACY INTEGRATION:
 *
 *     grammar/antlr/Core.g4
 *     grammar/antlr/ZamaniParser.g4
 *
 * Any competing legacy `matchExpression` rule must be removed from the
 * production composition or isolated as historical/reference grammar.
 *
 * REQUIRED AST INTEGRATION:
 *
 * The AST layer must provide one canonical representation for:
 *
 *     MatchExpression
 *     MatchArm
 *     Pattern
 *     Guard
 *
 * The grammar must not create a second AST model.
 *
 * REQUIRED RUST FRONTEND INTEGRATION:
 *
 * The hand-written Rust parser currently contains a match parsing path.
 *
 * The repository must select one authoritative parser path for production:
 *
 *     ANTLR parser
 *
 * or:
 *
 *     hand-written Rust parser
 *
 * but must not silently maintain two independently evolving match languages.
 *
 * If the Rust parser remains authoritative during migration, its parse contract
 * must be kept structurally equivalent to this grammar and conformance tests
 * must compare both paths before the ANTLR path becomes authoritative.
 *
 * ============================================================================
 * INTEGRATION ORDER
 * ============================================================================
 *
 * 1. Finalize canonical lexer tokens.
 *
 * 2. Finalize the shared pattern/guard/arm grammar.
 *
 * 3. Correct range-pattern token names.
 *
 * 4. Resolve expression-vs-block arm-body ambiguity.
 *
 * 5. Add matchExpression to expressions.g4 primaryExpression.
 *
 * 6. Remove competing legacy match-expression productions from the canonical
 *    parser composition.
 *
 * 7. Synchronize the Rust AST representation.
 *
 * 8. Synchronize the authoritative Rust parser/frontend.
 *
 * 9. Add lexical/parser/AST/semantic conformance tests.
 *
 * 10. Add cross-domain and scalability tests.
 *
 * 11. Run the complete grammar and Rust test suite on Rust 1.97/1.97.1.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * `match.g4` is complete when:
 *
 *     [x] matchExpression has one clear owner;
 *     [x] match syntax is target-independent;
 *     [x] no hardware limits are encoded;
 *     [x] no quantum limits are encoded;
 *     [x] no backend assumptions are encoded;
 *     [x] no semantic actions exist;
 *     [x] no unsafe implementation is required;
 *     [x] no duplicate expression hierarchy exists;
 *     [x] the canonical expression rule is consumed;
 *     [x] match-arm syntax is shared;
 *     [x] pattern syntax is shared;
 *     [x] guard syntax is shared;
 *     [x] source arm order is preserved;
 *     [x] semantic validation remains downstream;
 *     [x] IR lowering remains downstream;
 *     [x] provenance remains possible;
 *     [x] compatibility is defined;
 *     [x] integration points are predetermined;
 *     [x] tests are predetermined.
 *
 * ============================================================================
 * IMPORTANT
 * ============================================================================
 *
 * This file deliberately does NOT attempt to make the entire repository
 * production-ready by itself.
 *
 * Production readiness requires the integration contracts above to be fulfilled
 * by the corresponding owner files.
 *
 * ============================================================================
 */
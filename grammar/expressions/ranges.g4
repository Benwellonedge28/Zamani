/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/expressions/ranges.g4
 *
 * Status:
 *     Production range-expression parser grammar.
 *
 * Purpose:
 *     Own the syntax of Zamani range expressions while remaining completely
 *     independent of machine size, memory capacity, iterator implementation,
 *     hardware topology, quantum-device size, or target architecture.
 *
 * ============================================================================
 * ARCHITECTURAL OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - range-expression syntax;
 *   - range endpoints;
 *   - inclusive/exclusive endpoint syntax;
 *   - open/unbounded range syntax;
 *   - range-with-step syntax;
 *   - range-with-count syntax where the language specification supports it;
 *   - range composition syntax;
 *   - parser-level distinction between bounded and unbounded forms;
 *   - parser-level distinction between inclusive and exclusive bounds.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexer token definitions;
 *   - identifier spelling;
 *   - numeric literal spelling;
 *   - arithmetic precedence;
 *   - unary-expression precedence;
 *   - general expression precedence;
 *   - type checking;
 *   - constant evaluation;
 *   - range cardinality calculation;
 *   - overflow detection;
 *   - iteration;
 *   - collection allocation;
 *   - memory management;
 *   - parallel execution;
 *   - scheduling;
 *   - hardware discovery;
 *   - hardware topology;
 *   - quantum allocation;
 *   - physical qubit selection;
 *   - CPU/GPU/FPGA selection;
 *   - resource allocation;
 *   - optimization;
 *   - routing;
 *   - QEC;
 *   - ZQN;
 *   - simulation;
 *   - classical IR;
 *   - quantum::ir;
 *   - HDL IR;
 *   - runtime representation;
 *   - target lowering;
 *   - machine-specific limits.
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 *     lexer
 *       |
 *       v
 *     Ranges
 *       |
 *       v
 *     Expressions
 *       |
 *       v
 *     frontend AST
 *       |
 *       +--> name resolution
 *       +--> type checking
 *       +--> constant evaluation
 *       +--> effect checking
 *       +--> resource validation
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> HDL/control IR
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     scheduling / routing / lowering
 *       |
 *       v
 *     target realization
 *
 * There is deliberately NO dependency:
 *
 *     ranges -> quantum::ir
 *     ranges -> hardware
 *     ranges -> runtime
 *     ranges -> scheduler
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * A range expresses a computation-domain concept.
 *
 * It does not prescribe how the range is physically represented.
 *
 * For example:
 *
 *     0..n
 *
 * may become:
 *
 *     - a lazy iterator;
 *     - a compile-time sequence;
 *     - a classical loop;
 *     - a distributed partition;
 *     - a GPU launch domain;
 *     - an FPGA iteration domain;
 *     - a quantum-control iteration domain;
 *     - a symbolic mathematical domain;
 *     - an HDL generation domain;
 *
 * depending on downstream semantics.
 *
 * The grammar therefore MUST NOT choose any of those implementations.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are deliberately NO grammar constants such as:
 *
 *     MAX_RANGE_SIZE
 *     MAX_RANGE_LENGTH
 *     MAX_RANGE_STEP
 *     MAX_RANGE_DEPTH
 *     MAX_ENDPOINT_VALUE
 *     MAX_INTEGER_BITS
 *     MAX_ITERATIONS
 *     MAX_LOOP_ITERATIONS
 *     MAX_ARRAY_LENGTH
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_DEVICES
 *
 * A range can describe arbitrarily large values or domains subject only to
 * the resources and semantic policies available to the eventual implementation.
 *
 * The grammar imposes no artificial cardinality limit.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This is a PARSER grammar.
 *
 * The canonical Zamani lexer owns:
 *
 *     DOT
 *     RANGE / RANGE_INCLUSIVE / RANGE_EXCLUSIVE tokens, if adopted
 *     COLON
 *     HASH
 *     COMMA
 *     INTEGER
 *     FLOAT
 *     IDENT
 *     and all other lexical tokens.
 *
 * This file MUST NOT define lexer rules.
 *
 * ============================================================================
 * CANONICAL OPERATOR POLICY
 * ============================================================================
 *
 * Range punctuation must have exactly one lexical owner.
 *
 * This grammar intentionally uses the canonical token names:
 *
 *     RANGE
 *     RANGE_EXCLUSIVE
 *
 * only where those tokens are established by the Zamani lexer contract.
 *
 * If the canonical lexer uses a different spelling, the lexer vocabulary must
 * be reconciled centrally. This file must not invent a second lexical
 * vocabulary.
 *
 * ============================================================================
 * RANGE SEMANTICS
 * ============================================================================
 *
 * The grammar represents:
 *
 *     lower .. upper
 *
 * as a bounded range.
 *
 * Endpoint openness is represented separately from endpoint value.
 *
 * This permits the semantic layer to distinguish:
 *
 *     a..b
 *     a..<b
 *
 * without encoding iteration policy into parsing.
 *
 * Unbounded forms may represent:
 *
 *     ..b
 *     a..
 *     ..
 *
 * where those forms are admitted by the surrounding language construct.
 *
 * The semantic layer determines whether an unbounded range is valid in a
 * particular context.
 *
 * ============================================================================
 * STEP SEMANTICS
 * ============================================================================
 *
 * A step is a semantic value, not a machine increment instruction.
 *
 * Examples:
 *
 *     0..n step s
 *     0..<n step s
 *
 * are syntax.
 *
 * The semantic layer determines:
 *
 *     - whether the step is valid;
 *     - whether zero is permitted;
 *     - whether the direction is valid;
 *     - whether the endpoint is reachable;
 *     - whether overflow is possible;
 *     - whether the range is finite;
 *     - whether evaluation is lazy;
 *     - whether distribution is possible.
 *
 * ============================================================================
 * COUNT SEMANTICS
 * ============================================================================
 *
 * Count-based ranges, when supported, express a requested cardinality.
 *
 * Example conceptual form:
 *
 *     start .. #count
 *
 * The grammar does not decide whether the count is representable by a
 * particular machine.
 *
 * ============================================================================
 * RANGE COMPOSITION
 * ============================================================================
 *
 * A range endpoint is an expression.
 *
 * Therefore:
 *
 *     0..n
 *     start..end
 *     offset..(base + width)
 *     q..(q + count)
 *
 * are all syntactically possible.
 *
 * Type compatibility and range validity are semantic concerns.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Range syntax can be used in source constructs involving:
 *
 *     qubit ranges;
 *     logical-qubit ranges;
 *     classical registers;
 *     measurement domains;
 *     parameter domains;
 *     circuit iteration;
 *     quantum-control iteration.
 *
 * This grammar does NOT determine:
 *
 *     - number of physical qubits;
 *     - QPU topology;
 *     - qubit allocation;
 *     - routing;
 *     - QEC;
 *     - ZQN;
 *     - gate scheduling.
 *
 * A range such as:
 *
 *     0..qubit_count
 *
 * remains a source-level expression.
 *
 * If it contributes to quantum::ir, the frontend/semantic lowering layer
 * performs that translation.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Ranges may describe:
 *
 *     bus slices;
 *     generated instances;
 *     address domains;
 *     pipeline indices;
 *     replicated hardware structures;
 *     parameterized hardware generation.
 *
 * The grammar does not choose:
 *
 *     FPGA width;
 *     ASIC implementation;
 *     physical address space;
 *     number of generated units;
 *     clock domains.
 *
 * ============================================================================
 * DISTRIBUTED / PARALLEL INTEGRATION
 * ============================================================================
 *
 * A range can later be interpreted as:
 *
 *     sequential iteration;
 *     parallel iteration;
 *     partition domain;
 *     distributed index space;
 *     tensor dimension;
 *     accelerator launch domain.
 *
 * Those are semantic/compiler decisions.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST should preserve at least:
 *
 *     start
 *     end
 *     start_bound
 *     end_bound
 *     step
 *     count
 *     source span
 *
 * where applicable.
 *
 * The AST must distinguish:
 *
 *     absent endpoint
 *
 * from:
 *
 *     endpoint whose expression happens to evaluate to a value.
 *
 * The AST must also preserve whether an endpoint is inclusive or exclusive.
 *
 * The grammar does not construct AST nodes directly.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     - endpoint type compatibility;
 *     - step type compatibility;
 *     - count validity;
 *     - ordering;
 *     - direction;
 *     - zero-step rejection;
 *     - overflow policy;
 *     - infinite-range detection;
 *     - cardinality;
 *     - constant evaluation;
 *     - symbolic-range validity;
 *     - resource implications.
 *
 * None of these belong in this grammar.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Given identical:
 *
 *     source text;
 *     language version;
 *     lexer version;
 *
 * parsing must produce the same parse structure.
 *
 * Parsing must never depend on:
 *
 *     CPU count;
 *     GPU count;
 *     FPGA count;
 *     QPU topology;
 *     available memory;
 *     runtime state;
 *     backend;
 *     scheduler state;
 *     calibration state;
 *     network state.
 *
 * ============================================================================
 * ERROR BOUNDARY
 * ============================================================================
 *
 * SYNTAX ERRORS BELONG HERE:
 *
 *     malformed range punctuation;
 *     missing endpoint where the selected form requires one;
 *     malformed step syntax;
 *     malformed count syntax;
 *     malformed range composition.
 *
 * SEMANTIC ERRORS DO NOT BELONG HERE:
 *
 *     zero step;
 *     incompatible endpoint types;
 *     reversed range under a policy that forbids it;
 *     integer overflow;
 *     excessive cardinality;
 *     unavailable hardware resources;
 *     unavailable qubits;
 *     impossible FPGA placement;
 *     unsupported parallelization.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Range punctuation and associativity are language compatibility contracts.
 *
 * Existing range syntax must not be silently reinterpreted.
 *
 * Any breaking change requires an explicit language-version decision.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This file contains no Rust implementation code.
 *
 * Rust components consuming its generated parser must support:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * and safe Rust only.
 *
 * Repository Rust code must enforce:
 *
 *     #![forbid(unsafe_code)]
 *
 * or an equivalent repository-wide policy.
 *
 * ============================================================================
 */

parser grammar Ranges;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * `rangeExpression` is the single public range-expression rule.
 *
 * Other expression grammars should consume this rule rather than duplicating
 * range syntax.
 * ============================================================================
 */

rangeExpression
    : rangeCore
    ;


/*
 * ============================================================================
 * RANGE CORE
 * ============================================================================
 *
 * A range is composed from:
 *
 *     optional lower endpoint
 *     range operator
 *     optional upper endpoint
 *     optional range modifier
 *
 * Endpoint expressions are supplied by the surrounding expression grammar.
 *
 * `rangeEndpoint` is intentionally an integration boundary rather than a
 * duplicate expression grammar.
 * ============================================================================
 */

rangeCore
    : rangeEndpoint? rangeOperator rangeEndpoint? rangeModifier*
    ;


/*
 * ============================================================================
 * RANGE OPERATORS
 * ============================================================================
 *
 * The exact lexical representation is owned by ZamaniLexer.
 *
 * RANGE represents the canonical inclusive range operator.
 *
 * RANGE_EXCLUSIVE represents an explicitly exclusive upper-bound form.
 *
 * Keeping these as distinct tokens allows the parser to preserve endpoint
 * intent without semantic guessing.
 * ============================================================================
 */

rangeOperator
    : RANGE
    | RANGE_EXCLUSIVE
    ;


/*
 * ============================================================================
 * RANGE ENDPOINT
 * ============================================================================
 *
 * An endpoint is an arbitrary expression at the semantic level.
 *
 * It must not be restricted to integer literals.
 *
 * Examples:
 *
 *     0..n
 *     start..end
 *     offset..(base + width)
 *     q..(q + count)
 *     index..limit
 *
 * The actual expression rule is supplied by the canonical Expressions
 * grammar.
 *
 * `rangeEndpoint` is intentionally kept as an integration rule so that this
 * grammar does not recursively import the complete expression hierarchy.
 * ============================================================================
 */

rangeEndpoint
    : rangeEndpointExpression
    ;


/*
 * ============================================================================
 * EXPRESSION INTEGRATION BRIDGE
 * ============================================================================
 *
 * The canonical Expressions grammar MUST bind this rule to the appropriate
 * expression level.
 *
 * This file deliberately does not define:
 *
 *     expression
 *     assignmentExpression
 *     conditionalExpression
 *     logicalExpression
 *     additiveExpression
 *
 * because doing so would create duplicate expression ownership and circular
 * parser dependencies.
 *
 * The canonical integration point is therefore a parser-rule alias.
 *
 * In the composed expression grammar, `rangeEndpointExpression` must resolve
 * to the expression level that is valid as a range endpoint.
 *
 * ============================================================================
 *
 * IMPORTANT:
 *
 * Do not add a second complete expression grammar here.
 *
 * ============================================================================
 */

rangeEndpointExpression
    : expression
    ;


/*
 * ============================================================================
 * RANGE MODIFIERS
 * ============================================================================
 *
 * Range modifiers refine iteration/domain semantics without changing the
 * endpoint grammar.
 *
 * Supported forms:
 *
 *     step
 *     count
 *
 * Additional future modifiers should be introduced as explicit language
 * constructs or dialect extensions rather than overloading existing syntax.
 * ============================================================================
 */

rangeModifier
    : rangeStep
    | rangeCount
    ;


/*
 * ============================================================================
 * STEP
 * ============================================================================
 *
 * Canonical conceptual syntax:
 *
 *     range step expression
 *
 * Examples:
 *
 *     0..10 step 2
 *     start..end step stride
 *     0..<n step block
 *
 * The `step` keyword/token must be owned by the canonical lexer.
 *
 * ============================================================================
 */

rangeStep
    : STEP
      rangeModifierExpression
    ;


/*
 * ============================================================================
 * COUNT
 * ============================================================================
 *
 * Canonical conceptual syntax:
 *
 *     range count expression
 *
 * This represents a requested cardinality rather than a physical allocation.
 *
 * Example:
 *
 *     start..end count n
 *
 * Semantic analysis determines whether that form is meaningful for the
 * selected range.
 * ============================================================================
 */

rangeCount
    : COUNT
      rangeModifierExpression
    ;


/*
 * ============================================================================
 * MODIFIER EXPRESSION
 * ============================================================================
 *
 * Step and count values are expressions.
 *
 * This allows:
 *
 *     step 1
 *     step stride
 *     step (base + delta)
 *     count n
 *     count partitions
 *
 * without introducing artificial numeric restrictions.
 * ============================================================================
 */

rangeModifierExpression
    : rangeEndpointExpression
    ;


/*
 * ============================================================================
 * EXPLICIT BOUNDED RANGE
 * ============================================================================
 *
 * This convenience rule gives downstream grammar composition a stable name for
 * ranges where both endpoints are syntactically present.
 *
 * Examples:
 *
 *     a..b
 *     a..<b
 *
 * ============================================================================
 */

boundedRangeExpression
    : rangeEndpoint
      rangeOperator
      rangeEndpoint
      rangeModifier*
    ;


/*
 * ============================================================================
 * LOWER-BOUNDED RANGE
 * ============================================================================
 *
 * Examples:
 *
 *     a..
 *     a.. step s
 *     a.. count n
 *
 * The missing upper endpoint is represented syntactically by absence.
 *
 * The semantic layer decides whether the range is finite or infinite.
 * ============================================================================
 */

lowerBoundedRangeExpression
    : rangeEndpoint
      rangeOperator
      rangeModifier*
    ;


/*
 * ============================================================================
 * UPPER-BOUNDED RANGE
 * ============================================================================
 *
 * Examples:
 *
 *     ..b
 *     ..<b
 *     ..b step s
 *
 * ============================================================================
 */

upperBoundedRangeExpression
    : rangeOperator
      rangeEndpoint
      rangeModifier*
    ;


/*
 * ============================================================================
 * UNBOUNDED RANGE
 * ============================================================================
 *
 * Example:
 *
 *     ..
 *
 * Whether a completely unbounded range is legal in a particular syntactic
 * context is a semantic/contextual decision.
 *
 * The grammar preserves the construct without assigning it a runtime meaning.
 * ============================================================================
 */

unboundedRangeExpression
    : rangeOperator
      rangeModifier*
    ;


/*
 * ============================================================================
 * RANGE WITH STEP
 * ============================================================================
 *
 * Convenience rule for downstream parser composition.
 *
 * ============================================================================
 */

steppedRangeExpression
    : rangeCore
      rangeStep
    ;


/*
 * ============================================================================
 * RANGE WITH COUNT
 * ============================================================================
 *
 * Convenience rule for downstream parser composition.
 *
 * ============================================================================
 */

countedRangeExpression
    : rangeCore
      rangeCount
    ;


/*
 * ============================================================================
 * RANGE WITH BOTH STEP AND COUNT
 * ============================================================================
 *
 * Modifier order is intentionally syntactically flexible:
 *
 *     start..end step s count n
 *     start..end count n step s
 *
 * Semantic validation determines whether both modifiers are meaningful and
 * whether their combination is valid.
 *
 * ============================================================================
 */

parameterizedRangeExpression
    : rangeCore
      rangeModifier
      rangeModifier
      rangeModifier*
    ;


/*
 * ============================================================================
 * RANGE DOMAIN EXPRESSION
 * ============================================================================
 *
 * This stable rule is intended for constructs such as:
 *
 *     for i in 0..n
 *     foreach q in qubits
 *     forall i in domain
 *
 * It does NOT require the source to be a concrete materialized collection.
 *
 * ============================================================================
 */

rangeDomainExpression
    : rangeExpression
    ;


/*
 * ============================================================================
 * RANGE PATTERN
 * ============================================================================
 *
 * This parser-level form is useful to pattern/matching infrastructure without
 * introducing a second range syntax.
 *
 * ============================================================================
 */

rangePattern
    : rangeExpression
    ;


/*
 * ============================================================================
 * RANGE TYPE-AGNOSTICITY
 * ============================================================================
 *
 * The following are intentionally all syntactically representable:
 *
 *     0..n
 *     0.0..1.0
 *     start..end
 *     q0..qN
 *     row..rows
 *     time0..time1
 *     address0..address1
 *     symbolic_start..symbolic_end
 *
 * Semantic/type checking determines validity.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * The canonical expression parser should import this grammar:
 *
 *     import Ranges;
 *
 * It should delegate range syntax to:
 *
 *     rangeExpression
 *
 * and MUST NOT independently redefine:
 *
 *     rangeExpression
 *     rangeOperator
 *     rangeStep
 *     rangeCount
 *
 * ============================================================================
 *
 * RANGE PRECEDENCE
 * ============================================================================
 *
 * Range expressions must sit above arithmetic endpoint expressions.
 *
 * Conceptually:
 *
 *     ...
 *       relational
 *           |
 *       shift
 *           |
 *       additive
 *           |
 *       multiplicative
 *           |
 *       exponent
 *           |
 *       unary
 *           |
 *       postfix
 *           |
 *       primary
 *           |
 *       range composition
 *
 * The exact position in the canonical expression hierarchy must be fixed by
 * `expressions.g4` so this file does not create a second precedence graph.
 *
 * ============================================================================
 *
 * Examples that must remain structurally unambiguous:
 *
 *     0..n
 *     0..n + 1
 *     0..(n + 1)
 *     start..end step stride
 *     (start + offset)..(end + offset)
 *
 * Endpoint grouping belongs to the ordinary expression grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * AST INTEGRATION CONTRACT
 * ============================================================================
 *
 * Frontend AST construction must map:
 *
 *     rangeExpression
 *
 * into the repository's existing range-expression representation.
 *
 * The existing frontend already contains range-expression builder support.
 * Therefore this grammar must feed that existing representation rather than
 * introduce a grammar-specific range AST.
 *
 * The AST must retain:
 *
 *     start endpoint;
 *     end endpoint;
 *     endpoint openness/inclusivity;
 *     step;
 *     count;
 *     source span.
 *
 * Absent endpoints MUST remain absent.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SEMANTIC INTEGRATION CONTRACT
 * ============================================================================
 *
 * Semantic analysis consumes the AST and determines:
 *
 *     endpoint compatibility;
 *     step compatibility;
 *     count compatibility;
 *     range direction;
 *     finiteness;
 *     cardinality;
 *     zero-step validity;
 *     overflow behavior;
 *     symbolic validity;
 *     constant-evaluation opportunities.
 *
 * This grammar MUST NOT perform those operations.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * CLASSICAL IR INTEGRATION
 * ============================================================================
 *
 * A range may lower to:
 *
 *     iteration domain;
 *     slice;
 *     interval;
 *     index set;
 *     lazy sequence;
 *     symbolic domain.
 *
 * The grammar has no direct dependency on classical IR.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * QUANTUM IR INTEGRATION
 * ============================================================================
 *
 * A range may participate in quantum source constructs.
 *
 * Example conceptual source:
 *
 *     for q in logical_qubits {
 *         ...
 *     }
 *
 * or:
 *
 *     measure qubits[0..n]
 *
 * The range grammar does not create quantum operations.
 *
 * If the resulting semantic construct becomes a quantum operation, lowering
 * occurs through the canonical quantum frontend/IR path.
 *
 * There is no:
 *
 *     grammar -> quantum::ir
 *
 * dependency.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * HARDWARE / HDL INTEGRATION
 * ============================================================================
 *
 * Hardware-oriented users may use ranges to describe:
 *
 *     replicated structures;
 *     index domains;
 *     slices;
 *     generated instances;
 *     parameterized widths.
 *
 * This grammar remains unaware of the target.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * A range can describe a requested domain whose realization consumes resources.
 *
 * Resource analysis may determine:
 *
 *     memory requirements;
 *     parallelism;
 *     communication;
 *     execution cost;
 *     quantum resources;
 *     hardware resources.
 *
 * Those analyses are downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * DETERMINISM / REPRODUCIBILITY
 * ============================================================================
 *
 * No rule in this grammar may depend on runtime discovery.
 *
 * Identical source + identical language/lexer versions must produce identical
 * parse trees.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Required positive syntax cases include:
 *
 *     0..10
 *     0..<10
 *     start..end
 *     start..<end
 *     ..end
 *     ..<end
 *     start..
 *     ..
 *     0..10 step 2
 *     0..<10 step stride
 *     start..end count n
 *     start..end step stride count n
 *     start..end count n step stride
 *     (base + offset)..(limit + offset)
 *     symbolic_start..symbolic_end
 *
 * Required domain cases include:
 *
 *     classical indices
 *     vector ranges
 *     matrix ranges
 *     tensor ranges
 *     quantum indices
 *     logical-qubit ranges
 *     hardware-generation ranges
 *     distributed partition ranges
 *
 * ============================================================================
 *
 * Required negative syntax cases include:
 *
 *     malformed range operator
 *     incomplete range punctuation
 *     malformed step modifier
 *     malformed count modifier
 *     missing modifier expression
 *     malformed endpoint expression
 *     invalid token sequence around range punctuation
 *
 * ============================================================================
 *
 * Required semantic-negative cases are tested downstream, NOT by this grammar:
 *
 *     zero step
 *     incompatible endpoint types
 *     invalid count
 *     impossible direction
 *     overflow
 *     unsupported range type
 *
 * ============================================================================
 *
 * Required scalability tests:
 *
 *     tiny range
 *     symbolic range
 *     very large literal endpoint
 *     arbitrarily long range modifier expressions
 *     deeply nested endpoint expressions
 *     large source programs containing many ranges
 *
 * The tests must verify that the grammar itself does not introduce a finite
 * range-size or endpoint-size restriction.
 *
 * ============================================================================
 *
 * Required determinism tests:
 *
 *     identical source -> identical parse tree
 *
 * across repeated parser invocations.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file must contain NONE of:
 *
 *     numeric maximums;
 *     range cardinality constants;
 *     endpoint-size constants;
 *     machine-size constants;
 *     resource counts;
 *     hardware identifiers;
 *     device topology;
 *     fixed qubit counts;
 *     fixed CPU counts;
 *     fixed GPU counts;
 *     fixed FPGA counts;
 *     fixed memory sizes.
 *
 * Any such addition is an architectural violation unless it is purely a
 * syntactic token required by the language itself and is documented as such.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE only when:
 *
 * [ ] It compiles as an ANTLR parser grammar with the canonical lexer.
 *
 * [ ] It defines no lexer rules.
 *
 * [ ] It owns range syntax exclusively.
 *
 * [ ] No other expression grammar independently owns range syntax.
 *
 * [ ] `expressions.g4` imports/delegates to `Ranges`.
 *
 * [ ] Range endpoints use the canonical expression grammar.
 *
 * [ ] No artificial range-size limit exists.
 *
 * [ ] Open-ended ranges are represented without machine assumptions.
 *
 * [ ] Inclusive/exclusive endpoint intent is preserved.
 *
 * [ ] Step syntax is preserved without imposing step semantics.
 *
 * [ ] Count syntax is preserved without imposing allocation semantics.
 *
 * [ ] The existing frontend range AST is the downstream AST integration point.
 *
 * [ ] Semantic analysis, rather than grammar parsing, owns validity checks.
 *
 * [ ] Classical, quantum, HDL, hardware, distributed and accelerator use cases
 *     can consume range syntax without changing this grammar.
 *
 * [ ] No dependency on quantum::ir exists.
 *
 * [ ] No dependency on hardware discovery exists.
 *
 * [ ] No dependency on scheduling exists.
 *
 * [ ] No dependency on runtime state exists.
 *
 * [ ] Positive tests exist.
 *
 * [ ] Negative syntax tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Cross-domain tests exist.
 *
 * [ ] Determinism tests exist.
 *
 * [ ] Scalability tests exist.
 *
 * [ ] Rust integration remains compatible with Rust 1.97/1.97.1 and safe Rust.
 *
 * [ ] Documentation identifies this file as the authoritative range-syntax
 *     owner.
 *
 * ============================================================================
 */
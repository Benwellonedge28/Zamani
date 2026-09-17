/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/expressions/ranges.g4
 *
 * Status:
 *     Production modular range-expression grammar.
 *
 * Purpose:
 *     Own the reusable syntax of Zamani range expressions while delegating
 *     expression precedence, lexical tokenization, typing, semantics, AST
 *     construction, IR lowering, and target realization to their canonical
 *     owners.
 *
 * ============================================================================
 * ARCHITECTURAL AUTHORITY
 * ============================================================================
 *
 * Language specification
 *        |
 *        v
 * Modular grammar components
 *        |
 *        v
 * grammar/expressions/expression.g4
 *        |
 *        v
 * grammar/Zamani.g4
 *        |
 *        v
 * Lexer / parser implementation
 *        |
 *        v
 * Domain-neutral frontend AST
 *        |
 *        v
 * Semantic analysis
 *        |
 *        +------------------+------------------+
 *        |                  |                  |
 *        v                  v                  v
 *   Classical IR       quantum::ir       HDL/Hardware IR
 *        |                  |                  |
 *        +------------------+------------------+
 *                           |
 *                           v
 *                  optimization/lowering
 *                           |
 *                    routing/scheduling
 *                           |
 *                    resilience/QEC/ZQN
 *                           |
 *                           v
 *                          HAL
 *                           |
 *                           v
 *                    target realization
 *
 * This file has NO dependency on:
 *
 *     quantum::ir
 *     hardware
 *     HAL
 *     routing
 *     scheduling
 *     QEC
 *     ZQN
 *     runtime state
 *     target selection
 *
 * ============================================================================
 * FILE OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - range-expression syntax;
 *     - range operators;
 *     - optional lower endpoint;
 *     - optional upper endpoint;
 *     - inclusive/exclusive upper-bound syntax;
 *     - open-start syntax;
 *     - open-end syntax;
 *     - completely open range syntax;
 *     - composition of a range with the canonical lower expression layer.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer rules;
 *     - token spelling;
 *     - identifiers;
 *     - numeric literals;
 *     - arithmetic;
 *     - comparison;
 *     - logical operators;
 *     - assignment;
 *     - calls;
 *     - indexing;
 *     - statements;
 *     - loops;
 *     - iteration;
 *     - collection materialization;
 *     - range cardinality;
 *     - overflow;
 *     - type checking;
 *     - endpoint ordering;
 *     - resource allocation;
 *     - hardware mapping;
 *     - physical qubit selection;
 *     - QEC;
 *     - ZQN;
 *     - scheduling;
 *     - runtime execution.
 *
 * ============================================================================
 * CANONICAL LEXER CONTRACT
 * ============================================================================
 *
 * The repository's canonical lexer owns range punctuation.
 *
 * Canonical tokens:
 *
 *     DOT_DOT
 *         source spelling: ..
 *
 *     DOT_DOT_EQ
 *         source spelling: ..=
 *
 * This grammar MUST NOT redefine those tokens.
 *
 * In particular, this file MUST NOT introduce:
 *
 *     RANGE
 *     RANGE_INCLUSIVE
 *     RANGE_EXCLUSIVE
 *     STEP
 *     COUNT
 *
 * as alternative lexical vocabulary.
 *
 * The existing lexer already establishes:
 *
 *     DOT_DOT_EQ
 *     DOT_DOT
 *
 * and the modular grammar must consume those tokens directly.
 *
 * ============================================================================
 * RANGE MODEL
 * ============================================================================
 *
 * Supported syntactic forms:
 *
 *     start .. end
 *     start ..= end
 *
 *     start ..
 *     start ..=
 *
 *     .. end
 *     ..= end
 *
 *     ..
 *     ..=
 *
 * The grammar preserves endpoint presence/absence.
 *
 * It MUST NOT replace an omitted endpoint with:
 *
 *     0
 *     1
 *     MIN
 *     MAX
 *     infinity
 *     machine_word_max
 *     type_max
 *     any target-specific value.
 *
 * The semantic/type layer decides what an omitted endpoint means in context.
 *
 * ============================================================================
 * BOUNDARY MEANING
 * ============================================================================
 *
 *     start .. end
 *
 * has an exclusive upper boundary.
 *
 *     start ..= end
 *
 * has an inclusive upper boundary.
 *
 * The lower boundary is represented by the presence or absence of the lower
 * endpoint. No separate lower-inclusive/lower-exclusive token is invented.
 *
 * ============================================================================
 * SCALABILITY / POCO-REAF
 * ============================================================================
 *
 * No artificial resource limit is encoded here.
 *
 * This grammar does NOT define:
 *
 *     MAX_RANGE_LENGTH
 *     MAX_RANGE_VALUE
 *     MAX_INDEX
 *     MAX_ITERATIONS
 *     MAX_ELEMENTS
 *     MAX_DIMENSIONS
 *     MAX_INTEGER_BITS
 *     MAX_TENSOR_SIZE
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *
 * A range is a compact source-level description of a domain.
 *
 * For example:
 *
 *     0 .. n
 *
 * may ultimately be represented as:
 *
 *     - a lazy range;
 *     - an iteration domain;
 *     - a slice;
 *     - a tensor domain;
 *     - a distributed partition;
 *     - an accelerator launch domain;
 *     - an HDL generation domain;
 *     - a quantum index domain;
 *     - a symbolic mathematical interval.
 *
 * That decision belongs downstream.
 *
 * "Infinity" therefore means that this grammar introduces no artificial
 * language-level finite resource limit. Actual compiler/runtime limits remain
 * implementation and resource-policy concerns.
 *
 * ============================================================================
 * EXPRESSION PRECEDENCE CONTRACT
 * ============================================================================
 *
 * The canonical expression hierarchy in expression.g4 is:
 *
 *     expression
 *       |
 *       v
 *     assignmentExpression
 *       |
 *       v
 *     conditionalExpression
 *       |
 *       v
 *     rangeExpression
 *       |
 *       v
 *     logicalOrExpression
 *       |
 *       v
 *     logicalAndExpression
 *       |
 *       v
 *     bitwise...
 *       |
 *       v
 *     comparison...
 *       |
 *       v
 *     arithmetic...
 *       |
 *       v
 *     prefix
 *       |
 *       v
 *     postfix
 *       |
 *       v
 *     primary
 *
 * This file MUST NOT recreate that precedence graph.
 *
 * Range endpoints therefore consume:
 *
 *     logicalOrExpression
 *
 * rather than `expression`.
 *
 * This is important.
 *
 * Using `expression` as the endpoint would re-enter assignment/conditional/
 * range parsing and create a competing expression boundary.
 *
 * ============================================================================
 * PUBLIC COMPOSITION CONTRACT
 * ============================================================================
 *
 * The canonical expression grammar imports this grammar and owns the public
 * `expression` entry point.
 *
 * Conceptually:
 *
 *     expression.g4
 *         |
 *         +--> conditionalExpression
 *                 |
 *                 +--> rangeExpression
 *                         |
 *                         +--> lower expression layer
 *
 * This file provides the range-specific rules only.
 *
 * ============================================================================
 */

parser grammar Ranges;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * PUBLIC RANGE RULE
 * ============================================================================
 *
 * This is the single public range rule supplied by this grammar.
 *
 * The surrounding expression grammar should delegate to this rule.
 *
 * ============================================================================
 */

rangeExpression
    : rangeStart? rangeOperator rangeEnd?
    ;


/*
 * ============================================================================
 * RANGE START
 * ============================================================================
 *
 * The lower endpoint is optional.
 *
 * Examples:
 *
 *     0
 *     start
 *     index + offset
 *
 * The endpoint is deliberately the lower expression layer rather than the
 * complete `expression` rule.
 *
 * ============================================================================
 */

rangeStart
    : logicalOrExpression
    ;


/*
 * ============================================================================
 * RANGE END
 * ============================================================================
 *
 * The upper endpoint is optional.
 *
 * Examples:
 *
 *     10
 *     end
 *     limit + offset
 *
 * As with the lower endpoint, this consumes the canonical lower expression
 * layer and therefore cannot accidentally consume a second range operator.
 *
 * ============================================================================
 */

rangeEnd
    : logicalOrExpression
    ;


/*
 * ============================================================================
 * RANGE OPERATOR
 * ============================================================================
 *
 * Canonical lexical ownership:
 *
 *     DOT_DOT
 *     DOT_DOT_EQ
 *
 * No literal punctuation is defined here.
 *
 * ============================================================================
 */

rangeOperator
    : DOT_DOT
    | DOT_DOT_EQ
    ;


/*
 * ============================================================================
 * EXPLICIT RANGE FORMS
 * ============================================================================
 *
 * These named rules are integration aliases.
 *
 * They do not introduce additional syntax.
 *
 * They provide stable parser-rule names for downstream grammar consumers and
 * tests without duplicating the public range grammar.
 * ============================================================================
 */


/*
 * Both endpoints are explicitly present.
 *
 *     start .. end
 *     start ..= end
 */
boundedRangeExpression
    : rangeStart
      rangeOperator
      rangeEnd
    ;


/*
 * Lower endpoint is present and upper endpoint is omitted.
 *
 *     start ..
 *     start ..=
 */
lowerBoundedRangeExpression
    : rangeStart
      rangeOperator
    ;


/*
 * Lower endpoint is omitted and upper endpoint is present.
 *
 *     .. end
 *     ..= end
 */
upperBoundedRangeExpression
    : rangeOperator
      rangeEnd
    ;


/*
 * Neither endpoint is present.
 *
 *     ..
 *     ..=
 *
 * Whether this is legal as a standalone value is a semantic/contextual
 * decision. The parser preserves the syntax rather than inventing a bound.
 */
unboundedRangeExpression
    : rangeOperator
    ;


/*
 * ============================================================================
 * RANGE DOMAIN ALIAS
 * ============================================================================
 *
 * Consumers such as:
 *
 *     for
 *     indexing
 *     slicing
 *     tensor operations
 *     quantum selection
 *     HDL generation
 *     distributed partitioning
 *
 * may explicitly reference the generic range expression.
 *
 * This is an alias only. It does not introduce a second range syntax.
 * ============================================================================
 */

rangeDomainExpression
    : rangeExpression
    ;


/*
 * ============================================================================
 * RANGE PATTERN ALIAS
 * ============================================================================
 *
 * Pattern/match infrastructure may consume the same range representation.
 *
 * No second range grammar is created.
 * ============================================================================
 */

rangePattern
    : rangeExpression
    ;


/*
 * ============================================================================
 * SEMANTIC BOUNDARY
 * ============================================================================
 *
 * This grammar deliberately does NOT decide:
 *
 *     - whether endpoints have compatible types;
 *     - whether a range is finite;
 *     - whether it is empty;
 *     - whether ordering is valid;
 *     - whether an endpoint overflows;
 *     - whether an omitted endpoint is legal;
 *     - whether the range is materialized;
 *     - whether the range is lazy;
 *     - whether it is sequential;
 *     - whether it is parallel;
 *     - whether it is distributed;
 *     - whether it is mapped to an accelerator;
 *     - whether it represents quantum indices;
 *     - whether it represents physical qubits;
 *     - whether it is valid for HDL generation.
 *
 * Those decisions belong to semantic/type/resource analysis.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser must provide enough structure for the existing domain-neutral
 * frontend AST to preserve:
 *
 *     - lower endpoint presence;
 *     - upper endpoint presence;
 *     - lower endpoint expression;
 *     - upper endpoint expression;
 *     - exclusive/inclusive operator;
 *     - operator source span;
 *     - endpoint source spans;
 *     - complete range source span.
 *
 * The frontend AST must remain generic.
 *
 * It MUST NOT create:
 *
 *     QuantumRange
 *     TensorRange
 *     HardwareRange
 *     PhysicalQubitRange
 *     CpuRange
 *     GpuRange
 *     FpgaRange
 *
 * or similar domain-specific parser nodes.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * A range may participate in a quantum source construct, for example:
 *
 *     q[0 .. n]
 *
 * or:
 *
 *     measure q[start ..= end]
 *
 * The range grammar does not select physical qubits.
 *
 * The semantic pipeline remains:
 *
 *     source
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic quantum model
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     routing
 *       |
 *       v
 *     scheduling
 *       |
 *       v
 *     QEC / resilience / ZQN
 *       |
 *       v
 *     HAL
 *       |
 *       v
 *     physical target
 *
 * This grammar creates no quantum IR.
 *
 * ============================================================================
 * CLASSICAL / DATA / AI INTEGRATION
 * ============================================================================
 *
 * The same generic range syntax can describe semantic domains used for:
 *
 *     arrays
 *     vectors
 *     matrices
 *     tensors
 *     datasets
 *     streams
 *     partitions
 *     scientific domains
 *     numerical domains
 *     accelerator domains
 *
 * No fixed dimensionality, element count, vector width, or accelerator count
 * is encoded here.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * The same syntax may be consumed by HDL/hardware grammar constructs for:
 *
 *     parameterized widths
 *     generated instances
 *     address domains
 *     memory indices
 *     pipeline indices
 *     replicated structures
 *
 * This file does not determine actual hardware width or implementation.
 *
 * ============================================================================
 * DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * A range can describe an abstract domain used for:
 *
 *     partitioning
 *     sharding
 *     task generation
 *     distributed indexing
 *     collective domains
 *
 * It does not establish a fixed node count or physical placement.
 *
 * ============================================================================
 * RESOURCE / PORTABILITY CONTRACT
 * ============================================================================
 *
 * Requirements such as:
 *
 *     requires a capability
 *
 * and target decisions such as:
 *
 *     place on device X
 *
 * are not range syntax.
 *
 * A range describes a source-level domain.
 *
 * Resource analysis determines whether and how that domain can be realized
 * on the available target.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing this grammar must depend only on:
 *
 *     - token sequence;
 *     - grammar version;
 *     - parser configuration.
 *
 * It must never depend on:
 *
 *     - CPU count;
 *     - GPU count;
 *     - QPU availability;
 *     - memory capacity;
 *     - network state;
 *     - runtime scheduler state;
 *     - calibration state;
 *     - random state.
 *
 * ============================================================================
 * ERROR BOUNDARY
 * ============================================================================
 *
 * Syntax errors:
 *
 *     malformed range punctuation;
 *     malformed endpoint token sequence;
 *     malformed surrounding expression.
 *
 * Semantic errors:
 *
 *     incompatible endpoint types;
 *     invalid ordering;
 *     unsupported rangeable type;
 *     overflow;
 *     invalid use of an unbounded range;
 *     impossible materialization;
 *     unavailable resources.
 *
 * Semantic errors MUST be reported downstream.
 *
 * ============================================================================
 * STEPS / STRIDES
 * ============================================================================
 *
 * This file deliberately does NOT introduce a `step` or `stride` grammar.
 *
 * The repository's existing canonical range contract defines `..` and `..=`
 * only. Adding:
 *
 *     start .. end step stride
 *
 * here without first establishing a language-wide lexical/specification/AST/
 * semantic contract would create an incomplete feature and force later
 * re-editing.
 *
 * If stepped ranges are eventually standardized, they require a coordinated
 * contract across:
 *
 *     specification
 *     lexer
 *     expression grammar
 *     AST
 *     semantic analysis
 *     diagnostics
 *     IR
 *     compiler
 *     runtime
 *     compatibility
 *     tests
 *
 * This file therefore remains independently complete for the range feature
 * that the repository currently defines.
 *
 * ============================================================================
 * NO HARD-CODING AUDIT
 * ============================================================================
 *
 * No resource or hardware limit appears in this grammar.
 *
 * No fixed:
 *
 *     qubit count
 *     CPU count
 *     GPU count
 *     FPGA count
 *     node count
 *     memory size
 *     tensor dimension
 *     vector width
 *     range cardinality
 *     endpoint magnitude
 *
 * is encoded.
 *
 * ============================================================================
 * RUST INTEGRATION
 * ============================================================================
 *
 * This file contains no Rust code.
 *
 * Generated-parser consumers must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and safe Rust only.
 *
 * The grammar itself introduces no requirement for `unsafe`.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Required positive cases:
 *
 *     0 .. 10
 *     0 ..= 10
 *     start .. end
 *     start ..= end
 *     start ..
 *     start ..=
 *     .. end
 *     ..= end
 *     ..
 *     ..=
 *     (base + offset) .. (limit + offset)
 *     symbolic_start .. symbolic_end
 *
 * Required composition cases:
 *
 *     x = 0 .. n
 *     array[0 .. n]
 *     tensor[start ..= end]
 *     quantum_selection[0 .. qubit_count]
 *     generated_block[0 .. width]
 *
 * Required semantic-negative cases belong downstream:
 *
 *     incompatible endpoint types
 *     invalid ordering
 *     unsupported unbounded range
 *     overflow
 *     impossible materialization
 *     unavailable resources
 *
 * Required scalability cases:
 *
 *     small endpoints
 *     large representable endpoints
 *     symbolic endpoints
 *     deeply composed endpoint expressions
 *     many range expressions in one source unit
 *
 * Tests MUST NOT establish an artificial maximum range size.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [ ] It consumes only canonical lexer tokens.
 *     [ ] It uses DOT_DOT and DOT_DOT_EQ.
 *     [ ] It defines no lexer rules.
 *     [ ] It does not define `expression`.
 *     [ ] It does not reference the complete `expression` rule for endpoints.
 *     [ ] It consumes logicalOrExpression as its endpoint boundary.
 *     [ ] It defines exactly one canonical range syntax.
 *     [ ] It supports bounded ranges.
 *     [ ] It supports open-start ranges.
 *     [ ] It supports open-end ranges.
 *     [ ] It supports fully open ranges.
 *     [ ] It preserves inclusive/exclusive intent.
 *     [ ] It imposes no machine/resource limits.
 *     [ ] It does not encode iteration semantics.
 *     [ ] It does not encode step/stride semantics.
 *     [ ] It does not depend on quantum::ir.
 *     [ ] It does not depend on hardware.
 *     [ ] It does not depend on scheduling.
 *     [ ] It preserves the existing domain-neutral AST boundary.
 *     [ ] It has defined downstream semantic ownership.
 *     [ ] It has defined classical/quantum/HDL/resource integration.
 *     [ ] It has positive tests.
 *     [ ] It has negative tests.
 *     [ ] It has boundary tests.
 *     [ ] It has scalability tests.
 *     [ ] It has compatibility tests.
 *     [ ] It has deterministic parsing.
 *     [ ] It requires no unsafe Rust.
 *
 * ============================================================================
 */
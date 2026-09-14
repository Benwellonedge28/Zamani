/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/resources/latency.g4
 *
 * Grammar kind:
 *     ANTLR4 parser grammar
 *
 * Target:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL LATENCY RESOURCE GRAMMAR for Zamani.
 *
 * Latency is an abstract resource/performance property.
 *
 * This grammar allows Zamani source code to express latency intent without
 * encoding:
 *
 *     - a particular machine;
 *     - a particular processor;
 *     - a particular device;
 *     - a particular clock;
 *     - a particular topology;
 *     - a particular scheduler;
 *     - a particular hardware timing resolution;
 *     - a particular runtime;
 *     - a fixed maximum latency;
 *     - a fixed minimum latency;
 *     - a fixed number of operations;
 *     - a fixed machine size.
 *
 * The grammar describes LATENCY INTENT.
 *
 * Semantic analysis determines what that intent means in its enclosing
 * resource construct.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     parser
 *          |
 *          +------------------------------+
 *          |                              |
 *          v                              v
 *     general expressions          resource grammar
 *                                         |
 *                                         v
 *                                  latency grammar
 *                                         |
 *                                         v
 *                                  frontend AST
 *                                         |
 *                                         v
 *                              semantic resource model
 *                                         |
 *                       +-----------------+------------------+
 *                       |                 |                  |
 *                       v                 v                  v
 *                   compiler          scheduler           runtime
 *                       |                 |                  |
 *                       +-----------------+------------------+
 *                                         |
 *                                         v
 *                                  target realization
 *
 * This file is syntax only.
 *
 * It does not perform:
 *
 *     - latency measurement;
 *     - latency prediction;
 *     - scheduling;
 *     - hardware discovery;
 *     - resource allocation;
 *     - optimization;
 *     - routing;
 *     - runtime dispatch;
 *     - backend selection.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - latency clause syntax;
 *     - latency expression composition;
 *     - the reusable latency grammar boundary;
 *     - latency-specific parser entry points;
 *     - integration of latency with canonical resource expressions.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - identifiers;
 *     - numeric literals;
 *     - duration literal spelling;
 *     - general expressions;
 *     - arithmetic;
 *     - comparison precedence;
 *     - units;
 *     - duration semantics;
 *     - resource declarations;
 *     - resource allocation;
 *     - hardware discovery;
 *     - hardware timing;
 *     - scheduling algorithms;
 *     - scheduling policy;
 *     - optimization;
 *     - routing;
 *     - quantum::ir;
 *     - classical IR;
 *     - QEC;
 *     - ZQN;
 *     - runtime implementation;
 *     - simulator implementation;
 *     - deployment implementation.
 *
 * ============================================================================
 * NON-DUPLICATION CONTRACT
 * ============================================================================
 *
 * Latency MUST use the canonical resource-expression architecture.
 *
 * This file MUST NOT redefine:
 *
 *     expression
 *     assignmentExpression
 *     binaryExpression
 *     logicalOrExpression
 *     logicalAndExpression
 *     equalityExpression
 *     relationalExpression
 *     additiveExpression
 *     multiplicativeExpression
 *     unaryExpression
 *     primaryExpression
 *
 * It also MUST NOT redefine:
 *
 *     duration literals
 *     numeric literals
 *     identifiers
 *     operators
 *
 * Those concepts already have canonical owners elsewhere in grammar/.
 *
 * ============================================================================
 * RESOURCE ARCHITECTURE
 * ============================================================================
 *
 * Latency is one property in the universal resource model.
 *
 * It must remain distinct from:
 *
 *     throughput
 *     bandwidth
 *     energy
 *     power
 *     reliability
 *     resilience
 *     cost
 *     capacity
 *     availability
 *
 * For example:
 *
 *     latency = 10ns
 *
 * does not mean:
 *
 *     throughput = 10ns
 *
 * and:
 *
 *     latency = 10ns
 *
 * does not imply:
 *
 *     hardware supports a 10 ns clock.
 *
 * The enclosing semantic context determines the meaning.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * This file MUST preserve:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * Latency expressions may therefore depend on symbolic values:
 *
 *     latency = latency_budget;
 *
 *     latency = workload_latency;
 *
 *     latency = problem_size * latency_per_element;
 *
 *     latency = available_latency;
 *
 *     latency = 10ns;
 *
 * The grammar imposes no upper bound on:
 *
 *     - expression size;
 *     - numeric magnitude;
 *     - duration magnitude;
 *     - number of resource clauses;
 *     - number of resource declarations;
 *     - number of nested resource constructs.
 *
 * Actual limits are determined by available compilation/execution resources
 * and by semantic/resource feasibility rules.
 *
 * ============================================================================
 * HARD-CODING POLICY
 * ============================================================================
 *
 * This file MUST NOT contain:
 *
 *     MAX_LATENCY
 *     MIN_LATENCY
 *     MAX_DURATION
 *     MAX_RESOURCE_LATENCY
 *     MAX_OPERATIONS
 *     MAX_DEVICES
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_NODES
 *
 * Nor may it encode:
 *
 *     a device ID;
 *     a processor ID;
 *     a topology;
 *     a clock frequency;
 *     a scheduler;
 *     a backend;
 *     a provider;
 *     a hardware generation.
 *
 * ============================================================================
 * LATENCY SEMANTIC MODEL
 * ============================================================================
 *
 * The grammar intentionally distinguishes syntax from semantics.
 *
 * A latency clause provides a value expression.
 *
 * For example:
 *
 *     latency = 10ns;
 *
 * The parser recognizes the construct.
 *
 * Semantic analysis subsequently determines whether the surrounding context
 * means:
 *
 *     required latency;
 *     maximum permitted latency;
 *     minimum permitted latency;
 *     preferred latency;
 *     estimated latency;
 *     measured latency;
 *     target latency;
 *     latency budget;
 *     latency capability;
 *
 * This distinction MUST NOT be guessed by the grammar.
 *
 * The surrounding resource construct supplies the semantic role.
 *
 * ============================================================================
 * DURATION INTEGRATION
 * ============================================================================
 *
 * Duration literals are owned by:
 *
 *     grammar/lexer/duration-literals.g4
 *
 * That grammar provides lexical forms such as:
 *
 *     1fs
 *     1ps
 *     1ns
 *     1us
 *     1µs
 *     1μs
 *     1ms
 *     1s
 *     1min
 *     1h
 *     1d
 *     1wk
 *
 * This file MUST NOT reproduce those lexer rules.
 *
 * A latency expression may consume a duration literal through the canonical
 * expression grammar.
 *
 * ============================================================================
 * SYMBOLIC LATENCY
 * ============================================================================
 *
 * Latency does not have to be a literal.
 *
 * Valid semantic examples include:
 *
 *     latency = latency_budget;
 *
 *     latency = input_size * latency_per_item;
 *
 *     latency = estimated_latency;
 *
 *     latency = execution_context.latency;
 *
 *     latency = resource_latency;
 *
 * The grammar does not evaluate these expressions.
 *
 * ============================================================================
 * COMPOSITION
 * ============================================================================
 *
 * Latency may participate in canonical expressions when the type/semantic
 * system permits the operation.
 *
 * Examples:
 *
 *     latency = base_latency + communication_latency;
 *
 *     latency = operation_latency * operation_count;
 *
 *     latency = total_latency / parallelism;
 *
 *     latency = max_latency;
 *
 * Whether an operation is dimensionally and semantically valid is determined
 * by type/resource analysis.
 *
 * ============================================================================
 * UNIT / DIMENSION CONTRACT
 * ============================================================================
 *
 * This grammar does not establish a unit algebra.
 *
 * For example:
 *
 *     10ns
 *
 * is syntactically accepted as a duration literal.
 *
 * Whether:
 *
 *     10ns + 2ms
 *
 * is valid, and how it is normalized, belongs to semantic analysis.
 *
 * Likewise:
 *
 *     10ns * 4
 *
 * may be meaningful depending on the canonical duration/type system.
 *
 * This file must never perform such evaluation.
 *
 * ============================================================================
 * PRECISION CONTRACT
 * ============================================================================
 *
 * The grammar must not silently round latency values.
 *
 * If the lexical representation contains more precision than a selected
 * semantic representation can preserve, the semantic layer must issue the
 * appropriate deterministic diagnostic.
 *
 * A representational limit is not a grammar-level scalability limit.
 *
 * ============================================================================
 * OVERFLOW CONTRACT
 * ============================================================================
 *
 * Syntactically valid latency expressions must not be rejected merely because
 * a particular downstream numeric representation has bounded capacity.
 *
 * For example, the grammar must remain capable of parsing a syntactically
 * valid expression whose eventual semantic magnitude exceeds a particular
 * implementation's representation.
 *
 * Semantic analysis decides whether the selected representation can encode
 * the value.
 *
 * ============================================================================
 * NEGATIVE LATENCY
 * ============================================================================
 *
 * This grammar does not introduce a special negative-latency literal.
 *
 * The expression layer owns unary operators.
 *
 * Therefore:
 *
 *     latency = -10ns;
 *
 * is parsed as:
 *
 *     latency
 *         =
 *     unary(-, duration(10ns))
 *
 * Whether a negative latency is semantically legal is a resource/type
 * validation question.
 *
 * The grammar must not silently reinterpret negative values.
 *
 * ============================================================================
 * ZERO LATENCY
 * ============================================================================
 *
 * Zero latency is syntactically representable.
 *
 * Example:
 *
 *     latency = 0ns;
 *
 * Whether zero is semantically valid depends on the enclosing construct.
 *
 * The grammar must not impose a minimum positive latency.
 *
 * ============================================================================
 * LATENCY AND HARDWARE
 * ============================================================================
 *
 * Hardware grammars may consume latency expressions.
 *
 * For example:
 *
 *     latency = 10ns;
 *
 * may eventually be interpreted as a hardware capability or requirement.
 *
 * However, this grammar does not select:
 *
 *     CPU;
 *     GPU;
 *     FPGA;
 *     ASIC;
 *     QPU;
 *     accelerator;
 *     device;
 *     clock;
 *     topology.
 *
 * ============================================================================
 * LATENCY AND QUANTUM COMPUTING
 * ============================================================================
 *
 * Quantum source constructs may use latency expressions for concepts such as:
 *
 *     operation latency;
 *     measurement latency;
 *     reset latency;
 *     feed-forward latency;
 *     communication latency;
 *     execution latency;
 *     resource constraints.
 *
 * This grammar remains quantum-independent.
 *
 * Quantum semantic lowering may eventually produce canonical quantum::ir
 * structures, but latency.g4 MUST NOT create or modify quantum::ir.
 *
 * ============================================================================
 * LATENCY AND SCHEDULING
 * ============================================================================
 *
 * Scheduling consumes semantic timing information after parsing.
 *
 * This grammar does not implement:
 *
 *     ASAP scheduling;
 *     ALAP scheduling;
 *     list scheduling;
 *     critical-path scheduling;
 *     resource-constrained scheduling;
 *     alignment;
 *     delay insertion;
 *     dynamical decoupling;
 *     pulse scheduling.
 *
 * A latency expression becomes scheduling input only after semantic analysis.
 *
 * ============================================================================
 * LATENCY AND RUNTIME
 * ============================================================================
 *
 * Runtime may interpret latency information as:
 *
 *     timeout;
 *     service-level requirement;
 *     execution requirement;
 *     performance observation;
 *     admission condition;
 *     monitoring threshold.
 *
 * This grammar does not decide which interpretation applies.
 *
 * ============================================================================
 * LATENCY AND RESILIENCE
 * ============================================================================
 *
 * Resilience may observe latency as a telemetry or health signal.
 *
 * latency.g4 does not own:
 *
 *     retry policy;
 *     degradation policy;
 *     recovery;
 *     backend switching;
 *     incident diagnosis;
 *     mitigation.
 *
 * Those remain downstream responsibilities.
 *
 * ============================================================================
 * LATENCY AND NETWORKING
 * ============================================================================
 *
 * Networking constructs may consume latency expressions to describe abstract
 * communication characteristics.
 *
 * The grammar does not encode:
 *
 *     network topology;
 *     physical links;
 *     provider;
 *     route;
 *     node count;
 *     fixed network size.
 *
 * ============================================================================
 * LATENCY AND HDL
 * ============================================================================
 *
 * HDL constructs may use latency expressions where the language semantics
 * require explicit timing quantities.
 *
 * This file does not determine whether a latency is:
 *
 *     synthesizable;
 *     simulation-only;
 *     combinational;
 *     sequential;
 *     clock-relative;
 *     protocol-relative.
 *
 * HDL semantic layers own those decisions.
 *
 * ============================================================================
 * LATENCY AND DISTRIBUTED COMPUTING
 * ============================================================================
 *
 * Distributed programs may express latency requirements involving:
 *
 *     communication;
 *     coordination;
 *     service execution;
 *     data movement;
 *     synchronization.
 *
 * The grammar does not impose a finite number of nodes or links.
 *
 * ============================================================================
 * PUBLIC ENTRY POINTS
 * ============================================================================
 *
 * The primary reusable rule is:
 *
 *     latencyClause
 *
 * The primary expression boundary is:
 *
 *     latencyExpression
 *
 * A resource grammar can therefore consume:
 *
 *     latencyClause
 *
 * without duplicating latency syntax.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * 1. LEXER
 *
 * The canonical lexer supplies:
 *
 *     K_LATENCY
 *     ASSIGN
 *     SEMICOLON
 *
 * and the tokens consumed indirectly through resourceExpression.
 *
 * This file does not define lexer tokens.
 *
 * ---------------------------------------------------------------------------
 *
 * 2. RESOURCE EXPRESSIONS
 *
 * This grammar imports:
 *
 *     ResourceExpressions
 *
 * and delegates expression syntax to:
 *
 *     resourceExpression
 *
 * Therefore arithmetic, logical, comparison, calls, indexing, member access,
 * and future canonical expression extensions remain centralized.
 *
 * ---------------------------------------------------------------------------
 *
 * 3. RESOURCES
 *
 * grammar/resources/resources.g4 should import this grammar and delegate its
 * latency clause to:
 *
 *     latencyClause
 *
 * It must not retain an independent competing implementation of
 * resourceLatencyClause.
 *
 * Recommended integration:
 *
 *     import ResourceExpressions;
 *     import Latency;
 *
 * and:
 *
 *     resourceClause
 *         : ...
 *         | latencyClause
 *         | ...
 *         ;
 *
 * If the public rule name `resourceLatencyClause` is already consumed by
 * existing generated/parser code, resources.g4 should instead delegate through
 * a compatibility wrapper:
 *
 *     resourceLatencyClause
 *         : latencyClause
 *         ;
 *
 * That wrapper should be transitional and documented as an integration alias,
 * not a second implementation.
 *
 * ---------------------------------------------------------------------------
 *
 * 4. HARDWARE
 *
 * Hardware grammars may consume:
 *
 *     latencyExpression
 *
 * or:
 *
 *     latencyClause
 *
 * but must retain ownership of hardware-specific semantics.
 *
 * ---------------------------------------------------------------------------
 *
 * 5. QUANTUM
 *
 * Quantum resource grammars may consume:
 *
 *     latencyExpression
 *
 * for abstract timing/resource intent.
 *
 * They must not redefine duration or latency semantics.
 *
 * ---------------------------------------------------------------------------
 *
 * 6. HDL
 *
 * HDL grammars may consume:
 *
 *     latencyExpression
 *
 * where explicit timing quantities are part of Zamani HDL syntax.
 *
 * ---------------------------------------------------------------------------
 *
 * 7. SCHEDULING
 *
 * Scheduling consumes the semantic representation produced after parsing.
 *
 * It must not depend directly on this grammar file at runtime.
 *
 * The dependency is:
 *
 *     latency.g4
 *         ->
 *     parser AST
 *         ->
 *     semantic resource model
 *         ->
 *     scheduling
 *
 * ---------------------------------------------------------------------------
 *
 * 8. RUNTIME
 *
 * Runtime must not depend directly on grammar rules.
 *
 * It consumes canonical semantic/compiled representations.
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 * Allowed:
 *
 *     Latency
 *       |
 *       v
 *     ResourceExpressions
 *       |
 *       v
 *     Expressions
 *       |
 *       v
 *     AST / semantic analysis
 *
 * Not allowed:
 *
 *     Latency -> scheduler
 *     Latency -> runtime
 *     Latency -> hardware implementation
 *     Latency -> quantum::ir
 *     Latency -> QEC
 *     Latency -> ZQN
 *
 * The latter are downstream consumers of semantic information.
 *
 * ============================================================================
 * NO CIRCULAR DEPENDENCIES
 * ============================================================================
 *
 * This grammar MUST NOT import:
 *
 *     scheduling grammars;
 *     runtime grammars;
 *     hardware implementation grammars;
 *     quantum IR;
 *     QEC;
 *     ZQN.
 *
 * Hardware/quantum/HDL grammars may consume the latency abstraction, but
 * latency.g4 must remain independent of those domains.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * The following are intentionally unbounded by this grammar:
 *
 *     expression cardinality;
 *     resource clause cardinality;
 *     source program size;
 *     duration magnitude;
 *     symbolic expression complexity;
 *     namespace depth;
 *     number of latency declarations;
 *     number of resource groups;
 *     number of machines;
 *     number of processors;
 *     number of devices;
 *     number of qubits;
 *     number of nodes.
 *
 * Any actual implementation limit must come from:
 *
 *     available memory;
 *     available compute;
 *     parser implementation limits;
 *     compiler resource policy;
 *     runtime resource policy;
 *     target capability;
 *     deployment policy.
 *
 * Such limits must never be silently converted into grammar constants.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * For identical token streams, this grammar must produce deterministic parse
 * structure.
 *
 * The grammar contains:
 *
 *     no embedded Rust actions;
 *     no semantic predicates;
 *     no external state;
 *     no runtime callbacks;
 *     no hardware queries.
 *
 * ============================================================================
 * SAFETY CONTRACT
 * ============================================================================
 *
 * This grammar contains no Rust implementation code.
 *
 * Generated parser integration must compile under the repository's:
 *
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * safety policy.
 *
 * No unsafe Rust is required by this grammar.
 *
 * ============================================================================
 * VERSIONING CONTRACT
 * ============================================================================
 *
 * The syntax introduced here is part of the Zamani language surface.
 *
 * Changes to:
 *
 *     latencyClause
 *     latencyExpression
 *
 * must be treated as language compatibility changes.
 *
 * Adding new semantic latency interpretations must not require changing this
 * grammar if they can be represented by the existing canonical expression
 * form.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * This file must be tested independently through parser fixtures and through
 * the integrated resource parser.
 *
 * Positive examples:
 *
 *     latency = 0ns;
 *     latency = 1ns;
 *     latency = 250ps;
 *     latency = 1us;
 *     latency = 1µs;
 *     latency = 1μs;
 *     latency = 10ms;
 *     latency = 1s;
 *     latency = latency_budget;
 *     latency = workload_size * latency_per_item;
 *     latency = base_latency + communication_latency;
 *     latency = context.latency;
 *
 * Negative examples:
 *
 *     latency;
 *     latency = ;
 *     latency = ;
 *     latency = 10;
 *     latency = ns;
 *
 * The final two may be syntactically accepted by the general expression
 * grammar depending on Zamani's type system. If so, semantic validation must
 * reject them when a duration-valued latency is required. The parser itself
 * must not invent type rules.
 *
 * Boundary examples:
 *
 *     latency = 0ns;
 *
 *     latency = 999999999999999999999999999999999999999999999999ns;
 *
 *     latency = symbolic_latency_expression;
 *
 *     latency = expression_with_arbitrarily_many_terms;
 *
 * Cross-domain examples:
 *
 *     resource network {
 *         latency = network_latency;
 *     }
 *
 *     resource quantum {
 *         latency = measurement_latency;
 *     }
 *
 *     resource accelerator {
 *         latency = workload_size * per_item_latency;
 *     }
 *
 * Round-trip tests must preserve the semantic structure of the latency
 * expression.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * A production implementation of this file passes the hard-coding audit if:
 *
 *     [x] no fixed latency maximum exists;
 *     [x] no fixed latency minimum exists;
 *     [x] no fixed machine size exists;
 *     [x] no fixed device count exists;
 *     [x] no fixed processor count exists;
 *     [x] no fixed quantum size exists;
 *     [x] no fixed topology exists;
 *     [x] no fixed hardware identifier exists;
 *     [x] no provider-specific behavior exists;
 *     [x] no runtime lookup occurs;
 *     [x] no scheduling algorithm occurs;
 *     [x] no duplicated expression grammar exists;
 *     [x] no duplicated duration lexer exists.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * latency.g4 is complete when:
 *
 *     1. It parses the complete latency clause defined by the Zamani resource
 *        model.
 *
 *     2. It delegates expression syntax to the canonical resource-expression
 *        grammar.
 *
 *     3. It does not duplicate duration-literal syntax.
 *
 *     4. It does not duplicate arithmetic or comparison syntax.
 *
 *     5. It does not contain machine-specific constants.
 *
 *     6. It does not contain hardware-specific assumptions.
 *
 *     7. It has no dependency on scheduling/runtime/hardware implementation.
 *
 *     8. resources.g4 delegates to this grammar rather than implementing a
 *        second latency grammar.
 *
 *     9. Hardware, quantum, HDL, distributed and networking grammars can
 *        consume the same latency abstraction.
 *
 *    10. Semantic analysis receives a canonical latency expression without
 *        premature evaluation.
 *
 *    11. Generated Rust integration remains compatible with Rust 1.97 /
 *        Rust 1.97.1.
 *
 *    12. No unsafe Rust is required.
 *
 *    13. Positive, negative, boundary, scalability and cross-domain tests
 *        pass.
 *
 *    14. The same source construct remains portable across different target
 *        architectures and resource scales.
 *
 * ============================================================================
 */

parser grammar Latency;

options {
    tokenVocab = ZamaniLexer;
}

import ResourceExpressions;


/*
 * ============================================================================
 * 1. PUBLIC LATENCY CLAUSE
 * ============================================================================
 *
 * Canonical source form:
 *
 *     latency = <resource-expression>;
 *
 * The expression remains symbolic until semantic analysis.
 */
latencyClause
    : K_LATENCY
      ASSIGN
      latencyExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 2. LATENCY EXPRESSION
 * ============================================================================
 *
 * This is deliberately a semantic wrapper around the canonical resource
 * expression.
 *
 * It does NOT redefine expression precedence.
 */
latencyExpression
    : resourceLatencyExpression
    ;


/*
 * ============================================================================
 * 3. LATENCY VALUE
 * ============================================================================
 *
 * Explicit alias for consumers that need a value-oriented entry point.
 *
 * This remains the canonical resource expression architecture.
 */
latencyValue
    : latencyExpression
    ;


/*
 * ============================================================================
 * 4. OPTIONAL LATENCY CLAUSE
 * ============================================================================
 *
 * Useful for enclosing resource specifications where latency is optional.
 */
optionalLatencyClause
    : latencyClause?
    ;
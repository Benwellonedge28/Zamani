/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/hardware/performance.g4
 *
 * GRAMMAR
 * -------
 * HardwarePerformance
 *
 * STATUS
 * ------
 * CANONICAL HARDWARE PERFORMANCE COMPOSITION GRAMMAR
 *
 * RUNTIME BASELINE
 * ----------------
 * Rust 1.97 or later
 * Rust 2021
 * Safe Rust only
 * No unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the HARDWARE-DOMAIN PERFORMANCE COMPOSITION BOUNDARY.
 *
 * It does not create a second performance language.
 *
 * Universal performance intent belongs to:
 *
 *     grammar/resources/performance.g4
 *
 * This file exposes that canonical performance model to the hardware domain
 * without duplicating its semantics.
 *
 * Hardware performance intent may describe things such as:
 *
 *     throughput
 *     latency
 *     bandwidth
 *     energy
 *     power
 *     reliability
 *     scalability
 *     utilization
 *     efficiency
 *     responsiveness
 *     cost
 *     domain-specific performance properties
 *
 * The actual meaning, units, feasibility, measurement and realization are
 * determined by downstream semantic analysis.
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
 *     Hardware
 *          |
 *          v
 *     HardwarePerformance
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     semantic resource/performance model
 *          |
 *          +-----------------------------+
 *          |             |               |
 *          v             v               v
 *       compiler     optimizer       resource manager
 *          |             |               |
 *          +-------------+---------------+
 *                        |
 *                        v
 *                 scheduling/routing
 *                        |
 *                        v
 *                       HAL
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 *     hardware-performance composition
 *     hardware performance declaration boundary
 *     hardware performance group boundary
 *     hardware performance property composition
 *     hardware-domain reuse of canonical performance intent
 *
 * THIS FILE DOES NOT OWN
 * ----------------------
 *
 *     lexer definitions
 *     keyword definitions
 *     identifier definitions
 *     qualified-name definitions
 *     expression precedence
 *     resource-expression semantics
 *     performance measurement
 *     benchmarking
 *     hardware discovery
 *     resource allocation
 *     scheduling
 *     routing
 *     optimization
 *     device selection
 *     physical placement
 *     quantum operations
 *     quantum::ir
 *     QEC
 *     ZQN
 *     HAL
 *     runtime implementation
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/resources/resource-expressions.g4
 *     grammar/core/names.g4
 *     grammar/resources/performance.g4
 *
 * Imported grammar authorities:
 *
 *     ResourceExpressions
 *     Names
 *     ResourcePerformance
 *
 * EXPORTS
 * -------
 *
 *     hardwarePerformanceDeclaration
 *     hardwarePerformanceSpecification
 *     hardwarePerformanceItem
 *     hardwarePerformanceReference
 *     hardwarePerformanceProperty
 *     hardwarePerformanceExpression
 *
 * CONSUMED_BY
 * -----------
 *
 *     grammar/hardware/hardware.g4
 *
 * AST_OWNER
 * ---------
 *
 *     Domain-neutral frontend AST.
 *
 * SEMANTIC_OWNER
 * -------------
 *
 *     Compiler/frontend resource-performance semantic layer.
 *
 * IR_OWNER
 * --------
 *
 *     Canonical semantic resource model.
 *
 * TEST_OWNER
 * ----------
 *
 *     grammar/tests/hardware/performance/
 *
 * SPEC_OWNER
 * ----------
 *
 *     grammar/spec/resources.md
 *     grammar/spec/hardware.md
 *
 * ============================================================================
 * NON-DUPLICATION CONTRACT
 * ============================================================================
 *
 * This file MUST NOT redefine:
 *
 *     expression
 *     resourceExpression
 *     resource performance comparison semantics
 *     arithmetic precedence
 *     logical precedence
 *     units
 *     numeric literals
 *     identifiers
 *     qualified names
 *
 * It delegates those concerns to the canonical grammars.
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * Performance intent MUST remain independent of the physical machine.
 *
 * This grammar MUST NOT encode universal limits for:
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
 *     memory
 *     storage
 *     devices
 *     bandwidth
 *     throughput
 *     latency
 *
 * No finite performance-property count is imposed.
 *
 * No finite group size is imposed.
 *
 * No finite namespace depth is imposed.
 *
 * No finite expression-list size is imposed.
 *
 * "Infinity" means unbounded by the language architecture, not physically
 * infinite hardware.
 *
 * ============================================================================
 * PERFORMANCE SEMANTIC DISTINCTION
 * ============================================================================
 *
 * Performance intent MAY be:
 *
 *     descriptive
 *     required
 *     constrained
 *     preferred
 *     advisory
 *     conditional
 *
 * This grammar preserves syntax only.
 *
 * Semantic analysis determines the category according to the containing
 * construct.
 *
 * A performance objective MUST NOT automatically become a hard requirement.
 *
 * ============================================================================
 * RESOURCE BOUNDARY
 * ============================================================================
 *
 * Hardware performance expressions consume:
 *
 *     resourceExpression
 *
 * from ResourceExpressions.
 *
 * Therefore expressions such as:
 *
 *     workload_size
 *     problem_size * parallelism
 *     required_throughput
 *     input.size / duration
 *     available_bandwidth
 *
 * remain ordinary semantic expressions.
 *
 * The grammar does not evaluate them.
 *
 * ============================================================================
 * HARDWARE BOUNDARY
 * ============================================================================
 *
 * This grammar may describe hardware-oriented intent such as:
 *
 *     performance = required_performance;
 *
 *     performance {
 *         throughput = required_throughput;
 *         latency = latency_budget;
 *         bandwidth = required_bandwidth;
 *     };
 *
 * The source does NOT select:
 *
 *     CPU 0
 *     GPU 3
 *     FPGA 1
 *     physical QPU
 *     physical node
 *     PCI address
 *     memory address
 *
 * Those belong to downstream target-specific realization.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Hardware performance may influence quantum compilation.
 *
 * Example semantic flow:
 *
 *     quantum source
 *          |
 *          v
 *     performance intent
 *          |
 *          v
 *     quantum semantic analysis
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing/scheduling
 *
 * This grammar does NOT define quantum operations and MUST NOT create a
 * hardware-performance quantum IR.
 *
 * ============================================================================
 * HDL BOUNDARY
 * ============================================================================
 *
 * HDL may consume performance intent for:
 *
 *     timing
 *     throughput
 *     bandwidth
 *     energy
 *     power
 *     resource efficiency
 *
 * Hardware performance syntax does not implement:
 *
 *     synthesis
 *     timing closure
 *     placement
 *     routing
 *     physical design
 *
 * ============================================================================
 * AI / DATA / DISTRIBUTED BOUNDARY
 * ============================================================================
 *
 * The same performance model may describe:
 *
 *     tensor throughput
 *     inference throughput
 *     training throughput
 *     data throughput
 *     communication throughput
 *     distributed latency
 *     accelerator utilization
 *
 * Domain-specific interpretation belongs to semantic analysis.
 *
 * ============================================================================
 * SAFETY CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no Rust actions
 *     no Rust semantic predicates
 *     no filesystem access
 *     no network access
 *     no hardware access
 *     no runtime calls
 *     no allocation
 *     no benchmarking
 *
 * Generated Rust integration MUST remain safe Rust.
 *
 * No unsafe Rust is required.
 *
 * ============================================================================
 */

parser grammar HardwarePerformance;

options {
    tokenVocab = ZamaniLexer;
}

import
    ResourceExpressions,
    Names,
    ResourcePerformance
;


/*
 * ============================================================================
 * 1. PUBLIC HARDWARE PERFORMANCE DECLARATION
 * ============================================================================
 *
 * This is deliberately an adapter around the canonical resource-performance
 * grammar.
 *
 * No second performance semantic model is introduced.
 */
hardwarePerformanceDeclaration
    : resourcePerformanceDeclaration
    | resourcePerformanceSpecification
    | resourcePerformanceGroup
    | resourcePerformanceConditionalGroup
    ;


/*
 * ============================================================================
 * 2. HARDWARE PERFORMANCE SPECIFICATION
 * ============================================================================
 *
 * Public hardware-domain alias.
 *
 * The actual syntax is owned by ResourcePerformance.
 */
hardwarePerformanceSpecification
    : resourcePerformanceSpecification
    ;


/*
 * ============================================================================
 * 3. HARDWARE PERFORMANCE ITEM
 * ============================================================================
 *
 * This rule exists for consumers that need an individual reusable item.
 */
hardwarePerformanceItem
    : resourcePerformanceItem
    ;


/*
 * ============================================================================
 * 4. HARDWARE PERFORMANCE REFERENCE
 * ============================================================================
 *
 * Symbolic references are resolved semantically.
 *
 * They do not identify physical hardware.
 */
hardwarePerformanceReference
    : qualifiedName
    ;


/*
 * ============================================================================
 * 5. HARDWARE PERFORMANCE PROPERTY
 * ============================================================================
 *
 * Property values remain canonical resource expressions.
 */
hardwarePerformanceProperty
    : hardwarePerformanceReference
      ASSIGN
      resourceExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 6. HARDWARE PERFORMANCE EXPRESSION
 * ============================================================================
 *
 * No second expression grammar is created.
 */
hardwarePerformanceExpression
    : resourceExpression
    ;


/*
 * ============================================================================
 * 7. HARDWARE PERFORMANCE LIST
 * ============================================================================
 *
 * Unbounded by grammar design.
 */
hardwarePerformanceList
    : hardwarePerformanceItem*
    ;


/*
 * ============================================================================
 * 8. COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [ ] Hardware imports HardwarePerformance successfully.
 *
 * [ ] ResourcePerformance remains the sole universal performance owner.
 *
 * [ ] No performance expression grammar is duplicated here.
 *
 * [ ] No performance-specific lexer tokens are introduced here.
 *
 * [ ] No physical hardware identifiers are introduced.
 *
 * [ ] No machine-size constants are introduced.
 *
 * [ ] No fixed performance-property count exists.
 *
 * [ ] No fixed objective/group cardinality exists.
 *
 * [ ] Resource expressions resolve through ResourceExpressions.
 *
 * [ ] Qualified names resolve through Names.
 *
 * [ ] Hardware performance lowers into the canonical semantic resource model.
 *
 * [ ] Quantum consumers continue through quantum::ir.
 *
 * [ ] HDL consumers remain in the HDL/hardware semantic pipeline.
 *
 * [ ] Classical consumers remain in the classical semantic pipeline.
 *
 * [ ] AI/data/distributed consumers remain domain-specific semantic consumers.
 *
 * [ ] Positive tests exist.
 *
 * [ ] Negative tests exist.
 *
 * [ ] Boundary tests exist.
 *
 * [ ] Scalability tests exist.
 *
 * [ ] Cross-domain tests exist.
 *
 * [ ] Determinism tests exist.
 *
 * [ ] Compatibility tests exist.
 *
 * [ ] Generated Rust remains compatible with Rust 1.97 or later.
 *
 * [ ] No unsafe Rust is required.
 *
 * ============================================================================
 */
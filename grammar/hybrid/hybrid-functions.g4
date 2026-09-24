/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/hybrid/hybrid-functions.g4
 *
 * Grammar:
 *     HybridFunctions
 *
 * Status:
 *     PRODUCTION HYBRID-FUNCTION COMPOSITION CONTRACT
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     This file contains ANTLR4 parser grammar only.
 *
 *     It contains:
 *       - no embedded Rust;
 *       - no semantic predicates;
 *       - no filesystem access;
 *       - no network access;
 *       - no process execution;
 *       - no hardware discovery;
 *       - no target selection;
 *       - no resource allocation;
 *       - no runtime execution;
 *       - no backend-specific behavior;
 *       - no unsafe Rust.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file is the HYBRID FUNCTION COMPOSITION boundary.
 *
 * It does NOT create a second kind of function.
 *
 * Zamani has exactly one source-level function model:
 *
 *     grammar/functions/functions.g4
 *
 * A hybrid function is therefore an ordinary Zamani function whose:
 *
 *     - parameters;
 *     - return values;
 *     - body;
 *     - effects;
 *     - capabilities;
 *     - contracts;
 *     - resource requirements;
 *     - expressions;
 *     - statements;
 *
 * may participate in multiple computational domains.
 *
 * This file exists only to give the canonical Hybrid grammar a stable,
 * explicit integration point for functions.
 *
 * ============================================================================
 * ARCHITECTURAL RULE
 * ============================================================================
 *
 * DO NOT create:
 *
 *     HybridFunction
 *     QuantumFunction
 *     ClassicalFunction
 *     GPUFunction
 *     FPGAFunction
 *     QPUFunction
 *     AcceleratorFunction
 *     HDLFunction
 *
 * as separate language-level function universes.
 *
 * The function AST remains the ordinary domain-neutral Zamani Function node.
 *
 * The hybrid semantic system determines whether a function participates in:
 *
 *     classical computation
 *     quantum computation
 *     classical/quantum interaction
 *     accelerator computation
 *     hardware co-design
 *     distributed computation
 *     AI/data computation
 *     future computational domains
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical ZamaniLexer
 *          |
 *          v
 *     canonical ZamaniParser
 *          |
 *          v
 *     Hybrid
 *          |
 *          v
 *     HybridFunctions
 *          |
 *          v
 *     Functions
 *          |
 *          v
 *     domain-neutral Function AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +------------------------------+
 *          |                              |
 *          v                              v
 *     classical semantics           quantum semantics
 *                                          |
 *                                          v
 *                                      quantum::ir
 *          |                              |
 *          +---------------+--------------+
 *                          |
 *                          v
 *                 canonical semantic model
 *                          |
 *             +------------+-------------+
 *             |            |             |
 *             v            v             v
 *        classical      quantum       HDL/
 *           IR           ::ir       hardware
 *             |            |             |
 *             +------------+-------------+
 *                          |
 *                          v
 *                    optimization
 *                          |
 *             +------------+-------------+
 *             |            |             |
 *             v            v             v
 *          routing     scheduling    resilience
 *                                      |
 *                                      v
 *                                     QEC
 *                                      |
 *                                      v
 *                                     ZQN
 *                                      |
 *                                      v
 *                                     HAL
 *                                      |
 *                                      v
 *                              target realization
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * The authoritative function grammar is:
 *
 *     grammar/functions/functions.g4
 *
 * This file MUST NOT redefine:
 *
 *     functionDeclaration
 *     functionSignature
 *     functionSignatureCore
 *     functionName
 *     functionModifier
 *     functionImplementation
 *     functionDefinition
 *     functionPrototype
 *     functionBodyDeclaration
 *     functionPrototypeDeclaration
 *
 * It consumes those rules through the Functions grammar.
 *
 * ============================================================================
 * WHY THIS FILE EXISTS
 * ============================================================================
 *
 * Hybrid grammar currently owns:
 *
 *     hybridDeclaration
 *     hybridStatement
 *     hybridExpression
 *
 * and its region composition.
 *
 * A hybrid region may legitimately contain a function declaration.
 *
 * Without this adapter, the Hybrid grammar has no explicit function-domain
 * composition boundary.
 *
 * This file provides that boundary without duplicating function syntax.
 *
 * Conceptually:
 *
 *     Hybrid
 *        |
 *        +--> HybridFunctions
 *                 |
 *                 +--> Functions
 *
 * NOT:
 *
 *     Hybrid
 *        |
 *        +--> another function grammar
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * This is a parser delegate grammar.
 *
 * It MUST NOT import:
 *
 *     ZamaniParser
 *     Hybrid
 *
 * because that would create circular grammar composition.
 *
 * The intended dependency direction is:
 *
 *     ZamaniLexer
 *          |
 *          v
 *     Functions
 *          |
 *          v
 *     HybridFunctions
 *          |
 *          v
 *     Hybrid
 *          |
 *          v
 *     ZamaniParser
 *
 * More precisely, the canonical parser composition should expose:
 *
 *     Hybrid
 *         -> HybridFunctions
 *             -> Functions
 *
 * and:
 *
 *     ZamaniParser
 *         -> Hybrid
 *
 * There must be no:
 *
 *     HybridFunctions -> ZamaniParser
 *
 * and no:
 *
 *     Hybrid -> ZamaniParser
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical production lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Its lexical vocabulary is composed through:
 *
 *     grammar/lexer/tokens.g4
 *
 * Parser grammars consume:
 *
 *     tokenVocab = ZamaniLexer
 *
 * This file therefore uses the canonical production lexer vocabulary.
 *
 * No lexer rules are defined here.
 *
 * No function-specific tokens are introduced here.
 *
 * No hybrid-function-specific keywords are introduced here.
 *
 * ============================================================================
 * FUNCTION MODEL
 * ============================================================================
 *
 * A hybrid function is syntactically just a Zamani function.
 *
 * For example:
 *
 *     fn prepare(input: Data) -> State {
 *         ...
 *     }
 *
 * remains a normal function.
 *
 * Its hybrid nature may emerge because:
 *
 *     - its parameter type is quantum;
 *     - its return type is quantum;
 *     - its body invokes quantum operations;
 *     - its body consumes measurement results;
 *     - its body invokes an accelerator;
 *     - its effects identify cross-domain behavior;
 *     - its capabilities identify required computation;
 *     - its resource requirements span domains;
 *     - its semantic dependency graph crosses domains.
 *
 * The parser must not infer those meanings.
 *
 * ============================================================================
 * NO SPECIAL HYBRID FUNCTION KEYWORD
 * ============================================================================
 *
 * This file deliberately does NOT introduce:
 *
 *     HYBRID_FN
 *     HYBRID_FUNCTION
 *     QUANTUM_FN
 *     CLASSICAL_FN
 *     ACCELERATOR_FN
 *
 * Such keywords would unnecessarily fragment the language.
 *
 * Hybrid is a semantic/domain composition property, not a second function
 * declaration language.
 *
 * ============================================================================
 * NO DUPLICATED FUNCTION SYNTAX
 * ============================================================================
 *
 * The following remain exclusively owned by Functions:
 *
 *     attributes
 *     modifiers
 *     fn keyword
 *     function name
 *     generic parameters
 *     parameter list
 *     return clause
 *     generic constraints
 *     effects
 *     contracts
 *     function body
 *     function prototype
 *
 * Therefore this file intentionally contains no productions such as:
 *
 *     hybridParameterList
 *     hybridReturnType
 *     hybridGenericParameters
 *     hybridFunctionBody
 *     hybridFunctionContract
 *
 * Those would create competing ownership.
 *
 * ============================================================================
 * HYBRID FUNCTION DECLARATION
 * ============================================================================
 *
 * The primary public rule is:
 *
 *     hybridFunctionDeclaration
 *
 * It is an adapter over the canonical:
 *
 *     functionDeclaration
 *
 * The parse tree therefore retains a stable hybrid-domain boundary while the
 * underlying function syntax remains canonical.
 *
 * Semantic analysis may use the surrounding context to identify that this
 * function participates in hybrid computation.
 *
 * ============================================================================
 */

parser grammar HybridFunctions;

options {
    tokenVocab = ZamaniLexer;
}

import Functions;


/*
 * ============================================================================
 * 1. PUBLIC HYBRID FUNCTION DECLARATION
 * ============================================================================
 *
 * This is the only public entry point required by the Hybrid grammar.
 *
 * It delegates the complete function declaration to the canonical Functions
 * grammar.
 *
 * No function syntax is duplicated here.
 */
hybridFunctionDeclaration
    : functionDeclaration
    ;


/*
 * ============================================================================
 * 2. HYBRID FUNCTION DEFINITION
 * ============================================================================
 *
 * Stable adapter for consumers that explicitly require a function definition
 * rather than a declaration/prototype.
 *
 * The underlying syntax remains owned by Functions.
 */
hybridFunctionDefinition
    : functionDefinition
    ;


/*
 * ============================================================================
 * 3. HYBRID FUNCTION PROTOTYPE
 * ============================================================================
 *
 * Stable adapter for declaration-only contexts.
 *
 * Whether a prototype is legal in a particular hybrid context is determined
 * by semantic/declaration-context validation.
 */
hybridFunctionPrototype
    : functionPrototype
    ;


/*
 * ============================================================================
 * 4. HYBRID FUNCTION SIGNATURE
 * ============================================================================
 *
 * This rule exposes the canonical function signature without redefining it.
 *
 * It is useful to:
 *
 *     - parser tooling;
 *     - syntax indexing;
 *     - IDE tooling;
 *     - AST adapters;
 *     - hybrid-domain analysis.
 *
 * It does not alter the underlying function signature.
 */
hybridFunctionSignature
    : functionSignature
    ;


/*
 * ============================================================================
 * 5. HYBRID FUNCTION SIGNATURE CORE
 * ============================================================================
 *
 * Adapter over the canonical function signature core.
 *
 * The underlying source ordering remains owned by Functions.
 */
hybridFunctionSignatureCore
    : functionSignatureCore
    ;


/*
 * ============================================================================
 * 6. HYBRID FUNCTION IMPLEMENTATION
 * ============================================================================
 *
 * Adapter over the canonical function implementation boundary.
 *
 * This preserves the distinction between:
 *
 *     function definition
 *
 * and:
 *
 *     function declaration/prototype
 *
 * without inventing hybrid-specific body syntax.
 */
hybridFunctionImplementation
    : functionImplementation
    ;


/*
 * ============================================================================
 * 7. HYBRID FUNCTION BODY DECLARATION
 * ============================================================================
 *
 * Adapter for consumers requiring a function declaration with a body.
 */
hybridFunctionBodyDeclaration
    : functionBodyDeclaration
    ;


/*
 * ============================================================================
 * 8. HYBRID FUNCTION PROTOTYPE DECLARATION
 * ============================================================================
 *
 * Adapter for consumers requiring a declaration-only function form.
 */
hybridFunctionPrototypeDeclaration
    : functionPrototypeDeclaration
    ;


/*
 * ============================================================================
 * 9. HYBRID FUNCTION
 * ============================================================================
 *
 * Generic stable alias.
 *
 * This rule intentionally resolves to the ordinary function declaration.
 *
 * It does NOT create a second AST node or semantic function category.
 */
hybridFunction
    : hybridFunctionDeclaration
    ;


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every:
 *
 *     hybridFunctionDeclaration
 *
 * MUST lower to the SAME domain-neutral Function AST representation produced
 * by:
 *
 *     functionDeclaration
 *
 * This grammar MUST NOT introduce:
 *
 *     HybridFunctionNode
 *     QuantumFunctionNode
 *     ClassicalFunctionNode
 *     AcceleratorFunctionNode
 *     HardwareFunctionNode
 *
 * or any equivalent parallel AST hierarchy.
 *
 * The AST must preserve the ordinary function information already defined by
 * Functions, including as applicable:
 *
 *     - source metadata;
 *     - source span;
 *     - name;
 *     - visibility;
 *     - modifiers;
 *     - generic parameters;
 *     - parameters;
 *     - return type;
 *     - constraints;
 *     - effects;
 *     - contracts;
 *     - body.
 *
 * Hybrid participation is represented by the surrounding semantic/domain
 * relationships rather than by a second function AST.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes only:
 *
 *     "this is a function declaration occurring at a hybrid composition
 *      boundary."
 *
 * Semantic analysis determines whether the function actually is hybrid and
 * what domains it participates in.
 *
 * Semantic analysis may inspect:
 *
 *     - parameter types;
 *     - return types;
 *     - function effects;
 *     - capabilities;
 *     - resource requirements;
 *     - function body expressions;
 *     - function body statements;
 *     - quantum operations;
 *     - measurements;
 *     - classical control;
 *     - accelerator operations;
 *     - hardware intent;
 *     - distributed operations;
 *     - data movement;
 *     - synchronization.
 *
 * The grammar MUST NOT perform any of these semantic decisions.
 *
 * ============================================================================
 * DOMAIN PARTICIPATION
 * ============================================================================
 *
 * A single function may contain:
 *
 *     classical computation
 *         ->
 *     quantum computation
 *         ->
 *     measurement
 *         ->
 *     classical decision
 *         ->
 *     quantum computation
 *
 * It may also participate in:
 *
 *     classical + accelerator
 *
 *     quantum + accelerator
 *
 *     classical + quantum + accelerator
 *
 *     software + HDL/hardware co-design
 *
 *     distributed + accelerator
 *
 *     AI + quantum
 *
 *     data + accelerator
 *
 *     future domain combinations
 *
 * The function grammar remains unchanged for all of these cases.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum functions remain ordinary functions.
 *
 * Example conceptual source:
 *
 *     fn hybrid_step(state: quantum::State, value: Float) -> Bit {
 *         ...
 *     }
 *
 * The grammar does not determine whether:
 *
 *     quantum::State
 *
 * is a logical state, physical state, simulator state, or another semantic
 * abstraction.
 *
 * That is resolved downstream.
 *
 * Quantum lowering MUST follow:
 *
 *     Function AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum semantic representation
 *          |
 *          v
 *     quantum::ir
 *
 * `quantum::ir` remains the ONE canonical quantum semantic boundary.
 *
 * This file must never introduce:
 *
 *     HybridQuantumIR
 *     HybridCircuitIR
 *     HybridGateIR
 *
 * or another quantum IR.
 *
 * ============================================================================
 * CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Classical portions of a hybrid function lower through the repository's
 * canonical classical semantic/IR path.
 *
 * This grammar does not select:
 *
 *     CPU
 *     core
 *     thread
 *     vector width
 *     instruction set
 *     ABI
 *     runtime executor
 *
 * ============================================================================
 * ACCELERATOR INTEGRATION
 * ============================================================================
 *
 * A hybrid function may invoke accelerator operations through the existing
 * hybrid/accelerator grammar and semantic capability system.
 *
 * This file does not define accelerator syntax.
 *
 * Accelerator syntax remains owned by the appropriate hybrid and
 * interoperability grammars.
 *
 * This file merely permits the ordinary function boundary to contain that
 * computation.
 *
 * The compiler may subsequently lower the semantic operation to:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC-backed accelerator
 *     tensor accelerator
 *     AI accelerator
 *     quantum accelerator
 *     future accelerator
 *
 * without changing the function grammar.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * A hybrid function may participate in software/hardware co-design.
 *
 * Hardware intent belongs to:
 *
 *     grammar/hardware/
 *     grammar/hdl/
 *     grammar/resources/
 *     grammar/compile/
 *     grammar/execution/
 *
 * This file does not define:
 *
 *     registers
 *     ports
 *     wires
 *     physical addresses
 *     clocks
 *     buses
 *     pipeline widths
 *     FPGA resources
 *     ASIC resources
 *     physical device IDs
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Hybrid functions may have source-level requirements expressed through the
 * canonical function effects/contracts/capability/resource mechanisms.
 *
 * Examples of semantic intent include:
 *
 *     requires capability("quantum.measurement")
 *
 *     requires capability("tensor.compute")
 *
 *     requires capability("gpu.compute")
 *
 *     requires memory >= required_memory
 *
 *     requires qubits >= n
 *
 * These are semantic requirements.
 *
 * They do NOT mean:
 *
 *     use GPU 0
 *     use QPU 0
 *     use physical qubit 0
 *     use exactly N cores
 *     use exactly N threads
 *
 * Resolution belongs to:
 *
 *     semantic analysis
 *     resource analysis
 *     compilation
 *     scheduling
 *     routing
 *     hardware abstraction
 *     runtime
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * Effects remain owned by:
 *
 *     grammar/effects/
 *
 * This file does not define:
 *
 *     hybridEffect
 *     quantumEffect
 *     acceleratorEffect
 *
 * unless such types are independently established by the canonical effect
 * system.
 *
 * A function's effects remain part of the ordinary Function AST/semantic
 * model.
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Contracts remain owned by:
 *
 *     grammar/functions/contracts.g4
 *
 * This file does not redefine:
 *
 *     functionContractClause
 *     functionRequiresContract
 *     functionEnsuresContract
 *     functionInvariantContract
 *
 * Hybrid correctness requirements can therefore be expressed using ordinary
 * function contracts.
 *
 * Semantic analysis determines whether a contract refers to:
 *
 *     classical values
 *     quantum values
 *     measurements
 *     resources
 *     capabilities
 *     cross-domain state
 *     execution properties
 *
 * ============================================================================
 * GENERIC INTEGRATION
 * ============================================================================
 *
 * Generic parameters remain owned by:
 *
 *     grammar/functions/generics.g4
 *
 * Function constraints remain owned by:
 *
 *     grammar/functions/constraints.g4
 *
 * This file does not define:
 *
 *     hybridGenericParameter
 *     hybridGenericConstraint
 *
 * Generic functions therefore remain portable across computational domains.
 *
 * ============================================================================
 * PARAMETER INTEGRATION
 * ============================================================================
 *
 * Parameters remain owned by:
 *
 *     grammar/functions/parameters.g4
 *
 * A hybrid function can therefore accept any canonical Zamani type that the
 * type system supports, including future domain types.
 *
 * This avoids requiring a grammar change whenever a new computational domain
 * or resource abstraction is introduced.
 *
 * ============================================================================
 * RETURN INTEGRATION
 * ============================================================================
 *
 * Return syntax remains owned by:
 *
 *     grammar/functions/returns.g4
 *
 * A hybrid function may return:
 *
 *     classical values
 *     quantum values
 *     measurement values
 *     data values
 *     resource abstractions
 *     future domain values
 *
 * subject to semantic type validation.
 *
 * ============================================================================
 * FUNCTION BODY INTEGRATION
 * ============================================================================
 *
 * Function bodies remain owned by the canonical function/block composition.
 *
 * This file MUST NOT define:
 *
 *     hybridBlock
 *     hybridStatementBlock
 *     hybridFunctionBody
 *
 * as replacements for the ordinary function body.
 *
 * Hybrid statements and expressions can already participate in ordinary
 * statement/expression composition.
 *
 * ============================================================================
 * CONTROL-FLOW INTEGRATION
 * ============================================================================
 *
 * Classical control over quantum computation remains owned by the hybrid
 * control grammar.
 *
 * In particular:
 *
 *     grammar/hybrid/quantum-classical-control.g4
 *     grammar/hybrid/feedforward.g4
 *
 * provide the appropriate source-level hybrid control boundaries.
 *
 * This file does not redefine:
 *
 *     if
 *     while
 *     match
 *     return
 *     branch
 *     measurement control
 *
 * ============================================================================
 * CLASSICAL / QUANTUM BOUNDARY
 * ============================================================================
 *
 * Cross-domain interaction remains owned by the existing hybrid grammar
 * components, including:
 *
 *     grammar/hybrid/classical-quantum.g4
 *     grammar/hybrid/classical-quantum-boundary.g4
 *     grammar/hybrid/quantum-classical-control.g4
 *     grammar/hybrid/feedforward.g4
 *
 * This file does not duplicate those rules.
 *
 * A hybrid function simply provides a canonical function boundary in which
 * those constructs may occur.
 *
 * ============================================================================
 * SHARED DATA
 * ============================================================================
 *
 * Shared-data syntax remains owned by:
 *
 *     grammar/hybrid/shared-data.g4
 *
 * This file does not define:
 *
 *     hybridSharedData
 *     hybridTransfer
 *     hybridBuffer
 *     hybridMemory
 *
 * Function parameters and return values can participate in shared-data
 * semantics through the ordinary type/ownership/resource system.
 *
 * ============================================================================
 * SYNCHRONIZATION
 * ============================================================================
 *
 * Synchronization remains owned by:
 *
 *     grammar/hybrid/synchronization.g4
 *
 * and the broader concurrency/execution systems.
 *
 * This file does not introduce:
 *
 *     hybridAwait
 *     hybridBarrier
 *     hybridFence
 *     hybridTimestamp
 *
 * as function-specific constructs.
 *
 * ============================================================================
 * ASYNC INTEGRATION
 * ============================================================================
 *
 * Async functions remain ordinary functions using the canonical function
 * modifier and async grammar.
 *
 * This file does not define:
 *
 *     hybridAsyncFunction
 *     hybridAwait
 *     hybridExecutor
 *
 * Executor selection, scheduling, placement and concurrency semantics remain
 * downstream.
 *
 * Therefore:
 *
 *     async fn ...
 *
 * remains an ordinary function declaration that may participate in hybrid
 * computation.
 *
 * ============================================================================
 * GENERATORS
 * ============================================================================
 *
 * Generator syntax remains owned by:
 *
 *     grammar/functions/generators.g4
 *
 * This file does not define hybrid generator syntax.
 *
 * A generator may participate in hybrid computation if semantic analysis
 * permits it.
 *
 * ============================================================================
 * CLOSURES / LAMBDAS
 * ============================================================================
 *
 * Closures and lambdas remain expressions.
 *
 * They remain owned by:
 *
 *     grammar/functions/closures.g4
 *     grammar/functions/lambdas.g4
 *     grammar/expressions/
 *
 * This file does not duplicate them.
 *
 * ============================================================================
 * FOREIGN FUNCTIONS
 * ============================================================================
 *
 * Foreign function syntax remains owned by:
 *
 *     grammar/functions/foreign-functions.g4
 *     grammar/interoperability/
 *
 * This file does not define ABI or foreign-language syntax.
 *
 * A foreign callable may participate in hybrid computation when the semantic
 * interoperability system permits it.
 *
 * ============================================================================
 * COMPILE-TIME FUNCTIONS
 * ============================================================================
 *
 * Compile-time functions remain part of the universal function model.
 *
 * Compile-time/metaprogramming syntax remains owned by:
 *
 *     grammar/functions/compile-time-functions.g4
 *     grammar/metaprogramming/
 *
 * This file does not create:
 *
 *     hybridCompileTimeFunction
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * No hybrid-specific quantum IR is permitted.
 *
 * Required conceptual lowering:
 *
 *     hybrid function
 *          |
 *          v
 *     domain-neutral Function AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +-----------------------------+
 *          |                             |
 *          v                             v
 *     classical semantics           quantum semantics
 *                                          |
 *                                          v
 *                                      quantum::ir
 *          |                             |
 *          +--------------+--------------+
 *                         |
 *                         v
 *                canonical semantic model
 *                         |
 *                         v
 *              optimization / lowering
 *                         |
 *              routing / scheduling
 *                         |
 *              resilience / QEC / ZQN
 *                         |
 *                         v
 *                        HAL
 *                         |
 *                         v
 *                 target realization
 *
 * This grammar must never directly lower to:
 *
 *     LLVM
 *     MLIR
 *     QIR
 *     OpenQASM
 *     CUDA
 *     ROCm
 *     vendor assembly
 *     FPGA bitstream
 *     physical qubit mapping
 *     hardware schedule
 *
 * Interoperability/lowering systems may consume the canonical semantic model
 * downstream.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * This file imposes NO language-level limit on:
 *
 *     number of hybrid functions
 *     number of parameters
 *     number of generic parameters
 *     number of return values where supported
 *     number of effects
 *     number of contracts
 *     number of statements
 *     number of expressions
 *     number of domain crossings
 *     number of quantum operations
 *     number of classical operations
 *     number of accelerator operations
 *     number of resources
 *     number of capabilities
 *     source size
 *     function nesting/structure where syntactically supported
 *
 * It MUST NOT introduce:
 *
 *     MAX_HYBRID_FUNCTIONS
 *     MAX_DOMAIN_CROSSINGS
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_ACCELERATORS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_REGISTER_WIDTH
 *     MAX_TENSOR_RANK
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * Structural repetition remains unbounded at the language level.
 *
 * "Infinity" means that Zamani does not artificially impose a finite semantic
 * maximum. Actual compilation and execution remain constrained by resources
 * available to the implementation and target.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
 *
 *     CPU IDs
 *     GPU IDs
 *     FPGA IDs
 *     QPU IDs
 *     physical qubit IDs
 *     node IDs
 *     memory-bank IDs
 *     device IDs
 *     fixed topology
 *     fixed bus width
 *     fixed register width
 *     fixed accelerator count
 *     fixed qubit count
 *     fixed thread count
 *     fixed memory capacity
 *
 * Any target-specific realization belongs downstream.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing is deterministic.
 *
 * This grammar has:
 *
 *     - no semantic predicates;
 *     - no embedded actions;
 *     - no randomness;
 *     - no environment inspection;
 *     - no hardware inspection;
 *     - no network access;
 *     - no filesystem access;
 *     - no runtime calls.
 *
 * Identical token streams must produce equivalent parse structures.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * The grammar does not execute code.
 *
 * It cannot:
 *
 *     - launch processes;
 *     - read files;
 *     - access secrets;
 *     - access hardware;
 *     - access networks;
 *     - dynamically load libraries;
 *     - query runtime state.
 *
 * Security and capability semantics remain downstream.
 *
 * ============================================================================
 * ERROR / RECOVERY CONTRACT
 * ============================================================================
 *
 * Syntax errors must remain ordinary parser errors.
 *
 * This grammar should not attempt to diagnose semantic errors such as:
 *
 *     "this function cannot run on the selected QPU"
 *
 * or:
 *
 *     "the target has insufficient qubits"
 *
 * Those are downstream semantic/resource errors.
 *
 * Parser diagnostics should instead identify malformed syntax and preserve
 * source spans.
 *
 * ============================================================================
 * SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * Every adapter context produced by this grammar naturally inherits the source
 * span of the delegated Functions context.
 *
 * AST construction must preserve the original source locations.
 *
 * No source span may be synthesized from:
 *
 *     hardware state;
 *     runtime state;
 *     semantic analysis;
 *     target selection.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This file is a NEW file and therefore has no existing filename compatibility
 * obligation.
 *
 * Existing filenames are deliberately preserved.
 *
 * It must not rename:
 *
 *     grammar/hybrid/hybrid.g4
 *     grammar/functions/functions.g4
 *     grammar/functions/parameters.g4
 *     grammar/functions/returns.g4
 *     grammar/functions/generics.g4
 *     grammar/functions/contracts.g4
 *
 * Public rules introduced here:
 *
 *     hybridFunction
 *     hybridFunctionDeclaration
 *     hybridFunctionDefinition
 *     hybridFunctionPrototype
 *     hybridFunctionSignature
 *     hybridFunctionSignatureCore
 *     hybridFunctionImplementation
 *     hybridFunctionBodyDeclaration
 *     hybridFunctionPrototypeDeclaration
 *
 * These are composition adapters.
 *
 * Their semantics MUST remain equivalent to the corresponding canonical
 * Functions rules.
 *
 * ============================================================================
 * REQUIRED INTEGRATION WITH hybrid.g4
 * ============================================================================
 *
 * `grammar/hybrid/hybrid.g4` should import:
 *
 *     HybridFunctions
 *
 * in its parser imports.
 *
 * Its hybrid construct dispatcher should then add:
 *
 *     | hybridFunctionDeclaration
 *
 * to the existing `hybridConstruct` alternatives.
 *
 * Conceptually:
 *
 *     hybridConstruct
 *         : hybridRegion
 *         | hybridInvocation
 *         | hybridBinding
 *         | hybridControl
 *         | hybridConversion
 *         | hybridSynchronization
 *         | hybridRequirement
 *         | hybridCapability
 *         | hybridPreference
 *         | hybridConstraint
 *         | hybridHint
 *         | hybridFunctionDeclaration
 *         ;
 *
 * The function syntax itself remains owned by Functions.
 *
 * ============================================================================
 * IMPORTANT INTEGRATION NOTE
 * ============================================================================
 *
 * `hybridFunctionDeclaration` MUST be reachable only through the Hybrid
 * composition path.
 *
 * The universal parser continues to use:
 *
 *     universalFunction
 *         : functionDeclaration
 *         ;
 *
 * for ordinary functions.
 *
 * Therefore there are two parser contexts for the same canonical syntax:
 *
 *     ordinary function
 *         -> functionDeclaration
 *
 *     hybrid-context function
 *         -> hybridFunctionDeclaration
 *         -> functionDeclaration
 *
 * This does NOT create two function AST types.
 *
 * The frontend must normalize both contexts to the same Function AST node.
 *
 * ============================================================================
 * NO CIRCULAR DEPENDENCY
 * ============================================================================
 *
 * Dependency graph MUST remain:
 *
 *     Functions
 *        ^
 *        |
 *     HybridFunctions
 *        ^
 *        |
 *     Hybrid
 *        ^
 *        |
 *     ZamaniParser
 *
 * Never:
 *
 *     Functions -> HybridFunctions
 *
 * and never:
 *
 *     HybridFunctions -> Hybrid
 *
 * and never:
 *
 *     HybridFunctions -> ZamaniParser
 *
 * ============================================================================
 * AST NORMALIZATION CONTRACT
 * ============================================================================
 *
 * The frontend parser/AST adapter must normalize:
 *
 *     hybridFunctionDeclaration
 *
 * to the same AST construction path as:
 *
 *     functionDeclaration
 *
 * No semantic information may be lost.
 *
 * The normalization must preserve:
 *
 *     source span
 *     attributes
 *     modifiers
 *     function name
 *     generic parameters
 *     parameters
 *     return type
 *     constraints
 *     effects
 *     contracts
 *     body
 *
 * Hybrid/domain participation is then derived or represented through the
 * existing domain-neutral semantic model.
 *
 * ============================================================================
 * SEMANTIC CROSS-DOMAIN CONTRACT
 * ============================================================================
 *
 * The semantic analyzer may determine that a hybrid function contains:
 *
 *     classical -> quantum
 *
 *     quantum -> classical
 *
 *     classical -> accelerator
 *
 *     quantum -> accelerator
 *
 *     classical -> quantum -> classical
 *
 *     classical -> quantum -> accelerator
 *
 * or future combinations.
 *
 * These dependencies must remain explicit in semantic analysis so downstream
 * scheduling and resource analysis can preserve ordering and data
 * dependencies.
 *
 * ============================================================================
 * SCHEDULING CONTRACT
 * ============================================================================
 *
 * This grammar does not assign:
 *
 *     timestamps
 *     execution slots
 *     queue positions
 *     device order
 *     physical resources
 *
 * The semantic representation produced from the function body must preserve
 * dependencies required by the scheduler.
 *
 * For example:
 *
 *     measurement
 *         |
 *         v
 *     classical result
 *         |
 *         v
 *     conditional
 *         |
 *         v
 *     quantum operation
 *
 * must remain visible to scheduling.
 *
 * ============================================================================
 * ROUTING CONTRACT
 * ============================================================================
 *
 * The grammar does not assign:
 *
 *     physical qubits
 *     physical links
 *     GPU devices
 *     FPGA regions
 *     network nodes
 *
 * Routing receives semantic operations and resource/topology information
 * downstream.
 *
 * ============================================================================
 * QEC / ZQN CONTRACT
 * ============================================================================
 *
 * A hybrid function may contain operations that eventually require:
 *
 *     error correction
 *     resilience
 *     noise analysis
 *     fault handling
 *
 * This file does not implement those mechanisms.
 *
 * The semantic/quantum pipeline remains:
 *
 *     function AST
 *         ->
 *     semantic analysis
 *         ->
 *     quantum::ir
 *         ->
 *     resilience/QEC/ZQN
 *         ->
 *     routing/scheduling as required
 *         ->
 *     HAL
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * The following tests are required downstream.
 *
 * POSITIVE:
 *
 *     fn classical_only(x: Int) -> Int {
 *         x + 1
 *     }
 *
 *     fn quantum_only(q: Qubit) -> Measurement {
 *         ...
 *     }
 *
 *     fn hybrid_value(q: Qubit, x: Float) -> Float {
 *         ...
 *     }
 *
 *     fn hybrid_measurement(q: Qubit) -> Bit {
 *         ...
 *     }
 *
 *     fn hybrid_accelerator(data: Tensor<Float>) -> Tensor<Float> {
 *         ...
 *     }
 *
 *     fn hybrid_generic<T>(value: T) -> T {
 *         ...
 *     }
 *
 *     async fn hybrid_async(...) {
 *         ...
 *     }
 *
 * These examples demonstrate that hybrid participation does not require a
 * separate function syntax.
 *
 * NEGATIVE SYNTAX:
 *
 *     hybrid fn ...
 *
 *     quantum fn ...
 *
 *     accelerator fn ...
 *
 * unless those forms are separately standardized by the language specification.
 *
 *     hybridFunction ...
 *
 *     hybrid fn with malformed signature
 *
 *     missing parameter delimiter
 *
 *     malformed return clause
 *
 *     malformed generic declaration
 *
 *     malformed function body
 *
 * Semantic-invalid examples should be tested separately and must not be
 * incorrectly rejected by this grammar merely because the selected target
 * lacks a capability.
 *
 * ============================================================================
 * BOUNDARY TESTS
 * ============================================================================
 *
 * Test:
 *
 *     zero parameters
 *     one parameter
 *     many parameters
 *     zero generic parameters
 *     many generic parameters
 *     zero contracts
 *     many contracts
 *     zero effects
 *     many effects
 *     deeply nested function bodies
 *     large function bodies
 *     many hybrid domain crossings
 *     repeated quantum/classical boundaries
 *     repeated accelerator boundaries
 *
 * No fixed grammar cardinality is permitted.
 *
 * ============================================================================
 * SCALABILITY TESTS
 * ============================================================================
 *
 * Verify that the grammar contains no language-level artificial maximum for:
 *
 *     function count
 *     parameter count
 *     generic count
 *     statement count
 *     expression count
 *     hybrid crossing count
 *     quantum operation count
 *     accelerator operation count
 *     resource requirement count
 *
 * Large tests should be generated from semantic dimensions rather than
 * hard-coded machine capacities.
 *
 * ============================================================================
 * CROSS-DOMAIN TESTS
 * ============================================================================
 *
 * Required integration coverage:
 *
 *     classical + quantum
 *     quantum + classical
 *     classical + accelerator
 *     quantum + accelerator
 *     classical + quantum + accelerator
 *     classical + HDL/hardware intent
 *     distributed + quantum
 *     AI + quantum
 *     data + accelerator
 *     future domain represented through canonical extensible mechanisms
 *
 * ============================================================================
 * DETERMINISM TESTS
 * ============================================================================
 *
 * Identical source must produce:
 *
 *     identical lexical token stream
 *     equivalent parser structure
 *     equivalent AST normalization
 *
 * independently of:
 *
 *     hardware availability
 *     runtime state
 *     target selection
 *     network state
 *     filesystem state
 *     wall-clock time
 *     randomness
 *
 * ============================================================================
 * ROUND-TRIP TESTS
 * ============================================================================
 *
 * Where the formatter/serializer is available:
 *
 *     source
 *       ->
 *     lexer
 *       ->
 *     parser
 *       ->
 *     hybridFunctionDeclaration
 *       ->
 *     domain-neutral Function AST
 *       ->
 *     formatter
 *       ->
 *     parser
 *
 * must preserve function semantics.
 *
 * The adapter rule itself must not introduce information that cannot survive
 * normalization.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * The following identifiers MUST NOT occur as semantic limits:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * This file contains none of them as grammar productions.
 *
 * No fixed:
 *
 *     CPU count
 *     GPU count
 *     FPGA count
 *     QPU count
 *     qubit count
 *     node count
 *     thread count
 *     memory capacity
 *     register width
 *     tensor rank
 *     network size
 *
 * is permitted.
 *
 * ============================================================================
 * PRODUCTION-READINESS CHECKLIST
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] It is parser-only.
 *
 * [x] It uses the canonical ZamaniLexer vocabulary.
 *
 * [x] It imports the canonical Functions grammar.
 *
 * [x] It does not define a second function grammar.
 *
 * [x] It does not duplicate parameters.
 *
 * [x] It does not duplicate generic parameters.
 *
 * [x] It does not duplicate generic constraints.
 *
 * [x] It does not duplicate return types.
 *
 * [x] It does not duplicate effects.
 *
 * [x] It does not duplicate contracts.
 *
 * [x] It does not duplicate expressions.
 *
 * [x] It does not duplicate statements.
 *
 * [x] It does not duplicate blocks.
 *
 * [x] It does not introduce a HYBRID_FN keyword.
 *
 * [x] It does not introduce a QuantumFunction AST.
 *
 * [x] It does not introduce a HybridFunction AST.
 *
 * [x] It does not introduce a second quantum IR.
 *
 * [x] It preserves quantum::ir as the canonical quantum boundary.
 *
 * [x] It does not perform resource allocation.
 *
 * [x] It does not select hardware.
 *
 * [x] It does not perform routing.
 *
 * [x] It does not perform scheduling.
 *
 * [x] It does not implement QEC.
 *
 * [x] It does not implement ZQN.
 *
 * [x] It does not implement HAL behavior.
 *
 * [x] It contains no physical device assumptions.
 *
 * [x] It contains no fixed hardware limits.
 *
 * [x] It supports classical/quantum/accelerator hybrid semantics through
 *     existing canonical function and hybrid grammars.
 *
 * [x] It preserves POCO-REAF.
 *
 * [x] It requires no unsafe Rust.
 *
 * [x] It is compatible with the Rust 1.97 / 1.97.1 frontend baseline.
 *
 * ============================================================================
 * FINAL RULE
 * ============================================================================
 *
 * `hybrid-functions.g4` is an integration boundary, NOT a new function
 * language.
 *
 * The canonical function remains:
 *
 *     functionDeclaration
 *
 * The hybrid adapter is:
 *
 *     hybridFunctionDeclaration
 *         -> functionDeclaration
 *
 * Therefore:
 *
 *     one function syntax
 *     one Function AST
 *     one semantic function model
 *     one canonical IR architecture
 *
 * can support:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     accelerator
 *     distributed
 *     AI
 *     data
 *     networking
 *     future domains
 *
 * without rewriting the function grammar whenever a new computational
 * substrate appears.
 *
 * This is the required architecture for:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * ============================================================================
 */
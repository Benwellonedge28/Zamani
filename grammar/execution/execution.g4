/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/execution/execution.g4
 *
 * Grammar:
 *     Execution
 *
 * Status:
 *     Production execution-composition grammar
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar is the canonical source-level composition boundary for
 * execution intent.
 *
 * It defines WHAT it means syntactically to request execution of an already
 * described Zamani computation.
 *
 * It does NOT define HOW execution is realized.
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
 *     canonical parser
 *          |
 *          v
 *     frontend AST
 *          |
 *          +--> semantic analysis
 *          |       |
 *          |       +--> type analysis
 *          |       +--> effect analysis
 *          |       +--> capability analysis
 *          |       +--> resource analysis
 *          |       +--> target resolution
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
 *     compilation / optimization
 *          |
 *          +--> routing
 *          +--> scheduling
 *          +--> resilience
 *          +--> hardware HAL
 *          |
 *          v
 *     dispatch / deployment planning
 *          |
 *          v
 *     runtime
 *
 * ============================================================================
 * CORE PRINCIPLE
 * ============================================================================
 *
 * Execution syntax expresses execution intent.
 *
 * It MUST NOT permanently encode:
 *
 *     - a particular CPU;
 *     - a particular core;
 *     - a particular thread;
 *     - a particular GPU;
 *     - a particular FPGA;
 *     - a particular ASIC;
 *     - a particular QPU;
 *     - a particular simulator;
 *     - a particular vendor;
 *     - a physical device identifier;
 *     - a physical address;
 *     - a fixed topology;
 *     - a fixed number of devices;
 *     - a fixed number of qubits;
 *     - a fixed number of cores;
 *     - a fixed number of threads;
 *     - a fixed memory capacity;
 *     - a fixed network size;
 *     - a fixed accelerator count;
 *     - a fixed schedule;
 *     - a fixed placement;
 *     - a fixed deployment topology.
 *
 * Such information belongs to the appropriate semantic/resource/target/
 * hardware/runtime layer.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * The source program describes portable computation and execution intent.
 *
 * Physical realization is selected later from:
 *
 *     capabilities
 *     requirements
 *     constraints
 *     preferences
 *     hints
 *     resource availability
 *     target context
 *     scheduling context
 *     placement context
 *     dispatch context
 *     deployment context
 *
 * Therefore:
 *
 *     source semantics != physical realization
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - execution declaration composition;
 *     - execution subject composition;
 *     - execution-level optional context;
 *     - the boundary between an executable computation and execution intent;
 *     - integration points for specialized execution grammars.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical tokens;
 *     - identifiers;
 *     - expressions;
 *     - types;
 *     - declarations;
 *     - functions;
 *     - modules;
 *     - execution-context internals;
 *     - scheduling syntax;
 *     - placement syntax;
 *     - dispatch syntax;
 *     - synchronization syntax;
 *     - deployment syntax;
 *     - runtime capability syntax;
 *     - distributed execution syntax;
 *     - resource semantics;
 *     - hardware semantics;
 *     - target semantics;
 *     - routing;
 *     - optimization;
 *     - QEC;
 *     - ZQN;
 *     - resilience algorithms;
 *     - classical IR;
 *     - quantum::ir;
 *     - HDL IR;
 *     - runtime implementation;
 *     - backend APIs.
 *
 * ============================================================================
 * DEPENDENCY RULE
 * ============================================================================
 *
 * Lower-level language constructs are imported.
 *
 * Specialized execution grammars are composed here only through their public
 * entry points where doing so does not create a dependency cycle.
 *
 * The dependency direction is:
 *
 *     ZamaniLexer
 *          |
 *          v
 *        Core
 *          |
 *          v
 *   execution subgrammars
 *          |
 *          v
 *      Execution
 *          |
 *          v
 *   canonical parser
 *
 * Execution subgrammars MUST NOT depend on Execution merely to define their
 * own concepts.
 *
 * ============================================================================
 * IMPORTANT OWNERSHIP SEPARATION
 * ============================================================================
 *
 * The following files remain authoritative for their respective concepts:
 *
 *     execution-context.g4
 *         execution context structure
 *
 *     scheduling.g4
 *         scheduling intent
 *
 *     placement.g4
 *         placement intent
 *
 *     dispatch.g4
 *         dispatch intent
 *
 *     synchronization.g4
 *         execution-level synchronization intent
 *
 *     runtime-capabilities.g4
 *         runtime capability declarations/requirements
 *
 *     deployment.g4
 *         deployment intent
 *
 *     distributed/remote-execution.g4
 *         distributed remote-execution semantics
 *
 * Execution.g4 MUST NOT copy those grammars.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Quantum syntax remains owned by the quantum grammar/frontend.
 *
 * This grammar does not define:
 *
 *     qubits
 *     registers
 *     gates
 *     measurements
 *     quantum states
 *     physical qubits
 *     quantum topology
 *     pulses
 *     QEC
 *     noise
 *     ZQN
 *
 * Quantum source syntax is lowered through the canonical quantum semantic
 * pipeline and ultimately into quantum::ir.
 *
 * This grammar may execute a quantum computation because the execution subject
 * is deliberately generic.
 *
 * It does not become a second quantum IR.
 *
 * ============================================================================
 * CLASSICAL / HDL / DISTRIBUTED / AI BOUNDARY
 * ============================================================================
 *
 * The execution subject is intentionally domain-neutral.
 *
 * It can refer to semantic computations originating from:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     distributed
 *     AI
 *     data
 *     accelerator
 *     networking
 *     cryptographic
 *     scientific
 *     future
 *
 * domains without enumerating those domains here.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * No grammar-level finite machine limit exists here.
 *
 * This grammar contains no:
 *
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_RESOURCES
 *     MAX_TARGETS
 *     MAX_JOBS
 *     MAX_EXECUTIONS
 *     MAX_ARGUMENTS
 *     MAX_CONTEXT_ENTRIES
 *
 * Repetition and nesting are represented structurally by the grammar.
 *
 * Actual resource limits are determined by:
 *
 *     - parser resource policy;
 *     - semantic analysis;
 *     - compiler policy;
 *     - resource management;
 *     - target capabilities;
 *     - runtime policy;
 *     - operating-system limits;
 *     - explicitly declared constraints.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no semantic actions;
 *     - no predicates;
 *     - no filesystem operations;
 *     - no network operations;
 *     - no hardware discovery;
 *     - no runtime calls;
 *     - no random behavior;
 *     - no mutable global state.
 *
 * Given the same canonical token stream, parsing is deterministic.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This grammar contains no Rust code and therefore introduces no unsafe
 * operations.
 *
 * Generated/parser integration MUST remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *
 * No `unsafe` implementation is required by this grammar.
 *
 * ============================================================================
 */

parser grammar Execution;

options {
    tokenVocab = ZamaniLexer;
}

import Core, ExecutionContext;


/*
 * ============================================================================
 * 1. CANONICAL EXECUTION DECLARATION
 * ============================================================================
 *
 * Canonical conceptual forms:
 *
 *     execute computation;
 *
 *     execute computation with {
 *         ...
 *     };
 *
 * The computation is an existing language expression/semantic subject.
 *
 * Execution does not itself compile, schedule, route, deploy, or dispatch the
 * computation.
 *
 * ============================================================================
 */

executionDeclaration
    : EXECUTE executionRequest executionTerminator?
    ;


/*
 * ============================================================================
 * 2. EXECUTION REQUEST
 * ============================================================================
 *
 * The execution request consists of:
 *
 *     subject
 *     optional execution context
 *
 * The context is deliberately delegated to ExecutionContext.
 *
 * ============================================================================
 */

executionRequest
    : executionSubject executionContextAttachment?
    ;


/*
 * ============================================================================
 * 3. EXECUTION SUBJECT
 * ============================================================================
 *
 * The subject is an already-described computation.
 *
 * It is intentionally expression-based so this grammar does not need a
 * closed list of computational domains.
 *
 * Examples of semantic subjects include:
 *
 *     function calls
 *     named computations
 *     pipelines
 *     classical programs
 *     quantum programs
 *     hybrid programs
 *     hardware computations
 *     distributed computations
 *     AI pipelines
 *     accelerator computations
 *
 * The expression grammar remains authoritative for expression syntax.
 *
 * ============================================================================
 */

executionSubject
    : expression
    | blockExpression
    ;


/*
 * ============================================================================
 * 4. OPTIONAL EXECUTION CONTEXT
 * ============================================================================
 *
 * `with` is the explicit syntactic attachment point for execution context.
 *
 * ExecutionContext owns the internal context representation.
 *
 * Execution.g4 MUST NOT redefine:
 *
 *     context keys
 *     context values
 *     context objects
 *     context lists
 *     context comparisons
 *
 * ============================================================================
 */

executionContextAttachment
    : WITH executionContext
    ;


/*
 * ============================================================================
 * 5. EXECUTION TERMINATOR
 * ============================================================================
 *
 * Execution declarations use the canonical statement terminator from the
 * language grammar.
 *
 * The canonical token vocabulary owns the actual token.
 *
 * ============================================================================
 */

executionTerminator
    : SEMI
    ;


/*
 * ============================================================================
 * 6. SPECIALIZED EXECUTION INTEGRATION CONTRACT
 * ============================================================================
 *
 * Specialized execution grammars are NOT duplicated here.
 *
 * Their public entry points are consumed by the canonical parser composition
 * layer according to the language's source-level placement rules.
 *
 * The authoritative ownership is:
 *
 *     execution-context.g4
 *         -> executionContext
 *
 *     scheduling.g4
 *         -> scheduling intent
 *
 *     placement.g4
 *         -> placement intent
 *
 *     dispatch.g4
 *         -> dispatchDeclaration
 *
 *     synchronization.g4
 *         -> synchronization intent
 *
 *     runtime-capabilities.g4
 *         -> runtime capability intent
 *
 *     deployment.g4
 *         -> deployment intent
 *
 * This file does not reproduce those rules.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 7. SEMANTIC INTEGRATION CONTRACT
 * ============================================================================
 *
 * Parsing produces syntax nodes only.
 *
 * Conceptual AST:
 *
 *     ExecutionDeclaration
 *         |
 *         +--> Subject
 *         |
 *         +--> Optional ExecutionContext
 *
 * Semantic analysis transforms that structure into execution intent.
 *
 * Conceptually:
 *
 *     ExecutionDeclaration
 *             |
 *             v
 *     ExecutionIntent
 *             |
 *       +-----+------+-------------------+
 *       |            |                   |
 *       v            v                   v
 *   capability    resources           target
 *     analysis      analysis          resolution
 *       |            |                   |
 *       +------------+-------------------+
 *                    |
 *                    v
 *          canonical semantic model
 *
 * No runtime action occurs during parsing.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 8. IR INTEGRATION
 * ============================================================================
 *
 * Execution.g4 does not define an IR.
 *
 * The subject is lowered through the appropriate canonical semantic pipeline:
 *
 *     classical subject
 *          -> classical semantic/IR pipeline
 *
 *     quantum subject
 *          -> quantum frontend
 *          -> quantum::ir
 *
 *     HDL subject
 *          -> HDL/hardware semantic representation
 *
 *     distributed subject
 *          -> distributed semantic representation
 *
 *     hybrid subject
 *          -> combined canonical semantic representation
 *
 * Execution intent remains orthogonal to those representations.
 *
 * Therefore:
 *
 *     grammar -> AST -> semantic model -> IR
 *
 * and never:
 *
 *     grammar -> custom execution IR -> other IR
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 9. COMPILATION INTEGRATION
 * ============================================================================
 *
 * Compilation is separate from execution.
 *
 * Execution.g4 may identify the computation whose compiled representation is
 * ultimately executed, but it does not define:
 *
 *     compiler targets
 *     optimization passes
 *     lowering algorithms
 *     code generation
 *     target-specific instruction selection
 *
 * Those remain owned by grammar/compile and the compiler implementation.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 10. SCHEDULING INTEGRATION
 * ============================================================================
 *
 * Execution.g4 does not implement scheduling.
 *
 * It does not calculate:
 *
 *     start times
 *     end times
 *     dependencies
 *     resource occupancy
 *     critical paths
 *     ASAP schedules
 *     ALAP schedules
 *     RCPSP schedules
 *     pulse schedules
 *
 * Scheduling intent is consumed from the dedicated scheduling grammar and
 * resolved later by the scheduling subsystem.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 11. PLACEMENT / ROUTING INTEGRATION
 * ============================================================================
 *
 * Execution.g4 does not map computation onto physical resources.
 *
 * In particular it does not select:
 *
 *     CPU cores
 *     GPU devices
 *     FPGA regions
 *     ASIC instances
 *     QPU qubits
 *     cluster nodes
 *     network paths
 *
 * Placement and routing are downstream realization concerns.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 12. DISPATCH INTEGRATION
 * ============================================================================
 *
 * Dispatch remains owned by:
 *
 *     grammar/execution/dispatch.g4
 *
 * Execution.g4 MUST NOT reimplement dispatch declarations.
 *
 * The canonical integration contract is:
 *
 *     execution
 *          |
 *          +--> semantic execution intent
 *                         |
 *                         v
 *                    dispatch intent
 *                         |
 *                         v
 *                    dispatch plan
 *
 * The dispatch plan is not part of the source grammar AST.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 13. DEPLOYMENT INTEGRATION
 * ============================================================================
 *
 * Deployment remains owned by:
 *
 *     grammar/execution/deployment.g4
 *
 * Execution.g4 does not define:
 *
 *     replicas
 *     rollout algorithms
 *     service deployment
 *     provider APIs
 *     cluster topology
 *     node allocation
 *     container implementation
 *     cloud implementation
 *
 * Deployment intent is resolved downstream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 14. RESILIENCE INTEGRATION
 * ============================================================================
 *
 * Execution syntax may eventually carry references to resilience policy
 * through the execution-context system.
 *
 * Execution.g4 does not implement:
 *
 *     retry algorithms
 *     rollback
 *     checkpoint reconstruction
 *     fault diagnosis
 *     mitigation
 *     QEC
 *     ZQN
 *     backend switching
 *
 * Those remain outside the grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 15. RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Resource and capability information is interpreted semantically.
 *
 * Execution.g4 does not define physical resource inventories.
 *
 * For example, source semantics may require:
 *
 *     quantum capability
 *
 * without specifying:
 *
 *     a particular QPU
 *     a particular vendor
 *     a particular qubit identifier
 *     a fixed qubit count
 *
 * Similarly, a computation may express a resource relationship without
 * embedding a fixed machine capacity.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 16. FUTURE EXTENSIBILITY
 * ============================================================================
 *
 * New computational domains MUST NOT require modification to this grammar
 * merely because a new kind of machine becomes available.
 *
 * A future domain should be able to provide its own semantic grammar and
 * integrate through:
 *
 *     executionSubject
 *     expression
 *     executionContext
 *
 * or the canonical parser composition layer.
 *
 * The execution grammar therefore remains stable while the set of realizable
 * computing technologies grows.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 17. ERROR-BOUNDARY CONTRACT
 * ============================================================================
 *
 * Syntax errors belong to the parser.
 *
 * Semantic errors belong to semantic analysis.
 *
 * Examples:
 *
 *     Syntax error:
 *         malformed execution declaration
 *
 *     Semantic error:
 *         referenced computation does not exist
 *
 *     Type error:
 *         execution subject has an invalid semantic type
 *
 *     Capability error:
 *         required capability cannot be satisfied
 *
 *     Resource error:
 *         declared requirement cannot be satisfied
 *
 *     Target error:
 *         no compatible target exists
 *
 *     Runtime error:
 *         execution environment fails
 *
 * Execution.g4 MUST NOT attempt to collapse these distinct error classes.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 18. DETERMINISM / REPRODUCIBILITY CONTRACT
 * ============================================================================
 *
 * The parser must produce equivalent syntax structure for equivalent canonical
 * token streams.
 *
 * There is no:
 *
 *     timestamp generation
 *     UUID generation
 *     device discovery
 *     backend selection
 *     randomization
 *     runtime inspection
 *
 * inside this grammar.
 *
 * Reproducibility is therefore delegated to deterministic downstream semantic
 * and compilation policies.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 19. SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar scales structurally rather than by enumerating machine sizes.
 *
 * A single execution subject can represent a tiny computation or a semantic
 * computation whose eventual realization consumes arbitrarily many resources.
 *
 * The grammar imposes no fixed limit on:
 *
 *     program size
 *     expression complexity
 *     context nesting
 *     resource cardinality
 *     target cardinality
 *     device cardinality
 *     node cardinality
 *     qubit cardinality
 *     accelerator cardinality
 *
 * Subject only to actual parser/runtime memory and time resources and to
 * explicitly defined implementation safeguards.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 20. HARD-CODING AUDIT
 * ============================================================================
 *
 * No machine-size constant is present.
 *
 * No:
 *
 *     MAX_*
 *     device ID
 *     physical address
 *     CPU count
 *     GPU count
 *     FPGA count
 *     QPU count
 *     qubit count
 *     node count
 *     memory size
 *     topology
 *     queue size
 *
 * is encoded in this grammar.
 *
 * Any future machine-specific requirement MUST be represented through the
 * appropriate target/resource/capability/constraint system rather than added
 * as a grammar-level limit.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 21. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when:
 *
 * [ ] It parses the canonical execution declaration.
 *
 * [ ] It reuses the canonical lexer.
 *
 * [ ] It reuses Core expression/block syntax.
 *
 * [ ] It reuses ExecutionContext rather than duplicating context syntax.
 *
 * [ ] It contains no machine-size limits.
 *
 * [ ] It contains no hardware discovery.
 *
 * [ ] It contains no runtime behavior.
 *
 * [ ] It contains no scheduling implementation.
 *
 * [ ] It contains no placement implementation.
 *
 * [ ] It contains no routing implementation.
 *
 * [ ] It contains no dispatch implementation.
 *
 * [ ] It contains no deployment implementation.
 *
 * [ ] It does not duplicate quantum syntax.
 *
 * [ ] It does not create a second quantum IR.
 *
 * [ ] It has no semantic actions.
 *
 * [ ] It has no unsafe Rust dependency.
 *
 * [ ] It passes parser generation with the canonical ZamaniLexer.
 *
 * [ ] It passes Rust 1.97 / 1.97.1 compilation of the generated parser.
 *
 * [ ] Positive execution tests pass.
 *
 * [ ] Negative syntax tests pass.
 *
 * [ ] Cross-domain execution subjects parse.
 *
 * [ ] Large/repeated execution structures do not encounter artificial
 *     grammar-level machine limits.
 *
 * [ ] Parser output is deterministic.
 *
 * [ ] AST lowering has a documented ExecutionDeclaration ->
 *     ExecutionIntent contract.
 *
 * [ ] No downstream file needs to modify the fundamental ownership model
 *     established here.
 *
 * ============================================================================
 */
/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/functions/functions.g4
 *
 * Status:
 *     CANONICAL FUNCTION DECLARATION COMPOSITION GRAMMAR
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
 * This file is the canonical source-language composition boundary for named
 * function declarations.
 *
 * It owns FUNCTION DECLARATION FRAMING.
 *
 * It does not own the implementation of:
 *
 *     - identifiers;
 *     - qualified names;
 *     - attributes;
 *     - generic parameters;
 *     - parameters;
 *     - return types;
 *     - generic constraints;
 *     - effects;
 *     - contracts;
 *     - types;
 *     - expressions;
 *     - statements;
 *     - blocks;
 *     - closures;
 *     - lambdas;
 *     - generators;
 *     - foreign-function syntax;
 *     - compile-time-function syntax;
 *     - resource syntax;
 *     - capability syntax;
 *     - hardware syntax;
 *     - quantum syntax;
 *     - HDL syntax.
 *
 * Those concepts remain owned by their canonical grammar components.
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
 *     Functions
 *          |
 *          +-----------------------------+
 *          |             |               |
 *          v             v               v
 *       Types       Expressions      Statements
 *          |             |               |
 *          +-------------+---------------+
 *                        |
 *                        v
 *              domain-neutral AST
 *                        |
 *                        v
 *               structural validation
 *                        |
 *                        v
 *                semantic analysis
 *                        |
 *          +-------------+-------------+
 *          |             |             |
 *          v             v             v
 *     Classical IR   quantum::ir   HDL/Hardware
 *          |             |             |
 *          +-------------+-------------+
 *                        |
 *                        v
 *                   optimization
 *                        |
 *             +----------+----------+
 *             |          |          |
 *             v          v          v
 *          routing   scheduling   resilience
 *                                   |
 *                                   v
 *                                  QEC
 *                                   |
 *                                   v
 *                                  ZQN
 *                                   |
 *                                   v
 *                                  HAL
 *                                   |
 *                                   v
 *                            target realization
 *
 * Functions are therefore source-level semantic units, not machine functions.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * This file is the ONLY owner of the framing of ordinary named functions.
 *
 * It owns:
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
 * It MUST NOT redefine rules owned elsewhere.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * Canonical dependencies:
 *
 *     Attributes
 *         -> attribute
 *
 *     Names
 *         -> identifier
 *
 *     FunctionGenerics
 *         -> functionGenericParameters
 *
 *     Parameters
 *         -> parameterList
 *
 *     ReturnTypes
 *         -> functionReturnClause
 *
 *     FunctionConstraints
 *         -> functionConstraintClause
 *
 *     FunctionContracts
 *         -> functionContractClause
 *
 *     Effects
 *         -> effectClause
 *
 *     Types
 *         -> typeExpression and related type rules
 *
 *     Expressions
 *         -> expression and related expression rules
 *
 *     Statements
 *         -> block and statement composition
 *
 * The imported grammars are the authoritative owners of those constructs.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * Parser delegates in the current grammar architecture consume the canonical
 * Zamani token vocabulary:
 *
 *     ZamaniTokens
 *
 * The production lexer boundary remains:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * which composes the lexical vocabulary represented by:
 *
 *     grammar/lexer/tokens.g4
 *
 * This file therefore uses:
 *
 *     tokenVocab = ZamaniTokens;
 *
 * This prevents this delegate from creating a second parser-token vocabulary
 * or coupling itself to a generated lexer implementation detail.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * A function is portable source-level computation.
 *
 * This grammar places NO universal limit on:
 *
 *     - functions;
 *     - parameters;
 *     - generic parameters;
 *     - constraints;
 *     - attributes;
 *     - contracts;
 *     - effects;
 *     - statements;
 *     - expressions;
 *     - calls;
 *     - recursion;
 *     - domains;
 *     - quantum resources;
 *     - classical resources;
 *     - distributed resources;
 *     - hardware resources;
 *     - source size.
 *
 * Repetition is represented structurally by ANTLR repetition operators.
 *
 * The grammar MUST NOT encode:
 *
 *     MAX_FUNCTIONS
 *     MAX_PARAMETERS
 *     MAX_GENERIC_PARAMETERS
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_STORAGE
 *     MAX_DEVICES
 *     MAX_ACCELERATORS
 *     MAX_TENSOR_SIZE
 *
 * as language rules.
 *
 * Any implementation resource limit belongs to compiler/runtime policy and
 * MUST NOT change the language's semantic meaning.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This grammar MUST NOT encode universal concepts such as:
 *
 *     cpu0
 *     gpu0
 *     qpu0
 *     fpga0
 *     physical_qubit0
 *     device0
 *     memory_bank0
 *
 * Nor may it encode:
 *
 *     exactly N CPUs
 *     exactly N GPUs
 *     exactly N QPUs
 *     exactly N qubits
 *     exactly N threads
 *
 * Function syntax describes intent.
 *
 * Resource realization belongs downstream.
 *
 * ============================================================================
 * DOMAIN NEUTRALITY
 * ============================================================================
 *
 * One function grammar serves all Zamani computational domains.
 *
 * The same function declaration model can represent:
 *
 *     classical computation
 *     quantum computation
 *     hybrid computation
 *     HDL/co-design computation
 *     hardware-aware computation
 *     distributed computation
 *     parallel computation
 *     AI/ML computation
 *     data processing
 *     networking
 *     cryptography
 *     scientific computation
 *     edge/cloud computation
 *     future computational domains
 *
 * Domain meaning is determined by:
 *
 *     types
 *     effects
 *     capabilities
 *     resources
 *     declarations
 *     semantic analysis
 *     domain-specific lowering
 *
 * This grammar MUST NOT create:
 *
 *     cpuFunction
 *     gpuFunction
 *     qpuFunction
 *     fpgaFunction
 *     cudaFunction
 *     quantumFunction
 *     aiFunction
 *     hdlFunction
 *
 * as competing function languages.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum functions remain ordinary Zamani functions.
 *
 * Quantum-specific meaning may enter through:
 *
 *     - parameter types;
 *     - return types;
 *     - expressions;
 *     - statements;
 *     - effects;
 *     - capabilities;
 *     - resource requirements;
 *     - domain declarations;
 *     - attributes.
 *
 * This grammar MUST NOT:
 *
 *     - enumerate quantum gates;
 *     - allocate physical qubits;
 *     - define topology;
 *     - perform routing;
 *     - perform scheduling;
 *     - implement QEC;
 *     - implement ZQN;
 *     - implement HAL;
 *     - create a second quantum IR.
 *
 * Required downstream path:
 *
 *     function syntax
 *          ->
 *     domain-neutral AST
 *          ->
 *     semantic function model
 *          ->
 *     quantum semantics
 *          ->
 *     quantum::ir
 *          ->
 *     optimization
 *          ->
 *     routing
 *          ->
 *     scheduling
 *          ->
 *     QEC / resilience
 *          ->
 *     ZQN
 *          ->
 *     HAL
 *          ->
 *     target realization
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Functions may participate in software/hardware co-design.
 *
 * This grammar does not encode:
 *
 *     - fixed register widths;
 *     - fixed bus widths;
 *     - fixed port counts;
 *     - fixed FPGA resources;
 *     - fixed ASIC resources;
 *     - fixed accelerator counts;
 *     - fixed clock counts;
 *     - fixed pipeline depth;
 *     - physical topology;
 *     - physical addresses;
 *     - device identifiers.
 *
 * Hardware intent belongs to:
 *
 *     grammar/hardware/
 *     grammar/hdl/
 *     grammar/resources/
 *     grammar/compile/
 *     grammar/execution/
 *
 * and their downstream semantic systems.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Function declarations may participate in resource and capability semantics
 * through the surrounding language architecture.
 *
 * Examples of portable intent include:
 *
 *     requires a capability
 *     requires a resource
 *     requires an effect
 *     prefers an execution capability
 *     constrains an execution property
 *
 * This grammar does not define the resource semantics.
 *
 * It only provides the function declaration boundary within which those
 * constructs can be attached by the authoritative grammar components.
 *
 * Semantic analysis distinguishes:
 *
 *     requirement
 *     constraint
 *     capability
 *     preference
 *     hint
 *     implementation decision
 *
 * A source-level requirement is NOT a physical resource assignment.
 *
 * ============================================================================
 * FUNCTION MODIFIERS
 * ============================================================================
 *
 * The native frontend AST intentionally represents function modifiers as
 * extensible namespaced values rather than a closed backend-specific enum.
 *
 * Therefore this grammar distinguishes:
 *
 *     core declaration modifiers
 *
 * from:
 *
 *     extensible function metadata.
 *
 * Core modifiers that already have canonical lexical tokens may be accepted
 * here.
 *
 * Domain-specific and future modifiers MUST use the attribute/extension
 * mechanism rather than requiring this file to grow a finite catalogue.
 *
 * This preserves extensibility without creating parser forks.
 *
 * ============================================================================
 * ASYNC INTEGRATION
 * ============================================================================
 *
 * Async function declarations use the canonical ASYNC modifier token.
 *
 * Async expression/await syntax remains owned by:
 *
 *     grammar/functions/async.g4
 *     grammar/expressions/
 *
 * This file does not select:
 *
 *     - executor;
 *     - scheduler;
 *     - worker count;
 *     - thread count;
 *     - task placement;
 *     - CPU;
 *     - GPU;
 *     - accelerator.
 *
 * Those decisions are downstream.
 *
 * ============================================================================
 * GENERATOR INTEGRATION
 * ============================================================================
 *
 * Generator/yield syntax remains owned by:
 *
 *     grammar/functions/generators.g4
 *
 * A generator is therefore syntactically a function whose body contains
 * generator constructs.
 *
 * Semantic analysis determines:
 *
 *     - whether yielding is legal;
 *     - yielded value type;
 *     - resulting generator type;
 *     - suspension semantics;
 *     - runtime realization.
 *
 * ============================================================================
 * CLOSURE / LAMBDA INTEGRATION
 * ============================================================================
 *
 * Closures and lambdas are expressions, not named function declarations.
 *
 * Their syntax remains owned by:
 *
 *     grammar/functions/closures.g4
 *     grammar/functions/lambdas.g4
 *     grammar/expressions/
 *
 * This file does not duplicate them.
 *
 * ============================================================================
 * FOREIGN FUNCTION INTEGRATION
 * ============================================================================
 *
 * Foreign declarations remain interoperable function declarations, but their
 * foreign-language details belong to:
 *
 *     grammar/functions/foreign-functions.g4
 *     grammar/interoperability/
 *
 * `EXTERN` is therefore a declaration-level modifier only.
 *
 * ABI, symbol naming, representation, calling convention, and foreign-language
 * semantics are downstream/interoperability concerns.
 *
 * ============================================================================
 * COMPILE-TIME FUNCTION INTEGRATION
 * ============================================================================
 *
 * Compile-time functions remain part of the universal function model.
 *
 * Specialized compile-time syntax belongs to:
 *
 *     grammar/functions/compile-time-functions.g4
 *     grammar/expressions/compile-time.g4
 *     grammar/metaprogramming/
 *
 * This file does not create a second compile-time function language.
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Contracts are attached through:
 *
 *     functionContractClause
 *
 * from:
 *
 *     FunctionContracts
 *
 * Contract syntax is not interpreted here.
 *
 * Contract expressions remain ordinary canonical expressions where the
 * contract grammar requires them.
 *
 * ============================================================================
 * GENERIC INTEGRATION
 * ============================================================================
 *
 * Function generic parameter syntax is owned by:
 *
 *     FunctionGenerics
 *
 * Function generic constraints are owned by:
 *
 *     FunctionConstraints
 *
 * Generic solving, specialization, inference, substitution, trait solving,
 * capability solving, and resource solving are semantic/compiler concerns.
 *
 * ============================================================================
 * PARAMETER INTEGRATION
 * ============================================================================
 *
 * Parameter syntax is owned by:
 *
 *     Parameters
 *
 * This allows functions to accept:
 *
 *     classical values
 *     quantum values
 *     tensors
 *     resources
 *     capabilities
 *     references
 *     distributed values
 *     hardware abstractions
 *     future domain values
 *
 * without modifying this function grammar.
 *
 * ============================================================================
 * RETURN-TYPE INTEGRATION
 * ============================================================================
 *
 * Return syntax is owned by:
 *
 *     ReturnTypes
 *
 * This file only controls where the return clause occurs.
 *
 * ============================================================================
 * BODY INTEGRATION
 * ============================================================================
 *
 * Function bodies use the canonical `block` rule supplied through the
 * statement/core composition hierarchy.
 *
 * This file MUST NOT duplicate:
 *
 *     `{`
 *     `}`
 *     statement
 *     expression
 *     block element
 *
 * syntax.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every function declaration MUST map to the existing domain-neutral frontend
 * AST.
 *
 * The existing AST function representation stores:
 *
 *     - source node metadata;
 *     - function name;
 *     - visibility;
 *     - extensible modifiers;
 *     - generic parameter NodeIds;
 *     - parameter NodeIds;
 *     - optional return-type NodeId;
 *     - body NodeId;
 *     - effects;
 *     - capabilities;
 *     - contracts.
 *
 * The parser/frontend integration must therefore preserve all syntactically
 * available function information.
 *
 * This grammar MUST NOT introduce:
 *
 *     QuantumFunction
 *     CpuFunction
 *     GpuFunction
 *     QpuFunction
 *     HdlFunction
 *     HardwareFunction
 *
 * or another parallel AST.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * This grammar establishes structure only.
 *
 * Semantic analysis owns:
 *
 *     - duplicate modifiers;
 *     - modifier compatibility;
 *     - visibility legality;
 *     - declaration context;
 *     - generic validity;
 *     - parameter validity;
 *     - return-type validity;
 *     - effect validity;
 *     - contract validity;
 *     - capability validity;
 *     - resource requirements;
 *     - ownership;
 *     - borrowing;
 *     - lifetime rules;
 *     - recursion policy;
 *     - async legality;
 *     - generator legality;
 *     - foreign declaration legality;
 *     - compile-time restrictions;
 *     - domain legality;
 *     - determinism requirements;
 *     - portability validation.
 *
 * The parser remains context-light.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * Function syntax MUST NOT lower directly to a backend.
 *
 * Required conceptual path:
 *
 *     function grammar
 *          ->
 *     domain-neutral AST
 *          ->
 *     semantic function model
 *          ->
 *     canonical semantic IR
 *
 * Then:
 *
 *     classical -> classical IR
 *     quantum   -> quantum::ir
 *     HDL       -> HDL/hardware semantic representation
 *     hybrid    -> appropriate combined semantic representation
 *
 * Never:
 *
 *     grammar -> LLVM
 *     grammar -> QIR
 *     grammar -> MLIR
 *     grammar -> CUDA
 *     grammar -> vendor assembly
 *     grammar -> physical qubit mapping
 *     grammar -> scheduler
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing this grammar depends only on:
 *
 *     - source token sequence;
 *     - grammar version;
 *     - imported grammar versions;
 *     - canonical token vocabulary.
 *
 * It MUST NOT depend on:
 *
 *     - wall-clock time;
 *     - randomness;
 *     - filesystem state;
 *     - network state;
 *     - environment state;
 *     - hardware availability;
 *     - runtime state.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar contains no executable actions.
 *
 * It performs no:
 *
 *     - filesystem access;
 *     - network access;
 *     - process execution;
 *     - dynamic library loading;
 *     - hardware access;
 *     - secret access;
 *     - runtime execution.
 *
 * The Rust compiler/frontend consuming this grammar must remain safe Rust and
 * must not require `unsafe`.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * Structural repetition is used wherever the language has no semantic
 * cardinality restriction:
 *
 *     attribute*
 *     functionModifier*
 *     functionContractClause*
 *     parameterList
 *     genericParameterList
 *     constraintList
 *
 * The grammar therefore scales subject only to actual compiler/runtime
 * resources.
 *
 * A parser implementation may impose operational resource policies, but those
 * policies are NOT language-level semantic limits.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing filename retained:
 *
 *     grammar/functions/functions.g4
 *
 * Existing function rule names retained:
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
 * Existing delegate grammars remain independently authoritative.
 *
 * This file does not rename or absorb them.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] Function declaration framing has one owner.
 *
 * [x] Function signatures have one canonical composition.
 *
 * [x] Function names come from the canonical name grammar.
 *
 * [x] Generic parameters come from FunctionGenerics.
 *
 * [x] Parameters come from Parameters.
 *
 * [x] Return types come from ReturnTypes.
 *
 * [x] Generic constraints come from FunctionConstraints.
 *
 * [x] Effects come from Effects.
 *
 * [x] Contracts come from FunctionContracts.
 *
 * [x] Types come from Types.
 *
 * [x] Expressions come from Expressions.
 *
 * [x] Bodies come from the canonical block composition.
 *
 * [x] No lexer rules are duplicated.
 *
 * [x] No backend is selected by the grammar.
 *
 * [x] No hardware limit is encoded.
 *
 * [x] No physical resource is assigned.
 *
 * [x] No quantum gate list is encoded.
 *
 * [x] No second quantum IR is introduced.
 *
 * [x] No domain-specific function grammar fork is introduced.
 *
 * [x] The grammar is compatible with domain-neutral Function AST design.
 *
 * [x] The grammar is compatible with quantum::ir as the canonical quantum
 *     semantic boundary.
 *
 * [x] The grammar remains compatible with Rust 1.97 / 1.97.1 safe frontend
 *     implementation.
 *
 * [x] POCO-REAF is preserved.
 *
 * ============================================================================
 * FINAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * A Zamani function describes:
 *
 *     WHAT computation exists
 *     WHAT its interface is
 *     WHAT source-level properties apply
 *
 * It does NOT prescribe:
 *
 *     WHICH CPU
 *     WHICH GPU
 *     WHICH QPU
 *     WHICH FPGA
 *     WHICH physical qubit
 *     WHICH memory bank
 *     WHICH network node
 *     WHICH accelerator
 *
 * The compiler, semantic/resource system, scheduler, router, resilience
 * subsystem, ZQN, HAL, runtime and deployment layers determine realization.
 *
 * Therefore:
 *
 *     Program Once
 *          ->
 *     Compile Once
 *          ->
 *     Run Everywhere
 *          ->
 *     Run Anywhere
 *          ->
 *     Run Forever
 *
 * subject to program semantics and resources actually available at realization.
 *
 * ============================================================================
 */

parser grammar Functions;

options {
    tokenVocab = ZamaniTokens;
}

import
    Attributes,
    Names,
    FunctionGenerics,
    Parameters,
    ReturnTypes,
    FunctionConstraints,
    FunctionContracts,
    Effects,
    Types,
    Expressions,
    Statements
    ;

/*
 * ============================================================================
 * 1. FUNCTION DECLARATION
 * ============================================================================
 *
 * This is the public function-declaration entry point consumed by declaration
 * and module composition.
 *
 * A named function has exactly one signature followed by one implementation
 * boundary.
 */
functionDeclaration
    : functionSignature functionImplementation
    ;

/*
 * ============================================================================
 * 2. FUNCTION SIGNATURE
 * ============================================================================
 *
 * This intentionally remains a thin named boundary.
 *
 * The complete signature is defined by functionSignatureCore.
 *
 * Keeping this named rule allows downstream grammar consumers and tooling to
 * depend on a stable rule name without owning the composition themselves.
 */
functionSignature
    : functionSignatureCore
    ;

/*
 * ============================================================================
 * 3. FUNCTION SIGNATURE CORE
 * ============================================================================
 *
 * Canonical source order:
 *
 *     attributes*
 *     modifiers*
 *     fn
 *     name
 *     generic-parameters?
 *     parameter-list
 *     return-clause?
 *     constraints?
 *     effects?
 *     contracts*
 *
 * This order is stable and is the source-level function declaration contract.
 */
functionSignatureCore
    : attribute*
      functionModifier*
      FN
      functionName
      functionGenericParameters?
      LPAREN
      parameterList?
      RPAREN
      functionReturnClause?
      functionConstraintClause?
      effectClause?
      functionContractClause*
    ;

/*
 * ============================================================================
 * 4. FUNCTION NAME
 * ============================================================================
 *
 * Name syntax remains owned by Names.
 */
functionName
    : identifier
    ;

/*
 * ============================================================================
 * 5. CORE FUNCTION MODIFIERS
 * ============================================================================
 *
 * These are declaration-level modifiers already represented by the canonical
 * lexical vocabulary.
 *
 * Domain-specific/future behavior must use the attribute/extension mechanism
 * rather than expanding this closed list indefinitely.
 *
 * Semantic analysis determines:
 *
 *     - duplicate modifiers;
 *     - incompatible combinations;
 *     - declaration-context legality.
 */
functionModifier
    : PUBLIC
    | PUB
    | PRIVATE
    | PROTECTED
    | STATIC
    | CONST
    | ASYNC
    | EXTERN
    | ABSTRACT
    | FINAL
    | VIRTUAL
    | OVERRIDE
    | INLINE
    | VOLATILE
    | PURE
    ;

/*
 * ============================================================================
 * 6. FUNCTION IMPLEMENTATION BOUNDARY
 * ============================================================================
 *
 * A function may be:
 *
 *     - defined with a block;
 *     - declared without a body.
 *
 * Whether a body-less function is legal is a semantic/declaration-context
 * concern. For example:
 *
 *     extern
 *     abstract
 *     interface
 *     trait
 *     generated
 *     declaration-only
 *
 * may permit it according to their respective contracts.
 *
 * This grammar deliberately does not encode backend-specific rules.
 */
functionImplementation
    : block
    | SEMICOLON
    ;

/*
 * ============================================================================
 * 7. FUNCTION DEFINITION
 * ============================================================================
 *
 * Explicit definition-only adapter.
 *
 * It reuses the canonical signature and body rules.
 */
functionDefinition
    : functionSignature block
    ;

/*
 * ============================================================================
 * 8. FUNCTION PROTOTYPE
 * ============================================================================
 *
 * Explicit declaration-only adapter.
 */
functionPrototype
    : functionSignature SEMICOLON
    ;

/*
 * ============================================================================
 * 9. BODY DECLARATION ADAPTER
 * ============================================================================
 *
 * Stable reusable definition boundary for consumers that specifically require
 * a function body.
 */
functionBodyDeclaration
    : functionSignatureCore block
    ;

/*
 * ============================================================================
 * 10. PROTOTYPE DECLARATION ADAPTER
 * ============================================================================
 *
 * Stable reusable declaration-only boundary.
 */
functionPrototypeDeclaration
    : functionSignatureCore SEMICOLON
    ;
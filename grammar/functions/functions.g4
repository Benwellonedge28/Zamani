/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/functions/functions.g4
 *
 * Grammar:
 *     Functions
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
 * This file is the SINGLE SOURCE-LEVEL COMPOSITION OWNER for ordinary
 * named function declarations.
 *
 * It assembles the already-existing independent function grammar contracts:
 *
 *     grammar/core/attributes.g4
 *     grammar/core/names.g4
 *     grammar/functions/generics.g4
 *     grammar/functions/parameters.g4
 *     grammar/functions/return-types.g4
 *     grammar/functions/constraints.g4
 *     grammar/functions/contracts.g4
 *     grammar/effects/effects.g4
 *     grammar/functions/async.g4
 *     grammar/types/types.g4
 *     grammar/expressions/expressions.g4
 *     grammar/statements/statements.g4
 *
 * This file MUST NOT recreate rules owned by those grammars.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         Zamani source
 *                              |
 *                              v
 *                    grammar/antlr/ZamaniLexer.g4
 *                              |
 *                              v
 *                       Zamani parser
 *                              |
 *                              v
 *                   +-----------------------+
 *                   |       Functions       |
 *                   |       THIS FILE       |
 *                   +-----------------------+
 *                              |
 *             +----------------+----------------+
 *             |                |                |
 *             v                v                v
 *          Types          Expressions       Statements
 *             |                |                |
 *             +----------------+----------------+
 *                              |
 *                              v
 *                     domain-neutral AST
 *                              |
 *                              v
 *                    structural validation
 *                              |
 *                              v
 *                     semantic analysis
 *                              |
 *             +----------------+----------------+
 *             |                |                |
 *             v                v                v
 *       Classical IR       quantum::ir      HDL/Hardware IR
 *                              |
 *                              v
 *                         optimization
 *                              |
 *                    routing / scheduling
 *                              |
 *                     resilience / QEC
 *                              |
 *                             ZQN
 *                              |
 *                             HAL
 *                              |
 *                      target realization
 *
 * The function grammar is therefore source-language syntax only.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - functionDeclaration;
 *   - functionSignature;
 *   - functionSignatureCore;
 *   - functionDefinition;
 *   - functionPrototype;
 *   - functionBodyDeclaration;
 *   - functionPrototypeDeclaration;
 *   - functionImplementation;
 *   - functionName;
 *   - functionModifier;
 *   - the ordering of function-level source constructs;
 *   - attachment of attributes to functions;
 *   - attachment of generic parameters to functions;
 *   - attachment of parameters to functions;
 *   - attachment of return clauses to functions;
 *   - attachment of generic constraints to functions;
 *   - attachment of effect clauses to functions;
 *   - attachment of contracts to functions;
 *   - the declaration/body boundary.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexical tokens;
 *   - keyword spellings;
 *   - punctuation;
 *   - identifiers;
 *   - qualified-name syntax;
 *   - attributes themselves;
 *   - generic-parameter internals;
 *   - parameter internals;
 *   - type-expression internals;
 *   - expression internals;
 *   - statement internals;
 *   - block internals;
 *   - effect-set internals;
 *   - contract internals;
 *   - generic-constraint internals;
 *   - lambda syntax;
 *   - closure syntax;
 *   - generator syntax;
 *   - foreign-function syntax;
 *   - compile-time-function semantics;
 *   - module resolution;
 *   - name resolution;
 *   - overload resolution;
 *   - type inference;
 *   - generic substitution;
 *   - trait solving;
 *   - ownership analysis;
 *   - borrowing;
 *   - lifetime analysis;
 *   - effect checking;
 *   - capability checking;
 *   - resource discovery;
 *   - hardware discovery;
 *   - target selection;
 *   - ABI selection;
 *   - calling-convention selection;
 *   - optimization;
 *   - routing;
 *   - scheduling;
 *   - calibration;
 *   - QEC;
 *   - ZQN;
 *   - HAL;
 *   - runtime execution;
 *   - quantum IR.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * The following rules deliberately remain in their existing dedicated files:
 *
 *     functionGenericParameters
 *         -> grammar/functions/generics.g4
 *
 *     parameterList
 *         -> grammar/functions/parameters.g4
 *
 *     functionReturnClause
 *         -> grammar/functions/return-types.g4
 *
 *     functionConstraintClause
 *         -> grammar/functions/constraints.g4
 *
 *     functionContractClause
 *         -> grammar/functions/contracts.g4
 *
 *     effectClause
 *         -> grammar/effects/effects.g4
 *
 *     typeExpression
 *         -> grammar/types/types.g4
 *
 *     expression
 *         -> grammar/expressions/expressions.g4
 *
 *     block
 *         -> grammar/core/blocks.g4
 *           reached through the canonical statement composition.
 *
 *     identifier
 *         -> grammar/core/names.g4
 *
 *     attribute
 *         -> grammar/core/attributes.g4
 *
 * This file MUST NOT redefine any of those rules.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The parser consumes the canonical generated lexer vocabulary:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * ZamaniLexer itself composes:
 *
 *     grammar/lexer/tokens.g4
 *
 * Therefore this file uses:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * This file MUST NOT define lexer rules.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * A Zamani function describes portable computation and semantic intent.
 *
 * The grammar imposes no finite language-level limit on:
 *
 *   - number of functions;
 *   - number of parameters;
 *   - number of generic parameters;
 *   - number of generic constraints;
 *   - number of attributes;
 *   - number of contracts;
 *   - number of effects;
 *   - number of calls;
 *   - source-program size;
 *   - computational domains;
 *   - quantum objects;
 *   - distributed objects;
 *   - resources;
 *   - capabilities.
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
 *     MAX_MEMORY
 *     MAX_NODES
 *     MAX_DEVICES
 *
 * Nor may it encode:
 *
 *     cpu0
 *     gpu0
 *     qpu0
 *     physical_qubit0
 *     device0
 *
 * as universal language concepts.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * A function declaration may describe semantic requirements through:
 *
 *     - parameter types;
 *     - result types;
 *     - effects;
 *     - contracts;
 *     - generic constraints;
 *     - attributes;
 *     - declarations surrounding the function.
 *
 * Hardware/resource realization remains downstream.
 *
 * For example, a function may semantically require:
 *
 *     quantum measurement
 *     tensor computation
 *     distributed communication
 *     persistent storage
 *     a capability
 *
 * without saying:
 *
 *     use GPU 0
 *     use QPU 1
 *     use physical qubit 17
 *     use exactly 8 cores
 *
 * The latter are implementation decisions and do not belong in this grammar.
 *
 * ============================================================================
 * DOMAIN NEUTRALITY
 * ============================================================================
 *
 * The same function grammar can describe:
 *
 *     fn classical(...)
 *     fn quantum(...)
 *     fn hybrid(...)
 *     fn hardware(...)
 *     fn hdl(...)
 *     fn distributed(...)
 *     fn ai(...)
 *     fn data(...)
 *     fn networking(...)
 *     fn security(...)
 *     fn future_domain(...)
 *
 * Domain meaning is obtained from semantic types, effects, capabilities,
 * declarations, and domain-specific semantic analysis.
 *
 * The function grammar MUST NOT introduce:
 *
 *     gpuFunction
 *     qpuFunction
 *     cpuFunction
 *     fpgaFunction
 *     cudaFunction
 *     quantumFunction
 *
 * as separate parser languages.
 *
 * ============================================================================
 * QUANTUM INTEGRATION CONTRACT
 * ============================================================================
 *
 * A function may contain quantum syntax through ordinary parameters,
 * expressions, statements, types, effects, and domain declarations.
 *
 * This file MUST NOT define quantum operations or a quantum IR.
 *
 * Quantum lowering remains:
 *
 *     source
 *       ->
 *     domain-neutral AST
 *       ->
 *     semantic analysis
 *       ->
 *     quantum::ir
 *       ->
 *     optimization
 *       ->
 *     routing
 *       ->
 *     scheduling
 *       ->
 *     QEC / resilience
 *       ->
 *     ZQN
 *       ->
 *     HAL
 *       ->
 *     target realization
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION CONTRACT
 * ============================================================================
 *
 * A function may participate in software/hardware co-design.
 *
 * This file does not encode:
 *
 *     - register widths;
 *     - number of ports;
 *     - number of processing units;
 *     - FPGA resources;
 *     - ASIC resources;
 *     - physical topology;
 *     - clock frequency;
 *     - memory capacity;
 *     - device identifiers.
 *
 * Those belong to hardware/resource/deployment semantics.
 *
 * ============================================================================
 * ASYNC INTEGRATION CONTRACT
 * ============================================================================
 *
 * Asynchronous syntax is owned by:
 *
 *     grammar/functions/async.g4
 *
 * That grammar provides `asyncModifier`.
 *
 * This file attaches the modifier to the canonical function signature.
 *
 * The grammar does NOT choose:
 *
 *     - executor;
 *     - scheduler;
 *     - worker count;
 *     - thread count;
 *     - task placement;
 *     - CPU;
 *     - GPU;
 *     - accelerator;
 *     - runtime implementation.
 *
 * ============================================================================
 * FOREIGN FUNCTION INTEGRATION
 * ============================================================================
 *
 * Foreign functions are owned by:
 *
 *     grammar/functions/foreign-functions.g4
 *     grammar/interoperability/
 *
 * `extern` is therefore a declaration modifier available to the source
 * function boundary, but the ABI/foreign-language details remain owned by
 * the foreign/interoperability grammar.
 *
 * This file does NOT define:
 *
 *     foreignLanguageClause
 *     foreignAbiClause
 *     foreignSymbolClause
 *     foreignRepresentationClause
 *
 * ============================================================================
 * COMPILE-TIME FUNCTION INTEGRATION
 * ============================================================================
 *
 * Compile-time functions remain Zamani functions.
 *
 * Their specialized syntax and compile-time semantics remain owned by:
 *
 *     grammar/functions/compile-time-functions.g4
 *     grammar/expressions/compile-time.g4
 *     compiler semantic analysis.
 *
 * This file MUST NOT create a second compile-time function language.
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Contracts are attached after the function's generic constraints/effects
 * and before the implementation boundary.
 *
 * Contract syntax remains owned by:
 *
 *     grammar/functions/contracts.g4
 *
 * Contract expressions use the canonical expression grammar.
 *
 * The function grammar does not interpret contract expressions.
 *
 * ============================================================================
 * GENERIC INTEGRATION
 * ============================================================================
 *
 * Generic parameters are supplied by:
 *
 *     grammar/functions/generics.g4
 *
 * Generic constraints are supplied by:
 *
 *     grammar/functions/constraints.g4
 *
 * Generic semantics are downstream:
 *
 *     generic syntax
 *         ->
 *     AST
 *         ->
 *     semantic generic model
 *         ->
 *     type/trait/capability solving
 *
 * No generic parameter may encode a fixed machine size.
 *
 * ============================================================================
 * PARAMETER INTEGRATION
 * ============================================================================
 *
 * Parameters are supplied by:
 *
 *     grammar/functions/parameters.g4
 *
 * Parameter syntax may represent:
 *
 *     - ordinary values;
 *     - references;
 *     - resources;
 *     - capabilities;
 *     - quantum values;
 *     - tensors;
 *     - distributed values;
 *     - hardware abstractions;
 *     - future domain values.
 *
 * Their semantic legality is not decided here.
 *
 * ============================================================================
 * FUNCTION BODY CONTRACT
 * ============================================================================
 *
 * Function bodies use the canonical `block` rule.
 *
 * The block grammar owns:
 *
 *     - braces;
 *     - block elements;
 *     - statements;
 *     - expressions.
 *
 * This file only decides that a function implementation may terminate in:
 *
 *     block
 *
 * or:
 *
 *     SEMICOLON
 *
 * A semicolon-terminated function is validated semantically according to
 * context:
 *
 *     extern
 *     abstract
 *     interface
 *     trait
 *     declaration-only
 *     foreign
 *     generated
 *     dialect-specific
 *
 * The parser does not encode those semantic policies as hardware-dependent
 * behavior.
 *
 * ============================================================================
 * MODIFIER POLICY
 * ============================================================================
 *
 * Function modifiers are intentionally finite at the core-language level.
 *
 * Extension modifiers should NOT be added here one by one.
 *
 * Future/domain-specific modifiers belong to the canonical modifier/attribute
 * extension mechanisms.
 *
 * This prevents the function grammar from becoming a catalogue of every
 * accelerator, device, vendor, backend, or future machine.
 *
 * ============================================================================
 * ATTRIBUTE POLICY
 * ============================================================================
 *
 * Attributes are open-ended metadata.
 *
 * They are owned by grammar/core/attributes.g4.
 *
 * This file permits zero or more attributes before the function declaration.
 *
 * The attribute grammar itself determines the attribute's syntactic shape.
 *
 * Semantic validation determines whether an attribute is legal on a function.
 *
 * ============================================================================
 * SOURCE ORDER
 * ============================================================================
 *
 * The canonical function declaration order is:
 *
 *     attributes*
 *     modifiers*
 *     fn
 *     name
 *     generic-parameters?
 *     parameter-list
 *     return-clause?
 *     generic-constraints?
 *     effect-clause?
 *     contract*
 *     implementation
 *
 * Canonical examples:
 *
 *     fn main() {
 *     }
 *
 *     pub fn add(a: Int, b: Int) -> Int {
 *         return a + b;
 *     }
 *
 *     async fn compute<T>(value: T) -> Result<T> {
 *         ...
 *     }
 *
 *     fn measure<Q>(q: Q) -> Measurement
 *     with effects { quantum::measurement }
 *     contract {
 *         requires(...);
 *     }
 *     {
 *         ...
 *     }
 *
 *     fn declaration<T>(value: T) -> T;
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every function declaration MUST lower into the existing domain-neutral
 * frontend AST.
 *
 * The AST must retain, at minimum:
 *
 *     - source span;
 *     - attributes;
 *     - modifiers;
 *     - function name;
 *     - generic parameters;
 *     - parameters;
 *     - return type;
 *     - generic constraints;
 *     - effects;
 *     - contracts;
 *     - body/prototype state.
 *
 * This grammar MUST NOT introduce:
 *
 *     QuantumGate
 *     CpuFunction
 *     GpuFunction
 *     QpuFunction
 *     HardwareFunction
 *
 * or any other backend-specific function AST.
 *
 * The AST remains domain-neutral.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     - duplicate modifier detection;
 *     - modifier compatibility;
 *     - visibility rules;
 *     - declaration context;
 *     - generic constraint validity;
 *     - parameter validity;
 *     - return-type validity;
 *     - effect validity;
 *     - contract validity;
 *     - async legality;
 *     - generator legality;
 *     - foreign declaration legality;
 *     - compile-time restrictions;
 *     - capability requirements;
 *     - resource requirements;
 *     - ownership;
 *     - borrowing;
 *     - lifetime rules;
 *     - determinism;
 *     - domain legality;
 *     - recursion policy;
 *     - ABI decisions;
 *     - target-independent semantic correctness.
 *
 * The parser MUST remain context-light.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * Functions do not lower directly from grammar to a backend.
 *
 * Required conceptual path:
 *
 *     Function grammar
 *          ->
 *     frontend AST
 *          ->
 *     semantic function model
 *          ->
 *     canonical semantic IR
 *
 * Domain-specific lowering then occurs downstream.
 *
 * Classical:
 *
 *     semantic function
 *          ->
 *     classical IR
 *
 * Quantum:
 *
 *     semantic function
 *          ->
 *     quantum semantic operations
 *          ->
 *     quantum::ir
 *
 * HDL/hardware:
 *
 *     semantic function
 *          ->
 *     hardware/co-design semantic representation
 *
 * Never:
 *
 *     grammar -> LLVM
 *     grammar -> QIR
 *     grammar -> MLIR
 *     grammar -> CUDA
 *     grammar -> vendor assembly
 *     grammar -> physical qubit map
 *     grammar -> scheduler
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics must identify the source location associated with:
 *
 *     - function attributes;
 *     - modifiers;
 *     - function keyword;
 *     - function name;
 *     - generic parameters;
 *     - parameter list;
 *     - return clause;
 *     - constraints;
 *     - effects;
 *     - contracts;
 *     - implementation boundary.
 *
 * The grammar itself must not emit implementation-specific diagnostics.
 *
 * Semantic diagnostics should be responsible for errors such as:
 *
 *     duplicate visibility
 *     incompatible modifiers
 *     missing body
 *     illegal body
 *     invalid generic constraint
 *     invalid effect
 *     invalid contract
 *     invalid async usage
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no actions;
 *     - no semantic predicates;
 *     - no mutable global state;
 *     - no filesystem access;
 *     - no network access;
 *     - no hardware inspection;
 *     - no random behavior.
 *
 * Given the same token stream and grammar version, parsing must be deterministic.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar performs no:
 *
 *     - code execution;
 *     - command execution;
 *     - filesystem access;
 *     - network access;
 *     - dynamic library loading;
 *     - hardware access;
 *     - secret access.
 *
 * Any unsafe operation must be rejected or handled by downstream policy and
 * runtime boundaries.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * The grammar uses unbounded structural repetition where the language itself
 * does not impose a semantic cardinality limit:
 *
 *     attribute*
 *     functionModifier*
 *     contract*
 *     genericParameter*
 *     parameter*
 *     constraint*
 *
 * ANTLR/runtime/parser implementation limits are implementation-resource
 * concerns and must not become language semantics.
 *
 * Large functions must therefore remain semantically the same language
 * constructs as small functions.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * Forbidden universal function-level assumptions:
 *
 *     - fixed parameter count;
 *     - fixed generic count;
 *     - fixed function count;
 *     - fixed recursion depth;
 *     - fixed resource count;
 *     - fixed device count;
 *     - fixed processor count;
 *     - fixed qubit count;
 *     - fixed accelerator count;
 *     - fixed tensor dimensions;
 *     - fixed network-node count;
 *     - fixed topology;
 *     - fixed memory size;
 *     - fixed register width.
 *
 * No such limits are represented by this grammar.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing filenames are intentionally retained.
 *
 * This file remains:
 *
 *     grammar/functions/functions.g4
 *
 * Existing specialized files remain the owners of their respective rules.
 *
 * The cleanup performed here is architectural:
 *
 *     OLD:
 *         functions.g4 duplicated delegates.
 *
 *     NEW:
 *         functions.g4 composes delegates.
 *
 * Existing rule names retained where they represent useful stable integration
 * boundaries:
 *
 *     functionDeclaration
 *     functionName
 *     functionModifier
 *     functionImplementation
 *     functionSignature
 *     functionPrototype
 *     functionDefinition
 *     functionSignatureCore
 *     functionBodyDeclaration
 *     functionPrototypeDeclaration
 *
 * Removed from this file because they belong elsewhere:
 *
 *     functionReturnClause
 *     functionEffectClause
 *     functionEffectList
 *     functionEffectReference
 *     functionContractClause
 *     functionContractItem
 *     functionRequiresContract
 *     functionEnsuresContract
 *     functionInvariantContract
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] It is the sole composition owner of named function declarations.
 *
 * [x] It does not duplicate generic syntax.
 *
 * [x] It does not duplicate parameter syntax.
 *
 * [x] It does not duplicate return-type syntax.
 *
 * [x] It does not duplicate contract syntax.
 *
 * [x] It does not duplicate effect syntax.
 *
 * [x] It does not duplicate type syntax.
 *
 * [x] It does not duplicate expression syntax.
 *
 * [x] It does not duplicate block syntax.
 *
 * [x] It consumes ZamaniLexer.
 *
 * [x] It contains no lexer rules.
 *
 * [x] It contains no Rust code.
 *
 * [x] It contains no unsafe code.
 *
 * [x] It contains no hardware limits.
 *
 * [x] It contains no quantum-gate enumeration.
 *
 * [x] It contains no physical-device identifiers.
 *
 * [x] It contains no backend-specific function kinds.
 *
 * [x] It supports classical functions.
 *
 * [x] It supports quantum-capable functions through shared language constructs.
 *
 * [x] It supports hybrid functions through shared language constructs.
 *
 * [x] It supports hardware/co-design functions through shared language
 *     constructs.
 *
 * [x] It supports distributed functions through shared language constructs.
 *
 * [x] It supports AI/data/network/security domains without grammar forks.
 *
 * [x] It preserves the canonical quantum::ir boundary.
 *
 * [x] It preserves POCO-REAF.
 *
 * [x] It leaves resource discovery to resource/capability semantics.
 *
 * [x] It leaves scheduling to scheduling.
 *
 * [x] It leaves routing to routing.
 *
 * [x] It leaves QEC to QEC.
 *
 * [x] It leaves ZQN to ZQN.
 *
 * [x] It leaves HAL to HAL.
 *
 * [x] It leaves backend selection to compilation/lowering.
 *
 * [x] It remains scalable subject to actual implementation resources.
 *
 * ============================================================================
 * FINAL RULE
 * ============================================================================
 *
 * A Zamani function describes WHAT computation exists and WHAT semantic
 * properties apply.
 *
 * It does not prescribe WHICH machine performs it.
 *
 * Therefore:
 *
 *     function syntax
 *          !=
 *     hardware topology
 *
 *     function declaration
 *          !=
 *     backend selection
 *
 *     function parameter
 *          !=
 *     physical resource assignment
 *
 *     function grammar
 *          !=
 *     quantum IR
 *
 * The complete architecture remains:
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
 * subject to the actual program semantics, implementation capabilities,
 * compatibility requirements, and resources available at realization time.
 *
 * ============================================================================
 */

parser grammar Functions;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * IMPORTED AUTHORITIES
 * ============================================================================
 *
 * Each imported parser grammar is an existing independent contract.
 *
 * Do not copy their rules into this file.
 */
import
    Attributes,
    Names,
    FunctionGenerics,
    Parameters,
    ReturnTypes,
    FunctionConstraints,
    FunctionContracts,
    Effects,
    AsyncFunctions,
    Types,
    Expressions,
    Statements
    ;

/*
 * ============================================================================
 * 1. CANONICAL FUNCTION DECLARATION
 * ============================================================================
 *
 * This is the primary entry point consumed by declaration/item composition.
 *
 * The declaration has exactly one canonical signature followed by either:
 *
 *     block
 *
 * or:
 *
 *     semicolon
 *
 * Semantic analysis determines whether a declaration-only function is legal
 * in the surrounding context.
 */
functionDeclaration
    : functionSignature
      functionImplementation
    ;


/*
 * ============================================================================
 * 2. CANONICAL FUNCTION SIGNATURE
 * ============================================================================
 *
 * Signature composition order is fixed here so all downstream consumers have
 * one stable source-level representation.
 *
 *     attributes*
 *     modifiers*
 *     fn
 *     name
 *     generics?
 *     parameters
 *     return?
 *     constraints?
 *     effects?
 *     contracts*
 *
 * No semantic interpretation occurs here.
 */
functionSignature
    : functionSignatureCore
    ;


/*
 * ============================================================================
 * 3. REUSABLE FUNCTION SIGNATURE CORE
 * ============================================================================
 *
 * This rule intentionally excludes:
 *
 *     - function body;
 *     - prototype semicolon.
 *
 * That makes it reusable by:
 *
 *     - interface declarations;
 *     - trait declarations;
 *     - abstract declarations;
 *     - generated declarations;
 *     - dialect declarations;
 *     - foreign adapters;
 *     - other callable declaration boundaries.
 *
 * Those consumers remain responsible for their own declaration context.
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
 * Name syntax is owned by the canonical Names grammar.
 *
 * This rule creates the stable semantic attachment point for function names
 * without redefining identifier syntax.
 */
functionName
    : identifier
    ;


/*
 * ============================================================================
 * 5. FUNCTION MODIFIERS
 * ============================================================================
 *
 * Function modifiers express source-level declaration properties.
 *
 * They do not select a physical machine.
 *
 * Repeated modifiers are syntactically accepted and rejected semantically
 * when incompatible or duplicated.
 *
 * The grammar deliberately does not enumerate vendor/backend/domain-specific
 * function kinds.
 */
functionModifier
    : PUBLIC
    | PUB
    | PRIVATE
    | PROTECTED
    | INTERNAL
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
 * 6. FUNCTION IMPLEMENTATION
 * ============================================================================
 *
 * A function declaration terminates with exactly one implementation boundary.
 *
 *     fn f() { ... }
 *
 * or:
 *
 *     fn f();
 *
 * The semantic layer determines whether the second form is legal.
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
 * Explicit definition-only entry point.
 *
 * This does not create a second syntax. It reuses the canonical signature.
 */
functionDefinition
    : functionSignature
      block
    ;


/*
 * ============================================================================
 * 8. FUNCTION PROTOTYPE
 * ============================================================================
 *
 * Explicit declaration-only entry point.
 *
 * The semicolon is owned here rather than by functionSignature so that
 * signature consumers can embed the signature without inheriting a terminator.
 */
functionPrototype
    : functionSignature
      SEMICOLON
    ;


/*
 * ============================================================================
 * 9. BODY DECLARATION ADAPTER
 * ============================================================================
 *
 * Stable named adapter for consumers that specifically require a function
 * definition.
 */
functionBodyDeclaration
    : functionSignatureCore
      block
    ;


/*
 * ============================================================================
 * 10. PROTOTYPE DECLARATION ADAPTER
 * ============================================================================
 *
 * Stable named adapter for consumers that specifically require a declaration
 * without a body.
 */
functionPrototypeDeclaration
    : functionSignatureCore
      SEMICOLON
    ;


/*
 * ============================================================================
 * 11. ASYNC INTEGRATION
 * ============================================================================
 *
 * `ASYNC` is accepted through functionModifier.
 *
 * The dedicated async grammar remains responsible for reusable asynchronous
 * syntax such as:
 *
 *     asyncModifier
 *     awaitExpression
 *     asyncExpression
 *
 * This file intentionally does not duplicate those constructs.
 *
 * Therefore:
 *
 *     async fn compute(...) -> T { ... }
 *
 * is represented by the ordinary function declaration:
 *
 *     functionModifier*
 *         -> ASYNC
 *
 * followed by the canonical signature.
 *
 * Executor/scheduler/runtime placement is downstream.
 */


/*
 * ============================================================================
 * 12. GENERATOR INTEGRATION
 * ============================================================================
 *
 * Generator syntax remains statement-level and is owned by:
 *
 *     grammar/functions/generators.g4
 *
 * A function containing `yield` therefore remains an ordinary function
 * declaration syntactically.
 *
 * Semantic analysis determines generator legality and return/yield typing.
 *
 * This prevents generator declarations from becoming another function
 * language.
 */


/*
 * ============================================================================
 * 13. CLOSURE / LAMBDA INTEGRATION
 * ============================================================================
 *
 * Closures and lambdas are not named function declarations.
 *
 * Their syntax remains owned by:
 *
 *     grammar/functions/closures.g4
 *     grammar/expressions/lambdas.g4
 *
 * They may nevertheless use function-compatible semantic types.
 *
 * No closure grammar is duplicated here.
 */


/*
 * ============================================================================
 * 14. FOREIGN FUNCTION INTEGRATION
 * ============================================================================
 *
 * `EXTERN` is available as a source-level linkage modifier.
 *
 * Detailed foreign syntax remains owned by:
 *
 *     grammar/functions/foreign-functions.g4
 *     grammar/interoperability/
 *
 * In particular, this file does not define:
 *
 *     foreignLanguageClause
 *     foreignAbiClause
 *     foreignLinkageClause
 *     foreignSymbolClause
 *     foreignRepresentationClause
 *
 * The semantic/declaration composition layer must ensure that an extern
 * declaration is interpreted exactly once rather than being simultaneously
 * interpreted by competing foreign-function productions.
 */


/*
 * ============================================================================
 * 15. COMPILE-TIME FUNCTION INTEGRATION
 * ============================================================================
 *
 * Compile-time functions remain functions.
 *
 * Their specialized compile-time declaration boundary remains owned by:
 *
 *     grammar/functions/compile-time-functions.g4
 *
 * That grammar must reuse this file's function signature contract rather than
 * silently inventing an unrelated function syntax.
 *
 * This file deliberately does not define:
 *
 *     compileTimeFunctionDeclaration
 *
 * because doing so would create duplicate ownership.
 */


/*
 * ============================================================================
 * 16. EFFECT INTEGRATION
 * ============================================================================
 *
 * Function effects use the canonical:
 *
 *     effectClause
 *
 * from:
 *
 *     grammar/effects/effects.g4
 *
 * Therefore:
 *
 *     fn f() with effects { io, quantum::measurement } { ... }
 *
 * is composed through the effect grammar.
 *
 * This file does not enumerate effect names.
 *
 * That is essential because future domains may introduce effects without
 * requiring a rewrite of the function grammar.
 */


/*
 * ============================================================================
 * 17. CONTRACT INTEGRATION
 * ============================================================================
 *
 * Function contracts use:
 *
 *     functionContractClause
 *
 * from:
 *
 *     grammar/functions/contracts.g4
 *
 * Contracts may contain ordinary canonical expressions.
 *
 * The function grammar controls only placement and cardinality.
 *
 * Contract meaning remains semantic.
 */


/*
 * ============================================================================
 * 18. GENERIC CONSTRAINT INTEGRATION
 * ============================================================================
 *
 * Generic parameter syntax:
 *
 *     functionGenericParameters
 *
 * is owned by:
 *
 *     grammar/functions/generics.g4
 *
 * Function-level `where` constraints:
 *
 *     functionConstraintClause
 *
 * are owned by:
 *
 *     grammar/functions/constraints.g4
 *
 * No constraint solver belongs here.
 */


/*
 * ============================================================================
 * 19. PARAMETER INTEGRATION
 * ============================================================================
 *
 * Parameter syntax is owned by:
 *
 *     grammar/functions/parameters.g4
 *
 * This grammar therefore does not duplicate:
 *
 *     parameterList
 *     parameter
 *
 * Parameter semantics may later represent:
 *
 *     classical values
 *     quantum values
 *     resources
 *     capabilities
 *     tensors
 *     distributed values
 *     hardware abstractions
 *     future domains
 *
 * without modifying this declaration grammar.
 */


/*
 * ============================================================================
 * 20. RETURN TYPE INTEGRATION
 * ============================================================================
 *
 * Return syntax is owned by:
 *
 *     grammar/functions/return-types.g4
 *
 * This file merely attaches:
 *
 *     functionReturnClause?
 *
 * It does not duplicate:
 *
 *     -> typeExpression
 *
 * This prevents multiple return-type authorities.
 */


/*
 * ============================================================================
 * 21. TYPE INTEGRATION
 * ============================================================================
 *
 * Type syntax is owned by:
 *
 *     grammar/types/types.g4
 *
 * Function declarations therefore remain independent of the number or nature
 * of concrete types supported by Zamani.
 *
 * This permits future:
 *
 *     classical types
 *     quantum types
 *     hybrid types
 *     tensor types
 *     hardware types
 *     resource types
 *     capability types
 *     distributed types
 *     temporal types
 *     future-domain types
 *
 * without adding function-specific productions.
 */


/*
 * ============================================================================
 * 22. EXPRESSION INTEGRATION
 * ============================================================================
 *
 * Function bodies and contracts use the canonical expression system.
 *
 * This grammar never defines a second expression language.
 */


/*
 * ============================================================================
 * 23. BLOCK INTEGRATION
 * ============================================================================
 *
 * `block` is owned by:
 *
 *     grammar/core/blocks.g4
 *
 * and is made reachable through the canonical statement composition.
 *
 * Function bodies therefore contain the same source-language block semantics
 * used elsewhere in Zamani.
 */


/*
 * ============================================================================
 * 24. ATTRIBUTE INTEGRATION
 * ============================================================================
 *
 * Function attributes use the canonical:
 *
 *     attribute
 *
 * from:
 *
 *     grammar/core/attributes.g4
 *
 * This gives functions an extensible metadata mechanism without adding
 * backend-specific keywords.
 *
 * Examples of possible semantic domains include:
 *
 *     compiler attributes
 *     optimization hints
 *     resource requirements
 *     capability requirements
 *     quantum metadata
 *     HDL metadata
 *     interoperability metadata
 *     security metadata
 *
 * The grammar does not enumerate those domains.
 */


/*
 * ============================================================================
 * 25. HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * The following are deliberately absent:
 *
 *     cpuFunction
 *     gpuFunction
 *     fpgaFunction
 *     qpuFunction
 *     acceleratorFunction
 *     deviceFunction
 *     coreFunction
 *
 * Hardware-specific intent must be represented through:
 *
 *     types
 *     effects
 *     capabilities
 *     resources
 *     attributes
 *     constraints
 *     surrounding hardware/co-design declarations
 *
 * and resolved downstream.
 */


/*
 * ============================================================================
 * 26. QUANTUM INDEPENDENCE
 * ============================================================================
 *
 * The function grammar deliberately contains no:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     CNOT
 *     SWAP
 *     fixed gate enumeration
 *     fixed qubit count
 *     physical qubit identifier
 *     topology
 *     calibration
 *
 * A quantum-capable function is still a Zamani function.
 *
 * Example:
 *
 *     fn algorithm(q: quantum::Register) -> quantum::Result {
 *         ...
 *     }
 *
 * The semantic quantum operations are lowered later to `quantum::ir`.
 */


/*
 * ============================================================================
 * 27. DISTRIBUTED / HPC INDEPENDENCE
 * ============================================================================
 *
 * There is no:
 *
 *     fn_on_8_nodes
 *     fn_on_128_threads
 *     fn_on_gpu_0
 *
 * A function may express distributed semantics through canonical types,
 * effects, capabilities, resources, and concurrency constructs.
 *
 * The actual number of nodes/processors/workers is resolved downstream.
 */


/*
 * ============================================================================
 * 28. AI / DATA / NETWORKING INDEPENDENCE
 * ============================================================================
 *
 * AI, data, networking and security are domains of the language, not alternate
 * function declaration syntaxes.
 *
 * This prevents:
 *
 *     neuralFunction
 *     tensorFunction
 *     networkFunction
 *     cryptographicFunction
 *
 * from becoming an ever-growing parser catalogue.
 */


/*
 * ============================================================================
 * 29. FRONTEND AST CONTRACT
 * ============================================================================
 *
 * The parser must provide enough structure for the existing domain-neutral
 * AST/frontend to represent:
 *
 *     Function
 *       attributes
 *       modifiers
 *       name
 *       generics
 *       parameters
 *       return type
 *       constraints
 *       effects
 *       contracts
 *       body/prototype
 *
 * Source spans must be preserved by the frontend parser/AST layer.
 *
 * The grammar does not construct AST nodes itself.
 */


/*
 * ============================================================================
 * 30. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     modifier compatibility
 *     duplicate modifiers
 *     visibility
 *     declaration context
 *     generic validity
 *     generic constraint solving
 *     parameter validity
 *     return-type validity
 *     effect validity
 *     contract validity
 *     async legality
 *     generator legality
 *     extern legality
 *     compile-time restrictions
 *     ownership
 *     borrowing
 *     lifetimes
 *     capability checking
 *     resource requirements
 *     determinism
 *     domain compatibility
 *     recursion policy
 *     overload resolution
 *     ABI decisions
 *
 * Parser acceptance must not be confused with semantic validity.
 */


/*
 * ============================================================================
 * 31. IR INTEGRATION
 * ============================================================================
 *
 * Function declarations are lowered through the frontend semantic model.
 *
 * They do NOT lower directly to backend representations.
 *
 * Required architecture:
 *
 *     Function syntax
 *         ->
 *     domain-neutral AST
 *         ->
 *     semantic model
 *         ->
 *     canonical IR
 *
 * Quantum:
 *
 *     semantic model
 *         ->
 *     quantum::ir
 *
 * Classical:
 *
 *     semantic model
 *         ->
 *     classical IR
 *
 * HDL/hardware:
 *
 *     semantic model
 *         ->
 *     canonical hardware/HDL representation
 *
 * This keeps the function grammar independent from LLVM, QIR, MLIR, vendor
 * assembly, CUDA, physical qubits, FPGA primitives, and runtime schedulers.
 */


/*
 * ============================================================================
 * 32. RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime behavior is not encoded here.
 *
 * Runtime components may consume the semantic/IR representation of:
 *
 *     functions
 *     calls
 *     effects
 *     resources
 *     capabilities
 *     concurrency
 *     quantum operations
 *     hardware intent
 *
 * but the grammar remains target-independent.
 */


/*
 * ============================================================================
 * 33. TOOLING INTEGRATION
 * ============================================================================
 *
 * The stable rule names provide tooling anchors for:
 *
 *     formatter
 *     syntax highlighter
 *     LSP
 *     diagnostics
 *     documentation generator
 *     grammar conformance tooling
 *     AST builder
 *     source indexer
 *     refactoring tools
 *
 * Tooling must use source spans and AST/semantic identities rather than
 * reparsing backend-specific representations.
 */


/*
 * ============================================================================
 * 34. NEGATIVE CASES
 * ============================================================================
 *
 * These must be rejected or diagnosed at the appropriate parser/semantic
 * boundary:
 *
 *     fn
 *     fn ()
 *     fn 123() {}
 *     fn f(
 *     fn f() ->
 *     fn f(a: ) {}
 *     fn f<T,>() {}
 *
 * Depending on semantic context:
 *
 *     fn f() {}
 *     extern fn f() {}
 *     abstract fn f() {}
 *
 * may be legal or illegal.
 *
 * The parser should accept structurally valid forms and let semantic analysis
 * enforce declaration-context rules.
 */


/*
 * ============================================================================
 * 35. BOUNDARY CASES
 * ============================================================================
 *
 * The conformance suite must include:
 *
 *     fn f() {}
 *
 *     fn f(a: T) {}
 *
 *     fn f(a: T, b: U, c: V) {}
 *
 *     fn f<T>(value: T) -> T {}
 *
 *     fn f<T, U>(a: T, b: U) -> Result<T, U> {}
 *
 *     async fn f() {}
 *
 *     pub async fn f<T>(x: T) -> T {}
 *
 *     fn f<T>(x: T) where T: Numeric {}
 *
 *     fn f() with effects { io } {}
 *
 *     fn f() contract {
 *         requires(true);
 *     } {}
 *
 *     fn f<T>(x: T) -> T
 *     where T: Numeric
 *     with effects { quantum::measurement }
 *     contract {
 *         requires(true);
 *     }
 *     {}
 *
 *     extern fn f() -> Value;
 *
 *     abstract fn f() -> Value;
 *
 * The exact legality of combinations is semantic.
 */


/*
 * ============================================================================
 * 36. SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Tests must verify that the grammar does not impose artificial cardinality
 * limits.
 *
 * Required generated/adversarial cases include:
 *
 *     - many parameters;
 *     - many generic parameters;
 *     - many generic bounds;
 *     - many constraints;
 *     - many attributes;
 *     - many effects;
 *     - many contracts;
 *     - deeply nested type expressions;
 *     - very large function bodies;
 *     - large source files containing many functions.
 *
 * No test may establish an arbitrary maximum as a language rule.
 *
 * Resource exhaustion is an implementation concern and must be reported as
 * an implementation/resource diagnostic rather than silently becoming a
 * language semantic limit.
 */


/*
 * ============================================================================
 * 37. DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * The same source/token sequence must produce the same parse structure.
 *
 * There are:
 *
 *     - no semantic predicates;
 *     - no actions;
 *     - no randomness;
 *     - no external state;
 *     - no target discovery.
 */


/*
 * ============================================================================
 * 38. HARD-CODING COMPLETION AUDIT
 * ============================================================================
 *
 * This grammar contains no:
 *
 *     MAX_*
 *     CPU counts
 *     GPU counts
 *     FPGA counts
 *     QPU counts
 *     qubit counts
 *     memory capacities
 *     register widths
 *     node counts
 *     topology sizes
 *     physical addresses
 *     physical device identifiers
 *     vendor-specific function kinds
 *
 * Therefore function declaration syntax remains scalable from tiny programs
 * to programs whose realized size is constrained only by the actual semantic
 * requirements and resources available at compilation/execution.
 */


/*
 * ============================================================================
 * 39. FINAL ARCHITECTURAL GUARANTEE
 * ============================================================================
 *
 * This file deliberately makes function declarations:
 *
 *     domain-neutral
 *     resource-neutral
 *     topology-neutral
 *     backend-neutral
 *     device-neutral
 *     quantum-IR-neutral
 *
 * while still allowing functions to participate in:
 *
 *     classical computing
 *     quantum computing
 *     hybrid computing
 *     HDL
 *     hardware/software co-design
 *     AI/ML
 *     data processing
 *     distributed computing
 *     HPC
 *     networking
 *     security
 *     embedded systems
 *     future computational domains
 *
 * The resulting architecture supports:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * because the function declaration describes portable program semantics while
 * realization is delegated to the later compilation/runtime layers.
 *
 * ============================================================================
 */
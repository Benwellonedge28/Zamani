/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/expressions/lambdas.g4
 *
 * Role:
 *     Canonical parser grammar for source-level lambda expressions.
 *
 * Status:
 *     Production grammar contract.
 *
 * Language:
 *     Zamani
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     Safe Rust only.
 *
 * ============================================================================
 * AUTHORITY
 * ============================================================================
 *
 * This file is the SINGLE grammar owner of:
 *
 *     lambdaExpression
 *     lambdaModifier
 *     lambdaParameterClause
 *     lambdaReturnTypeClause
 *     lambdaBody
 *
 * This file MUST NOT be duplicated under:
 *
 *     grammar/functions/lambdas.g4
 *
 * The existing repository location:
 *
 *     grammar/expressions/lambdas.g4
 *
 * is retained as the canonical location because lambda expressions are
 * expressions, not declarations.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * A lambda is an anonymous callable expression.
 *
 * Canonical forms:
 *
 *     || expression
 *
 *     |x| expression
 *
 *     |x, y| x + y
 *
 *     |x: Int| x + 1
 *
 *     |x| {
 *         x + 1
 *     }
 *
 * Explicit return type:
 *
 *     |x| -> Int x + 1
 *
 * Explicit capture:
 *
 *     capture { x } |x| x + 1
 *
 * Asynchronous lambda:
 *
 *     async |x| await x
 *
 * The grammar describes syntax only.
 *
 * It does not decide:
 *
 *     - type inference;
 *     - ownership;
 *     - borrowing;
 *     - capture legality;
 *     - effect legality;
 *     - resource requirements;
 *     - scheduling;
 *     - execution placement;
 *     - quantum mapping;
 *     - hardware selection;
 *     - runtime representation.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - lambdaExpression;
 *     - lambdaModifier;
 *     - lambdaParameterClause;
 *     - lambdaReturnTypeClause;
 *     - lambdaBody;
 *     - lambda expression composition.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer tokens;
 *     - identifiers;
 *     - keywords;
 *     - parameter internals;
 *     - function declarations;
 *     - function return clauses;
 *     - type expressions;
 *     - ordinary expressions;
 *     - blocks;
 *     - closure capture internals;
 *     - statements;
 *     - semantic analysis;
 *     - AST definitions;
 *     - IR definitions;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - routing;
 *     - scheduling;
 *     - runtime representation;
 *     - ABI;
 *     - backend selection.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     canonical lexer
 *       |
 *       v
 *     canonical parser
 *       |
 *       v
 *     lambdaExpression
 *       |
 *       v
 *     domain-neutral frontend AST
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +--> name resolution
 *       +--> type checking / inference
 *       +--> capture analysis
 *       +--> ownership / borrowing
 *       +--> lifetime analysis
 *       +--> effects
 *       +--> capabilities
 *       +--> resources
 *       |
 *       v
 *     canonical semantic model
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> HDL / hardware IR
 *       +--> distributed / AI / data representations
 *       |
 *       v
 *     optimization / lowering
 *       |
 *       v
 *     routing / scheduling / resilience / QEC / ZQN
 *       |
 *       v
 *     HAL
 *       |
 *       v
 *     target realization
 *
 * Lambda syntax MUST remain above target-specific realization.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Lambda syntax imposes no language-level limit on:
 *
 *     - lambda count;
 *     - parameter count;
 *     - capture count;
 *     - nesting depth;
 *     - expression size;
 *     - body size;
 *     - source size;
 *     - callable composition;
 *     - computational domain;
 *     - hardware size;
 *     - number of CPUs;
 *     - number of cores;
 *     - number of threads;
 *     - number of GPUs;
 *     - number of FPGAs;
 *     - number of QPUs;
 *     - number of qubits;
 *     - memory capacity;
 *     - distributed node count.
 *
 * Resource limits belong to implementation/resource policy and MUST NOT be
 * represented as grammar constants.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This grammar MUST NOT contain:
 *
 *     MAX_LAMBDA_PARAMETERS
 *     MAX_LAMBDA_DEPTH
 *     MAX_CAPTURES
 *     MAX_CLOSURES
 *     MAX_THREADS
 *     MAX_CORES
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_QUBITS
 *     MAX_NODES
 *     MAX_MEMORY
 *
 * A valid lambda is constrained only by the language grammar and implementation
 * resource availability.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexer is:
 *
 *     ZamaniLexer
 *
 * This grammar contains NO lexer rules.
 *
 * Relevant canonical tokens already represented by the repository include:
 *
 *     PIPE
 *     THIN_ARROW
 *     ASYNC
 *     MOVE
 *     IDENTIFIER
 *     LPAREN
 *     RPAREN
 *     LBRACE
 *     RBRACE
 *
 * The token spelling is owned by grammar/lexer/.
 *
 * IMPORTANT:
 *
 *     ->  MUST use THIN_ARROW.
 *
 * Do NOT introduce a second ARROW token merely for lambdas.
 *
 * ============================================================================
 * PARSER DEPENDENCIES
 * ============================================================================
 *
 * Canonical dependencies:
 *
 *     functions/parameters.g4
 *         -> parameterList
 *
 *     types/*.g4
 *         -> typeExpression
 *
 *     expressions/*.g4
 *         -> expression
 *
 *     core/blocks.g4
 *         -> blockExpression
 *
 *     functions/closures.g4
 *         -> closureCaptureClause
 *
 * This file MUST consume those contracts rather than duplicate them.
 *
 * ============================================================================
 * FUNCTION VS LAMBDA BOUNDARY
 * ============================================================================
 *
 * functions/functions.g4 owns:
 *
 *     fn name(parameters) -> Type { ... }
 *
 * functions/returns.g4 owns:
 *
 *     functionReturnClause
 *
 * This file owns:
 *
 *     |parameters| -> Type body
 *
 * These are intentionally different constructs.
 *
 * Do NOT import or reuse functionReturnClause here.
 *
 * The common token:
 *
 *     THIN_ARROW
 *
 * is shared, but the parser rule ownership is not.
 *
 * ============================================================================
 * CLOSURE BOUNDARY
 * ============================================================================
 *
 * functions/closures.g4 owns:
 *
 *     closureCaptureClause
 *
 * This file consumes that rule.
 *
 * This file MUST NOT define:
 *
 *     closureCaptureClause
 *     closureCaptureList
 *     closureCapture
 *     closureCaptureMode
 *     closureCaptureTarget
 *     closureCaptureAlias
 *
 * again.
 *
 * ============================================================================
 * PARAMETER BOUNDARY
 * ============================================================================
 *
 * The lambda parameter clause owns only:
 *
 *     PIPE parameter-list PIPE
 *
 * The contents are delegated to the canonical parameter grammar.
 *
 * This prevents a second lambda-specific parameter hierarchy.
 *
 * ============================================================================
 * RETURN TYPE BOUNDARY
 * ============================================================================
 *
 * Lambda return syntax is:
 *
 *     THIN_ARROW typeExpression
 *
 * It is optional.
 *
 * No semantic default is assigned here.
 *
 * If omitted, semantic analysis determines the callable result according to
 * the language's inference/unit rules.
 *
 * ============================================================================
 * BODY BOUNDARY
 * ============================================================================
 *
 * A lambda body is either:
 *
 *     expression
 *
 * or:
 *
 *     blockExpression
 *
 * The lambda grammar does not define a third body representation.
 *
 * ============================================================================
 * MODIFIER CONTRACT
 * ============================================================================
 *
 * Only modifiers explicitly approved for lambda expressions belong here.
 *
 * Current canonical modifiers:
 *
 *     async
 *
 *     move
 *
 * `async` describes asynchronous source intent.
 *
 * It does NOT select:
 *
 *     a thread;
 *     a core;
 *     a scheduler;
 *     an executor;
 *     a CPU;
 *     a GPU;
 *     a QPU.
 *
 * `move` describes source-level callable/capture intent.
 *
 * Ownership legality is semantic.
 *
 * The grammar MUST NOT grow a separate modifier for every execution target.
 *
 * Do NOT add:
 *
 *     gpu
 *     qpu
 *     cpu
 *     fpga
 *     accelerator
 *     device0
 *     thread8
 *
 * as lambda modifiers.
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * Lambdas are domain-neutral.
 *
 * They may:
 *
 *     - calculate quantum parameters;
 *     - transform measurement results;
 *     - generate quantum operations;
 *     - control hybrid computation;
 *     - construct quantum data;
 *     - participate in higher-order quantum algorithms.
 *
 * This grammar MUST NOT define:
 *
 *     quantumLambda
 *     qubitLambda
 *     quantumCapture
 *     physicalQubitLambda
 *     qpuLambda
 *     quantumBackendLambda
 *
 * Quantum semantics are resolved downstream.
 *
 * The canonical semantic quantum boundary remains:
 *
 *     quantum::ir
 *
 * No lambda grammar creates a quantum IR.
 *
 * ============================================================================
 * CLASSICAL / HDL / AI / DATA / DISTRIBUTED CONTRACT
 * ============================================================================
 *
 * The same lambda syntax may operate over:
 *
 *     classical values;
 *     tensors;
 *     datasets;
 *     AI models;
 *     distributed values;
 *     hardware intent;
 *     HDL elaboration;
 *     resource descriptions;
 *     networking values;
 *     security values;
 *     hybrid quantum/classical values;
 *     future domains.
 *
 * Domain-specific meaning belongs to semantic analysis.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The grammar produces one domain-neutral LambdaExpression structure.
 *
 * Conceptually:
 *
 *     LambdaExpression {
 *         parameters,
 *         return_type,
 *         body,
 *         modifiers,
 *         capture,
 *         source_span
 *     }
 *
 * The exact Rust AST representation remains owned by:
 *
 *     src/frontend/ast/
 *
 * or the repository's current canonical AST layer.
 *
 * This grammar MUST NOT introduce:
 *
 *     QuantumLambda
 *     HardwareLambda
 *     GpuLambda
 *     QpuLambda
 *     AnonymousFunction2
 *
 * or another parallel AST.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must determine:
 *
 *     - parameter binding;
 *     - parameter type validity;
 *     - inferred parameter types where permitted;
 *     - return type validity;
 *     - return type inference;
 *     - capture set;
 *     - capture modes;
 *     - ownership;
 *     - borrowing;
 *     - lifetime validity;
 *     - effect legality;
 *     - capability requirements;
 *     - resource requirements;
 *     - callable type;
 *     - purity;
 *     - async legality;
 *     - concurrency legality;
 *     - distributed legality;
 *     - quantum legality.
 *
 * None of these decisions are parser responsibilities.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar emits no IR.
 *
 * Lambda expressions first enter the domain-neutral semantic representation.
 *
 * Depending on their resolved semantics they may eventually lower into:
 *
 *     classical IR
 *     quantum::ir
 *     HDL / hardware IR
 *     distributed IR
 *     data / AI representations
 *
 * The lambda grammar MUST NOT determine which IR is selected.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * Downstream compiler transformations may include:
 *
 *     closure conversion
 *     lambda lifting
 *     environment conversion
 *     specialization
 *     monomorphization
 *     inlining
 *     partial evaluation
 *     parallelization
 *     distribution
 *     accelerator lowering
 *     quantum lowering
 *     hardware generation
 *
 * These transformations MUST NOT require changes to this grammar.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime representation may use:
 *
 *     stack environments
 *     heap environments
 *     static environments
 *     function objects
 *     specialized callables
 *     distributed environments
 *     accelerator representations
 *
 * The grammar does not prescribe any runtime representation.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source token stream
 *     grammar/version configuration
 *
 * Parsing MUST NOT depend on:
 *
 *     system time
 *     randomness
 *     filesystem state
 *     network state
 *     CPU availability
 *     GPU availability
 *     QPU availability
 *     scheduler state
 *     calibration state
 *     runtime state.
 *
 * ============================================================================
 * SOURCE-COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * The existing canonical form remains valid:
 *
 *     |x| x + 1
 *
 *     |x, y| x + y
 *
 *     || 42
 *
 *     |x| {
 *         return x;
 *     }
 *
 * Optional extensions are additive:
 *
 *     async |x| await x
 *
 *     |x| -> Int x + 1
 *
 *     capture { x } |x| x + 1
 *
 * No existing lambda syntax is removed by this file.
 *
 * ============================================================================
 * SYNTAX RULES
 * ============================================================================
 */

parser grammar Lambdas;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * The canonical lambda expression.
 *
 * Capture syntax is optional and owned by functions/closures.g4.
 *
 * Modifiers are optional and owned by lambdaModifier.
 *
 * Parameter clause is mandatory.
 *
 * Return type is optional.
 *
 * Body is mandatory.
 */
lambdaExpression
    : closureCaptureClause?
      lambdaModifier*
      lambdaParameterClause
      lambdaReturnTypeClause?
      lambdaBody
    ;


/*
 * Source-level lambda modifiers.
 *
 * These are deliberately small and extensible through future specification
 * work rather than by tying lambdas to hardware targets.
 */
lambdaModifier
    : ASYNC
    | MOVE
    ;


/*
 * Lambda parameter delimiters.
 *
 * Examples:
 *
 *     ||
 *     |x|
 *     |x, y|
 */
lambdaParameterClause
    : PIPE
      parameterList?
      PIPE
    ;


/*
 * Explicit lambda return type.
 *
 * IMPORTANT:
 *
 *     THIN_ARROW
 *
 * is the canonical `->` token.
 *
 * Do not use ARROW here.
 */
lambdaReturnTypeClause
    : THIN_ARROW
      typeExpression
    ;


/*
 * Lambda body.
 *
 * Expression and block bodies reuse their canonical owners.
 */
lambdaBody
    : expression
    | blockExpression
    ;
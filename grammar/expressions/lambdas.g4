/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/expressions/lambdas.g4
 *
 * Role:
 *     Canonical source-level lambda-expression parser delegate.
 *
 * Baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Edition 2021
 *     Safe Rust only
 *     No unsafe Rust.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns ONLY the source syntax of lambda expressions.
 *
 * Canonical source form:
 *
 *     |[parameterList]| expression
 *
 *     |[parameterList]| blockExpression
 *
 * Optional source-level extensions that are already represented by the
 * surrounding language contracts may occur before the parameter clause or
 * between the parameter clause and body.
 *
 * Examples:
 *
 *     || x
 *
 *     |x| x + 1
 *
 *     |x, y| x + y
 *
 *     |x: Int| x + 1
 *
 *     |x| {
 *         let y = x + 1;
 *         return y;
 *     }
 *
 *     |x| -> Int x + 1
 *
 * The exact semantic meaning of the lambda is determined downstream.
 *
 * ============================================================================
 * OWNS
 * ============================================================================
 *
 * This file owns:
 *
 *   - lambdaExpression;
 *   - lambda parameter-clause delimiters;
 *   - lambda body selection;
 *   - optional lambda return-type syntax;
 *   - lambda-specific syntactic composition;
 *   - the public parser entry point lambdaExpression.
 *
 * ============================================================================
 * DOES NOT OWN
 * ============================================================================
 *
 * This file does NOT own:
 *
 *   - lexer rules;
 *   - identifiers;
 *   - keywords;
 *   - punctuation token definitions;
 *   - ordinary parameter syntax;
 *   - type syntax;
 *   - general expression syntax;
 *   - block syntax;
 *   - statement syntax;
 *   - closure capture semantics;
 *   - name resolution;
 *   - type checking;
 *   - inference;
 *   - ownership;
 *   - borrowing;
 *   - lifetime analysis;
 *   - effect analysis;
 *   - capability analysis;
 *   - resource analysis;
 *   - scheduling;
 *   - routing;
 *   - optimization;
 *   - quantum IR;
 *   - classical IR;
 *   - HDL IR;
 *   - runtime representation;
 *   - ABI;
 *   - backend selection;
 *   - hardware selection.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     ZamaniLexer
 *          |
 *          v
 *     lambdaExpression
 *          |
 *          v
 *     domain-neutral frontend AST
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> name resolution
 *          +--> type checking/inference
 *          +--> capture analysis
 *          +--> ownership/borrow analysis
 *          +--> effect analysis
 *          +--> capability analysis
 *          +--> resource analysis
 *          |
 *          v
 *     canonical semantic model / ZUIR
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL/hardware IR
 *          +--> distributed/data/AI representations
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing / scheduling / resilience / QEC / ZQN
 *          |
 *          v
 *     HAL / target realization
 *
 * Lambda syntax MUST remain above all target-specific decisions.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * A lambda describes computation, not its eventual machine.
 *
 * The grammar therefore imposes no language-level limit on:
 *
 *   - number of lambdas;
 *   - lambda nesting;
 *   - parameter count;
 *   - body size;
 *   - expression size;
 *   - source-file size;
 *   - closure count;
 *   - capture count;
 *   - target devices;
 *   - CPUs;
 *   - cores;
 *   - threads;
 *   - GPUs;
 *   - FPGAs;
 *   - QPUs;
 *   - qubits;
 *   - memory;
 *   - nodes.
 *
 * Actual limits are implementation/resource-policy concerns.
 *
 * There MUST be no grammar-level constants such as:
 *
 *     MAX_LAMBDA_PARAMETERS
 *     MAX_LAMBDA_DEPTH
 *     MAX_CLOSURES
 *     MAX_CAPTURES
 *     MAX_THREADS
 *     MAX_QUBITS
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * The canonical lexer vocabulary is:
 *
 *     ZamaniLexer
 *
 * No lexer rules are defined here.
 *
 * Punctuation and keyword spellings remain owned by the lexer.
 *
 * ============================================================================
 * COMPOSITION CONTRACT
 * ============================================================================
 *
 * This file is a parser delegate.
 *
 * The canonical expression composition grammar is responsible for importing
 * this grammar and exposing lambdaExpression from primaryExpression.
 *
 * Conceptually:
 *
 *     primaryExpression
 *         : ...
 *         | lambdaExpression
 *         | ...
 *         ;
 *
 * This file MUST NOT create another public `expression` rule.
 *
 * This file also MUST NOT recreate:
 *
 *     parameterList
 *     typeExpression
 *     expression
 *     blockExpression
 *     identifier
 *     qualifiedName
 *
 * Those belong to their canonical grammar owners.
 *
 * ============================================================================
 * ANTLR DEPENDENCY RULE
 * ============================================================================
 *
 * The final expression composition must make the canonical rules referenced
 * below available to this delegate.
 *
 * The intended ownership is:
 *
 *     functions/parameters.g4  -> parameterList
 *     types/*.g4               -> typeExpression
 *     expressions/*.g4         -> expression
 *     core/blocks.g4            -> blockExpression/block
 *     functions/closures.g4    -> closureCaptureClause
 *
 * Do not copy those rules into this file.
 *
 * This keeps the grammar modular and prevents competing definitions.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every successful lambdaExpression must map to the repository's canonical
 * frontend LambdaExpression representation.
 *
 * Existing AST contract:
 *
 *     LambdaExpression
 *       - parameters: Vec<NodeId>
 *       - body: NodeId
 *       - return_type: Option<NodeId>
 *
 * The parser must therefore preserve:
 *
 *     parameter order
 *     body
 *     explicit return type, if present
 *     complete source span
 *
 * The lambda grammar must NOT introduce another:
 *
 *     Lambda
 *     Closure
 *     AnonymousFunction
 *
 * AST representation.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes only source structure.
 *
 * Semantic analysis establishes:
 *
 *     - binding;
 *     - scope;
 *     - parameter types;
 *     - return type;
 *     - capture set;
 *     - capture modes;
 *     - ownership;
 *     - borrowing;
 *     - lifetimes;
 *     - effects;
 *     - capabilities;
 *     - resource requirements;
 *     - callable type;
 *     - purity;
 *     - concurrency legality;
 *     - distributed-execution legality;
 *     - quantum legality.
 *
 * The parser must not perform these decisions.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Lambdas are domain-neutral.
 *
 * A lambda may:
 *
 *     - calculate a quantum parameter;
 *     - transform measurement results;
 *     - construct a quantum operation;
 *     - control a hybrid computation;
 *     - participate in a higher-order quantum algorithm;
 *     - produce values consumed by quantum operations.
 *
 * None of those uses require a quantum-specific lambda grammar.
 *
 * This file MUST NOT introduce:
 *
 *     quantumLambda
 *     qubitLambda
 *     quantumCapture
 *     physicalQubitParameter
 *     quantumBackendParameter
 *
 * The resulting semantic computation may eventually lower through:
 *
 *     quantum::ir
 *
 * when its resolved semantics are quantum.
 *
 * This file never creates a second quantum IR.
 *
 * ============================================================================
 * CLASSICAL / HDL / AI / DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * The same lambda syntax may be used for:
 *
 *     classical computation
 *     data transformations
 *     tensor operations
 *     AI/ML transformations
 *     distributed computation
 *     hardware-generation functions
 *     HDL elaboration
 *     compile-time computation
 *     hybrid quantum-classical computation
 *
 * Domain-specific meaning belongs downstream.
 *
 * ============================================================================
 * CLOSURE INTEGRATION
 * ============================================================================
 *
 * Explicit closure capture syntax belongs to:
 *
 *     grammar/functions/closures.g4
 *
 * This file consumes its canonical closure-capture rule if the language
 * composition enables explicit capture syntax.
 *
 * Capture analysis itself remains semantic.
 *
 * The lambda AST must not acquire a second capture representation merely
 * because capture syntax exists.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing a lambda MUST depend only on:
 *
 *     source tokens
 *     grammar version
 *
 * It MUST NOT depend on:
 *
 *     system time
 *     randomness
 *     environment variables
 *     hardware discovery
 *     device state
 *     network state
 *     runtime scheduler state
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Parser diagnostics should identify:
 *
 *     - missing opening pipe;
 *     - missing closing pipe;
 *     - malformed parameter list;
 *     - malformed return type;
 *     - missing lambda body;
 *     - invalid body delimiter;
 *     - malformed closure-capture clause where applicable.
 *
 * Semantic diagnostics belong downstream.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Existing canonical source form:
 *
 *     |parameterList| body
 *
 * MUST remain valid.
 *
 * This file does not silently replace that syntax with:
 *
 *     lambda(...)
 *     fn(...)
 *     => ...
 *
 * Such alternate syntaxes require explicit language-version/specification
 * approval and compatibility treatment.
 *
 * ============================================================================
 * NO HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
 *
 *     hardware IDs
 *     physical addresses
 *     CPU counts
 *     GPU counts
 *     FPGA sizes
 *     QPU sizes
 *     qubit limits
 *     memory limits
 *     tensor limits
 *     thread limits
 *     node limits
 *     fixed lambda arity
 *     fixed nesting depth
 *
 * Repetition is structural and unbounded by language-level constants.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *   [x] lambdaExpression is the sole public lambda entry point;
 *   [x] canonical parameterList is reused;
 *   [x] canonical typeExpression is reused;
 *   [x] canonical expression is reused;
 *   [x] canonical blockExpression is reused;
 *   [x] closure capture syntax is delegated to closures.g4;
 *   [x] no lexer rules are duplicated;
 *   [x] no second expression hierarchy is created;
 *   [x] no second parameter hierarchy is created;
 *   [x] no domain-specific lambda grammar is created;
 *   [x] source order is preserved;
 *   [x] empty parameter lists are supported;
 *   [x] arbitrary parameter counts are supported;
 *   [x] expression bodies are supported;
 *   [x] block bodies are supported;
 *   [x] optional return types are supported;
 *   [x] AST mapping is predetermined;
 *   [x] semantic ownership is predetermined;
 *   [x] quantum::ir remains downstream;
 *   [x] POCO-REAF is preserved;
 *   [x] no machine limits are encoded;
 *   [x] negative cases are testable;
 *   [x] boundary cases are testable;
 *   [x] scalability cases are testable;
 *   [x] deterministic parsing is preserved.
 *
 * ============================================================================
 */

parser grammar Lambdas;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * Canonical form:
 *
 *     |parameterList| body
 *
 * Optional explicit closure capture syntax and asynchronous syntax are kept
 * outside the parameter/body representation.
 *
 * The capture rule is supplied by functions/closures.g4.
 *
 * `ASYNC` is consumed only when the canonical lexer/specification has promoted
 * it to lambda syntax. It does not imply a particular runtime scheduler.
 */
lambdaExpression
    : closureCaptureClause?
      lambdaModifier*
      lambdaParameterClause
      lambdaReturnTypeClause?
      lambdaBody
    ;


/*
 * ============================================================================
 * LAMBDA MODIFIERS
 * ============================================================================
 *
 * These are source-level modifiers only.
 *
 * They do not select hardware or execution resources.
 *
 * `MOVE` is a capture/execution-intent modifier whose legality is determined
 * semantically.
 *
 * `ASYNC` describes asynchronous source intent; scheduling remains downstream.
 */
lambdaModifier
    : ASYNC
    | MOVE
    ;


/*
 * ============================================================================
 * PARAMETER CLAUSE
 * ============================================================================
 *
 * The pipe delimiters are owned lexically by ZamaniLexer.
 *
 * The parameter contents are delegated to the canonical parameter grammar.
 *
 * This avoids creating a second lambda-specific parameter representation.
 */
lambdaParameterClause
    : PIPE parameterList? PIPE
    ;


/*
 * ============================================================================
 * RETURN TYPE
 * ============================================================================
 *
 * Explicit return type is source-level type syntax.
 *
 * Type semantics remain owned by the type system.
 *
 * Example:
 *
 *     |x| -> Int x + 1
 */
lambdaReturnTypeClause
    : ARROW typeExpression
    ;


/*
 * ============================================================================
 * BODY
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
 * No third lambda-specific body representation is introduced.
 */
lambdaBody
    : lambdaExpressionBody
    | lambdaBlockBody
    ;


lambdaExpressionBody
    : expression
    ;


lambdaBlockBody
    : blockExpression
    ;
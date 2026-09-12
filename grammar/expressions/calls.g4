/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/expressions/calls.g4
 *
 * Status:
 *     Production parser-grammar component.
 *
 * Purpose:
 *     Canonical source-level syntax for callable invocation and argument
 *     passing.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar.
 *
 * Rust integration baseline:
 *     Rust 1.97 / Rust 1.97.1.
 *
 * Safety:
 *     - No embedded Rust actions.
 *     - No embedded Rust predicates.
 *     - No filesystem access.
 *     - No network access.
 *     - No process execution.
 *     - No hardware discovery.
 *     - No runtime execution.
 *     - No unsafe Rust.
 *
 * ============================================================================
 * ARCHITECTURAL ROLE
 * ============================================================================
 *
 * Source
 *   |
 *   v
 * Canonical Zamani Lexer
 *   |
 *   v
 * Parser
 *   |
 *   v
 * Frontend AST
 *   |
 *   +--> name resolution
 *   +--> overload resolution
 *   +--> generic/type resolution
 *   +--> effect checking
 *   +--> capability checking
 *   +--> resource checking
 *   |
 *   v
 * Canonical semantic representation
 *   |
 *   +--> classical IR
 *   +--> quantum::ir
 *   +--> HDL / hardware IR
 *   +--> distributed/control/data IR
 *   |
 *   v
 * optimization / lowering / routing / scheduling
 *   |
 *   v
 * target realization
 *
 * This file describes syntax only.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - call suffix syntax;
 *   - invocation parentheses;
 *   - argument-list syntax;
 *   - positional arguments;
 *   - named arguments;
 *   - spread arguments;
 *   - optional trailing commas;
 *   - call-site generic/type arguments;
 *   - argument ordering as written;
 *   - call-site source structure;
 *   - syntactic call-expression compatibility aliases.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - identifier spelling;
 *   - qualified-name spelling;
 *   - function declarations;
 *   - method declarations;
 *   - closure declarations;
 *   - lambda declarations;
 *   - function types;
 *   - type definitions;
 *   - generic type definitions;
 *   - overload resolution;
 *   - name resolution;
 *   - callable-type checking;
 *   - arity validation;
 *   - parameter matching;
 *   - default-argument evaluation;
 *   - ownership checking;
 *   - borrow checking;
 *   - effect checking;
 *   - capability checking;
 *   - resource allocation;
 *   - hardware selection;
 *   - quantum operation semantics;
 *   - quantum::ir;
 *   - QEC;
 *   - ZQN;
 *   - optimization;
 *   - scheduling;
 *   - routing;
 *   - runtime dispatch;
 *   - ABI selection.
 *
 * ============================================================================
 * IMPORTANT MODULAR-GRAMMAR RULE
 * ============================================================================
 *
 * The canonical expression aggregator owns expression precedence.
 *
 * This file owns the invocation component used by the postfix-expression
 * layer.
 *
 * Conceptually:
 *
 *     primaryExpression
 *          |
 *          v
 *     postfixExpression
 *          |
 *          +--> callSuffix
 *          +--> indexSuffix
 *          +--> memberSuffix
 *          +--> other postfix components
 *
 * calls.g4 MUST NOT become a second expression-precedence hierarchy.
 *
 * ============================================================================
 * CANONICAL PARSER COMPOSITION
 * ============================================================================
 *
 * The authoritative expression grammar should compose this grammar so that:
 *
 *     postfixExpression
 *         -> primaryExpression postfixPart*
 *
 * and:
 *
 *     postfixPart
 *         -> callSuffix
 *
 * The older inline definitions in:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * and the duplicate call definitions currently present in:
 *
 *     grammar/expressions/expressions.g4
 *
 * must eventually be removed from those ownership locations when the
 * authoritative modular parser is assembled.
 *
 * They must not remain as independent competing definitions.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * Tokens are supplied by the canonical Zamani lexer.
 *
 * Required tokens:
 *
 *     LPAREN
 *     RPAREN
 *     COMMA
 *     COLON
 *     ASSIGN
 *     ELLIPSIS
 *     LESS_THAN
 *     GREATER_THAN
 *     DOUBLE_COLON
 *
 * The lexer owns their textual spelling.
 *
 * This parser grammar MUST NOT redefine lexer tokens.
 *
 * Existing repository lexer infrastructure already treats punctuation such
 * as LPAREN, RPAREN, COMMA and COLON as lexical structure, while operators
 * such as ELLIPSIS and DOUBLE_COLON belong to the appropriate operator
 * vocabulary.  
 *
 * ============================================================================
 * OPEN-WORLD CALLABLE MODEL
 * ============================================================================
 *
 * A call target is intentionally not restricted to:
 *
 *     functionName
 *
 * A callable expression may eventually be:
 *
 *     function
 *     method
 *     closure
 *     lambda
 *     returned function
 *     generic callable
 *     object implementing a call capability
 *     hardware abstraction
 *     accelerator abstraction
 *     quantum operation abstraction
 *     distributed service abstraction
 *     future callable construct
 *
 * The grammar therefore does not maintain a finite list of callable names.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This file contains no fixed limits such as:
 *
 *     MAX_ARGUMENTS
 *     MAX_PARAMETERS
 *     MAX_CALL_DEPTH
 *     MAX_GENERIC_ARGUMENTS
 *     MAX_NESTING
 *     MAX_CALLABLES
 *     MAX_FUNCTIONS
 *     MAX_METHODS
 *     MAX_DEVICES
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_NODES
 *     MAX_ACCELERATORS
 *
 * Lists and postfix chains use unbounded grammar repetition.
 *
 * Practical limits are implementation/resource limits and MUST NOT become
 * language semantics.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * A call expresses:
 *
 *     invoke this callable with these source-level arguments.
 *
 * It does NOT express:
 *
 *     use CPU X
 *     use GPU Y
 *     use QPU Z
 *     use physical qubit N
 *     use node N
 *     use accelerator N
 *     use topology T
 *
 * Those decisions belong downstream to:
 *
 *     capability analysis
 *     resource analysis
 *     compilation
 *     placement
 *     routing
 *     scheduling
 *     deployment
 *     runtime dispatch
 *
 * ============================================================================
 * ARGUMENT EVALUATION
 * ============================================================================
 *
 * This grammar preserves source ordering.
 *
 * It does NOT determine:
 *
 *     - evaluation strategy;
 *     - eager/lazy evaluation;
 *     - parallel evaluation;
 *     - memoization;
 *     - constant folding;
 *     - argument ownership;
 *     - move/copy semantics;
 *     - quantum measurement timing;
 *     - hardware side effects.
 *
 * Those are semantic/compiler concerns.
 *
 * ============================================================================
 * NAMED ARGUMENT COMPATIBILITY
 * ============================================================================
 *
 * Existing Zamani material contains two named-argument spellings:
 *
 *     name = expression
 *
 * and:
 *
 *     name: expression
 *
 * The language documentation contains the former, while existing grammar
 * infrastructure contains the latter.  
 *
 * To prevent silent loss of an existing feature during modularization,
 * this grammar accepts BOTH forms at the syntax level.
 *
 * The semantic/compatibility layer must establish the canonical source
 * spelling and migration policy.
 *
 * The AST must preserve which delimiter was written if source-preserving
 * formatting or migration diagnostics require that information.
 *
 * ============================================================================
 * SPREAD ARGUMENTS
 * ============================================================================
 *
 * The canonical lexer already defines ELLIPSIS (`...`). 
 *
 * This grammar uses it for argument expansion:
 *
 *     f(...values)
 *
 * Whether the expanded value is:
 *
 *     iterable
 *     tuple-like
 *     parameter-pack
 *     hardware argument pack
 *     quantum operand pack
 *     distributed argument pack
 *
 * is a semantic question.
 *
 * ============================================================================
 * GENERIC CALLS
 * ============================================================================
 *
 * Generic/type arguments at a call site are syntactically supported through:
 *
 *     ::
 *
 * followed by:
 *
 *     < ... >
 *
 * followed by the invocation parentheses.
 *
 * Example:
 *
 *     compute::<T>(value)
 *
 * The semantic layer decides whether:
 *
 *     - the callable is generic;
 *     - the supplied arguments are valid;
 *     - inference is required;
 *     - explicit arguments conflict with inferred arguments.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Calls may represent quantum operations at the source level:
 *
 *     H(q)
 *     measure(q)
 *     reset(q)
 *     custom_operation(q0, q1)
 *
 * This grammar does not define gate inventories.
 *
 * It does not define:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     quantum::ir
 *     topology
 *     calibration
 *     QEC
 *     ZQN
 *
 * Quantum semantic lowering remains downstream.
 *
 * Therefore a call such as:
 *
 *     H(q)
 *
 * remains portable source syntax rather than an instruction to select a
 * particular physical machine.
 *
 * ============================================================================
 * HARDWARE / HDL INTEGRATION
 * ============================================================================
 *
 * Calls may represent hardware-independent abstractions:
 *
 *     accelerator.run(data)
 *     device.configure(...)
 *     signal.drive(...)
 *
 * The grammar does not determine whether a callable maps to:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     embedded peripheral
 *     remote service
 *     distributed node
 *     future accelerator
 *
 * Semantic and target layers determine realization.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing a call MUST NOT execute it.
 *
 * For example:
 *
 *     system.run(...)
 *
 * is only syntax during parsing.
 *
 * The parser must never:
 *
 *     - execute commands;
 *     - load libraries;
 *     - open files;
 *     - contact networks;
 *     - access credentials;
 *     - inspect hardware;
 *     - invoke a backend.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source token stream
 *     grammar version
 *
 * It must not depend on:
 *
 *     system time
 *     randomness
 *     environment variables
 *     hardware state
 *     filesystem contents
 *     network state
 *     installed devices
 *     backend availability
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend AST should represent calls conceptually as:
 *
 *     CallExpression {
 *         callee,
 *         generic_arguments?,
 *         arguments,
 *         source_span
 *     }
 *
 * Each argument should preserve:
 *
 *     Positional(expression)
 *     Named(name, expression, delimiter)
 *     Spread(expression)
 *
 * The grammar does not define the Rust AST types.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     - resolving the callable;
 *     - resolving generic arguments;
 *     - checking argument count;
 *     - matching positional arguments;
 *     - matching named arguments;
 *     - checking duplicate names;
 *     - checking ordering rules;
 *     - validating spread arguments;
 *     - checking parameter types;
 *     - checking implicit conversions;
 *     - checking ownership;
 *     - checking borrowing;
 *     - checking effects;
 *     - checking capabilities;
 *     - checking resource requirements;
 *     - determining invocation semantics.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * The grammar does not directly create:
 *
 *     classical IR
 *     quantum::ir
 *     HDL IR
 *     hardware IR
 *     runtime call objects
 *
 * The frontend AST / semantic layer performs the appropriate lowering.
 *
 * For quantum programs:
 *
 *     source call
 *         ->
 *     frontend semantic representation
 *         ->
 *     canonical quantum::ir
 *
 * No duplicate quantum gate/call IR is permitted here.
 *
 * ============================================================================
 * COMPILER CONTRACT
 * ============================================================================
 *
 * The compiler may subsequently:
 *
 *     inline
 *     specialize
 *     devirtualize
 *     vectorize
 *     distribute
 *     lower
 *     route
 *     schedule
 *     map
 *     optimize
 *
 * None of these transformations belong to this grammar.
 *
 * ============================================================================
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime dispatch is downstream.
 *
 * A parsed call does not imply immediate execution.
 *
 * Runtime may eventually resolve the callable using:
 *
 *     static binding
 *     dynamic binding
 *     capability negotiation
 *     resource negotiation
 *     distributed service resolution
 *     hardware abstraction
 *     quantum backend selection
 *
 * without changing the source grammar.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing call syntax must remain representable:
 *
 *     f()
 *     f(x)
 *     f(x, y)
 *     f(x,)
 *     f(x, y,)
 *     object.method(x)
 *     namespace::function(x)
 *     f(name = value)
 *     f(name: value)
 *     f(...values)
 *
 * Generic invocation:
 *
 *     f::<T>(x)
 *     f::<T, U>(x, y)
 *
 * is supported where the surrounding type grammar supplies the referenced
 * type expression.
 *
 * ============================================================================
 */

parser grammar Calls;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 1. CANONICAL CALL SUFFIX
 * ============================================================================
 *
 * This is the primary rule consumed by postfixExpression.
 *
 * Examples:
 *
 *     value()
 *     value(x)
 *     value(x, y)
 *     value(x,)
 */
callSuffix
    : callTypeArguments?
      LPAREN
      argumentList?
      RPAREN
    ;


/*
 * ============================================================================
 * 2. GENERIC / TYPE ARGUMENTS AT CALL SITE
 * ============================================================================
 *
 * Example:
 *
 *     function::<T>(value)
 *     function::<T, U>(value)
 *
 * DOUBLE_COLON separates ordinary qualified/member syntax from explicit
 * call-site type arguments.
 *
 * The actual type grammar remains owned by the types subsystem.
 */
callTypeArguments
    : DOUBLE_COLON
      LESS_THAN
      callTypeArgumentList
      GREATER_THAN
    ;


/*
 * ============================================================================
 * 3. CALL TYPE-ARGUMENT LIST
 * ============================================================================
 *
 * No fixed generic arity.
 */
callTypeArgumentList
    : typeExpression
      (
          COMMA
          typeExpression
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 4. ARGUMENT LIST
 * ============================================================================
 *
 * No fixed argument count.
 *
 * A trailing comma is permitted:
 *
 *     f(a, b,)
 *
 * Empty calls are represented by absence of argumentList:
 *
 *     f()
 */
argumentList
    : argument
      (
          COMMA
          argument
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 5. ARGUMENT
 * ============================================================================
 *
 * Ordering of alternatives is deliberate:
 *
 *     named
 *     spread
 *     positional
 *
 * A named argument begins with an identifier followed by `=` or `:`.
 *
 * A spread argument begins with ELLIPSIS.
 *
 * Everything else is an ordinary expression.
 */
argument
    : namedArgument
    | spreadArgument
    | positionalArgument
    ;


/*
 * ============================================================================
 * 6. POSITIONAL ARGUMENT
 * ============================================================================
 *
 * The expression grammar owns expression semantics.
 */
positionalArgument
    : expression
    ;


/*
 * ============================================================================
 * 7. NAMED ARGUMENT
 * ============================================================================
 *
 * Both existing Zamani forms are accepted:
 *
 *     name = expression
 *
 *     name : expression
 *
 * Semantic compatibility policy determines the canonical spelling.
 */
namedArgument
    : identifier
      (
          ASSIGN
        | COLON
      )
      expression
    ;


/*
 * ============================================================================
 * 8. SPREAD ARGUMENT
 * ============================================================================
 *
 * Example:
 *
 *     f(...values)
 *
 * ELLIPSIS is lexically owned by the canonical lexer.
 */
spreadArgument
    : ELLIPSIS
      expression
    ;


/*
 * ============================================================================
 * 9. OPTIONAL ARGUMENT LIST
 * ============================================================================
 *
 * Convenience rule for grammar consumers that need an explicit optional
 * invocation argument list.
 */
optionalArgumentList
    : argumentList?
    ;


/*
 * ============================================================================
 * 10. COMPATIBILITY CALL EXPRESSION
 * ============================================================================
 *
 * Existing Zamani grammar infrastructure uses the rule name `callExpression`
 * for the parenthesized invocation suffix.
 *
 * Keep this compatibility alias during migration so existing parser
 * composition and tooling can transition to `callSuffix` without changing
 * call semantics.
 *
 * The authoritative modular postfix grammar should prefer:
 *
 *     callSuffix
 *
 * over:
 *
 *     callExpression
 */
callExpression
    : callSuffix
    ;


/*
 * ============================================================================
 * 11. CALL ARGUMENT VALUE
 * ============================================================================
 *
 * This rule exists as a stable semantic-parser boundary for tooling that
 * wants to inspect the expression/value portion of an argument without
 * depending on the argument variant.
 */
callArgumentValue
    : expression
    ;


/*
 * ============================================================================
 * 12. CALLABLE INVOCATION CONTRACT
 * ============================================================================
 *
 * The grammar intentionally does not define:
 *
 *     callableExpression
 *
 * here.
 *
 * The expression subsystem owns:
 *
 *     primaryExpression
 *     postfixExpression
 *     member access
 *     indexing
 *     qualified names
 *     closures
 *     lambdas
 *     function values
 *
 * A callable is whatever expression is accepted by the postfix-expression
 * layer before this call suffix is applied.
 *
 * Examples therefore include:
 *
 *     f()
 *     object.f()
 *     array[index]()
 *     make_function()()
 *     closure(value)
 *
 * subject to the surrounding expression grammar.
 */


/*
 * ============================================================================
 * 13. SOURCE ORDER PRESERVATION
 * ============================================================================
 *
 * The parser preserves argument order exactly as written.
 *
 * For:
 *
 *     f(a, b = c, ...rest)
 *
 * the AST must retain:
 *
 *     1. positional a
 *     2. named b = c
 *     3. spread rest
 *
 * Reordering, normalization, duplicate detection, and parameter binding belong
 * to semantic analysis.
 */


/*
 * ============================================================================
 * 14. NO ARGUMENT SEMANTICS IN THE GRAMMAR
 * ============================================================================
 *
 * This grammar intentionally accepts structurally valid forms even when
 * semantic analysis may reject them.
 *
 * Examples:
 *
 *     f(x, x = y)
 *     f(a = x, b)
 *     f(...x, ...y)
 *
 * Whether these are legal depends on the language's parameter-binding rules.
 *
 * The grammar must not encode those semantic policies as arbitrary syntax
 * restrictions unless the language specification explicitly makes them
 * syntactic requirements.
 */


/*
 * ============================================================================
 * 15. QUANTUM CALL CONTRACT
 * ============================================================================
 *
 * Examples:
 *
 *     H(q)
 *     measure(q)
 *     reset(q)
 *     controlled(H, control, target)
 *
 * are represented as ordinary call syntax.
 *
 * The parser does not maintain:
 *
 *     gate tables
 *     qubit counts
 *     physical topology
 *     device IDs
 *     calibration data
 *     error models
 *
 * Quantum semantics are lowered downstream into the canonical quantum IR.
 */


/*
 * ============================================================================
 * 16. HARDWARE / HDL CALL CONTRACT
 * ============================================================================
 *
 * Examples:
 *
 *     accelerator.run(data)
 *     signal.drive(value)
 *     register.write(value)
 *
 * remain source-level calls.
 *
 * The grammar does not determine whether the target is:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     embedded device
 *     distributed service
 *     future accelerator
 *
 * Target realization remains downstream.
 */


/*
 * ============================================================================
 * 17. DISTRIBUTED CALL CONTRACT
 * ============================================================================
 *
 * A distributed invocation can use exactly the same call syntax:
 *
 *     service.execute(request)
 *
 * Remote/local semantics are not encoded in the parentheses themselves.
 *
 * Placement, transport, serialization, consistency, retry and fault handling
 * belong to semantic/runtime/distributed subsystems.
 */


/*
 * ============================================================================
 * 18. AI / DATA CALL CONTRACT
 * ============================================================================
 *
 * The same syntax supports:
 *
 *     model(input)
 *     tensor.reshape(shape)
 *     dataset.map(transform)
 *     pipeline.execute(data)
 *
 * No AI-specific callable inventory belongs in this grammar.
 */


/*
 * ============================================================================
 * 19. SOURCE-PRESERVING AST CONTRACT
 * ============================================================================
 *
 * Frontend AST concept:
 *
 *     CallExpression
 *         callee
 *         generic_arguments?
 *         arguments
 *         span
 *
 * Argument:
 *
 *     Positional
 *         expression
 *
 *     Named
 *         name
 *         expression
 *         delimiter
 *
 *     Spread
 *         expression
 *
 * Generic arguments:
 *
 *     ordered type expressions
 *
 * The AST layer should preserve source spans for:
 *
 *     call
 *     callee
 *     type arguments
 *     each argument
 *     argument name
 *     argument delimiter
 *     spread marker
 *     opening parenthesis
 *     closing parenthesis
 *
 * This supports precise diagnostics, refactoring, formatting, migration and
 * source-to-source tooling.
 */


/*
 * ============================================================================
 * 20. SEMANTIC VALIDATION CONTRACT
 * ============================================================================
 *
 * The semantic layer MUST validate:
 *
 *     callable existence
 *     callable kind
 *     callable visibility
 *     callable capability
 *     generic arity
 *     generic constraints
 *     argument arity
 *     named argument validity
 *     duplicate named arguments
 *     positional/named ordering
 *     spread validity
 *     parameter compatibility
 *     conversion rules
 *     ownership
 *     borrowing
 *     effects
 *     resource requirements
 *     quantum/classical boundaries
 *     hardware capability requirements
 *
 * None of those validations belong in calls.g4.
 */


/*
 * ============================================================================
 * 21. DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics cover syntax.
 *
 * Examples:
 *
 *     f(
 *     f(a,
 *     f(,a)
 *     f(a,,b)
 *     f(name =)
 *     f(name:)
 *     f(... )
 *
 * Semantic diagnostics cover:
 *
 *     unknown function
 *     wrong argument count
 *     unknown named argument
 *     duplicate named argument
 *     invalid spread value
 *     invalid generic argument
 *     incompatible argument type
 *     unavailable capability
 *     unavailable resource
 *
 * Parser and semantic diagnostics must remain separate.
 */


/*
 * ============================================================================
 * 22. DETERMINISM CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no actions
 *     no predicates
 *     no randomness
 *     no environment access
 *     no I/O
 *     no hardware queries
 *
 * Identical token streams under the same grammar version must produce the
 * same parse structure.
 */


/*
 * ============================================================================
 * 23. COMPATIBILITY / EVOLUTION CONTRACT
 * ============================================================================
 *
 * New callable categories must NOT require changes here merely because a new
 * backend or computational domain is introduced.
 *
 * Examples:
 *
 *     new quantum gate
 *     new accelerator
 *     new AI operator
 *     new distributed service
 *     new hardware primitive
 *     new future computational substrate
 *
 * should be represented through existing callable syntax whenever their
 * source-level invocation model is unchanged.
 *
 * New argument forms require:
 *
 *     lexer contract
 *     grammar update
 *     AST update
 *     semantic update
 *     tests
 *     compatibility documentation
 *
 * and must not silently alter existing forms.
 */


/*
 * ============================================================================
 * 24. TEST CONTRACT
 * ============================================================================
 *
 * POSITIVE:
 *
 *     f()
 *     f(x)
 *     f(x, y)
 *     f(x,)
 *     f(x, y,)
 *     object.f(x)
 *     namespace::f(x)
 *     f(name = value)
 *     f(name: value)
 *     f(...values)
 *     f(x, ...values)
 *     f::<T>(x)
 *     f::<T, U>(x, y)
 *
 * NESTED:
 *
 *     f(g(x))
 *     f(g(x), h(y))
 *     f(object.method(x))
 *     f(make_function()(x))
 *
 * QUANTUM:
 *
 *     H(q)
 *     measure(q)
 *     reset(q)
 *     custom_gate(control, target)
 *
 * HARDWARE:
 *
 *     accelerator.run(data)
 *     register.write(value)
 *
 * DISTRIBUTED:
 *
 *     service.execute(request)
 *
 * AI / DATA:
 *
 *     model(input)
 *     tensor.reshape(shape)
 *     dataset.map(transform)
 *
 * NEGATIVE:
 *
 *     f(
 *     f(,
 *     f(a,,b)
 *     f(name =)
 *     f(name:)
 *     f(... )
 *
 * BOUNDARY:
 *
 *     zero arguments
 *     one argument
 *     many arguments
 *     deeply nested calls
 *     deeply nested generic calls
 *     long argument lists
 *     long postfix chains
 *
 * SCALABILITY:
 *
 *     no grammar-level argument maximum
 *     no grammar-level generic-argument maximum
 *     no grammar-level nesting constant
 *     no machine-size dependency
 *
 * DETERMINISM:
 *
 *     same source -> same token stream -> same parse structure
 *
 * ROUND TRIP:
 *
 *     source
 *       -> lexer
 *       -> parser
 *       -> AST
 *       -> formatter/serializer
 *       -> parser
 *
 * must preserve intended call semantics.
 */


/*
 * ============================================================================
 * 25. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
 *
 *     MAX_ARGUMENTS
 *     MAX_PARAMETERS
 *     MAX_GENERIC_ARGUMENTS
 *     MAX_CALL_DEPTH
 *     MAX_CALLABLES
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_ACCELERATORS
 *
 * Any implementation/resource ceiling belongs outside the language grammar.
 */


/*
 * ============================================================================
 * 26. COMPLETION CRITERIA
 * ============================================================================
 *
 * calls.g4 is COMPLETE only when:
 *
 * [ ] canonical ZamaniLexer vocabulary is used;
 * [ ] no lexer tokens are duplicated here;
 * [ ] callSuffix is the canonical invocation rule;
 * [ ] argumentList is unbounded;
 * [ ] positional arguments are supported;
 * [ ] named arguments are supported;
 * [ ] existing `=` named arguments remain parseable;
 * [ ] existing `:` named arguments remain parseable during compatibility;
 * [ ] spread arguments are supported;
 * [ ] trailing commas are supported;
 * [ ] empty calls are supported;
 * [ ] generic call-site arguments are supported;
 * [ ] callExpression compatibility alias is retained during migration;
 * [ ] no expression-precedence hierarchy is duplicated;
 * [ ] no semantic actions exist;
 * [ ] no Rust actions exist;
 * [ ] no unsafe implementation is required;
 * [ ] no hardware assumptions exist;
 * [ ] no quantum-machine assumptions exist;
 * [ ] no fixed argument/resource limits exist;
 * [ ] AST contract is implemented downstream;
 * [ ] semantic contract is implemented downstream;
 * [ ] quantum lowering reaches canonical quantum::ir;
 * [ ] existing parser consumers are migrated;
 * [ ] duplicate inline call rules are removed from the authoritative
 *     expression aggregator;
 * [ ] positive tests pass;
 * [ ] negative tests pass;
 * [ ] boundary tests pass;
 * [ ] cross-domain tests pass;
 * [ ] determinism tests pass;
 * [ ] round-trip tests pass.
 *
 * ============================================================================
 */
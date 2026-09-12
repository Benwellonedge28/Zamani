/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/expressions/lambdas.g4
 *
 * Role:
 *     Canonical source-level lambda-expression grammar.
 *
 * Language:
 *     Zamani
 *
 * Minimum implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     The Rust implementation MUST compile with unsafe code forbidden.
 *     This grammar contains no Rust implementation code.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - lambda-expression syntax;
 *   - lambda parameter syntax specific to expressions;
 *   - lambda parameter patterns;
 *   - optional lambda parameter type annotations;
 *   - optional lambda return-type annotations;
 *   - lambda body syntax;
 *   - expression-bodied lambdas;
 *   - block-bodied lambdas;
 *   - async lambda syntax;
 *   - move/capture-mode syntax where supported by the language contract;
 *   - lambda attributes/modifiers that are explicitly part of lambda syntax;
 *   - lambda-local generic parameter syntax where the language specification
 *     permits it;
 *   - lambda syntax needed by expression-level higher-order computation.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexer definitions;
 *   - identifiers;
 *   - keywords;
 *   - operators;
 *   - punctuation tokens;
 *   - general function declarations;
 *   - ordinary function parameter declarations;
 *   - type definitions;
 *   - type checking;
 *   - generic type checking;
 *   - closure capture analysis;
 *   - ownership checking;
 *   - borrow checking;
 *   - lifetime checking;
 *   - effect checking;
 *   - capability checking;
 *   - resource checking;
 *   - hardware selection;
 *   - quantum resource allocation;
 *   - quantum IR;
 *   - classical IR;
 *   - scheduling;
 *   - routing;
 *   - optimization;
 *   - QEC;
 *   - ZQN;
 *   - simulation;
 *   - runtime closure representation;
 *   - ABI selection;
 *   - backend selection;
 *   - machine-specific limits.
 *
 * ============================================================================
 * ARCHITECTURAL BOUNDARY
 * ============================================================================
 *
 * Source
 *   |
 *   v
 * ZamaniLexer
 *   |
 *   v
 * Lambda parser rules
 *   |
 *   v
 * Frontend AST
 *   |
 *   +--> name resolution
 *   +--> type checking
 *   +--> effect checking
 *   +--> ownership / borrow analysis
 *   +--> capability analysis
 *   +--> resource analysis
 *   |
 *   v
 * Canonical semantic representation
 *   |
 *   +--> classical IR
 *   +--> quantum::ir
 *   +--> control/data IR
 *   +--> resource/effect metadata
 *   |
 *   v
 * optimization
 *   |
 *   v
 * scheduling / routing / lowering
 *   |
 *   v
 * runtime / hardware
 *
 * A lambda is SOURCE SYNTAX.
 *
 * A lambda MUST NOT itself decide:
 *
 *   - CPU vs GPU;
 *   - CPU core count;
 *   - thread count;
 *   - memory size;
 *   - accelerator count;
 *   - FPGA size;
 *   - ASIC resources;
 *   - quantum processor;
 *   - qubit count;
 *   - physical qubit placement;
 *   - network topology;
 *   - execution location.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Lambda syntax is architecture independent.
 *
 * Examples:
 *
 *     |x| x + 1
 *
 *     |x, y| x * y
 *
 *     |x: Int, y: Int| x + y
 *
 *     |x| {
 *         let y = x + 1;
 *         return y;
 *     }
 *
 * The number of parameters is not artificially capped.
 *
 * No machine resource count is encoded by this grammar.
 *
 * Any resource limitation belongs to:
 *
 *     semantic analysis
 *     compilation
 *     resource management
 *     scheduling
 *     runtime
 *     deployment
 *
 * ============================================================================
 * ANTLR INTEGRATION CONTRACT
 * ============================================================================
 *
 * This is a PARSER grammar.
 *
 * It intentionally defines NO lexer rules.
 *
 * Tokens MUST come from:
 *
 *     ZamaniLexer
 *
 * via:
 *
 *     options {
 *         tokenVocab = ZamaniLexer;
 *     }
 *
 * Do not add local definitions for:
 *
 *     IDENT
 *     PIPE
 *     COMMA
 *     COLON
 *     ARROW
 *     LPAREN
 *     RPAREN
 *     LBRACE
 *     RBRACE
 *     ASYNC
 *     MOVE
 *     MUT
 *     etc.
 *
 * If a required token does not exist in ZamaniLexer, the lexer is the correct
 * ownership boundary to address it. Do not create a second lexer here.
 *
 * ============================================================================
 * IMPORTANT IMPORT/DEPENDENCY RULE
 * ============================================================================
 *
 * This grammar is deliberately kept as a lambda syntax component.
 *
 * The integration layer MUST expose:
 *
 *     lambdaExpression
 *
 * to the canonical expression grammar.
 *
 * The canonical expression grammar remains responsible for deciding where a
 * lambda is valid as an expression.
 *
 * Do NOT create a circular parser dependency:
 *
 *     Expressions -> Lambdas -> Expressions
 *
 * Instead, the integration layer should expose the expression rule to this
 * component through the repository's canonical parser composition mechanism.
 *
 * If the ANTLR build uses grammar imports, the final composition must provide
 * the lambda body with access to the canonical expression rule without
 * introducing a recursive grammar-import cycle.
 *
 * ============================================================================
 */

parser grammar Lambdas;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ========================================================================== */

/**
 * Complete lambda expression.
 *
 * Supported conceptual forms include:
 *
 *     |x| x + 1
 *     |x, y| x + y
 *     |x: Int| x + 1
 *     |x| -> Int { x + 1 }
 *     |x| {
 *         return x + 1;
 *     }
 *
 * Optional lambda modifiers are intentionally limited to syntax whose
 * semantics can be represented independently of a target machine.
 */
lambdaExpression
    : lambdaModifier*
      lambdaParameterClause
      lambdaReturnTypeClause?
      lambdaBody
    ;


/* ============================================================================
 * 2. LAMBDA MODIFIERS
 * ========================================================================== */

/**
 * Lambda modifiers are syntactic annotations of lambda execution/capture
 * intent.
 *
 * They do not select hardware.
 *
 * The semantic layer determines whether a requested modifier is valid in the
 * surrounding context.
 */
lambdaModifier
    : ASYNC
    | MOVE
    ;


/* ============================================================================
 * 3. PARAMETER CLAUSE
 * ========================================================================== */

/**
 * Canonical Zamani lambda parameter clause.
 *
 * The pipe delimiters make lambda syntax unambiguous from ordinary function
 * calls and parenthesized expressions.
 *
 * Examples:
 *
 *     || x
 *
 *     |x| x + 1
 *
 *     |x, y| x + y
 *
 *     |x: Int, y: Int| x + y
 *
 *     |mut x| x += 1
 *
 * No finite parameter-count limit is encoded.
 */
lambdaParameterClause
    : PIPE lambdaParameterList? PIPE
    ;


/**
 * Lambda parameter list.
 *
 * A trailing comma is accepted deliberately for source compatibility and
 * formatter-friendly generated code.
 */
lambdaParameterList
    : lambdaParameter
      (COMMA lambdaParameter)*
      COMMA?
    ;


/**
 * A lambda parameter is an expression-level parameter.
 *
 * It is deliberately NOT the same grammar rule as an ordinary function
 * parameter because lambda parameters may use expression-specific capture
 * and pattern forms.
 */
lambdaParameter
    : lambdaParameterModifier*
      lambdaParameterPattern
      lambdaParameterTypeClause?
      lambdaParameterDefaultClause?
    ;


/**
 * Parameter modifiers are semantic hints/requirements rather than machine
 * placement directives.
 */
lambdaParameterModifier
    : MUT
    ;


/* ============================================================================
 * 4. PARAMETER PATTERNS
 * ========================================================================== */

/**
 * Lambda parameter patterns.
 *
 * The grammar supports:
 *
 *     |x| ...
 *
 *     |(x, y)| ...
 *
 *     |[x, y]| ...
 *
 *     |Some(value)| ...
 *
 *     |_ | ...
 *
 * Actual pattern compatibility is checked by semantic analysis.
 */
lambdaParameterPattern
    : lambdaBindingPattern
    | lambdaWildcardPattern
    | lambdaTuplePattern
    | lambdaArrayPattern
    | lambdaStructPattern
    | lambdaEnumPattern
    | lambdaParenthesizedPattern
    ;


/**
 * Simple binding.
 */
lambdaBindingPattern
    : identifier
    ;


/**
 * Wildcard parameter.
 *
 * The underscore token intentionally means that the incoming value is not
 * semantically bound by name.
 */
lambdaWildcardPattern
    : UNDERSCORE
    ;


/**
 * Tuple destructuring.
 *
 * At least two elements are required so that:
 *
 *     (x)
 *
 * remains a parenthesized pattern rather than a tuple.
 */
lambdaTuplePattern
    : LPAREN
      lambdaPatternElement
      COMMA
      lambdaPatternElement
      (COMMA lambdaPatternElement)*
      COMMA?
      RPAREN
    ;


/**
 * Array-style destructuring.
 */
lambdaArrayPattern
    : LBRACKET
      lambdaPatternElementList?
      RBRACKET
    ;


lambdaPatternElementList
    : lambdaPatternElement
      (COMMA lambdaPatternElement)*
      COMMA?
    ;


lambdaPatternElement
    : lambdaParameterPattern
    ;


/**
 * Struct-like destructuring.
 *
 * Example:
 *
 *     |Point { x, y }| ...
 *
 * The semantic layer resolves the named type and fields.
 */
lambdaStructPattern
    : qualifiedName
      LBRACE
      lambdaStructPatternFieldList?
      RBRACE
    ;


lambdaStructPatternFieldList
    : lambdaStructPatternField
      (COMMA lambdaStructPatternField)*
      COMMA?
    ;


lambdaStructPatternField
    : identifier
      (COLON lambdaParameterPattern)?
    ;


/**
 * Enum/variant destructuring.
 *
 * Examples:
 *
 *     |Some(x)| ...
 *
 *     |Result::Ok(value)| ...
 */
lambdaEnumPattern
    : qualifiedName
      LPAREN
      lambdaPatternElementList?
      RPAREN
    | qualifiedName
      LBRACE
      lambdaStructPatternFieldList?
      RBRACE
    ;


/**
 * Explicitly parenthesized parameter pattern.
 */
lambdaParenthesizedPattern
    : LPAREN
      lambdaParameterPattern
      RPAREN
    ;


/* ============================================================================
 * 5. PARAMETER TYPE ANNOTATIONS
 * ========================================================================== */

/**
 * Optional parameter type.
 *
 * Example:
 *
 *     |x: Int|
 *
 * Type ownership remains in the canonical type grammar.
 */
lambdaParameterTypeClause
    : COLON typeExpression
    ;


/**
 * Optional default parameter value.
 *
 * Default values remain expressions and are interpreted by semantic analysis.
 */
lambdaParameterDefaultClause
    : ASSIGN expression
    ;


/* ============================================================================
 * 6. RETURN TYPE
 * ========================================================================== */

/**
 * Optional lambda return type.
 *
 * Example:
 *
 *     |x| -> Int x + 1
 *
 *     |x| -> Int {
 *         return x + 1;
 *     }
 */
lambdaReturnTypeClause
    : ARROW typeExpression
    ;


/* ============================================================================
 * 7. LAMBDA BODY
 * ========================================================================== */

/**
 * A lambda may return:
 *
 *     expression
 *
 * or execute:
 *
 *     block
 *
 * Expression-bodied lambdas are essential for higher-order programming,
 * functional transformations, map/filter/reduce operations, compile-time
 * computation and mathematical expressions.
 */
lambdaBody
    : lambdaExpressionBody
    | lambdaBlockBody
    ;


/**
 * Expression-bodied lambda.
 *
 * This references the canonical expression rule supplied by the final parser
 * composition.
 */
lambdaExpressionBody
    : expression
    ;


/**
 * Block-bodied lambda.
 *
 * The block rule is owned by the canonical statement/block grammar.
 */
lambdaBlockBody
    : block
    ;


/* ============================================================================
 * 8. CANONICAL TYPE BRIDGE
 * ========================================================================== */

/**
 * Type syntax is NOT owned by this file.
 *
 * The final ANTLR composition must provide the canonical:
 *
 *     typeExpression
 *
 * rule from the Zamani type grammar.
 *
 * This bridge deliberately exists as a named contract rather than duplicating
 * the type grammar here.
 */


/* ============================================================================
 * 9. CANONICAL NAME BRIDGE
 * ========================================================================== */

/**
 * Identifier syntax is NOT owned by this file.
 *
 * The final parser composition provides:
 *
 *     identifier
 *
 * and:
 *
 *     qualifiedName
 *
 * from the canonical core/name grammar.
 */


/* ============================================================================
 * 10. CANONICAL EXPRESSION BRIDGE
 * ========================================================================== */

/**
 * `expression` is intentionally not redefined here.
 *
 * This prevents:
 *
 *     Lambdas -> Expressions -> Lambdas
 *
 * parser cycles.
 *
 * The grammar composition layer must make the canonical expression rule
 * available to lambdaBody.
 */


/* ============================================================================
 * 11. CANONICAL BLOCK BRIDGE
 * ========================================================================== */

/**
 * `block` belongs to the canonical statement grammar.
 *
 * Lambda expressions reuse it rather than creating a second block language.
 */


/* ============================================================================
 * 12. SEMANTIC CONTRACT
 * ========================================================================== */

/**
 * The parser MUST NOT decide:
 *
 *   - whether a lambda captures by reference or by value;
 *   - whether a capture is legal;
 *   - whether a parameter is Send/Sync-like;
 *   - whether a closure is thread-safe;
 *   - whether a closure is quantum-safe;
 *   - whether a closure can execute on a GPU;
 *   - whether a closure can execute on a quantum backend;
 *   - whether a closure is pure;
 *   - whether a closure has effects;
 *   - whether a closure can escape its defining scope;
 *   - whether a closure requires heap allocation;
 *   - whether a closure can be stack allocated;
 *   - whether a closure is inlined;
 *   - whether a closure becomes a function pointer;
 *   - whether a closure becomes a distributed task;
 *   - whether a closure becomes a hardware process.
 *
 * Those decisions belong downstream.
 *
 * A lambda is semantic computation.
 *
 * The compiler may lower it to:
 *
 *   - a function;
 *   - a closure object;
 *   - an IR region;
 *   - a task;
 *   - a GPU kernel;
 *   - an accelerator operation;
 *   - a distributed computation;
 *   - another target representation.
 *
 * None of those choices belong in this grammar.
 */


/* ============================================================================
 * 13. QUANTUM INTEGRATION
 * ========================================================================== */

/**
 * Lambda syntax is domain neutral.
 *
 * It may appear in semantic contexts such as:
 *
 *     map(...)
 *     reduce(...)
 *     filter(...)
 *     classical control around quantum operations
 *     parameter generation
 *     observable construction
 *     compile-time quantum transformation
 *
 * This grammar MUST NOT define:
 *
 *     qubit
 *     gate
 *     physical qubit
 *     topology
 *     calibration
 *     QEC
 *     ZQN
 *
 * Those concepts remain owned by their respective grammar/semantic layers.
 *
 * A lambda receiving or producing quantum values is still parsed exactly like
 * any other lambda.
 */


/* ============================================================================
 * 14. HARDWARE / HDL INTEGRATION
 * ========================================================================== */

/**
 * Lambda syntax is also independent of hardware description.
 *
 * A semantic compiler may lower a lambda to hardware-oriented constructs
 * where the target semantics permit it.
 *
 * This file must never contain:
 *
 *     clock counts
 *     pipeline depths
 *     FPGA LUT counts
 *     register counts
 *     ASIC cell counts
 *     GPU SM counts
 *     CPU core counts
 *     device IDs
 *
 * Such properties belong to target/resource descriptions.
 */


/* ============================================================================
 * 15. EFFECT / CAPABILITY INTEGRATION
 * ========================================================================== */

/**
 * Lambda effects are not encoded as arbitrary machine properties.
 *
 * The semantic layer may infer or validate:
 *
 *     IO
 *     network
 *     mutation
 *     allocation
 *     quantum
 *     hardware
 *     distributed
 *     security
 *     custom effects
 *
 * Explicit effect syntax, where supported, belongs to the effect grammar.
 *
 * This file only establishes the lambda's syntactic body and parameters.
 */


/* ============================================================================
 * 16. DETERMINISM
 * ========================================================================== */

/**
 * The grammar must produce deterministic parse structure for equivalent source
 * text.
 *
 * No semantic environment lookup is permitted during parsing.
 *
 * No:
 *
 *     hardware discovery
 *     runtime discovery
 *     network discovery
 *     random selection
 *     backend selection
 *
 * may influence lambda parsing.
 */


/* ============================================================================
 * 17. SCALABILITY
 * ========================================================================== */

/**
 * This grammar imposes no source-level maximum on:
 *
 *     lambda count
 *     parameter count
 *     nesting depth
 *     expression size
 *     block size
 *     pattern size
 *     source-file size
 *
 * Subject to:
 *
 *     available memory
 *     parser implementation limits
 *     compiler resource policy
 *     operating-system limits
 *     configured execution resources
 *
 * The language itself does not encode arbitrary machine-scale limits.
 */


/* ============================================================================
 * 18. SECURITY
 * ========================================================================== */

/**
 * Lambda parsing is pure syntax processing.
 *
 * This grammar MUST NOT:
 *
 *     access files
 *     access networks
 *     execute code
 *     load plugins
 *     inspect hardware
 *     invoke external processes
 *     access secrets
 *
 * Compile-time execution, where supported, is controlled by the separate
 * metaprogramming/compiler security policy.
 */


/* ============================================================================
 * 19. ERROR-RECOVERY CONTRACT
 * ========================================================================== */

/**
 * The parser implementation should recover from malformed lambda expressions
 * without silently changing the intended expression into another construct.
 *
 * Examples of errors that must remain diagnosable:
 *
 *     |x
 *     |x| 
 *     |x,| body
 *     |x:| body
 *     |x| -> body
 *     || 
 *
 * Error recovery belongs to the frontend/parser implementation rather than
 * this grammar's semantic layer.
 */


/* ============================================================================
 * 20. AST CONTRACT
 * ========================================================================== */

/**
 * The frontend AST produced from this grammar should preserve, at minimum:
 *
 *     Lambda
 *       modifiers
 *       parameters
 *         pattern
 *         mutability
 *         optional type
 *         optional default
 *       optional return type
 *       body
 *
 * Source spans MUST be retained by the AST layer.
 *
 * The grammar itself does not construct the AST.
 */


/* ============================================================================
 * 21. COMPILER CONTRACT
 * ========================================================================== */

/**
 * Downstream compiler stages may transform:
 *
 *     Lambda
 *
 * into:
 *
 *     Closure
 *     Function
 *     Region
 *     Task
 *     Kernel
 *     Distributed computation
 *
 * according to semantic analysis and target capabilities.
 *
 * The grammar does not select the lowering.
 */


/* ============================================================================
 * 22. RUNTIME CONTRACT
 * ========================================================================== */

/**
 * Runtime representation is explicitly outside this grammar.
 *
 * Possible runtime representations include:
 *
 *     direct function
 *     closure object
 *     environment + function
 *     task
 *     asynchronous computation
 *     distributed computation
 *
 * The same source lambda must retain the same language-level semantics even
 * when its runtime realization changes.
 */


/* ============================================================================
 * 23. TEST CONTRACT
 * ========================================================================== */

/**
 * Minimum positive examples:
 *
 *     |x| x
 *
 *     |x| x + 1
 *
 *     |x, y| x + y
 *
 *     |x: Int| x + 1
 *
 *     |x: Int, y: Int| x + y
 *
 *     || 42
 *
 *     |x| {
 *         return x + 1;
 *     }
 *
 *     |x, y, z,| x + y + z
 *
 *     |mut x| x
 *
 *     async |x| await(x)
 *
 *     move |x| x
 *
 *     |(x, y)| x + y
 *
 *     |[x, y]| x + y
 *
 *     |Point { x, y }| x + y
 *
 *     |Some(value)| value
 *
 *     |Result::Ok(value)| value
 *
 *     |x| map(values, |y| x + y)
 *
 *     |q| measure(q)
 *
 *     |x| {
 *         let y = x;
 *         y
 *     }
 *
 * Minimum negative examples:
 *
 *     |
 *
 *     |x
 *
 *     |x|
 *
 *     |x,|
 *
 *     |,x| x
 *
 *     |x:| x
 *
 *     |x| ->
 *
 *     |x| -> { }
 *
 *     |x,,y| x + y
 *
 *     |(x)| x
 *
 *     malformed destructuring
 *
 *     malformed type annotations
 *
 *     malformed return types
 *
 *     malformed block bodies
 *
 *     malformed nested lambdas
 *
 * Boundary tests:
 *
 *     zero-parameter lambda
 *     one-parameter lambda
 *     many-parameter lambda
 *     deeply nested lambdas
 *     lambda returning lambda
 *     lambda accepting lambda
 *     nested block lambdas
 *     large pattern structures
 *
 * Cross-domain tests:
 *
 *     classical + lambda
 *     quantum + lambda
 *     hybrid + lambda
 *     HDL + lambda
 *     distributed + lambda
 *     AI + lambda
 *     resource expressions + lambda
 */


/* ============================================================================
 * 24. COMPATIBILITY CONTRACT
 * ========================================================================== */

/**
 * Existing source syntax:
 *
 *     |parameterList| body
 *
 * remains the canonical compatibility form.
 *
 * Existing expression-level lambda syntax found in the repository must be
 * migrated to this rule rather than silently removed.
 *
 * The old lambda rule currently present in the monolithic expression grammar
 * should become an integration point to this grammar rather than remain a
 * competing definition.
 */


/* ============================================================================
 * 25. COMPLETION CRITERIA
 * ========================================================================== */

/**
 * This file is COMPLETE when:
 *
 * [ ] It has exactly one authoritative lambda-expression syntax.
 *
 * [ ] It contains no lexer rules.
 *
 * [ ] It does not duplicate general identifier syntax.
 *
 * [ ] It does not duplicate general type syntax.
 *
 * [ ] It does not duplicate general expression syntax.
 *
 * [ ] It does not duplicate general block syntax.
 *
 * [ ] It does not duplicate ordinary function parameter semantics.
 *
 * [ ] It does not encode machine limits.
 *
 * [ ] It does not select a hardware target.
 *
 * [ ] It does not define quantum IR.
 *
 * [ ] It does not define QEC.
 *
 * [ ] It does not define ZQN.
 *
 * [ ] It does not define scheduling.
 *
 * [ ] It does not define routing.
 *
 * [ ] It does not execute code.
 *
 * [ ] It does not perform I/O.
 *
 * [ ] It is deterministic.
 *
 * [ ] It has positive tests.
 *
 * [ ] It has negative tests.
 *
 * [ ] It has boundary tests.
 *
 * [ ] It has nested-lambda tests.
 *
 * [ ] It has cross-domain integration tests.
 *
 * [ ] Its ANTLR import/composition graph contains no cycle.
 *
 * [ ] Its token vocabulary is exclusively the canonical ZamaniLexer.
 *
 * [ ] Its AST mapping is defined before frontend implementation.
 *
 * [ ] Its semantic ownership is documented.
 *
 * [ ] Its downstream integration contracts are documented.
 *
 * [ ] Rust consumers compile under Rust 1.97 / 1.97.1.
 *
 * [ ] Rust consumers enforce forbidden unsafe code.
 *
 * [ ] No source-level scalability ceiling is introduced.
 */
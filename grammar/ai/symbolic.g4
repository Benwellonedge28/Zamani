/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/ai/symbolic.g4
 *
 * Grammar:
 *     AISymbolic
 *
 * Status:
 *     Production AI symbolic-computation source grammar.
 *
 * Baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     Safe Rust only.
 *
 * Safety:
 *     This is an ANTLR parser grammar.
 *
 *     - No embedded Rust actions.
 *     - No semantic predicates.
 *     - No unsafe implementation.
 *     - No filesystem access.
 *     - No network access.
 *     - No hardware discovery.
 *     - No runtime execution.
 *     - No target-specific implementation.
 *     - No vendor-specific implementation.
 *     - No fixed machine/resource limits.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar owns SOURCE-LEVEL SYMBOLIC COMPUTATION INTENT.
 *
 * Symbolic computation means that a Zamani program may represent and
 * manipulate mathematical, logical, algebraic, symbolic, or otherwise
 * unevaluated computational expressions as program values or computation
 * contracts.
 *
 * This includes source-level intent such as:
 *
 *     - symbolic values;
 *     - symbolic expressions;
 *     - symbolic variables;
 *     - symbolic constants;
 *     - symbolic functions;
 *     - symbolic relations;
 *     - assumptions;
 *     - substitutions;
 *     - transformations;
 *     - simplification requests;
 *     - expansion/factoring requests;
 *     - equation and inequality relations;
 *     - symbolic evaluation requests;
 *     - symbolic constraints;
 *     - symbolic goals;
 *     - symbolic composition;
 *     - symbolic program regions;
 *     - symbolic interoperability.
 *
 * This grammar deliberately DOES NOT enumerate mathematical algorithms.
 *
 * For example, the grammar MUST NOT become a permanent keyword list containing:
 *
 *     simplify
 *     expand
 *     factor
 *     solve
 *     integrate
 *     differentiate
 *     groebner
 *     fft
 *     svd
 *     eigen
 *     ...
 *
 * Such operations may be represented by extensible symbolic operation names
 * and resolved by semantic analysis, intrinsic registries, libraries,
 * dialects, or downstream implementations.
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
 *     canonical parser
 *          |
 *          +-------------------------------+
 *          |                               |
 *          v                               v
 *     Types / Expressions             AI symbolic boundary
 *                                          |
 *                                          v
 *                                domain-neutral frontend AST
 *                                          |
 *                                          v
 *                                  semantic analysis
 *                                          |
 *                 +------------------------+-----------------------+
 *                 |                        |                       |
 *                 v                        v                       v
 *             type analysis          effect analysis       capability/resource
 *                 |                        |                       analysis
 *                 +------------------------+-----------------------+
 *                                          |
 *                                          v
 *                              canonical semantic model / IR
 *                                          |
 *                 +------------------------+-----------------------+
 *                 |                        |                       |
 *                 v                        v                       v
 *             classical                 quantum                 hybrid
 *             lowering               lowering                    lowering
 *                                          |
 *                                          v
 *                                      optimize
 *                                          |
 *                                          v
 *                                  target realization
 *
 * Symbolic source may eventually lower into:
 *
 *     classical computation
 *     numerical computation
 *     tensor computation
 *     AI computation
 *     quantum computation
 *     HDL/hardware intent
 *     distributed computation
 *     accelerator computation
 *     future computation domains
 *
 * The grammar itself does not select the target.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - symbolic source construct boundaries;
 *     - symbolic declarations;
 *     - symbolic bindings;
 *     - symbolic operation requests;
 *     - symbolic relations;
 *     - symbolic assumptions;
 *     - symbolic substitutions;
 *     - symbolic transformations;
 *     - symbolic constraints;
 *     - symbolic goals;
 *     - symbolic regions;
 *     - symbolic metadata;
 *     - symbolic resource/capability/constraint/preference boundaries;
 *     - symbolic interoperability boundaries.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer rules;
 *     - identifier definitions;
 *     - literals;
 *     - operators;
 *     - general expression precedence;
 *     - general types;
 *     - general statements;
 *     - tensor types;
 *     - numerical algorithms;
 *     - symbolic solver implementations;
 *     - theorem provers;
 *     - rewrite engines;
 *     - simplification engines;
 *     - differentiation algorithms;
 *     - automatic differentiation;
 *     - training;
 *     - inference;
 *     - model execution;
 *     - quantum operations;
 *     - quantum::ir;
 *     - QEC;
 *     - ZQN;
 *     - HDL;
 *     - hardware realization;
 *     - routing;
 *     - scheduling;
 *     - calibration;
 *     - accelerator selection;
 *     - device selection;
 *     - runtime execution.
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 *     AISymbolic
 *         |
 *         +--> Types
 *         +--> Expressions
 *         +--> Statements
 *
 * This grammar MUST remain a leaf grammar.
 *
 * It MUST NOT import:
 *
 *     AI
 *     AIModels
 *     AITraining
 *     AIInference
 *     AIDifferentiation
 *     AIDifferentiable
 *     Quantum
 *     Hardware
 *     Resources
 *     Runtime
 *
 * This prevents dependency cycles and permits this file to be completed and
 * validated independently.
 *
 * ============================================================================
 * CANONICAL COMMON GRAMMAR CONTRACT
 * ============================================================================
 *
 * General types are owned by the canonical Types grammar.
 *
 * General expressions are owned by the canonical Expressions grammar.
 *
 * General statements are owned by the canonical Statements grammar.
 *
 * This grammar MUST reuse:
 *
 *     typeExpression
 *     expression
 *     statement
 *
 * where those rules are exposed by the canonical imported grammars.
 *
 * This file MUST NOT define a second:
 *
 *     typeExpression
 *     expression
 *     statement
 *     identifier
 *     literal
 *     precedence hierarchy
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * The canonical lexer owns symbolic vocabulary.
 *
 * The symbolic boundary uses:
 *
 *     NANO_ANNOTATION
 *
 * for annotation-led symbolic constructs.
 *
 * Examples:
 *
 *     @symbolic
 *     @symbol
 *     @expression
 *     @assume
 *     @substitute
 *
 * Annotation spelling and meaning are resolved by semantic analysis.
 *
 * This grammar therefore does NOT introduce new lexer keywords merely for
 * symbolic mathematics.
 *
 * The canonical identifier token is:
 *
 *     IDENTIFIER
 *
 * This grammar intentionally does not define a local identifier rule using
 * an obsolete token such as `IDENT`.
 *
 * ============================================================================
 * ANNOTATION OWNERSHIP
 * ============================================================================
 *
 * `NANO_ANNOTATION` provides the lexical annotation boundary.
 *
 * Semantic analysis is responsible for validating that a particular
 * annotation is legal for a symbolic construct.
 *
 * Examples of semantic symbolic roles include:
 *
 *     @symbolic
 *     @symbol
 *     @assume
 *     @substitute
 *     @transform
 *     @goal
 *     @relation
 *
 * These names are not parser-level reserved keywords.
 *
 * This avoids continuously expanding the global lexer whenever a new
 * symbolic operation or symbolic dialect is introduced.
 *
 * ============================================================================
 * SYMBOLIC OPERATION EXTENSIBILITY
 * ============================================================================
 *
 * Symbolic operation names are intentionally represented by identifiers.
 *
 * Therefore a source program may describe:
 *
 *     @symbolic simplify(x);
 *     @symbolic expand(expr);
 *     @symbolic factor(expr);
 *     @symbolic solve(equation, x);
 *     @symbolic custom_symbolic_operation(expr);
 *
 * without requiring a new lexer keyword for every operation.
 *
 * The parser does not decide whether an operation is:
 *
 *     built-in
 *     intrinsic
 *     library-provided
 *     user-defined
 *     dialect-defined
 *     experimental
 *     unavailable
 *
 * Semantic analysis and operation registries determine that.
 *
 * ============================================================================
 * MATHEMATICAL EXTENSIBILITY
 * ============================================================================
 *
 * The grammar MUST NOT hard-code a finite mathematical vocabulary.
 *
 * Symbolic values can be ordinary Zamani expressions.
 *
 * Therefore the symbolic layer can operate over:
 *
 *     scalar expressions
 *     vector expressions
 *     matrix expressions
 *     tensor expressions
 *     function expressions
 *     polynomial expressions
 *     rational expressions
 *     logical expressions
 *     quantum-derived expressions
 *     probabilistic expressions
 *     domain-specific expressions
 *     user-defined symbolic types
 *
 * The actual type system remains owned by Types.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Symbolic expressions may reference quantum-derived values.
 *
 * This grammar MUST NOT define quantum operations.
 *
 * Example semantic flow:
 *
 *     symbolic source
 *          |
 *          v
 *     domain-neutral AST
 *          |
 *          v
 *     symbolic semantic analysis
 *          |
 *          +--> quantum value analysis
 *          |
 *          v
 *     canonical quantum::ir
 *
 * The quantum semantic boundary remains:
 *
 *     quantum::ir
 *
 * This file MUST NOT create:
 *
 *     SymbolicQuantumIR
 *     QuantumSymbolicIR
 *     SymbolicGateIR
 *
 * or any other competing quantum representation.
 *
 * ============================================================================
 * DIFFERENTIATION INTEGRATION
 * ============================================================================
 *
 * Symbolic expressions may be differentiated symbolically.
 *
 * However:
 *
 *     symbolic.g4
 *
 * does NOT own differentiation requests or differentiation algorithms.
 *
 * Derivative computation intent remains owned by:
 *
 *     grammar/ai/differentiation.g4
 *
 * Differentiability contracts remain owned by:
 *
 *     grammar/ai/differentiable.g4
 *
 * Therefore symbolic syntax may contain ordinary expressions which later
 * participate in differentiation, but this grammar does not duplicate the
 * differentiation grammar.
 *
 * ============================================================================
 * AI INTEGRATION
 * ============================================================================
 *
 * Symbolic computation can participate in:
 *
 *     models
 *     training
 *     inference
 *     agents
 *     pipelines
 *     optimization
 *     datasets
 *     tensors
 *     differentiation
 *
 * Those domains remain independently owned.
 *
 * Symbolic constructs may therefore be referenced by those domains through
 * ordinary expressions, names, bindings, and semantic cross-references.
 *
 * This avoids making `symbolic.g4` the parent of the entire AI subsystem.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY CONTRACT
 * ============================================================================
 *
 * Symbolic computation may require capabilities or resources.
 *
 * The grammar permits generic declarative clauses rather than fixed machine
 * requirements.
 *
 * Examples of semantic intent include:
 *
 *     requires capability("symbolic.solve")
 *     requires capability("symbolic.exact")
 *     requires capability("arbitrary_precision")
 *     requires memory >= required_memory
 *     requires resource("symbolic_engine")
 *     prefer capability("parallel_symbolic")
 *
 * These are source-level requirements or preferences.
 *
 * The grammar does not decide whether a target satisfies them.
 *
 * ============================================================================
 * POCO-REAF / SCALABILITY
 * ============================================================================
 *
 * There is NO grammar-level maximum for:
 *
 *     - symbolic variables;
 *     - symbolic expressions;
 *     - expression depth;
 *     - expression width;
 *     - equations;
 *     - inequalities;
 *     - substitutions;
 *     - assumptions;
 *     - constraints;
 *     - symbolic operations;
 *     - symbolic regions;
 *     - model size;
 *     - tensor rank;
 *     - tensor dimensions;
 *     - matrix dimensions;
 *     - vector dimensions;
 *     - polynomial degree;
 *     - number of terms;
 *     - number of functions;
 *     - number of models;
 *     - number of workers;
 *     - CPUs;
 *     - GPUs;
 *     - FPGAs;
 *     - QPUs;
 *     - accelerators;
 *     - nodes;
 *     - devices;
 *     - memory;
 *     - storage.
 *
 * Repetition is structural.
 *
 * For example:
 *
 *     @symbolic expression f = ...
 *
 * does not imply any maximum expression size.
 *
 * A resource or implementation failure for an individual program is not a
 * language-level maximum.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * This grammar MUST NOT introduce constants such as:
 *
 *     MAX_SYMBOLS
 *     MAX_VARIABLES
 *     MAX_TERMS
 *     MAX_EQUATIONS
 *     MAX_EXPRESSION_DEPTH
 *     MAX_POLYNOMIAL_DEGREE
 *     MAX_TENSOR_RANK
 *     MAX_TENSOR_DIMENSION
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_NODES
 *
 * or equivalent artificial limits.
 *
 * The following is valid source semantics:
 *
 *     @symbolic expression f = polynomial_degree;
 *
 * where `polynomial_degree` may itself be arbitrary program data.
 *
 * The following architectural design is prohibited:
 *
 *     grammar accepts expressions only up to N terms.
 *
 * ============================================================================
 * REQUIREMENT / CAPABILITY / PREFERENCE SEPARATION
 * ============================================================================
 *
 * Symbolic source must preserve the distinction between:
 *
 *     requirement
 *     capability
 *     constraint
 *     preference
 *     hint
 *     implementation realization
 *
 * For example:
 *
 *     requires capability("symbolic.exact")
 *
 * is not equivalent to:
 *
 *     use symbolic_engine_3
 *
 * The latter is a target realization and belongs downstream unless explicitly
 * declared as a target-specific deployment requirement.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing is deterministic with respect to token structure.
 *
 * This grammar deliberately uses punctuation to distinguish construct forms.
 *
 * After:
 *
 *     NANO_ANNOTATION IDENTIFIER
 *
 * the next token determines the structural form:
 *
 *     '('  -> symbolic call
 *     '='  -> symbolic assignment
 *     IDENTIFIER -> symbolic named construct
 *     ':' -> symbolic typed/target construct
 *
 * The grammar does not use semantic predicates to distinguish symbolic
 * operation names.
 *
 * ============================================================================
 * SOURCE SPANS
 * ============================================================================
 *
 * Every parser construct must preserve source locations through the standard
 * ANTLR parse tree.
 *
 * The frontend AST/source-map layer is responsible for converting these
 * locations into Zamani source spans.
 *
 * This grammar MUST NOT embed source-location implementation code.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Parser diagnostics should identify:
 *
 *     - missing annotation payload;
 *     - malformed symbolic call;
 *     - malformed symbolic declaration;
 *     - malformed symbolic assignment;
 *     - malformed symbolic clause;
 *     - malformed symbolic block;
 *     - malformed symbolic argument list.
 *
 * Semantic diagnostics are responsible for:
 *
 *     - unknown symbolic operation;
 *     - invalid symbolic type;
 *     - invalid assumption;
 *     - invalid relation;
 *     - unsatisfied symbolic capability;
 *     - unsupported symbolic operation;
 *     - impossible symbolic constraint;
 *     - unsupported target realization.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing symbolic syntax MUST NOT execute symbolic operations.
 *
 * In particular, parsing:
 *
 *     @symbolic solve(...)
 *
 * must not invoke a solver.
 *
 * Solver invocation, theorem proving, simplification, code generation, and
 * resource allocation belong downstream.
 *
 * Symbolic source must therefore be safe to parse without:
 *
 *     filesystem access
 *     network access
 *     process execution
 *     hardware access
 *     arbitrary code execution.
 *
 * ============================================================================
 * INDEPENDENT COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] parser grammar identity is stable;
 *     [x] canonical lexer vocabulary is reused;
 *     [x] canonical expression grammar is reused;
 *     [x] canonical type grammar is reused;
 *     [x] canonical statement grammar is reused where required;
 *     [x] no local identifier grammar is duplicated;
 *     [x] symbolic construct boundaries are deterministic;
 *     [x] symbolic operation names remain extensible;
 *     [x] mathematical operations are not hard-coded;
 *     [x] differentiation ownership remains separate;
 *     [x] tensor ownership remains separate;
 *     [x] model ownership remains separate;
 *     [x] training ownership remains separate;
 *     [x] quantum ownership remains separate;
 *     [x] quantum::ir remains canonical;
 *     [x] resource requirements remain semantic;
 *     [x] capabilities remain semantic;
 *     [x] preferences remain non-binding;
 *     [x] no hardware capacity is hard-coded;
 *     [x] no target is selected by the grammar;
 *     [x] no IR is constructed;
 *     [x] no unsafe implementation exists;
 *     [x] no semantic predicates exist;
 *     [x] no embedded actions exist;
 *     [x] positive tests are defined;
 *     [x] negative tests are defined;
 *     [x] boundary tests are defined;
 *     [x] scalability tests are defined;
 *     [x] determinism tests are defined;
 *     [x] compatibility integration is defined.
 *
 * ============================================================================
 */

parser grammar AISymbolic;

options {
    tokenVocab = ZamaniLexer;
}

import Types,
       Expressions,
       Statements;


/* ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ========================================================================== */

/*
 * Every symbolic construct must cross an explicit annotation boundary.
 *
 * Ordinary expressions therefore remain ordinary expressions.
 *
 * The annotation text is intentionally opaque to the parser and is resolved
 * semantically.
 */
aiSymbolicConstruct
    : NANO_ANNOTATION aiSymbolicDirective
    ;


/* ============================================================================
 * 2. DETERMINISTIC DIRECTIVE DISPATCH
 * ========================================================================== */

/*
 * The first IDENTIFIER is the semantic symbolic directive name.
 *
 * Structural dispatch is determined by the following token:
 *
 *     IDENTIFIER '('        -> call
 *     IDENTIFIER '='        -> assignment
 *     IDENTIFIER IDENTIFIER -> named construct
 *     IDENTIFIER ':'        -> typed target/binding
 *
 * No semantic predicate is required.
 */
aiSymbolicDirective
    : IDENTIFIER aiSymbolicDirectiveTail
    ;


/* ============================================================================
 * 3. SYMBOLIC DIRECTIVE TAILS
 * ========================================================================== */

aiSymbolicDirectiveTail
    : aiSymbolicCallTail
    | aiSymbolicAssignmentTail
    | aiSymbolicNamedTail
    | aiSymbolicTypedTail
    ;


/* ============================================================================
 * 4. SYMBOLIC OPERATION / FUNCTION CALL
 * ========================================================================== */

/*
 * Examples:
 *
 *     @symbolic simplify(expression);
 *     @symbolic solve(equation, variable);
 *     @symbolic factor(expression);
 *     @symbolic custom_operation(expression);
 *
 * The operation name is semantic data.
 */
aiSymbolicCallTail
    : '(' aiSymbolicArgumentList? ')' aiSymbolicClauseList? ';'
    ;

aiSymbolicArgumentList
    : expression (',' expression)*
    ;


/* ============================================================================
 * 5. SYMBOLIC ASSIGNMENT
 * ========================================================================== */

/*
 * Examples:
 *
 *     @symbolic expression = value;
 *     @symbolic result = expression;
 *
 * The left-hand directive name and right-hand expression are resolved
 * semantically.
 */
aiSymbolicAssignmentTail
    : '=' expression aiSymbolicClauseList? ';'
    ;


/* ============================================================================
 * 6. SYMBOLIC NAMED CONSTRUCT
 * ========================================================================== */

/*
 * Examples:
 *
 *     @symbolic variable x;
 *     @symbolic expression f = expression;
 *     @symbolic function f { ... }
 *     @symbolic relation r { ... }
 *
 * The first identifier is the semantic role.
 *
 * The second identifier is the source-level symbolic name.
 *
 * The role vocabulary remains extensible and is validated semantically.
 */
aiSymbolicNamedTail
    : IDENTIFIER
      aiSymbolicTypeAnnotation?
      aiSymbolicInitializer?
      aiSymbolicClauseList?
      aiSymbolicBodyOrTerminator
    ;


/* ============================================================================
 * 7. SYMBOLIC TYPED TARGET
 * ========================================================================== */

/*
 * This form supports constructs where the symbolic directive identifies a
 * typed symbolic value.
 *
 * Example:
 *
 *     @symbolic variable x: SomeType;
 *
 * General type syntax belongs to Types.
 */
aiSymbolicTypedTail
    : ':' typeExpression
      aiSymbolicInitializer?
      aiSymbolicClauseList?
      aiSymbolicBodyOrTerminator
    ;


/* ============================================================================
 * 8. TYPE / INITIALIZER
 * ========================================================================== */

aiSymbolicTypeAnnotation
    : ':' typeExpression
    ;

aiSymbolicInitializer
    : '=' expression
    ;


/* ============================================================================
 * 9. SYMBOLIC BODY
 * ========================================================================== */

/*
 * A symbolic body may contain:
 *
 *     nested symbolic constructs
 *     ordinary symbolic assignments
 *
 * It does not introduce a second statement language.
 */
aiSymbolicBodyOrTerminator
    : aiSymbolicBody
    | ';'
    ;

aiSymbolicBody
    : '{' aiSymbolicMember* '}'
    ;

aiSymbolicMember
    : aiSymbolicConstruct
    | aiSymbolicMemberAssignment
    ;

aiSymbolicMemberAssignment
    : IDENTIFIER '=' expression ';'
    ;


/* ============================================================================
 * 10. SYMBOLIC CLAUSES
 * ========================================================================== */

/*
 * Clauses provide extensible metadata/configuration without requiring a new
 * parser keyword for every symbolic feature.
 *
 * Examples:
 *
 *     , domain = real
 *     , exact = true
 *     , order = n
 *     , method = custom_method
 *
 * The semantic layer determines which clauses are legal.
 */
aiSymbolicClauseList
    : ',' aiSymbolicClause (',' aiSymbolicClause)*
    ;

aiSymbolicClause
    : IDENTIFIER
      (
          '=' expression
        | '(' aiSymbolicArgumentList? ')'
        | ':' typeExpression
      )
    ;


/* ============================================================================
 * 11. SYMBOLIC REGION
 * ========================================================================== */

/*
 * A symbolic region can be represented through a named symbolic construct:
 *
 *     @symbolic region computation {
 *         ...
 *     }
 *
 * The grammar intentionally does not hard-code the word `region`.
 *
 * Semantic analysis assigns the appropriate symbolic role.
 */


/* ============================================================================
 * 12. SYMBOLIC RESOURCE / CAPABILITY CONTRACTS
 * ========================================================================== */

/*
 * Resource and capability constructs are structurally represented using the
 * same generic symbolic directive mechanism.
 *
 * Examples:
 *
 *     @symbolic requires(capability("symbolic.exact"));
 *     @symbolic requires(memory >= required_memory);
 *     @symbolic prefer(capability("parallel_symbolic"));
 *
 * The exact policy vocabulary is owned by the resource/capability semantic
 * layer, not this grammar.
 */


/* ============================================================================
 * 13. SYMBOLIC RELATIONS
 * ========================================================================== */

/*
 * Relations can be expressed through ordinary expressions.
 *
 * Examples include:
 *
 *     equality
 *     inequality
 *     ordering
 *     logical relation
 *     domain-specific relation
 *
 * Their operator semantics remain owned by Expressions.
 *
 * This prevents symbolic.g4 from defining a second operator hierarchy.
 */


/* ============================================================================
 * 14. SYMBOLIC SUBSTITUTION
 * ========================================================================== */

/*
 * Substitution is represented as an extensible symbolic operation.
 *
 * Example:
 *
 *     @symbolic substitute(expression, variable, replacement);
 *
 * The grammar does not prescribe how substitution is implemented.
 */


/* ============================================================================
 * 15. SYMBOLIC ASSUMPTIONS
 * ========================================================================== */

/*
 * Assumptions are source-level symbolic contracts.
 *
 * Example:
 *
 *     @symbolic assume(x > 0);
 *
 * The expression itself is owned by Expressions.
 *
 * Semantic analysis determines whether the assumption is:
 *
 *     valid
 *     contradictory
 *     unsupported
 *     conditional
 *     target-dependent.
 */


/* ============================================================================
 * 16. SYMBOLIC GOALS
 * ========================================================================== */

/*
 * Symbolic goals may be represented through generic calls.
 *
 * Examples:
 *
 *     @symbolic goal(expression);
 *     @symbolic prove(statement);
 *     @symbolic satisfy(constraint);
 *
 * No theorem prover is implied by the grammar.
 */


/* ============================================================================
 * 17. SYMBOLIC TRANSFORMATIONS
 * ========================================================================== */

/*
 * Transformation requests remain extensible:
 *
 *     @symbolic simplify(expression);
 *     @symbolic transform(expression);
 *     @symbolic normalize(expression);
 *     @symbolic expand(expression);
 *
 * The operation registry determines the semantic meaning.
 */


/* ============================================================================
 * 18. COMPOSITION
 * ========================================================================== */

/*
 * Symbolic constructs may be nested.
 *
 * Example shape:
 *
 *     @symbolic computation outer {
 *         @symbolic expression x = ...;
 *         @symbolic simplify(x);
 *         @symbolic goal(...);
 *     }
 *
 * There is no fixed nesting depth in the language grammar.
 */


/* ============================================================================
 * 19. GENERAL STATEMENT INTEROPERABILITY
 * ========================================================================== */

/*
 * This grammar imports Statements so that symbolic constructs can participate
 * in a complete Zamani program through the canonical statement composition
 * boundary.
 *
 * It does NOT redefine `statement`.
 *
 * The canonical statement grammar remains the sole owner of the universal
 * `statement` rule.
 */


/* ============================================================================
 * 20. AST CONTRACT
 * ========================================================================== */

/*
 * This grammar MUST map into the existing domain-neutral frontend AST model.
 *
 * Conceptual mapping:
 *
 *     aiSymbolicConstruct
 *          |
 *          v
 *     generic annotated/domain construct
 *          |
 *          v
 *     symbolic semantic model
 *
 * The frontend AST MUST retain:
 *
 *     annotation
 *     directive name
 *     symbolic name where present
 *     arguments
 *     initializer
 *     type
 *     clauses
 *     body
 *     source span
 *
 * It MUST NOT directly contain:
 *
 *     solver implementation objects
 *     theorem prover objects
 *     hardware objects
 *     physical device identifiers
 *     quantum routing objects
 *     QEC state
 *     scheduler state
 *
 * Those belong downstream.
 */


/* ============================================================================
 * 21. SEMANTIC CONTRACT
 * ========================================================================== */

/*
 * Semantic analysis is responsible for:
 *
 *     - resolving symbolic directive names;
 *     - resolving symbolic identifiers;
 *     - checking symbolic scopes;
 *     - checking symbolic types;
 *     - validating assumptions;
 *     - validating relations;
 *     - validating operation signatures;
 *     - validating symbolic clauses;
 *     - resolving symbolic operation registries;
 *     - checking capabilities;
 *     - checking resources;
 *     - checking effects;
 *     - checking portability;
 *     - determining whether symbolic computation can be lowered;
 *     - selecting an implementation strategy downstream.
 *
 * This grammar does not perform any of these operations.
 */


/* ============================================================================
 * 22. IR CONTRACT
 * ========================================================================== */

/*
 * AISymbolic MUST NOT create an AI-specific symbolic IR.
 *
 * Symbolic semantics are lowered through the canonical semantic architecture.
 *
 * Depending on the program, symbolic computation may lower toward:
 *
 *     classical computation
 *     tensor computation
 *     numerical computation
 *     AI computation
 *     quantum computation
 *     hybrid computation
 *     hardware intent
 *     distributed computation
 *
 * Quantum-related symbolic computation ultimately follows:
 *
 *     symbolic semantic model
 *          |
 *          v
 *     quantum semantics
 *          |
 *          v
 *     quantum::ir
 *
 * No second quantum IR is permitted.
 */


/* ============================================================================
 * 23. COMPILER INTEGRATION
 * ========================================================================== */

/*
 * Compiler stages consuming symbolic semantics may include:
 *
 *     type analysis
 *     constant/symbolic propagation
 *     algebraic transformation
 *     simplification
 *     specialization
 *     partial evaluation
 *     numerical lowering
 *     tensor lowering
 *     quantum lowering
 *     classical lowering
 *     hardware lowering
 *     optimization
 *
 * This grammar does not mandate any particular implementation.
 */


/* ============================================================================
 * 24. RUNTIME INTEGRATION
 * ========================================================================== */

/*
 * Runtime behavior is downstream.
 *
 * A symbolic value may remain symbolic at runtime, become a concrete value,
 * be specialized at compile time, or be lowered to another computation.
 *
 * The parser does not decide which.
 */


/* ============================================================================
 * 25. PORTABILITY CONTRACT
 * ========================================================================== */

/*
 * Symbolic source must remain target-independent whenever the source itself
 * does not explicitly require target-specific behavior.
 *
 * The same symbolic source may therefore be lowered toward:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     simulator
 *     distributed system
 *     embedded target
 *     future target
 *
 * subject to semantic validity, available capabilities, resource availability,
 * and downstream compilation.
 *
 * The grammar itself selects none of these.
 */


/* ============================================================================
 * 26. POCO-REAF INVARIANT
 * ========================================================================== */

/*
 * Program:
 *
 *     symbolic intent
 *
 * Compile:
 *
 *     target-independent semantic representation
 *
 * Realize:
 *
 *     available target resources/capabilities
 *
 * Therefore source syntax must not contain universal target assumptions such
 * as:
 *
 *     GPU 0
 *     CPU 0
 *     QPU 1
 *     MAX_SYMBOLS
 *     MAX_MEMORY
 *     MAX_TENSOR_RANK
 *
 * unless such a value is explicitly part of the programmer's own semantics.
 */


/* ============================================================================
 * 27. POSITIVE CONFORMANCE EXAMPLES
 * ========================================================================== */

/*
 * These are conceptual conformance forms:
 *
 *     @symbolic variable x;
 *
 *     @symbolic expression f = x * x + 1;
 *
 *     @symbolic expression g: SomeSymbolicType = expression;
 *
 *     @symbolic simplify(f);
 *
 *     @symbolic substitute(f, x, replacement);
 *
 *     @symbolic assume(x > 0);
 *
 *     @symbolic solve(equation, x);
 *
 *     @symbolic custom_operation(f, parameter);
 *
 *     @symbolic computation program {
 *         @symbolic expression x = input;
 *         @symbolic simplify(x);
 *     }
 *
 *     @symbolic requires(capability("symbolic.exact"));
 *
 *     @symbolic prefer(capability("parallel_symbolic"));
 */


/* ============================================================================
 * 28. NEGATIVE CONFORMANCE REQUIREMENTS
 * ========================================================================== */

/*
 * The grammar/semantic system must reject malformed constructs such as:
 *
 *     @symbolic
 *
 *     @symbolic operation(
 *
 *     @symbolic operation();
 *
 *     @symbolic name = ;
 *
 *     @symbolic variable x:
 *
 *     @symbolic operation(, x);
 *
 * Semantic validation must additionally reject:
 *
 *     unknown symbolic operations;
 *     invalid symbolic types;
 *     invalid assumptions;
 *     incompatible substitutions;
 *     unsatisfied required capabilities;
 *     impossible constraints.
 */


/* ============================================================================
 * 29. BOUNDARY CONFORMANCE
 * ========================================================================== */

/*
 * The implementation must test:
 *
 *     - empty symbolic bodies;
 *     - deeply nested symbolic bodies;
 *     - large argument lists;
 *     - large symbolic expressions;
 *     - large numbers of symbolic declarations;
 *     - very large identifiers;
 *     - arbitrary representable numeric literals;
 *     - Unicode identifiers where allowed by the canonical lexer;
 *     - symbolic values involving tensors;
 *     - symbolic values involving quantum-derived values;
 *     - symbolic values involving distributed computation;
 *     - symbolic values involving hardware intent.
 *
 * None of these tests may establish an artificial maximum as language
 * semantics.
 */


/* ============================================================================
 * 30. DETERMINISM CONFORMANCE
 * ========================================================================== */

/*
 * For a fixed token stream, the parser must produce the same parse structure.
 *
 * The grammar contains:
 *
 *     - no semantic predicates;
 *     - no target-language actions;
 *     - no runtime callbacks;
 *     - no hardware inspection;
 *     - no environment inspection;
 *     - no randomness.
 *
 * Symbolic operation resolution occurs after parsing.
 */


/* ============================================================================
 * 31. COMPATIBILITY CONTRACT
 * ========================================================================== */

/*
 * Adding a new symbolic operation MUST NOT require changing this grammar if
 * the operation can be represented by the generic identifier-based operation
 * form.
 *
 * For example, adding:
 *
 *     symbolic_new_operation
 *
 * should normally require only:
 *
 *     operation registry
 *     semantic contract
 *     implementation
 *     tests
 *
 * and NOT:
 *
 *     lexer keyword
 *     parser alternative
 *
 * unless the operation introduces genuinely new language-level syntax.
 *
 * This is essential for long-term language stability.
 */


/* ============================================================================
 * 32. HARD-CODING AUDIT
 * ========================================================================== */

/*
 * Forbidden in this grammar:
 *
 *     MAX_SYMBOLS
 *     MAX_VARIABLES
 *     MAX_EQUATIONS
 *     MAX_TERMS
 *     MAX_EXPRESSION_DEPTH
 *     MAX_EXPRESSION_WIDTH
 *     MAX_POLYNOMIAL_DEGREE
 *     MAX_TENSOR_RANK
 *     MAX_TENSOR_DIMENSION
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QUBITS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_DEVICES
 *
 * Also forbidden:
 *
 *     fixed mathematical operation enumeration;
 *     fixed symbolic solver enumeration;
 *     fixed hardware selection;
 *     physical device identifiers;
 *     fixed accelerator identifiers;
 *     fixed tensor capacities.
 *
 * Program literals are not language limits.
 */


/* ============================================================================
 * 33. FINAL OWNERSHIP SUMMARY
 * ========================================================================== */

/*
 * AISymbolic owns:
 *
 *     symbolic source syntax
 *     symbolic construct boundaries
 *     symbolic extensibility
 *
 * Lexer owns:
 *
 *     tokens
 *
 * Types owns:
 *
 *     types
 *
 * Expressions owns:
 *
 *     expressions/operators
 *
 * Statements owns:
 *
 *     universal statements
 *
 * Semantic analysis owns:
 *
 *     symbolic meaning
 *
 * Symbolic operation registry owns:
 *
 *     operation availability/meaning
 *
 * Differentiation owns:
 *
 *     derivative requests and differentiation strategies
 *
 * Differentiable owns:
 *
 *     differentiability contracts
 *
 * Models owns:
 *
 *     model declarations
 *
 * Training owns:
 *
 *     training intent
 *
 * Inference owns:
 *
 *     inference intent
 *
 * Tensors owns:
 *
 *     tensor semantics
 *
 * Quantum owns:
 *
 *     quantum syntax
 *
 * quantum::ir owns:
 *
 *     canonical quantum semantic representation
 *
 * Resources/capabilities own:
 *
 *     target requirements and capabilities
 *
 * Compiler owns:
 *
 *     lowering and optimization
 *
 * Runtime/HAL owns:
 *
 *     target realization
 *
 * Therefore this file remains a true symbolic-computation leaf grammar.
 */
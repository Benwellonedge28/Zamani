/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/metaprogramming/reflection.g4
 *
 * Grammar:
 *     Reflection
 *
 * Status:
 *     Production parser-grammar component
 *
 * Baseline:
 *     ANTLR4
 *     Rust 2021
 *     Rust 1.97 / Rust 1.97.1
 *     Safe Rust only
 *     No unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL SYNTAX of Zamani reflection.
 *
 * Reflection provides a language-defined way to request structured information
 * about a source-level entity, type, declaration, operation, function, module,
 * metadata-bearing object, or other semantically reflectable entity.
 *
 * Reflection is intentionally a VIEW over the canonical Zamani semantic model.
 *
 * It does not define:
 *
 *     - a second type system;
 *     - a second AST;
 *     - a second IR;
 *     - a quantum IR;
 *     - a hardware model;
 *     - a runtime object model;
 *     - a compiler API;
 *     - a host introspection API.
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
 *     domain-neutral frontend AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> name resolution
 *          +--> type analysis
 *          +--> visibility analysis
 *          +--> effect analysis
 *          +--> capability analysis
 *          +--> reflection validation
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          +--> classical semantics
 *          +--> quantum::ir
 *          +--> HDL / hardware semantics
 *          +--> distributed semantics
 *          +--> AI / data / networking semantics
 *          |
 *          v
 *     optimization / lowering / routing / scheduling / resilience
 *          |
 *          v
 *     HAL / target realization
 *
 * Reflection is therefore a FRONTEND LANGUAGE FACILITY.
 *
 * It is not a backend facility.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - reflection expression syntax;
 *     - reflection subject syntax;
 *     - explicit type-reflection syntax;
 *     - reflection projection syntax;
 *     - reflection selector syntax;
 *     - reflection query chaining;
 *     - the stable `reflectionExpressionCore` integration boundary.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexical identifiers;
 *     - qualified-name syntax;
 *     - ordinary expressions;
 *     - type syntax;
 *     - declarations;
 *     - statements;
 *     - attributes;
 *     - generic syntax;
 *     - compile-time execution;
 *     - macro expansion;
 *     - source generation;
 *     - specialization;
 *     - semantic reflection;
 *     - type checking;
 *     - name resolution;
 *     - capability discovery;
 *     - resource discovery;
 *     - hardware discovery;
 *     - target selection;
 *     - quantum compilation;
 *     - quantum::ir;
 *     - classical IR;
 *     - HDL IR;
 *     - routing;
 *     - scheduling;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - runtime execution.
 *
 * ============================================================================
 * CRITICAL DESIGN DECISION
 * ============================================================================
 *
 * Reflection MUST NOT embed the complete `expression` grammar as its subject.
 *
 * The reason is architectural.
 *
 * If:
 *
 *     expression
 *         -> primaryExpression
 *             -> reflectionExpression
 *                 -> expression
 *
 * then reflection introduces an indirect recursive dependency into the
 * canonical expression hierarchy.
 *
 * That creates avoidable ambiguity and makes modular ANTLR composition fragile.
 *
 * Therefore this grammar deliberately uses a NON-RECURSIVE source subject:
 *
 *     qualifiedName
 *
 * for value/declaration reflection, and:
 *
 *     typeExpression
 *
 * for explicit type reflection.
 *
 * Examples:
 *
 *     reflect(value)
 *     reflect(myFunction)
 *     reflect(quantum::operation)
 *     reflect(hardware::capability)
 *
 *     reflect type(MyType)
 *     reflect type(Qubit)
 *     reflect type(Tensor<float>)
 *
 * Arbitrary computed expressions are NOT silently accepted as reflection
 * subjects.
 *
 * If future Zamani semantics require reflection of an arbitrary computed
 * expression, that feature must be introduced through an explicit quotation,
 * compile-time-value, or other canonical metaprogramming facility rather than
 * recursively embedding `expression` here.
 *
 * ============================================================================
 * REFLECTION IS OPEN-WORLD
 * ============================================================================
 *
 * Reflection selectors are identifiers.
 *
 * The grammar does NOT enumerate:
 *
 *     name
 *     kind
 *     type
 *     members
 *     fields
 *     methods
 *     parameters
 *     returnType
 *     attributes
 *     generics
 *     effects
 *     capabilities
 *     requirements
 *     declaration
 *     source
 *     relationships
 *
 * Those are semantic reflection properties.
 *
 * This permits future language domains to expose reflection metadata without
 * changing the lexical vocabulary for every new property.
 *
 * For example, the same syntax can semantically expose information about:
 *
 *     classical declarations
 *     quantum operations
 *     logical qubits
 *     circuits
 *     measurements
 *     HDL modules
 *     ports
 *     signals
 *     hardware capabilities
 *     distributed services
 *     AI models
 *     data schemas
 *     networking abstractions
 *     security metadata
 *     future computing domains
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * The canonical parser consumes:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * `REFLECT` is the only reflection-specific reserved lexical word required by
 * this grammar.
 *
 * `REFLECT` MUST be owned by:
 *
 *     grammar/lexer/keywords.g4
 *
 * and composed into:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This file MUST NOT define lexer rules.
 *
 * Reflection selectors remain ordinary identifiers.
 *
 * Therefore future selectors do not require permanent keyword additions.
 *
 * ============================================================================
 * NAME CONTRACT
 * ============================================================================
 *
 * Name syntax is owned by:
 *
 *     grammar/core/names.g4
 *
 * This grammar imports and consumes:
 *
 *     qualifiedName
 *
 * It does not redefine:
 *
 *     identifier
 *     nameSegment
 *     qualifiedName
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * Type syntax is owned by:
 *
 *     grammar/types/types.g4
 *
 * This grammar consumes:
 *
 *     typeExpression
 *
 * It does not redefine:
 *
 *     typeCore
 *     namedType
 *     genericType
 *     quantumType
 *     dependentType
 *     or any other type form.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Reflection must preserve:
 *
 *     Program Once
 *          |
 *          v
 *     Compile Once
 *          |
 *          v
 *     Run Everywhere
 *          |
 *          v
 *     Run Anywhere
 *          |
 *          v
 *     Run Forever
 *
 * Reflection syntax therefore MUST NOT encode:
 *
 *     CPU counts
 *     core counts
 *     thread counts
 *     GPU counts
 *     FPGA counts
 *     ASIC counts
 *     accelerator counts
 *     QPU counts
 *     qubit limits
 *     memory capacities
 *     register widths
 *     tensor limits
 *     node counts
 *     topology sizes
 *     device identifiers
 *     physical addresses
 *     vendor-specific hardware
 *     backend identifiers
 *     calibration data
 *     scheduling data
 *
 * Reflection of a capability or resource description is still only source
 * syntax. Its actual availability is determined downstream by semantic,
 * resource, capability, compilation, deployment, or runtime systems.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are NO language-level finite limits on:
 *
 *     - qualified-name depth;
 *     - type-expression complexity;
 *     - reflection projection count;
 *     - reflection chain depth;
 *     - reflected declarations;
 *     - reflected members;
 *     - reflected metadata;
 *     - reflected generic parameters;
 *     - reflected quantum operations;
 *     - reflected HDL entities;
 *     - reflected distributed entities;
 *     - reflected AI/data entities.
 *
 * Repetition is represented structurally using:
 *
 *     *
 *
 * and not by enumerating finite alternatives.
 *
 * Compiler implementation limits MAY exist for:
 *
 *     parser memory
 *     AST memory
 *     compilation time
 *     diagnostics
 *     semantic analysis
 *     reflection evaluation
 *
 * Those are implementation/resource policies.
 *
 * They MUST NOT become language-level constants.
 *
 * In particular, this grammar MUST NOT introduce:
 *
 *     MAX_REFLECTION_DEPTH
 *     MAX_REFLECTIONS
 *     MAX_MEMBERS
 *     MAX_TYPES
 *     MAX_DECLARATIONS
 *     MAX_ATTRIBUTES
 *     MAX_GENERIC_PARAMETERS
 *     MAX_QUERY_LENGTH
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing reflection syntax is deterministic.
 *
 * Given identical:
 *
 *     source
 *     language version
 *     lexical specification
 *     grammar version
 *
 * the parser must produce the same reflection structure.
 *
 * Parsing MUST NOT depend on:
 *
 *     hardware
 *     filesystem state
 *     network state
 *     environment variables
 *     wall-clock time
 *     randomness
 *     QPU availability
 *     GPU availability
 *     FPGA availability
 *     deployment topology
 *     runtime state
 *
 * Reflection RESULT determinism is a semantic concern.
 *
 * Static language metadata SHOULD be deterministic.
 *
 * External-state reflection MUST be explicitly classified by the effect,
 * capability, resource, and portability systems.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar is syntactic only.
 *
 * Parsing reflection MUST NOT:
 *
 *     - read files;
 *     - read environment variables;
 *     - inspect credentials;
 *     - inspect arbitrary memory;
 *     - inspect processes;
 *     - inspect network state;
 *     - inspect physical hardware;
 *     - contact a QPU;
 *     - contact a GPU;
 *     - contact an FPGA;
 *     - execute host code;
 *     - execute generated code.
 *
 * A semantic reflection implementation may expose additional capabilities only
 * through the repository's established capability/effect/security model.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Reflection may inspect semantic descriptions of quantum entities, including:
 *
 *     Qubit
 *     logical qubits
 *     quantum registers
 *     quantum operations
 *     circuits
 *     measurements
 *     observables
 *     quantum declarations
 *     quantum capabilities
 *
 * It MUST NOT:
 *
 *     - enumerate a fixed gate set;
 *     - assign physical qubits;
 *     - select a QPU;
 *     - route a circuit;
 *     - schedule a circuit;
 *     - perform QEC;
 *     - perform ZQN analysis;
 *     - perform calibration;
 *     - mutate quantum::ir.
 *
 * The canonical quantum path remains:
 *
 *     source
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     semantic quantum model
 *       |
 *       v
 *     quantum::ir
 *
 * Reflection is a read/view facility over that model.
 *
 * ============================================================================
 * HARDWARE / RESOURCE BOUNDARY
 * ============================================================================
 *
 * Reflection does not implicitly mean hardware discovery.
 *
 * For example:
 *
 *     reflect(hardware::capability)
 *
 * is syntactically a reflection request.
 *
 * Whether that entity represents:
 *
 *     source-declared capability metadata
 *     compile-time capability information
 *     target capability information
 *     runtime capability information
 *
 * is decided semantically.
 *
 * The grammar MUST NOT make runtime hardware state implicit.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser produces an ANTLR parse tree only.
 *
 * The frontend must map the structure into the existing domain-neutral AST.
 *
 * Conceptually:
 *
 *     ReflectionExpr
 *         subject
 *             ValueName | Type
 *         projections[]
 *         source_span
 *
 * A projection conceptually contains:
 *
 *     ReflectionProjection
 *         selector
 *         source_span
 *
 * The exact Rust AST type names belong to:
 *
 *     src/frontend/ast/
 *
 * This grammar MUST NOT define Rust AST types.
 *
 * The AST MUST preserve:
 *
 *     - complete source span;
 *     - subject kind;
 *     - qualified-name structure;
 *     - type-expression structure;
 *     - projection order;
 *     - projection spelling;
 *     - nesting;
 *
 * The AST MUST NOT contain:
 *
 *     - device handles;
 *     - physical addresses;
 *     - mutable compiler state;
 *     - runtime pointers;
 *     - backend objects;
 *     - scheduler objects;
 *     - quantum::ir nodes.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Successful parsing means only:
 *
 *     syntactically valid reflection expression
 *
 * Semantic analysis must determine:
 *
 *     1. whether the subject resolves;
 *     2. whether it is reflectable;
 *     3. whether it is visible;
 *     4. whether the requested projection exists;
 *     5. whether the projection is valid for the subject;
 *     6. whether reflection is legal in the current phase;
 *     7. whether the result is compile-time known;
 *     8. whether the result is deterministic;
 *     9. whether capabilities are required;
 *    10. whether effects are required;
 *    11. whether the operation is portable;
 *    12. whether implementation-private information is protected.
 *
 * Semantic errors MUST NOT be encoded as parser alternatives merely to produce
 * custom error messages.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar creates NO IR.
 *
 * Reflection MUST NOT directly create:
 *
 *     classical IR
 *     quantum::ir
 *     HDL IR
 *     hardware IR
 *     routing IR
 *     scheduling IR
 *     QEC operations
 *     ZQN faults
 *     resilience actions
 *
 * The correct relationship is:
 *
 *     reflection syntax
 *          |
 *          v
 *     canonical AST
 *          |
 *          v
 *     semantic reflection model
 *          |
 *          v
 *     existing canonical semantic representation
 *
 * If a compile-time reflection result disappears before target lowering,
 * it remains a compile-time metaprogramming operation.
 *
 * If a reflection result survives into the compiled program, it must use the
 * ordinary canonical type/IR model.
 *
 * ============================================================================
 * NO SECOND REFLECTION TYPE SYSTEM
 * ============================================================================
 *
 * Reflection may have a semantic result type supplied by the canonical type
 * system.
 *
 * This grammar does not define:
 *
 *     ReflectionType
 *     ReflectedType
 *     ReflectionValue
 *     ReflectionObject
 *
 * as a separate source-level type universe.
 *
 * Any semantic reflection value must integrate with the ordinary Zamani type
 * system.
 *
 * ============================================================================
 * NO SECOND QUERY LANGUAGE
 * ============================================================================
 *
 * This grammar intentionally does not define a reflection-specific DSL for:
 *
 *     filters
 *     predicates
 *     sorting
 *     grouping
 *     traversal
 *     search
 *     joins
 *
 * Such facilities, if required, belong to ordinary Zamani expressions or a
 * separately specified metaprogramming query facility.
 *
 * This prevents reflection from becoming a second programming language inside
 * Zamani.
 *
 * ============================================================================
 * PROJECTION MODEL
 * ============================================================================
 *
 * A reflection request consists of:
 *
 *     REFLECT
 *     (
 *         subject
 *     )
 *     projection*
 *
 * Each projection is:
 *
 *     . selector
 *
 * The projection sequence is ordered and unbounded by the grammar.
 *
 * Examples:
 *
 *     reflect(MyType).name
 *
 *     reflect(MyType).kind
 *
 *     reflect(MyType).members
 *
 *     reflect(MyType).members.attributes
 *
 *     reflect(myFunction).parameters
 *
 *     reflect(myFunction).returnType
 *
 *     reflect(myFunction).effects
 *
 *     reflect(type(MyType)).members
 *
 * The grammar does not decide what any selector means.
 *
 * ============================================================================
 * PUBLIC RULE
 * ============================================================================
 *
 * `reflectionExpressionCore` is the canonical public integration boundary
 * consumed by:
 *
 *     grammar/metaprogramming/metaprogramming.g4
 *
 * The wrapper in that composition grammar is expected to be:
 *
 *     reflectionExpression
 *         : reflectionExpressionCore
 *         ;
 *
 * This file MUST NOT redefine `reflectionExpression` itself.
 *
 * This avoids the current duplicate-rule problem.
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */


/*
 * ============================================================================
 * Parser declaration
 * ============================================================================
 */

parser grammar Reflection;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * Canonical grammar dependencies
 * ============================================================================
 *
 * Names:
 *     qualifiedName
 *
 * Types:
 *     typeExpression
 *
 * These are canonical shared grammar components.
 *
 * We deliberately do NOT import the complete Expressions grammar because doing
 * so would create a circular dependency once reflection is inserted into the
 * canonical expression hierarchy.
 *
 * ============================================================================
 */

import
    Names,
    Types
;


/*
 * ============================================================================
 * 1. CANONICAL REFLECTION EXPRESSION
 * ============================================================================
 *
 * Canonical source forms:
 *
 *     reflect(name)
 *
 *     reflect(namespace::name)
 *
 *     reflect type(Type)
 *
 *     reflect type(namespace::Type)
 *
 * followed by zero or more semantic projections.
 *
 * ============================================================================
 */

reflectionExpressionCore
    : REFLECT
      LPAREN
      reflectionSubject
      RPAREN
      reflectionProjection*
    ;


/*
 * ============================================================================
 * 2. REFLECTION SUBJECT
 * ============================================================================
 *
 * There are deliberately two subject classes:
 *
 *     value/declaration name
 *     explicit type
 *
 * The value/declaration form uses qualifiedName rather than expression.
 *
 * This prevents:
 *
 *     expression
 *         -> reflection
 *             -> expression
 *
 * recursion.
 *
 * ============================================================================
 */

reflectionSubject
    : reflectionValueSubject
    | reflectionTypeSubject
    ;


/*
 * ============================================================================
 * 3. VALUE / DECLARATION SUBJECT
 * ============================================================================
 *
 * A qualified name can semantically resolve to:
 *
 *     - a value;
 *     - a constant;
 *     - a function;
 *     - a type-associated declaration;
 *     - a module;
 *     - a namespace;
 *     - a quantum operation;
 *     - an HDL entity;
 *     - a hardware capability;
 *     - another reflectable language entity.
 *
 * Semantic resolution decides which interpretation is valid.
 *
 * ============================================================================
 */

reflectionValueSubject
    : qualifiedName
    ;


/*
 * ============================================================================
 * 4. EXPLICIT TYPE SUBJECT
 * ============================================================================
 *
 * Syntax:
 *
 *     reflect type(T)
 *
 * The TYPE keyword is already part of the canonical Zamani lexical vocabulary.
 *
 * Type syntax remains completely delegated to `typeExpression`.
 *
 * ============================================================================
 */

reflectionTypeSubject
    : TYPE
      LPAREN
      typeExpression
      RPAREN
    ;


/*
 * ============================================================================
 * 5. REFLECTION PROJECTION
 * ============================================================================
 *
 * Projection is deliberately open-ended.
 *
 * The grammar recognizes the structural operation:
 *
 *     . identifier
 *
 * Semantic analysis determines whether the selector is legal.
 *
 * ============================================================================
 */

reflectionProjection
    : DOT
      reflectionSelector
    ;


/*
 * ============================================================================
 * 6. REFLECTION SELECTOR
 * ============================================================================
 *
 * Selectors remain identifiers rather than reserved keywords.
 *
 * This allows future reflection metadata to evolve without continuously
 * changing the lexer.
 *
 * ============================================================================
 */

reflectionSelector
    : identifier
    ;


/*
 * ============================================================================
 * 7. EXPLICIT QUERY ALIAS
 * ============================================================================
 *
 * Tooling and semantic layers may refer to the complete request using the
 * named `reflectionQuery` boundary.
 *
 * It is an alias only.
 *
 * It does not create a second reflection syntax.
 *
 * ============================================================================
 */

reflectionQuery
    : reflectionExpressionCore
    ;


/*
 * ============================================================================
 * 8. REFLECTION PATH
 * ============================================================================
 *
 * A reflection path is the complete reflection expression plus its ordered
 * projection chain.
 *
 * The public expression already contains the same structure, so this rule is
 * provided only as a named tooling/semantic boundary.
 *
 * It does not add syntax.
 *
 * ============================================================================
 */

reflectionPath
    : reflectionExpressionCore
    ;


/*
 * ============================================================================
 * 9. SEMANTIC CATEGORY BOUNDARIES
 * ============================================================================
 *
 * These aliases intentionally do not duplicate syntax.
 *
 * They allow downstream documentation and AST conversion code to refer to the
 * same canonical reflection structure without creating separate parser
 * languages.
 *
 * ============================================================================
 */

reflectionValueQuery
    : reflectionExpressionCore
    ;


reflectionTypeQuery
    : reflectionExpressionCore
    ;


/*
 * ============================================================================
 * 10. AST INTEGRATION CONTRACT
 * ============================================================================
 *
 * The expected semantic conversion is:
 *
 *     reflectionExpressionCore
 *          |
 *          v
 *     ReflectionExpr
 *          |
 *          +--> subject
 *          |      |
 *          |      +--> qualified-name reference
 *          |      |
 *          |      +--> canonical TypeExpr
 *          |
 *          +--> ordered projections
 *          |
 *          +--> source span
 *
 * No parser rule in this file is an AST type declaration.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 11. SEMANTIC INTEGRATION CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     name resolution
 *     type resolution
 *     visibility
 *     projection validation
 *     phase legality
 *     effect checking
 *     capability checking
 *     portability checking
 *     determinism classification
 *     implementation-detail protection
 *
 * Example:
 *
 *     reflect(quantum::operation).capabilities
 *
 * is syntactically valid.
 *
 * Whether:
 *
 *     quantum::operation
 *
 * exists, and whether:
 *
 *     capabilities
 *
 * is a valid projection, is a semantic question.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 12. CLASSICAL INTEGRATION
 * ============================================================================
 *
 * Reflection may inspect:
 *
 *     functions
 *     constants
 *     variables
 *     classes
 *     structs
 *     records
 *     traits
 *     modules
 *     mathematical abstractions
 *     data structures
 *
 * Their meaning remains owned by the corresponding semantic systems.
 *
 * Reflection does not create a classical reflection IR.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 13. QUANTUM INTEGRATION
 * ============================================================================
 *
 * Reflection may inspect semantic quantum entities through ordinary reflection:
 *
 *     reflect(type(Qubit))
 *     reflect(quantum::operation)
 *     reflect(quantum::circuit)
 *
 * It does not enumerate or reserve gate names.
 *
 * Therefore:
 *
 *     H
 *     X
 *     Y
 *     Z
 *     CNOT
 *
 * remain ordinary semantic operation names unless independently reserved by the
 * canonical language.
 *
 * No reflection rule is tied to a finite gate set.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 14. QUANTUM IR INTEGRATION
 * ============================================================================
 *
 * Reflection has no direct dependency on quantum::ir.
 *
 * The correct relationship is:
 *
 *     source
 *       |
 *       v
 *     reflection AST
 *       |
 *       v
 *     semantic reflection
 *       |
 *       v
 *     existing quantum semantic model
 *       |
 *       v
 *     quantum::ir
 *
 * Reflection MUST NOT:
 *
 *     - create quantum::ir nodes;
 *     - mutate quantum::ir;
 *     - assign physical qubits;
 *     - route operations;
 *     - schedule operations;
 *     - invoke QEC;
 *     - invoke ZQN;
 *     - perform calibration.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 15. HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Reflection may inspect semantic descriptions of:
 *
 *     HDL modules
 *     ports
 *     signals
 *     registers
 *     interfaces
 *     hardware capabilities
 *     resource requirements
 *     accelerator intent
 *
 * It must not expose physical implementation state merely because the source
 * contains a reflection request.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 16. DISTRIBUTED / AI / DATA / NETWORKING INTEGRATION
 * ============================================================================
 *
 * The same reflection boundary applies to all language domains.
 *
 * No new parser syntax is needed merely because the reflected entity belongs
 * to:
 *
 *     distributed computing
 *     AI
 *     data processing
 *     networking
 *     security
 *     scientific computing
 *     embedded computing
 *     future computing domains.
 *
 * Domain-specific meaning is semantic.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 17. METAPROGRAMMING INTEGRATION
 * ============================================================================
 *
 * The canonical metaprogramming composition grammar:
 *
 *     grammar/metaprogramming/metaprogramming.g4
 *
 * owns the wrapper:
 *
 *     reflectionExpression
 *         : reflectionExpressionCore
 *         ;
 *
 * This file therefore owns:
 *
 *     reflectionExpressionCore
 *
 * and not:
 *
 *     reflectionExpression
 *
 * This is the single-authority rule.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 18. EXPRESSION INTEGRATION
 * ============================================================================
 *
 * Reflection is an expression-producing metaprogramming facility.
 *
 * The canonical expression hierarchy must integrate the metaprogramming
 * composition boundary at its designated primary-expression extension point.
 *
 * Conceptually:
 *
 *     primaryExpression
 *         :
 *             ...
 *           | metaprogrammingExpression
 *           ;
 *
 * The exact primary-expression composition remains owned by:
 *
 *     grammar/expressions/
 *
 * This file does not redefine primaryExpression.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 19. COMPILE-TIME EXECUTION INTEGRATION
 * ============================================================================
 *
 * Reflection may be used by compile-time semantic evaluation.
 *
 * However:
 *
 *     parsing != execution
 *
 * This grammar never executes reflection.
 *
 * The compile-time subsystem must establish:
 *
 *     phase
 *     capabilities
 *     effects
 *     determinism
 *     provenance
 *     resource policy
 *
 * before evaluating a reflection request.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 20. MACRO INTEGRATION
 * ============================================================================
 *
 * Macros may consume reflection information where the semantic model permits
 * it.
 *
 * Reflection does not:
 *
 *     - expand macros;
 *     - bypass macro hygiene;
 *     - manufacture tokens;
 *     - manufacture syntax trees;
 *     - mutate macro state.
 *
 * Generated source must return through the ordinary parser/AST/semantic
 * pipeline.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 21. GENERATION INTEGRATION
 * ============================================================================
 *
 * Source generation may consume reflection results.
 *
 * Generation remains owned by:
 *
 *     grammar/metaprogramming/generation.g4
 *
 * This grammar does not generate source.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 22. SPECIALIZATION INTEGRATION
 * ============================================================================
 *
 * Specialization may consume reflected type or declaration information.
 *
 * Specialization remains owned by its canonical metaprogramming subsystem.
 *
 * Reflection itself does not specialize code.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 23. RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * Reflection syntax does not resolve resources or capabilities.
 *
 * For example:
 *
 *     reflect(hardware::capability)
 *
 * does not mean:
 *
 *     discover the current GPU
 *
 * or:
 *
 *     discover the current QPU
 *
 * or:
 *
 *     inspect physical memory.
 *
 * Semantic analysis determines whether the reflected entity represents a
 * source-level capability declaration, compile-time capability, target
 * capability, or some explicitly authorized external state.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 24. PORTABILITY CONTRACT
 * ============================================================================
 *
 * A portable program must not accidentally specialize itself based on the
 * machine used to compile it.
 *
 * Therefore:
 *
 *     static language reflection
 *
 * is naturally portable when its semantic inputs are portable.
 *
 * Target-dependent reflection must be explicitly represented by the relevant
 * capability/effect/resource model.
 *
 * It MUST NOT silently become a compile-time machine probe.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 25. RUNTIME REFLECTION
 * ============================================================================
 *
 * Runtime reflection is NOT implied by this grammar.
 *
 * If runtime reflection is introduced later, it must have an explicit semantic
 * contract covering:
 *
 *     runtime effects
 *     capabilities
 *     determinism
 *     security
 *     provenance
 *     portability
 *     resource usage
 *
 * This grammar should not be modified merely to turn static reflection into
 * unrestricted runtime introspection.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 26. ERROR CONTRACT
 * ============================================================================
 *
 * Parser-level errors include malformed syntax such as:
 *
 *     reflect(
 *     reflect()
 *     reflect type(
 *     reflect type()
 *     reflect(name
 *     reflect(name). 
 *
 * Semantic errors include:
 *
 *     unresolved subject
 *     inaccessible subject
 *     unknown projection
 *     invalid projection for subject
 *     phase violation
 *     capability violation
 *     effect violation
 *     non-portable reflection
 *     forbidden implementation-detail exposure
 *
 * Semantic errors belong to semantic analysis.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 27. NEGATIVE SYNTAX CONTRACT
 * ============================================================================
 *
 * The following must not be accepted as reflection syntax:
 *
 *     reflect()
 *     reflect(,)
 *     reflect type()
 *     reflect type(,)
 *     reflect(name
 *     reflect(name)).
 *
 *     reflect name
 *     reflect type name
 *
 *     reflect(unknown expression operators)
 *
 * The final category is intentionally not accepted as a reflection subject
 * because arbitrary expression reflection is a separate semantic design
 * problem and must not introduce expression-recursion into this grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 28. POSITIVE TEST CONTRACT
 * ============================================================================
 *
 * Minimum positive cases:
 *
 *     reflect(value)
 *     reflect(myFunction)
 *     reflect(quantum::operation)
 *     reflect(hardware::capability)
 *     reflect(classical::algorithm)
 *     reflect(type(MyType))
 *     reflect(type(Qubit))
 *     reflect(type(Tensor<float>))
 *
 * Projection cases:
 *
 *     reflect(MyType).name
 *     reflect(MyType).kind
 *     reflect(MyType).members
 *     reflect(MyType).members.attributes
 *     reflect(myFunction).parameters
 *     reflect(myFunction).returnType
 *     reflect(myFunction).effects
 *     reflect(quantum::operation).capabilities
 *     reflect(hardware::capability).requirements
 *
 * Deep chains must be tested structurally without introducing a fixed depth.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 29. BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Boundary tests must include:
 *
 *     - one-segment names;
 *     - deeply qualified names;
 *     - large type expressions;
 *     - deeply nested generic types;
 *     - long projection chains;
 *     - Unicode identifiers where permitted;
 *     - selectors adjacent to keywords;
 *     - source spans at EOF;
 *     - whitespace variation;
 *     - comments around reflection syntax.
 *
 * Boundary tests must distinguish:
 *
 *     language syntax limits
 *
 * from:
 *
 *     compiler resource limits.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 30. SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * Scalability tests should progressively exercise:
 *
 *     reflect(a)
 *     reflect(a::b)
 *     reflect(a::b::c)
 *     ...
 *
 * and:
 *
 *     reflect(a).x
 *     reflect(a).x.y
 *     reflect(a).x.y.z
 *     ...
 *
 * without declaring a maximum language depth.
 *
 * Large source programs containing many independent reflection expressions must
 * remain structurally representable.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 31. DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * Parsing identical source with identical grammar and lexical configuration
 * must produce equivalent parse structures.
 *
 * Reflection parsing must not depend on:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     QPU
 *     filesystem
 *     network
 *     environment
 *     wall clock
 *     randomness
 *     deployment topology.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 32. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * The filename:
 *
 *     grammar/metaprogramming/reflection.g4
 *
 * is retained.
 *
 * The canonical public production is:
 *
 *     reflectionExpressionCore
 *
 * Existing consumers expecting the old local `reflectionExpression` production
 * must migrate to the composition wrapper in:
 *
 *     grammar/metaprogramming/metaprogramming.g4
 *
 * This avoids maintaining two competing reflection-expression authorities.
 *
 * The spelling:
 *
 *     reflect
 *
 * becomes a reserved lexical spelling only after `REFLECT` is added to the
 * canonical keyword vocabulary.
 *
 * Projection names remain identifiers.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 33. REQUIRED LEXER INTEGRATION
 * ============================================================================
 *
 * grammar/lexer/keywords.g4
 *
 * MUST contain exactly one canonical reflection keyword:
 *
 *     REFLECT : 'reflect' ;
 *
 * It must not be defined in this parser grammar.
 *
 * The canonical lexer composition must make REFLECT available to:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * and therefore to:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 34. REQUIRED METAPROGRAMMING INTEGRATION
 * ============================================================================
 *
 * `grammar/metaprogramming/metaprogramming.g4` currently expects:
 *
 *     reflectionExpressionCore
 *     reflectionDeclarationCore
 *     reflectionStatementCore
 *
 * Only the first is legitimately supplied by this file.
 *
 * `reflectionDeclarationCore` and `reflectionStatementCore` MUST NOT be
 * invented here merely to satisfy undefined references.
 *
 * Reflection is an expression facility in this grammar.
 *
 * Therefore the metaprogramming composition grammar must be corrected so that:
 *
 *     metaprogrammingDeclaration
 *
 * does not include a nonexistent reflection declaration form, and:
 *
 *     metaprogrammingStatement
 *
 * does not include a nonexistent reflection statement form.
 *
 * If a future declaration/statement-level reflection feature is desired, it
 * must be specified independently with real source semantics and tests.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 35. REQUIRED EXPRESSION INTEGRATION
 * ============================================================================
 *
 * `grammar/expressions/metaprogramming.g4` should continue to own the
 * expression-level metaprogramming dispatch:
 *
 *     metaprogrammingExpression
 *         :
 *             macroExpression
 *           | compileTimeExpression
 *           | generationExpression
 *           | reflectionExpression
 *           | specializationExpression
 *           ;
 *
 * The reflection wrapper:
 *
 *     reflectionExpression
 *         : reflectionExpressionCore
 *         ;
 *
 * belongs to:
 *
 *     grammar/metaprogramming/metaprogramming.g4
 *
 * or its finalized composition boundary.
 *
 * There must be exactly one such wrapper.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 36. NO HARDWARE LIMITS
 * ============================================================================
 *
 * This grammar contains no:
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
 *     MAX_REFLECTION_DEPTH
 *
 * It also contains no fixed:
 *
 *     physical qubit IDs
 *     GPU IDs
 *     CPU IDs
 *     FPGA IDs
 *     device IDs
 *     vendor IDs
 *     topology IDs
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 37. SAFE-RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no embedded Rust;
 *     - no semantic predicates;
 *     - no target-language actions;
 *     - no filesystem access;
 *     - no network access;
 *     - no runtime execution;
 *     - no hardware access;
 *     - no unsafe code.
 *
 * The compiler implementation consuming this grammar MUST compile with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and MUST remain safe Rust.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * 38. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] Existing filename is retained.
 *
 * [x] Reflection owns one canonical public core production.
 *
 * [x] `reflectionExpressionCore` is the integration boundary.
 *
 * [x] No duplicate reflection-expression authority exists here.
 *
 * [x] Reflection does not recursively import the complete expression grammar.
 *
 * [x] Qualified names use canonical `qualifiedName`.
 *
 * [x] Types use canonical `typeExpression`.
 *
 * [x] Reflection selectors remain extensible identifiers.
 *
 * [x] Projection chains are structurally unbounded.
 *
 * [x] No finite hardware/resource limits are encoded.
 *
 * [x] No quantum gate enumeration is encoded.
 *
 * [x] No physical hardware mapping is encoded.
 *
 * [x] No quantum IR is created.
 *
 * [x] No classical IR is created.
 *
 * [x] No HDL IR is created.
 *
 * [x] No runtime execution is performed.
 *
 * [x] No host introspection is implicit.
 *
 * [x] Static reflection remains suitable for POCO-REAF.
 *
 * [x] Semantic reflection remains downstream.
 *
 * [x] AST integration is defined.
 *
 * [x] IR integration is defined.
 *
 * [x] Quantum integration is defined.
 *
 * [x] Hardware/resource integration is defined.
 *
 * [x] Compile-time integration is defined.
 *
 * [x] Macro integration is defined.
 *
 * [x] Generation integration is defined.
 *
 * [x] Specialization integration is defined.
 *
 * [x] Diagnostics responsibilities are defined.
 *
 * [x] Positive tests are defined.
 *
 * [x] Negative tests are defined.
 *
 * [x] Boundary tests are defined.
 *
 * [x] Scalability tests are defined.
 *
 * [x] Determinism requirements are defined.
 *
 * [x] Compatibility requirements are defined.
 *
 * [x] Rust 1.97 / 1.97.1 safe-Rust requirements are defined.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * Reflection answers:
 *
 *     "What does the canonical Zamani semantic model say about this source
 *      entity or type?"
 *
 * It does NOT implicitly answer:
 *
 *     "What hardware exists right now?"
 *
 *     "Which QPU is available?"
 *
 *     "How many GPUs exist?"
 *
 *     "What is the physical topology?"
 *
 *     "What device should be selected?"
 *
 * Those questions belong to capability, resource, compilation, deployment,
 * runtime, and HAL layers.
 *
 * ============================================================================
 */
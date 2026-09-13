/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/functions/generics.g4
 *
 * Purpose:
 *     Canonical parser grammar for GENERIC PARAMETERS DECLARED BY FUNCTIONS.
 *
 * Scope:
 *     This file owns the syntax used when a function declares generic
 *     parameters.
 *
 * Examples:
 *
 *     fn identity<T>(value: T) -> T { ... }
 *
 *     fn convert<T, U>(value: T) -> U { ... }
 *
 *     fn solve<Q extends QuantumResource>(value: Q) -> Result { ... }
 *
 *     fn transform<T extends Numeric + Ordered>(value: T) -> T { ... }
 *
 * ============================================================================
 * ARCHITECTURAL PRINCIPLE
 * ============================================================================
 *
 * Zamani source expresses computation and semantic requirements.
 *
 * Generic parameters are therefore SOURCE-LEVEL SYMBOLIC PARAMETERS.
 *
 * They do NOT represent:
 *
 *     - physical processors;
 *     - CPU counts;
 *     - GPU counts;
 *     - FPGA counts;
 *     - ASIC counts;
 *     - QPU counts;
 *     - qubit counts;
 *     - memory capacity;
 *     - register capacity;
 *     - hardware topology;
 *     - device identifiers;
 *     - physical addresses;
 *     - deployment size;
 *     - vendor-specific resources.
 *
 * Generic declarations must remain valid from tiny systems to arbitrarily
 * large systems permitted by available compiler/runtime resources.
 *
 * No language-level generic-arity maximum is encoded here.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Generic function declarations are one of the mechanisms that allow:
 *
 *     Program_Once
 *         ->
 *     Compile_Once
 *         ->
 *     Run_Everywhere
 *         ->
 *     Run_Anywhere
 *         ->
 *     Run_Forever
 *
 * A generic function describes reusable semantics.
 *
 * Target realization belongs downstream:
 *
 *     source
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +-------------------+
 *       |                   |
 *       v                   v
 *   classical IR       quantum::ir
 *       |                   |
 *       +---------+---------+
 *                 |
 *                 v
 *       optimization / routing /
 *       scheduling / hardware /
 *       runtime
 *
 * This file must not create dependencies in the reverse direction.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - functionGenericParameters;
 *     - functionGenericParameter;
 *     - functionGenericParameterBounds;
 *     - functionGenericParameterBound;
 *     - functionGenericParameterName;
 *     - source-level generic declaration arity;
 *     - source ordering of generic parameters;
 *     - source ordering of bounds;
 *     - trailing-comma syntax for generic parameter lists.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer tokens;
 *     - identifiers;
 *     - ordinary type-expression syntax;
 *     - generic type application;
 *     - type inference;
 *     - generic substitution;
 *     - monomorphization;
 *     - specialization;
 *     - overload resolution;
 *     - trait/interface resolution;
 *     - constraint solving;
 *     - semantic type checking;
 *     - ABI selection;
 *     - calling conventions;
 *     - hardware selection;
 *     - resource allocation;
 *     - quantum allocation;
 *     - routing;
 *     - scheduling;
 *     - optimization;
 *     - QEC;
 *     - ZQN;
 *     - quantum IR;
 *     - classical IR;
 *     - runtime execution.
 *
 * ============================================================================
 * CANONICAL AST CONTRACT
 * ============================================================================
 *
 * The repository already defines the canonical source-level generic parameter
 * representation:
 *
 *     frontend::ast::node::generics::parameter::TypeParameter
 *
 * with the semantic shape:
 *
 *     TypeParameter
 *         name
 *         bounds: Vec<TypeExpr>
 *
 * This grammar MUST preserve that shape.
 *
 * In particular:
 *
 *     <T>
 *
 * becomes:
 *
 *     name = T
 *     bounds = []
 *
 * and:
 *
 *     <T extends Numeric + Ordered>
 *
 * becomes:
 *
 *     name = T
 *     bounds = [
 *         Numeric,
 *         Ordered
 *     ]
 *
 * Bound order is source order.
 *
 * This grammar MUST NOT introduce a second generic-parameter AST.
 *
 * ============================================================================
 * GENERIC APPLICATION SEPARATION
 * ============================================================================
 *
 * Declaration:
 *
 *     fn identity<T>(value: T) -> T
 *
 * belongs here.
 *
 * Application:
 *
 *     Vec<T>
 *     Result<T, Error>
 *     Matrix<float, Rows, Cols>
 *
 * belongs to:
 *
 *     grammar/types/generic-types.g4
 *
 * Keeping declaration and application syntax separate prevents a circular
 * and duplicated generic type system.
 *
 * ============================================================================
 * TYPE CONSTRAINT SEPARATION
 * ============================================================================
 *
 * This grammar recognizes the SOURCE SYNTAX of bounds.
 *
 * It does not determine whether a bound is semantically valid.
 *
 * For example:
 *
 *     T extends Numeric
 *
 * is syntactically valid if Numeric is a valid type expression.
 *
 * Whether Numeric:
 *
 *     - exists;
 *     - denotes a trait/interface/constraint;
 *     - is applicable to T;
 *     - is satisfiable;
 *     - has an implementation;
 *
 * is determined by semantic analysis.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum-related bounds are intentionally not hard-coded.
 *
 * This is syntactically possible:
 *
 *     fn operate<Q extends QuantumResource>(q: Q) -> Q
 *
 * but this grammar does not know what QuantumResource means.
 *
 * It does NOT determine:
 *
 *     - number of qubits;
 *     - physical qubit identifiers;
 *     - QPU topology;
 *     - gate set;
 *     - calibration;
 *     - noise model;
 *     - QEC strategy;
 *     - routing;
 *     - scheduling.
 *
 * Those concerns remain downstream.
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * ============================================================================
 * CLASSICAL / HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Generic functions can be used with all supported Zamani computational
 * domains without changing this grammar.
 *
 * Examples:
 *
 *     fn map<T, U>(value: T) -> U
 *
 *     fn transform<T extends Numeric>(value: T) -> T
 *
 *     fn execute<Q extends QuantumResource>(value: Q) -> Q
 *
 *     fn synthesize<H extends HardwareResource>(value: H) -> H
 *
 *     fn distribute<N extends NodeResource>(value: N) -> N
 *
 * The meaning of those constraints belongs to semantic analysis and the
 * corresponding domain/resource systems.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There is intentionally NO:
 *
 *     MAX_GENERIC_PARAMETERS
 *     MAX_GENERIC_BOUNDS
 *     MAX_FUNCTION_PARAMETERS
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_MEMORY
 *
 * in this grammar.
 *
 * Repetition is represented using ANTLR repetition operators.
 *
 * If an implementation needs protection against pathological input, that
 * protection belongs to an explicit parser/compiler resource policy rather
 * than the language grammar.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The authoritative lexical vocabulary is:
 *
 *     grammar/lexer/tokens.g4
 *
 * This grammar therefore consumes, but does not define:
 *
 *     IDENTIFIER
 *     LESS_THAN
 *     GREATER_THAN
 *     COMMA
 *     COLON
 *     PLUS
 *     K_EXTENDS
 *
 * No lexer rules are permitted in this file.
 *
 * ============================================================================
 * TYPE GRAMMAR CONTRACT
 * ============================================================================
 *
 * `typeExpression` is owned by the canonical type grammar.
 *
 * This file consumes it as an integration rule.
 *
 * It must not redefine:
 *
 *     typeExpression
 *     typePath
 *     primitiveType
 *     functionType
 *     genericType
 *     referenceType
 *     tupleType
 *     arrayType
 *     resourceType
 *     quantumType
 *     hardwareType
 *
 * ============================================================================
 * FUNCTIONS.G4 INTEGRATION
 * ============================================================================
 *
 * `grammar/functions/functions.g4` already expects:
 *
 *     functionGenericParameters?
 *
 * Therefore that rule is intentionally provided here.
 *
 * The canonical composed Functions grammar must import/delegate this grammar
 * rather than redefine:
 *
 *     functionGenericParameters
 *     functionGenericParameter
 *
 * This eliminates duplicate ownership.
 *
 * ============================================================================
 * CORE/SOURCE-UNIT INTEGRATION
 * ============================================================================
 *
 * `grammar/core/source-unit.g4` currently contains a simpler genericParameter
 * rule.
 *
 * That rule must NOT become a competing generic-parameter model.
 *
 * Migration target:
 *
 *     source-unit generic parameter syntax
 *             |
 *             v
 *     canonical generic declaration grammar
 *
 * The source-unit grammar should eventually delegate generic declarations to
 * the appropriate declaration/function grammar.
 *
 * ============================================================================
 * TYPES/GENERIC-TYPES.G4 INTEGRATION
 * ============================================================================
 *
 * `grammar/types/generic-types.g4` owns GENERIC APPLICATIONS.
 *
 * Example:
 *
 *     Vec<T>
 *
 * It does not own:
 *
 *     <T>
 *
 * after `fn identity`.
 *
 * This file therefore has no dependency on generic application rules beyond
 * the canonical `typeExpression` integration boundary.
 *
 * ============================================================================
 * WHERE-CLAUSE INTEGRATION
 * ============================================================================
 *
 * The AST already contains a canonical WhereClause representation.
 *
 * However, the current lexical contract does not establish a canonical `where`
 * keyword.
 *
 * This file therefore intentionally DOES NOT invent a WHERE token.
 *
 * A future function-level `where` syntax must be introduced as one coordinated
 * change across:
 *
 *     lexer
 *     grammar
 *     AST
 *     semantic analysis
 *     tests
 *     language specification
 *     compatibility policy
 *
 * Until that contract exists, inline `extends` bounds are the canonical
 * function-generic constraint syntax.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * The generated parser/frontend integration MUST target:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * and:
 *
 *     - stable Rust;
 *     - safe Rust;
 *     - no unsafe;
 *     - no unsafe blocks;
 *     - no unsafe functions;
 *     - no machine-specific parser behavior.
 *
 * This grammar itself contains no Rust implementation code.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Given identical source text and identical lexer configuration:
 *
 *     generic parameters
 *     parameter order
 *     bound order
 *
 * must be represented deterministically.
 *
 * The grammar does not use unordered alternatives for semantically identical
 * structures.
 *
 * ============================================================================
 * ERROR BOUNDARY
 * ============================================================================
 *
 * Syntax errors belong to parsing.
 *
 * Semantic errors belong to semantic analysis.
 *
 * Examples of syntax errors:
 *
 *     fn identity<>(x: T) -> T
 *     fn identity<T,>(x: T) -> T
 *
 * are governed by the grammar's selected trailing-comma policy.
 *
 * Examples of semantic errors:
 *
 *     duplicate generic parameter names;
 *     unknown bound;
 *     incompatible bound;
 *     unsatisfied constraint;
 *     invalid generic instantiation;
 *
 * must NOT be encoded here.
 *
 * ============================================================================
 */

parser grammar FunctionGenerics;

options {
    tokenVocab = ZamaniTokens;
}


/*
 * ============================================================================
 * FUNCTION GENERIC PARAMETER LIST
 * ============================================================================
 *
 * Canonical form:
 *
 *     <T>
 *     <T, U>
 *     <T, U, V>
 *
 * Bounded:
 *
 *     <T extends Numeric>
 *     <T extends Numeric + Ordered>
 *     <T extends Numeric, U extends Serializable>
 *
 * A trailing comma is accepted:
 *
 *     <T,>
 *     <T, U,>
 *
 * This is a syntactic convenience and has no semantic meaning.
 *
 * An empty list:
 *
 *     <>
 *
 * is intentionally rejected.
 *
 * There is no fixed upper bound on parameter count.
 */
functionGenericParameters
    : LESS_THAN
      functionGenericParameter
      (
          COMMA
          functionGenericParameter
      )*
      COMMA?
      GREATER_THAN
    ;


/*
 * ============================================================================
 * INDIVIDUAL FUNCTION GENERIC PARAMETER
 * ============================================================================
 *
 * Canonical forms:
 *
 *     T
 *     T extends Numeric
 *     T extends Numeric + Ordered
 *
 * The parameter name is a source identifier.
 *
 * The semantic layer determines:
 *
 *     - whether the name is unique;
 *     - whether the bound is valid;
 *     - whether the bound is satisfiable;
 *     - what declaration the bound denotes.
 */
functionGenericParameter
    : functionGenericParameterName
      functionGenericParameterBounds?
    ;


/*
 * ============================================================================
 * GENERIC PARAMETER NAME
 * ============================================================================
 *
 * Generic parameter names intentionally use the canonical IDENTIFIER token.
 *
 * This means the grammar does not impose conventions such as:
 *
 *     T
 *     U
 *     V
 *
 * Those are naming conventions, not language semantics.
 *
 * Therefore all valid identifiers remain available subject to the canonical
 * lexical rules.
 */
functionGenericParameterName
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * GENERIC PARAMETER BOUNDS
 * ============================================================================
 *
 * Canonical source syntax:
 *
 *     T extends Numeric
 *
 * or:
 *
 *     T extends Numeric + Ordered
 *
 * `K_EXTENDS` belongs to the authoritative lexer.
 *
 * The grammar only establishes the syntactic relationship.
 */
functionGenericParameterBounds
    : K_EXTENDS
      functionGenericParameterBound
      (
          PLUS
          functionGenericParameterBound
      )*
    ;


/*
 * ============================================================================
 * INDIVIDUAL GENERIC BOUND
 * ============================================================================
 *
 * A bound is a canonical type expression.
 *
 * The type grammar owns the actual type syntax.
 *
 * This rule is deliberately only an integration boundary.
 */
functionGenericParameterBound
    : typeExpression
    ;
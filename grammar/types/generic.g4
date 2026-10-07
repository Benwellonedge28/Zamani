/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/generic.g4
 *
 * Grammar:
 *     Generic
 *
 * Status:
 *     CANONICAL GENERIC TYPE-APPLICATION SYNTAX
 *
 * Compiler baseline:
 *     Rust 1.97+
 *     Rust 2021
 *     safe Rust only
 *     no unsafe
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 * Define the reusable source syntax for applying a type constructor to an
 * ordered, arbitrarily sized list of type arguments.
 *
 * Examples:
 *
 *     Vec<T>
 *     Map<Key, Value>
 *     Result<Value, Error>
 *     Option<T>
 *     Tensor<Element>
 *     Model<Input, Output>
 *     Register<Qubit>
 *     Signal<Value>
 *     Accelerator<Model>
 *     Container<Map<Key, Vec<Value>>>
 *
 * OWNED BY THIS FILE
 * ------------------
 * - genericTypeArguments
 * - genericArgumentList
 * - genericTypeArgumentTrailingComma
 * - genericTypeApplicationSuffix
 *
 * NOT OWNED BY THIS FILE
 * ---------------------
 * - typeExpression
 * - typeCore
 * - typePath
 * - identifiers
 * - primitive types
 * - tuple types
 * - arrays
 * - functions
 * - references
 * - pointers
 * - quantum types
 * - HDL types
 * - hardware types
 * - resource types
 * - capability types
 * - dependent/value arguments
 * - generic declarations
 * - generic parameter declarations
 * - generic bounds
 * - type inference
 * - unification
 * - substitution
 * - specialization
 * - monomorphization
 * - overload resolution
 * - trait/type-class resolution
 * - resource negotiation
 * - capability negotiation
 * - target selection
 * - lowering
 * - routing
 * - scheduling
 * - QEC
 * - ZQN
 * - HAL
 *
 * ============================================================================
 * ARCHITECTURAL RULE
 * ============================================================================
 *
 * Generic type APPLICATION and generic parameter DECLARATION are different
 * language constructs.
 *
 * This file owns:
 *
 *     Vec<T>
 *     Map<K, V>
 *     Result<T, E>
 *
 * The generic declaration grammar owns constructs such as:
 *
 *     <T>
 *     <T extends Numeric>
 *     <T, U>
 *
 * Generic declarations therefore remain owned by:
 *
 *     grammar/functions/generics.g4
 *
 * This file MUST NOT define generic parameter declarations.
 *
 * ============================================================================
 * CANONICAL AST
 * ============================================================================
 *
 * Successful generic applications must lower to the existing frontend
 * TypeExpr representation:
 *
 *     TypeExpr::Generic {
 *         base: Box<TypeExpr>,
 *         arguments: Vec<TypeExpr>,
 *     }
 *
 * No GenericTypeApplication AST, GenericArgument AST, or competing type
 * hierarchy is introduced by this grammar.
 *
 * The parser is responsible only for preserving:
 *
 *     - constructor/base structure;
 *     - argument ordering;
 *     - nesting;
 *     - source locations.
 *
 * Semantic analysis is responsible for:
 *
 *     - resolving the constructor;
 *     - checking declared generic arity;
 *     - checking bounds;
 *     - substitution;
 *     - inference;
 *     - specialization;
 *     - compatibility;
 *     - target feasibility.
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 * IMPORTANT:
 *
 * This component must not define a second type-expression grammar.
 *
 * A generic argument is ultimately a complete Zamani type expression.
 *
 * Therefore the production grammar architecture requires a lower-level
 * type-expression composition boundary that makes the canonical
 * `typeExpression` rule available to this component without creating:
 *
 *     Type -> Generic -> Type
 *
 * circular ownership.
 *
 * The intended dependency direction is:
 *
 *     canonical type-expression boundary
 *                  |
 *                  +----> Generic
 *                  |
 *                  +----> Named
 *                  |
 *                  +----> Tuple
 *                  |
 *                  +----> Array
 *                  |
 *                  +----> Function
 *                  |
 *                  +----> Quantum
 *                  |
 *                  +----> Hardware
 *                  |
 *                  +----> ...
 *
 * The final orchestrator may expose this through the existing Type/Types
 * compatibility facade, but `generic.g4` must have exactly one owner for the
 * generic argument syntax.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This grammar contains NO lexer rules.
 *
 * Canonical parser tokens:
 *
 *     LESS
 *     GREATER
 *     COMMA
 *
 * These correspond to:
 *
 *     <
 *     >
 *     ,
 *
 * The following obsolete/competing names MUST NOT be used here:
 *
 *     LESS_THAN
 *     GREATER_THAN
 *
 * The repository's lexical normalization specifies LESS/GREATER as the
 * canonical generic delimiters.
 *
 * ============================================================================
 * GENERIC APPLICATION
 * ============================================================================
 *
 * The base type is deliberately NOT owned here.
 *
 * A composing type grammar is responsible for parsing:
 *
 *     typePath
 *
 * or another canonical type constructor.
 *
 * It then attaches:
 *
 *     genericTypeApplicationSuffix
 *
 * producing the semantic structure:
 *
 *     base<arguments...>
 *
 * ============================================================================
 * ARGUMENT LIST
 * ============================================================================
 *
 * At least one type argument is required.
 *
 * Therefore:
 *
 *     <>
 *
 * is invalid.
 *
 * A list may contain any number of complete type expressions.
 *
 * Examples:
 *
 *     <T>
 *     <T, U>
 *     <T, U, V>
 *     <T, U, V, W>
 *
 * There is no grammar-level maximum.
 *
 * ============================================================================
 * TRAILING COMMA
 * ============================================================================
 *
 * A trailing comma is valid:
 *
 *     Vec<T,>
 *     Map<K, V,>
 *     Result<T, E,>
 *
 * The trailing comma does NOT represent an additional argument.
 *
 * Therefore:
 *
 *     Vec<T,>
 *
 * contains exactly one type argument.
 *
 * ============================================================================
 * NESTING
 * ============================================================================
 *
 * Generic applications may be nested arbitrarily subject to implementation
 * resource availability:
 *
 *     Vec<Option<T>>
 *
 *     Map<K, Vec<V>>
 *
 *     Result<Vec<T>, Error>
 *
 *     Container<Map<K, Result<Vec<V>, Error>>>
 *
 * The language grammar does not impose a semantic nesting ceiling.
 *
 * Operational parser/compiler protection may exist outside language semantics
 * and must be:
 *
 *     explicit;
 *     configurable;
 *     diagnosable;
 *     independent of physical hardware limits.
 *
 * ============================================================================
 * TYPE ARGUMENTS
 * ============================================================================
 *
 * Each argument is a canonical type expression.
 *
 * This permits generic applications to compose with:
 *
 *     named types
 *     qualified types
 *     other generic types
 *     tuples
 *     arrays
 *     slices
 *     function types
 *     references
 *     pointers
 *     option/result types
 *     quantum types
 *     classical types
 *     hardware types
 *     resource types
 *     capability types
 *     dependent types
 *     future domain-neutral type forms
 *
 * Examples:
 *
 *     Vec<&T>
 *     Vec<&mut T>
 *     Vec<(A, B)>
 *     Vec<fn(A) -> B>
 *     Vec<Result<A, B>>
 *     Register<LogicalQubit>
 *     Tensor<Value>
 *
 * ============================================================================
 * DEPENDENT / VALUE PARAMETERS
 * ============================================================================
 *
 * Generic.g4 does not reinterpret arbitrary arguments as values.
 *
 * Type-level values remain owned by the canonical dependent/value type
 * grammar.
 *
 * For example, if the language defines:
 *
 *     Matrix<T>[Rows, Columns]
 *
 * the generic component must not silently redefine:
 *
 *     Matrix<T, Rows>
 *
 * as a value-parameterized generic.
 *
 * Type/value distinction is preserved until semantic analysis.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * These are NOT parser errors:
 *
 *     Unknown<T>
 *     Result<T>
 *     Vec<T, U, V>
 *
 * if the syntax itself is valid.
 *
 * Their validity depends on declarations and semantic information.
 *
 * Semantic analysis determines:
 *
 *     whether the constructor exists;
 *     whether it is generic;
 *     its declared arity;
 *     whether the supplied arguments satisfy its bounds;
 *     whether substitution succeeds;
 *     whether the resulting type is well formed.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Generic type syntax introduces no execution effect.
 *
 * Examples:
 *
 *     Network<Packet>
 *     Qubit
 *     Resource<Device>
 *     Model<Input, Output>
 *
 * do not perform network access, quantum measurement, resource allocation,
 * learning, adaptation, or any other effect merely by appearing as types.
 *
 * Effects belong to executable operations and the effect system.
 *
 * ============================================================================
 * CAPABILITY / RESOURCE CONTRACT
 * ============================================================================
 *
 * Generic syntax does not select or allocate physical resources.
 *
 * For example:
 *
 *     Register<Qubit>
 *     Tensor<Value>
 *     Accelerator<Model>
 *
 * does not determine:
 *
 *     QPU;
 *     CPU;
 *     GPU;
 *     FPGA;
 *     ASIC;
 *     memory device;
 *     topology;
 *     physical qubit;
 *     deployment location.
 *
 * Resource and capability analysis occurs downstream.
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * Quantum types may appear as generic arguments.
 *
 * Examples:
 *
 *     Register<Qubit>
 *     Register<LogicalQubit>
 *     QuantumState<State>
 *     QuantumOperation<Operation>
 *
 * This grammar does not:
 *
 *     allocate qubits;
 *     select physical qubits;
 *     select a QPU;
 *     perform routing;
 *     perform decomposition;
 *     schedule operations;
 *     choose calibration;
 *     perform QEC;
 *     construct ZQN;
 *     access HAL state.
 *
 * Quantum semantics continue through the canonical:
 *
 *     source
 *       |
 *       v
 *     domain-neutral AST
 *       |
 *       v
 *     semantic quantum model
 *       |
 *       v
 *     quantum::ir
 *
 * ============================================================================
 * HDL / HARDWARE CONTRACT
 * ============================================================================
 *
 * Generic applications can describe hardware-neutral abstractions:
 *
 *     Signal<Value>
 *     Register<Value>
 *     Buffer<Data>
 *     Accelerator<Model>
 *     Module<Configuration>
 *
 * They must not encode universal physical limits such as:
 *
 *     fixed register width;
 *     fixed memory capacity;
 *     fixed device count;
 *     fixed pipeline depth;
 *     fixed topology size.
 *
 * ============================================================================
 * AI / DATA CONTRACT
 * ============================================================================
 *
 * Generic syntax is sufficient to represent domain-neutral data and model
 * relationships such as:
 *
 *     Tensor<Element>
 *     Dataset<Record>
 *     Model<Input, Output>
 *     Distribution<Value>
 *     Probability<Value>
 *     KnowledgeGraph<Node, Edge>
 *
 * No algorithm-specific keyword inventory belongs here.
 *
 * ============================================================================
 * CONTRACT / POLICY / PROVENANCE
 * ============================================================================
 *
 * Generic applications may appear anywhere a type expression is accepted,
 * including declarations governed by:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * and policies.
 *
 * This grammar does not own contracts, policies, or provenance.
 *
 * Source spans from the parser must remain available to downstream semantic
 * and provenance systems.
 *
 * ============================================================================
 * PORTABILITY / POCO-REAF
 * ============================================================================
 *
 * Generic types describe source-level relationships, not target capacity.
 *
 * A generic program can therefore remain source-compatible across:
 *
 *     embedded systems;
 *     CPUs;
 *     multicore systems;
 *     GPUs;
 *     FPGAs;
 *     ASICs;
 *     accelerators;
 *     QPUs;
 *     simulators;
 *     HPC systems;
 *     clusters;
 *     distributed systems;
 *     cloud systems;
 *     future computational substrates.
 *
 * Physical feasibility is resolved downstream.
 *
 * Changing target resources must not require changing the generic syntax or
 * the source-level type meaning.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * This grammar contains no artificial limits on:
 *
 *     generic argument count;
 *     generic constructor count;
 *     nesting;
 *     type declaration count;
 *     source program size;
 *     domain count;
 *     target count;
 *     hardware count;
 *     quantum resource count;
 *     memory capacity;
 *     processor count;
 *     accelerator count.
 *
 * The grammar MUST NOT contain constants such as:
 *
 *     MAX_GENERIC_ARGUMENTS
 *     MAX_GENERIC_ARITY
 *     MAX_GENERIC_DEPTH
 *     MAX_TYPE_DEPTH
 *
 * Nor any machine-specific ceiling.
 *
 * Operational safeguards belong to compiler/parser configuration and are not
 * part of the source-language grammar semantics.
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Given identical:
 *
 *     source;
 *     language version;
 *     lexical configuration;
 *     grammar version;
 *
 * the generic grammar must produce identical structural parsing.
 *
 * Generic parsing must not depend on:
 *
 *     time;
 *     randomness;
 *     filesystem state;
 *     network state;
 *     target hardware;
 *     runtime state;
 *     resource availability.
 *
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Syntax errors owned here include malformed generic delimiters/lists.
 *
 * Examples:
 *
 *     <>
 *     <, T>
 *     <T,, U>
 *     <T U>
 *     <T, ,>
 *     <T, U
 *
 * Valid:
 *
 *     <T>
 *     <T,>
 *     <T, U>
 *     <T, U,>
 *
 * Semantic diagnostics remain downstream:
 *
 *     wrong arity;
 *     unknown constructor;
 *     invalid bound;
 *     invalid substitution;
 *     incompatible type;
 *     unavailable target realization.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * The old generic argument rules currently duplicated in the type
 * orchestrator must be removed from the canonical type grammar once this
 * component is connected.
 *
 * There must be exactly one owner for:
 *
 *     genericTypeArguments
 *     genericArgumentList
 *     genericTypeArgumentTrailingComma
 *
 * Compatibility aliases must only be retained when an actual repository
 * consumer requires them.
 *
 * No duplicate `genericArgumentList` may exist in another active grammar.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     canonical type-expression composition boundary
 *     ZamaniLexer
 *
 * CONSUMES:
 *
 *     typeExpression
 *     LESS
 *     GREATER
 *     COMMA
 *
 * EXPORTS:
 *
 *     genericTypeArguments
 *     genericArgumentList
 *     genericTypeArgumentTrailingComma
 *     genericTypeApplicationSuffix
 *
 * AST_OWNER:
 *
 *     existing frontend TypeExpr::Generic
 *
 * SEMANTIC_OWNER:
 *
 *     generic/type semantic analysis
 *
 * IR_OWNER:
 *
 *     canonical semantic type model
 *
 * QUANTUM_IR_OWNER:
 *
 *     quantum::ir
 *
 * TEST_OWNER:
 *
 *     grammar/tests/types/
 *     parser/type conformance tests
 *     frontend AST tests
 *
 * SPEC_OWNER:
 *
 *     grammar/specification/types.md
 *     grammar/specification/poco-reaf.md
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     1. It contains no lexer rules.
 *     2. It contains no generic declaration rules.
 *     3. It contains no duplicate type-expression grammar.
 *     4. It uses canonical LESS/GREATER tokens.
 *     5. It rejects empty generic applications.
 *     6. It accepts arbitrary source-defined argument counts.
 *     7. It accepts trailing commas.
 *     8. It preserves argument ordering.
 *     9. It supports recursive generic nesting.
 *    10. It has exactly one active generic-list owner.
 *    11. It lowers to the existing TypeExpr::Generic representation.
 *    12. It introduces no target/hardware limits.
 *    13. It introduces no execution effects.
 *    14. It does not perform resource/capability resolution.
 *    15. It does not create a second quantum IR.
 *    16. It has positive, negative, boundary, scalability, and deterministic
 *        conformance coverage.
 *    17. The Rust frontend consuming it remains compatible with Rust 1.97+
 *        and safe Rust only.
 *
 * ============================================================================
 * GRAMMAR
 * ============================================================================
 */

parser grammar Generic;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * The enclosing type-expression composition layer supplies `typeExpression`.
 *
 * This rule owns only the angle-delimited argument sequence.
 */
genericTypeArguments
    : LESS
      genericArgumentList
      GREATER
    ;

/*
 * One or more complete type expressions.
 *
 * The final optional comma is syntax only and does not create an argument.
 */
genericArgumentList
    : typeExpression
      (
          COMMA
          typeExpression
      )*
      genericTypeArgumentTrailingComma?
    ;

/*
 * Explicit trailing comma.
 *
 * Keeping this separate makes the argument-count semantics unambiguous:
 * argument count is the number of typeExpression occurrences.
 */
genericTypeArgumentTrailingComma
    : COMMA
    ;

/*
 * Reusable suffix for the canonical type-expression composition layer.
 *
 * Example:
 *
 *     typePath genericTypeApplicationSuffix
 *
 * represents:
 *
 *     TypeExpr::Generic(base, arguments)
 */
genericTypeApplicationSuffix
    : genericTypeArguments
    ;
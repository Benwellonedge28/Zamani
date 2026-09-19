/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/functions/return-types.g4
 *
 * Grammar:
 *     ReturnTypes
 *
 * Role:
 *     Canonical reusable parser grammar for callable return-type clauses.
 *
 * Status:
 *     Production parser contract.
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     This grammar contains no Rust implementation code.
 *     Zamani compiler/frontend/runtime Rust MUST remain safe Rust only.
 *     No unsafe Rust is required by this grammar.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns exactly one source-level syntactic construct:
 *
 *     functionReturnClause
 *
 * whose canonical spelling is:
 *
 *     -> TypeExpression
 *
 * Examples:
 *
 *     fn main() -> Int {
 *         ...
 *     }
 *
 *     fn identity<T>(value: T) -> T {
 *         ...
 *     }
 *
 *     fn measure(q: Qubit) -> Measurement {
 *         ...
 *     }
 *
 *     fn compute() -> Result<Value, Error> {
 *         ...
 *     }
 *
 *     fn transform() -> quantum::State {
 *         ...
 *     }
 *
 *     fn tensor() -> Tensor<Float, N> {
 *         ...
 *     }
 *
 * The enclosing callable declaration owns optionality:
 *
 *     functionReturnClause?
 *
 * This file therefore MUST NOT encode whether a function is required to have
 * a return clause.
 *
 * ============================================================================
 * FILE OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - functionReturnClause;
 *     - the structural attachment of THIN_ARROW;
 *     - the structural attachment of exactly one typeExpression;
 *     - the reusable parser boundary for callable result types.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer definitions;
 *     - token spellings;
 *     - keywords;
 *     - identifiers;
 *     - typeExpression;
 *     - primitive types;
 *     - generic types;
 *     - quantum types;
 *     - hardware types;
 *     - resource types;
 *     - function types;
 *     - references;
 *     - pointers;
 *     - optional types;
 *     - Result types;
 *     - dependent types;
 *     - type-level values;
 *     - function declarations;
 *     - function names;
 *     - parameters;
 *     - generic declarations;
 *     - effects;
 *     - capabilities;
 *     - contracts;
 *     - function bodies;
 *     - return statements;
 *     - type checking;
 *     - type inference;
 *     - name resolution;
 *     - generic substitution;
 *     - overload resolution;
 *     - ownership;
 *     - borrowing;
 *     - resource allocation;
 *     - hardware discovery;
 *     - target selection;
 *     - ABI selection;
 *     - calling-convention selection;
 *     - optimization;
 *     - routing;
 *     - scheduling;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution;
 *     - quantum IR.
 *
 * ============================================================================
 * AUTHORITY
 * ============================================================================
 *
 * The authority chain is:
 *
 *     language specification
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     canonical parser grammar
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     semantic type analysis
 *          |
 *          v
 *     canonical semantic representation / IR
 *
 * This file is NOT an independent type-system authority.
 *
 * The canonical type syntax is owned by:
 *
 *     grammar/types/types.g4
 *
 * The canonical source-level AST representation is owned by the frontend AST,
 * including:
 *
 *     src/frontend/ast/node/types/type_expr.rs
 *
 * ============================================================================
 * CANONICAL RETURN SYNTAX
 * ============================================================================
 *
 * The only universal callable return-clause syntax owned here is:
 *
 *     -> typeExpression
 *
 * The arrow is represented by:
 *
 *     THIN_ARROW
 *
 * The type is represented by:
 *
 *     typeExpression
 *
 * No alternative return-arrow spelling is introduced here.
 *
 * In particular, this grammar MUST NOT independently introduce:
 *
 *     ARROW typeExpression
 *     FAT_ARROW typeExpression
 *     ':' typeExpression
 *     RETURNS typeExpression
 *     RETURN_TYPE typeExpression
 *
 * unless a future language version explicitly establishes such syntax.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical parser lexer vocabulary is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * The modular lexical specification is organized under:
 *
 *     grammar/lexer/
 *
 * The canonical return arrow is:
 *
 *     THIN_ARROW
 *
 * with spelling:
 *
 *     ->
 *
 * owned by:
 *
 *     grammar/lexer/operators.g4
 *
 * This grammar MUST NOT define lexer rules.
 *
 * ============================================================================
 * TYPE CONTRACT
 * ============================================================================
 *
 * `typeExpression` comes from:
 *
 *     grammar/types/types.g4
 *
 * This file MUST NOT duplicate it.
 *
 * Consequently every type already accepted by the canonical type grammar can
 * structurally occur as a callable return type.
 *
 * Examples include, where supported by the type system:
 *
 *     Int
 *     Float
 *     Bool
 *     String
 *     User
 *     module::User
 *     Vec<T>
 *     Result<T, E>
 *     (A, B)
 *     [T]
 *     [T; N]
 *     fn(T) -> U
 *     &T
 *     &'a T
 *     &mut T
 *     *T
 *     Qubit
 *     Quantum<T>
 *     Resource<T>
 *     Capability<C>
 *     Tensor<T, N>
 *     temporal/domain-specific types
 *     extensible domain types
 *
 * This means this file does not need to change when a new domain-specific type
 * is added to the canonical type system.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * A return type describes the source-level result of computation.
 *
 * It does NOT describe target realization.
 *
 * This grammar therefore contains no:
 *
 *     MAX_RETURN_TYPES
 *     MAX_TYPE_DEPTH
 *     MAX_GENERIC_ARITY
 *     MAX_TUPLE_ARITY
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_REGISTER_WIDTH
 *     MAX_TENSOR_RANK
 *     MAX_TENSOR_DIMENSION
 *
 * A return type such as:
 *
 *     Tensor<Float, N>
 *
 * remains source-level semantic information.
 *
 * It does not select:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     accelerator
 *     physical memory
 *     physical qubit
 *     network node
 *     device identifier
 *
 * Those decisions belong downstream.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * The grammar imposes no language-level finite limit on:
 *
 *     - number of functions;
 *     - number of callable declarations;
 *     - number of return clauses;
 *     - type nesting;
 *     - generic nesting;
 *     - qualified-name depth;
 *     - tuple arity;
 *     - symbolic dimensions;
 *     - type-level expressions;
 *     - quantum-resource cardinality;
 *     - hardware-resource cardinality.
 *
 * For example, all of the following use the same grammar boundary:
 *
 *     -> T
 *
 *     -> Result<T, E>
 *
 *     -> Result<Vec<Option<T>>, E>
 *
 *     -> domain::Result<domain::Container<T, N>>
 *
 * Practical parser/resource limits, if required for hostile-input protection,
 * belong to explicit implementation policy. They MUST NOT be represented as
 * grammar restrictions.
 *
 * ============================================================================
 * ANTLR COMPOSITION
 * ============================================================================
 *
 * This is a parser delegate.
 *
 * It consumes the canonical lexer vocabulary and imports the canonical Types
 * parser grammar:
 *
 *     Types
 *
 * Therefore:
 *
 *     ReturnTypes
 *         |
 *         +--> THIN_ARROW
 *         |
 *         +--> typeExpression
 *
 * There must be exactly one authoritative definition of:
 *
 *     functionReturnClause
 *
 * ============================================================================
 */

parser grammar ReturnTypes;

options {
    tokenVocab = ZamaniLexer;
}

import Types;


/* ============================================================================
 * CANONICAL FUNCTION RETURN CLAUSE
 * ============================================================================
 *
 * Complete return-type attachment:
 *
 *     -> TypeExpression
 *
 * Optionality belongs to the enclosing callable grammar:
 *
 *     functionReturnClause?
 *
 * This rule itself is never empty.
 * ============================================================================
 */

functionReturnClause
    : THIN_ARROW
      typeExpression
    ;
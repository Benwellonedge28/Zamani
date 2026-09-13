/**
 * Zamani Programming Language
 *
 * File:
 *     grammar/declarations/structs.g4
 *
 * Purpose:
 *     Canonical modular ANTLR4 parser grammar for Zamani structure
 *     declarations and structure fields.
 *
 * Architectural role:
 *
 *     This grammar owns the SOURCE SYNTAX of:
 *
 *       - struct declarations
 *       - struct bodies
 *       - struct fields
 *       - field separators
 *
 *     It does NOT own semantic type checking, layout, ABI, alignment,
 *     memory placement, hardware resources, quantum resources, scheduling,
 *     routing, optimization, QEC, ZQN, runtime state, or physical
 *     representation.
 *
 * POCO-REAF:
 *
 *     A struct describes a source-level data abstraction.
 *
 *     It MUST NOT encode accidental properties of the machine on which
 *     the program eventually executes.
 *
 *     Therefore this grammar deliberately contains no fixed limits for:
 *
 *       - fields
 *       - nesting depth
 *       - struct declarations
 *       - type parameters
 *       - object size
 *       - alignment
 *       - memory
 *       - registers
 *       - cores
 *       - threads
 *       - devices
 *       - qubits
 *       - nodes
 *       - topology
 *
 *     Any actual implementation/resource limit belongs to the appropriate
 *     compiler, resource, target, runtime, or deployment layer.
 *
 * Repository boundary:
 *
 *     Source
 *       -> lexer
 *       -> parser
 *       -> AST
 *       -> semantic analysis
 *       -> canonical IR/domain IR
 *       -> optimization/lowering
 *       -> target/resource selection
 *       -> runtime
 *
 *     This grammar participates only in the syntax portion of that flow.
 *
 * Quantum boundary:
 *
 *     This grammar MUST NOT create or redefine quantum IR.
 *
 *     Quantum-specific types appearing in a struct are parsed through the
 *     shared type-expression contract and are interpreted downstream.
 *
 * Hardware boundary:
 *
 *     Hardware-specific types appearing in a struct are syntax only.
 *     Hardware interpretation belongs to the hardware/HDL/compiler layers.
 *
 * Rust integration:
 *
 *     Generated parser/front-end integration MUST remain compatible with
 *     Rust 1.97 / 1.97.1.
 *
 *     No embedded target-language actions are permitted here.
 *
 *     This grammar therefore contains no unsafe code and introduces no
 *     Rust unsafe requirement.
 *
 * Ownership:
 *
 *     OWNS:
 *       - structDeclaration
 *       - structBody
 *       - structField
 *       - structFieldList
 *       - structFieldSeparator
 *
 *     DOES NOT OWN:
 *       - lexer tokens
 *       - identifiers
 *       - qualified names
 *       - visibility semantics
 *       - generic parameter semantics
 *       - where-clause semantics
 *       - type-expression semantics
 *       - attributes
 *       - annotations
 *       - expressions
 *       - methods/functions
 *       - implementations
 *       - traits/interfaces
 *       - enum variants
 *       - unions
 *       - memory layout
 *       - ABI
 *       - serialization format
 *       - hardware layout
 *       - quantum layout
 *       - scheduling
 *       - routing
 *       - QEC
 *       - ZQN
 *       - optimization
 *       - runtime execution
 *
 * Shared-rule ownership:
 *
 *     identifier
 *         -> core/names.g4
 *
 *     visibilityModifier
 *         -> shared declaration/core visibility grammar
 *
 *     genericParameters
 *         -> dependency-neutral shared generic grammar
 *
 *     whereClause
 *         -> dependency-neutral shared constraint grammar
 *
 *     typeExpression
 *         -> types/types.g4 and its subordinate type grammars
 *
 *     attributes / annotations
 *         -> core/attributes.g4 and/or core/annotations.g4
 *
 *     No rule above is redefined locally.
 *
 * Compatibility:
 *
 *     Existing Zamani struct syntax must be migrated into this grammar
 *     without silently removing valid syntax.
 *
 *     Existing grammar.md describes a field approximately as:
 *
 *         ["public"] IDENT [":" TypeExpr] [","]
 *
 *     This grammar preserves the essential form while allowing the
 *     repository's canonical visibility and identifier rules to remain
 *     authoritative.
 *
 *     If the canonical parser supports additional valid struct modifiers,
 *     attributes, or generic/where syntax, those must be incorporated
 *     through the shared contracts rather than duplicated here.
 *
 * ANTLR:
 *
 *     This is a parser grammar and therefore consumes the canonical lexer
 *     token vocabulary.
 *
 *     The canonical lexer authority is:
 *
 *         ZamaniLexer
 *
 *     No second token vocabulary is introduced here.
 */

parser grammar ZamaniDeclarationStructs;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * -------------------------------------------------------------------------
 * Struct declaration
 * -------------------------------------------------------------------------
 *
 * Canonical source-level form:
 *
 *     struct Name {
 *         field: Type
 *     }
 *
 * Generic and constraint syntax is delegated to shared grammar contracts.
 *
 * Attributes/annotations are intentionally not consumed here unless the
 * repository establishes them as part of the declaration wrapper. This
 * prevents declaration-specific duplication.
 */

structDeclaration
    : visibilityModifier?
      STRUCT
      identifier
      genericParameters?
      whereClause?
      structBody
    ;

/*
 * -------------------------------------------------------------------------
 * Struct body
 * -------------------------------------------------------------------------
 *
 * Zero fields is legal.
 *
 * There is deliberately no numeric upper bound.
 *
 * An implementation may impose operational limits elsewhere, but such
 * limits are not language grammar restrictions.
 */

structBody
    : LBRACE
      structFieldList?
      RBRACE
    ;

/*
 * -------------------------------------------------------------------------
 * Struct field list
 * -------------------------------------------------------------------------
 *
 * A field is separated by commas.
 *
 * A trailing comma is accepted.
 *
 * This gives both of the following forms:
 *
 *     struct Point {
 *         x: Int,
 *         y: Int
 *     }
 *
 * and:
 *
 *     struct Point {
 *         x: Int,
 *         y: Int,
 *     }
 *
 * The separator belongs to the field-list syntax, not to the field's
 * semantic meaning.
 */

structFieldList
    : structField
      (
          COMMA
          structField
      )*
      COMMA?
    ;

/*
 * -------------------------------------------------------------------------
 * Struct field
 * -------------------------------------------------------------------------
 *
 * This is the canonical reusable field contract.
 *
 * It is intentionally exposed as `structField` because enum struct
 * variants may reuse exactly the same field representation.
 *
 * A field consists of:
 *
 *     optional visibility
 *     field identifier
 *     required type annotation
 *
 * Example:
 *
 *     public x: Int
 *
 *     y: Float
 *
 * The grammar does not infer machine layout from declaration order.
 * Declaration order is retained by the parser/AST for semantic purposes,
 * including deterministic source ordering.
 */

structField
    : visibilityModifier?
      identifier
      COLON
      typeExpression
    ;

/*
 * -------------------------------------------------------------------------
 * Compatibility alias
 * -------------------------------------------------------------------------
 *
 * `structFieldSeparator` exists as a named contract so downstream grammar
 * aggregation and diagnostics can reason about field separation without
 * duplicating separator syntax.
 *
 * It is intentionally not used as a second separator inside
 * `structFieldList`, because the list itself owns comma placement.
 *
 * If the final grammar aggregator does not require this rule, it may be
 * omitted from the generated parser API during grammar consolidation.
 */

structFieldSeparator
    : COMMA
    ;

/*
 * -------------------------------------------------------------------------
 * Multiple struct declarations
 * -------------------------------------------------------------------------
 *
 * Useful to grammar aggregators and declaration-level tests.
 *
 * No fixed declaration count is imposed.
 */

structDeclarations
    : structDeclaration+
    ;
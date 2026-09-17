/**
 * Zamani Record Declarations
 *
 * File:
 *   grammar/declarations/records.g4
 *
 * Grammar:
 *   ZamaniDeclarationRecords
 *
 * Purpose:
 *   Defines the declaration-level syntax for target-independent Zamani
 *   record types.
 *
 * Authority:
 *   This grammar is a parser component. It does not define lexical tokens,
 *   semantic typing, runtime layout, ABI representation, serialization,
 *   storage placement, hardware mapping, or IR implementation.
 *
 * Integration:
 *   - Imported by the canonical declaration composition grammar.
 *   - Composed by grammar/Zamani.g4.
 *   - Reuses canonical identifier, visibility, attributes, generic,
 *     where-clause, type, and field rules.
 *   - Record values/literals and data operations belong to grammar/data/
 *     and must not duplicate this declaration grammar.
 *
 * Portability:
 *   Records are target-independent semantic aggregates.
 *   No CPU, GPU, FPGA, QPU, memory, register, word-size, field-count,
 *   nesting-depth, or other hardware limit is encoded here.
 *
 * POCO-REAF:
 *   Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
 *
 * Rust:
 *   This grammar is consumed by the Rust frontend/compiler.
 *   It imposes no unsafe Rust requirement and must remain compatible
 *   with Rust 1.97 / 1.97.1 implementation constraints.
 */

parser grammar ZamaniDeclarationRecords;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * Record declaration
 * ============================================================================
 *
 * General form:
 *
 *   record Name {
 *       field: Type,
 *       ...
 *   }
 *
 * Generic form:
 *
 *   record Name<T, U> {
 *       first: T,
 *       second: U,
 *   }
 *
 * Constrained form:
 *
 *   record Name<T>
 *   where
 *       T: Constraint
 *   {
 *       value: T,
 *   }
 *
 * Visibility and attributes are deliberately delegated to shared grammar
 * contracts rather than being reimplemented here.
 */
recordDeclaration
    : attribute* visibilityModifier? RECORD identifier
      genericParameters?
      recordWhereClause?
      recordBody
    ;


/*
 * ============================================================================
 * Record body
 * ============================================================================
 *
 * A record may contain zero or more fields.
 *
 * No fixed field count is imposed.
 */
recordBody
    : LBRACE recordFieldList? RBRACE
    ;


/*
 * ============================================================================
 * Record fields
 * ============================================================================
 *
 * Field syntax is deliberately based on the canonical shared field contract.
 *
 * This prevents records, structs, enum struct variants, and other aggregate
 * declarations from developing incompatible field grammars.
 */
recordFieldList
    : recordField (COMMA recordField)* COMMA?
    ;


/*
 * ============================================================================
 * Individual record field
 * ============================================================================
 *
 * A record field is:
 *
 *   name: Type
 *
 * The canonical `structField` rule owns the actual field syntax so that
 * field declarations have one grammar-wide definition.
 *
 * Keeping this adapter rule gives records a stable public grammar contract
 * without duplicating field syntax.
 */
recordField
    : structField
    ;


/*
 * ============================================================================
 * Where clause
 * ============================================================================
 *
 * `whereClause` is the canonical constraint grammar.
 *
 * This adapter is intentionally retained so callers of this grammar have a
 * stable record-specific rule while the actual constraint language remains
 * centralized.
 */
recordWhereClause
    : whereClause
    ;
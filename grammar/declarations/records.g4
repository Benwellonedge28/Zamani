/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/data/records.g4
 *
 * Grammar:
 *     ZamaniDataRecords
 *
 * Status:
 *     Production / canonical logical-record grammar
 *
 * Purpose:
 *     Own the source syntax of logical data-record declarations.
 *
 * ============================================================================
 * ARCHITECTURAL ROLE
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     grammar/antlr/ZamaniLexer.g4
 *          |
 *          v
 *     canonical parser
 *          |
 *          v
 *     grammar/data/data.g4
 *          |
 *          +--> schemas.g4
 *          +--> records.g4       <-- this grammar
 *          +--> collections.g4
 *          +--> streams.g4
 *          +--> serialization.g4
 *          +--> transformations.g4
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
 *          v
 *     canonical data/type representation
 *          |
 *          +--> classical representation
 *          +--> quantum::ir consumers
 *          +--> HDL/hardware representation
 *          +--> AI/data representation
 *          +--> distributed representation
 *          +--> future-domain representations
 *          |
 *          v
 *     optimization / lowering
 *          |
 *          v
 *     scheduling / resource resolution / target realization
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     - logical record declarations;
 *     - record declaration attributes/modifiers;
 *     - record names;
 *     - record generic parameters;
 *     - record where clauses;
 *     - record inheritance/composition references;
 *     - record field declarations;
 *     - record field attributes/modifiers;
 *     - record field separators;
 *     - record declaration source ordering.
 *
 * THIS FILE DOES NOT OWN:
 *
 *     - lexer rules;
 *     - keyword definitions;
 *     - identifiers;
 *     - qualified-name lexical rules;
 *     - universal type syntax;
 *     - universal expressions;
 *     - functions;
 *     - statements;
 *     - schemas;
 *     - collections;
 *     - streams;
 *     - transformations;
 *     - serialization implementations;
 *     - database implementations;
 *     - filesystem implementations;
 *     - physical memory layout;
 *     - ABI;
 *     - CPU/GPU/FPGA/QPU selection;
 *     - quantum allocation;
 *     - quantum routing;
 *     - scheduling;
 *     - optimization;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Record syntax describes logical data semantics.
 *
 * It imposes NO language-level maximum on:
 *
 *     - records;
 *     - fields;
 *     - generic parameters;
 *     - type nesting;
 *     - declaration nesting;
 *     - record size;
 *     - data size;
 *     - tensor dimensions;
 *     - distributed partitions;
 *     - nodes;
 *     - devices;
 *     - CPUs;
 *     - GPUs;
 *     - FPGAs;
 *     - QPUs;
 *     - qubits;
 *     - memory;
 *     - storage;
 *     - network capacity.
 *
 * Repetition is therefore represented with unbounded parser constructs.
 *
 * Practical resource limits are compiler/runtime policy, not language syntax.
 *
 * ============================================================================
 * HARD-CODING POLICY
 * ============================================================================
 *
 * Forbidden:
 *
 *     MAX_RECORDS
 *     MAX_FIELDS
 *     MAX_GENERIC_PARAMETERS
 *     MAX_RECORD_SIZE
 *     MAX_NESTING
 *     MAX_MEMORY
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_QPUS
 *
 * Source-level numeric values remain legal when they are actual program
 * semantics. What is prohibited is turning implementation limits into
 * universal language restrictions.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no embedded Rust;
 *     - no actions;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no runtime calls;
 *     - no hardware discovery.
 *
 * Zamani-owned Rust integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * and must contain no `unsafe`.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical parser vocabulary is:
 *
 *     ZamaniLexer
 *
 * Do not use the combined `Zamani.g4` grammar as the lexer vocabulary here.
 *
 * The repository already identifies:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * as the canonical lexer, while grammar/lexer/* provides the lexical
 * decomposition/documentation. The modular lexer files must converge into
 * that canonical lexer rather than causing parser grammars to use competing
 * vocabularies.
 *
 * ============================================================================
 * SHARED GRAMMAR CONTRACT
 * ============================================================================
 *
 * This grammar reuses:
 *
 *     ZamaniDeclarationSupport
 *     ZamaniTypeSyntax
 *
 * It does not duplicate:
 *
 *     identifier
 *     visibility
 *     attributes
 *     modifiers
 *     genericParameters
 *     whereClause
 *     typeExpression
 *
 * ============================================================================
 */

parser grammar ZamaniDataRecords;

options {
    tokenVocab = ZamaniLexer;
}

import
    ZamaniDeclarationSupport,
    ZamaniTypeSyntax;


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * `dataRecordDeclaration` is the canonical integration rule.
 *
 * grammar/data/data.g4 MUST delegate to this rule.
 *
 * No EOF is consumed here because this is a reusable parser component.
 * The compilation-unit grammar owns EOF.
 * ============================================================================
 */

dataRecordDeclaration
    : declarationAttributes?
      declarationVisibility?
      declarationModifiers?
      RECORD
      identifier
      genericParameters?
      whereClause?
      recordInheritanceClause?
      recordBody
    ;


/*
 * ============================================================================
 * RECORD INHERITANCE / COMPOSITION
 * ============================================================================
 *
 * This expresses logical type relationships.
 *
 * It does NOT define:
 *
 *     ABI
 *     physical layout
 *     memory offsets
 *     implementation inheritance
 *     hardware representation
 *
 * Those meanings belong to semantic analysis and downstream lowering.
 * ============================================================================
 */

recordInheritanceClause
    : EXTENDS
      typeExpression
      (
          COMMA
          typeExpression
      )*
    ;


/*
 * ============================================================================
 * RECORD BODY
 * ============================================================================
 *
 * Empty records are legal.
 *
 * There is no source-language field-count limit.
 * ============================================================================
 */

recordBody
    : LBRACE
      recordFieldList?
      RBRACE
    ;


/*
 * ============================================================================
 * RECORD FIELD LIST
 * ============================================================================
 *
 * Existing Zamani struct syntax uses field separators and supports trailing
 * separators. Records follow the same structural convention so that:
 *
 *     struct fields
 *
 * and:
 *
 *     record fields
 *
 * do not acquire unrelated field languages.
 *
 * Both comma and semicolon are accepted because the existing declaration
 * family already uses both styles.
 *
 * A separator is required between adjacent fields.
 * ============================================================================
 */

recordFieldList
    : recordField
      (
          recordFieldSeparator
          recordField
      )*
      recordFieldSeparator?
    ;


recordFieldSeparator
    : COMMA
    | SEMICOLON
    ;


/*
 * ============================================================================
 * RECORD FIELD
 * ============================================================================
 *
 * Canonical form:
 *
 *     fieldName: Type
 *
 * Field type syntax is delegated to the canonical type grammar.
 *
 * Field names use the canonical identifier rule.
 *
 * Attributes/modifiers use the shared declaration-support grammar.
 *
 * ============================================================================
 */

recordField
    : declarationAttributes?
      declarationVisibility?
      declarationModifiers?
      identifier
      COLON
      typeExpression
    ;


/*
 * ============================================================================
 * MULTIPLE RECORD DECLARATIONS
 * ============================================================================
 *
 * Reusable parser entry point for tooling/tests.
 *
 * No finite declaration count is encoded.
 * ============================================================================
 */

dataRecordDeclarations
    : dataRecordDeclaration+
    ;


/*
 * ============================================================================
 * SOURCE ORDER CONTRACT
 * ============================================================================
 *
 * The parse tree preserves:
 *
 *     - record declaration order;
 *     - generic parameter order;
 *     - inheritance reference order;
 *     - field order;
 *     - field source spans.
 *
 * This grammar performs no sorting, normalization or deduplication.
 *
 * Duplicate-name checking is semantic analysis.
 * ============================================================================
 */


/*
 * ============================================================================
 * DOMAIN-NEUTRAL TYPE CONTRACT
 * ============================================================================
 *
 * A record field can use any type admitted by `typeExpression`.
 *
 * Therefore this grammar can automatically participate in:
 *
 *     classical types
 *     quantum types
 *     hybrid types
 *     tensor types
 *     AI/data types
 *     resource types
 *     capability types
 *     hardware/software co-design types
 *     distributed types
 *     networking types
 *     security types
 *     future domain types
 *
 * without modifying this grammar whenever a new type is added.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * Records may contain quantum-domain values through the canonical type system.
 *
 * Example:
 *
 *     record MeasurementResult<T> {
 *         value: T,
 *         state: quantum::State<T>,
 *     }
 *
 * This grammar does NOT:
 *
 *     - allocate qubits;
 *     - enumerate physical qubits;
 *     - choose QPUs;
 *     - choose topology;
 *     - select gates;
 *     - route circuits;
 *     - schedule operations;
 *     - perform QEC;
 *     - implement ZQN;
 *     - construct quantum::ir.
 *
 * The downstream direction remains:
 *
 *     source
 *       ->
 *     domain-neutral AST
 *       ->
 *     semantic analysis
 *       ->
 *     canonical semantic representation
 *       ->
 *     quantum::ir where quantum semantics require it.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * HDL / HARDWARE CONTRACT
 * ============================================================================
 *
 * Records may carry data used by HDL/hardware/software co-design.
 *
 * They do not define:
 *
 *     wires
 *     ports
 *     registers
 *     clocks
 *     physical addresses
 *     buses
 *     device IDs
 *     topology
 *     accelerator counts
 *
 * Those are owned by grammar/hdl/, grammar/hardware/, resources/, and
 * downstream compilation.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * DISTRIBUTED CONTRACT
 * ============================================================================
 *
 * A record can describe logical distributed data.
 *
 * It does not determine:
 *
 *     partition count
 *     node count
 *     replica count
 *     storage provider
 *     network topology
 *     physical placement
 *
 * Those are semantic/resource/deployment decisions.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing establishes structure only.
 *
 * Semantic analysis determines:
 *
 *     - record-name uniqueness;
 *     - field-name uniqueness;
 *     - type validity;
 *     - generic validity;
 *     - where-clause validity;
 *     - inheritance validity;
 *     - recursive-type legality;
 *     - visibility;
 *     - ownership;
 *     - effects;
 *     - resource requirements;
 *     - capability requirements;
 *     - portability;
 *     - target compatibility.
 *
 * This grammar does none of those things.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar maps to the canonical domain-neutral frontend AST.
 *
 * Required semantic shape:
 *
 *     RecordDeclaration
 *         |
 *         +--> attributes
 *         +--> visibility
 *         +--> modifiers
 *         +--> name
 *         +--> generic parameters
 *         +--> where clause
 *         +--> inheritance references
 *         +--> ordered fields
 *
 *     RecordField
 *         |
 *         +--> attributes
 *         +--> visibility
 *         +--> modifiers
 *         +--> name
 *         +--> type
 *         +--> source span
 *
 * The grammar must not introduce a second record AST.
 *
 * Source order and source spans must be preserved.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * Records do not create a special record IR merely because their source
 * spelling is `record`.
 *
 * Semantic lowering may represent them as:
 *
 *     algebraic/product data
 *     classical aggregate
 *     data schema
 *     serialization schema
 *     distributed value
 *     accelerator data structure
 *     HDL/hardware data representation
 *
 * depending on semantic use.
 *
 * Quantum-containing records may participate in quantum lowering, but this
 * grammar never creates or modifies quantum::ir.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SERIALIZATION CONTRACT
 * ============================================================================
 *
 * Serialization syntax belongs to:
 *
 *     grammar/data/serialization.g4
 *
 * Record declarations merely provide the logical structure that serialization
 * semantics may consume.
 *
 * This grammar therefore does not define:
 *
 *     JSON
 *     CBOR
 *     binary layout
 *     wire format
 *     byte order
 *     padding
 *     physical offsets
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * MEMORY / LAYOUT CONTRACT
 * ============================================================================
 *
 * Field order is source information and must be preserved.
 *
 * This grammar does not define:
 *
 *     alignment
 *     padding
 *     packing
 *     ABI
 *     addresses
 *     cache placement
 *     register allocation
 *     accelerator memory placement
 *     NUMA placement
 *     quantum storage representation
 *
 * Those decisions occur after semantic analysis.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Syntax errors owned by this grammar include:
 *
 *     record {}
 *     record Name { field }
 *     record Name { : Type }
 *     record Name { field: }
 *     record Name { field: Type other: Type }
 *     record Name { field: Type,, other: Type }
 *
 * Semantic errors belong downstream:
 *
 *     duplicate record name
 *     duplicate field name
 *     unknown type
 *     invalid generic parameter
 *     invalid inheritance
 *     inaccessible type
 *     unsatisfied capability
 *     insufficient resources
 *     target incompatibility
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] `dataRecordDeclaration` is the sole public record declaration rule.
 * [x] `recordBody` owns record body syntax.
 * [x] `recordFieldList` owns field sequencing.
 * [x] `recordField` owns record field syntax.
 * [x] field separators have one owner.
 * [x] identifier syntax is delegated.
 * [x] attributes are delegated.
 * [x] visibility is delegated.
 * [x] declaration modifiers are delegated.
 * [x] generic parameters are delegated.
 * [x] where clauses are delegated.
 * [x] type syntax is delegated.
 * [x] no lexer rules are duplicated.
 * [x] no expressions are reimplemented.
 * [x] no second type system exists.
 * [x] no record-value expression language is hidden here.
 * [x] no record-pattern language is hidden here.
 * [x] no serialization implementation exists here.
 * [x] no database implementation exists here.
 * [x] no physical layout exists here.
 * [x] no hardware limits exist here.
 * [x] no quantum hardware logic exists here.
 * [x] no quantum IR exists here.
 * [x] no QEC/ZQN/routing/scheduling logic exists here.
 * [x] no Rust actions exist.
 * [x] no unsafe Rust is required.
 * [x] arbitrary field cardinality is supported.
 * [x] arbitrary generic cardinality is delegated.
 * [x] source ordering is preserved.
 * [x] quantum/classical/HDL/resource types can pass through typeExpression.
 *
 * ============================================================================
 */
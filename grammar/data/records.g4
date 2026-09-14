/*
 * ============================================================================
 * Zamani — Universal Data Record Grammar
 * ============================================================================
 *
 * File:
 *   grammar/data/records.g4
 *
 * Status:
 *   Production grammar component.
 *
 * Purpose:
 *   Own the syntax of logical data-record declarations in Zamani.
 *
 * Architectural role:
 *
 *   Source
 *      |
 *      v
 *   Zamani Lexer
 *      |
 *      v
 *   Zamani Parser
 *      |
 *      v
 *   records.g4
 *      |
 *      v
 *   Parse Tree / AST
 *      |
 *      v
 *   Semantic Analysis
 *      |
 *      v
 *   Canonical Data/Type Representation
 *      |
 *      +--> Classical compilation
 *      +--> Quantum/classical interoperability
 *      +--> AI/data pipelines
 *      +--> Serialization
 *      +--> Distributed execution
 *      +--> Hardware/runtime lowering
 *
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - logical data-record declarations
 *   - record names
 *   - record generic parameters
 *   - record inheritance/composition references
 *   - record fields
 *   - record field modifiers
 *   - record defaults
 *   - record-level invariants
 *   - record-level annotations
 *   - record-level logical requirements
 *   - record-level logical constraints
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - the universal type system
 *   - primitive type definitions
 *   - generic type semantics
 *   - expressions
 *   - statements
 *   - functions
 *   - modules
 *   - schemas
 *   - collections
 *   - streams
 *   - transformations
 *   - serialization implementations
 *   - databases
 *   - physical storage
 *   - physical memory layout
 *   - network protocols
 *   - hardware topology
 *   - CPU/GPU/FPGA/ASIC selection
 *   - quantum hardware
 *   - quantum IR
 *   - classical IR
 *   - scheduling
 *   - optimization
 *   - routing
 *   - runtime resource discovery
 *   - migration execution
 *
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * A record describes logical data semantics.
 *
 * It MUST NOT encode accidental assumptions about the machine executing it.
 *
 * Therefore this grammar contains no fixed:
 *
 *   - number of fields
 *   - number of records
 *   - record size
 *   - memory size
 *   - alignment size
 *   - address width
 *   - register count
 *   - CPU count
 *   - GPU count
 *   - accelerator count
 *   - node count
 *   - network size
 *   - storage capacity
 *   - serialization buffer size
 *   - quantum resource count
 *
 * Such properties belong to downstream resource, target, compilation,
 * scheduling, deployment, or runtime systems.
 *
 *
 * ============================================================================
 * INTEGRATION PRINCIPLE
 * ============================================================================
 *
 * `records.g4` is a delegated parser grammar.
 *
 * The authoritative lexer remains the Zamani lexer.
 *
 * This file MUST NOT introduce a second lexer.
 *
 * Existing canonical rules from the root grammar are reused where appropriate:
 *
 *   IDENTIFIER
 *   INTEGER
 *   annotation
 *   expression
 *   typeExpr
 *   genericParameters
 *
 * Where the repository later centralizes these constructs into dedicated
 * grammar files, those centralized rules become the authoritative providers.
 *
 *
 * ============================================================================
 * RUST
 * ============================================================================
 *
 * This grammar contains no target-language actions.
 *
 * It therefore does not require unsafe Rust or target-specific executable
 * grammar code.
 *
 * Generated parser integration MUST remain compatible with:
 *
 *   Rust 1.97
 *   Rust 1.97.1
 *
 * Zamani-owned Rust integration MUST contain no `unsafe`.
 *
 *
 * ============================================================================
 * IMPORTANT OWNERSHIP RULE
 * ============================================================================
 *
 * `record` here means a LOGICAL DATA RECORD.
 *
 * It does not mean:
 *
 *   - a CPU register
 *   - a hardware register
 *   - a quantum register
 *   - an HDL register
 *   - a database row implementation
 *   - a network packet implementation
 *   - a physical memory structure
 *
 * Those concepts belong to their respective domains.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * GRAMMAR DECLARATION
 * ============================================================================
 */

parser grammar records;

options {
    /*
     * Current repository root grammar is `grammar/Zamani.g4`, which provides
     * the shared lexer vocabulary.
     *
     * When the lexer is eventually split into a dedicated ZamaniLexer.g4,
     * this option must point to that authoritative lexer vocabulary.
     */
    tokenVocab = Zamani;
}


/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * `dataRecordDeclaration` is the canonical integration rule.
 *
 * `data.g4` MUST delegate to this rule rather than redefine record syntax.
 *
 * Do not add EOF here because this is a reusable delegated grammar rule.
 * The root compilation-unit rule owns EOF.
 * ============================================================================
 */

dataRecordDeclaration
    : recordAnnotation*
      recordVisibility?
      'record'
      recordName
      genericParameters?
      recordExtendsClause?
      recordImplementsClause*
      recordRequirementsClause*
      recordConstraintsClause*
      '{'
      dataRecordMember*
      '}'
    ;


/*
 * ============================================================================
 * RECORD NAME
 * ============================================================================
 *
 * Record identity remains symbolic and portable.
 *
 * No machine/device identity is encoded here.
 * ============================================================================
 */

recordName
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * VISIBILITY
 * ============================================================================
 *
 * Reuse the repository's canonical visibility vocabulary.
 *
 * This local rule intentionally avoids redefining semantic visibility rules.
 * ============================================================================
 */

recordVisibility
    : visibilityModifier
    ;


/*
 * ============================================================================
 * ANNOTATIONS
 * ============================================================================
 *
 * Annotation semantics are owned by the central annotation system.
 *
 * The record grammar only provides the legal attachment point.
 * ============================================================================
 */

recordAnnotation
    : annotation
    ;


/*
 * ============================================================================
 * RECORD MEMBERS
 * ============================================================================
 */

dataRecordMember
    : recordAnnotation*
      dataRecordField
    | recordAnnotation*
      dataRecordInvariant
    | recordAnnotation*
      dataRecordRequirement
    | recordAnnotation*
      dataRecordConstraint
    | recordAnnotation*
      dataRecordAttribute
    ;


/*
 * ============================================================================
 * RECORD FIELDS
 * ============================================================================
 *
 * A field consists of:
 *
 *   name
 *   type
 *   optional modifiers
 *   optional default
 *   optional constraints
 *
 * Type semantics remain owned by the canonical type system.
 *
 * There is deliberately no field-count limit.
 * ============================================================================
 */

dataRecordField
    : recordFieldModifier*
      IDENTIFIER
      ':'
      typeExpr
      recordFieldAttribute*
      recordDefaultValue?
      recordFieldConstraint*
      ';'
    ;


/*
 * ============================================================================
 * FIELD MODIFIERS
 * ============================================================================
 *
 * These modifiers describe logical record semantics.
 *
 * They do not define physical layout.
 * ============================================================================
 */

recordFieldModifier
    : 'optional'
    | 'required'
    | 'nullable'
    | 'nonnullable'
    | 'mutable'
    | 'immutable'
    | 'computed'
    | 'transient'
    | 'sensitive'
    | 'deprecated'
    ;


/*
 * ============================================================================
 * FIELD ATTRIBUTES
 * ============================================================================
 *
 * Attributes are deliberately extensible.
 *
 * Backend-specific interpretation must happen after parsing.
 * ============================================================================
 */

recordFieldAttribute
    : 'tag' INTEGER
    | 'default' expression
    | 'generated' 'by' expression
    | 'property' IDENTIFIER
    | 'property' IDENTIFIER '=' expression
    | annotation
    ;


/*
 * ============================================================================
 * DEFAULT VALUES
 * ============================================================================
 */

recordDefaultValue
    : '=' expression
    ;


/*
 * ============================================================================
 * FIELD CONSTRAINTS
 * ============================================================================
 *
 * Constraints are semantic expressions.
 *
 * The grammar does not decide whether a constraint is enforced:
 *
 *   statically
 *   dynamically
 *   at compile time
 *   at runtime
 *   by hardware
 *   by a backend
 *
 * That decision belongs to semantic analysis and downstream compilation.
 * ============================================================================
 */

recordFieldConstraint
    : 'where'
      expression
    ;


/*
 * ============================================================================
 * RECORD INVARIANTS
 * ============================================================================
 *
 * An invariant applies to the logical record as a whole.
 * ============================================================================
 */

dataRecordInvariant
    : 'invariant'
      '('
      expression
      ')'
      ';'
    ;


/*
 * ============================================================================
 * RECORD REQUIREMENTS
 * ============================================================================
 *
 * Requirements express semantic conditions.
 *
 * They are intentionally separate from:
 *
 *   constraints
 *   preferences
 *   hints
 *   physical resource selections
 *
 * Examples of valid downstream meanings could include:
 *
 *   requires a capability
 *   requires a property
 *   requires a representation
 *
 * The actual capability/resource model owns interpretation.
 * ============================================================================
 */

dataRecordRequirement
    : 'requires'
      recordRequirementExpression
      ';'
    ;

recordRequirementExpression
    : expression
    ;


/*
 * ============================================================================
 * RECORD CONSTRAINTS
 * ============================================================================
 */

dataRecordConstraint
    : 'constraint'
      IDENTIFIER?
      '('
      expression
      ')'
      ';'
    ;


/*
 * ============================================================================
 * RECORD ATTRIBUTES
 * ============================================================================
 *
 * Open-ended attributes prevent this grammar from becoming a closed list of
 * future concepts.
 *
 * Semantic validation determines which properties are recognized.
 * ============================================================================
 */

dataRecordAttribute
    : 'attribute'
      IDENTIFIER
      recordAttributeValue?
      ';'
    ;

recordAttributeValue
    : '=' expression
    ;


/*
 * ============================================================================
 * RECORD INHERITANCE
 * ============================================================================
 *
 * A record may compose or extend logical record contracts.
 *
 * There is deliberately no limit on the number of referenced parents.
 *
 * Semantic analysis determines whether a particular inheritance graph is
 * valid.
 * ============================================================================
 */

recordExtendsClause
    : 'extends'
      recordTypeReference
      (
          ','
          recordTypeReference
      )*
    ;


/*
 * ============================================================================
 * RECORD IMPLEMENTATION / CONTRACTS
 * ============================================================================
 *
 * `implements` is a logical contract relationship.
 *
 * It does not imply a programming-language ABI or physical implementation.
 * ============================================================================
 */

recordImplementsClause
    : 'implements'
      recordTypeReference
      (
          ','
          recordTypeReference
      )*
    ;


/*
 * ============================================================================
 * RECORD TYPE REFERENCES
 * ============================================================================
 *
 * This grammar MUST NOT duplicate the canonical type grammar.
 *
 * It therefore accepts qualified type references and generic arguments using
 * the canonical type-expression grammar where possible.
 *
 * The current root grammar already owns `typeExpr`.
 *
 * This adapter exists so that record inheritance/contract references remain
 * syntactically explicit without introducing another type system.
 * ============================================================================
 */

recordTypeReference
    : IDENTIFIER
      (
          '::'
          IDENTIFIER
      )*
      recordTypeArguments?
    ;

recordTypeArguments
    : '<'
      recordTypeArgument
      (
          ','
          recordTypeArgument
      )*
      '>'
    ;

recordTypeArgument
    : typeExpr
    | expression
    ;


/*
 * ============================================================================
 * RECORD REQUIREMENT / CAPABILITY BLOCKS
 * ============================================================================
 *
 * These are logical declarations only.
 *
 * They MUST NOT encode:
 *
 *   device IDs
 *   fixed machine sizes
 *   physical addresses
 *   topology
 *   hardware vendor selection
 *
 * Those concepts belong to the capability/resource/target layers.
 * ============================================================================
 */

recordRequirementsClause
    : 'requires'
      '{'
      recordRequirementEntry*
      '}'
    ;

recordRequirementEntry
    : IDENTIFIER
      (
          '='
          expression
      )?
      ';'
    ;


/*
 * ============================================================================
 * RECORD CONSTRAINT BLOCKS
 * ============================================================================
 */

recordConstraintsClause
    : 'constraints'
      '{'
      recordConstraintEntry*
      '}'
    ;

recordConstraintEntry
    : IDENTIFIER?
      expression
      ';'
    ;


/*
 * ============================================================================
 * RECORD FIELD INITIALIZATION
 * ============================================================================
 *
 * This rule is intentionally separate from the general expression grammar.
 *
 * `data.g4` may integrate it into a record-value expression without making
 * records responsible for the entire expression system.
 * ============================================================================
 */

recordFieldInitializer
    : IDENTIFIER
      ':'
      expression
    ;

recordFieldInitializerList
    : recordFieldInitializer
      (
          ','
          recordFieldInitializer
      )*
    ;


/*
 * ============================================================================
 * RECORD VALUE
 * ============================================================================
 *
 * A record value is explicitly introduced by `record` to prevent accidental
 * ambiguity with:
 *
 *   blocks
 *   maps
 *   object literals
 *   schema literals
 *   hardware descriptions
 *
 * Examples:
 *
 *   record User {
 *       name: "Samuel",
 *       active: true
 *   }
 *
 *   record package::User {
 *       name: "Samuel"
 *   }
 *
 * The expression system decides where this rule is admitted.
 * ============================================================================
 */

recordValue
    : 'record'
      recordTypeReference?
      '{'
      recordFieldInitializerList?
      '}'
    ;


/*
 * ============================================================================
 * RECORD UPDATE VALUE
 * ============================================================================
 *
 * Supports immutable-style construction without requiring a physical memory
 * model.
 *
 * Example:
 *
 *   record existing with {
 *       name: "new-name"
 *   }
 *
 * The semantic layer decides whether this becomes:
 *
 *   copy
 *   persistent update
 *   structural sharing
 *   mutation
 *   distributed update
 *   hardware-specific operation
 * ============================================================================
 */

recordUpdateValue
    : 'record'
      recordTypeReference
      'from'
      expression
      'with'
      '{'
      recordFieldInitializerList?
      '}'
    ;


/*
 * ============================================================================
 * RECORD PATTERN
 * ============================================================================
 *
 * Record patterns belong to pattern matching semantically but the record
 * grammar owns the record-specific structural syntax.
 *
 * The statement/pattern grammar can integrate this rule.
 * ============================================================================
 */

recordPattern
    : recordTypeReference?
      '{'
      recordPatternFieldList?
      '}'
    ;

recordPatternFieldList
    : recordPatternField
      (
          ','
          recordPatternField
      )*
    ;

recordPatternField
    : IDENTIFIER
      (
          ':'
          pattern
      )?
    ;


/*
 * ============================================================================
 * RECORD FIELD PATH
 * ============================================================================
 *
 * Supports arbitrarily deep logical field paths without imposing a nesting
 * limit.
 *
 * Physical memory offsets are deliberately NOT represented here.
 * ============================================================================
 */

recordFieldPath
    : IDENTIFIER
      (
          '.'
          IDENTIFIER
      )*
    ;


/*
 * ============================================================================
 * RECORD SELECTION
 * ============================================================================
 *
 * This is a syntactic hook for downstream data expressions.
 *
 * It does not define query execution.
 * ============================================================================
 */

recordFieldSelection
    : expression
      '.'
      IDENTIFIER
    ;


/*
 * ============================================================================
 * RECORD TYPE CONVERSION
 * ============================================================================
 *
 * Conversion semantics belong to type checking / semantic analysis.
 * ============================================================================
 */

recordConversionExpression
    : 'record_cast'
      '('
      expression
      'as'
      typeExpr
      ')'
    ;


/*
 * ============================================================================
 * RECORD COMPOSITION
 * ============================================================================
 *
 * Logical record composition remains independent from physical layout.
 * ============================================================================
 */

recordCompositionExpression
    : 'record_merge'
      '('
      expression
      ','
      expression
      (
          ','
          expression
      )*
      ')'
    ;


/*
 * ============================================================================
 * RECORD PROJECTION
 * ============================================================================
 *
 * Projection is useful to downstream data processing without coupling this
 * grammar to a database/query language.
 * ============================================================================
 */

recordProjectionExpression
    : 'record_project'
      '('
      expression
      'select'
      recordProjectionList
      ')'
    ;

recordProjectionList
    : recordProjection
      (
          ','
          recordProjection
      )*
    ;

recordProjection
    : recordFieldPath
      (
          'as'
          IDENTIFIER
      )?
    ;


/*
 * ============================================================================
 * RECORD UPDATE EXPRESSION
 * ============================================================================
 */

recordUpdateExpression
    : 'record_update'
      '('
      expression
      ','
      '{'
      recordFieldInitializerList?
      '}'
      ')'
    ;


/*
 * ============================================================================
 * RECORD EQUALITY / IDENTITY HOOKS
 * ============================================================================
 *
 * These are syntax hooks only.
 *
 * Semantic analysis determines whether equality means:
 *
 *   structural equality
 *   nominal equality
 *   identity equality
 *   logical equality
 *   application-defined equality
 * ============================================================================
 */

recordEqualityExpression
    : 'record_equal'
      '('
      expression
      ','
      expression
      ')'
    ;


/*
 * ============================================================================
 * RECORD METADATA
 * ============================================================================
 *
 * Metadata is deliberately open-ended.
 *
 * The semantic metadata system decides what a particular metadata key means.
 * ============================================================================
 */

recordMetadata
    : 'metadata'
      '{'
      recordMetadataEntry*
      '}'
    ;

recordMetadataEntry
    : IDENTIFIER
      (
          '='
          expression
      )?
      ';'
    ;


/*
 * ============================================================================
 * RECORD SERIALIZATION INTENT
 * ============================================================================
 *
 * This is NOT a serialization implementation.
 *
 * It merely allows a record to express a logical serialization requirement.
 *
 * Concrete formats and codecs belong to interoperability/serialization
 * subsystems.
 * ============================================================================
 */

recordSerializationIntent
    : 'serialize'
      recordSerializationTarget?
      recordSerializationOptions?
      ';'
    ;

recordSerializationTarget
    : 'as'
      recordTypeReference
    ;

recordSerializationOptions
    : 'with'
      '{'
      recordSerializationOption*
      '}'
    ;

recordSerializationOption
    : IDENTIFIER
      (
          '='
          expression
      )?
      ';'
    ;


/*
 * ============================================================================
 * RECORD COMPATIBILITY INTENT
 * ============================================================================
 *
 * Compatibility policy belongs to semantic compatibility analysis.
 *
 * This grammar merely provides an extensible syntactic declaration.
 * ============================================================================
 */

recordCompatibility
    : 'compatibility'
      recordCompatibilityPolicy
      ';'
    ;

recordCompatibilityPolicy
    : recordTypeReference
      (
          '('
          argumentList?
          ')'
      )?
    ;


/*
 * ============================================================================
 * RECORD DIALECT EXTENSION
 * ============================================================================
 *
 * Future domains may add record semantics through dialects without changing
 * the core record model.
 *
 * The dialect system owns registration and semantic validation.
 * ============================================================================
 */

recordDialectExtension
    : 'dialect'
      recordTypeReference
      (
          '('
          argumentList?
          ')'
      )?
      ';'
    ;
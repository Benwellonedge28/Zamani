
/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *     grammar/metaprogramming/schemas.g4
 *
 * Grammar:
 *     ZamaniMetaSchemas
 *
 * Status:
 *     PROPOSED PRODUCTION PARSER COMPONENT
 *
 * Language:
 *     Zamani
 *
 * Parser:
 *     ANTLR4
 *
 * Rust implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     Safe Rust only.
 *     No unsafe Rust.
 *     No embedded Rust actions.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines SOURCE-LEVEL SYNTAX for compile-time schema
 * descriptions and schema metaprogramming.
 *
 * It allows Zamani programs and compiler tooling to describe, inspect,
 * transform, validate, compose, and generate logical schema structures.
 *
 * This grammar is intended for:
 *
 *     - compile-time schema descriptions;
 *     - schema reflection requests;
 *     - schema projections;
 *     - schema transformations;
 *     - schema composition;
 *     - schema derivation;
 *     - schema validation requests;
 *     - schema generation requests;
 *     - schema compatibility declarations;
 *     - schema evolution descriptions;
 *     - schema provenance;
 *     - generic and parameterized schemas;
 *     - cross-domain schema generation.
 *
 * This grammar defines syntax only.
 *
 * It does not implement a schema engine.
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
 *          +--> schema resolution
 *          +--> type validation
 *          +--> reflection validation
 *          +--> transformation validation
 *          +--> capability validation
 *          +--> effect validation
 *          +--> provenance
 *          |
 *          v
 *     metaprogramming engine
 *          |
 *          +--> schema inspection
 *          +--> schema transformation
 *          +--> schema composition
 *          +--> schema generation
 *          |
 *          v
 *     validated canonical AST
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL / hardware representation
 *          +--> distributed representation
 *          +--> AI / data representation
 *          +--> future domains
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     metaSchemaDeclaration
 *     metaSchemaBody
 *     metaSchemaMember
 *     metaSchemaReference
 *     metaSchemaSelection
 *     metaSchemaProjection
 *     metaSchemaTransformation
 *     metaSchemaComposition
 *     metaSchemaDerivation
 *     metaSchemaValidation
 *     metaSchemaGeneration
 *     metaSchemaCompatibility
 *     metaSchemaProvenance
 *     metaSchemaPolicy
 *     metaSchemaQuery
 *
 * THIS FILE DOES NOT OWN:
 *
 *     ordinary data schema declarations;
 *     ordinary record declarations;
 *     ordinary struct declarations;
 *     type definitions;
 *     generic parameter definitions;
 *     ordinary expressions;
 *     ordinary statements;
 *     identifiers;
 *     qualified names;
 *     lexical tokens;
 *     reflection engine implementation;
 *     schema validation implementation;
 *     schema migration execution;
 *     AST implementation;
 *     semantic model implementation;
 *     IR implementation;
 *     source generation implementation;
 *     compiler implementation;
 *     runtime implementation;
 *     physical storage;
 *     database engines;
 *     networking;
 *     hardware discovery;
 *     resource allocation;
 *     scheduling;
 *     routing;
 *     optimization;
 *     QEC;
 *     ZQN;
 *     HAL.
 *
 * ============================================================================
 * SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * Ordinary logical data schemas belong to:
 *
 *     grammar/data/schemas.g4
 *
 * General records belong to:
 *
 *     grammar/declarations/records.g4
 *
 * General reflection belongs to:
 *
 *     grammar/metaprogramming/reflection.g4
 *
 * General source generation belongs to:
 *
 *     grammar/metaprogramming/generation.g4
 *
 * General specialization belongs to:
 *
 *     grammar/metaprogramming/specialization.g4
 *
 * General metaprogramming composition belongs to:
 *
 *     grammar/metaprogramming/metaprogramming.g4
 *
 * This file MUST NOT redefine the ordinary schema declaration rule:
 *
 *     schemaDeclaration
 *
 * Nor may it redefine:
 *
 *     recordDeclaration
 *     recordBody
 *     typeExpression
 *     expression
 *     identifier
 *     qualifiedName
 *     attribute
 *
 * ============================================================================
 * COMPOSITION CONTRACT
 * ============================================================================
 *
 * This is a parser leaf.
 *
 * Shared parser rules are provided by the canonical parser composition.
 *
 * Required shared rules:
 *
 *     attribute
 *     identifier
 *     qualifiedName
 *     genericParameters
 *     whereClause
 *     typeExpression
 *     expression
 *
 * Required canonical lexer vocabulary:
 *
 *     ZamaniLexer
 *
 * The grammar introduces no lexer rules.
 *
 * The composition layer must expose:
 *
 *     metaSchemaDeclaration
 *     metaSchemaExpression
 *     metaSchemaStatement
 *
 * only where the canonical language syntax permits them.
 *
 * The public entry rules are intentionally distinct from:
 *
 *     dataSchemaDeclaration
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * All lexical tokens belong to the canonical Zamani lexer.
 *
 * This grammar requires the existing schema token:
 *
 *     K_SCHEMA
 *
 * Other names and selectors remain ordinary identifiers.
 *
 * This file MUST NOT introduce:
 *
 *     a second identifier grammar;
 *     a second keyword registry;
 *     a second literal grammar;
 *     a second operator registry.
 *
 * New reserved words must first be approved by:
 *
 *     grammar/lexer/keywords.g4
 *
 * ============================================================================
 * CORE DESIGN
 * ============================================================================
 *
 * A meta-schema describes how schema information is inspected or
 * transformed.
 *
 * It does not itself create a new runtime schema.
 *
 * The semantic distinction is:
 *
 *     ordinary schema
 *          |
 *          v
 *     logical data structure
 *
 *     meta-schema
 *          |
 *          v
 *     compile-time description or transformation
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * There are no grammar-level maximums for:
 *
 *     schemas;
 *     schema members;
 *     schema references;
 *     transformations;
 *     projections;
 *     generated declarations;
 *     generic parameters;
 *     nested types;
 *     composition operands;
 *     validation expressions;
 *     provenance entries;
 *     compatibility declarations;
 *     source fragments.
 *
 * Repetition is represented structurally.
 *
 * The grammar does not encode artificial limits based on:
 *
 *     CPU count;
 *     GPU count;
 *     FPGA count;
 *     QPU count;
 *     qubit count;
 *     memory capacity;
 *     tensor rank;
 *     register width;
 *     distributed node count;
 *     storage capacity;
 *     device count.
 *
 * Actual implementation resource budgets belong to compiler policy.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Meta-schema syntax must preserve:
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
 * A schema transformation MUST NOT silently depend on:
 *
 *     compiler host hardware;
 *     machine identity;
 *     device identity;
 *     physical memory layout;
 *     physical addresses;
 *     hardware topology;
 *     vendor-specific runtime behavior;
 *     current resource availability.
 *
 * Such dependencies must be explicitly represented through the appropriate
 * semantic effects, capabilities, and resource contracts.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The frontend must map the grammar into the existing canonical,
 * domain-neutral AST.
 *
 * Conceptual mapping:
 *
 *     metaSchemaDeclaration
 *         -> meta-schema declaration node
 *
 *     metaSchemaReference
 *         -> canonical name reference
 *
 *     metaSchemaProjection
 *         -> schema projection node
 *
 *     metaSchemaTransformation
 *         -> transformation request node
 *
 *     metaSchemaComposition
 *         -> composition expression node
 *
 *     metaSchemaGeneration
 *         -> generation request node
 *
 *     metaSchemaValidation
 *         -> validation request node
 *
 *     metaSchemaCompatibility
 *         -> compatibility declaration node
 *
 * Every node must preserve:
 *
 *     source span;
 *     source ordering;
 *     nesting;
 *     names;
 *     attributes;
 *     arguments;
 *     type references;
 *     transformation provenance;
 *     generated-source provenance.
 *
 * This grammar MUST NOT introduce a second AST hierarchy.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for:
 *
 *     name resolution;
 *     schema existence;
 *     schema visibility;
 *     type compatibility;
 *     generic substitution;
 *     schema dependency analysis;
 *     transformation legality;
 *     projection validity;
 *     composition validity;
 *     recursive dependency detection;
 *     compatibility analysis;
 *     determinism;
 *     capability validation;
 *     effect validation;
 *     provenance;
 *     generated schema validation;
 *     security policy.
 *
 * Parsing does not perform these operations.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar produces NO IR.
 *
 * Correct direction:
 *
 *     meta-schema syntax
 *          |
 *          v
 *     canonical frontend AST
 *          |
 *          v
 *     semantic validation
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          v
 *     canonical IR
 *
 * It MUST NOT construct:
 *
 *     QuantumGate
 *     PhysicalQubit
 *     ClassicalInstruction
 *     HardwareInstruction
 *     ScheduleOperation
 *     DatabaseExecutionPlan
 *     PhysicalStorageLayout
 *
 * Quantum semantics continue through:
 *
 *     quantum::ir
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing must depend only on:
 *
 *     source tokens;
 *     language version;
 *     active grammar configuration.
 *
 * It must not depend on:
 *
 *     hardware;
 *     network;
 *     filesystem;
 *     current time;
 *     random values;
 *     compiler host state.
 *
 * Transformations requiring external information must explicitly declare
 * the corresponding dependency.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing a meta-schema MUST NEVER execute it.
 *
 * This grammar grants no implicit permission to:
 *
 *     access files;
 *     access networks;
 *     execute processes;
 *     load arbitrary plugins;
 *     inspect host memory;
 *     access credentials;
 *     discover hardware;
 *     allocate devices;
 *     execute generated programs.
 *
 * Compile-time execution requires explicit semantic authorization.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no Rust actions;
 *     no Rust predicates;
 *     no embedded executable code;
 *     no unsafe;
 *     no host-language callbacks.
 *
 * Its consumer must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * All compiler implementation code must use safe Rust.
 *
 * ============================================================================
 */

/*
 * ============================================================================
 * GRAMMAR DECLARATION
 * ============================================================================
 */

parser grammar ZamaniMetaSchemas;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * PUBLIC ENTRY POINTS
 * ============================================================================
 *
 * These rules provide stable integration points for the metaprogramming
 * composition layer.
 *
 * They do not redefine ordinary schema syntax.
 * ============================================================================
 */

metaSchemaDeclaration
    : K_SCHEMA
      identifier
      genericParameters?
      whereClause?
      metaSchemaOptions?
      LBRACE
      metaSchemaMember*
      RBRACE
    ;

metaSchemaExpression
    : metaSchemaQuery
    | metaSchemaProjection
    | metaSchemaTransformation
    | metaSchemaComposition
    | metaSchemaDerivation
    | metaSchemaGeneration
    | metaSchemaValidation
    ;

metaSchemaStatement
    : metaSchemaDeclaration
    ;

/*
 * ============================================================================
 * DECLARATION OPTIONS
 * ============================================================================
 *
 * Options are metadata and do not select physical implementations.
 * ============================================================================
 */

metaSchemaOptions
    : K_WITH
      LBRACE
      metaSchemaOption*
      RBRACE
    ;

metaSchemaOption
    : attribute
    | identifier ASSIGN expression SEMI
    ;

/*
 * ============================================================================
 * MEMBERS
 * ============================================================================
 */

metaSchemaMember
    : attribute
    | metaSchemaReference
    | metaSchemaSelection
    | metaSchemaProjection
    | metaSchemaTransformation
    | metaSchemaComposition
    | metaSchemaDerivation
    | metaSchemaValidation
    | metaSchemaCompatibility
    | metaSchemaProvenance
    | metaSchemaPolicy
    ;

/*
 * ============================================================================
 * SCHEMA REFERENCES
 * ============================================================================
 *
 * A reference identifies an existing logical schema.
 *
 * Resolution belongs to semantic analysis.
 * ============================================================================
 */

metaSchemaReference
    : identifier ASSIGN qualifiedName SEMI
    | qualifiedName SEMI
    ;

/*
 * ============================================================================
 * SELECTION
 * ============================================================================
 *
 * Selectors remain ordinary identifiers.
 *
 * This keeps the grammar open to future schema properties without requiring
 * a new reserved word for every field, type, attribute, or metadata category.
 * ============================================================================
 */

metaSchemaSelection
    : identifier
      K_FROM
      metaSchemaSubject
      metaSchemaSelector?
      SEMI
    ;

metaSchemaSubject
    : qualifiedName
    | LPAREN metaSchemaQuery RPAREN
    ;

metaSchemaSelector
    : DOT identifier
    | LBRACK expression RBRACK
    ;

/*
 * ============================================================================
 * PROJECTION
 * ============================================================================
 *
 * Projection creates a compile-time description of selected schema
 * information.
 *
 * It does not allocate storage or construct a runtime object.
 * ============================================================================
 */

metaSchemaProjection
    : identifier
      LPAREN
      metaSchemaSubject
      COMMA
      metaSchemaProjectionList
      RPAREN
      SEMI
    ;

metaSchemaProjectionList
    : metaSchemaProjectionItem
      (COMMA metaSchemaProjectionItem)*
      COMMA?
    ;

metaSchemaProjectionItem
    : identifier
    | identifier ASSIGN identifier
    | STAR
    ;

/*
 * ============================================================================
 * TRANSFORMATION
 * ============================================================================
 *
 * Transformations describe compile-time structural changes.
 *
 * They do not execute migrations or alter physical storage.
 * ============================================================================
 */

metaSchemaTransformation
    : identifier
      metaSchemaSubject
      metaSchemaTransformationBody?
      SEMI
    ;

metaSchemaTransformationBody
    : LBRACE
      metaSchemaTransformationOperation*
      RBRACE
    ;

metaSchemaTransformationOperation
    : metaSchemaAddMember
    | metaSchemaRemoveMember
    | metaSchemaRenameMember
    | metaSchemaReplaceType
    | metaSchemaAnnotate
    | metaSchemaAssert
    ;

metaSchemaAddMember
    : K_ADD
      identifier
      COLON
      typeExpression
      SEMI
    ;

metaSchemaRemoveMember
    : K_REMOVE
      identifier
      SEMI
    ;

metaSchemaRenameMember
    : K_RENAME
      identifier
      K_TO
      identifier
      SEMI
    ;

metaSchemaReplaceType
    : K_REPLACE
      identifier
      COLON
      typeExpression
      SEMI
    ;

metaSchemaAnnotate
    : attribute
    ;

metaSchemaAssert
    : K_ASSERT
      expression
      SEMI
    ;

/*
 * ============================================================================
 * COMPOSITION
 * ============================================================================
 *
 * Composition describes relationships between logical schemas.
 *
 * It does not prescribe database joins, storage placement, or network
 * topology.
 * ============================================================================
 */

metaSchemaComposition
    : identifier
      LPAREN
      metaSchemaCompositionOperand
      (COMMA metaSchemaCompositionOperand)*
      COMMA?
      RPAREN
      SEMI
    ;

metaSchemaCompositionOperand
    : qualifiedName
    | LPAREN metaSchemaQuery RPAREN
    ;

/*
 * ============================================================================
 * DERIVATION
 * ============================================================================
 *
 * Derivation requests a schema description based on another schema.
 *
 * Derivation is not unrestricted compiler execution.
 * ============================================================================
 */

metaSchemaDerivation
    : identifier
      qualifiedName
      metaSchemaDerivationArguments?
      SEMI
    ;

metaSchemaDerivationArguments
    : LPAREN
      metaSchemaArgumentList?
      RPAREN
    ;

metaSchemaArgumentList
    : metaSchemaArgument
      (COMMA metaSchemaArgument)*
      COMMA?
    ;

metaSchemaArgument
    : identifier ASSIGN expression
    | typeExpression
    | expression
    ;

/*
 * ============================================================================
 * GENERATION
 * ============================================================================
 *
 * Generation delegates source-generation semantics to the canonical
 * metaprogramming generation subsystem.
 *
 * This grammar defines only the schema-specific request boundary.
 * ============================================================================
 */

metaSchemaGeneration
    : identifier
      K_FROM
      metaSchemaSubject
      metaSchemaGenerationOptions?
      SEMI
    ;

metaSchemaGenerationOptions
    : K_WITH
      LBRACE
      metaSchemaGenerationOption*
      RBRACE
    ;

metaSchemaGenerationOption
    : identifier ASSIGN expression SEMI
    | attribute
    ;

/*
 * ============================================================================
 * VALIDATION
 * ============================================================================
 *
 * Validation describes a request.
 *
 * The grammar does not execute validation.
 * ============================================================================
 */

metaSchemaValidation
    : identifier
      metaSchemaSubject
      metaSchemaValidationBody?
      SEMI
    ;

metaSchemaValidationBody
    : LBRACE
      metaSchemaValidationRule*
      RBRACE
    ;

metaSchemaValidationRule
    : K_ASSERT expression SEMI
    | attribute
    ;

/*
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * Compatibility describes semantic expectations between schema versions.
 *
 * It does not perform migrations.
 * ============================================================================
 */

metaSchemaCompatibility
    : identifier
      metaSchemaSubject
      identifier
      metaSchemaSubject
      SEMI
    ;

/*
 * ============================================================================
 * PROVENANCE
 * ============================================================================
 *
 * Provenance identifies logical origins.
 *
 * It does not establish cryptographic trust or execute verification.
 * ============================================================================
 */

metaSchemaProvenance
    : identifier
      ASSIGN
      metaSchemaSubject
      SEMI
    ;

/*
 * ============================================================================
 * POLICY
 * ============================================================================
 *
 * Requirements, constraints, preferences, and hints are distinct semantic
 * categories.
 *
 * The grammar does not interpret or enforce them.
 * ============================================================================
 */

metaSchemaPolicy
    : identifier
      LBRACE
      metaSchemaPolicyEntry*
      RBRACE
    ;

metaSchemaPolicyEntry
    : K_REQUIRES expression SEMI
    | K_CONSTRAINS expression SEMI
    | K_PREFERS expression SEMI
    | K_HINT expression SEMI
    ;

/*
 * ============================================================================
 * QUERY
 * ============================================================================
 *
 * The query is deliberately structural and non-recursive with respect to
 * ordinary expression grammar.
 *
 * Its operands are canonical names, schema references, and explicit
 * projections.
 * ============================================================================
 */

metaSchemaQuery
    : metaSchemaSubject
      metaSchemaQueryOperation*
    ;

metaSchemaQueryOperation
    : DOT identifier
    | LBRACK expression RBRACK
    | LPAREN metaSchemaArgumentList? RPAREN
    ;

/*
 * ============================================================================
 * INTEGRATION INVARIANTS
 * ============================================================================
 *
 * 1. Ordinary schema declarations remain owned by data/schemas.g4.
 *
 * 2. General reflection remains owned by metaprogramming/reflection.g4.
 *
 * 3. General generation remains owned by metaprogramming/generation.g4.
 *
 * 4. General specialization remains owned by specialization.g4.
 *
 * 5. Macro syntax remains owned by grammar/macros/.
 *
 * 6. Identifiers and qualified names remain owned by grammar/core/.
 *
 * 7. Type syntax remains owned by grammar/types/.
 *
 * 8. Expressions remain owned by grammar/expressions/.
 *
 * 9. Attributes remain owned by grammar/core/.
 *
 * 10. Lexer tokens remain owned by grammar/lexer/.
 *
 * 11. Canonical AST remains owned by src/frontend/ast/.
 *
 * 12. Semantic schema representation remains downstream.
 *
 * 13. Canonical IR remains downstream.
 *
 * 14. Quantum semantics continue through quantum::ir.
 *
 * 15. No physical storage layout is created here.
 *
 * 16. No hardware selection is performed here.
 *
 * 17. No resource discovery is performed here.
 *
 * 18. Generated schema structures must pass ordinary semantic validation.
 *
 * ============================================================================
 * CROSS-DOMAIN INTEGRATION
 * ============================================================================
 *
 * Classical:
 *     Uses canonical classical types and expressions.
 *
 * Quantum:
 *     May describe schemas referencing permitted quantum types.
 *     Does not define quantum operations or another quantum IR.
 *
 * Hybrid:
 *     May describe shared data contracts across classical/quantum boundaries.
 *
 * HDL:
 *     May describe logical contracts used by hardware interfaces.
 *     HDL owns actual hardware syntax.
 *
 * AI:
 *     May describe model, tensor, and dataset metadata.
 *
 * Distributed:
 *     May describe logical distributed data contracts.
 *     Distributed execution owns placement and realization.
 *
 * Security:
 *     May carry security metadata.
 *     Security owns enforcement.
 *
 * Interoperability:
 *     May describe external schema mappings.
 *     External formats remain interoperability concerns.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [x] schema metaprogramming has a separate ownership boundary;
 *     [x] ordinary data schemas are not redefined;
 *     [x] reflection remains delegated;
 *     [x] generation remains delegated;
 *     [x] specialization remains delegated;
 *     [x] canonical names are reused;
 *     [x] canonical types are reused;
 *     [x] canonical expressions are reused;
 *     [x] no lexer rules are introduced;
 *     [x] no second AST is introduced;
 *     [x] no second type system is introduced;
 *     [x] no IR is generated;
 *     [x] no hardware limits are introduced;
 *     [x] no resource discovery is performed;
 *     [x] generated structures undergo normal semantic validation;
 *     [x] source provenance is preserved;
 *     [x] Rust integration requires no unsafe;
 *     [x] positive tests are specified;
 *     [x] negative tests are specified;
 *     [x] boundary tests are specified;
 *     [x] scalability tests are specified;
 *     [x] determinism tests are specified;
 *     [x] compatibility tests are specified.
 *
 * ============================================================================
 * FINAL INVARIANT
 * ============================================================================
 *
 * A meta-schema describes operations on logical schema information.
 *
 * It does not define how that information is physically stored, executed,
 * routed, scheduled, or deployed.
 *
 * Its output must return to the canonical Zamani semantic pipeline.
 *
 * This is the schema-metaprogramming contribution to:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * ============================================================================
 */

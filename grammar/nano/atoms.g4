
/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/nano/atoms.g4
 *
 * GRAMMAR
 * -------
 * NanoAtoms
 *
 * STATUS
 * ------
 * PROPOSED CANONICAL NANO-ATOM DOMAIN GRAMMAR
 *
 * LANGUAGE
 * --------
 * Zamani
 *
 * COMPILER BASELINE
 * -----------------
 * Rust 1.97 / Rust 1.97.1
 * Rust 2021
 * Safe Rust only
 * No unsafe Rust
 *
 * ============================================================================
 * 1. PURPOSE
 * ============================================================================
 *
 * This grammar owns the SOURCE-LEVEL STRUCTURE of atomic entities used by
 * nano-oriented Zamani programs.
 *
 * It supports:
 *
 *   - atom declarations;
 *   - named atom definitions;
 *   - parameterized atom declarations;
 *   - atom type specifications;
 *   - atom composition;
 *   - atom properties;
 *   - atom attributes;
 *   - atom-local requirements;
 *   - atom-local constraints;
 *   - atom-local capabilities;
 *   - atom-local preferences;
 *   - atom-local expressions;
 *   - references to other atoms;
 *   - open-ended atomic metadata;
 *   - nested atomic declarations where permitted by the enclosing grammar.
 *
 * This grammar describes source-level computational and structural intent.
 *
 * It does NOT implement physical chemistry, quantum mechanics, atomic
 * simulation, physical material discovery, or hardware execution.
 *
 * ============================================================================
 * 2. ARCHITECTURAL POSITION
 * ============================================================================
 *
 * Zamani source
 *      |
 *      v
 * ZamaniLexer
 *      |
 *      v
 * ZamaniParser
 *      |
 *      v
 * Nano domain dispatcher
 *      |
 *      v
 * NanoAtoms
 *      |
 *      v
 * Domain-neutral frontend AST
 *      |
 *      v
 * Structural validation
 *      |
 *      v
 * Semantic analysis
 *      |
 *      +--> name resolution
 *      +--> type checking
 *      +--> property validation
 *      +--> capability analysis
 *      +--> resource analysis
 *      +--> domain-specific validation
 *      |
 *      v
 * Canonical semantic representation
 *      |
 *      +--> classical semantics
 *      +--> quantum semantics
 *      +--> nano/material semantics
 *      +--> hardware/software co-design
 *      |
 *      v
 * Canonical IR
 *      |
 *      v
 * Optimization / simulation / lowering
 *      |
 *      v
 * Target realization
 *
 * Quantum-related atomic computation MUST use the existing canonical
 * quantum::ir boundary.
 *
 * This grammar MUST NOT create another quantum IR.
 *
 * ============================================================================
 * 3. OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   nanoAtomDeclaration
 *   nanoAtomAnnotation
 *   nanoAtomDeclarationBody
 *   nanoAtomMember
 *   nanoAtomTypeClause
 *   nanoAtomParameterClause
 *   nanoAtomParameterList
 *   nanoAtomParameter
 *   nanoAtomProperty
 *   nanoAtomPropertyValue
 *   nanoAtomComposition
 *   nanoAtomReference
 *   nanoAtomRequirement
 *   nanoAtomConstraint
 *   nanoAtomCapability
 *   nanoAtomPreference
 *   nanoAtomMetadata
 *   nanoAtomExpression
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexical definitions;
 *   - identifiers;
 *   - general qualified names;
 *   - general expression precedence;
 *   - general types;
 *   - generic declaration dispatch;
 *   - general statements;
 *   - agent declarations;
 *   - molecular declarations;
 *   - material declarations;
 *   - physical chemistry;
 *   - atomic number tables;
 *   - periodic-table implementations;
 *   - isotope databases;
 *   - electron configuration algorithms;
 *   - quantum state simulation;
 *   - quantum operation definitions;
 *   - physical resource allocation;
 *   - hardware discovery;
 *   - device placement;
 *   - routing;
 *   - scheduling;
 *   - calibration;
 *   - QEC;
 *   - ZQN;
 *   - HAL;
 *   - backend implementation;
 *   - runtime execution;
 *   - IR construction.
 *
 * ============================================================================
 * 4. CANONICAL DEPENDENCIES
 * ============================================================================
 *
 * Lexer:
 *
 *     ZamaniLexer
 *
 * Required lexer tokens:
 *
 *     AT
 *     LPAREN
 *     RPAREN
 *     LBRACE
 *     RBRACE
 *     LBRACKET
 *     RBRACKET
 *     COMMA
 *     COLON
 *     SEMICOLON
 *     ASSIGN
 *     DOT
 *
 * Canonical parser dependencies:
 *
 *     identifier
 *     typeExpression
 *     expression
 *     argumentList
 *     statement
 *
 * These rules MUST be supplied by the canonical parser composition.
 *
 * This file MUST NOT define duplicate lexer rules or duplicate general
 * expression/type rules.
 *
 * ============================================================================
 * 5. OPEN-WORLD DESIGN
 * ============================================================================
 *
 * Atomic concepts are not restricted to a hard-coded list of elements,
 * isotopes, materials, particles, or physical implementations.
 *
 * Names and metadata remain open-ended.
 *
 * Examples of annotation names:
 *
 *     @atom
 *     @isotope
 *     @stable
 *     @radioactive
 *     @quantum
 *     @material
 *     @nano
 *     @physical
 *     @symbolic
 *
 * These names are identifiers, not reserved lexer keywords.
 *
 * Their meaning is established by semantic analysis and registered domain
 * definitions.
 *
 * The grammar does not define which annotations are scientifically valid.
 *
 * ============================================================================
 * 6. SCALABILITY AND POCO-REAF
 * ============================================================================
 *
 * Zamani supports source programs ranging from small atomic models to
 * arbitrarily large compositions, subject to actual semantic requirements
 * and available implementation resources.
 *
 * This grammar imposes NO universal limits on:
 *
 *     atoms
 *     atom properties
 *     parameters
 *     compositions
 *     annotations
 *     metadata
 *     nesting
 *     interactions
 *     references
 *     source declarations
 *
 * It MUST NOT introduce artificial constants such as:
 *
 *     MAX_ATOMS
 *     MAX_ATOMIC_NUMBER
 *     MAX_ISOTOPES
 *     MAX_ATOM_PROPERTIES
 *     MAX_ELECTRONS
 *     MAX_COMPOSITION_DEPTH
 *     MAX_ATOM_INTERACTIONS
 *
 * Nor may it enumerate physical atoms as the universal representation.
 *
 * Explicit numeric values are permitted when they are program semantics.
 *
 * The grammar must not interpret a source-level number as an implementation
 * ceiling.
 *
 * POCO-REAF requires the source to express WHAT is intended, while the
 * compiler/runtime determines HOW and WHERE that intent can be realized.
 *
 * ============================================================================
 * 7. RESOURCE AND CAPABILITY SEPARATION
 * ============================================================================
 *
 * This grammar permits declarations of requirements and preferences.
 *
 * It does not determine whether a target satisfies them.
 *
 * Semantic analysis distinguishes:
 *
 *     requirement
 *     constraint
 *     capability
 *     preference
 *     implementation decision
 *
 * Physical realization is downstream.
 *
 * ============================================================================
 * 8. DETERMINISM AND SAFETY
 * ============================================================================
 *
 * This grammar is declarative.
 *
 * It contains:
 *
 *     no embedded Rust;
 *     no grammar actions;
 *     no semantic predicates;
 *     no filesystem access;
 *     no network access;
 *     no environment inspection;
 *     no hardware discovery;
 *     no random behavior;
 *     no runtime execution.
 *
 * Parsing depends only on source text, grammar version, and explicitly
 * configured language extensions.
 *
 * ============================================================================
 * 9. AST AND SEMANTIC INTEGRATION
 * ============================================================================
 *
 * Every parsed atom construct must map to the domain-neutral frontend AST.
 *
 * Required conceptual mappings:
 *
 *     nanoAtomDeclaration
 *         -> declaration node with nano/atom domain metadata
 *
 *     nanoAtomProperty
 *         -> property/member node
 *
 *     nanoAtomComposition
 *         -> composition expression or relationship node
 *
 *     nanoAtomReference
 *         -> ordinary name/reference node
 *
 *     nanoAtomRequirement
 *         -> requirement/constraint representation
 *
 *     nanoAtomCapability
 *         -> capability requirement representation
 *
 *     nanoAtomMetadata
 *         -> attribute/metadata representation
 *
 * The AST must preserve source spans for declarations, names, expressions,
 * attributes, and nested members.
 *
 * Semantic analysis validates meaning, units, scientific models, and
 * domain-specific constraints.
 *
 * No semantic interpretation is performed by this grammar.
 *
 * ============================================================================
 * 10. COMPATIBILITY
 * ============================================================================
 *
 * This grammar is an additive leaf component.
 *
 * Existing nano-agent syntax remains owned by:
 *
 *     grammar/nano/agents.g4
 *
 * Molecular syntax belongs to the molecular grammar component.
 *
 * Material syntax belongs to the material grammar component.
 *
 * Existing root grammar filenames remain unchanged.
 *
 * No new reserved words are required by this file.
 *
 * ============================================================================
 * 11. COMPLETION CRITERIA
 * ============================================================================
 *
 * This component is integrated only when:
 *
 *   [ ] ANTLR generation succeeds with the canonical ZamaniLexer vocabulary.
 *   [ ] The nano dispatcher imports NanoAtoms.
 *   [ ] The universal parser reaches the atom declaration rule.
 *   [ ] All referenced parser rules resolve to canonical definitions.
 *   [ ] AST mappings are implemented.
 *   [ ] Source spans are preserved.
 *   [ ] Semantic validation is implemented.
 *   [ ] Diagnostics are tested.
 *   [ ] Positive and negative tests pass.
 *   [ ] Boundary and scalability tests pass.
 *   [ ] Existing nano-agent syntax remains compatible.
 *   [ ] No artificial physical limits are introduced.
 *   [ ] No duplicate quantum IR is introduced.
 *   [ ] No unsafe Rust is required.
 *
 * ============================================================================
 */

parser grammar NanoAtoms;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * PUBLIC ENTRY POINT
 * ============================================================================
 *
 * The nano domain dispatcher should call nanoAtomDeclaration.
 *
 * This rule deliberately does not consume EOF because it is a composable
 * declaration component.
 */

nanoAtomDeclaration
    : nanoAtomAnnotation*
      identifier
      nanoAtomParameterClause?
      nanoAtomTypeClause?
      nanoAtomDeclarationBody
    ;

/*
 * ============================================================================
 * ANNOTATIONS
 * ============================================================================
 *
 * Annotation names are ordinary identifiers.
 *
 * Examples:
 *
 *     @atom
 *     @isotope
 *     @quantum
 *     @physical
 *     @symbolic
 *
 * Annotation meaning is resolved semantically.
 */

nanoAtomAnnotation
    : AT identifier
      (LPAREN argumentList? RPAREN)?
    ;

/*
 * ============================================================================
 * PARAMETERS
 * ============================================================================
 *
 * Parameters reuse canonical type and expression rules.
 *
 * No fixed parameter count is imposed.
 */

nanoAtomParameterClause
    : LPAREN nanoAtomParameterList? RPAREN
    ;

nanoAtomParameterList
    : nanoAtomParameter
      (COMMA nanoAtomParameter)*
    ;

nanoAtomParameter
    : identifier
      (COLON typeExpression)?
      (ASSIGN expression)?
    ;

/*
 * ============================================================================
 * TYPE
 * ============================================================================
 *
 * The type system owns the meaning of the declared type.
 */

nanoAtomTypeClause
    : COLON typeExpression
    ;

/*
 * ============================================================================
 * DECLARATION BODY
 * ============================================================================
 *
 * Empty bodies are permitted syntactically.
 *
 * Whether an empty atom declaration is semantically meaningful is determined
 * by the domain specification.
 */

nanoAtomDeclarationBody
    : LBRACE nanoAtomMember* RBRACE
    ;

/*
 * ============================================================================
 * MEMBERS
 * ============================================================================
 *
 * Atom members are structurally extensible.
 *
 * They may describe properties, composition, requirements, constraints,
 * capabilities, preferences, metadata, or nested statements.
 */

nanoAtomMember
    : nanoAtomProperty
    | nanoAtomComposition
    | nanoAtomRequirement
    | nanoAtomConstraint
    | nanoAtomCapability
    | nanoAtomPreference
    | nanoAtomMetadata
    | statement
    ;

/*
 * ============================================================================
 * PROPERTIES
 * ============================================================================
 *
 * Properties use ordinary names and expressions.
 *
 * No scientific property vocabulary is hard-coded here.
 */

nanoAtomProperty
    : identifier COLON nanoAtomPropertyValue SEMICOLON?
    ;

nanoAtomPropertyValue
    : expression
    ;

/*
 * ============================================================================
 * COMPOSITION
 * ============================================================================
 *
 * Composition expresses relationships between named atomic entities.
 *
 * It does not allocate physical atoms or choose a physical substrate.
 */

nanoAtomComposition
    : identifier
      (ASSIGN nanoAtomReference)?
      SEMICOLON?
    ;

/*
 * ============================================================================
 * REFERENCES
 * ============================================================================
 *
 * References reuse canonical identifier syntax.
 *
 * Resolution belongs to semantic analysis.
 */

nanoAtomReference
    : identifier
    ;

/*
 * ============================================================================
 * REQUIREMENTS
 * ============================================================================
 *
 * Requirements express source-level conditions.
 *
 * The compiler/runtime determines feasibility.
 */

nanoAtomRequirement
    : identifier expression SEMICOLON?
    ;

/*
 * ============================================================================
 * CONSTRAINTS
 * ============================================================================
 *
 * Constraints are retained as expressions for semantic validation.
 */

nanoAtomConstraint
    : identifier expression SEMICOLON?
    ;

/*
 * ============================================================================
 * CAPABILITIES
 * ============================================================================
 *
 * Capability names are open-ended.
 */

nanoAtomCapability
    : identifier LPAREN argumentList? RPAREN SEMICOLON?
    ;

/*
 * ============================================================================
 * PREFERENCES
 * ============================================================================
 *
 * Preferences do not change the required semantics of the program.
 */

nanoAtomPreference
    : identifier LPAREN argumentList? RPAREN SEMICOLON?
    ;

/*
 * ============================================================================
 * METADATA
 * ============================================================================
 *
 * Metadata remains extensible without adding lexer keywords for every
 * scientific or technological concept.
 */

nanoAtomMetadata
    : AT identifier
      (LPAREN argumentList? RPAREN)?
      SEMICOLON?
    ;

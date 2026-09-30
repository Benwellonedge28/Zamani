
/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/nano/molecules.g4
 *
 * GRAMMAR
 * -------
 * NanoMolecules
 *
 * STATUS
 * ------
 * PROPOSED CANONICAL NANO-MOLECULE DOMAIN GRAMMAR
 *
 * LANGUAGE
 * --------
 * Zamani
 *
 * COMPILER BASELINE
 * -----------------
 * Rust 1.97 / Rust 1.97.1
 * Rust 2021
 * Safe Rust only; no unsafe Rust
 *
 * ============================================================================
 * 1. PURPOSE
 * ============================================================================
 *
 * This grammar owns the source-level STRUCTURE of molecular declarations
 * and molecular composition in Zamani.
 *
 * It provides syntax for:
 *
 *   - annotated molecule declarations;
 *   - named and parameterized molecule declarations;
 *   - optional molecule types;
 *   - molecule bodies;
 *   - typed molecular members;
 *   - member initializers;
 *   - component bindings and references;
 *   - nested molecular declarations;
 *   - open-world molecular annotations/directives;
 *   - ordinary Zamani statements inside a molecule body;
 *   - references and expressions supplied by the canonical language grammar.
 *
 * This is a syntax contract, not a chemistry implementation.
 *
 * The grammar does not determine whether a molecular structure is chemically
 * possible, stable, synthesizable, safe, or physically realizable.
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
 * Canonical Zamani parser composition
 *      |
 *      v
 * NanoMolecules
 *      |
 *      v
 * Domain-neutral frontend AST
 *      |
 *      v
 * Name resolution / type checking / semantic validation
 *      |
 *      +--> molecular structure validation
 *      +--> atom and component resolution
 *      +--> capability and resource analysis
 *      +--> quantum/classical/hybrid analysis
 *      +--> physical-model validation, where applicable
 *      |
 *      v
 * Canonical semantic model
 *      |
 *      v
 * Canonical IR
 *      |
 *      v
 * Optimization / simulation / lowering / target realization
 *
 * Quantum-related computation MUST use the existing canonical quantum::ir
 * boundary. This grammar MUST NOT introduce a second quantum IR.
 *
 * ============================================================================
 * 3. OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   nanoMoleculeConstruct
 *   nanoMoleculeAnnotatedConstruct
 *   nanoMoleculeAnnotation
 *   nanoMoleculeTail
 *   nanoMoleculeDeclaration
 *   nanoMoleculeName
 *   nanoMoleculeParameterClause
 *   nanoMoleculeParameterList
 *   nanoMoleculeParameter
 *   nanoMoleculeTypeClause
 *   nanoMoleculeInitializer
 *   nanoMoleculeBody
 *   nanoMoleculeMember
 *   nanoMoleculeTypedMember
 *   nanoMoleculeComponentBinding
 *   nanoMoleculeNestedConstruct
 *   nanoMoleculeDirective
 *   nanoMoleculeDirectiveTail
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexical token definitions or tokenization;
 *   - identifiers, qualified names, or name resolution;
 *   - general expression precedence;
 *   - general type syntax;
 *   - general statements or blocks;
 *   - atom declarations;
 *   - agent declarations;
 *   - material declarations;
 *   - chemical element or isotope registries;
 *   - periodic-table data;
 *   - molecular bonding rules;
 *   - chemical reaction models;
 *   - physical constants or simulation algorithms;
 *   - quantum state representation or quantum operations;
 *   - hardware topology or device discovery;
 *   - resource allocation, placement, routing, or scheduling;
 *   - calibration, QEC, ZQN, or HAL;
 *   - backend implementation, runtime execution, or IR construction.
 *
 * ============================================================================
 * 4. DEPENDENCIES
 * ============================================================================
 *
 * Lexer vocabulary:
 *
 *     ZamaniLexer
 *
 * Required token names:
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
 *
 * Canonical parser rules consumed:
 *
 *     identifier
 *     typeExpression
 *     expression
 *     argumentList
 *     statement
 *
 * These rules are owned by the canonical Types, Expressions, and Statements
 * parser grammars. This file does not redefine them.
 *
 * The token names above are integration requirements. Before enabling this
 * grammar in the ANTLR build, verify that the actual ZamaniLexer vocabulary
 * exports each name. Do not add duplicate token definitions here.
 *
 * ============================================================================
 * 5. OPEN-WORLD DESIGN
 * ============================================================================
 *
 * Annotation names are identifiers, not a closed list of reserved words.
 *
 * Examples:
 *
 *     @molecule
 *     @component
 *     @atom
 *     @bond
 *     @interaction
 *     @property
 *     @requires
 *     @capability
 *     @constraint
 *     @prefer
 *     @future_extension
 *
 * The parser recognizes the structural annotation form. Semantic analysis
 * owns the registered meaning, validation, versioning, and compatibility of
 * each annotation name.
 *
 * Adding a new molecular annotation must not require adding a lexer keyword
 * or enumerating a new parser alternative.
 *
 * ============================================================================
 * 6. SCALABILITY AND POCO-REAF
 * ============================================================================
 *
 * The grammar imposes no universal upper bound on:
 *
 *     molecules
 *     components
 *     members
 *     parameters
 *     annotations
 *     references
 *     nesting
 *     expressions
 *     declarations
 *     source units
 *
 * It MUST NOT introduce artificial limits such as:
 *
 *     MAX_MOLECULES
 *     MAX_COMPONENTS
 *     MAX_ATOMS_PER_MOLECULE
 *     MAX_BONDS
 *     MAX_MOLECULAR_DEPTH
 *     MAX_MOLECULAR_SIZE
 *
 * Explicit numeric values are valid when they are part of program meaning.
 * They must not be interpreted as language-wide implementation ceilings.
 *
 * "Scale to infinity" means no arbitrary language-level maximum. Actual
 * compilation and execution remain subject to semantic requirements,
 * implementation policy, and available resources.
 *
 * POCO-REAF requires source to describe WHAT is intended. The compiler and
 * runtime determine HOW and WHERE that intent can be realized.
 *
 * ============================================================================
 * 7. SYNTAX MODEL
 * ============================================================================
 *
 * A molecule is introduced by an annotation followed by a name:
 *
 *     @molecule Water {
 *         ...
 *     }
 *
 * A typed or parameterized declaration may be written as:
 *
 *     @molecule Complex<T>(left: T, right: T): MolecularStructure {
 *         ...
 *     }
 *
 * The annotation name is not interpreted by the parser. The canonical nano
 * dispatcher and semantic layer determine whether the annotation denotes a
 * molecule declaration, another nano construct, or an invalid use.
 *
 * Within a molecule body, members have distinct structural forms:
 *
 *     name: Type;
 *     name: Type = expression;
 *     component = expression;
 *     @annotation;
 *     @annotation(arguments);
 *     @annotation { ... }
 *     ordinary_statement
 *
 * The grammar deliberately does not infer chemistry from member names.
 *
 * ============================================================================
 * 8. AST CONTRACT
 * ============================================================================
 *
 * The frontend should preserve:
 *
 *   - the annotation and its source span;
 *   - the declared name and source span;
 *   - parameter syntax and source spans;
 *   - optional type and initializer syntax;
 *   - ordered body members;
 *   - member annotations/directives;
 *   - component expressions;
 *   - ordinary statement nodes;
 *   - complete source spans for the declaration and its children.
 *
 * Prefer existing domain-neutral AST representations for attributes,
 * declarations, bindings, expressions, blocks, and statements.
 *
 * Do not introduce NanoMoleculeIR or a second quantum IR merely because this
 * parser grammar exists. Any specialized semantic representation must be
 * designed at the canonical semantic-model/IR boundary.
 *
 * ============================================================================
 * 9. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis, not parsing, is responsible for:
 *
 *   - resolving the molecule name and referenced components;
 *   - validating parameter names, types, and scopes;
 *   - validating component types and composition relationships;
 *   - detecting invalid or cyclic composition where prohibited;
 *   - interpreting registered annotations and directives;
 *   - checking domain-specific molecular constraints;
 *   - checking capabilities, effects, and resource requirements;
 *   - determining whether a requested physical model is supported;
 *   - producing stable, source-located diagnostics.
 *
 * Unknown annotation names must follow the language's configured extension
 * policy: preserve them for registered extensions or report a semantic
 * diagnostic. The parser must not silently assign them physical meaning.
 *
 * ============================================================================
 * 10. INTEGRATION CONTRACT
 * ============================================================================
 *
 * Expected neighboring files:
 *
 *     grammar/nano/agents.g4
 *     grammar/nano/atoms.g4
 *     grammar/nano/materials.g4
 *     grammar/nano/interactions.g4
 *     grammar/nano/protocols.g4
 *
 * The canonical parser composition layer must:
 *
 *   1. import NanoMolecules;
 *   2. expose nanoMoleculeConstruct through the nano/domain dispatcher;
 *   3. ensure only one dispatcher owns the decision to parse a top-level
 *      nano construct;
 *   4. use the same Types, Expressions, and Statements contracts;
 *   5. avoid importing this grammar through a cycle.
 *
 * Atoms, materials, interactions, and protocols remain separate owners.
 * Their declarations may be referenced through canonical expressions,
 * types, or registered annotations; this grammar does not duplicate them.
 *
 * Rust integration remains in the existing frontend:
 *
 *     src/lexer.rs
 *     src/parser.rs
 *     src/frontend/ast/
 *
 * The Rust frontend must implement or explicitly mark this syntax as
 * unsupported until its AST and parser support are complete. ANTLR grammar
 * acceptance alone does not establish Rust frontend conformance.
 *
 * ============================================================================
 * 11. DETERMINISM AND SAFETY
 * ============================================================================
 *
 * This grammar contains no actions, semantic predicates, embedded code,
 * hardware queries, file access, network access, or physical execution.
 *
 * Parsing is determined by the input token sequence and grammar version.
 *
 * Runtime/compiler implementation must use Rust 2021 and Rust 1.97 or
 * Rust 1.97.1, and must not use unsafe Rust.
 *
 * ============================================================================
 */

parser grammar NanoMolecules;

options {
    tokenVocab = ZamaniLexer;
}

import Types,
       Expressions,
       Statements;

/*
 * Public entry point.
 *
 * The universal parser decides when a nano molecule construct is legal.
 * This grammar does not create another program/source-unit root.
 */
nanoMoleculeConstruct
    : nanoMoleculeAnnotatedConstruct
    ;

/*
 * Annotation-led construct.
 *
 * The annotation name remains open-world. Semantic analysis determines
 * whether it is a molecule declaration or another registered construct.
 */
nanoMoleculeAnnotatedConstruct
    : nanoMoleculeAnnotation
      nanoMoleculeTail
    ;

nanoMoleculeAnnotation
    : AT identifier
    ;

/*
 * Structural alternatives are distinguished by their following punctuation:
 *
 *   identifier ...  named declaration
 *   ( ... )         annotation invocation
 *   { ... }         anonymous annotated body
 *   ;               empty directive
 *
 * A directive without a declaration name cannot accidentally consume an
 * arbitrary expression.
 */
nanoMoleculeTail
    : nanoMoleculeDeclaration
    | nanoMoleculeInvocation
    | nanoMoleculeBody
    | SEMICOLON
    ;

nanoMoleculeInvocation
    : LPAREN
      argumentList?
      RPAREN
      nanoMoleculeInvocationTail
    ;

nanoMoleculeInvocationTail
    : nanoMoleculeBody
    | SEMICOLON
    ;

/*
 * Named molecule declaration.
 *
 * Examples:
 *
 *   @molecule Water { ... }
 *   @molecule Complex<T>(left: T): Structure { ... }
 *   @molecule Complex = expression;
 *
 * The annotation's semantic role is validated outside the parser.
 */
nanoMoleculeDeclaration
    : nanoMoleculeName
      nanoMoleculeParameterClause?
      nanoMoleculeTypeClause?
      nanoMoleculeInitializer?
      nanoMoleculeDeclarationTail
    ;

nanoMoleculeName
    : identifier
    ;

nanoMoleculeParameterClause
    : LT
      nanoMoleculeParameterList?
      GT
    ;

nanoMoleculeParameterList
    : nanoMoleculeParameter
      (COMMA nanoMoleculeParameter)*
      COMMA?
    ;

nanoMoleculeParameter
    : identifier
      (COLON typeExpression)?
      (ASSIGN expression)?
    ;

nanoMoleculeTypeClause
    : COLON typeExpression
    ;

nanoMoleculeInitializer
    : ASSIGN expression
    ;

nanoMoleculeDeclarationTail
    : nanoMoleculeBody
    | SEMICOLON
    ;

/*
 * A body is an ordered sequence. Repetition is not a language-level
 * cardinality limit; practical parser/resource budgets belong to tooling
 * policy and must be diagnosed separately from language validity.
 */
nanoMoleculeBody
    : LBRACE
      nanoMoleculeMember*
      RBRACE
    ;

nanoMoleculeMember
    : nanoMoleculeNestedConstruct
    | nanoMoleculeTypedMember
    | nanoMoleculeComponentBinding
    | nanoMoleculeDirective
    | statement
    ;

/*
 * A typed member declares a molecular property/component slot.
 *
 * Examples:
 *
 *   charge: Charge;
 *   geometry: MolecularGeometry = geometry_expression;
 *   substrate: AtomType;
 *
 * The grammar does not decide whether a member is an atom, bond, property,
 * site, or another domain concept. Registered annotations and semantic
 * types provide that meaning.
 */
nanoMoleculeTypedMember
    : identifier
      COLON
      typeExpression
      (ASSIGN expression)?
      SEMICOLON?
    ;

/*
 * Component binding is intentionally generic.
 *
 * Examples:
 *
 *   oxygen = oxygen_component;
 *   ligand = resolve_ligand(configuration);
 *
 * Whether the expression denotes a valid component is semantic analysis.
 */
nanoMoleculeComponentBinding
    : identifier
      ASSIGN
      expression
      SEMICOLON
    ;

/*
 * Nested annotation-led constructs allow open-world molecular composition
 * without importing atom/material grammars into one another.
 *
 * Examples:
 *
 *   @atom Oxygen { ... }
 *   @molecule Ligand { ... }
 *   @material Substrate { ... }
 *
 * The nano domain dispatcher/semantic layer validates which nested
 * constructs are permitted in the enclosing context.
 */
nanoMoleculeNestedConstruct
    : nanoMoleculeAnnotatedConstruct
    ;

/*
 * Open-world directive inside a molecule.
 *
 * Examples:
 *
 *   @bond(left, right);
 *   @requires(capability("molecular.model"));
 *   @constraint(valid_geometry);
 *   @future_extension { ... }
 *
 * Directive names are identifiers. Their meanings and argument contracts
 * are registered and checked semantically.
 */
nanoMoleculeDirective
    : nanoMoleculeAnnotation
      nanoMoleculeDirectiveTail
    ;

nanoMoleculeDirectiveTail
    : LPAREN
      argumentList?
      RPAREN
      (nanoMoleculeBody | SEMICOLON)?
    | nanoMoleculeBody
    | SEMICOLON
    ;

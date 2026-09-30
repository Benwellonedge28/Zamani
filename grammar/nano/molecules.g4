
/*
 * ============================================================================
 * Zamani Universal Computing Language
 *
 * File: grammar/nano/molecules.g4
 * Grammar: NanoMolecules
 * Status: Proposed canonical molecular-domain parser component
 *
 * Baseline:
 *   Rust 1.97 / 1.97.1
 *   Rust 2021
 *   Safe Rust only
 *   No unsafe Rust
 *
 * ============================================================================
 * 1. PURPOSE
 * ============================================================================
 *
 * Owns the source-level syntax for molecular structures and composition.
 *
 * Supports:
 *   - annotated molecular declarations;
 *   - named and generic molecules;
 *   - optional molecular types;
 *   - molecular parameters;
 *   - typed molecular members;
 *   - component bindings;
 *   - nested nano constructs;
 *   - open-world annotations and directives;
 *   - initialization expressions;
 *   - ordinary Zamani statements;
 *   - symbolic molecular dimensions and properties.
 *
 * This grammar describes computational and structural intent.
 *
 * It does not implement chemistry, quantum mechanics, molecular simulation,
 * chemical feasibility, physical synthesis, or target realization.
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
 * Canonical parser composition
 *      |
 *      v
 * NanoMolecules
 *      |
 *      v
 * Domain-neutral frontend AST
 *      |
 *      v
 * Name resolution / type analysis
 *      |
 *      +--> atom resolution
 *      +--> component resolution
 *      +--> molecular validation
 *      +--> resource analysis
 *      +--> capability analysis
 *      +--> classical analysis
 *      +--> quantum analysis
 *      +--> physical-model validation
 *      |
 *      v
 * Canonical semantic model
 *      |
 *      v
 * Canonical IR
 *      |
 *      +--> classical IR
 *      +--> quantum::ir
 *      +--> hardware/HDL IR where applicable
 *      |
 *      v
 * Optimization / simulation / lowering
 *      |
 *      v
 * Target realization
 *
 * This grammar MUST NOT introduce another quantum IR.
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
 *   nanoMoleculeInvocation
 *   nanoMoleculeInvocationTail
 *   nanoMoleculeDeclaration
 *   nanoMoleculeName
 *   nanoMoleculeParameterClause
 *   nanoMoleculeParameterList
 *   nanoMoleculeParameter
 *   nanoMoleculeTypeClause
 *   nanoMoleculeInitializer
 *   nanoMoleculeDeclarationTail
 *   nanoMoleculeBody
 *   nanoMoleculeMember
 *   nanoMoleculeTypedMember
 *   nanoMoleculeComponentBinding
 *   nanoMoleculeNestedConstruct
 *   nanoMoleculeDirective
 *   nanoMoleculeDirectiveTail
 *   nanoMoleculeMemberAnnotation
 *
 * DOES NOT OWN:
 *
 *   - lexical tokens;
 *   - identifiers or qualified-name resolution;
 *   - general expressions or expression precedence;
 *   - general types;
 *   - ordinary statements;
 *   - atom declarations;
 *   - agent declarations;
 *   - material declarations;
 *   - interactions;
 *   - chemical element/isotope registries;
 *   - molecular bond semantics;
 *   - chemistry algorithms;
 *   - physical constants;
 *   - simulation algorithms;
 *   - quantum state representation;
 *   - quantum operation implementation;
 *   - resource allocation;
 *   - hardware topology;
 *   - physical placement;
 *   - routing or scheduling;
 *   - QEC, ZQN, calibration, or HAL.
 *
 * ============================================================================
 * 4. CANONICAL DEPENDENCIES
 * ============================================================================
 *
 * Lexer vocabulary:
 *
 *   ZamaniLexer
 *
 * Required token names:
 *
 *   AT LPAREN RPAREN
 *   LBRACE RBRACE
 *   COMMA COLON SEMICOLON ASSIGN
 *   LT GT
 *
 * Imported parser grammars:
 *
 *   Types       -> typeExpression
 *   Expressions -> expression, argumentList
 *   Statements  -> statement
 *
 * The canonical lexer MUST export these tokens.
 * This grammar must not define duplicate lexer tokens.
 *
 * ============================================================================
 * 5. OPEN-WORLD DESIGN
 * ============================================================================
 *
 * Annotation names are identifiers, not a closed keyword enumeration.
 *
 * Examples:
 *
 *   @molecule
 *   @atom
 *   @material
 *   @bond
 *   @interaction
 *   @component
 *   @property
 *   @requires
 *   @capability
 *   @constraint
 *   @prefer
 *
 * New annotation names do not require changes to the lexer or this grammar.
 * Their meaning is determined by registered semantic extensions.
 *
 * ============================================================================
 * 6. SCALABILITY AND POCO-REAF
 * ============================================================================
 *
 * No artificial grammar-level maximum is imposed on:
 *
 *   molecules
 *   components
 *   parameters
 *   annotations
 *   members
 *   references
 *   nesting
 *   molecular dimensions
 *   source declarations
 *
 * Prohibited universal language limits include:
 *
 *   MAX_MOLECULES
 *   MAX_COMPONENTS
 *   MAX_ATOMS_PER_MOLECULE
 *   MAX_BONDS
 *   MAX_MOLECULAR_DEPTH
 *   MAX_MOLECULAR_SIZE
 *
 * Numeric values explicitly written in source are program semantics,
 * not universal implementation ceilings.
 *
 * Resource availability and implementation limits are evaluated downstream.
 *
 * ============================================================================
 * 7. AST CONTRACT
 * ============================================================================
 *
 * Preserve:
 *
 *   - complete declaration source span;
 *   - annotation name and span;
 *   - molecule name and span;
 *   - parameter syntax and spans;
 *   - optional type;
 *   - initializer;
 *   - ordered body members;
 *   - member annotations;
 *   - component expressions;
 *   - nested declarations;
 *   - ordinary statements;
 *   - original source locations.
 *
 * Prefer domain-neutral AST nodes.
 *
 * Do not introduce NanoMoleculeIR or a second quantum IR.
 *
 * ============================================================================
 * 8. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *   - determining whether an annotation denotes a molecule;
 *   - resolving molecular identities;
 *   - resolving referenced atoms and components;
 *   - validating parameter scopes and types;
 *   - validating component relationships;
 *   - detecting prohibited cyclic composition;
 *   - interpreting annotations;
 *   - validating molecular properties;
 *   - checking capabilities and resources;
 *   - validating physical-model requirements;
 *   - producing source-located diagnostics.
 *
 * Unknown annotations must follow the configured extension policy.
 * Parsing must never silently assign physical meaning.
 *
 * ============================================================================
 * 9. INTEGRATION CONTRACT
 * ============================================================================
 *
 * Existing neighboring components:
 *
 *   grammar/nano/atoms.g4
 *   grammar/nano/agents.g4
 *   grammar/nano/materials.g4       (when present)
 *   grammar/nano/interactions.g4    (when present)
 *   grammar/nano/protocols.g4       (when present)
 *
 * Shared grammar dependencies:
 *
 *   Types
 *   Expressions
 *   Statements
 *
 * The canonical nano dispatcher owns top-level nano construct selection.
 *
 * Atom, agent, material, and interaction grammars retain their own ownership.
 * This file references their constructs through the shared syntax boundary.
 *
 * Rust frontend integration:
 *
 *   src/lexer.rs
 *   src/parser.rs
 *   src/frontend/ast/
 *
 * ANTLR acceptance does not imply Rust frontend implementation.
 *
 * ============================================================================
 * 10. DETERMINISM AND SAFETY
 * ============================================================================
 *
 * No grammar actions.
 * No semantic predicates.
 * No embedded Rust.
 * No hardware queries.
 * No file or network access.
 * No physical execution.
 *
 * Parsing depends only on the input token sequence and grammar version.
 *
 * Production Rust implementation must use Rust 2021 and Rust 1.97 or
 * Rust 1.97.1, with no unsafe code.
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
 * ============================================================================
 * 11. PUBLIC ENTRY POINT
 * ============================================================================
 *
 * The canonical nano dispatcher invokes nanoMoleculeConstruct.
 *
 * This grammar does not create another program root.
 */

nanoMoleculeConstruct
    : nanoMoleculeAnnotatedConstruct
    ;


/*
 * ============================================================================
 * 12. ANNOTATION-LED CONSTRUCT
 * ============================================================================
 *
 * The annotation's meaning is semantic.
 *
 * The structural tail determines whether the source is:
 *
 *   - a named declaration;
 *   - an annotation invocation;
 *   - an annotated body;
 *   - an empty annotation directive.
 *
 * This avoids treating every annotation as a molecule declaration.
 */

nanoMoleculeAnnotatedConstruct
    : nanoMoleculeAnnotation
      nanoMoleculeTail
    ;


nanoMoleculeAnnotation
    : AT identifier
    ;


nanoMoleculeTail
    : nanoMoleculeDeclaration
    | nanoMoleculeInvocation
    | nanoMoleculeBody
    | SEMICOLON
    ;


/*
 * ============================================================================
 * 13. ANNOTATION INVOCATION
 * ============================================================================
 *
 * Examples:
 *
 *   @molecule(...)
 *   @component(...)
 *   @requires(...)
 *
 * An invocation may optionally have a body or terminate with a semicolon.
 *
 * The invocation itself does not establish a molecular declaration.
 */

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
 * ============================================================================
 * 14. MOLECULAR DECLARATION
 * ============================================================================
 *
 * Supported structural forms:
 *
 *   @molecule Water { ... }
 *
 *   @molecule Complex<T> {
 *       ...
 *   }
 *
 *   @molecule Complex<T>(left: T, right: T): Structure {
 *       ...
 *   }
 *
 *   @molecule Complex = expression;
 *
 * The semantic layer verifies that the annotation is valid for this form.
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
 * ============================================================================
 * 15. MOLECULAR BODY
 * ============================================================================
 *
 * An ordered, extensible sequence of molecular members.
 *
 * No arbitrary member count is imposed.
 *
 * A body may contain nested nano constructs, typed members, component
 * bindings, directives, and ordinary Zamani statements.
 */

nanoMoleculeBody
    : LBRACE
      nanoMoleculeMember*
      RBRACE
    ;


nanoMoleculeMember
    : nanoMoleculeNestedConstruct
    | nanoMoleculeMemberAnnotation
    | nanoMoleculeTypedMember
    | nanoMoleculeComponentBinding
    | nanoMoleculeDirective
    | statement
    ;


/*
 * ============================================================================
 * 16. MEMBER ANNOTATIONS
 * ============================================================================
 *
 * An annotation attached to a member is distinct from a nested declaration.
 *
 * Examples:
 *
 *   @property;
 *   @property(...);
 *
 * An annotation followed by a body is represented as a nested construct.
 */

nanoMoleculeMemberAnnotation
    : nanoMoleculeAnnotation
      nanoMoleculeMemberAnnotationTail
    ;


nanoMoleculeMemberAnnotationTail
    : LPAREN
      argumentList?
      RPAREN
      SEMICOLON
    | SEMICOLON
    ;


/*
 * ============================================================================
 * 17. TYPED MOLECULAR MEMBER
 * ============================================================================
 *
 * Examples:
 *
 *   charge: Charge;
 *   geometry: MolecularGeometry = geometry_expression;
 *   substrate: AtomType;
 *
 * The grammar does not infer whether a member represents an atom, bond,
 * property, site, or another molecular concept.
 *
 * The optional semicolon preserves compatibility with the existing grammar.
 * A future strict syntax profile may require it.
 */

nanoMoleculeTypedMember
    : identifier
      COLON
      typeExpression
      (ASSIGN expression)?
      SEMICOLON?
    ;


/*
 * ============================================================================
 * 18. COMPONENT BINDING
 * ============================================================================
 *
 * Examples:
 *
 *   oxygen = oxygen_component;
 *   ligand = resolve_ligand(configuration);
 *
 * The expression is parsed using the canonical expression grammar.
 *
 * Whether it denotes a valid molecular component is semantic.
 *
 * Semicolon is required to distinguish a completed binding from a following
 * statement.
 */

nanoMoleculeComponentBinding
    : identifier
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 19. NESTED NANO CONSTRUCT
 * ============================================================================
 *
 * Supports composition such as:
 *
 *   @atom Oxygen { ... }
 *   @molecule Ligand { ... }
 *   @material Substrate { ... }
 *
 * This is structural reuse, not ownership transfer.
 *
 * The enclosing semantic context decides which nested constructs are valid.
 */

nanoMoleculeNestedConstruct
    : nanoMoleculeAnnotatedConstruct
    ;


/*
 * ============================================================================
 * 20. OPEN-WORLD DIRECTIVE
 * ============================================================================
 *
 * Examples:
 *
 *   @bond(left, right);
 *   @requires(capability("molecular.model"));
 *   @constraint(valid_geometry);
 *   @future_extension { ... }
 *
 * Directive names remain identifiers.
 *
 * Their meaning is registered and checked semantically.
 */

nanoMoleculeDirective
    : nanoMoleculeAnnotation
      nanoMoleculeDirectiveTail
    ;


nanoMoleculeDirectiveTail
    : LPAREN
      argumentList?
      RPAREN
      (
          nanoMoleculeBody
        | SEMICOLON
      )
    | nanoMoleculeBody
    | SEMICOLON
    ;


/*
 * ============================================================================
 * 21. INTEGRATION INVARIANTS
 * ============================================================================
 *
 * The following invariants are mandatory.
 *
 * 1. One canonical lexer vocabulary.
 *
 * 2. One canonical expression grammar.
 *
 * 3. One canonical type grammar.
 *
 * 4. One canonical statement grammar.
 *
 * 5. One canonical nano dispatcher.
 *
 * 6. Atom declarations remain owned by NanoAtoms.
 *
 * 7. Agent declarations remain owned by NanoAgents.
 *
 * 8. Material declarations remain owned by their material grammar.
 *
 * 9. Interaction declarations remain owned by their interaction grammar.
 *
 * 10. Molecular syntax maps into the domain-neutral AST.
 *
 * 11. Quantum computation uses quantum::ir.
 *
 * 12. No molecular-specific parallel quantum IR is permitted.
 *
 * 13. Resource and capability checks happen after parsing.
 *
 * 14. Physical feasibility is not a parsing decision.
 *
 * 15. No arbitrary molecular-size limits are introduced.
 *
 * 16. No parser rule performs execution or physical simulation.
 *
 * 17. Source spans must be preserved for every declaration and member.
 *
 * 18. Grammar acceptance and Rust frontend acceptance are tracked separately.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */

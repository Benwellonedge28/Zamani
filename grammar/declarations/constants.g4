/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/declarations/constants.g4
 *
 * Grammar role:
 *     Canonical parser grammar for source-level constant declarations.
 *
 * Status:
 *     Production design.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - constantDeclaration;
 *   - constant initializer syntax;
 *   - optional explicit constant type annotation;
 *   - constant declaration generic parameters, where supported by the
 *     language-level declaration model;
 *   - constant-specific compile-time/value declaration syntax;
 *   - the syntactic distinction between a constant and a mutable/local
 *     variable;
 *   - constant declaration structure required by the AST.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexer rules;
 *   - keyword spelling;
 *   - identifier spelling;
 *   - comments;
 *   - attributes;
 *   - visibility;
 *   - general declaration modifiers;
 *   - variable declarations;
 *   - type syntax;
 *   - expression syntax;
 *   - constant evaluation;
 *   - compile-time execution;
 *   - symbol tables;
 *   - name resolution;
 *   - type inference;
 *   - constant-folding implementation;
 *   - overflow checking;
 *   - purity checking;
 *   - effect checking;
 *   - resource validation;
 *   - quantum::ir;
 *   - classical IR;
 *   - HDL IR;
 *   - optimization;
 *   - scheduling;
 *   - routing;
 *   - hardware discovery;
 *   - runtime execution;
 *   - backend selection;
 *   - machine-specific resource limits.
 *
 * Those responsibilities belong to their respective repository subsystems.
 *
 * ============================================================================
 * ARCHITECTURAL BOUNDARY
 * ============================================================================
 *
 * Source
 *   |
 *   v
 * Canonical lexer
 *   |
 *   v
 * Zamani parser
 *   |
 *   +--> declarations/constants.g4
 *   |
 *   +--> declarations/variables.g4
 *   |
 *   +--> types/types.g4
 *   |
 *   +--> expressions/expressions.g4
 *   |
 *   v
 * Frontend AST
 *   |
 *   v
 * Semantic analysis
 *   |
 *   +--> name resolution
 *   +--> type checking
 *   +--> constant evaluation
 *   +--> purity/effect validation
 *   +--> capability/resource validation
 *   |
 *   v
 * Canonical semantic representation / IR
 *   |
 *   +--> classical IR
 *   +--> quantum::ir
 *   +--> HDL/hardware representation
 *   +--> resource/effect metadata
 *   |
 *   v
 * Optimization / lowering / routing / scheduling
 *   |
 *   v
 * Target realization
 *   |
 *   v
 * Runtime / hardware
 *
 * There is intentionally NO direct:
 *
 *     constants.g4 -> runtime
 *     constants.g4 -> hardware
 *     constants.g4 -> quantum::ir
 *     constants.g4 -> ZQN
 *     constants.g4 -> QEC
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Constants describe source-level semantic values.
 *
 * They MUST NOT encode arbitrary machine limits.
 *
 * Examples that MUST remain valid syntactically:
 *
 *     const qubits: Count = desired_qubits;
 *     const dimensions: Shape = shape_expression;
 *     const resource_budget = requested_capacity;
 *     const precision = application_precision;
 *
 * The grammar MUST NOT impose limits such as:
 *
 *     MAX_CONSTANTS
 *     MAX_CONSTANT_NAME_LENGTH
 *     MAX_CONSTANT_EXPRESSION_DEPTH
 *     MAX_ARRAY_ELEMENTS
 *     MAX_TENSOR_RANK
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_MEMORY
 *     MAX_DEVICES
 *     MAX_RESOURCE_COUNT
 *
 * Any practical parser/compiler resource limit is an implementation policy,
 * not a language grammar limit.
 *
 * ============================================================================
 * CONSTANT SEMANTICS
 * ============================================================================
 *
 * A constant declaration introduces a named binding whose value is required
 * by semantic analysis to satisfy the language's constant-expression rules.
 *
 * Syntax establishes:
 *
 *     name
 *     optional type
 *     initializer
 *
 * Semantic analysis determines:
 *
 *     whether the initializer is constant-evaluable;
 *     whether its effects are permitted;
 *     whether its type is valid;
 *     whether inference succeeds;
 *     whether conversions are legal;
 *     whether the resulting value is representable;
 *     whether the declaration is usable in a compile-time context.
 *
 * The grammar MUST NOT attempt to evaluate expressions.
 *
 * ============================================================================
 * TYPE INFERENCE
 * ============================================================================
 *
 * Both forms are intentionally supported:
 *
 *     const answer: int = 42;
 *
 * and:
 *
 *     const answer = 42;
 *
 * The semantic layer decides whether inferred constants are permitted in the
 * relevant declaration scope and computes their canonical type.
 *
 * An explicit annotation remains authoritative when present.
 *
 * ============================================================================
 * CONSTANT EXPRESSION MODEL
 * ============================================================================
 *
 * The initializer is syntactically an ordinary `expression`.
 *
 * This is intentional.
 *
 * The parser must not maintain a second expression grammar for constants.
 *
 * Semantic analysis determines whether the expression is a valid constant
 * expression.
 *
 * This permits future constant evaluation over:
 *
 *     classical values
 *     symbolic values
 *     generic values
 *     type-level values
 *     dimensions
 *     resource quantities
 *     compile-time configuration
 *     quantum-independent mathematical values
 *     hardware-independent configuration values
 *
 * without requiring this grammar to be rewritten for every new domain.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * A constant MAY syntactically contain an expression related to quantum
 * programming if the general expression grammar permits it.
 *
 * However, semantic analysis must distinguish:
 *
 *     compile-time classical value
 *
 * from:
 *
 *     runtime quantum state
 *
 * An arbitrary runtime quantum state is NOT automatically a constant merely
 * because it appears in a constant initializer.
 *
 * The grammar does not attempt to make that distinction.
 *
 * In particular, this grammar does not:
 *
 *     allocate qubits;
 *     identify physical qubits;
 *     select a QPU;
 *     select a topology;
 *     encode a gate set;
 *     serialize arbitrary quantum state;
 *     invoke QEC;
 *     invoke ZQN;
 *     invoke scheduling.
 *
 * ============================================================================
 * HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Constants may provide source-level parameters for hardware/HDL constructs
 * when those constructs consume ordinary language values.
 *
 * Example:
 *
 *     const pipeline_depth: Count = configured_depth;
 *
 * This does NOT mean the grammar selects a physical implementation.
 *
 * Hardware realization remains downstream.
 *
 * ============================================================================
 * RUST CONTRACT
 * ============================================================================
 *
 * This file is an ANTLR parser grammar and contains no Rust implementation
 * code.
 *
 * Rust consumers of the generated parser must support:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * The Zamani Rust implementation MUST NOT require `unsafe`.
 *
 * Recommended crate-level policy:
 *
 *     #![deny(unsafe_code)]
 *     #![deny(unsafe_op_in_unsafe_fn)]
 *
 * This grammar contains no embedded actions or semantic predicates and
 * therefore does not require unsafe Rust or any Rust-specific parser action.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This grammar consumes tokens supplied by the canonical lexer.
 *
 * It MUST NOT define lexer rules.
 *
 * The canonical lexer is responsible for:
 *
 *     CONST
 *     IDENT
 *     COLON
 *     ASSIGN
 *     SEMICOLON
 *
 * and all other lexical tokens.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * This grammar is intended to be imported by the declaration composition
 * grammar:
 *
 *     declarations.g4
 *
 * Conceptually:
 *
 *     parser grammar Declarations;
 *
 *     options {
 *         tokenVocab = ZamaniLexer;
 *     }
 *
 *     import Constants,
 *            Variables,
 *            Types,
 *            ...;
 *
 *     declaration
 *         : constantDeclaration
 *         | variableDeclaration
 *         | ...
 *         ;
 *
 * `declarations.g4` MUST NOT redefine `constantDeclaration`.
 *
 * The previous monolithic `constDecl` rule in `Zamani.g4` is a compatibility
 * concern and must ultimately be migrated to this rule.
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * Direct grammar dependencies:
 *
 *     canonical lexer
 *     Types
 *     Expressions
 *
 * Expected imported parser grammars:
 *
 *     Types
 *     Expressions
 *
 * This file deliberately does NOT import `Declarations`, because that would
 * create the following forbidden cycle:
 *
 *     Declarations -> Constants -> Declarations
 *
 * Declaration modifiers and visibility are applied by the declaration
 * composition layer rather than duplicated here.
 *
 * ============================================================================
 * MODIFIER / ATTRIBUTE INTEGRATION
 * ============================================================================
 *
 * Constant-specific syntax is:
 *
 *     constantDeclaration
 *
 * while declaration-level decoration remains outside this file.
 *
 * For example, the composition layer may produce:
 *
 *     public const answer: int = 42;
 *
 *     @compile_time
 *     public const answer: int = 42;
 *
 * The exact attribute/visibility grammar remains owned by the corresponding
 * declaration/core grammar.
 *
 * This separation prevents:
 *
 *     constants.g4
 *
 * from having to be edited whenever a new general declaration modifier is
 * introduced.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * Every successful `constantDeclaration` parse must provide enough structure
 * for the frontend AST to represent at least:
 *
 *     ConstantDeclaration {
 *         name
 *         explicit_type?
 *         initializer
 *         source_span
 *     }
 *
 * Optional declaration-level metadata such as:
 *
 *     attributes
 *     visibility
 *     documentation
 *     modifiers
 *
 * is attached by the surrounding declaration parser/AST layer.
 *
 * The grammar must preserve source structure and source locations through
 * the normal ANTLR parse tree/token stream.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * After parsing, semantic analysis MUST determine:
 *
 * 1. Name validity.
 * 2. Scope validity.
 * 3. Duplicate-definition errors.
 * 4. Explicit type validity.
 * 5. Initializer type validity.
 * 6. Type inference, when no explicit type is supplied.
 * 7. Constant-expression validity.
 * 8. Evaluation dependencies.
 * 9. Dependency cycles.
 * 10. Purity/effect restrictions.
 * 11. Overflow/range/representation validity.
 * 12. Generic/type-level validity where applicable.
 * 13. Resource/capability legality where applicable.
 *
 * None of these are grammar responsibilities.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * The grammar contains no:
 *
 *     semantic predicates
 *     target-specific actions
 *     external state
 *     filesystem access
 *     network access
 *     runtime callbacks
 *
 * Therefore parsing is structurally deterministic for a fixed token stream.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * PARSER GRAMMAR
 * ============================================================================
 */

parser grammar Constants;


/*
 * ============================================================================
 * TOKEN VOCABULARY
 * ============================================================================
 *
 * The lexer is the single lexical authority.
 */

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * IMPORTS
 * ============================================================================
 *
 * Constants consume the canonical expression and type grammars.
 *
 * No declaration grammar is imported here to avoid a dependency cycle.
 */

import Types,
       Expressions;


/*
 * ============================================================================
 * 1. CANONICAL CONSTANT DECLARATION
 * ============================================================================
 *
 * Explicit type:
 *
 *     const answer: int = 42;
 *
 * Inferred type:
 *
 *     const answer = 42;
 *
 * The initializer is mandatory.
 *
 * A constant without a value has no well-defined constant semantics and is
 * therefore rejected syntactically.
 */

constantDeclaration
    : CONST
      identifier
      constantTypeAnnotation?
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * 2. EXPLICIT CONSTANT TYPE
 * ============================================================================
 *
 * The canonical type grammar owns the actual type syntax.
 *
 * This file only establishes the declaration-level colon boundary.
 *
 * Examples:
 *
 *     const answer: int = 42;
 *     const name: string = "Zamani";
 *     const qcount: Count = requested;
 *     const shape: Shape = dimensions;
 *     const value: quantum::Value = source_value;
 */

constantTypeAnnotation
    : COLON
      typeExpression
    ;


/*
 * ============================================================================
 * 3. IDENTIFIER BRIDGE
 * ============================================================================
 *
 * Identifier spelling belongs to the canonical lexer/name system.
 *
 * This rule only gives the constant grammar a stable parser-level bridge.
 */

identifier
    : IDENT
    ;


/*
 * ============================================================================
 * END OF CONSTANT GRAMMAR
 * ============================================================================
 *
 * No semantic evaluation occurs here.
 *
 * No machine/resource limit is encoded here.
 *
 * No quantum physical resource is selected here.
 *
 * No hardware target is selected here.
 *
 * No runtime action is performed here.
 *
 * The complete downstream flow is:
 *
 *     constantDeclaration
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     name/type/effect analysis
 *          |
 *          v
 *     constant evaluation
 *          |
 *          v
 *     canonical semantic representation
 *          |
 *          +--> classical IR
 *          +--> quantum::ir metadata where semantically applicable
 *          +--> HDL/hardware representation where applicable
 *          +--> resource metadata
 *          |
 *          v
 *     optimization / lowering / scheduling / routing
 *          |
 *          v
 *     target realization
 *
 * This preserves POCO-REAF:
 *
 *     Program Once
 *     Compile Once
 *     Run Everywhere
 *     Run Anywhere
 *     Run Forever
 */
/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/metaprogramming/metaprogramming.g4
 *
 * Grammar:
 *     Metaprogramming
 *
 * Status:
 *     Production integration contract
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     No embedded target-language actions.
 *     No unsafe Rust required or permitted.
 *
 * ============================================================================
 * 1. PURPOSE
 * ============================================================================
 *
 * This grammar defines the canonical metaprogramming composition boundary.
 *
 * It integrates:
 *
 *     - macros;
 *     - compile-time computation;
 *     - reflection;
 *     - source generation;
 *     - specialization;
 *     - quotation;
 *     - unquotation.
 *
 * It recognizes syntax only.
 *
 * It does not execute metaprograms, expand macros, evaluate expressions,
 * construct AST implementations, generate machine code, or create IR.
 *
 * ============================================================================
 * 2. ARCHITECTURAL AUTHORITY
 * ============================================================================
 *
 * Normative architecture:
 *     grammar/DESIGN.md
 *
 * Metaprogramming architecture:
 *     grammar/metaprogramming/README.md
 *
 * Macro syntax:
 *     grammar/macros/
 *
 * Compile-time syntax:
 *     grammar/metaprogramming/compile-time-execution.g4
 *
 * Reflection syntax:
 *     grammar/metaprogramming/reflection.g4
 *
 * Generation syntax:
 *     grammar/metaprogramming/generation.g4
 *
 * Specialization syntax:
 *     grammar/metaprogramming/specialization.g4
 *
 * Quotation syntax:
 *     grammar/metaprogramming/quotation.g4
 *
 * Canonical frontend:
 *     src/lexer.rs
 *     src/parser.rs
 *     src/frontend/ast/
 *
 * Canonical quantum representation:
 *     quantum::ir
 *
 * This grammar must not become another language authority.
 *
 * ============================================================================
 * 3. OWNERSHIP
 * ============================================================================
 *
 * OWNS:
 *
 *     - metaprogramming entry-point dispatch;
 *     - integration categories;
 *     - shared metaprogramming boundaries;
 *     - feature composition;
 *     - declaration/expression/statement dispatch;
 *     - canonical source-structure boundaries.
 *
 * DOES NOT OWN:
 *
 *     - ordinary expressions;
 *     - ordinary statements;
 *     - declarations;
 *     - identifiers;
 *     - names;
 *     - paths;
 *     - types;
 *     - patterns;
 *     - attributes;
 *     - generic parameters;
 *     - lexer tokens;
 *     - keyword definitions;
 *     - macro expansion;
 *     - macro hygiene;
 *     - compile-time evaluation;
 *     - reflection implementation;
 *     - source generation implementation;
 *     - specialization algorithms;
 *     - canonical AST implementation;
 *     - semantic analysis;
 *     - canonical IR;
 *     - quantum::ir;
 *     - HDL/hardware IR;
 *     - optimization;
 *     - scheduling;
 *     - routing;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime execution.
 *
 * ============================================================================
 * 4. SCALABILITY AND PORTABILITY
 * ============================================================================
 *
 * This grammar introduces no language-level resource limits.
 *
 * It must not define fixed limits for:
 *
 *     macros
 *     generated declarations
 *     generated expressions
 *     generated statements
 *     specializations
 *     reflection projections
 *     quotation nesting
 *     type complexity
 *     expression complexity
 *     compile-time computations
 *     quantum operations
 *     hardware devices
 *     distributed nodes
 *
 * Compiler resource admission, cancellation, memory budgets, recursion
 * protection, and execution budgets belong to implementation policy.
 *
 * Such policies must not silently become language-level syntax restrictions.
 *
 * ============================================================================
 * 5. SECURITY
 * ============================================================================
 *
 * Parsing is never execution.
 *
 * A metaprogram must not gain implicit access to:
 *
 *     filesystem
 *     network
 *     environment variables
 *     credentials
 *     subprocesses
 *     devices
 *     hardware
 *     clocks
 *     randomness
 *
 * Such access requires explicit semantic authorization.
 *
 * Generated code must pass through the normal frontend and semantic pipeline.
 *
 * ============================================================================
 * 6. POCO-REAF
 * ============================================================================
 *
 * Metaprogramming must preserve portable source meaning.
 *
 * It must not implicitly select:
 *
 *     a CPU;
 *     a GPU;
 *     an FPGA;
 *     a QPU;
 *     a physical qubit;
 *     a vendor;
 *     a backend;
 *     a memory bank;
 *     a network node;
 *     a physical address.
 *
 * Resource requirements and capabilities are resolved downstream.
 *
 * ============================================================================
 * 7. AST CONTRACT
 * ============================================================================
 *
 * The parser produces syntax structure.
 *
 * The frontend maps that structure to the canonical domain-neutral AST.
 *
 * The AST mapping must preserve:
 *
 *     source spans;
 *     source ordering;
 *     nesting;
 *     syntactic identity;
 *     names and paths;
 *     attributes;
 *     argument structure;
 *     quotation structure;
 *     transformation origin;
 *     generated-source provenance.
 *
 * ============================================================================
 * 8. IR CONTRACT
 * ============================================================================
 *
 * This grammar creates no IR.
 *
 * Generated quantum source follows the ordinary pipeline:
 *
 *     source
 *       -> lexer
 *       -> parser
 *       -> canonical AST
 *       -> semantic analysis
 *       -> quantum::ir
 *
 * No parallel quantum frontend IR is permitted.
 *
 * ============================================================================
 * 9. COMPOSITION CONTRACT
 * ============================================================================
 *
 * This file is a parser grammar component.
 *
 * The final parser composition must supply every delegated rule.
 *
 * The component names and rule names documented below are integration
 * requirements, not definitions of implementations.
 *
 * No unresolved parser-rule references may remain in the final composition.
 *
 * ============================================================================
 */

parser grammar Metaprogramming;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * 10. PUBLIC ENTRY POINTS
 * ============================================================================
 *
 * The canonical parser may call these rules from its appropriate contexts.
 *
 * The canonical parser determines where metaprogramming is legal.
 */

metaprogrammingDeclaration
    : macroDeclaration
    | compileTimeDeclaration
    | generationDeclaration
    | reflectionDeclaration
    | specializationDeclaration
    ;

metaprogrammingExpression
    : macroExpression
    | compileTimeExpression
    | generationExpression
    | reflectionExpression
    | specializationExpression
    ;

metaprogrammingStatement
    : macroStatement
    | compileTimeStatement
    | generationStatement
    | reflectionStatement
    | specializationStatement
    ;

/*
 * ============================================================================
 * 11. MACRO INTEGRATION
 * ============================================================================
 *
 * Owner:
 *     grammar/macros/
 *
 * The macro grammar must expose the following integration rules:
 *
 *     macroDeclarationCore
 *     macroInvocationCore
 *     macroInvocationStatementCore
 *
 * These are delegated contracts.
 *
 * Macro syntax must not be independently redefined here.
 */

macroDeclaration
    : macroDeclarationCore
    ;

macroExpression
    : macroInvocationCore
    ;

macroStatement
    : macroInvocationStatementCore
    ;

/*
 * ============================================================================
 * 12. COMPILE-TIME EXECUTION
 * ============================================================================
 *
 * Owner:
 *     grammar/metaprogramming/compile-time-execution.g4
 *
 * Required integration rules:
 *
 *     compileTimeDeclarationCore
 *     compileTimeExpressionCore
 *     compileTimeStatementCore
 *
 * Parsing must not evaluate these constructs.
 */

compileTimeDeclaration
    : compileTimeDeclarationCore
    ;

compileTimeExpression
    : compileTimeExpressionCore
    ;

compileTimeStatement
    : compileTimeStatementCore
    ;

/*
 * ============================================================================
 * 13. SOURCE GENERATION
 * ============================================================================
 *
 * Owner:
 *     grammar/metaprogramming/generation.g4
 *
 * Required integration rules:
 *
 *     generationDeclarationCore
 *     generationExpressionCore
 *     generationStatementCore
 *
 * Generated code must re-enter the canonical frontend.
 */

generationDeclaration
    : generationDeclarationCore
    ;

generationExpression
    : generationExpressionCore
    ;

generationStatement
    : generationStatementCore
    ;

/*
 * ============================================================================
 * 14. REFLECTION
 * ============================================================================
 *
 * Owner:
 *     grammar/metaprogramming/reflection.g4
 *
 * Required integration rules:
 *
 *     reflectionDeclarationCore
 *     reflectionExpressionCore
 *     reflectionStatementCore
 *
 * Reflection must not create another type system or hardware model.
 */

reflectionDeclaration
    : reflectionDeclarationCore
    ;

reflectionExpression
    : reflectionExpressionCore
    ;

reflectionStatement
    : reflectionStatementCore
    ;

/*
 * ============================================================================
 * 15. SPECIALIZATION
 * ============================================================================
 *
 * Owner:
 *     grammar/metaprogramming/specialization.g4
 *
 * Required integration rules:
 *
 *     specializationDeclarationCore
 *     specializationExpressionCore
 *     specializationStatementCore
 *
 * Specialization may change implementation strategy but must preserve
 * observable source semantics.
 */

specializationDeclaration
    : specializationDeclarationCore
    ;

specializationExpression
    : specializationExpressionCore
    ;

specializationStatement
    : specializationStatementCore
    ;

/*
 * ============================================================================
 * 16. QUOTATION AND UNQUOTATION
 * ============================================================================
 *
 * Owner:
 *     grammar/metaprogramming/quotation.g4
 *
 * Quotation and unquotation must be integrated through the generation
 * subsystem.
 *
 * They must not establish a second quotation language.
 *
 * Required integration rules:
 *
 *     metaQuoteCore
 *     metaSpliceCore
 *
 * These rules must be supplied exactly once by the quotation/generation
 * composition.
 */

metaQuote
    : metaQuoteCore
    ;

metaSplice
    : metaSpliceCore
    ;

/*
 * ============================================================================
 * 17. CANONICAL META-VALUE BOUNDARIES
 * ============================================================================
 *
 * These rules deliberately delegate ordinary language constructs.
 *
 * They do not redefine the canonical expression, type, pattern, or name
 * grammar.
 *
 * Required canonical rules:
 *
 *     expression
 *     typeExpression
 *     pattern
 *     qualifiedName
 *     identifier
 *     statement
 *     item
 *     block
 *     genericParameters
 *     parameterList
 *     argumentList
 *     attribute
 */

metaValue
    : expression
    | typeExpression
    | pattern
    | qualifiedName
    ;

metaSource
    : expression
    | statement
    | item
    ;

metaName
    : identifier
    ;

metaPath
    : qualifiedName
    ;

metaType
    : typeExpression
    ;

metaPattern
    : pattern
    ;

metaBlock
    : block
    ;

metaExpression
    : expression
    ;

metaStatement
    : statement
    ;

metaDeclaration
    : item
    ;

metaGenericParameters
    : genericParameters
    ;

metaParameterList
    : parameterList
    ;

metaArgumentList
    : argumentList
    ;

metaAttribute
    : attribute
    ;

/*
 * ============================================================================
 * 18. SOURCE STRUCTURE
 * ============================================================================
 *
 * Source structure is represented through canonical Zamani constructs.
 *
 * This grammar does not accept embedded ANTLR grammar definitions.
 *
 * Language-definition and dialect-description syntax belongs to the
 * appropriate language-definition subsystem.
 */

metaSourceStructure
    : metaSource
    ;

/*
 * ============================================================================
 * 19. TRANSFORMATION BOUNDARY
 * ============================================================================
 *
 * Transformations are represented as semantic requests.
 *
 * The grammar does not perform transformations.
 *
 * The compiler must validate every transformed or generated construct.
 */

metaprogrammingTransformation
    : metaprogrammingExpression
    ;

/*
 * ============================================================================
 * 20. PORTABILITY CONTRACT
 * ============================================================================
 *
 * A metaprogramming transformation must preserve:
 *
 *     source meaning;
 *     type safety;
 *     effect correctness;
 *     capability correctness;
 *     resource requirements;
 *     provenance;
 *     deterministic behavior where required.
 *
 * Target-specific specialization must remain an implementation decision
 * downstream of semantic validation.
 *
 * ============================================================================
 * 21. INTEGRATION ACCEPTANCE
 * ============================================================================
 *
 * The final ANTLR composition must verify:
 *
 *     [ ] All delegated rules resolve.
 *     [ ] Every delegated rule has one authoritative owner.
 *     [ ] No duplicate parser rule names exist.
 *     [ ] No duplicate lexer vocabulary exists.
 *     [ ] The canonical parser imports or composes this component correctly.
 *     [ ] Canonical expression/type/name rules are reused.
 *     [ ] Macro syntax is not duplicated.
 *     [ ] Quotation syntax is not duplicated.
 *     [ ] Generated code re-enters the canonical frontend.
 *     [ ] AST source spans and provenance are preserved.
 *     [ ] No target-language actions execute during parsing.
 *     [ ] No hardware limits are encoded.
 *     [ ] No second quantum IR is introduced.
 *     [ ] Rust implementation uses safe Rust.
 *
 * ============================================================================
 */
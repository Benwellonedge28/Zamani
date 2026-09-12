/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/core/compilation-unit.g4
 *
 * Purpose:
 *     Canonical parser entry-point contract for a complete Zamani source
 *     compilation unit.
 *
 * Language:
 *     Zamani
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     Safe Rust only.
 *     This grammar contains no Rust implementation code and introduces no
 *     unsafe requirement.
 *
 * ============================================================================
 * ARCHITECTURAL ROLE
 * ============================================================================
 *
 * This file owns ONLY the source-root composition of Zamani.
 *
 * It answers:
 *
 *     "What constitutes one complete, independently parseable Zamani source
 *      artifact?"
 *
 * It does NOT own:
 *
 *     - lexical definitions
 *     - identifiers
 *     - literals
 *     - expressions
 *     - statements
 *     - declarations
 *     - types
 *     - functions
 *     - modules
 *     - quantum operations
 *     - classical operations
 *     - HDL constructs
 *     - hardware constructs
 *     - resource semantics
 *     - capability semantics
 *     - scheduling
 *     - routing
 *     - optimization
 *     - QEC
 *     - ZQN
 *     - resilience
 *     - runtime behavior
 *     - target-specific implementation
 *
 * Those concerns belong to their owning grammar or semantic subsystem.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * The compilation unit is intentionally independent of:
 *
 *     - CPU count
 *     - GPU count
 *     - FPGA count
 *     - ASIC count
 *     - qubit count
 *     - register count
 *     - memory capacity
 *     - topology
 *     - device identifier
 *     - vendor
 *     - runtime instance
 *     - deployment size
 *
 * A compilation unit therefore describes source-level computation and intent.
 *
 * Resource availability, target capability, placement, scheduling, routing,
 * calibration and execution are resolved downstream.
 *
 * ============================================================================
 * DEPENDENCY DIRECTION
 * ============================================================================
 *
 *     ZamaniLexer
 *          |
 *          v
 *     compilationUnit
 *          |
 *          v
 *     source structure
 *          |
 *          v
 *     AST
 *          |
 *          v
 *     name/module resolution
 *          |
 *          v
 *     type/effect/capability/resource analysis
 *          |
 *          v
 *     canonical semantic IR
 *          |
 *          +----> classical IR
 *          |
 *          +----> quantum::ir
 *          |
 *          +----> HDL/hardware semantic representations
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing / scheduling / resilience / ZQN / QEC
 *          |
 *          v
 *     target lowering
 *          |
 *          v
 *     runtime / hardware
 *
 * The dependency direction MUST NOT be reversed.
 *
 * ============================================================================
 * IMPORTANT
 * ============================================================================
 *
 * This grammar MUST NOT:
 *
 *     1. create a second AST;
 *     2. create an IR;
 *     3. encode hardware capacity;
 *     4. encode a maximum source size;
 *     5. encode a maximum declaration count;
 *     6. encode a maximum nesting depth;
 *     7. encode a maximum qubit count;
 *     8. encode a maximum device count;
 *     9. select a physical backend;
 *    10. perform semantic validation.
 *
 * Parser implementation resource exhaustion is an implementation/runtime
 * concern, not a language-level grammar bound.
 *
 * ============================================================================
 */

parser grammar CompilationUnit;

/*
 * The canonical Zamani lexer is assembled independently under:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * The lexer components under:
 *
 *     grammar/lexer/
 *
 * are subordinate lexical components and MUST NOT become alternate lexer
 * authorities.
 *
 * The parser therefore consumes the canonical ZamaniLexer token vocabulary.
 */
options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * CANONICAL ROOT
 * ============================================================================
 *
 * `compilationUnit` is the single canonical root for a complete Zamani source
 * artifact.
 *
 * There is intentionally no machine-specific root such as:
 *
 *     quantumCompilationUnit
 *     gpuCompilationUnit
 *     cpuCompilationUnit
 *     fpgaCompilationUnit
 *     hardwareCompilationUnit
 *
 * A single Zamani source language can contain multiple computational domains.
 *
 * ============================================================================
 */

compilationUnit
    : sourcePrologue?
      sourceItem*
      EOF
    ;


/*
 * ============================================================================
 * SOURCE PROLOGUE
 * ============================================================================
 *
 * The prologue contains source-level metadata that must occur before ordinary
 * source items when the language specification assigns such ordering.
 *
 * The exact semantic meaning of each member is owned by its corresponding
 * grammar/semantic subsystem.
 *
 * This rule deliberately does not interpret metadata.
 * ============================================================================
 */

sourcePrologue
    : sourceDirective+
    ;


/*
 * ============================================================================
 * SOURCE DIRECTIVES
 * ============================================================================
 *
 * Directives are source-level declarations that influence interpretation of
 * the compilation unit without becoming machine-specific execution commands.
 *
 * This rule is intentionally extensible through qualified names.
 *
 * A directive is not automatically a compiler instruction.
 *
 * Its semantic meaning must be established by semantic analysis.
 * ============================================================================
 */

sourceDirective
    : directiveName directiveArguments? SEMI?
    ;


/*
 * A directive name is a qualified language-level name.
 *
 * The concrete identifier/qualified-name implementation belongs to the core
 * name grammar.
 */
directiveName
    : qualifiedName
    ;


/*
 * Directive arguments remain expressions rather than introducing another
 * argument representation.
 *
 * This prevents compilation-unit metadata from creating a parallel expression
 * language.
 */
directiveArguments
    : LPAREN argumentList? RPAREN
    ;


/*
 * ============================================================================
 * SOURCE ITEMS
 * ============================================================================
 *
 * A source item is the top-level structural unit of a compilation unit.
 *
 * Attributes are attached structurally to the item they precede.
 *
 * This preserves source ordering and allows the AST to retain the exact
 * ownership relationship:
 *
 *     attributes -> item
 *
 * rather than treating attributes as unrelated global metadata.
 * ============================================================================
 */

sourceItem
    : attributes* topLevelItem
    ;


/*
 * ============================================================================
 * TOP-LEVEL ITEMS
 * ============================================================================
 *
 * This rule is the composition boundary between this file and all other
 * grammar domains.
 *
 * It intentionally references semantic categories rather than reproducing
 * their internal syntax.
 *
 * ============================================================================
 */

topLevelItem
    : declaration
    | statement
    ;


/*
 * ============================================================================
 * DECLARATIONS
 * ============================================================================
 *
 * This rule is the compilation-unit integration point for declaration grammar.
 *
 * Every declaration family must have exactly one owner elsewhere in the
 * grammar architecture.
 *
 * No declaration syntax is duplicated here.
 * ============================================================================
 */

declaration
    : moduleDeclaration
    | importDeclaration
    | exportDeclaration
    | useDeclaration

    | packageDeclaration

    | functionDeclaration
    | structDeclaration
    | enumDeclaration
    | traitDeclaration
    | implDeclaration
    | classDeclaration
    | interfaceDeclaration
    | recordDeclaration
    | unionDeclaration
    | typeAliasDeclaration
    | constantDeclaration
    | variableDeclaration

    | effectDeclaration
    | capabilityDeclaration
    | resourceDeclaration
    | requirementDeclaration
    | constraintDeclaration
    | preferenceDeclaration
    | targetDeclaration

    | quantumDeclaration
    | hardwareDeclaration
    | hdlDeclaration
    | classicalDeclaration
    | hybridDeclaration

    | distributedDeclaration
    | dataDeclaration
    | aiDeclaration
    | networkingDeclaration
    | securityDeclaration

    | externDeclaration
    | foreignDeclaration

    | macroDeclaration
    | dialectDeclaration
    | languageDeclaration

    | compileDeclaration
    | executionDeclaration
    ;


/*
 * ============================================================================
 * STATEMENTS
 * ============================================================================
 *
 * Statement syntax remains owned by the statement grammar.
 *
 * This file only establishes that a statement can participate in a compilation
 * unit where the language specification permits top-level executable source.
 *
 * Whether a particular statement is legal at module/package/file scope is a
 * semantic/context validation responsibility and MUST NOT be encoded by
 * duplicating statement grammar here.
 * ============================================================================
 */

statement
    : variableStatement
    | constantStatement
    | expressionStatement

    | returnStatement
    | breakStatement
    | continueStatement
    | yieldStatement

    | ifStatement
    | whileStatement
    | doWhileStatement
    | forStatement
    | forallStatement
    | foreachStatement

    | matchStatement

    | tryStatement
    | throwStatement
    | handleStatement

    | blockStatement

    | quantumStatement
    | hardwareStatement
    | hdlStatement
    | hybridStatement

    | concurrentStatement
    | distributedStatement

    | temporalStatement

    | compileStatement
    | executionStatement

    | unsafeSourceBlock
    | emptyStatement
    ;


/*
 * ============================================================================
 * SOURCE ATTRIBUTES
 * ============================================================================
 *
 * Attribute syntax is owned by the attribute grammar.
 *
 * Keeping the reference here instead of defining:
 *
 *     '@' identifier ...
 *
 * prevents the compilation-unit grammar from becoming another attribute
 * authority.
 * ============================================================================
 */

attributes
    : attribute+
    ;


/*
 * ============================================================================
 * NAME CONTRACT
 * ============================================================================
 *
 * The compilation-unit grammar consumes the canonical qualified-name rule.
 *
 * It MUST NOT introduce an alternative identifier grammar.
 *
 * This is important for deterministic name resolution and for avoiding
 * collisions between:
 *
 *     module names
 *     type names
 *     hardware names
 *     quantum names
 *     resource names
 *     dialect names
 * ============================================================================
 */

qualifiedName
    : identifier (DOUBLE_COLON identifier)*
    ;

identifier
    : IDENTIFIER
    ;


/*
 * ============================================================================
 * PLACEHOLDER INTEGRATION CONTRACTS
 * ============================================================================
 *
 * These rules are intentionally declared as parser-level integration points.
 *
 * In the final grammar assembly they MUST be supplied by the owning parser
 * grammars through ANTLR parser-grammar imports or an equivalent canonical
 * composition mechanism.
 *
 * They are NOT semantic implementations.
 *
 * They exist here to make the compilation-unit dependency contract explicit.
 *
 * A repository integration MUST NOT create duplicate definitions with
 * different meanings.
 * ============================================================================
 */


/*
 * ---------------------------------------------------------------------------
 * CORE / MODULE SYSTEM
 * ---------------------------------------------------------------------------
 */

moduleDeclaration
    : MODULE qualifiedName moduleBody?
    ;

moduleBody
    : LBRACE sourceItem* RBRACE
    ;

importDeclaration
    : IMPORT importTarget importSource? SEMI?
    ;

importTarget
    : qualifiedName
    | STAR AS identifier
    | LBRACE importSpecifierList RBRACE
    ;

importSpecifierList
    : importSpecifier (COMMA importSpecifier)* COMMA?
    ;

importSpecifier
    : identifier (AS identifier)?
    ;

importSource
    : FROM stringLiteral
    ;

exportDeclaration
    : EXPORT exportTarget exportSource? SEMI?
    ;

exportTarget
    : STAR
    | qualifiedName
    | LBRACE exportSpecifierList RBRACE
    ;

exportSpecifierList
    : exportSpecifier (COMMA exportSpecifier)* COMMA?
    ;

exportSpecifier
    : identifier (AS identifier)?
    ;

exportSource
    : FROM stringLiteral
    ;

useDeclaration
    : USE qualifiedName (AS identifier)? SEMI?
    ;

packageDeclaration
    : PACKAGE identifier LBRACE packageField* RBRACE
    ;

packageField
    : identifier COLON expression SEMI?
    ;


/*
 * ---------------------------------------------------------------------------
 * FUNCTIONS / TYPES / DECLARATIONS
 * ---------------------------------------------------------------------------
 *
 * These declarations are integration names.
 *
 * Their complete syntax belongs to the corresponding grammar component.
 * ---------------------------------------------------------------------------
 */

functionDeclaration
    : FUNCTION_DECLARATION
    ;

structDeclaration
    : STRUCT_DECLARATION
    ;

enumDeclaration
    : ENUM_DECLARATION
    ;

traitDeclaration
    : TRAIT_DECLARATION
    ;

implDeclaration
    : IMPL_DECLARATION
    ;

classDeclaration
    : CLASS_DECLARATION
    ;

interfaceDeclaration
    : INTERFACE_DECLARATION
    ;

recordDeclaration
    : RECORD_DECLARATION
    ;

unionDeclaration
    : UNION_DECLARATION
    ;

typeAliasDeclaration
    : TYPE_ALIAS_DECLARATION
    ;

constantDeclaration
    : CONSTANT_DECLARATION
    ;

variableDeclaration
    : VARIABLE_DECLARATION
    ;


/*
 * ---------------------------------------------------------------------------
 * EFFECT / RESOURCE / CAPABILITY SYSTEM
 * ---------------------------------------------------------------------------
 *
 * These are source-level declarations.
 *
 * They do not mean that a requested resource exists.
 *
 * Capability/resource satisfiability is downstream semantic analysis.
 * ---------------------------------------------------------------------------
 */

effectDeclaration
    : EFFECT_DECLARATION
    ;

capabilityDeclaration
    : CAPABILITY_DECLARATION
    ;

resourceDeclaration
    : RESOURCE_DECLARATION
    ;

requirementDeclaration
    : REQUIREMENT_DECLARATION
    ;

constraintDeclaration
    : CONSTRAINT_DECLARATION
    ;

preferenceDeclaration
    : PREFERENCE_DECLARATION
    ;

targetDeclaration
    : TARGET_DECLARATION
    ;


/*
 * ---------------------------------------------------------------------------
 * COMPUTATIONAL DOMAINS
 * ---------------------------------------------------------------------------
 *
 * These integration points deliberately avoid fixed hardware inventories.
 *
 * For example:
 *
 *     quantumDeclaration
 *
 * MUST NOT imply:
 *
 *     N <= 32
 *     N <= 64
 *     q[0]
 *     q[1]
 *
 * Physical realization belongs downstream.
 * ---------------------------------------------------------------------------
 */

quantumDeclaration
    : QUANTUM_DECLARATION
    ;

hardwareDeclaration
    : HARDWARE_DECLARATION
    ;

hdlDeclaration
    : HDL_DECLARATION
    ;

classicalDeclaration
    : CLASSICAL_DECLARATION
    ;

hybridDeclaration
    : HYBRID_DECLARATION
    ;

distributedDeclaration
    : DISTRIBUTED_DECLARATION
    ;

dataDeclaration
    : DATA_DECLARATION
    ;

aiDeclaration
    : AI_DECLARATION
    ;

networkingDeclaration
    : NETWORKING_DECLARATION
    ;

securityDeclaration
    : SECURITY_DECLARATION
    ;


/*
 * ---------------------------------------------------------------------------
 * INTEROPERABILITY / META-LANGUAGE
 * ---------------------------------------------------------------------------
 */

externDeclaration
    : EXTERN_DECLARATION
    ;

foreignDeclaration
    : FOREIGN_DECLARATION
    ;

macroDeclaration
    : MACRO_DECLARATION
    ;

dialectDeclaration
    : DIALECT_DECLARATION
    ;

languageDeclaration
    : LANGUAGE_DECLARATION
    ;


/*
 * ---------------------------------------------------------------------------
 * COMPILATION / EXECUTION
 * ---------------------------------------------------------------------------
 */

compileDeclaration
    : COMPILE_DECLARATION
    ;

executionDeclaration
    : EXECUTION_DECLARATION
    ;


/*
 * ============================================================================
 * STATEMENT INTEGRATION CONTRACT
 * ============================================================================
 */

variableStatement
    : VARIABLE_STATEMENT
    ;

constantStatement
    : CONSTANT_STATEMENT
    ;

expressionStatement
    : EXPRESSION_STATEMENT
    ;

returnStatement
    : RETURN_STATEMENT
    ;

breakStatement
    : BREAK_STATEMENT
    ;

continueStatement
    : CONTINUE_STATEMENT
    ;

yieldStatement
    : YIELD_STATEMENT
    ;

ifStatement
    : IF_STATEMENT
    ;

whileStatement
    : WHILE_STATEMENT
    ;

doWhileStatement
    : DO_WHILE_STATEMENT
    ;

forStatement
    : FOR_STATEMENT
    ;

forallStatement
    : FORALL_STATEMENT
    ;

foreachStatement
    : FOREACH_STATEMENT
    ;

matchStatement
    : MATCH_STATEMENT
    ;

tryStatement
    : TRY_STATEMENT
    ;

throwStatement
    : THROW_STATEMENT
    ;

handleStatement
    : HANDLE_STATEMENT
    ;

blockStatement
    : BLOCK_STATEMENT
    ;

quantumStatement
    : QUANTUM_STATEMENT
    ;

hardwareStatement
    : HARDWARE_STATEMENT
    ;

hdlStatement
    : HDL_STATEMENT
    ;

hybridStatement
    : HYBRID_STATEMENT
    ;

concurrentStatement
    : CONCURRENT_STATEMENT
    ;

distributedStatement
    : DISTRIBUTED_STATEMENT
    ;

temporalStatement
    : TEMPORAL_STATEMENT
    ;

compileStatement
    : COMPILE_STATEMENT
    ;

executionStatement
    : EXECUTION_STATEMENT
    ;

unsafeSourceBlock
    : UNSAFE_SOURCE_BLOCK
    ;

emptyStatement
    : SEMI
    ;


/*
 * ============================================================================
 * SHARED LEXICAL/EXPRESSION INTEGRATION
 * ============================================================================
 *
 * These rules are intentionally small forwarding contracts.
 * The owning grammar must provide the actual implementation.
 * ============================================================================
 */

argumentList
    : expression (COMMA expression)* COMMA?
    ;

expression
    : EXPRESSION
    ;

stringLiteral
    : STRING_LITERAL
    ;


/*
 * ============================================================================
 * ARCHITECTURAL INVARIANTS
 * ============================================================================
 *
 * 1. Exactly one canonical complete-source entry point exists:
 *
 *        compilationUnit
 *
 * 2. compilationUnit always consumes EOF.
 *
 * 3. No source item is silently discarded.
 *
 * 4. Source ordering is preserved.
 *
 * 5. Attributes remain associated with the item they decorate.
 *
 * 6. Nested module bodies use the same source-item model.
 *
 * 7. No fixed source-item count exists.
 *
 * 8. No fixed nesting count exists.
 *
 * 9. No fixed machine/resource quantity exists.
 *
 * 10. No quantum hardware inventory exists here.
 *
 * 11. No target device is selected here.
 *
 * 12. No scheduling decision is made here.
 *
 * 13. No routing decision is made here.
 *
 * 14. No optimization decision is made here.
 *
 * 15. No QEC algorithm is defined here.
 *
 * 16. No ZQN noise model is defined here.
 *
 * 17. No resilience policy is defined here.
 *
 * 18. No runtime behavior is defined here.
 *
 * 19. No physical resource is allocated here.
 *
 * 20. No canonical quantum IR is created here.
 *
 * 21. `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * 22. The parser is deterministic for identical token streams.
 *
 * 23. Diagnostics can retain source ordering and source locations.
 *
 * 24. Future domains can be integrated through owned declaration/statement
 *     grammar components without changing the conceptual source root.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * There are intentionally no constructs such as:
 *
 *     MAX_ITEMS
 *     MAX_DECLARATIONS
 *     MAX_MODULES
 *     MAX_QUANTUM_RESOURCES
 *     MAX_QUBITS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_THREADS
 *
 * A source artifact is limited only by:
 *
 *     - representational validity;
 *     - parser implementation resources;
 *     - host memory;
 *     - host execution resources;
 *     - downstream compiler resources;
 *     - target capabilities;
 *     - physical reality.
 *
 * Such limits are NOT language grammar constants.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * The compilation unit must remain valid independently of where it executes.
 *
 * Therefore:
 *
 *     source semantics
 *         !=
 *     target realization
 *
 * and:
 *
 *     source compilation unit
 *         !=
 *     physical machine description
 *
 * A valid source program may subsequently be realized as:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     FPGA
 *     ASIC
 *     CPU
 *     GPU
 *     distributed
 *     embedded
 *     cloud
 *     heterogeneous
 *     future target
 *
 * without changing the compilation-unit abstraction itself.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * The parser must lower this root structurally to an AST equivalent to:
 *
 *     CompilationUnit {
 *         prologue,
 *         items,
 *         source_span
 *     }
 *
 * The actual Rust AST type is owned by the repository AST/frontend subsystem.
 *
 * This grammar MUST NOT define a second AST representation.
 *
 * The AST must preserve:
 *
 *     - source ordering;
 *     - item boundaries;
 *     - attributes;
 *     - source locations;
 *     - documentation metadata where applicable;
 *     - nested module structure.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing this rule successfully means only:
 *
 *     "the source has valid compilation-unit structure."
 *
 * It does NOT mean:
 *
 *     "the program is type correct."
 *     "the resources exist."
 *     "the target can execute it."
 *     "the quantum program fits a machine."
 *     "the hardware can realize it."
 *     "the schedule is feasible."
 *     "the program is safe to execute."
 *
 * Those questions belong to downstream semantic analysis.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * QUANTUM INTEGRATION CONTRACT
 * ============================================================================
 *
 * Quantum syntax enters through:
 *
 *     quantumDeclaration
 *     quantumStatement
 *
 * The compilation-unit grammar must not know:
 *
 *     gate inventories
 *     qubit counts
 *     topology
 *     coupling maps
 *     calibration
 *     pulse durations
 *     physical qubit identifiers
 *     error rates
 *
 * Those are downstream concerns.
 *
 * The eventual semantic path is:
 *
 *     Zamani source
 *          |
 *          v
 *     AST
 *          |
 *          v
 *     semantic quantum model
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing
 *          |
 *          v
 *     scheduling
 *          |
 *          v
 *     ZQN / QEC / resilience
 *          |
 *          v
 *     hardware/runtime
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * HARDWARE / HDL INTEGRATION CONTRACT
 * ============================================================================
 *
 * Hardware and HDL declarations are syntactic domain members.
 *
 * They MUST NOT turn the compilation-unit grammar into a machine inventory.
 *
 * Hardware capabilities and constraints are represented by their owning
 * grammar/semantic layers.
 *
 * This keeps:
 *
 *     language semantics
 *
 * separate from:
 *
 *     physical realization.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * ERROR / DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * A complete parser invocation must consume EOF.
 *
 * Trailing tokens are therefore errors rather than silently ignored input.
 *
 * The Rust frontend must report:
 *
 *     source identifier
 *     span
 *     line/column where available
 *     expected syntax
 *     encountered token
 *     stable diagnostic category/code
 *
 * Diagnostic construction belongs to the Rust frontend/diagnostic subsystem,
 * not to this grammar.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * For an identical token stream:
 *
 *     compilationUnit
 *
 * MUST produce the same parse structure.
 *
 * The grammar must not depend on:
 *
 *     wall-clock time
 *     random values
 *     machine identity
 *     hardware topology
 *     resource availability
 *     runtime state
 *     network state
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing valid Zamani source must not be silently reinterpreted by this
 * root grammar.
 *
 * When declaration/statement grammar is migrated from:
 *
 *     grammar/Zamani.g4
 *     grammar/antlr/Core.g4
 *
 * into modular grammar components, the migration must preserve the semantic
 * source categories unless an explicit language-version migration is defined.
 *
 * Legacy grammar material in:
 *
 *     grammar/Zamani-Grammar.md
 *
 * is reference/design material and must not silently become a second parser
 * authority.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * This file requires at least the following parser tests.
 *
 * POSITIVE:
 *
 *     empty compilation unit
 *     single declaration
 *     multiple declarations
 *     attributes + declaration
 *     module containing source items
 *     imports
 *     exports
 *     packages
 *     functions
 *     classical declaration
 *     quantum declaration
 *     hybrid declaration
 *     HDL declaration
 *     hardware declaration
 *     distributed declaration
 *     AI/data declaration
 *     compile declaration
 *     execution declaration
 *
 * NEGATIVE:
 *
 *     unexpected trailing tokens
 *     malformed module
 *     malformed import
 *     malformed export
 *     malformed declaration
 *     malformed attribute
 *     unterminated block
 *     incomplete source item
 *
 * BOUNDARY:
 *
 *     zero source items
 *     one source item
 *     many source items
 *     deeply nested modules
 *     large source units
 *     very large declaration sequences
 *
 * SCALABILITY:
 *
 *     no grammar-defined maximum number of source items
 *     no grammar-defined maximum modules
 *     no grammar-defined maximum domain declarations
 *     no grammar-defined machine/resource limit
 *
 * CROSS-DOMAIN:
 *
 *     classical + quantum
 *     quantum + hardware
 *     classical + HDL
 *     quantum + HDL
 *     quantum + distributed
 *     classical + quantum + distributed
 *     classical + quantum + HDL + hardware
 *
 * DETERMINISM:
 *
 *     identical input -> identical parse tree
 *
 * EOF:
 *
 *     valid source followed by arbitrary trailing tokens -> diagnostic
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when:
 *
 * [ ] It is the sole canonical complete-source root.
 * [ ] It consumes EOF.
 * [ ] It does not duplicate domain grammar ownership.
 * [ ] It does not define machine limits.
 * [ ] It does not define quantum hardware limits.
 * [ ] It does not create a second AST.
 * [ ] It does not create an IR.
 * [ ] It integrates with ZamaniLexer.
 * [ ] It integrates with the modular parser grammar.
 * [ ] It preserves source ordering.
 * [ ] It preserves attribute ownership.
 * [ ] It supports nested module source items.
 * [ ] It has deterministic parsing.
 * [ ] It has positive tests.
 * [ ] It has negative tests.
 * [ ] It has boundary tests.
 * [ ] It has cross-domain tests.
 * [ ] It has scalability tests.
 * [ ] It has EOF/trailing-token tests.
 * [ ] It has compatibility tests against the previous root grammar.
 * [ ] Its AST mapping is documented.
 * [ ] Its semantic boundary is documented.
 * [ ] Its integration with quantum::ir is documented.
 * [ ] Its integration with hardware/HDL is documented.
 * [ ] Its Rust frontend integration is documented.
 * [ ] No unsafe Rust is required.
 *
 * ============================================================================
 */
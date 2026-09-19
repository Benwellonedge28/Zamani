/**
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/Zamani.g4
 *
 * STATUS
 * ------
 * CANONICAL ANTLR LANGUAGE COMPOSITION ROOT
 *
 * PURPOSE
 * -------
 * This file is the single canonical ANTLR entry point for the Zamani
 * programming language.
 *
 * It composes the lexical and parser architecture of the entire grammar/
 * hierarchy without becoming the implementation owner of individual
 * language domains.
 *
 * Zamani is one language capable of expressing:
 *
 *   - classical computation
 *   - quantum computation
 *   - hybrid computation
 *   - hardware description
 *   - hardware/software co-design
 *   - embedded computation
 *   - systems programming
 *   - distributed computation
 *   - parallel/HPC computation
 *   - AI/ML
 *   - data processing
 *   - networking
 *   - cryptography/security
 *   - scientific computation
 *   - accelerators
 *   - edge/cloud execution
 *   - future computational paradigms
 *
 * PRINCIPLE
 * ---------
 *
 *     Program Once
 *          ↓
 *     Compile Once
 *          ↓
 *     Run Everywhere
 *          ↓
 *     Run Anywhere
 *          ↓
 *     Forever
 *
 * The source program describes portable computation and its semantic
 * requirements. Target realization is performed downstream.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 *
 *   - canonical grammar identity
 *   - canonical program entry point
 *   - top-level composition
 *   - parser/grammar integration boundaries
 *   - universal source-unit composition
 *   - EOF enforcement
 *   - feature-composition policy
 *
 * THIS FILE DOES NOT OWN
 *
 *   - individual lexer rules
 *   - individual parser rules
 *   - type-system implementation
 *   - semantic analysis
 *   - AST implementation
 *   - classical IR
 *   - quantum::ir
 *   - HDL IR
 *   - optimization
 *   - routing
 *   - scheduling
 *   - QEC
 *   - ZQN
 *   - HAL
 *   - calibration
 *   - target discovery
 *   - hardware mapping
 *   - runtime execution
 *
 * ============================================================================
 * AUTHORITY MODEL
 * ============================================================================
 *
 * Normative language specification:
 *
 *     grammar/specification/
 *
 * Canonical grammar:
 *
 *     grammar/Zamani.g4
 *
 * Lexical composition:
 *
 *     grammar/lexer/tokens.g4
 *
 * Lexer integration:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Parser composition:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * Rust implementation/conformance:
 *
 *     src/lexer.rs
 *     src/parser.rs
 *
 * Domain-neutral AST:
 *
 *     src/ast/
 *
 * Canonical quantum semantic boundary:
 *
 *     quantum::ir
 *
 * ============================================================================
 * NON-DUPLICATION RULE
 * ============================================================================
 *
 * A construct MUST have one authoritative grammar owner.
 *
 * This root may reference a construct.
 *
 * It must not independently redefine that construct when the construct is
 * owned by another grammar.
 *
 * In particular, this file MUST NOT become another monolithic implementation
 * of:
 *
 *     expressions
 *     types
 *     quantum
 *     HDL
 *     AI
 *     networking
 *     etc.
 *
 * ============================================================================
 * SCALABILITY / POCO-REAF
 * ============================================================================
 *
 * There are NO language-level universal limits on:
 *
 *     qubits
 *     logical qubits
 *     physical resources
 *     CPUs
 *     cores
 *     threads
 *     GPUs
 *     FPGAs
 *     accelerators
 *     QPUs
 *     nodes
 *     processes
 *     tasks
 *     channels
 *     memory
 *     storage
 *     registers
 *     tensor dimensions
 *     tensor rank
 *     vector width
 *     timelines
 *     devices
 *     network links
 *     modules
 *     declarations
 *     functions
 *     source size
 *
 * Repetition in this grammar is therefore expressed with ANTLR repetition
 * constructs such as *, + and ? rather than artificial implementation limits.
 *
 * A program may contain a literal value such as:
 *
 *     1024
 *
 * without that value becoming a universal compiler limit.
 *
 * ============================================================================
 * HARD-CODING PROHIBITION
 * ============================================================================
 *
 * Never introduce language-level constants such as:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_DEVICES
 *
 * Physical capacity belongs to target capabilities and resource management,
 * not to the source grammar.
 *
 * ============================================================================
 * SEMANTIC REQUIREMENT VS IMPLEMENTATION DECISION
 * ============================================================================
 *
 * Source syntax may express:
 *
 *     requirement
 *     constraint
 *     capability
 *     preference
 *     hint
 *     budget
 *
 * It must not accidentally turn those into physical implementation choices.
 *
 * For example:
 *
 *     requires capability("quantum.measurement")
 *
 * expresses semantic capability requirements.
 *
 * A later compiler stage may decide which device and physical resources
 * satisfy that requirement.
 *
 * ============================================================================
 * QUANTUM ARCHITECTURE
 * ============================================================================
 *
 * Quantum syntax is accepted here through the quantum parser composition
 * boundary.
 *
 * This file MUST NOT define a second quantum IR.
 *
 * The canonical path is:
 *
 *     Zamani source
 *          ↓
 *     domain-neutral AST
 *          ↓
 *     semantic quantum model
 *          ↓
 *     quantum::ir
 *          ↓
 *     optimization
 *          ↓
 *     routing
 *          ↓
 *     scheduling
 *          ↓
 *     QEC / resilience
 *          ↓
 *     ZQN
 *          ↓
 *     HAL
 *          ↓
 *     target realization
 *
 * The grammar must not hard-code a closed universe of gates.
 *
 * Generic operations remain extensible.
 *
 * ============================================================================
 * HDL / HARDWARE ARCHITECTURE
 * ============================================================================
 *
 * HDL syntax describes hardware intent.
 *
 * Hardware realization is downstream.
 *
 * Source syntax must not impose universal:
 *
 *     bus width
 *     register count
 *     memory size
 *     device count
 *     accelerator count
 *     topology size
 *
 * unless the value is explicitly part of the programmer's semantic design.
 *
 * ============================================================================
 * CROSS-DOMAIN COMPOSITION
 * ============================================================================
 *
 * Classical, quantum, HDL, AI, data, distributed, networking and other
 * domains are not separate languages.
 *
 * They share:
 *
 *     names
 *     types
 *     expressions
 *     statements
 *     declarations
 *     modules
 *     functions
 *     effects
 *     memory
 *     concurrency
 *     resources
 *     capabilities
 *     diagnostics
 *     source locations
 *     versioning
 *
 * ============================================================================
 * SOURCE SPANS
 * ============================================================================
 *
 * Every parser construct must preserve enough source-location information
 * for the frontend to construct accurate diagnostics and AST spans.
 *
 * The grammar must not discard source information merely because a construct
 * is domain-specific.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Grammar parsing must be deterministic for a given:
 *
 *     source
 *     grammar version
 *     dialect configuration
 *     lexical configuration
 *
 * Parsing must not depend on:
 *
 *     wall-clock time
 *     randomness
 *     machine hardware
 *     runtime resources
 *     network state
 *     filesystem state
 *     environment variables
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar contains no:
 *
 *     target-language actions
 *     arbitrary code execution
 *     filesystem access
 *     network access
 *     hardware discovery
 *     environment inspection
 *     secret access
 *
 * ============================================================================
 * RUST IMPLEMENTATION CONTRACT
 * ============================================================================
 *
 * Consumers of this grammar are maintained for:
 *
 *     Rust 2021
 *     Rust 1.97 / 1.97.1
 *
 * Rust implementation must use safe Rust.
 *
 * No `unsafe` is required by this grammar architecture.
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * The grammar hierarchy is:
 *
 *     lexical leaves
 *          ↓
 *     lexer composition
 *          ↓
 *     ZamaniLexer
 *          ↓
 *     parser leaves
 *          ↓
 *     domain dispatchers
 *          ↓
 *     universal parser composition
 *          ↓
 *     Zamani
 *
 * This root is deliberately thin.
 *
 * ============================================================================
 */

grammar Zamani;


/* ============================================================================
 * CANONICAL PROGRAM ENTRY
 * ============================================================================
 *
 * Exactly one complete-program entry point exists.
 *
 * `program` consumes a complete Zamani compilation unit and requires EOF.
 *
 * No domain grammar may introduce another competing universal program root.
 * ============================================================================
 */

program
    : sourceUnit EOF
    ;


/* ============================================================================
 * SOURCE UNIT
 * ============================================================================
 *
 * A source unit is an ordered, unbounded sequence of top-level source items.
 *
 * There is deliberately no artificial maximum number of declarations,
 * modules, functions, domains or other source elements.
 * ============================================================================
 */

sourceUnit
    : sourceItem*
    ;


/* ============================================================================
 * UNIVERSAL SOURCE ITEM
 * ============================================================================
 *
 * Domain-specific syntax enters the language through explicit composition
 * boundaries.
 *
 * The detailed rules belong to their owning grammar.
 * ============================================================================
 */

sourceItem
    : documentationItem
    | attributeItem
    | packageItem
    | moduleItem
    | importItem
    | exportItem
    | usingItem
    | declarationItem
    | statementItem
    | domainItem
    ;


/* ============================================================================
 * CORE COMPOSITION
 * ============================================================================
 *
 * These names are contracts for the canonical core dispatcher.
 *
 * The implementation must provide exactly one authoritative definition for
 * each dispatcher.
 * ============================================================================
 */

documentationItem
    : coreDocumentation
    ;

attributeItem
    : coreAttribute
    ;

packageItem
    : packageDeclaration
    ;

moduleItem
    : moduleDeclaration
    ;

importItem
    : importDeclaration
    ;

exportItem
    : exportDeclaration
    ;

usingItem
    : usingDeclaration
    ;


/* ============================================================================
 * DECLARATION COMPOSITION
 * ============================================================================
 */

declarationItem
    : declaration
    ;


/* ============================================================================
 * STATEMENT COMPOSITION
 * ============================================================================
 */

statementItem
    : statement
    ;


/* ============================================================================
 * DOMAIN COMPOSITION
 * ============================================================================
 *
 * These are semantic domains of the same language.
 *
 * The domain dispatchers own their detailed syntax.
 * ============================================================================
 */

domainItem
    : classicalItem
    | quantumItem
    | hybridItem
    | hdlItem
    | hardwareItem
    | distributedItem
    | aiItem
    | dataItem
    | networkingItem
    | securityItem
    | resourceItem
    | compileItem
    | executionItem
    | interoperabilityItem
    | dialectItem
    | macroItem
    | metaprogrammingItem
    ;


/* ============================================================================
 * CLASSICAL COMPUTING
 * ============================================================================
 */

classicalItem
    : classicalDeclaration
    | classicalStatement
    | classicalExpression
    ;


/* ============================================================================
 * QUANTUM COMPUTING
 * ============================================================================
 *
 * Quantum grammar owns source syntax only.
 *
 * No physical qubit assignment is performed here.
 * ============================================================================
 */

quantumItem
    : quantumDeclaration
    | quantumStatement
    | quantumExpression
    ;


/* ============================================================================
 * HYBRID COMPUTING
 * ============================================================================
 */

hybridItem
    : hybridDeclaration
    | hybridStatement
    | hybridExpression
    ;


/* ============================================================================
 * HDL
 * ============================================================================
 */

hdlItem
    : hdlDeclaration
    | hdlStatement
    | hdlExpression
    ;


/* ============================================================================
 * HARDWARE INTENT
 * ============================================================================
 */

hardwareItem
    : hardwareDeclaration
    | hardwareStatement
    | hardwareExpression
    ;


/* ============================================================================
 * DISTRIBUTED COMPUTING
 * ============================================================================
 */

distributedItem
    : distributedDeclaration
    | distributedStatement
    | distributedExpression
    ;


/* ============================================================================
 * AI / ML
 * ============================================================================
 */

aiItem
    : aiDeclaration
    | aiStatement
    | aiExpression
    ;


/* ============================================================================
 * DATA
 * ============================================================================
 */

dataItem
    : dataDeclaration
    | dataStatement
    | dataExpression
    ;


/* ============================================================================
 * NETWORKING
 * ============================================================================
 */

networkingItem
    : networkingDeclaration
    | networkingStatement
    | networkingExpression
    ;


/* ============================================================================
 * SECURITY
 * ============================================================================
 */

securityItem
    : securityDeclaration
    | securityStatement
    | securityExpression
    ;


/* ============================================================================
 * RESOURCES / CAPABILITIES
 * ============================================================================
 */

resourceItem
    : resourceDeclaration
    | resourceStatement
    | resourceExpression
    ;


/* ============================================================================
 * COMPILATION
 * ============================================================================
 */

compileItem
    : compileDeclaration
    | compileStatement
    | compileExpression
    ;


/* ============================================================================
 * EXECUTION
 * ============================================================================
 */

executionItem
    : executionDeclaration
    | executionStatement
    | executionExpression
    ;


/* ============================================================================
 * INTEROPERABILITY
 * ============================================================================
 */

interoperabilityItem
    : interoperabilityDeclaration
    | interoperabilityStatement
    | interoperabilityExpression
    ;


/* ============================================================================
 * DIALECTS
 * ============================================================================
 */

dialectItem
    : dialectDeclaration
    | dialectStatement
    | dialectExpression
    ;


/* ============================================================================
 * MACROS
 * ============================================================================
 */

macroItem
    : macroDeclaration
    | macroStatement
    | macroExpression
    ;


/* ============================================================================
 * METAPROGRAMMING
 * ============================================================================
 */

metaprogrammingItem
    : metaprogrammingDeclaration
    | metaprogrammingStatement
    | metaprogrammingExpression
    ;


/* ============================================================================
 * INTEGRATION INVARIANTS
 * ============================================================================
 *
 * 1. No domain grammar may bypass the universal AST boundary.
 *
 * 2. No quantum grammar may create a second quantum IR.
 *
 * 3. No grammar may introduce physical hardware limits.
 *
 * 4. No domain may create an incompatible top-level program grammar.
 *
 * 5. Domain syntax must remain composable with:
 *
 *        declarations
 *        statements
 *        expressions
 *        types
 *        modules
 *        functions
 *        effects
 *        memory
 *        concurrency
 *        resources
 *
 * 6. All constructs must preserve source-span information.
 *
 * 7. All constructs must have predetermined:
 *
 *        AST mapping
 *        semantic mapping
 *        IR mapping
 *        diagnostics
 *        compatibility policy
 *        positive tests
 *        negative tests
 *        boundary tests
 *        scalability tests
 *
 * 8. Hardware realization occurs downstream.
 *
 * 9. Parser behavior must not depend on target hardware.
 *
 * 10. This root must remain a composition layer rather than becoming a
 *     second monolithic grammar.
 *
 * ============================================================================
 * END
 * ============================================================================
 */
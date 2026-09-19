/**
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *     grammar/antlr/ZamaniParser.g4
 *
 * Status:
 *     CANONICAL PRODUCTION ANTLR PARSER ORCHESTRATOR
 *
 * Purpose:
 *     Single ANTLR4 parser composition root for the complete Zamani language.
 *
 * ============================================================================
 * ARCHITECTURE
 * ============================================================================
 *
 *                           Zamani source
 *                                |
 *                                v
 *                         ZamaniLexer
 *                                |
 *                                v
 *                         ZamaniParser
 *                                |
 *          +---------------------+----------------------+
 *          |                     |                      |
 *          v                     v                      v
 *        Core                  Types                 Domains
 *          |                     |                      |
 *          +---------------------+----------------------+
 *                                |
 *                                v
 *                         Domain-neutral AST
 *                                |
 *                                v
 *                    structural / semantic analysis
 *                                |
 *                                v
 *                       canonical semantic model
 *                                |
 *          +---------------------+----------------------+
 *          |                     |                      |
 *          v                     v                      v
 *     Classical IR          quantum::ir          HDL/Hardware IR
 *                                |
 *                                v
 *                     optimization / lowering
 *                                |
 *              +-----------------+----------------+
 *              |                 |                |
 *              v                 v                v
 *           routing          scheduling       resilience
 *                                |
 *                                v
 *                               ZQN
 *                                |
 *                                v
 *                               HAL
 *                                |
 *                                v
 *                        target realization
 *
 * ============================================================================
 * AUTHORITATIVE BOUNDARIES
 * ============================================================================
 *
 * This file owns ONLY:
 *
 *   - the parser composition root;
 *   - the single public program entry point;
 *   - universal source-item dispatch;
 *   - cross-domain composition;
 *   - parser-level integration of domain delegates.
 *
 * This file does NOT own:
 *
 *   - lexical token definitions;
 *   - individual expressions;
 *   - individual statements;
 *   - individual declarations;
 *   - type implementations;
 *   - quantum operation implementations;
 *   - QEC;
 *   - ZQN;
 *   - routing;
 *   - scheduling;
 *   - HAL;
 *   - hardware discovery;
 *   - compiler backend selection;
 *   - runtime execution.
 *
 * Every non-root syntax rule MUST have exactly one owning grammar component.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * The parser imposes NO universal implementation limits on:
 *
 *   CPUs
 *   cores
 *   threads
 *   GPUs
 *   FPGAs
 *   ASICs
 *   QPUs
 *   qubits
 *   registers
 *   memory
 *   storage
 *   nodes
 *   devices
 *   accelerators
 *   tensor dimensions
 *   tensor rank
 *   vector width
 *   processes
 *   tasks
 *   channels
 *   timelines
 *   program size
 *
 * Repetition and recursion express source semantics.
 *
 * Hardware/resource limits are downstream concerns.
 *
 * A source-level requirement such as:
 *
 *     requires qubits >= n;
 *
 * is valid.
 *
 * A grammar-level universal limit such as:
 *
 *     MAX_QUBITS = 1024
 *
 * is prohibited.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Quantum syntax ultimately lowers to:
 *
 *     quantum::ir
 *
 * quantum::ir is the canonical quantum semantic boundary.
 *
 * This parser MUST NOT create:
 *
 *     - a second quantum IR;
 *     - a physical-qubit mapping;
 *     - a QPU topology;
 *     - a routing plan;
 *     - a schedule;
 *     - a calibration;
 *     - a QEC implementation;
 *     - a ZQN implementation.
 *
 * Quantum operation names remain extensible source-level identifiers.
 *
 * ============================================================================
 * SAFETY / DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no embedded Rust actions;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no environment inspection;
 *     - no hardware discovery;
 *     - no randomness;
 *     - no runtime execution.
 *
 * The Rust implementation consuming this grammar MUST use:
 *
 *     Rust 2021
 *     Rust 1.97 / 1.97.1
 *     safe Rust only
 *     no unsafe
 *
 * ============================================================================
 *
 * IMPORTANT BUILD CONTRACT
 * ============================================================================
 *
 * ANTLR grammar imports are by GRAMMAR NAME, not arbitrary repository
 * filesystem path.
 *
 * Therefore this file imports canonical delegate grammars.
 *
 * Each delegate MUST be made resolvable by the ANTLR build through the
 * configured grammar source/library path.
 *
 * The delegates themselves compose the leaf grammars belonging to their
 * respective directories.
 *
 * Example:
 *
 *     grammar/quantum/
 *         quantum.g4
 *         qubits.g4
 *         operations.g4
 *         measurement.g4
 *         ...
 *
 * becomes one canonical Quantum parser delegate.
 *
 * The root parser MUST NOT import every leaf grammar independently.
 *
 * ============================================================================
 */

parser grammar ZamaniParser;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * CANONICAL DOMAIN DELEGATES
 * ============================================================================
 *
 * These names are architectural contracts.
 *
 * They MUST correspond to canonical parser-composition grammars supplied to
 * the ANTLR grammar library/source path by the build system.
 *
 * There must be exactly one canonical delegate for each ownership domain.
 *
 * ============================================================================
 */

import
    Core,
    Types,
    Expressions,
    Statements,
    Declarations,
    Functions,
    Modules,
    Effects,
    Memory,
    Concurrency,
    Classical,
    Quantum,
    Hybrid,
    HDL,
    Hardware,
    Distributed,
    AI,
    Data,
    Networking,
    Security,
    Resources,
    Compile,
    Execution,
    Interoperability,
    Dialects,
    Macros,
    Metaprogramming
;


/* ============================================================================
 * 1. SINGLE PUBLIC ENTRY POINT
 * ============================================================================
 *
 * There is exactly one public source-program entry point.
 *
 * No domain delegate may define another competing program root.
 * ============================================================================
 */

program
    : sourceUnit EOF
    ;


/* ============================================================================
 * 2. SOURCE UNIT
 * ============================================================================
 *
 * Core owns the lexical/source-unit concepts.
 *
 * The root parser owns only their composition.
 * ============================================================================
 */

sourceUnit
    : sourceElement*
    ;


/* ============================================================================
 * 3. UNIVERSAL SOURCE-ELEMENT DISPATCH
 * ============================================================================
 *
 * Every construct entering a Zamani compilation unit passes through this
 * dispatch boundary.
 *
 * The root does not implement the detailed syntax of the selected construct.
 * ============================================================================
 */

sourceElement
    : documentationElement
    | attributeElement
    | pragmaElement
    | packageElement
    | moduleElement
    | importElement
    | exportElement
    | declarationElement
    | statementElement
    | domainElement
    ;


/* ============================================================================
 * 4. DOCUMENTATION
 * ============================================================================
 *
 * Documentation syntax is delegated to Core.
 * ============================================================================
 */

documentationElement
    : coreDocumentation
    ;


/* ============================================================================
 * 5. ATTRIBUTES / METADATA
 * ============================================================================
 */

attributeElement
    : coreAttribute
    ;


/* ============================================================================
 * 6. PRAGMAS
 * ============================================================================
 */

pragmaElement
    : corePragma
    ;


/* ============================================================================
 * 7. PACKAGE / MODULE COMPOSITION
 * ============================================================================
 */

packageElement
    : modulePackageDeclaration
    ;

moduleElement
    : moduleDeclaration
    ;

importElement
    : importDeclaration
    ;

exportElement
    : exportDeclaration
    ;


/* ============================================================================
 * 8. DECLARATION DISPATCH
 * ============================================================================
 *
 * Declarations remain domain-neutral at this boundary.
 *
 * Detailed ownership belongs to declarations/, functions/, modules/, effects/,
 * classical/, quantum/, hdl/, hardware/, AI, data, etc.
 * ============================================================================
 */

declarationElement
    : declaration
    ;


/* ============================================================================
 * 9. STATEMENT DISPATCH
 * ============================================================================
 */

statementElement
    : statement
    ;


/* ============================================================================
 * 10. DOMAIN DISPATCH
 * ============================================================================
 *
 * This is the principal cross-domain orchestration point.
 *
 * A Zamani source file may combine domains in one compilation unit.
 *
 * Examples include:
 *
 *     classical + quantum
 *     classical + HDL
 *     quantum + hardware
 *     AI + data
 *     distributed + networking
 *     software + accelerator
 *     quantum + classical + distributed
 *
 * No domain is a separate language.
 * ============================================================================
 */

domainElement
    : classicalElement
    | quantumElement
    | hybridElement
    | hdlElement
    | hardwareElement
    | distributedElement
    | aiElement
    | dataElement
    | networkingElement
    | securityElement
    | resourceElement
    | compileElement
    | executionElement
    | interoperabilityElement
    | dialectElement
    | macroElement
    | metaprogrammingElement
    ;


/* ============================================================================
 * 11. CLASSICAL
 * ============================================================================
 */

classicalElement
    : classicalDeclaration
    | classicalStatement
    | classicalExpression
    ;


/* ============================================================================
 * 12. QUANTUM
 * ============================================================================
 *
 * The Quantum delegate owns:
 *
 *     qubits
 *     registers
 *     states
 *     operations
 *     parameterized operations
 *     controlled operations
 *     adjoints
 *     measurement
 *     reset
 *     observables
 *     dynamic circuits
 *     mid-circuit control
 *     logical qubits
 *     physical-qubit intent
 *     error-correction intent
 *     quantum resources
 *     quantum capabilities
 *     quantum dialects
 *
 * The root merely composes it.
 * ============================================================================
 */

quantumElement
    : quantumDeclaration
    | quantumStatement
    | quantumExpression
    ;


/* ============================================================================
 * 13. HYBRID
 * ============================================================================
 */

hybridElement
    : hybridDeclaration
    | hybridStatement
    | hybridExpression
    ;


/* ============================================================================
 * 14. HDL
 * ============================================================================
 */

hdlElement
    : hdlDeclaration
    | hdlStatement
    | hdlExpression
    ;


/* ============================================================================
 * 15. HARDWARE
 * ============================================================================
 */

hardwareElement
    : hardwareDeclaration
    | hardwareStatement
    | hardwareExpression
    ;


/* ============================================================================
 * 16. DISTRIBUTED
 * ============================================================================
 */

distributedElement
    : distributedDeclaration
    | distributedStatement
    | distributedExpression
    ;


/* ============================================================================
 * 17. AI / ML
 * ============================================================================
 */

aiElement
    : aiDeclaration
    | aiStatement
    | aiExpression
    ;


/* ============================================================================
 * 18. DATA
 * ============================================================================
 */

dataElement
    : dataDeclaration
    | dataStatement
    | dataExpression
    ;


/* ============================================================================
 * 19. NETWORKING
 * ============================================================================
 */

networkingElement
    : networkingDeclaration
    | networkingStatement
    | networkingExpression
    ;


/* ============================================================================
 * 20. SECURITY
 * ============================================================================
 */

securityElement
    : securityDeclaration
    | securityStatement
    | securityExpression
    ;


/* ============================================================================
 * 21. RESOURCES / CAPABILITIES
 * ============================================================================
 *
 * Resource syntax expresses requirements, constraints, preferences, hints and
 * capabilities.
 *
 * It MUST NOT perform hardware discovery.
 * ============================================================================
 */

resourceElement
    : resourceDeclaration
    | resourceStatement
    | resourceExpression
    ;


/* ============================================================================
 * 22. COMPILATION INTENT
 * ============================================================================
 */

compileElement
    : compileDeclaration
    | compileStatement
    | compileExpression
    ;


/* ============================================================================
 * 23. EXECUTION INTENT
 * ============================================================================
 */

executionElement
    : executionDeclaration
    | executionStatement
    | executionExpression
    ;


/* ============================================================================
 * 24. INTEROPERABILITY
 * ============================================================================
 */

interoperabilityElement
    : interoperabilityDeclaration
    | interoperabilityStatement
    | interoperabilityExpression
    ;


/* ============================================================================
 * 25. DIALECTS
 * ============================================================================
 */

dialectElement
    : dialectDeclaration
    | dialectStatement
    | dialectExpression
    ;


/* ============================================================================
 * 26. MACROS
 * ============================================================================
 */

macroElement
    : macroDeclaration
    | macroStatement
    | macroExpression
    ;


/* ============================================================================
 * 27. METAPROGRAMMING
 * ============================================================================
 *
 * This is the canonical integration point for:
 *
 *     grammar/metaprogramming/
 *
 * including:
 *
 *     reflection
 *     generation
 *     specialization
 *     compile-time execution
 *
 * The obsolete grammar/antlr/Meta.g4 MUST NOT become a second metaprogramming
 * authority.
 * ============================================================================
 */

metaprogrammingElement
    : metaprogrammingDeclaration
    | metaprogrammingStatement
    | metaprogrammingExpression
    ;


/* ============================================================================
 * 28. UNIVERSAL CROSS-DOMAIN DECLARATION
 * ============================================================================
 *
 * Domain-specific declarations can occur wherever Zamani declarations are
 * legal.
 *
 * The delegate grammars provide the actual alternatives.
 * ============================================================================
 */

declaration
    : coreDeclaration
    | typeDeclaration
    | functionDeclaration
    | moduleOwnedDeclaration
    | effectDeclaration
    | memoryDeclaration
    | concurrencyDeclaration
    | classicalDeclaration
    | quantumDeclaration
    | hybridDeclaration
    | hdlDeclaration
    | hardwareDeclaration
    | distributedDeclaration
    | aiDeclaration
    | dataDeclaration
    | networkingDeclaration
    | securityDeclaration
    | resourceDeclaration
    | compileDeclaration
    | executionDeclaration
    | interoperabilityDeclaration
    | dialectDeclaration
    | macroDeclaration
    | metaprogrammingDeclaration
    ;


/* ============================================================================
 * 29. UNIVERSAL CROSS-DOMAIN STATEMENT
 * ============================================================================
 */

statement
    : coreStatement
    | expressionStatement
    | controlFlowStatement
    | functionStatement
    | effectStatement
    | memoryStatement
    | concurrencyStatement
    | classicalStatement
    | quantumStatement
    | hybridStatement
    | hdlStatement
    | hardwareStatement
    | distributedStatement
    | aiStatement
    | dataStatement
    | networkingStatement
    | securityStatement
    | resourceStatement
    | compileStatement
    | executionStatement
    | interoperabilityStatement
    | dialectStatement
    | macroStatement
    | metaprogrammingStatement
    ;


/* ============================================================================
 * 30. UNIVERSAL EXPRESSION
 * ============================================================================
 *
 * Expression precedence and primary/postfix syntax belong to Expressions.
 *
 * This root does not reproduce them.
 * ============================================================================
 */

expression
    : coreExpression
    | classicalExpression
    | quantumExpression
    | hybridExpression
    | hdlExpression
    | hardwareExpression
    | distributedExpression
    | aiExpression
    | dataExpression
    | networkingExpression
    | securityExpression
    | resourceExpression
    | compileExpression
    | executionExpression
    | interoperabilityExpression
    | dialectExpression
    | macroExpression
    | metaprogrammingExpression
    ;


/* ============================================================================
 * 31. DOMAIN-NEUTRAL SOURCE CONTRACT
 * ============================================================================
 *
 * The following invariants are mandatory for every delegate.
 *
 * A delegate MUST:
 *
 *   1. consume the canonical Zamani lexical vocabulary;
 *   2. avoid defining universal implementation limits;
 *   3. preserve source structure needed by the AST;
 *   4. have an explicit AST mapping;
 *   5. have an explicit semantic mapping;
 *   6. have an explicit IR mapping;
 *   7. define diagnostics for malformed syntax;
 *   8. provide positive tests;
 *   9. provide negative tests;
 *  10. provide boundary tests;
 *  11. provide scalability tests;
 *  12. provide compatibility tests;
 *  13. avoid target-specific implementation decisions;
 *  14. avoid Rust actions;
 *  15. avoid unsafe implementation requirements.
 *
 * ============================================================================
 */


/* ============================================================================
 * 32. POCO-REAF INVARIANT
 * ============================================================================
 *
 * The parser accepts source whose scale is determined by program semantics and
 * available implementation resources, not by constants embedded in this
 * grammar.
 *
 * Therefore this grammar intentionally contains no rules such as:
 *
 *     oneTo1024Qubits
 *     eightCores
 *     sixteenThreads
 *     fourGPUs
 *     thirtyTwoBitRegister
 *     fixedNodeCount
 *     fixedTensorRank
 *
 * Program-defined numeric values remain legal wherever the language semantics
 * require them.
 *
 * ============================================================================
 */


/* ============================================================================
 * 33. CANONICAL DOWNSTREAM CONTRACT
 * ============================================================================
 *
 *                         ZamaniParser
 *                              |
 *                              v
 *                       Domain-neutral AST
 *                              |
 *                              v
 *                     Structural validation
 *                              |
 *                              v
 *                     Semantic analysis
 *                              |
 *                              v
 *                    Canonical semantic model
 *                              |
 *            +-----------------+------------------+
 *            |                 |                  |
 *            v                 v                  v
 *       Classical IR      quantum::ir       HDL/Hardware IR
 *                              |
 *                              v
 *                         optimization
 *                              |
 *                    routing / scheduling
 *                              |
 *                        resilience/QEC
 *                              |
 *                             ZQN
 *                              |
 *                             HAL
 *                              |
 *                       target lowering
 *
 * ============================================================================
 */
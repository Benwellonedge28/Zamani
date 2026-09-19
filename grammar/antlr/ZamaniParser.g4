/**
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *     grammar/antlr/ZamaniParser.g4
 *
 * Status:
 *     CANONICAL PARSER COMPOSITION ROOT
 *
 * Purpose:
 *     Orchestrate the complete Zamani parser grammar.
 *
 * This file is intentionally NOT a leaf grammar.
 *
 * It owns:
 *
 *   - the canonical parser entry point;
 *   - source-unit composition;
 *   - universal declaration dispatch;
 *   - universal statement dispatch;
 *   - universal expression dispatch;
 *   - cross-domain composition;
 *   - integration of the grammar/* ownership tree.
 *
 * It does NOT own:
 *
 *   - lexer rules;
 *   - token definitions;
 *   - expression implementations;
 *   - type implementations;
 *   - declaration-family implementations;
 *   - statement implementations;
 *   - quantum operation implementations;
 *   - HDL implementations;
 *   - hardware realization;
 *   - QEC;
 *   - ZQN;
 *   - routing;
 *   - scheduling;
 *   - calibration;
 *   - HAL;
 *   - runtime execution;
 *   - compiler backend selection.
 *
 * ============================================================================
 * ARCHITECTURAL PIPELINE
 * ============================================================================
 *
 *                    Zamani source
 *                          |
 *                          v
 *                    ZamaniLexer
 *                          |
 *                          v
 *                  ZamaniParser
 *                          |
 *          +---------------+----------------+
 *          |               |                |
 *          v               v                v
 *       Core            Types            Domains
 *          |               |                |
 *          +---------------+----------------+
 *                          |
 *                          v
 *                  Domain-neutral AST
 *                          |
 *                          v
 *               structural validation
 *                          |
 *                          v
 *                 semantic analysis
 *                          |
 *                          v
 *              canonical semantic model
 *                          |
 *             +------------+------------+
 *             |            |            |
 *             v            v            v
 *        Classical     quantum::ir    HDL/Hardware
 *           IR                         semantics
 *             |            |            |
 *             +------------+------------+
 *                          |
 *                          v
 *                    optimization
 *                          |
 *             +------------+------------+
 *             |            |            |
 *             v            v            v
 *          routing     scheduling    resilience
 *                                      |
 *                                      v
 *                                     ZQN
 *                                      |
 *                                      v
 *                                     HAL
 *                                      |
 *                                      v
 *                              target realization
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * This parser imposes NO universal machine-size limits.
 *
 * In particular, this grammar contains no language-level limits on:
 *
 *   CPU count
 *   core count
 *   thread count
 *   GPU count
 *   FPGA count
 *   accelerator count
 *   QPU count
 *   qubit count
 *   register count
 *   memory capacity
 *   storage capacity
 *   node count
 *   process count
 *   task count
 *   channel count
 *   tensor dimensions
 *   tensor rank
 *   vector width
 *   topology size
 *   timeline count
 *   program size
 *
 * Any finite size is therefore governed by:
 *
 *   - source semantics;
 *   - compiler implementation resources;
 *   - runtime resources;
 *   - target capabilities;
 *   - explicitly declared resource requirements.
 *
 * Those are NOT parser constants.
 *
 * ============================================================================
 * QUANTUM INVARIANT
 * ============================================================================
 *
 * All quantum syntax ultimately maps through the domain-neutral AST and
 * semantic analysis to:
 *
 *     quantum::ir
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * This parser MUST NOT introduce:
 *
 *   - QuantumGate enum alternatives;
 *   - physical-qubit allocation;
 *   - topology;
 *   - routing;
 *   - scheduling;
 *   - calibration;
 *   - QEC implementation;
 *   - ZQN implementation.
 *
 * Quantum operation names remain extensible source-level names.
 *
 * ============================================================================
 * SAFETY INVARIANT
 * ============================================================================
 *
 * This grammar contains:
 *
 *   - no embedded target-language actions;
 *   - no semantic predicates;
 *   - no filesystem access;
 *   - no network access;
 *   - no environment inspection;
 *   - no hardware discovery;
 *   - no randomness;
 *   - no runtime execution;
 *   - no unsafe Rust;
 *   - no dependency on Rust implementation details.
 *
 * The Rust implementation consuming this grammar is required to remain:
 *
 *   Rust 2021
 *   Rust 1.97 / 1.97.1
 *   safe Rust only
 *
 * ============================================================================
 * IMPORTANT ANTLR RULE
 * ============================================================================
 *
 * ANTLR imports are grammar-name imports.
 *
 * This file therefore imports ONLY canonical parser-composition grammars.
 *
 * It must NOT attempt imports such as:
 *
 *     import grammar.quantum.operations;
 *
 * or:
 *
 *     import quantum/operations;
 *
 * Leaf grammars are composed by their directory's canonical dispatcher.
 *
 * ============================================================================
 */

parser grammar ZamaniParser;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * CANONICAL COMPOSITION IMPORTS
 * ============================================================================
 *
 * These grammar names are architectural contracts.
 *
 * Each imported grammar must be a parser grammar and must itself compose the
 * leaf grammars belonging to its ownership directory.
 *
 * The root parser deliberately does not import every leaf grammar.
 *
 * This keeps:
 *
 *     leaf grammar
 *          ↓
 *     domain dispatcher
 *          ↓
 *     universal dispatcher
 *          ↓
 *     ZamaniParser
 *
 * as the only composition direction.
 *
 * IMPORTANT:
 *
 * The build system must place the canonical composition grammars in the ANTLR
 * grammar source path before generation.
 *
 * ============================================================================
 */

import
    Core,
    Types,
    Expressions,
    Declarations,
    Statements,
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
 * 1. PUBLIC PROGRAM ENTRY POINT
 * ============================================================================
 *
 * There is exactly one parser entry point for a complete Zamani source unit.
 *
 * No domain is allowed to create another competing program root.
 * ========================================================================== */

program
    : sourceUnit EOF
    ;


/* ============================================================================
 * 2. SOURCE UNIT
 * ============================================================================
 *
 * The source unit is an ordered, unbounded sequence of source elements.
 *
 * The repetition is semantic rather than resource-bounded.
 *
 * ============================================================================
 */

sourceUnit
    : sourceElement*
    ;


/* ============================================================================
 * 3. UNIVERSAL SOURCE ELEMENT
 * ============================================================================
 *
 * All top-level constructs enter through this composition boundary.
 *
 * Detailed syntax belongs to the owning grammar.
 * ========================================================================== */

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
 * 4. CORE
 * ============================================================================
 */

documentationElement
    : coreDocumentation
    ;

attributeElement
    : coreAttribute
    ;

pragmaElement
    : corePragma
    ;


/* ============================================================================
 * 5. MODULE / PACKAGE COMPOSITION
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
 * 6. UNIVERSAL DECLARATIONS
 * ============================================================================
 *
 * `declaration` is supplied by the canonical declaration dispatcher.
 *
 * The root parser does not duplicate individual declaration families.
 * ========================================================================== */

declarationElement
    : declaration
    ;


/* ============================================================================
 * 7. UNIVERSAL STATEMENTS
 * ============================================================================
 */

statementElement
    : statement
    ;


/* ============================================================================
 * 8. UNIVERSAL DOMAIN COMPOSITION
 * ============================================================================
 *
 * Zamani is one language.
 *
 * The following are domains of the same language rather than separate
 * languages:
 *
 *   classical
 *   quantum
 *   hybrid
 *   HDL
 *   hardware
 *   distributed
 *   AI
 *   data
 *   networking
 *   security
 *   resources
 *   compilation
 *   execution
 *   interoperability
 *   dialects
 *   macros
 *   metaprogramming
 *
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
 * 9. CLASSICAL COMPUTING
 * ============================================================================
 *
 * The Classical dispatcher owns:
 *
 *   - scalar computation;
 *   - integer computation;
 *   - floating-point computation;
 *   - vectors;
 *   - matrices;
 *   - tensors;
 *   - symbolic computation;
 *   - numerical computation;
 *   - scientific computation;
 *   - signal processing;
 *   - optimization;
 *   - control;
 *   - mathematical capabilities.
 *
 * Algorithm names do not become an ever-growing parser keyword list.
 *
 * Generic operations such as:
 *
 *     fft(...)
 *     svd(...)
 *     gradient(...)
 *     optimize(...)
 *
 * are resolved semantically through the canonical operation/type model.
 *
 * ============================================================================
 */

classicalElement
    : classicalDeclaration
    | classicalStatement
    | classicalExpression
    ;


/* ============================================================================
 * 10. QUANTUM COMPUTING
 * ============================================================================
 *
 * Quantum owns source syntax for:
 *
 *   - logical qubits;
 *   - registers;
 *   - states;
 *   - operations;
 *   - parameterized operations;
 *   - controls;
 *   - adjoints;
 *   - measurement;
 *   - reset;
 *   - observables;
 *   - channels;
 *   - dynamic circuits;
 *   - classical feed-forward;
 *   - error-correction intent;
 *   - logical-operation intent;
 *   - quantum resources;
 *   - quantum capabilities.
 *
 * It MUST NOT enumerate the complete universe of gates here.
 *
 * ============================================================================
 */

quantumElement
    : quantumDeclaration
    | quantumStatement
    | quantumExpression
    ;


/* ============================================================================
 * 11. HYBRID COMPUTING
 * ============================================================================
 *
 * Hybrid computation is first-class composition of classical and quantum
 * semantics.
 *
 * Example semantic flow:
 *
 *     classical
 *        ↓
 *     quantum
 *        ↓
 *     measurement
 *        ↓
 *     classical decision
 *        ↓
 *     quantum
 *
 * No second hybrid IR is introduced here.
 * ========================================================================== */

hybridElement
    : hybridDeclaration
    | hybridStatement
    | hybridExpression
    ;


/* ============================================================================
 * 12. HDL
 * ============================================================================
 */

hdlElement
    : hdlDeclaration
    | hdlStatement
    | hdlExpression
    ;


/* ============================================================================
 * 13. TARGET-INDEPENDENT HARDWARE INTENT
 * ============================================================================
 */

hardwareElement
    : hardwareDeclaration
    | hardwareStatement
    | hardwareExpression
    ;


/* ============================================================================
 * 14. DISTRIBUTED / PARALLEL COMPUTING
 * ============================================================================
 *
 * No fixed node/core/process/thread count belongs here.
 * ========================================================================== */

distributedElement
    : distributedDeclaration
    | distributedStatement
    | distributedExpression
    ;


/* ============================================================================
 * 15. AI / MACHINE LEARNING
 * ============================================================================
 *
 * Framework-neutral.
 *
 * The grammar may express:
 *
 *   model
 *   dataset
 *   tensor
 *   training
 *   inference
 *   differentiability
 *   probabilistic computation
 *   agents
 *   model deployment
 *
 * but does not become a CUDA/PyTorch/TensorFlow/vendor grammar.
 * ========================================================================== */

aiElement
    : aiDeclaration
    | aiStatement
    | aiExpression
    ;


/* ============================================================================
 * 16. DATA
 * ============================================================================
 */

dataElement
    : dataDeclaration
    | dataStatement
    | dataExpression
    ;


/* ============================================================================
 * 17. NETWORKING
 * ============================================================================
 */

networkingElement
    : networkingDeclaration
    | networkingStatement
    | networkingExpression
    ;


/* ============================================================================
 * 18. SECURITY
 * ============================================================================
 */

securityElement
    : securityDeclaration
    | securityStatement
    | securityExpression
    ;


/* ============================================================================
 * 19. RESOURCES / CAPABILITIES
 * ============================================================================
 *
 * Resource syntax expresses semantic intent.
 *
 * It must distinguish:
 *
 *     requirement
 *     constraint
 *     capability
 *     preference
 *     hint
 *     budget
 *     negotiation
 *
 * from downstream implementation decisions.
 *
 * Examples of portable intent:
 *
 *     requires qubits >= n
 *     requires capability("quantum.measurement")
 *     requires memory(...)
 *
 * The parser does not discover whether the machine satisfies those requests.
 *
 * ============================================================================
 */

resourceElement
    : resourceDeclaration
    | resourceStatement
    | resourceExpression
    ;


/* ============================================================================
 * 20. COMPILATION
 * ============================================================================
 *
 * Compilation syntax describes intent and policy.
 *
 * It does not select a physical machine during parsing.
 * ========================================================================== */

compileElement
    : compileDeclaration
    | compileStatement
    | compileExpression
    ;


/* ============================================================================
 * 21. EXECUTION
 * ============================================================================
 *
 * Runtime intent is kept separate from physical realization.
 * ========================================================================== */

executionElement
    : executionDeclaration
    | executionStatement
    | executionExpression
    ;


/* ============================================================================
 * 22. INTEROPERABILITY
 * ============================================================================
 *
 * OpenQASM, QIR, HDL, C, C++, Rust, WASM and other representations belong
 * here as interoperability surfaces.
 *
 * They are NOT the canonical Zamani semantic model.
 * ========================================================================== */

interoperabilityElement
    : interoperabilityDeclaration
    | interoperabilityStatement
    | interoperabilityExpression
    ;


/* ============================================================================
 * 23. DIALECTS
 * ============================================================================
 *
 * Dialects are controlled extensions to the common language.
 *
 * A dialect must ultimately map to the canonical AST/semantic model/IR.
 *
 * A dialect cannot silently redefine stable core semantics.
 * ========================================================================== */

dialectElement
    : dialectDeclaration
    | dialectStatement
    | dialectExpression
    ;


/* ============================================================================
 * 24. MACROS
 * ============================================================================
 *
 * Macro syntax is parsed as source syntax.
 *
 * Expansion and hygiene occur downstream.
 *
 * Expanded source must re-enter the ordinary AST and semantic validation
 * pipeline.
 * ========================================================================== */

macroElement
    : macroDeclaration
    | macroStatement
    | macroExpression
    ;


/* ============================================================================
 * 25. METAPROGRAMMING
 * ============================================================================
 *
 * Reflection, quotation, generation and compile-time facilities remain
 * controlled language constructs.
 *
 * They cannot bypass:
 *
 *   - type checking;
 *   - effect checking;
 *   - capability checking;
 *   - resource checking;
 *   - ownership checking;
 *   - semantic validation.
 * ========================================================================== */

metaprogrammingElement
    : metaprogrammingDeclaration
    | metaprogrammingStatement
    | metaprogrammingExpression
    ;


/* ============================================================================
 * 26. CROSS-DOMAIN DECLARATION CONTRACT
 * ============================================================================
 *
 * This rule exists only where the imported declaration dispatcher exposes the
 * canonical universal declaration contract.
 *
 * No declaration implementation belongs in this file.
 *
 * ============================================================================
 */

universalDeclaration
    : declaration
    ;


/* ============================================================================
 * 27. CROSS-DOMAIN STATEMENT CONTRACT
 * ============================================================================
 *
 * No statement implementation belongs here.
 * ========================================================================== */

universalStatement
    : statement
    ;


/* ============================================================================
 * 28. CROSS-DOMAIN EXPRESSION CONTRACT
 * ============================================================================
 *
 * Expression precedence and leaf expression syntax are owned by
 * grammar/expressions/.
 *
 * This root never creates a second expression grammar.
 * ========================================================================== */

universalExpression
    : expression
    ;


/* ============================================================================
 * 29. CROSS-DOMAIN TYPE CONTRACT
 * ============================================================================
 *
 * Type syntax is owned by grammar/types/.
 *
 * Semantic validity is downstream.
 * ========================================================================== */

universalType
    : typeExpression
    ;


/* ============================================================================
 * 30. UNIVERSAL BLOCK
 * ============================================================================
 *
 * Blocks are supplied by the canonical statement/core composition.
 * ========================================================================== */

universalBlock
    : block
    ;


/* ============================================================================
 * 31. UNIVERSAL CALLABLE
 * ============================================================================
 *
 * Function syntax remains owned by grammar/functions/.
 * ========================================================================== */

universalFunction
    : functionDeclaration
    ;


/* ============================================================================
 * 32. UNIVERSAL MODULE
 * ============================================================================
 */

universalModule
    : moduleDeclaration
    ;


/* ============================================================================
 * 33. UNIVERSAL EFFECT
 * ============================================================================
 */

universalEffect
    : effectDeclaration
    ;


/* ============================================================================
 * 34. UNIVERSAL RESOURCE CONTRACT
 * ============================================================================
 */

universalResource
    : resourceDeclaration
    ;


/* ============================================================================
 * 35. UNIVERSAL HARDWARE CONTRACT
 * ============================================================================
 *
 * Hardware intent is deliberately separated from actual target realization.
 * ========================================================================== */

universalHardware
    : hardwareDeclaration
    ;


/* ============================================================================
 * 36. UNIVERSAL QUANTUM CONTRACT
 * ============================================================================
 *
 * This provides a stable parser-level integration point for tooling that
 * wants to identify a quantum declaration without creating another quantum
 * semantic representation.
 * ========================================================================== */

universalQuantum
    : quantumDeclaration
    ;


/* ============================================================================
 * 37. UNIVERSAL HDL CONTRACT
 * ============================================================================
 */

universalHDL
    : hdlDeclaration
    ;


/* ============================================================================
 * 38. UNIVERSAL DISTRIBUTED CONTRACT
 * ============================================================================
 */

universalDistributed
    : distributedDeclaration
    ;


/* ============================================================================
 * 39. UNIVERSAL AI CONTRACT
 * ============================================================================
 */

universalAI
    : aiDeclaration
    ;


/* ============================================================================
 * 40. UNIVERSAL DATA CONTRACT
 * ============================================================================
 */

universalData
    : dataDeclaration
    ;


/* ============================================================================
 * 41. UNIVERSAL NETWORK CONTRACT
 * ============================================================================
 */

universalNetworking
    : networkingDeclaration
    ;


/* ============================================================================
 * 42. UNIVERSAL SECURITY CONTRACT
 * ============================================================================
 */

universalSecurity
    : securityDeclaration
    ;


/* ============================================================================
 * 43. UNIVERSAL COMPILATION CONTRACT
 * ============================================================================
 */

universalCompilation
    : compileDeclaration
    ;


/* ============================================================================
 * 44. UNIVERSAL EXECUTION CONTRACT
 * ============================================================================
 */

universalExecution
    : executionDeclaration
    ;


/* ============================================================================
 * 45. UNIVERSAL INTEROPERABILITY CONTRACT
 * ============================================================================
 */

universalInteroperability
    : interoperabilityDeclaration
    ;


/* ============================================================================
 * 46. UNIVERSAL DIALECT CONTRACT
 * ============================================================================
 */

universalDialect
    : dialectDeclaration
    ;


/* ============================================================================
 * 47. UNIVERSAL MACRO CONTRACT
 * ============================================================================
 */

universalMacro
    : macroDeclaration
    ;


/* ============================================================================
 * 48. UNIVERSAL METAPROGRAMMING CONTRACT
 * ============================================================================
 */

universalMetaprogramming
    : metaprogrammingDeclaration
    ;
/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/macros/macros.g4
 *
 * Grammar:
 *     Macros
 *
 * Status:
 *     CANONICAL MACRO GRAMMAR COMPOSITION ROOT
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     Safe Rust only.
 *     No unsafe Rust.
 *     No embedded Rust actions.
 *     No semantic predicates.
 *     No host-language execution.
 *
 * ============================================================================
 * 1. PURPOSE
 * ============================================================================
 *
 * This file is the SINGLE composition boundary for the Zamani macro grammar.
 *
 * It composes the independently owned macro grammar components:
 *
 *     MacroDeclarations
 *     MacroParameters
 *     invocations
 *     expansion
 *     hygiene
 *     diagnostics
 *     syntaxTree
 *
 * It does not reimplement their productions.
 *
 * The canonical parser composition root:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * imports this grammar through:
 *
 *     import Macros;
 *
 * Therefore the grammar identity MUST remain:
 *
 *     Macros
 *
 * Do not change this identity without updating the canonical parser
 * composition contract and compatibility metadata.
 *
 * ============================================================================
 * 2. ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         Zamani source
 *                              |
 *                              v
 *                         ZamaniLexer
 *                              |
 *                              v
 *                       ZamaniParser
 *                              |
 *                              v
 *                            Macros
 *                              |
 *          +-------------------+-------------------+
 *          |                   |                   |
 *          v                   v                   v
 *   declarations         invocations         metadata
 *          |                   |                   |
 *          |                   |          +--------+---------+
 *          |                   |          |        |         |
 *          |                   |          v        v         v
 *          |                   |      expansion hygiene diagnostics
 *          |                   |
 *          |                   +-------------------------+
 *          |                                             |
 *          +-------------------+-------------------------+
 *                              |
 *                              v
 *                       frontend AST
 *                              |
 *                              v
 *                       macro resolution
 *                              |
 *                              v
 *                     controlled expansion
 *                              |
 *                              v
 *                   hygiene / provenance
 *                              |
 *                              v
 *                    semantic analysis
 *                              |
 *                              v
 *                   canonical semantic model
 *                              |
 *             +----------------+----------------+
 *             |                |                |
 *             v                v                v
 *        classical         quantum::ir     HDL/hardware
 *             |                |                |
 *             +----------------+----------------+
 *                              |
 *                              v
 *                     optimization / lowering
 *                              |
 *                     routing / scheduling
 *                              |
 *                     resilience / QEC / ZQN
 *                              |
 *                             HAL
 *                              |
 *                       target realization
 *
 * Macro expansion is therefore a frontend/compiler transformation.
 *
 * It is NOT an ANTLR parser action.
 *
 * ============================================================================
 * 3. SINGLE-AUTHORITY RULE
 * ============================================================================
 *
 * Exactly one grammar component owns each macro production.
 *
 * Ownership:
 *
 *     macroDeclaration
 *         -> MacroDeclarations
 *
 *     macroParameterList
 *         -> MacroParameters
 *
 *     macroParameter
 *         -> MacroParameters
 *
 *     macroParameterDefault
 *         -> MacroParameters
 *
 *     macroPath
 *         -> invocations
 *
 *     macroInvocation
 *         -> invocations
 *
 *     macroExpression
 *         -> invocations
 *
 *     macro expansion metadata
 *         -> expansion
 *
 *     macro hygiene metadata
 *         -> hygiene
 *
 *     macro diagnostic metadata
 *         -> diagnostics
 *
 *     syntax-tree adapter
 *         -> syntaxTree
 *
 *     balanced token-tree syntax
 *         -> tokenStream
 *            through syntaxTree's canonical import
 *
 *     macro composition
 *         -> Macros
 *
 * This file MUST NOT redefine any of those productions.
 *
 * ============================================================================
 * 4. IMPORTANT CORRECTION: NO macroStatement
 * ============================================================================
 *
 * The previous composition contract referenced:
 *
 *     macroStatement
 *
 * but the inspected canonical macro components do not provide a
 * macroStatement production.
 *
 * Macro declarations and macro invocations are sufficient for the current
 * macro source-language boundary:
 *
 *     macroDeclaration
 *     macroExpression
 *
 * A future macro-specific statement form must first receive an independent
 * language specification and ownership contract.
 *
 * It MUST NOT be invented here merely to satisfy composition.
 *
 * Therefore:
 *
 *     macroElement
 *         -> macroDeclaration
 *         |  macroExpression
 *
 * This prevents an unresolved parser-rule reference and avoids creating a
 * speculative macro statement language.
 *
 * ============================================================================
 * 5. CANONICAL LEXER CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * The canonical production lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Parser grammars consume:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * This file therefore MUST NOT:
 *
 *     - define lexer tokens;
 *     - define keyword spellings;
 *     - define punctuation;
 *     - define operators;
 *     - define identifiers;
 *     - define literals;
 *     - create a second lexer vocabulary.
 *
 * In particular, this file MUST continue to use:
 *
 *     ZamaniLexer
 *
 * rather than:
 *
 *     ZamaniTokens
 *
 * directly.
 *
 * ZamaniTokens is the lexical composition vocabulary.
 *
 * ZamaniLexer is the canonical production lexer consumed by parser grammars.
 *
 * ============================================================================
 * 6. SHARED RULE CONTRACT
 * ============================================================================
 *
 * Macro components intentionally reference canonical rules owned elsewhere.
 *
 * Examples include:
 *
 *     identifier
 *     qualifiedName
 *     visibilityModifier
 *     genericParameters
 *     typeExpression
 *     expression
 *     argumentList
 *     blockExpression
 *     annotation
 *
 * Those rules MUST NOT be copied into this file.
 *
 * Their canonical owners remain responsible for their syntax.
 *
 * ============================================================================
 * 7. PARAMETER-GRAMMAR INTEGRATION
 * ============================================================================
 *
 * The repository contains:
 *
 *     grammar/macros/parameters.g4
 *
 * whose grammar identity is:
 *
 *     MacroParameters
 *
 * That file is the intended canonical owner of:
 *
 *     macroParameterList
 *     macroParameter
 *     macroParameterDefault
 *
 * Therefore MacroDeclarations MUST consume those rules from
 * MacroParameters rather than redefining them.
 *
 * The required integration is:
 *
 *     MacroDeclarations
 *             |
 *             +--> MacroParameters
 *
 * followed by:
 *
 *     Macros
 *             |
 *             +--> MacroDeclarations
 *
 * This produces one ownership chain instead of duplicate parameter rules.
 *
 * ============================================================================
 * 8. TOKEN-TREE INTEGRATION
 * ============================================================================
 *
 * The repository also contains:
 *
 *     grammar/macros/token-stream.g4
 *
 * with grammar identity:
 *
 *     tokenStream
 *
 * and:
 *
 *     grammar/macros/syntax-tree.g4
 *
 * with grammar identity:
 *
 *     syntaxTree
 *
 * syntaxTree already composes the canonical token-tree grammar.
 *
 * Therefore this composition root imports:
 *
 *     syntaxTree
 *
 * rather than independently importing tokenStream as a second path.
 *
 * This prevents unnecessary duplicate composition paths.
 *
 * The canonical token-tree ownership remains:
 *
 *     tokenStream
 *
 * while the macro syntax-tree adapter remains:
 *
 *     syntaxTree
 *
 * ============================================================================
 * 9. DIAGNOSTIC INTEGRATION
 * ============================================================================
 *
 * diagnostics.g4 owns parser-level macro diagnostic metadata.
 *
 * It does NOT own compiler diagnostics themselves.
 *
 * Importing diagnostics here makes its parser contexts reachable from the
 * canonical macro grammar without making diagnostics part of the macro
 * execution mechanism.
 *
 * Diagnostics remain downstream concerns for:
 *
 *     lexer errors
 *     parser errors
 *     macro-resolution errors
 *     expansion errors
 *     hygiene errors
 *     provenance errors
 *     semantic errors
 *     resource failures
 *     capability failures
 *     quantum failures
 *     HDL failures
 *     hardware realization failures
 *     routing failures
 *     scheduling failures
 *     QEC failures
 *     ZQN/resilience failures
 *     backend failures
 *
 * The grammar only preserves source-level diagnostic metadata where the
 * language specification permits it.
 *
 * ============================================================================
 * 10. EXPANSION INTEGRATION
 * ============================================================================
 *
 * expansion.g4 owns expansion metadata syntax.
 *
 * It does NOT:
 *
 *     - execute macros;
 *     - evaluate expressions;
 *     - generate source;
 *     - mutate the AST;
 *     - perform recursion detection;
 *     - select expansion order;
 *     - inspect hardware;
 *     - access files;
 *     - access networks;
 *     - invoke processes.
 *
 * Those responsibilities belong to the controlled compiler macro engine.
 *
 * ============================================================================
 * 11. HYGIENE INTEGRATION
 * ============================================================================
 *
 * hygiene.g4 owns parser-level hygiene metadata boundaries.
 *
 * Hygiene semantics remain downstream.
 *
 * The grammar does not:
 *
 *     - assign binding identities;
 *     - resolve captures;
 *     - create scopes;
 *     - generate fresh identifiers;
 *     - bypass visibility;
 *     - bypass ownership;
 *     - bypass type checking;
 *     - bypass effects;
 *     - bypass capabilities;
 *     - bypass resources;
 *     - authorize filesystem/network/process access.
 *
 * Hygiene metadata must ultimately preserve source provenance and expansion
 * context through the frontend AST.
 *
 * ============================================================================
 * 12. DOMAIN NEUTRALITY
 * ============================================================================
 *
 * Macro syntax is domain-neutral.
 *
 * A macro may generate source belonging to:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     distributed
 *     AI
 *     data
 *     networking
 *     security
 *     scientific computing
 *     accelerator computation
 *     future Zamani domains
 *
 * This file MUST NOT introduce:
 *
 *     quantumMacro
 *     classicalMacro
 *     gpuMacro
 *     qpuMacro
 *     fpgaMacro
 *     hdlMacro
 *     hardwareMacro
 *     distributedMacro
 *
 * merely to support those domains.
 *
 * The generated source re-enters the ordinary language semantic pipeline.
 *
 * ============================================================================
 * 13. QUANTUM INTEGRATION
 * ============================================================================
 *
 * A macro may generate quantum source syntax.
 *
 * This grammar does NOT:
 *
 *     - enumerate quantum gates;
 *     - allocate qubits;
 *     - select physical qubits;
 *     - select a QPU;
 *     - select a native gate set;
 *     - perform routing;
 *     - perform scheduling;
 *     - perform QEC;
 *     - implement ZQN;
 *     - select calibration data;
 *     - construct quantum::ir.
 *
 * The resulting expanded quantum source eventually follows:
 *
 *     macro expansion
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing / scheduling / resilience / QEC / ZQN
 *          |
 *          v
 *     HAL
 *
 * No second macro-specific quantum IR is permitted.
 *
 * ============================================================================
 * 14. CLASSICAL / HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * The same macro syntax may generate:
 *
 *     classical computations
 *     tensor computations
 *     HDL constructs
 *     hardware intent
 *     accelerator intent
 *     distributed computations
 *     networking operations
 *     AI/data pipelines
 *
 * The macro layer does not need target-specific grammar branches.
 *
 * Target-independent semantics remain target-independent until the appropriate
 * compiler lowering phase.
 *
 * ============================================================================
 * 15. POCO-REAF
 * ============================================================================
 *
 * Macro syntax MUST remain compatible with:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * The grammar therefore MUST NOT encode universal hardware ceilings such as:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * It also MUST NOT introduce equivalent disguised limits.
 *
 * Examples of prohibited grammar concepts include:
 *
 *     "macro may have at most N parameters"
 *     "macro expansion may contain at most N nodes"
 *     "macro namespace may be at most N segments"
 *     "macro nesting may be at most N levels"
 *
 * Repetition and recursion are therefore structural grammar mechanisms.
 *
 * Actual finite limits are implementation/resource-policy concerns.
 *
 * ============================================================================
 * 16. RESOURCE / CAPABILITY SEPARATION
 * ============================================================================
 *
 * A macro may generate semantic resource requirements.
 *
 * For example, generated source may eventually express:
 *
 *     requires qubits >= n
 *     requires memory >= required_memory
 *     requires capability("quantum.measurement")
 *     requires capability("tensor.compute")
 *     requires topology(...)
 *
 * Those meanings belong to semantic/resource/capability analysis.
 *
 * This grammar does not turn such requirements into physical placement.
 *
 * In particular:
 *
 *     semantic requirement
 *
 * is distinct from:
 *
 *     implementation decision
 *
 * and:
 *
 *     physical resource identity
 *
 * A macro invocation MUST NOT silently convert a portable requirement into a
 * hard-coded device assignment.
 *
 * ============================================================================
 * 17. SIDE-EFFECT BOUNDARY
 * ============================================================================
 *
 * Parsing MUST be side-effect free.
 *
 * This grammar cannot:
 *
 *     - read files;
 *     - write files;
 *     - access networks;
 *     - spawn processes;
 *     - inspect environment variables;
 *     - access secrets;
 *     - inspect hardware;
 *     - access QPU state;
 *     - access compiler-global mutable state;
 *     - execute macro code.
 *
 * Macro execution belongs to a separate controlled compiler phase.
 *
 * ============================================================================
 * 18. DETERMINISM
 * ============================================================================
 *
 * Given the same canonical token stream and parser configuration, the grammar
 * must produce the same structural parse.
 *
 * This file contains:
 *
 *     - no actions;
 *     - no semantic predicates;
 *     - no randomness;
 *     - no time-dependent behavior;
 *     - no filesystem queries;
 *     - no network queries;
 *     - no hardware discovery.
 *
 * Expansion determinism is a separate compiler contract.
 *
 * The downstream macro engine must define deterministic behavior for:
 *
 *     resolution
 *     parameter binding
 *     expansion ordering
 *     recursion handling
 *     hygiene
 *     provenance
 *     diagnostics
 *     reproducibility
 *
 * ============================================================================
 * 19. AST CONTRACT
 * ============================================================================
 *
 * This file defines no AST structures.
 *
 * Imported parser contexts are lowered into the repository's canonical
 * frontend AST.
 *
 * The macro subsystem MUST NOT create a competing:
 *
 *     MacroAst
 *     MacroInvocationAst
 *     MacroExpansionAst
 *     MacroSyntaxTreeAst
 *
 * hierarchy merely because grammar components are separated.
 *
 * The frontend representation must preserve, where applicable:
 *
 *     - source span;
 *     - source identity;
 *     - macro declaration identity;
 *     - invocation identity;
 *     - qualified path;
 *     - parameter order;
 *     - argument order;
 *     - generic arguments;
 *     - body structure;
 *     - expansion provenance;
 *     - hygiene metadata;
 *     - token-tree structure.
 *
 * ============================================================================
 * 20. SEMANTIC CONTRACT
 * ============================================================================
 *
 * This grammar establishes syntax only.
 *
 * Semantic analysis remains responsible for:
 *
 *     macro existence
 *     visibility
 *     path resolution
 *     argument compatibility
 *     generic constraints
 *     default validity
 *     recursion policy
 *     expansion authorization
 *     hygiene
 *     provenance
 *     capability requirements
 *     resource requirements
 *     effect checking
 *     type checking
 *     generated quantum semantics
 *     generated HDL semantics
 *     generated hardware intent
 *     portability
 *
 * Parsing a macro does NOT grant any capability.
 *
 * Parsing a macro does NOT make an unavailable resource available.
 *
 * Parsing a macro does NOT authorize host execution.
 *
 * ============================================================================
 * 21. IR CONTRACT
 * ============================================================================
 *
 * This file creates no IR.
 *
 * Macro processing must eventually produce ordinary Zamani source semantics
 * which enter the canonical semantic/IR pipeline.
 *
 * The canonical quantum boundary remains:
 *
 *     quantum::ir
 *
 * There is no:
 *
 *     macroQuantumIr
 *
 * or other competing domain IR introduced by this grammar.
 *
 * ============================================================================
 * 22. HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * No production in this file may depend on:
 *
 *     CPU model
 *     GPU model
 *     FPGA family
 *     ASIC family
 *     QPU model
 *     simulator implementation
 *     accelerator inventory
 *     physical qubit identifier
 *     physical memory bank
 *     processor width
 *     network node identifier
 *     machine topology
 *     calibration data
 *
 * Hardware realization remains downstream.
 *
 * ============================================================================
 * 23. COMPATIBILITY
 * ============================================================================
 *
 * Existing macro source forms remain the compatibility baseline:
 *
 *     macro name() { }
 *     macro name(value) { }
 *     macro name(a, b) { }
 *     macro name<T>(value: T) { }
 *
 * and:
 *
 *     build!()
 *     build!(value)
 *     build!(a, b)
 *     math::build!(value)
 *     package::math::build!(a, b)
 *
 * This composition root does not change those forms.
 *
 * The important compatibility correction is that the grammar identity is:
 *
 *     Macros
 *
 * because the inspected canonical parser imports:
 *
 *     Macros
 *
 * ============================================================================
 * 24. CANONICAL COMPOSITION
 * ============================================================================
 *
 * This grammar imports the macro components by their EXISTING grammar
 * identities.
 *
 * Do not rename the existing files merely to make their identities match.
 *
 * Existing identities are retained:
 *
 *     MacroDeclarations
 *     invocations
 *     expansion
 *     hygiene
 *     diagnostics
 *     syntaxTree
 *
 * Parameter ownership is reached transitively through:
 *
 *     MacroDeclarations
 *         -> MacroParameters
 *
 * Syntax-tree token ownership is reached transitively through:
 *
 *     syntaxTree
 *         -> tokenStream
 *
 * This gives one composition path for each responsibility.
 *
 * ============================================================================
 * 25. PUBLIC COMPOSITION RULE
 * ============================================================================
 *
 * The canonical macro source boundary is:
 *
 *     macroElement
 *
 * It represents a macro construct that can occur in the universal domain
 * dispatcher.
 *
 * It deliberately contains only constructs that currently have canonical
 * ownership:
 *
 *     macroDeclaration
 *     macroExpression
 *
 * No speculative macro statement production is included.
 *
 * ============================================================================
 * 26. INTEGRATION WITH ZamaniParser.g4
 * ============================================================================
 *
 * The inspected canonical parser already contains:
 *
 *     import ... Macros, ...
 *
 * and:
 *
 *     macroElement
 *         : macroDeclaration
 *         | macroStatement
 *         | macroExpression
 *         ;
 *
 * This file corrects the missing grammar identity and the missing
 * macroStatement production by making the canonical macro composition expose:
 *
 *     macroElement
 *         : macroDeclaration
 *         | macroExpression
 *         ;
 *
 * However, because `macroElement` is currently owned by ZamaniParser.g4,
 * the final integration must use ONE owner.
 *
 * Therefore the preferred final ownership is:
 *
 *     ZamaniParser.g4
 *         owns universal macro dispatch
 *
 *     Macros
 *         owns macro-subsystem composition
 *
 * The universal parser's existing `macroElement` should be changed to:
 *
 *     macroElement
 *         : macroDeclaration
 *         | macroExpression
 *         ;
 *
 * It must NOT duplicate any imported macro production.
 *
 * ============================================================================
 * 27. INTEGRATION WITH expressions/macros.g4
 * ============================================================================
 *
 * The inspected repository already has:
 *
 *     grammar/expressions/macros.g4
 *
 * as the expression-side macro integration boundary.
 *
 * That file owns expression integration documentation and must continue to
 * consume:
 *
 *     macroExpression
 *
 * rather than reconstructing:
 *
 *     macroPath
 *     BANG
 *     LPAREN
 *     argumentList
 *     RPAREN
 *
 * This keeps invocation syntax owned exclusively by:
 *
 *     grammar/macros/invocations.g4
 *
 * ============================================================================
 * 28. INTEGRATION WITH CORE NAMES / ANNOTATIONS
 * ============================================================================
 *
 * Macro components consume canonical:
 *
 *     identifier
 *     qualifiedName
 *     annotation
 *
 * They must not define macro-specific copies.
 *
 * Consequently, macro metadata remains extensible without requiring a new
 * lexer keyword every time a macro facility is introduced.
 *
 * This is important for future:
 *
 *     quantum
 *     HDL
 *     hardware
 *     AI
 *     distributed
 *     security
 *     interoperability
 *     dialect
 *
 * extensions.
 *
 * ============================================================================
 * 29. INTEGRATION WITH RUST
 * ============================================================================
 *
 * This grammar contains no Rust implementation.
 *
 * The generated parser integration must remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * The compiler implementation must use safe Rust.
 *
 * No `unsafe` Rust is required by this grammar or by its architecture.
 *
 * Rust-side resource policies must remain explicit, configurable, and
 * separate from language-level grammar semantics.
 *
 * ============================================================================
 * 30. TEST CONTRACT
 * ============================================================================
 *
 * Composition tests must cover:
 *
 * DECLARATIONS
 *
 *     macro empty() { }
 *     macro one(value) { }
 *     macro many(a, b, c) { }
 *     macro typed(value: T) { }
 *     macro defaulted(value = expression) { }
 *     macro typedDefaulted(value: T = expression) { }
 *     macro generic<T>(value: T) { }
 *
 * INVOCATIONS
 *
 *     build!()
 *     build!(value)
 *     build!(a, b)
 *     math::build!(value)
 *     package::math::build!(a, b)
 *
 * HYGIENE / EXPANSION / DIAGNOSTICS
 *
 *     canonical annotation metadata
 *     qualified annotation metadata
 *     nested annotation values
 *     expansion metadata
 *     hygiene metadata
 *     diagnostic metadata
 *
 * TOKEN TREES
 *
 *     empty balanced groups
 *     nested groups
 *     mixed delimiter groups
 *     macro-like token sequences
 *
 * CROSS-DOMAIN
 *
 *     macro-generated classical syntax
 *     macro-generated quantum syntax
 *     macro-generated hybrid syntax
 *     macro-generated HDL syntax
 *     macro-generated hardware intent
 *     macro-generated distributed syntax
 *     macro-generated AI/data syntax
 *     macro-generated networking syntax
 *     macro-generated security syntax
 *
 * ============================================================================
 * 31. NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * Required rejection coverage includes:
 *
 *     malformed macro declarations
 *     malformed macro invocations
 *     malformed paths
 *     malformed arguments
 *     malformed annotations
 *     malformed token-tree delimiters
 *     unmatched delimiters
 *     crossed delimiters
 *     unresolved grammar ownership
 *     duplicate parameter ownership
 *
 * Semantic tests must additionally verify that parsed macro syntax does NOT
 * automatically grant:
 *
 *     capabilities
 *     resource availability
 *     hardware access
 *     filesystem access
 *     network access
 *     process execution
 *     secret access
 *     target selection
 *     quantum-device access
 *
 * ============================================================================
 * 32. SCALABILITY TEST CONTRACT
 * ============================================================================
 *
 * The grammar imposes no finite language-level maxima for:
 *
 *     macro count
 *     parameter count
 *     argument count
 *     namespace depth
 *     body size
 *     token-tree size
 *     annotation count
 *     expansion result size
 *     source size
 *
 * Tests should scale dimensions upward subject to the test runner's available
 * resources.
 *
 * Compiler admission policies may separately bound:
 *
 *     source bytes
 *     tokens
 *     AST memory
 *     expansion work
 *     generated nodes
 *     compilation time
 *     compiler memory
 *
 * Those are implementation policies, not grammar semantics.
 *
 * ============================================================================
 * 33. DETERMINISM TEST CONTRACT
 * ============================================================================
 *
 * Repeated parsing of identical canonical token streams must produce
 * structurally equivalent parser results.
 *
 * Tests must not depend on:
 *
 *     wall-clock time
 *     random numbers
 *     filesystem ordering
 *     network state
 *     hardware availability
 *     backend availability
 *
 * ============================================================================
 * 34. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file must contain:
 *
 *     ZERO hardware capacity constants
 *     ZERO universal macro-count ceilings
 *     ZERO fixed expansion-size language limits
 *     ZERO target identifiers
 *     ZERO physical resource assignments
 *
 * In particular, none of the following may occur as language limits:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * ============================================================================
 * 35. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] Grammar identity is `Macros`.
 * [x] `tokenVocab = ZamaniLexer`.
 * [x] Macro declarations are imported.
 * [x] Macro parameters are reachable through MacroDeclarations.
 * [x] Macro invocations are imported.
 * [x] Expansion metadata is imported.
 * [x] Hygiene metadata is imported.
 * [x] Diagnostic metadata is imported.
 * [x] Syntax-tree support is imported.
 * [x] Token-tree support is reached through syntaxTree.
 * [x] No macro production is duplicated here.
 * [x] No lexer rule is duplicated here.
 * [x] No `macroStatement` is invented.
 * [x] Macro syntax remains domain-neutral.
 * [x] No hardware capacity is encoded.
 * [x] No quantum capacity is encoded.
 * [x] No physical target is selected.
 * [x] No macro execution occurs during parsing.
 * [x] No Rust action occurs during parsing.
 * [x] No unsafe Rust is required.
 * [x] `quantum::ir` remains downstream and canonical.
 *
 * Repository integration is complete when:
 *
 * [ ] MacroDeclarations imports MacroParameters and removes its duplicate
 *     parameter productions.
 *
 * [ ] ZamaniParser.g4 changes its macro dispatch from:
 *
 *         macroDeclaration
 *         | macroStatement
 *         | macroExpression
 *
 *     to:
 *
 *         macroDeclaration
 *         | macroExpression
 *
 * [ ] ZamaniParser.g4 imports this grammar using the identity `Macros`.
 *
 * [ ] expressions/macros.g4 consumes the canonical macroExpression.
 *
 * [ ] ANTLR generation succeeds using the canonical ZamaniLexer vocabulary.
 *
 * [ ] No duplicate macro productions remain in grammar/antlr/Core.g4 or
 *     grammar/antlr/Meta.g4.
 *
 * [ ] AST lowering preserves source spans and macro provenance.
 *
 * [ ] Macro expansion remains a controlled compiler phase.
 *
 * [ ] Positive, negative, boundary, scalability, compatibility, and
 *     determinism tests pass.
 *
 * ============================================================================
 * 36. FINAL ARCHITECTURAL GUARANTEE
 * ============================================================================
 *
 * The macro grammar is therefore:
 *
 *     syntax-only
 *     domain-neutral
 *     target-independent
 *     resource-independent
 *     capability-neutral
 *     deterministic
 *     provenance-preserving
 *     compositional
 *
 * while allowing macro-generated programs to participate in:
 *
 *     classical computing
 *     quantum computing
 *     hybrid computing
 *     HDL
 *     hardware/software co-design
 *     distributed computing
 *     AI/data computation
 *     networking
 *     security
 *     future Zamani domains
 *
 * without introducing separate languages or fixed hardware assumptions.
 *
 * The macro subsystem consequently preserves:
 *
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * ============================================================================
 * END OF MACRO COMPOSITION CONTRACT
 * ============================================================================
 */

parser grammar Macros;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * CANONICAL MACRO COMPONENT COMPOSITION
 * ============================================================================
 *
 * The imported grammar identities are intentionally retained exactly as they
 * exist in the repository.
 *
 * Do not replace these with filesystem paths.
 *
 * Do not introduce a second macro root.
 * ============================================================================
 */

import
    MacroDeclarations,
    invocations,
    expansion,
    hygiene,
    diagnostics,
    syntaxTree
;


/*
 * ============================================================================
 * UNIVERSAL MACRO ELEMENT
 * ============================================================================
 *
 * This is the only macro-subsystem dispatch rule owned by this composition
 * root.
 *
 * Macro statements are intentionally absent because the repository currently
 * has no canonical macroStatement owner.
 *
 * If a future macro statement is added, it must receive its own independent
 * grammar component and AST/semantic contract before being added here.
 * ============================================================================
 */

macroElement
    : macroDeclaration
    | macroExpression
    ;
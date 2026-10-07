/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/macros/syntax-tree.g4
 *
 * GRAMMAR IDENTITY
 * ----------------
 * syntaxTree
 *
 * STATUS
 * ------
 * CANONICAL MACRO SYNTAX-TREE ADAPTER
 *
 * PURPOSE
 * -------
 * Provide the canonical parser boundary between the macro subsystem and the
 * repository's canonical balanced token-tree representation.
 *
 * This grammar owns ONLY the syntax-tree adapter boundary.
 *
 * It does not own:
 *
 *   - lexical tokens;
 *   - token text;
 *   - delimiter definitions;
 *   - token-tree construction;
 *   - ordinary Zamani expressions;
 *   - ordinary Zamani statements;
 *   - ordinary Zamani declarations;
 *   - types;
 *   - macro declarations;
 *   - macro invocations;
 *   - macro expansion;
 *   - hygiene;
 *   - name resolution;
 *   - semantic analysis;
 *   - contracts;
 *   - effects;
 *   - capabilities;
 *   - resource selection;
 *   - policies;
 *   - target selection;
 *   - IR construction;
 *   - execution.
 *
 * The canonical token-tree representation remains owned by:
 *
 *     grammar/macros/token-stream.g4
 *
 * The macro composition boundary remains owned by:
 *
 *     grammar/macros/macros.g4
 *
 * The canonical complete-language composition remains owned by:
 *
 *     grammar/Zamani.g4
 *
 * and:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * COMPILER BASELINE
 * -----------------
 * Rust 2021
 * Rust 1.97 or later
 *
 * SAFETY
 * ------
 * Safe Rust only.
 *
 * This grammar contains:
 *
 *   - no embedded Rust actions;
 *   - no semantic predicates;
 *   - no target-language execution;
 *   - no filesystem access;
 *   - no network access;
 *   - no process execution;
 *   - no hardware discovery;
 *   - no runtime execution;
 *   - no unsafe requirement.
 *
 * PORTABILITY
 * -----------
 * Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)
 *
 * This file contains no target-specific representation.
 *
 * SCALABILITY
 * -----------
 * The language grammar defines no finite maximum for:
 *
 *   - syntax-tree nodes;
 *   - token-tree elements;
 *   - sibling elements;
 *   - delimiter nesting;
 *   - source-unit size;
 *   - generated source size;
 *   - macro declarations;
 *   - macro invocations;
 *   - generated constructs;
 *   - quantum resources;
 *   - classical resources;
 *   - HDL resources;
 *   - accelerator resources;
 *   - distributed resources.
 *
 * Compiler implementations may impose configurable admission/resource
 * safeguards. Such safeguards are implementation policy and MUST NOT be
 * represented as language-level capacity constants.
 *
 * ============================================================================
 * 1. SINGLE-AUTHORITY CONTRACT
 * ============================================================================
 *
 * THIS FILE OWNS
 * -------------
 *
 * Public parser entry:
 *
 *     syntaxTree
 *
 * Public structural adapter:
 *
 *     syntaxTreeNode
 *
 * These rules establish the macro-facing syntax-tree boundary.
 *
 * THIS FILE DOES NOT OWN
 * ---------------------
 *
 * Token-tree structure:
 *
 *     tokenTree
 *     tokenTreeElement
 *     tokenTreeGroup
 *     tokenTreeLeaf
 *     parenthesizedTokenTree
 *     bracketedTokenTree
 *     bracedTokenTree
 *
 * Those rules belong exclusively to:
 *
 *     grammar/macros/token-stream.g4
 *
 * Macro declarations belong to:
 *
 *     grammar/macros/declarations.g4
 *
 * Macro parameters belong to:
 *
 *     grammar/macros/parameters.g4
 *
 * Macro invocation syntax belongs to:
 *
 *     grammar/macros/invocations.g4
 *
 * Expansion syntax belongs to:
 *
 *     grammar/macros/expansion.g4
 *
 * Hygiene syntax belongs to:
 *
 *     grammar/macros/hygiene.g4
 *
 * Macro diagnostics belong to:
 *
 *     grammar/macros/diagnostics.g4
 *
 * Macro subsystem composition belongs to:
 *
 *     grammar/macros/macros.g4
 *
 * Metaprogramming composition belongs to:
 *
 *     grammar/metaprogramming/metaprogramming.g4
 *
 * Quotation syntax belongs to:
 *
 *     grammar/metaprogramming/quotation.g4
 *
 * Unquotation syntax belongs to:
 *
 *     grammar/metaprogramming/unquotation.g4
 *
 * Code generation syntax belongs to the appropriate generation component.
 *
 * Reflection and introspection belong to their respective components.
 *
 * ============================================================================
 * 2. ARCHITECTURAL ROLE
 * ============================================================================
 *
 * The syntax-tree adapter exists because macro processing must be able to
 * preserve source structure without requiring the macro subsystem to know
 * every Zamani language domain.
 *
 * The structural pipeline is:
 *
 *     source
 *       |
 *       v
 *     canonical lexer
 *       |
 *       v
 *     canonical token stream
 *       |
 *       v
 *     token-tree grammar
 *       |
 *       v
 *     syntax-tree adapter
 *       |
 *       v
 *     frontend representation
 *       |
 *       v
 *     macro resolution
 *       |
 *       v
 *     controlled expansion
 *       |
 *       v
 *     hygiene + provenance
 *       |
 *       v
 *     ordinary Zamani validation
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +-------------------------------+
 *       |               |               |
 *       v               v               v
 *     classical     quantum::ir       HDL/domain IR
 *       |               |               |
 *       +---------------+---------------+
 *                       |
 *                       v
 *                 optimization
 *                       |
 *                 lowering/routing
 *                       |
 *                 scheduling
 *                       |
 *                 resilience/QEC
 *                       |
 *                      ZQN
 *                       |
 *                      HAL
 *                       |
 *                 target realization
 *
 * This file participates only at the structural macro boundary.
 *
 * ============================================================================
 * 3. LEXER CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * Canonical lexer vocabulary:
 *
 *     ZamaniLexer
 *
 * The lexer is responsible for:
 *
 *     token identity
 *     token spelling
 *     token text
 *     source positions
 *     lexical diagnostics
 *     canonical delimiter tokens
 *
 * This grammar MUST NOT:
 *
 *     - declare lexer rules;
 *     - declare replacement tokens;
 *     - duplicate keywords;
 *     - duplicate identifiers;
 *     - duplicate literals;
 *     - duplicate operators;
 *     - duplicate delimiters;
 *     - introduce lexer modes;
 *     - introduce lexer channels;
 *     - reinterpret token identity.
 *
 * The syntax-tree adapter consumes the canonical token vocabulary exactly as
 * supplied by the lexer.
 *
 * ============================================================================
 * 4. TOKEN-TREE CONTRACT
 * ============================================================================
 *
 * grammar/macros/token-stream.g4 is the sole owner of:
 *
 *     tokenTree
 *     tokenTreeElement
 *     tokenTreeGroup
 *     tokenTreeLeaf
 *
 * It also owns:
 *
 *     parenthesizedTokenTree
 *     bracketedTokenTree
 *     bracedTokenTree
 *
 * This file MUST reuse those rules.
 *
 * It MUST NOT redefine them.
 *
 * It MUST NOT create:
 *
 *     syntaxTreeLeaf
 *     syntaxTreeGroup
 *     syntaxTreeToken
 *     syntaxTreeDelimiter
 *
 * merely to duplicate token-tree structure.
 *
 * The syntax-tree abstraction is an adapter over the canonical token tree,
 * not a competing token-tree implementation.
 *
 * ============================================================================
 * 5. DELIMITER CONTRACT
 * ============================================================================
 *
 * The canonical token-tree grammar owns balanced delimiters:
 *
 *     LPAREN RPAREN
 *     LBRACK RBRACK
 *     LBRACE RBRACE
 *
 * This file does not interpret delimiter meaning.
 *
 * It therefore accepts whatever balanced token-tree structure is supplied by
 * tokenStream/tokenTree.
 *
 * Valid structural examples include:
 *
 *     ()
 *     []
 *     {}
 *
 *     (a + b)
 *     [a, b, c]
 *     { a = b }
 *
 *     ({ [value] })
 *     ([{ value }])
 *     { call([value]) }
 *
 * Mismatched structures are rejected by the canonical token-tree grammar.
 *
 * Examples that must not be structurally accepted as balanced token trees:
 *
 *     (a]
 *     [a}
 *     {a)
 *     )
 *     ]
 *     }
 *
 * This file does not duplicate delimiter validation.
 *
 * ============================================================================
 * 6. SYNTAX-TREE DEFINITION
 * ============================================================================
 *
 * A complete syntax tree is a sequence of canonical token-tree nodes followed
 * by EOF.
 *
 * Conceptually:
 *
 *     syntaxTree
 *         =
 *             zero or more token-tree nodes
 *             followed by EOF
 *
 * Each syntaxTreeNode is exactly one canonical tokenTree.
 *
 * Therefore:
 *
 *     syntaxTreeNode
 *         =
 *             tokenTree
 *
 * No semantic classification occurs here.
 *
 * A captured structure may contain syntax that will later become:
 *
 *     expression
 *     statement
 *     declaration
 *     type
 *     module
 *     function
 *     data construct
 *     classical construct
 *     quantum construct
 *     hybrid construct
 *     HDL construct
 *     hardware-intent construct
 *     distributed construct
 *     networking construct
 *     metaprogramming construct
 *     or a future domain construct.
 *
 * The syntax-tree parser does not decide which category applies.
 *
 * ============================================================================
 * 7. COMPLETE-INPUT CONTRACT
 * ============================================================================
 *
 * `syntaxTree` is the complete-input entry point.
 *
 * It consumes EOF exactly once.
 *
 * The imported `tokenTree` rule intentionally does NOT consume EOF because it
 * is an inline structural rule.
 *
 * This distinction is mandatory:
 *
 *     syntaxTree
 *         -> complete input
 *
 *     tokenTree
 *         -> inline token-tree structure
 *
 * The adapter therefore MUST NOT invoke `tokenStream` here.
 *
 * Reason:
 *
 *     tokenStream already consumes EOF.
 *
 * Nesting one complete-input rule inside another would create an incorrect
 * composition boundary.
 *
 * The correct dependency is:
 *
 *     tokenStream
 *          |
 *          v
 *     tokenTree
 *          |
 *          v
 *     syntaxTree
 *
 * and not:
 *
 *     syntaxTree
 *          |
 *          v
 *     tokenStream
 *
 * ============================================================================
 * 8. EMPTY INPUT CONTRACT
 * ============================================================================
 *
 * `syntaxTree` permits an empty token-tree sequence:
 *
 *     syntaxTree
 *         : syntaxTreeNode* EOF
 *
 * This does NOT mean that every macro declaration, macro invocation,
 * quotation, generation request, or source unit accepts empty syntax.
 *
 * The owning grammar decides whether an empty syntax tree is legal in a
 * particular context.
 *
 * Examples:
 *
 *     token-stream parsing may allow zero elements;
 *
 *     a macro argument may require one or more elements;
 *
 *     a generated declaration may require a declaration;
 *
 *     a quotation may require a syntactic fragment.
 *
 * Those policies do not belong here.
 *
 * ============================================================================
 * 9. AST CONTRACT
 * ============================================================================
 *
 * ANTLR parser contexts are not the persistent frontend AST.
 *
 * This grammar therefore does not define Rust AST structures.
 *
 * The frontend representation must preserve, as applicable:
 *
 *     - ordered children;
 *     - canonical token identity;
 *     - token text;
 *     - delimiter identity;
 *     - source span;
 *     - source file/module identity;
 *     - macro-definition provenance;
 *     - macro-invocation provenance;
 *     - generated-source provenance;
 *     - expansion ancestry;
 *     - deterministic source order.
 *
 * The frontend AST remains domain-neutral.
 *
 * This file MUST NOT introduce a second AST hierarchy such as:
 *
 *     MacroSyntaxTreeAst
 *     MacroSyntaxNodeAst
 *     MacroSyntaxTokenAst
 *
 * solely because this parser has a `syntaxTreeNode` rule.
 *
 * Parser-rule names do not automatically imply AST-node types.
 *
 * If a dedicated token-tree representation is required by the frontend, it
 * must be owned by the existing macro/frontend AST architecture rather than
 * by this grammar.
 *
 * ============================================================================
 * 10. SOURCE-PROVENANCE CONTRACT
 * ============================================================================
 *
 * Macro-generated syntax must remain diagnosable.
 *
 * The downstream representation should therefore be able to preserve the
 * relationship between:
 *
 *     original source
 *     macro definition
 *     macro invocation
 *     macro arguments
 *     captured token tree
 *     generated syntax
 *     expansion ancestry
 *
 * This grammar does not implement source maps.
 *
 * It does not allocate source IDs.
 *
 * It does not assign generated IDs.
 *
 * It does not generate fresh names.
 *
 * It merely preserves the parser structure needed for the downstream
 * implementation to maintain that information.
 *
 * ============================================================================
 * 11. SEMANTIC BOUNDARY
 * ============================================================================
 *
 * Parsing a syntax tree does NOT:
 *
 *     - resolve names;
 *     - resolve macro paths;
 *     - bind parameters;
 *     - evaluate expressions;
 *     - execute compile-time code;
 *     - infer types;
 *     - check effects;
 *     - check capabilities;
 *     - check resources;
 *     - evaluate policies;
 *     - validate contracts;
 *     - select targets;
 *     - select hardware;
 *     - construct quantum::ir;
 *     - construct HDL IR;
 *     - construct classical IR;
 *     - optimize;
 *     - route;
 *     - schedule;
 *     - perform QEC;
 *     - access runtime state.
 *
 * The correct semantic pipeline remains:
 *
 *     syntax
 *       |
 *       v
 *     structural representation
 *       |
 *       v
 *     macro resolution
 *       |
 *       v
 *     controlled expansion
 *       |
 *       v
 *     hygiene
 *       |
 *       v
 *     provenance
 *       |
 *       v
 *     canonical AST
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     canonical semantic model
 *
 * Generated syntax MUST be subjected to the same applicable language
 * validation as directly authored syntax.
 *
 * ============================================================================
 * 12. HYGIENE CONTRACT
 * ============================================================================
 *
 * Hygiene is not owned here.
 *
 * The syntax-tree adapter MUST NOT:
 *
 *     - resolve lexical scopes;
 *     - assign binding identities;
 *     - create fresh identifiers;
 *     - decide capture behavior;
 *     - bypass visibility;
 *     - bypass ownership;
 *     - bypass type checking;
 *     - bypass effect checking;
 *     - bypass capability checking;
 *     - bypass resource checking;
 *     - bypass security policy.
 *
 * Hygiene-related source metadata is consumed from the canonical macro
 * hygiene subsystem.
 *
 * Canonical owner:
 *
 *     grammar/macros/hygiene.g4
 *
 * Compiler implementation:
 *
 *     macro/frontend hygiene subsystem
 *
 * ============================================================================
 * 13. EXPANSION CONTRACT
 * ============================================================================
 *
 * This grammar does not perform macro expansion.
 *
 * Expansion belongs to the macro compiler/frontend subsystem.
 *
 * Expansion implementation responsibilities may include:
 *
 *     - macro registration;
 *     - macro lookup;
 *     - macro argument binding;
 *     - expansion ordering;
 *     - recursion detection;
 *     - generated-source construction;
 *     - hygiene;
 *     - provenance;
 *     - generated-source validation;
 *     - deterministic expansion;
 *     - configurable compiler admission/resource policies.
 *
 * No expansion limit is encoded here.
 *
 * The grammar MUST NOT contain:
 *
 *     MAX_EXPANSION_DEPTH
 *     MAX_EXPANSION_SIZE
 *     MAX_GENERATED_NODES
 *     MAX_MACRO_ARGUMENTS
 *
 * or equivalent language-level limits.
 *
 * If an implementation requires such safeguards, they belong to explicit
 * compiler configuration and diagnostics.
 *
 * ============================================================================
 * 14. METAPROGRAMMING INTEGRATION
 * ============================================================================
 *
 * The metaprogramming subsystem may consume syntax trees for:
 *
 *     quotation;
 *     unquotation;
 *     source generation;
 *     compile-time computation;
 *     controlled reflection;
 *     syntax transformation;
 *     specialization;
 *     type-level computation;
 *     schema-driven generation;
 *     future metaprogramming facilities.
 *
 * These facilities MUST consume this canonical syntax-tree boundary rather
 * than defining another token-tree/syntax-tree representation.
 *
 * In particular:
 *
 *     grammar/metaprogramming/metaprogramming.g4
 *
 * remains the metaprogramming composition boundary.
 *
 * This file is a dependency of that subsystem where syntax-tree structure is
 * required.
 *
 * This file does not import the complete metaprogramming subsystem.
 *
 * Dependency direction:
 *
 *     token-stream
 *          |
 *          v
 *     syntax-tree
 *          |
 *          +-----------------------+
 *          |                       |
 *          v                       v
 *       macros              metaprogramming
 *
 * This avoids a circular dependency.
 *
 * ============================================================================
 * 15. QUOTATION INTEGRATION
 * ============================================================================
 *
 * Quotation syntax remains owned by:
 *
 *     grammar/metaprogramming/quotation.g4
 *
 * Quotation may produce or consume syntax-tree representations according to
 * its semantic contract.
 *
 * This file does NOT define:
 *
 *     quote
 *     quote-expression
 *     quote-block
 *     quote-token
 *
 * unless those rules are explicitly owned by this component in a future
 * specification.
 *
 * ============================================================================
 * 16. UNQUOTATION INTEGRATION
 * ============================================================================
 *
 * Unquotation remains owned by:
 *
 *     grammar/metaprogramming/unquotation.g4
 *
 * This file does not define interpolation/unquotation syntax.
 *
 * The downstream implementation must ensure that inserted syntax is still
 * represented using the canonical source/token/provenance model.
 *
 * ============================================================================
 * 17. CODE-GENERATION INTEGRATION
 * ============================================================================
 *
 * Code-generation facilities may consume syntax trees to construct source
 * fragments.
 *
 * Generated source MUST re-enter the canonical Zamani frontend.
 *
 * Required conceptual path:
 *
 *     generated token/syntax structure
 *          |
 *          v
 *     canonical source representation
 *          |
 *          v
 *     lexer/parser
 *          |
 *          v
 *     canonical AST
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     semantic analysis
 *
 * Generated syntax must not bypass ordinary language validation.
 *
 * This grammar does not produce source files and does not execute generators.
 *
 * ============================================================================
 * 18. REFLECTION / INTROSPECTION INTEGRATION
 * ============================================================================
 *
 * Reflection and introspection may inspect language structures.
 *
 * They do not gain unrestricted authority merely because syntax trees exist.
 *
 * Reflection/introspection MUST remain subject to:
 *
 *     type rules;
 *     effect rules;
 *     capability rules;
 *     resource rules;
 *     security policy;
 *     provenance;
 *     compilation policy.
 *
 * This file does not define reflection operations.
 *
 * ============================================================================
 * 19. TYPE-SYSTEM INTEGRATION
 * ============================================================================
 *
 * A syntax tree is structurally untyped.
 *
 * It MUST NOT be treated as an ordinary Zamani type merely because it can
 * contain tokens representing a type expression.
 *
 * Type interpretation occurs after the appropriate syntax has re-entered the
 * canonical parser/AST/semantic pipeline.
 *
 * Therefore this grammar does not import or duplicate:
 *
 *     typeExpression
 *     genericParameters
 *     constraints
 *     dependent types
 *     associated types
 *     type classes
 *
 * Such constructs remain owned by the canonical type system.
 *
 * ============================================================================
 * 20. EFFECT INTEGRATION
 * ============================================================================
 *
 * Capturing syntax is not itself permission to perform an effect.
 *
 * This grammar does not declare or grant effects such as:
 *
 *     IO
 *     network
 *     filesystem
 *     native execution
 *     foreign execution
 *     reflection
 *     code generation
 *     adaptation
 *     randomness
 *     quantum measurement
 *
 * If a macro or metaprogramming operation has effects, those effects are
 * declared and checked by the canonical effects subsystem.
 *
 * ============================================================================
 * 21. CAPABILITY INTEGRATION
 * ============================================================================
 *
 * A syntax tree carries no implicit hardware or execution capability.
 *
 * Capturing syntax MUST NOT imply access to:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     accelerator
 *     QPU
 *     filesystem
 *     network
 *     process execution
 *     native execution
 *     secrets
 *     privileged compiler functionality.
 *
 * Capabilities remain semantic/compiler policy.
 *
 * A generated program may later express requirements such as:
 *
 *     requires capability("quantum.measurement");
 *
 * or:
 *
 *     requires capability("tensor.compute");
 *
 * The syntax-tree adapter does not evaluate those requirements.
 *
 * ============================================================================
 * 22. RESOURCE INTEGRATION
 * ============================================================================
 *
 * This grammar introduces no resource limits.
 *
 * In particular, it MUST NOT encode:
 *
 *     maximum syntax nodes;
 *     maximum token count;
 *     maximum nesting depth;
 *     maximum generated source;
 *     maximum macro count;
 *     maximum qubits;
 *     maximum CPUs;
 *     maximum GPUs;
 *     maximum FPGAs;
 *     maximum nodes;
 *     maximum memory;
 *     maximum threads;
 *     maximum tensor rank;
 *     maximum register width;
 *     maximum network size;
 *     maximum device count.
 *
 * Resource admission belongs outside language syntax.
 *
 * The implementation may distinguish:
 *
 *     language validity
 *     parser implementation capacity
 *     configured compilation budget
 *     available system resources
 *
 * Resource exhaustion MUST be diagnosable and MUST NOT silently truncate,
 * reinterpret, or corrupt the syntax tree.
 *
 * ============================================================================
 * 23. POLICY INTEGRATION
 * ============================================================================
 *
 * Policy evaluation is not performed by this grammar.
 *
 * Macro/metaprogramming operations may later be constrained by:
 *
 *     security policy;
 *     capability policy;
 *     resource policy;
 *     compilation policy;
 *     reproducibility policy;
 *     provenance policy.
 *
 * The policy subsystem remains authoritative.
 *
 * A syntax tree does not grant permission to violate a policy.
 *
 * ============================================================================
 * 24. SECURITY CONTRACT
 * ============================================================================
 *
 * Parsing this grammar MUST have no externally observable side effects.
 *
 * It MUST NOT:
 *
 *     - read files;
 *     - write files;
 *     - access the network;
 *     - execute processes;
 *     - inspect environment variables;
 *     - access credentials;
 *     - inspect hardware;
 *     - execute generated code;
 *     - execute macro code;
 *     - mutate compiler-global state.
 *
 * Token capture is representation, not authority.
 *
 * A syntax tree is data.
 *
 * ============================================================================
 * 25. DOMAIN NEUTRALITY
 * ============================================================================
 *
 * The syntax-tree mechanism is intentionally domain-neutral.
 *
 * It can represent source structures belonging to:
 *
 *     classical computing;
 *     numerical computing;
 *     symbolic computing;
 *     tensor computing;
 *     AI/ML;
 *     quantum computing;
 *     hybrid quantum-classical computing;
 *     HDL;
 *     hardware/software co-design;
 *     FPGA/ASIC intent;
 *     accelerators;
 *     distributed computing;
 *     HPC;
 *     networking;
 *     cryptography;
 *     security;
 *     embedded systems;
 *     data processing;
 *     future computational domains.
 *
 * This file therefore MUST NOT create domain-specific syntax-tree rules such
 * as:
 *
 *     quantumSyntaxTree
 *     classicalSyntaxTree
 *     hdlSyntaxTree
 *     gpuSyntaxTree
 *     qpuSyntaxTree
 *     hardwareSyntaxTree
 *
 * The token-tree representation is universal.
 *
 * Domain interpretation happens after ordinary parsing and semantic analysis.
 *
 * ============================================================================
 * 26. QUANTUM BOUNDARY
 * ============================================================================
 *
 * A syntax tree may contain quantum source.
 *
 * This file does not:
 *
 *     - enumerate quantum operations;
 *     - enumerate gates;
 *     - allocate qubits;
 *     - identify physical qubits;
 *     - select a QPU;
 *     - select topology;
 *     - route operations;
 *     - schedule operations;
 *     - perform calibration;
 *     - perform QEC;
 *     - construct ZQN;
 *     - construct HAL;
 *     - construct quantum::ir.
 *
 * The canonical quantum boundary remains:
 *
 *     source
 *       |
 *       v
 *     frontend AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       v
 *     quantum::ir
 *       |
 *       v
 *     optimization
 *       |
 *       v
 *     decomposition/routing
 *       |
 *       v
 *     scheduling
 *       |
 *       v
 *     resilience/QEC
 *       |
 *       v
 *     ZQN
 *       |
 *       v
 *     HAL
 *       |
 *       v
 *     target realization
 *
 * No second quantum representation is introduced here.
 *
 * ============================================================================
 * 27. HDL / HARDWARE BOUNDARY
 * ============================================================================
 *
 * Syntax trees may represent HDL or hardware-intent syntax.
 *
 * This file does not encode:
 *
 *     register widths;
 *     bus widths;
 *     device counts;
 *     memory capacities;
 *     FPGA resources;
 *     ASIC dimensions;
 *     physical pins;
 *     pipeline limits;
 *     clock limits;
 *     topology limits.
 *
 * Those are source semantics, resource requirements, target capabilities,
 * implementation constraints, or backend concerns as appropriate.
 *
 * ============================================================================
 * 28. DETERMINISM CONTRACT
 * ============================================================================
 *
 * For a fixed:
 *
 *     grammar version;
 *     lexer vocabulary;
 *     parser configuration;
 *     canonical token sequence;
 *
 * the structural parse must be deterministic.
 *
 * The syntax-tree adapter introduces no:
 *
 *     randomness;
 *     timestamps;
 *     hardware queries;
 *     network queries;
 *     filesystem dependencies;
 *     unordered semantic lookup;
 *     target-dependent decisions.
 *
 * Child ordering is source ordering.
 *
 * Macro expansion determinism is a downstream compiler responsibility.
 *
 * ============================================================================
 * 29. ERROR AND DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * This grammar relies on the canonical parser/lexer diagnostic system.
 *
 * Structural errors include:
 *
 *     - malformed token input;
 *     - malformed delimiter structure;
 *     - unmatched opening delimiters;
 *     - unmatched closing delimiters;
 *     - crossed delimiters;
 *     - unexpected end of input inside a group.
 *
 * This file MUST NOT silently convert malformed delimiter syntax into ordinary
 * token leaves.
 *
 * Semantic errors such as:
 *
 *     unknown macro;
 *     invalid macro argument;
 *     illegal expansion;
 *     invalid capability;
 *     invalid effect;
 *     invalid resource requirement;
 *     invalid type;
 *
 * are NOT syntax-tree parser errors and remain downstream responsibilities.
 *
 * ============================================================================
 * 30. ERROR-RECOVERY CONTRACT
 * ============================================================================
 *
 * ANTLR's normal error-recovery machinery may construct recovery contexts for
 * diagnostics.
 *
 * Recovery MUST NOT be interpreted by downstream tooling as successful,
 * semantically valid syntax.
 *
 * Production tooling should distinguish:
 *
 *     successfully parsed syntax
 *
 * from:
 *
 *     parser-recovered syntax.
 *
 * The frontend should preserve diagnostics alongside source provenance.
 *
 * ============================================================================
 * 31. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Existing macro invocation syntax remains unchanged.
 *
 * Existing ordinary expression syntax remains owned by:
 *
 *     grammar/expressions/
 *
 * Existing ordinary statement syntax remains owned by:
 *
 *     grammar/statements/
 *
 * Existing declaration syntax remains owned by:
 *
 *     grammar/declarations/
 *
 * Existing token-tree syntax remains owned by:
 *
 *     grammar/macros/token-stream.g4
 *
 * This file does not change the spelling or meaning of existing lexical
 * tokens.
 *
 * Adding future lexer tokens should not require changes to this file unless
 * the canonical delimiter contract itself changes.
 *
 * ============================================================================
 * 32. FORWARD COMPATIBILITY
 * ============================================================================
 *
 * The syntax-tree adapter intentionally does not enumerate domain token types.
 *
 * Therefore a future canonical lexer token may be captured by the canonical
 * token-tree leaf rule without this file being modified, provided the token
 * does not change the established grouping-delimiter contract.
 *
 * This is essential for:
 *
 *     future language features;
 *     future hardware domains;
 *     future quantum operations;
 *     future accelerator models;
 *     future data representations;
 *     future dialects.
 *
 * A new semantic feature should normally be added to its owning grammar,
 * semantic model, AST contract, and tests rather than modifying this adapter.
 *
 * ============================================================================
 * 33. NO SECOND AST / NO SECOND IR
 * ============================================================================
 *
 * This file MUST NOT introduce:
 *
 *     MacroAST;
 *     SyntaxTreeAST;
 *     TokenTreeIR;
 *     MacroIR;
 *     QuantumMacroIR;
 *     HDLMacroIR;
 *     HardwareMacroIR.
 *
 * Parser structure is not an IR.
 *
 * The canonical frontend representation remains the repository AST.
 *
 * Generated source subsequently follows the ordinary semantic/IR pipeline.
 *
 * ============================================================================
 * 34. POCO-REAF CONTRACT
 * ============================================================================
 *
 * The macro syntax must remain independent of target realization.
 *
 * A macro may generate source intended eventually for:
 *
 *     tiny embedded systems;
 *     CPUs;
 *     multicore systems;
 *     GPUs;
 *     FPGAs;
 *     ASICs;
 *     accelerators;
 *     QPUs;
 *     simulators;
 *     clusters;
 *     HPC systems;
 *     distributed systems;
 *     cloud systems;
 *     future hardware.
 *
 * This grammar must not require a macro to know which realization will
 * eventually execute the generated source.
 *
 * Target selection remains downstream.
 *
 * ============================================================================
 * 35. SCALABILITY CONTRACT
 * ============================================================================
 *
 * The grammar uses repetition and recursion rather than finite enumerations.
 *
 * It imposes no language-defined finite limit on:
 *
 *     syntaxTreeNode count;
 *     tokenTreeElement count;
 *     sibling count;
 *     nesting depth;
 *     source size;
 *     macro-generated structure.
 *
 * The phrase "unbounded by the language" means that the grammar does not
 * establish an artificial ceiling.
 *
 * Every physical compiler implementation necessarily operates within the
 * resources actually available to it.
 *
 * Implementations may therefore provide configurable safeguards based on:
 *
 *     available memory;
 *     compilation time budget;
 *     parser resource budget;
 *     configured security policy;
 *     deployment constraints.
 *
 * Such safeguards are not language semantics and must not be encoded as
 * grammar constants.
 *
 * ============================================================================
 * 36. INTEGRATION CONTRACT — UPSTREAM
 * ============================================================================
 *
 * UPSTREAM COMPONENT
 * ------------------
 *
 * grammar/macros/token-stream.g4
 *
 * Required exported rules:
 *
 *     tokenTree
 *
 * This file imports the token-stream grammar and consumes `tokenTree`.
 *
 * No token-tree rule is duplicated here.
 *
 * ============================================================================
 * 37. INTEGRATION CONTRACT — MACRO COMPOSITION
 * ============================================================================
 *
 * grammar/macros/macros.g4
 *
 * This remains the macro composition root.
 *
 * It may import/compose:
 *
 *     syntaxTree
 *
 * through the repository's established ANTLR composition path.
 *
 * This file MUST NOT import `macros.g4`.
 *
 * Dependency direction:
 *
 *     syntaxTree
 *          |
 *          v
 *        Macros
 *
 * not:
 *
 *     Macros
 *          |
 *          v
 *     syntaxTree
 *          |
 *          v
 *        Macros
 *
 * This avoids circular grammar composition.
 *
 * ============================================================================
 * 38. INTEGRATION CONTRACT — MACRO DECLARATIONS
 * ============================================================================
 *
 * grammar/macros/declarations.g4
 *
 * Continues to own macro declaration syntax.
 *
 * It may consume:
 *
 *     tokenTree
 *
 * or:
 *
 *     syntaxTree
 *
 * only where its language contract explicitly requires a syntax-tree
 * representation.
 *
 * It MUST NOT redefine:
 *
 *     syntaxTree
 *     syntaxTreeNode
 *     tokenTree
 *
 * ============================================================================
 * 39. INTEGRATION CONTRACT — MACRO INVOCATIONS
 * ============================================================================
 *
 * grammar/macros/invocations.g4
 *
 * Continues to own:
 *
 *     macroPath
 *     macroInvocation
 *     macroExpression
 *
 * This file does not replace ordinary macro arguments with token trees.
 *
 * Token-tree arguments should be introduced only where the invocation
 * specification explicitly requires them.
 *
 * This preserves compatibility with existing expression-oriented invocation
 * forms.
 *
 * ============================================================================
 * 40. INTEGRATION CONTRACT — EXPANSION
 * ============================================================================
 *
 * grammar/macros/expansion.g4
 *
 * Continues to own explicit expansion-related source syntax.
 *
 * Expansion implementation remains downstream.
 *
 * This grammar provides structural input only.
 *
 * ============================================================================
 * 41. INTEGRATION CONTRACT — HYGIENE
 * ============================================================================
 *
 * grammar/macros/hygiene.g4
 *
 * Continues to own explicit hygiene-related source syntax and metadata.
 *
 * Hygiene implementation must preserve:
 *
 *     lexical scope;
 *     binding identity;
 *     provenance;
 *     source location;
 *     deterministic expansion.
 *
 * ============================================================================
 * 42. INTEGRATION CONTRACT — DIAGNOSTICS
 * ============================================================================
 *
 * grammar/macros/diagnostics.g4
 *
 * Continues to own macro diagnostic metadata.
 *
 * Syntax-tree parser diagnostics must be associated downstream with source
 * provenance rather than reconstructed from normalized text.
 *
 * ============================================================================
 * 43. INTEGRATION CONTRACT — METAPROGRAMMING
 * ============================================================================
 *
 * grammar/metaprogramming/metaprogramming.g4
 *
 * remains the metaprogramming composition boundary.
 *
 * Metaprogramming components may consume the canonical syntax-tree boundary.
 *
 * They must not define another independent syntax-tree grammar.
 *
 * Relevant consumers include:
 *
 *     quotation.g4
 *     unquotation.g4
 *     generation.g4
 *     code-generation.g4
 *     reflection.g4
 *     introspection.g4
 *     specialization.g4
 *     type-level.g4
 *     compile-time.g4
 *     schemas.g4
 *
 * Each consumer retains its own semantic responsibility.
 *
 * ============================================================================
 * 44. INTEGRATION CONTRACT — CANONICAL PARSER
 * ============================================================================
 *
 * grammar/antlr/ZamaniParser.g4
 *
 * owns complete parser composition.
 *
 * This component MUST reach the canonical parser only through the repository's
 * established macro composition hierarchy.
 *
 * `Zamani.g4` MUST NOT directly duplicate or independently compose this
 * component when `ZamaniParser.g4` already owns macro composition.
 *
 * ============================================================================
 * 45. INTEGRATION CONTRACT — CANONICAL ROOT
 * ============================================================================
 *
 * grammar/Zamani.g4
 *
 * remains the complete ANTLR grammar root.
 *
 * This file must never become a direct second root.
 *
 * Correct hierarchy:
 *
 *     Zamani.g4
 *         |
 *         v
 *     ZamaniParser
 *         |
 *         v
 *     macro composition
 *         |
 *         v
 *     syntaxTree
 *         |
 *         v
 *     tokenStream
 *
 * ============================================================================
 * 46. INTEGRATION CONTRACT — FRONTEND AST
 * ============================================================================
 *
 * src/frontend/ast/
 *
 * The frontend owns persistent syntax representation.
 *
 * The AST lowering layer must map:
 *
 *     syntaxTree
 *         |
 *         +--> ordered token-tree structure
 *         +--> source/provenance information
 *
 * without assuming that every parser rule corresponds to an AST node.
 *
 * The AST must remain domain-neutral.
 *
 * No hardware target information belongs in this mapping.
 *
 * ============================================================================
 * 47. INTEGRATION CONTRACT — MACRO ENGINE
 * ============================================================================
 *
 * src/compiler/macro_engine.rs
 *
 * Macro implementation remains responsible for:
 *
 *     registration;
 *     resolution;
 *     binding;
 *     expansion;
 *     hygiene;
 *     provenance;
 *     recursion/resource policy;
 *     generated-source validation.
 *
 * This grammar does not turn the macro engine into a token-tree executor.
 *
 * Any future token-tree expansion implementation must preserve:
 *
 *     deterministic behavior;
 *     source provenance;
 *     hygiene;
 *     semantic validation;
 *     capability checks;
 *     effect checks;
 *     resource checks;
 *     security policy.
 *
 * ============================================================================
 * 48. INTEGRATION CONTRACT — SEMANTIC MODEL
 * ============================================================================
 *
 * After macro expansion, generated source must enter the normal semantic
 * pipeline.
 *
 * The syntax-tree grammar does not create semantic operations.
 *
 * The semantic pipeline remains:
 *
 *     AST
 *       |
 *       v
 *     structural validation
 *       |
 *       v
 *     name resolution
 *       |
 *       v
 *     type analysis
 *       |
 *       v
 *     effect analysis
 *       |
 *       v
 *     capability analysis
 *       |
 *       v
 *     resource analysis
 *       |
 *       v
 *     contract validation
 *       |
 *       v
 *     policy validation
 *       |
 *       v
 *     provenance
 *       |
 *       v
 *     canonical semantic model
 *
 * ============================================================================
 * 49. INTEGRATION CONTRACT — IR
 * ============================================================================
 *
 * This file produces NO IR.
 *
 * Generated source eventually follows the repository's canonical IR pipeline.
 *
 * Conceptually:
 *
 *     generated source
 *          |
 *          v
 *     canonical AST
 *          |
 *          v
 *     semantic model
 *          |
 *          +---------------------+
 *          |                     |
 *          v                     v
 *     classical IR          quantum::ir
 *          |                     |
 *          +----------+----------+
 *                     |
 *                     v
 *                 lowering
 *                     |
 *              optimization
 *                     |
 *              routing/scheduling
 *                     |
 *              resilience/QEC
 *                     |
 *                    ZQN
 *                     |
 *                    HAL
 *
 * ============================================================================
 * 50. HARD-CODING AUDIT
 * ============================================================================
 *
 * This file MUST contain:
 *
 *     no machine-size constants;
 *     no quantum-resource constants;
 *     no hardware-size constants;
 *     no token-count constants;
 *     no syntax-tree-size constants;
 *     no nesting constants;
 *     no macro-expansion constants;
 *     no domain enumeration required for structural capture.
 *
 * Forbidden architectural concepts include:
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
 * This grammar contains none of those limits.
 *
 * ============================================================================
 * 51. TEST CONTRACT
 * ============================================================================
 *
 * Positive tests MUST cover:
 *
 *     empty syntax tree;
 *     one token-tree node;
 *     multiple adjacent nodes;
 *     ordinary identifiers;
 *     operators;
 *     literals;
 *     keywords;
 *     empty groups;
 *     nested parentheses;
 *     nested brackets;
 *     nested braces;
 *     mixed delimiter nesting;
 *     classical source fragments;
 *     quantum source fragments;
 *     hybrid source fragments;
 *     HDL source fragments;
 *     hardware-intent source fragments;
 *     metaprogramming fragments;
 *     future/unknown canonical tokens.
 *
 * Negative tests MUST cover:
 *
 *     unmatched opening delimiters;
 *     unmatched closing delimiters;
 *     crossed delimiters;
 *     mismatched delimiters;
 *     malformed nested groups;
 *     invalid lexical input;
 *     unexpected EOF inside a group.
 *
 * Boundary tests MUST cover:
 *
 *     zero nodes;
 *     one node;
 *     one group;
 *     adjacent groups;
 *     nested groups;
 *     mixed leaves and groups;
 *     large sibling sequences;
 *     deep nesting within configured test resources.
 *
 * Scalability tests MUST increase workload without asserting a universal
 * language maximum.
 *
 * Determinism tests MUST verify that identical canonical token sequences
 * produce structurally equivalent parser results.
 *
 * ============================================================================
 * 52. CROSS-DOMAIN TEST CONTRACT
 * ============================================================================
 *
 * At minimum, the macro syntax-tree test suite should prove that structural
 * capture does not depend on a domain-specific grammar.
 *
 * Representative categories:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     AI/ML
 *     distributed
 *     networking
 *     data
 *     metaprogramming
 *
 * These tests verify structural neutrality.
 *
 * They do NOT make this grammar responsible for validating the semantics of
 * those domains.
 *
 * ============================================================================
 * 53. RESOURCE-EXHAUSTION TEST CONTRACT
 * ============================================================================
 *
 * Tests may intentionally construct increasingly large or deeply nested
 * structures.
 *
 * They must distinguish:
 *
 *     successful parsing
 *
 * from:
 *
 *     configured implementation-resource rejection.
 *
 * A configured resource rejection is not evidence that the language grammar
 * has a semantic maximum.
 *
 * The implementation must not silently truncate the syntax tree.
 *
 * ============================================================================
 * 54. COMPILER-SAFETY CONTRACT
 * ============================================================================
 *
 * Rust implementations consuming this grammar must compile with:
 *
 *     Rust 1.97 or later
 *
 * and:
 *
 *     Rust 2021
 *
 * The implementation must use safe Rust.
 *
 * No `unsafe` block, `unsafe` function, unsafe trait implementation, or other
 * unsafe Rust mechanism is required by this grammar.
 *
 * The grammar itself contains no Rust code.
 *
 * ============================================================================
 * 55. ANTLR CONTRACT
 * ============================================================================
 *
 * Grammar type:
 *
 *     parser grammar
 *
 * Grammar identity:
 *
 *     syntaxTree
 *
 * Lexer vocabulary:
 *
 *     ZamaniLexer
 *
 * Imported grammar:
 *
 *     tokenStream
 *
 * Public adapter rules:
 *
 *     syntaxTree
 *     syntaxTreeNode
 *
 * Required imported rule:
 *
 *     tokenTree
 *
 * Complete-input behavior:
 *
 *     syntaxTree consumes EOF exactly once.
 *
 * Inline behavior:
 *
 *     syntaxTreeNode consumes one canonical tokenTree and does not consume
 *     EOF.
 *
 * No lexer rules are declared.
 *
 * ============================================================================
 * 56. WHY `tokenStream` IS IMPORTED
 * ============================================================================
 *
 * The import:
 *
 *     import tokenStream;
 *
 * is intentional.
 *
 * `tokenStream.g4` is the canonical owner of token-tree structure.
 *
 * Importing it makes `tokenTree` available without duplicating:
 *
 *     delimiter groups;
 *     token leaves;
 *     balanced nesting.
 *
 * The complete `tokenStream` rule itself is not called here because it owns
 * EOF.
 *
 * This gives the repository a single structural source of truth.
 *
 * ============================================================================
 * 57. WHY THIS FILE DOES NOT DEFINE MORE RULES
 * ============================================================================
 *
 * It may be tempting to add rules such as:
 *
 *     syntaxTreeExpression
 *     syntaxTreeStatement
 *     syntaxTreeDeclaration
 *     syntaxTreeType
 *     syntaxTreeQuantum
 *     syntaxTreeHDL
 *
 * Those rules would incorrectly turn this adapter into a second domain parser.
 *
 * The correct design is:
 *
 *     syntaxTree
 *         |
 *         v
 *     canonical token-tree representation
 *         |
 *         v
 *     appropriate canonical language parser/AST
 *
 * This keeps one language grammar and one semantic pipeline.
 *
 * ============================================================================
 * 58. FUTURE EXTENSION CONTRACT
 * ============================================================================
 *
 * A future feature may consume syntax trees without modifying this file when:
 *
 *     - it can consume the canonical `syntaxTree` boundary;
 *     - it does not require a new structural delimiter model;
 *     - it does not require a second token vocabulary;
 *     - it does not require semantic interpretation at this layer.
 *
 * This allows new domains to be added without repeatedly modifying the
 * universal macro syntax-tree adapter.
 *
 * If a future feature genuinely changes the token-tree model itself, the
 * change must first update the canonical token-stream specification and then
 * this adapter only as required by that contract.
 *
 * ============================================================================
 * 59. DEFINITION OF DONE
 * ============================================================================
 *
 * STRUCTURAL
 * ----------
 *
 * [ ] `syntaxTree` is the only complete-input syntax-tree entry point.
 *
 * [ ] `syntaxTreeNode` adapts exactly one canonical `tokenTree`.
 *
 * [ ] `tokenTree` is not duplicated.
 *
 * [ ] Delimiter rules are not duplicated.
 *
 * [ ] No lexer rule exists in this file.
 *
 * [ ] No token is invented here.
 *
 * SEMANTIC
 * --------
 *
 * [ ] No expression semantics are implemented.
 *
 * [ ] No statement semantics are implemented.
 *
 * [ ] No type semantics are implemented.
 *
 * [ ] No macro expansion is implemented.
 *
 * [ ] No hygiene is implemented.
 *
 * [ ] No reflection is implemented.
 *
 * [ ] No compile-time execution is implemented.
 *
 * [ ] No capability evaluation is implemented.
 *
 * [ ] No resource evaluation is implemented.
 *
 * [ ] No policy evaluation is implemented.
 *
 * [ ] No IR is constructed.
 *
 * PORTABILITY
 * -----------
 *
 * [ ] No target-specific grammar is introduced.
 *
 * [ ] No hardware capacity is encoded.
 *
 * [ ] No quantum capacity is encoded.
 *
 * [ ] No artificial syntax-tree capacity is encoded.
 *
 * [ ] No macro-expansion capacity is encoded as language syntax.
 *
 * SAFETY
 * ------
 *
 * [ ] No embedded Rust exists.
 *
 * [ ] No unsafe Rust requirement exists.
 *
 * [ ] Parser execution has no external side effects.
 *
 * INTEGRATION
 * -----------
 *
 * [ ] `token-stream.g4` is imported successfully.
 *
 * [ ] `macros.g4` composes `syntaxTree` exactly once.
 *
 * [ ] No circular macro grammar import exists.
 *
 * [ ] `ZamaniParser.g4` reaches the macro component through the canonical
 *     composition hierarchy.
 *
 * [ ] `Zamani.g4` remains the single complete-language root.
 *
 * [ ] Frontend AST mapping preserves token order and provenance.
 *
 * [ ] Generated source re-enters ordinary validation.
 *
 * TESTING
 * -------
 *
 * [ ] Positive parser tests pass.
 *
 * [ ] Negative delimiter tests pass.
 *
 * [ ] Empty-tree tests pass.
 *
 * [ ] Nested-tree tests pass.
 *
 * [ ] Cross-domain structural tests pass.
 *
 * [ ] Determinism tests pass.
 *
 * [ ] Scalability tests pass within configured implementation resources.
 *
 * [ ] Resource-exhaustion behavior is explicit and non-silent.
 *
 * [ ] Rust 1.97+ safe-Rust CI passes.
 *
 * ============================================================================
 * 60. FINAL OWNERSHIP SUMMARY
 * ============================================================================
 *
 *     syntax-tree.g4
 *          |
 *          +--> owns syntaxTree
 *          +--> owns syntaxTreeNode
 *          |
 *          +--> adapts tokenTree
 *          |
 *          +--> does NOT own tokenTree
 *          +--> does NOT own lexer
 *          +--> does NOT own AST implementation
 *          +--> does NOT own semantics
 *          +--> does NOT own expansion
 *          +--> does NOT own hygiene
 *          +--> does NOT own IR
 *          +--> does NOT own targets
 *
 * Canonical relationship:
 *
 *     lexer
 *       |
 *       v
 *     tokenStream
 *       |
 *       v
 *     syntaxTree
 *       |
 *       v
 *     macro/metaprogramming processing
 *       |
 *       v
 *     canonical AST
 *       |
 *       v
 *     semantic model
 *       |
 *       +-------------------+
 *       |                   |
 *       v                   v
 *     classical         quantum::ir
 *       |                   |
 *       +---------+---------+
 *                 |
 *                 v
 *       target-independent optimization
 *                 |
 *                 v
 *       lowering / routing / scheduling
 *                 |
 *                 v
 *       resilience / QEC / ZQN / HAL
 *                 |
 *                 v
 *       target realization
 *
 * ============================================================================
 * END OF FILE CONTRACT
 * ============================================================================
 */

parser grammar syntaxTree;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * ============================================================================
 * COMPLETE SYNTAX-TREE ENTRY POINT
 * ============================================================================
 *
 * This is the only complete-input entry point owned by this grammar.
 *
 * Zero or more syntax-tree nodes are permitted.
 *
 * EOF is consumed exactly once here.
 *
 * The imported tokenTree rule does not consume EOF and is therefore safe to
 * use through syntaxTreeNode.
 */
syntaxTree
    : syntaxTreeNode* EOF
    ;

/*
 * ============================================================================
 * SYNTAX-TREE NODE ADAPTER
 * ============================================================================
 *
 * Exactly one canonical tokenTree.
 *
 * No semantic classification occurs here.
 *
 * No alternate token-tree implementation is introduced.
 */
syntaxTreeNode
    : tokenTree
    ;
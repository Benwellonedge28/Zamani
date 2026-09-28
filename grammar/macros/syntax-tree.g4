/*
 * ============================================================================
 * Zamani Universal Computing Language
 * ============================================================================
 *
 * File:
 *     grammar/macros/syntax-tree.g4
 *
 * Grammar:
 *     syntaxTree
 *
 * Status:
 *     CANONICAL MACRO SYNTAX-TREE ADAPTER
 *
 * Purpose:
 *     Provide the macro subsystem's syntax-tree parser entry point by
 *     composing the canonical token-tree grammar. Preserve source structure
 *     without defining a competing AST, expression grammar, or token model.
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *
 * Safety:
 *     Safe Rust only. This grammar contains no Rust actions.
 *
 * Portability:
 *     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)
 *
 * Scalability:
 *     No language-defined maximum tree size, node count, token count, or
 *     nesting depth. Implementations may apply explicit configurable resource
 *     budgets outside the language grammar.
 *
 * ============================================================================
 * 1. OWNERSHIP
 * ============================================================================
 *
 * This file owns only:
 *
 *     syntaxTree
 *     syntaxTreeNode
 *
 * It adapts the canonical token-tree representation for macro syntax-tree
 * consumers.
 *
 * It does NOT own:
 *
 *     lexer tokens or token text;
 *     delimiter definitions;
 *     token-tree grouping;
 *     expressions, statements, types, or declarations;
 *     macro declarations or invocations;
 *     AST node storage or NodeId allocation;
 *     source-span implementation;
 *     macro expansion or execution;
 *     hygiene or name resolution;
 *     semantic analysis;
 *     canonical IR construction;
 *     resource or hardware decisions.
 *
 * The canonical token-tree grammar remains the sole owner of:
 *
 *     tokenTree
 *     tokenTreeElement
 *     tokenTreeGroup
 *     tokenTreeLeaf
 *     parenthesizedTokenTree
 *     bracketedTokenTree
 *     bracedTokenTree
 *
 * Do not redefine those rules here.
 *
 * ============================================================================
 * 2. COMPOSITION CONTRACT
 * ============================================================================
 *
 * This is a parser grammar. It reuses the canonical Zamani lexer vocabulary
 * and imports the existing token-stream parser grammar.
 *
 * The import is intentional: syntax-tree consumers and token-stream consumers
 * must agree on one representation of balanced delimiters and token leaves.
 *
 * The imported tokenStream entry rule is not used as a child because it
 * consumes EOF. Instead, this grammar composes its reusable tokenTree rule
 * and supplies its own complete-input entry point.
 *
 * The grammar file name and grammar name must remain aligned with the
 * repository's ANTLR generation conventions.
 *
 * ============================================================================
 * 3. STRUCTURAL CONTRACT
 * ============================================================================
 *
 * A syntax tree at this boundary is a sequence of canonical token-tree
 * nodes. A node is either:
 *
 *     - a canonical token leaf; or
 *     - a balanced parenthesized, bracketed, or braced token group.
 *
 * This is a syntax-preserving structural representation. It is NOT a
 * semantically typed Zamani AST.
 *
 * The parser does not decide whether a captured sequence is a valid:
 *
 *     expression, statement, type, declaration, quantum operation,
 *     HDL construct, hardware requirement, or other domain construct.
 *
 * The ordinary Zamani parser and semantic pipeline make those decisions
 * after the appropriate macro-processing stage.
 *
 * ============================================================================
 * 4. SOURCE AND TOKEN PRESERVATION
 * ============================================================================
 *
 * The grammar preserves token-tree structure. It does not itself store:
 *
 *     token type;
 *     token text;
 *     token index;
 *     source identity;
 *     byte offsets;
 *     line and column;
 *     comments or whitespace discarded by the lexer.
 *
 * Those are properties of the canonical token stream and frontend source
 * representation. The AST adapter must retain the original token references
 * or equivalent source provenance rather than reconstructing source from
 * normalized text.
 *
 * If exact trivia preservation is required, it must be supplied by the
 * lexer/source layer; this grammar cannot recover discarded trivia.
 *
 * ============================================================================
 * 5. AST CONTRACT
 * ============================================================================
 *
 * Parser output is lowered into the existing canonical frontend AST and
 * macro token-tree representation.
 *
 * This grammar does not require or authorize a parallel hierarchy such as:
 *
 *     MacroSyntaxTreeAst
 *     MacroSyntaxNodeAst
 *     MacroTokenAst
 *
 * merely to mirror parser rules.
 *
 * The downstream representation must preserve:
 *
 *     - ordered children;
 *     - leaf token identity and text;
 *     - delimiter kind;
 *     - source span/provenance;
 *     - original token references or equivalent stable source metadata.
 *
 * NodeId allocation, spans, ownership, and storage remain frontend AST
 * responsibilities. Do not put Rust types or implementation actions in
 * this grammar.
 *
 * ============================================================================
 * 6. SEMANTIC AND EXPANSION BOUNDARIES
 * ============================================================================
 *
 * Parsing this grammar MUST NOT:
 *
 *     - expand or execute a macro;
 *     - evaluate expressions;
 *     - resolve names or imports;
 *     - perform type/effect/capability checking;
 *     - inspect hardware or resources;
 *     - read or write files;
 *     - access a network;
 *     - spawn processes;
 *     - construct quantum::ir or another domain IR.
 *
 * The intended pipeline is:
 *
 *     canonical lexer
 *          |
 *          v
 *     token-stream grammar
 *          |
 *          v
 *     syntax-tree adapter
 *          |
 *          v
 *     canonical frontend AST
 *          |
 *          v
 *     macro resolution / controlled expansion
 *          |
 *          v
 *     hygiene / provenance / validation
 *          |
 *          v
 *     semantic analysis
 *          |
 *          v
 *     canonical semantic and domain IR
 *
 * Quantum programs continue to reach the existing quantum::ir boundary.
 * This file introduces no quantum-specific AST or IR.
 *
 * ============================================================================
 * 7. SCALABILITY AND PORTABILITY
 * ============================================================================
 *
 * Repetition and recursion describe sequences and nested groups. This grammar
 * does not encode finite language limits for:
 *
 *     syntax nodes;
 *     tokens;
 *     children;
 *     nested groups;
 *     macro declarations;
 *     macro invocations;
 *     generated constructs;
 *     quantum resources;
 *     hardware resources;
 *     distributed resources.
 *
 * No MAX_* capacity or target-specific identifier belongs here.
 *
 * Actual parsers necessarily operate with finite available memory, time,
 * and stack/implementation resources. A compiler may apply configurable,
 * observable, diagnosable admission budgets. Those are implementation
 * safeguards, not syntax or language semantics.
 *
 * POCO-REAF is supported by keeping this representation independent of the
 * eventual CPU, GPU, FPGA, ASIC, QPU, simulator, cluster, or future target.
 *
 * ============================================================================
 * 8. DETERMINISM AND SAFETY
 * ============================================================================
 *
 * For a fixed grammar version and canonical token stream, parsing must
 * produce the same structural result.
 *
 * This grammar contains no target-language actions, embedded code,
 * predicates, semantic side effects, or unsafe Rust requirement.
 *
 * Rust 1.97 / 1.97.1 compatibility and the prohibition on unsafe Rust apply
 * to the compiler implementation and generated-code integration, not to
 * ANTLR grammar syntax itself.
 *
 * ============================================================================
 * 9. INTEGRATION CONTRACT — DECIDED IN ADVANCE
 * ============================================================================
 *
 * grammar/macros/token-stream.g4
 *     Sole owner of tokenTree and balanced token groups. This grammar imports
 *     it and reuses tokenTree; do not duplicate its productions.
 *
 * grammar/macros/macros.g4
 *     Macro composition boundary. Integrate this grammar there only through
 *     the repository's established ANTLR composition mechanism. Do not add
 *     a second macro composition root.
 *
 * grammar/macros/declarations.g4
 *     Continues to own macro declaration syntax. It may reference the
 *     canonical syntaxTree entry/node where a declaration explicitly accepts
 *     token-tree syntax; it must not redefine syntaxTree or tokenTree.
 *
 * grammar/macros/invocations.g4
 *     Continues to own invocation syntax. Invocation arguments remain under
 *     that file's established contract unless a separately specified
 *     token-tree argument form is adopted. Do not silently replace ordinary
 *     expression arguments with syntax trees.
 *
 * grammar/macros/expansion.g4
 *     Continues to own explicit expansion-related syntax. It may consume
 *     syntaxTree only where the language specification explicitly requires
 *     a syntax-tree operand. This grammar does not perform expansion.
 *
 * grammar/macros/hygiene.g4
 *     Continues to own any explicit hygiene syntax. Hygiene implementation
 *     and provenance remain downstream responsibilities.
 *
 * grammar/expressions/macros.g4
 *     Continues to own expression-side macro integration. Do not import this
 *     grammar there merely to make all macro expressions token trees.
 *
 * grammar/antlr/ZamaniParser.g4 and grammar/Zamani.g4
 *     The canonical parser hierarchy/root owns complete-language composition.
 *     Do not add direct competing imports to Zamani.g4. Add this component
 *     through the established macro composition chain.
 *
 * src/frontend/ast/
 *     Lower parser contexts into the existing canonical AST. Before adding
 *     storage, define the exact AST contract and preserve NodeId/source-span
 *     conventions. Do not create a duplicate AST solely for this grammar.
 *
 * src/compiler/macro_engine.rs
 *     Existing textual template expansion is not made token-tree-aware by
 *     this grammar. A future token-tree expansion implementation must be a
 *     separately specified compiler change, with hygiene, provenance,
 *     deterministic behavior, and resource-budget tests.
 *
 * ============================================================================
 * 10. VALIDATION CONTRACT
 * ============================================================================
 *
 * Positive cases:
 *
 *     empty syntax tree;
 *     one ordinary token;
 *     multiple adjacent tokens;
 *     empty delimiter groups;
 *     nested (), [], and {};
 *     mixed nested groups;
 *     classical syntax captured without interpretation;
 *     quantum syntax captured without gate enumeration;
 *     HDL syntax captured without fixed width assumptions;
 *     unknown/future lexer token types, provided they are not delimiters.
 *
 * Negative cases:
 *
 *     unmatched opening delimiter;
 *     unmatched closing delimiter;
 *     crossed delimiters;
 *     mismatched delimiter kinds;
 *     trailing invalid lexer input;
 *     malformed token stream rejected by the canonical lexer/parser.
 *
 * Boundary/scalability cases:
 *
 *     zero nodes;
 *     one node;
 *     deeply nested groups within configured parser budget;
 *     large sibling sequences within configured parser budget;
 *     large token text within configured source policy;
 *     deterministic repeated parsing.
 *
 * Tests must not assert a universal maximum node count, token count, or
 * nesting depth.
 *
 * ============================================================================
 * 11. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *     [ ] ANTLR recognizes this as a parser grammar.
 *     [ ] The tokenStream grammar is available through the configured import
 *         and generation path.
 *     [ ] tokenTree is reused rather than redefined.
 *     [ ] syntaxTree consumes the complete input exactly once.
 *     [ ] Empty input is accepted as an empty syntax tree.
 *     [ ] Nested and mismatched delimiter tests pass.
 *     [ ] Canonical token vocabulary is used without local lexer rules.
 *     [ ] AST/source-provenance mapping is documented and tested downstream.
 *     [ ] Macro declaration/invocation ownership is unchanged.
 *     [ ] No macro expansion or semantic action occurs during parsing.
 *     [ ] No hardware/domain-specific limit is introduced.
 *     [ ] Generated parser integration passes under the repository's
 *         supported toolchain.
 *     [ ] Rust implementation and tests use safe Rust only.
 *
 * ============================================================================
 */

parser grammar syntaxTree;

options {
    tokenVocab = ZamaniLexer;
}

/*
 * Import the canonical token-tree rules.
 *
 * tokenStream itself consumes EOF, so this grammar uses the imported
 * tokenTree rule directly and owns its own complete-input entry point.
 */
import tokenStream;

/*
 * Public entry point for parsing a complete macro syntax tree.
 *
 * The empty tree is valid. EOF is consumed here exactly once.
 */
syntaxTree
    : syntaxTreeNode* EOF
    ;

/*
 * A syntax-tree node is one canonical token tree.
 *
 * The imported tokenTree rule preserves leaf tokens and balanced delimiter
 * groups. This adapter intentionally does not classify domain syntax.
 */
syntaxTreeNode
    : tokenTree
    ;
```</WritingBlock>

### Integration notes before marking this file done

| Existing file | Required relationship |
|---|---|
| `grammar/macros/token-stream.g4` | Import and reuse its `tokenTree`; do not duplicate delimiter rules. |
| `grammar/macros/macros.g4` | Add this component only through the existing macro composition chain. |
| `declarations.g4` / `invocations.g4` | Keep their ownership; use syntax trees only where an explicit token-tree form is specified. |
| `expansion.g4` / `hygiene.g4` | Consume the representation only where needed; expansion and hygiene remain downstream. |
| `grammar/antlr/ZamaniParser.g4` | Integrate through the established parser composition path, not as a second root. |
| `src/frontend/ast/` | Lower into the canonical AST with source provenance; do not create a parallel AST hierarchy. |
| `src/compiler/macro_engine.rs` | This grammar does not make the existing string-template engine token-tree-aware. That requires a separate implementation contract and tests. |

**Important build gate:** this file assumes ANTLR can resolve the imported grammar named `tokenStream` from the repository’s configured grammar search path, and that `ZamaniLexer` is the generated vocabulary name. Verify those two names and the generation command in CI before calling it production-ready. I have provided the file content here; I have not committed it to GitHub or run ANTLR generation against the repository.
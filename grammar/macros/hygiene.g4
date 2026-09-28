
/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File: grammar/macros/hygiene.g4
 *
 * Grammar: hygiene
 *
 * Status: Canonical macro-hygiene syntax component
 *
 * Language: Zamani
 * Grammar technology: ANTLR4 parser grammar
 *
 * Compiler baseline:
 *   Rust 1.97
 *   Rust 1.97.1
 *   Rust 2021
 *
 * Safety:
 *   No unsafe Rust.
 *   No embedded Rust actions.
 *   No executable grammar predicates.
 *   No host-language execution.
 *
 * ============================================================================
 * 1. PURPOSE
 * ============================================================================
 *
 * This file defines the parser-level boundary for macro hygiene metadata.
 *
 * Macro hygiene preserves the intended relationship between identifiers,
 * declarations, references, lexical scopes, and macro expansion contexts.
 *
 * This grammar preserves the source structure required by downstream
 * hygiene analysis.
 *
 * It does not implement hygiene.
 *
 * ============================================================================
 * 2. ARCHITECTURAL POSITION
 * ============================================================================
 *
 * Zamani source
 *      |
 *      v
 * Canonical lexer
 *      |
 *      v
 * Canonical parser
 *      |
 *      v
 * Frontend AST
 *      |
 *      v
 * Macro resolution
 *      |
 *      v
 * Controlled expansion
 *      |
 *      v
 * Hygiene and provenance analysis
 *      |
 *      v
 * Name resolution
 *      |
 *      v
 * Type/effect/resource/capability analysis
 *      |
 *      v
 * Canonical semantic model
 *      |
 *      v
 * Canonical IR
 *
 * Quantum constructs continue through quantum::ir.
 *
 * No second frontend-specific quantum IR is introduced.
 *
 * ============================================================================
 * 3. OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - macroHygieneAnnotation;
 *   - macroHygieneAnnotations;
 *   - optionalMacroHygieneAnnotations;
 *   - macroHygienePrefix;
 *   - optionalMacroHygienePrefix;
 *   - macroInvocationHygiene;
 *   - macroDeclarationHygiene;
 *   - macroExpansionHygiene.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - lexer tokens;
 *   - annotation lexical syntax;
 *   - identifiers;
 *   - qualified names;
 *   - annotation arguments;
 *   - macro declarations;
 *   - macro invocations;
 *   - macro expansion algorithms;
 *   - name resolution;
 *   - scope construction;
 *   - symbol identity;
 *   - capture analysis;
 *   - fresh identifier generation;
 *   - source-map implementation;
 *   - provenance storage;
 *   - AST storage;
 *   - semantic analysis;
 *   - canonical IR;
 *   - quantum::ir;
 *   - classical IR;
 *   - HDL IR;
 *   - hardware realization;
 *   - resource allocation;
 *   - routing;
 *   - scheduling;
 *   - QEC;
 *   - ZQN;
 *   - HAL;
 *   - runtime execution.
 *
 * ============================================================================
 * 4. SINGLE-AUTHORITY CONTRACT
 * ============================================================================
 *
 * The canonical annotation grammar owns:
 *
 *   annotation
 *   annotationList
 *   annotationName
 *   annotationArguments
 *   annotationArgument
 *   annotationValue
 *
 * This component MUST NOT redefine those rules.
 *
 * The macro declaration grammar owns:
 *
 *   macroDeclaration
 *   macroParameterList
 *   macroParameter
 *
 * The macro invocation grammar owns:
 *
 *   macroPath
 *   macroInvocation
 *   macroExpression
 *
 * The macro expansion grammar owns:
 *
 *   expansion metadata syntax
 *
 * The macro composition grammar owns the integration of these components.
 *
 * ============================================================================
 * 5. LEXER CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * All lexical tokens originate from the canonical Zamani lexer.
 *
 * This component introduces no lexer tokens.
 *
 * In particular, it MUST NOT introduce speculative tokens such as:
 *
 *   HYGIENE
 *   CAPTURE
 *   FRESH
 *   CALL_SITE
 *   DEF_SITE
 *   UNHYGIENIC
 *   RAW_IDENTIFIER
 *
 * These concepts are represented through canonical annotation syntax and
 * interpreted by the semantic annotation registry.
 *
 * The grammar does not reserve additional identifiers.
 *
 * ============================================================================
 * 6. HYGIENE MODEL
 * ============================================================================
 *
 * The semantic implementation must distinguish:
 *
 *   - identifiers originating in macro definitions;
 *   - identifiers originating in macro arguments;
 *   - identifiers generated during expansion;
 *   - references to existing bindings;
 *   - declarations introduced by expansion;
 *   - explicitly authorized captures;
 *   - references resolved in definition-site context;
 *   - references resolved in invocation-site context.
 *
 * These are semantic identities and relationships.
 *
 * They are not lexer tokens or hardware resources.
 *
 * ============================================================================
 * 7. CANONICAL ANNOTATION INTEGRATION
 * ============================================================================
 *
 * A hygiene annotation is structurally an ordinary canonical annotation.
 *
 * The parser preserves the annotation.
 *
 * The semantic annotation registry determines whether its qualified name
 * denotes a recognized hygiene facility.
 *
 * Unknown annotations must not acquire hygiene behavior merely because
 * they appear near a macro.
 *
 * Recognized annotations must be validated for:
 *
 *   - permitted attachment point;
 *   - argument shape;
 *   - semantic meaning;
 *   - authorization;
 *   - compatibility;
 *   - expansion-context legality.
 *
 * ============================================================================
 * 8. CAPTURE AND HYGIENE SAFETY
 * ============================================================================
 *
 * Explicit capture is not equivalent to unrestricted name resolution.
 *
 * A capture request must be validated against the macro's declared policy
 * and the semantic context in which expansion occurs.
 *
 * The parser does not grant access to private bindings.
 *
 * The parser does not grant access to unavailable capabilities.
 *
 * The parser does not bypass:
 *
 *   - visibility;
 *   - ownership;
 *   - type checking;
 *   - effect checking;
 *   - resource checking;
 *   - capability checking;
 *   - security validation.
 *
 * An invalid capture request must produce a structured semantic diagnostic.
 *
 * ============================================================================
 * 9. PROVENANCE CONTRACT
 * ============================================================================
 *
 * Every accepted hygiene annotation must preserve:
 *
 *   - source-file identity;
 *   - source span;
 *   - annotation span;
 *   - enclosing macro declaration or invocation;
 *   - expansion-context identity;
 *   - generated-source provenance, where applicable.
 *
 * The AST must retain sufficient information to connect generated syntax
 * to its originating source.
 *
 * Source locations must not be discarded during expansion.
 *
 * Provenance must remain available to:
 *
 *   - diagnostics;
 *   - debugging;
 *   - IDE tooling;
 *   - source maps;
 *   - reproducible builds;
 *   - deterministic compilation;
 *   - security auditing.
 *
 * This grammar does not allocate provenance identifiers.
 *
 * ============================================================================
 * 10. SCALABILITY
 * ============================================================================
 *
 * This grammar defines no artificial machine-dependent capacity.
 *
 * It introduces no fixed maximum for:
 *
 *   - hygiene annotations;
 *   - captures;
 *   - identifiers;
 *   - bindings;
 *   - lexical scopes;
 *   - macro declarations;
 *   - macro invocations;
 *   - generated declarations;
 *   - expansion depth;
 *   - expansion output.
 *
 * Compiler implementations may provide configurable resource budgets.
 *
 * Such budgets are implementation policies, not language semantics.
 *
 * Resource exhaustion must produce a diagnostic rather than silently
 * changing program meaning.
 *
 * The grammar must not contain universal MAX_* hardware or compiler limits.
 *
 * ============================================================================
 * 11. DETERMINISM
 * ============================================================================
 *
 * Parsing is deterministic for a given:
 *
 *   - source input;
 *   - canonical token stream;
 *   - grammar version;
 *   - language configuration.
 *
 * Hygiene analysis must produce reproducible binding relationships for
 * equivalent compilation inputs.
 *
 * Fresh internal identifiers must be generated by the compiler's
 * deterministic identity subsystem.
 *
 * Random identifiers must not alter observable program semantics.
 *
 * ============================================================================
 * 12. SECURITY
 * ============================================================================
 *
 * Hygiene metadata is declarative source information.
 *
 * It must never authorize:
 *
 *   - filesystem access;
 *   - network access;
 *   - process execution;
 *   - arbitrary host-language execution;
 *   - compiler configuration mutation;
 *   - capability escalation;
 *   - security-policy bypass;
 *   - target selection;
 *   - resource allocation.
 *
 * Macro expansion must remain subject to the compiler's controlled
 * expansion and security policies.
 *
 * ============================================================================
 * 13. POCO-REAF
 * ============================================================================
 *
 * Hygiene is target-independent.
 *
 * The same macro semantics must remain valid across:
 *
 *   - embedded systems;
 *   - CPUs;
 *   - multicore processors;
 *   - GPUs;
 *   - FPGAs;
 *   - ASICs;
 *   - quantum processors;
 *   - quantum simulators;
 *   - heterogeneous accelerators;
 *   - clusters;
 *   - HPC systems;
 *   - distributed systems;
 *   - cloud environments;
 *   - future architectures.
 *
 * Hygiene must never select physical hardware.
 *
 * ============================================================================
 * 14. CANONICAL HYGIENE ANNOTATION
 * ============================================================================
 *
 * This rule delegates all annotation syntax to the canonical annotation
 * grammar.
 *
 * No second annotation language is created.
 *
 * No hygiene behavior is inferred by the parser.
 */

parser grammar hygiene;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * 15. SINGLE HYGIENE ANNOTATION
 * ============================================================================
 *
 * Reuses the canonical annotation rule.
 *
 * The semantic registry determines whether the annotation is relevant
 * to macro hygiene.
 *
 * No annotation spelling is reserved by this component.
 */

macroHygieneAnnotation
    : annotation
    ;


/*
 * ============================================================================
 * 16. HYGIENE ANNOTATION SEQUENCE
 * ============================================================================
 *
 * One or more canonical annotations.
 *
 * No grammar-level count limit is imposed.
 *
 * Ordering is preserved by the parser and AST.
 */

macroHygieneAnnotations
    : macroHygieneAnnotation+
    ;


/*
 * ============================================================================
 * 17. OPTIONAL HYGIENE ANNOTATIONS
 * ============================================================================
 *
 * Used only by consumers whose syntax explicitly permits optional
 * hygiene metadata.
 *
 * The empty alternative is represented by the Kleene closure.
 */

optionalMacroHygieneAnnotations
    : macroHygieneAnnotation*
    ;


/*
 * ============================================================================
 * 18. REQUIRED HYGIENE PREFIX
 * ============================================================================
 *
 * Reusable boundary for a construct that requires at least one
 * hygiene annotation.
 *
 * This rule does not establish the attachment position.
 *
 * The consuming grammar owns that position.
 */

macroHygienePrefix
    : macroHygieneAnnotations
    ;


/*
 * ============================================================================
 * 19. OPTIONAL HYGIENE PREFIX
 * ============================================================================
 *
 * Reusable boundary for a construct that permits zero or more
 * hygiene annotations.
 *
 * A consumer must use this rule only where the normative syntax
 * explicitly allows it.
 */

optionalMacroHygienePrefix
    : optionalMacroHygieneAnnotations
    ;


/*
 * ============================================================================
 * 20. INVOCATION-SIDE HYGIENE
 * ============================================================================
 *
 * Integration boundary for invocation-related hygiene metadata.
 *
 * The invocation grammar continues to own macroInvocation.
 *
 * This rule does not redefine invocation syntax.
 *
 * It does not introduce a new invocation spelling.
 */

macroInvocationHygiene
    : macroHygieneAnnotations
    ;


/*
 * ============================================================================
 * 21. DECLARATION-SIDE HYGIENE
 * ============================================================================
 *
 * Integration boundary for declaration-related hygiene metadata.
 *
 * The declaration grammar continues to own macroDeclaration.
 *
 * This rule does not redefine declaration syntax.
 *
 * It does not imply that every macro declaration must carry
 * hygiene annotations.
 */

macroDeclarationHygiene
    : macroHygieneAnnotations
    ;


/*
 * ============================================================================
 * 22. EXPANSION-SIDE HYGIENE
 * ============================================================================
 *
 * Integration boundary for hygiene metadata preserved during expansion.
 *
 * The expansion grammar owns expansion metadata syntax.
 *
 * The expansion engine owns the transformation.
 *
 * This rule only preserves the canonical annotation structure.
 */

macroExpansionHygiene
    : macroHygieneAnnotations
    ;


/*
 * ============================================================================
 * 23. AST CONTRACT
 * ============================================================================
 *
 * The frontend AST must represent the result of these rules as canonical
 * annotation nodes, not as a second hygiene-specific annotation hierarchy.
 *
 * Required information:
 *
 *   annotation identity;
 *   annotation arguments;
 *   source span;
 *   attachment context;
 *   original source provenance.
 *
 * The AST must not assign binding identities during parsing.
 *
 * The AST must not perform name resolution.
 *
 * The AST must not execute macro expansion.
 *
 * ============================================================================
 * 24. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must:
 *
 *   1. Resolve annotation names.
 *   2. Identify registered hygiene annotations.
 *   3. Validate attachment locations.
 *   4. Validate annotation arguments.
 *   5. Establish expansion-context relationships.
 *   6. Preserve definition-site and invocation-site distinctions.
 *   7. Validate explicit captures.
 *   8. Reject unauthorized binding access.
 *   9. Preserve provenance.
 *  10. Produce deterministic diagnostics.
 *
 * An annotation that is not registered for hygiene has no implicit
 * hygiene effect.
 *
 * ============================================================================
 * 25. IR CONTRACT
 * ============================================================================
 *
 * Hygiene is a source-expansion and semantic-analysis concern.
 *
 * The parser grammar does not create a hygiene IR.
 *
 * Fully expanded, validated constructs proceed through the existing
 * canonical semantic and IR pipeline.
 *
 * Quantum constructs continue to use quantum::ir.
 *
 * ============================================================================
 * 26. DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Diagnostics must distinguish:
 *
 *   - malformed canonical annotation syntax;
 *   - unknown hygiene annotation;
 *   - unsupported attachment point;
 *   - invalid annotation arguments;
 *   - unauthorized capture;
 *   - unresolved generated reference;
 *   - invalid definition-site reference;
 *   - invalid invocation-site reference;
 *   - provenance loss;
 *   - expansion-context mismatch.
 *
 * Syntax errors belong to the parser.
 *
 * Annotation and capture validity belong to semantic analysis.
 *
 * Resource exhaustion belongs to compiler resource diagnostics.
 *
 * ============================================================================
 * 27. INTEGRATION CONTRACT
 * ============================================================================
 *
 * Direct dependencies:
 *
 *   grammar/antlr/ZamaniLexer.g4
 *       Canonical token vocabulary.
 *
 *   Canonical annotation grammar
 *       Owns annotation syntax.
 *
 *   grammar/macros/declarations.g4
 *       Owns macro declarations.
 *
 *   grammar/macros/invocations.g4
 *       Owns macro invocations.
 *
 *   grammar/macros/expansion.g4
 *       Owns expansion metadata syntax.
 *
 *   grammar/macros/macros.g4
 *       Owns macro component composition.
 *
 *   grammar/expressions/macros.g4
 *       Owns expression-side macro integration.
 *
 * Downstream consumers:
 *
 *   src/frontend/ast/
 *       Preserves annotation nodes and source spans.
 *
 *   src/compiler/macro_engine.rs
 *       Performs controlled expansion.
 *
 *   src/toolchain/meta_programming.rs
 *       Integrates metaprogramming policy.
 *
 *   Semantic analysis
 *       Resolves hygiene annotations and binding contexts.
 *
 *   Provenance/source-map infrastructure
 *       Preserves generated-source relationships.
 *
 *   Canonical IR
 *       Receives validated expanded constructs.
 *
 * Integration rule:
 *
 *   This file must be imported or composed by the authoritative parser
 *   build in a way that makes its rules reachable.
 *
 *   A separate parser grammar file is not automatically visible to
 *   another parser grammar merely because both use the same token
 *   vocabulary.
 *
 *   The build must explicitly establish grammar imports or generate
 *   the canonical parser from the repository's supported composition
 *   mechanism.
 *
 * No duplicate rule definitions may be introduced during composition.
 *
 * ============================================================================
 * 28. COMPATIBILITY
 * ============================================================================
 *
 * Existing public rule names are retained.
 *
 * The canonical annotation rule remains the source of annotation syntax.
 *
 * No new reserved keywords are introduced.
 *
 * No new lexer tokens are required.
 *
 * Any future change to annotation attachment positions requires:
 *
 *   - specification approval;
 *   - AST compatibility review;
 *   - parser integration review;
 *   - semantic validation review;
 *   - positive and negative tests;
 *   - versioned compatibility treatment.
 *
 * ============================================================================
 * 29. CONFORMANCE TEST CONTRACT
 * ============================================================================
 *
 * Positive tests:
 *
 *   - one canonical hygiene annotation;
 *   - multiple canonical hygiene annotations;
 *   - optional hygiene metadata where permitted;
 *   - declaration-side integration;
 *   - invocation-side integration;
 *   - expansion-side integration;
 *   - qualified annotation names;
 *   - annotations with canonical arguments;
 *   - annotations containing nested canonical values.
 *
 * Negative tests:
 *
 *   - malformed annotation;
 *   - malformed annotation arguments;
 *   - malformed qualified name;
 *   - invalid attachment location;
 *   - duplicate conflicting semantic directives;
 *   - unauthorized capture;
 *   - unknown annotation treated as hygiene;
 *   - malformed macro expansion context.
 *
 * Boundary tests:
 *
 *   - empty permitted annotation sequence;
 *   - single annotation;
 *   - multiple annotations;
 *   - nested annotation values;
 *   - deeply nested source structures;
 *   - generated declarations and references.
 *
 * Scalability tests:
 *
 *   - large annotation sequences;
 *   - large generated syntax trees;
 *   - deeply nested macro invocations;
 *   - large binding environments;
 *   - large expansion provenance graphs.
 *
 * Determinism tests:
 *
 *   - identical source produces identical parse structure;
 *   - identical expansion inputs preserve binding identity;
 *   - deterministic diagnostics;
 *   - reproducible provenance relationships.
 *
 * Portability tests:
 *
 *   - hygiene metadata does not depend on target hardware;
 *   - the same source-level binding semantics hold across target profiles.
 *
 * ============================================================================
 * 30. HARD-CODING AUDIT
 * ============================================================================
 *
 * Prohibited:
 *
 *   MAX_MACRO_HYGIENE_ANNOTATIONS
 *   MAX_CAPTURE_COUNT
 *   MAX_BINDINGS
 *   MAX_SCOPES
 *   MAX_EXPANSION_DEPTH
 *   MAX_GENERATED_NODES
 *   MAX_QUBITS
 *   MAX_CPUS
 *   MAX_GPUS
 *   MAX_FPGAS
 *   MAX_NODES
 *   MAX_MEMORY
 *
 * These must not become universal grammar or language limits.
 *
 * Configurable implementation resource budgets are permitted.
 *
 * ============================================================================
 * 31. DEFINITION OF DONE
 * ============================================================================
 *
 * This file is complete when:
 *
 *   [ ] It parses with the canonical Zamani token vocabulary.
 *   [ ] The canonical annotation rule is available through composition.
 *   [ ] No annotation syntax is duplicated.
 *   [ ] No lexer tokens are invented.
 *   [ ] Existing public hygiene rule names are preserved.
 *   [ ] Declaration-side integration is established.
 *   [ ] Invocation-side integration is established.
 *   [ ] Expansion-side integration is established.
 *   [ ] The authoritative parser reaches these rules.
 *   [ ] AST annotation nodes preserve source spans.
 *   [ ] Provenance requirements are implemented downstream.
 *   [ ] Semantic hygiene analysis is separate from parsing.
 *   [ ] Capture validation cannot bypass security checks.
 *   [ ] No fixed resource limits are introduced.
 *   [ ] No domain-specific syntax is introduced.
 *   [ ] No second IR is introduced.
 *   [ ] Positive tests pass.
 *   [ ] Negative tests pass.
 *   [ ] Boundary tests pass.
 *   [ ] Scalability tests pass.
 *   [ ] Determinism tests pass.
 *   [ ] Compatibility tests pass.
 *   [ ] Rust implementation uses Rust 1.97.1.
 *   [ ] No unsafe Rust is required.
 *
 * ============================================================================
 * FINAL GUARANTEE
 * ============================================================================
 *
 * This grammar preserves the boundary:
 *
 *   annotation syntax != hygiene semantics
 *
 *   macro syntax != macro expansion
 *
 *   source identifier != physical resource identity
 *
 *   source-level capture != unrestricted privilege
 *
 *   language semantics != target hardware
 *
 * Zamani macro hygiene therefore remains compatible with:
 *
 *   Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
 *
 * The same source-level binding meaning must be preserved across
 * different hardware, execution environments, and scales.
 *
 * ============================================================================
 */

/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/core/pragmas.g4
 *
 * Purpose:
 *     Canonical parser-level grammar for Zamani source pragmas.
 *
 * Grammar technology:
 *     ANTLR4 parser grammar
 *
 * Rust implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     This grammar contains no embedded Rust actions, predicates, I/O,
 *     filesystem access, networking, process execution, hardware discovery,
 *     runtime calls, or unsafe code.
 *
 * ============================================================================
 * ARCHITECTURAL ROLE
 * ============================================================================
 *
 * A pragma is source-level directive/metadata syntax.
 *
 * A pragma is DATA.
 *
 * It is NOT an execution permission.
 *
 * This grammar owns only the syntactic structure required to represent a
 * pragma in the Zamani source language.
 *
 * Semantic analysis is responsible for determining whether a pragma is:
 *
 *     - recognized;
 *     - supported;
 *     - applicable;
 *     - valid in its context;
 *     - compatible with other pragmas;
 *     - meaningful to compilation;
 *     - meaningful to a particular dialect;
 *     - meaningful to a particular target;
 *     - merely preserved metadata.
 *
 * ============================================================================
 * THIS FILE OWNS
 * ============================================================================
 *
 *     - pragma declaration syntax;
 *     - pragma names;
 *     - pragma namespaces;
 *     - pragma arguments;
 *     - positional pragma arguments;
 *     - named pragma arguments;
 *     - structured pragma values;
 *     - pragma lists;
 *     - pragma attachment as a source item;
 *     - pragma termination;
 *     - source-level pragma grouping;
 *     - syntax-preserving pragma composition.
 *
 * ============================================================================
 * THIS FILE DOES NOT OWN
 * ============================================================================
 *
 *     - pragma semantics;
 *     - compiler policy;
 *     - optimization;
 *     - scheduling;
 *     - routing;
 *     - QEC;
 *     - ZQN;
 *     - resilience;
 *     - hardware discovery;
 *     - calibration;
 *     - backend selection;
 *     - device selection;
 *     - resource allocation;
 *     - quantum::ir;
 *     - classical IR;
 *     - HDL IR;
 *     - runtime execution;
 *     - filesystem operations;
 *     - network operations;
 *     - process execution;
 *     - environment-variable access;
 *     - credentials;
 *     - target-specific limits.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * A pragma MUST NOT introduce an implicit physical-machine requirement.
 *
 * This grammar therefore contains no:
 *
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_ASICS
 *     MAX_NODES
 *     MAX_DEVICES
 *     MAX_MEMORY
 *     MAX_REGISTERS
 *     MAX_TENSOR_RANK
 *     MAX_PRAGMAS
 *     MAX_ARGUMENTS
 *
 * Repetition is intentionally expressed using ANTLR's unbounded:
 *
 *     *
 *     +
 *
 * constructs.
 *
 * Actual parser resource exhaustion is an implementation/runtime concern,
 * not a language-level semantic restriction.
 *
 * ============================================================================
 * OPEN-WORLD DESIGN
 * ============================================================================
 *
 * Pragma names are intentionally NOT an enum.
 *
 * The following are all structurally valid names:
 *
 *     zamani::optimize
 *     quantum::policy
 *     hardware::placement
 *     compiler::diagnostics
 *     vendor::extension
 *     future::new_feature
 *
 * This grammar does not need to change merely because a new pragma namespace
 * is introduced.
 *
 * ============================================================================
 * SECURITY PRINCIPLE
 * ============================================================================
 *
 * A pragma payload is never executable by virtue of parsing.
 *
 * For example:
 *
 *     pragma tool::execute("...")
 *
 * is syntax/data only.
 *
 * The parser MUST NOT:
 *
 *     - spawn a process;
 *     - execute a command;
 *     - open a file;
 *     - contact a network;
 *     - access credentials;
 *     - access hardware;
 *     - invoke a backend;
 *     - modify the filesystem.
 *
 * Semantic interpretation, if any, occurs downstream under explicit compiler
 * policy and authorization.
 *
 * ============================================================================
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * A pragma may describe quantum-related intent:
 *
 *     pragma quantum::dynamic_control;
 *     pragma quantum::measurement_policy(...);
 *
 * but this grammar does NOT create or modify:
 *
 *     QubitId
 *     PhysicalQubitId
 *     GateKind
 *     quantum::ir
 *     topology
 *     calibration
 *     QEC state
 *     ZQN state
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * ============================================================================
 * HARDWARE INDEPENDENCE
 * ============================================================================
 *
 * This file does not know:
 *
 *     - how many CPUs exist;
 *     - how many GPUs exist;
 *     - how many QPUs exist;
 *     - how many qubits exist;
 *     - how many nodes exist;
 *     - what topology exists;
 *     - what device is installed;
 *     - what backend is available.
 *
 * A pragma can syntactically express a resource requirement or preference,
 * but resource/capability analysis determines whether that requirement can
 * actually be satisfied.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no semantic predicates;
 *     - no actions;
 *     - no random behavior;
 *     - no external calls;
 *     - no stateful lookup;
 *     - no machine inspection.
 *
 * Parsing is therefore deterministic for a deterministic token stream.
 *
 * ============================================================================
 * SOURCE PRESERVATION
 * ============================================================================
 *
 * The parser should preserve enough source structure for the AST layer to
 * retain:
 *
 *     - namespace;
 *     - name;
 *     - argument order;
 *     - named argument keys;
 *     - literal structure;
 *     - nested values;
 *     - source spans;
 *     - optional terminator.
 *
 * Semantic normalization belongs downstream.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * Canonical lexer:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Required canonical lexical token:
 *
 *     PRAGMA : 'pragma' ;
 *
 * Existing canonical tokens consumed here include:
 *
 *     IDENTIFIER
 *     DOUBLE_COLON
 *     LPAREN
 *     RPAREN
 *     LBRACE
 *     RBRACE
 *     LBRACKET
 *     RBRACKET
 *     COMMA
 *     ASSIGN
 *     COLON
 *     SEMICOLON
 *     DOT
 *     STRING_LITERAL
 *     CHARACTER_LITERAL
 *     INTEGER_LITERAL
 *     DECIMAL_LITERAL
 *     BOOLEAN_LITERAL
 *     NULL_LITERAL
 *
 * If the canonical lexer uses a different literal-token spelling, that
 * spelling MUST be mapped in the lexer contract rather than redefining
 * lexer tokens here.
 *
 * ============================================================================
 * PARSER COMPOSITION
 * ============================================================================
 *
 * This file is a parser grammar and therefore consumes the canonical
 * `ZamaniLexer` vocabulary.
 *
 * It must be composed into the authoritative Zamani parser through the
 * repository's parser grammar assembly.
 *
 * It MUST NOT become a second lexer authority.
 *
 * It MUST NOT redefine the canonical identifier or literal lexer.
 *
 * ============================================================================
 */

parser grammar Pragmas;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 1. PRAGMA
 * ============================================================================
 *
 * Canonical source form:
 *
 *     pragma name;
 *
 *     pragma namespace::name;
 *
 *     pragma namespace::name(...);
 *
 *     pragma namespace::name key = value;
 *
 *     pragma namespace::name(
 *         positional,
 *         option = value,
 *     );
 *
 * The semantic meaning of every form is deferred.
 */

pragma
    : PRAGMA
      pragmaName
      pragmaPayload?
      pragmaTerminator?
    ;


/* ============================================================================
 * 2. PRAGMA LIST
 * ============================================================================
 *
 * No fixed number of pragmas is permitted.
 */

pragmaList
    : pragma+
    ;


/* ============================================================================
 * 3. OPTIONAL PRAGMA LIST
 * ========================================================================== */

optionalPragmaList
    : pragma*
    ;


/* ============================================================================
 * 4. PRAGMA NAME
 * ============================================================================
 *
 * A pragma name is deliberately open-ended.
 *
 * It may be:
 *
 *     name
 *     namespace::name
 *     namespace::subnamespace::name
 *
 * No fixed namespace depth is imposed.
 */

pragmaName
    : qualifiedName
    ;


/* ============================================================================
 * 5. PRAGMA PAYLOAD
 * ============================================================================
 *
 * A pragma may have:
 *
 *     - parenthesized arguments;
 *     - a structured block;
 *     - a named/value sequence.
 *
 * The parser does not decide which payload has semantic meaning.
 */

pragmaPayload
    : pragmaArgumentGroup
    | pragmaPropertyGroup
    | pragmaBlock
    ;


/* ============================================================================
 * 6. PARENTHESIZED ARGUMENT GROUP
 * ============================================================================
 *
 * Examples:
 *
 *     pragma compiler::optimize();
 *
 *     pragma compiler::optimize(level = 3);
 *
 *     pragma quantum::policy(
 *         mode = "portable",
 *         strategy = "adaptive",
 *     );
 *
 * Empty argument groups are valid.
 */

pragmaArgumentGroup
    : LPAREN
      pragmaArgumentList?
      RPAREN
    ;


/* ============================================================================
 * 7. PRAGMA ARGUMENT LIST
 * ============================================================================
 *
 * No fixed argument count.
 *
 * A trailing comma is accepted.
 */

pragmaArgumentList
    : pragmaArgument
      (
          COMMA pragmaArgument
      )*
      COMMA?
    ;


/* ============================================================================
 * 8. PRAGMA ARGUMENT
 * ============================================================================
 *
 * Both positional and named arguments are supported.
 *
 * Semantic validation determines:
 *
 *     - whether positional arguments are permitted;
 *     - whether named arguments are permitted;
 *     - whether both may coexist;
 *     - whether duplicate names are legal;
 *     - whether ordering matters.
 */

pragmaArgument
    : pragmaNamedArgument
    | pragmaValue
    ;


/* ============================================================================
 * 9. NAMED PRAGMA ARGUMENT
 * ============================================================================
 *
 * Example:
 *
 *     level = 3
 *     mode = "portable"
 *     capability = quantum::dynamic_control
 */

pragmaNamedArgument
    : pragmaArgumentName
      ASSIGN
      pragmaValue
    ;


/* ============================================================================
 * 10. ARGUMENT NAME
 * ============================================================================
 *
 * Argument keys are local names.
 *
 * Namespace qualification belongs to the pragma name rather than the local
 * argument key.
 */

pragmaArgumentName
    : IDENTIFIER
    ;


/* ============================================================================
 * 11. PROPERTY GROUP
 * ============================================================================
 *
 * This provides an alternative to parenthesized arguments for directive-style
 * pragmas.
 *
 * Example:
 *
 *     pragma compiler::optimize level = 3, mode = "portable";
 *
 * The parser preserves structure; semantic interpretation is downstream.
 */

pragmaPropertyGroup
    : pragmaProperty
      (
          COMMA pragmaProperty
      )*
      COMMA?
    ;


/* ============================================================================
 * 12. PRAGMA PROPERTY
 * ========================================================================== */

pragmaProperty
    : pragmaArgumentName
      ASSIGN
      pragmaValue
    ;


/* ============================================================================
 * 13. STRUCTURED PRAGMA BLOCK
 * ============================================================================
 *
 * Example:
 *
 *     pragma compiler::configuration {
 *         optimization = "portable";
 *         reproducibility = true;
 *     }
 *
 * A block does not imply execution.
 */

pragmaBlock
    : LBRACE
      pragmaPropertyList?
      RBRACE
    ;


/* ============================================================================
 * 14. PRAGMA PROPERTY LIST
 * ============================================================================ */

pragmaPropertyList
    : pragmaProperty
      (
          SEMICOLON pragmaProperty
      )*
      SEMICOLON?
    ;


/* ============================================================================
 * 15. PRAGMA VALUE
 * ============================================================================
 *
 * Values are static source-level structures.
 *
 * Runtime expression execution is deliberately NOT embedded in this grammar.
 *
 * This prevents a pragma from silently becoming a second programming
 * language.
 */

pragmaValue
    : pragmaScalarValue
    | pragmaReference
    | pragmaArray
    | pragmaObject
    | pragmaTuple
    | pragmaTaggedValue
    ;


/* ============================================================================
 * 16. SCALAR VALUE
 * ============================================================================
 *
 * Literal lexical definitions remain owned by ZamaniLexer.
 */

pragmaScalarValue
    : INTEGER_LITERAL
    | DECIMAL_LITERAL
    | STRING_LITERAL
    | CHARACTER_LITERAL
    | BOOLEAN_LITERAL
    | NULL_LITERAL
    ;


/* ============================================================================
 * 17. PRAGMA REFERENCE
 * ============================================================================
 *
 * A reference is source data.
 *
 * Resolution belongs to semantic analysis.
 */

pragmaReference
    : qualifiedName
    ;


/* ============================================================================
 * 18. PRAGMA ARRAY
 * ============================================================================
 *
 * No fixed element count.
 */

pragmaArray
    : LBRACKET
      pragmaValueList?
      RBRACKET
    ;


/* ============================================================================
 * 19. PRAGMA VALUE LIST
 * ============================================================================
 */

pragmaValueList
    : pragmaValue
      (
          COMMA pragmaValue
      )*
      COMMA?
    ;


/* ============================================================================
 * 20. PRAGMA OBJECT
 * ============================================================================
 *
 * Structured key/value values.
 *
 * Duplicate keys are intentionally syntactically permitted.
 *
 * Semantic validation decides whether a particular pragma schema requires:
 *
 *     reject
 *     first-wins
 *     last-wins
 *     merge
 *     accumulate
 */

pragmaObject
    : LBRACE
      pragmaObjectEntryList?
      RBRACE
    ;


/* ============================================================================
 * 21. PRAGMA OBJECT ENTRY LIST
 * ========================================================================== */

pragmaObjectEntryList
    : pragmaObjectEntry
      (
          COMMA pragmaObjectEntry
      )*
      COMMA?
    ;


/* ============================================================================
 * 22. PRAGMA OBJECT ENTRY
 * ============================================================================
 */

pragmaObjectEntry
    : pragmaObjectKey
      ASSIGN
      pragmaValue
    ;


/* ============================================================================
 * 23. PRAGMA OBJECT KEY
 * ============================================================================
 *
 * String keys allow extension schemas without reserving every possible key
 * as a Zamani keyword.
 */

pragmaObjectKey
    : IDENTIFIER
    | STRING_LITERAL
    ;


/* ============================================================================
 * 24. PRAGMA TUPLE
 * ============================================================================
 *
 * A tuple is distinguished structurally from an array.
 *
 * A one-element tuple requires a trailing comma.
 */

pragmaTuple
    : LPAREN
      pragmaTupleElements
      RPAREN
    ;


/* ============================================================================
 * 25. PRAGMA TUPLE ELEMENTS
 * ============================================================================
 */

pragmaTupleElements
    : pragmaValue COMMA
    | pragmaValue
      COMMA
      pragmaValue
      (
          COMMA pragmaValue
      )*
      COMMA?
    ;


/* ============================================================================
 * 26. TAGGED PRAGMA VALUE
 * ============================================================================
 *
 * Examples:
 *
 *     resource(...)
 *     capability(...)
 *     policy(...)
 *     provenance(...)
 *
 * The tag is an ordinary source-level qualified name.
 *
 * No future tag requires a grammar modification.
 */

pragmaTaggedValue
    : qualifiedName
      LPAREN
      pragmaValueList?
      RPAREN
    ;


/* ============================================================================
 * 27. OPTIONAL VALUE
 * ============================================================================
 */

optionalPragmaValue
    : pragmaValue?
    ;


/* ============================================================================
 * 28. PRAGMA TERMINATOR
 * ============================================================================
 *
 * Semicolon is optional at this grammar boundary because source composition
 * may place the pragma inside another directive/declaration structure.
 *
 * The authoritative top-level statement grammar should decide whether
 * omission is legal in a particular context.
 */

pragmaTerminator
    : SEMICOLON
    ;


/* ============================================================================
 * 29. STANDALONE PRAGMA ITEM
 * ============================================================================
 *
 * This is the integration rule intended for source-item/declaration grammars.
 *
 * Example:
 *
 *     sourceItem
 *         : pragma
 *         | declaration
 *         | statement
 *         ;
 */

pragmaItem
    : pragma
    ;


/* ============================================================================
 * 30. PRAGMA PREFIX
 * ============================================================================
 *
 * Reusable form for grammars that permit one or more pragmas before another
 * construct.
 */

pragmaPrefix
    : pragmaList
    ;


/* ============================================================================
 * 31. OPTIONAL PRAGMA PREFIX
 * ============================================================================
 */

optionalPragmaPrefix
    : optionalPragmaList
    ;


/* ============================================================================
 * 32. NAMED PRAGMA SEQUENCE
 * ============================================================================
 *
 * Useful for compiler/front-end grammars that permit a directive sequence
 * without requiring every pragma to be represented as a top-level declaration.
 */

pragmaSequence
    : pragmaList
    ;


/* ============================================================================
 * 33. PRAGMA VALUE SEQUENCE
 * ============================================================================
 *
 * Explicit reusable sequence form for downstream dialect grammars.
 */

pragmaValueSequence
    : pragmaValue
      (
          COMMA pragmaValue
      )*
      COMMA?
    ;


/* ============================================================================
 * 34. PRAGMA PROPERTY SEQUENCE
 * ============================================================================
 */

pragmaPropertySequence
    : pragmaProperty
      (
          COMMA pragmaProperty
      )*
      COMMA?
    ;


/* ============================================================================
 * 35. PRAGMA DECLARATION GROUP
 * ============================================================================
 *
 * Allows an enclosing grammar to consume a syntactically explicit group
 * without interpreting it.
 */

pragmaGroup
    : LBRACE
      pragmaList?
      RBRACE
    ;


/* ============================================================================
 * 36. EMPTY-SAFE PRAGMA GROUP
 * ============================================================================
 */

optionalPragmaGroup
    : pragmaGroup?
    ;


/* ============================================================================
 * 37. SEMANTICALLY OPAQUE PAYLOAD BOUNDARY
 * ============================================================================
 *
 * Consumers that need only to recognize the existence of a pragma payload
 * may use this rule rather than depending on any specific pragma dialect.
 *
 * The actual structured productions remain available above for AST builders.
 */

pragmaPayloadPresence
    : pragmaPayload?
    ;


/* ============================================================================
 * 38. SOURCE-LEVEL PRAGMA DIRECTIVE
 * ============================================================================
 *
 * This is the canonical integration production.
 *
 * It intentionally does not distinguish:
 *
 *     compiler pragma
 *     quantum pragma
 *     hardware pragma
 *     HDL pragma
 *     resource pragma
 *     optimization pragma
 *     scheduling pragma
 *     vendor pragma
 *     future pragma
 *
 * Those are semantic namespaces.
 */

pragmaDirective
    : pragma
    ;


/* ============================================================================
 * 39. PRAGMA NAME COMPONENT
 * ============================================================================
 *
 * This helper exists as a stable extension point while preserving the
 * canonical qualified-name boundary.
 */

pragmaNameComponent
    : IDENTIFIER
    ;


/* ============================================================================
 * 40. PRAGMA QUALIFIED NAME
 * ============================================================================
 *
 * The canonical `qualifiedName` remains authoritative.
 *
 * This alias exists so downstream tooling can identify pragma-name syntax
 * explicitly without duplicating qualification rules.
 */

pragmaQualifiedName
    : qualifiedName
    ;


/*
 * ============================================================================
 * COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete when all of the following are true:
 *
 * [ ] ZamaniLexer exposes exactly one canonical PRAGMA token.
 *
 * [ ] `qualifiedName` resolves to the canonical core name grammar.
 *
 * [ ] Literal tokens resolve to the canonical lexer vocabulary.
 *
 * [ ] The authoritative parser composes this grammar exactly once.
 *
 * [ ] No parser action is introduced here.
 *
 * [ ] No semantic pragma registry is introduced here.
 *
 * [ ] No machine-specific resource limit is introduced here.
 *
 * [ ] No quantum IR dependency is introduced here.
 *
 * [ ] No hardware dependency is introduced here.
 *
 * [ ] Unknown pragma namespaces remain syntactically representable.
 *
 * [ ] Nested values remain syntactically representable.
 *
 * [ ] Empty argument groups are accepted.
 *
 * [ ] Trailing commas are accepted where specified.
 *
 * [ ] Duplicate object/property keys remain syntactically representable.
 *
 * [ ] Semantic duplicate-key policy remains downstream.
 *
 * [ ] Pragmas cannot execute source payloads during parsing.
 *
 * [ ] Pragmas do not select a backend during parsing.
 *
 * [ ] Pragmas do not select a physical device during parsing.
 *
 * [ ] Pragmas do not impose qubit/core/device limits.
 *
 * [ ] The resulting AST can preserve namespace, name, payload and source
 *     structure required by `src/frontend/ast/node/annotations/pragma.rs`.
 *
 * [ ] Positive, negative, boundary, deterministic and round-trip tests exist
 *     in the grammar test suite.
 *
 * ============================================================================
 */
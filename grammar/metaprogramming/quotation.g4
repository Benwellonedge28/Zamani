/*

* ============================================================================
* Zamani Universal Programming Language
* ============================================================================
* 
* File:
* grammar/metaprogramming/quotation.g4
* 
* Grammar:
* Quotation / Unquotation
* 
* Status:
* Production parser-grammar component
* 
* Language:
* Zamani
* 
* ANTLR:
* ANTLR4 parser grammar
* 
* Compiler baseline:
* Rust 2021
* Rust 1.97 / Rust 1.97.1
* 
* Safety:
* Safe Rust only
* No unsafe Rust
* 
* ============================================================================
* PURPOSE
* ============================================================================
* 
* This file owns the SOURCE-LEVEL SYNTAX of Zamani quotation and unquotation.
* 
* Quotation represents canonical Zamani source structure as a compile-time
* metaprogramming value.
* 
* Unquotation re-enters an ordinary Zamani expression/value into a quotation
* context according to semantic phase rules.
* 
* The fundamental model is:
* 
* Zamani source
*      |
*      v
*   quotation
*      |
*      v
* canonical source structure
*      |
*      v
* metaprogramming semantics
*      |
*      v
* validated Zamani AST
*      |
*      v
* ordinary semantic analysis
*      |
*      v
* canonical semantic representation / IR
* 
* Quotation is therefore a SOURCE-STRUCTURE mechanism.
* 
* It is not:
* 
* - a second programming language;
* - a second lexer;
* - a second AST;
* - a second type system;
* - a second IR;
* - a macro implementation;
* - a compiler escape hatch;
* - a runtime evaluator;
* - a hardware interface;
* - a quantum IR;
* - a host-language execution mechanism.
* 
* ============================================================================
* ARCHITECTURAL POSITION
* ============================================================================
* 
* source
*   |
*   v
* canonical lexer
*   |
*   v
* canonical parser
*   |
*   v
* domain-neutral frontend AST
*   |
*   +-------------------------------+
*   |                               |
*   v                               v
* ordinary semantics             metaprogramming semantics
*                                   |
*                                   v
*                              quotation
*                                   |
*                                   v
*                            unquotation
*                                   |
*                                   v
*                          generated/constructed
*                          canonical source
*                                   |
*                                   v
*                           ordinary validation
*                                   |
*                                   v
*                          canonical semantic model
*                                   |
*              +--------------------+--------------------+
*              |                    |                    |
*              v                    v                    v
*         classical            quantum::ir          HDL/hardware
*              |                    |                    |
*              +--------------------+--------------------+
*                                   |
*                                   v
*                           optimization/lowering
*                                   |
*                          routing/scheduling
*                                   |
*                          resilience/QEC/ZQN
*                                   |
*                                   v
*                                  HAL
*                                   |
*                                   v
*                           target realization
* 
* ============================================================================
* AUTHORITY
* ============================================================================
* 
* Normative architectural authority:
* 
* grammar/DESIGN.md
* 
* Metaprogramming architecture:
* 
* grammar/metaprogramming/README.md
* 
* Canonical metaprogramming composition:
* 
* grammar/metaprogramming/metaprogramming.g4
* 
* Canonical language syntax specification:
* 
* grammar/spec/syntax.md
* 
* Canonical lexical vocabulary:
* 
* grammar/lexer/keywords.g4
* 
* Canonical parser composition:
* 
* grammar/antlr/ZamaniParser.g4
* 
* Canonical frontend implementation:
* 
* src/parser.rs
* src/lexer.rs
* src/frontend/ast/
* 
* This file MUST conform to those authorities.
* 
* ============================================================================
* OWNERSHIP
* ============================================================================
* 
* THIS FILE OWNS:
* 
* - quote expression syntax;
* - unquote expression syntax;
* - the quotation integration boundary;
* - the unquotation integration boundary;
* - quotation body entry through the canonical block grammar;
* - quotation-specific syntactic wrappers;
* - quotation/splicing composition contracts.
* 
* THIS FILE DOES NOT OWN:
* 
* - identifiers;
* - qualified names;
* - expressions generally;
* - statements generally;
* - declarations generally;
* - blocks generally;
* - types;
* - patterns;
* - attributes;
* - macro declarations;
* - macro invocation syntax;
* - macro hygiene;
* - macro expansion algorithms;
* - reflection;
* - introspection;
* - source generation;
* - specialization;
* - compile-time evaluation;
* - semantic analysis;
* - AST implementation;
* - IR implementation;
* - quantum::ir;
* - hardware discovery;
* - resource discovery;
* - routing;
* - scheduling;
* - QEC;
* - ZQN;
* - HAL;
* - runtime execution.
* 
* ============================================================================
* CRITICAL DESIGN RULE
* ============================================================================
* 
* Quotation MUST reuse canonical Zamani syntax inside its quoted body.
* 
* It MUST NOT define a miniature language such as:
* 
* quoteToken
* quoteExpression
* quoteStatement
* quoteDeclaration
* quoteIdentifier
* quoteType
* 
* with duplicate implementations of the ordinary language.
* 
* Instead:
* 
* quote
*   {
*      canonical Zamani block
*   }
* 
* is represented through:
* 
* blockExpression
* 
* owned by the canonical block grammar.
* 
* This guarantees that quoted Zamani syntax evolves with the language rather
* than creating a second syntax that must be maintained independently.
* 
* ============================================================================
* QUOTATION SEMANTIC MODEL
* ============================================================================
* 
* A quotation captures SOURCE STRUCTURE.
* 
* Conceptually:
* 
* quote { ... }
* 
* means:
* 
* "represent this canonical Zamani source structure as metaprogramming
*  data according to the semantic quotation contract."
* 
* The grammar does not determine:
* 
* - whether the result is compile-time only;
* - whether it is serializable;
* - whether it is executable;
* - whether it is generated source;
* - whether it is a syntax tree;
* - whether it is a token representation;
* - whether it is retained after compilation.
* 
* Those decisions belong to semantic analysis.
* 
* ============================================================================
* UNQUOTATION SEMANTIC MODEL
* ============================================================================
* 
* An unquotation expression provides a controlled phase transition from
* ordinary Zamani expression semantics into a quotation context.
* 
* Conceptually:
* 
* unquote expression
* 
* supplies a semantic value to the enclosing quotation.
* 
* The grammar recognizes the boundary only.
* 
* Semantic analysis determines:
* 
* - whether unquotation occurs inside a valid quotation phase;
* - what kind of value is being unquoted;
* - whether the value is source structure;
* - whether the value is type-level structure;
* - whether the value is a token/syntax representation;
* - whether the value is compile-time known;
* - whether the operation is authorized;
* - whether the operation is deterministic;
* - whether the resulting source is valid Zamani;
* - whether the resulting source is semantically valid.
* 
* ============================================================================
* QUOTE / UNQUOTE ARE NOT MACROS
* ============================================================================
* 
* Quotation and macros are related but are not identical.
* 
* Macro declaration/invocation syntax remains owned by:
* 
* grammar/macros/
* 
* Macro expansion and hygiene remain downstream concerns.
* 
* Quotation provides source-structure representation.
* 
* A macro implementation may consume quoted syntax, but quotation itself does
* not perform macro expansion.
* 
* This prevents:
* 
* quotation -> hidden macro engine
* 
* and:
* 
* quotation -> direct semantic bypass
* 
* ============================================================================
* NO QUASIQUOTE LANGUAGE
* ============================================================================
* 
* This component intentionally does not introduce a separate "quasiquote"
* keyword.
* 
* The canonical language specification currently establishes:
* 
* quote
* unquote
* 
* as the quotation boundary.
* 
* If a future quasiquotation facility is required, it must be specified as an
* explicit language feature with:
* 
* lexical contract
* syntax contract
* AST contract
* semantic contract
* phase contract
* hygiene contract
* diagnostics contract
* compatibility contract
* tests
* 
* It must not be silently inferred from this grammar.
* 
* ============================================================================
* LEXICAL CONTRACT
* ============================================================================
* 
* This is a parser grammar.
* 
* It defines NO lexer rules.
* 
* The canonical lexical vocabulary must provide:
* 
* QUOTE
*     'quote'
* 
* UNQUOTE
*     'unquote'
* 
* Those tokens belong to:
* 
* grammar/lexer/keywords.g4
* 
* They must be composed by:
* 
* grammar/antlr/ZamaniLexer.g4
* 
* and consumed through:
* 
* tokenVocab = ZamaniLexer
* 
* This file MUST NOT define:
* 
* QUOTE
* UNQUOTE
* 
* as lexer rules.
* 
* ============================================================================
* WHY QUOTE AND UNQUOTE ARE RESERVED
* ============================================================================
* 
* The language specification already defines:
* 
* QuoteExpression ::= "quote" BlockExpression
* 
* UnquoteExpression ::= "unquote" Expression
* 
* Therefore "quote" and "unquote" have language-level syntactic meaning.
* 
* They must not remain ordinary identifiers in the canonical lexical contract.
* 
* Treating them as ordinary identifiers would make the parser unable to
* distinguish:
* 
* quote { ... }
* 
* from:
* 
* quote
* <ordinary identifier use>
* 
* without introducing semantic predicates or duplicated contextual parsing.
* 
* Reserved lexical tokens provide deterministic syntax.
* 
* ============================================================================
* TOKEN OWNERSHIP
* ============================================================================
* 
* Canonical owner:
* 
* grammar/lexer/keywords.g4
* 
* Required entries:
* 
* QUOTE   : 'quote' ;
* UNQUOTE : 'unquote' ;
* 
* No other lexical grammar may define those tokens.
* 
* This component intentionally does not define:
* 
* SPLICE
* QUASIQUOTE
* EXPAND
* EVAL
* 
* merely because those concepts may be useful to future metaprogramming.
* 
* ============================================================================
* CANONICAL DEPENDENCIES
* ============================================================================
* 
* This grammar consumes:
* 
* QUOTE
* UNQUOTE
* blockExpression
* expression
* 
* "blockExpression" remains owned by:
* 
* grammar/core/blocks.g4
* 
* or the repository's canonical block composition layer.
* 
* "expression" remains owned by:
* 
* grammar/expressions/
* 
* This file MUST NOT redefine either.
* 
* ============================================================================
* PUBLIC INTEGRATION CONTRACT
* ============================================================================
* 
* This file exposes the following productions:
* 
* metaQuoteCore
* metaSpliceCore
* quoteExpressionCore
* unquoteExpressionCore
* 
* The primary metaprogramming composition boundary consumes:
* 
* metaQuoteCore
* metaSpliceCore
* 
* through:
* 
* grammar/metaprogramming/metaprogramming.g4
* 
* The canonical expression composition should expose:
* 
* quoteExpression
* unquoteExpression
* 
* only through one authoritative expression-level wrapper.
* 
* There must not be several independently implemented quote-expression rules.
* 
* ============================================================================
* QUOTE EXPRESSION
* ============================================================================
* 
* Normative syntax:
* 
* quote { ... }
* 
* Grammar:
* 
* quoteExpressionCore
*     : QUOTE blockExpression
*     ;
* 
* The block contents are canonical Zamani syntax.
* 
* ============================================================================
* UNQUOTE EXPRESSION
* ============================================================================
* 
* Normative syntax:
* 
* unquote expression
* 
* Grammar:
* 
* unquoteExpressionCore
*     : UNQUOTE expression
*     ;
* 
* This deliberately consumes the canonical expression rule.
* 
* The semantic phase checker must subsequently reject unquote expressions
* appearing outside an active quotation context.
* 
* The grammar does not need to duplicate that contextual rule.
* 
* ============================================================================
* META QUOTE
* ============================================================================
* 
* "metaQuoteCore" is the stable integration contract consumed by:
* 
* grammar/metaprogramming/metaprogramming.g4
* 
* It delegates to the quotation owner.
* 
* ============================================================================
* META SPLICE
* ============================================================================
* 
* The existing metaprogramming composition grammar expects:
* 
* metaSpliceCore
* 
* The language specification currently uses the term "unquote", rather than
* introducing a separate "splice" keyword.
* 
* Therefore "metaSpliceCore" is an integration alias for the canonical
* unquotation construct:
* 
* metaSpliceCore
*     : unquoteExpressionCore
*     ;
* 
* This does NOT introduce a second source syntax.
* 
* It gives the composition grammar the structural category it already expects
* while keeping the language surface defined by "unquote".
* 
* Semantically, whether an unquoted value is a scalar insertion, a structural
* splice, or another permitted quotation operation is determined by the
* quotation semantic model.
* 
* ============================================================================
* IMPORTANT: NO EXPRESSION REIMPLEMENTATION
* ============================================================================
* 
* The rule:
* 
* unquoteExpressionCore
*     : UNQUOTE expression
* 
* intentionally references the canonical "expression".
* 
* This creates a semantic dependency from quotation to the canonical
* expression grammar.
* 
* It does NOT create a new expression grammar.
* 
* The repository's parser composition must therefore ensure that this grammar
* is composed at the same parser level as the canonical expression grammar.
* 
* No circular grammar import is required from this file.
* 
* ============================================================================
* QUOTATION BODY
* ============================================================================
* 
* The quotation body is a canonical:
* 
* blockExpression
* 
* rather than a locally defined:
* 
* quotationBlock
* 
* containing duplicated statement/declaration/expression rules.
* 
* This is deliberate.
* 
* A quoted body must be able to represent the same source structure that
* ordinary Zamani source can represent, subject to semantic quotation rules.
* 
* Therefore future support for:
* 
* classical constructs
* quantum constructs
* hybrid constructs
* HDL constructs
* hardware intent
* distributed constructs
* AI constructs
* data constructs
* networking constructs
* security constructs
* dialect constructs
* 
* does not require changing this file.
* 
* ============================================================================
* QUANTUM INTEGRATION
* ============================================================================
* 
* Quotation may contain quantum source.
* 
* For example, conceptually:
* 
* quote {
*     apply H to q;
* }
* 
* or future extensible quantum operations.
* 
* This file does not enumerate:
* 
* H
* X
* Y
* Z
* CNOT
* RX
* RY
* RZ
* 
* or any other gate set.
* 
* Quantum syntax remains owned by:
* 
* grammar/quantum/
* 
* Generated or reconstructed quantum source must subsequently enter:
* 
* canonical frontend AST
*      |
*      v
* semantic quantum model
*      |
*      v
* quantum::ir
* 
* Quotation MUST NOT directly construct:
* 
* quantum::ir
* 
* or a quotation-specific quantum representation.
* 
* ============================================================================
* HDL / HARDWARE INTEGRATION
* ============================================================================
* 
* Quotation may represent HDL or target-independent hardware intent because
* those constructs are canonical Zamani source.
* 
* It must not introduce syntax for:
* 
* physical FPGA resources
* physical ASIC cells
* CPU identifiers
* GPU identifiers
* QPU identifiers
* physical addresses
* device inventory
* fixed topology
* 
* unless those constructs are independently defined by the canonical language.
* 
* Quoted source remains subject to the same resource/capability and
* portability semantics as directly authored source.
* 
* ============================================================================
* AI / DATA / DISTRIBUTED INTEGRATION
* ============================================================================
* 
* Quotation is domain-neutral.
* 
* It can represent canonical source structures belonging to:
* 
* AI
* tensors
* datasets
* distributed computation
* networking
* security
* classical computation
* quantum computation
* HDL
* hybrid computation
* future domains
* 
* No domain-specific quotation grammar is required.
* 
* ============================================================================
* POCO-REAF
* ============================================================================
* 
* Quotation must preserve target-independent program meaning.
* 
* It MUST NOT introduce language-level requirements for:
* 
* CPU count
* core count
* thread count
* GPU count
* FPGA count
* ASIC count
* QPU count
* qubit count
* memory capacity
* register width
* tensor rank
* node count
* device count
* topology size
* 
* There are no grammar constants such as:
* 
* MAX_QUOTATIONS
* MAX_QUOTE_DEPTH
* MAX_UNQUOTE_DEPTH
* MAX_QUOTED_ITEMS
* MAX_SPLICES
* MAX_GENERATED_NODES
* MAX_QUOTE_SIZE
* 
* Program values may contain arbitrary numeric values.
* 
* Those values are semantic program data, not grammar-level capacity limits.
* 
* ============================================================================
* SCALABILITY
* ============================================================================
* 
* The grammar imposes no finite language-level limit on:
* 
* - number of quotations;
* - quotation nesting;
* - quotation body size;
* - number of unquotations;
* - number of expressions;
* - number of generated structures;
* - number of quoted declarations;
* - number of quoted statements;
* - number of quoted quantum operations;
* - number of quoted HDL elements;
* - number of quoted resources;
* - number of quoted domains.
* 
* Repetition and nesting are structural properties of the canonical language.
* 
* Practical compiler limits may exist because of:
* 
* available memory;
* compilation time;
* parser resource budgets;
* AST storage;
* semantic-analysis budgets;
* metaprogram execution budgets;
* diagnostic budgets.
* 
* Such limits are implementation policy.
* 
* They MUST NOT be represented as language-level maxima.
* 
* Resource exhaustion must produce an explicit diagnostic and MUST NOT
* silently truncate quotation contents.
* 
* ============================================================================
* PHASE SAFETY
* ============================================================================
* 
* Quotation introduces a phase boundary.
* 
* The grammar records the syntax only.
* 
* Semantic analysis must classify:
* 
* source phase
* quotation phase
* unquotation phase
* generated-source phase
* ordinary semantic phase
* 
* An "unquote" outside an active quotation is a semantic error unless another
* explicitly specified language construct establishes a valid unquotation
* context.
* 
* This is intentionally NOT enforced through parser predicates.
* 
* ============================================================================
* NESTED QUOTATION
* ============================================================================
* 
* Nested quotations are syntactically permitted because the canonical
* "blockExpression" can contain ordinary Zamani expressions and statements,
* and the expression grammar may contain quotation expressions.
* 
* Example:
* 
* quote {
*     let inner = quote {
*         ...
*     };
* }
* 
* The grammar imposes no nesting ceiling.
* 
* Semantic analysis is responsible for:
* 
* phase tracking;
* scope;
* capture;
* hygiene;
* provenance;
* quotation-value typing.
* 
* ============================================================================
* HYGIENE
* ============================================================================
* 
* Quotation MUST NOT silently capture or rewrite lexical bindings.
* 
* Hygiene is not a parser operation.
* 
* The semantic/macro system must preserve:
* 
* source span;
* lexical scope;
* binding identity;
* quotation phase;
* unquotation provenance;
* macro expansion provenance where applicable.
* 
* The grammar must not rename identifiers.
* 
* It must not invent hygienic identifier syntax.
* 
* ============================================================================
* SOURCE SPANS AND PROVENANCE
* ============================================================================
* 
* The frontend AST must preserve source spans for:
* 
* quote keyword;
* quotation body;
* unquote keyword;
* unquoted expression;
* enclosing quotation;
* nested quotation boundaries.
* 
* When quoted structures are transformed or generated, provenance must retain:
* 
* original source file;
* original source span;
* quotation origin;
* unquotation origin;
* transformation origin;
* expansion origin where applicable.
* 
* This is required for production diagnostics and reproducible builds.
* 
* ============================================================================
* AST CONTRACT
* ============================================================================
* 
* This grammar produces an ANTLR parse tree only.
* 
* The canonical frontend AST owns the actual representation.
* 
* Conceptual representation:
* 
* QuoteExpression
*     body
*     source_span
* 
* UnquoteExpression
*     expression
*     source_span
* 
* The exact Rust type names MUST be determined by:
* 
* src/frontend/ast/
* 
* This grammar must not create or require a parallel:
* 
* QuoteAst
* QuotationAst
* UnquoteAst
* SyntaxAst
* 
* unless the canonical frontend AST independently establishes those types.
* 
* If quotation values require structured syntax representation, that
* representation must remain part of the canonical metaprogramming semantic
* model rather than becoming a competing frontend AST hierarchy.
* 
* ============================================================================
* SEMANTIC CONTRACT
* ============================================================================
* 
* Parsing establishes only:
* 
* valid quotation syntax
* 
* and:
* 
* valid unquotation expression syntax
* 
* Semantic analysis must determine:
* 
* 1. whether quotation is legal in the current source context;
* 2. whether the quotation body is semantically representable;
* 3. what quotation value type is produced;
* 4. whether unquotation occurs in a valid quotation phase;
* 5. whether the unquoted expression is compile-time available;
* 6. whether the unquoted value is structurally admissible;
* 7. whether scope/capture rules are satisfied;
* 8. whether hygiene rules are satisfied;
* 9. whether generated source remains valid;
* 10. whether generated source remains semantically valid;
* 11. whether effects are permitted;
* 12. whether capabilities are permitted;
* 13. whether resource requirements are satisfied;
* 14. whether the operation is deterministic;
* 15. whether the result is portable;
* 16. whether provenance can be preserved.
* 
* None of those semantic checks should be encoded as hard-coded parser
* alternatives.
* 
* ============================================================================
* GENERATED SOURCE VALIDATION
* ============================================================================
* 
* If quotation/unquotation produces source structures that become executable
* Zamani constructs, they MUST pass through the ordinary pipeline:
* 
* quoted/constructed source
*      |
*      v
* canonical AST
*      |
*      v
* name resolution
*      |
*      v
* type checking
*      |
*      v
* effect checking
*      |
*      v
* capability/resource checking
*      |
*      v
* semantic validation
*      |
*      v
* canonical semantic representation
*      |
*      v
* canonical IR
* 
* Quotation MUST NOT provide a way to skip:
* 
* type checking
* ownership checking
* effect checking
* capability checking
* security checking
* resource checking
* domain validation
* portability checking
* 
* ============================================================================
* IR CONTRACT
* ============================================================================
* 
* This grammar creates NO IR.
* 
* It MUST NOT create:
* 
* ClassicalIR
* QuantumIR
* quantum::ir
* HDLIR
* HardwareIR
* ScheduleIR
* RoutingIR
* QEC representation
* ZQN representation
* runtime bytecode
* 
* If a quoted construct eventually becomes executable source, it follows the
* ordinary semantic-to-IR path.
* 
* For quantum:
* 
* quotation
*     |
*     v
* canonical AST
*     |
*     v
* quantum semantic analysis
*     |
*     v
* quantum::ir
* 
* There is no:
* 
* quotation -> quotation-specific quantum IR
* 
* ============================================================================
* MACRO INTEGRATION
* ============================================================================
* 
* Macro grammar remains owned by:
* 
* grammar/macros/
* 
* Quotation may be consumed by macro machinery.
* 
* Macro machinery may use quotation values.
* 
* But this file MUST NOT redefine:
* 
* macroDeclaration
* macroInvocation
* macro parameter syntax
* macro expansion
* hygiene
* 
* The macro subsystem must preserve quotation provenance when quotation values
* cross a macro expansion boundary.
* 
* ============================================================================
* GENERATION INTEGRATION
* ============================================================================
* 
* Source generation remains owned by:
* 
* grammar/metaprogramming/generation.g4
* 
* Quotation provides source structure.
* 
* Generation may consume source structures produced by quotation.
* 
* The two concepts must remain distinct:
* 
* quotation
*     = represent source structure
* 
* generation
*     = produce/emit canonical source structure
* 
* Generation must not assume that every quotation is generated code.
* 
* ============================================================================
* REFLECTION INTEGRATION
* ============================================================================
* 
* Reflection remains owned by:
* 
* grammar/metaprogramming/reflection.g4
* 
* A quotation may contain reflection syntax because the quotation body is
* canonical Zamani syntax.
* 
* Reflection may semantically produce information that is later used in
* quotation construction.
* 
* Neither facility may duplicate the other's grammar.
* 
* ============================================================================
* SPECIALIZATION INTEGRATION
* ============================================================================
* 
* Specialization remains owned by:
* 
* grammar/metaprogramming/specialization.g4
* 
* A specialization request may consume a quoted structure only if semantic
* rules explicitly permit it.
* 
* Quotation itself does not specialize code.
* 
* ============================================================================
* COMPILE-TIME EXECUTION INTEGRATION
* ============================================================================
* 
* Compile-time execution remains owned by:
* 
* grammar/metaprogramming/compile-time-execution.g4
* 
* Unquotation may consume an expression whose value is produced by an
* authorized compile-time computation.
* 
* However:
* 
* unquote
* 
* does not itself grant compile-time execution privileges.
* 
* Capability/effect/phase checks remain mandatory.
* 
* ============================================================================
* SECURITY CONTRACT
* ============================================================================
* 
* Parsing quotation MUST NOT:
* 
* - execute code;
* - execute the quoted body;
* - execute unquoted expressions;
* - read files;
* - inspect environment variables;
* - inspect credentials;
* - access arbitrary memory;
* - inspect processes;
* - access the network;
* - inspect hardware;
* - contact a QPU;
* - contact a GPU;
* - contact an FPGA;
* - invoke external processes;
* - bypass compiler policy.
* 
* Quoted source is DATA until semantic phase rules explicitly authorize
* transformation/evaluation.
* 
* ============================================================================
* DETERMINISM
* ============================================================================
* 
* Parsing is deterministic.
* 
* Given identical:
* 
* source
* lexical vocabulary
* grammar
* language version
* 
* the parser must produce the same syntactic structure.
* 
* Quotation evaluation/transformation may have semantic dependencies, but
* deterministic builds must explicitly classify those dependencies.
* 
* Quotation must not silently depend on:
* 
* machine identity;
* CPU count;
* GPU count;
* FPGA count;
* QPU count;
* physical qubit mapping;
* filesystem state;
* network state;
* wall-clock time;
* random state;
* deployment topology.
* 
* ============================================================================
* RESOURCE AND CAPABILITY CONTRACT
* ============================================================================
* 
* Quotation syntax does not discover or allocate resources.
* 
* If compile-time quotation processing requires resources, those requirements
* are handled by the compile-time/resource/capability systems.
* 
* There are no quotation-specific machine capacities.
* 
* A compiler may enforce configurable resource budgets for:
* 
* parser memory;
* semantic analysis;
* metaprogram execution;
* generated source size;
* compilation time.
* 
* Those budgets are implementation policy and must not become grammar
* constants.
* 
* Exhaustion must produce a structured diagnostic.
* 
* Silent truncation is prohibited.
* 
* ============================================================================
* DIAGNOSTICS CONTRACT
* ============================================================================
* 
* Syntax errors include:
* 
* quote
* quote {
* quote { ...
* unquote
* 
* unquote ...
* 
* where the ordinary expression is malformed.
* 
* Semantic errors include:
* 
* unquote outside quotation;
* invalid quotation phase;
* invalid unquoted value;
* unavailable compile-time value;
* invalid source structure;
* invalid generated syntax;
* invalid scope/capture;
* hygiene violation;
* capability violation;
* effect violation;
* non-portable compile-time dependency;
* resource-budget exhaustion;
* prohibited host access.
* 
* Diagnostics must preserve:
* 
* source span;
* quotation nesting;
* unquotation origin;
* generated-source provenance.
* 
* The grammar must not emit diagnostics directly.
* 
* ============================================================================
* NEGATIVE SYNTAX CONTRACT
* ============================================================================
* 
* These forms must not be accepted as valid quotation syntax:
* 
* quote
* 
* quote(
* 
* quote()
* 
* quote(
*     ...
* )
* 
* unquote
* 
* unquote(
* 
* unquote()
* 
* unquote(,)
* 
* The following are syntactically valid only if the ordinary expression
* grammar accepts them:
* 
* unquote x
* unquote foo()
* unquote a + b
* 
* Whether they are semantically valid depends on quotation phase.
* 
* ============================================================================
* POSITIVE TEST CONTRACT
* ============================================================================
* 
* Minimum positive cases:
* 
* quote {}
* 
* quote {
*     let x = 1;
* }
* 
* quote {
*     fn generated() {
*         return 1;
*     }
* }
* 
* quote {
*     apply operation to q;
* }
* 
* quote {
*     module example {}
* }
* 
* Nested quotation:
* 
* quote {
*     let nested = quote {
*         let x = 1;
*     };
* }
* 
* Unquotation:
* 
* quote {
*     let x = unquote value;
* }
* 
* quote {
*     unquote generatedExpression;
* }
* 
* Qualified names and domain constructs remain governed by their canonical
* grammars.
* 
* ============================================================================
* BOUNDARY TEST CONTRACT
* ============================================================================
* 
* Boundary tests must include:
* 
* - empty quotation;
* - one-element quotation;
* - deeply nested quotation;
* - deeply nested canonical blocks;
* - long quoted source;
* - many quoted statements;
* - many quoted declarations;
* - many unquotation sites;
* - nested expressions;
* - Unicode identifiers;
* - comments inside quoted source;
* - strings containing quotation words;
* - escaped string quotation marks;
* - quotation adjacent to ordinary expressions;
* - quotation at end-of-file;
* - whitespace variations;
* - semicolon variations where the canonical block grammar permits them.
* 
* Tests must distinguish grammar structure from compiler resource limits.
* 
* ============================================================================
* SCALABILITY TEST CONTRACT
* ============================================================================
* 
* The test suite must progressively exercise:
* 
* quote { ... }
* 
* with increasing:
* 
* quotation count;
* nesting;
* body size;
* expression complexity;
* declaration complexity;
* unquotation count;
* generated structure size.
* 
* There must be no test whose purpose is to establish a universal maximum.
* 
* Scalability means:
* 
* as resources increase,
* representable program structure can increase,
* 
* without changing the language definition.
* 
* ============================================================================
* PORTABILITY TEST CONTRACT
* ============================================================================
* 
* The same quotation source must remain structurally portable across:
* 
* CPU
* multicore CPU
* GPU
* FPGA
* ASIC
* QPU
* quantum simulator
* accelerator
* HPC
* cluster
* distributed environment
* cloud
* future targets
* 
* where the quoted source itself is semantically applicable.
* 
* A target that cannot satisfy a quoted program's requirements must produce
* an explicit downstream resource/capability diagnostic rather than changing
* quotation semantics.
* 
* ============================================================================
* CROSS-DOMAIN TEST CONTRACT
* ============================================================================
* 
* Quoted canonical syntax must be able to represent, subject to each domain's
* own grammar:
* 
* classical source;
* quantum source;
* hybrid source;
* HDL source;
* hardware intent;
* distributed source;
* AI/data source;
* networking source;
* security source;
* resource declarations;
* dialect-controlled source.
* 
* This file must not be changed merely because a new canonical domain is
* added.
* 
* The canonical block grammar provides the extensibility boundary.
* 
* ============================================================================
* COMPATIBILITY CONTRACT
* ============================================================================
* 
* Existing language specification:
* 
* QuoteExpression ::= "quote" BlockExpression
* 
* UnquoteExpression ::= "unquote" Expression
* 
* remains the source-language contract.
* 
* This file does not rename or replace those constructs.
* 
* Required lexical compatibility change:
* 
* `quote`
*     becomes token QUOTE
* 
* `unquote`
*     becomes token UNQUOTE
* 
* in the canonical keyword vocabulary.
* 
* This is a lexical clarification of already-specified language syntax, not
* a replacement of the quotation feature.
* 
* The token names must remain stable after adoption.
* 
* ============================================================================
* REQUIRED REPOSITORY INTEGRATION
* ============================================================================
* 
* 1. grammar/lexer/keywords.g4
* 
* Add exactly:
* 
* QUOTE   : 'quote' ;
* UNQUOTE : 'unquote' ;
* 
* Do not define them anywhere else.
* 
* 
* 2. grammar/metaprogramming/metaprogramming.g4
* 
* Its existing:
* 
* metaQuote
*     : metaQuoteCore
*     ;
* 
* metaSplice
*     : metaSpliceCore
*     ;
* 
* becomes satisfied by this file.
* 
* "metaSpliceCore" intentionally aliases canonical unquotation syntax rather
* than inventing a new "splice" keyword.
* 
* 
* 3. Canonical parser composition
* 
* The grammar containing this file must be composed at the same parser level
* as:
* 
* Metaprogramming
* Expressions
* Core
* 
* so that "blockExpression" and "expression" are canonical shared rules.
* 
* 
* 4. Expression integration
* 
* The canonical expression composition must expose quotation constructs at
* exactly one expression-level integration point:
* 
* expression
*     ...
*     | quoteExpression
*     | unquoteExpression
*     ...
* 
* The actual expression dispatcher must use the repository's established
* precedence architecture.
* 
* 
* 5. Frontend AST
* 
* The parser-to-AST conversion must preserve:
* 
* quote span;
* body span;
* unquote span;
* expression span;
* nesting;
* provenance.
* 
* 
* 6. Semantic analysis
* 
* The semantic layer must implement:
* 
* quotation phase;
* unquotation phase;
* quotation value typing;
* scope/capture rules;
* hygiene;
* provenance;
* capability/effect validation;
* deterministic-build rules.
* 
* 
* 7. Macro subsystem
* 
* Macro expansion must consume quotation structures without creating a second
* quotation syntax.
* 
* 
* 8. Generation subsystem
* 
* Generated source must re-enter the ordinary parser/AST/semantic pipeline or
* equivalent canonical validated source representation.
* 
* 
* 9. Quantum subsystem
* 
* Quoted quantum source must follow:
* 
* AST
*   ->
* semantic quantum model
*   ->
* quantum::ir
* 
* with no quotation-specific quantum IR.
* 
* 
* 10. HDL/hardware subsystem
* 
* Quoted HDL/hardware intent must follow the same canonical semantic path and
* must not select physical resources during parsing.
* 
* ============================================================================
* INTEGRATION ORDER
* ============================================================================
* 
* This file is intentionally independently complete at the syntax-contract
* level.
* 
* Integration order should be:
* 
* 1. lexical token ownership
*      |
*      v
* 2. this quotation grammar
*      |
*      v
* 3. metaprogramming composition
*      |
*      v
* 4. expression composition
*      |
*      v
* 5. frontend AST mapping
*      |
*      v
* 6. semantic phase validation
*      |
*      v
* 7. macro/generation integration
*      |
*      v
* 8. canonical semantic model
*      |
*      v
* 9. canonical IR
*      |
*      v
* 10. conformance tests
* 
* No later domain file should require this grammar to be rewritten merely
* because that domain gains additional canonical source constructs.
* 
* ============================================================================
* HARD-CODING AUDIT
* ============================================================================
* 
* This grammar MUST contain none of:
* 
* MAX_QUBITS
* MAX_CPUS
* MAX_GPUS
* MAX_FPGAS
* MAX_NODES
* MAX_MEMORY
* MAX_THREADS
* MAX_TENSOR_RANK
* MAX_REGISTER_WIDTH
* MAX_NETWORK_SIZE
* MAX_DEVICE_COUNT
* MAX_QUOTE_DEPTH
* MAX_UNQUOTE_DEPTH
* MAX_QUOTED_ITEMS
* MAX_GENERATED_ITEMS
* 
* It also contains no:
* 
* physical device identifiers;
* physical qubit identifiers;
* fixed topology;
* vendor-specific backend syntax;
* fixed register width;
* fixed tensor rank;
* fixed machine size.
* 
* ============================================================================
* PERFORMANCE
* ============================================================================
* 
* The grammar uses structural repetition supplied by the canonical block and
* expression grammars.
* 
* It introduces no intentionally quadratic quotation-specific production.
* 
* The parser implementation must preserve source ordering and source spans
* without recursively copying arbitrarily large quotation bodies solely for
* syntax recognition.
* 
* Any AST/source-structure duplication required by semantic quotation values
* is a semantic implementation decision and must be subject to compiler
* resource accounting.
* 
* ============================================================================
* RUST CONTRACT
* ============================================================================
* 
* This grammar contains no Rust actions.
* 
* It requires no unsafe implementation.
* 
* The consuming compiler must remain compatible with:
* 
* Rust 2021
* Rust 1.97
* Rust 1.97.1
* 
* No "unsafe" Rust is required or permitted for quotation parsing or semantic
* integration.
* 
* ============================================================================
* COMPLETION CRITERIA
* ============================================================================
* 
* This file is complete when:
* 
* [x] Existing filename is retained.
* 
* [x] Quote syntax matches grammar/spec/syntax.md.
* 
* [x] Unquote syntax matches grammar/spec/syntax.md.
* 
* [x] QUOTE and UNQUOTE are lexer-owned rather than parser-defined.
* 
* [x] Quotation uses canonical blockExpression.
* 
* [x] Unquotation uses canonical expression.
* 
* [x] No duplicate expression grammar exists.
* 
* [x] No duplicate block grammar exists.
* 
* [x] No macro grammar is duplicated.
* 
* [x] No reflection grammar is duplicated.
* 
* [x] No generation grammar is duplicated.
* 
* [x] No specialization grammar is duplicated.
* 
* [x] No compile-time evaluator is implemented in grammar.
* 
* [x] No second AST is created by the grammar.
* 
* [x] No IR is created by the grammar.
* 
* [x] quantum::ir remains the canonical quantum boundary.
* 
* [x] Quoted quantum syntax remains domain-neutral at this layer.
* 
* [x] HDL/hardware syntax remains owned by its canonical domain grammars.
* 
* [x] No hardware/resource limits are encoded.
* 
* [x] No fixed gate set is encoded.
* 
* [x] No physical hardware mapping is encoded.
* 
* [x] No host introspection is granted.
* 
* [x] No runtime execution occurs during parsing.
* 
* [x] Phase semantics remain downstream.
* 
* [x] Hygiene remains downstream.
* 
* [x] Provenance requirements are specified.
* 
* [x] Security requirements are specified.
* 
* [x] Determinism requirements are specified.
* 
* [x] Positive tests are specified.
* 
* [x] Negative tests are specified.
* 
* [x] Boundary tests are specified.
* 
* [x] Scalability tests are specified.
* 
* [x] Cross-domain tests are specified.
* 
* [x] Portability requirements are specified.
* 
* [x] Rust 1.97 / 1.97.1 compatibility is specified.
* 
* [x] No unsafe Rust is required.
* 
* ============================================================================
* FINAL INVARIANT
* ============================================================================
* 
* Quotation represents canonical Zamani source structure.
* 
* Unquotation provides an explicitly delimited phase transition into that
* structure.
* 
* Neither facility changes the meaning of the underlying Zamani language.
* 
* Neither facility bypasses semantic validation.
* 
* Neither facility introduces machine-dependent syntax.
* 
* Neither facility introduces a second IR.
* 
* Therefore quotation remains compatible with:
* 
* Program Once
*      ->
* Compile Once
*      ->
* Run Everywhere
*      ->
* Run Anywhere
*      ->
* Run Forever
* 
* while allowing the language's metaprogramming system to grow with future
* classical, quantum, HDL, AI, distributed, networking, hardware and other
* computational domains without requiring a new quotation grammar for each
* domain.
* 
* ============================================================================
  */

/* ============================================================================

* ANTLR PARSER GRAMMAR
* ========================================================================== */

parser grammar Quotation;

options {
tokenVocab = ZamaniLexer;
}

/*

* ============================================================================
* PUBLIC QUOTATION CORE
* ============================================================================
* 
* Canonical language syntax:
* 
* quote { ... }
* 
* The body is the repository's canonical block expression.
  */
  quoteExpressionCore
  : QUOTE blockExpression
  ;

/*

* ============================================================================
* PUBLIC UNQUOTATION CORE
* ============================================================================
* 
* Canonical language syntax:
* 
* unquote expression
* 
* The expression after UNQUOTE is the ordinary Zamani expression grammar.
* 
* Whether this construct is legal at the current phase is semantic.
  */
  unquoteExpressionCore
  : UNQUOTE expression
  ;

/*

* ============================================================================
* METAPROGRAMMING COMPOSITION CONTRACT
* ============================================================================
* 
* grammar/metaprogramming/metaprogramming.g4 currently expects:
* 
* metaQuoteCore
* metaSpliceCore
* 
* These aliases provide those exact integration points without introducing
* another quotation syntax.
  */
  metaQuoteCore
  : quoteExpressionCore
  ;

metaSpliceCore
: unquoteExpressionCore
;

/*

* ============================================================================
* EXPRESSION-LEVEL WRAPPERS
* ============================================================================
* 
* These wrappers are intentionally thin.
* 
* They exist so the canonical expression dispatcher can expose stable names
* without copying quotation syntax.
  */
  quoteExpression
  : quoteExpressionCore
  ;

unquoteExpression
: unquoteExpressionCore
;

/*

* ============================================================================
* CANONICAL METAPROGRAMMING VALUE BOUNDARY
* ============================================================================
* 
* Quotation values remain semantic values.
* 
* This rule is deliberately structural and delegates all ordinary syntax to
* canonical owners.
  */
  quotationExpression
  : quoteExpressionCore
  | unquoteExpressionCore
  ;

/*

* ============================================================================
* END OF FILE
* ============================================================================
  */
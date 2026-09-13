/*
 * ============================================================================
 * Zamani Programming Language — Function Return Grammar
 * ============================================================================
 *
 * File:
 *     grammar/functions/returns.g4
 *
 * Grammar role:
 *     Reusable parser delegate defining the concrete syntax of function
 *     return types.
 *
 * Language:
 *     Zamani
 *
 * Specification authority:
 *     grammar/spec/syntax.md
 *
 * Minimum implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *
 * Safety:
 *     The Zamani compiler implementation MUST use safe Rust only.
 *     Rust `unsafe` is not required by this grammar and must not be used
 *     to implement parsing, AST construction, semantic analysis, or
 *     lowering.
 *
 * ============================================================================
 *
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     Zamani lexer
 *       |
 *       v
 *     root parser
 *       |
 *       +--> functions.g4
 *              |
 *              +--> parameters.g4
 *              |
 *              +--> returns.g4  <--- THIS FILE
 *              |
 *              +--> generics.g4
 *              |
 *              +--> constraints.g4
 *              |
 *              v
 *           frontend AST
 *              |
 *              v
 *           semantic/type/effect/resource analysis
 *              |
 *              v
 *           canonical semantic IR
 *              |
 *              +--> classical IR
 *              +--> quantum::ir
 *              +--> hardware/HDL IR
 *              +--> distributed/accelerator IR
 *              +--> future IR domains
 *              |
 *              v
 *           optimization / scheduling / routing / resilience
 *              |
 *              v
 *           target realization
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * This file owns ONLY the concrete syntax of a function's optional return
 * type marker.
 *
 * Canonical form:
 *
 *     ReturnType ::= "->" TypeExpression ;
 *
 * Examples:
 *
 *     fn compute() -> Int { ... }
 *
 *     fn measure() -> Result { ... }
 *
 *     fn transform<T>(value: T) -> T { ... }
 *
 *     fn quantum_operation() -> QuantumState { ... }
 *
 * A function without a return type remains valid:
 *
 *     fn compute() {
 *         ...
 *     }
 *
 * ============================================================================
 *
 * THIS FILE DOES NOT OWN
 * ============================================================================
 *
 * This grammar does NOT own:
 *
 *     - the `->` token;
 *     - lexical analysis;
 *     - identifiers;
 *     - TypeExpression;
 *     - primitive types;
 *     - generic type syntax;
 *     - function declarations;
 *     - function parameters;
 *     - function bodies;
 *     - return statements;
 *     - type inference;
 *     - type checking;
 *     - ABI rules;
 *     - calling conventions;
 *     - register allocation;
 *     - machine return registers;
 *     - stack layout;
 *     - memory layout;
 *     - CPU architecture;
 *     - GPU architecture;
 *     - QPU architecture;
 *     - hardware return mechanisms;
 *     - quantum measurement semantics;
 *     - quantum state serialization;
 *     - runtime result transport;
 *     - optimization;
 *     - scheduling;
 *     - lowering;
 *     - execution.
 *
 * In particular, this file MUST NOT encode a fixed number of return values,
 * fixed machine widths, fixed registers, fixed ABI conventions, or
 * target-specific result representations.
 *
 * ============================================================================
 *
 * CANONICAL SPECIFICATION
 * ============================================================================
 *
 * The canonical language specification defines:
 *
 *     FunctionDeclaration ::=
 *         ["async"]
 *         "fn"
 *         IDENT
 *         [GenericParameters]
 *         "(" [ParameterList] ")"
 *         ["->" TypeExpression]
 *         [WhereClause]
 *         BlockExpression ;
 *
 * Therefore this delegate owns only:
 *
 *     returnType
 *
 * and leaves its optional presence to the function declaration/signature that
 * consumes it.
 *
 * ============================================================================
 *
 * DESIGN PRINCIPLE
 * ============================================================================
 *
 * A return type is a SOURCE-LEVEL TYPE.
 *
 * It is NOT a physical machine result.
 *
 * For example:
 *
 *     fn compute() -> Result { ... }
 *
 * does not imply:
 *
 *     - a particular CPU register;
 *     - a particular calling convention;
 *     - a particular memory representation;
 *     - a particular device;
 *     - a particular quantum backend;
 *     - a particular number of classical values;
 *     - a particular hardware result channel.
 *
 * Those decisions belong downstream.
 *
 * ============================================================================
 *
 * POCO-REAF
 * ============================================================================
 *
 * Return syntax participates in:
 *
 *     Program Once
 *     Compile Once
 *     Run Everywhere
 *     Anywhere
 *     Forever
 *
 * A return type therefore describes the semantic result contract of a
 * computation, not the physical mechanism used to transport that result.
 *
 * The same source-level return type must be capable of being lowered to:
 *
 *     - embedded systems;
 *     - CPUs;
 *     - multicore CPUs;
 *     - GPUs;
 *     - FPGAs;
 *     - ASICs;
 *     - accelerators;
 *     - quantum systems;
 *     - simulators;
 *     - distributed systems;
 *     - cloud execution;
 *     - future computational substrates.
 *
 * ============================================================================
 *
 * SCALABILITY
 * ============================================================================
 *
 * There is intentionally NO grammar-level finite limit on:
 *
 *     - type-expression complexity;
 *     - generic nesting;
 *     - type composition;
 *     - semantic type size;
 *     - number of function declarations;
 *     - number of functions in a program.
 *
 * Resource limitations are not grammar limitations.
 *
 * Any actual implementation/resource restriction belongs to the appropriate
 * compiler, semantic-analysis, resource-management, or execution layer.
 *
 * ============================================================================
 *
 * IMPORTANT DISTINCTION
 * ============================================================================
 *
 * This grammar permits a syntactically valid type expression.
 *
 * It does NOT guarantee that the type:
 *
 *     - exists;
 *     - is visible;
 *     - satisfies generic constraints;
 *     - is constructible;
 *     - is supported by a target;
 *     - is representable by a selected backend;
 *     - satisfies resource requirements;
 *     - is legal for a particular ABI;
 *     - is legal for a particular execution environment.
 *
 * Those are semantic or target-realization questions.
 *
 * ============================================================================
 *
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum return types are intentionally not special-cased here.
 *
 * If the type system defines a valid quantum type, it may occur as the
 * TypeExpression after `->`.
 *
 * For example, subject to the canonical quantum type system:
 *
 *     fn prepare() -> QuantumState { ... }
 *
 *     fn execute() -> MeasurementResult { ... }
 *
 *     fn transform(q: Qubit) -> Qubit { ... }
 *
 * The grammar does NOT determine whether such values correspond to:
 *
 *     - logical qubits;
 *     - physical qubits;
 *     - classical measurement data;
 *     - simulator objects;
 *     - encoded quantum states;
 *     - provider-specific representations.
 *
 * That distinction belongs to semantic analysis and the canonical IR.
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * ============================================================================
 *
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware-oriented types may be used as return types only when defined by
 * the canonical type system.
 *
 * This file does not introduce:
 *
 *     CPUResult
 *     GPUResult
 *     FPGAResult
 *     ASICResult
 *     QPUResult
 *     DeviceResult
 *
 * or any equivalent provider-specific grammar construct.
 *
 * Hardware realization belongs downstream.
 *
 * ============================================================================
 *
 * DISTRIBUTED / ASYNC INTEGRATION
 * ============================================================================
 *
 * `async` is owned by the function declaration grammar, not this file.
 *
 * An asynchronous function may consume this return-type rule:
 *
 *     async fn compute() -> Result {
 *         ...
 *     }
 *
 * Whether the semantic result is represented as a future, task, promise,
 * distributed value, stream, actor message, or another execution abstraction
 * is determined outside this grammar.
 *
 * ============================================================================
 *
 * FUNCTION-TYPE INTEGRATION
 * ============================================================================
 *
 * This file defines return syntax for DECLARATIONS/SIGNATURES:
 *
 *     fn name(...) -> Type
 *
 * Function-type grammar may independently define syntax for a function type,
 * for example:
 *
 *     (A, B) -> C
 *
 * That is a different syntactic owner.
 *
 * `returns.g4` MUST NOT duplicate or redefine function-type syntax.
 *
 * The shared `THIN_ARROW` lexical token remains owned by the lexer.
 *
 * ============================================================================
 *
 * ERROR HANDLING
 * ============================================================================
 *
 * The parser should reject malformed return syntax deterministically.
 *
 * Examples of malformed syntax include:
 *
 *     fn f() -> { }
 *     fn f() -> ; 
 *     fn f() -> ) { }
 *
 * The grammar must not silently reinterpret malformed return types.
 *
 * Diagnostic wording, source spans, error codes, recovery policy, and
 * user-facing diagnostic rendering belong to the diagnostic/error subsystem.
 *
 * ============================================================================
 *
 * NO TRAILING RETURN SYNTAX
 * ============================================================================
 *
 * The canonical syntax contains exactly one optional return-type clause:
 *
 *     -> TypeExpression
 *
 * This file therefore does NOT add:
 *
 *     -> Type1, Type2
 *     -> (Type1, Type2)
 *     returns Type
 *     : Type
 *
 * unless a future language specification explicitly standardizes such syntax.
 *
 * Future multi-result semantics can be introduced through the type system,
 * for example a tuple/result type, without changing this grammar.
 *
 * ============================================================================
 *
 * AST CONTRACT
 * ============================================================================
 *
 * The parser must preserve whether a return type was syntactically supplied.
 *
 * The frontend AST should represent:
 *
 *     no return annotation
 *
 * separately from:
 *
 *     explicit return annotation
 *
 * where that distinction is semantically relevant.
 *
 * The AST should contain the parsed TypeExpression and its source span.
 *
 * This grammar does not define the AST data structure.
 *
 * ============================================================================
 *
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis is responsible for determining:
 *
 *     - whether the referenced type exists;
 *     - whether it is well formed;
 *     - whether generic parameters are valid;
 *     - whether bounds are satisfied;
 *     - whether inferred return values match the declared type;
 *     - whether control-flow paths return compatible values;
 *     - whether `void`/unit semantics are valid;
 *     - whether quantum return semantics are valid;
 *     - whether hardware/resource constraints are satisfiable;
 *     - whether the type is supported by a selected compilation target.
 *
 * The grammar performs none of these checks.
 *
 * ============================================================================
 *
 * IR CONTRACT
 * ============================================================================
 *
 * This grammar must not construct canonical IR.
 *
 * The frontend converts the parsed return type into the language's canonical
 * AST/semantic representation.
 *
 * Semantic analysis then lowers that representation into the appropriate IR.
 *
 * For quantum computations:
 *
 *     source return syntax
 *          |
 *          v
 *     frontend AST
 *          |
 *          v
 *     semantic/type analysis
 *          |
 *          v
 *     quantum semantic representation
 *          |
 *          v
 *     quantum::ir
 *
 * There must be no:
 *
 *     grammar -> quantum::ir -> grammar
 *
 * dependency.
 *
 * ============================================================================
 *
 * COMPILER CONTRACT
 * ============================================================================
 *
 * Compilation stages downstream may use the semantic return type for:
 *
 *     - type checking;
 *     - ABI selection;
 *     - calling convention selection;
 *     - result lowering;
 *     - ownership analysis;
 *     - effect analysis;
 *     - resource analysis;
 *     - optimization;
 *     - code generation;
 *     - target lowering.
 *
 * None of those decisions belong in this file.
 *
 * ============================================================================
 *
 * RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime behavior is determined by the lowered program and execution model.
 *
 * This grammar does not prescribe:
 *
 *     - stack return;
 *     - register return;
 *     - heap return;
 *     - message return;
 *     - network return;
 *     - quantum measurement transport;
 *     - device result transport.
 *
 * ============================================================================
 *
 * TOOLING CONTRACT
 * ============================================================================
 *
 * Tools may use this grammar delegate for:
 *
 *     - syntax highlighting;
 *     - parsing;
 *     - AST indexing;
 *     - signature extraction;
 *     - documentation generation;
 *     - IDE navigation;
 *     - refactoring;
 *     - source formatting.
 *
 * Tooling must use the same canonical parser/token vocabulary and must not
 * create an incompatible return-type grammar.
 *
 * ============================================================================
 *
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * Canonical syntax:
 *
 *     -> TypeExpression
 *
 * remains the sole return annotation syntax defined here.
 *
 * Existing parser implementations that currently define:
 *
 *     returnType
 *         : THIN_ARROW typeExpression
 *         ;
 *
 * should migrate to this reusable delegate.
 *
 * The migration must preserve the same accepted source syntax.
 *
 * ============================================================================
 *
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * This file is a parser delegate.
 *
 * It expects the importing/root parser to provide:
 *
 *     typeExpression
 *
 * and the lexer vocabulary to provide:
 *
 *     THIN_ARROW
 *
 * It intentionally does not redefine either symbol.
 *
 * A consuming function grammar should import this delegate and use:
 *
 *     returnType?
 *
 * after its parameter list.
 *
 * Example:
 *
 *     functionDeclaration
 *         : ...
 *           LPAREN parameterList? RPAREN
 *           returnType?
 *           ...
 *         ;
 *
 * ============================================================================
 *
 * ANTLR IMPORT CONTRACT
 * ============================================================================
 *
 * Intended integration:
 *
 *     parser grammar Functions;
 *
 *     options {
 *         tokenVocab = ZamaniLexer;
 *     }
 *
 *     import Parameters;
 *     import Returns;
 *     ...
 *
 * The root parser ultimately imports/uses the function grammar.
 *
 * Imported parser rules are part of the parser composition model; this file
 * therefore remains deliberately independent of the root parser's declaration
 * rule.
 *
 * ============================================================================
 *
 * MIGRATION REQUIREMENTS
 * ============================================================================
 *
 * Existing duplicate rules must be removed from their previous owners once
 * the importing grammar is migrated.
 *
 * In particular, the following duplicate implementation:
 *
 *     returnType
 *         : THIN_ARROW typeExpression
 *         ;
 *
 * in `grammar/antlr/Core.g4`
 *
 * should no longer remain there once `Returns` is integrated.
 *
 * Likewise, the duplicate `returnType` rule in:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * should be migrated to this reusable grammar component.
 *
 * There must be exactly one authoritative reusable concrete-syntax
 * implementation of the function return annotation.
 *
 * ============================================================================
 *
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive syntax tests MUST include:
 *
 *     fn f() { }
 *
 *     fn f() -> void { }
 *
 *     fn f() -> int { }
 *
 *     fn f() -> SomeType { }
 *
 *     fn f() -> A::B { }
 *
 *     fn f() -> Generic<T> { }
 *
 *     fn f() -> (A, B) { }
 *
 * where each referenced type is valid under the active type grammar.
 *
 * Cross-domain tests MUST include valid type expressions representing:
 *
 *     - classical results;
 *     - numerical results;
 *     - generic results;
 *     - collection results;
 *     - quantum results;
 *     - hybrid results;
 *     - hardware/HDL results where defined;
 *     - distributed results;
 *     - accelerator results.
 *
 * ============================================================================
 *
 * NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * The following must be rejected as malformed return syntax:
 *
 *     fn f() -> { }
 *
 *     fn f() -> ;
 *
 *     fn f() -> ) { }
 *
 *     fn f() -> -> Int { }
 *
 *     fn f() -> , { }
 *
 *     fn f() -> Int, Float { }
 *
 * unless a future specification explicitly changes the syntax.
 *
 * Semantic-invalid types are NOT necessarily parser-negative tests.
 *
 * For example, if:
 *
 *     fn f() -> UnknownType { }
 *
 * is syntactically valid,
 *
 * then the parser should accept it and semantic analysis should diagnose
 * `UnknownType`.
 *
 * ============================================================================
 *
 * BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Tests must verify that return syntax remains valid for arbitrarily deep
 * type expressions supported by the type grammar, subject only to explicit
 * implementation/resource limits outside the language semantics.
 *
 * Examples include:
 *
 *     fn f() -> Outer<Inner<Value>> { }
 *
 *     fn f() -> A::B::C::D { }
 *
 *     fn f() -> Result<Tuple<A, B>, E> { }
 *
 * The grammar must not introduce a fixed nesting depth.
 *
 * ============================================================================
 *
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * Given identical source text and identical lexer/parser configuration:
 *
 *     source -> tokens -> parse tree
 *
 * must be deterministic.
 *
 * This file contains no semantic state, global mutation, I/O, filesystem
 * access, network access, timing dependency, or random behavior.
 *
 * ============================================================================
 *
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * No machine-specific hard-coding is permitted here.
 *
 * Allowed:
 *
 *     `THIN_ARROW`
 *     `TypeExpression`
 *
 * because these are language-level syntax concepts.
 *
 * Forbidden:
 *
 *     maximum return type size;
 *     maximum return count;
 *     CPU return registers;
 *     GPU result slots;
 *     QPU result slots;
 *     physical qubit count;
 *     memory capacity;
 *     hardware address;
 *     device identifier;
 *     ABI-specific machine layout.
 *
 * ============================================================================
 *
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is COMPLETE when:
 *
 * 1. The canonical syntax `-> TypeExpression` is represented exactly once
 *    by this reusable delegate.
 *
 * 2. `THIN_ARROW` is consumed from the canonical lexer vocabulary.
 *
 * 3. `TypeExpression` is consumed from the canonical type grammar.
 *
 * 4. No identifier, type, ABI, hardware, quantum, or runtime grammar is
 *    duplicated here.
 *
 * 5. Function declaration grammar can import and consume `returnType`.
 *
 * 6. Existing Core/ZamaniParser duplicate `returnType` definitions are
 *    migrated away without changing canonical source semantics.
 *
 * 7. Valid return annotations parse deterministically.
 *
 * 8. Malformed return annotations are rejected deterministically.
 *
 * 9. No finite machine/resource limitation exists in the grammar.
 *
 * 10. AST/semantic/IR ownership remains downstream and unambiguous.
 *
 * 11. Classical, quantum, HDL, distributed, accelerator, and future type
 *     systems can use the same return syntax without this file changing.
 *
 * 12. Parser tests, negative tests, boundary tests, compatibility tests,
 *     and cross-domain tests pass.
 *
 * 13. Rust integration remains compatible with Rust 1.97 / 1.97.1 and uses
 *     no Rust `unsafe`.
 *
 * ============================================================================
 */


/*
 * ============================================================================
 * FUNCTION RETURN TYPE
 * ============================================================================
 *
 * Canonical specification:
 *
 *     FunctionDeclaration ::=
 *         ...
 *         ["->" TypeExpression]
 *         ...
 *
 * The optionality belongs to the consuming function declaration/signature.
 * This rule represents the return clause itself.
 *
 * `THIN_ARROW` is owned by the canonical lexer.
 * `typeExpression` is owned by the canonical type grammar.
 * ============================================================================
 */

returnType
    : THIN_ARROW typeExpression
    ;
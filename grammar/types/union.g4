/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/union.g4
 *
 * Grammar:
 *     Union
 *
 * Status:
 *     CANONICAL SOURCE-LEVEL UNION-TYPE DELEGATE
 *
 * Compiler baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     safe Rust only
 *     no unsafe
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file owns ONLY anonymous/source-level UNION TYPE syntax.
 *
 * Examples:
 *
 *     int | string
 *     Success | Failure
 *     Qubit | ClassicalValue
 *     Result<T, E> | Pending
 *     Some<T> | None
 *
 * A union type expresses that a value may have one of several alternative
 * source-level types.
 *
 * This is distinct from a named union declaration:
 *
 *     union Result<T, E> = Ok(T) | Err(E);
 *
 * Named union declarations are owned by:
 *
 *     grammar/declarations/unions.g4
 *
 * Algebraic constructor/sum syntax is owned by:
 *
 *     grammar/types/algebraic-types.g4
 *
 * This file must not duplicate either of those responsibilities.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     ANTLR parser
 *          |
 *          v
 *     Types.typeExpression
 *          |
 *          v
 *     Union.typeUnion
 *          |
 *          v
 *     domain-neutral frontend TypeExpr
 *          |
 *          v
 *     structural validation
 *          |
 *          v
 *     semantic type resolution
 *          |
 *          +----------------------+----------------------+
 *          |                      |                      |
 *          v                      v                      v
 *      classical             quantum semantics      HDL/hardware
 *      semantics                  |                  semantics
 *                                 v
 *                             quantum::ir
 *                                 |
 *                         optimization/lowering
 *                                 |
 *                         routing/scheduling
 *                                 |
 *                          resilience/QEC
 *                                 |
 *                                ZQN
 *                                 |
 *                                HAL
 *                                 |
 *                         target realization
 *
 * This grammar has no direct dependency on hardware, runtime, QEC, ZQN,
 * scheduling, routing or HAL.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     typeUnion
 *     unionTypeMember
 *
 * THIS FILE DOES NOT OWN:
 *
 *     typeExpression
 *     typeCore
 *     typeQualifier
 *     typePostfix
 *     primitiveType
 *     namedType
 *     genericType
 *     tupleType
 *     arrayType
 *     sliceType
 *     functionType
 *     referenceType
 *     pointerType
 *     resultType
 *     optionalType
 *     quantumType
 *     temporalType
 *     dependentType
 *
 * It also does not own:
 *
 *     named union declarations
 *     enum declarations
 *     algebraic constructor declarations
 *     pattern matching
 *     guard semantics
 *     exhaustiveness checking
 *     type inference
 *     type unification
 *     type normalization
 *     type equivalence
 *     resource allocation
 *     capability discovery
 *     hardware selection
 *     quantum allocation
 *     routing
 *     scheduling
 *     QEC
 *     ZQN
 *     HAL
 *     runtime representation
 *     ABI layout
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     canonical lexer vocabulary
 *     typeCore
 *     typePostfix
 *
 * EXPORTS:
 *
 *     typeUnion
 *     unionTypeMember
 *
 * CONSUMED_BY:
 *
 *     grammar/types/types.g4
 *
 * AST_OWNER:
 *
 *     existing frontend TypeExpr / type representation
 *
 * SEMANTIC_OWNER:
 *
 *     canonical semantic type system
 *
 * IR_OWNER:
 *
 *     canonical semantic IR and applicable domain IR
 *
 * SPEC_OWNER:
 *
 *     authoritative type-system specification
 *
 * TEST_OWNER:
 *
 *     grammar/tests/types/union/
 *
 * ============================================================================
 * COMPOSITION CONTRACT
 * ============================================================================
 *
 * This is a parser delegate.
 *
 * The canonical composition owner remains:
 *
 *     grammar/types/types.g4
 *
 * `types.g4` must import this grammar.
 *
 * This file MUST NOT import `Types`.
 *
 * The dependency direction is:
 *
 *     Types
 *       |
 *       v
 *     Union
 *
 * not:
 *
 *     Types <-> Union
 *
 * The delegate consumes the `typeCore` and `typePostfix` rules supplied by
 * the canonical Types grammar.
 *
 * This prevents a circular grammar dependency and prevents this file from
 * creating a second type-expression authority.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This grammar requires only the canonical PIPE token.
 *
 * The token MUST come from:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * through the repository's canonical lexer vocabulary.
 *
 * This file MUST NOT:
 *
 *     - define PIPE;
 *     - define another union operator;
 *     - introduce UNION_TYPE;
 *     - introduce a second vertical-bar token;
 *     - define lexer actions.
 *
 * The `|` spelling is punctuation/operator syntax, not a new keyword.
 *
 * ============================================================================
 * UNION TYPE MODEL
 * ============================================================================
 *
 * Canonical source form:
 *
 *     A | B
 *
 * More alternatives:
 *
 *     A | B | C
 *
 * Nested examples:
 *
 *     Option<T> | Error
 *
 *     Qubit | ClassicalValue
 *
 *     int | float | decimal
 *
 *     fn(A) -> B | Error
 *
 *     Result<T, E> | Pending
 *
 * The semantic model is conceptually:
 *
 *     Union(
 *         [A, B, C]
 *     )
 *
 * The exact Rust AST/semantic representation is owned by the existing
 * frontend type system.
 *
 * This grammar MUST NOT create a second Rust-specific union representation.
 *
 * ============================================================================
 * MINIMUM CARDINALITY
 * ============================================================================
 *
 * A union type requires at least TWO alternatives.
 *
 * Therefore:
 *
 *     int
 *
 * is an ordinary type, not a union type.
 *
 *     int | string
 *
 * is a union type.
 *
 * The grammar does not need a special single-member union representation.
 *
 * ============================================================================
 * ALTERNATIVE ORDER
 * ============================================================================
 *
 * Source ordering MUST be preserved.
 *
 * For:
 *
 *     A | B | C
 *
 * the parser must preserve:
 *
 *     A
 *     B
 *     C
 *
 * in that order.
 *
 * Whether semantic normalization later treats union alternatives as
 * commutative is a semantic/type-system decision and must not destroy source
 * information required for diagnostics, tooling or provenance.
 *
 * ============================================================================
 * DUPLICATE ALTERNATIVES
 * ============================================================================
 *
 * The grammar accepts syntactically valid duplicate alternatives:
 *
 *     A | A
 *
 * if the syntax is otherwise valid.
 *
 * Duplicate detection is NOT a parser responsibility.
 *
 * Semantic analysis decides whether:
 *
 *     A | A
 *
 * is:
 *
 *     - rejected as redundant;
 *     - normalized to A;
 *     - accepted with a warning;
 *     - handled according to a future type-equivalence policy.
 *
 * This separation is important because semantic type equivalence may require
 * name resolution, generic substitution or normalization.
 *
 * ============================================================================
 * NESTED UNION TYPES
 * ============================================================================
 *
 * The grammar must support nested unions through the canonical type system.
 *
 * Examples:
 *
 *     (A | B)
 *
 *     Option<A | B>
 *
 *     Result<A | B, E>
 *
 *     fn(A | B) -> C
 *
 * Parenthesized unions use the canonical parenthesized type grammar.
 *
 * Generic arguments use the canonical generic-type grammar.
 *
 * This file must not recreate either syntax.
 *
 * ============================================================================
 * POSTFIX INTEGRATION
 * ============================================================================
 *
 * Union alternatives consume `typePostfix`.
 *
 * Therefore constructs such as:
 *
 *     A? | B
 *
 * can be represented without requiring this grammar to redefine optional
 * syntax.
 *
 * Likewise:
 *
 *     A | B?
 *
 * remains structurally distinct from:
 *
 *     (A | B)?
 *
 * where both forms are supported by the canonical type system.
 *
 * The semantic type system determines their precise meaning.
 *
 * ============================================================================
 * QUALIFIER INTEGRATION
 * ============================================================================
 *
 * Type qualifiers remain owned by `Types.typeExpression`.
 *
 * This grammar must not redefine:
 *
 *     linear
 *     affine
 *
 * or future type qualifiers.
 *
 * If the canonical type system permits qualified union alternatives, that
 * behavior must be introduced through the canonical `typeExpression` /
 * `typeCore` composition rather than duplicated here.
 *
 * ============================================================================
 * FUNCTION TYPE INTEGRATION
 * ============================================================================
 *
 * Function types remain owned by:
 *
 *     grammar/types/function.g4
 *
 * A function type may participate in a union:
 *
 *     fn(A) -> B | fn(C) -> D
 *
 * provided the canonical type composition parses the function-type members
 * correctly.
 *
 * This file does not redefine `functionType`.
 *
 * ============================================================================
 * GENERIC INTEGRATION
 * ============================================================================
 *
 * Generic applications remain owned by the canonical type grammar.
 *
 * Examples:
 *
 *     Vec<int> | Vec<float>
 *
 *     Result<T, E> | Pending
 *
 *     QuantumState<T> | ClassicalState<T>
 *
 * This grammar merely consumes the already-composed type member.
 *
 * It does not define:
 *
 *     genericType
 *     typeArguments
 *     genericParameterList
 *
 * ============================================================================
 * DEPENDENT / VALUE-PARAMETERIZED TYPE INTEGRATION
 * ============================================================================
 *
 * Union alternatives may contain dependent/value-parameterized types:
 *
 *     Vector<T>[N] | EmptyVector<T>
 *
 *     Matrix<T>[Rows, Cols] | SparseMatrix<T>
 *
 * Symbolic values remain owned by the canonical dependent-type grammar.
 *
 * This file never evaluates:
 *
 *     N
 *     Rows
 *     Cols
 *
 * and never converts them into fixed machine limits.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum types may participate in unions:
 *
 *     Qubit | ClassicalValue
 *
 *     LogicalQubit | Measurement
 *
 *     QuantumState<T> | Error
 *
 *     QRegister<N> | EmptyRegister
 *
 * This does NOT imply:
 *
 *     physical qubit allocation;
 *     QPU selection;
 *     topology selection;
 *     gate selection;
 *     calibration;
 *     routing;
 *     scheduling;
 *     QEC;
 *     resilience;
 *     ZQN generation.
 *
 * The grammar only represents source-level type alternatives.
 *
 * Downstream:
 *
 *     source type
 *       |
 *       v
 *     TypeExpr
 *       |
 *       v
 *     semantic type
 *       |
 *       v
 *     quantum semantic analysis
 *       |
 *       v
 *     quantum::ir
 *
 * where applicable.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware/HDL types may participate in unions:
 *
 *     Signal<T> | ControlSignal
 *
 *     Register<T> | MemoryReference
 *
 *     HardwareValue | SoftwareValue
 *
 * This grammar does not encode:
 *
 *     register width;
 *     bus width;
 *     pin count;
 *     device count;
 *     memory capacity;
 *     FPGA capacity;
 *     ASIC capacity;
 *     accelerator count;
 *     physical topology.
 *
 * ============================================================================
 * RESOURCE / CAPABILITY INTEGRATION
 * ============================================================================
 *
 * A union type is a type-level semantic choice.
 *
 * It does not allocate or reserve resources.
 *
 * For example:
 *
 *     Qubit | ClassicalValue
 *
 * does not request one QPU.
 *
 * Resource requirements remain owned by:
 *
 *     grammar/resources/
 *
 * Capabilities remain owned by:
 *
 *     grammar/core/capabilities.g4
 *
 * Policies remain owned by:
 *
 *     grammar/policies/
 *
 * Effects remain owned by:
 *
 *     grammar/effects/
 *
 * ============================================================================
 * AI / DATA / DISTRIBUTED INTEGRATION
 * ============================================================================
 *
 * The union grammar is domain-neutral.
 *
 * Therefore future types such as:
 *
 *     Tensor<T>
 *     Model<I, O>
 *     Dataset<T>
 *     AgentState
 *     DistributedValue<T>
 *     NetworkMessage<T>
 *
 * can participate without changing this file.
 *
 * This is critical for ecosystem scalability.
 *
 * ============================================================================
 * ALGEBRAIC TYPE DISTINCTION
 * ============================================================================
 *
 * DO NOT confuse:
 *
 *     A | B
 *
 * with a named algebraic declaration:
 *
 *     union Result = Ok(A) | Err(B);
 *
 * or with:
 *
 *     grammar/types/algebraic-types.g4
 *
 * Algebraic constructor syntax describes constructors and alternatives.
 *
 * This file describes an anonymous type-level union of existing types.
 *
 * The two concepts may eventually share semantic machinery downstream, but
 * their source syntax and ownership remain distinct.
 *
 * ============================================================================
 * PATTERN MATCHING INTEGRATION
 * ============================================================================
 *
 * Union types may be consumed by pattern matching.
 *
 * However:
 *
 *     match
 *     pattern
 *     guard
 *     exhaustiveness
 *
 * remain owned by:
 *
 *     grammar/expressions/
 *     grammar/statements/
 *     semantic validation
 *
 * This grammar does not introduce pattern syntax.
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Contracts may constrain values whose type is a union.
 *
 * Example semantic intent:
 *
 *     requires value is A | B
 *
 * or:
 *
 *     ensures result satisfies condition
 *
 * Contract syntax remains owned by:
 *
 *     grammar/validation/
 *
 * This file must not introduce `requires`, `ensures`, `invariant`, or
 * equivalent contract syntax.
 *
 * ============================================================================
 * EFFECT INTEGRATION
 * ============================================================================
 *
 * A union type does not inherently introduce an effect.
 *
 * The semantic type system may later determine effect relationships for
 * particular contained types.
 *
 * This file must not define:
 *
 *     unionEffect
 *     quantumUnionEffect
 *     aiUnionEffect
 *
 * or another parallel effect vocabulary.
 *
 * ============================================================================
 * PROVENANCE
 * ============================================================================
 *
 * Source spans for:
 *
 *     complete union;
 *     each member;
 *     each separator;
 *
 * must remain available through the existing parser/frontend location system.
 *
 * This enables:
 *
 *     diagnostics;
 *     IDE tooling;
 *     refactoring;
 *     formatting;
 *     documentation;
 *     provenance;
 *     compatibility analysis.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar produces ANTLR parser contexts only.
 *
 * It does NOT define Rust AST structures.
 *
 * The frontend must map:
 *
 *     typeUnion
 *
 * into the existing canonical type representation.
 *
 * Conceptually:
 *
 *     TypeExpr::Union(
 *         members
 *     )
 *
 * ONLY if the existing frontend type model already has this representation.
 *
 * If the existing AST uses another canonical representation, this grammar
 * must use that representation rather than introducing a second one.
 *
 * Required AST properties:
 *
 *     - member ordering;
 *     - source locations;
 *     - nested type structure;
 *     - generic arguments;
 *     - qualifiers where applicable.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis owns:
 *
 *     - member type resolution;
 *     - duplicate detection;
 *     - type normalization;
 *     - subtype relationships;
 *     - assignability;
 *     - coercion;
 *     - narrowing;
 *     - pattern exhaustiveness;
 *     - generic substitution;
 *     - recursive-type validity;
 *     - type equivalence;
 *     - representation decisions.
 *
 * The parser only establishes structure.
 *
 * ============================================================================
 * TYPE-CHECKING INTEGRATION
 * ============================================================================
 *
 * The type checker must define at least:
 *
 *     construction;
 *     assignment;
 *     parameter passing;
 *     return compatibility;
 *     pattern matching;
 *     narrowing;
 *     equality/compatibility;
 *     generic substitution.
 *
 * Example:
 *
 *     let x: int | string = ...
 *
 * The grammar accepts the type.
 *
 * The semantic layer determines which values may be assigned to `x`.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * This file does not create an IR.
 *
 * Union semantics may lower to:
 *
 *     tagged representation;
 *     discriminated representation;
 *     variant representation;
 *     nullable/optional representation;
 *     target-specific representation;
 *
 * depending on semantic meaning and target capabilities.
 *
 * The grammar MUST NOT select the representation.
 *
 * For quantum-containing unions, any quantum-specific executable semantics
 * eventually cross:
 *
 *     quantum::ir
 *
 * rather than creating a union-specific quantum IR.
 *
 * ============================================================================
 * RESOURCE SCALABILITY
 * ============================================================================
 *
 * There is no language-defined maximum for:
 *
 *     union member count;
 *     nesting depth;
 *     generic arity;
 *     type-expression size;
 *     symbolic dimension complexity;
 *     number of union declarations;
 *     number of uses.
 *
 * Example conceptual sequence:
 *
 *     A | B
 *     A | B | C
 *     A | B | C | D
 *     ...
 *
 * The grammar never changes to enumerate a finite set of alternatives.
 *
 * Practical parser/compiler resource limits may exist, but must be:
 *
 *     explicit;
 *     configurable where appropriate;
 *     documented;
 *     reported as implementation/resource diagnostics;
 *
 * rather than silently becoming language semantics.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing MUST depend only on:
 *
 *     token sequence;
 *     grammar;
 *     parser configuration.
 *
 * It MUST NOT depend on:
 *
 *     CPU model;
 *     GPU availability;
 *     QPU availability;
 *     network state;
 *     memory topology;
 *     runtime state;
 *     random hardware behavior;
 *     calibration.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar:
 *
 *     - performs no I/O;
 *     - performs no network operations;
 *     - executes no source code;
 *     - queries no hardware;
 *     - accesses no filesystem;
 *     - performs no resource allocation;
 *     - contains no parser actions;
 *     - contains no unsafe Rust.
 *
 * ============================================================================
 * PERFORMANCE
 * ============================================================================
 *
 * The grammar must remain structurally predictable.
 *
 * The union operator is represented by a simple repetition:
 *
 *     member (PIPE member)+
 *
 * rather than an ambiguous recursive alternative.
 *
 * This avoids unnecessary parser ambiguity and does not require backtracking
 * through an unbounded binary tree.
 *
 * ============================================================================
 * DIAGNOSTICS
 * ============================================================================
 *
 * Syntax errors include:
 *
 *     int |
 *     | string
 *     int || string
 *     int | | string
 *
 *     int | 
 *
 *     <missing member>
 *
 * Semantic errors include:
 *
 *     duplicate equivalent members;
 *     unresolved member types;
 *     illegal recursive relationships;
 *     incompatible generic arguments.
 *
 * These must not be reported as parser errors when the syntax itself is
 * valid.
 *
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 *
 * This grammar introduces anonymous union types only through the canonical
 * type-expression composition.
 *
 * Existing named union declarations remain governed by:
 *
 *     grammar/declarations/unions.g4
 *
 * No existing declaration syntax is changed by this file itself.
 *
 * If the language previously used `|` exclusively for another type-level
 * meaning, compatibility resolution must occur before enabling this grammar
 * in `types.g4`.
 *
 * ============================================================================
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
 *
 *     MAX_VARIANTS
 *     MAX_UNION_MEMBERS
 *     MAX_TYPES
 *     MAX_GENERIC_ARITY
 *     MAX_TYPE_DEPTH
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *
 * It contains no finite enumeration of alternatives.
 *
 * ============================================================================
 * REQUIRED INTEGRATION
 * ============================================================================
 *
 * `grammar/types/types.g4` must:
 *
 *     1. import `Union`;
 *     2. retain ownership of `typeExpression`;
 *     3. retain ownership of `typeCore`;
 *     4. expose `typeUnion` through `typeExpression`;
 *     5. remove any duplicate union-type rules if they exist.
 *
 * The canonical composition should conceptually be:
 *
 *     typeExpression
 *         : typeQualifier* typeUnion
 *         ;
 *
 *     typeUnion
 *         : unionTypeMember (PIPE unionTypeMember)*
 *         ;
 *
 *     unionTypeMember
 *         : typeCore typePostfix*
 *         ;
 *
 * However, because `typeUnion` must represent a union only when there are at
 * least two members, the production below is the preferred canonical form:
 *
 *     typeUnion
 *         : unionTypeMember (PIPE unionTypeMember)+
 *         ;
 *
 * and the surrounding `typeExpression` must permit a non-union member as well.
 *
 * Therefore the recommended final composition is:
 *
 *     typeExpression
 *         : typeQualifier*
 *           (typeUnion | unionTypeMember)
 *         ;
 *
 * This keeps the specialized union delegate responsible for union syntax
 * while Types remains responsible for the complete type-expression entry
 * point.
 *
 * ============================================================================
 * IMPORTANT INTEGRATION NOTE
 * ============================================================================
 *
 * The current `types.g4` architecture already has:
 *
 *     typeExpression
 *         : typeQualifier* typeCore typePostfix*
 *         ;
 *
 * Therefore integrating this file requires a deliberate refactoring of that
 * composition.
 *
 * Do NOT simply add:
 *
 *     | unionType
 *
 * to `typeCore`
 *
 * while defining unionType in terms of `typeExpression`.
 *
 * That would create recursive type-expression ambiguity.
 *
 * Instead, Types should factor the grammar into:
 *
 *     typeExpression
 *         : typeQualifier* typeUnionExpression
 *         ;
 *
 *     typeUnionExpression
 *         : unionType
 *         | unionTypeMember
 *         ;
 *
 * with:
 *
 *     unionType
 *         : unionTypeMember (PIPE unionTypeMember)+
 *         ;
 *
 * and:
 *
 *     unionTypeMember
 *         : typeCore typePostfix*
 *         ;
 *
 * The exact public rule names may remain:
 *
 *     typeExpression
 *     typeCore
 *     typePostfix
 *
 * as long as ownership remains unambiguous.
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive:
 *
 *     int | string
 *
 *     int | float | string
 *
 *     User | Error
 *
 *     Option<T> | Error
 *
 *     Qubit | ClassicalValue
 *
 *     Result<T, E> | Pending
 *
 *     A? | B
 *
 *     A | B?
 *
 *     (A | B)
 *
 *     Option<A | B>
 *
 *     fn(A | B) -> C
 *
 *     fn(A) -> B | C
 *
 *     Vector<T>[N] | Empty
 *
 * Negative:
 *
 *     |
 *
 *     | A
 *
 *     A |
 *
 *     A || B
 *
 *     A | | B
 *
 *     A | | | B
 *
 *     A |
 *     B
 *
 *     A B
 *
 * Boundary:
 *
 *     A | B
 *
 *     A | B | C
 *
 *     deeply nested union members
 *
 *     generic union members
 *
 *     function union members
 *
 *     quantum union members
 *
 *     hardware/HDL union members
 *
 *     dependent union members
 *
 * Scalability:
 *
 * Generate unions with increasing member counts:
 *
 *     T0 | T1
 *     T0 | T1 | T2
 *     ...
 *
 * without defining a maximum language-level count.
 *
 * The scalability harness must measure implementation behavior rather than
 * define a language ceiling.
 *
 * Determinism:
 *
 * The same token stream must yield the same parse structure and member order.
 *
 * Compatibility:
 *
 * Existing type syntax must continue to parse identically except where the
 * explicit introduction of anonymous union types intentionally assigns `|`
 * the newly specified type-level meaning.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 * [ ] It is the sole owner of anonymous union-type syntax.
 *
 * [ ] Named union declarations remain owned by declarations/unions.g4.
 *
 * [ ] Algebraic constructor syntax remains owned by algebraic-types.g4.
 *
 * [ ] It imports no competing type grammar.
 *
 * [ ] It defines no lexer rules.
 *
 * [ ] It introduces no hardware/resource limits.
 *
 * [ ] It introduces no Rust actions.
 *
 * [ ] It requires no unsafe Rust.
 *
 * [ ] It has a canonical integration contract with types.g4.
 *
 * [ ] `int | string` parses.
 *
 * [ ] Multiple alternatives parse.
 *
 * [ ] Nested types parse.
 *
 * [ ] Generic members parse.
 *
 * [ ] Function members parse.
 *
 * [ ] Quantum members parse.
 *
 * [ ] HDL/hardware members parse where those types are legal.
 *
 * [ ] Dependent/value-parameterized members parse.
 *
 * [ ] Invalid empty alternatives are rejected.
 *
 * [ ] Duplicate alternatives are handled semantically, not lexically.
 *
 * [ ] Source order is preserved.
 *
 * [ ] Source spans are preserved downstream.
 *
 * [ ] No artificial member-count limit exists.
 *
 * [ ] Positive tests pass.
 *
 * [ ] Negative tests pass.
 *
 * [ ] Boundary tests pass.
 *
 * [ ] Scalability tests pass.
 *
 * [ ] Determinism tests pass.
 *
 * [ ] Compatibility tests pass.
 *
 * [ ] AST mapping is confirmed against the existing frontend type model.
 *
 * [ ] Semantic union normalization is defined downstream.
 *
 * [ ] IR representation remains downstream.
 *
 * [ ] Quantum lowering, when applicable, reaches `quantum::ir`.
 *
 * ============================================================================
 * END OF FILE
 * ============================================================================
 */

parser grammar Union;

typeUnion
    : unionTypeMember
      (PIPE unionTypeMember)+
    ;

unionTypeMember
    : typeCore
      typePostfix*
    ;
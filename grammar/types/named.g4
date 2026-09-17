/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/named.g4
 *
 * Status:
 *     Production-ready modular SOURCE TYPE grammar component.
 *
 * Purpose:
 *     Defines unresolved named-type syntax and qualified type paths.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - namedType;
 *   - typePath;
 *   - typePathSegment;
 *   - the source-level qualification separator used by type paths;
 *   - ordered type-name path structure;
 *   - syntactic recognition of an unresolved type name;
 *   - preservation of arbitrary namespace/module qualification depth.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - identifier lexical spelling;
 *   - keyword/token definitions;
 *   - generic type application;
 *   - generic argument lists;
 *   - generic parameter declarations;
 *   - type aliases;
 *   - primitive types;
 *   - tuple types;
 *   - array/slice types;
 *   - function types;
 *   - reference/pointer types;
 *   - optional/result types;
 *   - quantum types;
 *   - classical types;
 *   - HDL types;
 *   - hardware types;
 *   - resource types;
 *   - capability types;
 *   - dependent type semantics;
 *   - name resolution;
 *   - import resolution;
 *   - module resolution;
 *   - type inference;
 *   - generic substitution;
 *   - overload resolution;
 *   - trait/interface resolution;
 *   - resource allocation;
 *   - hardware discovery;
 *   - physical qubit allocation;
 *   - routing;
 *   - scheduling;
 *   - calibration;
 *   - QEC;
 *   - ZQN;
 *   - HAL;
 *   - compiler backend selection;
 *   - runtime representation;
 *   - ABI layout.
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
 *     parser
 *          |
 *          v
 *     NamedTypes
 *          |
 *          v
 *     TypeExpr::Identifier(TypePath)
 *          |
 *          v
 *     structural AST validation
 *          |
 *          v
 *     semantic name/type resolution
 *          |
 *          v
 *     SemanticType
 *          |
 *          +----------------------+----------------------+
 *          |                      |                      |
 *          v                      v                      v
 *      classical             quantum::ir          HDL/resource
 *       semantics             semantics             semantics
 *          |                      |                      |
 *          +----------------------+----------------------+
 *                                 |
 *                                 v
 *                         canonical semantic IR
 *                                 |
 *                     optimization / lowering
 *                                 |
 *                     routing / scheduling
 *                                 |
 *                       resilience / QEC / ZQN
 *                                 |
 *                                HAL
 *                                 |
 *                         target realization
 *
 * IMPORTANT:
 *
 * A named type is an unresolved SOURCE-LEVEL reference.
 *
 * The grammar must never resolve a name to:
 *
 *     - a machine type;
 *     - a physical resource;
 *     - a backend;
 *     - a device;
 *     - a physical qubit;
 *     - a memory bank;
 *     - a vendor implementation;
 *     - an ABI representation.
 *
 * ============================================================================
 * CANONICAL AST CONTRACT
 * ============================================================================
 *
 * A successful `namedType` parse represents:
 *
 *     TypeExpr::Identifier(TypePath)
 *
 * The canonical frontend already contains:
 *
 *     TypePath
 *     TypeName
 *     NamedType
 *     TypeExpr
 *
 * `TypePath` preserves ordered source-level path segments.
 *
 * The grammar MUST NOT introduce another AST representation for paths.
 *
 * Generic application is represented separately:
 *
 *     Foo<T>
 *        |
 *        v
 *     TypeExpr::Generic
 *
 * while:
 *
 *     Foo
 *     module::Foo
 *     a::b::Foo
 *
 * maps to:
 *
 *     TypeExpr::Identifier(TypePath)
 *
 * ============================================================================
 * GENERIC INTEGRATION
 * ============================================================================
 *
 * This file intentionally DOES NOT define:
 *
 *     genericType
 *     genericArguments
 *     genericArgument
 *
 * Those belong to:
 *
 *     grammar/types/generic-types.g4
 *
 * Therefore:
 *
 *     module::Type
 *
 * is a named type.
 *
 *     module::Type<T>
 *
 * is a generic type application whose BASE is the named type path:
 *
 *     module::Type
 *
 * The generic grammar owns the `<...>` portion.
 *
 * This separation is mandatory because otherwise named.g4 and
 * generic-types.g4 would compete to parse the same source construct.
 *
 * ============================================================================
 * TYPE COMPOSITION INTEGRATION
 * ============================================================================
 *
 * `grammar/types/types.g4` is the public type-expression composition grammar.
 *
 * After integration it MUST:
 *
 *     import NamedTypes;
 *
 * and its type composition MUST delegate named-type syntax to:
 *
 *     namedType
 *
 * rather than defining another `namedType` or `typePath`.
 *
 * The following rules MUST therefore be removed from the composition grammar
 * when this delegate is integrated:
 *
 *     namedType
 *     typePath
 *
 * No semantic behavior is moved into this file.
 *
 * ============================================================================
 * ROOT-GRAMMAR INTEGRATION
 * ============================================================================
 *
 * `grammar/Zamani.g4` remains the language composition/root grammar.
 *
 * It must ultimately consume the canonical `typeExpression` supplied by the
 * modular type grammar rather than defining an independent named-type grammar.
 *
 * This file MUST NOT be imported directly by application/runtime code.
 *
 * ============================================================================
 * PATH INTEGRATION
 * ============================================================================
 *
 * The repository distinguishes type paths from general source/resource paths.
 *
 * This grammar therefore owns:
 *
 *     typePath
 *
 * and does NOT reuse or redefine a general filesystem, URI, source-unit,
 * address, deployment, or resource path.
 *
 * A TypePath is specifically:
 *
 *     ordered type-name segments
 *
 * separated by:
 *
 *     ::
 *
 * ============================================================================
 * NAME RESOLUTION CONTRACT
 * ============================================================================
 *
 * The following are syntactically equivalent in responsibility:
 *
 *     User
 *     module::User
 *     package::module::User
 *
 * The grammar records the spelling and order only.
 *
 * Semantic analysis determines whether the path resolves to:
 *
 *     - a local type;
 *     - an imported type;
 *     - a module type;
 *     - a package type;
 *     - a generic parameter;
 *     - an associated type;
 *     - a type alias;
 *     - a domain type;
 *     - a capability/resource abstraction;
 *     - another valid type declaration.
 *
 * An unresolved path is a semantic diagnostic, not a parser error.
 *
 * ============================================================================
 * GENERIC PARAMETER CONTRACT
 * ============================================================================
 *
 * A generic parameter such as:
 *
 *     T
 *
 * is syntactically a named type.
 *
 * Whether `T` denotes a generic type parameter is determined by semantic
 * scope resolution.
 *
 * This grammar must NOT contain a special:
 *
 *     genericParameterType
 *
 * alternative.
 *
 * Doing so would duplicate semantic scope information inside the parser.
 *
 * ============================================================================
 * DOMAIN NEUTRALITY
 * ============================================================================
 *
 * Named types may refer to any language-level domain, including:
 *
 *     classical
 *     quantum
 *     hybrid
 *     HDL
 *     hardware
 *     accelerator
 *     AI/ML
 *     data
 *     networking
 *     security
 *     distributed computing
 *     embedded computing
 *     scientific computing
 *     future computational domains
 *
 * Examples:
 *
 *     classical::Matrix
 *     quantum::Qubit
 *     quantum::LogicalQubit
 *     hardware::Memory
 *     hdl::Signal
 *     ai::Tensor
 *     distributed::Node
 *
 * The parser does not know which domain a name belongs to.
 *
 * ============================================================================
 * QUANTUM CONTRACT
 * ============================================================================
 *
 * A quantum name remains a source abstraction:
 *
 *     Qubit
 *     quantum::Qubit
 *     quantum::LogicalQubit
 *     quantum::State
 *
 * This file MUST NOT recognize:
 *
 *     q0
 *     physical_qubit_17
 *     device_7
 *
 * as special quantum types.
 *
 * Ordinary identifiers remain ordinary identifiers.
 *
 * Physical identity, topology, calibration, routing, scheduling and device
 * realization belong downstream.
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Named types must scale independently of machine size.
 *
 * This grammar MUST NOT establish limits such as:
 *
 *     MAX_TYPE_PATH_DEPTH
 *     MAX_IDENTIFIER_LENGTH
 *     MAX_NAMESPACE_DEPTH
 *     MAX_MODULE_DEPTH
 *     MAX_PACKAGE_DEPTH
 *     MAX_TYPE_NAME_COUNT
 *     MAX_GENERIC_ARITY
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_DEVICES
 *
 * A source program may therefore express:
 *
 *     A
 *
 * or:
 *
 *     a::b::c::d::e::Type
 *
 * without the language grammar imposing a finite semantic ceiling.
 *
 * Any parser/compiler resource protection must be implemented as an explicit,
 * configurable compiler validation policy outside this grammar.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This is a PARSER grammar.
 *
 * It MUST NOT define lexer rules.
 *
 * The canonical lexer owns:
 *
 *     IDENT
 *     DOUBLE_COLON
 *
 * and all other lexical tokens.
 *
 * This file assumes the canonical lexical vocabulary used by the existing
 * modular ANTLR parser grammars:
 *
 *     tokenVocab = ZamaniLexer
 *
 * No grammar-local identifier token is created.
 *
 * ============================================================================
 * SOURCE PRESERVATION
 * ============================================================================
 *
 * A path:
 *
 *     foo::bar::Baz
 *
 * must preserve the exact logical segment order:
 *
 *     foo
 *     bar
 *     Baz
 *
 * The parser must not:
 *
 *     - reverse segments;
 *     - normalize semantic identity;
 *     - resolve aliases;
 *     - collapse qualification;
 *     - attach module IDs;
 *     - attach symbol IDs.
 *
 * ============================================================================
 * DIAGNOSTICS CONTRACT
 * ============================================================================
 *
 * The grammar is responsible only for syntactic diagnostics.
 *
 * Examples of parser errors:
 *
 *     ::Type
 *     foo::
 *     foo::::Type
 *     foo:::
 *
 * Examples of semantic errors:
 *
 *     unknown::Type
 *     imported::MissingType
 *     private::Type
 *
 * The parser must not attempt to diagnose semantic lookup failures.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * `typePath` is a deterministic ordered sequence.
 *
 * The grammar contains:
 *
 *     no unordered alternatives;
 *     no semantic predicates;
 *     no target-dependent branches;
 *     no hardware-dependent branches;
 *     no runtime actions.
 *
 * The same token sequence therefore produces the same parse structure.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * This grammar:
 *
 *     - performs no I/O;
 *     - executes no user code;
 *     - performs no hardware discovery;
 *     - performs no network access;
 *     - performs no filesystem access;
 *     - contains no target-specific actions.
 *
 * Generated parser/runtime safety remains the responsibility of the parser
 * implementation and compiler policy.
 *
 * Rust implementation requirements:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Edition 2021
 *     stable Rust
 *     no unsafe
 *
 * ============================================================================
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive syntax cases:
 *
 *     T
 *     User
 *     Foo::Bar
 *     std::String
 *     std::collections::Map
 *     quantum::Qubit
 *     quantum::logical::Qubit
 *     hardware::memory::Buffer
 *     ai::tensor::Tensor
 *
 * Boundary/scalability cases:
 *
 *     one-segment path
 *     many-segment path
 *     very long identifiers
 *     deeply qualified source names
 *
 * Negative syntax cases:
 *
 *     ::Type
 *     Type::
 *     Type:::Nested
 *     Type::::Nested
 *     ::
 *
 * Integration cases:
 *
 *     named type as a parameter type
 *     named type as a return type
 *     named type as a field type
 *     named type as a generic bound
 *     named type as an implementation target
 *     named type as a type-alias target
 *
 * Generic integration cases belong to generic-types.g4:
 *
 *     Vec<T>
 *     module::Vec<T>
 *     Result<module::Value, module::Error>
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete when:
 *
 *   [x] namedType is the sole named-type entry rule;
 *   [x] typePath is owned here;
 *   [x] typePathSegment is owned here;
 *   [x] generic application is not duplicated here;
 *   [x] identifier lexing is delegated to the lexer;
 *   [x] qualification uses the canonical DOUBLE_COLON token;
 *   [x] no namespace-depth limit exists;
 *   [x] no machine-size limit exists;
 *   [x] no semantic resolution occurs;
 *   [x] no hardware assumptions occur;
 *   [x] no quantum hardware assumptions occur;
 *   [x] AST mapping is TypeExpr::Identifier(TypePath);
 *   [x] semantic resolution remains downstream;
 *   [x] deterministic parsing is preserved;
 *   [x] negative syntax cases are defined;
 *   [x] scalability cases are defined;
 *   [x] integration points are explicit.
 *
 * ============================================================================
 */

parser grammar NamedTypes;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 1. PUBLIC ENTRY POINT
 * ========================================================================== */

/**
 * Canonical source-level named type.
 *
 * This rule represents an unresolved source-level type name.
 *
 * Examples:
 *
 *     T
 *     User
 *     std::String
 *     std::collections::Map
 *     quantum::Qubit
 */
namedType
    : typePath
    ;


/* ============================================================================
 * 2. TYPE PATH
 * ========================================================================== */

/**
 * Canonical unresolved type path.
 *
 * A type path consists of one or more source-level name segments separated
 * by the canonical `::` qualification operator.
 *
 * Examples:
 *
 *     T
 *     User
 *     module::User
 *     package::module::User
 *     quantum::logical::Qubit
 *
 * There is intentionally no finite path-depth limit.
 */
typePath
    : typePathSegment
      (DOUBLE_COLON typePathSegment)*
    ;


/* ============================================================================
 * 3. TYPE PATH SEGMENT
 * ========================================================================== */

/**
 * One lexical type-name component.
 *
 * Identifier spelling and Unicode rules belong to the canonical lexer.
 *
 * Semantic interpretation belongs to name/type resolution.
 */
typePathSegment
    : IDENT
    ;
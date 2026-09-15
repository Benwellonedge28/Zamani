/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/interoperability/python.g4
 *
 * Grammar:
 *     PythonInterop
 *
 * Status:
 *     Production-ready Python interoperability boundary grammar.
 *
 * Language implementation baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     unsafe Rust forbidden
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar defines Zamani SOURCE-LEVEL PYTHON INTEROPERABILITY.
 *
 * It does NOT define the Python programming language.
 *
 * It defines how Zamani source may DECLARE a semantic boundary to Python
 * implementations, modules, callables, objects, buffers, asynchronous
 * interfaces, exceptions, iterators, generators, and other Python-facing
 * capabilities.
 *
 * The grammar is intentionally independent of:
 *
 *     CPython
 *     PyPy
 *     GraalPy
 *     MicroPython
 *     a particular Python version
 *     a particular operating system
 *     a particular CPU
 *     a particular GPU
 *     a particular accelerator
 *     a particular machine
 *     a particular Python installation
 *     a particular virtual environment
 *     a particular filesystem
 *     a particular process
 *     a particular memory address
 *     a particular pointer width
 *     a particular ABI
 *     a particular deployment topology
 *
 * Those are semantic, compilation, deployment, runtime, or target concerns.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *                         Zamani source
 *                              |
 *                              v
 *                         Zamani lexer
 *                              |
 *                              v
 *                       parser composition
 *                              |
 *                              v
 *                  Python interoperability
 *                              |
 *                              v
 *                     frontend AST
 *                              |
 *                              v
 *                     semantic analysis
 *                              |
 *              +---------------+----------------+
 *              |               |                |
 *              v               v                v
 *            types         capabilities       effects
 *              |               |                |
 *              +---------------+----------------+
 *                              |
 *                              v
 *                  canonical semantic model
 *                              |
 *             +----------------+----------------+
 *             |                |                |
 *             v                v                v
 *       classical IR      quantum::ir      hardware/HDL IR
 *             |                |                |
 *             +----------------+----------------+
 *                              |
 *                              v
 *                    optimization / lowering
 *                              |
 *                              v
 *                       Python adapter
 *                              |
 *                              v
 *                     runtime / deployment
 *
 * Python interoperability MUST NOT bypass the canonical semantic boundary.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - Python interoperability declarations;
 *   - Python interface declarations;
 *   - Python module references;
 *   - Python callable references;
 *   - Python object boundary declarations;
 *   - Python conversion contracts;
 *   - Python ownership metadata;
 *   - Python borrowing metadata;
 *   - Python lifetime metadata;
 *   - Python nullability metadata;
 *   - Python buffer/data-transfer metadata;
 *   - Python iterator/generator contracts;
 *   - Python asynchronous contracts;
 *   - Python exception boundary declarations;
 *   - Python execution/capability requirements;
 *   - Python implementation compatibility metadata;
 *   - Python environment requirements as symbolic constraints;
 *   - Python interoperability policies;
 *   - Python interoperability attributes;
 *   - Python interoperability calls;
 *   - Python callable references;
 *   - Python callback declarations;
 *   - Python boundary intent.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - Python language syntax;
 *   - Python AST;
 *   - Python bytecode;
 *   - Python interpreter implementation;
 *   - Python runtime implementation;
 *   - CPython internals;
 *   - PyPy internals;
 *   - Python package installation;
 *   - filesystem access;
 *   - process creation;
 *   - interpreter discovery;
 *   - library loading;
 *   - symbol resolution;
 *   - native pointer manipulation;
 *   - raw memory access;
 *   - ABI implementation;
 *   - calling-convention implementation;
 *   - general FFI declarations;
 *   - general ABI declarations;
 *   - ordinary function declarations;
 *   - ordinary types;
 *   - ordinary expressions;
 *   - modules as a general language feature;
 *   - capabilities as a general language feature;
 *   - resources as a general language feature;
 *   - scheduling;
 *   - routing;
 *   - optimization;
 *   - hardware discovery;
 *   - calibration;
 *   - QEC;
 *   - ZQN;
 *   - resilience;
 *   - quantum IR;
 *   - classical IR;
 *   - HDL IR;
 *   - runtime dispatch.
 *
 * General FFI ownership remains with:
 *
 *     grammar/interoperability/ffi.g4
 *
 * General foreign-function declarations remain with:
 *
 *     grammar/interoperability/foreign-functions.g4
 *
 * ABI contracts remain with:
 *
 *     grammar/interoperability/abi.g4
 *
 * Interoperability composition remains with:
 *
 *     grammar/interoperability/interoperability.g4
 *
 * Ordinary function syntax remains with:
 *
 *     grammar/functions/
 *
 * Ordinary types remain with:
 *
 *     grammar/types/
 *
 * Ordinary expressions remain with:
 *
 *     grammar/expressions/
 *
 * Canonical names remain with:
 *
 *     grammar/core/
 *
 * ============================================================================
 * POCO-REAF
 * ============================================================================
 *
 * Python interoperability MUST preserve:
 *
 *     Program Once
 *     Compile Once
 *     Run Everywhere
 *     Run Anywhere
 *     Run Forever
 *
 * Therefore this grammar MUST NOT encode:
 *
 *     a fixed Python implementation;
 *     a fixed Python version;
 *     a fixed machine;
 *     a fixed interpreter path;
 *     a fixed library path;
 *     a fixed CPU;
 *     a fixed GPU;
 *     a fixed device;
 *     a fixed node;
 *     a fixed memory size;
 *     a fixed pointer width;
 *     a fixed address;
 *     a fixed deployment location.
 *
 * A source-level Python requirement is a semantic requirement.
 *
 * Example:
 *
 *     python;
 *
 * means that the semantic operation requires Python interoperability.
 *
 * It does NOT mean:
 *
 *     CPython on Linux
 *
 * or:
 *
 *     Python at /usr/bin/python
 *
 * or:
 *
 *     Python version X
 *
 * or:
 *
 *     a particular device.
 *
 * ============================================================================
 * OPEN-WORLD IMPLEMENTATION MODEL
 * ============================================================================
 *
 * Python implementations are symbolic.
 *
 * Examples that MAY be expressed downstream include:
 *
 *     CPython
 *     PyPy
 *     GraalPy
 *     MicroPython
 *     custom implementation
 *
 * This grammar does not enumerate them.
 *
 * The same rule supports future implementations without changing the grammar.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * This grammar imposes no language-level finite limit on:
 *
 *     interfaces
 *     modules
 *     callables
 *     callbacks
 *     parameters
 *     arguments
 *     conversions
 *     attributes
 *     requirements
 *     capabilities
 *     effects
 *     objects
 *     streams
 *     buffers
 *     iterators
 *     generators
 *     declarations
 *     program size
 *
 * Repetition is structural through ANTLR '*' and '+' operators.
 *
 * There is intentionally no:
 *
 *     MAX_PYTHON_MODULES
 *     MAX_PYTHON_CALLABLES
 *     MAX_ARGUMENTS
 *     MAX_OBJECTS
 *     MAX_BUFFERS
 *     MAX_ITERATORS
 *     MAX_GENERATORS
 *     MAX_THREADS
 *     MAX_DEVICES
 *     MAX_CORES
 *     MAX_QUBITS
 *
 * or equivalent source-level limit.
 *
 * Practical resource limits belong to compiler/resource/runtime policy.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * Parsing this grammar MUST NEVER:
 *
 *     load Python;
 *     start an interpreter;
 *     import a Python module;
 *     execute Python;
 *     evaluate Python expressions;
 *     execute Python bytecode;
 *     access the filesystem;
 *     access the network;
 *     inspect installed packages;
 *     inspect environment variables;
 *     resolve native symbols;
 *     dereference pointers;
 *     allocate native memory;
 *     create interpreter state;
 *     invoke callbacks.
 *
 * This grammar is declarative and inert.
 *
 * A declaration is not an authorization.
 *
 * Security and runtime policy must independently authorize execution.
 *
 * ============================================================================
 * RUST SAFETY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no Rust actions;
 *     no semantic predicates;
 *     no embedded code;
 *     no unsafe blocks;
 *     no runtime calls.
 *
 * Downstream Rust implementation MUST remain compatible with:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Edition 2021
 *
 * and MUST NOT require unsafe Rust.
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * The canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Therefore this grammar uses:
 *
 *     tokenVocab = ZamaniLexer
 *
 * Shared syntax is imported rather than redefined.
 *
 * Expected canonical imported rules:
 *
 *     expression
 *     expressionList
 *     typeExpression
 *     qualifiedNameReference
 *
 * If the repository later renames any of these canonical rules, the composition
 * layer MUST adapt the names. This file must not introduce duplicate
 * definitions merely to compensate.
 *
 * ============================================================================
 */

parser grammar PythonInterop;

options {
    tokenVocab = ZamaniLexer;
}

import Expressions,
       Types,
       QualifiedNames;


/* ============================================================================
 * PUBLIC ENTRY POINT
 * ========================================================================== */

/*
 * A reusable Python interoperability item.
 *
 * The canonical composition grammar decides whether an item is legal at:
 *
 *     declaration level
 *     expression level
 *     statement level
 *     module level
 *
 * This grammar does not make itself the global compilation root.
 */
pythonInterop
    : pythonInteropItem*
    ;


pythonInteropItem
    : pythonInterfaceDeclaration
    | pythonModuleDeclaration
    | pythonCallableDeclaration
    | pythonObjectDeclaration
    | pythonCallbackDeclaration
    | pythonConversionDeclaration
    | pythonBufferDeclaration
    | pythonAsyncDeclaration
    | pythonIteratorDeclaration
    | pythonGeneratorDeclaration
    | pythonExceptionDeclaration
    | pythonRequirementDeclaration
    | pythonPolicyDeclaration
    | pythonLinkDeclaration
    ;


/* ============================================================================
 * PYTHON INTERFACE
 * ========================================================================== */

/*
 * Defines a reusable symbolic Python interoperability boundary.
 *
 * Example:
 *
 *     python interface numerical {
 *         ...
 *     }
 *
 * The identifier is semantic identity only.
 */
pythonInterfaceDeclaration
    : attribute*
      'python'
      'interface'
      qualifiedNameReference
      pythonInterfaceParameterClause?
      '{'
      pythonInterfaceMember*
      '}'
    ;


pythonInterfaceParameterClause
    : '<'
      pythonInterfaceParameter
      (',' pythonInterfaceParameter)*
      '>'
    ;


pythonInterfaceParameter
    : identifier
      (':' qualifiedNameReference)?
    ;


pythonInterfaceMember
    : attribute*
      (
          pythonModuleDeclaration
        | pythonCallableDeclaration
        | pythonObjectDeclaration
        | pythonCallbackDeclaration
        | pythonConversionDeclaration
        | pythonBufferDeclaration
        | pythonAsyncDeclaration
        | pythonIteratorDeclaration
        | pythonGeneratorDeclaration
        | pythonExceptionDeclaration
        | pythonRequirementDeclaration
        | pythonPolicyDeclaration
        | pythonLinkDeclaration
      )
    ;


/* ============================================================================
 * MODULES
 * ========================================================================== */

/*
 * A Python module is a symbolic namespace.
 *
 * The grammar deliberately does not interpret the module reference as:
 *
 *     filesystem path
 *     wheel
 *     package directory
 *     URL
 *     installed package
 *
 * Those interpretations belong to later resolution policy.
 */
pythonModuleDeclaration
    : attribute*
      'python'
      'module'
      qualifiedNameReference
      pythonModuleClause*
      ';'
    ;


pythonModuleClause
    : pythonImplementationClause
    | pythonVersionClause
    | pythonCompatibilityClause
    | pythonRequirementClause
    | pythonAttributeBlock
    ;


/*
 * Implementation identity is symbolic.
 *
 * Examples:
 *
 *     implementation = CPython
 *     implementation = PyPy
 *
 * are semantic metadata rather than hard-coded grammar categories.
 */
pythonImplementationClause
    : 'implementation'
      '='
      pythonSymbolicValue
    ;


/*
 * Version is represented as a value rather than a grammar-level enumeration.
 *
 * This permits future Python versions without grammar changes.
 */
pythonVersionClause
    : 'version'
      pythonVersionOperator?
      pythonVersionValue
    ;


pythonVersionOperator
    : '='
    | '=='
    | '!='
    | '<'
    | '<='
    | '>'
    | '>='
    | '^'
    | '~'
    ;


pythonVersionValue
    : stringLiteral
    | integerLiteral
    | pythonVersionIdentifier
    ;


pythonVersionIdentifier
    : identifier
    | qualifiedNameReference
    ;


/* ============================================================================
 * CALLABLE DECLARATIONS
 * ========================================================================== */

/*
 * Declares a Python callable contract.
 *
 * This is NOT ordinary Zamani function syntax.
 *
 * It describes an external callable boundary.
 */
pythonCallableDeclaration
    : attribute*
      'python'
      'fn'
      qualifiedNameReference
      '('
      pythonParameterList?
      ')'
      pythonReturnClause?
      pythonCallableClause*
      ';'
    ;


pythonParameterList
    : pythonParameter
      (',' pythonParameter)*
    ;


pythonParameter
    : parameter
      pythonParameterClause*
    ;


pythonParameterClause
    : pythonConversionClause
    | pythonOwnershipClause
    | pythonBorrowClause
    | pythonNullabilityClause
    | pythonBufferClause
    | pythonOptionalClause
    | pythonKeywordOnlyClause
    | pythonPositionalOnlyClause
    | pythonVariadicClause
    | pythonAttributeBlock
    ;


pythonReturnClause
    : '->'
      typeExpression
      pythonReturnClauseItem*
    ;


pythonReturnClauseItem
    : pythonConversionClause
    | pythonOwnershipClause
    | pythonNullabilityClause
    | pythonBufferClause
    | pythonAttributeBlock
    ;


pythonCallableClause
    : pythonImplementationClause
    | pythonModuleReferenceClause
    | pythonSymbolClause
    | pythonConversionClause
    | pythonOwnershipClause
    | pythonBorrowClause
    | pythonLifetimeClause
    | pythonNullabilityClause
    | pythonBufferClause
    | pythonExceptionClause
    | pythonEffectClause
    | pythonRequirementClause
    | pythonCompatibilityClause
    | pythonAsyncClause
    | pythonStreamingClause
    | pythonThreadingClause
    | pythonGILClause
    | pythonEnvironmentClause
    | pythonAttributeBlock
    ;


/* ============================================================================
 * SYMBOLS
 * ========================================================================== */

/*
 * A Python symbol remains symbolic.
 *
 * The grammar does not resolve it.
 */
pythonSymbolClause
    : 'symbol'
      '='
      pythonSymbolicValue
    ;


pythonModuleReferenceClause
    : 'module'
      '='
      qualifiedNameReference
    ;


/* ============================================================================
 * OBJECT BOUNDARIES
 * ========================================================================== */

/*
 * Describes a Python object that may cross the Zamani/Python boundary.
 *
 * This does not create a runtime Python object.
 */
pythonObjectDeclaration
    : attribute*
      'python'
      'object'
      qualifiedNameReference
      pythonObjectClause*
      ';'
    ;


pythonObjectClause
    : pythonTypeClause
    | pythonRepresentationClause
    | pythonOwnershipClause
    | pythonBorrowClause
    | pythonLifetimeClause
    | pythonNullabilityClause
    | pythonConversionClause
    | pythonBufferClause
    | pythonRequirementClause
    | pythonCompatibilityClause
    | pythonAttributeBlock
    ;


pythonTypeClause
    : 'type'
      '='
      typeExpression
    ;


pythonRepresentationClause
    : 'representation'
      '='
      pythonSymbolicValue
    ;


/* ============================================================================
 * CONVERSION / MARSHALLING
 * ========================================================================== */

/*
 * Conversion is semantic metadata.
 *
 * It does not prescribe a particular native representation.
 */
pythonConversionDeclaration
    : attribute*
      'python'
      'conversion'
      pythonConversionSpec
      pythonConversionClause*
      ';'
    ;


pythonConversionClause
    : 'convert'
      pythonConversionSpec
    ;


pythonConversionSpec
    : pythonSymbolicValue
    | '('
      expression
      ')'
    ;


pythonMarshalClause
    : 'marshal'
      pythonConversionSpec
      (
          'as'
          pythonConversionSpec
      )?
    ;


/* ============================================================================
 * OWNERSHIP / BORROWING / LIFETIMES
 * ========================================================================== */

pythonOwnershipClause
    : 'ownership'
      '='
      pythonOwnershipMode
    ;


pythonOwnershipMode
    : 'owned'
    | 'borrowed'
    | 'shared'
    | 'transferred'
    | 'returned'
    | 'in'
    | 'out'
    | 'inout'
    | pythonSymbolicValue
    ;


pythonBorrowClause
    : 'borrow'
      '='
      pythonBorrowMode
    ;


pythonBorrowMode
    : 'shared'
    | 'exclusive'
    | 'read'
    | 'write'
    | pythonSymbolicValue
    | '('
      expression
      ')'
    ;


pythonLifetimeClause
    : 'lifetime'
      '='
      pythonLifetimeSpec
    ;


pythonLifetimeSpec
    : qualifiedNameReference
    | stringLiteral
    | '('
      expression
      ')'
    ;


pythonNullabilityClause
    : 'nullability'
      '='
      pythonNullability
    ;


pythonNullability
    : 'nullable'
    | 'nonnull'
    | 'unknown'
    | pythonSymbolicValue
    ;


/* ============================================================================
 * BUFFER / ARRAY / ZERO-COPY BOUNDARIES
 * ========================================================================== */

/*
 * Python interoperability frequently crosses through buffer-oriented data.
 *
 * The grammar describes the semantic contract but never assumes:
 *
 *     a particular pointer width
 *     a particular stride
 *     a particular machine alignment
 *     a particular address
 *     a particular contiguous memory size
 */
pythonBufferDeclaration
    : attribute*
      'python'
      'buffer'
      qualifiedNameReference
      pythonBufferClause*
      ';'
    ;


pythonBufferClause
    : 'buffer'
      '='
      pythonBufferMode
    | 'layout'
      '='
      pythonSymbolicValue
    | 'shape'
      '='
      expression
    | 'strides'
      '='
      expression
    | 'format'
      '='
      pythonSymbolicValue
    | 'contiguous'
      '='
      pythonBooleanValue
    | 'writable'
      '='
      pythonBooleanValue
    | 'zero'
      'copy'
      '='
      pythonBooleanValue
    | 'owner'
      '='
      pythonSymbolicValue
    | 'lifetime'
      '='
      pythonLifetimeSpec
    ;


pythonBufferMode
    : 'read'
    | 'write'
    | 'readwrite'
    | 'borrowed'
    | 'owned'
    | pythonSymbolicValue
    ;


/* ============================================================================
 * OPTIONAL / KEYWORD / POSITIONAL / VARIADIC PARAMETERS
 * ========================================================================== */

pythonOptionalClause
    : 'optional'
      (
          '='
          expression
      )?
    ;


pythonKeywordOnlyClause
    : 'keyword'
      'only'
    ;


pythonPositionalOnlyClause
    : 'positional'
      'only'
    ;


pythonVariadicClause
    : 'variadic'
      pythonVariadicMode?
    ;


pythonVariadicMode
    : 'positional'
    | 'keyword'
    | 'both'
    | pythonSymbolicValue
    ;


/* ============================================================================
 * ASYNCHRONOUS PYTHON
 * ========================================================================== */

/*
 * Python async semantics are represented as an interoperability contract.
 *
 * The grammar does not execute an event loop.
 */
pythonAsyncDeclaration
    : attribute*
      'python'
      'async'
      qualifiedNameReference
      pythonAsyncClause*
      ';'
    ;


pythonAsyncClause
    : 'async'
      (
          '='
          pythonBooleanValue
      )?
    | 'awaitable'
      (
          '='
          pythonBooleanValue
      )?
    | 'scheduler'
      '='
      pythonSymbolicValue
    | 'context'
      '='
      pythonSymbolicValue
    | 'cancellation'
      '='
      pythonSymbolicValue
    | 'ordering'
      '='
      pythonSymbolicValue
    ;


/* ============================================================================
 * ITERATORS
 * ========================================================================== */

pythonIteratorDeclaration
    : attribute*
      'python'
      'iterator'
      qualifiedNameReference
      pythonIteratorClause*
      ';'
    ;


pythonIteratorClause
    : 'element'
      '='
      typeExpression
    | 'ownership'
      '='
      pythonOwnershipMode
    | 'termination'
      '='
      pythonSymbolicValue
    | 'lazy'
      '='
      pythonBooleanValue
    | 'reusable'
      '='
      pythonBooleanValue
    | 'async'
      '='
      pythonBooleanValue
    | pythonExceptionClause
    | pythonRequirementClause
    ;


/* ============================================================================
 * GENERATORS
 * ========================================================================== */

pythonGeneratorDeclaration
    : attribute*
      'python'
      'generator'
      qualifiedNameReference
      pythonGeneratorClause*
      ';'
    ;


pythonGeneratorClause
    : 'yield'
      '='
      typeExpression
    | 'send'
      '='
      typeExpression
    | 'receive'
      '='
      typeExpression
    | 'return'
      '='
      typeExpression
    | 'async'
      '='
      pythonBooleanValue
    | 'lazy'
      '='
      pythonBooleanValue
    | pythonExceptionClause
    | pythonRequirementClause
    ;


/* ============================================================================
 * CALLBACKS
 * ========================================================================== */

/*
 * A callback is a semantic callable boundary.
 *
 * The callback is not installed or invoked during parsing.
 */
pythonCallbackDeclaration
    : attribute*
      'python'
      'callback'
      qualifiedNameReference
      '('
      pythonParameterList?
      ')'
      pythonReturnClause?
      pythonCallbackClause*
      ';'
    ;


pythonCallbackClause
    : pythonOwnershipClause
    | pythonLifetimeClause
    | pythonThreadingClause
    | pythonGILClause
    | pythonExceptionClause
    | pythonAsyncClause
    | pythonRequirementClause
    | pythonCompatibilityClause
    | pythonAttributeBlock
    ;


/* ============================================================================
 * EXCEPTION BOUNDARIES
 * ========================================================================== */

/*
 * Foreign exceptions MUST NOT silently disappear.
 *
 * The semantic layer determines the actual mapping to a Zamani error/result
 * representation.
 */
pythonExceptionDeclaration
    : attribute*
      'python'
      'exception'
      qualifiedNameReference
      pythonExceptionItem*
      ';'
    ;


pythonExceptionClause
    : 'errors'
      '{'
      pythonExceptionItem*
      '}'
    ;


pythonExceptionItem
    : 'mode'
      '='
      pythonExceptionMode
      ';'?
    | 'map'
      qualifiedNameReference
      'to'
      typeExpression
      ';'?
    | 'type'
      '='
      typeExpression
      ';'?
    | 'catch'
      pythonSymbolicValue
      ';'?
    | 'propagate'
      pythonBooleanValue
      ';'?
    | 'attribute'
      identifier
      '='
      expression
      ';'?
    ;


pythonExceptionMode
    : 'result'
    | 'exception'
    | 'status'
    | 'panic'
    | 'abort'
    | 'unknown'
    | pythonSymbolicValue
    ;


/* ============================================================================
 * EFFECTS
 * ========================================================================== */

/*
 * Effects are symbolic references.
 *
 * The canonical effects grammar owns the effect vocabulary.
 */
pythonEffectClause
    : 'with'
      'effects'
      '{'
      pythonEffectReferenceList?
      '}'
    ;


pythonEffectReferenceList
    : qualifiedNameReference
      (',' qualifiedNameReference)*
    ;


/* ============================================================================
 * REQUIREMENTS / CAPABILITIES
 * ========================================================================== */

/*
 * Requirements are semantic prerequisites, not target selections.
 */
pythonRequirementDeclaration
    : attribute*
      'python'
      'requires'
      '{'
      pythonRequirement*
      '}'
    ;


pythonRequirementClause
    : 'requires'
      '{'
      pythonRequirement*
      '}'
    ;


pythonRequirement
    : qualifiedNameReference
      (
          '='
          expression
      )?
    ;


/* ============================================================================
 * COMPATIBILITY
 * ========================================================================== */

pythonCompatibilityClause
    : 'compatible'
      'with'
      pythonCompatibilityRequirement
    ;


pythonCompatibilityRequirement
    : qualifiedNameReference
    | stringLiteral
    | expression
    ;


/* ============================================================================
 * THREADING
 * ========================================================================== */

/*
 * Threading policy is semantic metadata.
 *
 * It does not create threads and does not encode a fixed thread count.
 */
pythonThreadingClause
    : 'threading'
      '='
      pythonThreadingMode
    ;


pythonThreadingMode
    : 'single'
    | 'multi'
    | 'concurrent'
    | 'serialized'
    | 'free'
    | 'unknown'
    | pythonSymbolicValue
    ;


/* ============================================================================
 * GIL / INTERPRETER-SERIALIZATION CONTRACT
 * ========================================================================== */

/*
 * "GIL" is represented as a semantic boundary policy rather than an assumption
 * that every future Python implementation has the same execution model.
 *
 * This allows implementations with or without a global interpreter lock.
 */
pythonGILClause
    : 'interpreter'
      'serialization'
      '='
      pythonInterpreterSerializationMode
    ;


pythonInterpreterSerializationMode
    : 'required'
    | 'preferred'
    | 'forbidden'
    | 'unknown'
    | pythonSymbolicValue
    ;


/* ============================================================================
 * STREAMING
 * ========================================================================== */

pythonStreamingClause
    : 'streaming'
      pythonStreamingMode?
    ;


pythonStreamingMode
    : 'input'
    | 'output'
    | 'bidirectional'
    | 'lazy'
    | 'eager'
    | pythonSymbolicValue
    ;


/* ============================================================================
 * ENVIRONMENT
 * ========================================================================== */

/*
 * Environment metadata remains declarative.
 *
 * It does not read the environment.
 */
pythonEnvironmentClause
    : 'environment'
      '{'
      pythonEnvironmentItem*
      '}'
    ;


pythonEnvironmentItem
    : 'name'
      '='
      pythonSymbolicValue
      ';'?
    | 'version'
      '='
      pythonVersionValue
      ';'?
    | 'package'
      '='
      pythonSymbolicValue
      ';'?
    | 'dependency'
      '='
      pythonSymbolicValue
      ';'?
    | 'constraint'
      '='
      expression
      ';'?
    | 'attribute'
      identifier
      '='
      expression
      ';'?
    ;


/* ============================================================================
 * POLICY
 * ========================================================================== */

/*
 * Policies describe source-level interoperability intent.
 *
 * They do not themselves grant permissions.
 */
pythonPolicyDeclaration
    : attribute*
      'python'
      'policy'
      qualifiedNameReference
      '{'
      pythonPolicyItem*
      '}'
    ;


pythonPolicyItem
    : 'allow'
      pythonPolicySubject
      ';'?
    | 'deny'
      pythonPolicySubject
      ';'?
    | 'require'
      pythonPolicySubject
      ';'?
    | 'prefer'
      pythonPolicySubject
      ';'?
    | 'forbid'
      pythonPolicySubject
      ';'?
    | 'attribute'
      identifier
      '='
      expression
      ';'?
    ;


pythonPolicySubject
    : qualifiedNameReference
    | pythonSymbolicValue
    | expression
    ;


/* ============================================================================
 * LINK / IMPLEMENTATION ASSOCIATION
 * ========================================================================== */

/*
 * A link declaration associates a semantic Python boundary with a symbolic
 * implementation identity.
 *
 * It does NOT load anything.
 */
pythonLinkDeclaration
    : attribute*
      'python'
      'link'
      qualifiedNameReference
      '{'
      pythonLinkItem*
      '}'
    ;


pythonLinkItem
    : 'module'
      '='
      qualifiedNameReference
      ';'?
    | 'symbol'
      '='
      pythonSymbolicValue
      ';'?
    | 'implementation'
      '='
      pythonSymbolicValue
      ';'?
    | 'version'
      '='
      pythonVersionValue
      ';'?
    | 'compatibility'
      '='
      expression
      ';'?
    | 'requires'
      '='
      expression
      ';'?
    | 'attribute'
      identifier
      '='
      expression
      ';'?
    ;


/* ============================================================================
 * PYTHON CALL EXPRESSIONS
 * ========================================================================== */

/*
 * Explicit Python call.
 *
 * Example:
 *
 *     python call math.sin(x)
 *
 * The call represents intent only.
 *
 * It does not import or execute Python during parsing.
 */
pythonCallExpression
    : 'python'
      'call'
      qualifiedNameReference
      '('
      argumentList?
      ')'
    ;


/*
 * Explicit module-qualified Python call.
 *
 * Example:
 *
 *     python call "numpy"::linalg.solve(a, b)
 *
 * The string is an opaque source-level module identity.
 */
pythonQualifiedCallExpression
    : 'python'
      'call'
      stringLiteral
      '::'
      qualifiedNameReference
      '('
      argumentList?
      ')'
    ;


/*
 * Explicit Python callable reference.
 */
pythonCallableReferenceExpression
    : 'python'
      'ref'
      qualifiedNameReference
    ;


/*
 * Explicit Python object reference.
 */
pythonObjectReferenceExpression
    : 'python'
      'object'
      qualifiedNameReference
    ;


/* ============================================================================
 * STATEMENT FORMS
 * ========================================================================== */

pythonCallStatement
    : pythonCallExpression
      ';'
    | pythonQualifiedCallExpression
      ';'
    ;


/* ============================================================================
 * BOOLEAN / SYMBOLIC VALUES
 * ========================================================================== */

/*
 * Boolean values are represented through the canonical language literals where
 * available. These rules intentionally do not introduce a second boolean type.
 */
pythonBooleanValue
    : 'true'
    | 'false'
    | pythonSymbolicValue
    | '('
      expression
      ')'
    ;


/*
 * Open-world symbolic metadata.
 *
 * Python-specific vocabularies are intentionally not closed here.
 *
 * This prevents future Python implementations, package systems, execution
 * models, buffer formats, schedulers, or interoperability mechanisms from
 * requiring a grammar rewrite.
 */
pythonSymbolicValue
    : qualifiedNameReference
    | stringLiteral
    | integerLiteral
    | identifier
    ;


/* ============================================================================
 * ATTRIBUTE BLOCK
 * ========================================================================== */

pythonAttributeBlock
    : 'attributes'
      '{'
      pythonAttribute*
      '}'
    ;


pythonAttribute
    : identifier
      (
          '='
          expression
      )?
      ';'
    ;


/* ============================================================================
 * GENERIC PYTHON BOUNDARY CLAUSES
 * ========================================================================== */

/*
 * Allows future semantic properties to be attached without modifying this
 * grammar for every implementation-specific extension.
 *
 * Unknown attributes MUST still be validated semantically.
 *
 * The grammar's ability to parse an attribute does not imply that the compiler
 * accepts its meaning.
 */
pythonMetadataClause
    : identifier
      (
          '='
          expression
      )?
    ;


/* ============================================================================
 * SEMANTIC INTEGRATION CONTRACT
 * ========================================================================== */

/*
 * The frontend AST produced from this grammar MUST preserve:
 *
 *     source span
 *     declaration kind
 *     symbolic Python identity
 *     module identity
 *     callable identity
 *     parameter types
 *     return type
 *     conversion metadata
 *     ownership metadata
 *     borrowing metadata
 *     lifetime metadata
 *     nullability
 *     buffer contracts
 *     async contracts
 *     iterator/generator contracts
 *     callback contracts
 *     exception contracts
 *     effects
 *     requirements
 *     compatibility constraints
 *     environment requirements
 *     policy metadata
 *     attributes
 *
 * It MUST NOT materialize:
 *
 *     Python object handles
 *     interpreter handles
 *     pointers
 *     device handles
 *     addresses
 *     processes
 *     file descriptors
 *     native allocations
 *     runtime state
 */


/*
 * Semantic analysis MUST validate:
 *
 *     1. Python interface existence.
 *     2. Module identity validity.
 *     3. Callable identity validity.
 *     4. Type compatibility.
 *     5. Conversion legality.
 *     6. Ownership legality.
 *     7. Borrow/lifetime correctness.
 *     8. Buffer compatibility.
 *     9. Async compatibility.
 *    10. Iterator/generator compatibility.
 *    11. Callback safety.
 *    12. Exception/error mapping.
 *    13. Effect authorization.
 *    14. Capability satisfaction.
 *    15. Resource requirements.
 *    16. Security policy.
 *    17. Compatibility constraints.
 *    18. Target availability.
 *    19. Lowering availability.
 *
 * None of these decisions belong in the grammar.
 */


/* ============================================================================
 * IR INTEGRATION
 * ========================================================================== */

/*
 * Python interoperability syntax MUST lower through a semantic interoperability
 * representation.
 *
 * Conceptual flow:
 *
 *     PythonInterop AST
 *            |
 *            v
 *     semantic Python boundary
 *            |
 *            +--------------------+
 *            |                    |
 *            v                    v
 *      classical semantics   quantum semantics
 *            |                    |
 *            v                    v
 *      classical IR          quantum::ir
 *
 * A Python boundary may therefore participate in hybrid programs.
 *
 * Example semantic flow:
 *
 *     Python numerical computation
 *              |
 *              v
 *     classical semantic representation
 *
 *     Python quantum tooling
 *              |
 *              v
 *     quantum semantic representation
 *              |
 *              v
 *          quantum::ir
 *
 * This grammar MUST NEVER create:
 *
 *     QuantumGate
 *     QuantumInstruction
 *     QubitId
 *     PhysicalQubitId
 *     QuantumCircuit
 *
 * Those remain owned by the canonical quantum IR layer.
 */


/* ============================================================================
 * HARDWARE INTEGRATION
 * ========================================================================== */

/*
 * A Python boundary may eventually execute on:
 *
 *     CPU
 *     GPU
 *     accelerator
 *     quantum-classical host
 *     distributed node
 *     embedded runtime
 *     cloud environment
 *     future execution target
 *
 * This grammar does not choose one.
 *
 * Target selection belongs to:
 *
 *     compilation
 *     resource management
 *     hardware abstraction
 *     execution
 *     deployment
 */


/* ============================================================================
 * SECURITY INTEGRATION
 * ========================================================================== */

/*
 * Python interoperability MUST be treated as an explicit security boundary.
 *
 * The semantic/security layer must be able to distinguish:
 *
 *     declaration
 *     capability
 *     authorization
 *     execution
 *
 * A source declaration such as:
 *
 *     python call module.function(...)
 *
 * does NOT grant:
 *
 *     filesystem permission
 *     network permission
 *     process permission
 *     native-code permission
 *     package-installation permission
 *     environment inspection permission
 *
 * Those permissions must be independently represented and authorized.
 */


/* ============================================================================
 * ERROR INTEGRATION
 * ========================================================================== */

/*
 * The parser reports syntactic failures.
 *
 * Semantic analysis reports:
 *
 *     unknown module
 *     unknown callable
 *     invalid type conversion
 *     invalid ownership
 *     invalid lifetime
 *     incompatible buffer contract
 *     unsupported async contract
 *     unsupported callback contract
 *     unsatisfied capability
 *     unsatisfied resource requirement
 *     incompatible implementation
 *
 * Runtime reports:
 *
 *     unavailable interpreter
 *     unavailable implementation
 *     invocation failure
 *     foreign exception
 *     cancellation
 *     timeout
 *     resource exhaustion
 *
 * The grammar must never turn runtime failures into comments or silently ignore
 * them.
 */


/* ============================================================================
 * DETERMINISM
 * ========================================================================== */

/*
 * Parsing must be deterministic with respect to:
 *
 *     source
 *     lexer configuration
 *     grammar version
 *
 * No runtime environment may influence parsing.
 *
 * In particular, parsing must not depend on:
 *
 *     installed Python versions
 *     installed modules
 *     filesystem state
 *     network state
 *     hardware
 *     environment variables
 */


/* ============================================================================
 * COMPATIBILITY
 * ========================================================================== */

/*
 * Adding a new Python implementation MUST NOT require changing this grammar.
 *
 * Adding a new Python package MUST NOT require changing this grammar.
 *
 * Adding a new Python version MUST NOT require changing this grammar.
 *
 * Adding a new execution backend MUST NOT require changing this grammar.
 *
 * New semantics should normally be introduced through:
 *
 *     symbolic identifiers
 *     attributes
 *     capabilities
 *     requirements
 *     dialects
 *     versioned semantic rules
 *
 * rather than fixed keyword inventories.
 */


/* ============================================================================
 * SCALABILITY AUDIT
 * ========================================================================== */

/*
 * The grammar deliberately contains no:
 *
 *     finite module limit
 *     finite callable limit
 *     finite argument limit
 *     finite parameter limit
 *     finite object limit
 *     finite buffer limit
 *     finite iterator limit
 *     finite generator limit
 *     finite callback limit
 *     finite resource limit
 *     finite machine limit
 *
 * ANTLR repetition operators are used for unbounded language structures subject
 * only to actual compiler/runtime resource availability.
 */


/* ============================================================================
 * COMPLETION CONTRACT
 * ========================================================================== */

/*
 * This file is complete only when:
 *
 * [ ] It compiles as an ANTLR4 parser grammar.
 * [ ] It uses the canonical Zamani lexer.
 * [ ] It does not duplicate canonical identifier rules.
 * [ ] It does not duplicate canonical type rules.
 * [ ] It does not duplicate canonical expression rules.
 * [ ] It does not define Python language syntax.
 * [ ] It does not execute Python.
 * [ ] It does not perform filesystem access.
 * [ ] It does not perform network access.
 * [ ] It does not contain machine-size constants.
 * [ ] It does not contain device identifiers.
 * [ ] It does not contain fixed Python-version assumptions.
 * [ ] It does not contain fixed interpreter paths.
 * [ ] It does not define ABI implementation.
 * [ ] It does not define quantum IR.
 * [ ] It does not define QEC.
 * [ ] It does not define ZQN.
 * [ ] It does not define scheduling.
 * [ ] It does not define routing.
 * [ ] It does not define optimization.
 * [ ] It preserves source spans through the AST contract.
 * [ ] It supports symbolic/open-world Python implementations.
 * [ ] It supports synchronous calls.
 * [ ] It supports asynchronous calls.
 * [ ] It supports callbacks.
 * [ ] It supports iterators.
 * [ ] It supports generators.
 * [ ] It supports buffer-oriented interoperability.
 * [ ] It supports explicit ownership metadata.
 * [ ] It supports borrowing/lifetime metadata.
 * [ ] It supports nullability metadata.
 * [ ] It supports conversion contracts.
 * [ ] It supports exception mapping.
 * [ ] It supports effects.
 * [ ] It supports requirements.
 * [ ] It supports compatibility constraints.
 * [ ] It supports declarative environment requirements.
 * [ ] It supports semantic policies.
 * [ ] It supports hybrid classical/quantum use.
 * [ ] It remains independent of physical hardware.
 * [ ] It remains compatible with POCO-REAF.
 * [ ] Positive parser tests exist.
 * [ ] Negative parser tests exist.
 * [ ] Boundary tests exist.
 * [ ] Cross-domain tests exist.
 * [ ] Determinism tests exist.
 * [ ] Round-trip tests exist where the canonical printer supports them.
 */


/* ============================================================================
 * END OF PYTHON INTEROPERABILITY GRAMMAR
 * ========================================================================== */
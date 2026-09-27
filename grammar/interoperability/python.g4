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
 *     Production interoperability grammar.
 *
 * Baseline:
 *     Rust 1.97 / Rust 1.97.1
 *     Rust 2021
 *     Safe Rust only.
 *     No unsafe Rust.
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This grammar defines the Zamani SOURCE-LEVEL PYTHON INTEROPERABILITY
 * CONTRACT.
 *
 * It does NOT define the Python programming language.
 *
 * It does NOT parse Python source code, Python bytecode, Python ASTs, or
 * implementation-specific Python syntax.
 *
 * It defines the Zamani-side semantic boundary for interoperability with
 * Python implementations and Python-facing interfaces.
 *
 * Python implementation details are deliberately open-ended.
 *
 * Examples that may be resolved downstream include:
 *
 *     CPython
 *     PyPy
 *     GraalPy
 *     MicroPython
 *     another conforming implementation
 *
 * No implementation is enumerated by this grammar.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * Zamani source
 *      |
 *      v
 * canonical ZamaniLexer
 *      |
 *      v
 * ZamaniParser / Interoperability
 *      |
 *      v
 * Python interoperability contract
 *      |
 *      v
 * domain-neutral frontend AST
 *      |
 *      v
 * semantic analysis
 *      |
 *      +--------------------+--------------------+
 *      |                    |                    |
 *      v                    v                    v
 *    types              effects             capabilities
 *      |                    |                    |
 *      +--------------------+--------------------+
 *                           |
 *                           v
 *                 canonical semantic model
 *                           |
 *             +-------------+-------------+
 *             |             |             |
 *             v             v             v
 *        classical      quantum::ir   HDL/hardware
 *             |             |             |
 *             +-------------+-------------+
 *                           |
 *                           v
 *                     optimization
 *                           |
 *                     target lowering
 *                           |
 *                 ABI / runtime / adapter
 *                           |
 *                           v
 *                     target realization
 *
 * Python interoperability MUST NOT bypass the canonical semantic boundary.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - Python interoperability boundary declarations;
 *   - Python module identities;
 *   - Python callable identities;
 *   - Python object boundary declarations;
 *   - Python callback contracts;
 *   - Python conversion contracts;
 *   - Python buffer contracts;
 *   - Python iterator contracts;
 *   - Python generator contracts;
 *   - Python asynchronous contracts;
 *   - Python exception mapping intent;
 *   - Python ownership/lifetime metadata;
 *   - Python compatibility requirements;
 *   - Python implementation requirements;
 *   - Python environment requirements;
 *   - Python interoperability policies;
 *   - Python interoperability call/reference expressions;
 *   - Python-specific semantic metadata.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - Python language syntax;
 *   - Python AST;
 *   - Python bytecode;
 *   - Python interpreter implementation;
 *   - FFI generally;
 *   - ABI generally;
 *   - calling conventions;
 *   - ordinary Zamani functions;
 *   - ordinary Zamani types;
 *   - ordinary expressions;
 *   - identifiers;
 *   - qualified names;
 *   - attributes generally;
 *   - resource semantics generally;
 *   - capability semantics generally;
 *   - effects generally;
 *   - runtime execution;
 *   - dynamic library loading;
 *   - symbol resolution;
 *   - filesystem access;
 *   - network access;
 *   - hardware discovery;
 *   - target selection;
 *   - routing;
 *   - scheduling;
 *   - QEC;
 *   - ZQN;
 *   - HAL;
 *   - quantum IR.
 *
 * General FFI ownership:
 *
 *     grammar/interoperability/ffi.g4
 *
 * Foreign callable ownership:
 *
 *     grammar/interoperability/foreign-functions.g4
 *
 * ABI ownership:
 *
 *     grammar/interoperability/abi.g4
 *
 * General types:
 *
 *     grammar/types/
 *
 * General expressions:
 *
 *     grammar/expressions/
 *
 * Names:
 *
 *     grammar/core/names.g4
 *
 * Qualified references:
 *
 *     grammar/core/qualified-names.g4
 *
 * Attributes:
 *
 *     grammar/core/attributes.g4
 *
 * ============================================================================
 * ANTLR COMPOSITION CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * The production lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * Therefore:
 *
 *     tokenVocab = ZamaniLexer
 *
 * is mandatory.
 *
 * Canonical parser imports:
 *
 *     Expressions
 *     Types
 *     QualifiedNames
 *     Attributes
 *
 * are used instead of redefining shared language constructs.
 *
 * IMPORTANT:
 *
 * The current canonical lexer vocabulary does not define a PYTHON keyword.
 *
 * Therefore this grammar intentionally uses the existing:
 *
 *     EXTERN
 *     LANGUAGE
 *     STRING
 *     IDENTIFIER
 *
 * vocabulary to identify Python boundaries.
 *
 * Examples:
 *
 *     extern language "python" { ... }
 *
 *     extern language python { ... }
 *
 * This avoids introducing a second keyword authority.
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
 * This grammar therefore MUST NOT encode:
 *
 *     a particular interpreter path;
 *     a particular executable;
 *     a particular operating system;
 *     a particular CPU;
 *     a particular GPU;
 *     a particular accelerator;
 *     a particular QPU;
 *     a particular node;
 *     a particular address;
 *     a particular pointer width;
 *     a particular memory capacity;
 *     a particular filesystem;
 *     a particular virtual environment;
 *     a particular package installation;
 *     a particular process.
 *
 * Concrete realization belongs downstream.
 *
 * ============================================================================
 * SCALABILITY
 * ============================================================================
 *
 * No grammar-level finite limits are imposed on:
 *
 *     modules
 *     interfaces
 *     functions
 *     parameters
 *     arguments
 *     callbacks
 *     objects
 *     buffers
 *     iterators
 *     generators
 *     declarations
 *     requirements
 *     capabilities
 *     effects
 *     metadata
 *     program size
 *
 * Repetition is structural through ANTLR repetition operators.
 *
 * There are deliberately no:
 *
 *     MAX_PYTHON_MODULES
 *     MAX_PYTHON_FUNCTIONS
 *     MAX_PYTHON_ARGUMENTS
 *     MAX_PYTHON_OBJECTS
 *     MAX_PYTHON_BUFFERS
 *     MAX_PYTHON_THREADS
 *     MAX_PYTHON_DEVICES
 *
 * or equivalent universal limits.
 *
 * ============================================================================
 * SAFETY
 * ============================================================================
 *
 * This grammar contains:
 *
 *     no embedded Rust actions;
 *     no semantic predicates;
 *     no filesystem access;
 *     no network access;
 *     no runtime calls;
 *     no Python execution;
 *     no interpreter loading;
 *     no native pointer dereference;
 *     no hardware discovery.
 *
 * Downstream Rust implementation MUST remain safe Rust.
 *
 * ============================================================================
 */

parser grammar PythonInterop;

options {
    tokenVocab = ZamaniLexer;
}

import Expressions,
       Types,
       QualifiedNames,
       Attributes;


/* ============================================================================
 * PUBLIC ENTRY POINT
 * ========================================================================== */

/*
 * A reusable Python interoperability declaration sequence.
 *
 * The interoperability composition grammar decides where this entry point
 * participates in the complete Zamani program.
 */
pythonInterop
    : pythonInteropItem*
    ;


/* ============================================================================
 * INTEROPERABILITY ITEM
 * ========================================================================== */

pythonInteropItem
    : pythonBoundaryDeclaration
    | pythonInterfaceDeclaration
    | pythonModuleDeclaration
    | pythonCallableDeclaration
    | pythonObjectDeclaration
    | pythonCallbackDeclaration
    | pythonConversionDeclaration
    | pythonBufferDeclaration
    | pythonIteratorDeclaration
    | pythonGeneratorDeclaration
    | pythonAsyncDeclaration
    | pythonExceptionDeclaration
    | pythonRequirementDeclaration
    | pythonPolicyDeclaration
    | pythonLinkDeclaration
    ;


/* ============================================================================
 * COMMON PYTHON BOUNDARY
 * ========================================================================== */

/*
 * Canonical Python identity.
 *
 * Python is represented as a language identity rather than as a dedicated
 * lexer keyword.
 *
 * Both of these forms are supported:
 *
 *     extern language "python" { ... }
 *     extern language python { ... }
 *
 * The semantic layer MUST canonicalize the identity.
 */
pythonBoundaryDeclaration
    : attribute*
      EXTERN
      LANGUAGE
      pythonLanguageIdentity
      LBRACE
      pythonBoundaryItem*
      RBRACE
    ;


pythonLanguageIdentity
    : STRING
    | identifier
    | qualifiedNameReference
    ;


pythonBoundaryItem
    : pythonInterfaceDeclaration
    | pythonModuleDeclaration
    | pythonCallableDeclaration
    | pythonObjectDeclaration
    | pythonCallbackDeclaration
    | pythonConversionDeclaration
    | pythonBufferDeclaration
    | pythonIteratorDeclaration
    | pythonGeneratorDeclaration
    | pythonAsyncDeclaration
    | pythonExceptionDeclaration
    | pythonRequirementDeclaration
    | pythonPolicyDeclaration
    | pythonLinkDeclaration
    ;


/* ============================================================================
 * INTERFACE
 * ========================================================================== */

pythonInterfaceDeclaration
    : optionalAttributePrefix
      INTERFACE
      qualifiedNameReference
      pythonInterfaceParameterClause?
      LBRACE
      pythonInterfaceMember*
      RBRACE
    ;


pythonInterfaceParameterClause
    : LESS
      pythonInterfaceParameter
      (
          COMMA
          pythonInterfaceParameter
      )*
      GREATER
    ;


pythonInterfaceParameter
    : identifier
      (
          COLON
          typeExpression
      )?
    ;


pythonInterfaceMember
    : optionalAttributePrefix
      (
          pythonModuleDeclaration
        | pythonCallableDeclaration
        | pythonObjectDeclaration
        | pythonCallbackDeclaration
        | pythonConversionDeclaration
        | pythonBufferDeclaration
        | pythonIteratorDeclaration
        | pythonGeneratorDeclaration
        | pythonAsyncDeclaration
        | pythonExceptionDeclaration
        | pythonRequirementDeclaration
        | pythonPolicyDeclaration
        | pythonLinkDeclaration
      )
    ;


/* ============================================================================
 * MODULE
 * ========================================================================== */

pythonModuleDeclaration
    : optionalAttributePrefix
      MODULE
      pythonModuleIdentity
      pythonModuleClause*
      SEMICOLON
    ;


pythonModuleIdentity
    : STRING
    | qualifiedNameReference
    ;


pythonModuleClause
    : pythonImplementationClause
    | pythonVersionClause
    | pythonCompatibilityClause
    | pythonRequirementClause
    | pythonEnvironmentClause
    ;


/* ============================================================================
 * IMPLEMENTATION
 * ========================================================================== */

pythonImplementationClause
    : identifier
      ASSIGN
      pythonSymbolicValue
    ;


/* ============================================================================
 * VERSION
 * ========================================================================== */

pythonVersionClause
    : identifier
      pythonVersionOperator?
      pythonVersionValue
    ;


pythonVersionOperator
    : ASSIGN
    | EQUAL_EQUAL
    | NOT_EQUAL
    | LESS
    | LESS_EQUAL
    | GREATER
    | GREATER_EQUAL
    ;


pythonVersionValue
    : STRING
    | INTEGER
    | FLOAT
    | identifier
    | qualifiedNameReference
    ;


/* ============================================================================
 * CALLABLE
 * ========================================================================== */

/*
 * Declares an externally implemented Python callable.
 *
 * The callable identity remains symbolic.
 *
 * This rule does not define Python function syntax.
 */
pythonCallableDeclaration
    : optionalAttributePrefix
      FN
      pythonCallableIdentity
      LPAREN
      pythonParameterList?
      RPAREN
      pythonReturnClause?
      pythonCallableClause*
      SEMICOLON
    ;


pythonCallableIdentity
    : qualifiedNameReference
    | STRING
    ;


pythonParameterList
    : pythonParameter
      (
          COMMA
          pythonParameter
      )*
    ;


pythonParameter
    : identifier
      COLON
      typeExpression
      pythonParameterClause*
    ;


pythonParameterClause
    : pythonConversionClause
    | pythonOwnershipClause
    | pythonBorrowClause
    | pythonLifetimeClause
    | pythonNullabilityClause
    | pythonBufferReferenceClause
    | pythonOptionalClause
    | pythonKeywordOnlyClause
    | pythonPositionalOnlyClause
    | pythonVariadicClause
    ;


pythonReturnClause
    : THIN_ARROW
      typeExpression
      pythonReturnClauseItem*
    ;


pythonReturnClauseItem
    : pythonConversionClause
    | pythonOwnershipClause
    | pythonNullabilityClause
    | pythonBufferReferenceClause
    | pythonLifetimeClause
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
    | pythonBufferReferenceClause
    | pythonExceptionClause
    | pythonEffectClause
    | pythonRequirementClause
    | pythonCompatibilityClause
    | pythonAsyncClause
    | pythonStreamingClause
    | pythonThreadingClause
    | pythonInterpreterSerializationClause
    | pythonEnvironmentClause
    ;


/* ============================================================================
 * MODULE / SYMBOL REFERENCES
 * ========================================================================== */

pythonModuleReferenceClause
    : identifier
      ASSIGN
      pythonModuleIdentity
    ;


pythonSymbolClause
    : identifier
      ASSIGN
      pythonSymbolicValue
    ;


/* ============================================================================
 * OBJECT
 * ========================================================================== */

pythonObjectDeclaration
    : optionalAttributePrefix
      TYPE
      pythonObjectIdentity
      pythonObjectClause*
      SEMICOLON
    ;


pythonObjectIdentity
    : qualifiedNameReference
    | STRING
    ;


pythonObjectClause
    : pythonTypeClause
    | pythonRepresentationClause
    | pythonOwnershipClause
    | pythonBorrowClause
    | pythonLifetimeClause
    | pythonNullabilityClause
    | pythonConversionClause
    | pythonBufferReferenceClause
    | pythonRequirementClause
    | pythonCompatibilityClause
    ;


pythonTypeClause
    : identifier
      ASSIGN
      typeExpression
    ;


pythonRepresentationClause
    : identifier
      ASSIGN
      pythonSymbolicValue
    ;


/* ============================================================================
 * CALLBACK
 * ========================================================================== */

pythonCallbackDeclaration
    : optionalAttributePrefix
      identifier
      qualifiedNameReference
      LPAREN
      pythonParameterList?
      RPAREN
      pythonReturnClause?
      pythonCallbackClause*
      SEMICOLON
    ;


pythonCallbackClause
    : pythonOwnershipClause
    | pythonBorrowClause
    | pythonLifetimeClause
    | pythonThreadingClause
    | pythonInterpreterSerializationClause
    | pythonExceptionClause
    | pythonAsyncClause
    | pythonRequirementClause
    | pythonCompatibilityClause
    ;


/* ============================================================================
 * CONVERSION
 * ========================================================================== */

pythonConversionDeclaration
    : optionalAttributePrefix
      identifier
      pythonConversionSpec
      pythonConversionClause*
      SEMICOLON
    ;


pythonConversionClause
    : identifier
      pythonConversionSpec
    ;


pythonConversionSpec
    : pythonSymbolicValue
    | LPAREN
      expression
      RPAREN
    ;


/* ============================================================================
 * OWNERSHIP
 * ========================================================================== */

pythonOwnershipClause
    : identifier
      ASSIGN
      pythonOwnershipMode
    ;


pythonOwnershipMode
    : identifier
    | qualifiedNameReference
    | pythonSymbolicValue
    ;


pythonBorrowClause
    : identifier
      ASSIGN
      pythonBorrowMode
    ;


pythonBorrowMode
    : identifier
    | qualifiedNameReference
    | LPAREN
      expression
      RPAREN
    ;


pythonLifetimeClause
    : identifier
      ASSIGN
      pythonLifetimeSpec
    ;


pythonLifetimeSpec
    : qualifiedNameReference
    | STRING
    | LPAREN
      expression
      RPAREN
    ;


pythonNullabilityClause
    : identifier
      ASSIGN
      pythonNullabilityMode
    ;


pythonNullabilityMode
    : identifier
    | qualifiedNameReference
    ;


/* ============================================================================
 * BUFFER CONTRACT
 * ========================================================================== */

/*
 * Buffer metadata is semantic.
 *
 * It does not encode:
 *
 *     pointer width;
 *     alignment width;
 *     memory capacity;
 *     physical address;
 *     device memory size.
 */
pythonBufferDeclaration
    : optionalAttributePrefix
      identifier
      pythonBufferIdentity
      pythonBufferClause*
      SEMICOLON
    ;


pythonBufferIdentity
    : qualifiedNameReference
    | STRING
    ;


pythonBufferClause
    : identifier
      ASSIGN
      pythonBufferValue
    ;


pythonBufferValue
    : pythonSymbolicValue
    | expression
    ;


pythonBufferReferenceClause
    : identifier
      ASSIGN
      pythonSymbolicValue
    ;


/* ============================================================================
 * PARAMETER MODES
 * ========================================================================== */

pythonOptionalClause
    : identifier
      (
          ASSIGN
          expression
      )?
    ;


pythonKeywordOnlyClause
    : identifier
    ;


pythonPositionalOnlyClause
    : identifier
    ;


pythonVariadicClause
    : identifier
      pythonVariadicMode?
    ;


pythonVariadicMode
    : identifier
    | qualifiedNameReference
    ;


/* ============================================================================
 * ASYNCHRONOUS BOUNDARY
 * ========================================================================== */

pythonAsyncDeclaration
    : optionalAttributePrefix
      ASYNC
      qualifiedNameReference
      pythonAsyncClause*
      SEMICOLON
    ;


pythonAsyncClause
    : identifier
      (
          ASSIGN
          pythonSymbolicValue
      )?
    ;


/* ============================================================================
 * ITERATORS
 * ========================================================================== */

pythonIteratorDeclaration
    : optionalAttributePrefix
      identifier
      qualifiedNameReference
      pythonIteratorClause*
      SEMICOLON
    ;


pythonIteratorClause
    : identifier
      ASSIGN
      (
          typeExpression
        | pythonSymbolicValue
        | expression
      )
    ;


/* ============================================================================
 * GENERATORS
 * ========================================================================== */

pythonGeneratorDeclaration
    : optionalAttributePrefix
      identifier
      qualifiedNameReference
      pythonGeneratorClause*
      SEMICOLON
    ;


pythonGeneratorClause
    : identifier
      ASSIGN
      (
          typeExpression
        | pythonSymbolicValue
        | expression
      )
    ;


/* ============================================================================
 * EXCEPTIONS
 * ========================================================================== */

pythonExceptionDeclaration
    : optionalAttributePrefix
      identifier
      qualifiedNameReference
      pythonExceptionClause*
      SEMICOLON
    ;


pythonExceptionClause
    : identifier
      (
          ASSIGN
          pythonSymbolicValue
      )?
      pythonExceptionBody?
    ;


pythonExceptionBody
    : LBRACE
      pythonExceptionItem*
      RBRACE
    ;


pythonExceptionItem
    : identifier
      (
          ASSIGN
          (
              typeExpression
            | pythonSymbolicValue
            | expression
          )
      )?
      SEMICOLON?
    ;


/* ============================================================================
 * EFFECTS
 * ========================================================================== */

pythonEffectClause
    : WITH
      EFFECTS
      LBRACE
      pythonEffectReferenceList?
      RBRACE
    ;


pythonEffectReferenceList
    : qualifiedNameReference
      (
          COMMA
          qualifiedNameReference
      )*
    ;


/* ============================================================================
 * REQUIREMENTS
 * ========================================================================== */

pythonRequirementDeclaration
    : optionalAttributePrefix
      REQUIRES
      LBRACE
      pythonRequirement*
      RBRACE
    ;


pythonRequirementClause
    : REQUIRES
      LBRACE
      pythonRequirement*
      RBRACE
    ;


pythonRequirement
    : qualifiedNameReference
      (
          ASSIGN
          expression
      )?
      SEMICOLON?
    ;


/* ============================================================================
 * COMPATIBILITY
 * ========================================================================== */

pythonCompatibilityClause
    : identifier
      pythonCompatibilityValue
    ;


pythonCompatibilityValue
    : qualifiedNameReference
    | STRING
    | expression
    ;


/* ============================================================================
 * THREADING / EXECUTION POLICY
 * ========================================================================== */

pythonThreadingClause
    : identifier
      ASSIGN
      pythonSymbolicValue
    ;


pythonInterpreterSerializationClause
    : identifier
      ASSIGN
      pythonSymbolicValue
    ;


/* ============================================================================
 * STREAMING
 * ========================================================================== */

pythonStreamingClause
    : identifier
      (
          ASSIGN
          pythonSymbolicValue
      )?
    ;


/* ============================================================================
 * ENVIRONMENT
 * ========================================================================== */

pythonEnvironmentClause
    : identifier
      LBRACE
      pythonEnvironmentItem*
      RBRACE
    ;


pythonEnvironmentItem
    : identifier
      (
          ASSIGN
          (
              pythonSymbolicValue
            | expression
          )
      )?
      SEMICOLON?
    ;


/* ============================================================================
 * POLICY
 * ========================================================================== */

pythonPolicyDeclaration
    : optionalAttributePrefix
      identifier
      qualifiedNameReference
      LBRACE
      pythonPolicyItem*
      RBRACE
    ;


pythonPolicyItem
    : identifier
      pythonPolicySubject?
      SEMICOLON?
    ;


pythonPolicySubject
    : qualifiedNameReference
    | pythonSymbolicValue
    | expression
    ;


/* ============================================================================
 * LINK ASSOCIATION
 * ========================================================================== */

pythonLinkDeclaration
    : optionalAttributePrefix
      identifier
      qualifiedNameReference
      LBRACE
      pythonLinkItem*
      RBRACE
    ;


pythonLinkItem
    : identifier
      ASSIGN
      (
          qualifiedNameReference
        | pythonSymbolicValue
        | expression
      )
      SEMICOLON?
    ;


/* ============================================================================
 * PYTHON CALL EXPRESSIONS
 * ========================================================================== */

/*
 * Python calls are semantic boundary operations.
 *
 * They do not execute during parsing.
 *
 * Examples:
 *
 *     python call math::sin(x)
 *
 *     python call "numpy"::linalg::solve(a, b)
 *
 * The first "python" is intentionally an identifier, not a new keyword.
 */
pythonCallExpression
    : identifier
      identifier
      qualifiedNameReference
      LPAREN
      argumentList?
      RPAREN
    ;


pythonQualifiedCallExpression
    : identifier
      identifier
      STRING
      DOUBLE_COLON
      qualifiedNameReference
      LPAREN
      argumentList?
      RPAREN
    ;


pythonCallableReferenceExpression
    : identifier
      identifier
      qualifiedNameReference
    ;


pythonObjectReferenceExpression
    : identifier
      identifier
      qualifiedNameReference
    ;


pythonCallStatement
    : pythonCallExpression
      SEMICOLON
    | pythonQualifiedCallExpression
      SEMICOLON
    ;


/* ============================================================================
 * GENERIC SYMBOLIC VALUES
 * ========================================================================== */

pythonSymbolicValue
    : qualifiedNameReference
    | STRING
    | INTEGER
    | FLOAT
    | CHAR
    | TRUE
    | FALSE
    | identifier
    ;


pythonAttributeBlock
    : LBRACE
      attribute*
      RBRACE
    ;


/* ============================================================================
 * SEMANTIC INTEGRATION CONTRACT
 * ============================================================================
 *
 * The frontend AST MUST preserve, where present:
 *
 *     source span
 *     Python boundary identity
 *     interface identity
 *     module identity
 *     callable identity
 *     parameter ordering
 *     parameter types
 *     return type
 *     conversion metadata
 *     ownership metadata
 *     borrow/lifetime metadata
 *     nullability
 *     buffer metadata
 *     asynchronous metadata
 *     iterator/generator metadata
 *     callback metadata
 *     exception metadata
 *     effect references
 *     requirements
 *     compatibility metadata
 *     environment metadata
 *     policy metadata
 *     implementation metadata
 *     symbolic attributes
 *
 * The AST MUST NOT contain runtime handles, pointers, interpreter instances,
 * native addresses, process identifiers, or device identifiers merely because
 * this grammar was parsed.
 *
 * ============================================================================
 * SEMANTIC RESPONSIBILITY
 * ============================================================================
 *
 * Semantic analysis MUST validate:
 *
 *     - Python language identity;
 *     - module identity;
 *     - callable identity;
 *     - type compatibility;
 *     - conversion legality;
 *     - ownership compatibility;
 *     - borrow/lifetime validity;
 *     - nullability;
 *     - buffer compatibility;
 *     - asynchronous compatibility;
 *     - iterator/generator contracts;
 *     - callback contracts;
 *     - exception mapping;
 *     - effects;
 *     - capabilities;
 *     - resource requirements;
 *     - security policy;
 *     - implementation compatibility;
 *     - target availability;
 *     - lowering availability.
 *
 * None of these are parser decisions.
 *
 * ============================================================================
 * ABI INTEGRATION
 * ============================================================================
 *
 * This grammar MUST NOT duplicate ABI syntax.
 *
 * If a Python boundary requires ABI information, the semantic interoperability
 * model references the canonical ABI contract owned by:
 *
 *     grammar/interoperability/abi.g4
 *
 * Likewise, calling conventions remain owned by:
 *
 *     grammar/interoperability/calling-conventions.g4
 *
 * Python syntax may refer to these contracts symbolically through names,
 * attributes, requirements, or semantic metadata.
 *
 * ============================================================================
 * FFI INTEGRATION
 * ============================================================================
 *
 * This grammar is a language-specific specialization of the general
 * interoperability architecture.
 *
 * General FFI semantics remain owned by:
 *
 *     grammar/interoperability/ffi.g4
 *
 * Python-specific information belongs here.
 *
 * The Python grammar MUST NOT become a second FFI implementation.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Python may act as a host or interoperability boundary for quantum
 * computation.
 *
 * This grammar therefore permits Python-facing types and symbolic operations
 * to participate in hybrid programs.
 *
 * It MUST NOT define:
 *
 *     quantum gates;
 *     physical qubits;
 *     QubitId;
 *     topology;
 *     routing;
 *     scheduling;
 *     calibration;
 *     QEC;
 *     ZQN.
 *
 * If Python tooling produces or consumes quantum semantics, the downstream
 * path remains:
 *
 *     Python boundary
 *          |
 *          v
 *     semantic quantum model
 *          |
 *          v
 *     quantum::ir
 *          |
 *          v
 *     optimization
 *          |
 *          v
 *     routing / scheduling
 *          |
 *          v
 *     QEC / resilience / ZQN
 *          |
 *          v
 *     HAL
 *          |
 *          v
 *     target
 *
 * ============================================================================
 * CLASSICAL / AI / DATA INTEGRATION
 * ============================================================================
 *
 * Python interoperability may expose:
 *
 *     numerical computation
 *     tensors
 *     datasets
 *     models
 *     symbolic computation
 *     scientific computation
 *     distributed computation
 *     networking
 *     data processing
 *
 * The grammar does not enumerate frameworks.
 *
 * Framework identities remain symbolic.
 *
 * ============================================================================
 * HARDWARE / HDL INTEGRATION
 * ============================================================================
 *
 * Python may be used as an orchestration or tooling boundary for hardware,
 * FPGA, accelerator, simulation, verification, or deployment workflows.
 *
 * This grammar does not select:
 *
 *     CPU
 *     GPU
 *     FPGA
 *     ASIC
 *     QPU
 *     device
 *     node
 *     physical address.
 *
 * Such choices belong downstream.
 *
 * ============================================================================
 * SECURITY
 * ============================================================================
 *
 * A declaration is not authorization.
 *
 * For example:
 *
 *     python call ...
 *
 * does not itself authorize:
 *
 *     filesystem access;
 *     network access;
 *     process creation;
 *     package installation;
 *     native code execution;
 *     environment inspection.
 *
 * Security/capability analysis must authorize those actions independently.
 *
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 *
 * Parsing depends only on:
 *
 *     source text;
 *     selected grammar;
 *     selected lexer vocabulary;
 *     explicit parser configuration.
 *
 * Parsing MUST NOT depend on:
 *
 *     installed Python versions;
 *     installed packages;
 *     filesystem contents;
 *     environment variables;
 *     network state;
 *     available hardware;
 *     runtime state.
 *
 * ============================================================================
 * COMPLETION CONTRACT
 * ============================================================================
 *
 * This file is complete when:
 *
 * [x] It is a parser grammar.
 * [x] It uses tokenVocab = ZamaniLexer.
 * [x] It imports canonical Expressions.
 * [x] It imports canonical Types.
 * [x] It imports canonical QualifiedNames.
 * [x] It imports canonical Attributes.
 * [x] It does not define Python language syntax.
 * [x] It does not enumerate Python implementations.
 * [x] It does not encode a Python version ceiling.
 * [x] It does not encode machine limits.
 * [x] It does not encode interpreter paths.
 * [x] It does not implement FFI generally.
 * [x] It does not implement ABI generally.
 * [x] It does not create a quantum IR.
 * [x] It does not perform runtime execution.
 * [x] It does not require unsafe Rust.
 * [x] It supports symbolic Python identities.
 * [x] It supports modules.
 * [x] It supports callable contracts.
 * [x] It supports callbacks.
 * [x] It supports object boundaries.
 * [x] It supports conversions.
 * [x] It supports buffer contracts.
 * [x] It supports asynchronous boundaries.
 * [x] It supports iterators.
 * [x] It supports generators.
 * [x] It supports exception contracts.
 * [x] It supports requirements.
 * [x] It supports compatibility metadata.
 * [x] It supports hybrid participation.
 * [x] It preserves open-world scalability.
 *
 * Repository integration still requires the interoperability dispatcher to
 * import this grammar and expose `pythonInterop` at the correct universal
 * interoperability boundary.
 *
 * ============================================================================
 * END OF PYTHON INTEROPERABILITY GRAMMAR
 * ============================================================================
 */
/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/interoperability/ffi.g4
 *
 * Grammar:
 *     Ffi
 *
 * Status:
 *     CANONICAL SOURCE-LEVEL FOREIGN-INTERFACE BOUNDARY
 *
 * Rust baseline:
 *     Rust 1.97+
 *     Rust 2021
 *     SAFE RUST ONLY
 *
 * ============================================================================
 * FEATURE CONTRACT
 * ============================================================================
 *
 * PURPOSE
 * -------
 *
 * This grammar owns the SOURCE-LEVEL FOREIGN-INTERFACE BOUNDARY.
 *
 * It describes the portable intent of a Zamani program when computation,
 * values, callbacks, services, or data cross a foreign implementation
 * boundary.
 *
 * This file describes:
 *
 *     - foreign interface declarations;
 *     - foreign callable bindings;
 *     - foreign callable references;
 *     - foreign calls;
 *     - callback contracts;
 *     - marshalling intent;
 *     - ownership intent;
 *     - borrowing intent;
 *     - lifetime intent;
 *     - nullability intent;
 *     - representation intent;
 *     - encoding intent;
 *     - effect references;
 *     - capability requirements;
 *     - resource requirements;
 *     - error-boundary intent;
 *     - compatibility intent;
 *     - security intent;
 *     - execution intent;
 *     - concurrency intent;
 *     - determinism intent;
 *     - provenance metadata;
 *     - adaptation metadata.
 *
 * This grammar DOES NOT implement:
 *
 *     - an ABI;
 *     - a linker;
 *     - a loader;
 *     - a dynamic-library resolver;
 *     - a foreign runtime;
 *     - a foreign programming language;
 *     - native memory access;
 *     - pointer dereferencing;
 *     - physical address access;
 *     - machine-register access;
 *     - hardware discovery;
 *     - device selection;
 *     - quantum routing;
 *     - quantum scheduling;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime invocation.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 * --------------
 *
 *     ffiItem
 *     ffiForeignDeclaration
 *     ffiInterfaceDeclaration
 *     ffiBindingDeclaration
 *     ffiCallbackDeclaration
 *     ffiCallExpression
 *     ffiCallbackCallExpression
 *     ffiCallableReferenceExpression
 *     ffiCallStatement
 *     ffiCallbackCallStatement
 *     FFI-specific boundary contracts
 *
 * THIS FILE DOES NOT OWN
 * ----------------------
 *
 *     identifier
 *     qualifiedName
 *     attribute
 *     expression
 *     typeExpression
 *     literals
 *     ordinary functions
 *     ordinary parameters
 *     ordinary calls
 *     ABI declarations
 *     ABI layouts
 *     calling conventions
 *     foreign type declarations
 *     general foreign-function declarations
 *     effects
 *     capabilities
 *     resources
 *     policies
 *     security semantics
 *     provenance semantics
 *     target selection
 *     runtime execution
 *
 * Canonical owners:
 *
 *     names
 *         -> grammar/core/names.g4
 *
 *     attributes
 *         -> grammar/core/attributes.g4
 *
 *     expressions
 *         -> grammar/expressions/expressions.g4
 *
 *     types
 *         -> grammar/types/types.g4
 *
 *     ABI
 *         -> grammar/interoperability/abi.g4
 *
 *     foreign callable declarations
 *         -> grammar/interoperability/foreign-functions.g4
 *
 *     foreign types
 *         -> grammar/interoperability/foreign-types.g4
 *
 *     calling conventions
 *         -> grammar/interoperability/calling-conventions.g4
 *
 *     interoperability composition
 *         -> grammar/interoperability/interoperability.g4
 *
 * ============================================================================
 * DEPENDENCY CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON
 * ----------
 *
 *     ZamaniLexer
 *     Names
 *     Attributes
 *     Expressions
 *     Type
 *
 * EXPORTS
 * -------
 *
 *     ffiItem
 *     ffiForeignDeclaration
 *     ffiInterfaceDeclaration
 *     ffiBindingDeclaration
 *     ffiCallbackDeclaration
 *     ffiCallExpression
 *     ffiCallbackCallExpression
 *     ffiCallableReferenceExpression
 *     ffiCallStatement
 *     ffiCallbackCallStatement
 *     ffiQualifiedCallExpression
 *     ffiQualifiedCallbackCallExpression
 *
 * CONSUMED_BY
 * -----------
 *
 *     grammar/interoperability/interoperability.g4
 *     grammar/antlr/ZamaniParser.g4
 *     interoperability semantic analysis
 *     AST construction
 *     effect analysis
 *     capability analysis
 *     resource analysis
 *     policy/security analysis
 *     canonical semantic model
 *     compiler/lowering
 *
 * AST_OWNER
 * ---------
 *
 *     domain-neutral frontend AST
 *
 * FFI syntax MUST NOT create a foreign-specific AST universe.
 *
 * SEMANTIC_OWNER
 * --------------
 *
 *     interoperability semantic analysis
 *
 * IR_OWNER
 * --------
 *
 *     canonical semantic model
 *
 * Foreign calls participating in quantum computation MUST continue through
 * the canonical quantum::ir boundary where their semantic operation requires
 * quantum representation.
 *
 * TEST_OWNER
 * ----------
 *
 *     grammar/tests/interoperability/
 *     grammar/tests/negative/
 *     grammar/tests/boundary/
 *     grammar/tests/scalability/
 *
 * SPEC_OWNER
 * ----------
 *
 *     grammar/spec/interoperability.md
 *
 * ============================================================================
 * POCO-REAF CONTRACT
 * ============================================================================
 *
 * A foreign boundary expresses portable computational intent.
 *
 * It MUST NOT impose universal limits on:
 *
 *     CPUs
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     QPUs
 *     qubits
 *     registers
 *     memory
 *     nodes
 *     threads
 *     devices
 *     network endpoints
 *     tensor dimensions
 *     interface members
 *     parameters
 *     callbacks
 *     declarations
 *
 * There are deliberately no MAX_* constants in this grammar.
 *
 * A source program may therefore describe a foreign boundary for:
 *
 *     tiny embedded systems
 *     CPUs
 *     multicore systems
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     quantum processors
 *     simulators
 *     HPC systems
 *     clusters
 *     distributed systems
 *     cloud systems
 *     future computational targets
 *
 * Target feasibility is determined downstream from:
 *
 *     requirements
 *     capabilities
 *     resources
 *     policies
 *     effects
 *     compatibility
 *
 * ============================================================================
 * SAFETY CONTRACT
 * ============================================================================
 *
 * This grammar contains:
 *
 *     - no embedded Rust actions;
 *     - no semantic predicates;
 *     - no filesystem access;
 *     - no network access;
 *     - no process execution;
 *     - no foreign-code execution;
 *     - no hardware inspection;
 *     - no runtime callbacks;
 *     - no unsafe implementation requirement.
 *
 * Rust code consuming the grammar MUST remain compatible with:
 *
 *     Rust 1.97+
 *     Rust 2021
 *     safe Rust only
 *
 * ============================================================================
 * LEXICAL CONTRACT
 * ============================================================================
 *
 * The parser-facing lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This grammar MUST consume:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * It MUST NOT create:
 *
 *     - a second lexer;
 *     - parser-local lexer rules;
 *     - duplicated token definitions;
 *     - target-language keyword vocabularies.
 *
 * Important:
 *
 * The existing canonical lexer already provides:
 *
 *     FOREIGN
 *     EXTERN
 *     FN
 *     INTERFACE
 *     ASYNC
 *     WITH
 *     EFFECTS
 *     REQUIRES
 *     CAPABILITY
 *     RESOURCES
 *     SECURITY
 *     VERSION
 *     FROM
 *     TO
 *     AS
 *     IN
 *     OUT
 *     TYPE
 *     RESULT
 *     THROW
 *     TRY
 *     CATCH
 *
 * Where a dedicated FFI spelling is not lexically reserved, this grammar
 * deliberately uses existing canonical tokens rather than inventing an
 * implicit parser token.
 *
 * ============================================================================
 * NON-DUPLICATION CONTRACT
 * ============================================================================
 *
 * Do NOT add:
 *
 *     parameterList
 *     argumentList
 *     typeExpr
 *     stringLiteral
 *     identifier
 *     qualifiedName
 *
 * to this file.
 *
 * FFI-specific wrappers use the canonical rules already imported from:
 *
 *     Names
 *     Attributes
 *     Expressions
 *     Type
 *
 * ============================================================================
 */

parser grammar Ffi;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names,
    Attributes,
    Expressions,
    Type
;


/*
 * ============================================================================
 * PUBLIC FFI ENTRY POINT
 * ============================================================================
 *
 * This is a delegate entry point.
 *
 * The complete source program remains owned by:
 *
 *     grammar/antlr/ZamaniParser.g4
 *
 * ============================================================================
 */

ffiItem
    : ffiForeignDeclaration
    | ffiCallStatement
    | ffiCallbackCallStatement
    ;


/*
 * ============================================================================
 * FOREIGN DECLARATION DISPATCH
 * ============================================================================
 */

ffiForeignDeclaration
    : ffiInterfaceDeclaration
    | ffiBindingDeclaration
    | ffiCallbackDeclaration
    ;


/*
 * ============================================================================
 * FOREIGN INTERFACE
 * ============================================================================
 *
 * Canonical surface form:
 *
 *     foreign interface Math {
 *         foreign fn sin(value: Real) -> Real;
 *     }
 *
 * The `foreign` keyword is already part of the canonical lexer vocabulary.
 *
 * The interface name is symbolic.
 *
 * It is NOT interpreted here as:
 *
 *     a library filename
 *     a filesystem path
 *     a device
 *     a runtime
 *     an executable
 *     a network endpoint
 *
 * ============================================================================
 */

ffiInterfaceDeclaration
    : attribute*
      FOREIGN
      INTERFACE
      identifier
      ffiInterfaceGenericParameters?
      ffiInterfaceBody
    ;


ffiInterfaceGenericParameters
    : LESS
      ffiGenericParameter
      (
          COMMA
          ffiGenericParameter
      )*
      GREATER
    ;


ffiGenericParameter
    : identifier
      (
          COLON
          qualifiedName
      )*
    ;


ffiInterfaceBody
    : LBRACE
      ffiInterfaceMember*
      RBRACE
    ;


ffiInterfaceMember
    : attribute*
      ffiBindingDeclaration
    | attribute*
      ffiCallbackDeclaration
    ;


/*
 * ============================================================================
 * FOREIGN CALLABLE BINDING
 * ============================================================================
 *
 * Canonical surface form:
 *
 *     foreign fn sin(value: Real) -> Real;
 *
 *     foreign fn sin(value: Real) -> Real
 *         with {
 *             effects { math::pure };
 *             requires { capability::foreign_call; };
 *         };
 *
 * This is a declaration only.
 *
 * It does not resolve, load, link, or execute the foreign symbol.
 *
 * ============================================================================
 */

ffiBindingDeclaration
    : attribute*
      FOREIGN
      FN
      identifier
      ffiGenericParameterList?
      LPAREN
      ffiParameterList?
      RPAREN
      ffiReturnType?
      ffiContractClause?
      ffiTargetClause?
      SEMICOLON
    ;


ffiGenericParameterList
    : LESS
      ffiGenericParameter
      (
          COMMA
          ffiGenericParameter
      )*
      GREATER
    ;


ffiParameterList
    : ffiParameter
      (
          COMMA
          ffiParameter
      )*
    ;


ffiParameter
    : attribute*
      identifier
      (
          COLON
          typeExpression
      )?
      ffiParameterContractClause?
    ;


ffiParameterContractClause
    : ffiBoundaryClause*
    ;


ffiReturnType
    : THIN_ARROW
      typeExpression
    ;


/*
 * ============================================================================
 * CALLBACK CONTRACT
 * ============================================================================
 *
 * A callback is represented as a foreign callable contract with callback
 * metadata rather than by inventing a second callable type system.
 *
 * The semantic layer MUST identify the callback role from the declaration's
 * attributes/contract metadata.
 *
 * This preserves one callable representation while allowing foreign code to
 * invoke Zamani-owned functionality.
 *
 * ============================================================================
 */

ffiCallbackDeclaration
    : attribute*
      FOREIGN
      FN
      identifier
      ffiGenericParameterList?
      LPAREN
      ffiParameterList?
      RPAREN
      ffiReturnType?
      ffiCallbackContractClause
      SEMICOLON
    ;


ffiCallbackContractClause
    : WITH
      LBRACE
      ffiCallbackContractItem*
      RBRACE
    ;


ffiCallbackContractItem
    : ffiBoundaryClause
    | ffiEffectClause
    | ffiRequirementClause
    | ffiErrorClause
    | ffiCompatibilityClause
    | ffiSecurityClause
    | ffiConcurrencyClause
    | ffiDeterminismClause
    | ffiProvenanceClause
    | ffiAttributeClause
    ;


/*
 * ============================================================================
 * FOREIGN TARGET
 * ============================================================================
 *
 * A target is symbolic.
 *
 * Examples:
 *
 *     foreign fn sin(...) -> Real with { ... };
 *
 *     foreign fn sin(...) -> Real
 *         with {
 *             target::symbol = "sin";
 *         };
 *
 * The grammar does not interpret a string as a path or executable.
 *
 * ============================================================================
 */

ffiTargetClause
    : WITH
      LBRACE
      ffiTargetItem*
      RBRACE
    ;


ffiTargetItem
    : ffiTargetIdentity
    | ffiAttributeClause
    ;


ffiTargetIdentity
    : FOREIGN
      AS
      qualifiedName
      SEMICOLON
    | AS
      qualifiedName
      SEMICOLON
    ;


/*
 * ============================================================================
 * CONTRACT
 * ============================================================================
 *
 * The contract groups source-level interoperability metadata.
 *
 * Each semantic concern remains independently analyzable downstream.
 * ============================================================================
 */

ffiContractClause
    : WITH
      LBRACE
      ffiContractItem*
      RBRACE
    ;


ffiContractItem
    : ffiBoundaryClause
    | ffiEffectClause
    | ffiRequirementClause
    | ffiResourceClause
    | ffiErrorClause
    | ffiCompatibilityClause
    | ffiSecurityClause
    | ffiExecutionClause
    | ffiConcurrencyClause
    | ffiDeterminismClause
    | ffiProvenanceClause
    | ffiAdaptationClause
    | ffiAttributeClause
    ;


/*
 * ============================================================================
 * BOUNDARY CONTRACT
 * ============================================================================
 */

ffiBoundaryClause
    : ffiMarshalClause
    | ffiOwnershipClause
    | ffiBorrowClause
    | ffiLifetimeClause
    | ffiNullabilityClause
    | ffiRepresentationClause
    | ffiEncodingClause
    | ffiSizeClause
    | ffiAlignmentClause
    | ffiDirectionClause
    | ffiPinningClause
    | ffiBlockingClause
    | ffiAsyncClause
    | ffiStreamingClause
    ;


/*
 * ============================================================================
 * MARSHALLING
 * ============================================================================
 *
 * Marshalling is intent.
 *
 * It does not prescribe the implementation.
 * ============================================================================
 */

ffiMarshalClause
    : FOREIGN
      AS
      ffiMarshalSpecification
      SEMICOLON
    ;


ffiMarshalSpecification
    : qualifiedName
    | STRING_LITERAL
    | LPAREN
      expression
      RPAREN
    ;


/*
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 */

ffiOwnershipClause
    : FOREIGN
      ffiOwnershipSpecification
      SEMICOLON
    ;


ffiOwnershipSpecification
    : qualifiedName
    | STRING_LITERAL
    | expression
    ;


/*
 * ============================================================================
 * BORROWING
 * ============================================================================
 */

ffiBorrowClause
    : FOREIGN
      ffiBorrowSpecification
      SEMICOLON
    ;


ffiBorrowSpecification
    : qualifiedName
    | STRING_LITERAL
    | expression
    ;


/*
 * ============================================================================
 * LIFETIME
 * ============================================================================
 */

ffiLifetimeClause
    : ffiNamedExpressionClause
    ;


ffiNullabilityClause
    : ffiNamedValueClause
    ;


ffiRepresentationClause
    : ffiNamedValueClause
    ;


ffiEncodingClause
    : ffiNamedValueClause
    ;


ffiSizeClause
    : ffiNamedExpressionClause
    ;


ffiAlignmentClause
    : ffiNamedExpressionClause
    ;


ffiPinningClause
    : ffiNamedValueClause
    ;


ffiBlockingClause
    : ffiNamedValueClause
    ;


ffiNamedExpressionClause
    : identifier
      ASSIGN
      expression
      SEMICOLON
    ;


ffiNamedValueClause
    : identifier
      ASSIGN
      ffiSymbolicValue
      SEMICOLON
    ;


/*
 * ============================================================================
 * DIRECTION
 * ============================================================================
 */

ffiDirectionClause
    : identifier
      ASSIGN
      ffiDirection
      SEMICOLON
    ;


ffiDirection
    : IN
    | OUT
    | IN
      OUT
    ;


/*
 * ============================================================================
 * ASYNCHRONY
 * ============================================================================
 *
 * ASYNC is an existing canonical lexer token.
 *
 * Runtime scheduling remains downstream.
 * ============================================================================
 */

ffiAsyncClause
    : ASYNC
      ffiSymbolicValue?
      SEMICOLON
    ;


ffiStreamingClause
    : identifier
      ASSIGN
      ffiSymbolicValue
      SEMICOLON
    ;


/*
 * ============================================================================
 * EFFECTS
 * ============================================================================
 *
 * FFI effects are references to the canonical effect system.
 *
 * This file does not define effect semantics.
 * ============================================================================
 */

ffiEffectClause
    : EFFECTS
      LBRACE
      ffiEffectReferenceList?
      RBRACE
    ;


ffiEffectReferenceList
    : qualifiedName
      (
          COMMA
          qualifiedName
      )*
    ;


/*
 * ============================================================================
 * REQUIREMENTS / CAPABILITIES
 * ============================================================================
 *
 * Requirements are symbolic semantic constraints.
 *
 * They do not select a target.
 * ============================================================================
 */

ffiRequirementClause
    : REQUIRES
      LBRACE
      ffiRequirement*
      RBRACE
    ;


ffiRequirement
    : qualifiedName
      (
          ASSIGN
          expression
      )?
      SEMICOLON?
    ;


/*
 * ============================================================================
 * RESOURCES
 * ============================================================================
 *
 * Resource quantities remain expressions.
 *
 * No fixed capacity is represented here.
 * ============================================================================
 */

ffiResourceClause
    : RESOURCES
      LBRACE
      ffiResourceItem*
      RBRACE
    ;


ffiResourceItem
    : REQUIRES
      expression
      SEMICOLON
    | identifier
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * ERROR BOUNDARY
 * ============================================================================
 */

ffiErrorClause
    : identifier
      LBRACE
      ffiErrorItem*
      RBRACE
    ;


ffiErrorItem
    : identifier
      ASSIGN
      (
          typeExpression
        | expression
        | ffiSymbolicValue
      )
      SEMICOLON
    ;


/*
 * ============================================================================
 * COMPATIBILITY
 * ============================================================================
 */

ffiCompatibilityClause
    : identifier
      WITH
      ffiCompatibilityTarget
      (
          LBRACE
          ffiCompatibilityItem*
          RBRACE
      )?
    ;


ffiCompatibilityTarget
    : qualifiedName
    | STRING_LITERAL
    ;


ffiCompatibilityItem
    : identifier
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * SECURITY
 * ============================================================================
 */

ffiSecurityClause
    : identifier
      LBRACE
      ffiSecurityItem*
      RBRACE
    ;


ffiSecurityItem
    : identifier
      ASSIGN
      (
          qualifiedName
        | expression
        | ffiSymbolicValue
      )
      SEMICOLON
    ;


/*
 * ============================================================================
 * EXECUTION
 * ============================================================================
 *
 * Execution metadata is descriptive only.
 *
 * It must not become physical backend selection.
 * ============================================================================
 */

ffiExecutionClause
    : identifier
      LBRACE
      ffiExecutionItem*
      RBRACE
    ;


ffiExecutionItem
    : identifier
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * CONCURRENCY
 * ============================================================================
 */

ffiConcurrencyClause
    : identifier
      LBRACE
      ffiConcurrencyItem*
      RBRACE
    ;


ffiConcurrencyItem
    : identifier
      ASSIGN
      (
          expression
        | ffiSymbolicValue
      )
      SEMICOLON
    ;


/*
 * ============================================================================
 * DETERMINISM
 * ============================================================================
 */

ffiDeterminismClause
    : identifier
      ASSIGN
      ffiSymbolicValue
      SEMICOLON
    ;


/*
 * ============================================================================
 * PROVENANCE
 * ============================================================================
 *
 * Provenance remains metadata.
 *
 * It does not establish trust by itself.
 * ============================================================================
 */

ffiProvenanceClause
    : PROVENANCE
      LBRACE
      ffiProvenanceItem*
      RBRACE
    ;


ffiProvenanceItem
    : identifier
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * ADAPTATION
 * ============================================================================
 *
 * Adaptation is a semantic contract.
 *
 * It does not authorize arbitrary self-modification.
 * ============================================================================
 */

ffiAdaptationClause
    : ADAPTATION
      LBRACE
      ffiAdaptationItem*
      RBRACE
    ;


ffiAdaptationItem
    : identifier
      ASSIGN
      (
          expression
        | qualifiedName
      )
      SEMICOLON
    ;


/*
 * ============================================================================
 * ATTRIBUTE METADATA
 * ============================================================================
 *
 * General attributes remain owned by Attributes.
 *
 * This local wrapper is only used where an FFI contract needs a sequence of
 * arbitrary metadata assignments.
 * ============================================================================
 */

ffiAttributeClause
    : identifier
      ASSIGN
      expression
      SEMICOLON
    ;


/*
 * ============================================================================
 * SYMBOLIC VALUES
 * ============================================================================
 *
 * Open symbolic values are intentional.
 *
 * They allow future:
 *
 *     ABIs
 *     runtimes
 *     languages
 *     execution models
 *     representations
 *     vendors
 *     accelerators
 *     quantum systems
 *     hardware systems
 *
 * without adding a new grammar branch for every possibility.
 * ============================================================================
 */

ffiSymbolicValue
    : qualifiedName
    | STRING_LITERAL
    ;


/*
 * ============================================================================
 * CALLBACK INVOCATION
 * ============================================================================
 *
 * Callback invocation uses an explicitly foreign-qualified callable reference.
 *
 * The runtime performs invocation only after semantic resolution.
 * ============================================================================
 */

ffiCallbackCallExpression
    : FOREIGN
      ffiCallableTarget
      LPAREN
      expressionArgumentList?
      RPAREN
    ;


ffiCallbackCallStatement
    : ffiCallbackCallExpression
      SEMICOLON
    ;


/*
 * ============================================================================
 * FOREIGN CALL
 * ============================================================================
 *
 * The canonical call form is:
 *
 *     foreign target(...)
 *
 * This avoids requiring a separate parser-local `call` keyword.
 * ============================================================================
 */

ffiCallExpression
    : FOREIGN
      ffiCallableTarget
      LPAREN
      expressionArgumentList?
      RPAREN
    ;


ffiCallStatement
    : ffiCallExpression
      SEMICOLON
    ;


ffiCallableTarget
    : qualifiedName
    | STRING_LITERAL
      DOUBLE_COLON
      qualifiedName
    | STRING_LITERAL
    ;


/*
 * ============================================================================
 * CALLABLE REFERENCE
 * ============================================================================
 */

ffiCallableReferenceExpression
    : FOREIGN
      qualifiedName
    ;


/*
 * ============================================================================
 * EXPRESSION ARGUMENT LIST
 * ============================================================================
 *
 * Expressions remain owned by Expressions.
 *
 * This rule only composes the canonical expression rule into an FFI call.
 * ============================================================================
 */

expressionArgumentList
    : expression
      (
          COMMA
          expression
      )*
    ;


/*
 * ============================================================================
 * STABLE COMPOSITION ALIASES
 * ============================================================================
 */

ffiQualifiedCallExpression
    : ffiCallExpression
    ;


ffiQualifiedCallbackCallExpression
    : ffiCallbackCallExpression
    ;


/*
 * ============================================================================
 * LOCAL INTEGRATION INVARIANTS
 * ============================================================================
 *
 * 1. This grammar consumes ZamaniLexer.
 *
 * 2. No lexer rule exists here.
 *
 * 3. No AST type exists here.
 *
 * 4. No semantic implementation exists here.
 *
 * 5. No ABI implementation exists here.
 *
 * 6. No foreign language grammar exists here.
 *
 * 7. No physical hardware identity exists here.
 *
 * 8. No fixed machine capacity exists here.
 *
 * 9. No target selection exists here.
 *
 * 10. No runtime execution exists here.
 *
 * 11. Foreign calls participate downstream in:
 *
 *         type checking
 *         effect checking
 *         capability checking
 *         resource checking
 *         policy checking
 *         provenance
 *         compatibility
 *
 * 12. ABI-specific information is consumed through:
 *
 *         grammar/interoperability/abi.g4
 *
 * 13. Foreign types are consumed through:
 *
 *         grammar/interoperability/foreign-types.g4
 *
 * 14. General external callable declarations are consumed through:
 *
 *         grammar/interoperability/foreign-functions.g4
 *
 * 15. The interoperability composition grammar owns the public dispatcher.
 *
 * 16. The canonical Zamani parser remains the only complete-program parser.
 *
 * ============================================================================
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when:
 *
 *     [ ] ANTLR generation succeeds with the canonical ZamaniLexer.
 *
 *     [ ] No implicit parser token is created.
 *
 *     [ ] No duplicate parser grammar name exists.
 *
 *     [ ] No duplicate identifier rule exists.
 *
 *     [ ] No duplicate qualified-name rule exists.
 *
 *     [ ] No duplicate type-expression rule exists.
 *
 *     [ ] No duplicate general expression rule exists.
 *
 *     [ ] No duplicate ABI grammar exists.
 *
 *     [ ] No duplicate foreign-type grammar exists.
 *
 *     [ ] No duplicate general foreign-function grammar exists.
 *
 *     [ ] FFI declarations parse deterministically.
 *
 *     [ ] FFI calls parse deterministically.
 *
 *     [ ] Callback contracts parse deterministically.
 *
 *     [ ] Effects remain externally owned.
 *
 *     [ ] Requirements remain externally owned semantically.
 *
 *     [ ] Resources remain externally owned semantically.
 *
 *     [ ] Policies remain externally owned semantically.
 *
 *     [ ] Provenance remains externally owned semantically.
 *
 *     [ ] No machine-size limit exists.
 *
 *     [ ] No hardware identity is required.
 *
 *     [ ] No runtime behavior occurs during parsing.
 *
 *     [ ] Positive tests exist.
 *
 *     [ ] Negative tests exist.
 *
 *     [ ] Boundary tests exist.
 *
 *     [ ] Scalability tests exist.
 *
 *     [ ] Determinism tests exist.
 *
 *     [ ] Compatibility tests exist.
 *
 *     [ ] Foreign calls reach the canonical semantic model.
 *
 *     [ ] Quantum-facing foreign operations preserve the quantum::ir boundary.
 *
 *     [ ] HDL/hardware-facing foreign operations remain target-independent.
 *
 * ============================================================================
 */
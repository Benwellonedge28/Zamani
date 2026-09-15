/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/metaprogramming/specialization.g4
 *
 * Status:
 *     Production parser component
 *
 * Purpose:
 *     Defines source-level SPECIALIZATION REQUEST syntax.
 *
 * ============================================================================
 *
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          v
 *     source AST
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +-----------------------------+
 *          |                             |
 *          v                             v
 *     generic resolution          specialization policy
 *          |                             |
 *          +-------------+---------------+
 *                        |
 *                        v
 *              specialization planning
 *                        |
 *                        v
 *              canonical semantic IR
 *                        |
 *              +---------+---------+
 *              |         |         |
 *              v         v         v
 *          classical   quantum   hardware/
 *             IR       quantum::ir resource IR
 *              |         |         |
 *              +---------+---------+
 *                        |
 *                        v
 *              optimization / lowering /
 *              routing / scheduling /
 *              hardware / runtime
 *
 * This grammar defines SOURCE SYNTAX ONLY.
 *
 * It does NOT perform specialization.
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - explicit source-level specialization requests;
 *   - specialization target references;
 *   - specialization argument syntax;
 *   - ordered specialization arguments;
 *   - named specialization arguments;
 *   - type specialization arguments;
 *   - value/constant specialization arguments;
 *   - specialization policies expressed syntactically;
 *   - specialization request metadata;
 *   - specialization request bodies where the language permits them.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - generic parameter declarations;
 *   - generic type applications;
 *   - generic type definitions;
 *   - generic constraint solving;
 *   - type inference;
 *   - type substitution;
 *   - monomorphization;
 *   - partial evaluation;
 *   - constant evaluation;
 *   - overload resolution;
 *   - trait solving;
 *   - code generation;
 *   - optimization;
 *   - target selection;
 *   - hardware selection;
 *   - resource allocation;
 *   - quantum allocation;
 *   - physical qubit selection;
 *   - routing;
 *   - scheduling;
 *   - QEC;
 *   - ZQN;
 *   - simulation;
 *   - runtime dispatch;
 *   - device discovery;
 *   - compiler resource limits.
 *
 * Those concerns belong to their canonical downstream owners.
 *
 * ============================================================================
 *
 * GENERIC DECLARATION SEPARATION
 * ============================================================================
 *
 * Generic declarations belong to:
 *
 *     grammar/functions/generics.g4
 *
 * and the corresponding declaration/type grammars.
 *
 * This file MUST NOT redefine:
 *
 *     functionGenericParameters
 *     functionGenericParameter
 *     genericParameters
 *     genericParameter
 *
 * ============================================================================
 *
 * GENERIC APPLICATION SEPARATION
 * ============================================================================
 *
 * Generic type application belongs to:
 *
 *     grammar/types/generic-types.g4
 *
 * That grammar owns structures such as:
 *
 *     Vector<T>
 *     Result<T, E>
 *     Matrix<T, Rows, Columns>
 *
 * This file may consume canonical type expressions as specialization
 * arguments, but MUST NOT create another generic type representation.
 *
 * ============================================================================
 *
 * SPECIALIZATION SEMANTICS
 * ============================================================================
 *
 * A specialization request is an INTENT.
 *
 * For example:
 *
 *     specialize compute<int>
 *
 * does NOT mean:
 *
 *     "immediately generate machine code".
 *
 * It means:
 *
 *     "the program explicitly requests specialization of `compute`
 *      under the supplied semantic arguments."
 *
 * The semantic/compiler layer determines whether the request:
 *
 *     - is legal;
 *     - is meaningful;
 *     - is satisfiable;
 *     - is beneficial;
 *     - is deterministic;
 *     - can be performed;
 *     - should be cached;
 *     - should be deferred;
 *     - should be rejected;
 *     - should be lowered differently.
 *
 * ============================================================================
 *
 * POCO-REAF
 * ============================================================================
 *
 * Specialization MUST preserve the portable semantic meaning of the source.
 *
 * A specialization request MUST NOT inherently encode:
 *
 *     - CPU count;
 *     - core count;
 *     - thread count;
 *     - GPU count;
 *     - FPGA count;
 *     - ASIC count;
 *     - QPU count;
 *     - qubit count;
 *     - memory capacity;
 *     - physical topology;
 *     - physical address;
 *     - device identifier;
 *     - vendor;
 *     - calibration;
 *     - timing grid;
 *     - deployment location.
 *
 * Target/resource facts belong to:
 *
 *     resources/
 *     hardware/
 *     compile/
 *     execution/
 *
 * and are resolved downstream.
 *
 * ============================================================================
 *
 * SCALABILITY
 * ============================================================================
 *
 * This grammar imposes NO fixed limits on:
 *
 *     - specialization requests;
 *     - specialization arguments;
 *     - type arguments;
 *     - value arguments;
 *     - nested type expressions;
 *     - nested specialization expressions;
 *     - specialization declarations;
 *     - specialization metadata.
 *
 * There is deliberately no:
 *
 *     MAX_SPECIALIZATIONS
 *     MAX_TYPE_ARGUMENTS
 *     MAX_VALUE_ARGUMENTS
 *     MAX_TARGETS
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_MEMORY
 *
 * Any implementation safety/resource budget belongs to compiler policy.
 *
 * ============================================================================
 *
 * DETERMINISM
 * ============================================================================
 *
 * Source ordering is significant and MUST be preserved.
 *
 * In particular:
 *
 *     specialization target
 *     argument order
 *     named-argument names
 *     argument values
 *
 * must be represented deterministically in the AST.
 *
 * The semantic specialization engine may canonicalize equivalent requests,
 * but such canonicalization does not belong to this grammar.
 *
 * ============================================================================
 *
 * SECURITY
 * ============================================================================
 *
 * Specialization syntax does NOT grant:
 *
 *     filesystem access;
 *     network access;
 *     subprocess execution;
 *     environment-variable access;
 *     compiler-global mutation;
 *     hardware access;
 *     device discovery.
 *
 * Such operations require explicit capabilities/effects and are governed by
 * their respective security/compiler/runtime systems.
 *
 * ============================================================================
 *
 * RUST
 * ============================================================================
 *
 * Generated parser/compiler integration targets:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *     Rust 2021
 *
 * Integration MUST use safe Rust.
 *
 * This grammar contains:
 *
 *     - no Rust actions;
 *     - no embedded code;
 *     - no unsafe;
 *     - no unsafe blocks;
 *     - no host-language execution.
 *
 * ============================================================================
 *
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexer owns all tokens.
 *
 * This grammar therefore consumes tokens and does not declare lexer rules.
 *
 * Required canonical specialization keyword:
 *
 *     K_SPECIALIZE : 'specialize' ;
 *
 * This token is already referenced by quantum specialization grammar and
 * therefore must be established in the authoritative lexer vocabulary.
 *
 * If the repository continues using:
 *
 *     grammar/lexer/tokens.g4
 *
 * as the lexer vocabulary, `K_SPECIALIZE` belongs there.
 *
 * ============================================================================
 *
 * TYPE CONTRACT
 * ============================================================================
 *
 * `typeExpression` is owned by the canonical type grammar.
 *
 * This file consumes it but does not redefine it.
 *
 * ============================================================================
 *
 * EXPRESSION CONTRACT
 * ============================================================================
 *
 * `expression` is owned by the canonical expression grammar.
 *
 * This file consumes it but does not redefine:
 *
 *     literals
 *     arithmetic
 *     calls
 *     indexing
 *     member access
 *     operators
 *     conditionals
 *     lambdas
 *     comprehensions
 *     compile-time expressions.
 *
 * ============================================================================
 *
 * NAME CONTRACT
 * ============================================================================
 *
 * `qualifiedName` is owned by the core/name grammar.
 *
 * This file consumes it for specialization targets.
 *
 * ============================================================================
 *
 * AST CONTRACT
 * ============================================================================
 *
 * The parser MUST lower specialization syntax into the canonical AST
 * representation selected by the frontend.
 *
 * This file does NOT define an AST type.
 *
 * The semantic representation should preserve at minimum:
 *
 *     target
 *     ordered arguments
 *     argument kind
 *     source span
 *     source ordering
 *     optional policy metadata
 *
 * The compiler may subsequently create an internal specialization key.
 *
 * This grammar MUST NOT reproduce the compiler's internal:
 *
 *     SpecializationKey
 *     Specialization
 *     GenericDefinition
 *
 * structures.
 *
 * ============================================================================
 *
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Generic specialization can apply to quantum programs and abstractions.
 *
 * Example:
 *
 *     specialize quantum_algorithm<LogicalQubit>
 *
 * The grammar does not determine:
 *
 *     - number of qubits;
 *     - logical/physical mapping;
 *     - gate set;
 *     - topology;
 *     - calibration;
 *     - noise;
 *     - QEC;
 *     - routing;
 *     - scheduling.
 *
 * Those remain downstream.
 *
 * `quantum::ir` remains the canonical quantum semantic boundary.
 *
 * ============================================================================
 *
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Specialization may apply to parameterized hardware/HDL abstractions.
 *
 * Example:
 *
 *     specialize datapath<WordType> with (width = desired_width)
 *
 * This grammar does not decide physical width, placement, synthesis strategy,
 * FPGA resources, ASIC technology, clock implementation, or routing.
 *
 * ============================================================================
 *
 * RESOURCE INTEGRATION
 * ============================================================================
 *
 * A specialization value may refer to a semantic resource expression.
 *
 * It MUST NOT automatically become a physical resource allocation.
 *
 * For example, a value representing:
 *
 *     available_capacity
 *
 * is semantically different from:
 *
 *     physical_device_17
 *
 * The resource/capability grammar and semantic analysis determine the meaning.
 *
 * ============================================================================
 *
 * COMPILER INTEGRATION
 * ============================================================================
 *
 * The expected downstream path is:
 *
 *     specialization request
 *          |
 *          v
 *     semantic validation
 *          |
 *          v
 *     generic resolution
 *          |
 *          v
 *     constraint solving
 *          |
 *          v
 *     specialization planning
 *          |
 *          +--> no specialization
 *          +--> deferred specialization
 *          +--> partial specialization
 *          +--> monomorphization
 *          +--> target-aware specialization
 *          |
 *          v
 *     canonical IR
 *
 * The existing compiler monomorphization machinery already uses explicit
 * configurable limits and deterministic specialization records. Those limits
 * remain compiler policy and must not be moved into this grammar.
 *
 * ============================================================================
 *
 * COMPATIBILITY
 * ============================================================================
 *
 * Adding specialization syntax is a language-versioned operation.
 *
 * The following must remain stable:
 *
 *     K_SPECIALIZE
 *     specializationRequest
 *     specializationTarget
 *     specializationArgument
 *
 * Rule renaming requires a coordinated parser/AST/test update.
 *
 * ============================================================================
 */

parser grammar Specialization;

options {
    tokenVocab = ZamaniTokens;
}


/* ============================================================================
 * 1. PUBLIC SPECIALIZATION ENTRY POINT
 * ============================================================================
 *
 * The canonical metaprogramming orchestrator should expose:
 *
 *     specializationConstruct
 *
 * where source-level specialization is permitted.
 *
 * This grammar deliberately does not make specialization an ordinary
 * expression. It is an explicit compile-time/source transformation request.
 */

specializationConstruct
    : specializationRequest
    ;


/* ============================================================================
 * 2. SPECIALIZATION REQUEST
 * ============================================================================
 *
 * Canonical form:
 *
 *     specialize target
 *
 *     specialize target<type>
 *
 *     specialize target<type1, type2>
 *
 *     specialize target with (name = value)
 *
 *     specialize target<type> with (name = value)
 *
 * The request itself does not guarantee that specialization occurs.
 */

specializationRequest
    : K_SPECIALIZE
      specializationTarget
      specializationTypeArguments?
      specializationValueArguments?
      specializationPolicyClause*
      SEMI?
    ;


/* ============================================================================
 * 3. SPECIALIZATION TARGET
 * ============================================================================
 *
 * A target is a canonical source-level qualified name.
 *
 * This permits specialization of:
 *
 *     functions
 *     types
 *     modules
 *     circuits
 *     hardware abstractions
 *     data transformations
 *     accelerator abstractions
 *     domain-defined generic entities
 *
 * Semantic analysis determines whether the referenced entity is actually
 * specializable.
 */

specializationTarget
    : qualifiedName
    ;


/* ============================================================================
 * 4. TYPE SPECIALIZATION ARGUMENTS
 * ============================================================================
 *
 * Angle brackets are used explicitly for type specialization arguments.
 *
 * Example:
 *
 *     specialize compute<int>
 *
 *     specialize compute<Matrix<float>>
 *
 *     specialize quantum_algorithm<LogicalQubit>
 *
 * Type syntax itself belongs to the canonical type grammar.
 */

specializationTypeArguments
    : LESS_THAN
      specializationTypeArgument
      (
          COMMA
          specializationTypeArgument
      )*
      COMMA?
      GREATER_THAN
    ;


specializationTypeArgument
    : typeExpression
    ;


/* ============================================================================
 * 5. VALUE SPECIALIZATION ARGUMENTS
 * ============================================================================
 *
 * Value specialization is deliberately introduced with `with`.
 *
 * This avoids confusing:
 *
 *     type arguments
 *
 * with:
 *
 *     compile-time value arguments.
 *
 * Examples:
 *
 *     specialize matrix with (rows = rows, columns = columns)
 *
 *     specialize kernel<T> with (tile = tile_size)
 *
 *     specialize circuit<LogicalQubit> with (precision = precision)
 *
 * The values are ordinary Zamani expressions.
 *
 * The semantic layer decides whether an expression is compile-time
 * evaluable and suitable for specialization.
 */

specializationValueArguments
    : K_WITH
      LPAREN
      specializationValueArgumentList?
      RPAREN
    ;


specializationValueArgumentList
    : specializationValueArgument
      (
          COMMA
          specializationValueArgument
      )*
      COMMA?
    ;


specializationValueArgument
    : specializationNamedValueArgument
    | specializationPositionalValueArgument
    ;


/* ============================================================================
 * 6. NAMED SPECIALIZATION VALUE
 * ============================================================================
 *
 * Example:
 *
 *     with (width = width)
 *
 *     with (precision = 64)
 *
 * The left side is a source-level specialization parameter name.
 *
 * It does not itself allocate hardware or memory.
 */

specializationNamedValueArgument
    : identifier
      ASSIGN
      expression
    ;


/* ============================================================================
 * 7. POSITIONAL SPECIALIZATION VALUE
 * ============================================================================
 *
 * Example:
 *
 *     with (value)
 *
 * Positional semantics are resolved against the semantic specialization
 * parameter declaration.
 */

specializationPositionalValueArgument
    : expression
    ;


/* ============================================================================
 * 8. SPECIALIZATION POLICY
 * ============================================================================
 *
 * A specialization request may optionally carry a semantic policy.
 *
 * Policy is deliberately represented as a named semantic option rather than
 * hard-coded compiler behavior.
 *
 * Example:
 *
 *     specialize compute<T> with (value = n) as preferred
 *
 * The exact interpretation belongs to semantic/compiler policy.
 *
 * This grammar therefore allows an extensible policy name/value pair.
 */

specializationPolicyClause
    : K_AS
      specializationPolicy
    ;


specializationPolicy
    : identifier
    | qualifiedName
    ;


/* ============================================================================
 * 9. EXPLICIT SPECIALIZATION DECLARATION
 * ============================================================================
 *
 * Some language constructs may need a specialization declaration rather than
 * an immediate request.
 *
 * Example conceptual form:
 *
 *     specialize fn compute<int>
 *
 * This rule is intentionally conservative.
 *
 * It identifies the entity kind while leaving the actual declaration
 * semantics to the corresponding declaration subsystem.
 *
 * No duplicate function/type/module declaration grammar is introduced.
 */

specializationDeclaration
    : K_SPECIALIZE
      specializationDeclarationKind
      specializationTarget
      specializationTypeArguments?
      specializationValueArguments?
      specializationPolicyClause*
      SEMI?
    ;


specializationDeclarationKind
    : K_FN
    | K_TYPE
    | K_MODULE
    | K_CIRCUIT
    ;


/* ============================================================================
 * 10. SPECIALIZATION OF A CANONICAL DECLARATION
 * ============================================================================
 *
 * This rule provides a thin integration boundary for declaration contexts.
 *
 * The canonical declaration grammar remains authoritative.
 */

specializationDeclarationReference
    : specializationTarget
    ;


/* ============================================================================
 * 11. SPECIALIZATION ARGUMENT
 * ============================================================================
 *
 * Unified internal argument boundary.
 *
 * This rule is intentionally not exposed as a second generic-argument AST.
 */

specializationArgument
    : specializationTypeArgument
    | specializationValueArgument
    ;


/* ============================================================================
 * 12. SPECIALIZATION ARGUMENT LIST
 * ============================================================================
 *
 * Used by tooling and semantic composition where the surrounding construct
 * already establishes the argument category.
 *
 * There is no fixed argument count.
 */

specializationArgumentList
    : specializationArgument
      (
          COMMA
          specializationArgument
      )*
      COMMA?
    ;


/* ============================================================================
 * 13. SPECIALIZATION CONDITION
 * ============================================================================
 *
 * A specialization may be semantically conditional when embedded by a
 * compilation-control construct.
 *
 * This grammar only provides the expression boundary.
 */

specializationCondition
    : expression
    ;


/* ============================================================================
 * 14. SPECIALIZATION TARGET REFERENCE
 * ============================================================================
 *
 * Alias for composition with tools/semantic layers that need an explicitly
 * named target-reference rule.
 *
 * It delegates completely to the canonical name grammar.
 */

specializationTargetReference
    : qualifiedName
    ;


/* ============================================================================
 * 15. SPECIALIZATION METADATA
 * ============================================================================
 *
 * Metadata belongs to the canonical metadata/attribute system.
 *
 * This rule intentionally delegates rather than defining another annotation
 * language.
 */

specializationMetadata
    : attribute
    ;


/* ============================================================================
 * 16. SPECIALIZATION METADATA LIST
 * ============================================================================
 */

specializationMetadataList
    : specializationMetadata*
    ;


/* ============================================================================
 * 17. SPECIALIZATION REQUEST WITH METADATA
 * ============================================================================
 *
 * Thin composition rule for contexts where metadata is permitted.
 */

specializationRequestWithMetadata
    : specializationMetadataList
      specializationRequest
    ;


/* ============================================================================
 * 18. SPECIALIZATION BLOCK
 * ============================================================================
 *
 * A block is deliberately not interpreted by this grammar.
 *
 * If a future language construct requires a block associated with a
 * specialization request, it must use the canonical block grammar.
 *
 * This rule exists as an integration boundary only.
 */

specializationBlock
    : block
    ;


/* ============================================================================
 * 19. SPECIALIZATION REQUEST WITH BODY
 * ============================================================================
 *
 * A body is optional and is semantically interpreted downstream.
 *
 * This is useful for future declaration-level specialization forms while
 * preserving a single specialization request model.
 */

specializationRequestWithBody
    : specializationRequest
      specializationBlock?
    ;


/* ============================================================================
 * 20. TYPE-ONLY SPECIALIZATION
 * ============================================================================
 *
 * Explicit type-only form.
 *
 * Example:
 *
 *     specialize compute<int>
 *
 * This is useful to semantic analysis because it establishes that the angle
 * bracket portion is type specialization rather than an ordinary expression.
 */

typeSpecializationRequest
    : K_SPECIALIZE
      specializationTarget
      specializationTypeArguments
      specializationPolicyClause*
      SEMI?
    ;


/* ============================================================================
 * 21. VALUE-ONLY SPECIALIZATION
 * ============================================================================
 *
 * Explicit value specialization.
 *
 * Example:
 *
 *     specialize compute with (precision = precision)
 */

valueSpecializationRequest
    : K_SPECIALIZE
      specializationTarget
      specializationValueArguments
      specializationPolicyClause*
      SEMI?
    ;


/* ============================================================================
 * 22. MIXED SPECIALIZATION
 * ============================================================================
 *
 * Example:
 *
 *     specialize compute<int> with (precision = precision)
 */

mixedSpecializationRequest
    : K_SPECIALIZE
      specializationTarget
      specializationTypeArguments
      specializationValueArguments
      specializationPolicyClause*
      SEMI?
    ;


/* ============================================================================
 * 23. SPECIALIZATION DISPATCH
 * ============================================================================
 *
 * Canonical semantic dispatch point.
 *
 * The alternatives are intentionally explicit and non-recursive.
 */

specializationRequestForm
    : typeSpecializationRequest
    | valueSpecializationRequest
    | mixedSpecializationRequest
    | specializationRequest
    ;


/* ============================================================================
 * 24. SEMANTICALLY TRANSPARENT EXPRESSION BRIDGE
 * ============================================================================
 *
 * Specialization arguments may use all ordinary Zamani expression forms.
 *
 * The expression grammar remains authoritative.
 */

specializationExpression
    : expression
    ;


/* ============================================================================
 * 25. SEMANTICALLY TRANSPARENT TYPE BRIDGE
 * ============================================================================
 *
 * Type expressions remain owned by the canonical type system.
 */

specializationType
    : typeExpression
    ;


/* ============================================================================
 * 26. SPECIALIZATION VALUE BRIDGE
 * ============================================================================
 */

specializationValue
    : expression
    ;


/* ============================================================================
 * 27. SPECIALIZATION CONSTRAINT BRIDGE
 * ============================================================================
 *
 * Constraints are semantic expressions here.
 *
 * The resource/constraint systems determine their meaning.
 */

specializationConstraint
    : expression
    ;


/* ============================================================================
 * 28. SPECIALIZATION CAPABILITY BRIDGE
 * ============================================================================
 *
 * No hardware capability is defined in this grammar.
 */

specializationCapability
    : expression
    ;


/* ============================================================================
 * 29. SPECIALIZATION RESOURCE BRIDGE
 * ============================================================================
 *
 * No physical resource is allocated here.
 */

specializationResource
    : expression
    ;


/* ============================================================================
 * 30. SPECIALIZATION POLICY VALUE
 * ============================================================================
 *
 * Policy values are expressions.
 */

specializationPolicyValue
    : expression
    ;


/* ============================================================================
 * 31. NAMED POLICY VALUE
 * ============================================================================
 *
 * Example conceptual form:
 *
 *     specialize compute<int> as preferred
 *
 * or, where the surrounding future policy grammar permits:
 *
 *     policy = expression
 *
 * No implementation-specific policy names are hard-coded here.
 */

specializationNamedPolicyValue
    : identifier
      ASSIGN
      expression
    ;


/* ============================================================================
 * 32. SPECIALIZATION POLICY ARGUMENTS
 * ============================================================================
 */

specializationPolicyArguments
    : LPAREN
      specializationNamedPolicyValueList?
      RPAREN
    ;


specializationNamedPolicyValueList
    : specializationNamedPolicyValue
      (
          COMMA
          specializationNamedPolicyValue
      )*
      COMMA?
    ;


/* ============================================================================
 * 33. SPECIALIZATION POLICY INVOCATION
 * ============================================================================
 *
 * Extensible policy syntax.
 */

specializationPolicyInvocation
    : K_AS
      specializationPolicy
      specializationPolicyArguments?
    ;


/* ============================================================================
 * 34. COMPLETE SPECIALIZATION REQUEST
 * ============================================================================
 *
 * This is the recommended public composition rule for the metaprogramming
 * orchestrator.
 *
 * It intentionally keeps specialization explicit and bounded by source
 * structure rather than machine properties.
 */

completeSpecializationRequest
    : K_SPECIALIZE
      specializationTarget
      specializationTypeArguments?
      specializationValueArguments?
      specializationPolicyInvocation*
      SEMI?
    ;


/* ============================================================================
 * 35. SPECIALIZATION DECLARATION FORM
 * ============================================================================
 *
 * Declaration-level specialization is kept separate from an ordinary request.
 *
 * This prevents the parser from confusing:
 *
 *     "request specialization"
 *
 * with:
 *
 *     "declare a new generic entity".
 */

completeSpecializationDeclaration
    : K_SPECIALIZE
      specializationDeclarationKind
      specializationTarget
      specializationTypeArguments?
      specializationValueArguments?
      specializationPolicyInvocation*
      SEMI?
    ;


/* ============================================================================
 * 36. COMPOSITION CONTRACT
 * ============================================================================
 *
 * The authoritative metaprogramming aggregator should delegate:
 *
 *     specializationConstruct
 *
 * to this grammar.
 *
 * It should NOT redefine:
 *
 *     specializationRequest
 *     specializationTarget
 *     specializationTypeArguments
 *     specializationValueArguments
 *
 * ============================================================================
 *
 * FUNCTIONS INTEGRATION
 * ============================================================================
 *
 * Function generic declarations remain owned by:
 *
 *     grammar/functions/generics.g4
 *
 * This file only refers to the resulting target and specialization arguments.
 *
 * ============================================================================
 *
 * TYPES INTEGRATION
 * ============================================================================
 *
 * Generic type applications remain owned by:
 *
 *     grammar/types/generic-types.g4
 *
 * `typeExpression` remains canonical.
 *
 * ============================================================================
 *
 * EXPRESSIONS INTEGRATION
 * ============================================================================
 *
 * `expression` remains canonical.
 *
 * Compile-time expressions remain owned by:
 *
 *     grammar/expressions/compile-time.g4
 *
 * This file does not reproduce them.
 *
 * ============================================================================
 *
 * COMPILE INTEGRATION
 * ============================================================================
 *
 * Compilation-time control remains owned by:
 *
 *     grammar/compile/compile-time.g4
 *
 * That grammar may contain a specialization control construct which delegates
 * to this grammar rather than reproducing specialization syntax.
 *
 * ============================================================================
 *
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Quantum operation specialization remains owned by:
 *
 *     grammar/quantum/parameterized-operations.g4
 *
 * where specialization is specifically part of quantum operation syntax.
 *
 * This file owns domain-neutral specialization requests.
 *
 * Quantum specialization MUST eventually lower through semantic analysis into
 * the canonical `quantum::ir` boundary.
 *
 * ============================================================================
 *
 * HDL INTEGRATION
 * ============================================================================
 *
 * HDL generic/parameter declarations and HDL-specific parameter overrides
 * remain owned by their respective HDL grammars.
 *
 * This file does not redefine HDL parameter syntax.
 *
 * ============================================================================
 *
 * HARDWARE INTEGRATION
 * ============================================================================
 *
 * Hardware selection, capabilities, resources, placement and targets remain
 * outside this grammar.
 *
 * A specialization may depend on semantic hardware/resource information, but
 * that information is represented through canonical resource/capability
 * expressions rather than hard-coded device properties.
 *
 * ============================================================================
 *
 * DISTRIBUTED / AI / DATA INTEGRATION
 * ============================================================================
 *
 * Specialization is domain-neutral.
 *
 * Therefore the same syntax can specialize:
 *
 *     distributed algorithms
 *     AI models
 *     tensor operations
 *     data transformations
 *     accelerators
 *     classical algorithms
 *     quantum algorithms
 *     hardware abstractions
 *
 * without adding domain-specific specialization grammars here.
 *
 * ============================================================================
 *
 * IR INTEGRATION
 * ============================================================================
 *
 * This grammar does not create IR.
 *
 * The downstream semantic layer transforms specialization requests into
 * canonical semantic operations.
 *
 * Quantum results must enter:
 *
 *     quantum::ir
 *
 * rather than a specialization-specific quantum representation.
 *
 * Classical results must enter the canonical classical representation.
 *
 * Hardware/HDL results must use their existing semantic/IR boundaries.
 *
 * ============================================================================
 *
 * RUNTIME INTEGRATION
 * ============================================================================
 *
 * Runtime MUST NOT parse this grammar as a means of discovering hardware.
 *
 * Specialization normally occurs before runtime execution.
 *
 * Runtime may consume the resulting compiled/semantic representation and
 * runtime-discovered capabilities, but it must not become a dependency of
 * this grammar.
 *
 * ============================================================================
 *
 * TOOLING INTEGRATION
 * ============================================================================
 *
 * Tooling should be able to:
 *
 *     - syntax-highlight `specialize`;
 *     - locate specialization targets;
 *     - inspect specialization arguments;
 *     - preserve source spans;
 *     - format lists deterministically;
 *     - provide semantic diagnostics downstream;
 *     - navigate from a specialization request to its target;
 *     - display unresolved/invalid specialization requests without changing
 *       source semantics.
 *
 * ============================================================================
 *
 * ERROR BOUNDARY
 * ============================================================================
 *
 * Syntax errors belong to ANTLR parsing.
 *
 * Semantic errors include:
 *
 *     unknown specialization target;
 *     non-specializable target;
 *     wrong type-argument arity;
 *     wrong value-argument arity;
 *     duplicate named argument;
 *     unknown named parameter;
 *     incompatible type argument;
 *     unsatisfied generic bound;
 *     non-constant specialization value;
 *     specialization that changes observable semantics;
 *     specialization that violates an explicit semantic constraint.
 *
 * These MUST NOT be encoded as grammar-level limits.
 *
 * ============================================================================
 *
 * HARD-CODING AUDIT
 * ============================================================================
 *
 * This file contains no:
 *
 *     MAX_QUBITS
 *     MAX_CORES
 *     MAX_THREADS
 *     MAX_DEVICES
 *     MAX_NODES
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_MEMORY
 *     MAX_SPECIALIZATIONS
 *     MAX_TYPE_ARGUMENTS
 *     MAX_VALUE_ARGUMENTS
 *     DEVICE_ID
 *     HARDWARE_ADDRESS
 *     TOPOLOGY
 *
 * Any compiler resource protection belongs to explicit compiler configuration.
 *
 * ============================================================================
 *
 * TEST CONTRACT
 * ============================================================================
 *
 * Positive:
 *
 *     specialize compute<int>;
 *     specialize compute<Matrix<float>>;
 *     specialize compute with (width = width);
 *     specialize compute<int> with (width = width);
 *     specialize ns::compute<T> with (precision = precision) as preferred;
 *
 * Negative:
 *
 *     specialize;
 *     specialize <int>;
 *     specialize compute<>;
 *     specialize compute<int,>;
 *
 * Boundary:
 *
 *     deeply nested type expressions;
 *     deeply nested qualified names;
 *     very large specialization argument lists;
 *     very large value expressions;
 *     many independent specialization requests.
 *
 * Semantic-negative:
 *
 *     unknown target;
 *     wrong arity;
 *     duplicate named arguments;
 *     invalid type argument;
 *     invalid constant value;
 *     unsatisfied constraint.
 *
 * Cross-domain:
 *
 *     classical + quantum specialization;
 *     quantum + resource specialization;
 *     HDL + hardware specialization;
 *     AI + accelerator specialization;
 *     distributed + resource specialization;
 *     hybrid quantum/classical specialization.
 *
 * Determinism:
 *
 *     identical source produces identical parse structure and source ordering.
 *
 * Round-trip:
 *
 *     source
 *       -> lexer
 *       -> parser
 *       -> AST
 *       -> formatter
 *       -> parser
 *
 * preserves specialization meaning.
 *
 * ============================================================================
 *
 * COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is complete only when:
 *
 *   1. It composes with the authoritative Zamani parser.
 *
 *   2. It uses the authoritative lexer vocabulary.
 *
 *   3. `K_SPECIALIZE` exists exactly once in the lexer.
 *
 *   4. It does not redefine canonical names.
 *
 *   5. It does not redefine `expression`.
 *
 *   6. It does not redefine `typeExpression`.
 *
 *   7. It does not redefine generic declarations.
 *
 *   8. It does not redefine generic type applications.
 *
 *   9. It does not define an AST.
 *
 *  10. It does not define IR.
 *
 *  11. It does not contain hardware limits.
 *
 *  12. It does not contain quantum limits.
 *
 *  13. It does not perform specialization.
 *
 *  14. It preserves source ordering.
 *
 *  15. It supports arbitrary argument cardinality subject only to parser/
 *      compiler resource policy.
 *
 *  16. It has positive, negative, boundary, cross-domain, determinism and
 *      round-trip coverage.
 *
 *  17. Its integration with the existing monomorphization subsystem is
 *      semantic rather than grammatical.
 *
 *  18. Quantum specialization ultimately reaches `quantum::ir`, not a
 *      specialization-specific quantum IR.
 *
 *  19. Generated Rust integration remains compatible with Rust 1.97/1.97.1.
 *
 *  20. No unsafe Rust is required.
 *
 * ============================================================================
 */
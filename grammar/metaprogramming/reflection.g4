/*
 * ============================================================================
 * Zamani Universal Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/metaprogramming/reflection.g4
 *
 * Grammar:
 *     Reflection
 *
 * Status:
 *     Production parser-grammar component
 *
 * Purpose:
 *     Define the SOURCE-LEVEL SYNTAX for compile-time reflection.
 *
 * Reflection allows Zamani programs and compile-time tooling to request
 * structured information about language entities without introducing a second
 * type system, AST, IR, runtime object model, hardware model, or backend model.
 *
 * ============================================================================
 *
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 *     source
 *       |
 *       v
 *     ZamaniLexer
 *       |
 *       v
 *     authoritative Zamani parser
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     semantic analysis
 *       |
 *       +--> name resolution
 *       +--> type analysis
 *       +--> effect analysis
 *       +--> capability analysis
 *       +--> reflection validation
 *       +--> compile-time evaluation
 *       |
 *       v
 *     canonical semantic representation
 *       |
 *       +--> classical IR
 *       +--> quantum::ir
 *       +--> HDL / hardware representation
 *       +--> distributed representation
 *       |
 *       v
 *     optimization / lowering / routing / scheduling / HAL
 *       |
 *       v
 *     runtime
 *
 * Reflection MUST NOT bypass this architecture.
 *
 * ============================================================================
 *
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *   - reflection expression syntax;
 *   - reflection subject syntax;
 *   - reflection query syntax;
 *   - reflection member-selection syntax;
 *   - reflection metadata-selection syntax;
 *   - reflection type-selection syntax;
 *   - reflection declaration-selection syntax;
 *   - reflection capability-selection syntax;
 *   - reflection relationship-selection syntax;
 *   - reflection result-shaping syntax;
 *   - syntactic distinction between value reflection and type reflection.
 *
 * THIS FILE DOES NOT OWN:
 *
 *   - ordinary expressions;
 *   - identifiers;
 *   - qualified names;
 *   - type expressions;
 *   - declarations;
 *   - functions;
 *   - generic parameters;
 *   - attributes;
 *   - metadata definitions;
 *   - compile-time function declarations;
 *   - compile-time evaluation;
 *   - macro expansion;
 *   - code generation;
 *   - specialization;
 *   - type checking;
 *   - name resolution;
 *   - capability discovery;
 *   - hardware discovery;
 *   - resource discovery;
 *   - target selection;
 *   - quantum compilation;
 *   - quantum IR;
 *   - classical IR;
 *   - HDL IR;
 *   - optimization;
 *   - routing;
 *   - scheduling;
 *   - QEC;
 *   - ZQN;
 *   - resilience;
 *   - runtime execution.
 *
 * ============================================================================
 *
 * NON-DUPLICATION CONTRACT
 * ============================================================================
 *
 * Reflection MUST consume canonical parser rules for:
 *
 *     identifier
 *     qualifiedName
 *     expression
 *     typeExpression
 *     attribute
 *     genericArguments
 *     literal
 *
 * This file MUST NOT redefine any of them.
 *
 * Reflection metadata is a VIEW over existing language information.
 *
 * It is not a second representation of that information.
 *
 * ============================================================================
 *
 * QUANTUM BOUNDARY
 * ============================================================================
 *
 * Reflection may inspect the semantic description of:
 *
 *     - quantum types;
 *     - logical qubits;
 *     - quantum operations;
 *     - circuits;
 *     - measurements;
 *     - observables;
 *     - quantum declarations;
 *     - quantum capabilities exposed to the semantic layer.
 *
 * It MUST NOT create or modify quantum::ir.
 *
 * Reflection therefore follows:
 *
 *     source
 *       |
 *       v
 *     reflection AST
 *       |
 *       v
 *     semantic reflection model
 *       |
 *       v
 *     existing semantic/IR representation
 *
 * and never:
 *
 *     reflection -> independent quantum IR.
 *
 * ============================================================================
 *
 * HARDWARE / RESOURCE BOUNDARY
 * ============================================================================
 *
 * Reflection MUST NOT turn arbitrary machine state into permanent source
 * semantics.
 *
 * It may request information through explicitly authorized semantic
 * capabilities, but this grammar does not encode:
 *
 *     CPU count
 *     GPU count
 *     FPGA count
 *     QPU count
 *     qubit count
 *     memory size
 *     register count
 *     topology
 *     device identifiers
 *     physical addresses
 *     vendor-specific hardware state
 *
 * Those belong to capability/resource/hardware/runtime subsystems.
 *
 * ============================================================================
 *
 * POCO-REAF
 * ============================================================================
 *
 * Reflection MUST preserve:
 *
 *     Program Once
 *          |
 *          v
 *     stable source semantics
 *          |
 *          v
 *     compile-time reflection
 *          |
 *          v
 *     portable semantic representation
 *          |
 *          v
 *     target adaptation
 *
 * Reflection must not silently make a program depend on the machine used to
 * compile it.
 *
 * If machine-dependent information is intentionally requested, that dependency
 * must be represented by the language's capability/resource/effect model and
 * validated semantically.
 *
 * ============================================================================
 *
 * SECURITY
 * ============================================================================
 *
 * Parsing reflection syntax MUST NOT:
 *
 *     - inspect the host filesystem;
 *     - inspect the network;
 *     - inspect environment variables;
 *     - inspect credentials;
 *     - inspect compiler internals;
 *     - inspect arbitrary memory;
 *     - inspect physical hardware;
 *     - contact a QPU;
 *     - contact a GPU;
 *     - contact an FPGA;
 *     - execute generated code;
 *     - execute arbitrary host code.
 *
 * These are semantic/compiler/runtime decisions and require explicit
 * capabilities where supported.
 *
 * ============================================================================
 *
 * DETERMINISM
 * ============================================================================
 *
 * Reflection syntax itself is deterministic.
 *
 * Whether a particular reflection request is deterministic is a semantic
 * property.
 *
 * Reflection over declarations, types, signatures, attributes, and other
 * language-defined information SHOULD be deterministic.
 *
 * Reflection over dynamic external state MUST be explicitly classified by the
 * semantic/effect/capability system.
 *
 * ============================================================================
 *
 * SCALABILITY
 * ============================================================================
 *
 * No language-level finite limits are imposed on:
 *
 *     - reflection query depth;
 *     - member count;
 *     - declaration count;
 *     - metadata count;
 *     - generic arguments;
 *     - reflected entities;
 *     - reflection chains.
 *
 * Compiler resource budgets may exist, but they are implementation policy and
 * MUST NOT become grammar semantics.
 *
 * ============================================================================
 *
 * RUST CONTRACT
 * ============================================================================
 *
 * Compiler/frontend implementation target:
 *
 *     Rust 1.97
 *     Rust 1.97.1
 *
 * Generated parser integration MUST use safe Rust only.
 *
 * This grammar contains:
 *
 *     - no Rust actions;
 *     - no semantic predicates;
 *     - no unsafe code;
 *     - no filesystem access;
 *     - no network access;
 *     - no runtime execution.
 *
 * ============================================================================
 *
 * LEXER CONTRACT
 * ============================================================================
 *
 * The canonical lexer is:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * and the parser consumes:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * Reflection requires one reserved lexical boundary:
 *
 *     REFLECT : 'reflect' ;
 *
 * This keyword belongs in:
 *
 *     grammar/lexer/keywords.g4
 *
 * and must be imported by the canonical ZamaniLexer.
 *
 * This file intentionally does NOT define REFLECT.
 *
 * Other reflection operations remain parser-level identifiers where possible.
 * This avoids permanently reserving a large collection of library/API names.
 *
 * ============================================================================
 */

parser grammar Reflection;

options {
    tokenVocab = ZamaniLexer;
}


/* ============================================================================
 * 1. CANONICAL REFLECTION EXPRESSION
 * ============================================================================
 *
 * Primary source form:
 *
 *     reflect(value)
 *
 * or:
 *
 *     reflect type(TypeName)
 *
 * The first form reflects a value/expression.
 *
 * The second form explicitly reflects a type.
 *
 * Reflection is an expression-level facility and therefore integrates with
 * the canonical expression grammar.
 *
 * ============================================================================
 */

reflectionExpression
    : reflectionValueExpression
    | reflectionTypeExpression
    ;


/* ============================================================================
 * 2. VALUE REFLECTION
 * ============================================================================
 *
 *     reflect(expression)
 *
 * The expression is owned by the canonical expression grammar.
 *
 * Reflection does not evaluate the expression merely because it appears as a
 * reflection subject. Semantic analysis determines whether the subject is:
 *
 *     - compile-time available;
 *     - a declaration reference;
 *     - a type-bearing value;
 *     - a literal;
 *     - a compile-time object;
 *     - otherwise reflectable.
 *
 * ============================================================================
 */

reflectionValueExpression
    : REFLECT
      LPAREN
      expression
      RPAREN
      reflectionProjection*
    ;


/* ============================================================================
 * 3. TYPE REFLECTION
 * ============================================================================
 *
 * Explicit `type` distinguishes reflection of a type from reflection of a
 * runtime/compile-time value.
 *
 * Example:
 *
 *     reflect type(MyType)
 *
 * Type syntax remains owned by the canonical type grammar.
 *
 * ============================================================================
 */

reflectionTypeExpression
    : REFLECT
      TYPE
      LPAREN
      typeExpression
      RPAREN
      reflectionProjection*
    ;


/* ============================================================================
 * 4. REFLECTION PROJECTIONS
 * ============================================================================
 *
 * A projection asks for a particular view of the reflected entity.
 *
 * The projection vocabulary is deliberately open.
 *
 * This prevents the grammar from becoming a permanently closed enumeration
 * of every future language construct.
 *
 * Semantic analysis owns validation of the requested projection.
 *
 * ============================================================================
 */

reflectionProjection
    : DOT reflectionSelector
    ;


/* ============================================================================
 * 5. REFLECTION SELECTOR
 * ============================================================================
 *
 * Reflection selectors are semantic names rather than independent language
 * constructs.
 *
 * Examples of valid semantic selectors may include:
 *
 *     type
 *     name
 *     members
 *     fields
 *     methods
 *     parameters
 *     returnType
 *     attributes
 *     generics
 *     capabilities
 *     requirements
 *     effects
 *     source
 *     declaration
 *     kind
 *
 * The grammar intentionally does not enumerate them.
 *
 * This permits the reflection system to evolve without continuously changing
 * the lexer.
 *
 * ============================================================================
 */

reflectionSelector
    : identifier
    ;


/* ============================================================================
 * 6. REFLECTION QUERY
 * ============================================================================
 *
 * A query explicitly identifies a reflection subject and one or more requested
 * projections.
 *
 * This rule provides a named AST integration boundary for tooling.
 *
 * ============================================================================
 */

reflectionQuery
    : reflectionExpression
    ;


/* ============================================================================
 * 7. REFLECTION SUBJECT
 * ============================================================================
 *
 * This rule provides a semantic naming boundary without duplicating expression
 * or type syntax.
 *
 * It is intentionally structural.
 *
 * ============================================================================
 */

reflectionSubject
    : reflectionValueSubject
    | reflectionTypeSubject
    ;


reflectionValueSubject
    : expression
    ;


reflectionTypeSubject
    : TYPE
      LPAREN
      typeExpression
      RPAREN
    ;


/* ============================================================================
 * 8. REFLECTION MEMBER REQUEST
 * ============================================================================
 *
 * A member request is represented through ordinary projection syntax:
 *
 *     reflect(value).members
 *
 *     reflect(type(T)).members
 *
 * This grammar does not distinguish fields, methods, ports, signals, gates,
 * quantum operations, etc. at the lexical level.
 *
 * Their semantic categories come from the reflected entity.
 *
 * ============================================================================
 */

reflectionMemberRequest
    : reflectionExpression
      DOT
      identifier
    ;


/* ============================================================================
 * 9. REFLECTION ATTRIBUTE REQUEST
 * ============================================================================
 *
 * Attributes are language metadata.
 *
 * Attribute definitions remain owned by the common attribute system.
 *
 * ============================================================================
 */

reflectionAttributeRequest
    : reflectionExpression
      DOT
      identifier
    ;


/* ============================================================================
 * 10. REFLECTION DECLARATION REQUEST
 * ============================================================================
 *
 * Declaration information is exposed through the same reflection projection
 * model rather than a second declaration language.
 *
 * ============================================================================
 */

reflectionDeclarationRequest
    : reflectionExpression
      DOT
      identifier
    ;


/* ============================================================================
 * 11. REFLECTION TYPE REQUEST
 * ============================================================================
 *
 * A reflected value may expose type information through a projection.
 *
 * Example:
 *
 *     reflect(value).type
 *
 * Semantic validation determines whether the reflected entity has a type.
 *
 * ============================================================================
 */

reflectionTypeRequest
    : reflectionExpression
      DOT
      identifier
    ;


/* ============================================================================
 * 12. REFLECTION MEMBER ENUMERATION
 * ============================================================================
 *
 * Enumeration is represented as a semantic projection rather than a dedicated
 * grammar keyword.
 *
 * Example:
 *
 *     reflect(type(MyType)).members
 *
 * The result is a compile-time reflection value whose actual representation
 * belongs to the reflection semantic model.
 *
 * ============================================================================
 */

reflectionMembersRequest
    : reflectionExpression
      DOT
      identifier
    ;


/* ============================================================================
 * 13. REFLECTION FUNCTION INFORMATION
 * ============================================================================
 *
 * Function metadata may be requested through the reflected entity.
 *
 * Example:
 *
 *     reflect(myFunction).parameters
 *     reflect(myFunction).returnType
 *     reflect(myFunction).effects
 *
 * Function syntax itself remains owned by functions/.
 *
 * ============================================================================
 */

reflectionFunctionRequest
    : reflectionExpression
      DOT
      identifier
    ;


/* ============================================================================
 * 14. REFLECTION GENERIC INFORMATION
 * ============================================================================
 *
 * Generic information is semantic metadata over the canonical generic model.
 *
 * ============================================================================
 */

reflectionGenericRequest
    : reflectionExpression
      DOT
      identifier
    ;


/* ============================================================================
 * 15. REFLECTION EFFECT INFORMATION
 * ============================================================================
 *
 * Effects are owned by grammar/effects/.
 *
 * Reflection only requests a view of them.
 *
 * ============================================================================
 */

reflectionEffectRequest
    : reflectionExpression
      DOT
      identifier
    ;


/* ============================================================================
 * 16. REFLECTION CAPABILITY INFORMATION
 * ============================================================================
 *
 * Capabilities may describe semantic requirements or available capabilities.
 *
 * Reflection MUST NOT itself discover hardware.
 *
 * The semantic layer decides whether capability information is:
 *
 *     - static;
 *     - compile-time known;
 *     - target-derived;
 *     - runtime-derived;
 *     - unavailable.
 *
 * ============================================================================
 */

reflectionCapabilityRequest
    : reflectionExpression
      DOT
      identifier
    ;


/* ============================================================================
 * 17. REFLECTION QUANTUM INFORMATION
 * ============================================================================
 *
 * Quantum entities can be reflected through the same generic mechanism.
 *
 * Examples:
 *
 *     reflect(type(Qubit)).kind
 *     reflect(circuit).members
 *     reflect(operation).attributes
 *
 * No quantum-specific reflection syntax is introduced here.
 *
 * This is deliberate:
 *
 *     quantum grammar -> canonical semantic model
 *                         ^
 *                         |
 *                   reflection view
 *
 * not:
 *
 *     quantum grammar -> reflection-specific quantum model.
 *
 * ============================================================================
 */

reflectionQuantumRequest
    : reflectionExpression
      DOT
      identifier
    ;


/* ============================================================================
 * 18. REFLECTION HDL / HARDWARE INFORMATION
 * ============================================================================
 *
 * Hardware and HDL entities may be reflected only through their semantic
 * representation.
 *
 * Reflection does not expose physical addresses or silently query hardware.
 *
 * ============================================================================
 */

reflectionHardwareRequest
    : reflectionExpression
      DOT
      identifier
    ;


/* ============================================================================
 * 19. REFLECTION DISTRIBUTED INFORMATION
 * ============================================================================
 *
 * Distributed entities may expose semantic metadata through reflection.
 *
 * Runtime cluster state is NOT implicitly available.
 *
 * ============================================================================
 */

reflectionDistributedRequest
    : reflectionExpression
      DOT
      identifier
    ;


/* ============================================================================
 * 20. REFLECTION RESULT COMPOSITION
 * ============================================================================
 *
 * Reflection results can participate in ordinary expressions.
 *
 * No separate reflection expression language is introduced.
 *
 * ============================================================================
 */

reflectionResultExpression
    : reflectionExpression
    ;


/* ============================================================================
 * 21. REFLECTION IN COMPILE-TIME FUNCTIONS
 * ============================================================================
 *
 * Compile-time functions remain owned by:
 *
 *     grammar/functions/compile-time-functions.g4
 *
 * Reflection may be used inside compile-time functions through the canonical
 * expression grammar.
 *
 * This file does not duplicate compile-time function declarations.
 *
 * ============================================================================
 */

reflectionCompileTimeUse
    : reflectionExpression
    ;


/* ============================================================================
 * 22. REFLECTION IN MACROS
 * ============================================================================
 *
 * Macro declarations and expansion remain owned by:
 *
 *     grammar/macros/
 *     grammar/metaprogramming/metaprogramming.g4
 *
 * Reflection may supply semantic information to macro processing, but reflection
 * itself does not perform macro expansion.
 *
 * ============================================================================
 */

reflectionMacroUse
    : reflectionExpression
    ;


/* ============================================================================
 * 23. REFLECTION IN GENERATION
 * ============================================================================
 *
 * Code generation remains owned by:
 *
 *     grammar/metaprogramming/generation.g4
 *
 * Reflection can provide input information to generation.
 *
 * It does not generate source by itself.
 *
 * ============================================================================
 */

reflectionGenerationUse
    : reflectionExpression
    ;


/* ============================================================================
 * 24. REFLECTION IN SPECIALIZATION
 * ============================================================================
 *
 * Specialization remains owned by:
 *
 *     grammar/metaprogramming/specialization.g4
 *
 * Reflection can inspect generic/type information used by specialization.
 *
 * It does not perform specialization.
 *
 * ============================================================================
 */

reflectionSpecializationUse
    : reflectionExpression
    ;


/* ============================================================================
 * 25. REFLECTION METADATA
 * ============================================================================
 *
 * Reflection can inspect canonical attributes and metadata.
 *
 * Metadata definitions remain owned by:
 *
 *     grammar/core/metadata.g4
 *     grammar/core/attributes.g4
 *
 * This grammar only provides the syntactic request.
 *
 * ============================================================================
 */

reflectionMetadataRequest
    : reflectionExpression
      DOT
      identifier
    ;


/* ============================================================================
 * 26. REFLECTION RELATIONSHIP
 * ============================================================================
 *
 * Relationship information can include semantic relationships such as:
 *
 *     implements
 *     extends
 *     uses
 *     dependsOn
 *     references
 *     specializes
 *     instantiates
 *
 * The actual relationship vocabulary remains semantic and extensible.
 *
 * ============================================================================
 */

reflectionRelationshipRequest
    : reflectionExpression
      DOT
      identifier
    ;


/* ============================================================================
 * 27. REFLECTION PATH
 * ============================================================================
 *
 * Reflection can be chained:
 *
 *     reflect(type(MyType)).members
 *                           .attributes
 *
 * or:
 *
 *     reflect(value).type.members
 *
 * The parser represents the chain structurally.
 *
 * Semantic analysis validates every transition.
 *
 * ============================================================================
 */

reflectionPath
    : reflectionExpression
      reflectionProjection*
    ;


/* ============================================================================
 * 28. REFLECTION QUERY ARGUMENTS
 * ============================================================================
 *
 * Future reflection APIs may accept semantic query arguments.
 *
 * Arguments use the canonical expression grammar.
 *
 * ============================================================================
 */

reflectionArgumentList
    : LPAREN
      reflectionArgument*
      RPAREN
    ;


reflectionArgument
    : expression
    ;


reflectionArgumentListNonEmpty
    : LPAREN
      expression
      (COMMA expression)*
      COMMA?
      RPAREN
    ;


/* ============================================================================
 * 29. REFLECTION FILTER
 * ============================================================================
 *
 * Filtering is intentionally represented as a semantic call/projection rather
 * than a new query language.
 *
 * For example, a future reflection library can expose:
 *
 *     reflect(type(T)).members.filter(predicate)
 *
 * The grammar therefore does not hard-code a reflection-specific filter DSL.
 *
 * ============================================================================
 */

reflectionFilterUse
    : reflectionExpression
    ;


/* ============================================================================
 * 30. REFLECTION SOURCE INFORMATION
 * ============================================================================
 *
 * Source-location/source-text reflection is semantic metadata.
 *
 * It MUST respect compiler provenance and privacy/security policy.
 *
 * This grammar only permits the normal reflection projection form.
 *
 * ============================================================================
 */

reflectionSourceRequest
    : reflectionExpression
      DOT
      identifier
    ;


/* ============================================================================
 * 31. REFLECTION DECLARATION KIND
 * ============================================================================
 *
 * Declaration kind is represented as a projection.
 *
 * Examples:
 *
 *     reflect(value).kind
 *     reflect(type(T)).kind
 *
 * The semantic model determines the actual classification.
 *
 * ============================================================================
 */

reflectionKindRequest
    : reflectionExpression
      DOT
      identifier
    ;


/* ============================================================================
 * 32. REFLECTION AVAILABILITY
 * ============================================================================
 *
 * Reflection may encounter information that is not available at compile time.
 *
 * The grammar does not encode availability as a boolean assumption.
 *
 * Semantic analysis must distinguish:
 *
 *     known
 *     unknown
 *     unavailable
 *     target-dependent
 *     runtime-dependent
 *     capability-dependent
 *
 * This is essential for POCO-REAF.
 *
 * ============================================================================
 */

reflectionAvailabilityUse
    : reflectionExpression
    ;


/* ============================================================================
 * 33. REFLECTION SAFETY BOUNDARY
 * ============================================================================
 *
 * This grammar deliberately contains no:
 *
 *     hostFile(...)
 *     readMemory(...)
 *     inspectDevice(...)
 *     inspectProcess(...)
 *     inspectNetwork(...)
 *
 * Reflection is not an unrestricted introspection API.
 *
 * Any future external-state reflection must pass through the capability/effect
 * architecture and receive an explicit language-level semantic contract.
 *
 * ============================================================================
 */

reflectionSafeUse
    : reflectionExpression
    ;


/* ============================================================================
 * 34. REFLECTION FAILURE REPRESENTATION
 * ============================================================================
 *
 * Reflection failure is NOT represented by malformed source or parser
 * recovery.
 *
 * Semantic/compiler layers must distinguish at least:
 *
 *     invalid subject
 *     unknown entity
 *     inaccessible entity
 *     unavailable metadata
 *     unsupported reflection operation
 *     capability denied
 *     phase violation
 *     non-deterministic reflection
 *     target-dependent reflection
 *
 * The grammar intentionally has no error-token alternatives for these cases.
 *
 * ============================================================================
 */


/* ============================================================================
 * 35. AST CONTRACT
 * ============================================================================
 *
 * The parser should produce reflection-specific AST nodes equivalent in
 * semantic structure to:
 *
 *     ReflectionExpr {
 *         subject: ReflectionSubject,
 *         projections: [ReflectionProjection],
 *         span: SourceSpan
 *     }
 *
 * The exact Rust AST type is NOT defined by this grammar.
 *
 * The AST must preserve:
 *
 *     - source span;
 *     - subject kind;
 *     - canonical expression/type reference;
 *     - projection order;
 *     - projection names;
 *     - argument expressions where present.
 *
 * The AST must NOT contain:
 *
 *     - hardware handles;
 *     - device IDs;
 *     - runtime pointers;
 *     - mutable compiler state;
 *     - quantum::ir nodes;
 *     - scheduler state;
 *     - backend objects.
 *
 * ============================================================================
 */


/* ============================================================================
 * 36. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must determine:
 *
 *     1. What entity is being reflected?
 *
 *     2. Whether that entity exists.
 *
 *     3. Whether it is visible.
 *
 *     4. Whether reflection is permitted in the current compilation phase.
 *
 *     5. Whether the requested projection is valid.
 *
 *     6. Whether the result is compile-time known.
 *
 *     7. Whether the result is deterministic.
 *
 *     8. Whether capabilities are required.
 *
 *     9. Whether effects are permitted.
 *
 *    10. Whether the result is portable under POCO-REAF.
 *
 *    11. Whether the result can participate in the requested expression.
 *
 *    12. Whether reflection would expose forbidden implementation details.
 *
 * ============================================================================
 */


/* ============================================================================
 * 37. TYPE-SYSTEM CONTRACT
 * ============================================================================
 *
 * Reflection MUST NOT create a second type system.
 *
 * A reflected type refers back to the canonical type system.
 *
 * Therefore:
 *
 *     reflection -> type information
 *
 * and not:
 *
 *     reflection -> independent reflection type universe.
 *
 * A semantic reflection value may have a compiler-defined reflection type,
 * but that type must be integrated with the canonical type system.
 *
 * ============================================================================
 */


/* ============================================================================
 * 38. IR CONTRACT
 * ============================================================================
 *
 * Reflection syntax MUST NOT directly create:
 *
 *     classical IR
 *     quantum::ir
 *     HDL IR
 *     scheduling IR
 *     routing IR
 *
 * Instead:
 *
 *     reflection syntax
 *          |
 *          v
 *     reflection AST
 *          |
 *          v
 *     semantic reflection operation
 *          |
 *          v
 *     canonical semantic representation
 *
 * If reflection is compile-time-only, it may disappear before target lowering.
 *
 * If reflection explicitly produces program data that survives compilation,
 * that data must be represented by the ordinary canonical IR/type system.
 *
 * ============================================================================
 */


/* ============================================================================
 * 39. QUANTUM IR CONTRACT
 * ============================================================================
 *
 * Reflection may inspect quantum semantic entities.
 *
 * It MUST NOT:
 *
 *     - construct gates;
 *     - allocate qubits;
 *     - assign physical qubits;
 *     - select a backend;
 *     - schedule operations;
 *     - route circuits;
 *     - invoke QEC;
 *     - invoke ZQN;
 *     - mutate quantum::ir.
 *
 * quantum::ir remains the canonical quantum semantic boundary.
 *
 * ============================================================================
 */


/* ============================================================================
 * 40. HARDWARE / RESOURCE CONTRACT
 * ============================================================================
 *
 * Reflection of hardware/resource information must distinguish:
 *
 *     program requirement
 *     capability
 *     target
 *     resource
 *     runtime state
 *
 * These concepts MUST NOT be collapsed into a generic reflection result.
 *
 * In particular:
 *
 *     reflect(type(Qubit))
 *
 * is not equivalent to:
 *
 *     discover physical QPU state.
 *
 * ============================================================================
 */


/* ============================================================================
 * 41. RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime reflection is not implied by this grammar.
 *
 * If Zamani eventually supports runtime reflection, it must have an explicit
 * runtime semantic contract and effect/capability classification.
 *
 * This file therefore does not introduce runtime host introspection.
 *
 * ============================================================================
 */


/* ============================================================================
 * 42. TOOLING CONTRACT
 * ============================================================================
 *
 * IDEs, formatters, language servers, documentation tools, analyzers, and
 * refactoring tools may use reflection AST nodes.
 *
 * Tooling must be able to:
 *
 *     - preserve source spans;
 *     - resolve reflection subjects;
 *     - display unresolved projections;
 *     - distinguish compile-time from runtime information;
 *     - avoid executing reflection during parsing.
 *
 * ============================================================================
 */


/* ============================================================================
 * 43. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * `reflect` is a reserved lexical boundary.
 *
 * Once introduced, REFLECT token spelling/name becomes part of the language
 * compatibility contract.
 *
 * Do not later reuse REFLECT for another unrelated construct.
 *
 * Reflection projection names remain identifiers so that future reflection
 * facilities can grow without continuously expanding the reserved keyword set.
 *
 * ============================================================================
 */


/* ============================================================================
 * 44. SCALABILITY CONTRACT
 * ============================================================================
 *
 * There are intentionally no rules such as:
 *
 *     MAX_REFLECTION_DEPTH
 *     MAX_MEMBERS
 *     MAX_TYPES
 *     MAX_DECLARATIONS
 *     MAX_FIELDS
 *     MAX_METHODS
 *     MAX_ATTRIBUTES
 *     MAX_GENERIC_PARAMETERS
 *
 * Resource budgets belong to the compiler/execution environment.
 *
 * The language grammar remains capable of representing arbitrarily large
 * reflection structures subject to available resources.
 *
 * ============================================================================
 */


/* ============================================================================
 * 45. DETERMINISM CONTRACT
 * ============================================================================
 *
 * Parsing the same source with the same lexical specification must produce the
 * same reflection parse structure.
 *
 * Reflection result determinism is a semantic concern.
 *
 * A compiler MUST NOT silently inject host-specific information into a
 * supposedly portable reflection result.
 *
 * ============================================================================
 */


/* ============================================================================
 * 46. COMPOSITION CONTRACT
 * ============================================================================
 *
 * The authoritative Zamani parser should integrate:
 *
 *     reflectionExpression
 *
 * at the canonical expression-extension point.
 *
 * It MUST NOT integrate every helper rule separately into the expression
 * precedence hierarchy.
 *
 * The intended dependency is:
 *
 *     expressions/expressions.g4
 *             |
 *             +--> reflectionExpression
 *                         |
 *                         +--> reflectionValueExpression
 *                         |
 *                         +--> reflectionTypeExpression
 *
 * ============================================================================
 */


/* ============================================================================
 * 47. REQUIRED LEXER INTEGRATION
 * ============================================================================
 *
 * grammar/lexer/keywords.g4
 *
 * MUST eventually contain:
 *
 *     REFLECT : 'reflect' ;
 *
 * in its metaprogramming/language keyword section.
 *
 * grammar/antlr/ZamaniLexer.g4
 *
 * MUST import the keyword vocabulary.
 *
 * No reflection keyword should be added directly to this parser grammar.
 *
 * ============================================================================
 */


/* ============================================================================
 * 48. REQUIRED METAPROGRAMMING INTEGRATION
 * ============================================================================
 *
 * grammar/metaprogramming/metaprogramming.g4
 *
 * MUST treat reflectionExpression as the canonical reflection expression
 * boundary rather than defining another reflection grammar.
 *
 * Existing generic references such as:
 *
 *     reflection requests
 *
 * should resolve to this file.
 *
 * ============================================================================
 */


/* ============================================================================
 * 49. REQUIRED COMPILE-TIME FUNCTION INTEGRATION
 * ============================================================================
 *
 * grammar/functions/compile-time-functions.g4
 *
 * MUST NOT define reflection syntax.
 *
 * Compile-time functions may consume reflection through the canonical
 * expression grammar.
 *
 * This avoids:
 *
 *     functions -> duplicate reflection
 *
 * and establishes:
 *
 *     functions -> expressions -> reflection.
 *
 * ============================================================================
 */


/* ============================================================================
 * 50. REQUIRED GENERATION INTEGRATION
 * ============================================================================
 *
 * grammar/metaprogramming/generation.g4
 *
 * may consume reflection results as semantic input.
 *
 * It MUST NOT redefine reflection subjects or projections.
 *
 * ============================================================================
 */


/* ============================================================================
 * 51. REQUIRED SPECIALIZATION INTEGRATION
 * ============================================================================
 *
 * grammar/metaprogramming/specialization.g4
 *
 * may consume reflected generic/type information.
 *
 * Specialization remains the specialization subsystem's responsibility.
 *
 * ============================================================================
 */


/* ============================================================================
 * 52. REQUIRED MACRO INTEGRATION
 * ============================================================================
 *
 * grammar/macros/
 *
 * may consume reflection results during macro processing where the semantic
 * model permits it.
 *
 * Reflection does not expand macros.
 *
 * ============================================================================
 */


/* ============================================================================
 * 53. REQUIRED VALIDATION
 * ============================================================================
 *
 * Validation must reject semantic misuse including:
 *
 *     - reflection of inaccessible entities;
 *     - invalid projection;
 *     - runtime-only subject in compile-time-only context;
 *     - capability-required reflection without capability;
 *     - prohibited host introspection;
 *     - non-portable reflection silently used as portable computation;
 *     - mutation through reflection;
 *     - construction of compiler-internal objects;
 *     - attempts to obtain physical hardware state through ordinary reflection.
 *
 * ============================================================================
 */


/* ============================================================================
 * 54. TEST CONTRACT
 * ============================================================================
 *
 * Positive tests MUST include:
 *
 *     reflect(value)
 *     reflect(type(MyType))
 *     reflect(value).type
 *     reflect(type(MyType)).members
 *     reflect(type(MyType)).attributes
 *     reflect(function).parameters
 *     reflect(function).returnType
 *     reflect(function).effects
 *     reflect(type(Qubit)).kind
 *     reflect(type(HardwareModule)).members
 *
 * Negative tests MUST include:
 *
 *     malformed reflection calls
 *     missing closing parenthesis
 *     missing subject
 *     missing projection
 *     invalid reflection placement
 *
 * Semantic-negative tests MUST include:
 *
 *     inaccessible entity
 *     nonexistent entity
 *     invalid projection
 *     unavailable compile-time information
 *     forbidden host inspection
 *     forbidden hardware inspection
 *     capability violation
 *     phase violation
 *
 * Boundary tests MUST include:
 *
 *     deeply chained reflection projections
 *     large generic declarations
 *     large member sets
 *     large metadata sets
 *     large generated reflection structures
 *
 * The tests MUST NOT establish artificial language limits.
 *
 * ============================================================================
 */


/* ============================================================================
 * 55. ROUND-TRIP CONTRACT
 * ============================================================================
 *
 * Where the repository provides a canonical formatter/printer:
 *
 *     source
 *       |
 *       v
 *     lexer
 *       |
 *       v
 *     parser
 *       |
 *       v
 *     AST
 *       |
 *       v
 *     printer
 *       |
 *       v
 *     parser
 *
 * must preserve reflection semantics.
 *
 * Projection ordering must remain stable.
 *
 * ============================================================================
 */


/* ============================================================================
 * 56. FINAL OWNERSHIP RULE
 * ============================================================================
 *
 * Reflection answers:
 *
 *     "What does the language's semantic model say about this entity?"
 *
 * It does NOT answer:
 *
 *     "What machine happens to exist right now?"
 *
 * unless an explicitly declared capability/effect/resource model says that
 * such environment-dependent information is part of the program's intended
 * semantics.
 *
 * This distinction is fundamental to POCO-REAF.
 *
 * ============================================================================
 */
/*
 * ============================================================================
 * ZAMANI PROGRAMMING LANGUAGE
 * ============================================================================
 *
 * FILE
 * ----
 * grammar/metaprogramming/introspection.g4
 *
 * GRAMMAR
 * -------
 * Introspection
 *
 * STATUS
 * ------
 * PRODUCTION METAPROGRAMMING COMPONENT
 *
 * BASELINE
 * --------
 * ANTLR4
 * Rust 2021
 * Rust 1.97+
 * Safe Rust only
 * No unsafe Rust
 *
 * ============================================================================
 * 1. PURPOSE
 * ============================================================================
 *
 * This file owns the SOURCE-LEVEL SYNTAX for explicit introspection.
 *
 * Introspection is deliberately distinct from ordinary reflection.
 *
 * Reflection:
 *
 *     asks about language-defined semantic structure.
 *
 * Introspection:
 *
 *     explicitly asks for observable information about a semantic subject,
 *     execution context, target context, resource context, capability context,
 *     or deployment context.
 *
 * This distinction is important for POCO-REAF.
 *
 * A normal source program must not silently become target-dependent merely
 * because an implementation happens to expose additional machine information.
 *
 * Introspection therefore represents an EXPLICIT request.
 *
 * This grammar records syntax only.
 *
 * It does NOT:
 *
 *     - perform introspection;
 *     - inspect hardware;
 *     - inspect runtime state;
 *     - inspect operating-system state;
 *     - inspect filesystem state;
 *     - inspect network state;
 *     - authorize capabilities;
 *     - grant permissions;
 *     - resolve resources;
 *     - select targets;
 *     - select devices;
 *     - select physical qubits;
 *     - execute code;
 *     - execute compile-time code;
 *     - create an IR;
 *     - create quantum::ir;
 *     - create an HDL IR;
 *     - bypass semantic analysis.
 *
 * ============================================================================
 * 2. ARCHITECTURAL POSITION
 * ============================================================================
 *
 * Zamani source
 *      |
 *      v
 * canonical lexer
 *      |
 *      v
 * canonical parser
 *      |
 *      v
 * domain-neutral AST
 *      |
 *      v
 * structural validation
 *      |
 *      +--> name resolution
 *      +--> type analysis
 *      +--> effect analysis
 *      +--> capability analysis
 *      +--> resource analysis
 *      +--> policy analysis
 *      +--> portability analysis
 *      +--> provenance
 *      |
 *      v
 * canonical semantic model
 *      |
 *      +--> classical semantics
 *      +--> quantum semantics
 *      +--> quantum::ir
 *      +--> HDL / hardware semantics
 *      +--> distributed semantics
 *      +--> AI / data semantics
 *      +--> networking semantics
 *      |
 *      v
 * optimization / lowering / routing / scheduling / resilience
 *      |
 *      v
 * HAL / runtime / deployment
 *
 * Introspection stops at the frontend/semantic boundary.
 *
 * ============================================================================
 * 3. OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS
 * -------------
 *
 *     introspectionExpressionCore
 *     introspectionRequest
 *     introspectionGeneralRequest
 *     introspectionTypeRequest
 *     introspectionTargetRequest
 *     introspectionResourceRequest
 *     introspectionCapabilityRequest
 *     introspectionDeploymentRequest
 *     introspectionSubject
 *     introspectionArguments
 *     introspectionArgument
 *     introspectionArgumentValue
 *     introspectionProjection
 *     introspectionSelector
 *
 * THIS FILE DOES NOT OWN
 * ----------------------
 *
 *     - lexer tokens;
 *     - identifiers;
 *     - qualified names;
 *     - paths;
 *     - ordinary expressions;
 *     - ordinary member access;
 *     - ordinary calls;
 *     - types;
 *     - literals;
 *     - declarations;
 *     - statements;
 *     - reflection;
 *     - macros;
 *     - quotation;
 *     - unquotation;
 *     - source generation;
 *     - specialization;
 *     - compile-time execution;
 *     - effects;
 *     - capabilities;
 *     - resources;
 *     - policies;
 *     - semantic analysis;
 *     - AST implementation;
 *     - canonical IR;
 *     - quantum::ir;
 *     - HDL/hardware IR;
 *     - routing;
 *     - scheduling;
 *     - QEC;
 *     - ZQN;
 *     - HAL;
 *     - runtime implementation.
 *
 * ============================================================================
 * 4. SINGLE-OWNER RULE
 * ============================================================================
 *
 * This file is the sole owner of the parser-level introspection core.
 *
 * No other grammar file may redefine:
 *
 *     introspectionExpressionCore
 *
 * No other grammar file may create an alternative introspection grammar.
 *
 * The composition layer may expose a wrapper, but the syntax remains owned
 * here.
 *
 * ============================================================================
 * 5. REFLECTION BOUNDARY
 * ============================================================================
 *
 * Ordinary semantic reflection belongs to:
 *
 *     grammar/metaprogramming/reflection.g4
 *
 * Reflection answers questions such as:
 *
 *     what declaration is this?
 *     what type is this?
 *     what members does this type expose?
 *     what metadata belongs to this semantic entity?
 *
 * Introspection answers explicitly requested contextual questions such as:
 *
 *     what capabilities are observable here?
 *     what resources are available?
 *     what target context is active?
 *     what deployment context is active?
 *
 * This file MUST NOT redefine:
 *
 *     reflectionExpressionCore
 *
 * It MUST NOT turn reflection into hardware/runtime discovery.
 *
 * ============================================================================
 * 6. LEXICAL CONTRACT
 * ============================================================================
 *
 * This is a parser grammar.
 *
 * It consumes:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * No lexer rules are defined here.
 *
 * The canonical lexical vocabulary already provides the language-level tokens
 * needed by this grammar, including:
 *
 *     INTROSPECT
 *     TYPE
 *     TARGET
 *     RESOURCE
 *     CAPABILITY
 *     DEPLOY
 *     LPAREN
 *     RPAREN
 *     DOT
 *     COMMA
 *     ASSIGN
 *
 * No new lexer token is required for runtime introspection.
 *
 * Runtime context can be represented semantically through an ordinary source
 * name such as:
 *
 *     runtime
 *
 * for example:
 *
 *     introspect(target(runtime))
 *
 * This deliberately avoids reserving another global keyword merely for one
 * introspection category.
 *
 * New domain-specific selectors remain identifiers.
 *
 * Examples:
 *
 *     capacity
 *     topology
 *     architecture
 *     version
 *     availability
 *     calibration
 *     reliability
 *     memory
 *     compute
 *     measurement
 *
 * are NOT lexer keywords merely because they may be useful selectors.
 *
 * ============================================================================
 * 7. CANONICAL IMPORTS
 * ============================================================================
 *
 * Names:
 *
 *     grammar/core/names.g4
 *
 * owns:
 *
 *     identifier
 *     qualifiedName
 *
 * Types:
 *
 *     grammar/types/types.g4
 *
 * owns:
 *
 *     typeExpression
 *
 * Literals:
 *
 *     grammar/expressions/literals.g4
 *
 * owns:
 *
 *     literalExpression
 *
 * This file reuses those canonical rules.
 *
 * It does not recreate any of them.
 *
 * ============================================================================
 */

parser grammar Introspection;

options {
    tokenVocab = ZamaniLexer;
}

import
    Names,
    Type,
    Literals
;

/*
 * ============================================================================
 * 8. PUBLIC INTEGRATION BOUNDARY
 * ============================================================================
 *
 * This is the canonical public rule owned by this file.
 *
 * The metaprogramming composition layer should consume:
 *
 *     introspectionExpressionCore
 *
 * through a wrapper if a wrapper is required by the composition architecture.
 *
 * No competing public introspection expression rule is defined here.
 *
 * ============================================================================
 */

introspectionExpressionCore
    : INTROSPECT
      LPAREN
      introspectionRequest
      RPAREN
      introspectionProjection*
    ;


/*
 * ============================================================================
 * 9. REQUEST DISPATCH
 * ============================================================================
 *
 * The request category is explicit.
 *
 * This keeps the grammar open-ended while giving semantic analysis a stable
 * category boundary.
 *
 * ============================================================================
 */

introspectionRequest
    : introspectionGeneralRequest
    | introspectionTypeRequest
    | introspectionTargetRequest
    | introspectionResourceRequest
    | introspectionCapabilityRequest
    | introspectionDeploymentRequest
    ;


/*
 * ============================================================================
 * 10. GENERAL INTROSPECTION
 * ============================================================================
 *
 * General introspection operates on a named semantic subject.
 *
 * Example:
 *
 *     introspect(module::value)
 *
 * The parser does not determine whether the subject is:
 *
 *     - a value;
 *     - declaration;
 *     - module;
 *     - type;
 *     - operation;
 *     - resource;
 *     - capability;
 *     - policy;
 *     - domain object;
 *     - future semantic entity.
 *
 * Name resolution and semantic analysis determine that.
 *
 * ============================================================================
 */

introspectionGeneralRequest
    : qualifiedName
      introspectionArguments?
    ;


/*
 * ============================================================================
 * 11. TYPE INTROSPECTION
 * ============================================================================
 *
 * Type syntax remains owned by the canonical type grammar.
 *
 * Example:
 *
 *     introspect(type(MyType))
 *
 * or, for a qualified type:
 *
 *     introspect(type(module::MyType))
 *
 * The typeExpression rule is not reimplemented here.
 *
 * ============================================================================
 */

introspectionTypeRequest
    : TYPE
      LPAREN
      typeExpression
      RPAREN
      introspectionArguments?
    ;


/*
 * ============================================================================
 * 12. TARGET INTROSPECTION
 * ============================================================================
 *
 * Target introspection explicitly requests information about the compilation
 * or execution target context.
 *
 * Examples:
 *
 *     introspect(target(current))
 *
 *     introspect(target(runtime))
 *
 *     introspect(target(environment))
 *
 * The names:
 *
 *     current
 *     runtime
 *     environment
 *
 * remain ordinary semantic names.
 *
 * They are not universal target identities.
 *
 * The semantic layer determines what target context is observable.
 *
 * ============================================================================
 */

introspectionTargetRequest
    : TARGET
      LPAREN
      introspectionSubject
      introspectionArguments?
      RPAREN
    ;


/*
 * ============================================================================
 * 13. RESOURCE INTROSPECTION
 * ============================================================================
 *
 * Resource information is explicitly requested.
 *
 * Examples:
 *
 *     introspect(resource(memory))
 *
 *     introspect(resource(quantum))
 *
 *     introspect(resource(accelerator))
 *
 * The grammar does not define what resources exist.
 *
 * The resource subsystem owns those semantics.
 *
 * ============================================================================
 */

introspectionResourceRequest
    : RESOURCE
      LPAREN
      introspectionSubject
      introspectionArguments?
      RPAREN
    ;


/*
 * ============================================================================
 * 14. CAPABILITY INTROSPECTION
 * ============================================================================
 *
 * Capability information is explicitly requested.
 *
 * Examples:
 *
 *     introspect(capability(quantum::measurement))
 *
 *     introspect(capability(tensor::compute))
 *
 *     introspect(capability(accelerator))
 *
 * This syntax does not grant the capability.
 *
 * Capability resolution and authorization remain downstream.
 *
 * ============================================================================
 */

introspectionCapabilityRequest
    : CAPABILITY
      LPAREN
      introspectionSubject
      introspectionArguments?
      RPAREN
    ;


/*
 * ============================================================================
 * 15. DEPLOYMENT INTROSPECTION
 * ============================================================================
 *
 * DEPLOY is the existing canonical lexical token.
 *
 * This grammar intentionally does not introduce a separate DEPLOYMENT
 * keyword.
 *
 * Examples:
 *
 *     introspect(deploy(environment))
 *
 *     introspect(deploy(current))
 *
 * Deployment information may be target-dependent.
 *
 * Semantic analysis must therefore classify portability and effects.
 *
 * ============================================================================
 */

introspectionDeploymentRequest
    : DEPLOY
      LPAREN
      introspectionSubject
      introspectionArguments?
      RPAREN
    ;


/*
 * ============================================================================
 * 16. INTROSPECTION SUBJECT
 * ============================================================================
 *
 * A subject is deliberately restricted.
 *
 * Valid subjects are:
 *
 *     qualifiedName
 *     literalExpression
 *
 * Arbitrary expression recursion is deliberately excluded.
 *
 * This prevents a circular grammar dependency of the form:
 *
 *     expression
 *       -> introspection
 *       -> expression
 *       -> introspection
 *       -> ...
 *
 * If future semantics require introspecting the result of an arbitrary
 * computation, the program should explicitly represent that value through
 * the normal expression/quotation/metaprogramming architecture rather than
 * silently making introspection consume the entire expression grammar.
 *
 * ============================================================================
 */

introspectionSubject
    : qualifiedName
    | literalExpression
    ;


/*
 * ============================================================================
 * 17. QUERY ARGUMENTS
 * ============================================================================
 *
 * Query arguments belong to the request being introspected.
 *
 * Canonical shape:
 *
 *     introspect(
 *         target(subject,
 *             property = value
 *         )
 *     )
 *
 * There is intentionally no hard-coded list of property names.
 *
 * The semantic owner determines whether an argument is valid for the selected
 * introspection category.
 *
 * This allows future resource, quantum, hardware, distributed, networking,
 * AI, data, or other domains to add semantic selectors without modifying this
 * grammar.
 *
 * ============================================================================
 */

introspectionArguments
    : COMMA
      introspectionArgument
      (
          COMMA
          introspectionArgument
      )*
      COMMA?
    ;


/*
 * ============================================================================
 * 18. NAMED INTROSPECTION ARGUMENT
 * ============================================================================
 *
 * The argument name is an ordinary source identifier.
 *
 * The argument value is a canonical literal or symbolic name.
 *
 * Semantic analysis determines:
 *
 *     - whether the argument exists;
 *     - whether the argument is allowed;
 *     - the argument's type;
 *     - whether the value is valid;
 *     - whether the argument is portable;
 *     - whether the argument requires a capability;
 *     - whether the argument produces an effect.
 *
 * ============================================================================
 */

introspectionArgument
    : identifier
      ASSIGN
      introspectionArgumentValue
    ;


/*
 * ============================================================================
 * 19. ARGUMENT VALUES
 * ============================================================================
 *
 * Canonical literal syntax is reused rather than duplicating literal tokens.
 *
 * A qualified name is also accepted for symbolic values such as:
 *
 *     topology = hardware::topology
 *
 *     policy = execution::portable
 *
 *     profile = resource::preferred
 *
 * This grammar does not assign meaning to those names.
 *
 * ============================================================================
 */

introspectionArgumentValue
    : literalExpression
    | qualifiedName
    ;


/*
 * ============================================================================
 * 20. PROJECTION
 * ============================================================================
 *
 * After an introspection request, zero or more projections may select
 * information from the returned introspection view.
 *
 * Examples:
 *
 *     introspect(target(current)).capability
 *
 *     introspect(target(current)).availability
 *
 *     introspect(resource(memory)).capacity
 *
 *     introspect(capability(quantum::measurement)).availability
 *
 *     introspect(deploy(environment)).topology
 *
 * Projection names remain open-world identifiers.
 *
 * The parser does not maintain a closed list of possible properties.
 *
 * ============================================================================
 */

introspectionProjection
    : DOT
      introspectionSelector
    ;


/*
 * ============================================================================
 * 21. SELECTOR
 * ============================================================================
 *
 * Selectors are ordinary identifiers.
 *
 * They are intentionally NOT reserved keywords.
 *
 * This is essential for long-term extensibility.
 *
 * A future implementation may expose:
 *
 *     capacity
 *     topology
 *     architecture
 *     version
 *     feature
 *     calibration
 *     reliability
 *     availability
 *     memory
 *     compute
 *     measurement
 *     communication
 *     thermal
 *     power
 *     energy
 *     latency
 *
 * without changing the universal grammar.
 *
 * ============================================================================
 */

introspectionSelector
    : identifier
    ;


/*
 * ============================================================================
 * 22. AST CONTRACT
 * ============================================================================
 *
 * The parser creates parse-tree structure only.
 *
 * The frontend must map the construct into the existing domain-neutral AST.
 *
 * Conceptual representation:
 *
 *     IntrospectionExpr
 *     {
 *         request,
 *         projections,
 *         source_span
 *     }
 *
 * Request:
 *
 *     General
 *     {
 *         subject,
 *         arguments
 *     }
 *
 *     Type
 *     {
 *         type,
 *         arguments
 *     }
 *
 *     Target
 *     {
 *         subject,
 *         arguments
 *     }
 *
 *     Resource
 *     {
 *         subject,
 *         arguments
 *     }
 *
 *     Capability
 *     {
 *         subject,
 *         arguments
 *     }
 *
 *     Deployment
 *     {
 *         subject,
 *         arguments
 *     }
 *
 * Projection:
 *
 *     selector
 *
 * The actual Rust AST types remain owned by the frontend AST implementation.
 *
 * No Rust types are embedded in this grammar.
 *
 * ============================================================================
 * 23. SOURCE-SPAN CONTRACT
 * ============================================================================
 *
 * The frontend must preserve source spans for:
 *
 *     - the complete introspection expression;
 *     - request category;
 *     - subject;
 *     - each argument;
 *     - each selector;
 *     - each argument value.
 *
 * This is required for:
 *
 *     diagnostics;
 *     IDE tooling;
 *     provenance;
 *     generated-source tracking;
 *     semantic error reporting;
 *     reproducibility.
 *
 * ============================================================================
 * 24. SEMANTIC CONTRACT
 * ============================================================================
 *
 * Parsing an introspection expression does not establish that the request is
 * semantically valid.
 *
 * Semantic analysis must determine:
 *
 *     - whether the subject exists;
 *     - whether the subject is introspectable;
 *     - whether the selected scope is legal;
 *     - whether each argument is supported;
 *     - whether each selector is supported;
 *     - whether the information is observable;
 *     - whether a capability is required;
 *     - whether an effect is required;
 *     - whether a policy permits the request;
 *     - whether the request is compile-time or runtime;
 *     - whether the result is deterministic;
 *     - whether the result is portable;
 *     - whether provenance must be recorded.
 *
 * Unknown selectors are therefore normally semantic errors, not grammar
 * errors.
 *
 * ============================================================================
 * 25. TYPE CONTRACT
 * ============================================================================
 *
 * Introspection results must receive semantic types from the type system.
 *
 * This grammar does NOT define an:
 *
 *     IntrospectionType
 *
 * merely for convenience.
 *
 * Result typing may depend on:
 *
 *     - subject;
 *     - selector;
 *     - introspection scope;
 *     - semantic provider;
 *     - requested metadata;
 *     - compile-time/runtime phase.
 *
 * The type system remains authoritative.
 *
 * ============================================================================
 * 26. EFFECT CONTRACT
 * ============================================================================
 *
 * Introspection itself is syntactic intent.
 *
 * The semantic layer determines its effects.
 *
 * Possible effects include, where defined by the semantic system:
 *
 *     reflection
 *     runtime observation
 *     resource observation
 *     environment observation
 *     network observation
 *     hardware observation
 *
 * No effect is granted merely by parsing this construct.
 *
 * ============================================================================
 * 27. CAPABILITY CONTRACT
 * ============================================================================
 *
 * Introspection does not grant capabilities.
 *
 * For example:
 *
 *     introspect(resource(memory))
 *
 * does not grant memory access.
 *
 *     introspect(target(current))
 *
 * does not grant target-control authority.
 *
 *     introspect(deploy(environment))
 *
 * does not grant deployment-control authority.
 *
 * The capability subsystem decides what is permitted.
 *
 * ============================================================================
 * 28. RESOURCE CONTRACT
 * ============================================================================
 *
 * Introspection may observe resource information where the semantic contract
 * permits it.
 *
 * It must never encode universal resource ceilings.
 *
 * Forbidden grammar-level concepts include:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *     MAX_INTROSPECTION_DEPTH
 *     MAX_INTROSPECTION_RESULTS
 *
 * An implementation may impose resource-admission policies, but those are
 * implementation/environment constraints rather than language semantics.
 *
 * ============================================================================
 * 29. QUANTUM CONTRACT
 * ============================================================================
 *
 * Introspection may request semantic information related to quantum execution,
 * for example:
 *
 *     introspect(capability(quantum::measurement))
 *
 *     introspect(resource(quantum))
 *
 *     introspect(target(current)).availability
 *
 * The grammar MUST NOT expose or encode:
 *
 *     - physical qubit IDs;
 *     - fixed QPU IDs;
 *     - fixed topology sizes;
 *     - fixed gate sets;
 *     - calibration tables;
 *     - routing algorithms;
 *     - QEC implementations;
 *     - ZQN implementation;
 *     - hardware-specific instruction sets.
 *
 * Those remain downstream.
 *
 * The canonical quantum pipeline remains:
 *
 *     source
 *       ->
 *     domain-neutral AST
 *       ->
 *     semantic quantum model
 *       ->
 *     quantum::ir
 *       ->
 *     optimization
 *       ->
 *     decomposition
 *       ->
 *     routing
 *       ->
 *     scheduling
 *       ->
 *     resilience / QEC
 *       ->
 *     ZQN
 *       ->
 *     HAL
 *       ->
 *     target
 *
 * ============================================================================
 * 30. HDL / HARDWARE CONTRACT
 * ============================================================================
 *
 * Hardware and HDL information may be observable through semantic providers.
 *
 * Example:
 *
 *     introspect(target(current)).architecture
 *
 *     introspect(resource(memory)).capacity
 *
 *     introspect(capability(hardware::accelerator))
 *
 * The grammar remains independent of:
 *
 *     CPU width;
 *     register width;
 *     wire width;
 *     FPGA family;
 *     ASIC process;
 *     GPU model;
 *     accelerator model;
 *     device count;
 *     topology size.
 *
 * ============================================================================
 * 31. DISTRIBUTED CONTRACT
 * ============================================================================
 *
 * Distributed information must remain symbolic.
 *
 * For example:
 *
 *     introspect(resource(distributed))
 *
 *     introspect(target(current)).topology
 *
 * The grammar does not encode:
 *
 *     maximum node count;
 *     fixed cluster size;
 *     fixed network diameter;
 *     fixed device count.
 *
 * Those are environmental facts.
 *
 * ============================================================================
 * 32. PORTABILITY CONTRACT
 * ============================================================================
 *
 * Introspection can legitimately make an operation target/context-sensitive.
 *
 * That fact MUST be represented semantically.
 *
 * The compiler must not silently treat:
 *
 *     introspect(target(current))
 *
 * as equivalent to portable source-level computation if its result depends on
 * a particular target.
 *
 * Semantic analysis should therefore classify the result as appropriate:
 *
 *     portable;
 *     context-dependent;
 *     target-dependent;
 *     runtime-dependent;
 *     non-deterministic;
 *     unavailable.
 *
 * This classification belongs outside the grammar.
 *
 * ============================================================================
 * 33. POCO-REAF CONTRACT
 * ============================================================================
 *
 * The presence of introspection does not invalidate POCO-REAF.
 *
 * The important distinction is:
 *
 *     program semantics
 *
 * versus:
 *
 *     environmental observation.
 *
 * A program can remain source-stable while the compiler/runtime supplies a
 * different observation on a different target.
 *
 * Example:
 *
 *     introspect(resource(memory)).capacity
 *
 * may return different values on different machines without requiring source
 * rewriting.
 *
 * However, if program correctness depends on the result, the semantic system
 * must make that dependency explicit through contracts, policies, effects,
 * requirements, or control flow.
 *
 * ============================================================================
 * 34. DETERMINISM CONTRACT
 * ============================================================================
 *
 * Introspection may be deterministic or environment-dependent.
 *
 * The grammar does not decide which.
 *
 * Semantic analysis must record the relevant property.
 *
 * Reproducible builds/execution may therefore require:
 *
 *     - an explicit snapshot;
 *     - an execution context;
 *     - a capability;
 *     - a policy;
 *     - a recorded provenance source;
 *     - a deterministic provider.
 *
 * This grammar does not impose any particular mechanism.
 *
 * ============================================================================
 * 35. SECURITY CONTRACT
 * ============================================================================
 *
 * Introspection must not become a covert information-disclosure mechanism.
 *
 * Semantic/security analysis must control access to:
 *
 *     - environment information;
 *     - credentials;
 *     - secrets;
 *     - filesystem metadata;
 *     - network metadata;
 *     - hardware state;
 *     - deployment state;
 *     - security policy state;
 *     - protected resource information.
 *
 * The parser cannot enforce those controls.
 *
 * Parsing success MUST NOT imply authorization.
 *
 * ============================================================================
 * 36. PROVENANCE CONTRACT
 * ============================================================================
 *
 * If an introspection result contributes to:
 *
 *     generated source;
 *     compile-time decisions;
 *     specialization;
 *     optimization;
 *     deployment;
 *     runtime behavior;
 *     model selection;
 *     quantum compilation;
 *     hardware selection;
 *
 * semantic/compiler infrastructure should preserve provenance identifying:
 *
 *     - what was inspected;
 *     - which selector was requested;
 *     - which provider supplied the result;
 *     - when/under which phase it was observed;
 *     - which policy authorized it;
 *     - which transformation consumed it.
 *
 * This grammar only preserves the syntax required to establish that provenance
 * later.
 *
 * ============================================================================
 * 37. METAPROGRAMMING CONTRACT
 * ============================================================================
 *
 * Introspection may be used by compile-time metaprograms only when semantic
 * policy permits the requested observation.
 *
 * The pipeline remains:
 *
 *     parse
 *       ->
 *     AST
 *       ->
 *     semantic validation
 *       ->
 *     capability/effect/policy validation
 *       ->
 *     authorized introspection
 *       ->
 *     result validation
 *       ->
 *     metaprogram transformation
 *       ->
 *     generated source
 *       ->
 *     canonical frontend again
 *
 * Introspection MUST NOT bypass semantic validation simply because it occurs
 * during compilation.
 *
 * ============================================================================
 * 38. SOURCE GENERATION CONTRACT
 * ============================================================================
 *
 * If introspection participates in source generation:
 *
 *     introspection
 *          |
 *          v
 *     generated Zamani source
 *          |
 *          v
 *     canonical lexer
 *          |
 *          v
 *     canonical parser
 *          |
 *          v
 *     canonical AST
 *          |
 *          v
 *     semantic validation
 *
 * Generated source MUST NOT enter a backend directly.
 *
 * ============================================================================
 * 39. IR CONTRACT
 * ============================================================================
 *
 * This grammar produces no IR.
 *
 * Introspection results may influence semantic analysis or metaprogramming,
 * but any resulting program semantics must use the repository's canonical
 * representations.
 *
 * Quantum constructs continue through:
 *
 *     quantum::ir
 *
 * There is no introspection-specific quantum IR.
 *
 * ============================================================================
 * 40. COMPILER CONTRACT
 * ============================================================================
 *
 * Compiler implementation is responsible for:
 *
 *     - resolving the introspection subject;
 *     - validating scope;
 *     - checking effects;
 *     - checking capabilities;
 *     - checking policies;
 *     - obtaining observable information;
 *     - typing results;
 *     - recording provenance;
 *     - preserving deterministic/reproducible behavior where required;
 *     - rejecting unavailable or unauthorized observations.
 *
 * No compiler/backend behavior is embedded in this grammar.
 *
 * ============================================================================
 * 41. RUNTIME CONTRACT
 * ============================================================================
 *
 * Runtime introspection, where permitted, is implemented downstream.
 *
 * The grammar does not:
 *
 *     - access runtime memory;
 *     - access runtime devices;
 *     - access runtime topology;
 *     - access runtime network state;
 *     - access runtime credentials.
 *
 * Runtime access must pass through:
 *
 *     semantic authorization
 *       ->
 *     capability checking
 *       ->
 *     effect checking
 *       ->
 *     policy checking
 *       ->
 *     runtime provider
 *
 * ============================================================================
 * 42. DIAGNOSTIC CONTRACT
 * ============================================================================
 *
 * Parser diagnostics should be limited to syntactic failures such as:
 *
 *     - missing closing parenthesis;
 *     - malformed argument;
 *     - missing assignment;
 *     - malformed projection;
 *     - malformed subject;
 *     - malformed qualified name.
 *
 * The following are NOT parser errors:
 *
 *     unknown subject;
 *     unknown selector;
 *     unavailable capability;
 *     unavailable resource;
 *     unauthorized observation;
 *     target-dependent result;
 *     non-deterministic result;
 *     prohibited runtime observation.
 *
 * Those are semantic/policy/effect diagnostics.
 *
 * ============================================================================
 * 43. ERROR LOCALITY
 * ============================================================================
 *
 * The grammar deliberately keeps argument and projection boundaries explicit
 * so diagnostics can point to the smallest meaningful source span.
 *
 * For example:
 *
 *     introspect(target(current).availability
 *
 * should fail at the missing closing delimiter rather than causing a broad
 * failure elsewhere in the source program.
 *
 * ============================================================================
 * 44. SCALABILITY
 * ============================================================================
 *
 * The grammar contains no finite language-defined limits on:
 *
 *     - qualified-name depth;
 *     - number of query arguments;
 *     - number of projections;
 *     - selector length;
 *     - source program size;
 *     - number of introspection expressions.
 *
 * Repetition uses grammar constructs such as:
 *
 *     *
 *     +
 *
 * rather than hard-coded counts.
 *
 * Compiler implementation limits are resource-policy concerns, not language
 * ceilings.
 *
 * ============================================================================
 * 45. OPEN-WORLD EXTENSIBILITY
 * ============================================================================
 *
 * New introspection domains should normally NOT require a new parser rule.
 *
 * For example, future semantic providers may introduce:
 *
 *     introspect(resource(...)).thermal
 *     introspect(target(...)).architecture
 *     introspect(capability(...)).version
 *     introspect(deploy(...)).security
 *
 * without modifying this file.
 *
 * A new parser rule is justified only when a genuinely new syntactic category
 * is required.
 *
 * A new semantic property alone is NOT sufficient justification.
 *
 * ============================================================================
 * 46. FORBIDDEN HARD-CODING
 * ============================================================================
 *
 * This grammar MUST NOT encode:
 *
 *     MAX_QUBITS
 *     MAX_CPUS
 *     MAX_GPUS
 *     MAX_FPGAS
 *     MAX_NODES
 *     MAX_MEMORY
 *     MAX_THREADS
 *     MAX_TENSOR_RANK
 *     MAX_REGISTER_WIDTH
 *     MAX_NETWORK_SIZE
 *     MAX_DEVICE_COUNT
 *
 * It MUST also not encode:
 *
 *     - vendor-specific device names;
 *     - physical machine identifiers;
 *     - fixed QPU identifiers;
 *     - fixed GPU identifiers;
 *     - fixed CPU identifiers;
 *     - fixed topology sizes;
 *     - fixed memory sizes;
 *     - fixed register widths.
 *
 * ============================================================================
 * 47. COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * The canonical spelling:
 *
 *     introspect
 *
 * remains owned by the lexer.
 *
 * Existing parser integrations must consume:
 *
 *     introspectionExpressionCore
 *
 * rather than depending on private internal rules.
 *
 * Internal rule changes are permitted provided the public integration
 * contract remains stable.
 *
 * If the language later introduces another introspection scope, compatibility
 * requires:
 *
 *     specification update
 *     grammar update
 *     AST mapping
 *     semantic mapping
 *     diagnostics
 *     tests
 *     compatibility classification
 *
 * before the feature becomes stable.
 *
 * ============================================================================
 * 48. TEST CONTRACT
 * ============================================================================
 *
 * Positive tests MUST cover at least:
 *
 *     introspect(value)
 *     introspect(module::value)
 *     introspect(type(MyType))
 *     introspect(target(current))
 *     introspect(target(runtime))
 *     introspect(resource(memory))
 *     introspect(resource(quantum))
 *     introspect(capability(quantum::measurement))
 *     introspect(deploy(environment))
 *     introspect(target(current)).availability
 *     introspect(resource(memory)).capacity
 *     introspect(capability(quantum::measurement)).version
 *     introspect(target(current), profile = preferred).architecture
 *
 * where the final syntax is represented according to the argument contract
 * implemented by the parser composition.
 *
 * IMPORTANT:
 *
 * For target/resource/capability/deployment requests, arguments are inside
 * the scope parentheses.
 *
 * Example:
 *
 *     introspect(
 *         target(
 *             current,
 *             profile = preferred
 *         )
 *     ).architecture
 *
 * ============================================================================
 * 49. NEGATIVE TEST CONTRACT
 * ============================================================================
 *
 * Tests MUST reject malformed syntax such as:
 *
 *     introspect()
 *
 *     introspect(target())
 *
 *     introspect(resource())
 *
 *     introspect(capability())
 *
 *     introspect(deploy())
 *
 *     introspect(target(current,))
 *
 *     introspect(target(current, profile))
 *
 *     introspect(target(current, = preferred))
 *
 *     introspect(target(current).availability)
 *
 * when the latter is being parsed as a request subject rather than a
 * post-request projection.
 *
 * Semantic tests, separately, must reject:
 *
 *     unauthorized introspection;
 *     unknown selectors;
 *     unknown subjects;
 *     unavailable resources;
 *     unavailable capabilities;
 *     prohibited environment access.
 *
 * ============================================================================
 * 50. BOUNDARY TEST CONTRACT
 * ============================================================================
 *
 * Boundary tests MUST combine introspection with:
 *
 *     reflection;
 *     quotation;
 *     source generation;
 *     specialization;
 *     compile-time execution;
 *     contracts;
 *     policies;
 *     effects;
 *     capabilities;
 *     resources;
 *     quantum semantics;
 *     hardware semantics;
 *     distributed semantics.
 *
 * The purpose is to ensure that introspection never bypasses the canonical
 * semantic pipeline.
 *
 * ============================================================================
 * 51. QUANTUM BOUNDARY TEST
 * ============================================================================
 *
 * A representative semantic integration test should be able to express the
 * intent of:
 *
 *     inspect whether a target exposes quantum measurement capability
 *
 * without encoding:
 *
 *     a fixed number of qubits;
 *     a fixed QPU;
 *     a fixed gate set;
 *     a fixed topology;
 *     a vendor-specific implementation.
 *
 * Example:
 *
 *     introspect(
 *         capability(quantum::measurement)
 *     ).availability
 *
 * The returned semantic information is consumed by capability/policy
 * analysis, not directly by quantum routing.
 *
 * ============================================================================
 * 52. HARDWARE BOUNDARY TEST
 * ============================================================================
 *
 * A representative test should be able to query hardware characteristics
 * without making those characteristics part of the universal grammar.
 *
 * Example:
 *
 *     introspect(
 *         target(current)
 *     ).architecture
 *
 * The architecture value is environmental information.
 *
 * It must not redefine Zamani's source grammar.
 *
 * ============================================================================
 * 53. POCO-REAF ACCEPTANCE TEST
 * ============================================================================
 *
 * The same source program must remain syntactically valid across:
 *
 *     tiny target;
 *     embedded target;
 *     CPU target;
 *     multicore target;
 *     GPU target;
 *     FPGA target;
 *     accelerator target;
 *     QPU target;
 *     simulator;
 *     HPC target;
 *     cluster;
 *     distributed target;
 *     cloud target;
 *     future target.
 *
 * The observed introspection result may differ.
 *
 * That difference is environmental information, not a source-language
 * rewrite.
 *
 * ============================================================================
 * 54. SECURITY ACCEPTANCE
 * ============================================================================
 *
 * The implementation MUST demonstrate that:
 *
 *     parser success
 *
 * does NOT imply:
 *
 *     authorization;
 *     capability possession;
 *     resource access;
 *     hardware access;
 *     filesystem access;
 *     network access;
 *     environment access.
 *
 * ============================================================================
 * 55. DETERMINISM ACCEPTANCE
 * ============================================================================
 *
 * The implementation must be able to distinguish:
 *
 *     deterministic semantic metadata
 *
 * from:
 *
 *     target-dependent observation;
 *
 *     runtime-dependent observation;
 *
 *     mutable environment observation.
 *
 * If an introspection result affects generated or compiled output, the
 * compiler must preserve enough provenance to explain why that output was
 * produced.
 *
 * ============================================================================
 * 56. COMPLETION CRITERIA
 * ============================================================================
 *
 * This file is DONE when all of the following are true:
 *
 *     [ ] It imports the canonical Names grammar.
 *     [ ] It imports the canonical Type grammar.
 *     [ ] It imports the canonical Literals grammar.
 *     [ ] It consumes ZamaniLexer.
 *     [ ] It defines exactly one introspection core.
 *     [ ] It does not define lexer rules.
 *     [ ] It does not duplicate reflection syntax.
 *     [ ] It does not reference a nonexistent RUNTIME token.
 *     [ ] It does not reference obsolete literal token names.
 *     [ ] It does not define arbitrary expression recursion.
 *     [ ] It does not define fixed resource limits.
 *     [ ] It does not define vendor-specific hardware syntax.
 *     [ ] It keeps selectors open-world.
 *     [ ] It keeps semantic authorization downstream.
 *     [ ] It preserves source spans through the AST contract.
 *     [ ] It has a defined semantic contract.
 *     [ ] It has an effect contract.
 *     [ ] It has a capability contract.
 *     [ ] It has a resource contract.
 *     [ ] It has a policy contract.
 *     [ ] It has a provenance contract.
 *     [ ] It has a quantum boundary contract.
 *     [ ] It has an HDL/hardware boundary contract.
 *     [ ] It has a compiler boundary contract.
 *     [ ] It has a runtime boundary contract.
 *     [ ] It has positive tests.
 *     [ ] It has negative tests.
 *     [ ] It has boundary tests.
 *     [ ] It has scalability tests.
 *     [ ] It has determinism tests.
 *     [ ] It has compatibility tests.
 *
 * ============================================================================
 * 57. FINAL ARCHITECTURAL RULE
 * ============================================================================
 *
 * Introspection describes WHAT INFORMATION THE PROGRAM EXPLICITLY REQUESTS.
 *
 * It does not define:
 *
 *     HOW A PARTICULAR MACHINE PROVIDES THAT INFORMATION.
 *
 * Therefore:
 *
 *     source
 *       ->
 *     introspection syntax
 *       ->
 *     domain-neutral AST
 *       ->
 *     semantic validation
 *       ->
 *     authorized observation
 *       ->
 *     typed semantic value
 *       ->
 *     ordinary Zamani computation
 *
 * remains the canonical architecture.
 *
 * This preserves the fundamental Zamani principle:
 *
 *     Program Once
 *          ->
 *     Compile Once
 *          ->
 *     Run Everywhere
 *          ->
 *     Run Anywhere
 *          ->
 *     Forever
 *
 * while still allowing programs and metaprograms to explicitly reason about
 * the environment in which they are compiled or executed.
 *
 * ============================================================================
 */
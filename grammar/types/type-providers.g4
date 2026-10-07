/*
 * ============================================================================
 * Zamani Programming Language
 * ============================================================================
 *
 * File:
 *     grammar/types/type-providers.g4
 *
 * Grammar:
 *     TypeProviders
 *
 * Status:
 *     PRODUCTION TYPE / METAPROGRAMMING DELEGATE
 *
 * Compiler baseline:
 *     Rust 1.97+
 *     Rust 2021+
 *     safe Rust only
 *     no unsafe Rust
 *
 * ============================================================================
 * PURPOSE
 * ============================================================================
 *
 * This file defines the SOURCE-LEVEL SYNTAX for declarative type providers.
 *
 * A type provider is a compile-time semantic facility that can expose,
 * derive, query, or materialize type information from an explicitly declared
 * provider source.
 *
 * Examples of possible provider domains include:
 *
 *     schema providers
 *     database schema providers
 *     protocol/schema providers
 *     hardware-description providers
 *     data-model providers
 *     generated API type providers
 *     scientific schema providers
 *     quantum resource/type providers
 *     future domain-specific type providers
 *
 * The grammar is intentionally OPEN.
 *
 * It does NOT enumerate provider implementations.
 *
 * It does NOT enumerate databases.
 *
 * It does NOT enumerate protocols.
 *
 * It does NOT enumerate vendors.
 *
 * It does NOT enumerate hardware.
 *
 * It does NOT enumerate quantum devices.
 *
 * It does NOT enumerate AI frameworks.
 *
 * It does NOT define provider execution.
 *
 * It does NOT define a second type system.
 *
 * It does NOT define a second generic system.
 *
 * It does NOT define a second type-expression grammar.
 *
 * It does NOT define an IR.
 *
 * It does NOT perform provider resolution.
 *
 * It does NOT access external resources.
 *
 * It does NOT execute arbitrary compile-time code.
 *
 * ============================================================================
 * ARCHITECTURAL POSITION
 * ============================================================================
 *
 * The production pipeline remains:
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
 *     domain-neutral AST
 *          |
 *          v
 *     structural validation
 *          |
 *          +--> types
 *          +--> generics
 *          +--> constraints
 *          +--> effects
 *          +--> capabilities
 *          +--> resources
 *          +--> contracts
 *          +--> policies
 *          +--> provenance
 *          |
 *          v
 *     semantic analysis
 *          |
 *          +--> provider resolution
 *          +--> provider authorization
 *          +--> provider capability checking
 *          +--> provider effect checking
 *          +--> provider resource checking
 *          +--> provider determinism checking
 *          +--> type construction
 *          +--> type normalization
 *          +--> type checking
 *          |
 *          v
 *     canonical semantic model
 *          |
 *          +--> classical IR
 *          +--> quantum::ir
 *          +--> HDL/hardware semantic representation
 *          +--> other domain IR where explicitly required
 *          |
 *          v
 *     optimization / lowering / specialization
 *          |
 *          v
 *     routing / scheduling / resilience / QEC
 *          |
 *          v
 *     ZQN
 *          |
 *          v
 *     HAL
 *          |
 *          v
 *     target realization
 *
 * Type providers participate only in the type/metaprogramming portion of this
 * pipeline.
 *
 * ============================================================================
 * OWNERSHIP
 * ============================================================================
 *
 * THIS FILE OWNS:
 *
 *     typeProviderDeclaration
 *     typeProviderParameterList
 *     typeProviderParameter
 *     typeProviderSource
 *     typeProviderReference
 *     typeProviderQuery
 *     typeProviderQueryArgumentList
 *     typeProviderQueryArgument
 *     typeProviderResult
 *     typeProviderOptions
 *     typeProviderOption
 *
 * THIS FILE DOES NOT OWN:
 *
 *     typeExpression
 *     typePath
 *     qualifiedName
 *     genericParameter
 *     genericArgument
 *     generic type application
 *     type bounds
 *     type classes
 *     traits
 *     associated types
 *     dependent types
 *     higher-kinded types
 *     ordinary expressions
 *     ordinary declarations
 *     reflection
 *     macros
 *     compile-time execution
 *     source generation
 *     resource requirements
 *     capabilities
 *     effects
 *     contracts
 *     policies
 *     provenance
 *     target selection
 *     hardware discovery
 *     quantum allocation
 *     QPU selection
 *     HDL synthesis
 *     routing
 *     scheduling
 *     QEC
 *     ZQN
 *     HAL
 *     runtime execution
 *
 * ============================================================================
 * SINGLE TYPE-SYSTEM AUTHORITY
 * ============================================================================
 *
 * Type providers MUST NOT introduce another type-expression grammar.
 *
 * All provider-produced types ultimately become ordinary canonical Zamani
 * type expressions.
 *
 * The canonical type-expression owner remains:
 *
 *     grammar/types/types.g4
 *
 * or the repository's final canonical type-expression composition boundary.
 *
 * This file MUST NOT define:
 *
 *     typeExpression
 *     typeCore
 *     providerTypeExpression
 *     providerGenericType
 *     providerQualifiedType
 *
 * as replacements for the canonical type grammar.
 *
 * ============================================================================
 * TYPE PROVIDER VS TYPE DECLARATION
 * ============================================================================
 *
 * A type provider is NOT a replacement for:
 *
 *     type
 *     struct
 *     enum
 *     trait
 *     class
 *     interface
 *     record
 *     union
 *
 * Those declarations remain owned by their existing declaration grammars.
 *
 * A provider supplies or derives information that semantic analysis may
 * represent using the existing type system.
 *
 * Conceptually:
 *
 *     provider
 *         |
 *         v
 *     provider query
 *         |
 *         v
 *     provider result
 *         |
 *         v
 *     canonical TypeExpr
 *
 * There is therefore no ProviderType hierarchy parallel to TypeExpr.
 *
 * ============================================================================
 * OPEN-WORLD PRINCIPLE
 * ============================================================================
 *
 * Provider implementations are semantic extensions.
 *
 * The grammar must NOT contain alternatives such as:
 *
 *     databaseProvider
 *     jsonProvider
 *     xmlProvider
 *     sqlProvider
 *     grpcProvider
 *     protobufProvider
 *     quantumProvider
 *     gpuProvider
 *     fpgaProvider
 *     hardwareProvider
 *
 * Such implementation families remain:
 *
 *     libraries
 *     dialects
 *     provider implementations
 *     semantic plugins
 *     compiler extensions
 *     capability providers
 *
 * The universal grammar only represents the provider contract and invocation
 * boundary.
 *
 * ============================================================================
 * LEXER CONTRACT
 * ============================================================================
 *
 * This file contains NO lexer rules.
 *
 * Parser vocabulary comes exclusively from:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *
 * through:
 *
 *     tokenVocab = ZamaniLexer;
 *
 * A small amount of additional lexical vocabulary is required because the
 * repository currently does not expose a canonical `type_provider` keyword.
 *
 * Required canonical additions to grammar/lexer/keywords.g4:
 *
 *     TYPE_PROVIDER : 'type_provider' ;
 *     PROVIDES      : 'provides' ;
 *     PROVIDER      : 'provider' ;
 *
 * These additions belong to the lexical authority, NOT this file.
 *
 * They must then flow through:
 *
 *     grammar/lexer/tokens.g4
 *         ->
 *     grammar/lexer/lexer.g4
 *         ->
 *     grammar/antlr/ZamaniLexer.g4
 *
 * This file deliberately assumes those tokens exist after lexical integration.
 *
 * No token named:
 *
 *     TYPE_PROVIDER
 *     PROVIDES
 *     PROVIDER
 *
 * is defined here.
 *
 * ============================================================================
 * WHY A RESERVED TYPE_PROVIDER KEYWORD IS REQUIRED
 * ============================================================================
 *
 * A contextual identifier such as:
 *
 *     provider
 *
 * would create ambiguity with ordinary:
 *
 *     typeExpression
 *     typePath
 *     identifier
 *
 * and would make the provider declaration boundary dependent on semantic
 * interpretation.
 *
 * Production grammar should not rely on such accidental ambiguity.
 *
 * Therefore:
 *
 *     type_provider
 *
 * is an explicit declaration marker.
 *
 * Provider implementation names remain ordinary identifiers.
 *
 * ============================================================================
 * PROVIDER DECLARATION
 * ============================================================================
 *
 * Canonical form:
 *
 *     type_provider Name;
 *
 * Parameterized form:
 *
 *     type_provider Name<T>;
 *
 * Source-qualified form:
 *
 *     type_provider Name from schema::Source;
 *
 * Result-oriented form:
 *
 *     type_provider Name provides T;
 *
 * Full form:
 *
 *     type_provider Name<T>
 *         from schema::Source
 *         provides T
 *         ;
 *
 * The grammar deliberately does not require a provider implementation
 * language.
 *
 * Semantic analysis determines what `Name` denotes.
 *
 * ============================================================================
 * PROVIDER DECLARATION SEMANTICS
 * ============================================================================
 *
 * A provider declaration establishes a named provider contract.
 *
 * It does NOT execute the provider.
 *
 * It does NOT resolve external information during parsing.
 *
 * It does NOT automatically access:
 *
 *     filesystem
 *     network
 *     database
 *     credentials
 *     environment
 *     hardware
 *     QPU
 *     GPU
 *     FPGA
 *     operating-system state
 *
 * Such access requires explicit semantic authorization.
 *
 * ============================================================================
 * PROVIDER PARAMETERS
 * ============================================================================
 *
 * Provider parameters are intentionally represented using canonical
 * identifiers.
 *
 * Examples:
 *
 *     type_provider Schema<T>;
 *
 *     type_provider Schema<T, Source>;
 *
 * The grammar does not impose a finite parameter count.
 *
 * Semantic validation determines:
 *
 *     parameter uniqueness;
 *     parameter kind;
 *     parameter scope;
 *     parameter usage;
 *     provider contract validity.
 *
 * The provider parameter model must integrate with the repository's canonical
 * generic/type parameter infrastructure.
 *
 * ============================================================================
 * GENERIC INTEGRATION
 * ============================================================================
 *
 * Provider parameters MUST NOT create a second generic system.
 *
 * Where generic semantics are required, the provider declaration is lowered
 * into the repository's canonical generic/type-parameter representation.
 *
 * Canonical generic infrastructure remains owned by:
 *
 *     grammar/types/generic.g4
 *     grammar/functions/generics.g4
 *
 * as applicable.
 *
 * This file owns only the provider-specific declaration boundary.
 *
 * ============================================================================
 * TYPE-CLASS / TRAIT INTEGRATION
 * ============================================================================
 *
 * Provider-produced types may participate in:
 *
 *     trait bounds
 *     type-class constraints
 *     associated types
 *     generic constraints
 *     functional dependencies
 *     higher-kinded types
 *     dependent types
 *
 * This file does not define those mechanisms.
 *
 * Example semantic relationship:
 *
 *     provider
 *         ->
 *     canonical type
 *         ->
 *     generic constraint solving
 *         ->
 *     trait/type-class resolution
 *
 * A provider MUST NOT create a provider-specific constraint language.
 *
 * ============================================================================
 * HIGHER-KINDED TYPE INTEGRATION
 * ============================================================================
 *
 * Provider results may semantically describe type constructors.
 *
 * The canonical kind system remains owned by:
 *
 *     grammar/types/higher-kinded-types.g4
 *
 * A provider may therefore semantically provide:
 *
 *     *
 *
 *     * -> *
 *
 *     * -> * -> *
 *
 * or other valid kinds.
 *
 * This grammar does not enumerate kinds or arities.
 *
 * ============================================================================
 * PROVIDER SOURCE
 * ============================================================================
 *
 * `from` identifies the semantic source of provider information.
 *
 * Example:
 *
 *     type_provider DatabaseSchema
 *         from database::schema;
 *
 * The source is a symbolic provider reference.
 *
 * It is NOT a direct filesystem/network operation.
 *
 * The semantic layer resolves the source using the repository's:
 *
 *     module system
 *     dialect system
 *     provider registry
 *     capability system
 *     policy system
 *     provenance system
 *
 * ============================================================================
 * PROVIDER REFERENCE
 * ============================================================================
 *
 * A provider reference is a qualified identifier path:
 *
 *     schema
 *
 *     schema::database
 *
 *     organization::domain::provider
 *
 * The grammar does not enumerate namespaces.
 *
 * New provider ecosystems therefore do not require grammar modification.
 *
 * ============================================================================
 * PROVIDER RESULT
 * ============================================================================
 *
 * `provides` associates the provider with the canonical type-level result
 * expected from the provider contract.
 *
 * Example:
 *
 *     type_provider Schema
 *         provides SchemaType;
 *
 * The result is parsed using canonical:
 *
 *     typeExpression
 *
 * This guarantees that provider output participates in the same type system
 * as every other Zamani type.
 *
 * ============================================================================
 * PROVIDER QUERIES
 * ============================================================================
 *
 * Providers may be queried through a dedicated type-level query boundary.
 *
 * Canonical form:
 *
 *     provider::query(...)
 *
 * or:
 *
 *     provider::query(Provider, ...)
 *
 * The grammar intentionally keeps the operation open.
 *
 * No fixed operation catalogue is embedded.
 *
 * The qualified path is consumed by semantic provider infrastructure.
 *
 * ============================================================================
 * PROVIDER QUERY ARGUMENTS
 * ============================================================================
 *
 * Query arguments may be:
 *
 *     canonical type expressions;
 *     canonical expressions;
 *     provider references;
 *     symbolic identifiers.
 *
 * The grammar delegates interpretation to semantic analysis.
 *
 * It does not reinterpret runtime expressions as type expressions.
 *
 * ============================================================================
 * TYPE PROVIDER RESULT BOUNDARY
 * ============================================================================
 *
 * A provider query ultimately produces one of the canonical semantic results:
 *
 *     TypeExpr
 *
 * or another already-defined canonical type-level representation.
 *
 * It MUST NOT produce:
 *
 *     ProviderType
 *     ProviderIR
 *     ProviderAST
 *     UniversalProviderType
 *
 * as a parallel type hierarchy.
 *
 * ============================================================================
 * TYPE-LEVEL METAPROGRAMMING INTEGRATION
 * ============================================================================
 *
 * Type providers integrate with:
 *
 *     grammar/metaprogramming/type-level.g4
 *
 * and:
 *
 *     grammar/metaprogramming/metaprogramming.g4
 *
 * but do not replace them.
 *
 * Provider resolution is a type-level semantic operation.
 *
 * It must therefore follow the same safety boundary as type-level computation:
 *
 *     parse
 *       ->
 *     validate
 *       ->
 *     authorize
 *       ->
 *     evaluate/resolve
 *       ->
 *     validate result
 *       ->
 *     canonical TypeExpr
 *
 * ============================================================================
 * REFLECTION INTEGRATION
 * ============================================================================
 *
 * Reflection may inspect provider declarations or provider-produced semantic
 * types.
 *
 * Reflection remains owned by:
 *
 *     grammar/metaprogramming/reflection.g4
 *
 * Type providers MUST NOT define reflection syntax.
 *
 * ============================================================================
 * SOURCE GENERATION INTEGRATION
 * ============================================================================
 *
 * A provider may semantically participate in source generation.
 *
 * Generated declarations MUST re-enter the normal pipeline:
 *
 *     generated source
 *         ->
 *     lexer
 *         ->
 *     parser
 *         ->
 *     AST
 *         ->
 *     semantic validation
 *
 * There is no direct:
 *
 *     provider -> backend
 *
 * shortcut.
 *
 * ============================================================================
 * EFFECT CONTRACT
 * ============================================================================
 *
 * Declaring a provider has no runtime effect.
 *
 * Querying or resolving a provider may have semantic effects depending on the
 * provider implementation.
 *
 * Such effects MUST be represented using the canonical effect system.
 *
 * Possible semantic effects include:
 *
 *     io
 *     network
 *     foreign
 *     native
 *     reflection
 *     code_generation
 *
 * The grammar does not automatically grant any of these effects.
 *
 * ============================================================================
 * CAPABILITY CONTRACT
 * ============================================================================
 *
 * Provider resolution may require capabilities.
 *
 * Examples include conceptually:
 *
 *     capability("provider.resolve")
 *     capability("schema.read")
 *     capability("network.access")
 *
 * The actual capability vocabulary is semantic and open.
 *
 * This grammar does not grant capabilities.
 *
 * Provider syntax MUST NOT bypass:
 *
 *     authorization
 *     sandboxing
 *     policy
 *     trust
 *     provenance
 *
 * ============================================================================
 * RESOURCE CONTRACT
 * ============================================================================
 *
 * Provider resolution may require compiler or external resources.
 *
 * Examples include:
 *
 *     memory
 *     computation
 *     network bandwidth
 *     storage
 *     provider service availability
 *
 * No physical capacity is encoded here.
 *
 * There is no:
 *
 *     MAX_PROVIDER_PARAMETERS
 *     MAX_PROVIDER_RESULTS
 *     MAX_PROVIDER_DEPTH
 *     MAX_PROVIDER_QUERIES
 *     MAX_PROVIDER_SIZE
 *
 * Practical resource limits are implementation policy.
 *
 * ============================================================================
 * POLICY CONTRACT
 * ============================================================================
 *
 * Provider resolution MUST participate in the canonical policy system.
 *
 * Policies may control:
 *
 *     whether a provider may execute;
 *     which provider source may be accessed;
 *     which effects are permitted;
 *     which capabilities are permitted;
 *     whether network access is permitted;
 *     whether nondeterministic sources are permitted;
 *     whether generated types are trusted;
 *     whether cached provider results may be reused.
 *
 * Policy syntax is NOT defined here.
 *
 * ============================================================================
 * SANDBOX CONTRACT
 * ============================================================================
 *
 * Provider evaluation MUST be sandboxable.
 *
 * A provider must never gain unrestricted access merely because its declaration
 * parsed successfully.
 *
 * Provider execution must be constrained by:
 *
 *     effects
 *     capabilities
 *     resources
 *     policies
 *     authorization
 *     provenance
 *     compiler configuration
 *
 * ============================================================================
 * DETERMINISM CONTRACT
 * ============================================================================
 *
 * A provider may be:
 *
 *     deterministic
 *     reproducible
 *     externally stateful
 *     nondeterministic
 *
 * The grammar does not decide this.
 *
 * Semantic provider metadata must declare the relevant properties.
 *
 * Reproducible builds require provider results to be:
 *
 *     reproducible;
 *     pinned;
 *     cached;
 *     content-addressed;
 *     or otherwise governed by explicit build policy.
 *
 * Provider access to changing external state must never silently alter the
 * meaning of a supposedly reproducible build.
 *
 * ============================================================================
 * PROVENANCE CONTRACT
 * ============================================================================
 *
 * Provider resolution must preserve provenance sufficient to identify:
 *
 *     provider identity;
 *     provider version;
 *     provider source;
 *     provider query;
 *     provider inputs;
 *     provider result;
 *     resolution time where semantically relevant;
 *     dependency/version identity;
 *     policy decision;
 *     capability decision;
 *     generated/derived type relationship.
 *
 * The grammar only preserves source spans.
 *
 * Provenance construction belongs downstream.
 *
 * ============================================================================
 * CONTRACT INTEGRATION
 * ============================================================================
 *
 * Provider declarations may be referenced by:
 *
 *     requires
 *     ensures
 *     invariant
 *     assume
 *     guarantee
 *     property
 *
 * but this grammar does not define contract syntax.
 *
 * Type information produced by a provider may participate in contract
 * validation through the canonical semantic model.
 *
 * ============================================================================
 * QUANTUM INTEGRATION
 * ============================================================================
 *
 * Providers may describe quantum types or schemas.
 *
 * For example, a semantic provider may expose:
 *
 *     quantum::State
 *     quantum::Circuit
 *     quantum::Register
 *
 * This grammar does not define those types.
 *
 * Provider-produced quantum types continue through:
 *
 *     canonical type semantics
 *         ->
 *     quantum semantic model
 *         ->
 *     quantum::ir
 *
 * The provider layer MUST NOT:
 *
 *     allocate physical qubits;
 *     choose QPUs;
 *     choose coupling maps;
 *     perform routing;
 *     schedule operations;
 *     perform QEC;
 *     emit QZN/ZQN;
 *     access HAL directly.
 *
 * ============================================================================
 * HDL / HARDWARE INTEGRATION
 * ============================================================================
 *
 * Providers may expose hardware-intent types, schemas, interfaces, or
 * capabilities.
 *
 * Examples:
 *
 *     hardware::Device
 *     hardware::Signal
 *     hdl::Module
 *     hardware::Accelerator
 *
 * These remain ordinary canonical semantic types.
 *
 * Providers MUST NOT encode fixed hardware universes.
 *
 * ============================================================================
 * CLASSICAL / DATA / AI INTEGRATION
 * ============================================================================
 *
 * Providers may expose:
 *
 *     schemas
 *     datasets
 *     models
 *     tensors
 *     distributions
 *     knowledge structures
 *     protocol structures
 *
 * They remain canonical Zamani types.
 *
 * No application-specific keyword catalogue is required.
 *
 * ============================================================================
 * DISTRIBUTED / NETWORKING INTEGRATION
 * ============================================================================
 *
 * Provider sources may be distributed or network-backed.
 *
 * Such resolution is subject to:
 *
 *     network effects
 *     capabilities
 *     resources
 *     policies
 *     sandbox
 *     provenance
 *     reproducibility
 *
 * The grammar itself remains network-independent.
 *
 * ============================================================================
 * POCO-REAF / PORTABILITY CONTRACT
 * ============================================================================
 *
 * A type provider describes semantic information, not target hardware.
 *
 * Therefore provider declarations must remain independent of:
 *
 *     CPU count
 *     GPU count
 *     FPGA count
 *     ASIC count
 *     QPU count
 *     memory size
 *     register width
 *     tensor rank
 *     network size
 *     node count
 *     device count
 *     topology size
 *
 * The same provider-aware source can therefore participate in compilation
 * targeting:
 *
 *     tiny systems
 *     embedded systems
 *     CPUs
 *     multicore systems
 *     GPUs
 *     FPGAs
 *     ASICs
 *     accelerators
 *     QPUs
 *     simulators
 *     HPC
 *     clusters
 *     distributed systems
 *     cloud
 *     future computational substrates
 *
 * subject to semantic feasibility and available resources.
 *
 * ============================================================================
 * SCALABILITY CONTRACT
 * ============================================================================
 *
 * All lists use ANTLR repetition rather than fixed alternatives.
 *
 * There is no grammar-level limit on:
 *
 *     provider declarations
 *     provider parameters
 *     provider options
 *     query arguments
 *     qualification depth
 *     provider nesting
 *     type complexity
 *     source program size
 *     provider result complexity
 *
 * The grammar MUST NOT contain:
 *
 *     MAX_PROVIDER_PARAMETERS
 *     MAX_PROVIDER_ARGUMENTS
 *     MAX_PROVIDER_DEPTH
 *     MAX_PROVIDER_RESULTS
 *     MAX_PROVIDER_COUNT
 *
 * Practical parser/compiler safeguards may exist outside grammar semantics.
 *
 * Such safeguards must be:
 *
 *     configurable;
 *     explicit;
 *     diagnosable;
 *     implementation-specific;
 *     independent of target hardware.
 *
 * ============================================================================
 * AST CONTRACT
 * ============================================================================
 *
 * This grammar introduces no independent provider AST hierarchy.
 *
 * The frontend may represent provider syntax using the repository's existing
 * metaprogramming/type extension representation.
 *
 * Where a dedicated semantic node is necessary, the canonical frontend
 * representation should be conceptually:
 *
 *     TypeProvider {
 *         name,
 *         parameters,
 *         source,
 *         result,
 *         options,
 *         span
 *     }
 *
 * and:
 *
 *     TypeProviderQuery {
 *         provider,
 *         arguments,
 *         span
 *     }
 *
 * These are semantic/frontend constructs, not a replacement for TypeExpr.
 *
 * Provider results MUST ultimately lower to the existing canonical type
 * representation.
 *
 * ============================================================================
 * AST OWNERSHIP
 * ============================================================================
 *
 * AST_OWNER:
 *
 *     existing frontend type/metaprogramming AST
 *
 * This grammar MUST NOT require:
 *
 *     ProviderAST
 *     ProviderTypeAST
 *     TypeProviderIR
 *
 * as parallel universal representations.
 *
 * ============================================================================
 * SEMANTIC CONTRACT
 * ============================================================================
 *
 * Semantic analysis must establish:
 *
 *     provider declaration validity;
 *     provider name resolution;
 *     provider parameter resolution;
 *     provider source resolution;
 *     provider implementation availability;
 *     provider capability requirements;
 *     provider effect requirements;
 *     provider resource requirements;
 *     provider policy compliance;
 *     provider authorization;
 *     provider provenance;
 *     provider determinism;
 *     result validity;
 *     result kind;
 *     result type well-formedness;
 *     generic compatibility;
 *     constraint compatibility;
 *     associated-type compatibility;
 *     specialization compatibility.
 *
 * The grammar performs none of these operations.
 *
 * ============================================================================
 * KIND CONTRACT
 * ============================================================================
 *
 * Provider results may be:
 *
 *     ordinary types;
 *     type constructors;
 *     type-level values;
 *     canonical type-level semantic results.
 *
 * Kind checking belongs to:
 *
 *     grammar/types/higher-kinded-types.g4
 *
 * and the semantic kind system.
 *
 * Provider grammar does not enumerate kinds or arities.
 *
 * ============================================================================
 * IR CONTRACT
 * ============================================================================
 *
 * Type provider declarations and queries do not directly lower to:
 *
 *     classical machine instructions;
 *     quantum operations;
 *     HDL netlists;
 *     vendor IR;
 *     QASM;
 *     QIR;
 *     physical hardware instructions.
 *
 * Provider resolution occurs before final semantic IR construction.
 *
 * Provider-produced types are represented through the canonical semantic type
 * model.
 *
 * If a provider contributes quantum semantic information, that information
 * eventually crosses the established:
 *
 *     quantum::ir
 *
 * boundary.
 *
 * ============================================================================
 * BACKEND CONTRACT
 * ============================================================================
 *
 * Backends MUST consume resolved canonical semantic information.
 *
 * Backends MUST NOT be required to understand provider syntax.
 *
 * Therefore:
 *
 *     provider syntax
 *         ->
 *     semantic resolution
 *         ->
 *     canonical type model
 *         ->
 *     canonical IR
 *         ->
 *     backend
 *
 * This keeps provider implementations independent of target realization.
 *
 * ============================================================================
 * ERROR CONTRACT
 * ============================================================================
 *
 * Parser errors include:
 *
 *     missing provider name;
 *     malformed parameter list;
 *     empty parameter list;
 *     malformed provider source;
 *     malformed provider query;
 *     missing result type;
 *     malformed options;
 *     missing semicolon;
 *     malformed separators.
 *
 * Semantic errors include:
 *
 *     duplicate provider name;
 *     duplicate provider parameter;
 *     unresolved provider source;
 *     unknown provider;
 *     unauthorized provider;
 *     unavailable provider capability;
 *     forbidden provider effect;
 *     unavailable provider resource;
 *     invalid provider result;
 *     invalid provider kind;
 *     nondeterministic provider in reproducible compilation;
 *     invalid generic substitution;
 *     incompatible provider result.
 *
 * ============================================================================
 * COMPATIBILITY CONTRACT
 * ============================================================================
 *
 * This file is additive.
 *
 * It does not redefine:
 *
 *     type
 *     generic
 *     type-class
 *     bounds
 *     type constraints
 *     HKT
 *     reflection
 *     type-level computation
 *     compile-time execution.
 *
 * Existing source remains unaffected unless it explicitly uses the new
 * `type_provider` declaration or provider query syntax.
 *
 * ============================================================================
 * INTEGRATION CONTRACT
 * ============================================================================
 *
 * DEPENDS_ON:
 *
 *     grammar/antlr/ZamaniLexer.g4
 *     grammar/types/types.g4
 *     grammar/types/generic.g4
 *     grammar/types/bounds.g4
 *     grammar/types/type-constraints.g4
 *     grammar/types/type-class.g4
 *     grammar/types/higher-kinded-types.g4
 *     grammar/metaprogramming/type-level.g4
 *     grammar/metaprogramming/reflection.g4
 *     grammar/metaprogramming/metaprogramming.g4
 *     grammar/core/constraints.g4
 *     grammar/effects/
 *     grammar/resources/
 *     grammar/policies/
 *     grammar/security/
 *     grammar/compatibility/
 *
 * EXPORTS:
 *
 *     typeProviderDeclaration
 *     typeProviderParameterList
 *     typeProviderParameter
 *     typeProviderSource
 *     typeProviderReference
 *     typeProviderQuery
 *     typeProviderQueryArgumentList
 *     typeProviderQueryArgument
 *     typeProviderResult
 *     typeProviderOptions
 *     typeProviderOption
 *
 * CONSUMED_BY:
 *
 *     canonical type/metaprogramming composition grammar
 *     grammar/metaprogramming/metaprogramming.g4
 *     type-system semantic analysis
 *     compile-time/type-level semantic analysis
 *
 * AST_OWNER:
 *
 *     existing frontend type/metaprogramming AST
 *
 * SEMANTIC_OWNER:
 *
 *     canonical type-provider semantic subsystem
 *
 * IR_OWNER:
 *
 *     canonical semantic type model
 *
 *     quantum::ir only after provider-produced information becomes quantum
 *     semantic information.
 *
 * TEST_OWNER:
 *
 *     grammar/tests/types/type-providers/
 *     frontend/type-provider semantic tests
 *     metaprogramming conformance tests
 *
 * SPEC_OWNER:
 *
 *     grammar/spec/type-system.md
 *     grammar/spec/metaprogramming.md
 *     grammar/specification/
 *
 * ============================================================================
 * COMPOSITION RULE
 * ============================================================================
 *
 * The canonical type orchestrator remains the owner of type-expression
 * composition.
 *
 * Therefore this file MUST NOT be imported in a way that creates:
 *
 *     Type
 *       ->
 *     TypeProviders
 *       ->
 *     Type
 *
 * circular ownership.
 *
 * The recommended composition is:
 *
 *     canonical type/metaprogramming composition
 *                 |
 *                 +--> TypeProviders
 *
 * TypeProviders may consume canonical `typeExpression`, but does not own it.
 *
 * ============================================================================
 * REQUIRED COMPANION INTEGRATION
 * ============================================================================
 *
 * 1. grammar/lexer/keywords.g4
 *
 * Add:
 *
 *     TYPE_PROVIDER : 'type_provider' ;
 *     PROVIDES      : 'provides' ;
 *     PROVIDER      : 'provider' ;
 *
 * 2. grammar/lexer/tokens.g4
 *
 * Register those tokens in the canonical token registry with:
 *
 *     owner = keywords
 *     category = keyword
 *     context = type/metaprogramming
 *
 * 3. grammar/lexer/lexer.g4
 *
 * Consume the canonical keyword vocabulary through the existing lexical
 * composition mechanism.
 *
 * 4. grammar/antlr/ZamaniLexer.g4
 *
 * No direct token rule is added here.
 *
 * It continues importing the canonical lexer composition.
 *
 * 5. grammar/types/types.g4
 *
 * The canonical type orchestrator may consume provider-produced type results,
 * but MUST NOT redefine this provider grammar.
 *
 * 6. grammar/metaprogramming/metaprogramming.g4
 *
 * Import this grammar and expose:
 *
 *     typeProviderDeclaration
 *
 * through the existing metaprogramming declaration boundary.
 *
 * 7. grammar/metaprogramming/type-level.g4
 *
 * Provider queries may be consumed as a type-level facility.
 *
 * The existing type-level grammar remains the owner of generic type-level
 * computation.
 *
 * 8. AST/frontend
 *
 * Add or reuse canonical semantic representations for:
 *
 *     TypeProvider
 *     TypeProviderQuery
 *
 * without creating a second type hierarchy.
 *
 * 9. Semantic layer
 *
 * Add provider resolution after parsing and before canonical type
 * normalization.
 *
 * 10. Effects/capabilities/resources/policies
 *
 * Provider resolution must pass through the existing systems.
 *
 * ============================================================================
 * PUBLIC RULES
 * ============================================================================
 */

parser grammar TypeProviders;

options {
    tokenVocab = ZamaniLexer;
}


/*
 * ============================================================================
 * PROVIDER DECLARATION
 * ============================================================================
 *
 * Minimal:
 *
 *     type_provider Name;
 *
 * Parameterized:
 *
 *     type_provider Name<T>;
 *
 * Source:
 *
 *     type_provider Name from schema::source;
 *
 * Result:
 *
 *     type_provider Name provides T;
 *
 * Full:
 *
 *     type_provider Name<T>
 *         from schema::source
 *         provides T
 *         ;
 *
 * `from`, `provides`, and `options` are optional, but at least the provider
 * name and terminating semicolon are required.
 */
typeProviderDeclaration
    : TYPE_PROVIDER
      identifier
      typeProviderParameterList?
      typeProviderSource?
      typeProviderResult?
      typeProviderOptions?
      SEMICOLON
    ;


/*
 * ============================================================================
 * PROVIDER PARAMETERS
 * ============================================================================
 *
 * Provider parameters are names only at the grammar boundary.
 *
 * Their semantic kind/type is resolved by the existing type/generic system.
 */
typeProviderParameterList
    : LESS
      typeProviderParameter
      (
          COMMA
          typeProviderParameter
      )*
      COMMA?
      GREATER
    ;


typeProviderParameter
    : identifier
    ;


/*
 * ============================================================================
 * PROVIDER SOURCE
 * ============================================================================
 *
 * The source is symbolic.
 *
 * It does not perform IO.
 */
typeProviderSource
    : FROM
      typeProviderReference
    ;


typeProviderReference
    : identifier
      (
          DOUBLE_COLON
          identifier
      )*
    ;


/*
 * ============================================================================
 * PROVIDER RESULT
 * ============================================================================
 *
 * Provider output is always expressed through the canonical type expression
 * grammar.
 */
typeProviderResult
    : PROVIDES
      typeExpression
    ;


/*
 * ============================================================================
 * PROVIDER OPTIONS
 * ============================================================================
 *
 * Options are deliberately represented as a sequence of named options.
 *
 * Option semantics belong downstream.
 *
 * The syntax intentionally does not enumerate:
 *
 *     cache
 *     deterministic
 *     version
 *     network
 *     schema
 *     timeout
 *     authorization
 *
 * as a closed universal catalogue.
 *
 * Those concepts may be represented by attributes, policies, capabilities,
 * metadata, or future provider-specific semantic extensions.
 */
typeProviderOptions
    : WITH
      typeProviderOption
      (
          COMMA
          typeProviderOption
      )*
      COMMA?
    ;


typeProviderOption
    : identifier
      (
          ASSIGN
          typeProviderQueryArgument
      )?
    ;


/*
 * ============================================================================
 * TYPE-LEVEL PROVIDER QUERY
 * ============================================================================
 *
 * Open-world provider query boundary.
 *
 * Examples:
 *
 *     provider::query(...)
 *
 *     provider::query(DatabaseSchema, "users")
 *
 * The grammar does not enumerate provider operations.
 */
typeProviderQuery
    : typeProviderReference
      typeProviderQueryArgumentList
    ;


typeProviderQueryArgumentList
    : LPAREN
      typeProviderQueryArgument
      (
          COMMA
          typeProviderQueryArgument
      )*
      COMMA?
      RPAREN
    ;


typeProviderQueryArgument
    : typeExpression
    | expression
    | typeProviderReference
    ;


/*
 * ============================================================================
 * PROVIDER RESULT ADAPTER
 * ============================================================================
 *
 * This adapter exists for composition consumers that need to explicitly state
 * that the result of a provider operation is expected to be a canonical type.
 */
typeProviderResultExpression
    : typeProviderQuery
    ;
/*

* ============================================================================
* ZAMANI PROGRAMMING LANGUAGE
* ============================================================================
* 
* FILE
* ---
* grammar/metaprogramming/reflection.g4
* 
* GRAMMAR
* ---
* Reflection
* 
* STATUS
* ---
* CANONICAL PRODUCTION METAPROGRAMMING COMPONENT
* 
* BASELINE
* ---
* ANTLR4
* Rust 2021
* Rust 1.97+
* Safe Rust only
* No unsafe Rust required
* 
* ============================================================================
* PURPOSE
* ============================================================================
* 
* This file owns the SOURCE-LEVEL SYNTAX of explicit semantic reflection.
* 
* Reflection is a language facility for requesting structured information
* about a source-level entity or type.
* 
* Reflection is intentionally a VIEW over Zamani's canonical semantic model.
* 
* This grammar does not:
* 
* - define a second type system;
* - define a second expression language;
* - define a second AST;
* - define an IR;
* - define quantum::ir;
* - define an HDL IR;
* - define a hardware model;
* - select targets;
* - discover physical hardware;
* - execute code;
* - perform compile-time evaluation;
* - expand macros;
* - generate source;
* - specialize programs;
* - perform runtime introspection.
* 
* ============================================================================
* ARCHITECTURAL POSITION
* ============================================================================
* 
* source
*   |
*   v
* ZamaniLexer
*   |
*   v
* parser
*   |
*   v
* domain-neutral AST
*   |
*   v
* structural validation
*   |
*   v
* semantic analysis
*   |
*   +--> name resolution
*   +--> type resolution
*   +--> visibility
*   +--> effect analysis
*   +--> capability analysis
*   +--> resource analysis
*   +--> policy analysis
*   +--> provenance
*   |
*   v
* canonical semantic model
*   |
*   +--> classical semantics
*   +--> quantum semantics
*   +--> quantum::ir
*   +--> HDL/hardware semantics
*   +--> distributed semantics
*   +--> AI/data semantics
*   +--> networking semantics
*   |
*   v
* optimization / lowering / routing / scheduling / resilience
*   |
*   v
* HAL / target realization
* 
* Reflection participates only at the source/semantic boundary.
* 
* ============================================================================
* OWNERSHIP
* ============================================================================
* 
* THIS FILE OWNS
* ---
* 
* reflectionExpressionCore
* reflectionSubject
* reflectionValueSubject
* reflectionTypeSubject
* reflectionProjection
* reflectionSelector
* 
* THIS FILE DOES NOT OWN
* ---
* 
* identifier
* qualifiedName
* typeExpression
* ordinary expression syntax
* member access
* declarations
* statements
* attributes
* compile-time execution
* macros
* quotation
* source generation
* specialization
* introspection
* semantic reflection
* capability resolution
* resource resolution
* policies
* effects
* AST types
* IR
* quantum::ir
* HDL IR
* hardware discovery
* routing
* scheduling
* QEC
* ZQN
* HAL
* runtime execution
* 
* ============================================================================
* DEPENDENCY CONTRACT
* ============================================================================
* 
* DEPENDS_ON
* ---
* 
* Lexer:
* 
* grammar/antlr/ZamaniLexer.g4
* 
* Canonical parser components:
* 
* grammar/core/names.g4
* grammar/types/types.g4
* 
* ANTLR grammar names:
* 
* Names
* Type
* 
* Required imported rules:
* 
* qualifiedName
* identifier
* typeExpression
* 
* Required lexer tokens:
* 
* REFLECT
* TYPE
* LPAREN
* RPAREN
* DOT
* IDENTIFIER
* 
* IMPORTANT:
* 
* "REFLECT" is a language-level reserved word and must have exactly one
* lexical owner in the canonical lexical hierarchy.
* 
* It MUST be added to:
* 
* grammar/lexer/keywords.g4
* grammar/lexer/tokens.g4
* 
* with the canonical spelling:
* 
* REFLECT : 'reflect' ;
* 
* The parser must never define the token.
* 
* ============================================================================
* EXPORTS
* ============================================================================
* 
* Public parser integration boundary:
* 
* reflectionExpressionCore
* 
* Reusable internal rules:
* 
* reflectionSubject
* reflectionValueSubject
* reflectionTypeSubject
* reflectionProjection
* reflectionSelector
* 
* The metaprogramming composition layer owns the public wrapper:
* 
* reflectionExpression
* 
* This file deliberately does NOT define:
* 
* reflectionDeclarationCore
* reflectionStatementCore
* 
* because reflection is an expression facility.
* 
* ============================================================================
* CONSUMED BY
* ============================================================================
* 
* Primary consumer:
* 
* grammar/metaprogramming/metaprogramming.g4
* 
* Expression-level consumer:
* 
* grammar/expressions/metaprogramming.g4
* 
* Downstream semantic consumers:
* 
* semantic reflection analysis
* compile-time evaluation
* macro system
* source-generation system
* specialization system
* tooling / IDE / diagnostics
* 
* ============================================================================
* SINGLE-AUTHORITY RULE
* ============================================================================
* 
* There MUST be exactly one grammar owner for:
* 
* reflectionExpressionCore
* 
* There MUST be exactly one expression-level wrapper:
* 
* reflectionExpression
* 
* No other grammar file may reproduce the syntax:
* 
* REFLECT '(' ... ')'
* 
* Reflection properties such as:
* 
* name
* kind
* type
* members
* parameters
* effects
* capabilities
* requirements
* provenance
* 
* remain ordinary identifiers.
* 
* This is deliberate.
* 
* New semantic metadata must not require new lexer keywords.
* 
* ============================================================================
* CRITICAL RECURSION RULE
* ============================================================================
* 
* Reflection MUST NOT consume the complete "expression" grammar as its subject.
* 
* The canonical expression hierarchy eventually contains reflection:
* 
* expression
*   ->
* metaprogrammingExpression
*   ->
* reflectionExpression
*   ->
* reflectionExpressionCore
* 
* Therefore this file MUST NOT define:
* 
* reflectionSubject
*     : expression
*     ;
* 
* Such a definition would create an indirect recursive grammar dependency.
* 
* Reflection subjects are intentionally restricted to:
* 
* qualifiedName
* 
* or:
* 
* typeExpression
* 
* If future reflection of an arbitrary computed value is required, that
* facility must be expressed through the canonical quotation/value mechanism,
* not by making this grammar recursively consume "expression".
* 
* ============================================================================
* SOURCE FORMS
* ============================================================================
* 
* Canonical forms:
* 
* reflect(value)
* 
* reflect(namespace::value)
* 
* reflect(type(MyType))
* 
* reflect(type(namespace::MyType))
* 
* Projection:
* 
* reflect(value).name
* 
* reflect(value).type
* 
* reflect(type(MyType)).members
* 
* Chaining:
* 
* reflect(type(MyType)).members.attributes
* 
* reflect(namespace::operation).effects.capabilities
* 
* Projection depth has no grammar-defined finite limit.
* 
* ============================================================================
* OPEN-WORLD REFLECTION
* ============================================================================
* 
* The grammar recognizes only the STRUCTURE of a reflection request.
* 
* It does not enumerate semantic properties.
* 
* Therefore all of these are syntactically represented through the same rule:
* 
* reflect(foo).name
* reflect(foo).kind
* reflect(foo).type
* reflect(foo).members
* reflect(foo).quantum_metadata
* reflect(foo).hardware_requirements
* reflect(foo).provenance
* 
* Whether a property exists is a semantic question.
* 
* This permits reflection to evolve with:
* 
* classical computing
* numerical computing
* scientific computing
* AI
* data
* probabilistic computing
* quantum computing
* hybrid computing
* HDL
* hardware/software co-design
* accelerators
* distributed computing
* networking
* security
* future computational domains
* 
* without modifying this grammar for every new metadata property.
* 
* ============================================================================
* NAME CONTRACT
* ============================================================================
* 
* Names are owned by:
* 
* grammar/core/names.g4
* 
* This file consumes:
* 
* qualifiedName
* identifier
* 
* It does not redefine:
* 
* identifier
* nameSegment
* qualifiedName
* 
* Qualified names use the canonical:
* 
* nameSegment (DOUBLE_COLON nameSegment)*
* 
* syntax.
* 
* ============================================================================
* TYPE CONTRACT
* ============================================================================
* 
* Types are owned by:
* 
* grammar/types/types.g4
* 
* The canonical public type rule is:
* 
* typeExpression
* 
* This file consumes that rule directly.
* 
* It does not redefine:
* 
* typeCore
* namedType
* genericType
* dependentType
* quantumType
* hardwareType
* resourceType
* capabilityType
* or any other type constructor.
* 
* ============================================================================
* AST CONTRACT
* ============================================================================
* 
* The ANTLR parse tree is NOT the canonical AST.
* 
* The frontend AST layer should map:
* 
* reflectionExpressionCore
*      |
*      v
* ReflectionExpr
*      |
*      +--> subject
*      |      |
*      |      +--> NameReference
*      |      |
*      |      +--> TypeReference
*      |
*      +--> projections[]
*      |
*      +--> source span
* 
* The AST representation should preserve:
* 
* - complete source span;
* - subject span;
* - subject kind;
* - qualified-name segment order;
* - type structure;
* - projection order;
* - selector spelling;
* - source ordering;
* - provenance information supplied by the frontend.
* 
* This grammar MUST NOT depend on Rust AST implementation types.
* 
* ============================================================================
* SEMANTIC CONTRACT
* ============================================================================
* 
* Semantic analysis owns:
* 
* - name resolution;
* - type resolution;
* - visibility;
* - accessibility;
* - reflection eligibility;
* - projection validation;
* - phase legality;
* - effect analysis;
* - capability analysis;
* - resource analysis;
* - policy enforcement;
* - portability classification;
* - determinism classification;
* - provenance;
* - protection of implementation-private information.
* 
* Example:
* 
* reflect(quantum::operation).capabilities
* 
* is syntactically valid if "quantum::operation" is a valid qualified name.
* 
* Whether:
* 
* quantum::operation
* 
* resolves to a reflectable entity and whether:
* 
* capabilities
* 
* is a legal property is determined semantically.
* 
* ============================================================================
* REFLECTION SUBJECT SEMANTICS
* ============================================================================
* 
* A value subject may resolve to:
* 
* - value;
* - variable;
* - constant;
* - function;
* - method;
* - module;
* - namespace;
* - declaration;
* - quantum operation;
* - quantum circuit;
* - HDL entity;
* - hardware abstraction;
* - capability;
* - resource;
* - service;
* - actor;
* - AI model;
* - data schema;
* - dialect-defined entity;
* - future-domain entity.
* 
* A type subject may resolve to any valid Zamani type.
* 
* Semantic resolution, not syntax, determines the category.
* 
* ============================================================================
* QUANTUM CONTRACT
* ============================================================================
* 
* Reflection may inspect semantic descriptions of:
* 
* - quantum types;
* - qubits;
* - logical quantum resources;
* - quantum operations;
* - circuits;
* - measurements;
* - observables;
* - effects;
* - capabilities;
* - resource requirements;
* - provenance.
* 
* Reflection MUST NOT:
* 
* - enumerate a fixed gate set;
* - assign physical qubits;
* - select a QPU;
* - route operations;
* - schedule circuits;
* - perform decomposition;
* - perform QEC;
* - perform ZQN processing;
* - perform calibration;
* - mutate quantum::ir.
* 
* The quantum pipeline remains:
* 
* source
* ->
* domain-neutral AST
* ->
* semantic quantum model
* ->
* quantum::ir
* ->
* optimization
* ->
* decomposition
* ->
* routing
* ->
* scheduling
* ->
* resilience/QEC/ZQN
* ->
* HAL
* ->
* target
* 
* Reflection is a read/view request over semantic information.
* 
* ============================================================================
* HDL / HARDWARE CONTRACT
* ============================================================================
* 
* Reflection may inspect semantic descriptions of:
* 
* - HDL declarations;
* - modules;
* - ports;
* - signals;
* - timing intent;
* - resource requirements;
* - capabilities;
* - accelerator intent;
* - hardware-independent design metadata.
* 
* It MUST NOT imply:
* 
* - physical placement;
* - physical wiring;
* - device selection;
* - vendor selection;
* - physical register width;
* - physical memory size;
* - hardware inventory;
* - physical topology.
* 
* Those are downstream concerns.
* 
* ============================================================================
* RESOURCE / CAPABILITY CONTRACT
* ============================================================================
* 
* Reflection syntax does not perform capability discovery.
* 
* For example:
* 
* reflect(hardware::capability)
* 
* is merely a reflection request.
* 
* It does not implicitly mean:
* 
* inspect the current machine
* 
* or:
* 
* discover available GPUs
* 
* or:
* 
* discover available QPUs.
* 
* Target-dependent information requires explicit semantic authorization through
* the repository's capability/effect/resource/security model.
* 
* ============================================================================
* EFFECT CONTRACT
* ============================================================================
* 
* Parsing reflection has no runtime effects.
* 
* The following are NOT implied by this grammar:
* 
* IO
* network access
* native execution
* foreign execution
* hardware access
* measurement
* randomness
* mutation
* reflection of host state
* 
* If semantic evaluation of reflection requires an effect, the semantic layer
* must declare and validate it.
* 
* ============================================================================
* COMPILE-TIME CONTRACT
* ============================================================================
* 
* Reflection may be consumed by the compile-time subsystem.
* 
* However:
* 
* parsing != evaluation
* 
* This grammar never evaluates a reflection expression.
* 
* Compile-time evaluation must separately validate:
* 
* - phase;
* - capability;
* - effect;
* - determinism;
* - provenance;
* - resource policy;
* - security policy;
* - reproducibility.
* 
* ============================================================================
* MACRO CONTRACT
* ============================================================================
* 
* Macros may consume reflection results where permitted by semantic policy.
* 
* Reflection does not:
* 
* - expand macros;
* - create tokens;
* - create syntax trees;
* - bypass hygiene;
* - mutate macro state.
* 
* Generated source must re-enter the canonical frontend pipeline.
* 
* ============================================================================
* QUOTATION CONTRACT
* ============================================================================
* 
* Quotation remains owned by:
* 
* grammar/metaprogramming/quotation.g4
* 
* Reflection does not define:
* 
* quote
* unquote
* syntax quotation
* token quotation
* 
* If arbitrary expression reflection is eventually required, quotation or
* another explicitly specified value representation is the appropriate
* extension point.
* 
* ============================================================================
* GENERATION CONTRACT
* ============================================================================
* 
* Source generation remains owned by:
* 
* grammar/metaprogramming/generation.g4
* 
* Reflection may supply semantic metadata to generation, but reflection does
* not generate source.
* 
* Generated source must pass through ordinary:
* 
* lexer
* parser
* AST
* validation
* semantic analysis
* 
* before becoming part of a compilation.
* 
* ============================================================================
* SPECIALIZATION CONTRACT
* ============================================================================
* 
* Specialization may consume reflected information.
* 
* Reflection itself does not:
* 
* - select a target;
* - choose an implementation;
* - lower code;
* - specialize quantum circuits;
* - specialize hardware;
* - allocate resources.
* 
* Those decisions remain downstream.
* 
* ============================================================================
* INTROSPECTION BOUNDARY
* ============================================================================
* 
* "introspect" is a separate facility owned by:
* 
* grammar/metaprogramming/introspection.g4
* 
* Reflection and introspection must not be conflated.
* 
* Reflection:
* 
* source/semantic model information
* 
* Introspection:
* 
* explicitly requested context-dependent information
* 
* Therefore this file MUST NOT accept:
* 
* introspect(...)
* 
* and MUST NOT define introspection subjects.
* 
* ============================================================================
* PORTABILITY / POCO-REAF CONTRACT
* ============================================================================
* 
* Reflection syntax must remain target-independent.
* 
* The same source syntax may be compiled for:
* 
* tiny systems
* embedded systems
* CPU
* multicore CPU
* GPU
* FPGA
* ASIC
* accelerator
* QPU
* quantum simulator
* HPC
* cluster
* distributed systems
* cloud
* future execution substrates
* 
* Reflection MUST NOT encode:
* 
* - CPU counts;
* - GPU counts;
* - FPGA counts;
* - ASIC counts;
* - accelerator counts;
* - QPU counts;
* - qubit limits;
* - memory capacities;
* - register widths;
* - tensor ranks;
* - node counts;
* - topology sizes;
* - device identifiers;
* - vendor identifiers;
* - physical addresses;
* - calibration values;
* - scheduling decisions.
* 
* Target feasibility is determined downstream by:
* 
* semantic analysis
* resource analysis
* capability negotiation
* compilation
* lowering
* routing
* scheduling
* deployment
* runtime
* HAL
* 
* Reflection must never silently turn compilation into host-dependent
* specialization.
* 
* ============================================================================
* SCALABILITY CONTRACT
* ============================================================================
* 
* This grammar deliberately imposes no finite language-level limit on:
* 
* - qualified-name depth;
* - type-expression complexity;
* - projection-chain length;
* - number of reflected declarations;
* - number of reflected properties;
* - number of reflection expressions in a program;
* - generic type complexity;
* - domain size;
* - hardware scale.
* 
* Structural repetition uses:
* 
* * 
* 
* where appropriate.
* 
* Practical limits are implementation/resource limits, not language semantics.
* 
* The grammar MUST NOT introduce:
* 
* MAX_REFLECTION_DEPTH
* MAX_REFLECTIONS
* MAX_PROJECTIONS
* MAX_MEMBERS
* MAX_TYPES
* MAX_DECLARATIONS
* MAX_QUERY_LENGTH
* MAX_GENERIC_PARAMETERS
* MAX_HARDWARE_SIZE
* MAX_QUBITS
* MAX_CPUS
* MAX_GPUS
* MAX_FPGAS
* MAX_NODES
* MAX_MEMORY
* MAX_THREADS
* MAX_TENSOR_RANK
* MAX_REGISTER_WIDTH
* MAX_NETWORK_SIZE
* MAX_DEVICE_COUNT
* 
* ============================================================================
* DETERMINISM CONTRACT
* ============================================================================
* 
* Parsing must be deterministic.
* 
* Identical:
* 
* source
* lexer configuration
* grammar version
* language version
* 
* must produce equivalent parse structures.
* 
* Parsing must not depend on:
* 
* hardware;
* filesystem state;
* network state;
* environment variables;
* wall-clock time;
* randomness;
* target availability;
* deployment topology;
* runtime state.
* 
* Determinism of a reflection RESULT is semantic and may depend on the
* reflected information's classification.
* 
* ============================================================================
* SECURITY CONTRACT
* ============================================================================
* 
* Parsing reflection MUST NOT:
* 
* - read files;
* - read environment variables;
* - read credentials;
* - inspect arbitrary process memory;
* - inspect arbitrary processes;
* - inspect network state;
* - inspect physical hardware;
* - contact a QPU;
* - contact a GPU;
* - contact an FPGA;
* - execute host code;
* - execute generated code.
* 
* Security-sensitive semantic reflection requires explicit authorization
* through the existing security/capability/effect architecture.
* 
* ============================================================================
* DIAGNOSTIC CONTRACT
* ============================================================================
* 
* Parser diagnostics are restricted to malformed syntax.
* 
* Examples:
* 
* reflect(
* reflect()
* reflect(,)
* reflect(type())
* reflect(type(,))
* reflect(foo
* reflect(foo).
* reflect(foo)..
* 
* Semantic diagnostics belong downstream and include:
* 
* - unresolved reflection subject;
* - inaccessible subject;
* - invalid reflection target;
* - unknown semantic projection;
* - invalid projection for the reflected entity;
* - illegal phase;
* - missing capability;
* - missing effect authorization;
* - policy violation;
* - prohibited implementation detail;
* - non-portable dependency;
* - non-deterministic dependency where determinism is required.
* 
* These semantic errors MUST NOT be represented as an ever-growing parser
* catalogue of special cases.
* 
* ============================================================================
* ERROR RECOVERY
* ============================================================================
* 
* The grammar must preserve normal ANTLR error recovery.
* 
* It must not use:
* 
* semantic predicates;
* target-language actions;
* embedded Rust;
* runtime callbacks.
* 
* ============================================================================
* PERFORMANCE CONTRACT
* ============================================================================
* 
* The grammar uses only linear structural repetition for reflection:
* 
* projection*
* 
* There is no intentionally quadratic reflection-specific production.
* 
* Reflection does not copy or evaluate reflected entities.
* 
* Any memory associated with semantic reflection results belongs to the
* frontend/semantic implementation and is governed by implementation resource
* accounting.
* 
* ============================================================================
* ANTLR INTEGRATION CONTRACT
* ============================================================================
* 
* This file is a parser grammar:
* 
* parser grammar Reflection;
* 
* It consumes the canonical:
* 
* ZamaniLexer
* 
* It imports exactly:
* 
* Names
* Type
* 
* because those are the actual grammar names of:
* 
* grammar/core/names.g4
* grammar/types/types.g4
* 
* The old import:
* 
* Types
* 
* is intentionally NOT used.
* 
* ============================================================================
* PUBLIC COMPOSITION CONTRACT
* ============================================================================
* 
* This file exports:
* 
* reflectionExpressionCore
* 
* The composition grammar:
* 
* grammar/metaprogramming/metaprogramming.g4
* 
* owns:
* 
* reflectionExpression
* 
* and must contain:
* 
* reflectionExpression
*   : reflectionExpressionCore
*   ;
* 
* It MUST NOT expect:
* 
* reflectionDeclarationCore
* reflectionStatementCore
* 
* because those rules do not exist in this file and reflection is not a
* declaration/statement facility.
* 
* ============================================================================
* EXPRESSION INTEGRATION
* ============================================================================
* 
* The expression composition should conceptually be:
* 
* expression
*   ...
*   | metaprogrammingExpression
*   ...
*   ;
* 
* and:
* 
* metaprogrammingExpression
*   ...
*   | reflectionExpression
*   ...
*   ;
* 
* This file does not redefine either composition rule.
* 
* ============================================================================
* LEXER INTEGRATION REQUIRED BEFORE COMPOSITION
* ============================================================================
* 
* The canonical lexical hierarchy currently requires the addition:
* 
* REFLECT : 'reflect' ;
* 
* to its canonical keyword/token owner.
* 
* Required files:
* 
* grammar/lexer/keywords.g4
* grammar/lexer/tokens.g4
* 
* There must be exactly one emitted token identity for "reflect".
* 
* Do NOT add:
* 
* REFLECT
* 
* to:
* 
* grammar/antlr/ZamaniLexer.g4
* 
* because that file is only the public lexer composition boundary.
* 
* ============================================================================
* AST INTEGRATION
* ============================================================================
* 
* The frontend must map:
* 
* reflectionExpressionCore
* 
* into the existing domain-neutral AST.
* 
* No new domain-specific IR should be introduced merely for reflection.
* 
* Recommended semantic structure:
* 
* ReflectionExpr {
*   subject,
*   projections,
*   span
* }
* 
* where subject is represented by the existing name/type reference machinery.
* 
* If the existing AST does not yet have a reflection node, that is an AST
* integration task; it is not a reason to make this grammar own an AST.
* 
* ============================================================================
* SEMANTIC MODEL INTEGRATION
* ============================================================================
* 
* Reflection results should be represented through the canonical semantic
* reflection model.
* 
* The semantic model should distinguish at least:
* 
* declaration metadata
* type metadata
* function metadata
* effect metadata
* capability metadata
* resource metadata
* contract metadata
* policy metadata
* provenance metadata
* domain metadata
* 
* New domains should register semantic reflection metadata rather than modify
* this grammar.
* 
* ============================================================================
* IR CONTRACT
* ============================================================================
* 
* This grammar owns NO IR.
* 
* Reflection may be evaluated before IR generation or represented in a
* canonical semantic/compile-time representation.
* 
* It must not create:
* 
* - a reflection IR;
* - a quantum reflection IR;
* - a hardware reflection IR;
* - a backend-specific reflection IR.
* 
* If reflection is evaluated during compilation, its result must flow through
* the ordinary semantic/compile-time machinery.
* 
* ============================================================================
* QUANTUM IR BOUNDARY
* ============================================================================
* 
* If reflection inspects a quantum entity:
* 
* source
* ->
* AST
* ->
* quantum semantic model
* 
* Reflection may inspect the semantic representation.
* 
* It must not bypass the semantic model to inspect or mutate:
* 
* quantum::ir
* physical qubits
* routing state
* scheduling state
* QEC state
* calibration state
* 
* ============================================================================
* HARDWARE BOUNDARY
* ============================================================================
* 
* If reflection refers to hardware-related semantic names, the meaning is
* resolved by semantic analysis.
* 
* Reflection syntax itself does not select or discover a machine.
* 
* This is essential to POCO-REAF:
* 
* source intent
*   ->
* semantic requirements
*   ->
* capability/resource negotiation
*   ->
* target realization
* 
* rather than:
* 
* source
*   ->
* compiler-host-specific reflection
*   ->
* accidental specialization.
* 
* ============================================================================
* TOOLING CONTRACT
* ============================================================================
* 
* Tooling may use this grammar for:
* 
* - syntax highlighting;
* - parse-tree construction;
* - diagnostics;
* - formatting;
* - source navigation;
* - IDE/LSP structure;
* - source provenance.
* 
* Tooling must not infer runtime hardware semantics from syntax alone.
* 
* ============================================================================
* TEST CONTRACT
* ============================================================================
* 
* Positive syntax tests:
* 
* reflect(value)
* reflect(namespace::value)
* reflect(type(MyType))
* reflect(type(namespace::MyType))
* reflect(value).name
* reflect(value).kind
* reflect(value).type
* reflect(type(MyType)).members
* reflect(type(MyType)).members.attributes
* reflect(namespace::operation).effects.capabilities
* 
* Qualified-name scale:
* 
* reflect(a)
* reflect(a::b)
* reflect(a::b::c)
* ...
* 
* Projection scale:
* 
* reflect(a).x
* reflect(a).x.y
* reflect(a).x.y.z
* ...
* 
* Negative syntax tests:
* 
* reflect()
* reflect(,)
* reflect(type())
* reflect(type(,))
* reflect(value
* reflect(value).
* reflect(value)..
* reflect value
* reflect type(MyType)
* reflect(foo + bar)
* 
* The final negative category is intentional: arbitrary expression subjects
* belong to a separately specified quotation/value-reflection facility.
* 
* Boundary tests:
* 
* deeply qualified names;
* deeply nested generic types;
* long projection chains;
* Unicode identifiers permitted by the canonical lexer;
* comments and whitespace;
* EOF-adjacent constructs;
* large valid reflection expressions.
* 
* Cross-domain tests:
* 
* classical declaration reflection;
* quantum semantic reflection;
* hybrid reflection;
* HDL reflection;
* hardware-intent reflection;
* distributed reflection;
* AI/data reflection;
* networking reflection;
* security metadata reflection;
* dialect-defined reflection.
* 
* ============================================================================
* SCALABILITY TEST CONTRACT
* ============================================================================
* 
* Tests must demonstrate increasing structural size without establishing a
* universal maximum.
* 
* The test suite must NOT assert:
* 
* "N projections is the maximum"
* 
* or:
* 
* "N qualified-name segments is the maximum".
* 
* Instead, tests establish that additional structure remains syntactically
* representable until implementation resources are exhausted.
* 
* ============================================================================
* DETERMINISM TEST CONTRACT
* ============================================================================
* 
* Repeated parsing of identical source under identical grammar/lexer
* configuration must produce equivalent parse structures and source spans.
* 
* Parsing must not vary with:
* 
* CPU;
* GPU;
* FPGA;
* QPU;
* available memory;
* network;
* filesystem;
* environment;
* runtime state.
* 
* ============================================================================
* COMPATIBILITY CONTRACT
* ============================================================================
* 
* Existing filename remains:
* 
* grammar/metaprogramming/reflection.g4
* 
* Existing canonical production:
* 
* reflectionExpressionCore
* 
* is retained.
* 
* The wrapper:
* 
* reflectionExpression
* 
* remains owned by:
* 
* grammar/metaprogramming/metaprogramming.g4
* 
* The source spelling:
* 
* reflect
* 
* becomes a reserved lexical spelling through the canonical lexer vocabulary.
* 
* Reflection selectors remain ordinary identifiers.
* 
* ============================================================================
* SAFE-RUST CONTRACT
* ============================================================================
* 
* This grammar contains:
* 
* - no Rust actions;
* - no semantic predicates;
* - no unsafe code;
* - no filesystem access;
* - no network access;
* - no hardware access;
* - no process execution;
* - no runtime callbacks.
* 
* The consuming implementation must remain compatible with:
* 
* Rust 1.97+
* Rust 2021
* 
* and must use safe Rust.
* 
* ============================================================================
* HARD-CODING AUDIT
* ============================================================================
* 
* This grammar contains no language-level capacity constants.
* 
* In particular, it contains no:
* 
* MAX_QUBITS
* MAX_CPUS
* MAX_GPUS
* MAX_FPGAS
* MAX_NODES
* MAX_MEMORY
* MAX_THREADS
* MAX_TENSOR_RANK
* MAX_REGISTER_WIDTH
* MAX_NETWORK_SIZE
* MAX_DEVICE_COUNT
* MAX_REFLECTION_DEPTH
* MAX_REFLECTIONS
* MAX_PROJECTIONS
* 
* ============================================================================
* COMPLETION CRITERIA
* ============================================================================
* 
* This file is complete when:
* 
* [x] Existing filename is retained.
* [x] Reflection has one canonical source grammar.
* [x] reflectionExpressionCore is the public core boundary.
* [x] No declaration-level reflection is invented.
* [x] No statement-level reflection is invented.
* [x] No duplicate reflection syntax exists.
* [x] Actual grammar name "Type" is imported.
* [x] Actual grammar name "Names" is imported.
* [x] Canonical qualifiedName is reused.
* [x] Canonical typeExpression is reused.
* [x] Complete expression grammar is not recursively imported.
* [x] Projection names remain open-world identifiers.
* [x] No semantic property catalogue is hard-coded.
* [x] No quantum gate catalogue is encoded.
* [x] No hardware topology is encoded.
* [x] No machine-capacity limit is encoded.
* [x] No physical target selection is encoded.
* [x] No runtime hardware discovery is implicit.
* [x] No IR is created by the grammar.
* [x] quantum::ir remains downstream.
* [x] HDL/hardware realization remains downstream.
* [x] Compile-time execution remains downstream.
* [x] Macro expansion remains downstream.
* [x] Generation remains downstream.
* [x] Specialization remains downstream.
* [x] Introspection remains a separate facility.
* [x] AST ownership is explicit.
* [x] Semantic ownership is explicit.
* [x] Effect/capability/resource ownership is explicit.
* [x] Provenance requirements are explicit.
* [x] Security requirements are explicit.
* [x] Positive tests are defined.
* [x] Negative tests are defined.
* [x] Boundary tests are defined.
* [x] Scalability tests are defined.
* [x] Cross-domain tests are defined.
* [x] Determinism requirements are defined.
* [x] Safe-Rust requirements are defined.
* 
* ============================================================================
* FINAL INVARIANT
* ============================================================================
* 
* Reflection answers:
* 
* "What does the canonical Zamani semantic model expose about this
* source-level entity or type?"
* 
* It does not implicitly answer:
* 
* "What hardware exists?"
* "Which QPU is available?"
* "How many GPUs exist?"
* "What is the physical topology?"
* "What device should be selected?"
* 
* Those questions belong to explicitly authorized capability/resource/
* deployment/runtime/HAL mechanisms.
* 
* ============================================================================
  */

parser grammar Reflection;

options {
tokenVocab = ZamaniLexer;
}

/*

* ============================================================================
* CANONICAL DEPENDENCIES
* ============================================================================
* 
* Actual repository grammar names:
* 
* grammar/core/names.g4  -> parser grammar Names
* grammar/types/types.g4 -> parser grammar Type
* 
* Therefore the imports below intentionally use:
* 
* Names
* Type
* 
* and NOT "Types".
* ============================================================================
  */

import
Names,
Type
;

/*

* ============================================================================
* PUBLIC REFLECTION CORE
* ============================================================================
* 
* Canonical forms:
* 
* reflect(name)
* reflect(type(TypeName))
* 
* followed by zero or more semantic projections.
* 
* ============================================================================
  */

reflectionExpressionCore
: REFLECT
LPAREN
reflectionSubject
RPAREN
reflectionProjection*
;

/*

* ============================================================================
* REFLECTION SUBJECT
* ============================================================================
  */

reflectionSubject
: reflectionValueSubject
| reflectionTypeSubject
;

/*

* ============================================================================
* VALUE / DECLARATION SUBJECT
* ============================================================================
* 
* qualifiedName is intentionally reused from Names.
* 
* Semantic analysis determines what the name denotes.
* ============================================================================
  */

reflectionValueSubject
: qualifiedName
;

/*

* ============================================================================
* TYPE SUBJECT
* ============================================================================
* 
* Canonical source form:
* 
* reflect(type(MyType))
* 
* "typeExpression" is owned by Type.
* ============================================================================
  */

reflectionTypeSubject
: TYPE
LPAREN
typeExpression
RPAREN
;

/*

* ============================================================================
* PROJECTION
* ============================================================================
* 
* One projection is:
* 
* .identifier
* 
* The enclosing "reflectionExpressionCore" owns the repetition.
* 
* This avoids recursive projection rules while allowing arbitrary chain
* length subject only to implementation resources.
* ============================================================================
  */

reflectionProjection
: DOT
reflectionSelector
;

/*

* ============================================================================
* SELECTOR
* ============================================================================
* 
* Reflection metadata names are intentionally ordinary identifiers.
* 
* Do NOT replace this with a finite keyword list.
* ============================================================================
  */

reflectionSelector
: identifier
;
::

Required integration corrections

The file above is deliberately not accompanied by fake "reflectionDeclarationCore" or "reflectionStatementCore" rules. Those should be removed from "grammar/metaprogramming/metaprogramming.g4".

The metaprogramming composition should instead have this reflection section:

reflectionExpression
    : reflectionExpressionCore
    ;

and:

metaprogrammingExpression
    : macroExpression
    | compileTimeExpression
    | generationExpression
    | reflectionExpression
    | specializationExpression
    ;

The following must not remain:

reflectionDeclaration
    : reflectionDeclarationCore
    ;

reflectionStatement
    : reflectionStatementCore
    ;

because the production "reflection.g4" intentionally does not own declaration or statement syntax.

Required lexical integration

The repository currently has "REFLECTION" and "REFLECTIVE", but the production syntax requested by this grammar is:

reflect(...)

Therefore the canonical lexical vocabulary needs one new token:

REFLECT : 'reflect' ;

It belongs in the canonical keyword/token hierarchy, not in "reflection.g4" and not in "grammar/antlr/ZamaniLexer.g4".

The important distinction is:

reflection       -> effect/semantic vocabulary
reflect          -> source-language reflection operator
reflective       -> existing lexical vocabulary, if retained

Those should not be conflated.

Why I changed the existing design

The most important correction is that "reflection.g4" is now genuinely a leaf feature grammar:

Reflection
   │
   ├── Names
   │    └── qualifiedName
   │
   └── Type
        └── typeExpression

rather than trying to become a miniature language of its own.

The semantic pipeline is consequently:

reflect(...)
      │
      ▼
Reflection parse structure
      │
      ▼
domain-neutral AST
      │
      ▼
semantic reflection
      │
      ├── name/type resolution
      ├── visibility
      ├── effects
      ├── capabilities
      ├── resources
      ├── policies
      └── provenance
      │
      ▼
canonical semantic model
      │
      ├── classical
      ├── quantum
      ├── HDL
      ├── hardware
      ├── AI/data
      ├── distributed
      └── future domains

That means adding a future quantum construct, accelerator, tensor type, HDL entity, AI model, distributed service, or hardware capability does not require reopening this grammar. The new domain only needs to expose its metadata through the semantic reflection system.

Also, this preserves the important distinction between:

reflection

and:

introspection

so a program cannot accidentally become machine-dependent merely because it uses reflection.

One important caveat: the repository's existing ANTLR tree and the hand-written Rust frontend are not yet perfectly unified. In particular, "src/parser.rs" is a separate recursive-descent/Pratt parser with its own AST vocabulary. Therefore this grammar can be production-ready as the ANTLR grammar component, but full repository production readiness still requires the parser-to-AST adapter to recognize "ReflectionExpr" consistently with this contract. That should be treated as the next integration task rather than contaminating "reflection.g4" with Rust-specific behavior.
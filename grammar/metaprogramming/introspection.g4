/*

* ============================================================================
* Zamani Universal Programming Language
* ============================================================================
* 
* FILE
* ---
* grammar/metaprogramming/introspection.g4
* 
* STATUS
* ---
* PRODUCTION METAPROGRAMMING PARSER COMPONENT
* 
* PURPOSE
* ---
* This grammar defines the source-level syntax for EXPLICIT INTROSPECTION.
* 
* Introspection is distinct from ordinary semantic reflection:
* 
* reflection
*   = inspection of language-defined semantic information;
* 
* introspection
*   = an explicitly requested inspection operation whose information may
*     depend on an execution, compilation, deployment, capability,
*     resource, target, or runtime context.
* 
* Introspection is therefore an explicit language construct.
* 
* It MUST NOT silently turn ordinary source programs into hardware-dependent
* programs.
* 
* BASELINE
* ---
* ANTLR4
* Rust 2021
* Rust 1.97 / Rust 1.97.1
* Safe Rust only
* No unsafe Rust
* 
* ============================================================================
* ARCHITECTURAL POSITION
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
* domain-neutral frontend AST
*      |
*      v
* semantic analysis
*      |
*      +--> name resolution
*      +--> type analysis
*      +--> effect analysis
*      +--> capability analysis
*      +--> resource analysis
*      +--> portability analysis
*      +--> introspection authorization
*      |
*      v
* canonical semantic model
*      |
*      +--> classical semantics
*      +--> quantum semantics
*      +--> quantum::ir
*      +--> HDL / hardware semantics
*      +--> distributed semantics
*      +--> AI / data / networking semantics
*      |
*      v
* optimization / lowering / routing / scheduling / resilience
*      |
*      v
* HAL / runtime / deployment
* 
* Introspection is a FRONTEND SYNTAX FACILITY.
* 
* This grammar does not perform introspection.
* 
* ============================================================================
* OWNERSHIP
* ============================================================================
* 
* THIS FILE OWNS
* ---
* 
* - explicit "introspect" expression syntax;
* - introspection subjects;
* - introspection scopes;
* - introspection query paths;
* - introspection query arguments;
* - introspection option/qualifier syntax;
* - the canonical "introspectionExpressionCore" composition boundary.
* 
* THIS FILE DOES NOT OWN
* ---
* 
* - lexical token definitions;
* - identifiers;
* - qualified names;
* - ordinary expressions;
* - ordinary calls;
* - types;
* - declarations;
* - statements;
* - attributes;
* - effects;
* - capabilities;
* - resources;
* - hardware discovery;
* - runtime implementation;
* - reflection semantics;
* - compile-time evaluation;
* - macro expansion;
* - quotation;
* - source generation;
* - specialization;
* - classical IR;
* - quantum::ir;
* - HDL IR;
* - scheduling;
* - routing;
* - QEC;
* - ZQN;
* - resilience;
* - HAL implementation.
* 
* ============================================================================
* CORE ARCHITECTURAL DISTINCTION
* ============================================================================
* 
* Introspection MUST remain explicit.
* 
* The following are different semantic categories:
* 
* reflect(MyType)
* 
* introspect(target)
* 
* introspect(resource)
* 
* introspect(capability)
* 
* introspect(runtime)
* 
* introspect(deployment)
* 
* The grammar only records the programmer's requested category.
* 
* Semantic analysis determines:
* 
* - whether that category is supported;
* - whether the subject is valid;
* - whether the information is observable;
* - whether a capability is required;
* - whether an effect is required;
* - whether the result is portable;
* - whether the request is compile-time or runtime;
* - whether the result may be used in portable computation.
* 
* ============================================================================
* REFLECTION IS NOT DUPLICATED
* ============================================================================
* 
* "grammar/metaprogramming/reflection.g4" already owns:
* 
* reflectionExpressionCore
* 
* It defines language-semantic reflection such as:
* 
* reflect(value)
* reflect(type(T))
* reflect(value).name
* reflect(value).members
* 
* This file MUST NOT redefine those rules.
* 
* Introspection is deliberately separate because the specification distinguishes
* semantic reflection from implementation/target/runtime inspection.
* 
* ============================================================================
* NO IMPLICIT HOST INTROSPECTION
* ============================================================================
* 
* This grammar MUST NOT allow an introspection expression to mean:
* 
* inspect arbitrary process memory
* inspect arbitrary filesystem state
* inspect arbitrary environment variables
* inspect credentials
* inspect arbitrary operating-system state
* inspect arbitrary network state
* inspect compiler internals
* inspect backend internals
* 
* merely because the word "introspect" occurs in source.
* 
* Such operations require explicit semantic capability/effect authorization.
* 
* ============================================================================
* POCO-REAF
* ============================================================================
* 
* Introspection must preserve:
* 
* Program Once
*      |
* Compile Once
*      |
* Run Everywhere
*      |
* Run Anywhere
*      |
* Forever
* 
* Therefore introspection syntax MUST NOT encode universal limits such as:
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
* MAX_INTROSPECTION_DEPTH
* MAX_INTROSPECTION_RESULTS
* 
* Nor may it encode:
* 
* physical qubit identifiers
* fixed GPU identifiers
* fixed CPU identifiers
* fixed FPGA identifiers
* fixed QPU identifiers
* fixed device counts
* fixed topology sizes
* vendor-specific hardware identities
* 
* Numeric values appearing in an introspection argument are program values.
* 
* They are never universal compiler limits.
* 
* ============================================================================
* OPEN-WORLD DESIGN
* ============================================================================
* 
* Introspection scopes and selectors are intentionally open-ended.
* 
* The grammar does NOT enumerate every possible property.
* 
* Examples of semantic selectors may include:
* 
* capability
* availability
* capacity
* topology
* memory
* compute
* accelerator
* quantum
* measurement
* communication
* reliability
* version
* architecture
* feature
* 
* Those names remain ordinary identifiers.
* 
* Future domains can introduce new semantic information without continuously
* expanding the core lexical vocabulary.
* 
* ============================================================================
* LEXICAL CONTRACT
* ============================================================================
* 
* This grammar is parser-only.
* 
* It consumes:
* 
* tokenVocab = ZamaniLexer;
* 
* The canonical lexer MUST provide:
* 
* INTROSPECT
* 
* with the spelling:
* 
* introspect
* 
* "INTROSPECT" is the ONLY new reserved word required by this grammar.
* 
* It belongs to:
* 
* grammar/lexer/keywords.g4
* 
* and is composed by:
* 
* grammar/antlr/ZamaniLexer.g4
* 
* This grammar MUST NOT define lexer rules.
* 
* All other names used by introspection remain ordinary identifiers wherever
* the parser can distinguish them structurally.
* 
* ============================================================================
* NAME CONTRACT
* ============================================================================
* 
* Name syntax remains owned by:
* 
* grammar/core/names.g4
* 
* This grammar consumes:
* 
* qualifiedName
* identifier
* 
* It does not redefine them.
* 
* ============================================================================
* TYPE CONTRACT
* ============================================================================
* 
* Type syntax remains owned by:
* 
* grammar/types/types.g4
* 
* Introspection may explicitly inspect a type:
* 
* introspect type(T)
* 
* The type itself is parsed by:
* 
* typeExpression
* 
* This grammar does not define a second type grammar.
* 
* ============================================================================
* EXPRESSION CONTRACT
* ============================================================================
* 
* Introspection arguments deliberately use a restricted argument-value form.
* 
* This prevents:
* 
* expression
*     -> introspection
*         -> expression
* 
* from creating avoidable recursive grammar coupling.
* 
* The subject can be:
* 
* a qualified source name;
* an explicit type;
* an explicit target/resource/capability scope;
* a compile-time literal/value where the canonical argument grammar allows
* it.
* 
* Arbitrary computed-expression introspection is NOT implicitly accepted.
* 
* If future language semantics require arbitrary expression inspection, it
* should be introduced through the quotation/value metaprogramming facilities.
* 
* ============================================================================
* INTROSPECTION MODEL
* ============================================================================
* 
* Canonical source forms:
* 
* introspect(subject)
* 
* introspect type(Type)
* 
* introspect target(subject)
* 
* introspect resource(subject)
* 
* introspect capability(subject)
* 
* introspect runtime(subject)
* 
* introspect deployment(subject)
* 
* followed by zero or more selector projections:
* 
* .selector
* 
* Optional named query arguments may be supplied where the semantic contract
* permits them:
* 
* introspect target(subject, property = value)
* 
* The grammar does not decide whether an option is legal.
* 
* ============================================================================
* GRAMMAR DECLARATION
* ============================================================================
  */

parser grammar Introspection;

options {
tokenVocab = ZamaniLexer;
}

/*

* ============================================================================
* CANONICAL SHARED GRAMMAR IMPORTS
* ============================================================================
* 
* Names and types are imported from their canonical owners.
* 
* The complete expressions grammar is intentionally NOT imported.
* 
* ============================================================================
  */

import
Names,
Types
;

/*

* ============================================================================
* 1. PUBLIC INTEGRATION BOUNDARY
* ============================================================================
* 
* This is the ONLY public production owned by this file.
* 
* "grammar/metaprogramming/metaprogramming.g4" should expose the wrapper:
* 
* introspectionExpression
*     : introspectionExpressionCore
*     ;
* 
* No competing "introspectionExpression" production belongs here.
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
* 2. INTROSPECTION REQUEST
* ============================================================================
* 
* A request has one explicit scope.
* 
* Scope is syntactic intent.
* 
* Semantic authorization is downstream.
* ============================================================================
  */

introspectionRequest
: introspectionGeneralSubject
| introspectionTypeRequest
| introspectionTargetRequest
| introspectionResourceRequest
| introspectionCapabilityRequest
| introspectionRuntimeRequest
| introspectionDeploymentRequest
;

/*

* ============================================================================
* 3. GENERAL SUBJECT
* ============================================================================
* 
* A general subject is a canonical qualified name.
* 
* Semantic analysis determines what the name denotes and whether that entity
* is eligible for introspection.
* ============================================================================
  */

introspectionGeneralSubject
: qualifiedName
introspectionArgumentList?
;

/*

* ============================================================================
* 4. TYPE INTROSPECTION
* ============================================================================
* 
* Example:
* 
* introspect type(T)
* 
* Optional query arguments may follow the type.
* ============================================================================
  */

introspectionTypeRequest
: TYPE
LPAREN
typeExpression
RPAREN
introspectionArgumentList?
;

/*

* ============================================================================
* 5. TARGET INTROSPECTION
* ============================================================================
* 
* Target inspection is explicitly marked.
* 
* It MUST NOT be inferred from an ordinary source-level name.
* 
* Examples:
* 
* introspect target(current)
* introspect target(environment)
* 
* Whether the target is a compile-time target description or runtime target
* state is a semantic/effect/capability decision.
* ============================================================================
  */

introspectionTargetRequest
: TARGET
LPAREN
introspectionSubject
introspectionArgumentList?
RPAREN
;

/*

* ============================================================================
* 6. RESOURCE INTROSPECTION
* ============================================================================
* 
* Examples:
* 
* introspect resource(quantum)
* introspect resource(memory)
* 
* Resource semantics remain owned by the resource subsystem.
* ============================================================================
  */

introspectionResourceRequest
: RESOURCE
LPAREN
introspectionSubject
introspectionArgumentList?
RPAREN
;

/*

* ============================================================================
* 7. CAPABILITY INTROSPECTION
* ============================================================================
* 
* Examples:
* 
* introspect capability(quantum::measurement)
* introspect capability(accelerator)
* 
* Capability availability is not determined by the parser.
* ============================================================================
  */

introspectionCapabilityRequest
: CAPABILITY
LPAREN
introspectionSubject
introspectionArgumentList?
RPAREN
;

/*

* ============================================================================
* 8. RUNTIME INTROSPECTION
* ============================================================================
* 
* Runtime introspection is explicitly marked.
* 
* This rule does NOT grant runtime access by itself.
* 
* Semantic analysis must require whatever effect/capability contract the
* language defines for runtime inspection.
* ============================================================================
  */

introspectionRuntimeRequest
: RUNTIME
LPAREN
introspectionSubject
introspectionArgumentList?
RPAREN
;

/*

* ============================================================================
* 9. DEPLOYMENT INTROSPECTION
* ============================================================================
* 
* Deployment state is explicitly distinct from source semantics.
* 
* Example:
* 
* introspect deployment(environment)
* 
* This may be non-portable and therefore requires semantic portability
* classification.
* ============================================================================
  */

introspectionDeploymentRequest
: DEPLOY
LPAREN
introspectionSubject
introspectionArgumentList?
RPAREN
;

/*

* ============================================================================
* 10. INTROSPECTION SUBJECT
* ============================================================================
* 
* The subject is deliberately non-recursive.
* 
* It may be:
* 
* qualifiedName
* 
* or:
* 
* type(Type)
* 
* or a literal value where the canonical literal vocabulary permits a value
* to be used as an introspection key.
* 
* Arbitrary expressions are intentionally excluded.
* ============================================================================
  */

introspectionSubject
: qualifiedName
| introspectionLiteral
;

/*

* ============================================================================
* 11. LITERAL SUBJECT
* ============================================================================
* 
* Literal tokens are consumed from the canonical lexer.
* 
* This grammar does not redefine literal syntax.
* ============================================================================
  */

introspectionLiteral
: INTEGER
| FLOAT
| STRING
| CHAR
| TRUE
| FALSE
| NIL
| NULL
;

/*

* ============================================================================
* 12. QUERY ARGUMENTS
* ============================================================================
* 
* Query arguments are intentionally represented as:
* 
* name = literal
* 
* rather than embedding the complete expression grammar.
* 
* This keeps the grammar deterministic and avoids a circular dependency.
* 
* The semantic layer determines:
* 
* - whether the option is supported;
* - its type;
* - its effect;
* - its capability requirements;
* - whether it is portable.
* ============================================================================
  */

introspectionArgumentList
: LPAREN
introspectionArgument
(COMMA introspectionArgument)*
COMMA?
RPAREN
;

introspectionArgument
: identifier
ASSIGN
introspectionArgumentValue
;

introspectionArgumentValue
: introspectionLiteral
| qualifiedName
;

/*

* ============================================================================
* 13. PROJECTION
* ============================================================================
* 
* Projections are open-world identifiers.
* 
* Examples:
* 
* introspect(target).capability
* introspect(target).availability
* introspect(resource).capacity
* introspect(runtime).version
* introspect(deployment).topology
* 
* The semantic layer decides whether each projection is valid.
* ============================================================================
  */

introspectionProjection
: DOT
introspectionSelector
;

/*

* ============================================================================
* 14. SELECTOR
* ============================================================================
* 
* Selectors remain identifiers.
* 
* They are NOT keywords.
* ============================================================================
  */

introspectionSelector
: identifier
;

/*

* ============================================================================
* 15. QUERY ALIAS
* ============================================================================
* 
* Named semantic/tooling boundary.
* 
* This does not create another syntax.
* ============================================================================
  */

introspectionQuery
: introspectionExpressionCore
;

/*

* ============================================================================
* 16. PATH ALIAS
* ============================================================================
* 
* The complete introspection request including projections.
* ============================================================================
  */

introspectionPath
: introspectionExpressionCore
;

/*

* ============================================================================
* 17. TARGET QUERY ALIAS
* ============================================================================
  */

introspectionTargetQuery
: introspectionExpressionCore
;

/*

* ============================================================================
* 18. RESOURCE QUERY ALIAS
* ============================================================================
  */

introspectionResourceQuery
: introspectionExpressionCore
;

/*

* ============================================================================
* 19. CAPABILITY QUERY ALIAS
* ============================================================================
  */

introspectionCapabilityQuery
: introspectionExpressionCore
;

/*

* ============================================================================
* 20. RUNTIME QUERY ALIAS
* ============================================================================
  */

introspectionRuntimeQuery
: introspectionExpressionCore
;

/*

* ============================================================================
* 21. DEPLOYMENT QUERY ALIAS
* ============================================================================
  */

introspectionDeploymentQuery
: introspectionExpressionCore
;

/*

* ============================================================================
* AST CONTRACT
* ============================================================================
* 
* The parser produces only a parse tree.
* 
* The frontend must lower the parse structure into the existing domain-neutral
* AST architecture.
* 
* Conceptual representation:
* 
* IntrospectionExpr
*     request
*         scope
*         subject
*         arguments[]
*     projections[]
*     source_span
* 
* Conceptual request categories:
* 
* General
* Type
* Target
* Resource
* Capability
* Runtime
* Deployment
* 
* These are semantic categories, not a requirement to create Rust enum names
* with exactly these spellings.
* 
* The exact Rust AST representation belongs to:
* 
* src/frontend/ast/
* 
* This grammar MUST NOT define Rust AST types.
* 
* ============================================================================
* AST DATA THAT MUST BE PRESERVED
* ============================================================================
* 
* The AST conversion must preserve:
* 
* - complete source span;
* - request category;
* - subject structure;
* - qualified-name segments;
* - type-expression structure;
* - argument order;
* - argument names;
* - argument values;
* - projection order;
* - selector spelling;
* - source provenance.
* 
* It MUST NOT store:
* 
* - physical device handles;
* - runtime pointers;
* - scheduler state;
* - backend objects;
* - hardware instances;
* - mutable compiler state;
* - quantum::ir nodes.
* 
* ============================================================================
* SEMANTIC CONTRACT
* ============================================================================
* 
* Parsing proves only syntactic validity.
* 
* Semantic analysis must determine:
* 
* 1. whether the subject resolves;
* 2. whether the requested scope is legal;
* 3. whether the entity is introspectable;
* 4. whether the projection exists;
* 5. whether the projection applies to that subject;
* 6. whether the argument names are valid;
* 7. whether argument values have valid types;
* 8. whether the operation is compile-time or runtime;
* 9. whether an effect is required;
* 10. whether a capability is required;
* 11. whether a resource is required;
* 12. whether the result is deterministic;
* 13. whether the result is portable;
* 14. whether implementation-private information is being requested;
* 15. whether security policy permits the request.
* 
* Semantic errors MUST NOT be implemented by parser-specific hacks.
* 
* ============================================================================
* STATIC VERSUS DYNAMIC INFORMATION
* ============================================================================
* 
* The semantic layer must distinguish at least:
* 
* STATIC_LANGUAGE_INFORMATION
* COMPILE_TIME_INFORMATION
* TARGET_INFORMATION
* DEPLOYMENT_INFORMATION
* RUNTIME_INFORMATION
* 
* Static language information may be portable.
* 
* Target/deployment/runtime information may be environment-dependent.
* 
* An environment-dependent result MUST NOT silently become portable program
* semantics.
* 
* ============================================================================
* EFFECT CONTRACT
* ============================================================================
* 
* The grammar does not assign effects.
* 
* Semantic analysis determines whether an introspection request has effects.
* 
* In particular:
* 
* introspect(target(...))
* 
* does not automatically imply permission to inspect hardware.
* 
* introspect(runtime(...))
* 
* does not automatically grant runtime privileges.
* 
* introspect(deployment(...))
* 
* does not automatically grant deployment access.
* 
* Explicit capability/effect checking remains mandatory.
* 
* ============================================================================
* CAPABILITY CONTRACT
* ============================================================================
* 
* A capability may be required for:
* 
* target introspection;
* resource introspection;
* runtime introspection;
* deployment introspection;
* security-sensitive metadata;
* implementation-specific metadata.
* 
* The grammar merely records the request.
* 
* The semantic/security layers decide whether the request is authorized.
* 
* A compile-time introspection capability MUST NOT silently imply an unrelated
* runtime capability.
* 
* ============================================================================
* RESOURCE CONTRACT
* ============================================================================
* 
* Introspection may query resource descriptions only through the canonical
* resource model.
* 
* The grammar does not define:
* 
* RAM size
* VRAM size
* qubit capacity
* processor count
* accelerator count
* network-node count
* topology size
* 
* as language limits.
* 
* Resource values are semantic data.
* 
* ============================================================================
* HARDWARE CONTRACT
* ============================================================================
* 
* Hardware introspection is target/deployment/runtime information, not a
* parser-level hardware model.
* 
* It may eventually expose semantic properties such as:
* 
* capabilities
* availability
* supported operations
* resource classes
* topology descriptions
* reliability metadata
* 
* only where the corresponding semantic contract declares those properties
* observable.
* 
* It must not expose backend-private details as stable language semantics
* without an explicit target-dependent contract.
* 
* ============================================================================
* QUANTUM CONTRACT
* ============================================================================
* 
* Introspection may inspect semantic information associated with:
* 
* quantum capabilities;
* quantum resources;
* quantum operations;
* measurement capability;
* logical quantum resources;
* target quantum availability.
* 
* It MUST NOT itself:
* 
* - enumerate a fixed gate set;
* - allocate qubits;
* - assign physical qubits;
* - route circuits;
* - schedule circuits;
* - perform QEC;
* - perform ZQN analysis;
* - mutate quantum::ir;
* - select a QPU.
* 
* The canonical path remains:
* 
* source
*   |
*   v
* AST
*   |
*   v
* semantic quantum model
*   |
*   v
* quantum::ir
* 
* Introspection is a query over permitted semantic/capability information.
* 
* ============================================================================
* CLASSICAL CONTRACT
* ============================================================================
* 
* Classical introspection may inspect source-defined or explicitly exposed
* semantic information about:
* 
* functions
* types
* data
* algorithms
* capabilities
* execution environments
* 
* It does not introduce another classical type system.
* 
* ============================================================================
* HDL / HARDWARE CO-DESIGN CONTRACT
* ============================================================================
* 
* Introspection may inspect semantic descriptions of:
* 
* HDL modules
* ports
* interfaces
* capabilities
* timing contracts
* hardware resources
* 
* It does not itself perform:
* 
* synthesis
* placement
* routing
* timing closure
* physical design
* 
* Those remain downstream compiler/toolchain responsibilities.
* 
* ============================================================================
* DISTRIBUTED CONTRACT
* ============================================================================
* 
* Introspection may inspect explicitly exposed distributed properties such as:
* 
* capability;
* service availability;
* resource class;
* communication capability;
* consistency capability.
* 
* It must not establish a fixed node count or topology size.
* 
* ============================================================================
* AI / DATA CONTRACT
* ============================================================================
* 
* Introspection may inspect semantic descriptions of:
* 
* models
* tensors
* datasets
* schemas
* inference capabilities
* accelerator capabilities
* 
* It must not impose tensor-rank or model-size limits.
* 
* ============================================================================
* SECURITY CONTRACT
* ============================================================================
* 
* The grammar itself performs no security-sensitive operation.
* 
* Semantic implementation must reject or require explicit authorization for
* requests involving:
* 
* credentials
* secret material
* arbitrary host memory
* arbitrary process state
* unrestricted filesystem state
* unrestricted network state
* private compiler state
* backend-private implementation state
* 
* Introspection must never become an unrestricted compiler escape hatch.
* 
* ============================================================================
* DETERMINISM CONTRACT
* ============================================================================
* 
* Parsing must be deterministic.
* 
* For identical:
* 
* source
* language version
* lexical vocabulary
* parser grammar
* dialect configuration
* 
* the parser must produce the same syntactic structure.
* 
* Parsing MUST NOT depend on:
* 
* CPU availability
* GPU availability
* FPGA availability
* QPU availability
* filesystem state
* network state
* wall-clock time
* randomness
* deployment state
* runtime state.
* 
* Result determinism is semantic.
* 
* Static introspection SHOULD be deterministic.
* 
* Dynamic introspection may be environment-dependent, but that dependence must
* be explicit in the semantic/effect/capability/portability model.
* 
* ============================================================================
* SCALABILITY CONTRACT
* ============================================================================
* 
* The grammar imposes no finite limits on:
* 
* - qualified-name depth;
* - type complexity;
* - projection count;
* - query count;
* - argument count;
* - source size;
* - number of domains;
* - number of resources;
* - number of capabilities;
* - number of target properties.
* 
* Repetition is structural:
* 
* ( ... )*
* 
* rather than enumerated.
* 
* Compiler/parser implementation limits may exist for:
* 
* memory
* input size
* compilation time
* cancellation
* diagnostics
* semantic evaluation
* 
* Such limits are implementation/resource policy, not language semantics.
* 
* ============================================================================
* NO HARD-CODED SCALABILITY CONSTANTS
* ============================================================================
* 
* This grammar contains no:
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
* MAX_INTROSPECTION_DEPTH
* MAX_INTROSPECTION_RESULTS
* 
* It contains no fixed:
* 
* qubit IDs;
* processor IDs;
* GPU IDs;
* FPGA IDs;
* device IDs;
* topology IDs;
* vendor IDs.
* 
* ============================================================================
* NO SECOND QUERY LANGUAGE
* ============================================================================
* 
* This grammar deliberately does not introduce:
* 
* filters
* joins
* sorting
* grouping
* arbitrary predicates
* recursive search DSLs
* reflection-specific query comprehensions.
* 
* If such a facility becomes necessary, it must be specified independently and
* integrated through canonical Zamani expressions rather than silently
* expanding this grammar into a second programming language.
* 
* ============================================================================
* NO SECOND REFLECTION SYSTEM
* ============================================================================
* 
* "reflection.g4" remains authoritative for:
* 
* reflectionExpressionCore
* 
* This file is authoritative only for:
* 
* introspectionExpressionCore
* 
* The two mechanisms must remain semantically distinguishable.
* 
* ============================================================================
* IR CONTRACT
* ============================================================================
* 
* This grammar produces NO IR.
* 
* It must never directly create:
* 
* classical IR
* quantum::ir
* HDL IR
* hardware IR
* routing IR
* scheduling IR
* QEC operations
* ZQN faults
* resilience actions.
* 
* The required path is:
* 
* introspection syntax
*      |
*      v
* canonical frontend AST
*      |
*      v
* semantic introspection model
*      |
*      v
* canonical semantic representation
*      |
*      v
* ordinary IR lowering where applicable.
* 
* If introspection is compile-time-only and its result is consumed during
* compilation, it may disappear before target lowering.
* 
* If its result remains in the program, it must be represented by the ordinary
* canonical type/value/IR system.
* 
* ============================================================================
* GENERATED CODE CONTRACT
* ============================================================================
* 
* Introspection may be used by metaprograms that generate Zamani source.
* 
* Any generated source MUST re-enter the ordinary pipeline:
* 
* generated source
*      |
*      v
* lexer
*      |
*      v
* parser
*      |
*      v
* canonical AST
*      |
*      v
* semantic validation
*      |
*      v
* canonical semantic representation
* 
* Introspection must never authorize generated code to bypass validation.
* 
* ============================================================================
* MACRO CONTRACT
* ============================================================================
* 
* Macro expansion remains owned by:
* 
* grammar/macros/
* 
* Introspection may occur in an authorized metaprogramming context, but macro
* hygiene and expansion provenance remain owned by the macro subsystem.
* 
* Generated introspection expressions must be parsed and semantically validated
* exactly like handwritten expressions.
* 
* ============================================================================
* SOURCE PROVENANCE CONTRACT
* ============================================================================
* 
* The frontend must retain enough provenance to distinguish:
* 
* handwritten introspection
* macro-generated introspection
* generated introspection
* specialized introspection
* 
* Source spans must survive:
* 
* macro expansion;
* generation;
* specialization;
* diagnostic reporting.
* 
* This grammar itself does not implement source maps.
* 
* ============================================================================
* ERROR CONTRACT
* ============================================================================
* 
* Syntax errors belong to lexer/parser diagnostics.
* 
* Semantic diagnostics include, as applicable:
* 
* unknown introspection subject
* invalid introspection scope
* invalid selector
* invalid argument
* unavailable information
* forbidden information
* missing capability
* missing effect authorization
* non-portable introspection
* runtime-only request in compile-time context
* compile-time-only request in runtime context
* implementation-detail exposure
* unsupported target inspection
* security policy violation
* resource-policy violation
* 
* These must NOT be encoded as a growing collection of parser alternatives.
* 
* ============================================================================
* NEGATIVE SYNTAX CONTRACT
* ============================================================================
* 
* The following must be rejected syntactically:
* 
* introspect()
* 
* introspect(,)
* 
* introspect type()
* 
* introspect type(,)
* 
* introspect target()
* 
* introspect resource()
* 
* introspect capability()
* 
* introspect runtime()
* 
* introspect deployment()
* 
* introspect(target
* 
* introspect(target))
* 
* introspect target
* 
* introspect type T
* 
* introspect(target, = value)
* 
* introspect(target, property =)
* 
* introspect(target, property = , other = value)
* 
* introspect(target)..capability
* 
* introspect(target). 
* 
* Arbitrary expression subjects such as:
* 
* introspect(foo + bar)
* 
* are intentionally not accepted.
* 
* If expression introspection is required in the future, it must be specified
* through an explicit quotation/value mechanism.
* 
* ============================================================================
* POSITIVE CONFORMANCE CONTRACT
* ============================================================================
* 
* Minimum syntax cases:
* 
* introspect(foo)
* 
* introspect(namespace::foo)
* 
* introspect type(MyType)
* 
* introspect target(environment)
* 
* introspect resource(quantum)
* 
* introspect capability(quantum::measurement)
* 
* introspect runtime(environment)
* 
* introspect deployment(environment)
* 
* Projection cases:
* 
* introspect(target(environment)).capability
* 
* introspect(target(environment)).availability
* 
* introspect(resource(memory)).capacity
* 
* introspect(capability(quantum::measurement)).availability
* 
* introspect(runtime(environment)).version
* 
* introspect(deployment(environment)).topology
* 
* Argument cases:
* 
* introspect(target(environment, property = "compute"))
* 
* introspect(resource(memory, property = "capacity"))
* 
* introspect(capability(quantum::measurement, property = "availability"))
* 
* Multiple projections:
* 
* introspect(target(environment)).capability.availability
* 
* introspect(resource(memory)).capacity.unit
* 
* ============================================================================
* BOUNDARY TEST CONTRACT
* ============================================================================
* 
* Tests must include:
* 
* - one-segment names;
* - deeply qualified names;
* - nested type expressions;
* - long projection chains;
* - many query arguments;
* - Unicode identifiers where canonical lexer permits them;
* - whitespace variation;
* - comments around tokens;
* - source spans adjacent to EOF;
* - literal boundary forms;
* - empty/invalid argument forms;
* - large but valid query structures.
* 
* The tests must distinguish:
* 
* syntax boundaries
* 
* from:
* 
* compiler resource limits.
* 
* ============================================================================
* SCALABILITY TEST CONTRACT
* ============================================================================
* 
* The following must remain structurally representable without introducing
* grammar-level maxima:
* 
* introspect(a)
* 
* introspect(a::b::c)
* 
* introspect(a).x.y.z
* 
* introspect(target(a)).x.y.z
* 
* introspect(resource(a)).x.y.z
* 
* introspect(capability(a)).x.y.z
* 
* introspect(a, p1 = 1, p2 = 2, p3 = 3)
* 
* and arbitrarily larger structurally equivalent forms.
* 
* Tests must never assert a language maximum merely because the parser or test
* runner has an operational resource limit.
* 
* ============================================================================
* COMPATIBILITY CONTRACT
* ============================================================================
* 
* Existing filename:
* 
* grammar/metaprogramming/introspection.g4
* 
* is established without renaming any existing grammar file.
* 
* Existing:
* 
* grammar/metaprogramming/reflection.g4
* 
* remains unchanged in ownership.
* 
* Existing:
* 
* reflectionExpressionCore
* 
* remains the reflection integration boundary.
* 
* This file introduces:
* 
* introspectionExpressionCore
* 
* as its independent boundary.
* 
* The spelling:
* 
* introspect
* 
* is a new reserved lexical spelling.
* 
* It therefore requires a compatibility-aware lexer update.
* 
* No existing keyword token may be renamed or repurposed.
* 
* ============================================================================
* METAPROGRAMMING COMPOSITION CONTRACT
* ============================================================================
* 
* "grammar/metaprogramming/metaprogramming.g4" must expose exactly one wrapper:
* 
* introspectionExpression
*     : introspectionExpressionCore
*     ;
* 
* It must NOT redefine the actual introspection syntax.
* 
* Its expression dispatch may then include:
* 
* macroExpression
* compileTimeExpression
* generationExpression
* reflectionExpression
* introspectionExpression
* specializationExpression
* 
* The composition grammar remains responsible only for dispatch.
* 
* ============================================================================
* EXPRESSION COMPOSITION CONTRACT
* ============================================================================
* 
* "grammar/expressions/metaprogramming.g4" must consume the composition-level
* "introspectionExpression" wrapper rather than importing this grammar directly
* into the ordinary expression hierarchy.
* 
* There must be exactly one integration path.
* 
* This prevents duplicate alternatives and circular grammar dependencies.
* 
* ============================================================================
* PARSER COMPOSITION CONTRACT
* ============================================================================
* 
* "grammar/antlr/ZamaniParser.g4" remains the canonical parser composition
* boundary.
* 
* It should compose the metaprogramming subsystem through its established
* integration path.
* 
* This file must not become another parser root.
* 
* ============================================================================
* LEXER COMPOSITION CONTRACT
* ============================================================================
* 
* "grammar/lexer/keywords.g4" must add exactly:
* 
* INTROSPECT : 'introspect' ;
* 
* in its language/metaprogramming vocabulary.
* 
* "grammar/antlr/ZamaniLexer.g4" remains the canonical lexer entry point and
* must expose that token through its existing lexical composition mechanism.
* 
* No lexer rule belongs in this file.
* 
* ============================================================================
* AST / FRONTEND INTEGRATION
* ============================================================================
* 
* The AST integration must add or reuse a domain-neutral representation for:
* 
* introspection request
* 
* without creating:
* 
* HardwareIntrospectionAST
* QuantumIntrospectionAST
* RuntimeIntrospectionAST
* 
* as separate domain AST systems.
* 
* The semantic request category may distinguish the scopes after parsing.
* 
* Source spans and provenance remain mandatory.
* 
* ============================================================================
* SEMANTIC INTEGRATION
* ============================================================================
* 
* Semantic analysis must consume the AST and:
* 
* resolve the subject;
* validate the scope;
* validate projections;
* validate arguments;
* determine phase;
* determine effects;
* determine capabilities;
* determine resources;
* classify portability;
* protect implementation-private information;
* determine result type;
* preserve deterministic semantics where required.
* 
* No semantic behavior is embedded in this grammar.
* 
* ============================================================================
* RESOURCE / CAPABILITY INTEGRATION
* ============================================================================
* 
* Introspection must preserve the distinction:
* 
* requirement
* capability
* constraint
* preference
* hint
* observation
* 
* An observed target capability is not automatically a program requirement.
* 
* An observed resource availability is not automatically a fixed source-level
* resource allocation.
* 
* This distinction is essential to POCO-REAF.
* 
* ============================================================================
* COMPILER INTEGRATION
* ============================================================================
* 
* After parsing:
* 
* 1. Build canonical AST.
* 2. Resolve names.
* 3. Validate introspection scope.
* 4. Validate type/arguments.
* 5. Validate effects.
* 6. Validate capabilities.
* 7. Validate resource access.
* 8. Determine portability.
* 9. Evaluate only explicitly authorized introspection.
* 10. Preserve provenance.
* 11. Revalidate generated/specialized source if applicable.
* 12. Lower surviving semantic values through the ordinary IR pipeline.
* 
* No parser result may directly enter a backend.
* 
* ============================================================================
* RUNTIME INTEGRATION
* ============================================================================
* 
* Runtime introspection is a downstream concern.
* 
* The runtime may consume a semantically validated introspection operation only
* when the program's effect/capability contract authorizes it.
* 
* Runtime implementation must remain safe Rust:
* 
* Rust 1.97
* Rust 1.97.1
* Rust 2021
* 
* with no "unsafe".
* 
* This grammar itself contains no executable code.
* 
* ============================================================================
* IR INTEGRATION
* ============================================================================
* 
* Compile-time-only introspection may disappear after evaluation.
* 
* A surviving value is lowered using the ordinary canonical semantic/IR model.
* 
* There is no:
* 
* IntrospectionIR
* ReflectionIR
* HardwareIntrospectionIR
* QuantumIntrospectionIR
* 
* Quantum-related values still follow the canonical:
* 
* quantum::ir
* 
* boundary.
* 
* ============================================================================
* TARGET INTEGRATION
* ============================================================================
* 
* Target-specific introspection is explicitly downstream.
* 
* The grammar does not know:
* 
* CPU model
* GPU model
* FPGA family
* ASIC
* QPU
* vendor
* physical topology
* device count
* memory capacity
* accelerator identity.
* 
* The target layer determines which semantic observations are actually
* available.
* 
* ============================================================================
* SECURITY / TRUST INTEGRATION
* ============================================================================
* 
* Introspection of target/runtime/deployment state can be sensitive.
* 
* Semantic/security layers must therefore be able to distinguish:
* 
* publicly observable information
* capability-authorized information
* privileged information
* prohibited information.
* 
* The parser must remain unaware of the policy implementation.
* 
* ============================================================================
* TOOLING INTEGRATION
* ============================================================================
* 
* IDEs, formatters, language servers, documentation generators, and source-map
* systems must be able to identify:
* 
* introspectionExpressionCore
* introspectionRequest
* introspectionProjection
* 
* without executing introspection.
* 
* Formatting must preserve semantics and source provenance.
* 
* ============================================================================
* PERFORMANCE / HOSTILE INPUT CONTRACT
* ============================================================================
* 
* This grammar avoids:
* 
* fixed-depth recursion;
* enumerated selector lists;
* enumerated resource lists;
* enumerated hardware lists;
* enumerated capability lists.
* 
* Repetition is represented structurally.
* 
* Parser/compiler resource controls belong outside language semantics.
* 
* A hostile-input mitigation must never be expressed as:
* 
* MAX_INTROSPECTION_DEPTH
* 
* in this grammar.
* 
* ============================================================================
* SAFE-RUST CONTRACT
* ============================================================================
* 
* The grammar contains:
* 
* - no embedded Rust actions;
* - no semantic predicates;
* - no filesystem access;
* - no network access;
* - no environment access;
* - no hardware access;
* - no runtime execution;
* - no unsafe code.
* 
* Generated/compiler implementation must remain compatible with:
* 
* Rust 1.97
* Rust 1.97.1
* Rust 2021
* 
* and must not require Rust "unsafe".
* 
* ============================================================================
* COMPLETION CRITERIA
* ============================================================================
* 
* This file is complete when:
* 
* [x] Existing filenames are preserved.
* [x] Introspection has one canonical parser boundary.
* [x] Reflection remains owned by reflection.g4.
* [x] Introspection does not duplicate reflection syntax.
* [x] Introspection does not import the complete expression grammar.
* [x] Qualified names use canonical name syntax.
* [x] Types use canonical type syntax.
* [x] Selectors remain open-world identifiers.
* [x] Query arguments are structurally unbounded.
* [x] Projection chains are structurally unbounded.
* [x] No hardware limits are encoded.
* [x] No quantum limits are encoded.
* [x] No device IDs are encoded.
* [x] No second AST is defined.
* [x] No second IR is defined.
* [x] No second quantum IR is defined.
* [x] No runtime behavior is implemented in the grammar.
* [x] No host inspection is implicit.
* [x] Capability/effect authorization remains semantic.
* [x] Resource availability remains downstream.
* [x] Target realization remains downstream.
* [x] Generated source re-enters normal validation.
* [x] Quantum information ultimately respects quantum::ir.
* [x] Parser behavior is deterministic.
* [x] Source provenance is preserved by frontend integration.
* [x] Positive tests are defined.
* [x] Negative tests are defined.
* [x] Boundary tests are defined.
* [x] Scalability tests are defined.
* [x] Compatibility requirements are defined.
* [x] Security requirements are defined.
* [x] Rust 1.97/1.97.1 safe-Rust requirements are defined.
* 
* ============================================================================
* FINAL INVARIANT
* ============================================================================
* 
* "introspectionExpressionCore" answers:
* 
* "What explicitly observable information has the program requested from
*  its permitted semantic, target, resource, capability, deployment, or
*  runtime context?"
* 
* It does NOT mean:
* 
* "the compiler may inspect anything it wants."
* 
* It does NOT mean:
* 
* "hardware details are now language semantics."
* 
* It does NOT mean:
* 
* "the program is tied to one machine."
* 
* It does NOT mean:
* 
* "the parser may access runtime state."
* 
* The language remains target-independent unless the program explicitly and
* semantically requests target/environment-dependent information.
* 
* ============================================================================
  */
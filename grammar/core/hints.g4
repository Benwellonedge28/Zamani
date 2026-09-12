/*

* ============================================================================
* Zamani Universal Programming Language
* ============================================================================
* 
* File:
* grammar/core/hints.g4
* 
* Purpose:
* Canonical parser grammar for source-level implementation hints.
* 
* Language:
* Zamani
* 
* Grammar technology:
* ANTLR4 parser grammar
* 
* Rust implementation baseline:
* Rust 1.97 / Rust 1.97.1
* 
* Safety:
* This grammar contains no embedded Rust actions and requires no `unsafe`.
* The Zamani compiler MUST use safe Rust only.
* 
* ============================================================================
* ARCHITECTURAL ROLE
* ============================================================================
* 
* A hint is non-mandatory source-level information that MAY guide compilation,
* optimization, scheduling, lowering, placement, execution, or another
* implementation decision without becoming part of the required semantic
* meaning of the computation.
* 
* A hint answers:
* 
* "If there are multiple valid realizations, what implementation
*  characteristics would be useful or preferred?"
* 
* A hint MUST NOT become:
* 
* - a requirement;
* - a constraint;
* - a capability declaration;
* - a resource allocation;
* - a target selection;
* - a device selection;
* - a scheduling command;
* - a routing command;
* - an optimization command;
* - a calibration command;
* - a runtime authorization;
* - a quantum operation;
* - a hardware operation.
* 
* ============================================================================
* FUNDAMENTAL SEPARATION
* ============================================================================
* 
* Capability:
* 
* What an environment CAN provide.
* 
* Requirement:
* 
* What a program NEEDS.
* 
* Constraint:
* 
* What a valid realization MUST satisfy.
* 
* Preference:
* 
* Which valid realization is preferred.
* 
* Hint:
* 
* Non-mandatory information that MAY improve realization.
* 
* Resource:
* 
* A computational resource that may be available or requested.
* 
* Target:
* 
* A compilation or execution context.
* 
* The semantic layer MUST preserve these distinctions.
* 
* ============================================================================
* OWNERSHIP
* ============================================================================
* 
* THIS FILE OWNS:
* 
* - generic hint expression syntax;
* - hint subjects;
* - hint values;
* - hint arguments;
* - hint expression grouping;
* - hint lists;
* - optional hint clauses;
* - source-level hint structure;
* - syntactic hint modifiers;
* - open-ended hint identities.
* 
* THIS FILE DOES NOT OWN:
* 
* - lexer tokens;
* - keywords;
* - identifier syntax;
* - qualified-name syntax;
* - general expression syntax;
* - type syntax;
* - capability declarations;
* - requirement semantics;
* - constraint semantics;
* - preference semantics;
* - resource allocation;
* - target selection;
* - hardware discovery;
* - device selection;
* - routing;
* - scheduling;
* - optimization algorithms;
* - calibration;
* - QEC;
* - ZQN;
* - resilience;
* - quantum::ir;
* - classical IR;
* - HDL IR;
* - runtime execution.
* 
* ============================================================================
* OPEN-WORLD DESIGN
* ============================================================================
* 
* Hint identities MUST be open-ended.
* 
* This grammar MUST NOT contain closed enumerations such as:
* 
* cpuHint
* gpuHint
* fpgaHint
* quantumHint
* qpuHint
* vendorHint
* optimizationHint
* schedulingHint
* 
* Instead, hint subjects use canonical names.
* 
* Examples:
* 
* optimization::inline
* optimization::vectorize
* scheduling::critical_path
* execution::low_latency
* placement::locality
* quantum::native_operation
* hardware::pipeline
* future::compute::preferred_mode
* 
* The grammar does not assign semantic meaning to any of these names.
* 
* New domains can therefore introduce hints without changing this grammar.
* 
* ============================================================================
* IMPORTANT LEXER BOUNDARY
* ============================================================================
* 
* The repository currently has one canonical executable lexer:
* 
* grammar/antlr/ZamaniLexer.g4
* 
* Parser grammars consume:
* 
* tokenVocab = ZamaniLexer
* 
* This file MUST NOT introduce a competing lexer.
* 
* In particular, this grammar MUST NOT assume that a "HINT" token exists.
* 
* The source-level activation mechanism for hints is deliberately separated
* from the hint expression itself.
* 
* This permits hints to be introduced through the canonical source attribute,
* annotation, directive, or future explicitly reserved syntax without making
* the semantic hint model dependent on a particular keyword.
* 
* ============================================================================
* ATTRIBUTE / ANNOTATION BOUNDARY
* ============================================================================
* 
* General attribute and annotation syntax belongs to the canonical core
* attribute/annotation grammars.
* 
* This file MUST NOT redefine:
* 
* #[ ... ]
* @name(...)
* 
* syntax.
* 
* A parser composition layer MAY embed "hintExpression" or "hintClause"
* inside the canonical attribute/directive grammar.
* 
* The hint grammar therefore remains independently usable without creating
* another lexical keyword.
* 
* ============================================================================
* NAME BOUNDARY
* ============================================================================
* 
* Name syntax belongs to:
* 
* grammar/core/names.g4
* grammar/core/qualified-names.g4
* 
* This file consumes:
* 
* qualifiedName
* 
* and MUST NOT redefine:
* 
* identifier
* simpleName
* qualifiedName
* 
* ============================================================================
* VALUE BOUNDARY
* ============================================================================
* 
* Hints need values, but this grammar MUST NOT create a second expression
* language.
* 
* The canonical expression grammar owns:
* 
* literals
* identifiers
* arithmetic
* calls
* indexing
* member access
* ranges
* logical expressions
* other expression forms
* 
* Hint values are therefore represented through an integration rule:
* 
* hintValue
* 
* which the canonical parser composition layer binds to the appropriate
* expression/literal representation.
* 
* The semantic meaning of the value belongs downstream.
* 
* ============================================================================
* SEMANTIC CHARACTER OF HINTS
* ============================================================================
* 
* A hint is advisory.
* 
* A valid implementation MAY:
* 
* - honor it;
* - partially honor it;
* - transform it;
* - ignore it;
* - report that it cannot be honored.
* 
* A hint MUST NOT invalidate a semantically valid program merely because a
* particular implementation cannot honor it, unless another explicit
* semantic construct separately establishes a mandatory condition.
* 
* Therefore:
* 
* hint optimization::vectorize = true
* 
* MUST NOT be equivalent to:
* 
* requires optimization::vectorize
* 
* and MUST NOT be equivalent to:
* 
* where optimization::vectorize == true
* 
* ============================================================================
* POCO-REAF
* ============================================================================
* 
* Hints support:
* 
* Program Once
*     ->
* Compile Once
*     ->
* Run Everywhere
*     ->
* Run Anywhere
*     ->
* Run Forever
* 
* by allowing source authors to communicate useful implementation intent
* without making the source dependent on a particular machine.
* 
* For example, a program MAY express:
* 
* optimization::vectorize
* execution::low_latency
* scheduling::parallel
* placement::locality
* quantum::native_operation
* 
* without specifying:
* 
* CPU model
* GPU identifier
* QPU identifier
* FPGA identifier
* device address
* physical qubit
* topology
* cluster size
* memory capacity
* 
* A realization may therefore honor the same hint differently on different
* available machines.
* 
* ============================================================================
* SCALABILITY
* ============================================================================
* 
* This grammar imposes NO language-level finite limit on:
* 
* - number of hints;
* - number of hint arguments;
* - number of hint expressions;
* - number of hint alternatives;
* - number of nested groups;
* - qualified-name depth;
* - identifier size;
* - program size;
* - number of computational domains;
* - number of resources;
* - number of devices;
* - number of qubits;
* - number of nodes.
* 
* Repetition uses:
* 
* *
* +
* 
* rather than fixed cardinalities.
* 
* Practical parser, memory, source-size, operating-system, or deployment
* limits are implementation/resource limits and MUST NOT become language
* semantics.
* 
* ============================================================================
* HARDWARE INDEPENDENCE
* ============================================================================
* 
* This grammar MUST NOT encode:
* 
* cpu0
* gpu0
* qpu7
* fpga3
* device "..."
* fixed topology
* fixed accelerator count
* fixed memory size
* fixed qubit count
* fixed node count
* 
* A hint may describe a desired property:
* 
* hardware::locality
* 
* but not prescribe a physical realization.
* 
* Physical realization belongs to target, resource, placement, scheduling,
* routing, hardware abstraction, and runtime subsystems.
* 
* ============================================================================
* QUANTUM BOUNDARY
* ============================================================================
* 
* Quantum hints are source-level advisory identities.
* 
* Examples:
* 
* quantum::native_operation
* quantum::low_depth
* quantum::measurement_locality
* quantum::parallel_execution
* 
* The grammar MUST NOT import:
* 
* quantum::ir
* QubitId
* PhysicalQubitId
* GateKind
* topology
* calibration
* 
* A quantum hint may influence downstream compilation policy, but it does not
* become a quantum operation.
* 
* The canonical quantum semantic boundary remains:
* 
* quantum::ir
* 
* ============================================================================
* HDL / HARDWARE BOUNDARY
* ============================================================================
* 
* Hardware and HDL hints MAY describe implementation preferences such as:
* 
* hardware::pipeline
* hardware::resource_sharing
* hardware::locality
* timing::latency
* 
* but MUST NOT directly encode a particular FPGA, ASIC, clock network,
* physical placement, or synthesis implementation.
* 
* Such details belong downstream.
* 
* ============================================================================
* DISTRIBUTED BOUNDARY
* ============================================================================
* 
* Distributed hints MAY describe advisory properties such as:
* 
* distributed::locality
* distributed::parallelism
* communication::batching
* communication::coalescing
* 
* They MUST NOT select a particular node, host, cluster, address, or provider.
* 
* ============================================================================
* AI / DATA BOUNDARY
* ============================================================================
* 
* Hints MAY refer to open-ended computational properties such as:
* 
* ai::batching
* ai::memory_locality
* data::streaming
* data::parallelism
* 
* The grammar MUST NOT hard-code tensor sizes, accelerator counts, memory
* capacities, or model dimensions.
* 
* ============================================================================
* BOOLEAN COMPOSITION
* ============================================================================
* 
* Hints may be composed when the semantic model supports advisory
* alternatives.
* 
* The parser preserves:
* 
* and
* or
* not
* grouping
* 
* without deciding how an implementation should rank or apply them.
* 
* Example:
* 
* optimization::vectorize
*     and
* optimization::parallel
* 
* or:
* 
* execution::low_latency
*     or
* execution::throughput
* 
* The semantic layer decides whether the combination is meaningful.
* 
* ============================================================================
* NEGATION
* ============================================================================
* 
* "not" is syntactic.
* 
* This grammar does not decide whether:
* 
* not optimization::vectorize
* 
* means:
* 
* - avoid vectorization;
* - prefer a non-vectorized realization;
* - lower the priority of vectorization;
* - express an advisory exclusion.
* 
* The semantic hint policy defines that interpretation.
* 
* ============================================================================
* HINT ARGUMENTS
* ============================================================================
* 
* Hints may optionally carry arguments.
* 
* Generic structure:
* 
* hint-name
* 
* or:
* 
* hint-name(argument, ...)
* 
* or:
* 
* hint-name = value
* 
* or:
* 
* hint-name(value)
* 
* The exact semantic interpretation is not defined here.
* 
* Arguments are values supplied to downstream semantic analysis.
* 
* ============================================================================
* HINT VALUES
* ============================================================================
* 
* Values MUST remain open to the canonical expression system.
* 
* A hint may eventually accept:
* 
* boolean
* integer
* floating-point
* string
* character
* identifier
* qualified name
* enum-like symbolic value
* expression
* range
* collection
* domain-specific value
* 
* without this grammar having to enumerate every future domain.
* 
* ============================================================================
* NO EXECUTION
* ============================================================================
* 
* Hint arguments are data.
* 
* Parsing a hint MUST NOT:
* 
* - execute code;
* - invoke functions;
* - perform I/O;
* - access files;
* - access networks;
* - inspect hardware;
* - inspect runtime state;
* - allocate runtime resources;
* - alter compiler global state.
* 
* Compile-time evaluation, if supported, belongs to the metaprogramming or
* compile-time execution subsystem and MUST be explicitly controlled there.
* 
* ============================================================================
* DETERMINISM
* ============================================================================
* 
* This grammar contains:
* 
* - no semantic predicates;
* - no embedded actions;
* - no filesystem access;
* - no network access;
* - no hardware discovery;
* - no runtime calls;
* - no randomness.
* 
* Parsing therefore depends only on the token stream.
* 
* ============================================================================
* SOURCE PRESERVATION
* ============================================================================
* 
* The frontend AST MUST preserve:
* 
* - hint ordering;
* - hint identity;
* - qualified-name segment order;
* - argument ordering;
* - argument expression structure;
* - boolean composition;
* - grouping;
* - negation;
* - source spans.
* 
* Semantic normalization belongs downstream.
* 
* ============================================================================
* AST CONTRACT
* ============================================================================
* 
* Conceptual AST:
* 
* Hint
*     {
*         subject,
*         arguments,
*         value,
*         expression,
*         source_span
*     }
* 
* HintExpression:
* 
* HintReference(...)
* HintApplication(...)
* HintAssignment(...)
* HintNot(...)
* HintAll(...)
* HintAny(...)
* HintGroup(...)
* 
* The exact Rust representation belongs to the frontend AST subsystem.
* 
* This grammar MUST NOT embed Rust structures.
* 
* ============================================================================
* SEMANTIC CONTRACT
* ============================================================================
* 
* Semantic analysis determines:
* 
* - whether the hint identity is known;
* - whether it is registered;
* - whether its arguments are valid;
* - whether its values have valid types;
* - whether the hint is applicable;
* - how advisory priority is interpreted;
* - whether the hint conflicts with another hint;
* - whether the hint is deprecated;
* - whether the implementation can honor the hint.
* 
* Unknown hints SHOULD remain syntactically representable.
* 
* Depending on the active language compatibility policy, an unknown hint may
* produce:
* 
* informational diagnostic;
* warning;
* compatibility diagnostic;
* or semantic error.
* 
* That policy MUST NOT be encoded as a grammar decision.
* 
* ============================================================================
* HINT VS REQUIREMENT
* ============================================================================
* 
* This distinction is mandatory.
* 
* Hint:
* 
* advisory.
* 
* Requirement:
* 
* mandatory for valid realization.
* 
* Therefore:
* 
* hint execution::low_latency
* 
* MUST NOT imply:
* 
* requires execution::low_latency
* 
* Likewise:
* 
* hint quantum::native_operation
* 
* MUST NOT imply:
* 
* requires quantum::native_operation
* 
* A compiler may ignore a hint while still producing a semantically valid
* realization, subject to diagnostics and language policy.
* 
* ============================================================================
* HINT VS CONSTRAINT
* ============================================================================
* 
* A constraint expresses a mandatory condition.
* 
* A hint expresses advisory information.
* 
* Therefore:
* 
* hint hardware::locality
* 
* MUST NOT be interpreted as:
* 
* where hardware::locality == true
* 
* unless the program separately declares such a constraint.
* 
* ============================================================================
* HINT VS PREFERENCE
* ============================================================================
* 
* A preference and a hint are related but not identical.
* 
* A preference participates in choosing between valid alternatives.
* 
* A hint may provide implementation guidance without establishing an ordering
* over alternatives.
* 
* The semantic layer MUST therefore retain the distinction even if a specific
* optimizer eventually converts a hint into a weighted preference.
* 
* ============================================================================
* HINT VS RESOURCE
* ============================================================================
* 
* A hint may describe desired resource behavior:
* 
* resource::locality
* 
* but it does not allocate a resource.
* 
* It MUST NOT specify:
* 
* number of CPUs;
* number of GPUs;
* number of QPUs;
* fixed memory allocation;
* fixed node allocation;
* device IDs.
* 
* Resource realization belongs to the resource and execution subsystems.
* 
* ============================================================================
* HINT VS TARGET
* ============================================================================
* 
* A hint may guide target realization but MUST NOT identify a target.
* 
* For example:
* 
* hint execution::low_latency
* 
* does not mean:
* 
* target provider X.
* 
* Target selection belongs to the target/compilation/execution systems.
* 
* ============================================================================
* COMPILER CONTRACT
* ============================================================================
* 
* Compilation MAY consume hints to:
* 
* - select among semantically equivalent lowerings;
* - prioritize optimization strategies;
* - guide scheduling;
* - guide routing;
* - guide placement;
* - guide code generation;
* - guide accelerator mapping;
* - guide quantum compilation;
* - guide HDL synthesis;
* - guide distributed realization.
* 
* Compilation MUST NOT assume that every hint can be honored.
* 
* Ignoring a hint MUST NOT silently change the program's semantic meaning.
* 
* ============================================================================
* SCHEDULING CONTRACT
* ============================================================================
* 
* Scheduling may consume scheduling-related hints.
* 
* Examples:
* 
* scheduling::parallel
* scheduling::critical_path
* execution::low_latency
* 
* The hint grammar does not schedule operations.
* 
* Scheduling remains responsible for:
* 
* ordering;
* timing;
* resource conflicts;
* dependencies;
* alignment;
* execution feasibility.
* 
* ============================================================================
* OPTIMIZATION CONTRACT
* ============================================================================
* 
* Optimization may consume optimization hints.
* 
* Examples:
* 
* optimization::inline
* optimization::vectorize
* optimization::common_subexpression
* 
* This grammar does not define optimization algorithms.
* 
* Optimization remains downstream from semantic representation.
* 
* ============================================================================
* QUANTUM IR CONTRACT
* ============================================================================
* 
* A hint MUST NOT become a "quantum::ir" operation merely because the hint
* mentions quantum computation.
* 
* Correct direction:
* 
* Zamani source
*      |
*      v
* hint AST
*      |
*      v
* semantic hint model
*      |
*      v
* compilation policy
*      |
*      v
* quantum semantic lowering
*      |
*      v
* quantum::ir
* 
* Incorrect direction:
* 
* hint grammar
*      |
*      v
* quantum::ir operation
* 
* ============================================================================
* RUNTIME CONTRACT
* ============================================================================
* 
* Runtime MAY consume preserved hints when selecting among valid execution
* strategies.
* 
* Runtime MUST NOT treat an advisory hint as authorization.
* 
* Runtime MUST NOT reinterpret a hint as a mandatory requirement unless an
* explicit semantic transformation has established that policy.
* 
* ============================================================================
* RESILIENCE CONTRACT
* ============================================================================
* 
* Resilience MAY consume hints when choosing between valid recovery or
* adaptation strategies.
* 
* This grammar MUST NOT implement:
* 
* retry;
* rollback;
* backend switching;
* quarantine;
* recovery;
* mitigation;
* fault diagnosis.
* 
* Those responsibilities belong to the resilience and execution layers.
* 
* ============================================================================
* SECURITY CONTRACT
* ============================================================================
* 
* Hints are not permissions.
* 
* For example:
* 
* security::secure_execution
* 
* may express an advisory implementation property, but it does not grant
* access to protected resources.
* 
* Authorization belongs to the security subsystem.
* 
* ============================================================================
* IR CONTRACT
* ============================================================================
* 
* Hints do not directly lower into computational operations.
* 
* They should normally survive semantic analysis as:
* 
* semantic metadata
* compilation policy
* optimization metadata
* scheduling metadata
* target-realization guidance
* 
* where required.
* 
* They MUST NOT contaminate the canonical operation semantics of:
* 
* classical IR;
* quantum::ir;
* HDL IR;
* 
* merely because an implementation consumes them.
* 
* ============================================================================
* COMPATIBILITY CONTRACT
* ============================================================================
* 
* Existing source constructs that already carry advisory information MUST be
* migrated deliberately.
* 
* The migration order is:
* 
* existing syntax
*     ->
* identify advisory semantics
*     ->
* map to Hint AST
*     ->
* preserve source compatibility where practical
*     ->
* emit explicit migration diagnostics where necessary.
* 
* No existing valid syntax may be silently reinterpreted from advisory to
* mandatory behavior.
* 
* ============================================================================
* PUBLIC RULES
* ============================================================================
* 
* The following rules are intended as parser-composition integration points:
* 
* hintExpression
* hintOrExpression
* hintAndExpression
* hintUnaryExpression
* hintPrimary
* hintReference
* hintApplication
* hintAssignment
* hintArgumentList
* hintList
* optionalHintList
* hintClause
* 
* The exact source activation mechanism is supplied by the canonical parser
* composition layer.
* 
* ============================================================================
* ANTLR CONTRACT
* ============================================================================
* 
* This is a parser grammar.
* 
* It consumes:
* 
* tokenVocab = ZamaniLexer
* 
* It MUST NOT define lexer tokens.
* 
* ============================================================================
  */

parser grammar Hints;

options {
tokenVocab = ZamaniLexer;
}

/* ============================================================================

* HINT CLAUSE
* ============================================================================
* 
* This is intentionally a parser-level integration rule rather than a rule
* requiring a new HINT lexer keyword.
* 
* The canonical source grammar may embed this rule in the appropriate
* attribute/directive/metadata construct.
* 
* The semantic model is therefore independent of the spelling used by the
* outer source construct.
  */
  hintClause
  : hintExpression
  ;

/* ============================================================================

* HINT EXPRESSION
* ============================================================================
* 
* Precedence:
* 
* OR
*   >
* AND
*   >
* NOT
*   >
* primary
* 
* This preserves deterministic structural grouping.
  */
  hintExpression
  : hintOrExpression
  ;

hintOrExpression
: hintAndExpression
(OR hintAndExpression)*
;

hintAndExpression
: hintUnaryExpression
(AND hintUnaryExpression)*
;

hintUnaryExpression
: NOT hintUnaryExpression
| hintPrimary
;

hintPrimary
: LPAREN hintExpression RPAREN
| hintAssignment
| hintApplication
| hintReference
;

/* ============================================================================

* HINT REFERENCE
* ============================================================================
* 
* Examples:
* 
* optimization::vectorize
* execution::low_latency
* quantum::native_operation
* 
* Names are open-ended.
  */
  hintReference
  : qualifiedName
  ;

/* ============================================================================

* HINT APPLICATION
* ============================================================================
* 
* Examples:
* 
* optimization::vectorize(true)
* scheduling::parallel(mode)
* 
* Argument semantics are downstream.
  */
  hintApplication
  : qualifiedName
  LPAREN
  hintArgumentList?
  RPAREN
  ;

hintArgumentList
: hintArgument
(COMMA hintArgument)*
;

hintArgument
: hintValue
;

/* ============================================================================

* HINT ASSIGNMENT
* ============================================================================
* 
* Example:
* 
* execution::priority = value
* 
* "=" here is intentionally represented as advisory hint data rather than
* ordinary program assignment. The containing parser rule determines context.
  */
  hintAssignment
  : qualifiedName
  ASSIGN
  hintValue
  ;

/* ============================================================================

* HINT VALUE
* ============================================================================
* 
* The canonical expression subsystem owns expression semantics.
* 
* This generic boundary intentionally accepts the canonical expression rule
* rather than recreating literals, operators, calls, indexing, or ranges.
* 
* The canonical parser composition layer MUST bind "expression" to the
* repository's authoritative expression production.
  */
  hintValue
  : expression
  ;

/* ============================================================================

* HINT LIST
* ============================================================================
* 
* There is no fixed maximum.
  /
  hintList
  : hintExpression
  (COMMA hintExpression)
  ;

optionalHintList
: hintList?
;

/* ============================================================================

* HINT DECLARATION BODY
* ============================================================================
* 
* This rule is useful when an outer declaration/attribute has already
* established that its payload is a hint.
* 
* It deliberately does not introduce a new lexical keyword.
  */
  hintDeclarationBody
  : hintList
  ;

/* ============================================================================

* SINGLE HINT
* ============================================================================
* 
* Useful integration boundary for attributes, metadata, directives, or
* extension declarations.
  */
  hint
  : hintExpression
  ;

/* ============================================================================

* END
* ============================================================================
  */
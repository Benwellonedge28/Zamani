/*

* ============================================================================
* Zamani Universal Programming Language
* ============================================================================
* 
* File:
* grammar/compile/target.g4
* 
* Grammar:
* CompileTarget
* 
* Status:
* Production target-intent parser grammar
* 
* ============================================================================
* PURPOSE
* ============================================================================
* 
* This grammar owns SOURCE-LEVEL TARGET INTENT.
* 
* It describes the semantic properties of an acceptable execution
* realization without turning today's hardware inventory into permanent
* Zamani language syntax.
* 
* The central distinction is:
* 
* target intent
*      !=
* physical device selection
* 
* A target declaration may describe:
* 
* - target identity or family;
* - target alternatives;
* - requirements;
* - constraints;
* - capabilities;
* - preferences;
* - hints;
* - profiles;
* - portability;
* - scalability;
* - resource-related intent;
* - fallback/alternative policy;
* - target properties.
* 
* It MUST NOT perform:
* 
* - hardware discovery;
* - device allocation;
* - physical placement;
* - routing;
* - scheduling;
* - calibration;
* - QEC;
* - ZQN analysis;
* - backend execution;
* - runtime dispatch.
* 
* ============================================================================
* ARCHITECTURAL POSITION
* ============================================================================
* 
* Zamani source
*      |
*      v
* ZamaniLexer
*      |
*      v
* ZamaniParser
*      |
*      v
* CompileTarget
*      |
*      v
* domain-neutral frontend AST
*      |
*      v
* semantic analysis
*      |
*      +--> target intent validation
*      +--> resource/capability analysis
*      +--> portability analysis
*      +--> target resolution
*      |
*      v
* canonical semantic model / IR
*      |
*      +--> classical representation
*      +--> quantum::ir
*      +--> HDL/hardware representation
*      +--> distributed representation
*      |
*      v
* optimization
*      |
*      +--> routing
*      +--> scheduling
*      +--> resilience
*      +--> QEC
*      +--> ZQN
*      |
*      v
* HAL / backend / deployment
* 
* ============================================================================
* POCO-REAF
* ============================================================================
* 
* Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
* 
* The source program describes semantic requirements and acceptable
* realizations.
* 
* The compiler and execution infrastructure determine how those requirements
* are realized on the resources actually available.
* 
* This grammar therefore contains NO universal limits such as:
* 
* MAX_QUBITS
* MAX_CPUS
* MAX_CORES
* MAX_THREADS
* MAX_GPUS
* MAX_FPGAS
* MAX_ASICS
* MAX_QPUS
* MAX_NODES
* MAX_DEVICES
* MAX_MEMORY
* MAX_STORAGE
* MAX_REGISTER_WIDTH
* MAX_TENSOR_RANK
* MAX_TENSOR_DIMENSION
* MAX_TOPOLOGY_SIZE
* MAX_TARGETS
* MAX_CAPABILITIES
* 
* "Infinity" means that the language does not impose an artificial finite
* hardware ceiling. Actual execution remains constrained by resources,
* capabilities, compiler capacity, policies, and physical reality.
* 
* ============================================================================
* LEXICAL AUTHORITY
* ============================================================================
* 
* This grammar consumes ONLY the canonical:
* 
* ZamaniLexer
* 
* through:
* 
* tokenVocab = ZamaniLexer;
* 
* It does NOT introduce lexer rules.
* 
* The actual repository lexer already provides the relevant stable vocabulary,
* including:
* 
* TARGET
* PROFILE
* REQUIRES
* CONSTRAINT
* PREFER
* HINT
* CAPABILITY
* PROPERTY
* PORTABILITY
* SCALABILITY
* RESOURCE
* RESOURCES
* CAPACITY
* AVAILABILITY
* PERFORMANCE
* LATENCY
* THROUGHPUT
* BANDWIDTH
* ENERGY
* POWER
* RELIABILITY
* RESILIENCE
* COST
* RESERVE
* ACQUIRE
* RELEASE
* DERIVE
* GROUP
* CONTRACT
* 
* Operators and punctuation are likewise consumed from the canonical lexer.
* 
* This file deliberately does NOT invent tokens such as:
* 
* TARGET_FAMILY
* TARGET_REQUIRE
* TARGET_CONSTRAIN
* TARGET_PREFER
* TARGET_HINT
* TARGET_CAPABILITY
* TARGET_PROPERTY
* TARGET_ALTERNATIVE
* TARGET_FALLBACK
* TARGET_PORTABLE
* 
* Such invented token names would create an integration failure because they
* are not part of the repository's actual canonical lexical vocabulary.
* 
* ============================================================================
* OWNERSHIP
* ============================================================================
* 
* THIS FILE OWNS:
* 
* - target declaration syntax;
* - target designators;
* - target expressions;
* - target alternatives;
* - target blocks;
* - target profiles;
* - target requirements;
* - target constraints;
* - target preferences;
* - target hints;
* - target capability references;
* - target portability/scalability intent;
* - target resource-related intent;
* - target fallback/alternative properties;
* - target semantic properties.
* 
* THIS FILE DOES NOT OWN:
* 
* - lexical definitions;
* - identifiers;
* - qualified names;
* - ordinary expressions;
* - types;
* - general resource grammar;
* - hardware declarations;
* - hardware inventories;
* - physical devices;
* - topology realization;
* - placement;
* - routing;
* - scheduling;
* - optimization;
* - quantum operations;
* - quantum::ir;
* - classical IR;
* - HDL IR;
* - QEC;
* - ZQN;
* - resilience implementation;
* - HAL;
* - runtime execution.
* 
* ============================================================================
* RELATIONSHIP TO OTHER EXISTING FILES
* ============================================================================
* 
* grammar/compile/compile.g4
* Owns general compilation intent.
* 
* grammar/compile/target.g4
* Owns target intent.
* 
* grammar/resources/
* Owns general resource/capability semantics.
* 
* grammar/hardware/
* Owns hardware-description and hardware-intent syntax.
* 
* grammar/execution/
* Owns execution/deployment intent.
* 
* grammar/quantum/
* Owns quantum source semantics.
* 
* grammar/validation/
* Owns static conformance and hard-coding validation.
* 
* grammar/spec/portability.md
* Defines normative portability semantics.
* 
* grammar/spec/resources.md
* Defines normative resource semantics.
* 
* grammar/Zamani.g4
* Remains the canonical complete-program composition root.
* 
* grammar/antlr/ZamaniParser.g4
* Remains the canonical parser composition root.
* 
* quantum::ir
* Remains the canonical quantum semantic boundary.
* 
* ============================================================================
* IMPORTANT INTEGRATION RULE
* ============================================================================
* 
* Target declarations are part of the compilation/realization-intent layer.
* 
* The canonical parser composition should therefore expose:
* 
* compileElement
*     |
*     +--> compileDeclaration
*     +--> targetDeclaration
*     +--> compileStatement
*     +--> compileExpression
* 
* The exact integration point is owned by the Compile dispatcher rather than
* by this grammar.
* 
* This grammar must remain independently generatable.
* 
* ============================================================================
* SAFETY
* ============================================================================
* 
* This file contains:
* 
* - no embedded Rust;
* - no semantic actions;
* - no predicates;
* - no filesystem access;
* - no network access;
* - no environment access;
* - no hardware discovery;
* - no runtime execution;
* - no unsafe code.
* 
* The consuming compiler/frontend remains compatible with:
* 
* Rust 2021
* Rust 1.97
* Rust 1.97.1
* 
* ============================================================================
  */

/*

* ============================================================================
* ANTLR GRAMMAR DECLARATION
* ============================================================================
  */

parser grammar CompileTarget;

options {
tokenVocab = ZamaniLexer;
}

/*

* ============================================================================
* 1. PUBLIC TARGET DECLARATION
* ============================================================================
* 
* Canonical form:
* 
* target <target-expression>;
* 
* or:
* 
* target <target-expression> {
*     ...
* }
* 
* The target expression identifies an acceptable semantic target class or
* target composition.
* 
* It does NOT identify a physical device unless the programmer explicitly
* makes a concrete identity part of the source-level contract.
  */

targetDeclaration
: TARGET targetExpression targetBody? SEMI?
;

/*

* ============================================================================
* 2. TARGET BODY
* ============================================================================
* 
* A target body contains zero or more independent intent clauses.
* 
* No finite number of entries is imposed.
* 
* Each category remains structurally distinct so that the AST can preserve
* requirement/constraint/preference/hint semantics instead of collapsing
* everything into a generic option map.
  */

targetBody
: LBRACE targetEntry* RBRACE
;

targetEntry
: targetRequirement
| targetConstraint
| targetPreference
| targetHint
| targetCapability
| targetProfileReference
| targetPortability
| targetScalability
| targetResourceIntent
| targetProperty
| targetAlternative
;

/*

* ============================================================================
* 3. TARGET EXPRESSION
* ============================================================================
* 
* Target expressions describe acceptable semantic realizations.
* 
* They are intentionally open-ended.
* 
* There is NO grammar enumeration such as:
* 
* CPU
* GPU
* FPGA
* ASIC
* QPU
* 
* Those are names/families interpreted by semantic analysis.
* 
* The expression hierarchy provides deterministic precedence:
* 
* union
*   > intersection
*     > difference
*       > primary
* 
* The concrete operator spellings use the repository's existing lexical
* vocabulary:
* 
* PIPE
* AMPERSAND
* MINUS
* 
* No target-specific operator token is introduced.
  */

targetExpression
: targetUnionExpression
;

targetUnionExpression
: targetIntersectionExpression
(PIPE targetIntersectionExpression)*
;

targetIntersectionExpression
: targetDifferenceExpression
(AMPERSAND targetDifferenceExpression)*
;

targetDifferenceExpression
: targetPrimary
(MINUS targetPrimary)*
;

targetPrimary
: targetName
| targetSet
| targetExpressionGroup
;

targetExpressionGroup
: LPAREN targetExpression RPAREN
;

/*

* ============================================================================
* 4. TARGET NAME
* ============================================================================
* 
* Names remain semantic identifiers.
* 
* Examples:
* 
* cpu
* gpu
* quantum
* accelerator
* vendor.operation
* quantum.simulator
* future.architecture
* 
* The grammar does not decide what those names mean.
* 
* "qualifiedName" is consumed from the canonical parser composition rather
* than being redefined here.
  */

targetName
: qualifiedName
| STRING
;

/*

* ============================================================================
* 5. TARGET SET
* ============================================================================
* 
* A target set describes multiple acceptable target expressions.
* 
* Cardinality is unbounded at the language level.
  */

targetSet
: LBRACKET targetExpressionList? RBRACKET
;

targetExpressionList
: targetExpression
(COMMA targetExpression)*
;

/*

* ============================================================================
* 6. TARGET REQUIREMENT
* ============================================================================
* 
* A requirement is mandatory semantic intent.
* 
* Examples:
* 
* target quantum requires capability("quantum.measurement");
* 
* target accelerator {
*     requires qubits >= n;
* }
* 
* The expression is not evaluated by the parser.
* 
* Resource/capability satisfiability is determined downstream.
  */

targetRequirement
: REQUIRES expression SEMI?
;

/*

* ============================================================================
* 7. TARGET CONSTRAINT
* ============================================================================
* 
* A constraint limits acceptable realization without identifying a physical
* implementation.
* 
* Examples:
* 
* constraint latency <= bound;
* constraint reliability >= required;
* constraint topology_requirement;
* 
* The exact semantic meaning belongs to semantic/resource analysis.
  */

targetConstraint
: CONSTRAINT targetIntentValue SEMI?
;

targetIntentValue
: expression
| targetPropertyBlock
;

/*

* ============================================================================
* 8. TARGET PREFERENCE
* ============================================================================
* 
* A preference is optional guidance.
* 
* It MUST NOT silently become a mandatory requirement.
* 
* A backend is allowed to ignore a preference when necessary to preserve
* correctness or satisfy stronger constraints.
  */

targetPreference
: PREFER targetIntentValue SEMI?
;

/*

* ============================================================================
* 9. TARGET HINT
* ============================================================================
* 
* A hint provides non-authoritative implementation guidance.
* 
* It has no semantic force by itself.
  */

targetHint
: HINT targetIntentValue SEMI?
;

/*

* ============================================================================
* 10. TARGET CAPABILITY
* ============================================================================
* 
* Capability declarations/references describe semantic abilities expected from
* an acceptable realization.
* 
* The capability name remains open-ended.
* 
* Examples:
* 
* capability("quantum.measurement")
* capability("tensor.compute")
* capability("gpu.compute")
* 
* No finite capability enumeration belongs in this grammar.
  */

targetCapability
: CAPABILITY targetCapabilityValue SEMI?
;

targetCapabilityValue
: expression
| targetCapabilityReference
;

targetCapabilityReference
: targetName
;

/*

* ============================================================================
* 11. TARGET PROFILE REFERENCE
* ============================================================================
* 
* A profile is a reusable semantic target-intent contract.
* 
* Example:
* 
* target quantum {
*     profile portable_quantum;
* }
* 
* Profile definitions themselves are declarations and should be integrated
* through the compile subsystem.
* 
* This rule handles the reference form inside a target body.
  */

targetProfileReference
: PROFILE targetName SEMI?
;

/*

* ============================================================================
* 12. TARGET PORTABILITY
* ============================================================================
* 
* Portability expresses the intended portability boundary.
* 
* It does not prove portability.
* 
* Proof/validation is performed by semantic analysis and downstream
* conformance machinery.
  */

targetPortability
: PORTABILITY targetIntentValue SEMI?
;

/*

* ============================================================================
* 13. TARGET SCALABILITY
* ============================================================================
* 
* Scalability describes semantic scaling intent.
* 
* It does not define a finite machine size.
* 
* Examples of semantic properties that may be represented downstream:
* 
* scalable
* elastic
* resource_adaptive
* distributed
* parameterized
* 
* The parser only preserves the source construct.
  */

targetScalability
: SCALABILITY targetIntentValue SEMI?
;

/*

* ============================================================================
* 14. TARGET RESOURCE INTENT
* ============================================================================
* 
* General resource concepts already have lexical vocabulary in the repository.
* 
* This rule deliberately remains broad enough to avoid duplicating the
* resource subsystem.
* 
* Examples:
* 
* resource ...
* resources ...
* capacity ...
* availability ...
* performance ...
* latency ...
* throughput ...
* bandwidth ...
* energy ...
* power ...
* reliability ...
* resilience ...
* cost ...
* reserve ...
* acquire ...
* release ...
* 
* Their semantic ownership remains in grammar/resources/ and downstream
* resource analysis.
  */

targetResourceIntent
: targetResourceKeyword targetIntentValue SEMI?
;

targetResourceKeyword
: RESOURCE
| RESOURCES
| CAPACITY
| AVAILABILITY
| PERFORMANCE
| LATENCY
| THROUGHPUT
| BANDWIDTH
| ENERGY
| POWER
| RELIABILITY
| RESILIENCE
| COST
| RESERVE
| ACQUIRE
| RELEASE
;

/*

* ============================================================================
* 15. TARGET PROPERTY
* ============================================================================
* 
* Generic property syntax is the main extensibility mechanism.
* 
* This avoids adding a new universal keyword every time a new computing
* architecture introduces a semantic target property.
* 
* Examples:
* 
* property topology = ...;
* property precision = ...;
* property architecture = ...;
* property vendor = ...;
* property fallback = ...;
* 
* A property is data until semantic analysis assigns it a defined language
* meaning.
* 
* Unknown properties may be diagnosed according to the semantic/dialect
* policy rather than silently receiving backend-specific behavior.
  */

targetProperty
: PROPERTY targetPropertyName targetPropertyValue? SEMI?
;

targetPropertyName
: identifier
| qualifiedName
;

targetPropertyValue
: ASSIGN expression
| COLON expression
| targetPropertyBlock
;

targetPropertyBlock
: LBRACE targetPropertyEntry* RBRACE
;

targetPropertyEntry
: targetPropertyName
(ASSIGN | COLON)
expression
SEMI?
;

/*

* ============================================================================
* 16. TARGET ALTERNATIVE
* ============================================================================
* 
* The existing lexical vocabulary does not reserve a TARGET_ALTERNATIVE or
* TARGET_FALLBACK token.
* 
* Therefore alternative/fallback policy is intentionally represented as a
* semantic property rather than inventing new lexer tokens.
* 
* Example:
* 
* property fallback = [quantum, classical];
* 
* or:
* 
* property alternative = [gpu, cpu];
* 
* The semantic layer determines whether the alternatives are actually
* equivalent and whether fallback is legal.
  */

targetAlternative
: PROPERTY targetAlternativeName targetAlternativeValue SEMI?
;

targetAlternativeName
: identifier
| qualifiedName
;

targetAlternativeValue
: ASSIGN targetAlternativeExpression
| COLON targetAlternativeExpression
;

targetAlternativeExpression
: targetExpression
| targetSet
| expression
;

/*

* ============================================================================
* 17. TARGET PROFILE DECLARATION
* ============================================================================
* 
* Profiles are reusable target-intent definitions.
* 
* They are intentionally not physical machine configurations.
* 
* Canonical conceptual form:
* 
* profile portable_quantum {
*     requires capability("quantum.measurement");
*     prefer ...
* }
* 
* The profile declaration is exposed here so Compile can compose it without
* duplicating its syntax.
* 
* This does not mean that every PROFILE occurrence is a declaration:
* targetProfileReference remains the nested reference form.
  */

targetProfileDeclaration
: PROFILE targetProfileName targetProfileBody
;

targetProfileName
: identifier
| qualifiedName
;

targetProfileBody
: LBRACE targetProfileEntry* RBRACE
;

targetProfileEntry
: targetRequirement
| targetConstraint
| targetPreference
| targetHint
| targetCapability
| targetPortability
| targetScalability
| targetResourceIntent
| targetProperty
| targetAlternative
;

/*

* ============================================================================
* 18. TARGET CONTRACT
* ============================================================================
* 
* "contract" is already part of the repository lexical vocabulary.
* 
* A target contract is source-level semantic intent.
* 
* It is not a hardware ABI or executable contract.
  */

targetContract
: CONTRACT targetContractBody
;

targetContractBody
: targetIntentValue
| LBRACE targetProfileEntry* RBRACE
;

/*

* ============================================================================
* 19. TARGET GROUP
* ============================================================================
* 
* "group" is an existing lexical keyword.
* 
* This provides a named semantic grouping without enumerating physical
* hardware categories in the universal grammar.
  */

targetGroup
: GROUP targetName targetGroupBody?
;

targetGroupBody
: LBRACE targetEntry* RBRACE
;

/*

* ============================================================================
* 20. TARGET DERIVATION
* ============================================================================
* 
* "derive" is already part of the language vocabulary.
* 
* Derivation expresses semantic target intent, not code generation.
* 
* Example conceptual form:
* 
* derive target_name from target_expression;
* 
* The exact semantic rules belong to target/profile analysis.
  */

targetDerivation
: DERIVE targetName targetDerivationSource? SEMI?
;

targetDerivationSource
: FROM targetExpression
| ASSIGN targetExpression
;

/*

* ============================================================================
* 21. TARGET VALUE
* ============================================================================
* 
* All value-bearing positions eventually use the canonical expression rule.
* 
* This is critical:
* 
* target.g4
* 
* MUST NOT define another expression grammar.
* 
* Therefore:
* 
* arithmetic
* comparisons
* function calls
* indexing
* member access
* ranges
* literals
* generic expressions
* 
* remain owned by the canonical expression subsystem.
  */

targetValue
: expression
| targetPropertyBlock
;

/*

* ============================================================================
* 22. TARGET PROFILE/DECLARATION COMPOSITION
* ============================================================================
* 
* The Compile dispatcher may consume either:
* 
* targetDeclaration
* targetProfileDeclaration
* targetGroup
* targetDerivation
* 
* according to the canonical source-element policy.
* 
* This grammar itself does not decide where a construct is permitted in a
* complete source unit.
  */

/*

* ============================================================================
* 23. AST CONTRACT
* ============================================================================
* 
* Every accepted construct must map to a domain-neutral AST representation.
* 
* Recommended structural model:
* 
* TargetDeclaration
*   - source_span
*   - target_expression
*   - entries
* 
* TargetExpression:
*   - Name
*   - Set
*   - Union
*   - Intersection
*   - Difference
* 
* TargetEntry:
*   - Requirement
*   - Constraint
*   - Preference
*   - Hint
*   - Capability
*   - ProfileReference
*   - Portability
*   - Scalability
*   - ResourceIntent
*   - Property
*   - Alternative
* 
* TargetProfile:
*   - name
*   - entries
* 
* TargetProperty:
*   - name
*   - value
* 
* IMPORTANT:
* 
* The AST MUST preserve:
* 
* - source spans;
* - source ordering;
* - explicit category;
* - expression structure;
* - symbolic names;
* - provenance.
* 
* It must NOT prematurely create:
* 
* DeviceId
* PhysicalQubitId
* BackendId
* HardwareSchedule
* Route
* CalibrationRecord
* 
* Those are downstream representations.
  */

/*

* ============================================================================
* 24. SEMANTIC CONTRACT
* ============================================================================
* 
* Semantic analysis consumes this AST and determines:
* 
* 1. Whether the target expression is meaningful.
* 
* 2. Whether referenced profiles exist.
* 
* 3. Whether properties are known or permitted by the active specification
*    or dialect.
* 
* 4. Whether requirements are satisfiable.
* 
* 5. Whether constraints contradict one another.
* 
* 6. Whether preferences conflict with mandatory requirements.
* 
* 7. Whether fallback alternatives preserve required semantics.
* 
* 8. Whether portability guarantees are valid.
* 
* 9. Whether resource requirements can be represented without overflow,
*    truncation, or loss of precision.
* 
* 10. Whether the target contract is compatible with the program's domains.
* 
* Parser acceptance MUST NOT imply target feasibility.
* 
* A valid source program may be impossible to execute on a particular current
* machine because that machine lacks required resources or capabilities.
* 
* That is a semantic/resource/target-resolution result, not a grammar error.
  */

/*

* ============================================================================
* 25. REQUIREMENT / CONSTRAINT / PREFERENCE / HINT SEPARATION
* ============================================================================
* 
* These four categories have deliberately different semantics.
* 
* REQUIREMENT
* Mandatory for correctness or declared execution semantics.
* 
* CONSTRAINT
* Restricts acceptable realization.
* 
* PREFERENCE
* Desirable but not mandatory.
* 
* HINT
* Non-authoritative implementation guidance.
* 
* A compiler MUST NOT silently promote:
* 
* preference -> requirement
* 
* or:
* 
* hint -> requirement.
* 
* Likewise, an unavailable preference MUST NOT make an otherwise valid
* portable program semantically invalid unless the language specification
* explicitly declares that preference class mandatory.
  */

/*

* ============================================================================
* 26. RESOURCE INTEGRATION
* ============================================================================
* 
* Resource quantities remain semantic values.
* 
* Examples:
* 
* requires qubits >= n
* requires memory >= required_memory
* requires capability("tensor.compute")
* requires capability("gpu.compute")
* requires capability("quantum.measurement")
* 
* This grammar does not decide:
* 
* how many physical qubits exist;
* how much physical memory exists;
* how many GPUs exist;
* how many nodes exist;
* which device satisfies the request.
* 
* Those questions belong to:
* 
* grammar/resources/
* semantic resource analysis
* target resolution
* hardware/HAL
* scheduling
* runtime/deployment

*/

/*

* ============================================================================
* 27. HARDWARE INTEGRATION
* ============================================================================
* 
* Hardware descriptions remain owned by grammar/hardware/.
* 
* Target intent may request properties such as:
* 
* capability("gpu.compute")
* capability("fpga.synthesis")
* capability("quantum.measurement")
* capability("hardware.timing")
* 
* but target.g4 MUST NOT define:
* 
* registers;
* wires;
* ports;
* physical memory banks;
* physical cores;
* physical qubits;
* FPGA tiles;
* ASIC cells;
* clock implementation;
* calibration records.
* 
* Hardware realization remains downstream.
  */

/*

* ============================================================================
* 28. QUANTUM INTEGRATION
* ============================================================================
* 
* Target intent is domain-neutral.
* 
* Quantum source syntax remains owned by grammar/quantum/.
* 
* Quantum semantic lowering remains:
* 
* source
*   |
*   v
* domain-neutral AST
*   |
*   v
* semantic quantum model
*   |
*   v
* quantum::ir
*   |
*   v
* optimization
*   |
*   v
* routing
*   |
*   v
* scheduling
*   |
*   v
* QEC / resilience / ZQN
*   |
*   v
* HAL
* 
* This file MUST NOT create:
* 
* QuantumTargetIR
* PhysicalQuantumTargetIR
* QuantumCompileIR
* 
* "quantum::ir" remains canonical.
  */

/*

* ============================================================================
* 29. CLASSICAL / GPU / FPGA / ASIC / DISTRIBUTED INTEGRATION
* ============================================================================
* 
* The same target contract can apply to:
* 
* classical CPU computation;
* multicore computation;
* GPU acceleration;
* FPGA implementation;
* ASIC implementation;
* distributed computation;
* cloud execution;
* edge execution;
* heterogeneous execution;
* future architectures.
* 
* The grammar does not need a new root syntax merely because a new target
* family is introduced.
* 
* New capabilities should normally appear as semantic capability names or
* qualified properties.
  */

/*

* ============================================================================
* 30. FALLBACK / POCO-REAF INTEGRATION
* ============================================================================
* 
* Fallback must preserve semantics.
* 
* For example, a program may conceptually describe:
* 
* primary realization: quantum
* fallback realization: classical simulation
* 
* The grammar may preserve that intent through a property or target set.
* 
* Semantic analysis must determine whether the fallback actually preserves:
* 
* - functional semantics;
* - precision;
* - observable behavior;
* - effect semantics;
* - timing guarantees where required;
* - security requirements;
* - resource guarantees;
* - declared correctness properties.
* 
* A fallback is NOT automatically semantically equivalent merely because both
* targets can parse the same program.
  */

/*

* ============================================================================
* 31. TARGET RESOLUTION
* ============================================================================
* 
* Target resolution is downstream from parsing.
* 
* The intended pipeline is:
* 
* target AST
*     |
*     v
* semantic target contract
*     |
*     v
* resource/capability requirements
*     |
*     v
* available realization candidates
*     |
*     v
* constraint evaluation
*     |
*     v
* preference ordering
*     |
*     v
* selected realization
* 
* The parser MUST NOT perform this process.
* 
* In particular, parsing must succeed even when no currently available target
* satisfies the declaration.
  */

/*

* ============================================================================
* 32. DETERMINISM
* ============================================================================
* 
* Parsing is deterministic.
* 
* It MUST NOT depend on:
* 
* - current hardware;
* - CPU count;
* - GPU availability;
* - QPU availability;
* - filesystem state;
* - network state;
* - environment variables;
* - wall-clock time;
* - randomness;
* - backend availability.
* 
* Identical source and identical grammar version must produce identical parse
* structure.
  */

/*

* ============================================================================
* 33. SCALABILITY
* ============================================================================
* 
* The grammar uses repetition and recursive expressions rather than fixed
* hardware cardinalities.
* 
* There is no parser-level limit on:
* 
* target entries;
* target alternatives;
* target sets;
* requirements;
* constraints;
* preferences;
* capabilities;
* resource properties;
* target profiles;
* target-expression depth.
* 
* Practical parser/compiler resource limits may exist as implementation
* safeguards. They are NOT language semantics.
* 
* A compiler may reject an input because the compiler process exhausts its
* available resources. That must remain an implementation/resource diagnostic,
* not a fabricated Zamani hardware-language maximum.
  */

/*

* ============================================================================
* 34. NUMERIC VALUE POLICY
* ============================================================================
* 
* Numeric literals inside expressions are program data.
* 
* Therefore:
* 
* target quantum {
*     requires qubits >= 1024;
* }
* 
* may be valid source semantics.
* 
* The number 1024 MUST NOT become:
* 
* MAX_QUBITS = 1024
* 
* Likewise:
* 
* memory >= required_memory
* 
* must remain a semantic resource expression rather than being converted into
* a compiler-wide memory ceiling.
* 
* Numeric representation, arbitrary precision, overflow behavior, and
* conversion rules belong to the canonical literal/type/semantic contracts.
  */

/*

* ============================================================================
* 35. SECURITY
* ============================================================================
* 
* Target syntax is not an authorization mechanism.
* 
* A target declaration MUST NOT grant source code permission to:
* 
* access devices;
* access files;
* access networks;
* access credentials;
* access host processes;
* access compiler internals;
* bypass security policy.
* 
* Security policy remains authoritative downstream.
* 
* A target preference cannot override a security prohibition.
  */

/*

* ============================================================================
* 36. PROVENANCE
* ============================================================================
* 
* Downstream AST and semantic representations must preserve enough provenance
* to answer:
* 
* Which source target declaration introduced this requirement?
* 
* Which source property introduced this constraint?
* 
* Which profile contributed this preference?
* 
* Which compilation decision consumed it?
* 
* Which target realization satisfied it?
* 
* Provenance is particularly important for:
* 
* diagnostics;
* reproducibility;
* optimization;
* quantum compilation;
* hardware adaptation;
* resource negotiation;
* fallback;
* deployment.

*/

/*

* ============================================================================
* 37. ERROR CATEGORIES
* ============================================================================
* 
* Syntax errors:
* 
* malformed target expression;
* malformed target block;
* malformed property;
* malformed list;
* malformed profile.
* 
* Semantic errors:
* 
* unknown profile;
* contradictory constraints;
* invalid property;
* unsatisfied capability;
* impossible requirement;
* invalid fallback;
* incompatible portability contract.
* 
* Resource errors:
* 
* insufficient available resources;
* unavailable capability;
* unavailable topology;
* unavailable execution environment.
* 
* These categories MUST remain distinct.
  */

/*

* ============================================================================
* 38. COMPATIBILITY
* ============================================================================
* 
* Existing stable lexical names are preserved.
* 
* This grammar intentionally uses existing lexical vocabulary instead of
* introducing target-specific keyword tokens.
* 
* Therefore adding a new target family such as:
* 
* future.accelerator
* 
* does not require changing the target grammar.
* 
* A new semantic property can likewise be represented through:
* 
* property name/value
* 
* subject to specification and dialect rules.
* 
* Token renames remain compatibility-sensitive and belong to:
* 
* grammar/compatibility/
* 
* and:
* 
* grammar/spec/compatibility.md

*/

/*

* ============================================================================
* 39. DIALECT INTEGRATION
* ============================================================================
* 
* Dialects may extend target semantics through explicit namespaced properties.
* 
* A dialect MUST declare:
* 
* - name;
* - version;
* - semantic meaning;
* - AST mapping;
* - compatibility;
* - portability classification.
* 
* A dialect MUST NOT silently redefine the universal meaning of:
* 
* requirement;
* constraint;
* preference;
* hint;
* 
* nor may it introduce a hidden physical-device dependency into portable
* source semantics.
  */

/*

* ============================================================================
* 40. NO DUPLICATE IR
* ============================================================================
* 
* This grammar produces syntax only.
* 
* It does not define:
* 
* TargetIR
* HardwareIR
* QuantumTargetIR
* DeviceSelectionIR
* ScheduleIR
* RoutingIR
* 
* Existing compiler/IR subsystems remain authoritative.
* 
* Required direction:
* 
* syntax
*   |
*   v
* AST
*   |
*   v
* semantic model
*   |
*   v
* canonical IR
*   |
*   v
* target realization

*/

/*

* ============================================================================
* 41. RUNTIME INTEGRATION
* ============================================================================
* 
* Runtime consumes compiled representations and execution plans.
* 
* Runtime MUST NOT need to parse this grammar.
* 
* Therefore:
* 
* source
*   -> parser
*   -> AST
*   -> semantic analysis
*   -> compiler
*   -> executable/plan
*   -> runtime
* 
* rather than:
* 
* runtime
*   -> target.g4

*/

/*

* ============================================================================
* 42. TEST CONTRACT
* ============================================================================
* 
* Positive tests MUST cover at least:
* 
* target cpu;
* target gpu;
* target quantum;
* target quantum | classical;
* target gpu & accelerator;
* target gpu - unavailable;
* target [cpu, gpu, accelerator];
* 
* target quantum {
*     requires capability("quantum.measurement");
* }
* 
* target accelerator {
*     prefer capability("tensor.compute");
* }
* 
* target distributed {
*     constraint latency <= bound;
* }
* 
* target quantum {
*     property fallback = [quantum, classical];
* }
* 
* profile portable_quantum {
*     requires capability("quantum.measurement");
* }
* 
* target quantum {
*     profile portable_quantum;
* }
* 
* The exact test corpus should use canonical expression syntax accepted by
* the current parser.
* 
* Negative tests MUST cover:
* 
* target;
* target {};
* target [;
* target quantum { requires ; }
* target quantum { property ; }
* target quantum { property = ; }
* malformed target-expression operators;
* malformed property blocks;
* malformed profile blocks.
* 
* Boundary tests MUST cover:
* 
* deeply nested target expressions;
* large target sets;
* large requirement collections;
* large capability collections;
* large profile bodies;
* long qualified names;
* large symbolic resource expressions.
* 
* Scalability tests MUST verify that no artificial hardware ceiling appears.
* 
* Determinism tests MUST verify identical parse trees for identical source.
* 
* Cross-domain tests MUST include:
* 
* classical + target;
* quantum + target;
* hybrid + target;
* HDL + target;
* hardware + target;
* AI + target;
* distributed + target;
* networking + target;
* security + target;
* quantum + classical + HDL + target.

*/

/*

* ============================================================================
* 43. HARD-CODING AUDIT
* ============================================================================
* 
* This grammar MUST NOT contain implementation constants representing:
* 
* maximum qubits;
* maximum CPUs;
* maximum GPUs;
* maximum FPGAs;
* maximum nodes;
* maximum memory;
* maximum devices;
* maximum topology size;
* maximum tensor rank;
* maximum register width;
* maximum network size.
* 
* Numeric examples in comments are illustrative only.
* 
* No physical device identifier is part of the grammar.
* 
* No vendor ID is part of the grammar.
* 
* No backend ID is part of the grammar.
* 
* No physical qubit ID is part of the grammar.
  */

/*

* ============================================================================
* 44. INDEPENDENT COMPLETION CONTRACT
* ============================================================================
* 
* This file can be considered independently complete when:
* 
* Syntax
* [ ] ANTLR generation succeeds.
* [ ] All referenced tokens exist in ZamaniLexer.
* [ ] No lexer token is defined here.
* [ ] No duplicate expression grammar exists here.
* [ ] Target-expression precedence is deterministic.
* [ ] Target collections are unbounded at the language level.
* 
* Ownership
* [ ] Target intent has one owner.
* [ ] Resources remain owned by resources/.
* [ ] Hardware remains owned by hardware/.
* [ ] Quantum semantics remain owned by quantum/.
* [ ] Execution remains owned by execution/.
* 
* AST
* [ ] Every public rule has a documented AST mapping.
* [ ] Source spans are preserved.
* [ ] Requirements/constraints/preferences/hints remain distinguishable.
* [ ] Target properties remain structured.
* 
* Semantics
* [ ] Requirements are mandatory.
* [ ] Constraints restrict realization.
* [ ] Preferences remain optional.
* [ ] Hints remain advisory.
* [ ] Capability resolution is downstream.
* [ ] Resource resolution is downstream.
* [ ] Target resolution is downstream.
* 
* IR
* [ ] No duplicate IR is created.
* [ ] quantum::ir remains canonical.
* [ ] No physical target IR is introduced here.
* 
* POCO-REAF
* [ ] No universal machine limits exist.
* [ ] No fixed target enumeration exists.
* [ ] New hardware families can be represented without grammar changes.
* [ ] Resource availability determines realizable scale.
* 
* Safety
* [ ] No embedded actions.
* [ ] No filesystem access.
* [ ] No network access.
* [ ] No hardware discovery.
* [ ] No unsafe Rust requirement.
* [ ] Rust 1.97/1.97.1 integration remains possible.
* 
* Testing
* [ ] Positive tests exist.
* [ ] Negative tests exist.
* [ ] Boundary tests exist.
* [ ] Scalability tests exist.
* [ ] Determinism tests exist.
* [ ] Cross-domain tests exist.
* [ ] Compatibility tests exist.
* [ ] Hard-coding audit passes.
* 
* ============================================================================
* FINAL INVARIANT
* ============================================================================
* 
* This file expresses:
* 
* WHERE / UNDER WHAT SEMANTIC CONDITIONS
* a program may be realized.
* 
* It does NOT prescribe:
* 
* HOW / ON WHICH PHYSICAL DEVICE
* the program must be realized.
* 
* Therefore:
* 
* target intent
*      |
*      v
* resource + capability analysis
*      |
*      v
* target resolution
*      |
*      v
* canonical semantic IR
*      |
*      v
* optimization / routing / scheduling / resilience / QEC / ZQN
*      |
*      v
* HAL / backend
*      |
*      v
* CPU / GPU / FPGA / ASIC / QPU / distributed / future hardware
* 
* This separation is the target-selection foundation required for:
* 
* Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
* 
* while allowing Zamani to scale from the smallest realizable system to
* arbitrarily large heterogeneous systems subject only to actual resources
* and semantic feasibility.
* 
* ============================================================================
  */
Zamani Migration Specification

Path: "grammar/compatibility/migrations.md"
Status: Normative
Scope: Language, grammar, frontend, semantic, IR, artifact, interoperability, runtime, tooling, and repository migrations
Rust baseline: Rust 1.97 or later
Rust edition: 2021
Rust safety: Production Zamani Rust code MUST NOT use Rust "unsafe"
Primary portability objective: Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)

---

1. Purpose

This document is the normative procedure for migrating Zamani programs, language representations, compiler representations, serialized artifacts, dialects, and compatibility contracts between supported versions.

It defines how an existing representation moves to a newer representation while preserving specified semantics whenever the compatibility contract requires preservation.

This document governs migration of:

- source programs;
- lexical representations;
- keywords;
- identifiers;
- operators;
- literals;
- grammar rules;
- parser representations;
- declarations;
- statements;
- expressions;
- patterns;
- types;
- generics;
- ownership and borrowing;
- modules;
- functions;
- effects;
- resources;
- capabilities;
- requirements;
- constraints;
- preferences;
- hints;
- policies;
- contracts;
- provenance;
- evidence;
- uncertainty;
- reasoning;
- learning;
- adaptation;
- concurrency;
- distributed computation;
- classical computation;
- quantum computation;
- hybrid computation;
- HDL;
- hardware intent;
- AI/data semantics;
- networking;
- security;
- interoperability;
- dialects;
- metaprogramming;
- AST schemas;
- semantic schemas;
- canonical IR;
- "quantum::ir";
- compiled artifacts;
- ABI contracts;
- runtime contracts;
- diagnostics;
- tooling;
- generated documentation;
- compatibility metadata.

The governing principle is:

«Migrate representations; preserve specified meaning.»

A migration MUST NOT use current compiler limitations, current hardware availability, temporary backend limitations, or implementation convenience as justification for permanently reducing Zamani's language-level scalability or semantic model.

---

2. Normative Language

The following terms are normative:

- MUST — mandatory.
- MUST NOT — prohibited.
- REQUIRED — mandatory.
- SHOULD — recommended unless a documented technical reason prevents it.
- SHOULD NOT — discouraged unless justified.
- MAY — permitted.
- OPTIONAL — permitted but not required.

A migration is not complete merely because old source can be parsed.

A migration is complete only when all applicable semantic, compatibility, validation, provenance, scalability, determinism, and integration requirements have been satisfied.

---

3. Ownership

This file owns:

- migration classification;
- migration procedure;
- migration records;
- transformation requirements;
- semantic-preservation requirements;
- migration validation;
- migration provenance;
- migration completion criteria;
- migration tooling contracts;
- cross-layer migration integration.

This file does not own:

- language version numbering;
- AST schema definitions;
- grammar syntax;
- semantic definitions;
- IR schema definitions;
- deprecation policy;
- dialect version numbering;
- feature-gate policy;
- compatibility matrices;
- target capability definitions.

Those remain owned by their dedicated repository files.

---

4. Repository Authority Model

The migration system participates in the following authority hierarchy:

grammar/DESIGN.md
        │
        ▼
grammar/specification/
        │
        ├── normative language meaning
        ├── normative domain semantics
        └── compilation model
        │
        ▼
grammar/spec/
        │
        ├── focused machine-oriented contracts
        ├── compatibility contracts
        └── semantic contracts
        │
        ▼
grammar/compatibility/
        │
        ├── language-version.md
        ├── grammar-version.md
        ├── AST-version.md
        ├── semantic-version.md
        ├── ir-version.md
        ├── dialect-version.md
        ├── target-compatibility-version.md
        ├── versions.md
        ├── migrations.md
        ├── deprecated.md
        ├── reserved.md
        ├── feature-gates.md
        ├── compatibility-matrix.md
        ├── dialect-compatibility.md
        ├── frontend-conformance.md
        ├── ast-conformance.md
        └── ir-conformance.md
        │
        ▼
grammar/Zamani.g4
        │
        ▼
lexer
        │
        ▼
parser
        │
        ▼
AST
        │
        ▼
structural validation
        │
        ├── names
        ├── types
        ├── effects
        ├── resources
        ├── capabilities
        ├── contracts
        ├── policies
        └── provenance
        │
        ▼
semantic model
        │
        ├── Classical
        ├── Quantum
        ├── HDL
        ├── AI
        ├── Data
        ├── Distributed
        ├── Networking
        └── Hybrid
        │
        ▼
canonical IR
        │
        ├── Classical IR
        └── quantum::ir
        │
        ▼
optimization
        │
        ├── lowering
        ├── routing
        ├── scheduling
        ├── resilience
        └── QEC
        │
        ▼
ZQN
        │
        ▼
HAL
        │
        ▼
target/runtime realization

A migration MUST respect this hierarchy.

No migration document, compatibility mechanism, generated artifact, or legacy representation may silently become a competing language authority.

---

5. Companion File Ownership

The following ownership is normative.

File| Owner responsibility
"grammar/DESIGN.md"| Architecture and authority
"grammar/specification/"| Human-readable normative language specification
"grammar/spec/"| Focused formal/machine-oriented contracts
"grammar/compatibility/language-version.md"| Language version semantics
"grammar/compatibility/grammar-version.md"| Grammar version semantics
"grammar/compatibility/AST-version.md"| AST schema version semantics
"grammar/compatibility/semantic-version.md"| Semantic-model version semantics
"grammar/compatibility/ir-version.md"| IR version semantics
"grammar/compatibility/dialect-version.md"| Dialect version semantics
"grammar/compatibility/target-compatibility-version.md"| Target compatibility semantics
"grammar/compatibility/versions.md"| Compatibility/version policy
"grammar/compatibility/migrations.md"| Migration procedures
"grammar/compatibility/deprecated.md"| Deprecation lifecycle
"grammar/compatibility/reserved.md"| Reserved syntax/identifier policy
"grammar/compatibility/feature-gates.md"| Feature availability gates
"grammar/compatibility/compatibility-matrix.md"| Compatibility relationships
"grammar/compatibility/dialect-compatibility.md"| Dialect compatibility
"grammar/compatibility/frontend-conformance.md"| Frontend conformance
"grammar/compatibility/ast-conformance.md"| AST conformance
"grammar/compatibility/ir-conformance.md"| IR conformance
"grammar/grammar.md"| Implementation/conformance status
"grammar/Zamani-Grammar.md"| Historical/extended reference
"grammar/Zamani.g4"| Canonical ANTLR composition
"grammar/lexer/"| Lexical contracts
"src/lexer.rs"| Executable lexer
"src/parser.rs"| Executable parser
"src/ast/"| Frontend AST
semantic implementation| Semantic meaning
canonical IR implementation| Canonical compiler representation
"quantum::ir"| Canonical quantum IR boundary
compiler/backend| Lowering and target realization

If another repository file changes ownership, this document MUST reference the new owner rather than duplicating the contract.

---

6. Core Migration Invariant

For every migration:

OLD REPRESENTATION
        │
        ▼
MIGRATION
        │
        ▼
CURRENT REPRESENTATION
        │
        ▼
CURRENT SEMANTICS

The semantic result MUST be equivalent whenever the compatibility contract requires equivalence.

Formally:

Meaning(old_program)
    =
Meaning(migrate(old_program))

for migrations classified as semantics-preserving.

If the meaning cannot be preserved, the migration MUST explicitly classify the semantic change.

Silent semantic changes are prohibited.

---

7. Compatibility Is Layered

Compatibility MUST be evaluated independently across:

Source
  ↓
Lexical
  ↓
Grammar
  ↓
Parser
  ↓
AST
  ↓
Name Resolution
  ↓
Type
  ↓
Effect
  ↓
Resource
  ↓
Capability
  ↓
Contract
  ↓
Policy
  ↓
Provenance
  ↓
Semantic
  ↓
Canonical IR
  ↓
Artifact
  ↓
ABI
  ↓
Runtime
  ↓
Target
  ↓
Execution

A change MAY be compatible at one layer and incompatible at another.

Example:

Source:        compatible
Lexer:         compatible
Parser:        compatible
AST:           compatible
Semantics:     compatible
quantum::ir:   compatible
Artifact:      incompatible
Runtime:       target-dependent

Therefore:

«Compatibility MUST NOT be reduced to "does it parse?"»

Likewise:

«A backend representation change MUST NOT automatically become a source-language breaking change.»

---

8. Migration Classes

Every non-trivial migration MUST be assigned one or more migration classes.

Class| Name| Meaning
M0| No Migration| Representation remains compatible
M1| Transparent Compatibility| Old and new representations coexist without source transformation
M2| Automatic Mechanical| Deterministic semantics-preserving transformation
M3| Tool-Assisted| Tool performs deterministic portions and requests decisions for ambiguous portions
M4| Explicit Source Migration| Developer must modify source
M5| Semantic Migration| Meaning or semantic model changes
M6| Artifact Migration| Serialized/compiled artifact changes
M7| Dialect Migration| Dialect or extension representation changes
M8| Target Migration| Target/runtime realization changes
M9| Breaking Migration| Compatibility contract cannot preserve the old representation or semantics

A migration MAY have multiple classifications.

Example:

Source       = M2
Lexer        = M2
Parser       = M2
AST          = M0
Semantics    = M0
quantum::ir  = M0
Artifact     = M6
Target       = M8

---

9. Migration Classification Rules

A migration MUST answer these questions in order.

9.1 Does the old source still parse?

If yes, determine whether its meaning is unchanged.

9.2 Is its specified meaning unchanged?

If yes:

M0 or M1

If no:

M5

9.3 If it no longer parses, is there a deterministic transformation?

If yes:

M2

9.4 Does the transformation contain ambiguity?

If yes:

M3

9.5 Does the developer have to choose between semantic interpretations?

Then:

M4 or M5

9.6 Is compatibility impossible under the promised contract?

Then:

M9

Parser failure alone MUST NOT determine whether a migration is breaking.

---

10. Migration Identity

Every migration MUST have a stable migration identifier.

The identifier MUST identify the semantic feature rather than merely its textual spelling.

Preferred:

quantum.operation.generic.v2

Not:

apply_gate_to_q

The migration identity MUST survive lexical renaming.

A rename from:

old_keyword

to:

new_keyword

does not create a new semantic feature if the underlying semantics remain the same.

---

11. Required Migration Record

Every non-trivial migration MUST have a record containing:

Migration ID:
Feature ID:
Migration Name:

Source Language Version:
Target Language Version:

Source Representation:
Target Representation:

Migration Class:

Affected Layers:
Affected Domains:

Old Syntax:
New Syntax:

Old Tokens:
New Tokens:

Old AST:
New AST:

Old Semantic Model:
New Semantic Model:

Old IR:
New IR:

Quantum IR Impact:
HDL/HW Impact:

Type Impact:
Effect Impact:
Capability Impact:
Resource Impact:
Contract Impact:
Policy Impact:
Provenance Impact:

Semantic Preservation:
Semantic Changes:

Automatic Transformation:
Tool-Assisted Transformation:
Required Manual Action:

Diagnostics:
Source-Span Policy:
Comment Policy:
Formatting Policy:

Compatibility Window:
Deprecation Status:
Removal Version:

Artifact Impact:
ABI Impact:
Runtime Impact:
Target Impact:

Rollback:
Recovery:

Positive Tests:
Negative Tests:
Boundary Tests:
Scalability Tests:
Determinism Tests:
Compatibility Tests:
Diagnostic Tests:

Specification Files:
Grammar Files:
Lexer Files:
Parser Files:
AST Files:
Semantic Files:
IR Files:
Compiler Files:
Runtime Files:
Tooling Files:
Test Files:

Feature Manifest:
Compatibility Matrix:
Deprecation Record:

Hard-Coding Audit:
Unsafe-Rust Audit:

Completion Status:

A migration without the required applicable fields MUST NOT be marked production-complete.

---

12. Migration Manifest

Each migration SHOULD have a machine-readable manifest.

The manifest SHOULD identify:

migration_id
feature_id
source_version
target_version
migration_class
source_layers
target_layers
source_domains
target_domains
semantic_preserving
requires_manual_action
reversible
deprecated_source
removal_version
ast_change
semantic_change
ir_change
artifact_change
abi_change
runtime_change
target_change
tests
provenance

The manifest MUST be versioned independently from source syntax.

It MUST NOT contain fixed resource capacities.

---

13. Source Migration

A source migration transforms source representation while preserving specified meaning.

It MUST preserve, where applicable:

- bindings;
- scope;
- name resolution;
- evaluation order;
- control flow;
- types;
- generics;
- ownership;
- borrowing;
- effects;
- capabilities;
- requirements;
- resources;
- contracts;
- policies;
- provenance;
- concurrency;
- distributed behavior;
- numerical meaning;
- quantum semantics;
- HDL semantics;
- hardware intent.

Source migration MAY change:

- keyword spelling;
- syntactic sugar;
- module paths;
- declaration ordering where order is semantically irrelevant;
- equivalent expression syntax;
- deprecated syntax.

Source migration MUST NOT silently change:

- operation order;
- measurement meaning;
- synchronization;
- resource requirements;
- security policy;
- effects;
- ownership;
- mutation;
- adaptation authority.

---

14. Source-Span Preservation

Migration tooling MUST preserve source locations whenever practical.

The migration result SHOULD provide:

old span → new span

for transformed constructs.

When exact mapping is impossible, the migration tool MUST:

1. report the affected span;
2. identify the transformation;
3. provide the replacement span;
4. preserve enough provenance for diagnostics to remain actionable.

The repository's source-span contract remains owned by the validation/source-span specification.

---

15. Comment and Formatting Preservation

Migration tooling MUST distinguish:

- semantic preservation;
- formatting preservation;
- comment preservation.

A migration MAY preserve semantics without preserving exact formatting.

If comments are dropped, the tool MUST report that when comment preservation is part of the supported migration contract.

Comments MUST NOT be interpreted as semantic source unless the language specification explicitly makes them semantic.

---

16. Automatic Migration

Automatic migration is permitted only when the transformation is deterministic.

Examples include:

legacy_keyword → canonical_keyword
legacy_module_path → canonical_module_path
deprecated_syntax → equivalent_current_syntax

provided semantic equivalence has been established.

Automatic migration MUST NOT infer developer intent where multiple meanings are possible.

The tool MUST fail safely rather than selecting an arbitrary interpretation.

---

17. Tool-Assisted Migration

Tool-assisted migration MUST:

1. discover affected constructs;
2. report source locations;
3. identify applicable migration rules;
4. display proposed transformations;
5. distinguish deterministic transformations from decisions;
6. refuse unsafe semantic guessing;
7. produce machine-readable diagnostics;
8. validate the transformed source;
9. reparse the result;
10. re-run structural validation;
11. preserve migration provenance;
12. report unresolved items.

The tool MUST distinguish:

AUTOMATICALLY_TRANSFORMED

from:

REQUIRES_DEVELOPER_DECISION

---

18. Manual Migration

Manual migration is required when semantics cannot safely be inferred.

Examples:

- changed ownership;
- changed concurrency;
- changed effect behavior;
- changed resource semantics;
- changed capability requirements;
- changed security guarantees;
- changed quantum measurement semantics;
- changed hardware intent;
- incompatible dialect semantics;
- removed information;
- multiple valid replacements.

Manual migrations MUST document:

before
after
semantic difference
reason
developer decision
validation requirements

---

19. Grammar Reorganization

Moving grammar rules between files is not automatically a language migration.

For example:

historical monolithic grammar
        ↓
modular grammar/
        ↓
same canonical AST
        ↓
same semantics
        ↓
same canonical IR

is implementation restructuring when the accepted language and semantics remain equivalent.

A grammar reorganization MUST preserve:

- lexical meaning;
- token identity;
- precedence;
- associativity;
- ambiguity behavior;
- parse structure;
- AST semantics;
- semantic meaning.

Grammar modularization MUST NOT create multiple competing language definitions.

---

20. "grammar/Zamani.g4"

"grammar/Zamani.g4" remains the canonical ANTLR composition root.

Migration work MUST converge toward a composition architecture where the root grammar performs composition and dispatch rather than becoming a second semantic specification.

The root grammar MAY compose:

- core;
- declarations;
- statements;
- expressions;
- types;
- functions;
- modules;
- effects;
- memory;
- concurrency;
- classical;
- quantum;
- hybrid;
- HDL;
- hardware;
- resources;
- distributed;
- AI;
- data;
- networking;
- security;
- interoperability;
- dialects;
- macros;
- metaprogramming.

The root grammar MUST NOT contain implementation-specific hardware limits.

---

21. Lexer Migration

The lexical authority is distributed between the lexical specification and executable implementation according to repository ownership.

Relevant files include:

grammar/lexer/
grammar/antlr/
grammar/Zamani.g4
src/lexer.rs

Migration MUST ensure that these representations remain synchronized.

Before introducing a new keyword, determine whether the concept can instead be represented as:

- identifier;
- qualified name;
- operation name;
- attribute;
- type;
- capability;
- effect;
- dialect construct;
- metadata.

This is especially important for generic computational concepts such as:

- reasoning;
- inference;
- learning;
- adaptation;
- querying;
- evidence;
- provenance;
- policy.

Keyword growth MUST be deliberate.

---

22. Token Identity

One source spelling SHOULD correspond to one canonical lexical identity unless separate token identities have a demonstrable lexical purpose.

Where overlapping tokens exist, migration work MUST evaluate them before creating new aliases.

Examples include overlapping operator concepts such as:

BitAnd / Ampersand
Question / QuestionMark

If one spelling can be represented by one token and parser context determines its semantic interpretation, token consolidation SHOULD be preferred.

The migration sequence is:

source spelling
      ↓
canonical token
      ↓
parser context
      ↓
AST
      ↓
semantic meaning

not:

source spelling
      ↓
multiple competing lexical authorities

---

23. Keyword Migration

A keyword migration MUST account for:

- token registry;
- lexer;
- parser;
- grammar;
- AST;
- diagnostics;
- reserved identifiers;
- dialect interaction;
- tooling;
- tests;
- compatibility metadata.

A word appearing in historical or extended documentation MUST NOT automatically become a reserved keyword.

---

24. Literal Migration

Literal migrations MUST preserve value semantics.

Numeric literal migrations MUST NOT turn an implementation representation into a universal language limit.

The language may have representation-specific constraints in a particular compiler implementation, but those constraints MUST remain implementation constraints unless explicitly standardized.

Quantum literals and state expressions MUST preserve their specified quantum semantics.

---

25. Type Migration

Type migration MUST preserve:

- identity;
- generic arguments;
- bounds;
- constraints;
- associated types;
- ownership;
- variance where specified;
- effects;
- capabilities;
- resource requirements;
- quantum meaning;
- hardware intent.

Symbolic types MUST remain symbolic.

For example:

Qubit[n]
Tensor<T, shape>
Memory<T, required_memory>

MUST NOT be transformed into compiler-wide fixed maxima.

---

26. Requirement / Capability / Constraint / Preference Separation

Migration MUST preserve the distinction between:

Requirement
Capability
Constraint
Preference
Hint
Realization

For example:

requires capability("quantum.measurement");

means that the execution environment must provide the capability.

It does not mean:

use device 0

Likewise:

requires qubits >= n;

does not define a universal physical qubit numbering scheme.

A migration MUST preserve this abstraction.

---

27. Resource Migration

Resource requirements MUST remain open-ended.

Valid concepts include:

requires memory >= required_memory;
requires qubits >= n;
requires capability("gpu.compute");
requires capability("tensor.compute");
requires topology(required_topology);
prefer ...
constrain ...
allow ...
forbid ...

Migration MUST NOT convert these into fixed physical assumptions.

Resource realization belongs downstream.

---

28. Scalability Invariant

The migration system MUST preserve open-ended scalability.

It MUST NOT introduce universal language limits such as:

MAX_QUBITS
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_ASICS
MAX_QPUS
MAX_NODES
MAX_MEMORY
MAX_STORAGE
MAX_REGISTER_WIDTH
MAX_VECTOR_WIDTH
MAX_TENSOR_RANK
MAX_TENSOR_DIMENSION
MAX_DEVICE_COUNT
MAX_NETWORK_SIZE
MAX_AGENT_COUNT
MAX_TIMELINE_COUNT
MAX_MODULE_COUNT
MAX_PROGRAM_SIZE

This prohibition applies equally to:

- migration tools;
- compatibility manifests;
- grammar rules;
- tests;
- examples;
- documentation;
- AST schemas;
- semantic models;
- IR schemas.

An implementation MAY encounter actual resource exhaustion.

That is different from imposing a language-wide artificial ceiling.

---

29. POCO-REAF Migration Invariant

The fundamental distinction is:

PROGRAM INTENT
       ≠
TARGET REALIZATION

Migration MUST preserve program intent.

Target realization MAY change according to:

- available resources;
- capabilities;
- topology;
- memory;
- accelerators;
- quantum hardware;
- classical hardware;
- distributed infrastructure;
- timing;
- resilience;
- scheduling;
- routing;
- QEC;
- runtime capabilities;
- deployment policy.

A migration MUST NOT rewrite source semantics merely because the current target cannot satisfy a requirement.

The correct result is a capability/resource failure or a documented alternate realization.

---

30. Target Independence

Migration MUST NOT encode target identity into portable source unless target-specific syntax is explicitly part of a dialect.

The following distinctions are mandatory:

portable source
portable semantic requirements
target capability
target realization

A migration MUST NOT turn:

requires capability("gpu.compute")

into:

gpu0

or:

requires qubits >= n

into:

physical_qubit_0..physical_qubit_n

unless such mapping occurs in a target-specific realization layer.

---

31. Effects Migration

Effects are semantic properties, not merely keywords.

Migration MUST preserve effect identity.

Relevant effects may include:

io
network
mutation
randomness
native
foreign
distributed
measurement
quantum
learning
adaptation
reflection
code_generation
simulation

A migration MUST NOT silently transform:

pure → effectful

or:

effectful → pure

without an explicit semantic rule.

---

32. Contracts Migration

The following semantic constructs MUST retain their meaning:

requires
ensures
invariant
assume
guarantee
property
assert

Migration MUST preserve:

- scope;
- condition;
- evaluation point;
- failure behavior;
- proof/validation status;
- provenance;
- relationship to effects and policies.

A contract migration MUST NOT turn a requirement into a preference or a guarantee into an assumption.

---

33. Policy Migration

Policies MUST remain distinct from:

- requirements;
- capabilities;
- resources;
- effects;
- implementation choices.

Migration MUST preserve:

permission
prohibition
constraint
preference
fallback
authorization
scope
priority

A policy that prohibits an operation MUST NOT become merely advisory through migration.

---

34. Provenance Migration

Provenance MUST survive migrations whenever the provenance contract requires it.

The minimum conceptual relationship is:

original source
      ↓
migration
      ↓
derived source
      ↓
AST
      ↓
semantic representation
      ↓
IR

The provenance record SHOULD identify:

- migration ID;
- source version;
- target version;
- transformation;
- tool version;
- input digest;
- output digest;
- timestamp where required;
- operator or automation identity where required;
- diagnostics;
- unresolved decisions.

Provenance itself MUST NOT be used to change program semantics.

---

35. Evidence and Explainability Migration

Migration MUST preserve semantic evidence where the language contract requires it.

For constructs involving:

- reasoning;
- inference;
- learning;
- adaptation;
- decisions;
- policy selection;
- optimization;
- resource negotiation;

migration SHOULD preserve:

claim
evidence
source
derivation
confidence
decision
reason
provenance

Compiler transformations MAY produce additional provenance but MUST NOT fabricate evidence.

---

36. Knowledge Migration

Knowledge constructs such as:

assert
retract
query

MUST preserve the distinction between:

- source-level declarations;
- runtime knowledge state;
- compiler facts;
- external data;
- provenance.

A migration MUST NOT silently turn runtime mutable knowledge into compile-time immutable information or vice versa.

---

37. Reasoning Migration

Reasoning constructs such as:

infer
deduce
reason

MUST preserve their semantic role.

Migration MUST NOT hard-code a particular reasoning engine into the source language.

The semantic model SHOULD describe:

premises
evidence
rules
relations
conclusion
confidence
provenance

The realization may be classical, probabilistic, symbolic, learned, distributed, or hybrid.

---

38. Learning Migration

Learning constructs MUST preserve:

- input;
- target;
- model;
- data;
- objective;
- training/evaluation semantics;
- resource requirements;
- capabilities;
- effects;
- policy;
- provenance.

Algorithm-specific implementation details SHOULD remain outside universal syntax unless standardized.

A migration MUST NOT silently change training data or model semantics.

---

39. Adaptation Migration

Adaptation is a controlled semantic capability.

Migration MUST preserve:

- policy;
- authorization;
- capabilities;
- effects;
- resource constraints;
- provenance;
- validation;
- state/model/strategy identity.

Adaptation MUST NOT be migrated into unrestricted self-modifying execution.

The intended model is:

adaptation request
       ↓
policy evaluation
       ↓
authorization
       ↓
capability validation
       ↓
resource validation
       ↓
change
       ↓
validation
       ↓
provenance
       ↓
continued execution

---

40. Uncertainty Migration

Uncertainty-related constructs MUST preserve semantic distinctions among:

- probability;
- confidence;
- distribution;
- belief;
- uncertainty;
- deterministic values.

A migration MUST NOT convert a probabilistic statement into a deterministic assertion without explicit semantics.

Likewise, confidence MUST NOT be treated as probability unless the specification explicitly defines that relationship.

---

41. Pattern and Guard Migration

Pattern matching migrations MUST preserve:

- exhaustiveness;
- binding;
- guard evaluation;
- evaluation order;
- ownership;
- pattern specificity;
- fallthrough behavior.

A guard MUST remain semantically distinct from an unconditional pattern.

---

42. Actor and Multi-Agent Migration

AI-agent constructs MUST integrate with the existing concurrency architecture.

Migration MUST NOT create a second actor model.

The intended boundary is:

agent semantics
      ↓
actor semantics
      ↓
message
      ↓
channel
      ↓
task
      ↓
scheduler/runtime

Migration MUST preserve:

- actor identity;
- mailbox/message semantics;
- ordering guarantees;
- synchronization;
- cancellation;
- lifecycle;
- failure semantics;
- distributed behavior.

---

43. Simulation Migration

Simulation is an execution strategy.

Migration MUST preserve the distinction between:

program semantics

and:

simulation realization

The same semantic program MAY be realized through:

- classical simulation;
- quantum simulation;
- hardware simulation;
- distributed simulation;
- fault simulation;
- performance simulation;
- AI simulation.

A migration MUST NOT create a second language merely because the realization is simulated.

---

44. Adaptive Execution Migration

Adaptive execution MUST preserve semantic intent while allowing target realization to change.

Relevant states include:

Unknown
Healthy
Degraded
Unstable
Unavailable
Recovering
Quarantined
Retired

Relevant outcomes include:

ACCEPT
DEGRADED_ACCEPT
RETRY
RECOVER
ESCALATE
REJECT

Migration MUST preserve the distinction between:

program-level fallback

and:

runtime-level recovery

A target recovery strategy MUST NOT silently change source semantics.

---

45. Deterministic and Reproducible Migration

A semantics-preserving migration MUST be deterministic unless the migration specification explicitly permits nondeterminism.

Given identical:

source
source version
target version
migration rules
tool version
migration configuration

the migration SHOULD produce the same canonical result.

Where nondeterministic processing is unavoidable, the tool MUST expose the relevant source of nondeterminism and provide reproducibility controls.

Randomized migration behavior MUST NOT silently alter semantics.

---

46. Quantum Migration

Quantum migrations MUST preserve:

- qubit identity;
- logical resource identity;
- state semantics;
- operation ordering;
- parameter semantics;
- measurement;
- reset;
- classical feed-forward;
- entanglement;
- control flow;
- dynamic circuit behavior;
- noise semantics where specified;
- error-correction intent;
- resource requirements;
- capability requirements.

Quantum source migration MUST converge onto the canonical quantum semantic model.

---

47. "quantum::ir" Boundary

"quantum::ir" is the canonical quantum IR boundary.

A migration MUST NOT introduce a second permanent quantum IR merely to preserve historical syntax.

The preferred path is:

legacy quantum syntax
        ↓
migration
        ↓
current AST
        ↓
quantum semantic model
        ↓
quantum::ir

Historical quantum representations MAY be retained temporarily for compatibility tooling, import/export, or artifact conversion.

They MUST NOT become a second permanent compiler architecture.

---

48. Generic Quantum Operations

Quantum migration MUST support the data-driven operation model.

The semantic operation identity SHOULD be represented through concepts equivalent to:

operationSpecifier
targets
parameters
results
attributes
modifiers
effects
capabilities
resources
source

Migration MUST NOT require a universal enumeration of every known gate.

New vendor, research, parameterized, or future operations SHOULD be introduced through the appropriate operation metadata/dialect mechanisms.

---

49. Quantum Resource Migration

Quantum resources MUST remain symbolic and target-independent.

Examples:

requires qubits >= n;
requires capability("quantum.measurement");
requires capability("quantum.dynamic-circuit");
requires topology(required_topology);

Migration MUST NOT encode:

- a fixed number of qubits;
- a fixed number of QPUs;
- a fixed topology;
- a fixed physical qubit numbering scheme.

Physical mapping belongs to routing and target realization.

---

50. Quantum Error Correction Migration

Where migration affects QEC metadata, it MUST preserve:

- logical/physical distinction;
- code identity;
- distance;
- correction intent;
- syndrome/measurement semantics;
- decoder requirements;
- resilience requirements;
- resource relationships.

A migration MUST NOT replace symbolic QEC requirements with a fixed hardware assumption.

QEC realization remains downstream from semantic intent.

---

51. HDL Migration

HDL migration MUST preserve:

- hardware intent;
- signal identity;
- connectivity;
- timing;
- concurrency;
- state;
- reset;
- memory behavior;
- verification semantics;
- simulation semantics;
- synthesis intent;
- resource requirements.

Migration MUST distinguish:

hardware intent

from:

physical realization

A target-specific implementation may select a concrete implementation after semantic migration.

---

52. Hardware Migration

Hardware migration MUST preserve capability/resource abstractions.

The language may express:

requires capability("tensor.compute");
requires capability("gpu.compute");
requires capability("quantum.measurement");

but MUST NOT force a specific hardware identity into portable source.

Physical resources belong to target discovery, negotiation, routing, scheduling, and HAL.

---

53. Hybrid Migration

Hybrid programs may cross:

classical
quantum
AI
tensor
accelerator
HDL
distributed

Migration MUST preserve cross-domain data and control-flow semantics.

The canonical flow is:

source
  ↓
domain-neutral AST
  ↓
semantic analysis
  ↓
domain semantic models
  ↓
Classical IR + quantum::ir + other canonical representations
  ↓
optimization/lowering

Migration MUST NOT create a separate hybrid language.

---

54. Classical Migration

Classical constructs MUST preserve:

- numerical meaning;
- control flow;
- memory semantics;
- ownership;
- concurrency;
- effects;
- contracts;
- resource requirements.

A migration MUST NOT silently specialize classical semantics to one processor architecture.

---

55. Data Migration

Data migrations MUST preserve:

- schema meaning;
- value meaning;
- null/optional semantics;
- ordering where specified;
- precision;
- provenance;
- query semantics;
- serialization semantics.

External formats such as SQL, JSON, XML, and similar representations remain interoperability/dialect concerns where applicable.

Migration MUST NOT make an external interchange format part of the universal language merely because it is supported.

---

56. Dialect Migration

Dialect migrations are governed jointly by:

grammar/compatibility/dialect-version.md
grammar/compatibility/dialect-compatibility.md
grammar/dialects/

A dialect migration MUST specify:

- dialect identity;
- dialect version;
- language version dependency;
- grammar dependency;
- semantic dependency;
- AST impact;
- IR impact;
- external format impact;
- migration procedure;
- compatibility class.

A dialect MUST NOT redefine universal Zamani semantics.

---

57. Interoperability Migration

Interoperability formats may include:

- C;
- C++;
- Rust;
- Python;
- WebAssembly;
- OpenQASM;
- QIR;
- HDL;
- serialized data formats.

Migration MUST preserve the canonical Zamani semantic model.

The intended path is:

external representation
        ↓
interoperability adapter
        ↓
Zamani semantic representation
        ↓
canonical IR

not:

external representation
        ↓
permanent alternate Zamani IR

---

58. FFI and ABI Migration

FFI migrations MUST preserve:

- function identity;
- parameter layout;
- return layout;
- calling convention;
- ownership;
- lifetime assumptions;
- error behavior;
- data layout;
- effects;
- capabilities;
- ABI version.

FFI MUST remain visible to the effect and capability systems.

For example, a foreign/native operation MUST NOT silently become a pure operation.

---

59. Metaprogramming Migration

Migration of:

- reflection;
- introspection;
- compile-time execution;
- syntax trees;
- quotation;
- code generation;
- type-level computation;

MUST preserve the distinction between:

compile-time computation

and:

runtime computation

Reflection and code generation MUST retain their capability/effect requirements.

Migration MUST NOT turn controlled metaprogramming into unrestricted runtime self-modification.

---

60. Version Independence

Zamani has multiple version dimensions.

A migration MUST identify which dimensions change:

language version
grammar version
lexer version
AST version
semantic version
IR version
dialect version
artifact version
ABI version
runtime compatibility version
target compatibility version

A compiler implementation version MUST NOT automatically be treated as a language version.

The exact numbering policy remains owned by the relevant version documents.

---

61. Version Selection

Migration tooling MUST determine:

source language version
source grammar compatibility
source dialect versions
source AST/artifact version when applicable
target language version
target dialect versions
target compiler capabilities

before applying transformations.

The tool MUST NOT silently assume that:

latest compiler = latest source language

unless the language version contract explicitly defines that behavior.

---

62. No Silent Version Upgrade

A compiler MUST NOT silently reinterpret an old program as a newer language version when the interpretation could change semantics.

Where explicit version information is required, missing information MUST produce a deterministic diagnostic or use a documented default.

The selected interpretation MUST be recorded in diagnostics/provenance where required.

---

63. No Silent Version Downgrade

A compiler MUST NOT silently discard newer language semantics to make a program fit an older version.

Examples include silently dropping:

- effects;
- policies;
- contracts;
- resource requirements;
- capabilities;
- provenance;
- quantum semantics;
- type information;
- adaptation controls.

A downgrade requires explicit compatibility analysis.

---

64. Feature Gates

Feature gates are governed by:

grammar/compatibility/feature-gates.md

A feature gate controls feature availability.

It MUST NOT redefine:

- semantic meaning;
- hardware capacity;
- resource limits;
- backend identity;
- language version;
- target capability.

A migration MAY remove a feature gate when the feature becomes stable.

It MUST NOT use feature gates as a substitute for proper versioning.

---

65. Deprecation

Deprecation is governed by:

grammar/compatibility/deprecated.md

Migration MUST reference deprecation records when applicable.

A deprecated feature SHOULD have:

- replacement;
- migration ID;
- compatibility window;
- diagnostic;
- removal version;
- migration tooling where practical.

Deprecation status MUST NOT itself change semantics.

---

66. Removal

Before removing a stable feature:

[ ] Deprecated
[ ] Replacement documented
[ ] Migration defined where practical
[ ] Compatibility window elapsed
[ ] Removal version defined
[ ] Compatibility matrix updated
[ ] Diagnostics implemented
[ ] Negative tests added
[ ] Historical migration retained

Removal MUST produce deterministic diagnostics when old syntax is encountered.

Removed syntax MUST NOT silently acquire unrelated new semantics unless that behavior is explicitly specified.

---

67. Reserved Syntax

Reserved syntax is governed by:

grammar/compatibility/reserved.md

A migration that reserves a formerly legal identifier MUST:

1. identify the affected source;
2. classify the migration;
3. provide an automatic transformation where safe;
4. provide a deterministic diagnostic otherwise;
5. update compatibility metadata;
6. update lexical tests.

Keyword reservation MUST be justified by an actual language requirement.

---

68. AST Migration

AST migrations MUST preserve source semantics rather than merely tree shape.

Every AST schema migration MUST specify:

old node
new node
field mapping
default handling
removed fields
new fields
source-span mapping
metadata mapping
semantic interpretation

An AST schema change MUST NOT automatically imply a source-language breaking change.

A source-compatible grammar may produce a new AST representation while preserving semantic meaning.

---

69. AST Compatibility

AST compatibility is governed by:

grammar/compatibility/AST-version.md
grammar/compatibility/ast-conformance.md

Migration MUST specify whether the AST change is:

- additive;
- representational;
- normalization;
- structural;
- semantic;
- breaking.

Generated AST representations MUST NOT become an independent language authority.

---

70. Semantic Migration

Semantic migration is required when the meaning changes.

Examples:

old operation semantics
        ↓
new operation semantics

or:

old ownership rule
        ↓
new ownership rule

A semantic migration MUST document:

- old meaning;
- new meaning;
- motivation;
- compatibility impact;
- migration strategy;
- diagnostics;
- tests;
- affected IR;
- downstream effects.

Semantic migrations MUST NOT be disguised as parser changes.

---

71. Effect Migration

When an effect is added, removed, or reclassified, migration MUST explicitly evaluate:

type
effect
capability
policy
resource
provenance

For example:

pure operation

MUST NOT silently become:

operation with network effect

without explicit semantic treatment.

---

72. Capability Migration

Capability identities MUST be stable.

A renamed capability SHOULD have an explicit alias/migration path.

For example:

legacy.capability
        ↓
canonical.capability

The migration MUST preserve:

- requirement meaning;
- authorization;
- policy;
- target negotiation;
- provenance.

A capability migration MUST NOT silently grant additional authority.

---

73. Resource Migration

Resource expressions MUST preserve mathematical/semantic meaning.

For example:

requires memory >= required_memory;

MUST remain a requirement.

It MUST NOT become:

memory == fixed_value

unless the program itself specifies that equality.

Likewise, migration tooling MUST NOT replace symbolic dimensions with fixed implementation constants.

---

74. Constraint Migration

Constraints are not requirements and not preferences.

Migration MUST preserve:

requirement
constraint
preference
hint

as distinct concepts.

A constraint may restrict valid realizations.

A preference may influence selection.

A hint may inform optimization.

Migration MUST NOT silently change one into another.

---

75. Policy Migration

Policy migration MUST preserve scope and authority.

A policy MUST retain:

- subject;
- scope;
- condition;
- action;
- permission/prohibition;
- precedence;
- fallback;
- provenance.

Policy evaluation order MUST remain stable where order is semantically significant.

---

76. Contract Migration

Contracts MUST preserve their execution/verification phase.

A migration MUST distinguish:

compile-time requirement
runtime assertion
postcondition
invariant
assumption
guarantee
property

Moving a runtime assertion into compile-time validation is a semantic change unless explicitly specified.

---

77. Provenance Migration

Migration tooling MUST itself be provenance-aware.

At minimum, a migration record SHOULD identify:

source digest
target digest
migration ID
source version
target version
tool version
configuration
timestamp when required

A migration MUST NOT fabricate provenance.

---

78. Diagnostic Migration

Diagnostics are part of the compatibility contract where explicitly promised.

Migration MUST preserve diagnostic meaning where practical.

Diagnostics SHOULD identify:

code
severity
source span
migration ID
old construct
replacement
reason
action

Diagnostics MUST be deterministic for deterministic migrations.

---

79. Error Compatibility

Migration MUST distinguish:

source error
migration error
semantic error
resource error
capability error
policy error
target error
runtime error

A migration tool MUST NOT hide a target limitation as a source syntax error.

A resource shortage MUST NOT be reported as a grammar incompatibility.

---

80. Runtime Migration

Runtime compatibility MAY change independently of source compatibility.

A runtime migration MUST document:

- runtime contract;
- state representation;
- serialization;
- scheduling;
- recovery;
- cancellation;
- ABI;
- observability;
- provenance.

Runtime migration MUST NOT change source semantics merely because runtime internals changed.

---

81. Artifact Migration

Compiled or serialized artifacts MUST identify:

language version
grammar compatibility where relevant
AST version where relevant
semantic version where relevant
IR version
dialect versions
ABI version
target compatibility version
artifact schema version

An artifact migration MUST NOT pretend that a binary-compatible artifact is semantically portable if its target assumptions are incompatible.

---

82. IR Migration

IR migration MUST preserve semantic meaning.

An IR migration MAY:

- rename fields;
- normalize structure;
- add metadata;
- split nodes;
- merge equivalent nodes;
- change storage layout.

It MUST NOT silently change:

- operation semantics;
- effects;
- resource requirements;
- capability requirements;
- control flow;
- quantum semantics;
- HDL semantics.

IR compatibility is governed by:

grammar/compatibility/ir-version.md
grammar/compatibility/ir-conformance.md

---

83. Canonical IR Rule

Migration MUST converge onto the repository's canonical IR architecture.

The preferred model is:

current AST
      ↓
semantic model
      ↓
canonical IR
      ├── Classical IR
      └── quantum::ir

Temporary legacy IR adapters are permitted only where necessary for artifact interoperability.

They MUST NOT become permanent competing semantic architectures.

---

84. Optimization Migration

Optimization changes MUST preserve specified semantics.

An optimization migration MUST NOT change:

- observable behavior;
- effect ordering;
- resource requirements;
- contract meaning;
- quantum measurement semantics;
- synchronization;
- externally visible provenance guarantees.

Optimization MAY change target realization.

---

85. Routing Migration

Routing is a target realization concern.

Migration MUST NOT expose physical routing assumptions as portable semantic source unless explicitly target-specific.

Quantum routing MUST preserve:

- logical operation ordering;
- logical qubit identity;
- measurement semantics;
- required connectivity;
- resource requirements.

Physical mapping remains downstream.

---

86. Scheduling Migration

Scheduling migrations MUST preserve semantic ordering constraints.

The scheduler MAY change:

- execution placement;
- parallelism;
- timing;
- resource allocation;

provided the specified semantics remain unchanged.

A scheduler MUST NOT introduce a fixed global thread, device, node, or timeline count into the language.

---

87. Resilience Migration

Resilience migration MUST preserve the distinction between:

program behavior

and:

failure recovery

Recovery strategies MAY evolve.

They MUST NOT silently change the specified program result.

If recovery can change observable results, that behavior MUST be part of the semantic contract.

---

88. QEC Migration

QEC migration MUST preserve the declared logical computation.

Changes to:

- code;
- decoder;
- physical layout;
- syndrome processing;
- error model;

belong to implementation/realization unless they alter the specified logical semantics.

---

89. ZQN Migration

Where ZQN representation changes, migration MUST document:

- source semantic representation;
- ZQN version;
- serialization format;
- operation identity;
- resource metadata;
- capability metadata;
- provenance;
- target assumptions.

ZQN changes MUST NOT be used to redefine source language semantics.

---

90. HAL Migration

HAL changes are target implementation changes unless the HAL contract itself is part of a compatibility boundary.

Migration MUST preserve:

semantic request
      ↓
HAL realization

rather than making source syntax depend on a specific HAL implementation.

---

91. External Format Migration

External formats MAY evolve independently.

Examples:

OpenQASM
QIR
HDL formats
JSON
XML
SQL
WebAssembly
C ABI
C++ ABI
Rust ABI

Migration adapters MUST translate through the canonical semantic model whenever practical.

External formats MUST NOT become competing Zamani language authorities.

---

92. Dialect-Specific Migration

A dialect migration MAY define syntax unavailable in the universal language.

However:

dialect syntax
      ↓
dialect AST
      ↓
canonical semantic model

must remain the integration boundary.

A dialect MUST identify its language version dependency.

A dialect migration MUST NOT silently alter core Zamani semantics.

---

93. Future Domains

A future computational domain MUST use the same migration principles.

The integration model is:

shared lexical foundation
        ↓
shared syntax foundation
        ↓
shared types
        ↓
shared effects
        ↓
shared resources
        ↓
shared capabilities
        ↓
shared contracts
        ↓
shared policies
        ↓
shared provenance
        ↓
domain semantics
        ↓
canonical IR boundary
        ↓
target realization

A future domain MUST NOT require a second language.

---

94. Atom-to-Everywhere Migration Invariant

Migration must preserve the ability to express computation at arbitrary semantic scale.

This includes:

- tiny systems;
- embedded systems;
- single processors;
- multicore systems;
- GPUs;
- FPGAs;
- ASICs;
- accelerators;
- quantum processors;
- simulators;
- HPC systems;
- clusters;
- distributed systems;
- cloud systems;
- future computing systems;
- nanoscale/atom-scale semantic models.

The migration system MUST NOT encode a finite universe into the grammar.

Actual physical feasibility remains a property of the target realization.

---

95. Migration of Symbolic Scale

Symbolic values such as:

n
required_memory
required_qubits
tensor_shape
topology
device_count
worker_count

MUST remain symbolic when the source program expresses them symbolically.

Migration MUST NOT replace symbolic values with implementation constants.

For example:

requires qubits >= n;

MUST remain dependent on "n".

---

96. No Artificial Resource Ceilings

Migration documents, tools, tests, manifests, and examples MUST NOT define artificial universal ceilings.

Actual implementation failures such as:

out of memory
capability unavailable
resource exhausted
target unsupported

are valid runtime/compiler conditions.

They are not equivalent to:

Zamani permits no more than N

unless the semantic specification genuinely requires such a bound.

---

97. Migration and Reproducibility

A migration SHOULD support reproducible execution.

The migrated result SHOULD be reproducible from:

source
source version
migration version
migration manifest
tool version
dialect versions
configuration

Where external data participates, its identity/version/digest SHOULD be recorded where required.

---

98. Migration and Security

Migration tooling processes potentially untrusted source.

Production migration implementations MUST:

- avoid executing migrated source;
- avoid executing generated source;
- validate input;
- validate output;
- avoid hidden network access;
- avoid uncontrolled filesystem writes;
- expose failures;
- preserve provenance;
- avoid privilege escalation;
- respect sandbox/policy requirements.

---

99. Safe Rust Requirement

The production migration implementation MUST use safe Rust.

Required baseline:

Rust >= 1.97
edition = "2021"

Production Zamani migration code MUST NOT introduce:

unsafe
unsafe fn
unsafe impl
unsafe trait
unsafe { ... }

Migration implementation SHOULD prefer:

- ownership;
- borrowing;
- standard collections;
- checked arithmetic;
- explicit errors;
- deterministic iteration where ordering matters;
- safe parsing;
- immutable transformations where practical;
- explicit resource handling.

The migration architecture MUST NOT depend on "unsafe" as a correctness mechanism.

---

100. Dependency Safety

A Rust dependency may internally use implementation mechanisms unavailable to Zamani source.

That does not authorize Zamani's own production code to introduce "unsafe".

The project's direct migration implementation MUST maintain the safe-Rust requirement.

Where dependency behavior affects compatibility, the dependency version MUST be captured in the migration/tool provenance as appropriate.

---

101. Resource Safety in Migration Tools

Migration tools process potentially large source programs.

They MUST avoid arbitrary fixed-size assumptions such as:

maximum source file size
maximum AST nodes
maximum migration records
maximum declarations
maximum nesting depth
maximum number of modules

unless a bound is required for a specific algorithm and is explicitly an implementation safeguard rather than a language semantic limit.

Where practical, migration processing SHOULD use streaming, incremental, or resource-aware algorithms.

---

102. Large-Program Migration

Migration tooling MUST support programs whose size is limited by available implementation resources rather than an artificial language ceiling.

The architecture SHOULD allow:

small source
large source
very large source
distributed source
incrementally migrated source

without changing semantic rules.

---

103. Incremental Migration

A migration tool MAY operate incrementally.

Incremental migration MUST preserve the same final semantics as complete migration.

For:

source A + source B + source C

migrating components independently MUST NOT produce a different semantic result merely because migration occurred incrementally, provided all compatibility contracts are satisfied.

---

104. Partial Migration

A partially migrated program MUST have explicit status.

Allowed states include:

UNMIGRATED
PARTIALLY_MIGRATED
MIGRATION_REQUIRED
MIGRATION_BLOCKED
MIGRATED
VALIDATED
COMPATIBLE

A partially migrated representation MUST NOT be presented as fully compatible.

---

105. Migration Idempotence

A migration SHOULD be idempotent.

Applying the same migration twice SHOULD NOT produce a different result.

Formally:

M(M(source)) = M(source)

If idempotence is impossible, the migration MUST explicitly document why.

Migration tooling SHOULD detect already-migrated constructs.

---

106. Migration Ordering

When multiple migrations apply, the tool MUST use an explicit dependency order.

Each migration SHOULD declare:

requires_migration:
conflicts_with:
supersedes:
superseded_by:

The migration engine MUST reject ambiguous migration ordering rather than selecting arbitrary order.

---

107. Migration Dependencies

A migration MUST declare dependencies on:

language version
grammar version
AST version
semantic version
IR version
dialect version
artifact version

where applicable.

Migration dependencies MUST form a deterministic graph.

Cycles MUST be rejected unless the compatibility specification explicitly defines a fixed-point process.

---

108. Migration Conflicts

Two migrations conflict when applying both could produce incompatible semantics.

Examples:

migration A: old operator → meaning A
migration B: old operator → meaning B

The migration engine MUST detect the conflict.

It MUST NOT select a migration arbitrarily.

Conflicts MUST produce deterministic diagnostics.

---

109. Migration Supersession

A migration may supersede another migration.

Supersession MUST identify:

old migration
new migration
reason
effective version
semantic relationship

Historical migrations MUST remain documented.

Supersession MUST NOT erase provenance.

---

110. Migration Rollback

Where practical, migration tooling SHOULD support rollback.

Rollback MUST be defined as:

current representation
        ↓
inverse migration
        ↓
previous representation

A rollback is guaranteed only when the migration is explicitly reversible.

A migration MUST NOT claim reversibility when information was intentionally discarded.

---

111. Lossy Migration

A lossy migration is any migration that removes information.

It MUST explicitly identify:

lost information
reason
semantic impact
developer action

Lossy migration MUST NOT be classified as transparent compatibility.

---

112. Semantic Equivalence

Semantic equivalence MUST be evaluated at the semantic-model boundary rather than textual similarity.

Two programs may have different syntax while being semantically equivalent.

Conversely, two programs may have identical syntax while receiving different semantics under different language versions.

Therefore:

textual equality ≠ semantic equality
parse equality ≠ semantic equality
AST equality ≠ semantic equality

The semantic contract is authoritative.

---

113. Equivalence Validation

Where practical, migration validation SHOULD compare:

old semantic representation

with:

new semantic representation

using:

- canonicalization;
- structural comparison;
- semantic invariants;
- type checking;
- effect checking;
- resource checking;
- capability checking;
- contract checking;
- policy checking;
- domain-specific equivalence.

For quantum programs, equivalence MUST respect quantum semantic rules rather than textual gate sequence equality alone.

---

114. Quantum Equivalence

Quantum migration validation MUST account for:

- global operation order;
- control dependencies;
- measurement;
- reset;
- classical feedback;
- state preparation;
- entanglement;
- parameter values;
- observable behavior.

Equivalent circuits need not have identical physical gate decompositions.

Physical decomposition is downstream.

---

115. HDL Equivalence

HDL migration validation MUST distinguish:

behavioral equivalence

from:

structural identity

A structurally different implementation MAY be compatible when behavioral semantics remain equivalent.

---

116. Concurrency Equivalence

Concurrency migration MUST preserve required:

- happens-before relationships;
- synchronization;
- atomicity;
- message ordering;
- cancellation;
- ownership;
- deterministic behavior guarantees.

A migration MUST NOT silently introduce data races or change specified synchronization semantics.

---

117. Distributed Equivalence

Distributed migration MUST preserve:

- message semantics;
- consistency requirements;
- ordering guarantees;
- failure semantics;
- topology requirements;
- capability requirements;
- resource requirements.

A migration MUST NOT introduce a fixed node count.

---

118. Determinism

Where the original semantics are deterministic, migration MUST preserve determinism unless the specification explicitly changes it.

Where nondeterminism is already permitted, migration MUST preserve its permitted scope.

The migration tool itself SHOULD remain deterministic.

---

119. Randomness

If randomness is part of program semantics, migration MUST preserve:

- randomness effect;
- source of randomness where specified;
- seeding semantics where specified;
- reproducibility policy where specified.

A migration MUST NOT silently replace secure randomness with deterministic pseudo-randomness or vice versa.

---

120. Learning and Adaptation Determinism

Learning and adaptation MAY inherently involve nondeterminism.

Migration MUST distinguish:

semantic nondeterminism

from:

tool nondeterminism

Migration tooling MUST NOT introduce additional nondeterminism without documentation.

---

121. Diagnostics for Unsupported Migration

If a migration cannot be completed, the tool MUST report:

migration ID
feature
source location
source version
target version
reason
required action
affected semantic layer

The tool MUST NOT silently leave an unsupported construct unchanged and report successful migration.

---

122. Migration Preconditions

Before transformation, tooling SHOULD verify:

source parses
source version known
dialects known
migration available
dependencies satisfied
migration conflicts absent
required metadata available

If preconditions fail, migration MUST stop or explicitly mark the affected portion unresolved.

---

123. Migration Postconditions

After transformation:

new source parses
AST validates
types validate
effects validate
resources validate
capabilities validate
contracts validate
policies validate
provenance validates
semantic model validates
canonical IR conversion succeeds

where applicable.

A migration cannot be marked complete merely because textual transformation succeeded.

---

124. Migration Validation Pipeline

The normative pipeline is:

old source
    ↓
version identification
    ↓
migration discovery
    ↓
dependency/conflict resolution
    ↓
source transformation
    ↓
lexical validation
    ↓
parser validation
    ↓
AST validation
    ↓
name resolution
    ↓
type validation
    ↓
effect validation
    ↓
resource validation
    ↓
capability validation
    ↓
contract validation
    ↓
policy validation
    ↓
provenance validation
    ↓
semantic validation
    ↓
canonical IR
    ↓
domain IR validation
    ↓
artifact validation where applicable

---

125. Repository Integration Contract

Every migration MUST identify affected repository layers.

At minimum:

Specification
Lexer
Parser
AST
Semantic
IR
Compiler
Runtime
Tooling
Tests
Documentation

For domain-specific migrations also identify:

Classical
Quantum
HDL
Hybrid
AI
Data
Distributed
Networking
Security
Interoperability
Dialect

---

126. Independent File Completion Contract

A migration-related file MUST be independently completable.

Its contract MUST identify:

Purpose
Owns
Does Not Own
Inputs
Dependencies
Outputs
Public identifiers
Version dependencies
AST impact
Semantic impact
IR impact
Diagnostics
Tests
Downstream consumers
Completion criteria

The file MUST NOT depend on undocumented future edits elsewhere.

---

127. Cross-File Integration Contract

Before a migration is marked complete, all affected files MUST be known.

Typical integration:

compatibility/migrations.md
        ↓
compatibility/versions.md
        ↓
compatibility/*-version.md
        ↓
compatibility/compatibility-matrix.md
        ↓
compatibility/deprecated.md
        ↓
specification/
        ↓
spec/
        ↓
grammar/Zamani.g4
        ↓
lexer
        ↓
parser
        ↓
AST
        ↓
semantic analysis
        ↓
IR
        ↓
compiler
        ↓
runtime/HAL
        ↓
tests

No hidden dependency is permitted.

---

128. Feature Integration Matrix

Every migrated feature SHOULD be traceable through:

Layer| Required record
Specification| normative feature definition
Compatibility| migration ID/class
Lexer| token impact
Grammar| syntax impact
Parser| parser impact
AST| node/field mapping
Semantic| meaning mapping
Types| type impact
Effects| effect impact
Resources| resource impact
Capabilities| capability impact
Contracts| contract impact
Policies| policy impact
Provenance| provenance impact
IR| canonical representation
Quantum| "quantum::ir" impact when applicable
Compiler| lowering/optimization impact
Runtime| runtime contract
Target| realization impact
Tests| conformance evidence
Documentation| user-facing migration

---

129. UBUNTU-Derived Semantic Integration

The discussed additional computational capabilities MUST migrate into Zamani's existing universal architecture rather than create a parallel language.

Relevant semantic categories include:

infer
deduce
reason
assert
retract
query
learn
adapt
match
guards
requires
ensures
invariant
assume
guarantee
property
uncertainty
probability
confidence
evidence
provenance
explainability
decision records
policies
sandboxing
simulation
multi-agent execution
neural-symbolic composition
FFI
ABI
reflection
metaprogramming
richer type-system capabilities
data interoperability
quantum-classical hybrid computation
adaptive execution
deterministic/reproducible execution

Migration MUST place these capabilities into their existing Zamani owners.

Examples:

reasoning
    → grammar/ai/
    → semantic reasoning model

knowledge
    → grammar/ai/
    → data/knowledge where appropriate

learning
    → grammar/ai/

adaptation
    → grammar/ai/
    → grammar/execution/
    → grammar/policies/

contracts
    → grammar/validation/

sandbox
    → grammar/security/

simulation
    → grammar/execution/

agents
    → grammar/ai/
    → existing concurrency actor semantics

FFI/ABI
    → grammar/interoperability/

reflection
    → grammar/metaprogramming/

uncertainty
    → grammar/types/
    → grammar/ai/

quantum-classical hybrid
    → grammar/hybrid/

Migration MUST NOT introduce application-specific universal keywords for individual industries or applications.

Such functionality belongs in:

- libraries;
- dialects;
- capabilities;
- policies;
- services;
- applications.

---

130. Application-Specific Syntax Migration

Application-specific concepts SHOULD remain identifiers, library APIs, dialect constructs, or capability names unless there is a compelling universal language requirement.

Examples include:

vision
robotics
sentiment
payments
administration
legal workflows
blockchain applications
VR
AR
domain-specific business logic

Migration MUST NOT turn every library concept into a core keyword.

This preserves lexical scalability and prevents application-domain keyword explosion.

---

131. AI Migration

AI-related migrations MUST use the common:

types
effects
resources
capabilities
contracts
policies
provenance

architecture.

AI syntax MUST NOT become a separate semantic universe.

AI migration MUST preserve:

- model identity;
- data identity;
- training/evaluation semantics;
- inference semantics;
- uncertainty;
- evidence;
- provenance;
- resource requirements;
- capabilities;
- policy;
- effects.

---

132. Neural-Symbolic Migration

Neural-symbolic migrations MUST preserve the relationship among:

learned model
symbolic representation
reasoning
evidence
provenance

A migration MUST NOT silently convert a learned result into a formally proven fact.

Likewise, symbolic assertions MUST NOT automatically become learned predictions.

---

133. Explainability Migration

Explainability metadata MUST distinguish:

explanation
evidence
proof
provenance
confidence
decision

These concepts are not interchangeable.

Migration MUST preserve those distinctions.

---

134. Knowledge Provenance

Knowledge migration MUST preserve the source of facts.

For:

assert fact

the semantic record MAY contain:

fact
source
confidence
timestamp
derivation
provenance

Migration MUST NOT discard provenance when the contract requires it.

---

135. Reflection Migration

Reflection migrations MUST preserve authority boundaries.

Reflection MAY inspect:

- types;
- metadata;
- declarations;
- capabilities;
- source structure;
- semantic structures;

subject to policy and capability constraints.

Reflection MUST NOT silently acquire the authority to modify arbitrary program semantics.

---

136. Code Generation Migration

Generated code must remain distinguishable from source-authoritative semantics.

Migration MUST preserve:

generator identity
generator version
source input
generated output
provenance

Generated output MUST NOT silently become a new language authority.

---

137. Compilation Model Migration

The compilation model remains:

source
 ↓
frontend
 ↓
semantic validation
 ↓
canonical IR
 ↓
optimization
 ↓
lowering
 ↓
routing/scheduling
 ↓
resilience/QEC
 ↓
ZQN
 ↓
HAL
 ↓
target

A migration MUST NOT bypass semantic validation merely because an old representation already contains a low-level form.

---

138. Migration Across Compiler Generations

A newer compiler MAY migrate older source directly.

Preferred:

old source
    ↓
current migration layer
    ↓
current AST
    ↓
current semantic model
    ↓
current IR

Avoid:

old source
    ↓
old compiler
    ↓
old permanent IR
    ↓
new compiler

unless artifact compatibility explicitly requires the latter.

---

139. Legacy Artifact Import

Legacy artifacts MAY be imported through dedicated compatibility adapters.

The adapter MUST:

1. identify artifact version;
2. validate integrity;
3. decode safely;
4. validate schema;
5. migrate representation;
6. validate semantic meaning;
7. convert to canonical IR;
8. record provenance.

A legacy artifact adapter MUST NOT become a permanent semantic authority.

---

140. Serialization Migration

Serialized representations MUST include sufficient version identity to determine the correct migration.

If version identity is absent and cannot be safely inferred, the importer MUST report an explicit error rather than guessing.

---

141. Migration Security Boundaries

Migration input MUST be considered untrusted.

The migration implementation MUST NOT:

- execute arbitrary source;
- execute arbitrary generated code;
- load arbitrary native libraries;
- invoke external commands;
- make hidden network calls.

Any explicitly authorized external integration MUST occur through a separately specified tool boundary.

---

142. Migration Tool Isolation

A migration tool SHOULD operate as a pure transformation:

input
  ↓
parse
  ↓
transform
  ↓
validate
  ↓
output

External state SHOULD be minimized.

Where external state is required, it MUST be explicit and recorded for reproducibility.

---

143. Migration Logging

Migration tools SHOULD produce structured logs containing:

migration_id
source_version
target_version
file
span
action
status
diagnostic

Logs MUST NOT contain secrets or unrelated sensitive information.

---

144. Migration Provenance Chain

The preferred provenance graph is:

source
  ↓
migration-1
  ↓
source-1
  ↓
migration-2
  ↓
source-2
  ↓
current AST
  ↓
semantic model
  ↓
canonical IR

Each transformation SHOULD be independently identifiable.

This allows historical migration chains to remain auditable.

---

145. Migration Chain Compression

Multiple compatible migrations MAY be composed into one direct migration.

A direct migration MUST produce semantics equivalent to applying the individual migrations in order.

For:

M1
M2
M3

a composed migration:

M1→3

is valid only if:

M1→3(source)

is semantically equivalent to:

M3(M2(M1(source)))

---

146. Migration Chain Validation

The repository SHOULD test both:

sequential migration

and:

direct migration

when direct migration exists.

The resulting canonical semantic representation MUST agree.

---

147. Compatibility Matrix Integration

Every migration that changes compatibility MUST update:

grammar/compatibility/compatibility-matrix.md

The matrix entry MUST identify:

- source version;
- target version;
- compatibility class;
- migration ID;
- applicable layers;
- artifact compatibility;
- dialect compatibility;
- target compatibility.

"migrations.md" defines the procedure.

"compatibility-matrix.md" records the relationship.

---

148. Version Document Integration

Migration MUST reference the relevant version document rather than redefining version numbering.

Relevant files include:

language-version.md
grammar-version.md
AST-version.md
semantic-version.md
ir-version.md
dialect-version.md
target-compatibility-version.md
versions.md

A migration MUST NOT introduce an independent version numbering system.

---

149. Deprecation Integration

A migration involving deprecated syntax MUST reference:

grammar/compatibility/deprecated.md

The migration record MUST identify:

deprecated feature
migration ID
replacement
compatibility window
removal version

---

150. Feature-Gate Integration

A feature-gated migration MUST reference:

grammar/compatibility/feature-gates.md

The migration MUST distinguish:

feature availability

from:

language semantics

A disabled feature MUST NOT silently reinterpret the source as another feature.

---

151. Dialect Integration

Dialect migrations MUST reference:

grammar/compatibility/dialect-version.md
grammar/compatibility/dialect-compatibility.md
grammar/dialects/

The migration MUST identify whether the change affects:

syntax
AST
semantics
IR
external format
runtime
target

---

152. Frontend Conformance Integration

A migration affecting frontend behavior MUST satisfy:

grammar/compatibility/frontend-conformance.md

This includes:

- lexer;
- parser;
- diagnostics;
- source spans;
- AST generation.

Parsing alone is insufficient.

---

153. AST Conformance Integration

A migration affecting AST representation MUST satisfy:

grammar/compatibility/ast-conformance.md

The AST must remain semantically sufficient for downstream processing.

---

154. IR Conformance Integration

A migration affecting IR MUST satisfy:

grammar/compatibility/ir-conformance.md

For quantum features, this includes verification against "quantum::ir".

---

155. Validation Integration

Migration validation MUST integrate with:

grammar/validation/

Relevant validation responsibilities include:

- source spans;
- unreachable rules;
- semantic coverage;
- semantic boundaries;
- cross-layer validation;
- compatibility checks.

Migration validation MUST NOT duplicate validation rules owned elsewhere.

---

156. Tests

Every migration MUST have applicable tests.

Minimum categories:

positive
negative
boundary
scalability
determinism
compatibility
diagnostics
provenance

Domain-specific migrations additionally require:

quantum
HDL
classical
hybrid
AI
distributed
networking
interoperability

where applicable.

---

157. Positive Tests

Positive tests MUST demonstrate that valid legacy input migrates successfully.

They SHOULD test:

- minimal program;
- representative program;
- nested constructs;
- generic constructs;
- cross-domain constructs;
- metadata;
- contracts;
- resources;
- capabilities;
- effects.

---

158. Negative Tests

Negative tests MUST demonstrate deterministic rejection of:

- invalid source;
- unsupported migration;
- ambiguous migration;
- conflicting migration;
- incompatible versions;
- invalid semantic conversion;
- invalid artifact;
- unsupported dialect;
- unavailable required capability where validation requires it.

---

159. Boundary Tests

Boundary tests MUST test transitions between:

old/new syntax
old/new lexer
old/new AST
old/new semantics
old/new IR
old/new dialect
old/new artifact

Boundary tests MUST be especially strong for:

- quantum/classical boundaries;
- AI/classical boundaries;
- hybrid boundaries;
- HDL/software boundaries;
- dialect boundaries;
- FFI/ABI boundaries.

---

160. Scalability Tests

Scalability tests MUST demonstrate that migration does not introduce artificial capacity limits.

Tests SHOULD use symbolic scale rather than fixed universal maxima.

For example:

n
required_memory
tensor_shape
qubit_count
worker_count
topology

may vary according to the test environment.

The purpose is to prove that the migration algorithm does not contain hidden language ceilings.

---

161. Determinism Tests

Repeated migration of identical input under identical conditions MUST produce identical canonical results for deterministic migrations.

Tests SHOULD compare:

migrated source
AST
semantic representation
canonical IR
migration manifest

where stable serialization exists.

---

162. Compatibility Tests

Compatibility tests MUST cover:

old compiler/frontend
new compiler/frontend
old source
new source
old artifact
new artifact
old dialect
new dialect

as applicable.

The tests MUST verify semantic behavior rather than merely successful parsing.

---

163. Cross-Domain Tests

A production migration system MUST test cross-domain composition.

At minimum, the repository SHOULD contain migration cases combining:

classical + quantum
classical + AI
quantum + AI
quantum + HDL
AI + distributed
data + AI
resources + quantum
policies + effects
contracts + adaptation
provenance + reasoning
FFI + effects

where those features exist.

---

164. POCO-REAF Integration Test

The repository MUST maintain an end-to-end migration test representing the portability architecture.

Conceptually:

program intent
    ↓
requirements
capabilities
resources
constraints
preferences
policies
contracts
provenance
    ↓
current AST
    ↓
semantic model
    ↓
Classical IR
+
quantum::ir
    ↓
optimization
    ↓
lowering
    ↓
routing
    ↓
scheduling
    ↓
resilience
    ↓
QEC
    ↓
ZQN
    ↓
HAL

The test MUST verify that migration does not introduce target-specific assumptions into source semantics.

---

165. Migration Acceptance Matrix

A production migration SHOULD be accepted only when:

Area| Required
Specification| PASS
Version identity| PASS
Migration class| PASS
Source transformation| PASS
Lexer| PASS where affected
Parser| PASS where affected
AST| PASS where affected
Types| PASS where affected
Effects| PASS where affected
Resources| PASS where affected
Capabilities| PASS where affected
Contracts| PASS where affected
Policies| PASS where affected
Provenance| PASS where affected
Semantic equivalence| PASS
Canonical IR| PASS
"quantum::ir"| PASS where affected
Artifact| PASS where affected
ABI| PASS where affected
Runtime| PASS where affected
Target| PASS where affected
Tests| PASS
Scalability| PASS
Determinism| PASS
Safe Rust| PASS
Documentation| PASS
Compatibility metadata| PASS

---

166. Production Completion Checklist

A migration is production-complete only when:

[ ] Migration ID exists
[ ] Feature ID exists
[ ] Source version identified
[ ] Target version identified
[ ] Migration class identified
[ ] Affected layers identified
[ ] Affected domains identified

[ ] Old syntax documented
[ ] New syntax documented
[ ] Lexer mapping documented
[ ] Parser mapping documented
[ ] AST mapping documented
[ ] Semantic mapping documented
[ ] Type mapping documented
[ ] Effect mapping documented
[ ] Resource mapping documented
[ ] Capability mapping documented
[ ] Contract mapping documented
[ ] Policy mapping documented
[ ] Provenance mapping documented

[ ] Canonical IR mapping documented
[ ] quantum::ir impact verified where applicable
[ ] HDL/hardware impact verified where applicable

[ ] Artifact impact documented
[ ] ABI impact documented
[ ] Runtime impact documented
[ ] Target impact documented

[ ] Automatic transformation defined
[ ] Ambiguity handling defined
[ ] Manual actions defined
[ ] Diagnostics defined
[ ] Source-span mapping defined
[ ] Comment policy defined
[ ] Formatting policy defined

[ ] Version metadata updated
[ ] Compatibility matrix updated
[ ] Deprecation metadata updated where applicable
[ ] Feature-gate metadata updated where applicable
[ ] Dialect metadata updated where applicable

[ ] Provenance implemented
[ ] Positive tests pass
[ ] Negative tests pass
[ ] Boundary tests pass
[ ] Scalability tests pass
[ ] Determinism tests pass
[ ] Compatibility tests pass
[ ] Diagnostic tests pass

[ ] No artificial resource ceiling introduced
[ ] No target identity leaked into portable semantics
[ ] No duplicate permanent IR introduced
[ ] No duplicate quantum semantic architecture introduced
[ ] No hidden cross-file dependency exists
[ ] Safe-Rust audit passes
[ ] Documentation synchronized

Only after all applicable checks pass may a migration be marked:

PRODUCTION_COMPLETE

---

167. Migration Status

The following statuses are recommended:

PLANNED
DESIGNED
IMPLEMENTING
IMPLEMENTED
VALIDATING
MIGRATION_AVAILABLE
COMPATIBLE
REQUIRES_MANUAL_ACTION
BLOCKED
DEPRECATED
REMOVAL_ELIGIBLE
REMOVED

These statuses MUST NOT be conflated.

In particular:

IMPLEMENTED ≠ COMPATIBLE
COMPATIBLE ≠ STABLE
DEPRECATED ≠ REMOVED
MIGRATION_AVAILABLE ≠ MIGRATION_COMPLETE

---

168. Breaking Migration Requirements

A migration is breaking only when the compatibility contract cannot preserve the previous source or semantic behavior.

A breaking migration MUST include:

1. reason;
2. affected versions;
3. affected features;
4. affected source patterns;
5. semantic explanation;
6. migration procedure;
7. diagnostics;
8. compatibility matrix entry;
9. deprecation relationship where applicable;
10. tests;
11. replacement guidance where applicable.

Implementation inconvenience MUST NOT be sufficient justification.

---

169. Semantic Breaking Changes

A semantic breaking change MUST be explicitly identified.

Examples include:

evaluation order changed
ownership changed
effect changed
resource meaning changed
capability authority changed
policy behavior changed
quantum measurement meaning changed
HDL timing meaning changed
concurrency ordering changed
distributed consistency changed

A parser-only description is insufficient.

---

170. Migration of Reserved Names

If a previously valid identifier becomes reserved:

old identifier
      ↓
identifier collision

the migration system MUST:

1. detect the collision;
2. report the exact location;
3. provide a deterministic replacement where possible;
4. avoid changing references incorrectly;
5. preserve bindings;
6. update source-span mappings;
7. validate the entire program after transformation.

---

171. Binding-Preserving Renames

A rename migration MUST preserve binding identity.

For:

declaration
references

all references MUST resolve to the same declaration after migration.

Textual replacement alone is insufficient when shadowing exists.

The migration system MUST understand lexical/symbol scopes sufficiently to avoid accidental renaming of unrelated identifiers.

---

172. Module Migration

Module migrations MUST preserve:

- module identity;
- imports;
- exports;
- visibility;
- namespace;
- initialization behavior;
- dependency graph.

If a module is renamed, migration MUST update references semantically rather than through unsafe textual substitution.

---

173. Function Migration

Function migration MUST preserve:

- function identity;
- parameter order;
- parameter types;
- return type;
- generics;
- effects;
- ownership;
- asynchronous behavior;
- capabilities;
- resource requirements.

Changing syntax is not automatically semantic.

Changing calling semantics is.

---

174. Generic Migration

Generic migrations MUST preserve:

- type parameters;
- bounds;
- constraints;
- variance where specified;
- associated types;
- specialization semantics where specified.

A migration MUST NOT erase generic constraints merely because the current backend cannot specialize them.

---

175. Linear and Affine Type Migration

Where linear or affine types are supported, migration MUST preserve their usage guarantees.

The migration MUST NOT silently weaken:

must-use-once

or:

may-use-at-most-once

semantics.

---

176. Dependent-Type Migration

Where dependent-type semantics are supported, migration MUST preserve the relationship between:

type
value
constraint
proof/validation

A dependent constraint MUST NOT be replaced with an implementation constant unless the source explicitly defines that value.

---

177. Contract and Type Interaction

When a type migration changes a contract-relevant property, both layers MUST be migrated together.

For example:

type constraint
+
requires
+
ensures

must retain their semantic relationship.

---

178. Resource and Type Interaction

If a type encodes a resource requirement, migration MUST preserve that requirement symbolically.

For example:

Qubit[n]

must not become a type with an implementation-specific fixed cardinality.

---

179. Capability and Effect Interaction

Capability migrations MUST be checked against effect migrations.

An operation requiring:

effect(network)

and:

capability("network.connect")

must not be migrated to a pure operation merely because the syntax changed.

---

180. Policy and Adaptation Interaction

Adaptation migrations MUST validate policy compatibility.

A policy that restricts adaptation MUST remain effective after migration.

Migration MUST NOT bypass:

authorization
policy
capability
effect
provenance

checks.

---

181. Provenance and Migration Interaction

Migration itself is a transformation and therefore SHOULD be represented in provenance.

The provenance graph SHOULD allow:

source artifact
    ↓
migration
    ↓
derived artifact

to be independently audited.

---

182. Migration of Generated Grammar

Generated grammar artifacts MUST identify:

source authority
generator
generator version
input versions
generation timestamp where required

Generated files MUST NOT become semantic authorities.

---

183. Migration of "grammar.md"

"grammar/grammar.md" is a conformance/status artifact.

Migration MUST preserve its distinction among statuses such as:

SPECIFIED
IMPLEMENTED
PARTIALLY IMPLEMENTED
PLANNED
DEPRECATED

and additional implementation statuses where used:

AST_IMPLEMENTED
SEMANTIC_IMPLEMENTED
IR_IMPLEMENTED
TESTED
STABLE
EXPERIMENTAL

Migration MUST NOT use "grammar.md" as a substitute for normative specification.

---

184. Migration of "Zamani-Grammar.md"

"grammar/Zamani-Grammar.md" remains historical/extended reference material.

A feature appearing there MUST NOT automatically become stable language syntax.

Migration status MUST respect its labels, including:

stable
proposed
experimental
deprecated
historical
not implemented

---

185. Migration of "README.md"

The compatibility README is navigation/architecture material.

Migration MUST NOT make the README a competing normative source.

---

186. Migration of "DESIGN.md"

All migration rules MUST comply with the architectural principles in:

grammar/DESIGN.md

If a proposed migration conflicts with the architecture, the migration MUST be blocked until the authoritative architecture is intentionally changed.

"migrations.md" MUST NOT override "DESIGN.md".

---

187. Migration of Specification

When a migration changes language meaning, the normative specification MUST be updated before the migration is declared complete.

Required order:

semantic decision
      ↓
specification
      ↓
compatibility classification
      ↓
migration design
      ↓
implementation
      ↓
tests

A grammar edit MUST NOT be used to establish semantics retroactively.

---

188. Migration of Machine Contracts

Machine-oriented contracts under "grammar/spec/" MUST be updated when their owned contract changes.

Migration records MUST link to the relevant machine contract.

Duplicating machine contracts inside this file is prohibited.

---

189. Repository Consistency Rule

The repository MUST NOT reach a state where:

specification says A
grammar accepts B
AST means C
semantic layer means D
IR means E

for the same feature.

A migration MUST identify and resolve such inconsistencies before production completion.

---

190. Semantic Gap Rule

A migration is blocked if it creates an unresolved semantic gap between:

source
AST
semantic model
IR
runtime

A syntactically accepted migration without a complete semantic path MUST be considered incomplete.

---

191. Domain-Neutral Migration Rule

The migration layer itself MUST remain domain-neutral.

It may understand domain-specific migration metadata, but it MUST not become a second implementation of:

- quantum semantics;
- AI semantics;
- HDL semantics;
- networking semantics;
- distributed semantics.

Those remain owned by their domains.

---

192. Future Hardware Rule

Migration MUST remain valid when new hardware classes appear.

The migration architecture MUST NOT require adding a new universal migration rule merely because a new target appears.

New targets should normally integrate through:

capabilities
resources
constraints
topology
target compatibility
HAL

rather than changing source semantics.

---

193. Future Computing Model

If a future computing model introduces a new semantic domain, its migration MUST integrate through the common foundations:

types
operations
effects
resources
capabilities
contracts
policies
provenance

followed by a domain semantic model and canonical IR boundary.

---

194. Migration of Nano/Atom-Scale Semantics

Nano- and atom-scale semantic constructs MUST remain abstract.

Migration MUST NOT define a finite physical universe.

The language may describe:

atom
molecule
material
interaction
nano-structure
quantum state

but migration MUST NOT introduce a universal maximum count of physical entities.

Physical feasibility remains target-specific.

---

195. Temporal and Multi-Timeline Migration

Where temporal or timeline semantics exist, migration MUST preserve:

- timeline identity;
- temporal ordering;
- branching;
- merging;
- observation;
- speculative execution;
- rollback;
- causality.

No universal timeline count may be introduced.

---

196. Sankofa-Related Constructs

Where concepts associated with the repository's interoperability or historical computational model are represented in Zamani, migration MUST preserve their semantic identity without automatically making every historical term a permanent keyword.

Historical or experimental constructs MUST remain clearly classified.

Migration MUST pass through:

specification
→ syntax
→ AST
→ semantics
→ canonical IR
→ tests

before becoming stable.

---

197. Migration of Compiler Self-Description

Historical/extended grammar material may describe compiler-oriented constructs such as:

bootstrap
compiler
grammar_snapshot
self_compile
grammar_diff
compiler_snapshot

Such constructs MUST NOT automatically become stable Zamani syntax.

If accepted, they must have:

- specification;
- syntax;
- AST;
- semantic owner;
- implementation contract;
- migration record;
- tests.

---

198. Monolithic-to-Modular Migration

Migration from a broad monolithic grammar architecture to modular files MUST follow:

existing rule
      ↓
canonical owner identified
      ↓
modular rule
      ↓
same token semantics
      ↓
same AST semantics
      ↓
same semantic meaning
      ↓
same canonical IR

The migration MUST NOT produce:

old language + new language

It must produce:

one language
one semantic model
one canonical architecture

---

199. No Unnecessary Renames

Existing authoritative filenames MUST be retained unless there is a documented ownership reason to rename them.

In particular, migration MUST preserve the existing compatibility architecture rather than creating redundant parallel files.

Existing dedicated files such as:

language-version.md
grammar-version.md
AST-version.md
semantic-version.md
ir-version.md
dialect-version.md
target-compatibility-version.md
versions.md
migrations.md
deprecated.md
reserved.md
feature-gates.md
compatibility-matrix.md
dialect-compatibility.md
frontend-conformance.md
ast-conformance.md
ir-conformance.md

retain their distinct responsibilities.

---

200. Required Integration Graph

The complete migration integration graph is:

grammar/DESIGN.md
        │
        ▼
grammar/specification/
        │
        ▼
grammar/spec/
        │
        ▼
compatibility version contracts
        │
        ▼
migrations.md
        │
        ├── deprecated.md
        ├── reserved.md
        ├── feature-gates.md
        ├── compatibility-matrix.md
        └── dialect-compatibility.md
        │
        ▼
grammar/Zamani.g4
        │
        ▼
lexer
        │
        ▼
parser
        │
        ▼
AST
        │
        ▼
validation
        │
        ▼
semantic analysis
        │
        ├── types
        ├── effects
        ├── resources
        ├── capabilities
        ├── contracts
        ├── policies
        └── provenance
        │
        ▼
domain semantics
        │
        ├── classical
        ├── quantum
        ├── HDL
        ├── AI
        ├── data
        ├── distributed
        ├── networking
        └── hybrid
        │
        ▼
canonical IR
        │
        ├── Classical IR
        └── quantum::ir
        │
        ▼
compiler
        │
        ├── optimization
        ├── lowering
        ├── routing
        ├── scheduling
        ├── resilience
        └── QEC
        │
        ▼
ZQN
        │
        ▼
HAL
        │
        ▼
runtime / target

---

201. Required Per-File Migration Contract

Any repository file modified because of a migration SHOULD have an accompanying integration contract:

FILE:
PURPOSE:
OWNER:
DOES_NOT_OWN:

INPUTS:
DEPENDENCIES:

EXPORTS:
CONSUMERS:

VERSION_DEPENDENCIES:

LEXER_IMPACT:
GRAMMAR_IMPACT:
PARSER_IMPACT:
AST_IMPACT:
SEMANTIC_IMPACT:
TYPE_IMPACT:
EFFECT_IMPACT:
RESOURCE_IMPACT:
CAPABILITY_IMPACT:
CONTRACT_IMPACT:
POLICY_IMPACT:
PROVENANCE_IMPACT:
IR_IMPACT:
QUANTUM_IR_IMPACT:
HDL_IMPACT:
RUNTIME_IMPACT:
TARGET_IMPACT:

DIAGNOSTICS:

POSITIVE_TESTS:
NEGATIVE_TESTS:
BOUNDARY_TESTS:
SCALABILITY_TESTS:
DETERMINISM_TESTS:

COMPATIBILITY:
DEPRECATION:
REMOVAL:

HARD_CODING_AUDIT:
SAFE_RUST_AUDIT:

COMPLETION_CRITERIA:

This contract ensures that a file can be completed independently without being reopened merely because another file is subsequently implemented.

---

202. Migration Review

Every production migration SHOULD receive review against:

Authority

[ ] Correct specification owner
[ ] Correct implementation owner
[ ] No competing authority

Syntax

[ ] Lexer reviewed
[ ] Grammar reviewed
[ ] Parser reviewed
[ ] Ambiguity reviewed
[ ] Precedence reviewed

AST

[ ] Node mapping reviewed
[ ] Fields reviewed
[ ] Bindings preserved
[ ] Source spans reviewed

Semantics

[ ] Meaning preserved or explicitly changed
[ ] Types reviewed
[ ] Effects reviewed
[ ] Resources reviewed
[ ] Capabilities reviewed
[ ] Contracts reviewed
[ ] Policies reviewed
[ ] Provenance reviewed

IR

[ ] Canonical IR mapping reviewed
[ ] quantum::ir reviewed where applicable
[ ] No duplicate permanent IR

Target

[ ] Target independence preserved
[ ] Resource realization remains downstream
[ ] Routing remains downstream
[ ] Scheduling remains downstream
[ ] QEC remains downstream
[ ] HAL remains downstream

Safety

[ ] No Rust unsafe
[ ] No source execution
[ ] No hidden network access
[ ] No uncontrolled filesystem modification

Tests

[ ] Positive
[ ] Negative
[ ] Boundary
[ ] Scalability
[ ] Determinism
[ ] Compatibility
[ ] Diagnostics
[ ] Provenance

---

203. Production Readiness Gate

A migration system is production-ready only when:

EVERY SUPPORTED MIGRATION
        │
        ├── has stable identity
        ├── has explicit classification
        ├── has semantic-preservation analysis
        ├── has version dependencies
        ├── has layer ownership
        ├── has transformation rules
        ├── has diagnostics
        ├── has provenance
        ├── has compatibility metadata
        ├── has tests
        ├── has scalability validation
        ├── has determinism validation
        ├── has safe-Rust validation
        └── has repository integration validation

---

204. Final Acceptance Rule

The migration MUST NOT be marked complete merely because:

old source → new source

succeeds.

The complete requirement is:

old source
    ↓
version identification
    ↓
migration classification
    ↓
migration
    ↓
lexer
    ↓
parser
    ↓
AST
    ↓
structural validation
    ↓
type validation
    ↓
effect validation
    ↓
resource validation
    ↓
capability validation
    ↓
contract validation
    ↓
policy validation
    ↓
provenance validation
    ↓
semantic validation
    ↓
canonical IR
    ↓
quantum::ir where applicable
    ↓
optimization
    ↓
lowering
    ↓
routing
    ↓
scheduling
    ↓
resilience
    ↓
QEC where applicable
    ↓
ZQN
    ↓
HAL
    ↓
target/runtime

The migration is complete only when the applicable path succeeds.

---

205. Permanent Architectural Invariants

The following invariants are permanent unless the language specification itself is intentionally changed.

Invariant 1 — One Language

Zamani remains one language.

Historical syntax, dialects, and migration adapters MUST NOT silently create parallel permanent languages.

Invariant 2 — One Semantic Center

All supported domains converge through the common semantic architecture.

Invariant 3 — Canonical Quantum Boundary

Quantum semantics converge through:

quantum::ir

Invariant 4 — Target Independence

Portable source describes computation and intent, not today's physical hardware.

Invariant 5 — Open-Ended Scale

Language architecture MUST NOT impose artificial finite resource ceilings.

Invariant 6 — Explicit Capability

Unavailable capabilities MUST be diagnosed rather than silently substituted with different semantics.

Invariant 7 — Explicit Resource Negotiation

Requirements, capabilities, constraints, preferences, hints, and realization remain distinct.

Invariant 8 — Explicit Effects

Effect changes are semantic changes and MUST be migrated explicitly.

Invariant 9 — Provenance

Meaningful transformations SHOULD remain traceable.

Invariant 10 — Safe Rust

Production migration implementation uses safe Rust with Rust 1.97 or later.

Invariant 11 — Determinism

Deterministic migrations remain deterministic.

Invariant 12 — No Silent Semantic Downgrade

Unsupported newer semantics MUST NOT silently disappear.

---

206. The Fundamental Migration Equation

For a semantics-preserving migration:

S_old
   │
   ▼
M
   │
   ▼
S_new

the required property is:

Semantics(S_old)
=
Semantics(S_new)

and, where canonical IR exists:

CanonicalIR(S_old)
≈
CanonicalIR(S_new)

where "≈" means semantic equivalence rather than byte-for-byte equality.

For quantum programs:

QuantumSemantics(S_old)
=
QuantumSemantics(S_new)

must hold before physical routing or decomposition is considered.

---

207. The Fundamental POCO-REAF Equation

The portability architecture is:

SOURCE
  ↓
INTENT
  ↓
SEMANTICS
  ↓
REQUIREMENTS
CAPABILITIES
RESOURCES
CONSTRAINTS
PREFERENCES
POLICIES
CONTRACTS
PROVENANCE
  ↓
CANONICAL IR
  ↓
REALIZATION

Therefore:

SOURCE PORTABILITY

does not require:

IDENTICAL PHYSICAL REALIZATION

and:

TARGET DIFFERENCE

does not automatically imply:

SOURCE MIGRATION

---

208. Final Migration Architecture

The complete production architecture is:

                         OLD PROGRAM
                              │
                              ▼
                    VERSION IDENTIFICATION
                              │
                              ▼
                    MIGRATION CLASSIFICATION
                              │
                              ▼
                     MIGRATION DEPENDENCIES
                              │
                              ▼
                      SOURCE TRANSFORMATION
                              │
                              ▼
                       CURRENT SOURCE
                              │
                              ▼
                            LEXER
                              │
                              ▼
                           PARSER
                              │
                              ▼
                             AST
                              │
                              ▼
                    STRUCTURAL VALIDATION
                              │
             ┌────────────────┼────────────────┐
             ▼                ▼                ▼
           TYPES           EFFECTS        CAPABILITIES
             │                │                │
             └────────────────┼────────────────┘
                              │
             ┌────────────────┼────────────────┐
             ▼                ▼                ▼
        RESOURCES         CONTRACTS         POLICIES
             │                │                │
             └────────────────┼────────────────┘
                              ▼
                         PROVENANCE
                              │
                              ▼
                     SEMANTIC ANALYSIS
                              │
              ┌───────────────┼───────────────┐
              ▼               ▼               ▼
         CLASSICAL         QUANTUM           HDL
              │               │               │
              │               ▼               │
              │          quantum::ir          │
              │               │               │
              └───────────────┼───────────────┘
                              ▼
                       CANONICAL IR
                              │
                              ▼
                         OPTIMIZATION
                              │
                              ▼
                          LOWERING
                              │
                 ┌────────────┼────────────┐
                 ▼            ▼            ▼
              ROUTING     SCHEDULING   RESILIENCE
                                             │
                                             ▼
                                            QEC
                                             │
                                             ▼
                                            ZQN
                                             │
                                             ▼
                                            HAL
                                             │
                              ┌──────────────┼──────────────┐
                              ▼              ▼              ▼
                             CPU            GPU            FPGA
                              │              │              │
                              ├──────────────┼──────────────┤
                              ▼              ▼              ▼
                             ASIC           QPU       DISTRIBUTED
                              │              │              │
                              └──────────────┼──────────────┘
                                             ▼
                                      FUTURE TARGETS

---

209. Final Principle

The permanent rule for Zamani compatibility is:

«A language evolution may change representation, syntax, storage, compiler architecture, or target realization, but a semantics-preserving migration MUST NOT silently change what the program means.»

And the permanent scalability rule is:

«A Zamani program expresses computation, intent, requirements, capabilities, constraints, preferences, policies, contracts, and provenance; the compiler and runtime determine an appropriate realization from the resources and capabilities actually available.»

Therefore migration MUST never transform:

symbolic requirement

into:

fixed implementation capacity

and MUST never transform:

target limitation

into:

language limitation

The final compatibility architecture is consequently:

                    SOURCE
                      │
                      ▼
                 MIGRATION
                      │
                      ▼
                  MEANING
                      │
                      ▼
             CANONICAL SEMANTICS
                      │
          ┌───────────┴───────────┐
          ▼                       ▼
     Classical                Quantum
          │                       │
          │                  quantum::ir
          │                       │
          └───────────┬───────────┘
                      ▼
                CANONICAL IR
                      │
                      ▼
              TARGET-INDEPENDENT
                 COMPILATION
                      │
          ┌───────────┼───────────┐
          ▼           ▼           ▼
       ROUTING    SCHEDULING   RESILIENCE
                                  │
                                  ▼
                                 QEC
                                  │
                                  ▼
                                 ZQN
                                  │
                                  ▼
                                 HAL
                                  │
                                  ▼
                           TARGET REALIZATION

This is the required foundation for maintaining:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever

from the smallest useful computation through arbitrarily large realizations permitted by program semantics, available resources, target capabilities, physical feasibility, and implementation capacity.

The language itself MUST NOT manufacture finite ceilings where the semantics do not require them.

The migration system therefore exists to preserve one thing above all else:

OLD REPRESENTATION
        ↓
        MIGRATE
        ↓
CURRENT REPRESENTATION
        ↓
SAME SPECIFIED MEANING
        ↓
ARBITRARY VALID REALIZATION

That is the production compatibility contract.
Zamani Provenance Semantic Specification

Path: "grammar/spec/provenance.md"
Status: Production-target normative specification
Specification role: Universal provenance semantic contract
Language: Zamani
Grammar technology: ANTLR4
Implementation baseline: Rust 1.97 or later
Rust edition: Rust 2021 or later
Safety: Safe Rust only; "unsafe" is prohibited
Portability model: Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)

---

1. Purpose

This specification defines the universal semantic model for provenance in Zamani.

Provenance answers:

- what an artifact, value, declaration, model, circuit, hardware description, result, or decision came from;
- what inputs contributed to it;
- what transformations produced it;
- what dependencies participated;
- what compiler or toolchain context was involved;
- what semantic decisions were made;
- what evidence supported those decisions;
- what verification was performed;
- which policies, capabilities, resources, and contracts participated;
- how an artifact relates to earlier or later artifacts;
- which source locations and semantic entities are associated with a provenance record;
- how provenance remains valid across compilation, lowering, optimization, routing, scheduling, execution, verification, and deployment.

Provenance is a lineage and evidence model.

It is not:

- the compiler;
- the optimizer;
- the scheduler;
- the runtime;
- a security authority;
- a resource allocator;
- a hardware inventory;
- a cryptographic algorithm;
- a target-selection algorithm;
- a quantum IR;
- a second AST;
- a replacement for contracts;
- a replacement for policies;
- a replacement for reproducibility;
- a replacement for deterministic execution.

The central invariant is:

«Provenance records what happened, what participated, what was derived, what was decided, and what evidence exists. It does not itself perform those operations.»

---

2. Normative Language

The terms:

- MUST
- MUST NOT
- REQUIRED
- SHALL
- SHALL NOT
- SHOULD
- SHOULD NOT
- MAY

are normative.

A conforming implementation MUST obey all requirements marked "MUST", "MUST NOT", "REQUIRED", "SHALL", or "SHALL NOT".

A "SHOULD" requirement may be relaxed only when the implementation has a documented reason and doing so does not violate another normative requirement.

---

3. Authority

The repository has deliberately separated language specification, concrete grammar, semantic contracts, implementation, and realization.

The authority relationship for provenance is:

grammar/specification/
        │
        │ language-level authority
        ▼
grammar/spec/provenance.md
        │
        │ provenance semantic contract
        ▼
grammar/compile/provenance.g4
grammar/ai/provenance.g4
grammar/security/provenance.g4
grammar/data/provenance.g4
grammar/expressions/provenance.g4
        │
        ▼
domain-neutral AST
        │
        ▼
semantic provenance model
        │
        ├── source lineage
        ├── dependency lineage
        ├── transformation lineage
        ├── evidence
        ├── decisions
        ├── verification
        ├── policy participation
        ├── resource/capability context
        └── execution/deployment lineage
        │
        ▼
canonical semantic representation
        │
        ├── Classical IR
        └── quantum::ir
        │
        ▼
optimization / lowering / routing / scheduling
        │
        ▼
ZQN
        │
        ▼
HAL
        │
        ▼
target realization

"grammar/spec/provenance.md" is the normative semantic contract for provenance.

It does not supersede:

- "grammar/specification/" as the repository's language-specification authority;
- "grammar/Zamani.g4" as the combined ANTLR root;
- "grammar/antlr/ZamaniLexer.g4" as lexical authority;
- "grammar/antlr/ZamaniParser.g4" as parser composition authority;
- domain-specific grammar ownership;
- AST implementation ownership;
- canonical semantic-model ownership;
- canonical IR ownership;
- runtime and HAL ownership.

Instead, it defines what provenance means when those systems use it.

---

4. Ownership

4.1 This file owns

This specification owns the semantic meaning of:

1. provenance;
2. provenance subjects;
3. provenance entities;
4. provenance activities;
5. provenance agents;
6. provenance relationships;
7. lineage;
8. derivation;
9. transformation records;
10. generation records;
11. input/output relationships;
12. dependency lineage;
13. evidence;
14. claims;
15. decisions;
16. verification records;
17. provenance status;
18. provenance confidence metadata;
19. provenance scope;
20. provenance context;
21. provenance identity;
22. provenance versioning;
23. provenance inheritance;
24. provenance composition;
25. provenance merging;
26. provenance projection;
27. provenance redaction semantics;
28. provenance preservation across compiler stages;
29. provenance across domain boundaries;
30. provenance integrity semantics;
31. provenance reproducibility relationships;
32. provenance determinism relationships;
33. provenance diagnostics;
34. provenance compatibility;
35. provenance scalability requirements.

---

4.2 This file does not own

This file does not own:

- token spelling;
- lexer rules;
- parser rules;
- general expression syntax;
- identifiers;
- qualified-name syntax;
- declarations;
- type syntax;
- contracts syntax;
- policy syntax;
- resource syntax;
- effect syntax;
- quantum operation syntax;
- HDL syntax;
- hardware syntax;
- AI operation syntax;
- data-query syntax;
- security syntax;
- cryptographic algorithms;
- hash implementation;
- signing implementation;
- attestation implementation;
- compiler implementation;
- runtime implementation;
- target discovery;
- resource discovery;
- capability discovery;
- physical placement;
- quantum routing;
- scheduling;
- QEC;
- ZQN;
- HAL;
- artifact storage;
- filesystem access;
- network access.

Those systems produce or consume provenance.

They do not redefine its universal semantic meaning.

---

5. Existing Repository Integration

The current repository already contains provenance concepts in several domains.

They MUST remain coordinated through this semantic contract.

Existing file| Responsibility
"grammar/compile/provenance.g4"| compilation-provenance syntax
"grammar/ai/provenance.g4"| AI/model/knowledge provenance syntax
"grammar/security/provenance.g4"| security/trust provenance syntax
"grammar/data/provenance.g4"| data provenance syntax
"grammar/expressions/provenance.g4"| expression-level provenance syntax
"grammar/compile/reproducibility.g4"| reproducibility syntax/intent
"grammar/compile/deterministic-builds.g4"| deterministic-build syntax/intent
"grammar/validation/evidence.g4"| evidence validation syntax
"grammar/validation/contracts.g4"| contract semantics/syntax
"grammar/resources/*.g4"| resources and capabilities
"grammar/effects/*"| effect semantics
"grammar/security/policies.g4"| security policy syntax
"grammar/spec/determinism.md"| determinism contract
"grammar/spec/portability.md"| portability contract
"grammar/spec/resources.md"| resource semantic contract
"grammar/spec/effects.md"| effect semantic contract
"grammar/spec/quantum.md"| quantum semantic contract
"grammar/spec/hdl.md"| HDL semantic contract
"grammar/spec/semantics.md"| general semantic contract
"grammar/spec/source-spans.md"| source-location contract
"grammar/spec/source-map.md"| source mapping contract
"grammar/specification/poco-reaf.md"| POCO-REAF language-level contract
"grammar/specification/grammar-authority.md"| grammar authority contract

These files MUST NOT create independent incompatible provenance models.

They contribute domain-specific syntax and semantics to the universal provenance model defined here.

---

6. Core Definition

A provenance record describes relationships between semantic objects and activities.

The conceptual model is:

Entity
  │
  ├── was_generated_by ──► Activity
  │                           │
  │                           ├── used ──► Entity
  │                           │
  │                           ├── associated_with ──► Agent
  │                           │
  │                           ├── governed_by ──► Policy
  │                           │
  │                           ├── required ──► Capability/Resource
  │                           │
  │                           └── produced ──► Entity
  │
  └── was_derived_from ──► Entity

The model is intentionally graph-oriented.

A provenance graph MAY contain:

- one node;
- many nodes;
- one edge;
- many edges;
- branching lineage;
- converging lineage;
- repeated derivations;
- parallel activities;
- nested activities;
- long chains;
- distributed subgraphs;
- cyclic references where explicitly permitted by the provenance implementation model.

The language does not impose a finite graph size.

---

7. Provenance Subject

A provenance subject is the semantic thing whose lineage is being described.

A subject MAY be:

- source code;
- source unit;
- module;
- declaration;
- expression;
- type;
- value;
- dataset;
- model;
- tensor;
- classical IR entity;
- quantum semantic entity;
- "quantum::ir" entity;
- HDL representation;
- hardware description;
- artifact;
- executable;
- package;
- configuration;
- execution result;
- measurement result;
- decision;
- evidence;
- verification result;
- deployment;
- resource realization;
- generated representation;
- compiler result;
- diagnostic;
- externally supplied semantic object.

The provenance system MUST NOT require a closed list of subject kinds.

Future semantic domains MUST be representable without modifying the fundamental provenance model.

---

8. Entity

An entity is a provenance-addressable semantic object.

Conceptually:

Entity {
    identity
    kind
    scope
    version
    source
    properties
}

The exact implementation representation belongs to the AST/semantic layer.

An entity identity MUST be distinguishable from:

- physical address;
- memory address;
- device identifier;
- filesystem location;
- network endpoint.

A physical identity MAY be included as metadata when a target-specific program explicitly requires it.

It MUST NOT become the default meaning of a portable provenance identity.

---

9. Activity

An activity represents an operation, transformation, process, or semantic stage that contributes to the existence or state of an entity.

Examples:

parse
type_check
effect_check
resource_analysis
capability_resolution
optimization
lowering
quantum_lowering
quantum_routing
scheduling
verification
code_generation
packaging
deployment
execution
measurement
training
inference
learning
adaptation
simulation
synthesis

The list is open-ended.

Activities MUST NOT be tied to a finite catalogue of compiler passes.

---

10. Agent

An agent is an actor responsible for, participating in, or being associated with an activity.

An agent MAY represent:

- a human;
- a compiler;
- a compiler component;
- a tool;
- a runtime;
- a verifier;
- a build system;
- an execution environment;
- a service;
- a device;
- a distributed participant;
- a model;
- an automated process.

Agent identity MUST be semantically distinct from activity identity.

For example:

Agent:
    Zamani compiler

Activity:
    quantum lowering

are different concepts.

---

11. Provenance Relationships

The universal provenance model MUST support at least the following semantic relationships:

was_derived_from
was_generated_by
was_transformed_by
used
produced
associated_with
verified_by
supported_by
contradicted_by
governed_by
required
constrained_by
preferred_by
informed_by
depends_on
contains
part_of
specialized_from
lowered_from
raised_from
mapped_from
executed_as
realized_as

The relationship vocabulary is extensible.

A new relationship MUST NOT require a new core keyword unless repeated use establishes that it is genuinely universal language semantics.

---

12. Lineage

Lineage is the transitive relationship between an entity and its semantic ancestors or descendants.

For example:

source
  ↓
AST
  ↓
validated semantic model
  ↓
Classical IR
  ↓
optimized Classical IR
  ↓
target representation
  ↓
artifact
  ↓
execution result

For quantum computation:

source
  ↓
AST
  ↓
quantum semantic model
  ↓
quantum::ir
  ↓
optimized quantum::ir
  ↓
routed representation
  ↓
scheduled representation
  ↓
ZQN
  ↓
HAL realization
  ↓
execution result

Every transformation that materially changes semantic representation SHOULD preserve a provenance relationship to its source.

---

13. Provenance Is Not a Second IR

Provenance MUST NOT become a second executable IR.

It does not replace:

- Classical IR;
- "quantum::ir";
- HDL representations;
- target representations;
- ZQN;
- HAL representations.

The relationship is:

semantic computation
        │
        ├──────────────► canonical IR
        │
        └──────────────► provenance metadata

Provenance describes lineage of the computation.

It does not become the computation.

---

14. Provenance Across Canonical IR

When a semantic entity is lowered into canonical IR, the relationship MUST remain recoverable.

For classical computation:

AST entity
    ↓
semantic entity
    ↓
Classical IR entity

For quantum computation:

AST entity
    ↓
quantum semantic entity
    ↓
quantum::ir entity

A provenance implementation MAY store:

- source references;
- semantic entity identifiers;
- IR entity identifiers;
- transformation identifiers.

It MUST NOT require the provenance system to understand the internal instruction semantics of every future IR.

---

15. Quantum Provenance

Quantum provenance MUST integrate with "quantum::ir".

A quantum provenance chain MAY describe:

quantum source
    ↓
quantum AST
    ↓
semantic validation
    ↓
quantum::ir
    ↓
decomposition
    ↓
optimization
    ↓
routing
    ↓
scheduling
    ↓
resilience / QEC
    ↓
ZQN
    ↓
HAL
    ↓
target realization
    ↓
measurement/result

The provenance model MUST NOT:

- enumerate quantum gates;
- define physical qubit identifiers;
- define QPU topology;
- define calibration semantics;
- define QEC algorithms;
- create a second quantum IR.

Those remain owned by the quantum semantic and execution systems.

---

16. Classical Provenance

Classical provenance MUST support:

- source-to-AST lineage;
- type-analysis lineage;
- optimization lineage;
- dataflow lineage;
- compilation lineage;
- generated artifact lineage;
- execution lineage.

It MUST support arbitrary computation sizes without embedding:

- fixed register widths;
- fixed processor counts;
- fixed thread counts;
- fixed memory sizes.

---

17. HDL and Hardware Provenance

HDL provenance MUST be able to represent:

HDL source
    ↓
elaborated design
    ↓
verified design
    ↓
synthesized representation
    ↓
optimized representation
    ↓
placed/routed representation
    ↓
generated hardware artifact

Hardware provenance MAY include target-specific identities when explicitly required.

Portable provenance MUST NOT require:

- fixed FPGA dimensions;
- fixed ASIC resources;
- fixed bus widths;
- fixed register counts;
- fixed device counts.

---

18. AI and Learned-Model Provenance

AI-related provenance MUST support:

- dataset lineage;
- model lineage;
- parameter lineage;
- training lineage;
- inference lineage;
- reasoning lineage;
- evidence lineage;
- decision lineage;
- adaptation lineage;
- evaluation lineage;
- deployment lineage.

A model-derived decision MUST remain distinguishable from its supporting evidence.

A generated assertion MUST NOT automatically become independently verified evidence merely because a model produced it.

---

19. Evidence

Evidence is information that supports, qualifies, contradicts, or verifies a claim, decision, or semantic assertion.

Conceptually:

Evidence {
    identity
    source
    claim
    relation
    status
    provenance
    properties
}

Evidence MAY originate from:

- source code;
- compiler analysis;
- static verification;
- runtime observation;
- measurement;
- external input;
- test results;
- formal proof;
- human review;
- independent tool;
- simulation;
- hardware observation;
- another provenance graph.

Evidence MUST retain its origin.

---

20. Evidence Is Not Assertion

The following distinction is mandatory:

assertion
≠
evidence

A statement made by a program, model, compiler, or agent does not automatically become independently verified evidence.

For example:

claim: "execution succeeded"

is not equivalent to:

observed_execution_result: success

unless an observation or verification activity supports the latter.

This distinction is essential for trustworthy reasoning, compiler diagnostics, scientific computation, and reproducibility.

---

21. Claims

A claim is a proposition recorded in provenance.

A claim MAY be:

- supported;
- unsupported;
- contradicted;
- unresolved;
- verified;
- rejected;
- superseded.

Provenance MUST preserve these distinctions.

It MUST NOT silently collapse:

unknown

into:

false

and MUST NOT silently promote:

asserted

into:

verified

---

22. Decisions

A decision records a choice made by a semantic, compilation, deployment, execution, security, or other process.

A decision SHOULD preserve:

decision identity
decision maker/activity
inputs
alternatives considered
selected outcome
constraints
requirements
preferences
evidence
reason
provenance

The exact representation of alternatives is implementation-dependent.

A decision MUST remain distinguishable from the evidence used to make it.

---

23. Decision Provenance

For important decisions, the provenance graph SHOULD make the following relationship available:

decision
   │
   ├── based_on ──► evidence
   │
   ├── constrained_by ──► constraints
   │
   ├── required_by ──► requirements
   │
   ├── governed_by ──► policies
   │
   ├── informed_by ──► observations
   │
   └── produced ──► resulting entity

This supports explanation without requiring exposure of private internal reasoning.

The provenance model records externally meaningful lineage and evidence.

It does not require recording hidden chain-of-thought.

---

24. Explanation

Explainability MAY consume provenance.

A downstream explanation system can use:

source
→ transformation
→ evidence
→ decision
→ result

to explain:

- why an optimization was selected;
- why a target was rejected;
- why a resource was required;
- why a quantum route was selected;
- why a verification result was accepted;
- why a model decision was produced;
- why an adaptation occurred.

Provenance itself does not implement an explanation engine.

---

25. Verification

Verification is an activity that establishes a stated verification result according to a defined method.

A verification record SHOULD identify:

- subject;
- verifier/activity;
- verification method;
- inputs;
- result;
- evidence;
- provenance;
- applicable specification/version;
- scope.

Verification status MUST NOT be inferred solely from the existence of a provenance record.

---

26. Verification Levels

The semantic model SHOULD distinguish at least:

unverified
observed
checked
validated
verified
independently_verified

Implementations MAY define additional statuses.

Statuses MUST have documented semantics.

In particular:

generated

MUST NOT mean:

verified

and:

self_reported

MUST NOT automatically mean:

independently_verified

---

27. Confidence

Provenance MAY carry confidence metadata.

Confidence is not equivalent to truth.

A confidence value MUST NOT silently convert an uncertain claim into a guaranteed fact.

For example:

claim
confidence
evidence
verification

are separate semantic properties.

Probability and uncertainty semantics remain owned by the relevant type/AI/quantum/data specifications.

Provenance merely records their participation.

---

28. Provenance and Uncertainty

When a computation uses uncertain information, provenance SHOULD preserve:

- uncertainty source;
- uncertainty representation;
- relevant assumptions;
- evidence;
- transformation;
- resulting uncertainty metadata.

For example:

uncertain input
    ↓
inference
    ↓
probabilistic result

must not become:

uncertain input
    ↓
certain result

without a semantic operation that justifies the change.

---

29. Resource Provenance

Provenance MAY record resource information associated with a computation.

For example:

required:
    capability("quantum.measurement")

required:
    memory >= required_memory

A provenance record MAY state that these requirements participated in target realization.

It MUST NOT redefine resource semantics.

Resource meaning remains owned by:

grammar/spec/resources.md

and the corresponding resource grammar and semantic implementation.

---

30. Capability Provenance

A provenance record MAY state:

capability required
capability available
capability selected
capability unavailable
capability negotiated

These are distinct states.

The provenance record MUST NOT imply that a capability exists merely because a program requested it.

The resource/capability resolver determines actual availability.

---

31. Effect Provenance

Provenance MAY record effects associated with an activity.

Examples include:

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

Effect semantics remain owned by the effect system.

Provenance records their participation.

It does not grant effects.

---

32. Policy Provenance

Provenance MAY record policies that governed an activity.

Examples:

portability policy
security policy
resource policy
execution policy
adaptation policy
deployment policy
simulation policy

A provenance record saying:

governed_by policy::X

does not itself enforce policy "X".

Policy enforcement remains owned by the policy/security/execution systems.

---

33. Contract Provenance

Provenance MAY record:

- requirements;
- preconditions;
- postconditions;
- invariants;
- assumptions;
- guarantees;
- properties;
- proofs;
- verification results.

The contract system remains authoritative for contract semantics.

Provenance records:

which contract participated
which verification activity evaluated it
what result was obtained
what evidence supported that result

---

34. Source Provenance

Every provenance-aware compiler stage SHOULD preserve a relationship to source locations when source locations are semantically relevant.

The source-location model is owned by:

grammar/spec/source-spans.md
grammar/spec/source-map.md

A provenance record MAY reference:

source file
source unit
module
declaration
expression
token range
source span
AST node
semantic entity

Provenance MUST NOT invent a second source-location model.

---

35. Source Span Integration

Where an AST node or semantic entity has a source span, provenance SHOULD be able to reference that span.

For example:

provenance entity
    ↓
source entity
    ↓
source span

The provenance model MUST NOT require source text to be copied into every provenance record.

References SHOULD be preferred over unnecessary duplication.

---

36. Transformation Provenance

A transformation is a semantic activity that changes or derives a representation.

Examples:

parse
elaborate
type_check
normalize
specialize
optimize
lower
decompose
route
schedule
synthesize
generate
package
deploy

A transformation record SHOULD identify:

input entity
activity
agent
output entity
configuration/context
relevant policy
relevant requirements
relevant evidence

---

37. Semantic Preservation

A transformation MUST NOT be described as semantically equivalent merely because it has provenance.

Semantic preservation belongs to the relevant semantic verification system.

Provenance may record:

transformed_by
verified_by

but the provenance record itself does not prove equivalence.

---

38. Derivation

Derivation expresses that one entity originated from another entity through some semantic process.

Conceptually:

derived_entity
    was_derived_from
source_entity

Derivation MAY be:

- direct;
- transitive;
- conditional;
- parameterized;
- composite.

Implementations MUST preserve enough information to distinguish materially different derivation paths when the distinction matters.

---

39. Generation

Generation describes creation of a new representation or artifact by an activity.

Examples:

source → generated AST
semantic model → generated IR
IR → generated artifact
HDL → generated netlist
quantum::ir → generated ZQN
program → generated executable

Generation does not imply correctness.

Correctness requires the appropriate verification or validation evidence.

---

40. Dependency Provenance

Dependencies MUST be represented independently from direct derivation where necessary.

For example:

program
  ├── derived_from source
  └── depends_on library

A dependency is not necessarily the direct source of an artifact.

Dependency provenance MAY include:

- version;
- identity;
- source;
- selected variant;
- capability;
- compatibility information;
- integrity metadata.

---

41. Version Provenance

Provenance SHOULD record the versions relevant to reproducibility and interpretation.

Possible versioned entities include:

language
grammar
specification
AST schema
semantic schema
IR schema
quantum::ir schema
dialect
compiler
toolchain
dependency
library
model
dataset
target profile
deployment profile
provenance schema

Version information MUST be semantic metadata, not an arbitrary string with undocumented meaning.

---

42. Schema Provenance

Every serialized provenance representation MUST identify its schema/version when serialization is used.

A schema version MUST NOT silently change the meaning of existing records.

Schema evolution MUST follow:

compatible extension
    ↓
versioned migration
    ↓
deprecation
    ↓
removal only under compatibility policy

---

43. Provenance Identity

A provenance identity SHOULD be stable within its declared scope.

An identity MAY be:

- symbolic;
- content-derived;
- implementation-generated;
- externally assigned;
- cryptographically bound.

The semantic model does not mandate one identity mechanism for every use case.

However, an implementation MUST clearly distinguish:

identity

from:

display name

and:

human-readable label

---

44. Content Identity

When provenance needs to establish that two entities refer to identical content, an implementation MAY use a content digest or another integrity mechanism.

The semantic provenance model MUST NOT mandate a particular cryptographic algorithm.

Cryptographic algorithm selection belongs to:

grammar/security/

and the relevant cryptographic implementation.

A digest is evidence of content identity under its defined algorithm.

It is not automatically evidence of:

- authorship;
- correctness;
- legality;
- semantic equivalence;
- execution success.

---

45. Integrity

Provenance integrity concerns whether a provenance record or relationship has been altered relative to the integrity mechanism used.

Integrity MUST be distinguishable from authenticity.

integrity
≠
authenticity

Authenticity MUST be established through the appropriate identity/signature/attestation mechanisms.

---

46. Authenticity

A provenance record MAY be associated with an authenticated agent.

Authentication belongs to the security subsystem.

Provenance records the relationship:

activity
    associated_with
authenticated_agent

It does not implement authentication.

---

47. Trust

Trust information MAY be associated with provenance.

Trust MUST NOT be confused with lineage.

For example:

source lineage

answers:

«Where did this come from?»

while:

trust assessment

answers:

«What basis exists for relying on it?»

The two concepts MUST remain distinct.

---

48. Provenance and Security

Security provenance may record:

- authorization decisions;
- policy evaluations;
- identity;
- signatures;
- attestations;
- trust decisions;
- sandbox transitions;
- security events.

Security semantics remain owned by:

grammar/security/
grammar/spec/security.md

A provenance record MUST NOT grant authority.

---

49. Provenance and Sandboxing

A sandboxed activity MAY produce provenance.

The provenance system MUST respect the sandbox's security policy.

Recording provenance MUST NOT automatically grant:

- filesystem access;
- network access;
- device access;
- secret access;
- native execution;
- foreign-function access.

---

50. Provenance and Reflection

Reflection and metaprogramming MAY generate provenance describing:

input syntax
    ↓
reflection activity
    ↓
generated representation

Generated code MUST remain distinguishable from original source.

The provenance system SHOULD preserve:

generated_by
derived_from

relationships.

---

51. Provenance and Adaptation

Controlled adaptation MUST record provenance when adaptation materially changes program/model/execution state.

A useful lineage is:

initial state
    ↓
observation
    ↓
evidence
    ↓
policy evaluation
    ↓
adaptation decision
    ↓
adapted state

Adaptation provenance MUST NOT imply unrestricted self-modifying code.

Adaptation authority remains governed by:

- effects;
- capabilities;
- resources;
- authorization;
- policies;
- contracts;
- runtime semantics.

---

52. Provenance and Learning

Learning activities SHOULD record:

training/input data
model
parameters or parameter-set identity
objective
algorithm/profile
resources
capabilities
policy
activity
result
evaluation

The provenance model MUST NOT require every possible learning algorithm to become a language keyword.

Algorithm identity remains extensible.

---

53. Provenance and Reasoning

Reasoning activities MAY record:

premises
facts
evidence
rules
inference activity
conclusion
confidence/uncertainty
verification
decision

The provenance model MUST preserve the distinction between:

premise
evidence
inference
conclusion
verification

These are not interchangeable.

---

54. Provenance and Knowledge

Knowledge systems MAY record:

assertion
source
derivation
evidence
confidence
verification
retraction
supersession

A retracted assertion MUST NOT erase historical provenance.

Instead, provenance SHOULD preserve:

asserted
    ↓
retracted

with the relevant activity and reason.

---

55. Retraction

Retraction is a semantic event.

Retraction SHOULD record:

- subject;
- prior assertion;
- retraction activity;
- reason;
- authority;
- evidence;
- resulting status.

Retraction MUST NOT destroy the historical record merely by deleting the assertion from an active knowledge view.

Active state and historical provenance are distinct.

---

56. Provenance and Simulation

Simulation activities MAY record:

simulated input
simulation configuration
model
simulator
assumptions
resource model
result
verification

Simulation results MUST remain distinguishable from physical observations unless a verification process establishes an appropriate relationship.

For example:

simulation_result
≠
hardware_measurement

by default.

---

57. Provenance and Execution

Execution provenance MAY record:

program
compiled representation
target realization
runtime
configuration
inputs
outputs
observations
errors
recovery
resource observations
measurements

Execution provenance MUST NOT change source semantics.

---

58. Runtime Events

Runtime provenance may be event-oriented.

An event MAY record:

event identity
timestamp or logical ordering information
activity
entity
agent
status
input references
output references
properties

Wall-clock timestamps are optional semantic metadata.

A timestamp MUST NOT be required for deterministic source semantics unless the program explicitly uses time.

---

59. Logical Ordering

Distributed provenance MAY require logical ordering.

Implementations MAY use:

- sequence numbers;
- causal ordering;
- vector-like clocks;
- dependency ordering;
- activity identifiers.

The core provenance model MUST remain independent of one particular distributed-ordering algorithm.

---

60. Distributed Provenance

Distributed activities MAY generate partial provenance graphs.

A distributed provenance graph MUST support:

node A
   ↓
message
   ↓
node B

and may represent:

parallel activities
causal relationships
partial failure
retry
recovery
replication
aggregation

A distributed system MUST NOT require a single physical global clock for semantic provenance.

---

61. Concurrency

Concurrent activities MUST remain distinguishable.

If two activities execute independently:

A ──► X
B ──► Y

provenance MUST NOT invent a false ordering:

A before B

unless that ordering is actually established.

This is important for:

- actors;
- tasks;
- parallel computation;
- distributed computation;
- GPU work;
- quantum/classical hybrid execution;
- hardware pipelines.

---

62. Retry and Recovery

When an activity is retried, provenance SHOULD represent the attempts separately.

Conceptually:

attempt 1
    ↓
failure

attempt 2
    ↓
success

The system SHOULD preserve both attempts.

A retry MUST NOT overwrite the historical existence of the failed attempt.

This integrates with the existing resilience outcomes:

ACCEPT
DEGRADED_ACCEPT
RETRY
RECOVER
ESCALATE
REJECT

and resilience states:

Unknown
Healthy
Degraded
Unstable
Unavailable
Recovering
Quarantined
Retired

Provenance records these states when relevant; resilience semantics remain owned by the execution/resilience subsystem.

---

63. Partial Failure

Distributed or heterogeneous execution may produce partial success.

Provenance MUST be able to distinguish:

complete success
partial success
degraded success
failure
unknown

without forcing all sub-results into one global status.

---

64. Provenance Composition

When two independently produced provenance graphs are combined, their identities and relationships MUST remain distinguishable.

Composition MUST NOT silently:

- merge unrelated entities;
- collapse distinct activities;
- erase conflicting evidence;
- overwrite provenance status;
- treat identical names as identical identities.

Identity resolution must be explicit.

---

65. Provenance Merge

A merge operation MAY combine provenance graphs.

A conforming merge MUST preserve:

- entity identity;
- activity identity;
- agent identity;
- relationship identity;
- evidence;
- conflicts;
- verification status;
- provenance versions.

When conflicts exist, they MUST remain observable.

For example:

source A says X
source B says not-X

must not silently become:

X

or:

not-X

without a separate semantic resolution activity.

---

66. Provenance Projection

A provenance consumer MAY project a large graph into a smaller view.

Examples:

full compiler lineage
        ↓
user-facing explanation

or:

full execution lineage
        ↓
audit summary

Projection MUST NOT be presented as the complete original provenance graph unless it actually is complete.

A projection SHOULD identify its source graph and projection activity.

---

67. Provenance Redaction

Security, privacy, or policy may require provenance redaction.

Redaction MUST NOT be represented as if the removed information never existed when the distinction matters.

Where appropriate, the system SHOULD record:

redacted
reason
policy
authority
scope

Redaction MUST respect:

grammar/spec/security.md

and relevant privacy policies.

---

68. Provenance Retention

Retention is a lifecycle concern.

The semantic model does not require infinite physical storage.

However:

retention

MUST NOT be confused with:

deletion of semantic history

where historical preservation is required by the applicable policy.

---

69. Provenance Scope

Every provenance record MUST have an interpretable scope.

Possible scopes include:

source
module
program
compilation
artifact
execution
deployment
resource
activity
entity
dataset
model
experiment
system

Scope MAY be nested.

The scope MUST be explicit enough for consumers to determine which entities and activities the record describes.

---

70. Provenance Context

A provenance context describes the semantic environment necessary to interpret a provenance record.

It MAY include:

language version
grammar version
specification version
compiler version
toolchain version
dialect versions
dependency versions
IR version
target profile
execution profile
policy profile
resource profile
provenance schema version

Context SHOULD use references rather than duplicating large metadata structures.

---

71. Provenance Environment

Environment information MUST be distinguished between:

declared environment
observed environment
realized environment

For example:

declared target: GPU-capable

does not mean:

observed device: specific GPU

and neither necessarily means:

realized execution: that GPU

These are separate provenance facts.

---

72. Target Provenance

Target provenance MAY record:

target intent
target profile
target capability set
target realization
target-specific artifact

The provenance system MUST NOT convert target metadata into universal source requirements.

A portable program remains portable unless the source explicitly introduces a target constraint.

---

73. Resource-Scale Provenance

Provenance MUST remain valid across arbitrary resource scales.

The model MUST NOT define:

MAX_PROVENANCE_NODES
MAX_PROVENANCE_EDGES
MAX_COMPILATION_STAGES
MAX_INPUTS
MAX_OUTPUTS
MAX_DECISIONS
MAX_EVIDENCE
MAX_LINEAGE_DEPTH
MAX_ENTITIES
MAX_ACTIVITIES

as language constants.

An implementation may have finite memory and processing capacity.

Such limits are implementation/resource limits, not language semantics.

---

74. "Unbounded" Semantics

For this specification:

«Unbounded means that the Zamani language does not define an arbitrary finite universal ceiling.»

It does not mean that a physical implementation has infinite storage, infinite memory, infinite time, or infinite computational power.

Therefore:

language capacity

and:

implementation capacity

MUST remain distinct.

A provenance graph may be as large as the implementation and available resources permit.

---

75. Streaming Provenance

For extremely large computations, implementations SHOULD support streaming or incremental provenance where appropriate.

The semantic model MUST allow provenance to be produced incrementally without changing its meaning.

Examples:

event 1
event 2
event 3
...
event N

do not require the complete graph to be resident in memory simultaneously.

Streaming storage is an implementation concern.

---

76. Incremental Provenance

Compilation and execution may be incremental.

Provenance SHOULD support:

previous state
    ↓
incremental activity
    ↓
new state

An incremental compilation MUST NOT falsely claim that the entire program was rebuilt if only a subset was transformed.

---

77. Caching Provenance

When cached artifacts are reused, provenance SHOULD record:

cache lookup
cache key/context
cache hit
cached artifact
validation/integrity result
reuse decision

A cache hit MUST NOT imply that the cached result is valid without the required compatibility and integrity checks.

This integrates with:

grammar/compile/caching.g4

---

78. Reproducibility

Provenance is a necessary input to reproducibility but does not by itself guarantee reproducibility.

Reproducibility may require:

source
dependencies
toolchain
compiler
configuration
policies
randomness controls
environment
target profile
resource context
inputs
transformations

The reproducibility contract remains owned by:

grammar/compile/reproducibility.g4
grammar/spec/determinism.md

Provenance records the relevant lineage.

---

79. Determinism

A deterministic activity SHOULD have provenance sufficient to identify the semantic inputs that determine its result.

However, provenance MUST NOT manufacture determinism.

For example:

random execution

does not become deterministic merely because its random seed was recorded.

Determinism semantics remain owned by the determinism specification.

---

80. Randomness Provenance

When randomness materially affects a result, provenance MAY record:

randomness source
seed identity
randomness policy
randomness activity

Sensitive random state MUST NOT be exposed merely because provenance exists.

Security policy governs whether such information may be recorded.

---

81. Compiler Provenance

Compiler provenance SHOULD support:

source
lexer/parser configuration
AST
semantic validation
compiler version
toolchain
optimization configuration
resource requirements
capability resolution
policy
lowering
IR
backend
artifact
verification

Compiler provenance MUST preserve semantic lineage across compiler phases.

---

82. Optimization Provenance

An optimizer MAY record:

input representation
optimization activity
optimization profile
decision
evidence
output representation
verification

An optimization record MUST NOT imply correctness merely because an optimization occurred.

Correctness is established by semantic preservation/verification.

---

83. Lowering Provenance

Lowering SHOULD record:

source semantic entity
source IR
lowering activity
target semantic domain
result IR

For quantum lowering:

source quantum semantic entity
    ↓
quantum::ir

must remain distinguishable from later:

quantum::ir
    ↓
ZQN

---

84. Routing Provenance

Routing MAY record:

input representation
topology information
routing activity
routing decision
constraints
result
verification

Routing provenance MUST NOT make physical topology part of portable source semantics.

---

85. Scheduling Provenance

Scheduling MAY record:

input
scheduler
constraints
resources
decision
schedule
verification

Scheduling decisions are realization information.

They MUST NOT silently redefine source-level semantics.

---

86. Hardware Realization Provenance

When a portable program is realized on physical hardware, provenance MAY record:

abstract resource
    ↓
target resource
    ↓
physical resource

For example:

logical qubit
    ↓
physical qubit

or:

abstract accelerator
    ↓
realized accelerator

Physical identity MUST remain target/deployment metadata unless explicitly part of source semantics.

---

87. Multi-Level Provenance

The provenance system SHOULD support multiple levels:

source-level
semantic-level
IR-level
backend-level
artifact-level
deployment-level
runtime-level
measurement-level

Each level MAY provide more detail than the preceding level.

A lower level MUST NOT redefine the meaning of a higher level.

---

88. Provenance Across Domains

The same provenance model MUST work for:

- classical computation;
- quantum computation;
- HDL;
- hardware;
- AI;
- data;
- tensors;
- distributed computation;
- networking;
- cryptography;
- simulation;
- interoperability;
- metaprogramming;
- hybrid computation;
- future computational domains.

Domain-specific provenance is an extension of the universal model.

It is not a replacement.

---

89. Hybrid Provenance

Hybrid programs may contain:

classical
quantum
AI
HDL
hardware
distributed
data

activities in one semantic computation.

Provenance MUST be able to cross those boundaries.

Example:

classical preprocessing
        ↓
tensor transformation
        ↓
quantum computation
        ↓
measurement
        ↓
classical decision
        ↓
hardware control

must remain representable as one connected provenance graph.

---

90. Interoperability Provenance

FFI/ABI operations SHOULD record:

Zamani entity
    ↓
foreign boundary
    ↓
foreign entity
    ↓
result

The provenance record MAY identify:

- ABI;
- calling convention;
- external library;
- foreign type;
- external artifact.

FFI authority remains owned by:

grammar/interoperability/

and the corresponding semantic implementation.

---

91. Data Provenance

Data provenance SHOULD record:

input dataset
    ↓
transformation
    ↓
derived dataset

It MAY include:

- filtering;
- aggregation;
- joins;
- transformations;
- schema changes;
- serialization;
- deserialization;
- query execution.

Data syntax remains owned by the data subsystem.

---

92. Model and Dataset Lineage

For learned systems:

dataset
    ↓
preprocessing
    ↓
training
    ↓
model
    ↓
fine-tuning/adaptation
    ↓
deployed model
    ↓
inference

MUST remain representable.

A model MUST NOT be described as trained on a dataset merely because both appear in the same provenance graph; the graph MUST contain an appropriate training relationship.

---

93. Provenance of Assertions

Assertions from reasoning or knowledge systems SHOULD preserve:

assertion
source
activity
evidence
confidence
status

An assertion may later be:

verified
retracted
superseded
contradicted

without destroying its historical provenance.

---

94. Provenance of Adaptation

Adaptation SHOULD preserve:

initial state
observation
trigger
policy
authorization
decision
adaptation activity
new state
verification

This provides an auditable relationship between adaptation and the information that caused it.

---

95. Provenance of Policy Decisions

Policy evaluation MAY produce provenance:

input
    ↓
policy
    ↓
evaluation
    ↓
decision

The policy engine remains responsible for the actual decision.

Provenance records the decision lineage.

---

96. Provenance of Resource Negotiation

Resource negotiation MAY produce:

requirements
    ↓
available capabilities/resources
    ↓
candidate realizations
    ↓
selection/rejection
    ↓
execution plan

This is especially important for POCO-REAF.

The provenance record SHOULD distinguish:

program requirement

from:

compiler decision

and:

actual realization

---

97. Failed Negotiation

If no realization satisfies mandatory requirements, provenance SHOULD record:

requirements
candidate context
missing capability/resource
evaluation
failure

The failure MUST NOT be converted into a successful realization merely to preserve portability.

A portable source program can remain unchanged while a particular target is infeasible.

---

98. Provenance and POCO-REAF

POCO-REAF requires stable source semantics across different realization environments.

Provenance supports this by recording:

same source semantics
        ↓
different compilation decisions
        ↓
different target realizations

For example:

same Zamani program
    ├── tiny embedded realization
    ├── CPU realization
    ├── GPU realization
    ├── FPGA realization
    ├── accelerator realization
    ├── QPU realization
    ├── simulator realization
    ├── HPC realization
    └── distributed realization

The provenance graphs may differ because realization differs.

The source semantic identity remains the same.

---

99. Portability Invariant

Provenance MUST NOT turn a realization-specific fact into a source-level requirement.

For example:

realized_on GPU-X

does not imply:

requires GPU-X

unless the source program explicitly declared that requirement.

---

100. Semantic Identity Across Targets

Two executions of the same source program may have different:

- target;
- compiler;
- scheduling;
- resource allocation;
- physical placement;
- runtime state.

Provenance SHOULD make those differences explicit.

A provenance consumer MUST NOT assume that different target realization means different source semantics.

---

101. Semantic Equivalence

Provenance may record that an equivalence check occurred.

For example:

source representation
    ↓
transformation
    ↓
verification: equivalent

The verification result belongs to the verifier.

Provenance records:

verified_by
verification_result
evidence

It does not itself establish equivalence.

---

102. Provenance and Contracts

A contract may generate:

contract evaluation

which produces:

pass
fail
unknown

Provenance SHOULD preserve the evaluation activity and relevant evidence.

A contract failure MUST remain distinguishable from:

parser failure
type failure
resource failure
capability failure
policy failure
runtime failure

---

103. Provenance and Diagnostics

Diagnostics MAY have provenance.

A diagnostic SHOULD identify:

source
activity
rule/specification
severity
location
cause
related entities

Diagnostics MUST remain separate from provenance itself.

---

104. Provenance and Errors

Errors MAY be represented as provenance events.

For example:

activity
    ↓
error
    ↓
recovery

A recovered error MUST NOT disappear from historical provenance.

This allows retry/recovery lineage to remain observable.

---

105. Provenance and Resilience

Provenance SHOULD record significant resilience transitions:

Healthy
    ↓
Degraded
    ↓
Recovering
    ↓
Healthy

or:

Healthy
    ↓
Unavailable
    ↓
Quarantined

The resilience subsystem remains authoritative for state semantics.

---

106. Provenance and QEC

Quantum error-correction activities MAY produce provenance for:

- encoded representation;
- error-correction strategy;
- syndrome information;
- correction;
- recovery;
- verification;
- resulting quantum state representation.

The provenance model MUST NOT define QEC itself.

QEC semantics remain downstream of "quantum::ir".

---

107. Provenance and Measurements

Measurement provenance SHOULD identify:

measurement activity
subject
measurement context
result
instrument/realization where appropriate
verification/validation

For quantum systems, measurement provenance MUST remain compatible with quantum semantic rules.

A measurement record MUST NOT imply a deterministic value where the semantic model permits uncertainty.

---

108. Provenance and Physical Observation

An observed physical result SHOULD be distinguishable from:

simulation result
predicted result
inferred result
generated result

These are different provenance categories.

---

109. Provenance and Generated Content

Generated content MUST preserve its relationship to the activity that generated it.

For example:

generated_artifact
    was_generated_by
code_generation_activity

and:

generated_artifact
    was_derived_from
semantic_IR

This prevents generated artifacts from being mistaken for original source.

---

110. Provenance and Dependencies

A dependency MAY itself have provenance.

This allows recursive lineage:

program
  ↓
dependency A
  ↓
dependency B
  ↓
source/data/artifact

Implementations MAY materialize the complete transitive graph or use references.

The semantic meaning MUST remain equivalent.

---

111. Provenance and Packages

Package provenance SHOULD record:

package
version
dependencies
build activity
source
artifact
integrity
verification

Package-manager semantics remain outside this specification.

---

112. Provenance and Dialects

Dialect-defined constructs MAY emit provenance.

A dialect SHOULD identify:

dialect identity
dialect version
extension identity
semantic interpretation

The universal provenance model MUST remain usable without knowing every dialect.

---

113. Provenance and Future Domains

A future computational domain MUST be able to reuse:

entity
activity
agent
relationship
evidence
decision
verification
context

without redesigning provenance.

This is a mandatory extensibility property.

---

114. Open-World Principle

The provenance model is open-world.

It MUST NOT require a closed list of:

- entity kinds;
- activity kinds;
- agents;
- evidence kinds;
- relationship kinds;
- compiler stages;
- hardware types;
- quantum technologies;
- AI models;
- datasets;
- artifact formats;
- target types.

Universal concepts belong in the core.

Domain-specific concepts belong in qualified namespaces/extensions.

---

115. Namespaces

Provenance extension names SHOULD be qualified.

Examples:

quantum::routing
hardware::placement
ai::training
data::transformation
security::authorization
compile::optimization

This reduces collisions between independent domains.

---

116. Generic Extension Mechanism

Where a concept is not yet universal, implementations SHOULD use existing extensibility mechanisms rather than introducing new universal keywords.

Possible mechanisms include:

qualified properties
metadata
attributes
dialects
capability identifiers
domain extensions

This keeps the core grammar scalable.

---

117. Provenance Properties

Properties MAY attach additional semantic metadata.

Properties SHOULD have:

qualified name
typed value
declared scope
provenance

Properties MUST NOT silently override normative semantics.

An extension property cannot redefine:

was_derived_from

to mean something incompatible with this specification.

---

118. Provenance Attributes

Attributes may annotate provenance entities or activities.

Attributes SHOULD be:

- typed;
- versioned when necessary;
- namespace-aware;
- semantically documented.

Attributes are metadata.

They do not automatically change program behavior.

---

119. Provenance Event Identity

Each provenance activity/event SHOULD have an identity sufficient to distinguish repeated occurrences.

For example:

compile activity #1
compile activity #2

must not be collapsed merely because both have the same activity kind.

---

120. Idempotence

A provenance consumer MAY process the same record more than once.

Implementations that provide persistent provenance SHOULD define stable identity semantics so duplicate ingestion can be detected where required.

The language does not mandate one storage protocol.

---

121. Ordering of Provenance Records

Provenance records MUST NOT depend on textual ordering unless ordering is semantically declared.

A serialized provenance format MAY use an order for deterministic encoding.

That encoding order is not automatically semantic execution order.

---

122. Canonicalization

When a canonical serialized representation exists, canonicalization SHOULD be deterministic.

Canonicalization MAY be required for:

- hashing;
- signatures;
- reproducible manifests;
- comparison;
- caching.

The specific serialization/canonicalization algorithm is owned by the relevant tooling/security layer.

---

123. Serialization

Provenance MAY be serialized.

A serialized representation SHOULD contain:

schema/version
identity
scope
entities
activities
agents
relationships
evidence
decisions
verification
context
extensions

Unknown fields MUST be handled according to the compatibility contract.

---

124. Unknown Future Fields

A reader MUST NOT reinterpret an unknown future field as a known field merely because their names are similar.

Depending on the compatibility profile, a reader MAY:

- preserve unknown fields;
- ignore them;
- reject the record;
- report a compatibility diagnostic.

The behavior MUST be deterministic and documented.

---

125. Forward Compatibility

New provenance fields SHOULD preferably be additive.

A new required field that makes older records uninterpretable requires a versioned compatibility transition.

---

126. Backward Compatibility

Existing valid provenance records MUST remain interpretable according to their declared schema/version and compatibility policy.

Meaning MUST NOT silently change.

---

127. Provenance Version

The provenance schema MUST have an explicit version.

The version identifies the semantic schema, not merely the implementation version.

For example:

provenance_schema_version

is distinct from:

compiler_version

and:

language_version

---

128. Provenance Migration

A migration MAY transform one provenance schema into another.

A migration SHOULD itself have provenance:

old provenance
    ↓
migration activity
    ↓
new provenance

Migration MUST NOT silently discard information that the new schema claims to preserve.

---

129. Provenance Completeness

A provenance graph MAY be:

complete
partial
projected
redacted
unknown

The status MUST be explicit when it materially affects interpretation.

A partial graph MUST NOT be presented as complete.

---

130. Provenance Coverage

Coverage describes which relevant lineage relationships are represented.

Coverage MAY be:

- source-only;
- compilation;
- execution;
- deployment;
- full lifecycle;
- domain-specific.

Coverage is metadata, not a universal correctness guarantee.

---

131. Provenance Gaps

If an expected provenance relationship is unavailable, the system SHOULD represent the gap when it matters.

For example:

source
    ↓
[unknown transformation]
    ↓
artifact

is preferable to falsely claiming:

source
    ↓
verified transformation
    ↓
artifact

---

132. Provenance Trust Boundary

A provenance consumer MUST know whether information is:

declared
observed
derived
inferred
verified
externally attested

This is especially important for:

- AI;
- distributed systems;
- security;
- scientific computing;
- compiler verification.

---

133. Self-Attestation

An activity MAY produce a self-attestation.

Self-attestation MUST remain distinguishable from independent verification.

For example:

compiler reports:
    optimization valid

is not automatically equivalent to:

independent verifier establishes:
    optimization valid

---

134. Independent Verification

Independent verification MAY be represented when a separate verifier/activity establishes a result.

Independence semantics MUST be defined by the relevant verification system.

Provenance records the relationship but does not decide whether two verifiers are sufficiently independent.

---

135. Evidence Conflicts

Conflicting evidence MUST remain representable.

For example:

evidence A → supports claim
evidence B → contradicts claim

The provenance system MUST NOT silently erase one side.

A separate decision or adjudication activity may resolve the conflict.

---

136. Supersession

A later entity MAY supersede an earlier entity.

Supersession MUST preserve the earlier entity's historical provenance.

Example:

assertion v1
    ↓
superseded_by
assertion v2

---

137. Retraction vs Supersession

These MUST remain distinct.

retraction

means the previous assertion is withdrawn or invalidated.

supersession

means another entity becomes the current replacement.

A system MAY perform both.

---

138. Provenance of Deletion

Deletion of a live entity does not necessarily mean deletion of its provenance.

Where historical provenance is retained, deletion SHOULD be represented as an activity.

---

139. Privacy

Provenance MAY contain sensitive information.

Implementations MUST apply applicable privacy/security policies.

The provenance specification does not require collecting information merely because it could be collected.

Data minimization SHOULD be preferred where complete lineage is unnecessary.

---

140. Secret Handling

Secrets MUST NOT be copied into provenance merely because an activity used them.

A provenance record SHOULD reference:

secret identity

rather than recording:

secret value

when the value is sensitive.

---

141. Credential Handling

Credentials, private keys, access tokens, and authentication secrets MUST NOT be placed into provenance as ordinary values.

Security policy remains authoritative.

---

142. Network Provenance

Network activities MAY record:

logical endpoint
protocol
request/result identity
activity
policy
resource
security context

Sensitive payloads MAY be excluded or referenced indirectly.

Networking semantics remain owned by:

grammar/networking/

---

143. Filesystem Provenance

Filesystem identity MAY be recorded where relevant.

However:

filesystem path

is not automatically a stable semantic identity.

A provenance implementation SHOULD use stable logical identities and integrity metadata where reproducibility requires it.

---

144. Environment Provenance

Environment information MAY include:

compiler
toolchain
language version
target profile
dependency graph
configuration
resource profile
policy profile

The implementation MUST distinguish declared values from observed values.

---

145. Host Provenance

Host information MAY be recorded for execution provenance.

It MUST NOT become an implicit source-level requirement.

For example:

executed_on host::X

does not imply:

requires host::X

---

146. Hardware Provenance

Hardware provenance MAY include:

target type
device identity
capability set
resource observations
topology
calibration context
placement
execution result

Hardware identity remains target/deployment metadata.

---

147. Calibration Provenance

Quantum or hardware systems may use calibration data.

Calibration provenance MAY record:

calibration identity
validity context
activity
target
result

Calibration semantics remain outside the universal provenance model.

---

148. Temporal Provenance

Provenance MAY record:

- wall-clock timestamps;
- logical timestamps;
- intervals;
- durations;
- ordering.

Temporal metadata MUST NOT alter source semantics unless time is explicitly part of the program.

---

149. Clock Independence

A portable provenance consumer MUST NOT assume that distributed activities share one perfectly synchronized physical clock.

Causal relationships SHOULD be represented semantically.

---

150. Provenance and Reproducible Builds

A reproducible build SHOULD be able to identify:

source
dependencies
compiler
toolchain
configuration
relevant policies
relevant resource assumptions
IR version
target profile
artifact
verification

Provenance supports this lineage.

The reproducibility subsystem determines whether reproduction actually succeeds.

---

151. Provenance and Cache Validity

A cache result SHOULD be reusable only when relevant provenance/context remains compatible.

Relevant context may include:

source identity
dependency identities
compiler version
language version
dialect versions
semantic configuration
target-independent compilation profile

Target-specific caches may additionally include target context.

---

152. Provenance and Specialization

Specialization SHOULD record:

generic entity
specialization parameters
specialization activity
specialized entity

Specialization MUST remain distinguishable from source rewriting.

---

153. Provenance and Generic Computation

Generic programs can produce many specialized realizations.

For example:

generic source
    ├── specialization A
    ├── specialization B
    └── specialization C

Provenance SHOULD preserve the common source lineage.

---

154. Provenance and Dynamic Scaling

When a runtime dynamically changes resource allocation, provenance MAY record:

initial resource state
observation
scaling decision
new allocation
reason
policy
result

The source program need not change.

This supports POCO-REAF without embedding machine capacities into the grammar.

---

155. Provenance and Elastic Execution

Distributed/cloud execution may expand or contract resources.

Provenance MAY record:

resource scale-up
resource scale-down
placement
migration
replication
deallocation

These are execution events, not language-level limits.

---

156. Provenance and Migration

An executing computation may migrate.

Provenance SHOULD represent:

execution state
    ↓
migration activity
    ↓
new execution context

The semantic computation remains one lineage.

---

157. Provenance and Fault Recovery

Recovery MAY produce:

failure
    ↓
diagnosis
    ↓
recovery decision
    ↓
recovered execution

The provenance graph SHOULD preserve the failed path and recovered path.

---

158. Provenance and Deterministic Replay

A replay system may consume provenance.

Replay MAY require:

- source identity;
- input identity;
- configuration;
- randomness context;
- toolchain;
- target profile;
- execution events.

Provenance itself does not guarantee replayability.

---

159. Provenance and Formal Proof

A proof MAY be an evidence entity.

The provenance graph MAY represent:

claim
    supported_by
proof

A proof object MUST NOT be considered valid merely because it is present.

The proof/verification subsystem establishes validity.

---

160. Provenance and Testing

Tests MAY produce provenance:

test
    ↓
execution
    ↓
observed result
    ↓
verification status

A test declaration is not the same as an observed passing test.

This distinction MUST remain visible.

---

161. Provenance and CI

Continuous integration MAY produce provenance for:

- source revision;
- build;
- tests;
- artifacts;
- diagnostics;
- verification;
- deployment.

CI tooling consumes the provenance model but does not redefine it.

---

162. Provenance and Release Artifacts

A release artifact SHOULD identify its lineage to:

source
build
compiler
dependencies
verification
packaging
release activity

---

163. Provenance and Package Distribution

Package distribution MAY attach:

package identity
version
artifact identity
dependency identities
build provenance
integrity
verification

Distribution mechanisms remain outside this specification.

---

164. Provenance and Interchange

Provenance exchanged between systems MUST preserve enough semantic information to avoid changing:

- identity;
- lineage;
- evidence status;
- verification status;
- relationship meaning.

Interchange formats may omit optional information only when the omission is explicit.

---

165. Provenance Loss

If an interchange operation drops provenance information, the result SHOULD identify the loss where it materially affects completeness.

For example:

full provenance
    ↓
export
    ↓
partial provenance

must not be silently labeled:

complete provenance

---

166. Provenance and Canonical Printing

A formatter MAY print provenance declarations.

Formatting MUST NOT change provenance semantics.

A formatter MUST NOT reorder semantically ordered records unless ordering is not semantic.

---

167. Provenance and AST

The AST SHOULD preserve sufficient provenance syntax structure for semantic construction.

The AST MUST NOT become the provenance graph itself.

The separation is:

parser context
    ↓
AST
    ↓
semantic provenance

---

168. Provenance and Semantic Analysis

Semantic analysis MUST:

- resolve provenance references;
- validate required relationships;
- validate entity identity rules;
- validate scope;
- validate version compatibility;
- validate types of provenance properties;
- validate relationship legality;
- distinguish missing evidence from negative evidence;
- distinguish declared from observed information;
- preserve source locations.

---

169. Provenance and Type Checking

Provenance properties MUST obey the type system.

For example:

confidence

must have a semantically valid representation according to the type system.

The provenance model MUST NOT introduce a second incompatible type system.

---

170. Provenance and Effects

Provenance recording MAY itself be an effect at runtime when it performs observable external persistence.

The language MUST distinguish:

provenance metadata as semantic information

from:

runtime provenance persistence

The latter may require I/O, storage, networking, or security capabilities.

---

171. Provenance and Capabilities

Persisting provenance may require capabilities such as:

provenance.record
provenance.store
provenance.attest
provenance.export

These are capability examples, not mandatory universal keywords.

Capability semantics remain owned by the capability system.

---

172. Provenance and Resources

Large provenance graphs consume resources.

The language MUST NOT pretend that provenance storage is free.

Implementations MAY:

- stream;
- compress;
- shard;
- summarize;
- project;
- retain references;
- use external stores.

These implementation strategies MUST preserve the declared provenance semantics.

---

173. Provenance Storage

Storage architecture is outside this specification.

Possible implementations include:

in-memory graph
database
append-only log
object store
content-addressed store
distributed store
stream
file

The semantic model is independent of storage technology.

---

174. Provenance Retrieval

Retrieval SHOULD support queries such as:

where did this entity come from?
what inputs produced this result?
which transformations affected this artifact?
which evidence supported this decision?
which policy governed this activity?
which resources/capabilities were involved?
which source location produced this entity?

Query syntax is owned by the appropriate data/query subsystem.

---

175. Provenance Query Semantics

A provenance query MUST distinguish:

stored provenance

from:

inferred provenance

An inferred relationship SHOULD be labeled as inferred.

The query engine MUST NOT present inference as recorded fact.

---

176. Provenance Inference

Implementations MAY infer transitive lineage.

For example:

A → B
B → C

may imply:

A → C

if the relationship is transitively defined.

The inferred relationship MUST remain distinguishable from an explicitly recorded relationship where that distinction matters.

---

177. Provenance Cycles

Some provenance graphs may contain cycles because of:

- recursive systems;
- feedback;
- long-running services;
- iterative learning;
- adaptive execution;
- distributed workflows.

The semantic model MUST NOT assume that every provenance graph is a finite tree.

Graph algorithms MUST handle cycles safely.

---

178. Provenance DAGs

Many compiler provenance graphs will naturally be DAGs.

Implementations SHOULD exploit DAG structure for:

- caching;
- incremental compilation;
- dependency analysis;
- reproducibility;
- optimization.

But the semantic model MUST remain general enough for systems that require cycles.

---

179. Provenance Identity and Equality

Two provenance entities are equal only according to their declared identity/equality semantics.

Equal display names do not imply equal identity.

Equal content does not necessarily imply equal provenance identity.

Equal semantic meaning does not necessarily imply equal historical lineage.

---

180. Provenance and Deduplication

Implementations MAY deduplicate equivalent provenance entities.

Deduplication MUST preserve semantic identity and lineage.

Two distinct activities MUST NOT be merged merely because they have identical parameters if they represent separate historical events.

---

181. Provenance and Parallelism

Parallel activities MUST preserve their independent lineage.

For example:

input
 ├──► activity A ──► result A
 └──► activity B ──► result B

must not be serialized artificially unless serialization is semantically established.

---

182. Provenance and Actors

Actors MAY be provenance agents or activity owners.

The actor model remains owned by:

grammar/concurrency/actors.g4

AI multi-agent semantics must integrate with the existing actor model rather than creating a competing provenance-specific actor model.

---

183. Provenance and Networking

Network messages MAY be provenance entities.

The provenance model SHOULD support:

sender
message
receiver
activity
result

without requiring a universal network protocol list.

---

184. Provenance and Security Decisions

Security decisions MAY have provenance:

request
    ↓
authorization policy
    ↓
evaluation
    ↓
decision

The provenance record does not grant authorization.

---

185. Provenance and Sandbox Decisions

Sandbox decisions MAY be represented similarly:

requested effect
    ↓
sandbox policy
    ↓
evaluation
    ↓
allow/deny

---

186. Provenance and Foreign Calls

An FFI call MAY record:

Zamani entity
    ↓
foreign call
    ↓
foreign function
    ↓
result/error

The actual ABI and calling convention remain owned by interoperability.

---

187. Provenance and Code Generation

Generated code SHOULD preserve:

semantic source
generation activity
generator/compiler
generator version
configuration
generated artifact
verification

Generated source MUST remain distinguishable from handwritten source.

---

188. Provenance and Macros

Macro expansion MAY record:

macro invocation
    ↓
macro expansion activity
    ↓
generated syntax

The macro system remains the syntax/semantic authority for macros.

---

189. Provenance and Compile-Time Execution

Compile-time execution MAY produce provenance.

It MUST distinguish:

compile-time activity

from:

runtime activity

when the distinction affects semantics.

---

190. Provenance and Reflection

Reflection may inspect semantic information.

Provenance MAY record:

reflection input
reflection activity
generated/derived output

Reflection does not gain unrestricted access merely because provenance exists.

---

191. Provenance and Generated Metadata

Compiler-generated metadata MAY itself have provenance.

This includes:

- optimization metadata;
- resource plans;
- capability decisions;
- diagnostics;
- source maps;
- debug information;
- verification results.

---

192. Provenance and Source Maps

Source maps SHOULD allow provenance consumers to move between:

generated representation

and:

original source

when such mapping exists.

The source-map subsystem remains authoritative for mapping semantics.

---

193. Provenance and Diagnostics Locations

A provenance-related diagnostic MUST preserve sufficient location information to identify:

- declaration;
- relationship;
- property;
- evidence;
- decision;
- source entity.

Diagnostic wording remains owned by the diagnostics subsystem.

---

194. Deterministic Provenance Generation

When provenance is generated in a deterministic mode, semantically equivalent inputs SHOULD produce equivalent provenance apart from explicitly nondeterministic metadata.

Nondeterministic fields such as wall-clock timestamps MUST be excluded, normalized, or explicitly identified when deterministic output is required.

---

195. Provenance and Timestamps

Timestamps are metadata.

A timestamp MUST NOT become semantic evidence merely because it exists.

Where trusted time is required, the appropriate attestation/time mechanism must establish its meaning.

---

196. Provenance and External Evidence

External evidence MAY be referenced.

The provenance record SHOULD identify:

external source identity
retrieval activity
retrieval context
content identity/integrity when available

Retrieval does not automatically establish truth.

---

197. Freshness

Evidence MAY have freshness metadata.

Freshness is distinct from:

truth
authenticity
integrity
verification

A stale verified fact remains historically verified but may no longer satisfy a current policy.

---

198. Provenance and Versioned Evidence

Evidence MAY become superseded as systems evolve.

The provenance graph MUST preserve the historical relationship.

Current applicability is a separate semantic determination.

---

199. Provenance and Scientific Computing

Scientific computations SHOULD be able to trace:

input
method
parameters
resource environment
simulation/measurement
transformation
result
verification

This supports reproducibility without requiring scientific-domain concepts in the core grammar.

---

200. Provenance and Numerical Computation

Numerical results MAY record:

- input identities;
- algorithm;
- precision context;
- transformations;
- target;
- verification;
- uncertainty.

The provenance system MUST NOT impose a fixed numeric precision.

---

201. Provenance and Tensor Computation

Tensor computation MAY record:

tensor input
shape/type metadata
operation
model
accelerator
result

Tensor semantics remain owned by the type/data/AI systems.

No universal tensor-rank limit may be encoded here.

---

202. Provenance and Memory

Memory-related provenance MAY identify:

logical allocation
resource requirement
realization
lifetime
result

It MUST NOT require a fixed machine address.

---

203. Provenance and Storage

Storage provenance MAY identify:

logical artifact
storage activity
storage identity
integrity
retrieval

Physical storage topology is implementation metadata.

---

204. Provenance and Energy

Energy-aware compilation/execution MAY record:

energy requirement
measurement
estimate
constraint
realization

Estimates MUST remain distinguishable from observations.

---

205. Provenance and Reliability

Reliability metadata MAY record:

required reliability
observed reliability
verification
failure
recovery

Reliability semantics remain owned by resource/resilience systems.

---

206. Provenance and Performance

Performance provenance SHOULD distinguish:

predicted performance
estimated performance
measured performance
verified performance

These MUST NOT be silently collapsed.

---

207. Provenance and Optimization Decisions

Optimization provenance MAY record:

candidate
decision
reason
evidence
result
verification

The optimizer may choose among valid alternatives.

Provenance explains the selected lineage.

---

208. Provenance and Target Negotiation

Target negotiation MAY produce:

requirements
candidate target
capabilities
constraints
decision
realization

The decision is not part of the source program unless explicitly declared.

---

209. Provenance and Portability Failures

If a target cannot satisfy the program's requirements, provenance MAY record:

source
requirements
target context
missing capability/resource
decision
failure

The source program remains unchanged.

---

210. Provenance and Fallbacks

If the execution model permits fallback:

primary realization
    ↓
failure
    ↓
fallback decision
    ↓
secondary realization

the provenance graph SHOULD preserve both paths.

The fallback MUST preserve the semantics permitted by the program's policy/contract.

---

211. Provenance and Graceful Degradation

A degraded result MAY have provenance indicating:

original requirement
degradation policy
resource condition
decision
degraded realization
result

A degraded result MUST NOT be represented as fully equivalent to the original result unless the relevant semantic contract establishes equivalence.

---

212. Provenance and Future Hardware

A future hardware architecture can produce provenance using the same:

entity
activity
agent
relationship
evidence
decision
context

model.

No redesign of the universal provenance semantics should be necessary merely because a new hardware class appears.

---

213. Hard-Coding Prohibition

This specification MUST NOT introduce universal limits such as:

MAX_PROVENANCE_NODES
MAX_PROVENANCE_EDGES
MAX_ENTITIES
MAX_ACTIVITIES
MAX_EVIDENCE
MAX_DECISIONS
MAX_LINEAGE_DEPTH
MAX_INPUTS
MAX_OUTPUTS
MAX_COMPILATION_STAGES
MAX_TARGETS
MAX_DEVICES
MAX_QUBITS
MAX_CPUS
MAX_GPUS
MAX_FPGAS
MAX_NODES
MAX_MEMORY
MAX_THREADS
MAX_REGISTER_WIDTH
MAX_TENSOR_RANK
MAX_NETWORK_SIZE
MAX_DEVICE_COUNT

No finite universal computational capacity may be encoded as a provenance semantic constant.

---

214. Resource-Bounded Implementation

An implementation is necessarily constrained by:

- memory;
- storage;
- processing time;
- network capacity;
- target capabilities;
- operating-system limits;
- physical constraints.

These are implementation constraints.

They MUST NOT become language-level provenance limits.

---

215. Safe Rust Requirement

All Zamani provenance implementation components written in Rust MUST:

- target Rust 1.97 or later;
- use Rust 2021 or later;
- use safe Rust;
- contain no "unsafe" blocks;
- not require an "unsafe" implementation contract;
- preserve deterministic semantics where required;
- use explicit error handling;
- avoid panics for malformed external provenance data where recoverable errors are expected.

The provenance specification itself is implementation-language independent, but the repository's production Rust implementation MUST satisfy these requirements.

---

216. Parser Safety Boundary

ANTLR grammar files MUST NOT contain embedded Rust actions or unsafe execution mechanisms for provenance semantics.

The parser produces structure.

Semantic validation occurs downstream.

---

217. Error Handling

Malformed provenance MUST produce structured diagnostics.

Implementations MUST distinguish at least:

syntax error
semantic error
reference-resolution error
schema/version error
compatibility error
integrity error
verification error
authorization error
resource error
capability error

The exact diagnostic taxonomy may be refined by:

grammar/spec/diagnostics.md

---

218. Provenance Reference Resolution

A provenance reference MUST resolve according to declared identity and scope rules.

Unresolved references MUST NOT silently resolve to another similarly named entity.

---

219. Dangling References

A provenance graph MAY contain a dangling external reference when the referenced entity is unavailable.

Such a reference MUST remain distinguishable from a successfully resolved reference.

---

220. Integrity Failure

If integrity verification fails, the provenance consumer MUST NOT report the affected content as successfully verified.

It MAY preserve the record as:

integrity_failed

for diagnostic/audit purposes.

---

221. Verification Failure

A failed verification MUST remain distinct from:

not verified

The difference is:

not verified

means no verification result is available,

while:

verification failed

means a verification activity produced a negative result.

---

222. Unknown Status

An unknown result MUST remain unknown.

The provenance model MUST NOT use absence of evidence as automatic evidence of failure.

---

223. Contradiction Preservation

Contradictory provenance MUST remain representable.

For example:

claim A
    supported_by evidence 1

claim A
    contradicted_by evidence 2

The graph remains valid.

Resolution requires a separate semantic activity.

---

224. Provenance and Human Review

Human review MAY be an activity/agent.

A human review result MUST remain distinguishable from automated verification.

The exact identity/privacy policy for human agents is outside this specification.

---

225. Provenance and Automated Agents

Automated agents MAY generate provenance.

Their outputs MUST remain distinguishable from independent verification.

Agent identity SHOULD include sufficient version/context information to reproduce or interpret the activity where required.

---

226. Provenance and Model Decisions

Model decisions SHOULD identify:

model identity/version
input
activity
result
evidence if available
uncertainty/confidence if available
policy
verification status

A model output is not automatically verified merely because it has provenance.

---

227. Provenance and Explainability

An explanation system SHOULD be able to traverse:

result
  ↓
decision
  ↓
evidence
  ↓
input
  ↓
transformation
  ↓
source

without requiring access to private internal reasoning.

---

228. Provenance and Knowledge Retraction

When knowledge is retracted:

old assertion
    ↓
retraction activity
    ↓
new active knowledge state

Historical lineage remains available subject to retention/security policy.

---

229. Provenance and Learning Adaptation

A model that adapts from new evidence SHOULD preserve:

previous model
    ↓
new evidence
    ↓
adaptation activity
    ↓
new model

This allows model lineage to remain explicit.

---

230. Provenance and Neural-Symbolic Computation

Hybrid AI systems MAY connect:

symbolic reasoning
        ↓
learned model
        ↓
inference
        ↓
symbolic decision

Provenance MUST preserve cross-domain relationships.

---

231. Provenance and Actors/Multi-Agent Systems

Each agent may have:

agent identity
activity
message
decision
result

Messages may be provenance entities.

Multi-agent semantics remain integrated with the existing concurrency/actor architecture.

---

232. Provenance and Simulation/Reality Boundary

The provenance graph MUST distinguish:

simulated
predicted
inferred
observed
measured
verified

unless a domain-specific semantic rule establishes a relationship.

---

233. Provenance and HDL Verification

HDL workflows may record:

source
elaboration
simulation
assertion
verification
synthesis
artifact

Verification status must distinguish simulation evidence from physical validation.

---

234. Provenance and Hardware Synthesis

Synthesis provenance MAY record:

HDL
    ↓
synthesis activity
    ↓
netlist/artifact

Physical implementation provenance may then continue:

netlist
    ↓
placement
    ↓
routing
    ↓
bitstream/mask/artifact

---

235. Provenance and Quantum Compilation

Quantum compilation provenance SHOULD preserve:

source circuit
    ↓
semantic circuit
    ↓
quantum::ir
    ↓
optimization
    ↓
decomposition
    ↓
routing
    ↓
scheduling
    ↓
resilience/QEC
    ↓
ZQN
    ↓
HAL

No quantum-specific provenance grammar may replace "quantum::ir".

---

236. Provenance and Hybrid Quantum-Classical Execution

A hybrid computation may produce:

classical input
    ↓
quantum preparation
    ↓
quantum execution
    ↓
measurement
    ↓
classical processing
    ↓
decision

All relationships MUST be representable in one provenance model.

---

237. Provenance and Data/Query Systems

Queries may produce derived data.

Provenance SHOULD preserve:

query
inputs
dependencies
transformation
result

Query syntax remains owned by data/query subsystems.

---

238. Provenance and JSON/XML/SQL Interoperability

External data formats MAY carry provenance.

Conversion SHOULD preserve provenance when the external format supports it.

Conversion itself SHOULD be represented as an activity.

---

239. Provenance and ABI

ABI adaptation MAY produce:

source type
ABI conversion
foreign representation
result conversion

The ABI remains owned by interoperability.

---

240. Provenance and Compilation Profiles

Compilation profiles MAY affect:

optimization
resource selection
target selection
reproducibility
debugging
verification

Provenance SHOULD identify the profile used.

---

241. Provenance and Compatibility

Compatibility decisions MAY be recorded:

source version
target version
compatibility evaluation
migration
result

The compatibility subsystem remains authoritative.

---

242. Provenance and Deprecation

When deprecated constructs are transformed or migrated, provenance SHOULD preserve:

original construct
migration activity
replacement construct
compatibility rule

---

243. Provenance and Historical Records

Historical provenance SHOULD remain append-oriented conceptually.

A newer interpretation may supersede an old one without rewriting historical facts.

---

244. Provenance and Audit

Audit systems may consume provenance.

Audit policy determines:

- retention;
- access;
- integrity;
- authorization;
- redaction.

Provenance provides semantic lineage.

---

245. Provenance and Observability

Observability systems may produce runtime provenance.

Observability data MUST remain distinguishable from normative source semantics.

---

246. Provenance and Debugging

Debuggers MAY use:

source maps
provenance
IR mappings
runtime events

to connect runtime behavior to source.

The debugger does not redefine provenance.

---

247. Provenance and Tooling

LSPs, formatters, package managers, compilers, test tools, deployment tools, and analysis tools MAY consume provenance.

No tool may independently invent a conflicting provenance meaning.

---

248. Provenance and Documentation

Documentation MAY describe provenance behavior.

Normative provenance meaning remains here plus the higher-level language specifications.

---

249. Required Cross-File Integration

The following integration contract is mandatory.

249.1 "grammar/compile/provenance.g4"

Owns:

compileProvenanceDeclaration
compileProvenanceBody
compileProvenanceMember

It MUST conform to this semantic specification.

It MUST NOT define a competing provenance semantic model.

---

249.2 "grammar/ai/provenance.g4"

Owns AI-specific provenance syntax.

It MUST map to the universal:

Entity
Activity
Agent
Relationship
Evidence
Decision
Verification

model.

---

249.3 "grammar/security/provenance.g4"

Owns security-specific provenance syntax.

It MUST integrate with:

identity
authorization
policy
trust
attestation
audit

without redefining generic lineage.

---

249.4 "grammar/data/provenance.g4"

Owns data-specific provenance syntax.

It MUST use the universal derivation/lineage semantics.

---

249.5 "grammar/expressions/provenance.g4"

Owns expression-level provenance syntax.

It MUST produce references to semantic entities rather than creating a separate provenance object model.

---

249.6 "grammar/validation/evidence.g4"

Owns evidence syntax.

Evidence semantics MUST conform to this file.

---

249.7 "grammar/spec/resources.md"

Owns resource semantics.

Provenance MAY record resource facts but MUST NOT redefine:

requirement
constraint
capability
budget
preference
hint

---

249.8 "grammar/spec/effects.md"

Owns effect semantics.

Provenance records effect participation but does not grant effects.

---

249.9 "grammar/spec/determinism.md"

Owns determinism semantics.

Provenance records deterministic context but does not create determinism.

---

249.10 "grammar/spec/portability.md"

Owns portability semantics.

Provenance records target/realization differences without weakening source portability.

---

249.11 "grammar/spec/source-spans.md"

Owns source-span semantics.

Provenance references source spans rather than redefining them.

---

249.12 "grammar/spec/source-map.md"

Owns source-map semantics.

Provenance may consume source maps to connect generated representations with source.

---

249.13 "grammar/spec/quantum.md"

Owns quantum semantic meaning.

Provenance references "quantum::ir" and quantum activities without defining quantum computation.

---

249.14 "grammar/spec/hdl.md"

Owns HDL semantics.

Provenance records HDL transformations without becoming an HDL representation.

---

249.15 "grammar/specification/poco-reaf.md"

Owns the language-level POCO-REAF contract.

This file provides the provenance mechanism supporting that contract.

---

250. Canonical Semantic Provenance Model

The implementation SHOULD converge on a domain-neutral model conceptually equivalent to:

ProvenanceGraph
{
    schema,
    scope,
    entities,
    activities,
    agents,
    relationships,
    evidence,
    decisions,
    verification,
    context,
    extensions
}

The exact Rust structure is implementation-owned.

The semantic fields are normative.

---

251. Entity Model

Conceptually:

ProvenanceEntity
{
    id,
    kind,
    scope,
    version,
    source,
    properties
}

---

252. Activity Model

Conceptually:

ProvenanceActivity
{
    id,
    kind,
    scope,
    agent,
    inputs,
    outputs,
    context,
    properties
}

---

253. Agent Model

Conceptually:

ProvenanceAgent
{
    id,
    kind,
    version,
    identity,
    properties
}

---

254. Relationship Model

Conceptually:

ProvenanceRelationship
{
    id,
    kind,
    source,
    target,
    activity,
    properties
}

---

255. Evidence Model

Conceptually:

ProvenanceEvidence
{
    id,
    claim,
    source,
    relation,
    status,
    verifier,
    context,
    properties
}

---

256. Decision Model

Conceptually:

ProvenanceDecision
{
    id,
    activity,
    inputs,
    alternatives,
    selected,
    constraints,
    evidence,
    policy,
    result
}

---

257. Verification Model

Conceptually:

ProvenanceVerification
{
    id,
    subject,
    verifier,
    method,
    result,
    evidence,
    specification,
    context
}

---

258. No Fixed Rust Representation

The conceptual structures above are semantic contracts.

The implementation MUST NOT be forced into exactly these Rust structs.

The Rust implementation MAY use:

- structs;
- enums;
- interned identifiers;
- arena-like structures;
- graph indexes;
- streaming records;
- persistent storage references.

The semantic behavior must remain equivalent.

---

259. Rust Safety

No provenance implementation may require:

unsafe

for correctness.

Safe Rust MUST be sufficient for:

- parsing;
- semantic construction;
- validation;
- serialization;
- graph processing;
- provenance queries;
- provenance verification.

---

260. Panic Safety

Malformed provenance supplied by an external source MUST produce controlled errors rather than process termination through avoidable panics.

This is especially important for:

- untrusted serialized provenance;
- network provenance;
- package metadata;
- compiler artifacts;
- distributed execution records.

---

261. Resource Exhaustion Safety

Implementations SHOULD detect and report resource exhaustion rather than assuming provenance graphs are small.

Examples:

memory exhausted
storage exhausted
input too large
graph processing budget exceeded
serialization budget exceeded

These are implementation/resource errors.

They are not semantic language limits.

---

262. Deterministic Validation

Given identical:

provenance input
schema
semantic context
compatibility context

validation SHOULD produce deterministic results.

Validation MUST NOT depend on:

- random selection;
- current wall-clock time;
- unspecified hash-map iteration order;
- current hardware;
- network state;

unless those are explicitly part of the provenance being validated.

---

263. Concurrency Safety

Concurrent provenance processing MUST preserve semantic identity.

Implementations MUST avoid data races.

Safe Rust's ownership model SHOULD be used to enforce this.

---

264. Thread/Task Independence

No provenance algorithm may assume a fixed number of threads or tasks.

Parallel processing MAY scale according to available resources.

---

265. Storage Scalability

Implementations SHOULD support partitioning provenance across:

files
objects
shards
nodes
streams
databases

without changing semantic identity.

---

266. Distributed Storage

A distributed provenance store MAY partition a graph.

References MUST remain globally interpretable within the declared provenance namespace.

---

267. Large Graph Traversal

Consumers SHOULD support bounded traversal where a complete graph is impractical.

A bounded query result MUST be identified as a projection/subgraph.

---

268. Infinite or Unbounded Processes

Some computations may conceptually run indefinitely.

Provenance MUST support incremental records for such activities.

It MUST NOT require completion before provenance becomes valid.

---

269. Long-Running Activities

A long-running activity MAY have:

started
progress
checkpoint
pause
resume
failure
recovery
completed

events.

These remain one activity lineage unless semantic execution creates distinct activities.

---

270. Checkpoint Provenance

Checkpoints SHOULD identify:

activity
state identity
source lineage
checkpoint context
verification/integrity

A checkpoint MUST NOT automatically be treated as a completed result.

---

271. Resume Provenance

A resumed activity SHOULD reference its checkpoint.

Conceptually:

activity
   ↓
checkpoint
   ↓
resume
   ↓
continuation

---

272. Provenance and State Machines

Execution state transitions MAY be recorded as activities/events.

The execution subsystem owns state semantics.

---

273. Provenance and Transaction Boundaries

Transactional systems MAY record:

transaction
inputs
activities
commit
rollback
result

A rolled-back transaction MUST remain distinguishable from a committed transaction.

---

274. Provenance and Atomicity

Provenance recording MUST NOT falsely imply that an operation was atomic if it was not.

Atomicity semantics remain owned by the execution/data system.

---

275. Provenance and Side Effects

Activities with side effects SHOULD record the relevant effect category and resulting entities where permitted.

Provenance does not authorize side effects.

---

276. Provenance and Native/Foreign Execution

Native or foreign execution MAY have special provenance because it crosses a semantic boundary.

The record SHOULD identify:

foreign boundary
ABI/context
external entity
result

---

277. Provenance and Code Generation Effects

Code generation MAY produce source or executable artifacts.

Generated artifacts MUST retain their derivation relationship.

---

278. Provenance and Security-Sensitive Generation

Security-sensitive generated artifacts MAY require verification before deployment.

Provenance SHOULD connect:

generation
    ↓
verification
    ↓
deployment

---

279. Provenance and Deployment

Deployment provenance MAY record:

artifact
deployment activity
target
policy
resource/capability context
result

Deployment remains downstream of compilation.

---

280. Provenance and Cloud/Cluster Execution

Cloud/cluster metadata MAY record:

logical deployment
resource allocation
placement
execution
results

Cloud provider identities MUST NOT become universal language constructs.

---

281. Provenance and Embedded Execution

Tiny targets may have minimal provenance support.

A constrained target MAY emit compact provenance references or forward provenance to another system.

The source semantics remain unchanged.

---

282. Provenance and Very Large Systems

Large systems MAY generate extremely large provenance graphs.

The implementation MUST use scalable representation strategies rather than imposing language-level graph limits.

---

283. Provenance and Future Architectures

A future target may introduce entirely new:

resource
capability
activity
artifact
execution model

Provenance MUST remain able to represent these through extension mechanisms.

---

284. Extension Registration

An implementation MAY maintain registries for:

entity kinds
activity kinds
relationship kinds
property schemas
evidence kinds
verification methods

Registries are implementation/extensibility infrastructure.

They are not a closed language enumeration.

---

285. Extension Versioning

Extensions SHOULD declare:

identity
version
compatibility
owner
schema

---

286. Vendor Extensions

Vendor-specific provenance MAY exist.

Vendor identifiers MUST remain qualified.

Vendor extensions MUST NOT redefine universal relationships incompatibly.

---

287. Dialect Extensions

Dialect provenance MAY describe:

dialect
extension
version
semantic contribution

---

288. Experimental Extensions

Experimental provenance features MUST be marked experimental.

They MUST NOT silently become stable semantics.

---

289. Deprecated Provenance

Deprecated provenance constructs MUST specify:

deprecation version
replacement
migration path
compatibility policy
removal status

---

290. Historical Provenance

Historical provenance records may remain readable after the current language evolves.

The schema/version identifies the historical interpretation.

---

291. Provenance and Language Version

A provenance record SHOULD identify the language version when source semantics depend on it.

Language version semantics remain owned by:

grammar/specification/language-version.md

---

292. Provenance and Grammar Version

Where parser interpretation depends on a grammar version, provenance SHOULD identify the relevant grammar version.

---

293. Provenance and Specification Version

Verification records SHOULD identify which specification/version was applied.

This is especially important for:

- semantic validation;
- contract verification;
- quantum verification;
- hardware verification;
- reproducibility.

---

294. Provenance and IR Version

IR-producing activities SHOULD identify the IR schema/version.

For quantum compilation this includes the applicable "quantum::ir" version.

---

295. Provenance and Target Profile

Target-specific realization MAY identify:

target profile
capability profile
resource profile
backend version

---

296. Provenance and Semantic Profile

A semantic profile MAY identify:

language features
dialects
policies
compatibility mode

---

297. Provenance and Compilation Intent

Compilation intent may include:

portable
deterministic
reproducible
optimized
debug
verified
simulation

Provenance may record which intent governed an activity.

---

298. Provenance and Simulation Mode

A simulation compilation/execution should preserve:

simulation profile
simulator identity/version
simulation assumptions
input
result

---

299. Provenance and Verification Mode

A verified compilation may preserve:

verification profile
verifier
method
result
evidence

---

300. Provenance and Policy Mode

A policy-constrained compilation may preserve:

policy identity
policy version
evaluation
decision
result

---

301. Provenance Graph Invariants

A conforming semantic provenance graph MUST preserve the following invariants.

Invariant 1 — Identity

Every referenced entity MUST have an interpretable identity.

Invariant 2 — Relationship

Every relationship MUST have a defined semantic kind.

Invariant 3 — Scope

Every identity MUST have an interpretable scope.

Invariant 4 — Status

Verification/evidence status MUST remain distinguishable.

Invariant 5 — Lineage

Derivation relationships MUST preserve direction.

Invariant 6 — Evidence

Evidence MUST remain distinguishable from assertions.

Invariant 7 — Decision

Decisions MUST remain distinguishable from evidence.

Invariant 8 — Observation

Observations MUST remain distinguishable from predictions/inferences.

Invariant 9 — Realization

Target realization MUST remain distinguishable from source intent.

Invariant 10 — Version

Schema/version information MUST remain interpretable.

Invariant 11 — Extensibility

Unknown domain extensions MUST NOT corrupt universal relationships.

Invariant 12 — Scalability

No arbitrary universal finite capacity may be encoded.

---

302. Provenance Graph Validity

A provenance graph is semantically valid when:

1. identities resolve according to their declared scope;
2. relationships are legal;
3. required references exist or are explicitly external/unresolved;
4. version compatibility is satisfied;
5. evidence statuses are valid;
6. verification statuses are valid;
7. source mappings are valid where present;
8. policy/security restrictions are respected;
9. extensions do not redefine universal semantics;
10. no forbidden hard-coded capacity is introduced.

---

303. Semantic Validation Order

Where practical, validation SHOULD proceed in dependency order:

schema/version
    ↓
identity
    ↓
references
    ↓
relationship structure
    ↓
types
    ↓
scope
    ↓
evidence
    ↓
verification
    ↓
policy/security
    ↓
domain-specific semantics

Implementations MAY optimize the order while preserving observable semantic correctness.

---

304. Provenance Diagnostics

Required diagnostic classes SHOULD include:

unknown provenance schema
unsupported provenance version
unknown entity reference
duplicate identity
invalid relationship
invalid relationship direction
invalid scope
invalid status
invalid evidence state
invalid verification state
incompatible version
invalid source mapping
invalid extension
integrity failure
authentication failure
authorization failure
resource exhaustion
incomplete provenance

Exact diagnostic wording belongs to the diagnostics subsystem.

---

305. Positive Conformance Requirements

The repository MUST include positive tests covering:

minimal provenance
source provenance
dependency provenance
transformation provenance
artifact provenance
compiler provenance
quantum provenance
HDL provenance
AI provenance
data provenance
security provenance
distributed provenance
resource provenance
capability provenance
policy provenance
contract provenance
evidence
decision
verification
adaptation
learning
simulation
reproducibility
deterministic compilation
cross-domain provenance

---

306. Negative Conformance Requirements

Tests MUST cover rejection of:

unknown schema
invalid relationship
unresolved required reference
invalid version
invalid status
invalid evidence state
invalid verification state
conflicting identity
malformed source reference
illegal scope
invalid extension
forbidden semantic override

---

307. Boundary Tests

Tests MUST include:

- empty provenance;
- one entity;
- many entities;
- one activity;
- many activities;
- deep lineage;
- branching lineage;
- merging lineage;
- parallel activities;
- retries;
- recovery;
- partial failure;
- conflicting evidence;
- redaction;
- projection;
- unknown extension;
- future schema field;
- large property sets;
- cross-domain references.

---

308. Scalability Tests

Scalability tests MUST verify that the semantic model imposes no artificial finite limit on:

entities
activities
agents
relationships
evidence
decisions
inputs
outputs
lineage depth
branching
parallel activities
domains
target realizations

Tests MUST scale according to available resources.

The test suite MUST distinguish:

language rejection

from:

test-environment resource exhaustion

---

309. Determinism Tests

The repository MUST test deterministic provenance processing for identical semantic inputs.

Tests SHOULD cover:

- serialization;
- canonicalization;
- validation;
- graph traversal;
- merge;
- projection;
- migration.

Nondeterministic metadata MUST be explicitly controlled.

---

310. Compatibility Tests

Tests MUST cover:

current → current
previous → current
current → future-compatible reader
extension → base reader
base → extension reader

according to the repository compatibility policy.

---

311. Cross-Domain Tests

The provenance suite MUST include combinations such as:

classical + quantum
classical + HDL
quantum + HDL
quantum + hardware
quantum + distributed
AI + quantum
AI + hardware
AI + data
classical + quantum + distributed
classical + quantum + HDL + hardware
AI + quantum + classical + distributed

The purpose is to prove that provenance composes rather than producing separate incompatible provenance universes.

---

312. POCO-REAF Integration Test

At least one end-to-end test MUST trace one source program through:

source
  ↓
lexer
  ↓
parser
  ↓
AST
  ↓
structural validation
  ↓
type/effect/resource/capability analysis
  ↓
contracts
  ↓
policies
  ↓
provenance
  ↓
semantic model
  ↓
Classical IR and/or quantum::ir
  ↓
optimization
  ↓
lowering
  ↓
routing
  ↓
scheduling
  ↓
resilience/QEC where applicable
  ↓
ZQN
  ↓
HAL
  ↓
target realization

The provenance graph MUST preserve the meaningful lineage between stages.

---

313. Tiny-to-Large Integration Test

The same source semantic program SHOULD be evaluated under different target/resource contexts:

tiny
embedded
single CPU
multicore
GPU
FPGA
ASIC
accelerator
QPU
simulator
HPC
cluster
distributed
cloud
future target profile

The source provenance identity MUST remain stable.

Realization provenance MAY differ.

---

314. No Reinterpretation Rule

A downstream file MUST NOT need to reinterpret this specification to determine the meaning of:

entity
activity
agent
derivation
generation
evidence
decision
verification
lineage

Those semantics are fixed here.

Domain-specific files add domain information.

---

315. Independent-File Completion Contract

This file is considered complete independently when all of the following are true:

- provenance has a complete semantic definition;
- ownership is explicit;
- non-ownership is explicit;
- existing provenance grammars are integrated;
- AST boundary is explicit;
- semantic boundary is explicit;
- IR boundary is explicit;
- "quantum::ir" boundary is explicit;
- resource integration is explicit;
- capability integration is explicit;
- effect integration is explicit;
- contract integration is explicit;
- policy integration is explicit;
- evidence integration is explicit;
- source-span integration is explicit;
- source-map integration is explicit;
- determinism integration is explicit;
- reproducibility integration is explicit;
- security integration is explicit;
- AI integration is explicit;
- data integration is explicit;
- distributed integration is explicit;
- hardware integration is explicit;
- HDL integration is explicit;
- runtime integration is explicit;
- POCO-REAF integration is explicit;
- scalability rules are explicit;
- Rust 1.97+ requirement is explicit;
- safe-Rust/no-"unsafe" requirement is explicit;
- positive tests are defined;
- negative tests are defined;
- boundary tests are defined;
- scalability tests are defined;
- determinism tests are defined;
- compatibility tests are defined;
- cross-domain tests are defined;
- completion criteria do not depend on future edits to this file merely because another integrated file changes.

---

316. Integration Contract for Future Files

A future provenance-related file MUST declare:

PURPOSE
OWNS
DOES_NOT_OWN
DEPENDS_ON
EXPORTS
CONSUMED_BY
AST_OWNER
SEMANTIC_OWNER
IR_OWNER
SPEC_OWNER
TEST_OWNER
RESOURCE_CONTRACT
CAPABILITY_CONTRACT
EFFECT_CONTRACT
CONTRACT_CONTRACT
POLICY_CONTRACT
PROVENANCE_CONTRACT
SCALABILITY_CONTRACT
COMPATIBILITY_CONTRACT
COMPLETION_CRITERIA

The new file MUST reference this specification rather than redefining universal provenance semantics.

---

317. Canonical Ownership Matrix

The repository MUST maintain the following ownership direction:

Language specification
        ↓
Semantic specifications
        ↓
Domain grammar
        ↓
AST
        ↓
Semantic model
        ↓
Canonical IR
        ↓
Optimization
        ↓
Lowering
        ↓
Routing
        ↓
Scheduling
        ↓
Runtime/HAL

Provenance crosses these layers as metadata.

It does not replace them.

---

318. Final Architectural Model

The complete provenance architecture is:

                         SOURCE
                           │
                           ▼
                          AST
                           │
                           ▼
                    SEMANTIC MODEL
                           │
             ┌─────────────┼─────────────┐
             │             │             │
             ▼             ▼             ▼
         CLASSICAL      QUANTUM         HDL
             │             │             │
             ▼             ▼             ▼
      Classical IR     quantum::ir   HDL representation
             │             │             │
             └─────────────┼─────────────┘
                           │
                           ▼
                    TRANSFORMATION
                           │
                           ▼
                    TARGET IR / ZQN
                           │
                           ▼
                          HAL
                           │
                           ▼
                    TARGET REALIZATION
                           │
                           ▼
                       EXECUTION
                           │
                           ▼
                         RESULT

At every important boundary:

                 ┌──────────────────┐
                 │    PROVENANCE    │
                 └──────────────────┘
                    ▲    ▲    ▲
                    │    │    │
                 source activity result
                    │    │    │
                    └────┼────┘
                         │
                    evidence
                    decisions
                    verification
                    policy
                    resources
                    capabilities

---

319. Fundamental Invariant

The fundamental invariant is:

PROVENANCE DESCRIBES LINEAGE.
PROVENANCE DOES NOT CREATE LINEAGE.

The actual activity creates the semantic event.

The provenance system records it.

---

320. Fundamental Trust Invariant

The following distinctions MUST remain intact:

assertion     ≠ evidence
evidence      ≠ verification
verification  ≠ truth
prediction    ≠ observation
simulation    ≠ physical observation
decision      ≠ evidence
policy        ≠ provenance
resource      ≠ capability
requirement   ≠ realization
source intent ≠ target realization

---

321. Fundamental Portability Invariant

The provenance model MUST preserve:

one source semantic program
        ↓
many valid realizations

without converting realization details into source requirements.

---

322. Fundamental Scalability Invariant

The provenance model MUST remain valid from:

tiny computation

through:

embedded
CPU
multicore
GPU
FPGA
ASIC
accelerator
QPU
simulator
HPC
cluster
distributed
cloud
future architectures

without changing the universal provenance semantics.

---

323. Fundamental Extensibility Invariant

A new computational domain MUST be able to introduce:

new entity kinds
new activity kinds
new relationship kinds
new evidence kinds
new properties

without requiring the universal provenance model to enumerate the entire future of computing.

---

324. Fundamental Safety Invariant

Production Zamani provenance implementation MUST use:

Rust 1.97+
Rust 2021+
safe Rust

and MUST NOT require:

unsafe

for parsing, validation, semantic construction, storage, serialization, graph processing, or verification.

---

325. Fundamental Physical-Reality Invariant

POCO-REAF does not mean physically infinite hardware.

It means the language does not impose arbitrary finite limits on the computational universe.

Actual execution remains bounded by:

available resources
target capabilities
physical constraints
time
energy
memory
storage
network capacity
implementation capacity

A resource failure is therefore not automatically a language failure.

---

326. Final Production-Readiness Gate

"grammar/spec/provenance.md" is production-ready only when every provenance-producing feature can answer:

WHAT is the subject?
WHERE did it originate?
WHICH activity produced it?
WHO/WHAT performed the activity?
WHAT inputs were used?
WHAT outputs were produced?
WHAT transformations occurred?
WHAT dependencies participated?
WHAT evidence exists?
WHAT decisions were made?
WHAT policies governed them?
WHAT resources/capabilities participated?
WHAT verification occurred?
WHAT source location is associated?
WHAT specification/version applies?
WHAT target realization occurred?
WHAT was observed?
WHAT remains unknown?
WHAT was redacted?
WHAT was superseded?
WHAT remains reproducible?
WHAT is the compatibility status?

No stable provenance feature may have an unknown semantic owner.

---

327. Final Completion Checklist

This file is DONE when:

- [ ] "grammar/spec/provenance.md" exists.
- [ ] "grammar/compile/provenance.g4" references it as its specification owner.
- [ ] "grammar/ai/provenance.g4" conforms to it.
- [ ] "grammar/security/provenance.g4" conforms to it.
- [ ] "grammar/data/provenance.g4" conforms to it.
- [ ] "grammar/expressions/provenance.g4" conforms to it.
- [ ] "grammar/validation/evidence.g4" conforms to it.
- [ ] compilation provenance has no competing semantic model.
- [ ] AI provenance has no competing semantic model.
- [ ] security provenance has no competing semantic model.
- [ ] data provenance has no competing semantic model.
- [ ] source-span ownership remains in "grammar/spec/source-spans.md".
- [ ] source-map ownership remains in "grammar/spec/source-map.md".
- [ ] determinism ownership remains in "grammar/spec/determinism.md".
- [ ] reproducibility ownership remains in "grammar/compile/reproducibility.g4".
- [ ] resource ownership remains in "grammar/spec/resources.md".
- [ ] effect ownership remains in "grammar/spec/effects.md".
- [ ] contract ownership remains in validation specifications.
- [ ] policy ownership remains in policy/security specifications.
- [ ] quantum ownership remains in "grammar/spec/quantum.md".
- [ ] "quantum::ir" remains the canonical quantum IR boundary.
- [ ] no second quantum IR is introduced.
- [ ] HDL ownership remains in "grammar/spec/hdl.md".
- [ ] target realization remains downstream.
- [ ] runtime remains downstream.
- [ ] provenance does not grant capabilities.
- [ ] provenance does not grant permissions.
- [ ] provenance does not create resources.
- [ ] provenance does not perform compilation.
- [ ] provenance does not perform optimization.
- [ ] provenance does not perform routing.
- [ ] provenance does not perform scheduling.
- [ ] provenance does not implement QEC.
- [ ] provenance does not implement cryptography.
- [ ] provenance does not expose secrets.
- [ ] provenance distinguishes evidence from assertions.
- [ ] provenance distinguishes observations from predictions.
- [ ] provenance distinguishes decisions from evidence.
- [ ] provenance distinguishes verification from assertion.
- [ ] provenance preserves contradictory evidence.
- [ ] provenance preserves retractions.
- [ ] provenance preserves supersession.
- [ ] provenance supports partial graphs.
- [ ] provenance supports distributed graphs.
- [ ] provenance supports incremental graphs.
- [ ] provenance supports very large graphs.
- [ ] provenance has no universal finite graph limit.
- [ ] provenance has no hard-coded hardware capacity.
- [ ] provenance supports POCO-REAF.
- [ ] Rust 1.97+ compatibility is defined.
- [ ] Rust 2021+ compatibility is defined.
- [ ] safe Rust is mandatory.
- [ ] "unsafe" is prohibited.
- [ ] positive tests are defined.
- [ ] negative tests are defined.
- [ ] boundary tests are defined.
- [ ] scalability tests are defined.
- [ ] determinism tests are defined.
- [ ] compatibility tests are defined.
- [ ] cross-domain tests are defined.
- [ ] end-to-end POCO-REAF provenance testing is defined.
- [ ] tiny-to-large realization testing is defined.

---

328. Final Law

Zamani provenance SHALL follow this law:

ONE LANGUAGE
ONE SEMANTIC PROVENANCE MODEL
MANY DOMAINS
MANY REPRESENTATIONS
MANY COMPILATION STRATEGIES
MANY TARGETS
MANY EXECUTION ENVIRONMENTS
MANY RESOURCE SCALES
ONE CONTINUOUS LINEAGE

Therefore:

                 Zamani Source
                       │
                       ▼
               Domain-Neutral AST
                       │
                       ▼
                Semantic Model
                       │
          ┌────────────┼────────────┐
          ▼            ▼            ▼
     Classical       Quantum        HDL
          │            │            │
          ▼            ▼            ▼
    Classical IR   quantum::ir   HDL IR/model
          │            │            │
          └────────────┼────────────┘
                       │
                       ▼
              Optimization/Lowering
                       │
                       ▼
                Routing/Scheduling
                       │
                       ▼
                 ZQN / Backend
                       │
                       ▼
                      HAL
                       │
                       ▼
                Target Realization
                       │
                       ▼
                    Execution
                       │
                       ▼
                     Result

PROVENANCE accompanies every meaningful transformation:

Source
  ↓
Entity
  ↓
Activity
  ↓
Derived Entity
  ↓
Verification/Evidence
  ↓
Decision
  ↓
Realization
  ↓
Result

The provenance model therefore provides the missing semantic lineage layer needed for a production-grade POCO-REAF architecture without turning provenance into another grammar authority, another IR, another resource system, another policy system, or another execution engine.

Its governing principle is:

«Record the lineage of meaning without hard-coding the limits of the machine that realizes that meaning.»
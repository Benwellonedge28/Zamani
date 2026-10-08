Zamani Universal Capability Integration Specification

Path: "grammar/specification/ubuntu-integration.md"
Repository: "Benwellonedge28/Zamani"
Language: Zamani
Specification class: Normative integration specification
Status: Production architecture contract
Rust baseline: Rust 1.97.1 or later
Rust edition: Rust 2021
Rust safety: Zamani-owned Rust MUST use safe Rust; "unsafe" MUST NOT be used
Portability model: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)
Scalability model: Open-ended and resource-dependent
Primary integration principle: Extend existing Zamani architecture; do not create a parallel language or semantic subsystem.

---

1. Purpose

This specification defines how the useful computational capabilities identified from an external language design are to be integrated into Zamani.

The integration is intentionally semantic and architectural, not a grammar transplant.

The integrated capabilities include:

- reasoning;
- inference;
- deduction;
- knowledge;
- assertion and retraction;
- querying;
- learning;
- controlled adaptation;
- pattern matching;
- guards;
- contracts;
- assumptions;
- guarantees;
- properties;
- uncertainty;
- probability;
- distributions;
- confidence;
- evidence;
- provenance;
- explainability;
- decision records;
- policies;
- sandboxing;
- simulation;
- agents;
- actor-based multi-agent execution;
- neural-symbolic computation;
- richer type-system capabilities;
- FFI and ABI;
- controlled reflection;
- metaprogramming;
- data interoperability;
- SQL interoperability;
- JSON interoperability;
- XML interoperability;
- quantum-classical hybrid computation;
- adaptive execution;
- deterministic and reproducible execution.

These capabilities MUST become first-class participants in Zamani's existing architecture.

They MUST NOT establish:

- a second programming language;
- a second lexer;
- a second parser authority;
- a second AST;
- a second type system;
- a second effect system;
- a second resource system;
- a second actor system;
- a second policy system;
- a second provenance system;
- a second quantum IR;
- a second compiler pipeline.

---

2. Normative Authority

This specification is subordinate to:

1. "grammar/DESIGN.md"
2. the normative specification architecture under "grammar/specification/"
3. the focused contracts under "grammar/spec/"

It integrates with:

- "grammar/Zamani.g4";
- "grammar/antlr/ZamaniLexer.g4";
- "grammar/antlr/ZamaniParser.g4";
- "grammar/lexer/";
- "grammar/core/";
- "grammar/types/";
- "grammar/expressions/";
- "grammar/statements/";
- "grammar/declarations/";
- "grammar/functions/";
- "grammar/modules/";
- "grammar/effects/";
- "grammar/resources/";
- "grammar/validation/";
- "grammar/policies/" where present;
- "grammar/ai/";
- "grammar/classical/";
- "grammar/data/";
- "grammar/quantum/";
- "grammar/hybrid/";
- "grammar/hdl/";
- "grammar/hardware/";
- "grammar/concurrency/";
- "grammar/distributed/";
- "grammar/networking/";
- "grammar/security/";
- "grammar/execution/";
- "grammar/interoperability/";
- "grammar/metaprogramming/";
- "grammar/dialects/";
- "grammar/compatibility/";
- "grammar/compile/";
- "grammar/tests/";
- "src/lexer.rs";
- "src/parser.rs";
- "src/frontend/ast/";
- semantic analysis;
- canonical IR;
- "src/quantum/ir/";
- optimization;
- lowering;
- routing;
- scheduling;
- resilience;
- QEC;
- ZQN;
- HAL;
- runtime;
- target backends.

If this document conflicts with "grammar/DESIGN.md", "DESIGN.md" governs and this specification MUST be corrected.

If this document conflicts with another domain specification, the conflict MUST be resolved explicitly through the specification/versioning process.

No implementation may resolve specification conflicts implicitly.

---

3. Integration Objective

The objective is to extend Zamani's universal semantic model so that computational reasoning, knowledge, learning, adaptation, uncertainty, evidence, policies, contracts, explainability, and related capabilities can operate alongside:

- classical computation;
- quantum computation;
- HDL;
- hardware/software co-design;
- AI/ML;
- tensor computation;
- data computation;
- concurrency;
- distributed computation;
- networking;
- accelerators;
- embedded computation;
- HPC;
- cloud;
- future computational domains.

The common architecture is:

VALUE
  │
  ▼
TYPE
  │
  ▼
OPERATION
  │
  ├───────────────┬────────────────┬────────────────┐
  ▼               ▼                ▼                ▼
EFFECT        CAPABILITY        RESOURCE       PROVENANCE
  │               │                │                │
  └───────────────┴────────────────┴────────────────┘
                          │
                          ▼
                     REQUIREMENT
                          │
                          ▼
                      CONSTRAINT
                          │
                          ▼
                        POLICY
                          │
                          ▼
                       CONTRACT
                          │
                          ▼
                       EVIDENCE
                          │
                          ▼
                       DECISION
                          │
                          ▼
                  SEMANTIC REPRESENTATION
                          │
              ┌───────────┴───────────┐
              ▼                       ▼
        Classical IR             quantum::ir
              │                       │
              └───────────┬───────────┘
                          ▼
                     Optimization
                          ▼
                       Lowering
                          ▼
                  Routing / Mapping
                          ▼
                      Scheduling
                          ▼
                  Resilience / QEC
                          ▼
                          ZQN
                          ▼
                          HAL
                          ▼
                        Target

This common model is mandatory.

---

4. Scope

This specification governs:

- language-level syntax integration;
- semantic ownership;
- AST integration;
- type integration;
- effect integration;
- resource integration;
- capability integration;
- policy integration;
- contract integration;
- provenance integration;
- IR integration;
- compiler integration;
- runtime integration;
- target integration;
- interoperability;
- scalability;
- determinism;
- reproducibility;
- security;
- compatibility;
- conformance testing.

It does not define:

- individual machine-learning algorithms;
- individual commercial services;
- particular AI products;
- physical hardware implementations;
- vendor-specific execution protocols;
- operating-system administration;
- payment systems;
- legal procedures;
- application-specific business logic;
- arbitrary administrative commands;
- physical device inventories.

Those belong to libraries, dialects, services, policies, capabilities, runtimes, or applications.

---

5. Architectural Principles

5.1 One Language

All integrated capabilities are part of Zamani.

There MUST NOT be an embedded independent language with its own incompatible:

- grammar;
- type system;
- AST;
- effect system;
- resource model;
- actor model;
- IR.

---

5.2 Existing Features MUST Be Extended

Before creating a new file, implementation MUST:

1. locate an existing owner;
2. inspect its grammar;
3. inspect its consumers;
4. inspect its AST mapping;
5. inspect its semantic model;
6. inspect its tests;
7. determine whether extension is sufficient.

A new file is justified only when the responsibility is genuinely distinct.

---

5.3 Generic Semantics Over Keyword Explosion

A new capability MUST be represented through existing generic abstractions whenever possible.

For example, future machine-learning algorithms SHOULD be represented as:

operation
+
model
+
parameters
+
types
+
capabilities
+
resources
+
effects
+
policy
+
provenance

rather than requiring a new reserved keyword for every algorithm.

---

5.4 Application Features Are Not Core Language Concepts

Application-specific concepts MUST NOT become universal keywords merely because they are useful in an application domain.

Examples include:

- computer vision operations;
- sentiment-analysis operations;
- robotics operations;
- blockchain operations;
- VR/AR operations;
- payment operations;
- administrative operations;
- legal workflows;
- domain-specific business processes.

Such capabilities belong in:

- libraries;
- registered operations;
- dialects;
- capability registries;
- policies;
- services;
- applications.

---

6. POCO-REAF Contract

POCO-REAF is achieved through semantic portability, not through universal physical assumptions.

The required model is:

Source intent
    ↓
Types
    ↓
Effects
    ↓
Requirements
    ↓
Capabilities
    ↓
Constraints
    ↓
Policies
    ↓
Canonical semantic model
    ↓
Canonical IR
    ↓
Target discovery
    ↓
Capability negotiation
    ↓
Specialization
    ↓
Optimization
    ↓
Routing
    ↓
Scheduling
    ↓
Resilience
    ↓
Target realization

The source program SHOULD remain unchanged as target scale changes.

A target may be:

- tiny embedded hardware;
- a single CPU;
- multicore CPU;
- GPU;
- FPGA;
- ASIC;
- accelerator;
- QPU;
- simulator;
- HPC system;
- cluster;
- distributed environment;
- cloud environment;
- future computational target.

The source MUST NOT silently change meaning because a target cannot satisfy a requirement.

The compiler/runtime MUST instead:

1. satisfy the requirement;
2. select an explicitly permitted fallback;
3. request another realization;
4. report the unmet requirement;
5. or reject the realization.

---

7. Open-Ended Scalability

Zamani MUST NOT encode artificial universal capacity limits.

The following kinds of limits are prohibited from the universal language architecture:

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

Equivalent renamed constants are also prohibited.

This applies to:

- grammar;
- AST;
- semantic model;
- canonical IR;
- universal resource model;
- universal capability registry;
- language specification.

Actual implementations MAY have resource limits.

Such limits MUST be represented as:

- target constraints;
- compiler resource limits;
- runtime policies;
- security budgets;
- memory exhaustion;
- implementation capacity;
- physical limitations.

They MUST NOT become language semantics.

---

8. Resource Model

Integrated constructs MUST use the existing resource model.

The semantic distinction is:

Requirement
Constraint
Capability
Preference
Hint
Budget
Negotiation
Realization

Examples:

requires qubits >= required_qubits;
requires memory >= required_memory;
requires capability("quantum.measurement");
requires capability("gpu.compute");
requires capability("tensor.compute");
requires topology(required_topology);

prefer capability("accelerator.compute");

constrain latency <= latency_budget;

The grammar expresses intent.

Resource analysis determines feasibility.

Target discovery determines availability.

Realization determines physical mapping.

---

9. Effect Model

All new computational operations MUST participate in the existing effect system.

Applicable effects include:

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

Additional effect categories MAY be registered.

Examples:

learn
    → learning effect

adapt
    → adaptation effect

measure
    → measurement / quantum effect

reflect
    → reflection effect

foreign_call
    → foreign effect

simulate
    → simulation effect

An effect MUST have semantic consequences.

It MUST NOT be merely documentation.

---

10. Contract Model

The following universal contract concepts are supported:

requires
ensures
invariant
assume
guarantee
property
assert

A contract has:

condition
scope
assumptions
requirements
guarantees
evidence
provenance

Contracts MAY apply to:

- functions;
- operations;
- quantum operations;
- hardware modules;
- AI models;
- agents;
- actors;
- distributed services;
- data transformations;
- resource negotiations;
- compilation transformations.

Contracts MUST remain traceable through semantic lowering where their guarantees remain observable or required.

---

11. Policy Model

Policies are distinct from contracts.

A contract states a semantic obligation.

A policy governs permitted or preferred behavior.

Policies MAY express:

- requirements;
- constraints;
- permissions;
- prohibitions;
- preferences;
- fallbacks;
- authorization;
- adaptation rules;
- execution rules;
- security rules;
- deployment rules;
- simulation rules;
- resource-selection rules.

Policy decisions MUST be available to:

- semantic validation;
- resource negotiation;
- execution planning;
- security;
- adaptation;
- target selection;
- provenance.

---

12. Provenance Model

All integrated capabilities MUST use the repository-wide provenance model.

Where applicable, provenance records include:

source
derived_from
generated_by
transformed_by
verified_by
reason
evidence
decision
version
timestamp

Provenance MAY describe:

- source facts;
- inferred facts;
- learned models;
- adaptations;
- compiler transformations;
- optimization;
- resource selection;
- target selection;
- quantum routing;
- hardware placement;
- security decisions;
- simulation results.

The semantic model MUST NOT require unbounded in-memory history.

Persistence and storage strategies belong to the implementation.

---

13. Reasoning

13.1 Ownership

Primary grammar owner:

grammar/ai/reasoning.g4

Supporting owners:

grammar/expressions/
grammar/statements/
grammar/validation/
grammar/ai/
grammar/spec/provenance.md
grammar/spec/effects.md

13.2 Operations

The language MAY expose:

infer
deduce
reason

These MUST converge into one reasoning semantic model.

The language MUST NOT create three unrelated reasoning systems.

13.3 Semantic Model

A reasoning operation SHOULD represent:

Reasoning {
    premises
    evidence
    relations
    conclusion
    confidence
    policy
    effects
    capabilities
    resources
    provenance
}

The exact AST representation MUST be domain-neutral.

Reasoning MAY operate on:

- scientific facts;
- program facts;
- symbolic expressions;
- compiler information;
- hardware capabilities;
- quantum information;
- AI knowledge;
- security facts;
- distributed state.

---

14. Knowledge

Primary ownership:

grammar/ai/knowledge.g4
grammar/data/

Knowledge operations include:

assert
retract
query

The semantic model MUST represent knowledge independently from any specific database.

Conceptually:

KnowledgeAssertion {
    subject
    relation
    object
    metadata
    provenance
}

Knowledge MUST integrate with:

- data;
- reasoning;
- evidence;
- provenance;
- contracts;
- policies;
- AI;
- scientific computation.

---

15. Querying

Queries MUST use existing expression/data infrastructure where possible.

Query semantics MUST remain independent from SQL syntax.

SQL is an interoperability/dialect concern.

A query can target:

- in-memory data;
- structured data;
- graph data;
- knowledge;
- distributed data;
- external data services.

Query execution MAY require:

capability("data.query")
effect("io")
effect("network")

depending on its realization.

---

16. Learning

Primary ownership:

grammar/ai/learning.g4

Learning MUST be represented as a semantic operation rather than an algorithm catalogue.

A learning operation MUST be able to carry:

input
target
model
data
objective
algorithm
parameters
resources
capabilities
effects
constraints
policy
provenance

The grammar MUST NOT require a new keyword for each future learning algorithm.

Algorithm identity SHOULD be represented through:

- qualified operation names;
- libraries;
- dialects;
- metadata;
- registered capabilities.

Learning MUST integrate with:

types
data
tensors
classical computation
quantum computation
hybrid computation
resources
effects
policies
provenance

---

17. Controlled Adaptation

Primary ownership:

grammar/ai/adaptation.g4
grammar/execution/adaptive.g4
grammar/policies/
grammar/security/

Adaptation MUST NOT imply unrestricted self-modifying code.

A permitted adaptation requires appropriate:

authorization
policy
capability
effect
resource
validation
provenance

Conceptually:

adapt
   ↓
authorize
   ↓
validate
   ↓
evaluate
   ↓
apply permitted change
   ↓
record provenance
   ↓
continue / recover / reject

Adaptation MAY modify:

- model parameters;
- execution strategy;
- scheduling strategy;
- resource selection;
- algorithm selection;
- routing strategy;
- deployment strategy;
- permitted state.

The semantic system MUST define exactly what category of state may be modified.

Arbitrary mutation of compiler or language semantics is prohibited.

---

18. Pattern Matching and Guards

Pattern matching belongs to:

grammar/expressions/
grammar/statements/

It MUST use the existing expression and type architecture.

Patterns MAY match:

- values;
- types;
- records;
- variants;
- tuples;
- knowledge structures;
- messages;
- quantum results;
- execution states;
- hardware capabilities;
- resource states.

Guards MUST be expressions subject to normal:

- type checking;
- effect checking;
- capability checking;
- resource checking;
- policy checking.

---

19. Uncertainty and Probability

Primary integration:

grammar/types/
grammar/ai/
grammar/data/
grammar/expressions/

Conceptual semantic types include:

Uncertain<T>
Probability<T>
Distribution<T>
Confidence<T>

These names describe semantic categories.

They MUST NOT require one universal probabilistic runtime representation.

Implementations MAY realize them using:

- exact representations;
- symbolic representations;
- numerical representations;
- sampling;
- hardware acceleration;
- quantum methods;
- distributed methods.

The source semantics remain stable.

---

20. Evidence

Evidence is a universal semantic facility.

An evidence record MAY contain:

claim
source
support
confidence
derivation
verification
provenance

Evidence MAY support:

- reasoning;
- AI decisions;
- scientific conclusions;
- compiler transformations;
- resource decisions;
- hardware placement;
- quantum routing;
- security decisions.

Evidence MUST NOT be restricted to AI.

---

21. Explainability

Explainability is a semantic capability for describing why a decision or transformation occurred.

It MAY explain:

- an inference;
- a learned-model result;
- an adaptation;
- a compiler optimization;
- a target selection;
- a resource allocation;
- a quantum routing decision;
- a hardware placement;
- a security decision;
- a simulation result.

Explanation records SHOULD connect:

decision
reason
evidence
inputs
transformation
provenance

The implementation MAY choose the storage format.

---

22. Decision Records

A decision record MUST be representable independently of a specific AI system.

Conceptually:

Decision {
    subject
    alternatives
    selected
    reason
    evidence
    policy
    constraints
    provenance
}

Decision records MAY be generated by:

- compiler;
- runtime;
- resource negotiation;
- optimizer;
- scheduler;
- router;
- AI subsystem;
- security subsystem.

---

23. Agents

Agent semantics MUST reuse the existing actor architecture.

Required direction:

AI agent
    ↓
existing actor semantics
    ↓
message
    ↓
channel
    ↓
scheduler/runtime

"grammar/concurrency/actors.g4" remains the owner of actor lifecycle and actor semantics.

"grammar/ai/multi-agent.g4" MAY define:

- agent identity;
- goals;
- capabilities;
- policies;
- reasoning;
- learning;
- adaptation;
- agent-specific metadata.

It MUST NOT define a second actor lifecycle.

---

24. Neural-Symbolic Computation

Neural-symbolic computation MUST compose existing semantic domains.

Required conceptual architecture:

symbolic computation
       ↕
reasoning
       ↕
learned model
       ↕
tensor/data computation
       ↕
classical / quantum / accelerator execution

The implementation MUST NOT create a separate neural-symbolic compiler.

All operations MUST use the common:

- type system;
- effects;
- capabilities;
- resources;
- contracts;
- policies;
- provenance;
- canonical IR.

---

25. Multi-Agent Computation

Multi-agent execution MUST integrate with:

grammar/concurrency/
grammar/distributed/
grammar/ai/
grammar/networking/

It MUST support, where applicable:

- actor identity;
- messages;
- channels;
- tasks;
- goals;
- policies;
- capabilities;
- resource requirements;
- distributed placement;
- failure handling;
- provenance.

No fixed maximum number of agents may exist in the universal grammar.

---

26. Simulation

Primary ownership:

grammar/execution/simulation.g4

Simulation is an execution strategy, not a separate programming language.

It MAY simulate:

- classical systems;
- quantum systems;
- hardware;
- distributed systems;
- networks;
- AI models;
- performance;
- faults;
- resource availability.

Simulation MUST consume the same semantic program representation as ordinary execution.

The semantic distinction is:

program meaning
        ↓
execution strategy
        ├── physical execution
        └── simulation

---

27. Adaptive Execution

Primary ownership:

grammar/execution/adaptive.g4

Adaptive execution MAY provide:

detect
evaluate
select
fallback
retry
recover
adapt

It MUST integrate with existing resilience states:

Unknown
Healthy
Degraded
Unstable
Unavailable
Recovering
Quarantined
Retired

and existing outcomes:

ACCEPT
DEGRADED_ACCEPT
RETRY
RECOVER
ESCALATE
REJECT

These states MUST remain semantic/runtime concepts rather than becoming a second execution language.

---

28. Deterministic and Reproducible Execution

The integrated capabilities MUST distinguish:

deterministic semantics

from:

deterministic implementation

and:

reproducible execution

Where deterministic behavior is required, the specification MUST define:

- ordering;
- seed semantics;
- numerical semantics;
- concurrency constraints;
- randomness;
- provenance;
- target-dependent variation.

Randomness MUST participate in the effect system.

Learning and adaptation MUST expose reproducibility requirements where applicable.

---

29. Sandbox

Sandboxing belongs to:

grammar/security/
grammar/execution/
grammar/policies/

A sandbox MAY constrain:

- effects;
- capabilities;
- resources;
- filesystem;
- network;
- native execution;
- FFI;
- reflection;
- code generation;
- adaptation;
- external processes.

Conceptually:

sandbox {
    forbid effect("network");
    forbid capability("native.execute");
}

The exact syntax MUST follow the existing policy grammar.

A sandbox MUST NOT be implemented as a special AI-only facility.

---

30. Reflection and Metaprogramming

Primary ownership:

grammar/metaprogramming/
grammar/macros/

Capabilities include:

- reflection;
- introspection;
- compile-time execution;
- code generation;
- quotation;
- syntax-tree manipulation;
- type-level computation.

Reflection MUST be capability/effect aware.

Code generation MUST be provenance-aware.

Generated code MUST remain subject to:

- parsing;
- validation;
- typing;
- effect checking;
- capability checking;
- resource checking;
- policy checking;
- compatibility checking.

Reflection MUST NOT silently bypass language safety or policy rules.

---

31. Type-System Expansion

The useful type-system capabilities MUST be integrated into the existing type architecture.

Priority 1

- generics;
- generic bounds;
- associated types;
- type constraints;
- linear types;
- affine types;
- pattern-oriented types.

Priority 2

- dependent types;
- type classes;
- variance;
- existential types.

Future extension

- higher-kinded types;
- type families;
- functional dependencies;
- path-dependent types;
- type providers;
- advanced type-level computation.

No type-system extension may introduce artificial hardware capacity limits.

Type-level dimensions MUST remain capable of being:

- literal;
- symbolic;
- generic;
- computed;
- runtime-dependent where supported.

---

32. FFI and ABI

Primary ownership:

grammar/interoperability/

The subsystem MUST cover:

- foreign declarations;
- foreign functions;
- foreign types;
- ABI;
- calling conventions;
- linkage;
- data layout;
- ownership/borrowing boundaries;
- external symbols.

FFI MUST participate in the effect system.

For example:

foreign call
    → foreign effect

Additional capabilities MAY be required.

Zamani-owned Rust MUST remain safe Rust.

External unsafe implementation details MUST NOT require "unsafe" in Zamani-owned Rust.

---

33. SQL Interoperability

SQL MUST NOT become universal Zamani grammar.

Use:

grammar/dialects/sql/

or the existing interoperability architecture.

Required pipeline:

SQL dialect
    ↓
dialect parser
    ↓
Zamani query/data semantic model
    ↓
canonical semantic representation
    ↓
backend/connector

SQL syntax MUST NOT redefine Zamani's universal query semantics.

---

34. JSON and XML

JSON and XML are interoperability/data formats.

They MUST NOT become universal core language syntax.

Use:

grammar/dialects/json/
grammar/dialects/xml/

or the existing interoperability ownership.

The semantic pipeline is:

format
   ↓
decoder/encoder
   ↓
Zamani data model
   ↓
types / schemas / provenance
   ↓
canonical representation

---

35. Quantum Integration

Quantum functionality MUST integrate with:

grammar/quantum/
grammar/hybrid/
grammar/ai/
grammar/data/
grammar/resources/
grammar/effects/
grammar/policies/

The canonical quantum boundary remains:

src/quantum/ir/

No separate quantum-learning IR, reasoning IR, or hybrid quantum IR may replace "quantum::ir".

---

36. Quantum Operation Extensibility

Quantum operations MUST be data-driven.

The grammar MUST NOT permanently enumerate every operation.

Conceptual structure:

operationSpecifier
+
targets
+
parameters
+
results
+
attributes
+
modifiers

An operation MAY be:

- built-in;
- user-defined;
- parameterized;
- namespaced;
- vendor-provided;
- dialect-provided;
- future-defined.

The semantic operation model SHOULD contain:

name
namespace
operands
parameters
results
attributes
modifiers
effects
capabilities
source

Adding a future quantum operation MUST NOT require rewriting the universal operation grammar merely because the operation did not exist previously.

---

37. Quantum-Classical Hybrid Integration

Hybrid computation MUST support:

classical → quantum
quantum → classical
classical control → quantum operation
measurement → classical decision
AI → quantum
quantum → AI
CPU → accelerator
host → device

All such interactions MUST use common:

- types;
- effects;
- resources;
- capabilities;
- policies;
- provenance.

---

38. HDL and Hardware Integration

Hardware intent MUST remain separate from physical realization.

The source may express:

what computation is required
what timing is required
what interface is required
what resources are required
what capabilities are required
what constraints apply

The compiler determines:

where
how
with which device
with which physical resources
with which placement
with which routing

HDL MUST integrate with:

- types;
- contracts;
- verification;
- resources;
- capabilities;
- scheduling;
- routing;
- synthesis;
- provenance.

No universal fixed hardware capacity may be embedded.

---

39. Classical Integration

Reasoning, learning, data, uncertainty and knowledge MUST be able to operate over classical computation.

Classical operations MUST use the same:

types
operations
effects
capabilities
resources
contracts
policies
provenance

AI is therefore not a separate computational universe.

---

40. Data Integration

Data capabilities MUST integrate with:

grammar/data/
grammar/types/
grammar/ai/
grammar/expressions/
grammar/interoperability/

Supported semantic categories MAY include:

- records;
- schemas;
- arrays;
- tensors;
- graphs;
- streams;
- datasets;
- knowledge structures;
- distributions;
- provenance records.

No universal maximum dataset size, graph size, tensor rank, or stream length may be imposed by grammar architecture.

---

41. Networking Integration

Networking MUST use existing:

grammar/networking/
grammar/distributed/
grammar/security/
grammar/effects/
grammar/resources/
grammar/policies/

Network operations SHOULD expose:

network effect
network capability
resource requirements
security policy
provenance

The language MUST NOT hard-code a universal network topology or device count.

---

42. Distributed Integration

Distributed computation MUST reuse:

actors
messages
channels
tasks
services
collectives
topologies
consistency
fault tolerance

No fixed maximum number of nodes, agents, services, tasks or devices may exist at the universal language level.

Placement remains downstream.

---

43. Capability Architecture

Capabilities are open-ended identifiers.

Examples:

capability("quantum.measurement")
capability("quantum.dynamic_circuit")
capability("gpu.compute")
capability("tensor.compute")
capability("network.secure")
capability("data.query")
capability("native.execute")

The capability namespace MUST be extensible.

The language MUST NOT require a universal exhaustive list of all future hardware or software capabilities.

A capability describes availability.

It does not grant permission by itself.

Authorization remains a policy/security concern.

---

44. Requirement/Capability Separation

The program states:

requires capability("tensor.compute");

The target advertises:

capability("tensor.compute")

Negotiation determines whether the target can satisfy the requirement.

This distinction is mandatory.

A program MUST NOT encode target discovery directly into universal source syntax.

---

45. Policy/Capability Separation

A capability answers:

Can this environment provide it?

A policy answers:

May this operation use it?

Both checks are required where applicable.

For example:

capability("native.execute")

does not automatically authorize native execution.

---

46. Provenance/Decision Separation

Provenance records what happened and why it can be traced.

A decision represents a selected outcome.

They are related but distinct.

Conceptually:

Decision
    ↓
Reason
    ↓
Evidence
    ↓
Provenance

This allows decisions from:

- optimization;
- scheduling;
- resource selection;
- AI;
- security;
- quantum routing;
- adaptation

to be explained without making provenance itself responsible for decision semantics.

---

47. AST Contract

All integrated constructs MUST map into the existing domain-neutral AST architecture.

The AST MUST NOT contain target-specific structures such as:

- physical CPU identifiers;
- vendor-specific QPU topology;
- physical qubit assignments;
- calibration state;
- scheduler state;
- runtime device state.

Where a universal operation is required, the AST SHOULD preserve generic fields such as:

name
namespace
operands
parameters
results
attributes
modifiers
effects
capabilities
source

Additional fields require an explicit AST specification and ownership review.

---

48. Semantic Contract

After parsing, semantic analysis MUST resolve:

names
types
operations
effects
capabilities
resources
contracts
policies
provenance

Domain-specific semantics then refine the universal representation.

For example:

learn

may become:

LearningOperation

while retaining universal:

types
effects
capabilities
resources
policies
provenance

---

49. Canonical IR Contract

The integrated features MUST lower to existing canonical IR boundaries.

Classical computation MUST use the canonical classical IR.

Quantum computation MUST use:

quantum::ir

AI, reasoning, learning, adaptation and knowledge operations MUST NOT create independent permanent IR universes.

Temporary domain IRs MAY exist for optimization/lowering, provided that they have:

- explicit ownership;
- versioning;
- conversion rules;
- semantic-preservation rules;
- tests;
- provenance;
- a defined lifetime in the pipeline.

---

50. Compiler Integration

Every integrated feature MUST be traceable through:

specification
    ↓
lexer
    ↓
grammar
    ↓
AST
    ↓
structural validation
    ↓
type checking
    ↓
effect checking
    ↓
capability checking
    ↓
resource checking
    ↓
contract checking
    ↓
policy checking
    ↓
provenance
    ↓
canonical IR
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
ZQN where applicable
    ↓
HAL
    ↓
target

A parser-only implementation MUST NOT be reported as complete.

---

51. Runtime Integration

Runtime behavior MUST consume validated semantic intent.

Runtime MUST NOT silently bypass:

- policies;
- capabilities;
- resource requirements;
- effects;
- contracts.

Runtime adaptation MUST remain within the authorized adaptation policy.

Runtime decisions SHOULD be provenance-capable where the language or execution policy requires traceability.

---

52. Safe Rust Contract

All Zamani-owned implementation associated with these features MUST:

- target Rust 1.97.1 or later;
- use Rust 2021;
- use safe Rust;
- avoid "unsafe";
- preserve source spans;
- use typed representations;
- use structured diagnostics;
- avoid hard-coded universal capacities;
- remain deterministic where semantics require determinism.

External dependencies MAY internally use implementation mechanisms outside Zamani's control, but Zamani-owned code MUST NOT require "unsafe".

---

53. Compatibility

Every integrated construct MUST define:

- introduction version;
- stability state;
- syntax compatibility;
- semantic compatibility;
- AST compatibility;
- IR compatibility;
- serialization compatibility where applicable;
- dialect compatibility;
- deprecation behavior.

A construct MUST NOT be removed silently.

Compatibility changes MUST use the repository's existing compatibility/versioning architecture.

---

54. Feature Lifecycle

Every feature follows:

proposal
   ↓
semantic design
   ↓
specification
   ↓
lexer contract
   ↓
grammar
   ↓
AST
   ↓
semantic implementation
   ↓
IR
   ↓
compiler/runtime integration
   ↓
tests
   ↓
compatibility review
   ↓
stable

Possible states include:

PROPOSED
SPECIFIED
EXPERIMENTAL
IMPLEMENTED
PARTIALLY_IMPLEMENTED
TESTED
STABLE
DEPRECATED
HISTORICAL

No feature becomes stable merely because its grammar parses.

---

55. Required Repository Integration

The following ownership map is normative.

Capability| Primary owner| Supporting owners
Reasoning| "grammar/ai/"| expressions, statements, validation
Knowledge| "grammar/ai/", "grammar/data/"| provenance
Assertion| existing validation/assertion owner| AI/data
Retraction| "grammar/ai/"| data
Query| "grammar/data/"| expressions
Learning| "grammar/ai/"| data, types, effects
Adaptation| "grammar/ai/", "grammar/execution/"| policies, security
Pattern matching| expressions/statements| types
Guards| expressions| validation
Contracts| validation| policies, provenance
Uncertainty| types/AI/data| expressions
Evidence| AI/provenance| validation
Explainability| AI/provenance| compiler/runtime
Decisions| provenance/semantic model| execution
Policies| policy subsystem| security/resources/execution
Sandbox| security/execution| policies
Simulation| execution| quantum/HDL/distributed/AI
Agents| AI| concurrency/distributed
Actor lifecycle| concurrency| AI
Neural-symbolic| AI| types/data/quantum/hybrid
FFI| interoperability| effects/security
ABI| interoperability| compiler/backend
Reflection| metaprogramming| security/effects
SQL| dialect/interoperability| data
JSON| dialect/interoperability| data
XML| dialect/interoperability| data
QML| AI + quantum + hybrid| quantum::ir
Adaptive execution| execution| resilience/policies
Reproducibility| compile/execution/provenance| compatibility

---

56. Lexer Integration

Canonical lexical ownership remains:

grammar/antlr/ZamaniLexer.g4
grammar/lexer/

Only language-level reserved words that genuinely require reserved lexical treatment should be introduced.

Potential canonical words include:

infer
deduce
reason
learn
adapt
query
retract
explain
evidence
provenance
policy
requires
ensures
invariant
assume
guarantee
property
sandbox
simulate

Before adding any token:

1. determine whether ordinary identifiers suffice;
2. determine whether the word creates ambiguity;
3. update the token registry;
4. update lexer tests;
5. update compatibility data;
6. update parser consumers;
7. update documentation.

Application names MUST remain identifiers.

---

57. Parser Integration

"grammar/antlr/ZamaniParser.g4" remains the canonical parser composition point.

The parser MUST consume subordinate domain grammars.

The root parser MUST NOT duplicate every new AI, data, quantum, policy or execution production.

The integration MUST preserve:

core
→ expressions
→ statements
→ declarations
→ types
→ domains
→ validation
→ execution

without creating cyclic semantic ownership.

---

58. No Duplicate Subsystems

The following MUST NOT be duplicated:

- actor systems;
- effects;
- resource systems;
- capability systems;
- policies;
- provenance;
- contracts;
- quantum IR;
- type systems;
- query systems.

When two domains need the same concept, they MUST consume the common abstraction.

---

59. New Directory Policy

A new directory MAY be introduced only when:

1. an existing directory has no clear ownership;
2. the new responsibility is independently maintainable;
3. the directory has a specification owner;
4. the directory has an AST/semantic/IR integration contract;
5. tests have a defined home;
6. the dependency direction is documented.

A possible dedicated:

grammar/policies/

directory is justified because policies span security, execution, resources, adaptation, deployment and simulation.

However, if an equivalent existing policy owner is already present, that owner MUST be extended instead of creating a duplicate.

---

60. Individual Feature Contract

Every new or modified grammar feature MUST document:

Feature identity
Purpose
Normative status
Owns
Does not own

DEPENDS_ON
EXPORTS
CONSUMED_BY

Lexer dependencies
Grammar dependencies

AST_OWNER
SEMANTIC_OWNER
IR_OWNER
TEST_OWNER
SPEC_OWNER

AST contract
Semantic contract
Type contract
Effect contract
Capability contract
Resource contract
Contract contract
Policy contract
Provenance contract

Compiler integration
Runtime integration
Backend integration
Quantum boundary
HDL boundary
Dialect boundary

Diagnostics
Compatibility
Scalability
Determinism
Security

Positive tests
Negative tests
Boundary tests
Cross-domain tests
Scalability tests
Compatibility tests
Determinism tests

Completion criteria

This contract is required to support independent file completion.

---

61. No-Rework Completion Rule

A file is considered complete only when its integration boundaries are already known.

A feature specification MUST NOT contain unresolved references such as:

AST will be decided later
IR will be decided later
backend integration TBD
policy integration TBD
resource integration TBD

Instead, the file MUST identify the owner even when implementation is pending.

For example:

AST_OWNER: src/frontend/ast/
SEMANTIC_OWNER: semantic analysis
IR_OWNER: canonical IR
TEST_OWNER: grammar/tests/ai/

This allows implementation to proceed independently without repeatedly redesigning completed files.

---

62. Testing Requirements

Every integrated feature MUST have:

1. lexical tests where applicable;
2. parser tests;
3. AST tests;
4. semantic tests;
5. type tests;
6. effect tests;
7. capability tests;
8. resource tests;
9. contract tests;
10. policy tests;
11. provenance tests;
12. positive tests;
13. negative tests;
14. boundary tests;
15. scalability tests;
16. deterministic/reproducibility tests;
17. compatibility tests;
18. cross-domain tests.

---

63. Required Integration Test Programs

The existing integration programs MUST remain authoritative for their respective domains.

Extend the test suite with cases equivalent to:

reasoning
learning
adaptation
knowledge
contracts
provenance
policies
uncertainty
agents
neural-symbolic
sandbox
simulation
FFI
patterns
dependent types
QML
hybrid AI/quantum

The exact filenames MUST follow the existing repository naming conventions.

---

64. Mandatory Cross-Domain Test

At least one end-to-end test MUST combine:

reasoning
learning
adaptation
classical computation
tensor computation
quantum operation
measurement
AI agent
parallelism
resource requirements
capabilities
effects
contracts
provenance
policy
simulation
hardware intent

The test MUST verify:

source
 ↓
lexer
 ↓
parser
 ↓
AST
 ↓
semantic validation
 ↓
type analysis
 ↓
effect analysis
 ↓
capability analysis
 ↓
resource analysis
 ↓
contract analysis
 ↓
policy analysis
 ↓
provenance
 ↓
Classical IR
 ↓
quantum::ir
 ↓
execution planning

Where a stage is not yet implemented in the repository, the conformance status MUST explicitly say so rather than pretending the test is complete.

---

65. Scalability Tests

Scalability tests MUST use symbolic or generated sizes rather than universal constants.

Tests SHOULD cover:

- one element;
- small collections;
- dynamically sized collections;
- large generated collections;
- distributed data;
- large tensor shapes;
- large quantum register requirements;
- large hardware graphs;
- large actor sets;
- large distributed topologies.

The purpose is to prove that no artificial grammar-level capacity has been introduced.

---

66. Hard-Coding Audit

Every integrated feature MUST pass a hard-coding audit.

The audit MUST search for:

MAX_
hard-coded universal capacities
fixed device counts
fixed qubit counts
fixed node counts
fixed accelerator counts
fixed tensor rank ceilings
fixed network sizes
fixed agent counts

The audit MUST also detect semantically equivalent renamed constants.

A target-specific implementation limit is acceptable only when:

- it is target-specific;
- it is externally discoverable;
- it is documented;
- it produces a diagnostic;
- it does not alter the language's universal semantics.

---

67. Determinism

The following must be deterministic unless explicitly specified otherwise:

- lexical recognition;
- parsing;
- AST construction;
- structural validation;
- type checking;
- policy evaluation where policy semantics require deterministic results;
- canonical serialization;
- semantic hashing where provided;
- provenance identity where required.

Learning, randomness, distributed scheduling and hardware nondeterminism MUST be explicitly represented when they affect observable semantics.

---

68. Security

Integrated features MUST respect:

- capability boundaries;
- effect restrictions;
- sandbox policies;
- authorization;
- provenance;
- resource budgets;
- FFI restrictions;
- reflection restrictions;
- code-generation restrictions;
- adaptation authorization.

Learning MUST NOT automatically authorize mutation.

Reflection MUST NOT automatically authorize code generation.

FFI MUST NOT automatically authorize arbitrary native operations.

A capability MUST NOT be treated as permission.

---

69. Error Model

Diagnostics MUST distinguish:

lexical error
syntax error
AST/structural error
name-resolution error
type error
effect error
capability error
resource error
contract error
policy error
provenance error
IR error
lowering error
target compatibility error
runtime resource failure
runtime execution failure

Examples:

required capability unavailable

is different from:

insufficient runtime memory

and both differ from:

invalid syntax

No generic error may erase this distinction.

---

70. Target Failure Semantics

If a target cannot satisfy:

required capability
required resource
required precision
required timing
required topology
required effect policy
required contract

the implementation MUST:

1. reject the target;
2. choose an explicitly permitted alternative;
3. degrade only where the source permits degradation;
4. or request another target.

It MUST NOT silently change semantics.

---

71. Approximation

Approximation MUST be explicit.

For example:

prefer approximation <= error_bound;

may permit approximate realization if the specification defines the semantics.

Exact computation MUST NOT silently become approximate.

---

72. Future Extensions

Future computational domains MUST integrate through the same foundation.

A future domain should require only:

domain syntax
domain semantic model
domain operations
domain capabilities
domain resources
domain effects
domain contracts
domain policies
domain provenance
domain lowering
domain IR integration
tests

It MUST NOT require redesigning:

- the entire lexer;
- the entire AST;
- the entire resource system;
- the entire effect system;
- the entire policy system;
- the entire provenance system.

---

73. Required Specification Links

This specification MUST remain synchronized with:

grammar/DESIGN.md
grammar/README.md
grammar/grammar.md
grammar/Zamani-Grammar.md
grammar/specification/README.md
grammar/spec/resources.md
grammar/spec/effects.md
grammar/spec/provenance.md
grammar/spec/policies.md
grammar/spec/quantum.md
grammar/spec/ai.md
grammar/spec/hdl.md

It MUST also identify relevant implementation owners in:

grammar/ai/
grammar/types/
grammar/data/
grammar/effects/
grammar/resources/
grammar/validation/
grammar/security/
grammar/execution/
grammar/concurrency/
grammar/interoperability/
grammar/metaprogramming/
grammar/quantum/
grammar/hybrid/
grammar/hdl/
grammar/hardware/
grammar/distributed/
grammar/networking/
grammar/compatibility/
grammar/compile/
grammar/tests/

---

74. Specification-to-Implementation Matrix

Layer| Responsibility| Required integration
Specification| Meaning| This file + domain specifications
Lexer| Tokens| canonical lexer/token registry
Parser| Syntax| subordinate grammar + parser composition
AST| Structure| domain-neutral AST
Validation| Structure| validation subsystem
Types| Type meaning| type subsystem
Effects| Side effects| effect subsystem
Capabilities| Available operations| resource/capability subsystem
Resources| Feasibility| resource subsystem
Contracts| Obligations| validation subsystem
Policies| Authorization/selection| policy/security subsystem
Provenance| Traceability| provenance subsystem
Semantic model| Domain meaning| semantic analyzer
Classical IR| Classical realization boundary| canonical classical IR
Quantum IR| Quantum realization boundary| "quantum::ir"
Optimization| Meaning-preserving transformation| compiler
Lowering| Target transformation| compiler/backend
Routing| Physical/logical mapping| routing
Scheduling| Resource/time ordering| scheduler
Resilience| Recovery/degradation| execution/runtime
ZQN| Quantum noise/fault semantics| quantum stack
HAL| Target abstraction| hardware/runtime
Backend| Physical realization| target backend
Tests| Conformance| "grammar/tests/"

---

75. Production Readiness Gate

This integration MUST NOT be declared production-ready until:

- [ ] every feature has a normative owner;
- [ ] no duplicate subsystem exists;
- [ ] lexical ownership is defined;
- [ ] parser ownership is defined;
- [ ] AST mapping is defined;
- [ ] semantic mapping is defined;
- [ ] type interaction is defined;
- [ ] effect interaction is defined;
- [ ] capability interaction is defined;
- [ ] resource interaction is defined;
- [ ] contract interaction is defined;
- [ ] policy interaction is defined;
- [ ] provenance interaction is defined;
- [ ] canonical IR destination is defined;
- [ ] quantum features reach "quantum::ir";
- [ ] classical features reach canonical classical IR;
- [ ] compiler integration is defined;
- [ ] runtime integration is defined;
- [ ] backend integration is defined;
- [ ] diagnostics are defined;
- [ ] compatibility is defined;
- [ ] scalability is tested;
- [ ] deterministic behavior is tested;
- [ ] security behavior is tested;
- [ ] positive tests pass;
- [ ] negative tests pass;
- [ ] boundary tests pass;
- [ ] cross-domain tests pass;
- [ ] hard-coding audit passes;
- [ ] Rust 1.97.1 compatibility passes;
- [ ] Zamani-owned Rust contains no "unsafe";
- [ ] target failure does not silently change semantics;
- [ ] provenance is preserved where required;
- [ ] resource and capability negotiation is explicit;
- [ ] adaptation is policy-controlled;
- [ ] reflection is capability/effect-controlled;
- [ ] FFI is effect-controlled;
- [ ] simulation uses the same semantic model as execution;
- [ ] agents reuse existing concurrency semantics;
- [ ] no application-specific keyword explosion has entered the core language.

---

76. Definition of Done for This Specification

This file is complete when it provides an unambiguous contract for:

feature
  ↓
owner
  ↓
syntax
  ↓
AST
  ↓
semantics
  ↓
types
  ↓
effects
  ↓
capabilities
  ↓
resources
  ↓
contracts
  ↓
policies
  ↓
provenance
  ↓
canonical IR
  ↓
compiler
  ↓
runtime
  ↓
target
  ↓
tests

No downstream implementation should need to reinterpret this document merely because another domain file has changed.

A downstream file may implement its assigned contract.

If the contract itself changes, that is a specification change and MUST go through the repository's compatibility and conformance process.

---

77. Final Architectural Contract

The integrated capabilities are not a second language.

They become additional semantic capabilities of Zamani.

The final architecture is:

                         ZAMANI
                           │
                           ▼
                    Universal Core
                           │
          ┌────────────────┼────────────────┐
          │                │                │
        Types          Operations         Values
          │                │                │
          └────────────────┼────────────────┘
                           │
        ┌──────────────────┼──────────────────┐
        │                  │                  │
      Effects         Capabilities        Resources
        │                  │                  │
        └──────────────────┼──────────────────┘
                           │
                     Requirements
                           │
                      Constraints
                           │
                        Policies
                           │
                       Contracts
                           │
                        Evidence
                           │
                       Provenance
                           │
                        Decisions
                           │
                  Domain Semantic Model
                           │
       ┌──────────┬────────┼────────┬──────────┐
       │          │        │        │          │
   Classical   Quantum    HDL      AI       Data
       │          │        │        │          │
       └──────────┴────────┼────────┴──────────┘
                           │
                    Hybrid / Distributed /
                    Networking / Accelerators
                           │
                           ▼
                 Canonical Semantic IR
                           │
                ┌──────────┴──────────┐
                ▼                     ▼
          Classical IR          quantum::ir
                │                     │
                └──────────┬──────────┘
                           ▼
                     Optimization
                           ▼
                       Lowering
                           ▼
                Routing / Scheduling
                           ▼
                    Resilience / QEC
                           ▼
                           ZQN
                           ▼
                           HAL
                           ▼
                    Target Realization

The governing principle is:

«Zamani source describes computational intent and semantic obligations; compilation and execution determine how that intent is realized on available resources without silently changing its meaning.»

This is the mechanism by which Zamani can scale from the smallest practical computation to arbitrarily large feasible computation.

Physical limits remain physical limits.

Implementation limits remain implementation limits.

Resource availability remains resource availability.

None of those become artificial limits on the language itself.

The integrated reasoning, knowledge, learning, adaptation, uncertainty, evidence, explainability, contracts, policies, agents, simulation, interoperability, metaprogramming and related capabilities therefore strengthen Zamani's universal computational model while preserving:

- one language;
- one lexical authority;
- one parser architecture;
- one domain-neutral AST;
- one universal semantic foundation;
- one resource/capability model;
- one effect model;
- one policy model;
- one provenance model;
- one concurrency/actor architecture;
- one canonical quantum IR boundary;
- one target-independent compilation architecture;
- safe Rust;
- open-ended scalability;
- explicit target feasibility;
- semantic preservation;
- reproducibility;
- and POCO-REAF.
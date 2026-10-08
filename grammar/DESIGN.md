Zamani Grammar Architecture and Production Design

Path: "grammar/DESIGN.md"
Repository: "Benwellonedge28/Zamani"
Language: Zamani
Compiler/frontend: ZUTC / Zamani compiler
Rust edition: 2021
Minimum Rust version: Rust 1.97.1 or later
Rust safety: Safe Rust only; production Rust code MUST NOT use "unsafe"
Architecture: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)
Status: Normative architectural specification
Scope: Language grammar, lexical architecture, parser integration, AST integration, semantic ownership, IR boundaries, domain integration, portability, scalability, compatibility, validation, conformance, and production readiness.

---

1. Purpose

This document defines the normative architecture of the "grammar/" subsystem.

It establishes:

- what the grammar subsystem owns;
- what it does not own;
- which files are authoritative;
- how syntax is represented;
- how syntax integrates with the Rust frontend;
- how domain-neutral AST structures are produced;
- how semantic meaning is established;
- how types, effects, resources, capabilities, contracts, policies, and provenance interact;
- how classical, quantum, HDL, AI, data, distributed, networking, accelerator, and future computational domains integrate;
- how canonical IR is reached;
- how quantum computation crosses the "quantum::ir" boundary;
- how target independence is preserved;
- how programs scale from tiny computations to arbitrarily large feasible computations;
- how compatibility is maintained;
- how production conformance is established.

This document is an architectural contract.

It is not an inventory of every keyword, grammar production, AST field, instruction, hardware device, or backend.

A feature described here is not automatically implemented merely because the architecture permits it.

A feature MUST NOT be marked implemented or stable until the required implementation and conformance evidence exists.

---

2. Normative terminology

The following terms have mandatory meanings.

MUST

A mandatory requirement.

MUST NOT

A prohibited behavior.

SHOULD

A strong recommendation. A deviation requires documented justification.

SHOULD NOT

A behavior normally prohibited unless documented justification exists.

MAY

An allowed option.

Specification

The normative definition of language meaning.

Grammar

The formal syntactic representation accepted by a grammar implementation.

Lexer

The component that converts source characters into tokens.

Parser

The component that converts tokens into syntactic structures.

AST

The domain-neutral representation of source-level structure and meaning-bearing syntax.

Semantic model

The validated meaning of an AST independent of a particular physical target.

IR

An intermediate representation used after semantic analysis.

Canonical IR

The repository-defined target-independent intermediate representation used as the stable compiler boundary.

"quantum::ir"

The canonical quantum intermediate representation boundary. Quantum semantics MUST converge through this representation rather than creating competing quantum IRs.

Target

A compilation or execution environment.

Examples include CPU, GPU, FPGA, ASIC, accelerator, QPU, simulator, embedded device, HPC system, cluster, distributed environment, or future computational environment.

Capability

A property or operation an execution environment can provide.

Resource

A logical or physical quantity required or consumed by computation.

Requirement

A condition that must be satisfied for a declared program meaning or execution strategy.

Constraint

A restriction on an acceptable realization.

Preference

A non-mandatory selection preference.

Hint

Optimization information that MUST NOT change program meaning.

Policy

A set of rules governing allowed, required, preferred, prohibited, or fallback behavior.

Effect

A semantic description of observable or relevant computational consequences.

Contract

A machine-checkable semantic obligation such as a precondition, postcondition, invariant, assumption, guarantee, property, or assertion.

Provenance

Information describing the origin, transformation, evidence, derivation, verification, or decision history of an artifact or semantic fact.

Realization

The concrete mapping of portable program intent onto a target.

Dialect

An explicitly registered extension or external syntax domain that maps into Zamani semantic structures.

Conformance

Evidence that an implementation satisfies its applicable normative contracts.

Production-ready

A feature whose specification, syntax, AST, semantics, diagnostics, integration, tests, compatibility, and required downstream behavior are complete and validated.

---

3. Fundamental architectural principles

The "grammar/" subsystem MUST follow these principles.

3.1 One language

Zamani is one programming language.

Classical computation, quantum computation, HDL, AI, data, distributed computation, networking, accelerators, and future domains are domains of the same language.

They MUST NOT become unrelated languages sharing a repository.

---

3.2 One architectural authority

There MUST be one architectural authority for the grammar subsystem:

"grammar/DESIGN.md"

There MUST NOT be competing architecture documents with equal authority.

---

3.3 One canonical language specification

Normative language meaning belongs under:

"grammar/specification/"

Machine-oriented contracts, schemas, conformance metadata, and validation definitions belong under:

"grammar/spec/"

The two MUST agree.

When a contradiction exists, it MUST be resolved explicitly rather than silently choosing whichever file was edited last.

---

3.4 One canonical grammar composition root

"grammar/Zamani.g4" is the canonical ANTLR grammar composition root.

It MUST remain a composition root rather than becoming a second semantic specification.

It SHOULD contain only the universal root dispatch and imports/delegation necessary to compose the grammar.

Domain-specific productions belong to their designated grammar owners.

---

3.5 One lexical authority

"grammar/antlr/ZamaniLexer.g4" is the canonical ANTLR lexical authority.

"grammar/lexer/" owns the lexical contracts and registries.

The Rust implementation in "src/lexer.rs" is the executable frontend lexical implementation.

These representations MUST be reconciled through shared conformance tests.

Neither the ANTLR lexer nor Rust lexer may silently define a different language.

---

3.6 One semantic model

Different domains MAY have domain-specific semantic models, but they MUST share universal semantic foundations:

- names;
- types;
- values;
- operations;
- effects;
- capabilities;
- resources;
- requirements;
- constraints;
- contracts;
- policies;
- provenance;
- diagnostics;
- compatibility.

---

3.7 One canonical IR architecture

The compiler MUST have a canonical target-independent IR boundary.

Classical computation MUST converge into the canonical classical IR.

Quantum computation MUST converge through:

"quantum::ir"

Domain-specific IRs MAY exist for lowering or optimization, but they MUST NOT replace the canonical semantic boundary.

---

3.8 Grammar is not hardware realization

The grammar describes program meaning and portable intent.

It MUST NOT hard-code a particular physical machine universe.

Hardware realization belongs downstream.

---

3.9 Open-world scalability

The architecture MUST be open-ended.

Adding:

- a new processor;
- a new accelerator;
- a new QPU;
- a new FPGA family;
- a new tensor engine;
- a new network topology;
- a new quantum operation;
- a new AI model;
- a new data format;
- a new computational paradigm

MUST NOT require rewriting the universal language architecture merely because the new target or operation did not previously exist.

---

3.10 Safe Rust

The reference implementation MUST use safe Rust.

Production Rust code MUST NOT use:

- "unsafe";
- unsafe blocks;
- unsafe traits;
- unsafe implementations;
- FFI implemented through unsafe Rust without an approved safe abstraction boundary.

Where external systems require unsafe primitives internally, the project architecture MUST isolate that concern outside the safe Rust language implementation and expose only a validated safe interface to the compiler.

---

4. Existing repository integration baseline

This architecture is designed against the existing repository rather than an assumed empty project.

The repository contains major components including:

- "grammar/Zamani.g4";
- "grammar/Zamani-Grammar.md";
- "grammar/grammar.md";
- "grammar/README.md";
- "grammar/lexer/";
- "grammar/antlr/";
- "grammar/core/";
- "grammar/classical/";
- "grammar/quantum/";
- "grammar/hdl/";
- "grammar/ai/";
- "grammar/data/";
- "grammar/distributed/";
- "grammar/networking/";
- "grammar/effects/";
- "grammar/resources/";
- "grammar/execution/";
- "grammar/interoperability/";
- "grammar/metaprogramming/";
- "grammar/concurrency/";
- "grammar/security/";
- "grammar/validation/";
- "grammar/dialects/";
- "grammar/hybrid/";
- "grammar/hardware/";
- "grammar/memory/";
- "grammar/modules/";
- "grammar/types/";
- "grammar/statements/";
- "grammar/expressions/";
- "grammar/tests/";
- Rust frontend implementation under "src/".

The current Rust frontend includes its own lexer and parser behavior.

Therefore:

«Adding an ANTLR production does not, by itself, implement a Zamani feature.»

Every new feature MUST be traced across the complete required implementation path.

---

5. Rust implementation baseline

The minimum supported Rust toolchain is:

Rust 1.97.1

Newer stable Rust versions MAY be supported.

The repository SHOULD continuously test at least:

- minimum supported Rust version;
- current stable Rust;
- supported CI toolchains.

The language implementation MUST use Rust 2021.

Production implementation MUST compile without Rust "unsafe".

Dependency additions MUST NOT silently introduce an architecture that requires unsafe Rust in the Zamani compiler itself.

---

6. POCO-REAF

POCO-REAF means:

Program Once, Compile Once, Run Everywhere, Anywhere, Forever.

This is an architectural goal concerning preservation of program meaning across targets and scales.

It does not mean that one target-specific binary can physically execute on every architecture.

The architecture distinguishes:

source portability
        ↓
semantic portability
        ↓
canonical IR portability
        ↓
target realization
        ↓
target executable

A program SHOULD be written once and retain its semantic meaning across:

- tiny devices;
- embedded systems;
- CPUs;
- multicore systems;
- GPUs;
- FPGAs;
- ASICs;
- accelerators;
- QPUs;
- simulators;
- HPC systems;
- clusters;
- distributed environments;
- cloud environments;
- future computational targets.

A target that cannot satisfy the program's requirements MUST NOT silently change its meaning.

The compiler/runtime MUST instead report the unsatisfied condition or select an explicitly authorized alternative.

---

7. Validity is layered

The implementation MUST distinguish:

Lexically valid
      ≠
Syntactically valid
      ≠
Structurally valid
      ≠
Type valid
      ≠
Effect valid
      ≠
Contract valid
      ≠
Policy valid
      ≠
Resource valid
      ≠
Capability valid
      ≠
Semantically valid
      ≠
Compilable
      ≠
Target-compatible
      ≠
Resource-feasible
      ≠
Executable
      ≠
Successfully executed

Diagnostics MUST identify the layer at which a failure occurs.

For example:

- malformed source is a lexical/syntactic error;
- incompatible types are semantic/type errors;
- forbidden network access is an effect/policy error;
- unavailable QPU measurement is a capability error;
- insufficient memory is a resource-feasibility error;
- unsupported target lowering is a backend error.

These MUST NOT be collapsed into a generic "compile failed" state.

---

8. Scalability model

Zamani MUST NOT define arbitrary universal hardware ceilings.

The architecture MUST NOT contain universal limits such as:

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
MAX_NETWORK_SIZE
MAX_DEVICE_COUNT
MAX_ACCELERATOR_COUNT
MAX_AGENT_COUNT
MAX_GATE_COUNT

Nor may equivalent renamed constants be introduced.

The prohibition applies to:

- grammar;
- AST;
- semantic models;
- compiler configuration;
- universal resource registries;
- IR;
- target-independent specifications.

---

8.1 Actual limits

Physical systems and software implementations necessarily have finite resources.

Therefore Zamani MUST distinguish:

language expressiveness

from:

implementation representation capacity

and:

available target resources

An implementation limit MUST:

- be documented;
- be detectable;
- produce a deterministic diagnostic where applicable;
- never be disguised as a language-wide semantic limit;
- never silently corrupt program meaning.

---

8.2 Symbolic resource requirements

The preferred resource model is symbolic.

Examples:

requires qubits >= required_qubits;
requires memory >= required_memory;
requires capability("quantum.measurement");
requires capability("gpu.compute");
requires capability("tensor.compute");
requires topology(required_topology);

The identifiers and expressions are semantic requirements.

They are not hard-coded machine capacities.

---

8.3 Scaling

A program SHOULD scale through:

- symbolic dimensions;
- runtime values;
- generic types;
- resource negotiation;
- target discovery;
- capability negotiation;
- decomposition;
- parallelization;
- distribution;
- scheduling;
- routing;
- specialization;
- runtime allocation.

The source program MUST NOT need to know the physical size of the target unless the programmer deliberately requests target-specific behavior.

---

9. Separation between intent and realization

The source language describes:

- values;
- types;
- computation;
- relationships;
- contracts;
- effects;
- requirements;
- constraints;
- capabilities;
- policies;
- preferences;
- hints;
- provenance.

The compiler/runtime determines:

- physical placement;
- scheduling;
- routing;
- decomposition;
- memory allocation;
- accelerator selection;
- QPU mapping;
- hardware-specific instructions;
- distributed placement;
- execution strategy.

The grammar MUST NOT own physical realization.

---

10. Universal resource model

The following concepts MUST remain semantically distinct.

10.1 Requirement

A requirement states something necessary for the intended computation or execution mode.

Example:

requires qubits >= required_qubits;

---

10.2 Capability

A capability identifies something an environment can provide.

Example:

requires capability("quantum.measurement");
requires capability("tensor.compute");

Capability identifiers MUST be extensible.

They MUST NOT require a universal enumeration of every future device.

---

10.3 Constraint

A constraint restricts acceptable realization.

Example:

requires latency <= latency_budget;

Units and comparison semantics MUST be defined by the owning specification.

---

10.4 Preference

A preference influences selection without becoming a correctness requirement.

Example conceptual intent:

prefer capability("accelerator.compute");

---

10.5 Hint

A hint may improve optimization but MUST NOT change program meaning.

---

10.6 Budget

A budget bounds permitted consumption for a particular compilation or execution context.

A budget MUST NOT be confused with a universal machine capacity.

---

10.7 Negotiation

Negotiation determines whether a target or execution environment can satisfy requirements and policies.

---

10.8 Realization

Realization maps logical intent onto actual resources.

The grammar expresses the requirements; downstream compilation and execution resolve them.

---

11. Universal effect model

Effects describe semantically relevant consequences.

The effect system MUST be shared across all domains.

Applicable effect categories include:

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

Additional effects MAY be registered.

An operation such as learning, adaptation, measurement, reflection, simulation, or foreign execution MUST participate in effect checking where its semantics require it.

Effects MUST NOT be implemented as merely descriptive comments.

---

12. Contracts

Contracts are universal semantic obligations.

They include:

- "requires";
- "ensures";
- "invariant";
- "assume";
- "guarantee";
- "property";
- assertions;
- refinement conditions;
- verification obligations.

Contracts MAY apply to:

- ordinary functions;
- quantum operations;
- hardware descriptions;
- AI models;
- actors;
- distributed services;
- resources;
- data transformations;
- compilation transformations.

Contracts MUST survive semantic lowering whenever their meaning requires preservation.

---

13. Policies

Policies govern what is:

- required;
- allowed;
- preferred;
- prohibited;
- authorized;
- restricted;
- recoverable;
- selectable as fallback.

Policies MUST be separate from raw syntax.

They MAY govern:

- security;
- resource allocation;
- execution;
- adaptation;
- simulation;
- target selection;
- deployment;
- quantum execution;
- distributed execution;
- FFI;
- reflection;
- generated code.

Policy evaluation MUST produce diagnostics or decisions that can be represented in provenance where required.

---

14. Provenance

Provenance is a cross-domain facility.

It MUST be capable of recording, where applicable:

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

Provenance MAY be attached to:

- source declarations;
- inferred facts;
- AI decisions;
- compiler transformations;
- resource decisions;
- target selection;
- quantum routing;
- optimization;
- HDL synthesis;
- security decisions;
- simulation results.

Provenance MUST NOT require every implementation to retain unlimited history in memory.

Storage strategy is an implementation concern.

Semantic provenance requirements MUST remain explicit.

---

15. Evidence and explainability

Evidence is a semantic concept independent of any specific AI system.

An evidence record MAY contain:

- claim;
- supporting evidence;
- source;
- confidence;
- derivation;
- verification status;
- provenance.

Explainability MAY be used for:

- AI decisions;
- compiler transformations;
- optimization;
- target selection;
- resource negotiation;
- quantum routing;
- hardware placement;
- security decisions.

The grammar MUST NOT require a particular explanation algorithm.

---

16. Controlled reasoning

Zamani MAY provide generic reasoning constructs including concepts corresponding to:

- inference;
- deduction;
- induction;
- abduction;
- causal reasoning;
- counterfactual reasoning;
- evidence-based conclusion.

These are language capabilities, not application-specific language universes.

Reasoning MUST integrate with:

- types;
- effects;
- evidence;
- provenance;
- policies;
- contracts;
- canonical semantic representation.

---

17. Knowledge model

Knowledge constructs MAY support:

- assertions;
- retractions;
- queries;
- relations;
- derived facts;
- metadata;
- provenance.

The knowledge model MUST be generic enough to support:

- AI knowledge;
- graph data;
- scientific data;
- configuration;
- compiler facts;
- hardware capability facts;
- security facts.

The core grammar MUST NOT require an application-specific knowledge vocabulary.

---

18. Learning

Learning is a semantic operation.

A learning construct SHOULD be able to express:

- input;
- target;
- data;
- model;
- objective;
- resources;
- capabilities;
- effects;
- policy;
- provenance.

Algorithms MUST NOT be permanently enumerated in the universal grammar.

New learning algorithms SHOULD be introduced through libraries, semantic registries, dialects, metadata, or implementations rather than requiring universal grammar modification.

---

19. Controlled adaptation

Adaptation is permitted only under explicit semantic control.

Adaptation MUST NOT mean unrestricted self-modifying code.

An adaptation operation MUST participate in applicable:

- authorization;
- policy;
- capabilities;
- effects;
- resource requirements;
- contracts;
- provenance.

The conceptual execution sequence is:

adaptation request
       ↓
policy evaluation
       ↓
authorization
       ↓
capability check
       ↓
effect check
       ↓
resource check
       ↓
state/model/strategy update
       ↓
validation
       ↓
provenance
       ↓
continued execution

Unauthorized code generation or arbitrary executable mutation MUST be rejected.

---

20. Pattern matching and guards

Pattern matching is a universal language feature.

It MUST support, where specified:

- structural patterns;
- value patterns;
- type patterns;
- destructuring;
- guarded patterns;
- nested patterns;
- domain-owned patterns.

Patterns MUST integrate with:

- type checking;
- exhaustiveness analysis where applicable;
- control flow;
- ownership/borrowing semantics where applicable;
- contracts.

Domain grammars MUST NOT create competing pattern languages.

---

21. Type-system architecture

The type system MUST be target-independent.

The architecture MAY support:

High priority

- generics;
- generic bounds;
- associated types;
- type constraints;
- tuples;
- records;
- sum types;
- option/result types;
- linear types;
- affine types;
- pattern-oriented types.

Later or advanced capabilities

- dependent types;
- type classes;
- existential types;
- variance;
- higher-kinded types;
- type-level computation;
- type families;
- other advanced generic mechanisms.

The type system MUST NOT encode fixed physical capacities as universal type limits.

---

22. Uncertainty and probability

Uncertainty MUST be representable without forcing one implementation strategy.

Concepts MAY include:

- uncertain values;
- probability;
- distributions;
- confidence;
- belief;
- stochastic computation.

The semantic layer MUST distinguish:

probabilistic meaning

from:

one particular probabilistic runtime implementation

---

23. Classical computation

The classical subsystem provides ordinary computational semantics.

It MUST integrate with the universal foundations:

- types;
- expressions;
- statements;
- functions;
- memory;
- effects;
- resources;
- capabilities;
- contracts;
- policies;
- provenance.

Classical IR is the canonical representation for classical computational meaning before target-specific lowering.

---

24. Quantum computation

The quantum subsystem MUST be a first-class domain of the language.

It MUST integrate with:

- types;
- effects;
- resources;
- capabilities;
- contracts;
- policies;
- provenance;
- hybrid execution;
- classical control.

Quantum semantics MUST remain target-independent until target realization.

---

25. Quantum operation extensibility

Quantum operations MUST be data-driven.

The universal grammar MUST NOT depend on a permanently enumerated list such as:

H
X
Y
Z
CNOT

Instead, the grammar SHOULD represent the semantic structure of an operation through concepts such as:

operationSpecifier
targets
parameters
results
attributes
modifiers

The operation specifier MAY identify:

- built-in operation;
- registered operation;
- custom operation;
- vendor operation;
- parameterized operation;
- dialect operation.

Adding a new quantum operation MUST NOT require changing the universal grammar when the existing operation representation can express it.

---

26. "quantum::ir" boundary

All quantum semantic operations that proceed toward compilation MUST have a defined path into:

quantum::ir

There MUST NOT be multiple competing canonical quantum IRs.

Domain-specific intermediate forms MAY exist temporarily, but they MUST have explicit lowering into "quantum::ir".

The quantum pipeline is conceptually:

Zamani quantum syntax
        ↓
domain-neutral AST
        ↓
quantum semantic validation
        ↓
quantum semantic model
        ↓
quantum::ir
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
HAL / target realization

---

27. Quantum resources and capabilities

Quantum programs MUST describe logical requirements rather than universal physical ceilings.

Examples include:

requires qubits >= required_qubits;
requires capability("quantum.measurement");
requires capability("quantum.dynamic_circuit");
requires capability("quantum.error_correction");
requires topology(required_topology);

The grammar MUST NOT determine the maximum number of physical qubits supported by the language.

The target determines actual availability.

---

28. Quantum error correction and resilience

Quantum error correction belongs downstream of quantum semantic representation.

The architecture MUST permit:

- error models;
- resilience requirements;
- QEC strategy;
- logical-to-physical mapping;
- distance requirements;
- recovery;
- fault-aware scheduling.

QEC-specific implementation limits MUST NOT become universal language limits.

---

29. Hybrid computation

Hybrid computation is a first-class integration boundary.

The architecture MUST support interactions such as:

classical → quantum
quantum → classical
measurement → classical decision
classical control → quantum operation
AI → quantum
quantum → AI
CPU → accelerator
host → device

Hybrid semantics MUST converge into shared semantic and IR structures.

A separate hybrid language MUST NOT be created.

---

30. Hardware description

HDL is a semantic domain of Zamani.

The HDL architecture MUST support, where specified:

- signals;
- state;
- combinational behavior;
- sequential behavior;
- timing;
- clocks;
- interfaces;
- memory;
- hardware intent;
- verification;
- simulation;
- synthesis.

HDL source MUST describe hardware intent rather than impose universal hardware dimensions.

Physical implementation belongs downstream.

---

31. Hardware/software co-design

Hardware and software MUST be able to share:

- types;
- interfaces;
- resources;
- capabilities;
- contracts;
- effects;
- provenance.

A hardware implementation MAY expose capabilities consumed by software.

A software program MAY express hardware requirements.

The compiler determines the realization.

---

32. AI and intelligent computation

AI is a semantic domain, not a replacement language.

AI capabilities MAY include:

- inference;
- deduction;
- induction;
- abduction;
- learning;
- adaptation;
- uncertainty;
- knowledge;
- evidence;
- explanation;
- planning;
- agents;
- neural-symbolic computation;
- probabilistic computation.

AI constructs MUST reuse universal Zamani foundations.

---

33. Agents and concurrency

AI agents MUST integrate with the existing concurrency architecture.

The architecture is:

AI agent
   ↓
actor
   ↓
message
   ↓
channel
   ↓
task
   ↓
scheduler

The AI subsystem MUST NOT create a second actor model.

"grammar/concurrency/actors.g4" remains the owner of general actor syntax and lifecycle semantics.

AI-specific semantics describe agent behavior while reusing the common actor infrastructure.

---

34. Distributed computation

Distributed computation MUST integrate with:

- concurrency;
- actors;
- messages;
- channels;
- topology;
- resource requirements;
- capabilities;
- policies;
- resilience;
- provenance.

No universal maximum node count may exist.

---

35. Networking

Networking MUST be represented through explicit effects and capabilities.

Network operations SHOULD participate in:

effect(network)

and applicable:

capability("network.*")

requirements.

Networking MUST integrate with:

- security;
- policies;
- effects;
- distributed computation;
- provenance;
- interoperability.

---

36. Data computation

Data capabilities MAY include:

- structured data;
- tensors;
- arrays;
- graphs;
- datasets;
- schemas;
- queries;
- streaming;
- provenance;
- uncertainty.

Data representations MUST remain independent of a fixed maximum size.

---

37. External data languages and formats

SQL, JSON, XML, and similar formats MUST NOT become universal Zamani grammar requirements merely because the language interoperates with them.

They belong under appropriate:

grammar/dialects/

or:

grammar/interoperability/

depending on ownership.

Their semantic output SHOULD map into common Zamani data/query/interoperability models.

---

38. Interoperability

FFI and ABI are explicit interoperability boundaries.

The interoperability subsystem SHOULD own:

ffi.g4
abi.g4
foreign.g4
calling-convention.g4
data-layout.g4
linkage.g4
external-functions.g4
external-types.g4

Foreign calls MUST participate in:

- effects;
- capabilities;
- policy;
- type compatibility;
- data-layout validation;
- provenance.

Rust implementation boundaries MUST remain safe.

---

39. Metaprogramming

Metaprogramming MAY include:

- reflection;
- introspection;
- compile-time evaluation;
- syntax trees;
- quotation;
- generation;
- type-level computation.

Reflection MUST NOT automatically grant arbitrary runtime mutation.

Code generation and executable generation MUST participate in:

- capabilities;
- effects;
- policy;
- provenance;
- validation.

---

40. Simulation

Simulation is an execution strategy, not a separate programming language.

The architecture MUST support simulation of:

- classical systems;
- quantum systems;
- hardware;
- distributed systems;
- AI systems;
- performance;
- fault conditions;
- execution strategies.

Simulation SHOULD preserve the same semantic model used by actual execution.

---

41. Adaptive execution

Adaptive execution MAY:

- inspect target conditions;
- select among permitted strategies;
- route differently;
- schedule differently;
- retry;
- recover;
- degrade where explicitly permitted;
- migrate;
- select simulation;
- choose alternative implementations.

Adaptive execution MUST NOT silently change semantic guarantees.

It MUST respect policies and contracts.

---

42. Resilience

The architecture recognizes execution states such as:

Unknown
Healthy
Degraded
Unstable
Unavailable
Recovering
Quarantined
Retired

and outcomes such as:

ACCEPT
DEGRADED_ACCEPT
RETRY
RECOVER
ESCALATE
REJECT

These are execution semantics, not grammar-level hardware limits.

Resilience mechanisms MUST preserve source-level meaning wherever possible.

---

43. Deterministic and reproducible execution

The language/toolchain SHOULD support reproducible compilation and execution.

Where deterministic behavior is requested, the implementation MUST identify relevant sources of nondeterminism, including:

- random seeds;
- scheduling;
- distributed ordering;
- target selection;
- optimization choices;
- simulation state;
- external data;
- timestamps;
- environment-dependent behavior.

Reproducibility metadata SHOULD participate in provenance.

---

44. Security and sandboxing

Sandboxing is a security/execution concern.

The architecture MAY constrain:

- filesystem effects;
- network effects;
- native execution;
- foreign calls;
- reflection;
- code generation;
- adaptation;
- resources;
- capabilities.

A sandbox policy MUST be represented independently from ordinary application semantics.

Example conceptual policy:

sandbox {
    forbid effect("network");
    forbid capability("native.execute");
}

The exact syntax is owned by the security/policy specifications.

---

45. Application-specific functionality

The core language MUST remain universal.

Application concepts such as:

- computer vision;
- sentiment analysis;
- robotics;
- payments;
- administration;
- legal workflows;
- blockchain applications;
- VR/AR;
- specialized AI taxonomies;
- future application-specific systems

MUST NOT become universal keywords merely because Zamani can support them.

They SHOULD be implemented through:

- libraries;
- packages;
- dialects;
- capabilities;
- policies;
- services;
- domain models;
- applications.

This keeps the core language extensible instead of continually expanding the keyword universe.

---

46. Lexer architecture

The lexical architecture consists of:

grammar/lexer/
grammar/antlr/ZamaniLexer.g4
src/lexer.rs

"grammar/lexer/" defines lexical contracts.

"grammar/antlr/ZamaniLexer.g4" defines ANTLR lexical behavior.

"src/lexer.rs" implements current Rust lexical behavior.

They MUST share conformance tests.

---

47. Token registry

"grammar/lexer/tokens.g4" or the repository's designated token registry MUST define the canonical token vocabulary.

Each token SHOULD have metadata describing:

- name;
- spelling;
- category;
- owner;
- reserved status;
- lexical context;
- compatibility;
- tests.

A keyword MUST NOT be added solely because an external language used that keyword.

If ordinary identifiers are sufficient, identifiers SHOULD remain identifiers.

This prevents application-specific keyword explosion.

---

48. Lexer/Rust parser reconciliation

The current Rust lexer/parser implementation is an independent executable frontend.

Therefore every grammar change MUST answer:

Does ANTLR accept it?
Does Rust lexer recognize it?
Does Rust parser parse it?
Does the AST represent it?
Does semantic analysis understand it?
Does IR represent it?
Are tests present?

No feature is complete until the applicable answers are yes.

---

49. AST architecture

The AST MUST be domain-neutral at its foundation.

It MUST preserve:

- source locations;
- declarations;
- expressions;
- statements;
- types;
- names;
- attributes;
- modifiers;
- domain-specific meaning where necessary;
- diagnostics context.

It MUST NOT directly embed:

- physical CPU objects;
- GPU vendor objects;
- QPU device handles;
- LLVM implementation objects;
- hardware driver handles;
- backend-specific scheduling structures.

Target realization belongs downstream.

---

50. AST ownership

"src/ast/mod.rs" is the current Rust AST integration authority.

Every new syntax feature MUST define before implementation:

AST node or representation
source span behavior
child nodes
attributes
modifiers
semantic payload
error recovery behavior
compatibility representation

AST changes MUST be accompanied by semantic and test integration plans.

---

51. Parser architecture

"src/parser.rs" is the current executable parser.

The parser MUST:

- implement normative syntax;
- preserve precedence;
- preserve associativity;
- preserve source spans;
- produce deterministic diagnostics;
- reject invalid structures;
- avoid silently accepting unsupported features;
- integrate new grammar constructs through explicit parser rules.

The ANTLR grammar and Rust parser MUST be tested against shared syntax fixtures.

---

52. Canonical compiler pipeline

The production pipeline is:

Zamani Source
      │
      ▼
Source Management
      │
      ▼
Lexer
      │
      ▼
Token Stream
      │
      ▼
Parser
      │
      ▼
Domain-Neutral AST
      │
      ▼
Structural Validation
      │
      ├── Names
      ├── Modules
      ├── Types
      ├── Effects
      ├── Capabilities
      ├── Resources
      ├── Contracts
      ├── Policies
      └── Provenance
      │
      ▼
Semantic Analysis
      │
      ▼
Canonical Semantic Model
      │
      ├───────────────┬────────────────┐
      ▼               ▼                ▼
 Classical        Quantum            HDL
 Semantics        Semantics          Intent
      │               │                │
      ▼               ▼                ▼
Classical IR     quantum::ir       Hardware IR
      │               │                │
      └───────────────┼────────────────┘
                      ▼
                Optimization
                      │
                      ▼
                   Lowering
                      │
                      ▼
             Routing / Scheduling
                      │
                      ▼
             Resilience / Recovery
                      │
                      ▼
                    ZQN
                      │
                      ▼
                     HAL
                      │
                      ▼
             Target Realization

Not every program needs every stage.

Not every stage belongs physically inside "grammar/".

"grammar/" defines the frontend architectural contracts and language syntax.

---

53. Domain integration model

All major domains MUST integrate through common foundations.

                    Universal Zamani Core
                            │
       ┌────────────────────┼────────────────────┐
       │                    │                    │
     Types                Effects             Values
       │                    │                    │
       ├──────────────┬─────┴─────┬──────────────┤
       │              │           │              │
 Resources       Capabilities  Contracts      Policies
       │              │           │              │
       └──────────────┴─────┬─────┴──────────────┘
                            │
                       Provenance
                            │
                     Semantic Model
                            │
       ┌──────────┬─────────┼──────────┬───────────┐
       │          │         │          │           │
   Classical   Quantum     HDL        AI        Distributed
       │          │         │          │           │
       └──────────┴─────────┼──────────┴───────────┘
                            │
                       Canonical IR

No domain may bypass universal semantic validation merely because it has specialized syntax.

---

54. File ownership model

Every grammar file MUST have an explicit owner.

A feature file MUST document:

Purpose
Owns
Does Not Own
Dependencies
Exports
Lexer Dependencies
Grammar Dependencies
AST Contract
Semantic Contract
Type Contract
Effect Contract
Capability Contract
Resource Contract
Contract Integration
Policy Integration
Provenance Integration
IR Contract
Backend Boundary
Diagnostics
Positive Tests
Negative Tests
Boundary Tests
Scalability Tests
Compatibility
Integration
Completion Criteria

This contract MUST be established before implementation is considered complete.

---

55. File dependency contract

Every significant feature file SHOULD declare:

DEPENDS_ON:
EXPORTS:
CONSUMED_BY:
AST_OWNER:
SEMANTIC_OWNER:
IR_OWNER:
SPEC_OWNER:
TEST_OWNER:
COMPATIBILITY_OWNER:

Example:

DEPENDS_ON:
    lexer/tokens.g4
    quantum/operations.g4
    expressions/expressions.g4

EXPORTS:
    quantumMeasurement

AST_OWNER:
    src/ast/...

SEMANTIC_OWNER:
    quantum semantic layer

IR_OWNER:
    quantum::ir

SPEC_OWNER:
    specification/quantum.md

TEST_OWNER:
    tests/quantum/measurement/

The exact paths MUST be adjusted to the repository's final ownership structure.

---

56. Independent-file-first implementation rule

Features MUST be implemented in dependency order.

A file is considered complete only when its own contract is satisfied.

Later files MUST consume its published interface rather than requiring undocumented assumptions.

A completed file SHOULD NOT require reopening merely because another downstream feature was subsequently added.

If a downstream feature genuinely requires changing an established interface, that is an explicit architectural change and MUST be treated as such.

---

57. Specification authority

"grammar/specification/" is the normative human-readable language specification.

It owns:

- language meaning;
- syntax contracts;
- semantic contracts;
- domain semantics;
- compatibility rules;
- normative examples where applicable.

It MUST NOT merely copy implementation source code.

---

58. Machine contracts

"grammar/spec/" is the machine-oriented contract layer.

It MAY contain:

- schemas;
- feature metadata;
- conformance metadata;
- ownership metadata;
- compatibility declarations;
- generated validation information.

Machine contracts MUST be derived from or consistent with normative specifications.

---

59. "grammar/Zamani-Grammar.md"

This document is not the final authority for language law.

It MAY preserve:

- historical grammar material;
- extended proposals;
- experimental constructs;
- explanatory material;
- compatibility references.

Features MUST be classified as:

stable
proposed
experimental
deprecated
historical
not implemented

A construct appearing there does not automatically become valid Zamani syntax.

---

60. "grammar/grammar.md"

This document records implementation/conformance status.

Feature status SHOULD distinguish:

SPECIFIED
LEXER_IMPLEMENTED
PARSER_IMPLEMENTED
AST_IMPLEMENTED
STRUCTURAL_VALIDATION_IMPLEMENTED
SEMANTIC_IMPLEMENTED
TYPE_IMPLEMENTED
EFFECT_IMPLEMENTED
RESOURCE_IMPLEMENTED
IR_IMPLEMENTED
BACKEND_IMPLEMENTED
TESTED
STABLE
EXPERIMENTAL
DEPRECATED

Status MUST be evidence-based.

A feature MUST NOT be marked implemented because documentation exists.

---

61. Generated artifacts

Generated grammar files MUST have clear ownership.

Generated output MUST NOT become a second source of truth.

The architecture MUST distinguish:

source specification
      ↓
source grammar
      ↓
generated artifact

Generated artifacts MUST be reproducible.

Manual edits to generated files MUST be prohibited unless the repository explicitly defines them as source files.

---

62. Diagnostics architecture

Diagnostics MUST be deterministic and structured.

A diagnostic SHOULD identify:

- severity;
- error code;
- source span;
- primary message;
- related locations;
- cause;
- semantic layer;
- suggested correction where available;
- provenance/context where appropriate.

Diagnostics MUST distinguish:

- syntax errors;
- type errors;
- effect violations;
- capability failures;
- resource failures;
- contract failures;
- policy violations;
- unsupported target realization.

---

63. Error recovery

Parser error recovery MUST preserve as much useful source structure as safely possible.

Recovery MUST NOT:

- fabricate valid semantics;
- silently skip security-critical constructs;
- convert unsupported constructs into unrelated constructs;
- produce an AST that appears valid when the source is not valid.

---

64. Compatibility

Language compatibility MUST be explicit.

Compatibility applies to:

- lexical syntax;
- parser syntax;
- AST;
- semantic representation;
- IR;
- dialects;
- capabilities;
- target interfaces;
- generated artifacts.

A compatibility change MUST define:

- old behavior;
- new behavior;
- migration;
- diagnostics;
- deprecation period where appropriate.

---

65. Versioning

The architecture distinguishes:

language version
grammar version
lexer version
AST version
semantic model version
IR version
quantum::ir version
dialect version
capability schema version
target interface version

These MUST NOT be conflated.

Version metadata SHOULD participate in provenance.

---

66. Determinism

Grammar processing MUST be deterministic.

Given identical:

- source;
- language version;
- grammar version;
- compiler configuration;
- relevant external inputs;

the frontend SHOULD produce equivalent lexical, syntactic, and semantic results.

Maps and registries MUST NOT introduce observable nondeterministic ordering where ordering affects semantics or generated output.

---

67. Resource safety

Resource exhaustion MUST be handled explicitly.

The compiler MUST NOT assume that:

- source files are small;
- ASTs fit in a fixed universal size;
- tensors have a fixed rank;
- quantum programs have a fixed qubit count;
- distributed programs have a fixed node count.

Implementation resource limits MAY exist, but they MUST be configurable where appropriate and diagnostically visible.

---

68. Security boundaries

Security-sensitive operations MUST have explicit boundaries.

Particularly sensitive capabilities include:

- native execution;
- FFI;
- filesystem access;
- network access;
- reflection;
- code generation;
- adaptation;
- external process execution.

Such operations MUST participate in applicable:

effects
capabilities
policies
authorization
provenance

---

69. No unrestricted self-modification

The language MUST NOT provide unrestricted executable self-modification merely through adaptation, reflection, metaprogramming, or learning constructs.

Any runtime modification of executable behavior MUST have an explicit semantic model and applicable authorization/policy requirements.

---

70. Dialect architecture

Dialects allow domain-specific syntax without polluting the universal grammar.

A dialect MUST define:

- identity;
- version;
- syntax;
- lexer requirements;
- parser entry point;
- semantic mapping;
- capability requirements;
- effects;
- resource implications;
- provenance;
- compatibility;
- registration.

A dialect MUST lower into common Zamani semantic structures.

---

71. Extension architecture

Future functionality SHOULD prefer:

library
dialect
capability
policy
metadata
semantic registry

over adding a new universal keyword.

A universal keyword requires justification that:

1. it expresses a language-wide concept;
2. ordinary identifiers are insufficient;
3. contextual syntax is insufficient;
4. a library/dialect cannot reasonably provide it;
5. its semantics must be understood by the core compiler.

---

72. Quantum extension architecture

New quantum operations SHOULD be registered through operation metadata or dialect mechanisms.

The core grammar MUST NOT need modification merely because a new physical gate or future quantum operation appears.

The semantic model MUST support:

- parameters;
- targets;
- results;
- modifiers;
- attributes;
- operation identity;
- capabilities;
- resource requirements;
- provenance.

---

73. AI extension architecture

New AI algorithms SHOULD NOT require new universal keywords.

For example, a future learning algorithm SHOULD be representable through the existing learning/model/operation abstraction.

The same principle applies to:

- inference methods;
- optimization algorithms;
- model architectures;
- reasoning strategies;
- uncertainty models;
- planning algorithms.

---

74. Generic computation architecture

The language MUST distinguish:

what computation means

from:

how one target implements it

A matrix multiplication may therefore be realized through:

- scalar CPU operations;
- SIMD;
- GPU tensor hardware;
- FPGA logic;
- ASIC;
- accelerator;
- distributed execution;
- quantum-assisted strategy where semantically valid;
- future hardware.

The source meaning remains stable.

---

75. Target realization

Target realization MAY consider:

- capabilities;
- resources;
- topology;
- performance;
- power;
- memory;
- latency;
- reliability;
- security;
- cost;
- availability;
- policy;
- preferences.

Target-specific information MUST NOT leak backward into universal source semantics unless the programmer explicitly requested target-specific behavior.

---

76. Resource negotiation

Resource negotiation MUST occur downstream of source parsing.

Conceptually:

source requirement
        ↓
semantic requirement
        ↓
candidate targets
        ↓
capability discovery
        ↓
resource evaluation
        ↓
policy evaluation
        ↓
realization selection

A failure MUST identify which requirement could not be satisfied.

---

77. Fallbacks

Fallbacks MAY include:

- alternative implementation;
- decomposition;
- simulation;
- routing;
- distribution;
- scheduling;
- retry;
- recovery;
- migration.

Fallbacks MUST be explicitly permitted by semantic policy.

The compiler MUST NOT silently choose a fallback that changes required semantics.

---

78. Contracts across lowering

If a contract remains semantically relevant after lowering, the lowering process MUST preserve it or produce an equivalent validated representation.

Compiler optimizations MUST NOT invalidate:

- "requires";
- "ensures";
- "invariant";
- "assume";
- "guarantee";
- "property".

An optimization that changes a contract's interpretation is invalid.

---

79. Provenance across lowering

Important transformations SHOULD be traceable:

source
  ↓
AST
  ↓
semantic fact
  ↓
optimization
  ↓
lowering
  ↓
routing
  ↓
scheduling
  ↓
target realization

Provenance MUST identify the transformation or decision when the applicable specification requires traceability.

---

80. Classical IR

Classical IR is the canonical target-independent representation for classical computation.

It SHOULD represent concepts such as:

- names;
- namespaces;
- operands;
- parameters;
- results;
- attributes;
- modifiers;
- effects;
- capabilities;
- resources;
- source provenance.

The IR MUST remain independent of a single backend.

---

81. Quantum IR

"quantum::ir" is the canonical quantum IR.

It MUST represent quantum semantic operations without requiring a specific physical QPU.

It SHOULD preserve:

- operation identity;
- targets;
- parameters;
- results;
- attributes;
- modifiers;
- effects;
- capabilities;
- resources;
- source/provenance information.

Physical mapping belongs downstream.

---

82. IR invariants

IR MUST satisfy:

- type correctness;
- operand/result correctness;
- effect correctness;
- capability correctness;
- resource correctness;
- contract preservation;
- provenance requirements;
- deterministic serialization where specified;
- version compatibility.

Invalid IR MUST NOT be emitted as successful compiler output.

---

83. Optimization

Optimization MUST preserve program semantics.

Optimizations MAY exploit:

- target capabilities;
- resources;
- topology;
- parallelism;
- data locality;
- quantum connectivity;
- hardware features;
- execution policies.

Optimization MUST NOT convert a preference into a semantic requirement or remove a required guarantee.

---

84. Routing

Routing is a downstream realization concern.

Examples include:

- memory placement;
- network routing;
- quantum qubit mapping;
- accelerator data movement;
- distributed task placement.

Routing MUST operate from semantic/IR requirements.

It MUST NOT require source programs to enumerate every physical resource unless explicit target-specific programming is intended.

---

85. Scheduling

Scheduling MAY account for:

- dependencies;
- effects;
- resources;
- topology;
- timing;
- power;
- concurrency;
- reliability;
- policy.

The source language SHOULD remain independent of a specific scheduler.

---

86. ZQN and HAL

The grammar subsystem MUST NOT redefine ZQN or HAL.

It MUST provide the semantic information necessary for downstream systems to reach them.

Conceptual boundary:

Zamani source
    ↓
semantic model
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
ZQN
    ↓
HAL

---

87. Domain ownership boundaries

The following ownership model applies.

Domain| Primary owner
Universal syntax| "core/"
Types| "types/"
Expressions| "expressions/"
Statements| "statements/"
Declarations| "declarations/"
Functions| "functions/"
Modules| "modules/"
Memory| "memory/"
Classical| "classical/"
Quantum| "quantum/"
Hybrid| "hybrid/"
HDL| "hdl/"
Hardware| "hardware/"
AI| "ai/"
Data| "data/"
Concurrency| "concurrency/"
Distributed| "distributed/"
Networking| "networking/"
Effects| "effects/"
Resources| "resources/"
Validation| "validation/"
Policies| "policies/" where established
Security| "security/"
Execution| "execution/"
Interoperability| "interoperability/"
Dialects| "dialects/"
Metaprogramming| "metaprogramming/"
Macros| "macros/"
Compatibility| "compatibility/"
Tests| "tests/"

No directory may silently become a second owner of another subsystem.

---

88. Recommended policy subsystem

Where not already present, policy architecture SHOULD be centralized under:

grammar/policies/

Potential ownership:

policy.g4
scopes.g4
requirements.g4
constraints.g4
permissions.g4
prohibitions.g4
preferences.g4
fallbacks.g4
adaptation.g4
execution.g4
security.g4
resource.g4
deployment.g4
simulation.g4
provenance.g4

Existing security, execution, and resource grammars consume the common policy model rather than creating competing policy languages.

---

89. Recommended AI organization

The AI subsystem SHOULD remain organized around generic computational capabilities:

grammar/ai/
├── ai.g4
├── agents.g4
├── models.g4
├── inference.g4
├── reasoning.g4
├── deduction.g4
├── induction.g4
├── abduction.g4
├── knowledge.g4
├── facts.g4
├── assertions.g4
├── retraction.g4
├── queries.g4
├── learning.g4
├── adaptation.g4
├── feedback.g4
├── uncertainty.g4
├── probability.g4
├── distributions.g4
├── confidence.g4
├── causality.g4
├── explanations.g4
├── evidence.g4
├── decisions.g4
├── provenance.g4
├── transfer-learning.g4
├── reinforcement.g4
├── neural-symbolic.g4
├── cognitive.g4
├── multi-agent.g4
├── planning.g4
├── policies.g4
└── ai-capabilities.g4

The exact final inventory MUST follow actual repository ownership and avoid duplicate existing files.

---

90. Universal grammar versus semantic registry

The grammar SHOULD describe structural categories.

Registries SHOULD describe extensible entities.

Examples suitable for registries include:

- capabilities;
- quantum operations;
- dialects;
- target features;
- effect extensions;
- AI algorithms;
- hardware features;
- protocol identifiers.

This is essential for long-term scalability.

---

91. No physical enumeration in universal grammar

The universal grammar MUST NOT enumerate every:

- CPU;
- GPU;
- FPGA;
- ASIC;
- QPU;
- accelerator;
- network;
- processor topology;
- memory technology;
- future device.

The grammar describes categories and semantic requirements.

Target registries describe available implementations.

---

92. No fixed tensor universe

Tensor syntax MUST support symbolic or runtime dimensions where semantics allow.

The grammar MUST NOT impose a universal tensor rank or dimension maximum.

Implementation limits remain implementation concerns.

---

93. No fixed network universe

Distributed/network syntax MUST NOT assume a fixed number of:

- nodes;
- services;
- endpoints;
- actors;
- channels;
- devices.

Network topology is represented semantically and resolved against available infrastructure.

---

94. No fixed quantum universe

The language MUST NOT assume:

- a fixed maximum qubit count;
- a fixed number of quantum registers;
- a fixed gate inventory;
- a fixed topology;
- a fixed QPU architecture.

The only limits that matter to execution are those established by actual semantics and target availability.

---

95. No fixed concurrency universe

Concurrency MUST NOT assume:

- fixed thread counts;
- fixed actor counts;
- fixed task counts;
- fixed queue sizes.

Resource-aware execution determines feasible realization.

---

96. Application of generic reasoning

Reasoning constructs MUST integrate with normal expressions and statements where appropriate.

The architecture should permit forms conceptually equivalent to:

infer conclusion from evidence;
deduce conclusion from premises;
reason {
    premises ...
    evidence ...
    conclusion ...
}

The final syntax belongs to the normative expression/statement specification.

---

97. Knowledge integration

Knowledge operations SHOULD conceptually support:

assert fact;
retract fact;
query pattern;

Knowledge state MUST have explicit ownership and effect semantics.

A knowledge mutation is not semantically equivalent to an ordinary pure expression.

---

98. Learning integration

Learning MUST integrate with the effect system.

For example:

learning

may imply:

effect(learning)

and may require:

capability("learning.compute")

or another registered capability.

The exact capability vocabulary MUST remain extensible.

---

99. Adaptation integration

Adaptation MUST integrate with:

effects
capabilities
resources
policies
authorization
contracts
provenance

Adaptation MUST be rejected if required policy authorization is absent.

---

100. Explanation integration

Explanation SHOULD be available for any semantic decision that requires auditability.

The compiler MAY explain:

- why a target was selected;
- why a fallback occurred;
- why a quantum route was selected;
- why an optimization was applied;
- why a capability requirement failed;
- why an adaptation was rejected.

This information SHOULD be connected to provenance.

---

101. Evidence integration

Evidence SHOULD be reusable by:

- reasoning;
- contracts;
- AI;
- validation;
- security;
- compiler decisions;
- scientific computation.

Evidence is not an AI-only abstraction.

---

102. Multi-agent integration

Agent semantics MUST reuse concurrency.

The architecture MUST NOT create:

AI actor system

separate from:

Zamani actor system

Instead:

agent semantics
      ↓
actor semantics
      ↓
concurrency runtime

---

103. Neural-symbolic integration

Neural-symbolic computation MUST combine:

learned models
+
symbolic representations
+
reasoning
+
knowledge

within the same semantic framework.

It MUST NOT create a second IR architecture.

---

104. Reflection and controlled introspection

Reflection MUST distinguish:

inspect

from:

mutate

Inspection may be permitted under one capability.

Mutation may require another.

Generated code may require another.

This separation is essential for security and deterministic compilation.

---

105. Compile-time computation

Compile-time execution MUST be distinguishable from runtime execution.

It MUST have:

- defined inputs;
- defined outputs;
- resource policy;
- effect restrictions;
- determinism requirements where applicable;
- provenance.

Compile-time execution MUST NOT silently gain unrestricted runtime capabilities.

---

106. Language-level unsafe versus Rust safety

The existence of an existing Zamani source-level construct representing an unsafe operation does not permit unsafe Rust implementation.

The architecture distinguishes:

Zamani source-language semantics

from:

Rust implementation safety

The Rust compiler implementation MUST remain safe.

Any source-level unsafe semantics MUST be explicitly specified, validated, diagnosed, and integrated with security policy.

---

107. FFI safety

FFI MUST have a safe semantic abstraction.

The compiler MUST validate:

- type compatibility;
- calling convention;
- ABI;
- data layout;
- ownership requirements;
- lifetime requirements where applicable;
- effects;
- capabilities;
- policy.

The implementation MUST NOT use unsafe Rust as a shortcut around semantic validation.

---

108. Testing architecture

The grammar subsystem MUST maintain tests at multiple layers.

Required categories include:

lexical
parser
AST
semantic
type
effect
resource
capability
contract
policy
provenance
classical
quantum
hybrid
HDL
AI
concurrency
distributed
networking
interoperability
metaprogramming
security
simulation
compatibility
scalability
portability
boundary
negative
determinism

---

109. Positive tests

Every implemented feature MUST have valid examples demonstrating intended behavior.

---

110. Negative tests

Every implemented feature MUST have invalid examples proving that prohibited behavior is rejected.

Negative tests are especially mandatory for:

- policies;
- capabilities;
- resources;
- effects;
- contracts;
- unsafe source operations;
- adaptation;
- reflection;
- FFI;
- unsupported target features.

---

111. Boundary tests

Boundary tests MUST verify interactions between domains.

Examples:

classical + quantum
AI + quantum
AI + concurrency
AI + contracts
AI + provenance
HDL + simulation
data + AI
data + distributed
quantum + resources
quantum + policies
network + security
FFI + effects
reflection + policy
adaptation + provenance

---

112. Scalability tests

Scalability tests MUST verify that the architecture does not contain artificial fixed capacities.

Tests SHOULD use:

- symbolic dimensions;
- dynamically selected resource counts;
- large generated structures;
- nested domains;
- large ASTs where practical;
- large quantum descriptions;
- large tensor descriptions;
- large distributed descriptions.

The purpose is to detect hidden limits rather than prove literal mathematical infinity.

---

113. Portability tests

At minimum, conformance tests SHOULD demonstrate that identical source semantics can be lowered toward multiple classes of targets without modifying the source algorithm.

Examples include:

CPU
GPU
FPGA
accelerator
QPU
simulator
HPC
distributed

where the relevant backend exists.

---

114. Determinism tests

The same input and configuration MUST produce equivalent frontend results.

Tests MUST detect:

- token-order instability;
- nondeterministic diagnostics;
- unstable AST generation;
- unstable semantic resolution;
- unstable registry ordering;
- unstable serialization.

---

115. Integration fixtures

The repository SHOULD maintain representative source fixtures including:

minimal.zm
classical.zm
generic.zm
quantum.zm
hybrid.zm
hdl.zm
poco-reaf.zm

Additional cross-domain fixtures SHOULD cover:

reasoning
knowledge
learning
adaptation
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
advanced types
quantum learning
AI + quantum hybrid execution

Fixtures MUST be categorized by implementation status.

---

116. The POCO-REAF integration fixture

At least one comprehensive integration fixture MUST combine, where supported:

classical computation
+
generic types
+
data/tensor computation
+
reasoning
+
knowledge
+
learning
+
adaptation
+
uncertainty
+
contracts
+
policies
+
provenance
+
resource requirements
+
capability requirements
+
effects
+
concurrency
+
distributed behavior
+
quantum computation
+
measurement
+
hybrid control
+
simulation
+
hardware intent

The fixture MUST be traceable through:

source
→ lexer
→ parser
→ AST
→ structural validation
→ semantic model
→ resource analysis
→ capability analysis
→ effect analysis
→ contract analysis
→ policy analysis
→ provenance
→ classical IR
→ quantum::ir
→ optimization
→ lowering
→ routing
→ scheduling
→ resilience

where the selected program actually requires those stages.

---

117. Completion contract for every grammar file

Every production grammar file MUST have an associated completion contract.

At minimum:

PURPOSE:
OWNERS:
DOES_NOT_OWN:
DEPENDS_ON:
EXPORTS:
LEXER_DEPENDENCIES:
GRAMMAR_DEPENDENCIES:
AST_CONTRACT:
SEMANTIC_CONTRACT:
TYPE_CONTRACT:
EFFECT_CONTRACT:
CAPABILITY_CONTRACT:
RESOURCE_CONTRACT:
CONTRACT_INTEGRATION:
POLICY_INTEGRATION:
PROVENANCE_INTEGRATION:
IR_CONTRACT:
BACKEND_BOUNDARY:
DIAGNOSTICS:
POSITIVE_TESTS:
NEGATIVE_TESTS:
BOUNDARY_TESTS:
SCALABILITY_TESTS:
DETERMINISM_TESTS:
COMPATIBILITY:
SPECIFICATION:
IMPLEMENTATION:
COMPLETION_CRITERIA:

A file is not complete until every applicable field is resolved.

---

118. Completion means end-to-end completeness

A grammar file MUST NOT be considered complete merely because:

- its ".g4" rules compile;
- the parser accepts its syntax;
- documentation exists.

Completion requires all applicable layers:

Specification
    ↓
Lexer
    ↓
Parser
    ↓
AST
    ↓
Structural validation
    ↓
Semantic validation
    ↓
Type checking
    ↓
Effect checking
    ↓
Capability checking
    ↓
Resource checking
    ↓
Contract checking
    ↓
Policy checking
    ↓
Provenance
    ↓
IR
    ↓
Backend integration
    ↓
Tests

Not every feature needs every downstream layer, but every omitted layer MUST be explicitly marked not applicable.

---

119. No hidden dependencies

A grammar file MUST NOT depend on undocumented behavior from another grammar file.

Dependencies MUST be explicit.

Circular ownership MUST be avoided.

If two subsystems require each other semantically, their shared abstraction MUST be moved to a lower-level common owner.

---

120. No duplicate semantic ownership

The following MUST NOT have competing implementations:

token vocabulary
root program grammar
pattern semantics
actor semantics
effect semantics
resource semantics
capability semantics
policy semantics
provenance semantics
canonical classical IR
canonical quantum IR

Specialized consumers may exist, but one semantic owner MUST exist for each universal concept.

---

121. No duplicate quantum IR

There MUST be exactly one canonical quantum IR boundary:

quantum::ir

Adapters and temporary forms are allowed only when they have documented lowering into it.

---

122. No domain bypass

A domain grammar MUST NOT bypass:

types
effects
capabilities
resources
contracts
policies
provenance

when those concepts apply.

For example, a quantum measurement cannot bypass effect/capability semantics merely because its syntax is in "quantum/".

---

123. Repository navigation authority

"grammar/README.md" is navigation.

It MUST tell developers:

- where the architecture lives;
- where language specifications live;
- where machine contracts live;
- where grammar composition lives;
- where lexer authority lives;
- where tests live;
- where domain subsystems live.

It MUST NOT become an independent language specification.

---

124. "grammar/DESIGN.md" scope boundary

This file owns:

- architecture;
- ownership;
- boundaries;
- invariants;
- integration contracts;
- scalability principles;
- compatibility principles;
- production readiness criteria.

It MUST NOT become an inventory of every implemented token.

---

125. Production migration rule

Existing grammar files MUST be migrated incrementally.

Migration MUST follow:

inspect
  ↓
classify
  ↓
assign ownership
  ↓
define contract
  ↓
reconcile specification
  ↓
reconcile lexer
  ↓
reconcile parser
  ↓
reconcile AST
  ↓
reconcile semantics
  ↓
reconcile IR
  ↓
add tests
  ↓
mark status

Existing functionality MUST NOT be deleted merely to make the architecture appear clean.

---

126. Conflict-resolution hierarchy

When conflicts exist, resolve them in this order:

1. grammar/DESIGN.md
        ↓
2. grammar/specification/
        ↓
3. grammar/spec/
        ↓
4. grammar/Zamani.g4
        ↓
5. grammar/antlr/
        ↓
6. Rust lexer/parser/AST implementation
        ↓
7. domain implementations
        ↓
8. tests and generated artifacts

This hierarchy describes authority, not necessarily implementation dependency.

An implementation that disagrees with a normative specification is an implementation defect unless the specification itself is deliberately changed.

---

127. Specification-to-implementation traceability

Every stable feature MUST be traceable:

SPECIFICATION
      ↓
TOKEN
      ↓
GRAMMAR RULE
      ↓
AST
      ↓
SEMANTIC MODEL
      ↓
VALIDATION
      ↓
IR
      ↓
BACKEND
      ↓
TEST

The repository SHOULD maintain machine-readable traceability metadata where practical.

---

128. Feature lifecycle

A feature progresses through:

proposal
    ↓
architectural review
    ↓
semantic specification
    ↓
AST contract
    ↓
grammar contract
    ↓
lexer/parser implementation
    ↓
semantic implementation
    ↓
IR integration
    ↓
tests
    ↓
compatibility review
    ↓
stable

Possible statuses include:

PROPOSED
SPECIFIED
PARTIALLY_IMPLEMENTED
IMPLEMENTED
EXPERIMENTAL
STABLE
DEPRECATED
HISTORICAL
UNSUPPORTED

---

129. Production readiness gate

The grammar subsystem MUST NOT be called production-ready until:

1. ownership is unambiguous;
2. normative specifications exist;
3. lexer and parser agree;
4. AST contracts exist;
5. semantic contracts exist;
6. type interactions are defined;
7. effect interactions are defined;
8. resource interactions are defined;
9. capability interactions are defined;
10. contract interactions are defined;
11. policy interactions are defined;
12. provenance behavior is defined;
13. IR lowering exists where applicable;
14. quantum constructs lower through "quantum::ir";
15. target-independent semantics are preserved;
16. scalability tests pass;
17. negative tests pass;
18. boundary tests pass;
19. compatibility tests pass;
20. determinism tests pass;
21. Rust implementation uses no "unsafe";
22. minimum Rust toolchain is supported;
23. generated artifacts are reproducible;
24. unsupported features are explicitly reported;
25. no universal physical capacity is hard-coded.

---

130. Definition of "scalable to infinity"

Zamani MUST NOT claim literal execution of mathematically infinite programs on finite machines.

The correct architectural guarantee is:

«The language places no arbitrary universal hardware ceiling on the semantic size of a valid program. Actual execution is bounded only by the requirements of the computation, implementation representation, target capabilities, policies, and resources available to the selected realization.»

Therefore the architecture is open-ended with respect to:

- qubits;
- CPUs;
- GPUs;
- FPGAs;
- ASICs;
- nodes;
- memory;
- tensors;
- devices;
- agents;
- threads;
- networks;
- datasets;
- program size.

Any finite implementation limit is an implementation constraint, not a universal language limit.

---

131. Small-to-large execution model

The same logical program MAY be realized differently depending on available resources.

Conceptually:

                    Same Zamani Program
                            │
                            ▼
                  Semantic Requirements
                            │
             ┌──────────────┼──────────────┐
             │              │              │
          Tiny target    Large target   Distributed
             │              │              │
             ▼              ▼              ▼
        compact plan    parallel plan   distributed plan
             │              │              │
             └──────────────┼──────────────┘
                            ▼
                       Same Meaning

The compiler may specialize implementation without requiring the developer to rewrite the program.

---

132. Future hardware

Future hardware MUST be able to integrate through:

capability registration
resource registration
target description
dialect registration where necessary
lowering
HAL integration

A future device MUST NOT require rewriting universal source semantics simply because it did not exist when the language was designed.

---

133. Future computational paradigms

The same rule applies to future computational paradigms.

A future domain SHOULD integrate through:

domain grammar
      ↓
domain semantic model
      ↓
common types/effects/resources/capabilities
      ↓
canonical semantic representation
      ↓
domain IR if necessary
      ↓
target realization

The universal core evolves only when a truly universal language concept is discovered.

---

134. Library-first principle

Functionality that does not require compiler-wide semantic knowledge SHOULD be implemented as libraries rather than grammar additions.

Examples include application algorithms, domain models, specialized AI models, protocol implementations, and business logic.

This keeps the language compact and extensible.

---

135. Dialect-first principle

Functionality requiring specialized syntax but not universal language semantics SHOULD be implemented as a dialect.

Dialect registration MUST include versioning and compatibility.

---

136. Capability-first principle

Hardware or environment properties SHOULD be represented as capabilities.

Examples:

capability("quantum.measurement")
capability("gpu.compute")
capability("tensor.compute")
capability("network.transport")
capability("accelerator.matrix")

Capability names are semantic identifiers, not physical device enumerations.

---

137. Policy-first principle

Security, deployment, resource selection, adaptation, and execution restrictions SHOULD be represented as policies.

Policies MUST NOT be encoded through scattered special-case keywords.

---

138. Metadata-first principle

Extensible entities SHOULD use metadata when structural grammar is already sufficient.

Examples include:

- operation attributes;
- capability descriptors;
- dialect metadata;
- target metadata;
- provenance metadata;
- model metadata.

This prevents grammar proliferation.

---

139. Backward compatibility

Existing valid Zamani programs MUST remain valid unless an explicitly approved breaking language change occurs.

A breaking change MUST include:

- migration guidance;
- version behavior;
- diagnostics;
- compatibility tests;
- deprecation policy where practical.

---

140. Forward compatibility

Where practical, the architecture SHOULD permit unknown future metadata or extension identifiers to be handled without corrupting unrelated semantic information.

Unsupported extensions MUST be diagnosed explicitly when execution requires them.

---

141. Unknown capabilities

An unknown capability MUST NOT automatically be interpreted as:

supported

or:

unsupported

without the applicable target/capability protocol determining the state.

The compiler SHOULD distinguish:

known unavailable
known available
unknown
not applicable

where the target model supports those distinctions.

---

142. Resource units

Resource comparisons MUST have defined units.

For example, memory requirements MUST distinguish the appropriate unit and representation.

The grammar MUST NOT assume a universal byte capacity.

---

143. Resource expressions

Resource requirements SHOULD permit symbolic expressions.

For example:

requires memory >= required_memory;

where "required_memory" is derived from program semantics.

The compiler evaluates the expression against the selected realization.

---

144. Capability composition

Capabilities MAY be hierarchical or compositional.

For example:

quantum
quantum.measurement
quantum.dynamic_circuit

or:

gpu
gpu.compute
gpu.tensor

The capability system MUST define inheritance/implication explicitly rather than relying on string-prefix assumptions.

---

145. Effect composition

Effects MAY compose.

For example, an operation might involve:

network
foreign
mutation

The semantic model MUST retain the complete applicable effect set.

---

146. Contract composition

Contracts MAY be inherited, composed, refined, or specialized according to the type/function/domain specification.

Contradictory contracts MUST produce deterministic diagnostics.

---

147. Policy composition

Policy precedence MUST be explicitly defined.

Policies MUST NOT rely on undocumented source order.

The policy model SHOULD distinguish:

require
allow
deny
forbid
prefer
fallback
override

only where each has formally specified semantics.

---

148. Provenance composition

Provenance from independent transformations MAY be composed into a derivation graph.

The representation MUST remain extensible.

Implementations MAY compress provenance, but MUST preserve required semantic traceability.

---

149. Source spans

Tokens, AST nodes, diagnostics, and semantic errors MUST preserve source-location information wherever meaningful.

Source spans MUST remain valid across UTF-8 source handling.

The Rust lexer/parser MUST maintain consistent byte-position and line/column semantics.

---

150. Unicode

Unicode handling MUST be explicitly specified.

The lexer MUST distinguish:

- valid identifiers;
- reserved words;
- Unicode literals;
- source punctuation;
- invalid characters.

Unicode support MUST NOT introduce ambiguous tokenization.

---

151. Literal architecture

Literal forms MUST have explicit lexical and semantic ownership.

Existing specialized literals, including quantum and nano-related forms, MUST be reconciled between:

lexer
ANTLR lexer
parser
AST
semantic model
tests

A documented literal that is not emitted by the executable lexer is not considered implemented.

---

152. Lexer keyword policy

Reserved keywords MUST be kept to a minimum.

A token SHOULD be reserved only when compiler-wide syntax requires it.

Application vocabulary SHOULD remain identifier-based.

Contextual keywords MAY be used where they reduce compatibility impact and the parser semantics justify them.

---

153. Parser precedence

Expression precedence and associativity MUST have one normative specification.

The Rust parser and ANTLR grammar MUST implement the same precedence model.

No domain subsystem may silently change universal expression precedence.

---

154. Module boundaries

Modules MUST provide explicit namespace boundaries.

Cross-module references MUST participate in:

- name resolution;
- visibility;
- type checking;
- capability/resource requirements;
- provenance;
- compatibility.

---

155. Names and identifiers

Names MUST remain target-independent.

Physical identifiers SHOULD be represented as target metadata rather than universal language names.

For example, a source-level logical resource SHOULD not require a particular physical QPU identifier.

---

156. Attributes and modifiers

Attributes and modifiers SHOULD provide extensibility without multiplying grammar productions.

They MUST have explicit owners and semantics.

Unknown attributes MUST be handled according to declared extension policy.

---

157. Contracts on hardware

HDL constructs MAY have:

requires
ensures
invariant
property

to support verification.

Hardware contracts MUST integrate with the same contract architecture rather than creating a separate verification language.

---

158. Simulation and verification

Simulation results SHOULD be distinguishable from proof/verification results.

The compiler MUST NOT represent a simulation result as a formal proof unless the verification specification explicitly permits that interpretation.

Provenance SHOULD record how a result was obtained.

---

159. AI confidence versus proof

Confidence, probability, and evidence MUST NOT automatically imply formal correctness.

The semantic model MUST distinguish:

confidence
probability
evidence
verification
proof

These concepts may interact but are not interchangeable.

---

160. Deterministic learning and randomness

Learning algorithms that use randomness MUST expose the relevant randomness effect.

Where reproducibility is requested, the applicable random state MUST be controlled and represented in the execution/provenance model.

---

161. Distributed determinism

Distributed execution MUST define the semantics of ordering where ordering matters.

The compiler MUST NOT assume that physical message delivery order is deterministic.

---

162. Resource-aware AI

AI operations MUST use the universal resource model.

A model may require:

memory
compute
tensor capability
accelerator capability
network capability

without defining fixed universal capacities.

---

163. Resource-aware quantum computation

Quantum operations MAY require:

qubits
connectivity
measurement
coherence
error correction
dynamic circuits

These requirements MUST be represented semantically and resolved against target capabilities.

---

164. Resource-aware HDL

HDL realization MAY require:

logic resources
memory resources
timing resources
I/O capabilities
fabric capabilities

but universal limits MUST remain outside the language architecture.

---

165. Compiler knowledge

The knowledge model MAY be reused internally by compiler systems.

Examples:

target supports operation
target satisfies capability
transformation preserves property

Compiler knowledge MUST remain distinguishable from user program state.

---

166. Decision records

Important compiler/runtime decisions MAY have structured decision records containing:

decision
alternatives
selected_option
reason
evidence
constraints
provenance

This supports explainability without requiring a special application-specific language.

---

167. Security auditability

Security-sensitive decisions SHOULD be traceable through:

request
policy
capability
authorization
decision
provenance

This is especially important for:

- FFI;
- native operations;
- adaptation;
- reflection;
- code generation;
- network access.

---

168. Compiler configuration

Compiler configuration MUST NOT silently change language semantics.

Configuration MAY select:

- target;
- optimization level;
- resource policy;
- execution strategy;
- dialect;
- compatibility mode.

The effective configuration SHOULD participate in provenance where reproducibility requires it.

---

169. Target-specific programming

Target-specific programming MAY exist.

However, it MUST be explicit.

Target-specific constructs MUST NOT become hidden assumptions of otherwise portable source code.

The architecture SHOULD make target-specific behavior visibly distinct from portable semantics.

---

170. Portable default

The default language model SHOULD favor portable intent.

For example:

requires capability("gpu.compute");

is preferable to embedding a particular GPU model in universal grammar.

---

171. Compilation failure semantics

When realization fails, the compiler MUST distinguish:

program invalid

from:

program valid but target cannot satisfy requirements

The second case is important to POCO-REAF.

A valid program does not become invalid merely because one machine lacks the necessary resources.

---

172. Simulation fallback

Simulation MAY be used where explicitly authorized.

For example, a quantum program may be simulated if a QPU is unavailable and the policy permits simulation.

The compiler MUST clearly identify that realization as simulation.

It MUST NOT claim physical execution occurred.

---

173. Decomposition

A computation MAY be decomposed to fit target resources.

Decomposition MUST preserve semantics.

Where decomposition weakens a guarantee, it MUST be rejected or explicitly represented as a different execution contract.

---

174. Distribution

A program MAY be distributed across available resources.

Distribution MUST respect:

- effects;
- contracts;
- resource requirements;
- consistency semantics;
- ordering;
- policies;
- security.

---

175. Compilation once

Compilation artifacts SHOULD preserve sufficient target-independent semantic information to enable later realization where the chosen artifact format supports this.

A target-specific binary is not inherently portable.

The architecture therefore distinguishes:

portable compilation artifacts

from:

target executables

---

176. "Forever" in POCO-REAF

"Forever" means architectural continuity rather than an impossible promise that all future hardware will execute every historical binary unchanged.

Future systems MUST be able to preserve program meaning through:

- compatibility layers;
- migration;
- IR versioning;
- dialect evolution;
- target adapters;
- semantic preservation;
- explicit deprecation.

---

177. Repository growth rule

As the repository grows, new directories MAY be created when an existing directory has accumulated unrelated responsibilities.

A new directory MUST have:

- a clear owner;
- README;
- specification location;
- integration boundary;
- tests;
- dependency contract.

Directories MUST NOT be created merely for cosmetic organization.

---

178. Directory README requirements

Each major subsystem README SHOULD state:

Purpose
Scope
Owns
Does Not Own
Architecture Authority
Specification Authority
Grammar Authority
AST Owner
Semantic Owner
IR Owner
Dependencies
Consumers
Tests
Compatibility
Scalability
Completion Criteria

This keeps repository navigation consistent with "DESIGN.md".

---

179. Production documentation requirements

Every production subsystem MUST document:

- architecture;
- ownership;
- syntax;
- semantics;
- diagnostics;
- compatibility;
- examples;
- tests;
- implementation status.

Documentation MUST distinguish normative rules from examples.

---

180. Implementation status discipline

A file MUST NOT claim:

production-ready

unless the production readiness gate has been met.

Terms such as:

planned
prototype
experimental
partial
specified

MUST be used honestly.

---

181. Existing implementation reconciliation

When existing implementation and architecture disagree, developers MUST:

1. identify the disagreement;
2. determine whether specification or implementation is authoritative;
3. update the appropriate authority;
4. update implementation;
5. update tests;
6. update conformance status.

They MUST NOT silently maintain contradictory behavior.

---

182. Repository-wide audit requirements

Before declaring "grammar/" production-ready, audit for:

- duplicate tokens;
- duplicate grammar ownership;
- duplicate AST nodes;
- duplicate semantic concepts;
- duplicate resource models;
- duplicate capability models;
- duplicate effect models;
- duplicate policy models;
- duplicate provenance models;
- duplicate quantum IRs;
- fixed hardware constants;
- hidden resource limits;
- inconsistent keyword registries;
- parser/lexer divergence;
- undocumented dialects;
- stale generated files;
- unsafe Rust;
- untested grammar productions.

---

183. Hard-coding audit

The following search categories MUST be checked throughout the repository:

MAX_
maximum
max_
limit
capacity
qubits
cpus
gpus
fpgas
nodes
threads
memory
tensor rank
register width
network size
device count

The purpose is not to ban legitimate local implementation limits.

The purpose is to identify accidental universal language limits.

Every discovered limit MUST be classified as:

language semantic limit
implementation limit
target limit
configuration limit
test fixture
documentation example

Only the appropriate category may remain.

---

184. No artificial example capacities

Documentation MUST avoid presenting arbitrary hardware capacities as if they were language-wide limits.

Examples MAY use concrete values as ordinary data.

For example:

let n = 1024;

is valid.

What is prohibited is defining:

MAX_QUBITS = 1024

as a universal language boundary.

---

185. API stability

Public Rust structures used by frontend components SHOULD have explicit stability contracts.

Changes to:

- tokens;
- AST;
- semantic structures;
- IR;
- diagnostics

MUST be reviewed for downstream impact.

---

186. Safe collection handling

Rust implementations SHOULD use standard safe collections and checked operations.

Where indexing or arithmetic may fail, the implementation SHOULD use:

- checked arithmetic;
- explicit validation;
- safe indexing;
- structured errors.

Panics MUST NOT be the normal mechanism for handling malformed user programs.

---

187. No silent overflow

Numeric/resource calculations MUST define overflow behavior.

The compiler MUST NOT silently overflow values used to determine:

- memory requirements;
- tensor dimensions;
- qubit counts;
- resource budgets;
- allocation sizes;
- offsets;
- program structure.

Overflow MUST produce a defined diagnostic or use a representation whose semantics explicitly support the required range.

---

188. Large-number architecture

Where language semantics require large symbolic or arbitrary-precision values, the implementation SHOULD use an appropriate safe representation rather than imposing an arbitrary small machine integer.

The selected representation MUST be documented.

---

189. Memory scalability

Compiler data structures SHOULD avoid unnecessary duplication.

For very large programs, the implementation SHOULD support:

- incremental processing;
- lazy structures where appropriate;
- compact representations;
- streaming where semantics allow;
- bounded diagnostic retention;
- configurable resource policies.

These are implementation optimizations and MUST NOT change language semantics.

---

190. Incremental compilation

The architecture SHOULD support incremental compilation where practical.

Incremental invalidation MUST respect:

- modules;
- dependencies;
- semantic changes;
- type changes;
- capability changes;
- resource changes;
- policy changes;
- dialect changes.

---

191. Parallel compilation

Compilation MAY be parallelized.

Parallel compiler execution MUST produce semantically equivalent results to serial execution.

Ordering-sensitive diagnostics or generated artifacts MUST have deterministic ordering rules.

---

192. Domain-neutral AST requirement

The AST MUST NOT become a collection of backend-specific data structures.

Domain-specific nodes MAY exist where the source language has domain-specific semantics.

However, those nodes MUST represent logical meaning rather than physical implementation.

---

193. Semantic normalization

Equivalent source constructs SHOULD normalize into equivalent semantic structures where their semantics are identical.

Normalization reduces downstream duplication.

---

194. Canonical semantic operations

Operations SHOULD carry a common semantic shape where applicable:

name
namespace
operands
parameters
results
attributes
modifiers
effects
capabilities
resources
contracts
policies
source
provenance

Domain-specific fields MAY extend this model.

---

195. Extensibility of operation metadata

Operation metadata MUST be extensible.

This is particularly important for:

- quantum operations;
- accelerators;
- tensor operations;
- future hardware;
- domain dialects.

The grammar MUST represent common structure rather than enumerating every operation.

---

196. Semantic ownership of operation identity

An operation's semantic identity belongs to the semantic registry/domain specification.

The lexer MUST NOT determine operation semantics.

The parser MUST only establish syntactic structure.

The semantic layer determines whether an operation exists and what it means.

---

197. Capability ownership

Capability definitions belong to the capability/resource subsystem.

Domain grammar files may consume capabilities.

They MUST NOT redefine the global capability model independently.

---

198. Effect ownership

Effect definitions belong to the effects subsystem.

Domain files may declare effects.

They MUST NOT create incompatible effect semantics independently.

---

199. Resource ownership

Resource semantics belong to the resources subsystem.

Domains declare requirements.

The resource subsystem defines their common representation and validation.

---

200. Policy ownership

Policy semantics belong to the policy/security architecture.

Domains consume policy decisions.

They MUST NOT silently invent incompatible policy precedence.

---

201. Provenance ownership

Provenance semantics belong to the common provenance architecture.

Every domain MAY attach domain-specific provenance fields, but the underlying model remains common.

---

202. Contract ownership

Contract semantics belong to validation/specification.

Domains MAY define domain-specific contract predicates, but the lifecycle remains common.

---

203. Testing ownership

Every feature MUST identify its tests.

A test that only checks parsing does not establish semantic implementation.

A test that only checks runtime behavior does not establish lexical conformance.

Conformance MUST be layered.

---

204. Conformance matrix

The repository SHOULD maintain a feature matrix:

Feature| Spec| Lexer| Parser| AST| Semantics| Types| Effects| Resources| Policies| Provenance| IR| Backend| Tests| Status

This matrix MUST be generated or maintained consistently with "grammar/grammar.md".

---

205. Cross-file integration contract

When a file is declared complete, its integration contract MUST already identify:

upstream dependencies
downstream consumers
AST representation
semantic representation
IR destination
tests
compatibility
diagnostics

Later files MUST consume those published contracts.

---

206. Changes to completed files

A completed file MAY be changed only when:

- a specification defect is discovered;
- a compatibility defect is discovered;
- a security defect is discovered;
- a scalability defect is discovered;
- an integration contract was demonstrably incorrect;
- a required language feature cannot be represented correctly.

Such a change MUST update its completion evidence.

---

207. Avoiding architecture drift

CI SHOULD check:

- duplicate token declarations;
- duplicate rule ownership;
- undocumented grammar imports;
- inconsistent status metadata;
- missing feature contracts;
- missing tests;
- forbidden hard-coded capacities;
- unsafe Rust;
- stale generated artifacts.

---

208. CI production gates

CI SHOULD include:

format check
lint
minimum Rust build
current stable Rust build
unit tests
grammar generation
grammar validation
frontend conformance
semantic tests
IR tests
integration tests
negative tests
scalability tests
determinism tests
hard-coding audit
unsafe-code audit

---

209. Unsafe-code CI gate

The project SHOULD enforce a zero-unsafe policy for the Rust implementation.

CI MUST fail if production Rust source introduces prohibited unsafe constructs.

The exact mechanism may evolve with repository tooling.

---

210. Grammar generation CI

CI MUST verify that generated grammar artifacts are reproducible.

A clean generation followed by repository comparison SHOULD produce no unexpected changes.

---

211. Grammar/parser parity CI

Shared fixtures SHOULD be parsed by:

- ANTLR frontend;
- Rust frontend;

where both are supported.

Differences MUST be investigated.

---

212. AST parity

Where ANTLR and Rust frontends both produce AST-equivalent structures, conformance tests SHOULD verify semantic equivalence.

Where they intentionally differ internally, both MUST converge on the same language semantics.

---

213. Semantic parity

Different frontend representations MUST produce equivalent canonical semantic models for equivalent source programs.

---

214. IR parity

Equivalent source programs MUST produce semantically equivalent canonical IR where normalization permits comparison.

---

215. Backend independence

The grammar MUST remain usable even when no backend is available.

A valid source program may reach:

semantic validation successful
target realization unavailable

without becoming syntactically invalid.

---

216. Unsupported target behavior

When no target can satisfy a program's requirements, diagnostics SHOULD report:

- required capability;
- required resource;
- applicable policy;
- selected target;
- reason for failure;
- possible permitted alternatives.

---

217. Target discovery

Target discovery is not owned by universal grammar.

It belongs to the compilation/runtime environment.

The grammar expresses what is required.

The environment reports what exists.

---

218. Capability negotiation

Capability negotiation is not hard-coded into grammar.

It is an interaction between:

semantic requirements
target capability registry
policy
resource model
realization engine

---

219. Hardware abstraction

HAL remains the final hardware abstraction boundary.

The grammar MUST NOT import HAL-specific data structures into source semantics.

---

220. Compiler/runtime boundary

The grammar subsystem defines source-language semantics.

The compiler/runtime determines execution.

Runtime behavior MUST NOT be retroactively used to define undocumented syntax.

---

221. Source compatibility with future hardware

A source program should remain meaningful when:

- processor count increases;
- memory increases;
- memory decreases but requirements remain satisfiable;
- GPU changes;
- QPU changes;
- topology changes;
- accelerator changes;
- cluster size changes.

The implementation may select different realizations.

---

222. Resource-aware specialization

Specialization MAY generate target-specific variants.

Specialization MUST preserve the source semantic contract.

Specialized variants MUST retain provenance linking them to the original program.

---

223. Optimization provenance

Optimizations SHOULD record:

transformation
reason
preconditions
result

where required by provenance policy.

---

224. Quantum routing provenance

Quantum routing SHOULD record:

logical operation
physical mapping
routing transformation
target topology
reason

when provenance is enabled.

---

225. AI decision provenance

AI decision semantics SHOULD support:

input
model
evidence
reasoning
confidence
decision
policy
provenance

without requiring a universal AI application vocabulary.

---

226. Knowledge provenance

Assertions and derived knowledge SHOULD preserve origin information when required.

Retraction SHOULD preserve sufficient provenance to explain why a fact ceased to participate in the knowledge state.

---

227. Adaptation provenance

Every authorized adaptation SHOULD be traceable to:

request
policy
authorization
previous state
change
validation
result

where the applicable policy requires auditability.

---

228. Simulation provenance

Simulation SHOULD identify:

- simulated target/model;
- relevant configuration;
- input;
- execution mode;
- random state where applicable;
- result;
- provenance.

---

229. Reproducible compilation provenance

A reproducible build SHOULD record:

- source version;
- language version;
- compiler version;
- grammar version;
- AST/semantic/IR versions;
- dialect versions;
- target configuration;
- relevant policies;
- relevant capability information.

---

230. Semantic preservation

The most important invariant is:

«Lowering, optimization, routing, scheduling, specialization, simulation, distribution, and target realization MUST preserve the declared semantics unless the program explicitly permits an alternative semantic mode.»

---

231. No silent semantic degradation

The compiler MUST NOT silently turn:

requirement

into:

preference

or:

guarantee

into:

best effort

or:

physical execution

into:

simulation

without an explicit semantic decision and policy.

---

232. Domain neutrality

Domain-specific syntax MUST be translated into shared semantic structures.

For example:

AI reasoning
quantum measurement
HDL verification
distributed messaging
tensor computation

may all use:

effects
capabilities
resources
contracts
policies
provenance

without creating incompatible semantic systems.

---

233. Architectural test for new features

Before adding a feature, ask:

1. Is it truly universal?
2. Can an existing construct express it?
3. Can a library express it?
4. Can a dialect express it?
5. Can metadata express it?
6. Does it require a new semantic concept?
7. Does it require a new token?
8. Does it require a new AST node?
9. Does it require a new IR representation?
10. What are its effects?
11. What capabilities does it require?
12. What resources does it require?
13. What contracts apply?
14. What policies apply?
15. What provenance is required?
16. How does it scale?
17. How does it behave on unavailable targets?
18. How is it tested?
19. How is it versioned?

A feature should enter the core only after these questions are answered.

---

234. Architectural test for new keywords

A new keyword MUST demonstrate that:

- it represents a compiler-wide language concept;
- identifier syntax is insufficient;
- contextual syntax is insufficient;
- dialect/library mechanisms are insufficient;
- its semantics require universal compiler understanding.

Otherwise it SHOULD remain an identifier or extension-level construct.

---

235. Architectural test for new directories

A new directory MUST demonstrate:

- distinct ownership;
- sufficient feature scope;
- stable integration boundary;
- independent tests;
- specification needs;
- maintainability benefit.

Otherwise the feature SHOULD remain in its existing owner.

---

236. Architectural test for new IR

A new IR MUST NOT be created merely because a domain is new.

A new IR is justified only if:

- the existing canonical IR cannot represent the semantics;
- a domain-specific optimization boundary genuinely requires it;
- explicit lowering into the canonical architecture exists.

There MUST NOT be competing canonical IRs.

---

237. Architectural test for physical features

A physical hardware feature SHOULD enter the language as:

capability
resource
constraint
policy
attribute
dialect

before becoming a universal syntax construct.

---

238. Architectural test for AI features

An AI feature SHOULD enter as:

semantic operation
library
model
dialect
capability
effect

rather than a new universal application keyword.

---

239. Architectural test for quantum features

A quantum feature SHOULD enter as:

operation metadata
capability
resource requirement
semantic operation

rather than a hard-coded gate production when the generic operation model is sufficient.

---

240. Architectural test for interoperability features

External formats SHOULD enter through:

dialect
parser
adapter
interoperability layer

rather than becoming universal core syntax.

---

241. Final architecture

The resulting architecture is:

                         Zamani Language
                               │
                    ┌──────────┴──────────┐
                    │                     │
                Universal Core        Domain Systems
                    │                     │
          ┌─────────┼─────────┐     ┌─────┼─────┐
          │         │         │     │     │     │
        Types    Operations  Values Classical Quantum HDL
          │         │         │     │     │     │
          └─────────┼─────────┘     └─────┼─────┘
                    │                     │
          ┌─────────┴─────────────────────┴─────────┐
          │                                          │
       Effects   Capabilities   Resources   Contracts
          │                                          │
          └───────────────┬──────────────────────────┘
                          │
                       Policies
                          │
                      Provenance
                          │
                   Semantic Analysis
                          │
                  Canonical Semantic Model
                          │
             ┌────────────┴─────────────┐
             │                          │
       Classical IR                 quantum::ir
             │                          │
             └────────────┬─────────────┘
                          │
                     Optimization
                          │
                       Lowering
                          │
                  Routing / Scheduling
                          │
                  Resilience / Recovery
                          │
                          ZQN
                          │
                          HAL
                          │
          ┌───────────────┼────────────────┐
          │               │                │
         CPU             GPU              FPGA
          │               │                │
         ASIC         Accelerator          QPU
          │               │                │
       Embedded          HPC           Simulator
          │               │                │
          └───────────────┼────────────────┘
                          │
                 Cluster / Distributed
                          │
                     Future Targets

---

242. Final non-negotiable invariants

The following are architectural invariants.

Invariant 1 — One language

There is one Zamani language, not a collection of competing languages.

Invariant 2 — One architecture authority

"grammar/DESIGN.md" owns grammar architecture.

Invariant 3 — One normative specification authority

"grammar/specification/" owns normative human-readable language meaning.

Invariant 4 — One machine-contract layer

"grammar/spec/" owns machine-oriented contracts and conformance metadata.

Invariant 5 — One canonical grammar root

"grammar/Zamani.g4" is the canonical ANTLR composition root.

Invariant 6 — One lexical architecture

"grammar/lexer/", "grammar/antlr/ZamaniLexer.g4", and "src/lexer.rs" MUST remain reconciled.

Invariant 7 — One domain-neutral AST architecture

ASTs MUST preserve language meaning rather than physical implementation.

Invariant 8 — One canonical classical IR

Classical semantics MUST converge through the canonical classical IR.

Invariant 9 — One canonical quantum IR

Quantum semantics MUST converge through "quantum::ir".

Invariant 10 — No artificial universal limits

The language MUST NOT hard-code universal physical capacity limits.

Invariant 11 — Resource/capability separation

Resources and capabilities MUST remain distinct concepts.

Invariant 12 — Effect separation

Effects MUST remain distinct from capabilities and resources.

Invariant 13 — Contract separation

Contracts MUST remain distinct from policies.

Invariant 14 — Policy separation

Policies MUST govern authorization and realization without replacing semantic meaning.

Invariant 15 — Provenance

Important semantic decisions and transformations MUST be traceable where required.

Invariant 16 — Safe Rust

Production Rust implementation MUST use no "unsafe".

Invariant 17 — Target independence

Portable source semantics MUST NOT depend on a specific physical target.

Invariant 18 — Extensibility

New hardware, quantum operations, AI algorithms, dialects, and computational paradigms MUST be addable without arbitrary universal grammar rewrites.

Invariant 19 — Determinism

Frontend and semantic processing MUST be deterministic for equivalent inputs and configuration.

Invariant 20 — Explicit failure

Unavailable resources or capabilities MUST produce explicit diagnostics rather than silent semantic degradation.

Invariant 21 — Independent-file completion

Every production grammar file MUST have a complete ownership, dependency, semantic, IR, testing, compatibility, and integration contract before being considered complete.

Invariant 22 — No duplicate subsystem authority

No subsystem may silently establish a second owner for a universal semantic concept.

Invariant 23 — Application neutrality

Application-specific functionality belongs in libraries, dialects, capabilities, policies, services, or applications rather than the universal keyword set.

Invariant 24 — Semantic preservation

Compilation, optimization, lowering, routing, scheduling, specialization, simulation, distribution, and target realization MUST preserve declared program meaning.

Invariant 25 — Open-ended scale

The language architecture MUST remain capable of expressing computations whose resource requirements grow with available resources rather than being bounded by arbitrary language-level constants.

---

243. Definition of DONE for "grammar/DESIGN.md"

This document is complete when:

- the repository authority hierarchy is unambiguous;
- the lexer/parser/AST relationship is explicit;
- the specification/implementation relationship is explicit;
- resource/capability/effect/contract/policy/provenance ownership is explicit;
- classical IR ownership is explicit;
- "quantum::ir" ownership is explicit;
- POCO-REAF semantics are explicit;
- target realization is separated from source meaning;
- artificial universal capacity limits are prohibited;
- safe Rust requirements are explicit;
- scalability semantics are explicit;
- extensibility rules are explicit;
- dialect rules are explicit;
- application-specific functionality is kept outside the universal core;
- independent-file completion contracts are mandatory;
- testing and conformance requirements are explicit;
- compatibility and versioning requirements are explicit;
- production-readiness criteria are explicit.

No downstream grammar file may contradict these architectural invariants.

---

244. Final architectural statement

Zamani grammar is not intended to describe a fixed machine.

It describes a portable computational language whose semantic meaning can be realized across different machines, scales, execution models, and future computational environments.

The fundamental abstraction is therefore:

Program Intent
      +
Types
      +
Operations
      +
Effects
      +
Capabilities
      +
Resources
      +
Constraints
      +
Contracts
      +
Policies
      +
Provenance
      ↓
Canonical Semantic Meaning
      ↓
Canonical IR
      ↓
Target-appropriate Realization

The programmer expresses what the computation means.

The compiler and runtime determine how that meaning can be realized under the available resources, capabilities, policies, topology, and target characteristics.

Consequently:

tiny machine
      │
      ├── same source semantics
      │
      ▼
larger machine
      │
      ├── same source semantics
      │
      ▼
heterogeneous accelerator
      │
      ├── same source semantics
      │
      ▼
quantum system
      │
      ├── same source semantics
      │
      ▼
HPC / cluster / distributed system
      │
      ├── same source semantics
      │
      ▼
future computational target

The physical realization changes.

The program's declared meaning does not.

That is the architectural foundation required for Program Once, Compile Once, Run Everywhere, Anywhere, Forever while retaining extensibility, safety, semantic correctness, resource awareness, quantum/classical/HDL integration, and long-term repository maintainability.
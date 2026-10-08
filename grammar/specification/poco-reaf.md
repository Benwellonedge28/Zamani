Zamani POCO-REAF Specification

Path: "grammar/specification/poco-reaf.md"
Status: Normative
Specification class: Language architecture / portability / compilation contract
Language: Zamani
Minimum implementation: Rust 1.97
Edition: Rust 2021
Safety model: Safe Rust only
Primary objective: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)

---

1. Purpose

This document defines the normative POCO-REAF contract for Zamani.

POCO-REAF means that a Zamani program expresses its computational meaning independently of any particular machine, processor, accelerator, quantum processor, FPGA, ASIC, simulator, cluster, network, or deployment environment.

The same source program MUST be capable of being considered for different realizations without requiring source modification merely because the available computational substrate changes.

POCO-REAF therefore separates:

program meaning
    ↓
semantic requirements
    ↓
capabilities
    ↓
resources
    ↓
constraints
    ↓
policies
    ↓
portable semantic compilation
    ↓
target realization

The central invariant is:

«Target variation MUST change realization, not the meaning of a valid portable program.»

POCO-REAF does not require every program to execute on every physical target.

A target may lack:

- required resources;
- required capabilities;
- required precision;
- required timing guarantees;
- required topology;
- required execution modes;
- required security properties;
- required quantum resources;
- required memory;
- required communication facilities;
- required accelerator support;
- required runtime services.

When this occurs, the implementation MUST report the incompatibility explicitly.

It MUST NOT silently change the program's semantics merely to make execution possible.

---

2. Normative Language

The following terms are normative:

- MUST
- MUST NOT
- REQUIRED
- SHALL
- SHALL NOT
- SHOULD
- SHOULD NOT
- MAY
- OPTIONAL

A conforming implementation MUST interpret these terms according to their normative meaning.

---

3. Specification Authority

This document is the normative authority for POCO-REAF semantics.

It MUST be interpreted together with:

grammar/specification/grammar-authority.md
grammar/specification/language.md
grammar/specification/language-principles.md
grammar/specification/language-scope.md
grammar/specification/lexical.md
grammar/specification/syntax.md
grammar/specification/syntax-model.md
grammar/specification/semantics.md
grammar/specification/semantic-model.md
grammar/specification/types.md
grammar/specification/scalability-model.md
grammar/specification/portability.md
grammar/specification/compilation-model.md
grammar/specification/execution-model.md
grammar/specification/compatibility.md
grammar/specification/language-version.md

and the detailed domain specifications under:

grammar/spec/

Concrete syntax is implemented through the grammar architecture headed by:

grammar/Zamani.g4
grammar/antlr/ZamaniLexer.g4
grammar/antlr/ZamaniParser.g4

where applicable in the current repository architecture.

The executable Rust frontend remains authoritative for actual implementation conformance:

src/lexer.rs
src/parser.rs
src/ast/
src/semantic.rs
src/ir_gen.rs
src/ir_verify.rs

The canonical quantum semantic boundary is:

src/quantum/ir/

and is referred to throughout this specification as:

quantum::ir

The canonical classical semantic representation is the repository's existing Classical IR.

This document MUST NOT create a competing IR.

---

4. Scope

This specification governs:

1. source portability;
2. semantic portability;
3. compilation portability;
4. target-independent semantic artifacts;
5. capability negotiation;
6. resource negotiation;
7. target specialization;
8. optimization;
9. lowering;
10. routing;
11. scheduling;
12. deployment;
13. execution;
14. runtime adaptation;
15. resilience;
16. reproducibility;
17. deterministic compilation;
18. compatibility;
19. provenance;
20. diagnostics;
21. quantum portability;
22. classical portability;
23. HDL/hardware portability;
24. heterogeneous execution;
25. distributed execution;
26. simulation;
27. AI and reasoning portability;
28. controlled adaptation;
29. policy enforcement;
30. security boundaries.

This specification does not own the implementation of:

- hardware discovery;
- target drivers;
- QEC algorithms;
- routing algorithms;
- scheduling algorithms;
- optimization algorithms;
- backend code generation;
- machine-code generation;
- physical calibration;
- runtime implementation;
- operating-system services;
- physical device management.

Those systems consume and realize the contracts defined here.

---

5. Fundamental POCO-REAF Principle

Zamani follows this invariant:

SOURCE MEANING
      ↓
PORTABLE SEMANTIC MODEL
      ↓
CANONICAL IR
      ↓
TARGET REALIZATION

Never:

SOURCE
  ↓
TARGET-SPECIFIC MEANING

The source language describes what the computation means.

The target layer determines how that computation can be realized.

---

6. The Five Portability Layers

POCO-REAF distinguishes five different kinds of portability.

6.1 Source Portability

The same valid source program can be parsed and semantically interpreted by conforming implementations without being rewritten merely for another target.

6.2 Semantic Portability

The program retains the same defined meaning across compatible target environments.

6.3 Compilation Portability

The semantic representation can be compiled, transformed, serialized, cached, transported, or reused across compatible compiler and target environments.

6.4 Execution Portability

The program can execute on a target when the target satisfies its declared requirements and the implementation has a conforming realization.

6.5 Evolution Portability

Existing valid programs remain meaningful as Zamani, hardware, execution environments, and computational domains evolve.

These five forms MUST NOT be conflated.

---

7. Program Once

7.1 Definition

"Program Once" means that programmers describe computation using Zamani's semantic model rather than encoding accidental assumptions about a particular machine.

A source program MAY express:

- computation;
- types;
- values;
- control flow;
- concurrency;
- parallelism;
- quantum computation;
- hardware intent;
- HDL intent;
- AI computation;
- reasoning;
- learning;
- adaptation;
- uncertainty;
- contracts;
- policies;
- provenance;
- resource requirements;
- capability requirements;
- constraints;
- preferences;
- optimization intent;
- timing requirements;
- precision requirements;
- security requirements;
- deployment intent.

The program MUST NOT need to be rewritten merely because the target changes its:

- processor;
- ISA;
- core count;
- memory capacity;
- accelerator availability;
- GPU architecture;
- FPGA family;
- ASIC implementation;
- quantum architecture;
- qubit topology;
- device count;
- network topology;
- storage capacity;
- execution provider;
- deployment location.

---

8. Compile Once

8.1 Definition

"Compile Once" means that the source can be compiled into a reusable, versioned, target-independent semantic representation.

Compilation MUST preserve:

- source meaning;
- type meaning;
- effects;
- capabilities;
- resource requirements;
- contracts;
- policies;
- provenance;
- domain semantics.

The reusable compilation artifact MAY then be specialized for different target environments.

Therefore:

source
   ↓
frontend
   ↓
domain-neutral AST
   ↓
semantic analysis
   ↓
canonical semantic representation
   ↓
reusable compilation artifact

A target-specific realization MAY subsequently perform:

specialization
optimization
decomposition
routing
scheduling
lowering
code generation
deployment

This is still consistent with "Compile Once".

---

8.2 What Compile Once Does Not Mean

POCO-REAF MUST NOT require:

«one immutable machine-code binary that executes unchanged on every architecture.»

Such a requirement would conflict with the existence of different:

- instruction sets;
- quantum instruction models;
- FPGA fabrics;
- ASIC implementations;
- accelerator interfaces;
- memory systems;
- execution models.

Instead:

«One source-level semantic program MUST have one stable meaning, while target-specific realization MAY differ.»

---

9. Run Everywhere

A program is executable on a target when all mandatory conditions are satisfied.

Conceptually:

Program
  +
Types
  +
Effects
  +
Capabilities
  +
Resources
  +
Contracts
  +
Policies
  +
Target
  =
Feasible Realization

A target MUST NOT be considered feasible merely because it can parse the source.

A conforming implementation MUST establish, where applicable:

1. type validity;
2. effect validity;
3. capability satisfaction;
4. resource sufficiency;
5. contract compatibility;
6. policy authorization;
7. security compatibility;
8. semantic lowering availability;
9. required runtime support.

---

10. Run Anywhere

A conforming Zamani implementation MAY realize a program through:

- local execution;
- embedded execution;
- native CPU execution;
- multicore execution;
- GPU execution;
- FPGA execution;
- ASIC execution;
- accelerator execution;
- quantum execution;
- quantum simulation;
- classical simulation;
- distributed execution;
- cluster execution;
- HPC execution;
- cloud execution;
- heterogeneous execution;
- emulation;
- future execution substrates.

The existence of a new target class MUST NOT require changing the meaning of existing portable programs.

---

11. Run Forever

"Forever" is a compatibility and evolution property.

It does not mean that:

- every historical device remains available;
- every compiler remains operational;
- every runtime remains supported;
- physical hardware remains operational indefinitely.

It means that Zamani's semantic model is designed so that future implementations can continue to interpret existing valid programs without requiring those programs to encode assumptions about future hardware that does not yet exist.

Evolution MUST therefore be governed by:

grammar/specification/language-version.md
grammar/specification/compatibility.md
grammar/compatibility/

and the associated language, AST, semantic, dialect, target, and IR version contracts.

---

12. Meaning Versus Realization

The following concepts are distinct.

Concept| Meaning
Requirement| Something the program needs for correctness
Capability| Something the target can provide
Constraint| A realization that is forbidden
Preference| A realization preferred by the program
Hint| Information useful to optimization but not semantic
Budget| An explicitly declared resource bound
Target fact| Information discovered about a target
Policy| Rules governing allowed behavior
Contract| Correctness obligations
Provenance| Evidence of where information or decisions originated

A target fact MUST NOT automatically become program semantics.

For example:

requires capability("quantum.measurement")

does not mean:

use quantum device X

Similarly:

requires memory >= required_memory

does not define the amount of memory that every Zamani machine must have.

---

13. Resource Independence

Resource quantities MUST remain open-ended.

The language MUST NOT define universal limits for:

- qubits;
- logical qubits;
- physical qubits;
- CPUs;
- cores;
- threads;
- GPUs;
- FPGAs;
- ASICs;
- accelerators;
- devices;
- nodes;
- processes;
- tasks;
- channels;
- memory;
- storage;
- network endpoints;
- tensor dimensions;
- tensor rank;
- matrix dimensions;
- vector lengths;
- circuit depth;
- module count;
- source size;
- deployment size.

The following classes of limits are legitimate:

1. physical limitations;
2. target capability limits;
3. runtime limits;
4. explicitly selected budgets;
5. security policies;
6. implementation representation limits;
7. test-fixture limits.

Such limits MUST NOT become universal Zamani language semantics.

---

14. No Artificial Capacity Ceiling

The grammar and language specification MUST NOT establish an artificial universal capacity.

Forbidden architecture:

MAX_QUBITS = ...
MAX_CPUS = ...
MAX_GPUS = ...
MAX_FPGAS = ...
MAX_NODES = ...
MAX_MEMORY = ...
MAX_THREADS = ...
MAX_REGISTER_WIDTH = ...
MAX_TENSOR_RANK = ...
MAX_NETWORK_SIZE = ...
MAX_DEVICE_COUNT = ...

The same rule applies to disguised equivalents.

A constant is not automatically forbidden merely because it contains a numeric value.

The implementation MUST classify each capacity value as one of:

language semantics
explicit user budget
target capability
runtime limit
implementation representation limit
security limit
test fixture

Only the first category belongs to language semantics.

---

15. "Infinity" and Practical Limits

POCO-REAF uses "infinity" in the architectural sense of:

«No artificial finite language-level ceiling.»

It does not claim physically infinite:

- memory;
- processing power;
- bandwidth;
- qubits;
- storage;
- execution time;
- energy;
- communication;
- hardware.

A computation may therefore scale as far as the actual combination of:

program requirements
+
implementation representation
+
available resources
+
target capabilities
+
security policy
+
execution policy

permits.

The language itself MUST NOT impose an arbitrary smaller ceiling.

---

16. Parametric Scaling

Portable programs SHOULD describe scale through:

- values;
- parameters;
- types;
- resource expressions;
- capability expressions;
- data size;
- runtime discovery;
- policies;
- topology descriptions;
- algorithmic structure.

Examples of valid semantic intent include:

requires qubits >= required_qubits;
requires memory >= required_memory;
requires capability("quantum.measurement");
requires capability("gpu.compute");
requires capability("tensor.compute");
requires topology(required_topology);

and, where supported by the resource grammar:

prefer ...
constrain ...
allow ...
forbid ...

These describe requirements or realization policy.

They MUST NOT become hidden universal limits.

---

17. Canonical Compilation Pipeline

The repository's POCO-REAF pipeline is:

Zamani source
       │
       ▼
Lexer
       │
       ▼
ANTLR grammar
       │
       ▼
Domain-neutral AST
       │
       ▼
Name / module / import resolution
       │
       ▼
Type analysis
       │
       ▼
Effect analysis
       │
       ▼
Capability analysis
       │
       ▼
Resource analysis
       │
       ▼
Contract analysis
       │
       ▼
Policy analysis
       │
       ▼
Provenance analysis
       │
       ▼
Semantic model
       │
       ├───────────────┐
       ▼               ▼
Classical IR      quantum::ir
       │               │
       └───────┬───────┘
               ▼
     Target-independent
       optimization
               │
               ▼
       specialization
               │
               ▼
       routing / mapping
               │
               ▼
          scheduling
               │
               ▼
      resilience / QEC
               │
               ▼
             ZQN
               │
               ▼
        target lowering
               │
               ▼
              HAL
               │
               ▼
            runtime

The grammar MUST NOT bypass this architecture.

---

18. Grammar Boundary

The grammar owns:

- syntax;
- lexical composition;
- syntactic structure;
- declarations;
- expressions;
- statements;
- domain syntax;
- resource expressions;
- capability expressions;
- policy syntax;
- contract syntax;
- effect syntax;
- version syntax.

The grammar does NOT own:

- target discovery;
- physical allocation;
- hardware calibration;
- routing;
- scheduling;
- QEC algorithms;
- runtime adaptation algorithms;
- machine code;
- physical device control.

---

19. AST Boundary

The AST MUST remain domain-neutral at the common frontend boundary.

The AST MUST preserve enough information to reconstruct:

- source meaning;
- source spans;
- explicit requirements;
- capabilities;
- constraints;
- preferences;
- effects;
- contracts;
- policies;
- provenance;
- domain operations;
- type information required downstream.

The AST MUST NOT prematurely replace source intent with:

- physical device IDs;
- hardware addresses;
- physical qubit mappings;
- backend instructions;
- calibration data;
- machine-code instructions;
- scheduler state.

---

20. Semantic Boundary

Semantic analysis owns interpretation.

It MUST determine, as applicable:

- names;
- types;
- effects;
- capabilities;
- resources;
- contracts;
- policies;
- domain legality;
- portability properties;
- provenance relationships.

A syntactically valid program MAY still be semantically invalid.

A semantically valid program MAY still be physically infeasible on a selected target.

These are different failure classes.

---

21. Canonical IR Boundary

POCO-REAF requires canonical semantic representations.

Classical computation MUST converge into the repository's Classical IR.

Quantum computation MUST converge into:

quantum::ir

The grammar MUST NOT define another permanent quantum IR.

The frontend MAY create temporary AST representations.

Those representations MUST lower into the canonical semantic structures.

---

22. Quantum POCO-REAF

Quantum computation is a first-class Zamani domain.

Quantum source MUST be hardware-independent by default.

Hardware-specific intent is permitted only when explicitly expressed as part of the program's semantics or realization constraints.

---

22.1 Quantum Operation Model

Quantum operations MUST remain open-ended.

The language MUST NOT require a universal enumeration such as:

H
X
Y
Z
CNOT
...

to define the entire universe of quantum operations.

Quantum operations SHOULD be represented through the existing data-driven operation model:

operation specifier
targets
parameters
results
attributes
modifiers

New operations MAY be supplied through:

- libraries;
- dialects;
- metadata;
- registered operations;
- vendor extensions.

A new quantum operation MUST NOT require changing the universal language grammar unless genuinely new syntax is required.

---

22.2 Quantum Resource Identity

Source-level qubit references express program-level identity.

Physical qubit identity belongs to the quantum IR and target realization boundary.

The implementation MUST preserve the distinction between:

source qubit
logical qubit
canonical IR resource
physical qubit

No layer may silently reinterpret one identity as another.

---

22.3 Quantum Resource Scaling

Quantum programs MUST NOT be restricted by an arbitrary universal qubit count.

A program may legitimately require a particular number of logical qubits.

That is a property of the program.

It is not a universal limit of Zamani.

---

22.4 Quantum Realization

The realization path is:

quantum source intent
       ↓
quantum AST
       ↓
semantic validation
       ↓
quantum::ir
       ↓
optimization / decomposition
       ↓
logical-to-physical mapping
       ↓
routing
       ↓
scheduling
       ↓
QEC / resilience where required
       ↓
ZQN / target information
       ↓
target lowering
       ↓
QPU / simulator / other realization

A backend MUST NOT silently replace a semantically different quantum operation merely because its native operation set differs.

If exact realization is impossible, the compiler MUST diagnose the failure.

Approximation is allowed only when the program's semantics or explicit policy permits it.

The approximation MUST have an explicit correctness/error contract.

---

22.5 Quantum Measurement

Measurement is a semantic operation.

Its realization MAY depend on the target.

The language MUST preserve:

- measurement meaning;
- result type;
- result ordering;
- relevant probabilistic semantics;
- classical feed-forward semantics.

A target's physical measurement mechanism MUST NOT redefine the source meaning.

---

22.6 Quantum Dynamic Control

Where supported, POCO-REAF MUST permit semantic representation of:

- mid-circuit measurement;
- classical feed-forward;
- conditional quantum operations;
- dynamic circuits;
- reset;
- adaptive quantum control.

The compiler MAY realize these through target-specific mechanisms.

---

22.7 Quantum Error Correction

The grammar MAY express:

- error-correction intent;
- fault-tolerance requirements;
- logical-resource requirements;
- correctness constraints.

The grammar does not own QEC algorithms.

QEC owns:

- encoding;
- syndrome extraction;
- decoding;
- correction;
- logical error processing;
- code-specific realization.

The POCO-REAF layer only requires that the declared correctness semantics survive the realization.

---

22.8 ZQN

ZQN is downstream of the semantic program.

ZQN may provide information about:

- faults;
- noise;
- fault locations;
- leakage;
- loss;
- erasure;
- correlated faults;
- calibration-related conditions;
- target behavior.

POCO-REAF MUST NOT duplicate the ZQN model in the language grammar.

The relationship is:

program semantics
       ↓
quantum::ir
       ↓
target / ZQN information
       ↓
planning / resilience / realization

---

22.9 Quantum Scheduling

Scheduling owns:

- temporal placement;
- operation ordering;
- resource conflicts;
- timing;
- synchronization;
- execution slots;
- delays;
- alignment.

The grammar may express semantic timing requirements.

It MUST NOT hard-code target-specific:

- pulse durations;
- timing grids;
- channel counts;
- gate durations;
- calibration values.

---

23. Classical POCO-REAF

Classical computation MUST remain independent of physical machine size.

Zamani MUST support classical computation across:

- tiny embedded systems;
- single-core systems;
- multicore systems;
- CPUs;
- GPUs;
- accelerators;
- distributed systems;
- HPC systems;
- future computational systems.

The same semantics MAY be realized differently according to available capabilities.

The source MUST NOT depend on:

- fixed register width;
- fixed cache size;
- fixed core count;
- fixed vector width;
- fixed memory size.

---

24. HDL and Hardware POCO-REAF

HDL/hardware semantics MAY express:

- modules;
- ports;
- signals;
- registers;
- memories;
- clocks;
- timing;
- state machines;
- pipelines;
- interfaces;
- combinational behavior;
- sequential behavior;
- hardware parameters.

A design parameter that is part of the intended hardware behavior is semantic.

A physical implementation limit is not automatically semantic.

For example:

parameter width = application_width

is different from defining a universal maximum hardware width.

Hardware realization MUST remain downstream.

---

25. Hybrid Computation

Zamani MUST support composition of:

classical
quantum
HDL
hardware
AI
data
distributed
networking
accelerator
simulation

without creating separate language universes.

Examples include:

classical → quantum
quantum → classical
classical → accelerator
AI → quantum
quantum → AI
software → HDL
HDL → software
distributed → quantum
quantum → distributed

Each boundary MUST preserve:

- type correctness;
- effect correctness;
- capability requirements;
- resource requirements;
- contracts;
- policies;
- provenance.

Hybrid computation MUST converge into the existing canonical IR architecture rather than creating a separate hybrid IR.

---

26. Resources

The resource subsystem is governed by:

grammar/resources/
grammar/spec/resources.md

POCO-REAF depends on the resource model distinguishing:

requirement
capability
constraint
preference
hint
budget
target fact

Resource expressions MUST remain symbolic and extensible.

Examples:

requires qubits >= required_qubits;
requires memory >= required_memory;
requires capability("gpu.compute");
requires capability("quantum.measurement");
requires topology(required_topology);

Resource quantities MUST NOT be interpreted as universal language capacities.

---

27. Capability Negotiation

Capability negotiation occurs after parsing.

The pipeline is:

source requirement
       ↓
semantic capability expression
       ↓
target capability discovery
       ↓
capability matching
       ↓
feasibility result

A capability MAY describe:

- computation;
- quantum measurement;
- tensor computation;
- GPU execution;
- FPGA synthesis;
- network access;
- cryptography;
- simulation;
- distributed execution;
- native execution;
- foreign ABI support;
- learning;
- adaptation;
- reflection;
- code generation.

Capabilities MUST remain open-ended.

The language MUST NOT enumerate every future capability.

---

28. Preferences

Preferences are not requirements.

For example:

prefer gpu;

MAY influence target selection.

It MUST NOT mean:

gpu is required

unless the source explicitly declares a requirement.

The compiler MUST therefore distinguish:

required
preferred
permitted
forbidden
suggested

---

29. Constraints

A constraint limits acceptable realizations.

A constraint MUST NOT silently become a universal language limit.

For example:

constrain topology(required_topology);

may restrict realization.

It does not define the topology of every Zamani target.

---

30. Policies

Policies are governed by:

grammar/policies/
grammar/execution/policies.g4
grammar/spec/policies.md

Policies MAY govern:

- security;
- execution;
- resource selection;
- adaptation;
- deployment;
- simulation;
- target selection;
- fallback;
- permissions;
- prohibitions.

Policies MUST NOT silently alter the meaning of the source program.

---

31. Effects

Effects are governed by:

grammar/effects/
grammar/spec/effects.md

POCO-REAF requires effects to remain part of semantic portability.

Relevant effects may include:

- I/O;
- network;
- mutation;
- randomness;
- native execution;
- foreign execution;
- distributed execution;
- quantum measurement;
- learning;
- adaptation;
- reflection;
- code generation;
- simulation;
- hardware access.

An effect MUST NOT be silently removed merely because a target does not provide it.

The target must either satisfy the effect or report infeasibility.

---

32. Contracts

Contracts are governed by the validation and specification layers.

Relevant constructs include:

requires
ensures
invariant
assume
guarantee
property
assertion

Contracts MUST remain semantic obligations.

A target realization MUST preserve required contracts.

An optimization MUST NOT invalidate a contract.

An approximation MUST NOT claim exact contract satisfaction when only an explicitly permitted approximation contract is available.

---

33. Knowledge, Reasoning, Learning, and Adaptation

Zamani's generic computational model MAY include:

- inference;
- deduction;
- reasoning;
- knowledge;
- assertions;
- retraction;
- queries;
- learning;
- adaptation;
- uncertainty;
- evidence;
- explanation;
- provenance;
- decision records.

These capabilities MUST use the same:

types
effects
resources
capabilities
contracts
policies
provenance
IR

as other computational domains.

They MUST NOT create a second semantic universe.

---

33.1 Reasoning

Reasoning MUST remain portable.

A reasoning operation may consume:

- premises;
- evidence;
- relations;
- models;
- constraints;
- policies.

The semantic result MUST be independent of the particular CPU/GPU/QPU used to execute it.

---

33.2 Learning

Learning is an effectful semantic operation.

A learning operation MUST account for applicable:

- input;
- data;
- model;
- objective;
- resources;
- capabilities;
- effects;
- policy;
- provenance.

Specific algorithms SHOULD remain libraries, dialects, or semantic registrations rather than becoming an ever-growing core grammar enumeration.

---

33.3 Adaptation

Adaptation MUST be controlled.

Adaptation MUST NOT mean unrestricted self-modifying execution.

A conforming adaptation model MUST permit enforcement of:

policy
capability
effect
resource
authorization
provenance
contract

The semantic model is:

adaptation request
       ↓
authorization
       ↓
policy evaluation
       ↓
resource/capability validation
       ↓
controlled change
       ↓
verification
       ↓
provenance record
       ↓
continued execution

---

33.4 Evidence

Evidence MAY support:

- reasoning;
- learning;
- verification;
- compiler decisions;
- optimization decisions;
- target selection;
- security decisions;
- runtime decisions.

Evidence MUST retain provenance where the relevant subsystem requires it.

---

33.5 Explainability

Explanation MAY describe:

- program decisions;
- compiler transformations;
- target selection;
- resource allocation;
- optimization;
- quantum routing;
- hardware placement;
- security decisions;
- adaptive execution.

Explanation is not restricted to AI.

---

34. Determinism

POCO-REAF distinguishes:

deterministic parsing
deterministic semantic analysis
reproducible compilation
deterministic execution

These are separate properties.

Parsing MUST be deterministic with respect to:

- source;
- lexer configuration;
- grammar version;
- parser configuration.

Parsing MUST NOT depend on:

- hardware availability;
- runtime state;
- wall-clock time;
- random target selection;
- network state.

Where reproducible compilation is requested, the compiler MUST preserve the required compilation inputs and versions.

Runtime nondeterminism MAY exist when explicitly permitted by program semantics.

---

35. Reproducibility

A reproducible artifact MUST identify the information required to reproduce its semantic compilation.

This may include:

- language version;
- grammar version;
- compiler version;
- relevant dialect versions;
- relevant library versions;
- semantic configuration;
- optimization configuration;
- target-independent inputs;
- provenance;
- declared policies.

Target-specific realization may still differ when target facts differ.

Reproducibility MUST NOT be confused with identical physical execution.

---

36. Target Discovery

Target discovery belongs downstream of parsing.

It may discover:

- CPU capabilities;
- GPU capabilities;
- FPGA capabilities;
- ASIC capabilities;
- QPU capabilities;
- simulator capabilities;
- memory;
- topology;
- timing;
- communication;
- energy;
- reliability;
- security;
- available runtime services.

Target discovery MUST NOT modify source semantics.

---

37. Target Selection

Target selection consumes:

program requirements
program constraints
program preferences
program policies
target capabilities
target resources
target state

A selected target MUST be compatible with all mandatory source requirements.

If multiple targets satisfy the requirements, selection MAY be influenced by preferences and policies.

---

38. Specialization

Specialization MAY adapt a semantic artifact to:

- known resource quantities;
- known target capabilities;
- known data shapes;
- known execution environments;
- known topology.

Specialization MUST preserve source semantics.

Specialization MUST NOT turn an optional target property into a mandatory property of the source language.

---

39. Optimization

Optimization MAY change implementation while preserving semantics.

Optimization may include:

- algebraic simplification;
- common-subexpression elimination;
- loop transformations;
- tensor optimization;
- quantum operation simplification;
- gate decomposition;
- memory optimization;
- parallelization;
- vectorization;
- accelerator mapping.

Optimization preferences MUST be distinguishable from correctness requirements.

---

40. Lowering

Lowering transforms canonical semantic operations into a target-specific representation.

The lowering chain MAY include:

Classical IR
      ↓
target representation

and:

quantum::ir
      ↓
quantum target representation

and:

HDL/hardware semantic representation
      ↓
synthesis / target representation

Lowering MUST preserve required semantics.

---

41. Routing

Routing maps logical computation onto available physical resources.

Routing MAY include:

- quantum logical-to-physical mapping;
- distributed placement;
- accelerator placement;
- memory placement;
- communication routing.

Routing MUST operate downstream of the canonical semantic model.

The source grammar MUST NOT own routing algorithms.

---

42. Scheduling

Scheduling MAY determine:

- execution order;
- resource sharing;
- timing;
- synchronization;
- communication;
- quantum operation timing;
- accelerator execution;
- distributed execution.

Scheduling MUST use target facts rather than embedding them into universal language semantics.

---

43. Resilience

The repository's resilience architecture owns adaptation to execution conditions.

POCO-REAF-compatible resilience MAY respond to:

- resource degradation;
- target failure;
- backend failure;
- quantum faults;
- unavailable services;
- network failure;
- runtime errors.

Existing resilience states such as:

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

remain downstream execution/resilience concepts.

POCO-REAF requires that any adaptation preserve source correctness.

---

44. No Silent Semantic Substitution

The following behavior is prohibited:

requested operation unavailable
        ↓
silently use a different operation
        ↓
claim success

Instead:

requested operation unavailable
        ↓
search for conforming realization
        ↓
if one exists → use it
if approximation is explicitly permitted → use permitted approximation
otherwise → report failure

This rule applies to:

- quantum operations;
- numerical operations;
- precision;
- timing;
- memory;
- communication;
- security;
- AI model behavior;
- contracts;
- hardware operations.

---

45. Approximation

Approximation MUST be explicit.

A target MAY use an approximate realization only when the source semantics or policy permits it.

The implementation MUST preserve the relevant:

- error bound;
- confidence;
- probability;
- tolerance;
- fidelity;
- precision;
- correctness contract.

A target MUST NOT silently downgrade exact semantics into approximate semantics.

---

46. Simulation

Simulation is an execution strategy, not a separate language.

The same source semantic model MAY be realized through:

- classical simulation;
- quantum simulation;
- hardware simulation;
- distributed simulation;
- fault simulation;
- performance simulation.

Simulation MUST preserve the semantic distinction between:

simulated resource
physical resource

A simulation result MUST NOT automatically be treated as evidence that a physical target can realize the computation.

---

47. Sandboxing

Sandboxing is governed by the security and execution policy systems.

A sandbox MAY constrain:

- I/O;
- network;
- filesystem;
- native calls;
- foreign calls;
- reflection;
- code generation;
- adaptation;
- resource consumption;
- device access.

Sandboxing MUST NOT change the meaning of permitted operations.

If an operation is prohibited, execution MUST fail or follow an explicitly defined policy path.

---

48. Foreign Interfaces

FFI/ABI boundaries are explicit interoperability boundaries.

A foreign interface MUST identify applicable:

- ABI;
- calling convention;
- data representation;
- ownership;
- effects;
- capabilities;
- target assumptions.

Foreign code is not automatically portable.

The foreign boundary MUST NOT force foreign machine limitations into Zamani's universal language semantics.

---

49. Dialects

Dialects MAY extend Zamani for:

- specialized domains;
- vendors;
- experimental hardware;
- scientific domains;
- data formats;
- interoperability formats;
- future computational models.

A dialect MUST define:

- identity;
- version;
- syntax ownership;
- semantic ownership;
- capabilities;
- lowering;
- compatibility;
- provenance.

A dialect MUST NOT silently redefine core Zamani semantics.

A new domain SHOULD use existing universal primitives whenever possible.

---

50. Application-Specific Functionality

Application functionality MUST normally be provided through:

- libraries;
- dialects;
- capabilities;
- policies;
- services;
- modules;
- applications.

The core language MUST NOT become an ever-growing enumeration of application-specific keywords.

The same principle applies to:

- domain services;
- administrative functions;
- business systems;
- scientific packages;
- robotics;
- vision;
- language processing;
- payment systems;
- legal workflows;
- virtual/augmented environments;
- specialized AI systems.

The universal core remains computational.

---

51. Distributed POCO-REAF

Distributed computation MUST remain independent of a fixed node count.

The language MAY express:

- actors;
- tasks;
- channels;
- services;
- messages;
- collectives;
- distributed state;
- topology requirements;
- consistency requirements;
- fault-tolerance requirements.

The implementation MUST NOT establish a universal number of nodes.

An explicit application requirement for a particular scale is valid.

That requirement is not a language capacity.

---

52. Concurrency and Actors

AI agents and other agents MUST integrate with the existing concurrency architecture rather than create a second actor runtime.

The intended relationship is:

agent
  ↓
actor
  ↓
message
  ↓
channel
  ↓
scheduler
  ↓
runtime

Agent-specific semantics MAY be defined in the AI layer.

Actor lifecycle and message semantics remain owned by the concurrency subsystem.

---

53. Data and Interoperability

Data formats such as:

- SQL;
- JSON;
- XML;
- external schemas;

MUST remain dialect/interoperability concerns where they are not core Zamani syntax.

Their semantic results SHOULD map into the common Zamani data/query model.

They MUST NOT impose fixed limits on:

- rows;
- columns;
- graph size;
- document size;
- tensor dimensions;
- schema size.

Actual limits are target or implementation limits.

---

54. Security and Trust

Security semantics MUST remain portable.

Target variation MUST NOT silently weaken:

- authorization;
- permissions;
- capability restrictions;
- cryptographic requirements;
- privacy requirements;
- trust policies;
- sandbox policies.

Credentials and secrets MUST remain external security material.

They MUST NOT become universal source semantics.

---

55. Provenance

POCO-REAF requires provenance to remain available where semantic decisions depend on transformations or external facts.

Relevant provenance may include:

source
    ↓
AST
    ↓
semantic decision
    ↓
optimization
    ↓
specialization
    ↓
routing
    ↓
scheduling
    ↓
lowering
    ↓
artifact
    ↓
execution

A provenance record SHOULD identify:

- origin;
- transformation;
- reason;
- evidence;
- version;
- policy;
- target fact where applicable;
- verification status.

Target-specific facts MUST be distinguishable from source-defined facts.

---

56. Compatibility

POCO-REAF depends on compatibility across:

- language versions;
- grammar versions;
- lexer versions;
- AST versions;
- semantic versions;
- dialect versions;
- Classical IR versions;
- quantum::ir versions;
- target compatibility versions.

The compatibility architecture under:

grammar/compatibility/

is authoritative for version migration.

A new target MUST NOT require rewriting a source program merely because it is new.

A breaking semantic change MUST be explicitly versioned.

---

57. Semantic Stability

The appearance of new hardware MUST NOT change the meaning of existing programs.

For example, adding a new quantum processor MUST NOT cause an existing operation to acquire a different mathematical meaning.

Adding a new GPU MUST NOT redefine the semantics of a tensor operation.

Adding a new FPGA MUST NOT redefine a hardware interface.

New target capabilities may create new realizations.

They do not rewrite historical semantics.

---

58. Target Compatibility

A target is compatible when it can satisfy all mandatory source obligations.

Compatibility is evaluated over:

types
effects
capabilities
resources
contracts
policies
precision
timing
security
execution model
domain semantics

A target may be:

fully compatible
partially compatible
conditionally compatible
incompatible
temporarily unavailable

The exact classifications belong to the target/execution compatibility model.

---

59. Failure Model

POCO-REAF requires structured failures.

The implementation MUST distinguish at least:

Syntax failure

The source is malformed.

Semantic failure

The source is structurally valid but semantically invalid.

Capability failure

The target lacks a required capability.

Resource failure

The target cannot provide required resources.

Policy failure

Execution violates a policy.

Contract failure

A required correctness obligation cannot be established.

Lowering failure

No conforming target realization exists.

Routing failure

Required physical mapping cannot be established.

Scheduling failure

Required execution schedule cannot be constructed.

Runtime failure

Execution failed after a valid realization was established.

These failures MUST NOT be collapsed into a generic parser error.

---

60. Resource Insufficiency

A valid program may require more resources than the selected target provides.

For example:

program requires N qubits
target provides fewer than N

This does not make the source invalid.

It means:

source = valid
target = insufficient
execution = infeasible on this target

The compiler MUST preserve this distinction.

---

61. Target Substitution

A target MAY substitute an implementation mechanism when semantic equivalence is established.

For example:

high-level operation
        ↓
target-specific implementation

is permitted.

However:

high-level operation
        ↓
semantically different operation

is forbidden unless the source explicitly permits the transformation.

---

62. Runtime Adaptation

Runtime adaptation MAY use current target facts such as:

- available processors;
- available accelerators;
- available memory;
- queue state;
- target health;
- network topology;
- quantum calibration;
- device availability;
- energy constraints.

Runtime adaptation MUST remain within:

- source semantics;
- contracts;
- policies;
- capabilities;
- resource constraints.

Runtime adaptation MUST NOT silently change program meaning.

---

63. Checkpointing

POCO-REAF MUST NOT imply that every computation can be checkpointed arbitrarily.

Checkpoint semantics MUST distinguish:

- classical state;
- compiled state;
- logical state;
- algorithmic state;
- provider-supported state;
- reconstructible state;
- quantum state.

Arbitrary physical quantum state serialization MUST NOT be assumed.

---

64. Recompilation

Target-specific recompilation MAY occur when required for realization.

This does not violate POCO-REAF if:

1. source remains unchanged;
2. semantic meaning remains unchanged;
3. canonical semantic compilation remains stable;
4. target-specific work occurs below the semantic boundary.

Examples:

same source
    ↓
same semantic artifact
    ├── CPU lowering
    ├── GPU lowering
    ├── FPGA lowering
    ├── QPU lowering
    └── simulator lowering

---

65. Caching

Compiled semantic artifacts MAY be cached.

Cache keys MUST include all semantically relevant inputs.

A cache MUST NOT reuse an artifact when doing so would violate:

- language compatibility;
- semantic compatibility;
- dialect compatibility;
- IR compatibility;
- required policy;
- required target facts.

Target-specific caches MUST remain distinguishable from target-independent semantic artifacts.

---

66. Deployment

Deployment is downstream of compilation.

Deployment MAY choose:

- local;
- embedded;
- remote;
- distributed;
- cluster;
- cloud;
- edge;
- accelerator;
- QPU;
- simulator.

Deployment decisions MUST NOT redefine source semantics.

---

67. Future Computational Domains

A future computational domain SHOULD integrate through:

syntax
   ↓
domain-neutral AST
   ↓
semantic model
   ↓
capability/resource/effect contracts
   ↓
canonical IR or existing canonical domain IR
   ↓
lowering

A future domain MUST NOT require arbitrary changes to the fundamental POCO-REAF contract.

This is the mechanism by which Zamani remains extensible beyond currently known computational technologies.

---

68. Rust Implementation Contract

The production implementation MUST target:

Rust 1.97 or later
Rust 2021 edition

The implementation MUST use Safe Rust.

The implementation MUST NOT require Rust "unsafe".

This applies to:

- lexer;
- parser;
- AST;
- semantic analysis;
- type analysis;
- resource analysis;
- effect analysis;
- capability analysis;
- IR generation;
- IR verification;
- quantum IR;
- optimization;
- compiler infrastructure;
- grammar tooling;
- production tests.

Safe scalable structures SHOULD include normal dynamically sized Rust collections such as:

Vec<T>
String
VecDeque<T>
HashMap<K, V>
HashSet<T>
BTreeMap<K, V>
BTreeSet<T>

where appropriate.

Fixed-size collections MUST NOT be introduced merely to impose artificial language capacity.

---

69. Rust Resource Limits

Rust's own representation limits are implementation limits.

They MUST NOT be presented as Zamani language semantics.

For example, a Rust integer type may have a finite representation.

That does not establish a universal Zamani resource limit.

Where larger quantities are semantically required, the implementation MUST use an appropriate representation rather than silently narrowing the language semantics.

---

70. Recursion and Deep Programs

Implementations MUST consider stack limitations when processing generated or deeply nested source structures.

Where practical, scalable compiler algorithms SHOULD use:

- iterative worklists;
- explicit stacks;
- streaming;
- incremental processing;
- bounded recursion where semantically appropriate.

A host-language stack limit MUST NOT become a language-level maximum.

---

71. Memory Scaling

Large Zamani programs MAY exceed the memory available to a particular compiler process.

That is an implementation/resource limitation.

The language MUST remain semantically unbounded with respect to arbitrary artificial source-size ceilings.

Implementations SHOULD support scalable techniques such as:

- incremental parsing;
- streaming where appropriate;
- compact representations;
- lazy processing;
- structural sharing where appropriate;
- incremental semantic analysis.

---

72. Compilation Scalability

The compiler SHOULD avoid algorithms whose complexity grows unnecessarily with total repository or program size.

Where possible:

- use indexed lookup;
- use dependency graphs;
- use worklists;
- cache semantic results;
- use incremental invalidation;
- preserve provenance;
- avoid repeated full-program rescans.

Compilation scalability is an implementation concern.

It MUST NOT be solved by reducing language capacity.

---

73. Deterministic Frontend

For identical:

source
lexer configuration
grammar version
language version
dialect set
frontend configuration

the frontend MUST produce equivalent AST and semantic results.

Frontend results MUST NOT depend on:

- hardware availability;
- random target selection;
- runtime queue state;
- wall-clock time;
- network availability.

---

74. Deterministic Compilation

When deterministic compilation is requested, all semantically relevant inputs MUST be fixed or recorded.

This includes:

- compiler version;
- language version;
- grammar version;
- dialect versions;
- library versions;
- optimization configuration;
- semantic configuration;
- source;
- declared policies.

Target-dependent compilation may additionally depend on target facts.

Those facts MUST be recorded when reproducibility requires them.

---

75. Performance Portability

POCO-REAF guarantees semantic portability, not identical performance.

The same program MAY have different:

- latency;
- throughput;
- energy use;
- memory use;
- communication cost;
- quantum fidelity;
- compilation time;
- execution time.

Performance differences MUST NOT be interpreted as semantic differences.

---

76. Resource-Aware Performance

Performance preferences MAY be expressed through the resource and compilation systems.

For example:

prefer capability("gpu.compute");
prefer lower_latency;
prefer energy_efficiency;

These preferences MUST remain distinguishable from correctness requirements.

---

77. Security-Aware Portability

A target that is faster but violates a required security policy is not a valid POCO-REAF realization.

Target selection MUST therefore consider:

performance
+
capability
+
resource
+
security
+
policy
+
correctness

not performance alone.

---

78. Provenance-Aware Compilation

Compiler transformations SHOULD preserve provenance sufficient to answer:

1. Where did this operation originate?
2. Why was this transformation applied?
3. Which requirement caused this realization?
4. Which target capability was used?
5. Which policy constrained the choice?
6. Which optimization changed the representation?
7. Which routing decision mapped the operation?
8. Which scheduling decision placed it?
9. Which verification step accepted it?

This is especially important for:

- quantum execution;
- AI decisions;
- safety-critical hardware;
- distributed systems;
- scientific computation;
- adaptive execution.

---

79. POCO-REAF and Contracts

A portable program is not merely syntax-compatible.

Its required contracts MUST survive target realization.

For example:

source
  ↓
optimization
  ↓
routing
  ↓
lowering
  ↓
execution

MUST preserve:

requires
ensures
invariant
guarantee
property

where those contracts are applicable.

---

80. POCO-REAF and Policies

Policies MAY prohibit a technically feasible realization.

Therefore:

resource feasible

does not necessarily imply:

policy permitted

The final realization MUST satisfy both.

---

81. POCO-REAF and Effects

Effects form part of the semantic contract.

A target that cannot provide a required effect is not a valid realization.

For example, if a program requires:

effect(network)

and the target is sandboxed from networking, the compiler/runtime MUST report the policy/capability conflict.

It MUST NOT silently remove the network operation.

---

82. POCO-REAF and AI

AI computation is not exempt from portability.

Models, inference, learning, reasoning, adaptation, uncertainty, and evidence MUST participate in the same semantic system.

AI implementations MAY differ across:

- CPU;
- GPU;
- accelerator;
- distributed system;
- simulator;
- quantum/classical hybrid system.

The semantic result MUST remain consistent with the declared model and contracts.

---

83. POCO-REAF and Learning

Learning introduces state and adaptation.

Therefore learning operations MUST have explicit semantic treatment for:

- mutable state;
- training data;
- randomness;
- model state;
- resource use;
- provenance;
- reproducibility;
- authorization where required.

A learning system MUST NOT silently alter unrelated program semantics.

---

84. POCO-REAF and Adaptation

Adaptation MUST be constrained.

The implementation MUST be able to establish:

who/what may adapt
what may adapt
why adaptation is allowed
which policy permits it
which resources may be consumed
which capabilities may be used
what contract must remain true
what provenance must be recorded

Unrestricted self-modification is not part of the POCO-REAF contract.

---

85. POCO-REAF and Uncertainty

Uncertainty MUST remain explicit.

Where the program expresses:

- probability;
- confidence;
- distribution;
- uncertainty;
- belief;
- approximation;

the compiler MUST preserve the semantic distinction between:

certain
probabilistic
approximate
unknown
unavailable

A target MUST NOT silently turn an uncertain result into a falsely exact result.

---

86. POCO-REAF and Explainability

Where explanation is requested or required, target-specific decisions SHOULD be explainable through provenance.

For example:

program
  ↓
required capability
  ↓
target selection
  ↓
optimization
  ↓
routing
  ↓
schedule

The implementation SHOULD be able to expose the relevant decision chain without exposing secrets.

---

87. POCO-REAF and Interoperability

External languages and formats remain explicit boundaries.

Examples include:

C
C++
Python
OpenQASM
Verilog
SystemVerilog
foreign ABIs
external runtimes
SQL
JSON
XML

Interop MUST preserve explicit information about:

- ABI;
- ownership;
- effects;
- target requirements;
- representation;
- compatibility;
- provenance.

External limitations MUST NOT become universal Zamani limits.

---

88. Target-Independent Semantic Artifact

A POCO-REAF implementation SHOULD be able to produce a target-independent artifact containing, where applicable:

source identity
language version
grammar version
AST/semantic version
semantic program
type information
effect information
capability requirements
resource requirements
constraints
preferences
contracts
policies
provenance
domain information
Classical IR
quantum::ir

This artifact is the principal meaning-preserving compilation product.

---

89. Target-Specific Artifact

A target-specific artifact MAY additionally contain:

target identity
target capability snapshot
target resource snapshot
specialization
optimization decisions
routing
schedule
lowering
backend representation
deployment information

Target-specific information MUST NOT be confused with the target-independent semantic artifact.

---

90. Artifact Layering

The architecture SHOULD distinguish:

Source Artifact
      ↓
Semantic Artifact
      ↓
Target Plan
      ↓
Target Artifact
      ↓
Deployment Artifact
      ↓
Runtime State

This prevents target facts from contaminating source semantics.

---

91. Cross-Compilation

Cross-compilation MUST be supported where a target backend exists.

The compiler MAY compile on one environment for execution on another.

The host environment MUST NOT automatically become the target semantics.

Host and target information MUST remain distinct.

---

92. Heterogeneous Compilation

A single program MAY use:

CPU + GPU
CPU + FPGA
CPU + QPU
CPU + accelerator
CPU + GPU + QPU
CPU + distributed cluster
GPU + FPGA
classical + quantum
software + HDL

The compiler MUST preserve the semantic relationships between components.

Each target component may have its own capabilities and resources.

The overall program remains one semantic program.

---

93. Multi-Target Realization

A semantic program MAY be realized across multiple targets simultaneously.

For example:

host CPU
   ↓
GPU computation
   ↓
QPU computation
   ↓
CPU post-processing
   ↓
distributed storage

The source program MUST remain one semantic unit.

The compiler/runtime may partition the realization.

---

94. Future Hardware

Future hardware MUST be introducible without requiring a universal grammar rewrite when existing semantic abstractions are sufficient.

The preferred extension mechanism is:

new capability
new resource
new dialect
new lowering
new backend
new target profile

not:

new universal machine keyword

---

95. Target Metadata

Target metadata MAY describe:

- resources;
- capabilities;
- topology;
- performance;
- timing;
- reliability;
- security;
- energy;
- supported dialects;
- supported IR versions.

Target metadata is not language syntax.

It is target information.

---

96. Target Facts and Source Facts

The compiler MUST distinguish:

source fact
target fact
derived fact

Example:

source:
requires qubits >= n

target:
provides qubits = m

derived:
m >= n

The target's "m" MUST NOT rewrite the source's "n".

---

97. Capability Failure

A capability failure SHOULD identify:

- capability required;
- target considered;
- capability missing;
- whether an alternative realization exists;
- whether simulation is available;
- whether another target may satisfy the requirement.

The diagnostic MUST NOT silently remove the operation.

---

98. Resource Failure

A resource failure SHOULD identify:

- resource required;
- amount required;
- amount available where safely reportable;
- resource type;
- relevant target;
- whether scaling or another target could satisfy the requirement.

The source remains valid.

---

99. Portability Does Not Mean Identical Representation

Different targets MAY use completely different physical representations.

For example:

same semantic operation
        ↓
CPU instructions
GPU kernel
FPGA circuit
ASIC logic
QPU operations
simulator operations

This is expected.

POCO-REAF protects semantic meaning, not representation identity.

---

100. Portability Does Not Mean Identical Performance

The following may differ:

runtime
latency
throughput
energy
memory usage
network usage
compilation time
queue time
quantum fidelity

These are target properties unless explicitly made part of program correctness.

---

101. Portable Hardware Intent

Hardware intent MAY be expressed at an abstraction level above a specific device.

Examples:

requires capability("tensor.compute");
requires capability("fpga.compute");
requires capability("quantum.measurement");
requires topology(required_topology);

A source program MAY constrain realization when necessary.

However, it SHOULD avoid unnecessary target-specific identities.

---

102. Explicit Target Dependence

A program MAY intentionally depend on a specific target capability or property.

When it does, that dependence MUST be explicit.

For example:

requires capability("vendor.specific.feature");

is semantically different from silently depending on the feature.

Explicit target dependence remains portable as a source artifact, but its execution domain becomes correspondingly narrower.

---

103. Portability Classes

A program MAY be classified as:

fully target-independent
capability-constrained
resource-constrained
topology-constrained
vendor-constrained
environment-constrained
non-portable by explicit design

These classifications describe the program's declared requirements.

They MUST NOT be inferred merely from the compiler's current target.

---

104. Production Conformance

A production POCO-REAF implementation MUST establish all of the following.

Frontend

- deterministic lexical analysis;
- deterministic parsing;
- source-span preservation;
- domain-neutral AST;
- version-aware parsing.

Semantic layer

- type validation;
- effect validation;
- capability validation;
- resource validation;
- contract validation;
- policy validation;
- provenance preservation.

IR

- Classical IR integration;
- "quantum::ir" integration;
- IR version compatibility;
- IR validation.

Compilation

- optimization;
- specialization;
- lowering;
- routing;
- scheduling;
- target realization.

Runtime

- capability reporting;
- resource reporting;
- structured failures;
- policy enforcement;
- resilience;
- provenance;
- reproducibility support.

---

105. Testing Contract

POCO-REAF MUST be tested at every layer.

105.1 Lexical Tests

Test:

- portable keywords;
- identifiers;
- numeric resource expressions;
- capability expressions;
- version syntax;
- dialect syntax.

105.2 Parser Tests

Test:

- minimal programs;
- classical programs;
- quantum programs;
- HDL programs;
- hybrid programs;
- distributed programs;
- resource requirements;
- capabilities;
- policies;
- contracts;
- provenance;
- adaptation;
- simulation.

105.3 Semantic Tests

Test:

- valid requirements;
- conflicting requirements;
- incompatible capabilities;
- insufficient resources;
- invalid policies;
- invalid contracts;
- invalid effects;
- invalid cross-domain operations.

105.4 IR Tests

Test:

source
→ AST
→ semantic model
→ Classical IR

and:

source
→ AST
→ semantic model
→ quantum::ir

where applicable.

105.5 Target Tests

The same semantic program MUST be evaluated against multiple target profiles.

Tests MUST distinguish:

valid + realizable
valid + resource insufficient
valid + capability unavailable
valid + policy prohibited
valid + lowering unavailable
invalid source

---

106. Scalability Tests

Scalability tests MUST NOT define an arbitrary value as the language's maximum.

Tests SHOULD use generated or parameterized workloads.

They MUST cover:

- increasing source size;
- increasing data size;
- increasing tensor dimensions;
- increasing quantum resource requirements;
- increasing task count;
- increasing distributed scale;
- increasing hardware modules;
- increasing compilation units.

The purpose is to verify absence of artificial grammar ceilings.

---

107. Cross-Domain Tests

At minimum, production testing SHOULD include:

classical
quantum
HDL
hardware
AI
data
distributed
networking
hybrid

and combinations such as:

classical + quantum
classical + HDL
quantum + HDL
quantum + distributed
AI + quantum
AI + classical
AI + hardware
classical + quantum + distributed
classical + quantum + HDL
classical + quantum + AI + hardware

---

108. POCO-REAF End-to-End Test

A mandatory integration test SHOULD represent one program containing, where supported:

classical computation
+
generic types
+
resource requirements
+
capabilities
+
effects
+
contracts
+
policy
+
provenance
+
reasoning
+
learning
+
controlled adaptation
+
uncertainty
+
parallelism
+
distributed execution
+
quantum computation
+
measurement
+
HDL/hardware intent
+
simulation

The program MUST be able to pass through:

lexer
→ parser
→ AST
→ semantic analysis
→ resource analysis
→ capability analysis
→ effect analysis
→ contract analysis
→ policy analysis
→ provenance
→ Classical IR
→ quantum::ir
→ optimization
→ routing
→ scheduling
→ resilience/QEC where applicable
→ ZQN where applicable
→ target lowering

without introducing a second semantic universe.

---

109. Determinism Tests

Identical:

source
language version
grammar version
dialect versions
frontend configuration

MUST produce equivalent frontend and semantic results.

Where deterministic compilation is requested, repeated compilation MUST produce equivalent semantic artifacts.

---

110. Compatibility Tests

Compatibility testing MUST cover:

- language versions;
- grammar versions;
- AST versions;
- semantic versions;
- Classical IR versions;
- "quantum::ir" versions;
- dialect versions;
- target compatibility versions.

Breaking changes MUST be detected explicitly.

---

111. Hard-Coding Audit

Every POCO-REAF implementation change MUST audit for accidental limits.

The audit MUST search for:

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

and equivalent disguised constants.

The existence of such a constant in an implementation does not automatically mean the architecture is wrong.

It MUST be classified.

For example:

target-specific capacity

is valid.

test fixture

is valid.

universal language capacity

is prohibited.

---

112. Existing Repository Limits

Existing implementation limits discovered under the repository's quantum, benchmarking, memory, resilience, or other subsystems MUST be treated as implementation/target/test constraints unless they are explicitly proven to be language semantics.

They MUST NOT be allowed to leak into:

grammar/Zamani.g4
grammar/specification/
grammar/spec/

as universal language capacity.

Where an existing limit currently has ambiguous ownership, it MUST be migrated to the appropriate:

target profile
resource budget
runtime policy
implementation safeguard
benchmark configuration

rather than becoming a POCO-REAF language limit.

---

113. Repository Integration Matrix

Repository component| POCO-REAF responsibility
"grammar/DESIGN.md"| Overall architecture and ownership
"grammar/README.md"| Navigation
"grammar/Zamani.g4"| Canonical grammar composition
"grammar/antlr/ZamaniLexer.g4"| Canonical lexical implementation
"grammar/specification/grammar-authority.md"| Authority hierarchy
"grammar/specification/language.md"| Language definition
"grammar/specification/language-scope.md"| Language scope
"grammar/specification/scalability-model.md"| Detailed scalability rules
"grammar/specification/portability.md"| General portability rules
"grammar/specification/compilation-model.md"| Compilation behavior
"grammar/specification/execution-model.md"| Execution behavior
"grammar/specification/compatibility.md"| Compatibility guarantees
"grammar/specification/language-version.md"| Language versioning
"grammar/spec/resources.md"| Resource semantics
"grammar/spec/effects.md"| Effect semantics
"grammar/spec/policies.md"| Policy semantics
"grammar/spec/quantum.md"| Quantum semantics
"grammar/spec/semantics.md"| General semantic model
"grammar/spec/type-system.md"| Type portability
"grammar/spec/determinism.md"| Determinism
"grammar/compile/"| Compilation syntax and intent
"grammar/resources/"| Resource syntax
"grammar/effects/"| Effect syntax
"grammar/policies/"| Policy syntax
"grammar/validation/"| Contract/validation syntax
"grammar/execution/"| Execution syntax
"grammar/quantum/"| Quantum syntax
"grammar/classical/"| Classical syntax
"grammar/hdl/"| HDL syntax
"grammar/hardware/"| Hardware intent
"grammar/hybrid/"| Hybrid syntax
"grammar/distributed/"| Distributed syntax
"grammar/networking/"| Networking syntax
"grammar/interoperability/"| Foreign/data interoperability
"grammar/compatibility/"| Compatibility machinery
"src/lexer.rs"| Executable lexer
"src/parser.rs"| Executable parser
"src/ast/"| Domain-neutral AST
"src/semantic.rs"| Semantic analysis
"src/ir_gen.rs"| Canonical IR lowering
"src/ir_verify.rs"| Canonical IR validation
Classical IR| Canonical classical semantics
"src/quantum/ir/"| Canonical quantum semantics
"src/quantum/zqn/"| Quantum noise/fault information
quantum routing subsystem| Quantum realization mapping
quantum scheduling subsystem| Quantum execution scheduling
quantum resilience subsystem| Quantum adaptation/recovery
QEC subsystem| Error correction
hardware HAL| Target realization
runtime| Execution and state reporting

---

114. Ownership Rule

Every concept MUST have one semantic owner.

Examples:

resource requirements → resource system
effects → effect system
contracts → validation/contract system
policies → policy system
provenance → provenance system
quantum semantics → quantum semantic layer / quantum::ir
classical semantics → Classical IR
routing → routing subsystem
scheduling → scheduling subsystem
QEC → QEC subsystem
noise/faults → ZQN
runtime adaptation → execution/resilience

A grammar file MAY compose a concept.

It MUST NOT create a second semantic definition for that concept.

---

115. No Duplicate IR

The repository MUST NOT introduce:

POCOIR
PortableIR
UniversalIR
QuantumPortableIR
HybridPortableIR
TargetPortableIR

merely to implement POCO-REAF.

The established canonical semantic representations remain authoritative.

POCO-REAF is an architectural property.

It is not another IR.

---

116. No POCO-REAF Keyword Requirement

POCO-REAF MUST NOT require a universal source keyword such as:

program_once
compile_once
run_everywhere

The property is provided by the architecture.

Source syntax only needs to express semantic requirements when such requirements are meaningful.

---

117. Source Stability Invariant

For a portable program "P":

Meaning(P, target_A)
=
Meaning(P, target_B)

whenever both targets satisfy the same required semantic contract.

Performance and representation may differ.

Meaning MUST NOT.

---

118. Realization Equivalence

Two target realizations are POCO-REAF equivalent when both satisfy the same required semantic contract.

They need not have:

- identical instructions;
- identical circuit decomposition;
- identical schedule;
- identical memory layout;
- identical execution time;
- identical device topology.

They MUST preserve the required observable semantics.

---

119. Observability

Observable behavior includes whatever the language semantics explicitly expose.

Depending on the program, this may include:

- return values;
- mutations;
- I/O;
- measurement outcomes;
- probabilistic behavior;
- communication;
- timing guarantees;
- contract results;
- errors;
- externally visible effects.

Target-specific internal implementation details are not automatically observable semantics.

---

120. Timing

Timing is semantic only when explicitly required.

A target-specific operation duration is normally a target fact.

A source-level deadline, latency bound, real-time requirement, or synchronization guarantee MAY be semantic.

A target that cannot satisfy a required timing contract is incompatible.

---

121. Precision

Precision requirements MUST be explicit.

A target MUST NOT silently reduce precision when the program requires higher precision.

If approximation is permitted, the relevant tolerance MUST be part of the semantic contract.

---

122. Numerical Portability

Numerical operations MAY have target-dependent implementations.

The semantic model MUST distinguish:

exact operation
approximate operation
floating-point operation
probabilistic operation
implementation-defined behavior

where relevant.

The compiler MUST NOT silently cross these semantic categories.

---

123. Quantum Numerical Portability

Quantum parameters and amplitudes MUST preserve the declared mathematical semantics within the program's numerical contract.

Target precision limitations MUST be reported when they violate that contract.

---

124. Distributed Portability

Distributed execution may vary in:

- node count;
- topology;
- communication latency;
- bandwidth;
- failure rate;
- consistency mechanisms.

The program remains portable when the target can satisfy the declared distributed semantics.

---

125. Network Portability

Network operations MUST carry appropriate:

- effect information;
- capability requirements;
- security policy;
- resource requirements.

Network topology is target information unless explicitly made part of program semantics.

---

126. Hardware Discovery

Hardware discovery MUST remain outside the parser.

The parser MUST NOT:

- inspect physical hardware;
- query device inventories;
- allocate memory;
- allocate qubits;
- discover GPUs;
- probe network topology.

Discovery occurs downstream.

---

127. Resource Negotiation

Resource negotiation MUST be performed after semantic analysis.

Conceptually:

program requirement
       ↓
resource expression
       ↓
target resource model
       ↓
negotiation
       ↓
feasible / infeasible

Negotiation MUST NOT modify the source AST merely because target capacity differs.

---

128. Resource Preferences

Preferences MAY be used to select among valid realizations.

They MUST NOT override mandatory requirements.

The priority is:

semantic correctness
    >
mandatory requirements
    >
mandatory constraints/policies
    >
capabilities/resources
    >
preferences
    >
optimization hints

Exact policy ordering MUST follow the relevant resource/policy specification where more detail is required.

---

129. Portability and Vendor Extensions

Vendor-specific features MAY be used explicitly.

They MUST be represented through:

- dialects;
- capabilities;
- explicit requirements;
- target metadata;
- interoperability contracts.

Vendor features MUST NOT silently become core Zamani requirements.

---

130. Portability and Future Targets

A future target may introduce:

- new instruction sets;
- new quantum models;
- new accelerator architectures;
- new memory systems;
- new execution models.

Existing source remains valid provided its semantic requirements can be satisfied.

The future target is responsible for providing the realization.

---

131. Portability and Language Evolution

When the language evolves, the compiler MUST distinguish:

old source
new compiler
new target

from:

old source
changed semantics

A compiler MUST NOT silently reinterpret old semantics merely because a new target has appeared.

---

132. Migration

When a breaking semantic change is necessary, the repository MUST provide an explicit migration path where practical.

Migration MUST be governed by:

grammar/compatibility/
grammar/specification/compatibility.md

Migration tooling MUST NOT silently change program meaning.

---

133. Diagnostics Contract

POCO-REAF diagnostics SHOULD contain structured information.

Where applicable:

diagnostic code
severity
source span
construct
requirement
constraint
capability
resource
target
policy
contract
provenance
remediation

Diagnostics MUST distinguish source invalidity from target infeasibility.

---

134. Error Stability

Diagnostic codes SHOULD remain stable across compatible versions.

Diagnostic wording MAY improve.

Machine-readable diagnostic identity SHOULD remain versioned where tooling depends on it.

---

135. Tooling Integration

Formatter:

- MUST preserve semantic constructs;
- MUST NOT rewrite target facts into source semantics.

LSP/IDE:

- SHOULD expose capability/resource diagnostics;
- SHOULD distinguish source errors from target errors;
- SHOULD expose provenance where available.

Static analysis:

- SHOULD inspect portability constraints;
- SHOULD identify target assumptions;
- SHOULD identify resource requirements.

Build tooling:

- SHOULD cache semantic artifacts;
- SHOULD record versions;
- SHOULD preserve reproducibility information.

---

136. Independent File Completion Contract

"grammar/specification/poco-reaf.md" is considered independently complete when its downstream files can be implemented without redefining the POCO-REAF principles here.

Each dependent file MUST integrate with this specification through its own explicit ownership contract.

The dependent file MUST NOT redefine:

- Program Once;
- Compile Once;
- semantic portability;
- target realization separation;
- no artificial universal capacity;
- canonical IR boundaries.

---

137. Integration Contract by Layer

Lexer

Consumes only syntax definitions.

Must not perform target discovery.

Parser

Builds syntax/AST.

Must not allocate resources.

AST

Preserves portable intent.

Must not become target-specific.

Semantic Analysis

Validates meaning.

Must integrate:

types
effects
capabilities
resources
contracts
policies
provenance

Classical IR

Owns classical computational semantics.

"quantum::ir"

Owns canonical quantum semantics.

Optimization

Preserves semantics.

Routing

Maps semantics to physical topology.

Scheduling

Maps operations to execution time/resources.

ZQN

Represents quantum noise/fault information.

QEC

Provides error-correction realization.

Resilience

Provides adaptation/recovery.

HAL

Exposes target capabilities and execution mechanisms.

Runtime

Executes and reports actual execution behavior.

---

138. Exact Integration With Existing Compilation Grammar

The POCO-REAF specification integrates with:

grammar/compile/compile.g4
grammar/compile/compilation.g4
grammar/compile/intent.g4
grammar/compile/profiles.g4
grammar/compile/features.g4
grammar/compile/feature-selection.g4
grammar/compile/conditional-compilation.g4
grammar/compile/target.g4
grammar/compile/target-selection.g4
grammar/compile/specialization.g4
grammar/compile/optimization.g4
grammar/compile/lowering.g4
grammar/compile/code-generation.g4
grammar/compile/cross-compilation.g4
grammar/compile/reproducibility.g4
grammar/compile/deterministic-builds.g4
grammar/compile/provenance.g4
grammar/compile/caching.g4
grammar/compile/deployment.g4

These files provide syntax/composition.

This specification provides the semantic portability contract.

They MUST NOT create a competing POCO-REAF definition.

---

139. Exact Integration With Resource Grammar

POCO-REAF integrates with:

grammar/resources/resource.g4
grammar/resources/resources.g4
grammar/resources/requirements.g4
grammar/resources/resource-expressions.g4
grammar/resources/capabilities.g4
grammar/resources/constraints.g4
grammar/resources/preferences.g4
grammar/resources/hints.g4
grammar/resources/budgets.g4
grammar/resources/negotiation.g4
grammar/resources/placement.g4
grammar/resources/topology-related resource rules
grammar/resources/scalability.g4

The resource subsystem owns the concrete resource syntax and semantic resource model.

This file only defines its POCO-REAF role.

---

140. Exact Integration With Effects

POCO-REAF integrates with:

grammar/effects/effects.g4
grammar/effects/effect-types.g4
grammar/effects/effect-sets.g4
grammar/effects/effect-composition.g4
grammar/effects/effect-declarations.g4
grammar/effects/quantum.g4
grammar/effects/measurement.g4
grammar/effects/learning.g4
grammar/effects/adaptation.g4
grammar/effects/simulation.g4
grammar/effects/foreign.g4
grammar/effects/native.g4
grammar/effects/network.g4
grammar/effects/distributed.g4
grammar/effects/reflection.g4
grammar/effects/code_generation.g4

Effects remain semantic obligations.

---

141. Exact Integration With Execution

POCO-REAF integrates with:

grammar/execution/adaptive.g4
grammar/execution/dispatch.g4
grammar/execution/distributed-execution.g4
grammar/execution/environments.g4
grammar/execution/execution-context.g4
grammar/execution/parallel-execution.g4
grammar/execution/placement.g4
grammar/execution/policies.g4
grammar/execution/recovery.g4
grammar/execution/resilience.g4
grammar/execution/runtime-capabilities.g4
grammar/execution/scheduling.g4
grammar/execution/simulation.g4
grammar/execution/speculative.g4
grammar/execution/tracing.g4

These implement execution behavior.

POCO-REAF constrains their behavior to preserve source semantics.

---

142. Exact Integration With Quantum

POCO-REAF integrates with:

grammar/quantum/
src/quantum/ir/
src/quantum/zqn/
src/quantum/routing/
src/quantum/scheduling/
src/quantum/resilience/
src/quantum/error_correction/

where those subsystems are present in the current architecture.

No quantum subsystem may create a competing POCO-REAF semantic model.

---

143. Exact Integration With Validation

POCO-REAF integrates with:

grammar/validation/

especially:

contracts
assertions
properties
assumptions
guarantees
semantic boundaries
portability
scalability
IR coverage
source spans
ambiguity

Validation is responsible for proving that the program satisfies the applicable structural and semantic requirements.

---

144. Exact Integration With Compatibility

POCO-REAF integrates with:

grammar/compatibility/

including:

language version
grammar version
AST version
semantic version
IR version
dialect version
target compatibility
migration
deprecation
feature gates
conformance

No compatibility mechanism may introduce an artificial resource ceiling.

---

145. Exact Integration With Classical IR

Classical source semantics MUST lower through the existing canonical Classical IR.

A new POCO-REAF construct MUST NOT create another classical IR merely because it introduces:

- AI;
- reasoning;
- data;
- tensor operations;
- distributed computation;
- hardware interaction.

Such semantics MUST reuse existing IR infrastructure where possible.

---

146. Exact Integration With "quantum::ir"

Quantum constructs MUST lower through:

src/quantum/ir/

The quantum IR remains responsible for canonical quantum semantic identity.

POCO-REAF MUST NOT define:

- another qubit identity;
- another quantum operation IR;
- another quantum resource identity;
- another quantum scheduling IR.

---

147. Exact Integration With ZQN

ZQN remains responsible for quantum fault/noise representation.

POCO-REAF only specifies how source semantics interact with target information.

The language grammar MUST NOT duplicate ZQN's fault ontology.

---

148. Exact Integration With QEC

QEC remains a downstream realization mechanism.

POCO-REAF MAY require fault-tolerance properties.

It MUST NOT prescribe one QEC algorithm.

---

149. Exact Integration With Routing

Routing remains downstream.

POCO-REAF expresses requirements and constraints.

Routing determines realization.

---

150. Exact Integration With Scheduling

Scheduling remains downstream.

POCO-REAF may express timing requirements.

Scheduling maps them onto target timing resources.

---

151. Exact Integration With Resilience

Resilience remains downstream.

POCO-REAF establishes:

adaptation must preserve semantics

The resilience subsystem determines:

retry
recover
remap
reroute
reschedule
recompile
switch target
abort

according to policy and target state.

---

152. Completion Criteria

"grammar/specification/poco-reaf.md" is complete when all of the following are satisfied:

Authority

- [ ] It is the normative POCO-REAF specification.
- [ ] It does not compete with "grammar/DESIGN.md".
- [ ] It does not compete with "grammar/specification/compilation-model.md".
- [ ] It does not compete with "grammar/specification/scalability-model.md".
- [ ] It does not compete with "grammar/specification/compatibility.md".

Portability

- [ ] Source portability is defined.
- [ ] Semantic portability is defined.
- [ ] Compilation portability is defined.
- [ ] Execution portability is defined.
- [ ] Evolution portability is defined.

Compilation

- [ ] Program Once is defined.
- [ ] Compile Once is defined.
- [ ] Universal machine-code assumptions are explicitly rejected.
- [ ] Canonical semantic compilation is defined.
- [ ] Target-specific realization is separated.

Scalability

- [ ] No artificial universal capacity is permitted.
- [ ] Resource scaling is symbolic/parametric.
- [ ] Physical limitations remain target/resource facts.
- [ ] Implementation limits cannot become language semantics accidentally.
- [ ] "Infinity" is defined as absence of artificial language ceilings.

Resources

- [ ] Requirements are distinct from capabilities.
- [ ] Constraints are distinct from preferences.
- [ ] Hints are distinct from requirements.
- [ ] Budgets are explicit.
- [ ] Target facts are distinct from source facts.

Quantum

- [ ] "quantum::ir" is canonical.
- [ ] Quantum operations remain open-ended.
- [ ] Qubit identity remains layered.
- [ ] Routing is downstream.
- [ ] Scheduling is downstream.
- [ ] QEC is downstream.
- [ ] ZQN is downstream.
- [ ] Quantum adaptation preserves semantics.

Classical

- [ ] Classical IR remains canonical.
- [ ] No fixed machine size is encoded.
- [ ] Numerical portability is addressed.
- [ ] Performance is distinguished from semantics.

HDL/Hardware

- [ ] Hardware intent is separated from hardware realization.
- [ ] No universal hardware capacity is defined.
- [ ] Target hardware facts remain downstream.

Hybrid

- [ ] Classical/quantum/HDL/hardware composition is supported.
- [ ] No competing hybrid IR is created.

AI and reasoning

- [ ] Reasoning participates in the semantic model.
- [ ] Learning participates in the effect/resource model.
- [ ] Adaptation is controlled.
- [ ] Evidence/provenance are preserved.
- [ ] Uncertainty remains explicit.
- [ ] Explanation is portable.

Execution

- [ ] Runtime adaptation is constrained.
- [ ] Resilience preserves semantics.
- [ ] Simulation is treated as an execution strategy.
- [ ] Sandboxing is policy-driven.

Compatibility

- [ ] Language versioning is addressed.
- [ ] AST compatibility is addressed.
- [ ] IR compatibility is addressed.
- [ ] Dialect compatibility is addressed.
- [ ] Target compatibility is addressed.

Safety

- [ ] Rust 1.97+ is required.
- [ ] Rust 2021 is required.
- [ ] Safe Rust is required.
- [ ] Rust "unsafe" is forbidden.

Testing

- [ ] Lexical tests are defined.
- [ ] Parser tests are defined.
- [ ] Semantic tests are defined.
- [ ] IR tests are defined.
- [ ] scalability tests are defined.
- [ ] cross-domain tests are defined.
- [ ] target-variation tests are defined.
- [ ] determinism tests are defined.
- [ ] compatibility tests are defined.
- [ ] POCO-REAF end-to-end tests are defined.

---

153. Final POCO-REAF Invariant

The complete Zamani architecture is:

                         ONE SOURCE PROGRAM
                                │
                                ▼
                       ONE PROGRAM MEANING
                                │
                                ▼
                         DOMAIN-NEUTRAL AST
                                │
                                ▼
                   ┌────────────┼────────────┐
                   │            │            │
                   ▼            ▼            ▼
                 TYPES       EFFECTS      POLICIES
                   │            │            │
                   └────────────┼────────────┘
                                │
                   ┌────────────┼────────────┐
                   │            │            │
                   ▼            ▼            ▼
             CAPABILITIES   RESOURCES    CONTRACTS
                   │            │            │
                   └────────────┼────────────┘
                                │
                                ▼
                           PROVENANCE
                                │
                                ▼
                         SEMANTIC MODEL
                                │
                  ┌─────────────┴─────────────┐
                  │                           │
                  ▼                           ▼
             Classical IR                 quantum::ir
                  │                           │
                  └─────────────┬─────────────┘
                                │
                                ▼
                         OPTIMIZATION
                                │
                                ▼
                         SPECIALIZATION
                                │
                                ▼
                           ROUTING
                                │
                                ▼
                         SCHEDULING
                                │
                                ▼
                      RESILIENCE / QEC
                                │
                                ▼
                              ZQN
                                │
                                ▼
                         TARGET LOWERING
                                │
                  ┌─────────────┼─────────────┐
                  │             │             │
                  ▼             ▼             ▼
                 CPU           GPU           FPGA
                  │             │             │
                  ├─────────────┼─────────────┤
                  │             │             │
                  ▼             ▼             ▼
                 ASIC       Accelerator       QPU
                  │             │             │
                  └─────────────┼─────────────┘
                                │
                 ┌──────────────┼──────────────┐
                 │              │              │
                 ▼              ▼              ▼
             Simulator         HPC        Distributed
                 │              │              │
                 └──────────────┼──────────────┘
                                │
                                ▼
                         FUTURE TARGETS
                                │
                                ▼
                             RUNTIME

The invariant is:

«One Zamani program has one semantic meaning.»

Target differences MAY change:

- representation;
- optimization;
- specialization;
- routing;
- scheduling;
- resource allocation;
- execution strategy;
- physical realization.

They MUST NOT silently change:

- program meaning;
- required correctness;
- declared effects;
- mandatory capabilities;
- mandatory resource requirements;
- contracts;
- security policies;
- explicit semantic guarantees.

Therefore:

PROGRAM ONCE
     ↓
SEMANTICALLY STABLE
     ↓
COMPILE ONCE
     ↓
REUSABLE CANONICAL SEMANTIC ARTIFACT
     ↓
TARGET-SPECIFIC REALIZATION
     ↓
RUN WHERE REQUIREMENTS ARE SATISFIED
     ↓
SCALE WITH AVAILABLE RESOURCES
     ↓
REMAIN COMPATIBLE AS TARGETS EVOLVE

The ultimate POCO-REAF rule is:

«Zamani source describes computation, not today's machine. The compiler preserves that computation's meaning, and target systems realize it according to the capabilities, resources, constraints, contracts, policies, and execution conditions actually available.»

This is what permits Zamani to scale from the smallest supported computational substrate to arbitrarily large computational systems without making an arbitrary machine capacity part of the language itself.
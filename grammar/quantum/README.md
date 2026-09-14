Zamani Quantum Grammar

Path: "grammar/quantum/README.md"

Status: Production architecture and integration contract

Language: Zamani

Grammar technology: ANTLR

Rust baseline for repository integration: Rust 1.97 / Rust 1.97.1, Rust 2021

Safety requirement: Safe Rust only. No "unsafe" Rust.

Architecture principle: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF).

---

1. Purpose

"grammar/quantum/" defines the source-language syntax for quantum computation in Zamani.

It provides a modular, extensible and backend-independent quantum grammar capable of expressing quantum computation without imposing accidental limitations derived from a particular quantum processor, simulator, compiler, vendor, topology, calibration profile or deployment environment.

The quantum grammar is one part of the complete Zamani language.

It must integrate with:

Zamani source
    |
    v
canonical lexer
    |
    v
canonical parser
    |
    v
frontend AST
    |
    v
semantic analysis
    |
    v
canonical semantic representation
    |
    +--> quantum::ir
    |
    +--> classical IR
    |
    v
optimization
    |
    v
routing / mapping
    |
    v
scheduling
    |
    +--> QEC
    +--> ZQN
    +--> resilience
    |
    v
hardware abstraction
    |
    v
target lowering
    |
    v
runtime

The grammar does not replace any of these downstream representations or subsystems.

---

2. Core architectural rule

The quantum grammar describes:

- quantum computation;
- quantum intent;
- quantum program structure;
- quantum/classical interaction;
- logical quantum resources;
- explicitly requested physical-resource intent;
- resource requirements;
- capability requirements;
- constraints;
- portable quantum operations;
- dynamic-circuit semantics;
- measurement;
- observables;
- state-related syntax;
- error-correction intent;
- extensibility points.

It does not describe:

- a particular QPU;
- a particular simulator;
- a particular vendor;
- a fixed qubit count;
- a fixed topology;
- a fixed coupling map;
- calibration values;
- pulse schedules;
- hardware addresses;
- device identifiers;
- runtime job identifiers;
- routing algorithms;
- scheduling algorithms;
- QEC decoder implementations;
- noise models;
- optimization algorithms;
- execution infrastructure.

The distinction is fundamental:

«The grammar describes what the programmer means. Downstream compilation determines how that meaning is realized.»

---

3. POCO-REAF contract

Zamani quantum source must support:

Program Once
        |
        v
Compile Once
        |
        v
Semantic representation
        |
        +--> simulator
        +--> QPU
        +--> logical quantum computer
        +--> heterogeneous system
        +--> future quantum architecture

A source program must not need to be rewritten solely because the available machine changes.

For example, source syntax must not require:

device IBM_X
qubits 127
topology heavy_hex

unless the programmer intentionally expresses those things as semantic target requirements.

Instead, portable source should be able to express concepts such as:

requires quantum;
requires capability measurement;
requires capability dynamic_circuit;
requires resource quantum_storage(amount);

The interpretation of those requirements belongs to semantic analysis, compilation and resource negotiation.

---

4. Scalability contract

The grammar imposes no artificial finite hardware ceiling.

It must not encode constants such as:

MAX_QUBITS
MAX_QUBIT_REGISTERS
MAX_CONTROLS
MAX_CIRCUIT_DEPTH
MAX_PARAMETERS
MAX_MEASUREMENTS
MAX_GATES
MAX_CLASSICAL_BITS
MAX_DEVICES
MAX_NODES

It must not contain hidden restrictions equivalent to:

q[0]
q[1]
q[63]

as the only supported resources.

Arbitrary source-level resource counts must be represented through the language's resource and type systems.

The grammar therefore scales conceptually from:

one qubit

to:

arbitrarily large quantum computation

subject only to actual resources available to the compiler, runtime and target.

Important distinction

"Unlimited by grammar" does not mean "physically infinite."

Actual execution can be constrained by:

- available memory;
- compiler capacity;
- parser capacity;
- target capacity;
- QPU capacity;
- simulator capacity;
- execution policies;
- provider limits;
- physical laws;
- runtime resources;
- user-defined constraints.

Those are not grammar-level machine constants.

---

5. Ownership

"grammar/quantum/" owns syntax composition.

It does not own quantum semantics.

This directory owns

- quantum syntax;
- quantum grammar composition;
- quantum declaration syntax;
- quantum block syntax;
- quantum statement composition;
- quantum-specific syntax boundaries;
- quantum/classical syntax boundaries;
- quantum dialect syntax;
- grammar extension points;
- grammar-level resource/capability syntax integration.

This directory does not own

- canonical AST implementation;
- semantic type checking;
- quantum IR;
- quantum optimization;
- routing;
- scheduling;
- hardware discovery;
- calibration;
- QEC algorithms;
- ZQN fault models;
- resilience policy;
- runtime dispatch;
- backend implementation.

---

6. Canonical semantic boundary

The canonical quantum semantic boundary remains:

quantum::ir

The grammar must never become a competing quantum intermediate representation.

The intended architecture is:

Zamani quantum syntax
        |
        v
ANTLR parse tree
        |
        v
frontend AST
        |
        v
semantic validation
        |
        v
quantum::ir

"quantum::ir" is responsible for representing canonical quantum semantics after parsing and semantic analysis.

The grammar merely establishes the source-language structure necessary to construct that representation.

---

7. Quantum grammar files

The quantum directory is modular.

The following ownership model applies.

File| Responsibility
"README.md"| Architecture and integration contract
"quantum.g4"| Quantum grammar orchestration
"qubits.g4"| Qubit declarations and references
"quantum-registers.g4"| Quantum register syntax
"quantum-types.g4"| Quantum type syntax
"quantum-states.g4"| Quantum state expressions
"gates.g4"| Gate declaration/name-level syntax
"operations.g4"| Quantum operation invocation
"controlled-operations.g4"| Controlled operation syntax
"parameterized-operations.g4"| Parameterized operation syntax
"circuits.g4"| Circuit declarations and composition
"measurement.g4"| Measurement syntax
"observables.g4"| Observable syntax
"reset.g4"| Reset syntax
"dynamic-circuits.g4"| Dynamic circuit syntax
"mid-circuit-control.g4"| Measurement-dependent control
"quantum-classical.g4"| Quantum/classical boundaries
"logical-qubits.g4"| Logical-qubit source syntax
"physical-qubits.g4"| Explicit physical-resource intent
"error-correction.g4"| QEC source declarations/intention
"quantum-resources.g4"| Quantum resource requirements
"quantum-capabilities.g4"| Quantum capability requirements
"quantum-dialects.g4"| Quantum dialect extensions

Every production must have exactly one owner.

---

8. "quantum.g4"

"quantum.g4" is the composition boundary.

It must:

- expose quantum source entry points;
- compose specialized grammar fragments;
- provide quantum blocks;
- provide quantum element composition;
- connect quantum declarations and statements;
- connect quantum/classical constructs;
- connect dynamic-circuit constructs;
- connect resource and capability constructs;
- provide explicit extension points.

It must not duplicate productions owned by specialized files.

It must not become a second implementation of:

- gates;
- qubits;
- registers;
- measurements;
- states;
- QEC;
- resources;
- capabilities.

---

9. "qubits.g4"

Owns source syntax for:

- qubit declarations;
- qubit references;
- qubit collections;
- symbolic qubit references;
- qubit bindings;
- qubit aliases where supported.

It must not determine:

- how many physical qubits exist;
- physical topology;
- physical placement;
- routing;
- allocation algorithms.

A source construct such as a qubit collection must remain independent of the eventual target size.

---

10. "quantum-registers.g4"

Owns:

- quantum register declarations;
- register names;
- register expressions;
- register indexing;
- register slicing where supported;
- symbolic register dimensions.

Register dimensions must not be restricted to an arbitrary machine maximum.

The semantic layer determines whether a requested register is realizable.

---

11. "quantum-types.g4"

Owns syntax for quantum types.

Potential semantic categories include:

- qubit;
- qubit reference;
- qubit collection;
- quantum register;
- logical qubit;
- physical resource reference;
- quantum state;
- measurement result;
- observable.

The grammar must not encode physical machine properties into these types.

---

12. "quantum-states.g4"

Owns syntax for source-level state expressions.

It may represent concepts such as:

- named states;
- basis states;
- state constructors;
- state preparation expressions;
- state references;
- symbolic state descriptions.

It must not implement a simulator.

It must not contain state-vector allocation.

It must not encode finite simulator dimensions.

---

13. "gates.g4"

Owns gate-level source syntax.

Gate names must not be treated as a permanently closed list unless the language specification explicitly defines a standard intrinsic operation.

The grammar must permit extensibility for:

- standard operations;
- user-defined operations;
- parameterized operations;
- controlled operations;
- future operations;
- dialect-provided operations.

Hardware-native gate availability belongs to target capability analysis.

---

14. "operations.g4"

Owns invocation syntax.

It represents the structural relationship between:

operation
parameters
targets
controls
modifiers

It must not decide:

- whether an operation is supported by hardware;
- whether it is native;
- how it is synthesized;
- how it is routed;
- how long it takes.

Those decisions belong downstream.

---

15. "controlled-operations.g4"

Owns syntax for controlled operations.

The grammar must support arbitrary syntactically valid control structures without imposing a fixed maximum.

For example, it must not encode:

maximum 2 controls

or equivalent hidden restrictions.

Whether a target supports a controlled operation is a capability/lowering question.

---

16. "parameterized-operations.g4"

Owns parameterized quantum operations.

Parameters must be expressions rather than fixed constants wherever the language semantics allow.

Examples include:

- symbolic parameters;
- compile-time parameters;
- runtime parameters;
- classical values;
- expressions;
- generic parameters.

Parameter count must not be restricted by arbitrary grammar constants.

---

17. "circuits.g4"

Owns circuit declarations and circuit-level structure.

Circuit syntax must remain independent of:

- target topology;
- hardware size;
- gate timing;
- device calibration;
- physical placement.

A circuit represents computation.

Compilation determines how the circuit is realized.

---

18. "measurement.g4"

Owns measurement syntax.

It must support the language's measurement model without assuming:

- fixed measurement count;
- fixed classical register size;
- fixed QPU behavior.

Measurement semantics are handled after parsing.

Measurement results must integrate with the canonical classical type/value model.

---

19. "observables.g4"

Owns observable syntax.

It must support composable observable descriptions without embedding simulator algorithms or numerical estimation strategies.

The grammar expresses:

what is observed

rather than:

how a backend computes the observation

---

20. "reset.g4"

Owns reset syntax.

Reset semantics must remain independent of:

- pulse implementation;
- hardware reset mechanism;
- calibration;
- timing;
- backend-specific instructions.

---

21. "dynamic-circuits.g4"

Owns dynamic-circuit constructs.

Dynamic circuits must support source-level structures where later computation depends on information produced during execution.

The grammar must not assume that every target supports dynamic circuits.

Instead:

syntax
    -> semantic requirement
    -> capability analysis
    -> lowering/adaptation

A target lacking the required capability must produce a semantic/compilation diagnostic rather than forcing the grammar to reject otherwise valid Zamani syntax.

---

22. "mid-circuit-control.g4"

Owns syntax for operations controlled by information produced during an executing quantum program.

This includes integration with:

- measurement;
- classical conditions;
- dynamic control;
- runtime-visible results.

The grammar must not implement runtime branching.

---

23. "quantum-classical.g4"

Owns explicit quantum/classical boundary syntax.

The boundary must remain explicit because quantum and classical values have different semantic rules.

This file integrates with:

grammar/classical/
grammar/types/
grammar/expressions/
grammar/statements/

It must not duplicate the classical language.

---

24. "logical-qubits.g4"

Owns syntax expressing logical-qubit intent.

It must not implement QEC.

Logical qubit syntax may identify:

- logical resources;
- logical operations;
- logical relationships;
- logical resource requirements.

Actual encoding, decoding, stabilizer construction and fault-tolerant implementation belong to QEC and downstream compilation.

---

25. "physical-qubits.g4"

Physical qubit syntax is an explicit escape from completely abstract hardware independence.

It exists only when the programmer intentionally expresses physical-resource intent.

It must never silently convert all quantum source into hardware-specific programming.

Physical references must remain distinct from logical qubits.

Hardware mapping belongs to:

routing
hardware HAL
target lowering

not the grammar.

---

26. "error-correction.g4"

This file expresses source-level QEC intent.

It must not contain:

- decoder implementations;
- syndrome algorithms;
- recovery matrices;
- stabilizer simulation;
- QEC scheduling algorithms;
- code-distance assumptions;
- hardware-specific correction procedures.

The grammar may express that a computation requires or uses an error-correction abstraction.

The QEC subsystem determines how that requirement is implemented.

---

27. "quantum-resources.g4"

Owns quantum resource expressions.

Resource concepts must remain declarative.

Examples include:

requires quantum;
requires quantum resource;
requires logical qubits;
requires measurement capability;
requires coherence capability;

A resource requirement must not automatically select a particular machine.

Resource resolution occurs downstream.

---

28. "quantum-capabilities.g4"

Owns capability requirements.

Examples include semantic capability categories such as:

measurement
dynamic_circuit
mid_circuit_measurement
conditional_control
parameterized_operation
error_correction

Capabilities describe what must be possible.

They do not identify which implementation provides the capability.

---

29. "quantum-dialects.g4"

Provides the controlled extension mechanism.

Dialects allow Zamani to evolve without requiring the base grammar to permanently enumerate every future quantum technology.

Dialect identity must be:

- namespaced;
- version-aware;
- capability-aware;
- compatibility-aware.

Vendor-specific syntax must not silently become universal Zamani syntax.

---

30. Lexer integration

The quantum grammar consumes the canonical lexer.

Quantum grammar files must not create independent lexer vocabularies.

The lexer must provide the tokens needed for:

- identifiers;
- literals;
- operators;
- punctuation;
- annotations;
- keywords;
- quantum syntax.

Quantum operation names should remain extensible.

Avoid turning every possible future gate into a lexer keyword.

For example, the grammar should not require a permanent lexer inventory containing every possible gate name.

---

31. Parser integration

The quantum grammar is composed into the canonical Zamani parser.

There must be one parser architecture.

Do not create:

QuantumParser
ClassicalParser
HardwareParser

as disconnected language parsers that each define their own conflicting language.

Instead:

canonical Zamani parser
        |
        +--> classical productions
        +--> quantum productions
        +--> HDL productions
        +--> hardware productions
        +--> distributed productions
        +--> future dialects

---

32. AST integration

The grammar produces parse structures consumed by the frontend AST layer.

The grammar must not invent a second semantic AST.

The AST should preserve:

- source locations;
- identifiers;
- expressions;
- declarations;
- operation structure;
- quantum/classical relationships;
- resource requirements;
- capabilities;
- annotations;
- dialect information.

Semantic analysis then validates those structures.

---

33. "quantum::ir" integration

The intended flow is:

quantum syntax
    |
    v
parse tree
    |
    v
AST
    |
    v
semantic analysis
    |
    v
quantum::ir

The grammar must not import or depend on Rust quantum IR types.

There must be no:

grammar -> quantum::ir

compile-time dependency.

Instead:

grammar
   |
   v
frontend
   |
   v
semantic lowering
   |
   v
quantum::ir

This prevents circular architecture.

---

34. Optimization integration

Optimization consumes canonical semantic representations.

The grammar must not perform:

- gate cancellation;
- peephole optimization;
- synthesis;
- T-gate reduction;
- circuit rewriting.

This preserves the repository boundary:

grammar
    -> semantics
    -> IR
    -> optimization

rather than:

grammar
    -> optimization

---

35. Routing integration

The grammar must not encode routing.

Logical-to-physical mapping belongs to routing.

Source-level physical-resource requirements are allowed only when explicitly meaningful.

The absence of a physical requirement must mean that routing is free to select an appropriate target mapping.

---

36. Scheduling integration

The grammar must not encode machine-specific schedules.

It may express semantic timing requirements where timing itself is part of program meaning.

However:

gate duration
pulse duration
hardware clock
coupling timing

must not become implicit grammar constants.

Scheduling determines actual execution order and timing.

---

37. QEC integration

The grammar expresses QEC intent.

QEC owns:

- code implementation;
- syndrome processing;
- correction;
- decoder selection;
- QEC execution;
- fault-tolerant transformations.

The grammar must not duplicate these responsibilities.

---

38. ZQN integration

ZQN owns quantum noise and fault semantics.

The grammar may express source-level requirements relevant to fault tolerance or noise-aware execution where the language specification requires it.

It must not contain:

- noise probabilities;
- calibration data;
- provider noise models;
- fault simulation;
- stochastic execution algorithms.

Those belong to ZQN and related semantic/runtime layers.

---

39. Resilience integration

Resilience is downstream.

The grammar may express semantic requirements relevant to resilience, but must not decide:

- retry;
- rollback;
- reroute;
- reschedule;
- backend switching;
- mitigation;
- recovery policy.

Those decisions belong to the resilience subsystem.

---

40. Hardware HAL integration

The grammar must remain independent of hardware discovery.

Hardware HAL determines:

what hardware exists
what it supports
what resources are available
what constraints apply

The grammar describes the program's requirements.

The compiler/runtime resolves:

program requirements
        +
target capabilities
        +
resource availability

---

41. Resource model

Quantum source must distinguish:

Requirement

Something the program needs.

Constraint

Something the implementation must satisfy.

Capability

Something a target can provide.

Preference

Something preferred but not mandatory.

Hint

Information that may guide compilation without changing semantics.

Resource

A quantity or category of computational capacity.

Target

An execution environment.

These concepts must never be collapsed into one grammar concept.

---

42. No vendor lock-in

The base quantum grammar must not contain permanent dependencies on:

- IBM;
- Google;
- Microsoft;
- Amazon;
- Rigetti;
- IonQ;
- Quantinuum;
- D-Wave;
- any other current or future provider.

Provider-specific syntax belongs in dialects/interoperability layers.

The base language must remain vendor-neutral.

---

43. No machine-size assumptions

Never encode:

127 qubits
133 qubits
1000 qubits
1 million qubits

as language limits.

A number occurring in source may be a legitimate program value or requirement.

That is different from the grammar imposing a maximum.

For example:

requires logical_qubits(count);

may contain an arbitrary source expression.

The grammar must not impose an implementation maximum.

---

44. No hidden resource allocation

The grammar must not silently imply:

allocate all available qubits

or:

use q[0] and q[1]

or:

use the first available QPU

Resource allocation belongs to semantic compilation and runtime policy.

---

45. Error handling

Invalid quantum syntax must generate deterministic parser diagnostics.

Diagnostics should preserve:

- source location;
- offending token;
- expected construct;
- context;
- grammar rule;
- recoverability information where supported.

The grammar must not silently convert malformed quantum syntax into comments or ignored operations.

---

46. Diagnostics boundary

The grammar answers:

«Is this syntactically valid?»

Semantic analysis answers:

«Is this construct meaningful?»

Capability analysis answers:

«Can the requested computation be supported?»

Compilation answers:

«Can it be lowered to the selected target?»

Runtime answers:

«Can it execute successfully?»

These diagnostic categories must remain distinguishable.

---

47. Determinism

Given identical source text and identical grammar version, parsing must be deterministic.

The grammar must not depend on:

- current machine;
- device discovery;
- runtime state;
- random numbers;
- calibration;
- network state;
- backend selection.

---

48. Versioning

Quantum syntax must be versioned through the central Zamani language versioning mechanism.

Do not introduce an independent quantum language version that conflicts with the language version.

Dialect versions may exist independently but must have explicit compatibility rules.

---

49. Backward compatibility

Existing valid Zamani quantum syntax must not be silently removed.

For every incompatible change:

1. identify the previous syntax;
2. determine its consumers;
3. classify the change;
4. provide migration information;
5. update compatibility tests;
6. document deprecation where appropriate.

---

50. Extensibility

The grammar must support future quantum technologies without repeatedly redesigning the base architecture.

Future possibilities may include:

- fault-tolerant quantum computing;
- logical architectures;
- photonic systems;
- neutral atoms;
- trapped ions;
- superconducting systems;
- spin systems;
- topological systems;
- quantum annealing;
- quantum simulation;
- quantum networking;
- distributed quantum computing;
- quantum memories;
- future architectures not currently known.

The base grammar should express common semantics while dialects provide specialized syntax.

---

51. Quantum networking

Quantum networking must not be confused with ordinary quantum computation.

If quantum networking syntax is introduced, it should integrate through the networking/distributed language layers.

The quantum grammar should provide only the quantum-side semantic boundary required for interoperability.

Network topology belongs to networking/distributed resource models.

---

52. Quantum + classical integration

Hybrid computation is a first-class requirement.

The language must permit:

classical computation
        |
        v
quantum computation
        |
        v
measurement
        |
        v
classical computation
        |
        v
quantum control

without creating separate incompatible languages.

Quantum/classical boundaries must therefore integrate with:

types
expressions
statements
functions
memory
concurrency
resources
execution

---

53. Generic programming

Quantum constructs should participate in Zamani's generic/type system where semantically valid.

Do not introduce a separate quantum generic system.

Generic quantum programs must be able to defer resource-specific decisions until compilation.

---

54. Compile-time versus runtime

The grammar must distinguish constructs that are:

- compile-time;
- runtime;
- semantic;
- target-dependent.

A compile-time quantum parameter must not automatically become a hardware constant.

Likewise, a runtime measurement must remain a runtime semantic event.

---

55. Security

Quantum grammar must not provide unrestricted escape hatches that bypass Zamani's security model.

Security-sensitive operations must integrate with the central:

security
effects
capabilities
permissions

systems.

The grammar must not silently grant hardware or network authority.

---

56. Unsafe Rust prohibition

Grammar infrastructure implemented in Rust must use safe Rust.

Forbidden:

unsafe

and unsafe abstractions whose sole purpose is performance.

The grammar architecture must remain compatible with:

Rust 1.97
Rust 1.97.1
Rust 2021

as established by the repository.

ANTLR-generated artifacts and Rust integration code must be validated against the repository's actual supported toolchain.

---

57. Testing requirements

Every quantum grammar file requires tests.

Tests must include:

Positive syntax

- qubit declarations;
- registers;
- states;
- operations;
- parameterized operations;
- controlled operations;
- circuits;
- measurement;
- reset;
- observables;
- dynamic circuits;
- mid-circuit control;
- logical qubits;
- physical-resource intent;
- QEC intent;
- capabilities;
- resources;
- dialects.

Negative syntax

Test:

- malformed declarations;
- missing targets;
- malformed parameters;
- invalid delimiters;
- invalid control structures;
- malformed measurements;
- invalid dialect syntax;
- invalid resource expressions.

---

58. Scalability tests

Tests must demonstrate that grammar syntax itself does not impose artificial machine limits.

The test suite must cover progressively larger symbolic constructs without defining a fake language maximum.

Examples should test:

small quantum program
large quantum program
large symbolic register
large operation sequence
large parameter set
large circuit structure

The tests must distinguish:

grammar limitation

from:

test runner resource limitation

---

59. Cross-domain tests

Required integration cases include:

classical + quantum
classical + quantum + concurrency
classical + quantum + distributed
quantum + HDL
quantum + hardware
quantum + QEC
quantum + ZQN-related semantics
quantum + resilience requirements
quantum + networking
AI + quantum
AI + hardware + quantum
classical + quantum + HDL + hardware

---

60. Round-trip tests

Where a canonical printer/serializer exists:

source
  |
  v
lexer
  |
  v
parser
  |
  v
AST
  |
  v
printer
  |
  v
parser

must preserve semantic meaning.

Formatting differences are acceptable where the language permits them.

Semantic changes are not.

---

61. Hard-coding audit

Every change to this directory must be checked for:

- fixed qubit limits;
- fixed register limits;
- fixed control counts;
- fixed operation counts;
- fixed circuit depth;
- fixed parameter counts;
- fixed backend names;
- fixed QPU IDs;
- fixed topology;
- fixed device sizes;
- fixed simulator dimensions;
- fixed hardware addresses;
- fixed calibration assumptions.

Each finding must be classified as:

1. semantic requirement;
2. target requirement;
3. resource constraint;
4. implementation limitation;
5. accidental hard-coding;
6. test limitation;
7. documentation limitation.

Accidental hard-coding must be removed.

---

62. Integration with repository subsystems

The dependency direction is:

grammar/quantum/
        |
        v
frontend
        |
        v
semantic analysis
        |
        v
quantum::ir
        |
        +--> optimization
        |
        +--> routing
        |
        +--> scheduling
        |
        +--> QEC
        |
        +--> ZQN
        |
        +--> resilience
        |
        +--> hardware HAL
        |
        v
runtime

There must be no reverse dependency:

quantum::ir -> grammar/quantum
runtime -> grammar/quantum
hardware -> grammar/quantum
scheduler -> grammar/quantum

unless a separate tooling component explicitly consumes grammar metadata without making the grammar depend on that subsystem.

---

63. Repository ownership matrix

Concern| Owner
Syntax| "grammar/"
Parse structure| frontend/parser
AST| frontend
Type semantics| type system
Quantum semantic IR| "quantum::ir"
Quantum optimization| optimization
Mapping| routing
Timing/order| scheduling
Error correction| QEC
Noise/fault semantics| ZQN
Adaptation/recovery| resilience
Hardware capabilities| hardware HAL
Calibration| hardware/calibration
Resource management| resource subsystem
Execution| runtime
Target lowering| compiler/backend
Syntax compatibility| grammar compatibility layer
Diagnostics| parser/semantic diagnostics

---

64. Anti-duplication rules

The quantum grammar must not create duplicate versions of:

- "QubitId";
- "PhysicalQubitId";
- quantum operation IR;
- quantum gate IR;
- hardware topology;
- resource manager;
- QEC limits;
- noise models;
- scheduler models;
- runtime execution models.

The grammar only creates source-level syntax that can later be mapped into these repository-owned representations.

---

65. Generated files

Generated ANTLR parser/lexer artifacts must not be treated as handwritten source-of-truth grammar.

The normative source is the grammar specification.

Generated files must be:

- reproducible;
- version-compatible;
- excluded from manual modification where repository policy permits;
- regenerated deterministically.

---

66. Documentation synchronization

The following documentation must remain consistent with this directory:

grammar/README.md
grammar/DESIGN.md
grammar/Zamani-Grammar.md
grammar/grammar.md

The authoritative syntax source must ultimately be clearly identified.

Documentation must not claim a quantum construct exists when the parser cannot accept it.

Likewise, grammar must not introduce undocumented public syntax without updating the appropriate specification.

---

67. Completion criteria

"grammar/quantum/" is production-ready only when:

- every grammar file has a single defined owner;
- no production is accidentally duplicated;
- quantum syntax composes through the canonical parser;
- lexer ownership is centralized;
- quantum syntax is deterministic;
- no artificial machine-size limits exist;
- no vendor is hard-coded into the base grammar;
- no hardware topology is hard-coded;
- quantum/classical integration is defined;
- resource/capability semantics are separated;
- logical and physical quantum concepts are separated;
- QEC is separated from QEC implementation;
- ZQN is separated from grammar;
- resilience is separated from grammar;
- scheduling is separated from grammar;
- routing is separated from grammar;
- optimization is separated from grammar;
- "quantum::ir" remains the canonical semantic boundary;
- AST integration is defined;
- diagnostics are deterministic;
- compatibility rules are documented;
- dialect extension is defined;
- scalability tests exist;
- negative tests exist;
- boundary tests exist;
- cross-domain tests exist;
- round-trip tests exist where supported;
- hard-coding audit passes;
- Rust integration uses Rust 1.97/1.97.1;
- no "unsafe" Rust is introduced;
- generated parser artifacts are reproducible;
- repository-wide integration tests pass.

---

68. Definition of "done" for this directory

A quantum grammar component is DONE only when:

syntax defined
+
ownership defined
+
dependencies defined
+
AST contract defined
+
semantic contract defined
+
IR boundary defined
+
compiler boundary defined
+
runtime boundary defined
+
resource boundary defined
+
hardware boundary defined
+
QEC boundary defined
+
ZQN boundary defined
+
resilience boundary defined
+
compatibility defined
+
scalability verified
+
hard-coding audited
+
positive tests pass
+
negative tests pass
+
boundary tests pass
+
cross-domain tests pass
+
documentation synchronized

No later grammar file is allowed to retroactively redefine the fundamental contract of a completed component.

---

69. Implementation order

The quantum directory should be implemented only after its upstream grammar contracts exist.

Recommended dependency order:

1. grammar/specification/
       |
2. grammar/lexer/
       |
3. grammar/core/
       |
4. grammar/types/
       |
5. grammar/expressions/
       |
6. grammar/statements/
       |
7. grammar/declarations/
       |
8. grammar/functions/
       |
9. grammar/modules/
       |
10. grammar/effects/
       |
11. quantum/quantum-types.g4
       |
12. quantum/qubits.g4
       |
13. quantum/quantum-registers.g4
       |
14. quantum/quantum-states.g4
       |
15. quantum/gates.g4
       |
16. quantum/operations.g4
       |
17. quantum/parameterized-operations.g4
       |
18. quantum/controlled-operations.g4
       |
19. quantum/measurement.g4
       |
20. quantum/reset.g4
       |
21. quantum/observables.g4
       |
22. quantum/circuits.g4
       |
23. quantum/logical-qubits.g4
       |
24. quantum/physical-qubits.g4
       |
25. quantum/quantum-resources.g4
       |
26. quantum/quantum-capabilities.g4
       |
27. quantum/dynamic-circuits.g4
       |
28. quantum/mid-circuit-control.g4
       |
29. quantum/quantum-classical.g4
       |
30. quantum/error-correction.g4
       |
31. quantum/quantum-dialects.g4
       |
32. quantum/quantum.g4
       |
33. quantum integration tests

"README.md" is the architectural contract and should be established before implementing the grammar fragments so that subsequent files have stable ownership and integration boundaries.

---

70. Final principle

The quantum grammar exists to make the following possible:

One Zamani program
        |
        v
One semantic meaning
        |
        +-------------------+
        |                   |
        v                   v
classical system       quantum system
        |                   |
        +---------+---------+
                  |
                  v
             hybrid system
                  |
                  v
          heterogeneous system
                  |
                  v
        distributed execution
                  |
                  v
         future architectures

The source program must describe computation and intent, not the accidental properties of whichever machine happens to execute it.

Therefore:

«Zamani quantum syntax must remain independent of machine scale, topology, vendor, calibration, resource count and current hardware technology while remaining expressive enough to state the semantic requirements that genuinely matter to the computation.»

This is the quantum-grammar foundation for:

Zamani — From Atom to Everywhere

and:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF).
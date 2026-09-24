Zamani Quantum Grammar

Path: "grammar/quantum/README.md"
Status: Normative quantum-subsystem architecture and integration contract
Language: Zamani
Grammar technology: ANTLR
Rust baseline: Rust 1.97 / Rust 1.97.1, Rust 2021
Safety: Safe Rust only; "unsafe" Rust is prohibited
Architecture: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)

---

1. Purpose

"grammar/quantum/" is the quantum source-language grammar subsystem of Zamani.

It defines the syntax required to express quantum computation as part of the single Zamani language while preserving:

- target independence;
- quantum/classical composition;
- arbitrary resource scale;
- open-ended quantum operations;
- logical and physical resource intent;
- dynamic circuits;
- measurement and feed-forward;
- observables;
- channels and noise intent;
- error-correction intent;
- pulse-level intent where explicitly required;
- quantum kernels and circuits;
- capability and resource requirements;
- dialect extensibility;
- interoperability with external quantum representations.

This directory does not define a second language.

It does not define a second quantum compiler.

It does not define a second quantum AST.

It does not define a second quantum IR.

The complete Zamani architecture remains:

Zamani source
    │
    ▼
canonical lexer
    │
    ▼
canonical Zamani parser
    │
    ▼
domain-neutral frontend AST
    │
    ▼
semantic analysis
    │
    ├── types
    ├── effects
    ├── capabilities
    ├── resources
    ├── ownership
    ├── portability
    └── quantum semantics
    │
    ▼
canonical semantic representation
    │
    ▼
quantum::ir
    │
    ├── optimization
    ├── decomposition
    ├── routing
    ├── scheduling
    ├── QEC
    ├── resilience
    └── ZQN
    │
    ▼
hardware abstraction / HAL
    │
    ▼
target lowering
    │
    ▼
runtime / simulator / QPU / accelerator / future target

The quantum grammar participates only in the source-language parsing boundary.

---

2. Normative authority

The quantum grammar is governed by the repository-wide authority hierarchy.

2.1 Architecture

The normative architectural authority is:

grammar/DESIGN.md

It defines:

- grammar authority;
- composition;
- frontend boundaries;
- AST ownership;
- semantic boundaries;
- IR ownership;
- portability;
- resource/capability separation;
- compatibility;
- validation;
- safety;
- POCO-REAF.

2.2 Quantum specification

The normative quantum specification is:

grammar/spec/quantum.md

It defines the language-level semantic contract for quantum constructs.

2.3 General syntax

General syntax remains governed by:

grammar/spec/syntax.md
grammar/lexer/
grammar/core/
grammar/expressions/
grammar/types/
grammar/declarations/
grammar/statements/

Quantum grammar files must reuse these contracts rather than redefining them.

2.4 Composition root

The repository-wide source grammar remains:

grammar/Zamani.g4

"Zamani.g4" owns complete-language composition.

"grammar/quantum/quantum.g4" owns only quantum-domain composition.

2.5 Parser implementation

The generated/canonical parser infrastructure must remain aligned with the repository's canonical ANTLR parser structure.

Quantum grammar files must not become an independent parser architecture.

2.6 AST

The frontend AST is owned outside this directory:

src/frontend/ast/

Quantum grammar files describe syntax that is lowered into the domain-neutral AST.

2.7 Canonical quantum IR

The canonical quantum semantic/IR boundary is:

quantum::ir

There must be exactly one canonical quantum IR.

This directory must never introduce another competing:

QuantumIR
QuantumOperationIR
QuantumGateIR
QuantumCircuitIR
QuantumHardwareIR

or equivalent semantic IR.

---

3. Core architectural principle

The fundamental rule is:

«Quantum grammar expresses portable computational meaning and intent; downstream compilation determines realization.»

The grammar describes:

- what operation is requested;
- what quantum resources are referenced;
- what classical information participates;
- what measurements occur;
- what capabilities are required;
- what resource requirements exist;
- what constraints/preferences/hints are expressed;
- what logical quantum structure is intended.

The grammar does not decide:

- which QPU is selected;
- which simulator is selected;
- which physical qubits are used unless explicitly requested;
- how logical qubits map to physical qubits;
- how operations are decomposed;
- how circuits are optimized;
- how routing occurs;
- how schedules are generated;
- how pulses are synthesized;
- how QEC is implemented;
- how noise is simulated;
- how a provider is contacted;
- how runtime resources are allocated.

---

4. POCO-REAF

Zamani quantum syntax is designed around:

Program
   Once
     │
     ▼
Compile
   Once
     │
     ▼
Semantic meaning
     │
     ├── tiny quantum system
     ├── CPU-assisted quantum system
     ├── GPU-assisted simulator
     ├── FPGA accelerator
     ├── ASIC accelerator
     ├── QPU
     ├── heterogeneous system
     ├── distributed quantum/classical system
     └── future architecture

The same source program must not require semantic rewriting merely because the target has a different:

- qubit count;
- topology;
- memory capacity;
- accelerator;
- instruction set;
- vendor;
- calibration;
- physical implementation;
- execution environment.

This is the quantum component of:

Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever

---

5. Unbounded-by-grammar scalability

The grammar imposes no artificial universal hardware ceilings.

The quantum grammar must not define language limits such as:

MAX_QUBITS
MAX_QUBIT_REGISTERS
MAX_LOGICAL_QUBITS
MAX_PHYSICAL_QUBITS
MAX_CONTROLS
MAX_TARGETS
MAX_PARAMETERS
MAX_OPERATIONS
MAX_GATES
MAX_CIRCUIT_DEPTH
MAX_CIRCUIT_WIDTH
MAX_MEASUREMENTS
MAX_SHOTS
MAX_CHANNELS
MAX_TIMELINES
MAX_QPUS
MAX_DEVICES
MAX_MEMORY

or equivalent hidden limits.

The following must never become grammar-level limits:

64 qubits
128 qubits
256 qubits
1024 qubits
32 controls
32 parameters
64 targets

A number appearing in source code is allowed when it is program data or semantic intent.

For example:

let n = 1024;
allocate qubits[n];

is fundamentally different from:

the grammar supports at most 1024 qubits

The former is source semantics.

The latter is an artificial language limitation and is prohibited.

---

6. Physical reality versus grammar scalability

"Unbounded quantum grammar" does not mean physically infinite execution.

Actual execution remains constrained by available:

- memory;
- compiler resources;
- simulator resources;
- QPU resources;
- accelerator resources;
- runtime resources;
- provider resources;
- physical hardware;
- topology;
- timing;
- error rates;
- energy;
- thermal conditions;
- user-defined constraints.

Those limitations are discovered and evaluated downstream.

Therefore:

grammar capacity

must never be confused with:

target capacity

or:

runtime capacity

---

7. Requirement / constraint / preference / hint / capability separation

Quantum grammar must preserve the distinction between:

Concept| Meaning
Requirement| Must be satisfied
Constraint| Condition that must be respected
Preference| Desired property that may be traded off
Hint| Advisory information
Capability| Property supplied by an environment
Resource| Abstract quantity or facility
Target| Realization destination
Placement| Explicit realization intent
Mapping| Physical/logical realization information

For example:

requires qubits >= n

is a requirement.

requires capability("quantum.mid_circuit_measurement")

is a capability requirement.

prefer capability("quantum.dynamic_control")

is a preference.

hint resource("quantum_memory")

is advisory.

None of those automatically means:

use QPU 0
use physical qubit 17
use topology X

---

8. Open-world quantum operation model

This is a non-negotiable architectural rule.

Quantum operations are open-ended.

The grammar must not define a permanently closed list such as:

H
X
Y
Z
S
T
CNOT
CZ
SWAP
RX
RY
RZ

as the complete language of quantum operations.

Those names may be recognized by semantic libraries, standard-operation registries, dialects, or target backends.

They must not define the grammar's fundamental operation model.

The grammar must support the same structural representation for:

apply H(q);

apply X(q);

apply CNOT(q0, q1);

apply RX(theta)(q);

apply custom_gate(q);

apply library::operation(q);

apply vendor::operation(parameter)(q0, q1);

apply future::operation(arguments)(targets);

The operation identity is semantic data.

---

9. Generic operation model

The canonical conceptual operation structure is:

QuantumOperation {
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
}

The exact Rust AST/IR representation is owned downstream.

The grammar must provide enough source structure to populate this semantic model.

It must not turn the model into a fixed gate enum.

---

10. Canonical operation structure

The preferred conceptual syntax is:

quantumOperation
    = operationSpecifier
      operationParameters?
      operationTargets

The grammar must support:

operationSpecifier
operation parameters
operation targets
operation modifiers
operation controls
operation attributes

without requiring every future quantum operation to be added to the grammar.

---

11. Operation namespaces

Qualified operation names must be supported.

Examples:

library::operation
vendor::operation
domain::operation
future::operation

Namespace depth should be determined by the common qualified-name grammar.

Quantum grammar must not invent a separate namespace system.

It must reuse:

grammar/core/
grammar/modules/
grammar/expressions/

where applicable.

---

12. Gate syntax versus gate semantics

The existing "gates.g4" must not become a fixed gate catalogue.

Its role is limited to source-level gate declaration/name-level syntax.

The following are semantic concerns, not grammar concerns:

- unitary validation;
- matrix construction;
- decomposition;
- native-gate determination;
- synthesis;
- equivalence;
- optimization;
- hardware support.

A standard operation such as "H" may be known to the semantic operation registry without "H" becoming the only possible operation.

---

13. Existing quantum directory and ownership

The current repository contains the following quantum grammar surfaces.

They must not all become independent authorities.

The production ownership model is:

Existing file| Canonical responsibility
"quantum.g4"| Quantum composition root
"operations.g4"| Generic quantum operation invocation
"gates.g4"| Gate declaration/name-level syntax
"parameterized-operations.g4"| Parameterized-operation syntax
"controlled-operations.g4"| Controlled-operation syntax
"controls.g4"| General control/modifier syntax
"adjoints.g4"| Adjoint/inverse operation syntax
"qubits.g4"| Qubit source syntax
"registers.g4"| Quantum register syntax
"logical-qubits.g4"| Logical-qubit intent
"physical-qubits.g4"| Explicit physical-resource intent
"types.g4"| Canonical quantum type syntax
"quantum-types.g4"| Legacy/overlapping quantum-type surface; must not remain a second owner
"states.g4"| State syntax
"quantum-states.g4"| Legacy/overlapping state surface; must not remain a second owner
"measurement.g4"| Measurement syntax
"reset.g4"| Reset syntax
"barriers.g4"| Barrier syntax
"observables.g4"| Observable syntax
"channels.g4"| Channel syntax
"noise.g4"| Source-level noise intent
"error-correction.g4"| QEC intent
"logical-operations.g4"| Logical operation syntax
"dynamic-circuits.g4"| Dynamic-circuit structure
"dynamic-control.g4"| Dynamic control semantics at syntax level
"mid-circuit-control.g4"| Measurement-dependent control syntax
"quantum-classical.g4"| Quantum/classical boundary
"classical-feedforward.g4"| Classical feed-forward syntax
"quantum-capabilities.g4"| Quantum capability requirements
"quantum-resources.g4"| Quantum resource contracts
"resource-requirements.g4"| Resource-requirement compatibility/overlap surface; must not duplicate ownership
"circuits.g4"| Circuit structure
"kernels.g4"| Quantum kernel structure
"parameters.g4"| Quantum parameter syntax
"pulse-intent.g4"| Abstract pulse-level intent
"quantum-dialects.g4"| Quantum dialect extension
"interoperability.md"| Quantum interoperability contract
"README.md"| This architecture/integration contract

The existence of a file does not mean it automatically owns a production rule.

---

14. Duplicate-file resolution policy

The current directory contains overlapping file names and responsibilities.

This must be resolved through single ownership, not parallel composition.

The following pairs are especially important:

types.g4
quantum-types.g4

states.g4
quantum-states.g4

controlled-operations.g4
controls.g4

dynamic-circuits.g4
dynamic-control.g4
mid-circuit-control.g4

quantum-classical.g4
classical-feedforward.g4

quantum-resources.g4
resource-requirements.g4

The production rule is:

«One semantic concept → one canonical grammar owner.»

Existing files must not be unnecessarily renamed.

Instead, each overlapping file must be classified as exactly one of:

1. canonical implementation;
2. composition wrapper;
3. compatibility grammar;
4. historical/reference grammar;
5. migration surface;
6. obsolete and removable after dependency verification.

Two files must never silently contribute the same production to the canonical parser.

---

15. "quantum.g4"

"quantum.g4" is the single quantum-domain composition grammar.

It owns:

- quantum-domain entry;
- quantum declaration dispatch;
- quantum statement dispatch;
- quantum expression dispatch;
- quantum type dispatch;
- quantum-domain composition;
- cross-quantum-subdomain composition;
- extension dispatch.

It does not own detailed leaf syntax.

It must not duplicate:

- operation rules;
- measurement rules;
- state rules;
- qubit rules;
- register rules;
- QEC rules;
- resource rules;
- capability rules;
- classical rules.

The root grammar must compose canonical leaf owners.

---

16. Composition-root invariant

The composition hierarchy must remain:

grammar/Zamani.g4
        │
        ▼
    Quantum
        │
        ├── QuantumOperations
        ├── QuantumMeasurement
        ├── QuantumReset
        ├── QuantumTypes
        ├── QuantumStates
        ├── QuantumCapabilities
        ├── QuantumDialects
        ├── QuantumErrorCorrection
        ├── QuantumDynamicControl
        ├── QuantumClassical
        ├── QuantumClassicalFeedForward
        ├── QuantumObservables
        ├── QuantumResourceRequirements
        ├── LogicalOperations
        ├── QuantumCircuits
        ├── QuantumRegisters
        ├── QuantumQubits
        ├── QuantumParameters
        ├── QuantumControls
        └── QuantumAdjoints

The exact generated ANTLR import list must contain each canonical grammar exactly once.

In particular, the current duplicate "QuantumClassical" import must be removed.

---

17. Canonical lexer authority

Quantum grammars must use the repository's canonical lexical vocabulary.

The quantum subsystem must not introduce its own independent lexer.

The canonical production lexer must be whatever lexer is designated by the repository's current architecture and "grammar/DESIGN.md".

Where the ANTLR parser grammars use:

tokenVocab = ZamaniLexer;

that vocabulary must be used consistently throughout the production composition.

Historical references to:

ZamaniTokens

must not create a second lexical authority.

No compatibility alias should conceal an unresolved production-vocabulary conflict.

---

18. Quantum keywords

Quantum operations must not require every operation name to become a lexer keyword.

For example:

H
X
CNOT
RX
custom_gate
vendor::operation
future::operation

should be representable as operation designators without continually expanding the lexer keyword table.

Keywords should exist only when they have stable language-level grammatical meaning.

---

19. Identifiers and qualified names

Quantum grammar must reuse the canonical identifier and qualified-name syntax.

Do not redefine:

identifier
qualifiedName
namespace
path

inside each quantum file.

Use the common grammar infrastructure.

This ensures:

library::operation
vendor::operation
custom::future::operation

behave consistently throughout Zamani.

---

20. Qubit ownership

"qubits.g4" owns source syntax for:

- qubit declarations;
- qubit references;
- qubit collections;
- symbolic qubit references;
- aliases;
- indexed access;
- slices/ranges where supported.

It does not own:

- physical allocation;
- topology;
- routing;
- mapping;
- calibration;
- QPU discovery.

---

21. Register ownership

"registers.g4" owns:

- quantum-register declarations;
- register references;
- register indexing;
- register slicing;
- register extents;
- symbolic register dimensions.

Register cardinality is not a grammar limit.

For example:

Qubit[n]

is a semantic/program-level type or extent.

It must not mean:

n <= compiler_defined_constant

---

22. Quantum type ownership

"types.g4" is the canonical quantum type syntax owner.

Quantum types may represent concepts such as:

Qubit
Qubit[n]
LogicalQubit
PhysicalQubit
QuantumRegister
QuantumState
MeasurementResult
Observable
QuantumChannel

The exact semantic type model belongs to semantic analysis.

Types must remain independent of physical hardware capacity.

---

23. State syntax

The state grammar may represent:

- basis states;
- named states;
- state constructors;
- state references;
- state preparation;
- symbolic state expressions.

Examples may include:

|0⟩
|1⟩
|+⟩
|-⟩
|ψ⟩

where supported by the canonical lexical and syntax contracts.

The grammar does not allocate state vectors.

It does not simulate amplitudes.

It does not impose a finite simulator dimension.

---

24. Quantum literals

Quantum literals are lexical/source constructs.

The lexer and grammar must support the repository's specified forms without making the set artificially closed.

The semantic layer determines:

- state validity;
- type compatibility;
- dimensionality;
- normalization;
- preparation semantics.

---

25. Measurement

"measurement.g4" owns source measurement syntax.

Measurement syntax must support the semantic model for:

- measurement targets;
- result bindings;
- measurement bases where supported;
- measurement destinations;
- repeated measurement semantics where specified;
- mid-circuit measurement;
- final measurement.

It must not impose a maximum number of measurements.

It must not encode provider-specific measurement instructions.

---

26. Measurement results

Measurement results must integrate with the common Zamani type/value system.

The grammar must not create a separate classical language for measurement values.

The semantic boundary is:

quantum measurement
       │
       ▼
measurement result
       │
       ▼
canonical classical semantic model

This is essential for dynamic circuits and classical feed-forward.

---

27. Dynamic circuits

"dynamic-circuits.g4" owns circuit structures whose later behavior can depend on information produced during execution.

The grammar must permit semantic constructs such as:

measure
if result { ... }
apply ...

or the canonical Zamani equivalent.

Whether a target supports dynamic execution is not a parsing question.

It is a capability question.

Therefore:

valid source
    ↓
semantic capability analysis
    ↓
target supports capability?
    ├── yes → lower
    └── no  → diagnostic/adaptation

---

28. Mid-circuit control

"mid-circuit-control.g4" owns syntax for control derived from runtime quantum/classical information.

It integrates with:

measurement.g4
classical-feedforward.g4
dynamic-control.g4
dynamic-circuits.g4
expressions/
statements/

It must not implement runtime branching.

---

29. Classical feed-forward

"classical-feedforward.g4" owns source constructs in which classical information produced by quantum execution influences later computation.

It must reuse the canonical classical expression and statement grammars.

It must not create a second conditional-expression system.

---

30. Quantum/classical boundary

"quantum-classical.g4" defines the syntax boundary between quantum and classical computation.

The boundary must remain explicit enough for semantic analysis to determine:

- value domains;
- ownership;
- measurement flow;
- mutability;
- effects;
- timing;
- synchronization;
- runtime availability.

Quantum and classical are parts of one language, not two disconnected languages.

---

31. Parameterized operations

"parameterized-operations.g4" owns syntax for operation parameters.

Parameters may be:

- literals;
- identifiers;
- expressions;
- symbolic values;
- compile-time values;
- runtime values;
- generic parameters.

The grammar must not impose a fixed parameter count.

For example:

apply rotation(theta)(q);

is structurally preferable to requiring every possible rotation operation to have its own grammar rule.

---

32. Controls

"controls.g4" owns reusable control/modifier structure.

It must support arbitrary syntactically valid control composition without a fixed maximum.

For example, the language must not establish:

maximum two controls

as a grammar limitation.

Whether a target supports an operation with a given control structure is determined downstream.

---

33. Controlled operations

"controlled-operations.g4" owns controlled-operation syntax.

It must compose with:

operations.g4
controls.g4
parameters.g4
expressions/

It must not create a separate operation representation.

The semantic operation remains one generic operation with modifiers/controls.

---

34. Adjoints and inverses

"adjoints.g4" owns syntax expressing adjoint/inverse intent.

It must not perform algebraic transformation.

For example:

adjoint(operation)
inverse(operation)

is syntax/intent.

Actual transformation belongs downstream.

---

35. Circuit syntax

"circuits.g4" owns circuit declarations and composition.

A circuit is a source-level computational structure.

It must not encode:

- physical topology;
- gate duration;
- physical device;
- calibration;
- routing;
- scheduling;
- provider job identifiers.

---

36. Quantum kernels

"kernels.g4" owns source-level quantum kernel structure.

A kernel may represent a computation intended for specialized quantum execution.

It must remain composable with:

- functions;
- types;
- expressions;
- classical computation;
- resource requirements;
- capabilities;
- effects.

A kernel must not hard-code a particular QPU.

---

37. Logical qubits

"logical-qubits.g4" owns logical-qubit source intent.

Logical qubits are semantic abstractions.

The grammar does not implement:

- encoding;
- decoding;
- syndrome extraction;
- stabilizer simulation;
- logical-to-physical mapping.

Those belong to QEC and downstream compilation.

---

38. Physical qubits

"physical-qubits.g4" exists for cases where the programmer intentionally expresses physical-resource intent.

Physical references must remain clearly distinct from logical resources.

For example:

logical qubit

must not silently mean:

physical qubit 0

Physical mapping belongs downstream unless explicitly required by source semantics.

---

39. Error correction

"error-correction.g4" owns source-level QEC intent.

It may express concepts such as:

- error-correction requirements;
- fault-tolerance intent;
- logical-resource requirements;
- code-level semantic selection where specified;
- correction-related attributes.

It must not implement:

- decoders;
- syndrome algorithms;
- recovery algorithms;
- QEC scheduling;
- physical error correction;
- provider-specific QEC execution.

QEC remains a downstream subsystem.

---

40. QEC resource separation

A source requirement such as:

requires logical qubits >= n

must remain distinct from:

physical qubits required by selected QEC realization

The second quantity may be derived downstream.

The grammar must not encode an assumed physical overhead.

This is critical for scalability.

---

41. Noise

"noise.g4" owns source-level noise intent where Zamani exposes noise semantics.

It must not become a provider calibration database.

It must not hard-code:

- device-specific error rates;
- calibration values;
- fixed noise channels for particular hardware;
- simulator implementation;
- random execution.

Noise semantics belong to ZQN and related semantic/runtime systems.

---

42. Channels

"channels.g4" owns syntax for quantum-channel concepts.

The grammar expresses channel structure/intent.

It does not implement:

- Kraus-operator execution;
- density-matrix simulation;
- stochastic sampling;
- numerical linear algebra.

Those belong downstream.

---

43. Observables

"observables.g4" owns observable syntax.

The grammar expresses:

what is observed

not:

how a backend estimates it

Backend-specific strategies remain downstream.

---

44. Barriers

"barriers.g4" owns source-level barrier intent.

A barrier is not automatically a physical pulse or hardware instruction.

The semantic/compiler layers determine whether and how the barrier affects:

- optimization;
- scheduling;
- routing;
- execution.

---

45. Reset

"reset.g4" owns reset syntax.

Reset semantics remain independent of:

- hardware reset implementation;
- pulse sequences;
- calibration;
- timing;
- device-specific instructions.

---

46. Pulse intent

"pulse-intent.g4" provides a controlled source-level representation for applications that intentionally require pulse-level semantic information.

It must remain intent, not a hardware calibration language.

It must not embed:

- device calibration tables;
- physical channel IDs as universal syntax;
- provider-specific pulse APIs;
- hard-coded clock frequencies;
- physical device addresses.

The downstream target layer determines realization.

---

47. Quantum resources

"quantum-resources.g4" owns source-level quantum resource contracts.

Examples include abstract requirements involving:

qubits
logical_qubits
quantum_memory
measurement_capacity
coherence
communication
reliability
latency
energy

Resource quantities must be expressions.

No universal resource ceiling is permitted.

---

48. Resource requirements

"resource-requirements.g4" must not become a second resource grammar.

Its final status must be established explicitly during consolidation:

- canonical owner;
- compatibility wrapper;
- migration surface;
- historical/reference;
- removable.

The canonical semantic resource contract must remain one system.

---

49. Capability requirements

"quantum-capabilities.g4" owns quantum-specific capability references and requirements.

Examples:

requires capability("quantum.measurement");

requires capability("quantum.mid_circuit_measurement");

requires capability("quantum.dynamic_control");

requires capability("quantum.logical_qubits");

Capability names remain open-ended semantic identifiers.

The grammar must not enumerate every future hardware capability.

---

50. Capability versus device

A capability requirement:

requires capability("quantum.dynamic_control");

does not mean:

use vendor/device X

The compiler may satisfy the capability using any compatible target.

---

51. Quantum dialects

"quantum-dialects.g4" provides the controlled extension mechanism.

A dialect may introduce:

- syntax extensions;
- semantic extensions;
- operation namespaces;
- attributes;
- target-specific constructs.

Every dialect must identify:

- name;
- namespace;
- version;
- ownership;
- compatibility;
- syntax extensions;
- semantic mapping;
- AST mapping;
- IR mapping.

Vendor syntax must never silently become universal core Zamani syntax.

---

52. Dialect isolation

A dialect must not:

- replace the core operation model;
- introduce another quantum IR;
- create another type system;
- create another resource system;
- create another capability model;
- bypass semantic validation.

Dialect syntax must eventually converge into the canonical Zamani semantic model and "quantum::ir".

---

53. Interoperability

"interoperability.md" documents integration with external quantum representations.

Examples may include:

- OpenQASM;
- QIR;
- external circuit representations;
- other quantum interchange formats.

External formats are interoperability boundaries, not competing Zamani semantic authorities.

The canonical flow remains:

external representation
       │
       ▼
adapter/import
       │
       ▼
Zamani semantic model
       │
       ▼
quantum::ir

and in the other direction:

quantum::ir
       │
       ▼
export/lowering adapter
       │
       ▼
external representation

---

54. OpenQASM integration

OpenQASM syntax must be handled through the repository's interoperability/frontend architecture rather than copied wholesale into the Zamani core grammar.

The OpenQASM frontend must eventually integrate with:

src/quantum/frontend/

and its OpenQASM format support.

Imported OpenQASM constructs must map into Zamani's semantic model.

They must not create a second canonical quantum IR.

---

55. Canonical AST boundary

The grammar must lower conceptually as:

ANTLR parse tree
       │
       ▼
domain-neutral AST
       │
       ▼
semantic quantum model
       │
       ▼
quantum::ir

The AST must preserve enough information for:

- source locations;
- identifiers;
- operation names;
- namespaces;
- parameters;
- operands;
- targets;
- controls;
- modifiers;
- results;
- attributes;
- capabilities;
- resources;
- requirements;
- constraints;
- preferences;
- dialect information.

---

56. Source spans

Every parser-visible quantum construct must preserve source-location information through the frontend.

At minimum, diagnostics must be able to identify:

- file/source unit;
- start position;
- end position;
- offending construct;
- relevant semantic context where available.

The grammar must not discard source structure needed for high-quality diagnostics.

---

57. Semantic analysis

Semantic analysis owns:

- operation resolution;
- type checking;
- qubit/resource validity;
- scope;
- ownership;
- capability checking;
- resource checking;
- effect checking;
- dimensionality;
- parameter compatibility;
- control validity;
- measurement validity;
- logical/physical distinction;
- dialect semantics;
- interoperability validation.

The parser must not perform these checks.

---

58. Canonical "quantum::ir"

All semantically valid quantum computation must cross one canonical boundary:

quantum::ir

The grammar must never directly construct or depend upon Rust IR types.

The architecture must be:

grammar
  │
  ▼
AST
  │
  ▼
semantic analysis
  │
  ▼
quantum::ir

Never:

grammar
  │
  ▼
quantum::ir

and never:

grammar
  │
  ▼
QuantumGateIR
  │
  ▼
quantum::ir

unless a repository-wide architecture explicitly establishes that intermediate as a different, formally owned semantic layer. The quantum grammar itself must not introduce one.

---

59. "quantum::ir" semantic operation requirements

A generic quantum operation entering "quantum::ir" must be capable of preserving, as required by the canonical IR contract:

operation identity
namespace
operands
targets
parameters
controls
modifiers
results
attributes
effects
capabilities
resource requirements
source information

The grammar provides source information.

Semantic lowering determines canonical meaning.

---

60. Optimization boundary

Optimization starts after semantic lowering.

The quantum grammar does not perform:

- gate cancellation;
- commutation optimization;
- synthesis;
- decomposition;
- algebraic simplification;
- depth reduction;
- measurement optimization;
- hardware-specific optimization.

Those belong to optimization/decomposition passes.

---

61. Routing boundary

Routing starts after the canonical semantic representation.

Routing determines physical realization where necessary.

The grammar must not encode a coupling map as the universal execution model.

Source may express an explicit physical requirement where semantically justified, but ordinary portable programs must remain independent of physical topology.

---

62. Scheduling boundary

Scheduling determines actual:

- order;
- timing;
- resource occupation;
- synchronization;
- execution schedule.

The grammar may express semantic timing requirements where timing is part of program meaning.

It must not hard-code a particular target's clock or gate duration.

---

63. Resilience boundary

The repository's resilience subsystem owns decisions such as:

ACCEPT
DEGRADED_ACCEPT
RETRY
RECOVER
ESCALATE
REJECT

and the relevant resilience states:

Unknown
Healthy
Degraded
Unstable
Unavailable
Recovering
Quarantined
Retired

The quantum grammar may express relevant requirements or metadata, but it must not implement resilience policy.

---

64. ZQN boundary

ZQN owns fault/noise semantics and their downstream processing.

Quantum grammar may express source-level noise/fault requirements.

It must not implement:

- stochastic simulation;
- fault models;
- device calibration;
- mitigation algorithms;
- decoder algorithms;
- runtime noise adaptation.

---

65. Hardware/HAL boundary

Hardware availability is resolved after parsing.

The grammar must not perform hardware discovery.

The downstream system resolves:

program requirements
       +
capabilities
       +
resources
       +
constraints
       +
target availability

into an actual target realization.

---

66. Physical mapping

A portable quantum program should normally describe:

logical qubits
operations
measurements
requirements
capabilities

rather than:

physical qubit 17
physical qubit 18
physical link 17-18

Physical mapping becomes relevant only when the programmer intentionally requires physical-level control.

---

67. Topology

Topology does not belong in the core quantum computation grammar.

Topology belongs to the hardware/routing layer.

Quantum grammar may refer to topology-related requirements when the language specification explicitly supports them, but it must not implement topology.

---

68. Vendor independence

Vendor-specific operations may be represented through qualified names or dialects:

vendor::operation
vendor::namespace::operation

The core grammar must not accumulate permanent vendor keyword lists.

Vendor support must be implemented through:

dialect
semantic registry
capability model
lowering
backend

rather than by continuously modifying the core operation grammar.

---

69. No hidden hardware assumptions

The quantum grammar must not assume:

64 qubits
127 qubits
133 qubits
256 qubits
32-bit registers
64-bit registers
24 GB memory
specific topology
specific QPU count
specific accelerator count

as universal language characteristics.

Any such number in a grammar/test must be classified as:

- example;
- test data;
- program data;
- explicit user requirement;
- target-specific declaration;
- compatibility fixture.

It must never become an implicit universal language limit.

---

70. Generic resource expressions

Resource quantities should be represented by expressions.

Conceptually:

requires qubits >= n;

or:

requires resource quantum.qubits >= required_qubits;

where supported by the canonical resource grammar.

The exact syntax must follow the canonical repository resource contract rather than being independently invented here.

---

71. Quantum/classical scaling

Quantum programs may contain classical computation of arbitrary semantic scale.

For example:

let n = problem_size;
allocate qubits[n];

The grammar must not require the compiler to know a fixed "n" during parsing.

Semantic analysis determines what can be established statically.

Runtime-dependent values remain runtime-dependent where the language permits dynamic allocation.

---

72. Dynamic allocation

Where dynamic quantum resource allocation is supported by Zamani semantics, the grammar must represent it without assuming a fixed resource pool.

The grammar does not decide whether a backend can dynamically allocate those resources.

That becomes a capability/resource question.

---

73. Quantum memory

Quantum-memory syntax must remain abstract.

The grammar must not assume:

specific number of qubits in memory
specific memory technology
specific coherence duration
specific memory address space

Memory semantics belong to the common memory/resource architecture and quantum semantic analysis.

---

74. Effects

Quantum operations may have semantic effects.

Examples can include:

- measurement;
- state mutation;
- reset;
- allocation;
- deallocation;
- synchronization;
- classical observation.

Effects are analyzed downstream.

The grammar must not duplicate the global effect system.

---

75. Ownership and lifetime

Quantum resource ownership must integrate with the repository's common ownership/memory model.

The grammar must not invent an isolated ownership system for qubits.

Semantic analysis determines:

- lifetime;
- aliasing;
- borrowing/ownership constraints where applicable;
- use-after-measurement rules;
- resource release.

---

76. Deterministic parsing

Parsing must be deterministic.

Parsing may depend only on:

- source text;
- selected language version;
- canonical lexer;
- canonical grammar;
- explicitly selected dialect/version.

Parsing must not depend on:

- QPU availability;
- hardware discovery;
- calibration;
- network state;
- runtime state;
- filesystem state;
- wall-clock time;
- randomness;
- environment variables.

---

77. Safe Rust requirement

All Rust code integrating this grammar must remain:

Rust 2021
Rust 1.97 / Rust 1.97.1
safe Rust

The project must not introduce "unsafe".

The crate should retain or enforce:

#![deny(unsafe_code)]

where applicable to the repository's Rust crate architecture.

ANTLR grammar files must contain no embedded Rust actions.

---

78. No grammar actions

Production quantum grammar must contain no actions that:

- access files;
- access networks;
- access hardware;
- invoke runtime code;
- mutate global state;
- allocate device resources;
- query capabilities;
- perform randomness;
- perform semantic execution.

The grammar is declarative.

---

79. No parser-time hardware discovery

This is a permanent invariant.

Never:

parse source
    ↓
query QPU
    ↓
change grammar behavior

Instead:

parse source
    ↓
semantic model
    ↓
capability/resource analysis
    ↓
target realization

---

80. No parser-time capability discovery

Capability discovery belongs downstream.

The grammar recognizes the syntax of a requirement.

It does not determine whether the current machine satisfies it.

---

81. No parser-time resource allocation

Parsing must never allocate:

- qubits;
- memory;
- devices;
- execution slots;
- QPU jobs;
- simulator state.

Allocation belongs downstream.

---

82. Integration with "grammar/expressions/"

Quantum expressions must reuse the canonical expression grammar.

Quantum-specific syntax may add an explicit wrapper where necessary, but it must not create a separate precedence system.

Operation parameters should therefore be ordinary Zamani expressions wherever semantically appropriate.

---

83. Integration with "grammar/types/"

Quantum types must compose with:

- generic types;
- function types;
- references;
- collections;
- constraints;
- capabilities;
- resource types.

Quantum types must not create a disconnected type language.

---

84. Integration with "grammar/statements/"

Quantum statements must be legal Zamani statements.

The quantum grammar supplies domain-specific statement forms where needed.

Common control flow remains owned by:

grammar/statements/

Dynamic quantum control must integrate rather than duplicate general "if", "match", loop, and block syntax.

---

85. Integration with "grammar/functions/"

Quantum functions/kernels must integrate with:

- parameters;
- generic parameters;
- return types;
- effects;
- contracts;
- calling conventions.

A quantum function is still a Zamani function.

---

86. Integration with "grammar/modules/"

Quantum operations and types may be imported/exported through the normal module system.

Quantum namespaces must not create a separate module system.

---

87. Integration with "grammar/effects/"

Quantum effects must use the common effect system.

A quantum operation may be semantically effectful without requiring a second effect grammar.

---

88. Integration with "grammar/resources/"

Quantum resource requirements must compose with the universal resource model.

The quantum subsystem may add quantum-specific resource vocabulary, but it must not redefine:

requirement
constraint
preference
hint
capability
resource

---

89. Integration with "grammar/hardware/"

The quantum grammar may express abstract hardware-related requirements.

The hardware grammar owns:

- target descriptions;
- hardware capabilities;
- resource models;
- topology;
- placement;
- device classes;
- accelerator descriptions.

The quantum grammar must not duplicate those definitions.

---

90. Integration with "grammar/hybrid/"

Hybrid quantum/classical programs must remain first-class Zamani programs.

The integration path is:

classical
   +
quantum
   +
shared types/expressions
   +
effects/resources/capabilities

No separate hybrid language should be created.

---

91. Integration with "grammar/hdl/"

Quantum/HDL co-design may express:

- quantum computation;
- control hardware;
- accelerators;
- interfaces;
- timing intent;
- hardware resources.

HDL implementation remains owned by "grammar/hdl/".

Quantum grammar must not duplicate HDL syntax.

---

92. Integration with "grammar/distributed/"

Quantum programs may execute across distributed resources.

The quantum grammar can express distributed-relevant requirements where the language specifies them.

Distributed topology and deployment remain owned by:

grammar/distributed/

---

93. Integration with AI and data domains

Quantum computation may participate in:

- AI;
- machine learning;
- tensors;
- data processing;
- optimization;
- scientific computing.

Quantum grammar must compose with these domains through shared types, expressions, functions, resources, effects, and capabilities.

It must not create framework-specific syntax for every external AI or scientific library.

---

94. Quantum kernels and accelerators

Quantum kernels may be compiled to:

- QPU;
- simulator;
- CPU;
- GPU;
- FPGA;
- ASIC;
- heterogeneous accelerator;
- distributed execution environment.

The kernel source must express computation rather than hard-code the eventual implementation.

---

95. Interoperability with classical computation

Measurement results, parameters, control values, and classical conditions must use the common Zamani semantic model.

The language should permit:

classical computation
      ↓
quantum operation
      ↓
measurement
      ↓
classical computation
      ↓
quantum operation

without requiring separate source languages.

---

96. Mathematical operations

The quantum grammar must not become a second mathematics grammar.

Quantum parameters should use the canonical expression system.

Mathematical functions should generally remain:

- expressions;
- intrinsics;
- library operations;
- semantic capabilities.

They should not become new parser keywords merely because a new quantum algorithm uses them.

---

97. Algorithm independence

The grammar should not require dedicated syntax for every quantum algorithm.

Algorithms such as:

- search;
- factoring;
- simulation;
- optimization;
- chemistry;
- machine learning;
- phase estimation;

should normally be represented using the general language, libraries, kernels, and generic quantum operations.

Dedicated syntax is justified only when the construct has stable language-level semantics.

---

98. Program meaning versus implementation

The quantum grammar should answer:

«What quantum computation is being expressed?»

It should not answer:

«How does today's QPU execute it?»

This distinction must be maintained throughout the subsystem.

---

99. Target-independent example model

A conceptual portable program might express:

requires capability("quantum.measurement");
requires capability("quantum.dynamic_control");

let n = problem_size;

quantum {
    allocate q[n];

    apply operation(q);

    let result = measure q;

    if result {
        apply next_operation(q);
    }
}

The exact surface syntax must follow the canonical Zamani grammar contracts.

The architectural property is what matters:

- no QPU identifier;
- no fixed qubit limit;
- no fixed topology;
- no physical addresses;
- no vendor instruction set.

---

100. Explicit target intent

Zamani may permit explicit target intent where the programmer genuinely requires it.

Such intent must remain distinguishable from portable computation.

For example:

requires capability("quantum.mid_circuit_measurement");

is portable capability intent.

An explicit physical mapping is a much stronger statement and must remain isolated from normal portable source.

The compiler must never silently infer that ordinary abstract quantum source is physical-source programming.

---

101. Resource negotiation

Resource requirements should flow through:

source requirement
       ↓
semantic resource contract
       ↓
compiler resource analysis
       ↓
target capability/resource matching
       ↓
placement
       ↓
routing
       ↓
scheduling
       ↓
execution

This permits the same program to operate at different scales.

---

102. Tiny-to-large scaling

The language must be able to represent programs conceptually ranging from:

one qubit

to:

large logical computation

to:

large distributed heterogeneous quantum/classical computation

without changing the grammar because the resource scale changed.

The implementation may of course encounter finite limits imposed by the actual host and toolchain.

Those are implementation limits, not language semantics.

---

103. Infinite terminology

"Infinity" in POCO-REAF means:

«no arbitrary finite ceiling encoded by the language architecture.»

It does not mean that a parser, compiler, simulator, QPU, or physical system can literally allocate infinite resources.

This distinction must remain explicit in all documentation and tests.

---

104. Hard-coding audit

Every quantum grammar modification must be audited for:

- fixed qubit counts;
- fixed register counts;
- fixed operation counts;
- fixed target counts;
- fixed control counts;
- fixed parameter counts;
- fixed circuit depths;
- fixed measurement counts;
- fixed QPU counts;
- fixed device counts;
- fixed memory sizes;
- fixed topology;
- fixed addresses;
- fixed vendor assumptions;
- fixed simulator dimensions.

Each numeric value must be classified as:

1. language semantics;
2. program data;
3. explicit user requirement;
4. target-specific intent;
5. compatibility fixture;
6. test data;
7. accidental hard-coding.

Accidental hard-coding must be removed.

---

105. Forbidden examples

The quantum grammar must never evolve into:

quantumRegister : QUBIT_0 | QUBIT_1 | ... ;

or:

MAX_QUBITS = 1024;

or:

gate : H | X | Y | Z | CNOT | ... ;

or:

only 2 controls allowed

or:

only 64 targets allowed

or:

QPU_0
QPU_1
QPU_2

as the universal hardware model.

---

106. Preferred generic representation

The preferred model is:

operation name
    +
parameters
    +
controls/modifiers
    +
targets
    +
attributes
    +
semantic requirements

This enables future quantum operations without changing the core grammar.

---

107. Source-level operation extensibility

A future operation such as:

future::topological_operation

must be representable without modifying the generic operation grammar merely because the operation did not exist when the compiler was written.

Semantic validation determines whether that operation is known, imported, provided by a dialect, or otherwise valid.

---

108. Unknown operation handling

The parser should distinguish:

syntactically valid operation designator

from:

semantically unresolved operation

This allows extensibility.

An unknown operation name should normally produce a semantic-resolution diagnostic rather than requiring the parser grammar to be rewritten for every new operation.

---

109. Diagnostics

Quantum diagnostics must distinguish:

Lexical errors

Examples:

- malformed quantum literal;
- invalid identifier;
- invalid delimiter.

Syntax errors

Examples:

- missing target;
- malformed parameter list;
- malformed control expression.

Semantic errors

Examples:

- unknown operation;
- incompatible parameter;
- invalid target type;
- invalid measurement;
- unavailable required capability.

Resource errors

Examples:

- insufficient target resources.

Target errors

Examples:

- target cannot realize required capability.

The parser must not incorrectly report downstream capability/resource failures as syntax errors.

---

110. Deterministic diagnostics

Given the same:

source
language version
dialect set

the grammar must produce deterministic parse behavior and deterministic syntax diagnostics.

Hardware availability must not change parser diagnostics.

---

111. Testing architecture

Quantum grammar tests must exist at several levels.

lexer
syntax
AST
semantic
IR
compiler
target
runtime

The grammar tests themselves must not attempt to prove functionality that belongs to downstream systems.

---

112. Required positive tests

At minimum, the quantum conformance suite must cover:

- minimal quantum program;
- single qubit;
- multiple qubits;
- symbolic qubit counts;
- registers;
- indexed registers;
- state literals;
- generic operation;
- standard operation name;
- custom operation;
- qualified operation;
- parameterized operation;
- multiple parameters;
- multiple targets;
- controlled operation;
- nested modifiers;
- adjoint operation;
- measurement;
- reset;
- barrier;
- observable;
- circuit;
- kernel;
- logical qubit;
- physical-resource intent;
- dynamic circuit;
- mid-circuit measurement;
- classical feed-forward;
- resource requirement;
- capability requirement;
- dialect extension;
- channel;
- noise intent;
- QEC intent;
- pulse intent;
- hybrid classical/quantum program.

---

113. Required negative tests

Negative tests must cover:

- malformed operation;
- missing target;
- malformed parameter expression;
- malformed control;
- malformed measurement;
- invalid quantum literal;
- invalid register access;
- malformed resource requirement;
- malformed capability requirement;
- malformed dialect declaration;
- invalid syntax boundary;
- duplicate syntax where prohibited;
- ambiguous constructs;
- invalid source-level combinations.

---

114. Boundary tests

Boundary tests must cover:

- one qubit;
- many syntactically representable qubits;
- empty collections where legal;
- one target;
- many targets;
- one parameter;
- many parameters;
- deeply nested modifiers;
- deeply nested qualified names;
- large operation sequences;
- large circuits;
- large resource declarations;
- large capability lists.

No boundary test may accidentally become a language ceiling.

---

115. Scalability tests

Scalability tests must prove absence of grammar-defined limits.

Conceptually test:

n = small
n = larger
n = much larger

using generated source fixtures.

The expected invariant is:

language validity does not change merely because n increased

provided the syntax remains representable and the test environment has sufficient resources.

---

116. No artificial test ceilings

A test such as:

supports exactly 1024 qubits

is prohibited unless 1024 is explicitly testing a user program requirement.

The test suite must not establish arbitrary language limits.

---

117. Determinism tests

Repeated parsing of identical quantum source must produce equivalent parser results.

The test environment must not alter parsing through:

- hardware;
- network;
- randomness;
- time;
- environment variables.

---

118. AST conformance tests

Every public quantum grammar construct must map to a predetermined AST representation.

For every construct, the project must know:

grammar rule
    ↓
AST node/field

before declaring the grammar feature complete.

---

119. Semantic conformance tests

Every quantum AST construct must have a documented semantic interpretation.

For every construct:

AST
 ↓
semantic validation
 ↓
semantic quantum model

must be defined.

---

120. IR conformance tests

Every semantically meaningful quantum construct must have a known downstream path to:

quantum::ir

If a construct intentionally does not lower into quantum IR because it is compile-time metadata, resource metadata, or another semantic category, that must be explicitly documented.

No unexplained syntax is production complete.

---

121. Compiler integration tests

Tests must verify that valid quantum source can flow through the compiler architecture without requiring target-specific source rewrites.

Where target realization differs, the semantic source meaning must remain stable.

---

122. Cross-target tests

The same semantic quantum program should be tested against different target classes where supported:

simulator
CPU-assisted simulator
GPU-assisted simulator
FPGA accelerator
ASIC accelerator
QPU
heterogeneous target
distributed target

The test is not that every target can execute every program.

The test is that target differences are handled by capability/resource/lowering analysis rather than by changing the source grammar.

---

123. Cross-domain tests

Mandatory quantum combinations include:

classical + quantum
quantum + hardware
quantum + HDL
quantum + AI
quantum + data
quantum + distributed
quantum + networking
quantum + security
classical + quantum + hardware
quantum + HDL + hardware
AI + quantum + hardware
classical + quantum + HDL + hardware

The goal is one composable language.

---

124. Round-trip tests

Where a canonical formatter/printer exists:

source
  ↓
lexer
  ↓
parser
  ↓
AST
  ↓
formatter/printer
  ↓
parser

must preserve semantic meaning.

Formatting may change.

Meaning must not.

---

125. Interoperability tests

Where external formats are supported:

external format
    ↓
import
    ↓
Zamani semantic representation
    ↓
quantum::ir

and, where supported:

quantum::ir
    ↓
export
    ↓
external format

must be tested.

---

126. Compatibility

Quantum grammar changes must respect:

grammar/compatibility/

and repository language versioning.

Changes must be classified as:

- additive;
- compatible;
- breaking;
- deprecated;
- migration-required.

Renaming existing quantum files is not permitted merely for aesthetic reasons.

---

127. Grammar-version compatibility

A quantum dialect or syntax feature must be associated with a language version or feature lifecycle where necessary.

The parser must not silently reinterpret old source under a new incompatible meaning.

---

128. Feature lifecycle

Quantum features should progress through:

historical
    ↓
proposed
    ↓
experimental
    ↓
specified
    ↓
implemented
    ↓
tested
    ↓
stable

"Zamani-Grammar.md" is not sufficient authority to make a feature stable.

A feature becomes stable only after:

semantic design
    ↓
AST contract
    ↓
grammar
    ↓
semantic implementation
    ↓
IR contract
    ↓
tests
    ↓
compatibility

---

129. Existing "Zamani-Grammar.md"

Quantum features documented in:

grammar/Zamani-Grammar.md

must be treated according to their feature status.

The broad design document may contain:

- proposed features;
- historical concepts;
- future concepts;
- experimental concepts.

It must not silently override this quantum grammar.

---

130. Existing "grammar/grammar.md"

"grammar/grammar.md" is the implementation-conformance reference.

Quantum implementation status must eventually be reflected there as:

SPECIFIED
IMPLEMENTED
PARTIALLY IMPLEMENTED
PLANNED
DEPRECATED

This README does not replace that implementation status document.

---

131. Quantum grammar file header contract

Every ".g4" file in this directory must document:

File
Grammar name
Status
Purpose
Owns
Does not own
Inputs
Outputs
Dependencies
Upstream contracts
Downstream consumers
AST contract
Semantic contract
IR contract
Compiler integration
Runtime integration
Tooling integration
Cross-domain integration
Positive tests
Negative tests
Boundary tests
Scalability tests
Compatibility
Determinism
Hard-coding audit
Safety
Completion criteria

This is mandatory.

---

132. Single-owner rule

Every grammar production must have exactly one canonical owner.

For example:

quantumOperation

must have one owner.

Other files may compose or reference it.

They must not redefine it.

The same rule applies to:

- qubit;
- register;
- measurement;
- state;
- control;
- parameter;
- capability;
- resource;
- dialect;
- circuit.

---

133. No competing quantum grammars

The following architecture is prohibited:

Quantum grammar A
      +
Quantum grammar B
      +
Quantum grammar C

where all three claim to be canonical.

The correct architecture is:

one canonical quantum grammar
       +
explicitly owned leaf grammars
       +
compatibility/reference material

---

134. Dependency direction

Quantum grammar dependencies must flow toward shared syntax infrastructure.

Preferred:

canonical lexer
      ↓
core
      ↓
names/types/expressions
      ↓
quantum leaf grammars
      ↓
Quantum composition
      ↓
Zamani composition

Never:

quantum grammar ↔ runtime

or:

quantum grammar ↔ quantum::ir

or circular leaf-grammar dependencies.

---

135. Grammar composition versus semantic composition

ANTLR grammar composition is not semantic composition.

The grammar may compose:

operations
measurement
types
states
resources
capabilities

but semantic analysis determines how those concepts interact.

This distinction prevents grammar files from becoming semantic implementation containers.

---

136. No embedded simulator

The grammar must never:

- allocate state vectors;
- allocate density matrices;
- execute gates;
- sample measurement;
- calculate amplitudes;
- simulate noise.

Those belong to simulator/runtime implementations.

---

137. No embedded QPU execution

The grammar must never:

- submit jobs;
- query devices;
- reserve qubits;
- invoke hardware;
- query calibration;
- execute pulses.

Those belong to downstream systems.

---

138. No embedded routing

The grammar must never calculate:

logical qubit → physical qubit

routing.

It may preserve explicit source-level mapping intent.

Routing owns actual mapping.

---

139. No embedded scheduling

The grammar must not generate physical execution schedules.

Timing intent may be expressed.

Scheduling determines realization.

---

140. No embedded QEC

The grammar expresses QEC intent.

It does not perform QEC.

---

141. No embedded ZQN

The grammar may describe noise/fault-related source intent.

ZQN owns fault/noise semantics and processing.

---

142. No embedded calibration

Calibration is target-specific and dynamic.

It must not be encoded as core grammar semantics.

---

143. No embedded hardware discovery

The grammar must remain valid even when no hardware is available.

This property is essential for:

- offline compilation;
- CI;
- deterministic builds;
- simulation;
- cross-compilation;
- reproducibility.

---

144. Reproducibility

Parsing and semantic representation must be reproducible given the same:

source
language version
dialect versions
compiler version
specified semantic environment

Hardware discovery must not alter parsing.

---

145. Build reproducibility

The quantum grammar must not require:

- a live QPU;
- network connectivity;
- vendor credentials;
- calibration services;
- runtime access

to generate the parser or validate syntax.

---

146. Security

Grammar processing must not execute arbitrary source-language operations.

Quantum syntax must be treated as untrusted input.

The parser must not:

- execute code;
- access files;
- access network resources;
- invoke external processes;
- access devices.

---

147. Resource exhaustion

Although no language-level resource ceiling is permitted, implementation-level resource exhaustion must still be handled safely.

The distinction is:

language semantic limit

versus:

implementation resource exhaustion

The latter must produce controlled diagnostics/failure behavior rather than memory corruption or unsafe behavior.

---

148. No "unsafe"

No quantum grammar feature may require "unsafe" Rust.

The complete Rust implementation must remain compatible with:

Rust 1.97
Rust 1.97.1
Rust 2021

and safe Rust only.

---

149. Tooling

The grammar must support downstream tooling such as:

- syntax highlighting;
- formatting;
- language-server parsing;
- completion;
- navigation;
- documentation generation;
- syntax-tree inspection;
- diagnostics;
- refactoring.

Public grammar rule names therefore constitute compatibility surfaces.

---

150. Documentation generation

Quantum documentation should be generated or derived from authoritative contracts where practical.

Documentation must not become a competing grammar authority.

---

151. Generated files

Generated parser artifacts must not become manually maintained grammar authorities.

The source ".g4" contracts remain authoritative.

Generated files should be reproducible from the canonical grammar/toolchain.

---

152. Production quality requirements

The quantum subsystem is production-ready only when:

- grammar ownership is unambiguous;
- all duplicate surfaces are classified;
- canonical lexer vocabulary is used;
- the composition root is unique;
- operation syntax is open-ended;
- quantum types are composable;
- registers are scalable;
- controls are scalable;
- parameters are scalable;
- circuits are scalable;
- measurement is composable;
- dynamic circuits are represented;
- classical feed-forward is represented;
- logical and physical resources are distinct;
- QEC intent is represented;
- noise intent is represented;
- resource/capability semantics are separated;
- no hardware limits are encoded;
- no vendor gate catalogue is hard-coded;
- no runtime behavior occurs in grammar;
- no "unsafe" Rust is required;
- AST mappings exist;
- semantic mappings exist;
- "quantum::ir" mappings exist;
- compiler integration exists;
- target integration exists;
- interoperability is defined;
- diagnostics are deterministic;
- compatibility is documented;
- positive tests exist;
- negative tests exist;
- boundary tests exist;
- scalability tests exist;
- cross-domain tests exist;
- hard-coding audits pass.

---

153. Definition of Done for an individual quantum grammar file

A quantum ".g4" file is not complete merely because ANTLR accepts it.

It is complete only when all applicable items below are satisfied:

Identity

- [ ] file purpose documented;
- [ ] grammar name documented;
- [ ] status documented.

Ownership

- [ ] every rule has one owner;
- [ ] duplicate ownership eliminated;
- [ ] non-ownership documented.

Dependencies

- [ ] lexer dependency identified;
- [ ] upstream grammar dependencies identified;
- [ ] no circular dependency.

Syntax

- [ ] syntax specified;
- [ ] ambiguity resolved;
- [ ] precedence resolved where applicable;
- [ ] cardinality is intentionally unbounded.

AST

- [ ] every construct has an AST mapping;
- [ ] source spans preserved.

Semantics

- [ ] semantic meaning specified;
- [ ] invalid semantic cases specified.

IR

- [ ] canonical downstream representation identified;
- [ ] no competing quantum IR introduced.

Compiler

- [ ] compiler consumer identified;
- [ ] lowering expectations documented.

Runtime

- [ ] runtime relevance documented;
- [ ] no runtime behavior embedded in grammar.

Hardware

- [ ] no accidental hardware assumptions;
- [ ] no artificial capacity limits.

Testing

- [ ] positive tests;
- [ ] negative tests;
- [ ] boundary tests;
- [ ] scalability tests;
- [ ] determinism tests;
- [ ] compatibility tests;
- [ ] cross-domain tests where applicable.

Safety

- [ ] no grammar actions;
- [ ] no filesystem access;
- [ ] no network access;
- [ ] no device access;
- [ ] no randomness;
- [ ] no "unsafe" Rust integration.

Only then is the file independently complete.

---

154. Definition of Done for "quantum/"

The entire quantum grammar subsystem is complete only when:

all leaf owners
      ↓
canonical Quantum composition
      ↓
canonical Zamani composition
      ↓
canonical lexer/parser
      ↓
domain-neutral AST
      ↓
semantic analysis
      ↓
quantum::ir
      ↓
optimization
      ↓
routing
      ↓
scheduling
      ↓
QEC / resilience / ZQN
      ↓
HAL
      ↓
target realization

has an explicit, tested contract.

---

155. Repository-wide quantum integration matrix

Layer| Quantum responsibility
"grammar/DESIGN.md"| Architecture
"grammar/Zamani.g4"| Complete-language composition
"grammar/lexer/"| Tokens/lexical rules
"grammar/core/"| Names/paths/common syntax
"grammar/expressions/"| Expressions/precedence
"grammar/types/"| General type system
"grammar/statements/"| General statements/control flow
"grammar/functions/"| Functions/kernels/function semantics
"grammar/effects/"| Effects
"grammar/resources/"| Universal resources
"grammar/hardware/"| Hardware intent/capabilities/targets
"grammar/hybrid/"| Hybrid domain composition
"grammar/distributed/"| Distributed computation
"grammar/hdl/"| Hardware description
"grammar/interoperability/"| External representations
"grammar/quantum/"| Quantum source syntax
"src/frontend/ast/"| Domain-neutral AST
semantic analysis| Quantum semantic validation
"quantum::ir"| Canonical quantum representation
optimization| Quantum optimization/decomposition
routing| Logical/physical realization
scheduling| Execution scheduling
QEC| Error correction
ZQN| Noise/fault semantics
HAL| Hardware abstraction
runtime| Execution
tests| Conformance

---

156. Quantum ownership matrix

Concept| Owner
Quantum composition| "quantum.g4"
Quantum operation invocation| "operations.g4"
Gate declaration syntax| "gates.g4"
Parameterized operations| "parameterized-operations.g4"
Controls| "controls.g4"
Controlled operations| "controlled-operations.g4"
Adjoint/inverse| "adjoints.g4"
Qubits| "qubits.g4"
Registers| "registers.g4"
Quantum types| "types.g4"
States| canonical state grammar after duplicate consolidation
Measurement| "measurement.g4"
Reset| "reset.g4"
Barrier| "barriers.g4"
Observable| "observables.g4"
Channels| "channels.g4"
Noise intent| "noise.g4"
Dynamic circuits| "dynamic-circuits.g4"
Dynamic control| "dynamic-control.g4"
Mid-circuit control| "mid-circuit-control.g4"
Quantum/classical boundary| "quantum-classical.g4"
Feed-forward| "classical-feedforward.g4"
Logical qubits| "logical-qubits.g4"
Physical-resource intent| "physical-qubits.g4"
Logical operations| "logical-operations.g4"
QEC intent| "error-correction.g4"
Quantum resources| "quantum-resources.g4"
Quantum capabilities| "quantum-capabilities.g4"
Circuits| "circuits.g4"
Kernels| "kernels.g4"
Pulse intent| "pulse-intent.g4"
Quantum parameters| "parameters.g4"
Dialects| "quantum-dialects.g4"
Interoperability contract| "interoperability.md"
Canonical quantum semantics| semantic layer
Canonical quantum IR| "quantum::ir"
Routing| downstream routing subsystem
Scheduling| downstream scheduling subsystem
QEC implementation| QEC subsystem
Noise/fault processing| ZQN
Physical realization| HAL/backend

---

157. Existing duplicate surfaces: required consolidation

The following are explicitly identified as consolidation work.

"types.g4" / "quantum-types.g4"

Exactly one becomes the canonical type owner.

The other becomes compatibility/reference material or is removed only after repository-wide dependency verification.

"states.g4" / "quantum-states.g4"

Exactly one becomes the canonical state owner.

"controlled-operations.g4" / "controls.g4"

These may legitimately remain separate because:

- "controls.g4" can own reusable control/modifier syntax;
- "controlled-operations.g4" can own the controlled-operation composition.

They must not duplicate the same productions.

"dynamic-circuits.g4" / "dynamic-control.g4" / "mid-circuit-control.g4"

These can remain separate if their ownership is explicitly:

dynamic-circuits
    = circuit-level dynamic structure

dynamic-control
    = generic dynamic control composition

mid-circuit-control
    = measurement-result-dependent control

No overlapping production ownership is allowed.

"quantum-classical.g4" / "classical-feedforward.g4"

These may remain separate as:

quantum-classical
    = domain boundary

classical-feedforward
    = classical result propagation/control

"quantum-resources.g4" / "resource-requirements.g4"

These must be consolidated into one resource semantic ownership model.

They must not create two competing quantum resource languages.

---

158. Required "quantum.g4" cleanup

Before the quantum parser is declared production-conformant, "quantum.g4" must be audited for:

- duplicate imports;
- duplicate rules;
- stale grammar names;
- stale lexer vocabulary;
- obsolete imports;
- overlapping grammar owners;
- unreachable imports;
- circular dependencies;
- undefined imported rules;
- conflicting token vocabularies.

In particular, the current duplicate "QuantumClassical" import must not remain.

---

159. Required lexer cleanup

The quantum subsystem must participate in the repository-wide lexer cleanup.

Known lexical duplication must be resolved centrally rather than inside quantum grammar.

Examples include duplicate conceptual tokens such as:

Question / QuestionMark
Ampersand / BitAnd

where the distinction is not semantically justified.

Quantum grammar must consume the canonical token model.

---

160. Numeric magnitude

Quantum source must not impose artificial numeric magnitude limits.

The language may use implementation-appropriate numeric representations, but the quantum grammar must not define machine-width assumptions such as:

32-bit quantum count
64-bit quantum count

unless explicitly required by a separately specified language type.

---

161. Tensor and quantum data integration

Quantum programs may interact with:

Tensor<T, shape>

and other data structures.

The quantum grammar must reuse the common tensor/type/data semantics rather than creating a second tensor language.

---

162. Resource-size semantics

A resource quantity such as:

qubits >= n

is a semantic requirement.

It must not be interpreted as:

physical qubit identifiers 0 through n-1

This distinction is mandatory for POCO-REAF.

---

163. Capability examples

Valid semantic capability categories may include:

quantum.measurement
quantum.mid_circuit_measurement
quantum.dynamic_control
quantum.logical_qubits
quantum.error_correction
quantum.fault_tolerance
quantum.observable_measurement
quantum.parameterized_operations

The list is extensible.

The grammar must not make it a finite universal catalogue.

---

164. Future quantum technologies

The grammar must remain capable of representing future quantum technologies without requiring a new core grammar architecture.

Future technology should first attempt to use:

- generic operations;
- qualified names;
- attributes;
- types;
- capabilities;
- resources;
- requirements;
- constraints;
- dialects.

A new core syntax construct should be added only when existing abstractions cannot express the new semantic concept adequately.

---

165. Quantum technology neutrality

The grammar must remain neutral regarding:

- superconducting systems;
- trapped-ion systems;
- neutral atoms;
- photonic systems;
- spin systems;
- topological systems;
- annealing/optimization systems where semantically supported;
- future quantum architectures.

Technology-specific implementation belongs downstream.

---

166. Simulation neutrality

The same source language should support quantum simulation where the semantic model permits it.

The grammar must not contain simulator-specific syntax merely because simulation is one possible execution target.

---

167. Distributed quantum computing

The grammar should remain capable of representing distributed quantum computation through:

- abstract resources;
- communication;
- channels;
- capabilities;
- synchronization;
- distributed constructs.

It must not impose a fixed number of QPUs or nodes.

---

168. Fault-tolerant scaling

Fault-tolerant source intent must remain independent of a particular physical-code overhead.

For example:

requires logical qubits >= n

does not become:

requires exactly f(n) physical qubits

inside the grammar.

The physical overhead is determined by QEC/target compilation.

---

169. Compilation scaling

Compilation may specialize the same semantic source for different resource environments.

The source grammar must not force a new source program for each machine size.

---

170. Runtime scaling

Runtime resource availability may change dynamically.

The grammar remains deterministic and target-independent.

Runtime decisions occur after parsing.

---

171. No source rewriting for ordinary scaling

Changing:

small target

to:

larger target

must not require rewriting the quantum algorithm solely to account for hardware capacity.

The compiler may produce different lowerings.

The source semantic intent remains the same.

---

172. Completion workflow

The production workflow for every quantum feature is:

feature proposal
      ↓
quantum semantic design
      ↓
AST contract
      ↓
grammar owner selected
      ↓
leaf grammar implemented
      ↓
Quantum composition integrated
      ↓
Zamani.g4 integrated
      ↓
lexer/parser conformance
      ↓
semantic implementation
      ↓
quantum::ir mapping
      ↓
compiler integration
      ↓
target integration
      ↓
tests
      ↓
compatibility
      ↓
stable

A feature must not skip these stages.

---

173. Independent-file completion principle

A file must contain enough advance integration information that completing it does not require waiting for another file's undocumented design.

For every file, the contract must already identify:

who consumes it
what it consumes
what AST it produces
what semantics it represents
what IR receives it
what compiler uses it
what runtime uses it
what tests prove it
what files it must not duplicate

This is the required independent-first development model.

---

174. Recommended implementation order

The quantum subsystem should be stabilized in this order:

1. canonical quantum ownership map
2. canonical lexer vocabulary
3. quantum type owner
4. quantum qubit owner
5. quantum register owner
6. parameter owner
7. generic operation owner
8. controls/modifiers
9. controlled operations
10. adjoints
11. measurement
12. reset
13. states
14. circuits
15. kernels
16. quantum/classical boundary
17. dynamic control
18. classical feed-forward
19. observables
20. channels
21. noise intent
22. logical qubits
23. logical operations
24. physical-resource intent
25. resource requirements
26. capability requirements
27. QEC intent
28. pulse intent
29. dialects
30. interoperability
31. quantum.g4 composition
32. Zamani.g4 integration
33. AST conformance
34. semantic conformance
35. quantum::ir conformance
36. compiler integration
37. target integration
38. complete conformance suite

This ordering minimizes rework.

---

175. What must not be changed unnecessarily

This README does not authorize unnecessary renaming.

Existing major repository files remain:

grammar/Zamani.g4
grammar/grammar.md
grammar/Zamani-Grammar.md
grammar/DESIGN.md

Existing quantum filenames should also be retained unless dependency analysis proves that an overlapping file is obsolete and removal is preferable to maintaining a duplicate authority.

---

176. What may be deleted

A quantum file may be removed only when repository-wide dependency analysis establishes that it is:

- unused;
- redundant;
- superseded;
- not required for compatibility;
- not required as historical/reference documentation.

Deletion must never happen merely because two filenames look similar.

---

177. Production invariants

The following are permanent invariants of "grammar/quantum/":

ONE quantum composition root
ONE canonical quantum semantic boundary
ONE canonical quantum IR
ONE canonical lexer vocabulary
ONE canonical ownership per production
ONE language
OPEN-WORLD operations
NO fixed hardware limits
NO fixed qubit limits
NO fixed register limits
NO fixed control limits
NO fixed parameter limits
NO fixed circuit limits
NO hardware discovery during parsing
NO runtime execution during parsing
NO QEC implementation in grammar
NO ZQN implementation in grammar
NO routing implementation in grammar
NO scheduling implementation in grammar
NO optimization implementation in grammar
NO calibration implementation in grammar
NO simulator implementation in grammar
NO unsafe Rust
NO embedded Rust actions
NO circular grammar dependencies
NO silent vendor lock-in
NO competing quantum IR

And:

YES target independence
YES capability-based design
YES resource-based design
YES generic operations
YES logical/physical separation
YES quantum/classical composition
YES dynamic circuits
YES extensibility
YES dialects
YES interoperability
YES deterministic parsing
YES source spans
YES diagnostics
YES scalability
YES conformance testing
YES POCO-REAF

---

178. Final architecture

The production quantum subsystem must ultimately be:

                         Zamani Source
                              │
                              ▼
                     grammar/Zamani.g4
                              │
                              ▼
                     canonical lexer
                              │
                              ▼
                     canonical parser
                              │
                              ▼
                 ┌───────────────────────┐
                 │   Quantum grammar     │
                 │                       │
                 │ operations            │
                 │ qubits                │
                 │ registers             │
                 │ states                │
                 │ measurement           │
                 │ controls              │
                 │ circuits              │
                 │ kernels               │
                 │ dynamic control       │
                 │ classical feedforward │
                 │ observables           │
                 │ channels              │
                 │ noise intent          │
                 │ QEC intent            │
                 │ resources             │
                 │ capabilities          │
                 │ dialects              │
                 └───────────┬───────────┘
                             │
                             ▼
                    domain-neutral AST
                             │
                             ▼
                    semantic analysis
                             │
              ┌──────────────┼──────────────┐
              │              │              │
           types          effects       resources
              │              │              │
              └──────────────┼──────────────┘
                             │
                             ▼
                      quantum semantics
                             │
                             ▼
                        quantum::ir
                             │
          ┌──────────────────┼──────────────────┐
          │                  │                  │
          ▼                  ▼                  ▼
      optimization        routing          scheduling
          │                  │                  │
          └──────────────────┼──────────────────┘
                             │
                             ▼
                      QEC / resilience
                             │
                             ▼
                            ZQN
                             │
                             ▼
                            HAL
                             │
                             ▼
                    target realization
                             │
             ┌───────────────┼────────────────┐
             │               │                │
             ▼               ▼                ▼
          simulator         QPU          heterogeneous
             │               │             hardware
             └───────────────┼────────────────┘
                             │
                             ▼
                          runtime

---

179. Final principle

The success criterion for "grammar/quantum/" is not:

«"Does the grammar know every quantum gate and every QPU?"»

The success criterion is:

«Can Zamani express quantum computation, quantum intent, requirements, capabilities and constraints once, in a target-independent form, and allow the rest of the toolchain to realize that same semantic program across different scales, architectures and future quantum systems without imposing artificial language limits?»

Therefore:

ONE PROGRAM
     ↓
ONE LANGUAGE
     ↓
ONE SEMANTIC MEANING
     ↓
ONE CANONICAL quantum::ir
     ↓
MANY OPTIMIZATIONS
     ↓
MANY ROUTINGS
     ↓
MANY SCHEDULES
     ↓
MANY QEC/RESILIENCE STRATEGIES
     ↓
MANY HARDWARE TARGETS
     ↓
MANY SCALES

The grammar describes the computation.

The semantic layer determines the meaning.

"quantum::ir" provides the canonical quantum representation.

Optimization determines an efficient realization.

Routing determines placement.

Scheduling determines execution order and timing.

QEC determines fault-tolerant realization.

ZQN determines noise/fault semantics.

HAL determines available hardware capabilities.

The runtime determines execution.

That separation is the foundation of quantum POCO-REAF and of Zamani's goal of scaling from the smallest meaningful quantum computation to arbitrarily large systems constrained only by the resources actually available—not by arbitrary limits embedded in the language grammar.

"grammar/quantum/README.md" is complete when every file beneath "grammar/quantum/" can be judged against this contract independently, without creating a second quantum language, second AST, second quantum IR, or hidden hardware ceiling.
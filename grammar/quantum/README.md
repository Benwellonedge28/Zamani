

# Zamani Quantum Grammar

**Path:** `grammar/quantum/`  
**Status:** Normative quantum-grammar architecture and integration contract  
**Language:** Zamani  
**Grammar technology:** ANTLR4  
**Rust baseline:** Rust 1.97+ / Rust 2021  
**Safety:** Safe Rust only; `unsafe` is prohibited  
**Portability model:** Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)

---

## 1. Purpose

`grammar/quantum/` is the quantum-language grammar subsystem of Zamani.

It defines the source-level syntax required to express quantum computation while preserving:

- target independence;
- classical/quantum composition;
- arbitrary program scale;
- open-ended quantum operations;
- symbolic resource requirements;
- capability requirements;
- dynamic circuits;
- measurement and feed-forward;
- observables;
- quantum channels;
- noise intent;
- error-correction intent;
- logical quantum computation;
- physical-resource intent where explicitly requested;
- quantum kernels;
- quantum/classical interaction;
- adaptive execution;
- quantum learning;
- quantum reasoning;
- quantum interoperability;
- quantum dialect extensibility;
- provenance;
- contracts;
- policies;
- deterministic and reproducible source semantics.

This directory is part of the single Zamani language.

It is **not** a second language.

It is **not** a second compiler.

It is **not** a second AST.

It is **not** a second quantum semantic model.

It is **not** a second quantum IR.

---

# 2. Architectural position

The quantum grammar participates in the following canonical pipeline:

```text
Zamani source
    |
    v
Canonical lexer
    |
    v
Canonical ANTLR parser
    |
    v
Domain-neutral frontend AST
    |
    v
Structural validation
    |
    +--> type analysis
    +--> ownership analysis
    +--> effect analysis
    +--> capability analysis
    +--> resource analysis
    +--> contract analysis
    +--> policy analysis
    +--> provenance
    |
    v
Semantic model
    |
    +--> classical semantics
    +--> quantum semantics
    +--> hybrid semantics
    +--> AI/learning/reasoning semantics
    +--> distributed semantics
    +--> HDL/hardware intent
    |
    v
Canonical IR
    |
    +--> classical IR
    |
    +--> quantum::ir
    |
    v
Target-independent optimization
    |
    v
Lowering
    |
    v
Decomposition
    |
    v
Routing / placement
    |
    v
Scheduling
    |
    v
Resilience / recovery / QEC
    |
    v
ZQN
    |
    v
HAL
    |
    v
Target realization

The quantum grammar is responsible only for the source-language boundary.


---

3. Normative authority

The following ownership hierarchy is mandatory.

3.1 Repository architecture

The repository-wide architecture is governed by:

grammar/DESIGN.md

It defines:

grammar authority;

parser composition;

AST boundaries;

semantic boundaries;

IR ownership;

resource abstraction;

capability abstraction;

effect abstraction;

contracts;

policies;

provenance;

dialects;

compatibility;

portability;

scalability;

safety;

POCO-REAF.


No quantum grammar file may contradict it.


---

3.2 Quantum specification

The normative quantum-language semantics are defined by:

grammar/spec/quantum.md

Quantum grammar files implement the syntax described by that specification.

The specification remains the semantic authority.


---

3.3 General language specification

Quantum grammar must reuse the repository-wide definitions for:

grammar/spec/syntax.md
grammar/lexer/
grammar/core/
grammar/types/
grammar/expressions/
grammar/statements/
grammar/declarations/
grammar/functions/
grammar/modules/

Quantum files must not independently redefine common language concepts.


---

3.4 Complete-language composition

The complete Zamani grammar is composed by:

grammar/Zamani.g4

The quantum subsystem is composed by:

grammar/quantum/quantum.g4

Zamani.g4 remains the complete-language composition root.

quantum.g4 remains the quantum-domain composition root.

Neither root should duplicate leaf grammar rules.


---

4. AST ownership

The quantum grammar does not own the frontend AST.

The canonical frontend AST remains outside this directory.

The grammar produces parser structures that are converted into the repository's domain-neutral AST.

The AST must remain independent of:

LLVM;

QIR;

MLIR;

vendor instruction sets;

physical qubit maps;

QPU topology;

calibration;

routing decisions;

scheduling decisions;

pulse implementations;

QEC implementation details.


Quantum syntax may identify quantum intent.

It must not turn the frontend AST into a physical backend representation.


---

5. Canonical quantum IR

The canonical quantum semantic/IR boundary is:

quantum::ir

There must be exactly one canonical quantum IR.

This directory must not introduce competing structures such as:

QuantumIR
QuantumOperationIR
QuantumGateIR
QuantumCircuitIR
QuantumHardwareIR
QuantumBackendIR

when those structures would duplicate the canonical semantic/IR responsibility.

The intended flow is:

quantum grammar
    |
    v
domain-neutral AST
    |
    v
quantum semantic model
    |
    v
quantum::ir

After that boundary, optimization and target realization belong to downstream compiler layers.


---

6. Core principle

The fundamental quantum grammar rule is:

> Quantum syntax expresses portable computational meaning and intent. Downstream compilation determines realization.



The grammar may express:

quantum operations;

quantum operands;

parameters;

measurements;

states;

circuits;

dynamic control;

classical feed-forward;

channels;

noise intent;

observables;

logical operations;

error-correction intent;

resource requirements;

capability requirements;

constraints;

preferences;

hints;

policies;

provenance;

adaptive execution;

quantum/classical composition.


The grammar must not decide:

which QPU is selected;

which simulator is selected;

which physical qubits are selected;

how routing occurs;

how mapping occurs;

how an operation is decomposed;

which native instruction is selected;

how pulses are generated;

how calibration is performed;

how QEC is physically implemented;

which provider is contacted;

how runtime resources are allocated.



---

7. POCO-REAF requirement

Quantum source programs must be target-independent wherever the programmer has expressed target-independent intent.

The same semantic program should be able to participate in compilation for:

tiny systems
CPU systems
multicore systems
GPU systems
FPGA systems
ASIC systems
accelerators
quantum simulators
QPU systems
heterogeneous systems
HPC systems
clusters
distributed systems
cloud systems
future computational architectures

The source program must not need semantic rewriting merely because a target changes.

A target may instead report:

capability unavailable
resource insufficient
constraint unsatisfied
policy conflict
unsupported realization

rather than silently changing program meaning.


---

8. No artificial grammar limits

The quantum grammar must impose no universal machine-size limits.

It must not define constants such as:

MAX_QUBITS
MAX_LOGICAL_QUBITS
MAX_PHYSICAL_QUBITS
MAX_REGISTERS
MAX_TARGETS
MAX_CONTROLS
MAX_PARAMETERS
MAX_OPERATIONS
MAX_GATES
MAX_CIRCUIT_DEPTH
MAX_CIRCUIT_WIDTH
MAX_MEASUREMENTS
MAX_SHOTS
MAX_CHANNELS
MAX_QPUS
MAX_DEVICES
MAX_MEMORY

or equivalent hidden limits.

The same rule applies to numeric literals embedded in grammar rules.

For example:

64 qubits
128 targets
1024 operations
32 controls

must never become parser-level ceilings.

A number appearing in source is program data or semantic intent.

It must not become a language capacity limit.


---

9. Physical limits are not grammar limits

The language can be open-ended while real execution remains finite.

Actual execution can be constrained by:

available memory;

available compute;

available QPU resources;

simulator capacity;

accelerator capacity;

topology;

timing;

energy;

thermal conditions;

reliability;

calibration;

provider constraints;

user policies;

compiler resources.


Those conditions belong to semantic analysis, resource negotiation, compilation, scheduling, deployment, or runtime.

They do not belong in universal grammar cardinality rules.


---

10. Requirement, capability, resource and target separation

The quantum subsystem must preserve these distinctions:

Concept	Meaning

Requirement	Must be satisfied
Constraint	Must not be violated
Preference	Desired but potentially negotiable
Hint	Advisory information
Capability	Property supplied by an environment
Resource	Abstract quantity or facility
Target	Possible realization destination
Placement	Explicit realization intent
Mapping	Logical-to-real realization information
Policy	Rule controlling permitted behavior


Examples:

requires qubits >= required_qubits;

is a requirement.

requires capability("quantum.mid_circuit_measurement");

is a capability requirement.

prefer capability("quantum.dynamic_control");

is a preference.

hint resource("quantum_memory");

is advisory.

None of these means:

use device 0
use QPU 1
use physical qubit 17
use topology X

unless such realization intent is explicitly expressed by a separate, authorized construct.


---

11. Open-world quantum operations

Quantum operations are open-ended.

The grammar must not encode a permanently closed catalogue such as:

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

These names may be recognized by:

standard libraries;

operation registries;

semantic libraries;

dialects;

target descriptions;

interoperability layers.


They are not the fundamental grammar model.

The grammar must support operation identities such as:

H
custom_operation
quantum::operation
library::operation
vendor::operation
future::operation

without requiring a grammar modification for every new operation.


---

12. Canonical quantum operation model

The conceptual operation model is:

QuantumOperation
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

The concrete Rust representation belongs downstream.

The grammar only provides the syntactic structure required to construct that semantic representation.


---

13. Operation syntax

The canonical conceptual shape is:

quantumOperation
    operationSpecifier
    operationParameters?
    operationTargets

The operation subsystem must support:

operation identity
operation parameters
operation targets
operation modifiers
operation attributes
operation controls

without requiring future operations to be added to the grammar.


---

14. Operation namespaces

Operation names must reuse the repository's canonical qualified-name system.

Examples:

library::operation
vendor::operation
domain::operation
future::operation
library::quantum::operation

Quantum grammar must not create an independent namespace system.

Common ownership remains in:

grammar/core/
grammar/modules/
grammar/expressions/

where applicable.


---

15. Gate semantics versus grammar

gates.g4 must not become a closed gate catalogue.

Gate syntax may describe:

operation declarations;

operation names;

gate parameters;

gate bodies;

gate-level modifiers.


Semantic analysis determines:

whether an operation exists;

whether it is unitary;

whether it has an adjoint;

whether it has an inverse;

whether it is decomposable;

whether it is equivalent to another operation;

whether a target supports it.


These are not parser responsibilities.


---

16. Quantum directory ownership model

The quantum directory must have one owner for each semantic concept.

The expected ownership model is:

File	Responsibility

quantum.g4	Quantum composition root
operations.g4	Generic operation invocation
gates.g4	Gate declaration/name syntax
parameterized-operations.g4	Parameterized-operation compatibility/integration
controlled-operations.g4	Extended controlled-operation syntax
controls.g4	Control/modifier syntax
adjoints.g4	Adjoint/inverse syntax
qubits.g4	Qubit source syntax
registers.g4	Quantum-register syntax
logical-qubits.g4	Logical-qubit intent
physical-qubits.g4	Explicit physical-resource syntax
types.g4	Canonical quantum type syntax
quantum-types.g4	Compatibility/legacy surface if overlapping
states.g4	Canonical state syntax
quantum-states.g4	Compatibility/legacy surface if overlapping
measurement.g4	Measurement syntax
reset.g4	Reset syntax
barriers.g4	Barrier syntax
observables.g4	Observable syntax
channels.g4	Quantum-channel syntax
noise.g4	Noise intent
error-correction.g4	QEC intent
logical-operations.g4	Logical-operation syntax
dynamic-circuits.g4	Dynamic-circuit structure
dynamic-control.g4	Dynamic control integration
mid-circuit-control.g4	Measurement-dependent control
quantum-classical.g4	Quantum/classical boundary
classical-feedforward.g4	Classical feed-forward
quantum-capabilities.g4	Quantum capability requirements
quantum-resources.g4	Quantum resource contracts
resource-requirements.g4	Resource integration/compatibility
circuits.g4	Circuit structure
kernels.g4	Quantum kernel structure
parameters.g4	Quantum parameter syntax
pulse-intent.g4	Abstract pulse-level intent
quantum-dialects.g4	Quantum dialect extension
quantum-learning.g4	Quantum-learning integration
quantum-reasoning.g4	Quantum reasoning integration
adaptive-quantum-execution.g4	Adaptive quantum execution composition
README.md	Quantum subsystem architecture


The exact final file set may grow as capabilities expand.

Any new file must declare its ownership before it is added.


---

17. One concept, one canonical owner

The following overlaps require explicit ownership resolution:

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

Every overlapping file must be classified as exactly one of:

1. canonical implementation;


2. composition wrapper;


3. compatibility grammar;


4. historical/reference grammar;


5. migration surface;


6. removable obsolete surface.



Two files must never silently define the same canonical production.


---

18. quantum.g4

quantum.g4 is the quantum composition root.

It owns:

quantum-domain dispatch;

quantum declaration dispatch;

quantum statement dispatch;

quantum expression dispatch;

quantum type dispatch;

quantum subsystem composition;

quantum dialect dispatch.


It does not own detailed leaf syntax.

It must compose canonical leaf grammars.


---

19. Composition hierarchy

The intended hierarchy is:

grammar/Zamani.g4
        |
        v
grammar/quantum/quantum.g4
        |
        +-- operations
        +-- parameters
        +-- qubits
        +-- registers
        +-- types
        +-- states
        +-- measurement
        +-- reset
        +-- circuits
        +-- kernels
        +-- controls
        +-- adjoints
        +-- observables
        +-- channels
        +-- noise
        +-- dynamic control
        +-- classical feed-forward
        +-- quantum/classical integration
        +-- resources
        +-- capabilities
        +-- resilience
        +-- QEC
        +-- learning
        +-- reasoning
        +-- adaptive execution
        +-- dialects

The exact import graph must contain each canonical grammar exactly once.


---

20. Canonical lexer

Quantum grammars must use the repository's canonical lexer.

No quantum file may introduce an independent lexer.

Quantum operation names should normally remain identifiers rather than lexer keywords.

The lexer should reserve a word only when the word has stable language-level grammatical meaning.

For example, future operation names must not require new lexer keywords.


---

21. Identifier reuse

Quantum grammars must reuse the canonical:

identifier
qualifiedName
namespace
path
generic arguments
expression
literal
attribute
annotation

rules.

Do not redefine them independently inside quantum files.


---

22. Qubit ownership

qubits.g4 owns source syntax for:

qubit declarations;

qubit references;

qubit collections;

symbolic references;

indexed access;

aliases;

ranges;

slices where supported.


It does not own:

allocation;

routing;

placement;

physical mapping;

coupling maps;

calibration;

QPU discovery.



---

23. Register ownership

registers.g4 owns:

quantum-register declarations;

register references;

indexing;

slicing;

symbolic extents;

register expressions.


Register cardinality is semantic/program data.

The grammar must never define a maximum register size.


---

24. Quantum types

types.g4 is the canonical owner of quantum type syntax.

The semantic system may represent concepts including:

Qubit
Qubit[n]
LogicalQubit
PhysicalQubit
QuantumRegister
QuantumState
MeasurementResult
Observable
QuantumChannel

These are semantic types.

They do not imply hardware capacities.


---

25. Quantum states

State syntax may represent:

basis states;

named states;

symbolic states;

state constructors;

state references;

preparation intent.


Examples may include:

|0>
|1>
|+>
|->
|psi>

where permitted by the lexical specification.

The grammar does not allocate amplitudes.

The grammar does not simulate state vectors.

The grammar does not impose simulator dimensions.


---

26. Quantum literals

Quantum literals belong to the lexical and syntax contracts.

Semantic analysis determines:

dimensional validity;

normalization;

type compatibility;

state validity;

preparation semantics.


No finite catalogue of state representations should become a universal language ceiling.


---

27. Measurement

measurement.g4 owns source measurement syntax.

It must support the semantic forms required for:

measurement targets;

result bindings;

measurement bases;

destinations;

repeated measurement intent;

mid-circuit measurement;

final measurement.


It must not define a maximum measurement count.


---

28. Measurement/classical boundary

Measurement results enter the ordinary Zamani semantic value/type system.

The conceptual flow is:

quantum measurement
        |
        v
measurement result
        |
        v
classical semantic value
        |
        v
classical expression/control

The quantum grammar must not create a second classical expression language.


---

29. Dynamic circuits

Dynamic circuits permit later computation to depend on information produced during execution.

Conceptually:

measure q -> result

if result {
    apply operation(...);
}

The exact surface syntax must use the repository's canonical conditional/control grammar.

Whether a target supports dynamic execution is a capability issue.

The parser must not reject valid source merely because a particular backend lacks the capability.


---

30. Classical feed-forward

Classical feed-forward belongs at the quantum/classical integration boundary.

The architecture is:

measurement
    |
    v
classical result
    |
    v
classical computation
    |
    v
control decision
    |
    v
quantum operation

This must integrate with existing:

grammar/classical/
grammar/expressions/
grammar/statements/
grammar/quantum/

without creating a duplicate expression system.


---

31. Quantum/classical hybrid computation

Quantum syntax must be composable with ordinary Zamani computation.

Supported semantic relationships include:

classical -> quantum
quantum -> classical
classical control -> quantum
quantum measurement -> classical
classical optimization -> quantum kernel
AI model -> quantum operation
quantum result -> AI model

The semantic boundary belongs to the hybrid subsystem.

The quantum grammar only supplies the quantum-side syntax.


---

32. Quantum learning

Quantum learning must integrate with the universal learning model.

The intended architecture is:

learning semantics
       |
       +---- classical model
       |
       +---- quantum model
       |
       +---- hybrid model
       |
       v
canonical semantic representation
       |
       v
quantum::ir where quantum computation is present

The grammar must not enumerate every machine-learning algorithm.

Algorithms belong to libraries, semantic registries, dialects, or capabilities.


---

33. Quantum reasoning

Reasoning constructs must use the repository-wide reasoning and knowledge abstractions.

Quantum reasoning may consume:

measurement evidence;

quantum state metadata;

experimental data;

classical facts;

probabilistic information;

provenance;

contracts.


The quantum subsystem must not create an independent reasoning language.


---

34. Uncertainty and probability

Quantum computation naturally interacts with uncertainty.

The grammar must integrate with the common type/semantic model for:

probability
distribution
confidence
uncertainty
observation
belief

The grammar must not hard-code one probability representation or backend.


---

35. Observables

observables.g4 owns source syntax for observable intent.

Semantic analysis determines:

observable validity;

type;

compatibility;

measurement strategy;

decomposition;

execution strategy.


The grammar does not select a physical measurement implementation.


---

36. Channels

channels.g4 owns source syntax for channel intent.

The semantic layer determines:

channel validity;

dimensional compatibility;

composability;

representation;

realization.


The grammar must not enumerate every possible future channel.


---

37. Noise

noise.g4 expresses noise intent.

It must not become a provider-specific noise instruction set.

Noise models should be extensible through semantic metadata, libraries, dialects, or capability descriptions.


---

38. Error correction

error-correction.g4 expresses error-correction intent.

It must not encode:

fixed code sizes;

fixed physical-qubit counts;

provider-specific decoder limits;

routing algorithms;

calibration;

hardware topology.


QEC implementation belongs downstream.

The canonical quantum flow remains:

source
  |
  v
quantum::ir
  |
  v
optimization
  |
  v
decomposition
  |
  v
routing
  |
  v
scheduling
  |
  v
QEC / resilience
  |
  v
ZQN
  |
  v
HAL


---

39. Logical versus physical resources

Logical quantum intent and physical realization must remain separate.

Logical constructs can express:

logical qubit
logical operation
logical circuit
logical resource requirement

Physical constructs may be used only when the programmer explicitly requests physical realization information.

Physical syntax must never silently contaminate ordinary target-independent source.


---

40. Physical qubits

physical-qubits.g4 may represent explicit physical-resource intent.

It must not make physical qubits mandatory for ordinary quantum programs.

The normal POCO-REAF path is:

logical intent
    |
    v
semantic analysis
    |
    v
resource/capability negotiation
    |
    v
placement/mapping
    |
    v
physical realization


---

41. Controls and modifiers

Controlled operations, adjoints and inverses must be represented structurally.

The grammar should support recursive composition where specified:

control(operation)
adjoint(operation)
inverse(operation)

control(adjoint(operation))
inverse(control(operation))

No fixed modifier nesting depth may be encoded.

Semantic validation determines whether a requested modifier is valid.


---

42. Parameters

Parameter syntax must remain separate from target operands.

Conceptually:

operation(parameters)(targets)

The exact canonical form is determined by the operation grammar.

Parameter cardinality must remain open-ended.

Parameter validity is semantic.


---

43. Quantum kernels

kernels.g4 owns quantum-kernel structure.

A kernel describes quantum computation as a reusable semantic unit.

It does not select:

QPU;

simulator;

CPU;

GPU;

FPGA;

ASIC;

topology;

pulse implementation.


Those are downstream concerns.


---

44. Circuits

circuits.g4 owns circuit-level grouping and composition.

A circuit is a source-level semantic structure.

It does not imply a particular physical circuit representation.

Circuit size is not bounded by grammar constants.


---

45. Adaptive quantum execution

adaptive-quantum-execution.g4 owns only the source-level composition boundary for adaptive quantum execution.

It must integrate existing adaptive execution and resilience mechanisms rather than duplicate them.

The intended semantic sequence is:

observe
    |
    v
evaluate
    |
    v
select strategy
    |
    v
execute
    |
    v
evaluate outcome
    |
    +--> accept
    +--> degraded accept
    +--> retry
    +--> recover
    +--> escalate
    +--> reject

Adaptive execution must remain governed by:

capabilities;

resources;

effects;

contracts;

policies;

authorization;

provenance.


It must never imply unrestricted self-modification.


---

46. Resilience states

The canonical resilience state model is owned by the resilience subsystem.

The currently defined semantic states are:

Unknown
Healthy
Degraded
Unstable
Unavailable
Recovering
Quarantined
Retired

Quantum grammar files must reference the canonical resilience model rather than redefine it.


---

47. Execution outcomes

The canonical semantic outcomes are:

ACCEPT
DEGRADED_ACCEPT
RETRY
RECOVER
ESCALATE
REJECT

These are semantic/runtime outcomes.

They are not hardware limits.

They must not be redefined independently by individual quantum grammars.


---

48. Resource requirements

Quantum resource requirements must use the repository-wide resource abstraction.

Examples:

requires qubits >= required_qubits;
requires memory >= required_memory;
requires capability("quantum.measurement");
requires capability("quantum.dynamic_control");
requires topology(required_topology);

The grammar must preserve symbolic expressions.

It must not convert them into fixed machine constants.


---

49. Capabilities

Quantum capabilities describe what an environment can provide.

Examples include conceptual capabilities such as:

quantum.measurement
quantum.mid_circuit_measurement
quantum.dynamic_control
quantum.feedback
quantum.error_correction
quantum.reset
quantum.observable_measurement
quantum.parameterized_execution

The list is open-ended.

New capabilities must not require changing the fundamental operation grammar.


---

50. Effects

Quantum operations may participate in the universal effect system.

Possible effects include:

quantum
measurement
randomness
learning
adaptation
simulation
distributed
io
network
foreign
native

The grammar expresses source constructs.

Effect checking occurs downstream.


---

51. Contracts

Quantum constructs must integrate with the universal contract system.

Applicable concepts include:

requires
ensures
invariant
assume
guarantee
property
assert

Examples of semantic use include:

requires capability("quantum.measurement");
ensures measurement_result_valid;
invariant state_condition;

Contract syntax is owned by the common validation/contract subsystem.

Quantum grammar must not create a second contract language.


---

52. Policies

Quantum execution can be constrained by policies governing:

resource use;

capability use;

adaptation;

execution;

simulation;

security;

deployment;

provenance;

fallback;

resilience.


Policy syntax belongs to the universal policy subsystem.

Quantum files consume that policy model.


---

53. Provenance

Quantum computation must be compatible with the universal provenance system.

Provenance may capture:

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

Quantum grammar must preserve enough source structure for downstream provenance.

The grammar itself does not generate timestamps or runtime records.


---

54. Explainability

Quantum execution can produce decisions that need explanation.

The universal explanation model may explain:

why an operation was selected;

why a resource was required;

why a target was selected;

why a decomposition was chosen;

why routing changed;

why recovery occurred;

why execution was rejected.


These are downstream semantic/compiler/runtime responsibilities.

The quantum grammar only supplies source constructs that request or constrain such behavior.


---

55. Simulation

Simulation is an execution strategy, not a separate language.

Quantum programs must be capable of participating in:

exact simulation
approximate simulation
state simulation
circuit simulation
noise simulation
fault simulation
performance simulation
distributed simulation
hybrid simulation

The grammar must not impose simulator-specific limits.


---

56. Dialects

Quantum dialects provide controlled extensibility.

A dialect may introduce:

operation declarations;

semantic metadata;

domain-specific constructs;

external representations;

vendor-specific features.


A dialect must not silently modify the meaning of core Zamani syntax.

Dialect extensions must declare:

name
version
syntax
semantic owner
capabilities
effects
resources
compatibility
lowering
provenance
tests


---

57. Interoperability

Quantum interoperability must support external representations without making them the canonical Zamani grammar.

Examples may include:

OpenQASM
external circuit formats
quantum IR interchange
provider-specific formats
research formats
future formats

The architecture is:

external format
    |
    v
format-specific frontend
    |
    v
Zamani semantic model
    |
    v
quantum::ir

No external format becomes a second Zamani semantic authority.


---

58. OpenQASM integration

OpenQASM support must remain under its interoperability/frontend boundary.

Its operation names and syntax must be translated into the canonical Zamani semantic model.

The existing OpenQASM implementation must not force Zamani's core grammar to become OpenQASM-specific.


---

59. AI, reasoning and learning integration

Quantum computation must consume the universal semantic systems for:

reasoning
knowledge
learning
adaptation
uncertainty
evidence
explanation
provenance
agents
policies

The quantum subsystem must not duplicate those concepts.

The integration should be:

universal semantic primitive
        |
        +--> classical realization
        |
        +--> quantum realization
        |
        +--> hybrid realization


---

60. Multi-agent integration

Quantum agents must reuse the existing concurrency/actor architecture.

The intended relationship is:

AI agent
    |
    v
actor
    |
    v
message
    |
    v
channel/task
    |
    v
scheduler

Quantum grammar must not create a second actor runtime.


---

61. Security and sandboxing

Quantum execution must integrate with the universal security model.

A sandbox may constrain:

effects
capabilities
resources
network access
filesystem access
native calls
foreign calls
reflection
adaptation
deployment

Security syntax belongs to the security subsystem.

Quantum grammar consumes the resulting policy/capability model.


---

62. Interoperability with FFI/ABI

Quantum code may interface with external systems through the universal interoperability subsystem.

The boundary is:

Zamani quantum source
    |
    v
foreign declaration
    |
    v
ABI contract
    |
    v
effect/capability analysis
    |
    v
backend

Quantum grammar must not create a second FFI syntax.


---

63. Metaprogramming

Quantum metaprogramming must use the common metaprogramming architecture.

Possible facilities include:

reflection;

introspection;

compile-time execution;

code generation;

quotation;

syntax trees;

type-level computation.


These facilities must be governed by:

explicit capabilities;

effects;

policies;

provenance.


Unrestricted runtime self-modification is not implied.


---

64. Determinism and reproducibility

The quantum grammar must be deterministic.

Grammar files must contain:

no embedded Rust actions;

no random behavior;

no network access;

no filesystem access;

no hardware access;

no runtime calls;

no provider calls.


Reproducibility is enforced through compiler and provenance layers.

Where quantum execution is inherently probabilistic, the semantic model must distinguish:

deterministic compilation

from:

probabilistic execution


---

65. Safe Rust requirement

The grammar itself must not contain embedded Rust implementation code.

The generated Rust frontend must remain compatible with:

Rust 2021
Rust 1.97+
safe Rust
no unsafe

Quantum grammar changes must not introduce a requirement for unsafe.


---

66. Public/private rule discipline

Every .g4 file must explicitly identify:

Public Rules
Private Rules

Public rules are stable integration boundaries.

Private rules may be refactored internally without affecting consumers.

A file must not expose internal implementation rules as accidental public API.


---

67. Required feature contract for every grammar file

Every quantum .g4 file must contain or have an adjacent authoritative contract defining:

Purpose
Owns
Does Not Own
Public Rules
Private Rules
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
Quantum::IR Boundary
Classical Boundary
HDL Boundary
Backend Boundary
Diagnostics
Positive Tests
Negative Tests
Boundary Tests
Scalability Tests
Compatibility
Integration
Completion Criteria

This contract is mandatory.


---

68. File dependency contract

Every quantum grammar file must declare:

DEPENDS_ON:
EXPORTS:
CONSUMED_BY:
AST_OWNER:
SEMANTIC_OWNER:
IR_OWNER:
TEST_OWNER:
SPEC_OWNER:

For example:

DEPENDS_ON:
    lexer/tokens.g4
    core/names.g4
    expressions/expressions.g4
    quantum/parameters.g4

EXPORTS:
    quantumOperation
    quantumOperationSpecifier

CONSUMED_BY:
    quantum/quantum.g4
    quantum/dynamic-circuits.g4
    quantum/kernels.g4

AST_OWNER:
    canonical frontend AST

SEMANTIC_OWNER:
    quantum semantic layer

IR_OWNER:
    quantum::ir

TEST_OWNER:
    grammar/tests/quantum/

SPEC_OWNER:
    grammar/spec/quantum.md

The actual entries must reflect the repository's current canonical ownership.


---

69. Dependency direction

Dependencies must flow toward foundational abstractions.

The intended direction is:

lexer
  |
  v
core
  |
  v
expressions/types/statements
  |
  v
quantum leaf grammar
  |
  v
quantum composition
  |
  v
complete language composition

Quantum leaf grammars must not depend on:

backend implementations;

QPU drivers;

runtime instances;

physical calibration;

target-specific scheduling;

vendor SDK implementations.



---

70. No circular grammar ownership

No two grammar files may depend on each other merely to obtain each other's private rules.

If two domains need a common concept, move the common concept to its canonical owner.

For example:

quantum
    |
    +--> common expression

classical
    |
    +--> common expression

rather than:

quantum <--> classical

through duplicated definitions.


---

71. Error handling

The grammar must provide structurally precise syntax errors.

Semantic errors must remain semantic errors.

Examples of semantic errors include:

unknown quantum operation
invalid operation parameters
invalid target type
invalid modifier
missing capability
insufficient resource
unsupported dynamic execution
invalid contract
policy violation
invalid provenance relationship

The parser must not attempt to perform semantic resource or hardware validation.


---

72. Error recovery

ANTLR error recovery must remain compatible with the repository-wide parser strategy.

Quantum grammar must avoid ambiguous alternatives where a simpler unambiguous representation exists.

The grammar must not use semantic predicates merely to simulate semantic analysis.


---

73. Negative syntax testing

Every canonical quantum construct must have negative tests.

Examples include:

missing operation
missing targets
unbalanced delimiters
invalid modifier structure
invalid parameter syntax
invalid declaration structure
invalid quantum/classical boundary

Semantic-negative tests belong to semantic test suites rather than parser-only tests.


---

74. Scalability testing

Scalability tests must verify that syntax remains valid as source cardinality grows.

Tests must not depend on artificial constants such as:

MAX_QUBITS
MAX_TARGETS
MAX_PARAMETERS
MAX_CONTROLS
MAX_OPERATIONS

Instead, tests should generate workloads based on available test resources.

The grammar should remain structurally valid until an external implementation or environment limit is reached.


---

75. Cross-domain testing

Quantum grammar must be tested together with:

classical
hybrid
AI
data
concurrency
distributed
networking
HDL
hardware
security
interoperability
metaprogramming
execution
resources
effects
validation
policies
provenance

The goal is not merely to prove that quantum syntax parses.

The goal is to prove that quantum syntax composes with the rest of Zamani.


---

76. Mandatory integration examples

The quantum grammar test corpus should include at minimum:

minimal quantum program
classical/quantum hybrid program
parameterized operation
custom operation
qualified operation
controlled operation
adjoint operation
inverse operation
dynamic circuit
mid-circuit measurement
classical feed-forward
observable measurement
channel
noise intent
logical operation
resource requirement
capability requirement
contract
policy
provenance
adaptive execution
quantum learning
quantum reasoning
simulation
interoperability


---

77. Mandatory POCO-REAF test

At least one conformance program must express quantum intent without selecting a specific machine.

Conceptually:

requires capability("quantum.measurement");
requires qubits >= required_qubits;

apply operation(parameters)(targets);

measure targets -> result;

if result {
    apply next_operation(targets);
}

The same semantic source must be eligible for multiple realizations.

The compiler may choose different:

decomposition;

routing;

scheduling;

QEC;

simulator;

accelerator;

QPU;

runtime strategy.


The source meaning must remain stable.


---

78. Mandatory large-scale test

A scalability test must construct quantum programs whose size is determined by:

program data
symbolic extents
available resources
test configuration

rather than parser constants.

For example:

n = symbolic_or_runtime_extent;

must remain representable without grammar changes as n grows.


---

79. Hard-coding audit

Every quantum grammar file must pass the following audit.

It must contain no:

finite operation catalogue
finite hardware catalogue
finite vendor catalogue
fixed qubit count
fixed target count
fixed control count
fixed parameter count
fixed circuit width
fixed circuit depth
fixed resource capacity
fixed topology
fixed QPU count
fixed device count
backend selection
runtime discovery
hardware probing
provider API calls
embedded Rust
unsafe code

A finite list of lexical keywords is acceptable only when those keywords have genuine language-level grammatical meaning.


---

80. What belongs outside this directory

The following must remain outside the quantum grammar:

QPU discovery
device selection
physical calibration
routing implementation
placement algorithms
scheduling algorithms
pulse synthesis
QEC implementation
decoder implementation
backend drivers
runtime resource allocation
provider APIs
hardware probing
simulation engines
optimization algorithms
compiler cost models

Quantum grammar expresses the input to these systems.


---

81. Quantum compiler boundary

The canonical downstream boundary is:

quantum syntax
    |
    v
frontend AST
    |
    v
quantum semantic model
    |
    v
quantum::ir

After quantum::ir:

optimization
    |
    v
decomposition
    |
    v
routing
    |
    v
scheduling
    |
    v
resilience / QEC
    |
    v
ZQN
    |
    v
HAL
    |
    v
target

No grammar file may bypass this architecture.


---

82. Quantum grammar and hardware abstraction

The grammar may express hardware-related requirements or intent.

It must not encode hardware realization by default.

For example:

requires capability("quantum.dynamic_control");

is portable.

A hard-coded physical topology is not portable unless the programmer explicitly requests physical realization.

This distinction is essential to POCO-REAF.


---

83. Future extensibility

A future quantum technology must be addable through:

new semantic capability
new operation registration
new dialect
new interoperability adapter
new lowering
new backend
new target description

without requiring a redesign of the fundamental quantum grammar.

This is a primary production-readiness requirement.


---

84. Adding a new quantum operation

Adding a new quantum operation must normally require:

operation metadata
semantic registration
implementation/lowering
capability declaration if necessary
tests
documentation

It must not require modifying a closed grammar rule such as:

quantumOperation
    : H
    | X
    | Y
    | Z
    | ...

The fundamental grammar remains unchanged.


---

85. Adding a new hardware target

Adding a target must normally require:

target description
capabilities
resources
constraints
lowering
backend/HAL integration
tests

It must not require adding a grammar-level hardware size constant.


---

86. Adding a new quantum representation

Adding an external representation must use:

format frontend
    |
    v
Zamani semantic model
    |
    v
quantum::ir

The external representation must not become the canonical Zamani grammar.


---

87. Compatibility

Compatibility must be explicit.

Every changed public rule must declare whether it is:

stable
experimental
deprecated
compatibility-only
historical

Breaking changes require:

migration documentation;

compatibility tests;

version metadata;

diagnostics;

semantic impact analysis.



---

88. Deprecation

Deprecated quantum grammar must not silently remain a second authority.

A deprecated construct should follow:

deprecated syntax
    |
    v
diagnostic
    |
    v
canonical equivalent
    |
    v
canonical semantic model

Historical grammar must never become automatically legal syntax merely because it exists in documentation.


---

89. Documentation authority

The following distinction is mandatory:

README.md
    = architecture/navigation/integration

*.g4
    = syntax implementation

grammar/spec/quantum.md
    = normative quantum semantics

grammar/Zamani-Grammar.md
    = historical/extended reference

grammar/grammar.md
    = conformance/status

grammar/DESIGN.md
    = repository architecture

No documentation file should accidentally become a second syntax authority.


---

90. Test ownership

Quantum grammar tests should be organized under the repository's canonical test hierarchy.

Recommended structure:

grammar/tests/
    quantum/
        lexical/
        parser/
        ast/
        semantic/
        types/
        effects/
        capabilities/
        resources/
        contracts/
        policies/
        provenance/
        dynamic/
        hybrid/
        adaptive/
        learning/
        reasoning/
        interoperability/
        scalability/
        compatibility/
        negative/
        boundary/

The exact test path may follow the repository's established convention.


---

91. Required test classes

Every quantum feature must have:

positive tests
negative tests
boundary tests
scalability tests
cross-domain tests
compatibility tests
determinism tests

Where applicable it must also have:

resource tests
capability tests
effect tests
contract tests
policy tests
provenance tests
IR tests
lowering tests


---

92. Grammar conformance

A quantum grammar feature is not considered implemented merely because ANTLR accepts it.

The minimum conformance chain is:

SPECIFICATION
    |
    v
LEXER
    |
    v
GRAMMAR
    |
    v
AST
    |
    v
SEMANTICS
    |
    v
TYPE CHECKING
    |
    v
EFFECT CHECKING
    |
    v
CAPABILITY CHECKING
    |
    v
RESOURCE CHECKING
    |
    v
CONTRACT CHECKING
    |
    v
POLICY CHECKING
    |
    v
PROVENANCE
    |
    v
quantum::ir
    |
    v
OPTIMIZATION
    |
    v
LOWERING
    |
    v
ROUTING
    |
    v
SCHEDULING
    |
    v
RESILIENCE / QEC
    |
    v
ZQN
    |
    v
HAL
    |
    v
TARGET


---

93. Completion criteria for each file

A quantum grammar file is complete only when:

1. Its purpose is documented.


2. Its ownership is explicit.


3. Its non-ownership is explicit.


4. Its public rules are documented.


5. Its private rules are documented.


6. Lexer dependencies are known.


7. Grammar dependencies are known.


8. AST ownership is known.


9. Semantic ownership is known.


10. Type interactions are documented.


11. Effects are documented.


12. Capabilities are documented.


13. Resource interactions are documented.


14. Contract interactions are documented.


15. Policy interactions are documented.


16. Provenance interactions are documented.


17. IR destination is documented.


18. Quantum/classical boundaries are documented.


19. HDL/backend boundaries are documented where applicable.


20. Diagnostics are defined.


21. Positive tests exist.


22. Negative tests exist.


23. Boundary tests exist.


24. Scalability tests exist.


25. Cross-domain tests exist.


26. Compatibility behavior is defined.


27. Hard-coding audit passes.


28. No duplicate semantic owner exists.


29. The canonical AST boundary is preserved.


30. The canonical quantum::ir boundary is preserved.


31. No target-specific implementation is embedded.


32. Generated Rust remains compatible with Rust 1.97+.


33. No unsafe implementation is required.


34. Adding a future operation does not require reopening the file merely to extend a closed operation list.


35. Adding a future target does not require reopening the file merely to increase a capacity constant.




---

94. Completion criteria for grammar/quantum/

The entire directory is production-ready only when:

single lexical authority
        +
single quantum composition root
        +
single owner per semantic concept
        +
open-world operations
        +
open-ended cardinality
        +
target-independent syntax
        +
domain-neutral AST
        +
canonical semantic model
        +
canonical quantum::ir
        +
resource abstraction
        +
capability abstraction
        +
effect abstraction
        +
contract abstraction
        +
policy abstraction
        +
provenance
        +
dynamic circuits
        +
adaptive execution
        +
quantum/classical integration
        +
learning/reasoning integration
        +
interoperability
        +
dialect extensibility
        +
complete conformance tests
        =
production-ready quantum grammar


---

95. Final architecture invariant

The quantum subsystem must preserve this invariant:

SOURCE INTENT
     |
     v
OPEN-WORLD SYNTAX
     |
     v
DOMAIN-NEUTRAL AST
     |
     v
SEMANTIC VALIDATION
     |
     +--> TYPES
     +--> EFFECTS
     +--> CAPABILITIES
     +--> RESOURCES
     +--> CONTRACTS
     +--> POLICIES
     +--> PROVENANCE
     |
     v
quantum::ir
     |
     v
OPTIMIZATION
     |
     v
DECOMPOSITION
     |
     v
ROUTING
     |
     v
SCHEDULING
     |
     v
RESILIENCE / QEC
     |
     v
ZQN
     |
     v
HAL
     |
     v
TARGET

The grammar must never collapse these stages into one layer.


---

96. Final scalability invariant

The quantum grammar has no universal machine-size ceiling.

The following are intentionally open-ended:

number of qubits
number of logical qubits
number of physical qubits
number of registers
number of operations
number of parameters
number of targets
number of controls
number of measurements
number of circuits
number of kernels
number of channels
number of devices
namespace depth
modifier depth
program size
resource size

Any real limit belongs to the environment that processes the program.

Therefore the correct model is:

Zamani grammar
    |
    |  no artificial universal capacity
    v
semantic intent
    |
    v
available resources + capabilities + policies
    |
    v
feasible realization


---

97. Final POCO-REAF invariant

The quantum subsystem must make the following possible:

ONE SOURCE PROGRAM
       |
       v
ONE SEMANTIC MEANING
       |
       +-----------------------------+
       |                             |
       v                             v
SMALL SYSTEM                    LARGE SYSTEM
       |                             |
       v                             v
SIMULATOR                       QPU
       |                             |
       v                             v
ACCELERATOR                    DISTRIBUTED SYSTEM
       |                             |
       +-------------+---------------+
                     |
                     v
               FUTURE TARGET

The source program expresses computational intent.

The compiler determines realization.

The runtime determines execution.

The target provides actual capabilities and resources.

No layer may silently change the program's meaning merely to accommodate a particular machine.


---

98. Non-negotiable rules

The following rules apply to every file under grammar/quantum/:

1. No artificial universal capacity constants.


2. No fixed quantum-operation catalogue.


3. No vendor-specific core grammar.


4. No backend selection in the grammar.


5. No physical topology in target-independent syntax.


6. No routing implementation in grammar.


7. No scheduling implementation in grammar.


8. No QEC implementation in grammar.


9. No calibration implementation in grammar.


10. No runtime or hardware access from grammar.


11. No duplicate AST.


12. No duplicate quantum IR.


13. No duplicate resource system.


14. No duplicate capability system.


15. No duplicate effect system.


16. No duplicate contract system.


17. No duplicate policy system.


18. No duplicate provenance system.


19. No independent lexer.


20. No independent namespace system.


21. No embedded Rust actions.


22. No unsafe.


23. No finite operation list masquerading as universal syntax.


24. No semantic validation hidden inside parser predicates.


25. No application-specific keyword explosion.


26. No requirement to edit core grammar when a new operation is added.


27. No requirement to edit core grammar when a new hardware target is added.


28. No requirement to edit core grammar when a new provider is added.


29. All public rules have explicit ownership.


30. All files have explicit integration contracts.




---

99. Definition of production readiness

grammar/quantum/ is production-ready only when it is possible to add:

a new quantum operation
a new quantum algorithm
a new quantum learning method
a new reasoning method
a new quantum device
a new QPU
a new simulator
a new accelerator
a new backend
a new provider
a new interoperability format
a new resilience strategy
a new QEC strategy
a new routing strategy
a new scheduling strategy
a new capability
a new resource
a new policy

without changing the fundamental language architecture.

The quantum grammar must therefore describe what the program means, not what today's hardware happens to look like.

That is the quantum grammar's central contribution to POCO-REAF.

  
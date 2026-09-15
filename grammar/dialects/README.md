Zamani Dialects Grammar

Path: "grammar/dialects/README.md"
Project: Zamani Universal Programming Language
Language: Zamani
Grammar technology: ANTLR4-compatible grammar fragments
Compiler baseline: Rust 1.97 / Rust 1.97.1
Rust edition: Rust 2021
Safety: Safe Rust only; "unsafe" is prohibited
Architecture: Program Once, Compile Once, Run Everywhere, Anywhere, Forever (POCO-REAF)

---

1. Purpose

The "grammar/dialects/" directory defines the source-language syntax for declaring, importing, composing, describing, versioning, and referencing Zamani dialects.

A dialect is a controlled extension of the Zamani language.

A dialect may describe language facilities for:

- classical computing;
- quantum computing;
- hybrid computing;
- HDL;
- hardware/software co-design;
- embedded computing;
- distributed computing;
- parallel computing;
- HPC;
- AI/ML;
- data computing;
- networking;
- cryptography;
- scientific computing;
- accelerators;
- edge/cloud computing;
- future computing paradigms;
- domain-specific languages;
- organization-specific extensions;
- vendor-specific extensions;
- experimental language facilities.

The dialect grammar is intentionally domain-neutral.

It does not enumerate today's known dialects and must not need modification merely because a new computational domain, architecture, accelerator, quantum technology, hardware technology, or future language extension is introduced.

The fundamental boundary is:

Zamani source
     |
     v
lexing
     |
     v
parsing
     |
     v
AST
     |
     v
semantic analysis
     |
     v
dialect semantic model
     |
     +-------------------+
     |                   |
     v                   v
capability         compatibility
resolution         analysis
     |                   |
     +---------+---------+
               |
               v
canonical semantic representation
               |
       +-------+--------+
       |       |        |
       v       v        v
 classical  quantum   HDL/hardware
    IR       ::ir       representation
       |       |        |
       +-------+--------+
               |
               v
 optimization / routing / scheduling / lowering
               |
               v
 hardware / simulator / runtime / deployment

The grammar is therefore a syntax boundary, not an execution engine.

---

2. Architectural Authority

The dialect grammar participates in the repository's single-language architecture.

The repository already establishes:

grammar/Zamani.g4
        |
        v
canonical ANTLR language representation

and the dialect subsystem provides modular grammar components behind that canonical composition boundary. The repository documentation explicitly requires dialect extensions to have explicit ownership and namespaces and prevents extensions from silently redefining core semantics.

The responsibilities of the dialect files are:

grammar/dialects/dialects.g4
    public dialect parser boundary

grammar/dialects/registration.g4
    dialect declaration and registration syntax

grammar/dialects/namespaces.g4
    dialect namespace syntax

grammar/dialects/versioning.g4
    version declaration syntax

grammar/dialects/capabilities.g4
    capability declaration syntax

grammar/dialects/compatibility.g4
    compatibility declaration syntax

grammar/dialects/vendor.g4
    vendor-extension syntax

grammar/dialects/experimental.g4
    experimental-extension syntax

These files are not independent languages.

They are grammar fragments composing one Zamani language.

No dialect file may create an alternative root language.

---

3. File Ownership

"dialects.g4"

Owns:

- public dialect parser entry point;
- dialect parser boundary;
- delegation into dialect registration;
- stable dialect-reference entry points.

Does not own:

- lexical definitions;
- names;
- qualified-name semantics;
- version resolution;
- capability resolution;
- compatibility algorithms;
- plugin loading;
- hardware discovery;
- target selection;
- quantum IR;
- classical IR;
- HDL IR;
- scheduling;
- routing;
- optimization;
- QEC;
- ZQN;
- resilience;
- runtime execution.

---

"registration.g4"

Owns:

- dialect declarations;
- dialect registration structure;
- imports;
- aliases;
- composition;
- requirements;
- provisions/capabilities;
- extensions;
- syntax descriptors;
- semantic descriptors;
- lowering descriptors;
- compatibility declarations;
- deprecation declarations;
- registration metadata.

It is the primary detailed grammar for dialect declarations.

---

"namespaces.g4"

Owns only namespace-related syntax.

It must not resolve namespaces.

Namespace resolution belongs to semantic/compiler infrastructure.

---

"versioning.g4"

Owns version syntax.

It must not implement:

- version solving;
- compatibility algorithms;
- migrations;
- dependency resolution.

Those belong downstream.

---

"capabilities.g4"

Owns capability declaration syntax.

It does not discover whether a target actually provides a capability.

Capability discovery and resolution belong to semantic/compiler/runtime infrastructure.

---

"compatibility.g4"

Owns source-level compatibility declarations.

It does not implement compatibility algorithms.

---

"vendor.g4"

Owns syntax for explicitly declared vendor extensions.

A vendor extension must remain identifiable as such.

It must not silently become core Zamani syntax.

---

"experimental.g4"

Owns syntax for explicitly marked experimental facilities.

Experimental syntax must not silently become stable language semantics.

---

4. Open-World Dialect Model

Zamani uses an open-world dialect model.

The grammar must not contain a closed list such as:

dialect
    : quantum
    | openqasm
    | qiskit
    | verilog
    | cuda
    | vendor_x
    ;

That architecture would require grammar modification whenever a new dialect is introduced.

Instead, dialect identity is symbolic.

Conceptually:

namespace::dialect

or:

organization::domain::dialect

Examples:

quantum::standard
quantum::openqasm
classical::numeric
hdl::rtl
hardware::fpga
distributed::messaging
ai::tensor
vendor::domain::extension
future::computing::dialect

These names are examples only.

The grammar does not decide whether they exist.

Semantic infrastructure determines whether a referenced dialect is registered and usable.

---

5. Dialects Must Not Become Hardware Selectors

A dialect describes language capabilities.

It must not silently select:

- a CPU;
- a GPU;
- an FPGA;
- an ASIC;
- a QPU;
- a simulator;
- a device identifier;
- a physical qubit;
- a physical topology;
- a network node;
- a memory capacity;
- a machine address;
- a deployment location.

For example:

requires quantum::dynamic_control

means:

«The program requires the semantic capability represented by "quantum::dynamic_control".»

It does not mean:

use QPU X
use device X
use 64 qubits
use topology Y
use GPU Z

Those decisions belong downstream.

---

6. POCO-REAF

Dialect design must preserve:

Program Once
      |
      v
Portable Semantics
      |
      v
Compile Once
      |
      v
Target-Independent Representation
      |
      v
Capability / Resource / Target Resolution
      |
      v
Run Everywhere
      |
      v
Run Anywhere
      |
      v
Run Forever

POCO-REAF does not mean that every target possesses identical capabilities.

It means that the program's semantic meaning does not change merely because the target changes.

The compiler must distinguish:

language validity
    !=
semantic validity
    !=
capability satisfaction
    !=
compilation feasibility
    !=
resource availability
    !=
target compatibility
    !=
runtime availability

A dialect declaration therefore describes portable language semantics and requirements rather than today's machine limitations.

---

7. Absolute Scalability Rule

No dialect grammar file may impose arbitrary finite limits on source-level scalability.

Never encode:

MAX_DIALECTS
MAX_EXTENSIONS
MAX_CAPABILITIES
MAX_REQUIREMENTS
MAX_DEPENDENCIES
MAX_NAMESPACE_DEPTH
MAX_TARGETS
MAX_DEVICES
MAX_NODES
MAX_QUBITS
MAX_CORES
MAX_THREADS
MAX_MEMORY

Likewise, never hide those limits through:

- fixed alternatives;
- finite enumerations;
- fixed arrays;
- fixed register counts;
- fixed device lists;
- fixed topology definitions;
- fixed accelerator counts;
- fixed namespace depth;
- fixed extension counts.

Repetition must be represented through grammar constructs such as:

*
+

or equivalent unbounded structural forms.

Actual implementation limits may exist because of:

- available memory;
- parser resource budgets;
- operating-system constraints;
- compilation budgets;
- execution budgets;
- backend constraints;
- explicit user policies.

Those are resource constraints, not language semantics.

---

8. Dialect Identity Is Not a Physical Identity

A dialect name is a language-level symbolic identity.

It is not:

- a filesystem path;
- a URL;
- an IP address;
- a MAC address;
- a device address;
- a QPU identifier;
- a physical-qubit identifier;
- a hardware serial number;
- a deployment location.

This distinction is essential for POCO-REAF.

The same source program must remain meaningful when executed in different environments.

---

9. Dialect Composition

Dialects may compose other dialects.

Conceptually:

dialect A
    |
    +--> dialect B
    |
    +--> dialect C
    |
    +--> dialect D

The grammar allows the source representation.

Semantic analysis must determine:

- whether dependencies exist;
- whether composition is valid;
- whether composition contains cycles;
- whether declarations conflict;
- whether versions are compatible;
- whether inherited facilities remain valid;
- whether capability requirements are satisfied.

The grammar must not attempt to solve those questions.

---

10. Imports

Dialect imports establish source-level dependencies.

An import does not automatically:

- execute code;
- load a plugin;
- access the network;
- inspect hardware;
- discover capabilities;
- allocate resources;
- select a backend.

Resolution belongs to module/package/compiler infrastructure.

This preserves deterministic parsing.

---

11. Aliases

Aliases provide local source-level names for imported dialects.

An alias is syntactic metadata.

Semantic analysis determines:

- whether the alias is valid;
- whether it conflicts with another symbol;
- what dialect it denotes;
- whether the referenced dialect exists;
- whether the dialect is compatible.

---

12. Capabilities

Dialect capabilities are symbolic.

Examples include conceptual capabilities such as:

quantum::dynamic_control
quantum::measurement
hardware::programmable_logic
distributed::messaging
classical::vector_execution
ai::tensor_computation

The grammar must not assume that any particular target provides them.

Capability resolution occurs after parsing.

The distinction is:

capability declaration
        |
        v
semantic requirement
        |
        v
capability resolution
        |
        v
target realization

---

13. Requirements

Requirements describe semantic prerequisites.

They may express requirements concerning:

- language facilities;
- computational capabilities;
- execution models;
- timing semantics;
- communication semantics;
- memory models;
- quantum capabilities;
- hardware capabilities;
- security properties;
- numerical properties;
- distributed behavior;
- accelerator capabilities.

A requirement must not silently become a physical allocation.

For example:

requires quantum::dynamic_control

does not specify:

qubits = 32
device = xyz
topology = grid

Those are separate concerns.

---

14. Constraints

Constraints describe restrictions on valid realizations.

A constraint is distinct from:

- a requirement;
- a capability;
- a preference;
- a hint.

The semantic layer must preserve these distinctions.

For example:

requirement

means:

«This semantic capability is necessary.»

constraint

means:

«A realization violating this condition is not acceptable.»

preference

means:

«Prefer this realization when possible.»

hint

means:

«This information may guide implementation but does not necessarily affect validity.»

The grammar must not collapse these categories.

---

15. Preferences

Preferences must never become mandatory hardware requirements accidentally.

A preference can guide:

- compilation;
- target selection;
- optimization;
- scheduling;
- placement;
- deployment.

It must not redefine program semantics.

---

16. Hints

Hints are advisory.

They may communicate implementation guidance without becoming permanent semantic requirements.

This distinction is particularly important for POCO-REAF.

A hint should not make a portable program permanently dependent on a temporary hardware characteristic.

---

17. Versioning

Dialect versions and Zamani language versions are distinct.

For example:

Zamani language version
        !=
dialect version

Version syntax belongs in the grammar.

Version interpretation belongs downstream.

The grammar must not implement:

- semantic-version ordering;
- dependency solving;
- compatibility matrices;
- migration algorithms.

Those belong to the versioning/compatibility subsystem.

---

18. Compatibility

Dialect compatibility is a semantic property.

The parser should preserve declarations such as:

- compatible versions;
- required versions;
- incompatible versions;
- replacement declarations;
- deprecation metadata.

The compiler determines whether those declarations are satisfied.

Compatibility failure must produce structured diagnostics.

It must never be silently ignored.

---

19. Deprecation

Dialect facilities may be deprecated.

Deprecation is not removal.

The grammar must preserve enough information for tooling to report:

- deprecated dialects;
- deprecated versions;
- deprecated facilities;
- migration guidance;
- replacement facilities.

Semantic tooling decides the diagnostic severity according to language policy.

---

20. Vendor Extensions

Vendor extensions are permitted only through explicit extension mechanisms.

A vendor extension must not silently modify core Zamani syntax.

Conceptually:

vendor::organization::domain::extension

is a symbolic extension identity.

The grammar must not enumerate vendors.

Adding a vendor must not require modifying the core dialect grammar.

Vendor-specific semantics are resolved by appropriate extension infrastructure.

---

21. Experimental Extensions

Experimental features must be explicitly identifiable.

The grammar must not silently promote experimental syntax to stable language syntax.

Experimental status must remain available to:

- diagnostics;
- tooling;
- compatibility checking;
- language-version policy;
- migration tooling.

An experimental facility must never be assumed to have permanent semantic stability.

---

22. Quantum Boundary

Quantum dialects are supported by the dialect system.

However, the dialect grammar must never define a second quantum IR.

The ownership boundary is:

Zamani quantum syntax
        |
        v
frontend AST
        |
        v
semantic quantum model
        |
        v
quantum::ir
        |
        v
optimization
        |
        v
routing
        |
        v
scheduling
        |
        v
ZQN / QEC / resilience
        |
        v
hardware HAL / runtime

The dialect grammar must not own:

QubitId
PhysicalQubitId
QuantumGate
QuantumOperation
QuantumCircuit
Topology
Calibration
Pulse
Schedule
NoiseModel
QEC implementation

"quantum::ir" remains the canonical quantum semantic boundary.

---

23. Classical Boundary

Classical dialects may describe language facilities for:

- scalar computation;
- numerical computing;
- vector computation;
- matrix computation;
- tensor computation;
- symbolic computation;
- concurrency;
- parallelism;
- systems programming;
- accelerators.

The dialect grammar does not implement classical semantics.

Those semantics belong to the appropriate type, expression, statement, classical, IR, compiler, and runtime layers.

---

24. HDL Boundary

HDL dialects may describe language extensions for:

- RTL;
- hardware modules;
- ports;
- signals;
- wires;
- registers;
- clocks;
- timing;
- combinational logic;
- sequential logic;
- state machines;
- memories;
- pipelines;
- hardware interfaces.

The dialect layer does not implement HDL semantics.

It only provides the extension mechanism through which those semantics can be named and composed.

---

25. Hardware Boundary

Dialect declarations must not directly represent:

- physical devices;
- device IDs;
- board IDs;
- memory addresses;
- physical qubit IDs;
- fixed topology;
- fixed accelerator counts;
- deployment coordinates.

Hardware capabilities and resources belong to the hardware abstraction and resource systems.

The direction is:

dialect requirement
        |
        v
capability/resource analysis
        |
        v
hardware capability model
        |
        v
target realization

---

26. Distributed Computing Boundary

Distributed dialects may identify language facilities involving:

- communication;
- messaging;
- services;
- replication;
- consistency;
- distributed execution;
- fault tolerance.

The dialect grammar does not define:

- number of nodes;
- fixed cluster topology;
- fixed addresses;
- physical network layout.

Those belong to deployment/resource/runtime infrastructure.

---

27. AI/Data Boundary

AI and data dialects may identify language extensions involving:

- tensors;
- models;
- datasets;
- training;
- inference;
- differentiation;
- pipelines;
- agents;
- data streams.

The grammar must not hard-code:

- tensor dimensions;
- accelerator counts;
- memory capacities;
- model sizes;
- cluster sizes.

Those properties belong to semantic/resource/target analysis.

---

28. Determinism

Parsing must be deterministic.

Dialect parsing must never perform:

- filesystem lookup;
- network access;
- plugin discovery;
- hardware discovery;
- runtime execution;
- capability probing;
- target selection.

Given the same:

source
+
language version
+
grammar version

the parser must produce the same syntactic result.

External resolution belongs after parsing.

---

29. Security Boundary

Grammar parsing must be treated as untrusted-input processing.

The grammar must not:

- execute arbitrary code;
- invoke shell commands;
- access files;
- access networks;
- inspect devices;
- load arbitrary plugins;
- execute dialect implementations.

All such operations belong to explicitly controlled downstream infrastructure.

Generated Rust must remain safe Rust.

No dialect grammar may require:

unsafe
unsafe fn
unsafe impl
unsafe {
}

---

30. AST Contract

Parsing must preserve sufficient information for the AST to represent:

- dialect identity;
- namespace;
- source ordering;
- imports;
- aliases;
- composition;
- requirements;
- capabilities;
- constraints;
- preferences;
- hints;
- extensions;
- syntax descriptors;
- semantic descriptors;
- lowering descriptors;
- versions;
- compatibility;
- deprecation;
- vendor status;
- experimental status;
- metadata;
- source spans.

The parser must not construct target-specific objects.

---

31. Semantic Contract

Semantic analysis owns:

- dialect lookup;
- registration validation;
- duplicate detection;
- alias resolution;
- dependency resolution;
- composition validation;
- cycle detection;
- capability resolution;
- requirement checking;
- constraint checking;
- version compatibility;
- extension conflict detection;
- vendor policy;
- experimental policy;
- deprecation policy;
- lowering availability;
- semantic validity.

The parser only establishes syntactic structure.

---

32. IR Contract

Dialect grammar produces no IR.

The correct direction is:

source
  ↓
lexer
  ↓
parser
  ↓
AST
  ↓
semantic dialect model
  ↓
canonical semantic representation
  ↓
domain-specific IR

Possible downstream representations include:

classical IR
quantum::ir
HDL/hardware IR
data representation
control-flow representation
distributed representation
other future representations

The dialect grammar must never bypass the semantic layer.

---

33. Compiler Integration

The compiler must integrate dialects in this order:

parse
  ↓
AST construction
  ↓
name resolution
  ↓
module/dialect resolution
  ↓
type analysis
  ↓
effect analysis
  ↓
capability analysis
  ↓
requirement/constraint analysis
  ↓
dialect compatibility analysis
  ↓
canonical semantic representation
  ↓
IR lowering

The grammar must not directly call any compiler subsystem.

---

34. Runtime Integration

Runtime components must not depend directly on grammar files.

Runtime execution should consume:

- compiled representations;
- canonical IR;
- execution plans;
- target descriptions;
- capability results;
- resource decisions.

The runtime does not parse dialect grammar as part of ordinary execution.

This prevents the architecture:

runtime -> grammar

from becoming a hidden dependency.

---

35. Scheduling Integration

Dialect syntax may express semantic timing or execution requirements.

It must not implement scheduling algorithms.

Scheduling consumes downstream representations.

The dependency must remain:

dialect semantics
        ↓
canonical representation
        ↓
scheduling

not:

dialect grammar
        ↓
scheduler

---

36. Routing Integration

Dialect syntax may express requirements related to placement or communication.

It must not perform routing.

Routing consumes canonical semantic representations and target topology information.

---

37. Optimization Integration

Dialect declarations may identify semantic properties relevant to optimization.

They must not implement optimization algorithms.

Optimization must preserve the semantic meaning established before lowering.

---

38. QEC / ZQN / Resilience Integration

Dialect syntax may describe high-level intent involving:

- error correction;
- fault tolerance;
- noise awareness;
- resilience;
- mitigation.

The grammar must not implement these systems.

Ownership remains:

QEC
    error detection/correction

ZQN
    quantum noise/fault semantics

resilience
    adaptation/recovery decisions

The dialect layer merely provides the source-language extension mechanism.

---

39. Hardware HAL Integration

The hardware HAL owns:

- hardware capabilities;
- target properties;
- device state;
- calibration;
- physical resources;
- backend-specific realization.

Dialect grammar owns none of these.

The compiler may translate dialect requirements into queries against the HAL through semantic infrastructure.

---

40. Resource Integration

Resource requirements must remain symbolic where possible.

The dialect grammar must not hard-code:

qubits = 32
cores = 16
threads = 64
nodes = 8
memory = 1TB

as permanent language-level assumptions.

If source semantics genuinely require a quantity, the quantity should be represented as a source expression or requirement and evaluated by semantic/resource analysis.

---

41. Tooling Integration

Dialect information must remain available to:

- syntax highlighting;
- formatting;
- IDE tooling;
- language servers;
- diagnostics;
- documentation generation;
- completion;
- navigation;
- compatibility tools;
- migration tools;
- static analysis.

Tooling must consume structured AST/semantic information rather than reparsing dialect declarations independently.

---

42. Documentation Integration

Documentation must distinguish:

PROPOSED
DESIGNED
GRAMMAR_DEFINED
LEXER_IMPLEMENTED
PARSER_IMPLEMENTED
AST_IMPLEMENTED
SEMANTIC_IMPLEMENTED
IR_IMPLEMENTED
BACKEND_IMPLEMENTED
TESTED
STABLE
DEPRECATED

A dialect appearing in documentation does not imply that the compiler implements it.

A documented dialect must not be presented as executable unless its downstream pipeline exists.

---

43. Dependency Direction

The dialect grammar dependency direction is:

lexer
  ↓
names
  ↓
dialect grammar
  ↓
AST
  ↓
semantic analysis
  ↓
capability / requirement / compatibility analysis
  ↓
canonical semantic model
  ↓
IR
  ↓
optimization / routing / scheduling
  ↓
hardware / runtime

The following reverse dependencies are prohibited:

dialect grammar -> runtime
dialect grammar -> hardware
dialect grammar -> scheduler
dialect grammar -> router
dialect grammar -> optimizer
dialect grammar -> quantum::ir
dialect grammar -> QEC
dialect grammar -> ZQN
dialect grammar -> resilience

---

44. No Circular Architecture

The architecture must never become:

grammar
  ↓
IR
  ↓
grammar

or:

grammar
  ↓
runtime
  ↓
grammar

or:

quantum grammar
  ↓
hardware grammar
  ↓
quantum grammar

Grammar defines syntax.

Semantic infrastructure interprets it.

IR represents semantics.

Backends realize it.

---

45. Grammar Composition

Individual grammar fragments must not independently become complete parsers.

The canonical composition root is responsible for assembling the language.

Conceptually:

Zamani.g4
    |
    +-- lexer
    +-- core
    +-- types
    +-- expressions
    +-- statements
    +-- declarations
    +-- functions
    +-- modules
    +-- effects
    +-- memory
    +-- concurrency
    +-- classical
    +-- quantum
    +-- hybrid
    +-- HDL
    +-- hardware
    +-- distributed
    +-- AI
    +-- data
    +-- networking
    +-- security
    +-- resources
    +-- compile
    +-- execution
    +-- interoperability
    +-- dialects
    +-- macros
    +-- metaprogramming

The repository explicitly requires individual grammar components not to independently construct alternative complete parsers.

---

46. Relationship Between "dialects.g4" and "registration.g4"

"dialects.g4" is the public boundary.

"registration.g4" owns detailed registration syntax.

Therefore:

dialects.g4
      |
      v
dialectRegistration
      |
      v
registration.g4

"dialects.g4" must not duplicate rules owned by "registration.g4".

This allows detailed registration syntax to evolve without forcing every consumer to understand its internal structure.

---

47. Required Integration Files

The dialect system must be coordinated with:

grammar/Zamani.g4

grammar/core/names.g4
grammar/core/qualified-names.g4
grammar/core/versioning.g4
grammar/core/capabilities.g4
grammar/core/requirements.g4
grammar/core/constraints.g4
grammar/core/hints.g4

grammar/dialects/dialects.g4
grammar/dialects/registration.g4
grammar/dialects/namespaces.g4
grammar/dialects/versioning.g4
grammar/dialects/capabilities.g4
grammar/dialects/compatibility.g4
grammar/dialects/vendor.g4
grammar/dialects/experimental.g4

src/lexer.rs
src/parser.rs
src/ast/
src/semantic.rs
src/ir_gen.rs
src/ir_verify.rs

src/quantum/ir/

The exact implementation dependency must be determined by the repository's canonical parser assembly rather than by copying rules between files.

---

48. Cross-Domain Contract

A dialect mechanism must support composition such as:

classical + quantum
classical + HDL
quantum + HDL
quantum + hardware
quantum + distributed
AI + quantum
AI + hardware
classical + quantum + distributed
classical + quantum + HDL + hardware

The dialect grammar must not need domain-specific modifications for each combination.

That is a central requirement for universal computing.

---

49. Future-Domain Contract

A future computational domain must be introducible without modifying:

dialects.g4

when the new domain can be represented through the existing open dialect model.

For example, adding a future:

future::photonic

or:

future::neuromorphic

must not require adding another alternative to the core grammar.

The semantic subsystem may subsequently add the domain implementation.

---

50. Hard-Coding Audit

Every change to this directory must be audited for:

Language hard-coding

Does the grammar enumerate today's technologies?

Hardware hard-coding

Does it encode a device, architecture, topology, or capacity?

Quantum hard-coding

Does it encode a finite qubit count or hardware gate set?

Deployment hard-coding

Does it encode a fixed node or cluster count?

Namespace hard-coding

Does it impose a finite namespace depth?

Capability hard-coding

Does it enumerate all capabilities instead of supporting symbolic extension?

Vendor hard-coding

Does adding a vendor require modifying the core grammar?

Version hard-coding

Does the grammar implement today's compatibility algorithm?

Resource hard-coding

Does it turn a target resource into permanent language semantics?

Any accidental hard-coding must be removed.

---

51. Hard-Coding Classification

Every discovered limitation must be classified as exactly one of:

1. genuine language semantic requirement;
2. target-specific requirement;
3. resource constraint;
4. implementation limitation;
5. accidental hard-coding;
6. test-only limitation;
7. documentation-only limitation.

Only accidental hard-coding is automatically forbidden.

Target/resource limitations must remain outside permanent source semantics.

---

52. Diagnostics

Dialect-related diagnostics must eventually distinguish at least:

unknown dialect
duplicate dialect
duplicate alias
invalid namespace
invalid version
incompatible version
missing dependency
composition cycle
extension conflict
unsatisfied requirement
violated constraint
missing capability
unsupported experimental feature
deprecated dialect
invalid vendor extension
invalid dialect syntax

The grammar itself only detects syntactic failures.

Semantic errors belong to semantic diagnostics.

---

53. Error Recovery

Parser recovery must:

1. identify the unexpected token;
2. preserve source span information;
3. report a structured diagnostic;
4. consume input to make progress;
5. synchronize at a valid grammar boundary;
6. avoid infinite loops;
7. avoid manufacturing executable semantics from invalid syntax.

Malformed dialect declarations must never silently become valid declarations.

---

54. Deep-Program Scalability

The implementation must not assume shallow source programs.

For very deeply nested dialect structures, compiler infrastructure should prefer:

- iterative processing;
- explicit worklists;
- explicit stacks;
- incremental processing;
- configurable resource budgets;
- bounded diagnostics where policy requires it.

The language must not introduce arbitrary nesting limits merely for implementation convenience.

---

55. Determinism Tests

The dialect parser must pass tests establishing that identical source input produces identical parsing results.

Tests must cover:

- identical source;
- repeated dialect declarations;
- nested composition;
- large dependency lists;
- large capability lists;
- large requirement lists;
- long qualified names;
- vendor extensions;
- experimental extensions;
- version declarations.

No test may rely on nondeterministic discovery.

---

56. Positive Tests

Positive tests must include:

single dialect
dialect with namespace
dialect with version
dialect with import
dialect with alias
dialect composition
multiple compositions
requirements
capabilities
constraints
preferences
hints
extensions
vendor extension
experimental extension
compatibility declaration
deprecation declaration
nested qualified names
future symbolic dialect names

---

57. Negative Tests

Negative tests must include:

missing dialect name
invalid qualified name
missing braces
unclosed dialect
duplicate declarations
invalid alias
invalid version syntax
invalid compatibility declaration
invalid requirement
invalid capability
invalid extension
malformed vendor declaration
malformed experimental declaration
illegal syntax combination

Semantic-invalid cases must be tested separately from syntax-invalid cases.

---

58. Boundary Tests

Boundary tests must include:

- one dialect;
- many dialects;
- one capability;
- many capabilities;
- one requirement;
- many requirements;
- deep namespaces;
- deep composition;
- large extension sets;
- large source files;
- large cross-domain programs.

The tests must not define arbitrary maximums as language requirements.

If implementation limits are tested, they must be explicitly identified as implementation/resource limits.

---

59. Scalability Tests

Scalability testing must demonstrate that source syntax does not impose artificial limits on:

dialects
extensions
capabilities
requirements
dependencies
namespace depth
program size
qubits
cores
threads
devices
nodes
memory
accelerators

The grammar must remain independent of the physical scale.

---

60. Cross-Domain Tests

At minimum:

classical + quantum
classical + HDL
quantum + HDL
quantum + hardware
quantum + distributed
AI + quantum
AI + hardware
classical + quantum + distributed
classical + quantum + HDL + hardware

These tests verify that dialect composition remains domain-neutral.

---

61. Round-Trip Tests

Where a canonical Zamani printer/serializer exists:

source
  ↓
lexer
  ↓
parser
  ↓
AST
  ↓
printer
  ↓
parser
  ↓
AST

must preserve the intended dialect semantics.

The test must compare semantic structure rather than relying only on textual equality.

---

62. Compatibility Tests

Compatibility testing must cover:

- supported dialect versions;
- incompatible versions;
- deprecated versions;
- missing versions;
- multiple dependencies;
- composed dialects;
- vendor extensions;
- experimental extensions;
- language-version interactions.

Compatibility algorithms remain outside grammar files.

---

63. ANTLR Requirements

All grammar files must remain valid ANTLR grammar components.

They must contain no embedded Rust actions.

Forbidden:

@members {
    ...
}

when such actions introduce implementation coupling.

Also forbidden are grammar-level:

- filesystem operations;
- network operations;
- runtime calls;
- hardware callbacks;
- compiler callbacks;
- unsafe code.

Grammar fragments must remain declarative.

---

64. Rust Requirements

The generated/reference compiler infrastructure must support:

Rust 1.97
Rust 1.97.1
Rust 2021

and must use safe Rust.

The dialect grammar must not require "unsafe".

No dialect feature may require an unsafe runtime representation.

---

65. Ownership Rule

Every concept must have exactly one authoritative owner.

For example:

qualified-name syntax
    -> core/names / qualified-names

dialect registration
    -> dialects/registration.g4

dialect version syntax
    -> dialects/versioning.g4

capability declaration
    -> dialects/capabilities.g4

compatibility declaration
    -> dialects/compatibility.g4

quantum semantic representation
    -> quantum::ir

quantum noise
    -> ZQN

error correction
    -> QEC

routing
    -> routing

scheduling
    -> scheduling

hardware capabilities
    -> hardware HAL

runtime execution
    -> runtime

Duplication is prohibited unless one representation is explicitly generated from another.

---

66. What "dialects/" Must Never Become

The directory must never become:

- a plugin runtime;
- a hardware registry;
- a device registry;
- a package manager;
- a scheduler;
- a router;
- an optimizer;
- a quantum compiler;
- a QEC implementation;
- a noise model;
- a runtime;
- a backend;
- a simulator;
- a second IR;
- a second parser.

It is a language-extension grammar boundary.

---

67. Completion Contract for "dialects.g4"

"dialects.g4" is complete only when:

- its public dialect entry point is stable;
- registration syntax is delegated to its owner;
- name rules are delegated to their owner;
- no dialect semantics are duplicated;
- no hardware assumptions exist;
- no finite dialect limit exists;
- no finite capability limit exists;
- no finite namespace limit exists;
- no vendor enumeration exists;
- no quantum hardware enumeration exists;
- no embedded Rust exists;
- generated parser generation succeeds;
- root parser integration succeeds;
- AST integration succeeds;
- semantic integration succeeds;
- positive tests pass;
- negative tests pass;
- boundary tests pass;
- deterministic tests pass;
- cross-domain tests pass;
- scalability tests pass.

---

68. Completion Contract for "registration.g4"

"registration.g4" is complete only when:

- dialect declarations are structurally complete;
- imports are represented;
- aliases are represented;
- composition is represented;
- requirements are represented;
- capabilities are represented;
- extensions are represented;
- syntax descriptors are represented;
- semantic descriptors are represented;
- lowering descriptors are represented;
- compatibility metadata is represented;
- deprecation metadata is represented;
- metadata is preserved;
- semantic interpretation remains downstream;
- no hardware limits exist;
- no domain enumeration is required;
- parser generation succeeds;
- AST lowering is defined;
- semantic consumers are identified;
- all associated tests pass.

---

69. Completion Contract for the Whole Directory

The directory is production-ready only when all of the following are true:

single canonical language
        +
modular grammar ownership
        +
open-world dialect model
        +
deterministic parsing
        +
safe Rust integration
        +
complete diagnostics
        +
AST preservation
        +
semantic resolution
        +
version compatibility
        +
capability resolution
        +
cross-domain composition
        +
POCO-REAF independence
        +
no accidental hard-coding
        +
quantum IR boundary preservation
        +
repository integration
        +
conformance tests

---

70. Implementation Order

The dialect subsystem must be implemented dependency-first.

Recommended order:

1. core/names.g4
       ↓
2. core/qualified-names.g4
       ↓
3. dialects/namespaces.g4
       ↓
4. dialects/versioning.g4
       ↓
5. dialects/capabilities.g4
       ↓
6. dialects/compatibility.g4
       ↓
7. dialects/vendor.g4
       ↓
8. dialects/experimental.g4
       ↓
9. dialects/registration.g4
       ↓
10. dialects/dialects.g4
       ↓
11. root Zamani.g4 integration
       ↓
12. lexer/parser conformance
       ↓
13. AST integration
       ↓
14. semantic integration
       ↓
15. capability/requirement resolution
       ↓
16. compatibility validation
       ↓
17. cross-domain tests
       ↓
18. scalability/determinism tests

No later file should force a completed earlier file to be redesigned merely because its ownership contract was not established.

---

71. Independence Requirement

Before implementing each dialect file, its complete contract must be known:

Purpose
Ownership
Non-ownership
Inputs
Outputs
Dependencies
Upstream contracts
Downstream consumers
Public grammar contract
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
Compatibility tests
Scalability tests
Determinism tests
Hard-coding audit
Completion criteria

A file is not considered complete until all of these are resolved.

---

72. Final Architecture

The final architecture must remain:

                    Zamani Source
                          |
                          v
                        Lexer
                          |
                          v
                        Parser
                          |
                          v
                   Dialect Syntax
                          |
                          v
                         AST
                          |
                          v
                  Semantic Analysis
                          |
            +-------------+-------------+
            |             |             |
            v             v             v
       requirements   capabilities   compatibility
            |             |             |
            +-------------+-------------+
                          |
                          v
             Canonical Semantic Model
                          |
          +---------------+----------------+
          |               |                |
          v               v                v
     Classical IR    quantum::ir     HDL/Hardware IR
          |               |                |
          +---------------+----------------+
                          |
                          v
             Optimization / Lowering
                          |
                +---------+---------+
                |                   |
                v                   v
             Routing             Scheduling
                |                   |
                +---------+---------+
                          |
                          v
                Hardware / Runtime

The dialect grammar remains above all target-specific realization.

---

73. Fundamental Invariant

The following invariant must hold permanently:

«A Zamani dialect describes language extension semantics and intent; it does not describe an arbitrary limitation of the machine currently available.»

Therefore:

dialect
    ≠ device

dialect
    ≠ backend

dialect
    ≠ target

dialect
    ≠ topology

dialect
    ≠ resource allocation

dialect
    ≠ scheduler

dialect
    ≠ router

dialect
    ≠ optimizer

dialect
    ≠ quantum IR

dialect
    ≠ runtime

---

74. POCO-REAF Invariant

The ultimate invariant is:

ONE SOURCE PROGRAM
       |
       v
ONE STABLE SEMANTIC MEANING
       |
       +----> tiny machine
       +----> CPU
       +----> multicore
       +----> GPU
       +----> FPGA
       +----> ASIC
       +----> QPU
       +----> simulator
       +----> accelerator
       +----> cluster
       +----> supercomputer
       +----> distributed system
       +----> cloud
       +----> future architecture

The target determines how the computation is realized.

The source determines what the computation means.

---

75. Final Production-Readiness Checklist

Before declaring "grammar/dialects/" production-ready:

- [ ] One canonical Zamani language is maintained.
- [ ] Dialect grammar fragments are not independent languages.
- [ ] "dialects.g4" remains the public dialect boundary.
- [ ] "registration.g4" owns registration syntax.
- [ ] Namespace ownership is explicit.
- [ ] Version ownership is explicit.
- [ ] Capability ownership is explicit.
- [ ] Compatibility ownership is explicit.
- [ ] Vendor ownership is explicit.
- [ ] Experimental ownership is explicit.
- [ ] No dialect enumeration exists.
- [ ] No vendor enumeration exists.
- [ ] No machine enumeration exists.
- [ ] No device identifiers are encoded.
- [ ] No topology is encoded.
- [ ] No physical resource limits are encoded.
- [ ] No maximum dialect count exists.
- [ ] No maximum extension count exists.
- [ ] No maximum capability count exists.
- [ ] No maximum namespace depth exists.
- [ ] No maximum requirement count exists.
- [ ] No maximum dependency count exists.
- [ ] No maximum qubit count exists.
- [ ] No maximum CPU/core/thread count exists.
- [ ] No fixed accelerator count exists.
- [ ] No embedded Rust actions exist.
- [ ] No filesystem access exists.
- [ ] No network access exists.
- [ ] No hardware discovery exists.
- [ ] No runtime execution exists.
- [ ] AST integration is complete.
- [ ] Semantic integration is complete.
- [ ] Capability resolution integration is complete.
- [ ] Compatibility integration is complete.
- [ ] Canonical IR integration is defined.
- [ ] "quantum::ir" remains authoritative for quantum semantics.
- [ ] QEC remains owned by QEC infrastructure.
- [ ] ZQN remains owned by ZQN infrastructure.
- [ ] Scheduling remains owned by scheduling infrastructure.
- [ ] Routing remains owned by routing infrastructure.
- [ ] Hardware remains owned by hardware infrastructure.
- [ ] Runtime remains downstream.
- [ ] Deterministic parsing is verified.
- [ ] Positive tests pass.
- [ ] Negative tests pass.
- [ ] Boundary tests pass.
- [ ] Cross-domain tests pass.
- [ ] Scalability tests pass.
- [ ] Round-trip tests pass where supported.
- [ ] Compatibility tests pass.
- [ ] Hard-coding audit passes.
- [ ] Documentation matches implementation status.
- [ ] Rust 1.97/1.97.1 compatibility is verified.
- [ ] No "unsafe" Rust is required.
- [ ] Root "Zamani.g4" integration is verified.
- [ ] No circular grammar/IR/runtime dependency exists.

---

76. Final Principle

Zamani's dialect architecture exists to make the language extensible without making it fragile.

A new computational paradigm should be able to enter Zamani through an explicit dialect without requiring the core language to become a closed enumeration of technologies.

The permanent architecture is:

                     ZAMANI
                       |
              stable core semantics
                       |
                 dialect system
                       |
       +---------------+----------------+
       |               |                |
   classical        quantum            HDL
       |               |                |
       +---------------+----------------+
                       |
                hybrid semantics
                       |
             canonical semantic model
                       |
        +--------------+---------------+
        |              |               |
      IRs          capabilities      resources
        |              |               |
        +--------------+---------------+
                       |
              target realization
                       |
       +-------+-------+-------+--------+
       |       |       |       |        |
      CPU     GPU     FPGA    QPU    Future

Zamani describes computation, intent, capabilities, constraints, and semantics—not arbitrary limitations of the machine currently available.

That is the dialect-level foundation required for:

Program Once → Compile Once → Run Everywhere → Anywhere → Forever.

And it preserves the larger Zamani objective:

From Atom to Everywhere.
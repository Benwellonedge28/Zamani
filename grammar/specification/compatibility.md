Worked for 11s

I inspected the current grammar/ repository surface and the existing normative design/versioning material. The repository currently has grammar/DESIGN.md, grammar/README.md, Zamani.g4, Zamani-Grammar.md, grammar.md, and grammar/specification/language-version.md; compatibility.md is not currently present, so the file below is designed as the normative compatibility contract that integrates with those existing authorities.

# Zamani Compatibility Specification

**Path:** `grammar/specification/compatibility.md`

**Status:** Normative  
**Scope:** Zamani language compatibility across source, grammar, AST, semantics, IR, dialects, compilers, runtimes, targets, artifacts, tools, and future platforms  
**Implementation baseline:** Rust 1.97 / Rust 1.97.1  
**Rust safety requirement:** `unsafe` Rust MUST NOT be used  
**Primary objective:** Semantic stability, deterministic evolution, interoperability, scalability, and POCO-REAF  
**POCO-REAF:** Program Once, Compile Once, Run Everywhere, Anywhere, Forever

---

## 1. Purpose

This document defines the compatibility contract for the Zamani programming language and its surrounding compilation ecosystem.

It answers one central question:

> When can a Zamani program, module, artifact, dialect, compiler, IR, runtime, or target be considered compatible with another version or implementation?

Compatibility in Zamani MUST be defined in terms of **semantic contracts**, not accidental implementation details.

Zamani is intended to scale:

```text
atom
  ↓
embedded system
  ↓
single processor
  ↓
multicore processor
  ↓
accelerator
  ↓
GPU
  ↓
FPGA
  ↓
ASIC
  ↓
quantum processor
  ↓
hybrid quantum/classical system
  ↓
distributed system
  ↓
cluster
  ↓
supercomputer
  ↓
cloud
  ↓
heterogeneous computing fabric
  ↓
future computing architectures
  ↓
everywhere

The language MUST NOT acquire compatibility rules that impose artificial ceilings on:

qubits;

logical qubits;

physical qubits;

CPUs;

cores;

threads;

GPUs;

accelerators;

FPGAs;

ASIC resources;

memory;

registers;

vector lanes;

tensor dimensions;

nodes;

devices;

network participants;

services;

quantum registers;

hardware modules;

program size;

module count;

package count;

data size;

concurrency;

parallelism;

distributed resources;

or other scalable quantities.


Actual limits belong to:

available resources;

target capabilities;

implementation limits;

deployment policy;

compilation budgets;

runtime budgets;

explicitly requested program constraints;

physical laws;

representation limits.


They MUST NOT be accidentally encoded into the language compatibility model.


---

2. Normative terminology

The following terms are normative:

MUST

MUST NOT

REQUIRED

SHALL

SHALL NOT

SHOULD

SHOULD NOT

RECOMMENDED

MAY

OPTIONAL


Unless explicitly stated otherwise, these words describe mandatory language and repository behavior.


---

3. Compatibility principle

The fundamental compatibility principle is:

> A compatible implementation preserves the meaning of valid Zamani programs within the declared compatibility contract.



Compatibility therefore has two independent dimensions:

Representation compatibility
        +
Semantic compatibility
        =
Language compatibility

Two representations MAY differ while remaining semantically compatible.

Two representations MAY look syntactically similar while being semantically incompatible.

Therefore:

same syntax
≠ necessarily same semantics

different representation
≠ necessarily incompatible


---

4. Compatibility is multidimensional

Zamani MUST NOT use a single boolean notion of compatibility internally.

At minimum, compatibility MUST distinguish:

1. lexical compatibility;


2. grammar compatibility;


3. source compatibility;


4. AST compatibility;


5. type compatibility;


6. semantic compatibility;


7. effect compatibility;


8. capability compatibility;


9. resource compatibility;


10. module compatibility;


11. package compatibility;


12. dialect compatibility;


13. IR compatibility;


14. artifact compatibility;


15. ABI compatibility;


16. runtime compatibility;


17. target compatibility;


18. execution compatibility;


19. serialization compatibility;


20. tooling compatibility;


21. diagnostic compatibility;


22. interoperability compatibility.



An implementation MUST identify the relevant dimension when reporting compatibility.

A generic error such as:

incompatible

is insufficient for production diagnostics when the system can identify the actual incompatibility.

Prefer diagnostics conceptually equivalent to:

language version incompatible
dialect version incompatible
IR schema incompatible
target capability unavailable
ABI incompatible
runtime contract incompatible
source syntax removed
semantic contract incompatible


---

5. Relationship to language versioning

The language-version contract is defined by:

grammar/specification/language-version.md

That document owns the language version model.

This document owns the compatibility interpretation of those versions.

The distinction is:

language-version.md
    =
how Zamani versions itself

compatibility.md
    =
what those versions may and may not interoperate with

Neither document MUST duplicate the other's authority.


---

6. Compatibility authority

The repository MUST maintain one coherent authority chain.

The intended relationship is:

language-version.md
        │
        ▼
compatibility.md
        │
        ├── grammar-authority.md
        ├── syntax-model.md
        ├── semantic-model.md
        ├── compilation-model.md
        ├── execution-model.md
        └── scalability-model.md
                 │
                 ▼
        concrete grammar
                 │
        ┌────────┴────────┐
        ▼                 ▼
      lexer             parser
        │                 │
        └────────┬────────┘
                 ▼
                AST
                 │
                 ▼
       semantic/type/effect/
       capability analysis
                 │
                 ▼
          canonical IR
                 │
        ┌────────┼────────┐
        ▼        ▼        ▼
    classical quantum  hardware
       IR        IR        IR
                 │
                 ▼
        optimization/
        scheduling/
        lowering
                 │
                 ▼
        runtime / target

No downstream component may silently redefine an upstream language contract.


---

7. Existing repository integration

The compatibility model MUST integrate with the repository's existing architecture.

The current grammar surface includes:

grammar/DESIGN.md
grammar/README.md
grammar/Zamani.g4
grammar/Zamani-Grammar.md
grammar/grammar.md
grammar/specification/
grammar/antlr/
grammar/reference/
grammar/spec/
grammar/tests/

The compatibility system MUST account for these artifacts rather than create a parallel language architecture.

The current production design establishes the important boundary that grammar describes source-level computation and intent, while target-specific facts belong to later compilation and execution stages.

That separation is mandatory for compatibility.


---

8. Ownership

8.1 This file owns

compatibility.md owns:

compatibility categories;

compatibility guarantees;

compatibility levels;

compatibility matrices;

compatibility decision rules;

version-interoperation rules;

migration compatibility;

source compatibility policy;

artifact compatibility policy;

dialect compatibility policy;

target compatibility interpretation;

compatibility failure classification;

forward/backward compatibility rules;

transitive compatibility rules;

compatibility testing requirements.


8.2 This file does not own

This file does NOT own:

concrete lexical tokens;

parser productions;

AST implementation;

semantic type checking implementation;

canonical quantum IR structure;

QEC implementation;

ZQN implementation;

scheduler implementation;

hardware calibration;

backend-specific gate sets;

runtime implementation;

compiler optimization algorithms;

package-manager dependency resolution implementation.


Those remain owned by their respective subsystems.


---

9. Required separation of version dimensions

Zamani MUST distinguish at least:

Language Version
Grammar Revision
Lexer Contract
AST Contract
Semantic Model Version
Classical IR Version
Quantum IR Version
Hardware IR Version
Dialect Versions
Artifact Format Version
ABI Version
Compiler Version
Runtime Version
Target Descriptor Version
Backend Version
Toolchain Version

These versions MUST NOT be conflated.

For example:

compiler version changed

does NOT automatically mean:

language version changed

Likewise:

new GPU backend

does NOT automatically mean:

Zamani language changed

And:

quantum::ir schema changed

does NOT automatically mean:

Zamani source syntax changed

Each compatibility boundary MUST be evaluated independently.


---

10. Compatibility levels

Zamani implementations SHOULD classify compatibility using the following levels.

Level	Meaning

C0	No compatibility
C1	Lexically compatible
C2	Syntactically/source compatible
C3	AST compatible
C4	Semantically compatible
C5	IR compatible
C6	Artifact compatible
C7	Runtime compatible
C8	Target executable/realization compatible
C9	Fully compatible under the declared contract


A higher level implies satisfaction of the required lower-level contracts unless explicitly documented otherwise.

Example:

C4 semantic compatibility

does not necessarily imply:

C8 native execution compatibility

A program can have identical semantics while requiring a target capability unavailable on a particular machine.


---

11. Source compatibility

Source compatibility means that a Zamani source program accepted by one language version remains valid and semantically equivalent under another compatible language version.

Source compatibility has three primary forms:

11.1 Backward source compatibility

New implementation accepts source written for an older compatible version.

old source
    ↓
new compiler

11.2 Forward source compatibility

An older implementation understands a newer source version.

This is only possible when the newer source uses constructs explicitly guaranteed to be understood by the older implementation.

The language MUST NOT assume universal forward compatibility.

11.3 Cross-implementation compatibility

Two independent implementations accept the same source and produce equivalent semantics.

This requires conformance to the same language specification and compatibility profile.


---

12. Source compatibility invariant

For a compatible change:

Program P
+
Language Version V

and:

Program P
+
Compatible Language Version V'

MUST preserve the intended semantics of P.

If semantics cannot be preserved, the change MUST be classified as breaking.


---

13. Breaking changes

A change is breaking if it can cause a previously valid program to:

fail to parse;

fail semantic analysis;

change type meaning;

change effect meaning;

change resource meaning;

change execution meaning;

change observable behavior;

change ownership behavior;

change concurrency behavior;

change synchronization guarantees;

change quantum behavior;

change measurement behavior;

change hardware behavior;

change distributed consistency;

change security guarantees;

or produce a materially different result.


Breaking changes MUST NOT be hidden inside patch releases.


---

14. Non-breaking changes

A change MAY be non-breaking if it:

adds an unambiguous construct;

adds a new type without changing existing types;

adds a new dialect;

adds a backend;

adds a target;

adds an optimization that preserves semantics;

improves diagnostics;

fixes an implementation bug without changing specified behavior;

adds a capability without changing existing capability meanings;

adds a resource expression without changing existing requirements;

improves scalability;

improves parser performance;

improves compiler performance;

adds target lowering;

adds serialization support while preserving existing serialized meaning.


The change MUST still undergo compatibility validation.


---

15. Grammar compatibility

grammar/Zamani.g4 is a concrete grammar representation.

It MUST conform to the normative language specification.

A grammar implementation MUST NOT introduce a new semantic language merely because its parser can recognize additional syntax.

A grammar change MUST be classified as one of:

Equivalent
Compatible Extension
Deprecated
Breaking
Experimental
Reserved
Implementation-only


---

16. Grammar representation versus language semantics

Changing:

ANTLR grammar organization

does not necessarily change:

Zamani language version

For example, these MAY be language-version-neutral:

splitting one grammar into imported grammars;

renaming internal ANTLR rules;

factoring duplicated productions;

changing parser implementation;

improving error recovery;

changing internal generated parser structure.


Provided that:

accepted source language
+
semantic interpretation

remain unchanged.

ANTLR itself supports grammar imports and modular grammar structures; Zamani may use that mechanism without treating every grammar-file reorganization as a language-version change. 


---

17. Grammar authority

grammar/specification/grammar-authority.md owns the final authority relationship among:

Zamani.g4
grammar.md
Zamani-Grammar.md
lexer/parser implementation
formal specification

This compatibility document only defines how those representations must remain compatible.

No grammar file may silently become an alternative language authority.


---

18. grammar/Zamani.g4

grammar/Zamani.g4 MUST:

conform to the normative specification;

remain deterministic;

avoid semantic actions that introduce hidden state;

avoid target-dependent parsing;

avoid fixed machine limits;

preserve source spans where required by the frontend;

accept only syntax belonging to its declared compatibility profile;

reject removed syntax where required;

preserve compatibility diagnostics;

remain synchronized with the Rust frontend.


A grammar change MUST trigger grammar conformance tests.


---

19. grammar/grammar.md

grammar/grammar.md MUST remain an implementation/conformance reference.

It MUST NOT silently become a competing specification.

If it describes an implementation limitation that is not a language rule, it MUST label that limitation as implementation-specific.

For example:

implementation limit

must not be documented as:

language maximum


---

20. grammar/Zamani-Grammar.md

grammar/Zamani-Grammar.md may contain:

design material;

historical material;

aspirational constructs;

extended language concepts.


However, every construct MUST have an explicit status.

Recommended status values:

Normative
Stable
Experimental
Proposed
Aspirational
Historical
Deprecated
Removed
Implementation-defined
Dialect-defined

A construct MUST NOT become stable merely because it appears in this document.


---

21. AST compatibility

The AST is a semantic bridge between parsing and analysis.

An AST change is compatible only if all affected semantic information remains representable without loss.

An AST migration MUST preserve:

source meaning;

source spans where required;

names;

types;

generic parameters;

effects;

capabilities;

resource requirements;

quantum semantic identity;

hardware intent;

diagnostics context;

version metadata.


AST implementation details MAY change internally if these contracts remain intact.


---

22. AST compatibility and source syntax

A syntax change may be source-compatible even if AST structures change.

For example:

old syntax

may lower into the same canonical AST representation as:

new syntax

This is preferred over creating duplicate semantic representations.


---

23. Semantic compatibility

Semantic compatibility is the highest-priority source compatibility dimension.

A compatible compiler MUST preserve:

meaning

even when it changes:

representation
optimization
instruction selection
scheduling
hardware mapping


---

24. Classical compatibility

Classical constructs MUST preserve their specified:

evaluation semantics;

numerical semantics;

type semantics;

memory semantics;

ownership semantics;

concurrency semantics;

synchronization semantics;

exception/error semantics;

deterministic behavior where specified.


Changing the CPU, GPU, accelerator, or execution environment MUST NOT silently change language semantics.


---

25. Quantum compatibility

Quantum compatibility MUST preserve the semantic meaning of quantum programs.

This includes, where applicable:

qubit identity;

register identity;

operation ordering;

parameter meaning;

control semantics;

measurement semantics;

reset semantics;

observable semantics;

classical feed-forward;

dynamic-circuit behavior;

probability semantics;

entanglement semantics;

state evolution;

resource requirements.


A backend MAY reject a program because it lacks a required capability.

It MUST NOT silently reinterpret the quantum program into a different computation.


---

26. Quantum IR boundary

quantum::ir is the canonical quantum semantic boundary.

Compatibility rules MUST preserve this architectural relationship:

Zamani quantum syntax
        ↓
AST
        ↓
semantic validation
        ↓
quantum::ir
        ↓
optimization
        ↓
routing
        ↓
scheduling
        ↓
ZQN / resilience
        ↓
hardware lowering
        ↓
execution

The grammar MUST NOT define a competing quantum IR.

Frontend-specific quantum nodes MUST be translated into the repository's canonical quantum IR boundary.

Quantum IR compatibility MUST therefore be versioned independently from source grammar compatibility.


---

27. Quantum backend compatibility

A quantum target may differ in:

qubit count;

connectivity;

native operations;

gate durations;

calibration;

fidelity;

noise;

measurement capabilities;

reset capabilities;

dynamic-circuit support;

pulse capabilities;

error-correction support.


These differences MUST be expressed through:

target capabilities;

resource requirements;

constraints;

calibration;

scheduling;

routing;

ZQN;

backend lowering.


They MUST NOT become arbitrary source-language compatibility restrictions.


---

28. No fixed quantum compatibility limits

The language MUST NOT define compatibility using constants such as:

MAX_QUBITS = 32
MAX_QUBITS = 64
MAX_QUBITS = 1024
MAX_REGISTER = N

If a program semantically requires:

N qubits

then N is part of that program's resource requirement.

It is not a language-wide maximum.

Therefore:

program requires N qubits

MUST remain valid for arbitrary representable N, subject only to the implementation and actual resource availability.


---

29. Quantum dialect compatibility

Quantum dialects MUST be independently versioned.

A dialect MUST declare:

dialect identity;

dialect version;

language compatibility range;

semantic dependencies;

capability requirements;

IR requirements;

interoperability requirements;

experimental/stable status.


A quantum dialect MUST NOT silently modify core quantum semantics.


---

30. QEC compatibility

Quantum error correction MUST remain compatible with the canonical quantum architecture.

QEC-specific semantics belong to the QEC subsystem.

The grammar MAY express:

logical-qubit intent;

code selection;

correction requirements;

resilience requirements;

fault-tolerance requirements;

logical/physical mapping intent.


The grammar MUST NOT duplicate QEC algorithms or runtime state.

QEC implementation compatibility MUST be evaluated separately from source grammar compatibility.


---

31. ZQN compatibility

ZQN owns quantum noise-aware execution concepts.

The compatibility boundary is:

Zamani source
    ↓
quantum semantics
    ↓
quantum::ir
    ↓
ZQN

The grammar MUST NOT create a second noise model.

Noise-channel, fault, calibration, and noise-aware execution compatibility MUST be handled by ZQN and its versioned interfaces.


---

32. HDL compatibility

HDL compatibility MUST distinguish:

hardware semantics

from:

technology implementation

A Zamani hardware description may express:

modules;

ports;

signals;

wires;

registers;

clocks;

timing;

combinational logic;

sequential logic;

state machines;

memory;

pipelines;

interfaces;

generic parameters.


Changing:

FPGA family
ASIC process
clock implementation
physical placement
device size
technology node

MUST NOT automatically constitute a language breaking change.

Those properties belong to target and implementation contracts.


---

33. Hardware compatibility

Hardware targets MUST expose capabilities through target descriptions.

Examples include:

supports.vector_width
supports.atomic_operations
supports.fpga
supports.quantum
supports.dynamic_circuit
supports.tensor_acceleration
supports.hardware_encryption

The exact capability schema belongs to the capability/resource system.

Compatibility MUST be determined from the declared capability contract rather than hard-coded device names.


---

34. Hardware identifiers

Source-level compatibility MUST NOT depend on permanent hardware identifiers unless hardware identity itself is explicitly part of program semantics.

The following MUST NOT be embedded as implicit language compatibility assumptions:

GPU0
QPU0
CPU0
device_7
node_12
fpga_3
address_0x...

If a program genuinely requires a specific physical device, that requirement MUST be explicit and classified as target/deployment policy rather than portable language semantics.


---

35. Resource compatibility

Resource requirements MUST be distinguished from target identity.

For example:

requires quantum capability

is different from:

requires processor with identifier X

Likewise:

requires at least N units of a resource

is different from:

requires exactly N physical devices

The first may be portable.

The second may be deployment-specific.

The compatibility system MUST preserve that distinction.


---

36. Capability compatibility

Capabilities describe what an execution environment can do.

Capabilities MUST be:

explicitly represented;

versioned where necessary;

discoverable;

composable;

deterministic;

target-independent at the language semantic level.


A capability MUST NOT silently change the semantics of existing source.


---

37. Requirement compatibility

Requirements describe what a program needs.

A requirement MUST NOT automatically specify how the implementation satisfies it.

Example:

requires:
    quantum

does not imply:

use vendor X

Similarly:

requires:
    accelerator

does not imply:

use GPU Y

This separation is necessary for POCO-REAF.


---

38. Constraint compatibility

Constraints restrict valid realizations.

Examples include:

maximum latency;

minimum reliability;

required precision;

energy budget;

timing requirements;

memory locality;

communication constraints.


Constraints MUST be treated separately from semantic computation.

A constraint failure SHOULD produce a target/resource diagnostic rather than a language parsing error.


---

39. Preference compatibility

Preferences SHOULD be non-semantic hints unless explicitly specified otherwise.

For example:

prefer:
    low_energy

MUST NOT mean:

change program meaning

If a preference cannot be satisfied, the implementation MAY choose another valid realization unless the source declares the preference mandatory.


---

40. Hint compatibility

Hints MUST NOT become hidden semantic requirements.

Hints MAY guide:

optimization;

placement;

scheduling;

memory layout;

accelerator selection;

quantum routing;

hardware lowering.


A hint that changes program meaning MUST NOT be classified as a hint.

It must instead be an explicit semantic requirement.


---

41. Concurrency compatibility

Concurrency changes are breaking when they alter specified observable semantics.

Compatibility analysis MUST consider:

task ordering;

synchronization;

memory visibility;

atomicity;

cancellation;

channel semantics;

actor semantics;

futures;

parallel execution;

data races;

deterministic ordering guarantees.


An optimization MUST preserve the language's declared concurrency contract.


---

42. Distributed compatibility

Distributed programs MUST distinguish:

logical distributed semantics

from:

physical topology

Changing:

node count;

machine type;

network topology;

region;

cloud provider;

service placement;


MUST NOT change source semantics unless explicitly part of the program contract.

Compatibility MAY fail because a target lacks a required distributed capability.

That is target incompatibility, not necessarily language incompatibility.


---

43. AI/ML compatibility

AI/ML language constructs MUST preserve semantic intent across:

CPU;

GPU;

TPU-like accelerators;

FPGA;

quantum acceleration;

distributed training;

local inference;

remote inference.


Tensor shapes that are genuinely part of an algorithm are semantic.

Hardware tensor limits are not.

Therefore:

algorithm tensor shape

MUST remain distinct from:

hardware tensor capacity


---

44. Networking compatibility

Networking syntax MUST distinguish:

protocol semantics
endpoint identity
deployment location
transport capability
security requirements

A source program MUST NOT become incompatible merely because:

an IP address changes;

a network interface changes;

a routing topology changes;

a node moves;

a service is deployed elsewhere.


Explicit deployment-specific requirements MAY be non-portable by design.


---

45. Security compatibility

Security semantics are compatibility-critical.

Changes to:

authentication;

authorization;

confidentiality;

integrity;

identity;

capability enforcement;

cryptographic semantics;

privacy guarantees;


MUST be treated as semantic changes when observable by the program or security boundary.

Security weakening MUST NOT be silently introduced as a compatibility conversion.


---

46. Memory compatibility

Memory semantics MUST remain stable across target architectures.

Differences in:

cache size;

NUMA topology;

address width;

memory hierarchy;

accelerator memory;

shared memory;

distributed memory;


MUST NOT silently redefine source-level memory semantics.


---

47. Numeric compatibility

Numeric compatibility MUST explicitly account for:

integer width;

signedness;

overflow;

floating-point behavior;

precision;

rounding;

NaN behavior;

infinity;

exact arithmetic;

approximate arithmetic.


A target MUST NOT silently substitute a less precise numerical representation if doing so violates the language contract.

Approximation MUST be explicit.


---

48. Serialization compatibility

Serialized Zamani artifacts MUST have an independently versioned format.

An artifact format version MUST NOT be inferred solely from:

compiler version

Serialized representations SHOULD include enough information to determine:

language version;

artifact version;

IR versions;

dialect versions;

required capabilities;

relevant semantic profiles.



---

49. IR compatibility

Every canonical IR MUST have its own compatibility contract.

At minimum:

source language version
        ≠
IR version

A compiler MAY lower multiple language versions into one IR version.

A compiler MAY lower one language version into different IR versions.

Compatibility MUST be explicitly declared.


---

50. Canonical IR preservation

IR transformations MUST preserve semantic equivalence.

For an IR transformation:

IR_A → IR_B

the compiler MUST establish:

Semantics(IR_A) = Semantics(IR_B)

unless the transformation is explicitly classified as:

approximation;

nondeterministic transformation;

target-dependent transformation;

optimization with defined semantic relaxation.



---

51. Classical IR compatibility

Classical IR compatibility MUST be independent from source grammar compatibility.

The classical IR MUST preserve:

control flow;

data flow;

type information;

memory semantics;

effects;

concurrency semantics;

required resource semantics.



---

52. Quantum IR compatibility

Quantum IR compatibility MUST preserve:

qubit identity;

operation identity;

operation parameters;

operation ordering;

controls;

measurement;

reset;

classical feed-forward;

dynamic behavior;

relevant resource requirements.


A target-specific decomposition MUST remain semantically equivalent to the source-level quantum operation.


---

53. Hardware IR compatibility

Hardware IR compatibility MUST preserve hardware intent while permitting implementation-specific realization.

The hardware IR MAY represent:

modules;

signals;

timing;

resources;

interfaces;

placement constraints;

pipeline requirements.


It MUST NOT force a particular physical technology unless that technology is explicitly selected.


---

54. ABI compatibility

ABI compatibility is separate from language compatibility.

An ABI MAY vary because of:

calling convention;

architecture;

data layout;

register convention;

binary format;

operating system;

runtime ABI.


A source program can remain language-compatible while requiring a different ABI realization.

ABI compatibility MUST therefore be evaluated at the interoperability/backend boundary.


---

55. Foreign-function compatibility

Foreign interfaces MUST declare:

foreign language;

ABI;

calling convention;

type mapping;

ownership expectations;

lifetime expectations;

error behavior;

thread/concurrency expectations;

version compatibility.


The core language MUST NOT pretend that arbitrary foreign APIs are semantically universal.


---

56. Runtime compatibility

Runtime compatibility means that a runtime can execute an artifact while preserving the artifact's declared semantics.

Runtime compatibility MUST be checked using:

language requirements
+
IR requirements
+
runtime capabilities
+
artifact contract

Runtime version alone is insufficient.


---

57. Target compatibility

A target is compatible when it can realize the program's declared semantics and required capabilities within its declared constraints.

A target MAY be:

CPU;

GPU;

FPGA;

ASIC;

QPU;

simulator;

accelerator;

embedded platform;

distributed system;

cloud environment;

heterogeneous platform;

future architecture.


Target compatibility MUST be capability-driven.


---

58. Target failure versus language failure

The implementation MUST distinguish:

invalid program

from:

valid program, unsupported target

For example:

Program requires dynamic quantum control.
Target lacks dynamic quantum control.

is not necessarily a language error.

It is:

target capability incompatibility

Similarly:

Program requires more memory than currently available.

is a:

resource/deployment incompatibility

not a source-language incompatibility.


---

59. POCO-REAF compatibility

POCO-REAF requires the following compatibility hierarchy:

Source
  ↓
Stable semantics
  ↓
Portable semantic representation
  ↓
Capability/resource requirements
  ↓
Target realization

The source should not be rewritten merely because:

target A → target B

changes.


---

60. Compile-once interpretation

"Compile Once" MUST NOT be interpreted as:

> One historical native machine-code binary must execute directly on every future architecture.



That is neither technically nor semantically guaranteed.

Instead, "Compile Once" means:

> A program can be compiled into a stable, portable semantic/compiled artifact whose target-specific realization can be performed without rewriting the original source semantics.



The artifact may contain:

canonical IR;

portable executable representation;

semantic metadata;

target-independent resource requirements;

capability requirements;

dialect metadata;

lowering metadata.


This distinction is REQUIRED for a technically defensible POCO-REAF model.


---

61. Forward compatibility

Future implementations SHOULD be able to consume older compatible artifacts.

This requires:

stable semantic contracts;

explicit version metadata;

migration rules;

version negotiation;

preserved provenance;

extensible serialization.



---

62. Backward compatibility

Older implementations MAY consume newer artifacts only when the newer artifact explicitly declares compatibility with the older implementation.

The language MUST NOT promise universal backward compatibility.

Unsupported features MUST be rejected deterministically.


---

63. Unknown fields and extensions

Versioned serialized formats SHOULD permit unknown extension fields to be skipped where doing so is semantically safe.

However:

unknown semantic field

MUST NOT be silently ignored if it affects program meaning.

The compatibility layer MUST distinguish:

unknown metadata

from:

unknown semantics


---

64. Feature negotiation

Feature negotiation MUST use explicit identifiers.

A feature SHOULD expose:

feature identity
feature version
required language version
required semantic contract
required IR contract
required capability
compatibility status

Feature negotiation MUST be deterministic.


---

65. Capability negotiation

Capability negotiation is separate from language version negotiation.

For example:

language:
    Zamani 1.x

capabilities:
    quantum
    dynamic-circuit
    fault-tolerance

A target may support:

quantum

but not:

dynamic-circuit

The result is a capability mismatch, not automatically a language mismatch.


---

66. Version ranges

Version ranges MUST be interpreted deterministically.

Conceptually:

>= 1.2

means:

any version satisfying the declared compatibility contract

not:

latest version

A resolver MUST NOT silently select an incompatible major version.


---

67. Dependency compatibility

Modules and packages MUST declare compatibility requirements when required.

A dependency may constrain:

language version
dialect version
API version
IR version
ABI
runtime
capability

These requirements MUST remain distinct.

For example:

requires language >= 1.2
requires dialect quantum >= 2.0
requires runtime capability dynamic-circuit

must not be collapsed into a single opaque version.


---

68. Mixed-version modules

A project MAY contain modules written against different compatible language versions if the module system explicitly supports it.

The resolver MUST:

1. identify each module's language version;


2. identify its dependencies;


3. determine compatibility;


4. select a valid compilation interpretation;


5. reject conflicts deterministically.



A module MUST NOT silently inherit a conflicting language version.


---

69. Package compatibility

Package compatibility MUST distinguish:

package format
package API
package source language
package dialects
package ABI
package runtime requirements

Updating package metadata alone MUST NOT imply source-language incompatibility.


---

70. Dialect compatibility

A dialect is compatible only when:

1. its namespace is unambiguous;


2. its version is declared;


3. its semantics are defined;


4. its dependencies are declared;


5. its interaction with the core language is specified;


6. conflicts are deterministically resolved.



Dialect extensions MUST NOT silently modify core keywords or semantics.


---

71. Vendor extensions

Vendor-specific extensions MAY exist.

They MUST be explicitly namespaced.

Vendor extensions MUST NOT masquerade as portable Zamani.

Example conceptual structure:

vendor::<namespace>::<feature>

The exact syntax belongs to the dialect system.

A program using a vendor extension MAY become target-specific.

That fact MUST be explicit.


---

72. Experimental features

Experimental features MUST be:

explicitly identifiable;

versioned;

isolated where practical;

documented;

testable;

removable without corrupting stable semantics.


An experimental feature MUST NOT silently become a stable feature with changed semantics.

Promotion to stable requires an explicit compatibility review.


---

73. Reserved syntax

Reserved syntax exists to preserve future design space.

Reserved constructs MUST NOT be accidentally consumed by unrelated features.

The reserved namespace is governed by:

grammar/specification/reserved-space.md

Reserved syntax is not automatically compatible source syntax.


---

74. Deprecation compatibility

Deprecation MUST be gradual.

Lifecycle:

Proposed
   ↓
Experimental
   ↓
Stable
   ↓
Deprecated
   ↓
Removal-eligible
   ↓
Removed

Removal MUST follow the language-version compatibility policy.

A feature MUST NOT jump directly from stable to removed without an explicitly justified compatibility transition.


---

75. Migration compatibility

Every breaking migration SHOULD provide:

old construct
new construct
semantic mapping
compatibility notes
automated migration possibility
manual migration guidance
diagnostics
tests

Migration tools MUST NOT silently alter semantics.


---

76. Compatibility shims

Compatibility shims MAY preserve old source or artifact behavior.

A shim MUST:

be version-scoped;

be deterministic;

preserve semantics;

be documented;

be testable;

have an explicit lifecycle.


A compatibility shim MUST NOT become an invisible permanent second implementation of the language.


---

77. Semantic aliases

Two syntactic forms MAY map to the same canonical semantic representation.

For example:

old syntax
new syntax

may both lower to:

same AST/IR semantic node

This is preferred when the constructs are genuinely semantically identical.


---

78. No duplicate semantic concepts

The compatibility architecture MUST prevent duplicate representations of the same semantic concept.

This is especially important for:

quantum operations;

qubit identity;

hardware resources;

capabilities;

effects;

resource requirements;

types;

memory semantics;

scheduling metadata.


A surface syntax may have aliases.

The semantic model should have one canonical meaning.


---

79. Compiler compatibility

Compiler implementations MUST declare which language versions they support.

A compiler SHOULD expose compatibility information including:

supported language versions
supported dialect versions
supported IR versions
supported artifact versions
supported target families
supported capabilities

The compiler MUST reject unsupported versions deterministically.


---

80. Compiler implementation version

Compiler version MUST remain separate from language version.

A compiler upgrade MAY preserve the same language version.

For example:

compiler 10
language 2.0

and:

compiler 11
language 2.0

can remain language-compatible.


---

81. Compiler optimizations

An optimization is compatibility-preserving only when it preserves specified semantics.

Examples:

constant folding
dead-code elimination
instruction scheduling
quantum gate simplification
classical vectorization
hardware-specific lowering

are compatible when their outputs remain semantically equivalent.


---

82. Scheduling compatibility

Scheduling MAY change:

operation timing;

physical placement;

execution order where dependencies permit;

resource allocation.


It MUST NOT violate:

dependencies;

causality;

timing semantics;

quantum semantics;

synchronization semantics;

memory semantics.



---

83. Routing compatibility

Quantum or distributed routing MAY change physical paths.

It MUST preserve logical semantics.

For quantum computation:

logical qubits

MUST remain distinct from:

physical qubits

unless the program explicitly makes physical identity part of its semantics.


---

84. Calibration compatibility

Calibration data is target/runtime information.

Calibration updates MUST NOT silently redefine the Zamani language.

A calibration profile MAY change how a valid program is realized on a device.

It MUST NOT change the intended computation.


---

85. ZQN compatibility

ZQN versions MUST be separately tracked from:

Zamani language version

and:

quantum::ir version

ZQN may evolve its noise models and execution interfaces while preserving the language semantic contract.


---

86. Resource-manager compatibility

Resource management MUST distinguish:

requested resources
available resources
allocated resources
actual resources

A change in available resources does not necessarily imply a language compatibility failure.

It may produce a runtime scheduling or deployment failure.


---

87. Scalability compatibility

Compatibility MUST remain independent of scale.

The same language construct MUST be valid conceptually for:

1
10
100
1,000
1,000,000
...

when the value is semantically representable and resources are available.

No compatibility rule may introduce arbitrary fixed thresholds.


---

88. Infinity principle

"Infinity" in the Zamani scalability model means:

> No language-defined finite machine-size ceiling is imposed where the concept is inherently scalable.



It does NOT mean:

infinite physical memory;

infinite computation;

infinite storage;

infinite bandwidth;

infinite execution time.


All implementations remain bounded by:

physical resources;

mathematical representability;

implementation resources;

deployment constraints;

operating environment.


The language MUST avoid adding artificial limits beyond those necessary for implementation correctness.


---

89. Integer and size compatibility

Quantities representing resource sizes SHOULD use semantically appropriate integer representations.

The grammar MUST NOT assume that a resource count fits a small fixed integer merely because current hardware is small.

Examples:

qubit_count
node_count
tensor_dimension
memory_size
array_length
resource_quantity

must be represented according to the language's general numeric/type model.


---

90. No grammar-level resource ceilings

Grammar productions MUST NOT contain constructs equivalent to:

registerSize : [1..32]
qubitCount : [1..1024]
coreCount : [1..256]

unless the range is genuinely part of the language's semantic definition.

Hardware limits belong to target/resource validation.


---

91. Target profiles

Target profiles MAY define:

supported capabilities
supported IR profiles
supported ABI
resource limits
supported dialects
supported execution modes

Target profiles MUST be externally selectable.

They MUST NOT alter the meaning of portable source.


---

92. Profiles versus language versions

A profile is not automatically a language version.

For example:

Zamani language 2.x
quantum profile adaptive

is conceptually different from:

Zamani language 3.x

Profiles SHOULD be used to describe coherent capability subsets when useful.


---

93. Compatibility matrix

The repository SHOULD maintain a machine-readable compatibility matrix.

Conceptually:

Producer	Consumer	Compatibility condition

old source	new compiler	supported language range
new source	old compiler	explicitly supported forward compatibility
old AST	new semantic layer	AST contract compatible
old IR	new backend	IR compatibility
new IR	old backend	backend declares support
old artifact	new runtime	artifact/runtime compatibility
new artifact	old runtime	explicitly supported
portable source	new hardware	capability satisfaction
quantum IR	new QPU	target capability satisfaction
HDL IR	new FPGA	hardware capability satisfaction


The actual matrix SHOULD be generated from authoritative version metadata rather than manually duplicated across many documents.


---

94. Compatibility metadata

Where a serialized artifact requires compatibility metadata, it SHOULD include a structure conceptually equivalent to:

CompatibilityMetadata {
    language_version
    grammar_revision
    semantic_model_version
    artifact_format_version
    dialect_versions
    ir_versions
    capability_requirements
    resource_requirements
    runtime_requirements
    abi_requirements
}

This is a conceptual contract.

The concrete serialization belongs to the artifact/IR subsystem.


---

95. Provenance

Artifacts SHOULD preserve provenance sufficient to reproduce or validate their semantic interpretation.

Relevant provenance MAY include:

source language version
grammar revision
specification revision
compiler version
IR version
dialect versions
target profile
backend version
build configuration
dependency resolution

Provenance MUST NOT be used to turn a compiler implementation version into a language semantic requirement unless explicitly necessary.


---

96. Reproducibility

Compatible compilation SHOULD be reproducible.

Given equivalent:

source
language version
dependency graph
dialect versions
compiler configuration
target-independent semantic inputs

the compiler SHOULD produce equivalent semantic artifacts.

Where binary reproducibility is not possible because of target-specific information, semantic reproducibility MUST remain the baseline.


---

97. Deterministic version resolution

Version resolution MUST be deterministic.

Given the same:

requirements
available versions
compatibility rules

the resolver MUST produce the same result.

Ambiguous resolution MUST be rejected or resolved through explicitly specified deterministic precedence.


---

98. Conflicting requirements

Conflicts MUST be diagnosed.

Example:

dependency A requires language >= 2.0
dependency B requires language < 2.0

must not silently select an arbitrary version.

The diagnostic SHOULD identify:

conflicting requirements;

source modules;

dependency paths;

compatible alternatives, if any.



---

99. Transitive compatibility

Compatibility MUST be evaluated transitively.

If:

A → B → C

then A is compatible only if all required compatibility contracts are satisfied.

A package MUST NOT claim compatibility merely because its direct dependencies are individually valid while a transitive dependency is incompatible.


---

100. Optional dependencies

Optional dependencies MAY introduce optional compatibility requirements.

A program that does not use an optional feature SHOULD NOT become incompatible merely because that feature exists.

Feature activation MUST be explicit or deterministically inferred.


---

101. Feature flags

Feature flags MUST NOT silently change stable language semantics.

A feature flag may select:

experimental syntax;

optional dialect;

optional backend;

optional optimization;

optional capability.


If a feature changes program meaning, it MUST be represented as a semantic profile/version rather than an arbitrary compiler switch.


---

102. Conditional compilation

Conditional compilation MUST preserve source compatibility rules.

Conditions MAY depend on:

language version;

dialect;

capability;

target;

compile-time configuration.


However:

target condition

must not be confused with:

language semantics

Target-specific branches are inherently conditional source behavior and therefore MAY reduce portability by design.


---

103. Portable conditional compilation

POCO-REAF prefers semantic capability conditions over physical machine conditions.

Prefer conceptually:

if capability(quantum)

over:

if device == "some-device"

when the intent is capability portability.


---

104. Target-specific source

Target-specific source is permitted when explicitly requested.

It MUST be identifiable.

Such code MAY legitimately depend on:

architecture;

device;

ABI;

hardware feature;

physical topology.


But it MUST NOT be mistaken for universally portable Zamani semantics.


---

105. Compatibility of target-specific code

Target-specific code SHOULD declare its compatibility requirements.

For example:

requires target capability X
requires ABI Y
requires dialect Z

This enables deterministic failure rather than mysterious compilation errors.


---

106. Interoperability compatibility

Interoperability interfaces MUST declare compatibility at the boundary.

Relevant systems include:

C;

C++;

Python;

OpenQASM;

Verilog;

system interfaces;

foreign ABIs;

external runtimes.


Imported representations MUST NOT silently become native Zamani semantics.

They MUST be translated through explicit interoperability contracts.


---

107. OpenQASM compatibility

OpenQASM compatibility MUST remain an interoperability concern.

Zamani may import/export compatible quantum representations.

The OpenQASM representation MUST NOT become the canonical Zamani quantum semantic model.

The pipeline remains:

OpenQASM
   ↓
interop parser
   ↓
Zamani semantic representation
   ↓
quantum::ir

and:

Zamani
   ↓
quantum::ir
   ↓
OpenQASM lowering/export

where supported.


---

108. HDL interoperability

Similarly:

Verilog/SystemVerilog/etc.

must remain an interoperability representation.

The canonical Zamani hardware semantics remain owned by Zamani's hardware/HDL architecture.


---

109. Compatibility with external standards

External standards MAY evolve independently.

Zamani MUST explicitly record the version of an external standard when compatibility depends on it.

Examples include:

external ABI;

serialization format;

quantum interchange format;

HDL dialect;

protocol;

cryptographic standard.


External standard version MUST NOT automatically become Zamani language version.


---

110. Diagnostic compatibility

Diagnostics are not normally semantic compatibility contracts.

However, production tooling SHOULD preserve stable diagnostic categories and machine-readable error codes.

Human-readable wording MAY improve.

Machine-readable identifiers SHOULD remain stable within the declared compatibility policy.


---

111. Error classification

Compatibility failures SHOULD be classified into categories such as:

E-LANG-VERSION
E-GRAMMAR-VERSION
E-DIALECT-VERSION
E-AST-VERSION
E-SEMANTIC-VERSION
E-IR-VERSION
E-ARTIFACT-VERSION
E-ABI-VERSION
E-RUNTIME-VERSION
E-TARGET-CAPABILITY
E-RESOURCE
E-DEPENDENCY-CONFLICT
E-UNSUPPORTED-FEATURE
E-REMOVED-FEATURE

The exact diagnostic identifiers belong to the repository's diagnostic system.

This document defines their conceptual categories.


---

112. Compatibility and error recovery

Parser recovery MUST NOT convert incompatible syntax into a valid but semantically different program.

If versioned syntax is unsupported, the parser SHOULD identify it explicitly where possible.

Silent reinterpretation is prohibited.


---

113. Security of compatibility metadata

Compatibility metadata MUST be treated as untrusted input.

The implementation MUST:

validate versions;

reject malformed metadata;

reject impossible combinations;

avoid panics;

avoid memory-unsafe parsing;

avoid unsafe;

avoid arbitrary code execution during metadata interpretation.


Compatibility metadata MUST NOT grant capabilities merely by declaring them.

Capability authorization belongs to the appropriate security/runtime subsystem.


---

114. Rust implementation requirement

The compatibility implementation MUST target:

Rust 1.97

or:

Rust 1.97.1

as the supported baseline.

The implementation MUST use safe Rust.

unsafe MUST NOT be introduced to:

parse versions;

resolve compatibility;

deserialize metadata;

validate dialects;

construct compatibility matrices;

perform migrations;

inspect artifacts;

validate requirements.


If a dependency requires unsafe internally, that dependency MUST be evaluated against the project's safety policy before adoption.


---

115. Compatibility implementation architecture

The implementation SHOULD separate:

version parsing
        ↓
version comparison
        ↓
range evaluation
        ↓
compatibility classification
        ↓
dependency resolution
        ↓
capability validation
        ↓
diagnostics

Each layer MUST have a clear ownership boundary.


---

116. No compatibility cycles

The dependency graph MUST remain acyclic.

The intended architecture is:

version primitives
        ↓
compatibility primitives
        ↓
language compatibility
        ↓
dialect compatibility
        ↓
artifact compatibility
        ↓
compiler/runtime compatibility
        ↓
target compatibility

Compatibility infrastructure MUST NOT depend on the runtime merely to determine language-version validity.


---

117. Compatibility and grammar

The grammar may parse version declarations.

It MUST NOT perform semantic compatibility resolution.

The correct pipeline is:

source
 ↓
lexer
 ↓
parser
 ↓
AST
 ↓
version extraction
 ↓
compatibility validation
 ↓
semantic analysis

Version syntax belongs to:

grammar/core/versioning.g4

Version semantics belong to:

grammar/specification/language-version.md
grammar/specification/compatibility.md


---

118. Compatibility and semantic analysis

Semantic analysis owns:

version validation;

dialect compatibility;

capability compatibility;

requirement compatibility;

type compatibility;

effect compatibility.


The parser SHOULD preserve sufficient version metadata in the AST for semantic analysis.


---

119. Compatibility and compilation

Compilation MUST refuse to proceed when required semantic compatibility cannot be established.

Examples:

unsupported language version
incompatible dialect
unsupported IR version
missing required capability
incompatible ABI
unsupported artifact

The compiler MUST NOT silently downgrade semantics.


---

120. Compatibility and execution

Runtime execution MUST validate artifact compatibility before executing an artifact.

At minimum:

artifact format
language semantics
IR contracts
dialects
required capabilities
runtime contract

must be checked.


---

121. Compatibility and scheduling

Scheduling compatibility is not merely version matching.

The scheduler MUST determine whether:

semantic operation
+
timing requirements
+
resource requirements
+
target capabilities

can be satisfied.

A scheduling failure MUST NOT be reported as a language-version failure unless that is actually the cause.


---

122. Compatibility and optimization

Optimizers MUST declare the semantic contract they preserve.

An optimization MUST NOT exploit undefined or target-specific behavior as though it were universal language behavior.


---

123. Compatibility and hardware abstraction

Hardware abstraction layers MUST expose capabilities rather than leaking implementation assumptions into source compatibility.

The source language should depend on:

what can be done

rather than:

which exact machine currently exists

where portable semantics are intended.


---

124. Compatibility and resource management

Resource managers MUST remain downstream of semantic compatibility.

The resource manager determines:

can this target satisfy this valid program's requirements?

It does not determine:

is this source language construct valid?


---

125. Compatibility and QEC

QEC compatibility belongs after quantum semantic lowering.

The grammar declares intent.

Quantum semantic analysis validates intent.

quantum::ir represents canonical semantics.

QEC determines a compatible error-correction realization.


---

126. Compatibility and ZQN

ZQN compatibility is evaluated against:

quantum::ir
+
noise model
+
target capabilities
+
execution policy

It MUST NOT require the grammar to understand every physical noise model.


---

127. Compatibility and benchmarking

Benchmarking MUST NOT redefine semantic compatibility.

A benchmark MAY report:

supported capabilities;

performance;

latency;

throughput;

resource usage;

fidelity;

energy.


Those measurements describe implementation/target behavior.

They do not redefine language meaning.


---

128. Compatibility and examples

Examples under:

grammar/examples/

MUST declare or inherit their intended compatibility profile.

Examples MUST NOT silently demonstrate experimental syntax as stable language.


---

129. Compatibility tests

Compatibility testing MUST include:

Positive tests

old source with new compatible compiler;

compatible dialect combinations;

compatible IR combinations;

compatible artifacts;

compatible targets.


Negative tests

unsupported language version;

removed feature;

incompatible dialect;

conflicting requirements;

incompatible IR;

unsupported artifact;

missing target capability.


Boundary tests

smallest valid version;

largest representable version;

pre-release versions if supported;

large dependency graphs;

many dialects;

large compatibility matrices;

very large resource expressions.



---

130. Quantum compatibility tests

Tests MUST include:

arbitrary qubit identifiers;

dynamically sized registers;

logical/physical mapping;

parameterized operations;

controls;

measurement;

mid-circuit measurement;

reset;

dynamic circuits;

quantum/classical interaction;

quantum dialect versions;

unsupported target capabilities;

large qubit counts;

no hard-coded qubit ceiling.



---

131. Classical compatibility tests

Tests MUST cover:

primitive types;

composite types;

generics;

functions;

memory;

concurrency;

parallelism;

numerical operations;

symbolic operations;

accelerator selection;

large data structures.



---

132. HDL compatibility tests

Tests MUST cover:

modules;

ports;

signals;

wires;

registers;

clocks;

timing;

combinational logic;

sequential logic;

state machines;

pipelines;

hardware generics;

target-independent hardware semantics;

target-specific capability failures.



---

133. Cross-domain compatibility tests

The compatibility suite MUST include combinations such as:

classical + quantum
classical + HDL
quantum + HDL
quantum + hardware
quantum + distributed
AI + quantum
AI + hardware
classical + quantum + distributed
classical + quantum + HDL + hardware

The purpose is to ensure that compatibility contracts compose rather than work only in isolation.


---

134. Scalability tests

Compatibility tests MUST verify that no language compatibility rule introduces artificial limits.

Test parameterization SHOULD cover increasing scales rather than fixed constants.

The test framework MUST avoid encoding a new arbitrary maximum merely because the test machine has limited resources.


---

135. Determinism tests

Given identical:

source
language version
dialect set
dependency set
compatibility metadata

compatibility resolution MUST be deterministic.

Repeated runs MUST produce equivalent results.


---

136. Round-trip compatibility tests

Where serialization is supported:

source
 ↓
AST
 ↓
semantic representation
 ↓
artifact
 ↓
decode
 ↓
semantic representation

MUST preserve semantic meaning.

Version metadata MUST survive the round trip.


---

137. Compatibility fuzzing

The implementation SHOULD fuzz:

version strings;

version ranges;

compatibility metadata;

dialect declarations;

dependency graphs;

artifact headers;

malformed serialized input.


Fuzzing MUST verify:

no panic;

no infinite loop;

deterministic result;

safe resource behavior;

no unsafe.



---

138. Compatibility matrix tests

Every supported version combination SHOULD be tested through a matrix.

Conceptually:

producer version × consumer version

The matrix SHOULD classify:

compatible
conditionally compatible
migration required
incompatible
unknown

Unknown MUST NOT be treated as compatible by default.


---

139. Hard-coding audit

Compatibility implementation MUST undergo a hard-coding audit.

Search for:

MAX_VERSION
MAX_DIALECTS
MAX_MODULES
MAX_DEPENDENCIES
MAX_QUANTUM_VERSION
MAX_QUBITS
MAX_DEVICES
MAX_TARGETS
MAX_RESOURCES
MAX_FEATURES

Any fixed constant must be classified as:

1. semantic requirement;


2. representation requirement;


3. implementation safety limit;


4. resource limit;


5. test-only limit;


6. accidental hard-coding.



Accidental limits MUST be removed.


---

140. Legitimate implementation limits

Some implementation limits may be unavoidable.

Examples:

available memory;

addressable integer range;

parser stack;

operating-system limits;

filesystem limits;

network limits.


Such limits MUST:

be documented;

be detected;

produce diagnostics;

not be presented as language semantics;

not become compatibility guarantees;

not unnecessarily restrict other implementations.



---

141. No fixed machine compatibility

Compatibility MUST NEVER require:

exact CPU count
exact GPU count
exact QPU count
exact FPGA count
exact memory capacity
exact node count
exact topology

unless the program explicitly declares those as deployment requirements.


---

142. Program-level resource requirements

If a program requires a quantity as part of its algorithm, that quantity is semantic.

For example:

create N qubits

means the program requires N qubits.

It does NOT establish:

Zamani supports at most N qubits

The compatibility layer MUST preserve this distinction.


---

143. Target portability

A program is portable when its semantic requirements can be satisfied by multiple targets.

Portability MUST NOT mean that every target can execute every program.

Instead:

portable semantics
+
target capability negotiation
=
portable realization


---

144. Partial portability

A program MAY be portable across a capability class but not universally portable.

Examples:

requires quantum
requires dynamic quantum control
requires FPGA
requires vector acceleration
requires distributed execution

Compatibility metadata SHOULD make such requirements explicit.


---

145. Graceful degradation

Zamani MAY support explicit graceful degradation.

For example:

prefer accelerator
fallback to CPU

may be valid.

But fallback MUST preserve semantics.

A fallback that changes the mathematical, quantum, security, or timing meaning MUST NOT occur silently.


---

146. Approximation compatibility

Approximate execution MUST be explicit.

Examples:

reduced numerical precision;

approximate optimization;

approximate simulation;

approximate quantum synthesis.


The source or compilation profile MUST identify the permitted semantic relaxation.


---

147. Deterministic versus nondeterministic compatibility

If the language specifies nondeterminism, compatibility MUST preserve the allowed result space.

It need not produce identical execution traces unless determinism is required.

The compatibility contract must therefore distinguish:

exact result equality

from:

equivalent allowed behavior


---

148. Floating-point compatibility

Floating-point implementations MAY differ within explicitly documented language guarantees.

A compiler MUST NOT claim bit-for-bit compatibility unless the language specification requires it.

Numerical contracts SHOULD identify:

precision;

rounding;

exception behavior;

reproducibility requirements.



---

149. Temporal compatibility

Timing semantics MUST be preserved where timing is part of program meaning.

Hardware implementation timing MAY differ where timing is merely an optimization concern.

This distinction is especially important for:

HDL;

real-time systems;

quantum pulse execution;

distributed systems;

embedded systems.



---

150. Real-time compatibility

A real-time requirement MUST be represented as a semantic/constraint requirement when it is part of correctness.

The target system must then determine whether the requirement can be satisfied.

A target failing a real-time requirement is not automatically a language incompatibility.


---

151. Energy compatibility

Energy constraints MAY be expressed as resource/constraint metadata.

Changing hardware energy efficiency MUST NOT change program semantics.

A target that cannot satisfy the energy constraint is incompatible with that deployment requirement.


---

152. Reliability compatibility

Reliability requirements MUST be explicit.

Examples:

required reliability
fault tolerance
replication
error correction

A target must satisfy declared reliability requirements or reject deployment.

It MUST NOT silently weaken them.


---

153. Security-profile compatibility

Security profiles MUST be explicit.

A program requiring a stronger security profile MUST NOT run under a weaker profile merely because the syntax remains valid.

Compatibility includes security guarantees when they are part of the program contract.


---

154. Capability monotonicity

Adding a capability to a target MUST NOT make previously valid programs invalid merely because the capability exists.

Capability discovery is additive unless explicitly documented.


---

155. Version monotonicity

Version numbers MUST be comparable according to the versioning policy.

Compatibility MUST NOT depend on:

string sorting

or implementation-specific textual ordering.

Version parsing MUST be canonical.


---

156. Version normalization

Equivalent version representations MUST normalize to the same semantic version where the syntax permits equivalent forms.

Malformed or ambiguous versions MUST be rejected.


---

157. Pre-release versions

If pre-release versions are supported, they MUST have explicit ordering and compatibility rules.

A pre-release MUST NOT silently satisfy a stable dependency unless the version policy explicitly permits it.


---

158. Build metadata

Build metadata SHOULD NOT affect semantic compatibility.

For example:

1.2.3+build123

and:

1.2.3+build456

may represent the same language semantics while differing in provenance.

Build metadata may still matter for reproducibility and diagnostics.


---

159. Language editions

If Zamani introduces language editions in the future, an edition MUST be distinct from the semantic version.

An edition MAY define:

default syntax;

compatibility behavior;

migration mode.


An edition MUST NOT silently reinterpret old source.


---

160. Profiles

Profiles MAY define restricted or extended capability subsets.

Profiles MUST:

have names;

have versions;

define supported features;

define compatibility requirements;

avoid changing stable semantics.



---

161. Compatibility with future computing paradigms

The compatibility system MUST be extensible to future domains without requiring a redesign of the core compatibility architecture.

Potential future domains may include:

neuromorphic computing;

photonic computing;

molecular computing;

biological computing;

reversible computing;

optical accelerators;

analog computing;

probabilistic computing;

novel quantum models;

unknown future architectures.


Future domains MUST integrate through:

semantic capability
+
resource model
+
dialect/profile
+
canonical IR
+
target lowering

rather than by introducing arbitrary compatibility exceptions.


---

162. Compatibility and extensibility

A future feature MUST be able to declare:

feature identity
feature version
language compatibility
semantic dependencies
IR dependencies
capability requirements
target requirements
migration policy

without modifying unrelated compatibility mechanisms.


---

163. Reserved compatibility space

The compatibility architecture MUST reserve namespace for:

future language versions
future dialects
future profiles
future IRs
future artifact formats
future target models
future capability classes

Reserved space MUST be documented.


---

164. Compatibility and repository ownership

Every compatibility-relevant concept MUST have one owner.

Examples:

Concept	Owner

Language version policy	language-version.md
Compatibility policy	compatibility.md
Grammar authority	grammar-authority.md
Syntax	syntax-model.md
Semantics	semantic-model.md
Compilation semantics	compilation-model.md
Execution semantics	execution-model.md
Scalability	scalability-model.md
POCO-REAF	poco-reaf.md
Concrete ANTLR syntax	Zamani.g4
Rust lexer	src/lexer.rs
Rust parser	src/parser.rs
AST	src/ast/
Semantic analysis	repository semantic subsystem
Quantum canonical semantics	quantum::ir
Quantum noise	ZQN
Error correction	QEC subsystem
Scheduling	scheduling subsystem
Hardware realization	hardware/backend subsystem


No compatibility document should duplicate the implementation ownership of these systems.


---

165. Integration with grammar/core/versioning.g4

grammar/core/versioning.g4 MUST own the concrete grammar for:

version declarations;

version requirements;

version ranges;

version references;

where applicable, compatibility/profile declarations.


compatibility.md owns the meaning of those constructs.

The grammar MUST NOT encode the entire compatibility algorithm.


---

166. Integration with grammar/dialects/versioning.g4

grammar/dialects/versioning.g4 MUST own dialect-version syntax.

Dialect compatibility semantics belong here.

The dialect grammar MUST reference the central version model rather than invent another version syntax.


---

167. Integration with grammar/compatibility/versions.md

grammar/compatibility/versions.md SHOULD contain concrete compatibility tables and supported release relationships.

It MUST derive from this policy rather than contradict it.


---

168. Integration with migrations

grammar/compatibility/migrations.md owns detailed migration procedures.

This document defines the compatibility requirement:

breaking change → migration contract

The migration document defines the concrete transformations.


---

169. Integration with deprecations

grammar/compatibility/deprecated.md owns the inventory of deprecated constructs.

This document defines their compatibility lifecycle.


---

170. Integration with reserved space

grammar/compatibility/reserved.md owns concrete reserved identifiers and syntax.

This document defines why reserved space is necessary for compatibility.


---

171. Integration with compatibility matrix

grammar/compatibility/compatibility-matrix.md SHOULD provide the concrete matrix generated from supported versions.

It MUST NOT introduce compatibility rules not defined here.


---

172. Integration with tests

The compatibility test suite MUST be the executable verification of this document.

Tests SHOULD be organized under:

grammar/tests/compatibility/

and, where relevant:

grammar/tests/cross-domain/
grammar/tests/scalability/
grammar/tests/determinism/
grammar/tests/roundtrip/


---

173. Integration with examples

Compatibility examples SHOULD demonstrate:

old source → new compiler
portable source → multiple targets
quantum source → multiple quantum backends
HDL source → multiple hardware targets
classical source → CPU/GPU/accelerator

Examples MUST clearly distinguish portable from target-specific code.


---

174. Integration with documentation

Documentation MUST identify:

normative rules;

implementation behavior;

experimental features;

target-specific behavior;

compatibility guarantees.


Documentation MUST NOT promise stronger compatibility than the implementation provides.


---

175. Compatibility conformance

A Zamani implementation is conformant only if it:

1. implements the declared language version;


2. respects compatibility classifications;


3. rejects unsupported versions deterministically;


4. preserves stable semantics;


5. respects dialect compatibility;


6. respects IR compatibility;


7. respects artifact compatibility;


8. does not silently weaken requirements;


9. does not introduce accidental machine limits;


10. passes the relevant conformance suite.




---

176. Conformance levels

Implementations MAY declare:

Core Conformance
Extended Conformance
Quantum Conformance
HDL Conformance
Hybrid Conformance
Distributed Conformance
AI/Data Conformance
Interoperability Conformance
Full Conformance

Each conformance level MUST define its supported capabilities.

"Full" MUST NOT mean "supports every possible future target."

It means complete support for the currently declared stable contract.


---

177. Compatibility claims

An implementation MUST NOT claim:

compatible

without identifying the compatibility dimension.

Prefer:

source-compatible with Zamani 2.x
quantum::ir-compatible with profile X
artifact-compatible with format Y
target-compatible with capability set Z


---

178. Unknown compatibility

Unknown compatibility MUST NOT be treated as success.

The default rule is:

unknown ≠ compatible

unless a specific forward-compatibility rule explicitly guarantees safe handling.


---

179. Conservative failure

When compatibility cannot be established safely, the implementation SHOULD fail closed.

It MUST NOT silently guess.

This is especially important for:

security;

quantum semantics;

numerical semantics;

memory semantics;

ABI;

serialized artifacts;

distributed consistency.



---

180. Semantic preservation rule

The strongest compatibility invariant is:

If an implementation claims semantic compatibility,
the observable meaning of the program MUST be preserved.

Optimization, target selection, scheduling, routing, and lowering do not exempt the implementation from this rule.


---

181. Hardware independence rule

The language version MUST remain independent of:

CPU architecture
GPU architecture
FPGA family
ASIC technology
QPU vendor
QPU topology
memory capacity
network topology
cluster size

unless those properties are explicitly part of a program's target-specific contract.


---

182. Resource independence rule

Resource availability MUST NOT determine whether syntax is valid.

For example:

create 1,000,000 qubits

may be syntactically and semantically valid even if the current target cannot execute it.

The correct result is:

valid program
+
insufficient target resources

not:

invalid Zamani syntax


---

183. Compatibility and scale

Compatibility tests MUST include progressively larger resource expressions.

The implementation SHOULD use generated tests rather than encoding arbitrary maximum values into grammar rules.


---

184. Compatibility and arbitrary precision

Where a quantity is semantically unbounded, the compatibility layer SHOULD use the language's general numeric representation rather than a compatibility-specific narrow integer.


---

185. Compatibility and parser limits

Parser implementation limits MUST NOT be represented as language compatibility limits.

If implementation recursion is insufficient for extremely deep input, the parser SHOULD use iterative/worklist techniques where practical.

A parser implementation limitation MUST produce an implementation/resource diagnostic, not redefine the language.


---

186. Compatibility and source size

The language MUST NOT impose an arbitrary compatibility maximum on:

source files;

modules;

declarations;

expressions;

statements;

imports.


Implementations may have practical resource limits.

Those limits MUST remain implementation-specific.


---

187. Compatibility and program generation

Generated programs MUST obey the same compatibility contracts as handwritten programs.

Generators MUST NOT exploit undocumented parser behavior.


---

188. Compatibility and macros

Macros MUST be version-aware where their expansion depends on language syntax or semantics.

A macro expansion MUST NOT silently generate syntax incompatible with the declared language version.

Macro definitions SHOULD declare their compatibility range.


---

189. Compatibility and metaprogramming

Compile-time programs MUST use the declared language/semantic environment.

A compiler MUST NOT allow metaprogramming to bypass compatibility validation.

Generated artifacts MUST be validated under the same compatibility rules as source.


---

190. Compatibility and reflection

Reflection MUST NOT expose implementation details as though they were stable language contracts unless explicitly specified.

Reflective APIs SHOULD distinguish:

stable language metadata

from:

implementation metadata


---

191. Compatibility and standard library

The standard library has its own API compatibility surface.

Language compatibility does not automatically guarantee library compatibility.

The library SHOULD declare:

API version;

language compatibility;

ABI compatibility where relevant;

runtime requirements;

target capabilities.



---

192. Compatibility and package manager

The package manager MUST resolve:

language compatibility
package compatibility
dependency compatibility
dialect compatibility

without confusing them.

A package manager MUST NOT silently upgrade a language version merely to satisfy an unrelated dependency.


---

193. Compatibility and build systems

Build metadata MAY specify:

compiler version;

target;

profile;

features;

dialects;

dependencies.


Build configuration MUST NOT silently redefine source semantics.


---

194. Compatibility and deployment

Deployment systems MUST validate:

artifact
runtime
target
capabilities
resources
security

before execution where required.


---

195. Compatibility and cloud execution

Cloud deployment MUST preserve semantic compatibility.

Moving:

local → cloud

MUST NOT require source changes solely because execution location changed.

Deployment-specific requirements remain separate.


---

196. Compatibility and embedded execution

Embedded targets may have strict resource constraints.

Those constraints MUST be represented as target/resource constraints rather than universal language limits.


---

197. Compatibility and simulators

A simulator MAY implement a broader or narrower capability set than physical hardware.

A simulator MUST clearly declare its compatibility profile.

Simulation MUST NOT silently claim physical equivalence where it does not exist.


---

198. Quantum simulation compatibility

A quantum simulator may support arbitrary program sizes subject to available resources.

The grammar MUST NOT encode simulator-specific maximums.

Simulator limits belong to execution resources.


---

199. Hardware simulation compatibility

HDL/hardware simulation MAY support constructs not synthesizable to every target.

The semantic model MUST distinguish:

simulation capability

from:

synthesis capability


---

200. Compatibility and synthesis

A valid hardware description may be:

simulation-compatible

but:

not synthesis-compatible

for a particular target.

This MUST be reported as target/synthesis incompatibility rather than source invalidity unless the source violates the language itself.


---

201. Compatibility and optimization profiles

Optimization profiles MUST preserve language semantics unless explicitly marked as approximate.

For example:

-O0
-O1
-O2
-O3

are implementation choices, not language versions.


---

202. Compatibility and debug information

Debug metadata MAY change independently of language semantics.

Debug representation versions SHOULD be separately versioned.


---

203. Compatibility and source maps

Source-map formats MUST preserve sufficient source identity for diagnostics.

Changing source-map representation does not automatically imply a language version change.


---

204. Compatibility and formatting

Formatting changes MUST NOT alter semantics.

A formatter MAY evolve independently provided it preserves valid syntax and meaning.


---

205. Compatibility and linting

Lint rules MUST NOT silently become language semantics.

A lint warning MAY become an error only under an explicitly declared policy/profile.


---

206. Compatibility and IDEs

IDE tooling SHOULD consume the same compatibility metadata as the compiler.

The IDE MUST NOT invent syntax accepted by the editor but rejected by the canonical compiler without clearly identifying experimental support.


---

207. Compatibility and syntax highlighting

Syntax highlighting MAY lag or lead the grammar during development, but released tooling SHOULD correspond to the declared language version.

Highlighting is not a semantic authority.


---

208. Compatibility and ANTLR

ANTLR-generated artifacts MUST be treated as generated implementation artifacts.

Changing ANTLR-generated output does not itself constitute a language change.

Changing the grammar's accepted language or semantics may constitute a language change.

The ANTLR grammar MUST remain conformance-tested against the authoritative Zamani language contract.


---

209. Compatibility and multiple frontends

If Zamani later supports multiple frontends:

Rust frontend
ANTLR frontend
IDE parser
incremental parser
language server parser

they MUST implement the same language compatibility contract.

A frontend-specific dialect MUST be explicitly identified.


---

210. No frontend dialect drift

It is prohibited for:

frontend A

to accept syntax that:

frontend B

rejects while both claim the same language version, unless the difference is explicitly documented as implementation-defined or dialect-specific.


---

211. Compatibility and source spans

Version/compatibility diagnostics MUST preserve source locations where possible.

A diagnostic SHOULD identify:

source file;

span;

requested version;

available version;

conflicting requirement;

resolution path;

migration suggestion.



---

212. Compatibility and deterministic diagnostics

For identical inputs, compatibility diagnostics SHOULD be deterministic.

Dependency traversal order MUST NOT cause arbitrary diagnostics.


---

213. Compatibility and dependency cycles

Dependency cycles MUST be detected deterministically.

The compatibility resolver MUST NOT recurse indefinitely.


---

214. Compatibility and very large dependency graphs

The compatibility model MUST support large graphs subject to available resources.

It MUST NOT impose arbitrary graph-size ceilings as language semantics.

Implementations SHOULD use efficient graph algorithms and bounded resource policies where necessary.


---

215. Compatibility and caching

Compatibility results MAY be cached.

Cache keys MUST include all relevant compatibility inputs.

A cache MUST NOT return a compatibility result for a semantically different input.


---

216. Compatibility and hashing

Where compatibility metadata is hashed, canonical serialization MUST be used.

Equivalent metadata MUST produce equivalent canonical representations.


---

217. Compatibility and signatures

Signed artifacts SHOULD include compatibility metadata within the authenticated artifact boundary.

Changing compatibility metadata after signing MUST invalidate the signature where the metadata is security-critical.


---

218. Compatibility and trust

Compatibility MUST NOT imply trust.

A compatible artifact may still be:

untrusted;

unauthorized;

malicious;

invalid for deployment.


Security authorization remains a separate concern.


---

219. Compatibility and sandboxing

Sandbox policy MUST be separate from language compatibility.

A valid program may be rejected because it lacks permission.

That is:

authorization failure

not:

language incompatibility


---

220. Compatibility and privacy

Privacy requirements MUST be preserved across compatible execution environments.

A target that cannot satisfy required privacy semantics MUST reject the deployment or select an explicitly permitted alternative.


---

221. Compatibility and cryptography

Cryptographic semantics MUST be explicit.

A target-specific cryptographic acceleration may change implementation, not meaning.

Replacing an algorithm with a different algorithm is not compatibility-preserving unless the language explicitly defines them as semantically interchangeable.


---

222. Compatibility and future versions

Future versions MUST preserve this compatibility architecture.

A new version may extend:

language
grammar
types
effects
capabilities
resources
quantum
HDL
hardware
distributed
AI
data
networking
security

without changing the ownership model.


---

223. Compatibility extension protocol

Every new language feature SHOULD be reviewed using:

1. What semantic concept is introduced?
2. Who owns it?
3. Is it syntax or semantic infrastructure?
4. Is it backward compatible?
5. Does it require a language version change?
6. Does it require an IR version change?
7. Does it require a dialect?
8. Does it require capability negotiation?
9. Does it require migration?
10. What tests prove compatibility?


---

224. Feature admission checklist

Before a feature becomes stable:

[ ] Syntax defined.

[ ] Semantics defined.

[ ] AST representation defined.

[ ] Type behavior defined.

[ ] Effect behavior defined.

[ ] Capability behavior defined.

[ ] Resource behavior defined.

[ ] IR mapping defined.

[ ] Compiler behavior defined.

[ ] Runtime behavior defined.

[ ] Target interaction defined.

[ ] Compatibility classification assigned.

[ ] Migration policy defined if necessary.

[ ] Tests implemented.

[ ] Documentation synchronized.



---

225. Breaking-change checklist

Before accepting a breaking change:

[ ] Existing programs identified.

[ ] Semantic impact analyzed.

[ ] Grammar impact analyzed.

[ ] AST impact analyzed.

[ ] IR impact analyzed.

[ ] Dialect impact analyzed.

[ ] Package impact analyzed.

[ ] Runtime impact analyzed.

[ ] Interoperability impact analyzed.

[ ] Migration defined.

[ ] Deprecation period considered.

[ ] Tests updated.

[ ] Compatibility matrix updated.

[ ] Documentation updated.



---

226. File-level independent completion contract

This document itself MUST be independently complete.

File

grammar/specification/compatibility.md

Purpose

Define the normative compatibility contract for Zamani.

Owns

Compatibility semantics and compatibility policy.

Does not own

Concrete syntax, AST implementation, IR implementation, runtime implementation, or target implementation.

Inputs

language version policy;

grammar authority;

syntax model;

semantic model;

compilation model;

execution model;

scalability model;

dialect system;

IR contracts;

runtime contracts.


Outputs

compatibility rules;

compatibility classifications;

compatibility decision framework;

compatibility test requirements;

integration contracts.


Dependencies

Normative specification documents and repository architecture.

Upstream contracts

Primarily:

language-version.md
language-principles.md
language-scope.md
grammar-authority.md

Downstream consumers

grammar;

lexer;

parser;

AST;

semantic analysis;

compiler;

IR;

runtime;

package tooling;

dialect system;

interoperability;

tests.


Public grammar contract

This document defines semantics of version/compatibility constructs but does not define their concrete grammar.

AST contract

Version and compatibility metadata parsed from source MUST be representable without loss.

Semantic contract

Compatibility MUST be resolved deterministically.

IR integration

Relevant provenance and compatibility requirements MAY be carried into canonical IR metadata.

Compiler integration

The compiler MUST validate compatibility before target realization.

Runtime integration

The runtime MUST validate artifact compatibility before execution.

Tooling integration

Tooling SHOULD expose compatibility information consistently.

Cross-domain integration

The same compatibility model MUST work for classical, quantum, HDL, hardware, distributed, AI, data, networking, and future domains.

Tests

All compatibility dimensions MUST have positive, negative, boundary, deterministic, and cross-domain tests.

Negative tests

Unsupported versions, incompatible dialects, missing capabilities, conflicting dependencies, and incompatible artifacts MUST fail deterministically.

Boundary tests

Very small and very large version/resource/dependency values MUST be tested without introducing artificial semantic limits.

Compatibility requirements

This file is the normative compatibility authority.

Scalability requirements

No arbitrary machine-size or feature-count ceiling may be introduced.

Hard-coding audit

All fixed limits must be classified and justified.

Completion criteria

This file is complete when:

all compatibility dimensions are defined;

ownership is unambiguous;

version layers are separated;

source compatibility is defined;

IR compatibility is defined;

artifact compatibility is defined;

target compatibility is defined;

quantum compatibility is defined;

HDL compatibility is defined;

migration rules are defined;

dialect compatibility is defined;

POCO-REAF compatibility is defined;

hard-coding rules are defined;

testing requirements are defined;

repository integration is explicit.



---

227. File integration matrix

File/subsystem	Relationship

language-version.md	Defines version model; this file defines compatibility
grammar-authority.md	Defines authority; this file defines cross-representation compatibility
syntax-model.md	Defines syntax; this file defines syntax compatibility
semantic-model.md	Defines semantics; this file defines semantic compatibility
compilation-model.md	Defines compilation; this file defines compiler/IR compatibility
execution-model.md	Defines execution; this file defines runtime/target compatibility
scalability-model.md	Defines scaling; this file prevents compatibility limits becoming scale limits
poco-reaf.md	Defines POCO-REAF; this file defines its compatibility guarantees
core/versioning.g4	Concrete version syntax
dialects/versioning.g4	Concrete dialect-version syntax
Zamani.g4	Concrete ANTLR representation
grammar.md	Implementation/conformance reference
Zamani-Grammar.md	Design/explanatory material
src/lexer.rs	Lexical implementation
src/parser.rs	Parsing implementation
src/ast/	AST representation
semantic analysis	Compatibility validation
quantum::ir	Canonical quantum semantic boundary
QEC	Error-correction realization
ZQN	Noise-aware quantum realization
optimization	Semantics-preserving transformations
scheduling	Resource/timing realization
hardware	Target capability and realization
runtime	Artifact compatibility and execution
interoperability	External representation compatibility
tests	Executable conformance



---

228. Dependency order

The compatibility implementation SHOULD be developed in this order:

1. language version primitives
        ↓
2. version parsing
        ↓
3. version comparison
        ↓
4. version ranges
        ↓
5. compatibility classifications
        ↓
6. capability compatibility
        ↓
7. dialect compatibility
        ↓
8. dependency compatibility
        ↓
9. source compatibility
        ↓
10. AST compatibility
        ↓
11. semantic compatibility
        ↓
12. IR compatibility
        ↓
13. artifact compatibility
        ↓
14. ABI compatibility
        ↓
15. runtime compatibility
        ↓
16. target compatibility
        ↓
17. migration/deprecation
        ↓
18. repository-wide compatibility tests

No downstream component should require reopening a completed foundational version primitive because the compatibility model was underspecified.


---

229. Integration graph

The intended integration graph is:

language-version.md
        │
        ▼
compatibility.md
        │
        ├──────────────┐
        ▼              ▼
grammar-authority   dialect system
        │              │
        ▼              ▼
syntax model       dialect versions
        │              │
        └──────┬───────┘
               ▼
             AST
               │
               ▼
       semantic analysis
               │
       ┌───────┼────────┐
       ▼       ▼        ▼
   classical quantum  hardware
      IR       IR        IR
               │
               ▼
        optimization
               │
               ▼
         scheduling
               │
               ▼
        target lowering
               │
       ┌───────┼─────────────┐
       ▼       ▼             ▼
     CPU      QPU           FPGA
       │       │             │
       └───────┼─────────────┘
               ▼
            runtime
               │
               ▼
          execution

No cycle is permitted:

grammar → IR → grammar
grammar → runtime → grammar
quantum grammar → hardware grammar → quantum grammar
runtime → grammar → runtime


---

230. Compatibility decision algorithm

A production implementation SHOULD conceptually perform:

Input:
    source/artifact
    declared language version
    dialect set
    dependencies
    IR versions
    target profile
    runtime profile

Step 1:
    Parse metadata.

Step 2:
    Validate version syntax.

Step 3:
    Resolve language compatibility.

Step 4:
    Resolve dialect compatibility.

Step 5:
    Resolve dependency compatibility.

Step 6:
    Validate semantic compatibility.

Step 7:
    Validate IR compatibility.

Step 8:
    Validate artifact compatibility.

Step 9:
    Discover target capabilities.

Step 10:
    Validate requirements.

Step 11:
    Validate constraints.

Step 12:
    Produce deterministic compatibility result.

Step 13:
    Proceed to compilation/execution only if required contracts are satisfied.


---

231. Compatibility result model

A compatibility result SHOULD conceptually contain:

CompatibilityResult {
    status
    language_status
    grammar_status
    dialect_status
    semantic_status
    ir_status
    artifact_status
    runtime_status
    target_status
    capability_status
    resource_status
    diagnostics
    migrations
}

The actual Rust type belongs to the compiler/semantic compatibility implementation.


---

232. Compatibility status

The compatibility result SHOULD distinguish:

Compatible
CompatibleWithMigration
ConditionallyCompatible
TargetIncompatible
ResourceIncompatible
DialectIncompatible
IRIncompatible
ArtifactIncompatible
LanguageIncompatible
Unknown

Unknown MUST NOT be treated as Compatible.


---

233. Migration result

When migration is possible, compatibility tooling SHOULD return:

required migration
migration source version
migration target version
affected constructs
semantic risk
automatable
manual steps


---

234. Compatibility and semantic hashing

If Zamani later introduces semantic hashes, the hash MUST be derived from canonical semantic representation rather than raw grammar formatting.

Therefore:

different formatting

must not necessarily produce:

different semantic identity


---

235. Compatibility and source formatting

These may change without changing semantic identity:

whitespace;

comments;

formatting;

equivalent syntactic sugar;

source file organization where semantics are preserved.



---

236. Compatibility and syntactic sugar

Syntactic sugar SHOULD lower to stable canonical semantic constructs.

This minimizes long-term compatibility burden.


---

237. Compatibility and canonical representation

The long-term compatibility strategy is:

many source forms
        ↓
canonical semantics
        ↓
canonical IR
        ↓
many target realizations

This is preferable to:

many source forms
        ↓
many independent semantic implementations
        ↓
many target-specific interpretations


---

238. Compatibility and quantum canonicalization

Quantum source forms SHOULD lower to canonical quantum semantics.

For example, aliases for a mathematically equivalent operation MAY share the same canonical representation.

This prevents quantum syntax from becoming coupled to a finite hardware gate catalog.


---

239. Compatibility and HDL canonicalization

HDL source forms SHOULD lower to canonical hardware semantics.

Technology-specific constructs should remain dialect/target-specific unless standardized as Zamani semantics.


---

240. Compatibility and future target realization

A future backend SHOULD be able to consume stable Zamani semantics without requiring historical source programs to be rewritten merely because the machine model changed.

This is a central POCO-REAF compatibility requirement.


---

241. Compatibility guarantee boundaries

Zamani guarantees:

stable semantic interpretation

where the declared compatibility contract applies.

Zamani does NOT guarantee:

identical performance
identical latency
identical energy consumption
identical hardware mapping
identical machine code
identical topology
identical physical implementation

unless a stronger contract explicitly requires those properties.


---

242. Performance compatibility

Performance is normally non-semantic.

A compiler optimization may improve or reduce performance while remaining language-compatible.

Performance requirements become compatibility-relevant only when explicitly declared as constraints.


---

243. Latency compatibility

Latency is non-semantic unless declared as a correctness requirement.

For example:

must respond within T

is a constraint.

A target that cannot satisfy it is deployment-incompatible.


---

244. Energy compatibility

Energy requirements are constraints unless explicitly part of semantics.

Targets MAY differ in energy usage while remaining language-compatible.


---

245. Reliability compatibility

Reliability constraints are compatibility-relevant when explicitly declared.

A target failing a mandatory reliability requirement MUST be rejected.


---

246. Portability classification

Every program/package SHOULD be classifiable as:

Universal
Capability-Portable
Profile-Portable
Target-Constrained
Deployment-Specific
Non-Portable

This classification should derive from explicit requirements rather than assumptions.


---

247. Universal programs

A universal program expresses semantics without mandatory target-specific assumptions.

It MAY execute on any target capable of realizing its requirements.


---

248. Capability-portable programs

A capability-portable program requires a capability class but not a specific implementation.

Example:

requires quantum computation

rather than:

requires vendor/device X


---

249. Target-constrained programs

A target-constrained program explicitly requires a target property.

This is valid but reduces portability.

The compatibility metadata MUST expose this fact.


---

250. Deployment-specific programs

Deployment-specific programs may depend on:

exact hardware;

exact network;

exact address;

exact device;

exact deployment topology.


Such requirements MUST remain explicit.


---

251. Compatibility and source rewriting

A target change MUST NOT require source rewriting merely because:

CPU → GPU
CPU → FPGA
CPU → QPU
QPU → simulator
single node → cluster
local → cloud

The semantic source program should remain stable whenever the target can satisfy its requirements.


---

252. Compatibility and fallback

Fallback mechanisms MUST preserve semantics.

If:

GPU unavailable

then:

CPU fallback

is compatible only when the CPU realization is semantically equivalent and the program permits fallback.


---

253. Compatibility and graceful failure

When no compatible realization exists, the system MUST provide a deterministic failure.

Example:

Valid Zamani program.
Target lacks required capability.

The diagnostic should state that the program is valid but cannot be realized on that target.


---

254. Compatibility and future machines

Future hardware MUST be allowed to expose new capabilities without requiring old language semantics to be rewritten.

This is achieved through:

capabilities
profiles
dialects
resource requirements
IR extensions
target lowering


---

255. Compatibility and obsolete machines

A historical target may eventually cease to support newer language features.

This does not invalidate the language.

It means:

target no longer satisfies required capability

The language remains semantically stable.


---

256. Compatibility and obsolete compilers

Old compilers may stop supporting new language versions.

This does not make the new language version invalid.

It means:

compiler capability < program requirement


---

257. Compatibility and compiler replacement

A new compiler implementation MUST be permitted to replace an old compiler without forcing source migration when it supports the same language contract.


---

258. Compatibility and parser replacement

A parser implementation MAY be completely replaced if it preserves:

accepted language;

AST contract;

diagnostics contract where required;

semantic meaning.



---

259. Compatibility and lexer replacement

Likewise, a lexer MAY be rewritten if it preserves the lexical contract.

Token implementation details are not automatically language semantics.


---

260. Compatibility and grammar refactoring

Grammar refactoring SHOULD be encouraged when it improves:

maintainability;

determinism;

modularity;

parser performance;

diagnostics;

correctness.


It MUST NOT unnecessarily create a language version break.


---

261. Compatibility and file splitting

Splitting:

Zamani.g4

into modular grammar files does not inherently change language compatibility.

The resulting composed grammar MUST accept the same language for the same compatibility profile.


---

262. Compatibility and generated grammar

Generated grammar files MUST identify their source/provenance where practical.

Generated artifacts MUST NOT become independent authorities.


---

263. Compatibility and documentation drift

Documentation drift is a compatibility risk.

CI SHOULD detect mismatches between:

specification
grammar
AST
lexer
parser
tests
examples


---

264. Compatibility CI

Production CI SHOULD include:

grammar conformance
lexer conformance
parser conformance
AST conformance
semantic conformance
IR conformance
compatibility matrix
dialect compatibility
artifact compatibility
round-trip tests
scalability tests
hard-coding audit


---

265. Release gate

A Zamani release MUST NOT be considered production-ready until:

compatibility metadata is updated;

compatibility tests pass;

migration policy is updated;

deprecated features are identified;

removed features are documented;

grammar/specification consistency is verified;

IR compatibility is verified;

target compatibility is verified where applicable.



---

266. Release provenance

A release SHOULD record:

language version
grammar revision
specification revision
compiler version
IR versions
dialect versions
artifact format versions

This allows future systems to determine compatibility without guessing.


---

267. Compatibility audit

Before every breaking release, perform an audit covering:

syntax
tokens
keywords
AST
types
effects
capabilities
resources
classical semantics
quantum semantics
HDL semantics
hardware semantics
distributed semantics
AI/data semantics
networking
security
IR
ABI
runtime
interop
dialects
tooling
examples
tests


---

268. Compatibility review questions

Every compatibility review MUST answer:

1. Does existing valid source remain valid?


2. If not, why?


3. Does existing source retain its meaning?


4. Does the AST preserve required information?


5. Does canonical IR preserve meaning?


6. Does quantum::ir remain canonical?


7. Does hardware remain downstream?


8. Does target selection remain separate from language semantics?


9. Are resource limits still external?


10. Are any fixed machine assumptions introduced?


11. Is migration required?


12. Are diagnostics sufficient?


13. Are tests present?


14. Is documentation synchronized?




---

269. Final compatibility invariant

The ultimate invariant is:

Stable Zamani semantics
        ↓
versioned compatibility contract
        ↓
canonical representation
        ↓
capability/resource negotiation
        ↓
target realization

not:

Zamani syntax
        ↓
current machine assumptions
        ↓
permanent language limitation


---

270. Final POCO-REAF contract

Zamani compatibility MUST support:

PROGRAM ONCE
    ↓
stable source semantics
    ↓
COMPILE ONCE
    ↓
portable semantic/compiled artifact
    ↓
RUN EVERYWHERE
    ↓
capability-driven realization
    ↓
RUN ANYWHERE
    ↓
local / embedded / distributed / cloud / quantum / classical / hybrid
    ↓
RUN FOREVER
    ↓
versioned semantics + migration + extensible representations

The guarantee is semantic continuity, not perpetual execution of one immutable native binary on every physically possible future architecture.


---

271. Final scalability contract

The compatibility system MUST support:

tiny
→ small
→ medium
→ large
→ massive
→ distributed
→ heterogeneous
→ quantum
→ hybrid
→ future

without introducing language-level ceilings merely because current implementations are smaller.

The only limits that may remain are:

physical resources
implementation resources
representation limits
explicit program constraints
target constraints
deployment constraints

and these MUST remain distinguishable from language compatibility.


---

272. Final ownership contract

When this file is complete:

language-version.md

can define what a Zamani version means.

compatibility.md

defines what can interoperate with that version.

grammar-authority.md

defines which representation is authoritative.

syntax-model.md

defines what syntax means structurally.

semantic-model.md

defines what programs mean.

compilation-model.md

defines how semantics become compilable representations.

execution-model.md

defines how compiled semantics execute.

scalability-model.md

defines how the language scales without artificial ceilings.

poco-reaf.md

defines the Program Once, Compile Once, Run Everywhere, Anywhere, Forever model.

This file connects those contracts without duplicating their ownership.


---

273. Production-readiness checklist

grammar/specification/compatibility.md is production-ready only when all of the following are true:

Authority

[ ] Compatibility authority is explicit.

[ ] No competing compatibility specification exists.

[ ] Version policy is delegated to language-version.md.

[ ] Grammar authority is delegated to grammar-authority.md.


Language

[ ] Source compatibility is defined.

[ ] Semantic compatibility is defined.

[ ] Grammar compatibility is defined.

[ ] AST compatibility is defined.

[ ] Type compatibility is defined.

[ ] Effect compatibility is defined.

[ ] Capability compatibility is defined.

[ ] Resource compatibility is defined.


Quantum

[ ] Quantum compatibility is defined.

[ ] quantum::ir remains canonical.

[ ] No fixed qubit compatibility limit exists.

[ ] Logical/physical qubits remain distinct.

[ ] QEC remains separately owned.

[ ] ZQN remains separately owned.

[ ] Backend capability failures are distinguishable from language errors.


Classical

[ ] Classical semantics are protected.

[ ] Numerical compatibility is defined.

[ ] Concurrency compatibility is defined.

[ ] Memory compatibility is defined.


HDL/hardware

[ ] HDL compatibility is defined.

[ ] Hardware compatibility is defined.

[ ] Technology-specific properties remain target-specific.

[ ] No fixed hardware size is encoded.


Distributed

[ ] Distributed semantics are protected.

[ ] Physical topology remains separate.

[ ] Resource failures are distinguishable from language failures.


AI/data

[ ] AI/ML compatibility is defined.

[ ] Tensor semantics remain independent of hardware tensor capacity.

[ ] Data compatibility is defined.


Networking/security

[ ] Network compatibility is defined.

[ ] Security semantics are protected.

[ ] ABI and protocol versions are separate from language versions.


Toolchain

[ ] Compiler version is separate.

[ ] Runtime version is separate.

[ ] IR version is separate.

[ ] Artifact version is separate.

[ ] Dialect versions are separate.

[ ] Target versions are separate.


POCO-REAF

[ ] Program Once is defined.

[ ] Compile Once is defined realistically.

[ ] Run Everywhere is capability-based.

[ ] Run Anywhere is deployment-independent.

[ ] Run Forever is version/migration based.

[ ] Native binary universality is not falsely promised.


Scalability

[ ] No arbitrary machine-size compatibility ceilings exist.

[ ] No fixed qubit maximum exists.

[ ] No fixed CPU/core/thread maximum exists.

[ ] No fixed GPU/FPGA/ASIC maximum exists.

[ ] No fixed cluster/network maximum exists.

[ ] No fixed tensor/data maximum exists where semantics are inherently scalable.


Safety

[ ] Rust 1.97/1.97.1 is supported.

[ ] No unsafe implementation is required.

[ ] Malformed compatibility metadata cannot cause undefined behavior.

[ ] Compatibility resolution is deterministic.

[ ] Unknown compatibility is not treated as success.


Testing

[ ] Positive tests exist.

[ ] Negative tests exist.

[ ] Boundary tests exist.

[ ] Cross-domain tests exist.

[ ] Scalability tests exist.

[ ] Determinism tests exist.

[ ] Round-trip tests exist.

[ ] Compatibility matrix tests exist.

[ ] Migration tests exist.

[ ] Hard-coding audit exists.



---

274. Final principle

Zamani compatibility MUST preserve the following architectural law:

> Zamani describes computation, intent, capabilities, constraints, and semantics—not arbitrary limitations of the machine currently available.



Therefore:

One Program
     ↓
One Stable Meaning
     ↓
Many Compatible Representations
     ↓
Many IR Realizations
     ↓
Many Targets
     ↓
Many Hardware Configurations
     ↓
Many Execution Environments
     ↓
Future Platforms

The compatibility system exists to make that promise technically enforceable.

The ultimate invariant is:

SOURCE SEMANTICS
        ≠
CURRENT HARDWARE

SOURCE SEMANTICS
        +
EXPLICIT REQUIREMENTS
        +
CAPABILITY NEGOTIATION
        +
VERSIONED COMPATIBILITY
        =
PORTABLE ZAMANI COMPUTATION

And therefore:

Zamani
    =
semantic stability
    +
explicit compatibility
    +
hardware independence
    +
canonical IR boundaries
    +
capability/resource negotiation
    +
deterministic evolution
    +
unbounded scalability within available resources
    +
POCO-REAF

From Atom to Everywhere.
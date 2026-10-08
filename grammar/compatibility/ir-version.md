Zamani IR Versioning Specification

Path: "grammar/compatibility/ir-version.md"
Status: Normative — Production IR Versioning Contract
Language: Zamani
Repository: "github.com/Benwellonedge28/Zamani/"
Grammar technology: ANTLR4
Rust baseline: Rust 1.97 or later
Rust edition: 2021
Rust safety requirement: Production Zamani implementation MUST use safe Rust. Rust "unsafe" MUST NOT be required or used.
Primary portability objective: "Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever" (POCO-REAF)

---

1. Purpose

This document defines the normative versioning contract for Zamani Intermediate Representations.

It establishes how IR contracts evolve while preserving, where promised:

- source semantics;
- canonical semantic meaning;
- type meaning;
- effect meaning;
- capability requirements;
- resource requirements;
- classical computation;
- quantum computation;
- "quantum::ir";
- HDL/hardware intent;
- hybrid computation;
- concurrency;
- distributed computation;
- data and AI computation;
- networking;
- interoperability;
- provenance;
- deterministic compilation;
- reproducibility;
- migration;
- artifact compatibility;
- downstream lowering compatibility.

This document specifically answers:

«What does an IR version mean, when must it change, how is compatibility determined, how are old IR representations migrated, and how is canonical IR protected from accidental semantic or hardware coupling?»

The central rule is:

«IR versioning describes the contract of a canonical IR representation. It does not define the language itself, a target machine, a backend, or physical hardware.»

IR evolution MUST preserve the distinction between:

source language
      ↓
domain-neutral AST
      ↓
semantic model
      ↓
canonical IR
      ↓
optimization
      ↓
lowering
      ↓
routing / scheduling / resilience / QEC / ZQN
      ↓
HAL
      ↓
target realization

---

2. Scope

This specification applies to:

- canonical classical IR;
- canonical quantum IR;
- "quantum::ir";
- hybrid IR boundaries;
- HDL/hardware semantic IR where such a canonical boundary is defined;
- extensible operation representations;
- type representations;
- control-flow representations;
- regions and blocks;
- values;
- operands and results;
- attributes;
- effects;
- capabilities;
- resource requirements;
- contracts where represented in IR;
- policies where represented in IR;
- provenance;
- source mappings;
- deterministic identity;
- serialization;
- hashing;
- compatibility metadata;
- dialect identity;
- IR migrations;
- IR validation;
- IR tooling;
- IR artifacts.

It does not define:

- source-language syntax;
- lexical tokens;
- parser rules;
- AST structure;
- individual language semantics;
- compiler optimization algorithms;
- routing algorithms;
- scheduling algorithms;
- QEC algorithms;
- ZQN implementation;
- HAL implementation;
- runtime implementation;
- target-specific instruction sets;
- physical hardware limits;
- calibration;
- device inventories.

Those remain owned by their respective repository contracts.

---

3. Ownership

3.1 This file owns

"grammar/compatibility/ir-version.md" owns:

- IR version identity;
- IR version domains;
- IR version numbering;
- IR compatibility classes;
- IR version bump rules;
- IR representation compatibility;
- IR schema evolution;
- IR serialization compatibility policy;
- IR canonicalization compatibility;
- IR migration requirements;
- IR version negotiation;
- IR version validation;
- IR version metadata;
- IR-version provenance requirements;
- IR-version diagnostics;
- IR-version testing requirements;
- compatibility rules between canonical IR revisions.

3.2 This file does not own

This file does NOT own:

- the semantic meaning of language constructs;
- source grammar;
- token definitions;
- AST definitions;
- semantic type definitions;
- effect definitions;
- resource definitions;
- capability definitions;
- quantum operation semantics;
- HDL semantics;
- backend algorithms;
- target discovery;
- physical resource allocation;
- physical qubit mapping;
- routing;
- scheduling;
- QEC;
- ZQN;
- runtime behavior.

---

4. Repository Authority

The repository authority relationship is:

grammar/DESIGN.md
        │
        ▼
grammar/specification/
        │
        ├── language meaning
        ├── language version
        ├── semantic model
        ├── compilation model
        ├── portability
        └── scalability
        │
        ▼
grammar/spec/
        │
        ├── compatibility
        ├── semantics
        ├── types
        ├── effects
        ├── resources
        ├── domains
        └── diagnostics
        │
        ▼
grammar/Zamani.g4
        │
        ▼
lexer / parser
        │
        ▼
src/ast/
        │
        ▼
semantic analysis
        │
        ▼
canonical semantic representation
        │
        ├───────────────┬─────────────────┐
        ▼               ▼                 ▼
   classical       quantum::ir       HDL/hardware
      IR              canonical           IR
        │               │                 │
        └───────────────┼─────────────────┘
                        ▼
                optimization/lowering
                        │
              ┌─────────┼─────────┐
              ▼         ▼         ▼
           routing   scheduling resilience
                                  │
                             ┌────┼────┐
                             ▼    ▼    ▼
                            QEC  ZQN  other
                                  │
                                  ▼
                                 HAL
                                  │
                                  ▼
                         target realization

IR versioning MUST fit into this hierarchy.

It MUST NOT create another authority layer.

---

5. Relationship to Other Compatibility Files

File| Owns
"grammar/compatibility/versions.md"| Repository-wide version taxonomy and compatibility policy
"grammar/compatibility/language-version.md"| Language-version compatibility
"grammar/compatibility/grammar-version.md"| Grammar-version compatibility
"grammar/compatibility/AST-version.md"| AST-version compatibility
"grammar/compatibility/semantic-version.md"| Semantic-contract versioning
"grammar/compatibility/ir-version.md"| IR-contract versioning
"grammar/compatibility/ir-conformance.md"| Source/semantic → IR conformance
"grammar/compatibility/compatibility-matrix.md"| Cross-layer compatibility relationships
"grammar/compatibility/migrations.md"| Migration procedure
"grammar/compatibility/deprecated.md"| Deprecation lifecycle
"grammar/compatibility/feature-gates.md"| Feature lifecycle/gating
"grammar/spec/compatibility.md"| Normative compatibility concepts
"grammar/spec/versioning.md"| Cross-layer versioning model
"grammar/specification/poco-reaf.md"| POCO-REAF
"grammar/specification/scalability-model.md"| Scalability semantics

No file listed above may redefine the meaning of an IR version established here.

Conversely, this file MUST NOT redefine their owned contracts.

---

6. IR Versioning Is Independent

Zamani contains independently evolving version domains.

At minimum:

Language
Grammar
Lexer
Parser
AST
Semantic Model
Type System
Effect System
Resource Model
Capability Model
Policy Model
Provenance Model
Classical IR
quantum::ir
HDL/HW IR
Dialect
Compiler
Artifact
ABI
Runtime
Target

These versions MUST remain distinct.

A change in one domain MUST NOT automatically imply a change in every other domain.

For example:

new GPU backend

does not automatically require:

language version change
IR version change

Likewise:

new QPU topology

does not automatically require:

quantum::ir version change

unless the canonical quantum IR contract itself changes.

---

7. Canonical IR Domains

An IR version MUST identify the IR domain it versions.

The canonical repository model includes, at minimum:

classical.ir
quantum.ir

where:

quantum.ir

corresponds to the repository's canonical:

src/quantum/ir/

The exact registry of canonical IR domains belongs to the repository compatibility registry.

An implementation MUST NOT infer an IR domain from a bare version number.

This is invalid as a complete identity:

3.0.0

This is a valid conceptual identity:

quantum.ir@3.0.0

and:

classical.ir@1.4.0

---

8. Canonical Quantum IR

The repository establishes:

src/quantum/ir/

as the canonical quantum IR boundary.

Therefore:

«"quantum::ir" MUST remain the canonical quantum semantic IR.»

No second permanent quantum IR may be introduced merely because another compiler phase needs a convenient representation.

The canonical path remains:

Zamani source
    ↓
lexer
    ↓
parser
    ↓
domain-neutral AST
    ↓
semantic quantum model
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
resilience / QEC
    ↓
ZQN
    ↓
HAL
    ↓
target

Temporary internal representations MAY exist.

They MUST have:

- explicit ownership;
- explicit lifetime;
- explicit conversion;
- deterministic lowering;
- no claim to canonical IR authority.

---

9. No Duplicate Canonical IR

The following MUST NOT become independent canonical IR authorities:

grammar::quantum::ir
frontend::quantum::ir
parser::quantum::ir
compiler::quantum::ir
backend::quantum::ir
dialect::quantum::ir

when they duplicate the semantic role of "quantum::ir".

Compatibility wrappers MAY exist.

They MUST delegate to or convert into the canonical representation.

They MUST NOT create semantically divergent duplicate types.

---

10. IR Version Identity

Every independently versioned IR contract MUST contain:

ir-domain
ir-version
compatibility-status

The canonical version form is:

<IR-DOMAIN>@<MAJOR>.<MINOR>.<PATCH>

Examples:

classical.ir@1.0.0
quantum.ir@1.0.0

These examples illustrate format only.

Actual repository versions MUST be maintained in the repository's version registry.

---

11. Semantic Version Format

The canonical IR version uses:

MAJOR.MINOR.PATCH

Optional prerelease metadata MAY be used:

MAJOR.MINOR.PATCH-PRERELEASE

Optional build metadata MAY be used:

MAJOR.MINOR.PATCH+BUILD

Both may be combined:

MAJOR.MINOR.PATCH-PRERELEASE+BUILD

Build metadata MUST NOT conceal an incompatible IR contract.

Prerelease identifiers MUST NOT be treated as stable compatibility guarantees.

---

12. Meaning of MAJOR

Increment "MAJOR" when the stable IR contract becomes incompatible.

Examples include:

- changing operation semantics incompatibly;
- removing required IR fields;
- changing operand interpretation incompatibly;
- changing result interpretation incompatibly;
- changing type semantics incompatibly;
- changing region semantics incompatibly;
- changing control-flow meaning incompatibly;
- changing effect meaning incompatibly;
- changing capability meaning incompatibly;
- changing resource requirement meaning incompatibly;
- changing quantum-state semantics incompatibly;
- changing measurement semantics incompatibly;
- changing logical/physical identity semantics incompatibly;
- changing HDL intent semantics incompatibly;
- changing canonicalization rules in a way that changes semantic identity;
- changing required serialization meaning incompatibly.

A major IR version change MUST identify the affected IR domain.

---

13. Meaning of MINOR

Increment "MINOR" when the IR contract is extended without breaking existing stable consumers.

Examples:

- adding optional metadata;
- adding a new extensible operation;
- adding an optional attribute;
- adding a new capability representation;
- adding a new resource expression form;
- adding an optional provenance field;
- adding a new operation namespace;
- adding a new domain extension that existing consumers can safely ignore according to the contract.

A minor version MUST NOT change the established meaning of existing stable IR.

---

14. Meaning of PATCH

Increment "PATCH" for compatible corrections.

Examples:

- correcting documentation;
- correcting metadata interpretation without semantic change;
- correcting serialization defects while preserving valid semantics;
- correcting diagnostics;
- correcting canonical encoding bugs where semantic identity remains unchanged;
- correcting implementation conformance.

A patch version MUST NOT introduce a semantic break.

---

15. Representation Compatibility vs Semantic Compatibility

These concepts MUST remain separate.

Two IR versions may have different physical representations while preserving semantic compatibility.

For example:

IR v1:
Operation {
    ...
}

may evolve to:

IR v2:
Operation {
    ...
    provenance: ...
}

If the new field is optional and existing meaning remains unchanged, this MAY be a minor-compatible change.

Conversely, changing:

measurement

from one semantic interpretation to another is a semantic break even if the serialized structure looks almost identical.

Therefore:

binary similarity

does not establish:

semantic compatibility

---

16. Compatibility Classes

IR compatibility MUST be classified explicitly.

Class| Meaning
"IR-C0"| Fully compatible; no migration required
"IR-C1"| Compatible representation extension
"IR-C2"| Compatible representation with optional normalization
"IR-C3"| Deterministic migration available
"IR-C4"| Tool-assisted migration required
"IR-C5"| Manual semantic migration required
"IR-C6"| Artifact-only incompatibility
"IR-C7"| Dialect-specific incompatibility
"IR-C8"| Target/backend incompatibility
"IR-C9"| Semantically incompatible IR

Compatibility MUST NOT be represented only by a boolean.

The diagnostic system MUST retain the reason and affected domain.

---

17. Source Compatibility Does Not Imply IR Compatibility

A source program may remain valid while its generated IR changes.

For example:

source: compatible
AST: compatible
semantic model: compatible
IR: incompatible

This can be legitimate if a migration exists or if the IR is intentionally regenerated.

Therefore source compatibility and IR compatibility MUST be evaluated separately.

---

18. IR Compatibility Does Not Imply Target Compatibility

A valid canonical IR may not be executable on every target.

For example:

source
  ↓
semantic analysis
  ↓
quantum::ir

may succeed while:

target capability resolution

fails because the selected target lacks a required capability.

This MUST be reported as a target/capability/resource issue.

It MUST NOT be misreported as:

IR version incompatibility

unless the IR contract itself cannot be interpreted.

---

19. IR Version vs Hardware Capability

IR versions MUST NOT encode physical hardware capacity.

The following are NOT IR-version numbers:

- CPU generation;
- GPU generation;
- FPGA generation;
- ASIC generation;
- QPU generation;
- physical qubit count;
- node count;
- device count;
- memory capacity;
- register width;
- vector width;
- tensor rank;
- network size.

A target may provide:

capability("quantum.measurement")

or:

capability("tensor.compute")

without changing the canonical IR version.

---

20. POCO-REAF Requirement

IR versioning exists to support long-term semantic portability.

POCO-REAF requires:

Program Once
      ↓
Compile Once
      ↓
Run Everywhere
      ↓
Anywhere
      ↓
Forever

This does NOT mean that one serialized target-specific binary must execute unchanged on every physical machine.

It means that portable program meaning remains stable while realization may change.

Therefore:

stable source meaning
        ↓
stable semantic meaning
        ↓
compatible canonical IR
        ↓
target-specific realization

A new hardware target MUST NOT require changing the source program merely because the target has different physical resources.

---

21. Resource Independence

The canonical IR MUST distinguish:

program requirement

from:

available resource

from:

target realization

For example:

requires qubits >= n

is a program requirement.

The available number of physical qubits belongs to target discovery.

The mapping:

logical q0 → physical q17

belongs to downstream realization.

IR versioning MUST preserve this separation.

---

22. No Universal Capacity Constants

This specification MUST NOT establish language or IR limits such as:

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
MAX_REGISTER_WIDTH
MAX_VECTOR_WIDTH
MAX_TENSOR_RANK
MAX_NETWORK_SIZE
MAX_DEVICE_COUNT
MAX_PROGRAM_SIZE
MAX_OPERATION_COUNT
MAX_BLOCK_COUNT
MAX_REGION_COUNT

A compiler MAY impose operational policies for:

- security;
- denial-of-service protection;
- memory protection;
- compilation budgets;
- execution budgets;
- implementation limits.

Those MUST be represented as policy/resource constraints.

They MUST NOT be presented as universal language or IR ceilings.

---

23. Arbitrary-Scale IR

IR representations MUST be scalable according to the actual program and available implementation resources.

A program may require:

n qubits

where "n" is determined symbolically or dynamically.

The IR MUST NOT require a fixed maximum solely because the current Rust implementation uses a particular collection capacity.

The correct model is:

semantic quantity
        ↓
representation
        ↓
implementation resources

not:

implementation representation
        ↓
language maximum

---

24. Symbolic Values

Where the language semantics permit symbolic values, IR MUST preserve those values until the semantic phase responsible for resolution.

For example:

Tensor<T, Shape>

must not silently become:

Tensor<T, fixed-shape>

unless specialization has explicitly established that shape.

Likewise:

qubits >= required_qubits

must remain a symbolic/resource requirement when "required_qubits" is not statically known.

---

25. Numeric Preservation

IR versioning MUST preserve numeric meaning.

A fixed-width implementation representation MUST NOT silently become a semantic limit.

If an AST or frontend currently represents numeric literals using fixed-width Rust forms such as:

i64
f64

the implementation MUST distinguish:

source numeric value

from:

frontend representation

and:

target numeric representation

If a source value cannot be represented by a particular implementation or target, the compiler MUST emit a structured diagnostic.

It MUST NOT silently:

- truncate;
- wrap;
- saturate;
- round;
- reinterpret;
- overflow into another value.

---

26. IR Operation Identity

An operation SHOULD be identified by a stable semantic identity rather than a closed compiler enum.

Conceptually:

Operation {
    name
    namespace
    operands
    results
    parameters
    attributes
    modifiers
    effects
    capabilities
    resource_requirements
    regions
    provenance
}

The exact executable Rust structure is owned by the IR implementation.

This document defines the compatibility obligations of that structure.

---

27. Extensible Operations

The IR MUST support extensible semantic operations.

A new operation MUST NOT require modification of the universal IR schema merely because its name is new.

This is especially important for:

- quantum operations;
- accelerators;
- tensor operations;
- scientific operations;
- cryptographic operations;
- future computational models;
- dialect extensions;
- vendor-neutral extensions;
- future hardware.

Known operations MAY receive optimized internal representations.

The canonical semantic model MUST remain capable of representing valid operations that are not hard-coded into a closed enumeration.

---

28. Quantum Operation Versioning

Quantum IR version changes MUST be evaluated against:

- operation identity;
- namespace;
- targets;
- controls;
- parameters;
- conditions;
- results;
- modifiers;
- attributes;
- effects;
- capabilities;
- resource requirements;
- measurement semantics;
- state semantics;
- provenance.

A new quantum operation does not automatically require a "quantum::ir" major version.

If the existing extensible operation contract already represents it, it MAY be a dialect or capability addition.

---

29. No Closed Quantum Operation Universe

The canonical quantum IR MUST NOT define the universe of quantum operations as a permanently closed list such as:

H
X
Y
Z
CNOT
SWAP
...

These may be recognized operations.

They MUST NOT prevent representation of:

custom.operation
vendor.operation
parameterized.operation
future.operation

when the applicable semantic contract permits them.

This protects long-term extensibility.

---

30. Logical and Physical Quantum Resources

The canonical IR MUST preserve:

logical quantum identity

as distinct from:

physical quantum identity

For example:

logical q0

may later be mapped to:

physical qubit 17

or any other target-selected physical resource.

That mapping MUST NOT alter the source semantic identity.

IR version changes MUST NOT collapse these two concepts.

---

31. Quantum Resource Policies

The repository contains "QuantumIrLimits" and related resource-policy structures.

These remain implementation/policy boundaries.

They MUST NOT be interpreted as universal language limits.

For example:

QuantumIrLimits

may constrain one compiler invocation.

It MUST NOT redefine:

maximum number of qubits supported by Zamani

The distinction is:

IR semantics
      ≠
IR validation policy
      ≠
compiler budget
      ≠
target capability
      ≠
physical capacity

---

32. Unbounded Policies

Where an IR policy supports an unbounded state, "unbounded()" means:

«No finite ceiling is imposed by that particular policy object.»

It does NOT mean:

«Infinite physical resources exist.»

Actual execution may still be bounded by:

- memory;
- address space;
- allocator capacity;
- process limits;
- compiler resources;
- target resources;
- runtime resources;
- network resources;
- physical constraints.

The language remains scalable without making an impossible physical claim.

---

33. Types

IR types MUST represent resolved semantic types.

The normal pipeline is:

source TypeExpr
       ↓
type resolution
       ↓
semantic Type
       ↓
canonical IR type

IR consumers MUST NOT be required to reinterpret source-level type syntax.

IR version changes involving types MUST preserve:

- type identity;
- generic parameters;
- bounds;
- associated information;
- ownership;
- resource semantics;
- capability semantics;
- quantum semantics;
- hardware intent.

---

34. Generic Types

Generic parameters that affect semantics MUST be preserved.

Examples include:

Tensor<T, Shape>
Memory<T, Size>
Vector<T, N>
Qubit<N>

A compiler MUST NOT replace symbolic parameters with arbitrary constants merely for implementation convenience.

Specialization MAY replace symbolic parameters when specialization is explicitly requested or semantically justified.

Such specialization MUST be represented in provenance.

---

35. Dependent and Symbolic Information

If a semantic property depends on a value, the IR MUST preserve the dependency until the responsible phase resolves it.

For example:

allocate qubits[n]

must not silently become:

allocate qubits[fixed]

unless the fixed value is established by:

- source semantics;
- verified compile-time evaluation;
- explicit specialization;
- explicit deployment configuration.

---

36. Effects

IR MUST preserve semantic effects whenever removing or changing the effect could change program meaning.

Examples include:

io
mutation
allocation
randomness
measurement
network
foreign
native
distributed
simulation
learning
adaptation
reflection
code_generation

An optimizer MUST NOT remove an effect merely because the associated result appears unused.

IR version changes affecting effect representation MUST be classified semantically.

---

37. Capabilities

Capabilities MUST remain distinct from operations and physical devices.

Examples include:

quantum.measurement
quantum.mid_circuit_measurement
gpu.compute
tensor.compute
distributed.communication
persistent.storage
native.execute
foreign.call
simulation.execute

The capability name is semantic metadata.

The target layer determines which realization provides that capability.

Changing the inventory of available target capabilities does not automatically change the IR version.

---

38. Resource Requirements

Canonical IR resource requirements MUST preserve:

- resource identity;
- quantity;
- unit;
- symbolic expressions;
- minimum/maximum semantics;
- hard/soft classification;
- relation to capabilities;
- provenance;
- diagnostic context.

For example:

requires memory >= required_memory

must remain a requirement.

It must not become a hidden allocation decision.

---

39. Policies

Policies represented in IR MUST remain distinguishable from semantic program meaning unless the language specification explicitly defines the policy as semantic.

Policies may constrain:

- compilation;
- optimization;
- resource use;
- target selection;
- security;
- adaptation;
- simulation;
- deployment;
- reproducibility.

Changing a compiler policy MUST NOT automatically require an IR semantic-version change.

Changing the specified meaning of a policy-bearing IR construct MAY require an IR version change.

---

40. Contracts

Where contracts reach IR, the representation MUST preserve their semantic role.

Relevant constructs include:

requires
ensures
invariant
assume
guarantee
property

The IR MUST distinguish:

assertion
assumption
requirement
guarantee
property

where those distinctions affect semantics.

A verifier MAY lower these constructs into target-specific verification mechanisms.

The canonical IR MUST preserve their meaning until the responsible validation/lowering phase.

---

41. Provenance

IR versioning MUST be provenance-aware.

Where provenance is enabled or required, an IR artifact SHOULD preserve:

source identity
source span
language version
grammar version
AST version
semantic version
IR domain
IR version
dialect versions
compiler version
transformation identity
transformation reason
parent artifact identity
evidence
verification status
target capability context

The exact universal provenance schema belongs to the repository provenance specification.

This file only establishes the IR-versioning obligation.

---

42. Provenance of IR Migration

When IR is migrated:

old IR
   ↓
migration
   ↓
new IR

the resulting artifact MUST be able to identify:

source IR version
target IR version
migration identity
migration tool/version
migration status
semantic verification status

Where provenance is enabled.

A migration MUST NOT erase the identity of the original IR artifact.

---

43. Deterministic IR

Equivalent semantic input MUST produce deterministic canonical IR whenever deterministic compilation is requested by the applicable compilation contract.

Determinism includes, where promised:

- stable operation identity;
- stable ordering;
- stable attribute serialization;
- stable canonicalization;
- stable symbol identity;
- stable provenance ordering;
- stable hashing;
- stable version metadata.

Nondeterminism MUST be explicit when permitted by the semantic contract.

---

44. Canonicalization

Canonicalization may normalize structurally equivalent representations.

For example:

equivalent attribute ordering

may be normalized.

However:

semantic distinction

must never be erased merely to obtain a canonical encoding.

Canonicalization MUST be:

- deterministic;
- documented;
- versioned when its compatibility contract changes;
- testable.

A canonicalization change that changes semantic identity requires compatibility review.

---

45. IR Hashing

If an IR artifact is hashed, the hash MUST be defined over a canonical representation.

The hash contract MUST specify whether it includes:

- semantic content;
- provenance;
- source mappings;
- compiler metadata;
- build metadata;
- target context.

Build-specific information MUST NOT accidentally make semantically identical IR artifacts incomparable unless the hash contract explicitly defines that behavior.

If a hash algorithm changes, the hash-domain/version identity MUST make the distinction explicit.

---

46. Serialization

An IR serialization format MUST identify:

IR domain
IR version
serialization format
serialization version
dialect versions where applicable

A serialized IR artifact MUST NOT rely on a reader guessing its version from field presence.

Unknown mandatory fields MUST result in an explicit compatibility diagnostic.

Unknown optional fields MAY be ignored only when the serialization contract explicitly guarantees forward tolerance.

---

47. Forward Compatibility

An older IR reader MUST NOT claim to understand a newer IR version merely because it can deserialize some fields.

The reader may accept a newer representation only when:

1. the compatibility contract explicitly permits it;
2. all required semantics are understood;
3. unknown information cannot alter meaning;
4. validation succeeds.

Otherwise it MUST reject the artifact with a structured diagnostic.

---

48. Backward Compatibility

A newer IR reader MAY accept an older IR version when the compatibility matrix explicitly declares that relationship.

Acceptance may be:

direct
normalized
migrated
dialect-assisted

The reader MUST NOT silently reinterpret incompatible old semantics.

---

49. Unknown IR Versions

Unknown IR versions MUST fail closed.

For example:

quantum.ir@99.0.0

must not be treated as:

quantum.ir@1.0.0

merely because the implementation has no other option.

The diagnostic SHOULD identify:

- IR domain;
- requested version;
- supported versions;
- compatibility class;
- migration availability.

---

50. Version Negotiation

When two components exchange IR, they MUST negotiate compatibility using:

IR domain
version
supported ranges
required features
dialects
serialization formats
capabilities
migration availability

Negotiation MUST be deterministic.

A successful negotiation MUST produce an explicit selected contract.

The implementation MUST NOT select a version based only on:

- newest available;
- oldest available;
- lexical ordering;
- target hardware preference.

The selection algorithm belongs to the compatibility implementation, but its outcome MUST be observable and reproducible where reproducibility is required.

---

51. IR Version Ranges

A consumer MAY specify a supported IR range.

Conceptually:

quantum.ir >= A,<B

The exact range syntax belongs to the repository's compatibility/versioning implementation.

Version ranges MUST NOT be confused with resource expressions.

For example:

quantum.ir >= 2,<3

is an IR compatibility constraint.

It is not:

requires qubits >= 2

---

52. IR Feature Compatibility

IR version alone is not sufficient to determine all compatibility.

An IR artifact MAY additionally require:

feature identity
dialect identity
capability
effect support
resource support

Therefore a complete compatibility decision is conceptually:

IR domain
+
IR version
+
required features
+
dialects
+
semantic contract
+
consumer support

A compatible version MUST NOT be treated as sufficient when a required feature is unavailable.

---

53. Dialect Versioning

Dialect versions remain distinct from canonical IR versions.

For example:

quantum.ir@2.0.0
quantum.vendor.example@4.1.0

may coexist.

Adding a dialect does not automatically require changing the canonical IR version if the canonical IR already provides the required extension boundary.

Dialect-specific semantics MUST NOT silently become universal canonical IR semantics.

---

54. Classical IR

The classical IR MUST remain capable of representing, where specified:

- scalar values;
- arbitrary supported numeric values;
- structured values;
- arrays;
- vectors;
- matrices;
- tensors;
- functions;
- calls;
- control flow;
- memory;
- concurrency;
- parallelism;
- symbolic operations;
- data processing;
- distributed computation;
- accelerator intent.

No classical IR version may impose an arbitrary universal machine scale.

---

55. Quantum IR

"quantum::ir" MUST support the repository's canonical quantum semantic requirements, including applicable:

- quantum operations;
- states;
- qubits;
- logical qubits;
- physical qubits;
- measurement;
- reset;
- controlled operations;
- parameterized operations;
- dynamic circuits;
- classical feed-forward;
- channels;
- noise;
- observables;
- error-correction intent;
- pulse intent;
- quantum-classical interaction;
- quantum learning;
- quantum inference;
- uncertainty;
- provenance;
- resource requirements;
- capabilities.

Not every target must implement every semantic feature.

Target support is a downstream concern.

---

56. HDL/Hardware IR

Where HDL/hardware constructs reach canonical IR, the representation MUST preserve hardware intent rather than silently becoming a fixed machine description.

It may represent:

- signals;
- computation;
- storage intent;
- timing requirements;
- concurrency;
- interfaces;
- verification properties;
- synthesis intent;
- resource requirements;
- constraints;
- topology requirements.

It MUST NOT silently introduce universal fixed:

- bus widths;
- register widths;
- device counts;
- FPGA resource counts;
- memory capacities;
- timing grids.

Those belong to target realization.

---

57. Hybrid IR

Hybrid computation MUST remain representable without requiring separate incompatible IR universes.

The semantic relationship may include:

classical → quantum
quantum → classical
classical control → quantum operation
measurement → classical decision
AI → quantum
quantum → AI
CPU → accelerator
host → device

The representation MUST preserve the semantic boundary.

A hybrid IR extension MUST NOT duplicate "quantum::ir".

---

58. Concurrency and Distributed IR

IR versioning MUST preserve semantic concurrency information where relevant:

- tasks;
- actors;
- messages;
- channels;
- synchronization;
- ordering;
- parallel regions;
- dependencies;
- distributed communication;
- consistency requirements.

An implementation may lower these to different schedulers.

Changing a scheduler does not automatically require an IR version change.

Changing the specified concurrency semantics does.

---

59. Data and AI IR

Where data/AI semantics are represented in canonical IR, the IR may represent:

- models;
- inference;
- learning;
- adaptation;
- reasoning;
- knowledge;
- uncertainty;
- evidence;
- provenance;
- tensor operations;
- symbolic operations;
- neural-symbolic composition;
- agent semantics.

Application-specific concepts SHOULD remain represented through:

- operations;
- dialects;
- libraries;
- capabilities;
- metadata;
- policies.

They MUST NOT force unnecessary universal IR schema growth.

---

60. Adaptive Computation

An adaptive operation must preserve its semantic conditions.

For example:

adapt

may depend on:

- policy;
- capability;
- effect;
- resource;
- authorization;
- provenance.

The IR MUST preserve those dependencies when they affect semantics.

Adaptive execution MUST NOT be represented as unrestricted silent mutation of the IR.

---

61. Simulation

Simulation intent may be represented in IR for:

- classical simulation;
- quantum simulation;
- hardware simulation;
- distributed simulation;
- AI simulation;
- fault simulation;
- performance simulation.

Simulation is an execution strategy.

It MUST NOT create a second source language or a second canonical IR.

---

62. FFI and ABI Boundaries

Foreign and native operations MUST preserve enough semantic information for compatibility analysis.

Where applicable, IR metadata should identify:

foreign
ABI
calling convention
external type
data layout
effect
capability
provenance

An ABI version change does not automatically imply an IR version change.

It becomes an IR compatibility issue only when the IR contract itself depends on the changed ABI semantics.

---

63. IR Migration

The migration architecture is:

old IR
   ↓
version detection
   ↓
compatibility classification
   ↓
migration
   ↓
validation
   ↓
canonical current IR
   ↓
semantic verification

Migration MUST converge on the current canonical semantic IR.

It MUST NOT preserve obsolete permanent IR universes merely for convenience.

---

64. Migration Classes

IR migrations use:

Class| Meaning
"IR-M0"| No migration
"IR-M1"| Transparent compatibility
"IR-M2"| Deterministic mechanical migration
"IR-M3"| Tool-assisted migration
"IR-M4"| Explicit developer migration
"IR-M5"| Semantic migration
"IR-M6"| Serialization/artifact migration
"IR-M7"| Dialect migration
"IR-M8"| Target/backend migration
"IR-M9"| Breaking migration

A migration MAY have different classifications for:

representation
serialization
semantics
dialect
artifact
target

These MUST NOT be collapsed into one classification.

---

65. Required IR Migration Record

Every non-trivial IR migration MUST define:

Migration ID:
IR Domain:
Source IR Version:
Target IR Version:

Source Representation:
Target Representation:

Migration Class:

Affected Semantic Domains:

Old Operation Model:
New Operation Model:

Old Type Model:
New Type Model:

Old Effect Model:
New Effect Model:

Old Capability Model:
New Capability Model:

Old Resource Model:
New Resource Model:

Old Quantum Representation:
New Quantum Representation:

Old HDL/HW Representation:
New HDL/HW Representation:

Semantic Preservation:
Semantic Changes:

Serialization Impact:
Hashing Impact:
Provenance Impact:

Automatic Transformation:
Tool-Assisted Transformation:
Manual Action:

Diagnostics:

Positive Tests:
Negative Tests:
Boundary Tests:
Scalability Tests:
Determinism Tests:
Round-Trip Tests:
Compatibility Tests:

Rollback / Recovery:

Affected Repository Files:
Downstream Consumers:

Completion Status:

A migration MUST NOT be considered production-complete while required fields remain unspecified.

---

66. Semantic Preservation

A migration is successful only if the specified semantic meaning is preserved, unless the migration is explicitly a semantic migration.

The fundamental invariant is:

old valid IR
      ↓
migration
      ↓
new IR

such that:

meaning(old IR)
=
meaning(new IR)

for all semantics promised by the compatibility contract.

Structural equality is not required.

Semantic preservation is required.

---

67. IR Round-Trip Requirement

Where serialization is reversible, the implementation SHOULD support:

IR
 ↓
serialize
 ↓
deserialize
 ↓
canonicalize
 ↓
IR

with semantic equivalence.

If byte-for-byte identity is promised, canonical serialization MUST define that requirement explicitly.

A round-trip MUST NOT silently lose:

- operations;
- operands;
- results;
- parameters;
- types;
- effects;
- capabilities;
- resources;
- provenance;
- quantum meaning;
- HDL intent;
- contracts.

---

68. IR Validation

Every IR version MUST have a corresponding validator.

Validation MUST cover, where applicable:

- schema;
- operation structure;
- type correctness;
- operand/result consistency;
- region/block correctness;
- control-flow correctness;
- effect consistency;
- capability consistency;
- resource-expression correctness;
- quantum invariants;
- provenance consistency;
- dialect compatibility;
- version compatibility.

Validation MUST distinguish:

malformed IR

from:

valid IR but unsupported target

---

69. Error Classification

IR diagnostics MUST distinguish at least:

IR_VERSION_UNKNOWN
IR_VERSION_UNSUPPORTED
IR_VERSION_INCOMPATIBLE
IR_MIGRATION_REQUIRED
IR_MIGRATION_UNAVAILABLE
IR_SCHEMA_INVALID
IR_SEMANTICS_INVALID
IR_DIALECT_UNSUPPORTED
IR_FEATURE_UNSUPPORTED
IR_CAPABILITY_UNAVAILABLE
IR_RESOURCE_UNSATISFIED
IR_TARGET_UNSUPPORTED
IR_SERIALIZATION_INVALID
IR_PROVENANCE_INVALID

The exact diagnostic identifiers belong to the repository diagnostic system.

This document establishes their semantic distinction.

---

70. No Silent Downgrade

An implementation MUST NOT silently convert:

new IR

to:

older IR

if the conversion loses semantics.

A downgrade is permitted only if:

1. the target IR can represent all required semantics;
2. the conversion is specified;
3. the conversion is validated;
4. provenance records the downgrade;
5. compatibility is explicitly declared.

Otherwise the compiler MUST reject the operation.

---

71. No Silent Upgrade

An older IR MUST NOT be labeled as a newer IR merely because a new reader can consume it.

An upgrade requires either:

representation migration

or:

explicit compatibility declaration

The resulting version metadata MUST accurately identify the representation actually present.

---

72. IR and Optimization

Optimization MUST operate within the IR semantic contract.

An optimization may change:

IR structure

without changing:

IR semantic meaning

when legal.

For example:

operation A
operation B

may be transformed into:

operation C

provided semantic equivalence is established.

Optimization MUST NOT be used as an excuse to change the declared IR version after every optimization pass.

---

73. Optimization Provenance

Where provenance is required, transformations MUST be identifiable.

Conceptually:

IR-v1
  ↓
optimization.pass
  ↓
IR-v2 representation

The transformation record SHOULD identify:

- pass identity;
- pass version;
- input IR identity;
- output IR identity;
- reason;
- legality status;
- verification status.

---

74. Lowering Boundary

Lowering transforms canonical semantic IR into a representation closer to a target.

The direction is:

canonical IR
      ↓
lowering
      ↓
target-oriented representation

Lowering MUST NOT retroactively redefine the canonical IR.

A target may require:

- decomposition;
- instruction selection;
- layout;
- memory placement;
- routing;
- scheduling;
- device mapping.

Those are downstream transformations.

---

75. Routing Boundary

Routing MUST remain downstream of canonical quantum semantics.

The canonical IR represents logical relationships.

Routing may determine:

logical q0 → physical resource

The routing result MUST NOT be mistaken for the canonical semantic identity.

A routing algorithm change does not automatically require a "quantum::ir" version change.

---

76. Scheduling Boundary

Scheduling may determine:

- execution order;
- timing;
- resource conflicts;
- synchronization;
- device slots;
- durations.

Canonical IR MUST NOT hard-code target-specific timing tables.

A scheduling implementation change does not automatically require an IR version change.

---

77. Resilience and Error Correction Boundary

Resilience, recovery, fault handling, and error correction remain downstream.

The canonical IR may represent:

resilience intent
error-correction intent
fault policy
recovery requirement

but MUST NOT become the owner of:

- physical calibration;
- recovery algorithms;
- hardware-specific fault tables;
- physical error rates unless explicitly semantic;
- target-specific correction implementation.

A QEC implementation change does not automatically require a quantum IR version change.

---

78. Target Independence

An IR artifact MUST remain interpretable without requiring the reader to assume a specific:

- CPU;
- GPU;
- FPGA;
- ASIC;
- accelerator;
- QPU;
- operating system;
- machine size;
- node count;
- device count.

Target-specific metadata MAY exist when explicitly classified as target context.

It MUST NOT silently become portable semantic meaning.

---

79. Resource Availability

A target may have insufficient resources to realize a valid IR.

For example:

valid IR
+
required capability
+
required resources

may result in:

target cannot satisfy requirement

This is not an IR version error.

The compiler MUST distinguish:

IR invalid

from:

IR valid but target infeasible

---

80. Arbitrarily Large Resource Scales

The IR architecture MUST scale from:

tiny computation

to:

very large computation

subject only to:

- program-defined requirements;
- representation feasibility;
- compiler resources;
- runtime resources;
- target resources;
- physical constraints;
- explicit security/resource policies.

The IR contract MUST NOT define an artificial upper bound merely because a current implementation has finite resources.

---

81. Current Implementation Limits

A Rust implementation may have finite:

- address space;
- allocation capacity;
- stack/heap resources;
- integer representations;
- collection sizes;
- parser resources;
- compilation time;
- runtime resources.

Such limitations MUST be treated as implementation/resource constraints.

They MUST NOT automatically become:

language semantic limit

or:

IR semantic limit

unless the specification explicitly establishes such a limit.

---

82. Safe Rust Requirement

The reference implementation of IR versioning MUST use safe Rust.

The implementation MUST NOT require:

unsafe

for:

- version parsing;
- version comparison;
- compatibility checking;
- IR validation;
- migration;
- serialization;
- canonicalization;
- hashing;
- diagnostics;
- provenance.

The implementation SHOULD prefer:

- strongly typed enums;
- validated newtypes;
- "Result";
- "Option";
- checked arithmetic;
- ownership and borrowing;
- standard collections;
- deterministic algorithms.

Rust implementation details remain outside the grammar semantics, but the production safety requirement is mandatory for repository implementation.

---

83. Rust Version Independence

Zamani IR versioning MUST remain independent from the Rust toolchain version.

For example:

Zamani IR: quantum.ir@X.Y.Z
Rust: 1.97+

A Rust compiler update does not automatically constitute an IR version change.

Conversely, an IR version change does not automatically require a Rust version change.

The repository's build configuration and CI own exact toolchain enforcement.

---

84. IR Registry

The repository SHOULD maintain one machine-readable canonical registry for actual IR versions.

The registry SHOULD contain:

domain
version
status
owner
specification
implementation
serialization
migration
conformance
dialects
compatibility
deprecation

No individual grammar file should invent the current IR version independently.

This document defines the rules.

The registry defines the current values.

---

85. Version Status

Every IR version MUST have a lifecycle state.

Recommended states:

PROPOSED
DESIGNED
EXPERIMENTAL
IMPLEMENTED
STABLE
SUPPORTED
DEPRECATED
REMOVED

A version MUST NOT be marked "STABLE" solely because:

- its Rust type exists;
- serialization works;
- a grammar file mentions it.

Stable status requires complete conformance.

---

86. IR Version Lifecycle

The lifecycle is:

PROPOSED
    ↓
DESIGNED
    ↓
EXPERIMENTAL
    ↓
IMPLEMENTED
    ↓
TESTED
    ↓
STABLE
    ↓
DEPRECATED
    ↓
REMOVED

Promotion requires:

- specification;
- implementation;
- validation;
- compatibility classification;
- migration strategy;
- tests;
- deterministic behavior where promised;
- scalability audit;
- hard-coding audit;
- provenance audit;
- safe-Rust audit.

---

87. Deprecation

An IR version or representation MUST NOT be removed immediately after replacement.

Deprecation should provide:

- replacement version;
- compatibility window;
- migration procedure;
- diagnostic;
- removal policy.

The deprecation lifecycle is owned by:

grammar/compatibility/deprecated.md

This file defines how IR version compatibility participates in that lifecycle.

---

88. Feature Gates

IR features may be gated independently of IR versions.

For example:

quantum.dynamic-circuit
quantum.error-correction
tensor.compute
simulation
adaptive.execution

A feature gate MUST NOT be used to hide an incompatible IR schema change.

If a feature changes the IR contract incompatibly, the appropriate IR compatibility/version policy still applies.

---

89. Compatibility Matrix Integration

Every stable IR version MUST have an entry in:

grammar/compatibility/compatibility-matrix.md

The entry MUST identify compatibility with:

- language versions;
- AST versions;
- semantic versions;
- dialect versions;
- compiler versions where relevant;
- artifact versions;
- ABI versions where relevant;
- runtime versions where relevant.

The matrix MUST distinguish:

direct compatibility
migration compatibility
conditional compatibility
target incompatibility
unsupported

---

90. IR Conformance Integration

This document works with:

grammar/compatibility/ir-conformance.md

The division of responsibility is:

ir-version.md
    ↓
"What version does this IR contract have?"

while:

ir-conformance.md
    ↓
"Does source/semantic meaning conform to this IR?"

"ir-version.md" MUST NOT duplicate the full source-to-IR conformance specification.

"ir-conformance.md" MUST NOT redefine version numbering.

---

91. AST Integration

The compatibility boundary is:

AST version
      ↓
semantic model
      ↓
IR version

An AST change does not automatically require an IR version change if semantic meaning remains unchanged.

For example:

AST structure changed
semantic meaning unchanged
canonical IR unchanged

may be:

AST compatibility change
IR unchanged

Conversely:

AST unchanged
semantic meaning changed
IR meaning changed

may require both semantic and IR version changes.

---

92. Semantic Version Integration

The semantic-versioning contract determines whether semantic meaning changed.

The IR-versioning contract determines whether the canonical IR representation/contract changed.

The relationship is:

semantic change analysis
        ↓
IR impact analysis

not:

IR version change
        ↓
automatic semantic version change

Every IR version change MUST explicitly state whether semantic compatibility was affected.

---

93. Grammar Integration

Grammar changes must not be inferred from IR changes.

A new IR representation does not automatically require new Zamani syntax.

Likewise:

grammar reorganization

does not automatically require an IR version change.

The determining question is:

«Did the canonical semantic contract represented by the IR change?»

---

94. Compilation Integration

The compilation subsystem under:

grammar/compile/

must treat IR version as a compilation contract, not as a target identity.

Relevant existing compilation concerns include:

intent.g4
target.g4
target-selection.g4
specialization.g4
optimization.g4
lowering.g4
artifacts.g4
reproducibility.g4
deterministic-builds.g4
cross-compilation.g4
deployment.g4

The integration is:

compile intent
      ↓
semantic analysis
      ↓
IR version selection
      ↓
canonical IR
      ↓
optimization/lowering

Compilation intent MUST NOT hard-code one universal IR version as the language's permanent identity.

---

95. Specialization

Specialization may produce an IR variant optimized for known:

- types;
- values;
- capabilities;
- resources;
- target characteristics.

Specialization MUST preserve provenance.

A specialized IR is not automatically a new canonical IR domain.

The compiler MUST identify whether the specialized representation is:

canonical

or:

temporary/target-oriented

---

96. Reproducibility

When reproducible compilation is requested, the selected IR version MUST be part of the reproducibility identity.

A reproducible compilation record SHOULD identify:

source identity
language version
grammar version
AST version
semantic version
IR domain
IR version
dialect versions
compiler version
compiler configuration
relevant feature gates
relevant policy
relevant target context

The exact reproducibility contract remains owned by the compilation/specification subsystems.

---

97. Artifact Integration

A serialized or packaged artifact containing canonical IR MUST identify:

IR domain
IR version
serialization version

An artifact MAY contain multiple compatible IR forms when explicitly declared.

If multiple IR forms are present, exactly one MUST be designated canonical for the artifact's declared semantic stage.

---

98. Artifact vs IR Version

Artifact version and IR version MUST remain separate.

For example:

artifact@5.0.0
quantum.ir@3.0.0

An artifact packaging change does not necessarily change IR semantics.

An IR semantic change does not necessarily require an artifact-format major version if the artifact container explicitly supports the new IR version.

---

99. ABI and Runtime Integration

IR version compatibility MUST be evaluated before assuming ABI/runtime compatibility.

The relationship is:

IR
 ↓
lowering
 ↓
ABI
 ↓
runtime
 ↓
target

A runtime update does not automatically require an IR version change.

An ABI change does not automatically require an IR version change.

However, if the canonical IR embeds semantic ABI guarantees, the IR compatibility impact MUST be assessed.

---

100. Target Compatibility

Target support is determined downstream using:

capabilities
resources
constraints
policies
topology
runtime facilities

The IR version MUST NOT encode:

target = specific physical device

as its semantic identity.

A target may support:

quantum.ir@X.Y.Z

while another target supports the same IR version with different capabilities.

---

101. Capability Negotiation

The compatibility decision should conceptually be:

IR version
    +
IR features
    +
dialects
    +
semantic requirements
    +
target capabilities
    +
resource availability
    +
policy

This is what permits one program to scale across different target sizes.

---

102. Target Failure vs IR Failure

The compiler MUST distinguish:

IR failure

The artifact cannot be interpreted under the declared IR contract.

from:

Target failure

The artifact is valid, but the selected target cannot satisfy its requirements.

from:

Resource failure

The current execution/compilation context lacks required resources.

from:

Policy failure

The requested operation is disallowed by policy.

These MUST have different diagnostics.

---

103. Deterministic Version Resolution

When several compatible IR versions exist, resolution MUST be deterministic.

The implementation MUST NOT make the decision based on:

- hash-map iteration order;
- thread scheduling;
- unstable filesystem ordering;
- nondeterministic plugin discovery;
- current hardware order.

The chosen version MUST be reproducible where deterministic compilation is requested.

---

104. Version Comparison

IR versions MUST be compared structurally.

The implementation MUST NOT compare versions lexicographically as ordinary strings.

For example:

10.0.0

must not incorrectly sort before:

2.0.0

because of textual ordering.

The implementation SHOULD use validated version components.

---

105. Compatibility Comparison

Compatibility MUST evaluate:

domain
major
minor
patch
prerelease
feature set
dialects
serialization
semantic requirements

where applicable.

A version match alone does not guarantee semantic compatibility if additional required extensions are absent.

---

106. Version Aliases

Compatibility aliases MAY be provided for renamed IR domains or historical names.

An alias MUST:

- identify the canonical domain;
- be versioned;
- preserve provenance;
- have a deprecation lifecycle;
- avoid creating a competing canonical IR.

Historical aliases MUST NOT become permanent duplicate authorities.

---

107. Legacy IR

Legacy IR MAY be accepted during a compatibility window.

The preferred path is:

legacy IR
   ↓
migration
   ↓
canonical semantic IR

Legacy IR MUST NOT be propagated indefinitely through the entire backend stack merely because migration was inconvenient.

---

108. Migration Safety

An IR migration MUST fail rather than guess when information required for semantic preservation is missing.

For example:

old representation

containing insufficient information to distinguish:

semantic A

from:

semantic B

MUST NOT be automatically transformed into one arbitrary interpretation.

The migration tool MUST report an ambiguity.

---

109. Migration Idempotence

Where practical, a migration should be idempotent:

migrate(old)
    =
new

and:

migrate(new)
    =
new

A migration tool MUST NOT repeatedly modify an already-current canonical representation.

---

110. Migration Validation

Every migration MUST be followed by:

schema validation
semantic validation
type validation
effect validation
resource validation
capability validation
provenance validation
canonicalization

where applicable.

For quantum IR:

quantum invariants

MUST also be validated.

---

111. No Semantic Loss

A migration MUST NOT silently discard:

- operands;
- results;
- parameters;
- attributes;
- modifiers;
- types;
- effects;
- capabilities;
- resource requirements;
- regions;
- control flow;
- quantum information;
- HDL intent;
- provenance.

If information cannot be preserved, the migration MUST either:

1. preserve it as explicit metadata;
2. request developer action;
3. reject the migration.

---

112. IR Extension Model

The IR MUST remain extensible.

Extensions SHOULD use:

namespace
operation identity
attributes
dialects
capabilities
metadata

rather than modifying the universal IR schema for every new computational technology.

This is necessary for long-term evolution across:

- classical computing;
- quantum computing;
- HDL;
- accelerators;
- AI;
- tensor computation;
- distributed systems;
- networking;
- future computational substrates.

---

113. Namespace Stability

Operation namespaces MUST be stable enough for compatibility tooling to distinguish:

core.operation
quantum.operation
tensor.operation
vendor.operation
dialect.operation

Renaming a namespace is compatibility-sensitive.

The migration must preserve semantic operation identity.

---

114. Operation Renaming

A renamed operation is compatible when:

old operation identity
        ↓
canonical migration
        ↓
new operation identity

preserves semantics.

The migration record MUST identify:

old namespace/name
new namespace/name

and must not rely solely on textual replacement if context can alter meaning.

---

115. Operation Removal

Removing an operation from a stable IR is a major compatibility event unless:

- it was experimental;
- an explicit migration exists;
- the compatibility contract does not promise its preservation.

A removal MUST be recorded in:

deprecated.md
migrations.md
compatibility-matrix.md

where applicable.

---

116. Operation Attribute Evolution

Adding optional attributes is generally minor-compatible.

Changing the meaning of an existing attribute is compatibility-sensitive.

Removing a required attribute is a breaking change unless an equivalent semantic representation exists and migration is guaranteed.

Unknown attributes may only be ignored when the IR contract explicitly classifies them as non-semantic metadata.

---

117. Operand and Result Evolution

Changing:

- operand count;
- operand order;
- operand type;
- result count;
- result order;
- result type

is compatibility-sensitive.

Such changes MUST be classified based on semantic effect.

A parser or deserializer MUST NOT silently reorder operands or results unless the schema explicitly defines canonical ordering.

---

118. Region and Block Evolution

Changing:

- region count;
- block structure;
- terminator semantics;
- control-flow semantics;
- dominance requirements;
- region isolation rules

may be a major IR compatibility change.

A migration must preserve control-flow meaning.

---

119. Effect Evolution

Adding a new optional effect annotation may be minor-compatible.

Changing:

effect-free

to:

effectful

for an existing operation is semantic compatibility-sensitive.

An optimizer MUST respect the updated effect contract.

---

120. Capability Evolution

Adding a capability requirement to an existing operation is compatibility-sensitive because it can make previously realizable IR impossible on targets that previously satisfied it.

Therefore the compatibility analysis MUST distinguish:

representation-compatible

from:

target-realizability-compatible

A capability change MUST be explicitly documented.

---

121. Resource Requirement Evolution

Similarly, changing:

resource requirement

may preserve IR syntax while changing target feasibility.

For example:

minimum memory

changing from symbolic to a larger requirement is not necessarily an IR schema break, but it is a compatibility-sensitive semantic change.

It MUST be represented in the appropriate semantic/IR compatibility metadata.

---

122. Quantum Measurement Evolution

Any change to quantum measurement semantics is a major compatibility concern.

The following MUST be preserved unless explicitly changed by specification:

- measured observable;
- target;
- basis;
- classical result;
- conditional behavior;
- measurement ordering;
- collapse semantics where applicable;
- probabilistic meaning;
- provenance.

A representation change that preserves these semantics may be compatible.

A semantic change is not.

---

123. Quantum State Evolution

IR versions MUST preserve the distinction between:

logical state

and:

physical realization

A new backend representation of state does not automatically constitute a canonical quantum IR semantic change.

---

124. HDL Timing Evolution

Timing semantics are compatibility-sensitive.

The IR MUST distinguish:

semantic timing requirement

from:

target timing realization

A backend may choose different physical timing while preserving source/IR intent.

A change to the meaning of a timing constraint may require an IR version change.

---

125. Concurrency Ordering

If the IR promises ordering, synchronization, or happens-before relationships, those guarantees are semantic.

An optimization MUST preserve them.

Changing those guarantees requires compatibility analysis.

---

126. Memory Semantics

IR memory operations MUST preserve the language's specified:

- ownership;
- aliasing;
- mutation;
- lifetime;
- ordering;
- visibility;
- synchronization

semantics.

Changing the internal allocator does not automatically change the IR version.

Changing the semantic meaning of memory operations does.

---

127. Error and Result Semantics

If an IR operation has:

Result
Option
Error
exception
failure
recovery

semantics, those semantics MUST be versioned.

An operation MUST NOT silently change from:

failure is observable

to:

failure is ignored

without an explicit compatibility change.

---

128. Determinism and Nondeterminism

IR MUST distinguish:

deterministic computation

from:

nondeterministic computation

when the language semantics require that distinction.

Sources may include:

- randomness;
- concurrency;
- distributed execution;
- quantum measurement;
- external input.

A version change MUST NOT silently turn nondeterministic behavior into deterministic behavior or vice versa where observable semantics are affected.

---

129. Provenance of Compatibility Decisions

Where compatibility decisions affect compilation, provenance SHOULD record:

requested IR version
selected IR version
consumer-supported versions
compatibility decision
migration applied
migration version
target context
policy

This makes compatibility decisions auditable and reproducible.

---

130. Compatibility Diagnostics

Diagnostics SHOULD contain structured information such as:

code
IR domain
requested version
supported versions
feature
dialect
migration
source span
artifact identity
suggested action

Diagnostics MUST NOT claim:

unsupported hardware

when the actual issue is:

unsupported IR version

and vice versa.

---

131. Testing Requirements

Every IR version MUST have:

positive tests
negative tests
boundary tests
migration tests
round-trip tests
serialization tests
determinism tests
compatibility tests
scalability tests
cross-domain tests
provenance tests

Quantum IR additionally requires applicable:

quantum semantic tests
logical/physical identity tests
measurement tests
dynamic-circuit tests
resource tests

---

132. Positive Tests

Positive tests MUST demonstrate that valid IR can be:

- constructed;
- validated;
- serialized;
- deserialized;
- canonicalized;
- consumed by the expected compiler stage.

Tests must include:

- classical;
- quantum;
- hybrid;
- HDL/hardware;
- extensible operations;
- resource requirements;
- capabilities;
- provenance.

---

133. Negative Tests

Negative tests MUST verify rejection of:

- unknown major versions;
- unsupported versions;
- malformed version identifiers;
- incompatible operations;
- missing required fields;
- invalid types;
- invalid operands;
- invalid regions;
- invalid dialects;
- incompatible migrations;
- semantic loss;
- invalid provenance.

---

134. Boundary Tests

Boundary tests MUST cover:

AST → semantic model
semantic model → IR
IR → optimization
IR → lowering
IR → quantum::ir
IR → HDL/HW representation
IR → artifact
IR → runtime boundary

---

135. Scalability Tests

Scalability tests MUST NOT use artificial universal ceilings.

They should validate behavior across resource scales by parameterization.

Examples:

n = symbolic
n = small
n = larger
n = implementation-feasible large

The test harness itself may have finite execution budgets.

Those budgets belong to test infrastructure.

They MUST NOT become language or IR limits.

---

136. Cross-Domain Tests

At least one integration test MUST exercise:

classical computation
+
quantum computation
+
resource requirements
+
capabilities
+
effects
+
contracts
+
provenance
+
adaptive execution
+
parallelism
+
simulation
+
target selection

The test MUST verify that all domains converge on the canonical IR architecture rather than creating incompatible parallel IR systems.

---

137. Quantum Integration Tests

Quantum tests MUST verify:

source quantum construct
        ↓
AST
        ↓
semantic quantum representation
        ↓
quantum::ir

and must verify that:

logical identity

remains distinct from:

physical realization

through routing.

---

138. Determinism Tests

Given identical:

source
language version
compiler version
IR version
dialect versions
configuration
policy
target context

a deterministic build MUST produce equivalent canonical IR.

Where byte identity is promised, it MUST produce identical canonical bytes.

---

139. Hard-Coding Audit

Every IR implementation and compatibility test MUST be audited for accidental constants such as:

MAX_QUBITS
MAX_CPUS
MAX_GPUS
MAX_FPGAS
MAX_QPUS
MAX_NODES
MAX_MEMORY
MAX_THREADS
MAX_REGISTER_WIDTH
MAX_VECTOR_WIDTH
MAX_TENSOR_RANK
MAX_NETWORK_SIZE
MAX_DEVICE_COUNT

Any such constant must be classified as one of:

semantic constant
validation policy
security budget
implementation limit
test budget
target property

A constant MUST NOT masquerade as a universal language/IR capacity.

---

140. Resource-Policy Audit

Existing resource policy structures such as:

QuantumIrLimits

MUST be reviewed as policy objects.

Their semantics must remain:

validation/implementation boundary

rather than:

universal language capability

The distinction MUST be visible in implementation APIs and diagnostics.

---

141. Safe-Rust Audit

The IR versioning implementation MUST be checked for:

unsafe blocks
unsafe traits
unsafe functions
unsafe implementations

Production compatibility infrastructure MUST NOT require unsafe Rust.

CI SHOULD fail the relevant production crate/check when forbidden unsafe code is introduced.

---

142. Repository Integration Contract

The completed "ir-version.md" integrates with the following repository areas.

Specification

grammar/specification/compatibility.md
grammar/specification/poco-reaf.md
grammar/specification/scalability-model.md
grammar/specification/compilation-model.md
grammar/specification/semantic-model.md

These define language meaning and architectural intent.

Focused specifications

grammar/spec/compatibility.md
grammar/spec/versioning.md
grammar/spec/semantics.md
grammar/spec/resources.md
grammar/spec/effects.md
grammar/spec/quantum.md
grammar/spec/hdl.md
grammar/spec/hybrid.md

These provide semantic contracts consumed by IR.

Compatibility

grammar/compatibility/versions.md
grammar/compatibility/semantic-version.md
grammar/compatibility/AST-version.md
grammar/compatibility/grammar-version.md
grammar/compatibility/ir-conformance.md
grammar/compatibility/compatibility-matrix.md
grammar/compatibility/migrations.md
grammar/compatibility/deprecated.md
grammar/compatibility/feature-gates.md

Compilation

grammar/compile/intent.g4
grammar/compile/target.g4
grammar/compile/target-selection.g4
grammar/compile/specialization.g4
grammar/compile/optimization.g4
grammar/compile/lowering.g4
grammar/compile/reproducibility.g4
grammar/compile/deterministic-builds.g4
grammar/compile/artifacts.g4
grammar/compile/deployment.g4

Quantum

grammar/quantum/quantum.g4
grammar/quantum/operations.g4
grammar/quantum/quantum-capabilities.g4
grammar/quantum/quantum-resources.g4
grammar/quantum/resource-requirements.g4
grammar/quantum/measurement.g4
grammar/quantum/dynamic-circuits.g4
grammar/quantum/adaptive-quantum-execution.g4
grammar/quantum/quantum-learning.g4
grammar/quantum/quantum-inference.g4
grammar/quantum/uncertainty.g4
grammar/quantum/provenance.g4

Executable implementation

src/ast/
src/quantum/ir/
src/quantum/
src/compiler/
src/ir_gen.rs
src/ir_verify.rs
src/optimizer.rs
src/backend.rs
src/runtime/

---

143. Integration Ownership Table

Concern| Owner| "ir-version.md" role
Language meaning| "grammar/specification/"| Consume
Language version| "language-version.md"| Reference
Grammar version| "grammar-version.md"| Reference
AST version| "AST-version.md"| Reference
Semantic version| "semantic-version.md"| Reference
IR version| this file| Own
Source→IR conformance| "ir-conformance.md"| Reference
Migration procedure| "migrations.md"| Reference
Compatibility matrix| "compatibility-matrix.md"| Register relationship
Deprecation| "deprecated.md"| Participate
Feature gating| "feature-gates.md"| Participate
Classical IR| canonical IR implementation| Version contract
"quantum::ir"| "src/quantum/ir/"| Version contract
Routing| downstream subsystem| Boundary
Scheduling| downstream subsystem| Boundary
QEC| downstream subsystem| Boundary
ZQN| downstream subsystem| Boundary
HAL| downstream subsystem| Boundary
Target realization| downstream subsystem| Boundary

---

144. Required Implementation Data Model

The Rust implementation SHOULD represent IR version identity with validated types rather than arbitrary strings.

Conceptually:

struct IrVersion {
    major: u64,
    minor: u64,
    patch: u64,
    pre_release: Option<PreRelease>,
    build: Option<BuildMetadata>,
}

and:

struct IrDomain(String);

The actual implementation MAY use a more suitable validated representation.

The important requirements are:

- no unchecked parsing;
- no implicit coercion;
- no string-based semantic comparison;
- no unsafe code;
- deterministic ordering;
- explicit validation.

---

145. Required Compatibility Result

Compatibility checking SHOULD produce a structured result conceptually equivalent to:

Compatible
CompatibleWithMigration
CompatibleWithNormalization
DialectRequired
FeatureRequired
MigrationRequired
TargetUnsupported
ResourceUnsatisfied
CapabilityUnavailable
UnsupportedVersion
SemanticallyIncompatible
MalformedArtifact

A simple:

true / false

result is insufficient for production diagnostics.

---

146. Version Selection Algorithm

A production implementation SHOULD follow:

1. Parse requested IR identity.
2. Validate version syntax.
3. Identify IR domain.
4. Identify supported versions.
5. Evaluate direct compatibility.
6. Evaluate feature compatibility.
7. Evaluate dialect compatibility.
8. Evaluate migration availability.
9. Select the deterministic compatible contract.
10. Record the decision.
11. Migrate if necessary.
12. Validate resulting IR.
13. Continue compilation.

Failure at any stage MUST produce an appropriate structured diagnostic.

---

147. Compatibility Must Be Explicit

The implementation MUST NOT infer compatibility merely from:

same field names
same struct layout
same serialization keys
same compiler
same source version
same target

Compatibility is established by the version contract and semantic conformance.

---

148. Versioned Semantic Boundaries

Every canonical IR boundary SHOULD have a stable identity.

For example:

classical.ir
quantum.ir

The boundary identity MUST remain stable even when implementation internals change.

This allows:

implementation evolution

without:

semantic identity fragmentation

---

149. Compatibility of Internal Rust Types

Internal Rust types may change without changing the public IR version if:

- serialized representation remains compatible;
- semantic representation remains compatible;
- canonical behavior remains compatible;
- public contracts remain satisfied.

Conversely, changing Rust types may require an IR version change if the external canonical IR contract changes.

The Rust implementation structure MUST NOT be confused with IR identity.

---

150. Compiler Evolution

A compiler may support multiple IR versions.

For example:

Compiler
 ├── classical.ir@1.x
 ├── classical.ir@2.x
 ├── quantum.ir@1.x
 └── quantum.ir@2.x

The compiler MUST declare its supported set.

It MUST NOT silently reinterpret unsupported versions.

---

151. Multiple IR Versions in One Compiler

Supporting multiple IR versions is permitted.

However, the compiler SHOULD normalize internally toward one current canonical representation whenever possible:

old IR
   ↓
migration
   ↓
current canonical IR
   ↓
compiler pipeline

This prevents duplicated optimization and lowering architectures.

---

152. Compatibility Window

Each stable IR version SHOULD have an explicit compatibility window.

The window should define:

- supported readers;
- supported writers;
- migration availability;
- deprecation state;
- planned removal;
- supported serialization formats.

The actual duration is release-policy dependent.

---

153. Release Checklist

An IR version MUST NOT be released as stable until:

[ ] Specification complete
[ ] Ownership established
[ ] Version registered
[ ] IR schema implemented
[ ] Canonical domain established
[ ] Serialization defined
[ ] Deserialization validated
[ ] Canonicalization defined
[ ] Hashing contract defined where applicable
[ ] Provenance integrated
[ ] Diagnostics integrated
[ ] Compatibility matrix updated
[ ] Migration strategy defined
[ ] Deprecation strategy defined
[ ] Positive tests complete
[ ] Negative tests complete
[ ] Boundary tests complete
[ ] Migration tests complete
[ ] Round-trip tests complete
[ ] Determinism tests complete
[ ] Scalability tests complete
[ ] Quantum tests complete where applicable
[ ] Classical tests complete where applicable
[ ] HDL tests complete where applicable
[ ] Hybrid tests complete where applicable
[ ] Cross-domain tests complete
[ ] Hard-coding audit complete
[ ] Safe-Rust audit complete
[ ] No duplicate canonical IR
[ ] No hidden hardware limits
[ ] Documentation complete

---

154. Definition of Done for This File

"grammar/compatibility/ir-version.md" is complete when:

1. IR version identity is unambiguous.
2. IR version domains are distinct.
3. Major/minor/patch rules are defined.
4. IR compatibility classes are defined.
5. IR migration classes are defined.
6. Canonical "quantum::ir" ownership is explicit.
7. Duplicate canonical IR is prohibited.
8. Resource limits are separated from IR semantics.
9. Capabilities are separated from physical targets.
10. Source, AST, semantic, IR, artifact, ABI, runtime, and target versions remain distinct.
11. Serialization compatibility is defined.
12. Canonicalization compatibility is defined.
13. Hashing/provenance requirements are defined.
14. Unknown versions fail closed.
15. Compatibility decisions are deterministic.
16. Migration requirements are explicit.
17. Target incompatibility is distinguished from IR incompatibility.
18. POCO-REAF is preserved.
19. Arbitrary resource scale is supported conceptually.
20. No universal hardware ceilings are introduced.
21. Rust 1.97+ compatibility is explicit.
22. Safe Rust is mandatory.
23. The existing repository's IR conformance contract is integrated.
24. The compatibility matrix has an explicit integration point.
25. Migration/deprecation/version documents have non-overlapping ownership.
26. Quantum, classical, HDL, hybrid, distributed, data, and AI IR concerns converge on the canonical semantic architecture.
27. No application-specific concept is promoted into universal IR syntax merely because an application uses it.
28. Every compatibility decision is testable.
29. Every migration is provenance-preserving.
30. A future IR extension can be introduced without rewriting this document's architecture.

---

155. Final Architectural Invariant

The complete production invariant is:

Zamani source
      ↓
language contract
      ↓
grammar
      ↓
lexer/parser
      ↓
domain-neutral AST
      ↓
semantic model
      ↓
versioned canonical IR
      │
      ├───────────────┐
      ▼               ▼
classical IR      quantum::ir
      │               │
      └───────┬───────┘
              ▼
        optimization
              ↓
          lowering
              ↓
      routing/scheduling
              ↓
      resilience / QEC
              ↓
             ZQN
              ↓
             HAL
              ↓
       target realization

The invariant is:

«Version changes may change representation, but they MUST NOT silently change specified meaning.»

And:

«Target limitations, resource availability, hardware generations, compiler implementation limits, and backend constraints MUST NOT become artificial language or canonical-IR capacity limits.»

Therefore the intended scaling model remains:

small computation
      ↓
larger computation
      ↓
very large computation
      ↓
arbitrarily large computation

subject to the resources and capabilities actually available to the compiler/runtime/target, rather than a fixed capacity encoded into Zamani.

The canonical compatibility relationship is:

Language Version
      ↓
Semantic Version
      ↓
IR Domain + IR Version
      ↓
IR Feature/Dialect Contract
      ↓
Canonical IR
      ↓
Target Capability/Resource Resolution
      ↓
Target Realization

This preserves the architectural separation required for POCO-REAF while allowing Zamani's classical, quantum, HDL, hybrid, distributed, AI, data, networking, accelerator, and future computational domains to evolve independently without creating competing IR authorities.
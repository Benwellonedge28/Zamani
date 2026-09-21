Zamani Module System

Production Architecture, Ownership, Integration, Scalability, and Conformance Contract

Path: "grammar/modules/"
Language: Zamani
Grammar technology: ANTLR4
Rust baseline: Rust 1.97 / Rust 1.97.1
Rust safety: Safe Rust only; "unsafe" is prohibited
Portability objective: "Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF)"
Scope: Modules, imports, exports, namespaces, aliases, packages, dependencies, versioning, and module-specific attributes

---

1. Purpose

"grammar/modules/" defines the source-level module-system syntax contract for the Zamani programming language.

The module subsystem provides the language infrastructure required to organize arbitrarily large Zamani programs while allowing the same source language to contain and compose:

- classical computation;
- systems programming;
- embedded computation;
- parallel computation;
- distributed computation;
- HPC;
- quantum computation;
- hybrid quantum/classical computation;
- HDL;
- hardware/software co-design;
- accelerators;
- AI/ML;
- data processing;
- scientific computation;
- networking;
- cryptography;
- security;
- temporal computation;
- Sankofa concepts;
- nano computation;
- future computational domains.

The module subsystem is deliberately domain-neutral.

A new computational domain must not require a new module language.

A module is a source-level organization and compilation abstraction. It is not inherently a hardware resource, runtime object, process, deployment unit, package archive, or filesystem directory.

---

2. Non-Negotiable Production Principles

The module subsystem MUST satisfy all of the following.

2.1 One language

Modules must organize one Zamani language.

They must not create separate module systems for:

- quantum;
- classical;
- HDL;
- AI;
- distributed computing;
- hardware;
- networking;
- security;
- or future domains.

2.2 One lexical authority

The canonical ANTLR lexer is:

grammar/antlr/ZamaniLexer.g4

Module grammars consume its vocabulary.

They do not create a competing lexer vocabulary.

2.3 One parser composition architecture

The canonical ANTLR parser composition is:

grammar/antlr/ZamaniParser.g4

"grammar/Zamani.g4" remains the repository-level grammar authority/composition surface.

The two files must not evolve into competing complete parsers.

2.4 One canonical source-item boundary

Module bodies consume the repository's canonical source-item composition.

The module subsystem MUST NOT create another universal:

item

or equivalent competing root-item grammar.

The relevant canonical source-unit/compilation-unit infrastructure is responsible for assembling:

sourceItem

and integrating module declarations with the rest of the language.

2.5 One canonical name system

Identifiers, simple names, qualified names, and paths must have one canonical owner.

Module grammars consume those definitions.

They do not redefine them.

2.6 Syntax is not semantics

The module grammar recognizes syntax.

It does not:

- resolve symbols;
- resolve packages;
- solve dependencies;
- access registries;
- access the filesystem;
- access networks;
- inspect environment variables;
- select hardware;
- discover hardware;
- allocate memory;
- allocate qubits;
- construct IR;
- perform QEC;
- perform ZQN analysis;
- route;
- schedule;
- calibrate;
- execute programs.

2.7 No machine-size language limits

The module grammar must contain no language-level limits for:

- number of modules;
- module nesting;
- qualified-name segments;
- declarations per module;
- imports;
- exports;
- dependencies;
- packages;
- source units;
- compilation units;
- nodes;
- processes;
- threads;
- CPUs;
- cores;
- GPUs;
- FPGAs;
- ASICs;
- QPUs;
- qubits;
- memory;
- storage;
- accelerators;
- network links;
- tensor dimensions;
- vector widths;
- timelines;
- workers;
- replicas.

Practical parser/compiler/OS/resource limits are implementation resource policies, not Zamani language semantics.

2.8 Safe Rust only

The Rust implementation associated with the module subsystem MUST compile under Rust 1.97 / Rust 1.97.1 without requiring "unsafe".

The grammar files themselves contain no Rust actions or unsafe operations.

---

3. Repository Authority Model

The module subsystem follows the repository-wide authority chain:

grammar/DESIGN.md
        │
        ▼
grammar/specification/
        │
        ▼
grammar/spec/
        │
        ▼
canonical grammar architecture
        │
        ├── grammar/Zamani.g4
        └── grammar/antlr/ZamaniParser.g4
                │
                ▼
        grammar/antlr/ZamaniLexer.g4
                │
                ▼
        parser / frontend
                │
                ▼
        domain-neutral frontend AST
                │
                ▼
        semantic analysis
                │
                ├── module resolution
                ├── name resolution
                ├── visibility
                ├── package resolution
                ├── dependency resolution
                ├── version compatibility
                ├── feature compatibility
                └── domain semantics
                │
                ▼
        canonical semantic representations
                │
        ┌───────┼────────┐
        ▼       ▼        ▼
   classical quantum   HDL/
    semantic   ::ir    hardware
    model              semantic model
        │       │        │
        └───────┼────────┘
                ▼
          optimization
                │
                ▼
      routing / scheduling /
      resilience / QEC / ZQN
                │
                ▼
               HAL
                │
                ▼
        target realization

The module grammar exists only in the syntax portion of this architecture.

---

4. Existing Files — Do Not Rename

The production module directory retains the existing filenames:

grammar/modules/
├── README.md
├── modules.g4
├── imports.g4
├── exports.g4
├── visibility.g4
├── namespaces.g4
├── aliases.g4
├── packages.g4
├── dependencies.g4
├── versioning.g4
└── module-attributes.g4

No unnecessary rename is required.

Each file has one primary owner responsibility.

---

5. File Responsibility Matrix

File| Primary responsibility| Must not own
"modules.g4"| Module declarations and module bodies| Names, imports, exports, package semantics, IR
"imports.g4"| Import syntax| Resolution, package loading, filesystem/network
"exports.g4"| Export/re-export syntax| Symbol resolution, ABI, deployment
"visibility.g4"| Canonical visibility syntax| Visibility semantics
"namespaces.g4"| Namespace syntax| Symbol resolution or package identity
"aliases.g4"| Alias syntax| Alias semantic resolution
"packages.g4"| Package source syntax| Registry/package-manager behavior
"dependencies.g4"| Dependency declaration syntax| Dependency solving/installing
"versioning.g4"| Version syntax/contracts| Compatibility decisions
"module-attributes.g4"| Module-specific attribute syntax| General attribute semantics
"README.md"| Architecture/integration contract| Competing grammar definitions

---

6. "modules.g4"

Owns

"modules.g4" is the sole grammar owner of module declaration syntax.

It owns:

- module declaration headers;
- module names as wrappers around canonical qualified names;
- module attributes;
- module visibility;
- module body delimiters;
- module declaration termination;
- module-body composition through the canonical "sourceItem" boundary;
- compatibility wrappers explicitly retained for existing tooling.

Conceptually:

moduleDeclaration
    → module header
    → module name
    → module declaration tail

and:

moduleBody
    → {
          sourceItem*
      }

Does not own

"modules.g4" must not own:

- identifiers;
- qualified-name construction;
- import syntax;
- export syntax;
- namespace syntax;
- package syntax;
- dependency syntax;
- version syntax;
- general attributes;
- declarations;
- functions;
- types;
- expressions;
- statements;
- quantum operations;
- HDL operations;
- hardware operations;
- AI operations;
- resource semantics;
- capabilities;
- compilation;
- runtime behavior.

Required integration

"modules.g4" consumes:

- canonical lexer tokens;
- canonical qualified names;
- canonical visibility;
- canonical module attributes;
- canonical source items.

It must not define another universal source-item rule.

---

7. "imports.g4"

Owns

"imports.g4" owns source-level import syntax, including:

- import declarations;
- named imports;
- grouped imports;
- wildcard imports;
- import aliases;
- symbolic import sources;
- literal import sources;
- import-specifier lists;
- import source clauses.

Examples of syntax that may be represented structurally include:

import math::linear;
import math::linear::Matrix;
import math::linear::Matrix as Matrix;
import { Matrix, Vector } from math::linear;
import * from math::linear;
import * as math from math::linear;

The exact accepted syntax remains governed by the canonical specification and grammar.

Does not own

It must not:

- open files;
- access a package registry;
- access the network;
- download dependencies;
- load dynamic libraries;
- resolve symbols;
- solve dependencies;
- verify artifacts;
- discover hardware;
- select targets;
- execute code.

Integration

import syntax
    ↓
frontend AST import node
    ↓
module/package/name resolution
    ↓
dependency graph
    ↓
semantic validation

---

8. "exports.g4"

Owns

"exports.g4" owns:

- export declarations;
- export lists;
- exported names;
- re-exports;
- export aliases;
- wildcard exports where specified;
- export source clauses.

Does not own

It must not determine:

- whether a symbol exists;
- whether an imported symbol resolves;
- whether a symbol is visible;
- ABI layout;
- package publication;
- runtime loading;
- deployment.

Important distinction

Visibility and export are different concepts.

Visibility answers:

Who may access this declaration?

Export answers:

Which declarations are part of this module's externally exposed interface?

The semantic layer determines whether a particular combination is legal.

---

9. "visibility.g4"

Owns

"visibility.g4" is the canonical owner of visibility syntax.

Its vocabulary must be reusable by:

- modules;
- functions;
- types;
- declarations;
- interfaces;
- traits;
- classical constructs;
- quantum constructs;
- HDL constructs;
- hardware constructs;
- distributed constructs;
- future domains.

Critical invariant

There must not be independent visibility systems such as:

moduleVisibility
quantumVisibility
hardwareVisibility
functionVisibility

when they represent the same language concept.

All such constructs must consume the canonical visibility vocabulary.

Does not own

It does not decide semantic access rights.

Semantic analysis owns:

- scope;
- accessibility;
- visibility violations;
- re-export legality;
- module-interface rules.

---

10. "namespaces.g4"

Owns

"namespaces.g4" owns namespace declaration/reference syntax.

It provides:

- namespace declarations;
- namespace paths;
- namespace references;
- namespace aliases;
- namespace path lists where required.

Namespace paths consume the canonical qualified-name system.

Does not own

A namespace is not automatically:

- a module;
- a package;
- a filesystem path;
- a deployment identity;
- a runtime identity;
- a hardware namespace.

Those relationships are semantic decisions.

---

11. "aliases.g4"

Owns

"aliases.g4" owns alias syntax where aliases are part of the module system.

Aliases may apply to:

- imported names;
- imported modules;
- exported names;
- module references;
- package references;
- namespace references.

Does not own

It does not decide:

- whether two symbols are equivalent;
- whether an alias creates a new declaration;
- whether an alias is visible;
- whether an alias creates ambiguity;
- whether an alias crosses a package boundary;
- whether an alias is compatible with dependency resolution.

Those belong to semantic/name/module resolution.

Canonical AST requirement

Aliases must map to the repository's canonical name/reference representation.

The module subsystem must not create a second symbol model merely for aliases.

---

12. "packages.g4"

Owns

"packages.g4" owns package-related source syntax.

A package may provide source-level information concerning:

- package identity;
- package metadata;
- package declaration;
- package version information;
- package-level dependency declarations;
- package-facing source organization.

Does not own

It must not:

- access registries;
- download packages;
- install packages;
- modify the filesystem;
- resolve dependency graphs;
- verify signatures;
- select artifacts;
- select hardware;
- execute package code.

Package-management behavior belongs to tooling/compiler infrastructure.

---

13. "dependencies.g4"

Owns

"dependencies.g4" owns dependency declaration syntax.

It may express:

- dependency identity;
- dependency requirements;
- dependency sources;
- version requirements;
- dependency aliases;
- optional dependencies;
- feature requirements;
- capability requirements;
- structured dependency metadata.

The grammar may remain structurally open-ended so that new dependency metadata does not require new keywords.

Does not own

It must not perform:

- dependency solving;
- package retrieval;
- network access;
- filesystem access;
- installation;
- lockfile generation;
- artifact verification;
- compilation;
- runtime loading.

The semantic/toolchain layer builds and validates the dependency graph.

---

14. "versioning.g4"

Owns

"versioning.g4" owns syntax for version-related declarations and constraints.

It must distinguish the syntactic concepts of:

language version
module version
package version
dependency version requirement
feature version
dialect version
compatibility declaration
API compatibility
ABI compatibility
compiler compatibility
runtime compatibility
target compatibility

These are not automatically the same semantic object.

Does not own

It must not:

- decide whether versions are compatible;
- select dependency versions;
- perform migrations;
- install versions;
- contact registries;
- choose compiler backends.

Those decisions belong to compatibility, dependency, package, compiler, and toolchain systems.

---

15. "module-attributes.g4"

Owns

"module-attributes.g4" owns syntax specifically attached to module declarations.

It may structurally represent module metadata such as:

- module status;
- stability;
- experimental status;
- module-level contracts;
- module-level semantic annotations;
- language feature declarations.

Does not own

It does not define the meaning of attributes.

It must not:

- execute an attribute;
- select hardware;
- allocate resources;
- bypass semantic analysis;
- mutate compiler state;
- perform I/O.

General attributes remain owned by the canonical attribute infrastructure in "grammar/core/attributes.g4".

"module-attributes.g4" is a specialized syntax boundary, not a second general attribute language.

---

16. Canonical Name Integration

All module-system names must use the repository's canonical naming infrastructure.

Conceptually:

lexer
  ↓
identifier
  ↓
simple/name segment
  ↓
qualifiedName
  ↓
module/package/namespace/import/export/dependency references

For example:

quantum::algorithms::optimization

is a qualified name.

"modules.g4" may say:

moduleName
    : qualifiedName
    ;

but it must not redefine:

identifier
simpleName
nameSegment
qualifiedName

---

17. Module Name ≠ Filesystem Path

This distinction is mandatory.

The source:

module quantum::algorithms;

does not inherently mean:

filesystem/quantum/algorithms

It does not inherently mean:

package://quantum/algorithms

It does not inherently mean:

hardware://quantum/algorithms

It is a Zamani language-level identity.

A toolchain may later map that identity to:

- files;
- generated sources;
- archives;
- packages;
- registries;
- virtual sources;
- embedded sources;
- remote sources.

That mapping is outside the grammar.

This separation is necessary for POCO-REAF.

---

18. Module Identity Must Remain Distinct

The semantic architecture must distinguish:

module identity
namespace identity
package identity
source identity
artifact identity
deployment identity
runtime identity
hardware identity

The parser must preserve enough source information for the semantic layer to make these distinctions.

The parser must not collapse them prematurely.

---

19. Module Nesting and Scale

The language must not impose a maximum module depth.

These are all structurally valid forms:

module a;

module a::b;

module a::b::c;

module a::b::c::d::e::f::g;

and arbitrary additional qualified-name depth subject only to implementation resource availability.

There must be no:

MAX_MODULE_DEPTH
MAX_MODULE_SEGMENTS
MAX_MODULES

language constant.

ANTLR repetition and recursion express structure.

Toolchain resource budgets may constrain a particular compilation invocation, but such constraints must never become portable language semantics.

---

20. Module Bodies

A module body must use the canonical source-item architecture.

Conceptually:

module
    ↓
module body
    ↓
sourceItem*

The module grammar must not create a special module-only declaration universe.

Consequently, subject to semantic legality, a module can contain:

imports
exports
nested modules
functions
types
classical declarations
quantum declarations
hybrid declarations
HDL declarations
hardware declarations
AI declarations
data declarations
distributed declarations
network declarations
security declarations
resource declarations
effect declarations
future declarations

A new domain should be able to enter a module through the canonical source-item/declaration architecture without changing module syntax.

---

21. Module Composition Must Be Domain-Neutral

The module grammar must never contain alternatives such as:

quantumModule
gpuModule
cpuModule
fpgaModule
qpuModule
aiModule
networkModule

when those concepts are merely domain-specific module classifications.

A normal module can organize any domain.

For example:

module scientific.simulation {
    ...
}

may contain:

- classical numerical computation;
- tensor computation;
- quantum kernels;
- HDL/co-design declarations;
- distributed execution;
- resource requirements.

The module grammar does not need to know what those constructs mean.

---

22. Quantum Integration

Modules must be capable of organizing quantum programs without owning quantum semantics.

A module may contain:

- quantum types;
- quantum operations;
- circuits;
- kernels;
- measurement;
- dynamic control;
- logical operations;
- QEC intent;
- resilience requirements;
- quantum resource requirements.

But "grammar/modules/" must not define:

- quantum gates;
- qubit IDs;
- physical qubit IDs;
- device topology;
- routing;
- scheduling;
- calibration;
- QEC implementation;
- ZQN implementation;
- "quantum::ir".

The established pipeline remains:

Zamani source
    ↓
lexer
    ↓
parser
    ↓
domain-neutral AST
    ↓
semantic analysis
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
QEC / resilience / ZQN
    ↓
HAL
    ↓
target realization

"quantum::ir" remains the canonical quantum semantic boundary.

---

23. Classical Integration

A module may contain classical computation without the module subsystem knowing:

- CPU count;
- core count;
- thread count;
- register width;
- vector width;
- cache size;
- memory size;
- accelerator count.

For example:

module numerical.linear_algebra {
    ...
}

does not select a CPU architecture.

A downstream semantic/resource/compiler system determines an appropriate realization.

---

24. HDL and Hardware Integration

A module may organize:

- HDL;
- hardware/software co-design;
- accelerators;
- memories;
- interfaces;
- protocols;
- synthesis intent;
- verification;
- timing requirements.

The module subsystem must not encode:

MAX_REGISTER_WIDTH
MAX_BUS_WIDTH
MAX_FPGA_LUTS
MAX_ASIC_UNITS
MAX_MEMORY

or any equivalent universal hardware limit.

Hardware intent remains in:

grammar/hdl/
grammar/hardware/
grammar/resources/

and is resolved downstream.

---

25. Distributed and HPC Integration

Modules may contain distributed/HPC constructs.

The module grammar must not encode fixed:

node counts
worker counts
process counts
actor counts
replica counts
partition counts
channel counts
topology sizes

A module is source organization, not a cluster description.

The distributed semantic layer determines how source-level requirements can be realized.

---

26. AI/ML Integration

Modules can organize:

- models;
- datasets;
- training;
- inference;
- agents;
- symbolic computation;
- probabilistic computation;
- tensor computation;
- distributed training;
- accelerator workloads.

The module grammar must not embed framework-specific semantics such as:

torchModule
tensorflowModule
jaxModule
cudaModule

as language-level module kinds.

Framework interoperability belongs to interoperability/dialect/toolchain layers.

---

27. Sankofa, Temporal, Nano, and Future Domains

Existing and proposed Zamani concepts such as:

- Sankofa;
- memory;
- recall;
- learning;
- wisdom;
- temporal computation;
- multi-timeline systems;
- nano computation;
- future computational substrates

must be able to live inside ordinary modules.

The module grammar must not need a separate grammar for every new conceptual domain.

For example:

module sankofa.learning {
    ...
}

remains a normal module.

Semantic interpretation belongs to the relevant domain.

---

28. Imports Must Not Imply Runtime Loading

This is critical for POCO-REAF.

An import declaration describes source-level dependency/reference intent.

It does not necessarily mean:

load a dynamic library

or:

load a runtime module

or:

start a process

or:

allocate hardware

The compiler/runtime determines the appropriate realization.

This allows the same program to be compiled for:

embedded
CPU
multicore
GPU
FPGA
ASIC
QPU
simulator
accelerator
cluster
HPC
distributed
cloud
future target

without changing the module syntax merely because the execution substrate changes.

---

29. Package and Module Separation

The architecture must maintain:

module
    = source organization / compilation identity

package
    = distribution/dependency identity

namespace
    = naming/scope identity

dependency
    = relationship between separately managed components

They may interact semantically, but they must not be collapsed into one concept.

---

30. Dependency Graph Semantics

The grammar may express dependency declarations.

It must not construct the dependency graph.

The downstream architecture is:

dependency syntax
    ↓
frontend AST
    ↓
dependency declarations
    ↓
dependency resolver
    ↓
dependency graph
    ↓
version/compatibility validation
    ↓
package/artifact resolution
    ↓
compiler input set

The dependency graph may contain arbitrarily many nodes and edges.

The language must not encode a maximum.

---

31. Cycles and Self-Dependencies

The grammar may syntactically accept dependency relationships.

Semantic analysis must determine whether they are legal.

Examples include:

A → A

and:

A → B
B → A

The grammar must not attempt graph analysis.

The semantic/dependency subsystem owns:

- self-dependency detection;
- cycle detection;
- strongly connected components;
- dependency policy;
- optional cycle legality;
- diagnostic reporting.

---

32. Versioning Must Remain Semantic

The grammar recognizes version syntax.

Compatibility is semantic.

For example:

module_version
dependency_version
language_version
dialect_version

must not automatically be treated as interchangeable.

The compatibility subsystem determines:

- accepted versions;
- incompatible versions;
- migration requirements;
- feature gates;
- deprecations;
- API compatibility;
- ABI compatibility;
- compiler compatibility;
- runtime compatibility.

---

33. Frontend AST Contract

The repository already contains a domain-neutral frontend AST module representation under:

src/frontend/ast/node/program/module.rs

and module-aware item architecture.

The module grammar must therefore map into that architecture rather than introduce a second AST.

The conceptual information preserved by the AST must include, as applicable:

Module
├── identity/name
├── attributes
├── visibility
├── body/items
└── source span

The AST must preserve enough source information for:

- diagnostics;
- name resolution;
- formatting;
- source mapping;
- IDE tooling;
- refactoring;
- semantic analysis;
- dependency analysis;
- compatibility analysis;
- provenance.

The grammar must not define Rust AST structures.

---

34. Existing Rust Parser Integration

The repository's "src/parser.rs" currently contains legacy/direct module parsing through "parse_module()" and dispatches "KeywordModule".

This is an implementation surface, not a reason to create another module grammar.

The production architecture must converge on:

language specification
        ↓
canonical grammar
        ↓
frontend parser
        ↓
domain-neutral AST

The Rust parser implementation must remain behaviorally conformant with the canonical specification.

If ANTLR-generated parsing and the Rust parser are both retained during migration, they must be treated as conformance implementations, not independent language authorities.

A change to module syntax is complete only when the selected production parser implementation and the canonical grammar/specification agree.

No module README change should require the module grammar to be rewritten merely because an unrelated domain changes.

---

35. AST → Semantic Contract

The module AST is structural.

Semantic analysis owns:

Name resolution

- module names;
- namespace names;
- imported names;
- exported names;
- aliases.

Scope

- module scope;
- nested scope;
- declaration scope;
- visibility.

Dependency semantics

- dependency identity;
- dependency graph;
- version constraints;
- feature constraints.

Package semantics

- package identity;
- package/module relationship;
- package compatibility.

Compatibility

- language version;
- module version;
- package version;
- dialect version;
- API/ABI compatibility.

Cross-domain semantics

Whether a module may contain or expose a particular:

- quantum;
- classical;
- HDL;
- hardware;
- AI;
- distributed;
- networking;
- security;
- future-domain construct.

The grammar does not answer these questions.

---

36. AST → IR Contract

The module grammar must never become an IR.

Module information is lowered through semantic/module infrastructure.

Conceptually:

module syntax
    ↓
frontend AST
    ↓
module semantic graph
    ↓
canonical semantic representation
    ↓
domain-specific IR

For quantum content:

module
  ↓
AST
  ↓
semantic analysis
  ↓
quantum::ir

There must be no:

module grammar
    ↓
module-specific quantum IR

created by this directory.

---

37. Resource and Capability Separation

Modules themselves do not choose physical resources.

A module may contain constructs that express:

requires capability("quantum.measurement")

or:

requires capability("tensor.compute")

or:

requires resource(...)

but those requirements belong to the resource/capability semantic architecture.

The module grammar merely provides the organizational context in which those constructs occur.

The following distinction must remain explicit:

requirement
constraint
capability
preference
hint
implementation decision

For example:

requires capability("quantum.measurement")

is not equivalent to:

use physical_qpu_0

Likewise:

requires memory(...)

is not:

use memory_bank_3

The former expresses portable intent.

The latter is a target realization decision.

---

38. POCO-REAF Contract

The module subsystem must preserve:

Program Once
        ↓
Compile Once
        ↓
Run Everywhere
        ↓
Anywhere
        ↓
Forever

This requires module syntax to remain independent of the physical execution substrate.

The same module structure must be capable of organizing a program that is later realized on:

atom-scale substrate
molecular/nano substrate
embedded system
CPU
multicore CPU
GPU
FPGA
ASIC
QPU
quantum simulator
accelerator
cluster
HPC system
distributed infrastructure
cloud
future computational substrate

subject to the program's semantic requirements and the resources/capabilities available at realization time.

---

39. Hard-Coding Prohibition

The module subsystem must contain no universal machine constants such as:

MAX_MODULES
MAX_MODULE_DEPTH
MAX_IMPORTS
MAX_EXPORTS
MAX_DEPENDENCIES
MAX_PACKAGES
MAX_NODES
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_ASICS
MAX_QPUS
MAX_QUBITS
MAX_MEMORY
MAX_STORAGE
MAX_REGISTER_WIDTH
MAX_VECTOR_WIDTH
MAX_TENSOR_RANK
MAX_TIMELINES

It must contain no hard-coded:

- device IDs;
- hardware addresses;
- physical qubit numbers;
- CPU core numbers;
- GPU numbers;
- FPGA coordinates;
- cluster node IDs;
- memory-bank IDs;
- vendor-specific hardware topology.

Any such realization belongs downstream.

---

40. Unbounded-by-Specification Does Not Mean Unlimited by Hardware

The language must distinguish:

language expressiveness

from:

implementation resource availability

A program may express an arbitrarily large module/dependency/name structure.

A particular compilation may fail because:

- available memory is insufficient;
- parser resource budget is exhausted;
- compilation time policy is exceeded;
- dependency artifacts are unavailable;
- target capabilities are insufficient.

Such failures are implementation/environment/resource diagnostics.

They must not become universal language ceilings.

---

41. Determinism

Parsing identical source with identical parser configuration must produce equivalent syntactic structure.

Module parsing must not depend on:

- filesystem state;
- network state;
- registry state;
- hardware state;
- environment variables;
- clock time;
- random numbers;
- runtime state.

For example:

module quantum::algorithms {
    ...
}

must parse identically regardless of whether a QPU, GPU, FPGA, or cluster is present.

---

42. Security Contract

The module grammar is a pure syntax layer.

Parsing must not:

- execute imported code;
- execute package code;
- access remote registries;
- access arbitrary paths;
- follow arbitrary URLs;
- invoke shell commands;
- inspect credentials;
- inspect environment variables;
- dynamically load libraries;
- discover hardware.

Semantic and toolchain systems must enforce:

- trust;
- authorization;
- dependency policy;
- package policy;
- artifact verification;
- sandboxing;
- capability policy.

---

43. Reproducibility and Provenance

Module syntax must preserve enough information for downstream systems to establish:

- source provenance;
- module identity;
- package identity;
- dependency provenance;
- version information;
- source spans;
- compatibility information.

The grammar itself does not generate provenance records.

The frontend/compiler/toolchain consumes the AST information to construct them.

---

44. Diagnostics

Every module-related syntax error must preserve source location information.

At minimum, downstream diagnostics need:

source file
source span
module/import/export/package/dependency construct
expected syntax
actual syntax

Diagnostics must be deterministic.

Semantic diagnostics must be distinguished from syntax diagnostics.

For example:

syntax error:
expected qualified module name

is different from:

semantic error:
module name conflicts with an existing declaration

and different from:

dependency error:
requested dependency version is incompatible

The module grammar should not attempt to perform the latter two checks.

---

45. Compatibility

The module subsystem must preserve existing accepted syntax unless the language specification explicitly changes or deprecates it.

Compatibility must be tracked across:

grammar/specification/
grammar/spec/
grammar/modules/
grammar/Zamani.g4
grammar/antlr/ZamaniParser.g4
grammar/antlr/ZamaniLexer.g4
src/lexer.rs
src/parser.rs
src/frontend/ast/
semantic analysis
compiler

The repository's compatibility system remains responsible for migration/deprecation policy.

"grammar/grammar.md" remains an implementation-conformance reference and must not silently become another source of syntax authority.

"grammar/Zamani-Grammar.md" remains historical/extended/design material and does not automatically legalize module syntax.

---

46. Interoperability

Modules may organize interoperability boundaries such as:

- foreign functions;
- foreign types;
- C/C++;
- Rust;
- Python;
- WebAssembly;
- OpenQASM;
- QIR;
- HDL;
- accelerator interfaces.

The module subsystem does not own the semantics of those formats.

Interoperability remains owned by:

grammar/interoperability/

and downstream frontend/compiler infrastructure.

For quantum interoperability, external formats must ultimately map into Zamani's canonical semantic architecture rather than create a second canonical quantum model.

---

47. Dialects

A dialect may introduce controlled extensions, but dialect syntax must not silently modify the module system.

Dialect information belongs to:

grammar/dialects/

The module system may provide syntactic locations for dialect/module declarations or imports where specified, but:

- dialect registration;
- dialect version compatibility;
- dialect semantics;
- dialect-to-AST mapping;
- dialect-to-IR mapping

belong to dialect/semantic infrastructure.

---

48. Macros and Metaprogramming

Module syntax may coexist with:

grammar/macros/
grammar/metaprogramming/

but modules must not bypass macro/metaprogramming ownership.

Generated module/import/export declarations must be revalidated by the canonical grammar and semantic pipeline.

Metaprogramming must not become a mechanism for bypassing:

- visibility;
- dependency validation;
- security;
- type checking;
- resource/capability analysis;
- canonical IR construction.

---

49. Source Ordering

The module grammar must not accidentally impose source-order semantics merely because grammar files are modularized.

Where the language specification requires ordering, the canonical source-unit/program grammar owns that ordering.

For example, if imports must precede declarations in a particular source context, that is a source-unit/module semantic or syntactic contract—not a reason to duplicate the entire source grammar inside "imports.g4".

Fragment grammars remain reusable.

---

50. Compatibility Wrappers

Existing compatibility rules may remain where tooling depends on them.

Examples include wrappers around:

modulePath
moduleReference
moduleBodyItems
inlineModuleBody

Such wrappers must remain thin.

They must not become competing definitions of:

qualifiedName
sourceItem
moduleDeclaration

Compatibility wrappers must have documented owners and migration status.

---

51. Grammar Composition Contract

The intended dependency direction is:

ZamaniLexer
    │
    ▼
canonical names / attributes / visibility / source-unit
    │
    ▼
modules/*.g4
    │
    ▼
ZamaniParser
    │
    ▼
frontend AST
    │
    ▼
semantic analysis

The module grammars must never depend backward on:

semantic analysis
IR
compiler
runtime
HAL
hardware discovery
QEC
ZQN
routing
scheduling

---

52. No Domain-Specific Module Grammar

Do not create:

quantum-modules.g4
gpu-modules.g4
fpga-modules.g4
ai-modules.g4
distributed-modules.g4

merely to represent domains.

A module is universal.

Domain grammars define their own constructs and enter modules through "sourceItem".

---

53. Testing Contract

The module subsystem is production-ready only when tests cover all of the following.

53.1 Positive syntax tests

At minimum:

module a;
module a::b;
module a::b::c;
module a { }
module a::b { }

and valid combinations of:

- visibility;
- attributes;
- imports;
- exports;
- aliases;
- namespaces;
- packages;
- dependencies;
- versions.

53.2 Nested modules

Test:

module a {
    module b {
        module c {
            ...
        }
    }
}

with no artificial nesting ceiling.

53.3 Large qualified names

Test progressively larger qualified names.

The test suite must verify that no language-level maximum has been introduced.

53.4 Import tests

Cover:

- direct import;
- grouped import;
- wildcard import;
- aliases;
- symbolic sources;
- literal sources;
- nested qualified names;
- trailing separators where specified.

53.5 Export tests

Cover:

- direct export;
- named export;
- wildcard export;
- re-export;
- export aliases;
- exported qualified names.

53.6 Namespace tests

Cover:

- namespace declarations;
- nested namespace names;
- namespace aliases;
- references;
- namespace/module distinction.

53.7 Package tests

Cover:

- package declarations;
- package metadata;
- package version;
- package dependencies;
- package/module distinction.

53.8 Dependency tests

Cover:

- direct dependencies;
- optional dependencies;
- aliases;
- version requirements;
- features;
- capabilities;
- structured metadata.

53.9 Version tests

Cover:

- exact versions;
- version ranges;
- compatibility constraints;
- language/module/package/dialect versions;
- malformed versions.

53.10 Negative tests

Must include:

- malformed module names;
- malformed qualified names;
- malformed imports;
- malformed exports;
- invalid alias syntax;
- invalid package syntax;
- malformed dependency clauses;
- malformed versions;
- unterminated module bodies;
- invalid nesting;
- invalid separators;
- ambiguous constructs.

53.11 Boundary tests

Boundary tests must cover:

- empty modules;
- empty imports where permitted;
- empty exports where permitted;
- one-item modules;
- very large source units;
- deeply qualified names;
- deeply nested modules;
- large dependency metadata;
- large import/export lists.

No test may establish a false universal maximum.

53.12 Scalability tests

The suite must demonstrate structural scalability across increasing source sizes.

Test dimensions include:

module count
nesting depth
qualified-name length
imports
exports
dependencies
package metadata
source-item count

Tests should distinguish:

language validity

from:

test-runner/compiler resource exhaustion

53.13 Determinism tests

Parsing identical source repeatedly must produce equivalent results.

No dependence on:

- hardware;
- environment;
- network;
- filesystem state;
- clock;
- random state.

53.14 Round-trip tests

Where a formatter/printer exists:

source
  ↓
parse
  ↓
AST
  ↓
print
  ↓
parse

must preserve module semantics and relevant source structure.

53.15 Cross-domain tests

At least one module test must contain combinations of:

classical + quantum
classical + HDL
quantum + hardware
quantum + distributed
AI + quantum
AI + accelerator
HDL + hardware
distributed + networking
security + modules

The module grammar should remain unchanged by these combinations.

53.16 Compatibility tests

Existing valid module syntax must remain accepted unless intentionally changed by the specification.

---

54. Hard-Coding Audit Tests

Automated validation should inspect the module grammar for suspicious universal limits such as:

MAX_MODULES
MAX_MODULE_DEPTH
MAX_IMPORTS
MAX_EXPORTS
MAX_DEPENDENCIES
MAX_PACKAGES
MAX_NODES
MAX_CPUS
MAX_CORES
MAX_THREADS
MAX_GPUS
MAX_FPGAS
MAX_ASICS
MAX_QPUS
MAX_QUBITS
MAX_MEMORY
MAX_STORAGE

The audit must also reject accidental universal embedding of:

- hardware IDs;
- physical addresses;
- fixed topology;
- fixed accelerator counts;
- fixed qubit counts;
- fixed node counts;
- fixed vector widths;
- fixed tensor ranks.

A numeric literal is not automatically prohibited.

The distinction is:

program data

versus:

language implementation limit

For example:

module version = 1024;

may be valid data.

A grammar rule that accepts only 1024 modules is not.

---

55. Performance Contract

The grammar should use ordinary declarative ANTLR constructs:

- rule references;
- repetition;
- optionality;
- alternatives;
- structured lists.

It must avoid:

- embedded actions;
- semantic predicates unless explicitly justified by the canonical architecture;
- arbitrary computation;
- I/O;
- dynamic resource discovery.

Parser/compiler resource budgets may be configured externally.

They must not become language-level constants.

---

56. Memory and Resource Scaling

The module grammar must support source structures that scale according to available resources.

The correct model is:

language expressiveness
        ≠
machine capacity

A machine with limited memory may be unable to compile a very large module graph.

That does not make the program invalid Zamani.

Likewise, a large HPC system may compile and execute the same source without requiring the module source to be rewritten.

---

57. No Runtime Behavior in Grammar

The module grammar must remain inert.

Parsing:

import quantum::algorithm;

must not:

- initialize a QPU;
- allocate a qubit;
- contact a quantum service;
- start a process;
- load a runtime library.

Parsing:

import gpu::compute;

must not:

- select a GPU;
- discover GPU count;
- bind to GPU 0.

Parsing:

import distributed::service;

must not:

- discover nodes;
- connect to a cluster;
- create network connections.

Those decisions occur later.

---

58. Resource Availability and POCO-REAF

A module can describe a program whose realization requires certain capabilities.

For example, a module may contain constructs whose semantic analysis determines:

required capability
required resource
required reliability
required communication property
required memory property
required quantum capability

The compiler/resource/deployment layers then determine whether the available environment can satisfy those requirements.

This permits:

same source
   ↓
small available machine
   ↓
smaller realization

same source
   ↓
larger available machine
   ↓
larger realization

without changing module syntax.

The module subsystem does not choose the realization.

---

59. Compiler Integration

Module information must reach the compiler through semantic/module structures.

The compiler may use module information for:

- compilation-unit organization;
- dependency ordering;
- visibility;
- symbol resolution;
- incremental compilation;
- artifact identity;
- provenance;
- optimization boundaries;
- link boundaries;
- code generation;
- deployment packaging.

The grammar must not directly call compiler APIs.

---

60. Runtime Integration

Runtime systems may use module information for:

- artifact identity;
- symbol lookup;
- service/module loading;
- execution boundaries;
- diagnostics;
- observability;
- provenance.

The grammar must not define runtime loading behavior.

---

61. Hardware Integration

Hardware information remains outside the module grammar.

The architecture is:

module
   ↓
semantic program
   ↓
resource/capability requirements
   ↓
compiler/resource manager
   ↓
target selection
   ↓
routing/scheduling
   ↓
HAL
   ↓
hardware

The module name must never itself imply:

CPU
GPU
FPGA
ASIC
QPU
node
device
memory bank

---

62. Quantum Hardware Integration

A module containing quantum code must remain independent of physical topology.

For example:

module quantum::algorithms {
    ...
}

does not imply:

QPU 0
physical qubit 0
physical qubit 1
specific coupling map
specific calibration
specific gate set

Those are downstream realization decisions.

---

63. HDL Hardware Integration

Similarly:

module accelerator::matrix {
    ...
}

does not imply:

FPGA device 0
fixed LUT count
fixed DSP count
fixed BRAM count
fixed bus width

Hardware intent remains portable.

---

64. Distributed Integration

Likewise:

module distributed::solver {
    ...
}

does not imply:

node 0
node 1
node 2

or a fixed cluster size.

Placement, partitioning, replication, communication, and topology belong downstream.

---

65. Module Graph

The semantic module graph may contain arbitrarily many:

modules
packages
namespaces
dependencies
imports
exports
aliases

The graph implementation must use dynamically sized data structures and configurable resource policies.

The grammar must not constrain graph cardinality.

---

66. Error Recovery

Parser error recovery must preserve as much source structure as practical for diagnostics/tooling.

Module syntax errors should not cause unrelated domain constructs to acquire different meanings.

For example, an invalid import should produce an import-related diagnostic rather than silently reinterpret the remainder of a module as hardware, quantum, or expression syntax.

---

67. Source Span Preservation

Every module-related AST construct must retain source spans sufficient to diagnose:

- module declarations;
- module names;
- imports;
- exports;
- aliases;
- namespaces;
- packages;
- dependencies;
- versions;
- attributes.

Source spans must flow through:

lexer
→ parser
→ AST
→ semantic analysis
→ diagnostics

without losing source identity.

---

68. Tooling Integration

The module grammar must support tooling such as:

- formatter;
- syntax highlighter;
- language server;
- navigation;
- rename;
- import management;
- dependency inspection;
- module graph visualization;
- documentation generation;
- reference search;
- diagnostics;
- source indexing.

Tooling must use the canonical AST/name model.

It must not parse modules using ad-hoc regular expressions as an alternative language definition.

---

69. Documentation Integration

The module subsystem must be documented consistently across:

grammar/DESIGN.md
grammar/specification/
grammar/spec/
grammar/modules/README.md
grammar/grammar.md
grammar/reference/
grammar/Zamani-Grammar.md

Authority remains:

DESIGN/specification
        ↓
canonical grammar
        ↓
implementation conformance

"Zamani-Grammar.md" does not silently promote proposed syntax.

"grammar.md" documents implementation status.

---

70. Feature Lifecycle

A new module-system feature must follow:

proposal
    ↓
specification
    ↓
semantic contract
    ↓
AST contract
    ↓
canonical grammar
    ↓
implementation
    ↓
positive tests
    ↓
negative tests
    ↓
boundary tests
    ↓
scalability tests
    ↓
compatibility tests
    ↓
stable

No feature becomes stable merely because it appears in "Zamani-Grammar.md".

---

71. Independent Completion Contract

Every file under "grammar/modules/" must be independently completable.

Before a file is marked complete, its contract must already identify:

Purpose
Status
Owns
Does not own
Inputs
Outputs
Dependencies
Upstream contracts
Downstream consumers
Lexer integration
Parser integration
AST mapping
Semantic mapping
IR boundary
Compiler integration
Runtime integration
Tooling integration
Cross-domain integration
Diagnostics
Security
Performance
Positive tests
Negative tests
Boundary tests
Scalability tests
Determinism tests
Round-trip tests
Compatibility tests
Hard-coding audit
Completion criteria

This prevents the situation where completing one file requires reopening another file simply to discover an undocumented dependency.

---

72. Per-File Integration Contract

"modules.g4"

Input:
    canonical lexer + names + visibility + attributes + sourceItem

Output:
    module declaration parse tree

AST:
    Module node

Semantic:
    module identity/scope/body

IR:
    no module-specific IR created here

"imports.g4"

Input:
    canonical names + identifiers + literals

Output:
    import declaration parse tree

AST:
    import declaration

Semantic:
    name/dependency resolution

IR:
    no direct IR

"exports.g4"

Input:
    canonical names + identifiers

Output:
    export declaration parse tree

AST:
    export/re-export structure

Semantic:
    public interface/export resolution

IR:
    indirect through semantic module graph

"visibility.g4"

Input:
    canonical visibility tokens

Output:
    visibility syntax

AST:
    visibility modifier

Semantic:
    accessibility rules

"namespaces.g4"

Input:
    canonical qualified names

Output:
    namespace syntax

AST:
    namespace declaration/reference

Semantic:
    namespace scope

"aliases.g4"

Input:
    canonical names/identifiers

Output:
    alias syntax

AST:
    alias relationship

Semantic:
    alias resolution

"packages.g4"

Input:
    canonical names/version/attributes

Output:
    package syntax

AST:
    package declaration

Semantic:
    package identity/distribution semantics

"dependencies.g4"

Input:
    canonical names/version/attributes

Output:
    dependency declaration

AST:
    dependency specification

Semantic:
    dependency graph

"versioning.g4"

Input:
    canonical literals/operators/names

Output:
    version syntax

AST:
    version expression/constraint

Semantic:
    compatibility

"module-attributes.g4"

Input:
    canonical attributes/name/value syntax

Output:
    module attribute syntax

AST:
    attribute representation

Semantic:
    attribute meaning

---

73. Canonical Integration With "Zamani.g4"

"grammar/Zamani.g4" must compose the module subsystem rather than duplicate it.

Conceptually:

Zamani.g4
    │
    ├── moduleDeclaration
    ├── importDeclaration
    ├── exportDeclaration
    ├── namespaceDeclaration
    ├── packageDeclaration
    ├── dependencyDeclaration
    ├── declarations
    ├── functions
    ├── statements
    ├── quantum
    ├── classical
    ├── HDL
    ├── hardware
    ├── distributed
    ├── AI
    ├── data
    ├── networking
    └── other domains

Each construct has one owner.

The aggregate grammar performs composition.

It does not duplicate module rules.

---

74. Canonical Integration With "ZamaniParser.g4"

"grammar/antlr/ZamaniParser.g4" is the canonical ANTLR parser composition root.

It must expose the module subsystem through one canonical entry point.

It must not retain an independent legacy module grammar with different semantics.

The target architecture is:

ZamaniParser
    ↓
moduleDeclaration
    ↓
grammar/modules/*

with the exact generated/import structure determined by the final ANTLR composition.

---

75. Canonical Integration With "ZamaniLexer.g4"

All module keywords and punctuation must come from:

grammar/antlr/ZamaniLexer.g4

Examples include the canonical lexical vocabulary for concepts such as:

module
import
export
namespace
package
dependency
version
as
from
visibility

The exact token names are owned by the lexer.

Module parser grammars must not silently invent duplicate token names.

---

76. Rust Lexer Integration

"src/lexer.rs" is the Rust implementation/conformance lexer.

It must agree with the canonical lexical specification and ANTLR lexer vocabulary.

Module grammar changes must not create an incompatible token vocabulary.

Where discrepancies exist, they must be resolved through the repository's lexical authority/conformance process rather than by making "grammar/modules/" maintain a third token model.

---

77. Rust Parser Integration

"src/parser.rs" is the Rust parser implementation/conformance surface.

Its current module handling must remain semantically compatible with the production module specification and frontend AST.

The long-term invariant is:

same source
    ↓
canonical grammar
    ↓
canonical parser behavior
    ↓
same AST/module meaning

No module feature should exist only in the legacy Rust parser without corresponding canonical specification/grammar/AST contracts.

---

78. Frontend AST Integration

The frontend AST module representation under:

src/frontend/ast/node/program/module.rs

is the preferred domain-neutral semantic syntax representation.

The module grammar must map into this existing architecture rather than creating another module AST.

The AST must remain independent of:

- LLVM;
- QIR;
- MLIR;
- vendor IR;
- physical topology;
- QEC;
- routing;
- calibration;
- backend-specific hardware.

---

79. Canonical Quantum IR Boundary

Module syntax must never create a second quantum IR.

If a module contains quantum constructs:

module
  ↓
frontend AST
  ↓
semantic analysis
  ↓
quantum::ir

"quantum::ir" remains the canonical quantum semantic boundary.

Downstream systems remain responsible for:

- optimization;
- decomposition;
- routing;
- scheduling;
- resilience;
- QEC;
- ZQN;
- HAL;
- hardware realization.

---

80. Classical / HDL / Hardware IR Boundaries

The same separation applies to classical and hardware domains.

The module subsystem does not create:

ClassicalModuleIR
QuantumModuleIR
HDLModuleIR
HardwareModuleIR

merely because a module contains those constructs.

Module information is structural organization.

Domain-specific semantic representations belong to their respective canonical owners.

---

81. No Backend Coupling

"grammar/modules/" must not import or depend on:

- LLVM;
- MLIR;
- QIR;
- OpenQASM implementation internals;
- CUDA;
- ROCm;
- vendor SDKs;
- FPGA vendor APIs;
- QPU vendor APIs;
- OS APIs;
- runtime APIs;
- HAL implementations.

Interoperability grammars may describe source-level boundaries, but module syntax remains target-neutral.

---

82. No Filesystem Coupling

A module declaration must not require a particular filesystem layout.

The module system can be implemented over:

- ordinary files;
- generated files;
- archives;
- virtual filesystems;
- embedded resources;
- remote source providers;
- package stores.

Those are toolchain concerns.

---

83. No Network Coupling

Imports and dependencies are source declarations.

They do not imply network access.

Network/package resolution is an external toolchain concern.

This keeps parsing deterministic and secure.

---

84. No Hardware Coupling

A module name cannot select:

CPU
GPU
FPGA
ASIC
QPU
accelerator
node
device

Hardware realization occurs only after semantic analysis.

---

85. No Fixed Deployment Topology

The module system must not encode:

node count
cluster size
GPU count
QPU count
CPU count
FPGA count
memory size
network topology

A module graph can therefore remain valid when deployed:

locally
on one machine
on many machines
on an HPC system
on a cloud
on a quantum processor
on a simulator
on a future computational substrate

subject to downstream requirements and available resources.

---

86. Security Boundary

Module imports and dependencies are untrusted source declarations.

Semantic/toolchain validation must enforce:

trust
authorization
dependency policy
package policy
artifact integrity
capability policy
sandbox policy

The grammar must remain inert.

---

87. Rust 1.97 / 1.97.1 Contract

All Rust implementation and conformance work associated with this subsystem must support:

Rust 1.97
Rust 1.97.1

The implementation must use safe Rust.

No module-system requirement may depend on:

unsafe

or require unsafe FFI as part of the module grammar/parser contract.

If an external backend later requires unsafe internals, that is outside the module grammar and must not leak into this syntax contract.

---

88. Completion Criteria

"grammar/modules/" is PRODUCTION READY only when all of the following are true.

Architecture

- [ ] "README.md" is the module architecture contract.
- [ ] No module README section contradicts repository-wide authority.
- [ ] Existing filenames remain stable unless a deliberate migration is documented.
- [ ] Each grammar file has exactly one primary owner.
- [ ] No duplicate module grammar exists.

Lexer

- [ ] All parser-facing tokens come from "ZamaniLexer".
- [ ] No duplicate module token vocabulary exists.
- [ ] Keyword ownership is documented.

Parser

- [ ] "modules.g4" is integrated into the canonical parser composition.
- [ ] "ZamaniParser.g4" does not retain a competing module implementation.
- [ ] "Zamani.g4" does not duplicate module syntax.
- [ ] The canonical source-item boundary is used.

Names

- [ ] Identifiers are canonical.
- [ ] Qualified names are canonical.
- [ ] Module paths do not redefine qualified names.
- [ ] Namespace names reuse canonical names.

Modules

- [ ] Module declaration syntax is complete.
- [ ] Module bodies consume canonical source items.
- [ ] Nested modules work.
- [ ] No module-depth ceiling exists.

Imports

- [ ] Named imports work.
- [ ] Grouped imports work.
- [ ] Wildcard imports work where specified.
- [ ] Import aliases work.
- [ ] Symbolic sources work.
- [ ] Literal sources work where specified.
- [ ] No filesystem/network behavior occurs during parsing.

Exports

- [ ] Direct exports work.
- [ ] Named exports work.
- [ ] Wildcard exports work where specified.
- [ ] Re-exports work.
- [ ] Export aliases work.

Namespaces

- [ ] Namespace declarations work.
- [ ] Namespace references work.
- [ ] Namespace aliases work.
- [ ] Namespace and module identity remain distinct.

Packages

- [ ] Package syntax is complete.
- [ ] Package identity is distinct from module identity.
- [ ] Package semantics remain outside parsing.

Dependencies

- [ ] Dependency declarations work.
- [ ] Version requirements work.
- [ ] Dependency aliases work.
- [ ] Optional dependencies work where specified.
- [ ] Feature/capability requirements are structurally representable.
- [ ] Dependency solving is downstream.

Versioning

- [ ] Version syntax is canonical.
- [ ] Version constraints are canonical.
- [ ] Module/package/language/dialect versions are distinguishable.
- [ ] Compatibility remains semantic.

Attributes

- [ ] Module-specific attributes work.
- [ ] General attributes remain owned by the core attribute system.
- [ ] Attribute semantics remain downstream.

AST

- [ ] Module syntax maps to the domain-neutral frontend AST.
- [ ] Source spans are preserved.
- [ ] Names are preserved.
- [ ] Attributes are preserved.
- [ ] Visibility is preserved.
- [ ] Module body/items are preserved.
- [ ] No duplicate module AST exists.

Semantic integration

- [ ] Name resolution is downstream.
- [ ] Visibility validation is downstream.
- [ ] Dependency resolution is downstream.
- [ ] Package resolution is downstream.
- [ ] Version compatibility is downstream.
- [ ] Resource/capability validation is downstream.

IR

- [ ] Module grammar does not create IR.
- [ ] Quantum constructs reach canonical "quantum::ir".
- [ ] Classical constructs reach their canonical semantic representation.
- [ ] HDL/hardware constructs reach their canonical semantic representation.
- [ ] No duplicate quantum IR exists.

POCO-REAF

- [ ] No CPU count is encoded.
- [ ] No GPU count is encoded.
- [ ] No FPGA count is encoded.
- [ ] No QPU count is encoded.
- [ ] No qubit limit is encoded.
- [ ] No memory limit is encoded.
- [ ] No node limit is encoded.
- [ ] No topology limit is encoded.
- [ ] No accelerator limit is encoded.
- [ ] No tensor/vector hardware limit is encoded.
- [ ] Module syntax remains target-independent.

Safety

- [ ] No embedded actions.
- [ ] No arbitrary I/O.
- [ ] No filesystem access.
- [ ] No network access.
- [ ] No hardware discovery.
- [ ] No runtime execution.
- [ ] No "unsafe" Rust requirement.

Tests

- [ ] Positive tests.
- [ ] Negative tests.
- [ ] Boundary tests.
- [ ] Scalability tests.
- [ ] Determinism tests.
- [ ] Round-trip tests.
- [ ] Compatibility tests.
- [ ] Cross-domain tests.
- [ ] Hard-coding audit tests.

Documentation

- [ ] "grammar/DESIGN.md" agrees.
- [ ] "grammar/specification/" agrees.
- [ ] "grammar/spec/" agrees.
- [ ] "grammar/Zamani.g4" agrees.
- [ ] "grammar/antlr/ZamaniParser.g4" agrees.
- [ ] "grammar/antlr/ZamaniLexer.g4" agrees.
- [ ] "grammar/grammar.md" accurately reports implementation status.
- [ ] "grammar/Zamani-Grammar.md" does not silently promote unimplemented syntax.

---

89. Definition of Done for This Directory

The directory is complete only when every module grammar file can be understood independently using this README and can answer:

What do I own?
What do I not own?
Which canonical tokens do I consume?
Which canonical name rules do I consume?
Which grammar composes me?
Which AST node represents me?
Which semantic subsystem consumes me?
Which IR boundary follows me?
Which compiler components consume that semantic information?
Which runtime components may consume the resulting artifact?
Which other grammar files depend on me?
Which files must never depend on me?
What positive tests prove me?
What negative tests prove me?
What boundary tests prove me?
What scalability tests prove me?
What compatibility tests prove me?
What hard-coding audit proves me?
What makes me complete?

No future file should require undocumented architectural reinterpretation of an existing module file.

---

90. Final Module Architecture

The production architecture is:

                         ZAMANI MODULE SYSTEM
                                  │
              ┌───────────────────┼───────────────────┐
              │                   │                   │
          modules              names              visibility
              │                   │                   │
              ├───────────────┬───┴────┬──────────────┤
              │               │        │              │
           imports          exports  aliases      namespaces
              │               │        │              │
              └───────────────┼────────┴──────────────┘
                              │
                    ┌─────────┴─────────┐
                    │                   │
                 packages          dependencies
                    │                   │
                    └─────────┬─────────┘
                              │
                          versioning
                              │
                     module attributes
                              │
                              ▼
                       canonical sourceItem
                              │
                              ▼
                      domain-neutral AST
                              │
                              ▼
                      semantic module graph
                              │
              ┌───────────────┼────────────────┐
              │               │                │
          classical        quantum           HDL/
          semantics       semantics         hardware
              │               │                │
              │          quantum::ir          │
              │               │                │
              └───────────────┼────────────────┘
                              │
                    resource/capability
                           analysis
                              │
                              ▼
                        optimization
                              │
                    ┌─────────┼─────────┐
                    │         │         │
                  routing  scheduling  resilience
                    │         │         │
                    └─────────┼─────────┘
                              │
                         QEC / ZQN
                              │
                              ▼
                             HAL
                              │
                              ▼
                    target realization
                              │
        ┌─────────┬────────┬──┼───────┬─────────┐
        │         │        │  │       │         │
       CPU       GPU      FPGA ASIC   QPU    future
        │         │        │  │       │         │
        └─────────┴────────┴──┴───────┴─────────┘

The invariant is:

«A Zamani module organizes semantic program structure; it does not encode the physical machine on which that structure will eventually be realized.»

Therefore:

module structure
        ≠
filesystem structure
        ≠
package structure
        ≠
runtime structure
        ≠
hardware structure
        ≠
deployment topology

This separation is what allows module syntax to remain stable while Zamani scales from the smallest available computational substrate to arbitrarily large realizations supported by available resources.

The module subsystem is consequently a universal organizational layer, not a hardware-selection layer, package-manager implementation, runtime loader, or IR.

That is the production contract required for "grammar/modules/" to participate correctly in Zamani's POCO-REAF architecture.